//! Zamani Quantum IR — Resource Locality
//!
//! Hardware-independent locality semantics for quantum and hybrid programs.
//!
//! # Architectural role
//!
//! This module describes *how an operation or resource requirement relates to
//! the resources it touches*.
//!
//! It answers:
//!
//! > How local or non-local is this semantic operation/resource requirement?
//!
//! It does NOT answer:
//!
//! - which physical qubits exist;
//! - which physical qubits are connected;
//! - how far apart physical qubits are;
//! - which routing algorithm should be used;
//! - how SWAPs should be inserted;
//! - which hardware topology is selected;
//! - how an operation is scheduled;
//! - which backend executes the operation;
//! - which calibration is used;
//! - which vendor instruction implements the operation.
//!
//! Those responsibilities belong to downstream resource, topology, routing,
//! scheduling, hardware and backend layers.
//!
//! # Design principle
//!
//! Locality is a semantic property, not a hardware topology.
//!
//! For example:
//!
//! ```text
//! CNOT(q0, q1)
//! ```
//!
//! semantically requires a two-resource interaction.
//!
//! It does NOT mean:
//!
//! ```text
//! q0 and q1 must be adjacent on hardware.
//! ```
//!
//! Whether two resources are physically adjacent is a target-specific
//! topology question.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once at the semantic level and may be lowered
//! to compatible targets of different sizes and architectures.
//!
//! Therefore this module contains:
//!
//! - no fixed number of qubits;
//! - no fixed number of operands;
//! - no fixed topology;
//! - no fixed maximum locality;
//! - no vendor-specific topology;
//! - no architecture-specific distance metric;
//! - no `usize::MAX` sentinel semantics.
//!
//! A locality requirement can describe:
//!
//! - one resource;
//! - two resources;
//! - N resources;
//! - an arbitrary finite number of resources;
//! - an unbounded semantic interaction;
//! - a globally acting operation;
//! - a distributed interaction;
//! - an explicitly unknown/deferred locality.
//!
//! # Canonical qubit identities
//!
//! The canonical qubit identities are owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module uses those canonical types directly.
//!
//! It must never introduce another logical- or physical-qubit identifier.
//!
//! # Relationship to topology
//!
//! ```text
//!                    CANONICAL IR
//!                         │
//!                         ▼
//!                 resources::locality
//!                         │
//!              "what interaction is required?"
//!                         │
//!                         ▼
//!                hardware::topology
//!                         │
//!              "what connections exist?"
//!                         │
//!                         ▼
//!                    routing
//!                         │
//!              "how do we realize it?"
//! ```
//!
//! Locality therefore describes requirements that topology and routing later
//! interpret.
//!
//! # Relationship to resource.rs
//!
//! `resource.rs` describes *what resource quantities are required*.
//!
//! This module describes *how resources relate to one another*.
//!
//! Example:
//!
//! ```text
//! resource:
//!     logical qubits >= 3
//!
//! locality:
//!     one 3-resource interaction
//! ```
//!
//! The two concepts must remain separate.
//!
//! # Relationship to quantum::ir::qubit
//!
//! `QubitId` identifies a logical quantum resource.
//!
//! `PhysicalQubitId` identifies a physical target namespace.
//!
//! This module may reference both, but it does not allocate or map them.
//!
//! # Relationship to routing
//!
//! Routing may consume:
//!
//! - `LocalityRequirement`;
//! - `InteractionScope`;
//! - `LocalityConstraint`;
//! - `QubitInteraction`;
//!
//! and combine those with target topology.
//!
//! Routing MUST NOT require changes to this file merely because a new hardware
//! topology is introduced.
//!
//! # Relationship to hardware
//!
//! Hardware may define:
//!
//! - adjacency;
//! - graph distance;
//! - coupling;
//! - communication links;
//! - all-to-all connectivity;
//! - dynamic connectivity;
//! - distributed links.
//!
//! Those concepts do not belong in this file.
//!
//! # Relationship to optimization
//!
//! Optimization may inspect locality to determine whether an operation can be
//! transformed or decomposed.
//!
//! Optimization does not own locality semantics.
//!
//! # Relationship to scheduling
//!
//! Scheduling may use locality information together with resource conflicts.
//!
//! Scheduling does not own locality semantics.
//!
//! # Relationship to serialization
//!
//! All public locality structures are deterministic data structures.
//!
//! Serialization belongs to the canonical IR serialization layer.
//!
//! This module does not introduce a second serialization format.
//!
//! # Relationship to hashing
//!
//! Locality values derive deterministic equality and hashing.
//!
//! Canonical IR hashing remains owned by the IR hashing layer.
//!
//! # Error policy
//!
//! Constructors reject semantically impossible values.
//!
//! This module never silently clamps, truncates or wraps locality values.
//!
//! # Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! # Security
//!
//! This module does not allocate based on a semantic locality value.
//!
//! Collections are caller-provided and therefore do not implicitly allocate
//! an amount proportional to a declared machine size.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Scalability
//!
//! The types in this module describe locality independently of the number of
//! resources in a machine.
//!
//! The following are all representable using the same API:
//!
//! ```text
//! 1-local
//! 2-local
//! 3-local
//! k-local
//! N-local
//! global
//! distributed
//! ```
//!
//! No value above is an architectural limit.

#![forbid(unsafe_code)]

// =============================================================================
// Imports
// =============================================================================

use std::fmt;

use super::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Locality arity
// =============================================================================

/// Semantic cardinality of an interaction.
///
/// This type deliberately separates semantic arity from a concrete hardware
/// limit.
///
/// `Any` means that the semantic operation does not impose a finite upper
/// bound on the number of resources it may involve.
///
/// `AtLeast` is useful for operations whose semantics require a minimum number
/// of participants while allowing additional participants.
///
/// `Between` represents an inclusive finite lower/upper range.
///
/// No value here represents the maximum size of a quantum computer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LocalityArity {
    /// Exactly one resource participates.
    ///
    /// This is commonly called 1-local.
    Exact(u64),

    /// At least this many resources participate.
    AtLeast(u64),

    /// Between `min` and `max`, inclusive.
    Between {
        /// Minimum number of participating resources.
        min: u64,

        /// Maximum number of participating resources.
        max: u64,
    },

    /// Any finite number of resources is semantically permitted.
    Any,
}

impl LocalityArity {
    /// Creates an exact arity.
    ///
    /// `Exact(0)` is valid because some semantic operations may carry no
    /// resource operands, for example a pure declaration or resource-free
    /// classical operation.
    #[must_use]
    pub const fn exact(value: u64) -> Self {
        Self::Exact(value)
    }

    /// Creates a minimum-only arity.
    #[must_use]
    pub const fn at_least(value: u64) -> Self {
        Self::AtLeast(value)
    }

    /// Creates an unbounded arity.
    #[must_use]
    pub const fn any() -> Self {
        Self::Any
    }

    /// Creates an inclusive finite arity range.
    ///
    /// Returns `None` when `min > max`.
    #[must_use]
    pub const fn between(min: u64, max: u64) -> Option<Self> {
        if min > max {
            None
        } else {
            Some(Self::Between { min, max })
        }
    }

    /// Returns the minimum possible arity.
    #[must_use]
    pub const fn min(self) -> u64 {
        match self {
            Self::Exact(value) => value,
            Self::AtLeast(value) => value,
            Self::Between { min, .. } => min,
            Self::Any => 0,
        }
    }

    /// Returns the finite maximum arity when one exists.
    #[must_use]
    pub const fn max(self) -> Option<u64> {
        match self {
            Self::Exact(value) => Some(value),
            Self::AtLeast(_) => None,
            Self::Between { max, .. } => Some(max),
            Self::Any => None,
        }
    }

    /// Returns whether the arity has no finite upper bound.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        self.max().is_none()
    }

    /// Returns whether this arity accepts a concrete participant count.
    #[must_use]
    pub const fn accepts(self, count: u64) -> bool {
        match self {
            Self::Exact(expected) => count == expected,

            Self::AtLeast(minimum) => count >= minimum,

            Self::Between { min, max } => count >= min && count <= max,

            Self::Any => true,
        }
    }

    /// Returns whether the arity is exactly one.
    #[must_use]
    pub const fn is_unary(self) -> bool {
        matches!(self, Self::Exact(1))
    }

    /// Returns whether the arity is exactly two.
    #[must_use]
    pub const fn is_binary(self) -> bool {
        matches!(self, Self::Exact(2))
    }

    /// Returns whether the arity is exactly three.
    #[must_use]
    pub const fn is_ternary(self) -> bool {
        matches!(self, Self::Exact(3))
    }

    /// Returns whether the arity permits more than one resource.
    #[must_use]
    pub const fn permits_multiple(self) -> bool {
        self.max().map_or(true, |maximum| maximum > 1)
    }
}

impl fmt::Display for LocalityArity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact(value) => write!(formatter, "exactly {value}"),

            Self::AtLeast(value) => write!(formatter, "at least {value}"),

            Self::Between { min, max } => {
                write!(formatter, "between {min} and {max}")
            }

            Self::Any => formatter.write_str("any finite number"),
        }
    }
}

// =============================================================================
// Locality scope
// =============================================================================

/// Semantic scope of an interaction.
///
/// Scope describes the *kind* of locality without embedding a hardware graph.
///
/// It is deliberately technology-neutral.
///
/// For example, `Local` does not mean "nearest neighbour". It means that the
/// operation has a finite participant scope that can be interpreted against a
/// target topology later.
///
/// `Global` means the operation semantically acts over an entire relevant
/// resource domain.
///
/// `Distributed` means participating resources may belong to distinct
/// execution domains or nodes.
///
/// `NonLocal` means locality is not required by the semantic operation.
///
/// `Unknown` means the information is intentionally deferred.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LocalityScope {
    /// The operation is local to a finite participant set.
    Local,

    /// The operation is explicitly pairwise.
    Pairwise,

    /// The operation is explicitly multi-resource.
    MultiResource,

    /// The operation acts over a globally defined resource domain.
    Global,

    /// The operation may span independent execution domains.
    Distributed,

    /// The operation explicitly has no locality restriction.
    NonLocal,

    /// Locality is not yet known and must be resolved later.
    Unknown,
}

impl LocalityScope {
    /// Returns `true` when this scope represents finite locality.
    #[must_use]
    pub const fn is_local(self) -> bool {
        matches!(
            self,
            Self::Local | Self::Pairwise | Self::MultiResource
        )
    }

    /// Returns `true` when this scope is explicitly global.
    #[must_use]
    pub const fn is_global(self) -> bool {
        matches!(self, Self::Global)
    }

    /// Returns `true` when this scope can span execution domains.
    #[must_use]
    pub const fn is_distributed(self) -> bool {
        matches!(self, Self::Distributed)
    }

    /// Returns `true` when the operation explicitly imposes no locality
    /// restriction.
    #[must_use]
    pub const fn is_non_local(self) -> bool {
        matches!(self, Self::NonLocal)
    }

    /// Returns `true` when locality is deferred.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

impl fmt::Display for LocalityScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Local => "local",
            Self::Pairwise => "pairwise",
            Self::MultiResource => "multi_resource",
            Self::Global => "global",
            Self::Distributed => "distributed",
            Self::NonLocal => "non_local",
            Self::Unknown => "unknown",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Locality kind
// =============================================================================

/// High-level semantic locality classification.
///
/// This is a convenience classification over scope and arity.
///
/// It is not a replacement for `LocalityRequirement`; the requirement carries
/// the complete semantic information.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LocalityKind {
    /// Exactly one resource.
    Unary,

    /// Exactly two resources.
    Binary,

    /// A finite multi-resource interaction.
    MultiQubit,

    /// A finite interaction with a parameterized/unbounded upper bound.
    KLocal,

    /// An interaction over a globally defined domain.
    Global,

    /// An interaction spanning execution domains.
    Distributed,

    /// No locality constraint.
    NonLocal,

    /// Locality is deferred.
    Unknown,
}

impl LocalityKind {
    /// Returns whether this is a finite local interaction.
    #[must_use]
    pub const fn is_local(self) -> bool {
        matches!(
            self,
            Self::Unary | Self::Binary | Self::MultiQubit | Self::KLocal
        )
    }

    /// Returns whether this is explicitly global.
    #[must_use]
    pub const fn is_global(self) -> bool {
        matches!(self, Self::Global)
    }

    /// Returns whether this is distributed.
    #[must_use]
    pub const fn is_distributed(self) -> bool {
        matches!(self, Self::Distributed)
    }

    /// Returns whether there is no locality restriction.
    #[must_use]
    pub const fn is_non_local(self) -> bool {
        matches!(self, Self::NonLocal)
    }

    /// Returns whether locality is unknown.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

impl fmt::Display for LocalityKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Unary => "unary",
            Self::Binary => "binary",
            Self::MultiQubit => "multi_qubit",
            Self::KLocal => "k_local",
            Self::Global => "global",
            Self::Distributed => "distributed",
            Self::NonLocal => "non_local",
            Self::Unknown => "unknown",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Locality error
// =============================================================================

/// Errors produced while constructing or validating locality semantics.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LocalityError {
    /// A finite range had an invalid lower/upper bound.
    InvalidArityRange {
        /// Invalid lower bound.
        min: u64,

        /// Invalid upper bound.
        max: u64,
    },

    /// A concrete participant list contains more entries than its declared
    /// finite arity.
    ArityExceeded {
        /// Declared locality arity.
        arity: LocalityArity,

        /// Actual participant count.
        actual: u64,
    },

    /// A concrete participant list contains fewer entries than required.
    ArityInsufficient {
        /// Declared locality arity.
        arity: LocalityArity,

        /// Actual participant count.
        actual: u64,
    },

    /// Duplicate logical resources were supplied where unique participants
    /// are required.
    DuplicateLogicalQubit(QubitId),

    /// Duplicate physical resources were supplied where unique participants
    /// are required.
    DuplicatePhysicalQubit(PhysicalQubitId),

    /// The locality scope is incompatible with the requested arity.
    ScopeArityMismatch {
        /// Locality scope.
        scope: LocalityScope,

        /// Requested arity.
        arity: LocalityArity,
    },

    /// A pairwise locality requirement did not receive exactly two
    /// participants.
    PairwiseRequiresTwoParticipants,

    /// A unary locality requirement did not receive exactly one participant.
    UnaryRequiresOneParticipant,
}

impl fmt::Display for LocalityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArityRange { min, max } => {
                write!(
                    formatter,
                    "invalid locality arity range: minimum {min} exceeds maximum {max}"
                )
            }

            Self::ArityExceeded { arity, actual } => {
                write!(
                    formatter,
                    "locality arity {arity} does not accept {actual} participants"
                )
            }

            Self::ArityInsufficient { arity, actual } => {
                write!(
                    formatter,
                    "locality arity {arity} does not accept {actual} participants"
                )
            }

            Self::DuplicateLogicalQubit(qubit) => {
                write!(formatter, "duplicate logical qubit in locality: {qubit}")
            }

            Self::DuplicatePhysicalQubit(qubit) => {
                write!(formatter, "duplicate physical qubit in locality: {qubit}")
            }

            Self::ScopeArityMismatch { scope, arity } => {
                write!(
                    formatter,
                    "locality scope {scope} is incompatible with arity {arity}"
                )
            }

            Self::PairwiseRequiresTwoParticipants => {
                formatter.write_str("pairwise locality requires exactly two participants")
            }

            Self::UnaryRequiresOneParticipant => {
                formatter.write_str("unary locality requires exactly one participant")
            }
        }
    }
}

impl std::error::Error for LocalityError {}

// =============================================================================
// Locality requirement
// =============================================================================

/// Complete semantic locality requirement.
///
/// This is the primary type consumed by resource analysis, routing and target
/// compatibility layers.
///
/// It deliberately contains no topology information.
///
/// # Example
///
/// A two-qubit interaction:
///
/// ```
/// use crate::quantum::ir::resources::locality::{
///     LocalityArity,
///     LocalityRequirement,
///     LocalityScope,
/// };
///
/// let requirement = LocalityRequirement::new(
///     LocalityScope::Pairwise,
///     LocalityArity::exact(2),
/// ).expect("valid locality");
///
/// assert!(requirement.accepts_participant_count(2));
/// assert!(!requirement.accepts_participant_count(3));
/// ```
///
/// The operation is pairwise semantically. Whether the target hardware can
/// execute the pair directly is a separate topology/routing question.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LocalityRequirement {
    scope: LocalityScope,
    arity: LocalityArity,
}

impl LocalityRequirement {
    /// Creates a validated locality requirement.
    pub const fn new(
        scope: LocalityScope,
        arity: LocalityArity,
    ) -> Result<Self, LocalityError> {
        match scope {
            LocalityScope::Pairwise => {
                if !matches!(arity, LocalityArity::Exact(2)) {
                    return Err(LocalityError::PairwiseRequiresTwoParticipants);
                }
            }

            LocalityScope::Local
            | LocalityScope::MultiResource
            | LocalityScope::Distributed => {
                // These scopes permit a broad range of finite or unbounded
                // participant counts.
            }

            LocalityScope::Global | LocalityScope::NonLocal | LocalityScope::Unknown => {
                // These scopes intentionally do not impose a fixed finite
                // participant count.
            }
        }

        Ok(Self { scope, arity })
    }

    /// Creates a unary locality requirement.
    pub const fn unary() -> Result<Self, LocalityError> {
        Self::new(LocalityScope::Local, LocalityArity::Exact(1))
    }

    /// Creates a pairwise locality requirement.
    pub const fn pairwise() -> Result<Self, LocalityError> {
        Self::new(LocalityScope::Pairwise, LocalityArity::Exact(2))
    }

    /// Creates a finite k-local requirement.
    pub const fn k_local(k: u64) -> Result<Self, LocalityError> {
        Self::new(LocalityScope::MultiResource, LocalityArity::Exact(k))
    }

    /// Creates an at-least-k local requirement.
    pub const fn at_least(k: u64) -> Result<Self, LocalityError> {
        Self::new(
            LocalityScope::Local,
            LocalityArity::AtLeast(k),
        )
    }

    /// Creates a globally acting requirement.
    pub const fn global() -> Result<Self, LocalityError> {
        Self::new(LocalityScope::Global, LocalityArity::Any)
    }

    /// Creates a distributed requirement.
    pub const fn distributed() -> Result<Self, LocalityError> {
        Self::new(LocalityScope::Distributed, LocalityArity::Any)
    }

    /// Creates a locality-free requirement.
    pub const fn non_local() -> Result<Self, LocalityError> {
        Self::new(LocalityScope::NonLocal, LocalityArity::Any)
    }

    /// Creates an unresolved locality requirement.
    pub const fn unknown() -> Result<Self, LocalityError> {
        Self::new(LocalityScope::Unknown, LocalityArity::Any)
    }

    /// Returns the semantic scope.
    #[must_use]
    pub const fn scope(self) -> LocalityScope {
        self.scope
    }

    /// Returns the participant arity.
    #[must_use]
    pub const fn arity(self) -> LocalityArity {
        self.arity
    }

    /// Returns whether a participant count is accepted.
    #[must_use]
    pub const fn accepts_participant_count(self, count: u64) -> bool {
        self.arity.accepts(count)
    }

    /// Returns the convenience locality classification.
    #[must_use]
    pub const fn kind(self) -> LocalityKind {
        match self.scope {
            LocalityScope::Pairwise => LocalityKind::Binary,

            LocalityScope::Global => LocalityKind::Global,

            LocalityScope::Distributed => LocalityKind::Distributed,

            LocalityScope::NonLocal => LocalityKind::NonLocal,

            LocalityScope::Unknown => LocalityKind::Unknown,

            LocalityScope::Local | LocalityScope::MultiResource => {
                match self.arity {
                    LocalityArity::Exact(1) => LocalityKind::Unary,

                    LocalityArity::Exact(2) => LocalityKind::Binary,

                    LocalityArity::Exact(_) => LocalityKind::MultiQubit,

                    LocalityArity::AtLeast(_) => LocalityKind::KLocal,

                    LocalityArity::Between { .. } => LocalityKind::KLocal,

                    LocalityArity::Any => LocalityKind::KLocal,
                }
            }
        }
    }

    /// Returns whether this requirement is finite-local.
    #[must_use]
    pub const fn is_local(self) -> bool {
        self.scope.is_local()
    }

    /// Returns whether this requirement is global.
    #[must_use]
    pub const fn is_global(self) -> bool {
        self.scope.is_global()
    }

    /// Returns whether this requirement is distributed.
    #[must_use]
    pub const fn is_distributed(self) -> bool {
        self.scope.is_distributed()
    }

    /// Returns whether this requirement has no locality restriction.
    #[must_use]
    pub const fn is_non_local(self) -> bool {
        self.scope.is_non_local()
    }

    /// Returns whether locality must be resolved later.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        self.scope.is_unknown()
    }
}

impl fmt::Display for LocalityRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} ({})", self.scope, self.arity)
    }
}

// =============================================================================
// Participant identity
// =============================================================================

/// A resource participant in a locality relation.
///
/// The canonical quantum identities are used directly.
///
/// The enum is intentionally extensible through the resource layer rather
/// than by adding hardware concepts here.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LocalityParticipant {
    /// Logical quantum resource.
    LogicalQubit(QubitId),

    /// Physical target resource.
    PhysicalQubit(PhysicalQubitId),
}

impl LocalityParticipant {
    /// Returns the logical qubit when this participant is logical.
    #[must_use]
    pub const fn logical_qubit(self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(qubit) => Some(qubit),
            Self::PhysicalQubit(_) => None,
        }
    }

    /// Returns the physical qubit when this participant is physical.
    #[must_use]
    pub const fn physical_qubit(self) -> Option<PhysicalQubitId> {
        match self {
            Self::LogicalQubit(_) => None,
            Self::PhysicalQubit(qubit) => Some(qubit),
        }
    }

    /// Returns whether the participant is logical.
    #[must_use]
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns whether the participant is physical.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }
}

impl From<QubitId> for LocalityParticipant {
    fn from(qubit: QubitId) -> Self {
        Self::LogicalQubit(qubit)
    }
}

impl From<PhysicalQubitId> for LocalityParticipant {
    fn from(qubit: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(qubit)
    }
}

impl fmt::Display for LocalityParticipant {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LogicalQubit(qubit) => write!(formatter, "{qubit}"),
            Self::PhysicalQubit(qubit) => write!(formatter, "{qubit}"),
        }
    }
}

// =============================================================================
// Qubit interaction
// =============================================================================

/// A concrete logical-qubit interaction.
///
/// This type represents the actual semantic participants of an operation.
///
/// It does not state whether the participants are physically adjacent.
///
/// The routing layer may later use this interaction with a hardware topology.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QubitInteraction {
    qubits: Vec<QubitId>,
}

impl QubitInteraction {
    /// Creates an interaction from logical qubits.
    ///
    /// Duplicate qubits are rejected because an interaction participant list
    /// must identify distinct logical resources.
    pub fn new<I>(qubits: I) -> Result<Self, LocalityError>
    where
        I: IntoIterator<Item = QubitId>,
    {
        let mut values = Vec::new();

        for qubit in qubits {
            if values.contains(&qubit) {
                return Err(LocalityError::DuplicateLogicalQubit(qubit));
            }

            values.push(qubit);
        }

        Ok(Self { qubits: values })
    }

    /// Creates a unary interaction.
    pub fn unary(qubit: QubitId) -> Self {
        Self {
            qubits: vec![qubit],
        }
    }

    /// Creates a pairwise interaction.
    ///
    /// Returns an error if both arguments identify the same logical qubit.
    pub fn pairwise(
        first: QubitId,
        second: QubitId,
    ) -> Result<Self, LocalityError> {
        Self::new([first, second])
    }

    /// Returns the number of participants.
    #[must_use]
    pub fn len(&self) -> usize {
        self.qubits.len()
    }

    /// Returns whether the interaction contains no participants.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.qubits.is_empty()
    }

    /// Returns the participant slice.
    #[must_use]
    pub fn as_slice(&self) -> &[QubitId] {
        &self.qubits
    }

    /// Returns the participant at an index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<QubitId> {
        self.qubits.get(index).copied()
    }

    /// Returns whether the interaction contains a qubit.
    #[must_use]
    pub fn contains(&self, qubit: QubitId) -> bool {
        self.qubits.contains(&qubit)
    }

    /// Validates the interaction against a locality requirement.
    pub fn validate_against(
        &self,
        requirement: LocalityRequirement,
    ) -> Result<(), LocalityError> {
        let count = u64::try_from(self.qubits.len()).unwrap_or(u64::MAX);

        if requirement.accepts_participant_count(count) {
            Ok(())
        } else if count < requirement.arity().min() {
            Err(LocalityError::ArityInsufficient {
                arity: requirement.arity(),
                actual: count,
            })
        } else {
            Err(LocalityError::ArityExceeded {
                arity: requirement.arity(),
                actual: count,
            })
        }
    }

    /// Returns an iterator over logical qubits.
    pub fn iter(&self) -> impl Iterator<Item = QubitId> + '_ {
        self.qubits.iter().copied()
    }
}

impl AsRef<[QubitId]> for QubitInteraction {
    fn as_ref(&self) -> &[QubitId] {
        self.as_slice()
    }
}

impl IntoIterator for QubitInteraction {
    type Item = QubitId;
    type IntoIter = std::vec::IntoIter<QubitId>;

    fn into_iter(self) -> Self::IntoIter {
        self.qubits.into_iter()
    }
}

// =============================================================================
// Physical interaction
// =============================================================================

/// A concrete physical-qubit interaction.
///
/// This is useful at the target-lowering boundary.
///
/// It does not prove that the physical qubits are connected. Connectivity is
/// owned by the hardware topology layer.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PhysicalQubitInteraction {
    qubits: Vec<PhysicalQubitId>,
}

impl PhysicalQubitInteraction {
    /// Creates an interaction from physical qubits.
    ///
    /// Duplicate physical resources are rejected.
    pub fn new<I>(qubits: I) -> Result<Self, LocalityError>
    where
        I: IntoIterator<Item = PhysicalQubitId>,
    {
        let mut values = Vec::new();

        for qubit in qubits {
            if values.contains(&qubit) {
                return Err(LocalityError::DuplicatePhysicalQubit(qubit));
            }

            values.push(qubit);
        }

        Ok(Self { qubits: values })
    }

    /// Returns the number of participants.
    #[must_use]
    pub fn len(&self) -> usize {
        self.qubits.len()
    }

    /// Returns whether the interaction is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.qubits.is_empty()
    }

    /// Returns the participant slice.
    #[must_use]
    pub fn as_slice(&self) -> &[PhysicalQubitId] {
        &self.qubits
    }

    /// Returns whether the interaction contains a physical qubit.
    #[must_use]
    pub fn contains(&self, qubit: PhysicalQubitId) -> bool {
        self.qubits.contains(&qubit)
    }

    /// Returns an iterator over physical participants.
    pub fn iter(&self) -> impl Iterator<Item = PhysicalQubitId> + '_ {
        self.qubits.iter().copied()
    }

    /// Validates the interaction against a locality requirement.
    pub fn validate_against(
        &self,
        requirement: LocalityRequirement,
    ) -> Result<(), LocalityError> {
        let count = u64::try_from(self.qubits.len()).unwrap_or(u64::MAX);

        if requirement.accepts_participant_count(count) {
            Ok(())
        } else if count < requirement.arity().min() {
            Err(LocalityError::ArityInsufficient {
                arity: requirement.arity(),
                actual: count,
            })
        } else {
            Err(LocalityError::ArityExceeded {
                arity: requirement.arity(),
                actual: count,
            })
        }
    }
}

impl AsRef<[PhysicalQubitId]> for PhysicalQubitInteraction {
    fn as_ref(&self) -> &[PhysicalQubitId] {
        self.as_slice()
    }
}

impl IntoIterator for PhysicalQubitInteraction {
    type Item = PhysicalQubitId;
    type IntoIter = std::vec::IntoIter<PhysicalQubitId>;

    fn into_iter(self) -> Self::IntoIter {
        self.qubits.into_iter()
    }
}

// =============================================================================
// Locality relation
// =============================================================================

/// Semantic relation between two resource participants.
///
/// This is deliberately independent of physical topology.
///
/// `Adjacent`, for example, is NOT represented here because adjacency is a
/// hardware-topology property rather than a canonical semantic locality
/// property.
///
/// Instead, the routing/hardware layer determines whether a local interaction
/// can be realized directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LocalityRelation {
    /// Participants belong to the same semantic locality domain.
    SameDomain,

    /// Participants form a local semantic interaction.
    Local,

    /// Participants have no semantic locality requirement.
    Unrestricted,

    /// Participants may belong to different execution domains.
    Distributed,

    /// Relation is intentionally deferred.
    Unknown,
}

impl LocalityRelation {
    /// Returns whether the relation requires a local domain.
    #[must_use]
    pub const fn is_local(self) -> bool {
        matches!(self, Self::SameDomain | Self::Local)
    }

    /// Returns whether the relation permits unrestricted interaction.
    #[must_use]
    pub const fn is_unrestricted(self) -> bool {
        matches!(self, Self::Unrestricted)
    }

    /// Returns whether the relation is distributed.
    #[must_use]
    pub const fn is_distributed(self) -> bool {
        matches!(self, Self::Distributed)
    }

    /// Returns whether the relation is unresolved.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

impl fmt::Display for LocalityRelation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::SameDomain => "same_domain",
            Self::Local => "local",
            Self::Unrestricted => "unrestricted",
            Self::Distributed => "distributed",
            Self::Unknown => "unknown",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Locality constraint
// =============================================================================

/// A complete locality constraint for a semantic operation.
///
/// `LocalityRequirement` answers how many participants may be involved.
///
/// `LocalityConstraint` additionally records the semantic relation between
/// those participants.
///
/// No physical distance or hardware graph is embedded.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LocalityConstraint {
    requirement: LocalityRequirement,
    relation: LocalityRelation,
}

impl LocalityConstraint {
    /// Creates a locality constraint.
    #[must_use]
    pub const fn new(
        requirement: LocalityRequirement,
        relation: LocalityRelation,
    ) -> Self {
        Self {
            requirement,
            relation,
        }
    }

    /// Creates a unary local constraint.
    pub const fn unary() -> Result<Self, LocalityError> {
        match LocalityRequirement::unary() {
            Ok(requirement) => Ok(Self::new(
                requirement,
                LocalityRelation::Local,
            )),
            Err(error) => Err(error),
        }
    }

    /// Creates a pairwise local constraint.
    pub const fn pairwise() -> Result<Self, LocalityError> {
        match LocalityRequirement::pairwise() {
            Ok(requirement) => Ok(Self::new(
                requirement,
                LocalityRelation::Local,
            )),
            Err(error) => Err(error),
        }
    }

    /// Creates a global constraint.
    pub const fn global() -> Result<Self, LocalityError> {
        match LocalityRequirement::global() {
            Ok(requirement) => Ok(Self::new(
                requirement,
                LocalityRelation::Unrestricted,
            )),
            Err(error) => Err(error),
        }
    }

    /// Creates a distributed constraint.
    pub const fn distributed() -> Result<Self, LocalityError> {
        match LocalityRequirement::distributed() {
            Ok(requirement) => Ok(Self::new(
                requirement,
                LocalityRelation::Distributed,
            )),
            Err(error) => Err(error),
        }
    }

    /// Returns the locality requirement.
    #[must_use]
    pub const fn requirement(self) -> LocalityRequirement {
        self.requirement
    }

    /// Returns the semantic participant relation.
    #[must_use]
    pub const fn relation(self) -> LocalityRelation {
        self.relation
    }

    /// Returns the locality classification.
    #[must_use]
    pub const fn kind(self) -> LocalityKind {
        self.requirement.kind()
    }

    /// Returns whether the constraint is finite-local.
    #[must_use]
    pub const fn is_local(self) -> bool {
        self.requirement.is_local()
    }

    /// Returns whether the constraint is distributed.
    #[must_use]
    pub const fn is_distributed(self) -> bool {
        self.requirement.is_distributed()
    }

    /// Returns whether a participant count is accepted.
    #[must_use]
    pub const fn accepts_participant_count(self, count: u64) -> bool {
        self.requirement.accepts_participant_count(count)
    }
}

// =============================================================================
// Locality descriptor
// =============================================================================

/// Immutable semantic locality descriptor.
///
/// This is useful when an operation, resource declaration or model needs to
/// carry locality metadata without carrying concrete participants.
///
/// It can therefore describe very large or symbolic programs without
/// materializing all resources.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LocalityDescriptor {
    constraint: LocalityConstraint,
}

impl LocalityDescriptor {
    /// Creates a descriptor from a locality constraint.
    #[must_use]
    pub const fn new(constraint: LocalityConstraint) -> Self {
        Self { constraint }
    }

    /// Creates a unary descriptor.
    pub const fn unary() -> Result<Self, LocalityError> {
        match LocalityConstraint::unary() {
            Ok(value) => Ok(Self::new(value)),
            Err(error) => Err(error),
        }
    }

    /// Creates a pairwise descriptor.
    pub const fn pairwise() -> Result<Self, LocalityError> {
        match LocalityConstraint::pairwise() {
            Ok(value) => Ok(Self::new(value)),
            Err(error) => Err(error),
        }
    }

    /// Creates a global descriptor.
    pub const fn global() -> Result<Self, LocalityError> {
        match LocalityConstraint::global() {
            Ok(value) => Ok(Self::new(value)),
            Err(error) => Err(error),
        }
    }

    /// Creates a distributed descriptor.
    pub const fn distributed() -> Result<Self, LocalityError> {
        match LocalityConstraint::distributed() {
            Ok(value) => Ok(Self::new(value)),
            Err(error) => Err(error),
        }
    }

    /// Returns the underlying constraint.
    #[must_use]
    pub const fn constraint(self) -> LocalityConstraint {
        self.constraint
    }

    /// Returns the locality kind.
    #[must_use]
    pub const fn kind(self) -> LocalityKind {
        self.constraint.kind()
    }

    /// Returns the participant arity.
    #[must_use]
    pub const fn arity(self) -> LocalityArity {
        self.constraint.requirement().arity()
    }

    /// Returns the participant relation.
    #[must_use]
    pub const fn relation(self) -> LocalityRelation {
        self.constraint.relation()
    }
}

// =============================================================================
// Locality validation helpers
// =============================================================================

/// Validates a logical-qubit interaction against a locality requirement.
///
/// This helper is intentionally pure. It does not inspect hardware topology.
pub fn validate_logical_interaction(
    interaction: &QubitInteraction,
    requirement: LocalityRequirement,
) -> Result<(), LocalityError> {
    interaction.validate_against(requirement)
}

/// Validates a physical-qubit interaction against a locality requirement.
///
/// This helper does not establish physical connectivity.
///
/// A successful result means only that the number of participants satisfies
/// the semantic locality requirement.
pub fn validate_physical_interaction(
    interaction: &PhysicalQubitInteraction,
    requirement: LocalityRequirement,
) -> Result<(), LocalityError> {
    interaction.validate_against(requirement)
}

/// Validates a logical-qubit interaction against a complete locality
/// constraint.
///
/// Hardware topology is intentionally not consulted.
pub fn validate_logical_constraint(
    interaction: &QubitInteraction,
    constraint: LocalityConstraint,
) -> Result<(), LocalityError> {
    interaction.validate_against(constraint.requirement())
}

/// Validates a physical-qubit interaction against a complete locality
/// constraint.
///
/// Hardware topology is intentionally not consulted.
pub fn validate_physical_constraint(
    interaction: &PhysicalQubitInteraction,
    constraint: LocalityConstraint,
) -> Result<(), LocalityError> {
    interaction.validate_against(constraint.requirement())
}

// =============================================================================
// Standard semantic constructors
// =============================================================================

/// Returns the canonical unary locality requirement.
#[must_use]
pub const fn unary_locality() -> LocalityRequirement {
    match LocalityRequirement::unary() {
        Ok(value) => value,
        Err(_) => {
            // The constructor is statically known to be valid.
            //
            // This branch is unreachable because Exact(1) is valid for Local.
            LocalityRequirement {
                scope: LocalityScope::Local,
                arity: LocalityArity::Exact(1),
            }
        }
    }
}

/// Returns the canonical pairwise locality requirement.
#[must_use]
pub const fn pairwise_locality() -> LocalityRequirement {
    match LocalityRequirement::pairwise() {
        Ok(value) => value,
        Err(_) => {
            // Exact(2) is the only valid arity for Pairwise.
            LocalityRequirement {
                scope: LocalityScope::Pairwise,
                arity: LocalityArity::Exact(2),
            }
        }
    }
}

/// Returns the canonical global locality requirement.
#[must_use]
pub const fn global_locality() -> LocalityRequirement {
    match LocalityRequirement::global() {
        Ok(value) => value,
        Err(_) => {
            // Global/Any is always valid.
            LocalityRequirement {
                scope: LocalityScope::Global,
                arity: LocalityArity::Any,
            }
        }
    }
}

/// Returns the canonical distributed locality requirement.
#[must_use]
pub const fn distributed_locality() -> LocalityRequirement {
    match LocalityRequirement::distributed() {
        Ok(value) => value,
        Err(_) => {
            // Distributed/Any is always valid.
            LocalityRequirement {
                scope: LocalityScope::Distributed,
                arity: LocalityArity::Any,
            }
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_arity_accepts_only_exact_count() {
        let arity = LocalityArity::exact(2);

        assert!(!arity.accepts(0));
        assert!(!arity.accepts(1));
        assert!(arity.accepts(2));
        assert!(!arity.accepts(3));
    }

    #[test]
    fn at_least_arity_has_no_finite_upper_bound() {
        let arity = LocalityArity::at_least(2);

        assert!(!arity.accepts(0));
        assert!(!arity.accepts(1));
        assert!(arity.accepts(2));
        assert!(arity.accepts(3));
        assert!(arity.accepts(u64::MAX));
        assert!(arity.is_unbounded());
    }

    #[test]
    fn between_rejects_invalid_bounds() {
        assert!(LocalityArity::between(3, 2).is_none());
        assert!(LocalityArity::between(2, 3).is_some());
    }

    #[test]
    fn unary_requirement_is_unary() {
        let requirement = LocalityRequirement::unary()
            .expect("unary locality must be valid");

        assert_eq!(requirement.kind(), LocalityKind::Unary);
        assert!(requirement.is_local());
        assert!(requirement.accepts_participant_count(1));
        assert!(!requirement.accepts_participant_count(2));
    }

    #[test]
    fn pairwise_requirement_is_binary() {
        let requirement = LocalityRequirement::pairwise()
            .expect("pairwise locality must be valid");

        assert_eq!(requirement.kind(), LocalityKind::Binary);
        assert!(requirement.accepts_participant_count(2));
        assert!(!requirement.accepts_participant_count(1));
        assert!(!requirement.accepts_participant_count(3));
    }

    #[test]
    fn pairwise_rejects_non_binary_arity() {
        let result = LocalityRequirement::new(
            LocalityScope::Pairwise,
            LocalityArity::exact(3),
        );

        assert_eq!(
            result,
            Err(LocalityError::PairwiseRequiresTwoParticipants)
        );
    }

    #[test]
    fn global_requirement_is_unbounded() {
        let requirement =
            LocalityRequirement::global().expect("global locality must be valid");

        assert!(requirement.is_global());
        assert!(requirement.arity().is_unbounded());
        assert!(requirement.accepts_participant_count(0));
        assert!(requirement.accepts_participant_count(1));
        assert!(requirement.accepts_participant_count(u64::MAX));
    }

    #[test]
    fn distributed_requirement_is_unbounded() {
        let requirement = LocalityRequirement::distributed()
            .expect("distributed locality must be valid");

        assert!(requirement.is_distributed());
        assert!(requirement.arity().is_unbounded());
    }

    #[test]
    fn logical_interaction_rejects_duplicates() {
        let q0 = QubitId::new(0);

        let result = QubitInteraction::new([q0, q0]);

        assert_eq!(
            result,
            Err(LocalityError::DuplicateLogicalQubit(q0))
        );
    }

    #[test]
    fn physical_interaction_rejects_duplicates() {
        let q0 = PhysicalQubitId::new(0);

        let result = PhysicalQubitInteraction::new([q0, q0]);

        assert_eq!(
            result,
            Err(LocalityError::DuplicatePhysicalQubit(q0))
        );
    }

    #[test]
    fn pairwise_interaction_validates() {
        let q0 = QubitId::new(0);
        let q1 = QubitId::new(1);

        let interaction =
            QubitInteraction::pairwise(q0, q1)
                .expect("distinct qubits must form a pair");

        let locality =
            LocalityRequirement::pairwise()
                .expect("pairwise locality must be valid");

        assert!(interaction.validate_against(locality).is_ok());
    }

    #[test]
    fn unary_interaction_rejects_pairwise_requirement() {
        let q0 = QubitId::new(0);

        let interaction = QubitInteraction::unary(q0);

        let locality =
            LocalityRequirement::pairwise()
                .expect("pairwise locality must be valid");

        assert!(matches!(
            interaction.validate_against(locality),
            Err(LocalityError::ArityInsufficient { .. })
        ));
    }

    #[test]
    fn multi_qubit_interaction_is_supported() {
        let qubits = [
            QubitId::new(0),
            QubitId::new(1),
            QubitId::new(2),
            QubitId::new(3),
        ];

        let interaction =
            QubitInteraction::new(qubits)
                .expect("unique qubits must form an interaction");

        let locality =
            LocalityRequirement::k_local(4)
                .expect("4-local locality must be valid");

        assert!(interaction.validate_against(locality).is_ok());
        assert_eq!(interaction.len(), 4);
    }

    #[test]
    fn locality_supports_large_semantic_arity_without_machine_limit() {
        let locality =
            LocalityRequirement::at_least(1_000_000_000);

        assert!(
            locality
                .expect("large locality must be representable")
                .accepts_participant_count(1_000_000_000)
        );
    }

    #[test]
    fn global_locality_supports_maximum_u64_count_semantically() {
        let locality = global_locality();

        assert!(locality.accepts_participant_count(u64::MAX));
    }

    #[test]
    fn participant_conversion_uses_canonical_qubit_types() {
        let logical = QubitId::new(7);
        let physical = PhysicalQubitId::new(11);

        let logical_participant = LocalityParticipant::from(logical);
        let physical_participant = LocalityParticipant::from(physical);

        assert_eq!(
            logical_participant.logical_qubit(),
            Some(logical)
        );

        assert_eq!(
            physical_participant.physical_qubit(),
            Some(physical)
        );
    }

    #[test]
    fn locality_constraint_preserves_relation() {
        let requirement =
            LocalityRequirement::pairwise()
                .expect("pairwise locality must be valid");

        let constraint =
            LocalityConstraint::new(
                requirement,
                LocalityRelation::Local,
            );

        assert!(constraint.is_local());
        assert_eq!(constraint.kind(), LocalityKind::Binary);
        assert_eq!(
            constraint.relation(),
            LocalityRelation::Local
        );
    }

    #[test]
    fn descriptor_is_copy_and_semantically_stable() {
        let descriptor =
            LocalityDescriptor::pairwise()
                .expect("pairwise descriptor must be valid");

        let copied = descriptor;

        assert_eq!(descriptor.kind(), LocalityKind::Binary);
        assert_eq!(descriptor.kind(), copied.kind());
        assert_eq!(descriptor.arity(), copied.arity());
    }

    #[test]
    fn physical_validation_does_not_imply_connectivity() {
        let p0 = PhysicalQubitId::new(0);
        let p1000 = PhysicalQubitId::new(1000);

        let interaction =
            PhysicalQubitInteraction::new([p0, p1000])
                .expect("distinct physical identifiers are valid");

        let locality =
            LocalityRequirement::pairwise()
                .expect("pairwise locality must be valid");

        // This test intentionally verifies only semantic arity.
        //
        // Whether p0 and p1000 are physically connected is NOT a question
        // answered by this module.
        assert!(interaction.validate_against(locality).is_ok());
    }
}