
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Feasibility Analysis
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! This module evaluates compiler-facing feasibility evidence.
//!
//! It combines evidence supplied by authoritative compiler subsystems.
//! It does not redefine semantic requirements, capabilities, resources,
//! dependencies, backend compatibility, or physical execution.
//!
//! # Architectural ownership
//!
//! Semantic authority:
//!     crate::frontend::semantic
//!
//! Requirement integration:
//!     crate::compiler::analysis::requirements
//!
//! Capability analysis:
//!     crate::compiler::analysis::capabilities
//!
//! Dependency analysis:
//!     crate::compiler::analysis::dependency
//!
//! Cost analysis:
//!     crate::compiler::analysis::cost
//!
//! Resource feasibility:
//!     crate::compiler::planning::resource_planning
//!
//! Realization planning:
//!     crate::compiler::planning::feasibility
//!
//! Physical allocation and execution:
//!     crate::runtime
//!
//! # Important invariants
//!
//! - Unknown evidence is never proof of feasibility.
//! - Missing capacity is never unlimited capacity.
//! - An unmet preference is not a mandatory failure.
//! - A mandatory constraint violation establishes infeasibility.
//! - An incomplete assessment is indeterminate.
//! - Resource quantities are not restricted to machine integer widths.
//! - No universal hardware population or capacity limits are introduced.
//! - No physical resources are acquired or allocated.
//! - No unsafe Rust is permitted.
//!
//! Literal infinity cannot be processed by finite hardware.
//! Actual processing is bounded by available resources and explicit
//! operational policies.

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable module identity.
pub const FEASIBILITY_ANALYSIS_MODULE_NAME: &str =
    "zamani.compiler.analysis.feasibility";

/// Version of the compiler-facing feasibility report contract.
pub const FEASIBILITY_ANALYSIS_SCHEMA_VERSION: u16 = 2;

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// An open, namespaced identifier.
///
/// No closed list of domains, hardware, vendors, or resource types
/// is imposed.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct Identifier(String);

impl Identifier {
    /// Creates a validated identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, FeasibilityError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(FeasibilityError::EmptyIdentifier);
        }

        if value.chars().any(char::is_control) {
            return Err(FeasibilityError::InvalidIdentifier {
                value,
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Identifier {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// EVIDENCE
// ============================================================================

/// State of supplied evidence.
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
pub enum EvidenceState {
    /// Evidence establishes satisfaction.
    Satisfied,

    /// Evidence establishes non-satisfaction.
    Unsatisfied,

    /// Evidence is missing, incomplete, or unresolved.
    Unknown,
}

impl EvidenceState {
    /// Returns whether satisfaction is established.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied)
    }

    /// Returns whether incompatibility is established.
    #[must_use]
    pub const fn is_unsatisfied(self) -> bool {
        matches!(self, Self::Unsatisfied)
    }

    /// Returns whether evidence is unresolved.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

// ============================================================================
// REQUIREMENT CLASSIFICATION
// ============================================================================

/// Classification of a condition being evaluated.
///
/// This is a compiler-facing classification, not a replacement for
/// canonical semantic requirement types.
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
pub enum RequirementClass {
    /// Mandatory program requirement.
    HardRequirement,

    /// Mandatory realization constraint.
    Constraint,

    /// Non-mandatory preference.
    Preference,

    /// Advisory implementation hint.
    Hint,
}

impl RequirementClass {
    /// Whether failure prevents feasibility.
    #[must_use]
    pub const fn is_mandatory(self) -> bool {
        matches!(
            self,
            Self::HardRequirement | Self::Constraint
        )
    }
}

/// One evaluated requirement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequirementEvidence {
    /// Requirement identity.
    pub id: Identifier,

    /// Requirement classification.
    pub class: RequirementClass,

    /// Evaluation result.
    pub state: EvidenceState,

    /// Optional explanation.
    pub explanation: Option<String>,
}

impl RequirementEvidence {
    /// Creates requirement evidence.
    #[must_use]
    pub fn new(
        id: Identifier,
        class: RequirementClass,
        state: EvidenceState,
    ) -> Self {
        Self {
            id,
            class,
            state,
            explanation: None,
        }
    }

    /// Attaches an explanation.
    #[must_use]
    pub fn with_explanation(
        mut self,
        explanation: impl Into<String>,
    ) -> Self {
        self.explanation = Some(explanation.into());
        self
    }
}

// ============================================================================
// ARBITRARY-LENGTH RESOURCE QUANTITIES
// ============================================================================

/// An exact non-negative integer.
///
/// Quantities use canonical decimal strings rather than u64, u128,
/// usize, floating-point values, or hardware-specific integer widths.
///
/// Actual representation size remains bounded by available memory.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ResourceQuantity(String);

impl ResourceQuantity {
    /// Creates a canonical quantity.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, FeasibilityError> {
        let value = value.into();

        if value.is_empty()
            || !value.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(FeasibilityError::InvalidQuantity {
                value,
            });
        }

        let normalized = value.trim_start_matches('0');

        Ok(Self(
            if normalized.is_empty() {
                "0".to_owned()
            } else {
                normalized.to_owned()
            },
        ))
    }

    /// Creates zero.
    #[must_use]
    pub fn zero() -> Self {
        Self("0".to_owned())
    }

    /// Returns the canonical representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns whether the quantity is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.0 == "0"
    }

    /// Compares arbitrary-length quantities.
    #[must_use]
    pub fn cmp_quantity(
        &self,
        other: &Self,
    ) -> Ordering {
        self.0
            .len()
            .cmp(&other.0.len())
            .then_with(|| self.0.cmp(&other.0))
    }

    /// Adds two quantities without machine-integer conversion.
    pub fn checked_add(
        &self,
        other: &Self,
    ) -> Result<Self, FeasibilityError> {
        let mut left = self.0.bytes().rev();
        let mut right = other.0.bytes().rev();

        let mut result = Vec::new();
        let mut carry = 0_u8;

        loop {
            let a = left.next();
            let b = right.next();

            if a.is_none() && b.is_none() && carry == 0 {
                break;
            }

            let a = a.map(|v| v - b'0').unwrap_or(0);
            let b = b.map(|v| v - b'0').unwrap_or(0);

            let sum = a + b + carry;

            result.push(b'0' + sum % 10);
            carry = sum / 10;
        }

        result.reverse();

        let digits = String::from_utf8(result)
            .map_err(|_| FeasibilityError::ArithmeticFailure)?;

        Ok(Self(digits))
    }
}

impl Ord for ResourceQuantity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_quantity(other)
    }
}

impl PartialOrd for ResourceQuantity {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for ResourceQuantity {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// RESOURCE IDENTITIES
// ============================================================================

/// An open resource dimension.
///
/// Examples include memory.bytes, quantum.logical_qubits,
/// ai.tensor_elements, networking.bandwidth, and future resources.
///
/// These examples are not an exhaustive enumeration.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct ResourceId(Identifier);

impl ResourceId {
    /// Creates a resource identity.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, FeasibilityError> {
        Ok(Self(Identifier::new(value)?))
    }

    /// Returns the identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for ResourceId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Demand for an abstract resource.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceDemand {
    /// Resource dimension.
    pub resource: ResourceId,

    /// Required quantity.
    pub required: ResourceQuantity,

    /// Whether the demand is mandatory.
    pub mandatory: bool,
}

/// Explicit available resource capacity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceCapacity {
    /// Resource dimension.
    pub resource: ResourceId,

    /// Available quantity.
    pub available: ResourceQuantity,
}

/// Result of assessing one demand.
///
/// Each demand receives an independent finding. Multiple independent
/// demands for one resource are never silently discarded.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceFinding {
    /// Resource dimension.
    pub resource: ResourceId,

    /// Required quantity.
    pub required: ResourceQuantity,

    /// Known capacity.
    pub available: Option<ResourceQuantity>,

    /// Assessment state.
    pub state: EvidenceState,

    /// Explanation.
    pub explanation: String,
}

// ============================================================================
// REALIZATION EVIDENCE
// ============================================================================

/// Opaque proposed realization identity.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct RealizationId(Identifier);

impl RealizationId {
    /// Creates a realization identity.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, FeasibilityError> {
        Ok(Self(Identifier::new(value)?))
    }

    /// Returns the realization identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for RealizationId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Evidence for one proposed realization.
///
/// This structure is not a hardware descriptor or device allocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealizationEvidence {
    /// Proposed realization.
    pub id: RealizationId,

    /// Capability compatibility.
    pub capabilities: EvidenceState,

    /// Backend compatibility.
    pub backend_compatibility: EvidenceState,

    /// Data movement compatibility.
    pub data_movement: EvidenceState,

    /// Synchronization compatibility.
    pub synchronization: EvidenceState,

    /// Completeness of the supplied evidence.
    pub evidence_complete: bool,
}

impl RealizationEvidence {
    /// Creates an unresolved realization.
    #[must_use]
    pub fn new(id: RealizationId) -> Self {
        Self {
            id,
            capabilities: EvidenceState::Unknown,
            backend_compatibility: EvidenceState::Unknown,
            data_movement: EvidenceState::Unknown,
            synchronization: EvidenceState::Unknown,
            evidence_complete: false,
        }
    }

    /// Returns aggregate compatibility evidence.
    #[must_use]
    pub fn compatibility_state(&self) -> EvidenceState {
        let states = [
            self.capabilities,
            self.backend_compatibility,
            self.data_movement,
            self.synchronization,
        ];

        if states.iter().any(|state| {
            state.is_unsatisfied()
        }) {
            EvidenceState::Unsatisfied
        } else if self.evidence_complete
            && states.iter().all(|state| {
                state.is_satisfied()
            })
        {
            EvidenceState::Satisfied
        } else {
            EvidenceState::Unknown
        }
    }
}

// ============================================================================
// DEPENDENCIES
// ============================================================================

/// Evidence concerning an existing dependency.
///
/// Dependency construction and ordering remain owned by
/// analysis/dependency.rs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyFinding {
    /// Dependency identity.
    pub id: Identifier,

    /// Evidence state.
    pub state: EvidenceState,

    /// Optional explanation.
    pub explanation: Option<String>,
}

// ============================================================================
// FINDINGS
// ============================================================================

/// Classification of a feasibility finding.
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
pub enum FindingKind {
    /// Mandatory requirement violation.
    RequirementViolation,

    /// Mandatory constraint violation.
    ConstraintViolation,

    /// Preference not satisfied.
    PreferenceNotMet,

    /// Hint unavailable.
    HintUnavailable,

    /// Unsupported capability.
    UnsupportedCapability,

    /// Backend incompatibility.
    BackendIncompatibility,

    /// Insufficient resource capacity.
    ResourceInsufficiency,

    /// Evidence is missing.
    MissingEvidence,

    /// Dependency violation.
    DependencyViolation,

    /// Data movement incompatibility.
    DataMovementIncompatibility,

    /// Synchronization incompatibility.
    SynchronizationIncompatibility,

    /// Operational budget exhausted.
    BudgetExhaustion,

    /// Evaluation could not be completed.
    EvaluationFailure,
}

/// Finding severity.
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
pub enum FindingSeverity {
    /// Informational.
    Information,

    /// Advisory.
    Warning,

    /// Mandatory condition failed or cannot be established.
    Error,
}

/// Structured feasibility finding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeasibilityFinding {
    /// Finding category.
    pub kind: FindingKind,

    /// Severity.
    pub severity: FindingSeverity,

    /// Associated identity.
    pub related_id: Option<Identifier>,

    /// Explanation.
    pub message: String,
}

impl FeasibilityFinding {
    /// Creates a finding.
    #[must_use]
    pub fn new(
        kind: FindingKind,
        severity: FindingSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            severity,
            related_id: None,
            message: message.into(),
        }
    }

    /// Associates an identifier.
    #[must_use]
    pub fn with_related_id(
        mut self,
        id: Identifier,
    ) -> Self {
        self.related_id = Some(id);
        self
    }
}

// ============================================================================
// STATUS
// ============================================================================

/// Aggregate feasibility classification.
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
pub enum FeasibilityStatus {
    /// All mandatory conditions have been established.
    Feasible,

    /// At least one mandatory condition is explicitly violated.
    Infeasible,

    /// Evidence is incomplete or evaluation could not be completed.
    Indeterminate,
}

impl FeasibilityStatus {
    /// Returns whether feasibility is established.
    #[must_use]
    pub const fn is_feasible(self) -> bool {
        matches!(self, Self::Feasible)
    }

    /// Returns whether infeasibility is established.
    #[must_use]
    pub const fn is_infeasible(self) -> bool {
        matches!(self, Self::Infeasible)
    }

    /// Returns whether the result is indeterminate.
    #[must_use]
    pub const fn is_indeterminate(self) -> bool {
        matches!(self, Self::Indeterminate)
    }
}

// ============================================================================
// INPUT
// ============================================================================

/// Evidence supplied to the feasibility analyzer.
///
/// Evidence must already have been obtained and validated by its
/// authoritative subsystem.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeasibilityInput {
    /// Proposed realization.
    pub realization: RealizationEvidence,

    /// Evaluated requirements.
    pub requirements: Vec<RequirementEvidence>,

    /// Resource demands.
    pub resource_demands: Vec<ResourceDemand>,

    /// Resource capacity evidence.
    pub resource_capacities: Vec<ResourceCapacity>,

    /// Dependency findings.
    pub dependencies: Vec<DependencyFinding>,

    /// Findings supplied by upstream analysis.
    pub upstream_findings: Vec<FeasibilityFinding>,
}

impl FeasibilityInput {
    /// Creates an empty evidence collection for a realization.
    #[must_use]
    pub fn new(realization: RealizationEvidence) -> Self {
        Self {
            realization,
            requirements: Vec::new(),
            resource_demands: Vec::new(),
            resource_capacities: Vec::new(),
            dependencies: Vec::new(),
            upstream_findings: Vec::new(),
        }
    }
}

// ============================================================================
// REPORT
// ============================================================================

/// Complete feasibility report.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeasibilityReport {
    /// Proposed realization.
    pub realization: RealizationId,

    /// Overall outcome.
    pub status: FeasibilityStatus,

    /// Structured findings.
    pub findings: Vec<FeasibilityFinding>,

    /// Individual resource assessments.
    pub resources: Vec<ResourceFinding>,

    /// Whether all required evidence is complete.
    pub evidence_complete: bool,
}

impl FeasibilityReport {
    /// Returns whether the realization is feasible.
    #[must_use]
    pub const fn is_feasible(&self) -> bool {
        self.status.is_feasible()
    }

    /// Returns whether the realization is infeasible.
    #[must_use]
    pub const fn is_infeasible(&self) -> bool {
        self.status.is_infeasible()
    }

    /// Returns whether more evidence is needed.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        self.status.is_indeterminate()
    }

    /// Returns the finding count.
    #[must_use]
    pub fn finding_count(&self) -> usize {
        self.findings.len()
    }

    /// Returns whether any mandatory evidence is unresolved.
    #[must_use]
    pub fn has_missing_evidence(&self) -> bool {
        self.findings.iter().any(|finding| {
            finding.kind == FindingKind::MissingEvidence
        })
    }

    /// Returns whether any preference was not satisfied.
    #[must_use]
    pub fn has_unmet_preferences(&self) -> bool {
        self.findings.iter().any(|finding| {
            finding.kind == FindingKind::PreferenceNotMet
        })
    }
}

// ============================================================================
// ANALYZER
// ============================================================================

/// Stateless feasibility analyzer.
#[derive(Clone, Copy, Debug, Default)]
pub struct FeasibilityAnalyzer;

impl FeasibilityAnalyzer {
    /// Analyzes one proposed realization.
    ///
    /// This function performs no hardware discovery, physical allocation,
    /// backend invocation, scheduling, or execution.
    pub fn analyze(
        &self,
        input: &FeasibilityInput,
    ) -> Result<FeasibilityReport, FeasibilityError> {
        validate_input(input)?;

        let mut findings = Vec::new();

        findings
            .try_reserve(
                input.upstream_findings.len()
                    .saturating_add(
                        input.requirements.len(),
                    )
                    .saturating_add(
                        input.resource_demands.len(),
                    )
                    .saturating_add(
                        input.dependencies.len(),
                    )
                    .saturating_add(4),
            )
            .map_err(|_| FeasibilityError::AllocationFailure)?;

        findings.extend(
            input.upstream_findings.iter().cloned(),
        );

        let mut evidence_complete =
            input.realization.evidence_complete;

        evaluate_realization(
            &input.realization,
            &mut findings,
            &mut evidence_complete,
        );

        evaluate_requirements(
            &input.requirements,
            &mut findings,
            &mut evidence_complete,
        );

        let resources = evaluate_resources(
            &input.resource_demands,
            &input.resource_capacities,
            &mut findings,
            &mut evidence_complete,
        )?;

        evaluate_dependencies(
            &input.dependencies,
            &mut findings,
            &mut evidence_complete,
        );

        findings.sort_by(|left, right| {
            left.kind
                .cmp(&right.kind)
                .then_with(|| {
                    left.severity.cmp(&right.severity)
                })
                .then_with(|| {
                    left.related_id.cmp(&right.related_id)
                })
                .then_with(|| {
                    left.message.cmp(&right.message)
                })
        });

        let status = determine_status(
            &findings,
            evidence_complete,
        );

        Ok(FeasibilityReport {
            realization: input.realization.id.clone(),
            status,
            findings,
            resources,
            evidence_complete,
        })
    }

    /// Analyzes multiple caller-supplied alternatives.
    ///
    /// No targets are discovered or invented.
    ///
    /// Reports are returned in deterministic realization identity order.
    pub fn analyze_alternatives(
        &self,
        inputs: &[FeasibilityInput],
    ) -> Result<Vec<FeasibilityReport>, FeasibilityError> {
        let mut reports = Vec::new();

        reports
            .try_reserve(inputs.len())
            .map_err(|_| FeasibilityError::AllocationFailure)?;

        for input in inputs {
            reports.push(self.analyze(input)?);
        }

        reports.sort_by(|left, right| {
            left.realization.cmp(&right.realization)
        });

        Ok(reports)
    }
}

// ============================================================================
// REALIZATION EVALUATION
// ============================================================================

fn evaluate_realization(
    realization: &RealizationEvidence,
    findings: &mut Vec<FeasibilityFinding>,
    evidence_complete: &mut bool,
) {
    let checks = [
        (
            realization.capabilities,
            FindingKind::UnsupportedCapability,
            "Required capabilities are unsupported",
        ),
        (
            realization.backend_compatibility,
            FindingKind::BackendIncompatibility,
            "Backend compatibility is not established",
        ),
        (
            realization.data_movement,
            FindingKind::DataMovementIncompatibility,
            "Required data movement is unsupported",
        ),
        (
            realization.synchronization,
            FindingKind::SynchronizationIncompatibility,
            "Required synchronization is unsupported",
        ),
    ];

    for (state, kind, message) in checks {
        match state {
            EvidenceState::Satisfied => {}

            EvidenceState::Unsatisfied => {
                findings.push(
                    FeasibilityFinding::new(
                        kind,
                        FindingSeverity::Error,
                        message,
                    )
                    .with_related_id(
                        realization.id.0.clone(),
                    ),
                );
            }

            EvidenceState::Unknown => {
                *evidence_complete = false;

                findings.push(
                    FeasibilityFinding::new(
                        FindingKind::MissingEvidence,
                        FindingSeverity::Warning,
                        format!(
                            "Evidence is missing: {message}"
                        ),
                    )
                    .with_related_id(
                        realization.id.0.clone(),
                    ),
                );
            }
        }
    }

    if !realization.evidence_complete {
        *evidence_complete = false;
    }
}

// ============================================================================
// REQUIREMENT EVALUATION
// ============================================================================

fn evaluate_requirements(
    requirements: &[RequirementEvidence],
    findings: &mut Vec<FeasibilityFinding>,
    evidence_complete: &mut bool,
) {
    for requirement in requirements {
        match requirement.state {
            EvidenceState::Satisfied => {}

            EvidenceState::Unsatisfied => {
                let (kind, severity) = match requirement.class {
                    RequirementClass::HardRequirement => (
                        FindingKind::RequirementViolation,
                        FindingSeverity::Error,
                    ),

                    RequirementClass::Constraint => (
                        FindingKind::ConstraintViolation,
                        FindingSeverity::Error,
                    ),

                    RequirementClass::Preference => (
                        FindingKind::PreferenceNotMet,
                        FindingSeverity::Warning,
                    ),

                    RequirementClass::Hint => (
                        FindingKind::HintUnavailable,
                        FindingSeverity::Information,
                    ),
                };

                findings.push(
                    FeasibilityFinding::new(
                        kind,
                        severity,
                        requirement.explanation.clone()
                            .unwrap_or_else(|| {
                                format!(
                                    "Condition `{}` is not satisfied",
                                    requirement.id
                                )
                            }),
                    )
                    .with_related_id(
                        requirement.id.clone(),
                    ),
                );
            }

            EvidenceState::Unknown => {
                *evidence_complete = false;

                findings.push(
                    FeasibilityFinding::new(
                        FindingKind::MissingEvidence,
                        FindingSeverity::Warning,
                        requirement.explanation.clone()
                            .unwrap_or_else(|| {
                                format!(
                                    "Evidence for `{}` is unresolved",
                                    requirement.id
                                )
                            }),
                    )
                    .with_related_id(
                        requirement.id.clone(),
                    ),
                );
            }
        }
    }
}

// ============================================================================
// RESOURCE EVALUATION
// ============================================================================

fn evaluate_resources(
    demands: &[ResourceDemand],
    capacities: &[ResourceCapacity],
    findings: &mut Vec<FeasibilityFinding>,
    evidence_complete: &mut bool,
) -> Result<Vec<ResourceFinding>, FeasibilityError> {
    let mut capacity_by_id = std::collections::BTreeMap::new();

    for capacity in capacities {
        if capacity_by_id
            .insert(
                capacity.resource.clone(),
                capacity.available.clone(),
            )
            .is_some()
        {
            return Err(
                FeasibilityError::DuplicateResourceCapacity {
                    resource: capacity.resource.clone(),
                },
            );
        }
    }

    let mut results = Vec::new();

    results
        .try_reserve(demands.len())
        .map_err(|_| FeasibilityError::AllocationFailure)?;

    for demand in demands {
        let available = capacity_by_id
            .get(&demand.resource)
            .cloned();

        let state = match &available {
            Some(capacity) if demand.required <= *capacity => {
                EvidenceState::Satisfied
            }

            Some(_) => EvidenceState::Unsatisfied,

            None => EvidenceState::Unknown,
        };

        let explanation = match &available {
            Some(capacity)
                if state == EvidenceState::Satisfied =>
            {
                format!(
                    "Resource `{}` requires {}; capacity is {}",
                    demand.resource,
                    demand.required,
                    capacity
                )
            }

            Some(capacity) => {
                format!(
                    "Resource `{}` requires {}, but only {} is available",
                    demand.resource,
                    demand.required,
                    capacity
                )
            }

            None => {
                format!(
                    "Capacity evidence for `{}` is unavailable",
                    demand.resource
                )
            }
        };

        results.push(ResourceFinding {
            resource: demand.resource.clone(),
            required: demand.required.clone(),
            available: available.clone(),
            state,
            explanation: explanation.clone(),
        });

        match state {
            EvidenceState::Satisfied => {}

            EvidenceState::Unsatisfied => {
                findings.push(
                    FeasibilityFinding::new(
                        FindingKind::ResourceInsufficiency,
                        if demand.mandatory {
                            FindingSeverity::Error
                        } else {
                            FindingSeverity::Warning
                        },
                        explanation,
                    )
                    .with_related_id(
                        Identifier::new(
                            demand.resource.as_str(),
                        )?,
                    ),
                );
            }

            EvidenceState::Unknown => {
                *evidence_complete = false;

                findings.push(
                    FeasibilityFinding::new(
                        FindingKind::MissingEvidence,
                        FindingSeverity::Warning,
                        explanation,
                    )
                    .with_related_id(
                        Identifier::new(
                            demand.resource.as_str(),
                        )?,
                    ),
                );
            }
        }
    }

    Ok(results)
}

// ============================================================================
// DEPENDENCY EVALUATION
// ============================================================================

fn evaluate_dependencies(
    dependencies: &[DependencyFinding],
    findings: &mut Vec<FeasibilityFinding>,
    evidence_complete: &mut bool,
) {
    for dependency in dependencies {
        match dependency.state {
            EvidenceState::Satisfied => {}

            EvidenceState::Unsatisfied => {
                findings.push(
                    FeasibilityFinding::new(
                        FindingKind::DependencyViolation,
                        FindingSeverity::Error,
                        dependency.explanation.clone()
                            .unwrap_or_else(|| {
                                format!(
                                    "Dependency `{}` is not satisfied",
                                    dependency.id
                                )
                            }),
                    )
                    .with_related_id(
                        dependency.id.clone(),
                    ),
                );
            }

            EvidenceState::Unknown => {
                *evidence_complete = false;

                findings.push(
                    FeasibilityFinding::new(
                        FindingKind::MissingEvidence,
                        FindingSeverity::Warning,
                        dependency.explanation.clone()
                            .unwrap_or_else(|| {
                                format!(
                                    "Dependency `{}` is unresolved",
                                    dependency.id
                                )
                            }),
                    )
                    .with_related_id(
                        dependency.id.clone(),
                    ),
                );
            }
        }
    }
}

// ============================================================================
// FINAL CLASSIFICATION
// ============================================================================

fn determine_status(
    findings: &[FeasibilityFinding],
    evidence_complete: bool,
) -> FeasibilityStatus {
    let infeasible = findings.iter().any(|finding| {
        finding.severity == FindingSeverity::Error
            && !matches!(
                finding.kind,
                FindingKind::MissingEvidence
                    | FindingKind::BudgetExhaustion
                    | FindingKind::EvaluationFailure
            )
    });

    if infeasible {
        return FeasibilityStatus::Infeasible;
    }

    let incomplete = !evidence_complete
        || findings.iter().any(|finding| {
            matches!(
                finding.kind,
                FindingKind::MissingEvidence
                    | FindingKind::BudgetExhaustion
                    | FindingKind::EvaluationFailure
            )
        });

    if incomplete {
        FeasibilityStatus::Indeterminate
    } else {
        FeasibilityStatus::Feasible
    }
}

// ============================================================================
// VALIDATION
// ============================================================================

fn validate_input(
    input: &FeasibilityInput,
) -> Result<(), FeasibilityError> {
    let mut requirement_ids = BTreeSet::new();

    for requirement in &input.requirements {
        if !requirement_ids.insert(
            requirement.id.clone(),
        ) {
            return Err(
                FeasibilityError::DuplicateRequirement {
                    id: requirement.id.clone(),
                },
            );
        }
    }

    let mut dependency_ids = BTreeSet::new();

    for dependency in &input.dependencies {
        if !dependency_ids.insert(
            dependency.id.clone(),
        ) {
            return Err(
                FeasibilityError::DuplicateDependency {
                    id: dependency.id.clone(),
                },
            );
        }
    }

    for finding in &input.upstream_findings {
        if finding.message.trim().is_empty() {
            return Err(
                FeasibilityError::EmptyFindingMessage,
            );
        }
    }

    Ok(())
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by feasibility analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FeasibilityError {
    /// Identifier is empty.
    EmptyIdentifier,

    /// Identifier contains control characters.
    InvalidIdentifier {
        /// Invalid value.
        value: String,
    },

    /// Invalid decimal quantity.
    InvalidQuantity {
        /// Invalid value.
        value: String,
    },

    /// Duplicate requirement.
    DuplicateRequirement {
        /// Requirement identity.
        id: Identifier,
    },

    /// Duplicate dependency.
    DuplicateDependency {
        /// Dependency identity.
        id: Identifier,
    },

    /// Duplicate resource capacity.
    DuplicateResourceCapacity {
        /// Resource identity.
        resource: ResourceId,
    },

    /// Arithmetic failure.
    ArithmeticFailure,

    /// Allocation reservation failed.
    AllocationFailure,

    /// Finding message is empty.
    EmptyFindingMessage,
}

impl fmt::Display for FeasibilityError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => {
                write!(formatter, "identifier cannot be empty")
            }

            Self::InvalidIdentifier { value } => {
                write!(
                    formatter,
                    "invalid identifier: {value:?}"
                )
            }

            Self::InvalidQuantity { value } => {
                write!(
                    formatter,
                    "invalid resource quantity: {value:?}"
                )
            }

            Self::DuplicateRequirement { id } => {
                write!(
                    formatter,
                    "duplicate requirement: {id}"
                )
            }

            Self::DuplicateDependency { id } => {
                write!(
                    formatter,
                    "duplicate dependency: {id}"
                )
            }

            Self::DuplicateResourceCapacity { resource } => {
                write!(
                    formatter,
                    "duplicate resource capacity: {resource}"
                )
            }

            Self::ArithmeticFailure => {
                write!(formatter, "resource arithmetic failed")
            }

            Self::AllocationFailure => {
                write!(
                    formatter,
                    "available memory could not satisfy allocation"
                )
            }

            Self::EmptyFindingMessage => {
                write!(
                    formatter,
                    "finding message cannot be empty"
                )
            }
        }
    }
}

impl Error for FeasibilityError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn identifier(value: &str) -> Identifier {
        Identifier::new(value).unwrap()
    }

    fn quantity(value: &str) -> ResourceQuantity {
        ResourceQuantity::new(value).unwrap()
    }

    fn resource(value: &str) -> ResourceId {
        ResourceId::new(value).unwrap()
    }

    fn complete_realization() -> RealizationEvidence {
        RealizationEvidence {
            id: RealizationId::new("realization.test").unwrap(),
            capabilities: EvidenceState::Satisfied,
            backend_compatibility: EvidenceState::Satisfied,
            data_movement: EvidenceState::Satisfied,
            synchronization: EvidenceState::Satisfied,
            evidence_complete: true,
        }
    }

    #[test]
    fn complete_evidence_is_feasible() {
        let input = FeasibilityInput::new(
            complete_realization(),
        );

        let report = FeasibilityAnalyzer.analyze(&input).unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Feasible
        );
    }

    #[test]
    fn unknown_evidence_is_indeterminate() {
        let realization = RealizationEvidence::new(
            RealizationId::new("unknown").unwrap(),
        );

        let input = FeasibilityInput::new(realization);

        let report = FeasibilityAnalyzer.analyze(&input).unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Indeterminate
        );
    }

    #[test]
    fn unsupported_capability_is_infeasible() {
        let mut realization = complete_realization();

        realization.capabilities =
            EvidenceState::Unsatisfied;

        let input = FeasibilityInput::new(realization);

        let report = FeasibilityAnalyzer.analyze(&input).unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Infeasible
        );
    }

    #[test]
    fn unmet_preference_does_not_cause_infeasibility() {
        let mut input = FeasibilityInput::new(
            complete_realization(),
        );

        input.requirements.push(
            RequirementEvidence::new(
                identifier("execution.low_latency"),
                RequirementClass::Preference,
                EvidenceState::Unsatisfied,
            ),
        );

        let report = FeasibilityAnalyzer.analyze(&input).unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Feasible
        );

        assert!(report.has_unmet_preferences());
    }

    #[test]
    fn unmet_hard_requirement_is_infeasible() {
        let mut input = FeasibilityInput::new(
            complete_realization(),
        );

        input.requirements.push(
            RequirementEvidence::new(
                identifier("quantum.measurement"),
                RequirementClass::HardRequirement,
                EvidenceState::Unsatisfied,
            ),
        );

        let report = FeasibilityAnalyzer.analyze(&input).unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Infeasible
        );
    }

    #[test]
    fn satisfied_resource_demand_is_feasible() {
        let mut input = FeasibilityInput::new(
            complete_realization(),
        );

        input.resource_demands.push(ResourceDemand {
            resource: resource("memory.bytes"),
            required: quantity("1024"),
            mandatory: true,
        });

        input.resource_capacities.push(ResourceCapacity {
            resource: resource("memory.bytes"),
            available: quantity("2048"),
        });

        let report = FeasibilityAnalyzer.analyze(&input).unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Feasible
        );
    }

    #[test]
    fn insufficient_mandatory_resource_is_infeasible() {
        let mut input = FeasibilityInput::new(
            complete_realization(),
        );

        input.resource_demands.push(ResourceDemand {
            resource: resource("memory.bytes"),
            required: quantity("2048"),
            mandatory: true,
        });

        input.resource_capacities.push(ResourceCapacity {
            resource: resource("memory.bytes"),
            available: quantity("1024"),
        });

        let report = FeasibilityAnalyzer.analyze(&input).unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Infeasible
        );
    }

    #[test]
    fn missing_resource_capacity_is_not_unlimited() {
        let mut input = FeasibilityInput::new(
            complete_realization(),
        );

        input.resource_demands.push(ResourceDemand {
            resource: resource("quantum.logical_qubits"),
            required: quantity("100000000000000000000000"),
            mandatory: true,
        });

        let report = FeasibilityAnalyzer.analyze(&input).unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Indeterminate
        );
    }

    #[test]
    fn arbitrary_length_quantities_are_supported() {
        let huge = quantity(
            "999999999999999999999999999999999999999999999999999999",
        );

        assert_eq!(
            huge.as_str(),
            "999999999999999999999999999999999999999999999999999999"
        );
    }

    #[test]
    fn arbitrary_length_addition_is_correct() {
        let left = quantity("999999999999999999999999");
        let right = quantity("1");

        assert_eq!(
            left.checked_add(&right).unwrap().as_str(),
            "1000000000000000000000000"
        );
    }

    #[test]
    fn duplicate_capacity_is_rejected() {
        let mut input = FeasibilityInput::new(
            complete_realization(),
        );

        input.resource_capacities.push(ResourceCapacity {
            resource: resource("memory.bytes"),
            available: quantity("100"),
        });

        input.resource_capacities.push(ResourceCapacity {
            resource: resource("memory.bytes"),
            available: quantity("200"),
        });

        assert!(matches!(
            FeasibilityAnalyzer.analyze(&input),
            Err(FeasibilityError::DuplicateResourceCapacity { .. })
        ));
    }

    #[test]
    fn reports_preserve_distinct_resource_demands() {
        let mut input = FeasibilityInput::new(
            complete_realization(),
        );

        for required in ["10", "20"] {
            input.resource_demands.push(ResourceDemand {
                resource: resource("memory.bytes"),
                required: quantity(required),
                mandatory: true,
            });
        }

        input.resource_capacities.push(ResourceCapacity {
            resource: resource("memory.bytes"),
            available: quantity("100"),
        });

        let report = FeasibilityAnalyzer.analyze(&input).unwrap();

        assert_eq!(report.resources.len(), 2);
    }

    #[test]
    fn failed_dependency_is_infeasible() {
        let mut input = FeasibilityInput::new(
            complete_realization(),
        );

        input.dependencies.push(DependencyFinding {
            id: identifier("module.library"),
            state: EvidenceState::Unsatisfied,
            explanation: None,
        });

        let report = FeasibilityAnalyzer.analyze(&input).unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Infeasible
        );
    }

    #[test]
    fn alternatives_are_deterministically_ordered() {
        let first = RealizationEvidence::new(
            RealizationId::new("target.z").unwrap(),
        );

        let second = RealizationEvidence::new(
            RealizationId::new("target.a").unwrap(),
        );

        let reports = FeasibilityAnalyzer
            .analyze_alternatives(&[
                FeasibilityInput::new(first),
                FeasibilityInput::new(second),
            ])
            .unwrap();

        assert_eq!(
            reports[0].realization.as_str(),
            "target.a"
        );

        assert_eq!(
            reports[1].realization.as_str(),
            "target.z"
        );
    }

    #[test]
    fn empty_finding_messages_are_rejected() {
        let mut input = FeasibilityInput::new(
            complete_realization(),
        );

        input.upstream_findings.push(
            FeasibilityFinding::new(
                FindingKind::EvaluationFailure,
                FindingSeverity::Error,
                "",
            ),
        );

        assert!(matches!(
            FeasibilityAnalyzer.analyze(&input),
            Err(FeasibilityError::EmptyFindingMessage)
        ));
    }

    #[test]
    fn unknown_dependency_never_becomes_feasible() {
        let mut input = FeasibilityInput::new(
            complete_realization(),
        );

        input.dependencies.push(DependencyFinding {
            id: identifier("dependency.unknown"),
            state: EvidenceState::Unknown,
            explanation: None,
        });

        let report = FeasibilityAnalyzer.analyze(&input).unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Indeterminate
        );
    }
}
