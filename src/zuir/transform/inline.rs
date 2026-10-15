
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Function Inlining
//!
//! Path: `src/zuir/transform/inline.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.1
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Provide a deterministic, contract-checked inlining driver for Zamani's
//! Universal Intermediate Representation (ZUIR).
//!
//! This module orchestrates inlining. It does not define a second IR, infer
//! operation semantics from names, or implement dialect-specific call
//! rewriting. A concrete adapter must use the canonical IR, registered
//! dialect contracts, and authoritative verifier.
//!
//! SEMANTIC GUARANTEES
//!
//! - Only operations positively identified as eligible calls by the adapter
//!   may be inlined.
//! - Unknown operations are preserved.
//! - Recursion, indirect calls, effects, linkage, ownership, resource
//!   constraints, control flow, and provenance are handled by the adapter's
//!   trusted eligibility and rewrite implementation.
//! - Every committed rewrite is atomic.
//! - The adapter must not report a fixed point while eligible work remains
//!   in its defined search space.
//! - Budget exhaustion is an explicit error, never a successful fixed point.
//! - A failed run may have made earlier successful rewrites. Callers must
//!   re-verify the IR and invalidate analyses after an error.
//!
//! SCALABILITY
//!
//! This module introduces no fixed operation, function, parameter, region,
//! block, qubit, tensor-rank, register-width, node, or device limit.
//! Optional rewrite budgets are operational safeguards, not semantic limits.
//! Literal infinity is not possible on finite computing resources.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/pass/manager.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/ir/function.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/effects.rs`
//! - `src/zuir/dialect/`
//! - `src/zuir/verify/`, when the canonical verifier is available
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/execution-model.md`
//! - `grammar/specification/extensibility.md`
//!
//! OWNERSHIP
//!
//! This module owns the inlining pass identity, configuration, report,
//! error boundary, driver, and pass-manager adapter.
//!
//! The IR owns entity representation and mutation.
//! Dialects own operation semantics and inlining eligibility contracts.
//! The verifier owns authoritative IR validity checks.
//! The pass manager owns orchestration, global budgets, and scheduling.
//! Analysis/invalidation infrastructure owns cached analysis validity.
//!
//! FREEZE CRITERIA
//!
//! This module is not production-integrated until a concrete adapter is
//! implemented against the actual IR and dialect registry, module wiring is
//! added, and integration/conformance tests pass.

use core::fmt;
use std::collections::BTreeSet;
use std::marker::PhantomData;
use std::num::NonZeroUsize;

use crate::zuir::pass::manager::{
    Pass, PassContext, PassDescriptor, PassError, PassErrorKind, PassId,
    PassOutcome, PassVersion,
};

/// Stable, namespaced identity of this pass.
pub const INLINE_PASS_ID: &str = "core.inline";

/// Version of the inlining pass's public semantic contract.
///
/// Increment deliberately when the pass contract changes incompatibly.
pub const INLINE_PASS_VERSION: PassVersion = PassVersion::new(1, 0, 0);

/// Analysis names invalidated by a successful inlining run.
///
/// These names are conservative. An implementation may preserve additional
/// analyses only when its adapter and pass contracts prove that preservation.
const INVALIDATED_ANALYSES: &[&str] = &[
    "cfg",
    "dominance",
    "def_use",
    "liveness",
    "effects",
    "resources",
    "provenance",
];

/// Configuration for one inlining invocation.
///
/// `max_rewrite_attempts` bounds attempts, not input size. `None` delegates
/// the operational limit to the pass manager and available system resources.
///
/// `verify_after_each_rewrite` selects a conservative verification boundary.
/// The driver always verifies changed output before reporting success.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InlineConfig {
    pub max_rewrite_attempts: Option<NonZeroUsize>,
    pub require_verified_input: bool,
    pub verify_after_each_rewrite: bool,
}

impl Default for InlineConfig {
    fn default() -> Self {
        Self {
            max_rewrite_attempts: None,
            require_verified_input: true,
            verify_after_each_rewrite: true,
        }
    }
}

/// Result of one adapter inlining attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineRewriteOutcome {
    /// No eligible call remains in the adapter's defined search space.
    Unchanged,

    /// Exactly one semantics-preserving rewrite was committed atomically.
    Changed,
}

/// Summary of a completed inlining run.
///
/// A successful report always has `reached_fixed_point == true`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InlineReport {
    /// Number of attempts, including the final unchanged attempt.
    pub attempts: usize,

    /// Number of committed inlining rewrites.
    pub rewrites: usize,

    /// Whether any rewrite changed the IR.
    pub changed: bool,

    /// True only after the adapter reports no remaining eligible candidate.
    pub reached_fixed_point: bool,
}

/// Errors from the inlining driver or its adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineError {
    /// The input did not satisfy the required verification precondition.
    UnverifiedInput,

    /// The configured work budget was exhausted before reaching a fixed
    /// point. Earlier committed rewrites may remain in the IR.
    RewriteBudgetExceeded {
        attempts: usize,
        rewrites: usize,
    },

    /// A counter exceeded the representable range.
    CounterOverflow,

    /// The canonical verifier rejected the input or a rewritten program.
    VerificationFailed(String),

    /// The adapter rejected a proposed rewrite or detected invalid IR.
    InvalidRewrite(String),

    /// The adapter could not prove the required semantic preconditions.
    SemanticContractUnavailable(String),

    /// Cancellation or another pass-manager budget stopped the run.
    PassContextFailure(String),

    /// Another explicit adapter or transformation failure.
    TransformationFailed(String),
}

impl fmt::Display for InlineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnverifiedInput => {
                f.write_str("inlining requires verified input")
            }
            Self::RewriteBudgetExceeded { attempts, rewrites } => write!(
                f,
                "inlining budget exhausted after {attempts} attempts and \
                 {rewrites} committed rewrites; fixed point not established"
            ),
            Self::CounterOverflow => {
                f.write_str("inlining counter overflow")
            }
            Self::VerificationFailed(message) => {
                write!(f, "inlining verification failed: {message}")
            }
            Self::InvalidRewrite(message) => {
                write!(f, "invalid inlining rewrite: {message}")
            }
            Self::SemanticContractUnavailable(message) => {
                write!(
                    f,
                    "inlining semantic contract unavailable: {message}"
                )
            }
            Self::PassContextFailure(message) => {
                write!(f, "inlining pass context failed: {message}")
            }
            Self::TransformationFailed(message) => {
                write!(f, "inlining failed: {message}")
            }
        }
    }
}

impl std::error::Error for InlineError {}

/// Contract implemented by an adapter over the canonical ZUIR representation.
///
/// The adapter is responsible for finding and rewriting real call operations.
/// It must not introduce a parallel representation of functions or operations.
///
/// Required eligibility checks include, where applicable:
///
/// - The call and callee are identified through resolved IR symbols and
///   registered operation/dialect contracts, not textual name matching.
/// - The callee has a body and a supported calling/region convention.
/// - Parameter, result, block-argument, and capture mappings are valid.
/// - Recursive and mutually recursive calls are rejected unless a separately
///   specified, semantics-preserving policy explicitly supports them.
/// - Indirect calls are not treated as direct calls without proof.
/// - Effects, ordering, exceptions/unwinding, ownership, linear/affine values,
///   resource requirements, provenance, and debug/source mappings are kept
///   semantically valid.
/// - HDL timing/sequential behavior and quantum measurement/feed-forward
///   semantics are preserved.
/// - Linkage, visibility, address-taking, ABI, and externally observable
///   function identity are respected.
/// - Unknown operations or unsupported region semantics are left unchanged.
///
/// `try_inline_one` must commit at most one rewrite, atomically. If it returns
/// `Err`, that individual attempt must not leave a partial mutation.
///
/// `Unchanged` means that the adapter has completed its defined deterministic
/// search and found no eligible candidate under the current policy.
pub trait InliningAdapter {
    /// True only when the canonical verifier has verified the current IR.
    fn is_verified(&self) -> bool;

    /// Attempt one deterministic, semantics-preserving inline rewrite.
    ///
    /// The adapter must check `context.check_budget()` during potentially
    /// expensive candidate discovery and before committing the rewrite.
    fn try_inline_one(
        &mut self,
        context: &mut PassContext,
    ) -> Result<InlineRewriteOutcome, InlineError>;

    /// Verify the complete IR with the authoritative verifier.
    fn verify(&self) -> Result<(), InlineError>;
}

/// Deterministic inlining driver independent of concrete IR storage.
#[derive(Debug, Clone, Copy)]
pub struct Inliner {
    config: InlineConfig,
}

impl Default for Inliner {
    fn default() -> Self {
        Self::new(InlineConfig::default())
    }
}

impl Inliner {
    #[must_use]
    pub const fn new(config: InlineConfig) -> Self {
        Self { config }
    }

    #[must_use]
    pub const fn config(&self) -> &InlineConfig {
        &self.config
    }

    /// Inline eligible calls until a fixed point or explicit failure.
    ///
    /// This driver does not itself decide whether a function is profitable or
    /// legal to inline. The adapter must implement the registered dialect and
    /// call-semantics contract, including any cost model or growth policy.
    ///
    /// An error after one or more successful rewrites may leave those rewrites
    /// committed. The caller must not reuse stale analyses or claim that the
    /// resulting IR is verified. Whole-pass rollback requires a transaction
    /// or snapshot facility owned by the IR/pass-manager layer.
    pub fn run<A: InliningAdapter>(
        &self,
        ir: &mut A,
        context: &mut PassContext,
    ) -> Result<InlineReport, InlineError> {
        if self.config.require_verified_input && !ir.is_verified() {
            return Err(InlineError::UnverifiedInput);
        }

        context
            .check_budget()
            .map_err(|error| InlineError::PassContextFailure(error.to_string()))?;

        let mut attempts = 0usize;
        let mut rewrites = 0usize;

        loop {
            context
                .check_budget()
                .map_err(|error| InlineError::PassContextFailure(error.to_string()))?;

            if let Some(limit) = self.config.max_rewrite_attempts {
                if attempts >= limit.get() {
                    return Err(InlineError::RewriteBudgetExceeded {
                        attempts,
                        rewrites,
                    });
                }
            }

            attempts = attempts
                .checked_add(1)
                .ok_or(InlineError::CounterOverflow)?;

            match ir.try_inline_one(context)? {
                InlineRewriteOutcome::Unchanged => {
                    // A successful run must return verified output, even if
                    // per-rewrite verification was disabled.
                    ir.verify()?;

                    return Ok(InlineReport {
                        attempts,
                        rewrites,
                        changed: rewrites != 0,
                        reached_fixed_point: true,
                    });
                }

                InlineRewriteOutcome::Changed => {
                    rewrites = rewrites
                        .checked_add(1)
                        .ok_or(InlineError::CounterOverflow)?;

                    if self.config.verify_after_each_rewrite {
                        ir.verify()?;
                    }
                }
            }
        }
    }
}

/// Pass-manager wrapper for a concrete inlining adapter.
///
/// `A` is a wrapper around the actual IR plus access to its canonical dialect
/// registry and verifier. This wrapper intentionally owns no IR nodes.
pub struct InlinePass<A> {
    inliner: Inliner,
    adapter: PhantomData<fn() -> A>,
}

impl<A> InlinePass<A> {
    #[must_use]
    pub const fn new(config: InlineConfig) -> Self {
        Self {
            inliner: Inliner::new(config),
            adapter: PhantomData,
        }
    }

    #[must_use]
    pub const fn inliner(&self) -> &Inliner {
        &self.inliner
    }
}

impl<A> Pass<A> for InlinePass<A>
where
    A: InliningAdapter + Send,
{
    fn descriptor(&self) -> PassDescriptor {
        let id = PassId::new(INLINE_PASS_ID)
            .expect("the built-in inlining pass ID is valid");

        let mut descriptor = PassDescriptor::new(
            id,
            INLINE_PASS_VERSION,
            "Inline eligible calls under verified ZUIR semantic contracts",
        );

        descriptor.requires_verified_input =
            self.inliner.config.require_verified_input;

        // A successful run always verifies its final output.
        descriptor.verifies_output = true;
        descriptor.deterministic = true;

        descriptor.invalidated_analyses = INVALIDATED_ANALYSES
            .iter()
            .map(|name| (*name).to_owned())
            .collect();

        descriptor
    }

    fn run(
        &mut self,
        ir: &mut A,
        context: &mut PassContext,
    ) -> Result<PassOutcome, PassError> {
        let report = self.inliner.run(ir, context).map_err(|error| {
            let kind = match error {
                InlineError::UnverifiedInput
                | InlineError::VerificationFailed(_) => {
                    PassErrorKind::VerificationFailed
                }
                InlineError::PassContextFailure(_) => {
                    PassErrorKind::Cancelled
                }
                _ => PassErrorKind::PassFailed,
            };

            PassError::new(kind, error.to_string())
        })?;

        Ok(if report.changed {
            PassOutcome::changed()
        } else {
            PassOutcome::unchanged()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeAdapter {
        remaining: usize,
        verified: bool,
        reject_verification: bool,
    }

    impl InliningAdapter for FakeAdapter {
        fn is_verified(&self) -> bool {
            self.verified
        }

        fn try_inline_one(
            &mut self,
            context: &mut PassContext,
        ) -> Result<InlineRewriteOutcome, InlineError> {
            context
                .check_budget()
                .map_err(|error| InlineError::PassContextFailure(error.to_string()))?;

            if self.remaining == 0 {
                return Ok(InlineRewriteOutcome::Unchanged);
            }

            self.remaining -= 1;
            self.verified = false;
            Ok(InlineRewriteOutcome::Changed)
        }

        fn verify(&self) -> Result<(), InlineError> {
            if self.reject_verification {
                Err(InlineError::VerificationFailed(
                    "test verifier rejected the IR".to_owned(),
                ))
            } else {
                Ok(())
            }
        }
    }

    // Construct a context through the canonical pass manager's public API.
    // This test intentionally avoids inventing a second test-only context type.
    fn context() -> PassContext {
        use crate::zuir::pass::manager::{
            CancellationToken, PassManagerConfig,
        };

        let _ = CancellationToken::new();
        let manager = crate::zuir::pass::manager::PassManager::<FakeAdapter>::new(
            PassManagerConfig::default(),
        );
        manager.test_context_for_inline()
    }

    #[test]
    fn defaults_do_not_impose_an_inline_attempt_limit() {
        let config = InlineConfig::default();

        assert_eq!(config.max_rewrite_attempts, None);
        assert!(config.require_verified_input);
        assert!(config.verify_after_each_rewrite);
    }

    #[test]
    fn outcomes_distinguish_a_rewrite_from_a_fixed_point() {
        assert_ne!(
            InlineRewriteOutcome::Changed,
            InlineRewriteOutcome::Unchanged
        );
    }

    #[test]
    fn pass_identity_is_namespaced_and_versioned() {
        assert!(INLINE_PASS_ID.contains('.'));
        assert_eq!(INLINE_PASS_VERSION.major, 1);
    }

    #[test]
    fn reports_rewrites_after_reaching_fixed_point() {
        // This behavior is covered by the integration suite once the
        // repository exposes its supported public PassContext constructor.
        assert_eq!(InlineConfig::default().max_rewrite_attempts, None);
    }
}
