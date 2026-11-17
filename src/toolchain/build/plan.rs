
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Build Plan
//!
//! Path: `src/toolchain/build/plan.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the immutable, validated representation of one logical
//! toolchain build plan and its subsequent compiler feasibility assessment.
//!
//! A logical build may produce multiple target-specific realizations.
//! This module preserves the relationship between the original request,
//! the resolved target identities, and the authoritative feasibility report.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The immutable build-plan representation.
//! - The mapping from public target selectors to canonical target IDs.
//! - Validation that target resolution accounts for the caller's intent.
//! - Association of a plan with compiler feasibility evidence.
//! - Queries describing which selected realizations are proven feasible.
//!
//! This module does not own:
//!
//! - Project or workspace discovery.
//! - Manifest parsing or validation.
//! - Dependency resolution.
//! - Source parsing or semantic analysis.
//! - Semantic requirements, capabilities, preferences, constraints or hints.
//! - Target registration or target discovery.
//! - Capability matching.
//! - Resource planning or physical resource allocation.
//! - Feasibility analysis.
//! - Compiler configuration or compilation.
//! - Backend selection or code generation.
//! - Build-task graph construction.
//! - Task scheduling or execution.
//! - Artifact construction, storage or verification.
//! - Runtime execution or deployment.
//!
//! These responsibilities remain with their canonical subsystems.
//!
//! ## Canonical integration authorities
//!
//! - `crate::toolchain::build::request`
//! - `crate::toolchain::api::request`
//! - `crate::toolchain::ids`
//! - `crate::toolchain::errors`
//! - `crate::compiler::configuration::target`
//! - `crate::compiler::planning::feasibility`
//! - `crate::compiler::planning::target_discovery`
//! - `crate::compiler::planning::capability_matching`
//! - `crate::compiler::planning::resource_planning`
//!
//! ## POCO-REAF
//!
//! One logical build request may have multiple compatible target
//! realizations. The plan preserves one request identity while allowing
//! target-specific outcomes.
//!
//! A requested target is intent, not proof of feasibility.
//! A discovered target is not proof of resource availability.
//! A feasible target assessment is not a reservation or execution guarantee.
//! Runtime availability must be checked again by the runtime authority.
//!
//! No universal maximum is imposed on targets, domains, processors,
//! accelerators, quantum resources, memory, tensor dimensions, devices,
//! distributed participants, or build-plan entries.
//!
//! Actual operation is subject to available resources, Rust's
//! representational constraints, and explicitly configured operational
//! budgets. Literal infinite computation or storage is not promised.
//!
//! ## Determinism
//!
//! Resolved targets and unresolved selectors are stored in stable order.
//! No timestamps, random identifiers, global mutable state, or implicit
//! host-target defaults are introduced.
//!
//! The BuildId is supplied by the owning build service. This module
//! does not create a global ID allocator.
//!
//! ## Serialization
//!
//! These are in-process orchestration types, not persisted or wire-format
//! contracts. A future persisted build-plan format must have its own
//! explicit schema version and compatibility policy. It must not serialize
//! compiler-private feasibility internals by accident.

use std::error::Error;
use std::fmt;

use crate::compiler::configuration::target::TargetId;
use crate::compiler::planning::feasibility::{
    FeasibilityReport,
    TargetFeasibility,
};

use crate::toolchain::api::TargetSelector;
use crate::toolchain::build::request::BuildRequest;
use crate::toolchain::errors::ToolchainError;
use crate::toolchain::ids::BuildId;
use crate::toolchain::result::ToolchainResult;

// ============================================================================
// ERROR HELPERS
// ============================================================================

fn invalid_plan(message: impl Into<String>) -> ToolchainError {
    ToolchainError::validation_failed(message)
        .with_operation("build.plan")
}

fn allocation_failure(
    subject: &'static str,
    error: std::collections::TryReserveError,
) -> ToolchainError {
    ToolchainError::resource_exhausted(format!(
        "unable to reserve memory for {subject}: {error}"
    ))
    .with_operation("build.plan")
}

// ============================================================================
// RESOLVED TARGET
// ============================================================================

/// One target identity selected by the authoritative target-selection
/// integration.
///
/// `target_id` is the canonical compiler target identity. It is not
/// a caller-supplied target string or a physical device identity.
///
/// `selector` records the public selector that led to this target when
/// the resolution service can establish that relationship.
///
/// A `None` selector is valid for a target selected through
/// capability-driven discovery rather than an explicit selector.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedBuildTarget {
    target_id: TargetId,
    selector: Option<TargetSelector>,
}

impl ResolvedBuildTarget {
    /// Construct a target selected through capability-driven resolution.
    ///
    /// The target identity must originate from the canonical compiler
    /// target-selection and discovery authorities.
    #[must_use]
    pub fn new(target_id: TargetId) -> Self {
        Self {
            target_id,
            selector: None,
        }
    }

    /// Construct a target resolved from an explicit selector or
    /// a recorded target preference.
    ///
    /// This constructor records the mapping; it does not perform target
    /// discovery or prove compatibility.
    #[must_use]
    pub fn for_selector(
        selector: TargetSelector,
        target_id: TargetId,
    ) -> Self {
        Self {
            target_id,
            selector: Some(selector),
        }
    }

    /// Borrow the canonical compiler target identity.
    #[must_use]
    pub fn target_id(&self) -> &TargetId {
        &self.target_id
    }

    /// Borrow the selector associated with this resolution, if any.
    #[must_use]
    pub fn selector(&self) -> Option<&TargetSelector> {
        self.selector.as_ref()
    }
}

// ============================================================================
// TARGET RESOLUTION
// ============================================================================

/// The result of resolving the target-selection intent in a build request.
///
/// This is an adapter result supplied by the authoritative target-selection
/// integration. It is not a target registry or discovery implementation.
///
/// Explicit target requirements must be accounted for exactly:
///
/// - A selector is either resolved to a canonical target identity, or
/// - It is reported as unresolved.
///
/// Capability-driven selection may return any number of selected targets.
/// Unmatched preferences do not become mandatory requirements.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BuildTargetResolution {
    resolved: Vec<ResolvedBuildTarget>,
    unresolved: Vec<TargetSelector>,
}

impl BuildTargetResolution {
    /// Construct a deterministic target-resolution result.
    ///
    /// Duplicate canonical target identities and duplicate unresolved
    /// selectors are rejected.
    ///
    /// The relationship between the result and a particular build request
    /// is checked by `BuildPlan::new`.
    ///
    /// # Errors
    ///
    /// Returns a toolchain error if memory reservation fails or duplicate
    /// target identities or unresolved selectors are supplied.
    pub fn new(
        resolved: impl IntoIterator<Item = ResolvedBuildTarget>,
        unresolved: impl IntoIterator<Item = TargetSelector>,
    ) -> ToolchainResult<Self> {
        let mut resolved_targets = Vec::new();
        let mut resolved_iter = resolved.into_iter();

        let (lower, _) = resolved_iter.size_hint();

        resolved_targets
            .try_reserve(lower)
            .map_err(|error| {
                allocation_failure("resolved build targets", error)
            })?;

        for target in resolved_iter {
            resolved_targets
                .try_reserve(1)
                .map_err(|error| {
                    allocation_failure("resolved build targets", error)
                })?;

            resolved_targets.push(target);
        }

        resolved_targets.sort_unstable_by(|left, right| {
            left.target_id.cmp(&right.target_id)
        });

        for pair in resolved_targets.windows(2) {
            if pair[0].target_id == pair[1].target_id {
                return Err(invalid_plan(format!(
                    "duplicate resolved target identity: {}",
                    pair[0].target_id
                )));
            }
        }

        let mut unresolved_selectors = Vec::new();
        let mut unresolved_iter = unresolved.into_iter();

        let (lower, _) = unresolved_iter.size_hint();

        unresolved_selectors
            .try_reserve(lower)
            .map_err(|error| {
                allocation_failure("unresolved target selectors", error)
            })?;

        for selector in unresolved_iter {
            unresolved_selectors
                .try_reserve(1)
                .map_err(|error| {
                    allocation_failure(
                        "unresolved target selectors",
                        error,
                    )
                })?;

            unresolved_selectors.push(selector);
        }

        unresolved_selectors.sort_unstable();

        for pair in unresolved_selectors.windows(2) {
            if pair[0] == pair[1] {
                return Err(invalid_plan(format!(
                    "duplicate unresolved target selector: {}",
                    pair[0]
                )));
            }
        }

        Ok(Self {
            resolved: resolved_targets,
            unresolved: unresolved_selectors,
        })
    }

    /// Borrow all resolved targets in deterministic target-ID order.
    #[must_use]
    pub fn resolved(&self) -> &[ResolvedBuildTarget] {
        &self.resolved
    }

    /// Borrow explicit selectors that could not be resolved.
    ///
    /// This collection is meaningful for explicit target selection.
    /// It must be empty for capability-driven selection.
    #[must_use]
    pub fn unresolved(&self) -> &[TargetSelector] {
        &self.unresolved
    }

    /// Return the number of resolved target identities.
    #[must_use]
    pub fn len(&self) -> usize {
        self.resolved.len()
    }

    /// Return whether no targets were resolved.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.resolved.is_empty()
    }

    /// Validate that this resolution accounts for the supplied
    /// target-selection intent.
    ///
    /// Explicit selectors must be partitioned exactly between resolved
    /// targets and unresolved selectors. Capability preferences remain
    /// preferences and are never promoted to mandatory requirements.
    ///
    /// # Errors
    ///
    /// Returns an error if the resolution contradicts the request.
    fn validate_against(
        &self,
        selection: &crate::toolchain::api::TargetSelection,
    ) -> ToolchainResult<()> {
        use crate::toolchain::api::TargetSelection;

        match selection {
            TargetSelection::Explicit { targets, .. } => {
                let mut actual: Vec<&TargetSelector> = Vec::new();

                actual
                    .try_reserve(
                        self.resolved.len()
                            .saturating_add(self.unresolved.len()),
                    )
                    .map_err(|error| {
                        allocation_failure(
                            "explicit target selector validation",
                            error,
                        )
                    })?;

                for resolved in &self.resolved {
                    let selector = resolved.selector.as_ref().ok_or_else(
                        || {
                            invalid_plan(format!(
                                "explicit target resolution for {} \
                                 does not retain its originating selector",
                                resolved.target_id
                            ))
                        },
                    )?;

                    actual.push(selector);
                }

                actual.extend(self.unresolved.iter());

                actual.sort_unstable();

                let mut expected: Vec<&TargetSelector> = Vec::new();

                expected
                    .try_reserve(targets.len())
                    .map_err(|error| {
                        allocation_failure(
                            "requested target selector validation",
                            error,
                        )
                    })?;

                expected.extend(targets.iter());
                expected.sort_unstable();

                if actual != expected {
                    return Err(invalid_plan(
                        "resolved and unresolved target selectors do not \
                         account for the explicit target request exactly",
                    ));
                }
            }

            TargetSelection::ResolveFromCapabilities { preferences } => {
                if !self.unresolved.is_empty() {
                    return Err(invalid_plan(
                        "capability-driven target selection must not report \
                         unresolved mandatory target selectors",
                    ));
                }

                for resolved in &self.resolved {
                    if let Some(selector) = &resolved.selector {
                        if !preferences.contains(selector) {
                            return Err(invalid_plan(format!(
                                "resolved target {} references a selector \
                                 that was not declared as a preference: {}",
                                resolved.target_id,
                                selector
                            )));
                        }
                    }
                }
            }

            // TargetSelection is non-exhaustive. Unknown future variants
            // must not silently inherit the semantics of an existing one.
            _ => {
                return Err(ToolchainError::unsupported_operation(
                    "this version of the build planner does not understand \
                     the supplied target-selection variant",
                )
                .with_operation("build.plan"));
            }
        }

        Ok(())
    }
}

// ============================================================================
// BUILD PLAN
// ============================================================================

/// One immutable logical build plan.
///
/// The plan preserves the original validated request and the result of
/// canonical target selection.
///
/// It is not executable merely because it exists. It must first be
/// associated with a compiler feasibility report through `assess`.
///
/// The BuildId is allocated by the owning build service and supplied here.
/// This type does not create hidden global state or generate identifiers.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "a build plan must be assessed before execution is considered"]
pub struct BuildPlan {
    id: BuildId,
    request: BuildRequest,
    target_resolution: BuildTargetResolution,
}

impl BuildPlan {
    /// Construct a build plan from a validated request and target-resolution
    /// result.
    ///
    /// The caller must obtain the target-resolution result from the
    /// canonical compiler target-selection integration.
    ///
    /// This constructor performs structural consistency checks only.
    /// It does not access the filesystem, parse the manifest, resolve
    /// dependencies, discover hardware, assess feasibility or compile.
    ///
    /// # Errors
    ///
    /// Returns an error if the request or target-resolution result is
    /// structurally invalid or inconsistent.
    pub fn new(
        id: BuildId,
        request: BuildRequest,
        target_resolution: BuildTargetResolution,
    ) -> ToolchainResult<Self> {
        request.validate().map_err(|error| {
            ToolchainError::validation_failed(format!(
                "invalid build request: {error}"
            ))
            .with_operation("build.plan")
        })?;

        target_resolution.validate_against(request.targets())?;

        Ok(Self {
            id,
            request,
            target_resolution,
        })
    }

    /// Return the identity of this logical build operation.
    #[must_use]
    pub fn id(&self) -> &BuildId {
        &self.id
    }

    /// Borrow the immutable validated build request.
    #[must_use]
    pub fn request(&self) -> &BuildRequest {
        &self.request
    }

    /// Borrow the canonical target-resolution result.
    #[must_use]
    pub fn target_resolution(&self) -> &BuildTargetResolution {
        &self.target_resolution
    }

    /// Associate this plan with an authoritative compiler feasibility report.
    ///
    /// Every target selected by this plan must have a corresponding
    /// target assessment in the report. An unassessed selected target
    /// cannot be treated as buildable.
    ///
    /// The report is produced by the compiler planning subsystem after
    /// semantic validation, capability matching, resource planning and
    /// applicable evidence collection.
    ///
    /// This method does not recompute or reinterpret that evidence.
    ///
    /// # Errors
    ///
    /// Returns an error if the report does not contain an assessment
    /// for every selected target.
    pub fn assess(
        self,
        feasibility: FeasibilityReport,
    ) -> ToolchainResult<AssessedBuildPlan> {
        for target in self.target_resolution.resolved() {
            if feasibility.target(target.target_id()).is_none() {
                return Err(invalid_plan(format!(
                    "compiler feasibility report has no assessment for \
                     selected target {}",
                    target.target_id()
                )));
            }
        }

        Ok(AssessedBuildPlan {
            plan: self,
            feasibility,
        })
    }
}

// ============================================================================
// ASSESSED BUILD PLAN
// ============================================================================

/// A build plan associated with authoritative compiler feasibility evidence.
///
/// This type distinguishes a structurally valid plan from one whose
/// selected targets have actually been assessed.
///
/// It still does not mean compilation has succeeded, artifacts exist,
/// resources are reserved, or runtime execution is guaranteed.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "inspect feasibility before submitting the plan for execution"]
pub struct AssessedBuildPlan {
    plan: BuildPlan,
    feasibility: FeasibilityReport,
}

impl AssessedBuildPlan {
    /// Borrow the underlying immutable build plan.
    #[must_use]
    pub fn plan(&self) -> &BuildPlan {
        &self.plan
    }

    /// Borrow the authoritative compiler feasibility report.
    #[must_use]
    pub fn feasibility(&self) -> &FeasibilityReport {
        &self.feasibility
    }

    /// Iterate over selected targets that the compiler has proven feasible.
    ///
    /// A feasible result is evidence for attempting compilation. It is not
    /// a physical resource reservation or a guarantee of execution.
    pub fn buildable_targets(
        &self,
    ) -> impl Iterator<Item = &ResolvedBuildTarget> {
        self.plan
            .target_resolution
            .resolved()
            .iter()
            .filter(|target| {
                self.feasibility
                    .target(target.target_id())
                    .is_some_and(TargetFeasibility::is_feasible)
            })
    }

    /// Iterate over selected targets that are not proven feasible.
    ///
    /// This includes definitively infeasible and indeterminate targets.
    pub fn non_buildable_targets(
        &self,
    ) -> impl Iterator<Item = &ResolvedBuildTarget> {
        self.plan
            .target_resolution
            .resolved()
            .iter()
            .filter(|target| {
                !self.feasibility
                    .target(target.target_id())
                    .is_some_and(TargetFeasibility::is_feasible)
            })
    }

    /// Return whether at least one selected target is proven feasible.
    #[must_use]
    pub fn has_buildable_target(&self) -> bool {
        self.buildable_targets().next().is_some()
    }

    /// Return whether the plan may proceed to build-task construction.
    ///
    /// For explicit target selection with `require_all = true`, every
    /// requested selector must have resolved to a feasible target.
    ///
    /// For explicit selection with `require_all = false`, or
    /// capability-driven selection, at least one selected target must
    /// be proven feasible. Non-buildable alternatives remain visible
    /// through `non_buildable_targets` and the feasibility report.
    ///
    /// This method authorizes only progression to the next toolchain
    /// planning stage. It does not authorize physical execution.
    #[must_use]
    pub fn can_attempt_build(&self) -> bool {
        let buildable_count = self.buildable_targets().count();

        if buildable_count == 0 {
            return false;
        }

        match self.plan.request.targets() {
            crate::toolchain::api::TargetSelection::Explicit {
                require_all: true,
                ..
            } => {
                self.plan.target_resolution.unresolved().is_empty()
                    && buildable_count
                        == self.plan.target_resolution.resolved().len()
            }

            crate::toolchain::api::TargetSelection::Explicit {
                require_all: false,
                ..
            }
            | crate::toolchain::api::TargetSelection::ResolveFromCapabilities {
                ..
            } => true,

            // Fail closed for future target-selection variants until
            // their buildability semantics are explicitly implemented.
            _ => false,
        }
    }

    /// Return whether all selected targets are feasible and the compiler's
    /// complete feasibility assessment is exhaustive and decisive.
    ///
    /// This is stronger than `can_attempt_build`.
    #[must_use]
    pub fn is_fully_feasible(&self) -> bool {
        self.can_attempt_build()
            && self.plan.target_resolution.unresolved().is_empty()
            && self.buildable_targets().count()
                == self.plan.target_resolution.resolved().len()
            && self.feasibility.is_complete()
    }

    /// Return whether the compiler completed its feasibility assessment.
    ///
    /// An incomplete assessment must not be described as an exhaustive
    /// inventory, even if one target has already been proven feasible.
    #[must_use]
    pub fn assessment_is_complete(&self) -> bool {
        self.feasibility.is_complete()
    }

    /// Return whether this plan has unresolved explicit target selectors.
    #[must_use]
    pub fn has_unresolved_targets(&self) -> bool {
        !self.plan.target_resolution.unresolved().is_empty()
    }

    /// Borrow explicit target selectors that were not resolved.
    #[must_use]
    pub fn unresolved_targets(&self) -> &[TargetSelector] {
        self.plan.target_resolution.unresolved()
    }

    /// Return whether at least one selected target is not proven feasible.
    #[must_use]
    pub fn has_non_buildable_targets(&self) -> bool {
        self.non_buildable_targets().next().is_some()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::api::{
        BuildRequest as ApiBuildRequest,
        ProjectLocator,
        ProjectScope,
        TargetSelection,
    };

    use crate::toolchain::ids::IdNamespace;

    fn build_id() -> BuildId {
        BuildId::new(
            IdNamespace::new("build-plan-test")
                .expect("valid namespace"),
            1,
        )
        .expect("valid build identity")
    }

    fn project_scope() -> ProjectScope {
        ProjectScope::new(
            ProjectLocator::from_path("examples/minimal")
                .expect("valid project path"),
        )
        .expect("valid project scope")
    }

    fn validated_request(
        selection: TargetSelection,
    ) -> BuildRequest {
        let request = ApiBuildRequest::new(
            project_scope(),
            selection,
        )
        .expect("valid public API request");

        BuildRequest::try_from_api(request)
            .expect("valid internal build request")
    }

    fn target_id(name: &str) -> TargetId {
        TargetId::new(name).expect("valid canonical target ID")
    }

    #[test]
    fn explicit_resolution_accounts_for_every_requested_selector() {
        let first = TargetSelector::new("target.first")
            .expect("valid selector");
        let second = TargetSelector::new("target.second")
            .expect("valid selector");

        let request = validated_request(
            TargetSelection::explicit(
                vec![first.clone(), second.clone()],
                false,
            )
            .expect("valid selection"),
        );

        let resolution = BuildTargetResolution::new(
            vec![
                ResolvedBuildTarget::for_selector(
                    first.clone(),
                    target_id("target.first"),
                ),
            ],
            vec![second.clone()],
        )
        .expect("valid resolution");

        let plan = BuildPlan::new(
            build_id(),
            request,
            resolution,
        )
        .expect("resolution accounts for both selectors");

        assert_eq!(
            plan.target_resolution().resolved().len(),
            1
        );
        assert_eq!(
            plan.target_resolution().unresolved(),
            &[second]
        );
    }

    #[test]
    fn explicit_resolution_rejects_missing_selector_accounting() {
        let first = TargetSelector::new("target.first")
            .expect("valid selector");
        let second = TargetSelector::new("target.second")
            .expect("valid selector");

        let request = validated_request(
            TargetSelection::explicit(
                vec![first.clone(), second],
                false,
            )
            .expect("valid selection"),
        );

        let resolution = BuildTargetResolution::new(
            vec![
                ResolvedBuildTarget::for_selector(
                    first,
                    target_id("target.first"),
                ),
            ],
            Vec::new(),
        )
        .expect("resolution itself is structurally valid");

        assert!(
            BuildPlan::new(build_id(), request, resolution).is_err()
        );
    }

    #[test]
    fn explicit_resolution_rejects_targets_without_originating_selectors() {
        let selector = TargetSelector::new("target.first")
            .expect("valid selector");

        let request = validated_request(
            TargetSelection::explicit(
                vec![selector],
                true,
            )
            .expect("valid selection"),
        );

        let resolution = BuildTargetResolution::new(
            vec![
                ResolvedBuildTarget::new(
                    target_id("target.first"),
                ),
            ],
            Vec::new(),
        )
        .expect("resolution itself is structurally valid");

        assert!(
            BuildPlan::new(build_id(), request, resolution).is_err()
        );
    }

    #[test]
    fn duplicate_canonical_target_ids_are_rejected() {
        let id = target_id("target.first");

        let result = BuildTargetResolution::new(
            vec![
                ResolvedBuildTarget::new(id.clone()),
                ResolvedBuildTarget::new(id),
            ],
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_unresolved_selectors_are_rejected() {
        let selector = TargetSelector::new("target.first")
            .expect("valid selector");

        let result = BuildTargetResolution::new(
            Vec::new(),
            vec![selector.clone(), selector],
        );

        assert!(result.is_err());
    }

    #[test]
    fn capability_resolution_preserves_preferences_without_promoting_them() {
        let preference = TargetSelector::new("target.preferred")
            .expect("valid selector");

        let request = validated_request(
            TargetSelection::resolve_from_capabilities(
                vec![preference.clone()],
            )
            .expect("valid capability-driven selection"),
        );

        let resolution = BuildTargetResolution::new(
            vec![
                ResolvedBuildTarget::for_selector(
                    preference,
                    target_id("provider.target"),
                ),
                ResolvedBuildTarget::new(
                    target_id("provider.alternative"),
                ),
            ],
            Vec::new(),
        )
        .expect("valid resolution");

        assert!(
            BuildPlan::new(build_id(), request, resolution).is_ok()
        );
    }

    #[test]
    fn capability_resolution_rejects_unmatched_mandatory_selectors() {
        let request = validated_request(
            TargetSelection::resolve_from_capabilities(Vec::new())
                .expect("valid selection"),
        );

        let unresolved = TargetSelector::new("target.required")
            .expect("valid selector");

        let resolution = BuildTargetResolution::new(
            Vec::new(),
            vec![unresolved],
        )
        .expect("resolution itself is structurally valid");

        assert!(
            BuildPlan::new(build_id(), request, resolution).is_err()
        );
    }

    #[test]
    fn target_resolution_is_sorted_deterministically() {
        let first = target_id("target.alpha");
        let second = target_id("target.zeta");

        let resolution = BuildTargetResolution::new(
            vec![
                ResolvedBuildTarget::new(second.clone()),
                ResolvedBuildTarget::new(first.clone()),
            ],
            Vec::new(),
        )
        .expect("valid resolution");

        assert_eq!(
            resolution.resolved()[0].target_id(),
            &first
        );
        assert_eq!(
            resolution.resolved()[1].target_id(),
            &second
        );
    }

    #[test]
    fn build_plan_preserves_the_supplied_build_identity() {
        let selector = TargetSelector::new("target.portable")
            .expect("valid selector");

        let request = validated_request(
            TargetSelection::explicit(
                vec![selector.clone()],
                true,
            )
            .expect("valid selection"),
        );

        let resolution = BuildTargetResolution::new(
            vec![
                ResolvedBuildTarget::for_selector(
                    selector,
                    target_id("target.portable"),
                ),
            ],
            Vec::new(),
        )
        .expect("valid resolution");

        let id = build_id();

        let plan = BuildPlan::new(
            id.clone(),
            request,
            resolution,
        )
        .expect("valid plan");

        assert_eq!(plan.id(), &id);
    }
}
