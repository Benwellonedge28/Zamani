//! Zamani Native AST — Named Types
//!
//! Canonical source-level representation of named type expressions.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! lexer
//!     │
//!     ▼
//! parser
//!     │
//!     ▼
//! Native AST
//!     │
//!     ├── TypeExpr
//!     │      └── NamedType
//!     │
//!     ▼
//! structural validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! SemanticType
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── classical
//!     ├── quantum
//!     ├── HDL
//!     ├── accelerator
//!     ├── distributed
//!     └── future domains
//!     │
//!     ▼
//! target/backend lowering
//! ```
//!
//! # Responsibility
//!
//! This module owns the source-level representation of a named type.
//!
//! A named type is a source-language reference to a type identified by an
//! ordered, qualified path, optionally followed by source-level type
//! arguments and annotations.
//!
//! This module deliberately does **not** resolve the name.
//!
//! For example:
//!
//! ```text
//! Int
//! std::String
//! collections::Map<String, Int>
//! quantum::State<Q>
//! my::module::FutureType<A, B>
//! ```
//!
//! remain source-level names until semantic analysis resolves them.
//!
//! # Domain neutrality
//!
//! `NamedType` does not know whether a name ultimately denotes:
//!
//! - a classical type;
//! - a quantum type;
//! - a logical quantum resource;
//! - a hardware-independent resource;
//! - an accelerator type;
//! - an HDL type;
//! - an AI type;
//! - a distributed type;
//! - a user-defined type;
//! - an imported type;
//! - a future computational-domain type.
//!
//! A name is only a source-level identity.
//!
//! The semantic layer determines its meaning.
//!
//! # POCO-REAF
//!
//! This module contains no machine-size assumptions.
//!
//! It does not encode:
//!
//! - maximum type width;
//! - maximum generic arity;
//! - maximum resource count;
//! - maximum qubit count;
//! - maximum register size;
//! - hardware topology;
//! - vendor;
//! - backend;
//! - instruction set;
//! - physical resource identity.
//!
//! Therefore a named type can represent a source abstraction that later
//! resolves to a tiny resource or a resource of arbitrary supported scale.
//!
//! # Important distinction
//!
//! ```text
//! NamedType
//!     = source-level name
//!
//! TypeExpr
//!     = complete source-level type expression
//!
//! SemanticType
//!     = resolved type meaning
//!
//! ZUIR
//!     = universal computational representation
//!
//! Domain IR
//!     = domain-specific implementation
//!
//! Target IR
//!     = target/backend representation
//! ```
//!
//! `NamedType` must never become a semantic symbol reference or backend type.
//!
//! # Name resolution
//!
//! This module intentionally stores no resolved symbol ID.
//!
//! Do not add fields such as:
//!
//! ```text
//! symbol_id
//! resolved_type
//! definition
//! module_id
//! backend_type
//! layout
//! resource_id
//! ```
//!
//! Those belong to later compiler phases and should be maintained in semantic
//! side tables where necessary.
//!
//! # Generic arguments
//!
//! Generic arguments are represented as source-level type expressions.
//!
//! The semantic layer decides whether a particular named declaration accepts:
//!
//! - type arguments;
//! - value arguments;
//! - resource arguments;
//! - lifetime arguments;
//! - capability arguments;
//! - domain-specific arguments.
//!
//! This file does not impose a fixed generic arity.
//!
//! # External languages
//!
//! OpenQASM, QIR, Q#, Quil, Cirq and vendor-specific representations must not
//! be represented by special variants here.
//!
//! External language frontends should translate their source-level names into
//! Zamani source-level type structures.
//!
//! # Rust requirements
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Dependency policy
//!
//! This module may depend on sibling source-level AST types required to
//! represent a named type.
//!
//! It must not depend on:
//!
//! - semantic analysis;
//! - compiler driver;
//! - ZUIR;
//! - quantum backend code;
//! - hardware code;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime execution;
//! - LLVM;
//! - QIR implementation types;
//! - MLIR implementation types;
//! - vendor SDKs.
//!
//! `NamedType` must not import the legacy `crate::ast` namespace.
//!
//! # Integration contract
//!
//! ## Parser
//!
//! The parser constructs `NamedType` from a parsed type path and optional
//! generic arguments.
//!
//! The parser owns syntactic recognition.
//!
//! `NamedType` owns the resulting source-level structure.
//!
//! ## Structural validation
//!
//! Structural validation checks that:
//!
//! - the path contains at least one segment;
//! - no path segment is structurally empty;
//! - all child type expressions are structurally valid;
//! - generic argument ordering is preserved.
//!
//! ## Semantic analysis
//!
//! Semantic analysis resolves the path against the active module/import/
//! namespace environment.
//!
//! It determines whether the name refers to:
//!
//! - a primitive;
//! - a type alias;
//! - a struct;
//! - an enum;
//! - a trait-associated type;
//! - a generic parameter;
//! - an extension type;
//! - a resource type;
//! - another valid language-level type.
//!
//! ## ZUIR
//!
//! ZUIR lowering must consume the resolved semantic type rather than using
//! `NamedType` to select a backend representation.
//!
//! ## Visitors
//!
//! Visitors must treat the path as an ordered source structure and visit every
//! generic type argument exactly once and in source order.
//!
//! ## Serialization
//!
//! Serialization preserves:
//!
//! - path segment order;
//! - generic argument order;
//! - source-level annotations;
//! - schema version supplied by the surrounding AST serialization layer.
//!
//! No resolved symbol identity is serialized here.
//!
//! ## Determinism
//!
//! The representation uses ordered collections.
//!
//! No hash-map iteration determines source or semantic ordering.
//!
//! ## Scalability
//!
//! There is no artificial limit on:
//!
//! - path length;
//! - identifier length;
//! - generic arity;
//! - generic nesting;
//! - number of annotations.
//!
//! Compiler resource policies may impose configurable limits outside this
//! source representation.
//!
//! ## Security
//!
//! This module performs no I/O and no code execution.
//!
//! Malformed source structures are represented as ordinary values and rejected
//! by structural validation rather than causing undefined behavior.
//!
//! # Canonical representation
//!
//! A named type is intentionally represented as:
//!
//! ```text
//! NamedType {
//!     path,
//!     arguments,
//!     annotations
//! }
//! ```
//!
//! rather than embedding a resolved symbol.
//!
//! This preserves the source/semantic boundary.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::type_expr::{TypeExpr, TypePath};

// =============================================================================
// Schema
// =============================================================================

/// Local schema version for [`NamedType`].
///
/// This is intentionally independent of the Zamani language version and the
/// serialized AST schema version.
pub const NAMED_TYPE_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// NamedType
// =============================================================================

/// A source-level named type.
///
/// # Examples
///
/// ```text
/// Int
/// std::String
/// collections::Map<String, Int>
/// quantum::State<Q>
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NamedType {
    /// Qualified source-level type path.
    path: TypePath,

    /// Ordered source-level generic type arguments.
    ///
    /// An empty vector means that no explicit type arguments were supplied.
    arguments: Vec<TypeExpr>,
}

impl NamedType {
    /// Creates a named type from a source-level path.
    ///
    /// No semantic lookup occurs.
    #[must_use]
    pub fn new(path: TypePath) -> Self {
        Self {
            path,
            arguments: Vec::new(),
        }
    }

    /// Creates a named type from a path and ordered generic arguments.
    ///
    /// The arguments are retained exactly in source order.
    #[must_use]
    pub fn with_arguments(path: TypePath, arguments: Vec<TypeExpr>) -> Self {
        Self { path, arguments }
    }

    /// Creates a single-segment named type.
    #[must_use]
    pub fn single<S>(name: S) -> Self
    where
        S: Into<super::type_expr::TypeName>,
    {
        Self::new(TypePath::single(name))
    }

    /// Returns the unresolved source-level path.
    #[must_use]
    pub fn path(&self) -> &TypePath {
        &self.path
    }

    /// Returns the ordered generic arguments.
    #[must_use]
    pub fn arguments(&self) -> &[TypeExpr] {
        &self.arguments
    }

    /// Returns whether explicit generic arguments were supplied.
    #[must_use]
    pub fn has_arguments(&self) -> bool {
        !self.arguments.is_empty()
    }

    /// Returns the number of generic arguments.
    ///
    /// This is a collection cardinality, not a language-level maximum.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Returns the final path component.
    ///
    /// This is useful for diagnostics and parser tooling.
    ///
    /// It does **not** imply semantic resolution.
    #[must_use]
    pub fn terminal_name(&self) -> Option<&super::type_expr::TypeName> {
        self.path.last()
    }

    /// Returns the source spelling of the name.
    ///
    /// Generic arguments are rendered using their `Display` implementation.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        self.to_string()
    }

    /// Returns the number of immediate AST children owned by this named type.
    ///
    /// The path itself is not counted as a `TypeExpr` child.
    #[must_use]
    pub fn child_count(&self) -> usize {
        self.arguments.len()
    }

    /// Visits every generic type argument in source order.
    ///
    /// This is deliberately iterative at this level. Recursive traversal of
    /// nested `TypeExpr` structures belongs to the canonical AST traversal
    /// infrastructure.
    pub fn visit_arguments<F>(&self, mut visitor: F)
    where
        F: FnMut(&TypeExpr),
    {
        for argument in &self.arguments {
            visitor(argument);
        }
    }

    /// Returns whether any generic argument satisfies `predicate`.
    ///
    /// This method only examines immediate arguments. Recursive traversal is
    /// intentionally delegated to the AST traversal subsystem.
    #[must_use]
    pub fn any_argument<F>(&self, predicate: F) -> bool
    where
        F: FnMut(&TypeExpr) -> bool,
    {
        self.arguments.iter().any(predicate)
    }

    /// Returns whether this named type has a valid source-level structure.
    ///
    /// This method performs only local structural checks. Semantic lookup is
    /// deliberately excluded.
    #[must_use]
    pub fn is_structurally_valid(&self) -> bool {
        if self.path.is_empty() {
            return false;
        }

        if self.path.segments().iter().any(|segment| segment.is_empty()) {
            return false;
        }

        true
    }

    /// Validates the local structure.
    ///
    /// Generic argument validation is delegated to `TypeExpr::validate()` so
    /// that there is one canonical recursive validation implementation.
    pub fn validate(
        &self,
        policy: super::type_expr::TypeValidationPolicy,
    ) -> Result<(), NamedTypeValidationError> {
        if self.path.is_empty() {
            return Err(NamedTypeValidationError::EmptyPath);
        }

        for (index, segment) in self.path.segments().iter().enumerate() {
            if segment.is_empty() {
                return Err(NamedTypeValidationError::EmptyPathSegment { index });
            }
        }

        for (index, argument) in self.arguments.iter().enumerate() {
            argument
                .validate(policy)
                .map_err(|source| NamedTypeValidationError::InvalidArgument {
                    index,
                    source: Box::new(source),
                })?;
        }

        Ok(())
    }

    /// Returns the canonical local schema version.
    #[must_use]
    pub const fn schema_version() -> u16 {
        NAMED_TYPE_SCHEMA_VERSION
    }
}

impl fmt::Display for NamedType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.path)?;

        if !self.arguments.is_empty() {
            formatter.write_str("<")?;

            for (index, argument) in self.arguments.iter().enumerate() {
                if index != 0 {
                    formatter.write_str(", ")?;
                }

                write!(formatter, "{argument}")?;
            }

            formatter.write_str(">")?;
        }

        Ok(())
    }
}

// =============================================================================
// Validation
// =============================================================================

/// Structural validation failure for [`NamedType`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamedTypeValidationError {
    /// No path segments were supplied.
    EmptyPath,

    /// A path contained an empty segment.
    EmptyPathSegment {
        /// Zero-based path segment index.
        index: usize,
    },

    /// A generic argument was structurally invalid.
    InvalidArgument {
        /// Zero-based generic argument index.
        index: usize,

        /// Underlying type-expression validation failure.
        source: Box<super::type_expr::TypeExprValidationError>,
    },
}

impl fmt::Display for NamedTypeValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPath => formatter.write_str("named type path cannot be empty"),

            Self::EmptyPathSegment { index } => {
                write!(formatter, "named type path contains an empty segment at index {index}")
            }

            Self::InvalidArgument { index, source } => {
                write!(
                    formatter,
                    "invalid generic argument at index {index}: {source}"
                )
            }
        }
    }
}

impl std::error::Error for NamedTypeValidationError {}

// =============================================================================
// Source-level helpers
// =============================================================================

/// Creates a named type from one source-level name.
///
/// This is a convenience constructor for parser and test code.
#[must_use]
pub fn named<S>(name: S) -> NamedType
where
    S: Into<super::type_expr::TypeName>,
{
    NamedType::single(name)
}

/// Creates a named type from a source-level path.
#[must_use]
pub fn named_path(path: TypePath) -> NamedType {
    NamedType::new(path)
}

/// Creates a parameterized named type.
///
/// Generic arguments remain source-level `TypeExpr` values and are not
/// resolved here.
#[must_use]
pub fn named_with_arguments(
    path: TypePath,
    arguments: Vec<TypeExpr>,
) -> NamedType {
    NamedType::with_arguments(path, arguments)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_single_named_type() {
        let ty = NamedType::single("Int");

        assert_eq!(ty.path().len(), 1);
        assert_eq!(
            ty.terminal_name()
                .expect("single-segment path has a terminal name")
                .as_str(),
            "Int"
        );
        assert!(!ty.has_arguments());
        assert!(ty.is_structurally_valid());
    }

    #[test]
    fn preserves_qualified_path() {
        let path = TypePath::from_names(["std", "collections", "Map"]);
        let ty = NamedType::new(path);

        assert_eq!(ty.path().to_source_string(), "std::collections::Map");
        assert_eq!(ty.to_string(), "std::collections::Map");
    }

    #[test]
    fn preserves_generic_argument_order() {
        let path = TypePath::single("Map");

        let first = TypeExpr::Identifier(TypePath::single("String"));
        let second = TypeExpr::Identifier(TypePath::single("Int"));

        let ty = NamedType::with_arguments(path, vec![first, second]);

        assert_eq!(ty.argument_count(), 2);
        assert_eq!(ty.to_string(), "Map<String, Int>");
    }

    #[test]
    fn supports_nested_generic_types() {
        let inner = TypeExpr::Generic {
            base: Box::new(TypeExpr::Identifier(TypePath::single("Vec"))),
            arguments: vec![TypeExpr::Identifier(TypePath::single("Int"))],
        };

        let outer = NamedType::with_arguments(
            TypePath::single("Container"),
            vec![inner],
        );

        assert_eq!(outer.to_string(), "Container<Vec<Int>>");
    }

    #[test]
    fn supports_unicode_names() {
        let ty = NamedType::single("量子状態");

        assert_eq!(
            ty.terminal_name()
                .expect("single-segment path has a terminal name")
                .as_str(),
            "量子状態"
        );

        assert!(ty.is_structurally_valid());
    }

    #[test]
    fn rejects_empty_paths() {
        let ty = NamedType::new(TypePath::new(Vec::new()));

        assert!(!ty.is_structurally_valid());

        let result = ty.validate(Default::default());

        assert_eq!(
            result,
            Err(NamedTypeValidationError::EmptyPath)
        );
    }

    #[test]
    fn rejects_empty_path_segments() {
        let ty = NamedType::new(TypePath::from_names(["std", "", "Type"]));

        assert!(!ty.is_structurally_valid());

        let result = ty.validate(Default::default());

        assert_eq!(
            result,
            Err(NamedTypeValidationError::EmptyPathSegment { index: 1 })
        );
    }

    #[test]
    fn visits_arguments_in_source_order() {
        let ty = NamedType::with_arguments(
            TypePath::single("Tuple"),
            vec![
                TypeExpr::Identifier(TypePath::single("A")),
                TypeExpr::Identifier(TypePath::single("B")),
                TypeExpr::Identifier(TypePath::single("C")),
            ],
        );

        let mut names = Vec::new();

        ty.visit_arguments(|argument| {
            if let TypeExpr::Identifier(path) = argument {
                names.push(path.to_source_string());
            }
        });

        assert_eq!(names, ["A", "B", "C"]);
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(
            NamedType::schema_version(),
            NAMED_TYPE_SCHEMA_VERSION
        );
    }

    #[test]
    fn named_helper_creates_expected_type() {
        let ty = named("Result");

        assert_eq!(ty.to_string(), "Result");
    }

    #[test]
    fn named_path_helper_preserves_path() {
        let ty = named_path(TypePath::from_names(["a", "b", "C"]));

        assert_eq!(ty.to_string(), "a::b::C");
    }

    #[test]
    fn named_with_arguments_helper_preserves_arguments() {
        let ty = named_with_arguments(
            TypePath::single("Vec"),
            vec![TypeExpr::Identifier(TypePath::single("u64"))],
        );

        assert_eq!(ty.to_string(), "Vec<u64>");
    }
}