
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! Hardware Capability Compatibility Evaluation.
//!
//! Path: src/zhal/capabilities/compatibility.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Evaluates whether explicitly supplied hardware capability evidence
//! satisfies an ordered collection of mandatory hardware capability
//! requirements.
//!
//! The evaluator is provider-independent and hardware-independent.
//!
//! It supports classical, quantum, HDL, accelerator, embedded,
//! distributed, hybrid and future computational technologies through
//! the canonical open-world capability identity.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Hardware capability compatibility evaluation.
//! - Compatibility policies.
//! - Compatibility findings.
//! - Compatibility reports.
//! - Provider/device-scoped evidence evaluation.
//! - Evidence verification policy enforcement.
//! - Structured compatibility diagnostics.
//! - Deterministic report generation.
//!
//! This module does NOT own:
//!
//! - Capability identity.
//! - Capability definitions.
//! - Source-language semantics.
//! - AST or grammar.
//! - Requirement construction.
//! - Preference semantics.
//! - Constraint semantics.
//! - Hint semantics.
//! - Physical hardware discovery.
//! - Provider registration.
//! - Provider lifecycle.
//! - Provider authentication.
//! - Cryptographic attestation.
//! - Device selection.
//! - Hardware allocation.
//! - Resource reservation.
//! - Resource scheduling.
//! - Hardware routing.
//! - Quantum execution.
//! - HDL synthesis.
//! - Distributed execution.
//! - Runtime orchestration.
//! - Compiler IR.
//! - Target realization.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! # Canonical authorities
//!
//! Capability identity:
//!     crate::stdlib::capability::CapabilityId
//!
//! Provider identity:
//!     crate::stdlib::provider::ProviderId
//!
//! Hardware requirements:
//!     crate::zhal::capabilities::requirements
//!
//! Hardware evidence:
//!     crate::zhal::capabilities::evidence
//!
//! Availability:
//!     crate::zhal::capabilities::availability
//!
//! Device identity:
//!     crate::zhal::identity
//!
//! Hardware models:
//!     crate::zhal::model
//!
//! Resource feasibility and allocation:
//!     crate::zur
//!
//! Host compatibility:
//!     crate::zur::platforms::compatibility
//!
//! Quantum-specific capabilities:
//!     crate::quantum::hardware
//!
//! HDL realization:
//!     crate::hdl::realization
//!
//! # Fundamental distinctions
//!
//! Requirement != Capability.
//! Capability != Evidence.
//! Evidence != Availability.
//! Availability != Compatibility.
//! Compatibility != Feasibility.
//! Feasibility != Reservation.
//! Reservation != Execution.
//!
//! A compatible report establishes only that the evaluated mandatory
//! capability requirements are supported by the supplied evidence
//! under the selected compatibility policy.
//!
//! It does not guarantee that resources remain available or that
//! physical execution will succeed.
//!
//! # POCO-REAF
//!
//! Capability identifiers remain open-world.
//!
//! No fixed enumeration of:
//!
//! - CPUs.
//! - GPUs.
//! - NPUs.
//! - TPUs.
//! - FPGAs.
//! - ASICs.
//! - QPUs.
//! - Quantum technologies.
//! - Embedded devices.
//! - Distributed nodes.
//! - Future hardware.
//!
//! No arbitrary maximum is imposed on:
//!
//! - Requirements.
//! - Evidence records.
//! - Providers.
//! - Devices.
//! - Capability identifiers.
//! - Computational domains.
//! - Report findings.
//!
//! Actual memory and computational consumption depend on available
//! resources and explicit operational budgets.
//!
//! Literal infinite computation is not promised.
//!
//! # Determinism
//!
//! Evaluation uses:
//!
//! - Explicit requirements.
//! - Explicit evidence.
//! - An explicitly supplied timestamp.
//! - An explicitly supplied evaluation scope.
//! - An explicitly supplied policy.
//!
//! No system clock is accessed.
//!
//! No provider is queried.
//!
//! No hardware is discovered.
//!
//! Identical inputs produce identical reports.
//!
//! # Security
//!
//! Evidence verification classifications are supplied by external
//! security and provider-trust authorities.
//!
//! This module does not authenticate providers or verify signatures.
//!
//! An evidence record marked independently verified is trusted only
//! as an externally established assertion.
//!
//! # Resource ownership
//!
//! This module does not reserve resources.
//!
//! Resource feasibility and physical resource allocation remain
//! responsibilities of the semantic, planning and runtime authorities.

use core::fmt;

use std::collections::BTreeMap;

use crate::stdlib::capability::CapabilityId;
use crate::stdlib::provider::ProviderId;

use crate::zhal::identity::HardwareDeviceIdentity;

use crate::zhal::capabilities::availability::{
    assess_capability,
    AccessObservation,
    AvailabilityState,
    EvidenceCounts,
};

use crate::zhal::capabilities::evidence::{
    EvidenceFreshness,
    EvidenceTimestamp,
    EvidenceVerification,
    HardwareCapabilityEvidence,
};

use crate::zhal::capabilities::requirements::{
    HardwareCapabilityRequirement,
    HardwareCapabilityRequirementSet,
};

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

// ============================================================================
// COMPONENT IDENTIFICATION
// ============================================================================

/// Stable component identifier.
pub const COMPONENT: &str = "zhal.capabilities.compatibility";

/// Compatibility report schema version.
///
/// Independent of the language, package, provider, firmware and
/// hardware-model versions.
pub const COMPATIBILITY_SCHEMA_VERSION: &str = "1";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn compatibility_error(
    kind: ZhalErrorKind,
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(kind, message)
        .in_component(COMPONENT)
}

// ============================================================================
// EVALUATION SCOPE
// ============================================================================

/// Defines which hardware evidence is eligible for evaluation.
///
/// Hardware observations from different providers and devices must
/// not automatically be interpreted as observations about one
/// particular execution target.
///
/// The default scope is `AllEvidence`.
///
/// Use a provider or device scope when evaluating a particular
/// physical execution target.
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
)]
#[non_exhaustive]
pub enum CompatibilityScope {

    /// Evaluate all matching supplied evidence.
    ///
    /// This is useful for aggregate capability assessment.
    ///
    /// It does not establish that one physical device can satisfy
    /// all requirements simultaneously.
    #[default]
    AllEvidence,

    /// Evaluate evidence belonging to one provider.
    Provider(ProviderId),

    /// Evaluate evidence belonging to one physical device.
    Device(HardwareDeviceIdentity),
}

impl CompatibilityScope {

    /// Returns whether an evidence record belongs to this scope.
    ///
    /// Evidence without the necessary provider/device attribution
    /// is excluded from scoped evaluation.
    #[must_use]
    pub fn includes(
        &self,
        evidence: &HardwareCapabilityEvidence,
    ) -> bool {

        match self {

            Self::AllEvidence => true,

            Self::Provider(provider_id) => {
                evidence.provider_id() == Some(provider_id)
            }

            Self::Device(device_id) => {
                evidence.device() == Some(device_id)
            }
        }
    }
}

// ============================================================================
// COMPATIBILITY POLICY
// ============================================================================

/// Controls how supplied evidence is accepted during compatibility
/// evaluation.
///
/// These settings affect the acceptance of hardware evidence.
///
/// They do not alter source-language semantics, mandatory
/// requirements or physical hardware capabilities.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
)]
pub struct CompatibilityPolicy {

    /// Require independently verified evidence for every current
    /// available assertion supporting a mandatory requirement.
    ///
    /// When enabled, provider-reported or unverified availability
    /// is insufficient.
    ///
    /// The verification itself must be established externally.
    pub require_independent_verification: bool,

}

impl CompatibilityPolicy {

    /// Creates the default compatibility policy.
    ///
    /// The default accepts current availability assertions without
    /// requiring independent verification.
    ///
    /// Evidence verification classifications remain preserved
    /// and available to callers.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            require_independent_verification: false,
        }
    }

    /// Requires independently verified availability evidence.
    #[must_use]
    pub const fn independently_verified_only(
        mut self,
    ) -> Self {
        self.require_independent_verification = true;
        self
    }

}

// ============================================================================
// FINDING CLASSIFICATION
// ============================================================================

/// Classification of an individual compatibility finding.
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
pub enum CompatibilityFindingKind {

    /// Current evidence establishes the required capability.
    Satisfied,

    /// Current evidence establishes that the capability is unavailable.
    Unavailable,

    /// The capability exists conditionally.
    Conditional,

    /// No evidence establishes capability availability.
    Unknown,

    /// Available evidence is expired or not yet valid.
    Stale,

    /// The relevant hardware cannot currently be observed or accessed.
    TemporarilyInaccessible,

    /// Current evidence contains contradictory availability assertions.
    ConflictingEvidence,

    /// Current available evidence does not satisfy the required
    /// independent-verification policy.
    InsufficientVerification,

}

impl CompatibilityFindingKind {

    /// Returns whether the mandatory requirement is established.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied)
    }

    /// Returns whether the evidence explicitly establishes
    /// unavailability.
    #[must_use]
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::Unavailable)
    }

    /// Returns whether additional evidence or evaluation is needed.
    #[must_use]
    pub const fn is_indeterminate(self) -> bool {
        matches!(
            self,
            Self::Conditional
                | Self::Unknown
                | Self::Stale
                | Self::TemporarilyInaccessible
                | Self::ConflictingEvidence
                | Self::InsufficientVerification
        )
    }

    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Unavailable => "unavailable",
            Self::Conditional => "conditional",
            Self::Unknown => "unknown",
            Self::Stale => "stale",
            Self::TemporarilyInaccessible => {
                "temporarily_inaccessible"
            }
            Self::ConflictingEvidence => "conflicting_evidence",
            Self::InsufficientVerification => {
                "insufficient_verification"
            }
        }
    }
}

impl fmt::Display for CompatibilityFindingKind {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// OVERALL COMPATIBILITY STATUS
// ============================================================================

/// Overall result of hardware capability compatibility evaluation.
///
/// Compatible:
///     Every evaluated mandatory capability requirement is satisfied.
///
/// Incompatible:
///     At least one mandatory capability is explicitly unavailable.
///
/// Indeterminate:
///     Available information cannot establish compatibility or
///     explicit incompatibility.
///
/// An empty requirement collection is indeterminate because it
/// establishes no positive compatibility evidence about a workload.
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
pub enum CompatibilityStatus {

    /// Every mandatory capability requirement is established.
    Compatible,

    /// At least one mandatory capability is explicitly unavailable.
    Incompatible,

    /// Compatibility cannot currently be established.
    Indeterminate,

}

impl CompatibilityStatus {

    /// Returns whether compatibility has been established.
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        matches!(self, Self::Compatible)
    }

    /// Returns whether a mandatory capability is explicitly unavailable.
    #[must_use]
    pub const fn is_incompatible(self) -> bool {
        matches!(self, Self::Incompatible)
    }

    /// Returns whether additional evidence or evaluation is necessary.
    #[must_use]
    pub const fn is_indeterminate(self) -> bool {
        matches!(self, Self::Indeterminate)
    }

    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compatible => "compatible",
            Self::Incompatible => "incompatible",
            Self::Indeterminate => "indeterminate",
        }
    }

}

impl fmt::Display for CompatibilityStatus {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// VERIFICATION COUNTS
// ============================================================================

/// Counts verification classifications among current available
/// evidence records for one capability.
///
/// These are counts of supplied evidence records, not counts of
/// physical devices or independent verification authorities.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
)]
pub struct VerificationCounts {

    unverified: usize,

    provider_reported: usize,

    independently_verified: usize,

}

impl VerificationCounts {

    /// Returns the number of unverified available assertions.
    #[must_use]
    pub const fn unverified(self) -> usize {
        self.unverified
    }

    /// Returns the number of provider-reported available assertions.
    #[must_use]
    pub const fn provider_reported(self) -> usize {
        self.provider_reported
    }

    /// Returns the number of independently verified available assertions.
    #[must_use]
    pub const fn independently_verified(self) -> usize {
        self.independently_verified
    }

    /// Returns the total number of available assertions.
    #[must_use]
    pub fn total(self) -> Option<usize> {
        self.unverified
            .checked_add(self.provider_reported)?
            .checked_add(self.independently_verified)
    }

    /// Returns whether every available assertion is independently verified.
    #[must_use]
    pub const fn all_independently_verified(self) -> bool {
        self.total_nonzero()
            && self.unverified == 0
            && self.provider_reported == 0
    }

    const fn total_nonzero(self) -> bool {
        self.unverified > 0
            || self.provider_reported > 0
            || self.independently_verified > 0
    }

}

// ============================================================================
// COMPATIBILITY FINDING
// ============================================================================

/// Structured result for one mandatory hardware capability requirement.
///
/// Multiple requirements for the same capability remain separate
/// findings in their original insertion order.
///
/// The finding preserves the original requirement index and canonical
/// capability identity.
///
/// It does not contain a physical reservation or execution guarantee.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub struct CompatibilityFinding {

    requirement_index: usize,

    capability: CapabilityId,

    kind: CompatibilityFindingKind,

    availability: AvailabilityState,

    evidence_counts: EvidenceCounts,

    verification_counts: VerificationCounts,

}

impl CompatibilityFinding {

    /// Returns the original requirement insertion-order index.
    #[must_use]
    pub const fn requirement_index(&self) -> usize {
        self.requirement_index
    }

    /// Returns the canonical capability identity.
    #[must_use]
    pub fn capability(&self) -> &CapabilityId {
        &self.capability
    }

    /// Returns the finding classification.
    #[must_use]
    pub const fn kind(&self) -> CompatibilityFindingKind {
        self.kind
    }

    /// Returns the underlying evidence availability assessment.
    #[must_use]
    pub const fn availability(&self) -> AvailabilityState {
        self.availability
    }

    /// Returns the evidence counts considered.
    #[must_use]
    pub const fn evidence_counts(&self) -> EvidenceCounts {
        self.evidence_counts
    }

    /// Returns the verification counts.
    #[must_use]
    pub const fn verification_counts(&self) -> VerificationCounts {
        self.verification_counts
    }

    /// Returns whether the mandatory requirement is satisfied.
    #[must_use]
    pub const fn is_satisfied(&self) -> bool {
        self.kind.is_satisfied()
    }

}

// ============================================================================
// COMPATIBILITY REPORT
// ============================================================================

/// Immutable compatibility report.
///
/// Findings preserve the original requirement ordering.
///
/// The report records the exact evaluation timestamp, scope and policy.
///
/// It does not claim that physical resources have been reserved.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub struct CompatibilityReport {

    evaluated_at: EvidenceTimestamp,

    scope: CompatibilityScope,

    policy: CompatibilityPolicy,

    status: CompatibilityStatus,

    findings: Vec<CompatibilityFinding>,

}

impl CompatibilityReport {

    /// Returns the evaluation timestamp.
    #[must_use]
    pub const fn evaluated_at(&self) -> EvidenceTimestamp {
        self.evaluated_at
    }

    /// Returns the evaluation scope.
    #[must_use]
    pub fn scope(&self) -> &CompatibilityScope {
        &self.scope
    }

    /// Returns the applied compatibility policy.
    #[must_use]
    pub const fn policy(&self) -> CompatibilityPolicy {
        self.policy
    }

    /// Returns the overall compatibility status.
    #[must_use]
    pub const fn status(&self) -> CompatibilityStatus {
        self.status
    }

    /// Returns the number of evaluated requirements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.findings.len()
    }

    /// Returns whether the report contains no evaluated requirements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.findings.is_empty()
    }

    /// Returns all findings in requirement insertion order.
    #[must_use]
    pub fn findings(&self) -> &[CompatibilityFinding] {
        &self.findings
    }

    /// Returns a finding by its original requirement index.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&CompatibilityFinding> {
        self.findings.get(index)
    }

    /// Returns all findings that have not established satisfaction.
    pub fn unsatisfied(
        &self,
    ) -> impl Iterator<Item = &CompatibilityFinding> {
        self.findings
            .iter()
            .filter(|finding| !finding.is_satisfied())
    }

    /// Returns the number of satisfied requirements.
    #[must_use]
    pub fn satisfied_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.is_satisfied())
            .count()
    }

    /// Returns the number of explicitly unavailable requirements.
    #[must_use]
    pub fn unavailable_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| {
                finding.kind().is_unavailable()
            })
            .count()
    }

    /// Returns the number of indeterminate requirements.
    #[must_use]
    pub fn indeterminate_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| {
                finding.kind().is_indeterminate()
            })
            .count()
    }

}

// ============================================================================
// EVIDENCE VERIFICATION AGGREGATION
// ============================================================================

fn increment(
    value: &mut usize,
) -> ZhalResult<()> {

    *value = value.checked_add(1).ok_or_else(|| {
        compatibility_error(
            ZhalErrorKind::InvalidData,
            "compatibility evidence count exceeds the supported representation",
        )
    })?;

    Ok(())
}

fn verification_counts(
    capability: &CapabilityId,
    evidence: &[HardwareCapabilityEvidence],
    evaluated_at: EvidenceTimestamp,
    scope: &CompatibilityScope,
) -> ZhalResult<VerificationCounts> {

    let mut counts = VerificationCounts::default();

    for record in evidence {

        if record.capability() != capability {
            continue;
        }

        if !scope.includes(record) {
            continue;
        }

        if record.freshness_at(evaluated_at)
            != EvidenceFreshness::Current
        {
            continue;
        }

        // The availability evaluator has already validated the
        // matching evidence records. Revalidation protects this
        // independent public evaluation boundary as well.
        record.validate()?;

        if !matches!(
            record.status(),
            crate::stdlib::capability::CapabilityStatus::Available
        ) {
            continue;
        }

        match record.verification() {

            EvidenceVerification::Unverified => {
                increment(&mut counts.unverified)?;
            }

            EvidenceVerification::ProviderReported => {
                increment(&mut counts.provider_reported)?;
            }

            EvidenceVerification::IndependentlyVerified => {
                increment(&mut counts.independently_verified)?;
            }

        }

    }

    Ok(counts)
}

// ============================================================================
// FINDING DERIVATION
// ============================================================================

fn derive_finding_kind(
    availability: AvailabilityState,
    verification: VerificationCounts,
    policy: CompatibilityPolicy,
) -> CompatibilityFindingKind {

    match availability {

        AvailabilityState::Available => {

            if policy.require_independent_verification
                && !verification.all_independently_verified()
            {
                CompatibilityFindingKind::InsufficientVerification
            } else {
                CompatibilityFindingKind::Satisfied
            }

        }

        AvailabilityState::Unavailable => {
            CompatibilityFindingKind::Unavailable
        }

        AvailabilityState::Conditional => {
            CompatibilityFindingKind::Conditional
        }

        AvailabilityState::Unknown => {
            CompatibilityFindingKind::Unknown
        }

        AvailabilityState::Stale => {
            CompatibilityFindingKind::Stale
        }

        AvailabilityState::TemporarilyInaccessible => {
            CompatibilityFindingKind::TemporarilyInaccessible
        }

        AvailabilityState::Conflicting => {
            CompatibilityFindingKind::ConflictingEvidence
        }

        // Preserve forward compatibility with future availability
        // classifications. Unknown future states must not become
        // successful compatibility.
        _ => CompatibilityFindingKind::Unknown,

    }

}

// ============================================================================
// REQUIREMENT VALIDATION
// ============================================================================

fn validate_requirements(
    requirements: &HardwareCapabilityRequirementSet,
) -> ZhalResult<()> {

    requirements.validate().map_err(|error| {

        compatibility_error(
            ZhalErrorKind::InvalidInput,
            format!(
                "invalid hardware capability requirement: {error}"
            ),
        )

    })

}

// ============================================================================
// PUBLIC COMPATIBILITY API
// ============================================================================

/// Evaluates hardware capability compatibility.
///
/// The evaluation:
///
/// 1. Validates the requirement collection.
/// 2. Applies the explicit provider/device evidence scope.
/// 3. Evaluates each distinct required capability.
/// 4. Checks evidence freshness at the supplied timestamp.
/// 5. Detects contradictory evidence.
/// 6. Applies the evidence-verification policy.
/// 7. Produces findings in original requirement order.
/// 8. Derives the overall compatibility status.
///
/// Duplicate requirements are preserved.
///
/// No provider, device or resource is discovered or reserved.
///
/// The evaluation timestamp must be supplied by the caller.
///
/// # Errors
///
/// Returns a structured ZHAL error when:
///
/// - A requirement is invalid.
/// - Matching evidence is invalid.
/// - Evidence aggregation fails.
/// - Evidence counts exceed their representable range.
///
/// # Important
///
/// `AllEvidence` aggregates matching observations across the supplied
/// providers and devices. It does not prove that one physical device
/// can satisfy every requirement simultaneously.
///
/// Use `Provider` or `Device` scope for candidate-target evaluation.
pub fn evaluate_compatibility(
    requirements: &HardwareCapabilityRequirementSet,
    evidence: &[HardwareCapabilityEvidence],
    evaluated_at: EvidenceTimestamp,
    scope: CompatibilityScope,
    policy: CompatibilityPolicy,
) -> ZhalResult<CompatibilityReport> {

    validate_requirements(requirements)?;

    // Group evidence by canonical capability identity.
    //
    // Only capabilities that are actually required are retained.
    //
    // BTreeMap provides deterministic capability iteration.
    // Requirement findings are reconstructed in insertion order.
    //
    // Each supplied evidence record is cloned at most once into
    // its corresponding capability group.
    let mut grouped:
        BTreeMap<CapabilityId, Vec<HardwareCapabilityEvidence>> =
        BTreeMap::new();

    for requirement in requirements.iter() {

        grouped
            .entry(requirement.capability().clone())
            .or_default();

    }

    for record in evidence {

        if !scope.includes(record) {
            continue;
        }

        if let Some(group) = grouped.get_mut(record.capability()) {

            record.validate()?;

            group.push(record.clone());

        }

    }

    // Evaluate each distinct capability once.
    //
    // This avoids repeatedly scanning the complete hardware evidence
    // collection for every duplicate requirement.
    let mut assessments = BTreeMap::new();

    for (capability, records) in &grouped {

        let assessment = assess_capability(
            capability,
            records,
            evaluated_at,
            AccessObservation::Accessible,
        )?;

        let verification = verification_counts(
            capability,
            records,
            evaluated_at,
            &CompatibilityScope::AllEvidence,
        )?;

        let kind = derive_finding_kind(
            assessment.state(),
            verification,
            policy,
        );

        assessments.insert(
            capability.clone(),
            (
                assessment.state(),
                assessment.evidence_counts(),
                verification,
                kind,
            ),
        );

    }

    // Reconstruct findings in the original requirement order.
    let mut findings = Vec::with_capacity(requirements.len());

    for (index, requirement) in requirements.iter().enumerate() {

        let capability = requirement.capability();

        let (
            availability,
            evidence_counts,
            verification_counts,
            kind,
        ) = assessments
            .get(capability)
            .copied()
            .ok_or_else(|| {

                compatibility_error(
                    ZhalErrorKind::Internal,
                    "missing capability assessment during report construction",
                )

            })?;

        findings.push(CompatibilityFinding {
            requirement_index: index,
            capability: capability.clone(),
            kind,
            availability,
            evidence_counts,
            verification_counts,
        });

    }

    // Derive the overall outcome.
    //
    // An empty collection is deliberately indeterminate.
    // It does not establish that a workload is executable.
    let status = if findings.is_empty() {

        CompatibilityStatus::Indeterminate

    } else if findings.iter().any(|finding| {
        finding.kind().is_unavailable()
    }) {

        CompatibilityStatus::Incompatible

    } else if findings.iter().all(|finding| {
        finding.is_satisfied()
    }) {

        CompatibilityStatus::Compatible

    } else {

        CompatibilityStatus::Indeterminate

    };

    Ok(CompatibilityReport {
        evaluated_at,
        scope,
        policy,
        status,
        findings,
    })

}

// ============================================================================
// SINGLE-REQUIREMENT CONVENIENCE API
// ============================================================================

/// Evaluates one hardware capability requirement.
///
/// This convenience API preserves the original requirement and
/// delegates all evaluation to the canonical collection evaluator.
pub fn evaluate_requirement(
    requirement: &HardwareCapabilityRequirement,
    evidence: &[HardwareCapabilityEvidence],
    evaluated_at: EvidenceTimestamp,
    scope: CompatibilityScope,
    policy: CompatibilityPolicy,
) -> ZhalResult<CompatibilityFinding> {

    let requirements =
        HardwareCapabilityRequirementSet::new()
            .with(requirement.clone());

    let report = evaluate_compatibility(
        &requirements,
        evidence,
        evaluated_at,
        scope,
        policy,
    )?;

    report.findings()
        .first()
        .cloned()
        .ok_or_else(|| {

            compatibility_error(
                ZhalErrorKind::Internal,
                "single-requirement evaluation produced no finding",
            )

        })

}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    use crate::stdlib::capability::CapabilityStatus;

    use crate::stdlib::provider::ProviderId;

    use crate::zhal::capabilities::evidence::{
        EvidenceOrigin,
        EvidenceTimestamp,
    };

    fn capability(
        name: &str,
    ) -> CapabilityId {

        CapabilityId::new(name.to_owned())
            .expect("test capability must be valid")

    }

    fn timestamp(
        value: i128,
    ) -> EvidenceTimestamp {

        EvidenceTimestamp::from_unix_nanos(value)

    }

    fn requirement(
        name: &str,
    ) -> HardwareCapabilityRequirement {

        HardwareCapabilityRequirement::new(
            capability(name)
        )

    }

    fn evidence(
        name: &str,
        status: CapabilityStatus,
        observed_at: i128,
        valid_until: Option<i128>,
        verification: EvidenceVerification,
    ) -> HardwareCapabilityEvidence {

        let origin = EvidenceOrigin::new(
            "provider.report",
            Some("test.provider".to_owned()),
        )
        .expect("test evidence origin must be valid");

        let record = HardwareCapabilityEvidence::new(
            capability(name),
            status,
            origin,
            timestamp(observed_at),
            "test hardware provider",
            "test hardware capability observation",
        )
        .expect("test evidence must be valid");

        let record = match valid_until {

            Some(expiration) => {
                record.with_valid_until(timestamp(expiration))
                    .expect("test validity interval must be valid")
            }

            None => record,

        };

        record.with_verification(
            verification,
            if verification == EvidenceVerification::IndependentlyVerified {
                Some("test.verifier".to_owned())
            } else {
                None
            },
        )
        .expect("test verification must be valid")

    }

    fn provider(
        name: &str,
    ) -> ProviderId {

        ProviderId::new(name.to_owned())
            .expect("test provider identity must be valid")

    }

    fn requirements(
        names: &[&str],
    ) -> HardwareCapabilityRequirementSet {

        HardwareCapabilityRequirementSet::from_requirements(
            names.iter()
                .map(|name| requirement(name))
        )

    }

    fn evaluate(
        requirements: &HardwareCapabilityRequirementSet,
        records: &[HardwareCapabilityEvidence],
    ) -> CompatibilityReport {

        evaluate_compatibility(
            requirements,
            records,
            timestamp(100),
            CompatibilityScope::AllEvidence,
            CompatibilityPolicy::default(),
        )
        .expect("compatibility evaluation must succeed")

    }

    #[test]
    fn current_available_capability_is_compatible() {

        let requirements = requirements(&[
            "classical.parallel_execution",
        ]);

        let records = vec![
            evidence(
                "classical.parallel_execution",
                CapabilityStatus::Available,
                50,
                Some(150),
                EvidenceVerification::ProviderReported,
            ),
        ];

        let report = evaluate(&requirements, &records);

        assert_eq!(
            report.status(),
            CompatibilityStatus::Compatible,
        );

        assert_eq!(
            report.findings()[0].kind(),
            CompatibilityFindingKind::Satisfied,
        );

    }

    #[test]
    fn explicitly_unavailable_capability_is_incompatible() {

        let requirements = requirements(&[
            "quantum.error_correction",
        ]);

        let records = vec![
            evidence(
                "quantum.error_correction",
                CapabilityStatus::Unavailable,
                50,
                None,
                EvidenceVerification::ProviderReported,
            ),
        ];

        let report = evaluate(&requirements, &records);

        assert_eq!(
            report.status(),
            CompatibilityStatus::Incompatible,
        );

        assert_eq!(
            report.findings()[0].kind(),
            CompatibilityFindingKind::Unavailable,
        );

    }

    #[test]
    fn missing_evidence_is_indeterminate() {

        let requirements = requirements(&[
            "future.photonic.computation",
        ]);

        let report = evaluate(&requirements, &[]);

        assert_eq!(
            report.status(),
            CompatibilityStatus::Indeterminate,
        );

        assert_eq!(
            report.findings()[0].kind(),
            CompatibilityFindingKind::Unknown,
        );

    }

    #[test]
    fn expired_evidence_is_not_compatible() {

        let requirements = requirements(&[
            "classical.compute",
        ]);

        let records = vec![
            evidence(
                "classical.compute",
                CapabilityStatus::Available,
                10,
                Some(90),
                EvidenceVerification::ProviderReported,
            ),
        ];

        let report = evaluate(&requirements, &records);

        assert_eq!(
            report.status(),
            CompatibilityStatus::Indeterminate,
        );

        assert_eq!(
            report.findings()[0].kind(),
            CompatibilityFindingKind::Stale,
        );

    }

    #[test]
    fn future_evidence_is_not_compatible() {

        let requirements = requirements(&[
            "accelerator.tensor_execution",
        ]);

        let records = vec![
            evidence(
                "accelerator.tensor_execution",
                CapabilityStatus::Available,
                200,
                None,
                EvidenceVerification::ProviderReported,
            ),
        ];

        let report = evaluate(&requirements, &records);

        assert_eq!(
            report.status(),
            CompatibilityStatus::Indeterminate,
        );

        assert_eq!(
            report.findings()[0].kind(),
            CompatibilityFindingKind::Stale,
        );

    }

    #[test]
    fn contradictory_evidence_is_not_silently_accepted() {

        let requirements = requirements(&[
            "quantum.measurement",
        ]);

        let records = vec![
            evidence(
                "quantum.measurement",
                CapabilityStatus::Available,
                50,
                None,
                EvidenceVerification::ProviderReported,
            ),
            evidence(
                "quantum.measurement",
                CapabilityStatus::Unavailable,
                60,
                None,
                EvidenceVerification::ProviderReported,
            ),
        ];

        let report = evaluate(&requirements, &records);

        assert_eq!(
            report.status(),
            CompatibilityStatus::Indeterminate,
        );

        assert_eq!(
            report.findings()[0].kind(),
            CompatibilityFindingKind::ConflictingEvidence,
        );

    }

    #[test]
    fn conditional_capability_is_not_compatible() {

        let requirements = requirements(&[
            "hdl.dynamic_reconfiguration",
        ]);

        let records = vec![
            evidence(
                "hdl.dynamic_reconfiguration",
                CapabilityStatus::Conditional,
                50,
                None,
                EvidenceVerification::ProviderReported,
            ),
        ];

        let report = evaluate(&requirements, &records);

        assert_eq!(
            report.status(),
            CompatibilityStatus::Indeterminate,
        );

        assert_eq!(
            report.findings()[0].kind(),
            CompatibilityFindingKind::Conditional,
        );

    }

    #[test]
    fn duplicate_requirements_are_preserved() {

        let requirements = requirements(&[
            "classical.compute",
            "classical.compute",
        ]);

        let records = vec![
            evidence(
                "classical.compute",
                CapabilityStatus::Available,
                50,
                None,
                EvidenceVerification::ProviderReported,
            ),
        ];

        let report = evaluate(&requirements, &records);

        assert_eq!(report.len(), 2);

        assert_eq!(
            report.findings()[0].requirement_index(),
            0,
        );

        assert_eq!(
            report.findings()[1].requirement_index(),
            1,
        );

    }

    #[test]
    fn independently_verified_policy_rejects_unverified_evidence() {

        let requirements = requirements(&[
            "distributed.execution",
        ]);

        let records = vec![
            evidence(
                "distributed.execution",
                CapabilityStatus::Available,
                50,
                None,
                EvidenceVerification::ProviderReported,
            ),
        ];

        let report = evaluate_compatibility(
            &requirements,
            &records,
            timestamp(100),
            CompatibilityScope::AllEvidence,
            CompatibilityPolicy::new()
                .independently_verified_only(),
        )
        .expect("evaluation must succeed");

        assert_eq!(
            report.status(),
            CompatibilityStatus::Indeterminate,
        );

        assert_eq!(
            report.findings()[0].kind(),
            CompatibilityFindingKind::InsufficientVerification,
        );

    }

    #[test]
    fn independently_verified_evidence_satisfies_strict_policy() {

        let requirements = requirements(&[
            "future.neuromorphic.execution",
        ]);

        let records = vec![
            evidence(
                "future.neuromorphic.execution",
                CapabilityStatus::Available,
                50,
                None,
                EvidenceVerification::IndependentlyVerified,
            ),
        ];

        let report = evaluate_compatibility(
            &requirements,
            &records,
            timestamp(100),
            CompatibilityScope::AllEvidence,
            CompatibilityPolicy::new()
                .independently_verified_only(),
        )
        .expect("evaluation must succeed");

        assert_eq!(
            report.status(),
            CompatibilityStatus::Compatible,
        );

    }

    #[test]
    fn provider_scope_excludes_other_providers() {

        let requirements = requirements(&[
            "classical.compute",
        ]);

        let first = provider("provider.first");
        let second = provider("provider.second");

        let origin = EvidenceOrigin::new(
            "provider.report",
            None,
        )
        .expect("valid origin");

        let first_record = HardwareCapabilityEvidence::new(
            capability("classical.compute"),
            CapabilityStatus::Unavailable,
            origin.clone(),
            timestamp(50),
            "first provider",
            "unavailable",
        )
        .unwrap()
        .with_provider(first.clone())
        .unwrap();

        let second_record = HardwareCapabilityEvidence::new(
            capability("classical.compute"),
            CapabilityStatus::Available,
            origin,
            timestamp(50),
            "second provider",
            "available",
        )
        .unwrap()
        .with_provider(second.clone())
        .unwrap();

        let records = vec![
            first_record,
            second_record,
        ];

        let report = evaluate_compatibility(
            &requirements,
            &records,
            timestamp(100),
            CompatibilityScope::Provider(second),
            CompatibilityPolicy::default(),
        )
        .expect("scoped evaluation must succeed");

        assert_eq!(
            report.status(),
            CompatibilityStatus::Compatible,
        );

    }

    #[test]
    fn empty_requirements_are_not_a_compatibility_guarantee() {

        let requirements =
            HardwareCapabilityRequirementSet::new();

        let report = evaluate(&requirements, &[]);

        assert_eq!(
            report.status(),
            CompatibilityStatus::Indeterminate,
        );

        assert!(report.is_empty());

    }

    #[test]
    fn unknown_future_capability_names_are_supported() {

        let requirements = requirements(&[
            "future.photonic.quantum_neuromorphic.hybrid",
        ]);

        let report = evaluate(&requirements, &[]);

        assert_eq!(
            report.findings()[0].capability().as_str(),
            "future.photonic.quantum_neuromorphic.hybrid",
        );

    }

    #[test]
    fn report_preserves_evaluation_timestamp() {

        let requirements = requirements(&[
            "classical.compute",
        ]);

        let report = evaluate(&requirements, &[]);

        assert_eq!(
            report.evaluated_at(),
            timestamp(100),
        );

    }

    #[test]
    fn incompatible_requirement_takes_precedence_over_unknown() {

        let requirements = requirements(&[
            "classical.compute",
            "quantum.error_correction",
        ]);

        let records = vec![
            evidence(
                "quantum.error_correction",
                CapabilityStatus::Unavailable,
                50,
                None,
                EvidenceVerification::ProviderReported,
            ),
        ];

        let report = evaluate(&requirements, &records);

        assert_eq!(
            report.status(),
            CompatibilityStatus::Incompatible,
        );

        assert_eq!(
            report.unavailable_count(),
            1,
        );

        assert_eq!(
            report.indeterminate_count(),
            1,
        );

    }

}
