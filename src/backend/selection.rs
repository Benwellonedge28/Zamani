
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani backend target selection.
//!
//! Path: src/backend/selection.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Responsibility
//!
//! Select a target descriptor from explicitly supplied candidates by applying:
//!
//! 1. An optional exact target request.
//! 2. Implementation-availability policy.
//! 3. Required capability checks.
//! 4. Explicit target-property constraints.
//! 5. Preference ranking.
//! 6. Deterministic tie-breaking.
//!
//! Resource requirements are reported as deferred feasibility work. This
//! module does not claim that a target has sufficient physical resources.
//!
//! # Ownership boundaries
//!
//! - target.rs owns TargetDescriptor and TargetId.
//! - capabilities.rs owns capability identifiers, profiles, and matching.
//! - requirements.rs owns requirements, preferences, constraints, hints,
//!   resource requirements, and their provenance.
//! - registry.rs owns backend registration and implementation discovery.
//! - compiler orchestration owns verification and compilation ordering.
//! - domain IRs own classical, quantum, HDL, tensor, and other semantics.
//! - resource planners and runtimes own physical feasibility and allocation.
//!
//! This module does not parse source, inspect grammar productions, infer
//! source meaning, lower IR, invoke tools, allocate hardware, route quantum
//! circuits, schedule work, or silently substitute another target.
//!
//! # POCO-REAF invariants
//!
//! - A target's identity is distinct from its capabilities.
//! - Required capabilities cannot be downgraded to preferences.
//! - Unknown constraint properties fail closed.
//! - Unknown resource feasibility is not reported as proven feasibility.
//! - Unavailable or metadata-only targets are not treated as usable emitters
//!   when the request requires a usable implementation.
//! - An explicit target request never silently falls back to another target.
//! - Preference ranking never overrides a failed mandatory condition.
//! - Equal-scoring candidates are ordered by canonical target identifier.
//! - No universal hardware-capacity ceiling is imposed.
//!
//! # Integration
//!
//! The canonical backend module root should declare `pub mod selection;`.
//! Callers pass TargetDescriptor values created and validated by target.rs.
//! When implementation registration is required, use select_registered()
//! and supply an explicit target-ID-to-backend-ID mapping.
//!
//! Selection is not proof that an emitter preserves semantics. The compiler
//! must still verify its input, validate the selected target contract, lower
//! through the appropriate domain IR, and validate the produced artifact.

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;

use super::capabilities::{
    CapabilityOutcome, CapabilityPolicy, CapabilityRequirement,
};
use super::errors::{BackendError, BackendErrorKind, BackendResult};
use super::registry::{BackendRegistry, RegistryError};
use super::requirements::{
    BackendRequirement, DecimalQuantity, RequirementKind, RequirementOperator,
    RequirementSet, RequirementValue,
};
use super::target::{
    ImplementationAvailability, TargetDescriptor, TargetId,
};

// ---------------------------------------------------------------------------
// Public request and result contracts
// ---------------------------------------------------------------------------

/// One target candidate and its optional implementation registration.
///
/// `backend_id` is intentionally explicit. A target identifier such as
/// `x86-64-linux` is not necessarily identical to a legacy backend name such
/// as `x86_64`. Never infer this mapping from architecture or display name.
#[derive(Debug, Clone)]
pub struct TargetCandidate {
    /// Validated target metadata.
    pub target: TargetDescriptor,

    /// Canonical backend registry identifier, if a mapping is known.
    pub backend_id: Option<String>,
}

impl TargetCandidate {
    /// Create a candidate without asserting that an emitter is registered.
    pub fn new(target: TargetDescriptor) -> Self {
        Self {
            target,
            backend_id: None,
        }
    }

    /// Attach an explicit backend registry identifier.
    pub fn with_backend_id(mut self, backend_id: impl Into<String>) -> Self {
        self.backend_id = Some(backend_id.into());
        self
    }
}

/// Policy for selecting a target.
///
/// The default does not require a registered emitter, because selection can
/// also be used for planning and target discovery. Use
/// `select_registered()` when a real registered implementation is required.
#[derive(Debug, Clone)]
pub struct SelectionRequest {
    /// If present, only this exact target identifier may be selected.
    pub explicit_target: Option<TargetId>,

    /// Workload requirements from semantic analysis or compilation planning.
    pub requirements: RequirementSet,

    /// Controls whether experimental/deprecated capability declarations
    /// and declarations without direct evidence may satisfy requirements.
    pub capability_policy: CapabilityPolicy,

    /// Require an emitter implementation to be present and available.
    pub require_usable_implementation: bool,
}

impl Default for SelectionRequest {
    fn default() -> Self {
        Self {
            explicit_target: None,
            requirements: RequirementSet::new(),
            capability_policy: CapabilityPolicy::default(),
            require_usable_implementation: false,
        }
    }
}

impl SelectionRequest {
    /// Create a request with no explicit target and no workload requirements.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restrict selection to one exact target identifier.
    #[must_use]
    pub fn for_target(mut self, target: TargetId) -> Self {
        self.explicit_target = Some(target);
        self
    }

    /// Replace the workload requirement set.
    #[must_use]
    pub fn with_requirements(mut self, requirements: RequirementSet) -> Self {
        self.requirements = requirements;
        self
    }

    /// Set the capability-evidence and lifecycle policy.
    #[must_use]
    pub fn with_capability_policy(mut self, policy: CapabilityPolicy) -> Self {
        self.capability_policy = policy;
        self
    }

    /// Require the candidate to advertise a usable implementation.
    #[must_use]
    pub fn requiring_implementation(mut self, required: bool) -> Self {
        self.require_usable_implementation = required;
        self
    }
}

/// Why a candidate could not be selected.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RejectionReason {
    /// An explicit target request names a different target.
    NotExplicitlyRequested {
        requested: String,
    },

    /// No usable emitter is declared for this target.
    ImplementationUnavailable {
        availability: String,
    },

    /// A registered backend is required but no mapping was supplied.
    MissingBackendMapping,

    /// The mapped backend identifier is not registered.
    BackendNotRegistered {
        backend_id: String,
    },

    /// A required capability is missing, unavailable, or disallowed.
    CapabilityUnsatisfied {
        capability: String,
        outcome: String,
    },

    /// A mandatory target-property constraint was not satisfied.
    ConstraintUnsatisfied {
        key: String,
        reason: String,
    },

    /// The constraint refers to a property the selector cannot evaluate.
    UnknownConstraintProperty {
        key: String,
    },

    /// The descriptor failed validation.
    InvalidDescriptor {
        reason: String,
    },
}

impl fmt::Display for RejectionReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotExplicitlyRequested { requested } => {
                write!(f, "target was not explicitly requested: {requested}")
            }
            Self::ImplementationUnavailable { availability } => {
                write!(f, "implementation is not usable: {availability}")
            }
            Self::MissingBackendMapping => {
                f.write_str("no target-to-backend registry mapping was supplied")
            }
            Self::BackendNotRegistered { backend_id } => {
                write!(f, "backend `{backend_id}` is not registered")
            }
            Self::CapabilityUnsatisfied { capability, outcome } => {
                write!(
                    f,
                    "required capability `{capability}` is unsatisfied: {outcome}"
                )
            }
            Self::ConstraintUnsatisfied { key, reason } => {
                write!(f, "constraint `{key}` is unsatisfied: {reason}")
            }
            Self::UnknownConstraintProperty { key } => {
                write!(
                    f,
                    "cannot safely evaluate constraint on unknown property `{key}`"
                )
            }
            Self::InvalidDescriptor { reason } => {
                write!(f, "invalid target descriptor: {reason}")
            }
        }
    }
}

/// A single candidate's deterministic evaluation report.
#[derive(Debug, Clone)]
pub struct CandidateEvaluation {
    /// The target evaluated.
    pub target_id: TargetId,

    /// Display name copied from the descriptor.
    pub display_name: String,

    /// Reasons that disqualify this candidate.
    ///
    /// An empty list means no selection-stage rejection was found. It does
    /// not prove physical feasibility or successful compilation.
    pub rejections: Vec<RejectionReason>,

    /// Number of preferences satisfied.
    ///
    /// This is a simple count, not a semantic priority or a guarantee that
    /// one optimization is more valuable than another.
    pub satisfied_preferences: u128,

    /// Resource requirements that require a downstream feasibility check.
    pub deferred_resource_requirements: Vec<String>,
}

impl CandidateEvaluation {
    /// Whether the candidate passed every selection-stage mandatory check.
    pub fn is_eligible(&self) -> bool {
        self.rejections.is_empty()
    }
}

/// The selected target and any explicitly mapped backend implementation.
#[derive(Debug, Clone)]
pub struct SelectedTarget {
    /// Selected, validated target descriptor.
    pub target: TargetDescriptor,

    /// Canonical registered backend identifier, when supplied.
    pub backend_id: Option<String>,

    /// Evaluation that explains why this candidate won.
    pub evaluation: CandidateEvaluation,
}

/// Full selection report, including rejected candidates.
#[derive(Debug, Clone, Default)]
pub struct SelectionReport {
    /// The selected target, or `None` if no candidate passed.
    pub selected: Option<SelectedTarget>,

    /// All evaluations in canonical target-ID order.
    pub evaluations: Vec<CandidateEvaluation>,
}

impl SelectionReport {
    /// Whether a target was selected.
    pub fn has_selection(&self) -> bool {
        self.selected.is_some()
    }

    /// Return eligible candidates, preserving deterministic report order.
    pub fn eligible(&self) -> impl Iterator<Item = &CandidateEvaluation> {
        self.evaluations.iter().filter(|evaluation| evaluation.is_eligible())
    }

    /// Return rejected candidates, preserving deterministic report order.
    pub fn rejected(&self) -> impl Iterator<Item = &CandidateEvaluation> {
        self.evaluations.iter().filter(|evaluation| !evaluation.is_eligible())
    }
}

// ---------------------------------------------------------------------------
// Selection entry points
// ---------------------------------------------------------------------------

/// Select from validated target descriptors without requiring a registered
/// code-generation implementation.
///
/// This is suitable for discovery and planning. It is not proof that the
/// selected target can compile or execute the workload.
pub fn select(
    candidates: &[TargetCandidate],
    request: &SelectionRequest,
) -> BackendResult<SelectionReport> {
    select_impl(candidates, request, None)
}

/// Select a target only if its explicit backend mapping resolves to a
/// registered backend.
///
/// Registration is necessary but not sufficient for production readiness.
/// This function does not construct the backend or invoke its emitter.
pub fn select_registered(
    registry: &BackendRegistry,
    candidates: &[TargetCandidate],
    request: &SelectionRequest,
) -> BackendResult<SelectionReport> {
    select_impl(candidates, request, Some(registry))
}

fn select_impl(
    candidates: &[TargetCandidate],
    request: &SelectionRequest,
    registry: Option<&BackendRegistry>,
) -> BackendResult<SelectionReport> {
    let mut evaluations = Vec::with_capacity(candidates.len());
    let mut candidate_by_id: BTreeMap<String, TargetCandidate> = BTreeMap::new();

    // Reject duplicate target IDs rather than letting input order determine
    // which descriptor silently wins.
    for candidate in candidates {
        let id = candidate.target.id().as_str().to_owned();

        if candidate_by_id.insert(id.clone(), candidate.clone()).is_some() {
            return Err(BackendError::new(
                BackendErrorKind::InvalidInput,
                format!("duplicate target candidate identifier `{id}`"),
            ));
        }
    }

    // BTreeMap iteration makes evaluation independent of candidate input order.
    for (_, candidate) in candidate_by_id {
        let evaluation = evaluate_candidate(&candidate, request, registry)?;
        evaluations.push(evaluation);
    }

    // Rank only eligible candidates. A preference never overrides a mandatory
    // failure. Equal scores are resolved by canonical target ID.
    let best_index = evaluations
        .iter()
        .enumerate()
        .filter(|(_, evaluation)| evaluation.is_eligible())
        .max_by(|(_, left), (_, right)| {
            left.satisfied_preferences
                .cmp(&right.satisfied_preferences)
                // max_by chooses the greater element; reverse ID comparison
                // so the lexically smaller target wins a score tie.
                .then_with(|| right.target_id.cmp(&left.target_id))
        })
        .map(|(index, _)| index);

    let selected = best_index.map(|index| {
        let evaluation = evaluations[index].clone();
        let id = evaluation.target_id.as_str().to_owned();
        let candidate = candidates
            .iter()
            .find(|candidate| candidate.target.id().as_str() == id)
            .expect("selected evaluation must originate from a candidate");

        SelectedTarget {
            target: candidate.target.clone(),
            backend_id: candidate.backend_id.clone(),
            evaluation,
        }
    });

    Ok(SelectionReport {
        selected,
        evaluations,
    })
}

fn evaluate_candidate(
    candidate: &TargetCandidate,
    request: &SelectionRequest,
    registry: Option<&BackendRegistry>,
) -> BackendResult<CandidateEvaluation> {
    let target = &candidate.target;
    let mut rejections = Vec::new();
    let mut satisfied_preferences = 0_u128;
    let mut deferred_resource_requirements = Vec::new();

    if let Err(error) = target.validate() {
        rejections.push(RejectionReason::InvalidDescriptor {
            reason: error.to_string(),
        });
    }

    if let Some(requested) = &request.explicit_target {
        if requested != target.id() {
            rejections.push(RejectionReason::NotExplicitlyRequested {
                requested: requested.as_str().to_owned(),
            });
        }
    }

    if request.require_usable_implementation {
        if target.availability() != ImplementationAvailability::LegacyEmitterPresent {
            rejections.push(RejectionReason::ImplementationUnavailable {
                availability: format!("{:?}", target.availability()),
            });
        }
    }

    if let Some(registry) = registry {
        match &candidate.backend_id {
            None => rejections.push(RejectionReason::MissingBackendMapping),
            Some(backend_id) => match registry.contains(backend_id) {
                Ok(true) => {}
                Ok(false) => rejections.push(RejectionReason::BackendNotRegistered {
                    backend_id: backend_id.clone(),
                }),
                Err(error) => return Err(registry_error(error)),
            },
        }
    }

    // Capability checks are delegated to the canonical capability profile.
    // This avoids implementing a second lifecycle/evidence policy here.
    let required_capabilities = request
        .requirements
        .for_kind(RequirementKind::RequiredCapability)
        .map(|requirement| {
            CapabilityRequirement::from_name(requirement.key().as_str())
        })
        .collect::<BackendResult<Vec<_>>>()?;

    let capability_results = target
        .capability_profile()
        .check(&required_capabilities, request.capability_policy);

    for result in capability_results.failures() {
        rejections.push(RejectionReason::CapabilityUnsatisfied {
            capability: result.id().as_str().to_owned(),
            outcome: format!("{:?}", result.outcome()),
        });
    }

    // Requirements preserve duplicates and provenance. Evaluate each record
    // without rewriting or deduplicating the caller's semantic requirements.
    for requirement in request.requirements.iter() {
        match requirement.kind() {
            RequirementKind::RequiredCapability => {
                // Already checked by CapabilityProfile::check above.
            }
            RequirementKind::Constraint => {
                match evaluate_constraint(target, requirement) {
                    Ok(true) => {}
                    Ok(false) => rejections.push(
                        RejectionReason::ConstraintUnsatisfied {
                            key: requirement.key().as_str().to_owned(),
                            reason: "target property does not satisfy the operator/value"
                                .to_owned(),
                        },
                    ),
                    Err(ConstraintEvaluationError::UnknownProperty) => {
                        rejections.push(
                            RejectionReason::UnknownConstraintProperty {
                                key: requirement.key().as_str().to_owned(),
                            },
                        );
                    }
                    Err(ConstraintEvaluationError::InvalidValue(reason)) => {
                        rejections.push(
                            RejectionReason::ConstraintUnsatisfied {
                                key: requirement.key().as_str().to_owned(),
                                reason,
                            },
                        );
                    }
                }
            }
            RequirementKind::Preference => {
                if matches_preference(target, requirement) {
                    satisfied_preferences = satisfied_preferences.saturating_add(1);
                }
            }
            RequirementKind::Hint => {
                // Hints are optimization suggestions, not eligibility rules.
            }
            RequirementKind::Resource => {
                // Selection cannot establish runtime or physical resource
                // feasibility from static target metadata alone.
                deferred_resource_requirements.push(
                    requirement.canonical_key(),
                );
            }
        }
    }

    deferred_resource_requirements.sort();
    deferred_resource_requirements.dedup();

    Ok(CandidateEvaluation {
        target_id: target.id().clone(),
        display_name: target.display_name().to_owned(),
        rejections,
        satisfied_preferences,
        deferred_resource_requirements,
    })
}

fn registry_error(error: RegistryError) -> BackendError {
    BackendError::new(
        BackendErrorKind::Internal,
        "backend registry lookup failed during target selection",
    )
    .with_cause(error.to_string())
}

// ---------------------------------------------------------------------------
// Constraint evaluation
// ---------------------------------------------------------------------------

#[derive(Debug)]
enum ConstraintEvaluationError {
    UnknownProperty,
    InvalidValue(String),
}

/// Evaluate only properties with an explicit, well-defined representation.
///
/// Unknown properties fail closed. A future property requires a deliberate
/// adapter here or in a dedicated target-constraint evaluator; it must not
/// silently be assumed true.
fn evaluate_constraint(
    target: &TargetDescriptor,
    requirement: &BackendRequirement,
) -> Result<bool, ConstraintEvaluationError> {
    let key = requirement.key().as_str();

    if requirement.operator() == RequirementOperator::Exists {
        return Ok(target_property(target, key).is_some());
    }

    if requirement.operator() == RequirementOperator::DoesNotExist {
        return Ok(target_property(target, key).is_none());
    }

    let actual = target_property(target, key)
        .ok_or(ConstraintEvaluationError::UnknownProperty)?;

    compare_property(actual, requirement)
}

fn target_property<'a>(
    target: &'a TargetDescriptor,
    key: &str,
) -> Option<&'a str> {
    match key {
        "target.id" => Some(target.id().as_str()),
        "target.name" => Some(target.display_name()),
        "target.execution_model" => Some(target.execution_model().as_str()),
        "target.architecture" => target.architecture(),
        "target.operating_system" => target.operating_system(),
        "target.triple" => target.triple().map(|value| value.as_str()),
        _ => target.properties().get(key).map(String::as_str),
    }
}

fn compare_property(
    actual: &str,
    requirement: &BackendRequirement,
) -> Result<bool, ConstraintEvaluationError> {
    let Some(expected) = requirement.value() else {
        return Err(ConstraintEvaluationError::InvalidValue(
            "operator requires a value but none was supplied".to_owned(),
        ));
    };

    match requirement.operator() {
        RequirementOperator::Equal => Ok(match expected {
            RequirementValue::Text(value) => actual == value,
            RequirementValue::Boolean(value) => {
                actual == if *value { "true" } else { "false" }
            }
            RequirementValue::Quantity(value) => {
                actual == value.as_str()
            }
            RequirementValue::TextSet(values) => values.iter().any(|value| value == actual),
        }),

        RequirementOperator::NotEqual => Ok(match expected {
            RequirementValue::Text(value) => actual != value,
            RequirementValue::Boolean(value) => {
                actual != if *value { "true" } else { "false" }
            }
            RequirementValue::Quantity(value) => actual != value.as_str(),
            RequirementValue::TextSet(values) => !values.iter().any(|value| value == actual),
        }),

        RequirementOperator::Contains => match expected {
            RequirementValue::Text(value) => Ok(actual.contains(value)),
            _ => Err(ConstraintEvaluationError::InvalidValue(
                "`contains` requires a text value".to_owned(),
            )),
        },

        RequirementOperator::OneOf => match expected {
            RequirementValue::TextSet(values) => {
                Ok(values.iter().any(|value| value == actual))
            }
            _ => Err(ConstraintEvaluationError::InvalidValue(
                "`one-of` requires a text-set value".to_owned(),
            )),
        },

        RequirementOperator::LessThan
        | RequirementOperator::LessThanOrEqual
        | RequirementOperator::GreaterThan
        | RequirementOperator::GreaterThanOrEqual => {
            let RequirementValue::Quantity(expected) = expected else {
                return Err(ConstraintEvaluationError::InvalidValue(
                    "ordered comparisons require a numeric quantity".to_owned(),
                ));
            };

            let actual = DecimalQuantity::new(actual).map_err(|error| {
                ConstraintEvaluationError::InvalidValue(format!(
                    "target property `{actual}` is not a non-negative decimal quantity: {error}"
                ))
            })?;

            let ordering = actual.numeric_cmp(expected);

            Ok(match requirement.operator() {
                RequirementOperator::LessThan => ordering == Ordering::Less,
                RequirementOperator::LessThanOrEqual => {
                    ordering != Ordering::Greater
                }
                RequirementOperator::GreaterThan => ordering == Ordering::Greater,
                RequirementOperator::GreaterThanOrEqual => {
                    ordering != Ordering::Less
                }
                _ => false,
            })
        }

        RequirementOperator::Exists | RequirementOperator::DoesNotExist => {
            unreachable!("existence operators are handled before value comparison")
        }
    }
}

fn matches_preference(
    target: &TargetDescriptor,
    requirement: &BackendRequirement,
) -> bool {
    let Some(actual) = target_property(target, requirement.key().as_str()) else {
        return false;
    };

    compare_property(actual, requirement).unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::target::ExecutionModel;
    use crate::backend::requirements::{
        BackendRequirement, RequirementKey, RequirementValue,
    };

    fn target(id: &str, architecture: &str) -> TargetDescriptor {
        TargetDescriptor::from_name(
            id,
            id,
            ExecutionModel::Native,
        )
        .expect("valid target ID")
        .with_architecture(architecture)
        .expect("valid architecture")
    }

    fn constraint(
        key: &str,
        operator: RequirementOperator,
        value: RequirementValue,
    ) -> BackendRequirement {
        BackendRequirement::constraint(
            RequirementKey::new(key).expect("valid requirement key"),
            operator,
            value,
        )
        .expect("valid constraint")
    }

    #[test]
    fn selection_is_deterministic_across_candidate_order() {
        let candidates = vec![
            TargetCandidate::new(target("native.zeta", "aarch64")),
            TargetCandidate::new(target("native.alpha", "x86_64")),
        ];

        let first = select(&candidates, &SelectionRequest::new())
            .expect("selection report");
        let second = select(
            &[candidates[1].clone(), candidates[0].clone()],
            &SelectionRequest::new(),
        )
        .expect("selection report");

        assert_eq!(
            first.selected.as_ref().map(|item| item.target.id().as_str()),
            second.selected.as_ref().map(|item| item.target.id().as_str())
        );
        assert_eq!(
            first.selected.as_ref().map(|item| item.target.id().as_str()),
            Some("native.alpha")
        );
    }

    #[test]
    fn explicit_target_never_silently_falls_back() {
        let requested = TargetId::new("native.missing").expect("valid ID");
        let report = select(
            &[TargetCandidate::new(target("native.alpha", "x86_64"))],
            &SelectionRequest::new().for_target(requested),
        )
        .expect("selection report");

        assert!(report.selected.is_none());
        assert!(report.evaluations.iter().all(|item| {
            item.rejections.iter().any(|reason| {
                matches!(reason, RejectionReason::NotExplicitlyRequested { .. })
            })
        }));
    }

    #[test]
    fn mandatory_constraint_rejects_incompatible_target() {
        let requirement = constraint(
            "target.architecture",
            RequirementOperator::Equal,
            RequirementValue::Text("aarch64".to_owned()),
        );
        let request = SelectionRequest::new().with_requirements(
            RequirementSet::new().with(requirement),
        );

        let report = select(
            &[
                TargetCandidate::new(target("native.x86", "x86_64")),
                TargetCandidate::new(target("native.arm", "aarch64")),
            ],
            &request,
        )
        .expect("selection report");

        assert_eq!(
            report.selected.as_ref().map(|item| item.target.id().as_str()),
            Some("native.arm")
        );
    }

    #[test]
    fn unknown_constraints_fail_closed() {
        let requirement = constraint(
            "future.unknown_property",
            RequirementOperator::Equal,
            RequirementValue::Text("some-value".to_owned()),
        );
        let report = select(
            &[TargetCandidate::new(target("native.x86", "x86_64"))],
            &SelectionRequest::new().with_requirements(
                RequirementSet::new().with(requirement),
            ),
        )
        .expect("selection report");

        assert!(report.selected.is_none());
        assert!(matches!(
            report.evaluations[0].rejections.first(),
            Some(RejectionReason::UnknownConstraintProperty { .. })
        ));
    }

    #[test]
    fn preferences_rank_only_candidates_that_pass_constraints() {
        let requirement = BackendRequirement::preference(
            RequirementKey::new("target.architecture").expect("valid key"),
            RequirementValue::Text("aarch64".to_owned()),
        )
        .expect("valid preference");

        let report = select(
            &[
                TargetCandidate::new(target("native.x86", "x86_64")),
                TargetCandidate::new(target("native.arm", "aarch64")),
            ],
            &SelectionRequest::new().with_requirements(
                RequirementSet::new().with(requirement),
            ),
        )
        .expect("selection report");

        assert_eq!(
            report.selected.as_ref().map(|item| item.target.id().as_str()),
            Some("native.arm")
        );
    }

    #[test]
    fn resource_requirements_are_deferred_not_assumed_satisfied() {
        let requirement = BackendRequirement::resource(
            RequirementKey::new("quantum.logical_qubits").expect("valid key"),
            RequirementOperator::GreaterThanOrEqual,
            DecimalQuantity::new("1000000000000000000000000000000")
                .expect("valid arbitrary-precision quantity"),
            Some("qubits".to_owned()),
        )
        .expect("valid resource requirement");

        let report = select(
            &[TargetCandidate::new(target("quantum.simulator", "virtual"))],
            &SelectionRequest::new().with_requirements(
                RequirementSet::new().with(requirement),
            ),
        )
        .expect("selection report");

        let selected = report.selected.expect("no selection-stage rejection");
        assert_eq!(selected.evaluation.deferred_resource_requirements.len(), 1);
    }
}
