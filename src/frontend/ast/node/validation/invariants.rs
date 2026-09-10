//! # Zamani Frontend AST — Structural Invariants
//!
//! `src/frontend/ast/node/validation/invariants.rs`
//!
//! Production-grade, domain-neutral structural invariants for the native
//! Zamani frontend AST.
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
//! native Zamani AST
//!      │
//!      ├── NodeId
//!      ├── NodeKind
//!      ├── Span
//!      ├── NodeMetadata
//!      └── structural child relationships
//!      │
//!      ▼
//! structural invariants  ← this module
//!      │
//!      ▼
//! canonical AST traversal
//!      │
//!      ▼
//! semantic analysis
//!      │
//!      ▼
//! semantic model
//!      │
//!      ▼
//! ZUIR
//! ```
//!
//! ## Purpose
//!
//! This module owns the smallest reusable set of invariants that can be
//! established from AST structural objects themselves.
//!
//! It answers questions such as:
//!
//! - Is an AST node identity valid?
//! - Does a child reference identify a valid AST node?
//! - Is a node reference accidentally self-referential?
//! - Is a `NodeKind` structurally valid?
//! - Is an extension kind represented through the canonical extension
//!   mechanism?
//! - Are structural collections free from impossible references?
//! - Can independently constructed AST fragments be checked before entering
//!   the canonical traversal/validation boundary?
//!
//! It deliberately does **not** answer:
//!
//! - Does an identifier resolve?
//! - Is an expression type-correct?
//! - Is a generic substitution valid?
//! - Is a quantum operation mathematically valid?
//! - Is a quantum operation supported by hardware?
//! - Are two qubits connected?
//! - Can routing be performed?
//! - Can an operation be scheduled?
//! - Is a pulse calibrated?
//! - Is a QEC code valid?
//! - Can a backend execute the program?
//! - Is a ZUIR lowering legal?
//!
//! Those questions belong to later compiler layers.
//!
//! ## Critical architectural rule
//!
//! This module must remain independent of:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - QIR;
//! - MLIR;
//! - LLVM;
//! - hardware;
//! - backend APIs;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime execution;
//! - vendor SDKs.
//!
//! ## POCO-REAF
//!
//! Nothing in this file assumes:
//!
//! - a maximum number of AST nodes;
//! - a maximum number of qubits;
//! - a maximum register size;
//! - a maximum number of operations;
//! - a machine topology;
//! - a processor architecture;
//! - a vendor;
//! - a backend;
//! - a quantum technology;
//! - a computational domain.
//!
//! The AST can therefore represent source programs from the smallest
//! computational resource to arbitrarily large resources permitted by the
//! compilation environment.
//!
//! Any operational node/edge/depth limits are owned by the traversal or
//! compiler-policy layer and are never language semantics.
//!
//! ## Relationship with `children.rs`
//!
//! Direct-child enumeration belongs to:
//!
//! `super::super::traversal::children`
//!
//! This module does not introduce another child-provider abstraction.
//!
//! ## Relationship with `walk.rs`
//!
//! Iterative traversal, cycle detection, revisit detection, cancellation and
//! operational traversal limits belong to the canonical walker.
//!
//! This module does not duplicate them.
//!
//! ## Relationship with `structural.rs`
//!
//! `structural.rs` owns structural validation orchestration.
//!
//! This module owns reusable local invariants which `structural.rs` and other
//! structural consumers may invoke.
//!
//! Conceptually:
//!
//! ```text
//! invariants.rs
//!     │
//!     ├── local node invariants
//!     ├── identity invariants
//!     ├── reference invariants
//!     └── kind invariants
//!             │
//!             ▼
//! structural.rs
//!     │
//!     └── complete AST structural validation
//! ```
//!
//! ## No recursive validation
//!
//! This module does not recursively walk an AST.
//!
//! Recursive graph validation belongs to the canonical iterative traversal
//! infrastructure. Keeping this module local prevents stack growth and avoids
//! creating a second traversal implementation.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! ## File completion contract
//!
//! Once this file is complete, adding:
//!
//! - a new declaration;
//! - a new expression;
//! - a new statement;
//! - a new type;
//! - a new resource;
//! - a new quantum operation;
//! - a new quantum technology;
//! - a new hardware backend;
//! - a new computational domain;
//!
//! must not require changing this file merely because that construct exists.
//!
//! A change is required only if the fundamental structural identity/reference
//! contract itself changes.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use super::super::node::Node;
use super::super::node_id::NodeId;
use super::super::node_kind::NodeKind;

// =============================================================================
// Public error type
// =============================================================================

/// Structural invariant failure.
///
/// This error type contains only AST-layer failures. It deliberately does not
/// expose semantic, quantum, hardware, scheduling, routing or backend errors.
///
/// The enum is `non_exhaustive` so additional structural diagnostics can be
/// introduced without making downstream exhaustive matches source-incompatible.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum InvariantError {
    /// A node identity is invalid.
    ///
    /// `NodeId` reserves zero and guarantees that normally constructed IDs are
    /// non-zero. This check remains useful at trust boundaries such as
    /// deserialization or foreign AST construction.
    InvalidNodeId {
        /// Node identity being validated.
        id: NodeId,
    },

    /// A child reference points to the same node as its parent.
    ///
    /// Self edges are never useful in the native source AST and would create an
    /// immediate structural cycle.
    SelfChild {
        /// Parent node.
        parent: NodeId,

        /// Child node.
        child: NodeId,
    },

    /// A child reference does not resolve in the supplied AST storage.
    MissingChild {
        /// Parent node containing the reference.
        parent: NodeId,

        /// Referenced child identity.
        child: NodeId,
    },

    /// The storage/provider returned a node whose identity differs from the
    /// identity requested by the caller.
    NodeIdentityMismatch {
        /// Requested identity.
        requested: NodeId,

        /// Identity returned by storage.
        returned: NodeId,
    },

    /// A node's kind cannot be accepted by the AST structural boundary.
    InvalidNodeKind {
        /// Node identity.
        node: NodeId,

        /// Human-readable reason.
        reason: &'static str,
    },

    /// An extension kind violates the structural extension contract.
    InvalidExtensionKind {
        /// Node identity.
        node: NodeId,

        /// Extension namespace.
        namespace: String,

        /// Human-readable reason.
        reason: &'static str,
    },
}

impl fmt::Display for InvariantError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeId { id } => {
                write!(formatter, "invalid AST node identity {id}")
            }

            Self::SelfChild { parent, child } => {
                write!(
                    formatter,
                    "AST node {parent} cannot contain itself as child {child}"
                )
            }

            Self::MissingChild { parent, child } => {
                write!(
                    formatter,
                    "AST node {parent} references missing child {child}"
                )
            }

            Self::NodeIdentityMismatch {
                requested,
                returned,
            } => {
                write!(
                    formatter,
                    "AST storage returned node {returned} for requested node {requested}"
                )
            }

            Self::InvalidNodeKind { node, reason } => {
                write!(
                    formatter,
                    "AST node {node} has an invalid node kind: {reason}"
                )
            }

            Self::InvalidExtensionKind {
                node,
                namespace,
                reason,
            } => {
                write!(
                    formatter,
                    "AST node {node} has invalid extension namespace \
                     `{namespace}`: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for InvariantError {}

// =============================================================================
// Node identity
// =============================================================================

/// Validates a single AST node identity.
///
/// `NodeId` is intentionally opaque. This function therefore checks only the
/// property that can be established at the AST boundary: the identity must
/// represent a valid allocated node.
///
/// The numeric value is never interpreted as:
///
/// - an array position;
/// - a memory address;
/// - a qubit;
/// - a resource;
/// - a backend identifier.
///
/// # Integration
///
/// This function can be called by:
///
/// - AST builders;
/// - deserializers;
/// - structural validation;
/// - AST transformations;
/// - foreign AST importers.
///
/// It does not allocate, mutate, traverse, or resolve anything.
#[inline]
pub fn validate_node_id(id: NodeId) -> Result<(), InvariantError> {
    //
    // NodeId::new() rejects zero, while a constructed NodeId cannot normally
    // contain zero. Keeping the check expressed through the canonical accessor
    // avoids relying on private representation details.
    //
    if id.get() == 0 {
        return Err(InvariantError::InvalidNodeId { id });
    }

    Ok(())
}

/// Returns whether a node identity is structurally valid.
///
/// This is the non-error convenience form for hot paths where callers already
/// have an error-reporting policy.
#[inline]
#[must_use]
pub fn is_valid_node_id(id: NodeId) -> bool {
    id.get() != 0
}

// =============================================================================
// Node identity consistency
// =============================================================================

/// Validates that the node returned for `requested` actually owns that identity.
///
/// This is a trust-boundary check for AST providers.
///
/// The storage implementation remains outside this module.
///
/// It prevents an incorrect provider from silently associating the wrong node
/// with a requested identity.
#[inline]
pub fn validate_node_identity(
    requested: NodeId,
    node: &Node,
) -> Result<(), InvariantError> {
    validate_node_id(requested)?;
    validate_node_id(node.id())?;

    if node.id() != requested {
        return Err(InvariantError::NodeIdentityMismatch {
            requested,
            returned: node.id(),
        });
    }

    Ok(())
}

// =============================================================================
// Canonical node validation
// =============================================================================

/// Validates the local structural invariants of a canonical [`Node`].
///
/// This function deliberately does not validate descendants.
///
/// Descendant relationships are checked by the complete structural validator
/// using the canonical traversal infrastructure.
///
/// # What is checked
///
/// - node identity;
/// - node kind classification;
/// - extension-kind structural identity.
///
/// # What is deliberately not checked
///
/// - semantic meaning;
/// - type information;
/// - source-file contents;
/// - symbol resolution;
/// - resource availability;
/// - quantum capabilities;
/// - hardware capabilities;
/// - backend support.
///
/// # Source spans
///
/// The canonical [`Span`] type owns source-coordinate representation. This
/// module does not duplicate span validation rules or attempt to interpret
/// source offsets independently of the source infrastructure.
///
/// That prevents multiple incompatible definitions of source-range validity.
pub fn validate_node(node: &Node) -> Result<(), InvariantError> {
    validate_node_id(node.id())?;
    validate_node_kind(node.id(), node.kind())?;

    Ok(())
}

// =============================================================================
// Node kind validation
// =============================================================================

/// Validates the structural form of a [`NodeKind`].
///
/// Core kinds are structurally valid by construction.
///
/// Extension kinds are checked through the public `NodeKind` API rather than
/// through private representation details.
///
/// Semantic extension registration is deliberately not performed here.
///
/// For example, this function does not ask whether:
///
/// ```text
/// quantum:some-operation
/// ```
///
/// is understood by a quantum compiler.
///
/// It only validates that the AST's extension identity is structurally
/// representable.
///
/// This distinction is essential for future computational domains.
pub fn validate_node_kind(
    node: NodeId,
    kind: &NodeKind,
) -> Result<(), InvariantError> {
    validate_node_id(node)?;

    if kind.is_core() {
        //
        // CoreNodeKind is an exhaustive structural value owned by node_kind.rs.
        // No semantic validation belongs here.
        //
        return Ok(());
    }

    if let Some(extension) = kind.as_extension() {
        let namespace = extension.namespace();

        //
        // The extension constructor is the canonical authority for extension
        // identifier validity. A structurally constructed/decoded value still
        // receives a defensive boundary check here.
        //
        if namespace.is_empty() {
            return Err(InvariantError::InvalidExtensionKind {
                node,
                namespace: String::new(),
                reason: "extension namespace must not be empty",
            });
        }

        //
        // Namespace/name are source-level identities. We deliberately do not
        // impose a finite list of namespaces because that would make the AST
        // closed to future domains.
        //
        return Ok(());
    }

    //
    // `NodeKind` currently has only Core and Extension forms. The branch is
    // retained as a defensive failure boundary for future representation
    // changes and because this function must remain an explicit structural
    // trust boundary.
    //
    Err(InvariantError::InvalidNodeKind {
        node,
        reason: "unrecognized node-kind representation",
    })
}

// =============================================================================
// Parent/child identity
// =============================================================================

/// Validates a single parent/child identity relationship.
///
/// This function does not resolve the child from storage.
///
/// It therefore detects only relationships that can be determined from the two
/// identities themselves.
///
/// In particular, a self-reference is always invalid.
///
/// Missing-child validation requires an AST provider and is exposed separately
/// by [`validate_child_reference`].
#[inline]
pub fn validate_child_identity(
    parent: NodeId,
    child: NodeId,
) -> Result<(), InvariantError> {
    validate_node_id(parent)?;
    validate_node_id(child)?;

    if parent == child {
        return Err(InvariantError::SelfChild { parent, child });
    }

    Ok(())
}

/// Validates a child reference against a provider.
///
/// The provider is intentionally represented as a small closure rather than
/// introducing another storage trait here.
///
/// The canonical AST traversal layer already owns `AstChildrenProvider` and
/// node resolution. This function therefore accepts the minimal capability it
/// needs:
///
/// ```text
/// NodeId -> Option<&Node>
/// ```
///
/// This keeps the invariant layer storage-independent and prevents duplicate
/// provider abstractions.
///
/// # Example
///
/// ```ignore
/// validate_child_reference(parent, child, |id| store.node(id))?;
/// ```
pub fn validate_child_reference<'a, F>(
    parent: NodeId,
    child: NodeId,
    resolve: F,
) -> Result<(), InvariantError>
where
    F: FnOnce(NodeId) -> Option<&'a Node>,
{
    validate_child_identity(parent, child)?;

    let resolved = resolve(child).ok_or(InvariantError::MissingChild {
        parent,
        child,
    })?;

    validate_node_identity(child, resolved)?;

    Ok(())
}

// =============================================================================
// Optional child-reference helper
// =============================================================================

/// Validates a child reference when the caller already has the resolved node.
///
/// This is useful for traversal implementations that have already resolved the
/// child and want to avoid resolving it a second time.
///
/// No traversal is performed.
#[inline]
pub fn validate_resolved_child(
    parent: NodeId,
    child: &Node,
) -> Result<(), InvariantError> {
    validate_child_identity(parent, child.id())?;
    validate_node(child)?;
    Ok(())
}

// =============================================================================
// Node collection validation
// =============================================================================

/// Validates a sequence of child identities belonging to one parent.
///
/// This function intentionally does not enforce a maximum number of children.
///
/// A source construct may have an arbitrarily large number of structural
/// children subject only to the host process and explicit compiler resource
/// policies.
///
/// It also does not reject repeated child IDs.
///
/// Repeated-node policy is owned by the canonical traversal layer because a
/// repeated node may represent either:
///
/// - malformed tree structure; or
/// - intentionally shared DAG structure.
///
/// Cycles are likewise owned by the canonical iterative walker.
pub fn validate_child_identities<I>(
    parent: NodeId,
    children: I,
) -> Result<(), InvariantError>
where
    I: IntoIterator<Item = NodeId>,
{
    validate_node_id(parent)?;

    for child in children {
        validate_child_identity(parent, child)?;
    }

    Ok(())
}

// =============================================================================
// Node slice validation
// =============================================================================

/// Validates an already-resolved collection of direct child nodes.
///
/// This is deliberately local:
///
/// ```text
/// parent
///   ├── child
///   ├── child
///   └── child
/// ```
///
/// It does not recursively inspect descendants.
///
/// Recursion/cycle/revisit policy remains owned by traversal.
pub fn validate_resolved_children<'a, I>(
    parent: NodeId,
    children: I,
) -> Result<(), InvariantError>
where
    I: IntoIterator<Item = &'a Node>,
{
    validate_node_id(parent)?;

    for child in children {
        validate_resolved_child(parent, child)?;
    }

    Ok(())
}

// =============================================================================
// Root validation
// =============================================================================

/// Validates an AST root node.
///
/// A root is not required to have a particular semantic kind here.
///
/// This is intentional because:
///
/// - imported syntax may use a wrapper root;
/// - parser recovery may use a dedicated root;
/// - future source forms may introduce additional root structures.
///
/// Root-kind policy belongs to the program-level structural validator, not to
/// this low-level invariant module.
#[inline]
pub fn validate_root(node: &Node) -> Result<(), InvariantError> {
    validate_node(node)
}

// =============================================================================
// Structural invariant summary
// =============================================================================

/// Compact result of local invariant validation.
///
/// This is intentionally small and allocation-free so it can be used by
/// tooling, tests and incremental compiler phases.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InvariantSummary {
    /// Number of nodes checked.
    pub nodes_checked: usize,

    /// Number of child relationships checked.
    pub edges_checked: usize,
}

impl InvariantSummary {
    /// Creates an empty summary.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nodes_checked: 0,
            edges_checked: 0,
        }
    }

    /// Records one successfully checked node.
    ///
    /// Returns `None` only if the host `usize` representation is exhausted.
    /// This is a representation boundary, not a Zamani language limit.
    #[inline]
    pub fn record_node(&mut self) -> Option<()> {
        self.nodes_checked = self.nodes_checked.checked_add(1)?;
        Some(())
    }

    /// Records one successfully checked edge.
    ///
    /// Returns `None` only if the host `usize` representation is exhausted.
    #[inline]
    pub fn record_edge(&mut self) -> Option<()> {
        self.edges_checked = self.edges_checked.checked_add(1)?;
        Some(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_node_id_is_accepted() {
        let id = NodeId::new(1).expect("one is a valid AST node identity");

        assert!(validate_node_id(id).is_ok());
        assert!(is_valid_node_id(id));
    }

    #[test]
    fn zero_cannot_be_constructed_as_node_id() {
        assert!(NodeId::new(0).is_none());
    }

    #[test]
    fn self_child_is_rejected() {
        let id = NodeId::new(1).expect("valid node ID");

        let error = validate_child_identity(id, id)
            .expect_err("a node cannot be its own direct child");

        assert_eq!(
            error,
            InvariantError::SelfChild {
                parent: id,
                child: id,
            }
        );
    }

    #[test]
    fn different_child_identity_is_accepted() {
        let parent = NodeId::new(1).expect("valid parent");
        let child = NodeId::new(2).expect("valid child");

        assert!(validate_child_identity(parent, child).is_ok());
    }

    #[test]
    fn child_collection_does_not_impose_a_fixed_width() {
        let parent = NodeId::new(1).expect("valid parent");

        let children = (2_u64..=10_u64)
            .map(|value| NodeId::new(value).expect("generated ID"));

        assert!(validate_child_identities(parent, children).is_ok());
    }

    #[test]
    fn invariant_summary_is_checked() {
        let mut summary = InvariantSummary::new();

        assert!(summary.record_node().is_some());
        assert!(summary.record_edge().is_some());

        assert_eq!(summary.nodes_checked, 1);
        assert_eq!(summary.edges_checked, 1);
    }
}