//! # Zamani Semantic Frontend — Structural Validation Boundary
//!
//! Proposed path:
//! `src/frontend/semantic/validation/structural.rs`
//!
//! STATUS: Integration adapter; requires frontend module integration.
//!
//! PURPOSE
//!
//! Provide the semantic pipeline with a stable entry point for structural
//! AST validation while preserving the AST layer as the sole owner of
//! structural invariants and graph traversal.
//!
//! OWNERSHIP
//!
//! This module owns:
//! - the semantic pipeline's structural-validation entry points;
//! - the choice between complete-program and AST-fragment validation;
//! - forwarding explicit structural-validation policies;
//! - the contract that semantic analysis must not proceed after failure.
//!
//! This module does NOT own:
//! - AST nodes, node identities, or source spans;
//! - graph traversal, cycle detection, or revisit detection;
//! - source-range validation algorithms;
//! - symbol resolution, type checking, or effect checking;
//! - resource feasibility, hardware inspection, or target selection.
//!
//! INTEGRATION
//!
//! Canonical structural implementation:
//! `crate::frontend::ast::node::validation`
//!
//! Canonical graph traversal:
//! `crate::frontend::ast::node::visitors::walk`
//!
//! Input: a canonical AST provider and a root NodeId.
//! Output: canonical structural-validation statistics or a typed error.
//!
//! SCALABILITY
//!
//! No machine-capacity limits are introduced here. Optional traversal
//! budgets remain explicit operational policies owned by the canonical
//! validator. Actual processing remains bounded by available resources.
//!
//! SAFETY
//!
//! Safe Rust only. No unsafe code.
//!
//! FREEZE CRITERIA
//!
//! 1. The canonical frontend module is exposed by the crate.
//! 2. The canonical AST validation module is public and re-exports the
//!    documented types and functions used below.
//! 3. The semantic pipeline invokes this boundary before semantic passes.
//! 4. Integration tests prove that validation errors stop semantic analysis.
//! 5. The workspace compiles and the structural-validation tests pass.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::traversal::walk::AstWalkProvider;
use crate::frontend::ast::node::validation::{
    validate_fragment,
    validate_program_ast,
    validate_with_config,
    StructuralValidationConfig,
    StructuralValidationError,
    StructuralValidationStatistics,
};

/// Validates the complete structure reachable from an AST root.
///
/// This function does not require the root to be a program node. Use
/// [`validate_program_structure`] when validating a complete program.
///
/// Structural graph checks are delegated to the canonical AST validator.
pub fn validate_structure<P>(
    provider: &P,
    root: NodeId,
) -> Result<
    StructuralValidationStatistics,
    StructuralValidationError<P::Error>,
>
where
    P: AstWalkProvider,
    P::Error: fmt::Display,
{
    validate_fragment(provider, root)
}

/// Validates an AST root using an explicit structural-validation policy.
///
/// Operational limits supplied through `config` are safety budgets, not
/// language-level restrictions on the size of a Zamani program.
pub fn validate_structure_with_config<P>(
    provider: &P,
    root: NodeId,
    config: StructuralValidationConfig,
) -> Result<
    StructuralValidationStatistics,
    StructuralValidationError<P::Error>,
>
where
    P: AstWalkProvider,
    P::Error: fmt::Display,
{
    validate_with_config(provider, root, config)
}

/// Validates a complete Zamani program AST.
///
/// The canonical AST validator verifies that the root has the required
/// program-node kind and that the reachable structure satisfies its
/// configured structural invariants.
pub fn validate_program_structure<P>(
    provider: &P,
    root: NodeId,
) -> Result<
    StructuralValidationStatistics,
    StructuralValidationError<P::Error>,
>
where
    P: AstWalkProvider,
    P::Error: fmt::Display,
{
    validate_program_ast(provider, root)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Compile-time contract: the default policy must not introduce
    /// language-level AST node, edge, or depth ceilings.
    #[test]
    fn default_policy_does_not_impose_artificial_scale_limits() {
        let config = StructuralValidationConfig::default();

        assert_eq!(config.traversal.max_nodes, None);
        assert_eq!(config.traversal.max_edges, None);
        assert_eq!(config.traversal.max_depth, None);
    }

    /// Compile-time contract: callers can opt into operational safeguards
    /// without changing the meaning of the source program.
    #[test]
    fn operational_budgets_are_explicit() {
        let config = StructuralValidationConfig::default()
            .with_max_nodes(10_000)
            .with_max_edges(40_000)
            .with_max_depth(512);

        assert_eq!(config.traversal.max_nodes, Some(10_000));
        assert_eq!(config.traversal.max_edges, Some(40_000));
        assert_eq!(config.traversal.max_depth, Some(512));
    }

    /// Compile-time contract: requiring a complete program root is an
    /// explicit validation choice, not a restriction on AST fragments.
    #[test]
    fn program_root_requirement_is_explicit() {
        let config = StructuralValidationConfig::default()
            .requiring_program_root(true);

        assert!(config.require_program_root);
    }
}