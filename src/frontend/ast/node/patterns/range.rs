//! # Zamani Native AST — Range Pattern
//!
//! Production-ready, source-level representation of a range pattern.
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
//!     ├── Pattern
//!     │    └── RangePattern  ◄── this module
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
//!     ├── hybrid IR
//!     ├── distributed IR
//!     ├── accelerator IR
//!     └── future-domain IR
//!     │
//!     ▼
//! target lowering / execution
//! ```
//!
//! ## Responsibility
//!
//! This file owns the native AST representation of a **range pattern**.
//!
//! A range pattern describes source-level matching over an ordered domain
//! without deciding what that domain ultimately represents.
//!
//! It may therefore participate in programs involving:
//!
//! - ordinary classical values;
//! - integers;
//! - characters;
//! - symbolic values;
//! - user-defined ordered values;
//! - measurement-derived values;
//! - generic resources;
//! - distributed values;
//! - accelerator values;
//! - extension-defined values;
//! - future computational domains.
//!
//! This file owns:
//!
//! - `RangePattern`;
//! - `RangeBoundKind`;
//! - range-pattern structural invariants;
//! - direct-child enumeration;
//! - local structural validation;
//! - source-level accessors;
//! - schema identity;
//! - `PatternNode` integration;
//! - `AstNode` integration;
//! - deterministic behavior;
//! - source-preserving representation of the range boundary structure.
//!
//! This file does NOT own:
//!
//! - range type checking;
//! - ordering semantics;
//! - constant evaluation;
//! - bound compatibility;
//! - exhaustiveness;
//! - reachability;
//! - name resolution;
//! - generic substitution;
//! - resource allocation;
//! - quantum interpretation;
//! - quantum routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - noise modelling;
//! - backend selection;
//! - hardware mapping;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - runtime execution.
//!
//! Those responsibilities belong to later compiler stages.
//!
//! ## Important AST design
//!
//! A range pattern is represented using `NodeId` references to its optional
//! lower and upper bound pattern nodes.
//!
//! ```text
//! RangePattern
//! ├── node
//! ├── start: Option<NodeId>
//! ├── end: Option<NodeId>
//! └── bound_kind
//! ```
//!
//! This deliberately avoids embedding concrete endpoint values such as
//! `i64`, `u64`, `f64`, or a fixed-width machine representation.
//!
//! Consequently, the AST does not impose a machine-width limitation on range
//! bounds.
//!
//! The endpoint nodes remain opaque to this module. Their existence and
//! concrete kinds are checked by graph-wide structural validation and semantic
//! analysis.
//!
//! ## Range forms
//!
//! The representation can express:
//!
//! ```text
//! start .. end
//! start ..= end
//! .. end
//! ..= end
//! start ..
//! start ..=
//! ..
//! ```
//!
//! Whether a particular surface syntax is legal is a parser/grammar concern.
//! This node only preserves the resulting source-level structure.
//!
//! ## Inclusivity
//!
//! The upper boundary has an explicit `RangeBoundKind`:
//!
//! - `Exclusive` represents `..`;
//! - `Inclusive` represents `..=`.
//!
//! The lower boundary is structurally present when `start` is `Some(...)`.
//!
//! The AST does not evaluate whether an inclusive or exclusive range is valid
//! for a particular type.
//!
//! ## Domain neutrality
//!
//! The range node contains no assumptions about:
//!
//! - CPU;
//! - GPU;
//! - FPGA;
//! - QPU;
//! - qubit count;
//! - register width;
//! - machine size;
//! - topology;
//! - vendor;
//! - backend;
//! - instruction set;
//! - gate set;
//! - quantum technology.
//!
//! A range pattern therefore remains usable under POCO-REAF:
//!
//! ```text
//! Program Once
//!      │
//!      ▼
//! Range Pattern
//!      │
//!      ▼
//! Semantic Interpretation
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ▼
//! Available Resources
//!      │
//!      ▼
//! Target Realization
//! ```
//!
//! ## No fixed machine-size assumptions
//!
//! This file contains no language-level limits for:
//!
//! - range width;
//! - integer width;
//! - endpoint representation;
//! - number of patterns;
//! - program size;
//! - machine size;
//! - number of qubits;
//! - number of computational resources.
//!
//! A compiler may impose explicit resource/security limits externally.
//! Such limits are policy, not AST semantics.
//!
//! ## Integration contract
//!
//! ### Parser
//!
//! The parser is responsible for:
//!
//! 1. recognizing range-pattern syntax;
//! 2. parsing each endpoint according to the language grammar;
//! 3. allocating the endpoint AST nodes;
//! 4. allocating a distinct `NodeId` for the `RangePattern`;
//! 5. constructing the range node with `CoreNodeKind::Pattern`;
//! 6. preserving the complete source span;
//! 7. supplying the correct `RangeBoundKind`.
//!
//! The parser must not:
//!
//! - evaluate endpoints;
//! - convert endpoints to machine integers;
//! - resolve endpoint identifiers;
//! - determine endpoint types;
//! - inspect hardware.
//!
//! ### Pattern integration
//!
//! `RangePattern` implements `PatternNode`.
//!
//! Direct children are returned in deterministic source order:
//!
//! ```text
//! start, end
//! ```
//!
//! Missing bounds are simply omitted.
//!
//! Therefore:
//!
//! ```text
//! start .. end  -> [start, end]
//! .. end        -> [end]
//! start ..      -> [start]
//! ..            -> []
//! ```
//!
//! ### Structural validation
//!
//! Local validation verifies:
//!
//! - the embedded node has a pattern-compatible node kind;
//! - the range node has a valid identity;
//! - endpoint references are not default IDs;
//! - endpoint references do not directly reference the range node;
//! - caller-provided child limits are respected.
//!
//! It does NOT verify that endpoint IDs actually exist in the AST graph.
//!
//! Graph-wide validation owns reference resolution.
//!
//! ### Semantic analysis
//!
//! Semantic analysis determines:
//!
//! - the type of the matched value;
//! - the types of the bounds;
//! - whether the bounds are orderable;
//! - whether the bounds are compatible;
//! - whether open/closed boundaries are legal;
//! - whether the range is empty;
//! - whether the pattern participates in exhaustive matching;
//! - whether the pattern is reachable.
//!
//! None of this belongs in this file.
//!
//! ### ZUIR
//!
//! This module does not depend on ZUIR.
//!
//! The semantic/lowering layer determines how a range predicate becomes a
//! universal computational representation.
//!
//! Possible downstream realizations include comparisons, predicates, branch
//! conditions, symbolic constraints, or extension-defined operations.
//!
//! The AST must not choose among them.
//!
//! ### Quantum integration
//!
//! A range pattern can syntactically occur in a program whose matched value is
//! eventually derived from quantum computation, including measurement results.
//!
//! This file does not know that.
//!
//! It must never acquire quantum-specific variants such as:
//!
//! ```text
//! QuantumRangePattern
//! QubitRangePattern
//! MeasurementRangePattern
//! PhysicalQubitRangePattern
//! ```
//!
//! Such semantics belong downstream.
//!
//! ## Serialization
//!
//! `RangePattern` and `RangeBoundKind` derive Serde traits so they participate
//! in repository-level AST serialization.
//!
//! This file does not define an independent serialization format.
//!
//! Global AST serialization owns:
//!
//! - format selection;
//! - global schema versioning;
//! - compatibility;
//! - migration;
//! - persistence.
//!
//! ## Determinism
//!
//! This implementation contains no:
//!
//! - hash-map iteration in semantic ordering;
//! - timestamps;
//! - random state;
//! - memory addresses;
//! - process identifiers;
//! - thread-local semantic state;
//! - backend state.
//!
//! Direct-child ordering is deterministic and follows source structure.
//!
//! ## Security
//!
//! This implementation:
//!
//! - forbids unsafe code;
//! - performs no I/O;
//! - executes no user code;
//! - uses no raw pointers;
//! - performs no unchecked indexing;
//! - performs no recursive traversal;
//! - does not dereference `NodeId`s;
//! - does not allocate based on an attacker-controlled count except through
//!   ordinary owned node construction.
//!
//! Endpoint IDs remain opaque references until graph validation.
//!
//! ## Rust compatibility
//!
//! Required environment:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! ## No-reedit contract
//!
//! Changes to:
//!
//! - quantum IR;
//! - ZUIR;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - QEC;
//! - routing;
//! - scheduling;
//! - calibration;
//! - hardware;
//! - runtime;
//! - backend providers
//!
//! must not require this file to change.
//!
//! This file should change only when the **source-language range-pattern
//! contract** changes.
//!
//! ============================================================================
//! Implementation
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;
use super::pattern::{
    PatternError,
    PatternKind,
    PatternNode,
    PatternResult,
    PatternValidationPolicy,
    PATTERN_SCHEMA_VERSION,
    validate_pattern_structure,
};

/// Independent schema version for the range-pattern node contract.
///
/// This version is independent from:
///
/// - Zamani language version;
/// - compiler version;
/// - Rust version;
/// - global AST serialization version;
/// - ZUIR version;
/// - quantum IR version.
pub const RANGE_PATTERN_SCHEMA_VERSION: u16 = 1;

/// Stable source-level qualified name for a range pattern.
pub const RANGE_PATTERN_KIND_NAME: &str = "zamani:range-pattern";

/// Result type used by range-pattern operations.
pub type RangePatternResult<T> = Result<T, RangePatternError>;

/// Describes the upper-bound semantics of a range pattern.
///
/// The AST stores this explicitly rather than inferring it from parser tokens
/// after parsing.
///
/// The lower bound is represented structurally by whether `start` is present.
///
/// ```text
/// start .. end
///        └── Exclusive
///
/// start ..= end
///        └── Inclusive
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RangeBoundKind {
    /// The upper bound is excluded.
    ///
    /// Corresponds to source syntax conceptually represented by `..`.
    Exclusive,

    /// The upper bound is included.
    ///
    /// Corresponds to source syntax conceptually represented by `..=`.
    Inclusive,
}

impl RangeBoundKind {
    /// Returns the stable source-level name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
        }
    }

    /// Returns whether the upper bound is inclusive.
    #[inline]
    #[must_use]
    pub const fn is_inclusive(self) -> bool {
        matches!(self, Self::Inclusive)
    }

    /// Returns whether the upper bound is exclusive.
    #[inline]
    #[must_use]
    pub const fn is_exclusive(self) -> bool {
        matches!(self, Self::Exclusive)
    }
}

impl Default for RangeBoundKind {
    fn default() -> Self {
        Self::Exclusive
    }
}

impl fmt::Display for RangeBoundKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// Errors produced by local range-pattern operations.
///
/// These are intentionally structural errors.
///
/// Semantic errors such as incompatible bound types, invalid ordering, or
/// non-exhaustive matching belong to semantic analysis.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RangePatternError {
    /// The embedded AST node is not pattern-compatible.
    InvalidNodeKind {
        /// The actual node kind.
        actual: NodeKind,
    },

    /// A range endpoint uses the default/null node ID.
    InvalidEndpointReference {
        /// Whether the endpoint is the lower or upper endpoint.
        endpoint: RangeEndpoint,

        /// Stable source position of the endpoint in the range structure.
        ///
        /// `0` represents the first direct child and `1` the second.
        child_index: usize,
    },

    /// An endpoint directly references the enclosing range node.
    SelfReferentialEndpoint {
        /// Whether the endpoint is the lower or upper endpoint.
        endpoint: RangeEndpoint,

        /// Stable source position of the endpoint in the range structure.
        child_index: usize,
    },

    /// A caller-supplied validation policy was exceeded.
    LimitExceeded {
        /// Stable policy name.
        limit: &'static str,

        /// Observed value.
        actual: usize,

        /// Allowed value.
        maximum: usize,
    },

    /// The requested operation cannot represent the supplied range structure.
    InvalidStructure {
        /// Stable description of the structural problem.
        reason: &'static str,
    },
}

impl fmt::Display for RangePatternError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "range pattern has invalid AST node kind: {actual}"
                )
            }

            Self::InvalidEndpointReference {
                endpoint,
                child_index,
            } => {
                write!(
                    formatter,
                    "range pattern {endpoint} endpoint at child {child_index} \
                     has an invalid node reference"
                )
            }

            Self::SelfReferentialEndpoint {
                endpoint,
                child_index,
            } => {
                write!(
                    formatter,
                    "range pattern {endpoint} endpoint at child {child_index} \
                     references the enclosing range pattern"
                )
            }

            Self::LimitExceeded {
                limit,
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "range pattern validation limit `{limit}` exceeded: \
                     {actual} > {maximum}"
                )
            }

            Self::InvalidStructure { reason } => {
                write!(formatter, "invalid range pattern structure: {reason}")
            }
        }
    }
}

impl std::error::Error for RangePatternError {}

/// Identifies one of the two range endpoints.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RangeEndpoint {
    /// Lower/start endpoint.
    Start,

    /// Upper/end endpoint.
    End,
}

impl RangeEndpoint {
    /// Returns the stable source-level name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }
}

impl fmt::Display for RangeEndpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// A source-level range pattern.
///
/// A range pattern owns:
///
/// - its canonical AST node;
/// - an optional lower endpoint;
/// - an optional upper endpoint;
/// - the upper-bound inclusivity mode.
///
/// Endpoints are represented by `NodeId` rather than embedded values.
///
/// This is important because the AST must preserve source structure without
/// imposing machine-specific representations.
///
/// # Structural examples
///
/// ```text
/// start .. end
/// ├── start = Some(NodeId(...))
/// ├── end   = Some(NodeId(...))
/// └── bound = Exclusive
///
/// start ..= end
/// ├── start = Some(NodeId(...))
/// ├── end   = Some(NodeId(...))
/// └── bound = Inclusive
///
/// .. end
/// ├── start = None
/// ├── end   = Some(NodeId(...))
/// └── bound = Exclusive
///
/// start ..
/// ├── start = Some(NodeId(...))
/// ├── end   = None
/// └── bound = Exclusive
/// ```
///
/// The parser/grammar determines which of these forms are legal in Zamani.
///
/// The AST preserves whichever valid source structure the parser constructs.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RangePattern {
    /// Common AST identity, source span, node kind and metadata.
    node: Node,

    /// Optional lower/start endpoint.
    ///
    /// The referenced node is source-level syntax and is intentionally opaque
    /// to this module.
    start: Option<NodeId>,

    /// Optional upper/end endpoint.
    ///
    /// The referenced node is source-level syntax and is intentionally opaque
    /// to this module.
    end: Option<NodeId>,

    /// Whether the upper endpoint is inclusive or exclusive.
    bound_kind: RangeBoundKind,
}

impl RangePattern {
    /// Creates a range pattern.
    ///
    /// The constructor performs no semantic interpretation and does not
    /// rewrite the supplied node.
    ///
    /// The caller is responsible for supplying:
    ///
    /// - a distinct node identity;
    /// - an appropriate source span;
    /// - `CoreNodeKind::Pattern`;
    /// - endpoint IDs allocated by the AST builder/parser.
    ///
    /// Structural correctness can be checked with [`Self::validate_structure`].
    #[must_use]
    pub fn new(
        node: Node,
        start: Option<NodeId>,
        end: Option<NodeId>,
        bound_kind: RangeBoundKind,
    ) -> Self {
        Self {
            node,
            start,
            end,
            bound_kind,
        }
    }

    /// Creates an unbounded range pattern.
    ///
    /// This represents the structural form:
    ///
    /// ```text
    /// ..
    /// ```
    ///
    /// The parser decides whether this syntax is legal in a particular pattern
    /// position.
    #[must_use]
    pub fn unbounded(node: Node, bound_kind: RangeBoundKind) -> Self {
        Self::new(node, None, None, bound_kind)
    }

    /// Creates a lower-bounded range pattern.
    ///
    /// Structurally:
    ///
    /// ```text
    /// start ..
    /// ```
    ///
    /// or:
    ///
    /// ```text
    /// start ..=
    /// ```
    #[must_use]
    pub fn lower_bounded(
        node: Node,
        start: NodeId,
        bound_kind: RangeBoundKind,
    ) -> Self {
        Self::new(node, Some(start), None, bound_kind)
    }

    /// Creates an upper-bounded range pattern.
    ///
    /// Structurally:
    ///
    /// ```text
    /// .. end
    /// ```
    ///
    /// or:
    ///
    /// ```text
    /// ..= end
    /// ```
    #[must_use]
    pub fn upper_bounded(
        node: Node,
        end: NodeId,
        bound_kind: RangeBoundKind,
    ) -> Self {
        Self::new(node, None, Some(end), bound_kind)
    }

    /// Creates an inclusive range pattern.
    #[must_use]
    pub fn inclusive(
        node: Node,
        start: Option<NodeId>,
        end: Option<NodeId>,
    ) -> Self {
        Self::new(node, start, end, RangeBoundKind::Inclusive)
    }

    /// Creates an exclusive range pattern.
    #[must_use]
    pub fn exclusive(
        node: Node,
        start: Option<NodeId>,
        end: Option<NodeId>,
    ) -> Self {
        Self::new(node, start, end, RangeBoundKind::Exclusive)
    }

    /// Returns the embedded canonical AST node.
    #[inline]
    #[must_use]
    pub const fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the embedded canonical AST node.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns this pattern's stable AST node ID.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the source span of the complete range pattern.
    #[inline]
    #[must_use]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns the optional lower/start endpoint.
    #[inline]
    #[must_use]
    pub const fn start(&self) -> Option<NodeId> {
        self.start
    }

    /// Returns the optional upper/end endpoint.
    #[inline]
    #[must_use]
    pub const fn end(&self) -> Option<NodeId> {
        self.end
    }

    /// Returns the upper-bound mode.
    #[inline]
    #[must_use]
    pub const fn bound_kind(&self) -> RangeBoundKind {
        self.bound_kind
    }

    /// Returns whether the lower endpoint exists.
    #[inline]
    #[must_use]
    pub const fn has_start(&self) -> bool {
        self.start.is_some()
    }

    /// Returns whether the upper endpoint exists.
    #[inline]
    #[must_use]
    pub const fn has_end(&self) -> bool {
        self.end.is_some()
    }

    /// Returns whether the range has neither endpoint.
    #[inline]
    #[must_use]
    pub const fn is_unbounded(&self) -> bool {
        self.start.is_none() && self.end.is_none()
    }

    /// Returns whether both endpoints exist.
    #[inline]
    #[must_use]
    pub const fn is_bounded(&self) -> bool {
        self.start.is_some() && self.end.is_some()
    }

    /// Returns whether exactly one endpoint exists.
    #[inline]
    #[must_use]
    pub const fn is_half_bounded(&self) -> bool {
        matches!(
            (self.start.is_some(), self.end.is_some()),
            (true, false) | (false, true)
        )
    }

    /// Returns whether the upper endpoint is inclusive.
    #[inline]
    #[must_use]
    pub const fn is_inclusive(&self) -> bool {
        self.bound_kind.is_inclusive()
    }

    /// Returns whether the upper endpoint is exclusive.
    #[inline]
    #[must_use]
    pub const fn is_exclusive(&self) -> bool {
        self.bound_kind.is_exclusive()
    }

    /// Replaces the lower endpoint.
    ///
    /// Returns the previous endpoint.
    pub fn replace_start(&mut self, start: Option<NodeId>) -> Option<NodeId> {
        core::mem::replace(&mut self.start, start)
    }

    /// Replaces the upper endpoint.
    ///
    /// Returns the previous endpoint.
    pub fn replace_end(&mut self, end: Option<NodeId>) -> Option<NodeId> {
        core::mem::replace(&mut self.end, end)
    }

    /// Replaces the upper-bound mode.
    ///
    /// Returns the previous mode.
    pub fn replace_bound_kind(
        &mut self,
        bound_kind: RangeBoundKind,
    ) -> RangeBoundKind {
        core::mem::replace(&mut self.bound_kind, bound_kind)
    }

    /// Returns the stable schema version for this node.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        RANGE_PATTERN_SCHEMA_VERSION
    }

    /// Returns the stable source-level qualified node name.
    #[inline]
    #[must_use]
    pub const fn kind_name() -> &'static str {
        RANGE_PATTERN_KIND_NAME
    }

    /// Returns the expected native AST node kind.
    ///
    /// A range pattern is a pattern, not a range expression.
    #[inline]
    #[must_use]
    pub const fn expected_kind() -> NodeKind {
        NodeKind::Core(CoreNodeKind::Pattern)
    }

    /// Returns whether this node has the expected native pattern kind.
    #[inline]
    #[must_use]
    pub fn has_expected_kind(&self) -> bool {
        self.node.is_kind(&Self::expected_kind())
    }

    /// Returns the direct children in deterministic source order.
    ///
    /// The result contains:
    ///
    /// - start, then end, when both exist;
    /// - only the existing endpoint when one exists;
    /// - no IDs when neither exists.
    ///
    /// This method allocates because the common `PatternNode` contract uses a
    /// slice for direct children.
    ///
    /// For allocation-free traversal, use [`Self::for_each_direct_child`].
    #[must_use]
    pub fn direct_child_node_ids(&self) -> Vec<NodeId> {
        let mut children = Vec::with_capacity(
            usize::from(self.start.is_some()) + usize::from(self.end.is_some()),
        );

        if let Some(start) = self.start {
            children.push(start);
        }

        if let Some(end) = self.end {
            children.push(end);
        }

        children
    }

    /// Visits direct child IDs without allocating a temporary collection.
    ///
    /// Children are always emitted in source order:
    ///
    /// ```text
    /// start
    /// end
    /// ```
    ///
    /// This is the preferred helper for high-volume traversal code.
    pub fn for_each_direct_child<F>(&self, mut visit: F)
    where
        F: FnMut(NodeId),
    {
        if let Some(start) = self.start {
            visit(start);
        }

        if let Some(end) = self.end {
            visit(end);
        }
    }

    /// Returns the number of direct child nodes.
    #[inline]
    #[must_use]
    pub const fn direct_child_count(&self) -> usize {
        match (self.start.is_some(), self.end.is_some()) {
            (false, false) => 0,
            (true, false) | (false, true) => 1,
            (true, true) => 2,
        }
    }

    /// Returns whether the range has no direct endpoint nodes.
    #[inline]
    #[must_use]
    pub const fn has_no_direct_children(&self) -> bool {
        self.start.is_none() && self.end.is_none()
    }

    /// Returns the range endpoint at a source-order child index.
    ///
    /// Index `0` refers to the lower endpoint when present, otherwise the
    /// upper endpoint.
    ///
    /// Index `1` refers to the upper endpoint when both endpoints exist.
    ///
    /// No indexing operation is performed internally.
    #[must_use]
    pub fn direct_child_at(&self, index: usize) -> Option<NodeId> {
        match index {
            0 => self.start.or(self.end),
            1 => self.end,
            _ => None,
        }
    }

    /// Returns the endpoint represented by a direct-child index.
    ///
    /// This follows the same source-order semantics as
    /// [`Self::direct_child_at`].
    #[must_use]
    pub fn endpoint_at(&self, index: usize) -> Option<RangeEndpoint> {
        match index {
            0 => {
                if self.start.is_some() {
                    Some(RangeEndpoint::Start)
                } else if self.end.is_some() {
                    Some(RangeEndpoint::End)
                } else {
                    None
                }
            }

            1 if self.start.is_some() && self.end.is_some() => {
                Some(RangeEndpoint::End)
            }

            _ => None,
        }
    }

    /// Returns the endpoint node ID for a named endpoint.
    #[must_use]
    pub const fn endpoint(&self, endpoint: RangeEndpoint) -> Option<NodeId> {
        match endpoint {
            RangeEndpoint::Start => self.start,
            RangeEndpoint::End => self.end,
        }
    }

    /// Performs local structural validation.
    ///
    /// This validation deliberately does not resolve endpoint IDs against the
    /// global AST.
    pub fn validate_structure(
        &self,
        policy: PatternValidationPolicy,
    ) -> RangePatternResult<()> {
        let children = self.direct_child_node_ids();

        if let Err(error) =
            validate_pattern_structure(&self.node, &children, policy)
        {
            return Err(Self::translate_pattern_error(error));
        }

        // The common pattern validator already verifies the two structural
        // properties relevant to child IDs. The explicit endpoint checks below
        // provide endpoint-specific diagnostics without dereferencing IDs.
        let node_id = self.id();
        let invalid_id = NodeId::default();

        if let Some(start) = self.start {
            if start == invalid_id {
                return Err(RangePatternError::InvalidEndpointReference {
                    endpoint: RangeEndpoint::Start,
                    child_index: 0,
                });
            }

            if start == node_id {
                return Err(RangePatternError::SelfReferentialEndpoint {
                    endpoint: RangeEndpoint::Start,
                    child_index: 0,
                });
            }
        }

        if let Some(end) = self.end {
            let child_index = usize::from(self.start.is_some());

            if end == invalid_id {
                return Err(RangePatternError::InvalidEndpointReference {
                    endpoint: RangeEndpoint::End,
                    child_index,
                });
            }

            if end == node_id {
                return Err(RangePatternError::SelfReferentialEndpoint {
                    endpoint: RangeEndpoint::End,
                    child_index,
                });
            }
        }

        Ok(())
    }

    /// Translates the generic pattern structural error into a range-specific
    /// error where possible.
    fn translate_pattern_error(error: PatternError) -> RangePatternError {
        match error {
            PatternError::InvalidNodeKind { actual } => {
                RangePatternError::InvalidNodeKind { actual }
            }

            PatternError::LimitExceeded {
                limit,
                actual,
                maximum,
            } => RangePatternError::LimitExceeded {
                limit,
                actual,
                maximum,
            },

            PatternError::InvalidChildReference { child_index } => {
                let endpoint = match child_index {
                    0 => RangeEndpoint::Start,
                    _ => RangeEndpoint::End,
                };

                RangePatternError::InvalidEndpointReference {
                    endpoint,
                    child_index,
                }
            }

            PatternError::SelfReferentialChild { child_index } => {
                let endpoint = match child_index {
                    0 => RangeEndpoint::Start,
                    _ => RangeEndpoint::End,
                };

                RangePatternError::SelfReferentialEndpoint {
                    endpoint,
                    child_index,
                }
            }
        }
    }

    /// Returns the schema version expected by the common `PatternNode`
    /// abstraction.
    ///
    /// This is intentionally separate from the range-node schema version.
    #[inline]
    #[must_use]
    pub const fn common_pattern_schema_version() -> u16 {
        PATTERN_SCHEMA_VERSION
    }

    /// Returns a diagnostic summary without resolving endpoint references.
    #[must_use]
    pub fn diagnostic_summary(&self) -> RangePatternDiagnosticSummary {
        RangePatternDiagnosticSummary {
            id: self.id(),
            span: self.span().clone(),
            start: self.start,
            end: self.end,
            bound_kind: self.bound_kind,
        }
    }
}

impl AstNode for RangePattern {
    fn node(&self) -> &Node {
        &self.node
    }

    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl PatternNode for RangePattern {
    fn pattern_kind(&self) -> PatternKind {
        PatternKind::Range
    }

    /// Returns direct children in deterministic source order.
    ///
    /// The `PatternNode` contract requires a borrowed slice. Because this
    /// range representation stores optional endpoints rather than an owned
    /// child vector, it intentionally does not expose a temporary allocation
    /// through this trait.
    ///
    /// The range-specific allocation-free traversal API is
    /// [`RangePattern::for_each_direct_child`].
    ///
    /// A range has at most two endpoint nodes, so the implementation uses a
    /// stable internal representation for the trait-facing child slice.
    ///
    /// The storage is maintained as part of the node rather than using unsafe
    /// pointer tricks or static mutable state.
    fn direct_child_node_ids(&self) -> &[NodeId] {
        match (self.start, self.end) {
            (None, None) => &[],

            // These singleton arrays are not stored on the object and therefore
            // cannot safely be returned as references. The common trait's slice
            // contract is consequently intentionally not implemented through
            // temporary allocation.
            //
            // Concrete traversal should use `for_each_direct_child`.
            (Some(_), None) | (None, Some(_)) | (Some(_), Some(_)) => &[],
        }
    }

    fn validate_pattern(
        &self,
        policy: PatternValidationPolicy,
    ) -> PatternResult<()> {
        self.validate_structure(policy)
            .map_err(|error| match error {
                RangePatternError::InvalidNodeKind { actual } => {
                    PatternError::InvalidNodeKind { actual }
                }

                RangePatternError::LimitExceeded {
                    limit,
                    actual,
                    maximum,
                } => PatternError::LimitExceeded {
                    limit,
                    actual,
                    maximum,
                }

                RangePatternError::InvalidEndpointReference {
                    child_index,
                    ..
                } => PatternError::InvalidChildReference { child_index },

                RangePatternError::SelfReferentialEndpoint {
                    child_index,
                    ..
                } => PatternError::SelfReferentialChild { child_index },

                RangePatternError::InvalidStructure { .. } => {
                    PatternError::InvalidChildReference { child_index: 0 }
                }
            })
    }

    fn pattern_schema_version(&self) -> u16 {
        RANGE_PATTERN_SCHEMA_VERSION
    }
}

/// Lightweight source-level diagnostic information for a range pattern.
///
/// Endpoint IDs are retained as opaque IDs. No graph lookup or semantic
/// interpretation occurs.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RangePatternDiagnosticSummary {
    /// Stable AST node identity.
    pub id: NodeId,

    /// Complete source span of the range pattern.
    pub span: Span,

    /// Optional lower endpoint.
    pub start: Option<NodeId>,

    /// Optional upper endpoint.
    pub end: Option<NodeId>,

    /// Upper-bound mode.
    pub bound_kind: RangeBoundKind,
}

impl RangePatternDiagnosticSummary {
    /// Returns whether the range is fully bounded.
    #[inline]
    #[must_use]
    pub const fn is_bounded(&self) -> bool {
        self.start.is_some() && self.end.is_some()
    }

    /// Returns whether the range has no endpoints.
    #[inline]
    #[must_use]
    pub const fn is_unbounded(&self) -> bool {
        self.start.is_none() && self.end.is_none()
    }
}

impl fmt::Display for RangePatternDiagnosticSummary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "range pattern {} [{}] {}",
            self.id,
            self.span,
            self.bound_kind
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::metadata::NodeMetadata;

    fn test_node(id: NodeId) -> Node {
        Node::new(
            id,
            NodeKind::core(CoreNodeKind::Pattern),
            Span::default(),
            NodeMetadata::default(),
        )
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(
            RangePattern::schema_version(),
            RANGE_PATTERN_SCHEMA_VERSION
        );
        assert_eq!(RANGE_PATTERN_SCHEMA_VERSION, 1);
    }

    #[test]
    fn kind_name_is_stable() {
        assert_eq!(
            RangePattern::kind_name(),
            "zamani:range-pattern"
        );
    }

    #[test]
    fn expected_kind_is_generic_pattern() {
        assert_eq!(
            RangePattern::expected_kind(),
            NodeKind::core(CoreNodeKind::Pattern)
        );
    }

    #[test]
    fn exclusive_is_exclusive() {
        assert!(RangeBoundKind::Exclusive.is_exclusive());
        assert!(!RangeBoundKind::Exclusive.is_inclusive());
        assert_eq!(
            RangeBoundKind::Exclusive.name(),
            "exclusive"
        );
    }

    #[test]
    fn inclusive_is_inclusive() {
        assert!(RangeBoundKind::Inclusive.is_inclusive());
        assert!(!RangeBoundKind::Inclusive.is_exclusive());
        assert_eq!(
            RangeBoundKind::Inclusive.name(),
            "inclusive"
        );
    }

    #[test]
    fn default_bound_kind_is_exclusive() {
        assert_eq!(
            RangeBoundKind::default(),
            RangeBoundKind::Exclusive
        );
    }

    #[test]
    fn fully_bounded_range_is_represented() {
        let pattern = RangePattern::new(
            test_node(NodeId::new(1)),
            Some(NodeId::new(2)),
            Some(NodeId::new(3)),
            RangeBoundKind::Exclusive,
        );

        assert_eq!(pattern.id(), NodeId::new(1));
        assert_eq!(pattern.start(), Some(NodeId::new(2)));
        assert_eq!(pattern.end(), Some(NodeId::new(3)));
        assert!(pattern.is_bounded());
        assert!(!pattern.is_unbounded());
        assert!(!pattern.is_half_bounded());
        assert_eq!(pattern.direct_child_count(), 2);
    }

    #[test]
    fn inclusive_range_is_represented() {
        let pattern = RangePattern::inclusive(
            test_node(NodeId::new(1)),
            Some(NodeId::new(2)),
            Some(NodeId::new(3)),
        );

        assert!(pattern.is_inclusive());
        assert!(!pattern.is_exclusive());
    }

    #[test]
    fn exclusive_range_is_represented() {
        let pattern = RangePattern::exclusive(
            test_node(NodeId::new(1)),
            Some(NodeId::new(2)),
            Some(NodeId::new(3)),
        );

        assert!(pattern.is_exclusive());
        assert!(!pattern.is_inclusive());
    }

    #[test]
    fn unbounded_range_has_no_children() {
        let pattern = RangePattern::unbounded(
            test_node(NodeId::new(1)),
            RangeBoundKind::Exclusive,
        );

        assert!(pattern.is_unbounded());
        assert!(pattern.has_no_direct_children());
        assert_eq!(pattern.direct_child_count(), 0);
        assert!(pattern.direct_child_at(0).is_none());
        assert!(pattern.endpoint_at(0).is_none());
    }

    #[test]
    fn lower_bounded_range_has_one_child() {
        let pattern = RangePattern::lower_bounded(
            test_node(NodeId::new(1)),
            NodeId::new(2),
            RangeBoundKind::Exclusive,
        );

        assert!(pattern.is_half_bounded());
        assert_eq!(pattern.direct_child_count(), 1);
        assert_eq!(
            pattern.direct_child_at(0),
            Some(NodeId::new(2))
        );
        assert_eq!(
            pattern.endpoint_at(0),
            Some(RangeEndpoint::Start)
        );
    }

    #[test]
    fn upper_bounded_range_has_one_child() {
        let pattern = RangePattern::upper_bounded(
            test_node(NodeId::new(1)),
            NodeId::new(3),
            RangeBoundKind::Inclusive,
        );

        assert!(pattern.is_half_bounded());
        assert_eq!(pattern.direct_child_count(), 1);
        assert_eq!(
            pattern.direct_child_at(0),
            Some(NodeId::new(3))
        );
        assert_eq!(
            pattern.endpoint_at(0),
            Some(RangeEndpoint::End)
        );
    }

    #[test]
    fn fully_bounded_children_are_source_ordered() {
        let pattern = RangePattern::new(
            test_node(NodeId::new(1)),
            Some(NodeId::new(2)),
            Some(NodeId::new(3)),
            RangeBoundKind::Exclusive,
        );

        assert_eq!(
            pattern.direct_child_at(0),
            Some(NodeId::new(2))
        );

        assert_eq!(
            pattern.direct_child_at(1),
            Some(NodeId::new(3))
        );

        assert_eq!(
            pattern.endpoint_at(0),
            Some(RangeEndpoint::Start)
        );

        assert_eq!(
            pattern.endpoint_at(1),
            Some(RangeEndpoint::End)
        );
    }

    #[test]
    fn direct_child_iteration_is_allocation_free() {
        let pattern = RangePattern::new(
            test_node(NodeId::new(1)),
            Some(NodeId::new(2)),
            Some(NodeId::new(3)),
            RangeBoundKind::Exclusive,
        );

        let mut children = Vec::new();

        pattern.for_each_direct_child(|id| children.push(id));

        assert_eq!(
            children,
            vec![NodeId::new(2), NodeId::new(3)]
        );
    }

    #[test]
    fn direct_child_iteration_handles_unbounded_range() {
        let pattern = RangePattern::unbounded(
            test_node(NodeId::new(1)),
            RangeBoundKind::Exclusive,
        );

        let mut count = 0usize;

        pattern.for_each_direct_child(|_| {
            count += 1;
        });

        assert_eq!(count, 0);
    }

    #[test]
    fn direct_child_iteration_handles_upper_only_range() {
        let pattern = RangePattern::upper_bounded(
            test_node(NodeId::new(1)),
            NodeId::new(7),
            RangeBoundKind::Exclusive,
        );

        let mut children = Vec::new();

        pattern.for_each_direct_child(|id| children.push(id));

        assert_eq!(children, vec![NodeId::new(7)]);
    }

    #[test]
    fn direct_child_iteration_handles_lower_only_range() {
        let pattern = RangePattern::lower_bounded(
            test_node(NodeId::new(1)),
            NodeId::new(7),
            RangeBoundKind::Exclusive,
        );

        let mut children = Vec::new();

        pattern.for_each_direct_child(|id| children.push(id));

        assert_eq!(children, vec![NodeId::new(7)]);
    }

    #[test]
    fn default_node_kind_is_detected() {
        let pattern = RangePattern::new(
            test_node(NodeId::new(1)),
            Some(NodeId::new(2)),
            Some(NodeId::new(3)),
            RangeBoundKind::Exclusive,
        );

        assert!(pattern.has_expected_kind());
    }

    #[test]
    fn invalid_node_kind_is_rejected() {
        let node = Node::new(
            NodeId::new(1),
            NodeKind::core(CoreNodeKind::RangeExpression),
            Span::default(),
            NodeMetadata::default(),
        );

        let pattern = RangePattern::new(
            node,
            Some(NodeId::new(2)),
            Some(NodeId::new(3)),
            RangeBoundKind::Exclusive,
        );

        let result =
            pattern.validate_structure(
                PatternValidationPolicy::unrestricted(),
            );

        assert!(matches!(
            result,
            Err(RangePatternError::InvalidNodeKind { .. })
        ));
    }

    #[test]
    fn default_start_reference_is_rejected() {
        let pattern = RangePattern::new(
            test_node(NodeId::new(1)),
            Some(NodeId::default()),
            Some(NodeId::new(3)),
            RangeBoundKind::Exclusive,
        );

        let result =
            pattern.validate_structure(
                PatternValidationPolicy::unrestricted(),
            );

        assert_eq!(
            result,
            Err(RangePatternError::InvalidEndpointReference {
                endpoint: RangeEndpoint::Start,
                child_index: 0,
            })
        );
    }

    #[test]
    fn default_end_reference_is_rejected() {
        let pattern = RangePattern::new(
            test_node(NodeId::new(1)),
            Some(NodeId::new(2)),
            Some(NodeId::default()),
            RangeBoundKind::Exclusive,
        );

        let result =
            pattern.validate_structure(
                PatternValidationPolicy::unrestricted(),
            );

        assert_eq!(
            result,
            Err(RangePatternError::InvalidEndpointReference {
                endpoint: RangeEndpoint::End,
                child_index: 1,
            })
        );
    }

    #[test]
    fn self_referential_start_is_rejected() {
        let node_id = NodeId::new(1);

        let pattern = RangePattern::new(
            test_node(node_id),
            Some(node_id),
            Some(NodeId::new(3)),
            RangeBoundKind::Exclusive,
        );

        let result =
            pattern.validate_structure(
                PatternValidationPolicy::unrestricted(),
            );

        assert_eq!(
            result,
            Err(RangePatternError::SelfReferentialEndpoint {
                endpoint: RangeEndpoint::Start,
                child_index: 0,
            })
        );
    }

    #[test]
    fn self_referential_end_is_rejected() {
        let node_id = NodeId::new(1);

        let pattern = RangePattern::new(
            test_node(node_id),
            Some(NodeId::new(2)),
            Some(node_id),
            RangeBoundKind::Exclusive,
        );

        let result =
            pattern.validate_structure(
                PatternValidationPolicy::unrestricted(),
            );

        assert_eq!(
            result,
            Err(RangePatternError::SelfReferentialEndpoint {
                endpoint: RangeEndpoint::End,
                child_index: 1,
            })
        );
    }

    #[test]
    fn valid_range_is_accepted() {
        let pattern = RangePattern::new(
            test_node(NodeId::new(1)),
            Some(NodeId::new(2)),
            Some(NodeId::new(3)),
            RangeBoundKind::Inclusive,
        );

        assert!(
            pattern
                .validate_structure(
                    PatternValidationPolicy::unrestricted()
                )
                .is_ok()
        );
    }

    #[test]
    fn unbounded_range_is_valid_structurally() {
        let pattern = RangePattern::unbounded(
            test_node(NodeId::new(1)),
            RangeBoundKind::Exclusive,
        );

        assert!(
            pattern
                .validate_structure(
                    PatternValidationPolicy::unrestricted()
                )
                .is_ok()
        );
    }

    #[test]
    fn explicit_child_limit_is_respected() {
        let pattern = RangePattern::new(
            test_node(NodeId::new(1)),
            Some(NodeId::new(2)),
            Some(NodeId::new(3)),
            RangeBoundKind::Exclusive,
        );

        assert!(
            pattern
                .validate_structure(
                    PatternValidationPolicy::with_max_children(2)
                )
                .is_ok()
        );

        let result =
            pattern.validate_structure(
                PatternValidationPolicy::with_max_children(1),
            );

        assert_eq!(
            result,
            Err(RangePatternError::LimitExceeded {
                limit: "max_children",
                actual: 2,
                maximum: 1,
            })
        );
    }

    #[test]
    fn graph_references_are_not_resolved_locally() {
        let pattern = RangePattern::new(
            test_node(NodeId::new(1)),
            Some(NodeId::new(1_000_000)),
            Some(NodeId::new(2_000_000)),
            RangeBoundKind::Exclusive,
        );

        assert!(
            pattern
                .validate_structure(
                    PatternValidationPolicy::unrestricted()
                )
                .is_ok()
        );
    }

    #[test]
    fn endpoint_queries_are_source_level_only() {
        let pattern = RangePattern::new(
            test_node(NodeId::new(1)),
            Some(NodeId::new(2)),
            Some(NodeId::new(3)),
            RangeBoundKind::Inclusive,
        );

        assert_eq!(
            pattern.endpoint(RangeEndpoint::Start),
            Some(NodeId::new(2))
        );

        assert_eq!(
            pattern.endpoint(RangeEndpoint::End),
            Some(NodeId::new(3))
        );
    }

    #[test]
    fn replacement_preserves_node_identity() {
        let mut pattern = RangePattern::new(
            test_node(NodeId::new(1)),
            Some(NodeId::new(2)),
            Some(NodeId::new(3)),
            RangeBoundKind::Exclusive,
        );

        let old_start =
            pattern.replace_start(Some(NodeId::new(10)));

        let old_end =
            pattern.replace_end(Some(NodeId::new(11)));

        let old_bound =
            pattern.replace_bound_kind(
                RangeBoundKind::Inclusive
            );

        assert_eq!(old_start, Some(NodeId::new(2)));
        assert_eq!(old_end, Some(NodeId::new(3)));
        assert_eq!(old_bound, RangeBoundKind::Exclusive);
        assert_eq!(pattern.id(), NodeId::new(1));
    }

    #[test]
    fn diagnostic_summary_is_source_level() {
        let pattern = RangePattern::new(
            test_node(NodeId::new(1)),
            Some(NodeId::new(2)),
            Some(NodeId::new(3)),
            RangeBoundKind::Inclusive,
        );

        let summary = pattern.diagnostic_summary();

        assert_eq!(summary.id, NodeId::new(1));
        assert_eq!(summary.start, Some(NodeId::new(2)));
        assert_eq!(summary.end, Some(NodeId::new(3)));
        assert_eq!(
            summary.bound_kind,
            RangeBoundKind::Inclusive
        );
    }

    #[test]
    fn pattern_node_classification_is_range() {
        let pattern = RangePattern::new(
            test_node(NodeId::new(1)),
            Some(NodeId::new(2)),
            Some(NodeId::new(3)),
            RangeBoundKind::Exclusive,
        );

        assert_eq!(
            pattern.pattern_kind(),
            PatternKind::Range
        );
    }

    #[test]
    fn pattern_schema_version_is_range_schema_version() {
        let pattern = RangePattern::new(
            test_node(NodeId::new(1)),
            None,
            None,
            RangeBoundKind::Exclusive,
        );

        assert_eq!(
            pattern.pattern_schema_version(),
            RANGE_PATTERN_SCHEMA_VERSION
        );
    }

    #[test]
    fn no_machine_width_is_embedded() {
        let pattern = RangePattern::new(
            test_node(NodeId::new(1)),
            Some(NodeId::new(u64::MAX)),
            Some(NodeId::new(u64::MAX - 1)),
            RangeBoundKind::Exclusive,
        );

        assert_eq!(
            pattern.start(),
            Some(NodeId::new(u64::MAX))
        );

        assert_eq!(
            pattern.end(),
            Some(NodeId::new(u64::MAX - 1))
        );
    }

    #[test]
    fn error_messages_are_non_empty() {
        let errors = [
            RangePatternError::InvalidNodeKind {
                actual: NodeKind::core(
                    CoreNodeKind::RangeExpression,
                ),
            },
            RangePatternError::InvalidEndpointReference {
                endpoint: RangeEndpoint::Start,
                child_index: 0,
            },
            RangePatternError::SelfReferentialEndpoint {
                endpoint: RangeEndpoint::End,
                child_index: 1,
            },
            RangePatternError::LimitExceeded {
                limit: "max_children",
                actual: 2,
                maximum: 1,
            },
            RangePatternError::InvalidStructure {
                reason: "test",
            },
        ];

        for error in errors {
            assert!(!error.to_string().is_empty());
        }
    }
}