
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Physical Resource Observations
//!
//! Path: `src/zhal/resources/observation.rs`
//!
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides immutable, provider-independent resource-observation
//! collections and evidence-management contracts.
//!
//! This module builds upon the canonical resource model in:
//!
//!     crate::zhal::model::resource
//!
//! It does not redefine ResourceObservation, ResourceCapacity,
//! ResourceAmount, ResourceAvailability or ResourceProvenance.
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - Installed capacity evidence.
//! - Advertised capacity evidence.
//! - Resource observation records.
//! - Observation freshness assessments.
//! - Observation collection consistency.
//! - Immutable resource observation collections.
//! - Duplicate observation detection.
//! - Deterministic observation lookup and iteration.
//! - Freshness-dependent observation validation.
//!
//! # Does not own
//!
//! This module does not own:
//!
//! - Source-language resource requirements.
//! - Semantic resource analysis.
//! - Capability discovery.
//! - Hardware provider registration.
//! - Hardware discovery algorithms.
//! - Physical device access.
//! - Resource allocation.
//! - Resource reservation.
//! - Resource lease lifecycle.
//! - Scheduling.
//! - Placement.
//! - Routing.
//! - Runtime execution.
//!
//! # Canonical authorities
//!
//! Resource model:
//!     crate::zhal::model::resource
//!
//! Hardware identity:
//!     crate::zhal::identity
//!
//! Snapshot consistency:
//!     crate::zhal::model::snapshot
//!
//! Observation policy:
//!     crate::zhal::config
//!
//! Hardware errors:
//!     crate::zhal::errors
//!
//! Runtime resource management:
//!     crate::zur::resources
//!
//! # Fundamental distinctions
//!
//! Installed capacity != advertised capacity.
//!
//! Advertised capacity != observed capacity.
//!
//! Observed capacity != currently available capacity.
//!
//! Currently available capacity != reserved capacity.
//!
//! Reserved capacity != allocated capacity.
//!
//! Allocated capacity != successful execution.
//!
//! Unknown != zero.
//!
//! Stale != unavailable.
//!
//! Observation != guarantee.
//!
//! # Scalability
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Resource observations.
//! - Resource quantities.
//! - Hardware devices.
//! - Resource categories.
//! - Resource identifiers.
//! - Collection sizes.
//! - Provider populations.
//! - Computational domains.
//!
//! Collections grow according to actual input and available memory.
//!
//! Literal infinite storage or physical resources are not promised.
//!
//! # POCO-REAF
//!
//! The same resource observation contracts apply to compatible
//! classical, quantum, HDL, accelerator, embedded, hybrid,
//! distributed and future hardware providers.

use core::fmt;

use std::collections::BTreeMap;
use std::time::{Duration, SystemTime};

use crate::zhal::config::{
    ObservationPolicy,
    StaleObservationPolicy,
};

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

use crate::zhal::identity::HardwareResourceIdentity;

use crate::zhal::model::resource::{
    HardwareResource,
    ResourceAmount,
    ResourceAvailability,
    ResourceCapacity,
    ResourceObservation,
};

use crate::zhal::model::snapshot::SnapshotConsistency;

// ============================================================================
// COMPONENT CONTRACT
// ============================================================================

/// Stable component identifier.
pub const COMPONENT: &str = "zhal.resources.observation";

/// Observation contract schema identifier.
pub const OBSERVATION_SCHEMA_ID: &str =
    "zamani.zhal.resources.observation";

/// Observation contract schema version.
pub const OBSERVATION_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// ERRORS
// ============================================================================

fn observation_error(
    kind: ZhalErrorKind,
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(kind, message)
        .in_component(COMPONENT)
}

fn invalid_observation(
    message: impl Into<String>,
) -> ZhalError {
    observation_error(
        ZhalErrorKind::InvalidData,
        message,
    )
}

fn invalid_input(
    message: impl Into<String>,
) -> ZhalError {
    observation_error(
        ZhalErrorKind::InvalidInput,
        message,
    )
}

// ============================================================================
// CAPACITY EVIDENCE
// ============================================================================

/// Separates independently reported physical capacity evidence.
///
/// The quantities use the unit declared by the associated
/// HardwareResource.
///
/// Installed capacity describes the physical capacity reported
/// as installed by the provider.
///
/// Advertised capacity describes the capacity declared or exposed
/// by the provider.
///
/// Actual observed capacity is maintained by the canonical
/// ResourceCapacity model.
///
/// These values are intentionally independent.
///
/// A provider may report installed capacity without reporting
/// advertised capacity, or vice versa.
///
/// The implementation does not assume that advertised capacity
/// must equal installed capacity.
///
/// Nor does it assume that all installed capacity is currently
/// accessible.
///
/// This structure does not perform resource allocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceCapacityEvidence {
    installed: Option<ResourceAmount>,
    advertised: Option<ResourceAmount>,
}

impl ResourceCapacityEvidence {
    /// Creates capacity evidence.
    ///
    /// `None` means that the corresponding information is unknown
    /// or was not supplied.
    ///
    /// Zero must be represented by an actual ResourceAmount of zero.
    #[must_use]
    pub fn new(
        installed: Option<ResourceAmount>,
        advertised: Option<ResourceAmount>,
    ) -> Self {
        Self {
            installed,
            advertised,
        }
    }

    /// Creates evidence without installed or advertised quantities.
    ///
    /// This does not mean that the physical resource has zero capacity.
    #[must_use]
    pub const fn unknown() -> Self {
        Self {
            installed: None,
            advertised: None,
        }
    }

    /// Returns reported installed capacity.
    #[must_use]
    pub fn installed(&self) -> Option<&ResourceAmount> {
        self.installed.as_ref()
    }

    /// Returns reported advertised capacity.
    #[must_use]
    pub fn advertised(&self) -> Option<&ResourceAmount> {
        self.advertised.as_ref()
    }

    /// Returns whether installed capacity is known.
    #[must_use]
    pub fn has_installed_capacity(&self) -> bool {
        self.installed.is_some()
    }

    /// Returns whether advertised capacity is known.
    #[must_use]
    pub fn has_advertised_capacity(&self) -> bool {
        self.advertised.is_some()
    }

    /// Returns whether either capacity declaration is known.
    #[must_use]
    pub fn has_declared_capacity(&self) -> bool {
        self.has_installed_capacity()
            || self.has_advertised_capacity()
    }
}

impl Default for ResourceCapacityEvidence {
    fn default() -> Self {
        Self::unknown()
    }
}

// ============================================================================
// FRESHNESS
// ============================================================================

/// Classification of an observation's temporal validity.
///
/// Freshness is evaluated against an explicitly supplied clock
/// and an optional maximum-age policy.
///
/// No system clock is accessed implicitly.
///
/// This makes freshness assessment deterministic and testable.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ObservationFreshness {
    /// The observation is within the configured age policy.
    Current,

    /// The observation exceeds the configured maximum age.
    Stale,

    /// The observation timestamp is later than the evaluation clock.
    ///
    /// This can indicate clock skew or inconsistent clock domains.
    /// It must not be interpreted as fresh evidence.
    FutureTimestamp,

    /// The observation has explicitly been marked stale by its source.
    ExplicitlyStale,
}

impl ObservationFreshness {
    /// Returns whether the observation is demonstrably current.
    #[must_use]
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }

    /// Returns whether the observation is stale.
    #[must_use]
    pub const fn is_stale(self) -> bool {
        matches!(
            self,
            Self::Stale | Self::ExplicitlyStale
        )
    }

    /// Returns whether the observation has uncertain clock validity.
    #[must_use]
    pub const fn has_clock_uncertainty(self) -> bool {
        matches!(self, Self::FutureTimestamp)
    }
}

/// Immutable result of an observation freshness assessment.
///
/// The result records the observed age where the clock permits
/// calculating it.
///
/// A future timestamp has no meaningful non-negative age and
/// therefore produces `None`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FreshnessAssessment {
    freshness: ObservationFreshness,
    age: Option<Duration>,
    maximum_age: Option<Duration>,
}

impl FreshnessAssessment {
    /// Constructs a freshness assessment.
    #[must_use]
    pub const fn new(
        freshness: ObservationFreshness,
        age: Option<Duration>,
        maximum_age: Option<Duration>,
    ) -> Self {
        Self {
            freshness,
            age,
            maximum_age,
        }
    }

    /// Returns the freshness classification.
    #[must_use]
    pub const fn freshness(&self) -> ObservationFreshness {
        self.freshness
    }

    /// Returns the calculated observation age.
    ///
    /// Returns None when the observation timestamp is in the future.
    #[must_use]
    pub const fn age(&self) -> Option<Duration> {
        self.age
    }

    /// Returns the applicable maximum age.
    #[must_use]
    pub const fn maximum_age(&self) -> Option<Duration> {
        self.maximum_age
    }

    /// Returns whether the evidence is current.
    #[must_use]
    pub const fn is_current(&self) -> bool {
        self.freshness.is_current()
    }

    /// Returns whether the evidence is stale.
    #[must_use]
    pub const fn is_stale(&self) -> bool {
        self.freshness.is_stale()
    }

    /// Returns whether clock uncertainty exists.
    #[must_use]
    pub const fn has_clock_uncertainty(&self) -> bool {
        self.freshness.has_clock_uncertainty()
    }
}

impl fmt::Display for ObservationFreshness {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let value = match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::FutureTimestamp => "future_timestamp",
            Self::ExplicitlyStale => "explicitly_stale",
        };

        formatter.write_str(value)
    }
}

// ============================================================================
// RESOURCE OBSERVATION RECORD
// ============================================================================

/// Complete resource observation record.
///
/// This type reuses the canonical ResourceObservation instead
/// of introducing another physical resource observation model.
///
/// Additional capacity evidence is maintained independently
/// from actual observed capacity.
///
/// The observation remains immutable.
///
/// No resource is reserved or allocated by constructing this record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceObservationRecord {
    observation: ResourceObservation,
    capacity_evidence: ResourceCapacityEvidence,
}

impl ResourceObservationRecord {
    /// Creates a resource observation record.
    ///
    /// The underlying ResourceObservation has already validated
    /// its resource identity, provider provenance and capacity.
    #[must_use]
    pub fn new(
        observation: ResourceObservation,
        capacity_evidence: ResourceCapacityEvidence,
    ) -> Self {
        Self {
            observation,
            capacity_evidence,
        }
    }

    /// Creates a record without additional capacity declarations.
    #[must_use]
    pub fn without_capacity_declarations(
        observation: ResourceObservation,
    ) -> Self {
        Self::new(
            observation,
            ResourceCapacityEvidence::unknown(),
        )
    }

    /// Returns the canonical physical resource observation.
    #[must_use]
    pub fn observation(&self) -> &ResourceObservation {
        &self.observation
    }

    /// Returns the observed physical resource description.
    #[must_use]
    pub fn resource(&self) -> &HardwareResource {
        self.observation.resource()
    }

    /// Returns the actual observed capacity.
    #[must_use]
    pub fn observed_capacity(&self) -> &ResourceCapacity {
        self.observation.capacity()
    }

    /// Returns additional installed and advertised capacity evidence.
    #[must_use]
    pub fn capacity_evidence(&self) -> &ResourceCapacityEvidence {
        &self.capacity_evidence
    }

    /// Returns the physical resource availability observation.
    #[must_use]
    pub fn availability(&self) -> &ResourceAvailability {
        self.observation.availability()
    }

    /// Returns the provider provenance.
    #[must_use]
    pub fn provenance(
        &self,
    ) -> &crate::zhal::model::resource::ResourceProvenance {
        self.observation.provenance()
    }

    /// Returns the canonical resource identity.
    #[must_use]
    pub fn identity(&self) -> &HardwareResourceIdentity {
        self.resource().identity()
    }

    /// Returns the observation timestamp.
    #[must_use]
    pub fn observed_at(&self) -> SystemTime {
        self.observation.observed_at()
    }

    /// Returns whether the resource was explicitly reported available.
    ///
    /// This is not a reservation or continuing availability guarantee.
    #[must_use]
    pub fn is_observed_available(&self) -> bool {
        self.observation.is_observed_available()
    }

    /// Evaluates freshness against the supplied clock and policy.
    ///
    /// An explicitly stale provider observation always remains stale.
    ///
    /// A timestamp in the future is classified as clock uncertainty.
    ///
    /// If no maximum age is configured, an otherwise valid observation
    /// is current, but this does not establish continuing availability.
    #[must_use]
    pub fn assess_freshness(
        &self,
        now: SystemTime,
        policy: &ObservationPolicy,
    ) -> FreshnessAssessment {
        let maximum_age = policy.maximum_age();

        if matches!(
            self.availability(),
            ResourceAvailability::Stale
        ) {
            return FreshnessAssessment::new(
                ObservationFreshness::ExplicitlyStale,
                None,
                maximum_age,
            );
        }

        let age = match now.duration_since(self.observed_at()) {
            Ok(age) => age,

            Err(_) => {
                return FreshnessAssessment::new(
                    ObservationFreshness::FutureTimestamp,
                    None,
                    maximum_age,
                );
            }
        };

        let freshness = match maximum_age {
            Some(maximum_age) if age > maximum_age => {
                ObservationFreshness::Stale
            }

            _ => ObservationFreshness::Current,
        };

        FreshnessAssessment::new(
            freshness,
            Some(age),
            maximum_age,
        )
    }

    /// Ensures that the observation can be used for a
    /// freshness-dependent decision.
    ///
    /// The configured stale-observation policy is respected.
    ///
    /// Clock uncertainty is rejected because the observation
    /// cannot be established as current.
    ///
    /// This method does not establish resource availability,
    /// authorization, reservation or feasibility.
    ///
    /// # Errors
    ///
    /// Returns an error when the evidence is stale under a
    /// reject-stale policy, or its timestamp is in the future.
    pub fn ensure_acceptable(
        &self,
        now: SystemTime,
        policy: &ObservationPolicy,
    ) -> ZhalResult<FreshnessAssessment> {
        let assessment = self.assess_freshness(
            now,
            policy,
        );

        match assessment.freshness() {
            ObservationFreshness::FutureTimestamp => {
                Err(invalid_observation(
                    "resource observation timestamp is later than the evaluation clock",
                ))
            }

            ObservationFreshness::Stale
            | ObservationFreshness::ExplicitlyStale => {
                match policy.stale_policy() {
                    StaleObservationPolicy::ExposeAsStale => {
                        Ok(assessment)
                    }

                    StaleObservationPolicy::RejectForFreshnessDependentDecisions => {
                        Err(invalid_observation(
                            "stale resource evidence cannot be used for a freshness-dependent decision",
                        ))
                    }
                }
            }

            ObservationFreshness::Current => Ok(assessment),
        }
    }
}

// ============================================================================
// OBSERVATION COLLECTION
// ============================================================================

/// Immutable collection of resource observation records.
///
/// Resource identities are unique within a collection.
///
/// Records are ordered deterministically by their canonical
/// HardwareResourceIdentity.
///
/// The collection may contain resources from multiple providers.
///
/// The collection does not merge quantities from different resources
/// or units.
///
/// Partial and best-effort collections are supported.
///
/// A complete collection is complete only according to its declared
/// SnapshotConsistency. The declaration is not independent proof
/// of physical completeness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceObservationSet {
    observations: BTreeMap<
        HardwareResourceIdentity,
        ResourceObservationRecord,
    >,
    consistency: SnapshotConsistency,
}

impl ResourceObservationSet {
    /// Constructs an observation collection from resource records.
    ///
    /// Duplicate resource identities are rejected.
    ///
    /// No arbitrary maximum collection size is imposed.
    ///
    /// Actual collection growth is constrained by available memory
    /// and address-space representation.
    ///
    /// # Errors
    ///
    /// Returns an error if two records identify the same resource.
    pub fn new<I>(
        observations: I,
        consistency: SnapshotConsistency,
    ) -> ZhalResult<Self>
    where
        I: IntoIterator<Item = ResourceObservationRecord>,
    {
        let mut indexed = BTreeMap::new();

        for observation in observations {
            let identity = observation.identity().clone();

            if indexed.contains_key(&identity) {
                return Err(invalid_input(
                    "duplicate hardware resource identity in observation collection",
                )
                .with_context(
                    "resource_identity",
                    identity.to_string(),
                )?);
            }

            indexed.insert(
                identity,
                observation,
            );
        }

        Ok(Self {
            observations: indexed,
            consistency,
        })
    }

    /// Creates an empty collection with unknown consistency.
    ///
    /// An empty collection is not automatically considered complete.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            observations: BTreeMap::new(),
            consistency: SnapshotConsistency::Unknown,
        }
    }

    /// Returns the collection consistency.
    #[must_use]
    pub const fn consistency(&self) -> SnapshotConsistency {
        self.consistency
    }

    /// Returns whether the collection is explicitly complete.
    ///
    /// This reflects the declared collection contract.
    #[must_use]
    pub const fn is_declared_complete(&self) -> bool {
        matches!(
            self.consistency,
            SnapshotConsistency::Complete
        )
    }

    /// Returns whether the collection is explicitly partial.
    #[must_use]
    pub const fn is_partial(&self) -> bool {
        matches!(
            self.consistency,
            SnapshotConsistency::Partial
        )
    }

    /// Returns whether the collection uses best-effort semantics.
    #[must_use]
    pub const fn is_best_effort(&self) -> bool {
        matches!(
            self.consistency,
            SnapshotConsistency::BestEffort
        )
    }

    /// Returns the number of distinct observed resources.
    #[must_use]
    pub fn len(&self) -> usize {
        self.observations.len()
    }

    /// Returns whether the collection contains no resources.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.observations.is_empty()
    }

    /// Looks up an observation by canonical resource identity.
    #[must_use]
    pub fn get(
        &self,
        identity: &HardwareResourceIdentity,
    ) -> Option<&ResourceObservationRecord> {
        self.observations.get(identity)
    }

    /// Returns deterministic iteration over all resource observations.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (
        &HardwareResourceIdentity,
        &ResourceObservationRecord,
    )> {
        self.observations.iter()
    }

    /// Returns iteration over resource observation records.
    pub fn values(
        &self,
    ) -> impl Iterator<Item = &ResourceObservationRecord> {
        self.observations.values()
    }

    /// Returns iteration over canonical resource identities.
    pub fn identities(
        &self,
    ) -> impl Iterator<Item = &HardwareResourceIdentity> {
        self.observations.keys()
    }

    /// Returns whether every record has current evidence under
    /// the supplied freshness policy.
    ///
    /// An empty collection returns true vacuously. Callers must
    /// separately verify collection completeness and required coverage.
    #[must_use]
    pub fn all_current(
        &self,
        now: SystemTime,
        policy: &ObservationPolicy,
    ) -> bool {
        self.observations.values().all(|record| {
            record.assess_freshness(now, policy).is_current()
        })
    }

    /// Returns all observations whose evidence is stale.
    pub fn stale_observations(
        &self,
        now: SystemTime,
        policy: &ObservationPolicy,
    ) -> impl Iterator<Item = &ResourceObservationRecord> {
        self.observations.values().filter(move |record| {
            record
                .assess_freshness(now, policy)
                .is_stale()
        })
    }

    /// Returns observations with future timestamps.
    pub fn clock_uncertain_observations(
        &self,
        now: SystemTime,
        policy: &ObservationPolicy,
    ) -> impl Iterator<Item = &ResourceObservationRecord> {
        self.observations.values().filter(move |record| {
            record
                .assess_freshness(now, policy)
                .has_clock_uncertainty()
        })
    }

    /// Validates all records for freshness-dependent use.
    ///
    /// The first unacceptable observation returns an error.
    ///
    /// No resource is reserved or allocated.
    pub fn validate_freshness(
        &self,
        now: SystemTime,
        policy: &ObservationPolicy,
    ) -> ZhalResult<()> {
        for record in self.observations.values() {
            record.ensure_acceptable(now, policy)?;
        }

        Ok(())
    }
}

impl Default for ResourceObservationSet {
    fn default() -> Self {
        Self::empty()
    }
}

// ============================================================================
// OBSERVATION COLLECTION BUILDER
// ============================================================================

/// Builder for immutable resource observation collections.
///
/// The builder does not access hardware or discover resources.
///
/// All supplied observations must already have been constructed
/// and validated by their canonical provider/model interfaces.
#[derive(Clone, Debug, Default)]
pub struct ResourceObservationSetBuilder {
    observations: BTreeMap<
        HardwareResourceIdentity,
        ResourceObservationRecord,
    >,
    consistency: SnapshotConsistency,
}

impl ResourceObservationSetBuilder {
    /// Creates an empty builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the declared collection consistency.
    #[must_use]
    pub fn with_consistency(
        mut self,
        consistency: SnapshotConsistency,
    ) -> Self {
        self.consistency = consistency;
        self
    }

    /// Adds a resource observation.
    ///
    /// Duplicate identities are rejected instead of silently
    /// overwriting newer or older evidence.
    ///
    /// # Errors
    ///
    /// Returns an error if the resource identity already exists.
    pub fn add(
        &mut self,
        observation: ResourceObservationRecord,
    ) -> ZhalResult<()> {
        let identity = observation.identity().clone();

        if self.observations.contains_key(&identity) {
            return Err(invalid_input(
                "duplicate hardware resource identity in observation builder",
            )
            .with_context(
                "resource_identity",
                identity.to_string(),
            )?);
        }

        self.observations.insert(
            identity,
            observation,
        );

        Ok(())
    }

    /// Adds multiple resource observations.
    ///
    /// If any record is invalid, the builder is not returned
    /// as a completed collection.
    pub fn add_all<I>(
        &mut self,
        observations: I,
    ) -> ZhalResult<()>
    where
        I: IntoIterator<Item = ResourceObservationRecord>,
    {
        for observation in observations {
            self.add(observation)?;
        }

        Ok(())
    }

    /// Returns the current number of unique observations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.observations.len()
    }

    /// Returns whether the builder contains no observations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.observations.is_empty()
    }

    /// Builds the immutable observation collection.
    #[must_use]
    pub fn build(self) -> ResourceObservationSet {
        ResourceObservationSet {
            observations: self.observations,
            consistency: self.consistency,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::io::devices::DeviceId;
    use crate::stdlib::provider::ProviderId;

    use crate::zhal::identity::{
        HardwareDeviceIdentity,
        HardwareResourceId,
        HardwareResourceIdentity,
    };

    use crate::zhal::model::resource::{
        ResourceKindId,
        ResourceProvenance,
        ResourceUnitId,
    };

    fn provider() -> ProviderId {
        ProviderId::new("test.provider")
            .expect("valid provider")
    }

    fn identity(name: &str) -> HardwareResourceIdentity {
        HardwareResourceIdentity::new(
            HardwareDeviceIdentity::new(
                provider(),
                DeviceId::new("device.primary")
                    .expect("valid device"),
            ),
            HardwareResourceId::new(name)
                .expect("valid resource identity"),
        )
    }

    fn amount(value: &str) -> ResourceAmount {
        ResourceAmount::new(value)
            .expect("valid amount")
    }

    fn record(
        name: &str,
        observed_at: SystemTime,
        availability: ResourceAvailability,
    ) -> ResourceObservationRecord {
        let resource = HardwareResource::new(
            identity(name),
            ResourceKindId::new("classical.memory")
                .expect("valid resource kind"),
            ResourceUnitId::new(
                "information",
                "byte",
            )
            .expect("valid unit"),
        );

        let capacity = ResourceCapacity::new(
            Some(amount("1000")),
            Some(amount("400")),
            Some(amount("500")),
        )
        .expect("valid capacity");

        let observation = ResourceObservation::new(
            resource,
            capacity,
            availability,
            ResourceProvenance::new(provider()),
            observed_at,
        )
        .expect("valid resource observation");

        ResourceObservationRecord::without_capacity_declarations(
            observation,
        )
    }

    fn policy() -> ObservationPolicy {
        ObservationPolicy::default()
            .with_maximum_age(Some(Duration::from_secs(60)))
    }

    fn timestamp() -> SystemTime {
        SystemTime::UNIX_EPOCH
            + Duration::from_secs(1_000)
    }

    // ------------------------------------------------------------------------
    // Capacity evidence
    // ------------------------------------------------------------------------

    #[test]
    fn unknown_capacity_is_not_zero() {
        let evidence = ResourceCapacityEvidence::unknown();

        assert!(evidence.installed().is_none());
        assert!(evidence.advertised().is_none());
        assert!(!evidence.has_declared_capacity());
    }

    #[test]
    fn installed_and_advertised_capacity_are_independent() {
        let evidence = ResourceCapacityEvidence::new(
            Some(amount("100000000000000000000")),
            None,
        );

        assert_eq!(
            evidence.installed().unwrap().as_str(),
            "100000000000000000000",
        );

        assert!(evidence.advertised().is_none());
    }

    #[test]
    fn zero_capacity_is_distinct_from_unknown() {
        let evidence = ResourceCapacityEvidence::new(
            Some(amount("0")),
            None,
        );

        assert_eq!(
            evidence.installed().unwrap().as_str(),
            "0",
        );

        assert!(evidence.has_installed_capacity());
    }

    // ------------------------------------------------------------------------
    // Freshness
    // ------------------------------------------------------------------------

    #[test]
    fn recent_observation_is_current() {
        let now = timestamp();

        let record = record(
            "resource.current",
            now - Duration::from_secs(10),
            ResourceAvailability::Available,
        );

        let assessment = record.assess_freshness(
            now,
            &policy(),
        );

        assert_eq!(
            assessment.freshness(),
            ObservationFreshness::Current,
        );

        assert_eq!(
            assessment.age(),
            Some(Duration::from_secs(10)),
        );
    }

    #[test]
    fn old_observation_is_stale() {
        let now = timestamp();

        let record = record(
            "resource.old",
            now - Duration::from_secs(120),
            ResourceAvailability::Available,
        );

        let assessment = record.assess_freshness(
            now,
            &policy(),
        );

        assert_eq!(
            assessment.freshness(),
            ObservationFreshness::Stale,
        );
    }

    #[test]
    fn explicitly_stale_observation_remains_stale() {
        let now = timestamp();

        let record = record(
            "resource.explicitly_stale",
            now,
            ResourceAvailability::Stale,
        );

        assert_eq!(
            record.assess_freshness(now, &policy()).freshness(),
            ObservationFreshness::ExplicitlyStale,
        );
    }

    #[test]
    fn future_observation_is_not_treated_as_current() {
        let now = timestamp();

        let record = record(
            "resource.future",
            now + Duration::from_secs(30),
            ResourceAvailability::Available,
        );

        let assessment = record.assess_freshness(
            now,
            &policy(),
        );

        assert_eq!(
            assessment.freshness(),
            ObservationFreshness::FutureTimestamp,
        );

        assert!(assessment.age().is_none());
    }

    #[test]
    fn no_maximum_age_does_not_create_an_arbitrary_timeout() {
        let now = timestamp();

        let record = record(
            "resource.long_lived",
            now - Duration::from_secs(10_000_000),
            ResourceAvailability::Available,
        );

        let assessment = record.assess_freshness(
            now,
            &ObservationPolicy::default(),
        );

        assert_eq!(
            assessment.freshness(),
            ObservationFreshness::Current,
        );
    }

    #[test]
    fn stale_policy_can_reject_stale_evidence() {
        let now = timestamp();

        let record = record(
            "resource.stale",
            now - Duration::from_secs(120),
            ResourceAvailability::Available,
        );

        let strict = ObservationPolicy::default()
            .with_maximum_age(Some(Duration::from_secs(60)))
            .with_stale_policy(
                StaleObservationPolicy::
                    RejectForFreshnessDependentDecisions,
            );

        assert!(
            record.ensure_acceptable(now, &strict).is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Observation collections
    // ------------------------------------------------------------------------

    #[test]
    fn collection_preserves_distinct_resources() {
        let now = timestamp();

        let collection = ResourceObservationSet::new(
            [
                record(
                    "resource.one",
                    now,
                    ResourceAvailability::Available,
                ),
                record(
                    "resource.two",
                    now,
                    ResourceAvailability::Unknown,
                ),
            ],
            SnapshotConsistency::Complete,
        )
        .expect("valid collection");

        assert_eq!(collection.len(), 2);
        assert!(collection.is_declared_complete());
    }

    #[test]
    fn collection_rejects_duplicate_resource_identities() {
        let now = timestamp();

        let result = ResourceObservationSet::new(
            [
                record(
                    "resource.duplicate",
                    now,
                    ResourceAvailability::Available,
                ),
                record(
                    "resource.duplicate",
                    now,
                    ResourceAvailability::Unavailable,
                ),
            ],
            SnapshotConsistency::Complete,
        );

        assert!(result.is_err());
    }

    #[test]
    fn collection_supports_partial_observations() {
        let collection = ResourceObservationSet::new(
            [record(
                "resource.partial",
                timestamp(),
                ResourceAvailability::Unknown,
            )],
            SnapshotConsistency::Partial,
        )
        .unwrap();

        assert!(collection.is_partial());
        assert!(!collection.is_declared_complete());
    }

    #[test]
    fn empty_collection_is_not_automatically_complete() {
        let collection = ResourceObservationSet::empty();

        assert!(collection.is_empty());
        assert!(!collection.is_declared_complete());

        assert_eq!(
            collection.consistency(),
            SnapshotConsistency::Unknown,
        );
    }

    #[test]
    fn lookup_uses_canonical_resource_identity() {
        let now = timestamp();

        let expected = record(
            "resource.lookup",
            now,
            ResourceAvailability::Available,
        );

        let identity = expected.identity().clone();

        let collection = ResourceObservationSet::new(
            [expected],
            SnapshotConsistency::Complete,
        )
        .unwrap();

        assert!(collection.get(&identity).is_some());
    }

    #[test]
    fn collection_iteration_is_deterministic() {
        let now = timestamp();

        let collection = ResourceObservationSet::new(
            [
                record(
                    "resource.z",
                    now,
                    ResourceAvailability::Available,
                ),
                record(
                    "resource.a",
                    now,
                    ResourceAvailability::Available,
                ),
                record(
                    "resource.m",
                    now,
                    ResourceAvailability::Available,
                ),
            ],
            SnapshotConsistency::Complete,
        )
        .unwrap();

        let identities: Vec<&str> = collection
            .identities()
            .map(|identity| {
                identity.resource_id().as_str()
            })
            .collect();

        assert_eq!(
            identities,
            vec![
                "resource.a",
                "resource.m",
                "resource.z",
            ],
        );
    }

    #[test]
    fn collection_detects_stale_evidence() {
        let now = timestamp();

        let collection = ResourceObservationSet::new(
            [
                record(
                    "resource.fresh",
                    now,
                    ResourceAvailability::Available,
                ),
                record(
                    "resource.stale",
                    now - Duration::from_secs(120),
                    ResourceAvailability::Available,
                ),
            ],
            SnapshotConsistency::Complete,
        )
        .unwrap();

        assert_eq!(
            collection
                .stale_observations(now, &policy())
                .count(),
            1,
        );

        assert!(!collection.all_current(now, &policy()));
    }

    #[test]
    fn builder_rejects_duplicate_resources() {
        let now = timestamp();

        let mut builder = ResourceObservationSetBuilder::new();

        builder
            .add(record(
                "resource.same",
                now,
                ResourceAvailability::Available,
            ))
            .unwrap();

        assert!(
            builder
                .add(record(
                    "resource.same",
                    now,
                    ResourceAvailability::Unknown,
                ))
                .is_err()
        );
    }

    #[test]
    fn builder_preserves_declared_consistency() {
        let collection = ResourceObservationSetBuilder::new()
            .with_consistency(SnapshotConsistency::BestEffort)
            .build();

        assert!(collection.is_best_effort());
    }
}
