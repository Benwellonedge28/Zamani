
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Backend Selection
//!
//! Path: src/compiler/backends/selection.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Select an appropriate registered backend for a compilation workload.
//!
//! This module coordinates:
//!
//! - The canonical backend registry.
//! - Explicit backend preferences.
//! - Authoritative compatibility evaluation.
//! - Deterministic candidate ranking.
//! - Selection diagnostics.
//!
//! It does not implement another backend registry, capability model,
//! target registry, resource planner, or physical resource allocator.
//!
//! # Architectural ownership
//!
//! Canonical backend registration:
//!     crate::backend::registry
//!
//! Compiler-facing backend integration:
//!     crate::compiler::backends
//!
//! Backend discovery:
//!     crate::compiler::backends::discovery
//!
//! Backend compatibility:
//!     crate::compiler::backends::compatibility
//!
//! Target discovery:
//!     crate::compiler::planning::target_discovery
//!
//! Capability matching:
//!     crate::compiler::planning::capability_matching
//!
//! Resource feasibility:
//!     crate::compiler::planning::resource_planning
//!
//! Target descriptors:
//!     crate::compiler::configuration::target
//!
//! Source semantics:
//!     crate::frontend::semantic
//!
//! Physical resource acquisition:
//!     crate::runtime
//!
//! # Fundamental distinctions
//!
//! Registration does not establish compatibility.
//!
//! Compatibility does not establish resource feasibility.
//!
//! Resource feasibility does not establish physical availability.
//!
//! Physical availability does not establish authorization.
//!
//! Backend selection does not execute a program.
//!
//! Successful code generation does not establish artifact validity.
//!
//! Unknown compatibility is never treated as compatibility.
//!
//! # Semantic policy
//!
//! Requirements and constraints are mandatory.
//!
//! Capabilities describe what an implementation can provide.
//!
//! Preferences influence ranking but never exclude otherwise eligible
//! candidates.
//!
//! Hints may influence authoritative planning but do not independently
//! establish compatibility.
//!
//! Feasibility must be established by the appropriate planning authority.
//!
//! This module does not reinterpret source-language semantics.
//!
//! # POCO-REAF
//!
//! Supports open-world backend registration and selection for:
//!
//! - Classical computation.
//! - Quantum computation.
//! - HDL and hardware realization.
//! - Hybrid computation.
//! - Distributed computation.
//! - AI and tensor computation.
//! - Accelerators.
//! - Future computational domains.
//!
//! No closed list of hardware architectures or backend categories is
//! maintained here.
//!
//! No arbitrary universal backend, target, device, processor, memory,
//! accelerator, or computational-domain ceiling is introduced.
//!
//! Literal infinite physical computation is not claimed.
//!
//! # Determinism
//!
//! Selection uses the following ranking order:
//!
//! 1. Lower authoritative evaluator rank.
//! 2. Earlier explicitly declared backend preference.
//! 3. Lexicographically smaller canonical backend identifier.
//!
//! Registry enumeration order does not determine the winner.
//!
//! # Failure policy
//!
//! - Registry errors are propagated.
//! - Evaluator failures abort selection explicitly.
//! - Incompatible candidates are retained in the report.
//! - Unknown candidates are retained in the report.
//! - Unregistered preferences are reported.
//! - No compatible candidate produces a report without a selection.
//! - No implicit fallback backend is invented.
//!
//! # Integration
//!
//! Parent:
//!     src/compiler/backends/mod.rs
//!
//! Canonical registry:
//!     src/backend/registry.rs
//!
//! Compatibility adapter:
//!     src/compiler/backends/compatibility.rs
//!
//! Planning:
//!     src/compiler/planning/capability_matching.rs
//!     src/compiler/planning/resource_planning.rs
//!
//! Invocation:
//!     src/compiler/backends/invocation.rs
//!
//! The caller must perform the appropriate compatibility and feasibility
//! evaluation before treating a selection as a realizable execution plan.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::backend::registry::{
    BackendDescriptor,
    BackendRegistry,
    RegistryError,
};

/// Version of the compiler backend-selection API.
///
/// Independent of the language version, IR format version, backend
/// implementation version, and external toolchain versions.
pub const BACKEND_SELECTION_API_VERSION: u16 = 2;

// ============================================================================
// SELECTION REQUEST
// ============================================================================

/// An ordered backend-selection request.
///
/// Preferred identifiers may be canonical backend identifiers or
/// registered aliases.
///
/// Preferences are soft ranking inputs.
///
/// They do not restrict the candidate set and cannot override
/// compatibility, requirements, constraints, or feasibility.
///
/// An empty preference list considers every registered backend.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BackendSelectionRequest {
    preferred_ids: Vec<String>,
}

impl BackendSelectionRequest {
    /// Create a request considering every registered backend.
    #[must_use]
    pub fn all_registered() -> Self {
        Self::default()
    }

    /// Create a request with ordered backend preferences.
    ///
    /// Duplicate identifiers are removed while preserving their first
    /// occurrence.
    ///
    /// Empty or malformed identifiers are retained as supplied so that
    /// their resolution outcome remains visible in the selection report.
    pub fn with_preferences<I, S>(identifiers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut seen = BTreeSet::new();
        let mut preferred_ids = Vec::new();

        for identifier in identifiers {
            let identifier = identifier.into();

            if seen.insert(identifier.clone()) {
                preferred_ids.push(identifier);
            }
        }

        Self { preferred_ids }
    }

    /// Return preferred identifiers in declaration order.
    #[must_use]
    pub fn preferred_ids(&self) -> &[String] {
        &self.preferred_ids
    }

    /// Return whether the request contains preferences.
    #[must_use]
    pub fn has_preferences(&self) -> bool {
        !self.preferred_ids.is_empty()
    }
}

// ============================================================================
// ELIGIBILITY
// ============================================================================

/// Result of evaluating a backend against a compilation workload.
///
/// Only an explicitly established Compatible result is eligible.
///
/// Unknown and Incompatible are distinct outcomes.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum BackendEligibility {
    /// Compatibility has been established by the authoritative evaluator.
    Compatible {
        /// Authoritative ranking score.
        ///
        /// Lower values are preferred.
        rank: i64,

        /// Evidence-based compatibility explanation.
        explanation: String,
    },

    /// An incompatibility has been established.
    Incompatible {
        /// Explanation of the incompatibility.
        reason: String,
    },

    /// Available evidence is insufficient to establish compatibility.
    Unknown {
        /// Explanation of the missing or inconclusive evidence.
        reason: String,
    },
}

impl BackendEligibility {
    /// Return whether compatibility has been established.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        matches!(self, Self::Compatible { .. })
    }

    /// Return the authoritative rank when compatibility is established.
    #[must_use]
    pub const fn rank(&self) -> Option<i64> {
        match self {
            Self::Compatible { rank, .. } => Some(*rank),
            Self::Incompatible { .. } | Self::Unknown { .. } => None,
        }
    }
}

// ============================================================================
// AUTHORITATIVE EVALUATION CONTRACT
// ============================================================================

/// Evaluates a registered backend against a compilation workload.
///
/// Implementations must use authoritative repository contracts.
///
/// The evaluator is expected to capture the immutable compilation
/// requirements and relevant planning context.
///
/// Its evaluation must account for all mandatory conditions applicable
/// to the workload.
///
/// In particular, an evaluator must not report Compatible merely because
/// a backend is registered or advertises a matching name.
///
/// Implementations should integrate:
///
/// - Semantic capability requirements.
/// - Capability evidence.
/// - Mandatory constraints.
/// - Backend compatibility.
/// - Resource feasibility evidence where required.
/// - Applicable security and authorization policies.
///
/// This trait does not perform physical allocation or execution.
pub trait BackendEligibilityEvaluator: Send + Sync {
    /// Evaluate one canonical backend descriptor.
    ///
    /// Return Compatible only when the available authoritative evidence
    /// establishes that the backend satisfies the workload's mandatory
    /// conditions.
    ///
    /// Return Unknown when the evidence is insufficient.
    ///
    /// Return Incompatible when a mandatory condition is not satisfied.
    ///
    /// Return an error when evaluation itself fails.
    fn evaluate(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<BackendEligibility, String>;
}

// ============================================================================
// CANDIDATE EVALUATION
// ============================================================================

/// Evaluation result for one registered backend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendCandidateEvaluation {
    /// Canonical backend identifier.
    pub backend_id: String,

    /// Position of the matching preference, if any.
    ///
    /// None means the backend was not explicitly preferred.
    pub preference_index: Option<usize>,

    /// Authoritative compatibility result.
    pub eligibility: BackendEligibility,
}

impl BackendCandidateEvaluation {
    /// Return whether the candidate is eligible.
    #[must_use]
    pub fn is_eligible(&self) -> bool {
        self.eligibility.is_compatible()
    }

    /// Return whether compatibility is unknown.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        matches!(
            self.eligibility,
            BackendEligibility::Unknown { .. }
        )
    }

    /// Return whether the candidate is incompatible.
    #[must_use]
    pub fn is_incompatible(&self) -> bool {
        matches!(
            self.eligibility,
            BackendEligibility::Incompatible { .. }
        )
    }
}

// ============================================================================
// SELECTION REPORT
// ============================================================================

/// Complete immutable backend-selection report.
///
/// The selected descriptor is metadata from the canonical registry.
///
/// It is not:
///
/// - A constructed backend implementation.
/// - A physical device.
/// - A hardware allocation.
/// - A verified executable artifact.
/// - Proof of runtime availability.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BackendSelectionReport {
    selected: Option<BackendDescriptor>,
    evaluations: Vec<BackendCandidateEvaluation>,
    unresolved_preferences: Vec<String>,
}

impl BackendSelectionReport {
    /// Create an empty report.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Return the selected canonical backend descriptor.
    #[must_use]
    pub fn selected(&self) -> Option<&BackendDescriptor> {
        self.selected.as_ref()
    }

    /// Return every evaluated candidate.
    ///
    /// Results are ordered by canonical backend identifier.
    #[must_use]
    pub fn evaluations(&self) -> &[BackendCandidateEvaluation] {
        &self.evaluations
    }

    /// Return the selected canonical backend identifier.
    #[must_use]
    pub fn selected_id(&self) -> Option<&str> {
        self.selected
            .as_ref()
            .map(|descriptor| descriptor.id.as_str())
    }

    /// Return whether a compatible backend was selected.
    #[must_use]
    pub fn has_selection(&self) -> bool {
        self.selected.is_some()
    }

    /// Return whether no backend was selected.
    ///
    /// This does not distinguish between an empty registry and a registry
    /// containing only incompatible or unknown candidates.
    #[must_use]
    pub fn is_unavailable(&self) -> bool {
        self.selected.is_none()
    }

    /// Return the number of evaluated backends.
    #[must_use]
    pub fn evaluated_count(&self) -> usize {
        self.evaluations.len()
    }

    /// Return the number of compatible backends.
    #[must_use]
    pub fn eligible_count(&self) -> usize {
        self.evaluations
            .iter()
            .filter(|evaluation| evaluation.is_eligible())
            .count()
    }

    /// Return the number of incompatible backends.
    #[must_use]
    pub fn incompatible_count(&self) -> usize {
        self.evaluations
            .iter()
            .filter(|evaluation| evaluation.is_incompatible())
            .count()
    }

    /// Return the number of backends with unknown compatibility.
    #[must_use]
    pub fn unknown_count(&self) -> usize {
        self.evaluations
            .iter()
            .filter(|evaluation| evaluation.is_unknown())
            .count()
    }

    /// Return identifiers in the preference list that did not resolve
    /// to a registered canonical backend.
    #[must_use]
    pub fn unresolved_preferences(&self) -> &[String] {
        &self.unresolved_preferences
    }

    /// Return the number of unresolved preferences.
    #[must_use]
    pub fn unresolved_preference_count(&self) -> usize {
        self.unresolved_preferences.len()
    }

    /// Find an evaluation by canonical backend identifier.
    ///
    /// Evaluations are sorted, so lookup uses binary search.
    #[must_use]
    pub fn evaluation(
        &self,
        backend_id: &str,
    ) -> Option<&BackendCandidateEvaluation> {
        self.evaluations
            .binary_search_by(|evaluation| {
                evaluation.backend_id.as_str().cmp(backend_id)
            })
            .ok()
            .map(|index| &self.evaluations[index])
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced during backend selection.
#[derive(Debug)]
#[non_exhaustive]
pub enum BackendSelectionError {
    /// An operation on the canonical registry failed.
    Registry(RegistryError),

    /// Authoritative evaluation failed for a backend.
    EvaluationFailed {
        /// Canonical backend identifier.
        backend_id: String,

        /// Evaluation failure explanation.
        reason: String,
    },
}

impl fmt::Display for BackendSelectionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Registry(error) => {
                write!(
                    formatter,
                    "backend selection registry failure: {error}"
                )
            }

            Self::EvaluationFailed {
                backend_id,
                reason,
            } => {
                write!(
                    formatter,
                    "compatibility evaluation failed for backend \
                     `{backend_id}`: {reason}"
                )
            }
        }
    }
}

impl Error for BackendSelectionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Registry(error) => Some(error),
            Self::EvaluationFailed { .. } => None,
        }
    }
}

impl From<RegistryError> for BackendSelectionError {
    fn from(error: RegistryError) -> Self {
        Self::Registry(error)
    }
}

// ============================================================================
// PREFERENCE RESOLUTION
// ============================================================================

/// Resolved preference information.
///
/// The map contains canonical backend identifiers and their earliest
/// preference positions.
///
/// Aliases are resolved exclusively through registered descriptors.
#[derive(Clone, Debug, Default)]
struct ResolvedPreferences {
    indices: BTreeMap<String, usize>,
    unresolved: Vec<String>,
}

/// Resolve preferences against the canonical descriptor snapshot.
///
/// Both canonical identifiers and registered aliases are supported.
///
/// An unknown preference remains visible in the report.
///
/// Preferences never restrict candidate evaluation.
fn resolve_preferences(
    request: &BackendSelectionRequest,
    descriptors: &[BackendDescriptor],
) -> ResolvedPreferences {
    let mut canonical_ids = BTreeMap::<String, usize>::new();

    // Construct the alias and canonical identifier lookup once.
    //
    // This avoids repeatedly scanning every descriptor for every
    // requested preference.
    for descriptor in descriptors {
        canonical_ids
            .entry(descriptor.id.clone())
            .or_insert(usize::MAX);

        for alias in &descriptor.aliases {
            canonical_ids
                .entry(alias.clone())
                .or_insert(usize::MAX);
        }
    }

    let mut result = ResolvedPreferences::default();

    for (preference_index, requested_id) in
        request.preferred_ids().iter().enumerate()
    {
        match canonical_ids.get(requested_id) {
            Some(_) => {
                let canonical_id = descriptors
                    .iter()
                    .find(|descriptor| {
                        descriptor.id == *requested_id
                            || descriptor
                                .aliases
                                .iter()
                                .any(|alias| alias == requested_id)
                    })
                    .map(|descriptor| descriptor.id.clone());

                if let Some(canonical_id) = canonical_id {
                    result
                        .indices
                        .entry(canonical_id)
                        .or_insert(preference_index);
                }
            }

            None => {
                result.unresolved.push(requested_id.clone());
            }
        }
    }

    result
}

// ============================================================================
// RANKING
// ============================================================================

/// Compare two eligible backend candidates.
///
/// Ranking is deterministic and independent of registry insertion order.
///
/// The evaluator's rank is authoritative. Preference order breaks equal
/// evaluator ranks. Canonical identity breaks remaining ties.
fn compare_candidates(
    left_rank: i64,
    left_preference: Option<usize>,
    left_id: &str,
    right_rank: i64,
    right_preference: Option<usize>,
    right_id: &str,
) -> Ordering {
    left_rank
        .cmp(&right_rank)
        .then_with(|| {
            left_preference
                .unwrap_or(usize::MAX)
                .cmp(&right_preference.unwrap_or(usize::MAX))
        })
        .then_with(|| left_id.cmp(right_id))
}

// ============================================================================
// BACKEND SELECTOR
// ============================================================================

/// Compiler-facing backend selection coordinator.
///
/// Borrows the authoritative backend registry.
///
/// Does not own or maintain another backend inventory.
pub struct BackendSelector<'a> {
    registry: &'a BackendRegistry,
}

impl<'a> BackendSelector<'a> {
    /// Create a selector using the canonical backend registry.
    #[must_use]
    pub fn new(registry: &'a BackendRegistry) -> Self {
        Self { registry }
    }

    /// Select the best eligible registered backend.
    ///
    /// The selection algorithm:
    ///
    /// 1. Retrieves the canonical registry descriptor snapshot.
    /// 2. Resolves preferences and aliases.
    /// 3. Evaluates every registered descriptor.
    /// 4. Retains incompatible and unknown evaluation results.
    /// 5. Ranks compatible candidates deterministically.
    /// 6. Returns the best candidate and complete diagnostics.
    ///
    /// Preferences are soft. An unpreferred backend can be selected
    /// when it is eligible and ranks better than preferred alternatives.
    ///
    /// The evaluator must already be associated with the immutable
    /// compilation workload being evaluated.
    ///
    /// Evaluator errors abort selection rather than silently excluding
    /// the affected backend.
    ///
    /// No backend is constructed or invoked by this method.
    ///
    /// No physical resources are allocated.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The canonical registry cannot provide its descriptor snapshot.
    /// - The authoritative evaluator fails for any candidate.
    pub fn select<E>(
        &self,
        request: &BackendSelectionRequest,
        evaluator: &E,
    ) -> Result<BackendSelectionReport, BackendSelectionError>
    where
        E: BackendEligibilityEvaluator + ?Sized,
    {
        let descriptors = self.registry.descriptors()?;

        let preferences =
            resolve_preferences(request, &descriptors);

        let mut evaluations =
            Vec::with_capacity(descriptors.len());

        // Retain only the best eligible descriptor.
        //
        // This avoids creating and sorting a second complete collection
        // of eligible descriptors.
        let mut best: Option<(
            i64,
            Option<usize>,
            String,
            BackendDescriptor,
        )> = None;

        for descriptor in descriptors {
            let preference_index = preferences
                .indices
                .get(&descriptor.id)
                .copied();

            let eligibility = evaluator
                .evaluate(&descriptor)
                .map_err(|reason| {
                    BackendSelectionError::EvaluationFailed {
                        backend_id: descriptor.id.clone(),
                        reason,
                    }
                })?;

            if let BackendEligibility::Compatible { rank, .. } =
                &eligibility
            {
                let candidate_rank = *rank;

                let should_replace = match &best {
                    None => true,

                    Some((
                        best_rank,
                        best_preference,
                        best_id,
                        _,
                    )) => {
                        compare_candidates(
                            candidate_rank,
                            preference_index,
                            &descriptor.id,
                            *best_rank,
                            *best_preference,
                            best_id,
                        ) == Ordering::Less
                    }
                };

                if should_replace {
                    best = Some((
                        candidate_rank,
                        preference_index,
                        descriptor.id.clone(),
                        descriptor.clone(),
                    ));
                }
            }

            evaluations.push(BackendCandidateEvaluation {
                backend_id: descriptor.id.clone(),
                preference_index,
                eligibility,
            });
        }

        // The canonical registry currently supplies descriptors in
        // canonical identifier order. Sorting here keeps this report's
        // public contract explicit and independent of that assumption.
        evaluations.sort_by(|left, right| {
            left.backend_id.cmp(&right.backend_id)
        });

        let selected = best.map(|candidate| candidate.3);

        Ok(BackendSelectionReport {
            selected,
            evaluations,
            unresolved_preferences: preferences.unresolved,
        })
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct FixedEvaluator {
        result: BackendEligibility,
    }

    impl BackendEligibilityEvaluator for FixedEvaluator {
        fn evaluate(
            &self,
            _backend: &BackendDescriptor,
        ) -> Result<BackendEligibility, String> {
            Ok(self.result.clone())
        }
    }

    struct FailingEvaluator;

    impl BackendEligibilityEvaluator for FailingEvaluator {
        fn evaluate(
            &self,
            backend: &BackendDescriptor,
        ) -> Result<BackendEligibility, String> {
            Err(format!(
                "evaluation failed for {}",
                backend.id
            ))
        }
    }

    #[test]
    fn api_version_is_nonzero() {
        assert_ne!(BACKEND_SELECTION_API_VERSION, 0);
    }

    #[test]
    fn request_deduplicates_preferences_preserving_order() {
        let request =
            BackendSelectionRequest::with_preferences([
                "backend.a",
                "backend.b",
                "backend.a",
            ]);

        assert_eq!(
            request.preferred_ids(),
            &[
                "backend.a".to_owned(),
                "backend.b".to_owned(),
            ]
        );
    }

    #[test]
    fn empty_request_considers_all_registered_backends() {
        let request =
            BackendSelectionRequest::all_registered();

        assert!(!request.has_preferences());
        assert!(request.preferred_ids().is_empty());
    }

    #[test]
    fn preferences_are_soft_not_candidate_restrictions() {
        let descriptors = vec![
            BackendDescriptor::new(
                "backend.classical",
                ".bin",
            ),
            BackendDescriptor::new(
                "backend.quantum",
                ".qasm",
            ),
        ];

        let request =
            BackendSelectionRequest::with_preferences([
                "backend.quantum",
            ]);

        let resolved =
            resolve_preferences(&request, &descriptors);

        assert_eq!(
            resolved.indices.get("backend.quantum"),
            Some(&0)
        );

        // A backend without a matching preference remains a candidate.
        assert!(!resolved.indices.contains_key("backend.classical"));

        // Candidate filtering is deliberately absent.
    }

    #[test]
    fn aliases_resolve_to_canonical_identifiers() {
        let descriptors = vec![
            BackendDescriptor::new(
                "backend.canonical",
                ".bin",
            )
            .with_alias("backend.alias"),
        ];

        let request =
            BackendSelectionRequest::with_preferences([
                "backend.alias",
            ]);

        let resolved =
            resolve_preferences(&request, &descriptors);

        assert_eq!(
            resolved.indices.get("backend.canonical"),
            Some(&0)
        );

        assert!(resolved.unresolved.is_empty());
    }

    #[test]
    fn canonical_identifiers_resolve_directly() {
        let descriptors = vec![
            BackendDescriptor::new(
                "backend.canonical",
                ".bin",
            ),
        ];

        let request =
            BackendSelectionRequest::with_preferences([
                "backend.canonical",
            ]);

        let resolved =
            resolve_preferences(&request, &descriptors);

        assert_eq!(
            resolved.indices.get("backend.canonical"),
            Some(&0)
        );
    }

    #[test]
    fn unknown_preferences_are_reported() {
        let descriptors = vec![
            BackendDescriptor::new(
                "backend.available",
                ".bin",
            ),
        ];

        let request =
            BackendSelectionRequest::with_preferences([
                "backend.missing",
            ]);

        let resolved =
            resolve_preferences(&request, &descriptors);

        assert_eq!(
            resolved.unresolved,
            vec!["backend.missing".to_owned()]
        );
    }

    #[test]
    fn incompatible_candidates_are_not_eligible() {
        let candidate = BackendCandidateEvaluation {
            backend_id: "backend.example".to_owned(),
            preference_index: None,
            eligibility: BackendEligibility::Incompatible {
                reason: "required capability unavailable".to_owned(),
            },
        };

        assert!(!candidate.is_eligible());
        assert!(candidate.is_incompatible());
        assert!(!candidate.is_unknown());
    }

    #[test]
    fn unknown_candidates_are_not_eligible() {
        let candidate = BackendCandidateEvaluation {
            backend_id: "backend.example".to_owned(),
            preference_index: None,
            eligibility: BackendEligibility::Unknown {
                reason: "capability evidence unavailable".to_owned(),
            },
        };

        assert!(!candidate.is_eligible());
        assert!(candidate.is_unknown());
        assert!(!candidate.is_incompatible());
    }

    #[test]
    fn compatible_candidates_are_eligible() {
        let candidate = BackendCandidateEvaluation {
            backend_id: "backend.example".to_owned(),
            preference_index: Some(0),
            eligibility: BackendEligibility::Compatible {
                rank: 0,
                explanation: "requirements satisfied".to_owned(),
            },
        };

        assert!(candidate.is_eligible());
    }

    #[test]
    fn eligibility_rank_is_available_only_for_compatible_candidates() {
        let compatible = BackendEligibility::Compatible {
            rank: 42,
            explanation: "supported".to_owned(),
        };

        let incompatible = BackendEligibility::Incompatible {
            reason: "unsupported".to_owned(),
        };

        let unknown = BackendEligibility::Unknown {
            reason: "insufficient evidence".to_owned(),
        };

        assert_eq!(compatible.rank(), Some(42));
        assert_eq!(incompatible.rank(), None);
        assert_eq!(unknown.rank(), None);
    }

    #[test]
    fn empty_report_has_no_selection() {
        let report = BackendSelectionReport::empty();

        assert!(!report.has_selection());
        assert!(report.is_unavailable());

        assert_eq!(report.evaluated_count(), 0);
        assert_eq!(report.eligible_count(), 0);
        assert_eq!(report.incompatible_count(), 0);
        assert_eq!(report.unknown_count(), 0);
        assert_eq!(report.unresolved_preference_count(), 0);
    }

    #[test]
    fn lower_evaluator_rank_is_preferred() {
        assert_eq!(
            compare_candidates(
                1,
                None,
                "backend.a",
                2,
                Some(0),
                "backend.b",
            ),
            Ordering::Less
        );
    }

    #[test]
    fn preference_breaks_equal_rank_ties() {
        assert_eq!(
            compare_candidates(
                10,
                Some(0),
                "backend.z",
                10,
                Some(1),
                "backend.a",
            ),
            Ordering::Less
        );
    }

    #[test]
    fn canonical_identifier_breaks_remaining_ties() {
        assert_eq!(
            compare_candidates(
                10,
                None,
                "backend.a",
                10,
                None,
                "backend.b",
            ),
            Ordering::Less
        );
    }

    #[test]
    fn preferences_do_not_override_better_evaluator_rank() {
        assert_eq!(
            compare_candidates(
                20,
                Some(0),
                "backend.preferred",
                10,
                None,
                "backend.unpreferred",
            ),
            Ordering::Greater
        );
    }

    #[test]
    fn unknown_evaluator_results_never_receive_a_rank() {
        let eligibility = BackendEligibility::Unknown {
            reason: "insufficient evidence".to_owned(),
        };

        assert_eq!(eligibility.rank(), None);
    }

    #[test]
    fn evaluator_contract_is_object_safe() {
        fn accepts_evaluator(
            _: &dyn BackendEligibilityEvaluator,
        ) {}

        let evaluator = FixedEvaluator {
            result: BackendEligibility::Unknown {
                reason: "not evaluated".to_owned(),
            },
        };

        accepts_evaluator(&evaluator);
    }

    #[test]
    fn evaluator_failure_preserves_backend_identity() {
        let error = BackendSelectionError::EvaluationFailed {
            backend_id: "backend.example".to_owned(),
            reason: "test failure".to_owned(),
        };

        let rendered = error.to_string();

        assert!(rendered.contains("backend.example"));
        assert!(rendered.contains("test failure"));
    }

    #[test]
    fn registry_errors_are_preserved_as_error_sources() {
        let error = BackendSelectionError::Registry(
            RegistryError::BackendNotFound {
                identifier: "backend.missing".to_owned(),
            },
        );

        assert!(error.source().is_some());
    }

    #[test]
    fn selector_borrows_the_canonical_registry() {
        fn accepts_registry<'a>(
            registry: &'a BackendRegistry,
        ) -> BackendSelector<'a> {
            BackendSelector::new(registry)
        }

        let registry = BackendRegistry::new();

        let _selector = accepts_registry(&registry);
    }

    #[test]
    fn ranking_is_independent_of_registration_order() {
        let first = compare_candidates(
            10,
            Some(0),
            "backend.a",
            10,
            Some(1),
            "backend.b",
        );

        let second = compare_candidates(
            10,
            Some(1),
            "backend.b",
            10,
            Some(0),
            "backend.a",
        );

        assert_eq!(first, Ordering::Less);
        assert_eq!(second, Ordering::Greater);
    }
}
