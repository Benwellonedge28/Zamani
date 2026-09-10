//! Zamani Frontend AST — Function Type Expressions
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
//! TypeExpr::Function
//!     │
//!     ▼
//! FunctionType  ← this module
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic function/type model
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
//! This module provides the function-type-specific API for the canonical
//! `TypeExpr::Function` representation owned by `type_expr.rs`.
//!
//! It is deliberately a typed façade rather than a second AST representation.
//!
//! The native Zamani AST describes source-level function type intent. It does
//! not determine:
//!
//! - calling conventions;
//! - ABI layout;
//! - stack layout;
//! - register allocation;
//! - CPU instructions;
//! - GPU kernels;
//! - QPU execution conventions;
//! - quantum control hardware;
//! - physical qubit allocation;
//! - scheduling;
//! - routing;
//! - calibration;
//! - pulse generation;
//! - QEC implementation;
//! - backend selection;
//! - vendor APIs;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - target-specific function lowering.
//!
//! Those concerns belong to later compiler layers.
//!
//! # Canonical representation
//!
//! `type_expr.rs` is the authoritative owner of function type structure:
//!
//! ```text
//! TypeExpr::Function(
//!     parameters,
//!     return_type,
//! )
//! ```
//!
//! This file must never introduce a competing structure that becomes another
//! authoritative representation.
//!
//! `FunctionType` exists to provide a strongly typed API for consumers that
//! already know they are dealing with a function type.
//!
//! # POCO-REAF
//!
//! Function types are fundamental to Zamani's
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! objective.
//!
//! A function signature expresses source-level computation without prescribing
//! the machine on which the computation eventually executes.
//!
//! Parameter arity is therefore represented by the canonical dynamically sized
//! collection. There is no:
//!
//! ```text
//! MAX_FUNCTION_PARAMETERS
//! MAX_ARGUMENTS
//! MAX_RETURN_VALUES
//! MAX_QUBITS
//! MAX_MACHINE_SIZE
//! ```
//!
//! in this module.
//!
//! A function can therefore participate in programs ranging from tiny
//! computations to very large generated programs, subject only to explicitly
//! configured compiler resource policies and the resources actually available.
//!
//! # Domain neutrality
//!
//! A function type may eventually describe:
//!
//! - classical computation;
//! - quantum computation;
//! - hybrid classical/quantum computation;
//! - distributed computation;
//! - accelerator computation;
//! - HDL-related computation;
//! - AI/ML computation;
//! - future computational domains.
//!
//! This module does not know which domain a function belongs to.
//!
//! Domain identity, capabilities, effects, resource requirements and semantic
//! legality are resolved by later compiler stages.
//!
//! # Quantum independence
//!
//! A quantum function can be represented using ordinary function-type
//! structure, for example conceptually:
//!
//! ```text
//! fn(QuantumResource) -> Result
//! ```
//!
//! without this module knowing whether the eventual implementation uses:
//!
//! - one qubit;
//! - many qubits;
//! - logical qubits;
//! - physical qubits;
//! - a simulator;
//! - a distributed QPU;
//! - a future quantum technology.
//!
//! Function types must not contain:
//!
//! - quantum gates;
//! - gate sets;
//! - qubit topology;
//! - routing information;
//! - scheduler state;
//! - calibration data;
//! - physical-device identifiers;
//! - QEC layouts;
//! - backend instructions.
//!
//! # Semantic boundary
//!
//! This module does NOT perform:
//!
//! - name resolution;
//! - overload resolution;
//! - generic substitution;
//! - type inference;
//! - trait resolution;
//! - effect checking;
//! - capability checking;
//! - ownership checking;
//! - borrow checking;
//! - resource allocation;
//! - ABI selection;
//! - calling-convention selection;
//! - hardware mapping.
//!
//! Those responsibilities belong to later phases.
//!
//! # Integration contract
//!
//! ## Parser
//!
//! The parser constructs the canonical `TypeExpr::Function` representation.
//!
//! The parser may subsequently construct a `FunctionType` façade when it needs
//! function-specific inspection.
//!
//! The parser must not depend on `FunctionType` for syntax recognition.
//!
//! ## Structural validation
//!
//! Validation is delegated to the canonical `TypeExpr` implementation.
//!
//! This guarantees that function types have one authoritative validation path.
//!
//! ## Semantic analysis
//!
//! Semantic analysis consumes the canonical `TypeExpr` and/or this façade and
//! resolves:
//!
//! - parameter types;
//! - return type;
//! - generic substitutions;
//! - effects;
//! - capabilities;
//! - resource semantics;
//! - callable semantics;
//! - overload information;
//! - calling conventions.
//!
//! None of those resolutions belong here.
//!
//! ## ZUIR
//!
//! ZUIR lowering consumes the semantic representation.
//!
//! `FunctionType` must not contain ZUIR nodes and must not perform ZUIR
//! lowering itself.
//!
//! ## Serialization
//!
//! Serialization remains conceptually owned by the canonical `TypeExpr`
//! representation. This façade does not define a second wire format.
//!
//! Consumers requiring serialization should serialize the underlying
//! `TypeExpr`.
//!
//! # Scalability
//!
//! This implementation deliberately:
//!
//! - stores no fixed parameter array;
//! - stores no machine-sized semantic arity limit;
//! - performs no recursive validation itself;
//! - delegates validation to `TypeExpr`;
//! - preserves source ordering;
//! - avoids target-specific storage;
//! - avoids global mutable state.
//!
//! The canonical parameter collection is exposed as a slice so callers can
//! inspect it without unnecessary allocation.
//!
//! # Determinism
//!
//! Function parameters retain source order.
//!
//! No unordered collection participates in function-type representation.
//!
//! Source formatting delegates to the canonical `TypeExpr::to_source_string`
//! implementation so that there is exactly one source-formatting authority.
//!
//! # Security
//!
//! This module:
//!
//! - performs no I/O;
//! - executes no source code;
//! - performs no pointer arithmetic;
//! - contains no unsafe code;
//! - contains no raw memory manipulation;
//! - introduces no global mutable state.
//!
//! Malformed function structures are rejected through fallible construction and
//! canonical structural validation.
//!
//! # Rust compatibility
//!
//! This file targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe Rust.
//!
//! # Forbidden dependencies
//!
//! This file must not depend on:
//!
//! - `crate::ast` legacy AST;
//! - lexer implementation;
//! - parser implementation;
//! - semantic implementation;
//! - compiler driver;
//! - ZUIR;
//! - quantum backends;
//! - hardware;
//! - runtime;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - vendor APIs.
//!
//! Its implementation dependency is the canonical sibling `type_expr` module.
//!
//! # Compatibility
//!
//! Existing code using:
//!
//! ```text
//! TypeExpr::Function
//! ```
//!
//! remains authoritative.
//!
//! `FunctionType` is additive and is intended to make function-specific APIs
//! explicit without forcing unrelated AST consumers to migrate immediately.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use super::type_expr::{TypeExpr, TypeExprError, TypeValidationPolicy};

// =============================================================================
// FunctionType
// =============================================================================

/// Typed façade over the canonical [`TypeExpr::Function`] representation.
///
/// # Invariant
///
/// A successfully constructed `FunctionType` always contains a
/// `TypeExpr::Function` value.
///
/// The façade does not duplicate parameter or return-type storage. The
/// canonical `TypeExpr` remains the single source of truth.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionType(TypeExpr);

impl FunctionType {
    /// Creates a function type from an ordered parameter list and return type.
    ///
    /// This constructor performs structural construction but does not validate
    /// the resulting type.
    ///
    /// This distinction is intentional: parser/recovery code may construct AST
    /// structures before the complete AST is validated.
    ///
    /// Use [`Self::try_new`] when immediate structural validation is required.
    #[must_use]
    pub fn new(parameters: Vec<TypeExpr>, return_type: TypeExpr) -> Self {
        Self(TypeExpr::function(parameters, return_type))
    }

    /// Creates and structurally validates a function type using the canonical
    /// default validation policy.
    ///
    /// The language representation contains no fixed function-parameter limit.
    pub fn try_new(
        parameters: Vec<TypeExpr>,
        return_type: TypeExpr,
    ) -> Result<Self, TypeExprError> {
        let function = Self::new(parameters, return_type);
        function.validate()?;
        Ok(function)
    }

    /// Creates and structurally validates a function type using an explicit
    /// compiler validation/resource policy.
    ///
    /// Resource limits are compilation-policy concerns rather than language
    /// semantics.
    pub fn try_new_with_policy(
        parameters: Vec<TypeExpr>,
        return_type: TypeExpr,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, TypeExprError> {
        let function = Self::new(parameters, return_type);
        function.validate_with_policy(policy)?;
        Ok(function)
    }

    /// Attempts to create a `FunctionType` from an existing type expression.
    ///
    /// Returns [`FunctionTypeError::NotFunction`] when the supplied expression
    /// is another type-expression variant.
    pub fn try_from_type_expr(value: TypeExpr) -> Result<Self, FunctionTypeError> {
        match value {
            TypeExpr::Function(_, _) => Ok(Self(value)),
            other => Err(FunctionTypeError::NotFunction {
                variant: other.variant_name(),
            }),
        }
    }

    /// Attempts to create a validated `FunctionType` from an existing type
    /// expression.
    ///
    /// This is useful at AST boundaries where a canonical function expression
    /// has already been parsed but needs function-specific inspection.
    pub fn try_from_type_expr_with_policy(
        value: TypeExpr,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, FunctionTypeError> {
        let function = Self::try_from_type_expr(value)?;

        function
            .validate_with_policy(policy)
            .map_err(FunctionTypeError::InvalidType)?;

        Ok(function)
    }

    /// Returns the canonical underlying [`TypeExpr`].
    ///
    /// This is the primary interoperability boundary with existing frontend
    /// AST code.
    #[must_use]
    pub fn as_type_expr(&self) -> &TypeExpr {
        &self.0
    }

    /// Consumes the façade and returns the canonical [`TypeExpr`].
    #[must_use]
    pub fn into_type_expr(self) -> TypeExpr {
        self.0
    }

    /// Returns the ordered parameter types.
    ///
    /// The returned slice is borrowed directly from the canonical
    /// `TypeExpr::Function` representation; no allocation occurs.
    #[must_use]
    pub fn parameters(&self) -> &[TypeExpr] {
        match &self.0 {
            TypeExpr::Function(parameters, _) => parameters,
            _ => unreachable!("FunctionType invariant violated"),
        }
    }

    /// Returns the function return type.
    ///
    /// The returned reference points directly into the canonical AST.
    #[must_use]
    pub fn return_type(&self) -> &TypeExpr {
        match &self.0 {
            TypeExpr::Function(_, return_type) => return_type.as_ref(),
            _ => unreachable!("FunctionType invariant violated"),
        }
    }

    /// Returns the number of function parameters.
    ///
    /// This is a source-level arity and is not a hardware-resource count.
    ///
    /// There is intentionally no language-level maximum.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.parameters().len()
    }

    /// Returns whether the function has zero parameters.
    ///
    /// A zero-parameter function is represented by an empty parameter vector.
    #[must_use]
    pub fn is_parameterless(&self) -> bool {
        self.parameters().is_empty()
    }

    /// Returns whether the function has one or more parameters.
    #[must_use]
    pub fn has_parameters(&self) -> bool {
        !self.parameters().is_empty()
    }

    /// Returns whether any immediate parameter type requires inference.
    ///
    /// This only reports the canonical `TypeExpr` inference predicate. It does
    /// not perform inference.
    #[must_use]
    pub fn parameters_require_inference(&self) -> bool {
        self.parameters()
            .iter()
            .any(TypeExpr::requires_inference)
    }

    /// Returns whether the return type requires inference.
    ///
    /// This reports source-level inference requirements only.
    #[must_use]
    pub fn return_type_requires_inference(&self) -> bool {
        self.return_type().requires_inference()
    }

    /// Returns whether the complete function type requires inference.
    ///
    /// Actual inference remains the responsibility of semantic analysis.
    #[must_use]
    pub fn requires_inference(&self) -> bool {
        self.0.requires_inference()
    }

    /// Performs canonical structural validation using the default policy.
    pub fn validate(&self) -> Result<(), TypeExprError> {
        self.0.validate()
    }

    /// Performs canonical structural validation using an explicit policy.
    ///
    /// The policy may impose resource limits for compiler safety, but those
    /// limits do not become language-level function-arity limits.
    pub fn validate_with_policy(
        &self,
        policy: &TypeValidationPolicy,
    ) -> Result<(), TypeExprError> {
        self.0.validate_with_policy(policy)
    }

    /// Returns the canonical source-level spelling of this function type.
    ///
    /// Formatting is delegated to `TypeExpr` so the repository has one
    /// authoritative representation of function-type source syntax.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        self.0.to_source_string()
    }

    /// Returns the canonical AST variant name.
    ///
    /// For every valid `FunctionType`, this is `"function"`.
    #[must_use]
    pub fn variant_name(&self) -> &'static str {
        self.0.variant_name()
    }

    /// Returns an owned clone of the canonical type expression.
    ///
    /// This is useful when an API needs ownership while preserving the
    /// canonical `TypeExpr` representation.
    #[must_use]
    pub fn to_type_expr(&self) -> TypeExpr {
        self.0.clone()
    }

    /// Returns whether this value is a function type.
    ///
    /// This method is intentionally trivial but useful when generic code works
    /// with multiple typed type-expression façades.
    #[must_use]
    pub const fn is_function(&self) -> bool {
        true
    }
}

// =============================================================================
// Conversions
// =============================================================================

impl From<FunctionType> for TypeExpr {
    fn from(value: FunctionType) -> Self {
        value.into_type_expr()
    }
}

impl TryFrom<TypeExpr> for FunctionType {
    type Error = FunctionTypeError;

    fn try_from(value: TypeExpr) -> Result<Self, Self::Error> {
        Self::try_from_type_expr(value)
    }
}

impl AsRef<TypeExpr> for FunctionType {
    fn as_ref(&self) -> &TypeExpr {
        self.as_type_expr()
    }
}

impl fmt::Display for FunctionType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_source_string())
    }
}

// =============================================================================
// Error model
// =============================================================================

/// Errors produced when converting arbitrary `TypeExpr` values into
/// `FunctionType`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FunctionTypeError {
    /// The supplied expression was not a function type.
    NotFunction {
        /// Canonical variant name of the supplied expression.
        variant: &'static str,
    },

    /// The expression was a function type but failed canonical structural
    /// validation.
    InvalidType(TypeExprError),
}

impl fmt::Display for FunctionTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFunction { variant } => {
                write!(
                    formatter,
                    "expected function type expression, found `{variant}`"
                )
            }
            Self::InvalidType(error) => {
                write!(formatter, "invalid function type expression: {error}")
            }
        }
    }
}

impl std::error::Error for FunctionTypeError {}

impl From<TypeExprError> for FunctionTypeError {
    fn from(error: TypeExprError) -> Self {
        Self::InvalidType(error)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_parameterless_function() {
        let function = FunctionType::new(
            Vec::new(),
            TypeExpr::name("Unit"),
        );

        assert!(function.is_parameterless());
        assert_eq!(function.parameter_count(), 0);
        assert_eq!(function.return_type(), &TypeExpr::name("Unit"));
    }

    #[test]
    fn constructs_function_with_parameters() {
        let function = FunctionType::new(
            vec![
                TypeExpr::name("Input"),
                TypeExpr::name("Context"),
            ],
            TypeExpr::name("Output"),
        );

        assert_eq!(function.parameter_count(), 2);
        assert_eq!(
            function.parameters(),
            &[
                TypeExpr::name("Input"),
                TypeExpr::name("Context"),
            ]
        );
        assert_eq!(function.return_type(), &TypeExpr::name("Output"));
    }

    #[test]
    fn preserves_canonical_type_expr() {
        let canonical = TypeExpr::function(
            vec![TypeExpr::name("Input")],
            TypeExpr::name("Output"),
        );

        let function =
            FunctionType::try_from_type_expr(canonical.clone()).unwrap();

        assert_eq!(function.as_type_expr(), &canonical);
        assert_eq!(function.to_type_expr(), canonical);
    }

    #[test]
    fn converts_back_to_type_expr() {
        let function = FunctionType::new(
            vec![TypeExpr::name("Input")],
            TypeExpr::name("Output"),
        );

        let canonical: TypeExpr = function.into();

        assert!(matches!(
            canonical,
            TypeExpr::Function(_, _)
        ));
    }

    #[test]
    fn rejects_non_function_type() {
        let error =
            FunctionType::try_from_type_expr(TypeExpr::name("NotFunction"))
                .unwrap_err();

        assert!(matches!(
            error,
            FunctionTypeError::NotFunction { .. }
        ));
    }

    #[test]
    fn validates_function_type() {
        let function = FunctionType::new(
            vec![
                TypeExpr::name("A"),
                TypeExpr::name("B"),
            ],
            TypeExpr::name("Result"),
        );

        assert!(function.validate().is_ok());
    }

    #[test]
    fn validates_with_explicit_policy() {
        let function = FunctionType::new(
            vec![TypeExpr::name("Input")],
            TypeExpr::name("Output"),
        );

        let policy = TypeValidationPolicy::default();

        assert!(function.validate_with_policy(&policy).is_ok());
    }

    #[test]
    fn source_formatting_is_delegated_to_type_expr() {
        let function = FunctionType::new(
            vec![
                TypeExpr::name("A"),
                TypeExpr::name("B"),
            ],
            TypeExpr::name("C"),
        );

        assert_eq!(
            function.to_source_string(),
            "fn(A, B) -> C"
        );
    }

    #[test]
    fn display_matches_source_formatting() {
        let function = FunctionType::new(
            vec![TypeExpr::name("Input")],
            TypeExpr::name("Output"),
        );

        assert_eq!(
            function.to_string(),
            function.to_source_string()
        );
    }

    #[test]
    fn detects_parameter_inference_requirement() {
        let function = FunctionType::new(
            vec![TypeExpr::infer()],
            TypeExpr::name("Output"),
        );

        assert!(function.parameters_require_inference());
        assert!(function.requires_inference());
    }

    #[test]
    fn detects_return_type_inference_requirement() {
        let function = FunctionType::new(
            vec![TypeExpr::name("Input")],
            TypeExpr::infer(),
        );

        assert!(function.return_type_requires_inference());
        assert!(function.requires_inference());
    }

    #[test]
    fn detects_nested_function_types() {
        let inner = TypeExpr::function(
            vec![TypeExpr::name("A")],
            TypeExpr::name("B"),
        );

        let outer = FunctionType::new(
            vec![inner],
            TypeExpr::name("C"),
        );

        assert_eq!(outer.parameter_count(), 1);
        assert!(matches!(
            outer.parameters()[0],
            TypeExpr::Function(_, _)
        ));
    }

    #[test]
    fn supports_function_returning_function() {
        let return_type = TypeExpr::function(
            vec![TypeExpr::name("InnerInput")],
            TypeExpr::name("InnerOutput"),
        );

        let function = FunctionType::new(
            vec![TypeExpr::name("OuterInput")],
            return_type,
        );

        assert!(matches!(
            function.return_type(),
            TypeExpr::Function(_, _)
        ));
    }

    #[test]
    fn supports_generic_parameter_types() {
        let function = FunctionType::new(
            vec![TypeExpr::generic(
                TypeExpr::name("Container"),
                vec![TypeExpr::name("T")],
            )],
            TypeExpr::name("Result"),
        );

        assert_eq!(function.parameter_count(), 1);
        assert!(!function.parameters_require_inference());
    }

    #[test]
    fn supports_generic_return_types() {
        let function = FunctionType::new(
            vec![TypeExpr::name("Input")],
            TypeExpr::generic(
                TypeExpr::name("Result"),
                vec![
                    TypeExpr::name("Output"),
                    TypeExpr::name("Error"),
                ],
            ),
        );

        assert!(matches!(
            function.return_type(),
            TypeExpr::Generic { .. }
        ));
    }

    #[test]
    fn supports_nested_function_types_without_fixed_machine_limits() {
        let mut current =
            TypeExpr::name("Base");

        for _ in 0..256 {
            current = TypeExpr::function(
                vec![current],
                TypeExpr::name("Next"),
            );
        }

        let function = FunctionType::try_from_type_expr(current)
            .expect("outermost expression must remain a function");

        assert_eq!(function.parameter_count(), 1);
    }

    #[test]
    fn supports_large_parameter_lists_without_ast_level_fixed_limits() {
        let parameters = (0..4096)
            .map(|index| {
                TypeExpr::name(format!("Parameter{index}"))
            })
            .collect::<Vec<_>>();

        let function = FunctionType::new(
            parameters,
            TypeExpr::name("Output"),
        );

        assert_eq!(function.parameter_count(), 4096);
    }

    #[test]
    fn preserves_parameter_order() {
        let function = FunctionType::new(
            vec![
                TypeExpr::name("First"),
                TypeExpr::name("Second"),
                TypeExpr::name("Third"),
            ],
            TypeExpr::name("Output"),
        );

        let names = function
            .parameters()
            .iter()
            .map(TypeExpr::to_source_string)
            .collect::<Vec<_>>();

        assert_eq!(
            names,
            vec![
                "First",
                "Second",
                "Third",
            ]
        );
    }

    #[test]
    fn supports_quantum_resource_as_an_ordinary_type() {
        let function = FunctionType::new(
            vec![
                TypeExpr::quantum(TypeExpr::name("Q")),
            ],
            TypeExpr::name("Measurement"),
        );

        assert_eq!(function.parameter_count(), 1);
        assert_eq!(
            function.parameters()[0],
            TypeExpr::quantum(TypeExpr::name("Q"))
        );
    }

    #[test]
    fn remains_domain_neutral() {
        let function = FunctionType::new(
            vec![
                TypeExpr::name("Resource"),
                TypeExpr::name("Parameter"),
            ],
            TypeExpr::name("Result"),
        );

        // FunctionType contains no backend, topology, vendor, or hardware
        // information. Its role is solely source-level type structure.
        assert_eq!(function.parameter_count(), 2);
        assert_eq!(
            function.return_type().to_source_string(),
            "Result"
        );
    }

    #[test]
    fn variant_name_is_canonical() {
        let function = FunctionType::new(
            vec![],
            TypeExpr::name("Unit"),
        );

        assert_eq!(function.variant_name(), "function");
        assert!(function.is_function());
    }

    #[test]
    fn error_has_stable_display() {
        let error =
            FunctionType::try_from_type_expr(TypeExpr::name("Int"))
                .unwrap_err();

        assert_eq!(
            error.to_string(),
            "expected function type expression, found `identifier`"
        );
    }
}