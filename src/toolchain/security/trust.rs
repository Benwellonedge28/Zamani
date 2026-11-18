
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Trust
//!
//! Path: src/toolchain/security/trust.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Provides the toolchain-level trust evidence and provider trust
//! assessment boundary.
//!
//! Trust assessment is applicable to registered providers, package
//! publishers, registries, external tools, build environments and
//! other explicitly identified toolchain entities.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Trust evidence identities.
//! - Trust issuer identities.
//! - Verification-key references.
//! - Signature scheme identifiers.
//! - Trust policy descriptions.
//! - Trust evidence validity evaluation.
//! - Trust assessment orchestration.
//! - Structured trust findings.
//!
//! It does not own:
//!
//! - Canonical provider identity.
//! - Provider registration or discovery.
//! - Authentication.
//! - Authorization.
//! - Credential management.
//! - Cryptographic algorithms.
//! - Private-key management.
//! - Trust-anchor persistence.
//! - Package integrity implementation.
//! - Provenance generation.
//! - Hardware discovery.
//! - Runtime isolation.
//! - Execution permission.
//!
//! ## Canonical authorities
//!
//! Provider identity:
//!     crate::stdlib::provider::ProviderId
//!
//! Cryptographic operations:
//!     crate::stdlib::security
//!
//! Hardware-provider trust:
//!     crate::zhal::security::trust
//!
//! Toolchain authorization:
//!     crate::toolchain::security::policy
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! Supply-chain verification:
//!     crate::compiler::security
//!
//! Source-level trust intent:
//!     grammar/security/trust.g4
//!
//! ## Security invariants
//!
//! 1. Registration does not establish trust.
//! 2. Provider identity does not establish trust.
//! 3. Provider metadata does not establish trust.
//! 4. Self-attestation is not independent verification.
//! 5. Missing signature verification never establishes trust.
//! 6. Unknown revocation status never establishes trust.
//! 7. Expired evidence cannot establish current trust.
//! 8. Evidence cannot establish trust for another provider.
//! 9. Invalid signatures cannot establish trust.
//! 10. Operational verification failures cannot establish trust.
//! 11. Trust does not imply authentication.
//! 12. Trust does not imply authorization.
//! 13. Trust does not imply hardware availability.
//! 14. Trust assessment does not authorize execution.
//! 15. Cryptographic operations are delegated.
//!
//! ## Determinism
//!
//! Every evaluation receives an explicit timestamp.
//! No implicit wall-clock access is performed.
//!
//! ## Scalability
//!
//! Identifiers and collections are open-world.
//! No arbitrary universal provider, evidence, issuer, algorithm,
//! target, device or computing-domain limit is introduced.
//!
//! Operational budgets belong to their owning operations.
//! Literal infinite storage or computation is not promised.

use std::collections::BTreeSet;
use std::fmt;

use crate::stdlib::provider::ProviderId;

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
    ToolchainResult,
};

/// Canonical component identifier.
pub const COMPONENT: &str = "toolchain.security.trust";

/// Trust evidence schema version.
pub const TRUST_SCHEMA_VERSION: &str = "1";

/// Stable operation identity for trust assessment.
pub const ASSESS_TRUST_OPERATION: &str =
    "toolchain.security.trust.assess";

fn trust_error(
    kind: ToolchainErrorKind,
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(kind, message)
        .with_operation(ASSESS_TRUST_OPERATION)
        .with_context(COMPONENT)
}

fn invalid_input(
    message: impl Into<String>,
) -> ToolchainError {
    trust_error(
        ToolchainErrorKind::InvalidInput,
        message,
    )
}

fn invalid_evidence(
    message: impl Into<String>,
) -> ToolchainError {
    trust_error(
        ToolchainErrorKind::ValidationFailed,
        message,
    )
}

/// Validates an open-world descriptive identifier.
///
/// Identifiers are case-sensitive and preserved exactly.
/// No arbitrary maximum length or namespace depth is imposed.
fn validate_identifier(
    value: &str,
    field: &'static str,
) -> ToolchainResult<()> {
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
// IDENTIFIERS
// ============================================================================

/// Open-world identity of a trust evidence record.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TrustEvidenceId(String);

/// Open-world identity of a trust issuer.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TrustIssuerId(String);

/// Opaque reference to externally managed verification-key material.
///
/// This contains no private key or secret key material.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TrustKeyReference(String);

/// Open-world cryptographic signature scheme identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SignatureScheme(String);

/// Open-world identity of a trust policy.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TrustPolicyId(String);

macro_rules! implement_trust_identifier {
    ($type:ident, $field:literal) => {
        impl $type {
            /// Constructs a validated identifier.
            pub fn new(
                value: impl Into<String>,
            ) -> ToolchainResult<Self> {
                let value = value.into();

                validate_identifier(&value, $field)?;

                Ok(Self(value))
            }

            /// Returns the exact identifier.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $type {
            fn fmt(
                &self,
                formatter: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }
    };
}

implement_trust_identifier!(
    TrustEvidenceId,
    "trust evidence identifier"
);

implement_trust_identifier!(
    TrustIssuerId,
    "trust issuer identifier"
);

implement_trust_identifier!(
    TrustKeyReference,
    "trust verification-key reference"
);

implement_trust_identifier!(
    SignatureScheme,
    "signature scheme"
);

implement_trust_identifier!(
    TrustPolicyId,
    "trust policy identifier"
);

// ============================================================================
// TIME
// ============================================================================

/// Signed Unix timestamp expressed in nanoseconds.
///
/// An explicit timestamp makes assessments deterministic and
/// independent of hidden system-clock access.
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
    /// Constructs a timestamp from Unix nanoseconds.
    #[must_use]
    pub const fn from_unix_nanos(value: i128) -> Self {
        Self(value)
    }

    /// Returns Unix nanoseconds.
    #[must_use]
    pub const fn as_unix_nanos(self) -> i128 {
        self.0
    }

    /// Returns the elapsed nonnegative nanoseconds between two timestamps.
    ///
    /// Returns `None` if the first timestamp precedes the second
    /// or the subtraction cannot be represented.
    #[must_use]
    pub fn elapsed_since(
        self,
        earlier: Self,
    ) -> Option<u128> {
        let difference = self.0.checked_sub(earlier.0)?;

        if difference < 0 {
            return None;
        }

        Some(difference as u128)
    }
}

// ============================================================================
// TRUST EVIDENCE
// ============================================================================

/// Signed provider trust evidence.
///
/// The verifier must verify a canonical signed envelope binding:
///
/// - Evidence identity.
/// - Provider identity.
/// - Issuer identity.
/// - Verification-key reference.
/// - Signature scheme.
/// - Issue timestamp.
/// - Expiration timestamp.
/// - Signed payload.
///
/// The envelope encoding and cryptographic implementation belong
/// to the canonical security verification infrastructure.
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
    /// Construction validates structural consistency only.
    /// It does not establish authenticity or trust.
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
    ) -> ToolchainResult<Self> {
        if signed_payload.is_empty() {
            return Err(invalid_evidence(
                "signed trust payload must not be empty",
            ));
        }

        if signature.is_empty() {
            return Err(invalid_evidence(
                "trust evidence signature must not be empty",
            ));
        }

        if let Some(expiration) = expires_at {
            if expiration <= issued_at {
                return Err(invalid_evidence(
                    "expiration must follow evidence issuance",
                ));
            }
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

    /// Returns the evidence identity.
    #[must_use]
    pub fn id(&self) -> &TrustEvidenceId {
        &self.id
    }

    /// Returns the canonical provider identity.
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
    ///
    /// The caller must not interpret this payload as verified
    /// until signature verification succeeds.
    #[must_use]
    pub fn signed_payload(&self) -> &[u8] {
        &self.signed_payload
    }

    /// Returns the signature bytes.
    ///
    /// The signature is untrusted input until independently verified.
    #[must_use]
    pub fn signature(&self) -> &[u8] {
        &self.signature
    }

    /// Validates the evidence structure.
    pub fn validate(&self) -> ToolchainResult<()> {
        if self.signed_payload.is_empty() {
            return Err(invalid_evidence(
                "trust evidence has an empty signed payload",
            ));
        }

        if self.signature.is_empty() {
            return Err(invalid_evidence(
                "trust evidence has an empty signature",
            ));
        }

        if let Some(expiration) = self.expires_at {
            if expiration <= self.issued_at {
                return Err(invalid_evidence(
                    "trust evidence has an invalid validity interval",
                ));
            }
        }

        Ok(())
    }
}

/// Redacted diagnostic representation.
///
/// Payload and signature bytes are never printed.
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
            .field(
                "signed_payload_length",
                &self.signed_payload.len(),
            )
            .field("signature_length", &self.signature.len())
            .finish()
    }
}

// ============================================================================
// TRUST POLICY
// ============================================================================

/// Explicit provider trust policy.
///
/// Empty issuer or signature-scheme allowlists are invalid.
/// An unrestricted implicit trust policy is never created.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustPolicy {
    id: TrustPolicyId,
    trusted_issuers: BTreeSet<TrustIssuerId>,
    allowed_schemes: BTreeSet<SignatureScheme>,
    maximum_age_nanos: Option<u128>,
    require_expiration: bool,
    require_revocation_check: bool,
}

impl TrustPolicy {
    /// Creates an initially restrictive trust policy.
    pub fn new(id: TrustPolicyId) -> Self {
        Self {
            id,
            trusted_issuers: BTreeSet::new(),
            allowed_schemes: BTreeSet::new(),
            maximum_age_nanos: None,
            require_expiration: true,
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

    /// Adds an explicitly permitted signature scheme.
    #[must_use]
    pub fn allow_scheme(
        mut self,
        scheme: SignatureScheme,
    ) -> Self {
        self.allowed_schemes.insert(scheme);
        self
    }

    /// Configures the maximum permitted evidence age.
    ///
    /// `None` means no additional age restriction.
    #[must_use]
    pub fn maximum_age_nanos(
        mut self,
        value: Option<u128>,
    ) -> Self {
        self.maximum_age_nanos = value;
        self
    }

    /// Configures whether evidence expiration is mandatory.
    #[must_use]
    pub fn require_expiration(
        mut self,
        required: bool,
    ) -> Self {
        self.require_expiration = required;
        self
    }

    /// Configures whether affirmative revocation checking is mandatory.
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

    /// Returns the policy identity.
    #[must_use]
    pub fn id(&self) -> &TrustPolicyId {
        &self.id
    }

    /// Returns explicitly trusted issuers.
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

    /// Returns whether expiration is mandatory.
    #[must_use]
    pub const fn requires_expiration(&self) -> bool {
        self.require_expiration
    }

    /// Returns whether revocation checking is mandatory.
    #[must_use]
    pub const fn requires_revocation_check(&self) -> bool {
        self.require_revocation_check
    }

    /// Validates policy completeness.
    pub fn validate(&self) -> ToolchainResult<()> {
        if self.trusted_issuers.is_empty() {
            return Err(invalid_input(
                "trust policy must specify trusted issuers",
            ));
        }

        if self.allowed_schemes.is_empty() {
            return Err(invalid_input(
                "trust policy must specify permitted signature schemes",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// SIGNATURE VERIFICATION CONTRACT
// ============================================================================

/// Cryptographic signature verification outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SignatureValidity {
    /// The complete signed evidence envelope was verified.
    Valid,

    /// The signature or applicable trust chain was rejected.
    Invalid,

    /// Verification could not establish a reliable result.
    Indeterminate,
}

/// External cryptographic verification contract.
///
/// Implementations must delegate to the canonical Zamani security
/// verification infrastructure or an explicitly integrated provider.
///
/// Implementations must verify the complete canonical signed envelope,
/// not merely the payload while ignoring identity and validity metadata.
///
/// A provider-supplied boolean or metadata assertion is not proof.
///
/// This trait is an orchestration boundary, not a cryptographic
/// algorithm implementation.
pub trait SignatureVerifier: Send + Sync {
    /// Verifies the complete trust evidence envelope.
    ///
    /// # Errors
    ///
    /// Returns an operational toolchain error when verification
    /// infrastructure fails.
    fn verify(
        &self,
        evidence: &TrustEvidence,
    ) -> ToolchainResult<SignatureValidity>;
}

// ============================================================================
// REVOCATION CONTRACT
// ============================================================================

/// Trust evidence revocation outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RevocationStatus {
    /// An authoritative source confirms that evidence is not revoked.
    NotRevoked,

    /// Evidence has been revoked.
    Revoked,

    /// Revocation status cannot be established.
    Unknown,
}

/// External revocation-checking contract.
///
/// Implementations may integrate with authoritative trust registries,
/// certificate infrastructure, signed revocation records or other
/// approved security providers.
///
/// No particular vendor or protocol is hard-coded.
pub trait RevocationChecker: Send + Sync {
    /// Checks evidence revocation at an explicit timestamp.
    ///
    /// # Errors
    ///
    /// Returns an operational error when the revocation service fails.
    fn check(
        &self,
        evidence: &TrustEvidence,
        evaluated_at: TrustTimestamp,
    ) -> ToolchainResult<RevocationStatus>;
}

// ============================================================================
// DECISIONS AND FINDINGS
// ============================================================================

/// Final trust assessment decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TrustDecision {
    /// Every mandatory trust requirement was satisfied.
    Trusted,

    /// An explicit trust requirement was violated.
    Rejected,

    /// Available evidence was insufficient to establish trust.
    Indeterminate,
}

/// Structured trust assessment finding.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TrustFinding {
    /// The policy is incomplete.
    IncompletePolicy,

    /// The evidence concerns a different provider.
    SubjectMismatch,

    /// The issuer is not explicitly trusted.
    UntrustedIssuer,

    /// The signature scheme is not permitted.
    UnsupportedSignatureScheme,

    /// The evidence was issued in the future.
    IssuedInFuture,

    /// The evidence has expired.
    Expired,

    /// Evidence does not provide a required expiration.
    ExpirationMissing,

    /// Evidence exceeds the configured age.
    EvidenceTooOld,

    /// Cryptographic signature verification is unavailable.
    VerificationUnavailable,

    /// Cryptographic verification rejected the signature.
    InvalidSignature,

    /// Cryptographic verification was indeterminate.
    VerificationIndeterminate,

    /// Revocation checking is unavailable.
    RevocationUnavailable,

    /// Evidence has been revoked.
    Revoked,

    /// Revocation status is unknown.
    RevocationUnknown,

    /// A verification authority failed operationally.
    VerificationFailed,
}

fn contains_rejection(findings: &[TrustFinding]) -> bool {
    findings.iter().any(|finding| {
        matches!(
            finding,
            TrustFinding::IncompletePolicy
                | TrustFinding::SubjectMismatch
                | TrustFinding::UntrustedIssuer
                | TrustFinding::UnsupportedSignatureScheme
                | TrustFinding::IssuedInFuture
                | TrustFinding::Expired
                | TrustFinding::ExpirationMissing
                | TrustFinding::EvidenceTooOld
                | TrustFinding::InvalidSignature
                | TrustFinding::Revoked
        )
    })
}

// ============================================================================
// TRUST ASSESSMENT
// ============================================================================

/// Immutable result of an individual provider trust assessment.
#[derive(Clone, Debug, Eq, PartialEq)]
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

    /// Returns the decision.
    #[must_use]
    pub const fn decision(&self) -> TrustDecision {
        self.decision
    }

    /// Returns the applied policy.
    #[must_use]
    pub fn policy(&self) -> &TrustPolicyId {
        &self.policy
    }

    /// Returns the assessed evidence identity.
    #[must_use]
    pub fn evidence(&self) -> &TrustEvidenceId {
        &self.evidence
    }

    /// Returns the assessed provider.
    #[must_use]
    pub fn provider(&self) -> &ProviderId {
        &self.provider
    }

    /// Returns the explicit evaluation timestamp.
    #[must_use]
    pub const fn evaluated_at(&self) -> TrustTimestamp {
        self.evaluated_at
    }

    /// Returns structured findings.
    #[must_use]
    pub fn findings(&self) -> &[TrustFinding] {
        &self.findings
    }

    /// Returns whether trust was affirmatively established.
    #[must_use]
    pub const fn is_trusted(&self) -> bool {
        matches!(self.decision, TrustDecision::Trusted)
    }

    /// Returns whether trust was explicitly rejected.
    #[must_use]
    pub const fn is_rejected(&self) -> bool {
        matches!(self.decision, TrustDecision::Rejected)
    }

    /// Returns whether trust remains unresolved.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        matches!(self.decision, TrustDecision::Indeterminate)
    }
}

// ============================================================================
// TRUST EVALUATION
// ============================================================================

/// Evaluates one provider trust evidence record.
///
/// A trusted result requires:
///
/// 1. A complete policy.
/// 2. Matching provider identity.
/// 3. An explicitly trusted issuer.
/// 4. An explicitly permitted signature scheme.
/// 5. A valid evidence interval.
/// 6. Compliance with configured freshness.
/// 7. Successful cryptographic verification.
/// 8. Affirmative revocation status when required.
///
/// Missing verification infrastructure produces an indeterminate
/// result, never a trusted result.
///
/// Explicit policy violations produce rejection.
///
/// # Errors
///
/// Returns structural input errors. Operational verifier and
/// revocation errors are recorded as indeterminate findings so that
/// callers receive a structured assessment rather than a false success.
pub fn evaluate_trust(
    policy: &TrustPolicy,
    evidence: &TrustEvidence,
    expected_provider: &ProviderId,
    evaluated_at: TrustTimestamp,
    verifier: Option<&dyn SignatureVerifier>,
    revocation_checker: Option<&dyn RevocationChecker>,
) -> ToolchainResult<TrustAssessment> {
    evidence.validate()?;

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

    if evidence.subject != *expected_provider {
        findings.push(TrustFinding::SubjectMismatch);

        return Ok(TrustAssessment::new(
            TrustDecision::Rejected,
            policy,
            evidence,
            evaluated_at,
            findings,
        ));
    }

    if !policy.trusted_issuers.contains(&evidence.issuer) {
        findings.push(TrustFinding::UntrustedIssuer);
    }

    if !policy.allowed_schemes.contains(&evidence.scheme) {
        findings.push(TrustFinding::UnsupportedSignatureScheme);
    }

    if evaluated_at < evidence.issued_at {
        findings.push(TrustFinding::IssuedInFuture);
    }

    match evidence.expires_at {
        Some(expiration) if evaluated_at >= expiration => {
            findings.push(TrustFinding::Expired);
        }

        None if policy.require_expiration => {
            findings.push(TrustFinding::ExpirationMissing);
        }

        _ => {}
    }

    if let Some(maximum_age) = policy.maximum_age_nanos {
        match evaluated_at.elapsed_since(evidence.issued_at) {
            Some(age) if age > maximum_age => {
                findings.push(TrustFinding::EvidenceTooOld);
            }

            None => {
                if !findings.contains(&TrustFinding::IssuedInFuture) {
                    findings.push(TrustFinding::IssuedInFuture);
                }
            }

            _ => {}
        }
    }

    // Explicit policy failures cannot be repaired by a signature.

    if contains_rejection(&findings) {
        return Ok(TrustAssessment::new(
            TrustDecision::Rejected,
            policy,
            evidence,
            evaluated_at,
            findings,
        ));
    }

    // Cryptographic verification is mandatory.

    let signature_result = match verifier {
        Some(verifier) => verifier.verify(evidence),

        None => {
            findings.push(TrustFinding::VerificationUnavailable);

            return Ok(TrustAssessment::new(
                TrustDecision::Indeterminate,
                policy,
                evidence,
                evaluated_at,
                findings,
            ));
        }
    };

    match signature_result {
        Ok(SignatureValidity::Valid) => {}

        Ok(SignatureValidity::Invalid) => {
            findings.push(TrustFinding::InvalidSignature);

            return Ok(TrustAssessment::new(
                TrustDecision::Rejected,
                policy,
                evidence,
                evaluated_at,
                findings,
            ));
        }

        Ok(SignatureValidity::Indeterminate) => {
            findings.push(TrustFinding::VerificationIndeterminate);

            return Ok(TrustAssessment::new(
                TrustDecision::Indeterminate,
                policy,
                evidence,
                evaluated_at,
                findings,
            ));
        }

        Err(_) => {
            findings.push(TrustFinding::VerificationFailed);

            return Ok(TrustAssessment::new(
                TrustDecision::Indeterminate,
                policy,
                evidence,
                evaluated_at,
                findings,
            ));
        }
    }

    // Revocation verification is independently mandatory when configured.

    if policy.require_revocation_check {
        let checker = match revocation_checker {
            Some(checker) => checker,

            None => {
                findings.push(TrustFinding::RevocationUnavailable);

                return Ok(TrustAssessment::new(
                    TrustDecision::Indeterminate,
                    policy,
                    evidence,
                    evaluated_at,
                    findings,
                ));
            }
        };

        match checker.check(evidence, evaluated_at) {
            Ok(RevocationStatus::NotRevoked) => {}

            Ok(RevocationStatus::Revoked) => {
                findings.push(TrustFinding::Revoked);

                return Ok(TrustAssessment::new(
                    TrustDecision::Rejected,
                    policy,
                    evidence,
                    evaluated_at,
                    findings,
                ));
            }

            Ok(RevocationStatus::Unknown) => {
                findings.push(TrustFinding::RevocationUnknown);

                return Ok(TrustAssessment::new(
                    TrustDecision::Indeterminate,
                    policy,
                    evidence,
                    evaluated_at,
                    findings,
                ));
            }

            Err(_) => {
                findings.push(TrustFinding::VerificationFailed);

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
        ) -> ToolchainResult<SignatureValidity> {
            Ok(SignatureValidity::Valid)
        }
    }

    struct InvalidVerifier;

    impl SignatureVerifier for InvalidVerifier {
        fn verify(
            &self,
            _evidence: &TrustEvidence,
        ) -> ToolchainResult<SignatureValidity> {
            Ok(SignatureValidity::Invalid)
        }
    }

    struct FailingVerifier;

    impl SignatureVerifier for FailingVerifier {
        fn verify(
            &self,
            _evidence: &TrustEvidence,
        ) -> ToolchainResult<SignatureValidity> {
            Err(ToolchainError::new(
                ToolchainErrorKind::ProviderFailed,
                "verification provider unavailable",
            ))
        }
    }

    struct ValidRevocationChecker;

    impl RevocationChecker for ValidRevocationChecker {
        fn check(
            &self,
            _evidence: &TrustEvidence,
            _evaluated_at: TrustTimestamp,
        ) -> ToolchainResult<RevocationStatus> {
            Ok(RevocationStatus::NotRevoked)
        }
    }

    struct RevokedChecker;

    impl RevocationChecker for RevokedChecker {
        fn check(
            &self,
            _evidence: &TrustEvidence,
            _evaluated_at: TrustTimestamp,
        ) -> ToolchainResult<RevocationStatus> {
            Ok(RevocationStatus::Revoked)
        }
    }

    struct UnknownRevocationChecker;

    impl RevocationChecker for UnknownRevocationChecker {
        fn check(
            &self,
            _evidence: &TrustEvidence,
            _evaluated_at: TrustTimestamp,
        ) -> ToolchainResult<RevocationStatus> {
            Ok(RevocationStatus::Unknown)
        }
    }

    fn provider() -> ProviderId {
        ProviderId::new("org.example.provider")
            .expect("valid provider identity")
    }

    fn evidence() -> TrustEvidence {
        TrustEvidence::new(
            TrustEvidenceId::new("evidence.example.1").unwrap(),
            provider(),
            TrustIssuerId::new("authority.example").unwrap(),
            TrustKeyReference::new("keys.example.verification").unwrap(),
            SignatureScheme::new("signature.example.scheme").unwrap(),
            TrustTimestamp::from_unix_nanos(100),
            Some(TrustTimestamp::from_unix_nanos(10_000)),
            b"canonical-signed-envelope".to_vec(),
            b"signature-bytes".to_vec(),
        )
        .expect("valid evidence")
    }

    fn policy() -> TrustPolicy {
        TrustPolicy::new(
            TrustPolicyId::new("policy.example").unwrap(),
        )
        .trust_issuer(
            TrustIssuerId::new("authority.example").unwrap(),
        )
        .allow_scheme(
            SignatureScheme::new("signature.example.scheme").unwrap(),
        )
    }

    fn assess(
        verifier: Option<&dyn SignatureVerifier>,
        checker: Option<&dyn RevocationChecker>,
    ) -> TrustAssessment {
        evaluate_trust(
            &policy(),
            &evidence(),
            &provider(),
            TrustTimestamp::from_unix_nanos(200),
            verifier,
            checker,
        )
        .expect("assessment should complete")
    }

    #[test]
    fn open_world_identifiers_are_supported() {
        assert!(
            TrustIssuerId::new(
                "future.organization.security.authority",
            )
            .is_ok()
        );

        assert!(
            SignatureScheme::new(
                "future.post_quantum.signature.scheme",
            )
            .is_ok()
        );
    }

    #[test]
    fn empty_identifiers_are_rejected() {
        assert!(TrustEvidenceId::new("").is_err());
        assert!(TrustIssuerId::new(" ").is_err());
        assert!(TrustKeyReference::new("").is_err());
        assert!(SignatureScheme::new("").is_err());
        assert!(TrustPolicyId::new("").is_err());
    }

    #[test]
    fn evidence_rejects_empty_payload() {
        let result = TrustEvidence::new(
            TrustEvidenceId::new("evidence.1").unwrap(),
            provider(),
            TrustIssuerId::new("authority").unwrap(),
            TrustKeyReference::new("key.1").unwrap(),
            SignatureScheme::new("scheme.1").unwrap(),
            TrustTimestamp::from_unix_nanos(1),
            Some(TrustTimestamp::from_unix_nanos(2)),
            Vec::new(),
            vec![1],
        );

        assert!(result.is_err());
    }

    #[test]
    fn evidence_rejects_invalid_validity_interval() {
        let result = TrustEvidence::new(
            TrustEvidenceId::new("evidence.1").unwrap(),
            provider(),
            TrustIssuerId::new("authority").unwrap(),
            TrustKeyReference::new("key.1").unwrap(),
            SignatureScheme::new("scheme.1").unwrap(),
            TrustTimestamp::from_unix_nanos(10),
            Some(TrustTimestamp::from_unix_nanos(10)),
            vec![1],
            vec![1],
        );

        assert!(result.is_err());
    }

    #[test]
    fn valid_evidence_can_be_trusted() {
        let result = assess(
            Some(&ValidVerifier),
            Some(&ValidRevocationChecker),
        );

        assert!(result.is_trusted());
        assert!(result.findings().is_empty());
    }

    #[test]
    fn missing_signature_verifier_never_establishes_trust() {
        let result = assess(
            None,
            Some(&ValidRevocationChecker),
        );

        assert!(result.is_indeterminate());
        assert!(
            result.findings().contains(
                &TrustFinding::VerificationUnavailable,
            )
        );
    }

    #[test]
    fn missing_revocation_checker_never_establishes_trust() {
        let result = assess(
            Some(&ValidVerifier),
            None,
        );

        assert!(result.is_indeterminate());
        assert!(
            result.findings().contains(
                &TrustFinding::RevocationUnavailable,
            )
        );
    }

    #[test]
    fn invalid_signature_is_rejected() {
        let result = assess(
            Some(&InvalidVerifier),
            Some(&ValidRevocationChecker),
        );

        assert!(result.is_rejected());
        assert!(
            result.findings().contains(
                &TrustFinding::InvalidSignature,
            )
        );
    }

    #[test]
    fn revoked_evidence_is_rejected() {
        let result = assess(
            Some(&ValidVerifier),
            Some(&RevokedChecker),
        );

        assert!(result.is_rejected());
        assert!(
            result.findings().contains(
                &TrustFinding::Revoked,
            )
        );
    }

    #[test]
    fn unknown_revocation_is_indeterminate() {
        let result = assess(
            Some(&ValidVerifier),
            Some(&UnknownRevocationChecker),
        );

        assert!(result.is_indeterminate());
        assert!(
            result.findings().contains(
                &TrustFinding::RevocationUnknown,
            )
        );
    }

    #[test]
    fn operational_verification_failure_is_indeterminate() {
        let result = assess(
            Some(&FailingVerifier),
            Some(&ValidRevocationChecker),
        );

        assert!(result.is_indeterminate());
        assert!(
            result.findings().contains(
                &TrustFinding::VerificationFailed,
            )
        );
    }

    #[test]
    fn provider_mismatch_is_rejected() {
        let other = ProviderId::new(
            "org.example.other",
        )
        .unwrap();

        let result = evaluate_trust(
            &policy(),
            &evidence(),
            &other,
            TrustTimestamp::from_unix_nanos(200),
            Some(&ValidVerifier),
            Some(&ValidRevocationChecker),
        )
        .unwrap();

        assert!(result.is_rejected());
        assert!(
            result.findings().contains(
                &TrustFinding::SubjectMismatch,
            )
        );
    }

    #[test]
    fn untrusted_issuer_is_rejected() {
        let evidence = TrustEvidence::new(
            TrustEvidenceId::new("evidence.2").unwrap(),
            provider(),
            TrustIssuerId::new("untrusted.authority").unwrap(),
            TrustKeyReference::new("keys.example.verification").unwrap(),
            SignatureScheme::new("signature.example.scheme").unwrap(),
            TrustTimestamp::from_unix_nanos(100),
            Some(TrustTimestamp::from_unix_nanos(10_000)),
            vec![1],
            vec![1],
        )
        .unwrap();

        let result = evaluate_trust(
            &policy(),
            &evidence,
            &provider(),
            TrustTimestamp::from_unix_nanos(200),
            Some(&ValidVerifier),
            Some(&ValidRevocationChecker),
        )
        .unwrap();

        assert!(result.is_rejected());
        assert!(
            result.findings().contains(
                &TrustFinding::UntrustedIssuer,
            )
        );
    }

    #[test]
    fn expired_evidence_is_rejected() {
        let result = evaluate_trust(
            &policy(),
            &evidence(),
            &provider(),
            TrustTimestamp::from_unix_nanos(10_000),
            Some(&ValidVerifier),
            Some(&ValidRevocationChecker),
        )
        .unwrap();

        assert!(result.is_rejected());
        assert!(
            result.findings().contains(
                &TrustFinding::Expired,
            )
        );
    }

    #[test]
    fn future_evidence_is_rejected() {
        let result = evaluate_trust(
            &policy(),
            &evidence(),
            &provider(),
            TrustTimestamp::from_unix_nanos(50),
            Some(&ValidVerifier),
            Some(&ValidRevocationChecker),
        )
        .unwrap();

        assert!(result.is_rejected());
        assert!(
            result.findings().contains(
                &TrustFinding::IssuedInFuture,
            )
        );
    }

    #[test]
    fn incomplete_policy_is_rejected() {
        let policy = TrustPolicy::new(
            TrustPolicyId::new("policy.incomplete").unwrap(),
        );

        let result = evaluate_trust(
            &policy,
            &evidence(),
            &provider(),
            TrustTimestamp::from_unix_nanos(200),
            Some(&ValidVerifier),
            Some(&ValidRevocationChecker),
        )
        .unwrap();

        assert!(result.is_rejected());
        assert!(
            result.findings().contains(
                &TrustFinding::IncompletePolicy,
            )
        );
    }

    #[test]
    fn debug_output_does_not_expose_payload_or_signature() {
        let evidence = evidence();

        let output = format!("{evidence:?}");

        assert!(!output.contains("canonical-signed-envelope"));
        assert!(!output.contains("signature-bytes"));
        assert!(output.contains("signed_payload_length"));
    }

    #[test]
    fn timestamp_arithmetic_is_checked() {
        let later = TrustTimestamp::from_unix_nanos(200);
        let earlier = TrustTimestamp::from_unix_nanos(100);

        assert_eq!(later.elapsed_since(earlier), Some(100));
        assert_eq!(earlier.elapsed_since(later), None);
    }

    #[test]
    fn assessments_are_deterministic() {
        let first = assess(
            Some(&ValidVerifier),
            Some(&ValidRevocationChecker),
        );

        let second = assess(
            Some(&ValidVerifier),
            Some(&ValidRevocationChecker),
        );

        assert_eq!(first, second);
    }
}
