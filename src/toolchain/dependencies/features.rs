
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Dependency Features
//!
//! Path: `src/toolchain/dependencies/features.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the feature-activation assessment and feature-dependent
//! dependency-expansion contract used by the package dependency resolver.
//!
//! A package feature is a package-level capability or configuration
//! exposed by a particular package candidate. It is not a language
//! compiler feature gate, a source-language keyword, or a hardware target.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Feature activation assessment.
//! - Validation of requested features against candidate feature evidence.
//! - Distinguishing proven feature absence from incomplete evidence.
//! - Tracking previously activated and newly activated features.
//! - Collecting dependency declarations associated with newly activated
//!   package features.
//!
//! This module does not own:
//!
//! - Canonical dependency names or dependency declarations.
//! - The `DependencyFeature` identity type.
//! - Manifest parsing or manifest feature syntax.
//! - Language/compiler feature-gate lifecycle.
//! - Language-version compatibility.
//! - Candidate discovery.
//! - Version interpretation.
//! - Dependency candidate selection.
//! - Dependency graph construction.
//! - Resolver backtracking.
//! - Optional-dependency inclusion policy.
//! - Registry access or package downloads.
//! - Package installation.
//! - Lockfile serialization.
//! - Hardware discovery or target selection.
//! - Compiler IR or runtime scheduling.
//!
//! ## Canonical authorities
//!
//! Dependency declarations:
//!     `super::model`
//!
//! Dependency feature identity and candidate evidence:
//!     `super::constraints`
//!
//! Candidate feature metadata:
//!     `super::resolver::DependencyCandidate`
//!
//! Dependency solving and feature-unification orchestration:
//!     `super::resolver::DependencyResolver`
//!
//! Persisted activated feature selections:
//!     `super::lockfile`
//!
//! Manifest feature normalization:
//!     `crate::toolchain::manifest::adapter`
//!
//! Language/compiler feature gates:
//!     `grammar/compatibility/feature-gates.md`
//!
//! Source-language feature-selection syntax:
//!     `grammar/compile/feature-selection.g4`
//!
//! ## Evidence semantics
//!
//! Candidate feature inventories have explicit completeness:
//!
//! - Complete: an unlisted feature is established to be absent.
//! - Partial: an unlisted feature has unknown availability.
//! - Unavailable: no usable feature inventory is available.
//!
//! A feature observed in a partial inventory is known to be present.
//! A feature missing from a partial or unavailable inventory is not
//! established to be absent.
//!
//! The module therefore returns three distinct outcomes:
//!
//! - Ready: every requested feature is established to be available.
//! - Rejected: at least one requested feature is proven absent.
//! - Indeterminate: at least one requested feature cannot be verified.
//!
//! Unknown evidence must never be interpreted as successful activation.
//!
//! ## Activation semantics
//!
//! The resolver supplies:
//!
//! - The selected candidate.
//! - Features already activated for that candidate.
//! - All features currently requested by incoming declarations.
//!
//! The activation assessment computes the newly activated feature set.
//! Only dependencies attached to newly activated features are returned.
//!
//! The resolver remains responsible for inserting those declarations
//! into its pending dependency state, applying optional-dependency
//! policy, and continuing candidate selection/backtracking.
//!
//! ## Determinism
//!
//! Feature identities and activation sets use `BTreeSet`.
//! Feature-specific dependency groups are visited in feature order.
//! Dependency declaration order within each candidate-provided feature
//! group is preserved.
//!
//! Candidate providers must supply deterministic candidate metadata,
//! including deterministic declaration order where that order is
//! observable.
//!
//! ## Scalability
//!
//! This module imposes no arbitrary maximum on:
//!
//! - Features.
//! - Dependencies activated by features.
//! - Packages.
//! - Feature names.
//! - Feature groups.
//! - Computing domains.
//!
//! Memory use is proportional to the feature sets and dependency
//! declarations being assessed, subject to available memory and allocator
//! capacity.
//!
//! Operational budgets belong to the resolver and its caller.
//! Literal infinite computation or storage is not promised.
//!
//! ## Security
//!
//! This module performs no filesystem, network, process, environment,
//! credential, registry, or hardware access.
//!
//! Candidate declarations and feature evidence must be validated by
//! their owning model/provider boundaries before use.

use std::collections::BTreeSet;

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::result::Result;

use super::constraints::{
    Completeness,
    DependencyFeature,
};
use super::model::{
    DependencyDeclaration,
    DependencyName,
};
use super::resolver::DependencyCandidate;

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const OPERATION_FEATURE_ACTIVATION: &str =
    "dependency.features.activate";

const OPERATION_FEATURE_VALIDATION: &str =
    "dependency.features.validate";

// ============================================================================
// FEATURE ACTIVATION ISSUE
// ============================================================================

/// Classification of a feature activation issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FeatureActivationIssueKind {
    /// The candidate's complete inventory establishes that a requested
    /// feature is absent.
    Missing,

    /// The candidate's feature inventory is incomplete, so availability
    /// cannot be established.
    Unverified,
}

/// Structured information about an unsuccessful feature assessment.
///
/// The issue does not itself decide whether another candidate should be
/// selected. That decision belongs to the dependency resolver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureActivationIssue {
    package: DependencyName,
    kind: FeatureActivationIssueKind,
    features: BTreeSet<DependencyFeature>,
}

impl FeatureActivationIssue {
    fn new(
        package: DependencyName,
        kind: FeatureActivationIssueKind,
        features: BTreeSet<DependencyFeature>,
    ) -> Self {
        Self {
            package,
            kind,
            features,
        }
    }

    /// Return the canonical package name.
    #[must_use]
    pub fn package(&self) -> &DependencyName {
        &self.package
    }

    /// Return the issue classification.
    #[must_use]
    pub const fn kind(&self) -> FeatureActivationIssueKind {
        self.kind
    }

    /// Return the features involved in the issue.
    ///
    /// The returned set is in canonical order.
    #[must_use]
    pub fn features(&self) -> &BTreeSet<DependencyFeature> {
        &self.features
    }

    /// Return whether the issue represents proven feature absence.
    #[must_use]
    pub const fn is_missing(&self) -> bool {
        matches!(self.kind, FeatureActivationIssueKind::Missing)
    }

    /// Return whether the issue represents incomplete evidence.
    #[must_use]
    pub const fn is_unverified(&self) -> bool {
        matches!(self.kind, FeatureActivationIssueKind::Unverified)
    }
}

// ============================================================================
// FEATURE ACTIVATION PLAN
// ============================================================================

/// A validated feature-activation plan for one selected package candidate.
///
/// The plan is an immutable assessment result. It does not mutate the
/// candidate, dependency resolver, or dependency graph.
///
/// The resolver consumes the plan and applies its dependency declarations
/// to the resolution search state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureActivationPlan {
    package: DependencyName,
    activated_features: BTreeSet<DependencyFeature>,
    newly_activated_features: BTreeSet<DependencyFeature>,
    dependencies: Vec<DependencyDeclaration>,
}

impl FeatureActivationPlan {
    /// Assess requested features against a selected candidate.
    ///
    /// `already_activated` contains features previously expanded for this
    /// candidate. `requested` contains the accumulated feature requests
    /// from all incoming dependency declarations.
    ///
    /// The returned assessment distinguishes:
    ///
    /// - Proven feature availability.
    /// - Proven feature absence.
    /// - Incomplete feature evidence.
    ///
    /// Feature-dependent package declarations are returned only for
    /// features that are newly activated by this assessment.
    ///
    /// Optional declarations are preserved. The resolver decides whether
    /// they are included according to its configured optional-dependency
    /// policy.
    ///
    /// # Errors
    ///
    /// Returns an error if the candidate or its declarations fail their
    /// canonical validation contract.
    pub fn assess(
        candidate: &DependencyCandidate,
        already_activated: &BTreeSet<DependencyFeature>,
        requested: &BTreeSet<DependencyFeature>,
    ) -> Result<FeatureActivationAssessment> {
        candidate.validate()?;

        let inventory = candidate.facts().features();

        let mut missing = BTreeSet::new();
        let mut unverified = BTreeSet::new();

        for feature in requested {
            if inventory.contains(feature) {
                continue;
            }

            if inventory.establishes_absence() {
                missing.insert(feature.clone());
            } else {
                unverified.insert(feature.clone());
            }
        }

        // A complete inventory provides authoritative evidence of absence.
        // Do not downgrade a proven missing feature to an indeterminate
        // result merely because other facts are incomplete elsewhere.
        if !missing.is_empty() {
            return Ok(FeatureActivationAssessment::Rejected(
                FeatureActivationIssue::new(
                    candidate.name().clone(),
                    FeatureActivationIssueKind::Missing,
                    missing,
                ),
            ));
        }

        // An incomplete inventory cannot establish absence or availability
        // for a feature that was not observed.
        if !unverified.is_empty() {
            return Ok(FeatureActivationAssessment::Indeterminate(
                FeatureActivationIssue::new(
                    candidate.name().clone(),
                    FeatureActivationIssueKind::Unverified,
                    unverified,
                ),
            ));
        }

        let newly_activated_features: BTreeSet<_> = requested
            .difference(already_activated)
            .cloned()
            .collect();

        let mut activated_features = already_activated.clone();
        activated_features.extend(requested.iter().cloned());

        let mut dependencies = Vec::new();

        // BTreeSet iteration gives deterministic feature-group ordering.
        // The declaration order inside each group is the candidate's
        // provider-supplied order and must be deterministic at that boundary.
        for feature in &newly_activated_features {
            if let Some(feature_dependencies) =
                candidate.feature_dependencies().get(feature)
            {
                for declaration in feature_dependencies {
                    // The resolver also deduplicates declarations while
                    // accumulating pending dependencies. Avoid redundant
                    // copies within this activation plan as well.
                    if !dependencies.contains(declaration) {
                        dependencies.push(declaration.clone());
                    }
                }
            }
        }

        Ok(FeatureActivationAssessment::Ready(Self {
            package: candidate.name().clone(),
            activated_features,
            newly_activated_features,
            dependencies,
        }))
    }

    /// Return the canonical package name.
    #[must_use]
    pub fn package(&self) -> &DependencyName {
        &self.package
    }

    /// Return the complete feature set after this activation.
    #[must_use]
    pub fn activated_features(&self) -> &BTreeSet<DependencyFeature> {
        &self.activated_features
    }

    /// Return only the features newly activated by this assessment.
    #[must_use]
    pub fn newly_activated_features(
        &self,
    ) -> &BTreeSet<DependencyFeature> {
        &self.newly_activated_features
    }

    /// Return the dependency declarations attached to newly activated
    /// features.
    ///
    /// The resolver must apply its normal optional-dependency policy
    /// before adding these declarations to its pending dependency state.
    #[must_use]
    pub fn dependencies(&self) -> &[DependencyDeclaration] {
        &self.dependencies
    }

    /// Return whether this assessment introduces no new features.
    #[must_use]
    pub fn has_no_new_features(&self) -> bool {
        self.newly_activated_features.is_empty()
    }

    /// Return whether this assessment introduces no feature dependencies.
    #[must_use]
    pub fn has_no_dependencies(&self) -> bool {
        self.dependencies.is_empty()
    }
}

// ============================================================================
// FEATURE ACTIVATION ASSESSMENT
// ============================================================================

/// Result of assessing a requested package feature set.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FeatureActivationAssessment {
    /// All requested features are established as available.
    Ready(FeatureActivationPlan),

    /// At least one requested feature is proven absent from a complete
    /// candidate feature inventory.
    Rejected(FeatureActivationIssue),

    /// At least one requested feature cannot be verified because the
    /// candidate feature inventory is partial or unavailable.
    Indeterminate(FeatureActivationIssue),
}

impl FeatureActivationAssessment {
    /// Borrow the activation plan when all requested features are valid.
    #[must_use]
    pub fn plan(&self) -> Option<&FeatureActivationPlan> {
        match self {
            Self::Ready(plan) => Some(plan),
            Self::Rejected(_) | Self::Indeterminate(_) => None,
        }
    }

    /// Borrow the issue when activation could not be established.
    #[must_use]
    pub fn issue(&self) -> Option<&FeatureActivationIssue> {
        match self {
            Self::Ready(_) => None,
            Self::Rejected(issue) | Self::Indeterminate(issue) => {
                Some(issue)
            }
        }
    }

    /// Return whether all requested features are established as available.
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        matches!(self, Self::Ready(_))
    }

    /// Return whether a requested feature is proven absent.
    #[must_use]
    pub const fn is_rejected(&self) -> bool {
        matches!(self, Self::Rejected(_))
    }

    /// Return whether feature availability is not established.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        matches!(self, Self::Indeterminate(_))
    }
}

// ============================================================================
// FEATURE DECLARATION VALIDATION
// ============================================================================

/// Validate a requested feature set against the canonical feature identity
/// contract.
///
/// This function is useful at boundaries that receive feature identifiers
/// from a source other than `DependencyDeclaration`.
///
/// It does not parse source-language feature-selection syntax. That syntax
/// belongs to the canonical grammar and manifest adapter.
///
/// # Errors
///
/// Returns a structured error when a feature identifier cannot be
/// constructed by the canonical `DependencyFeature` validator.
pub fn validate_feature_names<I, S>(features: I) -> Result<BTreeSet<DependencyFeature>>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut validated = BTreeSet::new();

    for value in features {
        let feature = DependencyFeature::new(value)?;

        if !validated.insert(feature.clone()) {
            return Err(
                ToolchainError::validation_failed(
                    format!(
                        "duplicate dependency feature '{}'",
                        feature.as_str(),
                    ),
                )
                .with_operation(OPERATION_FEATURE_VALIDATION),
            );
        }
    }

    Ok(validated)
}

/// Validate that a dependency declaration's requested feature set is
/// internally consistent.
///
/// `DependencyDeclaration` already validates its feature identifiers and
/// rejects duplicates during construction. This function provides a
/// reusable explicit validation boundary for callers that need to check
/// declarations before resolution.
///
/// # Errors
///
/// Returns a structured error when declaration validation fails.
pub fn validate_declaration_features(
    declaration: &DependencyDeclaration,
) -> Result<()> {
    declaration.validate().map_err(|error| {
        error
            .with_operation(OPERATION_FEATURE_VALIDATION)
            .with_subject(declaration.name().to_string())
    })?;

    for feature in declaration.requested_features() {
        DependencyFeature::new(feature.clone()).map_err(|error| {
            error
                .with_operation(OPERATION_FEATURE_VALIDATION)
                .with_subject(declaration.name().to_string())
        })?;
    }

    Ok(())
}

// ============================================================================
// FEATURE INVENTORY UTILITIES
// ============================================================================

/// Return the completeness classification of a candidate's feature
/// inventory.
///
/// This is an inspection helper. It does not infer that unlisted features
/// are absent unless the inventory is complete.
#[must_use]
pub fn feature_inventory_completeness(
    candidate: &DependencyCandidate,
) -> Completeness {
    candidate.facts().features().completeness()
}

/// Return whether a candidate has established evidence for a feature.
///
/// A `false` result does not necessarily mean that the feature is absent.
/// Call [`feature_inventory_completeness`] to determine whether absence
/// can be established.
#[must_use]
pub fn candidate_has_observed_feature(
    candidate: &DependencyCandidate,
    feature: &DependencyFeature,
) -> bool {
    candidate.facts().features().contains(feature)
}

/// Return whether a candidate's feature inventory can establish the
/// absence of an unobserved feature.
#[must_use]
pub fn candidate_feature_absence_is_established(
    candidate: &DependencyCandidate,
) -> bool {
    candidate.facts().features().establishes_absence()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::dependencies::constraints::{
        CandidateVersion,
        DependencyCandidateFacts,
        EvidenceSet,
        VersionSchemeId,
    };

    use crate::toolchain::dependencies::resolver::{
        CandidateId,
        DependencyCandidate,
    };

    fn feature(value: &str) -> DependencyFeature {
        DependencyFeature::new(value)
            .expect("test feature must be valid")
    }

    fn candidate(
        inventory: EvidenceSet<DependencyFeature>,
        feature_dependencies: std::collections::BTreeMap<
            DependencyFeature,
            Vec<DependencyDeclaration>,
        >,
    ) -> DependencyCandidate {
        let version = CandidateVersion::new(
            VersionSchemeId::new("test.scheme")
                .expect("valid scheme"),
            "1.0.0",
        )
        .expect("valid candidate version");

        let facts = DependencyCandidateFacts::new()
            .with_version(version)
            .with_feature_inventory(inventory);

        DependencyCandidate::new(
            CandidateId::new("test-candidate")
                .expect("valid candidate ID"),
            DependencyName::new("test.package")
                .expect("valid package name"),
            facts,
            Vec::new(),
            feature_dependencies,
        )
        .expect("valid candidate")
    }

    #[test]
    fn complete_inventory_rejects_proven_missing_feature() {
        let candidate = candidate(
            EvidenceSet::complete([feature("compute")]),
            Default::default(),
        );

        let requested = BTreeSet::from([feature("quantum")]);

        let result = FeatureActivationPlan::assess(
            &candidate,
            &BTreeSet::new(),
            &requested,
        )
        .expect("assessment should execute");

        assert!(matches!(
            result,
            FeatureActivationAssessment::Rejected(ref issue)
                if issue.kind() == FeatureActivationIssueKind::Missing
        ));
    }

    #[test]
    fn partial_inventory_does_not_claim_missing_feature() {
        let candidate = candidate(
            EvidenceSet::partial([feature("compute")]),
            Default::default(),
        );

        let requested = BTreeSet::from([feature("quantum")]);

        let result = FeatureActivationPlan::assess(
            &candidate,
            &BTreeSet::new(),
            &requested,
        )
        .expect("assessment should execute");

        assert!(matches!(
            result,
            FeatureActivationAssessment::Indeterminate(ref issue)
                if issue.kind() == FeatureActivationIssueKind::Unverified
        ));
    }

    #[test]
    fn unavailable_inventory_is_indeterminate() {
        let candidate = candidate(
            EvidenceSet::unavailable(),
            Default::default(),
        );

        let requested = BTreeSet::from([feature("training")]);

        let result = FeatureActivationPlan::assess(
            &candidate,
            &BTreeSet::new(),
            &requested,
        )
        .expect("assessment should execute");

        assert!(result.is_indeterminate());
    }

    #[test]
    fn observed_feature_is_available_in_partial_inventory() {
        let candidate = candidate(
            EvidenceSet::partial([feature("training")]),
            Default::default(),
        );

        let requested = BTreeSet::from([feature("training")]);

        let result = FeatureActivationPlan::assess(
            &candidate,
            &BTreeSet::new(),
            &requested,
        )
        .expect("assessment should execute");

        let plan = result.plan().expect("feature should be available");

        assert_eq!(
            plan.newly_activated_features(),
            &BTreeSet::from([feature("training")]),
        );
    }

    #[test]
    fn previously_activated_features_are_not_expanded_again() {
        let candidate = candidate(
            EvidenceSet::complete([feature("compute")]),
            Default::default(),
        );

        let already = BTreeSet::from([feature("compute")]);
        let requested = BTreeSet::from([feature("compute")]);

        let result = FeatureActivationPlan::assess(
            &candidate,
            &already,
            &requested,
        )
        .expect("assessment should execute");

        let plan = result.plan().expect("feature should be available");

        assert!(plan.has_no_new_features());
        assert!(plan.has_no_dependencies());
        assert_eq!(plan.activated_features(), &already);
    }

    #[test]
    fn activation_is_deterministic() {
        let candidate = candidate(
            EvidenceSet::complete([
                feature("training"),
                feature("inference"),
                feature("tensor"),
            ]),
            Default::default(),
        );

        let requested = BTreeSet::from([
            feature("tensor"),
            feature("training"),
            feature("inference"),
        ]);

        let first = FeatureActivationPlan::assess(
            &candidate,
            &BTreeSet::new(),
            &requested,
        )
        .expect("first assessment");

        let second = FeatureActivationPlan::assess(
            &candidate,
            &BTreeSet::new(),
            &requested,
        )
        .expect("second assessment");

        assert_eq!(first, second);
    }

    #[test]
    fn feature_name_validation_rejects_duplicates() {
        let result = validate_feature_names([
            "training",
            "training",
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn feature_name_validation_returns_canonical_order() {
        let result = validate_feature_names([
            "training",
            "inference",
            "tensor",
        ])
        .expect("valid feature names");

        let names: Vec<_> = result
            .iter()
            .map(DependencyFeature::as_str)
            .collect();

        assert_eq!(
            names,
            vec!["inference", "tensor", "training"],
        );
    }

    #[test]
    fn candidate_inventory_completeness_is_preserved() {
        let candidate = candidate(
            EvidenceSet::partial([feature("compute")]),
            Default::default(),
        );

        assert_eq!(
            feature_inventory_completeness(&candidate),
            Completeness::Partial,
        );

        assert!(!candidate_feature_absence_is_established(&candidate));
    }

    #[test]
    fn candidate_feature_evidence_is_not_inferred_from_name() {
        let candidate = candidate(
            EvidenceSet::complete([feature("classical")]),
            Default::default(),
        );

        assert!(!candidate_has_observed_feature(
            &candidate,
            &feature("quantum"),
        ));

        assert!(candidate_feature_absence_is_established(&candidate));
    }
}
