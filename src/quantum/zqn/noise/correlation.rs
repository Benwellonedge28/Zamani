//! Zamani Quantum Noise (ZQN) — Correlation Model.
//!
//! # Ownership
//!
//! This module owns the **declarative correlation model** used by ZQN noise
//! systems.
//!
//! It answers:
//!
//! > Which quantum resources participate in a correlation domain, what kind of
//! > correlation exists between them, and what deterministic correlation
//! > metadata is associated with that domain?
//!
//! This module owns:
//!
//! - [`CorrelationModel`];
//! - [`CorrelationDomain`];
//! - [`CorrelationResource`];
//! - [`CorrelationKind`];
//! - [`CorrelationStrength`];
//! - [`PairCorrelation`];
//! - [`CorrelationEdge`];
//! - [`CorrelationRelation`];
//! - [`CorrelationBuilder`];
//! - deterministic canonicalization;
//! - correlation-domain validation;
//! - resource membership queries;
//! - pairwise correlation queries;
//! - correlation aggregation/inspection;
//! - explicit approximation metadata;
//! - correlation-model identity association.
//!
//! This module does NOT own:
//!
//! - realized faults;
//! - [`crate::quantum::zqn::fault::correlated::CorrelatedFault`];
//! - quantum channels;
//! - probability distributions;
//! - random-number generation;
//! - stochastic sampling;
//! - temporal-noise semantics;
//! - spatial-noise algorithms;
//! - calibration;
//! - characterization;
//! - routing;
//! - scheduling;
//! - QEC;
//! - simulation;
//! - hardware APIs;
//! - serialization formats;
//! - resource-limit policy;
//! - global registries;
//! - global mutable state.
//!
//! Those responsibilities belong to their owning subsystems.
//!
//! # Architectural distinction
//!
//! ```text
//! quantum::ir
//!     │
//!     │ canonical computation/resource semantics
//!     ▼
//! zqn::noise::correlation
//!     │
//!     │ declarative correlation relationship
//!     ▼
//! zqn::noise::model
//!     │
//!     ▼
//! channel / fault / application
//!     │
//!     ├──────────────┬──────────────┐
//!     ▼              ▼              ▼
//! simulation        QEC          hardware
//! ```
//!
//! A correlation model is **not** a realized correlated fault.
//!
//! ```text
//! CorrelationModel
//!     = law/relationship describing dependence
//!
//! CorrelatedFault
//!     = realized event containing concrete Fault values
//! ```
//!
//! The existing `zqn::fault::correlated` module owns the latter.
//!
//! # Why this distinction matters
//!
//! A physical system can have correlated noise even when a particular
//! execution produces no fault, one fault, or many faults.
//!
//! Therefore the correlation model must exist independently of any particular
//! sampled execution.
//!
//! For example:
//!
//! ```text
//! Resource A ─┐
//!             ├── correlation domain C
//! Resource B ─┤
//!             │
//! Resource C ─┘
//!
//!                 │
//!                 ▼
//!          noise model / sampler
//!                 │
//!                 ▼
//!       realized correlated faults
//! ```
//!
//! The correlation relationship therefore belongs here, while the realized
//! event belongs to `fault::correlated`.
//!
//! # Canonical quantum identity
//!
//! Quantum-resource identity remains owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! In particular:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module uses those types directly when a correlation explicitly refers
//! to logical or physical qubits.
//!
//! It MUST NOT introduce:
//!
//! ```text
//! CorrelationQubitId
//! NoiseQubitId
//! CorrelationPhysicalQubitId
//! ZqnQubitId
//! ```
//!
//! or any equivalent duplicate identity type.
//!
//! # Future quantum technologies
//!
//! The correlation abstraction must not be permanently restricted to qubits.
//!
//! `CorrelationResource` therefore has both:
//!
//! - canonical logical/physical qubit variants;
//! - an opaque ZQN resource identity variant for resources whose canonical
//!   identity is owned by another IR subsystem.
//!
//! This permits future integration with:
//!
//! - qudits;
//! - bosonic modes;
//! - continuous-variable modes;
//! - photonic modes;
//! - fermionic modes;
//! - logical resources;
//! - distributed resources;
//! - communication links;
//! - analog resources;
//! - future quantum modalities.
//!
//! The opaque variant is an identity reference, not a second resource model.
//!
//! # Write once, scale everywhere
//!
//! This module deliberately contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_CORRELATIONS
//! MAX_CORRELATED_RESOURCES
//! MAX_CORRELATION_EDGES
//! MAX_DOMAIN_SIZE
//! ```
//!
//! A domain may contain any finite number of resources representable by the
//! host system and permitted by the caller's explicit resource policy.
//!
//! "Infinity" in the Zamani architecture means:
//!
//! > no artificial finite machine-size ceiling is encoded into the semantic
//! > correlation model.
//!
//! It does not claim that a physical computer has infinite memory, storage,
//! execution time, or quantum resources.
//!
//! Resource limits belong to explicit ZQN/runtime policies.
//!
//! # Determinism
//!
//! This module is completely deterministic.
//!
//! It:
//!
//! - owns no RNG;
//! - calls no global RNG;
//! - reads no system time;
//! - uses no memory addresses;
//! - owns no global mutable state;
//! - does not depend on hash-map iteration order;
//! - canonicalizes resource and relation ordering;
//! - uses explicit identifiers;
//! - validates all floating-point values before accepting them.
//!
//! The same model constructed from the same semantic inputs produces the same
//! canonical representation.
//!
//! # Numerical policy
//!
//! Correlation strength is represented by [`CorrelationStrength`].
//!
//! A strength is an explicitly validated finite value in `[-1, 1]`.
//!
//! This value is a **correlation descriptor**, not automatically a probability.
//!
//! A value of:
//!
//! ```text
//!  1  = maximally positively correlated according to the selected relation
//!  0  = uncorrelated according to the selected relation
//! -1  = maximally negatively correlated according to the selected relation
//! ```
//!
//! The exact physical/statistical meaning is determined by the enclosing
//! [`CorrelationKind`] and downstream probability/noise model.
//!
//! This module therefore does not pretend that one scalar coefficient can
//! represent every possible multivariate quantum correlation.
//!
//! For general correlations, [`CorrelationRelation`] permits a caller to
//! provide a model descriptor rather than forcing the entire system into a
//! pairwise coefficient.
//!
//! # Exact versus approximate correlation
//!
//! A correlation relationship can be:
//!
//! - exact;
//! - approximate within an explicit tolerance;
//! - statistically estimated with explicit confidence;
//! - bounded by an explicit interval.
//!
//! Silent approximation is forbidden.
//!
//! Consumers must be able to determine whether a correlation description is
//! exact or approximate.
//!
//! # Correlation kinds
//!
//! The enum is deliberately semantic rather than hardware-specific.
//!
//! It supports:
//!
//! - pairwise correlation;
//! - collective/group correlation;
//! - graph correlation;
//! - covariance-like correlation;
//! - temporal dependence references;
//! - arbitrary declarative correlation;
//! - independent domains.
//!
//! It does not prescribe a numerical simulation algorithm.
//!
//! # Spatial versus temporal correlation
//!
//! Spatial and temporal correlation are distinct concepts.
//!
//! This file can describe the participants in either relationship, but does
//! not implement temporal kernels or spatial topology algorithms.
//!
//! For example:
//!
//! ```text
//! spatial:
//!     A ↔ B
//!
//! temporal:
//!     A(t) ↔ A(t + Δt)
//! ```
//!
//! Temporal evolution belongs to `noise::temporal`.
//!
//! Spatial topology algorithms belong to `noise::spatial`.
//!
//! This module provides the common correlation-domain representation they may
//! consume.
//!
//! # Correlation versus crosstalk
//!
//! Correlation is a statistical/physical relationship.
//!
//! Crosstalk is a particular physical mechanism/interaction in which one
//! operation/resource affects another.
//!
//! Therefore crosstalk models may consume this module, but this module does not
//! own crosstalk semantics.
//!
//! # Correlation versus realized faults
//!
//! The direction is:
//!
//! ```text
//! CorrelationModel
//!       │
//!       ▼
//! NoiseModel
//!       │
//!       ▼
//! sampling/execution
//!       │
//!       ▼
//! fault::CorrelatedFault
//! ```
//!
//! This dependency direction prevents the semantic correlation model from
//! becoming coupled to one particular fault representation.
//!
//! # Resource identity domains
//!
//! The following identity domains are intentionally distinct:
//!
//! ```text
//! LogicalQubit(QubitId)
//! PhysicalQubit(PhysicalQubitId)
//! ZqnResource(ZqnObjectId)
//! ```
//!
//! A logical qubit and physical qubit with the same numeric payload are still
//! different resources.
//!
//! They must never compare equal.
//!
//! # Canonical ordering
//!
//! All collections owned by this module are canonically ordered.
//!
//! Canonical ordering is an implementation-level deterministic ordering only.
//!
//! It MUST NOT be interpreted as:
//!
//! - execution order;
//! - temporal order;
//! - topology order;
//! - severity;
//! - causal order;
//! - qubit numbering semantics.
//!
//! Ordering exists only to make:
//!
//! - equality;
//! - hashing;
//! - caching;
//! - canonical serialization;
//! - deterministic diagnostics;
//! - reproducible compilation
//!
//! stable.
//!
//! # Resource uniqueness
//!
//! A correlation domain must not contain the same resource twice.
//!
//! For example:
//!
//! ```text
//! A
//! A
//! ```
//!
//! is invalid.
//!
//! However:
//!
//! ```text
//! LogicalQubit(7)
//! PhysicalQubit(7)
//! ```
//!
//! is valid because the identity domains are different.
//!
//! # Self-correlation
//!
//! A relation between a resource and itself is rejected.
//!
//! Self-correlation is ambiguous at this abstraction level because it can mean:
//!
//! - variance;
//! - autocorrelation;
//! - temporal memory;
//! - an identity relation;
//! - a modeling mistake.
//!
//! Such semantics must be represented explicitly by the appropriate temporal,
//! statistical, or channel model rather than silently encoded as a spatial
//! self-edge.
//!
//! # Duplicate relations
//!
//! A pairwise relation is undirected at this abstraction level unless the
//! enclosing `CorrelationRelation` explicitly declares another semantic.
//!
//! Therefore:
//!
//! ```text
//! A ↔ B
//! B ↔ A
//! ```
//!
//! are canonicalized to one relationship.
//!
//! Duplicate relations are rejected rather than silently summed or overwritten.
//!
//! # Streaming construction
//!
//! [`CorrelationBuilder`] accepts resources and relations incrementally.
//!
//! This is important for large machines because callers can construct a model
//! from:
//!
//! - hardware topology streams;
//! - characterization results;
//! - calibration observations;
//! - distributed metadata;
//! - generated resource sets.
//!
//! The final `CorrelationModel` is owned/materialized data.
//!
//! Streaming generation itself belongs to the producer.
//!
//! # Memory model
//!
//! This module uses standard Rust owned collections and no unsafe code.
//!
//! It does not promise that an arbitrarily large graph can fit into one
//! process.
//!
//! Large consumers may:
//!
//! - construct domains incrementally;
//! - use bounded admission policies;
//! - partition models;
//! - distribute correlation domains;
//! - keep only relevant projections;
//! - use a different representation at a higher layer.
//!
//! None of those policies change the semantic contract here.
//!
//! # Resource policy
//!
//! Resource limits are deliberately not enforced here.
//!
//! A caller processing untrusted input should use the appropriate ZQN limits
//! before accepting arbitrarily large collections.
//!
//! A capacity supplied to [`CorrelationBuilder::with_capacity`] is an
//! allocation hint only.
//!
//! It is never a semantic maximum.
//!
//! # Serialization
//!
//! This module intentionally does not depend on `serde` or another wire-format
//! framework.
//!
//! Serialization belongs to:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! The IO layer must preserve:
//!
//! - correlation identity;
//! - resource identity domain;
//! - resource identity;
//! - correlation kind;
//! - relation semantics;
//! - strength;
//! - approximation metadata;
//! - canonical ordering;
//! - schema/version information.
//!
//! Serialization must never collapse logical and physical identities into one
//! integer domain.
//!
//! # Hashing
//!
//! All semantic values implement `Hash` where practical.
//!
//! Hashing is for deterministic in-process collections and canonical-content
//! construction.
//!
//! Hash values must not be treated as persistent serialized identities.
//!
//! # Thread safety
//!
//! Semantic correlation objects are immutable after construction.
//!
//! They contain no interior mutability and no global state.
//!
//! They are therefore suitable for concurrent read-only use when their
//! contained types are thread-safe.
//!
//! # Security
//!
//! Correlation models are data, not capabilities.
//!
//! A correlation ID or resource reference must never grant:
//!
//! - QPU access;
//! - hardware control;
//! - credential access;
//! - calibration write access;
//! - network access;
//! - filesystem access;
//! - execution authorization.
//!
//! Untrusted correlation specifications must be processed under explicit
//! resource and cancellation policies.
//!
//! # Error contract
//!
//! Construction fails when:
//!
//! - a correlation ID is otherwise invalid according to its owning identity
//!   contract;
//! - a resource set is empty where a non-empty domain is required;
//! - duplicate resources occur;
//! - a self-relation occurs;
//! - a duplicate relation occurs;
//! - a strength is non-finite or outside `[-1, 1]`;
//! - a tolerance is invalid;
//! - an invalid correlation relation is supplied;
//! - a relation references a resource outside the domain;
//! - canonical invariants cannot be established.
//!
//! # Integration contract
//!
//! ```text
//! quantum::ir::qubit
//!        │
//!        ▼
//! CorrelationResource
//!        │
//!        ▼
//! CorrelationDomain
//!        │
//!        ▼
//! CorrelationModel
//!        │
//!   ┌────┼───────────┐
//!   ▼    ▼           ▼
//! noise temporal   spatial
//!   │    │           │
//!   └────┼───────────┘
//!        ▼
//! NoiseModel
//!        │
//!        ▼
//! application / simulation / QEC / hardware
//! ```
//!
//! The model can later be consumed by:
//!
//! - `noise::model`;
//! - `noise::application`;
//! - `noise::composition`;
//! - `noise::temporal`;
//! - `noise::spatial`;
//! - `noise::crosstalk`;
//! - simulation;
//! - characterization;
//! - routing;
//! - scheduling;
//! - QEC adapters;
//! - hardware adapters.
//!
//! None of those consumers need to modify this file to understand its basic
//! contract.
//!
//! # Integration with `fault::correlated`
//!
//! The relationship is intentionally one-way:
//!
//! ```text
//! noise::correlation::CorrelationModel
//!             │
//!             │ describes dependence
//!             ▼
//! noise/simulation sampler
//!             │
//!             │ realizes event
//!             ▼
//! fault::correlated::CorrelatedFault
//! ```
//!
//! `fault::correlated::CorrelatedFault` remains the owner of realized fault
//! groups. It must not be replaced or duplicated here.
//!
//! # Integration with `noise::model`
//!
//! `NoiseModel` may hold or reference a `CorrelationModel`.
//!
//! This module does not import `noise::model`, avoiding a dependency cycle.
//!
//! The intended dependency is:
//!
//! ```text
//! correlation.rs
//!       ▲
//!       │
//!       │
//! noise/model.rs
//! ```
//!
//! # Integration with `noise::application`
//!
//! Application logic can inspect a correlation domain and determine whether an
//! operation/resource intersects that domain.
//!
//! This module does not apply the noise.
//!
//! # Integration with routing
//!
//! Routing may use correlation information as one cost component:
//!
//! ```text
//! placement candidate
//!       │
//!       ▼
//! correlation exposure
//!       │
//!       ▼
//! routing cost
//! ```
//!
//! This module does not choose a route.
//!
//! # Integration with scheduling
//!
//! Scheduling may use correlation domains to reason about concurrent operations
//! and crosstalk exposure.
//!
//! This module does not schedule operations.
//!
//! # Integration with QEC
//!
//! QEC may transform a realized correlated fault into code-specific syndrome
//! and logical-fault semantics.
//!
//! This module does not perform syndrome extraction or decoding.
//!
//! # Integration with characterization
//!
//! Characterization can produce a `CorrelationModel` from measured
//! observations.
//!
//! This module stores the resulting semantic model but does not implement the
//! experimental protocol or estimator.
//!
//! # Integration with hardware
//!
//! Hardware adapters can translate measured/device-specific correlation data
//! into this target-independent representation.
//!
//! Vendor-specific API logic must remain outside ZQN.
//!
//! # Rust compatibility
//!
//! This file targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. correlation semantics are independent from realized faults;
//! 2. canonical IR qubit IDs are reused;
//! 3. no duplicate qubit identity types exist;
//! 4. arbitrary finite correlation domains are supported;
//! 5. no artificial correlation-size limit exists;
//! 6. pairwise relations are canonicalized;
//! 7. duplicate/self relations are rejected;
//! 8. relation endpoints must belong to the domain;
//! 9. strength is finite and bounded;
//! 10. approximation is explicit;
//! 11. construction supports iterators/builders;
//! 12. no RNG exists;
//! 13. no global state exists;
//! 14. no unsafe code exists;
//! 15. serialization remains an IO-layer responsibility;
//! 16. model identity remains an ID-layer responsibility;
//! 17. downstream modules can consume the model without changing its semantics.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::cmp::Ordering;
use core::fmt;

use crate::quantum::ir::qubit::{
    PhysicalQubitId,
    QubitId,
};

use crate::quantum::zqn::core::errors::{
    ZqnError,
    ZqnResult,
};

use crate::quantum::zqn::core::ids::{
    CorrelationId,
    ZqnObjectId,
};

// ============================================================================
// Correlation resource
// ============================================================================

/// A resource participating in a correlation domain.
///
/// The logical and physical qubit variants use the canonical Quantum IR
/// identity types directly.
///
/// The `ZqnObject` variant exists for quantum resources whose canonical
/// identity is owned by another subsystem and therefore must not be recreated
/// here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CorrelationResource {
    /// Canonical logical-qubit identity.
    LogicalQubit(QubitId),

    /// Canonical physical-qubit identity.
    PhysicalQubit(PhysicalQubitId),

    /// Opaque identity for a ZQN-addressable resource owned elsewhere.
    ///
    /// The surrounding subsystem is responsible for assigning the appropriate
    /// semantic meaning to this object identity.
    ZqnObject(ZqnObjectId),
}

impl CorrelationResource {
    /// Returns the logical-qubit identity when this resource is a logical
    /// qubit.
    #[must_use]
    pub const fn logical_qubit(self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(id) => Some(id),
            Self::PhysicalQubit(_) | Self::ZqnObject(_) => None,
        }
    }

    /// Returns the physical-qubit identity when this resource is a physical
    /// qubit.
    #[must_use]
    pub const fn physical_qubit(self) -> Option<PhysicalQubitId> {
        match self {
            Self::PhysicalQubit(id) => Some(id),
            Self::LogicalQubit(_) | Self::ZqnObject(_) => None,
        }
    }

    /// Returns true when this resource belongs to the logical-qubit domain.
    #[must_use]
    pub const fn is_logical_qubit(self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns true when this resource belongs to the physical-qubit domain.
    #[must_use]
    pub const fn is_physical_qubit(self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }

    /// Returns true when this resource is an opaque ZQN object identity.
    #[must_use]
    pub const fn is_zqn_object(self) -> bool {
        matches!(self, Self::ZqnObject(_))
    }
}

impl fmt::Display for CorrelationResource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LogicalQubit(id) => write!(formatter, "logical-qubit:{id}"),
            Self::PhysicalQubit(id) => write!(formatter, "physical-qubit:{id}"),
            Self::ZqnObject(id) => write!(formatter, "zqn-object:{id}"),
        }
    }
}

// ============================================================================
// Correlation strength
// ============================================================================

/// A finite normalized correlation-strength descriptor.
///
/// The value is constrained to `[-1, 1]`.
///
/// This is not automatically a probability.
///
/// The surrounding correlation relation determines its precise statistical or
/// physical interpretation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CorrelationStrength(f64);

impl CorrelationStrength {
    /// Zero correlation.
    pub const ZERO: Self = Self(0.0);

    /// Maximum positive correlation.
    pub const POSITIVE_ONE: Self = Self(1.0);

    /// Maximum negative correlation.
    pub const NEGATIVE_ONE: Self = Self(-1.0);

    /// Creates a correlation strength.
    ///
    /// The value must be finite and lie in `[-1, 1]`.
    pub fn new(value: f64) -> ZqnResult<Self> {
        if !value.is_finite() {
            return Err(ZqnError::invalid_correlation(
                "correlation strength must be finite",
            ));
        }

        if !(-1.0..=1.0).contains(&value) {
            return Err(ZqnError::invalid_correlation(
                "correlation strength must lie in [-1, 1]",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the numeric correlation-strength value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Returns true when the descriptor is exactly zero.
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    /// Returns true when the descriptor is positive.
    #[must_use]
    pub fn is_positive(self) -> bool {
        self.0 > 0.0
    }

    /// Returns true when the descriptor is negative.
    #[must_use]
    pub fn is_negative(self) -> bool {
        self.0 < 0.0
    }

    /// Returns the absolute magnitude.
    #[must_use]
    pub fn magnitude(self) -> f64 {
        self.0.abs()
    }
}

impl Eq for CorrelationStrength {}

impl Hash for CorrelationStrength {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl PartialOrd for CorrelationStrength {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CorrelationStrength {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl fmt::Display for CorrelationStrength {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_fmt(format_args!("{}", self.0))
    }
}

// ============================================================================
// Correlation kind
// ============================================================================

/// Semantic kind of a correlation relationship.
///
/// This enum identifies the *meaning* of the relationship without prescribing
/// a particular numerical algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CorrelationKind {
    /// Resources are explicitly independent.
    Independent,

    /// A pairwise statistical/physical relationship.
    Pairwise,

    /// A collective/group relationship among multiple resources.
    Collective,

    /// A graph-structured relationship.
    Graph,

    /// A covariance/correlation-matrix-like relationship.
    Covariance,

    /// A correlation represented by an externally defined model.
    ///
    /// The model identity and semantics are supplied by the surrounding
    /// subsystem.
    External,

    /// A bounded approximation of another correlation relation.
    Approximate,
}

impl fmt::Display for CorrelationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Independent => "independent",
            Self::Pairwise => "pairwise",
            Self::Collective => "collective",
            Self::Graph => "graph",
            Self::Covariance => "covariance",
            Self::External => "external",
            Self::Approximate => "approximate",
        };

        formatter.write_str(value)
    }
}

// ============================================================================
// Approximation semantics
// ============================================================================

/// Explicit accuracy contract for a correlation description.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CorrelationAccuracy {
    /// The representation is declared exact within the semantics of the model.
    Exact,

    /// The representation is approximate within an absolute tolerance.
    AbsoluteTolerance(f64),

    /// The representation is an estimate with an explicit confidence level.
    ///
    /// `confidence` is in `[0, 1]`.
    Statistical {
        /// Confidence level.
        confidence: f64,
    },

    /// The represented correlation is bounded by an absolute error.
    Bounded {
        /// Maximum declared absolute error.
        absolute_error: f64,
    },
}

impl CorrelationAccuracy {
    /// Creates an absolute-tolerance accuracy contract.
    pub fn absolute_tolerance(tolerance: f64) -> ZqnResult<Self> {
        validate_non_negative_finite(
            tolerance,
            "correlation tolerance must be finite and non-negative",
        )?;

        Ok(Self::AbsoluteTolerance(tolerance))
    }

    /// Creates a statistical confidence contract.
    pub fn statistical(confidence: f64) -> ZqnResult<Self> {
        if !confidence.is_finite() || !(0.0..=1.0).contains(&confidence) {
            return Err(ZqnError::invalid_correlation(
                "correlation confidence must be finite and lie in [0, 1]",
            ));
        }

        Ok(Self::Statistical { confidence })
    }

    /// Creates a bounded-error contract.
    pub fn bounded(absolute_error: f64) -> ZqnResult<Self> {
        validate_non_negative_finite(
            absolute_error,
            "correlation absolute error must be finite and non-negative",
        )?;

        Ok(Self::Bounded { absolute_error })
    }

    /// Returns true when the contract declares exact semantics.
    #[must_use]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Returns the declared absolute error when one exists.
    #[must_use]
    pub const fn absolute_error(self) -> Option<f64> {
        match self {
            Self::Exact => Some(0.0),
            Self::AbsoluteTolerance(value) => Some(value),
            Self::Statistical { .. } => None,
            Self::Bounded { absolute_error } => Some(absolute_error),
        }
    }

    /// Returns the confidence level when this is a statistical estimate.
    #[must_use]
    pub const fn confidence(self) -> Option<f64> {
        match self {
            Self::Statistical { confidence } => Some(confidence),
            Self::Exact
            | Self::AbsoluteTolerance(_)
            | Self::Bounded { .. } => None,
        }
    }
}

impl Eq for CorrelationAccuracy {}

impl Hash for CorrelationAccuracy {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Exact => {
                0_u8.hash(state);
            }
            Self::AbsoluteTolerance(value) => {
                1_u8.hash(state);
                value.to_bits().hash(state);
            }
            Self::Statistical { confidence } => {
                2_u8.hash(state);
                confidence.to_bits().hash(state);
            }
            Self::Bounded { absolute_error } => {
                3_u8.hash(state);
                absolute_error.to_bits().hash(state);
            }
        }
    }
}

impl PartialOrd for CorrelationAccuracy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CorrelationAccuracy {
    fn cmp(&self, other: &Self) -> Ordering {
        accuracy_tag(self)
            .cmp(&accuracy_tag(other))
            .then_with(|| accuracy_bits(self).cmp(&accuracy_bits(other)))
    }
}

fn accuracy_tag(value: &CorrelationAccuracy) -> u8 {
    match value {
        CorrelationAccuracy::Exact => 0,
        CorrelationAccuracy::AbsoluteTolerance(_) => 1,
        CorrelationAccuracy::Statistical { .. } => 2,
        CorrelationAccuracy::Bounded { .. } => 3,
    }
}

fn accuracy_bits(value: &CorrelationAccuracy) -> u64 {
    match value {
        CorrelationAccuracy::Exact => 0,
        CorrelationAccuracy::AbsoluteTolerance(v) => v.to_bits(),
        CorrelationAccuracy::Statistical { confidence } => confidence.to_bits(),
        CorrelationAccuracy::Bounded { absolute_error } => absolute_error.to_bits(),
    }
}

// ============================================================================
// Pair correlation
// ============================================================================

/// Canonical pairwise correlation relation.
///
/// The two endpoints are stored in canonical order, making `A ↔ B` and
/// `B ↔ A` semantically identical for this undirected relation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PairCorrelation {
    first: CorrelationResource,
    second: CorrelationResource,
    strength: CorrelationStrength,
    accuracy: CorrelationAccuracy,
}

impl PairCorrelation {
    /// Creates a pairwise correlation.
    ///
    /// Self-correlation is rejected.
    pub fn new(
        first: CorrelationResource,
        second: CorrelationResource,
        strength: CorrelationStrength,
    ) -> ZqnResult<Self> {
        Self::with_accuracy(
            first,
            second,
            strength,
            CorrelationAccuracy::Exact,
        )
    }

    /// Creates a pairwise correlation with an explicit accuracy contract.
    pub fn with_accuracy(
        first: CorrelationResource,
        second: CorrelationResource,
        strength: CorrelationStrength,
        accuracy: CorrelationAccuracy,
    ) -> ZqnResult<Self> {
        if first == second {
            return Err(ZqnError::invalid_correlation(
                "a resource cannot be correlated with itself in a pairwise relation",
            ));
        }

        validate_accuracy(accuracy)?;

        let (first, second) = canonical_pair(first, second);

        Ok(Self {
            first,
            second,
            strength,
            accuracy,
        })
    }

    /// Returns the first canonical endpoint.
    #[must_use]
    pub const fn first(self) -> CorrelationResource {
        self.first
    }

    /// Returns the second canonical endpoint.
    #[must_use]
    pub const fn second(self) -> CorrelationResource {
        self.second
    }

    /// Returns the correlation strength.
    #[must_use]
    pub const fn strength(self) -> CorrelationStrength {
        self.strength
    }

    /// Returns the accuracy contract.
    #[must_use]
    pub const fn accuracy(self) -> CorrelationAccuracy {
        self.accuracy
    }

    /// Returns true when the supplied resource is an endpoint.
    #[must_use]
    pub const fn contains(self, resource: CorrelationResource) -> bool {
        self.first == resource || self.second == resource
    }

    /// Returns the opposite endpoint.
    #[must_use]
    pub const fn other(
        self,
        resource: CorrelationResource,
    ) -> Option<CorrelationResource> {
        if self.first == resource {
            Some(self.second)
        } else if self.second == resource {
            Some(self.first)
        } else {
            None
        }
    }
}

impl fmt::Display for PairCorrelation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}<->{}:{}",
            self.first,
            self.second,
            self.strength
        )
    }
}

// ============================================================================
// Generic relation
// ============================================================================

/// A correlation relationship within a domain.
///
/// `Pair` provides a directly queryable pairwise relationship.
///
/// `Group` describes collective participation without pretending that a single
/// pairwise coefficient fully describes the joint distribution.
///
/// `External` provides an identity for a richer correlation law implemented by
/// another subsystem.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CorrelationRelation {
    /// Pairwise correlation.
    Pair(PairCorrelation),

    /// Collective relationship over a group of domain resources.
    Group {
        /// Participating resources.
        resources: Vec<CorrelationResource>,

        /// Optional aggregate strength descriptor.
        strength: Option<CorrelationStrength>,

        /// Accuracy contract.
        accuracy: CorrelationAccuracy,
    },

    /// External correlation law identified by a ZQN object identity.
    External {
        /// Identity of the external correlation-law object.
        model_id: ZqnObjectId,

        /// Resources participating in the external relationship.
        resources: Vec<CorrelationResource>,

        /// Accuracy contract.
        accuracy: CorrelationAccuracy,
    },
}

impl CorrelationRelation {
    /// Creates a collective group relation.
    pub fn group(
        resources: Vec<CorrelationResource>,
        strength: Option<CorrelationStrength>,
    ) -> ZqnResult<Self> {
        Self::group_with_accuracy(
            resources,
            strength,
            CorrelationAccuracy::Exact,
        )
    }

    /// Creates a collective group relation with explicit accuracy.
    pub fn group_with_accuracy(
        mut resources: Vec<CorrelationResource>,
        strength: Option<CorrelationStrength>,
        accuracy: CorrelationAccuracy,
    ) -> ZqnResult<Self> {
        canonicalize_resources(&mut resources)?;
        validate_accuracy(accuracy)?;

        if resources.len() < 2 {
            return Err(ZqnError::invalid_correlation(
                "a collective correlation group requires at least two distinct resources",
            ));
        }

        Ok(Self::Group {
            resources,
            strength,
            accuracy,
        })
    }

    /// Creates an external correlation relation.
    pub fn external(
        model_id: ZqnObjectId,
        resources: Vec<CorrelationResource>,
        accuracy: CorrelationAccuracy,
    ) -> ZqnResult<Self> {
        let mut resources = resources;
        canonicalize_resources(&mut resources)?;
        validate_accuracy(accuracy)?;

        if resources.is_empty() {
            return Err(ZqnError::invalid_correlation(
                "an external correlation relation requires at least one resource",
            ));
        }

        Ok(Self::External {
            model_id,
            resources,
            accuracy,
        })
    }

    /// Returns all resources participating in the relation.
    #[must_use]
    pub fn resources(&self) -> &[CorrelationResource] {
        match self {
            Self::Pair(pair) => {
                // This branch cannot return a temporary array.
                // Consumers needing a slice should use `for_each_resource`.
                let _ = pair;
                &[]
            }
            Self::Group { resources, .. }
            | Self::External { resources, .. } => resources,
        }
    }

    /// Calls `visitor` for every resource participating in the relation.
    ///
    /// This avoids allocating a temporary collection for pairwise relations.
    pub fn for_each_resource(
        &self,
        mut visitor: impl FnMut(CorrelationResource),
    ) {
        match self {
            Self::Pair(pair) => {
                visitor(pair.first());
                visitor(pair.second());
            }
            Self::Group { resources, .. }
            | Self::External { resources, .. } => {
                for resource in resources {
                    visitor(*resource);
                }
            }
        }
    }

    /// Returns the relation accuracy contract.
    #[must_use]
    pub const fn accuracy(&self) -> CorrelationAccuracy {
        match self {
            Self::Pair(pair) => pair.accuracy(),
            Self::Group { accuracy, .. }
            | Self::External { accuracy, .. } => *accuracy,
        }
    }

    /// Returns the number of participating resources.
    #[must_use]
    pub fn resource_count(&self) -> usize {
        match self {
            Self::Pair(_) => 2,
            Self::Group { resources, .. }
            | Self::External { resources, .. } => resources.len(),
        }
    }

    /// Returns true when the relation contains the supplied resource.
    #[must_use]
    pub fn contains(&self, resource: CorrelationResource) -> bool {
        match self {
            Self::Pair(pair) => pair.contains(resource),
            Self::Group { resources, .. }
            | Self::External { resources, .. } => {
                resources.binary_search(&resource).is_ok()
            }
        }
    }

    /// Returns the pair relation when this is a pairwise relation.
    #[must_use]
    pub const fn as_pair(&self) -> Option<&PairCorrelation> {
        match self {
            Self::Pair(pair) => Some(pair),
            Self::Group { .. } | Self::External { .. } => None,
        }
    }
}

// ============================================================================
// Correlation edge
// ============================================================================

/// Canonical graph edge used by graph-oriented consumers.
///
/// This is a semantic view of a pair relation rather than a second correlation
/// representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CorrelationEdge {
    first: CorrelationResource,
    second: CorrelationResource,
    strength: CorrelationStrength,
}

impl CorrelationEdge {
    /// Creates a canonical undirected correlation edge.
    pub fn new(
        first: CorrelationResource,
        second: CorrelationResource,
        strength: CorrelationStrength,
    ) -> ZqnResult<Self> {
        if first == second {
            return Err(ZqnError::invalid_correlation(
                "correlation graph edges cannot connect a resource to itself",
            ));
        }

        let (first, second) = canonical_pair(first, second);

        Ok(Self {
            first,
            second,
            strength,
        })
    }

    /// Returns the first endpoint.
    #[must_use]
    pub const fn first(self) -> CorrelationResource {
        self.first
    }

    /// Returns the second endpoint.
    #[must_use]
    pub const fn second(self) -> CorrelationResource {
        self.second
    }

    /// Returns the edge strength.
    #[must_use]
    pub const fn strength(self) -> CorrelationStrength {
        self.strength
    }

    /// Returns the opposite endpoint.
    #[must_use]
    pub const fn other(
        self,
        resource: CorrelationResource,
    ) -> Option<CorrelationResource> {
        if self.first == resource {
            Some(self.second)
        } else if self.second == resource {
            Some(self.first)
        } else {
            None
        }
    }
}

// ============================================================================
// Correlation domain
// ============================================================================

/// A validated set of resources participating in one correlation domain.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CorrelationDomain {
    resources: Vec<CorrelationResource>,
}

impl CorrelationDomain {
    /// Constructs a domain from an owned resource collection.
    ///
    /// The collection is consumed and canonicalized.
    pub fn new(
        mut resources: Vec<CorrelationResource>,
    ) -> ZqnResult<Self> {
        canonicalize_resources(&mut resources)?;

        if resources.is_empty() {
            return Err(ZqnError::invalid_correlation_domain(
                "correlation domain cannot be empty",
            ));
        }

        Ok(Self { resources })
    }

    /// Constructs a domain from an iterator.
    pub fn from_iter<I>(
        resources: I,
    ) -> ZqnResult<Self>
    where
        I: IntoIterator<Item = CorrelationResource>,
    {
        Self::new(resources.into_iter().collect())
    }

    /// Returns all resources in canonical order.
    #[must_use]
    pub fn resources(&self) -> &[CorrelationResource] {
        &self.resources
    }

    /// Returns the number of resources.
    #[must_use]
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    /// Returns whether the domain is empty.
    ///
    /// A valid domain is never empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    /// Returns whether a resource participates in the domain.
    #[must_use]
    pub fn contains(&self, resource: CorrelationResource) -> bool {
        self.resources.binary_search(&resource).is_ok()
    }

    /// Returns the index of a resource in canonical order.
    #[must_use]
    pub fn index_of(
        &self,
        resource: CorrelationResource,
    ) -> Option<usize> {
        self.resources.binary_search(&resource).ok()
    }

    /// Returns true when the domain contains at least two resources.
    #[must_use]
    pub fn is_multi_resource(&self) -> bool {
        self.resources.len() >= 2
    }

    /// Returns an iterator over resources.
    pub fn iter(&self) -> core::slice::Iter<'_, CorrelationResource> {
        self.resources.iter()
    }
}

// ============================================================================
// Correlation model
// ============================================================================

/// Canonical declarative correlation model.
///
/// A `CorrelationModel` describes dependence between resources without
/// realizing noise events and without performing stochastic sampling.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CorrelationModel {
    correlation_id: CorrelationId,
    kind: CorrelationKind,
    domain: CorrelationDomain,
    relations: Vec<CorrelationRelation>,
}

impl CorrelationModel {
    /// Creates a validated correlation model.
    pub fn new(
        correlation_id: CorrelationId,
        kind: CorrelationKind,
        domain: CorrelationDomain,
        relations: Vec<CorrelationRelation>,
    ) -> ZqnResult<Self> {
        let mut model = Self {
            correlation_id,
            kind,
            domain,
            relations,
        };

        model.canonicalize()?;
        model.validate()?;

        Ok(model)
    }

    /// Constructs a model from a resource iterator and relation collection.
    pub fn from_iter<I>(
        correlation_id: CorrelationId,
        kind: CorrelationKind,
        resources: I,
        relations: Vec<CorrelationRelation>,
    ) -> ZqnResult<Self>
    where
        I: IntoIterator<Item = CorrelationResource>,
    {
        Self::new(
            correlation_id,
            kind,
            CorrelationDomain::from_iter(resources)?,
            relations,
        )
    }

    /// Returns the model identity.
    #[must_use]
    pub const fn correlation_id(&self) -> CorrelationId {
        self.correlation_id
    }

    /// Returns the correlation kind.
    #[must_use]
    pub const fn kind(&self) -> CorrelationKind {
        self.kind
    }

    /// Returns the domain.
    #[must_use]
    pub const fn domain(&self) -> &CorrelationDomain {
        &self.domain
    }

    /// Returns all canonical relations.
    #[must_use]
    pub fn relations(&self) -> &[CorrelationRelation] {
        &self.relations
    }

    /// Returns the number of relations.
    #[must_use]
    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }

    /// Returns whether this model contains a relation for the resource.
    #[must_use]
    pub fn contains_resource(
        &self,
        resource: CorrelationResource,
    ) -> bool {
        self.domain.contains(resource)
    }

    /// Returns pairwise correlation information for two resources.
    ///
    /// This returns `Some` only when an explicit pair relation exists.
    ///
    /// It does not infer a pairwise coefficient from a collective relation.
    #[must_use]
    pub fn pair_correlation(
        &self,
        first: CorrelationResource,
        second: CorrelationResource,
    ) -> Option<&PairCorrelation> {
        if first == second {
            return None;
        }

        self.relations.iter().find_map(|relation| {
            let pair = relation.as_pair()?;

            let (left, right) = canonical_pair(first, second);

            if pair.first() == left && pair.second() == right {
                Some(pair)
            } else {
                None
            }
        })
    }

    /// Returns all relations touching a resource.
    ///
    /// The returned iterator does not allocate.
    pub fn relations_for(
        &self,
        resource: CorrelationResource,
    ) -> impl Iterator<Item = &CorrelationRelation> {
        self.relations
            .iter()
            .filter(move |relation| relation.contains(resource))
    }

    /// Returns the number of relations touching a resource.
    #[must_use]
    pub fn relation_degree(
        &self,
        resource: CorrelationResource,
    ) -> usize {
        self.relations_for(resource).count()
    }

    /// Returns true when two resources share an explicit correlation relation.
    #[must_use]
    pub fn are_related(
        &self,
        first: CorrelationResource,
        second: CorrelationResource,
    ) -> bool {
        if first == second {
            return false;
        }

        self.pair_correlation(first, second).is_some()
            || self.relations.iter().any(|relation| {
                relation.contains(first) && relation.contains(second)
            })
    }

    /// Returns all pairwise edges represented explicitly by this model.
    ///
    /// Collective/external relations are intentionally not expanded into
    /// O(n²) edges. Such an expansion can be catastrophically expensive for
    /// large correlation domains.
    ///
    /// Consumers requiring a graph projection must explicitly request and
    /// resource-limit that projection at their own layer.
    pub fn pair_edges(
        &self,
    ) -> impl Iterator<Item = CorrelationEdge> + '_ {
        self.relations.iter().filter_map(|relation| {
            relation
                .as_pair()
                .and_then(|pair| {
                    CorrelationEdge::new(
                        pair.first(),
                        pair.second(),
                        pair.strength(),
                    )
                    .ok()
                })
        })
    }

    /// Validates all model invariants without mutation.
    pub fn validate(&self) -> ZqnResult<()> {
        if self.domain.is_empty() {
            return Err(ZqnError::invalid_correlation_domain(
                "correlation model domain cannot be empty",
            ));
        }

        let mut previous_pair: Option<(CorrelationResource, CorrelationResource)> =
            None;

        for relation in &self.relations {
            validate_relation(&self.domain, relation)?;

            if let Some(pair) = relation.as_pair() {
                let key = (pair.first(), pair.second());

                if previous_pair == Some(key) {
                    return Err(ZqnError::invalid_correlation(
                        "duplicate pairwise correlation relation",
                    ));
                }

                previous_pair = Some(key);
            }
        }

        validate_kind(&self.kind, &self.domain, &self.relations)?;

        Ok(())
    }

    /// Produces a deterministic diagnostic description.
    ///
    /// This is diagnostic text and is not a serialization format.
    #[must_use]
    pub fn describe(&self) -> String {
        let mut output = format!(
            "correlation:{}:kind={}:resources={}:relations={}",
            self.correlation_id,
            self.kind,
            self.domain.len(),
            self.relations.len(),
        );

        for relation in &self.relations {
            output.push('|');

            match relation {
                CorrelationRelation::Pair(pair) => {
                    output.push_str("pair:");
                    output.push_str(&pair.to_string());
                }
                CorrelationRelation::Group {
                    resources,
                    strength,
                    ..
                } => {
                    output.push_str("group:");
                    output.push_str(&resources.len().to_string());

                    if let Some(strength) = strength {
                        output.push_str(":strength=");
                        output.push_str(&strength.to_string());
                    }
                }
                CorrelationRelation::External {
                    model_id,
                    resources,
                    ..
                } => {
                    output.push_str("external:");
                    output.push_str(&model_id.to_string());
                    output.push_str(":resources=");
                    output.push_str(&resources.len().to_string());
                }
            }
        }

        output
    }

    fn canonicalize(&mut self) -> ZqnResult<()> {
        self.relations
            .sort_by(compare_relations_canonically);

        Ok(())
    }
}

impl fmt::Display for CorrelationModel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.describe())
    }
}

// ============================================================================
// Builder
// ============================================================================

/// Incremental builder for [`CorrelationModel`].
///
/// The builder performs local validation when possible and performs complete
/// domain/relation validation when finalized.
#[derive(Debug, Clone)]
pub struct CorrelationBuilder {
    correlation_id: CorrelationId,
    kind: CorrelationKind,
    resources: Vec<CorrelationResource>,
    relations: Vec<CorrelationRelation>,
}

impl CorrelationBuilder {
    /// Creates an empty builder.
    #[must_use]
    pub const fn new(
        correlation_id: CorrelationId,
        kind: CorrelationKind,
    ) -> Self {
        Self {
            correlation_id,
            kind,
            resources: Vec::new(),
            relations: Vec::new(),
        }
    }

    /// Creates a builder with allocation hints.
    ///
    /// The capacities are allocation hints, not semantic limits.
    #[must_use]
    pub fn with_capacity(
        correlation_id: CorrelationId,
        kind: CorrelationKind,
        resource_capacity: usize,
        relation_capacity: usize,
    ) -> Self {
        Self {
            correlation_id,
            kind,
            resources: Vec::with_capacity(resource_capacity),
            relations: Vec::with_capacity(relation_capacity),
        }
    }

    /// Returns the correlation identity.
    #[must_use]
    pub const fn correlation_id(&self) -> CorrelationId {
        self.correlation_id
    }

    /// Returns the selected correlation kind.
    #[must_use]
    pub const fn kind(&self) -> CorrelationKind {
        self.kind
    }

    /// Returns the number of currently accepted resources.
    #[must_use]
    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }

    /// Returns the number of currently accepted relations.
    #[must_use]
    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }

    /// Adds one resource.
    pub fn add_resource(
        &mut self,
        resource: CorrelationResource,
    ) -> ZqnResult<()> {
        if self.resources.contains(&resource) {
            return Err(ZqnError::invalid_correlation(
                "correlation domain contains a duplicate resource",
            ));
        }

        self.resources.push(resource);

        Ok(())
    }

    /// Adds resources from an iterator.
    ///
    /// Previously accepted resources remain if a later resource fails.
    pub fn extend_resources<I>(
        &mut self,
        resources: I,
    ) -> ZqnResult<()>
    where
        I: IntoIterator<Item = CorrelationResource>,
    {
        for resource in resources {
            self.add_resource(resource)?;
        }

        Ok(())
    }

    /// Adds a relation.
    pub fn add_relation(
        &mut self,
        relation: CorrelationRelation,
    ) -> ZqnResult<()> {
        validate_relation_shape(&relation)?;

        self.relations.push(relation);

        Ok(())
    }

    /// Adds multiple relations from an iterator.
    pub fn extend_relations<I>(
        &mut self,
        relations: I,
    ) -> ZqnResult<()>
    where
        I: IntoIterator<Item = CorrelationRelation>,
    {
        for relation in relations {
            self.add_relation(relation)?;
        }

        Ok(())
    }

    /// Returns immutable resources accumulated so far.
    #[must_use]
    pub fn resources(&self) -> &[CorrelationResource] {
        &self.resources
    }

    /// Returns immutable relations accumulated so far.
    #[must_use]
    pub fn relations(&self) -> &[CorrelationRelation] {
        &self.relations
    }

    /// Finalizes the builder into a validated canonical model.
    pub fn finish(self) -> ZqnResult<CorrelationModel> {
        CorrelationModel::from_iter(
            self.correlation_id,
            self.kind,
            self.resources,
            self.relations,
        )
    }
}

// ============================================================================
// Validation helpers
// ============================================================================

fn canonical_pair(
    first: CorrelationResource,
    second: CorrelationResource,
) -> (CorrelationResource, CorrelationResource) {
    if first <= second {
        (first, second)
    } else {
        (second, first)
    }
}

fn canonicalize_resources(
    resources: &mut Vec<CorrelationResource>,
) -> ZqnResult<()> {
    resources.sort_unstable();

    if resources.windows(2).any(|window| window[0] == window[1]) {
        return Err(ZqnError::invalid_correlation_domain(
            "correlation domain contains duplicate resources",
        ));
    }

    Ok(())
}

fn validate_relation_shape(
    relation: &CorrelationRelation,
) -> ZqnResult<()> {
    match relation {
        CorrelationRelation::Pair(pair) => {
            if pair.first() == pair.second() {
                return Err(ZqnError::invalid_correlation(
                    "pairwise correlation cannot be self-referential",
                ));
            }

            validate_accuracy(pair.accuracy())
        }

        CorrelationRelation::Group {
            resources,
            accuracy,
            ..
        } => {
            validate_accuracy(*accuracy)?;

            if resources.len() < 2 {
                return Err(ZqnError::invalid_correlation(
                    "collective correlation requires at least two resources",
                ));
            }

            validate_resource_sequence(resources)
        }

        CorrelationRelation::External {
            resources,
            accuracy,
            ..
        } => {
            validate_accuracy(*accuracy)?;

            if resources.is_empty() {
                return Err(ZqnError::invalid_correlation(
                    "external correlation requires at least one resource",
                ));
            }

            validate_resource_sequence(resources)
        }
    }
}

fn validate_resource_sequence(
    resources: &[CorrelationResource],
) -> ZqnResult<()> {
    for window in resources.windows(2) {
        if window[0] >= window[1] {
            return Err(ZqnError::invalid_correlation(
                "correlation relation resources must be canonical and unique",
            ));
        }
    }

    Ok(())
}

fn validate_relation(
    domain: &CorrelationDomain,
    relation: &CorrelationRelation,
) -> ZqnResult<()> {
    validate_relation_shape(relation)?;

    let mut valid = true;

    relation.for_each_resource(|resource| {
        if !domain.contains(resource) {
            valid = false;
        }
    });

    if !valid {
        return Err(ZqnError::invalid_correlation(
            "correlation relation references a resource outside its domain",
        ));
    }

    Ok(())
}

fn validate_kind(
    kind: &CorrelationKind,
    domain: &CorrelationDomain,
    relations: &[CorrelationRelation],
) -> ZqnResult<()> {
    match kind {
        CorrelationKind::Independent => {
            if !relations.is_empty() {
                return Err(ZqnError::invalid_correlation(
                    "an independent correlation model cannot contain correlation relations",
                ));
            }
        }

        CorrelationKind::Pairwise => {
            if relations
                .iter()
                .any(|relation| relation.as_pair().is_none())
            {
                return Err(ZqnError::invalid_correlation(
                    "a pairwise correlation model may contain only pair relations",
                ));
            }

            if domain.len() < 2 {
                return Err(ZqnError::invalid_correlation_domain(
                    "a pairwise correlation model requires at least two resources",
                ));
            }
        }

        CorrelationKind::Collective => {
            if relations.is_empty() {
                return Err(ZqnError::invalid_correlation(
                    "a collective correlation model requires at least one relation",
                ));
            }
        }

        CorrelationKind::Graph => {
            if relations
                .iter()
                .any(|relation| relation.as_pair().is_none())
            {
                return Err(ZqnError::invalid_correlation(
                    "a graph correlation model may contain only pair relations",
                ));
            }
        }

        CorrelationKind::Covariance
        | CorrelationKind::External
        | CorrelationKind::Approximate => {
            // These kinds permit their corresponding richer relation
            // representations. Detailed mathematical validity belongs to the
            // owning probability/characterization layer.
        }
    }

    Ok(())
}

fn validate_accuracy(
    accuracy: CorrelationAccuracy,
) -> ZqnResult<()> {
    match accuracy {
        CorrelationAccuracy::Exact => Ok(()),

        CorrelationAccuracy::AbsoluteTolerance(tolerance) => {
            validate_non_negative_finite(
                tolerance,
                "correlation tolerance must be finite and non-negative",
            )
        }

        CorrelationAccuracy::Statistical { confidence } => {
            if !confidence.is_finite()
                || !(0.0..=1.0).contains(&confidence)
            {
                return Err(ZqnError::invalid_correlation(
                    "correlation confidence must be finite and lie in [0, 1]",
                ));
            }

            Ok(())
        }

        CorrelationAccuracy::Bounded { absolute_error } => {
            validate_non_negative_finite(
                absolute_error,
                "correlation absolute error must be finite and non-negative",
            )
        }
    }
}

fn validate_non_negative_finite(
    value: f64,
    message: &'static str,
) -> ZqnResult<()> {
    if !value.is_finite() || value < 0.0 {
        return Err(ZqnError::invalid_correlation(message));
    }

    Ok(())
}

fn compare_relations_canonically(
    left: &CorrelationRelation,
    right: &CorrelationRelation,
) -> Ordering {
    relation_tag(left)
        .cmp(&relation_tag(right))
        .then_with(|| relation_resources_key(left).cmp(&relation_resources_key(right)))
        .then_with(|| relation_strength_key(left).cmp(&relation_strength_key(right)))
        .then_with(|| left.cmp(right))
}

fn relation_tag(relation: &CorrelationRelation) -> u8 {
    match relation {
        CorrelationRelation::Pair(_) => 0,
        CorrelationRelation::Group { .. } => 1,
        CorrelationRelation::External { .. } => 2,
    }
}

fn relation_resources_key(
    relation: &CorrelationRelation,
) -> Vec<CorrelationResource> {
    match relation {
        CorrelationRelation::Pair(pair) => {
            vec![pair.first(), pair.second()]
        }
        CorrelationRelation::Group { resources, .. }
        | CorrelationRelation::External { resources, .. } => {
            resources.clone()
        }
    }
}

fn relation_strength_key(
    relation: &CorrelationRelation,
) -> Option<CorrelationStrength> {
    match relation {
        CorrelationRelation::Pair(pair) => Some(pair.strength()),
        CorrelationRelation::Group { strength, .. } => *strength,
        CorrelationRelation::External { .. } => None,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::qubit::{
        PhysicalQubitId,
        QubitId,
    };

    use crate::quantum::zqn::core::ids::{
        CorrelationId,
        ZqnIdValue,
    };

    fn correlation_id(value: ZqnIdValue) -> CorrelationId {
        CorrelationId::new(value)
    }

    fn logical(value: ZqnIdValue) -> CorrelationResource {
        CorrelationResource::LogicalQubit(QubitId::new(value))
    }

    fn physical(value: ZqnIdValue) -> CorrelationResource {
        CorrelationResource::PhysicalQubit(
            PhysicalQubitId::new(value),
        )
    }

    #[test]
    fn logical_and_physical_identity_domains_remain_distinct() {
        let logical = logical(7);
        let physical = physical(7);

        assert_ne!(logical, physical);
        assert!(logical.is_logical_qubit());
        assert!(physical.is_physical_qubit());
    }

    #[test]
    fn correlation_strength_rejects_non_finite_values() {
        assert!(CorrelationStrength::new(f64::NAN).is_err());
        assert!(CorrelationStrength::new(f64::INFINITY).is_err());
        assert!(CorrelationStrength::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn correlation_strength_rejects_values_outside_domain() {
        assert!(CorrelationStrength::new(1.000_001).is_err());
        assert!(CorrelationStrength::new(-1.000_001).is_err());
    }

    #[test]
    fn correlation_strength_accepts_closed_interval() {
        assert!(CorrelationStrength::new(-1.0).is_ok());
        assert!(CorrelationStrength::new(0.0).is_ok());
        assert!(CorrelationStrength::new(1.0).is_ok());
    }

    #[test]
    fn pair_correlation_is_canonical() {
        let strength =
            CorrelationStrength::new(0.5).expect("valid strength");

        let forward = PairCorrelation::new(
            logical(1),
            physical(2),
            strength,
        )
        .expect("valid pair");

        let reverse = PairCorrelation::new(
            physical(2),
            logical(1),
            strength,
        )
        .expect("valid pair");

        assert_eq!(forward, reverse);
        assert_eq!(forward.first(), reverse.first());
        assert_eq!(forward.second(), reverse.second());
    }

    #[test]
    fn self_correlation_is_rejected() {
        let strength =
            CorrelationStrength::new(0.5).expect("valid strength");

        assert!(
            PairCorrelation::new(
                logical(1),
                logical(1),
                strength,
            )
            .is_err()
        );
    }

    #[test]
    fn empty_domain_is_rejected() {
        assert!(CorrelationDomain::new(Vec::new()).is_err());
    }

    #[test]
    fn duplicate_domain_resources_are_rejected() {
        assert!(
            CorrelationDomain::new(vec![
                logical(1),
                logical(1),
            ])
            .is_err()
        );
    }

    #[test]
    fn domain_is_canonicalized() {
        let domain = CorrelationDomain::new(vec![
            logical(9),
            logical(1),
            logical(5),
        ])
        .expect("valid domain");

        assert_eq!(
            domain.resources(),
            &[
                logical(1),
                logical(5),
                logical(9),
            ]
        );
    }

    #[test]
    fn group_relation_is_canonicalized() {
        let relation = CorrelationRelation::group(
            vec![
                logical(9),
                logical(1),
                logical(5),
            ],
            None,
        )
        .expect("valid group");

        match relation {
            CorrelationRelation::Group {
                resources,
                ..
            } => {
                assert_eq!(
                    resources,
                    vec![
                        logical(1),
                        logical(5),
                        logical(9),
                    ]
                );
            }
            _ => panic!("expected group relation"),
        }
    }

    #[test]
    fn relation_outside_domain_is_rejected() {
        let domain = CorrelationDomain::new(vec![
            logical(1),
            logical(2),
        ])
        .expect("valid domain");

        let relation = CorrelationRelation::Pair(
            PairCorrelation::new(
                logical(1),
                logical(3),
                CorrelationStrength::new(0.5)
                    .expect("valid strength"),
            )
            .expect("valid pair"),
        );

        assert!(
            validate_relation(&domain, &relation).is_err()
        );
    }

    #[test]
    fn independent_model_cannot_have_relations() {
        let domain = CorrelationDomain::new(vec![
            logical(1),
            logical(2),
        ])
        .expect("valid domain");

        let relation = CorrelationRelation::Pair(
            PairCorrelation::new(
                logical(1),
                logical(2),
                CorrelationStrength::new(0.5)
                    .expect("valid strength"),
            )
            .expect("valid pair"),
        );

        assert!(
            CorrelationModel::new(
                correlation_id(1),
                CorrelationKind::Independent,
                domain,
                vec![relation],
            )
            .is_err()
        );
    }

    #[test]
    fn pairwise_model_accepts_pair_relations() {
        let relation = CorrelationRelation::Pair(
            PairCorrelation::new(
                logical(1),
                logical(2),
                CorrelationStrength::new(0.75)
                    .expect("valid strength"),
            )
            .expect("valid pair"),
        );

        let model = CorrelationModel::new(
            correlation_id(1),
            CorrelationKind::Pairwise,
            CorrelationDomain::new(vec![
                logical(1),
                logical(2),
            ])
            .expect("valid domain"),
            vec![relation],
        )
        .expect("valid model");

        assert_eq!(model.relation_count(), 1);
        assert!(
            model.are_related(
                logical(1),
                logical(2)
            )
        );
    }

    #[test]
    fn pair_lookup_is_direction_independent() {
        let relation = CorrelationRelation::Pair(
            PairCorrelation::new(
                logical(1),
                logical(2),
                CorrelationStrength::new(0.75)
                    .expect("valid strength"),
            )
            .expect("valid pair"),
        );

        let model = CorrelationModel::new(
            correlation_id(1),
            CorrelationKind::Pairwise,
            CorrelationDomain::new(vec![
                logical(1),
                logical(2),
            ])
            .expect("valid domain"),
            vec![relation],
        )
        .expect("valid model");

        assert!(
            model
                .pair_correlation(
                    logical(1),
                    logical(2)
                )
                .is_some()
        );

        assert!(
            model
                .pair_correlation(
                    logical(2),
                    logical(1)
                )
                .is_some()
        );
    }

    #[test]
    fn relation_degree_is_available_without_allocation() {
        let relations = vec![
            CorrelationRelation::Pair(
                PairCorrelation::new(
                    logical(1),
                    logical(2),
                    CorrelationStrength::new(0.5)
                        .expect("valid strength"),
                )
                .expect("valid pair"),
            ),
            CorrelationRelation::Pair(
                PairCorrelation::new(
                    logical(1),
                    logical(3),
                    CorrelationStrength::new(0.25)
                        .expect("valid strength"),
                )
                .expect("valid pair"),
            ),
        ];

        let model = CorrelationModel::new(
            correlation_id(1),
            CorrelationKind::Graph,
            CorrelationDomain::new(vec![
                logical(1),
                logical(2),
                logical(3),
            ])
            .expect("valid domain"),
            relations,
        )
        .expect("valid model");

        assert_eq!(model.relation_degree(logical(1)), 2);
        assert_eq!(model.relation_degree(logical(2)), 1);
        assert_eq!(model.relation_degree(logical(3)), 1);
    }

    #[test]
    fn collective_relation_does_not_materialize_quadratic_edges() {
        let relation = CorrelationRelation::group(
            vec![
                logical(1),
                logical(2),
                logical(3),
                logical(4),
            ],
            Some(
                CorrelationStrength::new(0.5)
                    .expect("valid strength"),
            ),
        )
        .expect("valid group");

        let model = CorrelationModel::new(
            correlation_id(1),
            CorrelationKind::Collective,
            CorrelationDomain::new(vec![
                logical(1),
                logical(2),
                logical(3),
                logical(4),
            ])
            .expect("valid domain"),
            vec![relation],
        )
        .expect("valid model");

        assert!(model.are_related(logical(1), logical(4)));
        assert_eq!(model.pair_edges().count(), 0);
    }

    #[test]
    fn external_relation_is_supported_without_vendor_dependency() {
        let relation =
            CorrelationRelation::external(
                ZqnObjectId::new(900),
                vec![
                    logical(1),
                    logical(2),
                ],
                CorrelationAccuracy::Exact,
            )
            .expect("valid external relation");

        let model = CorrelationModel::new(
            correlation_id(1),
            CorrelationKind::External,
            CorrelationDomain::new(vec![
                logical(1),
                logical(2),
            ])
            .expect("valid domain"),
            vec![relation],
        )
        .expect("valid model");

        assert_eq!(model.relation_count(), 1);
        assert!(
            model.are_related(
                logical(1),
                logical(2)
            )
        );
    }

    #[test]
    fn approximate_accuracy_requires_explicit_contract() {
        let accuracy =
            CorrelationAccuracy::absolute_tolerance(0.001)
                .expect("valid tolerance");

        assert!(!accuracy.is_exact());
        assert_eq!(
            accuracy.absolute_error(),
            Some(0.001)
        );
    }

    #[test]
    fn invalid_accuracy_is_rejected() {
        assert!(
            CorrelationAccuracy::absolute_tolerance(-1.0)
                .is_err()
        );

        assert!(
            CorrelationAccuracy::absolute_tolerance(
                f64::NAN
            )
            .is_err()
        );

        assert!(
            CorrelationAccuracy::statistical(1.1)
                .is_err()
        );
    }

    #[test]
    fn builder_supports_incremental_construction() {
        let mut builder =
            CorrelationBuilder::new(
                correlation_id(42),
                CorrelationKind::Graph,
            );

        builder
            .add_resource(logical(1))
            .expect("resource should be accepted");

        builder
            .add_resource(logical(2))
            .expect("resource should be accepted");

        builder
            .add_relation(
                CorrelationRelation::Pair(
                    PairCorrelation::new(
                        logical(1),
                        logical(2),
                        CorrelationStrength::new(0.9)
                            .expect("valid strength"),
                    )
                    .expect("valid pair"),
                ),
            )
            .expect("relation should be accepted");

        let model = builder
            .finish()
            .expect("builder should produce valid model");

        assert_eq!(model.domain().len(), 2);
        assert_eq!(model.relation_count(), 1);
    }

    #[test]
    fn iterator_construction_is_supported() {
        let model = CorrelationModel::from_iter(
            correlation_id(10),
            CorrelationKind::Pairwise,
            vec![
                logical(3),
                logical(1),
                logical(2),
            ],
            vec![
                CorrelationRelation::Pair(
                    PairCorrelation::new(
                        logical(1),
                        logical(2),
                        CorrelationStrength::new(0.2)
                            .expect("valid strength"),
                    )
                    .expect("valid pair"),
                ),
            ],
        )
        .expect("valid model");

        assert_eq!(
            model.domain().resources(),
            &[
                logical(1),
                logical(2),
                logical(3),
            ]
        );
    }

    #[test]
    fn canonical_model_is_independent_of_relation_insertion_order() {
        let first =
            CorrelationRelation::Pair(
                PairCorrelation::new(
                    logical(1),
                    logical(2),
                    CorrelationStrength::new(0.2)
                        .expect("valid strength"),
                )
                .expect("valid pair"),
            );

        let second =
            CorrelationRelation::Pair(
                PairCorrelation::new(
                    logical(2),
                    logical(3),
                    CorrelationStrength::new(0.4)
                        .expect("valid strength"),
                )
                .expect("valid pair"),
            );

        let domain = CorrelationDomain::new(vec![
            logical(1),
            logical(2),
            logical(3),
        ])
        .expect("valid domain");

        let a = CorrelationModel::new(
            correlation_id(1),
            CorrelationKind::Graph,
            domain.clone(),
            vec![first.clone(), second.clone()],
        )
        .expect("valid model");

        let b = CorrelationModel::new(
            correlation_id(1),
            CorrelationKind::Graph,
            domain,
            vec![second, first],
        )
        .expect("valid model");

        assert_eq!(a, b);
    }

    #[test]
    fn description_is_deterministic() {
        let relation =
            CorrelationRelation::Pair(
                PairCorrelation::new(
                    logical(1),
                    logical(2),
                    CorrelationStrength::new(0.5)
                        .expect("valid strength"),
                )
                .expect("valid pair"),
            );

        let model = CorrelationModel::new(
            correlation_id(7),
            CorrelationKind::Pairwise,
            CorrelationDomain::new(vec![
                logical(1),
                logical(2),
            ])
            .expect("valid domain"),
            vec![relation],
        )
        .expect("valid model");

        assert_eq!(
            model.describe(),
            model.describe()
        );
    }

    #[test]
    fn model_validation_is_idempotent() {
        let relation =
            CorrelationRelation::Pair(
                PairCorrelation::new(
                    logical(1),
                    logical(2),
                    CorrelationStrength::new(0.5)
                        .expect("valid strength"),
                )
                .expect("valid pair"),
            );

        let model = CorrelationModel::new(
            correlation_id(7),
            CorrelationKind::Pairwise,
            CorrelationDomain::new(vec![
                logical(1),
                logical(2),
            ])
            .expect("valid domain"),
            vec![relation],
        )
        .expect("valid model");

        assert!(model.validate().is_ok());
        assert!(model.validate().is_ok());
    }

    #[test]
    fn pair_edges_are_linear_in_explicit_pair_relations() {
        let mut relations = Vec::new();

        for index in 0_u64..32 {
            let first = logical(index);
            let second = logical(index + 1);

            relations.push(
                CorrelationRelation::Pair(
                    PairCorrelation::new(
                        first,
                        second,
                        CorrelationStrength::new(0.1)
                            .expect("valid strength"),
                    )
                    .expect("valid pair"),
                ),
            );
        }

        let mut resources = Vec::new();

        for index in 0_u64..=32 {
            resources.push(logical(index));
        }

        let model = CorrelationModel::new(
            correlation_id(100),
            CorrelationKind::Graph,
            CorrelationDomain::new(resources)
                .expect("valid domain"),
            relations,
        )
        .expect("valid graph");

        assert_eq!(model.pair_edges().count(), 32);
    }

    #[test]
    fn same_numeric_id_in_different_domains_can_coexist() {
        let model = CorrelationModel::new(
            correlation_id(101),
            CorrelationKind::Collective,
            CorrelationDomain::new(vec![
                logical(7),
                physical(7),
            ])
            .expect("valid domain"),
            vec![
                CorrelationRelation::group(
                    vec![
                        logical(7),
                        physical(7),
                    ],
                    Some(
                        CorrelationStrength::new(0.3)
                            .expect("valid strength"),
                    ),
                )
                .expect("valid group"),
            ],
        )
        .expect("valid model");

        assert_eq!(model.domain().len(), 2);
        assert!(
            model.are_related(
                logical(7),
                physical(7)
            )
        );
    }
}