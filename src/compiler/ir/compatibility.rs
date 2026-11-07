
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — IR Compatibility Integration
//!
//! Path: src/compiler/ir/compatibility.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide the compiler-facing compatibility integration boundary
//! for existing and future intermediate representations.
//!
//! This module coordinates two existing compatibility authorities:
//!
//! 1. Legacy IR compatibility:
//!    crate::classical::zcir::lowering::legacy_adapter
//!
//! 2. Target capability and resource compatibility:
//!    crate::backend::compatibility
//!
//! It does not implement a competing compatibility engine.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Compiler-facing compatibility entry points.
//! - Legacy compatibility API preservation.
//! - Backend compatibility API integration.
//! - Compiler compatibility admission decisions.
//! - Compatibility API versioning.
//! - Structured admission errors.
//!
//! It does not own:
//!
//! - Source syntax or grammar.
//! - AST construction.
//! - Semantic analysis.
//! - Type inference.
//! - Ownership analysis.
//! - Canonical ZUIR.
//! - Classical, quantum, HDL, AI or tensor IR.
//! - Domain-specific conversion algorithms.
//! - Backend capability definitions.
//! - Resource models.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Scheduling or routing.
//! - Code generation.
//! - Linking.
//! - Runtime execution.
//!
//! These responsibilities remain with their canonical owners.
//!
//! ## Canonical authorities
//!
//! Source grammar:
//! - grammar/Zamani.g4
//!
//! Source semantics:
//! - grammar/specification/semantic-model.md
//!
//! Semantic boundaries:
//! - grammar/validation/semantic-boundaries.md
//!
//! Canonical IR:
//! - crate::zuir
//! - crate::classical::zcir
//! - crate::quantum::ir
//! - crate::hdl::hdlir
//! - crate::ai
//!
//! Legacy compatibility:
//! - crate::classical::zcir::lowering::legacy_adapter
//!
//! Target compatibility:
//! - crate::backend::compatibility
//!
//! Compiler target descriptors:
//! - crate::compiler::configuration::target
//!
//! Compiler requirements:
//! - crate::compiler::analysis::requirements
//!
//! IR verification:
//! - crate::compiler::ir::verification
//!
//! ## Compatibility semantics
//!
//! Compatibility is an evidence-based decision.
//!
//! A representation is not compatible merely because its fields
//! resemble another representation.
//!
//! A target is not compatible merely because its identifier
//! resembles a supported architecture.
//!
//! Hard requirements must have sufficient supporting evidence.
//!
//! Unknown evidence is never accepted as proof.
//!
//! Partial support cannot satisfy a hard requirement.
//!
//! Experimental support is accepted for hard requirements only
//! when the canonical compatibility policy explicitly permits it.
//!
//! Preferences and hints cannot silently become hard requirements.
//!
//! An empty requirement collection proves no target compatibility.
//!
//! ## Architectural distinctions
//!
//! Requirement:
//!     What the program needs.
//!
//! Capability:
//!     What an implementation can support.
//!
//! Preference:
//!     What is desirable but not mandatory.
//!
//! Constraint:
//!     What an admissible realization must obey.
//!
//! Hint:
//!     Optional implementation guidance.
//!
//! Feasibility:
//!     Whether the available evidence supports a proposed realization.
//!
//! These concepts remain distinct throughout compilation.
//!
//! ## Verification boundary
//!
//! Compatibility assessment does not verify IR.
//!
//! Canonical ZUIR verification remains delegated to:
//!
//!     crate::zuir::verify::verify_full
//!
//! Domain representations must use their authoritative verifiers.
//!
//! Compatibility assessment cannot replace:
//!
//! - structural verification;
//! - type verification;
//! - effect verification;
//! - ownership verification;
//! - dialect verification;
//! - quantum validation;
//! - HDLIR verification;
//! - AI validation.
//!
//! A representation modified after verification must be verified
//! again before being trusted.
//!
//! ## Scalability and POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This module introduces no arbitrary maximum for:
//!
//! - Modules.
//! - Functions.
//! - Operations.
//! - Values.
//! - Types.
//! - Regions.
//! - Blocks.
//! - Qubits.
//! - Signals.
//! - Registers.
//! - Tensor dimensions.
//! - Tensor rank.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Accelerators.
//! - Distributed participants.
//! - Computing domains.
//! - Resource quantities.
//!
//! Resource quantities and compatibility evidence remain owned
//! by the canonical backend compatibility subsystem.
//!
//! This module introduces no fixed-width conversion of arbitrary
//! resource quantities.
//!
//! Processing remains subject to available memory, address space,
//! implementation support and explicitly configured budgets.
//!
//! Literal infinity cannot be guaranteed by finite software.
//!
//! Resource exhaustion must never be interpreted as compatibility.
//!
//! ## Identity boundaries
//!
//! Backend identity, target identity and IR representation identity
//! are distinct concepts.
//!
//! A target identifier must not be silently interpreted as a backend
//! provider identifier.
//!
//! A representation identifier must not be interpreted as a target.
//!
//! This module does not merge these identity systems.
//!
//! ## Determinism
//!
//! This module introduces:
//!
//! - No global mutable state.
//! - No random identifiers.
//! - No implicit target selection.
//! - No hardware discovery.
//! - No filesystem access.
//! - No network access.
//! - No console output.
//! - No mutation of supplied IR.
//!
//! Compatibility evaluation delegates to the canonical evaluator.
//!
//! ## Integration
//!
//! Parent:
//!     src/compiler/ir/mod.rs
//!
//! Backend compatibility:
//!     src/backend/compatibility.rs
//!
//! Backend module root:
//!     src/backend/mod.rs
//!
//! Legacy compatibility:
//!     src/classical/zcir/lowering/legacy_adapter.rs
//!
//! IR verification:
//!     src/compiler/ir/verification.rs
//!
//! IR adapters:
//!     src/compiler/ir/adapters.rs
//!
//! Compiler analysis:
//!     src/compiler/analysis/requirements.rs
//!
//! Target descriptors:
//!     src/compiler/configuration/target.rs
//!
//! Planning:
//!     src/compiler/planning/
//!
//! ## Freeze criteria
//!
//! - Existing legacy compatibility API remains available.
//! - Canonical backend compatibility types are reused.
//! - No duplicate capability or resource model is introduced.
//! - Unknown hard requirements are rejected.
//! - Empty requirements do not establish compatibility.
//! - Incompatible targets cannot pass the admission gate.
//! - Compatibility reports remain inspectable.
//! - No physical allocation is performed.
//! - No unsafe code is introduced.
//! - Parent module declarations resolve.
//! - Repository tests and quality gates pass.

use core::fmt;
use std::error::Error;

use crate::classical::zcir::lowering::legacy_adapter;

use crate::backend::compatibility::{
    CapabilityId,
    CapabilityRequirement,
    CapabilitySupport,
    CompatibilityError as BackendCompatibilityError,
    CompatibilityEvaluator,
    CompatibilityIssue,
    CompatibilityIssueCode,
    CompatibilityPolicy,
    CompatibilityReport,
    CompatibilityRequest,
    CompatibilitySeverity,
    CompatibilityStatus,
    DecimalInteger,
    InventoryCompleteness,
    RequirementStrength,
    ResourceAvailability,
    ResourceId,
    ResourceRequirement,
    SupportLevel,
    TargetCapabilityProfile,
};

// =============================================================================
// API VERSION
// =============================================================================

/// Version of the compiler-facing IR compatibility integration API.
///
/// This version is independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Semantic model version.
/// - ZUIR schema version.
/// - Domain IR versions.
/// - Backend API version.
/// - Backend provider versions.
/// - Target architecture versions.
pub const IR_COMPATIBILITY_API_VERSION: u16 = 2;

// =============================================================================
// LEGACY COMPATIBILITY API
// =============================================================================

/// Legacy compatibility finding.
///
/// This is the original canonical finding type owned by the
/// ZCIR legacy compatibility adapter.
///
/// The compiler does not duplicate its structure.
pub use legacy_adapter::LegacyFinding;

/// Legacy compatibility finding code.
///
/// The original canonical enum remains authoritative.
pub use legacy_adapter::LegacyFindingCode;

/// Legacy compatibility assessment.
///
/// This remains the original canonical assessment type.
pub use legacy_adapter::LegacyAssessment;

/// Compatibility alias retained for existing callers.
///
/// This is an alias of the canonical legacy finding type.
pub use legacy_adapter::LegacyFinding as CompatibilityFinding;

/// Compatibility alias retained for existing callers.
///
/// This is an alias of the canonical legacy finding code.
pub use legacy_adapter::LegacyFindingCode as CompatibilityFindingCode;

/// Compatibility alias retained for existing callers.
///
/// This is an alias of the canonical legacy assessment.
pub use legacy_adapter::LegacyAssessment as CompatibilityAssessment;

// =============================================================================
// CANONICAL BACKEND COMPATIBILITY CONTRACTS
// =============================================================================

/// Canonical capability identifier.
///
/// The underlying type belongs to `crate::backend::compatibility`.
pub use crate::backend::compatibility::CapabilityId as TargetCapabilityId;

/// Canonical resource identifier.
///
/// The underlying type belongs to `crate::backend::compatibility`.
pub use crate::backend::compatibility::ResourceId as TargetResourceId;

/// Canonical target capability profile.
///
/// This is not a backend provider identity or a physical allocation.
pub use crate::backend::compatibility::TargetCapabilityProfile
    as CompatibilityTargetProfile;

/// Canonical compatibility request.
///
/// The backend compatibility subsystem owns its interpretation.
pub use crate::backend::compatibility::CompatibilityRequest
    as TargetCompatibilityRequest;

/// Canonical compatibility report.
///
/// The original report and its findings are preserved.
pub use crate::backend::compatibility::CompatibilityReport
    as TargetCompatibilityReport;

// =============================================================================
// LEGACY IR COMPATIBILITY
// =============================================================================

/// Assess compatibility risks in an existing legacy IR module.
///
/// This delegates directly to the canonical ZCIR legacy adapter.
///
/// It does not:
///
/// - Mutate the module.
/// - Convert legacy IR to ZUIR.
/// - Establish semantic equivalence.
/// - Verify the module.
/// - Establish backend compatibility.
/// - Establish hardware feasibility.
///
/// A finding-free assessment is not proof of safe conversion.
///
/// # Arguments
///
/// `module` is an existing legacy IR module generated by
/// `crate::ir_gen`.
///
/// # Returns
///
/// The canonical legacy compatibility assessment.
#[must_use]
pub fn assess_legacy_ir(
    module: &crate::ir_gen::IrModule,
) -> CompatibilityAssessment {
    legacy_adapter::assess_legacy_module(module)
}

/// Determine whether the canonical legacy assessment permits
/// automatic conversion.
///
/// The decision remains owned by the legacy adapter.
///
/// No independent conversion policy is introduced here.
#[must_use]
pub const fn permits_automatic_legacy_conversion(
    assessment: &CompatibilityAssessment,
) -> bool {
    assessment.permits_automatic_conversion()
}

// =============================================================================
// TARGET COMPATIBILITY EVALUATION
// =============================================================================

/// Evaluate a compilation request against an evidence-backed target.
///
/// This function delegates to the canonical backend compatibility
/// evaluator.
///
/// It does not perform:
///
/// - Target discovery.
/// - Backend provider selection.
/// - Physical allocation.
/// - Resource reservation.
/// - Scheduling.
/// - Routing.
/// - Lowering.
/// - Code generation.
/// - Runtime execution.
///
/// The returned report preserves the canonical backend result.
///
/// IMPORTANT:
///
/// A Compatible report means that the supplied compatibility
/// requirements were satisfied by the supplied evidence.
///
/// It does not mean that the target is physically reserved or
/// guaranteed to remain available.
///
/// An empty request can produce a vacuously compatible backend
/// report. Use `ensure_compatible` before admitting a target
/// for compilation.
#[must_use]
pub fn evaluate_compatibility(
    request: &CompatibilityRequest,
    target: &TargetCapabilityProfile,
) -> CompatibilityReport {
    CompatibilityEvaluator.evaluate(request, target)
}

/// Check whether a request contains at least one hard requirement.
///
/// A request containing only preferences or hints cannot establish
/// that a target satisfies the program's mandatory requirements.
///
/// Required and Constraint are hard requirement strengths.
///
/// This function does not mutate the request.
#[must_use]
pub fn has_hard_requirements(
    request: &CompatibilityRequest,
) -> bool {
    request
        .capabilities
        .iter()
        .any(|item| item.strength.is_hard())
        || request
            .resources
            .iter()
            .any(|item| item.strength.is_hard())
}

/// Determine whether a compatibility report permits proceeding
/// through the target compatibility gate.
///
/// Only Compatible and CompatibleWithWarnings are accepted.
///
/// Indeterminate and Incompatible are rejected.
///
/// The caller must separately establish that the request contains
/// meaningful hard requirements.
#[must_use]
pub fn report_permits_compilation(
    report: &CompatibilityReport,
) -> bool {
    report.is_compatible()
}

/// Evaluate a request and determine whether it may pass the
/// compiler-facing target compatibility gate.
///
/// This function returns false when:
///
/// - The request has no hard requirements.
/// - A hard capability is unsupported.
/// - A hard capability has unknown support.
/// - A hard capability has only partial support.
/// - Required resource capacity is insufficient.
/// - Required resource availability is unknown.
/// - The canonical evaluator reports incompatibility.
/// - The canonical evaluator reports indeterminate evidence.
///
/// Experimental support is accepted only when the canonical
/// request policy explicitly permits it.
///
/// This function does not guarantee physical availability.
#[must_use]
pub fn permits_compilation(
    request: &CompatibilityRequest,
    target: &TargetCapabilityProfile,
) -> bool {
    if !has_hard_requirements(request) {
        return false;
    }

    let report = evaluate_compatibility(request, target);

    report_permits_compilation(&report)
}

// =============================================================================
// STRICT COMPATIBILITY ADMISSION
// =============================================================================

/// Reason why a compatibility request cannot pass admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityGateFailure {
    /// The request contains no hard requirements.
    ///
    /// Preferences and hints alone do not establish compatibility.
    NoHardRequirements,

    /// The canonical evaluator reports insufficient evidence.
    Indeterminate,

    /// The canonical evaluator establishes incompatibility.
    Incompatible,
}

/// Structured error returned by strict compatibility admission.
///
/// The complete canonical report is retained so that callers
/// can inspect every finding without reconstructing the evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityGateError {
    failure: CompatibilityGateFailure,
    report: CompatibilityReport,
}

impl CompatibilityGateError {
    /// Create an admission error while retaining the canonical report.
    fn new(
        failure: CompatibilityGateFailure,
        report: CompatibilityReport,
    ) -> Self {
        Self {
            failure,
            report,
        }
    }

    /// Return the reason admission failed.
    #[must_use]
    pub const fn failure(&self) -> CompatibilityGateFailure {
        self.failure
    }

    /// Borrow the complete canonical compatibility report.
    #[must_use]
    pub fn report(&self) -> &CompatibilityReport {
        &self.report
    }

    /// Consume the error and recover its compatibility report.
    #[must_use]
    pub fn into_report(self) -> CompatibilityReport {
        self.report
    }
}

impl fmt::Display for CompatibilityGateError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self.failure {
            CompatibilityGateFailure::NoHardRequirements => {
                write!(
                    formatter,
                    "target compatibility cannot be established: \
                     the request contains no hard requirements"
                )
            }

            CompatibilityGateFailure::Indeterminate => {
                write!(
                    formatter,
                    "target compatibility is indeterminate for `{}`",
                    self.report.target_id
                )
            }

            CompatibilityGateFailure::Incompatible => {
                write!(
                    formatter,
                    "target `{}` does not satisfy the compilation requirements",
                    self.report.target_id
                )
            }
        }
    }
}

impl Error for CompatibilityGateError {}

/// Evaluate and strictly admit a target for a compilation request.
///
/// Unlike `evaluate_compatibility`, this function refuses to treat
/// an empty requirement set as proof of target suitability.
///
/// On failure, the complete canonical compatibility report is
/// retained in `CompatibilityGateError`.
///
/// # Success
///
/// Returns the canonical report when:
///
/// 1. At least one hard requirement exists.
/// 2. The canonical evaluator establishes compatibility.
///
/// CompatibleWithWarnings is accepted when the canonical policy
/// explicitly permits the corresponding conditions.
///
/// # Failure
///
/// Returns:
///
/// - NoHardRequirements.
/// - Indeterminate.
/// - Incompatible.
///
/// This is a compatibility admission decision, not a physical
/// resource reservation or execution guarantee.
pub fn ensure_compatible(
    request: &CompatibilityRequest,
    target: &TargetCapabilityProfile,
) -> Result<CompatibilityReport, CompatibilityGateError> {
    let report = evaluate_compatibility(request, target);

    if !has_hard_requirements(request) {
        return Err(CompatibilityGateError::new(
            CompatibilityGateFailure::NoHardRequirements,
            report,
        ));
    }

    match report.status {
        CompatibilityStatus::Compatible
        | CompatibilityStatus::CompatibleWithWarnings => Ok(report),

        CompatibilityStatus::Indeterminate => {
            Err(CompatibilityGateError::new(
                CompatibilityGateFailure::Indeterminate,
                report,
            ))
        }

        CompatibilityStatus::Incompatible => {
            Err(CompatibilityGateError::new(
                CompatibilityGateFailure::Incompatible,
                report,
            ))
        }
    }
}

/// Evaluate compatibility and return the canonical report.
///
/// This is an explicitly named alternative for callers that prefer
/// the target-specific terminology.
#[must_use]
pub fn check_target_compatibility(
    request: &CompatibilityRequest,
    target: &TargetCapabilityProfile,
) -> CompatibilityReport {
    evaluate_compatibility(request, target)
}

// =============================================================================
// API INFORMATION
// =============================================================================

/// Return the compiler IR compatibility integration API version.
#[must_use]
pub const fn api_version() -> u16 {
    IR_COMPATIBILITY_API_VERSION
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn capability(value: &str) -> CapabilityId {
        CapabilityId::new(value).expect("valid capability")
    }

    fn resource(value: &str) -> ResourceId {
        ResourceId::new(value).expect("valid resource")
    }

    fn target() -> TargetCapabilityProfile {
        TargetCapabilityProfile::new("test-target")
            .expect("valid target")
    }

    fn supported_target() -> TargetCapabilityProfile {
        let mut target = target();

        target
            .declare_capability(CapabilitySupport::new(
                capability("cpu.integer"),
                SupportLevel::Supported,
            ))
            .expect("unique capability");

        target
    }

    fn required_request() -> CompatibilityRequest {
        CompatibilityRequest::new().with_capability(
            CapabilityRequirement::required(
                capability("cpu.integer"),
            ),
        )
    }

    #[test]
    fn compatibility_api_version_is_stable() {
        assert_eq!(api_version(), IR_COMPATIBILITY_API_VERSION);
        assert_eq!(IR_COMPATIBILITY_API_VERSION, 2);
    }

    #[test]
    fn canonical_backend_types_are_reused() {
        fn accepts_id(_: CapabilityId) {}
        fn accepts_resource(_: ResourceId) {}
        fn accepts_request(_: CompatibilityRequest) {}
        fn accepts_profile(_: TargetCapabilityProfile) {}
        fn accepts_report(_: CompatibilityReport) {}

        let _ = accepts_id;
        let _ = accepts_resource;
        let _ = accepts_request;
        let _ = accepts_profile;
        let _ = accepts_report;
    }

    #[test]
    fn supported_hard_requirement_passes_admission() {
        let request = required_request();
        let target = supported_target();

        let report = ensure_compatible(&request, &target)
            .expect("supported target");

        assert_eq!(
            report.status,
            CompatibilityStatus::Compatible
        );
    }

    #[test]
    fn unknown_hard_requirement_is_not_admitted() {
        let request = CompatibilityRequest::new()
            .with_capability(
                CapabilityRequirement::required(
                    capability("quantum.measurement"),
                ),
            );

        let report = evaluate_compatibility(&request, &target());

        assert_eq!(
            report.status,
            CompatibilityStatus::Indeterminate
        );

        assert!(!permits_compilation(&request, &target()));
    }

    #[test]
    fn empty_requirements_do_not_establish_compatibility() {
        let request = CompatibilityRequest::new();
        let target = target();

        let report = evaluate_compatibility(&request, &target);

        // Preserve the canonical backend evaluator's mathematical
        // result. The compiler admission gate applies the stricter
        // meaningful-requirement policy.
        assert_eq!(
            report.status,
            CompatibilityStatus::Compatible
        );

        assert!(!permits_compilation(&request, &target));

        let error = ensure_compatible(&request, &target)
            .expect_err("empty requirements must not pass admission");

        assert_eq!(
            error.failure(),
            CompatibilityGateFailure::NoHardRequirements
        );
    }

    #[test]
    fn preferences_alone_do_not_establish_admission() {
        let request = CompatibilityRequest::new()
            .with_capability(
                CapabilityRequirement::new(
                    capability("gpu.compute"),
                    RequirementStrength::Preference,
                ),
            );

        assert!(!has_hard_requirements(&request));
        assert!(!permits_compilation(&request, &target()));
    }

    #[test]
    fn explicit_unsupported_capability_is_rejected() {
        let mut target = target();

        target
            .declare_capability(CapabilitySupport::new(
                capability("cpu.integer"),
                SupportLevel::Unsupported,
            ))
            .expect("unique capability");

        let error = ensure_compatible(&required_request(), &target)
            .expect_err("unsupported capability");

        assert_eq!(
            error.failure(),
            CompatibilityGateFailure::Incompatible
        );

        assert!(!error.report().issues.is_empty());
    }

    #[test]
    fn partial_support_does_not_satisfy_hard_requirement() {
        let mut target = target();

        target
            .declare_capability(CapabilitySupport::new(
                capability("cpu.integer"),
                SupportLevel::Partial,
            ))
            .expect("unique capability");

        assert!(!permits_compilation(
            &required_request(),
            &target
        ));
    }

    #[test]
    fn experimental_support_requires_explicit_policy() {
        let mut target = target();

        target
            .declare_capability(CapabilitySupport::new(
                capability("gpu.compute"),
                SupportLevel::Experimental,
            ))
            .expect("unique capability");

        let request = CompatibilityRequest::new()
            .with_capability(
                CapabilityRequirement::required(
                    capability("gpu.compute"),
                ),
            );

        assert!(!permits_compilation(&request, &target));

        let permitted_request = request.with_policy(
            CompatibilityPolicy {
                allow_experimental_for_hard_requirements: true,
            },
        );

        let report = ensure_compatible(
            &permitted_request,
            &target,
        )
        .expect("explicit experimental permission");

        assert_eq!(
            report.status,
            CompatibilityStatus::CompatibleWithWarnings
        );
    }

    #[test]
    fn resource_requirements_use_canonical_decimal_quantities() {
        let mut target = target();

        target
            .declare_resource(
                resource("memory.bytes"),
                ResourceAvailability::known(
                    DecimalInteger::new("100000000000000000000")
                        .expect("valid arbitrary-length quantity"),
                ),
            )
            .expect("unique resource");

        let request = CompatibilityRequest::new()
            .with_resource(
                ResourceRequirement::required(
                    resource("memory.bytes"),
                    DecimalInteger::new("99999999999999999999")
                        .expect("valid quantity"),
                ),
            );

        let report = ensure_compatible(&request, &target)
            .expect("sufficient resource capacity");

        assert_eq!(
            report.status,
            CompatibilityStatus::Compatible
        );
    }

    #[test]
    fn insufficient_resources_are_rejected() {
        let mut target = target();

        target
            .declare_resource(
                resource("memory.bytes"),
                ResourceAvailability::known(
                    DecimalInteger::new("100")
                        .expect("valid quantity"),
                ),
            )
            .expect("unique resource");

        let request = CompatibilityRequest::new()
            .with_resource(
                ResourceRequirement::required(
                    resource("memory.bytes"),
                    DecimalInteger::new("101")
                        .expect("valid quantity"),
                ),
            );

        assert!(!permits_compilation(&request, &target));
    }

    #[test]
    fn indeterminate_failure_preserves_the_canonical_report() {
        let request = required_request();
        let target = target();

        let error = ensure_compatible(&request, &target)
            .expect_err("unknown capability");

        assert_eq!(
            error.failure(),
            CompatibilityGateFailure::Indeterminate
        );

        assert_eq!(
            error.report().status,
            CompatibilityStatus::Indeterminate
        );

        let report = error.into_report();

        assert_eq!(
            report.status,
            CompatibilityStatus::Indeterminate
        );
    }

    #[test]
    fn backend_and_target_identity_are_not_reinterpreted() {
        let target = TargetCapabilityProfile::new(
            "provider:device:architecture",
        )
        .expect("valid target identifier");

        assert_eq!(
            target.target_id,
            "provider:device:architecture"
        );
    }

    #[test]
    fn legacy_conversion_permission_remains_delegated() {
        fn check(
            assessment: &CompatibilityAssessment,
        ) -> bool {
            permits_automatic_legacy_conversion(assessment)
        }

        let _ = check;
    }

    #[test]
    fn hard_requirement_detection_handles_both_requirement_families() {
        let capability_request = CompatibilityRequest::new()
            .with_capability(
                CapabilityRequirement::required(
                    capability("cpu.integer"),
                ),
            );

        assert!(has_hard_requirements(&capability_request));

        let resource_request = CompatibilityRequest::new()
            .with_resource(
                ResourceRequirement::required(
                    resource("memory.bytes"),
                    DecimalInteger::zero(),
                ),
            );

        assert!(has_hard_requirements(&resource_request));
    }
}
