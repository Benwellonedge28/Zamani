
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! Provider Trust and Verification Boundary.
//!
//! Path: `src/zhal/security/trust.rs`
//!
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the provider-trust evidence and assessment boundary
//! for the Zamani Universal Hardware Abstraction Layer.
//!
//! The module establishes whether evidence about a hardware
//! provider satisfies an explicitly supplied trust policy.
//!
//! It supports providers associated with:
//!
//! - Classical computing.
//! - Quantum computing.
//! - HDL and hardware realization.
//! - Accelerators.
//! - Embedded computing.
//! - AI and tensor computing.
//! - Hybrid computing.
//! - Distributed computing.
//! - Networking.
//! - Future computational technologies.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Provider trust evidence.
//! - Trust policy descriptions.
//! - Trust issuer references.
//! - Signature-verification integration.
//! - Revocation-check integration.
//! - Evidence validity evaluation.
//! - Evidence freshness evaluation.
//! - Provider identity binding.
//! - Structured trust assessments.
//! - Fail-closed trust decisions.
//!
//! This module does NOT own:
//!
//! - Provider identity definitions.
//! - Provider registration.
//! - Provider discovery.
//! - Authentication.
//! - Authorization.
//! - Credential storage.
//! - Private-key management.
//! - Cryptographic algorithm implementations.
//! - Certificate authority management.
//! - Trust-anchor persistence.
//! - Runtime isolation.
//! - Hardware selection.
//! - Physical resource allocation.
//! - Scheduling.
//! - Routing.
//! - Quantum execution.
//! - HDL realization.
//! - Distributed execution.
//! - Source-language trust semantics.
//!
//! # Canonical repository authorities
//!
//! Provider identity:
//!     crate::stdlib::provider::ProviderId
//!
//! ZHAL errors:
//!     crate::zhal::errors
//!
//! ZHAL hardware identity:
//!     crate::zhal::identity
//!
//! Hardware capability evidence:
//!     crate::zhal::capabilities::evidence
//!
//! Provider admission validation:
//!     crate::zhal::providers::validation
//!
//! Runtime security:
//!     crate::zur::security
//!
//! Compiler supply-chain verification:
//!     crate::compiler::security
//!
//! Source-level trust intent:
//!     grammar/security/trust.g4
//!
//! # Security invariants
//!
//! 1. Provider registration does not establish trust.
//! 2. Provider identity does not establish trust.
//! 3. Provider metadata does not establish trust.
//! 4. Provider self-attestation does not independently establish trust.
//! 5. Missing verification is never interpreted as successful verification.
//! 6. Unknown revocation status is never interpreted as non-revocation.
//! 7. Expired evidence cannot establish current trust.
//! 8. Evidence for one provider cannot establish trust for another.
//! 9. Invalid signatures cannot establish trust.
//! 10. Verification errors cannot establish trust.
//! 11. Trust does not imply authorization.
//! 12. Trust does not imply isolation.
//! 13. Trust does not imply hardware availability.
//! 14. Trust assessment does not authorize execution.
//! 15. Cryptographic operations are delegated to explicit authorities.
//!
//! # POCO-REAF and scalability
//!
//! Trust identifiers are open-world.
//!
//! This module imposes no arbitrary maximum on:
//!
//! - Trust policies.
//! - Trust issuers.
//! - Providers.
//! - Evidence records.
//! - Signature schemes.
//! - Verification references.
//! - Revocation authorities.
//! - Computational domains.
//! - Hardware technologies.
//!
//! Collections grow according to available memory and platform
//! representation limits.
//!
//! Explicit operational budgets belong to the caller or security
//! authority.
//!
//! Literal infinite storage or computation is not promised.
//!
//! # Determinism
//!
//! Trust assessment receives an explicit evaluation timestamp.
//!
//! No hidden wall-clock lookup is performed.
//!
//! The same evidence, policy, verification outcomes and evaluation
//! timestamp produce the same assessment.
//!
//! # Cryptographic boundary
//!
//! This module deliberately does not implement cryptography.
//!
//! The supplied SignatureVerifier must verify a cryptographic
//! signature over a canonical envelope binding:
//!
//! - The evidence identifier.
//! - The provider identity.
//! - The issuer identity.
//! - The verification key reference.
//! - The signature scheme.
//! - The issue time.
//! - The expiration time.
//! - The signed payload.
//!
//! The verifier must also validate the applicable trust chain
//! according to its security configuration.
//!
//! A successful verification must not be fabricated from
//! provider-supplied metadata.
//!
//! # Revocation boundary
//!
//! Revocation checking is delegated to an explicitly supplied
//! RevocationChecker.
//!
//! When a policy requires revocation checking, unavailable,
//! unknown or failed revocation information must not result
//! in a trusted decision.

use core::fmt;
use std::collections::BTreeSet;

use crate::stdlib::provider::ProviderId;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable component identifier.
pub const COMPONENT: &str = "zhal.security.trust";

/// Trust evidence schema version.
pub const TRUST_SCHEMA_VERSION: &str = "1";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn trust_error(
    kind: ZhalErrorKind,
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(kind, message)
        .in_component(COMPONENT)
}

fn invalid_input(
    message: impl Into<String>,
) -> ZhalError {
    trust_error(
        ZhalErrorKind::InvalidInput,
        message,
    )
}

fn invalid_evidence(
    message: impl Into<String>,
) -> ZhalError {
    trust_error(
        ZhalErrorKind::InvalidData,
        message,
    )
}

fn verification_failure(
    message: impl Into<String>,
) -> ZhalError {
    trust_error(
        ZhalErrorKind::ProviderFailure,
        message,
    )
}

// ============================================================================
// COMMON VALIDATION
// ============================================================================

/// Validates a trust-related descriptive identifier.
///
/// Identifiers remain open-world and case-sensitive.
///
/// No arbitrary maximum length or namespace depth is imposed.
fn validate_text(
    value: &str,
    field: &'static str,
) -> ZhalResult<()> {
    if value.trim().is_empty() {
        return Err(invalid_input(
            format!("{field} must not be empty"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_input(
            format!("{field} must not contain control characters"),
        ));
    }

    Ok(())
}

// ============================================================================
// TRUST IDENTIFIERS
// ============================================================================

/// An open-world trust evidence identifier.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct TrustEvidenceId(String);

impl TrustEvidenceId {
    /// Constructs a validated trust evidence identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> ZhalResult<Self> {
        let value = value.into();

        validate_text(
            &value,
            "trust evidence identifier",
        )?;

        Ok(Self(value))
    }

    /// Returns the exact identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TrustEvidenceId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// An open-world trust issuer identifier.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct TrustIssuerId(String);

impl TrustIssuerId {
    /// Constructs a validated issuer identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> ZhalResult<Self> {
        let value = value.into();

        validate_text(
            &value,
            "trust issuer identifier",
        )?;

        Ok(Self(value))
    }

    /// Returns the exact issuer identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TrustIssuerId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// An opaque verification-key reference.
///
/// This is a reference to externally managed key material,
/// never the key material itself.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct TrustKeyReference(String);

impl TrustKeyReference {
    /// Constructs a validated verification-key reference.
    pub fn new(
        value: impl Into<String>,
    ) -> ZhalResult<Self> {
        let value = value.into();

        validate_text(
            &value,
            "trust key reference",
        )?;

        Ok(Self(value))
    }

    /// Returns the key reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TrustKeyReference {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// An open-world cryptographic signature scheme identifier.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct SignatureScheme(String);

impl SignatureScheme {
    /// Constructs a validated signature scheme identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> ZhalResult<Self> {
        let value = value.into();

        validate_text(
            &value,
            "signature scheme",
        )?;

        Ok(Self(value))
    }

    /// Returns the exact scheme identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SignatureScheme {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// An open-world trust policy identifier.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct TrustPolicyId(String);

impl TrustPolicyId {
    /// Constructs a validated policy identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> ZhalResult<Self> {
        let value = value.into();

        validate_text(
            &value,
            "trust policy identifier",
        )?;

        Ok(Self(value))
    }

    /// Returns the policy identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// TRUST TIMESTAMP
// ============================================================================

/// Signed Unix timestamp in nanoseconds.
///
/// This representation avoids hidden clock dependencies and
/// supports deterministic assessment.
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
pub struct TrustTimestamp(i128);

impl TrustTimestamp {
    /// Constructs a timestamp from signed Unix nanoseconds.
    #[must_use]
    pub const fn from_unix_nanos(
        value: i128,
    ) -> Self {
        Self(value)
    }

    /// Returns signed Unix nanoseconds.
    #[must_use]
    pub const fn as_unix_nanos(self) -> i128 {
        self.0
    }
}

// ============================================================================
// TRUST EVIDENCE
// ============================================================================

/// Provider trust evidence.
///
/// Evidence is an externally supplied, signed statement.
///
/// It does not establish trust until an applicable verification
/// authority verifies it and the trust policy accepts it.
///
/// The signed envelope must bind all security-relevant metadata.
///
/// Sensitive credentials and private keys must never be stored
/// in this structure.
#[derive(Clone, Eq, PartialEq)]
pub struct TrustEvidence {
    id: TrustEvidenceId,

    subject: ProviderId,

    issuer: TrustIssuerId,

    key_reference: TrustKeyReference,

    scheme: SignatureScheme,

    issued_at: TrustTimestamp,

    expires_at: Option<TrustTimestamp>,

    signed_payload: Vec<u8>,

    signature: Vec<u8>,
}

impl TrustEvidence {
    /// Constructs trust evidence.
    ///
    /// The payload and signature must be the exact values required
    /// by the configured signature-verification implementation.
    ///
    /// This constructor does not verify the signature.
    pub fn new(
        id: TrustEvidenceId,
        subject: ProviderId,
        issuer: TrustIssuerId,
        key_reference: TrustKeyReference,
        scheme: SignatureScheme,
        issued_at: TrustTimestamp,
        expires_at: Option<TrustTimestamp>,
        signed_payload: Vec<u8>,
        signature: Vec<u8>,
    ) -> ZhalResult<Self> {
        if let Some(expiration) = expires_at {
            if expiration <= issued_at {
                return Err(invalid_evidence(
                    "trust evidence expiration must follow issuance",
                ));
            }
        }

        if signature.is_empty() {
            return Err(invalid_evidence(
                "trust evidence signature must not be empty",
            ));
        }

        Ok(Self {
            id,
            subject,
            issuer,
            key_reference,
            scheme,
            issued_at,
            expires_at,
            signed_payload,
            signature,
        })
    }

    /// Returns the evidence identifier.
    #[must_use]
    pub fn id(&self) -> &TrustEvidenceId {
        &self.id
    }

    /// Returns the provider to which the evidence applies.
    #[must_use]
    pub fn subject(&self) -> &ProviderId {
        &self.subject
    }

    /// Returns the issuing authority.
    #[must_use]
    pub fn issuer(&self) -> &TrustIssuerId {
        &self.issuer
    }

    /// Returns the external verification-key reference.
    #[must_use]
    pub fn key_reference(&self) -> &TrustKeyReference {
        &self.key_reference
    }

    /// Returns the signature scheme.
    #[must_use]
    pub fn scheme(&self) -> &SignatureScheme {
        &self.scheme
    }

    /// Returns the issuance timestamp.
    #[must_use]
    pub const fn issued_at(&self) -> TrustTimestamp {
        self.issued_at
    }

    /// Returns the optional expiration timestamp.
    #[must_use]
    pub const fn expires_at(&self) -> Option<TrustTimestamp> {
        self.expires_at
    }

    /// Returns the signed payload.
    #[must_use]
    pub fn signed_payload(&self) -> &[u8] {
        &self.signed_payload
    }

    /// Returns the signature bytes.
    #[must_use]
    pub fn signature(&self) -> &[u8] {
        &self.signature
    }

    /// Validates the evidence structure.
    pub fn validate(&self) -> ZhalResult<()> {
        if let Some(expiration) = self.expires_at {
            if expiration <= self.issued_at {
                return Err(invalid_evidence(
                    "trust evidence contains an invalid validity interval",
                ));
            }
        }

        if self.signature.is_empty() {
            return Err(invalid_evidence(
                "trust evidence has an empty signature",
            ));
        }

        Ok(())
    }
}

/// Redacted diagnostic formatting for trust evidence.
///
/// Payload and signature bytes are deliberately excluded.
impl fmt::Debug for TrustEvidence {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("TrustEvidence")
            .field("id", &self.id)
            .field("subject", &self.subject)
            .field("issuer", &self.issuer)
            .field("key_reference", &self.key_reference)
            .field("scheme", &self.scheme)
            .field("issued_at", &self.issued_at)
            .field("expires_at", &self.expires_at)
            .field("signed_payload_length", &self.signed_payload.len())
            .field("signature_length", &self.signature.len())
            .finish()
    }
}

// ============================================================================
// TRUST POLICY
// ============================================================================

/// Explicit provider trust policy.
///
/// An empty issuer or signature-scheme allowlist does not establish
/// an unrestricted trust policy. Evaluation rejects such a policy.
///
/// Policies must explicitly identify acceptable trust authorities
/// and cryptographic schemes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustPolicy {
    id: TrustPolicyId,

    trusted_issuers: BTreeSet<TrustIssuerId>,

    allowed_schemes: BTreeSet<SignatureScheme>,

    maximum_age_nanos: Option<u128>,

    require_revocation_check: bool,
}

impl TrustPolicy {
    /// Creates an initially restrictive trust policy.
    pub fn new(
        id: TrustPolicyId,
    ) -> Self {
        Self {
            id,
            trusted_issuers: BTreeSet::new(),
            allowed_schemes: BTreeSet::new(),
            maximum_age_nanos: None,
            require_revocation_check: true,
        }
    }

    /// Adds an explicitly trusted issuer.
    #[must_use]
    pub fn trust_issuer(
        mut self,
        issuer: TrustIssuerId,
    ) -> Self {
        self.trusted_issuers.insert(issuer);
        self
    }

    /// Adds an allowed signature scheme.
    #[must_use]
    pub fn allow_scheme(
        mut self,
        scheme: SignatureScheme,
    ) -> Self {
        self.allowed_schemes.insert(scheme);
        self
    }

    /// Sets the maximum permitted evidence age.
    ///
    /// The value is expressed in nanoseconds.
    ///
    /// This is a configurable policy restriction, not a universal
    /// hardware or provider capacity limit.
    #[must_use]
    pub fn maximum_age_nanos(
        mut self,
        value: Option<u128>,
    ) -> Self {
        self.maximum_age_nanos = value;
        self
    }

    /// Sets whether an affirmative revocation check is mandatory.
    ///
    /// The default is true.
    #[must_use]
    pub fn require_revocation_check(
        mut self,
        required: bool,
    ) -> Self {
        self.require_revocation_check = required;
        self
    }

    /// Returns the policy identifier.
    #[must_use]
    pub fn id(&self) -> &TrustPolicyId {
        &self.id
    }

    /// Returns trusted issuers.
    #[must_use]
    pub fn trusted_issuers(&self) -> &BTreeSet<TrustIssuerId> {
        &self.trusted_issuers
    }

    /// Returns permitted signature schemes.
    #[must_use]
    pub fn allowed_schemes(&self) -> &BTreeSet<SignatureScheme> {
        &self.allowed_schemes
    }

    /// Returns the maximum permitted evidence age.
    #[must_use]
    pub const fn maximum_age(&self) -> Option<u128> {
        self.maximum_age_nanos
    }

    /// Returns whether revocation checking is mandatory.
    #[must_use]
    pub const fn requires_revocation_check(&self) -> bool {
        self.require_revocation_check
    }

    /// Validates policy completeness.
    pub fn validate(&self) -> ZhalResult<()> {
        if self.trusted_issuers.is_empty() {
            return Err(invalid_input(
                "trust policy must explicitly identify trusted issuers",
            ));
        }

        if self.allowed_schemes.is_empty() {
            return Err(invalid_input(
                "trust policy must explicitly allow signature schemes",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// SIGNATURE VERIFICATION
// ============================================================================

/// Cryptographic signature verification result.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
)]
pub enum SignatureValidity {
    /// The cryptographic verification succeeded.
    Valid,

    /// The signature or its applicable trust chain is invalid.
    Invalid,

    /// Verification could not establish a reliable result.
    Indeterminate,
}

/// External signature-verification contract.
///
/// Implementations belong to the canonical security or
/// cryptographic provider infrastructure.
///
/// Implementations must verify the complete canonical evidence
/// envelope and its binding to the provider identity.
///
/// They must not trust provider-supplied verification assertions.
pub trait SignatureVerifier: Send + Sync {
    /// Verifies signed provider trust evidence.
    ///
    /// # Errors
    ///
    /// Returns a structured provider failure when verification
    /// infrastructure cannot complete its operation.
    fn verify(
        &self,
        evidence: &TrustEvidence,
    ) -> ZhalResult<SignatureValidity>;
}

// ============================================================================
// REVOCATION VERIFICATION
// ============================================================================

/// Trust evidence revocation status.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
)]
pub enum RevocationStatus {
    /// An authoritative check confirms that evidence is not revoked.
    NotRevoked,

    /// Evidence has been revoked.
    Revoked,

    /// Revocation could not be established.
    Unknown,
}

/// External trust revocation-checking contract.
///
/// Implementations may integrate with certificate revocation,
/// signed revocation lists, trust registries or other authoritative
/// revocation mechanisms.
///
/// No particular technology is hard-coded.
pub trait RevocationChecker: Send + Sync {
    /// Checks whether evidence has been revoked.
    fn check(
        &self,
        evidence: &TrustEvidence,
        evaluated_at: TrustTimestamp,
    ) -> ZhalResult<RevocationStatus>;
}

// ============================================================================
// TRUST DECISIONS
// ============================================================================

/// Final trust assessment classification.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
)]
pub enum TrustDecision {
    /// All mandatory trust requirements were satisfied.
    Trusted,

    /// Evidence or policy explicitly failed a trust requirement.
    Rejected,

    /// Available evidence was insufficient to establish trust.
    Indeterminate,
}

/// Structured trust assessment findings.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
#[non_exhaustive]
pub enum TrustFinding {
    /// The policy does not explicitly trust the issuer.
    UntrustedIssuer,

    /// The signature scheme is not permitted.
    UnsupportedSignatureScheme,

    /// Evidence concerns another provider.
    SubjectMismatch,

    /// Evidence was issued in the future.
    IssuedInFuture,

    /// Evidence has expired.
    Expired,

    /// Evidence exceeds the configured maximum age.
    EvidenceTooOld,

    /// Signature verification failed.
    InvalidSignature,

    /// Signature verification could not establish validity.
    VerificationIndeterminate,

    /// Evidence has been revoked.
    Revoked,

    /// Revocation status could not be established.
    RevocationUnknown,

    /// Required verification infrastructure was not supplied.
    VerificationUnavailable,

    /// Trust policy was incomplete.
    IncompletePolicy,
}

// ============================================================================
// TRUST ASSESSMENT
// ============================================================================

/// Immutable result of a provider trust assessment.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub struct TrustAssessment {
    decision: TrustDecision,

    policy: TrustPolicyId,

    evidence: TrustEvidenceId,

    provider: ProviderId,

    evaluated_at: TrustTimestamp,

    findings: Vec<TrustFinding>,
}

impl TrustAssessment {
    fn new(
        decision: TrustDecision,
        policy: &TrustPolicy,
        evidence: &TrustEvidence,
        evaluated_at: TrustTimestamp,
        findings: Vec<TrustFinding>,
    ) -> Self {
        Self {
            decision,
            policy: policy.id.clone(),
            evidence: evidence.id.clone(),
            provider: evidence.subject.clone(),
            evaluated_at,
            findings,
        }
    }

    /// Returns the trust decision.
    #[must_use]
    pub const fn decision(&self) -> TrustDecision {
        self.decision
    }

    /// Returns the applied policy.
    #[must_use]
    pub fn policy(&self) -> &TrustPolicyId {
        &self.policy
    }

    /// Returns the evaluated evidence identifier.
    #[must_use]
    pub fn evidence(&self) -> &TrustEvidenceId {
        &self.evidence
    }

    /// Returns the assessed provider.
    #[must_use]
    pub fn provider(&self) -> &ProviderId {
        &self.provider
    }

    /// Returns the evaluation timestamp.
    #[must_use]
    pub const fn evaluated_at(&self) -> TrustTimestamp {
        self.evaluated_at
    }

    /// Returns all assessment findings.
    #[must_use]
    pub fn findings(&self) -> &[TrustFinding] {
        &self.findings
    }

    /// Returns whether trust was affirmatively established.
    #[must_use]
    pub const fn is_trusted(&self) -> bool {
        matches!(
            self.decision,
            TrustDecision::Trusted
        )
    }

    /// Returns whether trust was explicitly rejected.
    #[must_use]
    pub const fn is_rejected(&self) -> bool {
        matches!(
            self.decision,
            TrustDecision::Rejected
        )
    }

    /// Returns whether trust remains unresolved.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        matches!(
            self.decision,
            TrustDecision::Indeterminate
        )
    }
}

// ============================================================================
// TRUST EVALUATION
// ============================================================================

/// Evaluates provider trust evidence against an explicit policy.
///
/// All time-dependent checks use the supplied evaluation timestamp.
///
/// A trusted decision requires:
///
/// 1. A valid and complete policy.
/// 2. Matching provider identity.
/// 3. An explicitly trusted issuer.
/// 4. An explicitly permitted signature scheme.
/// 5. A valid evidence time interval.
/// 6. Compliance with any maximum evidence age.
/// 7. Successful cryptographic verification.
/// 8. Affirmative revocation status when required.
///
/// The function never treats missing verification as success.
///
/// # Errors
///
/// Returns an error when a verifier or revocation authority fails
/// operationally. Such errors must not be converted into trust.
pub fn evaluate_trust(
    policy: &TrustPolicy,
    evidence: &TrustEvidence,
    expected_provider: &ProviderId,
    evaluated_at: TrustTimestamp,
    verifier: &dyn SignatureVerifier,
    revocation_checker: Option<&dyn RevocationChecker>,
) -> ZhalResult<TrustAssessment> {
    let mut findings = Vec::new();

    if policy.validate().is_err() {
        findings.push(TrustFinding::IncompletePolicy);

        return Ok(TrustAssessment::new(
            TrustDecision::Rejected,
            policy,
            evidence,
            evaluated_at,
            findings,
        ));
    }

    if let Err(_) = evidence.validate() {
        return Err(invalid_evidence(
            "provider trust evidence failed structural validation",
        ));
    }

    if evidence.subject != *expected_provider {
        findings.push(
            TrustFinding::SubjectMismatch,
        );

        return Ok(TrustAssessment::new(
            TrustDecision::Rejected,
            policy,
            evidence,
            evaluated_at,
            findings,
        ));
    }

    if !policy.trusted_issuers.contains(&evidence.issuer) {
        findings.push(
            TrustFinding::UntrustedIssuer,
        );
    }

    if !policy.allowed_schemes.contains(&evidence.scheme) {
        findings.push(
            TrustFinding::UnsupportedSignatureScheme,
        );
    }

    if evaluated_at < evidence.issued_at {
        findings.push(
            TrustFinding::IssuedInFuture,
        );
    }

    if let Some(expiration) = evidence.expires_at {
        if evaluated_at >= expiration {
            findings.push(
                TrustFinding::Expired,
            );
        }
    }

    if let Some(maximum_age) = policy.maximum_age_nanos {
        let age = evaluated_at
            .as_unix_nanos()
            .checked_sub(evidence.issued_at.as_unix_nanos());

        match age {
            Some(value) if value >= 0 => {
                if value as u128 > maximum_age {
                    findings.push(
                        TrustFinding::EvidenceTooOld,
                    );
                }
            }

            _ => {
                findings.push(
                    TrustFinding::IssuedInFuture,
                );
            }
        }
    }

    // Explicit policy failures cannot be repaired by a valid signature.

    if !findings.is_empty() {
        return Ok(TrustAssessment::new(
            TrustDecision::Rejected,
            policy,
            evidence,
            evaluated_at,
            findings,
        ));
    }

    // Cryptographic verification is delegated.
    // No provider-supplied boolean can substitute for this operation.

    let signature_result = verifier.verify(evidence)?;

    match signature_result {
        SignatureValidity::Valid => {}

        SignatureValidity::Invalid => {
            findings.push(
                TrustFinding::InvalidSignature,
            );

            return Ok(TrustAssessment::new(
                TrustDecision::Rejected,
                policy,
                evidence,
                evaluated_at,
                findings,
            ));
        }

        SignatureValidity::Indeterminate => {
            findings.push(
                TrustFinding::VerificationIndeterminate,
            );

            return Ok(TrustAssessment::new(
                TrustDecision::Indeterminate,
                policy,
                evidence,
                evaluated_at,
                findings,
            ));
        }
    }

    // Revocation must be established when required.

    if policy.require_revocation_check {
        let Some(checker) = revocation_checker else {
            findings.push(
                TrustFinding::VerificationUnavailable,
            );

            return Ok(TrustAssessment::new(
                TrustDecision::Indeterminate,
                policy,
                evidence,
                evaluated_at,
                findings,
            ));
        };

        match checker.check(
            evidence,
            evaluated_at,
        )? {
            RevocationStatus::NotRevoked => {}

            RevocationStatus::Revoked => {
                findings.push(
                    TrustFinding::Revoked,
                );

                return Ok(TrustAssessment::new(
                    TrustDecision::Rejected,
                    policy,
                    evidence,
                    evaluated_at,
                    findings,
                ));
            }

            RevocationStatus::Unknown => {
                findings.push(
                    TrustFinding::RevocationUnknown,
                );

                return Ok(TrustAssessment::new(
                    TrustDecision::Indeterminate,
                    policy,
                    evidence,
                    evaluated_at,
                    findings,
                ));
            }
        }
    }

    Ok(TrustAssessment::new(
        TrustDecision::Trusted,
        policy,
        evidence,
        evaluated_at,
        findings,
    ))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct ValidVerifier;

    impl SignatureVerifier for ValidVerifier {
        fn verify(
            &self,
            _evidence: &TrustEvidence,
        ) -> ZhalResult<SignatureValidity> {
            Ok(SignatureValidity::Valid)
        }
    }

    struct InvalidVerifier;

    impl SignatureVerifier for InvalidVerifier {
        fn verify(
            &self,
            _evidence: &TrustEvidence,
        ) -> ZhalResult<SignatureValidity> {
            Ok(SignatureValidity::Invalid)
        }
    }

    struct UnknownVerifier;

    impl SignatureVerifier for UnknownVerifier {
        fn verify(
            &self,
            _evidence: &TrustEvidence,
        ) -> ZhalResult<SignatureValidity> {
            Ok(SignatureValidity::Indeterminate)
        }
    }

    struct GoodRevocationChecker;

    impl RevocationChecker for GoodRevocationChecker {
        fn check(
            &self,
            _evidence: &TrustEvidence,
            _evaluated_at: TrustTimestamp,
        ) -> ZhalResult<RevocationStatus> {
            Ok(RevocationStatus::NotRevoked)
        }
    }

    struct RevokedChecker;

    impl RevocationChecker for RevokedChecker {
        fn check(
            &self,
            _evidence: &TrustEvidence,
            _evaluated_at: TrustTimestamp,
        ) -> ZhalResult<RevocationStatus> {
            Ok(RevocationStatus::Revoked)
        }
    }

    struct UnknownRevocationChecker;

    impl RevocationChecker for UnknownRevocationChecker {
        fn check(
            &self,
            _evidence: &TrustEvidence,
            _evaluated_at: TrustTimestamp,
        ) -> ZhalResult<RevocationStatus> {
            Ok(RevocationStatus::Unknown)
        }
    }

    fn provider() -> ProviderId {
        ProviderId::new(
            "zamani.test.provider",
        ).expect("valid provider")
    }

    fn issuer() -> TrustIssuerId {
        TrustIssuerId::new(
            "zamani.test.authority",
        ).expect("valid issuer")
    }

    fn scheme() -> SignatureScheme {
        SignatureScheme::new(
            "test.signature.scheme",
        ).expect("valid scheme")
    }

    fn policy() -> TrustPolicy {
        TrustPolicy::new(
            TrustPolicyId::new(
                "zamani.test.policy",
            ).expect("valid policy"),
        )
        .trust_issuer(issuer())
        .allow_scheme(scheme())
    }

    fn evidence() -> TrustEvidence {
        TrustEvidence::new(
            TrustEvidenceId::new(
                "evidence-1",
            ).expect("valid evidence identifier"),
            provider(),
            issuer(),
            TrustKeyReference::new(
                "key-reference-1",
            ).expect("valid key reference"),
            scheme(),
            TrustTimestamp::from_unix_nanos(100),
            Some(
                TrustTimestamp::from_unix_nanos(1_000),
            ),
            b"canonical signed envelope".to_vec(),
            b"signature".to_vec(),
        )
        .expect("valid evidence")
    }

    fn now() -> TrustTimestamp {
        TrustTimestamp::from_unix_nanos(200)
    }

    #[test]
    fn valid_evidence_establishes_trust() {
        let assessment = evaluate_trust(
            &policy(),
            &evidence(),
            &provider(),
            now(),
            &ValidVerifier,
            Some(&GoodRevocationChecker),
        ).expect("assessment succeeds");

        assert_eq!(
            assessment.decision(),
            TrustDecision::Trusted,
        );
    }

    #[test]
    fn invalid_signature_is_rejected() {
        let assessment = evaluate_trust(
            &policy(),
            &evidence(),
            &provider(),
            now(),
            &InvalidVerifier,
            Some(&GoodRevocationChecker),
        ).expect("assessment succeeds");

        assert_eq!(
            assessment.decision(),
            TrustDecision::Rejected,
        );

        assert!(
            assessment.findings().contains(
                &TrustFinding::InvalidSignature,
            )
        );
    }

    #[test]
    fn indeterminate_signature_never_establishes_trust() {
        let assessment = evaluate_trust(
            &policy(),
            &evidence(),
            &provider(),
            now(),
            &UnknownVerifier,
            Some(&GoodRevocationChecker),
        ).expect("assessment succeeds");

        assert_eq!(
            assessment.decision(),
            TrustDecision::Indeterminate,
        );
    }

    #[test]
    fn revoked_evidence_is_rejected() {
        let assessment = evaluate_trust(
            &policy(),
            &evidence(),
            &provider(),
            now(),
            &ValidVerifier,
            Some(&RevokedChecker),
        ).expect("assessment succeeds");

        assert_eq!(
            assessment.decision(),
            TrustDecision::Rejected,
        );
    }

    #[test]
    fn unknown_revocation_is_indeterminate() {
        let assessment = evaluate_trust(
            &policy(),
            &evidence(),
            &provider(),
            now(),
            &ValidVerifier,
            Some(&UnknownRevocationChecker),
        ).expect("assessment succeeds");

        assert_eq!(
            assessment.decision(),
            TrustDecision::Indeterminate,
        );
    }

    #[test]
    fn missing_revocation_authority_fails_closed() {
        let assessment = evaluate_trust(
            &policy(),
            &evidence(),
            &provider(),
            now(),
            &ValidVerifier,
            None,
        ).expect("assessment succeeds");

        assert_eq!(
            assessment.decision(),
            TrustDecision::Indeterminate,
        );
    }

    #[test]
    fn provider_identity_mismatch_is_rejected() {
        let other = ProviderId::new(
            "zamani.other.provider",
        ).expect("valid provider");

        let assessment = evaluate_trust(
            &policy(),
            &evidence(),
            &other,
            now(),
            &ValidVerifier,
            Some(&GoodRevocationChecker),
        ).expect("assessment succeeds");

        assert_eq!(
            assessment.decision(),
            TrustDecision::Rejected,
        );

        assert!(
            assessment.findings().contains(
                &TrustFinding::SubjectMismatch,
            )
        );
    }

    #[test]
    fn expired_evidence_is_rejected() {
        let assessment = evaluate_trust(
            &policy(),
            &evidence(),
            &provider(),
            TrustTimestamp::from_unix_nanos(1_000),
            &ValidVerifier,
            Some(&GoodRevocationChecker),
        ).expect("assessment succeeds");

        assert_eq!(
            assessment.decision(),
            TrustDecision::Rejected,
        );
    }

    #[test]
    fn untrusted_issuer_is_rejected() {
        let restrictive = TrustPolicy::new(
            TrustPolicyId::new(
                "zamani.restrictive.policy",
            ).expect("valid policy"),
        )
        .trust_issuer(
            TrustIssuerId::new(
                "another.authority",
            ).expect("valid issuer"),
        )
        .allow_scheme(scheme());

        let assessment = evaluate_trust(
            &restrictive,
            &evidence(),
            &provider(),
            now(),
            &ValidVerifier,
            Some(&GoodRevocationChecker),
        ).expect("assessment succeeds");

        assert_eq!(
            assessment.decision(),
            TrustDecision::Rejected,
        );
    }

    #[test]
    fn empty_policy_is_rejected() {
        let empty = TrustPolicy::new(
            TrustPolicyId::new(
                "empty.policy",
            ).expect("valid policy"),
        );

        let assessment = evaluate_trust(
            &empty,
            &evidence(),
            &provider(),
            now(),
            &ValidVerifier,
            Some(&GoodRevocationChecker),
        ).expect("assessment succeeds");

        assert_eq!(
            assessment.decision(),
            TrustDecision::Rejected,
        );

        assert!(
            assessment.findings().contains(
                &TrustFinding::IncompletePolicy,
            )
        );
    }

    #[test]
    fn maximum_age_is_enforced() {
        let restrictive = policy()
            .maximum_age_nanos(Some(10));

        let assessment = evaluate_trust(
            &restrictive,
            &evidence(),
            &provider(),
            now(),
            &ValidVerifier,
            Some(&GoodRevocationChecker),
        ).expect("assessment succeeds");

        assert_eq!(
            assessment.decision(),
            TrustDecision::Rejected,
        );
    }

    #[test]
    fn evidence_cannot_expire_before_issuance() {
        let result = TrustEvidence::new(
            TrustEvidenceId::new(
                "invalid-evidence",
            ).expect("valid identifier"),
            provider(),
            issuer(),
            TrustKeyReference::new(
                "key",
            ).expect("valid key"),
            scheme(),
            TrustTimestamp::from_unix_nanos(100),
            Some(
                TrustTimestamp::from_unix_nanos(50),
            ),
            b"payload".to_vec(),
            b"signature".to_vec(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn empty_signature_is_rejected() {
        let result = TrustEvidence::new(
            TrustEvidenceId::new(
                "empty-signature",
            ).expect("valid identifier"),
            provider(),
            issuer(),
            TrustKeyReference::new(
                "key",
            ).expect("valid key"),
            scheme(),
            now(),
            None,
            b"payload".to_vec(),
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn evidence_debug_does_not_expose_payload_or_signature() {
        let value = evidence();
        let diagnostic = format!("{value:?}");

        assert!(
            !diagnostic.contains("canonical signed envelope")
        );

        assert!(
            !diagnostic.contains("signature\"")
        );
    }

    #[test]
    fn policy_allowlist_is_deterministic() {
        let first = policy();
        let second = policy();

        assert_eq!(
            first.trusted_issuers(),
            second.trusted_issuers(),
        );

        assert_eq!(
            first.allowed_schemes(),
            second.allowed_schemes(),
        );
    }
}
