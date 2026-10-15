//! ZUIR Legalization Pass
//!
//! Path: src/zuir/transform/legalize.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Provide a deterministic, budget-aware legalization driver for the
//! canonical Zamani Universal Intermediate Representation (ZUIR).
//!
//! Legalization converts a representation into a form accepted by a
//! specified downstream stage. The concrete IR, dialect registry, and
//! target-independent legality contracts are accessed through
//! `LegalizationAdapter`; this module does not define a second IR.
//!
//! AUTHORITY
//!
//! - src/zuir/ZUIR-SPEC.md
//! - src/zuir/pass/manager.rs
//! - src/zuir/ir/operation.rs
//! - src/zuir/ir/effects.rs
//! - src/zuir/dialect/
//! - src/zuir/verify/
//! - src/zuir/lowering/ (when its canonical API is available)
//! - grammar/DESIGN.md
//! - grammar/specification/semantic-model.md
//! - grammar/specification/compilation-model.md
//! - grammar/specification/execution-model.md
//!
//! OWNERSHIP
//!
//! This file owns legalization-pass identity, configuration, reporting,
//! the adapter protocol, and pass-manager integration.
//!
//! The canonical IR owns representation and mutation.
//! Dialects own operation semantics and authorized conversions.
//! The verifier owns authoritative IR validity checks.
//! The pass manager owns pipeline execution, cancellation, and global
//! execution budgets.
//! Downstream lowering owns target-specific realization.
//!
//! SEMANTIC GUARANTEES
//!
//! - A conversion requires an explicit, trusted legality contract.
//! - Unknown operations are preserved or rejected explicitly.
//! - No conversion may silently discard effects, ownership, provenance,
//!   resource constraints, contracts, or observable behavior.
//! - Quantum measurement and feed-forward semantics must be preserved.
//! - HDL timing, signal, and sequential semantics must be preserved.
//! - Concurrency, communication, ordering, and numerical behavior must
//!   remain valid under the applicable semantic contract.
//! - Each individual rewrite must be atomic.
//! - A successful run verifies its final output.
//! - Budget exhaustion is never reported as a completed fixed point.
//!
//! SCALABILITY
//!
//! This module defines no universal machine-size, operation-count, qubit,
//! tensor-rank, register-width, processor, device, or node limit.
//! Optional rewrite budgets limit work, not program meaning.
//! Literal infinity is not promised; resource exhaustion must be explicit.
//!
//! FREEZE CRITERIA
//!
//! This file is not production-integrated until a concrete adapter uses the
//! repository's canonical IR, registered dialect contracts, and verifier;
//! the module is exported through the actual module tree; and compilation,
//! integration, semantic-preservation, and stress tests pass.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::BTreeSet;
use std::marker::PhantomData;
use std::num::NonZeroUsize;

use crate::zuir::pass::manager::{
    Pass, PassContext, PassDescriptor, PassError, PassErrorKind, PassId,
    PassOutcome, PassVersion,
};

/// Stable, namespaced identifier of this pass.
pub const LEGALIZE_PASS_ID: &str = "core.legalize";

/// Version of the legalization pass contract.
///
/// Increment deliberately when the public semantic contract changes
/// incompatibly. Never derive this value from timestamps or registration
/// order.
pub const LEGALIZE_PASS_VERSION: PassVersion = PassVersion::new(1, 0, 0);

/// Analyses conservatively invalidated by a successful legalization run.
///
/// These are names used by the current pass-manager analysis contract.
/// Preserve an analysis only when the adapter can prove that it remains
/// valid for the resulting IR.
const INVALIDATED_ANALYSES: &[&str] = &[
    "cfg",
    "dominance",
    "def_use",
    "liveness",
    "effects",
    "ownership",
    "resources",
    "provenance",
    "types",
    "dialect_legality",
    "capabilities",
    "contracts",
];

/// Configuration for one legalization invocation.
///
/// `max_rewrite_attempts` limits attempts, not IR size, hardware capacity,
/// or the number of entities a valid program may contain.
///
/// `None` delegates operational limits to the pass manager and available
/// system resources. It does not mean unlimited memory or execution time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegalizeConfig {
    /// Optional local limit on rewrite attempts.
    pub max_rewrite_attempts: Option<NonZeroUsize>,

    /// Require the adapter to confirm that input is already verified.
    pub require_verified_input: bool,

    /// Verify the IR after every committed rewrite.
    ///
    /// When false, the complete output is still verified before a successful
    /// report is returned. Disabling per-rewrite checks is appropriate only
    /// when the adapter and surrounding pipeline provide an equivalent
    /// transaction and verification boundary.
    pub verify_after_each_rewrite: bool,
}

impl Default for LegalizeConfig {
    fn default() -> Self {
        Self {
            max_rewrite_attempts: None,
            require_verified_input: true,
            verify_after_each_rewrite: true,
        }
    }
}

/// Outcome of one atomic legalization attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegalizeRewriteOutcome {
    /// No further legal rewrite exists in the adapter's defined search space.
    ///
    /// The adapter must not return this merely because it encountered an
    /// unknown operation or temporarily gave up searching.
    Unchanged,

    /// Exactly one authorized rewrite was committed atomically.
    Changed,
}

/// Summary of a successful legalization run.
///
/// A successful report always means the adapter reached its defined fixed
/// point and verified the final IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegalizeReport {
    /// Number of attempts, including the final unchanged attempt.
    pub attempts: usize,

    /// Number of committed rewrites.
    pub rewrites: usize,

    /// Whether at least one rewrite changed the IR.
    pub changed: bool,

    /// True only after no further authorized rewrite remains.
    pub reached_fixed_point: bool,

    /// True only after final verification succeeded.
    pub output_verified: bool,
}

/// Errors produced by the legalization driver or adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LegalizeError {
    /// The adapter could not confirm that required input verification passed.
    UnverifiedInput,

    /// The configured local attempt budget was exhausted before reaching
    /// a fixed point. Earlier successful rewrites may remain committed.
    RewriteBudgetExceeded {
        attempts: usize,
        rewrites: usize,
    },

    /// A checked attempt or rewrite counter overflowed.
    CounterOverflow,

    /// The canonical verifier rejected the input or transformed output.
    VerificationFailed(String),

    /// A proposed rewrite violated an IR invariant or failed validation.
    InvalidRewrite(String),

    /// The required semantic equivalence or legality contract was absent,
    /// ambiguous, or could not be established.
    SemanticContractUnavailable(String),

    /// A pass-manager cancellation or global budget stopped the pass.
    ///
    /// Preserve the typed failure category; do not infer it from diagnostic
    /// wording, which is not a stable machine-readable interface.
    PassContextFailure {
        kind: PassErrorKind,
        message: String,
    },

    /// Another explicit adapter or transformation failure.
    TransformationFailed(String),
}

impl fmt::Display for LegalizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnverifiedInput => {
                f.write_str("legalization requires verified input")
            }
            Self::RewriteBudgetExceeded { attempts, rewrites } => write!(
                f,
                "legalization rewrite budget exhausted after {attempts} \
                 attempts and {rewrites} rewrites; fixed point not established"
            ),
            Self::CounterOverflow => {
                f.write_str("legalization counter overflow")
            }
            Self::VerificationFailed(message) => {
                write!(f, "legalization verification failed: {message}")
            }
            Self::InvalidRewrite(message) => {
                write!(f, "invalid legalization rewrite: {message}")
            }
            Self::SemanticContractUnavailable(message) => write!(
                f,
                "legalization semantic contract unavailable: {message}"
            ),
            Self::PassContextFailure { message, .. } => {
                write!(f, "legalization pass context failed: {message}")
            }
            Self::TransformationFailed(message) => {
                write!(f, "legalization failed: {message}")
            }
        }
    }
}

impl std::error::Error for LegalizeError {}

/// Adapter implemented by the owner of the canonical ZUIR representation.
///
/// This trait is an integration boundary, not a substitute for the IR,
/// dialect registry, verifier, or lowering legality model.
///
/// Implementations must:
///
/// 1. Traverse candidates deterministically when deterministic mode is on.
/// 2. Consult registered operation and dialect contracts.
/// 3. Never infer semantics from operation-name strings alone.
/// 4. Preserve unknown operations unless an explicit policy requires a
///    structured rejection; never silently reinterpret them.
/// 5. Commit at most one rewrite per call.
/// 6. Make each rewrite atomic: `Err` must not leave a partial rewrite.
/// 7. Return `Unchanged` only after completing the defined search and
///    establishing that no authorized rewrite remains.
/// 8. Check `context.check_budget()` during potentially expensive work and
///    before committing a rewrite.
/// 9. Use the canonical verifier for complete IR verification.
///
/// Whole-pass atomicity is not guaranteed by this trait. If a later attempt
/// fails after earlier rewrites committed, the IR may contain those earlier
/// changes. Callers must not assume that it is verified or reuse stale
/// analyses after an error.
pub trait LegalizationAdapter {
    /// True only when the authoritative verifier has verified the current IR.
    fn is_verified(&self) -> bool;

    /// Attempt one deterministic, semantics-preserving legalization rewrite.
    ///
    /// A rewrite may lower an operation to a representation accepted by a
    /// specified downstream stage, but only when the adapter can prove that
    /// the operation's required meaning is preserved.
    ///
    /// Target-specific feasibility is not the same as universal legality:
    /// a missing capability must be reported or handled by an explicitly
    /// authorized fallback, never by silently changing program behavior.
    fn try_legalize_one(
        &mut self,
        context: &mut PassContext,
    ) -> Result<LegalizeRewriteOutcome, LegalizeError>;

    /// Verify the complete current IR with the canonical verifier.
    fn verify(&self) -> Result<(), LegalizeError>;
}

/// Deterministic legalization driver independent of concrete IR storage.
#[derive(Debug, Clone, Copy)]
pub struct Legalizer {
    config: LegalizeConfig,
}

impl Default for Legalizer {
    fn default() -> Self {
        Self::new(LegalizeConfig::default())
    }
}

impl Legalizer {
    /// Construct a legalizer with an explicit configuration.
    #[must_use]
    pub const fn new(config: LegalizeConfig) -> Self {
        Self { config }
    }

    /// Return the active configuration.
    #[must_use]
    pub const fn config(&self) -> &LegalizeConfig {
        &self.config
    }

    /// Legalize until the adapter reaches a fixed point or an explicit
    /// failure occurs.
    ///
    /// A successful return guarantees:
    ///
    /// - the adapter reported no remaining authorized rewrite;
    /// - the final IR passed canonical verification;
    /// - attempt and rewrite counters remained representable.
    ///
    /// A failure after earlier rewrites may leave those rewrites committed.
    /// The caller must re-verify the IR and invalidate or discard stale
    /// analyses before reusing it. Whole-pass rollback requires a transaction
    /// or snapshot mechanism owned by the IR or pass-manager layer.
    pub fn run<A: LegalizationAdapter>(
        &self,
        ir: &mut A,
        context: &mut PassContext,
    ) -> Result<LegalizeReport, LegalizeError> {
        if self.config.require_verified_input && !ir.is_verified() {
            return Err(LegalizeError::UnverifiedInput);
        }

        context.check_budget().map_err(|error| {
            LegalizeError::PassContextFailure {
                kind: error.kind.clone(),
                message: error.to_string(),
            }
        })?;

        let mut attempts = 0usize;
        let mut rewrites = 0usize;

        loop {
            context.check_budget().map_err(|error| {
                LegalizeError::PassContextFailure {
                    kind: error.kind.clone(),
                    message: error.to_string(),
                }
            })?;

            if let Some(limit) = self.config.max_rewrite_attempts {
                if attempts >= limit.get() {
                    return Err(LegalizeError::RewriteBudgetExceeded {
                        attempts,
                        rewrites,
                    });
                }
            }

            attempts = attempts
                .checked_add(1)
                .ok_or(LegalizeError::CounterOverflow)?;

            match ir.try_legalize_one(context)? {
                LegalizeRewriteOutcome::Unchanged => {
                    // Always verify the final state, even if per-rewrite
                    // verification was explicitly disabled.
                    ir.verify()?;

                    return Ok(LegalizeReport {
                        attempts,
                        rewrites,
                        changed: rewrites != 0,
                        reached_fixed_point: true,
                        output_verified: true,
                    });
                }

                LegalizeRewriteOutcome::Changed => {
                    rewrites = rewrites
                        .checked_add(1)
                        .ok_or(LegalizeError::CounterOverflow)?;

                    if self.config.verify_after_each_rewrite {
                        ir.verify()?;
                    }
                }
            }
        }
    }
}

/// Pass-manager wrapper for a concrete legalization adapter.
///
/// `A` must be the actual canonical IR type or a wrapper that provides access
/// to that IR, its registered dialect contracts, and its authoritative
/// verifier. This wrapper does not own or duplicate IR nodes.
pub struct LegalizePass<A> {
    legalizer: Legalizer,
    adapter: PhantomData<fn() -> A>,
}

impl<A> LegalizePass<A> {
    /// Construct a pass with explicit legalization policy.
    #[must_use]
    pub const fn new(config: LegalizeConfig) -> Self {
        Self {
            legalizer: Legalizer::new(config),
            adapter: PhantomData,
        }
    }

    /// Return the configured driver.
    #[must_use]
    pub const fn legalizer(&self) -> &Legalizer {
        &self.legalizer
    }
}

impl<A> Pass<A> for LegalizePass<A>
where
    A: LegalizationAdapter + Send,
{
    fn descriptor(&self) -> PassDescriptor {
        // This constant is a valid namespaced built-in pass identifier.
        let id = PassId::new(LEGALIZE_PASS_ID)
            .expect("the built-in legalization pass ID is valid");

        let mut descriptor = PassDescriptor::new(
            id,
            LEGALIZE_PASS_VERSION,
            "Legalize ZUIR using registered semantics-preserving conversions",
        );

        descriptor.requires_verified_input =
            self.legalizer.config.require_verified_input;

        // The driver always verifies final output before returning success.
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
        let report = self.legalizer.run(ir, context).map_err(|error| {
            let kind = match &error {
                LegalizeError::UnverifiedInput
                | LegalizeError::VerificationFailed(_) => {
                    PassErrorKind::VerificationFailed
                }
                LegalizeError::RewriteBudgetExceeded { .. } => {
                    PassErrorKind::ExecutionLimitExceeded
                }
                LegalizeError::PassContextFailure { kind, .. } => kind.clone(),
                LegalizeError::CounterOverflow
                | LegalizeError::InvalidRewrite(_)
                | LegalizeError::SemanticContractUnavailable(_)
                | LegalizeError::TransformationFailed(_) => {
                    PassErrorKind::PassFailed
                }
            };

            PassError::for_pass(
                kind,
                PassId::new(LEGALIZE_PASS_ID)
                    .expect("the built-in legalization pass ID is valid"),
                error.to_string(),
            )
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

    #[test]
    fn default_configuration_has_no_artificial_rewrite_limit() {
        let config = LegalizeConfig::default();

        assert_eq!(config.max_rewrite_attempts, None);
        assert!(config.require_verified_input);
        assert!(config.verify_after_each_rewrite);
    }

    #[test]
    fn an_explicit_attempt_budget_is_nonzero() {
        let config = LegalizeConfig {
            max_rewrite_attempts: NonZeroUsize::new(64),
            require_verified_input: true,
            verify_after_each_rewrite: true,
        };

        assert_eq!(
            config.max_rewrite_attempts.map(NonZeroUsize::get),
            Some(64)
        );
    }

    #[test]
    fn reports_distinguish_rewrites_from_fixed_point() {
        assert_ne!(
            LegalizeRewriteOutcome::Changed,
            LegalizeRewriteOutcome::Unchanged
        );
    }

    #[test]
    fn pass_identity_is_namespaced_and_versioned() {
        assert!(LEGALIZE_PASS_ID.contains('.'));
        assert_eq!(LEGALIZE_PASS_VERSION.major, 1);
    }

    #[test]
    fn rewrite_budget_exhaustion_is_not_a_successful_fixed_point() {
        let error = LegalizeError::RewriteBudgetExceeded {
            attempts: 64,
            rewrites: 64,
        };

        assert!(error.to_string().contains("fixed point not established"));
    }

    #[test]
    fn context_errors_retain_typed_categories() {
        let error = LegalizeError::PassContextFailure {
            kind: PassErrorKind::Cancelled,
            message: "pass execution was cancelled".to_owned(),
        };

        match error {
            LegalizeError::PassContextFailure { kind, .. } => {
                assert_eq!(kind, PassErrorKind::Cancelled);
            }
            _ => panic!("expected a typed pass-context failure"),
        }
    }
}