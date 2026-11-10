
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler Extension Validation
//!
//! Path: `src/compiler/extensions/validation.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Coordinate structural manifest validation and provider compatibility
//! evaluation for compiler extensions.
//!
//! This module establishes whether an extension's declarations satisfy
//! the host's structural and compatibility requirements.
//!
//! A successful result is not authorization to execute extension code.
//!
//! ## Canonical ownership
//!
//! Provider identity and descriptors:
//!     crate::compiler::extensions::provider
//!
//! Manifest parsing and structural validation:
//!     crate::compiler::extensions::manifest
//!
//! Provider compatibility:
//!     crate::compiler::extensions::compatibility
//!
//! Extension discovery:
//!     crate::compiler::extensions::discovery
//!
//! Provider registration:
//!     crate::compiler::extensions::registry
//!
//! Supply-chain verification:
//!     crate::compiler::security::supply_chain
//!
//! Source-level semantic extensions:
//!     crate::frontend::semantic::domains::extensions
//!
//! ## Owns
//!
//! - Compiler extension validation requests.
//! - Coordination of manifest validation.
//! - Coordination of provider compatibility evaluation.
//! - Structured extension validation reports.
//! - Explicit validation outcomes.
//! - Validation error propagation.
//! - Preservation of canonical provider descriptors.
//!
//! ## Does not own
//!
//! - Provider identity or version definitions.
//! - Manifest parsing or serialization.
//! - Manifest structural validation rules.
//! - Provider discovery.
//! - Provider registration.
//! - Provider activation.
//! - Provider lifecycle.
//! - API compatibility algorithms.
//! - Dependency resolution.
//! - Publisher authentication.
//! - Cryptographic verification.
//! - Supply-chain trust.
//! - Authorization.
//! - Sandboxing or process isolation.
//! - Backend registration.
//! - Pass registration.
//! - Target discovery.
//! - Hardware discovery.
//! - Resource allocation.
//! - Scheduling.
//! - Runtime execution.
//! - Source grammar or semantic analysis.
//!
//! ## Validation semantics
//!
//! The outcomes are:
//!
//! - Compatible: structural validation succeeded and all requested
//!   compatibility requirements were established.
//! - Incompatible: structural validation succeeded but at least one
//!   explicit compatibility requirement was violated.
//! - Indeterminate: structural validation succeeded but compatibility
//!   could not be established from the supplied evidence.
//!
//! Structural validation errors are returned as errors.
//!
//! Indeterminate is never treated as Compatible.
//!
//! ## Security boundary
//!
//! A validation report does not establish:
//!
//! - Publisher identity.
//! - Signature validity.
//! - Artifact integrity.
//! - Provider authorization.
//! - Provider implementation correctness.
//! - Isolation enforcement.
//! - Runtime safety.
//!
//! Supply-chain verification and security admission must independently
//! succeed before executable provider code is activated.
//!
//! ## Scalability
//!
//! This module imposes no universal limits on:
//!
//! - Providers.
//! - Dependencies.
//! - Contributions.
//! - Extension points.
//! - Computing domains.
//! - Computing technologies.
//! - Hardware targets.
//!
//! Manifest processing budgets are supplied by the caller and remain
//! operational safeguards rather than semantic capacity limits.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## Determinism
//!
//! Compatibility findings are supplied by the canonical compatibility
//! evaluator, which orders its findings deterministically.
//!
//! This module does not make provider registration order significant.
//!
//! ## Integration
//!
//! 1. Discovery obtains a candidate manifest.
//! 2. Manifest validation checks its structure and declarations.
//! 3. The manifest is converted into a canonical ProviderDescriptor.
//! 4. Compatibility evaluates the descriptor against the host API,
//!    provider inventory and requested contribution contracts.
//! 5. The caller receives a structured validation report.
//! 6. Supply-chain and security admission are evaluated independently.
//! 7. Only the authoritative extension registry and lifecycle manager
//!    may proceed with provider registration and activation.
//!
//! The frontend semantic extension validator remains responsible for
//! source-language semantic extension declarations. This module does
//! not replace it.

use std::error::Error;
use std::fmt;

use super::compatibility::{
    ExtensionCompatibilityEvaluator,
    ExtensionCompatibilityReport,
    ExtensionCompatibilityRequest,
    ExtensionCompatibilityStatus,
};
use super::manifest::{
    ExtensionManifest,
    ManifestError,
    ManifestInputBudget,
};
use super::provider::{
    ProviderDescriptor,
    ProviderId,
    ProviderVersion,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable identity of the compiler extension validation module.
pub const EXTENSION_VALIDATION_MODULE_NAME: &str =
    "zamani.compiler.extensions.validation";

/// Public API version of this validation contract.
///
/// This version is independent of the language, grammar, provider,
/// manifest, backend, ZUIR and runtime versions.
pub const EXTENSION_VALIDATION_API_VERSION: u16 = 1;

/// Returns the canonical module identity.
#[must_use]
pub const fn module_name() -> &'static str {
    EXTENSION_VALIDATION_MODULE_NAME
}

/// Returns the validation API version.
#[must_use]
pub const fn api_version() -> u16 {
    EXTENSION_VALIDATION_API_VERSION
}

// ============================================================================
// VALIDATION REQUEST
// ============================================================================

/// Complete request for validating one extension manifest.
///
/// The request combines:
///
/// - An explicit provider compatibility request.
/// - Caller-supplied manifest processing budgets.
///
/// The compatibility request owns the host API version, provider
/// inventory and required contribution and extension-point contracts.
///
/// The manifest budget controls processing of untrusted declarations.
///
/// Neither input grants execution permission.
#[derive(Clone, Debug)]
pub struct ExtensionValidationRequest {
    compatibility: ExtensionCompatibilityRequest,
    manifest_budget: ManifestInputBudget,
}

impl ExtensionValidationRequest {
    /// Constructs a validation request from an explicit compatibility
    /// request and manifest input budget.
    #[must_use]
    pub fn new(
        compatibility: ExtensionCompatibilityRequest,
        manifest_budget: ManifestInputBudget,
    ) -> Self {
        Self {
            compatibility,
            manifest_budget,
        }
    }

    /// Constructs a request using the current host provider API and
    /// an unlimited local manifest budget.
    ///
    /// This convenience constructor does not mean that unlimited
    /// processing is recommended for untrusted input.
    ///
    /// Production callers should provide explicit operational budgets.
    #[must_use]
    pub fn current() -> Self {
        Self {
            compatibility: ExtensionCompatibilityRequest::current(),
            manifest_budget: ManifestInputBudget::unlimited(),
        }
    }

    /// Returns the provider compatibility request.
    #[must_use]
    pub fn compatibility(
        &self,
    ) -> &ExtensionCompatibilityRequest {
        &self.compatibility
    }

    /// Returns the manifest input budget.
    #[must_use]
    pub fn manifest_budget(
        &self,
    ) -> &ManifestInputBudget {
        &self.manifest_budget
    }

    /// Replaces the manifest processing budget.
    #[must_use]
    pub fn with_manifest_budget(
        mut self,
        budget: ManifestInputBudget,
    ) -> Self {
        self.manifest_budget = budget;
        self
    }

    /// Replaces the compatibility request.
    #[must_use]
    pub fn with_compatibility(
        mut self,
        compatibility: ExtensionCompatibilityRequest,
    ) -> Self {
        self.compatibility = compatibility;
        self
    }
}

// ============================================================================
// VALIDATION STATUS
// ============================================================================

/// Overall result of extension declaration and compatibility validation.
///
/// This is not a security or execution authorization decision.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[non_exhaustive]
pub enum ExtensionValidationStatus {
    /// Manifest validation succeeded and all requested compatibility
    /// requirements were established.
    Compatible,

    /// Manifest validation succeeded but an explicit compatibility
    /// requirement was violated.
    Incompatible,

    /// Manifest validation succeeded but compatibility could not
    /// be established from the supplied evidence.
    Indeterminate,
}

impl ExtensionValidationStatus {
    /// Returns whether compatibility was established.
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        matches!(self, Self::Compatible)
    }

    /// Returns whether an explicit incompatibility was established.
    #[must_use]
    pub const fn is_incompatible(self) -> bool {
        matches!(self, Self::Incompatible)
    }

    /// Returns whether compatibility remains unknown.
    #[must_use]
    pub const fn is_indeterminate(self) -> bool {
        matches!(self, Self::Indeterminate)
    }

    /// Returns whether this result permits the caller to proceed
    /// to independent security admission.
    ///
    /// This does not authorize registration, activation or execution.
    #[must_use]
    pub const fn may_proceed_to_security_admission(self) -> bool {
        self.is_compatible()
    }
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Complete result of validating one extension manifest.
///
/// The report retains the canonical provider descriptor and the
/// canonical compatibility report.
///
/// It deliberately does not contain a security verdict or execution
/// authorization token.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtensionValidationReport {
    descriptor: ProviderDescriptor,
    compatibility: ExtensionCompatibilityReport,
    status: ExtensionValidationStatus,
}

impl ExtensionValidationReport {
    /// Constructs a report from canonical validation results.
    fn new(
        descriptor: ProviderDescriptor,
        compatibility: ExtensionCompatibilityReport,
    ) -> Self {
        let status = match compatibility.status() {
            ExtensionCompatibilityStatus::Compatible => {
                ExtensionValidationStatus::Compatible
            }

            ExtensionCompatibilityStatus::Incompatible => {
                ExtensionValidationStatus::Incompatible
            }

            ExtensionCompatibilityStatus::Indeterminate => {
                ExtensionValidationStatus::Indeterminate
            }
        };

        Self {
            descriptor,
            compatibility,
            status,
        }
    }

    /// Returns the canonical provider descriptor.
    ///
    /// The descriptor may be passed to subsequent compatibility,
    /// security-admission or registration stages according to their
    /// independent contracts.
    ///
    /// Possessing this descriptor does not authorize provider activation.
    #[must_use]
    pub fn descriptor(&self) -> &ProviderDescriptor {
        &self.descriptor
    }

    /// Returns the validated provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        self.descriptor.id()
    }

    /// Returns the provider implementation version.
    #[must_use]
    pub const fn provider_version(&self) -> ProviderVersion {
        self.descriptor.version()
    }

    /// Returns the canonical compatibility report.
    #[must_use]
    pub fn compatibility(
        &self,
    ) -> &ExtensionCompatibilityReport {
        &self.compatibility
    }

    /// Returns the overall validation status.
    #[must_use]
    pub const fn status(&self) -> ExtensionValidationStatus {
        self.status
    }

    /// Returns whether structural validation and requested
    /// compatibility requirements succeeded.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        self.status.is_compatible()
    }

    /// Returns whether compatibility was explicitly violated.
    #[must_use]
    pub const fn is_incompatible(&self) -> bool {
        self.status.is_incompatible()
    }

    /// Returns whether compatibility could not be established.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        self.status.is_indeterminate()
    }

    /// Returns whether the caller may continue to independent
    /// security admission.
    ///
    /// This is not execution authorization.
    #[must_use]
    pub const fn may_proceed_to_security_admission(&self) -> bool {
        self.status.may_proceed_to_security_admission()
    }

    /// Returns the number of compatibility findings.
    #[must_use]
    pub fn finding_count(&self) -> usize {
        self.compatibility.finding_count()
    }

    /// Returns the number of blocking compatibility findings.
    #[must_use]
    pub fn blocking_count(&self) -> usize {
        self.compatibility.blocking_count()
    }

    /// Returns the number of indeterminate compatibility findings.
    #[must_use]
    pub fn indeterminate_count(&self) -> usize {
        self.compatibility.indeterminate_count()
    }
}

// ============================================================================
// VALIDATION ERROR
// ============================================================================

/// Errors preventing structural extension validation.
///
/// Compatibility violations are not represented as errors because
/// they are valid evaluation outcomes and are returned in a report.
///
/// Invalid manifests and invalid provider declarations are errors.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ExtensionValidationError {
    /// The extension manifest failed structural validation.
    Manifest(ManifestError),
}

impl fmt::Display for ExtensionValidationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Manifest(error) => {
                write!(
                    formatter,
                    "extension manifest validation failed: {error}"
                )
            }
        }
    }
}

impl Error for ExtensionValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Manifest(error) => Some(error),
        }
    }
}

impl From<ManifestError> for ExtensionValidationError {
    fn from(error: ManifestError) -> Self {
        Self::Manifest(error)
    }
}

/// Result type for compiler extension validation.
pub type ExtensionValidationResult<T> =
    Result<T, ExtensionValidationError>;

// ============================================================================
// VALIDATOR
// ============================================================================

/// Stateless compiler extension validator.
///
/// The validator coordinates the canonical manifest and compatibility
/// implementations. It owns no registry, provider inventory, discovery
/// mechanism, security provider or runtime state.
///
/// Instances can be reused across compilation sessions.
#[derive(Clone, Copy, Debug, Default)]
pub struct ExtensionValidator {
    compatibility_evaluator: ExtensionCompatibilityEvaluator,
}

impl ExtensionValidator {
    /// Creates a stateless extension validator.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            compatibility_evaluator:
                ExtensionCompatibilityEvaluator::new(),
        }
    }

    /// Validates one extension manifest.
    ///
    /// Processing order:
    ///
    /// 1. Validate the manifest using the supplied operational budget.
    /// 2. Convert it into the canonical ProviderDescriptor.
    /// 3. Evaluate the descriptor against the supplied compatibility
    ///    requirements and provider inventory.
    /// 4. Return the descriptor and complete compatibility report.
    ///
    /// Structural errors return `Err`.
    ///
    /// Compatibility violations and incomplete compatibility evidence
    /// return reports with explicit non-compatible statuses.
    ///
    /// This operation does not perform supply-chain verification,
    /// authorization, provider registration, loading or execution.
    pub fn validate(
        &self,
        manifest: &ExtensionManifest,
        request: &ExtensionValidationRequest,
    ) -> ExtensionValidationResult<ExtensionValidationReport> {
        manifest.validate_with_budget(
            request.manifest_budget(),
        )?;

        let descriptor = manifest.to_provider_descriptor()?;

        let compatibility = self.compatibility_evaluator.evaluate(
            &descriptor,
            request.compatibility(),
        );

        Ok(ExtensionValidationReport::new(
            descriptor,
            compatibility,
        ))
    }

    /// Validates one manifest using the current host API and the
    /// supplied manifest processing budget.
    ///
    /// The provider inventory remains partial unless the caller
    /// supplies an authoritative compatibility request through
    /// `validate`.
    pub fn validate_current(
        &self,
        manifest: &ExtensionManifest,
        budget: ManifestInputBudget,
    ) -> ExtensionValidationResult<ExtensionValidationReport> {
        let request = ExtensionValidationRequest::current()
            .with_manifest_budget(budget);

        self.validate(manifest, &request)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::extensions::compatibility::{
        CompatibilityFindingCode,
        ExtensionCompatibilityRequest,
        ExtensionCompatibilityStatus,
        ProviderInventory,
        ProviderInventoryCompleteness,
    };

    use crate::compiler::extensions::manifest::{
        ExtensionManifest,
        ManifestApiCompatibility,
        ManifestInputBudget,
    };

    use crate::compiler::extensions::provider::{
        api_major,
        api_minor,
        ProviderId,
        ProviderVersion,
    };

    fn api() -> ManifestApiCompatibility {
        ManifestApiCompatibility {
            major: api_major(),
            minimum_minor: 0,
            maximum_minor: None,
        }
    }

    fn manifest() -> ExtensionManifest {
        ExtensionManifest::new(
            "example.compiler",
            "1.0.0",
            api(),
            "Example compiler extension",
            vec!["compiler.pass".to_owned()],
        )
    }

    fn validator() -> ExtensionValidator {
        ExtensionValidator::new()
    }

    fn request() -> ExtensionValidationRequest {
        ExtensionValidationRequest::new(
            ExtensionCompatibilityRequest::current(),
            ManifestInputBudget::unlimited(),
        )
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            module_name(),
            "zamani.compiler.extensions.validation"
        );
    }

    #[test]
    fn api_version_is_nonzero() {
        assert_ne!(api_version(), 0);
    }

    #[test]
    fn valid_manifest_is_compatible_with_current_host() {
        let report = validator()
            .validate(&manifest(), &request())
            .expect("manifest should be structurally valid");

        assert!(report.is_compatible());
        assert_eq!(
            report.status(),
            ExtensionValidationStatus::Compatible
        );
        assert_eq!(
            report.provider_id().as_str(),
            "example.compiler"
        );
    }

    #[test]
    fn canonical_descriptor_is_preserved() {
        let report = validator()
            .validate(&manifest(), &request())
            .expect("manifest should validate");

        assert_eq!(
            report.descriptor().id().as_str(),
            "example.compiler"
        );

        assert_eq!(
            report.descriptor().version(),
            ProviderVersion::new(1, 0, 0)
        );
    }

    #[test]
    fn invalid_provider_identity_is_rejected() {
        let invalid = ExtensionManifest::new(
            "invalid provider identity",
            "1.0.0",
            api(),
            "Invalid provider",
            vec!["compiler.pass".to_owned()],
        );

        let result = validator().validate(
            &invalid,
            &request(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn invalid_provider_version_is_rejected() {
        let invalid = ExtensionManifest::new(
            "example.compiler",
            "not-a-version",
            api(),
            "Example compiler extension",
            vec!["compiler.pass".to_owned()],
        );

        let result = validator().validate(
            &invalid,
            &request(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn missing_contributions_are_rejected() {
        let invalid = ExtensionManifest::new(
            "example.compiler",
            "1.0.0",
            api(),
            "Example compiler extension",
            Vec::new(),
        );

        let result = validator().validate(
            &invalid,
            &request(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn manifest_budget_is_enforced() {
        let budget = ManifestInputBudget {
            maximum_text_bytes: Some(4),
            ..ManifestInputBudget::default()
        };

        let request = ExtensionValidationRequest::new(
            ExtensionCompatibilityRequest::current(),
            budget,
        );

        let result = validator().validate(
            &manifest(),
            &request,
        );

        assert!(result.is_err());
    }

    #[test]
    fn incompatible_host_api_is_reported() {
        let mut incompatible = manifest();
        incompatible.api.major = api_major().saturating_add(1);

        let report = validator()
            .validate(&incompatible, &request())
            .expect("manifest is structurally valid");

        assert!(report.is_incompatible());
        assert_eq!(
            report.compatibility().status(),
            ExtensionCompatibilityStatus::Incompatible
        );
    }

    #[test]
    fn missing_dependency_in_partial_inventory_is_indeterminate() {
        let mut dependent = manifest();

        dependent.dependencies.push(
            super::super::manifest::ManifestDependency {
                provider_id: "example.dependency".to_owned(),
                minimum_version: "1.0.0".to_owned(),
                maximum_version_exclusive: None,
            },
        );

        let request = ExtensionValidationRequest::new(
            ExtensionCompatibilityRequest::current()
                .with_inventory(ProviderInventory::new()),
            ManifestInputBudget::unlimited(),
        );

        let report = validator()
            .validate(&dependent, &request)
            .expect("manifest should be structurally valid");

        assert!(report.is_indeterminate());
        assert!(!report.is_compatible());
        assert!(!report.may_proceed_to_security_admission());
    }

    #[test]
    fn missing_dependency_in_complete_inventory_is_incompatible() {
        let mut dependent = manifest();

        dependent.dependencies.push(
            super::super::manifest::ManifestDependency {
                provider_id: "example.dependency".to_owned(),
                minimum_version: "1.0.0".to_owned(),
                maximum_version_exclusive: None,
            },
        );

        let inventory = ProviderInventory::from_versions(
            [],
            ProviderInventoryCompleteness::Complete,
        )
        .expect("empty complete inventory is valid");

        let request = ExtensionValidationRequest::new(
            ExtensionCompatibilityRequest::current()
                .with_inventory(inventory),
            ManifestInputBudget::unlimited(),
        );

        let report = validator()
            .validate(&dependent, &request)
            .expect("manifest should be structurally valid");

        assert!(report.is_incompatible());

        assert!(
            report.compatibility().findings().iter().any(
                |finding| {
                    finding.code()
                        == CompatibilityFindingCode::MissingDependency
                }
            )
        );
    }

    #[test]
    fn available_dependency_with_compatible_version_is_accepted() {
        let mut dependent = manifest();

        dependent.dependencies.push(
            super::super::manifest::ManifestDependency {
                provider_id: "example.dependency".to_owned(),
                minimum_version: "1.0.0".to_owned(),
                maximum_version_exclusive: Some(
                    "2.0.0".to_owned(),
                ),
            },
        );

        let inventory = ProviderInventory::from_versions(
            [(
                ProviderId::new("example.dependency")
                    .expect("valid dependency identity"),
                ProviderVersion::new(1, 5, 0),
            )],
            ProviderInventoryCompleteness::Complete,
        )
        .expect("valid inventory");

        let request = ExtensionValidationRequest::new(
            ExtensionCompatibilityRequest::current()
                .with_inventory(inventory),
            ManifestInputBudget::unlimited(),
        );

        let report = validator()
            .validate(&dependent, &request)
            .expect("manifest should be structurally valid");

        assert!(report.is_compatible());
    }

    #[test]
    fn validation_does_not_claim_security_admission() {
        let report = validator()
            .validate(&manifest(), &request())
            .expect("manifest should validate");

        assert!(report.is_compatible());

        // Compatibility only allows the caller to continue to
        // independent security admission. It does not authorize
        // provider activation or execution.
        assert!(report.may_proceed_to_security_admission());
    }

    #[test]
    fn current_request_uses_the_canonical_provider_api() {
        let request = ExtensionValidationRequest::current();

        assert_eq!(
            request.compatibility().host_api().major(),
            api_major()
        );

        assert_eq!(
            request.compatibility().host_api().minor(),
            api_minor()
        );
    }

    #[test]
    fn validator_is_stateless_and_thread_safe() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<ExtensionValidator>();
    }

    #[test]
    fn repeated_validation_produces_equal_reports() {
        let first = validator()
            .validate(&manifest(), &request())
            .expect("first validation should succeed");

        let second = validator()
            .validate(&manifest(), &request())
            .expect("second validation should succeed");

        assert_eq!(first, second);
    }
}
