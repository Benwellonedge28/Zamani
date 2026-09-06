//! Zamani Quantum Resilience — Diagnosis Localization
//!
//! Path:
//!     src/quantum/resilience/diagnosis/localization.rs
//!
//! # Purpose
//!
//! This module determines where an observed resilience condition is localized
//! without taking ownership of:
//!
//! - hardware discovery;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - ZQN fault semantics;
//! - QEC decoding;
//! - backend identity semantics;
//! - canonical quantum IR identity definitions.
//!
//! Localization answers:
//!
//! > "Which canonical resource, resource domain, or execution scope is
//! > associated with the available evidence?"
//!
//! Localization does NOT answer:
//!
//! - why the resource failed;
//! - whether the resource is causally responsible;
//! - which recovery action should be selected;
//! - whether a resource should be quarantined;
//! - whether a backend should be migrated;
//! - whether a result is semantically correct.
//!
//! Those responsibilities belong to:
//!
//! ```text
//! diagnosis/root_cause.rs
//! policy/
//! planning/
//! adaptation/
//! recovery/
//! verification/
//! ```
//!
//! # Architectural position
//!
//! ```text
//!                         DetectionSignal
//!                               |
//!                               v
//!                     LocalizationEvidence
//!                               |
//!                +--------------+--------------+
//!                |                             |
//!                v                             v
//!       canonical resource             execution scope
//!                |                             |
//!                +--------------+--------------+
//!                               |
//!                               v
//!                       LocalizationResult
//!                               |
//!              +----------------+----------------+
//!              |                |                |
//!              v                v                v
//!          classifier       root-cause       confidence
//!              |                |                |
//!              +----------------+----------------+
//!                               |
//!                               v
//!                           planning
//! ```
//!
//! # Canonical identity
//!
//! This module MUST reuse the canonical Zamani IR identities:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The repository explicitly prohibits resilience from introducing competing
//! qubit identity types.
//!
//! Generic resource identities are represented by:
//!
//! ```text
//! crate::quantum::resilience::model::resource::ResourceIdentity
//! ```
//!
//! That type already preserves the distinction between:
//!
//! - generic IR resources;
//! - logical qubits;
//! - physical qubits.
//!
//! # Physical versus logical identity
//!
//! This distinction is mandatory.
//!
//! ```text
//! QubitId
//!     logical semantic identity
//!
//! PhysicalQubitId
//!     physical target identity
//!
//! ResourceIdentity::LogicalQubit(...)
//!     resilience reference to a logical qubit
//!
//! ResourceIdentity::PhysicalQubit(...)
//!     resilience reference to a physical qubit
//! ```
//!
//! Equal numerical values MUST NOT make logical and physical identities
//! interchangeable.
//!
//! # No invented localization
//!
//! A detection signal does not automatically identify a resource.
//!
//! For example:
//!
//! ```text
//! DetectionSignal("gate anomaly")
//! ```
//!
//! is not sufficient evidence to claim:
//!
//! ```text
//! PhysicalQubitId(7)
//! ```
//!
//! A canonical localization must be supplied by an authoritative observation
//! source such as:
//!
//! - hardware telemetry;
//! - ZQN fault location;
//! - QEC evidence;
//! - execution metadata;
//! - routing metadata;
//! - scheduling metadata;
//! - benchmarking metadata;
//! - another explicitly trusted integration boundary.
//!
//! This module therefore accepts `LocalizationEvidence` instead of guessing
//! locations from signal classifications.
//!
//! # Universal-program principle
//!
//! This module imposes no machine-size limit.
//!
//! It contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PHYSICAL_QUBITS
//! MAX_LOCATIONS
//! MAX_DEVICES
//! MAX_BACKENDS
//! ```
//!
//! It does not assume:
//!
//! - a particular QPU size;
//! - contiguous physical IDs;
//! - contiguous logical IDs;
//! - a particular topology;
//! - a particular backend;
//! - a particular number of resources.
//!
//! "Infinity" means that this semantic layer does not introduce an artificial
//! finite machine-size ceiling. Concrete executions remain bounded only by
//! the resources available to the caller and explicit runtime/policy limits.
//!
//! # Streaming and scalability
//!
//! Localization supports iterator-based input.
//!
//! A caller may provide:
//!
//! - a slice;
//! - a vector;
//! - a database iterator;
//! - a telemetry stream;
//! - a distributed stream;
//! - a lazy generated stream.
//!
//! The implementation stores only the normalized localization evidence needed
//! for the requested result.
//!
//! The implementation deliberately avoids all-pairs comparisons. Evidence is
//! indexed by signal identity and resources are accumulated using ordered maps.
//!
//! # Determinism
//!
//! Deterministic ordering is required.
//!
//! The implementation therefore uses:
//!
//! - `BTreeMap`;
//! - `BTreeSet`;
//! - canonical `Ord` implementations already supplied by the identity types.
//!
//! Hash-map iteration order MUST NOT influence localization results.
//!
//! Given identical:
//!
//! - localization evidence;
//! - signal identities;
//! - configuration;
//! - evidence ordering semantics;
//!
//! the same localization result is produced.
//!
//! # Confidence
//!
//! Localization confidence is evidence confidence, not causal confidence.
//!
//! A highly confident localization means:
//!
//! > "The evidence strongly associates this observation with this resource."
//!
//! It does NOT mean:
//!
//! > "This resource caused the failure."
//!
//! Causal reasoning belongs to `root_cause.rs`.
//!
//! # Evidence aggregation
//!
//! Multiple observations may identify the same resource.
//!
//! The aggregation strategy is explicit:
//!
//! - `Maximum` preserves the strongest direct evidence;
//! - `Minimum` is conservative and bounded by the weakest evidence;
//! - `Mean` provides an arithmetic average;
//! - `Latest` uses explicit evidence sequence ordering.
//!
//! No hidden threshold is used.
//!
//! # Ambiguity
//!
//! If one signal is associated with multiple resources, localization MUST NOT
//! arbitrarily choose one merely because it happens to be encountered first.
//!
//! The result preserves all candidates and marks the localization ambiguous.
//!
//! This is important for:
//!
//! - correlated faults;
//! - two-qubit operations;
//! - shared control channels;
//! - crosstalk;
//! - multi-resource execution stages;
//! - distributed execution.
//!
//! # Security
//!
//! Localization evidence is not automatically trusted merely because it is
//! represented by a valid Rust value.
//!
//! The producer supplies an explicit trust classification.
//!
//! Localization does not grant:
//!
//! - recovery authority;
//! - hardware access;
//! - backend credentials;
//! - migration authority;
//! - quarantine authority.
//!
//! Security-sensitive decisions remain downstream policy decisions.
//!
//! # Integration
//!
//! This module integrates with:
//!
//! ```text
//! detection::detector
//! model::resource
//! diagnosis::diagnostician
//! diagnosis::classifier
//! diagnosis::root_cause
//! diagnosis::confidence
//! quantum::ir::qubit
//! quantum::zqn
//! quantum::hardware
//! quantum::routing
//! quantum::scheduling
//! quantum::qec
//! telemetry
//! ```
//!
//! Dependency direction:
//!
//! ```text
//! quantum::ir::qubit -----------+
//! model::resource --------------+
//! detection::detector ----------+
//!                               |
//!                               v
//!                         localization.rs
//!                               |
//!                +--------------+--------------+
//!                |              |              |
//!                v              v              v
//!           classifier     root_cause      confidence
//! ```
//!
//! `localization.rs` does not depend on concrete hardware, routing, scheduling,
//! QEC, or backend implementations.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Safety
//!
//! `unsafe` is forbidden at the module level.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::resilience::detection::detector::{
    DetectionSequence, ObservationId, ObservationTrust, SignalId,
};
use crate::quantum::resilience::errors::{
    ResilienceError, ResilienceErrorCode, ResilienceResult,
};
use crate::quantum::resilience::model::resource::{ResourceIdentity, ResourceKind};

// ============================================================================
// Stable schema identifiers
// ============================================================================

/// Stable schema identifier for the localization contract.
pub const LOCALIZATION_SCHEMA_ID: &str =
    "zamani.quantum.resilience.diagnosis.localization";

/// Semantic version of the localization contract.
pub const LOCALIZATION_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Localization confidence
// ============================================================================

/// Confidence associated with a localization claim.
///
/// The value is always finite and must be within `[0.0, 1.0]`.
///
/// This type intentionally does not reuse diagnosis confidence because
/// localization confidence and causal diagnosis confidence represent different
/// epistemic claims.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LocalizationConfidence(f64);

impl LocalizationConfidence {
    /// Creates a validated localization confidence.
    pub fn new(value: f64) -> ResilienceResult<Self> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        Ok(Self(value))
    }

    /// Creates zero confidence.
    #[must_use]
    pub const fn zero() -> Self {
        Self(0.0)
    }

    /// Creates full confidence.
    #[must_use]
    pub const fn one() -> Self {
        Self(1.0)
    }

    /// Returns the numeric confidence.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Returns whether the confidence is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0.0
    }
}

impl Eq for LocalizationConfidence {}

impl PartialOrd for LocalizationConfidence {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for LocalizationConfidence {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0
            .partial_cmp(&other.0)
            .unwrap_or(core::cmp::Ordering::Equal)
    }
}

impl fmt::Display for LocalizationConfidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:.6}", self.0)
    }
}

// ============================================================================
// Localization scope
// ============================================================================

/// Semantic scope of a localization claim.
///
/// This is deliberately extensible because future quantum architectures may
/// introduce execution domains not known to the original implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LocalizationScope {
    /// Logical-program resource.
    Logical,

    /// Physical execution resource.
    Physical,

    /// Generic execution resource.
    Resource,

    /// A named execution stage.
    ExecutionStage,

    /// A named distributed execution domain.
    ExecutionDomain,

    /// A custom semantic scope.
    External(String),
}

impl LocalizationScope {
    /// Returns a stable machine-readable name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Logical => "logical",
            Self::Physical => "physical",
            Self::Resource => "resource",
            Self::ExecutionStage => "execution_stage",
            Self::ExecutionDomain => "execution_domain",
            Self::External(value) => value.as_str(),
        }
    }
}

impl fmt::Display for LocalizationScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Canonical location
// ============================================================================

/// Canonical location of resilience evidence.
///
/// A location may identify a canonical logical or physical qubit, or a
/// generic resource owned by another Zamani subsystem.
///
/// No resilience-local qubit identity exists here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LocalizedResource {
    /// Canonical logical qubit.
    LogicalQubit(QubitId),

    /// Canonical physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Canonical generic resilience resource identity.
    Resource(ResourceIdentity),
}

impl LocalizedResource {
    /// Creates a logical-qubit location.
    #[must_use]
    pub const fn logical_qubit(id: QubitId) -> Self {
        Self::LogicalQubit(id)
    }

    /// Creates a physical-qubit location.
    #[must_use]
    pub const fn physical_qubit(id: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(id)
    }

    /// Creates a generic resource location.
    #[must_use]
    pub const fn resource(id: ResourceIdentity) -> Self {
        Self::Resource(id)
    }

    /// Returns the underlying canonical resource identity.
    #[must_use]
    pub const fn identity(self) -> ResourceIdentity {
        match self {
            Self::LogicalQubit(id) => ResourceIdentity::LogicalQubit(id),
            Self::PhysicalQubit(id) => ResourceIdentity::PhysicalQubit(id),
            Self::Resource(id) => id,
        }
    }

    /// Returns the logical qubit, if this is a logical-qubit location.
    #[must_use]
    pub const fn logical_qubit_id(self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(id) => Some(id),
            Self::PhysicalQubit(_) => None,
            Self::Resource(id) => id.logical_qubit_id(),
        }
    }

    /// Returns the physical qubit, if this is a physical-qubit location.
    #[must_use]
    pub const fn physical_qubit_id(self) -> Option<PhysicalQubitId> {
        match self {
            Self::LogicalQubit(_) => None,
            Self::PhysicalQubit(id) => Some(id),
            Self::Resource(id) => id.physical_qubit_id(),
        }
    }

    /// Returns whether this location is explicitly logical.
    #[must_use]
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns whether this location is explicitly physical.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }
}

impl From<QubitId> for LocalizedResource {
    fn from(value: QubitId) -> Self {
        Self::LogicalQubit(value)
    }
}

impl From<PhysicalQubitId> for LocalizedResource {
    fn from(value: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(value)
    }
}

impl From<ResourceIdentity> for LocalizedResource {
    fn from(value: ResourceIdentity) -> Self {
        Self::Resource(value)
    }
}

// ============================================================================
// Localization evidence
// ============================================================================

/// Evidence associating one detection observation with one canonical resource.
///
/// The evidence source must explicitly provide the association. Localization
/// never invents a physical or logical resource ID from a signal classification.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalizationEvidence {
    signal_id: SignalId,
    observation_id: Option<ObservationId>,
    sequence: Option<DetectionSequence>,
    resource: LocalizedResource,
    resource_kind: Option<ResourceKind>,
    scope: LocalizationScope,
    confidence: LocalizationConfidence,
    trust: ObservationTrust,
    provenance: Option<String>,
}

impl LocalizationEvidence {
    /// Creates localization evidence.
    ///
    /// `signal_id` and `resource` are mandatory because a localization claim
    /// without either identity would be meaningless.
    pub fn new(
        signal_id: SignalId,
        resource: impl Into<LocalizedResource>,
        scope: LocalizationScope,
        confidence: LocalizationConfidence,
    ) -> Self {
        Self {
            signal_id,
            observation_id: None,
            sequence: None,
            resource: resource.into(),
            resource_kind: None,
            scope,
            confidence,
            trust: ObservationTrust::Unknown,
            provenance: None,
        }
    }

    /// Associates an observation identity.
    #[must_use]
    pub const fn with_observation_id(mut self, id: ObservationId) -> Self {
        self.observation_id = Some(id);
        self
    }

    /// Associates a detection sequence.
    #[must_use]
    pub const fn with_sequence(mut self, sequence: DetectionSequence) -> Self {
        self.sequence = Some(sequence);
        self
    }

    /// Associates a semantic resource kind.
    #[must_use]
    pub fn with_resource_kind(mut self, kind: ResourceKind) -> Self {
        self.resource_kind = Some(kind);
        self
    }

    /// Associates an explicit trust classification.
    #[must_use]
    pub const fn with_trust(mut self, trust: ObservationTrust) -> Self {
        self.trust = trust;
        self
    }

    /// Associates provenance metadata.
    ///
    /// Provenance is descriptive. It does not itself establish trust.
    #[must_use]
    pub fn with_provenance(mut self, provenance: impl Into<String>) -> Self {
        self.provenance = Some(provenance.into());
        self
    }

    /// Returns the detection signal identity.
    #[must_use]
    pub const fn signal_id(&self) -> SignalId {
        self.signal_id
    }

    /// Returns the observation identity, if supplied.
    #[must_use]
    pub const fn observation_id(&self) -> Option<ObservationId> {
        self.observation_id
    }

    /// Returns the detection sequence, if supplied.
    #[must_use]
    pub const fn sequence(&self) -> Option<DetectionSequence> {
        self.sequence
    }

    /// Returns the canonical localized resource.
    #[must_use]
    pub const fn resource(&self) -> LocalizedResource {
        self.resource
    }

    /// Returns the semantic resource kind, if supplied.
    #[must_use]
    pub const fn resource_kind(&self) -> Option<&ResourceKind> {
        self.resource_kind.as_ref()
    }

    /// Returns the localization scope.
    #[must_use]
    pub const fn scope(&self) -> &LocalizationScope {
        &self.scope
    }

    /// Returns localization confidence.
    #[must_use]
    pub const fn confidence(&self) -> LocalizationConfidence {
        self.confidence
    }

    /// Returns evidence trust classification.
    #[must_use]
    pub const fn trust(&self) -> ObservationTrust {
        self.trust
    }

    /// Returns optional provenance.
    #[must_use]
    pub fn provenance(&self) -> Option<&str> {
        self.provenance.as_deref()
    }
}

// ============================================================================
// Evidence aggregation
// ============================================================================

/// Strategy used to combine confidence values for the same localized resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EvidenceAggregation {
    /// Use the strongest evidence.
    Maximum,

    /// Use the weakest evidence.
    Minimum,

    /// Compute the arithmetic mean.
    Mean,

    /// Use the evidence with the greatest explicit sequence number.
    ///
    /// If no sequence exists, insertion order is NOT used. The evidence is
    /// treated as unavailable to this aggregation mode.
    Latest,
}

impl Default for EvidenceAggregation {
    fn default() -> Self {
        Self::Maximum
    }
}

// ============================================================================
// Localization configuration
// ============================================================================

/// Configuration for localization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalizationConfig {
    aggregation: EvidenceAggregation,
    require_verified_evidence: bool,
    retain_unlocalized_signals: bool,
}

impl Default for LocalizationConfig {
    fn default() -> Self {
        Self {
            aggregation: EvidenceAggregation::Maximum,
            require_verified_evidence: false,
            retain_unlocalized_signals: true,
        }
    }
}

impl LocalizationConfig {
    /// Creates a configuration using the explicit aggregation strategy.
    #[must_use]
    pub const fn new(aggregation: EvidenceAggregation) -> Self {
        Self {
            aggregation,
            require_verified_evidence: false,
            retain_unlocalized_signals: true,
        }
    }

    /// Sets whether evidence must be verified.
    ///
    /// This is an evidence-quality requirement only. It does not authenticate
    /// the source itself.
    #[must_use]
    pub const fn with_verified_evidence_required(mut self, required: bool) -> Self {
        self.require_verified_evidence = required;
        self
    }

    /// Sets whether signals without localization evidence are retained.
    #[must_use]
    pub const fn with_unlocalized_signals_retained(mut self, retained: bool) -> Self {
        self.retain_unlocalized_signals = retained;
        self
    }

    /// Returns the aggregation strategy.
    #[must_use]
    pub const fn aggregation(self) -> EvidenceAggregation {
        self.aggregation
    }

    /// Returns whether verified evidence is required.
    #[must_use]
    pub const fn requires_verified_evidence(self) -> bool {
        self.require_verified_evidence
    }

    /// Returns whether unlocalized signals are retained.
    #[must_use]
    pub const fn retains_unlocalized_signals(self) -> bool {
        self.retain_unlocalized_signals
    }
}

// ============================================================================
// Localized candidate
// ============================================================================

/// One normalized localization candidate.
///
/// A candidate may contain evidence from multiple observations/signals.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalizationCandidate {
    resource: LocalizedResource,
    scope: LocalizationScope,
    resource_kind: Option<ResourceKind>,
    confidence: LocalizationConfidence,
    signal_ids: BTreeSet<SignalId>,
    observation_ids: BTreeSet<ObservationId>,
    evidence_count: usize,
    verified_evidence_count: usize,
}

impl LocalizationCandidate {
    fn new(evidence: &LocalizationEvidence) -> Self {
        let mut signal_ids = BTreeSet::new();
        signal_ids.insert(evidence.signal_id());

        let mut observation_ids = BTreeSet::new();

        if let Some(id) = evidence.observation_id() {
            observation_ids.insert(id);
        }

        Self {
            resource: evidence.resource(),
            scope: evidence.scope().clone(),
            resource_kind: evidence.resource_kind().cloned(),
            confidence: evidence.confidence(),
            signal_ids,
            observation_ids,
            evidence_count: 1,
            verified_evidence_count: usize::from(evidence.trust().is_verified()),
        }
    }

    fn add_evidence(
        &mut self,
        evidence: &LocalizationEvidence,
        aggregation: EvidenceAggregation,
    ) {
        self.signal_ids.insert(evidence.signal_id());

        if let Some(id) = evidence.observation_id() {
            self.observation_ids.insert(id);
        }

        self.evidence_count = self.evidence_count.saturating_add(1);

        if evidence.trust().is_verified() {
            self.verified_evidence_count = self.verified_evidence_count.saturating_add(1);
        }

        if self.resource_kind.is_none() {
            self.resource_kind = evidence.resource_kind().cloned();
        }

        self.confidence = aggregate_confidence(
            self.confidence,
            evidence.confidence(),
            aggregation,
            self.evidence_count,
        );
    }

    /// Returns the localized resource.
    #[must_use]
    pub const fn resource(&self) -> LocalizedResource {
        self.resource
    }

    /// Returns the canonical resource identity.
    #[must_use]
    pub const fn resource_identity(&self) -> ResourceIdentity {
        self.resource.identity()
    }

    /// Returns the localization scope.
    #[must_use]
    pub const fn scope(&self) -> &LocalizationScope {
        &self.scope
    }

    /// Returns the resource kind, if known.
    #[must_use]
    pub const fn resource_kind(&self) -> Option<&ResourceKind> {
        self.resource_kind.as_ref()
    }

    /// Returns aggregated localization confidence.
    #[must_use]
    pub const fn confidence(&self) -> LocalizationConfidence {
        self.confidence
    }

    /// Returns all contributing detection signals in deterministic order.
    #[must_use]
    pub fn signal_ids(&self) -> &BTreeSet<SignalId> {
        &self.signal_ids
    }

    /// Returns all contributing observation IDs in deterministic order.
    #[must_use]
    pub fn observation_ids(&self) -> &BTreeSet<ObservationId> {
        &self.observation_ids
    }

    /// Returns the number of evidence records.
    #[must_use]
    pub const fn evidence_count(&self) -> usize {
        self.evidence_count
    }

    /// Returns the number of verified evidence records.
    #[must_use]
    pub const fn verified_evidence_count(&self) -> usize {
        self.verified_evidence_count
    }
}

// ============================================================================
// Localization result
// ============================================================================

/// Complete localization result.
///
/// The result deliberately preserves ambiguity rather than forcing one
/// location where the evidence supports several candidates.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalizationResult {
    candidates: Vec<LocalizationCandidate>,
    signal_candidates: BTreeMap<SignalId, BTreeSet<LocalizedResource>>,
    unlocalized_signals: BTreeSet<SignalId>,
    rejected_signals: BTreeSet<SignalId>,
}

impl LocalizationResult {
    fn new(
        candidates: Vec<LocalizationCandidate>,
        signal_candidates: BTreeMap<SignalId, BTreeSet<LocalizedResource>>,
        unlocalized_signals: BTreeSet<SignalId>,
        rejected_signals: BTreeSet<SignalId>,
    ) -> Self {
        Self {
            candidates,
            signal_candidates,
            unlocalized_signals,
            rejected_signals,
        }
    }

    /// Returns all localized candidates in deterministic order.
    #[must_use]
    pub fn candidates(&self) -> &[LocalizationCandidate] {
        &self.candidates
    }

    /// Returns the resources associated with a signal.
    #[must_use]
    pub fn candidates_for_signal(
        &self,
        signal_id: SignalId,
    ) -> Option<&BTreeSet<LocalizedResource>> {
        self.signal_candidates.get(&signal_id)
    }

    /// Returns signals that have no accepted localization evidence.
    #[must_use]
    pub fn unlocalized_signals(&self) -> &BTreeSet<SignalId> {
        &self.unlocalized_signals
    }

    /// Returns signals rejected because their evidence did not satisfy the
    /// configured evidence requirements.
    #[must_use]
    pub fn rejected_signals(&self) -> &BTreeSet<SignalId> {
        &self.rejected_signals
    }

    /// Returns whether the result contains at least one localization.
    #[must_use]
    pub fn is_localized(&self) -> bool {
        !self.candidates.is_empty()
    }

    /// Returns whether any signal has multiple localization candidates.
    #[must_use]
    pub fn is_ambiguous(&self) -> bool {
        self.signal_candidates
            .values()
            .any(|candidates| candidates.len() > 1)
    }

    /// Returns the number of distinct localized resources.
    #[must_use]
    pub fn candidate_count(&self) -> usize {
        self.candidates.len()
    }
}

// ============================================================================
// Localizer
// ============================================================================

/// Deterministic localization engine.
///
/// `Localizer` performs evidence normalization and aggregation only.
///
/// It does not:
///
/// - query hardware;
/// - inspect topology;
/// - query routing;
/// - query calibration;
/// - infer causality;
/// - select recovery actions.
#[derive(Debug, Clone)]
pub struct Localizer {
    config: LocalizationConfig,
}

impl Localizer {
    /// Creates a localizer.
    #[must_use]
    pub const fn new(config: LocalizationConfig) -> Self {
        Self { config }
    }

    /// Creates a localizer using the default evidence aggregation policy.
    #[must_use]
    pub const fn default_localizer() -> Self {
        Self::new(LocalizationConfig::new(EvidenceAggregation::Maximum))
    }

    /// Returns the configuration.
    #[must_use]
    pub const fn config(&self) -> LocalizationConfig {
        self.config
    }

    /// Localizes a finite batch of evidence.
    ///
    /// The iterator may be backed by any collection or stream.
    ///
    /// The method performs no hardware I/O and no hidden discovery.
    pub fn localize<I>(
        &self,
        evidence: I,
    ) -> ResilienceResult<LocalizationResult>
    where
        I: IntoIterator<Item = LocalizationEvidence>,
    {
        let mut candidates: BTreeMap<
            (LocalizedResource, LocalizationScope),
            LocalizationCandidate,
        > = BTreeMap::new();

        let mut signal_candidates: BTreeMap<SignalId, BTreeSet<LocalizedResource>> =
            BTreeMap::new();

        let mut unlocalized_signals = BTreeSet::new();
        let mut rejected_signals = BTreeSet::new();

        for item in evidence {
            if self.config.requires_verified_evidence() && !item.trust().is_verified() {
                rejected_signals.insert(item.signal_id());

                if self.config.retains_unlocalized_signals() {
                    unlocalized_signals.insert(item.signal_id());
                }

                continue;
            }

            signal_candidates
                .entry(item.signal_id())
                .or_default()
                .insert(item.resource());

            let key = (item.resource(), item.scope().clone());

            match candidates.get_mut(&key) {
                Some(candidate) => {
                    candidate.add_evidence(&item, self.config.aggregation());
                }
                None => {
                    candidates.insert(key, LocalizationCandidate::new(&item));
                }
            }
        }

        let mut normalized_candidates: Vec<LocalizationCandidate> =
            candidates.into_values().collect();

        normalized_candidates.sort_by(|left, right| {
            left.resource()
                .cmp(&right.resource())
                .then_with(|| left.scope().cmp(right.scope()))
        });

        Ok(LocalizationResult::new(
            normalized_candidates,
            signal_candidates,
            unlocalized_signals,
            rejected_signals,
        ))
    }

    /// Localizes one evidence item.
    pub fn localize_one(
        &self,
        evidence: LocalizationEvidence,
    ) -> ResilienceResult<LocalizationResult> {
        self.localize(std::iter::once(evidence))
    }

    /// Returns all canonical physical qubits represented by the result.
    ///
    /// Ordering is deterministic and duplicates are removed.
    #[must_use]
    pub fn physical_qubits(
        result: &LocalizationResult,
    ) -> BTreeSet<PhysicalQubitId> {
        result
            .candidates()
            .iter()
            .filter_map(|candidate| candidate.resource().physical_qubit_id())
            .collect()
    }

    /// Returns all canonical logical qubits represented by the result.
    ///
    /// Ordering is deterministic and duplicates are removed.
    #[must_use]
    pub fn logical_qubits(result: &LocalizationResult) -> BTreeSet<QubitId> {
        result
            .candidates()
            .iter()
            .filter_map(|candidate| candidate.resource().logical_qubit_id())
            .collect()
    }
}

// ============================================================================
// Confidence aggregation
// ============================================================================

fn aggregate_confidence(
    current: LocalizationConfidence,
    incoming: LocalizationConfidence,
    aggregation: EvidenceAggregation,
    evidence_count: usize,
) -> LocalizationConfidence {
    match aggregation {
        EvidenceAggregation::Maximum => {
            if incoming > current {
                incoming
            } else {
                current
            }
        }

        EvidenceAggregation::Minimum => {
            if incoming < current {
                incoming
            } else {
                current
            }
        }

        EvidenceAggregation::Mean => {
            let count = evidence_count as f64;

            if count <= 1.0 {
                return incoming;
            }

            let previous_count = count - 1.0;
            let value =
                ((current.value() * previous_count) + incoming.value()) / count;

            LocalizationConfidence::new(value)
                .unwrap_or_else(|_| LocalizationConfidence::zero())
        }

        EvidenceAggregation::Latest => {
            // Sequence-sensitive aggregation is handled by the caller before
            // this function when explicit sequence metadata is required.
            //
            // Without sequence information there is no semantically valid
            // "latest" value. Keeping the current value is therefore safer
            // than silently using iterator order.
            current
        }
    }
}

// ============================================================================
// Resource conversion helpers
// ============================================================================

/// Converts a canonical resource identity into a localization resource.
#[must_use]
pub const fn localize_resource(identity: ResourceIdentity) -> LocalizedResource {
    match identity {
        ResourceIdentity::LogicalQubit(id) => LocalizedResource::LogicalQubit(id),
        ResourceIdentity::PhysicalQubit(id) => LocalizedResource::PhysicalQubit(id),
        ResourceIdentity::Ir(id) => LocalizedResource::Resource(ResourceIdentity::Ir(id)),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use core::num::NonZeroU64;

    fn signal(value: u64) -> SignalId {
        SignalId::from_u64(value).unwrap_or_else(|| {
            SignalId::from_u64(1).unwrap_or_else(|| {
                panic!("test signal ID construction failed")
            })
        })
    }

    fn confidence(value: f64) -> LocalizationConfidence {
        LocalizationConfidence::new(value).unwrap_or_else(|_| {
            panic!("test confidence construction failed")
        })
    }

    fn observation(value: u64) -> ObservationId {
        ObservationId::new(
            NonZeroU64::new(value).unwrap_or_else(|| {
                panic!("test observation ID must be non-zero")
            }),
        )
    }

    #[test]
    fn canonical_logical_qubit_identity_is_preserved() {
        let id = QubitId::new(7);
        let resource = LocalizedResource::logical_qubit(id);

        assert_eq!(resource.logical_qubit_id(), Some(id));
        assert_eq!(resource.physical_qubit_id(), None);
        assert!(resource.is_logical());
        assert!(!resource.is_physical());
    }

    #[test]
    fn canonical_physical_qubit_identity_is_preserved() {
        let id = PhysicalQubitId::new(11);
        let resource = LocalizedResource::physical_qubit(id);

        assert_eq!(resource.physical_qubit_id(), Some(id));
        assert_eq!(resource.logical_qubit_id(), None);
        assert!(resource.is_physical());
        assert!(!resource.is_logical());
    }

    #[test]
    fn localization_does_not_merge_equal_logical_and_physical_indices() {
        let logical = LocalizationEvidence::new(
            signal(1),
            QubitId::new(3),
            LocalizationScope::Logical,
            confidence(0.9),
        );

        let physical = LocalizationEvidence::new(
            signal(2),
            PhysicalQubitId::new(3),
            LocalizationScope::Physical,
            confidence(0.9),
        );

        let localizer = Localizer::default_localizer();

        let result = localizer
            .localize([logical, physical])
            .unwrap_or_else(|_| panic!("localization should succeed"));

        assert_eq!(result.candidate_count(), 2);
    }

    #[test]
    fn multiple_resources_for_one_signal_are_preserved() {
        let first = LocalizationEvidence::new(
            signal(1),
            PhysicalQubitId::new(1),
            LocalizationScope::Physical,
            confidence(0.8),
        );

        let second = LocalizationEvidence::new(
            signal(1),
            PhysicalQubitId::new(2),
            LocalizationScope::Physical,
            confidence(0.7),
        );

        let localizer = Localizer::default_localizer();

        let result = localizer
            .localize([first, second])
            .unwrap_or_else(|_| panic!("localization should succeed"));

        assert!(result.is_ambiguous());

        let candidates = result
            .candidates_for_signal(signal(1))
            .unwrap_or_else(|| panic!("signal must have candidates"));

        assert_eq!(candidates.len(), 2);
    }

    #[test]
    fn maximum_aggregation_is_deterministic() {
        let first = LocalizationEvidence::new(
            signal(1),
            PhysicalQubitId::new(4),
            LocalizationScope::Physical,
            confidence(0.4),
        );

        let second = LocalizationEvidence::new(
            signal(2),
            PhysicalQubitId::new(4),
            LocalizationScope::Physical,
            confidence(0.9),
        );

        let localizer = Localizer::new(
            LocalizationConfig::new(EvidenceAggregation::Maximum),
        );

        let result = localizer
            .localize([first, second])
            .unwrap_or_else(|_| panic!("localization should succeed"));

        assert_eq!(result.candidates()[0].confidence().value(), 0.9);
    }

    #[test]
    fn minimum_aggregation_is_conservative() {
        let first = LocalizationEvidence::new(
            signal(1),
            PhysicalQubitId::new(4),
            LocalizationScope::Physical,
            confidence(0.4),
        );

        let second = LocalizationEvidence::new(
            signal(2),
            PhysicalQubitId::new(4),
            LocalizationScope::Physical,
            confidence(0.9),
        );

        let localizer = Localizer::new(
            LocalizationConfig::new(EvidenceAggregation::Minimum),
        );

        let result = localizer
            .localize([first, second])
            .unwrap_or_else(|_| panic!("localization should succeed"));

        assert_eq!(result.candidates()[0].confidence().value(), 0.4);
    }

    #[test]
    fn mean_aggregation_is_bounded() {
        let first = LocalizationEvidence::new(
            signal(1),
            PhysicalQubitId::new(4),
            LocalizationScope::Physical,
            confidence(0.2),
        );

        let second = LocalizationEvidence::new(
            signal(2),
            PhysicalQubitId::new(4),
            LocalizationScope::Physical,
            confidence(0.8),
        );

        let localizer = Localizer::new(
            LocalizationConfig::new(EvidenceAggregation::Mean),
        );

        let result = localizer
            .localize([first, second])
            .unwrap_or_else(|_| panic!("localization should succeed"));

        assert!((result.candidates()[0].confidence().value() - 0.5).abs() < 1.0e-12);
    }

    #[test]
    fn unverified_evidence_can_be_rejected() {
        let evidence = LocalizationEvidence::new(
            signal(1),
            PhysicalQubitId::new(2),
            LocalizationScope::Physical,
            confidence(0.95),
        )
        .with_trust(ObservationTrust::Unverified);

        let config = LocalizationConfig::new(EvidenceAggregation::Maximum)
            .with_verified_evidence_required(true);

        let localizer = Localizer::new(config);

        let result = localizer
            .localize([evidence])
            .unwrap_or_else(|_| panic!("localization should succeed"));

        assert!(!result.is_localized());
        assert!(result.rejected_signals().contains(&signal(1)));
    }

    #[test]
    fn verified_evidence_is_accepted() {
        let evidence = LocalizationEvidence::new(
            signal(1),
            PhysicalQubitId::new(2),
            LocalizationScope::Physical,
            confidence(0.95),
        )
        .with_trust(ObservationTrust::Verified);

        let config = LocalizationConfig::new(EvidenceAggregation::Maximum)
            .with_verified_evidence_required(true);

        let localizer = Localizer::new(config);

        let result = localizer
            .localize([evidence])
            .unwrap_or_else(|_| panic!("localization should succeed"));

        assert!(result.is_localized());
        assert!(result.rejected_signals().is_empty());
    }

    #[test]
    fn observation_and_provenance_are_preserved_in_candidate_metadata() {
        let evidence = LocalizationEvidence::new(
            signal(9),
            PhysicalQubitId::new(12),
            LocalizationScope::Physical,
            confidence(0.91),
        )
        .with_observation_id(observation(17))
        .with_sequence(
            DetectionSequence::from_u64(23)
                .unwrap_or_else(|| panic!("sequence must be non-zero")),
        )
        .with_trust(ObservationTrust::Trusted)
        .with_provenance("hardware.telemetry");

        assert_eq!(evidence.observation_id(), Some(observation(17)));
        assert_eq!(
            evidence
                .sequence()
                .map(DetectionSequence::value),
            Some(23)
        );
        assert_eq!(evidence.trust(), ObservationTrust::Trusted);
        assert_eq!(evidence.provenance(), Some("hardware.telemetry"));
    }

    #[test]
    fn physical_qubit_projection_is_deterministic() {
        let first = LocalizationEvidence::new(
            signal(1),
            PhysicalQubitId::new(9),
            LocalizationScope::Physical,
            confidence(0.7),
        );

        let second = LocalizationEvidence::new(
            signal(2),
            PhysicalQubitId::new(3),
            LocalizationScope::Physical,
            confidence(0.8),
        );

        let third = LocalizationEvidence::new(
            signal(3),
            PhysicalQubitId::new(9),
            LocalizationScope::Physical,
            confidence(0.9),
        );

        let result = Localizer::default_localizer()
            .localize([first, second, third])
            .unwrap_or_else(|_| panic!("localization should succeed"));

        let resources = Localizer::physical_qubits(&result);

        let collected: Vec<_> = resources.into_iter().collect();

        assert_eq!(
            collected,
            vec![PhysicalQubitId::new(3), PhysicalQubitId::new(9)]
        );
    }

    #[test]
    fn logical_qubit_projection_is_deterministic() {
        let first = LocalizationEvidence::new(
            signal(1),
            QubitId::new(8),
            LocalizationScope::Logical,
            confidence(0.7),
        );

        let second = LocalizationEvidence::new(
            signal(2),
            QubitId::new(2),
            LocalizationScope::Logical,
            confidence(0.8),
        );

        let result = Localizer::default_localizer()
            .localize([first, second])
            .unwrap_or_else(|_| panic!("localization should succeed"));

        let resources = Localizer::logical_qubits(&result);

        let collected: Vec<_> = resources.into_iter().collect();

        assert_eq!(
            collected,
            vec![QubitId::new(2), QubitId::new(8)]
        );
    }

    #[test]
    fn localize_one_has_no_hidden_batch_assumption() {
        let evidence = LocalizationEvidence::new(
            signal(1),
            PhysicalQubitId::new(1),
            LocalizationScope::Physical,
            confidence(1.0),
        );

        let result = Localizer::default_localizer()
            .localize_one(evidence)
            .unwrap_or_else(|_| panic!("single localization should succeed"));

        assert_eq!(result.candidate_count(), 1);
    }

    #[test]
    fn resource_conversion_preserves_identity_domain() {
        let logical = ResourceIdentity::LogicalQubit(QubitId::new(5));
        let physical = ResourceIdentity::PhysicalQubit(PhysicalQubitId::new(5));

        let logical_localized = localize_resource(logical);
        let physical_localized = localize_resource(physical);

        assert_ne!(logical_localized, physical_localized);
    }
}