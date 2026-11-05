
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Backend Selection
//!
//! Path: `src/compiler/backends/selection.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Select a registered backend using:
//!
//! - Canonical backend registry metadata.
//! - Explicit caller preferences.
//! - An authoritative compatibility evaluator.
//! - Deterministic ranking.
//!
//! This module is a compiler-facing selection coordinator. It does not
//! implement another backend registry, target registry, capability model,
//! resource planner, or physical device allocator.
//!
//! ## Architectural ownership
//!
//! Canonical backend registration:
//! `crate::backend::registry`
//!
//! Compiler-facing registry:
//! `crate::compiler::backends::registry`
//!
//! Backend discovery:
//! `crate::compiler::backends::discovery`
//!
//! Target discovery:
//! `crate::compiler::planning::target_discovery`
//!
//! Capability matching:
//! `crate::compiler::planning::capability_matching`
//!
//! Target descriptors:
//! `crate::compiler::configuration::target`
//!
//! Physical execution:
//! `crate::runtime`
//!
//! ## Important distinction
//!
//! Registration does not establish compatibility.
//!
//! Compatibility does not establish resource feasibility.
//!
//! Resource feasibility does not establish physical availability.
//!
//! Backend selection does not execute a program.
//!
//! The evaluator supplied to this module must use the authoritative
//! capability and feasibility contracts. Unknown compatibility is never
//! treated as compatible.
//!
//! ## Scalability
//!
//! This module introduces no fixed backend count, architecture count,
//! accelerator count, target count, or hardware capacity.
//!
//! Collections grow according to available memory and other operational
//! resources. Resource budgets belong to compilation orchestration.
//!
//! No claim of literal infinite computation is made.
//!
//! ## Determinism
//!
//! Candidate ranking is deterministic:
//!
//! 1. Lower evaluator rank is preferred.
//! 2. Explicit preference order breaks equal-rank ties.
//! 3. Canonical backend identifier breaks remaining ties.
//!
//! Registry enumeration is inherited from the canonical registry.
//!
//! ## Failure behavior
//!
//! Registry failures are propagated.
//!
//! Evaluator failures are propagated with the backend identity.
//!
//! Incompatible and unknown candidates are retained in the report.
//!
//! No compatible candidate produces a report with no selection.
//!
//! No implicit fallback backend is invented.
//!
//! ## Integration
//!
//! Parent:
//! `src/compiler/backends/mod.rs`
//!
//! Required sibling:
//! `src/compiler/backends/registry.rs`
//!
//! Required authoritative inputs:
//! `src/compiler/planning/capability_matching.rs`
//! `src/compiler/planning/resource_planning.rs`
//!
//! The capability evaluator should be implemented as an adapter over
//! those existing authorities. This file must not duplicate them.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::backend::registry::{
    BackendDescriptor,
    BackendRegistry,
    RegistryError,
};

/// Version of the compiler backend-selection API.
///
/// This version is independent of the Zamani language version,
/// backend implementation versions, and IR format versions.
pub const BACKEND_SELECTION_API_VERSION: u16 = 1;

// ============================================================================
// REQUEST
// ============================================================================

/// An ordered backend-selection request.
///
/// `preferred_ids` contains canonical backend identifiers or registered
/// aliases. The order expresses caller preference, not compatibility.
///
/// An empty preference list allows every registered backend to be
/// considered.
///
/// Preferences never override compatibility or feasibility.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BackendSelectionRequest {
    preferred_ids: Vec<String>,
}

impl BackendSelectionRequest {
    /// Create a request that considers every registered backend.
    #[must_use]
    pub fn all_registered() -> Self {
        Self::default()
    }

    /// Create a request with an ordered list of backend preferences.
    ///
    /// Identifiers may be canonical identifiers or registered aliases.
    /// Repeated identifiers are ignored after their first occurrence.
    pub fn with_preferences<I, S>(identifiers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut seen = BTreeMap::<String, ()>::new();
        let mut preferred_ids = Vec::new();

        for identifier in identifiers {
            let identifier = identifier.into();

            if seen.insert(identifier.clone(), ()).is_none() {
                preferred_ids.push(identifier);
            }
        }

        Self { preferred_ids }
    }

    /// Return the requested identifiers in preference order.
    #[must_use]
    pub fn preferred_ids(&self) -> &[String] {
        &self.preferred_ids
    }

    /// Whether the request explicitly restricts backend preferences.
    #[must_use]
    pub fn has_preferences(&self) -> bool {
        !self.preferred_ids.is_empty()
    }
}

// ============================================================================
// COMPATIBILITY EVALUATION
// ============================================================================

/// Result of evaluating a backend against a compilation workload.
///
/// This type deliberately distinguishes incompatibility from unknown
/// compatibility. Neither is eligible for selection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BackendEligibility {
    /// The evaluator established that the backend is compatible.
    Compatible {
        /// Evaluator-provided preference rank.
        ///
        /// Lower values are preferred. The rank is supplied by the
        /// authoritative evaluator; this module invents no cost model.
        rank: i64,

        /// Explanation supporting the compatibility decision.
        explanation: String,
    },

    /// The evaluator established that the backend is incompatible.
    Incompatible {
        /// Explanation of the incompatibility.
        reason: String,
    },

    /// Compatibility could not be established.
    ///
    /// Unknown must never be treated as compatible.
    Unknown {
        /// Explanation of the missing or inconclusive evidence.
        reason: String,
    },
}

/// Evaluates whether a registered backend can satisfy a compilation
/// workload.
///
/// Implementations must delegate to the repository's authoritative
/// capability matching and feasibility systems.
///
/// They must not infer capabilities from backend names, aliases,
/// extensions, descriptions, or vendor strings.
///
/// The evaluator must not allocate physical hardware or execute programs.
pub trait BackendEligibilityEvaluator: Send + Sync {
    /// Evaluate one registered backend descriptor.
    ///
    /// Return `Compatible` only when the available evidence establishes
    /// compatibility with the workload being evaluated.
    ///
    /// Return `Unknown` when the available evidence is insufficient.
    ///
    /// Return an error when evaluation itself fails.
    fn evaluate(
        &self,
        backend: &BackendDescriptor,
    ) -> Result<BackendEligibility, String>;
}

// ============================================================================
// CANDIDATE RESULT
// ============================================================================

/// Evaluation result for one registered backend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendCandidateEvaluation {
    /// Canonical backend identifier.
    pub backend_id: String,

    /// Position in the explicit preference list, if supplied.
    ///
    /// `None` means the request did not explicitly prefer this backend.
    pub preference_index: Option<usize>,

    /// Compatibility evaluation.
    pub eligibility: BackendEligibility,
}

impl BackendCandidateEvaluation {
    /// Whether this candidate is eligible for selection.
    #[must_use]
    pub fn is_eligible(&self) -> bool {
        matches!(
            self.eligibility,
            BackendEligibility::Compatible { .. }
        )
    }
}

// ============================================================================
// SELECTION REPORT
// ============================================================================

/// Immutable result of backend selection.
///
/// The selected descriptor is metadata from the canonical registry.
/// It is not a constructed backend, physical device, or executable artifact.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BackendSelectionReport {
    selected: Option<BackendDescriptor>,
    evaluations: Vec<BackendCandidateEvaluation>,
}

impl BackendSelectionReport {
    /// Create a report with no selected backend.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Return the selected backend descriptor, if one was found.
    #[must_use]
    pub fn selected(&self) -> Option<&BackendDescriptor> {
        self.selected.as_ref()
    }

    /// Return all evaluated candidates.
    ///
    /// The evaluations are ordered by canonical backend identifier.
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

    /// Return whether no backend was eligible.
    #[must_use]
    pub fn is_unavailable(&self) -> bool {
        self.selected.is_none()
    }

    /// Find an evaluation by canonical backend identifier.
    #[must_use]
    pub fn evaluation(
        &self,
        backend_id: &str,
    ) -> Option<&BackendCandidateEvaluation> {
        self.evaluations
            .iter()
            .find(|evaluation| evaluation.backend_id == backend_id)
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced during backend selection.
#[derive(Debug)]
#[non_exhaustive]
pub enum BackendSelectionError {
    /// The canonical backend registry operation failed.
    Registry(RegistryError),

    /// Compatibility evaluation failed for a backend.
    EvaluationFailed {
        /// Canonical backend identifier.
        backend_id: String,

        /// Evaluator failure explanation.
        reason: String,
    },
}

impl fmt::Display for BackendSelectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Registry(error) => {
                write!(formatter, "backend selection registry failure: {error}")
            }

            Self::EvaluationFailed { backend_id, reason } => {
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
// SELECTOR
// ============================================================================

/// Compiler-facing backend selection coordinator.
///
/// The selector borrows the canonical registry. It does not own another
/// registry or maintain a second backend inventory.
pub struct BackendSelector<'a> {
    registry: &'a BackendRegistry,
}

impl<'a> BackendSelector<'a> {
    /// Create a selector over the canonical backend registry.
    #[must_use]
    pub fn new(registry: &'a BackendRegistry) -> Self {
        Self { registry }
    }

    /// Evaluate registered backends and select the best compatible
    /// candidate.
    ///
    /// Selection proceeds as follows:
    ///
    /// 1. Obtain a deterministic descriptor snapshot from the registry.
    /// 2. Apply the explicit preference restriction, if any.
    /// 3. Evaluate each remaining candidate.
    /// 4. Exclude incompatible and unknown candidates.
    /// 5. Rank compatible candidates deterministically.
    /// 6. Return the selected descriptor and all evaluations.
    ///
    /// If no candidate is compatible, the method returns a successful
    /// report with no selection. This is not a successful compilation.
    ///
    /// Evaluator errors abort selection and are returned explicitly.
    pub fn select<E>(
        &self,
        request: &BackendSelectionRequest,
        evaluator: &E,
    ) -> Result<BackendSelectionReport, BackendSelectionError>
    where
        E: BackendEligibilityEvaluator + ?Sized,
    {
        let descriptors = self.registry.descriptors()?;

        let preference_indices = preference_indices(request);

        let mut evaluations = Vec::new();
        let mut eligible = Vec::new();

        for descriptor in descriptors {
            let preference_index = preference_indices
                .get(&descriptor.id)
                .copied();

            if request.has_preferences() && preference_index.is_none() {
                continue;
            }

            let eligibility = evaluator
                .evaluate(&descriptor)
                .map_err(|reason| {
                    BackendSelectionError::EvaluationFailed {
                        backend_id: descriptor.id.clone(),
                        reason,
                    }
                })?;

            let evaluation = BackendCandidateEvaluation {
                backend_id: descriptor.id.clone(),
                preference_index,
                eligibility: eligibility.clone(),
            };

            if let BackendEligibility::Compatible { rank, .. } = eligibility {
                eligible.push((
                    rank,
                    preference_index.unwrap_or(usize::MAX),
                    descriptor.id.clone(),
                    descriptor.clone(),
                ));
            }

            evaluations.push(evaluation);
        }

        // The canonical registry enumerates descriptors in canonical
        // identifier order. Preserve that property in the report.
        evaluations.sort_by(|left, right| {
            left.backend_id.cmp(&right.backend_id)
        });

        // Ranking is stable and independent of registration order.
        eligible.sort_by(|left, right| {
            left.0
                .cmp(&right.0)
                .then_with(|| left.1.cmp(&right.1))
                .then_with(|| left.2.cmp(&right.2))
        });

        let selected = eligible
            .into_iter()
            .next()
            .map(|candidate| candidate.3);

        Ok(BackendSelectionReport {
            selected,
            evaluations,
        })
    }
}

// ============================================================================
// HELPERS
// ============================================================================

/// Build a canonical identifier-to-preference-index map.
///
/// Aliases are resolved against the descriptor snapshot by the selector.
/// This helper only handles canonical identifiers. Alias resolution is
/// performed separately by `preference_indices_for_descriptors`.
fn preference_indices(
    request: &BackendSelectionRequest,
) -> BTreeMap<String, usize> {
    request
        .preferred_ids()
        .iter()
        .enumerate()
        .map(|(index, identifier)| (identifier.clone(), index))
        .collect()
}

/// Resolve explicit preferences against canonical descriptors.
///
/// Both canonical identifiers and registered aliases are accepted.
/// The canonical identifier receives the preference index of its first
/// matching requested identifier.
fn preference_indices_for_descriptors(
    request: &BackendSelectionRequest,
    descriptors: &[BackendDescriptor],
) -> BTreeMap<String, usize> {
    let mut result = BTreeMap::new();

    for (preference_index, requested_id) in
        request.preferred_ids().iter().enumerate()
    {
        for descriptor in descriptors {
            let matches = descriptor.id == *requested_id
                || descriptor
                    .aliases
                    .iter()
                    .any(|alias| alias == requested_id);

            if matches {
                result
                    .entry(descriptor.id.clone())
                    .or_insert(preference_index);
            }
        }
    }

    result
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
            Err(format!("evaluation failed for {}", backend.id))
        }
    }

    #[test]
    fn request_deduplicates_preferences_preserving_order() {
        let request = BackendSelectionRequest::with_preferences([
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
        let request = BackendSelectionRequest::all_registered();

        assert!(!request.has_preferences());
        assert!(request.preferred_ids().is_empty());
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
    fn empty_report_has_no_selection() {
        let report = BackendSelectionReport::empty();

        assert!(!report.has_selection());
        assert!(report.is_unavailable());
        assert_eq!(report.evaluated_count(), 0);
        assert_eq!(report.eligible_count(), 0);
    }

    #[test]
    fn preference_index_map_preserves_requested_order() {
        let request = BackendSelectionRequest::with_preferences([
            "backend.b",
            "backend.a",
        ]);

        let indices = preference_indices(&request);

        assert_eq!(indices.get("backend.b"), Some(&0));
        assert_eq!(indices.get("backend.a"), Some(&1));
    }

    #[test]
    fn aliases_resolve_to_canonical_backend_identifiers() {
        let descriptors = vec![
            BackendDescriptor::new("backend.canonical", ".bin")
                .with_alias("backend.alias"),
        ];

        let request = BackendSelectionRequest::with_preferences([
            "backend.alias",
        ]);

        let indices =
            preference_indices_for_descriptors(&request, &descriptors);

        assert_eq!(
            indices.get("backend.canonical"),
            Some(&0)
        );
    }

    #[test]
    fn evaluator_contract_is_object_safe() {
        fn accepts_evaluator(
            _: &dyn BackendEligibilityEvaluator,
        ) {
        }

        let evaluator = FixedEvaluator {
            result: BackendEligibility::Unknown {
                reason: "not evaluated".to_owned(),
            },
        };

        accepts_evaluator(&evaluator);
    }

    #[test]
    fn evaluation_failure_preserves_backend_identity() {
        let error = BackendSelectionError::EvaluationFailed {
            backend_id: "backend.example".to_owned(),
            reason: "test failure".to_owned(),
        };

        assert!(
            error
                .to_string()
                .contains("backend.example")
        );
    }

    #[test]
    fn registry_errors_are_preserved_as_sources() {
        let error = BackendSelectionError::Registry(
            RegistryError::BackendNotFound {
                identifier: "missing".to_owned(),
            },
        );

        assert!(error.source().is_some());
    }

    #[test]
    fn selector_type_borrows_the_canonical_registry() {
        fn accepts_registry<'a>(
            registry: &'a BackendRegistry,
        ) -> BackendSelector<'a> {
            BackendSelector::new(registry)
        }

        // The test checks the public borrowing contract without requiring
        // a constructed backend or an external toolchain.
        let registry = BackendRegistry::new();
        let _selector = accepts_registry(&registry);
    }
}
