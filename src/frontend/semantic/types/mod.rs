
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Type System
//!
//! Path: `src/frontend/semantic/types/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! This module is the authoritative public aggregation boundary for the
//! native frontend's semantic type subsystem.
//!
//! It composes the canonical semantic type representation with inference,
//! unification, compatibility, generics, traits, aliases, constant
//! evaluation, ownership analysis, coercion, and type constraints.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! Normative specifications:
//! - `grammar/specification/types.md`
//! - `grammar/specification/semantics.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! Canonical source syntax:
//! - `grammar/types/`
//! - `grammar/declarations/types.g4`
//! - `grammar/declarations/user-defined-types.g4`
//!
//! Canonical source AST:
//! - `crate::frontend::ast::node::types`
//!
//! Canonical semantic type representation:
//! - `crate::frontend::semantic::types::ty::SemanticType`
//!
//! ## OWNS
//!
//! - Composition of the semantic type subsystem.
//! - Public module visibility and canonical re-exports.
//! - The documented integration boundary between type-system components.
//! - Module-surface and integration contract tests.
//!
//! ## DOES_NOT_OWN
//!
//! - Source syntax or AST construction.
//! - Name, import, or declaration resolution.
//! - A competing semantic type representation.
//! - The implementation of type inference or unification.
//! - Trait implementation lookup algorithms.
//! - Effect, capability, resource, or contract semantics.
//! - Quantum operation registries or physical qubit assignments.
//! - HDL elaboration, hardware layout, or ABI lowering.
//! - Classical IR, quantum IR, or other domain IR construction.
//! - Scheduling, routing, optimization, QEC, ZQN, HAL, or execution.
//!
//! ## MODULE OWNERSHIP
//!
//! `ty`            owns canonical resolved semantic types.
//! `inference`     owns expression type-inference orchestration.
//! `unification`   owns type-variable substitutions and unification.
//! `compatibility` owns type equality, compatibility, and assignability.
//! `generics`      owns generic parameter binding and instantiation.
//! `traits`        owns trait declarations and trait-obligation checking.
//! `aliases`       owns alias resolution and alias-cycle policies.
//! `constants`     owns exact constant values and constant evaluation.
//! `ownership`     owns ownership and borrowing analysis.
//! `coercions`     owns coercion rules and coercion decisions.
//! `constraints`   owns type-level obligations and their status.
//!
//! Each component has one implementation owner. This module must not
//! duplicate its data structures, algorithms, or semantic rules.
//!
//! ## DEPENDENCY DIRECTION
//!
//! ```text
//! Canonical AST TypeExpr
//!          |
//!          v
//! Symbol / declaration resolution
//!          |
//!          v
//! TypeExpr -> SemanticType
//!          |
//!          +-----------------------------+
//!          |                             |
//!          v                             v
//! Expression inference            Generic instantiation
//!          |                             |
//!          v                             v
//! Type constraints <----------> Type unification
//!          |                             |
//!          +--------------+--------------+
//!                         |
//!                         v
//!                  Compatibility
//!                         |
//!              +----------+----------+
//!              |                     |
//!              v                     v
//!       Domain validation      Semantic finalization
//!              |                     |
//!              +----------+----------+
//!                         |
//!                         v
//!             Canonical semantic model
//!                         |
//!                         v
//!                Downstream IR adapters
//! ```
//!
//! This diagram defines conceptual dependencies. Concrete call relationships
//! remain owned by the corresponding module APIs.
//!
//! ## POCO-REAF AND SCALABILITY
//!
//! This module imposes no fixed maximum for:
//! - qubits, processors, accelerators, devices, or distributed nodes;
//! - register widths, tensor dimensions, or generic parameter counts;
//! - user-defined types, fields, variants, or computational domains;
//! - resource quantities or symbolic type-level cardinalities.
//!
//! Type-level quantities must remain symbolic or exact until the appropriate
//! semantic rule authorizes evaluation or narrowing.
//!
//! This is not a promise of literally infinite execution. Finite processes
//! remain bounded by available memory, time, representable identities, and
//! explicit compiler operating policies.
//!
//! Operational budget exhaustion must remain distinguishable from a
//! semantically invalid program. A budget must never silently change a
//! program's type or certify an incomplete analysis as valid.
//!
//! ## ERROR AND VALIDITY POLICY
//!
//! - `SemanticType::Error` is a recovery representation, not a wildcard.
//! - An inference variable is not a finalized concrete type.
//! - An unresolved constraint is not a discharged constraint.
//! - A symbolic cardinality is not automatically equal to another
//!   cardinality.
//! - Missing extension support must not silently produce a valid type.
//! - Type resolution failures must retain enough context for diagnostics.
//! - Domain-specific types must compose with the shared semantic type model.
//!
//! ## INTEGRATION CONTRACT
//!
//! 1. The AST owns `TypeExpr` and source-level structure.
//! 2. Name resolution establishes declaration identities.
//! 3. The type resolver translates syntax into `SemanticType`.
//! 4. Inference and generic processing use the canonical type representation.
//! 5. Unification solves type-equality constraints.
//! 6. Compatibility implements the normative assignability rules.
//! 7. Constraint processing preserves unresolved obligations explicitly.
//! 8. Domain validators contribute domain-specific rules through their
//!    established semantic interfaces.
//! 9. Semantic finalization determines whether required obligations are
//!    satisfied before the program can be certified valid.
//! 10. Downstream IR adapters consume validated semantic facts without
//!     redefining source-language type meaning.
//!
//! This module does not invent a resolver API or assume a particular AST
//! traversal API. Those interfaces must be integrated with the existing
//! symbol-resolution, AST, and semantic-model modules.
//!
//! ## COMPATIBILITY
//!
//! Existing files in this subsystem are exposed through their canonical
//! module paths. The legacy `crate::semantic` subsystem remains separate
//! until its migration and compatibility checks are complete.
//!
//! Changes to public re-exports must be reviewed as API changes. Internal
//! modules must not depend on this facade when a direct sibling-module
//! dependency is sufficient.
//!
//! ## FREEZE CRITERIA
//!
//! - Every declared child module exists and compiles.
//! - Public re-exports refer to the canonical implementations.
//! - No duplicate semantic type model is introduced.
//! - All modules use the same canonical `SemanticType`.
//! - AST-to-semantic resolution is explicitly integrated.
//! - Incomplete analysis cannot be reported as certified validity.
//! - Module-level and workspace integration tests pass.
//! - The repository's supported Rust toolchain and lint checks pass.

/// Type aliases and alias-cycle handling.
pub mod aliases;

/// Explicit coercion rules and coercion decisions.
pub mod coercions;

/// Type equality, compatibility, and assignability.
pub mod compatibility;

/// Exact constant values and compile-time constant evaluation.
pub mod constants;

/// Type-level constraints and unresolved obligations.
pub mod constraints;

/// Generic parameters, substitutions, and instantiation.
pub mod generics;

/// Expression type inference.
pub mod inference;

/// Ownership, borrowing, and move analysis.
pub mod ownership;

/// Trait declarations, implementations, and trait obligations.
pub mod traits;

/// Canonical target-independent semantic types.
pub mod ty;

/// Type-variable substitution and unification.
pub mod unification;

// =============================================================================
// Canonical semantic type API
// =============================================================================

/// Canonical symbolic cardinalities, names, types, and type-level values.
///
/// These are resolved semantic representations, not AST syntax nodes.
pub use self::ty::{
    Cardinality,
    FloatFormat,
    FunctionParameter,
    IntegerSignedness,
    IntegerWidth,
    ParameterMode,
    PrimitiveType,
    QualifiedName,
    SemanticName,
    SemanticType,
    TypeAttribute,
    TypeLevelValue,
    TypeParameter,
    TypeValidationError,
    TypeValidationPolicy,
    TypeVariableId,
    SEMANTIC_TYPE_SCHEMA_VERSION,
};

// =============================================================================
// Core constraint-solving API
// =============================================================================

/// Canonical type-constraint collection and constraint status.
pub use self::constraints::{
    Constraint,
    ConstraintConfig,
    ConstraintError,
    ConstraintId,
    ConstraintIdAllocator,
    ConstraintSet,
    ConstraintStatus,
    TypeConstraint,
    TypeRelation,
    UnresolvedReason,
    ValueRelation,
};

/// Canonical type-unification service and its structured outcomes.
pub use self::unification::{
    TypeUnifier,
    UnificationConfig,
    UnificationError,
    UnificationResult,
};

// =============================================================================
// Compatibility API
// =============================================================================

/// Type-compatibility decisions and policies.
///
/// `Compatibility::Compatible` must only be returned when the applicable
/// language rules establish compatibility. Unknown or error types must not
/// be accepted as a universal compatibility shortcut.
pub use self::compatibility::{
    check_compatibility,
    is_assignable,
    types_equal,
    ArrayLength,
    Compatibility,
    CompatibilityFailure,
    CompatibilityMode,
    CompatibilityPolicy,
    TypeShape,
    TypeView,
};

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::{
        Cardinality,
        PrimitiveType,
        SemanticType,
        TypeLevelValue,
    };

    /// The facade exposes the canonical type representation rather than a
    /// duplicate type enum.
    #[test]
    fn canonical_type_is_available_through_facade() {
        let ty = SemanticType::Primitive(PrimitiveType::Boolean);

        assert_eq!(
            ty,
            SemanticType::Primitive(PrimitiveType::Boolean)
        );
    }

    /// Type-level integers retain their exact decimal representation instead
    /// of narrowing to a machine-sized integer.
    #[test]
    fn large_type_level_integer_is_preserved() {
        let decimal = "12345678901234567890123456789012345678901234567890";

        let value = TypeLevelValue::integer(decimal)
            .expect("valid non-negative decimal integer");

        assert_eq!(
            value,
            TypeLevelValue::Integer(decimal.into())
        );
    }

    /// Symbolic cardinalities remain symbolic until an appropriate solver
    /// establishes their value.
    #[test]
    fn symbolic_cardinality_is_preserved() {
        let value = TypeLevelValue::Parameter("N".into());
        let cardinality = Cardinality::Symbolic(value.clone());

        assert_eq!(
            cardinality,
            Cardinality::Symbolic(value)
        );
    }
}
