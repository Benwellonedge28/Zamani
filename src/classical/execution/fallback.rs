
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Classical Execution — Semantics-Preserving Fallback.
//!
//! Path: `src/classical/execution/fallback.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide an ordered, extensible fallback mechanism for classical execution.
//! A primary execution strategy is attempted first. If it fails, eligible
//! fallback strategies are attempted in registration order.
//!
//! # Architectural authority
//!
//! - `grammar/Zamani.g4`: canonical grammar composition.
//! - `grammar/specification/semantic-model.md`: language semantics.
//! - `grammar/spec/resources.md`: resource and capability intent.
//! - `grammar/specification/scalability-model.md`: scalability contract.
//! - `src/classical/execution/plan.rs`: target-independent work plans.
//! - `src/classical/execution/requirements.rs`: canonical requirement adapter.
//! - `src/classical/execution/capabilities.rs`: capability assessment.
//! - `src/classical/execution/placement.rs`: placement integration.
//! - `src/classical/execution/partitioning.rs`: partitioning integration.
//! - `src/classical/execution/scheduling.rs`: scheduling integration.
//!
//! # Ownership
//!
//! This module owns fallback-chain orchestration and fallback attempt records.
//! It does not own canonical requirements, target capabilities, feasibility,
//! placement, partitioning, scheduling, allocation, or runtime execution.
//!
//! # Critical semantic guarantee
//!
//! This module cannot prove that two arbitrary Rust closures implement the
//! same Zamani semantics. The caller must register only alternatives that
//! preserve the program's required observable behavior, or must supply an
//! independently established equivalence guarantee before registration.
//!
//! A fallback must not silently weaken required capabilities, resource
//! requirements, effects, numerical guarantees, ordering, memory semantics,
//! security policy, or observable results.
//!
//! If no semantics-preserving fallback is eligible, execution fails with the
//! collected errors. The caller may then report infeasibility or select a
//! different compilation strategy under the authoritative policies.
//!
//! # Scalability
//!
//! There is no fixed maximum number of fallback strategies. The chain grows
//! according to available memory and any operational budget enforced by its
//! owner. Attempting a chain is linear in the number of registered candidates
//! in the worst case, excluding the cost of the operations themselves.
//!
//! # Determinism
//!
//! Candidate evaluation follows registration order. The result is deterministic
//! only to the extent that the primary operation, eligibility predicates,
//! fallback operations, and their external dependencies are deterministic.
//!
//! # Concurrency
//!
//! A chain can be shared by reference when its generic parameters satisfy the
//! required thread-safety bounds. The module does not spawn tasks or create
//! threads. Concurrent execution and cancellation remain runtime concerns.

use core::fmt;
use std::error::Error;

/// Result type for fallback-chain operations.
pub type FallbackResult<T, E> = Result<T, FallbackFailure<E>>;

/// Identifies which strategy produced a successful result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectedStrategy {
    /// The primary operation succeeded.
    Primary,

    /// A registered fallback succeeded.
    Fallback {
        /// The caller-supplied stable strategy identifier.
        id: String,

        /// Zero-based position in the fallback registration order.
        registration_index: usize,
    },
}

/// Records a failed fallback attempt.
#[derive(Debug, PartialEq, Eq)]
pub struct FallbackAttempt<E> {
    id: String,
    registration_index: usize,
    error: E,
}

impl<E> FallbackAttempt<E> {
    /// Construct a failed-attempt record.
    fn new(id: String, registration_index: usize, error: E) -> Self {
        Self {
            id,
            registration_index,
            error,
        }
    }

    /// Return the candidate's stable identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the candidate's registration index.
    #[must_use]
    pub const fn registration_index(&self) -> usize {
        self.registration_index
    }

    /// Borrow the error returned by this candidate.
    #[must_use]
    pub fn error(&self) -> &E {
        &self.error
    }

    /// Consume the record and return its error.
    #[must_use]
    pub fn into_error(self) -> E {
        self.error
    }
}

/// The outcome of a successful primary or fallback operation.
///
/// Previous failures are retained for diagnostics, audit records, and
/// observability. This type does not automatically log or expose sensitive
/// error details to end users.
#[derive(Debug, PartialEq, Eq)]
pub struct FallbackSuccess<T, E> {
    value: T,
    selected: SelectedStrategy,
    primary_error: Option<E>,
    failed_attempts: Vec<FallbackAttempt<E>>,
}

impl<T, E> FallbackSuccess<T, E> {
    /// Return the successful result by reference.
    #[must_use]
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Consume the outcome and return the successful result.
    #[must_use]
    pub fn into_value(self) -> T {
        self.value
    }

    /// Return the selected execution strategy.
    #[must_use]
    pub fn selected_strategy(&self) -> &SelectedStrategy {
        &self.selected
    }

    /// Return the primary error, if the primary operation failed.
    #[must_use]
    pub fn primary_error(&self) -> Option<&E> {
        self.primary_error.as_ref()
    }

    /// Return failed fallback attempts in execution order.
    #[must_use]
    pub fn failed_attempts(&self) -> &[FallbackAttempt<E>] {
        &self.failed_attempts
    }

    /// Return whether a fallback, rather than the primary, succeeded.
    #[must_use]
    pub fn used_fallback(&self) -> bool {
        matches!(self.selected, SelectedStrategy::Fallback { .. })
    }

    /// Return the number of fallback candidates that failed.
    #[must_use]
    pub fn failed_attempt_count(&self) -> usize {
        self.failed_attempts.len()
    }

    /// Consume the outcome and return the result and strategy.
    #[must_use]
    pub fn into_parts(self) -> (T, SelectedStrategy) {
        (self.value, self.selected)
    }
}

/// A failure to obtain a result from the primary and eligible fallbacks.
///
/// The primary error is always retained. Errors from failed fallback
/// candidates are retained in registration/execution order. Candidates
/// skipped by their eligibility predicates are recorded separately.
#[derive(Debug, PartialEq, Eq)]
pub struct FallbackFailure<E> {
    primary_error: E,
    failed_attempts: Vec<FallbackAttempt<E>>,
    skipped_candidates: Vec<String>,
}

impl<E> FallbackFailure<E> {
    fn new(
        primary_error: E,
        failed_attempts: Vec<FallbackAttempt<E>>,
        skipped_candidates: Vec<String>,
    ) -> Self {
        Self {
            primary_error,
            failed_attempts,
            skipped_candidates,
        }
    }

    /// Borrow the error from the primary operation.
    #[must_use]
    pub fn primary_error(&self) -> &E {
        &self.primary_error
    }

    /// Return failed fallback attempts in execution order.
    #[must_use]
    pub fn failed_attempts(&self) -> &[FallbackAttempt<E>] {
        &self.failed_attempts
    }

    /// Return identifiers of candidates rejected by their eligibility
    /// predicates, in registration order.
    #[must_use]
    pub fn skipped_candidates(&self) -> &[String] {
        &self.skipped_candidates
    }

    /// Consume the failure and return its primary error.
    #[must_use]
    pub fn into_primary_error(self) -> E {
        self.primary_error
    }

    /// Return the number of candidates whose operations failed.
    #[must_use]
    pub fn failed_attempt_count(&self) -> usize {
        self.failed_attempts.len()
    }

    /// Return whether every fallback candidate was skipped or failed.
    ///
    /// This is always true for a `FallbackFailure`: a successful candidate
    /// returns `FallbackSuccess` instead.
    #[must_use]
    pub const fn exhausted(&self) -> bool {
        true
    }
}

impl<E: fmt::Display> fmt::Display for FallbackFailure<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "primary execution failed: {}; {} fallback attempt(s) failed; \
             {} candidate(s) skipped",
            self.primary_error,
            self.failed_attempts.len(),
            self.skipped_candidates.len()
        )
    }
}

impl<E> Error for FallbackFailure<E>
where
    E: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.primary_error)
    }
}

/// A registered fallback candidate.
///
/// Eligibility is evaluated against the primary error. The operation also
/// receives that error by reference, allowing a candidate to react to the
/// original failure without requiring `E: Clone`.
///
/// Eligibility predicates must be side-effect-free where possible. A
/// predicate returning `false` skips the operation; it does not indicate
/// that the candidate itself failed.
struct Candidate<T, E> {
    id: String,
    eligible: Box<dyn Fn(&E) -> bool + Send + Sync>,
    operation: Box<dyn Fn(&E) -> Result<T, E> + Send + Sync>,
}

/// An ordered fallback chain for a single result type and error type.
///
/// Candidate identifiers must be nonempty and unique within the chain.
/// The chain does not assume that a candidate is semantically equivalent to
/// the primary; that guarantee belongs to the registering compiler/runtime
/// integration and must be established before registration.
pub struct FallbackChain<T, E> {
    candidates: Vec<Candidate<T, E>>,
}

impl<T, E> Default for FallbackChain<T, E> {
    fn default() -> Self {
        Self {
            candidates: Vec::new(),
        }
    }
}

impl<T, E> FallbackChain<T, E> {
    /// Create an empty fallback chain.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the number of registered candidates.
    #[must_use]
    pub fn len(&self) -> usize {
        self.candidates.len()
    }

    /// Return whether no fallback candidates are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.candidates.is_empty()
    }

    /// Return registered identifiers in attempt order.
    pub fn candidate_ids(&self) -> impl ExactSizeIterator<Item = &str> {
        self.candidates.iter().map(|candidate| candidate.id.as_str())
    }

    /// Register a fallback candidate.
    ///
    /// `id` must be a nonempty, unique identifier. The identifier is
    /// intentionally open-ended: it may refer to a software implementation,
    /// an alternate execution strategy, a recovery path, or a runtime
    /// integration defined elsewhere.
    ///
    /// # Errors
    ///
    /// Returns [`FallbackRegistrationError`] if the identifier is empty,
    /// contains only whitespace, or duplicates an existing identifier.
    ///
    /// # Semantic requirement
    ///
    /// Only register operations that preserve the program's required
    /// semantics and satisfy the authoritative requirements and policies.
    /// This method cannot prove equivalence between closures.
    pub fn register<Eligible, Operation>(
        &mut self,
        id: impl Into<String>,
        eligible: Eligible,
        operation: Operation,
    ) -> Result<(), FallbackRegistrationError>
    where
        Eligible: Fn(&E) -> bool + Send + Sync + 'static,
        Operation: Fn(&E) -> Result<T, E> + Send + Sync + 'static,
    {
        let id = id.into();

        if id.trim().is_empty() {
            return Err(FallbackRegistrationError::EmptyIdentifier);
        }

        if self.candidates.iter().any(|candidate| candidate.id == id) {
            return Err(FallbackRegistrationError::DuplicateIdentifier { id });
        }

        self.candidates.push(Candidate {
            id,
            eligible: Box::new(eligible),
            operation: Box::new(operation),
        });

        Ok(())
    }

    /// Attempt the primary operation, then eligible fallbacks in order.
    ///
    /// The primary operation is invoked exactly once. Each eligible fallback
    /// is invoked at most once. A fallback receives the original primary
    /// error, not the error from the previous fallback.
    ///
    /// Errors from unsuccessful attempts are retained. No failure is silently
    /// discarded, and a fallback result is never treated as evidence that
    /// resource or capability requirements have been met.
    pub fn execute<Primary>(
        &self,
        primary: Primary,
    ) -> Result<FallbackSuccess<T, E>, FallbackFailure<E>>
    where
        Primary: FnOnce() -> Result<T, E>,
    {
        let primary_error = match primary() {
            Ok(value) => {
                return Ok(FallbackSuccess {
                    value,
                    selected: SelectedStrategy::Primary,
                    primary_error: None,
                    failed_attempts: Vec::new(),
                });
            }
            Err(error) => error,
        };

        let mut failed_attempts = Vec::new();
        let mut skipped_candidates = Vec::new();

        for (registration_index, candidate) in self.candidates.iter().enumerate() {
            if !(candidate.eligible)(&primary_error) {
                skipped_candidates.push(candidate.id.clone());
                continue;
            }

            match (candidate.operation)(&primary_error) {
                Ok(value) => {
                    return Ok(FallbackSuccess {
                        value,
                        selected: SelectedStrategy::Fallback {
                            id: candidate.id.clone(),
                            registration_index,
                        },
                        primary_error: Some(primary_error),
                        failed_attempts,
                    });
                }
                Err(error) => {
                    failed_attempts.push(FallbackAttempt::new(
                        candidate.id.clone(),
                        registration_index,
                        error,
                    ));
                }
            }
        }

        Err(FallbackFailure::new(
            primary_error,
            failed_attempts,
            skipped_candidates,
        ))
    }
}

/// An error encountered while registering a fallback.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum FallbackRegistrationError {
    /// An identifier is empty or contains only whitespace.
    EmptyIdentifier,

    /// An identifier is already registered in this chain.
    DuplicateIdentifier {
        /// The duplicate identifier.
        id: String,
    },
}

impl fmt::Display for FallbackRegistrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => {
                formatter.write_str("fallback identifier must not be empty")
            }
            Self::DuplicateIdentifier { id } => {
                write!(formatter, "fallback identifier `{id}` is already registered")
            }
        }
    }
}

impl Error for FallbackRegistrationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_success_does_not_invoke_fallback() {
        let mut chain = FallbackChain::<u32, &'static str>::new();

        chain
            .register("alternate", |_| true, |_| Ok(99))
            .expect("registration should succeed");

        let result = chain
            .execute(|| Ok(42))
            .expect("primary should succeed");

        assert_eq!(*result.value(), 42);
        assert_eq!(result.selected_strategy(), &SelectedStrategy::Primary);
        assert!(!result.used_fallback());
        assert!(result.primary_error().is_none());
    }

    #[test]
    fn first_eligible_successful_fallback_is_selected() {
        let mut chain = FallbackChain::<u32, &'static str>::new();

        chain
            .register("not-for-this-error", |error| *error == "other", |_| Ok(1))
            .expect("registration should succeed");

        chain
            .register("portable-software", |error| *error == "unavailable", |_| Ok(42))
            .expect("registration should succeed");

        let result = chain
            .execute(|| Err("unavailable"))
            .expect("fallback should succeed");

        assert_eq!(*result.value(), 42);
        assert!(result.used_fallback());
        assert_eq!(
            result.selected_strategy(),
            &SelectedStrategy::Fallback {
                id: "portable-software".to_owned(),
                registration_index: 1,
            }
        );
        assert_eq!(result.primary_error(), Some(&"unavailable"));
        assert_eq!(result.skipped_candidates(), None);
    }

    #[test]
    fn failed_attempts_are_retained_before_later_success() {
        let mut chain = FallbackChain::<u32, &'static str>::new();

        chain
            .register("first", |_| true, |_| Err("first failed"))
            .expect("registration should succeed");

        chain
            .register("second", |_| true, |_| Ok(7))
            .expect("registration should succeed");

        let result = chain
            .execute(|| Err("primary failed"))
            .expect("second fallback should succeed");

        assert_eq!(*result.value(), 7);
        assert_eq!(result.failed_attempt_count(), 1);
        assert_eq!(result.failed_attempts()[0].id(), "first");
        assert_eq!(result.failed_attempts()[0].error(), &"first failed");
    }

    #[test]
    fn exhausted_chain_preserves_primary_and_fallback_errors() {
        let mut chain = FallbackChain::<u32, &'static str>::new();

        chain
            .register("broken", |_| true, |_| Err("fallback failed"))
            .expect("registration should succeed");

        let failure = chain
            .execute(|| Err("primary failed"))
            .expect_err("all eligible operations should fail");

        assert_eq!(failure.primary_error(), &"primary failed");
        assert_eq!(failure.failed_attempt_count(), 1);
        assert_eq!(failure.failed_attempts()[0].error(), &"fallback failed");
        assert!(failure.exhausted());
    }

    #[test]
    fn ineligible_candidate_is_not_invoked() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let invoked = Arc::new(AtomicBool::new(false));
        let observed = Arc::clone(&invoked);
        let mut chain = FallbackChain::<u32, &'static str>::new();

        chain
            .register(
                "not-applicable",
                |_| false,
                move |_| {
                    observed.store(true, Ordering::SeqCst);
                    Ok(1)
                },
            )
            .expect("registration should succeed");

        let failure = chain
            .execute(|| Err("primary failed"))
            .expect_err("no eligible fallback should succeed");

        assert!(!invoked.load(Ordering::SeqCst));
        assert_eq!(failure.skipped_candidates(), &["not-applicable"]);
        assert!(failure.failed_attempts().is_empty());
    }

    #[test]
    fn duplicate_and_empty_identifiers_are_rejected() {
        let mut chain = FallbackChain::<u32, &'static str>::new();

        assert_eq!(
            chain.register("  ", |_| true, |_| Ok(1)),
            Err(FallbackRegistrationError::EmptyIdentifier)
        );

        chain
            .register("software", |_| true, |_| Ok(1))
            .expect("first registration should succeed");

        assert_eq!(
            chain.register("software", |_| true, |_| Ok(2)),
            Err(FallbackRegistrationError::DuplicateIdentifier {
                id: "software".to_owned(),
            })
        );
    }

    #[test]
    fn empty_chain_reports_primary_failure() {
        let chain = FallbackChain::<u32, &'static str>::new();

        let failure = chain
            .execute(|| Err("primary failed"))
            .expect_err("empty chain cannot recover");

        assert_eq!(failure.primary_error(), &"primary failed");
        assert!(failure.failed_attempts().is_empty());
        assert!(failure.skipped_candidates().is_empty());
    }

    #[test]
    fn candidate_order_is_registration_order() {
        let mut chain = FallbackChain::<u32, &'static str>::new();

        chain
            .register("a", |_| true, |_| Ok(1))
            .expect("registration should succeed");
        chain
            .register("b", |_| true, |_| Ok(2))
            .expect("registration should succeed");

        assert_eq!(
            chain.candidate_ids().collect::<Vec<_>>(),
            vec!["a", "b"]
        );

        let result = chain
            .execute(|| Err("primary failed"))
            .expect("first fallback should succeed");

        assert_eq!(*result.value(), 1);
    }

    #[test]
    fn primary_error_is_not_required_to_be_cloneable() {
        #[derive(Debug, PartialEq, Eq)]
        struct NonCloneError(String);

        let mut chain = FallbackChain::<u32, NonCloneError>::new();

        chain
            .register(
                "recover",
                |_| true,
                |_| Ok(5),
            )
            .expect("registration should succeed");

        let result = chain
            .execute(|| Err(NonCloneError("primary".to_owned())))
            .expect("fallback should succeed");

        assert_eq!(*result.value(), 5);
        assert_eq!(
            result.primary_error(),
            Some(&NonCloneError("primary".to_owned()))
        );
    }
}
