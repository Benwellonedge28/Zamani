//! # Zamani Frontend AST — Structural Validation
//!
//! `src/frontend/ast/node/validation/structural.rs`
//!
//! Canonical graph-level structural validation for the native Zamani AST.
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
//!      ├── NodeId
//!      ├── Node
//!      ├── source spans
//!      ├── metadata
//!      └── NodeId child relationships
//!      │
//!      ▼
//! ┌──────────────────────────────────────┐
//! │ structural.rs                        │
//! │                                      │
//! │ graph invariants                     │
//! │ node identity                       │
//! │ child references                    │
//! │ cycles / revisits                   │
//! │ configurable safety policy          │
//! │ source-structure invariants         │
//! └──────────────────┬───────────────────┘
//!                    │
//!                    ▼
//!             semantic analysis
//!                    │
//!                    ▼
//!              Semantic Model
//!                    │
//!                    ▼
//!                   ZUIR
//!                    │
//!          ┌─────────┼─────────┐
//!          ▼         ▼         ▼
//!      classical   quantum     HDL
//!         IR         IR        IR
//! ```
//!
//! ## Ownership
//!
//! This file owns **graph-level structural validation**.
//!
//! It validates:
//!
//! - that the validation root exists;
//! - that a provider returns the requested node identity;
//! - that every reachable node has a valid `NodeId`;
//! - that the node has a structurally valid source span representation;
//! - that node kinds are structurally admissible;
//! - that child references resolve;
//! - that cycles are rejected;
//! - that repeated nodes are rejected when tree semantics are requested;
//! - that optional parent/child source-span relationships are valid;
//! - that caller-supplied operational resource limits are respected;
//! - deterministic structural validation statistics.
//!
//! It deliberately does **not** validate:
//!
//! - names or symbol resolution;
//! - type correctness;
//! - generic substitution;
//! - overload resolution;
//! - effects semantics;
//! - capability satisfaction;
//! - quantum gate semantics;
//! - qubit counts;
//! - quantum topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - quantum error correction;
//! - resilience;
//! - backend compatibility;
//! - hardware availability;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - ZUIR semantics.
//!
//! Those belong to later compilation layers.
//!
//! ## Critical architectural boundary
//!
//! ```text
//! AST structural validity
//!          │
//!          ▼
//! semantic validity
//!          │
//!          ▼
//! computational validity
//!          │
//!          ▼
//! target validity
//!          │
//!          ▼
//! hardware realization
//! ```
//!
//! `structural.rs` must never collapse these stages.
//!
//! ## POCO-REAF
//!
//! Structural validation contains no machine-size assumptions.
//!
//! It does not define:
//!
//! - `MAX_QUBITS`;
//! - `MAX_REGISTERS`;
//! - `MAX_OPERATIONS`;
//! - `MAX_MACHINES`;
//! - `MAX_CORES`;
//! - `MAX_DEVICES`;
//! - `MAX_AST_NODES`;
//! - `MAX_PROGRAM_SIZE`;
//!
//! The same validator can therefore validate a tiny program or a structurally
//! enormous program, subject only to the resources and explicitly configured
//! operational safety policy supplied by its caller.
//!
//! "Infinity" in POCO-REAF means that this module introduces no artificial
//! language-level computational limit. Real execution remains bounded by the
//! resources available to the compiler process and the representational limits
//! of the host platform.
//!
//! ## Why validation uses the canonical walker
//!
//! The repository already has a canonical iterative AST walker. That walker
//! owns:
//!
//! - iterative graph traversal;
//! - cycle detection;
//! - revisit detection;
//! - missing-node detection;
//! - node identity mismatch detection;
//! - cancellation;
//! - configurable node/edge/depth limits;
//! - traversal statistics;
//! - provider error propagation.
//!
//! This module must not implement a second traversal algorithm.
//!
//! The dependency is therefore:
//!
//! ```text
//! structural.rs
//!       │
//!       ▼
//! traversal::walk
//!       │
//!       ▼
//! AstWalkProvider
//!       │
//!       ▼
//! AST storage
//! ```
//!
//! This keeps one authoritative graph traversal contract.
//!
//! ## Validation layers
//!
//! Structural validation is deliberately divided into three levels:
//!
//! ```text
//! Level 1 — local node invariants
//!     │
//!     ├── NodeId
//!     ├── NodeKind
//!     └── Span
//!
//! Level 2 — edge invariants
//!     │
//!     ├── child exists
//!     ├── child identity is canonical
//!     ├── optional parent/child span relationship
//!     └── duplicate/revisit policy
//!
//! Level 3 — graph invariants
//!     │
//!     ├── cycles
//!     ├── deterministic traversal
//!     └── operational resource policy
//! ```
//!
//! Semantic analysis begins only after these structural invariants hold.
//!
//! ## Generated and multi-source ASTs
//!
//! Source spans are locations rather than semantic identities. Consequently
//! this validator does not require every child span to be contained by its
//! parent by default.
//!
//! This is important for:
//!
//! - generated nodes;
//! - macro expansion;
//! - imported source fragments;
//! - desugaring;
//! - source-preserving transformations;
//! - future extension mechanisms.
//!
//! A caller that requires strict source-tree span containment can explicitly
//! select [`SpanValidationPolicy::SameSourceContained`].
//!
//! ## Tree versus DAG
//!
//! Native source ASTs are normally tree-shaped even though child relationships
//! are represented by `NodeId` references.
//!
//! The default policy therefore rejects revisits.
//!
//! A caller may explicitly allow completed-node sharing for a DAG-like
//! structural representation. Cycles are still always rejected.
//!
//! This distinction is important:
//!
//! ```text
//! repeated node in completed path = possibly shared DAG node
//! repeated node in active path     = cycle
//! ```
//!
//! Cycles are never accepted because they make a finite source AST structurally
//! unbounded.
//!
//! ## Determinism
//!
//! Determinism is inherited from the canonical walker and provider contract.
//!
//! The validator never:
//!
//! - sorts child IDs;
//! - sorts nodes;
//! - uses hash-map iteration order as source order;
//! - generates IDs;
//! - generates timestamps;
//! - consults hardware;
//! - consults runtime state;
//! - introduces randomness.
//!
//! ## Error model
//!
//! Errors are deliberately typed and compositional.
//!
//! Provider failures remain provider errors.
//!
//! Traversal failures remain traversal errors.
//!
//! Validation-specific failures are represented by [`StructuralValidationError`].
//!
//! No global compiler diagnostic type is imposed here.
//!
//! A later diagnostic layer may convert these errors into user-facing
//! diagnostics while retaining their structural identity.
//!
//! ## Safety
//!
//! This module contains no `unsafe` code and explicitly forbids it.
//!
//! Traversal is iterative through the canonical walker, so validation itself
//! does not recursively consume the Rust call stack for AST depth.
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
//! ## Integration contract
//!
//! ```text
//! parser
//!   │
//!   ▼
//! native AST
//!   │
//!   ▼
//! AST storage implementing AstWalkProvider
//!   │
//!   ▼
//! StructuralValidator::validate
//!   │
//!   ├── canonical traversal
//!   ├── local node checks
//!   ├── graph checks
//!   └── configured structural policies
//!   │
//!   ▼
//! valid AST
//!   │
//!   ▼
//! semantic analysis
//!   │
//!   ▼
//! SemanticModel
//!   │
//!   ▼
//! ZUIR
//! ```
//!
//! ## Integration guarantee
//!
//! Once this file is complete, adding:
//!
//! - a new expression;
//! - a new statement;
//! - a new declaration;
//! - a new type;
//! - a new resource;
//! - a new domain;
//! - a new quantum operation;
//! - a new quantum technology;
//! - a new hardware backend;
//! - a new computational domain;
//!
//! does not require changing this file merely because the construct exists.
//!
//! The new node only needs to participate in the existing `Node` and canonical
//! child-provider contracts.
//!
//! ============================================================================
//! Module configuration
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use super::super::node::Node;
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;
use super::traversal::walk::{
    walk_with_config,
    AstWalkProvider,
    WalkConfig,
    WalkError,
    WalkResult,
    WalkStatistics,
};
use super::visitors::visitor::{AstVisitor, VisitControl};

// ============================================================================
// Public configuration
// ============================================================================

/// Policy governing source-span relationships between parent and child nodes.
///
/// The native AST intentionally allows source fragments to come from different
/// source units because generated, imported, macro-expanded, and desugared
/// structures can legitimately have different provenance.
///
/// Therefore the default policy is [`SpanValidationPolicy::Permissive`].
///
/// Strict source-tree validation can be enabled explicitly.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SpanValidationPolicy {
    /// Validate only the intrinsic span representation supplied by `Span`.
    ///
    /// `Span::new` already guarantees that its start does not exceed its end.
    ///
    /// No parent/child source relationship is imposed.
    #[default]
    Permissive,

    /// Require each parent and direct child to originate from the same source
    /// and require the child span to be contained by the parent span.
    ///
    /// This is appropriate for a conventional source-derived tree.
    SameSourceContained,
}

impl SpanValidationPolicy {
    /// Returns whether parent/child source-span relationships should be
    /// validated.
    #[must_use]
    pub const fn validates_relationships(self) -> bool {
        matches!(self, Self::SameSourceContained)
    }
}

/// Configuration for structural AST validation.
///
/// All resource limits are operational policies only. They are never language
/// semantics and never represent machine or quantum limits.
///
/// The default configuration is deliberately unrestricted with respect to
/// node count, edge count, and depth.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StructuralValidationConfig {
    /// Canonical traversal policy.
    ///
    /// This owns optional node/edge/depth budgets and revisit behavior.
    pub traversal: WalkConfig,

    /// Policy for parent/child source-span relationships.
    pub span_policy: SpanValidationPolicy,

    /// Whether the supplied root must be a native `Program` node.
    ///
    /// This is disabled by default because structural validation is also useful
    /// for validating AST fragments and generated subtrees.
    pub require_program_root: bool,
}

impl Default for StructuralValidationConfig {
    fn default() -> Self {
        Self {
            traversal: WalkConfig::unrestricted(),
            span_policy: SpanValidationPolicy::Permissive,
            require_program_root: false,
        }
    }
}

impl StructuralValidationConfig {
    /// Creates the default production configuration.
    ///
    /// It introduces no artificial AST size or depth limit.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            traversal: WalkConfig {
                max_nodes: None,
                max_edges: None,
                max_depth: None,
                reject_revisits: true,
            },
            span_policy: SpanValidationPolicy::Permissive,
            require_program_root: false,
        }
    }

    /// Returns an unrestricted tree-validation configuration.
    ///
    /// "Unrestricted" means no artificial node, edge, or depth budget is
    /// introduced by this validator.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self::new()
    }

    /// Requires the validation root to be a `Program` node.
    #[must_use]
    pub const fn requiring_program_root(mut self, required: bool) -> Self {
        self.require_program_root = required;
        self
    }

    /// Configures parent/child span validation.
    #[must_use]
    pub const fn with_span_policy(
        mut self,
        policy: SpanValidationPolicy,
    ) -> Self {
        self.span_policy = policy;
        self
    }

    /// Sets an operational maximum number of entered nodes.
    ///
    /// This does not alter language semantics.
    #[must_use]
    pub const fn with_max_nodes(mut self, maximum: usize) -> Self {
        self.traversal.max_nodes = Some(maximum);
        self
    }

    /// Sets an operational maximum number of inspected child references.
    ///
    /// This does not alter language semantics.
    #[must_use]
    pub const fn with_max_edges(mut self, maximum: usize) -> Self {
        self.traversal.max_edges = Some(maximum);
        self
    }

    /// Sets an operational maximum logical traversal depth.
    ///
    /// This does not alter language semantics.
    #[must_use]
    pub const fn with_max_depth(mut self, maximum: usize) -> Self {
        self.traversal.max_depth = Some(maximum);
        self
    }

    /// Enables or disables completed-node revisit rejection.
    ///
    /// Cycles remain rejected regardless of this setting.
    #[must_use]
    pub const fn reject_revisits(mut self, enabled: bool) -> Self {
        self.traversal.reject_revisits = enabled;
        self
    }
}

// ============================================================================
// Validation statistics
// ============================================================================

/// Statistics produced by a successful structural validation.
///
/// These values are measurements, not language limits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct StructuralValidationStatistics {
    /// Number of AST nodes entered.
    pub nodes_validated: usize,

    /// Number of child references inspected.
    pub edges_validated: usize,

    /// Maximum logical AST depth observed.
    pub maximum_depth: usize,
}

impl StructuralValidationStatistics {
    /// Creates statistics containing no observations.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            nodes_validated: 0,
            edges_validated: 0,
            maximum_depth: 0,
        }
    }

    /// Creates validation statistics from canonical traversal statistics.
    #[must_use]
    pub const fn from_walk(
        statistics: WalkStatistics,
    ) -> Self {
        Self {
            nodes_validated: statistics.visited_nodes,
            edges_validated: statistics.visited_edges,
            maximum_depth: statistics.maximum_depth,
        }
    }
}

// ============================================================================
// Validation errors
// ============================================================================

/// Structural validation failure.
///
/// Provider and traversal failures are preserved rather than flattened into
/// strings so callers can make precise decisions about recovery, diagnostics,
/// logging, and compiler policy.
#[derive(Debug)]
#[non_exhaustive]
pub enum StructuralValidationError<PE> {
    /// The root node was required but could not be resolved.
    MissingRoot {
        /// Root identity supplied to the validator.
        id: NodeId,
    },

    /// The root node exists but is not a native program node when program-root
    /// validation was explicitly requested.
    InvalidRootKind {
        /// Root identity.
        id: NodeId,

        /// Actual root kind.
        actual: NodeKind,
    },

    /// A provider returned a node with an invalid identity.
    ///
    /// `NodeId` itself guarantees that ordinary construction cannot represent
    /// zero, but this error remains part of the validator contract so corrupted
    /// or future storage implementations cannot silently pass an identity
    /// mismatch.
    InvalidNodeId {
        /// Identity requested from storage.
        requested: NodeId,

        /// Identity returned by storage.
        returned: NodeId,
    },

    /// A child span violates the configured strict source-tree policy.
    InvalidChildSpan {
        /// Parent node identity.
        parent: NodeId,

        /// Parent span.
        parent_span: Span,

        /// Child node identity.
        child: NodeId,

        /// Child span.
        child_span: Span,
    },

    /// A direct child reference points back to its parent.
    ///
    /// This is reported explicitly in addition to the canonical walker's cycle
    /// protection because it is a particularly useful structural diagnostic.
    SelfReference {
        /// Parent/child node identity.
        id: NodeId,
    },

    /// A provider/storage failure occurred.
    Provider(PE),

    /// The canonical walker detected a structural traversal failure.
    ///
    /// This includes:
    ///
    /// - missing nodes;
    /// - identity mismatches;
    /// - cycles;
    /// - revisits;
    /// - configured node/edge/depth limits;
    /// - cancellation.
    Walk(WalkError<PE, StructuralValidationVisitorError>),
}

impl<PE: fmt::Display> fmt::Display for StructuralValidationError<PE> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::MissingRoot { id } => {
                write!(formatter, "AST validation root {id:?} does not exist")
            }

            Self::InvalidRootKind { id, actual } => {
                write!(
                    formatter,
                    "AST validation root {id:?} has kind {actual}, expected zamani:program"
                )
            }

            Self::InvalidNodeId {
                requested,
                returned,
            } => {
                write!(
                    formatter,
                    "AST storage returned node {returned:?} for requested node {requested:?}"
                )
            }

            Self::InvalidChildSpan {
                parent,
                parent_span,
                child,
                child_span,
            } => {
                write!(
                    formatter,
                    "AST child {child:?} span {child_span} is not contained by parent {parent:?} span {parent_span}"
                )
            }

            Self::SelfReference { id } => {
                write!(
                    formatter,
                    "AST node {id:?} directly references itself"
                )
            }

            Self::Provider(error) => {
                write!(
                    formatter,
                    "AST provider failed during structural validation: {error}"
                )
            }

            Self::Walk(error) => {
                write!(
                    formatter,
                    "AST structural traversal failed: {error}"
                )
            }
        }
    }
}

impl<PE> std::error::Error for StructuralValidationError<PE>
where
    PE: std::error::Error + 'static,
{
}

// ============================================================================
// Visitor error
// ============================================================================

/// Internal error used by the validation visitor.
///
/// The canonical walker requires visitors to own their error type. This
/// deliberately contains only structural validation failures and does not
/// depend on semantic/compiler layers.
#[derive(Debug)]
pub enum StructuralValidationVisitorError {
    /// The node's identity did not match the requested graph identity.
    InvalidNodeId {
        /// Identity supplied by the traversal.
        id: NodeId,
    },

    /// A node is not permitted to contain the supplied child according to the
    /// configured source-span policy.
    InvalidChildSpan {
        /// Parent identity.
        parent: NodeId,

        /// Parent span.
        parent_span: Span,

        /// Child identity.
        child: NodeId,

        /// Child span.
        child_span: Span,
    },

    /// A node directly references itself.
    SelfReference {
        /// Node identity.
        id: NodeId,
    },
}

impl fmt::Display for StructuralValidationVisitorError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidNodeId { id } => {
                write!(formatter, "invalid AST node identity {id:?}")
            }

            Self::InvalidChildSpan {
                parent,
                parent_span,
                child,
                child_span,
            } => {
                write!(
                    formatter,
                    "child {child:?} span {child_span} is invalid for parent {parent:?} span {parent_span}"
                )
            }

            Self::SelfReference { id } => {
                write!(
                    formatter,
                    "AST node {id:?} directly references itself"
                )
            }
        }
    }
}

impl std::error::Error for StructuralValidationVisitorError {}

// ============================================================================
// Validation visitor
// ============================================================================

/// Internal visitor used by [`StructuralValidator`].
///
/// The visitor deliberately contains only structural state.
///
/// It does not resolve symbols, types, resources, domains, quantum operations,
/// hardware, or semantic information.
struct StructuralValidationVisitor {
    span_policy: SpanValidationPolicy,
    active: Vec<(NodeId, Span)>,
}

impl StructuralValidationVisitor {
    /// Creates a validation visitor.
    #[must_use]
    fn new(span_policy: SpanValidationPolicy) -> Self {
        Self {
            span_policy,
            active: Vec::new(),
        }
    }

    /// Validates one node's local invariants.
    fn validate_node(
        &self,
        node: &Node,
    ) -> Result<(), StructuralValidationVisitorError> {
        // `NodeId` is opaque and guaranteed non-zero by its foundational type.
        // The explicit check remains useful as a defense against future
        // representation changes.
        if node.id().get() == 0 {
            return Err(
                StructuralValidationVisitorError::InvalidNodeId {
                    id: node.id(),
                },
            );
        }

        // `Span::new` guarantees start <= end. We intentionally do not
        // duplicate private span representation checks here.
        //
        // The validator also deliberately does not require a non-empty span:
        // EOF, insertion points, generated constructs, and recovery nodes may
        // legitimately use zero-width spans.

        // `NodeKind` is an opaque, canonical enum/extension representation.
        // The validator does not interpret extension semantics.
        //
        // This is deliberate: structural validation must not become a registry
        // of every future language/domain construct.

        Ok(())
    }

    /// Validates the relationship between an active parent and its child.
    fn validate_child_relationship(
        &self,
        parent_id: NodeId,
        parent_span: Span,
        child: &Node,
    ) -> Result<(), StructuralValidationVisitorError> {
        if child.id() == parent_id {
            return Err(
                StructuralValidationVisitorError::SelfReference {
                    id: parent_id,
                },
            );
        }

        if matches!(
            self.span_policy,
            SpanValidationPolicy::SameSourceContained
        ) && !parent_span.contains(child.span_owned())
        {
            return Err(
                StructuralValidationVisitorError::InvalidChildSpan {
                    parent: parent_id,
                    parent_span,
                    child: child.id(),
                    child_span: child.span_owned(),
                },
            );
        }

        Ok(())
    }

    /// Returns the current active parent.
    #[must_use]
    fn parent(&self) -> Option<(NodeId, Span)> {
        self.active.last().copied()
    }
}

impl AstVisitor for StructuralValidationVisitor {
    type Error = StructuralValidationVisitorError;

    fn enter(
        &mut self,
        node: &Node,
        _depth: usize,
    ) -> Result<VisitControl, Self::Error> {
        self.validate_node(node)?;

        if let Some((parent_id, parent_span)) = self.parent() {
            self.validate_child_relationship(
                parent_id,
                parent_span,
                node,
            )?;
        }

        self.active.push((node.id(), node.span_owned()));

        Ok(VisitControl::Continue)
    }

    fn exit(
        &mut self,
        node: &Node,
        _depth: usize,
    ) -> Result<(), Self::Error> {
        let active = self.active.pop();

        match active {
            Some((id, _)) if id == node.id() => Ok(()),

            Some((id, _)) => Err(
                StructuralValidationVisitorError::InvalidNodeId {
                    id,
                },
            ),

            None => Err(
                StructuralValidationVisitorError::InvalidNodeId {
                    id: node.id(),
                },
            ),
        }
    }
}

// ============================================================================
// Validator
// ============================================================================

/// Canonical graph-level structural AST validator.
///
/// `StructuralValidator` is stateless with respect to the AST itself. All
/// mutable traversal state lives in the canonical traversal engine and in a
/// short-lived validation visitor.
///
/// This makes the validator reusable across:
//!
//! - compiler sessions;
//! - parser instances;
//! - AST fragments;
//! - generated ASTs;
//! - tests;
//! - parallel read-only compilation tasks.
//!
//! No global mutable state is used.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StructuralValidator;

impl StructuralValidator {
    /// Creates the canonical stateless validator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Validates an AST root using the default structural policy.
    ///
    /// The default policy:
    ///
    /// - imposes no artificial node limit;
    /// - imposes no artificial edge limit;
    /// - imposes no artificial depth limit;
    /// - rejects cycles;
    /// - rejects repeated nodes;
    /// - allows zero-width spans;
    /// - permits generated/multi-source child spans;
    /// - does not require the root to be a `Program`.
    ///
    /// # Integration
    ///
    /// ```text
    /// AST storage
    ///     │
    ///     └── AstWalkProvider
    ///              │
    ///              ▼
    /// StructuralValidator::validate
    ///              │
    ///              ▼
    /// structural AST validity
    /// ```
    pub fn validate<P>(
        &self,
        provider: &P,
        root: NodeId,
    ) -> Result<StructuralValidationStatistics, StructuralValidationError<P::Error>>
    where
        P: AstWalkProvider,
        P::Error: fmt::Display,
    {
        self.validate_with_config(
            provider,
            root,
            StructuralValidationConfig::default(),
        )
    }

    /// Validates an AST root with an explicit structural policy.
    ///
    /// This is the primary production entry point.
    ///
    /// The validator delegates graph traversal to the canonical iterative
    /// walker, ensuring that structural validation and all other traversal
    /// consumers share exactly one cycle/revisit/limit implementation.
    pub fn validate_with_config<P>(
        &self,
        provider: &P,
        root: NodeId,
        config: StructuralValidationConfig,
    ) -> Result<StructuralValidationStatistics, StructuralValidationError<P::Error>>
    where
        P: AstWalkProvider,
        P::Error: fmt::Display,
    {
        let root_node = provider
            .node(root)
            .ok_or(StructuralValidationError::MissingRoot {
                id: root,
            })?;

        if root_node.id() != root {
            return Err(
                StructuralValidationError::InvalidNodeId {
                    requested: root,
                    returned: root_node.id(),
                },
            );
        }

        if config.require_program_root
            && root_node.kind().as_core()
                != Some(CoreNodeKind::Program)
        {
            return Err(
                StructuralValidationError::InvalidRootKind {
                    id: root,
                    actual: root_node.kind().clone(),
                },
            );
        }

        let mut visitor =
            StructuralValidationVisitor::new(config.span_policy);

        let result: WalkResult<
            WalkStatistics,
            P::Error,
            StructuralValidationVisitorError,
        > = walk_with_config(
            provider,
            root,
            &mut visitor,
            config.traversal,
        );

        match result {
            Ok(statistics) => {
                if !visitor.active.is_empty() {
                    return Err(
                        StructuralValidationError::Walk(
                            WalkError::Visitor(
                                StructuralValidationVisitorError::InvalidNodeId {
                                    id: visitor.active[0].0,
                                },
                            ),
                        ),
                    );
                }

                Ok(
                    StructuralValidationStatistics::from_walk(
                        statistics,
                    ),
                )
            }

            Err(WalkError::Provider(error)) => {
                Err(StructuralValidationError::Provider(error))
            }

            Err(WalkError::Visitor(error)) => {
                match error {
                    StructuralValidationVisitorError::InvalidNodeId {
                        id,
                    } => Err(
                        StructuralValidationError::InvalidNodeId {
                            requested: id,
                            returned: id,
                        },
                    ),

                    StructuralValidationVisitorError::InvalidChildSpan {
                        parent,
                        parent_span,
                        child,
                        child_span,
                    } => Err(
                        StructuralValidationError::InvalidChildSpan {
                            parent,
                            parent_span,
                            child,
                            child_span,
                        },
                    ),

                    StructuralValidationVisitorError::SelfReference {
                        id,
                    } => Err(
                        StructuralValidationError::SelfReference {
                            id,
                        },
                    ),
                }
            }

            Err(error) => Err(StructuralValidationError::Walk(error)),
        }
    }

    /// Validates that a root is structurally suitable for the complete native
    /// Zamani program AST.
    ///
    /// This is equivalent to default validation with
    /// `require_program_root = true`.
    pub fn validate_program<P>(
        &self,
        provider: &P,
        root: NodeId,
    ) -> Result<StructuralValidationStatistics, StructuralValidationError<P::Error>>
    where
        P: AstWalkProvider,
        P::Error: fmt::Display,
    {
        self.validate_with_config(
            provider,
            root,
            StructuralValidationConfig::default()
                .requiring_program_root(true),
        )
    }

    /// Validates an AST fragment rather than requiring a program root.
    ///
    /// This method exists primarily to make fragment validation explicit in
    /// parser recovery, generated-node, macro, and transformation pipelines.
    pub fn validate_fragment<P>(
        &self,
        provider: &P,
        root: NodeId,
    ) -> Result<StructuralValidationStatistics, StructuralValidationError<P::Error>>
    where
        P: AstWalkProvider,
        P::Error: fmt::Display,
    {
        self.validate(provider, root)
    }
}

// ============================================================================
// Convenience functions
// ============================================================================

/// Validates an AST root using the canonical default policy.
///
/// This is the preferred convenience function for compiler code that does not
/// need custom operational limits or strict source-span relationships.
pub fn validate<P>(
    provider: &P,
    root: NodeId,
) -> Result<StructuralValidationStatistics, StructuralValidationError<P::Error>>
where
    P: AstWalkProvider,
    P::Error: fmt::Display,
{
    StructuralValidator::new().validate(provider, root)
}

/// Validates an AST root using an explicit structural policy.
pub fn validate_with_config<P>(
    provider: &P,
    root: NodeId,
    config: StructuralValidationConfig,
) -> Result<StructuralValidationStatistics, StructuralValidationError<P::Error>>
where
    P: AstWalkProvider,
    P::Error: fmt::Display,
{
    StructuralValidator::new().validate_with_config(
        provider,
        root,
        config,
    )
}

/// Validates that a root is a complete native Zamani `Program` and that its
/// reachable structure satisfies the default structural invariants.
pub fn validate_program<P>(
    provider: &P,
    root: NodeId,
) -> Result<StructuralValidationStatistics, StructuralValidationError<P::Error>>
where
    P: AstWalkProvider,
    P::Error: fmt::Display,
{
    StructuralValidator::new().validate_program(provider, root)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_unrestricted() {
        let config = StructuralValidationConfig::default();

        assert_eq!(config.traversal.max_nodes, None);
        assert_eq!(config.traversal.max_edges, None);
        assert_eq!(config.traversal.max_depth, None);
        assert!(config.traversal.reject_revisits);
        assert_eq!(
            config.span_policy,
            SpanValidationPolicy::Permissive
        );
        assert!(!config.require_program_root);
    }

    #[test]
    fn unrestricted_configuration_has_no_language_level_limits() {
        let config = StructuralValidationConfig::unrestricted();

        assert_eq!(config.traversal.max_nodes, None);
        assert_eq!(config.traversal.max_edges, None);
        assert_eq!(config.traversal.max_depth, None);
    }

    #[test]
    fn strict_span_policy_is_explicit() {
        let config =
            StructuralValidationConfig::default()
                .with_span_policy(
                    SpanValidationPolicy::SameSourceContained,
                );

        assert_eq!(
            config.span_policy,
            SpanValidationPolicy::SameSourceContained
        );
    }

    #[test]
    fn operational_limits_are_optional() {
        let config =
            StructuralValidationConfig::default()
                .with_max_nodes(1024)
                .with_max_edges(4096)
                .with_max_depth(256);

        assert_eq!(config.traversal.max_nodes, Some(1024));
        assert_eq!(config.traversal.max_edges, Some(4096));
        assert_eq!(config.traversal.max_depth, Some(256));
    }

    #[test]
    fn program_root_requirement_is_explicit() {
        let config =
            StructuralValidationConfig::default()
                .requiring_program_root(true);

        assert!(config.require_program_root);
    }

    #[test]
    fn span_policy_default_allows_generated_cross_source_relationships() {
        assert_eq!(
            SpanValidationPolicy::default(),
            SpanValidationPolicy::Permissive
        );

        assert!(
            !SpanValidationPolicy::default()
                .validates_relationships()
        );
    }

    #[test]
    fn span_policy_strict_mode_validates_relationships() {
        assert!(
            SpanValidationPolicy::SameSourceContained
                .validates_relationships()
        );
    }

    #[test]
    fn statistics_empty_is_zeroed() {
        let statistics =
            StructuralValidationStatistics::empty();

        assert_eq!(statistics.nodes_validated, 0);
        assert_eq!(statistics.edges_validated, 0);
        assert_eq!(statistics.maximum_depth, 0);
    }

    #[test]
    fn visitor_starts_without_active_nodes() {
        let visitor =
            StructuralValidationVisitor::new(
                SpanValidationPolicy::Permissive,
            );

        assert!(visitor.active.is_empty());
        assert!(visitor.parent().is_none());
    }

    #[test]
    fn self_reference_is_structurally_rejected() {
        let visitor =
            StructuralValidationVisitor::new(
                SpanValidationPolicy::Permissive,
            );

        let id = NodeId::first();

        let span = Span::default();

        let result =
            visitor.validate_child_relationship(
                id,
                span,
                &Node::without_metadata(
                    id,
                    NodeKind::core(CoreNodeKind::Program),
                    span,
                ),
            );

        assert!(matches!(
            result,
            Err(
                StructuralValidationVisitorError::SelfReference {
                    id: returned
                }
            ) if returned == id
        ));
    }
}