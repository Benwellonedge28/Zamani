
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Hybrid Computing — Classical Integration
//!
//! Path: `src/hybrid/integration/classical.rs`
//!
//! ## Purpose
//!
//! Define the integration boundary through which canonical classical
//! computation participates in a hybrid Zamani program.
//!
//! This module connects classical IR and semantic facts to hybrid
//! composition without defining a second classical IR, type system,
//! expression language, identifier system, compiler pipeline, or runtime.
//!
//! ## Canonical ownership
//!
//! The following subsystems remain authoritative:
//!
//! - `crate::frontend`: source syntax, AST and source locations.
//! - `crate::frontend::semantic`: source-language semantic validation.
//! - `crate::quantum::ir::classical`: classical semantics used by
//!   the canonical quantum IR.
//! - `crate::classical::zcir`: classical-facing ZUIR integration.
//! - `crate::zuir`: canonical universal IR, where its relevant APIs
//!   are available and registered.
//! - `crate::compiler`: compilation and target-realization pipelines.
//! - `crate::runtime`: execution lifecycle and runtime resources.
//! - `crate::hybrid`: cross-domain composition contracts.
//!
//! This module owns only the classical-to-hybrid adapter contract,
//! structural integration checks, and traversal hooks.
//!
//! It does not claim ownership of the canonical classical operations,
//! values, types, IDs, control flow, effects, or serialization.
//!
//! ## Dependency direction
//!
//! ```text
//! Frontend AST
//!     |
//!     v
//! Canonical semantic model
//!     |
//!     v
//! Canonical classical IR / ZUIR
//!     |
//!     v
//! This integration boundary
//!     |
//!     v
//! Hybrid composition and graph verification
//!     |
//!     v
//! Compiler planning and target realization
//!     |
//!     v
//! Existing backend and runtime
//! ```
//!
//! The diagram specifies ownership and intended integration. It does
//! not assert that every downstream module is currently implemented.
//!
//! ## Scalability
//!
//! No universal hardware-size, operation-count, collection-size,
//! domain-count, tensor-rank, or graph-size ceilings are introduced.
//!
//! This module does not impose an arbitrary limit on traversal length.
//! Callers may enforce explicit operational budgets in the owning
//! validation or compilation context.
//!
//! A budget exhaustion or unavailable required capability must not
//! be interpreted as successful validation.
//!
//! Finite host resources and finite identifier representations remain
//! real constraints; this module does not promise literal infinity.
//!
//! ## Semantic preservation
//!
//! An adapter must preserve the meaning of canonical classical IR.
//! It must not silently:
//!
//! - Drop a classical dependency.
//! - Change a value's type or ownership.
//! - Treat an asynchronous result as immediately available.
//! - Remove observable effects.
//! - Reinterpret a predicate or control-flow condition.
//! - Replace an unsupported operation with a different operation.
//! - Convert a target limitation into a source-language restriction.
//!
//! This module defines generic contracts because the inspected repository
//! does not yet expose a complete, registered hybrid graph API to which
//! a concrete graph-writing adapter can safely bind.
//!
//! Implementations of these contracts must use the real canonical IR
//! APIs; they must not create replacement representations to satisfy
//! this interface.

use core::fmt;

/// Result type used by classical integration operations.
pub type ClassicalIntegrationResult<T, E> = Result<T, E>;

/// A stable, adapter-level classification of integration failures.
///
/// The owning semantic, IR, compiler, or hybrid subsystem remains
/// responsible for translating these failures into its canonical
/// structured diagnostic or error representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ClassicalIntegrationErrorKind {
    /// The canonical classical representation is structurally invalid.
    InvalidRepresentation,

    /// A required canonical entity or reference cannot be resolved.
    UnresolvedReference,

    /// A dependency required for semantic preservation is missing.
    MissingDependency,

    /// A required capability or semantic guarantee is unavailable.
    UnsupportedRequirement,

    /// The supplied adapter or interface is incompatible.
    IncompatibleInterface,

    /// A caller-supplied operational budget was exhausted.
    BudgetExhausted,

    /// An integration implementation failed.
    IntegrationFailure,
}

impl ClassicalIntegrationErrorKind {
    /// Return a stable machine-readable category identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidRepresentation => {
                "classical.invalid_representation"
            }
            Self::UnresolvedReference => {
                "classical.unresolved_reference"
            }
            Self::MissingDependency => {
                "classical.missing_dependency"
            }
            Self::UnsupportedRequirement => {
                "classical.unsupported_requirement"
            }
            Self::IncompatibleInterface => {
                "classical.incompatible_interface"
            }
            Self::BudgetExhausted => {
                "classical.budget_exhausted"
            }
            Self::IntegrationFailure => {
                "classical.integration_failure"
            }
        }
    }
}

impl fmt::Display for ClassicalIntegrationErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// An error returned by an adapter or integration visitor.
///
/// The error contains a stable category and contextual message. Callers
/// should use `kind()` for programmatic classification rather than parsing
/// the human-readable message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalIntegrationError {
    kind: ClassicalIntegrationErrorKind,
    context: String,
}

impl ClassicalIntegrationError {
    /// Construct an integration error.
    ///
    /// The caller supplies context from the canonical subsystem that
    /// detected the problem. This type does not parse diagnostic strings
    /// or replace the source subsystem's original error.
    #[must_use]
    pub fn new(
        kind: ClassicalIntegrationErrorKind,
        context: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            context: context.into(),
        }
    }

    /// Return the stable error category.
    #[must_use]
    pub const fn kind(&self) -> ClassicalIntegrationErrorKind {
        self.kind
    }

    /// Return the contextual explanation.
    #[must_use]
    pub fn context(&self) -> &str {
        &self.context
    }
}

impl fmt::Display for ClassicalIntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.context)
    }
}

impl std::error::Error for ClassicalIntegrationError {}

/// Describes a canonical classical IR provider.
///
/// Implement this trait for an existing canonical IR or ZUIR view.
///
/// Associated types are borrowed references to entities owned by the
/// provider. They are deliberately not replaced by hybrid-owned IDs.
///
/// `Operation`, `Value`, and `Dependency` are provider-owned types.
/// They may be canonical ZUIR entities or references into another
/// authoritative classical representation.
///
/// The trait does not require operations to be copied, converted to
/// strings, or re-created as hybrid operations.
///
/// The provider must preserve the semantics and identity of every
/// entity returned through this interface.
pub trait ClassicalIrView {
    /// Canonical operation or operation-reference type.
    type Operation: ?Sized;

    /// Canonical value or value-reference type.
    type Value: ?Sized;

    /// Canonical dependency or dependency-reference type.
    type Dependency: ?Sized;

    /// Provider-specific error type.
    type Error: std::error::Error;

    /// Visit canonical classical operations in a stable, documented order.
    ///
    /// Implementations must not silently omit unsupported operations.
    /// The visitor may stop traversal by returning an error.
    fn visit_operations(
        &self,
        visitor: &mut dyn FnMut(
            &Self::Operation,
        ) -> ClassicalIntegrationResult<(), ClassicalIntegrationError>,
    ) -> ClassicalIntegrationResult<(), Self::Error>;

    /// Visit canonical classical values referenced by the fragment.
    ///
    /// The provider determines which values belong to the supplied
    /// fragment and preserves their canonical identities.
    fn visit_values(
        &self,
        visitor: &mut dyn FnMut(
            &Self::Value,
        ) -> ClassicalIntegrationResult<(), ClassicalIntegrationError>,
    ) -> ClassicalIntegrationResult<(), Self::Error>;

    /// Visit dependencies required to preserve the fragment's meaning.
    ///
    /// Data, control, completion, ordering, and feedback dependencies
    /// must remain distinguishable in the provider's canonical model.
    fn visit_dependencies(
        &self,
        visitor: &mut dyn FnMut(
            &Self::Dependency,
        ) -> ClassicalIntegrationResult<(), ClassicalIntegrationError>,
    ) -> ClassicalIntegrationResult<(), Self::Error>;

    /// Validate the fragment using the provider's authoritative verifier.
    ///
    /// This must include every structural check that the provider claims
    /// is required for safe integration. It must not report success when
    /// validation is incomplete.
    fn validate(
        &self,
    ) -> ClassicalIntegrationResult<(), Self::Error>;
}

/// A sink that consumes canonical classical entities during hybrid
/// composition.
///
/// The sink is implemented by the owning hybrid graph builder or another
/// canonical composition component once that API is available.
///
/// It receives borrowed canonical entities. It must not silently discard
/// an entity or dependency it cannot represent.
pub trait ClassicalCompositionSink<V: ClassicalIrView> {
    /// Sink-specific error type.
    type Error: std::error::Error;

    /// Accept a validated canonical classical operation.
    fn accept_operation(
        &mut self,
        operation: &V::Operation,
    ) -> ClassicalIntegrationResult<(), Self::Error>;

    /// Accept a canonical classical value reference.
    fn accept_value(
        &mut self,
        value: &V::Value,
    ) -> ClassicalIntegrationResult<(), Self::Error>;

    /// Accept a canonical dependency.
    fn accept_dependency(
        &mut self,
        dependency: &V::Dependency,
    ) -> ClassicalIntegrationResult<(), Self::Error>;
}

/// Transfer a validated classical fragment into a hybrid composition sink.
///
/// This function deliberately performs validation before visiting entities,
/// so an invalid fragment is not partially accepted before its initial
/// validation fails.
///
/// The sink must preserve all accepted operations, values, and dependencies.
/// If accepting an entity fails partway through traversal, the sink is
/// responsible for documenting whether its state is transactional or
/// partially updated. This function does not pretend to provide rollback
/// that the sink does not implement.
///
/// The provider's own error type is preserved at the boundary. This
/// adapter does not parse error strings or invent a replacement semantic
/// diagnosis.
pub fn integrate_classical<V, S>(
    view: &V,
    sink: &mut S,
) -> ClassicalIntegrationResult<(), ClassicalIntegrationError>
where
    V: ClassicalIrView,
    S: ClassicalCompositionSink<V>,
{
    view.validate().map_err(|error| {
        ClassicalIntegrationError::new(
            ClassicalIntegrationErrorKind::InvalidRepresentation,
            error.to_string(),
        )
    })?;

    view.visit_operations(&mut |operation| {
        sink.accept_operation(operation).map_err(|error| {
            ClassicalIntegrationError::new(
                ClassicalIntegrationErrorKind::IntegrationFailure,
                error.to_string(),
            )
        })
    })
    .map_err(|error| {
        ClassicalIntegrationError::new(
            ClassicalIntegrationErrorKind::IntegrationFailure,
            error.to_string(),
        )
    })?;

    view.visit_values(&mut |value| {
        sink.accept_value(value).map_err(|error| {
            ClassicalIntegrationError::new(
                ClassicalIntegrationErrorKind::IntegrationFailure,
                error.to_string(),
            )
        })
    })
    .map_err(|error| {
        ClassicalIntegrationError::new(
            ClassicalIntegrationErrorKind::IntegrationFailure,
            error.to_string(),
        )
    })?;

    view.visit_dependencies(&mut |dependency| {
        sink.accept_dependency(dependency).map_err(|error| {
            ClassicalIntegrationError::new(
                ClassicalIntegrationErrorKind::MissingDependency,
                error.to_string(),
            )
        })
    })
    .map_err(|error| {
        ClassicalIntegrationError::new(
            ClassicalIntegrationErrorKind::IntegrationFailure,
            error.to_string(),
        )
    })?;

    Ok(())
}

/// Reports which semantic properties an adapter promises to preserve.
///
/// These declarations are not proof of conformance. The provider,
/// verifier, and conformance tests must establish that the promises hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClassicalPreservationContract {
    /// Preserve canonical operation meaning.
    pub operation_semantics: bool,

    /// Preserve canonical value identities and type associations.
    pub value_identity_and_types: bool,

    /// Preserve data and control dependencies.
    pub dependencies: bool,

    /// Preserve required ordering and completion relationships.
    pub ordering_and_completion: bool,

    /// Preserve ownership, effects, and provenance.
    pub ownership_effects_and_provenance: bool,
}

impl ClassicalPreservationContract {
    /// A contract declaring all listed properties mandatory.
    #[must_use]
    pub const fn required() -> Self {
        Self {
            operation_semantics: true,
            value_identity_and_types: true,
            dependencies: true,
            ordering_and_completion: true,
            ownership_effects_and_provenance: true,
        }
    }

    /// Check whether every mandatory property is declared preserved.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        self.operation_semantics
            && self.value_identity_and_types
            && self.dependencies
            && self.ordering_and_completion
            && self.ownership_effects_and_provenance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_categories_have_stable_identifiers() {
        assert_eq!(
            ClassicalIntegrationErrorKind::MissingDependency.as_str(),
            "classical.missing_dependency"
        );
    }

    #[test]
    fn integration_error_preserves_kind_and_context() {
        let error = ClassicalIntegrationError::new(
            ClassicalIntegrationErrorKind::UnresolvedReference,
            "value reference could not be resolved",
        );

        assert_eq!(
            error.kind(),
            ClassicalIntegrationErrorKind::UnresolvedReference
        );
        assert_eq!(
            error.context(),
            "value reference could not be resolved"
        );
    }

    #[test]
    fn required_preservation_contract_is_complete() {
        assert!(ClassicalPreservationContract::required().is_complete());
    }

    #[test]
    fn incomplete_preservation_contract_is_detected() {
        let contract = ClassicalPreservationContract {
            operation_semantics: true,
            value_identity_and_types: true,
            dependencies: false,
            ordering_and_completion: true,
            ownership_effects_and_provenance: true,
        };

        assert!(!contract.is_complete());
    }
}
