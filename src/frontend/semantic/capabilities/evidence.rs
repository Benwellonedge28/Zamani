
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Capability evidence for Zamani semantic analysis.
//!
//! File: `src/frontend/semantic/capabilities/evidence.rs`
//! Edition: Rust 2021
//! Minimum toolchain: Rust 1.97+
//! Safety: safe Rust only.
//!
//! # PURPOSE
//!
//! Represent claims and evidence about whether a named capability is
//! supported within a specified execution-environment scope. Provide
//! deterministic storage and conservative evaluation against the canonical
//! semantic `CapabilityRequirement`.
//!
//! # AUTHORITY
//!
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - `src/frontend/semantic/capabilities/model.rs`
//! - `src/frontend/ast/node/capabilities/capability.rs`
//!
//! # OWNERSHIP
//!
//! This file owns:
//! - evidence records and their source descriptions;
//! - the scope in which an evidence claim applies;
//! - evidence status and requirement-evaluation outcomes;
//! - deterministic storage and lookup of evidence records.
//!
//! This file does NOT own:
//! - source syntax, AST nodes, or name resolution;
//! - capability identity, version, or requirement definitions;
//! - authentication, credentials, cryptographic attestation, or trust policy;
//! - hardware discovery, target selection, resource allocation, or scheduling;
//! - quantum operations or any intermediate representation;
//! - a closed list of capabilities, domains, devices, or evidence providers.
//!
//! # IMPORTANT SEMANTIC DISTINCTION
//!
//! A source declaration that names a capability is not proof that a target
//! supports it. Evidence is a scoped claim supplied by an appropriate
//! analysis, manifest, environment description, or downstream provider.
//! This module records and evaluates claims; it does not authenticate them.
//!
//! `Supported` means the evidence source claims support in the specified
//! scope. It is not a security guarantee or proof that the claim is true.
//!
//! # SCALABILITY
//!
//! There are no language-level limits on the number of evidence records,
//! capabilities, subjects, namespaces, or properties. Collections grow with
//! available memory. Compiler-wide operational budgets must be enforced by
//! the owning analysis session, not by changing semantic meaning.
//!
//! Rust collections and strings are necessarily finite, as are actual
//! compiler resources. No finite implementation can promise literal infinity.
//!
//! # DETERMINISM
//!
//! Evidence records are stored in `BTreeSet` order. No timestamps, addresses,
//! random identifiers, global mutable registries, or hardware state are used
//! to determine ordering.
//!
//! # VALIDATION
//!
//! Constructors validate local representation invariants. Serde deserialization
//! can construct values without calling these constructors; callers accepting
//! deserialized or otherwise untrusted values MUST call `validate()` before
//! relying on those invariants.
//!
//! # INTEGRATION CONTRACT
//!
//! `CapabilityId`, `CapabilityVersion`, `CapabilityVersionConstraint`,
//! `CapabilityPropertyValue`, and `CapabilityRequirement` are imported from
//! the existing canonical semantic model. This module must be declared by
//! `capabilities/mod.rs` as `pub mod evidence;`.
//!
//! Requirement evaluation is conservative:
//! - missing evidence is not proof of support or non-support;
//! - missing version/property details can leave a result indeterminate;
//! - contradictory support and non-support claims in the same scope are
//!   reported as conflicting evidence;
//! - this module never decides whether source code is semantically valid
//!   merely because a particular target lacks a capability.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use super::model::{
    CapabilityId, CapabilityModelError, CapabilityPropertyValue, CapabilityRequirement,
    CapabilityVersion, CapabilityVersionConstraint,
};

/// Stable contract version for this module's evidence records.
///
/// This version identifies the logical evidence schema, not the Zamani
/// language version, target version, or provider version.
pub const CAPABILITY_EVIDENCE_SCHEMA_VERSION: u16 = 1;

/// Validate an open-world textual identifier.
///
/// Identifiers are preserved exactly. This function does not normalize case,
/// resolve aliases, or impose an arbitrary length limit.
fn validate_identifier(value: &str, field: &'static str) -> Result<(), CapabilityEvidenceError> {
    if value.trim().is_empty() {
        return Err(CapabilityEvidenceError::EmptyField { field });
    }

    if value.chars().any(char::is_control) {
        return Err(CapabilityEvidenceError::ControlCharacter { field });
    }

    Ok(())
}

/// Identifies the scope to which an evidence claim applies.
///
/// The value is an opaque, open-world identifier, such as an abstract target
/// profile, environment manifest, simulator profile, or deployment scope.
/// This type does not encode a physical device schema.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EvidenceSubject(String);

impl EvidenceSubject {
    /// Construct a validated evidence scope identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, CapabilityEvidenceError> {
        let value = value.into();
        validate_identifier(&value, "subject")?;
        Ok(Self(value))
    }

    /// Return the exact stored scope identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EvidenceSubject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Describes where an evidence claim came from.
///
/// `kind` is an extensible identifier, not a closed enum. `issuer` names the
/// component or authority that supplied the claim. `reference` can identify a
/// manifest entry, analysis result, report, or other externally managed
/// artifact. These strings are descriptive metadata, not proof of authenticity.
///
/// Authentication, signatures, trust decisions, and provenance verification
/// belong to dedicated services.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EvidenceOrigin {
    kind: String,
    issuer: String,
    reference: Option<String>,
}

impl EvidenceOrigin {
    /// Construct a validated description of the evidence source.
    pub fn new(
        kind: impl Into<String>,
        issuer: impl Into<String>,
        reference: Option<String>,
    ) -> Result<Self, CapabilityEvidenceError> {
        let kind = kind.into();
        let issuer = issuer.into();

        validate_identifier(&kind, "origin kind")?;
        validate_identifier(&issuer, "origin issuer")?;

        if let Some(reference) = reference.as_deref() {
            validate_identifier(reference, "origin reference")?;
        }

        Ok(Self {
            kind,
            issuer,
            reference,
        })
    }

    /// Return the open-world origin kind.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Return the origin issuer identifier.
    #[must_use]
    pub fn issuer(&self) -> &str {
        &self.issuer
    }

    /// Return the optional source-artifact reference.
    #[must_use]
    pub fn reference(&self) -> Option<&str> {
        self.reference.as_deref()
    }

    /// Revalidate the representation, including values from deserialization.
    pub fn validate(&self) -> Result<(), CapabilityEvidenceError> {
        validate_identifier(&self.kind, "origin kind")?;
        validate_identifier(&self.issuer, "origin issuer")?;

        if let Some(reference) = self.reference.as_deref() {
            validate_identifier(reference, "origin reference")?;
        }

        Ok(())
    }
}

/// The status claimed by one evidence record.
///
/// These statuses describe the claim, not its authenticity or trustworthiness.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum CapabilityEvidenceStatus {
    /// The origin claims the capability is supported in this scope.
    Supported,

    /// The origin claims the capability is not supported in this scope.
    Unsupported,

    /// The origin cannot establish support or non-support.
    Unknown,
}

/// One scoped claim about a semantic capability.
///
/// A version of `None` means that the origin has not supplied a concrete
/// version. It must not be interpreted as version zero or as proof that every
/// version is available.
///
/// Properties describe evidence associated with the capability. They are
/// generic typed values; domain-specific schemas are validated elsewhere.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CapabilityEvidence {
    capability: CapabilityId,
    subject: EvidenceSubject,
    status: CapabilityEvidenceStatus,
    version: Option<CapabilityVersion>,
    properties: std::collections::BTreeMap<String, CapabilityPropertyValue>,
    origin: EvidenceOrigin,
}

impl CapabilityEvidence {
    /// Construct a validated evidence record.
    ///
    /// Duplicate records can be inserted into [`CapabilityEvidenceSet`]
    /// idempotently. Different origins remain distinct records so contradictory
    /// claims are not silently overwritten.
    pub fn new(
        capability: CapabilityId,
        subject: EvidenceSubject,
        status: CapabilityEvidenceStatus,
        version: Option<CapabilityVersion>,
        properties: std::collections::BTreeMap<String, CapabilityPropertyValue>,
        origin: EvidenceOrigin,
    ) -> Result<Self, CapabilityEvidenceError> {
        let evidence = Self {
            capability,
            subject,
            status,
            version,
            properties,
            origin,
        };

        evidence.validate()?;
        Ok(evidence)
    }

    /// Return the semantic capability identity.
    #[must_use]
    pub fn capability(&self) -> &CapabilityId {
        &self.capability
    }

    /// Return the scope to which the claim applies.
    #[must_use]
    pub fn subject(&self) -> &EvidenceSubject {
        &self.subject
    }

    /// Return the status asserted by this record.
    #[must_use]
    pub const fn status(&self) -> CapabilityEvidenceStatus {
        self.status
    }

    /// Return the reported concrete version, if known.
    #[must_use]
    pub const fn version(&self) -> Option<CapabilityVersion> {
        self.version
    }

    /// Return properties in deterministic key order.
    #[must_use]
    pub fn properties(
        &self,
    ) -> &std::collections::BTreeMap<String, CapabilityPropertyValue> {
        &self.properties
    }

    /// Return the descriptive evidence origin.
    #[must_use]
    pub fn origin(&self) -> &EvidenceOrigin {
        &self.origin
    }

    /// Revalidate local invariants.
    ///
    /// Call this after deserialization or when values came from an untrusted
    /// boundary that did not use the constructor.
    pub fn validate(&self) -> Result<(), CapabilityEvidenceError> {
        self.origin.validate()?;

        for (name, value) in &self.properties {
            validate_identifier(name, "property name")?;
            value
                .validate()
                .map_err(CapabilityEvidenceError::InvalidProperty)?;
        }

        Ok(())
    }

    /// Evaluate this record against a requirement.
    ///
    /// The result is scoped to this record's subject and origin. A single
    /// record cannot resolve contradictions between independent origins.
    #[must_use]
    pub fn evaluate(
        &self,
        subject: &EvidenceSubject,
        requirement: &CapabilityRequirement,
    ) -> EvidenceEvaluation {
        if &self.subject != subject || self.capability != *requirement.capability() {
            return EvidenceEvaluation::NotApplicable;
        }

        match self.status {
            CapabilityEvidenceStatus::Unsupported => {
                EvidenceEvaluation::NotSatisfied
            }
            CapabilityEvidenceStatus::Unknown => EvidenceEvaluation::Indeterminate,
            CapabilityEvidenceStatus::Supported => {
                self.evaluate_supported_requirement(requirement)
            }
        }
    }

    fn evaluate_supported_requirement(
        &self,
        requirement: &CapabilityRequirement,
    ) -> EvidenceEvaluation {
        let version_constraint = requirement.version_constraint();

        if version_constraint != CapabilityVersionConstraint::Any {
            match self.version {
                Some(version) if version_constraint.matches(version) => {}
                Some(_) => return EvidenceEvaluation::NotSatisfied,
                None => return EvidenceEvaluation::Indeterminate,
            }
        }

        let mut missing_property = false;

        for (name, required_value) in requirement.properties() {
            match self.properties.get(name) {
                Some(actual_value) if actual_value == required_value => {}
                Some(_) => return EvidenceEvaluation::NotSatisfied,
                None => missing_property = true,
            }
        }

        if missing_property {
            EvidenceEvaluation::Indeterminate
        } else {
            EvidenceEvaluation::Satisfied
        }
    }
}

/// Result of evaluating evidence against one requirement and scope.
///
/// `NotSatisfied` means the available claim explicitly contradicts the
/// requirement or provides a known incompatible version/property.
/// `Indeterminate` means the record is relevant but lacks information needed
/// to decide. Neither result alone declares the source program semantically
/// invalid; target feasibility is a separate compilation decision.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum EvidenceEvaluation {
    /// This record supports the requirement in the requested scope.
    Satisfied,

    /// This record establishes non-support or a known incompatibility.
    NotSatisfied,

    /// The record is relevant, but required information is missing.
    Indeterminate,

    /// This record does not concern the requested scope or capability.
    NotApplicable,
}

/// Aggregated evaluation of a requirement against a set of evidence records.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum CapabilityEvidenceResult {
    /// No evidence record matched the requested scope and capability.
    NoEvidence,

    /// At least one relevant record supports the requirement, with no
    /// contradictory support/non-support claims in the same scope.
    Satisfied,

    /// Relevant evidence consistently establishes non-support or incompatibility.
    NotSatisfied,

    /// Relevant evidence exists, but it does not contain enough information
    /// to establish satisfaction or non-satisfaction.
    Indeterminate,

    /// At least one origin claims support and another claims non-support for
    /// the same capability and scope. The conflict must not be hidden.
    ConflictingEvidence,
}

/// Deterministic collection of capability evidence.
///
/// The set preserves claims from distinct origins. Exact duplicate records
/// are deduplicated; contradictory claims are retained and surfaced during
/// evaluation.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CapabilityEvidenceSet {
    records: BTreeSet<CapabilityEvidence>,
}

impl CapabilityEvidenceSet {
    /// Create an empty evidence set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the number of distinct evidence records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Return whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Iterate in deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = &CapabilityEvidence> {
        self.records.iter()
    }

    /// Insert one record after validation.
    ///
    /// Returns `true` if the set changed. An exact duplicate is a successful,
    /// idempotent insertion and returns `false`.
    pub fn insert(
        &mut self,
        evidence: CapabilityEvidence,
    ) -> Result<bool, CapabilityEvidenceError> {
        evidence.validate()?;
        Ok(self.records.insert(evidence))
    }

    /// Extend the set transactionally.
    ///
    /// If any record is invalid, the original set remains unchanged.
    pub fn extend<I>(
        &mut self,
        evidence: I,
    ) -> Result<(), CapabilityEvidenceError>
    where
        I: IntoIterator<Item = CapabilityEvidence>,
    {
        let mut next = self.records.clone();

        for record in evidence {
            record.validate()?;
            next.insert(record);
        }

        self.records = next;
        Ok(())
    }

    /// Iterate over records for the given scope and capability.
    pub fn for_capability<'a>(
        &'a self,
        subject: &'a EvidenceSubject,
        capability: &'a CapabilityId,
    ) -> impl Iterator<Item = &'a CapabilityEvidence> + 'a {
        self.records.iter().filter(move |record| {
            record.subject() == subject && record.capability() == capability
        })
    }

    /// Evaluate a requirement using all evidence in one scope.
    ///
    /// Claims about different subjects are never combined. Conflicting
    /// `Supported` and `Unsupported` claims for the same subject and capability
    /// produce `ConflictingEvidence`, regardless of insertion order.
    #[must_use]
    pub fn evaluate(
        &self,
        subject: &EvidenceSubject,
        requirement: &CapabilityRequirement,
    ) -> CapabilityEvidenceResult {
        let matching: Vec<&CapabilityEvidence> = self
            .for_capability(subject, requirement.capability())
            .collect();

        if matching.is_empty() {
            return CapabilityEvidenceResult::NoEvidence;
        }

        let has_supported = matching
            .iter()
            .any(|record| record.status() == CapabilityEvidenceStatus::Supported);
        let has_unsupported = matching
            .iter()
            .any(|record| record.status() == CapabilityEvidenceStatus::Unsupported);

        if has_supported && has_unsupported {
            return CapabilityEvidenceResult::ConflictingEvidence;
        }

        if !has_supported && has_unsupported {
            return CapabilityEvidenceResult::NotSatisfied;
        }

        if !has_supported {
            return CapabilityEvidenceResult::Indeterminate;
        }

        let mut indeterminate = false;
        let mut applicable_supported = false;

        for record in matching {
            if record.status() != CapabilityEvidenceStatus::Supported {
                continue;
            }

            applicable_supported = true;

            match record.evaluate(subject, requirement) {
                EvidenceEvaluation::Satisfied => {
                    return CapabilityEvidenceResult::Satisfied;
                }
                EvidenceEvaluation::Indeterminate => indeterminate = true,
                EvidenceEvaluation::NotSatisfied => {}
                EvidenceEvaluation::NotApplicable => {}
            }
        }

        if indeterminate {
            CapabilityEvidenceResult::Indeterminate
        } else if applicable_supported {
            CapabilityEvidenceResult::NotSatisfied
        } else {
            CapabilityEvidenceResult::NoEvidence
        }
    }

    /// Revalidate every record, including values obtained through deserialization.
    pub fn validate(&self) -> Result<(), CapabilityEvidenceError> {
        for record in &self.records {
            record.validate()?;
        }
        Ok(())
    }
}

/// Errors from evidence construction and local validation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CapabilityEvidenceError {
    /// A required textual field is empty or whitespace-only.
    EmptyField {
        /// Stable field identifier.
        field: &'static str,
    },

    /// A textual field contains a control character.
    ControlCharacter {
        /// Stable field identifier.
        field: &'static str,
    },

    /// A property value failed canonical semantic-model validation.
    InvalidProperty(CapabilityModelError),
}

impl fmt::Display for CapabilityEvidenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(formatter, "capability evidence {field} must not be empty")
            }
            Self::ControlCharacter { field } => {
                write!(
                    formatter,
                    "capability evidence {field} must not contain control characters"
                )
            }
            Self::InvalidProperty(error) => {
                write!(formatter, "invalid capability evidence property: {error}")
            }
        }
    }
}

impl Error for CapabilityEvidenceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidProperty(error) => Some(error),
            Self::EmptyField { .. } | Self::ControlCharacter { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::capabilities::model::{
        CapabilityId, CapabilityPropertyValue, CapabilityRequirement, CapabilityVersion,
        CapabilityVersionConstraint,
    };
    use std::collections::BTreeMap;

    fn capability() -> CapabilityId {
        CapabilityId::new("zamani.quantum", "dynamic_control")
            .expect("test capability should be valid")
    }

    fn subject() -> EvidenceSubject {
        EvidenceSubject::new("test-target-profile")
            .expect("test subject should be valid")
    }

    fn origin(issuer: &str) -> EvidenceOrigin {
        EvidenceOrigin::new("test.fixture", issuer, None)
            .expect("test origin should be valid")
    }

    fn supported(version: Option<CapabilityVersion>) -> CapabilityEvidence {
        CapabilityEvidence::new(
            capability(),
            subject(),
            CapabilityEvidenceStatus::Supported,
            version,
            BTreeMap::new(),
            origin("support-fixture"),
        )
        .expect("test evidence should be valid")
    }

    fn requirement(
        version: CapabilityVersionConstraint,
    ) -> CapabilityRequirement {
        CapabilityRequirement::with_version(capability(), version)
    }

    #[test]
    fn supported_capability_satisfies_unversioned_requirement() {
        let evidence = supported(None);
        let requirement = requirement(CapabilityVersionConstraint::Any);

        assert_eq!(
            evidence.evaluate(&subject(), &requirement),
            EvidenceEvaluation::Satisfied
        );
    }

    #[test]
    fn missing_required_version_is_indeterminate() {
        let evidence = supported(None);
        let requirement = requirement(CapabilityVersionConstraint::AtLeast(
            CapabilityVersion::new(1, 0, 0),
        ));

        assert_eq!(
            evidence.evaluate(&subject(), &requirement),
            EvidenceEvaluation::Indeterminate
        );
    }

    #[test]
    fn incompatible_known_version_does_not_satisfy_requirement() {
        let evidence = supported(Some(CapabilityVersion::new(1, 0, 0)));
        let requirement = requirement(CapabilityVersionConstraint::AtLeast(
            CapabilityVersion::new(2, 0, 0),
        ));

        assert_eq!(
            evidence.evaluate(&subject(), &requirement),
            EvidenceEvaluation::NotSatisfied
        );
    }

    #[test]
    fn property_evidence_is_checked_without_silent_assumptions() {
        let mut properties = BTreeMap::new();
        properties.insert(
            "measurement.mode".to_owned(),
            CapabilityPropertyValue::Text("mid_circuit".to_owned()),
        );

        let evidence = CapabilityEvidence::new(
            capability(),
            subject(),
            CapabilityEvidenceStatus::Supported,
            None,
            properties,
            origin("property-fixture"),
        )
        .expect("evidence should be valid");

        let mut requirement = requirement(CapabilityVersionConstraint::Any);
        requirement
            .add_property(
                "measurement.mode",
                CapabilityPropertyValue::Text("mid_circuit".to_owned()),
            )
            .expect("requirement property should be valid");

        assert_eq!(
            evidence.evaluate(&subject(), &requirement),
            EvidenceEvaluation::Satisfied
        );
    }

    #[test]
    fn missing_required_property_is_indeterminate() {
        let evidence = supported(None);
        let mut requirement = requirement(CapabilityVersionConstraint::Any);
        requirement
            .add_property(
                "measurement.mode",
                CapabilityPropertyValue::Text("mid_circuit".to_owned()),
            )
            .expect("requirement property should be valid");

        assert_eq!(
            evidence.evaluate(&subject(), &requirement),
            EvidenceEvaluation::Indeterminate
        );
    }

    #[test]
    fn unsupported_evidence_is_not_satisfaction() {
        let evidence = CapabilityEvidence::new(
            capability(),
            subject(),
            CapabilityEvidenceStatus::Unsupported,
            None,
            BTreeMap::new(),
            origin("unsupported-fixture"),
        )
        .expect("evidence should be valid");

        assert_eq!(
            evidence.evaluate(
                &subject(),
                &requirement(CapabilityVersionConstraint::Any)
            ),
            EvidenceEvaluation::NotSatisfied
        );
    }

    #[test]
    fn evidence_for_another_subject_is_not_applicable() {
        let evidence = supported(None);
        let other = EvidenceSubject::new("another-profile")
            .expect("other subject should be valid");

        assert_eq!(
            evidence.evaluate(
                &other,
                &requirement(CapabilityVersionConstraint::Any)
            ),
            EvidenceEvaluation::NotApplicable
        );
    }

    #[test]
    fn exact_duplicates_are_idempotent() {
        let record = supported(None);
        let mut set = CapabilityEvidenceSet::new();

        assert!(set.insert(record.clone()).expect("insert should succeed"));
        assert!(!set.insert(record).expect("duplicate insert should succeed"));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn contradictory_claims_are_preserved_and_reported() {
        let mut set = CapabilityEvidenceSet::new();

        set.insert(supported(None))
            .expect("supported record should insert");

        let unsupported = CapabilityEvidence::new(
            capability(),
            subject(),
            CapabilityEvidenceStatus::Unsupported,
            None,
            BTreeMap::new(),
            origin("negative-fixture"),
        )
        .expect("unsupported record should be valid");

        set.insert(unsupported)
            .expect("unsupported record should insert");

        assert_eq!(
            set.evaluate(
                &subject(),
                &requirement(CapabilityVersionConstraint::Any)
            ),
            CapabilityEvidenceResult::ConflictingEvidence
        );
    }

    #[test]
    fn empty_set_reports_no_evidence() {
        let set = CapabilityEvidenceSet::new();

        assert_eq!(
            set.evaluate(
                &subject(),
                &requirement(CapabilityVersionConstraint::Any)
            ),
            CapabilityEvidenceResult::NoEvidence
        );
    }

    #[test]
    fn unknown_evidence_is_indeterminate() {
        let record = CapabilityEvidence::new(
            capability(),
            subject(),
            CapabilityEvidenceStatus::Unknown,
            None,
            BTreeMap::new(),
            origin("unknown-fixture"),
        )
        .expect("unknown evidence should be valid");

        let mut set = CapabilityEvidenceSet::new();
        set.insert(record).expect("insert should succeed");

        assert_eq!(
            set.evaluate(
                &subject(),
                &requirement(CapabilityVersionConstraint::Any)
            ),
            CapabilityEvidenceResult::Indeterminate
        );
    }

    #[test]
    fn empty_subject_and_origin_identifiers_are_rejected() {
        assert!(EvidenceSubject::new("  ").is_err());
        assert!(EvidenceOrigin::new("fixture", "", None).is_err());
        assert!(EvidenceOrigin::new("fixture", "issuer", Some(" \t ".to_owned())).is_err());
    }

    #[test]
    fn evidence_set_iteration_is_deterministic() {
        let first = supported(Some(CapabilityVersion::new(1, 0, 0)));
        let second = CapabilityEvidence::new(
            capability(),
            subject(),
            CapabilityEvidenceStatus::Supported,
            Some(CapabilityVersion::new(2, 0, 0)),
            BTreeMap::new(),
            origin("second-fixture"),
        )
        .expect("second evidence should be valid");

        let mut left = CapabilityEvidenceSet::new();
        left.insert(first.clone()).expect("insert should succeed");
        left.insert(second.clone()).expect("insert should succeed");

        let mut right = CapabilityEvidenceSet::new();
        right.insert(second).expect("insert should succeed");
        right.insert(first).expect("insert should succeed");

        assert_eq!(
            left.iter().cloned().collect::<Vec<_>>(),
            right.iter().cloned().collect::<Vec<_>>()
        );
    }
}
