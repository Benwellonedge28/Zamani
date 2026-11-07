
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Backend Compatibility Integration
//!
//! Path: src/compiler/backends/compatibility.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Integrates the canonical backend compatibility evaluator with
//! the compiler backend-selection infrastructure.
//!
//! Authoritative compatibility implementation:
//!
//!     crate::backend::compatibility
//!
//! Authoritative backend registry:
//!
//!     crate::backend::registry
//!
//! Authoritative backend selection:
//!
//!     crate::compiler::backends::selection
//!
//! This module is an adapter. It does not implement another
//! capability model, resource model, target registry,
//! or compatibility algorithm.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Backend-to-target compatibility profile association.
//! - Compiler-facing profile provider contract.
//! - Detailed backend compatibility evaluation.
//! - Adaptation into BackendEligibility.
//! - Integration-boundary validation and errors.
//!
//! ## Non-responsibilities
//!
//! This module does not own:
//!
//! - Source-language semantics.
//! - Grammar or parsing.
//! - Semantic capability requirements.
//! - Capability identifiers.
//! - Resource identifiers or quantities.
//! - Target descriptors.
//! - Backend registration.
//! - Hardware discovery.
//! - Resource allocation.
//! - Resource feasibility planning.
//! - Scheduling or routing.
//! - IR construction or verification.
//! - Code generation.
//! - Artifact validation.
//! - Runtime execution.
//!
//! ## Scalability
//!
//! No arbitrary universal limits are imposed on:
//!
//! - Backends.
//! - Targets.
//! - Capabilities.
//! - Resource dimensions.
//! - Resource quantities.
//! - Computational domains.
//! - Hardware architectures.
//! - Distributed participants.
//!
//! Resource representation and evaluation are delegated to
//! the canonical backend compatibility implementation.
//!
//! Actual resource exhaustion remains an explicit failure.
//!
//! Literal infinite physical computation is not claimed.
//!
//! ## Safety
//!
//! Unsafe Rust is forbidden.
//!
//! External profile-provider results must be validated.
//!
//! Unknown compatibility must never be treated as success.

use std::error::Error;
use std::fmt;

use crate::backend::compatibility::{
    CompatibilityEvaluator,
    CompatibilityIssue,
    CompatibilityReport,
    CompatibilityRequest,
    CompatibilitySeverity,
    CompatibilityStatus,
    TargetCapabilityProfile,
};

use crate::backend::registry::BackendDescriptor;

use crate::compiler::backends::selection::{
    BackendEligibility,
    BackendEligibilityEvaluator,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable architectural identity of this module.
pub const BACKEND_COMPATIBILITY_MODULE_NAME: &str =
    "zamani.compiler.backends.compatibility";

/// Public API version of this adapter.
pub const BACKEND_COMPATIBILITY_API_VERSION: u16 = 1;

// ============================================================================
// ERRORS
// ============================================================================

/// Errors occurring at the compiler/backend compatibility boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BackendCompatibilityError {
    /// Backend identity is empty or malformed.
    InvalidBackendIdentifier {
        /// Supplied backend identifier.
        backend_id: String,

        /// Explanation of the validation failure.
        reason: String,
    },

    /// A target profile is associated with an invalid backend.
    InvalidProfileBinding {
        /// Backend identifier.
        backend_id: String,

        /// Explanation of the validation failure.
        reason: String,
    },

    /// Provider returned a profile belonging to another backend.
    BackendIdentityMismatch {
        /// Backend requested by the compiler.
        requested_backend_id: String,

        /// Backend associated with the returned profile.
        profile_backend_id: String,
    },

    /// The profile provider failed.
    ProviderFailure {
        /// Backend whose profile was requested.
        backend_id: String,

        /// Provider failure explanation.
        reason: String,
    },
}

impl fmt::Display for BackendCompatibilityError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidBackendIdentifier {
                backend_id,
                reason,
            } => {
                write!(
                    formatter,
                    "Invalid backend identifier `{backend_id}`: {reason}"
                )
            }

            Self::InvalidProfileBinding {
                backend_id,
                reason,
            } => {
                write!(
                    formatter,
                    "Invalid compatibility profile binding for \
                     backend `{backend_id}`: {reason}"
                )
            }

            Self::BackendIdentityMismatch {
                requested_backend_id,
                profile_backend_id,
            } => {
                write!(
                    formatter,
                    "Backend identity mismatch: requested \
                     `{requested_backend_id}`, received \
                     `{profile_backend_id}`"
                )
            }

            Self::ProviderFailure {
                backend_id,
                reason,
            } => {
                write!(
                    formatter,
                    "Failed to obtain compatibility profile \
                     for backend `{backend_id}`: {reason}"
                )
            }
        }
    }
}

impl Error for BackendCompatibilityError {}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

/// Validate a backend identifier at the integration boundary.
///
/// Canonical identifier syntax and uniqueness remain the responsibility
/// of the backend registry.
///
/// This function validates independently supplied identifiers without
/// imposing an arbitrary maximum identifier length.
fn validate_backend_identifier(
    backend_id: &str,
) -> Result<(), BackendCompatibilityError> {
    if backend_id.trim().is_empty() {
        return Err(
            BackendCompatibilityError::InvalidBackendIdentifier {
                backend_id: backend_id.to_owned(),
                reason: "identifier must not be empty".to_owned(),
            },
        );
    }

    if backend_id.chars().any(char::is_control) {
        return Err(
            BackendCompatibilityError::InvalidBackendIdentifier {
                backend_id: backend_id.to_owned(),
                reason: "identifier contains control characters".to_owned(),
            },
        );
    }

    if backend_id.trim() != backend_id {
        return Err(
            BackendCompatibilityError::InvalidBackendIdentifier {
                backend_id: backend_id.to_owned(),
                reason: "identifier contains surrounding whitespace".to_owned(),
            },
        );
    }

    Ok(())
}

/// Validate a canonical target profile identity.
///
/// Backend and target identities are intentionally independent.
///
/// One backend may support multiple targets.
fn validate_target_identity(
    backend_id: &str,
    target_id: &str,
) -> Result<(), BackendCompatibilityError> {
    if target_id.trim().is_empty() {
        return Err(
            BackendCompatibilityError::InvalidProfileBinding {
                backend_id: backend_id.to_owned(),
                reason: "target identifier must not be empty".to_owned(),
            },
        );
    }

    if target_id.chars().any(char::is_control) {
        return Err(
            BackendCompatibilityError::InvalidProfileBinding {
                backend_id: backend_id.to_owned(),
                reason: "target identifier contains control characters"
                    .to_owned(),
            },
        );
    }

    if target_id.trim() != target_id {
        return Err(
            BackendCompatibilityError::InvalidProfileBinding {
                backend_id: backend_id.to_owned(),
                reason: "target identifier contains surrounding whitespace"
                    .to_owned(),
            },
        );
    }

    Ok(())
}

// ============================================================================
// BACKEND-TO-TARGET BINDING
// ============================================================================

/// Explicit association between a registered backend and its
/// canonical target compatibility profile.
///
/// This type reuses TargetCapabilityProfile directly.
///
/// It does not create another capability or resource representation.
///
/// Backend identity identifies the registered implementation.
///
/// Target identity identifies the environment described by the profile.
///
/// These identities must never be assumed to be identical.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendCompatibilityBinding {
    backend_id: String,
    profile: TargetCapabilityProfile,
}

impl BackendCompatibilityBinding {
    /// Create an explicit backend-to-target association.
    ///
    /// # Errors
    ///
    /// Returns an error when either identifier is empty, contains
    /// control characters, or contains surrounding whitespace.
    pub fn new(
        backend_id: impl Into<String>,
        profile: TargetCapabilityProfile,
    ) -> Result<Self, BackendCompatibilityError> {
        let backend_id = backend_id.into();

        validate_backend_identifier(&backend_id)?;

        validate_target_identity(
            &backend_id,
            &profile.target_id,
        )?;

        Ok(Self {
            backend_id,
            profile,
        })
    }

    /// Return the associated canonical backend identifier.
    #[must_use]
    pub fn backend_id(&self) -> &str {
        &self.backend_id
    }

    /// Return the target identifier.
    #[must_use]
    pub fn target_id(&self) -> &str {
        &self.profile.target_id
    }

    /// Return the authoritative target capability profile.
    #[must_use]
    pub fn profile(&self) -> &TargetCapabilityProfile {
        &self.profile
    }

    /// Consume the binding and return its target profile.
    #[must_use]
    pub fn into_profile(self) -> TargetCapabilityProfile {
        self.profile
    }

    /// Validate the association against a requested backend.
    ///
    /// This check is repeated when a provider returns a binding.
    ///
    /// Construction-time validation is not sufficient when providers
    /// obtain metadata from external or serialized sources.
    ///
    /// # Errors
    ///
    /// Returns an error if the binding belongs to another backend
    /// or its target identity is invalid.
    pub fn validate_for(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<(), BackendCompatibilityError> {
        validate_backend_identifier(&backend.id)?;

        if self.backend_id != backend.id {
            return Err(
                BackendCompatibilityError::BackendIdentityMismatch {
                    requested_backend_id: backend.id.clone(),
                    profile_backend_id: self.backend_id.clone(),
                },
            );
        }

        validate_backend_identifier(&self.backend_id)?;

        validate_target_identity(
            &self.backend_id,
            &self.profile.target_id,
        )
    }
}

// ============================================================================
// COMPATIBILITY PROFILE PROVIDER
// ============================================================================

/// Supplies explicit target compatibility profiles for registered backends.
///
/// Providers may obtain profiles from:
///
/// - Backend metadata.
/// - Validated toolchain descriptions.
/// - Target adapters.
/// - Deployment descriptions.
/// - Trusted capability evidence.
/// - Explicit application configuration.
///
/// Providers must never infer compatibility from:
///
/// - Backend names.
/// - Vendor names.
/// - File extensions.
/// - Architecture strings.
/// - Display names.
///
/// Missing profile information means compatibility is unknown.
///
/// Provider implementations must be safe for concurrent use.
pub trait BackendCompatibilityProfileProvider: Send + Sync {
    /// Obtain a compatibility profile for a registered backend.
    ///
    /// The returned binding must identify the requested backend.
    ///
    /// Returning `Ok(None)` means no usable profile is available.
    ///
    /// It does not mean the backend is incompatible.
    ///
    /// # Errors
    ///
    /// Returns a descriptive error if profile retrieval fails.
    fn profile_for(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<
        Option<BackendCompatibilityBinding>,
        String,
    >;
}

/// Allow compatible closures to act as profile providers.
///
/// This avoids introducing another provider registry.
impl<F> BackendCompatibilityProfileProvider for F
where
    F: Fn(
            &BackendDescriptor,
        ) -> Result<
            Option<BackendCompatibilityBinding>,
            String,
        > + Send
        + Sync,
{
    fn profile_for(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<
        Option<BackendCompatibilityBinding>,
        String,
    > {
        self(backend)
    }
}

// ============================================================================
// DETAILED EVALUATION
// ============================================================================

/// Detailed compiler-facing compatibility evaluation.
///
/// The original canonical compatibility report is retained.
///
/// `None` report means that no authoritative target profile
/// was available or compatibility was not established.
///
/// An unknown result is never eligible for backend selection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendCompatibilityEvaluation {
    backend_id: String,
    target_id: Option<String>,
    report: Option<CompatibilityReport>,
    eligibility: BackendEligibility,
}

impl BackendCompatibilityEvaluation {
    /// Construct an unknown compatibility result.
    fn unknown(
        backend_id: String,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            backend_id,
            target_id: None,
            report: None,
            eligibility: BackendEligibility::Unknown {
                reason: reason.into(),
            },
        }
    }

    /// Adapt the authoritative compatibility report.
    fn from_report(
        backend_id: String,
        report: CompatibilityReport,
    ) -> Self {
        let target_id = Some(report.target_id.clone());

        let summary = summarize_issues(&report);

        let eligibility = match report.status {
            CompatibilityStatus::Compatible => {
                BackendEligibility::Compatible {
                    rank: 0,
                    explanation: format!(
                        "Backend `{backend_id}` is compatible with \
                         target `{}`.",
                        report.target_id
                    ),
                }
            }

            CompatibilityStatus::CompatibleWithWarnings => {
                BackendEligibility::Compatible {
                    rank: 0,
                    explanation: format!(
                        "Backend `{backend_id}` is compatible with \
                         target `{}`, with warnings: {summary}",
                        report.target_id
                    ),
                }
            }

            CompatibilityStatus::Indeterminate => {
                BackendEligibility::Unknown {
                    reason: format!(
                        "Compatibility for backend `{backend_id}` \
                         and target `{}` is indeterminate: {summary}",
                        report.target_id
                    ),
                }
            }

            CompatibilityStatus::Incompatible => {
                BackendEligibility::Incompatible {
                    reason: format!(
                        "Backend `{backend_id}` cannot satisfy \
                         the requirements of target `{}`: {summary}",
                        report.target_id
                    ),
                }
            }
        };

        Self {
            backend_id,
            target_id,
            report: Some(report),
            eligibility,
        }
    }

    /// Return the canonical backend identifier.
    #[must_use]
    pub fn backend_id(&self) -> &str {
        &self.backend_id
    }

    /// Return the target identifier when a profile was evaluated.
    #[must_use]
    pub fn target_id(&self) -> Option<&str> {
        self.target_id.as_deref()
    }

    /// Return the original canonical compatibility report.
    #[must_use]
    pub fn report(&self) -> Option<&CompatibilityReport> {
        self.report.as_ref()
    }

    /// Consume the evaluation and return its canonical report.
    #[must_use]
    pub fn into_report(self) -> Option<CompatibilityReport> {
        self.report
    }

    /// Return backend-selection eligibility.
    #[must_use]
    pub fn eligibility(&self) -> &BackendEligibility {
        &self.eligibility
    }

    /// Whether compatibility has been established.
    ///
    /// Warning-bearing compatible results are included.
    #[must_use]
    pub fn is_compatible(&self) -> bool {
        matches!(
            self.eligibility,
            BackendEligibility::Compatible { .. }
        )
    }

    /// Whether compatibility was established without warnings.
    #[must_use]
    pub fn is_fully_compatible(&self) -> bool {
        self.report
            .as_ref()
            .is_some_and(|report| {
                report.status == CompatibilityStatus::Compatible
            })
    }

    /// Whether the canonical report contains warnings.
    #[must_use]
    pub fn has_warnings(&self) -> bool {
        self.report
            .as_ref()
            .is_some_and(|report| {
                report.issues.iter().any(|issue| {
                    issue.severity == CompatibilitySeverity::Warning
                })
            })
    }

    /// Whether compatibility remains unknown.
    #[must_use]
    pub fn is_indeterminate(&self) -> bool {
        matches!(
            self.eligibility,
            BackendEligibility::Unknown { .. }
        )
    }

    /// Whether compatibility has been explicitly rejected.
    #[must_use]
    pub fn is_incompatible(&self) -> bool {
        matches!(
            self.eligibility,
            BackendEligibility::Incompatible { .. }
        )
    }
}

// ============================================================================
// DETERMINISTIC DIAGNOSTIC SUMMARY
// ============================================================================

/// Produce a summary while preserving the canonical report ordering.
///
/// No second diagnostic ordering policy is introduced.
///
/// The canonical evaluator remains responsible for ordering issues.
fn summarize_issues(report: &CompatibilityReport) -> String {
    if report.issues.is_empty() {
        return match report.status {
            CompatibilityStatus::Indeterminate => {
                "the evaluator could not establish compatibility".to_owned()
            }

            CompatibilityStatus::Incompatible => {
                "the evaluator established incompatibility".to_owned()
            }

            CompatibilityStatus::CompatibleWithWarnings => {
                "compatibility was established with warnings".to_owned()
            }

            CompatibilityStatus::Compatible => {
                "no compatibility issues were reported".to_owned()
            }
        };
    }

    let mut summary = String::new();

    for issue in &report.issues {
        if !summary.is_empty() {
            summary.push_str("; ");
        }

        summary.push_str(&issue.message);
    }

    summary
}

// ============================================================================
// CANONICAL COMPATIBILITY EVALUATOR ADAPTER
// ============================================================================

/// Compiler-facing adapter over the canonical backend compatibility engine.
///
/// The evaluator:
///
/// 1. Receives a registered backend descriptor.
/// 2. Requests its explicit target profile.
/// 3. Validates backend-to-target identity.
/// 4. Delegates compatibility evaluation.
/// 5. Retains the canonical compatibility report.
/// 6. Converts the result into backend-selection eligibility.
///
/// It does not own a backend registry.
///
/// It does not perform physical resource allocation.
///
/// It does not estimate performance.
///
/// It does not perform code generation.
pub struct BackendCompatibilityEvaluator<'a> {
    provider: &'a dyn BackendCompatibilityProfileProvider,
    request: CompatibilityRequest,
    evaluator: CompatibilityEvaluator,
}

impl<'a> BackendCompatibilityEvaluator<'a> {
    /// Construct an evaluator using an explicit provider and request.
    #[must_use]
    pub fn new(
        provider: &'a dyn BackendCompatibilityProfileProvider,
        request: CompatibilityRequest,
    ) -> Self {
        Self {
            provider,
            request,
            evaluator: CompatibilityEvaluator,
        }
    }

    /// Return the canonical compatibility request.
    #[must_use]
    pub fn request(&self) -> &CompatibilityRequest {
        &self.request
    }

    /// Evaluate one backend and retain its complete compatibility report.
    ///
    /// An empty requirement set deliberately produces an unknown result.
    ///
    /// An empty request cannot establish that a backend is suitable.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The backend identifier is malformed.
    /// - The profile provider fails.
    /// - The provider returns a profile for another backend.
    /// - The target identity is invalid.
    ///
    /// A missing profile is represented as unknown, not compatible.
    pub fn evaluate_detailed(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<
        BackendCompatibilityEvaluation,
        BackendCompatibilityError,
    > {
        validate_backend_identifier(&backend.id)?;

        // Empty requirements are not evidence of compatibility.
        if self.request.capabilities.is_empty()
            && self.request.resources.is_empty()
        {
            return Ok(
                BackendCompatibilityEvaluation::unknown(
                    backend.id.clone(),
                    "No capability or resource requirements were supplied; \
                     backend compatibility has not been established.",
                ),
            );
        }

        // Retrieve an explicit backend-to-target association.
        let binding = self
            .provider
            .profile_for(backend)
            .map_err(|reason| {
                BackendCompatibilityError::ProviderFailure {
                    backend_id: backend.id.clone(),
                    reason,
                }
            })?;

        let Some(binding) = binding else {
            return Ok(
                BackendCompatibilityEvaluation::unknown(
                    backend.id.clone(),
                    "No authoritative target compatibility profile \
                     is available for this backend.",
                ),
            );
        };

        // Never combine evidence belonging to different backends.
        binding.validate_for(backend)?;

        // The canonical evaluator is the sole compatibility authority.
        let report = self.evaluator.evaluate(
            &self.request,
            binding.profile(),
        );

        Ok(
            BackendCompatibilityEvaluation::from_report(
                backend.id.clone(),
                report,
            ),
        )
    }
}

// ============================================================================
// BACKEND SELECTION INTEGRATION
// ============================================================================

impl BackendEligibilityEvaluator for BackendCompatibilityEvaluator<'_> {
    /// Evaluate one registered backend using canonical compatibility.
    ///
    /// Compatibility ranking is neutral.
    ///
    /// This adapter does not invent latency, throughput, energy,
    /// monetary cost, or performance estimates.
    ///
    /// A separate optimization or cost-model subsystem may rank
    /// already compatible and feasible candidates.
    ///
    /// Unknown and incompatible results are never eligible.
    fn evaluate(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<BackendEligibility, String> {
        self.evaluate_detailed(backend)
            .map(|evaluation| evaluation.eligibility)
            .map_err(|error| error.to_string())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::backend::compatibility::{
        CapabilityId,
        CapabilityRequirement,
        CapabilitySupport,
        CompatibilityPolicy,
        DecimalInteger,
        RequirementStrength,
        ResourceAvailability,
        ResourceId,
        ResourceRequirement,
        SupportLevel,
    };

    use std::collections::BTreeMap;

    fn backend(id: &str) -> BackendDescriptor {
        BackendDescriptor::new(id, ".test")
    }

    fn profile(target_id: &str) -> TargetCapabilityProfile {
        TargetCapabilityProfile::new(target_id)
            .expect("valid target profile")
    }

    fn supported_profile(
        target_id: &str,
        capability: &str,
    ) -> TargetCapabilityProfile {
        let mut profile = profile(target_id);

        profile
            .declare_capability(
                CapabilitySupport::new(
                    CapabilityId::new(capability)
                        .expect("valid capability"),
                    SupportLevel::Supported,
                )
                .with_evidence("verified test evidence"),
            )
            .expect("capability declaration succeeds");

        profile
    }

    #[derive(Clone, Default)]
    struct TestProvider {
        bindings: BTreeMap<
            String,
            BackendCompatibilityBinding,
        >,
    }

    impl TestProvider {
        fn insert(
            &mut self,
            binding: BackendCompatibilityBinding,
        ) {
            self.bindings.insert(
                binding.backend_id().to_owned(),
                binding,
            );
        }
    }

    impl BackendCompatibilityProfileProvider for TestProvider {
        fn profile_for(
            &self,
            backend: &BackendDescriptor,
        ) -> Result<
            Option<BackendCompatibilityBinding>,
            String,
        > {
            Ok(self.bindings.get(&backend.id).cloned())
        }
    }

    fn required_capability(
        name: &str,
    ) -> CompatibilityRequest {
        CompatibilityRequest::new().with_capability(
            CapabilityRequirement::new(
                CapabilityId::new(name)
                    .expect("valid capability"),
                RequirementStrength::Required,
            ),
        )
    }

    fn evaluator<'a>(
        provider: &'a dyn BackendCompatibilityProfileProvider,
        request: CompatibilityRequest,
    ) -> BackendCompatibilityEvaluator<'a> {
        BackendCompatibilityEvaluator::new(provider, request)
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            BACKEND_COMPATIBILITY_MODULE_NAME,
            "zamani.compiler.backends.compatibility"
        );
    }

    #[test]
    fn api_version_is_nonzero() {
        assert_ne!(BACKEND_COMPATIBILITY_API_VERSION, 0);
    }

    #[test]
    fn backend_and_target_ids_are_independent() {
        let binding = BackendCompatibilityBinding::new(
            "backend.native",
            profile("target.cpu.variant"),
        )
        .expect("valid binding");

        assert_eq!(binding.backend_id(), "backend.native");
        assert_eq!(binding.target_id(), "target.cpu.variant");
    }

    #[test]
    fn empty_backend_identifier_is_rejected() {
        assert!(
            validate_backend_identifier("").is_err()
        );
    }

    #[test]
    fn whitespace_backend_identifier_is_rejected() {
        assert!(
            validate_backend_identifier(" backend.example").is_err()
        );
    }

    #[test]
    fn control_characters_are_rejected() {
        assert!(
            validate_backend_identifier("backend\nexample").is_err()
        );
    }

    #[test]
    fn empty_target_identifier_is_rejected() {
        assert!(
            validate_target_identity("backend.example", "").is_err()
        );
    }

    #[test]
    fn binding_rejects_wrong_backend() {
        let binding = BackendCompatibilityBinding::new(
            "backend.first",
            profile("target.example"),
        )
        .expect("valid binding");

        let result = binding.validate_for(
            &backend("backend.second"),
        );

        assert!(matches!(
            result,
            Err(BackendCompatibilityError::BackendIdentityMismatch { .. })
        ));
    }

    #[test]
    fn missing_profile_is_unknown() {
        let provider = TestProvider::default();

        let evaluator = evaluator(
            &provider,
            required_capability("cpu.integer"),
        );

        let result = evaluator
            .evaluate_detailed(&backend("backend.example"))
            .expect("evaluation succeeds");

        assert!(result.is_indeterminate());
        assert!(!result.is_compatible());
        assert!(result.report().is_none());
    }

    #[test]
    fn empty_requirements_are_unknown() {
        let provider = TestProvider::default();

        let evaluator = evaluator(
            &provider,
            CompatibilityRequest::new(),
        );

        let result = evaluator
            .evaluate_detailed(&backend("backend.example"))
            .expect("evaluation succeeds");

        assert!(result.is_indeterminate());
        assert!(!result.is_compatible());
    }

    #[test]
    fn supported_required_capability_is_compatible() {
        let mut provider = TestProvider::default();

        provider.insert(
            BackendCompatibilityBinding::new(
                "backend.example",
                supported_profile(
                    "target.example",
                    "quantum.measurement",
                ),
            )
            .expect("valid binding"),
        );

        let evaluator = evaluator(
            &provider,
            required_capability("quantum.measurement"),
        );

        let result = evaluator
            .evaluate_detailed(&backend("backend.example"))
            .expect("evaluation succeeds");

        assert!(result.is_compatible());
        assert!(result.is_fully_compatible());
        assert!(!result.has_warnings());
        assert_eq!(result.target_id(), Some("target.example"));
    }

    #[test]
    fn unsupported_hard_requirement_is_incompatible() {
        let mut target = profile("target.example");

        target
            .declare_capability(
                CapabilitySupport::new(
                    CapabilityId::new("quantum.measurement")
                        .expect("valid capability"),
                    SupportLevel::Unsupported,
                ),
            )
            .expect("declaration succeeds");

        let mut provider = TestProvider::default();

        provider.insert(
            BackendCompatibilityBinding::new(
                "backend.example",
                target,
            )
            .expect("valid binding"),
        );

        let evaluator = evaluator(
            &provider,
            required_capability("quantum.measurement"),
        );

        let result = evaluator
            .evaluate_detailed(&backend("backend.example"))
            .expect("evaluation succeeds");

        assert!(result.is_incompatible());
        assert!(!result.is_compatible());
    }

    #[test]
    fn partial_support_does_not_satisfy_hard_requirement() {
        let mut target = profile("target.example");

        target
            .declare_capability(
                CapabilitySupport::new(
                    CapabilityId::new("quantum.measurement")
                        .expect("valid capability"),
                    SupportLevel::Partial,
                ),
            )
            .expect("declaration succeeds");

        let mut provider = TestProvider::default();

        provider.insert(
            BackendCompatibilityBinding::new(
                "backend.example",
                target,
            )
            .expect("valid binding"),
        );

        let evaluator = evaluator(
            &provider,
            required_capability("quantum.measurement"),
        );

        let result = evaluator
            .evaluate_detailed(&backend("backend.example"))
            .expect("evaluation succeeds");

        assert!(result.is_incompatible());
    }

    #[test]
    fn experimental_support_requires_explicit_policy() {
        let mut target = profile("target.example");

        target
            .declare_capability(
                CapabilitySupport::new(
                    CapabilityId::new("quantum.measurement")
                        .expect("valid capability"),
                    SupportLevel::Experimental,
                ),
            )
            .expect("declaration succeeds");

        let mut provider = TestProvider::default();

        provider.insert(
            BackendCompatibilityBinding::new(
                "backend.example",
                target,
            )
            .expect("valid binding"),
        );

        let strict = evaluator(
            &provider,
            required_capability("quantum.measurement"),
        );

        let strict_result = strict
            .evaluate_detailed(&backend("backend.example"))
            .expect("evaluation succeeds");

        assert!(strict_result.is_incompatible());

        let permissive_request =
            required_capability("quantum.measurement")
                .with_policy(CompatibilityPolicy {
                    allow_experimental_for_hard_requirements: true,
                });

        let permissive = evaluator(
            &provider,
            permissive_request,
        );

        let permissive_result = permissive
            .evaluate_detailed(&backend("backend.example"))
            .expect("evaluation succeeds");

        assert!(permissive_result.is_compatible());
        assert!(permissive_result.has_warnings());
    }

    #[test]
    fn resource_quantities_are_not_limited_to_u64() {
        let mut target = profile("target.example");

        let capacity = DecimalInteger::new(
            "100000000000000000000000000000000000000000000000000",
        )
        .expect("valid arbitrary-length quantity");

        target
            .declare_resource(
                ResourceId::new("memory.bytes")
                    .expect("valid resource"),
                ResourceAvailability::known(capacity),
            )
            .expect("resource declaration succeeds");

        let mut provider = TestProvider::default();

        provider.insert(
            BackendCompatibilityBinding::new(
                "backend.example",
                target,
            )
            .expect("valid binding"),
        );

        let request = CompatibilityRequest::new()
            .with_resource(
                ResourceRequirement::required(
                    ResourceId::new("memory.bytes")
                        .expect("valid resource"),
                    DecimalInteger::new(
                        "99999999999999999999999999999999999999999999999",
                    )
                    .expect("valid quantity"),
                ),
            );

        let evaluator = evaluator(&provider, request);

        let result = evaluator
            .evaluate_detailed(&backend("backend.example"))
            .expect("evaluation succeeds");

        assert!(result.is_compatible());
    }

    #[test]
    fn provider_failure_is_not_silent_success() {
        let provider = |_backend: &BackendDescriptor| {
            Err("target discovery failed".to_owned())
                as Result<
                    Option<BackendCompatibilityBinding>,
                    String,
                >
        };

        let evaluator = evaluator(
            &provider,
            required_capability("cpu.integer"),
        );

        let result = evaluator
            .evaluate_detailed(&backend("backend.example"));

        assert!(matches!(
            result,
            Err(BackendCompatibilityError::ProviderFailure { .. })
        ));
    }

    #[test]
    fn selection_adapter_preserves_unknown_status() {
        let provider = TestProvider::default();

        let evaluator = evaluator(
            &provider,
            required_capability("cpu.integer"),
        );

        let result = BackendEligibilityEvaluator::evaluate(
            &evaluator,
            &backend("backend.example"),
        )
        .expect("evaluation succeeds");

        assert!(matches!(
            result,
            BackendEligibility::Unknown { .. }
        ));
    }

    #[test]
    fn detailed_report_remains_available() {
        let mut provider = TestProvider::default();

        provider.insert(
            BackendCompatibilityBinding::new(
                "backend.example",
                supported_profile(
                    "target.example",
                    "cpu.integer",
                ),
            )
            .expect("valid binding"),
        );

        let evaluator = evaluator(
            &provider,
            required_capability("cpu.integer"),
        );

        let result = evaluator
            .evaluate_detailed(&backend("backend.example"))
            .expect("evaluation succeeds");

        assert!(result.report().is_some());

        let owned_report = result.into_report();

        assert!(owned_report.is_some());
    }
}
