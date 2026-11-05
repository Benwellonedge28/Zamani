#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Compiler — Feasibility Analysis
//!
//! Path: src/compiler/analysis/feasibility.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! This module determines whether a proposed compilation realization is
//! supported by the available capability evidence, resource evidence,
//! requirements, and constraints.
//!
//! It does not define source-language semantics, hardware capabilities,
//! resource models, target discovery, backend compatibility, or runtime
//! execution.
//!
//! Those responsibilities remain with their canonical repository owners.
//!
//! ## Architecture
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Frontend semantic analysis
//!      |
//!      v
//! Canonical requirements
//!      |
//!      +------------------------+
//!      |                        |
//!      v                        v
//! Capability evidence     Resource evidence
//!      |                        |
//!      +------------+-----------+
//!                   |
//!                   v
//!             Feasibility
//!                   |
//!          +--------+--------+
//!          |        |        |
//!          v        v        v
//!        Feasible Infeasible Unknown
//!                   |
//!                   v
//!         Compiler planning
//!                   |
//!                   v
//!          Backend realization
//! ```
//!
//! ## Important limitation
//!
//! Literal infinity cannot be processed by a finite computer.
//!
//! This module introduces no fixed semantic limit on the number of
//! requirements, capabilities, resources, constraints, or alternatives.
//! Actual processing is bounded by available memory, address space,
//! allocator behavior, and explicit operational budgets.
//!
//! ## Integration
//!
//! - Frontend semantic analysis owns semantic requirements.
//! - Compiler capability analysis owns capability compatibility.
//! - Resource authorities own resource descriptions and evidence.
//! - Dependency analysis owns dependency relationships.
//! - Backend compatibility owns backend-specific compatibility.
//! - Runtime owns physical allocation and execution.
//!
//! This module combines externally supplied evidence without redefining
//! those authorities.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

// =============================================================================
// Module identity
// =============================================================================

/// Stable architectural identity of this module.
pub const FEASIBILITY_ANALYSIS_MODULE_NAME: &str =
    "zamani.compiler.analysis.feasibility";

/// Version of this module's in-memory report contract.
///
/// This is not the Zamani language version or artifact format version.
pub const FEASIBILITY_ANALYSIS_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Open identifiers
// =============================================================================

/// An open, namespaced identifier.
///
/// This type deliberately does not use a closed enum of hardware,
/// computing domains, or vendors.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Identifier(String);

impl Identifier {
    /// Creates a validated identifier.
    ///
    /// Identifiers must be non-empty and must not contain control
    /// characters.
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

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Evidence
// =============================================================================

/// Evidence state.
///
/// `Unknown` means that the evidence is insufficient to establish either
/// satisfaction or incompatibility.
///
/// It must never be treated as proof of feasibility.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EvidenceState {
    /// Evidence establishes that the property is satisfied.
    Satisfied,

    /// Evidence establishes that the property is not satisfied.
    Unsatisfied,

    /// Evidence is incomplete or unavailable.
    Unknown,
}

impl EvidenceState {
    /// Returns whether evidence establishes satisfaction.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied)
    }

    /// Returns whether evidence establishes incompatibility.
    #[must_use]
    pub const fn is_unsatisfied(self) -> bool {
        matches!(self, Self::Unsatisfied)
    }

    /// Returns whether additional evidence is needed.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

// =============================================================================
// Requirement classification
// =============================================================================

/// Importance of a feasibility condition.
///
/// This type is a compiler-facing classification, not a replacement
/// for the canonical semantic requirement model.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RequirementClass {
    /// Must be satisfied for a valid realization.
    HardRequirement,

    /// Must be obeyed when applicable.
    Constraint,

    /// Desirable but not mandatory.
    Preference,

    /// Advisory information that may guide planning.
    Hint,
}

impl RequirementClass {
    /// Returns whether failure of this condition prevents feasibility.
    #[must_use]
    pub const fn is_mandatory(self) -> bool {
        matches!(
            self,
            Self::HardRequirement | Self::Constraint
        )
    }
}

// =============================================================================
// Requirement evidence
// =============================================================================

/// One requirement assessment.
///
/// The requirement's semantic meaning remains owned by its originating
/// semantic authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequirementEvidence {
    /// Stable requirement identifier.
    pub id: Identifier,

    /// Requirement classification.
    pub class: RequirementClass,

    /// Evidence of satisfaction.
    pub state: EvidenceState,

    /// Human-readable explanation.
    pub explanation: Option<String>,
}

impl RequirementEvidence {
    /// Creates a requirement assessment.
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

// =============================================================================
// Resource quantities
// =============================================================================

/// An exact non-negative integer quantity.
///
/// A decimal string avoids introducing a fixed integer-width ceiling
/// into resource quantities.
///
/// Resource arithmetic is performed using checked operations.
/// Overflow is reported instead of silently wrapping.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ResourceQuantity(String);

impl ResourceQuantity {
    /// Creates a canonical quantity.
    ///
    /// Leading zeroes are removed.
    pub fn new(value: impl Into<String>) -> Result<Self, FeasibilityError> {
        let value = value.into();

        if value.is_empty()
            || !value.chars().all(|c| c.is_ascii_digit())
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

    /// Returns the canonical quantity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns whether the quantity is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.0 == "0"
    }

    /// Compares two quantities without converting them to a machine
    /// integer.
    #[must_use]
    pub fn cmp_quantity(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .len()
            .cmp(&other.0.len())
            .then_with(|| self.0.cmp(&other.0))
    }
}

impl fmt::Display for ResourceQuantity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Resource evidence
// =============================================================================

/// One abstract resource dimension.
///
/// Examples include memory, processing capacity, storage, qubits,
/// communication bandwidth, and domain-specific resource dimensions.
///
/// This is an open identifier rather than a closed resource enum.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResourceId(Identifier);

impl ResourceId {
    /// Creates a resource identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, FeasibilityError> {
        Ok(Self(Identifier::new(value)?))
    }

    /// Returns the resource identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for ResourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Demand for one abstract resource.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceDemand {
    /// Resource dimension.
    pub resource: ResourceId,

    /// Required quantity.
    pub required: ResourceQuantity,

    /// Whether the demand is mandatory.
    pub mandatory: bool,
}

/// Available capacity for one resource.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceCapacity {
    /// Resource dimension.
    pub resource: ResourceId,

    /// Available quantity.
    pub available: ResourceQuantity,
}

// =============================================================================
// Resource feasibility
// =============================================================================

/// Result of checking one resource dimension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceFinding {
    /// Resource dimension.
    pub resource: ResourceId,

    /// Required quantity.
    pub required: ResourceQuantity,

    /// Available quantity, if known.
    pub available: Option<ResourceQuantity>,

    /// Evidence state.
    pub state: EvidenceState,

    /// Explanation.
    pub explanation: String,
}

// =============================================================================
// Target and realization
// =============================================================================

/// An opaque realization identifier.
///
/// This may refer to a backend, a logical target, a composite target,
/// or a target-independent realization strategy.
///
/// It is not a physical device identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RealizationId(Identifier);

impl RealizationId {
    /// Creates a realization identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, FeasibilityError> {
        Ok(Self(Identifier::new(value)?))
    }

    /// Returns the realization identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for RealizationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Evidence describing one proposed realization.
///
/// The compiler receives this information from capability analysis,
/// resource analysis, and backend compatibility checks.
///
/// No hardware is discovered or allocated by this structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealizationEvidence {
    /// Realization identity.
    pub id: RealizationId,

    /// Whether capability requirements are satisfied.
    pub capabilities: EvidenceState,

    /// Whether backend compatibility is established.
    pub backend_compatibility: EvidenceState,

    /// Whether required data movement is supported.
    pub data_movement: EvidenceState,

    /// Whether required synchronization is supported.
    pub synchronization: EvidenceState,

    /// Whether the supplied evidence is complete.
    pub evidence_complete: bool,
}

impl RealizationEvidence {
    /// Creates realization evidence.
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

    /// Returns the combined compatibility state.
    ///
    /// An explicit incompatibility dominates.
    ///
    /// Unknown evidence prevents a positive conclusion.
    #[must_use]
    pub fn compatibility_state(&self) -> EvidenceState {
        let states = [
            self.capabilities,
            self.backend_compatibility,
            self.data_movement,
            self.synchronization,
        ];

        if states
            .iter()
            .any(|state| state.is_unsatisfied())
        {
            EvidenceState::Unsatisfied
        } else if states
            .iter()
            .all(|state| state.is_satisfied())
            && self.evidence_complete
        {
            EvidenceState::Satisfied
        } else {
            EvidenceState::Unknown
        }
    }
}

// =============================================================================
// Dependency evidence
// =============================================================================

/// Dependency relationship relevant to feasibility.
///
/// Dependency discovery and dependency ordering remain owned by
/// `src/compiler/analysis/dependency.rs`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyFinding {
    /// Stable dependency identifier.
    pub id: Identifier,

    /// Whether the dependency is satisfied.
    pub state: EvidenceState,

    /// Explanation.
    pub explanation: Option<String>,
}

// =============================================================================
// Finding classification
// =============================================================================

/// Category of a feasibility finding.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FindingKind {
    /// A mandatory requirement is not satisfied.
    RequirementViolation,

    /// A mandatory constraint is not satisfied.
    ConstraintViolation,

    /// A preference is not satisfied.
    PreferenceNotMet,

    /// A hint cannot be applied.
    HintUnavailable,

    /// A capability is unsupported.
    UnsupportedCapability,

    /// Backend compatibility is not established.
    BackendIncompatibility,

    /// Resource capacity is insufficient.
    ResourceInsufficiency,

    /// Required evidence is unavailable.
    MissingEvidence,

    /// A dependency is not satisfied.
    DependencyViolation,

    /// Data movement cannot be realized.
    DataMovementIncompatibility,

    /// Synchronization cannot be realized.
    SynchronizationIncompatibility,

    /// An operational budget has been exhausted.
    BudgetExhaustion,

    /// An internal feasibility evaluation failed.
    EvaluationFailure,
}

/// Severity of a feasibility finding.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FindingSeverity {
    /// Informational.
    Information,

    /// Advisory.
    Warning,

    /// Mandatory condition cannot be established.
    Error,
}

/// One structured feasibility finding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeasibilityFinding {
    /// Finding category.
    pub kind: FindingKind,

    /// Severity.
    pub severity: FindingSeverity,

    /// Related identifier, when applicable.
    pub related_id: Option<Identifier>,

    /// Human-readable message.
    pub message: String,
}

impl FeasibilityFinding {
    /// Creates a structured finding.
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

    /// Associates the finding with an identifier.
    #[must_use]
    pub fn with_related_id(
        mut self,
        related_id: Identifier,
    ) -> Self {
        self.related_id = Some(related_id);
        self
    }
}

// =============================================================================
// Feasibility outcome
// =============================================================================

/// Overall feasibility outcome.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FeasibilityStatus {
    /// All mandatory conditions are established as satisfied.
    Feasible,

    /// At least one mandatory condition is explicitly unsatisfied.
    Infeasible,

    /// No mandatory violation was established, but evidence is incomplete.
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

    /// Returns whether further evidence is needed.
    #[must_use]
    pub const fn is_indeterminate(self) -> bool {
        matches!(self, Self::Indeterminate)
    }
}

// =============================================================================
// Feasibility report
// =============================================================================

/// Complete feasibility report for one realization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeasibilityReport {
    /// Realization identity.
    pub realization: RealizationId,

    /// Overall outcome.
    pub status: FeasibilityStatus,

    /// Structured findings.
    pub findings: Vec<FeasibilityFinding>,

    /// Resource assessments.
    pub resources: Vec<ResourceFinding>,

    /// Whether all relevant evidence was complete.
    pub evidence_complete: bool,
}

impl FeasibilityReport {
    /// Returns whether this realization is feasible.
    #[must_use]
    pub const fn is_feasible(&self) -> bool {
        self.status.is_feasible()
    }

    /// Returns whether this realization is infeasible.
    #[must_use]
    pub const fn is_infeasible(&self) -> bool {
        self.status.is_infeasible()
    }

    /// Returns whether the result is indeterminate.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        self.status.is_indeterminate()
    }

    /// Returns the number of findings.
    #[must_use]
    pub fn finding_count(&self) -> usize {
        self.findings.len()
    }
}

// =============================================================================
// Input
// =============================================================================

/// All evidence required to assess one realization.
///
/// This is a compiler-side aggregation structure. It does not own the
/// canonical meaning of the supplied requirements or evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeasibilityInput {
    /// Proposed realization.
    pub realization: RealizationEvidence,

    /// Requirement evidence.
    pub requirements: Vec<RequirementEvidence>,

    /// Resource demands.
    pub resource_demands: Vec<ResourceDemand>,

    /// Resource capacity evidence.
    pub resource_capacities: Vec<ResourceCapacity>,

    /// Dependency evidence.
    pub dependencies: Vec<DependencyFinding>,

    /// Additional structured findings supplied by upstream analysis.
    pub upstream_findings: Vec<FeasibilityFinding>,
}

impl FeasibilityInput {
    /// Creates an input for a realization.
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

// =============================================================================
// Engine
// =============================================================================

/// Stateless feasibility-analysis engine.
#[derive(Clone, Copy, Debug, Default)]
pub struct FeasibilityAnalyzer;

impl FeasibilityAnalyzer {
    /// Analyzes one realization.
    ///
    /// The analyzer does not allocate resources, discover hardware,
    /// schedule execution, or generate backend code.
    pub fn analyze(
        &self,
        input: &FeasibilityInput,
    ) -> Result<FeasibilityReport, FeasibilityError> {
        validate_input(input)?;

        let mut findings = input.upstream_findings.clone();

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
                .then_with(|| left.severity.cmp(&right.severity))
                .then_with(|| {
                    left.related_id.cmp(&right.related_id)
                })
                .then_with(|| left.message.cmp(&right.message))
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

    /// Analyzes multiple candidate realizations.
    ///
    /// The caller supplies the candidates. This function does not
    /// perform target discovery or manufacture alternatives.
    ///
    /// Results are sorted by realization identity for deterministic output.
    pub fn analyze_alternatives(
        &self,
        inputs: &[FeasibilityInput],
    ) -> Result<Vec<FeasibilityReport>, FeasibilityError> {
        let mut reports = inputs
            .iter()
            .map(|input| self.analyze(input))
            .collect::<Result<Vec<_>, _>>()?;

        reports.sort_by(|left, right| {
            left.realization.cmp(&right.realization)
        });

        Ok(reports)
    }
}

// =============================================================================
// Evaluation
// =============================================================================

fn evaluate_realization(
    realization: &RealizationEvidence,
    findings: &mut Vec<FeasibilityFinding>,
    evidence_complete: &mut bool,
) {
    let state = realization.compatibility_state();

    match state {
        EvidenceState::Satisfied => {}

        EvidenceState::Unsatisfied => {
            if realization.capabilities.is_unsatisfied() {
                findings.push(
                    FeasibilityFinding::new(
                        FindingKind::UnsupportedCapability,
                        FindingSeverity::Error,
                        "The realization does not satisfy required capabilities",
                    )
                    .with_related_id(
                        realization.id.0.clone(),
                    ),
                );
            }

            if realization
                .backend_compatibility
                .is_unsatisfied()
            {
                findings.push(
                    FeasibilityFinding::new(
                        FindingKind::BackendIncompatibility,
                        FindingSeverity::Error,
                        "The backend cannot realize the requested computation",
                    )
                    .with_related_id(
                        realization.id.0.clone(),
                    ),
                );
            }

            if realization.data_movement.is_unsatisfied() {
                findings.push(
                    FeasibilityFinding::new(
                        FindingKind::DataMovementIncompatibility,
                        FindingSeverity::Error,
                        "Required data movement is unsupported",
                    )
                    .with_related_id(
                        realization.id.0.clone(),
                    ),
                );
            }

            if realization.synchronization.is_unsatisfied() {
                findings.push(
                    FeasibilityFinding::new(
                        FindingKind::SynchronizationIncompatibility,
                        FindingSeverity::Error,
                        "Required synchronization is unsupported",
                    )
                    .with_related_id(
                        realization.id.0.clone(),
                    ),
                );
            }
        }

        EvidenceState::Unknown => {
            *evidence_complete = false;

            findings.push(
                FeasibilityFinding::new(
                    FindingKind::MissingEvidence,
                    FindingSeverity::Warning,
                    "Realization compatibility is not fully established",
                )
                .with_related_id(
                    realization.id.0.clone(),
                ),
            );
        }
    }
}

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
                        requirement
                            .explanation
                            .clone()
                            .unwrap_or_else(|| {
                                format!(
                                    "Requirement `{}` is not satisfied",
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

                let severity =
                    if requirement.class.is_mandatory() {
                        FindingSeverity::Error
                    } else {
                        FindingSeverity::Warning
                    };

                findings.push(
                    FeasibilityFinding::new(
                        FindingKind::MissingEvidence,
                        severity,
                        requirement
                            .explanation
                            .clone()
                            .unwrap_or_else(|| {
                                format!(
                                    "Evidence for requirement `{}` is incomplete",
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

fn evaluate_resources(
    demands: &[ResourceDemand],
    capacities: &[ResourceCapacity],
    findings: &mut Vec<FeasibilityFinding>,
    evidence_complete: &mut bool,
) -> Result<Vec<ResourceFinding>, FeasibilityError> {
    let mut capacity_by_id = BTreeMap::new();

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

    let mut findings_by_resource = BTreeMap::new();

    for demand in demands {
        let available = capacity_by_id
            .get(&demand.resource)
            .cloned();

        let state = match &available {
            Some(available) => {
                if demand.required.cmp_quantity(available)
                    != std::cmp::Ordering::Greater
                {
                    EvidenceState::Satisfied
                } else {
                    EvidenceState::Unsatisfied
                }
            }

            None => EvidenceState::Unknown,
        };

        let explanation = match state {
            EvidenceState::Satisfied => {
                "Resource demand is within available capacity".to_owned()
            }

            EvidenceState::Unsatisfied => {
                format!(
                    "Resource `{}` requires {}, but only {} is available",
                    demand.resource,
                    demand.required,
                    available.as_ref().unwrap()
                )
            }

            EvidenceState::Unknown => {
                format!(
                    "Resource capacity for `{}` is unknown",
                    demand.resource
                )
            }
        };

        let finding = ResourceFinding {
            resource: demand.resource.clone(),
            required: demand.required.clone(),
            available,
            state,
            explanation: explanation.clone(),
        };

        match state {
            EvidenceState::Satisfied => {}

            EvidenceState::Unsatisfied => {
                if demand.mandatory {
                    findings.push(
                        FeasibilityFinding::new(
                            FindingKind::ResourceInsufficiency,
                            FindingSeverity::Error,
                            explanation,
                        )
                        .with_related_id(
                            Identifier::new(
                                demand.resource.as_str(),
                            )?,
                        ),
                    );
                } else {
                    findings.push(
                        FeasibilityFinding::new(
                            FindingKind::ResourceInsufficiency,
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

            EvidenceState::Unknown => {
                *evidence_complete = false;

                findings.push(
                    FeasibilityFinding::new(
                        FindingKind::MissingEvidence,
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
        }

        findings_by_resource.insert(
            demand.resource.clone(),
            finding,
        );
    }

    Ok(findings_by_resource.into_values().collect())
}

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
                        dependency
                            .explanation
                            .clone()
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
                        dependency
                            .explanation
                            .clone()
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

fn determine_status(
    findings: &[FeasibilityFinding],
    evidence_complete: bool,
) -> FeasibilityStatus {
    if findings.iter().any(|finding| {
        finding.severity == FindingSeverity::Error
            && matches!(
                finding.kind,
                FindingKind::RequirementViolation
                    | FindingKind::ConstraintViolation
                    | FindingKind::UnsupportedCapability
                    | FindingKind::BackendIncompatibility
                    | FindingKind::ResourceInsufficiency
                    | FindingKind::DependencyViolation
                    | FindingKind::DataMovementIncompatibility
                    | FindingKind::SynchronizationIncompatibility
            )
    }) {
        FeasibilityStatus::Infeasible
    } else if evidence_complete {
        FeasibilityStatus::Feasible
    } else {
        FeasibilityStatus::Indeterminate
    }
}

// =============================================================================
// Input validation
// =============================================================================

fn validate_input(
    input: &FeasibilityInput,
) -> Result<(), FeasibilityError> {
    let mut requirement_ids = BTreeMap::new();

    for requirement in &input.requirements {
        if requirement_ids
            .insert(requirement.id.clone(), ())
            .is_some()
        {
            return Err(
                FeasibilityError::DuplicateRequirement {
                    id: requirement.id.clone(),
                },
            );
        }
    }

    let mut resource_ids = BTreeMap::new();

    for demand in &input.resource_demands {
        if resource_ids
            .insert(demand.resource.clone(), ())
            .is_some()
        {
            return Err(
                FeasibilityError::DuplicateResourceDemand {
                    resource: demand.resource.clone(),
                },
            );
        }
    }

    Ok(())
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by feasibility analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FeasibilityError {
    /// An identifier was empty.
    EmptyIdentifier,

    /// An identifier contained invalid characters.
    InvalidIdentifier {
        /// Invalid identifier.
        value: String,
    },

    /// A resource quantity was malformed.
    InvalidQuantity {
        /// Invalid quantity.
        value: String,
    },

    /// A requirement was duplicated.
    DuplicateRequirement {
        /// Duplicate requirement.
        id: Identifier,
    },

    /// A resource demand was duplicated.
    DuplicateResourceDemand {
        /// Duplicate resource.
        resource: ResourceId,
    },

    /// A resource capacity was duplicated.
    DuplicateResourceCapacity {
        /// Duplicate resource.
        resource: ResourceId,
    },
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

            Self::DuplicateResourceDemand { resource } => {
                write!(
                    formatter,
                    "duplicate resource demand: {resource}"
                )
            }

            Self::DuplicateResourceCapacity { resource } => {
                write!(
                    formatter,
                    "duplicate resource capacity: {resource}"
                )
            }
        }
    }
}

impl Error for FeasibilityError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> Identifier {
        Identifier::new(value).unwrap()
    }

    fn quantity(value: &str) -> ResourceQuantity {
        ResourceQuantity::new(value).unwrap()
    }

    fn resource(value: &str) -> ResourceId {
        ResourceId::new(value).unwrap()
    }

    fn realization() -> RealizationEvidence {
        let mut evidence = RealizationEvidence::new(
            RealizationId::new("portable.cpu").unwrap(),
        );

        evidence.capabilities = EvidenceState::Satisfied;
        evidence.backend_compatibility = EvidenceState::Satisfied;
        evidence.data_movement = EvidenceState::Satisfied;
        evidence.synchronization = EvidenceState::Satisfied;
        evidence.evidence_complete = true;

        evidence
    }

    #[test]
    fn feasible_realization_is_reported() {
        let input = FeasibilityInput::new(realization());

        let report = FeasibilityAnalyzer
            .analyze(&input)
            .unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Feasible
        );
    }

    #[test]
    fn unsupported_capability_is_infeasible() {
        let mut evidence = realization();

        evidence.capabilities =
            EvidenceState::Unsatisfied;

        let input = FeasibilityInput::new(evidence);

        let report = FeasibilityAnalyzer
            .analyze(&input)
            .unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Infeasible
        );
    }

    #[test]
    fn unknown_evidence_is_indeterminate() {
        let mut evidence = realization();

        evidence.capabilities =
            EvidenceState::Unknown;

        evidence.evidence_complete = false;

        let input = FeasibilityInput::new(evidence);

        let report = FeasibilityAnalyzer
            .analyze(&input)
            .unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Indeterminate
        );
    }

    #[test]
    fn failed_preference_does_not_make_program_infeasible() {
        let mut input =
            FeasibilityInput::new(realization());

        input.requirements.push(
            RequirementEvidence::new(
                id("optimization.low_latency"),
                RequirementClass::Preference,
                EvidenceState::Unsatisfied,
            ),
        );

        let report = FeasibilityAnalyzer
            .analyze(&input)
            .unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Feasible
        );
    }

    #[test]
    fn failed_hard_requirement_is_infeasible() {
        let mut input =
            FeasibilityInput::new(realization());

        input.requirements.push(
            RequirementEvidence::new(
                id("quantum.measurement"),
                RequirementClass::HardRequirement,
                EvidenceState::Unsatisfied,
            ),
        );

        let report = FeasibilityAnalyzer
            .analyze(&input)
            .unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Infeasible
        );
    }

    #[test]
    fn sufficient_resource_capacity_is_feasible() {
        let mut input =
            FeasibilityInput::new(realization());

        input.resource_demands.push(
            ResourceDemand {
                resource: resource("memory.bytes"),
                required: quantity("1024"),
                mandatory: true,
            },
        );

        input.resource_capacities.push(
            ResourceCapacity {
                resource: resource("memory.bytes"),
                available: quantity("2048"),
            },
        );

        let report = FeasibilityAnalyzer
            .analyze(&input)
            .unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Feasible
        );
    }

    #[test]
    fn insufficient_resource_capacity_is_infeasible() {
        let mut input =
            FeasibilityInput::new(realization());

        input.resource_demands.push(
            ResourceDemand {
                resource: resource("memory.bytes"),
                required: quantity("2048"),
                mandatory: true,
            },
        );

        input.resource_capacities.push(
            ResourceCapacity {
                resource: resource("memory.bytes"),
                available: quantity("1024"),
            },
        );

        let report = FeasibilityAnalyzer
            .analyze(&input)
            .unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Infeasible
        );
    }

    #[test]
    fn unknown_resource_capacity_is_indeterminate() {
        let mut input =
            FeasibilityInput::new(realization());

        input.resource_demands.push(
            ResourceDemand {
                resource: resource("quantum.logical_qubits"),
                required: quantity("100000000000000000000000"),
                mandatory: true,
            },
        );

        let report = FeasibilityAnalyzer
            .analyze(&input)
            .unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Indeterminate
        );
    }

    #[test]
    fn arbitrary_resource_quantities_are_supported() {
        let huge = quantity(
            "999999999999999999999999999999999999999999999999999999",
        );

        assert_eq!(
            huge.as_str(),
            "999999999999999999999999999999999999999999999999999999"
        );
    }

    #[test]
    fn alternative_reports_are_deterministic() {
        let inputs = vec![
            FeasibilityInput::new(
                RealizationEvidence::new(
                    RealizationId::new("target.z").unwrap(),
                ),
            ),
            FeasibilityInput::new(
                RealizationEvidence::new(
                    RealizationId::new("target.a").unwrap(),
                ),
            ),
        ];

        let reports = FeasibilityAnalyzer
            .analyze_alternatives(&inputs)
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
    fn unknown_evidence_never_becomes_feasible() {
        let mut input =
            FeasibilityInput::new(realization());

        input.dependencies.push(
            DependencyFinding {
                id: id("dependency.module"),
                state: EvidenceState::Unknown,
                explanation: None,
            },
        );

        let report = FeasibilityAnalyzer
            .analyze(&input)
            .unwrap();

        assert_eq!(
            report.status,
            FeasibilityStatus::Indeterminate
        );
    }
}