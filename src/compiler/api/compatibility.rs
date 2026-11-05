
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Compatibility API.
//!
//! Path: src/compiler/api/compatibility.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Provides the stable compiler API for evaluating whether a compilation
//! request can be realized by a target based on declared capability and
//! resource evidence.
//!
//! This module is an adapter over the canonical compatibility authority:
//!
//!     crate::backend::compatibility
//!
//! It does not implement another compatibility engine.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - compiler-facing compatibility evaluation;
//! - compatibility decision enforcement;
//! - compiler API compatibility errors;
//! - stable access to backend compatibility contracts.
//!
//! This module does not own:
//!
//! - capability definitions;
//! - resource definitions;
//! - target registries;
//! - physical hardware discovery;
//! - hardware allocation;
//! - semantic analysis;
//! - source-language requirements;
//! - canonical intermediate representations;
//! - hardware partitioning;
//! - quantum routing;
//! - resource scheduling;
//! - backend code generation;
//! - runtime execution.
//!
//! Those responsibilities remain with their existing repository owners.
//!
//! # POCO-REAF
//!
//! Zamani must support:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Compatibility evaluation is independent of:
//!
//! - processor architecture;
//! - processor count;
//! - memory capacity;
//! - accelerator count;
//! - GPU vendor;
//! - FPGA architecture;
//! - ASIC implementation;
//! - quantum processor provider;
//! - distributed topology;
//! - networking technology;
//! - operating system;
//! - future computing architecture.
//!
//! The compiler must not assume that a target supports a capability
//! merely because its name or architecture suggests that it should.
//!
//! Compatibility must be established using actual declared evidence.
//!
//! # Important compatibility rules
//!
//! 1. Explicitly unsupported hard requirements are rejected.
//! 2. Partially supported hard requirements are rejected.
//! 3. Unknown hard requirements are not considered satisfied.
//! 4. Experimental hard requirements require explicit policy permission.
//! 5. Preferences do not automatically make a target incompatible.
//! 6. Hints do not become mandatory requirements.
//! 7. Unknown resources never imply unlimited resources.
//! 8. Missing information is not silently interpreted as support.
//! 9. No arbitrary hardware-capacity ceilings are introduced.
//! 10. Compatibility evaluation does not allocate physical resources.
//!
//! # Compatibility classification
//!
//! Compatible:
//!     All evaluated requirements are satisfied.
//!
//! CompatibleWithWarnings:
//!     Hard requirements are satisfied, but warnings exist.
//!
//! Indeterminate:
//!     Available evidence is insufficient to establish feasibility.
//!
//! Incompatible:
//!     One or more hard requirements cannot be satisfied.
//!
//! Only Compatible and CompatibleWithWarnings may pass the compatibility
//! gate.
//!
//! Indeterminate must never be treated as successful compatibility.
//!
//! # Integration
//!
//! Upstream:
//!
//! - compiler/api/request.rs;
//! - compiler/api/compiler.rs;
//! - compiler/api/session.rs;
//! - compiler/analysis/requirements.rs;
//! - semantic analysis;
//! - canonical IR verification;
//! - hardware capability providers;
//! - backend target adapters.
//!
//! Downstream:
//!
//! - compiler target planning;
//! - compiler hardware partitioning;
//! - backend selection;
//! - backend validation;
//! - target-specific lowering;
//! - artifact generation.
//!
//! The compatibility request must be constructed from validated semantic
//! requirements and other authoritative program contracts.
//!
//! The target profile must come from actual capability providers.
//!
//! The backend must independently enforce its compatibility contract
//! before target-specific emission.
//!
//! # Compatibility with existing repository code
//!
//! The existing backend compatibility types are re-exported.
//!
//! No duplicate capability, resource, requirement, target-profile,
//! compatibility-report or evaluator types are introduced.
//!
//! Existing compiler entry points remain unchanged.
//!
//! # Error handling
//!
//! CompatibilityGateError preserves the complete compatibility report.
//!
//! Callers can inspect structured issue codes, severity, evidence,
//! target identity and compatibility classification.
//!
//! Human-readable messages are not used as machine-readable identifiers.
//!
//! # Scalability
//!
//! The module introduces no fixed hardware-count limits, resource-count
//! limits, device-count limits, processor-count limits, or capability
//! inventory limits.
//!
//! Resource quantities and identifiers use the existing backend
//! compatibility contracts.
//!
//! Operational resource exhaustion remains an explicit failure of the
//! relevant compiler, operating-system or resource-management subsystem.

use std::error::Error;
use std::fmt;

pub use crate::backend::compatibility::{
    CapabilityId,
    CapabilityRequirement,
    CapabilitySupport,
    CompatibilityError,
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

/// Error returned when a target does not have established compatibility
/// with the requirements of a compilation request.
///
/// The complete underlying compatibility report is preserved.
///
/// This allows callers to distinguish:
///
/// - actual incompatibility;
/// - insufficient evidence;
/// - unsupported capabilities;
/// - unsupported resources;
/// - incomplete capability inventories.
///
/// The error does not discard the original diagnostic information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityGateError {
    report: CompatibilityReport,
}

impl CompatibilityGateError {
    /// Constructs a compatibility-gate error from a non-compatible report.
    ///
    /// Compatible and CompatibleWithWarnings reports are rejected as
    /// invalid inputs because they have passed the compatibility gate.
    ///
    /// This constructor returns None for successful reports.
    #[must_use]
    pub fn from_report(report: CompatibilityReport) -> Option<Self> {
        if report.is_compatible() {
            return None;
        }

        Some(Self { report })
    }

    /// Returns the complete compatibility report.
    #[must_use]
    pub fn report(&self) -> &CompatibilityReport {
        &self.report
    }

    /// Returns the target identifier.
    #[must_use]
    pub fn target_id(&self) -> &str {
        &self.report.target_id
    }

    /// Returns the compatibility classification.
    #[must_use]
    pub fn status(&self) -> CompatibilityStatus {
        self.report.status
    }

    /// Returns true when the failure is caused by insufficient evidence.
    #[must_use]
    pub fn is_indeterminate(&self) -> bool {
        self.report.status == CompatibilityStatus::Indeterminate
    }

    /// Returns true when the target is explicitly incompatible.
    #[must_use]
    pub fn is_incompatible(&self) -> bool {
        self.report.status == CompatibilityStatus::Incompatible
    }

    /// Returns structured compatibility errors.
    pub fn issues(&self) -> &[CompatibilityIssue] {
        &self.report.issues
    }
}

impl fmt::Display for CompatibilityGateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let classification = match self.report.status {
            CompatibilityStatus::Compatible => "compatible",
            CompatibilityStatus::CompatibleWithWarnings => {
                "compatible with warnings"
            }
            CompatibilityStatus::Indeterminate => "indeterminate",
            CompatibilityStatus::Incompatible => "incompatible",
        };

        write!(
            formatter,
            "compilation target `{}` has {} compatibility",
            self.report.target_id,
            classification
        )?;

        for issue in &self.report.issues {
            write!(
                formatter,
                "\n- [{:?}] {:?}: {}",
                issue.severity,
                issue.code,
                issue.message
            )?;
        }

        Ok(())
    }
}

impl Error for CompatibilityGateError {}

/// Evaluates compilation requirements against a target's declared
/// capabilities and resource availability.
///
/// This function delegates to the canonical backend compatibility
/// evaluator.
///
/// It does not:
///
/// - infer capabilities from target names;
/// - discover physical hardware;
/// - allocate hardware resources;
/// - execute compiler stages;
/// - generate target-specific instructions.
///
/// The returned report preserves the canonical compatibility semantics.
///
/// # Determinism
///
/// The canonical evaluator is responsible for deterministic report
/// classification and issue ordering.
///
/// # Scalability
///
/// No additional hardware or resource ceilings are introduced.
#[must_use]
pub fn evaluate_compatibility(
    request: &CompatibilityRequest,
    target: &TargetCapabilityProfile,
) -> CompatibilityReport {
    CompatibilityEvaluator.evaluate(request, target)
}

/// Enforces the compatibility gate for a previously evaluated report.
///
/// Successful classifications:
///
/// - Compatible
/// - CompatibleWithWarnings
///
/// Rejected classifications:
///
/// - Indeterminate
/// - Incompatible
///
/// The complete report is preserved in CompatibilityGateError.
///
/// This function must be called before a backend is allowed to generate
/// an artifact for the requested target.
///
/// The caller remains responsible for independently verifying IR and
/// enforcing backend-specific safety and legality requirements.
pub fn ensure_compatible(
    report: CompatibilityReport,
) -> Result<CompatibilityReport, CompatibilityGateError> {
    match report.status {
        CompatibilityStatus::Compatible
        | CompatibilityStatus::CompatibleWithWarnings => Ok(report),

        CompatibilityStatus::Indeterminate
        | CompatibilityStatus::Incompatible => {
            let error = CompatibilityGateError::from_report(report)
                .expect("non-compatible reports must produce gate errors");

            Err(error)
        }
    }
}

/// Evaluates a compatibility request and enforces the resulting decision.
///
/// This is the preferred compiler-facing operation when a caller needs
/// both evaluation and compatibility enforcement.
///
/// The function never silently substitutes another target.
///
/// The function never treats unknown capabilities or resources as
/// successfully supported.
///
/// # Errors
///
/// Returns CompatibilityGateError when the target is incompatible or
/// when available evidence is insufficient to establish compatibility.
pub fn check_target_compatibility(
    request: &CompatibilityRequest,
    target: &TargetCapabilityProfile,
) -> Result<CompatibilityReport, CompatibilityGateError> {
    let report = evaluate_compatibility(request, target);

    ensure_compatible(report)
}

/// Determines whether a compatibility report permits further compilation.
///
/// This is a convenience operation for orchestration code.
///
/// It does not establish that:
///
/// - IR is valid;
/// - lowering is legal;
/// - an artifact can be generated;
/// - a physical device is available;
/// - runtime execution will succeed.
///
/// It only answers whether the compatibility classification passes
/// the compatibility gate.
#[must_use]
pub fn permits_compilation(report: &CompatibilityReport) -> bool {
    report.is_compatible()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn capability(value: &str) -> CapabilityId {
        CapabilityId::new(value)
            .expect("test capability identifier must be valid")
    }

    fn resource(value: &str) -> ResourceId {
        ResourceId::new(value)
            .expect("test resource identifier must be valid")
    }

    fn quantity(value: &str) -> DecimalInteger {
        DecimalInteger::new(value)
            .expect("test resource quantity must be valid")
    }

    fn target() -> TargetCapabilityProfile {
        TargetCapabilityProfile::new("test.target")
            .expect("test target identifier must be valid")
    }

    fn supported_target() -> TargetCapabilityProfile {
        let mut target = target();

        target
            .declare_capability(CapabilitySupport::new(
                capability("classical.integer"),
                SupportLevel::Supported,
            ))
            .expect("capability declaration must succeed");

        target
            .declare_resource(
                resource("memory.bytes"),
                ResourceAvailability::known(quantity("4096")),
            )
            .expect("resource declaration must succeed");

        target
    }

    #[test]
    fn supported_hard_requirements_pass_the_gate() {
        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::required(
                capability("classical.integer"),
            ));

        let target = supported_target();

        let report = check_target_compatibility(&request, &target)
            .expect("supported target must pass");

        assert_eq!(
            report.status,
            CompatibilityStatus::Compatible
        );

        assert!(report.issues.is_empty());
    }

    #[test]
    fn insufficient_resource_requirements_are_rejected() {
        let request = CompatibilityRequest::new()
            .with_resource(ResourceRequirement::required(
                resource("memory.bytes"),
                quantity("4097"),
            ));

        let target = supported_target();

        let error = check_target_compatibility(&request, &target)
            .expect_err("insufficient resources must be rejected");

        assert!(error.is_incompatible());

        assert_eq!(
            error.status(),
            CompatibilityStatus::Incompatible
        );

        assert_eq!(error.target_id(), "test.target");

        assert!(!error.issues().is_empty());
    }

    #[test]
    fn unsupported_hard_capabilities_are_rejected() {
        let mut target = target();

        target
            .declare_capability(CapabilitySupport::new(
                capability("quantum.mid_circuit_measurement"),
                SupportLevel::Unsupported,
            ))
            .expect("capability declaration must succeed");

        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::required(
                capability("quantum.mid_circuit_measurement"),
            ));

        let error = check_target_compatibility(&request, &target)
            .expect_err("unsupported capability must be rejected");

        assert!(error.is_incompatible());

        assert_eq!(
            error.issues()[0].code,
            CompatibilityIssueCode::CapabilityUnsupported
        );
    }

    #[test]
    fn unknown_hard_capabilities_never_pass_the_gate() {
        let target = target();

        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::required(
                capability("future.accelerator.operation"),
            ));

        let error = check_target_compatibility(&request, &target)
            .expect_err("unknown capability must not be accepted");

        assert!(error.is_indeterminate());

        assert_eq!(
            error.issues()[0].code,
            CompatibilityIssueCode::CapabilityUnknown
        );
    }

    #[test]
    fn partial_support_does_not_satisfy_hard_requirements() {
        let mut target = target();

        target
            .declare_capability(CapabilitySupport::new(
                capability("quantum.measurement"),
                SupportLevel::Partial,
            ))
            .expect("capability declaration must succeed");

        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::required(
                capability("quantum.measurement"),
            ));

        let error = check_target_compatibility(&request, &target)
            .expect_err("partial support must not satisfy a hard requirement");

        assert!(error.is_incompatible());

        assert_eq!(
            error.issues()[0].code,
            CompatibilityIssueCode::CapabilityPartiallySupported
        );
    }

    #[test]
    fn experimental_support_requires_explicit_permission() {
        let mut target = target();

        target
            .declare_capability(CapabilitySupport::new(
                capability("ai.experimental.acceleration"),
                SupportLevel::Experimental,
            ))
            .expect("capability declaration must succeed");

        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::required(
                capability("ai.experimental.acceleration"),
            ));

        let rejected = check_target_compatibility(&request, &target);

        assert!(rejected.is_err());

        let permitted_request = request.with_policy(
            CompatibilityPolicy {
                allow_experimental_for_hard_requirements: true,
            },
        );

        let accepted = check_target_compatibility(
            &permitted_request,
            &target,
        )
        .expect("explicitly permitted experimental support should pass");

        assert_eq!(
            accepted.status,
            CompatibilityStatus::CompatibleWithWarnings
        );

        assert!(accepted.has_warnings());
    }

    #[test]
    fn preferences_do_not_become_hard_requirements() {
        let target = target();

        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::new(
                capability("gpu.compute"),
                RequirementStrength::Preference,
            ));

        let report = check_target_compatibility(&request, &target)
            .expect("missing preference must not block compilation");

        assert_eq!(
            report.status,
            CompatibilityStatus::CompatibleWithWarnings
        );

        assert!(!report.has_errors());
    }

    #[test]
    fn hints_do_not_block_compatibility() {
        let target = target();

        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::new(
                capability("future.optimization"),
                RequirementStrength::Hint,
            ));

        let report = check_target_compatibility(&request, &target)
            .expect("missing hint must not block compilation");

        assert!(report.is_compatible());
    }

    #[test]
    fn complete_inventory_can_prove_missing_capability_unsupported() {
        let mut target = target();

        target.mark_capability_inventory_complete();

        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::required(
                capability("quantum.error_correction"),
            ));

        let error = check_target_compatibility(&request, &target)
            .expect_err("missing capability in complete inventory must fail");

        assert!(error.is_incompatible());

        assert_eq!(
            error.issues()[0].code,
            CompatibilityIssueCode::CapabilityUnsupported
        );
    }

    #[test]
    fn empty_requirements_pass_without_inventing_capabilities() {
        let target = target();

        let report = check_target_compatibility(
            &CompatibilityRequest::new(),
            &target,
        )
        .expect("empty compatibility request must pass");

        assert_eq!(
            report.status,
            CompatibilityStatus::Compatible
        );

        assert!(report.issues.is_empty());
    }

    #[test]
    fn compatibility_gate_preserves_original_report() {
        let target = target();

        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::required(
                capability("unknown.operation"),
            ));

        let original = evaluate_compatibility(&request, &target);

        let expected = original.clone();

        let error = ensure_compatible(original)
            .expect_err("unknown requirement must fail");

        assert_eq!(error.report(), &expected);
    }

    #[test]
    fn compatibility_error_has_structured_classification() {
        let target = target();

        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::required(
                capability("missing.operation"),
            ));

        let error = check_target_compatibility(&request, &target)
            .expect_err("unknown operation must fail");

        assert!(error.is_indeterminate());
        assert!(!error.is_incompatible());

        assert!(!error.to_string().is_empty());
    }

    #[test]
    fn compatible_reports_pass_the_convenience_check() {
        let target = supported_target();

        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::required(
                capability("classical.integer"),
            ));

        let report = evaluate_compatibility(&request, &target);

        assert!(permits_compilation(&report));
    }

    #[test]
    fn indeterminate_reports_do_not_pass_the_convenience_check() {
        let target = target();

        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::required(
                capability("unknown.capability"),
            ));

        let report = evaluate_compatibility(&request, &target);

        assert!(!permits_compilation(&report));
    }

    #[test]
    fn arbitrarily_large_resource_quantities_are_supported() {
        let target = {
            let mut target = target();

            target
                .declare_resource(
                    resource("memory.bytes"),
                    ResourceAvailability::known(quantity(
                        "999999999999999999999999999999999999999999999999999999999999999999",
                    )),
                )
                .expect("resource declaration must succeed");

            target
        };

        let request = CompatibilityRequest::new()
            .with_resource(ResourceRequirement::required(
                resource("memory.bytes"),
                quantity(
                    "999999999999999999999999999999999999999999999999999999999999999998",
                ),
            ));

        let report = check_target_compatibility(&request, &target)
            .expect("large supported resource quantity must pass");

        assert_eq!(
            report.status,
            CompatibilityStatus::Compatible
        );
    }

    #[test]
    fn compatibility_evaluation_does_not_modify_target_evidence() {
        let target = supported_target();

        let original = target.clone();

        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::required(
                capability("classical.integer"),
            ));

        let _ = evaluate_compatibility(&request, &target);

        assert_eq!(target, original);
    }

    #[test]
    fn duplicate_target_evidence_remains_rejected_by_authority() {
        let mut target = target();

        let first = CapabilitySupport::new(
            capability("classical.integer"),
            SupportLevel::Supported,
        );

        let second = CapabilitySupport::new(
            capability("classical.integer"),
            SupportLevel::Unsupported,
        );

        target
            .declare_capability(first)
            .expect("first declaration must succeed");

        assert!(matches!(
            target.declare_capability(second),
            Err(CompatibilityError::DuplicateCapability { .. })
        ));
    }

    #[test]
    fn gate_error_constructor_rejects_successful_reports() {
        let target = target();

        let report = evaluate_compatibility(
            &CompatibilityRequest::new(),
            &target,
        );

        assert!(
            CompatibilityGateError::from_report(report).is_none()
        );
    }

    #[test]
    fn compatible_with_warnings_is_not_rejected() {
        let target = target();

        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::new(
                capability("gpu.compute"),
                RequirementStrength::Preference,
            ));

        let report = evaluate_compatibility(&request, &target);

        assert_eq!(
            report.status,
            CompatibilityStatus::CompatibleWithWarnings
        );

        assert!(ensure_compatible(report).is_ok());
    }
}
