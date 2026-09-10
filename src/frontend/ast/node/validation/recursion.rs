//! # Zamani Frontend AST — Recursion and Structural-Depth Validation
//!
//! `src/frontend/ast/node/validation/recursion.rs`
//!
//! This module defines the recursion-safety policy for the native Zamani AST.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! lexer / parser
//!      │
//!      ▼
//! Native Zamani AST
//!      │
//!      ▼
//! structural validation
//!      │
//!      ├── structural.rs
//!      ├── source_ranges.rs
//!      └── recursion.rs  ← this module
//!      │
//!      ▼
//! semantic analysis
//!      │
//!      ▼
//! Semantic Model
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ▼
//! domain / target / hardware lowering
//! ```
//!
//! ## Purpose
//!
//! This module owns the AST-specific recursion/depth safety boundary.
//!
//! It answers:
//!
//! - Is an AST structurally traversable without violating the caller's
//!   configured operational depth policy?
//! - Does the canonical AST walker detect a structural cycle?
//! - Does the AST contain an explicitly disallowed repeated node when tree
//!   semantics are requested?
//! - What structural depth was observed?
//!
//! It deliberately does **not** own the traversal algorithm.
//!
//! The canonical iterative walker in:
//!
//! ```text
//! src/frontend/ast/node/traversal/walk.rs
//! ```
//!
//! already owns:
//!
//! - iterative traversal;
//! - active-path cycle detection;
//! - completed-node revisit detection;
//! - cancellation;
//! - node limits;
//! - edge limits;
//! - depth limits;
//! - traversal statistics;
//! - provider error propagation;
//! - visitor error propagation.
//!
//! This module must reuse that implementation rather than introducing a
//! competing traversal mechanism.
//!
//! ## Why this file exists separately
//!
//! Recursion safety is a distinct validation concern from ordinary structural
//! validation.
//!
//! `structural.rs` is responsible for graph-level AST invariants.
//!
//! `source_ranges.rs` is responsible for source-coordinate/range invariants.
//!
//! This module provides the explicit policy boundary for structural depth and
//! recursion safety while delegating graph traversal to the canonical walker.
//!
//! ```text
//! recursion.rs
//!      │
//!      ▼
//! traversal/walk.rs
//!      │
//!      ▼
//! AstWalkProvider
//!      │
//!      ▼
//! AST storage
//! ```
//!
//! ## Critical distinction: recursion versus cycles
//!
//! A deeply nested AST is not necessarily cyclic.
//!
//! ```text
//! A → B → C → D → E → ...\n//! ```
//!
//! is finite even when it is extremely deep.
//!
//! A cycle is different:
//!
//! ```text
//! A → B → C → A\n//! ```
//!
//! A cycle is structurally invalid for the native source AST because it does
//! not represent a finite source tree.
//!
//! Cycle detection therefore remains mandatory regardless of whether an
//! operational depth limit is configured.
//!
//! The canonical walker distinguishes:
//!
//! ```text
//! active-path repetition     → CycleDetected
//! completed-node repetition  → RevisitedNode when tree mode is enabled
//! excessive logical depth    → DepthLimitExceeded
//! ```
//!
//! This module does not reproduce those algorithms.
//!
//! ## POCO-REAF
//!
//! There is intentionally **no fixed recursion limit**.
//!
//! In particular, this file must never contain language semantics such as:
//!
//! ```text
//! MAX_RECURSION = 1024
//! MAX_AST_DEPTH = 4096
//! MAX_NESTING = 1000
//! ```
//!
//! Such constants would make the AST impose an artificial limit on otherwise
//! valid programs.
//!
//! Instead, the caller may explicitly provide an operational depth budget.
//!
//! ```text
//! no configured limit
//!       │
//!       ▼
//! traverse until resources are exhausted
//!
//! configured limit
//!       │
//!       ▼
//! enforce caller-selected safety policy
//! ```
//!
//! An operational limit is a compiler/service safety policy, not a Zamani
//! language limitation.
//!
//! \"Infinity\" in POCO-REAF means that the AST architecture introduces no
//! artificial finite computational-size restriction. Real execution remains
//! constrained only by representational limits and resources actually
//! available to the compiler/runtime.
//!
//! ## Domain neutrality
//!
//! This module contains no knowledge of:
//!
//! - quantum computation;
//! - qubits;
//! - quantum gates;
//! - QEC;
//! - routing;
//! - scheduling;
//! - calibration;
//! - hardware topology;
//! - CPU;
//! - GPU;
//! - FPGA;
//! - ASIC;
//! - vendor backends;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - ZUIR.
//!
//! The same recursion policy therefore applies to:
//!
//! - classical programs;
//! - quantum programs;
//! - hybrid programs;
//! - HDL programs;
//! - accelerator programs;
//! - distributed programs;
//! - future computational domains.
//!
//! ## No Rust-stack recursion
//!
//! This module must never recursively call itself for AST traversal.
//!
//! A source program can be structurally deeper than the Rust call stack can
//! safely represent. The canonical walker is iterative and stores traversal
//! state explicitly.
//!
//! Therefore:
//!
//! ```text
//! AST depth ≠ Rust call-stack depth\n//! ```
//!
//! This distinction is essential for very deeply nested source programs.
//!
//! ## Dependencies
//!
//! This module may depend only on:
//!
//! - `core` / `std`;
//! - the canonical AST node identity types;
//! - the canonical AST traversal interfaces;
//! - the canonical visitor interface.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - hardware;
//! - backend execution;
//! - optimizer;
//! - scheduler;
//! - router;
//! - QEC;
//! - runtime.
//!
//! ## Integration contract
//!
//! ```text
//! parser
//!   │
//!   ▼
//! native AST
//!   │
//!   ▼
//! AST provider
//!   │
//!   ▼
//! recursion::validate
//!   │
//!   ▼
//! traversal::walk
//!   │
//!   ├── cycle detection
//!   ├── revisit policy
//!   ├── depth policy
//!   ├── cancellation
//!   └── traversal statistics
//!   │
//!   ▼
//! structural validation
//!   │
//!   ▼
//! semantic analysis
//! ```
//!
//! A future AST node does not require this module to change merely because the
//! node exists. The node only needs to participate in the canonical
//! `AstWalkProvider` / `AstChildrenProvider` contracts.
//!
//! ## Error model
//!
//! The canonical `WalkError` is preserved rather than converted into strings.
//! This allows callers to distinguish:
//!
//! - cycles;
//! - revisits;
//! - depth exhaustion;
//! - node exhaustion;
//! - edge exhaustion;
//! - cancellation;
//! - provider failures;
//! - visitor failures.
//!
//! No global diagnostic representation is imposed here.
//!
//! ## Determinism
//!
//! Determinism is inherited from the canonical walker and child provider.
//!
//! This module never:
//!
//! - sorts node IDs;
//! - changes child ordering;
//! - allocates node IDs;
//! - consults wall-clock time;
//! - uses randomness;
//! - consults hardware;
//! - depends on hash-map iteration order.
//!
//! ## Thread safety
//!
//! The validation policy is immutable after construction.
//!
//! The validator owns no global mutable state.
//!
//! Read-only AST providers may therefore be validated concurrently by
//! independent validator instances when the provider itself supports
//! concurrent access.
//!
//! ## Serialization
//!
//! Recursion validation policy is compiler/runtime configuration, not AST
//! source semantics. It must therefore not be serialized into the native AST.
//!
//! The AST serialization layer may serialize the resulting AST and its source
//! structure, but it must not silently serialize a process-specific traversal
//! budget as if it were language syntax.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! ============================================================================
//! Module configuration
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::convert::Infallible;

use super::super::node_id::NodeId;
use super::super::traversal::walk::{
    walk_with_config,
    AstWalkProvider,
    WalkConfig,
    WalkError,
    WalkResult,
    WalkStatistics,
};
use super::super::visitors::visitor::NoopVisitor;

// ============================================================================
// Public policy
// ============================================================================

/// Policy controlling structural recursion/depth validation.
///
/// This is an **operational compiler policy**.
///
/// It is not part of Zamani source semantics and must not be interpreted as a
/// machine-size, qubit-count, or language-size limitation.
///
/// The default is unrestricted with respect to logical AST depth.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RecursionPolicy {
    /// Optional maximum logical AST depth.
    ///
    /// Depth zero is the validation root.
    ///
    /// `None` means that this module imposes no artificial depth limit.
    max_depth: Option<usize>,

    /// Whether completed-node revisits should be rejected.
    ///
    /// `true` is appropriate for ordinary source-tree ASTs.
    ///
    /// `false` permits DAG-style structural sharing while still rejecting
    /// active-path cycles.
    reject_revisits: bool,
}

impl RecursionPolicy {
    /// Creates an unrestricted tree-oriented recursion policy.
    ///
    /// No artificial depth limit is imposed.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_depth: None,
            reject_revisits: true,
        }
    }

    /// Creates a policy with an explicit operational depth limit.
    ///
    /// The supplied value is a compiler/service safety budget only.
    #[must_use]
    pub const fn with_max_depth(max_depth: usize) -> Self {
        Self {
            max_depth: Some(max_depth),
            reject_revisits: true,
        }
    }

    /// Returns the configured maximum logical depth.
    #[must_use]
    pub const fn max_depth(self) -> Option<usize> {
        self.max_depth
    }

    /// Enables or disables rejection of completed-node revisits.
    ///
    /// Disabling this allows DAG-style sharing.
    ///
    /// Active-path cycles remain invalid and are always rejected by the
    /// canonical walker.
    #[must_use]
    pub const fn reject_revisits(mut self, enabled: bool) -> Self {
        self.reject_revisits = enabled;
        self
    }

    /// Returns whether completed-node revisits are rejected.
    #[must_use]
    pub const fn rejects_revisits(self) -> bool {
        self.reject_revisits
    }

    /// Converts this policy to the canonical walker configuration.
    #[must_use]
    pub const fn to_walk_config(self) -> WalkConfig {
        WalkConfig {
            max_nodes: None,
            max_edges: None,
            max_depth: self.max_depth,
            reject_revisits: self.reject_revisits,
        }
    }
}

// ============================================================================
// Validation result
// ============================================================================

/// Successful recursion validation result.
///
/// The statistics are measurements of the traversal performed. They are not
/// language limits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RecursionValidation {
    /// Number of AST nodes traversed.
    pub nodes_visited: usize,

    /// Number of child relationships inspected.
    pub edges_visited: usize,

    /// Maximum logical AST depth observed.
    pub maximum_depth: usize,
}

impl RecursionValidation {
    /// Creates an empty validation result.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            nodes_visited: 0,
            edges_visited: 0,
            maximum_depth: 0,
        }
    }

    /// Converts canonical walker statistics into recursion statistics.
    #[must_use]
    pub const fn from_walk(statistics: WalkStatistics) -> Self {
        Self {
            nodes_visited: statistics.nodes_visited,
            edges_visited: statistics.edges_visited,
            maximum_depth: statistics.maximum_depth,
        }
    }
}

// ============================================================================
// Error type
// ============================================================================

/// Error produced by recursion/depth validation.
///
/// Provider and traversal failures remain typed and are not flattened into
/// diagnostic strings.
#[derive(Debug)]
#[non_exhaustive]
pub enum RecursionValidationError<PE> {
    /// The requested root does not exist.
    MissingRoot {
        /// Root node identity supplied by the caller.
        id: NodeId,
    },

    /// The canonical walker reported a structural failure.
    Walk(WalkError<PE, Infallible>),
}

impl<PE: core::fmt::Display> core::fmt::Display for RecursionValidationError<PE> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingRoot { id } => {
                write!(
                    formatter,
                    "AST recursion validation root {id:?} could not be resolved"
                )
            }

            Self::Walk(error) => {
                write!(formatter, "AST recursion validation failed: {error}")
            }
        }
    }
}

impl<PE> std::error::Error for RecursionValidationError<PE>
where
    PE: std::error::Error + 'static,
{
}

// ============================================================================
// Classification helpers
// ============================================================================

impl<PE> RecursionValidationError<PE> {
    /// Returns the underlying walker error when this error originated from
    /// traversal.
    #[must_use]
    pub const fn walk_error(&self) -> Option<&WalkError<PE, Infallible>> {
        match self {
            Self::MissingRoot { .. } => None,
            Self::Walk(error) => Some(error),
        }
    }

    /// Returns `true` when validation failed because a structural cycle was
    /// detected.
    #[must_use]
    pub fn is_cycle(&self) -> bool {
        matches!(
            self,
            Self::Walk(WalkError::CycleDetected { .. })
        )
    }

    /// Returns `true` when validation failed because a completed node was
    /// revisited while tree semantics were required.
    #[must_use]
    pub fn is_revisit(&self) -> bool {
        matches!(
            self,
            Self::Walk(WalkError::RevisitedNode { .. })
        )
    }

    /// Returns `true` when the configured operational depth budget was
    /// exceeded.
    #[must_use]
    pub fn is_depth_limit_exceeded(&self) -> bool {
        matches!(
            self,
            Self::Walk(WalkError::DepthLimitExceeded { .. })
        )
    }

    /// Returns `true` when the configured node budget was exceeded.
    #[must_use]
    pub fn is_node_limit_exceeded(&self) -> bool {
        matches!(
            self,
            Self::Walk(WalkError::NodeLimitExceeded { .. })
        )
    }

    /// Returns `true` when the configured edge budget was exceeded.
    #[must_use]
    pub fn is_edge_limit_exceeded(&self) -> bool {
        matches!(
            self,
            Self::Walk(WalkError::EdgeLimitExceeded { .. })
        )
    }

    /// Returns `true` when validation was cancelled.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        matches!(self, Self::Walk(WalkError::Cancelled))
    }
}

// ============================================================================
// Canonical validation entry points
// ============================================================================

/// Validates structural recursion/depth using the unrestricted default policy.
///
/// This is the normal POCO-REAF entry point.
///
/// It imposes:
///
/// - no artificial node limit;
/// - no artificial edge limit;
/// - no artificial depth limit;
/// - tree semantics with completed-node revisit rejection;
/// - mandatory cycle rejection.
///
/// The canonical iterative walker remains responsible for traversal.
///
/// # Complexity
///
/// For a finite tree, traversal is linear in the number of visited nodes plus
/// inspected edges, subject to the storage/provider implementation.
///
/// No recursive Rust call stack is used by the walker.
pub fn validate<P>(
    provider: &P,
    root: NodeId,
) -> Result<RecursionValidation, RecursionValidationError<P::Error>>
where
    P: AstWalkProvider,
{
    validate_with_policy(provider, root, RecursionPolicy::unrestricted())
}

/// Validates structural recursion/depth using an explicit policy.
///
/// This function does not alter the AST and does not perform semantic
/// validation.
///
/// The provider remains responsible for canonical node storage and child
/// relationships.
pub fn validate_with_policy<P>(
    provider: &P,
    root: NodeId,
    policy: RecursionPolicy,
) -> Result<RecursionValidation, RecursionValidationError<P::Error>>
where
    P: AstWalkProvider,
{
    let mut visitor = NoopVisitor;

    let statistics = walk_with_config(
        provider,
        root,
        &mut visitor,
        policy.to_walk_config(),
    )
    .map_err(|error| match error {
        WalkError::MissingNode { id } => {
            RecursionValidationError::MissingRoot { id }
        }

        other => RecursionValidationError::Walk(other),
    })?;

    Ok(RecursionValidation::from_walk(statistics))
}

/// Validates structural recursion/depth using a complete canonical traversal
/// configuration.
///
/// This entry point exists for callers that need to combine recursion policy
/// with other operational traversal budgets.
///
/// It still delegates all traversal to the canonical walker.
///
/// Prefer [`validate_with_policy`] when only recursion/depth behavior needs to
/// be configured.
pub fn validate_with_walk_config<P>(
    provider: &P,
    root: NodeId,
    config: WalkConfig,
) -> Result<RecursionValidation, RecursionValidationError<P::Error>>
where
    P: AstWalkProvider,
{
    let mut visitor = NoopVisitor;

    let statistics = walk_with_config(
        provider,
        root,
        &mut visitor,
        config,
    )
    .map_err(|error| match error {
        WalkError::MissingNode { id } => {
            RecursionValidationError::MissingRoot { id }
        }

        other => RecursionValidationError::Walk(other),
    })?;

    Ok(RecursionValidation::from_walk(statistics))
}

// ============================================================================
// Policy conversion
// ============================================================================

/// Converts a recursion policy into the canonical traversal configuration.
///
/// This helper is useful to higher-level structural validation without forcing
/// that code to duplicate policy translation.
#[must_use]
pub const fn walk_config_for(
    policy: RecursionPolicy,
) -> WalkConfig {
    policy.to_walk_config()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_is_unrestricted() {
        let policy = RecursionPolicy::unrestricted();

        assert_eq!(policy.max_depth(), None);
        assert!(policy.rejects_revisits());

        let config = policy.to_walk_config();

        assert_eq!(config.max_nodes, None);
        assert_eq!(config.max_edges, None);
        assert_eq!(config.max_depth, None);
        assert!(config.reject_revisits);
    }

    #[test]
    fn explicit_depth_is_an_operational_policy() {
        let policy = RecursionPolicy::with_max_depth(128);

        assert_eq!(policy.max_depth(), Some(128));
        assert!(policy.rejects_revisits());

        let config = policy.to_walk_config();

        assert_eq!(config.max_depth, Some(128));
        assert_eq!(config.max_nodes, None);
        assert_eq!(config.max_edges, None);
    }

    #[test]
    fn dag_mode_disables_completed_revisit_rejection_only() {
        let policy = RecursionPolicy::unrestricted()
            .reject_revisits(false);

        assert!(!policy.rejects_revisits());

        let config = policy.to_walk_config();

        assert!(!config.reject_revisits);
        assert_eq!(config.max_depth, None);
    }

    #[test]
    fn empty_statistics_are_zero() {
        let statistics = RecursionValidation::empty();

        assert_eq!(statistics.nodes_visited, 0);
        assert_eq!(statistics.edges_visited, 0);
        assert_eq!(statistics.maximum_depth, 0);
    }

    #[test]
    fn walk_config_conversion_is_deterministic() {
        let policy = RecursionPolicy::with_max_depth(7)
            .reject_revisits(false);

        let first = policy.to_walk_config();
        let second = policy.to_walk_config();

        assert_eq!(first, second);
    }

    #[test]
    fn cycle_error_is_classified() {
        let error: RecursionValidationError<()> =
            RecursionValidationError::Walk(
                WalkError::CycleDetected {
                    id: NodeId::default(),
                },
            );

        assert!(error.is_cycle());
        assert!(!error.is_revisit());
        assert!(!error.is_depth_limit_exceeded());
    }

    #[test]
    fn revisit_error_is_classified() {
        let error: RecursionValidationError<()> =
            RecursionValidationError::Walk(
                WalkError::RevisitedNode {
                    id: NodeId::default(),
                },
            );

        assert!(error.is_revisit());
        assert!(!error.is_cycle());
    }

    #[test]
    fn depth_error_is_classified() {
        let error: RecursionValidationError<()> =
            RecursionValidationError::Walk(
                WalkError::DepthLimitExceeded {
                    depth: 9,
                    maximum: 8,
                },
            );

        assert!(error.is_depth_limit_exceeded());
        assert!(!error.is_cycle());
        assert!(!error.is_revisit());
    }

    #[test]
    fn node_limit_error_is_classified() {
        let error: RecursionValidationError<()> =
            RecursionValidationError::Walk(
                WalkError::NodeLimitExceeded {
                    visited: 10,
                    maximum: 9,
                },
            );

        assert!(error.is_node_limit_exceeded());
    }

    #[test]
    fn edge_limit_error_is_classified() {
        let error: RecursionValidationError<()> =
            RecursionValidationError::Walk(
                WalkError::EdgeLimitExceeded {
                    visited: 10,
                    maximum: 9,
                },
            );

        assert!(error.is_edge_limit_exceeded());
    }

    #[test]
    fn cancellation_is_classified() {
        let error: RecursionValidationError<()> =
            RecursionValidationError::Walk(
                WalkError::Cancelled,
            );

        assert!(error.is_cancelled());
    }

    #[test]
    fn missing_root_is_not_a_walk_cycle() {
        let error: RecursionValidationError<()> =
            RecursionValidationError::MissingRoot {
                id: NodeId::default(),
            };

        assert!(!error.is_cycle());
        assert!(!error.is_revisit());
        assert!(!error.is_depth_limit_exceeded());
    }
}