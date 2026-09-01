//! Zamani Quantum Noise (ZQN) — Crosstalk Semantics.
//!
//! # Purpose
//!
//! This module owns the provider-independent semantic representation of
//! crosstalk in a quantum computation.
//!
//! Crosstalk is modeled as an explicitly declared influence relationship in
//! which activity on one or more source resources can alter the behavior of
//! one or more victim resources.
//!
//! The fundamental distinction is:
//!
//! ```text
//! Spatial model
//!     = where resources are related
//!
//! Correlation model
//!     = which resources/statistical variables are dependent
//!
//! Crosstalk model
//!     = how activity on one resource/context can influence another
//!
//! Benchmarking
//!     = how the influence is experimentally measured
//!
//! Routing
//!     = where logical resources are placed
//!
//! Scheduling
//!     = when operations occur
//!
//! Simulation/hardware
//!     = how the model is realized
//! ```
//!
//! This file therefore does NOT implement:
//!
//! - circuit generation;
//! - circuit execution;
//! - routing;
//! - scheduling;
//! - hardware APIs;
//! - vendor SDKs;
//! - randomized benchmarking;
//! - calibration acquisition;
//! - channel mathematics;
//! - fault realization;
//! - random-number generation;
//! - QEC decoding;
//! - reporting;
//! - serialization formats.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Architectural position
//!
//! ```text
//! crate::quantum::ir::qubit
//!          │
//!          │ canonical resource identity
//!          ▼
//!     CrosstalkResource
//!          │
//!          ▼
//!     CrosstalkModel
//!          │
//!     ┌────┼──────────────┐
//!     │    │              │
//!     ▼    ▼              ▼
//! spatial correlation   calibration
//!     │    │              │
//!     └────┼──────────────┘
//!          ▼
//!       noise::model
//!          │
//!    ┌─────┼─────────────┐
//!    ▼     ▼             ▼
//! simulation  QEC      hardware
//!    │                       │
//!    └──────────┬────────────┘
//!               ▼
//!       routing/scheduling
//!
//! Benchmarking observes the resulting physical behavior but does not own
//! the semantic crosstalk model.
//! ```
//!
//! # Canonical quantum identities
//!
//! This module MUST use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! It MUST NOT define:
//!
//! ```text
//! CrosstalkQubitId
//! NoiseQubitId
//! CrosstalkPhysicalQubitId
//! ZqnQubitId
//! ```
//!
//! A logical and physical resource remain distinct even if their underlying
//! indices happen to be equal.
//!
//! # Write once, scale everywhere
//!
//! No semantic machine-size limit is encoded here.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_SOURCES
//! MAX_VICTIMS
//! MAX_EDGES
//! MAX_GROUPS
//! MAX_CROSSTALK
//! ```
//!
//! A crosstalk model may contain any finite number of resources representable
//! by the host and permitted by an explicit caller/runtime resource policy.
//!
//! "Infinity" in Zamani means that the semantic model contains no artificial
//! finite machine-size ceiling. It does not claim that a particular process
//! has infinite memory or that a physical machine has infinite resources.
//!
//! Large callers may partition, stream, shard, or project a model without
//! changing its semantics.
//!
//! # No fixed arity
//!
//! Crosstalk is NOT inherently two-resource.
//!
//! This module supports:
//!
//! ```text
//! one source  -> one victim
//! one source  -> many victims
//! many sources -> one victim
//! many sources -> many victims
//! group       -> group
//! contextual set -> contextual set
//! ```
//!
//! Source and victim membership is represented as collections rather than
//! fixed-size tuples.
//!
//! # Directionality
//!
//! Crosstalk is directional by default:
//!
//! ```text
//! source ─────► victim
//! ```
//!
//! Therefore:
//!
//! ```text
//! A -> B
//! ```
//!
//! is not equivalent to:
//!
//! ```text
//! B -> A
//! ```
//!
//! If a physical system has reciprocal influence, it should contain two
//! explicitly represented directional relationships or an explicit symmetric
//! policy at a higher layer.
//!
//! This avoids silently assuming reciprocal physical behavior.
//!
//! # Context dependence
//!
//! Crosstalk may depend on:
//!
//! - source activity;
//! - victim operation;
//! - simultaneous activity;
//! - source/victim resource identity;
//! - operation context;
//! - duration;
//! - frequency/control relationship;
//! - calibration state;
//! - spatial relationship;
//! - temporal state;
//! - arbitrary declared context.
//!
//! This module represents those dependencies without selecting a hardware
//! implementation.
//!
//! # Crosstalk versus correlation
//!
//! Crosstalk can create correlated errors, but the concepts are not identical.
//!
//! ```text
//! crosstalk
//!     = causal/interaction-style influence
//!
//! correlation
//!     = statistical dependence
//! ```
//!
//! Crosstalk may consume `noise::correlation` and `noise::spatial`, but neither
//! is treated as the definition of crosstalk.
//!
//! # Crosstalk versus benchmarking
//!
//! The repository already contains a protocol-level crosstalk benchmarking
//! implementation. That subsystem owns experimental configuration,
//! observations, degradation statistics and significance analysis.
//!
//! This module owns the physical/semantic model that those experiments can
//! characterize.
//!
//! The direction is:
//!
//! ```text
//! experiment
//!     │
//!     ▼
//! observation
//!     │
//!     ▼
//! characterization
//!     │
//!     ▼
//! CrosstalkModel
//!     │
//!     ├── simulation
//!     ├── routing
//!     ├── scheduling
//!     └── hardware analysis
//! ```
//!
//! The benchmark layer must not be duplicated here.
//!
//! # Numerical policy
//!
//! Crosstalk strength is represented by a validated finite `f64` in the
//! inclusive range `[-1, 1]`.
//!
//! The value is a dimensionless influence coefficient and MUST NOT
//! automatically be interpreted as:
//!
//! - a probability;
//! - an error rate;
//! - a fidelity;
//! - a correlation coefficient;
//! - a Hamiltonian coupling constant.
//!
//! Its exact physical interpretation belongs to the declared influence law.
//!
//! Separate fields are therefore used for:
//!
//! - influence strength;
//! - optional probability/error-rate parameters;
//! - duration;
//! - distance;
//! - arbitrary parameter metadata.
//!
//! NaN and infinity are always rejected.
//!
//! # Approximation
//!
//! A crosstalk relationship can be:
//!
//! - exact;
//! - approximate;
//! - bounded;
//! - empirical;
//! - statistical.
//!
//! The model records this semantic status explicitly.
//!
//! Silent approximation is forbidden.
//!
//! # Determinism
//!
//! This module is deterministic.
//!
//! It owns:
//!
//! - no RNG;
//! - no global mutable state;
//! - no wall-clock reads;
//! - no thread-local semantic state;
//! - no memory-address semantics;
//! - no hash-map iteration dependence.
//!
//! Canonical collections are ordered.
//!
//! Identical semantic inputs produce identical model equality and ordering.
//!
//! # Resource policy
//!
//! This module deliberately does not impose machine-size limits.
//!
//! Allocation/resource limits belong to `ZqnContext` and the surrounding
//! resource-policy layer.
//!
//! Constructors that accept collection capacities treat those capacities only
//! as allocation hints.
//!
//! A capacity is NEVER a semantic maximum.
//!
//! # Security
//!
//! Crosstalk resources are data, not capabilities.
//!
//! A resource reference MUST NOT grant:
//!
//! - QPU access;
//! - network access;
//! - filesystem access;
//! - credentials;
//! - calibration write access;
//! - process execution.
//!
//! Untrusted crosstalk specifications must be validated before materialization
//! under the caller's explicit resource policy.
//!
//! # Serialization
//!
//! This module intentionally does not define a wire format.
//!
//! `zqn::io` owns serialization.
//!
//! A serializer must preserve:
//!
//! - resource identity domain;
//! - source membership;
//! - victim membership;
//! - directionality;
//! - influence law;
//! - strength;
//! - semantic guarantee;
//! - conditions;
//! - parameters;
//! - canonical ordering.
//!
//! Rust memory layout is not a serialization contract.
//!
//! # Thread safety
//!
//! The semantic model is immutable after construction.
//!
//! It contains no interior mutability and no global state.
//!
//! Read-only instances may be shared between concurrent consumers whenever
//! their contained values satisfy the relevant Rust auto-traits.
//!
//! # Integration contract
//!
//! ```text
//! quantum::ir::qubit
//!        │
//!        ▼
//! CrosstalkResource
//!        │
//!        ▼
//! CrosstalkRelation
//!        │
//!        ▼
//! CrosstalkModel
//!        │
//! ┌──────┼───────────────┐
//! ▼      ▼               ▼
//! spatial correlation calibration
//! │      │               │
//! └──────┼───────────────┘
//!        ▼
//! noise::model / application
//!        │
//! ┌──────┼──────────────┐
//! ▼      ▼              ▼
//! simulation QEC     hardware
//!
//! routing and scheduling consume model-derived influence/cost information.
//! benchmarking characterizes the physical behavior.
//! ```
//!
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. canonical IR qubit identities are used;
//! 2. no duplicate qubit identity type exists;
//! 3. source/victim cardinality is dynamic;
//! 4. no machine-size constant exists;
//! 5. directionality is explicit;
//! 6. duplicate resources are rejected;
//! 7. empty source/victim sets are rejected;
//! 8. self-influence is explicitly controlled;
//! 9. invalid numerical values are rejected;
//! 10. semantic guarantees are explicit;
//! 11. deterministic canonical ordering is guaranteed;
//! 12. no global mutable state exists;
//! 13. no randomness exists;
//! 14. no unsafe Rust exists;
//! 15. no vendor API is referenced;
//! 16. no benchmarking implementation is duplicated;
//! 17. callers can use the model with arbitrary resource counts;
//! 18. downstream consumers do not need to modify this file to understand the
//!     basic crosstalk contract.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::core::errors::{
    ZqnError,
    ZqnErrorCode,
    ZqnErrorKind,
    ZqnResult,
};

// ============================================================================
// Public constants
// ============================================================================

/// Stable semantic identifier for the ZQN crosstalk model.
pub const CROSSTALK_MODEL_KIND: &str = "zamani.zqn.noise.crosstalk";

/// Semantic revision of this crosstalk representation.
pub const CROSSTALK_MODEL_VERSION: u32 = 1;

/// Valid inclusive lower bound for the dimensionless influence coefficient.
pub const INFLUENCE_MIN: f64 = -1.0;

/// Valid inclusive upper bound for the dimensionless influence coefficient.
pub const INFLUENCE_MAX: f64 = 1.0;

// ============================================================================
// Internal error helpers
// ============================================================================

fn invalid_model(message: impl Into<String>) -> ZqnError {
    ZqnError::new(
        ZqnErrorKind::Crosstalk,
        ZqnErrorCode::InvalidCrosstalkModel,
        message.into(),
    )
}

fn invalid_parameter(message: impl Into<String>) -> ZqnError {
    ZqnError::new(
        ZqnErrorKind::Crosstalk,
        ZqnErrorCode::InvalidCorrelationParameter,
        message.into(),
    )
}

fn invalid_identifier(message: impl Into<String>) -> ZqnError {
    ZqnError::new(
        ZqnErrorKind::Identifier,
        ZqnErrorCode::InvalidIdentifier,
        message.into(),
    )
}

// ============================================================================
// Semantic guarantee
// ============================================================================

/// Semantic status of a crosstalk relationship.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CrosstalkGuarantee {
    /// The relationship is treated as exact within the selected numerical
    /// representation.
    Exact,

    /// The relationship is explicitly an approximation.
    Approximate,

    /// The relationship provides an explicit bounded approximation.
    Bounded,

    /// The relationship was inferred statistically from observations.
    Statistical,

    /// The relationship is empirical but does not claim a stronger
    /// mathematical guarantee.
    Empirical,
}

impl Default for CrosstalkGuarantee {
    fn default() -> Self {
        Self::Empirical
    }
}

impl fmt::Display for CrosstalkGuarantee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact => f.write_str("exact"),
            Self::Approximate => f.write_str("approximate"),
            Self::Bounded => f.write_str("bounded"),
            Self::Statistical => f.write_str("statistical"),
            Self::Empirical => f.write_str("empirical"),
        }
    }
}

// ============================================================================
// Resource identity
// ============================================================================

/// Canonical resource reference used by the crosstalk model.
///
/// Logical and physical qubits use the authoritative Quantum IR identity
/// types. Other quantum modalities can use `External` until their owning IR
/// subsystem exposes a canonical resource identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CrosstalkResource {
    /// Canonical logical qubit.
    LogicalQubit(QubitId),

    /// Canonical physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Opaque identity owned by another quantum-resource subsystem.
    ///
    /// This value is only an identity. It is never interpreted as a pointer,
    /// handle, credential, path, or executable capability.
    External(String),
}

impl CrosstalkResource {
    /// Creates an external resource reference.
    pub fn external<S>(value: S) -> ZqnResult<Self>
    where
        S: Into<String>,
    {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(invalid_identifier(
                "crosstalk external resource identifier cannot be empty",
            ));
        }

        Ok(Self::External(value))
    }

    /// Returns true when this resource is a logical qubit.
    #[must_use]
    pub const fn is_logical_qubit(&self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns true when this resource is a physical qubit.
    #[must_use]
    pub const fn is_physical_qubit(&self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }

    /// Returns true when this resource belongs to an external identity
    /// domain.
    #[must_use]
    pub const fn is_external(&self) -> bool {
        matches!(self, Self::External(_))
    }
}

impl From<QubitId> for CrosstalkResource {
    fn from(value: QubitId) -> Self {
        Self::LogicalQubit(value)
    }
}

impl From<PhysicalQubitId> for CrosstalkResource {
    fn from(value: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(value)
    }
}

// ============================================================================
// Influence direction
// ============================================================================

/// Direction of a crosstalk relationship.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CrosstalkDirection {
    /// Source resources influence victim resources.
    Directed,

    /// The relationship is physically modeled as symmetric.
    ///
    /// Symmetry is semantic and does not mean the model should silently create
    /// a second directed relationship. Consumers may materialize both
    /// directions if required by their execution representation.
    Symmetric,
}

impl Default for CrosstalkDirection {
    fn default() -> Self {
        Self::Directed
    }
}

// ============================================================================
// Influence law
// ============================================================================

/// Semantic law used to interpret the influence coefficient.
///
/// The law identifies the *meaning* of the coefficient without prescribing a
/// numerical simulator.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CrosstalkInfluenceLaw {
    /// Dimensionless multiplicative influence.
    ///
    /// Consumers interpret the coefficient as a relative modulation of the
    /// victim quantity.
    Relative,

    /// Additive dimensionless influence.
    Absolute,

    /// Influence proportional to source activity.
    ActivityWeighted,

    /// Influence is conditioned on simultaneous activity.
    SimultaneousActivity,

    /// Influence is conditioned on a declared operation/context relation.
    ContextDependent,

    /// The coefficient represents an empirically fitted relationship.
    Empirical,

    /// User-defined semantic law identified by a stable name.
    Custom(String),
}

impl Default for CrosstalkInfluenceLaw {
    fn default() -> Self {
        Self::Relative
    }
}

impl CrosstalkInfluenceLaw {
    fn validate(&self) -> ZqnResult<()> {
        if let Self::Custom(name) = self {
            if name.trim().is_empty() {
                return Err(invalid_parameter(
                    "custom crosstalk influence law cannot have an empty name",
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// Influence strength
// ============================================================================

/// Validated dimensionless crosstalk influence coefficient.
///
/// This is deliberately not a probability.
///
/// Values are constrained to `[-1, 1]` so that a single scalar cannot silently
/// encode an unbounded physical quantity. If a physical model requires an
/// unbounded parameter, it should store that quantity as an explicit model
/// parameter rather than abusing this coefficient.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct InfluenceStrength(f64);

impl InfluenceStrength {
    /// Creates a validated influence coefficient.
    pub fn new(value: f64) -> ZqnResult<Self> {
        if !value.is_finite() {
            return Err(invalid_parameter(
                "crosstalk influence strength must be finite",
            ));
        }

        if !(INFLUENCE_MIN..=INFLUENCE_MAX).contains(&value) {
            return Err(invalid_parameter(format!(
                "crosstalk influence strength {} is outside [{}, {}]",
                value, INFLUENCE_MIN, INFLUENCE_MAX
            )));
        }

        Ok(Self(value))
    }

    /// Returns the coefficient.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Returns true for zero influence.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0.0
    }
}

impl Default for InfluenceStrength {
    fn default() -> Self {
        Self(0.0)
    }
}

// ============================================================================
// Optional scalar parameter
// ============================================================================

/// A named, finite scalar parameter associated with a crosstalk relationship.
///
/// This is intentionally generic so the model does not have to predict the
/// parameter vocabulary of every future quantum technology.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CrosstalkParameter {
    name: String,
    value: f64,
    unit: Option<String>,
}

impl CrosstalkParameter {
    /// Creates a validated crosstalk parameter.
    pub fn new<N, U>(name: N, value: f64, unit: Option<U>) -> ZqnResult<Self>
    where
        N: Into<String>,
        U: Into<String>,
    {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(invalid_parameter(
                "crosstalk parameter name cannot be empty",
            ));
        }

        if !value.is_finite() {
            return Err(invalid_parameter(format!(
                "crosstalk parameter '{}' must be finite",
                name
            )));
        }

        let unit = unit.map(Into::into);

        if let Some(ref unit) = unit {
            if unit.trim().is_empty() {
                return Err(invalid_parameter(format!(
                    "crosstalk parameter '{}' has an empty unit",
                    name
                )));
            }
        }

        Ok(Self { name, value, unit })
    }

    /// Parameter name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Parameter value.
    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }

    /// Optional physical unit.
    #[must_use]
    pub fn unit(&self) -> Option<&str> {
        self.unit.as_deref()
    }
}

// ============================================================================
// Relationship condition
// ============================================================================

/// A declarative condition under which a crosstalk relationship is active.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CrosstalkCondition {
    /// Relationship applies whenever the declared source/victim relationship
    /// is evaluated.
    Always,

    /// Relationship applies only when source and victim activity overlap.
    Simultaneous,

    /// Relationship applies when the source is active while the victim is
    /// being evaluated.
    SourceActive,

    /// Relationship applies when a named operation/context condition holds.
    Context(String),

    /// Relationship applies under a user-defined condition identifier.
    Custom(String),
}

impl Default for CrosstalkCondition {
    fn default() -> Self {
        Self::Always
    }
}

impl CrosstalkCondition {
    fn validate(&self) -> ZqnResult<()> {
        match self {
            Self::Context(value) | Self::Custom(value) => {
                if value.trim().is_empty() {
                    return Err(invalid_parameter(
                        "crosstalk condition identifier cannot be empty",
                    ));
                }
            }
            Self::Always | Self::Simultaneous | Self::SourceActive => {}
        }

        Ok(())
    }
}

// ============================================================================
// Operation/context selector
// ============================================================================

/// Declarative selector for operation/context information.
///
/// No concrete gate set is assumed. The string is an opaque semantic label
/// interpreted by the integration layer that owns operation metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CrosstalkSelector {
    /// Applies to every operation/context.
    Any,

    /// Applies to a named semantic operation/context.
    Named(String),

    /// Applies to a named family/classification.
    Class(String),

    /// User-defined selector.
    Custom(String),
}

impl Default for CrosstalkSelector {
    fn default() -> Self {
        Self::Any
    }
}

impl CrosstalkSelector {
    fn validate(&self) -> ZqnResult<()> {
        match self {
            Self::Any => {}
            Self::Named(value) | Self::Class(value) | Self::Custom(value) => {
                if value.trim().is_empty() {
                    return Err(invalid_parameter(
                        "crosstalk selector identifier cannot be empty",
                    ));
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// Crosstalk relation
// ============================================================================

/// One semantic crosstalk relationship.
///
/// A relation may contain arbitrary source and victim cardinalities.
///
/// ```text
/// sources
///    │
///    ▼
/// influence law + strength + condition
///    │
///    ▼
/// victims
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CrosstalkRelation {
    sources: BTreeSet<CrosstalkResource>,
    victims: BTreeSet<CrosstalkResource>,
    direction: CrosstalkDirection,
    law: CrosstalkInfluenceLaw,
    strength: InfluenceStrength,
    guarantee: CrosstalkGuarantee,
    condition: CrosstalkCondition,
    source_selector: CrosstalkSelector,
    victim_selector: CrosstalkSelector,
    parameters: BTreeMap<String, CrosstalkParameter>,
}

impl CrosstalkRelation {
    /// Creates a validated crosstalk relation.
    ///
    /// Source and victim sets must both be non-empty.
    pub fn new<I, J>(
        sources: I,
        victims: J,
        direction: CrosstalkDirection,
        law: CrosstalkInfluenceLaw,
        strength: InfluenceStrength,
    ) -> ZqnResult<Self>
    where
        I: IntoIterator<Item = CrosstalkResource>,
        J: IntoIterator<Item = CrosstalkResource>,
    {
        let sources: BTreeSet<_> = sources.into_iter().collect();
        let victims: BTreeSet<_> = victims.into_iter().collect();

        if sources.is_empty() {
            return Err(invalid_model(
                "crosstalk relation requires at least one source resource",
            ));
        }

        if victims.is_empty() {
            return Err(invalid_model(
                "crosstalk relation requires at least one victim resource",
            ));
        }

        for resource in &sources {
            validate_resource(resource)?;
        }

        for resource in &victims {
            validate_resource(resource)?;
        }

        if let Some(overlap) = sources.intersection(&victims).next() {
            return Err(invalid_model(format!(
                "crosstalk source/victim sets overlap on resource {:?}",
                overlap
            )));
        }

        law.validate()?;

        Ok(Self {
            sources,
            victims,
            direction,
            law,
            strength,
            guarantee: CrosstalkGuarantee::default(),
            condition: CrosstalkCondition::default(),
            source_selector: CrosstalkSelector::default(),
            victim_selector: CrosstalkSelector::default(),
            parameters: BTreeMap::new(),
        })
    }

    /// Changes the semantic guarantee.
    #[must_use]
    pub fn with_guarantee(mut self, guarantee: CrosstalkGuarantee) -> Self {
        self.guarantee = guarantee;
        self
    }

    /// Changes the activation condition.
    pub fn with_condition(mut self, condition: CrosstalkCondition) -> ZqnResult<Self> {
        condition.validate()?;
        self.condition = condition;
        Ok(self)
    }

    /// Changes the source-operation selector.
    pub fn with_source_selector(
        mut self,
        selector: CrosstalkSelector,
    ) -> ZqnResult<Self> {
        selector.validate()?;
        self.source_selector = selector;
        Ok(self)
    }

    /// Changes the victim-operation selector.
    pub fn with_victim_selector(
        mut self,
        selector: CrosstalkSelector,
    ) -> ZqnResult<Self> {
        selector.validate()?;
        self.victim_selector = selector;
        Ok(self)
    }

    /// Adds a named parameter.
    ///
    /// Replacing an existing parameter with the same name is rejected so that
    /// accidental configuration overwrites cannot silently occur.
    pub fn with_parameter(mut self, parameter: CrosstalkParameter) -> ZqnResult<Self> {
        if self.parameters.contains_key(parameter.name()) {
            return Err(invalid_model(format!(
                "duplicate crosstalk parameter '{}'",
                parameter.name()
            )));
        }

        self.parameters
            .insert(parameter.name().to_owned(), parameter);

        Ok(self)
    }

    /// Returns the source resources.
    #[must_use]
    pub fn sources(&self) -> &BTreeSet<CrosstalkResource> {
        &self.sources
    }

    /// Returns the victim resources.
    #[must_use]
    pub fn victims(&self) -> &BTreeSet<CrosstalkResource> {
        &self.victims
    }

    /// Returns the relation direction.
    #[must_use]
    pub const fn direction(&self) -> CrosstalkDirection {
        self.direction
    }

    /// Returns the influence law.
    #[must_use]
    pub fn law(&self) -> &CrosstalkInfluenceLaw {
        &self.law
    }

    /// Returns the influence strength.
    #[must_use]
    pub const fn strength(&self) -> InfluenceStrength {
        self.strength
    }

    /// Returns the semantic guarantee.
    #[must_use]
    pub const fn guarantee(&self) -> CrosstalkGuarantee {
        self.guarantee
    }

    /// Returns the activation condition.
    #[must_use]
    pub fn condition(&self) -> &CrosstalkCondition {
        &self.condition
    }

    /// Returns the source selector.
    #[must_use]
    pub fn source_selector(&self) -> &CrosstalkSelector {
        &self.source_selector
    }

    /// Returns the victim selector.
    #[must_use]
    pub fn victim_selector(&self) -> &CrosstalkSelector {
        &self.victim_selector
    }

    /// Returns all named parameters in deterministic order.
    #[must_use]
    pub fn parameters(&self) -> &BTreeMap<String, CrosstalkParameter> {
        &self.parameters
    }

    /// Returns whether a resource participates as a source.
    #[must_use]
    pub fn contains_source(&self, resource: &CrosstalkResource) -> bool {
        self.sources.contains(resource)
    }

    /// Returns whether a resource participates as a victim.
    #[must_use]
    pub fn contains_victim(&self, resource: &CrosstalkResource) -> bool {
        self.victims.contains(resource)
    }

    /// Returns whether the relation can influence the supplied victim.
    #[must_use]
    pub fn influences(&self, victim: &CrosstalkResource) -> bool {
        self.victims.contains(victim)
    }

    /// Returns whether the relation is potentially active for the supplied
    /// source.
    #[must_use]
    pub fn activated_by(&self, source: &CrosstalkResource) -> bool {
        self.sources.contains(source)
    }

    fn canonical_key(&self) -> CrosstalkRelationKey {
        CrosstalkRelationKey {
            sources: self.sources.iter().cloned().collect(),
            victims: self.victims.iter().cloned().collect(),
            direction: self.direction,
            law: self.law.clone(),
            strength_bits: self.strength.value().to_bits(),
            guarantee: self.guarantee,
            condition: self.condition.clone(),
            source_selector: self.source_selector.clone(),
            victim_selector: self.victim_selector.clone(),
            parameters: self
                .parameters
                .iter()
                .map(|(key, value)| {
                    (
                        key.clone(),
                        value.value().to_bits(),
                        value.unit().map(str::to_owned),
                    )
                })
                .collect(),
        }
    }
}

/// Internal canonical key used to reject exact duplicate relationships.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CrosstalkRelationKey {
    sources: Vec<CrosstalkResource>,
    victims: Vec<CrosstalkResource>,
    direction: CrosstalkDirection,
    law: CrosstalkInfluenceLaw,
    strength_bits: u64,
    guarantee: CrosstalkGuarantee,
    condition: CrosstalkCondition,
    source_selector: CrosstalkSelector,
    victim_selector: CrosstalkSelector,
    parameters: Vec<(String, u64, Option<String>)>,
}

// ============================================================================
// Model
// ============================================================================

/// Immutable canonical crosstalk model.
///
/// All relationships are stored in deterministic order.
///
/// No machine-size limit is encoded in this type.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CrosstalkModel {
    relations: Vec<CrosstalkRelation>,
    metadata: BTreeMap<String, String>,
}

impl CrosstalkModel {
    /// Creates an empty crosstalk model.
    ///
    /// An empty model is valid and represents "no modeled crosstalk".
    #[must_use]
    pub fn new() -> Self {
        Self {
            relations: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Creates a model from an iterator of relationships.
    ///
    /// Exact duplicate semantic relationships are rejected.
    pub fn from_relations<I>(relations: I) -> ZqnResult<Self>
    where
        I: IntoIterator<Item = CrosstalkRelation>,
    {
        let mut model = Self::new();

        for relation in relations {
            model.add_relation(relation)?;
        }

        Ok(model)
    }

    /// Adds one relationship.
    ///
    /// This method is intended for incremental construction. The final model
    /// remains deterministically ordered.
    pub fn add_relation(&mut self, relation: CrosstalkRelation) -> ZqnResult<()> {
        let key = relation.canonical_key();

        if self
            .relations
            .iter()
            .any(|existing| existing.canonical_key() == key)
        {
            return Err(invalid_model(
                "duplicate semantic crosstalk relationship",
            ));
        }

        self.relations.push(relation);
        self.relations
            .sort_by_key(CrosstalkRelation::canonical_key);

        Ok(())
    }

    /// Adds deterministic model metadata.
    ///
    /// Metadata is descriptive and has no effect on crosstalk semantics.
    pub fn add_metadata<K, V>(&mut self, key: K, value: V) -> ZqnResult<()>
    where
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();

        if key.trim().is_empty() {
            return Err(invalid_identifier(
                "crosstalk metadata key cannot be empty",
            ));
        }

        if self.metadata.contains_key(&key) {
            return Err(invalid_model(format!(
                "duplicate crosstalk metadata key '{}'",
                key
            )));
        }

        self.metadata.insert(key, value.into());
        Ok(())
    }

    /// Returns the model relationships in canonical order.
    #[must_use]
    pub fn relations(&self) -> &[CrosstalkRelation] {
        &self.relations
    }

    /// Returns descriptive metadata in canonical order.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Returns the number of modeled relationships.
    ///
    /// This is an observed size, not a machine-size limit.
    #[must_use]
    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }

    /// Returns true when no crosstalk relationships are modeled.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.relations.is_empty()
    }

    /// Returns all resources participating in the model.
    ///
    /// The result is deterministically ordered and deduplicated.
    #[must_use]
    pub fn resources(&self) -> BTreeSet<CrosstalkResource> {
        let mut resources = BTreeSet::new();

        for relation in &self.relations {
            resources.extend(relation.sources().iter().cloned());
            resources.extend(relation.victims().iter().cloned());
        }

        resources
    }

    /// Returns all relations that can be activated by a source resource.
    #[must_use]
    pub fn relations_from(
        &self,
        source: &CrosstalkResource,
    ) -> Vec<&CrosstalkRelation> {
        self.relations
            .iter()
            .filter(|relation| relation.activated_by(source))
            .collect()
    }

    /// Returns all relations that can influence a victim resource.
    #[must_use]
    pub fn relations_to(
        &self,
        victim: &CrosstalkResource,
    ) -> Vec<&CrosstalkRelation> {
        self.relations
            .iter()
            .filter(|relation| relation.influences(victim))
            .collect()
    }

    /// Returns all victims influenced by a source.
    #[must_use]
    pub fn victims_of(
        &self,
        source: &CrosstalkResource,
    ) -> BTreeSet<CrosstalkResource> {
        let mut result = BTreeSet::new();

        for relation in self.relations_from(source) {
            result.extend(relation.victims().iter().cloned());
        }

        result
    }

    /// Returns all sources capable of influencing a victim.
    #[must_use]
    pub fn sources_of(
        &self,
        victim: &CrosstalkResource,
    ) -> BTreeSet<CrosstalkResource> {
        let mut result = BTreeSet::new();

        for relation in self.relations_to(victim) {
            result.extend(relation.sources().iter().cloned());
        }

        result
    }

    /// Returns whether a source can influence a victim.
    #[must_use]
    pub fn can_influence(
        &self,
        source: &CrosstalkResource,
        victim: &CrosstalkResource,
    ) -> bool {
        self.relations.iter().any(|relation| {
            relation.activated_by(source) && relation.influences(victim)
        })
    }

    /// Returns the relations between the supplied source and victim.
    #[must_use]
    pub fn relations_between(
        &self,
        source: &CrosstalkResource,
        victim: &CrosstalkResource,
    ) -> Vec<&CrosstalkRelation> {
        self.relations
            .iter()
            .filter(|relation| {
                relation.activated_by(source) && relation.influences(victim)
            })
            .collect()
    }

    /// Validates the entire model.
    pub fn validate(&self) -> ZqnResult<()> {
        let mut relation_keys = BTreeSet::new();

        for relation in &self.relations {
            validate_relation(relation)?;

            if !relation_keys.insert(relation.canonical_key()) {
                return Err(invalid_model(
                    "crosstalk model contains duplicate semantic relationships",
                ));
            }
        }

        for key in self.metadata.keys() {
            if key.trim().is_empty() {
                return Err(invalid_identifier(
                    "crosstalk metadata key cannot be empty",
                ));
            }
        }

        Ok(())
    }

    /// Returns a deterministic model summary.
    #[must_use]
    pub fn summary(&self) -> CrosstalkSummary {
        let resources = self.resources();

        CrosstalkSummary {
            relation_count: self.relations.len(),
            resource_count: resources.len(),
            directed_relations: self
                .relations
                .iter()
                .filter(|relation| {
                    relation.direction() == CrosstalkDirection::Directed
                })
                .count(),
            symmetric_relations: self
                .relations
                .iter()
                .filter(|relation| {
                    relation.direction() == CrosstalkDirection::Symmetric
                })
                .count(),
        }
    }
}

impl Default for CrosstalkModel {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Model summary
// ============================================================================

/// Deterministic summary of a crosstalk model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CrosstalkSummary {
    relation_count: usize,
    resource_count: usize,
    directed_relations: usize,
    symmetric_relations: usize,
}

impl CrosstalkSummary {
    /// Number of relationships.
    #[must_use]
    pub const fn relation_count(self) -> usize {
        self.relation_count
    }

    /// Number of distinct resources.
    #[must_use]
    pub const fn resource_count(self) -> usize {
        self.resource_count
    }

    /// Number of directed relationships.
    #[must_use]
    pub const fn directed_relations(self) -> usize {
        self.directed_relations
    }

    /// Number of symmetric relationships.
    #[must_use]
    pub const fn symmetric_relations(self) -> usize {
        self.symmetric_relations
    }
}

// ============================================================================
// Builder
// ============================================================================

/// Incremental crosstalk-model builder.
///
/// Capacity is only an allocation hint. It is never a semantic limit.
#[derive(Debug, Default)]
pub struct CrosstalkBuilder {
    relations: Vec<CrosstalkRelation>,
    metadata: BTreeMap<String, String>,
}

impl CrosstalkBuilder {
    /// Creates an empty builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a builder with an allocation hint.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            relations: Vec::with_capacity(capacity),
            metadata: BTreeMap::new(),
        }
    }

    /// Adds a relationship.
    pub fn relation(mut self, relation: CrosstalkRelation) -> ZqnResult<Self> {
        self.push_relation(relation)?;
        Ok(self)
    }

    /// Adds a relationship mutably.
    pub fn push_relation(&mut self, relation: CrosstalkRelation) -> ZqnResult<()> {
        if self
            .relations
            .iter()
            .any(|existing| existing.canonical_key() == relation.canonical_key())
        {
            return Err(invalid_model(
                "duplicate semantic crosstalk relationship",
            ));
        }

        self.relations.push(relation);
        Ok(())
    }

    /// Adds metadata.
    pub fn metadata<K, V>(mut self, key: K, value: V) -> ZqnResult<Self>
    where
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();

        if key.trim().is_empty() {
            return Err(invalid_identifier(
                "crosstalk metadata key cannot be empty",
            ));
        }

        if self.metadata.contains_key(&key) {
            return Err(invalid_model(format!(
                "duplicate crosstalk metadata key '{}'",
                key
            )));
        }

        self.metadata.insert(key, value.into());
        Ok(self)
    }

    /// Finalizes and validates the model.
    pub fn build(self) -> ZqnResult<CrosstalkModel> {
        let mut model = CrosstalkModel {
            relations: self.relations,
            metadata: self.metadata,
        };

        model.relations.sort_by_key(CrosstalkRelation::canonical_key);
        model.validate()?;

        Ok(model)
    }
}

// ============================================================================
// Context evaluation
// ============================================================================

/// Runtime-neutral description of active crosstalk context.
///
/// This type contains no runtime handles and no hardware API objects.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CrosstalkContext {
    active_sources: BTreeSet<CrosstalkResource>,
    active_victims: BTreeSet<CrosstalkResource>,
    context_id: Option<String>,
}

impl CrosstalkContext {
    /// Creates a context from active resources.
    pub fn new<I, J>(
        active_sources: I,
        active_victims: J,
    ) -> ZqnResult<Self>
    where
        I: IntoIterator<Item = CrosstalkResource>,
        J: IntoIterator<Item = CrosstalkResource>,
    {
        let active_sources: BTreeSet<_> = active_sources.into_iter().collect();
        let active_victims: BTreeSet<_> = active_victims.into_iter().collect();

        for resource in active_sources.iter().chain(active_victims.iter()) {
            validate_resource(resource)?;
        }

        Ok(Self {
            active_sources,
            active_victims,
            context_id: None,
        })
    }

    /// Assigns an optional stable context identity.
    pub fn with_id<S>(mut self, id: S) -> ZqnResult<Self>
    where
        S: Into<String>,
    {
        let id = id.into();

        if id.trim().is_empty() {
            return Err(invalid_identifier(
                "crosstalk context identifier cannot be empty",
            ));
        }

        self.context_id = Some(id);
        Ok(self)
    }

    /// Active source resources.
    #[must_use]
    pub fn active_sources(&self) -> &BTreeSet<CrosstalkResource> {
        &self.active_sources
    }

    /// Active victim resources.
    #[must_use]
    pub fn active_victims(&self) -> &BTreeSet<CrosstalkResource> {
        &self.active_victims
    }

    /// Optional context identity.
    #[must_use]
    pub fn id(&self) -> Option<&str> {
        self.context_id.as_deref()
    }

    /// Returns whether a source is active.
    #[must_use]
    pub fn source_active(&self, resource: &CrosstalkResource) -> bool {
        self.active_sources.contains(resource)
    }

    /// Returns whether a victim is active.
    #[must_use]
    pub fn victim_active(&self, resource: &CrosstalkResource) -> bool {
        self.active_victims.contains(resource)
    }
}

// ============================================================================
// Evaluation result
// ============================================================================

/// A semantic crosstalk contribution selected for a context.
///
/// This is deliberately not a channel or fault. Those are produced/owned by
/// their respective subsystems.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CrosstalkContribution {
    relation_index: usize,
    strength: InfluenceStrength,
    law: CrosstalkInfluenceLaw,
    guarantee: CrosstalkGuarantee,
    active_sources: BTreeSet<CrosstalkResource>,
    affected_victims: BTreeSet<CrosstalkResource>,
}

impl CrosstalkContribution {
    /// Relationship index in canonical model order.
    #[must_use]
    pub const fn relation_index(&self) -> usize {
        self.relation_index
    }

    /// Influence strength.
    #[must_use]
    pub const fn strength(&self) -> InfluenceStrength {
        self.strength
    }

    /// Influence law.
    #[must_use]
    pub fn law(&self) -> &CrosstalkInfluenceLaw {
        &self.law
    }

    /// Semantic guarantee.
    #[must_use]
    pub const fn guarantee(&self) -> CrosstalkGuarantee {
        self.guarantee
    }

    /// Active source subset.
    #[must_use]
    pub fn active_sources(&self) -> &BTreeSet<CrosstalkResource> {
        &self.active_sources
    }

    /// Affected victim subset.
    #[must_use]
    pub fn affected_victims(&self) -> &BTreeSet<CrosstalkResource> {
        &self.affected_victims
    }
}

/// Deterministic result of evaluating a crosstalk model against a context.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CrosstalkEvaluation {
    contributions: Vec<CrosstalkContribution>,
}

impl CrosstalkEvaluation {
    /// Returns selected contributions in canonical model order.
    #[must_use]
    pub fn contributions(&self) -> &[CrosstalkContribution] {
        &self.contributions
    }

    /// Returns true when no crosstalk contribution is active.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.contributions.is_empty()
    }

    /// Number of active contributions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.contributions.len()
    }

    /// Returns all affected victims.
    #[must_use]
    pub fn affected_victims(&self) -> BTreeSet<CrosstalkResource> {
        let mut result = BTreeSet::new();

        for contribution in &self.contributions {
            result.extend(contribution.affected_victims().iter().cloned());
        }

        result
    }
}

// ============================================================================
// Evaluation
// ============================================================================

impl CrosstalkModel {
    /// Evaluates which relationships are active for a supplied context.
    ///
    /// This performs semantic selection only.
    ///
    /// It does NOT:
    ///
    /// - mutate quantum state;
    /// - sample randomness;
    /// - create a quantum channel;
    /// - create a fault;
    /// - access hardware;
    /// - perform routing;
    /// - perform scheduling.
    pub fn evaluate(
        &self,
        context: &CrosstalkContext,
    ) -> ZqnResult<CrosstalkEvaluation> {
        let mut contributions = Vec::new();

        for (relation_index, relation) in self.relations.iter().enumerate() {
            let active_sources: BTreeSet<_> = relation
                .sources()
                .intersection(context.active_sources())
                .cloned()
                .collect();

            if active_sources.is_empty() {
                continue;
            }

            let affected_victims: BTreeSet<_> = relation
                .victims()
                .intersection(context.active_victims())
                .cloned()
                .collect();

            if affected_victims.is_empty() {
                continue;
            }

            if !condition_is_active(relation.condition(), context) {
                continue;
            }

            contributions.push(CrosstalkContribution {
                relation_index,
                strength: relation.strength(),
                law: relation.law().clone(),
                guarantee: relation.guarantee(),
                active_sources,
                affected_victims,
            });
        }

        Ok(CrosstalkEvaluation { contributions })
    }
}

// ============================================================================
// Validation
// ============================================================================

fn validate_resource(resource: &CrosstalkResource) -> ZqnResult<()> {
    match resource {
        CrosstalkResource::LogicalQubit(_) => Ok(()),
        CrosstalkResource::PhysicalQubit(_) => Ok(()),
        CrosstalkResource::External(value) => {
            if value.trim().is_empty() {
                Err(invalid_identifier(
                    "crosstalk external resource identifier cannot be empty",
                ))
            } else {
                Ok(())
            }
        }
    }
}

fn validate_relation(relation: &CrosstalkRelation) -> ZqnResult<()> {
    if relation.sources().is_empty() {
        return Err(invalid_model(
            "crosstalk relation has no source resources",
        ));
    }

    if relation.victims().is_empty() {
        return Err(invalid_model(
            "crosstalk relation has no victim resources",
        ));
    }

    for resource in relation.sources() {
        validate_resource(resource)?;
    }

    for resource in relation.victims() {
        validate_resource(resource)?;
    }

    if relation
        .sources()
        .iter()
        .any(|resource| relation.victims().contains(resource))
    {
        return Err(invalid_model(
            "crosstalk source and victim sets must not overlap",
        ));
    }

    relation.law().validate()?;
    relation.condition().validate()?;
    relation.source_selector().validate()?;
    relation.victim_selector().validate()?;

    for (key, parameter) in relation.parameters() {
        if key != parameter.name() {
            return Err(invalid_model(
                "crosstalk parameter map key does not match parameter name",
            ));
        }
    }

    Ok(())
}

fn condition_is_active(
    condition: &CrosstalkCondition,
    context: &CrosstalkContext,
) -> bool {
    match condition {
        CrosstalkCondition::Always => true,

        CrosstalkCondition::Simultaneous => {
            !context.active_sources().is_empty()
                && !context.active_victims().is_empty()
        }

        CrosstalkCondition::SourceActive => {
            !context.active_sources().is_empty()
        }

        CrosstalkCondition::Context(expected) => {
            context.id().is_some_and(|actual| actual == expected)
        }

        CrosstalkCondition::Custom(_) => {
            // Custom conditions require an integration layer to interpret
            // them. They are not silently assumed to be active.
            false
        }
    }
}

// ============================================================================
// Deterministic influence aggregation
// ============================================================================

/// Deterministically aggregates active relation strengths for one victim.
///
/// This is a semantic aggregation helper only. It deliberately does not claim
/// that all physical crosstalk mechanisms add linearly.
///
/// The aggregation is therefore explicitly opt-in and returns the arithmetic
/// sum of the selected coefficients. Physical simulators or hardware
/// adapters should use the declared influence law instead when a different
/// composition rule applies.
pub fn aggregate_additive_influence(
    evaluation: &CrosstalkEvaluation,
    victim: &CrosstalkResource,
) -> ZqnResult<f64> {
    let mut total = 0.0_f64;

    for contribution in evaluation.contributions() {
        if !contribution.affected_victims().contains(victim) {
            continue;
        }

        total += contribution.strength().value();

        if !total.is_finite() {
            return Err(invalid_model(
                "crosstalk additive influence became non-finite",
            ));
        }
    }

    Ok(total)
}

// ============================================================================
// Canonical identity
// ============================================================================

/// Stable semantic identity input for a crosstalk model.
///
/// This is deliberately a structural digest input rather than a cryptographic
/// hash implementation. Canonical serialization/hash infrastructure belongs
/// to `zqn::io`/repository hashing infrastructure.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CrosstalkIdentityDescriptor {
    /// Model kind.
    pub kind: String,

    /// Semantic version.
    pub version: u32,

    /// Canonical relationship descriptors.
    pub relations: Vec<CrosstalkRelationDescriptor>,
}

/// Canonical descriptor for one relationship.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CrosstalkRelationDescriptor {
    /// Canonical source identities.
    pub sources: Vec<CrosstalkResourceDescriptor>,

    /// Canonical victim identities.
    pub victims: Vec<CrosstalkResourceDescriptor>,

    /// Direction.
    pub direction: CrosstalkDirection,

    /// Influence law.
    pub law: CrosstalkInfluenceLaw,

    /// Raw IEEE-754 representation of the validated coefficient.
    pub strength_bits: u64,

    /// Semantic guarantee.
    pub guarantee: CrosstalkGuarantee,

    /// Activation condition.
    pub condition: CrosstalkCondition,

    /// Source selector.
    pub source_selector: CrosstalkSelector,

    /// Victim selector.
    pub victim_selector: CrosstalkSelector,
}

/// Canonical resource descriptor.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CrosstalkResourceDescriptor {
    /// Logical qubit identity.
    Logical(String),

    /// Physical qubit identity.
    Physical(String),

    /// External resource identity.
    External(String),
}

impl CrosstalkModel {
    /// Produces deterministic structural identity data.
    ///
    /// Actual persistent hashing is intentionally delegated to the repository's
    /// canonical hashing/serialization subsystem.
    pub fn identity_descriptor(&self) -> CrosstalkIdentityDescriptor {
        let relations = self
            .relations
            .iter()
            .map(|relation| CrosstalkRelationDescriptor {
                sources: relation
                    .sources()
                    .iter()
                    .map(resource_descriptor)
                    .collect(),
                victims: relation
                    .victims()
                    .iter()
                    .map(resource_descriptor)
                    .collect(),
                direction: relation.direction(),
                law: relation.law().clone(),
                strength_bits: relation.strength().value().to_bits(),
                guarantee: relation.guarantee(),
                condition: relation.condition().clone(),
                source_selector: relation.source_selector().clone(),
                victim_selector: relation.victim_selector().clone(),
            })
            .collect();

        CrosstalkIdentityDescriptor {
            kind: CROSSTALK_MODEL_KIND.to_owned(),
            version: CROSSTALK_MODEL_VERSION,
            relations,
        }
    }
}

fn resource_descriptor(
    resource: &CrosstalkResource,
) -> CrosstalkResourceDescriptor {
    match resource {
        CrosstalkResource::LogicalQubit(id) => {
            CrosstalkResourceDescriptor::Logical(format!("{id:?}"))
        }
        CrosstalkResource::PhysicalQubit(id) => {
            CrosstalkResourceDescriptor::Physical(format!("{id:?}"))
        }
        CrosstalkResource::External(value) => {
            CrosstalkResourceDescriptor::External(value.clone())
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn logical(index: usize) -> CrosstalkResource {
        QubitId::new(index).into()
    }

    fn physical(index: usize) -> CrosstalkResource {
        PhysicalQubitId::new(index).into()
    }

    fn relation(
        source: CrosstalkResource,
        victim: CrosstalkResource,
        strength: f64,
    ) -> CrosstalkRelation {
        CrosstalkRelation::new(
            [source],
            [victim],
            CrosstalkDirection::Directed,
            CrosstalkInfluenceLaw::Relative,
            InfluenceStrength::new(strength).expect("valid strength"),
        )
        .expect("valid relation")
    }

    #[test]
    fn uses_canonical_logical_qubit_identity() {
        let resource = logical(7);

        assert!(resource.is_logical_qubit());
    }

    #[test]
    fn uses_canonical_physical_qubit_identity() {
        let resource = physical(7);

        assert!(resource.is_physical_qubit());
    }

    #[test]
    fn rejects_empty_sources() {
        let result = CrosstalkRelation::new(
            std::iter::empty(),
            [logical(1)],
            CrosstalkDirection::Directed,
            CrosstalkInfluenceLaw::Relative,
            InfluenceStrength::new(0.1).expect("valid strength"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_victims() {
        let result = CrosstalkRelation::new(
            [logical(0)],
            std::iter::empty(),
            CrosstalkDirection::Directed,
            CrosstalkInfluenceLaw::Relative,
            InfluenceStrength::new(0.1).expect("valid strength"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_source_victim_overlap() {
        let result = CrosstalkRelation::new(
            [logical(0)],
            [logical(0)],
            CrosstalkDirection::Directed,
            CrosstalkInfluenceLaw::Relative,
            InfluenceStrength::new(0.1).expect("valid strength"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_finite_strength() {
        assert!(InfluenceStrength::new(f64::NAN).is_err());
        assert!(InfluenceStrength::new(f64::INFINITY).is_err());
        assert!(InfluenceStrength::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn rejects_strength_outside_declared_domain() {
        assert!(InfluenceStrength::new(1.000_000_000_001).is_err());
        assert!(InfluenceStrength::new(-1.000_000_000_001).is_err());
    }

    #[test]
    fn accepts_boundary_strengths() {
        assert!(InfluenceStrength::new(-1.0).is_ok());
        assert!(InfluenceStrength::new(0.0).is_ok());
        assert!(InfluenceStrength::new(1.0).is_ok());
    }

    #[test]
    fn duplicate_relationship_is_rejected() {
        let r = relation(logical(0), logical(1), 0.1);

        let result = CrosstalkModel::from_relations([r.clone(), r]);

        assert!(result.is_err());
    }

    #[test]
    fn source_lookup_is_deterministic() {
        let model = CrosstalkModel::from_relations([
            relation(logical(0), logical(1), 0.1),
            relation(logical(0), logical(2), 0.2),
        ])
        .expect("valid model");

        let victims = model.victims_of(&logical(0));

        let expected = BTreeSet::from([logical(1), logical(2)]);

        assert_eq!(victims, expected);
    }

    #[test]
    fn victim_lookup_is_deterministic() {
        let model = CrosstalkModel::from_relations([
            relation(logical(0), logical(2), 0.1),
            relation(logical(1), logical(2), 0.2),
        ])
        .expect("valid model");

        let sources = model.sources_of(&logical(2));

        let expected = BTreeSet::from([logical(0), logical(1)]);

        assert_eq!(sources, expected);
    }

    #[test]
    fn directional_relationship_is_not_reciprocal() {
        let model =
            CrosstalkModel::from_relations([relation(logical(0), logical(1), 0.1)])
                .expect("valid model");

        assert!(model.can_influence(&logical(0), &logical(1)));
        assert!(!model.can_influence(&logical(1), &logical(0)));
    }

    #[test]
    fn evaluation_selects_active_context() {
        let model =
            CrosstalkModel::from_relations([relation(logical(0), logical(1), 0.1)])
                .expect("valid model");

        let context =
            CrosstalkContext::new([logical(0)], [logical(1)])
                .expect("valid context");

        let evaluation = model.evaluate(&context).expect("valid evaluation");

        assert_eq!(evaluation.len(), 1);
        assert!(evaluation.affected_victims().contains(&logical(1)));
    }

    #[test]
    fn evaluation_ignores_inactive_source() {
        let model =
            CrosstalkModel::from_relations([relation(logical(0), logical(1), 0.1)])
                .expect("valid model");

        let context =
            CrosstalkContext::new([logical(2)], [logical(1)])
                .expect("valid context");

        let evaluation = model.evaluate(&context).expect("valid evaluation");

        assert!(evaluation.is_empty());
    }

    #[test]
    fn evaluation_ignores_inactive_victim() {
        let model =
            CrosstalkModel::from_relations([relation(logical(0), logical(1), 0.1)])
                .expect("valid model");

        let context =
            CrosstalkContext::new([logical(0)], [logical(2)])
                .expect("valid context");

        let evaluation = model.evaluate(&context).expect("valid evaluation");

        assert!(evaluation.is_empty());
    }

    #[test]
    fn simultaneous_condition_requires_both_sides() {
        let relation = CrosstalkRelation::new(
            [logical(0)],
            [logical(1)],
            CrosstalkDirection::Directed,
            CrosstalkInfluenceLaw::SimultaneousActivity,
            InfluenceStrength::new(0.1).expect("valid strength"),
        )
        .expect("valid relation")
        .with_condition(CrosstalkCondition::Simultaneous)
        .expect("valid condition");

        let model =
            CrosstalkModel::from_relations([relation]).expect("valid model");

        let active =
            CrosstalkContext::new([logical(0)], [logical(1)])
                .expect("valid context");

        let inactive =
            CrosstalkContext::new([logical(0)], std::iter::empty())
                .expect("valid context");

        assert_eq!(model.evaluate(&active).expect("valid").len(), 1);
        assert_eq!(model.evaluate(&inactive).expect("valid").len(), 0);
    }

    #[test]
    fn context_condition_is_explicit() {
        let relation = relation(logical(0), logical(1), 0.1)
            .with_condition(CrosstalkCondition::Context("cycle-A".to_owned()))
            .expect("valid condition");

        let model =
            CrosstalkModel::from_relations([relation]).expect("valid model");

        let matching =
            CrosstalkContext::new([logical(0)], [logical(1)])
                .expect("valid context")
                .with_id("cycle-A")
                .expect("valid ID");

        let non_matching =
            CrosstalkContext::new([logical(0)], [logical(1)])
                .expect("valid context")
                .with_id("cycle-B")
                .expect("valid ID");

        assert_eq!(model.evaluate(&matching).expect("valid").len(), 1);
        assert_eq!(
            model.evaluate(&non_matching).expect("valid").len(),
            0
        );
    }

    #[test]
    fn custom_conditions_are_not_silently_assumed_active() {
        let relation = relation(logical(0), logical(1), 0.1)
            .with_condition(CrosstalkCondition::Custom("future-rule".to_owned()))
            .expect("valid condition");

        let model =
            CrosstalkModel::from_relations([relation]).expect("valid model");

        let context =
            CrosstalkContext::new([logical(0)], [logical(1)])
                .expect("valid context");

        assert!(model.evaluate(&context).expect("valid").is_empty());
    }

    #[test]
    fn additive_aggregation_is_explicit() {
        let model = CrosstalkModel::from_relations([
            relation(logical(0), logical(2), 0.1),
            relation(logical(1), logical(2), 0.2),
        ])
        .expect("valid model");

        let context =
            CrosstalkContext::new([logical(0), logical(1)], [logical(2)])
                .expect("valid context");

        let evaluation = model.evaluate(&context).expect("valid evaluation");

        let total =
            aggregate_additive_influence(&evaluation, &logical(2))
                .expect("valid aggregation");

        assert!((total - 0.3).abs() < 1.0e-12);
    }

    #[test]
    fn logical_and_physical_same_numeric_index_are_distinct() {
        let logical_resource = logical(7);
        let physical_resource = physical(7);

        assert_ne!(logical_resource, physical_resource);
    }

    #[test]
    fn model_identity_is_deterministic() {
        let model_a =
            CrosstalkModel::from_relations([relation(logical(0), logical(1), 0.1)])
                .expect("valid model");

        let model_b =
            CrosstalkModel::from_relations([relation(logical(0), logical(1), 0.1)])
                .expect("valid model");

        assert_eq!(
            model_a.identity_descriptor(),
            model_b.identity_descriptor()
        );
    }

    #[test]
    fn relation_order_does_not_change_canonical_model_order() {
        let first =
            relation(logical(0), logical(1), 0.1);
        let second =
            relation(logical(1), logical(2), 0.2);

        let model_a =
            CrosstalkModel::from_relations([first.clone(), second.clone()])
                .expect("valid model");

        let model_b =
            CrosstalkModel::from_relations([second, first])
                .expect("valid model");

        assert_eq!(model_a, model_b);
    }

    #[test]
    fn external_resource_requires_non_empty_identity() {
        assert!(CrosstalkResource::external("").is_err());
        assert!(CrosstalkResource::external("mode-0").is_ok());
    }

    #[test]
    fn parameter_requires_finite_value() {
        assert!(
            CrosstalkParameter::new("coupling", f64::NAN, None::<String>)
                .is_err()
        );

        assert!(
            CrosstalkParameter::new("coupling", 0.5, Some("dimensionless"))
                .is_ok()
        );
    }

    #[test]
    fn custom_selector_requires_name() {
        let relation = relation(logical(0), logical(1), 0.1);

        assert!(
            relation
                .with_source_selector(CrosstalkSelector::Custom(
                    String::new()
                ))
                .is_err()
        );
    }
}