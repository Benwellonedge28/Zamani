
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Semantic Integration
//!
//! Path: src/stdlib/integration/semantic.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide the standard library's integration boundary with the
//! canonical native frontend semantic-analysis subsystem.
//!
//! This module allows standard-library consumers to use the
//! authoritative semantic API without introducing a competing
//! semantic analyzer, semantic model, AST, type system, or
//! diagnostic implementation.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! Canonical semantic authority:
//!
//!     crate::frontend::semantic
//!
//! Canonical AST:
//!
//!     crate::frontend::ast::node
//!
//! Canonical semantic API:
//!
//!     crate::frontend::semantic::api
//!
//! Canonical semantic model:
//!
//!     crate::frontend::semantic::model
//!
//! Canonical diagnostics:
//!
//!     crate::frontend::semantic::diagnostics
//!
//! Normative specifications:
//!
//!     grammar/DESIGN.md
//!     grammar/Zamani.g4
//!     grammar/specification/semantic-model.md
//!     grammar/specification/semantic-boundaries.md
//!     grammar/specification/poco-reaf.md
//!     grammar/specification/scalability-model.md
//!
//! ## OWNS
//!
//! - Standard-library access to semantic analysis.
//! - Integration of the canonical semantic diagnostic contract.
//! - Integration of the canonical semantic model validity contract.
//! - Stable standard-library semantic API re-exports.
//! - Semantic integration contract tests.
//!
//! ## DOES_NOT_OWN
//!
//! - AST definitions or AST construction.
//! - Semantic analysis algorithms.
//! - Symbol resolution.
//! - Type inference or type checking.
//! - Effect analysis.
//! - Capability analysis.
//! - Resource analysis.
//! - Contract validation.
//! - Policy or provenance semantics.
//! - Domain-specific semantic validation.
//! - Intermediate representations.
//! - Compiler optimization or lowering.
//! - Hardware discovery or resource allocation.
//! - Scheduling, routing, or execution.
//!
//! Those responsibilities remain with their canonical owners.
//!
//! ## POCO-REAF
//!
//! Semantic meaning must remain independent of the physical
//! execution environment.
//!
//! No hardware-specific maximum or universal capacity limit
//! is introduced here.
//!
//! Symbolic resource requirements, capabilities, constraints,
//! preferences, and hints remain owned by the semantic subsystem.
//!
//! Actual target feasibility is determined downstream.
//!
//! This module does not promise literally infinite computation.
//! It introduces no arbitrary universal hardware ceiling.
//!
//! ## DEPENDENCY DIRECTION
//!
//!     Zamani source
//!          |
//!          v
//!     Canonical AST
//!          |
//!          v
//!     Frontend semantic authority
//!          |
//!          v
//!     Standard-library semantic integration
//!          |
//!          v
//!     Standard-library domain APIs
//!          |
//!          v
//!     Canonical IR / compiler / runtime
//!
//! The standard library consumes semantic authority.
//! The semantic authority must not depend on this adapter.
//!
//! ## INTEGRATION CONTRACT
//!
//! - `Program` is owned by the canonical frontend AST.
//! - `SemanticService` is owned by the canonical semantic API.
//! - `SemanticResult` is owned by the canonical semantic API.
//! - `SemanticProgram` is owned by the canonical semantic model.
//! - `Diagnostic` is owned by canonical semantic diagnostics.
//! - `SemanticApiError` preserves structural and service failures.
//!
//! No semantic information is silently discarded or reconstructed.
//!
//! ## ERROR POLICY
//!
//! Source-language errors are represented by canonical diagnostics.
//!
//! Structural AST errors remain `SemanticApiError::InvalidProgram`.
//!
//! Infrastructure failures remain `SemanticApiError::Service`.
//!
//! Incomplete analysis cannot be reported as valid.
//!
//! Target infeasibility is not equivalent to semantic invalidity.
//!
//! ## SAFETY
//!
//! This module uses safe Rust exclusively.
//!
//! It introduces no global mutable state, hardware assumptions,
//! environment inspection, or hidden initialization.

use crate::frontend::semantic::api::{
    SemanticApiError,
    SemanticDiagnostic,
    SemanticModelState,
    SemanticResult,
    SemanticService,
    SemanticStatus,
    SEMANTIC_API_VERSION,
};

use crate::frontend::semantic::diagnostics::Diagnostic;

use crate::frontend::semantic::model::SemanticProgram;

// =============================================================================
// CANONICAL SEMANTIC API
// =============================================================================

/// Canonical semantic-analysis entry point exposed through
/// the standard-library integration boundary.
///
/// This is the original frontend implementation, not a
/// second standard-library analyzer.
pub use crate::frontend::semantic::api::analyze as analyze_program;

/// Canonical semantic-analysis service contract.
pub use crate::frontend::semantic::api::SemanticService as StandardSemanticService;

/// Canonical semantic-analysis result.
pub use crate::frontend::semantic::api::SemanticResult as StandardSemanticResult;

/// Canonical semantic-analysis status.
pub use crate::frontend::semantic::api::SemanticStatus as StandardSemanticStatus;

/// Canonical semantic-analysis API error.
pub use crate::frontend::semantic::api::SemanticApiError as StandardSemanticApiError;

/// Canonical semantic diagnostic interface.
pub use crate::frontend::semantic::api::SemanticDiagnostic as StandardSemanticDiagnostic;

/// Canonical semantic-model validity interface.
pub use crate::frontend::semantic::api::SemanticModelState as StandardSemanticModelState;

/// Canonical API version.
pub const STANDARD_SEMANTIC_API_VERSION: u16 =
    SEMANTIC_API_VERSION;

// =============================================================================
// CANONICAL AST INTEGRATION
// =============================================================================

/// Canonical frontend program accepted by semantic analysis.
///
/// This is a re-export, not a new AST representation.
pub use crate::frontend::ast::node::program::Program as SemanticInputProgram;

// =============================================================================
// CANONICAL MODEL INTEGRATION
// =============================================================================

/// Canonical semantic program model.
///
/// The generic parameters are supplied by the authoritative
/// semantic fact, obligation, and provenance subsystems.
pub type StandardSemanticProgram<F, O = (), P = ()> =
    SemanticProgram<F, O, P>;

/// Connect the canonical semantic program model to the
/// semantic API's model-state contract.
///
/// Completion and validity are obtained directly from the
/// authoritative model.
///
/// No second validity flag is maintained here.
impl<F, O, P> SemanticModelState for SemanticProgram<F, O, P> {
    /// Whether all required semantic analysis work completed.
    fn establishes_completion(&self) -> bool {
        self.completion().is_complete()
    }

    /// Whether the canonical model establishes semantic validity.
    ///
    /// This deliberately does not establish target feasibility.
    fn establishes_validity(&self) -> bool {
        self.validity().is_valid()
    }
}

// =============================================================================
// CANONICAL DIAGNOSTIC INTEGRATION
// =============================================================================

/// Connect the canonical structured diagnostic implementation
/// to the semantic API diagnostic contract.
///
/// Error and fatal diagnostics remain errors.
/// Warnings and informational diagnostics do not become errors.
impl SemanticDiagnostic for Diagnostic {
    fn is_error(&self) -> bool {
        Diagnostic::is_error(self)
    }
}

// =============================================================================
// STANDARD-LIBRARY RESULT CONTRACT
// =============================================================================

/// Standard-library semantic result using the canonical
/// semantic model and canonical diagnostic implementation.
///
/// `F` represents semantic facts.
/// `O` represents unresolved or discharged obligations.
/// `P` represents provenance information.
///
/// These types remain owned by their respective semantic
/// authorities.
pub type CanonicalStandardSemanticResult<F, O = (), P = ()> =
    SemanticResult<
        SemanticProgram<F, O, P>,
        Diagnostic,
    >;

// =============================================================================
// SEMANTIC OUTCOME HELPERS
// =============================================================================

/// Determine whether a semantic-analysis outcome completed.
///
/// Completion does not imply semantic validity.
#[must_use]
pub const fn analysis_completed(
    status: SemanticStatus,
) -> bool {
    status.is_complete()
}

/// Determine whether an analysis outcome is non-complete.
///
/// Incomplete, aborted, and failed analyses must not be
/// treated as successful validity certificates.
#[must_use]
pub const fn analysis_did_not_complete(
    status: SemanticStatus,
) -> bool {
    status.is_non_complete()
}

// =============================================================================
// API CONTRACT TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::semantic::api::{
        SemanticDiagnostic,
        SemanticModelState,
    };

    /// The canonical semantic model implements the API contract.
    fn assert_semantic_model<T: SemanticModelState>() {}

    /// The canonical diagnostic implementation implements
    /// the semantic diagnostic contract.
    fn assert_semantic_diagnostic<T: SemanticDiagnostic>() {}

    #[test]
    fn canonical_semantic_model_is_integrated() {
        assert_semantic_model::<SemanticProgram<(), (), ()>>();
    }

    #[test]
    fn canonical_diagnostic_is_integrated() {
        assert_semantic_diagnostic::<Diagnostic>();
    }

    #[test]
    fn semantic_api_version_is_preserved() {
        assert_eq!(
            STANDARD_SEMANTIC_API_VERSION,
            SEMANTIC_API_VERSION,
        );
    }

    #[test]
    fn complete_status_is_recognized() {
        assert!(analysis_completed(SemanticStatus::Complete));
        assert!(!analysis_did_not_complete(SemanticStatus::Complete));
    }

    #[test]
    fn incomplete_status_is_not_completion() {
        assert!(!analysis_completed(SemanticStatus::Incomplete));
        assert!(analysis_did_not_complete(SemanticStatus::Incomplete));
    }

    #[test]
    fn aborted_status_is_not_completion() {
        assert!(!analysis_completed(SemanticStatus::Aborted));
        assert!(analysis_did_not_complete(SemanticStatus::Aborted));
    }

    #[test]
    fn failed_status_is_not_completion() {
        assert!(!analysis_completed(SemanticStatus::Failed));
        assert!(analysis_did_not_complete(SemanticStatus::Failed));
    }

    #[test]
    fn canonical_semantic_input_is_available() {
        fn accepts_canonical_program(
            _program: &SemanticInputProgram,
        ) {
        }

        let _ = accepts_canonical_program;
    }
}

// =============================================================================
// END
// =============================================================================
