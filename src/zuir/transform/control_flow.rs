
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Control-Flow Transformation Pass
//!
//! Canonical path: `src/zuir/transform/control_flow.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Provide the pass-manager integration boundary for semantics-preserving
//! control-flow transformations over Zamani's Universal Intermediate
//! Representation (ZUIR).
//!
//! This module owns the transformation driver, its configuration, reporting,
//! errors, and adapter contract. The canonical IR continues to own blocks,
//! regions, operations, terminators, successor edges, and values.
//!
//! TRANSFORMATION POLICY
//!
//! Concrete rewrites may include branch simplification, redundant control-flow
//! elimination, block merging, unreachable-block removal, or other registered
//! transformations. Each rewrite must be justified by the applicable IR and
//! dialect contracts. This driver does not assume any of those rewrites is
//! universally legal.
//!
//! The adapter must preserve:
//! - control-flow successor targets and edge arguments;
//! - SSA definitions, uses, and dominance;
//! - types, ownership, and linear or affine constraints;
//! - observable effects, ordering, and concurrency semantics;
//! - quantum measurement and classical feed-forward;
//! - HDL timing, clock, and sequential behavior;
//! - nested-region and extension-defined control-flow semantics;
//! - source provenance and other required semantic metadata.
//!
//! UNKNOWN OPERATIONS
//!
//! An unknown operation or terminator must not be treated as pure,
//! removable, reorderable, or equivalent to a known operation. The adapter
//! must preserve it unless a trusted registered contract authorizes a rewrite.
//!
//! SCALABILITY
//!
//! This module introduces no fixed block, edge, operation, qubit, register,
//! tensor-rank, device, or node limits. A local rewrite budget is optional;
//! the pass manager's cancellation and execution budget remain authoritative.
//! No implementation can promise literal infinity or unlimited resources.
//!
//! ATOMICITY
//!
//! Each adapter rewrite must either commit one complete, valid transformation
//! or return without changing the IR. Earlier successful rewrites are not
//! rolled back if a later rewrite fails. Callers requiring whole-pass rollback
//! must supply a transaction or snapshot at the IR/pass-manager layer.
//!
//! OWNERSHIP
//!
//! This file does not implement the IR, CFG verifier, dominance analysis,
//! dialect registry, SSA repair, or concrete rewrite algorithms.
//!
//! INTEGRATION
//!
//! - `pass/manager.rs` owns pass orchestration, cancellation, and budgets.
//! - `ir/block.rs` owns block and terminator representation.
//! - `ir/successor.rs` owns explicit successor edges and edge arguments.
//! - `ir/region.rs` owns region membership and ownership.
//! - `verify/control_flow.rs` owns canonical CFG structural verification.
//! - `verify/full.rs` owns full-IR verification orchestration.
//! - Registered dialect contracts own extension-defined semantics.
//! - Grammar and semantic specifications own source-language meaning.

use core::fmt;
use std::marker::PhantomData;
use std::num::NonZeroUsize;

use crate::zuir::pass::manager::{
    Pass,
    PassContext,
    PassDescriptor,
    PassError,
    PassErrorKind,
    PassId,
    PassOutcome,
    PassVersion,
};

/// Stable namespaced identity of this pass.
pub const CONTROL_FLOW_PASS_ID: &str = "core.control_flow";

/// Version of the pass's public semantic contract.
///
/// Increment deliberately when the pass contract changes incompatibly.
pub const CONTROL_FLOW_PASS_VERSION: PassVersion =
    PassVersion::new(1, 0, 0);

/// Configuration for a single control-flow transformation invocation.
///
/// `max_rewrite_attempts` limits attempted rewrites, not IR size or hardware
/// capacity. `None` disables this pass-local limit; the pass manager may still
/// enforce time budgets and cancellation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlFlowConfig {
    /// Optional maximum number of adapter attempts, including the final
    /// attempt that establishes a fixed point.
    pub max_rewrite_attempts: Option<NonZeroUsize>,

    /// Reject input unless the adapter confirms that it is verified.
    pub require_verified_input: bool,

    /// Run the canonical verifier after every committed rewrite.
    ///
    /// The driver always verifies the final IR before reporting success.
    /// Setting this to false skips intermediate full-IR verification only;
    /// it does not waive the adapter's atomicity or semantic obligations.
    pub verify_after_each_rewrite: bool,
}

impl Default for ControlFlowConfig {
    fn default() -> Self {
        Self {
            max_rewrite_attempts: None,
            require_verified_input: true,
            verify_after_each_rewrite: true,
        }
    }
}

/// Outcome of one atomic control-flow rewrite attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFlowRewrite {
    /// No authorized rewrite remains in the adapter's defined search space.
    Unchanged,

    /// Exactly one semantics-preserving rewrite was committed.
    Changed,
}

/// Summary of a successful control-flow transformation run.
///
/// A successful report always represents a verified fixed point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlFlowReport {
    /// Number of adapter attempts, including the final unchanged attempt.
    pub attempts: usize,

    /// Number of successfully committed rewrites.
    pub rewrites: usize,

    /// Whether any IR mutation occurred.
    pub changed: bool,

    /// Whether the adapter reached a fixed point.
    ///
    /// This is true for every successfully returned report.
    pub reached_fixed_point: bool,

    /// Whether the final IR passed the adapter's canonical verifier.
    pub final_ir_verified: bool,
}

/// Errors returned by the control-flow transformation boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlFlowError {
    /// The input was not verified as required by configuration.
    UnverifiedInput,

    /// The configured attempt budget was exhausted before a fixed point.
    RewriteBudgetExceeded {
        attempts: usize,
        rewrites: usize,
    },

    /// A rewrite counter could not be incremented safely.
    CounterOverflow,

    /// The verifier rejected the input or a transformed IR.
    VerificationFailed(String),

    /// A proposed rewrite violated a structural or semantic invariant.
    InvalidRewrite(String),

    /// A required dialect or semantic contract was unavailable.
    SemanticContractUnavailable(String),

    /// A rewrite failed for another explicit reason.
    TransformationFailed(String),
}

impl fmt::Display for ControlFlowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnverifiedInput => {
                f.write_str(
                    "control-flow transformation requires verified input",
                )
            }
            Self::RewriteBudgetExceeded { attempts, rewrites } => write!(
                f,
                "control-flow rewrite budget exhausted after \
                 {attempts} attempts and {rewrites} rewrites; \
                 fixed point not established"
            ),
            Self::CounterOverflow => {
                f.write_str("control-flow transformation counter overflow")
            }
            Self::VerificationFailed(message) => {
                write!(f, "control-flow verification failed: {message}")
            }
            Self::InvalidRewrite(message) => {
                write!(f, "invalid control-flow rewrite: {message}")
            }
            Self::SemanticContractUnavailable(message) => {
                write!(
                    f,
                    "control-flow semantic contract unavailable: {message}"
                )
            }
            Self::TransformationFailed(message) => {
                write!(f, "control-flow transformation failed: {message}")
            }
        }
    }
}

impl std::error::Error for ControlFlowError {}

/// Contract implemented by an adapter over the canonical ZUIR representation.
///
/// The adapter is responsible for actual graph traversal and mutation. It
/// must consult the canonical IR, verifier, and registered dialect contracts;
/// operation names or attributes alone are not proof of semantic equivalence.
///
/// REQUIRED ADAPTER GUARANTEES
///
/// 1. Candidate selection and traversal are deterministic.
/// 2. One call commits at most one atomic rewrite.
/// 3. A failed rewrite leaves the IR unchanged for that attempt.
/// 4. `Unchanged` means no eligible rewrite remains in the adapter's declared
///    search space at the current IR state.
/// 5. Unknown or unsupported semantics are preserved conservatively.
/// 6. Every committed rewrite preserves source-level observable behavior.
/// 7. Verification uses the repository's authoritative verifier.
/// 8. The adapter does not retain stale CFG, dominance, or use-def analyses
///    across mutations unless it updates or invalidates them correctly.
///
/// This trait is a transformation boundary, not a replacement IR.
pub trait ControlFlowAdapter {
    /// True only when the current IR has passed the required verifier.
    fn is_verified(&self) -> bool;

    /// Attempt one deterministic, semantics-preserving control-flow rewrite.
    ///
    /// The adapter should check `context.check_budget()` during long-running
    /// searches, not only at rewrite boundaries.
    fn try_rewrite_one(
        &mut self,
        context: &mut PassContext,
    ) -> Result<ControlFlowRewrite, ControlFlowError>;

    /// Verify the complete IR through the canonical verifier.
    fn verify(&self) -> Result<(), ControlFlowError>;
}

/// Domain-neutral driver for control-flow transformations.
#[derive(Debug, Clone, Copy, Default)]
pub struct ControlFlowTransformer {
    config: ControlFlowConfig,
}

impl ControlFlowTransformer {
    /// Create a transformer with explicit configuration.
    #[must_use]
    pub const fn new(config: ControlFlowConfig) -> Self {
        Self { config }
    }

    /// Return the active configuration.
    #[must_use]
    pub const fn config(&self) -> &ControlFlowConfig {
        &self.config
    }

    /// Apply authorized rewrites until a verified fixed point is reached.
    ///
    /// The local attempt budget is checked before each adapter attempt.
    /// The pass-manager budget and cancellation token are checked throughout
    /// the loop. Reaching a budget returns an error; it is never reported as
    /// a fixed point.
    ///
    /// Earlier rewrites are not rolled back if a later attempt fails.
    /// The caller must therefore discard or re-verify the IR after an error.
    pub fn run<A: ControlFlowAdapter>(
        &self,
        ir: &mut A,
        context: &mut PassContext,
    ) -> Result<ControlFlowReport, ControlFlowError> {
        if self.config.require_verified_input && !ir.is_verified() {
            return Err(ControlFlowError::UnverifiedInput);
        }

        context.check_budget().map_err(|error| {
            ControlFlowError::TransformationFailed(error.to_string())
        })?;

        let mut attempts = 0usize;
        let mut rewrites = 0usize;

        loop {
            context.check_budget().map_err(|error| {
                ControlFlowError::TransformationFailed(error.to_string())
            })?;

            if let Some(limit) = self.config.max_rewrite_attempts {
                if attempts >= limit.get() {
                    return Err(ControlFlowError::RewriteBudgetExceeded {
                        attempts,
                        rewrites,
                    });
                }
            }

            attempts = attempts
                .checked_add(1)
                .ok_or(ControlFlowError::CounterOverflow)?;

            match ir.try_rewrite_one(context)? {
                ControlFlowRewrite::Unchanged => {
                    // Verify the final state even when intermediate checks
                    // are disabled. A fixed point is not evidence of validity.
                    ir.verify()?;

                    return Ok(ControlFlowReport {
                        attempts,
                        rewrites,
                        changed: rewrites != 0,
                        reached_fixed_point: true,
                        final_ir_verified: true,
                    });
                }

                ControlFlowRewrite::Changed => {
                    rewrites = rewrites
                        .checked_add(1)
                        .ok_or(ControlFlowError::CounterOverflow)?;

                    if self.config.verify_after_each_rewrite {
                        ir.verify()?;
                    }
                }
            }
        }
    }
}

/// Pass-manager adapter for a concrete control-flow adapter type.
///
/// `A` must be the actual IR type or a wrapper that provides access to the
/// canonical IR, dialect registry, analyses, and verifier. This generic
/// wrapper introduces no duplicate IR representation.
pub struct ControlFlowPass<A> {
    transformer: ControlFlowTransformer,
    adapter: PhantomData<fn() -> A>,
}

impl<A> ControlFlowPass<A> {
    /// Construct the pass with explicit rewrite and verification policy.
    #[must_use]
    pub const fn new(config: ControlFlowConfig) -> Self {
        Self {
            transformer: ControlFlowTransformer::new(config),
            adapter: PhantomData,
        }
    }

    /// Access the configured transformation driver.
    #[must_use]
    pub const fn transformer(&self) -> &ControlFlowTransformer {
        &self.transformer
    }
}

impl<A> Pass<A> for ControlFlowPass<A>
where
    A: ControlFlowAdapter + Send,
{
    fn descriptor(&self) -> PassDescriptor {
        // This is a compile-time built-in ID. Failure indicates a programming
        // error in this module, not malformed user input.
        let id = PassId::new(CONTROL_FLOW_PASS_ID)
            .expect("the built-in control-flow pass ID is valid");

        let mut descriptor = PassDescriptor::new(
            id,
            CONTROL_FLOW_PASS_VERSION,
            "Apply verified, semantics-preserving control-flow rewrites",
        );

        descriptor.requires_verified_input =
            self.transformer.config.require_verified_input;

        // The transformer always verifies its final IR before returning Ok.
        descriptor.verifies_output = true;
        descriptor.deterministic = true;

        // The manager invalidates analyses after a successful mutation unless
        // the pass explicitly proves that an analysis remains valid.
        descriptor.preserves_all_analyses = false;

        descriptor
    }

    fn run(
        &mut self,
        ir: &mut A,
        context: &mut PassContext,
    ) -> Result<PassOutcome, PassError> {
        let report = self
            .transformer
            .run(ir, context)
            .map_err(|error| {
                let kind = match &error {
                    ControlFlowError::UnverifiedInput
                    | ControlFlowError::VerificationFailed(_) => {
                        PassErrorKind::VerificationFailed
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

    #[test]
    fn default_configuration_is_unbounded_locally_but_verifies() {
        let config = ControlFlowConfig::default();

        assert_eq!(config.max_rewrite_attempts, None);
        assert!(config.require_verified_input);
        assert!(config.verify_after_each_rewrite);
    }

    #[test]
    fn explicit_rewrite_budget_is_nonzero() {
        let config = ControlFlowConfig {
            max_rewrite_attempts: NonZeroUsize::new(512),
            require_verified_input: true,
            verify_after_each_rewrite: true,
        };

        assert_eq!(
            config.max_rewrite_attempts.map(NonZeroUsize::get),
            Some(512)
        );
    }

    #[test]
    fn rewrite_outcomes_distinguish_mutation_from_fixed_point() {
        assert_ne!(
            ControlFlowRewrite::Changed,
            ControlFlowRewrite::Unchanged
        );
    }

    #[test]
    fn successful_report_records_verified_fixed_point() {
        let report = ControlFlowReport {
            attempts: 1,
            rewrites: 0,
            changed: false,
            reached_fixed_point: true,
            final_ir_verified: true,
        };

        assert!(report.reached_fixed_point);
        assert!(report.final_ir_verified);
        assert!(!report.changed);
        assert_eq!(report.rewrites, 0);
    }

    #[test]
    fn pass_identity_is_namespaced_and_versioned() {
        assert!(CONTROL_FLOW_PASS_ID.contains('.'));
        assert_eq!(CONTROL_FLOW_PASS_VERSION.major, 1);
    }
}
