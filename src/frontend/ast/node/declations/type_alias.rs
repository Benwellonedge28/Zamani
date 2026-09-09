//! Zamani Frontend AST — Type Alias Declaration
//!
//! This module owns the source-level representation of a Zamani type alias.
//!
//! # Architectural boundary
//!
//! A [`TypeAlias`] describes a programmer-declared relationship between a
//! source-level name and a source-level type expression:
//
//! ```text
//! type Name = Type;
//! type Name<T> = Type<T>;
//! ```
//!
//! This module deliberately does NOT:
//!
//! - resolve names;
//! - perform type checking;
//! - instantiate generics;
//! - select a backend;
//! - select hardware;
//! - know the number of qubits/resources;
//! - know a quantum topology;
//! - lower to ZUIR;
//! - lower to QIR;
//! - perform optimization;
//! - perform scheduling;
//! - perform routing;
//! - perform error correction;
//! - perform code generation.
//!
//! Those responsibilities belong to later compiler phases.
//!
//! # POCO-REAF
//!
//! Type aliases are source-level abstractions. They therefore must not encode
//! machine-size assumptions or target-specific representations. A type alias
//! may describe a type containing symbolic, generic, quantum, resource, or
//! future-domain constructs, provided those constructs are represented by the
//! language's generic type system or registered extensions.
//!
//! # Dependency direction
//!
//! ```text
//! source infrastructure
//!        │
//!        ▼
//!     TypeAlias
//!        │
//!        ├── parser
//!        ├── structural validation
//!        ├── semantic analysis
//!        └── later lowering
//! ```
//!
//! This file must never depend on semantic analysis, ZUIR, quantum hardware,
//! runtime, optimization, scheduling, routing, or backend modules.
//!
//! # Rust compatibility
//!
//! Designed for Rust 1.97 / 1.97.1 and uses no `unsafe` code.

use crate::ast::{Identifier, TypeExpr, TypeParameter};
use crate::source_map::Span;

/// A source-level Zamani type-alias declaration.
///
/// A type alias introduces a new source-level name for an existing type
/// expression. It does not create a distinct runtime representation by
/// itself.
///
/// Examples:
///
/// ```text
/// type UserId = String;
/// type Pair<T> = (T, T);
/// type QuantumState<T> = Quantum<T>;
/// ```
///
/// The node intentionally stores the generic parameters and target type as
/// AST structures rather than resolved semantic types. This preserves source
/// intent and keeps the frontend independent of any particular target.
///
/// # Invariants
///
/// A valid `TypeAlias` must satisfy:
///
/// 1. `name` is non-empty.
/// 2. `name.span()` identifies the alias identifier in source.
/// 3. `span` covers the declaration.
/// 4. `target` is a syntactically valid [`TypeExpr`].
/// 5. Generic parameter names are structurally represented in source order.
/// 6. No backend or hardware information is stored in this node.
///
/// Duplicate generic parameter names, unresolved names, recursive aliases,
/// invalid bounds, and other semantic conditions are deliberately handled by
/// semantic analysis rather than this source AST node.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeAlias {
    /// Complete source span of the declaration.
    span: Span,

    /// Alias identifier introduced by this declaration.
    name: Identifier,

    /// Generic parameters declared by the alias.
    ///
    /// This is an ordered collection because declaration order is part of the
    /// source-level generic signature.
    parameters: Vec<TypeParameter>,

    /// Type expression to which the alias refers.
    target: TypeExpr,
}

impl TypeAlias {
    /// Constructs a type alias from its source-level components.
    ///
    /// This constructor does not perform semantic validation. It only
    /// establishes the structural representation of the declaration.
    ///
    /// Callers that require complete AST validation should additionally run
    /// the frontend AST validation phase.
    #[must_use]
    pub fn new(
        span: Span,
        name: Identifier,
        parameters: Vec<TypeParameter>,
        target: TypeExpr,
    ) -> Self {
        Self {
            span,
            name,
            parameters,
            target,
        }
    }

    /// Returns the complete source span of this declaration.
    #[must_use]
    pub fn span(&self) -> &Span {
        &self.span
    }

    /// Returns the identifier introduced by this alias.
    #[must_use]
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    /// Returns the textual alias name.
    #[must_use]
    pub fn name_str(&self) -> &str {
        self.name.name()
    }

    /// Returns the generic parameters in declaration order.
    #[must_use]
    pub fn parameters(&self) -> &[TypeParameter] {
        &self.parameters
    }

    /// Returns the target type expression.
    #[must_use]
    pub fn target(&self) -> &TypeExpr {
        &self.target
    }

    /// Returns whether this alias declares generic parameters.
    #[must_use]
    pub fn is_generic(&self) -> bool {
        !self.parameters.is_empty()
    }

    /// Returns the number of generic parameters.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Consumes the node and returns its source span.
    #[must_use]
    pub fn into_span(self) -> Span {
        self.span
    }

    /// Consumes the node and returns its identifier.
    #[must_use]
    pub fn into_name(self) -> Identifier {
        self.name
    }

    /// Consumes the node and returns its generic parameters.
    #[must_use]
    pub fn into_parameters(self) -> Vec<TypeParameter> {
        self.parameters
    }

    /// Consumes the node and returns its target type expression.
    #[must_use]
    pub fn into_target(self) -> TypeExpr {
        self.target
    }

    /// Decomposes the declaration into all of its source-level components.
    ///
    /// This is useful for compiler passes that need ownership without
    /// cloning potentially large type-expression trees.
    #[must_use]
    pub fn into_parts(self) -> (Span, Identifier, Vec<TypeParameter>, TypeExpr) {
        (self.span, self.name, self.parameters, self.target)
    }

    /// Performs local structural validation.
    ///
    /// This method intentionally performs only checks that can be determined
    /// from this node without symbol resolution.
    ///
    /// Semantic checks such as:
    ///
    /// - duplicate generic parameter names;
    /// - unknown bounds;
    /// - recursive alias cycles;
    /// - invalid type applications;
    /// - illegal aliases;
    ///
    /// belong to semantic analysis.
    ///
    /// The returned strings are intentionally independent of backend,
    /// hardware, and semantic-resolution infrastructure.
    pub fn validate_structure(&self) -> Result<(), TypeAliasValidationError> {
        if self.name_str().is_empty() {
            return Err(TypeAliasValidationError::EmptyName {
                span: self.name.span().clone(),
            });
        }

        if self.span.is_empty() {
            return Err(TypeAliasValidationError::EmptyDeclarationSpan {
                span: self.span.clone(),
            });
        }

        if !self.span.contains(self.name.span()) {
            return Err(TypeAliasValidationError::NameOutsideDeclaration {
                declaration: self.span.clone(),
                name: self.name.span().clone(),
            });
        }

        Ok(())
    }
}

/// Structural errors that can be detected without semantic name resolution.
///
/// This error type deliberately remains local to the AST declaration. It
/// does not depend on the compiler's global diagnostic system, which allows
/// the node to be constructed and tested independently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeAliasValidationError {
    /// The alias identifier contains no source text.
    EmptyName {
        /// Source location of the invalid identifier.
        span: Span,
    },

    /// The declaration has no source range.
    ///
    /// A zero-length span can be valid for certain generated AST constructs,
    /// but a source declaration requires a meaningful declaration range.
    EmptyDeclarationSpan {
        /// Source location associated with the invalid declaration.
        span: Span,
    },

    /// The identifier is not contained by the declaration span.
    NameOutsideDeclaration {
        /// Span claimed by the declaration.
        declaration: Span,

        /// Span claimed by the identifier.
        name: Span,
    },
}

impl std::fmt::Display for TypeAliasValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyName { .. } => {
                formatter.write_str("type alias name must not be empty")
            }
            Self::EmptyDeclarationSpan { .. } => {
                formatter.write_str("type alias declaration span must not be empty")
            }
            Self::NameOutsideDeclaration { .. } => {
                formatter.write_str(
                    "type alias identifier span must be contained by declaration span",
                )
            }
        }
    }
}

impl std::error::Error for TypeAliasValidationError {}

/// Convenience conversion into the legacy four-component representation.
///
/// The current repository still represents a type alias as:
///
/// ```text
/// Statement::TypeAlias(
///     Span,
///     Identifier,
///     Vec<TypeParameter>,
///     TypeExpr,
/// )
/// ```
///
/// Keeping this conversion here provides a clean migration boundary while
/// the monolithic `src/ast/mod.rs` is being decomposed.
///
/// This implementation deliberately does not make `TypeAlias` depend on the
/// legacy `Statement` representation. Instead, callers can use
/// [`TypeAlias::into_parts`] when constructing the legacy node.
///
/// The migration should eventually remove the legacy representation after
/// parser and semantic integration have moved to the modular AST.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Identifier;
    use crate::source_map::{BytePos, FileId, Span};

    fn span(start: u32, end: u32) -> Span {
        Span::new(
            FileId::new(1),
            BytePos::new(start),
            BytePos::new(end),
            1,
            1,
        )
    }

    fn identifier(name: &str, start: u32, end: u32) -> Identifier {
        Identifier::new(name, span(start, end))
    }

    #[test]
    fn constructs_non_generic_alias() {
        let declaration_span = span(0, 22);
        let name = identifier("UserId", 5, 11);
        let target = TypeExpr::Identifier(identifier("String", 14, 20));

        let alias = TypeAlias::new(
            declaration_span.clone(),
            name.clone(),
            Vec::new(),
            target.clone(),
        );

        assert_eq!(alias.span(), &declaration_span);
        assert_eq!(alias.name(), &name);
        assert_eq!(alias.name_str(), "UserId");
        assert!(!alias.is_generic());
        assert_eq!(alias.parameter_count(), 0);
        assert_eq!(alias.target(), &target);
    }

    #[test]
    fn constructs_generic_alias() {
        let declaration_span = span(0, 30);
        let name = identifier("Pair", 5, 9);
        let parameter_name = identifier("T", 10, 11);

        let parameter = TypeParameter {
            name: parameter_name,
            bounds: Vec::new(),
        };

        let target = TypeExpr::Tuple(vec![
            TypeExpr::Identifier(identifier("T", 16, 17)),
            TypeExpr::Identifier(identifier("T", 19, 20)),
        ]);

        let alias = TypeAlias::new(
            declaration_span,
            name,
            vec![parameter],
            target,
        );

        assert!(alias.is_generic());
        assert_eq!(alias.parameter_count(), 1);
        assert_eq!(alias.parameters()[0].name.name(), "T");
    }

    #[test]
    fn preserves_parameter_order() {
        let declaration_span = span(0, 40);
        let name = identifier("Map", 5, 8);

        let first = TypeParameter {
            name: identifier("K", 9, 10),
            bounds: Vec::new(),
        };

        let second = TypeParameter {
            name: identifier("V", 11, 12),
            bounds: Vec::new(),
        };

        let alias = TypeAlias::new(
            declaration_span,
            name,
            vec![first, second],
            TypeExpr::Identifier(identifier("V", 20, 21)),
        );

        assert_eq!(alias.parameters()[0].name.name(), "K");
        assert_eq!(alias.parameters()[1].name.name(), "V");
    }

    #[test]
    fn validates_structural_invariants() {
        let declaration_span = span(0, 22);
        let name = identifier("UserId", 5, 11);
        let target = TypeExpr::Identifier(identifier("String", 14, 20));

        let alias = TypeAlias::new(
            declaration_span,
            name,
            Vec::new(),
            target,
        );

        assert!(alias.validate_structure().is_ok());
    }

    #[test]
    fn rejects_empty_name() {
        let declaration_span = span(0, 10);
        let name = identifier("", 5, 5);
        let target = TypeExpr::Unit;

        let alias = TypeAlias::new(
            declaration_span,
            name,
            Vec::new(),
            target,
        );

        assert!(matches!(
            alias.validate_structure(),
            Err(TypeAliasValidationError::EmptyName { .. })
        ));
    }

    #[test]
    fn rejects_empty_declaration_span() {
        let declaration_span = span(0, 0);
        let name = identifier("T", 0, 1);
        let target = TypeExpr::Unit;

        let alias = TypeAlias::new(
            declaration_span,
            name,
            Vec::new(),
            target,
        );

        assert!(matches!(
            alias.validate_structure(),
            Err(TypeAliasValidationError::EmptyDeclarationSpan { .. })
        ));
    }

    #[test]
    fn rejects_name_outside_declaration() {
        let declaration_span = span(0, 5);
        let name = identifier("T", 10, 11);
        let target = TypeExpr::Unit;

        let alias = TypeAlias::new(
            declaration_span,
            name,
            Vec::new(),
            target,
        );

        assert!(matches!(
            alias.validate_structure(),
            Err(TypeAliasValidationError::NameOutsideDeclaration { .. })
        ));
    }

    #[test]
    fn into_parts_preserves_all_components() {
        let declaration_span = span(0, 22);
        let name = identifier("UserId", 5, 11);
        let target = TypeExpr::Identifier(identifier("String", 14, 20));

        let alias = TypeAlias::new(
            declaration_span.clone(),
            name.clone(),
            Vec::new(),
            target.clone(),
        );

        let (actual_span, actual_name, actual_parameters, actual_target) =
            alias.into_parts();

        assert_eq!(actual_span, declaration_span);
        assert_eq!(actual_name, name);
        assert!(actual_parameters.is_empty());
        assert_eq!(actual_target, target);
    }

    #[test]
    fn supports_quantum_wrapped_types_without_quantum_coupling() {
        let declaration_span = span(0, 35);
        let name = identifier("QState", 5, 11);

        let inner = TypeExpr::Identifier(identifier("State", 21, 26));
        let target = TypeExpr::Quantum(Box::new(inner));

        let alias = TypeAlias::new(
            declaration_span,
            name,
            Vec::new(),
            target,
        );

        assert_eq!(alias.target().name(), "Quantum<State>");
        assert!(alias.validate_structure().is_ok());
    }

    #[test]
    fn supports_symbolic_generic_targets() {
        let declaration_span = span(0, 25);
        let name = identifier("Buffer", 5, 11);

        let parameter = TypeParameter {
            name: identifier("T", 12, 13),
            bounds: Vec::new(),
        };

        let target = TypeExpr::Generic(
            Box::new(TypeExpr::Identifier(identifier("Vec", 16, 19))),
            vec![TypeExpr::Identifier(identifier("T", 20, 21))],
        );

        let alias = TypeAlias::new(
            declaration_span,
            name,
            vec![parameter],
            target,
        );

        assert_eq!(alias.parameters().len(), 1);
        assert_eq!(alias.target().name(), "Vec<T>");
        assert!(alias.validate_structure().is_ok());
    }
}