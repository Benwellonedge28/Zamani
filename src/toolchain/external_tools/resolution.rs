
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani External Tool Resolution
//!
//! Path: src/toolchain/external_tools/resolution.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Resolves declarative external-tool requirements against explicitly
//! registered executable candidates.
//!
//! ## Ownership
//!
//! This module owns:
//! - Requirement-to-provider-candidate matching.
//! - Deterministic candidate ordering.
//! - Resolution orchestration.
//! - Alternative candidate handling.
//! - Required and optional resolution outcomes.
//! - Structured resolution evidence.
//! - Resolution-level cancellation checks.
//!
//! This module does not own:
//! - Executable discovery.
//! - PATH lookup.
//! - Version probing.
//! - Process execution.
//! - Executable authorization.
//! - Provider registration or persistence.
//! - Hardware discovery.
//! - Target feasibility.
//! - Compiler or runtime execution.
//!
//! ## Canonical integration
//!
//! external_tools/requirements.rs owns declarative requirements.
//! backend/toolchain.rs owns ToolchainSpec, ToolVersion,
//! VersionRequirement, ToolchainManager and ToolchainDescriptor.
//! backend/process.rs owns safe process execution.
//! external_tools/validation.rs owns subsequent policy validation.
//! external_tools/reporting.rs owns presentation and reporting.
//!
//! ## Determinism
//!
//! Requirements are processed in canonical identifier order.
//! Candidates are processed in provider/candidate identifier order.
//! Alternative selection therefore does not depend on hash-map iteration.
//!
//! ## Security
//!
//! Requirement identifiers and provider identifiers are metadata.
//! They are never converted into shell commands.
//! Executable paths and invocation arguments remain under the authority
//! of backend::toolchain and backend::process.
//!
//! Discovery is not executable authorization. The security subsystem
//! must authorize executable paths before untrusted build operations.
//!
//! ## Scalability
//!
//! No fixed provider count, requirement count, candidate count,
//! computational-domain count or hardware-capacity limit is imposed.
//!
//! Memory and processing consumption grow with the supplied inputs.
//! Operational budgets belong to configuration and task orchestration.
//!
//! ## Important limitation
//!
//! The existing ToolchainManager::discover API does not accept a
//! CancellationToken. Consequently, this resolver can check cancellation
//! between discovery attempts, but cannot interrupt an active version probe.
//! A cancellable discovery API must be added to backend/toolchain.rs
//! before claiming interruption of an in-progress probe.

use crate::backend::toolchain::{
    ToolchainDescriptor,
    ToolchainError,
    ToolchainManager,
    ToolchainSpec,
    ToolVersion,
};

use crate::backend::process::CancellationToken;

use crate::toolchain::external_tools::requirements::{
    ExternalToolRequirement,
    ExternalToolRequirements,
    NumericVersion,
    RequirementId,
    RequirementKind,
};

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::path::Path;

// ============================================================================
// PROVIDER CANDIDATES
// ============================================================================

/// A registered executable candidate for satisfying a logical requirement.
///
/// A candidate explicitly associates:
///
/// - A logical requirement identifier.
/// - A provider identifier.
/// - A candidate identifier.
/// - An existing backend ToolchainSpec.
///
/// The logical requirement identifier is never interpreted as an executable
/// name.
///
/// Multiple candidates may satisfy one requirement. Their canonical
/// identifiers determine deterministic resolution order.
#[derive(Debug, Clone)]
pub struct ToolchainCandidate {
    requirement_id: RequirementId,
    provider_id: String,
    candidate_id: String,
    specification: ToolchainSpec,
}

impl ToolchainCandidate {
    /// Construct an explicitly mapped executable candidate.
    ///
    /// The provider and candidate identifiers are open-ended metadata.
    /// They must be nonempty and contain no surrounding whitespace.
    ///
    /// `specification` retains all executable, version-probe, environment,
    /// timeout and output-capture configuration owned by the backend.
    pub fn new(
        requirement_id: RequirementId,
        provider_id: impl Into<String>,
        candidate_id: impl Into<String>,
        specification: ToolchainSpec,
    ) -> Result<Self, ResolutionError> {
        let provider_id = validate_identifier(
            "provider",
            provider_id.into(),
        )?;

        let candidate_id = validate_identifier(
            "candidate",
            candidate_id.into(),
        )?;

        Ok(Self {
            requirement_id,
            provider_id,
            candidate_id,
            specification,
        })
    }

    /// Return the logical requirement identifier.
    #[must_use]
    pub fn requirement_id(&self) -> &RequirementId {
        &self.requirement_id
    }

    /// Return the provider identifier.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    /// Return the candidate identifier.
    #[must_use]
    pub fn candidate_id(&self) -> &str {
        &self.candidate_id
    }

    /// Return the backend-owned executable specification.
    #[must_use]
    pub fn specification(&self) -> &ToolchainSpec {
        &self.specification
    }

    /// Return the configured executable reference.
    ///
    /// This is the requested executable, not proof of its resolved path
    /// or authorization.
    #[must_use]
    pub fn requested_executable(&self) -> &Path {
        self.specification.executable()
    }
}

/// Validate provider and candidate metadata identifiers.
///
/// These identifiers are intentionally not restricted to a closed
/// vendor or computational-domain enumeration.
fn validate_identifier(
    category: &'static str,
    value: String,
) -> Result<String, ResolutionError> {
    if value.trim().is_empty() {
        return Err(ResolutionError::InvalidIdentifier {
            category,
            reason: "identifier must not be empty".to_owned(),
        });
    }

    if value.trim() != value {
        return Err(ResolutionError::InvalidIdentifier {
            category,
            reason: "identifier must not contain surrounding whitespace"
                .to_owned(),
        });
    }

    Ok(value)
}

// ============================================================================
// CANDIDATE COLLECTION
// ============================================================================

/// A deterministic collection of executable candidates.
///
/// Candidates are indexed by requirement, provider and candidate identity.
/// Duplicate identities are rejected.
///
/// Candidate registration is supplied by the owning provider registry.
/// This collection does not discover providers or inspect the host.
#[derive(Debug, Clone, Default)]
pub struct ToolchainCandidates {
    candidates: BTreeMap<
        RequirementId,
        BTreeMap<(String, String), ToolchainCandidate>,
    >,
}

impl ToolchainCandidates {
    /// Construct an empty candidate collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a candidate.
    ///
    /// Duplicate provider/candidate identities for the same requirement
    /// are rejected without replacing the existing candidate.
    pub fn insert(
        &mut self,
        candidate: ToolchainCandidate,
    ) -> Result<(), ResolutionError> {
        let requirement_id = candidate.requirement_id.clone();

        let identity = (
            candidate.provider_id.clone(),
            candidate.candidate_id.clone(),
        );

        let candidates = self
            .candidates
            .entry(requirement_id.clone())
            .or_default();

        if candidates.contains_key(&identity) {
            return Err(ResolutionError::DuplicateCandidate {
                requirement_id: requirement_id.to_string(),
                provider_id: identity.0,
                candidate_id: identity.1,
            });
        }

        candidates.insert(identity, candidate);

        Ok(())
    }

    /// Construct a candidate collection from an iterator.
    pub fn from_iter<I>(
        candidates: I,
    ) -> Result<Self, ResolutionError>
    where
        I: IntoIterator<Item = ToolchainCandidate>,
    {
        let mut collection = Self::new();

        for candidate in candidates {
            collection.insert(candidate)?;
        }

        Ok(collection)
    }

    /// Return candidates for a requirement in deterministic order.
    pub fn for_requirement(
        &self,
        requirement_id: &RequirementId,
    ) -> impl Iterator<Item = &ToolchainCandidate> {
        self.candidates
            .get(requirement_id)
            .into_iter()
            .flat_map(|candidates| candidates.values())
    }

    /// Return the number of registered candidates.
    #[must_use]
    pub fn len(&self) -> usize {
        self.candidates
            .values()
            .map(BTreeMap::len)
            .sum()
    }

    /// Return whether the collection contains no candidates.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.candidates.is_empty()
    }

    /// Return the number of requirements having registered candidates.
    #[must_use]
    pub fn requirement_count(&self) -> usize {
        self.candidates.len()
    }
}

// ============================================================================
// RESOLUTION STATUS
// ============================================================================

/// Final resolution status of one external-tool requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResolutionStatus {
    /// A required tool was successfully discovered and version-checked.
    ResolvedRequired,

    /// An optional tool was successfully discovered and version-checked.
    ResolvedOptional,

    /// No candidate was registered for a required tool.
    MissingRequired,

    /// No candidate was registered for an optional tool.
    MissingOptional,

    /// Candidates existed, but none satisfied a required tool.
    FailedRequired,

    /// Candidates existed, but none satisfied an optional tool.
    FailedOptional,

    /// Resolution was stopped by a cancellation request.
    Cancelled,
}

impl ResolutionStatus {
    /// Return whether this status represents a successful resolution.
    #[must_use]
    pub const fn is_resolved(self) -> bool {
        matches!(
            self,
            Self::ResolvedRequired | Self::ResolvedOptional
        )
    }

    /// Return whether this status represents a mandatory failure.
    #[must_use]
    pub const fn is_required_failure(self) -> bool {
        matches!(
            self,
            Self::MissingRequired
                | Self::FailedRequired
                | Self::Cancelled
        )
    }

    /// Return whether this status represents an optional failure.
    #[must_use]
    pub const fn is_optional_failure(self) -> bool {
        matches!(
            self,
            Self::MissingOptional | Self::FailedOptional
        )
    }
}

// ============================================================================
// FAILURE EVIDENCE
// ============================================================================

/// Failure evidence for one executable candidate.
///
/// The original backend error is preserved as a diagnostic string.
/// Its failure classification remains explicit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateFailure {
    provider_id: String,
    candidate_id: String,
    error: String,
}

impl CandidateFailure {
    /// Return the provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    /// Return the candidate identity.
    #[must_use]
    pub fn candidate_id(&self) -> &str {
        &self.candidate_id
    }

    /// Return the backend discovery or version-validation error.
    #[must_use]
    pub fn error(&self) -> &str {
        &self.error
    }
}

// ============================================================================
// RESOLVED TOOL
// ============================================================================

/// Evidence that one executable candidate satisfied a logical requirement.
///
/// The descriptor is produced by the existing backend ToolchainManager.
/// It contains the resolved executable and discovered numeric version.
///
/// It does not establish cryptographic executable identity or trust.
#[derive(Debug, Clone)]
pub struct ResolvedExternalTool {
    requirement_id: RequirementId,
    provider_id: String,
    candidate_id: String,
    descriptor: ToolchainDescriptor,
}

impl ResolvedExternalTool {
    /// Return the logical requirement identifier.
    #[must_use]
    pub fn requirement_id(&self) -> &RequirementId {
        &self.requirement_id
    }

    /// Return the selected provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    /// Return the selected candidate identity.
    #[must_use]
    pub fn candidate_id(&self) -> &str {
        &self.candidate_id
    }

    /// Return the backend-owned resolved descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ToolchainDescriptor {
        &self.descriptor
    }

    /// Return the resolved executable path.
    #[must_use]
    pub fn executable(&self) -> &Path {
        self.descriptor.executable()
    }

    /// Return the discovered numeric version.
    #[must_use]
    pub const fn version(&self) -> ToolVersion {
        self.descriptor.version()
    }
}

// ============================================================================
// REQUIREMENT RESULT
// ============================================================================

/// Resolution evidence for one logical external-tool requirement.
#[derive(Debug, Clone)]
pub struct RequirementResolution {
    requirement_id: RequirementId,
    requirement_kind: RequirementKind,
    status: ResolutionStatus,
    resolved: Option<ResolvedExternalTool>,
    failures: Vec<CandidateFailure>,
}

impl RequirementResolution {
    /// Return the logical requirement identifier.
    #[must_use]
    pub fn requirement_id(&self) -> &RequirementId {
        &self.requirement_id
    }

    /// Return whether the original requirement was mandatory or optional.
    #[must_use]
    pub const fn requirement_kind(&self) -> RequirementKind {
        self.requirement_kind
    }

    /// Return the final resolution status.
    #[must_use]
    pub const fn status(&self) -> ResolutionStatus {
        self.status
    }

    /// Return the successfully resolved tool, if one exists.
    #[must_use]
    pub fn resolved(&self) -> Option<&ResolvedExternalTool> {
        self.resolved.as_ref()
    }

    /// Return all failed candidate attempts in deterministic order.
    #[must_use]
    pub fn failures(&self) -> &[CandidateFailure] {
        &self.failures
    }

    /// Return whether this requirement was satisfied.
    #[must_use]
    pub const fn is_resolved(&self) -> bool {
        self.status.is_resolved()
    }
}

// ============================================================================
// COMPLETE RESOLUTION REPORT
// ============================================================================

/// Complete evidence from one external-tool resolution operation.
///
/// Results are indexed by logical requirement identity and therefore
/// have deterministic iteration order.
#[derive(Debug, Clone, Default)]
pub struct ResolutionReport {
    results: BTreeMap<RequirementId, RequirementResolution>,
}

impl ResolutionReport {
    /// Construct an empty resolution report.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the resolution for one logical requirement.
    #[must_use]
    pub fn get(
        &self,
        id: &RequirementId,
    ) -> Option<&RequirementResolution> {
        self.results.get(id)
    }

    /// Iterate through results in deterministic identifier order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &RequirementResolution> {
        self.results.values()
    }

    /// Return the total number of processed requirements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.results.len()
    }

    /// Return whether the report contains no requirements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Count successfully resolved requirements.
    #[must_use]
    pub fn resolved_count(&self) -> usize {
        self.results
            .values()
            .filter(|result| result.is_resolved())
            .count()
    }

    /// Count unresolved mandatory requirements.
    #[must_use]
    pub fn required_failure_count(&self) -> usize {
        self.results
            .values()
            .filter(|result| result.status.is_required_failure())
            .count()
    }

    /// Count unresolved optional requirements.
    #[must_use]
    pub fn optional_failure_count(&self) -> usize {
        self.results
            .values()
            .filter(|result| result.status.is_optional_failure())
            .count()
    }

    /// Return whether every mandatory requirement was satisfied.
    ///
    /// Optional failures do not make the required dependency set unsatisfied.
    #[must_use]
    pub fn required_requirements_satisfied(&self) -> bool {
        self.required_failure_count() == 0
    }

    /// Return whether every declared requirement was resolved.
    #[must_use]
    pub fn all_requirements_resolved(&self) -> bool {
        self.results.values().all(RequirementResolution::is_resolved)
    }

    /// Return whether resolution was cancelled.
    #[must_use]
    pub fn was_cancelled(&self) -> bool {
        self.results
            .values()
            .any(|result| result.status == ResolutionStatus::Cancelled)
    }

    /// Return successfully resolved tools in deterministic order.
    pub fn resolved_tools(
        &self,
    ) -> impl Iterator<Item = &ResolvedExternalTool> {
        self.results
            .values()
            .filter_map(RequirementResolution::resolved)
    }

    /// Return the original requirement resolution collection.
    #[must_use]
    pub fn results(
        &self,
    ) -> &BTreeMap<RequirementId, RequirementResolution> {
        &self.results
    }
}

// ============================================================================
// RESOLVER
// ============================================================================

/// Stateless external-tool resolver.
///
/// This type coordinates existing backend discovery without creating
/// another executable discovery or process execution implementation.
#[derive(Debug, Default, Clone, Copy)]
pub struct ExternalToolResolver {
    manager: ToolchainManager,
}

impl ExternalToolResolver {
    /// Construct a resolver using the canonical backend manager.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            manager: ToolchainManager::new(),
        }
    }

    /// Resolve all requirements against registered candidates.
    ///
    /// Required and optional requirements are both attempted.
    ///
    /// For each requirement:
    ///
    /// 1. Candidates are visited in deterministic order.
    /// 2. The backend discovers the executable.
    /// 3. The backend probes and validates its configured version policy.
    /// 4. The declarative requirement version constraint is checked.
    /// 5. The first candidate satisfying both policies is selected.
    /// 6. Failed attempts are retained as evidence.
    ///
    /// The method returns a report even when mandatory requirements fail.
    /// The caller must inspect `required_requirements_satisfied()` before
    /// proceeding with a dependent build operation.
    pub fn resolve(
        &self,
        requirements: &ExternalToolRequirements,
        candidates: &ToolchainCandidates,
    ) -> Result<ResolutionReport, ResolutionError> {
        self.resolve_with_cancellation(
            requirements,
            candidates,
            &CancellationToken::new(),
        )
    }

    /// Resolve requirements with cooperative cancellation between attempts.
    ///
    /// The current backend discovery API cannot interrupt an active version
    /// probe. Cancellation is observed before each candidate attempt and
    /// before processing subsequent requirements.
    ///
    /// Already resolved results are preserved in the returned report.
    pub fn resolve_with_cancellation(
        &self,
        requirements: &ExternalToolRequirements,
        candidates: &ToolchainCandidates,
        cancellation: &CancellationToken,
    ) -> Result<ResolutionReport, ResolutionError> {
        requirements
            .validate()
            .map_err(ResolutionError::InvalidRequirements)?;

        validate_candidate_references(requirements, candidates)?;

        let mut report = ResolutionReport::new();

        for requirement in requirements.iter() {
            let result = self.resolve_requirement(
                requirement,
                candidates,
                cancellation,
            );

            report
                .results
                .insert(requirement.id().clone(), result);

            if cancellation.is_cancelled() {
                break;
            }
        }

        Ok(report)
    }

    /// Resolve one logical requirement against its candidate alternatives.
    fn resolve_requirement(
        &self,
        requirement: &ExternalToolRequirement,
        candidates: &ToolchainCandidates,
        cancellation: &CancellationToken,
    ) -> RequirementResolution {
        let mut failures = Vec::new();

        let mut status = match requirement.requirement_kind() {
            RequirementKind::Required => ResolutionStatus::MissingRequired,
            RequirementKind::Optional => ResolutionStatus::MissingOptional,
        };

        let mut resolved = None;

        let mut had_candidate = false;

        for candidate in candidates.for_requirement(requirement.id()) {
            had_candidate = true;

            if cancellation.is_cancelled() {
                status = ResolutionStatus::Cancelled;
                break;
            }

            match self.resolve_candidate(requirement, candidate) {
                Ok(tool) => {
                    status = match requirement.requirement_kind() {
                        RequirementKind::Required => {
                            ResolutionStatus::ResolvedRequired
                        }
                        RequirementKind::Optional => {
                            ResolutionStatus::ResolvedOptional
                        }
                    };

                    resolved = Some(tool);
                    break;
                }

                Err(error) => {
                    failures.push(CandidateFailure {
                        provider_id: candidate.provider_id.clone(),
                        candidate_id: candidate.candidate_id.clone(),
                        error: error.to_string(),
                    });
                }
            }
        }

        if resolved.is_none()
            && status != ResolutionStatus::Cancelled
            && had_candidate
        {
            status = match requirement.requirement_kind() {
                RequirementKind::Required => ResolutionStatus::FailedRequired,
                RequirementKind::Optional => ResolutionStatus::FailedOptional,
            };
        }

        RequirementResolution {
            requirement_id: requirement.id().clone(),
            requirement_kind: requirement.requirement_kind(),
            status,
            resolved,
            failures,
        }
    }

    /// Resolve one explicitly mapped executable candidate.
    ///
    /// The backend is the sole authority for executable discovery and
    /// its configured version probe.
    fn resolve_candidate(
        &self,
        requirement: &ExternalToolRequirement,
        candidate: &ToolchainCandidate,
    ) -> Result<ResolvedExternalTool, CandidateResolutionError> {
        let descriptor = self
            .manager
            .discover(&candidate.specification)
            .map_err(CandidateResolutionError::Backend)?;

        let version = descriptor.version();

        let actual = NumericVersion::new(
            version.major,
            version.minor,
            version.patch,
        );

        let satisfies = requirement
            .version_constraint()
            .matches(actual)
            .map_err(CandidateResolutionError::Requirement)?;

        if !satisfies {
            return Err(CandidateResolutionError::VersionConstraint {
                requirement_id: requirement.id().to_string(),
                actual,
            });
        }

        Ok(ResolvedExternalTool {
            requirement_id: requirement.id().clone(),
            provider_id: candidate.provider_id.clone(),
            candidate_id: candidate.candidate_id.clone(),
            descriptor,
        })
    }
}

// ============================================================================
// INPUT VALIDATION
// ============================================================================

/// Ensure every registered candidate references a declared requirement.
///
/// This detects configuration mistakes before external processes are started.
fn validate_candidate_references(
    requirements: &ExternalToolRequirements,
    candidates: &ToolchainCandidates,
) -> Result<(), ResolutionError> {
    let declared: BTreeSet<_> = requirements
        .iter()
        .map(|requirement| requirement.id().clone())
        .collect();

    for requirement_id in candidates.candidates.keys() {
        if !declared.contains(requirement_id) {
            return Err(ResolutionError::UnknownRequirement {
                requirement_id: requirement_id.to_string(),
            });
        }
    }

    Ok(())
}

// ============================================================================
// CANDIDATE RESOLUTION ERRORS
// ============================================================================

/// A failure encountered while resolving one executable candidate.
#[derive(Debug)]
enum CandidateResolutionError {
    Backend(ToolchainError),

    Requirement(
        crate::toolchain::external_tools::requirements::RequirementError,
    ),

    VersionConstraint {
        requirement_id: String,
        actual: NumericVersion,
    },
}

impl fmt::Display for CandidateResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Backend(error) => error.fmt(formatter),

            Self::Requirement(error) => error.fmt(formatter),

            Self::VersionConstraint {
                requirement_id,
                actual,
            } => write!(
                formatter,
                "discovered version {actual} does not satisfy \
                 external-tool requirement `{requirement_id}`"
            ),
        }
    }
}

impl Error for CandidateResolutionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Backend(error) => Some(error),
            Self::Requirement(error) => Some(error),
            Self::VersionConstraint { .. } => None,
        }
    }
}

// ============================================================================
// PUBLIC RESOLUTION ERRORS
// ============================================================================

/// Errors that prevent an external-tool resolution operation from starting.
#[derive(Debug)]
pub enum ResolutionError {
    /// A declarative requirement collection is invalid.
    InvalidRequirements(
        crate::toolchain::external_tools::requirements::RequirementError,
    ),

    /// A provider or candidate identifier is invalid.
    InvalidIdentifier {
        /// Identifier category.
        category: &'static str,

        /// Reason for rejection.
        reason: String,
    },

    /// The same provider/candidate identity was registered twice.
    DuplicateCandidate {
        /// Logical requirement identifier.
        requirement_id: String,

        /// Provider identity.
        provider_id: String,

        /// Candidate identity.
        candidate_id: String,
    },

    /// A candidate references an undeclared requirement.
    UnknownRequirement {
        /// Unknown logical requirement identifier.
        requirement_id: String,
    },
}

impl fmt::Display for ResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequirements(error) => {
                write!(formatter, "invalid external-tool requirements: {error}")
            }

            Self::InvalidIdentifier {
                category,
                reason,
            } => {
                write!(formatter, "invalid {category} identifier: {reason}")
            }

            Self::DuplicateCandidate {
                requirement_id,
                provider_id,
                candidate_id,
            } => write!(
                formatter,
                "duplicate candidate `{provider_id}/{candidate_id}` \
                 for requirement `{requirement_id}`"
            ),

            Self::UnknownRequirement {
                requirement_id,
            } => write!(
                formatter,
                "candidate references undeclared requirement `{requirement_id}`"
            ),
        }
    }
}

impl Error for ResolutionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidRequirements(error) => Some(error),
            Self::InvalidIdentifier { .. }
            | Self::DuplicateCandidate { .. }
            | Self::UnknownRequirement { .. } => None,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::external_tools::requirements::{
        CapabilityId,
        DomainId,
        ExternalToolRequirement,
        VersionConstraint,
    };

    fn requirement(id: &str) -> ExternalToolRequirement {
        ExternalToolRequirement::new(id)
            .expect("valid test requirement")
    }

    fn requirement_id(id: &str) -> RequirementId {
        RequirementId::new(id).expect("valid test requirement ID")
    }

    fn executable_candidate(
        requirement: &str,
        provider: &str,
        candidate: &str,
        executable: &str,
    ) -> ToolchainCandidate {
        ToolchainCandidate::new(
            requirement_id(requirement),
            provider,
            candidate,
            ToolchainSpec::new(
                format!("{provider}.{candidate}"),
                executable,
            ),
        )
        .expect("valid test candidate")
    }

    #[test]
    fn candidate_identifiers_are_open_ended() {
        let candidate = executable_candidate(
            "future.compiler",
            "provider.future",
            "implementation-v2",
            "future-compiler",
        );

        assert_eq!(
            candidate.requirement_id().as_str(),
            "future.compiler"
        );

        assert_eq!(
            candidate.provider_id(),
            "provider.future"
        );

        assert_eq!(
            candidate.candidate_id(),
            "implementation-v2"
        );
    }

    #[test]
    fn candidate_identifiers_reject_empty_values() {
        let result = ToolchainCandidate::new(
            requirement_id("llvm.compiler"),
            "",
            "candidate",
            ToolchainSpec::new("llvm", "clang"),
        );

        assert!(matches!(
            result,
            Err(ResolutionError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn candidate_identifiers_reject_surrounding_whitespace() {
        let result = ToolchainCandidate::new(
            requirement_id("llvm.compiler"),
            " provider ",
            "candidate",
            ToolchainSpec::new("llvm", "clang"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn candidate_collection_rejects_duplicates() {
        let first = executable_candidate(
            "llvm.compiler",
            "provider.llvm",
            "clang",
            "clang",
        );

        let second = executable_candidate(
            "llvm.compiler",
            "provider.llvm",
            "clang",
            "clang-17",
        );

        let mut collection = ToolchainCandidates::new();

        collection.insert(first).expect("first candidate");

        assert!(matches!(
            collection.insert(second),
            Err(ResolutionError::DuplicateCandidate { .. })
        ));

        assert_eq!(collection.len(), 1);
    }

    #[test]
    fn candidate_order_is_deterministic() {
        let collection = ToolchainCandidates::from_iter([
            executable_candidate(
                "llvm.compiler",
                "provider.zeta",
                "zeta",
                "zeta",
            ),
            executable_candidate(
                "llvm.compiler",
                "provider.alpha",
                "alpha",
                "alpha",
            ),
            executable_candidate(
                "llvm.compiler",
                "provider.middle",
                "middle",
                "middle",
            ),
        ])
        .expect("unique candidates");

        let providers: Vec<_> = collection
            .for_requirement(&requirement_id("llvm.compiler"))
            .map(ToolchainCandidate::provider_id)
            .collect();

        assert_eq!(
            providers,
            vec![
                "provider.alpha",
                "provider.middle",
                "provider.zeta"
            ]
        );
    }

    #[test]
    fn candidate_cannot_reference_an_unknown_requirement() {
        let requirements = ExternalToolRequirements::from_iter([
            requirement("llvm.compiler"),
        ])
        .expect("valid requirements");

        let candidates = ToolchainCandidates::from_iter([
            executable_candidate(
                "unknown.compiler",
                "provider.test",
                "candidate",
                "compiler",
            ),
        ])
        .expect("valid candidate");

        let resolver = ExternalToolResolver::new();

        assert!(matches!(
            resolver.resolve(&requirements, &candidates),
            Err(ResolutionError::UnknownRequirement { .. })
        ));
    }

    #[test]
    fn empty_candidate_collection_is_valid() {
        let requirements = ExternalToolRequirements::from_iter([
            requirement("llvm.compiler"),
        ])
        .expect("valid requirements");

        let report = ExternalToolResolver::new()
            .resolve(
                &requirements,
                &ToolchainCandidates::new(),
            )
            .expect("resolution should produce a report");

        let result = report
            .get(&requirement_id("llvm.compiler"))
            .expect("requirement result");

        assert_eq!(
            result.status(),
            ResolutionStatus::MissingRequired
        );

        assert!(!report.required_requirements_satisfied());
    }

    #[test]
    fn missing_optional_requirement_does_not_fail_required_resolution() {
        let requirements = ExternalToolRequirements::from_iter([
            requirement("llvm.compiler"),
            requirement("optional.optimizer")
                .kind(RequirementKind::Optional),
        ])
        .expect("valid requirements");

        let report = ExternalToolResolver::new()
            .resolve(
                &requirements,
                &ToolchainCandidates::new(),
            )
            .expect("resolution report");

        assert_eq!(report.required_failure_count(), 1);
        assert_eq!(report.optional_failure_count(), 1);

        let optional = report
            .get(&requirement_id("optional.optimizer"))
            .expect("optional result");

        assert_eq!(
            optional.status(),
            ResolutionStatus::MissingOptional
        );
    }

    #[test]
    fn version_constraint_is_checked_against_discovered_version() {
        let constraint = VersionConstraint::any()
            .at_least(NumericVersion::new(2, 0, 0));

        assert!(
            constraint
                .matches(NumericVersion::new(2, 1, 0))
                .expect("valid constraint")
        );

        assert!(
            !constraint
                .matches(NumericVersion::new(1, 9, 0))
                .expect("valid constraint")
        );
    }

    #[test]
    fn version_constraint_supports_exact_versions() {
        let constraint = VersionConstraint::any()
            .exactly(NumericVersion::new(3, 2, 1));

        assert!(
            constraint
                .matches(NumericVersion::new(3, 2, 1))
                .expect("valid constraint")
        );

        assert!(
            !constraint
                .matches(NumericVersion::new(3, 2, 2))
                .expect("valid constraint")
        );
    }

    #[test]
    fn resolution_status_classifies_failures_correctly() {
        assert!(
            ResolutionStatus::MissingRequired
                .is_required_failure()
        );

        assert!(
            ResolutionStatus::FailedOptional
                .is_optional_failure()
        );

        assert!(
            ResolutionStatus::ResolvedRequired
                .is_resolved()
        );

        assert!(
            !ResolutionStatus::MissingOptional
                .is_required_failure()
        );
    }

    #[test]
    fn cancellation_is_observed_before_discovery() {
        let requirements = ExternalToolRequirements::from_iter([
            requirement("llvm.compiler"),
        ])
        .expect("valid requirements");

        let candidates = ToolchainCandidates::from_iter([
            executable_candidate(
                "llvm.compiler",
                "provider.test",
                "compiler",
                "clang",
            ),
        ])
        .expect("valid candidate");

        let cancellation = CancellationToken::new();
        cancellation.cancel();

        let report = ExternalToolResolver::new()
            .resolve_with_cancellation(
                &requirements,
                &candidates,
                &cancellation,
            )
            .expect("resolution report");

        let result = report
            .get(&requirement_id("llvm.compiler"))
            .expect("requirement result");

        assert_eq!(
            result.status(),
            ResolutionStatus::Cancelled
        );

        assert!(report.was_cancelled());
    }

    #[test]
    fn empty_requirements_produce_an_empty_successful_report() {
        let report = ExternalToolResolver::new()
            .resolve(
                &ExternalToolRequirements::new(),
                &ToolchainCandidates::new(),
            )
            .expect("empty resolution");

        assert!(report.is_empty());
        assert!(report.required_requirements_satisfied());
        assert!(report.all_requirements_resolved());
    }

    #[test]
    fn requirements_remain_domain_neutral() {
        let requirement = requirement("future.domain.tool")
            .domain(
                DomainId::new("neuromorphic")
                    .expect("valid domain"),
            )
            .capability(
                CapabilityId::new("future.capability")
                    .expect("valid capability"),
            );

        assert_eq!(requirement.domains().len(), 1);
        assert_eq!(requirement.capabilities().len(), 1);
    }

    #[test]
    fn toolchain_spec_is_not_replaced_by_a_second_process_api() {
        let candidate = executable_candidate(
            "llvm.compiler",
            "provider.llvm",
            "clang",
            "clang",
        );

        assert_eq!(
            candidate.specification().id(),
            "provider.llvm.clang"
        );

        assert_eq!(
            candidate.requested_executable(),
            Path::new("clang")
        );
    }
}
