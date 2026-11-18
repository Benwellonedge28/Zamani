
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Target Validation
//!
//! Path: src/toolchain/targets/validation.rs
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the toolchain-facing validation boundary for explicitly
//! identified compilation targets.
//!
//! Coordinates the existing authoritative backend validation and
//! compatibility services without introducing competing target,
//! capability, requirement, or feasibility models.
//!
//! # Architectural authorities
//!
//! Compiler target intent:
//!     crate::compiler::configuration::target
//!
//! Backend target identity and descriptors:
//!     crate::backend::target
//!
//! Backend precondition validation:
//!     crate::backend::validation
//!
//! Capability and resource compatibility:
//!     crate::backend::compatibility
//!
//! Toolchain compatibility adapter:
//!     crate::toolchain::targets::compatibility
//!
//! Target discovery:
//!     crate::compiler::planning::target_discovery
//!
//! Capability matching:
//!     crate::compiler::planning::capability_matching
//!
//! Overall feasibility:
//!     crate::compiler::planning::feasibility
//!
//! Target selection:
//!     crate::toolchain::targets::selection
//!
//! Target profiles:
//!     crate::toolchain::targets::profiles
//!
//! Source target intent:
//!     grammar/compile/target.g4
//!
//! Target-selection policy:
//!     grammar/compile/target-selection.g4
//!
//! Semantic ownership:
//!     grammar/specification/semantic-model.md
//!     grammar/validation/semantic-boundaries.md
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Toolchain-facing target validation requests.
//! - Coordination of existing backend validation.
//! - Coordination of existing compatibility assessment.
//! - Requested artifact-format validation.
//! - Explicit emitter-registration evidence handling.
//! - Aggregated target-validation results.
//! - Readiness classification for the checks performed here.
//!
//! # Non-ownership
//!
//! This module does not own:
//!
//! - Target identifiers or target descriptors.
//! - Target registration or discovery.
//! - Target alias resolution.
//! - Capability identifiers or capability evidence.
//! - Workload requirements or their semantic interpretation.
//! - Compatibility matching algorithms.
//! - Resource feasibility algorithms.
//! - Hardware discovery or inventory.
//! - Device allocation or reservation.
//! - Target ranking or preference scoring.
//! - Compiler lowering or optimization.
//! - Quantum routing, scheduling, or QEC.
//! - HDL synthesis, placement, or timing closure.
//! - Runtime execution or deployment.
//! - External process execution.
//!
//! # Fundamental distinctions
//!
//! A structurally valid target is not necessarily a compatible target.
//!
//! A compatible target is not necessarily an available target.
//!
//! A registered emitter is not proof of successful code generation.
//!
//! A successful compatibility assessment is not proof of physical
//! resource availability.
//!
//! A successful target validation is not proof of execution.
//!
//! Missing evidence never establishes support.
//!
//! # POCO-REAF
//!
//! Target identifiers, artifact formats, capability identifiers,
//! and requirement dimensions remain open-world.
//!
//! No universal limits are introduced for:
//!
//! - Targets.
//! - Providers.
//! - CPUs or cores.
//! - GPUs or accelerators.
//! - FPGAs or ASICs.
//! - QPUs or qubits.
//! - Distributed nodes.
//! - Memory or storage.
//! - Tensor rank or dimensions.
//! - Register widths.
//! - Computational domains.
//!
//! Actual processing is bounded by available implementation resources
//! and explicitly configured operational budgets.
//!
//! Literal infinite physical resources or execution are not promised.
//!
//! # Determinism
//!
//! Requested artifact formats are stored in BTreeSet.
//!
//! Backend validation and compatibility reports remain authoritative.
//!
//! This module performs no random selection, implicit discovery,
//! environment inspection, or time-dependent operation.
//!
//! # Security
//!
//! This module performs no filesystem, network, process, credential,
//! hardware, or runtime operations.
//!
//! Emitter-registration evidence must come from the canonical backend
//! registry. It must never be inferred from a target identifier.
//!
//! Provider evidence must be authenticated and trusted by its
//! authoritative security and provider subsystems.
//!
//! # Integration contract
//!
//! 1. Compiler planning supplies validated target intent.
//! 2. Backend integration resolves intent to a canonical backend
//!    TargetDescriptor.
//! 3. The backend provider supplies a TargetCapabilityProfile for
//!    exactly that target identity.
//! 4. Semantic adapters supply the canonical CompatibilityRequest.
//! 5. The backend validator validates target descriptors.
//! 6. The backend validator validates requested artifact formats.
//! 7. The canonical backend registry supplies emitter-registration
//!    evidence when code generation readiness is required.
//! 8. TargetCompatibilityService evaluates requirements and evidence.
//! 9. This module aggregates the results without changing their meaning.
//! 10. Target selection, build orchestration, backend emission,
//!     resource planning and runtime execution continue downstream.
//!
//! Compiler-facing and backend-facing target identifiers must be
//! connected through an explicit validated mapping when they differ.
//!
//! An empty compatibility request does not establish universal
//! target support.

use std::collections::BTreeSet;

use crate::backend::compatibility::{
    CompatibilityReport,
    CompatibilityRequest,
    CompatibilityStatus,
    TargetCapabilityProfile,
};

use crate::backend::target::{
    ArtifactFormat,
    TargetDescriptor,
};

use crate::backend::validation::{
    BackendValidator,
    ValidationIssue,
    ValidationIssueKind,
    ValidationReport,
    ValidationSeverity,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
    ToolchainResult,
};

use crate::toolchain::targets::compatibility::{
    TargetCompatibilityService,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable architectural identity of this module.
pub const TARGET_VALIDATION_MODULE_NAME: &str =
    "zamani.toolchain.targets.validation";

/// Public API version of target validation.
pub const TARGET_VALIDATION_API_VERSION: u16 = 1;

/// Operation identifier attached to validation errors.
const OPERATION_VALIDATE: &str =
    "toolchain.targets.validation.validate";

// ============================================================================
// VALIDATION STATUS
// ============================================================================

/// Overall outcome of the target-validation checks performed here.
///
/// This status does not certify physical resource allocation, successful
/// code generation, runtime availability, or execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TargetValidationStatus {
    /// All requested checks completed and passed.
    Ready,

    /// Available evidence establishes that a required condition failed.
    Rejected,

    /// Available evidence is insufficient to establish readiness.
    Unresolved,
}

// ============================================================================
// VALIDATION REQUEST
// ============================================================================

/// Immutable inputs required for validating a compilation target.
///
/// All authoritative target and compatibility models are borrowed.
/// This request does not create competing representations.
///
/// # Emitter registration
///
/// `emitter_registered` has three possible states:
///
/// - `Some(true)`: the canonical backend registry established that an
///   emitter is registered for the target.
/// - `Some(false)`: the canonical backend registry established that
///   no emitter is registered.
/// - `None`: registration has not been established.
///
/// The default is `None`.
///
/// A descriptor's implementation-availability declaration does not
/// replace authoritative registry evidence.
///
/// # Artifact formats
///
/// Artifact formats are explicit requested outputs.
///
/// An empty collection means that this request makes no artifact-format
/// compatibility claim. It does not mean every format is supported.
#[derive(Debug)]
pub struct TargetValidationRequest<'a> {
    target: &'a TargetDescriptor,

    compatibility_request: &'a CompatibilityRequest,

    evidence: &'a TargetCapabilityProfile,

    required_artifact_formats: BTreeSet<ArtifactFormat>,

    emitter_registered: Option<bool>,
}

impl<'a> TargetValidationRequest<'a> {
    /// Construct a target-validation request.
    ///
    /// Emitter registration remains unresolved until explicitly supplied.
    #[must_use]
    pub fn new(
        target: &'a TargetDescriptor,
        compatibility_request: &'a CompatibilityRequest,
        evidence: &'a TargetCapabilityProfile,
    ) -> Self {
        Self {
            target,
            compatibility_request,
            evidence,
            required_artifact_formats: BTreeSet::new(),
            emitter_registered: None,
        }
    }

    /// Add a required artifact format.
    ///
    /// Duplicate formats are deduplicated deterministically.
    #[must_use]
    pub fn require_artifact_format(
        mut self,
        format: ArtifactFormat,
    ) -> Self {
        self.required_artifact_formats.insert(format);
        self
    }

    /// Add multiple required artifact formats.
    ///
    /// No fixed cardinality ceiling is imposed.
    #[must_use]
    pub fn require_artifact_formats(
        mut self,
        formats: impl IntoIterator<Item = ArtifactFormat>,
    ) -> Self {
        self.required_artifact_formats.extend(formats);
        self
    }

    /// Supply emitter-registration evidence from the canonical backend
    /// registry.
    ///
    /// This value must not be guessed or derived from target metadata.
    #[must_use]
    pub fn with_emitter_registration(
        mut self,
        registered: bool,
    ) -> Self {
        self.emitter_registered = Some(registered);
        self
    }

    /// Return the canonical backend target descriptor.
    #[must_use]
    pub fn target(&self) -> &'a TargetDescriptor {
        self.target
    }

    /// Return the canonical compatibility request.
    #[must_use]
    pub fn compatibility_request(&self) -> &'a CompatibilityRequest {
        self.compatibility_request
    }

    /// Return the target-scoped capability and resource evidence.
    #[must_use]
    pub fn evidence(&self) -> &'a TargetCapabilityProfile {
        self.evidence
    }

    /// Return the explicitly required artifact formats.
    #[must_use]
    pub fn required_artifact_formats(
        &self,
    ) -> &BTreeSet<ArtifactFormat> {
        &self.required_artifact_formats
    }

    /// Return emitter-registration evidence, if established.
    #[must_use]
    pub const fn emitter_registered(&self) -> Option<bool> {
        self.emitter_registered
    }

    /// Return whether the request explicitly contains artifact formats.
    #[must_use]
    pub fn has_artifact_requirements(&self) -> bool {
        !self.required_artifact_formats.is_empty()
    }
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Aggregated target-validation report.
///
/// The backend validation report and compatibility report retain their
/// original authoritative meanings.
///
/// The report records whether compatibility requirements were actually
/// supplied, preventing an empty compatibility request from being
/// interpreted as universal target certification.
#[derive(Debug, Clone)]
pub struct TargetValidationReport {
    target_id: String,

    backend: ValidationReport,

    compatibility: CompatibilityReport,

    compatibility_scope_is_empty: bool,

    artifact_formats_requested: BTreeSet<ArtifactFormat>,

    emitter_registration_established: bool,

    status: TargetValidationStatus,
}

impl TargetValidationReport {
    /// Return the canonical backend target identifier.
    #[must_use]
    pub fn target_id(&self) -> &str {
        &self.target_id
    }

    /// Return the aggregated backend validation report.
    #[must_use]
    pub fn backend_report(&self) -> &ValidationReport {
        &self.backend
    }

    /// Return the original backend compatibility report.
    #[must_use]
    pub fn compatibility_report(&self) -> &CompatibilityReport {
        &self.compatibility
    }

    /// Return the overall validation outcome.
    #[must_use]
    pub const fn status(&self) -> TargetValidationStatus {
        self.status
    }

    /// Return whether all checks requested here established readiness.
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        matches!(self.status, TargetValidationStatus::Ready)
    }

    /// Return whether available evidence establishes a rejection.
    #[must_use]
    pub const fn is_rejected(&self) -> bool {
        matches!(self.status, TargetValidationStatus::Rejected)
    }

    /// Return whether readiness remains unresolved.
    #[must_use]
    pub const fn is_unresolved(&self) -> bool {
        matches!(self.status, TargetValidationStatus::Unresolved)
    }

    /// Return whether the compatibility request contained no requirements.
    ///
    /// An empty request must not be interpreted as proof of universal
    /// compatibility.
    #[must_use]
    pub const fn compatibility_scope_is_empty(&self) -> bool {
        self.compatibility_scope_is_empty
    }

    /// Return the requested artifact formats.
    #[must_use]
    pub fn artifact_formats_requested(
        &self,
    ) -> &BTreeSet<ArtifactFormat> {
        &self.artifact_formats_requested
    }

    /// Return whether emitter registration was explicitly established.
    #[must_use]
    pub const fn emitter_registration_established(&self) -> bool {
        self.emitter_registration_established
    }

    /// Return whether the compatibility evaluator established acceptance.
    ///
    /// This does not establish physical resource availability.
    #[must_use]
    pub fn compatibility_established(&self) -> bool {
        matches!(
            self.compatibility.status,
            CompatibilityStatus::Compatible
                | CompatibilityStatus::CompatibleWithWarnings
        )
    }

    /// Require all checks represented by this report to establish
    /// code-generation readiness.
    ///
    /// This method does not claim that code generation itself has
    /// completed or that execution is possible.
    ///
    /// # Errors
    ///
    /// Returns a structured validation error when the report is rejected
    /// or unresolved.
    pub fn ensure_codegen_ready(&self) -> ToolchainResult<()> {
        if self.is_ready() {
            return Ok(());
        }

        let message = match self.status {
            TargetValidationStatus::Ready => unreachable!(),

            TargetValidationStatus::Rejected => {
                format!(
                    "target `{}` failed one or more required validation checks",
                    self.target_id,
                )
            }

            TargetValidationStatus::Unresolved => {
                format!(
                    "target `{}` has unresolved validation evidence; \
                     code-generation readiness has not been established",
                    self.target_id,
                )
            }
        };

        Err(
            ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                message,
            )
            .with_operation(OPERATION_VALIDATE)
            .with_subject(self.target_id.clone()),
        )
    }
}

// ============================================================================
// VALIDATION SERVICE
// ============================================================================

/// Stateless toolchain target-validation service.
///
/// The service delegates structural validation, artifact-format validation,
/// and compatibility evaluation to their canonical authorities.
///
/// It contains no target registry, hardware inventory, mutable global state,
/// provider list, or selection policy.
///
/// It can therefore be shared between independent compilation operations.
#[derive(Debug, Clone, Copy, Default)]
pub struct TargetValidationService;

impl TargetValidationService {
    /// Construct the stateless validation service.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Validate one explicitly identified backend target.
    ///
    /// Validation consists of:
    ///
    /// 1. Backend target descriptor validation.
    /// 2. Emitter-registration validation or explicit unresolved reporting.
    /// 3. Validation of every requested artifact format.
    /// 4. Canonical capability and resource compatibility evaluation.
    /// 5. Aggregation of the authoritative reports.
    ///
    /// No target is automatically substituted when validation fails.
    ///
    /// No capability is inferred from a target name.
    ///
    /// An empty compatibility request is permitted, but its scope is
    /// explicitly marked as empty in the resulting report.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The target descriptor is malformed.
    /// - The target is explicitly unavailable.
    /// - A requested artifact format is not declared by the target.
    /// - Emitter registration is explicitly false.
    /// - Evidence belongs to another target.
    /// - An authoritative backend validation operation fails.
    ///
    /// Incompatible and indeterminate compatibility assessments are
    /// returned as reports rather than being misrepresented as
    /// infrastructure failures.
    pub fn validate(
        &self,
        request: &TargetValidationRequest<'_>,
    ) -> ToolchainResult<TargetValidationReport> {
        let target = request.target();

        let backend_validator = BackendValidator::new();

        // ---------------------------------------------------------------
        // 1. Target descriptor and emitter registration
        // ---------------------------------------------------------------

        let mut backend_report = match request.emitter_registered() {
            Some(registered) => backend_validator
                .validate_codegen_target(target, registered)
                .map_err(|error| {
                    adapt_backend_error(
                        error,
                        target.id().as_str(),
                    )
                })?,

            None => {
                backend_validator
                    .validate_target_descriptor(target)
                    .map_err(|error| {
                        adapt_backend_error(
                            error,
                            target.id().as_str(),
                        )
                    })?
            }
        };

        backend_report
            .record_check("toolchain.target.descriptor")
            .map_err(|error| {
                adapt_backend_error(
                    error,
                    target.id().as_str(),
                )
            })?;

        // A target's descriptor availability declaration is not a
        // substitute for a registry lookup. Record this distinction
        // even if the descriptor advertises a legacy emitter.
        if request.emitter_registered().is_none() {
            let registration_already_deferred =
                backend_report.issues().iter().any(|issue| {
                    issue.kind()
                        == ValidationIssueKind::EmitterNotRegistered
                });

            if !registration_already_deferred {
                let issue = ValidationIssue::new(
                    ValidationIssueKind::EmitterNotRegistered,
                    ValidationSeverity::Deferred,
                    "canonical backend emitter registration has not \
                     been established",
                )
                .map_err(|error| {
                    adapt_backend_error(
                        error,
                        target.id().as_str(),
                    )
                })?
                .with_target(target.id().as_str());

                backend_report.push_issue(issue);
            }

            backend_report
                .record_check(
                    "toolchain.target.emitter_registration_pending",
                )
                .map_err(|error| {
                    adapt_backend_error(
                        error,
                        target.id().as_str(),
                    )
                })?;
        } else {
            backend_report
                .record_check(
                    "toolchain.target.emitter_registration",
                )
                .map_err(|error| {
                    adapt_backend_error(
                        error,
                        target.id().as_str(),
                    )
                })?;
        }

        // ---------------------------------------------------------------
        // 2. Requested artifact formats
        // ---------------------------------------------------------------

        if request.required_artifact_formats().is_empty() {
            backend_report
                .record_check(
                    "toolchain.target.no_artifact_format_requested",
                )
                .map_err(|error| {
                    adapt_backend_error(
                        error,
                        target.id().as_str(),
                    )
                })?;
        } else {
            for format in request.required_artifact_formats() {
                let artifact_report = backend_validator
                    .validate_artifact_format(target, format)
                    .map_err(|error| {
                        adapt_backend_error(
                            error,
                            target.id().as_str(),
                        )
                    })?;

                backend_report.merge(artifact_report);
            }
        }

        // ---------------------------------------------------------------
        // 3. Canonical compatibility assessment
        // ---------------------------------------------------------------

        let compatibility =
            TargetCompatibilityService::new()
                .evaluate(
                    target,
                    request.compatibility_request(),
                    request.evidence(),
                )?;

        // ---------------------------------------------------------------
        // 4. Preserve compatibility-request scope
        // ---------------------------------------------------------------

        let compatibility_scope_is_empty =
            request.compatibility_request().capabilities.is_empty()
                && request.compatibility_request().resources.is_empty();

        backend_report
            .record_check(
                "toolchain.target.compatibility_assessment",
            )
            .map_err(|error| {
                adapt_backend_error(
                    error,
                    target.id().as_str(),
                )
            })?;

        // ---------------------------------------------------------------
        // 5. Determine readiness without changing evidence semantics
        // ---------------------------------------------------------------

        let status = determine_status(
            &backend_report,
            compatibility.status,
        );

        Ok(TargetValidationReport {
            target_id: target.id().as_str().to_owned(),
            backend: backend_report,
            compatibility,
            compatibility_scope_is_empty,
            artifact_formats_requested:
                request.required_artifact_formats().clone(),
            emitter_registration_established:
                request.emitter_registered().is_some_and(|value| value),
            status,
        })
    }
}

// ============================================================================
// PUBLIC CONVENIENCE FUNCTION
// ============================================================================

/// Validate a target using the canonical toolchain validation service.
///
/// This convenience function has the same semantics as
/// `TargetValidationService::validate`.
pub fn validate_target(
    request: &TargetValidationRequest<'_>,
) -> ToolchainResult<TargetValidationReport> {
    TargetValidationService::new().validate(request)
}

// ============================================================================
// STATUS AGGREGATION
// ============================================================================

/// Determine overall readiness without promoting unknown evidence.
fn determine_status(
    backend: &ValidationReport,
    compatibility: CompatibilityStatus,
) -> TargetValidationStatus {
    if !backend.is_valid()
        || compatibility == CompatibilityStatus::Incompatible
    {
        return TargetValidationStatus::Rejected;
    }

    if !backend.is_complete()
        || compatibility == CompatibilityStatus::Indeterminate
    {
        return TargetValidationStatus::Unresolved;
    }

    match compatibility {
        CompatibilityStatus::Compatible
        | CompatibilityStatus::CompatibleWithWarnings => {
            TargetValidationStatus::Ready
        }

        CompatibilityStatus::Incompatible => {
            TargetValidationStatus::Rejected
        }

        CompatibilityStatus::Indeterminate => {
            TargetValidationStatus::Unresolved
        }
    }
}

// ============================================================================
// ERROR ADAPTATION
// ============================================================================

/// Preserve an authoritative backend error as the causal source of a
/// toolchain validation error.
fn adapt_backend_error(
    error: impl std::error::Error + Send + Sync + 'static,
    target_id: &str,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ValidationFailed,
        format!(
            "backend validation failed for target `{target_id}`: {error}"
        ),
    )
    .with_operation(OPERATION_VALIDATE)
    .with_subject(target_id.to_owned())
    .with_source(error)
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
        CompatibilityRequest,
        DecimalInteger,
        ResourceAvailability,
        ResourceId,
        ResourceRequirement,
        SupportLevel,
    };

    use crate::backend::target::{
        ExecutionModel,
        TargetId,
    };

    fn target(id: &str) -> TargetDescriptor {
        TargetDescriptor::new(
            TargetId::new(id).expect("valid target identity"),
            format!("Test target {id}"),
            ExecutionModel::Other(
                "test.execution-model".to_owned(),
            ),
        )
        .expect("valid target descriptor")
    }

    fn evidence(id: &str) -> TargetCapabilityProfile {
        TargetCapabilityProfile::new(id)
            .expect("valid target evidence")
    }

    fn capability(name: &str) -> CapabilityId {
        CapabilityId::new(name)
            .expect("valid capability identifier")
    }

    fn base_request<'a>(
        target: &'a TargetDescriptor,
        compatibility: &'a CompatibilityRequest,
        evidence: &'a TargetCapabilityProfile,
    ) -> TargetValidationRequest<'a> {
        TargetValidationRequest::new(
            target,
            compatibility,
            evidence,
        )
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            TARGET_VALIDATION_MODULE_NAME,
            "zamani.toolchain.targets.validation",
        );

        assert_ne!(TARGET_VALIDATION_API_VERSION, 0);
    }

    #[test]
    fn service_is_stateless_and_constructible() {
        let _service = TargetValidationService::new();
    }

    #[test]
    fn mismatched_evidence_identity_is_rejected() {
        let target = target("vendor:target-a");
        let evidence = evidence("vendor:target-b");
        let compatibility = CompatibilityRequest::new();

        let request =
            base_request(&target, &compatibility, &evidence);

        assert!(
            validate_target(&request).is_err(),
            "evidence from another target must never be accepted"
        );
    }

    #[test]
    fn unknown_required_capability_remains_unresolved() {
        let target = target("future:target");
        let evidence = evidence("future:target");

        let compatibility = CompatibilityRequest::new()
            .with_capability(
                CapabilityRequirement::required(
                    capability("future.domain.operation"),
                ),
            );

        let request =
            base_request(&target, &compatibility, &evidence);

        let report = validate_target(&request)
            .expect("validation should return an assessment");

        assert_eq!(
            report.status(),
            TargetValidationStatus::Unresolved,
        );

        assert!(!report.is_ready());
        assert!(
            !report.compatibility_established()
        );
    }

    #[test]
    fn explicitly_unsupported_capability_is_rejected() {
        let target = target("unsupported:target");

        let mut evidence = evidence("unsupported:target");

        evidence
            .declare_capability(
                CapabilitySupport::new(
                    capability("required.operation"),
                    SupportLevel::Unsupported,
                ),
            )
            .expect("unique capability");

        let compatibility = CompatibilityRequest::new()
            .with_capability(
                CapabilityRequirement::required(
                    capability("required.operation"),
                ),
            );

        let request =
            base_request(&target, &compatibility, &evidence);

        let report = validate_target(&request)
            .expect("compatibility rejection is a report");

        assert_eq!(
            report.status(),
            TargetValidationStatus::Rejected,
        );
    }

    #[test]
    fn complete_capability_inventory_can_establish_missing_support() {
        let target = target("complete:target");

        let mut evidence = evidence("complete:target");

        evidence.mark_capability_inventory_complete();

        let compatibility = CompatibilityRequest::new()
            .with_capability(
                CapabilityRequirement::required(
                    capability("missing.operation"),
                ),
            );

        let request =
            base_request(&target, &compatibility, &evidence);

        let report = validate_target(&request)
            .expect("assessment should complete");

        assert_eq!(
            report.status(),
            TargetValidationStatus::Rejected,
        );
    }

    #[test]
    fn descriptor_only_target_does_not_establish_emitter_readiness() {
        let target = target("metadata:only");
        let evidence = evidence("metadata:only");
        let compatibility = CompatibilityRequest::new();

        let request =
            base_request(&target, &compatibility, &evidence);

        let report = validate_target(&request)
            .expect("descriptor validation should complete");

        assert_eq!(
            report.status(),
            TargetValidationStatus::Unresolved,
        );

        assert!(
            !report.emitter_registration_established()
        );
    }

    #[test]
    fn empty_compatibility_request_is_explicitly_identified() {
        let target = target("future:empty-request");
        let evidence = evidence("future:empty-request");
        let compatibility = CompatibilityRequest::new();

        let request =
            base_request(&target, &compatibility, &evidence);

        let report = validate_target(&request)
            .expect("assessment should complete");

        assert!(
            report.compatibility_scope_is_empty()
        );

        // An empty compatibility request is not a universal
        // target-support certification.
        assert_eq!(
            report.compatibility_report().target_id,
            "future:empty-request",
        );
    }

    #[test]
    fn declared_artifact_format_is_validated() {
        use crate::backend::target::ArtifactFormat;

        let format =
            ArtifactFormat::new("future:artifact")
                .expect("valid artifact format");

        let target = TargetDescriptor::builder(
            TargetId::new("future:artifact-target")
                .expect("valid target"),
            "Future artifact target",
        )
        .artifact_format(format.clone())
        .build()
        .expect("valid descriptor");

        let evidence = evidence("future:artifact-target");
        let compatibility = CompatibilityRequest::new();

        let request = base_request(
            &target,
            &compatibility,
            &evidence,
        )
        .require_artifact_format(format);

        let report = validate_target(&request)
            .expect("declared format should validate");

        assert_eq!(
            report.artifact_formats_requested().len(),
            1,
        );
    }

    #[test]
    fn undeclared_artifact_format_is_rejected() {
        use crate::backend::target::ArtifactFormat;

        let target = target("no:artifact");
        let evidence = evidence("no:artifact");
        let compatibility = CompatibilityRequest::new();

        let format =
            ArtifactFormat::new("future:missing-format")
                .expect("valid format");

        let request = base_request(
            &target,
            &compatibility,
            &evidence,
        )
        .require_artifact_format(format);

        assert!(validate_target(&request).is_err());
    }

    #[test]
    fn explicit_missing_emitter_is_rejected() {
        let target = target("no:emitter");
        let evidence = evidence("no:emitter");
        let compatibility = CompatibilityRequest::new();

        let request = base_request(
            &target,
            &compatibility,
            &evidence,
        )
        .with_emitter_registration(false);

        assert!(validate_target(&request).is_err());
    }

    #[test]
    fn explicit_emitter_registration_resolves_descriptor_only_status() {
        let target = target("registered:emitter");
        let evidence = evidence("registered:emitter");
        let compatibility = CompatibilityRequest::new();

        let request = base_request(
            &target,
            &compatibility,
            &evidence,
        )
        .with_emitter_registration(true);

        let report = validate_target(&request)
            .expect("registered emitter should pass this check");

        assert!(
            report.emitter_registration_established()
        );

        assert_eq!(
            report.status(),
            TargetValidationStatus::Ready,
        );
    }

    #[test]
    fn large_resource_quantities_are_delegated_to_backend_evaluator() {
        let target = target("large:resource-target");

        let mut evidence = evidence("large:resource-target");

        let quantity =
            "999999999999999999999999999999999999999999999999999999999999";

        let resource = ResourceId::new("memory.bytes")
            .expect("valid resource identifier");

        let quantity =
            DecimalInteger::new(quantity)
                .expect("valid arbitrary-size decimal quantity");

        evidence
            .declare_resource(
                resource.clone(),
                ResourceAvailability::known(
                    quantity.clone(),
                ),
            )
            .expect("unique resource");

        let compatibility = CompatibilityRequest::new()
            .with_resource(
                ResourceRequirement::required(
                    resource,
                    quantity,
                ),
            );

        let request =
            base_request(&target, &compatibility, &evidence);

        let report = validate_target(&request)
            .expect("large resource quantity should be evaluated");

        assert_eq!(
            report.compatibility_report().status,
            CompatibilityStatus::Compatible,
        );
    }

    #[test]
    fn unresolved_readiness_cannot_be_promoted_to_codegen_ready() {
        let target = target("unresolved:target");
        let evidence = evidence("unresolved:target");
        let compatibility = CompatibilityRequest::new();

        let request =
            base_request(&target, &compatibility, &evidence);

        let report = validate_target(&request)
            .expect("assessment should complete");

        assert!(
            report.ensure_codegen_ready().is_err()
        );
    }

    #[test]
    fn compatibility_report_preserves_target_identity() {
        let target = target("stable:identity");
        let evidence = evidence("stable:identity");
        let compatibility = CompatibilityRequest::new();

        let request =
            base_request(&target, &compatibility, &evidence);

        let report = validate_target(&request)
            .expect("assessment should complete");

        assert_eq!(
            report.target_id(),
            report.compatibility_report().target_id,
        );
    }
}
