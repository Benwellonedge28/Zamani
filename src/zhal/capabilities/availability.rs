
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer — Availability
//!
//! Path: `src/zhal/capabilities/availability.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Evaluates the availability of physical hardware capabilities using
//! explicitly supplied, provenance-preserving hardware evidence.
//!
//! This module provides a deterministic assessment boundary between
//! hardware observations and downstream compatibility evaluation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Hardware capability availability assessments.
//! - Evidence freshness interpretation for availability.
//! - Explicit temporary-inaccessibility reporting.
//! - Contradictory-evidence detection.
//! - Aggregation of multiple observations for one capability.
//! - Requirement-oriented availability reports.
//!
//! This module does NOT own:
//!
//! - Capability identity.
//! - Capability definitions.
//! - Capability evidence storage.
//! - Hardware discovery.
//! - Provider registration or lifecycle.
//! - Provider authentication or trust verification.
//! - Source-language requirements.
//! - Semantic analysis.
//! - Preferences, constraints or hints.
//! - Workload feasibility.
//! - Device selection.
//! - Resource allocation or reservation.
//! - Scheduling or routing.
//! - Hardware execution.
//! - Quantum-specific availability semantics.
//! - HDL realization semantics.
//! - Distributed execution policy.
//!
//! ## Canonical authorities
//!
//! Capability identity and reported status:
//!     crate::stdlib::capability
//!
//! Physical hardware evidence:
//!     crate::zhal::capabilities::evidence
//!
//! Hardware-facing requirements:
//!     crate::zhal::capabilities::requirements
//!
//! Provider identity:
//!     crate::stdlib::provider
//!
//! ZHAL errors:
//!     crate::zhal::errors
//!
//! Workload feasibility:
//!     The owning semantic, compatibility and runtime authorities.
//!
//! ## Availability semantics
//!
//! Available:
//!     Current evidence establishes availability.
//!
//! Unavailable:
//!     Current evidence establishes unavailability.
//!
//! Conditional:
//!     Current evidence establishes that availability depends on
//!     conditions that have not been evaluated here.
//!
//! Unknown:
//!     There is insufficient evidence to determine availability.
//!
//! Stale:
//!     Evidence exists, but none of the relevant observations is current.
//!
//! TemporarilyInaccessible:
//!     The caller explicitly reports that the relevant hardware
//!     observation or access path is temporarily inaccessible.
//!
//! Conflicting:
//!     Current evidence contains contradictory available and
//!     unavailable assertions.
//!
//! Unknown is never interpreted as available.
//!
//! Stale evidence is never silently treated as current.
//!
//! A temporary access failure does not prove that the physical
//! capability itself is unavailable.
//!
//! ## Important distinctions
//!
//! Capability != availability.
//! Availability != compatibility.
//! Compatibility != feasibility.
//! Feasibility != reservation.
//! Reservation != execution.
//!
//! This module establishes none of the latter guarantees.
//!
//! ## Determinism
//!
//! Evaluation uses only explicitly supplied evidence and timestamps.
//!
//! No system clock is accessed.
//!
//! No hardware provider is queried.
//!
//! No network access occurs.
//!
//! Identical inputs produce identical assessments.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Capability identifiers.
//! - Evidence records.
//! - Requirements.
//! - Providers.
//! - Devices.
//! - Hardware categories.
//! - Computational domains.
//! - Assessment results.
//!
//! Collections grow according to available memory.
//!
//! Operational budgets remain the responsibility of the caller,
//! provider or runtime.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Safety
//!
//! No unsafe Rust.
//! No global mutable state.
//! No implicit hardware discovery.
//! No fabricated availability.
//! No implicit resource reservation.

use core::fmt;
use std::collections::BTreeSet;

use crate::stdlib::capability::{
    CapabilityId,
    CapabilityStatus,
};

use crate::zhal::capabilities::evidence::{
    EvidenceFreshness,
    EvidenceTimestamp,
    HardwareCapabilityEvidence,
};

use crate::zhal::capabilities::requirements::{
    CapabilityRequirementSet,
    HardwareCapabilityRequirement,
};

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

// ============================================================================
// COMPONENT IDENTIFICATION
// ============================================================================

/// Stable identifier of the ZHAL availability component.
pub const COMPONENT: &str = "zhal.capabilities.availability";

/// Version of the availability assessment schema.
///
/// This is independent of the Zamani language version, Cargo package
/// version, ZHAL API version, provider API version and firmware version.
pub const AVAILABILITY_SCHEMA_VERSION: &str = "1";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn availability_error(
    kind: ZhalErrorKind,
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(kind, message)
        .in_component(COMPONENT)
}

// ============================================================================
// AVAILABILITY STATE
// ============================================================================

/// The evaluated availability state of a physical hardware capability.
///
/// These states are intentionally distinct. In particular, unknown,
/// stale, inaccessible and unavailable must never be conflated.
///
/// `Conflicting` indicates contradictory current evidence rather than
/// an assertion about the physical capability itself.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[non_exhaustive]
pub enum AvailabilityState {
    /// Current evidence establishes unconditional availability.
    Available,

    /// Current evidence establishes unavailability.
    Unavailable,

    /// Availability depends on conditions not evaluated here.
    Conditional,

    /// Insufficient evidence exists to determine availability.
    Unknown,

    /// Relevant evidence exists, but no relevant observation is current.
    Stale,

    /// The caller reports temporary inability to access or observe
    /// the relevant hardware.
    TemporarilyInaccessible,

    /// Current evidence contains contradictory availability assertions.
    Conflicting,
}

impl AvailabilityState {
    /// Returns whether availability has been positively established.
    #[must_use]
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }

    /// Returns whether unavailability has been positively established.
    #[must_use]
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::Unavailable)
    }

    /// Returns whether availability depends on unevaluated conditions.
    #[must_use]
    pub const fn is_conditional(self) -> bool {
        matches!(self, Self::Conditional)
    }

    /// Returns whether availability is not established.
    #[must_use]
    pub const fn is_indeterminate(self) -> bool {
        matches!(
            self,
            Self::Unknown
                | Self::Stale
                | Self::TemporarilyInaccessible
                | Self::Conflicting
        )
    }

    /// Returns whether current evidence establishes availability.
    ///
    /// Only `Available` returns true.
    #[must_use]
    pub const fn establishes_availability(self) -> bool {
        matches!(self, Self::Available)
    }

    /// Returns a stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Unavailable => "unavailable",
            Self::Conditional => "conditional",
            Self::Unknown => "unknown",
            Self::Stale => "stale",
            Self::TemporarilyInaccessible => {
                "temporarily_inaccessible"
            }
            Self::Conflicting => "conflicting",
        }
    }
}

impl fmt::Display for AvailabilityState {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// ACCESS OBSERVATION
// ============================================================================

/// Explicit information about whether the relevant hardware can
/// currently be accessed or observed.
///
/// This is supplied by the caller or an upstream provider integration.
/// It is not discovered or inferred by this module.
///
/// An inaccessible observation path does not establish that the
/// underlying hardware capability is physically unavailable.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
)]
#[non_exhaustive]
pub enum AccessObservation {
    /// No access failure has been reported.
    ///
    /// This does not independently establish capability availability.
    Accessible,

    /// The caller reports temporary inability to access or observe
    /// the relevant hardware.
    TemporarilyInaccessible,
}

impl Default for AccessObservation {
    fn default() -> Self {
        Self::Accessible
    }
}

// ============================================================================
// EVIDENCE COUNTS
// ============================================================================

/// Counts the evidence records considered during an assessment.
///
/// Counts describe the supplied input records. They do not describe
/// the number of physical devices or guarantee inventory completeness.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    PartialEq,
)]
pub struct EvidenceCounts {
    total: usize,
    current: usize,
    stale: usize,
    not_yet_observed: usize,
    available: usize,
    unavailable: usize,
    conditional: usize,
    unknown: usize,
}

impl EvidenceCounts {
    /// Returns the total number of matching evidence records.
    #[must_use]
    pub const fn total(self) -> usize {
        self.total
    }

    /// Returns the number of current evidence records.
    #[must_use]
    pub const fn current(self) -> usize {
        self.current
    }

    /// Returns the number of expired evidence records.
    #[must_use]
    pub const fn stale(self) -> usize {
        self.stale
    }

    /// Returns the number of observations whose timestamps are
    /// later than the supplied evaluation timestamp.
    #[must_use]
    pub const fn not_yet_observed(self) -> usize {
        self.not_yet_observed
    }

    /// Returns the number of current available assertions.
    #[must_use]
    pub const fn available(self) -> usize {
        self.available
    }

    /// Returns the number of current unavailable assertions.
    #[must_use]
    pub const fn unavailable(self) -> usize {
        self.unavailable
    }

    /// Returns the number of current conditional assertions.
    #[must_use]
    pub const fn conditional(self) -> usize {
        self.conditional
    }

    /// Returns the number of current unknown assertions.
    #[must_use]
    pub const fn unknown(self) -> usize {
        self.unknown
    }
}

// ============================================================================
// AVAILABILITY ASSESSMENT
// ============================================================================

/// Immutable availability assessment for one canonical capability.
///
/// The assessment retains the capability identity and the evidence
/// counts used to reach its state.
///
/// It does not retain or replace the original evidence records.
/// Callers that require full provenance should retain the original
/// evidence collection alongside this assessment.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub struct AvailabilityAssessment {
    capability: CapabilityId,
    state: AvailabilityState,
    evaluated_at: EvidenceTimestamp,
    counts: EvidenceCounts,
}

impl AvailabilityAssessment {
    /// Returns the canonical capability identity.
    #[must_use]
    pub fn capability(&self) -> &CapabilityId {
        &self.capability
    }

    /// Returns the evaluated availability state.
    #[must_use]
    pub const fn state(&self) -> AvailabilityState {
        self.state
    }

    /// Returns the timestamp at which evidence was evaluated.
    #[must_use]
    pub const fn evaluated_at(&self) -> EvidenceTimestamp {
        self.evaluated_at
    }

    /// Returns the evidence counts used by the assessment.
    #[must_use]
    pub const fn evidence_counts(&self) -> EvidenceCounts {
        self.counts
    }

    /// Returns whether current evidence establishes availability.
    #[must_use]
    pub const fn is_available(&self) -> bool {
        self.state.is_available()
    }

    /// Returns whether the assessment requires further evidence
    /// or evaluation.
    #[must_use]
    pub const fn requires_further_evaluation(&self) -> bool {
        self.state.is_indeterminate()
            || self.state.is_conditional()
    }
}

// ============================================================================
// REQUIREMENT AVAILABILITY
// ============================================================================

/// Availability result associated with one hardware capability requirement.
///
/// The original requirement is not modified or reinterpreted.
/// Multiple requirements may refer to the same capability and will
/// produce separate results in their original insertion order.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub struct RequirementAvailability {
    requirement_index: usize,
    capability: CapabilityId,
    state: AvailabilityState,
    evidence_counts: EvidenceCounts,
}

impl RequirementAvailability {
    /// Returns the requirement's insertion-order index.
    #[must_use]
    pub const fn requirement_index(&self) -> usize {
        self.requirement_index
    }

    /// Returns the canonical capability identity.
    #[must_use]
    pub fn capability(&self) -> &CapabilityId {
        &self.capability
    }

    /// Returns the evaluated availability state.
    #[must_use]
    pub const fn state(&self) -> AvailabilityState {
        self.state
    }

    /// Returns the evidence counts for this capability.
    #[must_use]
    pub const fn evidence_counts(&self) -> EvidenceCounts {
        self.evidence_counts
    }

    /// Returns whether current evidence establishes availability.
    #[must_use]
    pub const fn is_available(&self) -> bool {
        self.state.is_available()
    }
}

// ============================================================================
// REQUIREMENT AVAILABILITY REPORT
// ============================================================================

/// Ordered availability results for a collection of requirements.
///
/// The result preserves requirement insertion order and duplicate
/// capability requirements.
///
/// This report is not a workload feasibility report.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub struct RequirementAvailabilityReport {
    evaluated_at: EvidenceTimestamp,
    results: Vec<RequirementAvailability>,
}

impl RequirementAvailabilityReport {
    /// Returns the timestamp used for all assessments.
    #[must_use]
    pub const fn evaluated_at(&self) -> EvidenceTimestamp {
        self.evaluated_at
    }

    /// Returns the number of assessed requirements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.results.len()
    }

    /// Returns whether the report contains no requirements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Returns all results in requirement insertion order.
    #[must_use]
    pub fn results(&self) -> &[RequirementAvailability] {
        &self.results
    }

    /// Returns one result by requirement insertion-order index.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&RequirementAvailability> {
        self.results.get(index)
    }

    /// Returns whether every assessed requirement has established
    /// availability.
    ///
    /// An empty report returns true because it contains no unsatisfied
    /// requirements. This is not a claim that an empty workload is
    /// executable or feasible.
    #[must_use]
    pub fn all_available(&self) -> bool {
        self.results
            .iter()
            .all(RequirementAvailability::is_available)
    }

    /// Returns an iterator over requirements that do not have
    /// established availability.
    pub fn not_established(
        &self,
    ) -> impl Iterator<Item = &RequirementAvailability> {
        self.results
            .iter()
            .filter(|result| !result.is_available())
    }
}

// ============================================================================
// EVIDENCE AGGREGATION
// ============================================================================

fn increment(
    value: &mut usize,
) -> ZhalResult<()> {
    *value = value.checked_add(1).ok_or_else(|| {
        availability_error(
            ZhalErrorKind::InvalidData,
            "evidence count exceeds the supported representation",
        )
    })?;

    Ok(())
}

fn count_evidence(
    capability: &CapabilityId,
    evidence: &[HardwareCapabilityEvidence],
    now: EvidenceTimestamp,
) -> ZhalResult<EvidenceCounts> {
    let mut counts = EvidenceCounts::default();

    for record in evidence {
        if record.capability() != capability {
            continue;
        }

        record.validate()?;

        increment(&mut counts.total)?;

        match record.freshness_at(now) {
            EvidenceFreshness::NotYetObserved => {
                increment(&mut counts.not_yet_observed)?;
            }

            EvidenceFreshness::Expired => {
                increment(&mut counts.stale)?;
            }

            EvidenceFreshness::Current => {
                increment(&mut counts.current)?;

                match record.status() {
                    CapabilityStatus::Available => {
                        increment(&mut counts.available)?;
                    }

                    CapabilityStatus::Unavailable => {
                        increment(&mut counts.unavailable)?;
                    }

                    CapabilityStatus::Conditional => {
                        increment(&mut counts.conditional)?;
                    }

                    CapabilityStatus::Unknown => {
                        increment(&mut counts.unknown)?;
                    }
                }
            }
        }
    }

    Ok(counts)
}

fn derive_state(
    counts: EvidenceCounts,
    access: AccessObservation,
) -> AvailabilityState {
    // A current inability to access the observation path takes
    // precedence over older capability assertions.
    if matches!(
        access,
        AccessObservation::TemporarilyInaccessible
    ) {
        return AvailabilityState::TemporarilyInaccessible;
    }

    // Contradictory current assertions must not be resolved by
    // arbitrary evidence ordering.
    if counts.available > 0 && counts.unavailable > 0 {
        return AvailabilityState::Conflicting;
    }

    if counts.available > 0 {
        return AvailabilityState::Available;
    }

    if counts.conditional > 0 {
        return AvailabilityState::Conditional;
    }

    if counts.unavailable > 0 {
        return AvailabilityState::Unavailable;
    }

    // Current unknown observations do not establish availability.
    if counts.unknown > 0 {
        return AvailabilityState::Unknown;
    }

    // Expired observations are stale. Future-dated observations
    // are not yet valid observations at the evaluation instant.
    if counts.stale > 0 || counts.not_yet_observed > 0 {
        return AvailabilityState::Stale;
    }

    AvailabilityState::Unknown
}

// ============================================================================
// PUBLIC ASSESSMENT API
// ============================================================================

/// Assesses one canonical capability against supplied hardware evidence.
///
/// Only records with the requested capability identity are considered.
///
/// All matching records are validated before the assessment is returned.
///
/// Evidence freshness is evaluated at the explicitly supplied timestamp.
/// No clock is accessed.
///
/// The access observation is an independent caller-supplied statement.
/// `TemporarilyInaccessible` does not mean that the physical capability
/// is unavailable.
///
/// # Errors
///
/// Returns a structured ZHAL error if matching evidence violates
/// its local validation invariants or evidence counters cannot be
/// represented.
///
/// # Examples
///
/// ```ignore
/// let assessment = assess_capability(
///     &capability,
///     &evidence,
///     now,
///     AccessObservation::Accessible,
/// )?;
///
/// if assessment.is_available() {
///     // Availability is established; compatibility and resource
///     // feasibility must still be evaluated separately.
/// }
/// ```
pub fn assess_capability(
    capability: &CapabilityId,
    evidence: &[HardwareCapabilityEvidence],
    now: EvidenceTimestamp,
    access: AccessObservation,
) -> ZhalResult<AvailabilityAssessment> {
    let counts = count_evidence(
        capability,
        evidence,
        now,
    )?;

    let state = derive_state(
        counts,
        access,
    );

    Ok(AvailabilityAssessment {
        capability: capability.clone(),
        state,
        evaluated_at: now,
        counts,
    })
}

/// Assesses every hardware capability requirement in insertion order.
///
/// Duplicate requirements are preserved and assessed independently.
/// Evidence is matched using the canonical capability identity.
///
/// The function does not evaluate preferences, constraints or hints.
/// It does not produce a feasibility decision.
///
/// The caller supplies the access observation because temporary
/// inaccessibility is an observation from an external access authority.
///
/// # Errors
///
/// Returns a structured ZHAL error if any matching evidence record
/// fails validation or a result counter cannot be represented.
pub fn assess_requirements(
    requirements: &CapabilityRequirementSet,
    evidence: &[HardwareCapabilityEvidence],
    now: EvidenceTimestamp,
    access: AccessObservation,
) -> ZhalResult<RequirementAvailabilityReport> {
    let mut results = Vec::with_capacity(requirements.len());

    for (index, requirement) in requirements.iter().enumerate() {
        let assessment = assess_capability(
            requirement.capability(),
            evidence,
            now,
            access,
        )?;

        results.push(RequirementAvailability {
            requirement_index: index,
            capability: requirement.capability().clone(),
            state: assessment.state(),
            evidence_counts: assessment.evidence_counts(),
        });
    }

    Ok(RequirementAvailabilityReport {
        evaluated_at: now,
        results,
    })
}

/// Assesses one hardware capability requirement.
///
/// This convenience function preserves the requirement's canonical
/// capability identity and does not modify the requirement.
pub fn assess_requirement(
    requirement: &HardwareCapabilityRequirement,
    evidence: &[HardwareCapabilityEvidence],
    now: EvidenceTimestamp,
    access: AccessObservation,
) -> ZhalResult<AvailabilityAssessment> {
    assess_capability(
        requirement.capability(),
        evidence,
        now,
        access,
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zhal::capabilities::evidence::{
        EvidenceOrigin,
        EvidenceTimestamp,
    };

    fn capability() -> CapabilityId {
        CapabilityId::new("future.photonic.compute")
            .expect("valid open-world capability identifier")
    }

    fn evidence(
        status: CapabilityStatus,
        observed_at: i128,
        valid_until: Option<i128>,
    ) -> HardwareCapabilityEvidence {
        let origin = EvidenceOrigin::new(
            "provider.report",
            Some("test.provider".to_owned()),
        )
        .expect("valid evidence origin");

        let record = HardwareCapabilityEvidence::new(
            capability(),
            status,
            origin,
            EvidenceTimestamp::from_unix_nanos(observed_at),
            "test provider",
            "test capability observation",
        )
        .expect("valid evidence");

        match valid_until {
            Some(expiration) => record
                .with_valid_until(
                    EvidenceTimestamp::from_unix_nanos(expiration),
                )
                .expect("valid evidence interval"),

            None => record,
        }
    }

    fn now() -> EvidenceTimestamp {
        EvidenceTimestamp::from_unix_nanos(100)
    }

    #[test]
    fn no_evidence_is_unknown() {
        let result = assess_capability(
            &capability(),
            &[],
            now(),
            AccessObservation::Accessible,
        )
        .expect("assessment succeeds");

        assert_eq!(result.state(), AvailabilityState::Unknown);
        assert!(!result.is_available());
    }

    #[test]
    fn current_available_evidence_establishes_availability() {
        let records = vec![
            evidence(CapabilityStatus::Available, 50, Some(150)),
        ];

        let result = assess_capability(
            &capability(),
            &records,
            now(),
            AccessObservation::Accessible,
        )
        .expect("assessment succeeds");

        assert_eq!(result.state(), AvailabilityState::Available);
        assert_eq!(result.evidence_counts().current(), 1);
    }

    #[test]
    fn current_unavailable_evidence_establishes_unavailability() {
        let records = vec![
            evidence(CapabilityStatus::Unavailable, 50, None),
        ];

        let result = assess_capability(
            &capability(),
            &records,
            now(),
            AccessObservation::Accessible,
        )
        .expect("assessment succeeds");

        assert_eq!(result.state(), AvailabilityState::Unavailable);
    }

    #[test]
    fn conditional_evidence_is_not_available() {
        let records = vec![
            evidence(CapabilityStatus::Conditional, 50, None),
        ];

        let result = assess_capability(
            &capability(),
            &records,
            now(),
            AccessObservation::Accessible,
        )
        .expect("assessment succeeds");

        assert_eq!(result.state(), AvailabilityState::Conditional);
        assert!(!result.is_available());
    }

    #[test]
    fn unknown_evidence_is_not_available() {
        let records = vec![
            evidence(CapabilityStatus::Unknown, 50, None),
        ];

        let result = assess_capability(
            &capability(),
            &records,
            now(),
            AccessObservation::Accessible,
        )
        .expect("assessment succeeds");

        assert_eq!(result.state(), AvailabilityState::Unknown);
        assert!(!result.is_available());
    }

    #[test]
    fn expired_evidence_is_stale() {
        let records = vec![
            evidence(CapabilityStatus::Available, 10, Some(100)),
        ];

        let result = assess_capability(
            &capability(),
            &records,
            now(),
            AccessObservation::Accessible,
        )
        .expect("assessment succeeds");

        assert_eq!(result.state(), AvailabilityState::Stale);
        assert_eq!(result.evidence_counts().stale(), 1);
        assert!(!result.is_available());
    }

    #[test]
    fn future_dated_evidence_is_not_current() {
        let records = vec![
            evidence(CapabilityStatus::Available, 200, None),
        ];

        let result = assess_capability(
            &capability(),
            &records,
            now(),
            AccessObservation::Accessible,
        )
        .expect("assessment succeeds");

        assert_eq!(result.state(), AvailabilityState::Stale);
        assert_eq!(result.evidence_counts().not_yet_observed(), 1);
    }

    #[test]
    fn temporary_inaccessibility_is_not_unavailability() {
        let records = vec![
            evidence(CapabilityStatus::Available, 50, None),
        ];

        let result = assess_capability(
            &capability(),
            &records,
            now(),
            AccessObservation::TemporarilyInaccessible,
        )
        .expect("assessment succeeds");

        assert_eq!(
            result.state(),
            AvailabilityState::TemporarilyInaccessible
        );

        assert!(!result.is_available());
        assert!(!result.state().is_unavailable());
    }

    #[test]
    fn contradictory_current_evidence_is_conflicting() {
        let records = vec![
            evidence(CapabilityStatus::Available, 50, None),
            evidence(CapabilityStatus::Unavailable, 60, None),
        ];

        let result = assess_capability(
            &capability(),
            &records,
            now(),
            AccessObservation::Accessible,
        )
        .expect("assessment succeeds");

        assert_eq!(result.state(), AvailabilityState::Conflicting);
        assert_eq!(result.evidence_counts().available(), 1);
        assert_eq!(result.evidence_counts().unavailable(), 1);
    }

    #[test]
    fn stale_conflicting_evidence_does_not_create_current_conflict() {
        let records = vec![
            evidence(CapabilityStatus::Available, 10, Some(20)),
            evidence(CapabilityStatus::Unavailable, 10, Some(20)),
        ];

        let result = assess_capability(
            &capability(),
            &records,
            now(),
            AccessObservation::Accessible,
        )
        .expect("assessment succeeds");

        assert_eq!(result.state(), AvailabilityState::Stale);
    }

    #[test]
    fn current_available_evidence_can_coexist_with_old_stale_evidence() {
        let records = vec![
            evidence(CapabilityStatus::Available, 50, None),
            evidence(CapabilityStatus::Unavailable, 10, Some(20)),
        ];

        let result = assess_capability(
            &capability(),
            &records,
            now(),
            AccessObservation::Accessible,
        )
        .expect("assessment succeeds");

        assert_eq!(result.state(), AvailabilityState::Available);
        assert_eq!(result.evidence_counts().stale(), 1);
    }

    #[test]
    fn evidence_for_other_capabilities_is_ignored() {
        let other = CapabilityId::new("classical.compute")
            .expect("valid capability");

        let origin = EvidenceOrigin::new(
            "provider.report",
            None,
        )
        .expect("valid origin");

        let record = HardwareCapabilityEvidence::new(
            other,
            CapabilityStatus::Available,
            origin,
            now(),
            "test provider",
            "unrelated capability",
        )
        .expect("valid evidence");

        let result = assess_capability(
            &capability(),
            &[record],
            now(),
            AccessObservation::Accessible,
        )
        .expect("assessment succeeds");

        assert_eq!(result.state(), AvailabilityState::Unknown);
        assert_eq!(result.evidence_counts().total(), 0);
    }

    #[test]
    fn duplicate_requirements_remain_in_input_order() {
        let first = HardwareCapabilityRequirement::new(
            capability(),
        );

        let second = HardwareCapabilityRequirement::new(
            capability(),
        );

        let mut requirements = CapabilityRequirementSet::new();
        requirements.push(first);
        requirements.push(second);

        let records = vec![
            evidence(CapabilityStatus::Available, 50, None),
        ];

        let report = assess_requirements(
            &requirements,
            &records,
            now(),
            AccessObservation::Accessible,
        )
        .expect("assessment succeeds");

        assert_eq!(report.len(), 2);
        assert_eq!(
            report.get(0).expect("first result").requirement_index(),
            0
        );
        assert_eq!(
            report.get(1).expect("second result").requirement_index(),
            1
        );
        assert!(report.all_available());
    }

    #[test]
    fn empty_requirement_report_is_empty() {
        let requirements = CapabilityRequirementSet::new();

        let report = assess_requirements(
            &requirements,
            &[],
            now(),
            AccessObservation::Accessible,
        )
        .expect("assessment succeeds");

        assert!(report.is_empty());
        assert!(report.all_available());
    }

    #[test]
    fn availability_state_has_stable_identifiers() {
        assert_eq!(
            AvailabilityState::TemporarilyInaccessible.as_str(),
            "temporarily_inaccessible"
        );

        assert_eq!(
            AvailabilityState::Conflicting.as_str(),
            "conflicting"
        );
    }
}
