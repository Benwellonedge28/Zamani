
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Backend Compatibility Adapter
//!
//! Path: `src/compiler/backends/compatibility.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Adapt the canonical backend compatibility engine to the compiler's
//! backend-selection interface.
//!
//! The authoritative compatibility implementation is:
//!
//!     crate::backend::compatibility
//!
//! The authoritative backend-selection interface is:
//!
//!     crate::compiler::backends::selection
//!
//! This module connects those existing contracts. It does not implement
//! another capability model, resource model, target registry, or
//! compatibility algorithm.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Explicit association between a backend ID and a target profile.
//! - The compiler-facing compatibility-profile provider interface.
//! - Adaptation of canonical compatibility reports to backend eligibility.
//! - Structured errors at this integration boundary.
//!
//! This module does not own:
//!
//! - Source-language semantics.
//! - Semantic capability requirements.
//! - Capability identifiers.
//! - Resource identifiers or quantities.
//! - Target descriptors.
//! - Backend registration.
//! - Backend construction.
//! - Target discovery.
//! - Resource planning.
//! - Hardware allocation.
//! - Scheduling or routing.
//! - IR verification or transformation.
//! - Code generation.
//! - Artifact validation.
//! - Runtime execution.
//!
//! ## Important distinctions
//!
//! Registration is not compatibility.
//!
//! Compatibility is not resource allocation.
//!
//! A target profile is not proof of physical availability.
//!
//! A compatibility report is not proof that a backend can generate a
//! valid executable artifact.
//!
//! Unknown evidence is never treated as successful compatibility.
//!
//! An empty requirement set is not proof of compatibility.
//!
//! ## Scalability
//!
//! No fixed maximum is imposed on:
//!
//! - Backends.
//! - Targets.
//! - Capabilities.
//! - Resource dimensions.
//! - Resource quantities.
//! - Computational domains.
//! - Hardware architectures.
//!
//! The adapter uses the canonical compatibility types and collections.
//! Actual resource exhaustion remains an explicit implementation failure.
//!
//! ## Integration
//!
//! The parent module must declare:
//!
//!     pub mod compatibility;
//!
//! The compiler root must expose the backend module.
//!
//! The backend root must expose the canonical compatibility module.
//!
//! The canonical backend registry must remain the sole backend registry.

use std::error::Error;
use std::fmt;

use crate::backend::compatibility::{
    CompatibilityEvaluator,
    CompatibilityReport,
    CompatibilityRequest,
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

/// Errors originating at the compiler/backend compatibility boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BackendCompatibilityError {
    /// A backend identifier was empty or malformed.
    InvalidBackendIdentifier {
        /// Supplied backend identifier.
        backend_id: String,
        /// Explanation of the validation failure.
        reason: String,
    },

    /// A target profile was not associated with a valid backend identifier.
    InvalidProfileBinding {
        /// Backend identifier supplied to the binding.
        backend_id: String,
        /// Explanation of the validation failure.
        reason: String,
    },

    /// A provider returned a profile associated with another backend.
    BackendIdentityMismatch {
        /// Backend requested by the compiler.
        requested_backend_id: String,
        /// Backend identity declared by the returned binding.
        profile_backend_id: String,
    },

    /// The profile provider failed to produce a result.
    ProviderFailure {
        /// Backend whose profile was requested.
        backend_id: String,
        /// Provider's error description.
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
            } => write!(
                formatter,
                "invalid backend identifier `{backend_id}`: {reason}"
            ),

            Self::InvalidProfileBinding {
                backend_id,
                reason,
            } => write!(
                formatter,
                "invalid compatibility profile binding for \
                 backend `{backend_id}`: {reason}"
            ),

            Self::BackendIdentityMismatch {
                requested_backend_id,
                profile_backend_id,
            } => write!(
                formatter,
                "backend compatibility profile identity mismatch: \
                 requested `{requested_backend_id}`, received \
                 `{profile_backend_id}`"
            ),

            Self::ProviderFailure {
                backend_id,
                reason,
            } => write!(
                formatter,
                "failed to obtain compatibility profile for \
                 backend `{backend_id}`: {reason}"
            ),
        }
    }
}

impl Error for BackendCompatibilityError {}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

/// Validate a backend identifier at the adapter boundary.
///
/// The canonical registry remains responsible for canonical backend
/// identifier validation. This check protects independently supplied
/// profile bindings and provider results.
///
/// No arbitrary length limit is imposed.
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
                reason: "identifier must not contain control characters"
                    .to_owned(),
            },
        );
    }

    if backend_id.trim() != backend_id {
        return Err(
            BackendCompatibilityError::InvalidBackendIdentifier {
                backend_id: backend_id.to_owned(),
                reason: "identifier must not contain surrounding whitespace"
                    .to_owned(),
            },
        );
    }

    Ok(())
}

/// Validate the target identity contained in a canonical profile.
///
/// The target ID is deliberately not required to equal the backend ID.
/// One backend may support several target environments.
fn validate_target_identity(
    backend_id: &str,
    target_id: &str,
) -> Result<(), BackendCompatibilityError> {
    if target_id.trim().is_empty() {
        return Err(
            BackendCompatibilityError::InvalidProfileBinding {
                backend_id: backend_id.to_owned(),
                reason: "target profile identifier must not be empty"
                    .to_owned(),
            },
        );
    }

    if target_id.chars().any(char::is_control) {
        return Err(
            BackendCompatibilityError::InvalidProfileBinding {
                backend_id: backend_id.to_owned(),
                reason: "target profile identifier must not contain \
                         control characters"
                    .to_owned(),
            },
        );
    }

    if target_id.trim() != target_id {
        return Err(
            BackendCompatibilityError::InvalidProfileBinding {
                backend_id: backend_id.to_owned(),
                reason: "target profile identifier must not contain \
                         surrounding whitespace"
                    .to_owned(),
            },
        );
    }

    Ok(())
}

// ============================================================================
// BACKEND-TO-TARGET PROFILE BINDING
// ============================================================================

/// Explicit association between a registered backend and a canonical
/// target capability profile.
///
/// This type does not duplicate the target capability model. It holds the
/// canonical `TargetCapabilityProfile` directly.
///
/// The backend ID identifies the registered backend implementation.
/// The profile's target ID identifies the target environment described by
/// that profile.
///
/// These identifiers are intentionally independent.
///
/// For example, one backend may provide different profiles for multiple
/// targets. A provider can select the appropriate profile using the
/// compilation request and its own authoritative metadata.
///
/// A profile must never be associated with a backend merely because their
/// display names, extensions, vendors, or architecture names appear similar.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendCompatibilityBinding {
    backend_id: String,
    profile: TargetCapabilityProfile,
}

impl BackendCompatibilityBinding {
    /// Create an explicit backend-to-target compatibility binding.
    ///
    /// # Errors
    ///
    /// Returns an error if either identity is empty, contains surrounding
    /// whitespace, or contains control characters.
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

    /// Return the canonical backend identifier.
    #[must_use]
    pub fn backend_id(&self) -> &str {
        &self.backend_id
    }

    /// Return the target identifier described by this profile.
    #[must_use]
    pub fn target_id(&self) -> &str {
        &self.profile.target_id
    }

    /// Return the canonical target capability profile.
    #[must_use]
    pub fn profile(&self) -> &TargetCapabilityProfile {
        &self.profile
    }

    /// Consume the binding and return its canonical target profile.
    #[must_use]
    pub fn into_profile(self) -> TargetCapabilityProfile {
        self.profile
    }

    /// Verify that this binding belongs to the requested backend.
    ///
    /// This check is performed again when a provider returns a binding.
    /// Construction-time validation alone is insufficient because providers
    /// may load or deserialize profile metadata from external sources.
    ///
    /// # Errors
    ///
    /// Returns an identity mismatch if the binding belongs to another
    /// backend.
    pub fn validate_for(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<(), BackendCompatibilityError> {
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
// PROFILE PROVIDER
// ============================================================================

/// Supplies explicit compatibility profiles for registered backends.
///
/// Implementations may source profiles from:
///
/// - Backend metadata.
/// - Validated toolchain descriptions.
/// - Target adapters.
/// - Deployment descriptions.
/// - Trusted capability-evidence providers.
/// - Configuration supplied by the application.
///
/// The provider must not infer capabilities from backend names, file
/// extensions, vendor names, or architecture strings.
///
/// A missing profile means that compatibility cannot be established.
///
/// Provider implementations must be safe for concurrent use because the
/// backend-selection interface requires `Send + Sync`.
pub trait BackendCompatibilityProfileProvider: Send + Sync {
    /// Obtain the compatibility profile associated with a backend.
    ///
    /// The returned binding must explicitly identify the requested backend.
    ///
    /// Returning `Ok(None)` means that no usable compatibility profile is
    /// available. It does not mean that the backend is incompatible.
    ///
    /// # Errors
    ///
    /// Return an error when profile retrieval or validation fails.
    fn profile_for(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<
        Option<BackendCompatibilityBinding>,
        String,
    >;
}

/// Blanket provider implementation for compatible closures.
///
/// This allows applications and tests to provide profiles without
/// introducing another registry or provider wrapper.
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
// EVALUATION RESULT
// ============================================================================

/// Detailed compiler-facing compatibility evaluation.
///
/// The canonical report is retained without converting its issues into
/// another compatibility-report model.
///
/// `None` means no profile was available. It is not a compatible result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendCompatibilityEvaluation {
    backend_id: String,
    target_id: Option<String>,
    report: Option<CompatibilityReport>,
    eligibility: BackendEligibility,
}

impl BackendCompatibilityEvaluation {
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

    fn from_report(
        backend_id: String,
        report: CompatibilityReport,
    ) -> Self {
        let target_id = Some(report.target_id.clone());

        let eligibility = match report.status {
            CompatibilityStatus::Compatible => {
                BackendEligibility::Compatible {
                    rank: 0,
                    explanation: format!(
                        "canonical compatibility evaluation established \
                         compatibility for target `{}`",
                        report.target_id
                    ),
                }
            }

            CompatibilityStatus::CompatibleWithWarnings => {
                let warning_count = report
                    .issues
                    .iter()
                    .filter(|issue| {
                        issue.severity
                            == crate::backend::compatibility::
                                CompatibilitySeverity::Warning
                    })
                    .count();

                BackendEligibility::Compatible {
                    rank: 0,
                    explanation: format!(
                        "canonical compatibility evaluation established \
                         compatibility for target `{}` with {warning_count} \
                         warning(s)",
                        report.target_id
                    ),
                }
            }

            CompatibilityStatus::Indeterminate => {
                BackendEligibility::Unknown {
                    reason: format!(
                        "compatibility for target `{}` is indeterminate: {}",
                        report.target_id,
                        summarize_issues(&report)
                    ),
                }
            }

            CompatibilityStatus::Incompatible => {
                BackendEligibility::Incompatible {
                    reason: format!(
                        "target `{}` does not satisfy the compatibility \
                         request: {}",
                        report.target_id,
                        summarize_issues(&report)
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

    /// Return the registered backend identifier.
    #[must_use]
    pub fn backend_id(&self) -> &str {
        &self.backend_id
    }

    /// Return the target identifier, if a profile was evaluated.
    #[must_use]
    pub fn target_id(&self) -> Option<&str> {
        self.target_id.as_deref()
    }

    /// Return the original canonical compatibility report, if evaluated.
    #[must_use]
    pub fn report(&self) -> Option<&CompatibilityReport> {
        self.report.as_ref()
    }

    /// Return the backend-selection eligibility classification.
    #[must_use]
    pub fn eligibility(&self) -> &BackendEligibility {
        &self.eligibility
    }

    /// Whether compatibility was established.
    #[must_use]
    pub fn is_compatible(&self) -> bool {
        matches!(
            self.eligibility,
            BackendEligibility::Compatible { .. }
        )
    }
}

/// Produce a deterministic summary from the canonical compatibility issues.
///
/// The canonical report already sorts its issues. This adapter preserves
/// that order and does not create a second diagnostic ordering policy.
fn summarize_issues(report: &CompatibilityReport) -> String {
    if report.issues.is_empty() {
        return match report.status {
            CompatibilityStatus::Indeterminate => {
                "the canonical evaluator could not establish compatibility"
                    .to_owned()
            }
            CompatibilityStatus::Incompatible => {
                "the canonical evaluator reported incompatibility"
                    .to_owned()
            }
            _ => "no compatibility issues were reported".to_owned(),
        };
    }

    report
        .issues
        .iter()
        .map(|issue| issue.message.as_str())
        .collect::<Vec<_>>()
        .join("; ")
}

// ============================================================================
// COMPILER COMPATIBILITY EVALUATOR
// ============================================================================

/// Compiler-facing adapter over the canonical backend compatibility engine.
///
/// The evaluator borrows a profile provider and owns the canonical request.
/// It does not own a backend registry.
///
/// The provider is responsible for associating each registered backend
/// descriptor with the correct target profile.
///
/// The canonical `CompatibilityEvaluator` remains the sole authority for
/// evaluating capability and resource requirements.
pub struct BackendCompatibilityEvaluator<'a> {
    provider: &'a dyn BackendCompatibilityProfileProvider,
    request: CompatibilityRequest,
    evaluator: CompatibilityEvaluator,
}

impl<'a> BackendCompatibilityEvaluator<'a> {
    /// Construct an evaluator using an explicit profile provider and
    /// canonical compatibility request.
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

    /// Evaluate a backend and retain its complete canonical report.
    ///
    /// An empty requirement set deliberately produces `Unknown`.
    ///
    /// The canonical backend evaluator can return a compatible report for
    /// an empty request. That behavior is useful for its general API, but
    /// backend selection must not mistake the absence of requirements for
    /// evidence that a backend is suitable.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The profile provider fails.
    /// - The provider returns a profile bound to another backend.
    /// - Profile identity validation fails.
    ///
    /// A missing profile is represented as an evaluation with `Unknown`
    /// eligibility rather than as an error.
    pub fn evaluate_detailed(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<
        BackendCompatibilityEvaluation,
        BackendCompatibilityError,
    > {
        validate_backend_identifier(&backend.id)
            .map_err(|error| error)?;

        if self.request.capabilities.is_empty()
            && self.request.resources.is_empty()
        {
            return Ok(
                BackendCompatibilityEvaluation::unknown(
                    backend.id.clone(),
                    "no capability or resource requirements were supplied; \
                     compatibility has not been established",
                ),
            );
        }

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
                    "no target compatibility profile is available for \
                     this backend",
                ),
            );
        };

        binding.validate_for(backend)?;

        let report = self
            .evaluator
            .evaluate(&self.request, binding.profile());

        Ok(
            BackendCompatibilityEvaluation::from_report(
                backend.id.clone(),
                report,
            ),
        )
    }
}

impl BackendEligibilityEvaluator for BackendCompatibilityEvaluator<'_> {
    /// Evaluate one backend using the canonical compatibility engine.
    ///
    /// The returned rank is deliberately neutral (`0`). This evaluator
    /// establishes compatibility; it does not estimate latency, energy,
    /// throughput, cost, or performance.
    ///
    /// A separate optimization or cost-model evaluator may provide ranking
    /// after compatibility and feasibility have been established.
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
        CompatibilityRequest,
        DecimalInteger,
        InventoryCompleteness,
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
                        .expect("valid capability ID"),
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

    fn evaluator<'a>(
        provider: &'a dyn BackendCompatibilityProfileProvider,
        request: CompatibilityRequest,
    ) -> BackendCompatibilityEvaluator<'a> {
        BackendCompatibilityEvaluator::new(provider, request)
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

    #[test]
    fn binding_preserves_distinct_backend_and_target_ids() {
        let target_profile = profile("target.cpu.variant");

        let binding = BackendCompatibilityBinding::new(
            "backend.native",
            target_profile,
        )
        .expect("valid binding");

        assert_eq!(binding.backend_id(), "backend.native");
        assert_eq!(binding.target_id(), "target.cpu.variant");
    }

    #[test]
    fn binding_rejects_empty_backend_id() {
        let result = BackendCompatibilityBinding::new(
            "",
            profile("target.example"),
        );

        assert!(matches!(
            result,
            Err(
                BackendCompatibilityError::InvalidBackendIdentifier {
                    ..
                }
            )
        ));
    }

    #[test]
    fn binding_rejects_empty_target_id() {
        let result = TargetCapabilityProfile::new(" ")
            .and_then(|profile| {
                BackendCompatibilityBinding::new(
                    "backend.example",
                    profile,
                )
            });

        assert!(result.is_err());
    }

    #[test]
    fn binding_rejects_control_characters() {
        let result = BackendCompatibilityBinding::new(
            "backend\ninvalid",
            profile("target.example"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn mismatched_backend_binding_is_rejected() {
        let binding = BackendCompatibilityBinding::new(
            "backend.other",
            profile("target.example"),
        )
        .expect("valid binding");

        let requested = backend("backend.requested");

        let result = binding.validate_for(&requested);

        assert!(matches!(
            result,
            Err(
                BackendCompatibilityError::BackendIdentityMismatch {
                    ..
                }
            )
        ));
    }

    #[test]
    fn missing_profile_is_unknown() {
        let provider = TestProvider::default();

        let request = required_capability("cpu.integer");

        let evaluator = evaluator(&provider, request);

        let result = evaluator
            .evaluate_detailed(&backend("backend.example"))
            .expect("evaluation succeeds");

        assert!(matches!(
            result.eligibility(),
            BackendEligibility::Unknown { .. }
        ));

        assert!(result.report().is_none());
    }

    #[test]
    fn empty_requirements_are_unknown() {
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
            CompatibilityRequest::new(),
        );

        let result = evaluator
            .evaluate_detailed(&backend("backend.example"))
            .expect("evaluation succeeds");

        assert!(matches!(
            result.eligibility(),
            BackendEligibility::Unknown { .. }
        ));

        assert!(result.report().is_none());
    }

    #[test]
    fn supported_required_capability_is_compatible() {
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

        assert!(result.is_compatible());

        assert_eq!(
            result.target_id(),
            Some("target.example")
        );

        assert!(
            result.report().is_some()
        );
    }

    #[test]
    fn missing_capability_in_partial_inventory_is_unknown() {
        let mut provider = TestProvider::default();

        provider.insert(
            BackendCompatibilityBinding::new(
                "backend.example",
                profile("target.example"),
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

        assert!(matches!(
            result.eligibility(),
            BackendEligibility::Unknown { .. }
        ));
    }

    #[test]
    fn missing_capability_in_complete_inventory_is_incompatible() {
        let mut target_profile = profile("target.example");

        target_profile.mark_capability_inventory_complete();

        let mut provider = TestProvider::default();

        provider.insert(
            BackendCompatibilityBinding::new(
                "backend.example",
                target_profile,
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

        assert!(matches!(
            result.eligibility(),
            BackendEligibility::Incompatible { .. }
        ));
    }

    #[test]
    fn explicitly_unsupported_capability_is_incompatible() {
        let mut target_profile = profile("target.example");

        target_profile
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
                target_profile,
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

        assert!(matches!(
            result.eligibility(),
            BackendEligibility::Incompatible { .. }
        ));
    }

    #[test]
    fn partial_support_does_not_satisfy_hard_requirement() {
        let mut target_profile = profile("target.example");

        target_profile
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
                target_profile,
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

        assert!(matches!(
            result.eligibility(),
            BackendEligibility::Incompatible { .. }
        ));
    }

    #[test]
    fn experimental_support_requires_explicit_policy() {
        let mut target_profile = profile("target.example");

        target_profile
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
                target_profile,
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

        assert!(matches!(
            strict_result.eligibility(),
            BackendEligibility::Incompatible { .. }
        ));

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
    }

    #[test]
    fn insufficient_resource_is_incompatible() {
        let mut target_profile = profile("target.example");

        target_profile
            .declare_resource(
                ResourceId::new("memory.bytes")
                    .expect("valid resource"),
                ResourceAvailability::known(
                    DecimalInteger::new("1024")
                        .expect("valid quantity"),
                ),
            )
            .expect("resource declaration succeeds");

        let mut provider = TestProvider::default();

        provider.insert(
            BackendCompatibilityBinding::new(
                "backend.example",
                target_profile,
            )
            .expect("valid binding"),
        );

        let request = CompatibilityRequest::new().with_resource(
            ResourceRequirement::required(
                ResourceId::new("memory.bytes")
                    .expect("valid resource"),
                DecimalInteger::new("2048")
                    .expect("valid quantity"),
            ),
        );

        let evaluator = evaluator(&provider, request);

        let result = evaluator
            .evaluate_detailed(&backend("backend.example"))
            .expect("evaluation succeeds");

        assert!(matches!(
            result.eligibility(),
            BackendEligibility::Incompatible { .. }
        ));
    }

    #[test]
    fn unknown_resource_capacity_is_not_compatible() {
        let mut target_profile = profile("target.example");

        target_profile
            .declare_resource(
                ResourceId::new("memory.bytes")
                    .expect("valid resource"),
                ResourceAvailability::unknown(),
            )
            .expect("resource declaration succeeds");

        let mut provider = TestProvider::default();

        provider.insert(
            BackendCompatibilityBinding::new(
                "backend.example",
                target_profile,
            )
            .expect("valid binding"),
        );

        let request = CompatibilityRequest::new().with_resource(
            ResourceRequirement::required(
                ResourceId::new("memory.bytes")
                    .expect("valid resource"),
                DecimalInteger::new("2048")
                    .expect("valid quantity"),
            ),
        );

        let evaluator = evaluator(&provider, request);

        let result = evaluator
            .evaluate_detailed(&backend("backend.example"))
            .expect("evaluation succeeds");

        assert!(matches!(
            result.eligibility(),
            BackendEligibility::Unknown { .. }
        ));
    }

    #[test]
    fn provider_failure_is_not_silently_ignored() {
        let provider = |_backend: &BackendDescriptor| {
            Err("profile service unavailable".to_owned())
        };

        let evaluator = evaluator(
            &provider,
            required_capability("cpu.integer"),
        );

        let result = evaluator.evaluate(
            &backend("backend.example"),
        );

        assert!(result.is_err());

        assert!(
            result
                .expect_err("provider failure must propagate")
                .contains("profile service unavailable")
        );
    }

    #[test]
    fn closure_provider_is_supported() {
        let provider = |_backend: &BackendDescriptor| {
            Ok(None)
        };

        let evaluator = evaluator(
            &provider,
            required_capability("cpu.integer"),
        );

        let result = evaluator.evaluate(
            &backend("backend.example"),
        );

        assert!(matches!(
            result.expect("evaluation succeeds"),
            BackendEligibility::Unknown { .. }
        ));
    }

    #[test]
    fn compatibility_rank_is_neutral() {
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
            .evaluate(&backend("backend.example"))
            .expect("evaluation succeeds");

        assert!(matches!(
            result,
            BackendEligibility::Compatible {
                rank: 0,
                ..
            }
        ));
    }

    #[test]
    fn profile_binding_does_not_mark_inventory_complete() {
        let binding = BackendCompatibilityBinding::new(
            "backend.example",
            profile("target.example"),
        )
        .expect("valid binding");

        assert_eq!(
            binding.profile().inventory_completeness,
            InventoryCompleteness::Partial
        );
    }
}
