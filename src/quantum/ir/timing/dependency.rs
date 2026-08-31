//! Zamani Quantum IR — Temporal Dependencies
//!
//! Path:
//!     src/quantum/ir/timing/dependency.rs
//!
//! # Purpose
//!
//! This module defines target-independent temporal dependency semantics for
//! the canonical Zamani Quantum IR.
//!
//! A temporal dependency states that one semantic event/resource/value is
//! related to another event/resource/value in time.
//!
//! This module describes:
//!
//! - operation ordering;
//! - data dependencies;
//! - classical dependencies;
//! - measurement dependencies;
//! - pulse dependencies;
//! - resource dependencies;
//! - control-flow dependencies;
//! - synchronization dependencies;
//! - explicit temporal barriers;
//! - dependency strengths;
//! - dependency provenance;
//! - dependency collections;
//! - deterministic dependency ordering;
//! - dependency validation.
//!
//! It does NOT perform scheduling.
//!
//! # Architectural boundary
//!
//! ```text
//! canonical IR
//!      |
//!      v
//! temporal dependencies
//!      |
//!      +----> scheduling
//!      +----> analysis
//!      +----> validation
//!      +----> optimization
//!      +----> hardware lowering
//! ```
//!
//! The dependency graph describes semantic relationships.
//!
//! A scheduler later decides concrete placement in time.
//!
//! # Universal-program principle
//!
//! A dependency must not assume:
//!
//! - a particular number of qubits;
//! - a particular number of operations;
//! - a particular topology;
//! - a particular quantum technology;
//! - a particular backend;
//! - a particular clock;
//! - a particular scheduler;
//! - a particular pulse representation.
//!
//! The same dependency representation must work for:
//!
//! - one qubit;
//! - millions or more logical resources;
//! - dynamic circuits;
//! - pulse programs;
//! - analog programs;
//! - Hamiltonian evolution;
//! - annealing;
//! - logical/fault-tolerant programs;
//! - distributed quantum programs;
//! - hybrid quantum/classical programs;
//! - future quantum architectures.
//!
//! There is deliberately no fixed dependency-count or machine-size constant.
//!
//! # Dependency semantics
//!
//! A dependency is represented as:
//!
//! ```text
//! source event/resource/value
//!          |
//!          | relation
//!          v
//! destination event/resource/value
//! ```
//!
//! Examples:
//!
//! ```text
//! measurement -> classical feedback
//!
//! operation A -> operation B
//!
//! pulse A -> pulse B
//!
//! operation A -> resource use
//!
//! value producer -> value consumer
//!
//! control predicate -> conditional operation
//! ```
//!
//! # Important distinction
//!
//! A dependency is NOT:
//!
//! - a timestamp;
//! - a duration;
//! - a schedule;
//! - a routing decision;
//! - a hardware dependency;
//! - a physical-qubit allocation;
//! - an optimization pass.
//!
//! Those concerns belong to their respective IR/downstream modules.
//!
//! # Qubit boundary
//!
//! This module intentionally does not import `quantum::ir::qubit` directly.
//!
//! A dependency may concern an operation whose operands are qubits, but the
//! dependency itself is between semantic IR entities. The operation layer owns
//! qubit operands.
//!
//! This preserves the architectural boundary:
//!
//! ```text
//! quantum::ir::qubit
//!     owns qubit identity
//!
//! timing::dependency
//!     owns temporal relationships
//!
//! scheduling
//!     owns concrete placement
//! ```
//!
//! # Identity boundary
//!
//! Dependencies use stable IR identities rather than `usize` indexes.
//!
//! This is important because container indexes are implementation details and
//! are not suitable as persistent semantic identities.
//!
//! # Determinism
//!
//! Dependency collections provide deterministic ordering.
//!
//! No dependency API relies on hash-map iteration order.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! Requirements:
//!
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`;
//! - no external dependency.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! `timing.rs`
//!     Owns canonical timing values and timing primitives.
//!
//! `quantum::ir::identity`
//!     Owns stable IR identities.
//!
//! `program::operation`
//!     Owns semantic operations that dependencies may reference.
//!
//! `program::operand`
//!     Owns operation operands.
//!
//! `classical::*`
//!     Owns classical values and expressions.
//!
//! `pulse::*`
//!     Owns pulse-level semantics.
//!
//! `scheduling::*`
//!     Consumes these dependencies to compute concrete schedules.
//!
//! `analysis::*`
//!     Reads these dependencies for graph/dependency analysis.
//!
//! `validation::*`
//!     Validates dependency correctness.
//!
//! `optimization::*`
//!     May preserve, remove, introduce, or transform dependencies while
//!     maintaining semantic correctness.
//!
//! `hardware::*`
//!     May consume dependencies after target lowering, but this module never
//!     depends on hardware.
//!
//! # File contract
//!
//! ## Owns
//!
//! - dependency source/target identity;
//! - dependency relation;
//! - dependency strength;
//! - dependency provenance flags;
//! - dependency records;
//! - deterministic dependency sets;
//! - local dependency validation.
//!
//! ## Does not own
//!
//! - operation definitions;
//! - qubit definitions;
//! - hardware resources;
//! - topology;
//! - routing;
//! - scheduling;
//! - pulse generation;
//! - calibration;
//! - execution.
//!
//! ## Scalability contract
//!
//! All collections grow according to available memory/resources.
//!
//! No semantic upper bound is encoded in this module.
//!
//! `u64` identity values are identifiers, not resource counts.
//!
//! `usize` is used only where the Rust collection API requires a collection
//! index or caller-requested capacity.
//!
//! ## Serialization contract
//!
//! This module intentionally keeps semantic fields explicit and deterministic.
//! A future canonical serializer must serialize:
//!
//! - source;
//! - target;
//! - relation;
//! - strength;
//! - metadata flags;
//!
//! in a stable field order.
//!
//! The serializer must not serialize internal collection capacity or memory
//! addresses.
//!
//! ## Hashing contract
//!
//! Dependency hashing must be based on semantic fields only.
//!
//! Collection ordering is canonicalized before hashing.
//!
//! ## Thread-safety contract
//!
//! All types are composed of owned values and stable IDs and therefore do not
//! contain interior mutable global state.
//!
//! ## Error contract
//!
//! Invalid self-dependencies and invalid dependency combinations are rejected
//! explicitly.
//!
//! No invalid dependency is silently ignored.
//!
//! # Example
//!
//! ```text
//! measurement(q0)
//!       |
//!       v
//! classical value c0
//!       |
//!       v
//! if c0 == 1
//!       |
//!       v
//! X(q1)
//! ```
//!
//! The dependency graph can represent:
//!
//! ```text
//! measurement operation
//!     --MeasurementFeedback-->
//! conditional operation
//! ```
//!
//! The scheduler later converts this semantic relationship into a concrete
//! execution constraint.
//!
//! ```text
//! measurement
//!     -> latency
//!     -> classical evaluation
//!     -> conditional operation
//! ```
//!
//! The latency itself is not invented by this module.

#![forbid(unsafe_code)]

use std::cmp::Ordering;
use std::fmt;

use super::{Duration, TimingError, TimingResult};

use crate::quantum::ir::identity::{
    ChannelId,
    FrameId,
    OperationId,
    PulseId,
    ResourceId,
    ValueId,
    WaveformId,
};

// =============================================================================
// Dependency relation
// =============================================================================

/// Semantic relationship between two dependency endpoints.
///
/// The variants describe *why* the relationship exists, not how a scheduler
/// should implement it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DependencyKind {
    /// The source operation must complete before the target operation may
    /// begin.
    ///
    /// This is the canonical happens-before relationship.
    HappensBefore,

    /// The source and target must preserve program order.
    ProgramOrder,

    /// The target consumes a value produced by the source.
    Data,

    /// The target depends on a classical value produced by the source.
    Classical,

    /// The target depends on a measurement result produced by the source.
    Measurement,

    /// The target is conditionally enabled by the source's result.
    ClassicalFeedback,

    /// A pulse-level action depends on another pulse-level action.
    Pulse,

    /// The source and target share a resource constraint.
    Resource,

    /// The target depends on a control-flow predecessor.
    ControlFlow,

    /// Two entities must participate in a synchronization relationship.
    Synchronization,

    /// A semantic barrier prevents reordering across the dependency.
    Barrier,

    /// The target depends on an explicit user-declared ordering.
    Explicit,

    /// An extension-defined dependency.
    ///
    /// The namespace is represented separately by [`DependencyExtension`].
    Extension,
}

impl DependencyKind {
    /// Returns whether this relation normally represents execution ordering.
    #[must_use]
    pub const fn is_ordering(self) -> bool {
        matches!(
            self,
            Self::HappensBefore
                | Self::ProgramOrder
                | Self::ControlFlow
                | Self::Barrier
                | Self::Explicit
                | Self::Synchronization
        )
    }

    /// Returns whether this relation is value-producing/consuming.
    #[must_use]
    pub const fn is_data_flow(self) -> bool {
        matches!(
            self,
            Self::Data
                | Self::Classical
                | Self::Measurement
                | Self::ClassicalFeedback
        )
    }

    /// Returns whether this relation is pulse-oriented.
    #[must_use]
    pub const fn is_pulse_related(self) -> bool {
        matches!(self, Self::Pulse)
    }

    /// Returns whether this relation is resource-oriented.
    #[must_use]
    pub const fn is_resource_related(self) -> bool {
        matches!(self, Self::Resource)
    }

    /// Returns a stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HappensBefore => "happens_before",
            Self::ProgramOrder => "program_order",
            Self::Data => "data",
            Self::Classical => "classical",
            Self::Measurement => "measurement",
            Self::ClassicalFeedback => "classical_feedback",
            Self::Pulse => "pulse",
            Self::Resource => "resource",
            Self::ControlFlow => "control_flow",
            Self::Synchronization => "synchronization",
            Self::Barrier => "barrier",
            Self::Explicit => "explicit",
            Self::Extension => "extension",
        }
    }
}

impl fmt::Display for DependencyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Dependency strength
// =============================================================================

/// Semantic strength of a dependency.
///
/// Strength does not tell the scheduler how to implement the dependency. It
/// tells downstream consumers how authoritative the relationship is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DependencyStrength {
    /// Mandatory semantic dependency.
    Required,

    /// Dependency is semantically required when a condition is active.
    Conditional,

    /// Dependency expresses a preferred ordering but is not itself a semantic
    /// correctness requirement.
    Preferred,

    /// Dependency exists for diagnostics/provenance and must not be treated as
    /// a mandatory execution edge.
    Informational,
}

impl DependencyStrength {
    /// Returns whether the dependency is mandatory.
    #[must_use]
    pub const fn is_required(self) -> bool {
        matches!(
            self,
            Self::Required | Self::Conditional
        )
    }

    /// Returns a stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Conditional => "conditional",
            Self::Preferred => "preferred",
            Self::Informational => "informational",
        }
    }
}

impl Default for DependencyStrength {
    fn default() -> Self {
        Self::Required
    }
}

impl fmt::Display for DependencyStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Dependency endpoint
// =============================================================================

/// Semantic object that can participate in a temporal dependency.
///
/// This is deliberately broader than an operation so that the IR can express
/// dependencies involving values, pulses, frames, channels and abstract
/// resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DependencyEndpoint {
    /// An operation.
    Operation(OperationId),

    /// An SSA-like/value-producing entity.
    Value(ValueId),

    /// An abstract resource.
    Resource(ResourceId),

    /// A pulse operation.
    Pulse(PulseId),

    /// A control/acquisition channel.
    Channel(ChannelId),

    /// A control frame.
    Frame(FrameId),

    /// A waveform definition.
    Waveform(WaveformId),
}

impl DependencyEndpoint {
    /// Returns the endpoint category.
    #[must_use]
    pub const fn kind(self) -> DependencyEndpointKind {
        match self {
            Self::Operation(_) => DependencyEndpointKind::Operation,
            Self::Value(_) => DependencyEndpointKind::Value,
            Self::Resource(_) => DependencyEndpointKind::Resource,
            Self::Pulse(_) => DependencyEndpointKind::Pulse,
            Self::Channel(_) => DependencyEndpointKind::Channel,
            Self::Frame(_) => DependencyEndpointKind::Frame,
            Self::Waveform(_) => DependencyEndpointKind::Waveform,
        }
    }

    /// Returns the underlying stable numeric identity.
    ///
    /// This method is intended for deterministic diagnostics and canonical
    /// sorting only. The resulting number must not be interpreted as a
    /// collection index.
    #[must_use]
    pub const fn identity_value(self) -> u64 {
        match self {
            Self::Operation(id) => id.value(),
            Self::Value(id) => id.value(),
            Self::Resource(id) => id.value(),
            Self::Pulse(id) => id.value(),
            Self::Channel(id) => id.value(),
            Self::Frame(id) => id.value(),
            Self::Waveform(id) => id.value(),
        }
    }
}

impl fmt::Display for DependencyEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Operation(id) => write!(f, "{id}"),
            Self::Value(id) => write!(f, "{id}"),
            Self::Resource(id) => write!(f, "{id}"),
            Self::Pulse(id) => write!(f, "{id}"),
            Self::Channel(id) => write!(f, "{id}"),
            Self::Frame(id) => write!(f, "{id}"),
            Self::Waveform(id) => write!(f, "{id}"),
        }
    }
}

/// Category of a dependency endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DependencyEndpointKind {
    /// Operation endpoint.
    Operation,

    /// Value endpoint.
    Value,

    /// Resource endpoint.
    Resource,

    /// Pulse endpoint.
    Pulse,

    /// Channel endpoint.
    Channel,

    /// Frame endpoint.
    Frame,

    /// Waveform endpoint.
    Waveform,
}

impl DependencyEndpointKind {
    /// Returns a stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Operation => "operation",
            Self::Value => "value",
            Self::Resource => "resource",
            Self::Pulse => "pulse",
            Self::Channel => "channel",
            Self::Frame => "frame",
            Self::Waveform => "waveform",
        }
    }
}

impl fmt::Display for DependencyEndpointKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Dependency extension
// =============================================================================

/// Namespaced extension information for an extension-defined dependency.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DependencyExtension {
    /// Extension namespace.
    namespace: String,

    /// Extension-specific dependency name.
    name: String,
}

impl DependencyExtension {
    /// Creates an extension identifier.
    ///
    /// Empty namespace/name values are rejected because extension identities
    /// must remain unambiguous.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> TimingResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.trim().is_empty() || name.trim().is_empty() {
            return Err(TimingError::InvalidValue {
                message: "dependency extension namespace and name must not be empty"
                    .to_owned(),
            });
        }

        Ok(Self { namespace, name })
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the extension name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for DependencyExtension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.namespace, self.name)
    }
}

// =============================================================================
// Dependency edge
// =============================================================================

/// A single semantic temporal dependency.
///
/// A dependency is directed:
///
/// ```text
/// source -> target
/// ```
///
/// The meaning of the edge is determined by [`DependencyKind`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemporalDependency {
    source: DependencyEndpoint,
    target: DependencyEndpoint,
    kind: DependencyKind,
    strength: DependencyStrength,

    /// Optional minimum temporal separation.
    ///
    /// This is semantic intent. A scheduler later determines actual timestamps.
    minimum_separation: Option<Duration>,

    /// Optional maximum temporal separation.
    ///
    /// This is useful for latency-sensitive relationships such as measurement
    /// feedback.
    maximum_separation: Option<Duration>,

    /// Optional extension-defined relation.
    extension: Option<DependencyExtension>,
}

impl TemporalDependency {
    /// Creates a required dependency without explicit temporal bounds.
    pub fn new(
        source: DependencyEndpoint,
        target: DependencyEndpoint,
        kind: DependencyKind,
    ) -> TimingResult<Self> {
        Self::with_strength(
            source,
            target,
            kind,
            DependencyStrength::Required,
        )
    }

    /// Creates a dependency with an explicit strength.
    pub fn with_strength(
        source: DependencyEndpoint,
        target: DependencyEndpoint,
        kind: DependencyKind,
        strength: DependencyStrength,
    ) -> TimingResult<Self> {
        if source == target {
            return Err(TimingError::InvalidConstraint {
                message: "a temporal dependency cannot reference the same endpoint as both source and target"
                    .to_owned(),
            });
        }

        if kind == DependencyKind::Extension {
            return Err(TimingError::InvalidConstraint {
                message: "extension dependencies must be created with `with_extension`"
                    .to_owned(),
            });
        }

        Ok(Self {
            source,
            target,
            kind,
            strength,
            minimum_separation: None,
            maximum_separation: None,
            extension: None,
        })
    }

    /// Creates an extension-defined dependency.
    pub fn with_extension(
        source: DependencyEndpoint,
        target: DependencyEndpoint,
        extension: DependencyExtension,
        strength: DependencyStrength,
    ) -> TimingResult<Self> {
        if source == target {
            return Err(TimingError::InvalidConstraint {
                message: "an extension dependency cannot reference the same endpoint as both source and target"
                    .to_owned(),
            });
        }

        Ok(Self {
            source,
            target,
            kind: DependencyKind::Extension,
            strength,
            minimum_separation: None,
            maximum_separation: None,
            extension: Some(extension),
        })
    }

    /// Returns the source endpoint.
    #[must_use]
    pub const fn source(&self) -> DependencyEndpoint {
        self.source
    }

    /// Returns the target endpoint.
    #[must_use]
    pub const fn target(&self) -> DependencyEndpoint {
        self.target
    }

    /// Returns the dependency kind.
    #[must_use]
    pub const fn kind(&self) -> DependencyKind {
        self.kind
    }

    /// Returns the dependency strength.
    #[must_use]
    pub const fn strength(&self) -> DependencyStrength {
        self.strength
    }

    /// Returns the optional minimum separation.
    #[must_use]
    pub const fn minimum_separation(&self) -> Option<Duration> {
        self.minimum_separation
    }

    /// Returns the optional maximum separation.
    #[must_use]
    pub const fn maximum_separation(&self) -> Option<Duration> {
        self.maximum_separation
    }

    /// Returns the extension descriptor, if present.
    #[must_use]
    pub fn extension(&self) -> Option<&DependencyExtension> {
        self.extension.as_ref()
    }

    /// Sets a minimum temporal separation.
    pub fn with_minimum_separation(
        mut self,
        duration: Duration,
    ) -> TimingResult<Self> {
        if let Some(maximum) = self.maximum_separation {
            if duration > maximum {
                return Err(TimingError::InvalidConstraint {
                    message:
                        "minimum separation cannot exceed maximum separation"
                            .to_owned(),
                });
            }
        }

        self.minimum_separation = Some(duration);
        Ok(self)
    }

    /// Sets a maximum temporal separation.
    pub fn with_maximum_separation(
        mut self,
        duration: Duration,
    ) -> TimingResult<Self> {
        if let Some(minimum) = self.minimum_separation {
            if minimum > duration {
                return Err(TimingError::InvalidConstraint {
                    message:
                        "maximum separation cannot be smaller than minimum separation"
                            .to_owned(),
                });
            }
        }

        self.maximum_separation = Some(duration);
        Ok(self)
    }

    /// Sets both temporal bounds atomically.
    pub fn with_separation(
        mut self,
        minimum: Option<Duration>,
        maximum: Option<Duration>,
    ) -> TimingResult<Self> {
        if let (Some(minimum), Some(maximum)) = (minimum, maximum) {
            if minimum > maximum {
                return Err(TimingError::InvalidConstraint {
                    message:
                        "minimum separation cannot exceed maximum separation"
                            .to_owned(),
                });
            }
        }

        self.minimum_separation = minimum;
        self.maximum_separation = maximum;

        Ok(self)
    }

    /// Validates the dependency.
    pub fn validate(&self) -> TimingResult<()> {
        if self.source == self.target {
            return Err(TimingError::InvalidConstraint {
                message:
                    "dependency source and target must be different"
                        .to_owned(),
            });
        }

        if self.kind == DependencyKind::Extension
            && self.extension.is_none()
        {
            return Err(TimingError::InvalidConstraint {
                message:
                    "extension dependency requires extension metadata"
                        .to_owned(),
            });
        }

        if self.kind != DependencyKind::Extension
            && self.extension.is_some()
        {
            return Err(TimingError::InvalidConstraint {
                message:
                    "non-extension dependency cannot contain extension metadata"
                        .to_owned(),
            });
        }

        if let (Some(minimum), Some(maximum)) =
            (self.minimum_separation, self.maximum_separation)
        {
            if minimum > maximum {
                return Err(TimingError::InvalidConstraint {
                    message:
                        "minimum separation cannot exceed maximum separation"
                            .to_owned(),
                });
            }
        }

        if matches!(
            self.kind,
            DependencyKind::Data
                | DependencyKind::Classical
                | DependencyKind::Measurement
                | DependencyKind::ClassicalFeedback
        ) && !matches!(
            self.source,
            DependencyEndpoint::Operation(_)
                | DependencyEndpoint::Value(_)
        ) {
            return Err(TimingError::InvalidConstraint {
                message:
                    "data-flow dependency source must be an operation or value"
                        .to_owned(),
            });
        }

        if matches!(
            self.kind,
            DependencyKind::Measurement
                | DependencyKind::ClassicalFeedback
        ) && !matches!(
            self.target,
            DependencyEndpoint::Operation(_)
                | DependencyEndpoint::Value(_)
        ) {
            return Err(TimingError::InvalidConstraint {
                message:
                    "measurement/feedback dependency target must be an operation or value"
                        .to_owned(),
            });
        }

        Ok(())
    }

    /// Returns a deterministic ordering key.
    ///
    /// This is intended for canonical serialization, hashing, testing, and
    /// deterministic graph processing.
    #[must_use]
    pub fn canonical_key(
        &self,
    ) -> DependencyCanonicalKey {
        DependencyCanonicalKey {
            source_kind: self.source.kind(),
            source_id: self.source.identity_value(),
            target_kind: self.target.kind(),
            target_id: self.target.identity_value(),
            kind: self.kind,
            strength: self.strength,
            minimum_attoseconds: self
                .minimum_separation
                .map(Duration::attoseconds),
            maximum_attoseconds: self
                .maximum_separation
                .map(Duration::attoseconds),
            extension: self.extension.clone(),
        }
    }
}

impl Ord for TemporalDependency {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        self.canonical_key().cmp(&other.canonical_key())
    }
}

impl PartialOrd for TemporalDependency {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for TemporalDependency {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{} -{}-> {}",
            self.source,
            self.kind,
            self.target
        )?;

        if let Some(minimum) = self.minimum_separation {
            write!(f, " min={minimum}")?;
        }

        if let Some(maximum) = self.maximum_separation {
            write!(f, " max={maximum}")?;
        }

        Ok(())
    }
}

// =============================================================================
// Canonical key
// =============================================================================

/// Stable ordering key for a dependency.
///
/// This type intentionally contains only semantic fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DependencyCanonicalKey {
    /// Source endpoint category.
    pub source_kind: DependencyEndpointKind,

    /// Source identity value.
    pub source_id: u64,

    /// Target endpoint category.
    pub target_kind: DependencyEndpointKind,

    /// Target identity value.
    pub target_id: u64,

    /// Dependency kind.
    pub kind: DependencyKind,

    /// Dependency strength.
    pub strength: DependencyStrength,

    /// Minimum separation in canonical attoseconds.
    pub minimum_attoseconds: Option<u128>,

    /// Maximum separation in canonical attoseconds.
    pub maximum_attoseconds: Option<u128>,

    /// Extension descriptor.
    pub extension: Option<DependencyExtension>,
}

// =============================================================================
// Dependency graph
// =============================================================================

/// Deterministic collection of temporal dependencies.
///
/// This is deliberately a simple semantic collection rather than a graph
/// algorithm implementation.
///
/// Scheduling, cycle detection, transitive reduction and critical-path
/// computation belong to downstream consumers.
///
/// The collection maintains deterministic semantic ordering and rejects exact
/// duplicate edges.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DependencyGraph {
    dependencies: Vec<TemporalDependency>,
}

impl DependencyGraph {
    /// Creates an empty dependency graph.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            dependencies: Vec::new(),
        }
    }

    /// Creates an empty graph with caller-requested storage capacity.
    ///
    /// Capacity is an implementation/storage hint only. It is never a
    /// semantic limit.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            dependencies: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of dependencies.
    #[must_use]
    pub fn len(&self) -> usize {
        self.dependencies.len()
    }

    /// Returns whether the graph contains no dependencies.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dependencies.is_empty()
    }

    /// Returns all dependencies in deterministic canonical order.
    #[must_use]
    pub fn dependencies(&self) -> &[TemporalDependency] {
        &self.dependencies
    }

    /// Adds a dependency.
    ///
    /// Exact duplicate semantic edges are rejected instead of silently
    /// duplicated.
    pub fn add(
        &mut self,
        dependency: TemporalDependency,
    ) -> TimingResult<()> {
        dependency.validate()?;

        if self.dependencies.binary_search(&dependency).is_ok() {
            return Err(TimingError::InvalidConstraint {
                message:
                    "duplicate temporal dependency"
                        .to_owned(),
            });
        }

        let position = self
            .dependencies
            .binary_search(&dependency)
            .unwrap_or_else(|position| position);

        self.dependencies.insert(position, dependency);

        Ok(())
    }

    /// Adds a dependency and returns whether the graph changed.
    ///
    /// This method is useful for graph-building algorithms where duplicates
    /// are naturally encountered.
    pub fn insert(
        &mut self,
        dependency: TemporalDependency,
    ) -> TimingResult<bool> {
        dependency.validate()?;

        match self.dependencies.binary_search(&dependency) {
            Ok(_) => Ok(false),
            Err(position) => {
                self.dependencies.insert(position, dependency);
                Ok(true)
            }
        }
    }

    /// Removes an exact dependency.
    pub fn remove(
        &mut self,
        dependency: &TemporalDependency,
    ) -> bool {
        match self.dependencies.binary_search(dependency) {
            Ok(position) => {
                self.dependencies.remove(position);
                true
            }
            Err(_) => false,
        }
    }

    /// Returns whether an exact dependency exists.
    #[must_use]
    pub fn contains(
        &self,
        dependency: &TemporalDependency,
    ) -> bool {
        self.dependencies.binary_search(dependency).is_ok()
    }

    /// Removes all dependencies.
    pub fn clear(&mut self) {
        self.dependencies.clear();
    }

    /// Returns an iterator over dependencies originating from an endpoint.
    pub fn outgoing(
        &self,
        source: DependencyEndpoint,
    ) -> impl Iterator<Item = &TemporalDependency> {
        self.dependencies
            .iter()
            .filter(move |dependency| dependency.source() == source)
    }

    /// Returns an iterator over dependencies targeting an endpoint.
    pub fn incoming(
        &self,
        target: DependencyEndpoint,
    ) -> impl Iterator<Item = &TemporalDependency> {
        self.dependencies
            .iter()
            .filter(move |dependency| dependency.target() == target)
    }

    /// Returns whether the graph contains at least one dependency involving
    /// an endpoint.
    #[must_use]
    pub fn contains_endpoint(
        &self,
        endpoint: DependencyEndpoint,
    ) -> bool {
        self.dependencies.iter().any(|dependency| {
            dependency.source() == endpoint
                || dependency.target() == endpoint
        })
    }

    /// Validates every dependency.
    pub fn validate(&self) -> TimingResult<()> {
        for dependency in &self.dependencies {
            dependency.validate()?;
        }

        for pair in self.dependencies.windows(2) {
            if pair[0] >= pair[1] {
                return Err(TimingError::InvalidConstraint {
                    message:
                        "dependency graph is not in strict canonical order"
                            .to_owned(),
                });
            }
        }

        Ok(())
    }

    /// Returns a deterministic iterator over the graph.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &TemporalDependency> {
        self.dependencies.iter()
    }
}

impl<'a> IntoIterator for &'a DependencyGraph {
    type Item = &'a TemporalDependency;
    type IntoIter =
        std::slice::Iter<'a, TemporalDependency>;

    fn into_iter(self) -> Self::IntoIter {
        self.dependencies.iter()
    }
}

// =============================================================================
// Convenience constructors
// =============================================================================

/// Creates a required happens-before dependency.
pub fn happens_before(
    source: DependencyEndpoint,
    target: DependencyEndpoint,
) -> TimingResult<TemporalDependency> {
    TemporalDependency::new(
        source,
        target,
        DependencyKind::HappensBefore,
    )
}

/// Creates a program-order dependency.
pub fn program_order(
    source: DependencyEndpoint,
    target: DependencyEndpoint,
) -> TimingResult<TemporalDependency> {
    TemporalDependency::new(
        source,
        target,
        DependencyKind::ProgramOrder,
    )
}

/// Creates a data dependency.
pub fn data_dependency(
    source: DependencyEndpoint,
    target: DependencyEndpoint,
) -> TimingResult<TemporalDependency> {
    TemporalDependency::new(
        source,
        target,
        DependencyKind::Data,
    )
}

/// Creates a classical dependency.
pub fn classical_dependency(
    source: DependencyEndpoint,
    target: DependencyEndpoint,
) -> TimingResult<TemporalDependency> {
    TemporalDependency::new(
        source,
        target,
        DependencyKind::Classical,
    )
}

/// Creates a measurement dependency.
pub fn measurement_dependency(
    source: DependencyEndpoint,
    target: DependencyEndpoint,
) -> TimingResult<TemporalDependency> {
    TemporalDependency::new(
        source,
        target,
        DependencyKind::Measurement,
    )
}

/// Creates a measurement-feedback dependency.
pub fn classical_feedback(
    source: DependencyEndpoint,
    target: DependencyEndpoint,
) -> TimingResult<TemporalDependency> {
    TemporalDependency::new(
        source,
        target,
        DependencyKind::ClassicalFeedback,
    )
}

/// Creates a pulse dependency.
pub fn pulse_dependency(
    source: DependencyEndpoint,
    target: DependencyEndpoint,
) -> TimingResult<TemporalDependency> {
    TemporalDependency::new(
        source,
        target,
        DependencyKind::Pulse,
    )
}

/// Creates a resource dependency.
pub fn resource_dependency(
    source: DependencyEndpoint,
    target: DependencyEndpoint,
) -> TimingResult<TemporalDependency> {
    TemporalDependency::new(
        source,
        target,
        DependencyKind::Resource,
    )
}

/// Creates a control-flow dependency.
pub fn control_flow_dependency(
    source: DependencyEndpoint,
    target: DependencyEndpoint,
) -> TimingResult<TemporalDependency> {
    TemporalDependency::new(
        source,
        target,
        DependencyKind::ControlFlow,
    )
}

/// Creates a synchronization dependency.
pub fn synchronization_dependency(
    source: DependencyEndpoint,
    target: DependencyEndpoint,
) -> TimingResult<TemporalDependency> {
    TemporalDependency::new(
        source,
        target,
        DependencyKind::Synchronization,
    )
}

/// Creates a semantic barrier dependency.
pub fn barrier_dependency(
    source: DependencyEndpoint,
    target: DependencyEndpoint,
) -> TimingResult<TemporalDependency> {
    TemporalDependency::new(
        source,
        target,
        DependencyKind::Barrier,
    )
}

/// Creates an explicit user-declared dependency.
pub fn explicit_dependency(
    source: DependencyEndpoint,
    target: DependencyEndpoint,
) -> TimingResult<TemporalDependency> {
    TemporalDependency::new(
        source,
        target,
        DependencyKind::Explicit,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(value: u64) -> DependencyEndpoint {
        DependencyEndpoint::Operation(
            OperationId::new(value),
        )
    }

    fn value(value: u64) -> DependencyEndpoint {
        DependencyEndpoint::Value(
            ValueId::new(value),
        )
    }

    fn resource(value: u64) -> DependencyEndpoint {
        DependencyEndpoint::Resource(
            ResourceId::new(value),
        )
    }

    fn pulse(value: u64) -> DependencyEndpoint {
        DependencyEndpoint::Pulse(
            PulseId::new(value),
        )
    }

    #[test]
    fn operation_dependency_is_constructed() {
        let dependency =
            happens_before(operation(1), operation(2))
                .expect("valid dependency");

        assert_eq!(
            dependency.source(),
            operation(1)
        );
        assert_eq!(
            dependency.target(),
            operation(2)
        );
        assert_eq!(
            dependency.kind(),
            DependencyKind::HappensBefore
        );
        assert_eq!(
            dependency.strength(),
            DependencyStrength::Required
        );
    }

    #[test]
    fn self_dependency_is_rejected() {
        let result =
            happens_before(operation(1), operation(1));

        assert!(result.is_err());
    }

    #[test]
    fn minimum_and_maximum_separation_are_checked() {
        let minimum =
            Duration::nanoseconds(10)
                .expect("valid duration");

        let maximum =
            Duration::nanoseconds(20)
                .expect("valid duration");

        let dependency =
            happens_before(operation(1), operation(2))
                .expect("valid dependency")
                .with_separation(
                    Some(minimum),
                    Some(maximum),
                )
                .expect("valid separation");

        assert_eq!(
            dependency.minimum_separation(),
            Some(minimum)
        );
        assert_eq!(
            dependency.maximum_separation(),
            Some(maximum)
        );
    }

    #[test]
    fn invalid_separation_is_rejected() {
        let minimum =
            Duration::nanoseconds(20)
                .expect("valid duration");

        let maximum =
            Duration::nanoseconds(10)
                .expect("valid duration");

        let result =
            happens_before(operation(1), operation(2))
                .expect("valid dependency")
                .with_separation(
                    Some(minimum),
                    Some(maximum),
                );

        assert!(result.is_err());
    }

    #[test]
    fn data_dependency_accepts_value_source() {
        let dependency =
            data_dependency(value(1), operation(2))
                .expect("valid dependency");

        assert_eq!(
            dependency.kind(),
            DependencyKind::Data
        );
    }

    #[test]
    fn measurement_dependency_rejects_resource_source() {
        let result =
            measurement_dependency(resource(1), operation(2));

        assert!(result.is_ok());

        let dependency =
            result.expect("constructor accepts initial shape");

        assert!(dependency.validate().is_err());
    }

    #[test]
    fn graph_is_deterministic() {
        let mut graph =
            DependencyGraph::new();

        graph
            .add(
                happens_before(
                    operation(2),
                    operation(3),
                )
                .expect("valid"),
            )
            .expect("insert");

        graph
            .add(
                happens_before(
                    operation(1),
                    operation(2),
                )
                .expect("valid"),
            )
            .expect("insert");

        let dependencies =
            graph.dependencies();

        assert_eq!(
            dependencies[0].source(),
            operation(1)
        );
        assert_eq!(
            dependencies[1].source(),
            operation(2)
        );
    }

    #[test]
    fn duplicate_insert_is_idempotent() {
        let mut graph =
            DependencyGraph::new();

        let dependency =
            happens_before(
                operation(1),
                operation(2),
            )
            .expect("valid");

        assert!(
            graph
                .insert(dependency.clone())
                .expect("insert")
        );

        assert!(
            !graph
                .insert(dependency)
                .expect("duplicate")
        );

        assert_eq!(graph.len(), 1);
    }

    #[test]
    fn duplicate_add_is_rejected() {
        let mut graph =
            DependencyGraph::new();

        let dependency =
            happens_before(
                operation(1),
                operation(2),
            )
            .expect("valid");

        graph
            .add(dependency.clone())
            .expect("first insert");

        assert!(
            graph.add(dependency).is_err()
        );
    }

    #[test]
    fn outgoing_and_incoming_queries_work() {
        let mut graph =
            DependencyGraph::new();

        graph
            .add(
                happens_before(
                    operation(1),
                    operation(2),
                )
                .expect("valid"),
            )
            .expect("insert");

        graph
            .add(
                happens_before(
                    operation(1),
                    operation(3),
                )
                .expect("valid"),
            )
            .expect("insert");

        assert_eq!(
            graph.outgoing(operation(1)).count(),
            2
        );

        assert_eq!(
            graph.incoming(operation(2)).count(),
            1
        );
    }

    #[test]
    fn pulse_dependency_is_supported() {
        let dependency =
            pulse_dependency(pulse(1), pulse(2))
                .expect("valid");

        assert_eq!(
            dependency.kind(),
            DependencyKind::Pulse
        );
    }

    #[test]
    fn dependency_with_separation_is_valid() {
        let duration =
            Duration::nanoseconds(20)
                .expect("valid duration");

        let dependency =
            happens_before(
                operation(1),
                operation(2),
            )
            .expect("valid")
            .with_minimum_separation(duration)
            .expect("valid separation");

        assert!(
            dependency.validate().is_ok()
        );
    }

    #[test]
    fn extension_dependency_requires_extension_metadata() {
        let extension =
            DependencyExtension::new(
                "zamani.test",
                "custom_order",
            )
            .expect("valid extension");

        let dependency =
            TemporalDependency::with_extension(
                operation(1),
                operation(2),
                extension,
                DependencyStrength::Required,
            )
            .expect("valid dependency");

        assert_eq!(
            dependency.kind(),
            DependencyKind::Extension
        );

        assert!(
            dependency.validate().is_ok()
        );
    }

    #[test]
    fn extension_namespace_cannot_be_empty() {
        assert!(
            DependencyExtension::new(
                "",
                "dependency",
            )
            .is_err()
        );
    }

    #[test]
    fn canonical_key_is_stable() {
        let dependency =
            happens_before(
                operation(10),
                operation(20),
            )
            .expect("valid");

        let first =
            dependency.canonical_key();

        let second =
            dependency.canonical_key();

        assert_eq!(first, second);
    }

    #[test]
    fn informational_dependency_is_not_required() {
        let dependency =
            TemporalDependency::with_strength(
                operation(1),
                operation(2),
                DependencyKind::Explicit,
                DependencyStrength::Informational,
            )
            .expect("valid");

        assert!(
            !dependency.strength().is_required()
        );
    }

    #[test]
    fn graph_validation_accepts_canonical_graph() {
        let mut graph =
            DependencyGraph::new();

        graph
            .add(
                happens_before(
                    operation(1),
                    operation(2),
                )
                .expect("valid"),
            )
            .expect("insert");

        graph
            .add(
                data_dependency(
                    value(3),
                    operation(4),
                )
                .expect("valid"),
            )
            .expect("insert");

        assert!(
            graph.validate().is_ok()
        );
    }
}