//! # Zamani Frontend AST — Canonical Validation Errors
//!
//! `src/frontend/ast/node/validation/validation_error.rs`
//!
//! This module defines the format-independent error contract for structural
//! validation of the native Zamani AST.
//!
//! ## Architectural boundary
//!
//! ```text
//! source / parser
//!       │
//!       ▼
//! native Zamani AST
//!       │
//!       ▼
//! validation
//!       │
//!       └── validation_error.rs  ← this module
//!       │
//!       ▼
//! semantic analysis
//!       │
//!       ▼
//! semantic model
//!       │
//!       ▼
//! ZUIR
//!       │
//!       ▼
//! domain / target / hardware lowering
//! ```
//!
//! This module owns the common validation-error data model. It does not
//! perform validation and does not depend on semantic analysis, ZUIR,
//! quantum IR, QIR, LLVM, MLIR, hardware, routing, scheduling, calibration,
//! QEC, resilience, runtime execution, or backend APIs.
//!
//! ## Responsibilities
//!
//! This file owns:
//!
//! - validation error classification;
//! - stable validation error codes;
//! - optional AST-node/source locations;
//! - related source/node locations;
//! - deterministic structured context;
//! - human-readable messages;
//! - ordered validation-error aggregation;
//! - the generic validation result type.
//!
//! It does not own:
//!
//! - AST traversal;
//! - source-map lookup;
//! - source-range validation;
//! - node invariants;
//! - graph validation;
//! - semantic validation;
//! - type checking;
//! - quantum validation;
//! - hardware validation;
//! - validation policy.
//!
//! ## POCO-REAF
//!
//! Nothing in this module assumes a maximum:
//!
//! - AST size;
//! - node count;
//! - source size;
//! - qubit count;
//! - register count;
//! - operation count;
//! - machine size;
//! - computational-domain count;
//! - diagnostic count.
//!
//! Operational limits for untrusted input belong to validation/traversal
//! policies. They must not be hidden language limits in this type.
//!
//! ## Determinism
//!
//! All collections preserve insertion order. No hash-map iteration is used
//! for diagnostic ordering, and no global mutable state is used.
//!
//! ## Source locations
//!
//! A validation error can contain:
//!
//! - a node only;
//! - a span only;
//! - both;
//! - neither.
//!
//! A span is a coordinate, not proof that the corresponding source exists in
//! a source map. Source registration, source length, and UTF-8 boundary
//! validation belong to source infrastructure.
//!
//! ## Error-code stability
//!
//! `AST-*` is the namespace for native AST validation errors.
//!
//! Consumers must inspect `kind()` and `code()` rather than parse display
//! messages.
//!
//! Extensions and independently removable external-format frontends should
//! own their own error-code namespaces.
//!
//! ## Serialization
//!
//! The types in this module derive Serde serialization. The surrounding AST
//! serialization layer owns the AST serialization envelope/schema version.
//!
//! ## Rust compatibility
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021
//! - stable Rust only
//! - no unsafe code
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::node_id::NodeId;
use super::super::source::Span;

/// Stable high-level classification of a native AST validation failure.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ValidationErrorKind {
    /// A general AST structural invariant is invalid.
    Structural,

    /// A source span or source-coordinate invariant is invalid.
    SourceRange,

    /// A node identity or node-reference invariant is invalid.
    Identity,

    /// A node-kind or extension-identity invariant is invalid.
    NodeKind,

    /// A graph relationship is invalid.
    Graph,

    /// A recursion or nesting invariant is invalid.
    Recursion,

    /// Serialized AST or validation data is malformed.
    Serialization,

    /// An extension payload violates its structural contract.
    Extension,

    /// Validator input or configuration is invalid.
    InvalidInput,

    /// A configured operational validation limit was exceeded.
    LimitExceeded,

    /// A validator implementation invariant failed.
    Internal,
}

impl ValidationErrorKind {
    /// Returns the stable machine-readable spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structural => "structural",
            Self::SourceRange => "source_range",
            Self::Identity => "identity",
            Self::NodeKind => "node_kind",
            Self::Graph => "graph",
            Self::Recursion => "recursion",
            Self::Serialization => "serialization",
            Self::Extension => "extension",
            Self::InvalidInput => "invalid_input",
            Self::LimitExceeded => "limit_exceeded",
            Self::Internal => "internal",
        }
    }

    /// Returns whether this is a source-range failure.
    #[must_use]
    pub const fn is_source_range(self) -> bool {
        matches!(self, Self::SourceRange)
    }

    /// Returns whether this is an operational limit failure.
    #[must_use]
    pub const fn is_limit_failure(self) -> bool {
        matches!(self, Self::LimitExceeded)
    }

    /// Returns whether this is an AST structural failure.
    #[must_use]
    pub const fn is_structural(self) -> bool {
        matches!(
            self,
            Self::Structural
                | Self::SourceRange
                | Self::Identity
                | Self::NodeKind
                | Self::Graph
                | Self::Recursion
                | Self::Extension
        )
    }
}

impl fmt::Display for ValidationErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable machine-readable validation error code.
///
/// Core AST validation uses the `AST-*` namespace. Extensions may construct
/// their own trusted static namespaced codes with [`Self::new`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ValidationErrorCode(&'static str);

impl ValidationErrorCode {
    /// Generic structural validation failure.
    pub const STRUCTURAL: Self = Self("AST-001");

    /// Invalid source range.
    pub const SOURCE_RANGE: Self = Self("AST-002");

    /// Invalid node identity/reference.
    pub const IDENTITY: Self = Self("AST-003");

    /// Invalid node kind.
    pub const NODE_KIND: Self = Self("AST-004");

    /// Invalid graph relationship.
    pub const GRAPH: Self = Self("AST-005");

    /// Invalid recursion/nesting relationship.
    pub const RECURSION: Self = Self("AST-006");

    /// Malformed serialized AST/validation data.
    pub const SERIALIZATION: Self = Self("AST-007");

    /// Invalid extension structure.
    pub const EXTENSION: Self = Self("AST-008");

    /// Invalid validator input/configuration.
    pub const INVALID_INPUT: Self = Self("AST-009");

    /// Configured validation resource limit exceeded.
    pub const LIMIT_EXCEEDED: Self = Self("AST-010");

    /// Validator implementation invariant failed.
    pub const INTERNAL: Self = Self("AST-011");

    /// Creates a trusted static validation code.
    ///
    /// This is intended for separately versioned extensions.
    #[must_use]
    pub const fn new(code: &'static str) -> Self {
        Self(code)
    }

    /// Returns the stable code string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }

    /// Returns whether the code contains only machine-readable identifier
    /// characters.
    #[must_use]
    pub fn is_well_formed(self) -> bool {
        !self.0.is_empty()
            && self.0.bytes().all(|byte| {
                byte.is_ascii_alphanumeric()
                    || matches!(byte, b'-' | b'_' | b'.')
            })
    }
}

impl AsRef<str> for ValidationErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ValidationErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

/// Deterministic structured context attached to a validation error.
///
/// Context is ordered and intended for small structured facts such as
/// `relationship=parent-child`. Source contents and large AST payloads should
/// never be placed here.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ValidationContext {
    key: String,
    value: String,
}

impl ValidationContext {
    /// Creates one context item.
    #[must_use]
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Returns the context key.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the context value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for ValidationContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}={}", self.key, self.value)
    }
}

/// A source/node location associated with a validation error.
///
/// This type deliberately performs no source-map lookup.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ValidationLocation {
    node_id: Option<NodeId>,
    span: Option<Span>,
}

impl ValidationLocation {
    /// Creates a node-only location.
    #[must_use]
    pub const fn node(node_id: NodeId) -> Self {
        Self {
            node_id: Some(node_id),
            span: None,
        }
    }

    /// Creates a span-only location.
    #[must_use]
    pub const fn span(span: Span) -> Self {
        Self {
            node_id: None,
            span: Some(span),
        }
    }

    /// Creates a location containing both node and span.
    #[must_use]
    pub const fn node_and_span(
        node_id: NodeId,
        span: Span,
    ) -> Self {
        Self {
            node_id: Some(node_id),
            span: Some(span),
        }
    }

    /// Returns the optional node ID.
    #[must_use]
    pub const fn node_id(self) -> Option<NodeId> {
        self.node_id
    }

    /// Returns the optional span.
    #[must_use]
    pub const fn span(self) -> Option<Span> {
        self.span
    }

    /// Returns whether this location has neither a node nor a span.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.node_id.is_none() && self.span.is_none()
    }
}

impl fmt::Display for ValidationLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.node_id, self.span) {
            (Some(node), Some(span)) => {
                write!(formatter, "node {} at {}", node, span)
            }
            (Some(node), None) => {
                write!(formatter, "node {}", node)
            }
            (None, Some(span)) => {
                write!(formatter, "at {}", span)
            }
            (None, None) => Ok(()),
        }
    }
}

/// Canonical validation failure for the native Zamani AST.
///
/// The representation is intentionally generic enough for structural
/// validation, source-range validation, graph validation, serialization
/// validation, recursion validation, and future AST extensions without
/// coupling this type to a particular AST construct.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ValidationError {
    kind: ValidationErrorKind,
    code: ValidationErrorCode,
    message: String,
    location: Option<ValidationLocation>,
    related: Vec<ValidationLocation>,
    context: Vec<ValidationContext>,
}

impl ValidationError {
    /// Creates a validation error without a location.
    #[must_use]
    pub fn new(
        kind: ValidationErrorKind,
        code: ValidationErrorCode,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            code,
            message: message.into(),
            location: None,
            related: Vec::new(),
            context: Vec::new(),
        }
    }

    /// Creates a validation error associated with a location.
    #[must_use]
    pub fn at(
        kind: ValidationErrorKind,
        code: ValidationErrorCode,
        message: impl Into<String>,
        location: ValidationLocation,
    ) -> Self {
        Self {
            kind,
            code,
            message: message.into(),
            location: Some(location),
            related: Vec::new(),
            context: Vec::new(),
        }
    }

    /// Creates a node-local validation error.
    #[must_use]
    pub fn at_node(
        kind: ValidationErrorKind,
        code: ValidationErrorCode,
        message: impl Into<String>,
        node_id: NodeId,
    ) -> Self {
        Self::at(
            kind,
            code,
            message,
            ValidationLocation::node(node_id),
        )
    }

    /// Creates a source-range validation error.
    #[must_use]
    pub fn at_span(
        code: ValidationErrorCode,
        message: impl Into<String>,
        span: Span,
    ) -> Self {
        Self::at(
            ValidationErrorKind::SourceRange,
            code,
            message,
            ValidationLocation::span(span),
        )
    }

    /// Returns the validation error kind.
    #[must_use]
    pub const fn kind(&self) -> ValidationErrorKind {
        self.kind
    }

    /// Returns the stable error code.
    #[must_use]
    pub const fn code(&self) -> ValidationErrorCode {
        self.code
    }

    /// Returns the human-readable message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the primary location.
    #[must_use]
    pub const fn location(&self) -> Option<ValidationLocation> {
        self.location
    }

    /// Returns the primary node ID, if available.
    #[must_use]
    pub const fn node_id(&self) -> Option<NodeId> {
        match self.location {
            Some(location) => location.node_id(),
            None => None,
        }
    }

    /// Returns the primary source span, if available.
    #[must_use]
    pub const fn span(&self) -> Option<Span> {
        match self.location {
            Some(location) => location.span(),
            None => None,
        }
    }

    /// Returns related locations in deterministic insertion order.
    #[must_use]
    pub fn related(&self) -> &[ValidationLocation] {
        &self.related
    }

    /// Returns structured context in deterministic insertion order.
    #[must_use]
    pub fn context(&self) -> &[ValidationContext] {
        &self.context
    }

    /// Adds one related location.
    pub fn push_related(&mut self, location: ValidationLocation) {
        self.related.push(location);
    }

    /// Adds one context item.
    pub fn push_context(&mut self, context: ValidationContext) {
        self.context.push(context);
    }

    /// Builder-style addition of a related location.
    #[must_use]
    pub fn with_related(
        mut self,
        location: ValidationLocation,
    ) -> Self {
        self.push_related(location);
        self
    }

    /// Builder-style addition of structured context.
    #[must_use]
    pub fn with_context(
        mut self,
        context: ValidationContext,
    ) -> Self {
        self.push_context(context);
        self
    }

    /// Returns whether a primary location exists.
    #[must_use]
    pub const fn has_location(&self) -> bool {
        self.location.is_some()
    }

    /// Returns a concise deterministic summary.
    #[must_use]
    pub fn summary(&self) -> String {
        match self.location {
            Some(location) if !location.is_empty() => {
                format!(
                    "{} [{}] {}",
                    self.code, location, self.message
                )
            }
            _ => format!("{} {}", self.code, self.message),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)?;

        if let Some(location) = self.location {
            if !location.is_empty() {
                write!(formatter, " ({})", location)?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for ValidationError {}

/// Ordered collection of native AST validation errors.
///
/// There is deliberately no hidden maximum capacity. A caller handling
/// untrusted input may impose an explicit diagnostic-collection policy.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ValidationErrors {
    errors: Vec<ValidationError>,
}

impl ValidationErrors {
    /// Creates an empty validation-error collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            errors: Vec::new(),
        }
    }

    /// Creates a collection from an existing ordered vector.
    #[must_use]
    pub fn from_vec(errors: Vec<ValidationError>) -> Self {
        Self { errors }
    }

    /// Adds one error and returns the resulting number of errors.
    pub fn push(&mut self, error: ValidationError) -> usize {
        self.errors.push(error);
        self.errors.len()
    }

    /// Extends this collection while preserving input order.
    pub fn extend<I>(&mut self, errors: I)
    where
        I: IntoIterator<Item = ValidationError>,
    {
        self.errors.extend(errors);
    }

    /// Returns the number of errors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns an immutable ordered slice.
    #[must_use]
    pub fn as_slice(&self) -> &[ValidationError] {
        &self.errors
    }

    /// Returns an iterator over the errors.
    pub fn iter(&self) -> core::slice::Iter<'_, ValidationError> {
        self.errors.iter()
    }

    /// Consumes the collection and returns its vector.
    #[must_use]
    pub fn into_vec(self) -> Vec<ValidationError> {
        self.errors
    }

    /// Returns the first error.
    #[must_use]
    pub fn first(&self) -> Option<&ValidationError> {
        self.errors.first()
    }

    /// Returns the last error.
    #[must_use]
    pub fn last(&self) -> Option<&ValidationError> {
        self.errors.last()
    }
}

impl IntoIterator for ValidationErrors {
    type Item = ValidationError;
    type IntoIter = std::vec::IntoIter<ValidationError>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

impl<'a> IntoIterator for &'a ValidationErrors {
    type Item = &'a ValidationError;
    type IntoIter = core::slice::Iter<'a, ValidationError>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.iter()
    }
}

impl From<Vec<ValidationError>> for ValidationErrors {
    fn from(errors: Vec<ValidationError>) -> Self {
        Self::from_vec(errors)
    }
}

impl From<ValidationError> for ValidationErrors {
    fn from(error: ValidationError) -> Self {
        Self::from_vec(vec![error])
    }
}

/// Result type for validators that either return a value or collected
/// validation failures.
pub type ValidationResult<T> = Result<T, ValidationErrors>;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::source::{
        SourceId,
        SourceOffset,
    };

    #[test]
    fn predefined_codes_are_well_formed() {
        let codes = [
            ValidationErrorCode::STRUCTURAL,
            ValidationErrorCode::SOURCE_RANGE,
            ValidationErrorCode::IDENTITY,
            ValidationErrorCode::NODE_KIND,
            ValidationErrorCode::GRAPH,
            ValidationErrorCode::RECURSION,
            ValidationErrorCode::SERIALIZATION,
            ValidationErrorCode::EXTENSION,
            ValidationErrorCode::INVALID_INPUT,
            ValidationErrorCode::LIMIT_EXCEEDED,
            ValidationErrorCode::INTERNAL,
        ];

        assert!(codes.iter().all(|code| code.is_well_formed()));
    }

    #[test]
    fn location_can_contain_node_and_span() {
        let node = NodeId::first();

        let span = Span::new(
            SourceId::from_raw(1),
            SourceOffset::from_raw(4),
            SourceOffset::from_raw(9),
        )
        .expect("test span must be valid");

        let location =
            ValidationLocation::node_and_span(node, span);

        assert_eq!(location.node_id(), Some(node));
        assert_eq!(location.span(), Some(span));
        assert!(!location.is_empty());
    }

    #[test]
    fn context_and_related_locations_preserve_order() {
        let node = NodeId::first();

        let mut error = ValidationError::at_node(
            ValidationErrorKind::Structural,
            ValidationErrorCode::STRUCTURAL,
            "invalid child relationship",
            node,
        );

        error.push_context(ValidationContext::new(
            "relationship",
            "child",
        ));
        error.push_context(ValidationContext::new(
            "policy",
            "strict",
        ));
        error.push_related(ValidationLocation::node(node));

        assert_eq!(
            error.context()[0].key(),
            "relationship"
        );
        assert_eq!(error.context()[1].key(), "policy");
        assert_eq!(error.related().len(), 1);
    }

    #[test]
    fn validation_errors_preserve_insertion_order() {
        let first = ValidationError::new(
            ValidationErrorKind::Identity,
            ValidationErrorCode::IDENTITY,
            "first",
        );

        let second = ValidationError::new(
            ValidationErrorKind::SourceRange,
            ValidationErrorCode::SOURCE_RANGE,
            "second",
        );

        let mut errors = ValidationErrors::new();

        errors.push(first.clone());
        errors.push(second.clone());

        assert_eq!(errors.len(), 2);
        assert_eq!(errors.first(), Some(&first));
        assert_eq!(errors.last(), Some(&second));
        assert_eq!(
            errors.as_slice(),
            &[first, second]
        );
    }

    #[test]
    fn validation_error_serializes_and_deserializes() {
        let node = NodeId::first();

        let error = ValidationError::at_node(
            ValidationErrorKind::Serialization,
            ValidationErrorCode::SERIALIZATION,
            "malformed AST payload",
            node,
        )
        .with_context(ValidationContext::new(
            "format",
            "ast",
        ));

        let encoded =
            serde_json::to_string(&error)
                .expect("serialization must succeed");

        let decoded: ValidationError =
            serde_json::from_str(&encoded)
                .expect("deserialization must succeed");

        assert_eq!(decoded, error);
    }
}