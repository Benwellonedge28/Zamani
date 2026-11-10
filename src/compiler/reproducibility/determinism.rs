
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler
//!
//! Compiler-wide determinism and reproducibility assessment.
//!
//! Path:
//!     src/compiler/reproducibility/determinism.rs
//!
//! Rust:
//!     Edition: 2021
//!     Minimum supported version: 1.97
//!
//! Safety:
//!     Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module assesses whether the components participating in a compilation
//! declare determinism guarantees sufficient to satisfy the canonical
//! determinism policy.
//!
//! It coordinates determinism declarations across:
//!
//! - Frontend processing.
//! - Semantic analysis.
//! - Dependency analysis.
//! - Canonical IR construction.
//! - IR verification.
//! - Compiler transformations.
//! - Optimization.
//! - Lowering.
//! - Backend realization.
//! - Artifact serialization.
//! - Linking.
//! - Provenance generation.
//!
//! The component inventory is supplied by the compilation architecture.
//! This module does not maintain a closed list of compiler components.
//!
//! ============================================================================
//! CANONICAL AUTHORITIES
//! ============================================================================
//!
//! Determinism policy:
//!
//!     crate::classical::concurrency::determinism
//!
//! Numerical reproducibility:
//!
//!     crate::classical::numeric::reproducibility
//!
//! Parallel compilation ordering:
//!
//!     crate::compiler::parallel::determinism
//!
//! Dependency ordering:
//!
//!     crate::compiler::parallel::dependencies
//!
//! Dependency identity:
//!
//!     crate::compiler::context::dependency_context
//!
//! IR pass determinism:
//!
//!     crate::zuir::pass::determinism
//!
//! Compiler provenance:
//!
//!     crate::compiler::analysis::provenance
//!
//! Build inputs and environment:
//!
//!     crate::compiler::reproducibility::inputs
//!     crate::compiler::reproducibility::environment
//!
//! Artifact metadata:
//!
//!     crate::compiler::artifacts
//!
//! This module integrates with those authorities.
//! It does not replace them.
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! Owns:
//!
//! - ComponentId.
//! - DeterminismObservation.
//! - DeterminismEvidence.
//! - DeterminismFinding.
//! - DeterminismAssessment.
//! - DeterminismAssessmentStatus.
//! - Deterministic compiler-wide assessment.
//! - Assessment coverage validation.
//!
//! Does not own:
//!
//! - Language-level determinism semantics.
//! - Source syntax.
//! - Grammar.
//! - Semantic analysis.
//! - Dependency graph construction.
//! - Compilation-unit identity.
//! - Numerical algorithms.
//! - Incremental fingerprints.
//! - Cryptographic hashing.
//! - Artifact serialization.
//! - Physical scheduling.
//! - Runtime execution.
//! - Distributed consensus.
//!
//! ============================================================================
//! DETERMINISM AND REPRODUCIBILITY
//! ============================================================================
//!
//! Determinism and reproducibility are related but distinct.
//!
//! Determinism:
//!
//!     Identical semantic inputs produce the same permitted observable
//!     behavior under the applicable language contract.
//!
//! Reproducibility:
//!
//!     The stronger contract required to reproduce the relevant compilation
//!     result or execution result under declared equivalent conditions.
//!
//! Deterministic result ordering alone does not establish reproducible builds.
//!
//! Deterministic serialization alone does not establish deterministic
//! compilation.
//!
//! Deterministic compilation alone does not establish deterministic runtime
//! execution.
//!
//! This module assesses declared component guarantees and their evidence
//! references. It does not independently prove that an implementation
//! satisfies those declarations.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! There are no arbitrary universal limits on:
//!
//! - Compiler components.
//! - Computing domains.
//! - Compilation stages.
//! - Optimization passes.
//! - Backend providers.
//! - Targets.
//! - Artifacts.
//! - Dependencies.
//! - Evidence references.
//! - Assessment findings.
//!
//! Component identifiers are dynamically owned strings.
//!
//! Collections use dynamically sized standard-library containers.
//!
//! Assessment storage grows with the number of participating components and
//! their evidence references.
//!
//! Actual memory availability, execution time and host representation limits
//! remain operational constraints.
//!
//! Literal infinite physical resources are not promised.
//!
//! ============================================================================

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::classical::concurrency::determinism::{
    DeterminismMode,
    DeterminismPolicy,
    NumericReproducibility,
    ObservableOrder,
};

/// Public API version of this determinism assessment contract.
///
/// Independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Semantic-model version.
/// - IR version.
/// - Backend version.
/// - Runtime version.
///
/// Increment when incompatible public contract changes are introduced.
pub const DETERMINISM_ASSESSMENT_API_VERSION: u16 = 1;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Dynamically extensible identity of a compiler component.
///
/// Examples:
///
///     frontend.parser
///     frontend.semantic
///     compiler.dependency_analysis
///     compiler.ir_verification
///     compiler.optimizer
///     compiler.quantum_lowering
///     compiler.hdl_lowering
///     compiler.backend
///     compiler.artifact_serialization
///     compiler.linking
///
/// These are examples, not a closed registry.
///
/// New compiler components and computing domains can introduce their own
/// identifiers without modifying this module.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ComponentId(String);

impl ComponentId {
    /// Constructs a component identity.
    ///
    /// Empty identifiers and identifiers containing leading or trailing
    /// whitespace are rejected.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, DeterminismAssessmentError> {
        let value = value.into();

        if value.is_empty() || value.trim() != value {
            return Err(
                DeterminismAssessmentError::InvalidComponentId {
                    value,
                },
            );
        }

        Ok(Self(value))
    }

    /// Returns the component identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ComponentId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// EVIDENCE
// ============================================================================

/// Classification of a determinism evidence reference.
///
/// The evidence is owned and validated by the appropriate subsystem.
///
/// This enum describes evidence provenance. It does not independently
/// authenticate or verify the referenced evidence.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EvidenceKind {
    /// Evidence originating from a documented implementation contract.
    Contract,

    /// Evidence originating from an automated conformance test.
    ConformanceTest,

    /// Evidence originating from an independent reproducibility test.
    IndependentRebuild,

    /// Evidence originating from a formal verification process.
    FormalProof,

    /// Evidence originating from an externally signed attestation.
    SignedAttestation,
}

/// Reference to evidence supporting a component's determinism declaration.
///
/// The reference may identify:
///
/// - A test identifier.
/// - A proof identifier.
/// - A contract identifier.
/// - An attestation identifier.
/// - A build verification record.
///
/// The referenced content is not copied into this structure.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DeterminismEvidence {
    kind: EvidenceKind,
    reference: String,
}

impl DeterminismEvidence {
    /// Creates an evidence reference.
    pub fn new(
        kind: EvidenceKind,
        reference: impl Into<String>,
    ) -> Result<Self, DeterminismAssessmentError> {
        let reference = reference.into();

        if reference.is_empty() || reference.trim() != reference {
            return Err(
                DeterminismAssessmentError::InvalidEvidenceReference {
                    reference,
                },
            );
        }

        Ok(Self {
            kind,
            reference,
        })
    }

    /// Returns the evidence classification.
    #[must_use]
    pub const fn kind(&self) -> EvidenceKind {
        self.kind
    }

    /// Returns the evidence reference.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }
}

// ============================================================================
// COMPONENT OBSERVATIONS
// ============================================================================

/// A compiler component's declared determinism guarantee.
///
/// This deliberately reuses the canonical DeterminismPolicy.
///
/// It does not introduce another determinism policy representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeterminismObservation {
    component: ComponentId,
    guarantee: DeterminismPolicy,
    evidence: Vec<DeterminismEvidence>,
}

impl DeterminismObservation {
    /// Creates a component observation.
    ///
    /// Evidence can subsequently be attached using `add_evidence`.
    #[must_use]
    pub fn new(
        component: ComponentId,
        guarantee: DeterminismPolicy,
    ) -> Self {
        Self {
            component,
            guarantee,
            evidence: Vec::new(),
        }
    }

    /// Returns the component identity.
    #[must_use]
    pub fn component(&self) -> &ComponentId {
        &self.component
    }

    /// Returns the canonical determinism guarantee.
    #[must_use]
    pub const fn guarantee(&self) -> DeterminismPolicy {
        self.guarantee
    }

    /// Returns the evidence references.
    #[must_use]
    pub fn evidence(&self) -> &[DeterminismEvidence] {
        &self.evidence
    }

    /// Adds evidence supporting the component declaration.
    ///
    /// Duplicate evidence references are rejected during assessment.
    pub fn add_evidence(
        &mut self,
        evidence: DeterminismEvidence,
    ) {
        self.evidence.push(evidence);
    }

    /// Adds evidence through a builder-style interface.
    #[must_use]
    pub fn with_evidence(
        mut self,
        evidence: DeterminismEvidence,
    ) -> Self {
        self.add_evidence(evidence);
        self
    }
}

// ============================================================================
// ASSESSMENT FINDINGS
// ============================================================================

/// Individual problem identified during determinism assessment.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DeterminismFindingKind {
    /// An expected compiler component supplied no observation.
    MissingObservation,

    /// The component does not provide the required determinism guarantee.
    InsufficientGuarantee,

    /// The component declares deterministic behavior but provides no
    /// evidence reference.
    MissingEvidence,

    /// The component explicitly declares nondeterministic behavior.
    Nondeterministic,

    /// The component leaves its determinism behavior unspecified.
    Unspecified,

    /// The component relies on an implementation-defined guarantee that
    /// cannot establish the requested reproducibility contract.
    ImplementationDefined,
}

/// One determinism assessment finding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeterminismFinding {
    component: ComponentId,
    kind: DeterminismFindingKind,
}

impl DeterminismFinding {
    fn new(
        component: ComponentId,
        kind: DeterminismFindingKind,
    ) -> Self {
        Self {
            component,
            kind,
        }
    }

    /// Returns the affected component.
    #[must_use]
    pub fn component(&self) -> &ComponentId {
        &self.component
    }

    /// Returns the finding classification.
    #[must_use]
    pub fn kind(&self) -> &DeterminismFindingKind {
        &self.kind
    }
}

// ============================================================================
// ASSESSMENT STATUS
// ============================================================================

/// Overall result of a determinism assessment.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DeterminismAssessmentStatus {
    /// No components were supplied for assessment.
    ///
    /// This must never be interpreted as proof of reproducibility.
    NoScope,

    /// The canonical policy does not request additional determinism.
    NotRequired,

    /// All declared component guarantees satisfy the requested contract,
    /// and the required evidence references are present.
    ///
    /// This is a contract-completeness result, not an independent proof
    /// that the implementation is correct.
    Satisfied,

    /// One or more components do not satisfy the requested contract.
    Unsatisfied,

    /// Deterministic behavior is requested on a best-effort basis.
    ///
    /// Findings remain available to downstream consumers.
    BestEffort,
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors encountered while constructing a determinism assessment.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DeterminismAssessmentError {
    /// Invalid component identifier.
    InvalidComponentId {
        value: String,
    },

    /// Invalid evidence reference.
    InvalidEvidenceReference {
        reference: String,
    },

    /// An expected component was supplied more than once.
    DuplicateExpectedComponent {
        component: ComponentId,
    },

    /// Multiple observations were supplied for the same component.
    DuplicateObservation {
        component: ComponentId,
    },

    /// An observation refers to a component outside the expected inventory.
    UnexpectedObservation {
        component: ComponentId,
    },

    /// Duplicate evidence was supplied for a component.
    DuplicateEvidence {
        component: ComponentId,
        reference: String,
    },
}

impl fmt::Display for DeterminismAssessmentError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidComponentId { value } => {
                write!(
                    formatter,
                    "invalid determinism component identifier: {value:?}"
                )
            }

            Self::InvalidEvidenceReference { reference } => {
                write!(
                    formatter,
                    "invalid determinism evidence reference: {reference:?}"
                )
            }

            Self::DuplicateExpectedComponent { component } => {
                write!(
                    formatter,
                    "duplicate expected determinism component: {component}"
                )
            }

            Self::DuplicateObservation { component } => {
                write!(
                    formatter,
                    "duplicate determinism observation: {component}"
                )
            }

            Self::UnexpectedObservation { component } => {
                write!(
                    formatter,
                    "unexpected determinism observation: {component}"
                )
            }

            Self::DuplicateEvidence {
                component,
                reference,
            } => {
                write!(
                    formatter,
                    "duplicate evidence reference {reference:?} for {component}"
                )
            }
        }
    }
}

impl Error for DeterminismAssessmentError {}

// ============================================================================
// ASSESSMENT
// ============================================================================

/// Immutable compiler-wide determinism assessment.
///
/// Observations and findings are stored in deterministic component order.
///
/// The result is independent of:
///
/// - Observation insertion order.
/// - Hash-map iteration order.
/// - Worker completion order.
/// - Thread identity.
/// - Physical machine topology.
///
/// The expected component inventory must come from the actual compilation
/// pipeline or another authoritative component registry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeterminismAssessment {
    policy: DeterminismPolicy,
    status: DeterminismAssessmentStatus,
    observations: Vec<DeterminismObservation>,
    findings: Vec<DeterminismFinding>,
}

impl DeterminismAssessment {
    /// Assesses compiler component determinism.
    ///
    /// `expected_components`:
    ///
    ///     The complete component inventory participating in this
    ///     compilation.
    ///
    /// `observations`:
    ///
    ///     The determinism guarantees declared by those components.
    ///
    /// Components are not hard-coded into this module.
    ///
    /// Adding a new compiler domain does not require modifying this API.
    ///
    /// A missing observation is reported as a finding rather than silently
    /// treating the component as deterministic.
    pub fn evaluate<E, O>(
        policy: DeterminismPolicy,
        expected_components: E,
        observations: O,
    ) -> Result<Self, DeterminismAssessmentError>
    where
        E: IntoIterator<Item = ComponentId>,
        O: IntoIterator<Item = DeterminismObservation>,
    {
        let mut expected = BTreeSet::new();

        for component in expected_components {
            if !expected.insert(component.clone()) {
                return Err(
                    DeterminismAssessmentError::DuplicateExpectedComponent {
                        component,
                    },
                );
            }
        }

        let mut supplied = BTreeMap::new();

        for observation in observations {
            let component = observation.component.clone();

            if !expected.contains(&component) {
                return Err(
                    DeterminismAssessmentError::UnexpectedObservation {
                        component,
                    },
                );
            }

            if supplied
                .insert(component.clone(), observation)
                .is_some()
            {
                return Err(
                    DeterminismAssessmentError::DuplicateObservation {
                        component,
                    },
                );
            }
        }

        // An empty component inventory is not evidence of determinism.
        if expected.is_empty() {
            return Ok(Self {
                policy,
                status: DeterminismAssessmentStatus::NoScope,
                observations: Vec::new(),
                findings: Vec::new(),
            });
        }

        let mut ordered_observations = Vec::with_capacity(supplied.len());
        let mut findings = Vec::new();

        for component in expected {
            let Some(observation) = supplied.remove(&component) else {
                findings.push(
                    DeterminismFinding::new(
                        component,
                        DeterminismFindingKind::MissingObservation,
                    ),
                );

                continue;
            };

            if policy.requires_determinism()
                || policy.requires_reproducibility()
            {
                Self::assess_observation(
                    &policy,
                    &observation,
                    &mut findings,
                )?;
            }

            ordered_observations.push(observation);
        }

        let status = Self::determine_status(
            policy,
            &findings,
        );

        Ok(Self {
            policy,
            status,
            observations: ordered_observations,
            findings,
        })
    }

    /// Evaluates the guarantees and evidence of one component.
    fn assess_observation(
        required: &DeterminismPolicy,
        observation: &DeterminismObservation,
        findings: &mut Vec<DeterminismFinding>,
    ) -> Result<(), DeterminismAssessmentError> {
        let component = &observation.component;
        let guarantee = observation.guarantee;

        let mut evidence_seen = BTreeSet::new();

        for evidence in &observation.evidence {
            if !evidence_seen.insert(evidence.clone()) {
                return Err(
                    DeterminismAssessmentError::DuplicateEvidence {
                        component: component.clone(),
                        reference: evidence.reference.clone(),
                    },
                );
            }
        }

        if !guarantee_satisfies(required, &guarantee) {
            let kind = if !guarantee.requires_determinism()
                && !guarantee.requires_reproducibility()
            {
                DeterminismFindingKind::Unspecified
            } else {
                DeterminismFindingKind::InsufficientGuarantee
            };

            findings.push(
                DeterminismFinding::new(
                    component.clone(),
                    kind,
                ),
            );
        }

        if guarantee.requires_determinism()
            && observation.evidence.is_empty()
        {
            findings.push(
                DeterminismFinding::new(
                    component.clone(),
                    DeterminismFindingKind::MissingEvidence,
                ),
            );
        }

        Ok(())
    }

    /// Determines the overall assessment status.
    fn determine_status(
        policy: DeterminismPolicy,
        findings: &[DeterminismFinding],
    ) -> DeterminismAssessmentStatus {
        if policy.mode() == DeterminismMode::BestEffort {
            return DeterminismAssessmentStatus::BestEffort;
        }

        if !policy.requires_determinism()
            && !policy.requires_reproducibility()
        {
            return DeterminismAssessmentStatus::NotRequired;
        }

        if findings.is_empty() {
            DeterminismAssessmentStatus::Satisfied
        } else {
            DeterminismAssessmentStatus::Unsatisfied
        }
    }

    /// Returns the canonical determinism policy.
    #[must_use]
    pub const fn policy(&self) -> DeterminismPolicy {
        self.policy
    }

    /// Returns the overall assessment status.
    #[must_use]
    pub const fn status(&self) -> DeterminismAssessmentStatus {
        self.status
    }

    /// Returns whether the declared component contracts satisfy the policy.
    ///
    /// This does not independently prove the correctness of their
    /// implementations.
    #[must_use]
    pub const fn contract_satisfied(&self) -> bool {
        matches!(
            self.status,
            DeterminismAssessmentStatus::Satisfied
                | DeterminismAssessmentStatus::NotRequired
        )
    }

    /// Returns all ordered component observations.
    #[must_use]
    pub fn observations(&self) -> &[DeterminismObservation] {
        &self.observations
    }

    /// Returns all assessment findings.
    #[must_use]
    pub fn findings(&self) -> &[DeterminismFinding] {
        &self.findings
    }

    /// Returns the number of observed components.
    #[must_use]
    pub fn observed_component_count(&self) -> usize {
        self.observations.len()
    }

    /// Returns the number of assessment findings.
    #[must_use]
    pub fn finding_count(&self) -> usize {
        self.findings.len()
    }

    /// Returns whether the assessment contains no findings.
    #[must_use]
    pub fn has_findings(&self) -> bool {
        !self.findings.is_empty()
    }

    /// Looks up a component observation.
    #[must_use]
    pub fn observation(
        &self,
        component: &ComponentId,
    ) -> Option<&DeterminismObservation> {
        self.observations
            .binary_search_by(|observation| {
                observation.component.cmp(component)
            })
            .ok()
            .map(|index| &self.observations[index])
    }
}

// ============================================================================
// POLICY COMPATIBILITY
// ============================================================================

/// Determines whether a component guarantee satisfies a requested policy.
///
/// The canonical DeterminismPolicy remains the sole policy authority.
///
/// This function only evaluates whether the declared guarantees are
/// sufficiently strong.
///
/// It does not execute or prove the component's behavior.
fn guarantee_satisfies(
    required: &DeterminismPolicy,
    provided: &DeterminismPolicy,
) -> bool {
    if required.mode() > provided.mode() {
        return false;
    }

    if !observable_order_satisfies(
        required.observable_order(),
        provided.observable_order(),
    ) {
        return false;
    }

    numeric_policy_satisfies(
        required.numeric_reproducibility(),
        provided.numeric_reproducibility(),
    )
}

/// Checks whether an observable ordering guarantee is sufficiently strong.
fn observable_order_satisfies(
    required: ObservableOrder,
    provided: ObservableOrder,
) -> bool {
    match required {
        ObservableOrder::SemanticDependencies => true,

        ObservableOrder::LogicalKeyOrder => {
            matches!(
                provided,
                ObservableOrder::LogicalKeyOrder
                    | ObservableOrder::SourceOrder
            )
        }

        ObservableOrder::SourceOrder => {
            matches!(
                provided,
                ObservableOrder::SourceOrder
            )
        }
    }
}

/// Checks whether a numerical reproducibility guarantee is sufficiently
/// strong.
///
/// Numerical algorithms and their actual correctness remain the
/// responsibility of the canonical numerical subsystem.
fn numeric_policy_satisfies(
    required: NumericReproducibility,
    provided: NumericReproducibility,
) -> bool {
    use NumericReproducibility::*;

    match required {
        LanguageDefined => true,

        PreserveEvaluationOrder => {
            matches!(
                provided,
                PreserveEvaluationOrder
                    | StableReduction
                    | RequireReproducibleResult
            )
        }

        StableReduction => {
            matches!(
                provided,
                StableReduction
                    | RequireReproducibleResult
            )
        }

        RequireReproducibleResult => {
            matches!(
                provided,
                RequireReproducibleResult
            )
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn component(
        value: &str,
    ) -> ComponentId {
        ComponentId::new(value)
            .expect("valid component identity")
    }

    fn evidence(
        reference: &str,
    ) -> DeterminismEvidence {
        DeterminismEvidence::new(
            EvidenceKind::ConformanceTest,
            reference,
        )
        .expect("valid evidence reference")
    }

    fn observation(
        name: &str,
        policy: DeterminismPolicy,
    ) -> DeterminismObservation {
        DeterminismObservation::new(
            component(name),
            policy,
        )
        .with_evidence(evidence("determinism.conformance"))
    }

    fn evaluate(
        policy: DeterminismPolicy,
        names: &[&str],
        observations: Vec<DeterminismObservation>,
    ) -> DeterminismAssessment {
        DeterminismAssessment::evaluate(
            policy,
            names.iter().map(|name| component(name)),
            observations,
        )
        .expect("assessment should succeed")
    }

    // ------------------------------------------------------------------------
    // Component identity
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_dynamic_component_identifiers() {
        let id = component("future.quantum.backend");

        assert_eq!(
            id.as_str(),
            "future.quantum.backend"
        );
    }

    #[test]
    fn rejects_empty_component_identifiers() {
        assert!(
            ComponentId::new("").is_err()
        );
    }

    #[test]
    fn rejects_noncanonical_component_identifiers() {
        assert!(
            ComponentId::new(" compiler.backend").is_err()
        );

        assert!(
            ComponentId::new("compiler.backend ").is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Empty scope
    // ------------------------------------------------------------------------

    #[test]
    fn empty_scope_never_claims_determinism() {
        let result = DeterminismAssessment::evaluate(
            DeterminismPolicy::reproducible(),
            Vec::<ComponentId>::new(),
            Vec::<DeterminismObservation>::new(),
        )
        .expect("empty scope assessment");

        assert_eq!(
            result.status(),
            DeterminismAssessmentStatus::NoScope
        );

        assert!(!result.contract_satisfied());
    }

    // ------------------------------------------------------------------------
    // Determinism requirements
    // ------------------------------------------------------------------------

    #[test]
    fn deterministic_components_satisfy_deterministic_policy() {
        let result = evaluate(
            DeterminismPolicy::deterministic(),
            &["frontend", "optimizer"],
            vec![
                observation(
                    "optimizer",
                    DeterminismPolicy::deterministic(),
                ),
                observation(
                    "frontend",
                    DeterminismPolicy::deterministic(),
                ),
            ],
        );

        assert_eq!(
            result.status(),
            DeterminismAssessmentStatus::Satisfied
        );

        assert!(result.contract_satisfied());
    }

    #[test]
    fn reproducible_policy_rejects_merely_deterministic_components() {
        let result = evaluate(
            DeterminismPolicy::reproducible(),
            &["frontend"],
            vec![
                observation(
                    "frontend",
                    DeterminismPolicy::deterministic(),
                ),
            ],
        );

        assert_eq!(
            result.status(),
            DeterminismAssessmentStatus::Unsatisfied
        );

        assert!(!result.contract_satisfied());
    }

    #[test]
    fn reproducible_components_satisfy_reproducible_policy() {
        let result = evaluate(
            DeterminismPolicy::reproducible(),
            &["frontend"],
            vec![
                observation(
                    "frontend",
                    DeterminismPolicy::reproducible(),
                ),
            ],
        );

        assert_eq!(
            result.status(),
            DeterminismAssessmentStatus::Satisfied
        );
    }

    #[test]
    fn unspecified_policy_does_not_impose_additional_guarantees() {
        let result = evaluate(
            DeterminismPolicy::unspecified(),
            &["frontend"],
            vec![
                DeterminismObservation::new(
                    component("frontend"),
                    DeterminismPolicy::unspecified(),
                ),
            ],
        );

        assert_eq!(
            result.status(),
            DeterminismAssessmentStatus::NotRequired
        );

        assert!(result.contract_satisfied());
    }

    // ------------------------------------------------------------------------
    // Missing components and evidence
    // ------------------------------------------------------------------------

    #[test]
    fn detects_missing_component_observations() {
        let result = evaluate(
            DeterminismPolicy::deterministic(),
            &["frontend", "optimizer"],
            vec![
                observation(
                    "frontend",
                    DeterminismPolicy::deterministic(),
                ),
            ],
        );

        assert_eq!(
            result.status(),
            DeterminismAssessmentStatus::Unsatisfied
        );

        assert!(
            result.findings().iter().any(|finding| {
                matches!(
                    finding.kind(),
                    DeterminismFindingKind::MissingObservation
                )
            })
        );
    }

    #[test]
    fn detects_unspecified_component_guarantees() {
        let result = evaluate(
            DeterminismPolicy::deterministic(),
            &["frontend"],
            vec![
                DeterminismObservation::new(
                    component("frontend"),
                    DeterminismPolicy::unspecified(),
                ),
            ],
        );

        assert!(
            result.findings().iter().any(|finding| {
                matches!(
                    finding.kind(),
                    DeterminismFindingKind::Unspecified
                )
            })
        );
    }

    #[test]
    fn detects_missing_evidence() {
        let result = evaluate(
            DeterminismPolicy::deterministic(),
            &["frontend"],
            vec![
                DeterminismObservation::new(
                    component("frontend"),
                    DeterminismPolicy::deterministic(),
                ),
            ],
        );

        assert!(
            result.findings().iter().any(|finding| {
                matches!(
                    finding.kind(),
                    DeterminismFindingKind::MissingEvidence
                )
            })
        );
    }

    // ------------------------------------------------------------------------
    // Policy compatibility
    // ------------------------------------------------------------------------

    #[test]
    fn source_order_satisfies_logical_key_order() {
        assert!(
            observable_order_satisfies(
                ObservableOrder::LogicalKeyOrder,
                ObservableOrder::SourceOrder,
            )
        );
    }

    #[test]
    fn logical_key_order_does_not_satisfy_source_order() {
        assert!(
            !observable_order_satisfies(
                ObservableOrder::SourceOrder,
                ObservableOrder::LogicalKeyOrder,
            )
        );
    }

    #[test]
    fn stable_reduction_satisfies_preserved_evaluation_order() {
        assert!(
            numeric_policy_satisfies(
                NumericReproducibility::PreserveEvaluationOrder,
                NumericReproducibility::StableReduction,
            )
        );
    }

    #[test]
    fn stable_reduction_does_not_claim_reproducible_result() {
        assert!(
            !numeric_policy_satisfies(
                NumericReproducibility::RequireReproducibleResult,
                NumericReproducibility::StableReduction,
            )
        );
    }

    // ------------------------------------------------------------------------
    // Deterministic ordering
    // ------------------------------------------------------------------------

    #[test]
    fn observations_are_ordered_independently_of_insertion_order() {
        let first = evaluate(
            DeterminismPolicy::deterministic(),
            &["z", "a", "m"],
            vec![
                observation("z", DeterminismPolicy::deterministic()),
                observation("a", DeterminismPolicy::deterministic()),
                observation("m", DeterminismPolicy::deterministic()),
            ],
        );

        let second = evaluate(
            DeterminismPolicy::deterministic(),
            &["m", "z", "a"],
            vec![
                observation("m", DeterminismPolicy::deterministic()),
                observation("z", DeterminismPolicy::deterministic()),
                observation("a", DeterminismPolicy::deterministic()),
            ],
        );

        assert_eq!(
            first.observations(),
            second.observations()
        );
    }

    #[test]
    fn observation_lookup_uses_canonical_order() {
        let result = evaluate(
            DeterminismPolicy::deterministic(),
            &["z", "a"],
            vec![
                observation("z", DeterminismPolicy::deterministic()),
                observation("a", DeterminismPolicy::deterministic()),
            ],
        );

        assert!(
            result.observation(&component("a")).is_some()
        );

        assert!(
            result.observation(&component("missing")).is_none()
        );
    }

    // ------------------------------------------------------------------------
    // Input validation
    // ------------------------------------------------------------------------

    #[test]
    fn rejects_duplicate_expected_components() {
        let result = DeterminismAssessment::evaluate(
            DeterminismPolicy::deterministic(),
            vec![
                component("frontend"),
                component("frontend"),
            ],
            Vec::<DeterminismObservation>::new(),
        );

        assert!(matches!(
            result,
            Err(
                DeterminismAssessmentError::DuplicateExpectedComponent {
                    ..
                }
            )
        ));
    }

    #[test]
    fn rejects_duplicate_observations() {
        let result = DeterminismAssessment::evaluate(
            DeterminismPolicy::deterministic(),
            vec![component("frontend")],
            vec![
                observation(
                    "frontend",
                    DeterminismPolicy::deterministic(),
                ),
                observation(
                    "frontend",
                    DeterminismPolicy::deterministic(),
                ),
            ],
        );

        assert!(matches!(
            result,
            Err(
                DeterminismAssessmentError::DuplicateObservation {
                    ..
                }
            )
        ));
    }

    #[test]
    fn rejects_unexpected_observations() {
        let result = DeterminismAssessment::evaluate(
            DeterminismPolicy::deterministic(),
            vec![component("frontend")],
            vec![
                observation(
                    "unknown.backend",
                    DeterminismPolicy::deterministic(),
                ),
            ],
        );

        assert!(matches!(
            result,
            Err(
                DeterminismAssessmentError::UnexpectedObservation {
                    ..
                }
            )
        ));
    }

    #[test]
    fn rejects_duplicate_evidence_references() {
        let observation = DeterminismObservation::new(
            component("frontend"),
            DeterminismPolicy::deterministic(),
        )
        .with_evidence(evidence("test.same"))
        .with_evidence(evidence("test.same"));

        let result = DeterminismAssessment::evaluate(
            DeterminismPolicy::deterministic(),
            vec![component("frontend")],
            vec![observation],
        );

        assert!(matches!(
            result,
            Err(
                DeterminismAssessmentError::DuplicateEvidence {
                    ..
                }
            )
        ));
    }

    // ------------------------------------------------------------------------
    // Extensibility
    // ------------------------------------------------------------------------

    #[test]
    fn supports_future_computing_domains_without_registry_changes() {
        let domains = [
            "classical",
            "quantum",
            "hdl",
            "hybrid",
            "distributed",
            "ai",
            "neuromorphic",
            "future.accelerator",
        ];

        let expected = domains
            .iter()
            .map(|domain| {
                component(domain)
            })
            .collect::<Vec<_>>();

        let observations = domains
            .iter()
            .map(|domain| {
                observation(
                    domain,
                    DeterminismPolicy::deterministic(),
                )
            })
            .collect::<Vec<_>>();

        let result = DeterminismAssessment::evaluate(
            DeterminismPolicy::deterministic(),
            expected,
            observations,
        )
        .expect("extensible assessment");

        assert_eq!(
            result.status(),
            DeterminismAssessmentStatus::Satisfied
        );

        assert_eq!(
            result.observed_component_count(),
            domains.len()
        );
    }

    // ------------------------------------------------------------------------
    // Best effort
    // ------------------------------------------------------------------------

    #[test]
    fn best_effort_remains_distinct_from_guaranteed_determinism() {
        let policy = DeterminismPolicy::new(
            DeterminismMode::BestEffort,
            ObservableOrder::SemanticDependencies,
            NumericReproducibility::LanguageDefined,
        );

        let result = evaluate(
            policy,
            &["frontend"],
            vec![
                DeterminismObservation::new(
                    component("frontend"),
                    DeterminismPolicy::unspecified(),
                ),
            ],
        );

        assert_eq!(
            result.status(),
            DeterminismAssessmentStatus::BestEffort
        );

        assert!(!result.contract_satisfied());
    }
}
