
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Semantic Type Validation
//!
//! Path: `src/classical/validation/types.rs`
//! Edition: Rust 2021
//! Minimum intended Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Provide the classical-computing entry point for structural validation of
//! canonical semantic types.
//!
//! This module delegates type representation and structural invariants to
//! `crate::frontend::semantic::types::ty::SemanticType`. It does not define
//! another semantic type system, AST, type inference engine, unifier,
//! constraint solver, or intermediate representation.
//!
//! # Authority
//!
//! Source syntax:
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `src/frontend/ast/node/types/type_expr.rs`
//!
//! Normative semantic specifications:
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/types.md`
//! - `grammar/spec/type-system.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! Canonical semantic type implementation:
//! - `src/frontend/semantic/types/ty.rs`
//!
//! Canonical semantic type integrations:
//! - `src/frontend/semantic/types/generics`
//! - `src/frontend/semantic/types/constraints`
//! - `src/frontend/semantic/types/unification`
//! - `src/frontend/semantic/types/compatibility`
//!
//! Classical-facing type descriptors:
//! - `src/classical/types/scalar.rs`
//! - `src/classical/types/aggregate.rs`
//! - `src/classical/types/generic.rs`
//! - `src/classical/types/constraints.rs`
//! - `src/classical/types/layout.rs`
//!
//! # Ownership
//!
//! This module owns:
//! - Classical-facing semantic type validation policy.
//! - The classical validation error envelope.
//! - The distinction between structurally valid intermediate types and
//!   finalized types suitable for downstream compilation.
//! - The stable classical validation entry points.
//!
//! This module does not own:
//! - `SemanticType` or its variants.
//! - Source `TypeExpr` syntax or AST structural validation.
//! - Name resolution, type inference, unification, or substitution.
//! - Generic argument binding or constraint proving.
//! - Trait resolution or overload resolution.
//! - Ownership, borrowing, effects, or lifetime checking.
//! - Target ABI, memory layout, or resource feasibility.
//! - Classical IR or ZUIR construction.
//! - Hardware selection, scheduling, or execution.
//!
//! # Validation boundary
//!
//! ```text
//! Source grammar
//!      |
//!      v
//! TypeExpr
//!      |
//!      v
//! AST structural validation
//!      |
//!      v
//! Name resolution and semantic type construction
//!      |
//!      v
//! Canonical SemanticType
//!      |
//!      v
//! this classical validation adapter
//!      |
//!      +--> intermediate type validation
//!      |
//!      +--> finalized type validation
//!      |
//!      v
//! semantic model / canonical IR lowering
//! ```
//!
//! A successful result here does not prove that a name resolves, a generic
//! constraint is satisfied, an operation is type-compatible, or a target can
//! represent the type. Those guarantees belong to their respective owners.
//!
//! # Intermediate and finalized types
//!
//! Semantic analysis may temporarily contain inference variables, contextual
//! `Self` types, associated types awaiting resolution, or explicit recovery
//! types.
//!
//! Intermediate validation checks structural well-formedness while permitting
//! those states when configured to do so.
//!
//! Finalized validation rejects incomplete types. An error-recovery type is
//! never treated as a successfully resolved type.
//!
//! # POCO-REAF and scalability
//!
//! This module imposes no universal maximum on:
//! - Integer width.
//! - Tuple arity.
//! - Function parameter count.
//! - Generic argument count.
//! - Array cardinality.
//! - Symbolic dimensions.
//! - Type nesting as a language-level restriction.
//! - Hardware, target, or domain count.
//!
//! An optional traversal-depth budget is an operational safeguard, not a
//! semantic restriction. When a caller supplies a finite budget, exceeding
//! it returns a structured error instead of silently accepting the type.
//!
//! No finite process can handle literally infinite input. Actual work remains
//! bounded by available memory, time, and representational limits.
//!
//! # Integration
//!
//! 1. `src/classical/validation/mod.rs` declares this module.
//! 2. `src/classical/mod.rs` exposes `validation`.
//! 3. Semantic analysis produces canonical `SemanticType` values.
//! 4. This module validates those values through the canonical type API.
//! 5. Generic and constraint checks remain delegated to their existing
//!    semantic implementations.
//! 6. Lowering consumes validated semantic results and performs its own
//!    IR-specific and target-specific checks.
//!
//! # Freeze criteria
//!
//! This file is complete when:
//! - Its parent module exports it.
//! - Canonical semantic types remain the only semantic type representation.
//! - Intermediate and finalized policies are covered by tests.
//! - Malformed types and exhausted budgets fail explicitly.
//! - No target capability is inferred from structural type validity.
//! - The repository's supported formatting, compilation, test, and lint
//!   checks pass.
//!
//! Repository integration must be verified before this file is marked
//! compiled, tested, or production-ready.

use core::fmt;

use crate::frontend::semantic::types::ty::{
    SemanticType,
    TypeValidationError as SemanticTypeValidationError,
    TypeValidationPolicy as SemanticTypeValidationPolicy,
};

// ---------------------------------------------------------------------------
// Validation policy
// ---------------------------------------------------------------------------

/// The validation stage requested by the caller.
///
/// Intermediate types are useful during inference and semantic recovery.
/// Finalized types must not contain unresolved inference or contextual types.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TypeValidationStage {
    /// Validate structural invariants while allowing incomplete semantic types.
    ///
    /// This is suitable for compiler stages that intentionally retain
    /// inference variables or deferred type resolution.
    Intermediate,

    /// Require a complete semantic type suitable for downstream validation.
    ///
    /// This does not establish generic constraint satisfaction, operation
    /// compatibility, or target feasibility.
    #[default]
    Finalized,
}

/// Configuration for classical semantic type validation.
///
/// This policy delegates structural checks to the canonical semantic type
/// implementation. It does not override canonical type semantics.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ClassicalTypeValidationConfig {
    /// Optional operational traversal-depth budget.
    ///
    /// The canonical semantic validator interprets the root as depth zero.
    /// `None` adds no policy-level depth limit.
    pub max_depth: Option<usize>,

    /// Validation stage to enforce.
    pub stage: TypeValidationStage,
}

impl ClassicalTypeValidationConfig {
    /// Creates a policy for intermediate semantic analysis.
    #[must_use]
    pub const fn intermediate() -> Self {
        Self {
            max_depth: None,
            stage: TypeValidationStage::Intermediate,
        }
    }

    /// Creates a policy for finalized semantic types.
    #[must_use]
    pub const fn finalized() -> Self {
        Self {
            max_depth: None,
            stage: TypeValidationStage::Finalized,
        }
    }

    /// Sets an optional operational depth budget.
    ///
    /// A budget is not a language-level type-depth limit. It applies only to
    /// the current validation invocation.
    #[must_use]
    pub const fn with_max_depth(mut self, max_depth: Option<usize>) -> Self {
        self.max_depth = max_depth;
        self
    }

    /// Sets the validation stage.
    #[must_use]
    pub const fn with_stage(mut self, stage: TypeValidationStage) -> Self {
        self.stage = stage;
        self
    }
}

// ---------------------------------------------------------------------------
// Structured errors
// ---------------------------------------------------------------------------

/// Error returned by the classical semantic type validation boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClassicalTypeValidationError {
    /// The canonical semantic type failed its structural validation.
    Structural(SemanticTypeValidationError),

    /// The type is incomplete at a stage that requires finalization.
    IncompleteType,

    /// An explicit semantic error-recovery type was supplied where a valid
    /// finalized type was required.
    ErrorRecoveryType,
}

impl fmt::Display for ClassicalTypeValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Structural(error) => {
                write!(formatter, "invalid semantic type structure: {error}")
            }
            Self::IncompleteType => {
                formatter.write_str(
                    "semantic type is incomplete and cannot be finalized",
                )
            }
            Self::ErrorRecoveryType => {
                formatter.write_str(
                    "semantic error-recovery type cannot establish type validity",
                )
            }
        }
    }
}

impl std::error::Error for ClassicalTypeValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Structural(error) => Some(error),
            Self::IncompleteType | Self::ErrorRecoveryType => None,
        }
    }
}

impl From<SemanticTypeValidationError> for ClassicalTypeValidationError {
    fn from(error: SemanticTypeValidationError) -> Self {
        Self::Structural(error)
    }
}

// ---------------------------------------------------------------------------
// Validation entry points
// ---------------------------------------------------------------------------

/// Validates a canonical semantic type using the default finalized policy.
///
/// Finalized validation rejects inference variables, contextual types,
/// unresolved associated types, and explicit error-recovery types.
///
/// # Errors
///
/// Returns a structural error when the canonical semantic validator rejects
/// the type, or an incomplete-type error when finalization is required but
/// unresolved type information remains.
///
/// # Important distinction
///
/// Success means that the type is structurally valid and complete under the
/// canonical semantic type model. It does not prove that generic bounds,
/// traits, operation compatibility, effects, or target constraints are
/// satisfied.
pub fn validate_classical_type(
    ty: &SemanticType,
) -> Result<(), ClassicalTypeValidationError> {
    validate_classical_type_with_config(
        ty,
        &ClassicalTypeValidationConfig::finalized(),
    )
}

/// Validates a canonical semantic type using an explicit policy.
///
/// Structural invariants are always delegated to `SemanticType::validate`.
/// This adapter does not reimplement the canonical type validator.
///
/// The `max_depth` setting is passed unchanged to the canonical validator.
/// A finite depth budget may be appropriate for untrusted or generated input.
/// It must not be presented as a language-level restriction.
///
/// # Errors
///
/// Returns a typed error for malformed structure, exhausted depth budget,
/// or an incomplete type when the selected stage requires finalization.
pub fn validate_classical_type_with_config(
    ty: &SemanticType,
    config: &ClassicalTypeValidationConfig,
) -> Result<(), ClassicalTypeValidationError> {
    let canonical_policy = SemanticTypeValidationPolicy {
        max_depth: config.max_depth,
    };

    ty.validate(&canonical_policy)
        .map_err(ClassicalTypeValidationError::Structural)?;

    if config.stage == TypeValidationStage::Intermediate {
        return Ok(());
    }

    // Report an explicit root recovery type distinctly. Nested recovery
    // types are also incomplete, and are rejected by the check below.
    if ty.is_error() {
        return Err(ClassicalTypeValidationError::ErrorRecoveryType);
    }

    if ty.is_incomplete() {
        return Err(ClassicalTypeValidationError::IncompleteType);
    }

    Ok(())
}

/// Validates a collection of canonical semantic types in caller-supplied
/// order.
///
/// The first error is returned. The function does not reorder inputs,
/// infer missing types, or silently discard invalid entries.
///
/// This helper is useful for function signatures, generic argument lists,
/// aggregate members, and other ordered semantic type collections.
pub fn validate_classical_types(
    types: &[SemanticType],
    config: &ClassicalTypeValidationConfig,
) -> Result<(), ClassicalTypeValidationError> {
    for ty in types {
        validate_classical_type_with_config(ty, config)?;
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::semantic::types::ty::{
        IntegerSignedness,
        IntegerWidth,
        QualifiedName,
    };

    fn named_type(name: &str) -> SemanticType {
        SemanticType::named(QualifiedName::single(name))
    }

    fn integer_type() -> SemanticType {
        SemanticType::integer(
            IntegerSignedness::Unsigned,
            IntegerWidth::bits("128").expect("valid integer width"),
        )
    }

    #[test]
    fn finalized_policy_accepts_a_complete_named_type() {
        let ty = named_type("Number");

        assert_eq!(validate_classical_type(&ty), Ok(()));
    }

    #[test]
    fn finalized_policy_accepts_a_wide_integer_type() {
        // Width remains a decimal semantic value. This validator does not
        // narrow it to the host's usize or infer target support.
        let ty = integer_type();

        assert_eq!(validate_classical_type(&ty), Ok(()));
    }

    #[test]
    fn finalized_policy_rejects_an_inference_variable() {
        use crate::frontend::semantic::types::ty::TypeVariableId;

        let ty = SemanticType::Variable(TypeVariableId(1));

        assert_eq!(
            validate_classical_type(&ty),
            Err(ClassicalTypeValidationError::IncompleteType),
        );
    }

    #[test]
    fn intermediate_policy_allows_an_inference_variable() {
        use crate::frontend::semantic::types::ty::TypeVariableId;

        let ty = SemanticType::Variable(TypeVariableId(1));
        let config = ClassicalTypeValidationConfig::intermediate();

        assert_eq!(
            validate_classical_type_with_config(&ty, &config),
            Ok(()),
        );
    }

    #[test]
    fn finalized_policy_rejects_an_error_recovery_type() {
        assert_eq!(
            validate_classical_type(&SemanticType::Error),
            Err(ClassicalTypeValidationError::ErrorRecoveryType),
        );
    }

    #[test]
    fn intermediate_policy_can_preserve_an_error_recovery_type() {
        // Recovery types can be retained during error recovery, but their
        // presence must not be confused with finalized semantic validity.
        let config = ClassicalTypeValidationConfig::intermediate();

        assert_eq!(
            validate_classical_type_with_config(&SemanticType::Error, &config),
            Ok(()),
        );
    }

    #[test]
    fn malformed_integer_width_is_rejected() {
        let ty = SemanticType::integer(
            IntegerSignedness::Unsigned,
            IntegerWidth::Bits("not-a-width".into()),
        );

        assert!(matches!(
            validate_classical_type(&ty),
            Err(ClassicalTypeValidationError::Structural(_)),
        ));
    }

    #[test]
    fn empty_qualified_name_is_rejected() {
        let ty = SemanticType::named(QualifiedName::default());

        assert_eq!(
            validate_classical_type(&ty),
            Err(ClassicalTypeValidationError::Structural(
                SemanticTypeValidationError::EmptyQualifiedName,
            )),
        );
    }

    #[test]
    fn nested_incomplete_type_is_rejected_when_finalized() {
        use crate::frontend::semantic::types::ty::TypeVariableId;

        let ty = SemanticType::array(
            SemanticType::Variable(TypeVariableId(7)),
            crate::frontend::semantic::types::ty::Cardinality::Unspecified,
        );

        assert_eq!(
            validate_classical_type(&ty),
            Err(ClassicalTypeValidationError::IncompleteType),
        );
    }

    #[test]
    fn zero_depth_budget_accepts_a_leaf_type() {
        let ty = named_type("Leaf");
        let config = ClassicalTypeValidationConfig::finalized()
            .with_max_depth(Some(0));

        assert_eq!(
            validate_classical_type_with_config(&ty, &config),
            Ok(()),
        );
    }

    #[test]
    fn depth_budget_rejects_a_nested_type() {
        let ty = SemanticType::array(
            named_type("Element"),
            crate::frontend::semantic::types::ty::Cardinality::Unspecified,
        );

        let config = ClassicalTypeValidationConfig::finalized()
            .with_max_depth(Some(0));

        assert!(matches!(
            validate_classical_type_with_config(&ty, &config),
            Err(ClassicalTypeValidationError::Structural(
                SemanticTypeValidationError::TraversalBudgetExceeded {
                    limit: 0,
                },
            )),
        ));
    }

    #[test]
    fn validates_type_lists_in_input_order() {
        let types = vec![named_type("First"), integer_type()];

        assert_eq!(
            validate_classical_types(
                &types,
                &ClassicalTypeValidationConfig::finalized(),
            ),
            Ok(()),
        );
    }

    #[test]
    fn empty_type_list_is_valid() {
        assert_eq!(
            validate_classical_types(
                &[],
                &ClassicalTypeValidationConfig::finalized(),
            ),
            Ok(()),
        );
    }

    #[test]
    fn type_list_stops_at_the_first_invalid_type() {
        use crate::frontend::semantic::types::ty::TypeVariableId;

        let types = vec![
            named_type("Valid"),
            SemanticType::Variable(TypeVariableId(9)),
            named_type("NotReached"),
        ];

        assert_eq!(
            validate_classical_types(
                &types,
                &ClassicalTypeValidationConfig::finalized(),
            ),
            Err(ClassicalTypeValidationError::IncompleteType),
        );
    }

    #[test]
    fn depth_budget_is_configuration_not_a_global_ceiling() {
        let unrestricted = ClassicalTypeValidationConfig::default();
        let bounded =
            ClassicalTypeValidationConfig::finalized().with_max_depth(Some(64));

        assert_eq!(unrestricted.max_depth, None);
        assert_eq!(bounded.max_depth, Some(64));
    }

    #[test]
    fn policy_builder_preserves_the_requested_stage() {
        let config = ClassicalTypeValidationConfig::default()
            .with_stage(TypeValidationStage::Intermediate)
            .with_max_depth(Some(12));

        assert_eq!(config.stage, TypeValidationStage::Intermediate);
        assert_eq!(config.max_depth, Some(12));
    }
}
