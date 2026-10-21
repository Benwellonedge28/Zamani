
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! HDL target compatibility for Zamani.
//!
//! Path: `src/backend/hdl/compatibility.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97.
//!
//! # Responsibility
//!
//! Assess whether a declared HDL compilation contract is compatible with
//! the evidence reported by a backend target. This module does not parse
//! Zamani source, perform semantic analysis, construct HDLIR, allocate
//! hardware, synthesize a design, execute a simulator, or prove timing
//! closure.
//!
//! # Authority
//!
//! The normative authorities are:
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/spec/hdl.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/hdl/mod.rs`
//! - `src/hdl/targets/`
//! - `src/hdl/timing/`
//! - `src/hdl/verification/`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//!
//! # Integration contract
//!
//! The shared `crate::backend::compatibility` module owns generic capability,
//! resource, requirement, and compatibility-report semantics. This module
//! specializes those contracts for HDL. The HDL frontend and HDLIR remain
//! authoritative for design meaning and representation invariants.
//!
//! Callers must verify the source/IR using its authoritative verifier before
//! treating a successful target-compatibility report as permission to emit.
//! A compatibility report is not a synthesis, simulation, timing-closure,
//! bitstream-generation, or physical-manufacturing certificate.
//!
//! # Scalability
//!
//! This module defines no universal limit for modules, ports, signals, clock
//! domains, memory capacity, timing constraints, or design size. Operational
//! budgets and actual target limits belong to their respective contracts.

use std::collections::BTreeSet;
use std::fmt;

use crate::backend::compatibility::{
    CapabilityId,
    CapabilityRequirement,
    CompatibilityEvaluator,
    CompatibilityIssue,
    CompatibilityIssueCode,
    CompatibilityPolicy,
    CompatibilityReport,
    CompatibilityRequest,
    CompatibilitySeverity,
    CompatibilityStatus,
    RequirementStrength,
    ResourceRequirement,
    TargetCapabilityProfile,
};

/// Error returned when an HDL compatibility request is malformed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HdlCompatibilityError {
    /// A required identifier or descriptive value is empty.
    EmptyValue {
        /// Name of the field that is empty.
        field: &'static str,
    },

    /// A required capability identifier is invalid.
    InvalidCapability {
        /// Capability identifier supplied by the caller.
        value: String,
        /// Description of the underlying validation error.
        reason: String,
    },

    /// The same named contract item was declared more than once.
    DuplicateItem {
        /// Category of the duplicate item.
        kind: &'static str,
        /// Item identifier.
        name: String,
    },
}

impl fmt::Display for HdlCompatibilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyValue { field } => {
                write!(f, "HDL compatibility field `{field}` cannot be empty")
            }
            Self::InvalidCapability { value, reason } => {
                write!(
                    f,
                    "invalid HDL capability `{value}`: {reason}"
                )
            }
            Self::DuplicateItem { kind, name } => {
                write!(f, "duplicate HDL {kind} `{name}`")
            }
        }
    }
}

impl std::error::Error for HdlCompatibilityError {}

/// A capability required from an HDL target.
///
/// Identifiers are extensible. Examples include `hdl.combinational`,
/// `hdl.sequential`, `hdl.synthesis`, `hdl.simulation`,
/// `hdl.clock_domains`, `hdl.reset.synchronous`, `hdl.reset.asynchronous`,
/// `hdl.timing.constraints`, `hdl.formal_verification`, and
/// `hdl.physical.implementation`.
///
/// These examples are not a closed list. A capability must be declared by
/// the target adapter based on real implementation or toolchain evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdlCapabilityRequirement {
    /// Extensible canonical capability identifier.
    pub capability: CapabilityId,

    /// Whether the feature is mandatory or preferred.
    pub strength: RequirementStrength,

    /// Optional explanation of why the design needs the capability.
    pub rationale: Option<String>,
}

impl HdlCapabilityRequirement {
    /// Construct a capability requirement from an existing validated ID.
    pub fn new(
        capability: CapabilityId,
        strength: RequirementStrength,
    ) -> Self {
        Self {
            capability,
            strength,
            rationale: None,
        }
    }

    /// Mark a capability as mandatory.
    pub fn required(capability: CapabilityId) -> Self {
        Self::new(capability, RequirementStrength::Required)
    }

    /// Attach a human-readable explanation.
    pub fn with_rationale(mut self, rationale: impl Into<String>) -> Self {
        self.rationale = Some(rationale.into());
        self
    }
}

/// Kind of reset behavior required by a design.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResetKind {
    /// No reset behavior is required by this contract.
    None,

    /// Reset is asserted and sampled synchronously with a clock.
    Synchronous,

    /// Reset assertion or behavior is asynchronous by contract.
    Asynchronous,

    /// The design's reset contract supports multiple explicit reset styles.
    Mixed,
}

/// Evidence status for a property that requires validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EvidenceStatus {
    /// A responsible validator or tool has confirmed the property.
    Verified,

    /// The property has been checked and does not hold.
    Rejected,

    /// No authoritative evidence is available.
    Unknown,

    /// The property is partially checked or experimental.
    Partial,
}

/// A target-specific validation result.
///
/// The result must be supplied by the relevant verifier, simulator,
/// synthesis tool, timing engine, or trusted target adapter. It must not be
/// inferred solely from a target name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdlEvidence {
    /// Stable property identifier, such as `hdl.synthesis` or
    /// `hdl.timing.closure`.
    pub property: String,

    /// Validation outcome.
    pub status: EvidenceStatus,

    /// Optional provenance identifying the validator or toolchain.
    pub source: Option<String>,

    /// Optional explanation of limitations or failure.
    pub detail: Option<String>,
}

impl HdlEvidence {
    /// Construct a validation result.
    pub fn new(
        property: impl Into<String>,
        status: EvidenceStatus,
    ) -> Self {
        Self {
            property: property.into(),
            status,
            source: None,
            detail: None,
        }
    }

    /// Attach provenance.
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Attach explanatory detail.
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

/// Design-level HDL requirements.
///
/// These describe what a design needs. They do not claim that a target
/// implements those features.
///
/// The design's source-level semantics and detailed timing expressions
/// remain owned by the canonical semantic model and HDL subsystems.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HdlDesignRequirements {
    /// Whether sequential clocked behavior is required.
    pub requires_clock: bool,

    /// Whether reset behavior is required.
    pub reset: ResetKind,

    /// Whether the selected output must be synthesizable.
    pub requires_synthesis: bool,

    /// Whether simulation support is mandatory.
    pub requires_simulation: bool,

    /// Whether timing constraints must be checked by an appropriate tool.
    ///
    /// This requests timing analysis; it does not assert that timing closure
    /// has already been achieved.
    pub requires_timing_analysis: bool,

    /// Whether the request requires an actual timing-closure result.
    ///
    /// This is stricter than merely requiring timing-analysis capability.
    pub requires_timing_closure: bool,

    /// Whether formal verification is mandatory.
    pub requires_formal_verification: bool,

    /// Additional extensible capabilities required by the design.
    pub capabilities: Vec<HdlCapabilityRequirement>,

    /// Resource requirements inherited from the verified design contract.
    pub resources: Vec<ResourceRequirement>,
}

impl HdlDesignRequirements {
    /// Create an empty requirements contract.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an additional extensible capability requirement.
    pub fn require_capability(
        &mut self,
        requirement: HdlCapabilityRequirement,
    ) {
        self.capabilities.push(requirement);
    }

    /// Add a resource requirement from the shared compatibility model.
    pub fn require_resource(&mut self, requirement: ResourceRequirement) {
        self.resources.push(requirement);
    }
}

/// Evidence and requirements for one HDL target assessment.
///
/// `target` must describe the requested compilation environment, not merely
/// the physical machine on which the compiler happens to run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdlCompatibilityRequest {
    /// Required HDL features and resource contracts.
    pub requirements: HdlDesignRequirements,

    /// Explicit target capability and resource evidence.
    pub target: TargetCapabilityProfile,

    /// Additional evidence from authoritative validation stages.
    pub evidence: Vec<HdlEvidence>,

    /// Shared compatibility policy.
    pub policy: CompatibilityPolicy,
}

impl HdlCompatibilityRequest {
    /// Construct an assessment request.
    pub fn new(
        requirements: HdlDesignRequirements,
        target: TargetCapabilityProfile,
    ) -> Self {
        Self {
            requirements,
            target,
            evidence: Vec::new(),
            policy: CompatibilityPolicy::default(),
        }
    }

    /// Configure shared capability policy.
    pub fn with_policy(mut self, policy: CompatibilityPolicy) -> Self {
        self.policy = policy;
        self
    }

    /// Add a validation result.
    pub fn with_evidence(mut self, evidence: HdlEvidence) -> Self {
        self.evidence.push(evidence);
        self
    }
}

/// Stable HDL-specific issue code.
///
/// Generic capability and resource failures retain their shared issue codes.
/// These codes identify additional HDL contract checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HdlIssueCode {
    /// A required clocking capability is not established.
    ClockCapabilityMissing,

    /// A required reset style is not established.
    ResetCapabilityMissing,

    /// Synthesis is required but not established.
    SynthesisNotEstablished,

    /// Simulation is required but not established.
    SimulationNotEstablished,

    /// Timing analysis is required but not established.
    TimingAnalysisNotEstablished,

    /// Timing closure is required but not established.
    TimingClosureNotEstablished,

    /// Formal verification is required but not established.
    FormalVerificationNotEstablished,

    /// A supplied validation explicitly rejected a required property.
    EvidenceRejected,

    /// A supplied validation has only partial or unknown status.
    EvidenceInsufficient,

    /// The evidence property identifier is invalid.
    InvalidEvidence,
}

/// One HDL-specific compatibility finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdlCompatibilityIssue {
    /// Stable machine-readable issue code.
    pub code: HdlIssueCode,

    /// Severity of the finding.
    pub severity: CompatibilitySeverity,

    /// Canonical subject or property name.
    pub subject: String,

    /// Explanation intended for diagnostics and logs.
    pub message: String,

    /// Optional validation provenance.
    pub evidence: Option<String>,
}

/// Overall HDL compatibility status.
///
/// `Compatible` means the requested compatibility checks passed based on the
/// available declared evidence. It does not prove that emitted HDL is valid,
/// synthesizable, physically realizable, or timing-closed unless those
/// properties were separately established by the corresponding validators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HdlCompatibilityStatus {
    /// All mandatory compatibility checks passed without warnings.
    Compatible,

    /// Mandatory checks passed, but there are relevant warnings.
    CompatibleWithWarnings,

    /// Required properties cannot be established from available evidence.
    Indeterminate,

    /// Evidence explicitly contradicts at least one mandatory requirement.
    Incompatible,
}

/// Complete HDL compatibility assessment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdlCompatibilityReport {
    /// Target identifier from the shared compatibility profile.
    pub target_id: String,

    /// HDL-specific overall status.
    pub status: HdlCompatibilityStatus,

    /// Findings produced by the generic compatibility evaluator.
    pub generic: CompatibilityReport,

    /// Additional HDL-specific findings in deterministic order.
    pub issues: Vec<HdlCompatibilityIssue>,
}

impl HdlCompatibilityReport {
    /// Whether all mandatory compatibility requirements have been met.
    pub fn is_compatible(&self) -> bool {
        matches!(
            self.status,
            HdlCompatibilityStatus::Compatible
                | HdlCompatibilityStatus::CompatibleWithWarnings
        )
    }

    /// Whether the assessment has no warnings or errors.
    pub fn is_fully_compatible(&self) -> bool {
        self.status == HdlCompatibilityStatus::Compatible
    }

    /// Iterate over HDL-specific errors.
    pub fn errors(&self) -> impl Iterator<Item = &HdlCompatibilityIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.severity == CompatibilitySeverity::Error)
    }

    /// Iterate over HDL-specific warnings.
    pub fn warnings(&self) -> impl Iterator<Item = &HdlCompatibilityIssue> {
        self.issues
            .iter()
            .filter(|issue| {
                issue.severity == CompatibilitySeverity::Warning
            })
    }
}

/// Stateless HDL compatibility evaluator.
///
/// This evaluator is deterministic and has no filesystem, process, hardware,
/// or global-state side effects.
#[derive(Debug, Clone, Copy, Default)]
pub struct HdlCompatibilityEvaluator;

impl HdlCompatibilityEvaluator {
    /// Evaluate generic target requirements and HDL-specific requirements.
    ///
    /// A missing or unknown mandatory capability is not accepted as proven
    /// support. A successful report is a compatibility assessment only;
    /// code generation and downstream validation remain separate stages.
    pub fn evaluate(
        &self,
        request: &HdlCompatibilityRequest,
    ) -> HdlCompatibilityReport {
        let generic_request = self.generic_request(request);
        let generic = CompatibilityEvaluator.evaluate(
            &generic_request,
            &request.target,
        );

        let mut issues = Vec::new();

        self.check_named_requirement(
            request,
            "hdl.clock_domains",
            request.requirements.requires_clock,
            HdlIssueCode::ClockCapabilityMissing,
            "clocked behavior is required, but target support for HDL clock domains is not established",
            &mut issues,
        );

        let reset_capability = match request.requirements.reset {
            ResetKind::None => None,
            ResetKind::Synchronous => Some("hdl.reset.synchronous"),
            ResetKind::Asynchronous => Some("hdl.reset.asynchronous"),
            ResetKind::Mixed => Some("hdl.reset.multiple_styles"),
        };

        if let Some(capability) = reset_capability {
            self.check_named_requirement(
                request,
                capability,
                true,
                HdlIssueCode::ResetCapabilityMissing,
                "the required reset behavior is not established for this target",
                &mut issues,
            );
        }

        self.check_evidence_requirement(
            request,
            "hdl.synthesis",
            request.requirements.requires_synthesis,
            HdlIssueCode::SynthesisNotEstablished,
            "synthesis is required, but no successful synthesis validation is established",
            &mut issues,
        );

        self.check_named_requirement(
            request,
            "hdl.simulation",
            request.requirements.requires_simulation,
            HdlIssueCode::SimulationNotEstablished,
            "simulation support is required, but target support is not established",
            &mut issues,
        );

        self.check_named_requirement(
            request,
            "hdl.timing.constraints",
            request.requirements.requires_timing_analysis
                || request.requirements.requires_timing_closure,
            HdlIssueCode::TimingAnalysisNotEstablished,
            "timing analysis is required, but the target does not establish the required timing-analysis capability",
            &mut issues,
        );

        self.check_evidence_requirement(
            request,
            "hdl.timing.closure",
            request.requirements.requires_timing_closure,
            HdlIssueCode::TimingClosureNotEstablished,
            "timing closure is required, but no successful timing-closure result is established",
            &mut issues,
        );

        self.check_named_requirement(
            request,
            "hdl.formal_verification",
            request.requirements.requires_formal_verification,
            HdlIssueCode::FormalVerificationNotEstablished,
            "formal verification is required, but target support is not established",
            &mut issues,
        );

        self.validate_evidence(request, &mut issues);

        issues.sort_by(|left, right| {
            left.subject
                .cmp(&right.subject)
                .then_with(|| left.code.cmp(&right.code))
                .then_with(|| left.severity.cmp(&right.severity))
                .then_with(|| left.message.cmp(&right.message))
        });

        let status = classify_hdl_status(&generic, &issues);

        HdlCompatibilityReport {
            target_id: request.target.target_id.clone(),
            status,
            generic,
            issues,
        }
    }

    fn generic_request(
        &self,
        request: &HdlCompatibilityRequest,
    ) -> CompatibilityRequest {
        let mut generic = CompatibilityRequest::new()
            .with_policy(request.policy);

        for requirement in &request.requirements.capabilities {
            generic.require_capability(CapabilityRequirement::new(
                requirement.capability.clone(),
                requirement.strength,
            ));
        }

        for requirement in &request.requirements.resources {
            generic.require_resource(requirement.clone());
        }

        generic
    }

    fn check_named_requirement(
        &self,
        request: &HdlCompatibilityRequest,
        capability_name: &str,
        required: bool,
        code: HdlIssueCode,
        message: &str,
        issues: &mut Vec<HdlCompatibilityIssue>,
    ) {
        if !required {
            return;
        }

        let capability = match CapabilityId::new(capability_name) {
            Ok(capability) => capability,
            Err(error) => {
                issues.push(HdlCompatibilityIssue {
                    code: HdlIssueCode::InvalidEvidence,
                    severity: CompatibilitySeverity::Error,
                    subject: capability_name.to_owned(),
                    message: error.to_string(),
                    evidence: None,
                });
                return;
            }
        };

        let support = request.target.capability(&capability);

        let established = support.is_some_and(|support| {
            support.level.is_fully_supported()
        });

        if established {
            return;
        }

        let (severity, detail) = match support {
            Some(support) => (
                CompatibilitySeverity::Error,
                format!("reported support level is {:?}", support.level),
            ),
            None => (
                CompatibilitySeverity::Error,
                "no explicit target capability declaration exists".to_owned(),
            ),
        };

        issues.push(HdlCompatibilityIssue {
            code,
            severity,
            subject: capability_name.to_owned(),
            message: format!("{message}: {detail}"),
            evidence: support.and_then(|support| support.evidence.clone()),
        });
    }

    fn check_evidence_requirement(
        &self,
        request: &HdlCompatibilityRequest,
        property: &str,
        required: bool,
        code: HdlIssueCode,
        message: &str,
        issues: &mut Vec<HdlCompatibilityIssue>,
    ) {
        if !required {
            return;
        }

        match request.evidence.iter().find(|item| item.property == property) {
            Some(evidence) if evidence.status == EvidenceStatus::Verified => {}
            Some(evidence) if evidence.status == EvidenceStatus::Rejected => {
                issues.push(HdlCompatibilityIssue {
                    code: HdlIssueCode::EvidenceRejected,
                    severity: CompatibilitySeverity::Error,
                    subject: property.to_owned(),
                    message: format!(
                        "{message}: validator explicitly rejected the property"
                    ),
                    evidence: evidence
                        .detail
                        .clone()
                        .or_else(|| evidence.source.clone()),
                });
            }
            Some(evidence) => {
                issues.push(HdlCompatibilityIssue {
                    code,
                    severity: CompatibilitySeverity::Error,
                    subject: property.to_owned(),
                    message: format!(
                        "{message}: evidence status is {:?}",
                        evidence.status
                    ),
                    evidence: evidence
                        .detail
                        .clone()
                        .or_else(|| evidence.source.clone()),
                });
            }
            None => issues.push(HdlCompatibilityIssue {
                code,
                severity: CompatibilitySeverity::Error,
                subject: property.to_owned(),
                message: format!("{message}: validation evidence is absent"),
                evidence: None,
            }),
        }
    }

    fn validate_evidence(
        &self,
        request: &HdlCompatibilityRequest,
        issues: &mut Vec<HdlCompatibilityIssue>,
    ) {
        let mut seen = BTreeSet::new();

        for evidence in &request.evidence {
            let property = evidence.property.trim();

            if property.is_empty() {
                issues.push(HdlCompatibilityIssue {
                    code: HdlIssueCode::InvalidEvidence,
                    severity: CompatibilitySeverity::Error,
                    subject: "<empty>".to_owned(),
                    message: "HDL validation evidence has an empty property identifier".to_owned(),
                    evidence: evidence.detail.clone(),
                });
                continue;
            }

            if !seen.insert(property.to_owned()) {
                issues.push(HdlCompatibilityIssue {
                    code: HdlIssueCode::InvalidEvidence,
                    severity: CompatibilitySeverity::Error,
                    subject: property.to_owned(),
                    message: "duplicate HDL evidence property makes the assessment ambiguous".to_owned(),
                    evidence: evidence.source.clone(),
                });
            }
        }
    }
}

fn classify_hdl_status(
    generic: &CompatibilityReport,
    issues: &[HdlCompatibilityIssue],
) -> HdlCompatibilityStatus {
    if generic.status == CompatibilityStatus::Incompatible
        || issues
            .iter()
            .any(|issue| issue.severity == CompatibilitySeverity::Error)
    {
        return HdlCompatibilityStatus::Incompatible;
    }

    if generic.status == CompatibilityStatus::Indeterminate {
        return HdlCompatibilityStatus::Indeterminate;
    }

    if issues
        .iter()
        .any(|issue| issue.severity == CompatibilitySeverity::Warning)
        || generic.status == CompatibilityStatus::CompatibleWithWarnings
    {
        return HdlCompatibilityStatus::CompatibleWithWarnings;
    }

    HdlCompatibilityStatus::Compatible
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::compatibility::{
        CapabilitySupport,
        DecimalInteger,
        ResourceId,
        SupportLevel,
    };

    fn capability(value: &str) -> CapabilityId {
        CapabilityId::new(value).expect("valid capability")
    }

    fn target() -> TargetCapabilityProfile {
        TargetCapabilityProfile::new("test.hdl.target")
            .expect("valid target")
    }

    fn declare_supported(
        profile: &mut TargetCapabilityProfile,
        name: &str,
    ) {
        profile
            .declare_capability(CapabilitySupport::new(
                capability(name),
                SupportLevel::Supported,
            ))
            .expect("unique capability");
    }

    #[test]
    fn empty_design_is_compatible_with_an_empty_partial_profile() {
        let request = HdlCompatibilityRequest::new(
            HdlDesignRequirements::new(),
            target(),
        );

        let report = HdlCompatibilityEvaluator.evaluate(&request);

        assert_eq!(report.status, HdlCompatibilityStatus::Compatible);
        assert!(report.is_compatible());
    }

    #[test]
    fn required_clock_is_not_inferred_from_target_name() {
        let mut requirements = HdlDesignRequirements::new();
        requirements.requires_clock = true;

        let request =
            HdlCompatibilityRequest::new(requirements, target());

        let report = HdlCompatibilityEvaluator.evaluate(&request);

        assert_eq!(report.status, HdlCompatibilityStatus::Incompatible);
        assert!(report.issues.iter().any(|issue| {
            issue.code == HdlIssueCode::ClockCapabilityMissing
        }));
    }

    #[test]
    fn explicit_clock_capability_satisfies_clock_requirement() {
        let mut profile = target();
        declare_supported(&mut profile, "hdl.clock_domains");

        let mut requirements = HdlDesignRequirements::new();
        requirements.requires_clock = true;

        let request =
            HdlCompatibilityRequest::new(requirements, profile);

        let report = HdlCompatibilityEvaluator.evaluate(&request);

        assert!(report.is_compatible());
        assert!(!report.issues.iter().any(|issue| {
            issue.code == HdlIssueCode::ClockCapabilityMissing
        }));
    }

    #[test]
    fn reset_styles_are_not_interchangeable() {
        let mut profile = target();
        declare_supported(&mut profile, "hdl.reset.synchronous");

        let mut requirements = HdlDesignRequirements::new();
        requirements.reset = ResetKind::Asynchronous;

        let request =
            HdlCompatibilityRequest::new(requirements, profile);

        let report = HdlCompatibilityEvaluator.evaluate(&request);

        assert_eq!(report.status, HdlCompatibilityStatus::Incompatible);
        assert!(report.issues.iter().any(|issue| {
            issue.code == HdlIssueCode::ResetCapabilityMissing
        }));
    }

    #[test]
    fn synthesis_requires_successful_validation_evidence() {
        let mut requirements = HdlDesignRequirements::new();
        requirements.requires_synthesis = true;

        let request =
            HdlCompatibilityRequest::new(requirements.clone(), target());

        let report = HdlCompatibilityEvaluator.evaluate(&request);
        assert_eq!(report.status, HdlCompatibilityStatus::Incompatible);

        let request = HdlCompatibilityRequest::new(requirements, target())
            .with_evidence(
                HdlEvidence::new("hdl.synthesis", EvidenceStatus::Verified)
                    .with_source("validated-synthesis-tool"),
            );

        let report = HdlCompatibilityEvaluator.evaluate(&request);
        assert!(report.is_compatible());
    }

    #[test]
    fn timing_analysis_does_not_claim_timing_closure() {
        let mut profile = target();
        declare_supported(&mut profile, "hdl.timing.constraints");

        let mut requirements = HdlDesignRequirements::new();
        requirements.requires_timing_analysis = true;
        requirements.requires_timing_closure = true;

        let request =
            HdlCompatibilityRequest::new(requirements, profile);

        let report = HdlCompatibilityEvaluator.evaluate(&request);

        assert_eq!(report.status, HdlCompatibilityStatus::Incompatible);
        assert!(report.issues.iter().any(|issue| {
            issue.code == HdlIssueCode::TimingClosureNotEstablished
        }));
    }

    #[test]
    fn timing_closure_needs_explicit_verified_evidence() {
        let mut profile = target();
        declare_supported(&mut profile, "hdl.timing.constraints");

        let mut requirements = HdlDesignRequirements::new();
        requirements.requires_timing_closure = true;

        let request = HdlCompatibilityRequest::new(requirements, profile)
            .with_evidence(
                HdlEvidence::new(
                    "hdl.timing.closure",
                    EvidenceStatus::Verified,
                )
                .with_source("timing-analysis-tool"),
            );

        let report = HdlCompatibilityEvaluator.evaluate(&request);
        assert!(report.is_compatible());
    }

    #[test]
    fn duplicate_evidence_is_rejected_as_ambiguous() {
        let request = HdlCompatibilityRequest::new(
            HdlDesignRequirements::new(),
            target(),
        )
        .with_evidence(HdlEvidence::new(
            "hdl.synthesis",
            EvidenceStatus::Verified,
        ))
        .with_evidence(HdlEvidence::new(
            "hdl.synthesis",
            EvidenceStatus::Rejected,
        ));

        let report = HdlCompatibilityEvaluator.evaluate(&request);

        assert_eq!(report.status, HdlCompatibilityStatus::Incompatible);
        assert!(report.issues.iter().any(|issue| {
            issue.code == HdlIssueCode::InvalidEvidence
        }));
    }

    #[test]
    fn resource_contracts_use_shared_arbitrary_length_quantities() {
        let mut profile = target();
        profile
            .declare_resource(
                ResourceId::new("hdl.memory.bits").expect("resource ID"),
                crate::backend::compatibility::ResourceAvailability::known(
                    DecimalInteger::new(
                        "999999999999999999999999999999999999",
                    )
                    .expect("arbitrary-length decimal"),
                ),
            )
            .expect("unique resource");

        let requirements = HdlDesignRequirements::new();
        let request = HdlCompatibilityRequest::new(requirements, profile);

        let report = HdlCompatibilityEvaluator.evaluate(&request);

        assert!(report.is_compatible());
    }

    #[test]
    fn evaluation_is_deterministic() {
        let mut profile = target();
        declare_supported(&mut profile, "hdl.clock_domains");

        let mut requirements = HdlDesignRequirements::new();
        requirements.requires_clock = true;
        requirements.requires_simulation = true;

        let request =
            HdlCompatibilityRequest::new(requirements, profile);

        let first = HdlCompatibilityEvaluator.evaluate(&request);
        let second = HdlCompatibilityEvaluator.evaluate(&request);

        assert_eq!(first, second);
    }
}
