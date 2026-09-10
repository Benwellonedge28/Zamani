//! Source-level resource annotations for the Zamani AST.
//!
//! # Architectural boundary
//!
//! This module describes annotations attached to resource declarations or
//! resource-related AST nodes. It deliberately does NOT describe:
//!
//! - physical hardware;
//! - quantum backends;
//! - qubit topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC implementation;
//! - runtime allocation;
//! - backend credentials;
//! - vendor-specific instruction sets.
//!
//! An annotation expresses source-level intent, constraints, metadata, or
//! requirements. Its interpretation belongs to later compiler phases.
//!
//! # Dependency direction
//!
//! ```text
//! source
//!   |
//!   v
//! parser
//!   |
//!   v
//! resource annotation AST
//!   |
//!   v
//! structural validation
//!   |
//!   v
//! semantic analysis
//!   |
//!   v
//! ZUIR / domain IR
//!   |
//!   v
//! target / backend
//! ```
//!
//! This module must never reverse that dependency direction.
//!
//! # Scalability
//!
//! No machine size, qubit count, backend, vendor, topology, gate set, or
//! finite domain list is encoded here.
//!
//! Collections are dynamically sized and preserve source order. Compiler
//! resource limits, if required for hostile/untrusted input, belong to an
//! external configurable policy rather than this AST representation.
//!
//! # Rust
//!
//! Designed for Rust 1.97 / 1.97.1.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.

use core::fmt;
use std::error::Error;

/* ------------------------------------------------------------------------- */
/* Namespace identity                                                        */
/* ------------------------------------------------------------------------- */

/// A stable, namespaced identifier for a resource annotation.
///
/// The namespace prevents unrelated annotations from accidentally acquiring
/// the same semantic identity. Both fields are source-level strings rather
/// than closed enums so future annotations do not require changes to the
/// native AST.
///
/// Examples:
///
/// - `core::resource`
— `quantum::resource`
— `vendor.example::resource`
— `user::my_annotation`
///
/// The AST does not decide whether a namespace is known or permitted. That is
/// the responsibility of the extension registry / semantic phase.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceAnnotationId {
    namespace: String,
    name: String,
}

impl ResourceAnnotationId {
    /// Creates an annotation identifier.
    ///
    /// Empty namespace or name is rejected because an annotation without a
    /// stable identity cannot be resolved deterministically.
    pub fn new<N, S>(namespace: N, name: S) -> Result<Self, ResourceAnnotationIdError>
    where
        N: Into<String>,
        S: Into<String>,
    {
        let namespace = namespace.into();
        let name = name.into();

        validate_identifier_component("namespace", &namespace)?;
        validate_identifier_component("name", &name)?;

        Ok(Self { namespace, name })
    }

    /// Returns the namespace.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the annotation name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the canonical `namespace::name` representation.
    pub fn canonical_name(&self) -> String {
        let mut result =
            String::with_capacity(self.namespace.len() + 2 + self.name.len());

        result.push_str(&self.namespace);
        result.push_str("::");
        result.push_str(&self.name);

        result
    }

    /// Consumes the identifier and returns its components.
    pub fn into_parts(self) -> (String, String) {
        (self.namespace, self.name)
    }
}

impl fmt::Display for ResourceAnnotationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.namespace, self.name)
    }
}

/* ------------------------------------------------------------------------- */
/* Identifier validation                                                     */
/* ------------------------------------------------------------------------- */

fn validate_identifier_component(
    field: &'static str,
    value: &str,
) -> Result<(), ResourceAnnotationIdError> {
    if value.is_empty() {
        return Err(ResourceAnnotationIdError::EmptyComponent { field });
    }

    if value.trim() != value {
        return Err(ResourceAnnotationIdError::Whitespace {
            field,
            value: value.to_owned(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(ResourceAnnotationIdError::ControlCharacter {
            field,
            value: value.to_owned(),
        });
    }

    Ok(())
}

/* ------------------------------------------------------------------------- */
/* Annotation value                                                          */
/* ------------------------------------------------------------------------- */

/// A source-preserving value carried by a resource annotation.
///
/// This is intentionally not tied to a particular semantic type system.
/// Semantic analysis can interpret the value later.
///
/// The representation is recursive and dynamically sized. It therefore does
/// not impose a fixed number of annotation arguments, object fields, or
/// nesting levels at the language level.
///
/// `Expression` stores an AST-level reference rather than embedding an
/// expression implementation here. The reference is opaque to this module
/// and can be resolved by the surrounding AST/semantic layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceAnnotationValue {
    /// Annotation without an explicit value.
    Unit,

    /// Boolean source value.
    Bool(bool),

    /// Integer represented textually so the AST does not impose a machine
    /// integer width or precision.
    Integer(String),

    /// Floating-point/real value represented textually.
    ///
    /// Keeping the lexical representation avoids prematurely selecting a
    /// target floating-point format.
    Real(String),

    /// String value.
    String(String),

    /// Identifier or symbolic name.
    Symbol(String),

    /// Reference to an AST expression.
    ///
    /// The ID is intentionally represented by the repository's public
    /// `NodeId` contract through a small opaque integer wrapper below rather
    /// than importing semantic or backend code.
    Expression(ResourceAnnotationNodeRef),

    /// Ordered sequence.
    List(Vec<ResourceAnnotationValue>),

    /// Ordered key/value object.
    ///
    /// A vector rather than a hash map preserves source order and gives
    /// deterministic serialization/traversal.
    Object(Vec<ResourceAnnotationField>),
}

impl ResourceAnnotationValue {
    /// Creates an integer value while preserving its lexical representation.
    pub fn integer<S: Into<String>>(value: S) -> Self {
        Self::Integer(value.into())
    }

    /// Creates a real value while preserving its lexical representation.
    pub fn real<S: Into<String>>(value: S) -> Self {
        Self::Real(value.into())
    }

    /// Creates a symbolic value.
    pub fn symbol<S: Into<String>>(value: S) -> Self {
        Self::Symbol(value.into())
    }

    /// Returns `true` when this value has no explicit payload.
    pub fn is_unit(&self) -> bool {
        matches!(self, Self::Unit)
    }

    /// Returns the number of immediate logical children.
    ///
    /// This is useful for structural validation without recursively walking
    /// an arbitrarily large annotation tree.
    pub fn child_count(&self) -> usize {
        match self {
            Self::Unit
            | Self::Bool(_)
            | Self::Integer(_)
            | Self::Real(_)
            | Self::String(_)
            | Self::Symbol(_)
            | Self::Expression(_) => 0,

            Self::List(values) => values.len(),

            Self::Object(fields) => fields.len(),
        }
    }
}

/* ------------------------------------------------------------------------- */
/* Opaque AST node reference                                                 */
/* ------------------------------------------------------------------------- */

/// Opaque reference to another AST node used by an annotation value.
///
/// This type intentionally contains no pointer, memory address, semantic
/// object, or backend object.
///
/// If the repository already provides a canonical `NodeId`, the surrounding
/// AST integration layer should provide a conversion between that canonical
/// ID and this reference. This keeps this file independent and prevents a
/// dependency cycle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceAnnotationNodeRef(u64);

impl ResourceAnnotationNodeRef {
    /// Creates a node reference from its stable numeric representation.
    ///
    /// The value is opaque. Allocation policy belongs to the AST node-ID
    /// subsystem.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the opaque numeric representation.
    pub const fn get(self) -> u64 {
        self.0
    }
}

/* ------------------------------------------------------------------------- */
/* Object fields                                                             */
/* ------------------------------------------------------------------------- */

/// A named field in an annotation object.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceAnnotationField {
    name: String,
    value: ResourceAnnotationValue,
}

impl ResourceAnnotationField {
    /// Creates a named annotation field.
    pub fn new<N: Into<String>>(
        name: N,
        value: ResourceAnnotationValue,
    ) -> Result<Self, ResourceAnnotationFieldError> {
        let name = name.into();

        validate_identifier_component("field", &name)
            .map_err(ResourceAnnotationFieldError::InvalidName)?;

        Ok(Self { name, value })
    }

    /// Returns the field name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the field value.
    pub fn value(&self) -> &ResourceAnnotationValue {
        &self.value
    }

    /// Returns the field value mutably.
    pub fn value_mut(&mut self) -> &mut ResourceAnnotationValue {
        &mut self.value
    }

    /// Consumes the field and returns its components.
    pub fn into_parts(self) -> (String, ResourceAnnotationValue) {
        (self.name, self.value)
    }
}

/* ------------------------------------------------------------------------- */
/* Annotation origin                                                         */
/* ------------------------------------------------------------------------- */

/// Describes where an annotation originated.
///
/// This is source/AST provenance only. It must not be interpreted as an
/// execution backend.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceAnnotationOrigin {
    /// Written directly by the programmer.
    Source,

    /// Produced by a compiler transformation.
    Generated,

    /// Produced by an imported external representation.
    Imported,

    /// Produced during macro or source expansion.
    Expanded,
}

/* ------------------------------------------------------------------------- */
/* Resource annotation                                                       */
/* ------------------------------------------------------------------------- */

/// A generic source-level annotation attached to a resource AST construct.
///
/// `ResourceAnnotation` is intentionally generic enough to represent
/// annotations for quantum resources, classical resources, accelerators,
/// distributed resources, future computational resources, and user-defined
/// extensions.
///
/// It does not determine what a resource physically is.
///
/// # Ownership
///
/// This type owns:
///
/// - annotation identity;
/// - optional annotation value;
/// - source/provenance metadata;
/// - source-order position where supplied by the parser.
///
/// # Non-ownership
///
/// This type does NOT own:
///
/// - physical resource allocation;
/// - resource availability;
/// - hardware topology;
/// - scheduling;
/// - routing;
/// - calibration;
/// - QEC;
/// - backend execution;
/// - semantic type information;
/// - capability satisfaction;
/// - runtime state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceAnnotation {
    id: ResourceAnnotationId,
    value: ResourceAnnotationValue,
    origin: ResourceAnnotationOrigin,
}

impl ResourceAnnotation {
    /// Creates a new source-level resource annotation.
    pub fn new(
        id: ResourceAnnotationId,
        value: ResourceAnnotationValue,
    ) -> Self {
        Self {
            id,
            value,
            origin: ResourceAnnotationOrigin::Source,
        }
    }

    /// Creates an annotation with explicit provenance.
    pub fn with_origin(
        id: ResourceAnnotationId,
        value: ResourceAnnotationValue,
        origin: ResourceAnnotationOrigin,
    ) -> Self {
        Self {
            id,
            value,
            origin,
        }
    }

    /// Returns the stable annotation identity.
    pub fn id(&self) -> &ResourceAnnotationId {
        &self.id
    }

    /// Returns the annotation value.
    pub fn value(&self) -> &ResourceAnnotationValue {
        &self.value
    }

    /// Returns the annotation value mutably.
    pub fn value_mut(&mut self) -> &mut ResourceAnnotationValue {
        &mut self.value
    }

    /// Returns the annotation origin.
    pub fn origin(&self) -> &ResourceAnnotationOrigin {
        &self.origin
    }

    /// Changes the annotation value without changing its identity.
    pub fn set_value(&mut self, value: ResourceAnnotationValue) {
        self.value = value;
    }

    /// Returns the canonical annotation identity.
    pub fn canonical_name(&self) -> String {
        self.id.canonical_name()
    }

    /// Returns whether the annotation is compiler-generated.
    pub fn is_generated(&self) -> bool {
        matches!(self.origin, ResourceAnnotationOrigin::Generated)
    }

    /// Returns whether the annotation came directly from source.
    pub fn is_source(&self) -> bool {
        matches!(self.origin, ResourceAnnotationOrigin::Source)
    }
}

/* ------------------------------------------------------------------------- */
/* Annotation collections                                                    */
/* ------------------------------------------------------------------------- */

/// Ordered resource annotations.
///
/// A dedicated collection type gives the AST a stable API without exposing
/// implementation details of the underlying storage.
///
/// The collection intentionally preserves insertion/source order. This is
/// important for deterministic diagnostics, source-preserving tooling and
/// reproducible serialization.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResourceAnnotations {
    entries: Vec<ResourceAnnotation>,
}

impl ResourceAnnotations {
    /// Creates an empty collection.
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Returns the number of annotations.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Adds an annotation.
    ///
    /// Duplicate identities are permitted at the AST level because duplicate
    /// source annotations can be meaningful for diagnostics and because the
    /// semantic layer is the correct place to determine whether a particular
    /// annotation namespace permits repetition.
    pub fn push(&mut self, annotation: ResourceAnnotation) {
        self.entries.push(annotation);
    }

    /// Returns annotations in source order.
    pub fn iter(&self) -> impl Iterator<Item = &ResourceAnnotation> {
        self.entries.iter()
    }

    /// Returns mutable annotations in source order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut ResourceAnnotation> {
        self.entries.iter_mut()
    }

    /// Returns the first annotation with the requested identity.
    pub fn get(&self, id: &ResourceAnnotationId) -> Option<&ResourceAnnotation> {
        self.entries.iter().find(|annotation| annotation.id() == id)
    }

    /// Returns all annotations with the requested identity.
    pub fn get_all(
        &self,
        id: &ResourceAnnotationId,
    ) -> impl Iterator<Item = &ResourceAnnotation> {
        self.entries
            .iter()
            .filter(move |annotation| annotation.id() == id)
    }

    /// Returns an owned iterator.
    pub fn into_iter(self) -> impl Iterator<Item = ResourceAnnotation> {
        self.entries.into_iter()
    }

    /// Clears all annotations.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Validates the structural invariants of the collection.
    ///
    /// Semantic duplicate detection deliberately does not occur here.
    pub fn validate(&self) -> Result<(), ResourceAnnotationValidationError> {
        for annotation in &self.entries {
            validate_annotation(annotation)?;
        }

        Ok(())
    }
}

impl<'a> IntoIterator for &'a ResourceAnnotations {
    type Item = &'a ResourceAnnotation;
    type IntoIter = std::slice::Iter<'a, ResourceAnnotation>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

impl<'a> IntoIterator for &'a mut ResourceAnnotations {
    type Item = &'a mut ResourceAnnotation;
    type IntoIter = std::slice::IterMut<'a, ResourceAnnotation>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter_mut()
    }
}

impl IntoIterator for ResourceAnnotations {
    type Item = ResourceAnnotation;
    type IntoIter = std::vec::IntoIter<ResourceAnnotation>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

/* ------------------------------------------------------------------------- */
/* Structural validation                                                     */
/* ------------------------------------------------------------------------- */

fn validate_annotation(
    annotation: &ResourceAnnotation,
) -> Result<(), ResourceAnnotationValidationError> {
    validate_identifier_component(
        "namespace",
        annotation.id.namespace(),
    )
    .map_err(ResourceAnnotationValidationError::InvalidId)?;

    validate_identifier_component(
        "name",
        annotation.id.name(),
    )
    .map_err(ResourceAnnotationValidationError::InvalidId)?;

    validate_value(&annotation.value)?;

    Ok(())
}

fn validate_value(
    value: &ResourceAnnotationValue,
) -> Result<(), ResourceAnnotationValidationError> {
    match value {
        ResourceAnnotationValue::Unit
        | ResourceAnnotationValue::Bool(_)
        | ResourceAnnotationValue::Expression(_) => Ok(()),

        ResourceAnnotationValue::Integer(value) => {
            if value.is_empty() {
                return Err(ResourceAnnotationValidationError::EmptyNumericLiteral);
            }

            if value.chars().any(char::is_control) {
                return Err(
                    ResourceAnnotationValidationError::InvalidNumericLiteral(
                        value.clone(),
                    ),
                );
            }

            Ok(())
        }

        ResourceAnnotationValue::Real(value) => {
            if value.is_empty() {
                return Err(ResourceAnnotationValidationError::EmptyNumericLiteral);
            }

            if value.chars().any(char::is_control) {
                return Err(
                    ResourceAnnotationValidationError::InvalidNumericLiteral(
                        value.clone(),
                    ),
                );
            }

            Ok(())
        }

        ResourceAnnotationValue::String(_)
        | ResourceAnnotationValue::Symbol(_) => Ok(()),

        ResourceAnnotationValue::List(values) => {
            for value in values {
                validate_value(value)?;
            }

            Ok(())
        }

        ResourceAnnotationValue::Object(fields) => {
            for field in fields {
                validate_identifier_component("field", field.name())
                    .map_err(ResourceAnnotationValidationError::InvalidId)?;

                validate_value(field.value())?;
            }

            Ok(())
        }
    }
}

/* ------------------------------------------------------------------------- */
/* Errors                                                                    */
/* ------------------------------------------------------------------------- */

/// Errors produced while constructing an annotation identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceAnnotationIdError {
    /// Namespace or name was empty.
    EmptyComponent {
        field: &'static str,
    },

    /// Component contained leading/trailing whitespace.
    Whitespace {
        field: &'static str,
        value: String,
    },

    /// Component contained a control character.
    ControlCharacter {
        field: &'static str,
        value: String,
    },
}

impl fmt::Display for ResourceAnnotationIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyComponent { field } => {
                write!(f, "resource annotation {field} cannot be empty")
            }

            Self::Whitespace { field, .. } => {
                write!(
                    f,
                    "resource annotation {field} cannot contain leading or trailing whitespace"
                )
            }

            Self::ControlCharacter { field, .. } => {
                write!(
                    f,
                    "resource annotation {field} cannot contain control characters"
                )
            }
        }
    }
}

impl Error for ResourceAnnotationIdError {}

/// Errors produced while constructing an annotation object field.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceAnnotationFieldError {
    /// Field name is structurally invalid.
    InvalidName(ResourceAnnotationIdError),
}

impl fmt::Display for ResourceAnnotationFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName(error) => {
                write!(f, "invalid resource annotation field: {error}")
            }
        }
    }
}

impl Error for ResourceAnnotationFieldError {}

/// Structural validation errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceAnnotationValidationError {
    /// Annotation identity is invalid.
    InvalidId(ResourceAnnotationIdError),

    /// Numeric source representation is empty.
    EmptyNumericLiteral,

    /// Numeric source representation contains forbidden control characters.
    InvalidNumericLiteral(String),
}

impl fmt::Display for ResourceAnnotationValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidId(error) => {
                write!(f, "invalid resource annotation identity: {error}")
            }

            Self::EmptyNumericLiteral => {
                write!(f, "resource annotation numeric literal cannot be empty")
            }

            Self::InvalidNumericLiteral(value) => {
                write!(
                    f,
                    "resource annotation numeric literal contains invalid characters: {value:?}"
                )
            }
        }
    }
}

impl Error for ResourceAnnotationValidationError {}

/* ------------------------------------------------------------------------- */
/* Tests                                                                     */
/* ------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_namespaced_annotation_identity() {
        let id = ResourceAnnotationId::new("core", "resource")
            .expect("valid annotation identity");

        assert_eq!(id.namespace(), "core");
        assert_eq!(id.name(), "resource");
        assert_eq!(id.canonical_name(), "core::resource");
    }

    #[test]
    fn rejects_empty_namespace() {
        let result = ResourceAnnotationId::new("", "resource");

        assert!(matches!(
            result,
            Err(ResourceAnnotationIdError::EmptyComponent {
                field: "namespace"
            })
        ));
    }

    #[test]
    fn rejects_empty_name() {
        let result = ResourceAnnotationId::new("core", "");

        assert!(matches!(
            result,
            Err(ResourceAnnotationIdError::EmptyComponent {
                field: "name"
            })
        ));
    }

    #[test]
    fn rejects_control_characters() {
        let result = ResourceAnnotationId::new("core", "resource\n");

        assert!(matches!(
            result,
            Err(ResourceAnnotationIdError::ControlCharacter {
                field: "name",
                ..
            })
        ));
    }

    #[test]
    fn preserves_large_integer_lexically() {
        let value = ResourceAnnotationValue::integer(
            "999999999999999999999999999999999999999999999999999999",
        );

        match value {
            ResourceAnnotationValue::Integer(value) => {
                assert_eq!(
                    value,
                    "999999999999999999999999999999999999999999999999999999"
                );
            }

            _ => panic!("expected integer"),
        }
    }

    #[test]
    fn supports_symbolic_resource_information() {
        let value = ResourceAnnotationValue::symbol("N");

        assert_eq!(value, ResourceAnnotationValue::Symbol("N".to_owned()));
    }

    #[test]
    fn supports_arbitrarily_sized_annotation_lists() {
        let mut annotations = ResourceAnnotations::new();

        for index in 0..10_000 {
            let id = ResourceAnnotationId::new("test", format!("annotation_{index}"))
                .expect("valid identity");

            annotations.push(ResourceAnnotation::new(
                id,
                ResourceAnnotationValue::Unit,
            ));
        }

        assert_eq!(annotations.len(), 10_000);
        assert!(!annotations.is_empty());
        assert!(annotations.validate().is_ok());
    }

    #[test]
    fn preserves_annotation_order() {
        let mut annotations = ResourceAnnotations::new();

        for name in ["first", "second", "third"] {
            annotations.push(ResourceAnnotation::new(
                ResourceAnnotationId::new("test", name).expect("valid identity"),
                ResourceAnnotationValue::Unit,
            ));
        }

        let names: Vec<&str> = annotations.iter().map(|a| a.id().name()).collect();

        assert_eq!(names, vec!["first", "second", "third"]);
    }

    #[test]
    fn permits_duplicate_source_annotations() {
        let mut annotations = ResourceAnnotations::new();

        let id =
            ResourceAnnotationId::new("test", "repeat").expect("valid identity");

        annotations.push(ResourceAnnotation::new(
            id.clone(),
            ResourceAnnotationValue::Bool(true),
        ));

        annotations.push(ResourceAnnotation::new(
            id.clone(),
            ResourceAnnotationValue::Bool(false),
        ));

        assert_eq!(annotations.get_all(&id).count(), 2);
        assert!(annotations.validate().is_ok());
    }

    #[test]
    fn validates_nested_values() {
        let value = ResourceAnnotationValue::Object(vec![
            ResourceAnnotationField::new(
                "count",
                ResourceAnnotationValue::symbol("N"),
            )
            .expect("valid field"),
            ResourceAnnotationField::new(
                "enabled",
                ResourceAnnotationValue::Bool(true),
            )
            .expect("valid field"),
        ]);

        let annotation = ResourceAnnotation::new(
            ResourceAnnotationId::new("core", "resource").expect("valid ID"),
            value,
        );

        assert!(ResourceAnnotations {
            entries: vec![annotation],
        }
        .validate()
        .is_ok());
    }

    #[test]
    fn generated_origin_is_preserved() {
        let annotation = ResourceAnnotation::with_origin(
            ResourceAnnotationId::new("compiler", "generated")
                .expect("valid ID"),
            ResourceAnnotationValue::Unit,
            ResourceAnnotationOrigin::Generated,
        );

        assert!(annotation.is_generated());
        assert!(!annotation.is_source());
    }

    #[test]
    fn node_reference_is_opaque_and_copyable() {
        let reference = ResourceAnnotationNodeRef::new(42);

        assert_eq!(reference.get(), 42);
        assert_eq!(reference, reference);
    }

    #[test]
    fn validation_rejects_empty_integer() {
        let annotation = ResourceAnnotation::new(
            ResourceAnnotationId::new("test", "value").expect("valid ID"),
            ResourceAnnotationValue::Integer(String::new()),
        );

        let annotations = ResourceAnnotations {
            entries: vec![annotation],
        };

        assert!(matches!(
            annotations.validate(),
            Err(ResourceAnnotationValidationError::EmptyNumericLiteral)
        ));
    }
}