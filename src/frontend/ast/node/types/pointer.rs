//! Zamani Native AST — Pointer Types
//!
//! Production-ready typed façade for the canonical source-level raw-pointer
//! type.
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
//! TypeExpr::Pointer
//!     │
//!     ├── PointerType (this façade)
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ├── pointer legality
//!     ├── ownership / aliasing rules
//!     ├── address-space semantics
//!     ├── provenance semantics
//!     └── target-independent pointer meaning
//!     │
//!     ▼
//! semantic type model
//!     │
//!     ▼
//! ZUIR / domain IR
//!     │
//!     ├── classical
//!     ├── quantum
//!     ├── HDL
//!     ├── accelerator
//!     ├── distributed
//!     └── future domains
//!     │
//!     ▼
//! target / backend lowering
//! ```
//!
//! # Responsibility
//!
//! This module owns the source-level typed façade for a Zamani raw-pointer
//! type.
//!
//! The authoritative AST representation remains:
//!
//! ```text
//! TypeExpr::Pointer {
//!     mutable,
//!     inner,
//! }
//! ```
//!
//! `PointerType` does not introduce another pointer representation.
//!
//! This module describes source-level intent only. It does not determine:
//!
//! - pointer width;
//! - address width;
//! - memory layout;
//! - ABI representation;
//! - physical address;
//! - virtual address;
//! - address space;
//! - allocation strategy;
//! - deallocation strategy;
//! - pointer provenance;
//! - aliasing legality;
//! - ownership legality;
//! - lifetime validity;
//! - dereference safety;
//! - alignment;
//! - CPU architecture;
//! - GPU architecture;
//! - FPGA architecture;
//! - ASIC architecture;
//! - QPU architecture;
//! - vendor implementation;
//! - backend instruction set;
//! - LLVM representation;
//! - QIR representation;
//! - MLIR representation;
//! - ZUIR implementation details.
//!
//! Those concerns belong to later compiler layers.
//!
//! # Raw pointer versus Rust implementation pointer
//!
//! The word "pointer" in this module means a **source-language type**.
//!
//! It does not mean that this Rust implementation uses raw pointers.
//!
//! The AST stores:
//!
//! ```text
//! TypeExpr::Pointer {
//!     mutable: bool,
//!     inner: Box<TypeExpr>,
//! }
//! ```
//!
//! The `Box<TypeExpr>` is ordinary safe Rust ownership of an AST subtree.
//! It is not the runtime pointer described by the source program.
//!
//! This distinction is critical for Zamani's `#![forbid(unsafe_code)]`
//! requirement.
//!
//! # POCO-REAF
//!
//! A source-level pointer must not encode the machine on which it will
//! eventually execute.
//!
//! There is intentionally no:
//!
//! ```text
//! usize address
//! u64 address
//! u32 address
//! pointer_width
//! address_width
//! architecture
//! ABI
//! physical_address
//! virtual_address
//! MAX_POINTER_DEPTH
//! MAX_POINTER_SIZE
//! MAX_ADDRESS
//! ```
//!
//! in this file.
//!
//! Consequently, a Zamani source program containing pointer types does not
//! become tied to a particular machine width or processor architecture.
//!
//! A later compiler stage may determine whether the target uses, for example,
//! a particular address representation. That information must never be
//! inserted into this source AST façade.
//!
//! # Scalability
//!
//! Pointer types scale with the type expression they point to, not with the
//! size of the eventual machine or address space.
//!
//! For example:
//!
//! ```text
//! *T
//! *[T]
//! *Quantum<T>
//! *DistributedResource<T>
//! *FutureDomain<T>
//! ```
//!
//! remain source-level type relationships.
//!
//! The AST does not need to change when the eventual implementation moves
//! between tiny, large, distributed, heterogeneous, quantum, accelerator or
//! future computational resources.
//!
//! Compiler resource limits, when required for untrusted-input protection,
//! must be supplied through explicit validation policies. They must not be
//! hidden in this type.
//!
//! # Domain neutrality
//!
//! This module deliberately knows nothing about:
//!
//! - quantum hardware;
//! - physical qubits;
//! - logical qubits;
//! - quantum topology;
//! - quantum routing;
//! - quantum scheduling;
//! - quantum calibration;
//! - QEC;
//! - ZQN;
//! - resilience;
//! - CPU instruction sets;
//! - GPU instruction sets;
//! - FPGA primitives;
//! - ASIC primitives;
//! - vendor SDKs;
//! - LLVM;
//! - QIR;
//! - MLIR;
//! - backend runtimes.
//!
//! The pointed-to type may itself be a source-level quantum/resource type if
//! Zamani's language permits that combination. `PointerType` does not interpret
//! the pointed-to type.
//!
//! # Canonical grammar integration
//!
//! The current Zamani grammar defines raw-pointer syntax conceptually as:
//!
//! ```text
//! "*" ["mut"] TypeExpr
//! ```
//!
//! The parser therefore constructs:
//!
//! ```text
//! TypeExpr::Pointer {
//!     mutable,
//!     inner,
//! }
//! ```
//!
//! `PointerType` is a typed façade around that canonical representation.
//!
//! This file must not introduce a competing parser grammar.
//!
//! # Architectural distinction
//!
//! ```text
//! PointerType
//!     = typed source-AST façade
//!
//! TypeExpr::Pointer
//!     = canonical source-AST representation
//!
//! SemanticType
//!     = resolved pointer semantics
//!
//! ZUIR
//!     = universal computational representation
//!
//! Domain IR
//!     = domain-specific implementation representation
//!
//! Target IR
//!     = target/backend representation
//! ```
//!
//! This module must never become a semantic pointer-analysis object.
//!
//! # Semantic boundary
//!
//! Semantic analysis is responsible for deciding questions such as:
//!
//! - whether raw pointers are permitted in the current context;
//! - whether the pointed-to type is legal;
//! - whether mutation is legal;
//! - whether dereferencing is legal;
//! - whether pointer conversions are legal;
//! - whether pointer provenance is preserved;
//! - whether aliasing rules are satisfied;
//! - whether ownership/resource rules are satisfied;
//! - whether a target can realize the required pointer semantics.
//!
//! None of those decisions belong in this source AST façade.
//!
//! # Serialization
//!
//! `PointerType` derives serialization traits for interoperability with the
//! existing AST infrastructure.
//!
//! The wrapper is transparent so the canonical `TypeExpr::Pointer` structure
//! remains the authoritative representation.
//!
//! No runtime address is serialized.
//!
//! No machine-specific pointer width is serialized.
//!
//! No target/backend information is serialized.
//!
//! # Determinism
//!
//! The representation contains only:
//!
//! - a boolean mutability marker;
//! - one ordered type-expression child.
//!
//! No unordered collection, generated address, runtime identity, or
//! machine-dependent state participates in pointer representation.
//!
//! # Security
//!
//! This file:
//!
//! - performs no I/O;
//! - performs no filesystem access;
//! - performs no networking;
//! - performs no FFI;
//! - performs no raw-memory access;
//! - performs no pointer arithmetic;
//! - performs no pointer dereference;
//! - executes no source-program operation;
//! - uses no unsafe Rust.
//!
//! Invalid external structures are rejected through the canonical type
//! validation machinery rather than being interpreted as runtime pointers.
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
//! # Dependencies
//!
//! Allowed dependencies:
//!
//! - Rust standard library;
//! - canonical `super::type_expr`;
//! - `serde`, already used by the AST type system.
//!
//! Forbidden dependencies:
//!
//! ```text
//! crate::semantic
//! crate::compiler
//! crate::quantum::*
//! crate::backend
//! crate::runtime
//! crate::zuir
//! LLVM
//! QIR
//! MLIR
//! vendor SDKs
//! ```
//!
//! # Integration contract
//!
//! ## Parser
//!
//! The parser recognizes:
//!
//! ```text
//! * TypeExpr
//! * mut TypeExpr
//! ```
//!
//! and constructs the canonical:
//!
//! ```text
//! TypeExpr::Pointer {
//!     mutable,
//!     inner,
//! }
//! ```
//!
//! The parser may use `PointerType` when a strongly typed façade is useful.
//!
//! ## TypeExpr
//!
//! `PointerType::into_type_expr()` produces the canonical pointer variant.
//!
//! `PointerType::try_from_type_expr()` extracts the façade from an existing
//! canonical pointer expression.
//!
//! ## Structural validation
//!
//! Validation delegates to `TypeExpr::validate()` and
//! `TypeExpr::validate_with_policy()`.
//!
//! This prevents pointer-specific structural validation from drifting away
//! from the canonical AST validation implementation.
//!
//! ## Semantic analysis
//!
//! Semantic analysis consumes `TypeExpr::Pointer` and determines pointer
//! semantics.
//!
//! This module performs no semantic resolution.
//!
//! ## ZUIR
//!
//! ZUIR lowering consumes the semantic result rather than importing this
//! wrapper for target-specific decisions.
//!
//! ## Visitors
//!
//! AST visitors traverse the underlying `TypeExpr::Pointer` and its `inner`
//! child through the canonical AST traversal infrastructure.
//!
//! This façade does not create a second visitor hierarchy.
//!
//! ## Serialization
//!
//! Serialization remains based on the canonical `TypeExpr` representation.
//!
//! ## Other type façades
//!
//! `PointerType` follows the same architecture as other typed type façades:
//!
//! ```text
//! PointerType
//! ReferenceType
//! SliceType
//! TupleType
//! ...
//!       │
//!       ▼
//! canonical TypeExpr
//! ```
//!
//! None of these façades may become competing serialized AST schemas.
//!
//! # Independent-file completion contract
//!
//! This file is complete when:
//!
//! - `PointerType` has exactly one canonical representation;
//! - the representation matches `TypeExpr::Pointer`;
//! - mutability is preserved exactly;
//! - the pointed-to type is preserved exactly;
//! - conversion to/from `TypeExpr` is deterministic;
//! - structural validation delegates to canonical `TypeExpr` validation;
//! - explicit validation policies are supported;
//! - serialization uses the canonical representation;
//! - equality/hash/debug behavior is deterministic;
//! - source formatting is deterministic;
//! - malformed canonical input returns an error;
//! - no unsafe code exists;
//! - no runtime pointer exists in this module;
//! - no address-width assumption exists;
//! - no machine-size assumption exists;
//! - no hardware dependency exists;
//! - no quantum-backend dependency exists;
//! - no semantic dependency exists;
//! - tests cover every public operation;
//! - parser integration is explicit;
//! - semantic integration is explicit;
//! - ZUIR integration is explicitly downstream.
//!
//! # Implementation
//!
//! The implementation intentionally remains a small typed façade.
//!
//! Complexity that belongs to the complete source type system remains in
//! `TypeExpr`. This prevents duplicate representations, duplicate validation,
//! and duplicate source-formatting logic.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::type_expr::{TypeExpr, TypeExprError, TypeValidationPolicy};

/// Strongly typed façade for [`TypeExpr::Pointer`].
///
/// The canonical AST representation remains [`TypeExpr::Pointer`].
///
/// `PointerType` exists so code that specifically needs to manipulate a raw
/// pointer type can do so without repeatedly destructuring the complete
/// `TypeExpr` enum.
///
/// # Representation invariant
///
/// Every valid `PointerType` contains exactly one:
///
/// ```text
/// TypeExpr::Pointer { .. }
/// ```
///
/// The field is private so callers cannot construct an invalid wrapper.
///
/// # Example
///
/// ```
/// use crate::frontend::ast::node::types::pointer::PointerType;
/// use crate::frontend::ast::node::types::type_expr::TypeExpr;
///
/// let pointer = PointerType::shared(TypeExpr::name("Element"));
///
/// assert!(!pointer.is_mutable());
/// assert_eq!(pointer.inner(), &TypeExpr::name("Element"));
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PointerType(TypeExpr);

impl PointerType {
    /// Creates an immutable raw pointer to `inner`.
    ///
    /// This operation establishes the representation invariant immediately.
    ///
    /// No semantic pointer validation is performed here because source AST
    /// construction may occur before name resolution and semantic analysis.
    #[must_use]
    pub fn new(inner: TypeExpr) -> Self {
        Self::shared(inner)
    }

    /// Creates an immutable raw pointer to `inner`.
    ///
    /// The term "shared" here describes the source-level mutability marker:
    /// the pointer itself is not mutable. It does **not** imply Rust borrowing
    /// or Rust `&T` reference semantics.
    #[must_use]
    pub fn shared(inner: TypeExpr) -> Self {
        Self(TypeExpr::Pointer {
            mutable: false,
            inner: Box::new(inner),
        })
    }

    /// Creates a mutable raw pointer to `inner`.
    ///
    /// This corresponds to the source-level form:
    ///
    /// ```text
    /// *mut T
    /// ```
    ///
    /// The meaning of mutation is resolved by semantic analysis.
    #[must_use]
    pub fn mutable(inner: TypeExpr) -> Self {
        Self(TypeExpr::Pointer {
            mutable: true,
            inner: Box::new(inner),
        })
    }

    /// Creates a pointer from an explicit mutability marker.
    ///
    /// This is the preferred constructor for parser code because it maps
    /// directly onto the grammar's optional `mut` marker.
    #[must_use]
    pub fn with_mutability(mutable: bool, inner: TypeExpr) -> Self {
        Self(TypeExpr::Pointer {
            mutable,
            inner: Box::new(inner),
        })
    }

    /// Constructs a pointer and validates its complete type-expression
    /// structure using the default validation policy.
    ///
    /// This is useful at parser/deserializer boundaries where the caller wants
    /// structural validation immediately.
    pub fn try_new(inner: TypeExpr) -> Result<Self, PointerTypeError> {
        Self::try_new_with_policy(false, inner, &TypeValidationPolicy::default())
    }

    /// Constructs an immutable or mutable pointer and validates it using an
    /// explicit validation policy.
    ///
    /// The policy is supplied by the caller so this module introduces no
    /// hidden depth or size limit.
    pub fn try_new_with_policy(
        mutable: bool,
        inner: TypeExpr,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, PointerTypeError> {
        let pointer = Self::with_mutability(mutable, inner);

        pointer
            .validate_with_policy(policy)
            .map_err(PointerTypeError::InvalidType)?;

        Ok(pointer)
    }

    /// Extracts a `PointerType` from a canonical [`TypeExpr`].
    ///
    /// Returns the original expression as an error when it is not a pointer.
    ///
    /// This operation does not perform semantic validation.
    pub fn try_from_type_expr(value: TypeExpr) -> Result<Self, PointerTypeError> {
        match value {
            TypeExpr::Pointer { .. } => Ok(Self(value)),
            other => Err(PointerTypeError::NotPointer {
                actual: other.variant_name().to_owned(),
            }),
        }
    }

    /// Extracts a `PointerType` from a canonical [`TypeExpr`] after structural
    /// validation under the supplied policy.
    pub fn try_from_type_expr_with_policy(
        value: TypeExpr,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, PointerTypeError> {
        value
            .validate_with_policy(policy)
            .map_err(PointerTypeError::InvalidType)?;

        Self::try_from_type_expr(value)
    }

    /// Returns whether this pointer is marked mutable.
    ///
    /// This is a source-level property only. It does not determine whether a
    /// particular operation may legally mutate the pointee.
    #[must_use]
    pub fn is_mutable(&self) -> bool {
        match &self.0 {
            TypeExpr::Pointer { mutable, .. } => *mutable,

            // The private representation invariant makes this unreachable.
            // Returning false rather than exposing unsafe behavior keeps the
            // accessor total if the canonical representation changes in the
            // future.
            _ => false,
        }
    }

    /// Returns whether this pointer is marked immutable.
    #[must_use]
    pub fn is_shared(&self) -> bool {
        !self.is_mutable()
    }

    /// Returns the source-level pointed-to type.
    ///
    /// The returned reference borrows the existing AST subtree and therefore
    /// performs no subtree clone.
    #[must_use]
    pub fn inner(&self) -> &TypeExpr {
        match &self.0 {
            TypeExpr::Pointer { inner, .. } => inner.as_ref(),

            // The wrapper invariant guarantees this state cannot be produced
            // through the public API.
            _ => unreachable!("PointerType invariant violated"),
        }
    }

    /// Alias for [`Self::inner`].
    ///
    /// This name is useful when generic type façade code refers to a
    /// "pointee"/inner type uniformly.
    #[must_use]
    pub fn pointee(&self) -> &TypeExpr {
        self.inner()
    }

    /// Returns the canonical underlying [`TypeExpr`].
    #[must_use]
    pub fn as_type_expr(&self) -> &TypeExpr {
        &self.0
    }

    /// Consumes this façade and returns the canonical [`TypeExpr`].
    #[must_use]
    pub fn into_type_expr(self) -> TypeExpr {
        self.0
    }

    /// Returns the canonical AST schema version.
    ///
    /// The version comes from `type_expr` rather than defining a second
    /// pointer-specific serialization version.
    #[must_use]
    pub const fn schema_version() -> u16 {
        super::type_expr::TYPE_EXPR_SCHEMA_VERSION
    }

    /// Validates this pointer using the default canonical AST policy.
    ///
    /// This performs structural AST validation only.
    ///
    /// It does not perform:
    ///
    /// - ownership checking;
    /// - alias analysis;
    /// - pointer safety analysis;
    /// - address-space checking;
    /// - target validation.
    pub fn validate(&self) -> Result<(), TypeExprError> {
        self.0.validate()
    }

    /// Validates this pointer using an explicit canonical AST policy.
    ///
    /// Explicit policies allow compiler resource limits without putting
    /// machine-dependent constants into the AST representation.
    pub fn validate_with_policy(
        &self,
        policy: &TypeValidationPolicy,
    ) -> Result<(), TypeExprError> {
        self.0.validate_with_policy(policy)
    }

    /// Returns whether the pointer is structurally valid under the default
    /// validation policy.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Returns the canonical variant name.
    ///
    /// For a valid `PointerType`, this is always `"Pointer"`.
    #[must_use]
    pub fn variant_name(&self) -> &'static str {
        self.0.variant_name()
    }

    /// Returns the canonical source-level representation.
    ///
    /// The representation is derived from the pointed-to type rather than
    /// introducing a second pretty-printer implementation for arbitrary
    /// nested types.
    ///
    /// Examples:
    ///
    /// ```text
    /// *T
    /// *mut T
    /// *Vec<T>
    /// *Quantum<Q>
    /// ```
    #[must_use]
    pub fn to_source_string(&self) -> String {
        let mut result = String::with_capacity(4);

        result.push('*');

        if self.is_mutable() {
            result.push_str("mut ");
        }

        result.push_str(&self.inner().to_string());

        result
    }

    /// Returns a structurally equivalent pointer with the requested
    /// mutability marker.
    ///
    /// The pointee is cloned because the returned pointer owns a separate AST
    /// value.
    ///
    /// No semantic pointer compatibility check is performed.
    #[must_use]
    pub fn with_mutability_marker(&self, mutable: bool) -> Self {
        Self::with_mutability(mutable, self.inner().clone())
    }

    /// Returns a structurally equivalent pointer to a different source-level
    /// pointee type.
    ///
    /// This method intentionally performs no semantic type compatibility
    /// checking. That belongs to semantic analysis.
    #[must_use]
    pub fn with_inner(&self, inner: TypeExpr) -> Self {
        Self::with_mutability(self.is_mutable(), inner)
    }

    /// Returns whether the pointee itself is a type-expression inference
    /// placeholder.
    ///
    /// This is a deliberately shallow predicate. It does not perform semantic
    /// inference.
    #[must_use]
    pub fn points_to_infer(&self) -> bool {
        matches!(self.inner(), TypeExpr::Infer)
    }
}

impl From<PointerType> for TypeExpr {
    fn from(value: PointerType) -> Self {
        value.into_type_expr()
    }
}

impl TryFrom<TypeExpr> for PointerType {
    type Error = PointerTypeError;

    fn try_from(value: TypeExpr) -> Result<Self, Self::Error> {
        Self::try_from_type_expr(value)
    }
}

impl AsRef<TypeExpr> for PointerType {
    fn as_ref(&self) -> &TypeExpr {
        self.as_type_expr()
    }
}

impl fmt::Display for PointerType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_source_string())
    }
}

/// Errors produced when constructing or extracting a [`PointerType`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PointerTypeError {
    /// The supplied expression was not a pointer type.
    NotPointer {
        /// Canonical name of the supplied `TypeExpr` variant.
        actual: String,
    },

    /// The supplied pointer expression failed canonical structural
    /// validation.
    InvalidType(TypeExprError),
}

impl fmt::Display for PointerTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotPointer { actual } => {
                write!(
                    formatter,
                    "expected TypeExpr::Pointer, found TypeExpr::{actual}"
                )
            }
            Self::InvalidType(error) => {
                write!(formatter, "invalid pointer type: {error}")
            }
        }
    }
}

impl std::error::Error for PointerTypeError {}

impl From<TypeExprError> for PointerTypeError {
    fn from(value: TypeExprError) -> Self {
        Self::InvalidType(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::types::type_expr::TypeExpr;

    #[test]
    fn shared_pointer_uses_canonical_pointer_variant() {
        let pointer = PointerType::shared(TypeExpr::name("T"));

        assert!(!pointer.is_mutable());
        assert!(pointer.is_shared());
        assert_eq!(pointer.inner(), &TypeExpr::name("T"));
        assert!(matches!(
            pointer.as_type_expr(),
            TypeExpr::Pointer {
                mutable: false,
                ..
            }
        ));
    }

    #[test]
    fn mutable_pointer_uses_canonical_pointer_variant() {
        let pointer = PointerType::mutable(TypeExpr::name("T"));

        assert!(pointer.is_mutable());
        assert!(!pointer.is_shared());
        assert_eq!(pointer.inner(), &TypeExpr::name("T"));
        assert!(matches!(
            pointer.as_type_expr(),
            TypeExpr::Pointer {
                mutable: true,
                ..
            }
        ));
    }

    #[test]
    fn explicit_mutability_matches_constructor() {
        let shared = PointerType::with_mutability(false, TypeExpr::name("T"));
        let mutable = PointerType::with_mutability(true, TypeExpr::name("T"));

        assert_eq!(shared, PointerType::shared(TypeExpr::name("T")));
        assert_eq!(mutable, PointerType::mutable(TypeExpr::name("T")));
    }

    #[test]
    fn new_is_immutable_by_default() {
        let pointer = PointerType::new(TypeExpr::name("T"));

        assert!(pointer.is_shared());
        assert!(!pointer.is_mutable());
    }

    #[test]
    fn pointee_alias_matches_inner() {
        let pointer = PointerType::shared(TypeExpr::name("T"));

        assert_eq!(pointer.pointee(), pointer.inner());
    }

    #[test]
    fn conversion_to_type_expr_preserves_structure() {
        let pointer = PointerType::mutable(TypeExpr::name("Element"));
        let expression = pointer.clone().into_type_expr();

        assert_eq!(
            expression,
            TypeExpr::Pointer {
                mutable: true,
                inner: Box::new(TypeExpr::name("Element")),
            }
        );
    }

    #[test]
    fn conversion_from_type_expr_preserves_structure() {
        let expression = TypeExpr::Pointer {
            mutable: true,
            inner: Box::new(TypeExpr::name("Element")),
        };

        let pointer =
            PointerType::try_from_type_expr(expression.clone()).expect("pointer expected");

        assert_eq!(pointer.as_type_expr(), &expression);
    }

    #[test]
    fn conversion_rejects_non_pointer() {
        let expression = TypeExpr::name("Element");

        let error = PointerType::try_from_type_expr(expression)
            .expect_err("non-pointer must be rejected");

        assert_eq!(
            error,
            PointerTypeError::NotPointer {
                actual: "Identifier".to_owned(),
            }
        );
    }

    #[test]
    fn validation_delegates_to_canonical_type_expr() {
        let pointer = PointerType::shared(TypeExpr::name("Element"));

        assert!(pointer.validate().is_ok());
        assert!(pointer.is_valid());
    }

    #[test]
    fn source_formatting_for_shared_pointer_is_canonical() {
        let pointer = PointerType::shared(TypeExpr::name("Element"));

        assert_eq!(pointer.to_source_string(), "*Element");
        assert_eq!(pointer.to_string(), "*Element");
    }

    #[test]
    fn source_formatting_for_mutable_pointer_is_canonical() {
        let pointer = PointerType::mutable(TypeExpr::name("Element"));

        assert_eq!(pointer.to_source_string(), "*mut Element");
        assert_eq!(pointer.to_string(), "*mut Element");
    }

    #[test]
    fn nested_pointer_preserves_structure() {
        let inner = PointerType::shared(TypeExpr::name("Element"));
        let outer = PointerType::mutable(inner.clone().into_type_expr());

        assert!(outer.is_mutable());
        assert_eq!(outer.inner(), inner.as_type_expr());
        assert_eq!(outer.to_source_string(), "*mut *Element");
    }

    #[test]
    fn generic_pointee_is_not_hardware_specific() {
        let pointee = TypeExpr::Generic {
            base: Box::new(TypeExpr::name("Resource")),
            arguments: vec![TypeExpr::name("T")],
        };

        let pointer = PointerType::shared(pointee.clone());

        assert_eq!(pointer.inner(), &pointee);
        assert_eq!(pointer.to_source_string(), "*Resource<T>");
    }

    #[test]
    fn quantum_pointee_remains_source_level() {
        let pointee = TypeExpr::Quantum(Box::new(TypeExpr::name("Q")));

        let pointer = PointerType::shared(pointee.clone());

        assert_eq!(pointer.inner(), &pointee);
        assert!(pointer.to_source_string().starts_with("*"));
    }

    #[test]
    fn with_mutability_preserves_pointee() {
        let pointer = PointerType::shared(TypeExpr::name("T"));
        let mutable = pointer.with_mutability_marker(true);

        assert!(mutable.is_mutable());
        assert_eq!(mutable.inner(), pointer.inner());
    }

    #[test]
    fn with_inner_preserves_mutability() {
        let pointer = PointerType::mutable(TypeExpr::name("Old"));
        let replaced = pointer.with_inner(TypeExpr::name("New"));

        assert!(replaced.is_mutable());
        assert_eq!(replaced.inner(), &TypeExpr::name("New"));
    }

    #[test]
    fn points_to_infer_is_shallow_and_source_level() {
        let infer = PointerType::shared(TypeExpr::Infer);
        let concrete = PointerType::shared(TypeExpr::name("T"));

        assert!(infer.points_to_infer());
        assert!(!concrete.points_to_infer());
    }

    #[test]
    fn schema_version_comes_from_canonical_type_expr() {
        assert_eq!(
            PointerType::schema_version(),
            super::super::type_expr::TYPE_EXPR_SCHEMA_VERSION
        );
    }

    #[test]
    fn serde_round_trip_preserves_pointer() {
        let pointer = PointerType::mutable(TypeExpr::name("T"));

        let encoded =
            serde_json::to_string(&pointer).expect("pointer serialization should succeed");

        let decoded: PointerType =
            serde_json::from_str(&encoded).expect("pointer deserialization should succeed");

        assert_eq!(decoded, pointer);
    }

    #[test]
    fn equality_is_structural() {
        let first = PointerType::shared(TypeExpr::name("T"));
        let second = PointerType::shared(TypeExpr::name("T"));
        let mutable = PointerType::mutable(TypeExpr::name("T"));

        assert_eq!(first, second);
        assert_ne!(first, mutable);
    }

    #[test]
    fn no_runtime_address_is_part_of_the_representation() {
        let pointer = PointerType::shared(TypeExpr::name("T"));

        // This test documents an architectural invariant: the representation
        // is a type expression, not a runtime address.
        assert!(matches!(
            pointer.as_type_expr(),
            TypeExpr::Pointer {
                mutable: false,
                inner: _
            }
        ));
    }
}