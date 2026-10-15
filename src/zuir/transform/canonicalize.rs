//! # ZUIR Canonicalization Pass
//!
//! Path: `src/zuir/transform/canonicalize.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Provide the pass-manager integration boundary for semantics-preserving
//! canonicalization of Zamani's Universal Intermediate Representation.
//!
//! Canonicalization may normalize representations only when equivalence is
//! established by the IR specification and the applicable dialect contract.
//!
//! This module deliberately does not assume that operations are pure,
//! commutative, removable, or reorderable merely because their names or
//! attributes look familiar.
//!
//! AUTHORITY
//!
//! - src/zuir/ZUIR-SPEC.md
//! - src/zuir/pass/manager.rs
//! - src/zuir/ir/operation.rs
//! - src/zuir/ir/block.rs
//! - src/zuir/ir/effects.rs
//! - src/zuir/dialect/operation.rs, when its canonical API is available
//! - grammar/DESIGN.md
//! - grammar/specification/semantic-model.md
//! - grammar/specification/compilation-model.md
//! - grammar/specification/execution-model.md
//! - grammar/specification/extensibility.md, if present
//!
//! OWNERSHIP
//!
//! This module owns the canonicalization pass identity, configuration,
//! rewrite protocol, and pass-manager adapter.
//!
//! The IR owns entity representation and mutation.
//! Dialects own operation-specific semantic equivalence.
//! The verifier owns authoritative IR validity checks.
//! The pass manager owns execution orchestration, budgets, cancellation,
//! and analysis invalidation.
//!
//! SCALABILITY
//!
//! This module introduces no machine-size, qubit-count, operation-count,
//! tensor-rank, register-width, or device-count ceiling.
//!
//! Work is bounded by the supplied pass context, configured rewrite budget,
//! and actual system resources. Literal infinity is not promised.
//!
//! SEMANTIC GUARANTEES
//!
//! - Unknown operations are not rewritten by this pass.
//! - Effectful operations are not reordered by this pass.
//! - Quantum measurement and other observable effects are not discarded.
//! - Hardware timing and ordering are preserved unless an authorized
//!   dialect transformation explicitly proves equivalence.
//! - A rewrite is committed only after the adapter accepts it.
//! - A failed or interrupted transformation must not be treated as verified.
//!
//! INTEGRATION
//!
//! The adapter is the boundary between this pass and the canonical ZUIR
//! representation. Implement it against the repository's finalized program,
//! module, region, block, operation, dialect-registry, and verifier APIs.
//!
//! This avoids duplicating IR structures or inventing a second operation
//! model inside the transformation layer.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::num::NonZeroUsize;

use crate::zuir::pass::manager::{
    Pass,
    PassContext,
    PassDescriptor,
    PassError,
    PassErrorKind,
    PassOutcome,
    PassVersion,
};

/// Stable identifier for this transformation.
pub const CANONICALIZE_PASS_ID: &str = "core.canonicalize";

/// Version of the canonicalization algorithm.
///
/// Increment the major version when its semantic contract changes
/// incompatibly. Do not derive this value from build timestamps or
/// registration order.
pub const CANONICALIZE_PASS_VERSION: PassVersion =
    PassVersion::new(1, 0, 0);

/// Maximum rewrite attempts for one invocation.
///
/// This is an operational budget, not an IR-size or hardware limit.
/// Reaching the budget is reported explicitly rather than silently
/// claiming that canonicalization reached a fixed point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalizeConfig {
    pub max_rewrite_attempts: Option<NonZeroUsize>,
    pub require_verified_input: bool,
    pub verify_after_rewrite: bool,
}

impl Default for CanonicalizeConfig {
    fn default() -> Self {
        Self {
            max_rewrite_attempts: None,
            require_verified_input: true,
            verify_after_rewrite: true,
        }
    }
}

/// Result of one canonicalization attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewriteResult {
    /// No authorized rewrite applies.
    Unchanged,

    /// A semantics-preserving rewrite was committed.
    Changed,
}

/// Result of a complete canonicalization invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalizeReport {
    /// Number of rewrite attempts made.
    pub attempts: usize,

    /// Number of successfully committed rewrites.
    pub rewrites: usize,

    /// Whether any IR mutation occurred.
    pub changed: bool,

    /// Whether the adapter established a fixed point.
    pub reached_fixed_point: bool,
}

/// Errors produced by the canonicalization boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalizeError {
    /// A rewrite would violate an IR or dialect invariant.
    InvalidRewrite(String),

    /// A rewrite budget was exhausted before reaching a fixed point.
    RewriteBudgetExceeded {
        attempts: usize,
        rewrites: usize,
    },

    /// The adapter could not establish the required semantic guarantees.
    SemanticContractViolation(String),

    /// The adapter rejected malformed input.
    InvalidInput(String),

    /// The transformation could not be completed safely.
    TransformationFailed(String),
}

impl std::fmt::Display for CanonicalizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRewrite(message) => {
                write!(f, "invalid canonicalization rewrite: {message}")
            }
            Self::RewriteBudgetExceeded { attempts, rewrites } => {
                write!(
                    f,
                    "canonicalization rewrite budget exhausted \
                     after {attempts} attempts and {rewrites} rewrites"
                )
            }
            Self::SemanticContractViolation(message) => {
                write!(
                    f,
                    "canonicalization semantic contract violation: {message}"
                )
            }
            Self::InvalidInput(message) => {
                write!(f, "invalid canonicalization input: {message}")
            }
            Self::TransformationFailed(message) => {
                write!(f, "canonicalization failed: {message}")
            }
        }
    }
}

impl std::error::Error for CanonicalizeError {}

/// Adapter implemented by the canonical IR owner.
///
/// This interface deliberately separates the transformation algorithm
/// from the concrete storage representation. Its implementation must use
/// registered dialect contracts and the canonical verifier.
///
/// IMPORTANT:
///
/// An adapter must not implement `try_canonicalize_one` by blindly sorting
/// operations, deleting unknown operations, or assuming that an operation
/// without a familiar effect marker is pure.
///
/// The adapter must also define whether a failed rewrite can leave mutations
/// behind. The required contract is that a failed attempt does not leave a
/// partially committed rewrite.
pub trait CanonicalizationAdapter {
    /// Return true only when the current IR has passed the authoritative
    /// verification required by the caller's policy.
    fn is_verified(&self) -> bool;

    /// Attempt one semantics-preserving canonical rewrite.
    ///
    /// Implementations must use deterministic candidate selection.
    /// `Unchanged` means no eligible rewrite remains at the current state.
    /// `Changed` means exactly one atomic rewrite was committed.
    fn try_canonicalize_one(
        &mut self,
        context: &mut PassContext,
    ) -> Result<RewriteResult, CanonicalizeError>;

    /// Verify the entire IR using the canonical verifier.
    ///
    /// This must check all applicable structural, type, control-flow,
    /// use-def, effect, ownership, resource, and dialect invariants.
    fn verify(&self) -> Result<(), CanonicalizeError>;
}

/// Canonicalization driver independent of the concrete IR representation.
#[derive(Debug, Clone)]
pub struct Canonicalizer {
    config: CanonicalizeConfig,
}

impl Canonicalizer {
    pub fn new(config: CanonicalizeConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &CanonicalizeConfig {
        &self.config
    }

    /// Canonicalize until no authorized rewrite applies or an explicit
    /// failure condition is encountered.
    ///
    /// A rewrite budget limits work; it does not truncate the program or
    /// change its semantics. If the budget is exhausted before a fixed point
    /// is established, this method returns an error.
    pub fn run<A: CanonicalizationAdapter>(
        &self,
        ir: &mut A,
        context: &mut PassContext,
    ) -> Result<CanonicalizeReport, CanonicalizeError> {
        if self.config.require_verified_input && !ir.is_verified() {
            return Err(CanonicalizeError::InvalidInput(
                "canonicalization requires verified input".into(),
            ));
        }

        context
            .check_budget()
            .map_err(|error| {
                CanonicalizeError::TransformationFailed(error.to_string())
            })?;

        let mut attempts = 0usize;
        let mut rewrites = 0usize;

        loop {
            context
                .check_budget()
                .map_err(|error| {
                    CanonicalizeError::TransformationFailed(error.to_string())
                })?;

            if let Some(limit) = self.config.max_rewrite_attempts {
                if attempts >= limit.get() {
                    return Err(CanonicalizeError::RewriteBudgetExceeded {
                        attempts,
                        rewrites,
                    });
                }
            }

            attempts = attempts.checked_add(1).ok_or_else(|| {
                CanonicalizeError::TransformationFailed(
                    "rewrite-attempt counter overflow".into(),
                )
            })?;

            match ir.try_canonicalize_one(context)? {
                RewriteResult::Unchanged => {
                    return Ok(CanonicalizeReport {
                        attempts,
                        rewrites,
                        changed: rewrites != 0,
                        reached_fixed_point: true,
                    });
                }

                RewriteResult::Changed => {
                    rewrites = rewrites.checked_add(1).ok_or_else(|| {
                        CanonicalizeError::TransformationFailed(
                            "rewrite counter overflow".into(),
                        )
                    })?;

                    // Revalidate after each committed rewrite. This is
                    // intentionally conservative; a later optimization may
                    // batch rewrites only after the verifier and adapter
                    // contracts establish an equivalent safety boundary.
                    if self.config.verify_after_rewrite {
                        ir.verify()?;
                    }
                }
            }
        }
    }
}

/// Pass-manager integration for a concrete canonicalization adapter.
///
/// `A` must be the actual canonical IR type or a verified wrapper around it.
/// No separate IR model is introduced by this pass.
pub struct CanonicalizePass<A> {
    canonicalizer: Canonicalizer,
    adapter: std::marker::PhantomData<fn() -> A>,
}

impl<A> CanonicalizePass<A> {
    pub fn new(config: CanonicalizeConfig) -> Self {
        Self {
            canonicalizer: Canonicalizer::new(config),
            adapter: std::marker::PhantomData,
        }
    }

    pub fn canonicalizer(&self) -> &Canonicalizer {
        &self.canonicalizer
    }
}

impl<A> Pass<A> for CanonicalizePass<A>
where
    A: CanonicalizationAdapter + Send,
{
    fn descriptor(&self) -> PassDescriptor {
        let id = crate::zuir::pass::manager::PassId::new(
            CANONICALIZE_PASS_ID,
        )
        .expect("the built-in canonicalization pass ID is valid");

        let mut descriptor = PassDescriptor::new(
            id,
            CANONICALIZE_PASS_VERSION,
            "Apply deterministic, semantics-preserving ZUIR canonicalization",
        );

        descriptor.requires_verified_input =
            self.canonicalizer.config.require_verified_input;
        descriptor.verifies_output =
            self.canonicalizer.config.verify_after_rewrite;
        descriptor.deterministic = true;
        descriptor
    }

    fn run(
        &mut self,
        ir: &mut A,
        context: &mut PassContext,
    ) -> Result<PassOutcome, PassError> {
        let report = self
            .canonicalizer
            .run(ir, context)
            .map_err(|error| {
                PassError::new(
                    PassErrorKind::PassFailed,
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
        let config = CanonicalizeConfig::default();

        assert_eq!(config.max_rewrite_attempts, None);
        assert!(config.require_verified_input);
        assert!(config.verify_after_rewrite);
    }

    #[test]
    fn rewrite_budget_is_explicit_and_nonzero() {
        let config = CanonicalizeConfig {
            max_rewrite_attempts: NonZeroUsize::new(128),
            require_verified_input: true,
            verify_after_rewrite: true,
        };

        assert_eq!(
            config.max_rewrite_attempts.map(NonZeroUsize::get),
            Some(128)
        );
    }

    #[test]
    fn rewrite_results_distinguish_mutation_from_fixed_point() {
        assert_ne!(RewriteResult::Changed, RewriteResult::Unchanged);
    }

    #[test]
    fn pass_identity_is_namespaced_and_versioned() {
        assert!(CANONICALIZE_PASS_ID.contains('.'));
        assert_eq!(CANONICALIZE_PASS_VERSION.major, 1);
    }
}