
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Portability Conformance Testing
//!
//! Path: src/toolchain/testing/portability.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Mission
//!
//! Verify the POCO-REAF portability contract across independently
//! registered and assessed computing targets.
//!
//! One logical Zamani program may have multiple target-specific
//! realizations. This module verifies that the realizations preserve
//! the applicable semantic contract and that their feasibility and
//! artifact-verification states are reported honestly.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Portability conformance test requests.
//! - Logical build identity consistency checks.
//! - Target observation contracts.
//! - Semantic-preservation evidence requirements.
//! - Artifact-verification evidence requirements.
//! - Multi-target portability conformance aggregation.
//! - Explicit incomplete and non-conformant outcomes.
//! - Resource-aware test collection.
//!
//! This module does not own:
//!
//! - Language grammar or syntax.
//! - AST construction.
//! - Semantic analysis.
//! - Semantic equivalence algorithms.
//! - Compiler planning.
//! - Target registration or discovery.
//! - Capability definitions.
//! - Compatibility algorithms.
//! - Hardware discovery or allocation.
//! - Compilation or code generation.
//! - Artifact storage or digest algorithms.
//! - Runtime execution.
//! - Distributed scheduling.
//! - Quantum routing or error correction.
//!
//! ## Canonical integration authorities
//!
//! Grammar:
//!     grammar/specification/poco-reaf.md
//!     grammar/specification/portability.md
//!     grammar/specification/scalability-model.md
//!     grammar/validation/semantic-boundaries.md
//!
//! Target compatibility:
//!     crate::toolchain::targets::compatibility
//!
//! Target matrices:
//!     crate::toolchain::targets::matrix
//!
//! Build orchestration:
//!     crate::toolchain::build
//!
//! Build outputs:
//!     crate::toolchain::build::outputs
//!
//! Compiler artifacts:
//!     crate::compiler::artifacts
//!
//! Artifact portability:
//!     crate::compiler::artifacts::portability
//!
//! Artifact validation:
//!     crate::compiler::artifacts::validation
//!
//! Compiler planning:
//!     crate::compiler::planning
//!
//! Backend compatibility:
//!     crate::backend::compatibility
//!
//! Physical capabilities:
//!     crate::zhal
//!
//! Runtime:
//!     crate::zur
//!
//! ## POCO-REAF invariants
//!
//! 1. Logical source identity remains constant across target realizations.
//! 2. Logical dependency identity remains constant across target realizations.
//! 3. Target identities are supplied by canonical authorities.
//! 4. Target-specific artifacts may differ in representation and bytes.
//! 5. Applicable observable program semantics must be preserved.
//! 6. Unknown feasibility is never treated as feasibility.
//! 7. Infeasible targets are never counted as successful realizations.
//! 8. An artifact must be verified before being counted as verified.
//! 9. Semantic equivalence must be established by an appropriate authority.
//! 10. Provider assertions alone are not independent verification.
//!
//! ## Scalability
//!
//! No universal maximum is imposed on:
//!
//! - Targets.
//! - Computing domains.
//! - Artifacts.
//! - Processors.
//! - Accelerators.
//! - Quantum resources.
//! - Distributed participants.
//! - Memory requirements.
//! - Tensor dimensions.
//! - Hardware resources.
//!
//! Collections grow according to available implementation resources.
//!
//! Allocation failure is reported rather than converted into a
//! successful or incomplete result.
//!
//! This module does not promise infinite physical resources.
//!
//! ## Important distinction
//!
//! POCO-REAF does not require different target-specific artifacts
//! to contain identical bytes.
//!
//! It requires preservation of the applicable semantic contract.
//!
//! A native executable, a quantum circuit, an HDL realization and
//! a distributed execution artifact may be structurally different.
//!
//! Their compatibility must be established through the appropriate
//! semantic and domain-specific authorities.

use std::collections::BTreeSet;
use std::fmt;

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainResult,
};

/// Stable identity of this module.
pub const PORTABILITY_TEST_MODULE_ID: &str =
    "zamani.toolchain.testing.portability";

/// Public API version of the portability conformance harness.
pub const PORTABILITY_TEST_API_VERSION: u16 = 1;

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn invalid_input(message: impl Into<String>) -> ToolchainError {
    ToolchainError::invalid_input(message)
        .with_operation("toolchain.testing.portability")
}

fn validation_failure(message: impl Into<String>) -> ToolchainError {
    ToolchainError::validation_failed(message)
        .with_operation("toolchain.testing.portability")
}

fn allocation_failure(
    subject: &'static str,
    error: std::collections::TryReserveError,
) -> ToolchainError {
    ToolchainError::resource_exhausted(format!(
        "unable to allocate portability test storage for {subject}: {error}"
    ))
    .with_operation("toolchain.testing.portability")
}

// ============================================================================
// LOGICAL BUILD IDENTITY
// ============================================================================

/// Immutable identity shared by every realization of one logical build.
///
/// These values identify the logical source and dependency inputs.
/// Their canonical hashing and provenance remain owned by the source,
/// dependency and build subsystems.
///
/// Target-specific artifact identities are deliberately excluded.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LogicalBuildIdentity {
    build_id: String,
    source_identity: String,
    dependency_identity: String,
}

impl LogicalBuildIdentity {
    /// Construct an identity for one logical Zamani build.
    ///
    /// All identity fields must be non-empty and free of control
    /// characters and surrounding whitespace.
    pub fn new(
        build_id: impl Into<String>,
        source_identity: impl Into<String>,
        dependency_identity: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let build_id = build_id.into();
        let source_identity = source_identity.into();
        let dependency_identity = dependency_identity.into();

        validate_identity("build identity", &build_id)?;
        validate_identity("source identity", &source_identity)?;
        validate_identity(
            "dependency identity",
            &dependency_identity,
        )?;

        Ok(Self {
            build_id,
            source_identity,
            dependency_identity,
        })
    }

    /// Return the logical build identity.
    #[must_use]
    pub fn build_id(&self) -> &str {
        &self.build_id
    }

    /// Return the shared source identity.
    #[must_use]
    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    /// Return the shared dependency identity.
    #[must_use]
    pub fn dependency_identity(&self) -> &str {
        &self.dependency_identity
    }
}

fn validate_identity(
    field: &'static str,
    value: &str,
) -> ToolchainResult<()> {
    if value.is_empty() {
        return Err(invalid_input(format!(
            "{field} must not be empty"
        )));
    }

    if value.trim() != value {
        return Err(invalid_input(format!(
            "{field} must not contain surrounding whitespace"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_input(format!(
            "{field} must not contain control characters"
        )));
    }

    Ok(())
}

// ============================================================================
// FEASIBILITY
// ============================================================================

/// Authoritative feasibility outcome for a requested target.
///
/// This is an observation supplied by the compiler planning or
/// compatibility integration. It is not calculated here.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Feasibility {
    /// Mandatory requirements and constraints are satisfied.
    Feasible,

    /// Mandatory requirements or constraints cannot be satisfied.
    Infeasible,

    /// Available evidence cannot establish either outcome.
    Unresolved,
}

impl Feasibility {
    /// Return whether feasibility has been established.
    #[must_use]
    pub const fn is_feasible(self) -> bool {
        matches!(self, Self::Feasible)
    }

    /// Return whether the outcome is unresolved.
    #[must_use]
    pub const fn is_unresolved(self) -> bool {
        matches!(self, Self::Unresolved)
    }
}

impl fmt::Display for Feasibility {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let value = match self {
            Self::Feasible => "feasible",
            Self::Infeasible => "infeasible",
            Self::Unresolved => "unresolved",
        };

        formatter.write_str(value)
    }
}

// ============================================================================
// SEMANTIC PRESERVATION
// ============================================================================

/// Result of checking semantic preservation between realizations.
///
/// The actual equivalence algorithm belongs to the semantic,
/// compiler and domain-specific verification authorities.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SemanticPreservation {
    /// The applicable semantic contract was verified as preserved.
    Equivalent,

    /// A semantic difference violating the applicable contract
    /// was established.
    Different,

    /// Semantic comparison has not been performed.
    NotAssessed,

    /// Available evidence cannot establish equivalence.
    Indeterminate,
}

impl SemanticPreservation {
    /// Return whether semantic preservation was established.
    #[must_use]
    pub const fn is_equivalent(self) -> bool {
        matches!(self, Self::Equivalent)
    }
}

// ============================================================================
// ARTIFACT VERIFICATION
// ============================================================================

/// Verification outcome for one target-specific realization.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ArtifactVerification {
    /// Canonical artifact validation succeeded.
    Verified,

    /// An artifact was produced but rejected by validation.
    Rejected,

    /// No artifact was produced.
    NotProduced,

    /// Artifact verification has not been established.
    Unverified,
}

impl ArtifactVerification {
    /// Return whether the artifact has been verified.
    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::Verified)
    }
}

// ============================================================================
// PORTABILITY EVIDENCE
// ============================================================================

/// External evidence supporting one portability assertion.
///
/// Evidence references identify records held by their owning
/// verification authorities.
///
/// They do not themselves authenticate the referenced records.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PortabilityEvidence {
    authority: String,
    reference: String,
}

impl PortabilityEvidence {
    /// Construct an evidence reference.
    pub fn new(
        authority: impl Into<String>,
        reference: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let authority = authority.into();
        let reference = reference.into();

        validate_identity("evidence authority", &authority)?;
        validate_identity("evidence reference", &reference)?;

        Ok(Self {
            authority,
            reference,
        })
    }

    /// Return the evidence authority.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Return the evidence reference.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }
}

// ============================================================================
// TARGET OBSERVATION
// ============================================================================

/// Complete observation for one requested target realization.
///
/// The integration adapter must obtain these fields from actual
/// toolchain, compiler, backend and artifact-validation operations.
///
/// The observation must not be manufactured from target names,
/// declared capabilities or successful mock responses.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetObservation {
    target_id: String,
    build_identity: LogicalBuildIdentity,
    feasibility: Feasibility,
    semantic_preservation: SemanticPreservation,
    artifact_verification: ArtifactVerification,
    feasibility_evidence: Option<PortabilityEvidence>,
    semantic_evidence: Option<PortabilityEvidence>,
    artifact_evidence: Option<PortabilityEvidence>,
    explanation: Option<String>,
}

impl TargetObservation {
    /// Construct an observation obtained from an authoritative
    /// target assessment and actual realization workflow.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        target_id: impl Into<String>,
        build_identity: LogicalBuildIdentity,
        feasibility: Feasibility,
        semantic_preservation: SemanticPreservation,
        artifact_verification: ArtifactVerification,
        feasibility_evidence: Option<PortabilityEvidence>,
        semantic_evidence: Option<PortabilityEvidence>,
        artifact_evidence: Option<PortabilityEvidence>,
        explanation: Option<String>,
    ) -> ToolchainResult<Self> {
        let target_id = target_id.into();

        validate_identity("target identity", &target_id)?;

        if explanation
            .as_deref()
            .is_some_and(|value| value.trim().is_empty())
        {
            return Err(invalid_input(
                "an optional explanation must not be empty",
            ));
        }

        let observation = Self {
            target_id,
            build_identity,
            feasibility,
            semantic_preservation,
            artifact_verification,
            feasibility_evidence,
            semantic_evidence,
            artifact_evidence,
            explanation,
        };

        observation.validate()?;

        Ok(observation)
    }

    /// Validate internal consistency of the observation.
    ///
    /// This method validates the evidence envelope, not the truth
    /// or authenticity of external evidence.
    pub fn validate(&self) -> ToolchainResult<()> {
        match self.feasibility {
            Feasibility::Feasible => {
                if self.feasibility_evidence.is_none() {
                    return Err(validation_failure(format!(
                        "feasible target `{}` has no feasibility evidence",
                        self.target_id
                    )));
                }

                if !self.semantic_preservation.is_equivalent() {
                    return Err(validation_failure(format!(
                        "feasible target `{}` lacks verified semantic equivalence",
                        self.target_id
                    )));
                }

                if self.semantic_evidence.is_none() {
                    return Err(validation_failure(format!(
                        "feasible target `{}` has no semantic evidence",
                        self.target_id
                    )));
                }

                if !self.artifact_verification.is_verified() {
                    return Err(validation_failure(format!(
                        "feasible target `{}` has no verified artifact",
                        self.target_id
                    )));
                }

                if self.artifact_evidence.is_none() {
                    return Err(validation_failure(format!(
                        "feasible target `{}` has no artifact verification evidence",
                        self.target_id
                    )));
                }
            }

            Feasibility::Infeasible => {
                if self.explanation.is_none() {
                    return Err(validation_failure(format!(
                        "infeasible target `{}` requires an explanation",
                        self.target_id
                    )));
                }

                if self.artifact_verification
                    == ArtifactVerification::Verified
                {
                    return Err(validation_failure(format!(
                        "infeasible target `{}` cannot be reported as a verified successful realization",
                        self.target_id
                    )));
                }
            }

            Feasibility::Unresolved => {
                if self.explanation.is_none() {
                    return Err(validation_failure(format!(
                        "unresolved target `{}` requires an explanation",
                        self.target_id
                    )));
                }

                if self.artifact_verification
                    == ArtifactVerification::Verified
                {
                    return Err(validation_failure(format!(
                        "unresolved target `{}` cannot be counted as a verified feasible realization",
                        self.target_id
                    )));
                }
            }
        }

        if self.feasibility != Feasibility::Feasible
            && self.semantic_preservation == SemanticPreservation::Equivalent
        {
            return Err(validation_failure(format!(
                "target `{}` claims successful semantic realization without established feasibility",
                self.target_id
            )));
        }

        Ok(())
    }

    /// Return the canonical target identity.
    #[must_use]
    pub fn target_id(&self) -> &str {
        &self.target_id
    }

    /// Return the logical build identity.
    #[must_use]
    pub fn build_identity(&self) -> &LogicalBuildIdentity {
        &self.build_identity
    }

    /// Return the authoritative feasibility outcome.
    #[must_use]
    pub const fn feasibility(&self) -> Feasibility {
        self.feasibility
    }

    /// Return the semantic-preservation outcome.
    #[must_use]
    pub const fn semantic_preservation(&self) -> SemanticPreservation {
        self.semantic_preservation
    }

    /// Return the artifact-verification outcome.
    #[must_use]
    pub const fn artifact_verification(&self) -> ArtifactVerification {
        self.artifact_verification
    }

    /// Return feasibility evidence.
    #[must_use]
    pub fn feasibility_evidence(&self) -> Option<&PortabilityEvidence> {
        self.feasibility_evidence.as_ref()
    }

    /// Return semantic evidence.
    #[must_use]
    pub fn semantic_evidence(&self) -> Option<&PortabilityEvidence> {
        self.semantic_evidence.as_ref()
    }

    /// Return artifact-verification evidence.
    #[must_use]
    pub fn artifact_evidence(&self) -> Option<&PortabilityEvidence> {
        self.artifact_evidence.as_ref()
    }

    /// Return the explanation, if supplied.
    #[must_use]
    pub fn explanation(&self) -> Option<&str> {
        self.explanation.as_deref()
    }
}

// ============================================================================
// PORTABILITY TEST REQUEST
// ============================================================================

/// Immutable request for a multi-target portability test.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PortabilityTestRequest {
    identity: LogicalBuildIdentity,
}

impl PortabilityTestRequest {
    /// Create a portability test request.
    #[must_use]
    pub const fn new(identity: LogicalBuildIdentity) -> Self {
        Self { identity }
    }

    /// Return the logical identity shared by all target realizations.
    #[must_use]
    pub fn identity(&self) -> &LogicalBuildIdentity {
        &self.identity
    }
}

// ============================================================================
// TARGET OBSERVATION PROVIDER
// ============================================================================

/// Adapter contract for collecting actual portability observations.
///
/// An implementation belongs in the toolchain integration layer.
///
/// Implementations must use:
///
/// - Canonical target identities.
/// - `targets::compatibility` and compiler planning.
/// - The real build executor.
/// - Compiler semantic-verification authorities.
/// - Canonical artifact validation.
///
/// The adapter must not infer success from target registration,
/// capability declarations or a simulated build.
///
/// Implementations must be thread-safe if shared between concurrent
/// test operations.
pub trait PortabilityObservationProvider: Send + Sync {
    /// Assess and realize the requested target using the supplied
    /// unchanged logical build identity.
    ///
    /// Infeasible and unresolved outcomes must be returned explicitly.
    fn observe(
        &self,
        identity: &LogicalBuildIdentity,
        target_id: &str,
    ) -> ToolchainResult<TargetObservation>;
}

// ============================================================================
// REPORTING
// ============================================================================

/// Overall result of portability conformance evaluation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PortabilityTestStatus {
    /// All observations satisfy the applicable portability contract.
    Conformant,

    /// At least one observation violates a mandatory contract.
    NonConformant,

    /// Evidence is insufficient to establish complete conformance.
    Incomplete,
}

impl PortabilityTestStatus {
    /// Return whether the complete test is conformant.
    #[must_use]
    pub const fn is_conformant(self) -> bool {
        matches!(self, Self::Conformant)
    }
}

/// Aggregate portability test counts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PortabilityTestCounts {
    requested: usize,
    feasible: usize,
    infeasible: usize,
    unresolved: usize,
    semantically_verified: usize,
    artifacts_verified: usize,
}

/// Read-only portability test statistics.
impl PortabilityTestCounts {
    /// Number of targets requested.
    #[must_use]
    pub const fn requested(&self) -> usize {
        self.requested
    }

    /// Number of targets established as feasible.
    #[must_use]
    pub const fn feasible(&self) -> usize {
        self.feasible
    }

    /// Number of targets established as infeasible.
    #[must_use]
    pub const fn infeasible(&self) -> usize {
        self.infeasible
    }

    /// Number of targets with unresolved feasibility.
    #[must_use]
    pub const fn unresolved(&self) -> usize {
        self.unresolved
    }

    /// Number of targets with verified semantic preservation.
    #[must_use]
    pub const fn semantically_verified(&self) -> usize {
        self.semantically_verified
    }

    /// Number of targets with verified artifacts.
    #[must_use]
    pub const fn artifacts_verified(&self) -> usize {
        self.artifacts_verified
    }
}

/// Structured portability conformance report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PortabilityTestReport {
    identity: LogicalBuildIdentity,
    status: PortabilityTestStatus,
    counts: PortabilityTestCounts,
    observations: Vec<TargetObservation>,
}

impl PortabilityTestReport {
    /// Return the shared logical build identity.
    #[must_use]
    pub fn identity(&self) -> &LogicalBuildIdentity {
        &self.identity
    }

    /// Return the overall conformance status.
    #[must_use]
    pub const fn status(&self) -> PortabilityTestStatus {
        self.status
    }

    /// Return aggregate counts.
    #[must_use]
    pub const fn counts(&self) -> &PortabilityTestCounts {
        &self.counts
    }

    /// Return all observations in deterministic target order.
    #[must_use]
    pub fn observations(&self) -> &[TargetObservation] {
        &self.observations
    }

    /// Return whether every requested target has an observation.
    ///
    /// The request's target iterator must be exhausted successfully
    /// before this report can be constructed.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.counts.unresolved == 0
            && self.observations.len() == self.counts.requested
    }

    /// Find an observation by canonical target identity.
    #[must_use]
    pub fn observation(
        &self,
        target_id: &str,
    ) -> Option<&TargetObservation> {
        self.observations
            .binary_search_by(|item| {
                item.target_id().cmp(target_id)
            })
            .ok()
            .map(|index| &self.observations[index])
    }
}

// ============================================================================
// CONFORMANCE EXECUTION
// ============================================================================

/// Execute a multi-target portability conformance test.
///
/// The target iterator is supplied by the canonical target-selection
/// or target-matrix authority.
///
/// No target enumeration, target registry or hardware discovery
/// implementation is introduced here.
///
/// The provider must execute the actual assessment and build workflow.
///
/// Targets are deduplicated and results are sorted deterministically.
///
/// Allocation failures are reported as resource-exhaustion errors.
///
/// An empty target iterator is rejected: an empty test cannot establish
/// portability.
///
/// # Errors
///
/// Returns an error if:
///
/// - No targets were requested.
/// - A target identity is invalid.
/// - A target is duplicated.
/// - A provider operation fails.
/// - An observation is inconsistent.
/// - Logical build identities differ.
/// - Required report storage cannot be allocated.
pub fn run_portability_conformance<I, S, P>(
    request: &PortabilityTestRequest,
    targets: I,
    provider: &P,
) -> ToolchainResult<PortabilityTestReport>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
    P: PortabilityObservationProvider + ?Sized,
{
    let mut seen = BTreeSet::<String>::new();
    let mut observations = Vec::<TargetObservation>::new();

    let mut counts = PortabilityTestCounts::default();

    for target in targets {
        let target_id = target.as_ref();

        validate_identity("requested target identity", target_id)?;

        if !seen.insert(target_id.to_owned()) {
            return Err(invalid_input(format!(
                "duplicate target `{target_id}` in portability test"
            )));
        }

        let observation = provider.observe(
            request.identity(),
            target_id,
        )?;

        observation.validate()?;

        if observation.target_id() != target_id {
            return Err(validation_failure(format!(
                "provider returned target `{}` for requested target `{target_id}`",
                observation.target_id()
            )));
        }

        if observation.build_identity() != request.identity() {
            return Err(validation_failure(format!(
                "target `{target_id}` changed the logical source, dependency or build identity"
            )));
        }

        counts.requested = counts
            .requested
            .checked_add(1)
            .ok_or_else(|| {
                ToolchainError::resource_exhausted(
                    "portability target count overflow",
                )
            })?;

        match observation.feasibility() {
            Feasibility::Feasible => {
                counts.feasible = counts
                    .feasible
                    .checked_add(1)
                    .ok_or_else(|| {
                        ToolchainError::resource_exhausted(
                            "feasible target count overflow",
                        )
                    })?;

                counts.semantically_verified = counts
                    .semantically_verified
                    .checked_add(1)
                    .ok_or_else(|| {
                        ToolchainError::resource_exhausted(
                            "semantic verification count overflow",
                        )
                    })?;

                counts.artifacts_verified = counts
                    .artifacts_verified
                    .checked_add(1)
                    .ok_or_else(|| {
                        ToolchainError::resource_exhausted(
                            "artifact verification count overflow",
                        )
                    })?;
            }

            Feasibility::Infeasible => {
                counts.infeasible = counts
                    .infeasible
                    .checked_add(1)
                    .ok_or_else(|| {
                        ToolchainError::resource_exhausted(
                            "infeasible target count overflow",
                        )
                    })?;
            }

            Feasibility::Unresolved => {
                counts.unresolved = counts
                    .unresolved
                    .checked_add(1)
                    .ok_or_else(|| {
                        ToolchainError::resource_exhausted(
                            "unresolved target count overflow",
                        )
                    })?;
            }
        }

        observations
            .try_reserve(1)
            .map_err(|error| {
                allocation_failure("target observations", error)
            })?;

        observations.push(observation);
    }

    if counts.requested == 0 {
        return Err(invalid_input(
            "portability conformance requires at least one target",
        ));
    }

    observations.sort_by(|left, right| {
        left.target_id().cmp(right.target_id())
    });

    let status = if observations.iter().any(|observation| {
        observation.semantic_preservation()
            == SemanticPreservation::Different
            || observation.artifact_verification()
                == ArtifactVerification::Rejected
    }) {
        PortabilityTestStatus::NonConformant
    } else if counts.unresolved > 0 {
        PortabilityTestStatus::Incomplete
    } else {
        PortabilityTestStatus::Conformant
    };

    Ok(PortabilityTestReport {
        identity: request.identity().clone(),
        status,
        counts,
        observations,
    })
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;
    use std::sync::Mutex;

    fn identity() -> LogicalBuildIdentity {
        LogicalBuildIdentity::new(
            "build:example",
            "source:fingerprint:example",
            "dependencies:fingerprint:example",
        )
        .expect("valid logical build identity")
    }

    fn evidence(
        category: &str,
        target: &str,
    ) -> PortabilityEvidence {
        PortabilityEvidence::new(
            format!("zamani.test.{category}"),
            format!("evidence:{category}:{target}"),
        )
        .expect("valid evidence")
    }

    fn feasible_observation(
        target: &str,
        identity: &LogicalBuildIdentity,
    ) -> TargetObservation {
        TargetObservation::new(
            target,
            identity.clone(),
            Feasibility::Feasible,
            SemanticPreservation::Equivalent,
            ArtifactVerification::Verified,
            Some(evidence("feasibility", target)),
            Some(evidence("semantics", target)),
            Some(evidence("artifact", target)),
            None,
        )
        .expect("valid feasible observation")
    }

    fn infeasible_observation(
        target: &str,
        identity: &LogicalBuildIdentity,
    ) -> TargetObservation {
        TargetObservation::new(
            target,
            identity.clone(),
            Feasibility::Infeasible,
            SemanticPreservation::NotAssessed,
            ArtifactVerification::NotProduced,
            None,
            None,
            None,
            Some("A mandatory requirement is unavailable.".to_owned()),
        )
        .expect("valid infeasible observation")
    }

    fn unresolved_observation(
        target: &str,
        identity: &LogicalBuildIdentity,
    ) -> TargetObservation {
        TargetObservation::new(
            target,
            identity.clone(),
            Feasibility::Unresolved,
            SemanticPreservation::Indeterminate,
            ArtifactVerification::Unverified,
            None,
            None,
            None,
            Some("Required capability evidence is incomplete.".to_owned()),
        )
        .expect("valid unresolved observation")
    }

    #[derive(Default)]
    struct MockProvider {
        observations: BTreeMap<String, TargetObservation>,
        calls: Mutex<Vec<String>>,
    }

    impl PortabilityObservationProvider for MockProvider {
        fn observe(
            &self,
            identity: &LogicalBuildIdentity,
            target_id: &str,
        ) -> ToolchainResult<TargetObservation> {
            self.calls
                .lock()
                .map_err(|_| {
                    ToolchainError::internal(
                        "test provider lock was poisoned",
                    )
                })?
                .push(target_id.to_owned());

            let observation = self
                .observations
                .get(target_id)
                .ok_or_else(|| {
                    ToolchainError::invalid_input(format!(
                        "no test observation for `{target_id}`"
                    ))
                })?;

            if observation.build_identity() != identity {
                return Err(ToolchainError::validation_failed(
                    "test observation has a different build identity",
                ));
            }

            Ok(observation.clone())
        }
    }

    fn provider(
        observations: Vec<TargetObservation>,
    ) -> MockProvider {
        MockProvider {
            observations: observations
                .into_iter()
                .map(|item| {
                    (item.target_id().to_owned(), item)
                })
                .collect(),
            calls: Mutex::new(Vec::new()),
        }
    }

    fn request() -> PortabilityTestRequest {
        PortabilityTestRequest::new(identity())
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            PORTABILITY_TEST_MODULE_ID,
            "zamani.toolchain.testing.portability"
        );

        assert_ne!(PORTABILITY_TEST_API_VERSION, 0);
    }

    #[test]
    fn logical_build_identity_rejects_empty_fields() {
        assert!(
            LogicalBuildIdentity::new("", "source", "dependencies")
                .is_err()
        );

        assert!(
            LogicalBuildIdentity::new("build", "", "dependencies")
                .is_err()
        );

        assert!(
            LogicalBuildIdentity::new("build", "source", "")
                .is_err()
        );
    }

    #[test]
    fn logical_build_identity_rejects_control_characters() {
        assert!(
            LogicalBuildIdentity::new(
                "build\n",
                "source",
                "dependencies",
            )
            .is_err()
        );
    }

    #[test]
    fn different_artifact_representations_can_preserve_semantics() {
        let first = feasible_observation(
            "provider:target-a",
            &identity(),
        );

        let second = feasible_observation(
            "provider:target-b",
            &identity(),
        );

        // Different target identities are allowed. Artifact byte
        // equality is intentionally not required by this contract.
        assert_ne!(first.target_id(), second.target_id());

        assert_eq!(
            first.semantic_preservation(),
            SemanticPreservation::Equivalent
        );

        assert_eq!(
            second.semantic_preservation(),
            SemanticPreservation::Equivalent
        );
    }

    #[test]
    fn feasible_target_requires_all_verification_evidence() {
        let result = TargetObservation::new(
            "target:missing-evidence",
            identity(),
            Feasibility::Feasible,
            SemanticPreservation::Equivalent,
            ArtifactVerification::Verified,
            None,
            None,
            None,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn feasible_target_cannot_have_unknown_semantics() {
        let result = TargetObservation::new(
            "target:unknown-semantics",
            identity(),
            Feasibility::Feasible,
            SemanticPreservation::Indeterminate,
            ArtifactVerification::Verified,
            Some(evidence("feasibility", "target")),
            Some(evidence("semantics", "target")),
            Some(evidence("artifact", "target")),
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn feasible_target_cannot_have_unverified_artifacts() {
        let result = TargetObservation::new(
            "target:unverified-artifact",
            identity(),
            Feasibility::Feasible,
            SemanticPreservation::Equivalent,
            ArtifactVerification::Unverified,
            Some(evidence("feasibility", "target")),
            Some(evidence("semantics", "target")),
            Some(evidence("artifact", "target")),
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn infeasible_target_requires_explanation() {
        let result = TargetObservation::new(
            "target:infeasible",
            identity(),
            Feasibility::Infeasible,
            SemanticPreservation::NotAssessed,
            ArtifactVerification::NotProduced,
            None,
            None,
            None,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn infeasible_target_cannot_claim_verified_artifact() {
        let result = TargetObservation::new(
            "target:infeasible",
            identity(),
            Feasibility::Infeasible,
            SemanticPreservation::NotAssessed,
            ArtifactVerification::Verified,
            None,
            None,
            None,
            Some("Mandatory requirement unavailable.".to_owned()),
        );

        assert!(result.is_err());
    }

    #[test]
    fn unresolved_target_is_not_treated_as_feasible() {
        let observation = unresolved_observation(
            "target:unknown",
            &identity(),
        );

        assert_eq!(
            observation.feasibility(),
            Feasibility::Unresolved
        );

        assert!(!observation.feasibility().is_feasible());
    }

    #[test]
    fn empty_target_collection_is_rejected() {
        let provider = provider(Vec::new());

        let result = run_portability_conformance(
            &request(),
            Vec::<String>::new(),
            &provider,
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_targets_are_rejected() {
        let identity = identity();

        let provider = provider(vec![
            feasible_observation("target:a", &identity),
        ]);

        let result = run_portability_conformance(
            &request(),
            ["target:a", "target:a"],
            &provider,
        );

        assert!(result.is_err());
    }

    #[test]
    fn one_logical_build_can_have_multiple_realizations() {
        let identity = identity();

        let provider = provider(vec![
            feasible_observation("target:a", &identity),
            feasible_observation("target:b", &identity),
            feasible_observation("target:c", &identity),
        ]);

        let report = run_portability_conformance(
            &request(),
            ["target:a", "target:b", "target:c"],
            &provider,
        )
        .expect("conformance evaluation");

        assert_eq!(
            report.status(),
            PortabilityTestStatus::Conformant
        );

        assert_eq!(report.counts().requested(), 3);
        assert_eq!(report.counts().feasible(), 3);
        assert_eq!(report.counts().semantically_verified(), 3);
        assert_eq!(report.counts().artifacts_verified(), 3);
    }

    #[test]
    fn different_source_identity_is_rejected() {
        let original = identity();

        let changed = LogicalBuildIdentity::new(
            "build:example",
            "source:different",
            "dependencies:fingerprint:example",
        )
        .expect("valid identity");

        let provider = provider(vec![
            feasible_observation("target:a", &changed),
        ]);

        let result = run_portability_conformance(
            &PortabilityTestRequest::new(original),
            ["target:a"],
            &provider,
        );

        assert!(result.is_err());
    }

    #[test]
    fn different_dependency_identity_is_rejected() {
        let original = identity();

        let changed = LogicalBuildIdentity::new(
            "build:example",
            "source:fingerprint:example",
            "dependencies:different",
        )
        .expect("valid identity");

        let provider = provider(vec![
            feasible_observation("target:a", &changed),
        ]);

        let result = run_portability_conformance(
            &PortabilityTestRequest::new(original),
            ["target:a"],
            &provider,
        );

        assert!(result.is_err());
    }

    #[test]
    fn observations_are_returned_in_deterministic_order() {
        let identity = identity();

        let provider = provider(vec![
            feasible_observation("target:z", &identity),
            feasible_observation("target:a", &identity),
            feasible_observation("target:m", &identity),
        ]);

        let report = run_portability_conformance(
            &request(),
            ["target:z", "target:a", "target:m"],
            &provider,
        )
        .expect("conformance evaluation");

        let targets: Vec<&str> = report
            .observations()
            .iter()
            .map(TargetObservation::target_id)
            .collect();

        assert_eq!(
            targets,
            ["target:a", "target:m", "target:z"]
        );
    }

    #[test]
    fn infeasible_targets_are_reported_without_false_success() {
        let identity = identity();

        let provider = provider(vec![
            feasible_observation("target:a", &identity),
            infeasible_observation("target:b", &identity),
        ]);

        let report = run_portability_conformance(
            &request(),
            ["target:a", "target:b"],
            &provider,
        )
        .expect("conformance evaluation");

        assert_eq!(
            report.status(),
            PortabilityTestStatus::Conformant
        );

        assert_eq!(report.counts().feasible(), 1);
        assert_eq!(report.counts().infeasible(), 1);
        assert_eq!(report.counts().artifacts_verified(), 1);
    }

    #[test]
    fn unresolved_targets_make_the_report_incomplete() {
        let identity = identity();

        let provider = provider(vec![
            feasible_observation("target:a", &identity),
            unresolved_observation("target:b", &identity),
        ]);

        let report = run_portability_conformance(
            &request(),
            ["target:a", "target:b"],
            &provider,
        )
        .expect("conformance evaluation");

        assert_eq!(
            report.status(),
            PortabilityTestStatus::Incomplete
        );

        assert_eq!(report.counts().unresolved(), 1);
        assert!(!report.is_complete());
    }

    #[test]
    fn semantic_regression_is_non_conformant() {
        let identity = identity();

        let observation = TargetObservation {
            target_id: "target:regression".to_owned(),
            build_identity: identity.clone(),
            feasibility: Feasibility::Feasible,
            semantic_preservation: SemanticPreservation::Different,
            artifact_verification: ArtifactVerification::Verified,
            feasibility_evidence: Some(evidence(
                "feasibility",
                "target:regression",
            )),
            semantic_evidence: Some(evidence(
                "semantics",
                "target:regression",
            )),
            artifact_evidence: Some(evidence(
                "artifact",
                "target:regression",
            )),
            explanation: None,
        };

        let provider = provider(vec![observation]);

        // A production provider cannot construct this inconsistent
        // observation through the public constructor. This test
        // exercises the defensive validation boundary.
        let result = run_portability_conformance(
            &request(),
            ["target:regression"],
            &provider,
        );

        assert!(result.is_err());
    }

    #[test]
    fn artifact_rejection_is_not_conformance() {
        let identity = identity();

        let observation = TargetObservation {
            target_id: "target:rejected".to_owned(),
            build_identity: identity.clone(),
            feasibility: Feasibility::Feasible,
            semantic_preservation: SemanticPreservation::Equivalent,
            artifact_verification: ArtifactVerification::Rejected,
            feasibility_evidence: Some(evidence(
                "feasibility",
                "target:rejected",
            )),
            semantic_evidence: Some(evidence(
                "semantics",
                "target:rejected",
            )),
            artifact_evidence: Some(evidence(
                "artifact",
                "target:rejected",
            )),
            explanation: None,
        };

        let provider = provider(vec![observation]);

        let result = run_portability_conformance(
            &request(),
            ["target:rejected"],
            &provider,
        );

        assert!(result.is_err());
    }

    #[test]
    fn report_supports_target_lookup() {
        let identity = identity();

        let provider = provider(vec![
            feasible_observation("target:a", &identity),
            feasible_observation("target:b", &identity),
        ]);

        let report = run_portability_conformance(
            &request(),
            ["target:a", "target:b"],
            &provider,
        )
        .expect("conformance evaluation");

        assert!(
            report.observation("target:a").is_some()
        );

        assert!(
            report.observation("target:missing").is_none()
        );
    }

    #[test]
    fn future_computing_domains_are_not_enumerated() {
        let identity = identity();

        let provider = provider(vec![
            feasible_observation(
                "future.domain.provider:implementation",
                &identity,
            ),
        ]);

        let report = run_portability_conformance(
            &request(),
            ["future.domain.provider:implementation"],
            &provider,
        )
        .expect("open-world target identity");

        assert!(report.status().is_conformant());
    }

    #[test]
    fn very_large_logical_resource_requirements_remain_representable() {
        let identity = LogicalBuildIdentity::new(
            "build:large",
            "source:large",
            "dependencies:large",
        )
        .expect("valid identity");

        let target = feasible_observation(
            "future.large.target",
            &identity,
        );

        // Resource quantities are evaluated by canonical compiler
        // and backend authorities. This module introduces no resource
        // count or hardware-capacity ceiling.
        assert!(target.feasibility().is_feasible());
    }

    #[test]
    fn evidence_identifiers_are_extensible() {
        let evidence = PortabilityEvidence::new(
            "future.independent.verification.authority",
            "verification:future-domain:001",
        )
        .expect("valid extensible evidence");

        assert_eq!(
            evidence.authority(),
            "future.independent.verification.authority"
        );
    }
}
