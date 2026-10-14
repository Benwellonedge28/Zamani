
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Traversal
//!
//! Path: `src/frontend/semantic/traversal/walk.rs`
//!
//! ## PURPOSE
//!
//! Provide the semantic subsystem with a stable, domain-neutral entry point
//! for traversing the canonical Zamani frontend AST.
//!
//! This module delegates all structural traversal to the existing canonical
//! AST walker. It deliberately does not implement another traversal engine.
//!
//! ## STATUS
//!
//! Proposed implementation for the new semantic subsystem.
//!
//! ## AUTHORITY
//!
//! - `grammar/DESIGN.md`
//! - `grammar/CONFORMANCE.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `src/frontend/ast/node/traversal/walk.rs`
//! - `src/frontend/ast/node/traversal/children.rs`
//! - `src/frontend/ast/node/visitors/visitor.rs`
//!
//! The normative language specification determines program meaning.
//! The canonical AST determines source structure. The canonical AST walker
//! determines structural traversal order.
//!
//! ## OWNS
//!
//! - Semantic-facing traversal entry points.
//! - Delegation to the canonical AST walker.
//! - Public exposure of the existing traversal configuration, cancellation,
//!   statistics, provider, and error contracts.
//! - Documentation of the semantic traversal boundary.
//!
//! ## DOES_NOT_OWN
//!
//! - AST node definitions or child relationships.
//! - A second AST visitor protocol.
//! - Name resolution, symbol tables, or scope management.
//! - Type inference, unification, or type checking.
//! - Effect, capability, resource, or contract validation.
//! - Domain-specific semantic rules.
//! - Semantic result storage or incremental dependency tracking.
//! - IR construction, target feasibility, resource allocation,
//!   scheduling, routing, or runtime execution.
//!
//! ## INPUTS
//!
//! A read-only provider implementing the canonical AST walk-provider
//! contract, a root `NodeId`, an `AstVisitor`, and optional traversal
//! policies and cancellation.
//!
//! ## OUTPUTS
//!
//! Canonical traversal statistics on successful completion, or the
//! canonical traversal error containing the provider or visitor error.
//!
//! ## INTEGRATION CONTRACT
//!
//! `traversal/visitor.rs` must use the existing AST visitor contract or
//! provide an explicit adapter implementing it. It must not introduce a
//! competing child traversal algorithm.
//!
//! `traversal/mod.rs` should re-export this module's public entry points.
//!
//! `pipeline.rs` may invoke these entry points during structural semantic
//! passes. Pass-specific state belongs to the visitor or analysis session.
//!
//! ## SCALABILITY
//!
//! This module introduces no fixed limits on program size, AST node count,
//! nesting depth, computational domains, resource populations, or hardware.
//!
//! Callers may choose explicit operational budgets through `WalkConfig`.
//! The canonical walker enforces those budgets. They are not language-level
//! limits.
//!
//! No finite implementation can promise completion for literally infinite
//! input. Unrestricted traversal means no additional artificial limit is
//! introduced by this adapter; available memory, time, address space, and
//! representational capacity still apply.
//!
//! ## DETERMINISM
//!
//! This module preserves the canonical walker's child ordering and traversal
//! behavior. It does not sort nodes, iterate over additional collections,
//! introduce randomness, or depend on hardware or runtime state.
//!
//! ## ERROR HANDLING
//!
//! Provider failures, missing nodes, identity mismatches, cycles, repeated
//! nodes, configured limit violations, cancellation, and visitor failures
//! are returned by the canonical walker without being silently discarded.
//!
//! ## SAFETY
//!
//! Safe Rust only. No raw pointers, unsafe blocks, unchecked indexing,
//! recursive traversal, or hidden global state are introduced here.
//!
//! ## FREEZE CRITERIA
//!
//! - The canonical AST walker and visitor APIs compile.
//! - The AST traversal module's public re-exports are internally consistent.
//! - The semantic traversal module exports these entry points.
//! - Delegation preserves canonical results and errors.
//! - Semantic integration tests pass.
//! - The full workspace builds and relevant tests pass.

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::traversal::walk::{
    walk as canonical_walk,
    walk_with_config as canonical_walk_with_config,
    walk_with_config_and_cancellation as canonical_walk_with_config_and_cancellation,
    AstWalkProvider,
    Cancellation,
    NeverCancelled,
    WalkConfig,
    WalkError,
    WalkStatistics,
};
use crate::frontend::ast::visitors::visitor::AstVisitor;

// =============================================================================
// Public semantic traversal contract
// =============================================================================

/// Result of a semantic-facing AST traversal.
///
/// The error type preserves both provider and visitor failures from the
/// canonical AST traversal engine.
pub type SemanticWalkResult<T, PE, VE> = Result<T, WalkError<PE, VE>>;

/// Re-export the canonical provider contract.
///
/// A semantic provider resolves AST nodes and enumerates their children
/// through the existing AST traversal infrastructure. It must not invent a
/// second representation of source-level child relationships.
pub use crate::frontend::ast::node::traversal::walk::AstWalkProvider as SemanticNodeProvider;

/// Re-export canonical traversal policies and outcomes.
///
/// These are aliases to the AST traversal contract, not independently
/// implemented semantic policies.
pub use crate::frontend::ast::node::traversal::walk::{
    Cancellation as SemanticCancellation,
    NeverCancelled as NeverSemanticCancellation,
    WalkConfig as SemanticWalkConfig,
    WalkError as SemanticWalkError,
    WalkStatistics as SemanticWalkStatistics,
};

// =============================================================================
// Default traversal
// =============================================================================

/// Traverse the AST rooted at `root` for a semantic analysis pass.
///
/// The canonical walker's default configuration is used. By default, repeated
/// node identities are rejected, while node, edge, and depth budgets remain
/// unrestricted.
///
/// # Arguments
///
/// - `provider`: read-only access to canonical AST nodes and child references.
/// - `root`: identity of the source AST node at which traversal begins.
/// - `visitor`: the semantic pass's visitor, implementing the canonical AST
///   visitor contract.
///
/// # Returns
///
/// Traversal statistics when the walk completes, including when the visitor
/// requests successful early termination. Returns a structured traversal
/// error if traversal cannot continue.
///
/// # Ordering
///
/// Nodes are visited in the canonical walker's deterministic depth-first
/// order, with each parent's children consumed in provider-defined order.
///
/// # Errors
///
/// Returns missing-node, identity, cycle, revisit, provider, cancellation, or
/// visitor errors as defined by the canonical AST walker.
///
/// # Example
///
/// ```ignore
/// let statistics =
///     crate::frontend::semantic::traversal::walk::walk(
///         &ast_store,
///         root_id,
///         &mut semantic_visitor,
///     )?;
/// ```
///
/// # Scalability
///
/// This function adds no artificial size or depth ceiling. It does not
/// guarantee that an arbitrarily large traversal will fit the resources
/// available to the current process.
pub fn walk<P, V>(
    provider: &P,
    root: NodeId,
    visitor: &mut V,
) -> SemanticWalkResult<WalkStatistics, P::Error, V::Error>
where
    P: AstWalkProvider,
    V: AstVisitor,
{
    canonical_walk(provider, root, visitor)
}

// =============================================================================
// Configurable traversal
// =============================================================================

/// Traverse an AST using an explicit operational policy.
///
/// Use this entry point when a compiler service needs an explicit node, edge,
/// or logical-depth budget, or wants to permit DAG-style shared subtrees.
///
/// `config.reject_revisits = false` permits completed nodes to be encountered
/// more than once. The canonical walker must still reject active-path cycles.
///
/// Operational limits do not redefine the language's semantic validity. If a
/// configured budget is exceeded, traversal returns an error instead of
/// reporting an incomplete walk as a complete analysis.
pub fn walk_with_config<P, V>(
    provider: &P,
    root: NodeId,
    visitor: &mut V,
    config: WalkConfig,
) -> SemanticWalkResult<WalkStatistics, P::Error, V::Error>
where
    P: AstWalkProvider,
    V: AstVisitor,
{
    canonical_walk_with_config(provider, root, visitor, config)
}

// =============================================================================
// Configurable traversal and cancellation
// =============================================================================

/// Traverse an AST using explicit policies and caller-owned cancellation.
///
/// Cancellation remains independent of asynchronous runtimes, compiler
/// schedulers, target devices, and hardware execution systems.
///
/// The cancellation source is checked by the canonical walker at its
/// documented traversal boundaries.
///
/// # Cancellation semantics
///
/// Cancellation is an unsuccessful, incomplete traversal represented by
/// `WalkError::Cancelled`. It must not be interpreted as proof that the
/// program is semantically valid.
///
/// A caller that needs to distinguish a complete semantic result from an
/// incomplete result must retain the `Result` outcome and must not publish
/// a certified-valid result after cancellation.
pub fn walk_with_config_and_cancellation<P, V, C>(
    provider: &P,
    root: NodeId,
    visitor: &mut V,
    config: WalkConfig,
    cancellation: &C,
) -> SemanticWalkResult<WalkStatistics, P::Error, V::Error>
where
    P: AstWalkProvider,
    V: AstVisitor,
    C: Cancellation,
{
    canonical_walk_with_config_and_cancellation(
        provider,
        root,
        visitor,
        config,
        cancellation,
    )
}

// =============================================================================
// Explicit unrestricted entry point
// =============================================================================

/// Traverse without imposing node, edge, or depth budgets.
///
/// This is equivalent to calling [`walk_with_config`] with
/// [`WalkConfig::unrestricted`].
///
/// Repeated-node rejection remains enabled by default because the native AST
/// is expected to be structurally tree-shaped. Disabling that policy requires
/// an explicit configuration and does not disable cycle detection.
///
/// "Unrestricted" describes this module's operational policy only. It does
/// not imply infinite memory, infinite time, unlimited integer representation,
/// or guaranteed completion for an infinite graph.
pub fn walk_unrestricted<P, V>(
    provider: &P,
    root: NodeId,
    visitor: &mut V,
) -> SemanticWalkResult<WalkStatistics, P::Error, V::Error>
where
    P: AstWalkProvider,
    V: AstVisitor,
{
    canonical_walk_with_config(
        provider,
        root,
        visitor,
        WalkConfig::unrestricted(),
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Compile-time contract check: the semantic adapter uses the canonical
    /// provider interface rather than introducing another provider trait.
    fn assert_semantic_provider<P: SemanticNodeProvider>() {}

    /// Compile-time contract check for the canonical visitor interface.
    fn assert_semantic_visitor<V: AstVisitor>() {}

    #[test]
    fn unrestricted_configuration_has_no_artificial_budgets() {
        let config = SemanticWalkConfig::unrestricted();

        assert_eq!(config.max_nodes, None);
        assert_eq!(config.max_edges, None);
        assert_eq!(config.max_depth, None);

        // An AST is tree-shaped by default. DAG sharing must be an explicit
        // caller decision; active-path cycle detection remains canonical.
        assert!(config.reject_revisits);
    }

    #[test]
    fn default_configuration_is_unrestricted_for_size_and_depth() {
        let config = SemanticWalkConfig::default();

        assert_eq!(config.max_nodes, None);
        assert_eq!(config.max_edges, None);
        assert_eq!(config.max_depth, None);
        assert!(config.reject_revisits);
    }

    #[test]
    fn semantic_surface_uses_canonical_contracts() {
        // Keep these generic function references type-checked without
        // requiring a particular AST storage implementation in this module.
        let _provider_contract = assert_semantic_provider::<
            crate::frontend::semantic::traversal::tests::ProviderPlaceholder
        >;
        let _visitor_contract = assert_semantic_visitor::<
            crate::frontend::semantic::traversal::tests::VisitorPlaceholder
        >;
    }
}
