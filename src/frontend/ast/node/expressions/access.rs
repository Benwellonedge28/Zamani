//! # Zamani Native AST — Member Access Expression
//!
//! Production-ready source-level representation of member/property access.
//!
//! ## Architectural position
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
//! Native Zamani AST
//!     │
//!     ├── MemberAccessExpression  ← this module
//!     │
//!     ▼
//! structural validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── classical IR
//!     ├── quantum IR
//!     ├── HDL IR
//!     └── future domain IRs
//! ```
//!
//! ## Purpose
//!
//! This module defines the canonical native AST representation of source-level
//! member/property access.
//!
//! Conceptually:
//!
//! ```text
//! receiver.member
//! ```
//!
//! The receiver is represented by [`NodeId`] and the accessed member is
//! represented by a source-level [`MemberName`].
//!
//! The node deliberately does **not** determine what the member means.
//! Semantic analysis may later determine that a member refers to:
//!
//! - a struct field;
//! - an object property;
//! - a module item;
//! - a namespace member;
//! - a method;
//! - a type-associated item;
//! - a resource property;
//! - a quantum-resource property;
//! - a classical value;
//! - an HDL signal;
//! - an extension-defined member;
//! - a future computational-domain construct.
//!
//! This module must not decide that meaning.
//!
//! ## POCO-REAF
//!
//! Member access is intentionally expressed in terms of source-level identity
//! and a child-node reference rather than hardware representation.
//!
//! Therefore the same AST structure can participate in programs targeting:
//!
//! - a tiny machine;
//! - a large machine;
//! - a simulator;
//! - a CPU;
//! - a GPU;
//! - an FPGA;
//! - an ASIC;
//! - a QPU;
//! - a distributed system;
//! - heterogeneous hardware;
//! - future computational systems.
//!
//! The AST does not encode machine size, register width, address width, qubit
//! count, topology, vendor, backend, instruction set, or execution strategy.
//!
//! ## Critical boundary
//!
//! This module is an AST node, not an IR.
//!
//! It must never contain:
//!
//! - resolved symbols;
//! - resolved types;
//! - SSA values;
//! - physical addresses;
//! - hardware registers;
//! - physical qubit IDs;
//! - logical-to-physical mappings;
//! - routing decisions;
//! - scheduling decisions;
//! - calibration data;
//! - QIR values;
//! - LLVM values;
//! - MLIR operations;
//! - backend handles;
//! - runtime state.
//!
//! Those belong to later compiler layers.
//!
//! ## Child representation
//!
//! The receiver is represented by [`NodeId`].
//!
//! This follows the canonical Zamani AST graph design. The node does not own
//! another concrete expression recursively.
//!
//! ```text
//! MemberAccessExpression
//! ├── Node
//! ├── receiver: NodeId
//! └── member: MemberName
//! ```
//!
//! The AST graph owns the referenced receiver node.
//!
//! This avoids recursive Rust ownership structures and allows traversal to be
//! implemented independently of this node.
//!
//! ## Member identity
//!
//! The member name is preserved as source-level text.
//!
//! The AST does not resolve it into a symbol ID because name resolution belongs
//! to semantic analysis.
//!
//! Consequently:
//!
//! ```text
//! object.field
//! ```
//!
//! stores the source-level `field` name rather than a semantic field identity.
//!
//! ## Qualified access
//!
//! Chained access is naturally represented without special cases:
//!
//! ```text
//! a.b.c
//! ```
//!
//! becomes conceptually:
//!
//! ```text
//! MemberAccess(
//!     receiver = MemberAccess(
//!         receiver = a,
//!         member = b
//!     ),
//!     member = c
//! )
//! ```
//!
//! No fixed access-chain length is imposed.
//!
//! ## Optional member access
//!
//! Optional/chained access syntax, if supported by the Zamani grammar, should
//! be represented by the appropriate source-language expression or extension
//! rather than by silently changing the semantics of ordinary member access.
//!
//! For example, a future syntax such as:
//!
//! ```text
//! value?.member
//! ```
//!
//! must not be interpreted here as ordinary `.` access.
//!
//! The parser/grammar should decide whether this is a distinct language
//! construct.
//!
//! ## Method calls
//!
//! A method invocation should normally remain compositional:
//!
//! ```text
//! object.method(argument)
//! ```
//!
//! becomes conceptually:
//!
//! ```text
//! Call(
//!     callee = MemberAccess(
//!         receiver = object,
//!         member = method
//!     ),
//!     arguments = [...]
//! )
//! ```
//!
//! `MemberAccessExpression` therefore does not need a special method-call
//! representation.
//!
//! This keeps the AST orthogonal and prevents method semantics from leaking
//! into member-access syntax.
//!
//! ## Quantum compatibility
//!
//! Quantum programs may use member access for source-level constructs such as:
//!
//! ```text
//! register.size
//! register[index]
//! resource.property
//! operation.metadata
//! ```
//!
//! The AST does not assume what those members represent.
//!
//! In particular, this module does **not** define:
//!
//! ```text
//! qubit.index
//! qubit.physical_id
//! qpu.topology
//! backend.name
//! ```
//!
//! as special AST concepts.
//!
//! If a quantum language extension introduces such semantics, semantic analysis
//! or the extension layer interprets them.
//!
//! ## Domain neutrality
//!
//! The same member-access node may be used for:
//!
//! ```text
//! classical values
//! quantum resources
//! hybrid resources
//! distributed objects
//! accelerator resources
//! HDL objects
//! future domain objects
//! ```
//!
//! without changing this file.
//!
//! ## Structural versus semantic validation
//!
//! This module performs only local structural validation.
//!
//! It may validate:
//!
//! - the node kind;
//! - receiver presence;
//! - member-name validity;
//! - configured structural limits.
//!
//! It must not validate:
//!
//! - whether the receiver actually has the member;
//! - whether the member is public/private;
//! - whether the member is a field or method;
//! - whether the member exists in a type;
//! - whether access is legal under ownership rules;
//! - whether the member refers to a quantum resource;
//! - whether the member is supported by hardware.
//!
//! Those checks belong downstream.
//!
//! ## Scalability
//!
//! There is no fixed:
//!
//! - number of access expressions;
//! - member-name count;
//! - machine size;
//! - resource count;
//! - qubit count;
//! - register width;
//! - access-chain length.
//!
//! A single access node has a constant number of direct children, while an
//! arbitrarily large program contains arbitrarily many access nodes in the AST
//! graph.
//!
//! Deep access chains are represented through AST node relationships and are
//! traversed by the external AST traversal infrastructure.
//!
//! This module never recursively walks the receiver.
//!
//! ## Determinism
//!
//! The node contains no hash-map-backed ordering.
//!
//! The receiver is a single deterministic [`NodeId`] and the member name is
//! deterministic source text.
//!
//! ## Serialization
//!
//! The node derives Serde serialization.
//!
//! Overall AST schema versioning belongs to the AST serialization subsystem.
//! This module exposes a local schema version solely to identify the structural
//! contract of this node.
//!
//! ## Security
//!
//! This module:
//!
//! - uses no `unsafe`;
//! - performs no I/O;
//! - executes no source code;
//! - dereferences no raw pointers;
//! - performs no unchecked indexing;
//! - performs no recursive traversal;
//! - does not allocate based on machine-specific assumptions;
//! - does not resolve external resources.
//!
//! Member names originate from compiler input and therefore remain untrusted
//! source data.
//!
//! Configurable identifier limits belong to compiler validation policy.
//!
//! ## Integration contract
//!
//! ### `node.rs`
//!
//! Provides [`Node`] containing:
//!
//! - [`NodeId`];
//! - [`NodeKind`];
//! - [`Span`];
//! - [`NodeMetadata`].
//!
//! ### `node_id.rs`
//!
//! Owns AST node identity.
//!
//! This file never creates IDs implicitly.
//!
//! ### `node_kind.rs`
//!
//! Uses:
//!
//! ```text
//! CoreNodeKind::MemberAccess
//! ```
//!
//! which is already part of the repository's native node-kind vocabulary.
//!
//! ### `expression.rsq` / expression aggregate
//!
//! The canonical expression aggregate should represent member access using
//! this node structure rather than duplicating receiver/member fields inline.
//!
//! Conceptually:
//!
//! ```text
//! ExpressionKind::MemberAccess(MemberAccessExpression)
//! ```
//!
//! The exact aggregate syntax must follow the current canonical expression
//! representation.
//!
//! ### parser
//!
//! The parser:
//!
//! 1. parses the receiver;
//! 2. parses the member identifier;
//! 3. allocates a `NodeId`;
//! 4. computes the complete source span;
//! 5. constructs this node.
//!
//! The parser does not resolve the member.
//!
//! ### structural validation
//!
//! The AST validation layer validates that the receiver ID exists in the AST
//! graph and invokes this node's local structural validation.
//!
//! ### visitors
//!
//! Visitor/traversal code visits:
//!
//! 1. the member-access node;
//! 2. the receiver node.
//!
//! The member name is scalar source data and is not an AST child.
//!
//! ### semantic analysis
//!
//! Semantic analysis resolves:
//!
//! - receiver type;
//! - member namespace;
//! - member symbol;
//! - access permissions;
//! - associated item resolution;
//! - overload/method resolution where applicable;
//! - generic substitutions;
//! - effects;
//! - capabilities;
//! - resource semantics.
//!
//! ### ZUIR
//!
//! ZUIR lowering consumes the semantic meaning of this expression.
//!
//! This module must not import ZUIR.
//!
//! ### quantum compiler
//!
//! Quantum lowering may interpret a resolved member as a quantum-resource
//! operation or property.
//!
//! This file remains unchanged.
//!
//! ### optimization
//!
//! Optimization belongs downstream.
//!
//! Constant folding, member resolution caching, layout decisions, resource
//! mapping and hardware-specific transformations must not be placed here.
//!
//! ## No-re-edit integration guarantee
//!
//! This file intentionally exposes a narrow source-level contract:
//!
//! ```text
//! Node
//! receiver NodeId
//! member MemberName
//! ```
//!
//! Downstream phases consume those values without requiring this node to know
//! about their internal representation.
//!
//! A change to semantic analysis, ZUIR, quantum IR, routing, scheduling,
//! hardware, or backend implementation therefore does not require reopening
//! this file.
//!
//! A change to the parser likewise does not require changing this file unless
//! the actual Zamani source-language grammar changes the semantics of member
//! access.
//!
//! =============================================================================
//! Implementation
//! =============================================================================
//
// Rust 1.97 / Rust 1.97.1
// Edition 2021
// No unsafe code.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::Node;
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Local structural schema version for member-access expressions.
///
/// This is deliberately independent of the Zamani language version and the
/// overall serialized AST schema version.
pub const MEMBER_ACCESS_SCHEMA_VERSION: u16 = 1;

/// Result type used by member-access construction and validation.
pub type MemberAccessResult<T> = Result<T, MemberAccessError>;

/// Source-level identifier used as the member name.
///
/// This type deliberately does not represent a resolved semantic symbol.
///
/// A member such as `length` remains `length` until semantic analysis resolves
/// it against the receiver's type/environment.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MemberName(String);

impl MemberName {
    /// Creates a member name from source text.
    ///
    /// Empty names are rejected because a member-access expression requires an
    /// actual source-level member identifier.
    pub fn new(value: impl Into<String>) -> MemberAccessResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(MemberAccessError::EmptyMemberName);
        }

        Ok(Self(value))
    }

    /// Returns the source-level member name.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the number of UTF-8 bytes in the member name.
    #[inline]
    pub fn len_bytes(&self) -> usize {
        self.0.len()
    }

    /// Returns whether the member name is empty.
    ///
    /// This is always false for values successfully constructed through
    /// [`MemberName::new`].
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Consumes the member name and returns its source text.
    #[inline]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for MemberName {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for MemberName {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Structural errors for [`MemberAccessExpression`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum MemberAccessError {
    /// The common node does not have the required native member-access kind.
    InvalidNodeKind {
        /// Actual node kind.
        actual: NodeKind,
    },

    /// The receiver reference is invalid.
    ///
    /// `NodeId` itself guarantees that zero is not a valid ID, but this error
    /// also gives the node a stable semantic place to report malformed input
    /// should the underlying identity contract evolve.
    InvalidReceiver,

    /// The source-level member name is empty.
    EmptyMemberName,

    /// A configured member-name byte limit was exceeded.
    MemberNameTooLong {
        /// Observed UTF-8 byte length.
        actual: usize,

        /// Maximum permitted by the caller's validation policy.
        maximum: usize,
    },
}

impl fmt::Display for MemberAccessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "member-access expression has invalid node kind: {actual}"
                )
            }

            Self::InvalidReceiver => {
                formatter.write_str("member-access expression has an invalid receiver")
            }

            Self::EmptyMemberName => {
                formatter.write_str("member-access expression has an empty member name")
            }

            Self::MemberNameTooLong { actual, maximum } => {
                write!(
                    formatter,
                    "member name exceeds configured limit: {actual} > {maximum} bytes"
                )
            }
        }
    }
}

impl std::error::Error for MemberAccessError {}

/// Configurable structural validation policy for member-access expressions.
///
/// These values are compiler/resource-policy controls. They are not language
/// limits and do not impose any limit on the number of expressions, members,
/// resources, qubits, registers, machines, or hardware devices in a program.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemberAccessValidationPolicy {
    /// Optional maximum UTF-8 byte length of a single member name.
    pub max_member_name_bytes: Option<usize>,
}

impl Default for MemberAccessValidationPolicy {
    fn default() -> Self {
        Self {
            max_member_name_bytes: None,
        }
    }
}

/// Canonical native Zamani member-access expression.
///
/// ```text
/// receiver.member
/// ```
///
/// The receiver is an AST child identified by [`NodeId`]. The member is
/// source-level text and is deliberately not resolved here.
///
/// # Invariants
///
/// A structurally valid value satisfies:
///
/// 1. `node.kind() == CoreNodeKind::MemberAccess`.
/// 2. `receiver` is a valid AST node identity.
/// 3. `member` is non-empty.
/// 4. `member` remains source-level text.
/// 5. No semantic or hardware state is embedded.
/// 6. The receiver is not recursively embedded.
/// 7. No fixed-size machine/resource assumptions exist.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MemberAccessExpression {
    /// Common source-level AST identity, kind, span and metadata.
    node: Node,

    /// AST node containing the value/namespace/object being accessed.
    receiver: NodeId,

    /// Source-level member identifier.
    member: MemberName,
}

impl MemberAccessExpression {
    /// Constructs a member-access expression.
    ///
    /// The caller supplies the node identity and source information because
    /// identity and source mapping belong to the AST construction layer.
    ///
    /// This constructor performs local structural validation only.
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        receiver: NodeId,
        member: MemberName,
    ) -> MemberAccessResult<Self> {
        if receiver == NodeId::default() {
            return Err(MemberAccessError::InvalidReceiver);
        }

        if member.is_empty() {
            return Err(MemberAccessError::EmptyMemberName);
        }

        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::MemberAccess),
            span,
            metadata,
        );

        Ok(Self {
            node,
            receiver,
            member,
        })
    }

    /// Constructs a member-access expression with default metadata.
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        receiver: NodeId,
        member: MemberName,
    ) -> MemberAccessResult<Self> {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            receiver,
            member,
        )
    }

    /// Returns the common AST node.
    #[inline]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to common AST metadata.
    ///
    /// Identity, kind and source structure remain controlled by `Node`.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the stable AST identity.
    #[inline]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the node kind.
    #[inline]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns the source span.
    #[inline]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns source metadata.
    #[inline]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable source metadata.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the receiver node ID.
    #[inline]
    pub fn receiver(&self) -> NodeId {
        self.receiver
    }

    /// Returns the source-level member name.
    #[inline]
    pub fn member(&self) -> &MemberName {
        &self.member
    }

    /// Returns the member name as source text.
    #[inline]
    pub fn member_name(&self) -> &str {
        self.member.as_str()
    }

    /// Returns the number of direct AST children.
    ///
    /// Member access has exactly one AST child: the receiver.
    ///
    /// The member name is scalar source data and is not itself an AST node.
    #[inline]
    pub const fn child_count(&self) -> usize {
        1
    }

    /// Returns the direct AST child IDs in deterministic source structure order.
    ///
    /// Member access has one child, the receiver.
    ///
    /// This method does not recursively walk that receiver.
    #[inline]
    pub fn child_node_ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        core::iter::once(self.receiver)
    }

    /// Validates local structural invariants.
    ///
    /// This deliberately does not verify whether the receiver actually has
    /// the requested member. That requires semantic type/name resolution.
    pub fn validate_structure(
        &self,
        policy: MemberAccessValidationPolicy,
    ) -> MemberAccessResult<()> {
        let expected = NodeKind::core(CoreNodeKind::MemberAccess);

        if self.kind() != &expected {
            return Err(MemberAccessError::InvalidNodeKind {
                actual: self.kind().clone(),
            });
        }

        if self.receiver == NodeId::default() {
            return Err(MemberAccessError::InvalidReceiver);
        }

        if self.member.is_empty() {
            return Err(MemberAccessError::EmptyMemberName);
        }

        if let Some(maximum) = policy.max_member_name_bytes {
            let actual = self.member.len_bytes();

            if actual > maximum {
                return Err(MemberAccessError::MemberNameTooLong {
                    actual,
                    maximum,
                });
            }
        }

        Ok(())
    }

    /// Returns the local schema version.
    #[inline]
    pub const fn schema_version() -> u16 {
        MEMBER_ACCESS_SCHEMA_VERSION
    }

    /// Returns a compact diagnostic representation.
    ///
    /// The member name is included because it is small source-level identity,
    /// while arbitrary metadata is intentionally excluded.
    pub fn diagnostic_summary(&self) -> MemberAccessDiagnosticSummary {
        MemberAccessDiagnosticSummary {
            id: self.id(),
            receiver: self.receiver,
            member: self.member.clone(),
            span: self.span().clone(),
        }
    }
}

/// Lightweight diagnostic representation of member access.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MemberAccessDiagnosticSummary {
    /// Member-access node ID.
    pub id: NodeId,

    /// Receiver node ID.
    pub receiver: NodeId,

    /// Source-level member name.
    pub member: MemberName,

    /// Source span.
    pub span: Span,
}

impl fmt::Display for MemberAccessDiagnosticSummary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "member access {}.{} at {}",
            self.receiver,
            self.member,
            self.span
        )
    }
}

/// Trait implemented by concrete AST nodes that expose member access.
pub trait MemberAccessNode {
    /// Returns the underlying member-access expression.
    fn member_access(&self) -> &MemberAccessExpression;

    /// Returns the receiver.
    #[inline]
    fn receiver(&self) -> NodeId {
        self.member_access().receiver()
    }

    /// Returns the source-level member name.
    #[inline]
    fn member(&self) -> &MemberName {
        self.member_access().member()
    }

    /// Returns the source-level member text.
    #[inline]
    fn member_name(&self) -> &str {
        self.member_access().member_name()
    }

    /// Returns direct AST children.
    #[inline]
    fn child_node_ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.member_access().child_node_ids()
    }
}

impl MemberAccessNode for MemberAccessExpression {
    #[inline]
    fn member_access(&self) -> &MemberAccessExpression {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value).expect("test node ID must be non-zero")
    }

    fn span() -> Span {
        Span::default()
    }

    #[test]
    fn constructs_member_access() {
        let expression = MemberAccessExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            MemberName::new("field").expect("valid member"),
        )
        .expect("valid member access");

        assert_eq!(expression.id(), node_id(1));
        assert_eq!(expression.receiver(), node_id(2));
        assert_eq!(expression.member_name(), "field");
        assert_eq!(
            expression.kind(),
            &NodeKind::core(CoreNodeKind::MemberAccess)
        );
    }

    #[test]
    fn exposes_exactly_one_ast_child() {
        let expression = MemberAccessExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            MemberName::new("field").expect("valid member"),
        )
        .expect("valid member access");

        let children: Vec<NodeId> = expression.child_node_ids().collect();

        assert_eq!(children, vec![node_id(2)]);
        assert_eq!(expression.child_count(), 1);
    }

    #[test]
    fn preserves_member_name_as_source_text() {
        let member = MemberName::new("π_value").expect("valid UTF-8 member");

        let expression = MemberAccessExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            member,
        )
        .expect("valid member access");

        assert_eq!(expression.member_name(), "π_value");
    }

    #[test]
    fn rejects_empty_member_name() {
        let result = MemberName::new("");

        assert_eq!(result, Err(MemberAccessError::EmptyMemberName));
    }

    #[test]
    fn rejects_default_receiver() {
        let result = MemberAccessExpression::without_metadata(
            node_id(1),
            span(),
            NodeId::default(),
            MemberName::new("field").expect("valid member"),
        );

        assert_eq!(result, Err(MemberAccessError::InvalidReceiver));
    }

    #[test]
    fn validates_successfully_with_unlimited_policy() {
        let expression = MemberAccessExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            MemberName::new("field").expect("valid member"),
        )
        .expect("valid member access");

        expression
            .validate_structure(MemberAccessValidationPolicy::default())
            .expect("structure must be valid");
    }

    #[test]
    fn validation_limit_is_explicit_and_configurable() {
        let expression = MemberAccessExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            MemberName::new("long_field").expect("valid member"),
        )
        .expect("valid member access");

        let result = expression.validate_structure(
            MemberAccessValidationPolicy {
                max_member_name_bytes: Some(4),
            },
        );

        assert_eq!(
            result,
            Err(MemberAccessError::MemberNameTooLong {
                actual: 10,
                maximum: 4,
            })
        );
    }

    #[test]
    fn validation_does_not_resolve_semantics() {
        let expression = MemberAccessExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            MemberName::new("does_not_matter").expect("valid member"),
        )
        .expect("valid member access");

        // Structural validation succeeds even though this module has no idea
        // whether the receiver semantically contains this member.
        expression
            .validate_structure(MemberAccessValidationPolicy::default())
            .expect("semantic resolution must remain downstream");
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(MemberAccessExpression::schema_version(), 1);
    }

    #[test]
    fn chained_access_is_compositional() {
        let first = MemberAccessExpression::without_metadata(
            node_id(2),
            span(),
            node_id(3),
            MemberName::new("b").expect("valid member"),
        )
        .expect("valid member access");

        // The outer access references the inner AST node by NodeId rather than
        // embedding it recursively.
        let second = MemberAccessExpression::without_metadata(
            node_id(1),
            span(),
            first.id(),
            MemberName::new("c").expect("valid member"),
        )
        .expect("valid member access");

        assert_eq!(second.receiver(), first.id());
        assert_eq!(second.member_name(), "c");
    }

    #[test]
    fn diagnostic_summary_is_deterministic() {
        let expression = MemberAccessExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            MemberName::new("field").expect("valid member"),
        )
        .expect("valid member access");

        let summary = expression.diagnostic_summary();

        assert_eq!(summary.id, node_id(1));
        assert_eq!(summary.receiver, node_id(2));
        assert_eq!(summary.member_name(), "field");
    }
}

impl MemberAccessDiagnosticSummary {
    /// Returns the source-level member text.
    #[inline]
    pub fn member_name(&self) -> &str {
        self.member.as_str()
    }
}