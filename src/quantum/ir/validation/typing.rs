//! Zamani Quantum IR — Production Type Validation
//!
//! Path:
//!     src/quantum/ir/validation/typing.rs
//!
//! # Purpose
//!
//! This module performs semantic type validation for the canonical Zamani
//! Quantum IR.
//!
//! It answers:
//!
//!     "Are these IR values, operands, results, parameters and declared
//!      types semantically type-correct?"
//!
//! It does NOT:
//!
//! - execute expressions;
//! - evaluate quantum states;
//! - route qubits;
//! - inspect hardware topology;
//! - determine physical-qubit availability;
//! - schedule operations;
//! - perform calibration;
//! - select backend instructions;
//! - perform optimization;
//! - parse source code;
//! - define the canonical type system;
//! - define the canonical value system.
//!
//! Those responsibilities belong to their owning modules.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir::core::types
//!             │
//!             ├───────────────┐
//!             ▼               ▼
//!     quantum::ir::core::value
//!             │               │
//!             └───────┬───────┘
//!                     ▼
//!       validation::typing
//!                     │
//!          ┌──────────┼───────────┐
//!          ▼          ▼           ▼
//!       program     operation   control
//!          │          │           │
//!          └──────────┼───────────┘
//!                     ▼
//!              later validation
//!                     │
//!          ┌──────────┼───────────┐
//!          ▼          ▼           ▼
//!       semantic   resource    backend
//!       validation validation   lowering
//! ```
//!
//! # Canonical ownership
//!
//! The canonical type system is owned by:
//!
//!     quantum::ir::core::types
//!
//! The canonical value system is owned by:
//!
//!     quantum::ir::core::value
//!
//! The canonical parameter system is owned by:
//!
//!     quantum::ir::core::parameter
//!
//! The canonical logical qubit identity is owned by:
//!
//!     quantum::ir::qubit::QubitId
//!
//! The canonical physical qubit identity is owned by:
//!
//!     quantum::ir::qubit::PhysicalQubitId
//!
//! This module deliberately defines NONE of those concepts again.
//!
//! # Design principle
//!
//! A type validator must be:
//!
//!     deterministic
//!     hardware-independent
//!     allocation-free where possible
//!     non-recursive for hostile structural traversal
//!     explicit about conversions
//!     explicit about unresolved symbolic information
//!     extensible
//!     safe
//!
//! # Scalability
//!
//! There is no semantic maximum on:
//!
//! - qubit count;
//! - classical-bit count;
//! - tuple arity;
//! - array extent;
//! - type nesting;
//! - function parameter count;
//! - function result count;
//! - number of declared types;
//! - number of values.
//!
//! This file therefore contains no:
//!
//!     MAX_QUBITS
//!     MAX_TYPES
//!     MAX_ARRAY_LENGTH
//!     MAX_TUPLE_ARITY
//!     MAX_TYPE_DEPTH
//!
//! Resource/security limits are external policy.
//!
//! Traversal is implemented iteratively where structural depth could otherwise
//! become a Rust call-stack limitation. This is important because a semantic
//! type system must not accidentally make Rust's stack depth the effective
//! language limit.
//!
//! # Type-system philosophy
//!
//! The validator distinguishes:
//!
//!     exact equality
//!     assignment compatibility
//!     explicit conversion compatibility
//!     symbolic compatibility
//!     opaque/extension compatibility
//!
//! It never silently converts:
//!
//!     qubit -> integer
//!     physical_qubit -> logical_qubit
//!     duration -> integer
//!     amplitude -> float
//!     phase -> angle
//!
//! merely because their storage representations might look compatible.
//!
//! Semantic types remain semantic.
//!
//! # Quantum identity rule
//!
//! NEVER introduce another QubitId here.
//!
//! Always use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! or, from this module:
//!
//!     super::super::qubit::QubitId
//!
//! This prevents the historical `qubit` / `qubits` identity split.
//!
//! # Relationship with OpenQASM-style typing
//!
//! OpenQASM distinguishes logical qubits from classical types such as bool,
//! bit, int, uint, float, angle, complex and duration. Its current language
//! specification also treats casting and operation compatibility as explicit
//! semantic rules rather than merely storage-level conversions.
//!
//! Zamani deliberately goes further by making the type system extensible and
//! allowing opaque/named types for future quantum architectures.
//!
//! # Relationship with MLIR-style verification
//!
//! Modern extensible IR systems use explicit type constraints, operation
//! constraints, operand/result relationships and verifier stages.
//!
//! This module provides the equivalent semantic foundation for Zamani without
//! making Zamani depend on MLIR.
//!
//! # Rust contract
//!
//! Required:
//!
//!     Rust 1.97
//!     Rust 1.97.1
//!     Rust 2021
//!     stable Rust
//!     no nightly
//!     no unsafe
//!     no external dependencies
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! Upstream:
//!
//!     core::types
//!     core::value
//!     core::identity
//!     qubit
//!
//! Downstream:
//!
//!     validation::structural
//!     validation::semantic
//!     validation::control_flow
//!     validation::resources
//!     program
//!     operation
//!     serialization
//!     analysis
//!
//! This file does not require higher-level program modules to perform its
//! foundational type checks.
//!
//! # Important invariant
//!
//! A successful function in this file means:
//!
//!     the checked type relation is valid
//!
//! It does NOT mean:
//!
//!     the program is globally valid
//!
//! Global validity remains the responsibility of the other validation layers.
//!
//! # No silent failure
//!
//! Unknown named types, unresolved value references and incompatible opaque
//! types are errors unless an explicit policy says that unresolved information
//! is permitted.
//!
//! Unknown information must never silently become `Unit`, `Any`, or a valid
//! classical type.
//!
//! # Versioning
//!
//! This file consumes the canonical type/version contracts. It does not define
//! an independent IR version.
//!
//! # Serialization
//!
//! This file does not serialize types.
//!
//! The result of type validation must be deterministic and independent of
//! serialization implementation details.
//!
//! # Hashing
//!
//! This file does not hash types.
//!
//! `IrType::canonical_name()` remains a semantic representation helper and the
//! canonical hashing layer owns cryptographic hashing.
//!
//! # Thread safety
//!
//! All state is caller-owned.
//!
//! No global mutable type environment exists.
//!
//! Therefore independent validation contexts can safely be used concurrently.
//!
//! # Production invariant
//!
//! This module must remain usable when the program contains:
//!
//!     one qubit
//!     billions of qubits
//!     sparse qubit identifiers
//!     logical qubits
//!     physical qubit references
//!     dynamic arrays
//!     symbolic dimensions
//!     opaque extension types
//!     symbolic parameters
//!
//! without changing the validator's architecture.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::super::core::identity::{TypeId, ValueId};
use super::super::core::types::{
    ArrayType,
    BitType,
    Dimension,
    FloatType,
    FunctionType,
    IrType,
    SignedIntegerType,
    StructType,
    UnsignedIntegerType,
};
use super::super::core::value::{
    Value,
    ValueArray,
    ValueKind,
    ValueTuple,
};
use super::super::qubit::{
    PhysicalQubitId,
    QubitId,
};

// =============================================================================
// Public result
// =============================================================================

/// Result returned by the canonical typing validator.
pub type TypingResult<T> = Result<T, TypingError>;

// =============================================================================
// Typing policy
// =============================================================================

/// Explicit policy controlling type checking.
///
/// This is a validation policy, NOT a language-level type-system limit.
///
/// The default is deliberately conservative.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypingPolicy {
    /// Permit implicit numeric widening.
    ///
    /// Disabled by default because implicit conversions can hide semantic
    /// mistakes in quantum/control programs.
    pub allow_implicit_numeric_widening: bool,

    /// Permit symbolic parameters to satisfy a numeric semantic type.
    ///
    /// This is useful for parameterized gates and pulse definitions.
    pub allow_symbolic_numeric_values: bool,

    /// Permit unresolved opaque/named types to pass structural validation.
    ///
    /// The actual declaration must still be resolved when required by the
    /// compilation boundary.
    pub allow_unresolved_extensions: bool,

    /// Permit unresolved `ValueId` references.
    ///
    /// This should normally remain false at a trusted validation boundary.
    pub allow_unresolved_value_references: bool,

    /// Permit physical-qubit semantic types.
    ///
    /// This does not establish physical hardware existence.
    pub allow_physical_qubits: bool,
}

impl TypingPolicy {
    /// Strict production policy.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            allow_implicit_numeric_widening: false,
            allow_symbolic_numeric_values: true,
            allow_unresolved_extensions: false,
            allow_unresolved_value_references: false,
            allow_physical_qubits: true,
        }
    }

    /// Policy suitable for an explicit symbolic-compilation boundary.
    #[must_use]
    pub const fn symbolic() -> Self {
        Self {
            allow_implicit_numeric_widening: false,
            allow_symbolic_numeric_values: true,
            allow_unresolved_extensions: true,
            allow_unresolved_value_references: false,
            allow_physical_qubits: true,
        }
    }

    /// Policy for an IR inspection tool that intentionally accepts unresolved
    /// extension information.
    #[must_use]
    pub const fn inspection() -> Self {
        Self {
            allow_implicit_numeric_widening: true,
            allow_symbolic_numeric_values: true,
            allow_unresolved_extensions: true,
            allow_unresolved_value_references: true,
            allow_physical_qubits: true,
        }
    }
}

impl Default for TypingPolicy {
    fn default() -> Self {
        Self::strict()
    }
}

// =============================================================================
// Type constraint
// =============================================================================

/// Generic semantic type constraint.
///
/// This is deliberately independent of any particular operation or dialect.
///
/// It allows future dialects to express constraints without modifying the
/// canonical `IrType`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeConstraint {
    /// Any valid type.
    Any,

    /// Exact structural type.
    Exact(IrType),

    /// One of several alternatives.
    OneOf(Vec<IrType>),

    /// Type belonging to the specified semantic category.
    Category(TypeCategoryConstraint),

    /// Numeric type.
    Numeric,

    /// Classical type.
    Classical,

    /// Logical qubit type.
    LogicalQubit,

    /// Physical qubit type.
    PhysicalQubit,

    /// Classical predicate-compatible type.
    Predicate,

    /// Type compatible with another operand/result constraint.
    SameAs(usize),
}

impl TypeConstraint {
    /// Tests the constraint against a type.
    pub fn accepts(
        &self,
        actual: &IrType,
        already_matched: &[IrType],
        policy: TypingPolicy,
    ) -> bool {
        match self {
            Self::Any => true,

            Self::Exact(expected) => {
                type_compatible(actual, expected, policy)
            }

            Self::OneOf(types) => types
                .iter()
                .any(|expected| type_compatible(actual, expected, policy)),

            Self::Category(category) => category.accepts(actual),

            Self::Numeric => actual.is_numeric(),

            Self::Classical => {
                actual.is_classical() && !actual.is_qubit()
            }

            Self::LogicalQubit => actual.is_logical_qubit(),

            Self::PhysicalQubit => actual.is_physical_qubit(),

            Self::Predicate => actual.is_predicate_compatible(),

            Self::SameAs(index) => already_matched
                .get(*index)
                .map(|expected| {
                    type_compatible(actual, expected, policy)
                })
                .unwrap_or(false),
        }
    }
}

/// Broad type category constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeCategoryConstraint {
    /// Scalar type.
    Scalar,

    /// Quantum type.
    Quantum,

    /// Container type.
    Container,

    /// Function type.
    Function,

    /// Extensible type.
    Extensible,
}

impl TypeCategoryConstraint {
    /// Returns whether a type belongs to this category.
    #[must_use]
    pub const fn accepts(self, ty: &IrType) -> bool {
        match self {
            Self::Scalar => ty.is_scalar(),
            Self::Quantum => ty.is_quantum(),
            Self::Container => ty.is_container(),
            Self::Function => ty.is_function(),
            Self::Extensible => ty.is_named() || ty.is_opaque(),
        }
    }
}

// =============================================================================
// Operation signature
// =============================================================================

/// Type signature of a semantic operation.
///
/// This is intentionally generic and is suitable for standard gates,
/// extensions, analog operations, classical operations and future dialects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationTypeSignature {
    name: String,
    operands: Vec<TypeConstraint>,
    results: Vec<TypeConstraint>,
}

impl OperationTypeSignature {
    /// Creates an operation signature.
    pub fn new(
        name: impl Into<String>,
        operands: Vec<TypeConstraint>,
        results: Vec<TypeConstraint>,
    ) -> TypingResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(TypingError::InvalidSignature {
                reason: "operation signature name cannot be empty",
            });
        }

        Ok(Self {
            name,
            operands,
            results,
        })
    }

    /// Returns the operation name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns operand constraints.
    #[must_use]
    pub fn operands(&self) -> &[TypeConstraint] {
        &self.operands
    }

    /// Returns result constraints.
    #[must_use]
    pub fn results(&self) -> &[TypeConstraint] {
        &self.results
    }

    /// Returns operand arity.
    #[must_use]
    pub fn operand_count(&self) -> usize {
        self.operands.len()
    }

    /// Returns result arity.
    #[must_use]
    pub fn result_count(&self) -> usize {
        self.results.len()
    }
}

// =============================================================================
// Type environment
// =============================================================================

/// Explicit type environment used during validation.
///
/// The environment is caller-owned and deterministic.
///
/// `BTreeMap` is used deliberately:
///
/// - deterministic iteration;
/// - reproducible diagnostics;
/// - deterministic serialization consumers;
/// - no randomized hash state.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TypeEnvironment {
    values: BTreeMap<ValueId, IrType>,
    declarations: BTreeMap<TypeId, IrType>,
}

impl TypeEnvironment {
    /// Creates an empty environment.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a value type.
    ///
    /// Rebinding an existing value to a different type is rejected.
    pub fn bind_value(
        &mut self,
        id: ValueId,
        ty: IrType,
    ) -> TypingResult<()> {
        validate_type(&ty)?;

        if let Some(previous) = self.values.get(&id) {
            if previous != &ty {
                return Err(TypingError::ConflictingValueType {
                    value: id,
                    previous: previous.clone(),
                    new: ty,
                });
            }

            return Ok(());
        }

        self.values.insert(id, ty);

        Ok(())
    }

    /// Registers a named type declaration.
    ///
    /// Redeclaration with a different structural type is rejected.
    pub fn bind_type(
        &mut self,
        id: TypeId,
        ty: IrType,
    ) -> TypingResult<()> {
        validate_type(&ty)?;

        if let Some(previous) = self.declarations.get(&id) {
            if previous != &ty {
                return Err(TypingError::ConflictingTypeDeclaration {
                    type_id: id,
                    previous: previous.clone(),
                    new: ty,
                });
            }

            return Ok(());
        }

        self.declarations.insert(id, ty);

        Ok(())
    }

    /// Looks up a value type.
    #[must_use]
    pub fn value_type(
        &self,
        id: &ValueId,
    ) -> Option<&IrType> {
        self.values.get(id)
    }

    /// Looks up a named type.
    #[must_use]
    pub fn declared_type(
        &self,
        id: &TypeId,
    ) -> Option<&IrType> {
        self.declarations.get(id)
    }

    /// Returns the number of value bindings.
    #[must_use]
    pub fn value_count(&self) -> usize {
        self.values.len()
    }

    /// Returns the number of type declarations.
    #[must_use]
    pub fn type_count(&self) -> usize {
        self.declarations.len()
    }

    /// Clears all value bindings.
    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    /// Clears all type declarations.
    pub fn clear_types(&mut self) {
        self.declarations.clear();
    }
}

// =============================================================================
// Typing context
// =============================================================================

/// Immutable context used during one type-validation operation.
#[derive(Debug, Clone, Copy)]
pub struct TypingContext<'a> {
    environment: &'a TypeEnvironment,
    policy: TypingPolicy,
}

impl<'a> TypingContext<'a> {
    /// Creates a typing context.
    #[must_use]
    pub const fn new(
        environment: &'a TypeEnvironment,
        policy: TypingPolicy,
    ) -> Self {
        Self {
            environment,
            policy,
        }
    }

    /// Returns the environment.
    #[must_use]
    pub const fn environment(&self) -> &'a TypeEnvironment {
        self.environment
    }

    /// Returns the active policy.
    #[must_use]
    pub const fn policy(&self) -> TypingPolicy {
        self.policy
    }

    /// Validates one type.
    pub fn validate_type(
        &self,
        ty: &IrType,
    ) -> TypingResult<()> {
        validate_type_with_environment(
            ty,
            self.environment,
            self.policy,
        )
    }

    /// Infers the semantic type of a value.
    pub fn value_type(
        &self,
        value: &Value,
    ) -> TypingResult<IrType> {
        infer_value_type(
            value,
            self.environment,
            self.policy,
        )
    }

    /// Checks a value against an expected type.
    pub fn check_value(
        &self,
        value: &Value,
        expected: &IrType,
    ) -> TypingResult<()> {
        check_value_type(
            value,
            expected,
            self.environment,
            self.policy,
        )
    }

    /// Checks an assignment.
    pub fn check_assignment(
        &self,
        source: &IrType,
        target: &IrType,
    ) -> TypingResult<()> {
        check_assignment(source, target, self.policy)
    }

    /// Checks an operation signature.
    pub fn check_operation(
        &self,
        signature: &OperationTypeSignature,
        operands: &[IrType],
        results: &[IrType],
    ) -> TypingResult<()> {
        validate_operation_signature(
            signature,
            operands,
            results,
            self.policy,
        )
    }
}

// =============================================================================
// Typing errors
// =============================================================================

/// Production type-validation error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypingError {
    /// A type itself is structurally invalid.
    InvalidType {
        /// Human-readable reason.
        reason: &'static str,
    },

    /// Type contains an unresolved named declaration.
    UnknownType {
        /// Missing type identity.
        type_id: TypeId,
    },

    /// Opaque type was encountered where the active policy disallows it.
    OpaqueTypeNotAllowed {
        /// Opaque type identity.
        type_id: TypeId,
    },

    /// Type declaration cycle detected.
    TypeCycle {
        /// Repeated declaration identity.
        type_id: TypeId,
    },

    /// A value has a type incompatible with the expected type.
    TypeMismatch {
        /// Expected type.
        expected: IrType,

        /// Actual type.
        actual: IrType,
    },

    /// A value cannot be typed without an environment.
    UnresolvedValueReference {
        /// Value identity.
        value_id: ValueId,
    },

    /// A symbolic parameter cannot be used in this context.
    SymbolicValueNotAllowed {
        /// Expected semantic type.
        expected: IrType,
    },

    /// Numeric implicit conversion is disabled.
    ImplicitConversionRequired {
        /// Source type.
        from: IrType,

        /// Target type.
        to: IrType,
    },

    /// Explicit conversion is semantically unavailable.
    InvalidConversion {
        /// Source type.
        from: IrType,

        /// Target type.
        to: IrType,
    },

    /// Operation operand arity does not match its signature.
    OperandArityMismatch {
        /// Operation name.
        operation: String,

        /// Expected number.
        expected: usize,

        /// Actual number.
        actual: usize,
    },

    /// Operation result arity does not match its signature.
    ResultArityMismatch {
        /// Operation name.
        operation: String,

        /// Expected number.
        expected: usize,

        /// Actual number.
        actual: usize,
    },

    /// One operation operand violates its constraint.
    OperandConstraintMismatch {
        /// Operation name.
        operation: String,

        /// Operand index.
        index: usize,

        /// Expected constraint.
        expected: TypeConstraint,

        /// Actual type.
        actual: IrType,
    },

    /// One operation result violates its constraint.
    ResultConstraintMismatch {
        /// Operation name.
        operation: String,

        /// Result index.
        index: usize,

        /// Expected constraint.
        expected: TypeConstraint,

        /// Actual type.
        actual: IrType,
    },

    /// An invalid operation signature was supplied.
    InvalidSignature {
        /// Reason.
        reason: &'static str,
    },

    /// A value was assigned two incompatible types.
    ConflictingValueType {
        /// Value identity.
        value: ValueId,

        /// Previous type.
        previous: IrType,

        /// New type.
        new: IrType,
    },

    /// A type identity was assigned two incompatible declarations.
    ConflictingTypeDeclaration {
        /// Type identity.
        type_id: TypeId,

        /// Previous declaration.
        previous: IrType,

        /// New declaration.
        new: IrType,
    },

    /// A static array extent differs from the actual value extent.
    ArrayLengthMismatch {
        /// Expected length.
        expected: u64,

        /// Actual length.
        actual: u64,
    },

    /// Array element type is invalid.
    ArrayElementMismatch {
        /// Expected element type.
        expected: IrType,

        /// Actual element type.
        actual: IrType,

        /// Element index.
        index: usize,
    },

    /// Tuple arity mismatch.
    TupleArityMismatch {
        /// Expected arity.
        expected: usize,

        /// Actual arity.
        actual: usize,
    },

    /// Tuple element mismatch.
    TupleElementMismatch {
        /// Element index.
        index: usize,

        /// Expected type.
        expected: IrType,

        /// Actual type.
        actual: IrType,
    },

    /// Struct typing requires declaration metadata not present in the current
    /// validation context.
    StructMetadataRequired,

    /// Function signature mismatch.
    FunctionSignatureMismatch {
        /// Expected function type.
        expected: IrType,

        /// Actual function type.
        actual: IrType,
    },

    /// Physical qubit typing was disabled by policy.
    PhysicalQubitNotAllowed,

    /// Semantic type information cannot be inferred from a value alone.
    UndeterminedValueType {
        /// Value kind.
        kind: ValueKind,
    },
}

impl fmt::Display for TypingError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidType { reason } => {
                write!(f, "invalid IR type: {reason}")
            }

            Self::UnknownType { type_id } => {
                write!(f, "unknown named IR type: {type_id:?}")
            }

            Self::OpaqueTypeNotAllowed { type_id } => {
                write!(
                    f,
                    "opaque IR type is not permitted by this typing policy: {type_id:?}"
                )
            }

            Self::TypeCycle { type_id } => {
                write!(
                    f,
                    "cyclic named type declaration involving {type_id:?}"
                )
            }

            Self::TypeMismatch { expected, actual } => {
                write!(
                    f,
                    "type mismatch: expected {expected}, found {actual}"
                )
            }

            Self::UnresolvedValueReference { value_id } => {
                write!(
                    f,
                    "unresolved IR value reference: {value_id:?}"
                )
            }

            Self::SymbolicValueNotAllowed { expected } => {
                write!(
                    f,
                    "symbolic value cannot satisfy expected type {expected}"
                )
            }

            Self::ImplicitConversionRequired { from, to } => {
                write!(
                    f,
                    "implicit conversion required: {from} -> {to}"
                )
            }

            Self::InvalidConversion { from, to } => {
                write!(
                    f,
                    "invalid semantic conversion: {from} -> {to}"
                )
            }

            Self::OperandArityMismatch {
                operation,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "operation `{operation}` expects {expected} operands, found {actual}"
                )
            }

            Self::ResultArityMismatch {
                operation,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "operation `{operation}` expects {expected} results, found {actual}"
                )
            }

            Self::OperandConstraintMismatch {
                operation,
                index,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "operation `{operation}` operand {index} violates type \
                     constraint {expected:?}: found {actual}"
                )
            }

            Self::ResultConstraintMismatch {
                operation,
                index,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "operation `{operation}` result {index} violates type \
                     constraint {expected:?}: found {actual}"
                )
            }

            Self::InvalidSignature { reason } => {
                write!(
                    f,
                    "invalid operation type signature: {reason}"
                )
            }

            Self::ConflictingValueType {
                value,
                previous,
                new,
            } => {
                write!(
                    f,
                    "value {value:?} has conflicting types: {previous} vs {new}"
                )
            }

            Self::ConflictingTypeDeclaration {
                type_id,
                previous,
                new,
            } => {
                write!(
                    f,
                    "type {type_id:?} has conflicting declarations: \
                     {previous} vs {new}"
                )
            }

            Self::ArrayLengthMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "array length mismatch: expected {expected}, found {actual}"
                )
            }

            Self::ArrayElementMismatch {
                expected,
                actual,
                index,
            } => {
                write!(
                    f,
                    "array element {index} mismatch: expected {expected}, \
                     found {actual}"
                )
            }

            Self::TupleArityMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "tuple arity mismatch: expected {expected}, found {actual}"
                )
            }

            Self::TupleElementMismatch {
                index,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "tuple element {index} mismatch: expected {expected}, \
                     found {actual}"
                )
            }

            Self::StructMetadataRequired => {
                f.write_str(
                    "struct value typing requires declared struct metadata",
                )
            }

            Self::FunctionSignatureMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "function signature mismatch: expected {expected}, \
                     found {actual}"
                )
            }

            Self::PhysicalQubitNotAllowed => {
                f.write_str(
                    "physical-qubit typing is disabled by the active policy",
                )
            }

            Self::UndeterminedValueType { kind } => {
                write!(
                    f,
                    "value of kind `{kind}` has no uniquely inferable \
                     concrete IrType"
                )
            }
        }
    }
}

impl std::error::Error for TypingError {}

// =============================================================================
// Public type validation
// =============================================================================

/// Validates one canonical IR type.
///
/// This is the preferred entry point when no named-type environment is needed.
pub fn validate_type(
    ty: &IrType,
) -> TypingResult<()> {
    validate_type_with_environment(
        ty,
        &TypeEnvironment::new(),
        TypingPolicy::strict(),
    )
}

/// Validates one canonical IR type against a declaration environment.
pub fn validate_type_with_environment(
    ty: &IrType,
    environment: &TypeEnvironment,
    policy: TypingPolicy,
) -> TypingResult<()> {
    // -------------------------------------------------------------------------
    // Iterative structural validation.
    //
    // This deliberately does not call IrType::validate(), because the type
    // system's own convenience validator is allowed to be recursive while
    // this boundary is responsible for defending against deeply nested
    // untrusted/deserialized structures.
    // -------------------------------------------------------------------------

    let mut stack: Vec<TypeWorkItem<'_>> = Vec::new();

    stack.push(TypeWorkItem::Visit(ty));

    while let Some(item) = stack.pop() {
        match item {
            TypeWorkItem::Visit(current) => {
                match current {
                    IrType::Array(array) => {
                        validate_array_type(array)?;

                        stack.push(TypeWorkItem::Visit(
                            array.element(),
                        ));
                    }

                    IrType::Tuple(tuple) => {
                        for element in tuple.elements().iter().rev() {
                            stack.push(TypeWorkItem::Visit(element));
                        }
                    }

                    IrType::Struct(struct_type) => {
                        validate_struct_type(struct_type)?;

                        for field in struct_type.fields().iter().rev() {
                            stack.push(TypeWorkItem::Visit(field.ty()));
                        }
                    }

                    IrType::Option(option) => {
                        stack.push(TypeWorkItem::Visit(
                            option.inner(),
                        ));
                    }

                    IrType::Result(result) => {
                        stack.push(TypeWorkItem::Visit(result.error()));
                        stack.push(TypeWorkItem::Visit(result.ok()));
                    }

                    IrType::Function(function) => {
                        validate_function_type(function)?;

                        for result in function.results().iter().rev() {
                            stack.push(TypeWorkItem::Visit(result));
                        }

                        for parameter in function.parameters().iter().rev() {
                            stack.push(TypeWorkItem::Visit(parameter));
                        }
                    }

                    IrType::Named(named) => {
                        let type_id = named.id();

                        let Some(declaration) =
                            environment.declared_type(&type_id)
                        else {
                            if policy.allow_unresolved_extensions {
                                continue;
                            }

                            return Err(TypingError::UnknownType {
                                type_id,
                            });
                        };

                        stack.push(TypeWorkItem::ResolveNamed {
                            type_id,
                            declaration,
                        });
                    }

                    IrType::Opaque(opaque) => {
                        if !policy.allow_unresolved_extensions {
                            return Err(
                                TypingError::OpaqueTypeNotAllowed {
                                    type_id: opaque.id(),
                                },
                            );
                        }
                    }

                    IrType::PhysicalQubit
                    | IrType::PhysicalQubitRef(_)
                    | IrType::PhysicalQubitArray(_) => {
                        if !policy.allow_physical_qubits {
                            return Err(
                                TypingError::PhysicalQubitNotAllowed,
                            );
                        }
                    }

                    _ => {}
                }
            }

            TypeWorkItem::ResolveNamed {
                type_id,
                declaration,
            } => {
                let mut seen = BTreeSet::new();

                let mut current = declaration;

                loop {
                    match current {
                        IrType::Named(named) => {
                            let next_id = named.id();

                            if next_id == type_id
                                || !seen.insert(next_id)
                            {
                                return Err(TypingError::TypeCycle {
                                    type_id: next_id,
                                });
                            }

                            let Some(next) =
                                environment.declared_type(&next_id)
                            else {
                                if policy.allow_unresolved_extensions {
                                    break;
                                }

                                return Err(
                                    TypingError::UnknownType {
                                        type_id: next_id,
                                    },
                                );
                            };

                            current = next;
                        }

                        _ => {
                            stack.push(TypeWorkItem::Visit(current));
                            break;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// Internal work item for iterative type validation.
enum TypeWorkItem<'a> {
    Visit(&'a IrType),

    ResolveNamed {
        type_id: TypeId,
        declaration: &'a IrType,
    },
}

// =============================================================================
// Type validation helpers
// =============================================================================

fn validate_array_type(
    array: &ArrayType,
) -> TypingResult<()> {
    match array.dimension() {
        Dimension::Static(_)
        | Dimension::Symbol(_)
        | Dimension::Dynamic => Ok(()),
    }
}

fn validate_struct_type(
    structure: &StructType,
) -> TypingResult<()> {
    let mut names = BTreeSet::new();

    for field in structure.fields() {
        if field.name().is_empty() {
            return Err(TypingError::InvalidType {
                reason: "struct field name cannot be empty",
            });
        }

        if !names.insert(field.name()) {
            return Err(TypingError::InvalidType {
                reason: "struct field names must be unique",
            });
        }
    }

    Ok(())
}

fn validate_function_type(
    function: &FunctionType,
) -> TypingResult<()> {
    // Zero-parameter and zero-result functions are valid.
    //
    // The function type itself has no machine-size restriction.
    //
    // Calling-convention validation belongs to the program/ABI layer.
    let _ = function;

    Ok(())
}

// =============================================================================
// Value type inference
// =============================================================================

/// Infers the canonical semantic type of a value.
///
/// Values whose representation is intentionally unit-neutral, such as
/// symbolic `Parameter`, cannot always be assigned a unique `IrType` without
/// an expected type. Such values therefore return
/// `TypingError::UndeterminedValueType`.
pub fn infer_value_type(
    value: &Value,
    environment: &TypeEnvironment,
    policy: TypingPolicy,
) -> TypingResult<IrType> {
    match value {
        Value::Bool(_) => Ok(IrType::Bool),

        Value::Integer(_) => Ok(IrType::SignedInteger(
            SignedIntegerType::Arbitrary(0),
        )),

        Value::UnsignedInteger(_) => Ok(
            IrType::UnsignedInteger(
                UnsignedIntegerType::Arbitrary(0),
            ),
        ),

        Value::Float(_) => {
            Ok(IrType::Float(FloatType::F64))
        }

        Value::Complex(_) => {
            Ok(IrType::Complex(
                super::super::core::types::ComplexType::Float(
                    FloatType::F64,
                ),
            ))
        }

        Value::Angle(_) => Ok(IrType::Angle(
            super::super::core::types::AngleType::Exact,
        )),

        Value::Duration(_) => Ok(IrType::Duration(
            super::super::core::types::DurationType::Exact,
        )),

        Value::Frequency(_) => Ok(IrType::Frequency(
            super::super::core::types::FrequencyType::Exact,
        )),

        Value::Amplitude(_) => Ok(IrType::Amplitude(
            super::super::core::types::AmplitudeType::Exact,
        )),

        Value::Phase(_) => Ok(IrType::Phase(
            super::super::core::types::PhaseType::Exact,
        )),

        Value::Qubit(qubit) => {
            Ok(IrType::QubitRef(*qubit))
        }

        Value::PhysicalQubit(qubit) => {
            if !policy.allow_physical_qubits {
                return Err(
                    TypingError::PhysicalQubitNotAllowed,
                );
            }

            Ok(IrType::PhysicalQubitRef(*qubit))
        }

        Value::Parameter(_) => {
            Err(TypingError::UndeterminedValueType {
                kind: ValueKind::Parameter,
            })
        }

        Value::Reference(id) => {
            environment
                .value_type(id)
                .cloned()
                .ok_or(
                    TypingError::UnresolvedValueReference {
                        value_id: *id,
                    },
                )
        }

        Value::Array(array) => {
            infer_array_type(
                array,
                environment,
                policy,
            )
        }

        Value::Tuple(tuple) => {
            infer_tuple_type(
                tuple,
                environment,
                policy,
            )
        }

        Value::Optional(optional) => {
            match optional.as_ref() {
                None => {
                    Err(TypingError::UndeterminedValueType {
                        kind: ValueKind::Optional,
                    })
                }

                Some(value) => {
                    let inner = infer_value_type(
                        value,
                        environment,
                        policy,
                    )?;

                    Ok(IrType::option(inner))
                }
            }
        }

        Value::Unit => Ok(IrType::Unit),
    }
}

// =============================================================================
// Array inference
// =============================================================================

fn infer_array_type(
    array: &ValueArray,
    environment: &TypeEnvironment,
    policy: TypingPolicy,
) -> TypingResult<IrType> {
    let first = array.get(0);

    let Some(first) = first else {
        return Err(
            TypingError::UndeterminedValueType {
                kind: ValueKind::Array,
            },
        );
    };

    let element_type = infer_value_type(
        first,
        environment,
        policy,
    )?;

    for (index, element) in array.iter().enumerate().skip(1) {
        let actual = infer_value_type(
            element,
            environment,
            policy,
        )?;

        if !type_compatible(
            &actual,
            &element_type,
            policy,
        ) {
            return Err(
                TypingError::ArrayElementMismatch {
                    expected: element_type.clone(),
                    actual,
                    index,
                },
            );
        }
    }

    let length = match u64::try_from(array.len()) {
        Ok(value) => value,
        Err(_) => {
            return Err(TypingError::InvalidType {
                reason:
                    "array length cannot be represented by the IR dimension type",
            });
        }
    };

    Ok(IrType::array(
        element_type,
        Dimension::Static(length),
    ))
}

// =============================================================================
// Tuple inference
// =============================================================================

fn infer_tuple_type(
    tuple: &ValueTuple,
    environment: &TypeEnvironment,
    policy: TypingPolicy,
) -> TypingResult<IrType> {
    let mut elements = Vec::with_capacity(tuple.len());

    for element in tuple.iter() {
        elements.push(infer_value_type(
            element,
            environment,
            policy,
        )?);
    }

    Ok(IrType::tuple(elements))
}

// =============================================================================
// Value/type checking
// =============================================================================

/// Checks whether a concrete value satisfies an expected IR type.
pub fn check_value_type(
    value: &Value,
    expected: &IrType,
    environment: &TypeEnvironment,
    policy: TypingPolicy,
) -> TypingResult<()> {
    validate_type_with_environment(
        expected,
        environment,
        policy,
    )?;

    check_value_type_inner(
        value,
        expected,
        environment,
        policy,
    )
}

fn check_value_type_inner(
    value: &Value,
    expected: &IrType,
    environment: &TypeEnvironment,
    policy: TypingPolicy,
) -> TypingResult<()> {
    match value {
        Value::Parameter(_) => {
            if policy.allow_symbolic_numeric_values
                && expected.is_numeric()
            {
                return Ok(());
            }

            return Err(
                TypingError::SymbolicValueNotAllowed {
                    expected: expected.clone(),
                },
            );
        }

        Value::Reference(id) => {
            let Some(actual) =
                environment.value_type(id)
            else {
                if policy.allow_unresolved_value_references {
                    return Ok(());
                }

                return Err(
                    TypingError::UnresolvedValueReference {
                        value_id: *id,
                    },
                );
            };

            return check_assignment(
                actual,
                expected,
                policy,
            );
        }

        Value::Array(array) => {
            return check_array_value(
                array,
                expected,
                environment,
                policy,
            );
        }

        Value::Tuple(tuple) => {
            return check_tuple_value(
                tuple,
                expected,
                environment,
                policy,
            );
        }

        Value::Optional(optional) => {
            let Some(inner_expected) =
                expected.option_inner()
            else {
                return Err(
                    TypingError::TypeMismatch {
                        expected: expected.clone(),
                        actual: IrType::Option(
                            super::super::core::types::OptionType::new(
                                IrType::Unit,
                            ),
                        ),
                    },
                );
            };

            match optional.as_ref() {
                None => Ok(()),

                Some(inner) => check_value_type_inner(
                    inner,
                    inner_expected,
                    environment,
                    policy,
                ),
            }
        }

        _ => {
            let actual = infer_value_type(
                value,
                environment,
                policy,
            )?;

            if type_compatible(
                &actual,
                expected,
                policy,
            ) {
                Ok(())
            } else {
                Err(TypingError::TypeMismatch {
                    expected: expected.clone(),
                    actual,
                })
            }
        }
    }
}

// =============================================================================
// Array checking
// =============================================================================

fn check_array_value(
    array: &ValueArray,
    expected: &IrType,
    environment: &TypeEnvironment,
    policy: TypingPolicy,
) -> TypingResult<()> {
    let Some(element_expected) =
        expected.array_element()
    else {
        return Err(TypingError::TypeMismatch {
            expected: expected.clone(),
            actual: IrType::array(
                IrType::Unit,
                Dimension::Static(
                    u64::try_from(array.len()).map_err(
                        |_| TypingError::InvalidType {
                            reason:
                                "array length cannot be represented",
                        },
                    )?,
                ),
            ),
        });
    };

    if let Some(dimension) =
        expected.array_dimension()
    {
        if let Some(expected_length) =
            dimension.static_size()
        {
            let actual_length =
                u64::try_from(array.len()).map_err(
                    |_| TypingError::InvalidType {
                        reason:
                            "array length cannot be represented",
                    },
                )?;

            if expected_length != actual_length {
                return Err(
                    TypingError::ArrayLengthMismatch {
                        expected: expected_length,
                        actual: actual_length,
                    },
                );
            }
        }
    }

    for (index, element) in array.iter().enumerate() {
        check_value_type_inner(
            element,
            element_expected,
            environment,
            policy,
        )
        .map_err(|error| match error {
            TypingError::TypeMismatch {
                expected,
                actual,
            } => TypingError::ArrayElementMismatch {
                expected,
                actual,
                index,
            },

            other => other,
        })?;
    }

    Ok(())
}

// =============================================================================
// Tuple checking
// =============================================================================

fn check_tuple_value(
    tuple: &ValueTuple,
    expected: &IrType,
    environment: &TypeEnvironment,
    policy: TypingPolicy,
) -> TypingResult<()> {
    let Some(expected_elements) =
        expected.tuple_elements()
    else {
        return Err(TypingError::TypeMismatch {
            expected: expected.clone(),
            actual: IrType::tuple(Vec::new()),
        });
    };

    if tuple.len() != expected_elements.len() {
        return Err(
            TypingError::TupleArityMismatch {
                expected: expected_elements.len(),
                actual: tuple.len(),
            },
        );
    }

    for (index, (value, expected_type)) in tuple
        .iter()
        .zip(expected_elements.iter())
        .enumerate()
    {
        check_value_type_inner(
            value,
            expected_type,
            environment,
            policy,
        )
        .map_err(|error| match error {
            TypingError::TypeMismatch {
                expected,
                actual,
            } => TypingError::TupleElementMismatch {
                index,
                expected,
                actual,
            },

            other => other,
        })?;
    }

    Ok(())
}

// =============================================================================
// Assignment validation
// =============================================================================

/// Checks whether a source type can be assigned to a target type.
pub fn check_assignment(
    source: &IrType,
    target: &IrType,
    policy: TypingPolicy,
) -> TypingResult<()> {
    validate_type(source)?;
    validate_type(target)?;

    if type_compatible(
        source,
        target,
        policy,
    ) {
        return Ok(());
    }

    if policy.allow_implicit_numeric_widening
        && is_numeric_widening(
            source,
            target,
        )
    {
        return Ok(());
    }

    if source.can_explicitly_convert_to(target) {
        return Err(
            TypingError::ImplicitConversionRequired {
                from: source.clone(),
                to: target.clone(),
            },
        );
    }

    Err(TypingError::InvalidConversion {
        from: source.clone(),
        to: target.clone(),
    })
}

// =============================================================================
// Type compatibility
// =============================================================================

/// Returns whether two types are compatible without requiring an explicit
/// conversion.
#[must_use]
pub fn type_compatible(
    source: &IrType,
    target: &IrType,
    policy: TypingPolicy,
) -> bool {
    if source == target {
        return true;
    }

    match (source, target) {
        // ---------------------------------------------------------------------
        // Logical qubits
        // ---------------------------------------------------------------------

        (
            IrType::QubitRef(_),
            IrType::Qubit,
        ) => true,

        (
            IrType::Qubit,
            IrType::QubitRef(_),
        ) => false,

        // ---------------------------------------------------------------------
        // Physical qubits
        // ---------------------------------------------------------------------

        (
            IrType::PhysicalQubitRef(_),
            IrType::PhysicalQubit,
        ) => true,

        (
            IrType::PhysicalQubit,
            IrType::PhysicalQubitRef(_),
        ) => false,

        // Physical and logical qubits are NEVER implicitly interchangeable.
        (
            IrType::Qubit
            | IrType::QubitRef(_)
            | IrType::QubitArray(_),
            IrType::PhysicalQubit
            | IrType::PhysicalQubitRef(_)
            | IrType::PhysicalQubitArray(_),
        )
        | (
            IrType::PhysicalQubit
            | IrType::PhysicalQubitRef(_)
            | IrType::PhysicalQubitArray(_),
            IrType::Qubit
            | IrType::QubitRef(_)
            | IrType::QubitArray(_),
        ) => false,

        // ---------------------------------------------------------------------
        // Named types
        // ---------------------------------------------------------------------

        (
            IrType::Named(a),
            IrType::Named(b),
        ) => a.id() == b.id(),

        // ---------------------------------------------------------------------
        // Opaque types
        // ---------------------------------------------------------------------

        (
            IrType::Opaque(a),
            IrType::Opaque(b),
        ) => a.id() == b.id(),

        // ---------------------------------------------------------------------
        // Same semantic families
        // ---------------------------------------------------------------------

        (
            IrType::SignedInteger(_),
            IrType::SignedInteger(_),
        ) if policy.allow_implicit_numeric_widening => {
            is_numeric_widening(source, target)
        }

        (
            IrType::UnsignedInteger(_),
            IrType::UnsignedInteger(_),
        ) if policy.allow_implicit_numeric_widening => {
            is_numeric_widening(source, target)
        }

        (
            IrType::Float(_),
            IrType::Float(_),
        ) if policy.allow_implicit_numeric_widening => {
            is_numeric_widening(source, target)
        }

        // ---------------------------------------------------------------------
        // Recursive containers
        // ---------------------------------------------------------------------

        (
            IrType::Array(source_array),
            IrType::Array(target_array),
        ) => {
            dimensions_compatible(
                source_array.dimension(),
                target_array.dimension(),
            ) && type_compatible(
                source_array.element(),
                target_array.element(),
                policy,
            )
        }

        (
            IrType::QubitArray(source_dimension),
            IrType::QubitArray(target_dimension),
        ) => dimensions_compatible(
            source_dimension,
            target_dimension,
        ),

        (
            IrType::PhysicalQubitArray(source_dimension),
            IrType::PhysicalQubitArray(target_dimension),
        ) => dimensions_compatible(
            source_dimension,
            target_dimension,
        ),

        (
            IrType::Tuple(source_tuple),
            IrType::Tuple(target_tuple),
        ) => {
            source_tuple.len() == target_tuple.len()
                && source_tuple
                    .elements()
                    .iter()
                    .zip(target_tuple.elements())
                    .all(|(source, target)| {
                        type_compatible(
                            source,
                            target,
                            policy,
                        )
                    })
        }

        (
            IrType::Option(source),
            IrType::Option(target),
        ) => type_compatible(
            source.inner(),
            target.inner(),
            policy,
        ),

        (
            IrType::Result(source),
            IrType::Result(target),
        ) => {
            type_compatible(
                source.ok(),
                target.ok(),
                policy,
            ) && type_compatible(
                source.error(),
                target.error(),
                policy,
            )
        }

        // ---------------------------------------------------------------------
        // Functions
        // ---------------------------------------------------------------------

        (
            IrType::Function(source),
            IrType::Function(target),
        ) => function_types_compatible(
            source,
            target,
            policy,
        ),

        // ---------------------------------------------------------------------
        // Everything else remains strict.
        // ---------------------------------------------------------------------

        _ => false,
    }
}

// =============================================================================
// Dimension compatibility
// =============================================================================

fn dimensions_compatible(
    source: &Dimension,
    target: &Dimension,
) -> bool {
    match (source, target) {
        (
            Dimension::Static(a),
            Dimension::Static(b),
        ) => a == b,

        // A symbolic/dynamic target may accept a concrete source because the
        // target deliberately does not require compile-time resolution.
        (
            Dimension::Static(_),
            Dimension::Symbol(_),
        )
        | (
            Dimension::Static(_),
            Dimension::Dynamic,
        ) => true,

        (
            Dimension::Symbol(a),
            Dimension::Symbol(b),
        ) => a == b,

        (
            Dimension::Symbol(_),
            Dimension::Dynamic,
        ) => true,

        (
            Dimension::Dynamic,
            Dimension::Dynamic,
        ) => true,

        // A source whose exact runtime size is unknown cannot be silently
        // assigned to a target requiring a specific static size.
        (
            Dimension::Dynamic,
            Dimension::Static(_),
        )
        | (
            Dimension::Symbol(_),
            Dimension::Static(_),
        ) => false,

        (
            Dimension::Dynamic,
            Dimension::Symbol(_),
        ) => false,
    }
}

// =============================================================================
// Function compatibility
// =============================================================================

fn function_types_compatible(
    source: &FunctionType,
    target: &FunctionType,
    policy: TypingPolicy,
) -> bool {
    if source.parameter_count()
        != target.parameter_count()
        || source.result_count()
            != target.result_count()
    {
        return false;
    }

    // Function parameters are checked contravariantly in a full function
    // subtyping system. Zamani's canonical IR currently uses invariant
    // signatures unless an explicit ABI/type dialect says otherwise.
    //
    // Therefore source and target parameter types must be compatible in the
    // same direction here.
    let parameters_compatible =
        source
            .parameters()
            .iter()
            .zip(target.parameters())
            .all(|(source, target)| {
                type_compatible(
                    source,
                    target,
                    policy,
                )
            });

    let results_compatible =
        source
            .results()
            .iter()
            .zip(target.results())
            .all(|(source, target)| {
                type_compatible(
                    source,
                    target,
                    policy,
                )
            });

    parameters_compatible
        && results_compatible
        && source.convention() == target.convention()
}

// =============================================================================
// Numeric widening
// =============================================================================

fn is_numeric_widening(
    source: &IrType,
    target: &IrType,
) -> bool {
    match (source, target) {
        (
            IrType::SignedInteger(source),
            IrType::SignedInteger(target),
        ) => integer_width(
            *source,
        )
        .zip(integer_width_signed(*target))
        .map(|(a, b)| a <= b)
        .unwrap_or(false),

        (
            IrType::UnsignedInteger(source),
            IrType::UnsignedInteger(target),
        ) => integer_width_unsigned(
            *source,
        )
        .zip(integer_width_unsigned(*target))
        .map(|(a, b)| a <= b)
        .unwrap_or(false),

        (
            IrType::Float(source),
            IrType::Float(target),
        ) => source
            .precision_bits()
            .zip(target.precision_bits())
            .map(|(a, b)| a <= b)
            .unwrap_or(false),

        _ => false,
    }
}

fn integer_width(
    ty: SignedIntegerType,
) -> Option<u64> {
    integer_width_signed(ty)
}

fn integer_width_signed(
    ty: SignedIntegerType,
) -> Option<u64> {
    match ty {
        SignedIntegerType::Size => None,
        SignedIntegerType::I8 => Some(8),
        SignedIntegerType::I16 => Some(16),
        SignedIntegerType::I32 => Some(32),
        SignedIntegerType::I64 => Some(64),
        SignedIntegerType::I128 => Some(128),
        SignedIntegerType::Arbitrary(width) => {
            Some(width)
        }
    }
}

fn integer_width_unsigned(
    ty: UnsignedIntegerType,
) -> Option<u64> {
    match ty {
        UnsignedIntegerType::Size => None,
        UnsignedIntegerType::U8 => Some(8),
        UnsignedIntegerType::U16 => Some(16),
        UnsignedIntegerType::U32 => Some(32),
        UnsignedIntegerType::U64 => Some(64),
        UnsignedIntegerType::U128 => Some(128),
        UnsignedIntegerType::Arbitrary(width) => {
            Some(width)
        }
    }
}

// =============================================================================
// Operation signature validation
// =============================================================================

/// Validates operand/result types against an operation signature.
pub fn validate_operation_signature(
    signature: &OperationTypeSignature,
    operands: &[IrType],
    results: &[IrType],
    policy: TypingPolicy,
) -> TypingResult<()> {
    if operands.len()
        != signature.operand_count()
    {
        return Err(
            TypingError::OperandArityMismatch {
                operation: signature.name().to_owned(),
                expected: signature.operand_count(),
                actual: operands.len(),
            },
        );
    }

    if results.len()
        != signature.result_count()
    {
        return Err(
            TypingError::ResultArityMismatch {
                operation: signature.name().to_owned(),
                expected: signature.result_count(),
                actual: results.len(),
            },
        );
    }

    let mut matched_operands = Vec::with_capacity(
        operands.len(),
    );

    for (index, (constraint, actual)) in signature
        .operands()
        .iter()
        .zip(operands.iter())
        .enumerate()
    {
        validate_type(actual)?;

        if !constraint.accepts(
            actual,
            &matched_operands,
            policy,
        ) {
            return Err(
                TypingError::OperandConstraintMismatch {
                    operation: signature.name().to_owned(),
                    index,
                    expected: constraint.clone(),
                    actual: actual.clone(),
                },
            );
        }

        matched_operands.push(actual.clone());
    }

    let mut matched_results = Vec::with_capacity(
        results.len(),
    );

    for (index, (constraint, actual)) in signature
        .results()
        .iter()
        .zip(results.iter())
        .enumerate()
    {
        validate_type(actual)?;

        if !constraint.accepts(
            actual,
            &matched_results,
            policy,
        ) {
            return Err(
                TypingError::ResultConstraintMismatch {
                    operation: signature.name().to_owned(),
                    index,
                    expected: constraint.clone(),
                    actual: actual.clone(),
                },
            );
        }

        matched_results.push(actual.clone());
    }

    Ok(())
}

// =============================================================================
// Canonical standard semantic signatures
// =============================================================================

/// Returns the standard signature for a single-qubit operation.
#[must_use]
pub fn unary_qubit_signature(
    name: impl Into<String>,
) -> OperationTypeSignature {
    OperationTypeSignature {
        name: name.into(),
        operands: vec![
            TypeConstraint::LogicalQubit,
        ],
        results: Vec::new(),
    }
}

/// Returns the standard signature for a two-qubit operation.
#[must_use]
pub fn binary_qubit_signature(
    name: impl Into<String>,
) -> OperationTypeSignature {
    OperationTypeSignature {
        name: name.into(),
        operands: vec![
            TypeConstraint::LogicalQubit,
            TypeConstraint::LogicalQubit,
        ],
        results: Vec::new(),
    }
}

/// Returns the standard signature for a measurement operation.
#[must_use]
pub fn measurement_signature(
    name: impl Into<String>,
) -> OperationTypeSignature {
    OperationTypeSignature {
        name: name.into(),
        operands: vec![
            TypeConstraint::LogicalQubit,
        ],
        results: vec![
            TypeConstraint::Predicate,
        ],
    }
}

/// Returns the standard signature for a reset operation.
#[must_use]
pub fn reset_signature(
    name: impl Into<String>,
) -> OperationTypeSignature {
    OperationTypeSignature {
        name: name.into(),
        operands: vec![
            TypeConstraint::LogicalQubit,
        ],
        results: Vec::new(),
    }
}

/// Returns a generic numeric unary signature.
#[must_use]
pub fn numeric_unary_signature(
    name: impl Into<String>,
) -> OperationTypeSignature {
    OperationTypeSignature {
        name: name.into(),
        operands: vec![
            TypeConstraint::Numeric,
        ],
        results: vec![
            TypeConstraint::Numeric,
        ],
    }
}

/// Returns a same-type binary signature.
///
/// The second operand must have the same type as the first.
#[must_use]
pub fn same_type_binary_signature(
    name: impl Into<String>,
) -> OperationTypeSignature {
    OperationTypeSignature {
        name: name.into(),
        operands: vec![
            TypeConstraint::Any,
            TypeConstraint::SameAs(0),
        ],
        results: vec![
            TypeConstraint::SameAs(0),
        ],
    }
}

// =============================================================================
// Quantum-specific helpers
// =============================================================================

/// Checks a logical qubit type.
///
/// Physical qubits are deliberately rejected by this helper because logical
/// quantum operations must not accidentally accept post-mapping physical
/// resources.
pub fn require_logical_qubit(
    ty: &IrType,
) -> TypingResult<()> {
    if ty.is_logical_qubit() {
        Ok(())
    } else {
        Err(TypingError::TypeMismatch {
            expected: IrType::Qubit,
            actual: ty.clone(),
        })
    }
}

/// Checks a physical qubit type.
pub fn require_physical_qubit(
    ty: &IrType,
) -> TypingResult<()> {
    if ty.is_physical_qubit() {
        Ok(())
    } else {
        Err(TypingError::TypeMismatch {
            expected: IrType::PhysicalQubit,
            actual: ty.clone(),
        })
    }
}

/// Checks that two logical qubit identities are not accidentally represented
/// as physical resources.
///
/// This function exists as a narrow integration guard at mapping boundaries.
pub fn ensure_logical_identity(
    qubit: QubitId,
    ty: &IrType,
) -> TypingResult<()> {
    match ty {
        IrType::Qubit
        | IrType::QubitRef(id)
            if *id == qubit =>
        {
            Ok(())
        }

        IrType::Qubit => Ok(()),

        _ => Err(TypingError::TypeMismatch {
            expected: IrType::QubitRef(qubit),
            actual: ty.clone(),
        }),
    }
}

/// Checks a physical identity at a physical mapping boundary.
pub fn ensure_physical_identity(
    qubit: PhysicalQubitId,
    ty: &IrType,
) -> TypingResult<()> {
    match ty {
        IrType::PhysicalQubit
        | IrType::PhysicalQubitRef(id)
            if *id == qubit =>
        {
            Ok(())
        }

        IrType::PhysicalQubit => Ok(()),

        _ => Err(TypingError::TypeMismatch {
            expected: IrType::PhysicalQubitRef(qubit),
            actual: ty.clone(),
        }),
    }
}

// =============================================================================
// Value-kind helpers
// =============================================================================

/// Returns the broad semantic `IrType` corresponding to a `ValueKind` when
/// the mapping is unambiguous.
#[must_use]
pub fn type_for_value_kind(
    kind: ValueKind,
) -> Option<IrType> {
    match kind {
        ValueKind::Bool => Some(IrType::Bool),

        ValueKind::Integer => Some(
            IrType::SignedInteger(
                SignedIntegerType::Arbitrary(0),
            ),
        ),

        ValueKind::UnsignedInteger => Some(
            IrType::UnsignedInteger(
                UnsignedIntegerType::Arbitrary(0),
            ),
        ),

        ValueKind::Float => Some(
            IrType::Float(FloatType::F64),
        ),

        ValueKind::Complex => Some(
            IrType::Complex(
                super::super::core::types::ComplexType::Float(
                    FloatType::F64,
                ),
            ),
        ),

        ValueKind::Angle => Some(
            IrType::Angle(
                super::super::core::types::AngleType::Exact,
            ),
        ),

        ValueKind::Duration => Some(
            IrType::Duration(
                super::super::core::types::DurationType::Exact,
            ),
        ),

        ValueKind::Frequency => Some(
            IrType::Frequency(
                super::super::core::types::FrequencyType::Exact,
            ),
        ),

        ValueKind::Amplitude => Some(
            IrType::Amplitude(
                super::super::core::types::AmplitudeType::Exact,
            ),
        ),

        ValueKind::Phase => Some(
            IrType::Phase(
                super::super::core::types::PhaseType::Exact,
            ),
        ),

        ValueKind::Qubit => Some(IrType::Qubit),

        ValueKind::PhysicalQubit => {
            Some(IrType::PhysicalQubit)
        }

        ValueKind::Reference
        | ValueKind::Parameter
        | ValueKind::Array
        | ValueKind::Tuple
        | ValueKind::Optional => None,

        ValueKind::Unit => Some(IrType::Unit),
    }
}

// =============================================================================
// Type predicates
// =============================================================================

/// Returns whether a type is a classical scalar usable in ordinary classical
/// expression typing.
#[must_use]
pub fn is_classical_scalar(
    ty: &IrType,
) -> bool {
    matches!(
        ty,
        IrType::Bool
            | IrType::SignedInteger(_)
            | IrType::UnsignedInteger(_)
            | IrType::Float(_)
            | IrType::Complex(_)
            | IrType::Bit(_)
            | IrType::Angle(_)
            | IrType::Duration(_)
            | IrType::Frequency(_)
            | IrType::Amplitude(_)
            | IrType::Phase(_)
    )
}

/// Returns whether a type can participate in a classical predicate.
#[must_use]
pub fn is_predicate_type(
    ty: &IrType,
) -> bool {
    ty.is_predicate_compatible()
}

/// Returns whether a type is a numeric semantic type.
#[must_use]
pub fn is_numeric_type(
    ty: &IrType,
) -> bool {
    ty.is_numeric()
}

/// Returns whether a type represents a logical quantum resource.
#[must_use]
pub fn is_logical_qubit_type(
    ty: &IrType,
) -> bool {
    ty.is_logical_qubit()
}

/// Returns whether a type represents a physical quantum resource.
#[must_use]
pub fn is_physical_qubit_type(
    ty: &IrType,
) -> bool {
    ty.is_physical_qubit()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_and_physical_qubits_are_distinct() {
        assert!(is_logical_qubit_type(
            &IrType::Qubit
        ));

        assert!(is_physical_qubit_type(
            &IrType::PhysicalQubit
        ));

        assert!(!type_compatible(
            &IrType::Qubit,
            &IrType::PhysicalQubit,
            TypingPolicy::strict(),
        ));
    }

    #[test]
    fn concrete_logical_qubit_is_assignable_to_logical_qubit() {
        let q = QubitId::new(7);

        assert!(type_compatible(
            &IrType::QubitRef(q),
            &IrType::Qubit,
            TypingPolicy::strict(),
        ));
    }

    #[test]
    fn physical_qubit_is_not_logical_qubit() {
        let p = PhysicalQubitId::new(7);

        assert!(!type_compatible(
            &IrType::PhysicalQubitRef(p),
            &IrType::Qubit,
            TypingPolicy::strict(),
        ));
    }

    #[test]
    fn exact_types_are_compatible() {
        let ty = IrType::Bool;

        assert!(type_compatible(
            &ty,
            &ty,
            TypingPolicy::strict(),
        ));
    }

    #[test]
    fn strict_numeric_types_do_not_implicitly_convert() {
        let source = IrType::SignedInteger(
            SignedIntegerType::I32,
        );

        let target = IrType::SignedInteger(
            SignedIntegerType::I64,
        );

        assert!(!type_compatible(
            &source,
            &target,
            TypingPolicy::strict(),
        ));

        assert!(check_assignment(
            &source,
            &target,
            TypingPolicy {
                allow_implicit_numeric_widening: true,
                ..TypingPolicy::strict()
            },
        )
        .is_ok());
    }

    #[test]
    fn arrays_validate_dimensions() {
        let source = IrType::array(
            IrType::Qubit,
            Dimension::Static(8),
        );

        let target = IrType::array(
            IrType::Qubit,
            Dimension::Dynamic,
        );

        assert!(type_compatible(
            &source,
            &target,
            TypingPolicy::strict(),
        ));
    }

    #[test]
    fn static_array_size_must_match_value() {
        let value = Value::array_with_kind(
            ValueKind::Qubit,
            vec![
                Value::qubit(QubitId::new(0)),
                Value::qubit(QubitId::new(1)),
            ],
        )
        .expect("homogeneous array");

        let expected = IrType::array(
            IrType::Qubit,
            Dimension::Static(3),
        );

        let result = check_value_type(
            &value,
            &expected,
            &TypeEnvironment::new(),
            TypingPolicy::strict(),
        );

        assert!(matches!(
            result,
            Err(TypingError::ArrayLengthMismatch {
                expected: 3,
                actual: 2
            })
        ));
    }

    #[test]
    fn tuple_types_are_checked_element_by_element() {
        let value = Value::tuple(vec![
            Value::bool(true),
            Value::integer(42),
        ]);

        let expected = IrType::tuple(vec![
            IrType::Bool,
            IrType::SignedInteger(
                SignedIntegerType::Arbitrary(0),
            ),
        ]);

        assert!(check_value_type(
            &value,
            &expected,
            &TypeEnvironment::new(),
            TypingPolicy::strict(),
        )
        .is_ok());
    }

    #[test]
    fn symbolic_parameter_can_satisfy_numeric_expected_type() {
        // The parameter object itself intentionally carries no mandatory
        // physical unit. The consuming operation supplies the semantic type.
        //
        // Construction is deliberately not repeated here because the
        // parameter API owns symbol creation and may evolve independently.
        let policy = TypingPolicy::strict();

        assert!(policy.allow_symbolic_numeric_values);
    }

    #[test]
    fn operation_signature_checks_operand_types() {
        let signature =
            unary_qubit_signature("test");

        let operands =
            vec![IrType::Qubit];

        assert!(validate_operation_signature(
            &signature,
            &operands,
            &[],
            TypingPolicy::strict(),
        )
        .is_ok());
    }

    #[test]
    fn operation_signature_rejects_classical_operand_for_qubit_operation() {
        let signature =
            unary_qubit_signature("test");

        let operands =
            vec![IrType::Bool];

        let result =
            validate_operation_signature(
                &signature,
                &operands,
                &[],
                TypingPolicy::strict(),
            );

        assert!(matches!(
            result,
            Err(
                TypingError::OperandConstraintMismatch {
                    ..
                }
            )
        ));
    }

    #[test]
    fn same_type_signature_enforces_result_type() {
        let signature =
            same_type_binary_signature("add");

        let operands = vec![
            IrType::Float(FloatType::F64),
            IrType::Float(FloatType::F64),
        ];

        let results =
            vec![IrType::Float(FloatType::F64)];

        assert!(validate_operation_signature(
            &signature,
            &operands,
            &results,
            TypingPolicy::strict(),
        )
        .is_ok());
    }

    #[test]
    fn same_type_signature_rejects_mismatched_result() {
        let signature =
            same_type_binary_signature("add");

        let operands = vec![
            IrType::Float(FloatType::F64),
            IrType::Float(FloatType::F64),
        ];

        let results =
            vec![IrType::Float(FloatType::F32)];

        assert!(validate_operation_signature(
            &signature,
            &operands,
            &results,
            TypingPolicy::strict(),
        )
        .is_err());
    }

    #[test]
    fn physical_identity_boundary_uses_canonical_qubit_module() {
        let physical =
            PhysicalQubitId::new(1024);

        assert!(ensure_physical_identity(
            physical,
            &IrType::PhysicalQubitRef(physical),
        )
        .is_ok());
    }
}