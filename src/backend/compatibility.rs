
#![forbid(unsafe_code)]

//! Backend compatibility and feasibility contracts for Zamani.
//!
//! Path: `src/backend/compatibility.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Responsibility
//!
//! This module answers:
//!
//! "Can this target satisfy the declared requirements of this compilation
//! request, based on the capability and resource evidence currently known?"
//!
//! It does not:
//! - define source-language semantics;
//! - verify or rewrite an IR;
//! - choose physical devices or allocate hardware resources;
//! - perform quantum routing, scheduling, calibration, or QEC;
//! - synthesize HDL or generate target code;
//! - launch external tools, link artifacts, or execute programs;
//! - infer unsupported facts from a target's name.
//!
//! Those responsibilities remain with their existing authoritative owners.
//!
//! # Architectural contract
//!
//! - Requirements are distinct from capabilities, preferences, and hints.
//! - Missing information is not automatically proof of incompatibility.
//! - A target that explicitly lacks a hard requirement is incompatible.
//! - Partial support never silently satisfies a hard requirement.
//! - Experimental support requires explicit policy permission for a hard
//!   requirement and remains visible in the resulting report.
//! - Resource quantities are non-negative decimal integers represented as
//!   strings, avoiding a fixed integer-width ceiling in this contract.
//! - Results and diagnostics have deterministic ordering.
//! - Adding a new capability or resource dimension does not require changing
//!   a closed enum.
//!
//! # Integration
//!
//! `selection.rs` should evaluate a request against a target profile before
//! selecting that target. `validation.rs` should use the same report before
//! emission. Each target adapter should report evidence from its actual
//! implementation and toolchain, not from assumptions.
//!
//! `capabilities.rs`, `requirements.rs`, and `target.rs` should build on these
//! shared compatibility primitives rather than introduce competing IDs or
//! compatibility result types.
//!
//! The caller remains responsible for verifying the input representation with
//! its authoritative verifier before backend emission.

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

/// Errors caused by malformed compatibility metadata or requests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatibilityError {
    EmptyIdentifier {
        kind: &'static str,
    },
    InvalidIdentifier {
        kind: &'static str,
        value: String,
    },
    InvalidDecimalInteger {
        value: String,
    },
    DuplicateCapability {
        capability: CapabilityId,
    },
    DuplicateResource {
        resource: ResourceId,
    },
    InvalidTargetIdentifier {
        value: String,
    },
}

impl fmt::Display for CompatibilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { kind } => {
                write!(f, "{kind} identifier cannot be empty")
            }
            Self::InvalidIdentifier { kind, value } => {
                write!(f, "invalid {kind} identifier `{value}`")
            }
            Self::InvalidDecimalInteger { value } => {
                write!(
                    f,
                    "resource quantity `{value}` must be a non-negative decimal integer"
                )
            }
            Self::DuplicateCapability { capability } => {
                write!(f, "capability `{capability}` is already declared")
            }
            Self::DuplicateResource { resource } => {
                write!(f, "resource `{resource}` is already declared")
            }
            Self::InvalidTargetIdentifier { value } => {
                write!(f, "invalid target identifier `{value}`")
            }
        }
    }
}

impl Error for CompatibilityError {}

/// Stable, extensible identifier for a capability.
///
/// Examples include `cpu.integer`, `quantum.measurement`,
/// `quantum.mid_circuit_measurement`, `gpu.compute`, `hdl.synthesis`, and
/// `distributed.execution`. These are examples, not a closed registry.
///
/// Identifiers are case-sensitive. Producers should publish canonical,
/// consistently named identifiers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CapabilityId(String);

impl CapabilityId {
    pub fn new(value: impl Into<String>) -> Result<Self, CompatibilityError> {
        let value = value.into();
        validate_identifier("capability", &value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CapabilityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Stable, extensible identifier for a measurable resource dimension.
///
/// Examples include `memory.bytes`, `quantum.logical_qubits`,
/// `quantum.physical_qubits`, `cpu.cores`, and `network.bandwidth`.
///
/// The identifier does not imply that the target has reported a capacity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceId(String);

impl ResourceId {
    pub fn new(value: impl Into<String>) -> Result<Self, CompatibilityError> {
        let value = value.into();
        validate_identifier("resource", &value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ResourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Validate an extensible identifier without imposing an arbitrary length
/// limit. ASCII makes the canonical identifier alphabet explicit and stable.
fn validate_identifier(
    kind: &'static str,
    value: &str,
) -> Result<(), CompatibilityError> {
    if value.is_empty() {
        return Err(CompatibilityError::EmptyIdentifier { kind });
    }

    let mut chars = value.chars();

    let first = chars.next().ok_or(CompatibilityError::EmptyIdentifier {
        kind,
    })?;

    if !first.is_ascii_alphanumeric()
        || !chars.all(|c| {
            c.is_ascii_alphanumeric()
                || matches!(c, '.' | '_' | '-' | '/' | '+' | ':')
        })
    {
        return Err(CompatibilityError::InvalidIdentifier {
            kind,
            value: value.to_owned(),
        });
    }

    Ok(())
}

/// Support level reported by a target adapter.
///
/// `Unsupported` is an explicit negative declaration.
/// `Unknown` means that the adapter cannot establish support.
/// `Partial` means only a subset is implemented; the subset should be
/// represented by narrower capability IDs whenever possible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SupportLevel {
    Unsupported,
    Unknown,
    Partial,
    Experimental,
    Supported,
}

impl SupportLevel {
    pub fn is_fully_supported(self) -> bool {
        self == Self::Supported
    }
}

/// Evidence attached to a target capability declaration.
///
/// Evidence should describe what was actually established, such as a
/// validated emitter version, a successful conformance test, or a documented
/// target/toolchain contract. It is descriptive metadata, not cryptographic
/// proof.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilitySupport {
    pub capability: CapabilityId,
    pub level: SupportLevel,
    pub evidence: Option<String>,
    pub limitations: Vec<String>,
}

impl CapabilitySupport {
    pub fn new(
        capability: CapabilityId,
        level: SupportLevel,
    ) -> Self {
        Self {
            capability,
            level,
            evidence: None,
            limitations: Vec::new(),
        }
    }

    pub fn with_evidence(mut self, evidence: impl Into<String>) -> Self {
        self.evidence = Some(evidence.into());
        self
    }

    pub fn with_limitation(mut self, limitation: impl Into<String>) -> Self {
        self.limitations.push(limitation.into());
        self
    }
}

/// Whether a capability inventory is exhaustive.
///
/// A partial inventory must not be interpreted as proof that unlisted
/// capabilities are unsupported.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryCompleteness {
    Partial,
    Complete,
}

/// Arbitrary-length, non-negative decimal integer.
///
/// Resource quantities are commonly finite in the real world, but this
/// compatibility contract does not impose a universal `u64`, `u128`, or
/// machine-word ceiling on their representation.
///
/// The value is normalized on construction, so equality and ordering are
/// numerical: `"00042"` and `"42"` represent the same quantity.
///
/// This type intentionally supports integer quantities only. Units and
/// resource-specific interpretation belong to the resource contract.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecimalInteger(String);

impl DecimalInteger {
    pub fn new(value: impl Into<String>) -> Result<Self, CompatibilityError> {
        let value = value.into();

        if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
            return Err(CompatibilityError::InvalidDecimalInteger { value });
        }

        let normalized = value.trim_start_matches('0');
        let normalized = if normalized.is_empty() {
            "0"
        } else {
            normalized
        };

        Ok(Self(normalized.to_owned()))
    }

    pub fn zero() -> Self {
        Self("0".to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_zero(&self) -> bool {
        self.0 == "0"
    }
}

impl fmt::Display for DecimalInteger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl PartialOrd for DecimalInteger {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DecimalInteger {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .len()
            .cmp(&other.0.len())
            .then_with(|| self.0.cmp(&other.0))
    }
}

/// How a request uses a capability or resource requirement.
///
/// `Required` and `Constraint` are hard requirements. `Preference` and
/// `Hint` must never make an otherwise feasible target incompatible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequirementStrength {
    Required,
    Constraint,
    Preference,
    Hint,
}

impl RequirementStrength {
    pub fn is_hard(self) -> bool {
        matches!(self, Self::Required | Self::Constraint)
    }
}

/// One requested capability.
///
/// A capability identifier should be specific enough to describe the actual
/// feature required. For example, requiring `quantum.measurement` must not be
/// used as a substitute for requiring a more specific mid-circuit measurement
/// feature when that distinction changes program behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityRequirement {
    pub capability: CapabilityId,
    pub strength: RequirementStrength,
    pub rationale: Option<String>,
}

impl CapabilityRequirement {
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

    pub fn required(capability: CapabilityId) -> Self {
        Self::new(capability, RequirementStrength::Required)
    }

    pub fn with_rationale(mut self, rationale: impl Into<String>) -> Self {
        self.rationale = Some(rationale.into());
        self
    }
}

/// One requested minimum resource capacity.
///
/// Both requested and reported quantities are non-negative decimal integers.
/// The resource's unit and semantics are defined by its `ResourceId` contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceRequirement {
    pub resource: ResourceId,
    pub minimum: DecimalInteger,
    pub strength: RequirementStrength,
    pub rationale: Option<String>,
}

impl ResourceRequirement {
    pub fn new(
        resource: ResourceId,
        minimum: DecimalInteger,
        strength: RequirementStrength,
    ) -> Self {
        Self {
            resource,
            minimum,
            strength,
            rationale: None,
        }
    }

    pub fn required(
        resource: ResourceId,
        minimum: DecimalInteger,
    ) -> Self {
        Self::new(resource, minimum, RequirementStrength::Required)
    }

    pub fn with_rationale(mut self, rationale: impl Into<String>) -> Self {
        self.rationale = Some(rationale.into());
        self
    }
}

/// Target-reported availability for a resource.
///
/// `Unavailable` means the target explicitly cannot provide that resource.
/// `Unknown` means evidence is missing. `Known` reports a measured, declared,
/// or otherwise authoritative capacity supplied by the target adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceAvailability {
    Known {
        capacity: DecimalInteger,
        evidence: Option<String>,
    },
    Unavailable {
        reason: String,
    },
    Unknown {
        reason: Option<String>,
    },
}

impl ResourceAvailability {
    pub fn known(capacity: DecimalInteger) -> Self {
        Self::Known {
            capacity,
            evidence: None,
        }
    }

    pub fn known_with_evidence(
        capacity: DecimalInteger,
        evidence: impl Into<String>,
    ) -> Self {
        Self::Known {
            capacity,
            evidence: Some(evidence.into()),
        }
    }

    pub fn unavailable(reason: impl Into<String>) -> Self {
        Self::Unavailable {
            reason: reason.into(),
        }
    }

    pub fn unknown() -> Self {
        Self::Unknown { reason: None }
    }
}

/// Policy for interpreting target evidence.
///
/// The default is deliberately strict: an experimental implementation does
/// not satisfy a hard requirement unless the caller explicitly opts in.
/// Partial implementations never satisfy hard requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompatibilityPolicy {
    pub allow_experimental_for_hard_requirements: bool,
}

impl Default for CompatibilityPolicy {
    fn default() -> Self {
        Self {
            allow_experimental_for_hard_requirements: false,
        }
    }
}

/// Requirements that a compilation request needs a target to satisfy.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CompatibilityRequest {
    pub capabilities: Vec<CapabilityRequirement>,
    pub resources: Vec<ResourceRequirement>,
    pub policy: CompatibilityPolicy,
}

impl CompatibilityRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_policy(mut self, policy: CompatibilityPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn require_capability(
        &mut self,
        requirement: CapabilityRequirement,
    ) {
        self.capabilities.push(requirement);
    }

    pub fn require_resource(&mut self, requirement: ResourceRequirement) {
        self.resources.push(requirement);
    }

    pub fn with_capability(
        mut self,
        requirement: CapabilityRequirement,
    ) -> Self {
        self.require_capability(requirement);
        self
    }

    pub fn with_resource(mut self, requirement: ResourceRequirement) -> Self {
        self.require_resource(requirement);
        self
    }
}

/// A target's declared compatibility facts.
///
/// This profile is an evidence container, not a target selector or hardware
/// allocator. It does not infer support from a target's name or architecture.
///
/// Target adapters should construct profiles from their actual backend,
/// toolchain, runtime, or deployment contracts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetCapabilityProfile {
    pub target_id: String,
    pub inventory_completeness: InventoryCompleteness,
    capabilities: BTreeMap<CapabilityId, CapabilitySupport>,
    resources: BTreeMap<ResourceId, ResourceAvailability>,
}

impl TargetCapabilityProfile {
    pub fn new(
        target_id: impl Into<String>,
    ) -> Result<Self, CompatibilityError> {
        let target_id = target_id.into();

        if target_id.trim().is_empty() {
            return Err(CompatibilityError::InvalidTargetIdentifier {
                value: target_id,
            });
        }

        Ok(Self {
            target_id,
            inventory_completeness: InventoryCompleteness::Partial,
            capabilities: BTreeMap::new(),
            resources: BTreeMap::new(),
        })
    }

    /// Mark the capability inventory as exhaustive.
    ///
    /// Once marked complete, an unlisted capability is treated as explicitly
    /// unsupported. Call this only when the adapter can substantiate that
    /// claim for the requested compilation environment.
    pub fn mark_capability_inventory_complete(&mut self) {
        self.inventory_completeness = InventoryCompleteness::Complete;
    }

    /// Declare one capability. Duplicate declarations are rejected rather
    /// than silently replacing evidence from another source.
    pub fn declare_capability(
        &mut self,
        support: CapabilitySupport,
    ) -> Result<(), CompatibilityError> {
        let id = support.capability.clone();

        if self.capabilities.contains_key(&id) {
            return Err(CompatibilityError::DuplicateCapability {
                capability: id,
            });
        }

        self.capabilities.insert(id, support);
        Ok(())
    }

    /// Declare resource availability. Duplicate declarations are rejected.
    pub fn declare_resource(
        &mut self,
        resource: ResourceId,
        availability: ResourceAvailability,
    ) -> Result<(), CompatibilityError> {
        if self.resources.contains_key(&resource) {
            return Err(CompatibilityError::DuplicateResource { resource });
        }

        self.resources.insert(resource, availability);
        Ok(())
    }

    pub fn capability(
        &self,
        id: &CapabilityId,
    ) -> Option<&CapabilitySupport> {
        self.capabilities.get(id)
    }

    pub fn resource(
        &self,
        id: &ResourceId,
    ) -> Option<&ResourceAvailability> {
        self.resources.get(id)
    }

    pub fn capabilities(
        &self,
    ) -> impl Iterator<Item = (&CapabilityId, &CapabilitySupport)> {
        self.capabilities.iter()
    }

    pub fn resources(
        &self,
    ) -> impl Iterator<Item = (&ResourceId, &ResourceAvailability)> {
        self.resources.iter()
    }
}

/// Overall compatibility classification.
///
/// `Indeterminate` is not a success. A caller must not proceed as if hard
/// requirements were satisfied when the evidence is insufficient.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompatibilityStatus {
    Compatible,
    CompatibleWithWarnings,
    Indeterminate,
    Incompatible,
}

/// Stable machine-readable reason for a compatibility finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompatibilityIssueCode {
    CapabilityUnsupported,
    CapabilityPartiallySupported,
    CapabilityExperimental,
    CapabilityUnknown,
    CapabilityPreferenceUnavailable,
    ResourceInsufficient,
    ResourceUnavailable,
    ResourceUnknown,
    ResourcePreferenceUnavailable,
}

/// Severity of a compatibility finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompatibilitySeverity {
    Info,
    Warning,
    Error,
}

/// One compatibility finding.
///
/// `subject` is the canonical capability/resource identifier.
/// `message` is for people; consumers should branch on `code` and `severity`,
/// not parse this text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityIssue {
    pub code: CompatibilityIssueCode,
    pub severity: CompatibilitySeverity,
    pub subject: String,
    pub strength: RequirementStrength,
    pub message: String,
    pub evidence: Option<String>,
}

/// Full result of checking a request against a target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityReport {
    pub target_id: String,
    pub status: CompatibilityStatus,
    pub issues: Vec<CompatibilityIssue>,
}

impl CompatibilityReport {
    pub fn is_compatible(&self) -> bool {
        matches!(
            self.status,
            CompatibilityStatus::Compatible
                | CompatibilityStatus::CompatibleWithWarnings
        )
    }

    pub fn is_fully_compatible(&self) -> bool {
        self.status == CompatibilityStatus::Compatible
    }

    pub fn has_errors(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.severity == CompatibilitySeverity::Error)
    }

    pub fn has_warnings(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.severity == CompatibilitySeverity::Warning)
    }

    pub fn errors(&self) -> impl Iterator<Item = &CompatibilityIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.severity == CompatibilitySeverity::Error)
    }

    pub fn warnings(&self) -> impl Iterator<Item = &CompatibilityIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.severity == CompatibilitySeverity::Warning)
    }
}

/// Stateless compatibility evaluator.
///
/// Keeping evaluation pure makes it deterministic and safe to reuse across
/// concurrent compilation requests.
#[derive(Debug, Clone, Copy, Default)]
pub struct CompatibilityEvaluator;

impl CompatibilityEvaluator {
    pub fn evaluate(
        &self,
        request: &CompatibilityRequest,
        target: &TargetCapabilityProfile,
    ) -> CompatibilityReport {
        let mut issues = Vec::new();

        for requirement in &request.capabilities {
            self.evaluate_capability(requirement, request.policy, target, &mut issues);
        }

        for requirement in &request.resources {
            self.evaluate_resource(requirement, target, &mut issues);
        }

        // Stable output independent of the insertion order of requirements.
        issues.sort_by(|left, right| {
            left.subject
                .cmp(&right.subject)
                .then_with(|| left.code.cmp(&right.code))
                .then_with(|| left.severity.cmp(&right.severity))
                .then_with(|| left.message.cmp(&right.message))
        });

        let status = classify_status(&issues);

        CompatibilityReport {
            target_id: target.target_id.clone(),
            status,
            issues,
        }
    }

    fn evaluate_capability(
        &self,
        requirement: &CapabilityRequirement,
        policy: CompatibilityPolicy,
        target: &TargetCapabilityProfile,
        issues: &mut Vec<CompatibilityIssue>,
    ) {
        let known = target.capability(&requirement.capability);

        let level = match known {
            Some(support) => support.level,
            None if target.inventory_completeness == InventoryCompleteness::Complete => {
                SupportLevel::Unsupported
            }
            None => SupportLevel::Unknown,
        };

        let evidence = known.and_then(|support| {
            let mut parts = Vec::new();

            if let Some(value) = &support.evidence {
                if !value.trim().is_empty() {
                    parts.push(value.clone());
                }
            }

            parts.extend(
                support
                    .limitations
                    .iter()
                    .filter(|value| !value.trim().is_empty())
                    .cloned(),
            );

            if parts.is_empty() {
                None
            } else {
                Some(parts.join("; "))
            }
        });

        match level {
            SupportLevel::Supported => {}
            SupportLevel::Experimental
                if requirement.strength.is_hard()
                    && policy.allow_experimental_for_hard_requirements =>
            {
                issues.push(CompatibilityIssue {
                    code: CompatibilityIssueCode::CapabilityExperimental,
                    severity: CompatibilitySeverity::Warning,
                    subject: requirement.capability.to_string(),
                    strength: requirement.strength,
                    message: format!(
                        "required capability `{}` is experimental on target `{}`; \
                         policy explicitly permits experimental support",
                        requirement.capability, target.target_id
                    ),
                    evidence,
                });
            }
            SupportLevel::Experimental if requirement.strength.is_hard() => {
                issues.push(CompatibilityIssue {
                    code: CompatibilityIssueCode::CapabilityExperimental,
                    severity: CompatibilitySeverity::Error,
                    subject: requirement.capability.to_string(),
                    strength: requirement.strength,
                    message: format!(
                        "required capability `{}` is experimental and policy \
                         does not permit it for hard requirements",
                        requirement.capability
                    ),
                    evidence,
                });
            }
            SupportLevel::Partial if requirement.strength.is_hard() => {
                issues.push(CompatibilityIssue {
                    code: CompatibilityIssueCode::CapabilityPartiallySupported,
                    severity: CompatibilitySeverity::Error,
                    subject: requirement.capability.to_string(),
                    strength: requirement.strength,
                    message: format!(
                        "hard requirement `{}` is only partially supported; \
                         partial support cannot satisfy a hard requirement",
                        requirement.capability
                    ),
                    evidence,
                });
            }
            SupportLevel::Unsupported if requirement.strength.is_hard() => {
                issues.push(CompatibilityIssue {
                    code: CompatibilityIssueCode::CapabilityUnsupported,
                    severity: CompatibilitySeverity::Error,
                    subject: requirement.capability.to_string(),
                    strength: requirement.strength,
                    message: format!(
                        "target `{}` does not support required capability `{}`",
                        target.target_id, requirement.capability
                    ),
                    evidence,
                });
            }
            SupportLevel::Unknown if requirement.strength.is_hard() => {
                issues.push(CompatibilityIssue {
                    code: CompatibilityIssueCode::CapabilityUnknown,
                    severity: CompatibilitySeverity::Error,
                    subject: requirement.capability.to_string(),
                    strength: requirement.strength,
                    message: format!(
                        "support for required capability `{}` is unknown on target `{}`",
                        requirement.capability, target.target_id
                    ),
                    evidence,
                });
            }
            SupportLevel::Experimental => {
                issues.push(CompatibilityIssue {
                    code: CompatibilityIssueCode::CapabilityPreferenceUnavailable,
                    severity: CompatibilitySeverity::Warning,
                    subject: requirement.capability.to_string(),
                    strength: requirement.strength,
                    message: format!(
                        "capability `{}` is experimental; this is not full stable support",
                        requirement.capability
                    ),
                    evidence,
                });
            }
            SupportLevel::Partial => {
                issues.push(CompatibilityIssue {
                    code: CompatibilityIssueCode::CapabilityPreferenceUnavailable,
                    severity: CompatibilitySeverity::Warning,
                    subject: requirement.capability.to_string(),
                    strength: requirement.strength,
                    message: format!(
                        "capability `{}` is only partially supported and cannot \
                         be treated as full support",
                        requirement.capability
                    ),
                    evidence,
                });
            }
            SupportLevel::Unsupported => {
                issues.push(CompatibilityIssue {
                    code: CompatibilityIssueCode::CapabilityPreferenceUnavailable,
                    severity: CompatibilitySeverity::Warning,
                    subject: requirement.capability.to_string(),
                    strength: requirement.strength,
                    message: format!(
                        "preferred or hinted capability `{}` is unsupported",
                        requirement.capability
                    ),
                    evidence,
                });
            }
            SupportLevel::Unknown => {
                issues.push(CompatibilityIssue {
                    code: CompatibilityIssueCode::CapabilityUnknown,
                    severity: CompatibilitySeverity::Info,
                    subject: requirement.capability.to_string(),
                    strength: requirement.strength,
                    message: format!(
                        "support for preferred or hinted capability `{}` is unknown",
                        requirement.capability
                    ),
                    evidence,
                });
            }
        }
    }

    fn evaluate_resource(
        &self,
        requirement: &ResourceRequirement,
        target: &TargetCapabilityProfile,
        issues: &mut Vec<CompatibilityIssue>,
    ) {
        let availability = target.resource(&requirement.resource);

        match availability {
            Some(ResourceAvailability::Known { capacity, evidence }) => {
                if capacity >= &requirement.minimum {
                    return;
                }

                let severity = if requirement.strength.is_hard() {
                    CompatibilitySeverity::Error
                } else {
                    CompatibilitySeverity::Warning
                };

                issues.push(CompatibilityIssue {
                    code: if requirement.strength.is_hard() {
                        CompatibilityIssueCode::ResourceInsufficient
                    } else {
                        CompatibilityIssueCode::ResourcePreferenceUnavailable
                    },
                    severity,
                    subject: requirement.resource.to_string(),
                    strength: requirement.strength,
                    message: format!(
                        "resource `{}` requires at least {}, but target `{}` \
                         reports capacity {}",
                        requirement.resource,
                        requirement.minimum,
                        target.target_id,
                        capacity
                    ),
                    evidence: evidence.clone(),
                });
            }
            Some(ResourceAvailability::Unavailable { reason }) => {
                let severity = if requirement.strength.is_hard() {
                    CompatibilitySeverity::Error
                } else {
                    CompatibilitySeverity::Warning
                };

                issues.push(CompatibilityIssue {
                    code: if requirement.strength.is_hard() {
                        CompatibilityIssueCode::ResourceUnavailable
                    } else {
                        CompatibilityIssueCode::ResourcePreferenceUnavailable
                    },
                    severity,
                    subject: requirement.resource.to_string(),
                    strength: requirement.strength,
                    message: format!(
                        "resource `{}` is unavailable on target `{}`: {}",
                        requirement.resource, target.target_id, reason
                    ),
                    evidence: Some(reason.clone()),
                });
            }
            Some(ResourceAvailability::Unknown { reason }) => {
                let severity = if requirement.strength.is_hard() {
                    CompatibilitySeverity::Error
                } else {
                    CompatibilitySeverity::Info
                };

                issues.push(CompatibilityIssue {
                    code: CompatibilityIssueCode::ResourceUnknown,
                    severity,
                    subject: requirement.resource.to_string(),
                    strength: requirement.strength,
                    message: format!(
                        "capacity for resource `{}` is unknown on target `{}`",
                        requirement.resource, target.target_id
                    ),
                    evidence: reason.clone(),
                });
            }
            None => {
                let explicitly_complete =
                    target.inventory_completeness == InventoryCompleteness::Complete;

                let severity = if requirement.strength.is_hard() {
                    CompatibilitySeverity::Error
                } else {
                    CompatibilitySeverity::Info
                };

                let code = if explicitly_complete && requirement.strength.is_hard() {
                    CompatibilityIssueCode::ResourceUnavailable
                } else {
                    CompatibilityIssueCode::ResourceUnknown
                };

                let message = if explicitly_complete && requirement.strength.is_hard() {
                    format!(
                        "target `{}` declares a complete inventory but has no \
                         availability for required resource `{}`",
                        target.target_id, requirement.resource
                    )
                } else {
                    format!(
                        "no capacity evidence is available for resource `{}` on target `{}`",
                        requirement.resource, target.target_id
                    )
                };

                issues.push(CompatibilityIssue {
                    code,
                    severity,
                    subject: requirement.resource.to_string(),
                    strength: requirement.strength,
                    message,
                    evidence: None,
                });
            }
        }
    }
}

fn classify_status(issues: &[CompatibilityIssue]) -> CompatibilityStatus {
    if issues
        .iter()
        .any(|issue| issue.severity == CompatibilitySeverity::Error)
    {
        // Hard requirements with unknown evidence are errors for purposes of
        // allowing code generation. The issue code distinguishes unknown from
        // proven incompatibility.
        if issues.iter().any(|issue| {
            issue.severity == CompatibilitySeverity::Error
                && matches!(
                    issue.code,
                    CompatibilityIssueCode::CapabilityUnknown
                        | CompatibilityIssueCode::ResourceUnknown
                )
        }) && !issues.iter().any(|issue| {
            issue.severity == CompatibilitySeverity::Error
                && matches!(
                    issue.code,
                    CompatibilityIssueCode::CapabilityUnsupported
                        | CompatibilityIssueCode::CapabilityPartiallySupported
                        | CompatibilityIssueCode::CapabilityExperimental
                        | CompatibilityIssueCode::ResourceInsufficient
                        | CompatibilityIssueCode::ResourceUnavailable
                )
        }) {
            return CompatibilityStatus::Indeterminate;
        }

        return CompatibilityStatus::Incompatible;
    }

    if issues
        .iter()
        .any(|issue| issue.severity == CompatibilitySeverity::Warning)
    {
        return CompatibilityStatus::CompatibleWithWarnings;
    }

    CompatibilityStatus::Compatible
}

#[cfg(test)]
mod tests {
    use super::*;

    fn capability(value: &str) -> CapabilityId {
        CapabilityId::new(value).expect("valid capability ID")
    }

    fn resource(value: &str) -> ResourceId {
        ResourceId::new(value).expect("valid resource ID")
    }

    fn quantity(value: &str) -> DecimalInteger {
        DecimalInteger::new(value).expect("valid resource quantity")
    }

    fn target() -> TargetCapabilityProfile {
        TargetCapabilityProfile::new("test-target").expect("valid target")
    }

    #[test]
    fn identifiers_are_extensible_and_reject_invalid_syntax() {
        assert!(CapabilityId::new("quantum.mid_circuit-measurement").is_ok());
        assert!(CapabilityId::new("vendor/qpu:measurement+v2").is_ok());
        assert!(CapabilityId::new("").is_err());
        assert!(CapabilityId::new("quantum measurement").is_err());
        assert!(ResourceId::new("memory.bytes").is_ok());
    }

    #[test]
    fn decimal_quantities_are_normalized_and_compared_without_fixed_width() {
        let small = quantity("00000042");
        let equal = quantity("42");
        let large = quantity(
            "999999999999999999999999999999999999999999999999999999999999999999",
        );

        assert_eq!(small, equal);
        assert!(large > small);
        assert_eq!(quantity("0000"), DecimalInteger::zero());
        assert!(DecimalInteger::new("-1").is_err());
        assert!(DecimalInteger::new("1.5").is_err());
        assert!(DecimalInteger::new("1e9").is_err());
    }

    #[test]
    fn supported_required_capability_is_compatible() {
        let mut target = target();
        target
            .declare_capability(CapabilitySupport::new(
                capability("cpu.integer"),
                SupportLevel::Supported,
            ))
            .expect("unique capability");

        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::required(capability("cpu.integer")));

        let report = CompatibilityEvaluator.evaluate(&request, &target);

        assert_eq!(report.status, CompatibilityStatus::Compatible);
        assert!(report.issues.is_empty());
    }

    #[test]
    fn explicit_unsupported_capability_blocks_hard_requirement() {
        let mut target = target();
        target
            .declare_capability(CapabilitySupport::new(
                capability("quantum.mid_circuit_measurement"),
                SupportLevel::Unsupported,
            ))
            .expect("unique capability");

        let request = CompatibilityRequest::new().with_capability(
            CapabilityRequirement::required(capability("quantum.mid_circuit_measurement")),
        );

        let report = CompatibilityEvaluator.evaluate(&request, &target);

        assert_eq!(report.status, CompatibilityStatus::Incompatible);
        assert_eq!(
            report.issues[0].code,
            CompatibilityIssueCode::CapabilityUnsupported
        );
    }

    #[test]
    fn missing_capability_in_partial_inventory_is_indeterminate() {
        let target = target();
        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::required(capability("gpu.compute")));

        let report = CompatibilityEvaluator.evaluate(&request, &target);

        assert_eq!(report.status, CompatibilityStatus::Indeterminate);
        assert_eq!(
            report.issues[0].code,
            CompatibilityIssueCode::CapabilityUnknown
        );
    }

    #[test]
    fn missing_capability_in_complete_inventory_is_unsupported() {
        let mut target = target();
        target.mark_capability_inventory_complete();

        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::required(capability("gpu.compute")));

        let report = CompatibilityEvaluator.evaluate(&request, &target);

        assert_eq!(report.status, CompatibilityStatus::Incompatible);
        assert_eq!(
            report.issues[0].code,
            CompatibilityIssueCode::CapabilityUnsupported
        );
    }

    #[test]
    fn partial_support_never_satisfies_a_hard_requirement() {
        let mut target = target();
        target
            .declare_capability(CapabilitySupport::new(
                capability("quantum.measurement"),
                SupportLevel::Partial,
            ))
            .expect("unique capability");

        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::required(capability("quantum.measurement")));

        let report = CompatibilityEvaluator.evaluate(&request, &target);

        assert_eq!(report.status, CompatibilityStatus::Incompatible);
        assert_eq!(
            report.issues[0].code,
            CompatibilityIssueCode::CapabilityPartiallySupported
        );
    }

    #[test]
    fn experimental_support_requires_explicit_policy_permission() {
        let mut target = target();
        target
            .declare_capability(CapabilitySupport::new(
                capability("gpu.compute"),
                SupportLevel::Experimental,
            ))
            .expect("unique capability");

        let request = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::required(capability("gpu.compute")));

        let denied = CompatibilityEvaluator.evaluate(&request, &target);
        assert_eq!(denied.status, CompatibilityStatus::Incompatible);

        let allowed = request.with_policy(CompatibilityPolicy {
            allow_experimental_for_hard_requirements: true,
        });
        let accepted = CompatibilityEvaluator.evaluate(&allowed, &target);

        assert_eq!(
            accepted.status,
            CompatibilityStatus::CompatibleWithWarnings
        );
        assert!(accepted.has_warnings());
    }

    #[test]
    fn resource_capacity_is_compared_numerically() {
        let mut target = target();
        target
            .declare_resource(
                resource("memory.bytes"),
                ResourceAvailability::known(quantity("00001024")),
            )
            .expect("unique resource");

        let sufficient = CompatibilityRequest::new().with_resource(
            ResourceRequirement::required(
                resource("memory.bytes"),
                quantity("1024"),
            ),
        );
        assert_eq!(
            CompatibilityEvaluator.evaluate(&sufficient, &target).status,
            CompatibilityStatus::Compatible
        );

        let insufficient = CompatibilityRequest::new().with_resource(
            ResourceRequirement::required(
                resource("memory.bytes"),
                quantity("1025"),
            ),
        );
        assert_eq!(
            CompatibilityEvaluator.evaluate(&insufficient, &target).status,
            CompatibilityStatus::Incompatible
        );
    }

    #[test]
    fn unknown_resource_capacity_does_not_claim_success() {
        let mut target = target();
        target
            .declare_resource(resource("quantum.logical_qubits"), ResourceAvailability::unknown())
            .expect("unique resource");

        let request = CompatibilityRequest::new().with_resource(
            ResourceRequirement::required(
                resource("quantum.logical_qubits"),
                quantity("100000000000000000000000000000000000000000"),
            ),
        );

        let report = CompatibilityEvaluator.evaluate(&request, &target);

        assert_eq!(report.status, CompatibilityStatus::Indeterminate);
        assert_eq!(
            report.issues[0].code,
            CompatibilityIssueCode::ResourceUnknown
        );
    }

    #[test]
    fn missing_resource_in_complete_inventory_is_not_assumed_available() {
        let mut target = target();
        target.mark_capability_inventory_complete();

        let request = CompatibilityRequest::new().with_resource(
            ResourceRequirement::required(resource("memory.bytes"), quantity("1")),
        );

        let report = CompatibilityEvaluator.evaluate(&request, &target);

        assert_eq!(report.status, CompatibilityStatus::Incompatible);
        assert_eq!(
            report.issues[0].code,
            CompatibilityIssueCode::ResourceUnavailable
        );
    }

    #[test]
    fn preferences_do_not_block_an_otherwise_feasible_target() {
        let target = target();
        let request = CompatibilityRequest::new().with_capability(
            CapabilityRequirement::new(
                capability("gpu.compute"),
                RequirementStrength::Preference,
            ),
        );

        let report = CompatibilityEvaluator.evaluate(&request, &target);

        assert_eq!(
            report.status,
            CompatibilityStatus::CompatibleWithWarnings
        );
        assert!(!report.has_errors());
    }

    #[test]
    fn duplicate_target_facts_are_rejected() {
        let mut target = target();
        let id = capability("cpu.integer");

        target
            .declare_capability(CapabilitySupport::new(
                id.clone(),
                SupportLevel::Supported,
            ))
            .expect("first declaration succeeds");

        assert!(matches!(
            target.declare_capability(CapabilitySupport::new(
                id.clone(),
                SupportLevel::Unsupported,
            )),
            Err(CompatibilityError::DuplicateCapability { .. })
        ));

        let memory = resource("memory.bytes");
        target
            .declare_resource(memory.clone(), ResourceAvailability::known(quantity("10")))
            .expect("first declaration succeeds");

        assert!(matches!(
            target.declare_resource(
                memory,
                ResourceAvailability::known(quantity("20"))
            ),
            Err(CompatibilityError::DuplicateResource { .. })
        ));
    }

    #[test]
    fn report_order_is_deterministic() {
        let mut target = target();
        target.mark_capability_inventory_complete();

        let first = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::required(capability("zeta.feature")))
            .with_capability(CapabilityRequirement::required(capability("alpha.feature")));

        let second = CompatibilityRequest::new()
            .with_capability(CapabilityRequirement::required(capability("alpha.feature")))
            .with_capability(CapabilityRequirement::required(capability("zeta.feature")));

        let a = CompatibilityEvaluator.evaluate(&first, &target);
        let b = CompatibilityEvaluator.evaluate(&second, &target);

        assert_eq!(a, b);
    }

    #[test]
    fn empty_request_is_compatible_with_known_target_identity() {
        let target = target();
        let report = CompatibilityEvaluator.evaluate(&CompatibilityRequest::new(), &target);

        assert_eq!(report.status, CompatibilityStatus::Compatible);
        assert!(report.issues.is_empty());
        assert!(report.is_fully_compatible());
    }
}
