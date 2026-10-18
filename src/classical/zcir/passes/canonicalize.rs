
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR canonicalization pass
//!
//! Path: `src/classical/zcir/passes/canonicalize.rs`
//!
//! STATUS
//!
//! Canonicalization policy and adapter contract. The concrete adapter must
//! be implemented against the authoritative IR operation/value APIs before
//! this module can be declared a complete ZCIR canonicalizer.
//!
//! PURPOSE
//!
//! Define deterministic, resource-aware, semantics-preserving canonicalization
//! for the ZCIR boundary without creating a second IR or pass-contract system.
//!
//! AUTHORITY
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `src/zuir/pass/contracts.rs`
//!
//! OWNS
//!
//! - Canonicalization configuration and operational budgets.
//! - The adapter contract for one canonicalization iteration.
//! - Fixed-point orchestration, progress accounting, and structured failures.
//! - The canonical ZCIR pass descriptor.
//!
//! DOES NOT OWN
//!
//! - IR operations, blocks, regions, values, types, or identities.
//! - Source-language semantic rules or semantic analysis.
//! - A competing pass manager or duplicate pass descriptor types.
//! - Target selection, physical resource allocation, scheduling, or lowering.
//! - Canonical serialization or persistent artifact encoding.
//! - Proof of semantic preservation for an adapter's rewrite rules.
//!
//! INVARIANTS
//!
//! - An adapter must preserve all required observable program semantics.
//! - Unknown or unsupported operations must be preserved or rejected explicitly.
//! - Canonicalization must not silently lower to a hardware-specific form.
//! - Deterministic mode must not depend on hash iteration order, addresses,
//!   thread scheduling, or machine-specific incidental properties.
//! - A budget failure must not leave a partially applied iteration.
//! - No universal maximum IR size, operation count, or iteration count is
//!   introduced. Budgets are optional operational policies.
//!
//! INTEGRATION
//!
//! `passes/mod.rs` should declare `pub mod canonicalize;`.
//! The pass manager should consume the canonical contracts from
//! `crate::zuir::pass::contracts`, not legacy duplicate contract types.
//! The actual IR adapter must be implemented after inspecting the canonical
//! ZUIR/ZCIR representation APIs.
//!
//! SAFETY
//!
//! Unsafe Rust is forbidden.

use core::fmt;
use std::error::Error;

use crate::zuir::pass::contracts::{
    PassDescriptor, PassError, PassId, PassVersion,
};

/// Stable identity of this pass in the canonical pass registry.
pub const CANONICALIZE_PASS_ID: &str = "classical.zcir.canonicalize";

/// Version of the canonicalization contract.
///
/// Increment the major version when canonicalization's externally observable
/// contract or compatibility guarantees change incompatibly. A rewrite-rule
/// change that can alter canonical output may require a version change under
/// the repository's artifact compatibility policy.
pub const CANONICALIZE_PASS_VERSION: PassVersion = PassVersion::new(1, 0, 0);

/// Policy for canonicalization execution.
///
/// `None` means that this layer imposes no explicit budget for that dimension.
/// It does not mean that memory, time, address spaces, or execution are
/// physically infinite. When no iteration budget is supplied, the adapter
/// must guarantee termination for every accepted input.
///
/// Operational budgets must not be treated as source-language semantic limits.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CanonicalizationConfig {
    /// Maximum number of fixed-point iterations.
    ///
    /// `None` delegates termination to the adapter's proven termination
    /// strategy. `Some(0)` is invalid.
    pub max_iterations: Option<u64>,

    /// Maximum total number of rewrites reported by the adapter.
    ///
    /// `None` means no explicit rewrite-count budget at this layer.
    /// `Some(0)` is valid only if the input is already canonical and the
    /// adapter can establish that without performing a rewrite.
    pub max_rewrite_steps: Option<u64>,

    /// Require repeatable results for equivalent inputs under equivalent
    /// configurations and adapter versions.
    ///
    /// The adapter must reject or avoid nondeterministic rewrite strategies
    /// when this is enabled.
    pub require_determinism: bool,

    /// Stop as soon as the adapter reports a fixed point.
    ///
    /// This is normally true. When false, the driver still stops on a fixed
    /// point because continuing would violate the fixed-point contract.
    /// The field is retained for API clarity and future policy evolution.
    pub stop_at_fixed_point: bool,
}

impl CanonicalizationConfig {
    /// Production-oriented default: deterministic execution, no arbitrary
    /// universal iteration/rewrite ceiling, and stop at a fixed point.
    ///
    /// The adapter remains responsible for guaranteeing termination when
    /// no explicit iteration budget is configured.
    #[must_use]
    pub const fn portable_default() -> Self {
        Self {
            max_iterations: None,
            max_rewrite_steps: None,
            require_determinism: true,
            stop_at_fixed_point: true,
        }
    }

    /// Validate operational policy before mutating IR.
    pub fn validate(&self) -> Result<(), CanonicalizationError> {
        if self.max_iterations == Some(0) {
            return Err(CanonicalizationError::invalid_configuration(
                "max_iterations must be greater than zero when configured",
            ));
        }

        if !self.stop_at_fixed_point {
            return Err(CanonicalizationError::invalid_configuration(
                "stop_at_fixed_point must be enabled; canonicalization must \
                 stop when the adapter reports no changes",
            ));
        }

        Ok(())
    }
}

/// Remaining operational budget passed to an adapter iteration.
///
/// Adapters must treat these values as operational limits, not language
/// semantics. They must check a configured rewrite budget before committing
/// changes, so a rejected iteration cannot leave partially rewritten IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IterationBudget {
    /// Remaining rewrites allowed, or `None` when unlimited by this policy.
    pub remaining_rewrite_steps: Option<u64>,

    /// One-based iteration number about to execute.
    pub iteration: u64,

    /// Whether deterministic behavior is required.
    pub require_determinism: bool,
}

/// Summary of one successfully committed canonicalization iteration.
///
/// `rewrite_count` counts actual committed rewrite applications, not merely
/// candidates examined. An adapter must report `changed == false` only when
/// it has reached a fixed point under the configured canonicalization rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IterationSummary {
    /// Whether this iteration changed the IR.
    pub changed: bool,

    /// Number of rewrite applications committed by this iteration.
    pub rewrite_count: u64,
}

/// Aggregate result of canonicalizing one IR value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalizationReport {
    /// Number of completed iterations, including the final fixed-point check.
    pub iterations: u64,

    /// Total number of committed rewrites.
    pub rewrite_count: u64,

    /// `true` only when the adapter reported a fixed point.
    pub reached_fixed_point: bool,
}

/// Structured canonicalization failure category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalizationErrorKind {
    InvalidConfiguration,
    AdapterFailure,
    NondeterministicAdapter,
    IterationBudgetExceeded,
    RewriteBudgetExceeded,
    CounterOverflow,
    InvalidAdapterReport,
}

/// Canonicalization failure.
///
/// Diagnostic text is explanatory, not a stable machine-readable identifier.
/// Callers should branch on `kind`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalizationError {
    pub kind: CanonicalizationErrorKind,
    pub message: String,
}

impl CanonicalizationError {
    #[must_use]
    pub fn new(
        kind: CanonicalizationErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    #[must_use]
    pub fn invalid_configuration(message: impl Into<String>) -> Self {
        Self::new(
            CanonicalizationErrorKind::InvalidConfiguration,
            message,
        )
    }
}

impl fmt::Display for CanonicalizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl Error for CanonicalizationError {}

/// Adapter implemented by the canonical IR owner.
///
/// This trait intentionally does not prescribe a new ZCIR operation model.
/// An implementation must use the repository's authoritative IR data types
/// and operation registry.
///
/// # Required adapter guarantees
///
/// 1. Every committed rewrite preserves observable semantics.
/// 2. Rewrite selection and traversal are deterministic when requested.
/// 3. Unknown operations are preserved or cause an explicit error.
/// 4. A failed iteration leaves the IR unchanged.
/// 5. A rewrite budget is checked before changes are committed.
/// 6. `changed == false` means no permitted canonicalization rewrite remains.
/// 7. If no iteration budget is supplied, termination is guaranteed for
///    accepted input. A pass must not loop indefinitely.
///
/// The driver checks reported counters and fixed-point behavior, but cannot
/// prove that an adapter's semantic claims are correct.
pub trait CanonicalizationAdapter {
    /// Perform exactly one canonicalization iteration transactionally.
    ///
    /// Implementations should gather candidate rewrites, validate legality,
    /// check the supplied budget, and commit the iteration atomically.
    fn canonicalize_iteration(
        &mut self,
        budget: IterationBudget,
    ) -> Result<IterationSummary, CanonicalizationError>;
}

/// Construct the pass descriptor using the canonical ZUIR contracts.
///
/// This function deliberately returns the canonical `PassError` rather than
/// defining a local descriptor/error type.
pub fn canonicalization_descriptor() -> Result<PassDescriptor, PassError> {
    let id = PassId::new(CANONICALIZE_PASS_ID)?;

    let descriptor = PassDescriptor::new(
        id,
        CANONICALIZE_PASS_VERSION,
        "Canonicalize ZCIR without changing program semantics",
    );

    descriptor.validate()?;
    Ok(descriptor)
}

/// Run canonicalization to a fixed point.
///
/// The input is mutated only by successful adapter iterations. The adapter
/// contract requires each iteration to be transactional; the driver cannot
/// roll back arbitrary external mutations performed by a nonconforming
/// adapter.
///
/// # Errors
///
/// Returns an error for invalid policy, adapter failure, a reported budget
/// violation, counter overflow, or an invalid adapter report.
///
/// A configured budget exhaustion is an operational failure, not evidence
/// that the source program is invalid or that the target is infeasible.
pub fn canonicalize<A: CanonicalizationAdapter>(
    adapter: &mut A,
    config: &CanonicalizationConfig,
) -> Result<CanonicalizationReport, CanonicalizationError> {
    config.validate()?;

    let mut iterations = 0_u64;
    let mut rewrite_count = 0_u64;

    loop {
        if let Some(limit) = config.max_iterations {
            if iterations >= limit {
                return Err(CanonicalizationError::new(
                    CanonicalizationErrorKind::IterationBudgetExceeded,
                    format!(
                        "canonicalization did not reach a fixed point within \
                         the configured iteration budget ({limit})"
                    ),
                ));
            }
        }

        let next_iteration = iterations.checked_add(1).ok_or_else(|| {
            CanonicalizationError::new(
                CanonicalizationErrorKind::CounterOverflow,
                "canonicalization iteration counter overflowed",
            )
        })?;

        let remaining_rewrites = match config.max_rewrite_steps {
            Some(limit) => Some(limit.checked_sub(rewrite_count).ok_or_else(|| {
                CanonicalizationError::new(
                    CanonicalizationErrorKind::RewriteBudgetExceeded,
                    "rewrite budget was exhausted before the next iteration",
                )
            })?),
            None => None,
        };

        let summary = adapter.canonicalize_iteration(IterationBudget {
            remaining_rewrite_steps: remaining_rewrites,
            iteration: next_iteration,
            require_determinism: config.require_determinism,
        })?;

        if !summary.changed && summary.rewrite_count != 0 {
            return Err(CanonicalizationError::new(
                CanonicalizationErrorKind::InvalidAdapterReport,
                "adapter reported rewrites while reporting that the IR was unchanged",
            ));
        }

        if summary.changed && summary.rewrite_count == 0 {
            return Err(CanonicalizationError::new(
                CanonicalizationErrorKind::InvalidAdapterReport,
                "adapter reported an IR change without reporting any committed rewrites",
            ));
        }

        let next_rewrite_count =
            rewrite_count.checked_add(summary.rewrite_count).ok_or_else(|| {
                CanonicalizationError::new(
                    CanonicalizationErrorKind::CounterOverflow,
                    "canonicalization rewrite counter overflowed",
                )
            })?;

        if let Some(limit) = config.max_rewrite_steps {
            if next_rewrite_count > limit {
                return Err(CanonicalizationError::new(
                    CanonicalizationErrorKind::RewriteBudgetExceeded,
                    format!(
                        "adapter reported {next_rewrite_count} rewrites, exceeding \
                         the configured budget ({limit})"
                    ),
                ));
            }
        }

        iterations = next_iteration;
        rewrite_count = next_rewrite_count;

        if !summary.changed {
            return Ok(CanonicalizationReport {
                iterations,
                rewrite_count,
                reached_fixed_point: true,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct MockAdapter {
        /// Number of rewrite iterations still to perform.
        changes_remaining: u64,
        /// Total changes observed by the mock.
        committed: u64,
    }

    impl CanonicalizationAdapter for MockAdapter {
        fn canonicalize_iteration(
            &mut self,
            budget: IterationBudget,
        ) -> Result<IterationSummary, CanonicalizationError> {
            if budget.require_determinism && budget.iteration == 0 {
                return Err(CanonicalizationError::new(
                    CanonicalizationErrorKind::NondeterministicAdapter,
                    "invalid iteration supplied by driver",
                ));
            }

            if self.changes_remaining == 0 {
                return Ok(IterationSummary::default());
            }

            if budget.remaining_rewrite_steps == Some(0) {
                return Err(CanonicalizationError::new(
                    CanonicalizationErrorKind::RewriteBudgetExceeded,
                    "mock refuses to commit beyond the rewrite budget",
                ));
            }

            self.changes_remaining -= 1;
            self.committed += 1;

            Ok(IterationSummary {
                changed: true,
                rewrite_count: 1,
            })
        }
    }

    #[test]
    fn descriptor_uses_canonical_pass_contracts() {
        let descriptor = canonicalization_descriptor().expect("valid descriptor");
        assert_eq!(descriptor.id.as_str(), CANONICALIZE_PASS_ID);
        assert!(descriptor.deterministic);
        assert!(descriptor.requires_verified_input);
        assert!(descriptor.verifies_output);
    }

    #[test]
    fn default_policy_has_no_arbitrary_iteration_ceiling() {
        let config = CanonicalizationConfig::portable_default();
        assert_eq!(config.max_iterations, None);
        assert_eq!(config.max_rewrite_steps, None);
        assert!(config.require_determinism);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn zero_iteration_budget_is_rejected_before_mutation() {
        let config = CanonicalizationConfig {
            max_iterations: Some(0),
            ..CanonicalizationConfig::portable_default()
        };
        let mut adapter = MockAdapter {
            changes_remaining: 2,
            committed: 0,
        };

        let result = canonicalize(&mut adapter, &config);

        assert_eq!(
            result.expect_err("zero budget must fail").kind,
            CanonicalizationErrorKind::InvalidConfiguration
        );
        assert_eq!(adapter.committed, 0);
    }

    #[test]
    fn canonicalization_stops_at_fixed_point() {
        let mut adapter = MockAdapter {
            changes_remaining: 3,
            committed: 0,
        };

        let report = canonicalize(
            &mut adapter,
            &CanonicalizationConfig::portable_default(),
        )
        .expect("mock should reach a fixed point");

        assert_eq!(report.iterations, 4);
        assert_eq!(report.rewrite_count, 3);
        assert!(report.reached_fixed_point);
    }

    #[test]
    fn iteration_budget_exhaustion_is_explicit() {
        let mut adapter = MockAdapter {
            changes_remaining: 3,
            committed: 0,
        };
        let config = CanonicalizationConfig {
            max_iterations: Some(2),
            ..CanonicalizationConfig::portable_default()
        };

        let error = canonicalize(&mut adapter, &config)
            .expect_err("two iterations cannot reach the fixed point");

        assert_eq!(
            error.kind,
            CanonicalizationErrorKind::IterationBudgetExceeded
        );
        assert_eq!(adapter.committed, 2);
    }

    #[test]
    fn adapter_must_refuse_a_rewrite_when_no_budget_remains() {
        let mut adapter = MockAdapter {
            changes_remaining: 1,
            committed: 0,
        };
        let config = CanonicalizationConfig {
            max_rewrite_steps: Some(0),
            ..CanonicalizationConfig::portable_default()
        };

        let error = canonicalize(&mut adapter, &config)
            .expect_err("adapter must refuse an over-budget rewrite");

        assert_eq!(
            error.kind,
            CanonicalizationErrorKind::RewriteBudgetExceeded
        );
        assert_eq!(adapter.committed, 0);
    }

    #[test]
    fn inconsistent_adapter_report_is_rejected() {
        struct InvalidAdapter;

        impl CanonicalizationAdapter for InvalidAdapter {
            fn canonicalize_iteration(
                &mut self,
                _: IterationBudget,
            ) -> Result<IterationSummary, CanonicalizationError> {
                Ok(IterationSummary {
                    changed: false,
                    rewrite_count: 1,
                })
            }
        }

        let error = canonicalize(
            &mut InvalidAdapter,
            &CanonicalizationConfig::portable_default(),
        )
        .expect_err("inconsistent report must fail");

        assert_eq!(
            error.kind,
            CanonicalizationErrorKind::InvalidAdapterReport
        );
    }
}
