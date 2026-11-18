
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Target Selection
//!
//! Path: `src/toolchain/targets/selection.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Selects one target from an explicitly supplied set of candidates after
//! their compatibility has been assessed by the canonical backend evaluator.
//!
//! This module is a deterministic policy adapter. It does not discover,
//! register, probe, allocate, compile for, or execute on targets.
//!
//! ## Architectural authorities
//!
//! Source-level target intent:
//!     grammar/compile/target.g4
//!
//! Source-level selection policy:
//!     grammar/compile/target-selection.g4
//!
//! Compiler-facing target identity and metadata:
//!     crate::compiler::configuration::target
//!
//! Backend target identity and descriptor:
//!     crate::backend::target
//!
//! Capability and compatibility assessment:
//!     crate::backend::compatibility
//!
//! Toolchain target profile configuration:
//!     crate::toolchain::targets::profiles
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Selection policy data.
//! - Deterministic ordering of assessed target candidates.
//! - Exclusion and target-profile scope application.
//! - Selection decisions and their explanations.
//! - Reporting unresolved policy references.
//!
//! ## Non-ownership
//!
//! This module does not own:
//!
//! - Target identity or target descriptors.
//! - Target registration or discovery.
//! - Hardware or environment probing.
//! - Capability declarations or evidence.
//! - Requirements, constraints, or compatibility algorithms.
//! - Physical resource planning or allocation.
//! - Backend selection or code generation.
//! - Quantum routing, scheduling, or error correction.
//! - HDL synthesis, placement, or timing closure.
//! - Runtime execution or deployment.
//!
//! ## Semantic distinctions
//!
//! Requirements and constraints must already have been evaluated by the
//! authoritative compatibility/planning layers before a candidate is passed
//! here.
//!
//! Preferences affect ordering among acceptable candidates. They never make
//! an incompatible or indeterminate candidate acceptable.
//!
//! Hints are non-binding and are not converted into preferences or
//! requirements by this module.
//!
//! A target-profile reference restricts selection only when the profile
//! explicitly declares a non-empty target-reference set. An empty set means
//! unspecified scope, not universal support.
//!
//! ## Compatibility semantics
//!
//! - `Compatible`: eligible for selection.
//! - `CompatibleWithWarnings`: eligible, with warnings retained in the
//!   original compatibility report.
//! - `Incompatible`: rejected.
//! - `Indeterminate`: unresolved and not selected.
//!
//! A compatibility report applies only to the request used to create it.
//! Selection does not certify unrelated workloads, artifacts, ABIs,
//! environments, providers, or future operations.
//!
//! ## POCO-REAF and scalability
//!
//! Target identifiers are open-world. No architecture, vendor, domain,
//! device type, processor count, qubit count, node count, memory size,
//! tensor rank, register width, or target-count ceiling is introduced.
//!
//! The implementation uses dynamically sized standard collections. Its
//! memory and execution costs are proportional to the supplied candidates
//! and policy entries, subject to available machine resources and operational
//! budgets enforced by their owning layers.
//!
//! Literal infinite resources or execution are not promised.
//!
//! ## Determinism
//!
//! Explicit preference order is respected first, followed by explicit
//! fallback order. Candidates not mentioned by either list are ordered by
//! canonical target identity. Input iteration order therefore cannot
//! arbitrarily change the result.
//!
//! ## Security
//!
//! No filesystem, network, environment, process, credential, hardware,
//! or runtime access occurs.
//!
//! Candidate compatibility reports are assumed to have been obtained from
//! trusted, validated upstream authorities. This module checks target/report
//! identity association but does not authenticate provider evidence.
//!
//! ## Integration contract
//!
//! 1. The compiler supplies target intent and semantic requirements.
//! 2. The compiler/backend integration resolves target intent to canonical
//!    backend target descriptors.
//! 3. Backend providers supply capability/resource evidence.
//! 4. `targets::compatibility` evaluates each candidate against the actual
//!    compatibility request.
//! 5. This module applies profile scope, exclusions, preferences, and
//!    fallback ordering.
//! 6. The build matrix records the selected candidate and all decisions.
//! 7. Later build, backend, resource, and runtime stages perform their own
//!    validation. Selection is not a realization or execution guarantee.

use std::collections::{BTreeMap, BTreeSet};

use crate::backend::compatibility::{
    CompatibilityReport,
    CompatibilityStatus,
};

use crate::backend::target::TargetDescriptor as BackendTargetDescriptor;

use crate::compiler::configuration::target::TargetId as CompilerTargetId;

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
    ToolchainResult,
};

use crate::toolchain::targets::profiles::TargetProfileSettings;

/// Stable architectural identity of this module.
pub const TARGET_SELECTION_MODULE_NAME: &str =
    "zamani.toolchain.targets.selection";

/// Public API version of target selection.
pub const TARGET_SELECTION_API_VERSION: u16 = 1;

/// Operation identifier attached to selection errors.
const OPERATION_SELECT: &str = "toolchain.targets.selection.select";

/// One target candidate with its authoritative compatibility assessment.
///
/// The descriptor and report are borrowed. This type does not copy or mutate
/// target metadata or compatibility evidence.
///
/// Construction validates that:
///
/// - The backend descriptor is structurally valid.
/// - The compatibility report identifies exactly the same target.
///
/// A candidate is not automatically eligible. Its compatibility status is
/// evaluated by the selection service.
#[derive(Debug)]
pub struct TargetSelectionCandidate<'a> {
    target: &'a BackendTargetDescriptor,
    compatibility: &'a CompatibilityReport,
}

impl<'a> TargetSelectionCandidate<'a> {
    /// Construct a candidate from a backend descriptor and compatibility
    /// report.
    ///
    /// The report must have been produced for this exact canonical target.
    ///
    /// # Errors
    ///
    /// Returns an error if the descriptor is invalid or if the report's
    /// target identity differs from the descriptor identity.
    pub fn new(
        target: &'a BackendTargetDescriptor,
        compatibility: &'a CompatibilityReport,
    ) -> ToolchainResult<Self> {
        target.validate().map_err(|error| {
            ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                format!(
                    "backend target descriptor `{}` failed validation: {error}",
                    target.id(),
                ),
            )
            .with_operation(OPERATION_SELECT)
            .with_subject(target.id().to_string())
            .with_source(error)
        })?;

        if compatibility.target_id != target.id().as_str() {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::ValidationFailed,
                    format!(
                        "compatibility report target `{}` does not match \
                         candidate target `{}`",
                        compatibility.target_id,
                        target.id(),
                    ),
                )
                .with_operation(OPERATION_SELECT)
                .with_subject(target.id().to_string()),
            );
        }

        Ok(Self {
            target,
            compatibility,
        })
    }

    /// Return the canonical backend target descriptor.
    #[must_use]
    pub fn target(&self) -> &'a BackendTargetDescriptor {
        self.target
    }

    /// Return the original authoritative compatibility report.
    #[must_use]
    pub fn compatibility(&self) -> &'a CompatibilityReport {
        self.compatibility
    }

    /// Return the canonical target identifier.
    #[must_use]
    pub fn target_id(&self) -> &str {
        self.target.id().as_str()
    }
}

/// Deterministic target-selection policy.
///
/// The policy contains only selection-level ordering and exclusion data.
/// Requirements and constraints belong to the compatibility request and
/// upstream semantic/planning authorities.
///
/// Preferred targets are considered in the order supplied. Fallback targets
/// are considered after all preferred targets, also in supplied order.
/// Remaining acceptable candidates are ordered lexicographically by
/// canonical target identity.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TargetSelectionPolicy {
    preferred_targets: Vec<String>,
    fallback_order: Vec<String>,
    excluded_targets: BTreeSet<String>,
}

impl TargetSelectionPolicy {
    /// Construct an empty policy.
    ///
    /// An empty policy does not select an arbitrary first input candidate.
    /// Eligible candidates are instead ordered by canonical target identity.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the explicitly preferred target identifiers.
    #[must_use]
    pub fn preferred_targets(&self) -> &[String] {
        &self.preferred_targets
    }

    /// Return the explicit fallback target order.
    #[must_use]
    pub fn fallback_order(&self) -> &[String] {
        &self.fallback_order
    }

    /// Return explicitly excluded target identifiers.
    #[must_use]
    pub fn excluded_targets(&self) -> &BTreeSet<String> {
        &self.excluded_targets
    }

    /// Replace the preferred target order.
    ///
    /// The operation is transactional: the existing policy is unchanged if
    /// an identifier is invalid or duplicated.
    pub fn set_preferred_targets<I, S>(
        &mut self,
        targets: I,
    ) -> ToolchainResult<()>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let validated = validate_ordered_target_ids(targets)?;
        self.preferred_targets = validated;
        Ok(())
    }

    /// Replace the fallback target order.
    ///
    /// Fallback order is consulted only after explicit preferences.
    /// Duplicate identifiers are rejected rather than assigned ambiguous
    /// positions.
    pub fn set_fallback_order<I, S>(
        &mut self,
        targets: I,
    ) -> ToolchainResult<()>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let validated = validate_ordered_target_ids(targets)?;
        self.fallback_order = validated;
        Ok(())
    }

    /// Replace the excluded target set.
    ///
    /// Duplicate exclusions are harmless and collapse into one set entry.
    /// Invalid identifiers cause the operation to fail without changing the
    /// existing exclusions.
    pub fn set_excluded_targets<I, S>(
        &mut self,
        targets: I,
    ) -> ToolchainResult<()>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut validated = BTreeSet::new();

        for target in targets {
            let target = target.into();
            validate_target_id(&target)?;
            validated.insert(target);
        }

        self.excluded_targets = validated;
        Ok(())
    }

    /// Add one preferred target.
    ///
    /// Returns an error if the target is invalid or already appears in the
    /// preferred list.
    pub fn prefer_target(
        &mut self,
        target: impl Into<String>,
    ) -> ToolchainResult<()> {
        let target = target.into();
        validate_target_id(&target)?;

        if self.preferred_targets.iter().any(|value| value == &target) {
            return Err(selection_input_error(format!(
                "target `{target}` is already present in the preferred order"
            )));
        }

        self.preferred_targets.push(target);
        Ok(())
    }

    /// Add one fallback target.
    ///
    /// Returns an error if the target is invalid or already appears in the
    /// fallback list.
    pub fn add_fallback(
        &mut self,
        target: impl Into<String>,
    ) -> ToolchainResult<()> {
        let target = target.into();
        validate_target_id(&target)?;

        if self.fallback_order.iter().any(|value| value == &target) {
            return Err(selection_input_error(format!(
                "target `{target}` is already present in the fallback order"
            )));
        }

        self.fallback_order.push(target);
        Ok(())
    }

    /// Exclude one target from selection.
    ///
    /// Returns whether this call added a new exclusion.
    pub fn exclude_target(
        &mut self,
        target: impl Into<String>,
    ) -> ToolchainResult<bool> {
        let target = target.into();
        validate_target_id(&target)?;
        Ok(self.excluded_targets.insert(target))
    }

    /// Remove one target from the exclusion set.
    ///
    /// Returns whether the target was excluded.
    pub fn include_target(&mut self, target: &str) -> bool {
        self.excluded_targets.remove(target)
    }

    /// Validate all policy identifiers and internal invariants.
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_unique_ordered_ids(&self.preferred_targets, "preferred")?;
        validate_unique_ordered_ids(&self.fallback_order, "fallback")?;

        for target in &self.excluded_targets {
            validate_target_id(target)?;
        }

        Ok(())
    }
}

/// Why a candidate was classified in a selection pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateDisposition {
    /// Candidate satisfies the supplied compatibility request and is
    /// eligible for selection.
    Eligible,

    /// Candidate is explicitly excluded by selection policy.
    ExcludedByPolicy,

    /// Candidate is outside an explicitly declared target-profile scope.
    OutsideProfileScope,

    /// The authoritative compatibility evaluator established that the
    /// candidate cannot satisfy the request.
    Incompatible,

    /// Available evidence cannot establish compatibility.
    ///
    /// An indeterminate candidate is never selected.
    Indeterminate,
}

/// The selection decision recorded for one candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateDecision {
    target_id: String,
    disposition: CandidateDisposition,
}

impl CandidateDecision {
    /// Return the candidate's canonical target identifier.
    #[must_use]
    pub fn target_id(&self) -> &str {
        &self.target_id
    }

    /// Return the candidate's disposition.
    #[must_use]
    pub const fn disposition(&self) -> CandidateDisposition {
        self.disposition
    }

    /// Return whether the candidate was eligible for selection.
    #[must_use]
    pub const fn is_eligible(&self) -> bool {
        matches!(self.disposition, CandidateDisposition::Eligible)
    }
}

/// Result of a deterministic selection pass.
///
/// The selected candidate is borrowed from the caller's candidate slice.
/// All candidate decisions are retained, including excluded, incompatible,
/// and indeterminate candidates.
///
/// An absent selected candidate is a valid result: it means no supplied
/// candidate could be selected under the supplied policy and evidence.
#[derive(Debug)]
pub struct SelectionOutcome<'a> {
    selected: Option<&'a TargetSelectionCandidate<'a>>,
    decisions: Vec<CandidateDecision>,
    unresolved_policy_targets: Vec<String>,
}

impl<'a> SelectionOutcome<'a> {
    /// Return the selected candidate, if one was eligible.
    #[must_use]
    pub fn selected(&self) -> Option<&'a TargetSelectionCandidate<'a>> {
        self.selected
    }

    /// Return the selected canonical target identifier, if any.
    #[must_use]
    pub fn selected_target_id(&self) -> Option<&str> {
        self.selected.map(TargetSelectionCandidate::target_id)
    }

    /// Return decisions for every supplied candidate.
    ///
    /// Decisions are returned in canonical target-identifier order, making
    /// reports independent of the candidate input order.
    #[must_use]
    pub fn decisions(&self) -> &[CandidateDecision] {
        &self.decisions
    }

    /// Return preferred/fallback references that did not occur in the
    /// supplied candidate set.
    ///
    /// These references are unresolved for this selection pass. They are not
    /// silently interpreted as supported targets.
    #[must_use]
    pub fn unresolved_policy_targets(&self) -> &[String] {
        &self.unresolved_policy_targets
    }

    /// Return whether a candidate was selected.
    #[must_use]
    pub fn has_selection(&self) -> bool {
        self.selected.is_some()
    }

    /// Return whether any supplied candidate remains unresolved.
    #[must_use]
    pub fn has_indeterminate_candidates(&self) -> bool {
        self.decisions.iter().any(|decision| {
            decision.disposition == CandidateDisposition::Indeterminate
        })
    }
}

/// Stateless service for deterministic target selection.
///
/// All target discovery and compatibility work must be completed before
/// invoking this service.
#[derive(Debug, Clone, Copy, Default)]
pub struct TargetSelectionService;

impl TargetSelectionService {
    /// Construct the stateless selection service.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Select the highest-priority eligible candidate.
    ///
    /// Selection precedence:
    ///
    /// 1. Explicit exclusions are applied.
    /// 2. An explicitly scoped target profile is applied.
    /// 3. Incompatible and indeterminate compatibility reports are rejected
    ///    as selection candidates.
    /// 4. Preferred target order is applied.
    /// 5. Explicit fallback order is applied.
    /// 6. Remaining eligible candidates are ordered by canonical target ID.
    ///
    /// A compatibility report with warnings remains eligible. Its original
    /// warnings remain available through the candidate's report.
    ///
    /// No target is selected if every candidate is excluded, outside profile
    /// scope, incompatible, or indeterminate.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The policy contains invalid or duplicated ordered identifiers.
    /// - The same target appears more than once in the candidate slice.
    /// - A candidate's descriptor or report association is invalid.
    /// - A target-profile reference cannot be validated.
    pub fn select<'a>(
        &self,
        candidates: &'a [TargetSelectionCandidate<'a>],
        policy: &TargetSelectionPolicy,
        profile: Option<&TargetProfileSettings>,
    ) -> ToolchainResult<SelectionOutcome<'a>> {
        policy.validate()?;

        let mut candidate_ids = BTreeSet::new();

        for candidate in candidates {
            // Revalidate at the service boundary. Candidate fields are
            // private, but this protects the boundary if construction
            // invariants are extended in a later API version.
            candidate.target.validate().map_err(|error| {
                ToolchainError::new(
                    ToolchainErrorKind::ValidationFailed,
                    format!(
                        "target `{}` failed selection validation: {error}",
                        candidate.target_id(),
                    ),
                )
                .with_operation(OPERATION_SELECT)
                .with_subject(candidate.target_id().to_owned())
                .with_source(error)
            })?;

            if candidate.compatibility.target_id != candidate.target_id() {
                return Err(
                    ToolchainError::new(
                        ToolchainErrorKind::ValidationFailed,
                        format!(
                            "compatibility report target `{}` does not match \
                             candidate target `{}`",
                            candidate.compatibility.target_id,
                            candidate.target_id(),
                        ),
                    )
                    .with_operation(OPERATION_SELECT)
                    .with_subject(candidate.target_id().to_owned()),
                );
            }

            if !candidate_ids.insert(candidate.target_id().to_owned()) {
                return Err(selection_input_error(format!(
                    "duplicate target candidate `{}`",
                    candidate.target_id(),
                )));
            }
        }

        let mut decisions = Vec::with_capacity(candidates.len());
        let mut eligible_indices = Vec::new();

        // The decision report is canonical and independent of input order.
        let mut decision_order: Vec<usize> =
            (0..candidates.len()).collect();

        decision_order.sort_by(|left, right| {
            candidates[*left]
                .target_id()
                .cmp(candidates[*right].target_id())
        });

        // Decisions are created in canonical ID order. Candidate eligibility
        // is tracked separately by the original candidate index.
        for index in decision_order {
            let candidate = &candidates[index];
            let target_id = candidate.target_id();

            let disposition = if policy.excluded_targets.contains(target_id) {
                CandidateDisposition::ExcludedByPolicy
            } else if let Some(profile) = profile {
                match profile.references_target(target_id)? {
                    Some(false) => CandidateDisposition::OutsideProfileScope,
                    Some(true) | None => {
                        compatibility_disposition(candidate.compatibility)
                    }
                }
            } else {
                compatibility_disposition(candidate.compatibility)
            };

            if disposition == CandidateDisposition::Eligible {
                eligible_indices.push(index);
            }

            decisions.push(CandidateDecision {
                target_id: target_id.to_owned(),
                disposition,
            });
        }

        let preferred_ranks = make_rank_map(&policy.preferred_targets);
        let fallback_ranks = make_rank_map(&policy.fallback_order);

        // Explicit preference order takes precedence over fallback order.
        // Unmentioned candidates use canonical target identity.
        eligible_indices.sort_by(|left, right| {
            let left_id = candidates[*left].target_id();
            let right_id = candidates[*right].target_id();

            let left_rank = selection_rank(
                left_id,
                &preferred_ranks,
                &fallback_ranks,
            );

            let right_rank = selection_rank(
                right_id,
                &preferred_ranks,
                &fallback_ranks,
            );

            left_rank.cmp(&right_rank)
                .then_with(|| left_id.cmp(right_id))
        });

        let selected = eligible_indices
            .first()
            .map(|index| &candidates[*index]);

        let unresolved_policy_targets =
            unresolved_policy_references(policy, &candidate_ids);

        Ok(SelectionOutcome {
            selected,
            decisions,
            unresolved_policy_targets,
        })
    }
}

/// Select a target using the stateless service.
pub fn select_target<'a>(
    candidates: &'a [TargetSelectionCandidate<'a>],
    policy: &TargetSelectionPolicy,
    profile: Option<&TargetProfileSettings>,
) -> ToolchainResult<SelectionOutcome<'a>> {
    TargetSelectionService::new().select(candidates, policy, profile)
}

/// Convert the authoritative compatibility status into a selection
/// disposition.
///
/// This is a conservative mapping. Unknown future statuses must be handled
/// explicitly when the backend compatibility contract evolves.
fn compatibility_disposition(
    report: &CompatibilityReport,
) -> CandidateDisposition {
    match report.status {
        CompatibilityStatus::Compatible
        | CompatibilityStatus::CompatibleWithWarnings => {
            CandidateDisposition::Eligible
        }
        CompatibilityStatus::Incompatible => {
            CandidateDisposition::Incompatible
        }
        CompatibilityStatus::Indeterminate => {
            CandidateDisposition::Indeterminate
        }
    }
}

/// Validate a target identifier using the compiler's canonical target-ID
/// contract.
///
/// The toolchain does not create a second target-ID grammar.
fn validate_target_id(value: &str) -> ToolchainResult<()> {
    CompilerTargetId::new(value.to_owned()).map_err(|error| {
        ToolchainError::new(
            ToolchainErrorKind::InvalidInput,
            format!("invalid target identifier `{value}`: {error}"),
        )
        .with_operation(OPERATION_SELECT)
        .with_subject(value.to_owned())
        .with_source(error)
    })?;

    Ok(())
}

/// Validate and collect an ordered list of target identifiers.
///
/// Duplicate entries are rejected because they create ambiguous policy
/// positions and usually indicate a configuration mistake.
fn validate_ordered_target_ids<I, S>(
    targets: I,
) -> ToolchainResult<Vec<String>>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut values = Vec::new();
    let mut seen = BTreeSet::new();

    for target in targets {
        let target = target.into();
        validate_target_id(&target)?;

        if !seen.insert(target.clone()) {
            return Err(selection_input_error(format!(
                "duplicate target identifier `{target}` in ordered selection policy"
            )));
        }

        values.push(target);
    }

    Ok(values)
}

/// Validate an existing ordered target list.
fn validate_unique_ordered_ids(
    targets: &[String],
    label: &str,
) -> ToolchainResult<()> {
    let mut seen = BTreeSet::new();

    for target in targets {
        validate_target_id(target)?;

        if !seen.insert(target) {
            return Err(selection_input_error(format!(
                "duplicate target identifier `{target}` in {label} target order"
            )));
        }
    }

    Ok(())
}

/// Build a deterministic target-to-position map.
fn make_rank_map(targets: &[String]) -> BTreeMap<&str, usize> {
    targets
        .iter()
        .enumerate()
        .map(|(position, target)| (target.as_str(), position))
        .collect()
}

/// Return the selection rank for one target.
///
/// Preferred targets rank before fallback targets. Unlisted targets are
/// placed after both ordered groups and are then sorted by canonical ID.
fn selection_rank(
    target: &str,
    preferred: &BTreeMap<&str, usize>,
    fallback: &BTreeMap<&str, usize>,
) -> (u8, usize) {
    if let Some(position) = preferred.get(target) {
        (0, *position)
    } else if let Some(position) = fallback.get(target) {
        (1, *position)
    } else {
        (2, 0)
    }
}

/// Find preference and fallback references that were not present in the
/// candidate set for this selection pass.
fn unresolved_policy_references(
    policy: &TargetSelectionPolicy,
    candidate_ids: &BTreeSet<String>,
) -> Vec<String> {
    let mut unresolved = BTreeSet::new();

    for target in policy
        .preferred_targets
        .iter()
        .chain(policy.fallback_order.iter())
    {
        if !candidate_ids.contains(target) {
            unresolved.insert(target.clone());
        }
    }

    unresolved.into_iter().collect()
}

/// Construct a structured selection-input error.
fn selection_input_error(message: impl Into<String>) -> ToolchainError {
    ToolchainError::new(ToolchainErrorKind::InvalidInput, message)
        .with_operation(OPERATION_SELECT)
}

#[cfg(test)]
mod tests {
    use super::{
        validate_ordered_target_ids,
        validate_target_id,
        CandidateDisposition,
        TargetSelectionPolicy,
        TARGET_SELECTION_API_VERSION,
        TARGET_SELECTION_MODULE_NAME,
    };

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            TARGET_SELECTION_MODULE_NAME,
            "zamani.toolchain.targets.selection",
        );
        assert_ne!(TARGET_SELECTION_API_VERSION, 0);
    }

    #[test]
    fn policy_starts_without_implicit_target_selection() {
        let policy = TargetSelectionPolicy::new();

        assert!(policy.preferred_targets().is_empty());
        assert!(policy.fallback_order().is_empty());
        assert!(policy.excluded_targets().is_empty());
    }

    #[test]
    fn open_world_target_identifiers_are_accepted() {
        for target in [
            "x86_64-linux",
            "aarch64",
            "wasm32",
            "quantum:provider:processor",
            "photonic:provider:device",
            "vendor:accelerator:model",
            "future.domain.target",
        ] {
            assert!(validate_target_id(target).is_ok(), "{target}");
        }
    }

    #[test]
    fn invalid_target_identifiers_are_rejected() {
        for target in ["", " target", "target ", "target\nname", "UPPERCASE"] {
            assert!(validate_target_id(target).is_err(), "{target:?}");
        }
    }

    #[test]
    fn ordered_target_ids_preserve_explicit_order() {
        let values = validate_ordered_target_ids([
            "quantum:provider:qpu",
            "classical:portable",
            "hdl:synthesis",
        ])
        .expect("valid target identifiers");

        assert_eq!(
            values,
            [
                "quantum:provider:qpu",
                "classical:portable",
                "hdl:synthesis",
            ],
        );
    }

    #[test]
    fn duplicate_ordered_targets_are_rejected() {
        let result = validate_ordered_target_ids([
            "classical:portable",
            "classical:portable",
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn policy_updates_are_transactional() {
        let mut policy = TargetSelectionPolicy::new();

        policy
            .set_preferred_targets(["classical:portable"])
            .expect("valid preference");

        let result = policy.set_preferred_targets([
            "quantum:provider:qpu",
            "quantum:provider:qpu",
        ]);

        assert!(result.is_err());
        assert_eq!(
            policy.preferred_targets(),
            &["classical:portable".to_owned()],
        );
    }

    #[test]
    fn exclusions_are_deduplicated() {
        let mut policy = TargetSelectionPolicy::new();

        policy
            .set_excluded_targets([
                "classical:portable",
                "classical:portable",
            ])
            .expect("valid exclusions");

        assert_eq!(policy.excluded_targets().len(), 1);
    }

    #[test]
    fn candidate_dispositions_are_explicit() {
        assert_ne!(
            CandidateDisposition::Eligible,
            CandidateDisposition::Indeterminate,
        );
        assert_ne!(
            CandidateDisposition::Incompatible,
            CandidateDisposition::OutsideProfileScope,
        );
    }
}
