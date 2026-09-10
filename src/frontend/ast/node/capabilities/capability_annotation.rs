//! Source-level capability annotations for the Zamani AST.
//!
//! # Architectural responsibility
//!
//! This module represents annotations attached to capability declarations,
//! requirements, provisions, constraints, permissions, or extension-defined
//! capabilities.
//!
//! An annotation is source-level information. It describes programmer intent
//! or provides extensible metadata that later compiler phases may interpret.
//!
//! This module deliberately does NOT resolve annotations into hardware,
//! backend, vendor, topology, scheduling, calibration, routing, QEC, or
//! execution concepts.
//!
//! # POCO-REAF
//!
//! Zamani follows:
//!
//!     Program Once
//!     Compile Once
//!     Run Everywhere
//!     Anywhere
//!     Forever
//!
//! Consequently, annotations must not encode assumptions such as:
//!
//! - fixed qubit counts;
//! - fixed machine sizes;
//! - fixed register widths;
//! - fixed topology;
//! - fixed gate sets;
//! - fixed vendors;
//! - fixed backends;
//! - fixed processor architectures;
//! - fixed quantum technologies.
//!
//! Concrete capability realization belongs downstream.
//!
//! # Extensibility
//!
//! Annotation names are namespaced and represented as data rather than a
//! closed enum. This allows future language and computational domains to
//! introduce annotations without modifying this file.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - the capability-annotation value;
//! - annotation namespace/name;
//! - optional source-level value;
//! - annotation validation;
//! - deterministic identity representation.
//!
//! `capability.rs` owns the capability itself.
//! `capability_kind.rs` owns capability classification.
//!
//! # Forbidden dependencies
//!
//! This module must not depend on:
//!
//! - quantum hardware;
//! - QPU implementations;
//! - vendor SDKs;
//! - backend APIs;
//! - topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC implementations;
//! - noise models;
//! - execution jobs;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - target-specific IR.
//!
//! Rust: 1.97 / 1.97.1
//! `unsafe`: forbidden.

use core::fmt;
use core::str::FromStr;

/// A source-level annotation attached to a capability.
///
/// An annotation consists of an open-ended namespaced identity and an
/// optional source-level value.
///
/// Examples of valid identities include conceptual forms such as:
///
/// - `quantum.requires_fault_tolerance`
/// - `resource.dynamic`
/// - `execution.adaptive`
/// - `future.example`
///
/// The AST does not assign special meaning to these names. Meaning is
/// resolved by the semantic/extension layer.
///
/// The value is preserved as source-level text rather than prematurely
/// converted into a target-specific representation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CapabilityAnnotation {
    identity: AnnotationIdentity,
    value: Option<AnnotationValue>,
}

impl CapabilityAnnotation {
    /// Creates an annotation without a value.
    ///
    /// This is appropriate for marker-style annotations.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, CapabilityAnnotationError> {
        Ok(Self {
            identity: AnnotationIdentity::new(namespace, name)?,
            value: None,
        })
    }

    /// Creates an annotation with a source-preserved value.
    pub fn with_value(
        namespace: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, CapabilityAnnotationError> {
        Ok(Self {
            identity: AnnotationIdentity::new(namespace, name)?,
            value: Some(AnnotationValue::new(value.into())?),
        })
    }

    /// Creates an annotation from an already validated identity.
    #[must_use]
    pub fn from_identity(identity: AnnotationIdentity) -> Self {
        Self {
            identity,
            value: None,
        }
    }

    /// Creates an annotation from an identity and validated value.
    #[must_use]
    pub fn from_parts(
        identity: AnnotationIdentity,
        value: Option<AnnotationValue>,
    ) -> Self {
        Self { identity, value }
    }

    /// Returns the annotation identity.
    #[must_use]
    pub fn identity(&self) -> &AnnotationIdentity {
        &self.identity
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        self.identity.namespace()
    }

    /// Returns the annotation name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.identity.name()
    }

    /// Returns the optional source-level value.
    #[must_use]
    pub fn value(&self) -> Option<&AnnotationValue> {
        self.value.as_ref()
    }

    /// Returns the raw annotation value when present.
    #[must_use]
    pub fn value_str(&self) -> Option<&str> {
        self.value.as_ref().map(AnnotationValue::as_str)
    }

    /// Returns whether the annotation is a marker annotation.
    #[must_use]
    pub fn is_marker(&self) -> bool {
        self.value.is_none()
    }

    /// Returns whether the annotation has a value.
    #[must_use]
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    /// Returns a deterministic qualified identity.
    ///
    /// The qualified form is:
    ///
    ///     namespace::name
    ///
    /// No semantic normalization is performed. The exact namespace and name
    /// supplied by the source are preserved.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        self.identity.qualified_name()
    }

    /// Returns the deterministic annotation key.
    ///
    /// The key contains only the annotation identity and intentionally
    /// excludes the value. This makes it suitable for identity-based lookup.
    #[must_use]
    pub fn key(&self) -> String {
        self.identity.qualified_name()
    }

    /// Validates the complete annotation.
    pub fn validate(&self) -> Result<(), CapabilityAnnotationError> {
        self.identity.validate()?;

        if let Some(value) = &self.value {
            value.validate()?;
        }

        Ok(())
    }
}

/// Stable namespaced identity for an annotation.
///
/// The identity is deliberately open-ended. It is not an enum and therefore
/// does not need to change when a new quantum technology, computational
/// domain, compiler feature, or extension introduces a new annotation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AnnotationIdentity {
    namespace: String,
    name: String,
}

impl AnnotationIdentity {
    /// Creates and validates an annotation identity.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, CapabilityAnnotationError> {
        let identity = Self {
            namespace: namespace.into(),
            name: name.into(),
        };

        identity.validate()?;
        Ok(identity)
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the namespace is non-empty.
    #[must_use]
    pub fn is_namespaced(&self) -> bool {
        !self.namespace.is_empty()
    }

    /// Returns the canonical qualified name.
    ///
    /// The representation is deliberately simple and deterministic:
    ///
    ///     namespace::name
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}::{}", self.namespace, self.name)
    }

    /// Validates the identity.
    ///
    /// This layer intentionally performs only structural validation.
    /// Language-specific identifier rules belong to the parser/semantic
    /// layer because extensions may have different identifier grammars.
    pub fn validate(&self) -> Result<(), CapabilityAnnotationError> {
        validate_component(
            &self.namespace,
            AnnotationComponent::Namespace,
        )?;

        validate_component(&self.name, AnnotationComponent::Name)?;

        Ok(())
    }
}

/// Source-preserved annotation value.
///
/// The value remains textual at the AST layer so that semantic analysis can
/// determine whether it represents a boolean, number, string, expression,
/// resource constraint, symbolic value, or extension-defined structure.
///
/// This prevents the AST from prematurely imposing machine-dependent types.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AnnotationValue(String);

impl AnnotationValue {
    /// Creates a validated annotation value.
    pub fn new(value: String) -> Result<Self, CapabilityAnnotationError> {
        let annotation_value = Self(value);
        annotation_value.validate()?;
        Ok(annotation_value)
    }

    /// Returns the preserved source-level value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the value and returns its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }

    /// Returns whether the value is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Validates the value.
    ///
    /// Empty values are rejected because a marker annotation should use
    /// `None`, while a value-bearing annotation should contain actual source
    /// data.
    pub fn validate(&self) -> Result<(), CapabilityAnnotationError> {
        if self.0.is_empty() {
            return Err(CapabilityAnnotationError::EmptyValue);
        }

        for (index, character) in self.0.char_indices() {
            if character.is_control() {
                return Err(CapabilityAnnotationError::ControlCharacter {
                    component: AnnotationComponent::Value,
                    index,
                });
            }
        }

        Ok(())
    }
}

impl AsRef<str> for AnnotationValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AnnotationValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Identifies which annotation component failed validation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AnnotationComponent {
    /// The annotation namespace.
    Namespace,

    /// The annotation name.
    Name,

    /// The annotation value.
    Value,
}

impl fmt::Display for AnnotationComponent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Namespace => formatter.write_str("namespace"),
            Self::Name => formatter.write_str("name"),
            Self::Value => formatter.write_str("value"),
        }
    }
}

/// Errors produced while constructing or validating capability annotations.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CapabilityAnnotationError {
    /// The namespace was empty.
    EmptyNamespace,

    /// The annotation name was empty.
    EmptyName,

    /// A namespace/name/value contained leading or trailing whitespace.
    BoundaryWhitespace {
        /// Component containing the invalid whitespace.
        component: AnnotationComponent,
    },

    /// A namespace/name/value contained a control character.
    ControlCharacter {
        /// Component containing the invalid character.
        component: AnnotationComponent,

        /// UTF-8 byte index of the character.
        index: usize,
    },

    /// An annotation value was explicitly supplied but empty.
    EmptyValue,
}

impl fmt::Display for CapabilityAnnotationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNamespace => {
                formatter.write_str("capability annotation namespace cannot be empty")
            }

            Self::EmptyName => {
                formatter.write_str("capability annotation name cannot be empty")
            }

            Self::BoundaryWhitespace { component } => {
                write!(
                    formatter,
                    "capability annotation {component} cannot have leading or trailing whitespace"
                )
            }

            Self::ControlCharacter { component, index } => {
                write!(
                    formatter,
                    "capability annotation {component} contains a control character at byte index {index}"
                )
            }

            Self::EmptyValue => {
                formatter.write_str(
                    "capability annotation value cannot be empty",
                )
            }
        }
    }
}

impl std::error::Error for CapabilityAnnotationError {}

impl fmt::Display for AnnotationIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.qualified_name())
    }
}

impl FromStr for AnnotationIdentity {
    type Err = CapabilityAnnotationError;

    /// Parses an identity in:
    ///
    ///     namespace::name
    ///
    /// Only the structural split is performed here. Semantic interpretation
    /// of namespaces belongs to the extension/capability registry.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let separator = value
            .rfind("::")
            .ok_or(CapabilityAnnotationError::EmptyNamespace)?;

        let namespace = &value[..separator];
        let name = &value[separator + 2..];

        Self::new(namespace, name)
    }
}

fn validate_component(
    value: &str,
    component: AnnotationComponent,
) -> Result<(), CapabilityAnnotationError> {
    match component {
        AnnotationComponent::Namespace if value.is_empty() => {
            return Err(CapabilityAnnotationError::EmptyNamespace);
        }

        AnnotationComponent::Name if value.is_empty() => {
            return Err(CapabilityAnnotationError::EmptyName);
        }

        AnnotationComponent::Value if value.is_empty() => {
            return Err(CapabilityAnnotationError::EmptyValue);
        }

        _ => {}
    }

    if value.trim() != value {
        return Err(
            CapabilityAnnotationError::BoundaryWhitespace { component },
        );
    }

    for (index, character) in value.char_indices() {
        if character.is_control() {
            return Err(
                CapabilityAnnotationError::ControlCharacter {
                    component,
                    index,
                },
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_marker_annotation() {
        let annotation =
            CapabilityAnnotation::new("quantum", "adaptive")
                .expect("valid annotation");

        assert_eq!(annotation.namespace(), "quantum");
        assert_eq!(annotation.name(), "adaptive");
        assert!(annotation.is_marker());
        assert!(!annotation.has_value());
        assert_eq!(annotation.qualified_name(), "quantum::adaptive");
    }

    #[test]
    fn creates_value_annotation() {
        let annotation = CapabilityAnnotation::with_value(
            "resource",
            "cardinality",
            "N",
        )
        .expect("valid annotation");

        assert_eq!(annotation.namespace(), "resource");
        assert_eq!(annotation.name(), "cardinality");
        assert!(annotation.has_value());
        assert_eq!(annotation.value_str(), Some("N"));
    }

    #[test]
    fn preserves_symbolic_values() {
        let annotation = CapabilityAnnotation::with_value(
            "quantum",
            "required_resources",
            "N * logical_qubits",
        )
        .expect("valid annotation");

        assert_eq!(
            annotation.value_str(),
            Some("N * logical_qubits")
        );
    }

    #[test]
    fn identity_round_trips() {
        let identity: AnnotationIdentity = "quantum::adaptive"
            .parse()
            .expect("valid identity");

        assert_eq!(identity.namespace(), "quantum");
        assert_eq!(identity.name(), "adaptive");
        assert_eq!(
            identity.qualified_name(),
            "quantum::adaptive"
        );
    }

    #[test]
    fn identity_supports_nested_namespaces() {
        let identity: AnnotationIdentity =
            "quantum.execution::adaptive"
                .parse()
                .expect("valid nested namespace");

        assert_eq!(
            identity.namespace(),
            "quantum.execution"
        );
        assert_eq!(identity.name(), "adaptive");
    }

    #[test]
    fn empty_namespace_is_rejected() {
        let error = AnnotationIdentity::new("", "adaptive")
            .expect_err("empty namespace must fail");

        assert_eq!(
            error,
            CapabilityAnnotationError::EmptyNamespace
        );
    }

    #[test]
    fn empty_name_is_rejected() {
        let error = AnnotationIdentity::new("quantum", "")
            .expect_err("empty name must fail");

        assert_eq!(
            error,
            CapabilityAnnotationError::EmptyName
        );
    }

    #[test]
    fn leading_whitespace_is_rejected() {
        let error = AnnotationIdentity::new(" quantum", "adaptive")
            .expect_err("leading whitespace must fail");

        assert_eq!(
            error,
            CapabilityAnnotationError::BoundaryWhitespace {
                component: AnnotationComponent::Namespace,
            }
        );
    }

    #[test]
    fn trailing_whitespace_is_rejected() {
        let error = AnnotationIdentity::new("quantum", "adaptive ")
            .expect_err("trailing whitespace must fail");

        assert_eq!(
            error,
            CapabilityAnnotationError::BoundaryWhitespace {
                component: AnnotationComponent::Name,
            }
        );
    }

    #[test]
    fn control_characters_are_rejected() {
        let error = AnnotationIdentity::new(
            "quantum",
            "adaptive\nexecution",
        )
        .expect_err("control character must fail");

        assert_eq!(
            error,
            CapabilityAnnotationError::ControlCharacter {
                component: AnnotationComponent::Name,
                index: 8,
            }
        );
    }

    #[test]
    fn empty_annotation_value_is_rejected() {
        let error = AnnotationValue::new(String::new())
            .expect_err("empty value must fail");

        assert_eq!(
            error,
            CapabilityAnnotationError::EmptyValue
        );
    }

    #[test]
    fn value_preserves_unicode() {
        let value = AnnotationValue::new(
            "能力-quantum".to_owned(),
        )
        .expect("unicode should be supported");

        assert_eq!(value.as_str(), "能力-quantum");
    }

    #[test]
    fn annotations_are_orderable_for_deterministic_collections() {
        let mut annotations = vec![
            CapabilityAnnotation::new("z", "one")
                .expect("valid annotation"),
            CapabilityAnnotation::new("a", "two")
                .expect("valid annotation"),
            CapabilityAnnotation::new("m", "three")
                .expect("valid annotation"),
        ];

        annotations.sort();

        assert_eq!(
            annotations[0].qualified_name(),
            "a::two"
        );
        assert_eq!(
            annotations[1].qualified_name(),
            "m::three"
        );
        assert_eq!(
            annotations[2].qualified_name(),
            "z::one"
        );
    }

    #[test]
    fn equality_includes_value() {
        let marker = CapabilityAnnotation::new(
            "quantum",
            "adaptive",
        )
        .expect("valid annotation");

        let valued = CapabilityAnnotation::with_value(
            "quantum",
            "adaptive",
            "true",
        )
        .expect("valid annotation");

        assert_ne!(marker, valued);
    }

    #[test]
    fn key_depends_only_on_identity() {
        let first = CapabilityAnnotation::with_value(
            "quantum",
            "precision",
            "high",
        )
        .expect("valid annotation");

        let second = CapabilityAnnotation::with_value(
            "quantum",
            "precision",
            "low",
        )
        .expect("valid annotation");

        assert_eq!(first.key(), second.key());
    }

    #[test]
    fn validation_is_idempotent() {
        let annotation = CapabilityAnnotation::with_value(
            "quantum",
            "adaptive",
            "N",
        )
        .expect("valid annotation");

        annotation
            .validate()
            .expect("first validation");

        annotation
            .validate()
            .expect("second validation");
    }
}