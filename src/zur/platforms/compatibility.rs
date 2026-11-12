
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Platform Compatibility
//!
//! Path: src/zur/platforms/compatibility.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Evaluates compatibility between an explicitly described compilation
//! target and an explicitly supplied host environment.
//!
//! The module consumes existing backend target and capability models.
//! It does not introduce another target, capability, or requirement model.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Platform compatibility evaluation.
//! - Platform compatibility policy.
//! - Compatibility findings.
//! - Compatibility reports.
//! - Explicit comparison of target and host metadata.
//! - Required host facility checks.
//! - Required execution interface checks.
//! - Integration of backend capability matching.
//!
//! THIS MODULE DOES NOT OWN
//!
//! - Source-language semantics.
//! - AST or semantic models.
//! - Requirements originating in source code.
//! - Capability definitions.
//! - Capability discovery.
//! - Host discovery.
//! - Backend registration.
//! - Target selection.
//! - Compiler planning.
//! - Resource allocation.
//! - Resource reservation.
//! - Resource feasibility.
//! - Scheduling.
//! - Device selection.
//! - Quantum routing.
//! - Hardware placement.
//! - Runtime service registration.
//! - Runtime execution.
//! - Execution results.
//!
//! These remain with their authoritative repository subsystems.
//!
//! FUNDAMENTAL DISTINCTIONS
//!
//! Requirement != Capability.
//! Capability != Availability.
//! Availability != Feasibility.
//! Feasibility != Resource reservation.
//! Resource reservation != Execution.
//! Execution submission != Successful completion.
//!
//! A compatibility report is not proof of successful execution.
//!
//! SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Platform identifiers.
//! - Targets.
//! - Architectures.
//! - Operating systems.
//! - ABIs.
//! - Execution interfaces.
//! - Host facilities.
//! - Capabilities.
//! - Computational domains.
//! - Devices.
//! - CPUs.
//! - GPUs.
//! - QPUs.
//! - FPGAs.
//! - Photonic resources.
//! - Distributed participants.
//!
//! Collections grow according to available resources.
//!
//! Literal infinite physical computation is not promised.
//!
//! SAFETY
//!
//! No unsafe Rust.
//! No raw pointers.
//! No global mutable state.
//! No implicit hardware discovery.
//! No fabricated availability.
//! No implicit target selection.
//! No resource reservation.
//!
//! DETERMINISM
//!
//! All supplied evidence is evaluated deterministically.
//!
//! Capability matching delegates to the canonical backend capability
//! implementation.
//!
//! INTEGRATION
//!
//! Upstream:
//!
//! - backend/target.rs
//! - backend/capabilities.rs
//! - zur/platforms/host.rs
//! - zur/error.rs
//!
//! Downstream:
//!
//! - zur/execution/loader.rs
//! - zur/execution/dispatch.rs
//! - zur/core/context.rs
//! - zur/domains/
//! - zur/hybrid/
//!
//! Runtime services are checked separately through
//! zur/core/service_registry.rs.
//!
//! Artifact validation remains owned by zur/execution/artifact.rs.

use core::fmt;

use std::collections::BTreeSet;

use crate::backend::capabilities::{
    CapabilityCheckResult,
    CapabilityOutcome,
    CapabilityPolicy,
    CapabilityProfile,
    CapabilityRequirement,
};

use crate::backend::target::{
    ArtifactFormat,
    ImplementationAvailability,
    TargetDescriptor,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::platforms::host::{
    ExecutionInterfaceId,
    HostAvailability,
    HostEnvironment,
    HostFacilityId,
};

// =============================================================================
// COMPONENT IDENTIFICATION
// =============================================================================

const COMPONENT: &str = "zur.platforms.compatibility";

// =============================================================================
// ERROR HANDLING
// =============================================================================

fn compatibility_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::compatibility(message)
        .with_component(COMPONENT)
}

// =============================================================================
// COMPATIBILITY STATUS
// =============================================================================

/// Overall outcome of platform compatibility evaluation.
///
/// Compatibility is established only when every applicable mandatory
/// check succeeds and the required evidence is available.
///
/// An indeterminate result must never be interpreted as compatible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PlatformCompatibilityStatus {

    /// Every evaluated mandatory compatibility check succeeded.
    Compatible,

    /// At least one mandatory compatibility condition failed.
    Incompatible,

    /// Available evidence is insufficient to establish compatibility.
    Indeterminate,
}

impl PlatformCompatibilityStatus {

    /// Returns whether compatibility has been established.
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        matches!(self, Self::Compatible)
    }

    /// Returns whether a mandatory compatibility condition failed.
    #[must_use]
    pub const fn is_incompatible(self) -> bool {
        matches!(self, Self::Incompatible)
    }

    /// Returns whether additional evidence is necessary.
    #[must_use]
    pub const fn is_indeterminate(self) -> bool {
        matches!(self, Self::Indeterminate)
    }
}

impl fmt::Display for PlatformCompatibilityStatus {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        formatter.write_str(match self {
            Self::Compatible => "compatible",
            Self::Incompatible => "incompatible",
            Self::Indeterminate => "indeterminate",
        })
    }
}

// =============================================================================
// INDIVIDUAL CHECK STATUS
// =============================================================================

/// Result of one individual platform compatibility check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CompatibilityFindingStatus {

    /// The supplied evidence satisfies the evaluated condition.
    Satisfied,

    /// The supplied evidence contradicts the evaluated condition.
    Failed,

    /// The necessary evidence was not supplied.
    Unknown,

    /// The condition was not applicable.
    NotApplicable,
}

impl CompatibilityFindingStatus {

    /// Returns whether the check succeeded.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied)
    }

    /// Returns whether the check failed.
    #[must_use]
    pub const fn is_failed(self) -> bool {
        matches!(self, Self::Failed)
    }

    /// Returns whether the check requires additional evidence.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

// =============================================================================
// CHECK IDENTIFICATION
// =============================================================================

/// Identifies the category of a compatibility finding.
///
/// The categories are platform properties, not computational domains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CompatibilityCheckKind {

    /// Target implementation availability.
    TargetImplementation,

    /// Processor architecture.
    Architecture,

    /// Operating-system identity.
    OperatingSystem,

    /// Target triple.
    TargetTriple,

    /// ABI identity.
    Abi,

    /// ABI version.
    AbiVersion,

    /// Artifact format.
    ArtifactFormat,

    /// Host execution interface.
    ExecutionInterface,

    /// Host-system facility.
    HostFacility,

    /// Backend capability.
    Capability,
}

// =============================================================================
// COMPATIBILITY FINDING
// =============================================================================

/// Immutable result of one compatibility check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityFinding {

    kind: CompatibilityCheckKind,

    subject: String,

    status: CompatibilityFindingStatus,

    explanation: String,

    evidence_reference: Option<String>,
}

impl CompatibilityFinding {

    fn new(
        kind: CompatibilityCheckKind,
        subject: impl Into<String>,
        status: CompatibilityFindingStatus,
        explanation: impl Into<String>,
        evidence_reference: Option<String>,
    ) -> Self {

        Self {
            kind,
            subject: subject.into(),
            status,
            explanation: explanation.into(),
            evidence_reference,
        }
    }

    /// Returns the check category.
    #[must_use]
    pub const fn kind(&self) -> CompatibilityCheckKind {
        self.kind
    }

    /// Returns the identity of the checked subject.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Returns the finding status.
    #[must_use]
    pub const fn status(&self) -> CompatibilityFindingStatus {
        self.status
    }

    /// Returns the explanation.
    #[must_use]
    pub fn explanation(&self) -> &str {
        &self.explanation
    }

    /// Returns the supplied evidence reference, if available.
    #[must_use]
    pub fn evidence_reference(&self) -> Option<&str> {
        self.evidence_reference.as_deref()
    }
}

// =============================================================================
// COMPATIBILITY POLICY
// =============================================================================

/// Policy controlling platform compatibility evaluation.
///
/// Missing target or host metadata is not silently inferred.
///
/// The default policy is conservative.
///
/// Preferences and hints are intentionally excluded from mandatory
/// compatibility evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformCompatibilityPolicy {

    /// Capability matching policy.
    capability_policy: CapabilityPolicy,

    /// Require an explicitly available host execution interface.
    require_execution_interface_evidence: bool,

    /// Require an explicitly available host facility.
    require_host_facility_evidence: bool,

    /// Permit targets whose implementation is described but whose
    /// implementation availability is not established.
    allow_descriptor_only_targets: bool,
}

impl Default for PlatformCompatibilityPolicy {

    fn default() -> Self {
        Self {
            capability_policy: CapabilityPolicy::default(),
            require_execution_interface_evidence: true,
            require_host_facility_evidence: true,
            allow_descriptor_only_targets: false,
        }
    }
}

impl PlatformCompatibilityPolicy {

    /// Constructs the conservative default policy.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            capability_policy: CapabilityPolicy {
                allow_experimental: false,
                allow_deprecated: false,
                require_direct_evidence: false,
            },
            require_execution_interface_evidence: true,
            require_host_facility_evidence: true,
            allow_descriptor_only_targets: false,
        }
    }

    /// Sets the backend capability matching policy.
    #[must_use]
    pub const fn with_capability_policy(
        mut self,
        policy: CapabilityPolicy,
    ) -> Self {
        self.capability_policy = policy;
        self
    }

    /// Controls whether execution-interface evidence is mandatory.
    #[must_use]
    pub const fn requiring_execution_interface_evidence(
        mut self,
        required: bool,
    ) -> Self {
        self.require_execution_interface_evidence = required;
        self
    }

    /// Controls whether host-facility evidence is mandatory.
    #[must_use]
    pub const fn requiring_host_facility_evidence(
        mut self,
        required: bool,
    ) -> Self {
        self.require_host_facility_evidence = required;
        self
    }

    /// Controls whether descriptor-only targets may be evaluated.
    ///
    /// Permitting a descriptor-only target does not establish that
    /// an implementation exists or that the target can execute.
    #[must_use]
    pub const fn allowing_descriptor_only_targets(
        mut self,
        allowed: bool,
    ) -> Self {
        self.allow_descriptor_only_targets = allowed;
        self
    }

    /// Returns the capability matching policy.
    #[must_use]
    pub const fn capability_policy(&self) -> CapabilityPolicy {
        self.capability_policy
    }
}

// =============================================================================
// PLATFORM COMPATIBILITY REQUIREMENTS
// =============================================================================

/// Explicit mandatory platform conditions for a target realization.
///
/// This is a runtime-facing compatibility input, not a replacement
/// for the frontend semantic requirement model.
///
/// It deliberately does not own preferences or hints.
///
/// The source semantic layer and compiler planning subsystem must
/// determine which conditions are mandatory before constructing it.
#[derive(Debug, Clone, Default)]
pub struct PlatformCompatibilityRequirements {

    required_execution_interfaces:
        BTreeSet<ExecutionInterfaceId>,

    required_host_facilities:
        BTreeSet<HostFacilityId>,

    required_capabilities:
        Vec<CapabilityRequirement>,

    required_artifact_format:
        Option<ArtifactFormat>,
}

impl PlatformCompatibilityRequirements {

    /// Creates an empty set of mandatory platform conditions.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a required host execution interface.
    ///
    /// Duplicate identities are naturally deduplicated.
    #[must_use]
    pub fn requiring_execution_interface(
        mut self,
        interface: ExecutionInterfaceId,
    ) -> Self {

        self.required_execution_interfaces.insert(interface);

        self
    }

    /// Adds a required host facility.
    #[must_use]
    pub fn requiring_host_facility(
        mut self,
        facility: HostFacilityId,
    ) -> Self {

        self.required_host_facilities.insert(facility);

        self
    }

    /// Adds a mandatory backend capability requirement.
    ///
    /// Capability identity remains owned by the backend capability model.
    #[must_use]
    pub fn requiring_capability(
        mut self,
        requirement: CapabilityRequirement,
    ) -> Self {

        self.required_capabilities.push(requirement);

        self
    }

    /// Specifies the required artifact format.
    #[must_use]
    pub fn requiring_artifact_format(
        mut self,
        format: ArtifactFormat,
    ) -> Self {

        self.required_artifact_format = Some(format);

        self
    }

    /// Returns required execution interfaces.
    #[must_use]
    pub fn execution_interfaces(
        &self,
    ) -> &BTreeSet<ExecutionInterfaceId> {

        &self.required_execution_interfaces
    }

    /// Returns required host facilities.
    #[must_use]
    pub fn host_facilities(
        &self,
    ) -> &BTreeSet<HostFacilityId> {

        &self.required_host_facilities
    }

    /// Returns mandatory capability requirements.
    #[must_use]
    pub fn capabilities(
        &self,
    ) -> &[CapabilityRequirement] {

        &self.required_capabilities
    }

    /// Returns the required artifact format.
    #[must_use]
    pub fn artifact_format(
        &self,
    ) -> Option<&ArtifactFormat> {

        self.required_artifact_format.as_ref()
    }
}

// =============================================================================
// COMPATIBILITY REPORT
// =============================================================================

/// Complete immutable platform compatibility report.
///
/// The report contains evidence-based findings.
///
/// It is not a resource reservation, feasibility certificate,
/// provider execution receipt or successful execution result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformCompatibilityReport {

    target_id: String,

    host_platform_id: String,

    status: PlatformCompatibilityStatus,

    findings: Vec<CompatibilityFinding>,

    capability_results: Option<CapabilityCheckResult>,
}

impl PlatformCompatibilityReport {

    fn new(
        target_id: String,
        host_platform_id: String,
        findings: Vec<CompatibilityFinding>,
        capability_results: Option<CapabilityCheckResult>,
    ) -> Self {

        let has_failure = findings.iter().any(|finding| {
            finding.status() == CompatibilityFindingStatus::Failed
        });

        let has_unknown = findings.iter().any(|finding| {
            finding.status() == CompatibilityFindingStatus::Unknown
        });

        let capability_failure = capability_results
            .as_ref()
            .is_some_and(|results| !results.is_compatible());

        let status = if has_failure || capability_failure {
            PlatformCompatibilityStatus::Incompatible
        } else if has_unknown {
            PlatformCompatibilityStatus::Indeterminate
        } else {
            PlatformCompatibilityStatus::Compatible
        };

        Self {
            target_id,
            host_platform_id,
            status,
            findings,
            capability_results,
        }
    }

    /// Returns the target identity.
    #[must_use]
    pub fn target_id(&self) -> &str {
        &self.target_id
    }

    /// Returns the host platform identity.
    #[must_use]
    pub fn host_platform_id(&self) -> &str {
        &self.host_platform_id
    }

    /// Returns the aggregate compatibility status.
    #[must_use]
    pub const fn status(&self) -> PlatformCompatibilityStatus {
        self.status
    }

    /// Returns whether compatibility has been established.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        self.status.is_compatible()
    }

    /// Returns all compatibility findings.
    #[must_use]
    pub fn findings(&self) -> &[CompatibilityFinding] {
        &self.findings
    }

    /// Returns the backend capability matching result, if evaluated.
    #[must_use]
    pub fn capability_results(
        &self,
    ) -> Option<&CapabilityCheckResult> {
        self.capability_results.as_ref()
    }

    /// Returns findings that failed.
    pub fn failures(
        &self,
    ) -> impl Iterator<Item = &CompatibilityFinding> {

        self.findings.iter().filter(|finding| {
            finding.status() == CompatibilityFindingStatus::Failed
        })
    }

    /// Returns findings that require additional evidence.
    pub fn unknowns(
        &self,
    ) -> impl Iterator<Item = &CompatibilityFinding> {

        self.findings.iter().filter(|finding| {
            finding.status() == CompatibilityFindingStatus::Unknown
        })
    }
}

// =============================================================================
// COMPATIBILITY EVALUATOR
// =============================================================================

/// Evaluates compatibility between an existing backend target and
/// an explicitly supplied host environment.
///
/// This evaluator does not discover or select a target.
///
/// All platform information must be supplied by the caller.
pub struct PlatformCompatibilityEvaluator {

    policy: PlatformCompatibilityPolicy,
}

impl PlatformCompatibilityEvaluator {

    /// Constructs an evaluator with the conservative default policy.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            policy: PlatformCompatibilityPolicy::new(),
        }
    }

    /// Constructs an evaluator with an explicit policy.
    #[must_use]
    pub const fn with_policy(
        policy: PlatformCompatibilityPolicy,
    ) -> Self {

        Self { policy }
    }

    /// Returns the active policy.
    #[must_use]
    pub const fn policy(&self) -> &PlatformCompatibilityPolicy {
        &self.policy
    }

    /// Evaluates one target against one supplied host environment.
    ///
    /// The backend TargetDescriptor and its CapabilityProfile remain
    /// authoritative.
    ///
    /// Missing platform information is reported as Unknown.
    ///
    /// Contradictory information is reported as Failed.
    ///
    /// # Errors
    ///
    /// Returns an error when the supplied target descriptor is invalid.
    pub fn evaluate(
        &self,
        target: &TargetDescriptor,
        host: &HostEnvironment,
        requirements: &PlatformCompatibilityRequirements,
    ) -> ZurResult<PlatformCompatibilityReport> {

        target.validate().map_err(|error| {
            compatibility_error(
                format!(
                    "target descriptor validation failed: {error}"
                ),
            )
        })?;

        let mut findings = Vec::new();

        self.check_target_implementation(
            target,
            &mut findings,
        );

        self.compare_optional(
            CompatibilityCheckKind::Architecture,
            "architecture",
            target.architecture(),
            host.platform().architecture(),
            &mut findings,
        );

        self.compare_optional(
            CompatibilityCheckKind::OperatingSystem,
            "operating-system",
            target.operating_system(),
            host.platform().operating_system(),
            &mut findings,
        );

        self.compare_optional(
            CompatibilityCheckKind::TargetTriple,
            "target-triple",
            target.triple().map(|value| value.as_str()),
            host.platform().target_triple(),
            &mut findings,
        );

        self.compare_optional(
            CompatibilityCheckKind::Abi,
            "abi",
            target.abi().and_then(|value| value.name()),
            host.platform().abi(),
            &mut findings,
        );

        self.compare_optional(
            CompatibilityCheckKind::AbiVersion,
            "abi-version",
            target.abi().and_then(|value| value.version()),
            host.platform().abi_version(),
            &mut findings,
        );

        self.check_artifact_format(
            target,
            requirements,
            &mut findings,
        );

        self.check_execution_interfaces(
            host,
            requirements,
            &mut findings,
        );

        self.check_host_facilities(
            host,
            requirements,
            &mut findings,
        );

        let capability_results =
            self.check_capabilities(
                target.capability_profile(),
                requirements,
            );

        if let Some(results) = &capability_results {
            for result in results.results() {

                let status = match result.outcome() {
                    CapabilityOutcome::Satisfied =>
                        CompatibilityFindingStatus::Satisfied,

                    CapabilityOutcome::InsufficientEvidence =>
                        CompatibilityFindingStatus::Unknown,

                    _ =>
                        CompatibilityFindingStatus::Failed,
                };

                findings.push(
                    CompatibilityFinding::new(
                        CompatibilityCheckKind::Capability,
                        result.id().to_string(),
                        status,
                        format!(
                            "Backend capability matching returned {:?}",
                            result.outcome()
                        ),
                        None,
                    ),
                );
            }
        }

        Ok(
            PlatformCompatibilityReport::new(
                target.id().to_string(),
                host.platform().id().to_string(),
                findings,
                capability_results,
            )
        )
    }

    // -------------------------------------------------------------------------
    // TARGET IMPLEMENTATION
    // -------------------------------------------------------------------------

    fn check_target_implementation(
        &self,
        target: &TargetDescriptor,
        findings: &mut Vec<CompatibilityFinding>,
    ) {

        let (status, explanation) = match target.availability() {

            ImplementationAvailability::Unavailable => (
                CompatibilityFindingStatus::Failed,
                "The target implementation is explicitly unavailable.",
            ),

            ImplementationAvailability::DescriptorOnly
                if !self.policy.allow_descriptor_only_targets => (
                CompatibilityFindingStatus::Unknown,
                "Only target metadata is available; implementation support \
                 has not been established.",
            ),

            ImplementationAvailability::DescriptorOnly => (
                CompatibilityFindingStatus::Unknown,
                "Descriptor-only targets do not establish executable support.",
            ),

            ImplementationAvailability::LegacyEmitterPresent => (
                CompatibilityFindingStatus::Satisfied,
                "The target declares an existing emitter path; this is \
                 not independent proof of runtime execution.",
            ),
        };

        findings.push(
            CompatibilityFinding::new(
                CompatibilityCheckKind::TargetImplementation,
                target.id().to_string(),
                status,
                explanation,
                None,
            ),
        );
    }

    // -------------------------------------------------------------------------
    // OPTIONAL PLATFORM PROPERTY COMPARISON
    // -------------------------------------------------------------------------

    fn compare_optional(
        &self,
        kind: CompatibilityCheckKind,
        subject: &str,
        target_value: Option<&str>,
        host_value: Option<&str>,
        findings: &mut Vec<CompatibilityFinding>,
    ) {

        let (status, explanation) = match (
            target_value,
            host_value,
        ) {

            (None, _) => (
                CompatibilityFindingStatus::NotApplicable,
                "The target does not specify this property.",
            ),

            (Some(_), None) => (
                CompatibilityFindingStatus::Unknown,
                "The host has not supplied evidence for this property.",
            ),

            (Some(expected), Some(actual))
                if expected == actual => (
                CompatibilityFindingStatus::Satisfied,
                "The target and host properties match.",
            ),

            (Some(expected), Some(actual)) => (
                CompatibilityFindingStatus::Failed,
                Box::leak(
                    format!(
                        "Target requires `{expected}`, but host reports \
                         `{actual}`."
                    )
                    .into_boxed_str()
                ),
            ),
        };

        findings.push(
            CompatibilityFinding::new(
                kind,
                subject,
                status,
                explanation,
                None,
            ),
        );
    }

    // -------------------------------------------------------------------------
    // ARTIFACT FORMAT
    // -------------------------------------------------------------------------

    fn check_artifact_format(
        &self,
        target: &TargetDescriptor,
        requirements: &PlatformCompatibilityRequirements,
        findings: &mut Vec<CompatibilityFinding>,
    ) {

        let Some(required) = requirements.artifact_format() else {
            return;
        };

        let available = target.artifact_formats()
            .contains(required);

        findings.push(
            CompatibilityFinding::new(
                CompatibilityCheckKind::ArtifactFormat,
                required.to_string(),
                if available {
                    CompatibilityFindingStatus::Satisfied
                } else {
                    CompatibilityFindingStatus::Failed
                },
                if available {
                    "The target declares the required artifact format."
                } else {
                    "The target does not declare the required artifact format."
                },
                None,
            ),
        );
    }

    // -------------------------------------------------------------------------
    // EXECUTION INTERFACES
    // -------------------------------------------------------------------------

    fn check_execution_interfaces(
        &self,
        host: &HostEnvironment,
        requirements: &PlatformCompatibilityRequirements,
        findings: &mut Vec<CompatibilityFinding>,
    ) {

        for required in requirements.execution_interfaces() {

            let interface = host.execution_interface(required);

            let (status, evidence, explanation) = match interface {

                None => (
                    if self.policy.require_execution_interface_evidence {
                        CompatibilityFindingStatus::Unknown
                    } else {
                        CompatibilityFindingStatus::NotApplicable
                    },
                    None,
                    "No host execution-interface evidence was supplied.",
                ),

                Some(interface) => match interface.availability() {

                    HostAvailability::Available => (
                        CompatibilityFindingStatus::Satisfied,
                        interface.evidence_reference().map(str::to_owned),
                        "The host reports this execution interface as available.",
                    ),

                    HostAvailability::Unavailable => (
                        CompatibilityFindingStatus::Failed,
                        interface.evidence_reference().map(str::to_owned),
                        "The host reports this execution interface as unavailable.",
                    ),

                    HostAvailability::Unknown => (
                        CompatibilityFindingStatus::Unknown,
                        interface.evidence_reference().map(str::to_owned),
                        "The host has not established execution-interface availability.",
                    ),
                },
            };

            findings.push(
                CompatibilityFinding::new(
                    CompatibilityCheckKind::ExecutionInterface,
                    required.to_string(),
                    status,
                    explanation,
                    evidence,
                ),
            );
        }
    }

    // -------------------------------------------------------------------------
    // HOST FACILITIES
    // -------------------------------------------------------------------------

    fn check_host_facilities(
        &self,
        host: &HostEnvironment,
        requirements: &PlatformCompatibilityRequirements,
        findings: &mut Vec<CompatibilityFinding>,
    ) {

        for required in requirements.host_facilities() {

            let facility = host.facility(required);

            let (status, evidence, explanation) = match facility {

                None => (
                    if self.policy.require_host_facility_evidence {
                        CompatibilityFindingStatus::Unknown
                    } else {
                        CompatibilityFindingStatus::NotApplicable
                    },
                    None,
                    "No host facility evidence was supplied.",
                ),

                Some(facility) => match facility.availability() {

                    HostAvailability::Available => (
                        CompatibilityFindingStatus::Satisfied,
                        facility.evidence_reference().map(str::to_owned),
                        "The host reports this facility as available.",
                    ),

                    HostAvailability::Unavailable => (
                        CompatibilityFindingStatus::Failed,
                        facility.evidence_reference().map(str::to_owned),
                        "The host reports this facility as unavailable.",
                    ),

                    HostAvailability::Unknown => (
                        CompatibilityFindingStatus::Unknown,
                        facility.evidence_reference().map(str::to_owned),
                        "The host has not established facility availability.",
                    ),
                },
            };

            findings.push(
                CompatibilityFinding::new(
                    CompatibilityCheckKind::HostFacility,
                    required.to_string(),
                    status,
                    explanation,
                    evidence,
                ),
            );
        }
    }

    // -------------------------------------------------------------------------
    // BACKEND CAPABILITY MATCHING
    // -------------------------------------------------------------------------

    fn check_capabilities(
        &self,
        profile: &CapabilityProfile,
        requirements: &PlatformCompatibilityRequirements,
    ) -> Option<CapabilityCheckResult> {

        if requirements.capabilities().is_empty() {
            return None;
        }

        Some(
            profile.check(
                requirements.capabilities(),
                self.policy.capability_policy,
            )
        )
    }
}

impl Default for PlatformCompatibilityEvaluator {

    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    use crate::backend::capabilities::{
        CapabilityDescriptor,
        CapabilityEvidence,
        CapabilityEvidenceKind,
        CapabilityId,
        CapabilityStatus,
    };

    use crate::backend::target::{
        ExecutionModel,
        TargetId,
    };

    use crate::zur::platforms::host::{
        ExecutionInterfaceDescriptor,
        HostPlatformDescriptor,
    };

    fn target() -> TargetDescriptor {

        TargetDescriptor::new(
            TargetId::new("example.native").unwrap(),
            "Example native target",
            ExecutionModel::Native,
        )
        .unwrap()
        .with_architecture("aarch64")
        .unwrap()
        .with_operating_system("linux")
        .unwrap()
        .with_availability(
            ImplementationAvailability::LegacyEmitterPresent,
        )
    }

    fn host() -> HostEnvironment {

        let platform = HostPlatformDescriptor::from_id(
            "example.host",
        )
        .unwrap()
        .with_architecture("aarch64")
        .unwrap()
        .with_operating_system("linux")
        .unwrap();

        HostEnvironment::builder(platform)
            .build()
            .unwrap()
    }

    #[test]
    fn matching_platform_is_compatible() {

        let report = PlatformCompatibilityEvaluator::new()
            .evaluate(
                &target(),
                &host(),
                &PlatformCompatibilityRequirements::new(),
            )
            .unwrap();

        assert!(
            report.is_compatible(),
            "{:?}",
            report.findings()
        );
    }

    #[test]
    fn architecture_mismatch_is_incompatible() {

        let target = target()
            .with_architecture("x86_64")
            .unwrap();

        let report = PlatformCompatibilityEvaluator::new()
            .evaluate(
                &target,
                &host(),
                &PlatformCompatibilityRequirements::new(),
            )
            .unwrap();

        assert_eq!(
            report.status(),
            PlatformCompatibilityStatus::Incompatible,
        );
    }

    #[test]
    fn missing_host_architecture_is_indeterminate() {

        let platform = HostPlatformDescriptor::from_id(
            "example.host",
        )
        .unwrap();

        let host = HostEnvironment::builder(platform)
            .build()
            .unwrap();

        let report = PlatformCompatibilityEvaluator::new()
            .evaluate(
                &target(),
                &host,
                &PlatformCompatibilityRequirements::new(),
            )
            .unwrap();

        assert_eq!(
            report.status(),
            PlatformCompatibilityStatus::Indeterminate,
        );
    }

    #[test]
    fn unavailable_target_is_rejected() {

        let target = target()
            .with_availability(
                ImplementationAvailability::Unavailable,
            );

        let report = PlatformCompatibilityEvaluator::new()
            .evaluate(
                &target,
                &host(),
                &PlatformCompatibilityRequirements::new(),
            )
            .unwrap();

        assert_eq!(
            report.status(),
            PlatformCompatibilityStatus::Incompatible,
        );
    }

    #[test]
    fn unavailable_execution_interface_fails() {

        let interface_id = ExecutionInterfaceId::new(
            "example.execution.native",
        )
        .unwrap();

        let platform = host().platform().clone();

        let host = HostEnvironment::builder(platform)
            .with_execution_interface(
                ExecutionInterfaceDescriptor::new(
                    interface_id.clone(),
                    HostAvailability::Unavailable,
                ),
            )
            .unwrap()
            .build()
            .unwrap();

        let requirements =
            PlatformCompatibilityRequirements::new()
                .requiring_execution_interface(interface_id);

        let report = PlatformCompatibilityEvaluator::new()
            .evaluate(
                &target(),
                &host,
                &requirements,
            )
            .unwrap();

        assert_eq!(
            report.status(),
            PlatformCompatibilityStatus::Incompatible,
        );
    }

    #[test]
    fn unknown_execution_interface_is_not_accepted() {

        let interface_id = ExecutionInterfaceId::new(
            "example.execution.native",
        )
        .unwrap();

        let requirements =
            PlatformCompatibilityRequirements::new()
                .requiring_execution_interface(interface_id);

        let report = PlatformCompatibilityEvaluator::new()
            .evaluate(
                &target(),
                &host(),
                &requirements,
            )
            .unwrap();

        assert_eq!(
            report.status(),
            PlatformCompatibilityStatus::Indeterminate,
        );
    }

    #[test]
    fn missing_required_capability_fails() {

        let requirement =
            CapabilityRequirement::from_name(
                "quantum.measurement",
            )
            .unwrap();

        let requirements =
            PlatformCompatibilityRequirements::new()
                .requiring_capability(requirement);

        let report = PlatformCompatibilityEvaluator::new()
            .evaluate(
                &target(),
                &host(),
                &requirements,
            )
            .unwrap();

        assert_eq!(
            report.status(),
            PlatformCompatibilityStatus::Incompatible,
        );
    }

    #[test]
    fn verified_capability_satisfies_requirement() {

        let mut target = target();

        let descriptor = CapabilityDescriptor::new(
            CapabilityId::new("quantum.measurement").unwrap(),
            CapabilityStatus::Supported,
            "Quantum measurement capability",
        )
        .with_evidence(
            CapabilityEvidence::new(
                CapabilityEvidenceKind::Verified,
                "example.conformance",
                None,
            ),
        );

        target.capability_profile_mut()
            .capabilities_mut()
            .insert(descriptor)
            .unwrap();

        let requirements =
            PlatformCompatibilityRequirements::new()
                .requiring_capability(
                    CapabilityRequirement::from_name(
                        "quantum.measurement",
                    )
                    .unwrap(),
                );

        let report = PlatformCompatibilityEvaluator::new()
            .evaluate(
                &target,
                &host(),
                &requirements,
            )
            .unwrap();

        assert_eq!(
            report.status(),
            PlatformCompatibilityStatus::Compatible,
        );
    }

    #[test]
    fn future_capability_names_are_supported() {

        let requirement =
            CapabilityRequirement::from_name(
                "future.photonics.optical_execution",
            )
            .unwrap();

        let requirements =
            PlatformCompatibilityRequirements::new()
                .requiring_capability(requirement);

        let report = PlatformCompatibilityEvaluator::new()
            .evaluate(
                &target(),
                &host(),
                &requirements,
            )
            .unwrap();

        assert_eq!(
            report.status(),
            PlatformCompatibilityStatus::Incompatible,
        );
    }
}
