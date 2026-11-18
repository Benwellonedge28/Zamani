#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Target Compatibility Integration
//!
//! Path: `src/toolchain/targets/compatibility.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the toolchain-facing boundary for assessing whether an explicitly
//! identified backend target satisfies a build's declared compatibility
//! requirements.
//!
//! This module adapts and validates the association between:
//!
//! - A canonical backend target descriptor.
//! - A backend-owned target capability/resource evidence profile.
//! - A backend-owned compatibility request.
//!
//! It delegates the actual compatibility evaluation to
//! `crate::backend::compatibility::CompatibilityEvaluator`.
//!
//! ## Architectural authorities
//!
//! Target identity and description:
//!     crate::backend::target
//!
//! Compatibility requirements, evidence, evaluation and reports:
//!     crate::backend::compatibility
//!
//! Compiler-facing target intent:
//!     crate::compiler::configuration::target
//!
//! Target-profile configuration:
//!     crate::toolchain::targets::profiles
//!
//! Multi-target build aggregation:
//!     crate::toolchain::targets::matrix
//!
//! Structured toolchain errors:
//!     crate::toolchain::errors
//!
//! Source-level target intent:
//!     grammar/compile/target.g4
//!
//! Source-level target-selection policy:
//!     grammar/compile/target-selection.g4
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The toolchain-facing target compatibility service.
//! - Validation that a target descriptor and evidence profile refer to the
//!   same canonical backend target identity.
//! - Adaptation of backend validation errors into toolchain errors while
//!   preserving their causal sources.
//! - The public entry point used by target-matrix and build orchestration.
//!
//! ## Non-ownership
//!
//! This module does not own:
//!
//! - Target identifiers or target descriptors.
//! - Capability identifiers, declarations, or evidence.
//! - Workload requirements, preferences, constraints, or hints.
//! - Compatibility matching or resource comparison algorithms.
//! - Target registration or discovery.
//! - Target selection or preference ranking.
//! - Compiler semantic validation.
//! - IR verification or lowering.
//! - Resource planning or physical allocation.
//! - Backend discovery or implementation registration.
//! - External tool discovery or process execution.
//! - Quantum routing, scheduling, calibration, or error correction.
//! - HDL synthesis, placement, routing, or timing closure.
//! - Runtime execution or deployment.
//!
//! ## Compatibility semantics
//!
//! The backend compatibility evaluator is authoritative.
//!
//! Its outcomes retain their original meanings:
//!
//! - `Compatible`: the supplied requirements are satisfied.
//! - `CompatibleWithWarnings`: requirements are accepted under the supplied
//!   policy, with limitations or warnings retained.
//! - `Incompatible`: sufficient evidence establishes a hard requirement
//!   cannot be satisfied.
//! - `Indeterminate`: available evidence cannot establish compatibility.
//!
//! `Indeterminate` is never promoted to compatibility.
//!
//! A compatibility report applies only to the requirements in the supplied
//! request. It does not certify every operation, artifact format, ABI,
//! execution environment, provider, or future workload for that target.
//!
//! An empty request is evaluated according to the backend evaluator's
//! vacuous-requirement semantics. Callers must not interpret that result as
//! proof of universal target support.
//!
//! ## Target identity
//!
//! The backend target descriptor is the identity authority at this boundary.
//!
//! The evidence profile's `target_id` must exactly equal
//! `TargetDescriptor::id().as_str()`.
//!
//! This module does not infer aliases from:
//!
//! - Architecture names.
//! - Vendor names.
//! - Operating-system names.
//! - Display names.
//! - Target triples.
//! - Artifact formats.
//!
//! If compiler-facing and backend-facing target identities differ, the
//! compiler/backend integration must supply an explicit, validated mapping.
//! This module must not invent one.
//!
//! ## Requirements and policies
//!
//! Requirements, constraints, preferences, hints, capability requirements,
//! resource requirements, evidence completeness, and experimental-support
//! policy remain defined by `backend::compatibility` and its upstream
//! semantic adapters.
//!
//! This module does not downgrade hard requirements or promote preferences
//! into mandatory requirements.
//!
//! ## POCO-REAF and scalability
//!
//! Target and capability identifiers remain open-world.
//!
//! No domain list, vendor list, target catalogue, processor count, device
//! count, qubit count, node count, memory ceiling, tensor-rank ceiling,
//! register-width ceiling, or other universal hardware limit is introduced.
//!
//! Resource quantities and their comparisons remain owned by the backend
//! compatibility contract.
//!
//! Collection growth is bounded only by available implementation resources
//! and explicitly configured operational budgets in their owning layers.
//!
//! Literal infinite hardware, storage, memory, or execution is not promised.
//!
//! ## Determinism
//!
//! This adapter performs no candidate discovery, selection, ranking,
//! environment inspection, or time-dependent operation.
//!
//! The backend evaluator owns deterministic issue ordering.
//!
//! ## Security
//!
//! This module performs no filesystem, network, process, environment,
//! credential, hardware, or runtime access.
//!
//! It does not authenticate evidence. Evidence provenance and trust validation
//! remain the responsibility of the backend, provider, and security layers.
//!
//! ## Integration contract
//!
//! 1. The compiler produces or supplies a validated target intent.
//! 2. The backend integration resolves that intent to a canonical backend
//!    `TargetDescriptor`.
//! 3. The backend/provider supplies a `TargetCapabilityProfile` for that
//!    exact target identity.
//! 4. Semantic and compiler planning adapters produce a
//!    `CompatibilityRequest` without changing requirement strength.
//! 5. This service validates the descriptor and target/profile association.
//! 6. The canonical backend evaluator produces the compatibility report.
//! 7. `targets::matrix` records the authoritative assessment and provenance.
//! 8. Build planning and backend validation decide whether later compilation
//!    or realization stages may proceed.
//!
//! A successful compatibility assessment is not a compiled artifact and is
//! not proof that physical resources have been allocated.

use crate::backend::compatibility::{
    CompatibilityEvaluator,
    CompatibilityReport,
    CompatibilityRequest,
    TargetCapabilityProfile,
};

use crate::backend::target::TargetDescriptor;

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
    ToolchainResult,
};

/// Stable architectural identity of this module.
pub const TARGET_COMPATIBILITY_MODULE_NAME: &str =
    "zamani.toolchain.targets.compatibility";

/// Public API version of the toolchain target-compatibility adapter.
pub const TARGET_COMPATIBILITY_API_VERSION: u16 = 1;

/// Operation identifier attached to errors from this module.
const OPERATION_EVALUATE: &str = "toolchain.targets.compatibility.evaluate";

/// Stateless toolchain adapter for canonical backend compatibility.
///
/// This service owns no registry, cache, target list, provider list, mutable
/// policy, or hardware state. Each call evaluates the explicitly supplied
/// inputs using the canonical backend evaluator.
///
/// The service is safe to share between independent build operations because
/// it contains no mutable state.
#[derive(Debug, Clone, Copy, Default)]
pub struct TargetCompatibilityService;

impl TargetCompatibilityService {
    /// Construct the stateless target compatibility service.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Evaluate one target against one compatibility request and evidence
    /// profile.
    ///
    /// The target descriptor and evidence profile must identify the same
    /// canonical backend target. A mismatch is rejected instead of silently
    /// evaluating evidence belonging to another target.
    ///
    /// The returned report is the backend's original report. Its status,
    /// issue codes, severity, evidence, and explanations are not rewritten.
    ///
    /// This method does not:
    ///
    /// - Discover or register targets.
    /// - Select another target on failure.
    /// - Allocate resources.
    /// - Verify provider authenticity.
    /// - Compile or generate artifacts.
    ///
    /// # Errors
    ///
    /// Returns a structured toolchain error when:
    ///
    /// - The target descriptor fails structural validation.
    /// - The evidence profile has an invalid target identity.
    /// - The profile belongs to a different target.
    ///
    /// Compatibility failures are returned as a successful
    /// `CompatibilityReport` with `Incompatible` or `Indeterminate` status.
    /// They are not converted into transport or infrastructure errors.
    pub fn evaluate(
        &self,
        target: &TargetDescriptor,
        request: &CompatibilityRequest,
        evidence: &TargetCapabilityProfile,
    ) -> ToolchainResult<CompatibilityReport> {
        validate_target_descriptor(target)?;

        validate_target_evidence_association(target, evidence)?;

        // Compatibility policy and matching are exclusively owned by the
        // backend compatibility subsystem. Do not reproduce its algorithms
        // or reinterpret its report here.
        let report = CompatibilityEvaluator.evaluate(request, evidence);

        // Defend the adapter boundary against a malformed or mismatched
        // report should the backend contract evolve.
        if report.target_id != target.id().as_str() {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::Internal,
                    format!(
                        "backend compatibility evaluator returned target \
                         `{}` for requested target `{}`",
                        report.target_id,
                        target.id(),
                    ),
                )
                .with_operation(OPERATION_EVALUATE)
                .with_subject(target.id().to_string()),
            );
        }

        Ok(report)
    }
}

/// Evaluate target compatibility through the canonical backend evaluator.
///
/// This is the convenience entry point for callers that do not need to retain
/// a `TargetCompatibilityService` value.
///
/// The request, target descriptor, and evidence profile are borrowed and are
/// not modified.
pub fn evaluate_target_compatibility(
    target: &TargetDescriptor,
    request: &CompatibilityRequest,
    evidence: &TargetCapabilityProfile,
) -> ToolchainResult<CompatibilityReport> {
    TargetCompatibilityService::new().evaluate(
        target,
        request,
        evidence,
    )
}

/// Validate the structural integrity of a backend target descriptor.
///
/// This function delegates descriptor validation to its owning backend
/// module and preserves the original error as the causal source.
fn validate_target_descriptor(
    target: &TargetDescriptor,
) -> ToolchainResult<()> {
    target.validate().map_err(|error| {
        ToolchainError::new(
            ToolchainErrorKind::ValidationFailed,
            format!(
                "backend target descriptor `{}` failed validation: {error}",
                target.id(),
            ),
        )
        .with_operation(OPERATION_EVALUATE)
        .with_subject(target.id().to_string())
        .with_source(error)
    })
}

/// Validate that the evidence profile belongs to the supplied target.
///
/// The backend evidence profile currently exposes its target identity as a
/// string. The target descriptor owns the validated `TargetId`.
///
/// Exact comparison is intentional. Target aliases require an explicit
/// mapping contract owned by the compiler/backend integration.
fn validate_target_evidence_association(
    target: &TargetDescriptor,
    evidence: &TargetCapabilityProfile,
) -> ToolchainResult<()> {
    let profile_target_id = evidence.target_id.as_str();

    if profile_target_id.trim().is_empty()
        || profile_target_id.trim() != profile_target_id
        || profile_target_id.chars().any(char::is_control)
    {
        return Err(
            ToolchainError::invalid_input(
                "target capability evidence contains an invalid target identity",
            )
            .with_operation(OPERATION_EVALUATE)
            .with_subject(target.id().to_string()),
        );
    }

    if profile_target_id != target.id().as_str() {
        return Err(
            ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                format!(
                    "target evidence identity `{profile_target_id}` does not \
                     match requested backend target `{}`; target aliases \
                     require an explicit compiler/backend mapping",
                    target.id(),
                ),
            )
            .with_operation(OPERATION_EVALUATE)
            .with_subject(target.id().to_string()),
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        evaluate_target_compatibility,
        TargetCompatibilityService,
        TARGET_COMPATIBILITY_API_VERSION,
        TARGET_COMPATIBILITY_MODULE_NAME,
    };

    use crate::backend::compatibility::{
        CapabilityId,
        CapabilityRequirement,
        CapabilitySupport,
        CompatibilityRequest,
        CompatibilityStatus,
        DecimalInteger,
        InventoryCompleteness,
        ResourceAvailability,
        ResourceId,
        ResourceRequirement,
        SupportLevel,
        TargetCapabilityProfile,
    };

    use crate::backend::target::{
        ExecutionModel,
        TargetDescriptor,
        TargetId,
    };

    fn target(id: &str) -> TargetDescriptor {
        TargetDescriptor::new(
            TargetId::new(id).expect("valid target ID"),
            format!("Target {id}"),
            ExecutionModel::Other("test.execution-model".to_owned()),
        )
        .expect("valid target descriptor")
    }

    fn profile(id: &str) -> TargetCapabilityProfile {
        TargetCapabilityProfile::new(id)
            .expect("valid target evidence profile")
    }

    fn capability(name: &str) -> CapabilityId {
        CapabilityId::new(name).expect("valid capability ID")
    }

    fn request_for(name: &str) -> CompatibilityRequest {
        CompatibilityRequest::new().with_capability(
            CapabilityRequirement::required(capability(name)),
        )
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            TARGET_COMPATIBILITY_MODULE_NAME,
            "zamani.toolchain.targets.compatibility",
        );

        assert_ne!(TARGET_COMPATIBILITY_API_VERSION, 0);
    }

    #[test]
    fn service_is_constructible_and_stateless() {
        let _service = TargetCompatibilityService::new();
    }

    #[test]
    fn matching_supported_capability_is_compatible() {
        let target = target("vendor:target-a");

        let mut evidence = profile("vendor:target-a");

        evidence
            .declare_capability(CapabilitySupport::new(
                capability("future.domain.operation"),
                SupportLevel::Supported,
            ))
            .expect("unique capability");

        let report = evaluate_target_compatibility(
            &target,
            &request_for("future.domain.operation"),
            &evidence,
        )
        .expect("assessment succeeds");

        assert_eq!(report.target_id, "vendor:target-a");
        assert_eq!(report.status, CompatibilityStatus::Compatible);
        assert!(report.issues.is_empty());
    }

    #[test]
    fn unknown_capability_remains_indeterminate() {
        let target = target("future.target");
        let evidence = profile("future.target");

        let report = evaluate_target_compatibility(
            &target,
            &request_for("future.domain.operation"),
            &evidence,
        )
        .expect("assessment succeeds");

        assert_eq!(report.status, CompatibilityStatus::Indeterminate);
        assert!(!report.is_compatible());
    }

    #[test]
    fn absent_capability_in_complete_inventory_is_incompatible() {
        let target = target("complete.target");
        let mut evidence = profile("complete.target");

        evidence.mark_capability_inventory_complete();

        let report = evaluate_target_compatibility(
            &target,
            &request_for("required.capability"),
            &evidence,
        )
        .expect("assessment succeeds");

        assert_eq!(report.status, CompatibilityStatus::Incompatible);
    }

    #[test]
    fn explicit_unsupported_capability_is_incompatible() {
        let target = target("unsupported.target");
        let mut evidence = profile("unsupported.target");

        evidence
            .declare_capability(CapabilitySupport::new(
                capability("required.capability"),
                SupportLevel::Unsupported,
            ))
            .expect("unique capability");

        let report = evaluate_target_compatibility(
            &target,
            &request_for("required.capability"),
            &evidence,
        )
        .expect("assessment succeeds");

        assert_eq!(report.status, CompatibilityStatus::Incompatible);
    }

    #[test]
    fn partial_support_does_not_satisfy_a_hard_requirement() {
        let target = target("partial.target");
        let mut evidence = profile("partial.target");

        evidence
            .declare_capability(CapabilitySupport::new(
                capability("required.capability"),
                SupportLevel::Partial,
            ))
            .expect("unique capability");

        let report = evaluate_target_compatibility(
            &target,
            &request_for("required.capability"),
            &evidence,
        )
        .expect("assessment succeeds");

        assert_eq!(report.status, CompatibilityStatus::Incompatible);
    }

    #[test]
    fn experimental_support_obeys_backend_policy() {
        let target = target("experimental.target");
        let mut evidence = profile("experimental.target");

        evidence
            .declare_capability(CapabilitySupport::new(
                capability("required.capability"),
                SupportLevel::Experimental,
            ))
            .expect("unique capability");

        let strict_report = evaluate_target_compatibility(
            &target,
            &request_for("required.capability"),
            &evidence,
        )
        .expect("assessment succeeds");

        assert_eq!(
            strict_report.status,
            CompatibilityStatus::Incompatible,
        );

        let permissive_request =
            request_for("required.capability").with_policy(
                crate::backend::compatibility::CompatibilityPolicy {
                    allow_experimental_for_hard_requirements: true,
                },
            );

        let permitted_report = evaluate_target_compatibility(
            &target,
            &permissive_request,
            &evidence,
        )
        .expect("assessment succeeds");

        assert_eq!(
            permitted_report.status,
            CompatibilityStatus::CompatibleWithWarnings,
        );

        assert!(permitted_report.has_warnings());
    }

    #[test]
    fn resource_comparison_uses_backend_arbitrary_length_quantities() {
        let target = target("large-resource.target");
        let mut evidence = profile("large-resource.target");

        evidence
            .declare_resource(
                ResourceId::new("memory.bytes")
                    .expect("valid resource ID"),
                ResourceAvailability::known(
                    DecimalInteger::new(
                        "999999999999999999999999999999999999999999999999",
                    )
                    .expect("valid decimal quantity"),
                ),
            )
            .expect("unique resource");

        let request = CompatibilityRequest::new().with_resource(
            ResourceRequirement::required(
                ResourceId::new("memory.bytes")
                    .expect("valid resource ID"),
                DecimalInteger::new(
                    "999999999999999999999999999999999999999999999998",
                )
                .expect("valid decimal quantity"),
            ),
        );

        let report = evaluate_target_compatibility(
            &target,
            &request,
            &evidence,
        )
        .expect("assessment succeeds");

        assert_eq!(report.status, CompatibilityStatus::Compatible);
    }

    #[test]
    fn insufficient_resource_is_incompatible() {
        let target = target("resource-limited.target");
        let mut evidence = profile("resource-limited.target");

        evidence
            .declare_resource(
                ResourceId::new("memory.bytes")
                    .expect("valid resource ID"),
                ResourceAvailability::known(
                    DecimalInteger::new("1024")
                        .expect("valid decimal quantity"),
                ),
            )
            .expect("unique resource");

        let request = CompatibilityRequest::new().with_resource(
            ResourceRequirement::required(
                ResourceId::new("memory.bytes")
                    .expect("valid resource ID"),
                DecimalInteger::new("1025")
                    .expect("valid decimal quantity"),
            ),
        );

        let report = evaluate_target_compatibility(
            &target,
            &request,
            &evidence,
        )
        .expect("assessment succeeds");

        assert_eq!(report.status, CompatibilityStatus::Incompatible);
    }

    #[test]
    fn a_profile_for_another_target_is_rejected() {
        let target = target("target.alpha");
        let evidence = profile("target.beta");

        let result = evaluate_target_compatibility(
            &target,
            &CompatibilityRequest::new(),
            &evidence,
        );

        assert!(result.is_err());
    }

    #[test]
    fn malformed_profile_identity_is_rejected() {
        let target = target("target.alpha");

        // The backend profile constructor rejects an empty identity, but its
        // public field permits external mutation. The adapter must still
        // validate the association at its trust boundary.
        let mut evidence = profile("target.alpha");
        evidence.target_id = "target.alpha\n".to_owned();

        let result = evaluate_target_compatibility(
            &target,
            &CompatibilityRequest::new(),
            &evidence,
        );

        assert!(result.is_err());
    }

    #[test]
    fn target_descriptor_is_validated_before_assessment() {
        let target = target("target.valid");
        let evidence = profile("target.valid");

        let report = evaluate_target_compatibility(
            &target,
            &CompatibilityRequest::new(),
            &evidence,
        )
        .expect("valid descriptor");

        assert_eq!(report.target_id, "target.valid");
    }

    #[test]
    fn incomplete_inventory_is_not_treated_as_complete() {
        let target = target("partial.inventory");
        let evidence = profile("partial.inventory");

        assert_eq!(
            evidence.inventory_completeness,
            InventoryCompleteness::Partial,
        );

        let report = evaluate_target_compatibility(
            &target,
            &request_for("unknown.capability"),
            &evidence,
        )
        .expect("assessment succeeds");

        assert_eq!(report.status, CompatibilityStatus::Indeterminate);
    }

    #[test]
    fn service_and_free_function_use_the_same_backend_evaluator() {
        let target = target("same.result");
        let mut evidence = profile("same.result");

        evidence
            .declare_capability(CapabilitySupport::new(
                capability("classical.integer"),
                SupportLevel::Supported,
            ))
            .expect("unique capability");

        let request = request_for("classical.integer");

        let service_report = TargetCompatibilityService::new()
            .evaluate(&target, &request, &evidence)
            .expect("service assessment");

        let function_report = evaluate_target_compatibility(
            &target,
            &request,
            &evidence,
        )
        .expect("function assessment");

        assert_eq!(service_report, function_report);
    }
}