
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Types
//!
//! Canonical, target-independent types produced by semantic analysis.
//!
//! ## Architectural contract
//!
//! PURPOSE:
//! Represent resolved type meaning independently of source syntax, hardware,
//! execution environment, and backend implementation.
//!
//! STATUS:
//! Proposed new file. Freeze only after repository integration and tests.
//!
//! AUTHORITY:
//! The normative Zamani type-system and semantic-model specifications.
//! Source syntax is defined by the canonical grammar and AST.
//!
//! OWNS:
//! - Resolved semantic type structures.
//! - Target-neutral numeric-width descriptions.
//! - Symbolic type-level cardinalities.
//! - Generic substitutions as type structures.
//! - Explicit unresolved/error type states.
//! - Structural validation and canonical type formatting.
//!
//! DOES_NOT_OWN:
//! - Source parsing or AST construction.
//! - Name or import resolution.
//! - Type inference or unification algorithms.
//! - Declaration identity allocation.
//! - Trait implementation lookup.
//! - Effect inference or authorization.
//! - Capability or resource feasibility.
//! - Quantum gate registries or physical qubits.
//! - Hardware layout, ABI, scheduling, routing, or code generation.
//!
//! DEPENDENCIES:
//! Rust standard library and the repository's existing `serde` dependency.
//!
//! INTEGRATION:
//! `types/inference.rs` constructs and infers these types.
//! `types/unification.rs` solves type-variable constraints.
//! `types/compatibility.rs` implements language-defined compatibility.
//! `types/generics.rs` performs generic substitution and instantiation.
//! `types/aliases.rs` resolves aliases and detects cycles.
//! `types/constraints.rs` handles type-level constraints.
//! `model/facts.rs` associates resolved types with AST identities.
//! `model/program.rs` exposes finalized semantic results.
//! Domain validators consume these types without redefining core type meaning.
//!
//! The AST's `TypeExpr` remains the authoritative source-syntax representation.
//! Conversion from `TypeExpr` into `SemanticType` belongs to the resolver,
//! because resolving names requires a declaration environment.
//!
//! SCALABILITY:
//! No hardware capacities, finite domain lists, fixed generic arity, or fixed
//! tensor-rank limits are imposed here. Arbitrary-precision integer widths and
//! cardinalities can be retained symbolically.
//!
//! Compiler safety budgets belong to configuration and analysis sessions.
//! An operational budget must never silently change type meaning.
//!
//! DETERMINISM:
//! Ordered collections use `Vec`; this module does not depend on hash-map
//! iteration order. Callers must canonicalize semantically unordered
//! collections where canonical ordering is required.
//!
//! ERROR HANDLING:
//! Malformed type structures return validation errors. `Error` is an explicit
//! recovery type, not a valid wildcard and not proof of semantic correctness.
//!
//! COMPATIBILITY:
//! Rust 2021, declared minimum Rust 1.97, stable features only.
//! No unsafe code.
//!
//! FREEZE CRITERIA:
//! - The complete AST-to-semantic type mapping is implemented by the resolver.
//! - Type variables have checked allocation and explicit finalization.
//! - Compatibility and unification tests cover all variants.
//! - Type aliases and recursive types have cycle policies.
//! - Semantic model and domain validators consume this representation.
//! - Workspace tests and lint checks pass.
//!
//! This module intentionally has no dependency on the legacy `crate::ast`,
//! `crate::semantic`, IR implementations, or target/backend modules.

use core::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

/// Schema version for serialized semantic type structures.
///
/// This version is independent of the language version, AST schema, compiler
/// release, and serialized semantic-program schema.
pub const SEMANTIC_TYPE_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Shared immutable names
// =============================================================================

/// Immutable, shareable semantic name.
///
/// Names retain their spelling. Name resolution and language-specific
/// normalization are performed by the symbol-resolution subsystem.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SemanticName(Arc<str>);

impl SemanticName {
    /// Creates a name without asserting that it is semantically valid.
    ///
    /// Keeping construction permissive lets parser recovery and deserialization
    /// preserve malformed input for diagnostics. Use [`Self::validate`] before
    /// accepting externally supplied names into a finalized semantic model.
    #[must_use]
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(Arc::<str>::from(value.as_ref()))
    }

    /// Returns the name's text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    /// Returns whether the name is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the UTF-8 byte length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Validates the name's basic structural requirements.
    pub fn validate(&self) -> Result<(), TypeValidationError> {
        if self.is_empty() {
            return Err(TypeValidationError::EmptyName);
        }

        if self.as_str().contains('\0') {
            return Err(TypeValidationError::NulInName);
        }

        Ok(())
    }
}

impl From<&str> for SemanticName {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for SemanticName {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for SemanticName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Qualified semantic name.
///
/// The segments are ordered and preserve qualification. The path itself does
/// not prove that the referenced declaration exists.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QualifiedName {
    segments: Vec<SemanticName>,
}

impl QualifiedName {
    /// Creates a qualified name from ordered segments.
    #[must_use]
    pub fn new(segments: Vec<SemanticName>) -> Self {
        Self { segments }
    }

    /// Creates a single-segment name.
    #[must_use]
    pub fn single(name: impl Into<SemanticName>) -> Self {
        Self {
            segments: vec![name.into()],
        }
    }

    /// Creates a qualified name from an iterator.
    #[must_use]
    pub fn from_segments<I, S>(segments: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<SemanticName>,
    {
        Self {
            segments: segments.into_iter().map(Into::into).collect(),
        }
    }

    /// Returns ordered name segments.
    #[must_use]
    pub fn segments(&self) -> &[SemanticName] {
        &self.segments
    }

    /// Returns the number of segments.
    #[must_use]
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Returns whether the path has no segments.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Returns the last segment, if any.
    #[must_use]
    pub fn last(&self) -> Option<&SemanticName> {
        self.segments.last()
    }

    /// Returns the canonical textual representation.
    #[must_use]
    pub fn to_qualified_string(&self) -> String {
        self.segments
            .iter()
            .map(SemanticName::as_str)
            .collect::<Vec<_>>()
            .join("::")
    }

    /// Validates the path and every segment.
    pub fn validate(&self) -> Result<(), TypeValidationError> {
        if self.segments.is_empty() {
            return Err(TypeValidationError::EmptyQualifiedName);
        }

        for segment in &self.segments {
            segment.validate()?;
        }

        Ok(())
    }
}

impl fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, segment) in self.segments.iter().enumerate() {
            if index != 0 {
                f.write_str("::")?;
            }
            write!(f, "{segment}")?;
        }

        Ok(())
    }
}

// =============================================================================
// Numeric representations without machine-capacity assumptions
// =============================================================================

/// Signedness of an integer semantic type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntegerSignedness {
    Signed,
    Unsigned,
}

/// An integer width independent of the host machine's word size.
///
/// `Bits` stores a positive decimal integer as text so widths beyond the host's
/// `usize` can be represented without truncation. This does not promise that
/// every backend can implement every width.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntegerWidth {
    /// A statically specified positive number of bits, encoded in base-10.
    Bits(Arc<str>),

    /// The language-defined target-sized integer.
    ///
    /// Its concrete width is selected by the target data model, not this file.
    TargetSized,

    /// An integer whose precision is selected by an implementation or policy.
    ArbitraryPrecision,

    /// A symbolic width that must be resolved by a later semantic constraint.
    Symbolic(Arc<str>),
}

impl IntegerWidth {
    /// Constructs and validates a positive decimal bit width.
    ///
    /// Leading zeroes are accepted and normalized away. Zero and non-decimal
    /// values are rejected. No machine-sized integer conversion is performed.
    pub fn bits(decimal: impl AsRef<str>) -> Result<Self, TypeValidationError> {
        let input = decimal.as_ref();

        if input.is_empty() || !input.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(TypeValidationError::InvalidIntegerWidth);
        }

        let normalized = input.trim_start_matches('0');

        if normalized.is_empty() {
            return Err(TypeValidationError::InvalidIntegerWidth);
        }

        Ok(Self::Bits(Arc::<str>::from(normalized)))
    }

    /// Constructs a symbolic width.
    pub fn symbolic(expression: impl AsRef<str>) -> Result<Self, TypeValidationError> {
        let expression = expression.as_ref();

        if expression.trim().is_empty() {
            return Err(TypeValidationError::EmptySymbolicExpression);
        }

        Ok(Self::Symbolic(Arc::<str>::from(expression)))
    }

    /// Returns a fixed decimal width, if this is a fixed-width type.
    #[must_use]
    pub fn fixed_decimal(&self) -> Option<&str> {
        match self {
            Self::Bits(value) => Some(value.as_ref()),
            Self::TargetSized | Self::ArbitraryPrecision | Self::Symbolic(_) => None,
        }
    }

    /// Validates a width, including values created by deserialization.
    pub fn validate(&self) -> Result<(), TypeValidationError> {
        match self {
            Self::Bits(value) => {
                if value.is_empty()
                    || !value.bytes().all(|byte| byte.is_ascii_digit())
                    || value.starts_with('0')
                {
                    return Err(TypeValidationError::InvalidIntegerWidth);
                }
                Ok(())
            }
            Self::TargetSized | Self::ArbitraryPrecision => Ok(()),
            Self::Symbolic(expression) => {
                if expression.trim().is_empty() {
                    Err(TypeValidationError::EmptySymbolicExpression)
                } else {
                    Ok(())
                }
            }
        }
    }
}

/// Semantic floating-point format.
///
/// A format is a language-level precision request, not a guarantee that every
/// target natively supports that format.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FloatFormat {
    /// Standardized binary floating-point format.
    Binary16,
    BFloat16,
    Binary32,
    Binary64,
    Binary128,

    /// Decimal floating-point format with a symbolic or specified precision.
    Decimal {
        precision: Option<Arc<str>>,
    },

    /// Extensible or implementation-defined floating-point format.
    Extension {
        name: QualifiedName,
        parameters: Vec<SemanticType>,
    },
}

/// Primitive semantic types.
///
/// Named user-defined and domain-specific types are represented separately.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrimitiveType {
    Unit,
    Boolean,
    Character,
    String,
    StringSlice,
    Integer {
        signedness: IntegerSignedness,
        width: IntegerWidth,
    },
    Float(FloatFormat),
    Complex(Box<SemanticType>),
}

// =============================================================================
// Symbolic values and cardinalities
// =============================================================================

/// Symbolic value used in dimensions, lengths, and dependent type arguments.
///
/// Integer values are decimal strings rather than machine integers. Arithmetic
/// and evaluation belong to the checked constant-evaluation subsystem.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeLevelValue {
    /// A non-negative integer represented in base 10.
    Integer(Arc<str>),

    /// A resolved or unresolved symbolic name.
    Name(QualifiedName),

    /// A generic value parameter.
    Parameter(SemanticName),

    /// A structured expression retained for a later constraint solver.
    Expression {
        operator: SemanticName,
        operands: Vec<TypeLevelValue>,
    },

    /// A domain extension's type-level value.
    Extension {
        namespace: QualifiedName,
        name: SemanticName,
        arguments: Vec<TypeLevelValue>,
    },
}

impl TypeLevelValue {
    /// Constructs a non-negative decimal integer without narrowing it.
    pub fn integer(decimal: impl AsRef<str>) -> Result<Self, TypeValidationError> {
        let input = decimal.as_ref();

        if input.is_empty() || !input.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(TypeValidationError::InvalidCardinality);
        }

        let normalized = input.trim_start_matches('0');
        let normalized = if normalized.is_empty() { "0" } else { normalized };

        Ok(Self::Integer(Arc::<str>::from(normalized)))
    }

    /// Validates the structure, including deserialized values.
    pub fn validate(&self) -> Result<(), TypeValidationError> {
        match self {
            Self::Integer(value) => {
                if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
                    return Err(TypeValidationError::InvalidCardinality);
                }
            }
            Self::Name(name) => name.validate()?,
            Self::Parameter(name) => name.validate()?,
            Self::Expression { operator, operands } => {
                operator.validate()?;
                for operand in operands {
                    operand.validate()?;
                }
            }
            Self::Extension {
                namespace,
                name,
                arguments,
            } => {
                namespace.validate()?;
                name.validate()?;
                for argument in arguments {
                    argument.validate()?;
                }
            }
        }

        Ok(())
    }
}

/// Cardinality of a semantic array, vector, tensor dimension, or resource-like
/// type parameter.
///
/// `Unspecified` means the source type does not establish a fixed cardinality;
/// it must not be treated as equal to every known length during compatibility
/// checking.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Cardinality {
    Known(TypeLevelValue),
    Symbolic(TypeLevelValue),
    Unspecified,
}

impl Cardinality {
    /// Validates a cardinality.
    pub fn validate(&self) -> Result<(), TypeValidationError> {
        match self {
            Self::Known(value) | Self::Symbolic(value) => value.validate(),
            Self::Unspecified => Ok(()),
        }
    }
}

// =============================================================================
// Type variables and generic parameters
// =============================================================================

/// Type-variable handle owned by an inference session.
///
/// The value is an identifier, not a hardware quantity. Its allocator must
/// detect exhaustion and must never silently wrap or reuse a live identity.
/// The semantic type itself does not allocate variables or define their scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TypeVariableId(pub u64);

/// Generic type parameter identity within a declaration.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeParameter {
    /// Source-level parameter name.
    pub name: SemanticName,

    /// Optional declaration-qualified identity assigned by symbol resolution.
    ///
    /// This is not a globally stable identifier unless the declaration system
    /// explicitly guarantees that property.
    pub declaration: Option<QualifiedName>,
}

// =============================================================================
// Semantic types
// =============================================================================

/// Canonical semantic type.
///
/// This is a description of type meaning, not physical representation.
///
/// Recursive children use `Box` or `Vec` to keep the enum finite-sized.
/// A compiler may intern or reference-count complete types in a separate
/// type store without changing this vocabulary.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SemanticType {
    /// A resolved primitive type.
    Primitive(PrimitiveType),

    /// A named type with optional nominal identity and ordered type arguments.
    ///
    /// `nominal_id` is supplied by the declaration system. The qualified name
    /// alone must not be used as nominal identity if the language permits
    /// shadowing, modules, versioned declarations, or distinct declarations
    /// with equal spellings.
    Named {
        name: QualifiedName,
        nominal_id: Option<QualifiedName>,
        arguments: Vec<SemanticType>,
    },

    /// A generic parameter reference.
    Parameter(TypeParameter),

    /// An inference variable.
    ///
    /// This variant is valid during analysis, but a finalized type must not
    /// contain unresolved variables unless the language explicitly permits
    /// them in that context.
    Variable(TypeVariableId),

    /// A tuple type whose component order is semantically significant.
    Tuple(Vec<SemanticType>),

    /// An array with an element type and semantic cardinality.
    Array {
        element: Box<SemanticType>,
        cardinality: Cardinality,
    },

    /// A dynamically sized sequence.
    Slice(Box<SemanticType>),

    /// A function type.
    ///
    /// Effects are represented by qualified semantic identities here to avoid
    /// coupling the type model to the effect-analysis implementation. The
    /// effect subsystem remains authoritative for effect meaning.
    Function {
        parameters: Vec<FunctionParameter>,
        result: Box<SemanticType>,
        effects: Vec<QualifiedName>,
        is_async: bool,
    },

    /// A reference type. Borrowing and lifetime validity are checked elsewhere.
    Reference {
        mutable: bool,
        lifetime: Option<SemanticName>,
        referent: Box<SemanticType>,
    },

    /// A pointer type. Pointer validity and unsafe-operation policies are
    /// semantic rules, not consequences of this representation.
    Pointer {
        mutable: bool,
        pointee: Box<SemanticType>,
    },

    /// Optional value.
    Optional(Box<SemanticType>),

    /// Result value with success and error types.
    Result {
        ok: Box<SemanticType>,
        error: Box<SemanticType>,
    },

    /// A union type. Canonicalization and subtype semantics are separate.
    Union(Vec<SemanticType>),

    /// An intersection type. Canonicalization and trait semantics are separate.
    Intersection(Vec<SemanticType>),

    /// The never type, representing computations that do not return normally.
    Never,

    /// The language's unit type.
    Unit,

    /// The contextual `Self` type before or during resolution.
    SelfType,

    /// Quantum value abstraction, independent of physical qubit allocation.
    Quantum(Box<SemanticType>),

    /// Linear resource semantics.
    Linear(Box<SemanticType>),

    /// Affine resource semantics.
    Affine(Box<SemanticType>),

    /// Temporal or lifetime-sensitive type abstraction.
    Temporal(Box<SemanticType>),

    /// Dependent function type.
    Pi {
        parameter: TypeParameter,
        parameter_type: Box<SemanticType>,
        body: Box<SemanticType>,
    },

    /// Dependent pair type.
    Sigma {
        parameter: TypeParameter,
        parameter_type: Box<SemanticType>,
        body: Box<SemanticType>,
    },

    /// Type equality proposition.
    Identity {
        left: Box<SemanticType>,
        right: Box<SemanticType>,
    },

    /// Associated type projection awaiting or carrying resolution.
    Associated {
        base: Box<SemanticType>,
        member: SemanticName,
    },

    /// Type-level constructor application.
    TypeApplication {
        constructor: Box<SemanticType>,
        arguments: Vec<SemanticType>,
    },

    /// Extensible domain type.
    ///
    /// Quantum, HDL, tensor, networking, distributed, cryptographic, and future
    /// domains may define types without adding a variant to this enum.
    Extension {
        namespace: QualifiedName,
        name: SemanticName,
        arguments: Vec<SemanticType>,
        attributes: Vec<TypeAttribute>,
    },

    /// Explicit error-recovery type.
    ///
    /// This prevents a failed type from being mistaken for a wildcard. It must
    /// be tracked as tainted by the semantic result and cannot certify success.
    Error,
}

/// A parameter in a semantic function type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionParameter {
    pub name: Option<SemanticName>,
    pub ty: SemanticType,
    pub mode: ParameterMode,
}

/// Function parameter passing semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParameterMode {
    Value,
    SharedReference,
    MutableReference,
    Move,
}

/// Namespaced attribute attached to an extensible semantic type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeAttribute {
    pub namespace: QualifiedName,
    pub name: SemanticName,
    pub value: Option<Arc<str>>,
}

// =============================================================================
// Constructors and inspection
// =============================================================================

impl SemanticType {
    /// Creates a named type.
    #[must_use]
    pub fn named(name: QualifiedName) -> Self {
        Self::Named {
            name,
            nominal_id: None,
            arguments: Vec::new(),
        }
    }

    /// Creates a parameterized named type.
    #[must_use]
    pub fn applied(
        name: QualifiedName,
        arguments: Vec<SemanticType>,
    ) -> Self {
        Self::Named {
            name,
            nominal_id: None,
            arguments,
        }
    }

    /// Creates an integer semantic type.
    #[must_use]
    pub fn integer(signedness: IntegerSignedness, width: IntegerWidth) -> Self {
        Self::Primitive(PrimitiveType::Integer { signedness, width })
    }

    /// Creates a symbolic array type.
    #[must_use]
    pub fn array(element: SemanticType, cardinality: Cardinality) -> Self {
        Self::Array {
            element: Box::new(element),
            cardinality,
        }
    }

    /// Creates a function type.
    #[must_use]
    pub fn function(
        parameters: Vec<FunctionParameter>,
        result: SemanticType,
        effects: Vec<QualifiedName>,
        is_async: bool,
    ) -> Self {
        Self::Function {
            parameters,
            result: Box::new(result),
            effects,
            is_async,
        }
    }

    /// Returns whether this is the explicit recovery type.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    /// Returns whether this type is or contains an unresolved inference
    /// variable, an unresolved contextual type, or an error.
    ///
    /// This is a structural check, not a substitute for the finalization rules
    /// in `types/inference.rs` and `model/validity.rs`.
    #[must_use]
    pub fn is_incomplete(&self) -> bool {
        match self {
            Self::Error | Self::Variable(_) | Self::SelfType | Self::Associated { .. } => true,

            Self::Primitive(PrimitiveType::Complex(inner)) => inner.is_incomplete(),

            Self::Primitive(PrimitiveType::Float(FloatFormat::Extension {
                parameters, ..
            })) => parameters.iter().any(Self::is_incomplete),

            Self::Named { arguments, .. }
            | Self::Tuple(arguments)
            | Self::Union(arguments)
            | Self::Intersection(arguments)
            | Self::TypeApplication {
                arguments,
                constructor: _,
            } => arguments.iter().any(Self::is_incomplete)
                || matches!(self, Self::TypeApplication { constructor, .. } if constructor.is_incomplete()),

            Self::Array { element, .. }
            | Self::Slice(element)
            | Self::Optional(element)
            | Self::Quantum(element)
            | Self::Linear(element)
            | Self::Affine(element)
            | Self::Temporal(element) => element.is_incomplete(),

            Self::Reference { referent, .. } => referent.is_incomplete(),
            Self::Pointer { pointee, .. } => pointee.is_incomplete(),

            Self::Result { ok, error } => ok.is_incomplete() || error.is_incomplete(),

            Self::Function {
                parameters, result, ..
            } => {
                parameters.iter().any(|parameter| parameter.ty.is_incomplete())
                    || result.is_incomplete()
            }

            Self::Pi {
                parameter_type,
                body,
                ..
            }
            | Self::Sigma {
                parameter_type,
                body,
                ..
            } => parameter_type.is_incomplete() || body.is_incomplete(),

            Self::Identity { left, right } => left.is_incomplete() || right.is_incomplete(),

            Self::Extension { arguments, .. } => arguments.iter().any(Self::is_incomplete),

            Self::Primitive(_) | Self::Parameter(_) | Self::Never | Self::Unit => false,
        }
    }

    /// Validates structural well-formedness under an explicit traversal policy.
    ///
    /// This does not perform name resolution, type compatibility, unification,
    /// trait resolution, or domain-specific semantic validation.
    pub fn validate(
        &self,
        policy: &TypeValidationPolicy,
    ) -> Result<(), TypeValidationError> {
        self.validate_at(policy, 0)
    }

    fn validate_at(
        &self,
        policy: &TypeValidationPolicy,
        depth: usize,
    ) -> Result<(), TypeValidationError> {
        if let Some(max_depth) = policy.max_depth {
            if depth > max_depth {
                return Err(TypeValidationError::TraversalBudgetExceeded {
                    limit: max_depth,
                });
            }
        }

        let next = depth.saturating_add(1);

        match self {
            Self::Primitive(PrimitiveType::Integer { width, .. }) => width.validate()?,

            Self::Primitive(PrimitiveType::Float(FloatFormat::Decimal {
                precision: Some(precision),
            })) => {
                if precision.trim().is_empty() {
                    return Err(TypeValidationError::EmptySymbolicExpression);
                }
            }

            Self::Primitive(PrimitiveType::Float(FloatFormat::Extension {
                name,
                parameters,
            })) => {
                name.validate()?;
                for parameter in parameters {
                    parameter.validate_at(policy, next)?;
                }
            }

            Self::Primitive(PrimitiveType::Complex(inner)) => {
                inner.validate_at(policy, next)?;
            }

            Self::Named {
                name,
                nominal_id,
                arguments,
            } => {
                name.validate()?;
                if let Some(identity) = nominal_id {
                    identity.validate()?;
                }
                for argument in arguments {
                    argument.validate_at(policy, next)?;
                }
            }

            Self::Parameter(parameter) => {
                parameter.name.validate()?;
                if let Some(declaration) = &parameter.declaration {
                    declaration.validate()?;
                }
            }

            Self::Variable(_) | Self::Never | Self::Unit | Self::SelfType | Self::Error => {}

            Self::Tuple(elements)
            | Self::Union(elements)
            | Self::Intersection(elements) => {
                for element in elements {
                    element.validate_at(policy, next)?;
                }
            }

            Self::Array {
                element,
                cardinality,
            } => {
                element.validate_at(policy, next)?;
                cardinality.validate()?;
            }

            Self::Slice(inner)
            | Self::Optional(inner)
            | Self::Quantum(inner)
            | Self::Linear(inner)
            | Self::Affine(inner)
            | Self::Temporal(inner) => inner.validate_at(policy, next)?,

            Self::Reference {
                lifetime, referent, ..
            } => {
                if let Some(lifetime) = lifetime {
                    lifetime.validate()?;
                }
                referent.validate_at(policy, next)?;
            }

            Self::Pointer { pointee, .. } => pointee.validate_at(policy, next)?,

            Self::Result { ok, error } => {
                ok.validate_at(policy, next)?;
                error.validate_at(policy, next)?;
            }

            Self::Function {
                parameters,
                result,
                effects,
                ..
            } => {
                for parameter in parameters {
                    if let Some(name) = &parameter.name {
                        name.validate()?;
                    }
                    parameter.ty.validate_at(policy, next)?;
                }
                result.validate_at(policy, next)?;
                for effect in effects {
                    effect.validate()?;
                }
            }

            Self::Pi {
                parameter,
                parameter_type,
                body,
            }
            | Self::Sigma {
                parameter,
                parameter_type,
                body,
            } => {
                parameter.name.validate()?;
                if let Some(declaration) = &parameter.declaration {
                    declaration.validate()?;
                }
                parameter_type.validate_at(policy, next)?;
                body.validate_at(policy, next)?;
            }

            Self::Identity { left, right } => {
                left.validate_at(policy, next)?;
                right.validate_at(policy, next)?;
            }

            Self::Associated { base, member } => {
                base.validate_at(policy, next)?;
                member.validate()?;
            }

            Self::TypeApplication {
                constructor,
                arguments,
            } => {
                constructor.validate_at(policy, next)?;
                for argument in arguments {
                    argument.validate_at(policy, next)?;
                }
            }

            Self::Extension {
                namespace,
                name,
                arguments,
                attributes,
            } => {
                namespace.validate()?;
                name.validate()?;

                for argument in arguments {
                    argument.validate_at(policy, next)?;
                }

                for attribute in attributes {
                    attribute.namespace.validate()?;
                    attribute.name.validate()?;
                }
            }
        }

        Ok(())
    }

    /// Formats a deterministic, target-independent type description.
    ///
    /// This format is intended for diagnostics and debugging. It is not a
    /// stable serialization format and must not be used as type identity.
    #[must_use]
    pub fn diagnostic_name(&self) -> String {
        self.to_string()
    }
}

// =============================================================================
// Validation policy and errors
// =============================================================================

/// Explicit operational policy for structural type validation.
///
/// `max_depth = None` imposes no policy-level depth limit. A caller processing
/// untrusted input should generally supply a finite budget. Such a budget is
/// an operational safety setting, not a Zamani language type restriction.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TypeValidationPolicy {
    pub max_depth: Option<usize>,
}

/// Structural validation failures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeValidationError {
    EmptyName,
    NulInName,
    EmptyQualifiedName,
    InvalidIntegerWidth,
    InvalidCardinality,
    EmptySymbolicExpression,
    TraversalBudgetExceeded { limit: usize },
}

impl fmt::Display for TypeValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => f.write_str("semantic name must not be empty"),
            Self::NulInName => f.write_str("semantic name must not contain NUL"),
            Self::EmptyQualifiedName => {
                f.write_str("qualified semantic name must contain at least one segment")
            }
            Self::InvalidIntegerWidth => {
                f.write_str("integer width must be a positive decimal integer")
            }
            Self::InvalidCardinality => {
                f.write_str("cardinality must be a non-negative decimal integer")
            }
            Self::EmptySymbolicExpression => {
                f.write_str("symbolic expression must not be empty")
            }
            Self::TraversalBudgetExceeded { limit } => {
                write!(f, "type validation traversal budget exceeded (limit {limit})")
            }
        }
    }
}

impl std::error::Error for TypeValidationError {}

// =============================================================================
// Deterministic diagnostic formatting
// =============================================================================

impl fmt::Display for IntegerWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bits(bits) => write!(f, "bits<{bits}>"),
            Self::TargetSized => f.write_str("target-sized"),
            Self::ArbitraryPrecision => f.write_str("arbitrary-precision"),
            Self::Symbolic(expression) => write!(f, "bits<{expression}>"),
        }
    }
}

impl fmt::Display for TypeLevelValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(value) => f.write_str(value),
            Self::Name(name) => write!(f, "{name}"),
            Self::Parameter(name) => write!(f, "{name}"),
            Self::Expression { operator, operands } => {
                write!(f, "{operator}(")?;
                for (index, operand) in operands.iter().enumerate() {
                    if index != 0 {
                        f.write_str(", ")?;
                    }
                    write!(f, "{operand}")?;
                }
                f.write_str(")")
            }
            Self::Extension {
                namespace,
                name,
                arguments,
            } => {
                write!(f, "{namespace}::{name}(")?;
                for (index, argument) in arguments.iter().enumerate() {
                    if index != 0 {
                        f.write_str(", ")?;
                    }
                    write!(f, "{argument}")?;
                }
                f.write_str(")")
            }
        }
    }
}

impl fmt::Display for Cardinality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Known(value) => write!(f, "{value}"),
            Self::Symbolic(value) => write!(f, "symbolic({value})"),
            Self::Unspecified => f.write_str("_"),
        }
    }
}

impl fmt::Display for SemanticType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Primitive(primitive) => match primitive {
                PrimitiveType::Unit => f.write_str("unit"),
                PrimitiveType::Boolean => f.write_str("bool"),
                PrimitiveType::Character => f.write_str("char"),
                PrimitiveType::String => f.write_str("String"),
                PrimitiveType::StringSlice => f.write_str("str"),
                PrimitiveType::Integer { signedness, width } => {
                    match signedness {
                        IntegerSignedness::Signed => f.write_str("signed<")?,
                        IntegerSignedness::Unsigned => f.write_str("unsigned<")?,
                    }
                    write!(f, "{width}>")
                }
                PrimitiveType::Float(format) => match format {
                    FloatFormat::Binary16 => f.write_str("binary16"),
                    FloatFormat::BFloat16 => f.write_str("bfloat16"),
                    FloatFormat::Binary32 => f.write_str("binary32"),
                    FloatFormat::Binary64 => f.write_str("binary64"),
                    FloatFormat::Binary128 => f.write_str("binary128"),
                    FloatFormat::Decimal { precision } => {
                        if let Some(precision) = precision {
                            write!(f, "decimal<{precision}>")
                        } else {
                            f.write_str("decimal")
                        }
                    }
                    FloatFormat::Extension { name, parameters } => {
                        write!(f, "{name}")?;
                        fmt_type_arguments(f, parameters)
                    }
                },
                PrimitiveType::Complex(inner) => write!(f, "Complex<{inner}>"),
            },

            Self::Named {
                name, arguments, ..
            } => {
                write!(f, "{name}")?;
                fmt_type_arguments(f, arguments)
            }

            Self::Parameter(parameter) => write!(f, "{}", parameter.name),
            Self::Variable(id) => write!(f, "?{}", id.0),

            Self::Tuple(elements) => {
                f.write_str("(")?;
                fmt_type_list(f, elements)?;
                if elements.len() == 1 {
                    f.write_str(",")?;
                }
                f.write_str(")")
            }

            Self::Array {
                element,
                cardinality,
            } => write!(f, "Array<{element}, {cardinality}>"),

            Self::Slice(element) => write!(f, "Slice<{element}>"),

            Self::Function {
                parameters,
                result,
                effects,
                is_async,
            } => {
                if *is_async {
                    f.write_str("async ")?;
                }
                f.write_str("fn(")?;
                for (index, parameter) in parameters.iter().enumerate() {
                    if index != 0 {
                        f.write_str(", ")?;
                    }
                    write!(f, "{}{}", parameter.mode, parameter.ty)?;
                }
                write!(f, ") -> {result}")?;
                if !effects.is_empty() {
                    f.write_str(" !{")?;
                    fmt_qualified_names(f, effects)?;
                    f.write_str("}")?;
                }
                Ok(())
            }

            Self::Reference {
                mutable,
                lifetime,
                referent,
            } => {
                f.write_str("&")?;
                if let Some(lifetime) = lifetime {
                    write!(f, "'{lifetime} ")?;
                }
                if *mutable {
                    f.write_str("mut ")?;
                }
                write!(f, "{referent}")
            }

            Self::Pointer { mutable, pointee } => {
                if *mutable {
                    write!(f, "*mut {pointee}")
                } else {
                    write!(f, "*const {pointee}")
                }
            }

            Self::Optional(inner) => write!(f, "Optional<{inner}>"),
            Self::Result { ok, error } => write!(f, "Result<{ok}, {error}>"),

            Self::Union(types) => {
                f.write_str("Union<")?;
                fmt_type_list(f, types)?;
                f.write_str(">")
            }

            Self::Intersection(types) => {
                f.write_str("Intersection<")?;
                fmt_type_list(f, types)?;
                f.write_str(">")
            }

            Self::Never => f.write_str("never"),
            Self::Unit => f.write_str("unit"),
            Self::SelfType => f.write_str("Self"),
            Self::Quantum(inner) => write!(f, "Quantum<{inner}>"),
            Self::Linear(inner) => write!(f, "Linear<{inner}>"),
            Self::Affine(inner) => write!(f, "Affine<{inner}>"),
            Self::Temporal(inner) => write!(f, "Temporal<{inner}>"),

            Self::Pi {
                parameter,
                parameter_type,
                body,
            } => write!(
                f,
                "Pi<{}: {}, {}>",
                parameter.name, parameter_type, body
            ),

            Self::Sigma {
                parameter,
                parameter_type,
                body,
            } => write!(
                f,
                "Sigma<{}: {}, {}>",
                parameter.name, parameter_type, body
            ),

            Self::Identity { left, right } => write!(f, "Identity<{left}, {right}>"),
            Self::Associated { base, member } => write!(f, "{base}::{member}"),

            Self::TypeApplication {
                constructor,
                arguments,
            } => {
                write!(f, "{constructor}")?;
                fmt_type_arguments(f, arguments)
            }

            Self::Extension {
                namespace,
                name,
                arguments,
                ..
            } => {
                write!(f, "{namespace}::{name}")?;
                fmt_type_arguments(f, arguments)
            }

            Self::Error => f.write_str("<type-error>"),
        }
    }
}

impl fmt::Display for ParameterMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value => Ok(()),
            Self::SharedReference => f.write_str("&"),
            Self::MutableReference => f.write_str("&mut "),
            Self::Move => f.write_str("move "),
        }
    }
}

fn fmt_type_arguments(
    f: &mut fmt::Formatter<'_>,
    arguments: &[SemanticType],
) -> fmt::Result {
    if arguments.is_empty() {
        return Ok(());
    }

    f.write_str("<")?;
    fmt_type_list(f, arguments)?;
    f.write_str(">")
}

fn fmt_type_list(f: &mut fmt::Formatter<'_>, types: &[SemanticType]) -> fmt::Result {
    for (index, ty) in types.iter().enumerate() {
        if index != 0 {
            f.write_str(", ")?;
        }
        write!(f, "{ty}")?;
    }
    Ok(())
}

fn fmt_qualified_names(
    f: &mut fmt::Formatter<'_>,
    names: &[QualifiedName],
) -> fmt::Result {
    for (index, name) in names.iter().enumerate() {
        if index != 0 {
            f.write_str(", ")?;
        }
        write!(f, "{name}")?;
    }
    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn name(value: &str) -> QualifiedName {
        QualifiedName::single(value)
    }

    #[test]
    fn arbitrary_decimal_integer_width_does_not_narrow() {
        let width_text = "999999999999999999999999999999999999999999999999";
        let width = IntegerWidth::bits(width_text).expect("valid decimal width");

        assert_eq!(width.fixed_decimal(), Some(width_text));
        assert_eq!(
            IntegerWidth::bits("000128").expect("valid width"),
            IntegerWidth::Bits(Arc::<str>::from("128"))
        );
        assert!(IntegerWidth::bits("0").is_err());
        assert!(IntegerWidth::bits("-8").is_err());
        assert!(IntegerWidth::bits("8.5").is_err());
    }

    #[test]
    fn arbitrary_decimal_cardinality_does_not_narrow() {
        let value_text = "12345678901234567890123456789012345678901234567890";
        let value = TypeLevelValue::integer(value_text).expect("valid cardinality");

        assert_eq!(
            value,
            TypeLevelValue::Integer(Arc::<str>::from(value_text))
        );
        assert!(TypeLevelValue::integer("-1").is_err());
        assert!(TypeLevelValue::integer("1.5").is_err());
    }

    #[test]
    fn symbolic_cardinality_is_preserved() {
        let cardinality = Cardinality::Symbolic(TypeLevelValue::Name(name("required_qubits")));
        let ty = SemanticType::array(
            SemanticType::Quantum(Box::new(SemanticType::Unit)),
            cardinality,
        );

        assert!(ty.validate(&TypeValidationPolicy::default()).is_ok());
        assert_eq!(
            ty.to_string(),
            "Array<Quantum<unit>, symbolic(required_qubits)>"
        );
    }

    #[test]
    fn named_type_keeps_generic_arguments() {
        let ty = SemanticType::applied(
            name("collections::Vector"),
            vec![SemanticType::Primitive(PrimitiveType::Boolean)],
        );

        assert_eq!(ty.to_string(), "collections::Vector<bool>");
        assert!(!ty.is_incomplete());
    }

    #[test]
    fn error_is_not_a_wildcard_or_a_complete_type() {
        let ty = SemanticType::Error;

        assert!(ty.is_error());
        assert!(ty.is_incomplete());
        assert!(ty.validate(&TypeValidationPolicy::default()).is_ok());
    }

    #[test]
    fn unresolved_variable_is_incomplete() {
        let ty = SemanticType::Tuple(vec![
            SemanticType::Unit,
            SemanticType::Variable(TypeVariableId(7)),
        ]);

        assert!(ty.is_incomplete());
    }

    #[test]
    fn validation_rejects_empty_qualified_names() {
        let ty = SemanticType::named(QualifiedName::default());

        assert_eq!(
            ty.validate(&TypeValidationPolicy::default()),
            Err(TypeValidationError::EmptyQualifiedName)
        );
    }

    #[test]
    fn validation_budget_is_explicit() {
        let ty = SemanticType::Optional(Box::new(SemanticType::Optional(Box::new(
            SemanticType::Unit,
        ))));

        let policy = TypeValidationPolicy {
            max_depth: Some(1),
        };

        assert_eq!(
            ty.validate(&policy),
            Err(TypeValidationError::TraversalBudgetExceeded { limit: 1 })
        );
        assert!(ty.validate(&TypeValidationPolicy::default()).is_ok());
    }

    #[test]
    fn function_formatting_is_deterministic() {
        let ty = SemanticType::function(
            vec![FunctionParameter {
                name: Some(SemanticName::from("input")),
                ty: SemanticType::Primitive(PrimitiveType::Boolean),
                mode: ParameterMode::Value,
            }],
            SemanticType::Unit,
            vec![name("io.write"), name("audit.record")],
            true,
        );

        assert_eq!(
            ty.to_string(),
            "async fn(bool) -> unit !{io.write, audit.record}"
        );
    }

    #[test]
    fn target_sized_integer_does_not_use_host_width() {
        let ty = SemanticType::integer(
            IntegerSignedness::Unsigned,
            IntegerWidth::TargetSized,
        );

        assert_eq!(ty.to_string(), "unsigned<target-sized>");
    }

    #[test]
    fn extension_types_do_not_require_core_enum_changes() {
        let ty = SemanticType::Extension {
            namespace: name("future.tensor"),
            name: SemanticName::from("DistributedTensor"),
            arguments: vec![SemanticType::Primitive(PrimitiveType::Boolean)],
            attributes: Vec::new(),
        };

        assert_eq!(ty.to_string(), "future.tensor::DistributedTensor<bool>");
        assert!(ty.validate(&TypeValidationPolicy::default()).is_ok());
    }
}
