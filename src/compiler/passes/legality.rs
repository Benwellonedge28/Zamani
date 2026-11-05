
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Universal Compiler Pass Legality
//!
//! Path: src/compiler/passes/legality.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Establish a fail-closed legality boundary for compiler passes and
//! proposed transformations across all supported computing domains.
//!
//! AUTHORITY
//!
//! Canonical pass contracts:
//!     crate::zuir::pass::contracts
//!
//! Canonical pass execution:
//!     crate::zuir::pass::manager
//!
//! Canonical pass registration:
//!     crate::zuir::pass::registry
//!
//! Canonical universal IR:
//!     crate::zuir
//!
//! Domain-specific semantic authorities remain in their respective
//! repository subsystems.
//!
//! OWNS
//!
//! - Pass execution prerequisite assessment.
//! - Analysis availability checking.
//! - Transformation preservation contracts.
//! - Extensible semantic-property identifiers.
//! - Preservation evidence validation.
//! - Fail-closed legality assessments.
//! - Separation of semantic legality and target feasibility.
//!
//! DOES NOT OWN
//!
//! - Semantic analysis.
//! - Type inference.
//! - IR construction.
//! - IR verification.
//! - Pass execution.
//! - Pass registration.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Resource allocation.
//! - Physical scheduling.
//! - Runtime execution.
//! - Mathematical proof of arbitrary program equivalence.
//!
//! SCALABILITY
//!
//! No universal pass-count, operation-count, hardware-count,
//! register-width, tensor-rank, memory-capacity, or program-size
//! ceiling is introduced.
//!
//! Collections grow according to available resources.
//!
//! Operational budgets belong to the pass manager.
//!
//! SAFETY
//!
//! Unsafe Rust is forbidden.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::zuir::pass::contracts::{
    PassDescriptor,
    PassError,
    PassErrorKind,
};

// ============================================================================
// LEGALITY POLICY
// ============================================================================

/// Policy controlling compiler pass legality checks.
///
/// Required analyses are always checked.
///
/// Determinism is configurable because some valid transformations
/// legitimately involve explicitly permitted nondeterminism.
///
/// This policy does not prove implementation correctness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LegalityPolicy {
    /// Reject passes that do not declare deterministic behavior.
    pub require_deterministic_passes: bool,
}

// ============================================================================
// ANALYSIS AVAILABILITY
// ============================================================================

/// Analysis information available for a particular IR revision.
///
/// The caller must ensure that the supplied analyses were computed
/// against the exact IR revision being transformed.
///
/// This type records declarations; it does not compute analyses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvailableAnalyses {
    ir_revision: String,
    names: BTreeSet<String>,
}

impl AvailableAnalyses {
    /// Construct an analysis snapshot.
    ///
    /// The revision identifies the exact input IR state.
    pub fn new(
        ir_revision: impl Into<String>,
        names: impl IntoIterator<Item = String>,
    ) -> Result<Self, LegalityAssessmentError> {
        let ir_revision = ir_revision.into();

        if ir_revision.trim().is_empty() {
            return Err(
                LegalityAssessmentError::InvalidIrRevision
            );
        }

        let mut validated = BTreeSet::new();

        for name in names {
            validate_identifier(&name)?;

            if !validated.insert(name.clone()) {
                return Err(
                    LegalityAssessmentError::DuplicateAnalysis(name)
                );
            }
        }

        Ok(Self {
            ir_revision,
            names: validated,
        })
    }

    /// Return the IR revision associated with this snapshot.
    #[must_use]
    pub fn ir_revision(&self) -> &str {
        &self.ir_revision
    }

    /// Check whether an analysis is available.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.names.contains(name)
    }

    /// Iterate over available analyses in deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.names.iter().map(String::as_str)
    }

    /// Return the number of available analyses.
    #[must_use]
    pub fn len(&self) -> usize {
        self.names.len()
    }

    /// Check whether no analyses are available.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn validate_identifier(
    identifier: &str,
) -> Result<(), LegalityAssessmentError> {
    if identifier.trim().is_empty() {
        return Err(
            LegalityAssessmentError::EmptyIdentifier
        );
    }

    if identifier.trim() != identifier {
        return Err(
            LegalityAssessmentError::InvalidIdentifier(
                identifier.to_owned()
            )
        );
    }

    Ok(())
}

// ============================================================================
// PASS LEGALITY
// ============================================================================

/// Successful pass legality assessment.
///
/// This means the declared prerequisites were satisfied.
/// It is not a proof that the implementation is correct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassLegalityReport {
    /// Canonical pass identity.
    pub pass_id: String,

    /// Input IR revision.
    pub ir_revision: String,

    /// Analyses required by the pass.
    pub required_analyses: BTreeSet<String>,

    /// Declared determinism.
    pub declares_determinism: bool,
}

/// Assess whether a pass can legally execute.
///
/// Checks:
///
/// - Canonical descriptor validity.
/// - Analysis availability.
/// - Analysis snapshot revision.
/// - Determinism requirements.
///
/// Missing analyses always fail closed.
///
/// # Errors
///
/// Returns an error for invalid descriptors, stale analysis snapshots,
/// missing analyses, or prohibited nondeterminism.
pub fn check_pass_legality(
    descriptor: &PassDescriptor,
    available: &AvailableAnalyses,
    expected_ir_revision: &str,
    policy: LegalityPolicy,
) -> Result<PassLegalityReport, PassLegalityError> {
    descriptor.validate()?;

    if expected_ir_revision.trim().is_empty() {
        return Err(
            LegalityAssessmentError::InvalidIrRevision.into()
        );
    }

    if available.ir_revision() != expected_ir_revision {
        return Err(
            LegalityAssessmentError::StaleAnalysisSnapshot {
                expected: expected_ir_revision.to_owned(),
                actual: available.ir_revision().to_owned(),
            }
            .into()
        );
    }

    if policy.require_deterministic_passes
        && !descriptor.deterministic
    {
        return Err(PassError::for_pass(
            PassErrorKind::NondeterministicPassRejected,
            descriptor.id.clone(),
            "legality policy requires deterministic passes",
        )
        .into());
    }

    for required in &descriptor.required_analyses {
        if !available.contains(required) {
            return Err(PassError::for_pass(
                PassErrorKind::MissingAnalysis,
                descriptor.id.clone(),
                format!(
                    "required analysis is unavailable: {required}"
                ),
            )
            .into());
        }
    }

    Ok(PassLegalityReport {
        pass_id: descriptor.id.to_string(),
        ir_revision: expected_ir_revision.to_owned(),
        required_analyses: descriptor.required_analyses.clone(),
        declares_determinism: descriptor.deterministic,
    })
}

// ============================================================================
// SEMANTIC PROPERTY IDENTITIES
// ============================================================================

/// Extensible semantic-property identifier.
///
/// Unlike a closed enum, this allows new computing domains to introduce
/// preservation obligations without changing the universal compiler.
///
/// Identifiers should be namespaced.
///
/// Examples:
///
/// - core.observable_behavior
/// - core.effect_ordering
/// - classical.numerical_semantics
/// - quantum.measurement_semantics
/// - quantum.state_transition_semantics
/// - hdl.clock_semantics
/// - hdl.sequential_state
/// - ai.model_output_semantics
/// - distributed.communication_order
/// - distributed.consistency
/// - crypto.constant_time
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemanticPropertyId(String);

impl SemanticPropertyId {
    /// Construct a semantic-property identifier.
    pub fn new(
        identifier: impl Into<String>,
    ) -> Result<Self, LegalityAssessmentError> {
        let identifier = identifier.into();

        validate_identifier(&identifier)?;

        Ok(Self(identifier))
    }

    /// Return the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SemanticPropertyId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// STANDARD SEMANTIC OBLIGATIONS
// ============================================================================

/// Standard cross-domain semantic preservation identifiers.
///
/// Domain-specific transformations must additionally declare every
/// domain-specific obligation relevant to their behavior.
pub mod properties {
    pub const OBSERVABLE_BEHAVIOR: &str =
        "core.observable_behavior";

    pub const EFFECT_ORDERING: &str =
        "core.effect_ordering";

    pub const EXCEPTION_AND_TRAP_BEHAVIOR: &str =
        "core.exception_and_trap_behavior";

    pub const NUMERICAL_SEMANTICS: &str =
        "core.numerical_semantics";

    pub const MEMORY_AND_ALIASING: &str =
        "core.memory_and_aliasing";

    pub const CONTROL_FLOW: &str =
        "core.control_flow";

    pub const CONCURRENCY_SEMANTICS: &str =
        "core.concurrency_semantics";

    pub const EVALUATION_ORDER: &str =
        "core.evaluation_order";

    pub const PROVENANCE: &str =
        "core.provenance";

    pub const SECURITY_PROPERTIES: &str =
        "core.security_properties";

    pub const RESOURCE_REQUIREMENTS: &str =
        "core.resource_requirements";

    pub const QUANTUM_MEASUREMENT: &str =
        "quantum.measurement_semantics";

    pub const QUANTUM_STATE_TRANSITIONS: &str =
        "quantum.state_transition_semantics";

    pub const HDL_TIMING: &str =
        "hdl.timing_semantics";

    pub const HDL_STATE: &str =
        "hdl.sequential_state";

    pub const DISTRIBUTED_ORDERING: &str =
        "distributed.communication_order";

    pub const DISTRIBUTED_CONSISTENCY: &str =
        "distributed.consistency";
}

/// Return the standard cross-domain preservation obligations.
///
/// These are a starting point, not a replacement for the transformation's
/// complete domain-specific contract.
pub fn core_semantic_obligations(
) -> BTreeSet<SemanticPropertyId> {
    [
        properties::OBSERVABLE_BEHAVIOR,
        properties::EFFECT_ORDERING,
        properties::EXCEPTION_AND_TRAP_BEHAVIOR,
        properties::NUMERICAL_SEMANTICS,
        properties::MEMORY_AND_ALIASING,
        properties::CONTROL_FLOW,
        properties::CONCURRENCY_SEMANTICS,
        properties::EVALUATION_ORDER,
        properties::PROVENANCE,
        properties::SECURITY_PROPERTIES,
        properties::RESOURCE_REQUIREMENTS,
    ]
    .into_iter()
    .map(|name| {
        SemanticPropertyId::new(name)
            .expect("built-in property identifier is valid")
    })
    .collect()
}

// ============================================================================
// TRANSFORMATION CONTRACT
// ============================================================================

/// Complete preservation obligations for a transformation.
///
/// Every transformation must explicitly identify the semantic properties
/// relevant to its behavior.
///
/// The contract is declarative and does not itself prove correctness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformationContract {
    /// Canonical identity of the responsible pass.
    pub pass_id: String,

    /// Required preservation properties.
    pub required_properties: BTreeSet<SemanticPropertyId>,
}

impl TransformationContract {
    /// Construct a transformation contract.
    pub fn new(
        pass_id: impl Into<String>,
        required_properties: impl IntoIterator<
            Item = SemanticPropertyId
        >,
    ) -> Result<Self, LegalityAssessmentError> {
        let pass_id = pass_id.into();

        validate_identifier(&pass_id)?;

        let required_properties: BTreeSet<_> =
            required_properties.into_iter().collect();

        if required_properties.is_empty() {
            return Err(
                LegalityAssessmentError::EmptyPreservationContract
            );
        }

        Ok(Self {
            pass_id,
            required_properties,
        })
    }

    /// Construct a contract with standard cross-domain obligations.
    ///
    /// The caller must add additional domain-specific obligations
    /// whenever the transformation requires them.
    pub fn with_core_obligations(
        pass_id: impl Into<String>,
    ) -> Result<Self, LegalityAssessmentError> {
        Self::new(
            pass_id,
            core_semantic_obligations(),
        )
    }

    /// Add a domain-specific preservation obligation.
    pub fn require_property(
        &mut self,
        property: SemanticPropertyId,
    ) {
        self.required_properties.insert(property);
    }
}

// ============================================================================
// PRESERVATION STATUS
// ============================================================================

/// Outcome of a preservation assessment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreservationStatus {
    /// Sufficient evidence was supplied to claim preservation.
    Preserved,

    /// The property is known to be violated.
    Violated,

    /// No sufficient evidence exists.
    Unknown,

    /// The evidence provider failed.
    ///
    /// Provider failure must never be interpreted as preservation.
    ProviderFailed {
        reason: String,
    },
}

// ============================================================================
// PRESERVATION EVIDENCE
// ============================================================================

/// Evidence associated with one semantic property.
///
/// Evidence is a declaration supplied by an implementation or analysis
/// provider. It is not automatically a mathematical proof.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreservationFinding {
    pub status: PreservationStatus,
    pub provider: String,
    pub explanation: String,
}

/// Evidence supplied for a proposed transformation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PreservationEvidence {
    findings: BTreeMap<
        SemanticPropertyId,
        PreservationFinding,
    >,
}

impl PreservationEvidence {
    /// Construct empty evidence.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record evidence for a semantic property.
    ///
    /// Duplicate declarations are rejected.
    pub fn record(
        &mut self,
        property: SemanticPropertyId,
        status: PreservationStatus,
        provider: impl Into<String>,
        explanation: impl Into<String>,
    ) -> Result<(), LegalityAssessmentError> {
        let provider = provider.into();
        let explanation = explanation.into();

        validate_identifier(&provider)?;

        if explanation.trim().is_empty() {
            return Err(
                LegalityAssessmentError::MissingExplanation
            );
        }

        if self.findings.contains_key(&property) {
            return Err(
                LegalityAssessmentError::DuplicateProperty(
                    property
                )
            );
        }

        self.findings.insert(
            property,
            PreservationFinding {
                status,
                provider,
                explanation,
            },
        );

        Ok(())
    }

    /// Retrieve evidence for a property.
    #[must_use]
    pub fn get(
        &self,
        property: &SemanticPropertyId,
    ) -> Option<&PreservationFinding> {
        self.findings.get(property)
    }

    /// Iterate over evidence in deterministic order.
    pub fn iter(
        &self,
    ) -> impl Iterator<
        Item = (&SemanticPropertyId, &PreservationFinding)
    > {
        self.findings.iter()
    }

    /// Return the number of evidence records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.findings.len()
    }

    /// Check whether no evidence exists.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.findings.is_empty()
    }
}

// ============================================================================
// TARGET FEASIBILITY
// ============================================================================

/// Target feasibility is independent of semantic legality.
///
/// A semantically valid transformation may be unavailable on a particular
/// target. That is a realization problem, not automatically a semantic error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetFeasibility {
    /// An authoritative target assessment established support.
    Supported,

    /// The selected target cannot implement the transformation.
    Unsupported {
        reason: String,
    },

    /// No authoritative target assessment has been performed.
    Unknown,
}

// ============================================================================
// LEGALITY ASSESSMENT
// ============================================================================

/// Result of assessing a proposed transformation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegalityAssessment {
    /// Transformation identity.
    pub pass_id: String,

    /// Explicit semantic violations.
    pub violations: BTreeSet<SemanticPropertyId>,

    /// Missing or insufficient evidence.
    pub unknown_properties: BTreeSet<SemanticPropertyId>,

    /// Evidence-provider failures.
    pub provider_failures: BTreeSet<SemanticPropertyId>,

    /// Target feasibility.
    pub target_feasibility: TargetFeasibility,
}

impl LegalityAssessment {
    /// Whether every required semantic property is preserved.
    #[must_use]
    pub fn is_semantically_legal(&self) -> bool {
        self.violations.is_empty()
            && self.unknown_properties.is_empty()
            && self.provider_failures.is_empty()
    }

    /// Whether the transformation is legal for the selected target.
    #[must_use]
    pub fn is_legal_for_target(&self) -> bool {
        self.is_semantically_legal()
            && matches!(
                self.target_feasibility,
                TargetFeasibility::Supported
            )
    }

    /// Whether the assessment contains any preservation failures.
    #[must_use]
    pub fn has_failures(&self) -> bool {
        !self.violations.is_empty()
            || !self.unknown_properties.is_empty()
            || !self.provider_failures.is_empty()
    }
}

// ============================================================================
// ASSESSMENT ERRORS
// ============================================================================

/// Errors produced by legality and evidence validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LegalityAssessmentError {
    EmptyIdentifier,

    InvalidIdentifier(String),

    InvalidIrRevision,

    StaleAnalysisSnapshot {
        expected: String,
        actual: String,
    },

    DuplicateAnalysis(String),

    DuplicateProperty(SemanticPropertyId),

    EmptyPreservationContract,

    MissingExplanation,

    UnexpectedProperty(SemanticPropertyId),
}

impl fmt::Display for LegalityAssessmentError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => {
                write!(formatter, "identifier cannot be empty")
            }

            Self::InvalidIdentifier(value) => {
                write!(
                    formatter,
                    "invalid identifier: {value:?}"
                )
            }

            Self::InvalidIrRevision => {
                write!(
                    formatter,
                    "IR revision cannot be empty"
                )
            }

            Self::StaleAnalysisSnapshot {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "stale analysis snapshot: expected IR revision \
                     {expected:?}, received {actual:?}"
                )
            }

            Self::DuplicateAnalysis(name) => {
                write!(
                    formatter,
                    "duplicate analysis declaration: {name}"
                )
            }

            Self::DuplicateProperty(property) => {
                write!(
                    formatter,
                    "duplicate preservation evidence: {property}"
                )
            }

            Self::EmptyPreservationContract => {
                write!(
                    formatter,
                    "transformation must declare preservation obligations"
                )
            }

            Self::MissingExplanation => {
                write!(
                    formatter,
                    "preservation evidence requires an explanation"
                )
            }

            Self::UnexpectedProperty(property) => {
                write!(
                    formatter,
                    "evidence supplied for an undeclared property: \
                     {property}"
                )
            }
        }
    }
}

impl Error for LegalityAssessmentError {}

/// Complete error type for pass legality.
#[derive(Debug)]
pub enum PassLegalityError {
    Canonical(PassError),
    Assessment(LegalityAssessmentError),
}

impl fmt::Display for PassLegalityError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Canonical(error) => {
                write!(formatter, "{error}")
            }

            Self::Assessment(error) => {
                write!(formatter, "{error}")
            }
        }
    }
}

impl Error for PassLegalityError {}

impl From<PassError> for PassLegalityError {
    fn from(error: PassError) -> Self {
        Self::Canonical(error)
    }
}

impl From<LegalityAssessmentError> for PassLegalityError {
    fn from(error: LegalityAssessmentError) -> Self {
        Self::Assessment(error)
    }
}

// ============================================================================
// TRANSFORMATION ASSESSMENT
// ============================================================================

/// Assess semantic preservation against a complete transformation contract.
///
/// Fail-closed rules:
///
/// - Missing evidence is unknown.
/// - Unknown evidence is not preservation.
/// - Explicit violations reject the transformation.
/// - Provider failures reject the transformation.
/// - Unexpected evidence is rejected.
/// - Duplicate evidence is rejected when recorded.
/// - Target support cannot override semantic violations.
///
/// # Errors
///
/// Returns an error when evidence contains properties not declared
/// by the transformation contract.
pub fn assess_transformation(
    contract: &TransformationContract,
    evidence: &PreservationEvidence,
    target_feasibility: TargetFeasibility,
) -> Result<LegalityAssessment, LegalityAssessmentError> {
    let mut violations = BTreeSet::new();
    let mut unknown_properties = BTreeSet::new();
    let mut provider_failures = BTreeSet::new();

    for property in &contract.required_properties {
        match evidence.get(property) {
            None => {
                unknown_properties.insert(property.clone());
            }

            Some(finding) => {
                match &finding.status {
                    PreservationStatus::Preserved => {}

                    PreservationStatus::Violated => {
                        violations.insert(property.clone());
                    }

                    PreservationStatus::Unknown => {
                        unknown_properties.insert(property.clone());
                    }

                    PreservationStatus::ProviderFailed { .. } => {
                        provider_failures.insert(property.clone());
                    }
                }
            }
        }
    }

    // Unexpected evidence must not be silently ignored.
    for property in evidence.findings.keys() {
        if !contract.required_properties.contains(property) {
            return Err(
                LegalityAssessmentError::UnexpectedProperty(
                    property.clone()
                )
            );
        }
    }

    Ok(LegalityAssessment {
        pass_id: contract.pass_id.clone(),
        violations,
        unknown_properties,
        provider_failures,
        target_feasibility,
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::pass::contracts::{
        PassDescriptor,
        PassId,
        PassVersion,
    };

    fn property(
        name: &str,
    ) -> SemanticPropertyId {
        SemanticPropertyId::new(name)
            .expect("valid property")
    }

    fn contract() -> TransformationContract {
        TransformationContract::new(
            "classical.test",
            [
                property(properties::OBSERVABLE_BEHAVIOR),
                property(properties::NUMERICAL_SEMANTICS),
            ],
        )
        .expect("valid contract")
    }

    fn preserved_evidence() -> PreservationEvidence {
        let mut evidence = PreservationEvidence::new();

        for name in [
            properties::OBSERVABLE_BEHAVIOR,
            properties::NUMERICAL_SEMANTICS,
        ] {
            evidence.record(
                property(name),
                PreservationStatus::Preserved,
                "test.verifier",
                "Preservation established by the test provider",
            )
            .expect("unique evidence");
        }

        evidence
    }

    fn descriptor() -> PassDescriptor {
        PassDescriptor::new(
            PassId::new("classical.test")
                .expect("valid pass ID"),
            PassVersion::new(1, 0, 0),
            "Legality test pass",
        )
    }

    #[test]
    fn missing_evidence_fails_closed() {
        let result = assess_transformation(
            &contract(),
            &PreservationEvidence::new(),
            TargetFeasibility::Supported,
        )
        .expect("assessment should complete");

        assert!(!result.is_semantically_legal());
        assert!(!result.is_legal_for_target());

        assert_eq!(
            result.unknown_properties.len(),
            2
        );
    }

    #[test]
    fn complete_preservation_evidence_is_accepted() {
        let result = assess_transformation(
            &contract(),
            &preserved_evidence(),
            TargetFeasibility::Supported,
        )
        .expect("assessment should complete");

        assert!(result.is_semantically_legal());
        assert!(result.is_legal_for_target());
    }

    #[test]
    fn explicit_violation_is_rejected() {
        let mut evidence = preserved_evidence();

        let property = property(properties::NUMERICAL_SEMANTICS);

        evidence.findings.insert(
            property.clone(),
            PreservationFinding {
                status: PreservationStatus::Violated,
                provider: "test.verifier".to_owned(),
                explanation: "Numerical behavior changed".to_owned(),
            },
        );

        let result = assess_transformation(
            &contract(),
            &evidence,
            TargetFeasibility::Supported,
        )
        .expect("assessment should complete");

        assert!(!result.is_semantically_legal());
        assert!(result.violations.contains(&property));
    }

    #[test]
    fn provider_failure_is_never_preservation() {
        let mut evidence = preserved_evidence();

        let property = property(properties::NUMERICAL_SEMANTICS);

        evidence.findings.insert(
            property.clone(),
            PreservationFinding {
                status: PreservationStatus::ProviderFailed {
                    reason: "analysis provider unavailable".to_owned(),
                },
                provider: "test.verifier".to_owned(),
                explanation: "Provider failed".to_owned(),
            },
        );

        let result = assess_transformation(
            &contract(),
            &evidence,
            TargetFeasibility::Supported,
        )
        .expect("assessment should complete");

        assert!(!result.is_semantically_legal());
        assert!(result.provider_failures.contains(&property));
    }

    #[test]
    fn unknown_target_does_not_establish_target_legality() {
        let result = assess_transformation(
            &contract(),
            &preserved_evidence(),
            TargetFeasibility::Unknown,
        )
        .expect("assessment should complete");

        assert!(result.is_semantically_legal());
        assert!(!result.is_legal_for_target());
    }

    #[test]
    fn unsupported_target_does_not_change_semantic_legality() {
        let result = assess_transformation(
            &contract(),
            &preserved_evidence(),
            TargetFeasibility::Unsupported {
                reason: "Target lacks a required operation".to_owned(),
            },
        )
        .expect("assessment should complete");

        assert!(result.is_semantically_legal());
        assert!(!result.is_legal_for_target());
    }

    #[test]
    fn duplicate_evidence_is_rejected() {
        let mut evidence = PreservationEvidence::new();

        let property = property(properties::OBSERVABLE_BEHAVIOR);

        evidence.record(
            property.clone(),
            PreservationStatus::Preserved,
            "test.provider",
            "First assessment",
        )
        .expect("first record");

        assert_eq!(
            evidence.record(
                property.clone(),
                PreservationStatus::Violated,
                "test.provider",
                "Conflicting assessment",
            ),
            Err(
                LegalityAssessmentError::DuplicateProperty(property)
            )
        );
    }

    #[test]
    fn unexpected_evidence_is_rejected() {
        let mut evidence = preserved_evidence();

        evidence.record(
            property("future_domain.special_semantics"),
            PreservationStatus::Preserved,
            "test.provider",
            "Unexpected evidence",
        )
        .expect("evidence insertion");

        assert!(matches!(
            assess_transformation(
                &contract(),
                &evidence,
                TargetFeasibility::Supported,
            ),
            Err(
                LegalityAssessmentError::UnexpectedProperty(_)
            )
        ));
    }

    #[test]
    fn empty_contract_is_rejected() {
        assert_eq!(
            TransformationContract::new(
                "classical.empty",
                [],
            ),
            Err(
                LegalityAssessmentError::EmptyPreservationContract
            )
        );
    }

    #[test]
    fn custom_domain_properties_are_supported() {
        let mut contract = contract();

        contract.require_property(
            property("neuromorphic.spike_timing_semantics")
        );

        assert!(
            contract.required_properties.contains(
                &property("neuromorphic.spike_timing_semantics")
            )
        );
    }

    #[test]
    fn stale_analysis_snapshot_is_rejected() {
        let available = AvailableAnalyses::new(
            "revision-1",
            vec!["core.control_flow".to_owned()],
        )
        .expect("valid snapshot");

        let result = check_pass_legality(
            &descriptor(),
            &available,
            "revision-2",
            LegalityPolicy::default(),
        );

        assert!(matches!(
            result,
            Err(PassLegalityError::Assessment(
                LegalityAssessmentError::StaleAnalysisSnapshot { .. }
            ))
        ));
    }

    #[test]
    fn required_analysis_must_exist() {
        let mut descriptor = descriptor();

        descriptor.required_analyses.insert(
            "core.control_flow".to_owned()
        );

        let available = AvailableAnalyses::new(
            "revision-1",
            Vec::new(),
        )
        .expect("valid snapshot");

        assert!(matches!(
            check_pass_legality(
                &descriptor,
                &available,
                "revision-1",
                LegalityPolicy::default(),
            ),
            Err(PassLegalityError::Canonical(_))
        ));
    }

    #[test]
    fn valid_pass_prerequisites_are_accepted() {
        let available = AvailableAnalyses::new(
            "revision-1",
            Vec::new(),
        )
        .expect("valid snapshot");

        assert!(
            check_pass_legality(
                &descriptor(),
                &available,
                "revision-1",
                LegalityPolicy::default(),
            )
            .is_ok()
        );
    }

    #[test]
    fn core_obligations_are_nonempty() {
        assert!(!core_semantic_obligations().is_empty());
    }
}
