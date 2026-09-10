//! Zamani Frontend AST — Array Type Expressions
//!
//! Typed façade for the canonical [`TypeExpr::Array`] representation.
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
//! TypeExpr::Array
//!     │
//!     ▼
//! ArrayType  ← this module
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic type model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ▼
//! domain / target lowering
//! ```
//!
//! # Purpose
//!
//! This module provides the array-specific typed API for the canonical
//! source-level Zamani [`TypeExpr`] representation.
//!
//! The authoritative representation remains:
//!
//! ```text
//! TypeExpr::Array {
//!     element: Box<TypeExpr>,
//!     length: Option<TypeValueExpr>,
//! }
//! ```
//!
//! [`ArrayType`] deliberately does **not** introduce another array AST node.
//! It is a strongly typed façade around the canonical representation.
//!
//! This prevents duplicate representations of arrays from developing separate:
//!
//! - validation rules;
//! - serialization schemas;
//! - visitor behavior;
//! - parser behavior;
//! - formatting behavior;
//! - semantic-lowering behavior;
//! - compatibility behavior.
//!
//! # POCO-REAF
//!
//! Array types must not encode a fixed machine size.
//!
//! The array length is represented by [`TypeValueExpr`] rather than `usize`,
//! `u32`, `u64`, or another machine-width representation. Consequently,
//! source-level array cardinality may remain symbolic until semantic analysis.
//!
//! Examples of representable source intent include:
//!
//! ```text
//! [T; N]
//! [Qubit; qubits]
//! [Element; resource_count]
//! ```
//!
//! where the eventual value of the length can be determined by later compiler
//! stages according to the language and target semantics.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_ARRAY_LENGTH
//! MAX_QUBITS
//! MAX_REGISTER_SIZE
//! MAX_MACHINE_SIZE
//! ```
//!
//! in this module.
//!
//! Compiler/resource-security limits, when required, belong to an explicit
//! validation policy rather than to the array type itself.
//!
//! # Domain neutrality
//!
//! `ArrayType` does not know what its element type eventually represents.
//!
//! The element may eventually describe:
//!
//! - classical data;
//! - a quantum resource abstraction;
//! - a logical resource;
//! - distributed data;
//! - accelerator data;
//! - HDL data;
//! - an AI/ML object;
//! - a future computational resource.
//!
//! The array façade does not select or imply any physical representation.
//!
//! It does not contain:
//!
//! - qubit topology;
//! - physical qubit allocation;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulse generation;
//! - QEC implementation;
//! - resilience implementation;
//! - backend instructions;
//! - vendor information;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - hardware-specific layouts.
//!
//! # Semantic boundary
//!
//! This module represents source-level type structure only.
//!
//! It does not perform:
//!
//! - name resolution;
//! - type inference;
//! - generic substitution;
//! - dependent-value evaluation;
//! - constant folding;
//! - type unification;
//! - ownership analysis;
//! - borrow checking;
//! - resource allocation;
//! - quantum legality checking;
//! - hardware mapping;
//! - backend selection.
//!
//! Those operations belong to later compiler phases.
//!
//! # Integration contract
//!
//! ## Parser
//!
//! The parser should construct the canonical representation through:
//!
//! ```text
//! TypeExpr::array(element, length)
//! ```
//!
//! and may obtain an [`ArrayType`] façade through [`ArrayType::try_from_type_expr`]
//! where array-specific inspection is useful.
//!
//! ## Structural validation
//!
//! Array validation delegates to the canonical [`TypeExpr::validate`] and
//! [`TypeExpr::validate_with_policy`] implementations.
//!
//! There is therefore exactly one authoritative structural-validation system.
//!
//! ## Semantic analysis
//!
//! Semantic analysis consumes [`TypeExpr`] / [`ArrayType`] and determines the
//! actual meaning of:
//!
//! - the element type;
//! - the array cardinality;
//! - symbolic/dependent cardinality;
//! - generic substitutions;
//! - resource semantics.
//!
//! This module does not resolve any of those meanings.
//!
//! ## ZUIR
//!
//! ZUIR lowering must consume the semantic representation rather than using
//! this façade to make target-specific decisions.
//!
//! ## Serialization
//!
//! Serialization remains owned by the canonical [`TypeExpr`] representation.
//!
//! `ArrayType` therefore does not define an independent wire format.
//!
//! ## Visitors and traversal
//!
//! Visitors should encounter the canonical `TypeExpr::Array` node through the
//! normal `TypeExpr` traversal infrastructure. `ArrayType` is an inspection
//! façade and does not create a second traversal tree.
//!
//! # Scalability
//!
//! The number of array dimensions, the size expression, and the element-type
//! nesting are not constrained by a language-level machine constant here.
//!
//! An array may contain another array:
//!
//! ```text
//! [[T; M]; N]
//! ```
//!
//! and this module imposes no artificial dimension limit.
//!
//! Any compiler resource policy must be explicitly supplied by the caller.
//!
//! # Determinism
//!
//! Array structure preserves source order and the canonical `TypeExpr`
//! representation owns serialization/formatting behavior.
//!
//! No unordered collection is introduced here.
//!
//! # Security
//!
//! This module:
//!
//! - performs no I/O;
//! - performs no execution;
//! - uses no unsafe code;
//! - performs no pointer arithmetic;
//! - does not allocate based on a semantic array length;
//! - does not convert symbolic lengths into machine-sized allocations.
//!
//! Constructing an AST array type therefore does not allocate the represented
//! runtime array.
//!
//! This distinction is critical: `[T; N]` in the AST represents a type, not
//! `N` runtime objects.
//!
//! # Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Forbidden dependencies
//!
//! This module must not depend on:
//!
//! - the legacy `crate::ast`;
//! - lexer token types;
//! - parser implementation details;
//! - semantic analysis;
//! - compiler drivers;
//! - ZUIR;
//! - quantum IR;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - quantum backends;
//! - hardware APIs;
//! - runtime execution.
//!
//! It depends only on the canonical sibling `type_expr` module and the Rust
//! standard library.
//!
//! # Compatibility principle
//!
//! Existing code using:
//!
//! ```text
//! TypeExpr::Array
//! ```
//!
//! remains authoritative.
//!
//! `ArrayType` is an additional typed API and does not require unrelated
//! consumers to migrate immediately.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! - the canonical array representation remains in `TypeExpr`;
//! - no duplicate array schema exists;
//! - arbitrary element types are supported;
//! - symbolic lengths are preserved;
//! - zero/unknown-length source forms remain representable;
//! - structural validation delegates to `TypeExpr`;
//! - serialization remains canonical;
//! - visitor/traversal integration remains canonical;
//! - no target/hardware dependency exists;
//! - no fixed array/resource limit is introduced;
//! - conversions are lossless;
//! - malformed/non-array expressions are rejected without panicking;
//! - tests cover ordinary, symbolic, nested, large-arity and quantum-resource
//!   element types;
//! - the API remains compatible with Rust 1.97.1.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::slice::Iter;

use super::type_expr::{TypeExpr, TypeExprError, TypeValidationPolicy, TypeValueExpr};

/// A typed façade over the canonical [`TypeExpr::Array`] representation.
///
/// # Invariant
///
/// A successfully constructed `ArrayType` always contains:
///
/// ```text
/// TypeExpr::Array { .. }
/// ```
///
/// The contained array remains owned by the canonical `TypeExpr` enum.
/// `ArrayType` does not duplicate its element or length fields.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArrayType(TypeExpr);

impl ArrayType {
    /// Creates an array type from an element type and optional source-level
    /// cardinality expression.
    ///
    /// `None` means that the source representation does not provide an
    /// explicit cardinality expression.
    ///
    /// This constructor does not perform structural validation. This is
    /// intentional: parser/recovery code may construct the AST first and run
    /// validation as a separate phase.
    ///
    /// Use [`Self::try_new`] when immediate validation is required.
    #[must_use]
    pub fn new(element: TypeExpr, length: Option<TypeValueExpr>) -> Self {
        Self(TypeExpr::array(element, length))
    }

    /// Creates and validates an array type using the default type-validation
    /// policy.
    ///
    /// The default policy does not impose an artificial collection-size limit.
    pub fn try_new(
        element: TypeExpr,
        length: Option<TypeValueExpr>,
    ) -> Result<Self, TypeExprError> {
        let array = Self::new(element, length);
        array.validate()?;
        Ok(array)
    }

    /// Creates and validates an array type using an explicit compiler policy.
    ///
    /// The policy controls compiler-resource protection; it does not become
    /// part of the language-level array type.
    pub fn try_new_with_policy(
        element: TypeExpr,
        length: Option<TypeValueExpr>,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, TypeExprError> {
        let array = Self::new(element, length);
        array.validate_with_policy(policy)?;
        Ok(array)
    }

    /// Attempts to create an [`ArrayType`] from an existing canonical type
    /// expression.
    ///
    /// Returns [`ArrayTypeError::NotArray`] when the expression is another
    /// type variant.
    pub fn try_from_type_expr(value: TypeExpr) -> Result<Self, ArrayTypeError> {
        match value {
            TypeExpr::Array { .. } => Ok(Self(value)),
            other => Err(ArrayTypeError::NotArray {
                variant: other.variant_name(),
            }),
        }
    }

    /// Attempts to create an [`ArrayType`] from an existing expression and
    /// structurally validates it using the supplied policy.
    pub fn try_from_type_expr_with_policy(
        value: TypeExpr,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, ArrayTypeError> {
        let array = Self::try_from_type_expr(value)?;

        array
            .validate_with_policy(policy)
            .map_err(ArrayTypeError::InvalidType)?;

        Ok(array)
    }

    /// Returns the canonical underlying [`TypeExpr`].
    ///
    /// This is the primary compatibility boundary with existing AST code.
    #[must_use]
    pub fn as_type_expr(&self) -> &TypeExpr {
        &self.0
    }

    /// Consumes this façade and returns the canonical [`TypeExpr`].
    #[must_use]
    pub fn into_type_expr(self) -> TypeExpr {
        self.0
    }

    /// Returns the array element type.
    ///
    /// The returned type is the canonical child owned by `TypeExpr::Array`.
    #[must_use]
    pub fn element(&self) -> &TypeExpr {
        match &self.0 {
            TypeExpr::Array { element, .. } => element.as_ref(),
            _ => {
                // The field is private and all public constructors enforce the
                // invariant. This branch is unreachable unless the internal
                // representation is changed incorrectly.
                unreachable!("ArrayType invariant violated")
            }
        }
    }

    /// Returns the optional source-level array cardinality expression.
    ///
    /// The result is deliberately not converted to `usize`.
    ///
    /// A symbolic expression may eventually depend on generic parameters or
    /// other source-level type information.
    #[must_use]
    pub fn length(&self) -> Option<&TypeValueExpr> {
        match &self.0 {
            TypeExpr::Array { length, .. } => length.as_ref(),
            _ => unreachable!("ArrayType invariant violated"),
        }
    }

    /// Returns whether an explicit source-level length expression exists.
    #[must_use]
    pub fn has_length(&self) -> bool {
        self.length().is_some()
    }

    /// Returns the number of dimensions represented by consecutive array
    /// constructors beginning at this type.
    ///
    /// For example:
    ///
    /// ```text
    /// T
    /// ```
    ///
    /// has zero array dimensions, while:
    ///
    /// ```text
    /// [[T; M]; N]
    /// ```
    ///
    /// has two.
    ///
    /// This operation walks only through the array façade structure and does
    /// not allocate.
    #[must_use]
    pub fn dimensions(&self) -> usize {
        let mut count = 0usize;
        let mut current = self.element();

        while let TypeExpr::Array { element, .. } = current {
            count += 1;
            current = element.as_ref();
        }

        count + 1
    }

    /// Returns whether the immediate element type is itself an array.
    #[must_use]
    pub fn has_array_element(&self) -> bool {
        matches!(self.element(), TypeExpr::Array { .. })
    }

    /// Returns the innermost non-array element type.
    ///
    /// For:
    ///
    /// ```text
    /// [[Qubit; M]; N]
    /// ```
    ///
    /// this returns:
    ///
    /// ```text
    /// Qubit
    /// ```
    ///
    /// The operation is iterative and does not recurse through the Rust call
    /// stack.
    #[must_use]
    pub fn innermost_element(&self) -> &TypeExpr {
        let mut current = self.element();

        while let TypeExpr::Array { element, .. } = current {
            current = element.as_ref();
        }

        current
    }

    /// Returns whether the array has no explicit cardinality.
    #[must_use]
    pub fn is_unsized(&self) -> bool {
        self.length().is_none()
    }

    /// Returns whether the array's immediate element is a source-level quantum
    /// type expression.
    ///
    /// This is a syntactic predicate only. It does not perform semantic
    /// quantum-type resolution.
    #[must_use]
    pub fn has_quantum_element(&self) -> bool {
        matches!(self.element(), TypeExpr::Quantum(_))
    }

    /// Returns whether the innermost element is a source-level quantum type
    /// expression.
    ///
    /// This is useful for inspecting nested resource arrays without coupling
    /// this module to quantum implementation details.
    #[must_use]
    pub fn has_quantum_innermost_element(&self) -> bool {
        matches!(self.innermost_element(), TypeExpr::Quantum(_))
    }

    /// Returns whether the canonical expression contains an inference
    /// placeholder.
    #[must_use]
    pub fn requires_inference(&self) -> bool {
        self.0.requires_inference()
    }

    /// Performs canonical structural validation using the default policy.
    pub fn validate(&self) -> Result<(), TypeExprError> {
        self.0.validate()
    }

    /// Performs canonical structural validation using an explicit compiler
    /// policy.
    pub fn validate_with_policy(
        &self,
        policy: &TypeValidationPolicy,
    ) -> Result<(), TypeExprError> {
        self.0.validate_with_policy(policy)
    }

    /// Returns the canonical source-level representation.
    ///
    /// Formatting remains owned by `TypeExpr`, preventing duplicate array
    /// serialization/formatting logic.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        self.0.to_source_string()
    }

    /// Returns the canonical AST variant name.
    ///
    /// For every valid `ArrayType`, this returns `"array"`.
    #[must_use]
    pub fn variant_name(&self) -> &'static str {
        self.0.variant_name()
    }

    /// Clones and returns the canonical [`TypeExpr`].
    ///
    /// Prefer [`Self::as_type_expr`] when borrowing is sufficient.
    #[must_use]
    pub fn to_type_expr(&self) -> TypeExpr {
        self.0.clone()
    }
}

impl From<ArrayType> for TypeExpr {
    fn from(value: ArrayType) -> Self {
        value.into_type_expr()
    }
}

impl TryFrom<TypeExpr> for ArrayType {
    type Error = ArrayTypeError;

    fn try_from(value: TypeExpr) -> Result<Self, Self::Error> {
        Self::try_from_type_expr(value)
    }
}

impl AsRef<TypeExpr> for ArrayType {
    fn as_ref(&self) -> &TypeExpr {
        self.as_type_expr()
    }
}

impl fmt::Display for ArrayType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_source_string())
    }
}

/// Structural errors specific to the [`ArrayType`] façade.
///
/// Canonical type-expression validation errors remain owned by
/// [`TypeExprError`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ArrayTypeError {
    /// The supplied expression was not an array.
    NotArray {
        /// Canonical variant name of the supplied expression.
        variant: &'static str,
    },

    /// The canonical array expression failed structural validation.
    InvalidType(TypeExprError),
}

impl fmt::Display for ArrayTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotArray { variant } => {
                write!(
                    formatter,
                    "expected array type expression, found {variant}"
                )
            }
            Self::InvalidType(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ArrayTypeError {}

impl From<TypeExprError> for ArrayTypeError {
    fn from(value: TypeExprError) -> Self {
        Self::InvalidType(value)
    }
}

/// Immutable iterator over an array's element dimensions.
///
/// This iterator is intentionally over the canonical `TypeExpr` children
/// rather than over a generated representation.
///
/// The first yielded value is the immediate element type, followed by nested
/// array element types.
///
/// For example:
///
/// ```text
/// [[T; M]; N]
/// ```
///
/// yields:
///
/// ```text
/// [T; M]
/// T
/// ```
pub struct ArrayDimensions<'a> {
    next: Option<&'a TypeExpr>,
}

impl<'a> Iterator for ArrayDimensions<'a> {
    type Item = &'a TypeExpr;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;

        match current {
            TypeExpr::Array { element, .. } => {
                self.next = Some(element.as_ref());
                Some(current)
            }
            _ => {
                self.next = None;
                Some(current)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl ArrayType {
    /// Iterates through this array and each nested array element until the
    /// innermost element is reached.
    ///
    /// The traversal is iterative and therefore does not grow the Rust call
    /// stack with array nesting depth.
    #[must_use]
    pub fn dimensions_iter(&self) -> ArrayDimensions<'_> {
        ArrayDimensions {
            next: Some(self.as_type_expr()),
        }
    }
}

impl<'a> IntoIterator for &'a ArrayType {
    type Item = &'a TypeExpr;
    type IntoIter = Iter<'a, TypeExpr>;

    fn into_iter(self) -> Self::IntoIter {
        match &self.0 {
            TypeExpr::Array { element, .. } => std::slice::from_ref(element.as_ref()).iter(),
            _ => unreachable!("ArrayType invariant violated"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn named(name: &str) -> TypeExpr {
        TypeExpr::name(name)
    }

    #[test]
    fn constructs_array_from_element() {
        let array = ArrayType::new(named("Element"), None);

        assert!(array.is_unsized());
        assert_eq!(array.element(), &named("Element"));
        assert_eq!(array.variant_name(), "array");
    }

    #[test]
    fn constructs_array_with_symbolic_length() {
        let array = ArrayType::new(
            named("Qubit"),
            Some(TypeValueExpr::Generic("N".into())),
        );

        assert!(array.has_length());
        assert_eq!(
            array.length(),
            Some(&TypeValueExpr::Generic("N".into()))
        );
    }

    #[test]
    fn symbolic_length_is_not_machine_sized() {
        let array = ArrayType::new(
            named("Qubit"),
            Some(TypeValueExpr::Identifier(
                super::super::type_expr::TypePath::single("N"),
            )),
        );

        assert!(array.has_length());
    }

    #[test]
    fn arbitrary_element_types_are_supported() {
        let elements = vec![
            named("Int"),
            named("Float"),
            named("UserType"),
            TypeExpr::tuple(vec![named("A"), named("B")]),
            TypeExpr::optional(named("T")),
            TypeExpr::quantum(named("State")),
            TypeExpr::function(vec![named("A")], named("B")),
        ];

        for element in elements {
            let array = ArrayType::new(element.clone(), None);
            assert_eq!(array.element(), &element);
        }
    }

    #[test]
    fn nested_arrays_are_supported_without_fixed_dimension_limits() {
        let mut ty = named("T");

        for _ in 0..256 {
            ty = TypeExpr::array(ty, None);
        }

        let array = ArrayType::try_from_type_expr(ty).expect("outer type is an array");

        assert_eq!(array.dimensions(), 256);
        assert_eq!(array.innermost_element(), &named("T"));
    }

    #[test]
    fn nested_array_dimension_iterator_is_iterative() {
        let mut ty = named("T");

        for _ in 0..128 {
            ty = TypeExpr::array(ty, None);
        }

        let array = ArrayType::try_from_type_expr(ty).expect("outer type is an array");

        let count = array.dimensions_iter().count();

        // The iterator includes every array layer plus the innermost element.
        assert_eq!(count, 129);
    }

    #[test]
    fn array_element_can_be_quantum_without_hardware_coupling() {
        let array = ArrayType::new(TypeExpr::quantum(named("State")), None);

        assert!(array.has_quantum_element());
        assert!(array.has_quantum_innermost_element());
    }

    #[test]
    fn nested_quantum_resource_array_remains_source_level() {
        let quantum = TypeExpr::quantum(named("Resource"));
        let inner = TypeExpr::array(quantum, None);
        let outer = TypeExpr::array(inner, None);

        let array = ArrayType::try_from_type_expr(outer).expect("array");

        assert!(!array.has_quantum_element());
        assert!(array.has_quantum_innermost_element());
        assert_eq!(array.dimensions(), 2);
    }

    #[test]
    fn inference_is_delegated_to_canonical_type_expression() {
        let array = ArrayType::new(named("_"), None);

        assert!(array.requires_inference());
    }

    #[test]
    fn canonical_conversion_is_lossless() {
        let original = TypeExpr::array(
            named("Element"),
            Some(TypeValueExpr::Generic("N".into())),
        );

        let façade = ArrayType::try_from_type_expr(original.clone()).expect("array");
        let recovered = façade.into_type_expr();

        assert_eq!(recovered, original);
    }

    #[test]
    fn non_array_expression_is_rejected() {
        let result = ArrayType::try_from_type_expr(named("NotArray"));

        assert!(matches!(
            result,
            Err(ArrayTypeError::NotArray { .. })
        ));
    }

    #[test]
    fn validation_uses_canonical_type_validation() {
        let array = ArrayType::new(named("Element"), None);

        assert!(array.validate().is_ok());
    }

    #[test]
    fn explicit_policy_is_forwarded() {
        let array = ArrayType::new(named("Element"), None);

        let policy = TypeValidationPolicy {
            max_collection_items: Some(0),
            ..TypeValidationPolicy::default()
        };

        // An array itself has one structural element type, so a policy that
        // permits no collection items must reject its canonical structure.
        assert!(array.validate_with_policy(&policy).is_err());
    }

    #[test]
    fn source_formatting_is_canonical() {
        let array = ArrayType::new(
            named("Element"),
            Some(TypeValueExpr::Generic("N".into())),
        );

        assert_eq!(array.to_source_string(), "[Element; N]");
        assert_eq!(array.to_string(), "[Element; N]");
    }

    #[test]
    fn display_matches_source_formatting() {
        let array = ArrayType::new(named("Element"), None);

        assert_eq!(format!("{array}"), array.to_source_string());
    }

    #[test]
    fn dimensions_iter_starts_with_the_array_expression() {
        let array = ArrayType::new(named("T"), None);

        let values: Vec<&TypeExpr> = array.dimensions_iter().collect();

        assert_eq!(values.len(), 2);
        assert_eq!(values[0], array.as_type_expr());
        assert_eq!(values[1], &named("T"));
    }

    #[test]
    fn borrowed_element_iteration_is_safe() {
        let array = ArrayType::new(named("T"), None);

        let elements: Vec<&TypeExpr> = (&array).into_iter().collect();

        assert_eq!(elements, vec![array.element()]);
    }
}