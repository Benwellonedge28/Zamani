//! Zamani Native AST — Type Builder
//!
//! `src/frontend/ast/node/builders/type_builder.rs`
//!
//! Production construction API for canonical source-level Zamani types.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! Lexer
//!     │
//!     ▼
//! Parser
//!     │
//!     ▼
//! TypeBuilder                    ← this module
//!     │
//!     ▼
//! canonical TypeExpr
//!     │
//!     ▼
//! AST structural validation
//!     │
//!     ▼
//! Semantic type resolution
//!     │
//!     ▼
//! Semantic Model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── classical domain IR
//!     ├── quantum domain IR
//!     ├── hybrid domain IR
//!     ├── HDL domain IR
//!     └── future domain IRs
//! ```
//!
//! # Responsibility
//!
//! This module owns construction convenience for the existing canonical
//! [`TypeExpr`] representation.
//!
//! It does NOT define another type hierarchy.
//!
//! `TypeExpr` remains the single authoritative source-level type
//! representation.
//!
//! Specialized façade types such as `ArrayType`, `GenericType`, `TupleType`,
//! `FunctionType`, `ReferenceType`, `PointerType`, `OptionalType`, `ResultType`,
//! `NeverType`, and `UnitType` remain owned by their respective modules.
//!
//! # Important architectural rule
//!
//! A builder is construction infrastructure, not an additional AST model.
//!
//! Therefore this module MUST NOT introduce:
//!
//! ```text
//! TypeNode
//! TypeNodeKind
//! SemanticType
//! QuantumTypeNode
//! HardwareType
//! QIRType
//! MLIRType
//! LLVMType
//! ```
//!
//! `TypeExpr` is already the canonical source-level representation.
//!
//! # POCO-REAF
//!
//! The builder introduces no:
//!
//! - machine-size assumptions;
//! - pointer-width assumptions;
//! - register-size assumptions;
//! - qubit-count limits;
//! - quantum-register limits;
//! - hardware topology;
//! - vendor;
//! - backend;
//! - instruction set;
//! - scheduler;
//! - router;
//! - calibration;
//! - QEC implementation;
//! - runtime state.
//!
//! Type cardinality, generic arity, tuple size, array size and nested type
//! structure are represented using the existing dynamically sized source
//! structures.
//!
//! No artificial maximum is introduced here.
//!
//! "Infinity" means that the builder introduces no language-level finite ceiling;
//! actual compilation and execution remain bounded by available memory,
//! representation limits and explicitly configured compiler resource policies.
//!
//! # Dependency direction
//!
//! ```text
//! parser
//!   │
//!   ▼
//! TypeBuilder
//!   │
//!   ▼
//! TypeExpr
//!   │
//!   ▼
//! structural validation
//!   │
//!   ▼
//! semantic analysis
//!   │
//!   ▼
//! semantic type model
//!   │
//!   ▼
//! ZUIR
//! ```
//!
//! This module MUST NOT depend on:
//!
//! - semantic analysis;
//! - compiler backend implementation;
//! - ZUIR;
//! - quantum IR;
//! - hardware;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - runtime;
//! - vendor SDKs;
//! - OpenQASM ASTs;
//! - QIR;
//! - LLVM;
//! - MLIR.
//!
//! # Source-oriented representation
//!
//! The builder preserves source-level intent.
//!
//! For example:
//!
//! ```text
//! Quantum<Q>
//! Array<T, N>
//! Vec<T>
//! fn(A, B) -> C
//! &'a T
//! *const T
//! Option<T>
//! Result<T, E>
//! ```
//!
//! are represented without deciding:
//!
//! - physical memory layout;
//! - machine word width;
//! - physical qubit allocation;
//! - hardware representation;
//! - backend instruction selection.
//!
//! Those decisions belong to later compilation stages.
//!
//! # Validation boundary
//!
//! The builder does not duplicate `TypeExpr` validation.
//!
//! Where the canonical type API provides validation, the builder delegates to
//! it. Where construction is intentionally permissive so that parser recovery
//! can preserve malformed source, the resulting expression is left for the
//! canonical AST validation layer.
//!
//! The builder MUST NOT perform:
//!
//! - name resolution;
//! - type inference;
//! - type unification;
//! - trait resolution;
//! - generic substitution;
//! - ownership checking;
//! - borrow checking;
//! - resource allocation;
//! - quantum legality checking;
//! - topology checking;
//! - scheduling;
//! - routing;
//! - backend selection.
//!
//! # Parser contract
//!
//! The parser:
//!
//! 1. recognizes source syntax;
//! 2. determines source ordering;
//! 3. constructs child `TypeExpr` values;
//! 4. invokes this builder where convenient;
//! 5. inserts the resulting type expression into the surrounding AST node.
//!
//! The builder does not own parser state.
//!
//! # Semantic-analysis contract
//!
//! Semantic analysis consumes `TypeExpr` and resolves:
//!
//! - names;
//! - generic parameters;
//! - aliases;
//! - type constraints;
//! - resource semantics;
//! - domain semantics;
//! - target-independent type meaning.
//!
//! This builder must remain independent of those operations.
//!
//! # ZUIR contract
//!
//! There is no direct TypeBuilder → ZUIR dependency.
//!
//! The intended path is:
//!
//! ```text
//! TypeExpr
//!   ↓
//! semantic type resolution
//!   ↓
//! semantic model
//!   ↓
//! ZUIR lowering
//! ```
//!
//! A source-level type must never be forced to contain ZUIR implementation
//! details merely to simplify a later lowering stage.
//!
//! # Scalability
//!
//! No constants such as:
//!
//! ```text
//! MAX_TYPES
//! MAX_GENERIC_ARGUMENTS
//! MAX_TUPLE_ELEMENTS
//! MAX_ARRAY_LENGTH
//! MAX_TYPE_DEPTH
//! MAX_FUNCTION_PARAMETERS
//! MAX_QUANTUM_RESOURCES
//! ```
//!
//! are introduced.
//!
//! Existing `Vec`-based type structures remain responsible for dynamic
//! collections.
//!
//! Operational limits, when necessary to protect the compiler from hostile
//! input, belong to an explicit validation/resource-policy layer.
//!
//! # Determinism
//!
//! The builder:
//!
//! - preserves argument order;
//! - preserves tuple element order;
//! - preserves path segment order;
//! - performs no unordered iteration;
//! - performs no I/O;
//! - uses no random state;
//! - uses no timestamps;
//! - uses no global mutable state;
//! - does not inspect hardware.
//!
//! Given equivalent inputs, construction is deterministic.
//!
//! # Security
//!
//! This module:
//!
//! - contains no `unsafe`;
//! - performs no I/O;
//! - performs no network operations;
//! - executes no user code;
//! - performs no hardware access;
//! - does not dereference raw pointers;
//! - does not use unchecked indexing;
//! - does not use `unwrap()`;
//! - does not use `expect()`;
//! - introduces no global mutable state.
//!
//! # Serialization
//!
//! The builder itself is not serialized.
//!
//! The resulting `TypeExpr` is serialized by the canonical AST serialization
//! infrastructure.
//!
//! This prevents construction configuration from becoming hidden program
//! state.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use super::super::types::{
    FloatType,
    IntegerType,
    LifetimeName,
    PrimitiveType,
    TypeExpr,
    TypeName,
    TypeParameterName,
    TypePath,
};

/// Errors produced by [`TypeBuilder`].
///
/// Construction itself is intentionally lightweight. The canonical
/// `TypeExpr` representation owns semantic validation errors, so this error
/// type is reserved for construction-level failures that are actually
/// meaningful at this boundary.
///
/// At present construction of a `TypeExpr` is infallible for the canonical
/// source representation. The type exists so parser-facing APIs can use a
/// stable result boundary without inventing backend or semantic errors.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TypeBuilderError {
    /// Reserved for future construction failures that belong specifically to
    /// the builder boundary.
    ///
    /// No artificial failure is manufactured merely to populate this variant.
    Construction {
        /// Stable builder-level error message.
        message: String,
    },
}

impl core::fmt::Display for TypeBuilderError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Construction { message } => {
                write!(formatter, "type construction failed: {message}")
            }
        }
    }
}

impl std::error::Error for TypeBuilderError {}

/// Result type for type construction.
pub type TypeBuilderResult<T> = Result<T, TypeBuilderError>;

/// Production builder for the canonical Zamani [`TypeExpr`].
///
/// The builder owns no AST storage and no compiler state. It is a stateless
/// construction façade.
///
/// This is deliberate: type expressions do not require `NodeId` allocation in
/// the current repository representation. A surrounding AST node owns the
/// identity of the construct that contains the type expression.
///
/// # Example
///
/// ```ignore
/// let ty = TypeBuilder::named("Quantum");
///
/// let generic = TypeBuilder::generic(
///     ty,
///     vec![TypeBuilder::parameter("Q")],
/// );
/// ```
///
/// The exact parser integration may use either these associated constructors
/// or an instance created through [`Self::new`].
#[derive(Clone, Copy, Debug, Default)]
pub struct TypeBuilder;

impl TypeBuilder {
    /// Creates a stateless type builder.
    ///
    /// No allocator is required because the canonical `TypeExpr`
    /// representation is value-based rather than a `NodeId`-identified node.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Constructs a type expression from an already-built canonical value.
    ///
    /// This method is intentionally trivial: it provides a uniform builder
    /// boundary without cloning or transforming the expression.
    #[must_use]
    pub fn build(type_expr: TypeExpr) -> TypeExpr {
        type_expr
    }

    // =========================================================================
    // Names and paths
    // =========================================================================

    /// Constructs a source-level type name.
    #[must_use]
    pub fn type_name<S>(name: S) -> TypeName
    where
        S: Into<String>,
    {
        TypeName::new(name)
    }

    /// Constructs a source-level type path from ordered names.
    ///
    /// Source ordering is preserved exactly.
    #[must_use]
    pub fn path<I, S>(segments: I) -> TypePath
    where
        I: IntoIterator<Item = S>,
        S: Into<TypeName>,
    {
        TypePath::from_names(segments)
    }

    /// Constructs a single-segment named type.
    #[must_use]
    pub fn named<S>(name: S) -> TypeExpr
    where
        S: Into<TypeName>,
    {
        TypeExpr::Identifier(TypePath::single(name))
    }

    /// Constructs a named type from an already-built path.
    #[must_use]
    pub fn named_path(path: TypePath) -> TypeExpr {
        TypeExpr::Identifier(path)
    }

    /// Constructs a source-level type parameter reference.
    #[must_use]
    pub fn parameter<S>(name: S) -> TypeExpr
    where
        S: Into<String>,
    {
        TypeExpr::Parameter(TypeParameterName::new(name))
    }

    /// Constructs a source-level lifetime.
    #[must_use]
    pub fn lifetime<S>(name: S) -> LifetimeName
    where
        S: Into<String>,
    {
        LifetimeName::new(name)
    }

    // =========================================================================
    // Primitive types
    // =========================================================================

    /// Constructs a primitive type expression.
    #[must_use]
    pub fn primitive(primitive: PrimitiveType) -> TypeExpr {
        TypeExpr::Primitive(primitive)
    }

    /// Constructs the unit type.
    #[must_use]
    pub fn unit() -> TypeExpr {
        TypeExpr::Unit
    }

    /// Constructs the boolean type.
    #[must_use]
    pub fn bool() -> TypeExpr {
        TypeExpr::Primitive(PrimitiveType::Bool)
    }

    /// Constructs the character type.
    #[must_use]
    pub fn char() -> TypeExpr {
        TypeExpr::Primitive(PrimitiveType::Char)
    }

    /// Constructs the borrowed string type.
    #[must_use]
    pub fn str() -> TypeExpr {
        TypeExpr::Primitive(PrimitiveType::Str)
    }

    /// Constructs the owned string type.
    #[must_use]
    pub fn string() -> TypeExpr {
        TypeExpr::Primitive(PrimitiveType::String)
    }

    /// Constructs a signed integer type.
    #[must_use]
    pub const fn signed_integer(width: IntegerType) -> TypeExpr {
        TypeExpr::Primitive(PrimitiveType::Int(width))
    }

    /// Constructs an unsigned integer type.
    #[must_use]
    pub const fn unsigned_integer(width: IntegerType) -> TypeExpr {
        TypeExpr::Primitive(PrimitiveType::UInt(width))
    }

    /// Constructs a floating-point type.
    #[must_use]
    pub const fn float(width: FloatType) -> TypeExpr {
        TypeExpr::Primitive(PrimitiveType::Float(width))
    }

    /// Constructs `i8`.
    #[must_use]
    pub const fn i8() -> TypeExpr {
        Self::signed_integer(IntegerType::Bits8)
    }

    /// Constructs `i16`.
    #[must_use]
    pub const fn i16() -> TypeExpr {
        Self::signed_integer(IntegerType::Bits16)
    }

    /// Constructs `i32`.
    #[must_use]
    pub const fn i32() -> TypeExpr {
        Self::signed_integer(IntegerType::Bits32)
    }

    /// Constructs `i64`.
    #[must_use]
    pub const fn i64() -> TypeExpr {
        Self::signed_integer(IntegerType::Bits64)
    }

    /// Constructs `i128`.
    #[must_use]
    pub const fn i128() -> TypeExpr {
        Self::signed_integer(IntegerType::Bits128)
    }

    /// Constructs `isize`.
    ///
    /// The target-dependent nature of `isize` is preserved. The builder does
    /// not inspect the host compiler's pointer width.
    #[must_use]
    pub const fn isize() -> TypeExpr {
        Self::signed_integer(IntegerType::Size)
    }

    /// Constructs `u8`.
    #[must_use]
    pub const fn u8() -> TypeExpr {
        Self::unsigned_integer(IntegerType::Bits8)
    }

    /// Constructs `u16`.
    #[must_use]
    pub const fn u16() -> TypeExpr {
        Self::unsigned_integer(IntegerType::Bits16)
    }

    /// Constructs `u32`.
    #[must_use]
    pub const fn u32() -> TypeExpr {
        Self::unsigned_integer(IntegerType::Bits32)
    }

    /// Constructs `u64`.
    #[must_use]
    pub const fn u64() -> TypeExpr {
        Self::unsigned_integer(IntegerType::Bits64)
    }

    /// Constructs `u128`.
    #[must_use]
    pub const fn u128() -> TypeExpr {
        Self::unsigned_integer(IntegerType::Bits128)
    }

    /// Constructs `usize`.
    ///
    /// No host-machine width is embedded into the AST.
    #[must_use]
    pub const fn usize() -> TypeExpr {
        Self::unsigned_integer(IntegerType::Size)
    }

    /// Constructs `f32`.
    #[must_use]
    pub const fn f32() -> TypeExpr {
        Self::float(FloatType::Bits32)
    }

    /// Constructs `f64`.
    #[must_use]
    pub const fn f64() -> TypeExpr {
        Self::float(FloatType::Bits64)
    }

    /// Constructs `f128`.
    #[must_use]
    pub const fn f128() -> TypeExpr {
        Self::float(FloatType::Bits128)
    }

    // =========================================================================
    // Generic types
    // =========================================================================

    /// Constructs a generic type application.
    ///
    /// The base type and arguments are retained exactly as supplied.
    ///
    /// No generic-arity limit is imposed here.
    #[must_use]
    pub fn generic(
        base: TypeExpr,
        arguments: Vec<TypeExpr>,
    ) -> TypeExpr {
        TypeExpr::Generic {
            base: Box::new(base),
            arguments,
        }
    }

    /// Constructs a generic type application from a named constructor.
    ///
    /// Example:
    ///
    /// ```text
    /// Vec<Int>
    /// ```
    #[must_use]
    pub fn generic_named<S>(
        name: S,
        arguments: Vec<TypeExpr>,
    ) -> TypeExpr
    where
        S: Into<TypeName>,
    {
        Self::generic(Self::named(name), arguments)
    }

    // =========================================================================
    // Composite types
    // =========================================================================

    /// Constructs a tuple type.
    ///
    /// Element ordering is preserved.
    ///
    /// No tuple-arity limit is imposed.
    #[must_use]
    pub fn tuple(elements: Vec<TypeExpr>) -> TypeExpr {
        TypeExpr::Tuple(elements)
    }

    /// Constructs an array type with an optional source-level cardinality.
    ///
    /// The cardinality remains represented by the canonical
    /// [`TypeValueExpr`] abstraction rather than being converted to a
    /// machine-sized integer.
    #[must_use]
    pub fn array(
        element: TypeExpr,
        length: Option<super::super::types::TypeValueExpr>,
    ) -> TypeExpr {
        TypeExpr::Array {
            element: Box::new(element),
            length,
        }
    }

    /// Constructs a slice type.
    #[must_use]
    pub fn slice(element: TypeExpr) -> TypeExpr {
        TypeExpr::Slice {
            element: Box::new(element),
        }
    }

    /// Constructs a reference type.
    ///
    /// Lifetime and mutability remain source-level information.
    #[must_use]
    pub fn reference(
        lifetime: Option<LifetimeName>,
        inner: TypeExpr,
        mutable: bool,
    ) -> TypeExpr {
        TypeExpr::Reference {
            lifetime,
            inner: Box::new(inner),
            mutable,
        }
    }

    /// Constructs an immutable reference.
    #[must_use]
    pub fn shared_reference(
        inner: TypeExpr,
    ) -> TypeExpr {
        Self::reference(None, inner, false)
    }

    /// Constructs a mutable reference.
    #[must_use]
    pub fn mutable_reference(
        inner: TypeExpr,
    ) -> TypeExpr {
        Self::reference(None, inner, true)
    }

    /// Constructs a raw pointer.
    ///
    /// The AST records source intent only. It does not imply a particular
    /// physical pointer width or target ABI.
    #[must_use]
    pub fn pointer(
        inner: TypeExpr,
        mutable: bool,
    ) -> TypeExpr {
        TypeExpr::Pointer {
            inner: Box::new(inner),
            mutable,
        }
    }

    /// Constructs an optional type.
    #[must_use]
    pub fn optional(inner: TypeExpr) -> TypeExpr {
        TypeExpr::Optional {
            inner: Box::new(inner),
        }
    }

    /// Constructs a result type.
    #[must_use]
    pub fn result(
        success: TypeExpr,
        error: TypeExpr,
    ) -> TypeExpr {
        TypeExpr::Result {
            ok: Box::new(success),
            err: Box::new(error),
        }
    }

    /// Constructs the never type.
    #[must_use]
    pub fn never() -> TypeExpr {
        TypeExpr::Never
    }

    /// Constructs a function type.
    ///
    /// Parameter order is preserved and the parameter collection is not
    /// subject to a builder-defined limit.
    #[must_use]
    pub fn function(
        parameters: Vec<TypeExpr>,
        return_type: TypeExpr,
    ) -> TypeExpr {
        TypeExpr::Function {
            parameters,
            return_type: Box::new(return_type),
        }
    }

    // =========================================================================
    // Extensions
    // =========================================================================

    /// Constructs a canonical source-level type extension.
    ///
    /// This method is intentionally generic over the repository's extension
    /// representation. The builder does not enumerate future domains,
    /// quantum technologies or hardware vendors.
    #[must_use]
    pub fn extension(
        extension: super::super::types::TypeExtension,
    ) -> TypeExpr {
        TypeExpr::Extension(extension)
    }

    // =========================================================================
    // Validation delegation
    // =========================================================================

    /// Validates a type expression using the canonical `TypeExpr` validation
    /// policy.
    ///
    /// This method is deliberately a thin delegation boundary. It prevents
    /// validation rules from being duplicated between `TypeExpr` and its
    /// builder.
    pub fn validate(
        type_expr: &TypeExpr,
        policy: &super::super::types::TypeValidationPolicy,
    ) -> Result<(), super::super::types::TypeExprError> {
        type_expr.validate_with_policy(policy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_is_stateless() {
        let first = TypeBuilder::new();
        let second = TypeBuilder::new();

        assert_eq!(first, second);
    }

    #[test]
    fn build_preserves_canonical_type_expression() {
        let source = TypeBuilder::named("Quantum");

        assert_eq!(TypeBuilder::build(source.clone()), source);
    }

    #[test]
    fn named_type_preserves_path_order() {
        let path = TypeBuilder::path([
            "quantum",
            "resources",
            "Qubit",
        ]);

        let ty = TypeBuilder::named_path(path.clone());

        assert_eq!(
            ty,
            TypeExpr::Identifier(path)
        );
    }

    #[test]
    fn generic_type_preserves_argument_order() {
        let ty = TypeBuilder::generic_named(
            "Pair",
            vec![
                TypeBuilder::u64(),
                TypeBuilder::bool(),
                TypeBuilder::named("Quantum"),
            ],
        );

        match ty {
            TypeExpr::Generic {
                base,
                arguments,
            } => {
                assert_eq!(
                    *base,
                    TypeBuilder::named("Pair")
                );

                assert_eq!(
                    arguments,
                    vec![
                        TypeBuilder::u64(),
                        TypeBuilder::bool(),
                        TypeBuilder::named("Quantum"),
                    ]
                );
            }

            other => panic!(
                "expected generic type, got {other:?}"
            ),
        }
    }

    #[test]
    fn tuple_type_preserves_element_order() {
        let elements = vec![
            TypeBuilder::bool(),
            TypeBuilder::u64(),
            TypeBuilder::string(),
        ];

        let ty = TypeBuilder::tuple(elements.clone());

        assert_eq!(ty, TypeExpr::Tuple(elements));
    }

    #[test]
    fn target_dependent_integer_width_is_not_host_dependent() {
        assert_eq!(
            TypeBuilder::usize(),
            TypeExpr::Primitive(
                PrimitiveType::UInt(IntegerType::Size)
            )
        );

        assert_eq!(
            TypeBuilder::isize(),
            TypeExpr::Primitive(
                PrimitiveType::Int(IntegerType::Size)
            )
        );
    }

    #[test]
    fn nested_types_have_no_builder_defined_depth_limit() {
        let mut ty = TypeBuilder::unit();

        for _ in 0..128 {
            ty = TypeBuilder::optional(ty);
        }

        assert!(matches!(
            ty,
            TypeExpr::Optional { .. }
        ));
    }

    #[test]
    fn array_cardinality_remains_source_level() {
        let length = super::super::types::TypeValueExpr::from_source(
            "N"
        );

        let ty = TypeBuilder::array(
            TypeBuilder::named("Qubit"),
            Some(length.clone()),
        );

        match ty {
            TypeExpr::Array {
                element,
                length: actual,
            } => {
                assert_eq!(
                    *element,
                    TypeBuilder::named("Qubit")
                );
                assert_eq!(actual, Some(length));
            }

            other => panic!(
                "expected array type, got {other:?}"
            ),
        }
    }

    #[test]
    fn function_parameter_order_is_preserved() {
        let parameters = vec![
            TypeBuilder::u64(),
            TypeBuilder::bool(),
            TypeBuilder::named("Q"),
        ];

        let ty = TypeBuilder::function(
            parameters.clone(),
            TypeBuilder::unit(),
        );

        match ty {
            TypeExpr::Function {
                parameters: actual,
                return_type,
            } => {
                assert_eq!(actual, parameters);
                assert_eq!(*return_type, TypeBuilder::unit());
            }

            other => panic!(
                "expected function type, got {other:?}"
            ),
        }
    }

    #[test]
    fn references_do_not_encode_machine_information() {
        let ty = TypeBuilder::shared_reference(
            TypeBuilder::named("T")
        );

        match ty {
            TypeExpr::Reference {
                lifetime,
                inner,
                mutable,
            } => {
                assert_eq!(lifetime, None);
                assert!(!mutable);
                assert_eq!(
                    *inner,
                    TypeBuilder::named("T")
                );
            }

            other => panic!(
                "expected reference type, got {other:?}"
            ),
        }
    }

    #[test]
    fn pointer_does_not_encode_pointer_width() {
        let ty = TypeBuilder::pointer(
            TypeBuilder::u64(),
            false,
        );

        assert!(matches!(
            ty,
            TypeExpr::Pointer {
                mutable: false,
                ..
            }
        ));
    }

    #[test]
    fn optional_and_result_are_source_level() {
        let optional =
            TypeBuilder::optional(TypeBuilder::named("Q"));

        let result = TypeBuilder::result(
            TypeBuilder::named("State"),
            TypeBuilder::named("Error"),
        );

        assert!(matches!(
            optional,
            TypeExpr::Optional { .. }
        ));

        assert!(matches!(
            result,
            TypeExpr::Result { .. }
        ));
    }
}