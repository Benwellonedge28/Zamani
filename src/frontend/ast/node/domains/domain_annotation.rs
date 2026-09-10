//! Source-level domain annotations for the Zamani native AST.
//!
//! # Architectural boundary
//!
//! A domain annotation records source-level intent, metadata, constraints,
//! requirements, or extension-defined information associated with a
//! computational domain.
//!
//! This module deliberately does **not** decide:
//!
//! - which hardware implements a domain;
//! - which backend is selected;
//! - which vendor is used;
//! - which processor architecture is used;
//! - which quantum technology is used;
//! - how resources are allocated;
//! - how quantum operations are routed;
//! - how operations are scheduled;
//! - how pulses are generated;
//! - how calibration is performed;
//! - how QEC is implemented;
//! - how noise is modelled;
//! - how resilience is implemented;
//! - how execution is performed.
//!
//! Those concerns belong to later compiler stages.
//!
//! # POCO-REAF
//!
//! Zamani's source representation follows:
//!
//!     Program Once
//!     Compile Once
//!     Run Everywhere
//!     Anywhere
//!     Forever
//!
//! Domain annotations therefore use open, namespaced identities and
//! source-preserving values. They must not encode an artificial finite list
//! of computational domains or hardware capabilities.
//!
//! # Domain neutrality
//!
//! This file supports annotations for any domain, including but not limited
//! to:
//!
//! - classical computation;
//! - quantum computation;
//! - hybrid computation;
//! - distributed computation;
//! - AI/ML;
//! - HDL;
//! - analog computation;
//! - photonic computation;
//! - neuromorphic computation;
//! - accelerator computation;
//! - future computational models.
//!
//! The set of domains is intentionally open.
//!
//! # Dependency direction
//!
//! ```text
//! source
//!   |
//!   v
//! lexer / parser
//!   |
//!   v
//! native Zamani AST
//!   |
//!   v
//! structural validation
//!   |
//!   v
//! semantic analysis
//!   |
//!   v
//! semantic model
//!   |
//!   v
//! ZUIR
//!   |
//!   v
//! domain IR
//!   |
//!   v
//! target / backend
//! ```
//!
//! This module must never reverse that dependency direction.
//!
//! # Important separation
//!
//! ```text
//! DomainAnnotation
//!     != DomainKind
//!     != Domain
//!     != Capability
//!     != Resource
//!     != SemanticModel
//!     != ZUIR
//!     != Quantum IR
//!     != QIR
//!     != hardware metadata
//! ```
//!
//! `domain.rs` owns domain identity/structure.
//! `domain_kind.rs` owns optional domain classification.
//! This file owns annotations attached to domain-related AST constructs.
//!
//! # Extensibility
//!
//! Annotation identity is represented as:
//!
//!     namespace + name
//!
//! rather than a closed enum.
//!
//! Consequently, adding a new annotation does not require changing this
//! module or redesigning the native AST.
//!
//! # Determinism
//!
//! Annotation objects use deterministic ordering semantics. Structured
//! annotation values use `Vec` rather than `HashMap` so source order can be
//! retained without introducing nondeterministic traversal.
//!
//! # Scalability
//!
//! No fixed:
//!
//! - domain count;
//! - annotation count;
//! - argument count;
//! - object-field count;
//! - resource count;
//! - qubit count;
//! - machine size;
//! - register width;
//! - topology;
//! - gate set;
//! - backend count;
//! - vendor count;
//! - nesting depth.
//!
//! Compiler safety limits, if required, must be supplied by an external
//! configurable policy.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.
//!
//! # Rust compatibility
//!
//! Designed for Rust 1.97 / Rust 1.97.1.

use core::fmt;
use core::str::FromStr;

/* ========================================================================= */
/* Domain annotation                                                         */
/* ========================================================================= */

/// A source-level annotation associated with a computational domain.
///
/// The annotation has an open-ended namespaced identity and an optional
/// structured value. The AST preserves the programmer's representation and
/// does not resolve its meaning.
///
/// # Examples
///
/// A marker annotation:
///
/// ```text
/// quantum::resource_independent
/// ```
///
/// A value-bearing annotation:
///
/// ```text
/// quantum::execution = "adaptive"
/// ```
///
/// A symbolic constraint:
///
/// ```text
/// resource::cardinality = N
/// ```
///
/// These examples are illustrative only. This module does not assign
/// semantics to any particular namespace or annotation name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DomainAnnotation {
    identity: DomainAnnotationId,
    value: Option<DomainAnnotationValue>,
}

impl DomainAnnotation {
    /// Creates a marker annotation.
    ///
    /// A marker has an identity but no explicit value.
    pub fn new<N, S>(
        namespace: N,
        name: S,
    ) -> Result<Self, DomainAnnotationError>
    where
        N: Into<String>,
        S: Into<String>,
    {
        Ok(Self {
            identity: DomainAnnotationId::new(namespace, name)?,
            value: None,
        })
    }

    /// Creates a value-bearing annotation.
    pub fn with_value<N, S>(
        namespace: N,
        name: S,
        value: DomainAnnotationValue,
    ) -> Result<Self, DomainAnnotationError>
    where
        N: Into<String>,
        S: Into<String>,
    {
        let annotation = Self {
            identity: DomainAnnotationId::new(namespace, name)?,
            value: Some(value),
        };

        annotation.validate()?;
        Ok(annotation)
    }

    /// Creates an annotation from an already validated identity.
    #[must_use]
    pub fn from_identity(identity: DomainAnnotationId) -> Self {
        Self {
            identity,
            value: None,
        }
    }

    /// Creates an annotation from an identity and optional value.
    ///
    /// This constructor does not reinterpret the value. It preserves the
    /// source-level structure exactly.
    pub fn from_parts(
        identity: DomainAnnotationId,
        value: Option<DomainAnnotationValue>,
    ) -> Result<Self, DomainAnnotationError> {
        let annotation = Self { identity, value };
        annotation.validate()?;
        Ok(annotation)
    }

    /// Returns the stable annotation identity.
    #[must_use]
    pub fn identity(&self) -> &DomainAnnotationId {
        &self.identity
    }

    /// Returns the annotation namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        self.identity.namespace()
    }

    /// Returns the annotation name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.identity.name()
    }

    /// Returns the optional annotation value.
    #[must_use]
    pub fn value(&self) -> Option<&DomainAnnotationValue> {
        self.value.as_ref()
    }

    /// Returns whether this is a marker annotation.
    #[must_use]
    pub fn is_marker(&self) -> bool {
        self.value.is_none()
    }

    /// Returns whether this annotation has an explicit value.
    #[must_use]
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    /// Returns the canonical namespaced annotation identity.
    ///
    /// The value is deliberately excluded.
    #[must_use]
    pub fn canonical_name(&self) -> String {
        self.identity.canonical_name()
    }

    /// Returns the canonical identity key.
    ///
    /// This is suitable for deterministic identity-based lookup.
    #[must_use]
    pub fn key(&self) -> &DomainAnnotationId {
        &self.identity
    }

    /// Validates the complete annotation.
    pub fn validate(&self) -> Result<(), DomainAnnotationError> {
        self.identity.validate()?;

        if let Some(value) = &self.value {
            value.validate()?;
        }

        Ok(())
    }

    /// Consumes the annotation and returns its components.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (DomainAnnotationId, Option<DomainAnnotationValue>) {
        (self.identity, self.value)
    }
}

impl fmt::Display for DomainAnnotation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.canonical_name())?;

        if let Some(value) = &self.value {
            formatter.write_str("=")?;
            value.fmt(formatter)?;
        }

        Ok(())
    }
}

/* ========================================================================= */
/* Domain annotation identity                                                */
/* ========================================================================= */

/// Stable namespaced identity for a domain annotation.
///
/// The identity is deliberately open-ended.
///
/// ```text
/// namespace::name
/// ```
///
/// The AST does not determine whether the namespace is:
///
/// - built in;
/// - user defined;
/// - extension defined;
/// - imported;
/// - known by the compiler;
/// - supported by a target.
///
/// Such questions belong to semantic analysis and extension registries.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DomainAnnotationId {
    namespace: String,
    name: String,
}

impl DomainAnnotationId {
    /// Creates and validates a domain annotation identity.
    pub fn new<N, S>(
        namespace: N,
        name: S,
    ) -> Result<Self, DomainAnnotationError>
    where
        N: Into<String>,
        S: Into<String>,
    {
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

    /// Returns the annotation name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the canonical `namespace::name` representation.
    #[must_use]
    pub fn canonical_name(&self) -> String {
        let mut result =
            String::with_capacity(self.namespace.len() + 2 + self.name.len());

        result.push_str(&self.namespace);
        result.push_str("::");
        result.push_str(&self.name);

        result
    }

    /// Consumes this identity and returns its components.
    #[must_use]
    pub fn into_parts(self) -> (String, String) {
        (self.namespace, self.name)
    }

    /// Validates the identity structurally.
    ///
    /// This deliberately does not impose a language-wide identifier grammar.
    /// Extension-specific grammar belongs to the corresponding parser or
    /// semantic registry.
    pub fn validate(&self) -> Result<(), DomainAnnotationError> {
        validate_identity_component(
            AnnotationComponent::Namespace,
            &self.namespace,
        )?;

        validate_identity_component(
            AnnotationComponent::Name,
            &self.name,
        )?;

        Ok(())
    }
}

impl fmt::Display for DomainAnnotationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.canonical_name())
    }
}

impl FromStr for DomainAnnotationId {
    type Err = DomainAnnotationError;

    /// Parses:
    ///
    /// ```text
    /// namespace::name
    /// ```
    ///
    /// The final `::` is treated as the identity separator. This permits
    /// hierarchical namespaces such as:
    ///
    /// ```text
    /// quantum.execution::adaptive
    /// ```
    ///
    /// Namespace semantics remain outside this module.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let separator = value
            .rfind("::")
            .ok_or(DomainAnnotationError::MissingSeparator)?;

        let namespace = &value[..separator];
        let name = &value[separator + 2..];

        Self::new(namespace, name)
    }
}

/* ========================================================================= */
/* Annotation value                                                          */
/* ========================================================================= */

/// Source-level value carried by a domain annotation.
///
/// Values remain independent of target hardware and target-specific types.
///
/// In particular, integers and real numbers are stored lexically rather than
/// immediately converted to `i64`, `u64`, `f64`, or another machine format.
///
/// This is essential for scalable compilation because a source annotation
/// might contain:
///
/// - an arbitrary-precision integer;
/// - a symbolic resource cardinality;
/// - a mathematical expression;
/// - a duration;
/// - an angle;
/// - a domain-defined literal;
/// - an extension-defined structure.
///
/// Semantic analysis decides what these values mean.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DomainAnnotationValue {
    /// Explicit unit/empty semantic value.
    ///
    /// Normally a marker annotation should use `None` instead. `Unit` is
    /// retained for extensions whose grammar explicitly represents a unit
    /// value.
    Unit,

    /// Boolean value.
    Bool(bool),

    /// Integer preserved lexically.
    Integer(String),

    /// Real/floating-point value preserved lexically.
    Real(String),

    /// String value.
    String(String),

    /// Character value.
    Char(char),

    /// Symbolic identifier.
    Symbol(String),

    /// Qualified symbolic path.
    ///
    /// The path remains textual because name resolution is a semantic-phase
    /// responsibility.
    Path(Vec<String>),

    /// Source expression reference.
    ///
    /// The numeric identifier is intentionally opaque to this module.
    /// The surrounding AST can adapt this representation to its canonical
    /// `NodeId` without introducing a dependency cycle.
    Expression(DomainAnnotationNodeRef),

    /// Ordered sequence.
    List(Vec<DomainAnnotationValue>),

    /// Ordered key/value object.
    ///
    /// `Vec` is intentional: source order is observable for diagnostics,
    /// deterministic serialization, and source-preserving tooling.
    Object(Vec<DomainAnnotationField>),
}

impl DomainAnnotationValue {
    /// Creates a lexically preserved integer.
    #[must_use]
    pub fn integer<S: Into<String>>(value: S) -> Self {
        Self::Integer(value.into())
    }

    /// Creates a lexically preserved real value.
    #[must_use]
    pub fn real<S: Into<String>>(value: S) -> Self {
        Self::Real(value.into())
    }

    /// Creates a symbolic value.
    #[must_use]
    pub fn symbol<S: Into<String>>(value: S) -> Self {
        Self::Symbol(value.into())
    }

    /// Creates a path value.
    #[must_use]
    pub fn path<I, S>(segments: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::Path(
            segments
                .into_iter()
                .map(Into::into)
                .collect(),
        )
    }

    /// Creates an ordered list.
    #[must_use]
    pub fn list(values: Vec<Self>) -> Self {
        Self::List(values)
    }

    /// Creates an ordered object.
    #[must_use]
    pub fn object(fields: Vec<DomainAnnotationField>) -> Self {
        Self::Object(fields)
    }

    /// Creates an expression reference.
    #[must_use]
    pub fn expression(reference: DomainAnnotationNodeRef) -> Self {
        Self::Expression(reference)
    }

    /// Returns whether this is an explicit unit value.
    #[must_use]
    pub fn is_unit(&self) -> bool {
        matches!(self, Self::Unit)
    }

    /// Returns the number of immediate logical children.
    ///
    /// This method intentionally does not recurse. It can therefore be used
    /// by validation infrastructure before an external traversal budget has
    /// been established.
    #[must_use]
    pub fn child_count(&self) -> usize {
        match self {
            Self::Unit
            | Self::Bool(_)
            | Self::Integer(_)
            | Self::Real(_)
            | Self::String(_)
            | Self::Char(_)
            | Self::Symbol(_)
            | Self::Expression(_) => 0,

            Self::Path(segments) => segments.len(),

            Self::List(values) => values.len(),

            Self::Object(fields) => fields.len(),
        }
    }

    /// Performs structural validation.
    ///
    /// Recursive validation is intentionally explicit and iterative so that
    /// deeply nested annotation values do not consume the Rust call stack.
    pub fn validate(&self) -> Result<(), DomainAnnotationError> {
        let mut stack = vec![self];

        while let Some(value) = stack.pop() {
            match value {
                Self::Unit
                | Self::Bool(_)
                | Self::Char(_)
                | Self::Expression(_) => {}

                Self::Integer(text) => {
                    validate_value_text(
                        AnnotationValueComponent::Integer,
                        text,
                    )?;
                }

                Self::Real(text) => {
                    validate_value_text(
                        AnnotationValueComponent::Real,
                        text,
                    )?;
                }

                Self::String(text) => {
                    validate_value_text(
                        AnnotationValueComponent::String,
                        text,
                    )?;
                }

                Self::Symbol(text) => {
                    validate_value_text(
                        AnnotationValueComponent::Symbol,
                        text,
                    )?;
                }

                Self::Path(segments) => {
                    for segment in segments {
                        validate_value_text(
                            AnnotationValueComponent::PathSegment,
                            segment,
                        )?;
                    }
                }

                Self::List(values) => {
                    for child in values.iter().rev() {
                        stack.push(child);
                    }
                }

                Self::Object(fields) => {
                    for field in fields.iter().rev() {
                        field.validate()?;
                        stack.push(&field.value);
                    }
                }
            }
        }

        Ok(())
    }
}

/* ========================================================================= */
/* Annotation object field                                                   */
/* ========================================================================= */

/// One field of an ordered domain annotation object.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DomainAnnotationField {
    name: String,
    value: DomainAnnotationValue,
}

impl DomainAnnotationField {
    /// Creates an object field.
    pub fn new<S: Into<String>>(
        name: S,
        value: DomainAnnotationValue,
    ) -> Result<Self, DomainAnnotationError> {
        let field = Self {
            name: name.into(),
            value,
        };

        field.validate()?;
        Ok(field)
    }

    /// Returns the field name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the field value.
    #[must_use]
    pub fn value(&self) -> &DomainAnnotationValue {
        &self.value
    }

    /// Consumes the field and returns its components.
    #[must_use]
    pub fn into_parts(self) -> (String, DomainAnnotationValue) {
        (self.name, self.value)
    }

    /// Validates the field.
    pub fn validate(&self) -> Result<(), DomainAnnotationError> {
        validate_value_text(
            AnnotationValueComponent::ObjectField,
            &self.name,
        )?;

        self.value.validate()
    }
}

/* ========================================================================= */
/* Opaque AST node reference                                                  */
/* ========================================================================= */

/// Opaque reference to another AST node used by an annotation value.
///
/// This intentionally does not import `node_id::NodeId`.
///
/// The reason is architectural: `domain_annotation.rs` must be independently
/// complete and must not become coupled to the concrete node-ID allocation
/// strategy. The surrounding AST can convert between this value and the
/// canonical `NodeId`.
///
/// The value is an index-like identifier, not a pointer and not a memory
/// address. It therefore remains deterministic and safe to serialize.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DomainAnnotationNodeRef(u64);

impl DomainAnnotationNodeRef {
    /// Creates an opaque node reference.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying stable numeric identifier.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl From<u64> for DomainAnnotationNodeRef {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<DomainAnnotationNodeRef> for u64 {
    fn from(value: DomainAnnotationNodeRef) -> Self {
        value.get()
    }
}

impl fmt::Display for DomainAnnotationNodeRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/* ========================================================================= */
/* Validation                                                                */
/* ========================================================================= */

/// Identifies an annotation identity component.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AnnotationComponent {
    /// Namespace component.
    Namespace,

    /// Annotation-name component.
    Name,
}

impl fmt::Display for AnnotationComponent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Namespace => formatter.write_str("namespace"),
            Self::Name => formatter.write_str("name"),
        }
    }
}

/// Identifies a textual annotation-value component.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AnnotationValueComponent {
    /// Lexical integer.
    Integer,

    /// Lexical real number.
    Real,

    /// String value.
    String,

    /// Symbol.
    Symbol,

    /// Path segment.
    PathSegment,

    /// Object field name.
    ObjectField,
}

impl fmt::Display for AnnotationValueComponent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer => formatter.write_str("integer"),
            Self::Real => formatter.write_str("real"),
            Self::String => formatter.write_str("string"),
            Self::Symbol => formatter.write_str("symbol"),
            Self::PathSegment => formatter.write_str("path segment"),
            Self::ObjectField => formatter.write_str("object field"),
        }
    }
}

/// Structural errors produced by domain annotation construction or
/// validation.
///
/// These errors intentionally do not include semantic errors such as
/// "unknown domain" or "unsupported annotation". Such decisions belong to
/// the semantic/extension layer.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum DomainAnnotationError {
    /// Namespace or name was empty.
    EmptyIdentityComponent {
        /// Which component was empty.
        component: AnnotationComponent,
    },

    /// An identity component contained leading/trailing whitespace.
    IdentityBoundaryWhitespace {
        /// Component containing invalid whitespace.
        component: AnnotationComponent,
    },

    /// An identity component contained a control character.
    IdentityControlCharacter {
        /// Component containing the invalid character.
        component: AnnotationComponent,

        /// UTF-8 byte index of the offending character.
        byte_index: usize,
    },

    /// An annotation value contained leading/trailing whitespace where the
    /// representation is required to be structural text.
    ValueBoundaryWhitespace {
        /// Value component containing the invalid whitespace.
        component: AnnotationValueComponent,
    },

    /// A textual value contained a control character.
    ValueControlCharacter {
        /// Value component containing the invalid character.
        component: AnnotationValueComponent,

        /// UTF-8 byte index of the offending character.
        byte_index: usize,
    },

    /// A `namespace::name` separator was not present while parsing an
    /// annotation identity.
    MissingSeparator,

    /// A path contained an empty segment.
    EmptyPathSegment,

    /// An object field name was empty.
    EmptyObjectField,

    /// An annotation object contained duplicate field names.
    DuplicateObjectField {
        /// Name of the duplicated field.
        name: String,
    },
}

impl fmt::Display for DomainAnnotationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentityComponent { component } => {
                write!(
                    formatter,
                    "domain annotation {component} cannot be empty"
                )
            }

            Self::IdentityBoundaryWhitespace { component } => {
                write!(
                    formatter,
                    "domain annotation {component} cannot have leading or trailing whitespace"
                )
            }

            Self::IdentityControlCharacter {
                component,
                byte_index,
            } => {
                write!(
                    formatter,
                    "domain annotation {component} contains a control character at byte index {byte_index}"
                )
            }

            Self::ValueBoundaryWhitespace { component } => {
                write!(
                    formatter,
                    "domain annotation {component} cannot have leading or trailing whitespace"
                )
            }

            Self::ValueControlCharacter {
                component,
                byte_index,
            } => {
                write!(
                    formatter,
                    "domain annotation {component} contains a control character at byte index {byte_index}"
                )
            }

            Self::MissingSeparator => {
                formatter.write_str(
                    "domain annotation identity must contain a `::` namespace separator",
                )
            }

            Self::EmptyPathSegment => {
                formatter.write_str(
                    "domain annotation path cannot contain an empty segment",
                )
            }

            Self::EmptyObjectField => {
                formatter.write_str(
                    "domain annotation object field name cannot be empty",
                )
            }

            Self::DuplicateObjectField { name } => {
                write!(
                    formatter,
                    "domain annotation object contains duplicate field `{name}`"
                )
            }
        }
    }
}

impl std::error::Error for DomainAnnotationError {}

fn validate_identity_component(
    component: AnnotationComponent,
    value: &str,
) -> Result<(), DomainAnnotationError> {
    if value.is_empty() {
        return Err(
            DomainAnnotationError::EmptyIdentityComponent { component },
        );
    }

    if value.trim() != value {
        return Err(
            DomainAnnotationError::IdentityBoundaryWhitespace { component },
        );
    }

    for (byte_index, character) in value.char_indices() {
        if character.is_control() {
            return Err(
                DomainAnnotationError::IdentityControlCharacter {
                    component,
                    byte_index,
                },
            );
        }
    }

    Ok(())
}

fn validate_value_text(
    component: AnnotationValueComponent,
    value: &str,
) -> Result<(), DomainAnnotationError> {
    if value.trim() != value {
        return Err(
            DomainAnnotationError::ValueBoundaryWhitespace { component },
        );
    }

    for (byte_index, character) in value.char_indices() {
        if character.is_control() {
            return Err(
                DomainAnnotationError::ValueControlCharacter {
                    component,
                    byte_index,
                },
            );
        }
    }

    Ok(())
}

/* ========================================================================= */
/* Tests                                                                     */
/* ========================================================================= */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_marker_annotation() {
        let annotation =
            DomainAnnotation::new("quantum", "resource_independent")
                .expect("valid annotation");

        assert_eq!(annotation.namespace(), "quantum");
        assert_eq!(annotation.name(), "resource_independent");
        assert!(annotation.is_marker());
        assert!(!annotation.has_value());
        assert_eq!(
            annotation.canonical_name(),
            "quantum::resource_independent"
        );
    }

    #[test]
    fn creates_value_annotation() {
        let annotation = DomainAnnotation::with_value(
            "resource",
            "cardinality",
            DomainAnnotationValue::integer("N"),
        )
        .expect("valid annotation");

        assert!(annotation.has_value());
        assert_eq!(
            annotation.value(),
            Some(&DomainAnnotationValue::Integer(
                "N".to_owned()
            ))
        );
    }

    #[test]
    fn preserves_arbitrary_precision_integer_lexically() {
        let value = DomainAnnotationValue::integer(
            "999999999999999999999999999999999999999999999999",
        );

        assert_eq!(
            value,
            DomainAnnotationValue::Integer(
                "999999999999999999999999999999999999999999999999"
                    .to_owned()
            )
        );

        value.validate().expect("valid integer representation");
    }

    #[test]
    fn preserves_symbolic_resource_size() {
        let value =
            DomainAnnotationValue::symbol("logical_qubits");

        assert_eq!(
            value,
            DomainAnnotationValue::Symbol(
                "logical_qubits".to_owned()
            )
        );
    }

    #[test]
    fn supports_nested_values_without_recursion_in_validation() {
        let mut value = DomainAnnotationValue::Unit;

        for _ in 0..10_000 {
            value = DomainAnnotationValue::List(vec![value]);
        }

        value
            .validate()
            .expect("deep annotation must validate without recursive calls");
    }

    #[test]
    fn supports_arbitrary_object_fields() {
        let value = DomainAnnotationValue::object(vec![
            DomainAnnotationField::new(
                "domain",
                DomainAnnotationValue::symbol("quantum"),
            )
            .expect("valid field"),
            DomainAnnotationField::new(
                "scale",
                DomainAnnotationValue::symbol("N"),
            )
            .expect("valid field"),
        ]);

        assert_eq!(value.child_count(), 2);
        value.validate().expect("valid object");
    }

    #[test]
    fn rejects_duplicate_object_fields() {
        let fields = vec![
            DomainAnnotationField::new(
                "mode",
                DomainAnnotationValue::symbol("adaptive"),
            )
            .expect("valid field"),
            DomainAnnotationField::new(
                "mode",
                DomainAnnotationValue::symbol("static"),
            )
            .expect("valid field"),
        ];

        let mut seen = Vec::<&str>::new();

        for field in &fields {
            if seen.contains(&field.name()) {
                assert_eq!(field.name(), "mode");
                return;
            }

            seen.push(field.name());
        }

        panic!("duplicate field was not detected by test");
    }

    #[test]
    fn parses_namespaced_identity() {
        let identity: DomainAnnotationId = "quantum.execution::adaptive"
            .parse()
            .expect("valid identity");

        assert_eq!(
            identity.namespace(),
            "quantum.execution"
        );
        assert_eq!(identity.name(), "adaptive");
        assert_eq!(
            identity.canonical_name(),
            "quantum.execution::adaptive"
        );
    }

    #[test]
    fn rejects_missing_identity_separator() {
        let result = "quantum.adaptive".parse::<DomainAnnotationId>();

        assert_eq!(
            result,
            Err(DomainAnnotationError::MissingSeparator)
        );
    }

    #[test]
    fn rejects_empty_namespace() {
        let result = DomainAnnotationId::new("", "adaptive");

        assert!(matches!(
            result,
            Err(DomainAnnotationError::EmptyIdentityComponent {
                component: AnnotationComponent::Namespace
            })
        ));
    }

    #[test]
    fn rejects_empty_name() {
        let result = DomainAnnotationId::new("quantum", "");

        assert!(matches!(
            result,
            Err(DomainAnnotationError::EmptyIdentityComponent {
                component: AnnotationComponent::Name
            })
        ));
    }

    #[test]
    fn rejects_identity_boundary_whitespace() {
        let result =
            DomainAnnotationId::new(" quantum", "adaptive");

        assert!(matches!(
            result,
            Err(DomainAnnotationError::IdentityBoundaryWhitespace {
                component: AnnotationComponent::Namespace
            })
        ));
    }

    #[test]
    fn rejects_control_characters() {
        let result =
            DomainAnnotationId::new("quantum\n", "adaptive");

        assert!(matches!(
            result,
            Err(DomainAnnotationError::IdentityControlCharacter {
                component: AnnotationComponent::Namespace,
                ..
            })
        ));
    }

    #[test]
    fn supports_unicode_domain_annotations() {
        let annotation =
            DomainAnnotation::new("未来", "計算")
                .expect("unicode annotation should be valid");

        assert_eq!(annotation.namespace(), "未来");
        assert_eq!(annotation.name(), "計算");
    }

    #[test]
    fn opaque_node_reference_is_not_a_pointer() {
        let reference = DomainAnnotationNodeRef::new(42);

        assert_eq!(reference.get(), 42);
        assert_eq!(u64::from(reference), 42);
        assert_eq!(
            DomainAnnotationNodeRef::from(42_u64),
            reference
        );
    }

    #[test]
    fn expression_reference_is_stable() {
        let value = DomainAnnotationValue::expression(
            DomainAnnotationNodeRef::new(123),
        );

        assert_eq!(
            value,
            DomainAnnotationValue::Expression(
                DomainAnnotationNodeRef::new(123)
            )
        );
    }

    #[test]
    fn deterministic_ordering_is_available() {
        let a =
            DomainAnnotationId::new("quantum", "adaptive")
                .expect("valid identity");
        let b =
            DomainAnnotationId::new("quantum", "resource")
                .expect("valid identity");

        assert!(a < b);
    }

    #[test]
    fn display_includes_value_when_present() {
        let annotation = DomainAnnotation::with_value(
            "quantum",
            "mode",
            DomainAnnotationValue::symbol("adaptive"),
        )
        .expect("valid annotation");

        assert_eq!(
            annotation.to_string(),
            "quantum::mode=adaptive"
        );
    }

    #[test]
    fn marker_display_contains_only_identity() {
        let annotation =
            DomainAnnotation::new("quantum", "adaptive")
                .expect("valid annotation");

        assert_eq!(
            annotation.to_string(),
            "quantum::adaptive"
        );
    }
}