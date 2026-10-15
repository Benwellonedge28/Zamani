
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Proof Records
//!
//! Canonical path: `src/zuir/values/proofs.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Represent claims, proof and analysis outcomes, evidence references,
//! assumptions, and proof dependencies in the canonical ZUIR value layer.
//!
//! OWNS
//!
//! - Stable local proof-record identities.
//! - Open-world, qualified claim and checker identities.
//! - Explicit proof methods and outcomes.
//! - Evidence references and optional integrity-digest metadata.
//! - Explicit assumptions and supporting-proof dependencies.
//! - Local validation and configurable validation budgets.
//!
//! DOES_NOT_OWN
//!
//! - The implementation of a theorem prover or proof checker.
//! - Type checking, effect inference, or semantic name resolution.
//! - Trust policy, authorization, signatures, or key management.
//! - Runtime behavior, target feasibility, or hardware discovery.
//! - Source parsing, AST construction, or source diagnostics rendering.
//! - Proof dependency resolution or whole-graph cycle detection.
//! - Cryptographic digest calculation or digest authenticity verification.
//! - Whole-program serialization or the global ZUIR verification policy.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/schema/types.rs`
//! - `src/frontend/semantic/`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compatibility.md`
//!
//! ARCHITECTURAL BOUNDARY
//!
//! This module stores proof-related values. It does not establish that a
//! recorded claim is true merely because a record says it is verified.
//! The named checker, proof method, evidence, assumptions, and verification
//! policy must be evaluated by the appropriate trusted consumer.
//!
//! INTEGRATION
//!
//! - `schema/types.rs` owns `QualifiedName`.
//! - `values/mod.rs` declares and exports this module.
//! - Semantic analysis creates claims and assumptions after name resolution.
//! - `verify/` and domain-specific verifiers consume applicable proof records.
//! - `source_map.rs` or provenance infrastructure owns source-origin mapping.
//! - A proof-checker adapter validates formal proof artifacts.
//! - `serialization/validation.rs` revalidates deserialized records.
//! - Trust and authorization policy remain outside this module.
//!
//! SCALABILITY
//!
//! No closed list of claim types, proof systems, computational domains,
//! evidence kinds, or checker implementations is imposed.
//! Collections are dynamically sized, and operational validation budgets
//! are explicit. Finite memory and representation limits still apply.
//!
//! DETERMINISM
//!
//! Assumptions and dependency identifiers use ordered sets.
//! Claim attributes use ordered maps. Evidence order is preserved because
//! it may be significant to the producing proof system.
//!
//! SEMANTIC SAFETY
//!
//! `Verified` records a producer's declared outcome. It is not a trust
//! decision and does not replace checking the producer, artifact, context,
//! assumptions, or dependencies.
//!
//! FREEZE CRITERIA
//!
//! - The module is declared and exported.
//! - Proof-record construction and revalidation tests pass.
//! - Serialization round-trip tests pass.
//! - Consumers explicitly check proof method, outcome, and trust policy.
//! - Proof dependency resolution detects missing references and cycles.
//! - Workspace compilation and integration tests pass.

use crate::zuir::schema::types::QualifiedName;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

/// Schema version for serialized ZUIR proof records.
///
/// This version is independent of the language version, checker version,
/// and compiler release.
pub const ZUIR_PROOFS_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Validation errors and budgets
// =============================================================================

/// Structural validation failures for proof-related ZUIR values.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProofValidationError {
    /// A required textual value is empty.
    EmptyText {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// A textual value contains a NUL character.
    NulCharacter {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// A textual value contains a forbidden whitespace or control character.
    InvalidText {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// A qualified name failed its schema-level validation.
    InvalidQualifiedName {
        /// Field that contains the invalid name.
        field: &'static str,
        /// Validation explanation.
        message: String,
    },

    /// A record contains a digest algorithm without a digest, or vice versa.
    IncompleteDigest,

    /// A record declares a successful outcome but provides no evidence or
    /// supporting proof dependency.
    MissingVerificationEvidence,

    /// An evidence-count budget was exceeded.
    EvidenceBudgetExceeded {
        /// Actual number of evidence references.
        actual: usize,
        /// Configured limit.
        limit: usize,
    },

    /// A dependency-count budget was exceeded.
    DependencyBudgetExceeded {
        /// Actual number of dependencies.
        actual: usize,
        /// Configured limit.
        limit: usize,
    },

    /// A textual-payload budget was exceeded.
    TextBudgetExceeded {
        /// Actual number of text bytes.
        actual: usize,
        /// Configured limit.
        limit: usize,
    },

    /// A validation counter overflowed.
    CounterOverflow,
}

impl fmt::Display for ProofValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText { field } => {
                write!(f, "{field} must not be empty")
            }
            Self::NulCharacter { field } => {
                write!(f, "{field} must not contain a NUL character")
            }
            Self::InvalidText { field } => {
                write!(
                    f,
                    "{field} contains forbidden whitespace or control characters"
                )
            }
            Self::InvalidQualifiedName { field, message } => {
                write!(f, "invalid {field}: {message}")
            }
            Self::IncompleteDigest => {
                f.write_str("digest algorithm and digest value must be provided together")
            }
            Self::MissingVerificationEvidence => {
                f.write_str(
                    "a verified proof record must reference evidence or a supporting proof",
                )
            }
            Self::EvidenceBudgetExceeded { actual, limit } => {
                write!(f, "proof evidence budget exceeded: {actual}, limit {limit}")
            }
            Self::DependencyBudgetExceeded { actual, limit } => {
                write!(
                    f,
                    "proof dependency budget exceeded: {actual}, limit {limit}"
                )
            }
            Self::TextBudgetExceeded { actual, limit } => {
                write!(f, "proof text budget exceeded: {actual} bytes, limit {limit}")
            }
            Self::CounterOverflow => {
                f.write_str("proof validation counter overflowed")
            }
        }
    }
}

impl Error for ProofValidationError {}

/// Optional operational limits for validating proof records.
///
/// These limits restrict validation work; they do not define the meaning
/// or legal size of a Zamani program.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProofValidationBudget {
    /// Maximum number of evidence references in one record.
    pub max_evidence: Option<usize>,

    /// Maximum number of supporting proof references in one record.
    pub max_dependencies: Option<usize>,

    /// Maximum aggregate UTF-8 bytes in textual fields.
    pub max_text_bytes: Option<usize>,
}

impl ProofValidationBudget {
    /// Construct a budget without local limits.
    ///
    /// Actual processing remains bounded by available system resources.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_evidence: None,
            max_dependencies: None,
            max_text_bytes: None,
        }
    }

    /// Construct a budget with explicitly configured limits.
    #[must_use]
    pub const fn new(
        max_evidence: Option<usize>,
        max_dependencies: Option<usize>,
        max_text_bytes: Option<usize>,
    ) -> Self {
        Self {
            max_evidence,
            max_dependencies,
            max_text_bytes,
        }
    }
}

fn validate_text(
    field: &'static str,
    value: &str,
) -> Result<(), ProofValidationError> {
    if value.is_empty() {
        return Err(ProofValidationError::EmptyText { field });
    }

    if value.contains('\0') {
        return Err(ProofValidationError::NulCharacter { field });
    }

    if value
        .chars()
        .any(|ch| ch.is_whitespace() || ch.is_control())
    {
        return Err(ProofValidationError::InvalidText { field });
    }

    Ok(())
}

fn validate_qualified_name(
    field: &'static str,
    name: &QualifiedName,
) -> Result<(), ProofValidationError> {
    name.validate()
        .map_err(|error| ProofValidationError::InvalidQualifiedName {
            field,
            message: error.to_string(),
        })
}

fn add_count(
    current: usize,
    amount: usize,
) -> Result<usize, ProofValidationError> {
    current
        .checked_add(amount)
        .ok_or(ProofValidationError::CounterOverflow)
}

fn check_budget(
    actual: usize,
    limit: Option<usize>,
    error: impl FnOnce(usize, usize) -> ProofValidationError,
) -> Result<(), ProofValidationError> {
    if let Some(limit) = limit {
        if actual > limit {
            return Err(error(actual, limit));
        }
    }

    Ok(())
}

// =============================================================================
// Proof record identity
// =============================================================================

/// An opaque, validated identity for a proof record.
///
/// This identity is not a content fingerprint and does not automatically
/// remain stable across compiler runs. The enclosing IR owner is responsible
/// for uniqueness within its proof-record collection.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct ProofId(String);

impl ProofId {
    /// Construct a proof identity.
    pub fn new(value: impl Into<String>) -> Result<Self, ProofValidationError> {
        let value = value.into();
        validate_text("proof ID", &value)?;
        Ok(Self(value))
    }

    /// Return the identity as stored.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Revalidate a deserialized identity.
    pub fn validate(&self) -> Result<(), ProofValidationError> {
        validate_text("proof ID", &self.0)
    }
}

impl fmt::Display for ProofId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// =============================================================================
// Claim identity
// =============================================================================

/// A qualified identity for the kind of claim being evaluated.
///
/// The name is open-world. A registered checker or semantic subsystem defines
/// what the claim means; this type does not hard-code claim categories.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct ClaimKind {
    name: QualifiedName,
}

impl ClaimKind {
    /// Construct a claim kind from a qualified name.
    pub fn new(name: QualifiedName) -> Result<Self, ProofValidationError> {
        validate_qualified_name("claim kind", &name)?;
        Ok(Self { name })
    }

    /// Parse a canonical qualified name.
    pub fn parse(value: &str) -> Result<Self, ProofValidationError> {
        let name = QualifiedName::parse(value).map_err(|error| {
            ProofValidationError::InvalidQualifiedName {
                field: "claim kind",
                message: error.to_string(),
            }
        })?;

        Self::new(name)
    }

    /// Borrow the qualified claim identity.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Revalidate the claim kind.
    pub fn validate(&self) -> Result<(), ProofValidationError> {
        validate_qualified_name("claim kind", &self.name)
    }
}

/// A claim about a semantic entity or artifact.
///
/// `subject` identifies what the claim concerns. `kind` identifies the
/// claim's meaning. Attributes provide additional structured parameters.
///
/// A claim is not itself evidence that the claim is true.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofClaim {
    subject: QualifiedName,
    kind: ClaimKind,

    /// Additional claim parameters, ordered by key for deterministic output.
    #[serde(default)]
    attributes: BTreeMap<String, String>,
}

impl ProofClaim {
    /// Construct a claim.
    pub fn new(
        subject: QualifiedName,
        kind: ClaimKind,
        attributes: BTreeMap<String, String>,
    ) -> Result<Self, ProofValidationError> {
        let claim = Self {
            subject,
            kind,
            attributes,
        };

        claim.validate()?;
        Ok(claim)
    }

    /// Borrow the subject identity.
    #[must_use]
    pub fn subject(&self) -> &QualifiedName {
        &self.subject
    }

    /// Borrow the claim kind.
    #[must_use]
    pub fn kind(&self) -> &ClaimKind {
        &self.kind
    }

    /// Borrow the claim parameters.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }

    /// Validate the claim, including after deserialization.
    pub fn validate(&self) -> Result<(), ProofValidationError> {
        validate_qualified_name("claim subject", &self.subject)?;
        self.kind.validate()?;

        for (key, value) in &self.attributes {
            validate_text("claim attribute key", key)?;

            if value.contains('\0') {
                return Err(ProofValidationError::NulCharacter {
                    field: "claim attribute value",
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Proof method and outcome
// =============================================================================

/// The broad method used to produce a proof record.
///
/// These categories describe how a result was obtained; they do not establish
/// trustworthiness or equivalent assurance between different methods.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ProofMethod {
    /// A formal proof artifact checked against a defined proof system.
    Formal,

    /// A result produced by static or abstract analysis.
    StaticAnalysis,

    /// A result produced by executing tests or experiments.
    Empirical,

    /// An externally issued attestation or report.
    Attestation,

    /// A domain-specific method defined by an extension.
    Extension,
}

/// The declared outcome of evaluating a claim.
///
/// `Verified` means that the producing system reports a successful result.
/// Consumers must still decide whether that system, context, assumptions,
/// and evidence satisfy their trust and validation policies.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ProofOutcome {
    /// The producer reports that the claim passed its declared check.
    Verified,

    /// The producer reports that the claim failed its declared check.
    Disproved,

    /// The check did not determine the claim.
    Inconclusive,

    /// No check was performed; the record may describe planned verification.
    NotChecked,
}

// =============================================================================
// Evidence
// =============================================================================

/// A reference to evidence produced or stored elsewhere.
///
/// `locator` is an opaque, non-empty reference interpreted by the relevant
/// evidence resolver. This module does not fetch it or trust its contents.
///
/// If a digest is provided, both its algorithm identity and digest value
/// must be present. Digest syntax and cryptographic verification belong to
/// the corresponding digest implementation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceReference {
    kind: QualifiedName,
    locator: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    digest_algorithm: Option<QualifiedName>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    digest: Option<String>,
}

impl EvidenceReference {
    /// Construct an evidence reference without a digest.
    pub fn new(
        kind: QualifiedName,
        locator: impl Into<String>,
    ) -> Result<Self, ProofValidationError> {
        Self::with_digest(kind, locator, None, None)
    }

    /// Construct an evidence reference with optional paired digest fields.
    pub fn with_digest(
        kind: QualifiedName,
        locator: impl Into<String>,
        digest_algorithm: Option<QualifiedName>,
        digest: Option<String>,
    ) -> Result<Self, ProofValidationError> {
        let evidence = Self {
            kind,
            locator: locator.into(),
            digest_algorithm,
            digest,
        };

        evidence.validate()?;
        Ok(evidence)
    }

    /// Borrow the evidence kind.
    #[must_use]
    pub fn kind(&self) -> &QualifiedName {
        &self.kind
    }

    /// Borrow the opaque evidence locator.
    #[must_use]
    pub fn locator(&self) -> &str {
        &self.locator
    }

    /// Borrow the optional digest algorithm.
    #[must_use]
    pub fn digest_algorithm(&self) -> Option<&QualifiedName> {
        self.digest_algorithm.as_ref()
    }

    /// Borrow the optional digest value.
    #[must_use]
    pub fn digest(&self) -> Option<&str> {
        self.digest.as_deref()
    }

    /// Validate the evidence reference.
    pub fn validate(&self) -> Result<(), ProofValidationError> {
        validate_qualified_name("evidence kind", &self.kind)?;
        validate_text("evidence locator", &self.locator)?;

        match (&self.digest_algorithm, &self.digest) {
            (None, None) => {}
            (Some(algorithm), Some(digest)) => {
                validate_qualified_name("digest algorithm", algorithm)?;
                validate_text("evidence digest", digest)?;
            }
            _ => return Err(ProofValidationError::IncompleteDigest),
        }

        Ok(())
    }

    fn text_bytes(&self) -> Result<usize, ProofValidationError> {
        let mut total = self.locator.len();

        if let Some(algorithm) = &self.digest_algorithm {
            total = add_count(total, algorithm.canonical_string().len())?;
        }

        if let Some(digest) = &self.digest {
            total = add_count(total, digest.len())?;
        }

        total = add_count(total, self.kind.canonical_string().len())?;
        Ok(total)
    }
}

// =============================================================================
// Proof record
// =============================================================================

/// A structured record of a proof or analysis result.
///
/// The record captures the declared result and its context. It is not a
/// substitute for executing the relevant checker or validating referenced
/// artifacts. Consumers must reject stale or untrusted evidence according
/// to their own policies.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofRecord {
    id: ProofId,
    claim: ProofClaim,
    method: ProofMethod,
    outcome: ProofOutcome,
    checker: QualifiedName,
    checker_version: String,

    /// Evidence references in producer-specified order.
    #[serde(default)]
    evidence: Vec<EvidenceReference>,

    /// Assumptions under which the claim was evaluated.
    #[serde(default)]
    assumptions: BTreeSet<QualifiedName>,

    /// References to other proof records required by this record.
    ///
    /// This file validates local identities only. The enclosing proof graph
    /// verifier must check that dependencies exist and are acyclic.
    #[serde(default)]
    dependencies: BTreeSet<ProofId>,

    /// Optional fingerprint of the semantic context used by the producer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    context_fingerprint: Option<String>,

    /// Optional human-readable explanation; not authoritative proof evidence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    explanation: Option<String>,
}

impl ProofRecord {
    /// Construct a proof record and validate its local representation.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ProofId,
        claim: ProofClaim,
        method: ProofMethod,
        outcome: ProofOutcome,
        checker: QualifiedName,
        checker_version: impl Into<String>,
        evidence: Vec<EvidenceReference>,
        assumptions: BTreeSet<QualifiedName>,
        dependencies: BTreeSet<ProofId>,
        context_fingerprint: Option<String>,
        explanation: Option<String>,
    ) -> Result<Self, ProofValidationError> {
        let record = Self {
            id,
            claim,
            method,
            outcome,
            checker,
            checker_version: checker_version.into(),
            evidence,
            assumptions,
            dependencies,
            context_fingerprint,
            explanation,
        };

        record.validate()?;
        Ok(record)
    }

    /// Return the proof-record identity.
    #[must_use]
    pub fn id(&self) -> &ProofId {
        &self.id
    }

    /// Return the evaluated claim.
    #[must_use]
    pub fn claim(&self) -> &ProofClaim {
        &self.claim
    }

    /// Return the declared proof method.
    #[must_use]
    pub fn method(&self) -> ProofMethod {
        self.method
    }

    /// Return the producer-declared outcome.
    #[must_use]
    pub fn outcome(&self) -> ProofOutcome {
        self.outcome
    }

    /// Return the checker identity.
    #[must_use]
    pub fn checker(&self) -> &QualifiedName {
        &self.checker
    }

    /// Return the checker version string.
    #[must_use]
    pub fn checker_version(&self) -> &str {
        &self.checker_version
    }

    /// Return the evidence references.
    #[must_use]
    pub fn evidence(&self) -> &[EvidenceReference] {
        &self.evidence
    }

    /// Return the assumptions.
    #[must_use]
    pub fn assumptions(&self) -> &BTreeSet<QualifiedName> {
        &self.assumptions
    }

    /// Return the supporting proof dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &BTreeSet<ProofId> {
        &self.dependencies
    }

    /// Return the optional context fingerprint.
    #[must_use]
    pub fn context_fingerprint(&self) -> Option<&str> {
        self.context_fingerprint.as_deref()
    }

    /// Return the optional explanation.
    #[must_use]
    pub fn explanation(&self) -> Option<&str> {
        self.explanation.as_deref()
    }

    /// Validate local invariants without imposing explicit operational limits.
    pub fn validate(&self) -> Result<(), ProofValidationError> {
        self.validate_with(&ProofValidationBudget::unlimited())
    }

    /// Validate local invariants and enforce explicit operational budgets.
    ///
    /// Call this after deserializing untrusted records. Serde deserialization
    /// alone does not enforce this module's cross-field invariants.
    pub fn validate_with(
        &self,
        budget: &ProofValidationBudget,
    ) -> Result<(), ProofValidationError> {
        self.id.validate()?;
        self.claim.validate()?;
        validate_qualified_name("proof checker", &self.checker)?;
        validate_text("proof checker version", &self.checker_version)?;

        if let Some(fingerprint) = &self.context_fingerprint {
            validate_text("context fingerprint", fingerprint)?;
        }

        if let Some(explanation) = &self.explanation {
            if explanation.contains('\0') {
                return Err(ProofValidationError::NulCharacter {
                    field: "proof explanation",
                });
            }
        }

        for assumption in &self.assumptions {
            validate_qualified_name("proof assumption", assumption)?;
        }

        for dependency in &self.dependencies {
            dependency.validate()?;

            if dependency == &self.id {
                // Direct self-dependency is always invalid. Longer cycles
                // require whole-graph validation by the proof graph owner.
                return Err(ProofValidationError::InvalidText {
                    field: "proof dependency (self-reference)",
                });
            }
        }

        for evidence in &self.evidence {
            evidence.validate()?;
        }

        if self.outcome == ProofOutcome::Verified
            && self.evidence.is_empty()
            && self.dependencies.is_empty()
        {
            return Err(ProofValidationError::MissingVerificationEvidence);
        }

        check_budget(
            self.evidence.len(),
            budget.max_evidence,
            |actual, limit| ProofValidationError::EvidenceBudgetExceeded {
                actual,
                limit,
            },
        )?;

        check_budget(
            self.dependencies.len(),
            budget.max_dependencies,
            |actual, limit| ProofValidationError::DependencyBudgetExceeded {
                actual,
                limit,
            },
        )?;

        let text_bytes = self.text_bytes()?;

        check_budget(
            text_bytes,
            budget.max_text_bytes,
            |actual, limit| ProofValidationError::TextBudgetExceeded {
                actual,
                limit,
            },
        )?;

        Ok(())
    }

    fn text_bytes(&self) -> Result<usize, ProofValidationError> {
        let mut total = self.id.as_str().len();

        total = add_count(total, self.claim.subject.canonical_string().len())?;
        total = add_count(total, self.claim.kind.name.canonical_string().len())?;
        total = add_count(total, self.checker.canonical_string().len())?;
        total = add_count(total, self.checker_version.len())?;

        for (key, value) in &self.claim.attributes {
            total = add_count(total, key.len())?;
            total = add_count(total, value.len())?;
        }

        for evidence in &self.evidence {
            total = add_count(total, evidence.text_bytes()?)?;
        }

        for assumption in &self.assumptions {
            total = add_count(total, assumption.canonical_string().len())?;
        }

        for dependency in &self.dependencies {
            total = add_count(total, dependency.as_str().len())?;
        }

        if let Some(fingerprint) = &self.context_fingerprint {
            total = add_count(total, fingerprint.len())?;
        }

        if let Some(explanation) = &self.explanation {
            total = add_count(total, explanation.len())?;
        }

        Ok(total)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn qname(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("valid qualified name")
    }

    fn evidence() -> EvidenceReference {
        EvidenceReference::new(qname("testing::report"), "artifact://report/1")
            .expect("valid evidence")
    }

    fn verified_record() -> ProofRecord {
        let claim = ProofClaim::new(
            qname("program::function"),
            ClaimKind::new(qname("type::well_formed")).expect("valid claim kind"),
            BTreeMap::new(),
        )
        .expect("valid claim");

        ProofRecord::new(
            ProofId::new("proof-1").expect("valid ID"),
            claim,
            ProofMethod::Formal,
            ProofOutcome::Verified,
            qname("checker::formal"),
            "1.0",
            vec![evidence()],
            BTreeSet::new(),
            BTreeSet::new(),
            Some("context-fingerprint-1".to_owned()),
            None,
        )
        .expect("valid proof record")
    }

    #[test]
    fn supports_open_world_claim_and_checker_names() {
        let record = verified_record();

        assert_eq!(
            record.claim().kind().name().canonical_string(),
            "type::well_formed"
        );
        assert_eq!(
            record.checker().canonical_string(),
            "checker::formal"
        );
    }

    #[test]
    fn rejects_empty_proof_identity() {
        assert!(matches!(
            ProofId::new(""),
            Err(ProofValidationError::EmptyText { .. })
        ));
    }

    #[test]
    fn verified_record_requires_evidence_or_dependency() {
        let claim = ProofClaim::new(
            qname("program::function"),
            ClaimKind::new(qname("safety::property")).unwrap(),
            BTreeMap::new(),
        )
        .unwrap();

        let result = ProofRecord::new(
            ProofId::new("proof-empty").unwrap(),
            claim,
            ProofMethod::Formal,
            ProofOutcome::Verified,
            qname("checker::formal"),
            "1",
            Vec::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            None,
            None,
        );

        assert!(matches!(
            result,
            Err(ProofValidationError::MissingVerificationEvidence)
        ));
    }

    #[test]
    fn allows_verified_record_with_supporting_proof() {
        let claim = ProofClaim::new(
            qname("program::function"),
            ClaimKind::new(qname("safety::property")).unwrap(),
            BTreeMap::new(),
        )
        .unwrap();

        let mut dependencies = BTreeSet::new();
        dependencies.insert(ProofId::new("supporting-proof").unwrap());

        let record = ProofRecord::new(
            ProofId::new("proof-derived").unwrap(),
            claim,
            ProofMethod::Formal,
            ProofOutcome::Verified,
            qname("checker::formal"),
            "1",
            Vec::new(),
            BTreeSet::new(),
            dependencies,
            None,
            None,
        )
        .expect("supporting proof is present");

        assert_eq!(record.dependencies().len(), 1);
    }

    #[test]
    fn rejects_direct_self_dependency() {
        let claim = ProofClaim::new(
            qname("program::function"),
            ClaimKind::new(qname("safety::property")).unwrap(),
            BTreeMap::new(),
        )
        .unwrap();

        let id = ProofId::new("proof-self").unwrap();
        let mut dependencies = BTreeSet::new();
        dependencies.insert(id.clone());

        let result = ProofRecord::new(
            id,
            claim,
            ProofMethod::Formal,
            ProofOutcome::Verified,
            qname("checker::formal"),
            "1",
            vec![evidence()],
            BTreeSet::new(),
            dependencies,
            None,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_incomplete_digest_metadata() {
        let result = EvidenceReference::with_digest(
            qname("testing::report"),
            "artifact://report/1",
            Some(qname("hash::example")),
            None,
        );

        assert!(matches!(
            result,
            Err(ProofValidationError::IncompleteDigest)
        ));
    }

    #[test]
    fn validation_budget_fails_explicitly() {
        let record = verified_record();
        let budget = ProofValidationBudget::new(Some(0), None, None);

        assert!(matches!(
            record.validate_with(&budget),
            Err(ProofValidationError::EvidenceBudgetExceeded {
                actual: 1,
                limit: 0
            })
        ));
    }

    #[test]
    fn text_budget_fails_explicitly() {
        let record = verified_record();
        let budget = ProofValidationBudget::new(None, None, Some(1));

        assert!(matches!(
            record.validate_with(&budget),
            Err(ProofValidationError::TextBudgetExceeded { .. })
        ));
    }

    #[test]
    fn preserves_distinct_outcomes() {
        assert_ne!(ProofOutcome::Verified, ProofOutcome::Inconclusive);
        assert_ne!(ProofOutcome::Disproved, ProofOutcome::NotChecked);
    }

    #[test]
    fn serde_round_trip_preserves_proof_record() {
        let original = verified_record();

        let encoded =
            serde_json::to_string(&original).expect("serialize proof record");
        let decoded: ProofRecord =
            serde_json::from_str(&encoded).expect("deserialize proof record");

        decoded.validate().expect("revalidate decoded record");
        assert_eq!(decoded, original);
    }
}
