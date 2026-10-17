
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Classical Computing — Optimization Legality.
//!
//! Defines conservative, target-independent legality assessments for
//! optimization transformations. This module owns neither IR nor analyses.
//!
//! Authority:
//! - grammar/specification/semantic-model.md
//! - grammar/validation/semantic-boundaries.md
//! - grammar/specification/scalability-model.md
//! - grammar/spec/resources.md
//! - src/zuir/, when selected by the compilation pipeline
//! - src/quantum/ir/classical/, the existing classical IR boundary
//! - src/classical/optimization/passes.rs
//!
//! Missing evidence never establishes legality. A transformation is
//! authorized only when every declared obligation is proven. This module
//! evaluates evidence; it does not independently prove equivalence.
//!
//! No universal maximum is imposed on obligations or extension properties.
//! Actual computation remains bounded by available resources and explicit
//! operational budgets.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

use super::passes::PassId;

/// Result type for legality operations.
pub type LegalityResult<T> = Result<T, LegalityError>;

/// Semantic property that a transformation may need to establish.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum LegalityProperty {
    /// Preserve the program's specified observable meaning.
    SemanticPreservation,
    /// Preserve type and generic-constraint validity.
    TypeCorrectness,
    /// Preserve observable effects and effect ordering.
    Effects,
    /// Preserve memory safety and ownership obligations.
    MemorySafety,
    /// Preserve aliasing and memory-dependence behavior.
    Aliasing,
    /// Preserve control-flow and termination behavior.
    ControlFlow,
    /// Preserve exception, trap, and failure behavior.
    ExceptionsAndTraps,
    /// Preserve integer and overflow semantics.
    IntegerSemantics,
    /// Preserve floating-point and numerical guarantees.
    FloatingPointSemantics,
    /// Preserve concurrency and synchronization semantics.
    Concurrency,
    /// Preserve required determinism and ordering.
    Determinism,
    /// Preserve declared resource intent and constraints.
    ResourceSemantics,
    /// Preserve security and authorization boundaries.
    Security,
    /// Preserve externally visible ABI and interoperability.
    Interoperability,
    /// Do not assume unsupported target capabilities.
    TargetCapabilities,
    /// Extension-defined property.
    Custom(String),
}

impl LegalityProperty {
    /// Constructs an extension-defined property.
    ///
    /// Names must be nonempty and contain no control characters.
    /// There is no universal name-length limit.
    pub fn custom(name: impl Into<String>) -> LegalityResult<Self> {
        let name = name.into();

        if name.trim().is_empty() || name.chars().any(char::is_control) {
            return Err(LegalityError::InvalidPropertyName { name });
        }

        if Self::is_reserved_name(&name) {
            return Err(LegalityError::ReservedPropertyName { name });
        }

        Ok(Self::Custom(name))
    }

    fn is_reserved_name(name: &str) -> bool {
        matches!(
            name,
            "semantic-preservation"
                | "type-correctness"
                | "effects"
                | "memory-safety"
                | "aliasing"
                | "control-flow"
                | "exceptions-and-traps"
                | "integer-semantics"
                | "floating-point-semantics"
                | "concurrency"
                | "determinism"
                | "resource-semantics"
                | "security"
                | "interoperability"
                | "target-capabilities"
        )
    }

    /// Returns the stable textual identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::SemanticPreservation => "semantic-preservation",
            Self::TypeCorrectness => "type-correctness",
            Self::Effects => "effects",
            Self::MemorySafety => "memory-safety",
            Self::Aliasing => "aliasing",
            Self::ControlFlow => "control-flow",
            Self::ExceptionsAndTraps => "exceptions-and-traps",
            Self::IntegerSemantics => "integer-semantics",
            Self::FloatingPointSemantics => "floating-point-semantics",
            Self::Concurrency => "concurrency",
            Self::Determinism => "determinism",
            Self::ResourceSemantics => "resource-semantics",
            Self::Security => "security",
            Self::Interoperability => "interoperability",
            Self::TargetCapabilities => "target-capabilities",
            Self::Custom(name) => name,
        }
    }
}

impl fmt::Display for LegalityProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Evidence for a single legality property.
///
/// `Proven` must mean that an authoritative analysis or trusted rule
/// established the property for the particular transformation and input.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProofEvidence {
    /// A sound analysis or trusted rule established the property.
    Proven {
        /// Explanation of the evidence and its scope.
        rationale: String,
    },
    /// Evidence establishes that the property does not hold.
    Disproven {
        /// Explanation of the counterexample or violated rule.
        rationale: String,
    },
    /// Available information cannot establish or disprove the property.
    Unknown {
        /// Explanation of missing or inconclusive evidence.
        rationale: String,
    },
}

impl ProofEvidence {
    /// Constructs proven evidence.
    #[must_use]
    pub fn proven(rationale: impl Into<String>) -> Self {
        Self::Proven {
            rationale: rationale.into(),
        }
    }

    /// Constructs disproven evidence.
    #[must_use]
    pub fn disproven(rationale: impl Into<String>) -> Self {
        Self::Disproven {
            rationale: rationale.into(),
        }
    }

    /// Constructs unknown evidence.
    #[must_use]
    pub fn unknown(rationale: impl Into<String>) -> Self {
        Self::Unknown {
            rationale: rationale.into(),
        }
    }

    /// Returns true only for explicitly proven evidence.
    #[must_use]
    pub const fn is_proven(&self) -> bool {
        matches!(self, Self::Proven { .. })
    }
}

/// Declared proof obligations for one optimization transformation.
///
/// Semantic preservation is always required. Other obligations must be
/// declared by the transformation or a sound pass-specific policy.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransformationContract {
    pass_id: PassId,
    obligations: BTreeSet<LegalityProperty>,
}

impl TransformationContract {
    /// Creates a transformation contract with mandatory semantic preservation.
    #[must_use]
    pub fn new(pass_id: PassId) -> Self {
        Self {
            pass_id,
            obligations: BTreeSet::from([
                LegalityProperty::SemanticPreservation,
            ]),
        }
    }

    /// Returns the associated optimization-pass identifier.
    #[must_use]
    pub fn pass_id(&self) -> &PassId {
        &self.pass_id
    }

    /// Adds a required proof obligation.
    pub fn require(&mut self, property: LegalityProperty) {
        self.obligations.insert(property);
    }

    /// Adds multiple required proof obligations.
    pub fn require_all(
        &mut self,
        properties: impl IntoIterator<Item = LegalityProperty>,
    ) {
        self.obligations.extend(properties);
    }

    /// Returns all obligations in deterministic order.
    #[must_use]
    pub fn obligations(&self) -> &BTreeSet<LegalityProperty> {
        &self.obligations
    }
}

/// Evidence supplied by canonical analyses for a transformation.
///
/// Evidence producers must ensure that evidence applies to the same IR
/// revision, transformation, configuration, and semantic context.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LegalityEvidence {
    properties: BTreeMap<LegalityProperty, ProofEvidence>,
}

impl LegalityEvidence {
    /// Creates an empty evidence set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Records or replaces evidence for a property.
    pub fn record(
        &mut self,
        property: LegalityProperty,
        evidence: ProofEvidence,
    ) {
        self.properties.insert(property, evidence);
    }

    /// Returns evidence for a property, if supplied.
    #[must_use]
    pub fn get(&self, property: &LegalityProperty) -> Option<&ProofEvidence> {
        self.properties.get(property)
    }

    /// Returns all evidence in deterministic order.
    #[must_use]
    pub fn properties(&self) -> &BTreeMap<LegalityProperty, ProofEvidence> {
        &self.properties
    }
}

/// Overall conclusion from a legality assessment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LegalityStatus {
    /// Every required property has proven evidence.
    Legal,
    /// At least one required property has disproven evidence.
    Illegal,
    /// At least one required property remains unproven.
    Unproven,
}

impl LegalityStatus {
    /// Returns true only when every obligation is proven.
    #[must_use]
    pub const fn is_legal(self) -> bool {
        matches!(self, Self::Legal)
    }
}

impl fmt::Display for LegalityStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Legal => "legal",
            Self::Illegal => "illegal",
            Self::Unproven => "unproven",
        })
    }
}

/// Assessment result for one required property.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LegalityFinding {
    property: LegalityProperty,
    evidence: ProofEvidence,
}

impl LegalityFinding {
    /// Returns the assessed property.
    #[must_use]
    pub fn property(&self) -> &LegalityProperty {
        &self.property
    }

    /// Returns the evidence used in the assessment.
    #[must_use]
    pub fn evidence(&self) -> &ProofEvidence {
        &self.evidence
    }
}

/// Deterministic report of a transformation legality assessment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LegalityReport {
    pass_id: PassId,
    status: LegalityStatus,
    findings: Vec<LegalityFinding>,
}

impl LegalityReport {
    /// Returns the associated pass identifier.
    #[must_use]
    pub fn pass_id(&self) -> &PassId {
        &self.pass_id
    }

    /// Returns the overall assessment.
    #[must_use]
    pub const fn status(&self) -> LegalityStatus {
        self.status
    }

    /// Returns findings in deterministic property order.
    #[must_use]
    pub fn findings(&self) -> &[LegalityFinding] {
        &self.findings
    }

    /// Returns all findings that prevent legal authorization.
    #[must_use]
    pub fn blocking_findings(&self) -> Vec<&LegalityFinding> {
        self.findings
            .iter()
            .filter(|finding| !finding.evidence.is_proven())
            .collect()
    }

    /// Returns success only if every obligation is proven.
    ///
    /// Both illegal and unproven transformations fail closed.
    pub fn require_legal(&self) -> LegalityResult<()> {
        if self.status.is_legal() {
            return Ok(());
        }

        Err(LegalityError::TransformationNotProven {
            pass_id: self.pass_id.clone(),
            status: self.status,
            blocking_properties: self
                .blocking_findings()
                .into_iter()
                .map(|finding| finding.property.clone())
                .collect(),
        })
    }
}

/// Failures while constructing or enforcing a legality contract.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum LegalityError {
    /// An extension property name is invalid.
    InvalidPropertyName {
        /// Rejected name.
        name: String,
    },
    /// An extension property reuses a built-in name.
    ReservedPropertyName {
        /// Rejected name.
        name: String,
    },
    /// A transformation is not proven legal.
    TransformationNotProven {
        /// Associated pass identifier.
        pass_id: PassId,
        /// Overall assessment.
        status: LegalityStatus,
        /// Obligations preventing authorization.
        blocking_properties: Vec<LegalityProperty>,
    },
}

impl fmt::Display for LegalityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPropertyName { name } => {
                write!(f, "invalid legality property name: {name:?}")
            }
            Self::ReservedPropertyName { name } => {
                write!(f, "reserved legality property name: {name:?}")
            }
            Self::TransformationNotProven {
                pass_id,
                status,
                blocking_properties,
            } => {
                let properties = blocking_properties
                    .iter()
                    .map(LegalityProperty::as_str)
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(
                    f,
                    "optimization pass {pass_id} is {status}; \
                     blocking properties: {properties}"
                )
            }
        }
    }
}

impl Error for LegalityError {}

/// Assesses every obligation declared by a transformation.
///
/// Missing evidence becomes `Unknown`. Any disproven obligation makes the
/// result `Illegal`; otherwise, any unknown obligation makes it `Unproven`.
/// Only an all-proven obligation set produces `Legal`.
///
/// This function is pure and deterministic. It neither mutates IR nor runs
/// analyses, inspects hardware, or independently proves equivalence.
#[must_use]
pub fn assess_legality(
    contract: &TransformationContract,
    evidence: &LegalityEvidence,
) -> LegalityReport {
    let mut findings = Vec::with_capacity(contract.obligations.len());
    let mut has_disproven = false;
    let mut has_unknown = false;

    for property in &contract.obligations {
        let proof = evidence.get(property).cloned().unwrap_or_else(|| {
            ProofEvidence::unknown(
                "no evidence was supplied for this obligation",
            )
        });

        // Borrow the proof here: the complete evidence must also be retained
        // in the report below.
        match &proof {
            ProofEvidence::Proven { .. } => {}
            ProofEvidence::Disproven { .. } => has_disproven = true,
            ProofEvidence::Unknown { .. } => has_unknown = true,
        }

        findings.push(LegalityFinding {
            property: property.clone(),
            evidence: proof,
        });
    }

    let status = if has_disproven {
        LegalityStatus::Illegal
    } else if has_unknown {
        LegalityStatus::Unproven
    } else {
        LegalityStatus::Legal
    };

    LegalityReport {
        pass_id: contract.pass_id.clone(),
        status,
        findings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pass_id(value: &str) -> PassId {
        PassId::new(value).expect("valid test identifier")
    }

    #[test]
    fn semantic_preservation_is_mandatory() {
        let contract = TransformationContract::new(pass_id("test.pass"));

        assert!(contract
            .obligations()
            .contains(&LegalityProperty::SemanticPreservation));
    }

    #[test]
    fn all_proven_obligations_are_legal() {
        let mut contract = TransformationContract::new(pass_id("test.fold"));
        contract.require(LegalityProperty::IntegerSemantics);
        contract.require(LegalityProperty::Effects);

        let mut evidence = LegalityEvidence::new();
        evidence.record(
            LegalityProperty::SemanticPreservation,
            ProofEvidence::proven("equivalence established"),
        );
        evidence.record(
            LegalityProperty::IntegerSemantics,
            ProofEvidence::proven("overflow behavior preserved"),
        );
        evidence.record(
            LegalityProperty::Effects,
            ProofEvidence::proven("observable effects preserved"),
        );

        let report = assess_legality(&contract, &evidence);

        assert_eq!(report.status(), LegalityStatus::Legal);
        assert!(report.require_legal().is_ok());
        assert_eq!(report.findings().len(), 3);
    }

    #[test]
    fn missing_evidence_fails_closed() {
        let contract = TransformationContract::new(pass_id("test.pass"));
        let report = assess_legality(&contract, &LegalityEvidence::new());

        assert_eq!(report.status(), LegalityStatus::Unproven);
        assert!(report.require_legal().is_err());
        assert_eq!(report.blocking_findings().len(), 1);
    }

    #[test]
    fn disproven_semantics_make_a_transformation_illegal() {
        let contract = TransformationContract::new(pass_id("test.dce"));
        let mut evidence = LegalityEvidence::new();

        evidence.record(
            LegalityProperty::SemanticPreservation,
            ProofEvidence::disproven(
                "removed write is externally observable",
            ),
        );

        let report = assess_legality(&contract, &evidence);

        assert_eq!(report.status(), LegalityStatus::Illegal);
        assert!(report.require_legal().is_err());
    }

    #[test]
    fn unknown_floating_point_policy_blocks_reassociation() {
        let mut contract =
            TransformationContract::new(pass_id("test.reassociate"));
        contract.require(LegalityProperty::FloatingPointSemantics);

        let mut evidence = LegalityEvidence::new();
        evidence.record(
            LegalityProperty::SemanticPreservation,
            ProofEvidence::proven("applicable semantic proof supplied"),
        );
        evidence.record(
            LegalityProperty::FloatingPointSemantics,
            ProofEvidence::unknown("numerical policy unresolved"),
        );

        assert_eq!(
            assess_legality(&contract, &evidence).status(),
            LegalityStatus::Unproven
        );
    }

    #[test]
    fn disproven_evidence_takes_precedence_over_unknown() {
        let mut contract =
            TransformationContract::new(pass_id("test.parallelize"));
        contract.require(LegalityProperty::Concurrency);
        contract.require(LegalityProperty::Effects);

        let mut evidence = LegalityEvidence::new();
        evidence.record(
            LegalityProperty::SemanticPreservation,
            ProofEvidence::proven("proof supplied"),
        );
        evidence.record(
            LegalityProperty::Concurrency,
            ProofEvidence::disproven("transformation introduces a data race"),
        );

        assert_eq!(
            assess_legality(&contract, &evidence).status(),
            LegalityStatus::Illegal
        );
    }

    #[test]
    fn custom_properties_are_extensible_and_validated() {
        assert!(LegalityProperty::custom("domain.new-property").is_ok());
        assert!(LegalityProperty::custom("").is_err());
        assert!(LegalityProperty::custom("bad\nproperty").is_err());
        assert!(LegalityProperty::custom("effects").is_err());
    }

    #[test]
    fn findings_are_deterministically_ordered() {
        let mut contract = TransformationContract::new(pass_id("test.order"));
        contract.require(LegalityProperty::Security);
        contract.require(LegalityProperty::Aliasing);
        contract.require(LegalityProperty::Effects);

        let report = assess_legality(&contract, &LegalityEvidence::new());

        let names: Vec<&str> = report
            .findings()
            .iter()
            .map(|finding| finding.property().as_str())
            .collect();

        let mut sorted = names.clone();
        sorted.sort_unstable();

        assert_eq!(names, sorted);
    }

    #[test]
    fn unrelated_evidence_does_not_prove_semantic_preservation() {
        let contract = TransformationContract::new(pass_id("test.pass"));
        let mut evidence = LegalityEvidence::new();

        evidence.record(
            LegalityProperty::Effects,
            ProofEvidence::proven("effects preserved"),
        );

        let report = assess_legality(&contract, &evidence);

        assert_eq!(report.status(), LegalityStatus::Unproven);
        assert!(report.findings().iter().any(|finding| {
            finding.property() == &LegalityProperty::SemanticPreservation
        }));
    }
}
