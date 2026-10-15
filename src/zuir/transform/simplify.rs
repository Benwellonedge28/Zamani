//! # ZUIR Semantics-Preserving Simplification
//!
//! Canonical path: `src/zuir/transform/simplify.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97 or later
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Provide a domain-neutral simplification driver for Zamani's Universal
//! Intermediate Representation (ZUIR). Concrete IR traversal and rewrite
//! rules are supplied by a trusted adapter backed by the canonical IR,
//! dialect registry, and verifier.
//!
//! OWNERSHIP
//!
//! This module owns:
//! - Simplification configuration and execution reporting.
//! - A deterministic, budget-aware simplification driver.
//! - The adapter contract for proposing and committing rewrites.
//! - Explicit failure behavior when a fixed point cannot be established.
//!
//! This module does not own:
//! - IR nodes, values, types, or identifiers.
//! - Operation semantics or dialect-specific rewrite rules.
//! - Constant evaluation or target-specific feasibility.
//! - Verification algorithms or pass-manager scheduling.
//! - Physical placement, routing, scheduling, or execution.
//!
//! SEMANTIC GUARANTEES
//!
//! - Unknown operations are not simplified unless a registered, trusted
//!   dialect explicitly supplies a semantics-preserving rewrite.
//! - Observable effects must be preserved.
//! - Quantum measurement, reset, classical feed-forward, ownership,
//!   synchronization, communication, and hardware timing must not be
//!   erased or reordered without an explicit proof of equivalence.
//! - A rewrite is atomic: failure must not leave a partially committed IR.
//! - Reaching a work budget is not reported as reaching a fixed point.
//! - The driver does not assume that every rewrite system terminates.
//!
//! SCALABILITY
//!
//! There is no fixed IR-size, operation-count, qubit-count, tensor-rank,
//! register-width, processor-count, device-count, or node-count limit here.
//! Optional work budgets are operational safeguards, not semantic limits.
//! Literal infinity is impossible on finite hardware; resource exhaustion
//! must be reported rather than silently changing program meaning.
//!
//! INTEGRATION
//!
//! - `pass/manager.rs` owns pipeline execution, cancellation, and global
//!   budgets.
//! - `pass/contracts.rs` owns the canonical pass contract types.
//! - `ir/operation.rs` owns operation representation.
//! - `dialect/` owns registered operation semantics and rewrite legality.
//! - `verify/` owns authoritative IR verification.
//! - `transform/canonicalize.rs` owns canonicalization, a distinct pass.
//!
//! This file deliberately avoids importing concrete IR node types until a
//! repository adapter can be implemented against their finalized APIs.
//!
//! FREEZE CRITERIA
//!
//! The adapter must be tested against the real IR and dialect registry.
//! The module must compile as part of the actual crate, pass its tests,
//! and be registered through the canonical pass manager before production
//! readiness is claimed.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::num::NonZeroUsize;

/// Stable, namespaced identifier for the simplification pass.
pub const SIMPLIFY_PASS_ID: &str = "core.simplify";

/// Version of this pass's public semantic contract.
///
/// Increment deliberately when the contract changes incompatibly.
pub const SIMPLIFY_PASS_VERSION: (u64, u64, u64) = (1, 0, 0);

/// Configuration for one simplification invocation.
///
/// `max_rewrite_attempts` limits attempted rewrites, not the size of the
/// input IR. `None` means no local attempt limit; the pass manager may still
/// enforce a global work budget or cancellation policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimplifyConfig {
    /// Optional local rewrite-attempt budget.
    pub max_rewrite_attempts: Option<NonZeroUsize>,

    /// Reject input that the adapter cannot confirm as verified.
    pub require_verified_input: bool,

    /// Verify the whole IR after each committed rewrite.
    ///
    /// This conservative mode is the default. Disabling it is only
    /// appropriate when the surrounding pass manager provides an equivalent
    /// verified transaction boundary.
    pub verify_after_each_rewrite: bool,
}

impl Default for SimplifyConfig {
    fn default() -> Self {
        Self {
            max_rewrite_attempts: None,
            require_verified_input: true,
            verify_after_each_rewrite: true,
        }
    }
}

/// Outcome of one adapter rewrite attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewriteOutcome {
    /// No eligible rewrite remains in the adapter's current search state.
    Unchanged,

    /// Exactly one semantics-preserving rewrite was committed atomically.
    Changed,
}

/// Outcome of a completed simplification run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimplifyReport {
    /// Number of adapter attempts, including the final unchanged attempt.
    pub attempts: usize,

    /// Number of successfully committed rewrites.
    pub rewrites: usize,

    /// Whether at least one rewrite changed the IR.
    pub changed: bool,

    /// True only when the adapter explicitly established a fixed point.
    pub reached_fixed_point: bool,
}

/// Failures returned by the simplification driver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimplifyError {
    /// Input did not satisfy the required verification precondition.
    UnverifiedInput,

    /// The local rewrite budget was exhausted before a fixed point.
    RewriteBudgetExceeded {
        attempts: usize,
        rewrites: usize,
    },

    /// The driver or adapter detected counter overflow.
    CounterOverflow,

    /// The verifier rejected the input or a committed rewrite.
    VerificationFailed(String),

    /// A proposed rewrite violated its declared semantic contract.
    InvalidRewrite(String),

    /// A dialect or adapter could not establish semantic equivalence.
    SemanticContractUnavailable(String),

    /// The transformation failed for another explicit reason.
    TransformationFailed(String),
}

impl fmt::Display for SimplifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnverifiedInput => {
                f.write_str("simplification requires verified input")
            }
            Self::RewriteBudgetExceeded { attempts, rewrites } => write!(
                f,
                "simplification budget exhausted after {attempts} attempts \
                 and {rewrites} committed rewrites; fixed point not established"
            ),
            Self::CounterOverflow => {
                f.write_str("simplification counter overflow")
            }
            Self::VerificationFailed(message) => {
                write!(f, "simplification verification failed: {message}")
            }
            Self::InvalidRewrite(message) => {
                write!(f, "invalid simplification rewrite: {message}")
            }
            Self::SemanticContractUnavailable(message) => {
                write!(
                    f,
                    "simplification semantic contract unavailable: {message}"
                )
            }
            Self::TransformationFailed(message) => {
                write!(f, "simplification failed: {message}")
            }
        }
    }
}

impl std::error::Error for SimplifyError {}

/// Boundary implemented by the owner of the concrete ZUIR representation.
///
/// Implementations must:
///
/// 1. Traverse deterministically.
/// 2. Consult registered dialect contracts rather than infer semantics from
///    operation names.
/// 3. Preserve unknown operations when no trusted rule applies.
/// 4. Commit at most one atomic rewrite per `try_simplify_one` call.
/// 5. Return `Unchanged` only when its defined search space has no remaining
///    eligible rewrite.
/// 6. Ensure a failed attempt leaves the IR unchanged.
/// 7. Verify through the repository's authoritative verifier.
///
/// This trait is intentionally independent of any particular IR storage
/// strategy. It is not a replacement for the repository's IR or dialect API.
pub trait SimplificationAdapter {
    /// True only if the current IR has passed the required verifier.
    fn is_verified(&self) -> bool;

    /// Attempt one deterministic, semantics-preserving rewrite.
    fn try_simplify_one(&mut self) -> Result<RewriteOutcome, SimplifyError>;

    /// Verify the entire IR through the canonical verifier.
    fn verify(&self) -> Result<(), SimplifyError>;
}

/// Domain-neutral simplification driver.
#[derive(Debug, Clone, Copy, Default)]
pub struct Simplifier {
    config: SimplifyConfig,
}

impl Simplifier {
    /// Construct a driver with explicit policy.
    #[must_use]
    pub const fn new(config: SimplifyConfig) -> Self {
        Self { config }
    }

    /// Return the active configuration.
    #[must_use]
    pub const fn config(&self) -> &SimplifyConfig {
        &self.config
    }

    /// Run deterministic simplification until a fixed point or failure.
    ///
    /// A successful report always has `reached_fixed_point == true`.
    /// Budget exhaustion, verification failure, or an adapter error returns
    /// `Err`; partial progress is not misreported as completed simplification.
    ///
    /// Note that an error after earlier successful rewrites does not roll back
    /// those already committed rewrites. The pass manager must therefore
    /// treat an error as a failed pass with potentially mutated IR and must
    /// not reuse stale analyses or claim that the IR is verified. Callers
    /// requiring whole-pass rollback must run this pass against a transaction
    /// or snapshot owned by the IR/pass-manager layer.
    pub fn run<A: SimplificationAdapter>(
        &self,
        ir: &mut A,
    ) -> Result<SimplifyReport, SimplifyError> {
        if self.config.require_verified_input && !ir.is_verified() {
            return Err(SimplifyError::UnverifiedInput);
        }

        let mut attempts = 0usize;
        let mut rewrites = 0usize;

        loop {
            if let Some(limit) = self.config.max_rewrite_attempts {
                if attempts >= limit.get() {
                    return Err(SimplifyError::RewriteBudgetExceeded {
                        attempts,
                        rewrites,
                    });
                }
            }

            attempts = attempts
                .checked_add(1)
                .ok_or(SimplifyError::CounterOverflow)?;

            match ir.try_simplify_one()? {
                RewriteOutcome::Unchanged => {
                    return Ok(SimplifyReport {
                        attempts,
                        rewrites,
                        changed: rewrites != 0,
                        reached_fixed_point: true,
                    });
                }

                RewriteOutcome::Changed => {
                    rewrites = rewrites
                        .checked_add(1)
                        .ok_or(SimplifyError::CounterOverflow)?;

                    if self.config.verify_after_each_rewrite {
                        ir.verify()?;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestAdapter {
        remaining: usize,
        verified: bool,
        reject_verification: bool,
    }

    impl SimplificationAdapter for TestAdapter {
        fn is_verified(&self) -> bool {
            self.verified
        }

        fn try_simplify_one(&mut self) -> Result<RewriteOutcome, SimplifyError> {
            if self.remaining == 0 {
                return Ok(RewriteOutcome::Unchanged);
            }

            self.remaining -= 1;
            self.verified = false;
            Ok(RewriteOutcome::Changed)
        }

        fn verify(&self) -> Result<(), SimplifyError> {
            if self.reject_verification {
                Err(SimplifyError::VerificationFailed(
                    "test verifier rejection".to_owned(),
                ))
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn default_has_no_local_attempt_limit() {
        assert_eq!(SimplifyConfig::default().max_rewrite_attempts, None);
        assert!(SimplifyConfig::default().require_verified_input);
        assert!(SimplifyConfig::default().verify_after_each_rewrite);
    }

    #[test]
    fn reaches_fixed_point_and_reports_rewrites() {
        let mut adapter = TestAdapter {
            remaining: 2,
            verified: true,
            reject_verification: false,
        };

        // This fixture keeps verification status external to verify() so that
        // the driver contract can be tested independently of a real IR.
        let report = Simplifier::new(SimplifyConfig {
            require_verified_input: true,
            verify_after_each_rewrite: true,
            max_rewrite_attempts: None,
        })
        .run(&mut adapter)
        .expect("simplification should reach a fixed point");

        assert_eq!(report.rewrites, 2);
        assert_eq!(report.attempts, 3);
        assert!(report.changed);
        assert!(report.reached_fixed_point);
    }

    #[test]
    fn refuses_unverified_input() {
        let mut adapter = TestAdapter {
            remaining: 0,
            verified: false,
            reject_verification: false,
        };

        let result = Simplifier::default().run(&mut adapter);

        assert_eq!(result, Err(SimplifyError::UnverifiedInput));
    }

    #[test]
    fn budget_exhaustion_does_not_claim_fixed_point() {
        let mut adapter = TestAdapter {
            remaining: 3,
            verified: true,
            reject_verification: false,
        };

        let result = Simplifier::new(SimplifyConfig {
            max_rewrite_attempts: NonZeroUsize::new(1),
            require_verified_input: true,
            verify_after_each_rewrite: false,
        })
        .run(&mut adapter);

        assert!(matches!(
            result,
            Err(SimplifyError::RewriteBudgetExceeded {
                attempts: 1,
                rewrites: 1
            })
        ));
    }

    #[test]
    fn verifier_failure_is_not_suppressed() {
        let mut adapter = TestAdapter {
            remaining: 1,
            verified: true,
            reject_verification: true,
        };

        let result = Simplifier::default().run(&mut adapter);

        assert!(matches!(
            result,
            Err(SimplifyError::VerificationFailed(_))
        ));
    }

    #[test]
    fn pass_identity_is_namespaced() {
        assert!(SIMPLIFY_PASS_ID.contains('.'));
        assert_eq!(SIMPLIFY_PASS_VERSION, (1, 0, 0));
    }
}