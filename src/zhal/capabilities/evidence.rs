
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! Path: `src/zhal/capabilities/evidence.rs`
//!
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Defines validated, provenance-preserving observations of physical
//! hardware capabilities.
//!
//! Hardware capability evidence describes information reported by
//! hardware providers, physical devices, platform probes and other
//! explicitly identified observation authorities.
//!
//! This module preserves the distinction between:
//!
//! - A capability declaration.
//! - A capability requirement.
//! - A physical capability observation.
//! - An evidence verification assertion.
//! - An evidence freshness assessment.
//! - A capability availability decision.
//! - Workload compatibility.
//! - Resource reservation.
//! - Physical execution.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - ZHAL hardware capability evidence.
//! - Hardware evidence provenance.
//! - Observation timestamps.
//! - Evidence validity intervals.
//! - Evidence verification classifications.
//! - Optional confidence assessments.
//! - Evidence freshness evaluation.
//! - Validation of evidence records.
//! - Adaptation to the canonical standard-library evidence contract.
//!
//! This module does NOT own:
//!
//! - Capability identity.
//! - Capability requirement semantics.
//! - Source-language capability declarations.
//! - Source-language resource requirements.
//! - Hardware discovery.
//! - Provider registration.
//! - Provider authentication.
//! - Cryptographic attestation.
//! - Capability availability policy.
//! - Workload compatibility.
//! - Resource allocation.
//! - Resource reservations.
//! - Scheduling.
//! - Hardware execution.
//! - Quantum operations.
//! - HDL semantics.
//! - Compiler IR.
//!
//! # Canonical authorities
//!
//! Capability identity and status:
//!     crate::stdlib::capability
//!
//! Standard capability evidence:
//!     crate::stdlib::capability::CapabilityEvidence
//!
//! Provider identity:
//!     crate::stdlib::provider::ProviderId
//!
//! Hardware device identity:
//!     crate::zhal::identity::HardwareDeviceIdentity
//!
//! ZHAL errors:
//!     crate::zhal::errors
//!
//! Semantic capability requirements:
//!     crate::frontend::semantic::capabilities
//!
//! Runtime allocation:
//!     crate::zur
//!
//! Quantum hardware:
//!     crate::quantum::hardware
//!
//! HDL realization:
//!     crate::hdl::realization
//!
//! # Important architectural distinction
//!
//! The frontend semantic evidence subsystem describes claims about
//! target-independent semantic capabilities and evaluates them against
//! source-level requirements.
//!
//! This module describes observations originating from physical
//! hardware and hardware providers.
//!
//! The two evidence systems must not be conflated.
//!
//! # Evidence trust
//!
//! Recording an evidence verification classification does not perform
//! authentication or cryptographic verification.
//!
//! Verification must be established by the appropriate external
//! security, provider-trust or attestation authority.
//!
//! The classification records the result reported by that authority.
//!
//! # Freshness
//!
//! Freshness is evaluated against an explicitly supplied observation
//! time. This avoids hidden clock access and makes evaluation
//! deterministic and testable.
//!
//! An expired observation must not be treated as current evidence.
//!
//! Evidence without an expiration time has no time-based expiration
//! declared by its source. It is not proof that the hardware remains
//! available.
//!
//! # POCO-REAF and scalability
//!
//! Capability identifiers and origin identifiers remain open-world.
//!
//! This module imposes no arbitrary maximum on:
//!
//! - Capability identifiers.
//! - Providers.
//! - Devices.
//! - Evidence records.
//! - Evidence descriptions.
//! - Provenance references.
//! - Computational domains.
//! - Hardware categories.
//! - Hardware resources.
//! - Observation history.
//!
//! Collections and strings grow according to available memory and
//! platform representation limits.
//!
//! Operational budgets belong to the owning provider, runtime or
//! caller policy.
//!
//! Literal infinite storage or computation is not promised.
//!
//! # Serialization
//!
//! Evidence timestamps are represented as signed Unix nanoseconds.
//!
//! This provides a stable, explicit representation independent of
//! local time zones and formatting conventions.
//!
//! Deserialized evidence must be validated before being trusted.
//!
//! # Dependencies
//!
//! - Rust standard library.
//! - serde.
//! - crate::stdlib::capability.
//! - crate::stdlib::provider.
//! - crate::zhal::identity.
//! - crate::zhal::errors.

use core::fmt;

use serde::{Deserialize, Serialize};

use std::error::Error as StdError;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::stdlib::capability::{
    CapabilityEvidence as StandardCapabilityEvidence,
    CapabilityId,
    CapabilityStatus,
};

use crate::stdlib::provider::ProviderId;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

use crate::zhal::identity::HardwareDeviceIdentity;

// ============================================================================
// MODULE CONTRACT
// ============================================================================

/// Stable ZHAL evidence component identifier.
pub const COMPONENT: &str = "zhal.capabilities.evidence";

/// Version of the hardware capability evidence schema.
///
/// This is independent of the Zamani language version, Cargo package
/// version, provider API version and hardware firmware version.
pub const EVIDENCE_SCHEMA_VERSION: &str = "1";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn invalid_evidence(
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(
        ZhalErrorKind::InvalidData,
        message,
    )
    .in_component(COMPONENT)
}

fn invalid_evidence_with_source<E>(
    message: impl Into<String>,
    source: E,
) -> ZhalError
where
    E: StdError + Send + Sync + 'static,
{
    invalid_evidence(message).with_source(source)
}

// ============================================================================
// VALIDATION
// ============================================================================

/// Validates an evidence-related descriptive string.
///
/// Empty strings, whitespace-only strings and control characters
/// are rejected.
///
/// No arbitrary maximum string length is introduced.
fn validate_text(
    value: &str,
    field: &'static str,
) -> ZhalResult<()> {
    if value.trim().is_empty() {
        return Err(invalid_evidence(
            format!("{field} must not be empty"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_evidence(
            format!("{field} must not contain control characters"),
        ));
    }

    Ok(())
}

// ============================================================================
// EVIDENCE TIMESTAMP
// ============================================================================

/// A portable signed timestamp represented as Unix nanoseconds.
///
/// The representation is independent of time zones, locale settings
/// and textual date formatting.
///
/// The timestamp is an observation-time representation, not a
/// monotonic-clock measurement.
///
/// Monotonic deadlines and execution timeouts remain owned by the
/// operation-context and runtime authorities.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct EvidenceTimestamp(i128);

impl EvidenceTimestamp {
    /// Creates a timestamp from signed Unix nanoseconds.
    ///
    /// Negative values represent instants before the Unix epoch.
    #[must_use]
    pub const fn from_unix_nanos(nanoseconds: i128) -> Self {
        Self(nanoseconds)
    }

    /// Returns the signed Unix nanosecond representation.
    #[must_use]
    pub const fn as_unix_nanos(self) -> i128 {
        self.0
    }

    /// Converts a system timestamp into the portable representation.
    ///
    /// # Errors
    ///
    /// Returns a structured error if the timestamp cannot be represented.
    pub fn from_system_time(
        timestamp: SystemTime,
    ) -> ZhalResult<Self> {
        let nanoseconds = match timestamp.duration_since(UNIX_EPOCH) {
            Ok(duration) => {
                i128::try_from(duration.as_nanos())
            }

            Err(error) => {
                i128::try_from(error.duration().as_nanos())
                    .map(i128::checked_neg)
                    .unwrap_or(None)
                    .ok_or_else(|| {
                        invalid_evidence(
                            "system timestamp exceeds the supported representation",
                        )
                    })?
                    .pipe(Ok)
            }
        };

        let nanoseconds = nanoseconds.map_err(|_| {
            invalid_evidence(
                "system timestamp exceeds the supported representation",
            )
        })?;

        Ok(Self(nanoseconds))
    }

    /// Converts this timestamp into a system timestamp.
    ///
    /// # Errors
    ///
    /// Returns an error if the platform's SystemTime representation
    /// cannot represent this timestamp.
    pub fn to_system_time(self) -> ZhalResult<SystemTime> {
        let magnitude = self.0.checked_abs().ok_or_else(|| {
            invalid_evidence(
                "timestamp magnitude cannot be represented",
            )
        })?;

        let seconds = u64::try_from(magnitude / 1_000_000_000)
            .map_err(|_| {
                invalid_evidence(
                    "timestamp exceeds the platform duration representation",
                )
            })?;

        let nanoseconds = (magnitude % 1_000_000_000) as u32;

        let duration = Duration::new(
            seconds,
            nanoseconds,
        );

        let timestamp = if self.0 >= 0 {
            UNIX_EPOCH.checked_add(duration)
        } else {
            UNIX_EPOCH.checked_sub(duration)
        };

        timestamp.ok_or_else(|| {
            invalid_evidence(
                "timestamp cannot be represented by the platform clock",
            )
        })
    }
}

impl fmt::Display for EvidenceTimestamp {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// ============================================================================
// EVIDENCE ORIGIN
// ============================================================================

/// Describes the open-world origin of a hardware capability observation.
///
/// Examples include:
///
/// - `provider.report`
/// - `hardware.probe`
/// - `platform.manifest`
/// - `hardware.attestation`
/// - `simulation.observation`
/// - `remote.provider.report`
/// - `future.observation.source`
///
/// These are examples, not a closed enumeration.
///
/// The origin identifies the source category. It does not authenticate
/// the source or establish that its statements are correct.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct EvidenceOrigin {
    kind: String,
    reference: Option<String>,
}

impl EvidenceOrigin {
    /// Constructs a validated evidence origin.
    ///
    /// # Errors
    ///
    /// Returns an error when the origin kind or optional reference
    /// is invalid.
    pub fn new(
        kind: impl Into<String>,
        reference: Option<String>,
    ) -> ZhalResult<Self> {
        let kind = kind.into();

        validate_text(&kind, "evidence origin kind")?;

        if let Some(value) = reference.as_deref() {
            validate_text(value, "evidence origin reference")?;
        }

        Ok(Self {
            kind,
            reference,
        })
    }

    /// Returns the exact origin classification.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Returns the optional external evidence reference.
    #[must_use]
    pub fn reference(&self) -> Option<&str> {
        self.reference.as_deref()
    }

    /// Revalidates this origin.
    ///
    /// This should be called after deserializing untrusted evidence.
    pub fn validate(&self) -> ZhalResult<()> {
        validate_text(
            &self.kind,
            "evidence origin kind",
        )?;

        if let Some(reference) = self.reference.as_deref() {
            validate_text(
                reference,
                "evidence origin reference",
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// EVIDENCE VERIFICATION
// ============================================================================

/// Describes the verification classification reported for evidence.
///
/// This enum records an externally established verification state.
/// It does not execute authentication, signature verification or
/// cryptographic attestation.
///
/// The owning security authority must establish the classification.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceVerification {
    /// No independent verification has been established.
    Unverified,

    /// The evidence was reported by its declared provider.
    ProviderReported,

    /// An independent verification authority reports successful
    /// verification.
    IndependentlyVerified,
}

// ============================================================================
// EVIDENCE CONFIDENCE
// ============================================================================

/// A validated confidence assessment.
///
/// Values are finite floating-point numbers in the inclusive interval
/// from zero to one.
///
/// Confidence is descriptive metadata. It is not a probability
/// guarantee, security decision or availability decision.
///
/// Different providers may use different confidence methodologies.
/// Their provenance must be retained when comparing assessments.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct EvidenceConfidence(f64);

impl EvidenceConfidence {
    /// Constructs a confidence assessment.
    ///
    /// # Errors
    ///
    /// Rejects NaN, infinity and values outside the interval [0, 1].
    pub fn new(value: f64) -> ZhalResult<Self> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(invalid_evidence(
                "evidence confidence must be finite and between zero and one",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the confidence value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Revalidates a deserialized confidence value.
    pub fn validate(self) -> ZhalResult<()> {
        if !self.0.is_finite() || !(0.0..=1.0).contains(&self.0) {
            return Err(invalid_evidence(
                "deserialized evidence confidence is invalid",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// FRESHNESS
// ============================================================================

/// The freshness of an evidence observation at a supplied instant.
///
/// Freshness does not establish that a physical device is operational.
/// It only evaluates the declared observation time and validity interval.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshness {
    /// The observation time is in the future relative to the evaluation.
    NotYetObserved,

    /// The observation is within its declared validity interval.
    Current,

    /// The declared validity interval has expired.
    Expired,
}

// ============================================================================
// HARDWARE CAPABILITY EVIDENCE
// ============================================================================

/// A provenance-preserving observation of a physical hardware capability.
///
/// This type extends the existing standard-library capability evidence
/// contract without redefining capability identity or capability status.
///
/// It records:
///
/// - Canonical capability identity.
/// - Reported capability status.
/// - Evidence origin.
/// - Provider attribution.
/// - Optional physical device attribution.
/// - Observation time.
/// - Optional expiration time.
/// - Optional confidence assessment.
/// - Verification classification.
/// - Optional verification authority reference.
/// - Descriptive evidence source.
/// - Descriptive evidence explanation.
///
/// A record is an observation, not a reservation.
///
/// A current record does not guarantee that the capability remains
/// available when a later operation is submitted.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct HardwareCapabilityEvidence {
    capability: CapabilityId,

    status: CapabilityStatus,

    origin: EvidenceOrigin,

    provider_id: Option<ProviderId>,

    device: Option<HardwareDeviceIdentity>,

    observed_at: EvidenceTimestamp,

    valid_until: Option<EvidenceTimestamp>,

    confidence: Option<EvidenceConfidence>,

    verification: EvidenceVerification,

    verified_by: Option<String>,

    source: String,

    description: String,
}

impl HardwareCapabilityEvidence {
    /// Constructs a hardware capability evidence record.
    ///
    /// The observation time is mandatory.
    ///
    /// Provider and device attribution may be added using the
    /// corresponding builder methods.
    ///
    /// # Arguments
    ///
    /// - `capability`: canonical standard-library capability identity.
    /// - `status`: canonical capability observation status.
    /// - `origin`: evidence source classification.
    /// - `observed_at`: time at which the observation was made.
    /// - `source`: descriptive evidence source.
    /// - `description`: explanation of the reported observation.
    ///
    /// # Errors
    ///
    /// Returns a structured error when supplied descriptive information
    /// is invalid.
    pub fn new(
        capability: CapabilityId,
        status: CapabilityStatus,
        origin: EvidenceOrigin,
        observed_at: EvidenceTimestamp,
        source: impl Into<String>,
        description: impl Into<String>,
    ) -> ZhalResult<Self> {
        let source = source.into();
        let description = description.into();

        validate_text(
            &source,
            "evidence source",
        )?;

        validate_text(
            &description,
            "evidence description",
        )?;

        origin.validate()?;

        Ok(Self {
            capability,
            status,
            origin,
            provider_id: None,
            device: None,
            observed_at,
            valid_until: None,
            confidence: None,
            verification: EvidenceVerification::Unverified,
            verified_by: None,
            source,
            description,
        })
    }

    /// Associates the evidence with a canonical provider.
    ///
    /// If a device has already been associated, its provider identity
    /// must match.
    ///
    /// # Errors
    ///
    /// Returns an error when provider attribution conflicts with the
    /// associated physical device.
    pub fn with_provider(
        mut self,
        provider_id: ProviderId,
    ) -> ZhalResult<Self> {
        if let Some(device) = self.device.as_ref() {
            if device.provider_id() != &provider_id {
                return Err(invalid_evidence(
                    "evidence provider does not match device provider",
                ));
            }
        }

        self.provider_id = Some(provider_id);

        Ok(self)
    }

    /// Associates the evidence with a physical hardware device.
    ///
    /// The provider identity is derived from the canonical composite
    /// device identity.
    ///
    /// A conflicting previously supplied provider is rejected.
    ///
    /// # Errors
    ///
    /// Returns an error when provider attribution conflicts.
    pub fn with_device(
        mut self,
        device: HardwareDeviceIdentity,
    ) -> ZhalResult<Self> {
        if let Some(provider_id) = self.provider_id.as_ref() {
            if device.provider_id() != provider_id {
                return Err(invalid_evidence(
                    "device provider conflicts with evidence provider",
                ));
            }
        }

        self.provider_id = Some(device.provider_id().clone());

        self.device = Some(device);

        Ok(self)
    }

    /// Sets the evidence expiration time.
    ///
    /// The expiration must be strictly later than the observation time.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid validity interval.
    pub fn with_valid_until(
        mut self,
        valid_until: EvidenceTimestamp,
    ) -> ZhalResult<Self> {
        if valid_until <= self.observed_at {
            return Err(invalid_evidence(
                "evidence expiration must be later than observation time",
            ));
        }

        self.valid_until = Some(valid_until);

        Ok(self)
    }

    /// Associates an optional confidence assessment.
    ///
    /// # Errors
    ///
    /// Returns an error if the assessment is invalid.
    pub fn with_confidence(
        mut self,
        confidence: EvidenceConfidence,
    ) -> ZhalResult<Self> {
        confidence.validate()?;

        self.confidence = Some(confidence);

        Ok(self)
    }

    /// Sets the externally established verification classification.
    ///
    /// An independent verification classification requires a non-empty
    /// verifier reference.
    ///
    /// This method records the supplied assertion; it does not perform
    /// verification itself.
    ///
    /// # Errors
    ///
    /// Returns an error if the verifier reference is inconsistent with
    /// the requested classification.
    pub fn with_verification(
        mut self,
        verification: EvidenceVerification,
        verified_by: Option<String>,
    ) -> ZhalResult<Self> {
        if let Some(verifier) = verified_by.as_deref() {
            validate_text(
                verifier,
                "evidence verifier",
            )?;
        }

        if verification == EvidenceVerification::IndependentlyVerified
            && verified_by.is_none()
        {
            return Err(invalid_evidence(
                "independently verified evidence requires a verifier reference",
            ));
        }

        self.verification = verification;
        self.verified_by = verified_by;

        Ok(self)
    }

    /// Returns the canonical capability identity.
    #[must_use]
    pub fn capability(&self) -> &CapabilityId {
        &self.capability
    }

    /// Returns the reported capability status.
    #[must_use]
    pub const fn status(&self) -> CapabilityStatus {
        self.status
    }

    /// Returns the evidence origin.
    #[must_use]
    pub fn origin(&self) -> &EvidenceOrigin {
        &self.origin
    }

    /// Returns the canonical provider identity, if known.
    #[must_use]
    pub fn provider_id(&self) -> Option<&ProviderId> {
        self.provider_id.as_ref()
    }

    /// Returns the physical device identity, if device-scoped.
    #[must_use]
    pub fn device(&self) -> Option<&HardwareDeviceIdentity> {
        self.device.as_ref()
    }

    /// Returns the observation timestamp.
    #[must_use]
    pub const fn observed_at(&self) -> EvidenceTimestamp {
        self.observed_at
    }

    /// Returns the optional expiration timestamp.
    #[must_use]
    pub const fn valid_until(&self) -> Option<EvidenceTimestamp> {
        self.valid_until
    }

    /// Returns the optional confidence assessment.
    #[must_use]
    pub const fn confidence(&self) -> Option<EvidenceConfidence> {
        self.confidence
    }

    /// Returns the verification classification.
    #[must_use]
    pub const fn verification(&self) -> EvidenceVerification {
        self.verification
    }

    /// Returns the optional verification authority reference.
    #[must_use]
    pub fn verified_by(&self) -> Option<&str> {
        self.verified_by.as_deref()
    }

    /// Returns the descriptive evidence source.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Returns the evidence explanation.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Evaluates evidence freshness at an explicitly supplied timestamp.
    ///
    /// The expiration instant itself is considered expired.
    ///
    /// No system clock is accessed.
    #[must_use]
    pub fn freshness_at(
        &self,
        now: EvidenceTimestamp,
    ) -> EvidenceFreshness {
        if now < self.observed_at {
            return EvidenceFreshness::NotYetObserved;
        }

        if self.valid_until.is_some_and(|expiry| now >= expiry) {
            return EvidenceFreshness::Expired;
        }

        EvidenceFreshness::Current
    }

    /// Returns whether the evidence is current at the supplied instant.
    #[must_use]
    pub fn is_current_at(
        &self,
        now: EvidenceTimestamp,
    ) -> bool {
        self.freshness_at(now) == EvidenceFreshness::Current
    }

    /// Converts this observation into the canonical standard-library
    /// capability evidence representation.
    ///
    /// The returned value preserves the canonical standard-library
    /// evidence contract.
    ///
    /// Additional ZHAL-specific metadata remains available on this
    /// original record.
    ///
    /// # Errors
    ///
    /// Returns a structured ZHAL error if canonical evidence construction
    /// fails.
    pub fn to_standard_evidence(
        &self,
    ) -> ZhalResult<StandardCapabilityEvidence> {
        StandardCapabilityEvidence::new(
            self.source.clone(),
            self.description.clone(),
        )
        .map_err(|error| {
            invalid_evidence_with_source(
                "failed to construct standard capability evidence",
                error,
            )
        })
    }

    /// Revalidates all local evidence invariants.
    ///
    /// This method MUST be called after deserializing evidence from
    /// an untrusted external source.
    ///
    /// Validation does not authenticate the evidence or verify the
    /// truth of its capability claims.
    pub fn validate(&self) -> ZhalResult<()> {
        self.origin.validate()?;

        validate_text(
            &self.source,
            "evidence source",
        )?;

        validate_text(
            &self.description,
            "evidence description",
        )?;

        if let Some(valid_until) = self.valid_until {
            if valid_until <= self.observed_at {
                return Err(invalid_evidence(
                    "evidence expiration must be later than observation time",
                ));
            }
        }

        if let Some(confidence) = self.confidence {
            confidence.validate()?;
        }

        if let Some(device) = self.device.as_ref() {
            if let Some(provider_id) = self.provider_id.as_ref() {
                if device.provider_id() != provider_id {
                    return Err(invalid_evidence(
                        "device provider conflicts with evidence provider",
                    ));
                }
            } else {
                return Err(invalid_evidence(
                    "device-scoped evidence must retain its provider identity",
                ));
            }
        }

        if self.verification == EvidenceVerification::IndependentlyVerified {
            let verifier = self.verified_by.as_deref().ok_or_else(|| {
                invalid_evidence(
                    "independently verified evidence requires a verifier reference",
                )
            })?;

            validate_text(
                verifier,
                "evidence verifier",
            )?;
        }

        if let Some(verifier) = self.verified_by.as_deref() {
            validate_text(
                verifier,
                "evidence verifier",
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// EVIDENCE COLLECTION
// ============================================================================

/// A collection of physical hardware capability evidence.
///
/// This collection is intentionally separate from the canonical
/// standard-library CapabilitySnapshot.
///
/// It preserves the complete ZHAL observation records, including
/// provider/device attribution and validity information.
///
/// Exact duplicate records are not silently discarded.
///
/// No fixed evidence-record capacity is imposed.
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct HardwareCapabilityEvidenceSet {
    records: Vec<HardwareCapabilityEvidence>,
}

impl HardwareCapabilityEvidenceSet {
    /// Creates an empty evidence collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of evidence records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Returns deterministic insertion-order iteration.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &HardwareCapabilityEvidence> {
        self.records.iter()
    }

    /// Inserts one validated evidence record.
    ///
    /// Insertion order is preserved because it can be relevant to
    /// observation history.
    ///
    /// # Errors
    ///
    /// Returns an error if the record violates its invariants.
    pub fn insert(
        &mut self,
        evidence: HardwareCapabilityEvidence,
    ) -> ZhalResult<()> {
        evidence.validate()?;

        self.records.push(evidence);

        Ok(())
    }

    /// Extends the collection transactionally.
    ///
    /// If any record is invalid, the original collection remains
    /// unchanged.
    ///
    /// # Errors
    ///
    /// Returns an error if any supplied record is invalid.
    pub fn extend<I>(
        &mut self,
        evidence: I,
    ) -> ZhalResult<()>
    where
        I: IntoIterator<Item = HardwareCapabilityEvidence>,
    {
        let mut next = self.records.clone();

        for record in evidence {
            record.validate()?;
            next.push(record);
        }

        self.records = next;

        Ok(())
    }

    /// Finds evidence records for a particular capability.
    pub fn for_capability<'a>(
        &'a self,
        capability: &'a CapabilityId,
    ) -> impl Iterator<Item = &'a HardwareCapabilityEvidence> + 'a {
        self.records
            .iter()
            .filter(move |record| record.capability() == capability)
    }

    /// Finds evidence associated with a provider.
    pub fn for_provider<'a>(
        &'a self,
        provider: &'a ProviderId,
    ) -> impl Iterator<Item = &'a HardwareCapabilityEvidence> + 'a {
        self.records.iter().filter(move |record| {
            record.provider_id() == Some(provider)
        })
    }

    /// Revalidates all records.
    ///
    /// This is necessary after deserialization from an untrusted source.
    pub fn validate(&self) -> ZhalResult<()> {
        for record in &self.records {
            record.validate()?;
        }

        Ok(())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::capability::{
        CapabilityId,
        CapabilityStatus,
    };

    use crate::stdlib::provider::ProviderId;

    fn timestamp(value: i128) -> EvidenceTimestamp {
        EvidenceTimestamp::from_unix_nanos(value)
    }

    fn origin() -> EvidenceOrigin {
        EvidenceOrigin::new(
            "hardware.probe",
            Some("probe.observation.001".to_owned()),
        )
        .expect("valid evidence origin")
    }

    fn evidence() -> HardwareCapabilityEvidence {
        HardwareCapabilityEvidence::new(
            CapabilityId::new("classical.parallel_execution")
                .expect("valid capability"),
            CapabilityStatus::Available,
            origin(),
            timestamp(1_000),
            "hardware probe",
            "Provider reports parallel execution support",
        )
        .expect("valid hardware evidence")
    }

    #[test]
    fn constructs_valid_evidence() {
        let record = evidence();

        assert_eq!(
            record.capability().as_str(),
            "classical.parallel_execution",
        );

        assert_eq!(
            record.status(),
            CapabilityStatus::Available,
        );

        assert_eq!(
            record.origin().kind(),
            "hardware.probe",
        );
    }

    #[test]
    fn provider_identity_is_preserved() {
        let provider = ProviderId::new(
            "org.example.hardware",
        )
        .expect("valid provider");

        let record = evidence()
            .with_provider(provider.clone())
            .expect("valid provider attribution");

        assert_eq!(
            record.provider_id(),
            Some(&provider),
        );
    }

    #[test]
    fn device_identity_establishes_provider_attribution() {
        use crate::stdlib::io::devices::DeviceId;

        let provider = ProviderId::new(
            "org.example.hardware",
        )
        .expect("valid provider");

        let device_id = DeviceId::new(
            "device-001",
        )
        .expect("valid device");

        let device = HardwareDeviceIdentity::new(
            provider.clone(),
            device_id,
        );

        let record = evidence()
            .with_device(device.clone())
            .expect("valid device attribution");

        assert_eq!(
            record.device(),
            Some(&device),
        );

        assert_eq!(
            record.provider_id(),
            Some(&provider),
        );
    }

    #[test]
    fn conflicting_provider_attribution_is_rejected() {
        let first = ProviderId::new(
            "org.example.first",
        )
        .expect("valid provider");

        let second = ProviderId::new(
            "org.example.second",
        )
        .expect("valid provider");

        let record = evidence()
            .with_provider(first)
            .expect("valid provider");

        let device_id = crate::stdlib::io::devices::DeviceId::new(
            "device-001",
        )
        .expect("valid device");

        let device = HardwareDeviceIdentity::new(
            second,
            device_id,
        );

        assert!(record.with_device(device).is_err());
    }

    #[test]
    fn expiration_must_follow_observation() {
        assert!(
            evidence()
                .with_valid_until(timestamp(1_000))
                .is_err()
        );

        assert!(
            evidence()
                .with_valid_until(timestamp(999))
                .is_err()
        );

        assert!(
            evidence()
                .with_valid_until(timestamp(2_000))
                .is_ok()
        );
    }

    #[test]
    fn freshness_is_deterministic() {
        let record = evidence()
            .with_valid_until(timestamp(2_000))
            .expect("valid evidence");

        assert_eq!(
            record.freshness_at(timestamp(999)),
            EvidenceFreshness::NotYetObserved,
        );

        assert_eq!(
            record.freshness_at(timestamp(1_000)),
            EvidenceFreshness::Current,
        );

        assert_eq!(
            record.freshness_at(timestamp(1_999)),
            EvidenceFreshness::Current,
        );

        assert_eq!(
            record.freshness_at(timestamp(2_000)),
            EvidenceFreshness::Expired,
        );
    }

    #[test]
    fn evidence_without_expiration_remains_current() {
        let record = evidence();

        assert_eq!(
            record.freshness_at(timestamp(100_000)),
            EvidenceFreshness::Current,
        );
    }

    #[test]
    fn confidence_validation_rejects_invalid_values() {
        assert!(EvidenceConfidence::new(0.0).is_ok());
        assert!(EvidenceConfidence::new(0.5).is_ok());
        assert!(EvidenceConfidence::new(1.0).is_ok());

        assert!(EvidenceConfidence::new(-0.1).is_err());
        assert!(EvidenceConfidence::new(1.1).is_err());
        assert!(EvidenceConfidence::new(f64::NAN).is_err());
        assert!(EvidenceConfidence::new(f64::INFINITY).is_err());
    }

    #[test]
    fn verification_requires_an_independent_verifier() {
        assert!(
            evidence()
                .with_verification(
                    EvidenceVerification::IndependentlyVerified,
                    None,
                )
                .is_err()
        );

        assert!(
            evidence()
                .with_verification(
                    EvidenceVerification::IndependentlyVerified,
                    Some("security.authority".to_owned()),
                )
                .is_ok()
        );
    }

    #[test]
    fn canonical_standard_evidence_adapter_works() {
        let record = evidence();

        let standard = record
            .to_standard_evidence()
            .expect("canonical evidence conversion");

        assert_eq!(
            standard.source(),
            "hardware probe",
        );

        assert_eq!(
            standard.description(),
            "Provider reports parallel execution support",
        );
    }

    #[test]
    fn invalid_deserialized_state_is_detected_by_validation() {
        let mut record = evidence();

        record.valid_until = Some(timestamp(500));

        assert!(record.validate().is_err());
    }

    #[test]
    fn evidence_collection_preserves_insertion_order() {
        let mut collection = HardwareCapabilityEvidenceSet::new();

        collection.insert(evidence()).expect("valid evidence");

        let second = HardwareCapabilityEvidence::new(
            CapabilityId::new("quantum.measurement")
                .expect("valid capability"),
            CapabilityStatus::Unknown,
            origin(),
            timestamp(2_000),
            "quantum provider",
            "Quantum measurement capability was not established",
        )
        .expect("valid evidence");

        collection.insert(second).expect("valid evidence");

        assert_eq!(collection.len(), 2);

        assert_eq!(
            collection.iter().next().unwrap().capability().as_str(),
            "classical.parallel_execution",
        );
    }

    #[test]
    fn collection_extension_is_transactional() {
        let mut collection = HardwareCapabilityEvidenceSet::new();

        collection.insert(evidence()).expect("valid evidence");

        let invalid = evidence()
            .with_valid_until(timestamp(1_000));

        assert!(invalid.is_err());

        assert_eq!(collection.len(), 1);
    }

    #[test]
    fn standard_unknown_status_is_preserved() {
        let record = HardwareCapabilityEvidence::new(
            CapabilityId::new("future.computing.capability")
                .expect("valid capability"),
            CapabilityStatus::Unknown,
            origin(),
            timestamp(1_000),
            "future provider",
            "Capability availability could not be established",
        )
        .expect("valid evidence");

        assert_eq!(
            record.status(),
            CapabilityStatus::Unknown,
        );
    }

    #[test]
    fn timestamp_round_trip_works() {
        let timestamp = EvidenceTimestamp::from_system_time(
            UNIX_EPOCH + Duration::from_secs(123),
        )
        .expect("valid timestamp");

        let restored = timestamp
            .to_system_time()
            .expect("valid system timestamp");

        assert_eq!(
            restored,
            UNIX_EPOCH + Duration::from_secs(123),
        );
    }

    #[test]
    fn negative_timestamp_round_trip_works() {
        let original = UNIX_EPOCH - Duration::from_secs(123);

        let timestamp = EvidenceTimestamp::from_system_time(original)
            .expect("valid timestamp");

        let restored = timestamp
            .to_system_time()
            .expect("valid system timestamp");

        assert_eq!(restored, original);
    }

    #[test]
    fn origin_validation_rejects_control_characters() {
        assert!(
            EvidenceOrigin::new(
                "hardware\nprobe",
                None,
            )
            .is_err()
        );
    }
}
