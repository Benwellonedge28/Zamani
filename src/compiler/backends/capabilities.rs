
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Backend Capability Integration
//!
//! Path: `src/compiler/backends/capabilities.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97+
//! Safety: Safe Rust only. Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Integrate the canonical backend capability model with the compiler's
//! backend-selection interface.
//!
//! This module does not introduce another capability model, capability
//! registry, target registry, compatibility engine, or resource planner.
//!
//! # Canonical authorities
//!
//! - `crate::backend::capabilities`:
//!   Capability identifiers, declarations, profiles, evidence and matching.
//!
//! - `crate::backend::registry`:
//!   Registered backend identities and implementations.
//!
//! - `crate::compiler::backends::selection`:
//!   Backend eligibility and deterministic selection.
//!
//! - `crate::compiler::analysis::capabilities`:
//!   Compiler-facing semantic capability analysis.
//!
//! - `crate::compiler::planning::capability_matching`:
//!   Target-scoped semantic capability matching.
//!
//! - `crate::compiler::planning::resource_planning`:
//!   Resource feasibility.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Backend-to-capability-profile association.
//! - The backend capability profile provider interface.
//! - Compiler-facing capability evaluation.
//! - Conversion of canonical capability outcomes into backend eligibility.
//!
//! It does not own:
//!
//! - Source-language capability semantics.
//! - Canonical backend capability definitions.
//! - Capability evidence authentication.
//! - Backend registration.
//! - Target discovery.
//! - Resource feasibility.
//! - Hardware allocation.
//! - Scheduling or routing.
//! - Code generation.
//! - Runtime execution.
//!
//! # Conservative compatibility rules
//!
//! - Empty requirements do not prove compatibility.
//! - Missing profiles do not prove compatibility.
//! - Missing capability declarations are unresolved, not proof of support.
//! - Explicitly unavailable capabilities are incompatible.
//! - Disallowed experimental or deprecated capabilities are incompatible
//!   under the active policy.
//! - Insufficient evidence remains unresolved.
//! - Provider failures are reported as errors.
//! - A profile associated with a different backend is rejected.
//!
//! # Scalability
//!
//! No fixed backend, capability, architecture, accelerator, device,
//! processor, node, memory, or resource-count ceiling is introduced.
//!
//! Collections grow according to available resources.
//!
//! Operational budgets belong to compilation orchestration.
//!
//! Literal infinity is not promised.
//!
//! # Integration
//!
//! The resulting `BackendCapabilityEvaluator` implements the existing
//! `BackendEligibilityEvaluator` interface from `selection.rs`.
//!
//! Its output can be passed directly to `BackendSelector::select`.
//!
//! The profile provider is deliberately external. The canonical backend
//! registry does not currently own capability profiles, and this module
//! must not create a second mutable registry to compensate for that fact.

use std::error::Error;
use std::fmt;

pub use crate::backend::capabilities::{
    CapabilityCheckResult,
    CapabilityDescriptor,
    CapabilityEvidence,
    CapabilityEvidenceKind,
    CapabilityId,
    CapabilityOutcome,
    CapabilityPolicy,
    CapabilityProfile,
    CapabilityRequirement,
    CapabilityRequirementResult,
    CapabilitySet,
    CapabilityStatus,
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
pub const BACKEND_CAPABILITIES_MODULE_NAME: &str =
    "zamani.compiler.backends.capabilities";

/// Version of this compiler-facing capability integration contract.
///
/// This is independent of the language, backend, target, and artifact
/// schema versions.
pub const BACKEND_CAPABILITIES_API_VERSION: u16 = 1;

// ============================================================================
// BACKEND PROFILE ASSOCIATION
// ============================================================================

/// A canonical backend capability profile associated with one registered
/// backend identifier.
///
/// `backend_id` is the canonical identifier from `BackendDescriptor`.
///
/// The profile name is descriptive metadata. It is not used to infer
/// backend identity.
///
/// This association prevents a provider from accidentally returning the
/// capability profile of a different backend.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BackendCapabilityProfile {
    backend_id: String,
    profile: CapabilityProfile,
}

impl BackendCapabilityProfile {
    /// Construct and validate a backend capability profile association.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The backend identifier is empty.
    /// - The backend identifier contains control characters.
    /// - The canonical capability profile is invalid.
    pub fn new(
        backend_id: impl Into<String>,
        profile: CapabilityProfile,
    ) -> Result<Self, BackendCapabilityError> {
        let backend_id = backend_id.into();

        if backend_id.trim().is_empty() {
            return Err(BackendCapabilityError::EmptyBackendId);
        }

        if backend_id.chars().any(char::is_control) {
            return Err(BackendCapabilityError::InvalidBackendId {
                backend_id,
                reason: "control characters are forbidden".to_owned(),
            });
        }

        profile.validate().map_err(|error| {
            BackendCapabilityError::InvalidProfile {
                backend_id: backend_id.clone(),
                reason: error.to_string(),
            }
        })?;

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

    /// Return the canonical backend capability profile.
    #[must_use]
    pub fn profile(&self) -> &CapabilityProfile {
        &self.profile
    }

    /// Validate the profile and its association with a backend descriptor.
    ///
    /// This must be called when accepting data from an external provider,
    /// deserialized configuration, or another untrusted source.
    ///
    /// # Errors
    ///
    /// Returns an error if the profile belongs to another backend or if
    /// the canonical profile validation fails.
    pub fn validate_for(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<(), BackendCapabilityError> {
        if self.backend_id != backend.id {
            return Err(
                BackendCapabilityError::BackendIdentityMismatch {
                    expected: backend.id.clone(),
                    actual: self.backend_id.clone(),
                },
            );
        }

        self.profile.validate().map_err(|error| {
            BackendCapabilityError::InvalidProfile {
                backend_id: self.backend_id.clone(),
                reason: error.to_string(),
            }
        })
    }
}

// ============================================================================
// PROFILE PROVIDER
// ============================================================================

/// Supplies the canonical capability profile associated with a registered
/// backend.
///
/// Implementations may adapt:
///
/// - Explicit backend configuration.
/// - A validated backend manifest.
/// - A verified backend implementation declaration.
/// - A target-provider inventory.
/// - A toolchain conformance report.
/// - A hardware capability inventory.
///
/// The provider must use an explicit canonical backend-ID mapping.
///
/// It must never infer capability support from:
///
/// - Backend names.
/// - Aliases.
/// - Vendor strings.
/// - File extensions.
/// - Architecture names.
/// - Display descriptions.
///
/// Returning `None` means that no usable profile is available. It does not
/// mean that the backend supports every capability or supports none.
///
/// The provider must not allocate physical hardware, reserve resources,
/// execute a program, or invoke a compiler backend.
pub trait BackendCapabilityProfileProvider: Send + Sync {
    /// Retrieve the capability profile for one registered backend.
    ///
    /// `Ok(None)` means that capability evidence is unavailable.
    ///
    /// `Err` means that profile retrieval failed.
    ///
    /// The returned profile must identify the same canonical backend as
    /// the descriptor supplied to this method.
    fn profile_for(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<Option<BackendCapabilityProfile>, String>;
}

/// Allow a thread-safe closure to act as a capability profile provider.
///
/// This makes it possible to integrate existing inventories and manifests
/// without introducing another registry implementation.
impl<F> BackendCapabilityProfileProvider for F
where
    F: Fn(
            &BackendDescriptor,
        ) -> Result<Option<BackendCapabilityProfile>, String>
        + Send
        + Sync,
{
    fn profile_for(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<Option<BackendCapabilityProfile>, String> {
        self(backend)
    }
}

// ============================================================================
// EVALUATION REQUEST
// ============================================================================

/// Backend capability requirements and the policy used to evaluate them.
///
/// The contained requirement type belongs to the canonical backend
/// capability model.
///
/// Requirements retain their original order, including duplicates.
/// This module does not silently remove or rewrite requirements.
///
/// An empty requirement collection is allowed as an input representation,
/// but it cannot establish backend compatibility.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BackendCapabilityRequest {
    requirements: Vec<CapabilityRequirement>,
    policy: CapabilityPolicy,
}

impl BackendCapabilityRequest {
    /// Create a backend capability request.
    #[must_use]
    pub fn new(
        requirements: Vec<CapabilityRequirement>,
        policy: CapabilityPolicy,
    ) -> Self {
        Self {
            requirements,
            policy,
        }
    }

    /// Return the required backend capabilities.
    #[must_use]
    pub fn requirements(&self) -> &[CapabilityRequirement] {
        &self.requirements
    }

    /// Return the active capability policy.
    #[must_use]
    pub const fn policy(&self) -> CapabilityPolicy {
        self.policy
    }

    /// Return the number of requirements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.requirements.len()
    }

    /// Whether the request contains no capability requirements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors associated with compiler-facing backend capability contracts.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum BackendCapabilityError {
    /// The associated backend identifier is empty.
    EmptyBackendId,

    /// The associated backend identifier is malformed.
    InvalidBackendId {
        /// Supplied identifier.
        backend_id: String,

        /// Validation explanation.
        reason: String,
    },

    /// The supplied profile does not belong to the requested backend.
    BackendIdentityMismatch {
        /// Expected canonical backend identifier.
        expected: String,

        /// Actual profile backend identifier.
        actual: String,
    },

    /// The canonical capability profile is invalid.
    InvalidProfile {
        /// Associated backend identifier.
        backend_id: String,

        /// Validation explanation.
        reason: String,
    },
}

impl fmt::Display for BackendCapabilityError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyBackendId => {
                formatter.write_str(
                    "backend capability profile requires a backend identifier",
                )
            }

            Self::InvalidBackendId {
                backend_id,
                reason,
            } => {
                write!(
                    formatter,
                    "invalid backend identifier `{backend_id}`: {reason}"
                )
            }

            Self::BackendIdentityMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "backend capability profile identity mismatch: \
                     expected `{expected}`, received `{actual}`"
                )
            }

            Self::InvalidProfile {
                backend_id,
                reason,
            } => {
                write!(
                    formatter,
                    "invalid capability profile for backend \
                     `{backend_id}`: {reason}"
                )
            }
        }
    }
}

impl Error for BackendCapabilityError {}

// ============================================================================
// EVALUATOR
// ============================================================================

/// Evaluates backend capability profiles using the canonical backend
/// capability matching implementation.
///
/// This adapter is intended to be supplied directly to
/// `BackendSelector::select`.
///
/// It does not maintain a profile registry or modify the backend registry.
pub struct BackendCapabilityEvaluator<'a> {
    provider: &'a dyn BackendCapabilityProfileProvider,
    request: BackendCapabilityRequest,
}

impl<'a> BackendCapabilityEvaluator<'a> {
    /// Construct an evaluator using an existing profile provider and
    /// capability request.
    #[must_use]
    pub fn new(
        provider: &'a dyn BackendCapabilityProfileProvider,
        request: BackendCapabilityRequest,
    ) -> Self {
        Self {
            provider,
            request,
        }
    }

    /// Return the evaluator's immutable capability request.
    #[must_use]
    pub fn request(&self) -> &BackendCapabilityRequest {
        &self.request
    }

    /// Evaluate one backend using its canonical capability profile.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The provider fails.
    /// - The provider returns a profile associated with another backend.
    /// - The profile fails validation.
    ///
    /// Missing profiles and missing capability evidence are represented
    /// as `Unknown`, not as errors.
    pub fn evaluate_backend(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<BackendEligibility, String> {
        if self.request.is_empty() {
            return Ok(BackendEligibility::Unknown {
                reason:
                    "no capability requirements were supplied; \
                     compatibility cannot be established"
                        .to_owned(),
            });
        }

        let Some(association) = self
            .provider
            .profile_for(backend)
            .map_err(|reason| {
                format!(
                    "capability profile provider failed for backend \
                     `{}`: {reason}",
                    backend.id
                )
            })?
        else {
            return Ok(BackendEligibility::Unknown {
                reason: format!(
                    "no capability profile is available for backend `{}`",
                    backend.id
                ),
            });
        };

        association
            .validate_for(backend)
            .map_err(|error| error.to_string())?;

        let profile = association.profile();

        let result = profile.check(
            self.request.requirements(),
            self.request.policy(),
        );

        classify_result(&result)
    }
}

impl BackendEligibilityEvaluator for BackendCapabilityEvaluator<'_> {
    /// Evaluate the backend using the canonical capability profile.
    fn evaluate(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<BackendEligibility, String> {
        self.evaluate_backend(backend)
    }
}

// ============================================================================
// RESULT CLASSIFICATION
// ============================================================================

/// Convert the canonical backend capability-check result into the existing
/// compiler backend eligibility contract.
///
/// This function does not reimplement capability matching. It only
/// classifies the outcomes already produced by `CapabilityProfile::check`.
fn classify_result(
    result: &CapabilityCheckResult,
) -> Result<BackendEligibility, String> {
    if result.is_empty() {
        return Ok(BackendEligibility::Unknown {
            reason:
                "the capability checker returned no evaluated requirements"
                    .to_owned(),
        });
    }

    if result.is_compatible() {
        return Ok(BackendEligibility::Compatible {
            // This adapter evaluates compatibility only. It does not
            // claim to estimate execution performance or cost.
            //
            // A separate optimization or cost-model evaluator may provide
            // a meaningful rank when that information is available.
            rank: 0,
            explanation:
                "all required backend capabilities satisfy the active policy"
                    .to_owned(),
        });
    }

    let mut incompatible = Vec::new();
    let mut unresolved = Vec::new();

    for failure in result.failures() {
        let explanation = format!(
            "capability `{}`: {:?}",
            failure.id(),
            failure.outcome()
        );

        match failure.outcome() {
            CapabilityOutcome::Unavailable
            | CapabilityOutcome::DisallowedStatus(_) => {
                incompatible.push(explanation);
            }

            CapabilityOutcome::Missing
            | CapabilityOutcome::InsufficientEvidence => {
                unresolved.push(explanation);
            }

            // CapabilityOutcome is non-exhaustive. New canonical outcomes
            // must fail closed until their meaning is explicitly mapped.
            _ => {
                unresolved.push(format!(
                    "{explanation}; outcome requires explicit evaluation"
                ));
            }
        }
    }

    if !incompatible.is_empty() {
        return Ok(BackendEligibility::Incompatible {
            reason: incompatible.join("; "),
        });
    }

    if !unresolved.is_empty() {
        return Ok(BackendEligibility::Unknown {
            reason: unresolved.join("; "),
        });
    }

    Ok(BackendEligibility::Unknown {
        reason:
            "capability compatibility could not be established"
                .to_owned(),
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct FixedProvider {
        profile: Option<BackendCapabilityProfile>,
    }

    impl BackendCapabilityProfileProvider for FixedProvider {
        fn profile_for(
            &self,
            _backend: &BackendDescriptor,
        ) -> Result<Option<BackendCapabilityProfile>, String> {
            Ok(self.profile.clone())
        }
    }

    struct FailingProvider;

    impl BackendCapabilityProfileProvider for FailingProvider {
        fn profile_for(
            &self,
            _backend: &BackendDescriptor,
        ) -> Result<Option<BackendCapabilityProfile>, String> {
            Err("test provider failure".to_owned())
        }
    }

    fn backend() -> BackendDescriptor {
        BackendDescriptor::new("backend.example", ".bin")
    }

    fn requirement() -> CapabilityRequirement {
        CapabilityRequirement::from_name("example.compute")
            .expect("valid capability requirement")
    }

    fn profile(
        status: CapabilityStatus,
        direct_evidence: bool,
    ) -> CapabilityProfile {
        let mut profile = CapabilityProfile::new("example-profile");

        let mut descriptor = CapabilityDescriptor::from_name(
            "example.compute",
            status,
            "Example computation capability",
        )
        .expect("valid capability descriptor");

        if direct_evidence {
            descriptor = descriptor.with_evidence(
                CapabilityEvidence::new(
                    CapabilityEvidenceKind::Verified,
                    "compiler-capability-test",
                    Some("conformance test passed".to_owned()),
                ),
            );
        }

        profile
            .capabilities_mut()
            .insert(descriptor)
            .expect("capability insertion succeeds");

        profile
    }

    fn request(
        policy: CapabilityPolicy,
    ) -> BackendCapabilityRequest {
        BackendCapabilityRequest::new(
            vec![requirement()],
            policy,
        )
    }

    fn association(
        profile: CapabilityProfile,
    ) -> BackendCapabilityProfile {
        BackendCapabilityProfile::new(
            "backend.example",
            profile,
        )
        .expect("valid backend profile association")
    }

    #[test]
    fn profile_association_preserves_backend_identity() {
        let association = association(profile(
            CapabilityStatus::Supported,
            false,
        ));

        assert_eq!(
            association.backend_id(),
            "backend.example"
        );
    }

    #[test]
    fn empty_backend_id_is_rejected() {
        let result = BackendCapabilityProfile::new(
            "",
            CapabilityProfile::new("profile"),
        );

        assert!(matches!(
            result,
            Err(BackendCapabilityError::EmptyBackendId)
        ));
    }

    #[test]
    fn profile_identity_mismatch_is_rejected() {
        let association = BackendCapabilityProfile::new(
            "backend.other",
            CapabilityProfile::new("profile"),
        )
        .expect("valid association");

        let result = association.validate_for(&backend());

        assert!(matches!(
            result,
            Err(
                BackendCapabilityError::BackendIdentityMismatch {
                    ..
                }
            )
        ));
    }

    #[test]
    fn supported_capability_is_eligible() {
        let provider = FixedProvider {
            profile: Some(association(profile(
                CapabilityStatus::Supported,
                false,
            ))),
        };

        let evaluator = BackendCapabilityEvaluator::new(
            &provider,
            request(CapabilityPolicy::default()),
        );

        let result = evaluator
            .evaluate_backend(&backend())
            .expect("evaluation succeeds");

        assert!(matches!(
            result,
            BackendEligibility::Compatible { .. }
        ));
    }

    #[test]
    fn unavailable_capability_is_incompatible() {
        let provider = FixedProvider {
            profile: Some(association(profile(
                CapabilityStatus::Unavailable,
                false,
            ))),
        };

        let evaluator = BackendCapabilityEvaluator::new(
            &provider,
            request(CapabilityPolicy::default()),
        );

        let result = evaluator
            .evaluate_backend(&backend())
            .expect("evaluation succeeds");

        assert!(matches!(
            result,
            BackendEligibility::Incompatible { .. }
        ));
    }

    #[test]
    fn missing_profile_is_unknown() {
        let provider = FixedProvider {
            profile: None,
        };

        let evaluator = BackendCapabilityEvaluator::new(
            &provider,
            request(CapabilityPolicy::default()),
        );

        let result = evaluator
            .evaluate_backend(&backend())
            .expect("evaluation succeeds");

        assert!(matches!(
            result,
            BackendEligibility::Unknown { .. }
        ));
    }

    #[test]
    fn missing_capability_is_unknown() {
        let provider = FixedProvider {
            profile: Some(association(
                CapabilityProfile::new("empty-profile"),
            )),
        };

        let evaluator = BackendCapabilityEvaluator::new(
            &provider,
            request(CapabilityPolicy::default()),
        );

        let result = evaluator
            .evaluate_backend(&backend())
            .expect("evaluation succeeds");

        assert!(matches!(
            result,
            BackendEligibility::Unknown { .. }
        ));
    }

    #[test]
    fn empty_requirements_do_not_prove_compatibility() {
        let provider = FixedProvider {
            profile: Some(association(profile(
                CapabilityStatus::Supported,
                false,
            ))),
        };

        let evaluator = BackendCapabilityEvaluator::new(
            &provider,
            BackendCapabilityRequest::new(
                Vec::new(),
                CapabilityPolicy::default(),
            ),
        );

        let result = evaluator
            .evaluate_backend(&backend())
            .expect("evaluation succeeds");

        assert!(matches!(
            result,
            BackendEligibility::Unknown { .. }
        ));
    }

    #[test]
    fn direct_evidence_policy_is_respected() {
        let provider = FixedProvider {
            profile: Some(association(profile(
                CapabilityStatus::Supported,
                false,
            ))),
        };

        let policy = CapabilityPolicy {
            allow_experimental: false,
            allow_deprecated: false,
            require_direct_evidence: true,
        };

        let evaluator = BackendCapabilityEvaluator::new(
            &provider,
            request(policy),
        );

        let result = evaluator
            .evaluate_backend(&backend())
            .expect("evaluation succeeds");

        assert!(matches!(
            result,
            BackendEligibility::Unknown { .. }
        ));
    }

    #[test]
    fn direct_verified_evidence_satisfies_strict_policy() {
        let provider = FixedProvider {
            profile: Some(association(profile(
                CapabilityStatus::Supported,
                true,
            ))),
        };

        let policy = CapabilityPolicy {
            allow_experimental: false,
            allow_deprecated: false,
            require_direct_evidence: true,
        };

        let evaluator = BackendCapabilityEvaluator::new(
            &provider,
            request(policy),
        );

        let result = evaluator
            .evaluate_backend(&backend())
            .expect("evaluation succeeds");

        assert!(matches!(
            result,
            BackendEligibility::Compatible { .. }
        ));
    }

    #[test]
    fn experimental_capability_is_rejected_by_default() {
        let provider = FixedProvider {
            profile: Some(association(profile(
                CapabilityStatus::Experimental,
                false,
            ))),
        };

        let evaluator = BackendCapabilityEvaluator::new(
            &provider,
            request(CapabilityPolicy::default()),
        );

        let result = evaluator
            .evaluate_backend(&backend())
            .expect("evaluation succeeds");

        assert!(matches!(
            result,
            BackendEligibility::Incompatible { .. }
        ));
    }

    #[test]
    fn experimental_capability_requires_explicit_permission() {
        let provider = FixedProvider {
            profile: Some(association(profile(
                CapabilityStatus::Experimental,
                false,
            ))),
        };

        let policy = CapabilityPolicy {
            allow_experimental: true,
            allow_deprecated: false,
            require_direct_evidence: false,
        };

        let evaluator = BackendCapabilityEvaluator::new(
            &provider,
            request(policy),
        );

        let result = evaluator
            .evaluate_backend(&backend())
            .expect("evaluation succeeds");

        assert!(matches!(
            result,
            BackendEligibility::Compatible { .. }
        ));
    }

    #[test]
    fn provider_failure_is_propagated() {
        let evaluator = BackendCapabilityEvaluator::new(
            &FailingProvider,
            request(CapabilityPolicy::default()),
        );

        let result = evaluator.evaluate_backend(&backend());

        assert!(result.is_err());

        assert!(
            result
                .expect_err("provider must fail")
                .contains("test provider failure")
        );
    }

    #[test]
    fn evaluator_implements_existing_selection_interface() {
        fn accepts_evaluator(
            _: &dyn BackendEligibilityEvaluator,
        ) {
        }

        let provider = FixedProvider {
            profile: Some(association(profile(
                CapabilityStatus::Supported,
                false,
            ))),
        };

        let evaluator = BackendCapabilityEvaluator::new(
            &provider,
            request(CapabilityPolicy::default()),
        );

        accepts_evaluator(&evaluator);
    }

    #[test]
    fn request_preserves_requirement_order_and_duplicates() {
        let first = requirement();
        let second = requirement();

        let request = BackendCapabilityRequest::new(
            vec![first, second],
            CapabilityPolicy::default(),
        );

        assert_eq!(request.len(), 2);
    }

    #[test]
    fn evaluator_does_not_infer_support_from_backend_name() {
        let provider = FixedProvider {
            profile: None,
        };

        let evaluator = BackendCapabilityEvaluator::new(
            &provider,
            request(CapabilityPolicy::default()),
        );

        let result = evaluator
            .evaluate_backend(&backend())
            .expect("evaluation succeeds");

        assert!(matches!(
            result,
            BackendEligibility::Unknown { .. }
        ));
    }
}
