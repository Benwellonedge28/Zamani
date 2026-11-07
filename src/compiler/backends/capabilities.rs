
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Backend Capability Integration
//!
//! Path:
//! `src/compiler/backends/capabilities.rs`
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Integrates the compiler with the canonical backend capability system.
//!
//! This module is an adapter, not an independent capability authority.
//!
//! Canonical capability definitions:
//! `crate::backend::capabilities`
//!
//! Canonical backend registration:
//! `crate::backend::registry`
//!
//! Compiler backend selection:
//! `crate::compiler::backends::selection`
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Backend capability profile association.
//! - Capability profile provider contracts.
//! - Compiler-facing capability requests.
//! - Capability evaluation coordination.
//! - Capability result classification.
//! - Backend realization feasibility integration.
//!
//! It does not own:
//!
//! - Source-language semantic requirements.
//! - Canonical capability definitions.
//! - Backend registration.
//! - Target discovery.
//! - Physical resource allocation.
//! - Hardware inventory.
//! - Scheduling or routing.
//! - Code generation.
//! - Runtime execution.
//!
//! # Important distinction
//!
//! Capability compatibility is not complete realization feasibility.
//!
//! A backend satisfying every capability requirement does not establish
//! that the complete workload can be compiled or executed.
//!
//! A production selection evaluator must combine capability evidence
//! with the authoritative realization feasibility evaluator.
//!
//! # Scalability
//!
//! No universal finite limits are imposed on:
//!
//! - Backends.
//! - Capabilities.
//! - Computational domains.
//! - CPU architectures.
//! - GPU architectures.
//! - FPGA architectures.
//! - QPU architectures.
//! - Devices.
//! - Distributed participants.
//! - Memory.
//! - Quantum resources.
//! - Future computing technologies.
//!
//! Operational budgets and actual physical limits remain external
//! contracts.

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

/// Stable identity of this compiler integration module.
pub const BACKEND_CAPABILITIES_MODULE_NAME: &str =
    "zamani.compiler.backends.capabilities";

/// Version of the compiler-facing capability integration contract.
pub const BACKEND_CAPABILITIES_API_VERSION: u16 = 1;

// ============================================================================
// BACKEND CAPABILITY PROFILE
// ============================================================================

/// Associates a canonical capability profile with a registered backend.
///
/// The backend identifier must match the canonical identifier from
/// `BackendDescriptor`.
///
/// The profile itself remains owned by `crate::backend::capabilities`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BackendCapabilityProfile {
    backend_id: String,
    profile: CapabilityProfile,
}

impl BackendCapabilityProfile {
    /// Creates and validates a backend capability profile association.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid backend identifier or profile.
    pub fn new(
        backend_id: impl Into<String>,
        profile: CapabilityProfile,
    ) -> Result<Self, BackendCapabilityError> {
        let backend_id = backend_id.into();

        validate_backend_id(&backend_id)?;

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

    /// Returns the canonical backend identifier.
    #[must_use]
    pub fn backend_id(&self) -> &str {
        &self.backend_id
    }

    /// Returns the canonical capability profile.
    #[must_use]
    pub fn profile(&self) -> &CapabilityProfile {
        &self.profile
    }

    /// Validates the association against a registered backend.
    ///
    /// # Errors
    ///
    /// Returns an error when the identities differ or the profile is invalid.
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
// IDENTIFIER VALIDATION
// ============================================================================

fn validate_backend_id(
    backend_id: &str,
) -> Result<(), BackendCapabilityError> {
    if backend_id.trim().is_empty() {
        return Err(BackendCapabilityError::EmptyBackendId);
    }

    if backend_id != backend_id.trim()
        || backend_id.chars().any(char::is_control)
        || backend_id.chars().any(char::is_whitespace)
        || backend_id.contains('/')
        || backend_id.contains('\\')
    {
        return Err(BackendCapabilityError::InvalidBackendId {
            backend_id: backend_id.to_owned(),
            reason: "invalid canonical backend identifier".to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// PROFILE PROVIDER
// ============================================================================

/// Provides canonical backend capability profiles.
///
/// Implementations may adapt existing backend manifests, verified
/// implementation declarations, toolchain conformance reports,
/// or target capability inventories.
///
/// A provider must never infer capabilities from names, aliases,
/// file extensions, vendor strings, or descriptions.
///
/// Returning `None` means that the capability profile is unknown.
///
/// It does not mean that the backend supports everything.
///
/// Providers must not allocate hardware or execute programs.
pub trait BackendCapabilityProfileProvider: Send + Sync {
    /// Retrieves the profile associated with a registered backend.
    fn profile_for(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<Option<BackendCapabilityProfile>, String>;
}

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
// CAPABILITY REQUEST
// ============================================================================

/// Mandatory capability requirements and their evaluation policy.
///
/// All entries represent mandatory capability requirements.
///
/// Preferences, constraints, hints, and resource requirements must
/// remain distinguishable in their authoritative semantic models.
///
/// This type does not convert them into capability requirements.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BackendCapabilityRequest {
    requirements: Vec<CapabilityRequirement>,
    policy: CapabilityPolicy,
}

impl BackendCapabilityRequest {
    /// Creates a capability evaluation request.
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

    /// Returns mandatory capability requirements.
    #[must_use]
    pub fn requirements(&self) -> &[CapabilityRequirement] {
        &self.requirements
    }

    /// Returns the capability evidence policy.
    #[must_use]
    pub const fn policy(&self) -> CapabilityPolicy {
        self.policy
    }

    /// Returns the requirement count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.requirements.len()
    }

    /// Returns whether the request contains no requirements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by compiler backend capability integration.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum BackendCapabilityError {
    /// Backend identifier is empty.
    EmptyBackendId,

    /// Backend identifier is malformed.
    InvalidBackendId {
        /// Supplied backend identifier.
        backend_id: String,

        /// Validation explanation.
        reason: String,
    },

    /// Profile belongs to another backend.
    BackendIdentityMismatch {
        /// Expected backend identifier.
        expected: String,

        /// Actual backend identifier.
        actual: String,
    },

    /// Capability profile failed validation.
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
                    "backend identity mismatch: expected `{expected}`, \
                     received `{actual}`"
                )
            }

            Self::InvalidProfile {
                backend_id,
                reason,
            } => {
                write!(
                    formatter,
                    "invalid capability profile for `{backend_id}`: {reason}"
                )
            }
        }
    }
}

impl Error for BackendCapabilityError {}

// ============================================================================
// CAPABILITY EVALUATOR
// ============================================================================

/// Evaluates mandatory capability requirements against canonical profiles.
///
/// This evaluator establishes capability compatibility only.
///
/// It must be combined with realization feasibility before final
/// production backend selection.
///
/// It does not own a profile registry.
pub struct BackendCapabilityEvaluator<'a> {
    provider: &'a dyn BackendCapabilityProfileProvider,
    request: BackendCapabilityRequest,
}

impl<'a> BackendCapabilityEvaluator<'a> {
    /// Constructs an evaluator.
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

    /// Returns the immutable capability request.
    #[must_use]
    pub fn request(&self) -> &BackendCapabilityRequest {
        &self.request
    }

    /// Evaluates capability compatibility for one registered backend.
    ///
    /// Empty requirements produce Unknown because an empty capability
    /// request cannot establish overall compatibility.
    ///
    /// Missing profiles and missing evidence fail closed.
    ///
    /// # Errors
    ///
    /// Returns an error when the provider fails or supplies an invalid
    /// profile association.
    pub fn evaluate_capabilities(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<BackendEligibility, String> {
        if self.request.is_empty() {
            return Ok(BackendEligibility::Unknown {
                reason:
                    "no mandatory capability requirements were supplied"
                        .to_owned(),
            });
        }

        let association = self
            .provider
            .profile_for(backend)
            .map_err(|reason| {
                format!(
                    "capability provider failed for backend `{}`: {reason}",
                    backend.id
                )
            })?;

        let Some(association) = association else {
            return Ok(BackendEligibility::Unknown {
                reason: format!(
                    "capability profile unavailable for backend `{}`",
                    backend.id
                ),
            });
        };

        association
            .validate_for(backend)
            .map_err(|error| error.to_string())?;

        let result = association.profile().check(
            self.request.requirements(),
            self.request.policy(),
        );

        classify_result(&result)
    }
}

// ============================================================================
// REALIZATION FEASIBILITY
// ============================================================================

/// Evaluates non-capability conditions required for a backend realization.
///
/// The implementation must integrate the authoritative planning,
/// compatibility, resource, target, ABI, and toolchain contracts.
///
/// It must not allocate physical hardware.
///
/// Returning Compatible means the available evidence establishes
/// feasibility for the supplied workload and target context.
///
/// Returning Unknown means feasibility cannot yet be established.
///
/// This interface deliberately does not prescribe a second resource model.
pub trait BackendRealizationFeasibilityEvaluator: Send + Sync {
    /// Evaluates non-capability realization feasibility.
    fn evaluate_feasibility(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<BackendEligibility, String>;
}

// ============================================================================
// COMPLETE BACKEND ELIGIBILITY COMPOSITION
// ============================================================================

/// Combines capability compatibility and realization feasibility.
///
/// This is the evaluator intended for production backend selection.
///
/// A backend is Compatible only when both independent evaluations
/// establish compatibility.
///
/// An incompatible result cannot be overridden by an unknown result.
///
/// Unknown is never promoted to Compatible.
pub struct CompleteBackendEligibilityEvaluator<'a> {
    capabilities: &'a BackendCapabilityEvaluator<'a>,
    feasibility: &'a dyn BackendRealizationFeasibilityEvaluator,
}

impl<'a> CompleteBackendEligibilityEvaluator<'a> {
    /// Creates a complete backend eligibility evaluator.
    #[must_use]
    pub fn new(
        capabilities: &'a BackendCapabilityEvaluator<'a>,
        feasibility: &'a dyn BackendRealizationFeasibilityEvaluator,
    ) -> Self {
        Self {
            capabilities,
            feasibility,
        }
    }

    /// Evaluates both capability and realization feasibility.
    pub fn evaluate_complete(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<BackendEligibility, String> {
        let capability_result =
            self.capabilities.evaluate_capabilities(backend)?;

        let feasibility_result =
            self.feasibility.evaluate_feasibility(backend)?;

        combine_eligibility(
            capability_result,
            feasibility_result,
        )
    }
}

impl BackendEligibilityEvaluator
    for CompleteBackendEligibilityEvaluator<'_>
{
    /// Evaluates complete backend eligibility.
    fn evaluate(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<BackendEligibility, String> {
        self.evaluate_complete(backend)
    }
}

// ============================================================================
// RESULT CLASSIFICATION
// ============================================================================

/// Converts the canonical capability-check result into compiler eligibility.
///
/// This function does not reimplement capability matching.
fn classify_result(
    result: &CapabilityCheckResult,
) -> Result<BackendEligibility, String> {
    if result.is_empty() {
        return Ok(BackendEligibility::Unknown {
            reason: "capability checker returned no evaluated requirements"
                .to_owned(),
        });
    }

    if result.is_compatible() {
        return Ok(BackendEligibility::Compatible {
            rank: 0,
            explanation:
                "all mandatory capability requirements satisfy the active policy"
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

            // Canonical outcome types are non-exhaustive.
            // Unknown future outcomes must not become Compatible.
            _ => {
                unresolved.push(format!(
                    "{explanation}; explicit interpretation required"
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
        reason: "capability compatibility could not be established"
            .to_owned(),
    })
}

// ============================================================================
// ELIGIBILITY COMPOSITION
// ============================================================================

fn combine_eligibility(
    capabilities: BackendEligibility,
    feasibility: BackendEligibility,
) -> Result<BackendEligibility, String> {
    match (&capabilities, &feasibility) {
        (
            BackendEligibility::Incompatible { reason: left },
            BackendEligibility::Incompatible { reason: right },
        ) => {
            Ok(BackendEligibility::Incompatible {
                reason: format!("{left}; {right}"),
            })
        }

        (
            BackendEligibility::Incompatible { reason },
            _,
        )
        | (
            _,
            BackendEligibility::Incompatible { reason },
        ) => {
            Ok(BackendEligibility::Incompatible {
                reason: reason.clone(),
            })
        }

        (
            BackendEligibility::Unknown { reason: left },
            BackendEligibility::Unknown { reason: right },
        ) => {
            Ok(BackendEligibility::Unknown {
                reason: format!("{left}; {right}"),
            })
        }

        (
            BackendEligibility::Unknown { reason },
            _
        )
        | (
            _,
            BackendEligibility::Unknown { reason },
        ) => {
            Ok(BackendEligibility::Unknown {
                reason: reason.clone(),
            })
        }

        (
            BackendEligibility::Compatible {
                explanation: left,
                ..
            },
            BackendEligibility::Compatible {
                explanation: right,
                ..
            },
        ) => {
            Ok(BackendEligibility::Compatible {
                // This is a neutral compatibility rank.
                // Actual optimization ranking belongs to the cost model.
                rank: 0,
                explanation: format!("{left}; {right}"),
            })
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn backend() -> BackendDescriptor {
        BackendDescriptor::new("test.backend", ".test")
    }

    fn profile() -> CapabilityProfile {
        let mut profile = CapabilityProfile::new("test.profile");

        let descriptor = CapabilityDescriptor::from_name(
            "classical.integer",
            CapabilityStatus::Supported,
            "Integer computation",
        )
        .expect("valid capability");

        profile
            .capabilities_mut()
            .insert(descriptor)
            .expect("insertion succeeds");

        profile
    }

    fn request() -> BackendCapabilityRequest {
        BackendCapabilityRequest::new(
            vec![
                CapabilityRequirement::from_name(
                    "classical.integer",
                )
                .expect("valid requirement"),
            ],
            CapabilityPolicy::default(),
        )
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            BackendCapabilityEvaluator::<'static>::new
                as fn(
                    &'static dyn BackendCapabilityProfileProvider,
                    BackendCapabilityRequest,
                ) -> BackendCapabilityEvaluator<'static>,
                BackendCapabilityEvaluator::new,
        );

        assert_ne!(BACKEND_CAPABILITIES_API_VERSION, 0);
        assert!(!BACKEND_CAPABILITIES_MODULE_NAME.is_empty());
    }

    #[test]
    fn rejects_empty_backend_identity() {
        assert!(BackendCapabilityProfile::new(
            " ",
            profile(),
        )
        .is_err());
    }

    #[test]
    fn rejects_control_characters_in_backend_identity() {
        assert!(BackendCapabilityProfile::new(
            "backend\ninvalid",
            profile(),
        )
        .is_err());
    }

    #[test]
    fn rejects_backend_identity_mismatch() {
        let association = BackendCapabilityProfile::new(
            "another.backend",
            profile(),
        )
        .expect("association construction succeeds");

        assert!(matches!(
            association.validate_for(&backend()),
            Err(BackendCapabilityError::BackendIdentityMismatch { .. })
        ));
    }

    #[test]
    fn accepts_matching_backend_identity() {
        let association = BackendCapabilityProfile::new(
            "test.backend",
            profile(),
        )
        .expect("association construction succeeds");

        assert!(association.validate_for(&backend()).is_ok());
    }

    #[test]
    fn missing_profile_is_unknown() {
        let provider = |_backend: &BackendDescriptor| {
            Ok::<_, String>(None)
        };

        let evaluator = BackendCapabilityEvaluator::new(
            &provider,
            request(),
        );

        assert!(matches!(
            evaluator.evaluate_capabilities(&backend()),
            Ok(BackendEligibility::Unknown { .. })
        ));
    }

    #[test]
    fn matching_capability_is_compatible_at_capability_level() {
        let provider = |_backend: &BackendDescriptor| {
            Ok::<_, String>(Some(
                BackendCapabilityProfile::new(
                    "test.backend",
                    profile(),
                )
                .expect("valid association"),
            ))
        };

        let evaluator = BackendCapabilityEvaluator::new(
            &provider,
            request(),
        );

        assert!(matches!(
            evaluator.evaluate_capabilities(&backend()),
            Ok(BackendEligibility::Compatible { .. })
        ));
    }

    #[test]
    fn empty_requirement_is_unknown() {
        let provider = |_backend: &BackendDescriptor| {
            Ok::<_, String>(Some(
                BackendCapabilityProfile::new(
                    "test.backend",
                    profile(),
                )
                .expect("valid association"),
            ))
        };

        let evaluator = BackendCapabilityEvaluator::new(
            &provider,
            BackendCapabilityRequest::new(
                Vec::new(),
                CapabilityPolicy::default(),
            ),
        );

        assert!(matches!(
            evaluator.evaluate_capabilities(&backend()),
            Ok(BackendEligibility::Unknown { .. })
        ));
    }

    #[test]
    fn provider_failure_is_propagated() {
        let provider = |_backend: &BackendDescriptor| {
            Err::<Option<BackendCapabilityProfile>, _>(
                "inventory unavailable",
            )
        };

        let evaluator = BackendCapabilityEvaluator::new(
            &provider,
            request(),
        );

        assert!(
            evaluator
                .evaluate_capabilities(&backend())
                .is_err()
        );
    }

    #[test]
    fn missing_required_capability_is_unknown() {
        let provider = |_backend: &BackendDescriptor| {
            Ok::<_, String>(Some(
                BackendCapabilityProfile::new(
                    "test.backend",
                    CapabilityProfile::new("empty"),
                )
                .expect("valid association"),
            ))
        };

        let evaluator = BackendCapabilityEvaluator::new(
            &provider,
            request(),
        );

        assert!(matches!(
            evaluator.evaluate_capabilities(&backend()),
            Ok(BackendEligibility::Unknown { .. })
        ));
    }

    #[test]
    fn explicit_unavailability_is_incompatible() {
        let mut unavailable = CapabilityProfile::new("unavailable");

        unavailable
            .capabilities_mut()
            .insert(
                CapabilityDescriptor::from_name(
                    "classical.integer",
                    CapabilityStatus::Unavailable,
                    "Unavailable",
                )
                .expect("valid descriptor"),
            )
            .expect("insertion succeeds");

        let provider = move |_backend: &BackendDescriptor| {
            Ok::<_, String>(Some(
                BackendCapabilityProfile::new(
                    "test.backend",
                    unavailable.clone(),
                )
                .expect("valid association"),
            ))
        };

        let evaluator = BackendCapabilityEvaluator::new(
            &provider,
            request(),
        );

        assert!(matches!(
            evaluator.evaluate_capabilities(&backend()),
            Ok(BackendEligibility::Incompatible { .. })
        ));
    }

    struct Feasible;

    impl BackendRealizationFeasibilityEvaluator for Feasible {
        fn evaluate_feasibility(
            &self,
            _backend: &BackendDescriptor,
        ) -> Result<BackendEligibility, String> {
            Ok(BackendEligibility::Compatible {
                rank: 0,
                explanation: "realization is feasible".to_owned(),
            })
        }
    }

    struct UnknownFeasibility;

    impl BackendRealizationFeasibilityEvaluator for UnknownFeasibility {
        fn evaluate_feasibility(
            &self,
            _backend: &BackendDescriptor,
        ) -> Result<BackendEligibility, String> {
            Ok(BackendEligibility::Unknown {
                reason: "resource availability is unknown".to_owned(),
            })
        }
    }

    #[test]
    fn complete_evaluator_requires_feasibility() {
        let provider = |_backend: &BackendDescriptor| {
            Ok::<_, String>(Some(
                BackendCapabilityProfile::new(
                    "test.backend",
                    profile(),
                )
                .expect("valid association"),
            ))
        };

        let capabilities = BackendCapabilityEvaluator::new(
            &provider,
            request(),
        );

        let feasibility = UnknownFeasibility;

        let complete = CompleteBackendEligibilityEvaluator::new(
            &capabilities,
            &feasibility,
        );

        assert!(matches!(
            complete.evaluate_complete(&backend()),
            Ok(BackendEligibility::Unknown { .. })
        ));
    }

    #[test]
    fn complete_evaluator_accepts_both_positive_results() {
        let provider = |_backend: &BackendDescriptor| {
            Ok::<_, String>(Some(
                BackendCapabilityProfile::new(
                    "test.backend",
                    profile(),
                )
                .expect("valid association"),
            ))
        };

        let capabilities = BackendCapabilityEvaluator::new(
            &provider,
            request(),
        );

        let feasibility = Feasible;

        let complete = CompleteBackendEligibilityEvaluator::new(
            &capabilities,
            &feasibility,
        );

        assert!(matches!(
            complete.evaluate_complete(&backend()),
            Ok(BackendEligibility::Compatible { .. })
        ));
    }

    #[test]
    fn incompatible_result_cannot_be_overridden() {
        let result = combine_eligibility(
            BackendEligibility::Incompatible {
                reason: "unsupported capability".to_owned(),
            },
            BackendEligibility::Compatible {
                rank: 0,
                explanation: "resources available".to_owned(),
            },
        )
        .expect("composition succeeds");

        assert!(matches!(
            result,
            BackendEligibility::Incompatible { .. }
        ));
    }
}
