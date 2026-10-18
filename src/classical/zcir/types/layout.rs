
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR logical type-layout adapter
//!
//! Path: `src/classical/zcir/types/layout.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provides the ZCIR-facing API for obtaining a target-independent logical
//! description of a canonical ZUIR type.
//!
//! This module delegates layout description and local type validation to
//! `crate::zuir::types::layout`. It does not implement a second layout
//! engine or define a competing type vocabulary.
//!
//! ## Authority and ownership
//!
//! Canonical ownership remains with:
//!
//! - `crate::zuir::types::type_def::TypeDef`: semantic type definitions.
//! - `crate::zuir::types::type_def::TypeKind`: canonical type variants.
//! - `crate::zuir::types::type_def::TypeRef`: named and inline type refs.
//! - `crate::zuir::types::layout::LogicalLayout`: logical layout description.
//! - `crate::zuir::types::layout::LayoutShape`: logical shape vocabulary.
//! - `crate::zuir::types::layout::LayoutPolicy`: traversal policy.
//! - `crate::zuir::types::layout::LayoutError`: canonical layout errors.
//!
//! This adapter owns only the ZCIR entry point and its stable wrapper API.
//!
//! ## Does not own
//!
//! This module does not own:
//!
//! - physical byte size, alignment, padding, endianness, or ABI;
//! - register widths, addresses, memory allocation, or object placement;
//! - named-type resolution, type identity, interning, or generic inference;
//! - target capability discovery or target feasibility decisions;
//! - operation lowering, scheduling, optimization, or execution;
//! - source parsing, AST construction, or semantic analysis;
//! - independent layout variants or serialization formats.
//!
//! ## Scalability and POCO-REAF
//!
//! No universal limits are imposed on integer width, aggregate arity,
//! dimensions, tensor rank, type count, or hardware resources.
//!
//! Symbolic and dynamic dimensions are preserved by the canonical layout
//! implementation. This adapter does not convert them to host-sized
//! integers, allocate values, or silently narrow widths.
//!
//! The canonical `LayoutPolicy` provides an optional inline-nesting budget.
//! That budget limits compiler work for a particular request; it is not a
//! language-level maximum. Callers handling untrusted input should configure
//! an appropriate policy at their trust boundary.
//!
//! Finite memory, execution time, address space, and target capacity remain
//! real constraints. Such failures must be explicit and must not redefine
//! source-language semantics.
//!
//! ## Integration contract
//!
//! - `types/mod.rs` declares `pub mod layout;`.
//! - The ZCIR module root must expose the types module through its reviewed
//!   module hierarchy.
//! - Frontend semantic analysis supplies canonical, semantically meaningful
//!   `TypeDef` values.
//! - ZUIR remains the canonical owner of logical layout.
//! - Type verification and the canonical type registry resolve references
//!   and establish cross-definition correctness.
//! - A future target-layout/ABI subsystem consumes logical layout plus an
//!   explicit target description to determine physical representation.
//! - Lowering reports unsupported target realizations instead of silently
//!   changing program meaning.
//!
//! ## Upstream integration requirement
//!
//! The canonical implementation in `src/zuir/types/layout.rs` currently
//! maps `TypeKind::Reference` to `LayoutShape::NamedType`. Before declaring
//! this adapter production-ready, that mapping must be corrected upstream:
//! introduce a distinct canonical reference shape and preserve it during
//! layout description. ZCIR must not work around that loss by creating a
//! competing layout tree.
//!
//! ## Freeze criteria
//!
//! Freeze only after:
//!
//! - the canonical reference-shape issue is resolved;
//! - this module is registered in the actual ZCIR module tree;
//! - formatting, checking, tests, and clippy pass on the declared baseline;
//! - interoperability and downstream layout consumers are tested.

use crate::zuir::types::type_def::TypeDef;
use crate::zuir::types::layout::{
    LayoutError, LayoutPolicy, LayoutShape, LogicalLayout,
};
use crate::zuir::schema::types::QualifiedName;

/// ZCIR-facing policy for describing canonical semantic types.
///
/// This is an alias to the canonical ZUIR policy, not a second policy
/// vocabulary. In particular, `max_inline_depth: None` disables that
/// specific traversal safeguard; it does not promise infinite resources.
pub use crate::zuir::types::layout::LayoutPolicy as ClassicalLayoutPolicy;

/// ZCIR-facing error type for logical layout operations.
///
/// Error identity and semantics remain owned by ZUIR.
pub use crate::zuir::types::layout::LayoutError as ClassicalLayoutError;

/// ZCIR-facing alias for the canonical logical shape vocabulary.
///
/// No shape variants or target-specific interpretations are introduced here.
pub use crate::zuir::types::layout::LayoutShape as ClassicalLayoutShape;

/// A ZCIR-facing wrapper around the canonical ZUIR logical layout.
///
/// The wrapper preserves the canonical layout without deriving physical
/// properties or creating a new semantic type identity.
///
/// Construct it through [`Self::from_type_def`] or
/// [`Self::from_type_def_with_policy`] so canonical validation and layout
/// construction are performed first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalTypeLayout {
    logical: LogicalLayout,
}

impl ClassicalTypeLayout {
    /// Builds a logical layout using the canonical default policy.
    ///
    /// The canonical layout builder validates the definition before
    /// describing its structure.
    ///
    /// # Errors
    ///
    /// Returns the canonical layout error if the definition is invalid,
    /// its qualified name is invalid, or the configured traversal budget
    /// is exceeded.
    pub fn from_type_def(definition: &TypeDef) -> Result<Self, LayoutError> {
        Self::from_type_def_with_policy(definition, &LayoutPolicy::default())
    }

    /// Builds a logical layout under an explicit operational policy.
    ///
    /// The policy controls compiler traversal work only. It does not change
    /// the semantics of the source program or impose a language-wide limit.
    ///
    /// # Errors
    ///
    /// Returns the canonical layout error when validation or layout
    /// construction fails.
    pub fn from_type_def_with_policy(
        definition: &TypeDef,
        policy: &LayoutPolicy,
    ) -> Result<Self, LayoutError> {
        let logical = LogicalLayout::from_type_def_with_policy(definition, policy)?;
        Ok(Self { logical })
    }

    /// Returns the semantic name of the described type.
    ///
    /// This name is not a physical symbol, address, or target type name.
    #[must_use]
    pub fn type_name(&self) -> &QualifiedName {
        &self.logical.type_name
    }

    /// Returns the canonical logical shape.
    ///
    /// The result contains logical structure only. It does not guarantee
    /// a particular physical representation or target support.
    #[must_use]
    pub fn shape(&self) -> &LayoutShape {
        &self.logical.shape
    }

    /// Borrows the canonical logical layout.
    ///
    /// This is useful when interoperating with existing ZUIR consumers
    /// without converting or copying the layout.
    #[must_use]
    pub fn as_logical_layout(&self) -> &LogicalLayout {
        &self.logical
    }

    /// Consumes the wrapper and returns the canonical logical layout.
    ///
    /// The conversion is lossless.
    #[must_use]
    pub fn into_logical_layout(self) -> LogicalLayout {
        self.logical
    }

    /// Returns an owned copy of the canonical logical layout.
    ///
    /// Prefer [`Self::as_logical_layout`] when ownership is not required.
    #[must_use]
    pub fn to_logical_layout(&self) -> LogicalLayout {
        self.logical.clone()
    }
}

impl AsRef<LogicalLayout> for ClassicalTypeLayout {
    fn as_ref(&self) -> &LogicalLayout {
        self.as_logical_layout()
    }
}

impl From<ClassicalTypeLayout> for LogicalLayout {
    fn from(layout: ClassicalTypeLayout) -> Self {
        layout.into_logical_layout()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::types::type_def::{
        Dimension, IntegerSignedness, IntegerWidth, SymbolicExpression, TypeKind,
        TypeRef,
    };

    fn name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("test qualified name must be valid")
    }

    fn definition(value: &str, kind: TypeKind) -> TypeDef {
        TypeDef::new(name(value), kind).expect("test type definition must be valid")
    }

    #[test]
    fn preserves_unbounded_integer_semantics() {
        let definition = definition(
            "classical::Integer",
            TypeKind::Integer {
                signedness: IntegerSignedness::Signless,
                width: IntegerWidth::Unbounded,
            },
        );

        let layout =
            ClassicalTypeLayout::from_type_def(&definition).expect("logical layout");

        assert_eq!(layout.type_name(), &name("classical::Integer"));
        assert!(matches!(
            layout.shape(),
            LayoutShape::Integer {
                width: crate::zuir::types::layout::LayoutIntegerWidth::Unbounded,
                ..
            }
        ));
    }

    #[test]
    fn preserves_wide_integer_width_without_host_narrowing() {
        let definition = definition(
            "classical::WideInteger",
            TypeKind::Integer {
                signedness: IntegerSignedness::Unsigned,
                width: IntegerWidth::Fixed(u128::MAX),
            },
        );

        let layout =
            ClassicalTypeLayout::from_type_def(&definition).expect("logical layout");

        assert!(matches!(
            layout.shape(),
            LayoutShape::Integer {
                width: crate::zuir::types::layout::LayoutIntegerWidth::Fixed(width),
                ..
            } if *width == u128::MAX
        ));
    }

    #[test]
    fn preserves_symbolic_array_dimensions() {
        let definition = definition(
            "classical::SymbolicArray",
            TypeKind::Array {
                element: TypeRef::Named(name("classical::Element")),
                length: Dimension::Symbolic(SymbolicExpression {
                    expression: "element_count".to_owned(),
                }),
            },
        );

        let layout =
            ClassicalTypeLayout::from_type_def(&definition).expect("logical layout");

        assert!(matches!(
            layout.shape(),
            LayoutShape::Array {
                length: Dimension::Symbolic(expression),
                ..
            } if expression.expression == "element_count"
        ));
    }

    #[test]
    fn preserves_dynamic_array_dimensions() {
        let definition = definition(
            "classical::DynamicArray",
            TypeKind::Array {
                element: TypeRef::Named(name("classical::Element")),
                length: Dimension::Dynamic,
            },
        );

        let layout =
            ClassicalTypeLayout::from_type_def(&definition).expect("logical layout");

        assert!(matches!(
            layout.shape(),
            LayoutShape::Array {
                length: Dimension::Dynamic,
                ..
            }
        ));
    }

    #[test]
    fn preserves_tuple_order() {
        let definition = definition(
            "classical::Pair",
            TypeKind::Tuple {
                elements: vec![
                    TypeRef::Named(name("classical::First")),
                    TypeRef::Named(name("classical::Second")),
                ],
            },
        );

        let layout =
            ClassicalTypeLayout::from_type_def(&definition).expect("logical layout");

        assert!(matches!(
            layout.shape(),
            LayoutShape::Tuple { elements }
                if matches!(&elements[..],
                    [
                        LayoutShape::NamedType { name: first },
                        LayoutShape::NamedType { name: second }
                    ] if first == &name("classical::First")
                        && second == &name("classical::Second"))
        ));
    }

    #[test]
    fn named_types_remain_unresolved() {
        let definition = definition(
            "classical::Container",
            TypeKind::Tuple {
                elements: vec![TypeRef::Named(name("application::Payload"))],
            },
        );

        let layout =
            ClassicalTypeLayout::from_type_def(&definition).expect("logical layout");

        assert!(matches!(
            layout.shape(),
            LayoutShape::Tuple { elements }
                if matches!(&elements[0], LayoutShape::NamedType { name: target }
                    if target == &name("application::Payload"))
        ));
    }

    #[test]
    fn applies_explicit_inline_nesting_budget() {
        let definition = definition(
            "classical::Nested",
            TypeKind::Tuple {
                elements: vec![TypeRef::Inline(Box::new(TypeKind::Tuple {
                    elements: vec![TypeRef::Inline(Box::new(TypeKind::Boolean))],
                }))],
            },
        );

        let policy = LayoutPolicy {
            max_inline_depth: Some(1),
        };

        let result =
            ClassicalTypeLayout::from_type_def_with_policy(&definition, &policy);

        assert!(matches!(
            result,
            Err(LayoutError::NestingBudgetExceeded { .. })
        ));
    }

    #[test]
    fn permits_more_nesting_when_policy_allows_it() {
        let definition = definition(
            "classical::Nested",
            TypeKind::Tuple {
                elements: vec![TypeRef::Inline(Box::new(TypeKind::Tuple {
                    elements: vec![TypeRef::Inline(Box::new(TypeKind::Boolean))],
                }))],
            },
        );

        let policy = LayoutPolicy {
            max_inline_depth: Some(8),
        };

        assert!(
            ClassicalTypeLayout::from_type_def_with_policy(&definition, &policy).is_ok()
        );
    }

    #[test]
    fn exposes_canonical_layout_without_copying() {
        let definition = definition("classical::Flag", TypeKind::Boolean);
        let layout =
            ClassicalTypeLayout::from_type_def(&definition).expect("logical layout");

        let canonical: &LogicalLayout = layout.as_ref();

        assert_eq!(canonical, layout.as_logical_layout());
        assert_eq!(canonical.type_name, name("classical::Flag"));
    }

    #[test]
    fn conversion_back_to_zuir_is_lossless() {
        let definition = definition("classical::Flag", TypeKind::Boolean);
        let layout =
            ClassicalTypeLayout::from_type_def(&definition).expect("logical layout");

        let canonical = layout.into_logical_layout();

        assert_eq!(canonical.type_name, name("classical::Flag"));
        assert!(matches!(canonical.shape, LayoutShape::Boolean));
    }

    #[test]
    fn rejects_invalid_mutated_type_definition() {
        let mut definition = definition("classical::InitiallyValid", TypeKind::Boolean);

        // TypeDef has public fields, so consumers must not assume that
        // a previously constructed definition remains valid after mutation.
        definition.kind = TypeKind::Union {
            alternatives: Vec::new(),
        };

        assert!(
            ClassicalTypeLayout::from_type_def(&definition).is_err(),
            "canonical layout construction must revalidate the definition"
        );
    }
}
