
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Dependency Compatibility
//!
//! Path: `src/toolchain/dependencies/compatibility.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides evidence-based compatibility assessment for dependency
//! candidates.
//!
//! Compatibility dimensions are extensible. The initial conventional
//! dimensions include language, ABI, IR, provider and package contracts.
//! Future domains and compatibility systems are introduced through
//! registered identifiers and evaluators rather than closed enumerations.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Compatibility dimension identities.
//! - Compatibility requirements.
//! - Compatibility profiles.
//! - Compatibility assessment results.
//! - Deterministic aggregation of compatibility evidence.
//! - An exact metadata compatibility evaluator.
//! - Integration with the canonical constraint evaluator registry.
//!
//! This module does not own:
//!
//! - Dependency declarations.
//! - Package identities.
//! - Version requirement parsing or version comparison.
//! - Dependency source discovery.
//! - Candidate discovery.
//! - Dependency selection or backtracking.
//! - Dependency graph construction.
//! - Feature unification.
//! - Manifest parsing.
//! - Package archive parsing.
//! - Package signatures or trust.
//! - Language semantics.
//! - ABI definitions.
//! - IR definitions.
//! - Provider implementation.
//! - Target selection.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Compilation or execution.
//!
//! Those responsibilities remain with their canonical repository
//! authorities.
//!
//! ## Existing authoritative integration
//!
//! - `super::model` owns dependency declarations and package coordinates.
//! - `super::constraints` owns constraint predicates, candidate facts,
//!   evidence completeness, evaluator registration and constraint reports.
//! - `super::resolver` owns candidate selection and dependency solving.
//! - `super::sources` owns dependency source discovery.
//! - `crate::toolchain::manifest` adapts the canonical project manifest.
//! - `crate::compiler` owns compilation contracts.
//! - `crate::zuir` owns the universal IR contract.
//! - `crate::backend` owns backend realization contracts.
//! - `crate::zhal` owns physical hardware integration.
//! - Domain modules own their respective semantic and domain contracts.
//!
//! This module must not duplicate these authorities.
//!
//! ## Evidence semantics
//!
//! Compatibility is evaluated using three outcomes:
//!
//! - Satisfied: sufficient evidence establishes compatibility.
//! - Violated: sufficient evidence establishes incompatibility.
//! - Indeterminate: available evidence cannot establish either result.
//!
//! Missing metadata in a complete inventory establishes absence.
//! Missing metadata in a partial or unavailable inventory is unknown.
//!
//! Unknown compatibility MUST NOT be interpreted as compatibility.
//!
//! ## Metadata contract
//!
//! The generic candidate metadata convention used by the exact evaluator
//! is:
//!
//! `zamani.compatibility.<dimension>`
//!
//! The metadata value is an opaque, canonical contract identifier.
//!
//! Example:
//!
//! `zamani.compatibility.abi = zamani.abi.v1`
//!
//! The exact evaluator compares identifiers. It does not parse semantic
//! versions, inspect ABI layouts, verify IR schemas, or establish that
//! two providers implement equivalent behavior.
//!
//! More sophisticated semantics must be supplied by registered
//! compatibility evaluators.
//!
//! A provider must not claim a complete metadata inventory unless it
//! has established completeness for the inventory's declared scope.
//!
//! ## POCO-REAF
//!
//! Compatibility concerns preservation of program and package contracts,
//! not equality of generated machine artifacts.
//!
//! A program may have different valid realizations for different targets.
//! A candidate is compatible only when its required contracts are
//! established by authoritative evidence.
//!
//! No universal hardware capacity or computational-domain limit is
//! introduced here.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Compatibility dimensions.
//! - Compatibility requirements.
//! - Candidate metadata entries.
//! - Provider contracts.
//! - Computing domains.
//! - Package compatibility declarations.
//! - Version components.
//! - Hardware or distributed resources.
//!
//! Collections grow according to available memory and allocator capacity.
//! Operational limits belong to explicitly configured policies and
//! budgets.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## Determinism
//!
//! Compatibility dimensions and requirements use ordered collections.
//! Reports follow deterministic requirement ordering.
//!
//! Registered evaluator implementations are responsible for deterministic
//! behavior when their contracts require reproducibility.
//!
//! ## Security
//!
//! This module performs no filesystem, network, process, environment,
//! credential or hardware access.
//!
//! Metadata is treated as evidence supplied by an authoritative candidate
//! provider. Its provenance and authenticity must be verified by the
//! appropriate source, package and security subsystems.
//!
//! ## Freeze contract
//!
//! This module is frozen when:
//!
//! - Compatibility requirements are validated.
//! - Unknown evidence remains indeterminate.
//! - Complete evidence can establish absence.
//! - Incompatible evidence cannot pass.
//! - Registered evaluators integrate with constraints.rs.
//! - Custom compatibility dimensions work without source changes.
//! - Preferences and hints remain distinct from mandatory requirements.
//! - All public contracts and tests pass.

use std::collections::BTreeSet;
use std::fmt;

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::result::Result;

use super::constraints::{
    Completeness,
    ConstraintDecision,
    ConstraintEvaluation,
    ConstraintEvaluatorRegistry,
    ConstraintKind,
    ConstraintNamespace,
    ConstraintPredicate,
    DependencyCandidateFacts,
    DependencyConstraint,
    DependencyConstraintSet,
    EvidenceMap,
    NamedConstraintEvaluator,
};

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const OPERATION_COMPATIBILITY: &str =
    "dependency.compatibility";

const OPERATION_REGISTRATION: &str =
    "dependency.compatibility.register";

const METADATA_PREFIX: &str =
    "zamani.compatibility.";

// ============================================================================
// VALIDATION
// ============================================================================

fn validate_text(
    value: &str,
    field: &'static str,
) -> Result<()> {
    if value.is_empty() {
        return Err(
            ToolchainError::invalid_input(
                format!("{field} must not be empty"),
            )
            .with_operation(OPERATION_COMPATIBILITY),
        );
    }

    if value.trim() != value {
        return Err(
            ToolchainError::invalid_input(
                format!(
                    "{field} must not contain leading or \
                     trailing whitespace"
                ),
            )
            .with_operation(OPERATION_COMPATIBILITY),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            ToolchainError::invalid_input(
                format!(
                    "{field} must not contain control characters"
                ),
            )
            .with_operation(OPERATION_COMPATIBILITY),
        );
    }

    Ok(())
}

// ============================================================================
// COMPATIBILITY DIMENSION
// ============================================================================

/// Extensible identity of a compatibility contract dimension.
///
/// A dimension describes the kind of contract being compared.
///
/// It does not define the comparison algorithm.
///
/// Conventional identifiers include:
///
/// - `language`
/// - `abi`
/// - `ir`
/// - `provider`
/// - `package`
///
/// New dimensions may be introduced without modifying this type.
///
/// Examples:
///
/// - `quantum.ir`
/// - `hdl.interface`
/// - `ai.model-format`
/// - `distributed.protocol`
/// - `tensor.contract`
///
/// The identifier is not a hardware or vendor identifier.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct CompatibilityDimension(String);

impl CompatibilityDimension {
    /// Construct a validated compatibility dimension.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self> {
        let value = value.into();

        validate_text(
            &value,
            "compatibility dimension",
        )?;

        Ok(Self(value))
    }

    /// Borrow the canonical dimension identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Construct the canonical candidate metadata key.
    ///
    /// The key is namespaced so that compatibility evidence does not
    /// collide with unrelated package metadata.
    #[must_use]
    pub fn metadata_key(&self) -> String {
        format!(
            "{METADATA_PREFIX}{}",
            self.as_str(),
        )
    }

    /// Construct the corresponding constraint namespace.
    pub fn constraint_namespace(
        &self,
    ) -> Result<ConstraintNamespace> {
        ConstraintNamespace::new(
            self.metadata_key(),
        )
    }
}

impl fmt::Display for CompatibilityDimension {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// COMPATIBILITY REQUIREMENT
// ============================================================================

/// A mandatory exact compatibility requirement.
///
/// The expected value is an opaque canonical contract identifier.
/// Its interpretation belongs to the evaluator registered for the
/// associated compatibility dimension.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct CompatibilityRequirement {
    dimension: CompatibilityDimension,
    expected: String,
}

impl CompatibilityRequirement {
    /// Construct a validated compatibility requirement.
    pub fn new(
        dimension: CompatibilityDimension,
        expected: impl Into<String>,
    ) -> Result<Self> {
        let expected = expected.into();

        validate_text(
            &expected,
            "expected compatibility value",
        )?;

        Ok(Self {
            dimension,
            expected,
        })
    }

    /// Borrow the required compatibility dimension.
    #[must_use]
    pub fn dimension(
        &self,
    ) -> &CompatibilityDimension {
        &self.dimension
    }

    /// Borrow the expected contract identifier.
    #[must_use]
    pub fn expected(&self) -> &str {
        &self.expected
    }

    /// Convert this requirement into the canonical dependency
    /// constraint representation.
    ///
    /// The resulting constraint is mandatory.
    ///
    /// The resolver can evaluate it through the existing
    /// `ConstraintEvaluatorRegistry`.
    pub fn to_constraint(
        &self,
    ) -> Result<DependencyConstraint> {
        self.to_constraint_of_kind(
            ConstraintKind::Constraint,
        )
    }

    /// Convert this requirement into a constraint of the supplied
    /// classification.
    ///
    /// Use `Requirement` or `Constraint` for mandatory compatibility.
    /// `Preference` and `Hint` are non-mandatory and must not reject
    /// a dependency candidate.
    pub fn to_constraint_of_kind(
        &self,
        kind: ConstraintKind,
    ) -> Result<DependencyConstraint> {
        let namespace =
            self.dimension.constraint_namespace()?;

        let predicate = ConstraintPredicate::compatibility(
            namespace,
            self.expected.clone(),
        )?;

        DependencyConstraint::new(
            kind,
            predicate,
        )
    }
}

// ============================================================================
// COMPATIBILITY PROFILE
// ============================================================================

/// Deterministically ordered collection of mandatory compatibility
/// requirements.
///
/// A profile is independent of a particular package candidate.
///
/// It can therefore be reused when evaluating multiple candidates.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CompatibilityProfile {
    requirements: BTreeSet<CompatibilityRequirement>,
}

impl CompatibilityProfile {
    /// Construct an empty compatibility profile.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a compatibility requirement.
    ///
    /// Duplicate requirements are rejected.
    pub fn insert(
        &mut self,
        requirement: CompatibilityRequirement,
    ) -> Result<()> {
        if !self.requirements.insert(requirement) {
            return Err(
                ToolchainError::invalid_input(
                    "duplicate compatibility requirement",
                )
                .with_operation(OPERATION_COMPATIBILITY),
            );
        }

        Ok(())
    }

    /// Insert a requirement using a dimension identifier and
    /// expected contract value.
    pub fn require(
        &mut self,
        dimension: CompatibilityDimension,
        expected: impl Into<String>,
    ) -> Result<()> {
        self.insert(
            CompatibilityRequirement::new(
                dimension,
                expected,
            )?,
        )
    }

    /// Return the number of requirements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.requirements.len()
    }

    /// Return whether the profile contains no requirements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }

    /// Iterate over requirements in deterministic order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &CompatibilityRequirement> {
        self.requirements.iter()
    }

    /// Convert the profile into the canonical dependency constraint set.
    ///
    /// Every generated constraint is mandatory.
    pub fn to_constraints(
        &self,
    ) -> Result<DependencyConstraintSet> {
        let mut constraints =
            DependencyConstraintSet::new();

        for requirement in self.iter() {
            constraints.insert(
                requirement.to_constraint()?,
            )?;
        }

        Ok(constraints)
    }

    /// Register one exact metadata evaluator for each distinct
    /// compatibility dimension in this profile.
    ///
    /// This integrates the compatibility subsystem with the existing
    /// constraint evaluator registry.
    ///
    /// Evaluators must be registered before the corresponding
    /// constraints are evaluated.
    ///
    /// Duplicate evaluator registration is an explicit error.
    pub fn register_evaluators(
        &self,
        registry: &mut ConstraintEvaluatorRegistry,
    ) -> Result<()> {
        let dimensions: BTreeSet<_> = self
            .requirements
            .iter()
            .map(|requirement| {
                requirement.dimension().clone()
            })
            .collect();

        for dimension in dimensions {
            register_exact_metadata_evaluator(
                registry,
                dimension,
            )?;
        }

        Ok(())
    }

    /// Assess this profile against one candidate's established facts.
    ///
    /// This operation performs no external discovery.
    ///
    /// Missing metadata in a complete inventory establishes a
    /// compatibility violation. Missing metadata in an incomplete
    /// inventory remains indeterminate.
    pub fn evaluate(
        &self,
        candidate: &DependencyCandidateFacts,
    ) -> CompatibilityReport {
        let observations = self
            .requirements
            .iter()
            .map(|requirement| {
                evaluate_requirement(
                    requirement,
                    candidate.metadata(),
                )
            })
            .collect::<Vec<_>>();

        CompatibilityReport::new(observations)
    }
}

// ============================================================================
// COMPATIBILITY STATUS
// ============================================================================

/// Evidence-based result for one compatibility assessment.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum CompatibilityStatus {
    /// Sufficient evidence establishes compatibility.
    Satisfied,

    /// Sufficient evidence establishes incompatibility.
    Violated,

    /// Available evidence is insufficient.
    Indeterminate,
}

impl CompatibilityStatus {
    /// Convert to the canonical constraint evaluation outcome.
    #[must_use]
    pub const fn as_constraint_evaluation(
        self,
    ) -> ConstraintEvaluation {
        match self {
            Self::Satisfied => {
                ConstraintEvaluation::Satisfied
            }

            Self::Violated => {
                ConstraintEvaluation::Violated
            }

            Self::Indeterminate => {
                ConstraintEvaluation::Indeterminate
            }
        }
    }
}

// ============================================================================
// INDIVIDUAL COMPATIBILITY OBSERVATION
// ============================================================================

/// Result of evaluating one compatibility requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityObservation {
    requirement: CompatibilityRequirement,
    observed: Option<String>,
    status: CompatibilityStatus,
    explanation: String,
}

impl CompatibilityObservation {
    fn new(
        requirement: CompatibilityRequirement,
        observed: Option<String>,
        status: CompatibilityStatus,
        explanation: impl Into<String>,
    ) -> Self {
        Self {
            requirement,
            observed,
            status,
            explanation: explanation.into(),
        }
    }

    /// Borrow the original requirement.
    #[must_use]
    pub fn requirement(
        &self,
    ) -> &CompatibilityRequirement {
        &self.requirement
    }

    /// Borrow the observed value, if one was established.
    #[must_use]
    pub fn observed(&self) -> Option<&str> {
        self.observed.as_deref()
    }

    /// Return the assessment outcome.
    #[must_use]
    pub const fn status(
        &self,
    ) -> CompatibilityStatus {
        self.status
    }

    /// Borrow the assessment explanation.
    #[must_use]
    pub fn explanation(&self) -> &str {
        &self.explanation
    }
}

// ============================================================================
// AGGREGATE COMPATIBILITY REPORT
// ============================================================================

/// Aggregate result of evaluating a compatibility profile.
///
/// A violation dominates an indeterminate observation because a
/// single established mandatory incompatibility is sufficient
/// to reject the candidate.
///
/// Otherwise, any indeterminate mandatory observation prevents
/// a successful compatibility decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityReport {
    decision: CompatibilityStatus,
    observations: Vec<CompatibilityObservation>,
}

impl CompatibilityReport {
    fn new(
        observations: Vec<CompatibilityObservation>,
    ) -> Self {
        let violated = observations.iter().any(
            |observation| {
                observation.status()
                    == CompatibilityStatus::Violated
            },
        );

        let indeterminate = observations.iter().any(
            |observation| {
                observation.status()
                    == CompatibilityStatus::Indeterminate
            },
        );

        let decision = if violated {
            CompatibilityStatus::Violated
        } else if indeterminate {
            CompatibilityStatus::Indeterminate
        } else {
            CompatibilityStatus::Satisfied
        };

        Self {
            decision,
            observations,
        }
    }

    /// Return the aggregate compatibility decision.
    #[must_use]
    pub const fn decision(
        &self,
    ) -> CompatibilityStatus {
        self.decision
    }

    /// Return whether compatibility has been established.
    ///
    /// An indeterminate result is not considered compatible.
    #[must_use]
    pub const fn is_compatible(
        &self,
    ) -> bool {
        matches!(
            self.decision,
            CompatibilityStatus::Satisfied,
        )
    }

    /// Borrow all individual observations.
    #[must_use]
    pub fn observations(
        &self,
    ) -> &[CompatibilityObservation] {
        &self.observations
    }

    /// Iterate over observations belonging to one dimension.
    pub fn observations_for(
        &self,
        dimension: &CompatibilityDimension,
    ) -> impl Iterator<Item = &CompatibilityObservation> {
        self.observations.iter().filter(
            move |observation| {
                observation.requirement().dimension()
                    == dimension
            },
        )
    }
}

// ============================================================================
// EXACT METADATA EVALUATION
// ============================================================================

fn evaluate_requirement(
    requirement: &CompatibilityRequirement,
    metadata: &EvidenceMap,
) -> CompatibilityObservation {
    let key = requirement.dimension().metadata_key();

    match metadata.get(&key) {
        Some(observed) => {
            if observed == requirement.expected() {
                CompatibilityObservation::new(
                    requirement.clone(),
                    Some(observed.to_owned()),
                    CompatibilityStatus::Satisfied,
                    format!(
                        "compatibility dimension '{}' matches '{}'",
                        requirement.dimension(),
                        requirement.expected(),
                    ),
                )
            } else {
                CompatibilityObservation::new(
                    requirement.clone(),
                    Some(observed.to_owned()),
                    CompatibilityStatus::Violated,
                    format!(
                        "compatibility dimension '{}' requires '{}', \
                         but candidate declares '{}'",
                        requirement.dimension(),
                        requirement.expected(),
                        observed,
                    ),
                )
            }
        }

        None => {
            let status = match metadata.completeness() {
                Completeness::Complete => {
                    CompatibilityStatus::Violated
                }

                Completeness::Partial
                | Completeness::Unavailable => {
                    CompatibilityStatus::Indeterminate
                }
            };

            let explanation = match status {
                CompatibilityStatus::Violated => {
                    format!(
                        "compatibility metadata '{}' is absent \
                         from a complete candidate inventory",
                        key,
                    )
                }

                CompatibilityStatus::Indeterminate => {
                    format!(
                        "compatibility metadata '{}' is unavailable \
                         or its inventory is incomplete",
                        key,
                    )
                }

                CompatibilityStatus::Satisfied => {
                    unreachable!(
                        "missing metadata cannot establish satisfaction"
                    )
                }
            };

            CompatibilityObservation::new(
                requirement.clone(),
                None,
                status,
                explanation,
            )
        }
    }
}

// ============================================================================
// CANONICAL CONSTRAINT EVALUATOR ADAPTER
// ============================================================================

/// Exact metadata evaluator for one compatibility dimension.
///
/// The evaluator implements the existing
/// `NamedConstraintEvaluator` contract.
///
/// It does not replace the canonical constraint registry.
///
/// Its expression is the expected opaque contract identifier.
#[derive(Debug, Clone)]
pub struct ExactMetadataCompatibilityEvaluator {
    dimension: CompatibilityDimension,
}

impl ExactMetadataCompatibilityEvaluator {
    /// Construct an evaluator for one compatibility dimension.
    #[must_use]
    pub fn new(
        dimension: CompatibilityDimension,
    ) -> Self {
        Self {
            dimension,
        }
    }

    /// Borrow the evaluated compatibility dimension.
    #[must_use]
    pub fn dimension(
        &self,
    ) -> &CompatibilityDimension {
        &self.dimension
    }
}

impl NamedConstraintEvaluator
    for ExactMetadataCompatibilityEvaluator
{
    fn evaluate(
        &self,
        expression: &str,
        candidate: &DependencyCandidateFacts,
    ) -> Result<ConstraintEvaluation> {
        validate_text(
            expression,
            "compatibility expression",
        )?;

        let requirement =
            CompatibilityRequirement::new(
                self.dimension.clone(),
                expression.to_owned(),
            )?;

        let observation = evaluate_requirement(
            &requirement,
            candidate.metadata(),
        );

        Ok(
            observation
                .status()
                .as_constraint_evaluation(),
        )
    }
}

// ============================================================================
// EVALUATOR REGISTRATION
// ============================================================================

/// Register the exact metadata evaluator for a compatibility dimension.
///
/// The registration namespace is derived from the dimension:
///
/// `zamani.compatibility.<dimension>`
///
/// Unknown dimensions must be registered explicitly.
///
/// Duplicate registrations are rejected by the canonical
/// `ConstraintEvaluatorRegistry`.
pub fn register_exact_metadata_evaluator(
    registry: &mut ConstraintEvaluatorRegistry,
    dimension: CompatibilityDimension,
) -> Result<()> {
    let namespace =
        dimension.constraint_namespace()?;

    registry
        .register_compatibility_evaluator(
            namespace,
            ExactMetadataCompatibilityEvaluator::new(
                dimension,
            ),
        )
        .map_err(|error| {
            error
                .with_operation(OPERATION_REGISTRATION)
        })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::{
        CompatibilityDimension,
        CompatibilityProfile,
        CompatibilityRequirement,
        CompatibilityStatus,
        ExactMetadataCompatibilityEvaluator,
        register_exact_metadata_evaluator,
    };

    use std::collections::BTreeMap;

    use crate::toolchain::result::Result;

    use super::super::constraints::{
        Completeness,
        ConstraintDecision,
        ConstraintEvaluation,
        ConstraintEvaluatorRegistry,
        ConstraintKind,
        DependencyCandidateFacts,
        EvidenceMap,
        NamedConstraintEvaluator,
    };

    fn dimension(
        name: &str,
    ) -> CompatibilityDimension {
        CompatibilityDimension::new(name)
            .expect("valid dimension")
    }

    fn candidate_with_metadata(
        values: &[(&str, &str)],
        completeness: Completeness,
    ) -> DependencyCandidateFacts {
        let metadata = values
            .iter()
            .map(|(key, value)| {
                (
                    (*key).to_owned(),
                    (*value).to_owned(),
                )
            })
            .collect::<BTreeMap<_, _>>();

        let evidence = match completeness {
            Completeness::Complete => {
                EvidenceMap::complete(metadata)
            }

            Completeness::Partial => {
                EvidenceMap::partial(metadata)
            }

            Completeness::Unavailable => {
                EvidenceMap::unavailable()
            }
        };

        DependencyCandidateFacts::new()
            .with_metadata_inventory(evidence)
    }

    fn profile() -> CompatibilityProfile {
        let mut profile = CompatibilityProfile::new();

        profile
            .require(
                dimension("language"),
                "zamani.language.v1",
            )
            .expect("language requirement");

        profile
            .require(
                dimension("abi"),
                "zamani.abi.v1",
            )
            .expect("ABI requirement");

        profile
            .require(
                dimension("ir"),
                "zamani.ir.v1",
            )
            .expect("IR requirement");

        profile
            .require(
                dimension("provider"),
                "zamani.provider.api.v1",
            )
            .expect("provider requirement");

        profile
            .require(
                dimension("package"),
                "zamani.package.v1",
            )
            .expect("package requirement");

        profile
    }

    fn complete_candidate() -> DependencyCandidateFacts {
        candidate_with_metadata(
            &[
                (
                    "zamani.compatibility.language",
                    "zamani.language.v1",
                ),
                (
                    "zamani.compatibility.abi",
                    "zamani.abi.v1",
                ),
                (
                    "zamani.compatibility.ir",
                    "zamani.ir.v1",
                ),
                (
                    "zamani.compatibility.provider",
                    "zamani.provider.api.v1",
                ),
                (
                    "zamani.compatibility.package",
                    "zamani.package.v1",
                ),
            ],
            Completeness::Complete,
        )
    }

    #[test]
    fn all_conventional_dimensions_can_be_assessed() {
        let report = profile().evaluate(
            &complete_candidate(),
        );

        assert_eq!(
            report.decision(),
            CompatibilityStatus::Satisfied,
        );

        assert!(report.is_compatible());

        assert_eq!(
            report.observations().len(),
            5,
        );
    }

    #[test]
    fn incompatible_abi_rejects_candidate() {
        let candidate = candidate_with_metadata(
            &[
                (
                    "zamani.compatibility.abi",
                    "zamani.abi.incompatible",
                ),
            ],
            Completeness::Partial,
        );

        let mut requirements =
            CompatibilityProfile::new();

        requirements
            .require(
                dimension("abi"),
                "zamani.abi.v1",
            )
            .expect("ABI requirement");

        let report = requirements.evaluate(&candidate);

        assert_eq!(
            report.decision(),
            CompatibilityStatus::Violated,
        );
    }

    #[test]
    fn missing_metadata_in_complete_inventory_is_violation() {
        let candidate = candidate_with_metadata(
            &[],
            Completeness::Complete,
        );

        let mut requirements =
            CompatibilityProfile::new();

        requirements
            .require(
                dimension("language"),
                "zamani.language.v1",
            )
            .expect("language requirement");

        assert_eq!(
            requirements.evaluate(&candidate).decision(),
            CompatibilityStatus::Violated,
        );
    }

    #[test]
    fn missing_metadata_in_partial_inventory_is_indeterminate() {
        let candidate = candidate_with_metadata(
            &[],
            Completeness::Partial,
        );

        let mut requirements =
            CompatibilityProfile::new();

        requirements
            .require(
                dimension("abi"),
                "zamani.abi.v1",
            )
            .expect("ABI requirement");

        assert_eq!(
            requirements.evaluate(&candidate).decision(),
            CompatibilityStatus::Indeterminate,
        );
    }

    #[test]
    fn unavailable_metadata_is_indeterminate() {
        let candidate = candidate_with_metadata(
            &[],
            Completeness::Unavailable,
        );

        let mut requirements =
            CompatibilityProfile::new();

        requirements
            .require(
                dimension("ir"),
                "zamani.ir.v1",
            )
            .expect("IR requirement");

        assert_eq!(
            requirements.evaluate(&candidate).decision(),
            CompatibilityStatus::Indeterminate,
        );
    }

    #[test]
    fn established_violation_dominates_unknown_evidence() {
        let candidate = candidate_with_metadata(
            &[
                (
                    "zamani.compatibility.abi",
                    "zamani.abi.incompatible",
                ),
            ],
            Completeness::Partial,
        );

        let report = profile().evaluate(&candidate);

        assert_eq!(
            report.decision(),
            CompatibilityStatus::Violated,
        );
    }

    #[test]
    fn empty_profile_is_satisfied() {
        let report = CompatibilityProfile::new()
            .evaluate(&DependencyCandidateFacts::new());

        assert_eq!(
            report.decision(),
            CompatibilityStatus::Satisfied,
        );
    }

    #[test]
    fn custom_dimensions_need_no_source_changes() {
        let mut requirements =
            CompatibilityProfile::new();

        requirements
            .require(
                dimension("future.domain.contract"),
                "future.contract.v1",
            )
            .expect("custom requirement");

        let candidate = candidate_with_metadata(
            &[
                (
                    "zamani.compatibility.future.domain.contract",
                    "future.contract.v1",
                ),
            ],
            Completeness::Complete,
        );

        assert_eq!(
            requirements.evaluate(&candidate).decision(),
            CompatibilityStatus::Satisfied,
        );
    }

    #[test]
    fn duplicate_requirements_are_rejected() {
        let requirement =
            CompatibilityRequirement::new(
                dimension("abi"),
                "zamani.abi.v1",
            )
            .expect("valid requirement");

        let mut profile = CompatibilityProfile::new();

        profile
            .insert(requirement.clone())
            .expect("first insertion");

        assert!(
            profile.insert(requirement).is_err()
        );
    }

    #[test]
    fn compatibility_requirements_become_mandatory_constraints() {
        let mut profile = CompatibilityProfile::new();

        profile
            .require(
                dimension("language"),
                "zamani.language.v1",
            )
            .expect("language requirement");

        let constraints = profile
            .to_constraints()
            .expect("constraint conversion");

        assert_eq!(constraints.len(), 1);

        assert!(
            constraints.iter().all(
                |constraint| {
                    constraint.kind()
                        == ConstraintKind::Constraint
                }
            )
        );
    }

    #[test]
    fn profile_registers_existing_constraint_evaluators() {
        let profile = profile();

        let mut registry =
            ConstraintEvaluatorRegistry::new();

        profile
            .register_evaluators(&mut registry)
            .expect("evaluator registration");

        let constraints = profile
            .to_constraints()
            .expect("constraint conversion");

        let report = registry
            .evaluate(
                &constraints,
                &complete_candidate(),
            )
            .expect("constraint evaluation");

        assert_eq!(
            report.decision(),
            ConstraintDecision::Satisfied,
        );
    }

    #[test]
    fn constraint_evaluator_preserves_indeterminate_evidence() {
        let dimension = dimension("abi");

        let evaluator =
            ExactMetadataCompatibilityEvaluator::new(
                dimension,
            );

        let candidate = DependencyCandidateFacts::new();

        let result = evaluator
            .evaluate(
                "zamani.abi.v1",
                &candidate,
            )
            .expect("evaluation");

        assert_eq!(
            result,
            ConstraintEvaluation::Indeterminate,
        );
    }

    #[test]
    fn constraint_evaluator_rejects_known_mismatch() {
        let evaluator =
            ExactMetadataCompatibilityEvaluator::new(
                dimension("abi"),
            );

        let candidate = candidate_with_metadata(
            &[
                (
                    "zamani.compatibility.abi",
                    "zamani.abi.v2",
                ),
            ],
            Completeness::Complete,
        );

        let result = evaluator
            .evaluate(
                "zamani.abi.v1",
                &candidate,
            )
            .expect("evaluation");

        assert_eq!(
            result,
            ConstraintEvaluation::Violated,
        );
    }

    #[test]
    fn registration_rejects_duplicate_dimensions() {
        let mut registry =
            ConstraintEvaluatorRegistry::new();

        let dimension = dimension("abi");

        register_exact_metadata_evaluator(
            &mut registry,
            dimension.clone(),
        )
        .expect("first registration");

        assert!(
            register_exact_metadata_evaluator(
                &mut registry,
                dimension,
            )
            .is_err()
        );
    }

    #[test]
    fn malformed_expected_values_are_rejected() {
        assert!(
            CompatibilityRequirement::new(
                dimension("abi"),
                " ",
            )
            .is_err()
        );

        assert!(
            CompatibilityDimension::new("")
                .is_err()
        );
    }

    #[test]
    fn compatibility_reports_preserve_individual_dimensions() {
        let report = profile().evaluate(
            &complete_candidate(),
        );

        assert_eq!(
            report
                .observations_for(&dimension("abi"))
                .count(),
            1,
        );

        assert_eq!(
            report
                .observations_for(&dimension("quantum.ir"))
                .count(),
            0,
        );
    }

    #[test]
    fn registration_failure_does_not_silently_pass() {
        let mut registry =
            ConstraintEvaluatorRegistry::new();

        let mut profile =
            CompatibilityProfile::new();

        profile
            .require(
                dimension("abi"),
                "zamani.abi.v1",
            )
            .expect("requirement");

        profile
            .register_evaluators(&mut registry)
            .expect("first registration");

        let result: Result<()> =
            profile.register_evaluators(&mut registry);

        assert!(result.is_err());
    }
}
