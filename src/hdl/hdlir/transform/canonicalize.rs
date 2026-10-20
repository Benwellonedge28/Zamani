
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Canonicalization
//!
//! Path: `src/hdl/hdlir/transform/canonicalize.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//! Status: Canonicalization contract and execution foundation.
//!
//! ## Purpose
//!
//! Define the checked execution contract for deterministic, semantics-preserving
//! canonicalization of Zamani's hardware-domain intermediate representation.
//!
//! This module deliberately does not invent design, operation, value, block,
//! region, or dialect node types. Those types belong to the canonical HDLIR
//! graph under `src/hdl/hdlir/ir/` and must be integrated after their actual
//! contracts have been established.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/config.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/passes/contracts.rs`
//! - `src/hdl/hdlir/passes/manager.rs`
//! - `src/hdl/hdlir/passes/preservation.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The HDLIR specification and the actual IR graph are authoritative for
//! which fields may be reordered or normalized. This file must not infer
//! semantic equivalence from structural similarity.
//!
//! ## Owns
//!
//! - The canonicalization execution contract.
//! - Explicit work-budget accounting.
//! - Deterministic, structured execution outcomes.
//! - A trait boundary for canonicalization of the actual HDLIR graph.
//!
//! ## Does not own
//!
//! - HDLIR graph, operation, type, value, or identity definitions.
//! - Source parsing, name resolution, or semantic validation.
//! - Operation-schema registration or dialect semantics.
//! - Hardware scheduling, placement, routing, synthesis, or allocation.
//! - A second pass manager or a substitute for preservation checking.
//! - A claim of formal equivalence without actual supporting evidence.
//!
//! ## Semantic safety
//!
//! Canonicalization may normalize representation only when the HDLIR
//! specification and registered operation semantics establish that the
//! transformation preserves observable behavior.
//!
//! In particular, an implementation must not blindly sort:
//!
//! - Ordered operations in a block.
//! - Clocked or stateful processes.
//! - Memory accesses whose ordering is observable.
//! - Side-effecting operations.
//! - Connections, ports, or declarations whose order is semantically defined.
//! - Unknown operations whose ordering or effects are not understood.
//!
//! Stable ordering is appropriate only for collections whose contract
//! explicitly defines ordering as semantically irrelevant.
//!
//! ## Scalability
//!
//! No hardware-capacity maximum is defined here. Work limits are operational
//! safeguards, not HDL language limits. A budget-exhausted transformation
//! returns an error and must not be reported as completed.
//!
//! ## Integration
//!
//! Once the core IR API exists, implement `Canonicalizable` for its canonical
//! design type. The implementation must use actual schema and IR contracts,
//! consume work budget during expensive traversal and comparison, preserve
//! semantic ordering, and return accurate change counts.
//!
//! The pass manager must use the repository's existing pass contracts and
//! preservation checker. It must not treat this module's report as proof of
//! semantic equivalence.
//!
//! ## Completion criteria
//!
//! - The actual IR type implements the contract.
//! - All canonicalization rules are specified and semantics-aware.
//! - Budget exhaustion is explicit and cannot become success.
//! - Determinism and idempotence are tested against real HDLIR designs.
//! - Preservation obligations are evaluated by the existing pass framework.
//! - Rust formatting, checking, tests, and Clippy pass.

use core::fmt;
use core::num::NonZeroU64;

/// Operational limits for a single canonicalization invocation.
///
/// `max_work = None` means that this layer imposes no work-unit limit.
/// It does not promise infinite execution or remove operating-system,
/// allocator, host, or downstream operational constraints.
///
/// Work units are abstract, deterministic units defined by the concrete
/// `Canonicalizable` implementation. An implementation must document how
/// it charges traversal, comparisons, or other expensive operations.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CanonicalizationBudget {
    max_work: Option<NonZeroU64>,
}

impl CanonicalizationBudget {
    /// Construct an unlimited work-unit budget.
    ///
    /// Unlimited here means no limit enforced by this budget object.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self { max_work: None }
    }

    /// Construct a budget with a nonzero maximum number of work units.
    #[must_use]
    pub const fn limited(max_work: NonZeroU64) -> Self {
        Self {
            max_work: Some(max_work),
        }
    }

    /// Return the configured maximum, or `None` when unlimited.
    #[must_use]
    pub const fn max_work(self) -> Option<NonZeroU64> {
        self.max_work
    }
}

/// The outcome counters for a completed canonicalization invocation.
///
/// Counts are descriptive metrics, not semantic proof. A zero change count
/// means the implementation reports no changes; it does not establish that
/// the input was valid or that every preservation property was verified.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CanonicalizationReport {
    /// Number of canonicalization work units consumed.
    pub work_used: u64,

    /// Number of IR entities examined under the implementation's counting
    /// contract.
    pub entities_visited: u64,

    /// Number of entities or fields actually changed.
    ///
    /// The concrete implementation must document what counts as a change.
    pub changes_made: u64,
}

impl CanonicalizationReport {
    /// Create an empty report.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            work_used: 0,
            entities_visited: 0,
            changes_made: 0,
        }
    }
}

/// Failure to execute or complete canonicalization.
///
/// Errors deliberately distinguish operational exhaustion from arithmetic
/// overflow and invalid implementation accounting. They must not be converted
/// into a successful, unchanged result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalizationError {
    /// The invocation would exceed its configured work budget.
    WorkBudgetExceeded {
        /// Work already charged to this invocation.
        work_used: u64,

        /// The additional work requested.
        requested: u64,

        /// The configured maximum work.
        limit: u64,
    },

    /// Work accounting overflowed the representable counter.
    WorkCounterOverflow,

    /// An entity counter overflowed.
    EntityCounterOverflow,

    /// A change counter overflowed.
    ChangeCounterOverflow,

    /// An implementation reported metrics inconsistent with its context.
    InvalidReport {
        /// Work units reported by the implementation.
        reported_work: u64,

        /// Work units recorded by the context.
        accounted_work: u64,
    },

    /// The implementation detected invalid input or an internal invariant
    /// violation that prevents safe canonicalization.
    InvalidInput {
        /// Stable diagnostic code defined by the owning implementation.
        code: &'static str,
    },
}

impl fmt::Display for CanonicalizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WorkBudgetExceeded {
                work_used,
                requested,
                limit,
            } => write!(
                f,
                "HDLIR canonicalization work budget exceeded: \
                 used {work_used}, requested {requested}, limit {limit}"
            ),
            Self::WorkCounterOverflow => {
                f.write_str("HDLIR canonicalization work counter overflow")
            }
            Self::EntityCounterOverflow => {
                f.write_str("HDLIR canonicalization entity counter overflow")
            }
            Self::ChangeCounterOverflow => {
                f.write_str("HDLIR canonicalization change counter overflow")
            }
            Self::InvalidReport {
                reported_work,
                accounted_work,
            } => write!(
                f,
                "invalid HDLIR canonicalization report: \
                 reported work {reported_work}, accounted work {accounted_work}"
            ),
            Self::InvalidInput { code } => {
                write!(f, "invalid HDLIR canonicalization input ({code})")
            }
        }
    }
}

impl std::error::Error for CanonicalizationError {}

/// Per-invocation state for checked canonicalization.
///
/// Contexts are independent and contain no global state. Create one context
/// per invocation; do not share it across concurrent passes.
///
/// Concrete implementations must charge work before undertaking the
/// corresponding bounded unit of work. A budget is useful only when all
/// expensive loops and recursive or graph traversals honor it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalizationContext {
    budget: CanonicalizationBudget,
    work_used: u64,
    entities_visited: u64,
    changes_made: u64,
}

impl CanonicalizationContext {
    /// Start a new invocation with the supplied budget.
    #[must_use]
    pub const fn new(budget: CanonicalizationBudget) -> Self {
        Self {
            budget,
            work_used: 0,
            entities_visited: 0,
            changes_made: 0,
        }
    }

    /// Return the budget configured for this invocation.
    #[must_use]
    pub const fn budget(&self) -> CanonicalizationBudget {
        self.budget
    }

    /// Return work units charged so far.
    #[must_use]
    pub const fn work_used(&self) -> u64 {
        self.work_used
    }

    /// Charge work before performing the corresponding operation.
    ///
    /// A zero charge is valid and does not advance the counter.
    ///
    /// # Errors
    ///
    /// Returns `WorkBudgetExceeded` if the charge would exceed the configured
    /// limit, or `WorkCounterOverflow` if checked arithmetic fails.
    pub fn charge_work(&mut self, units: u64) -> Result<(), CanonicalizationError> {
        let next = self
            .work_used
            .checked_add(units)
            .ok_or(CanonicalizationError::WorkCounterOverflow)?;

        if let Some(limit) = self.budget.max_work {
            if next > limit.get() {
                return Err(CanonicalizationError::WorkBudgetExceeded {
                    work_used: self.work_used,
                    requested: units,
                    limit: limit.get(),
                });
            }
        }

        self.work_used = next;
        Ok(())
    }

    /// Record one visited entity.
    ///
    /// The implementation must use a consistent definition of an entity
    /// across repeated invocations.
    pub fn record_entity(&mut self) -> Result<(), CanonicalizationError> {
        self.entities_visited = self
            .entities_visited
            .checked_add(1)
            .ok_or(CanonicalizationError::EntityCounterOverflow)?;
        Ok(())
    }

    /// Record one actual change.
    ///
    /// Do not increment this counter for an examined entity that remains
    /// unchanged.
    pub fn record_change(&mut self) -> Result<(), CanonicalizationError> {
        self.changes_made = self
            .changes_made
            .checked_add(1)
            .ok_or(CanonicalizationError::ChangeCounterOverflow)?;
        Ok(())
    }

    /// Produce the report for a completed invocation.
    ///
    /// This method does not prove semantic preservation or IR validity.
    #[must_use]
    pub const fn report(&self) -> CanonicalizationReport {
        CanonicalizationReport {
            work_used: self.work_used,
            entities_visited: self.entities_visited,
            changes_made: self.changes_made,
        }
    }

    /// Validate a report returned by a canonicalization implementation.
    ///
    /// This prevents an implementation from claiming less or more work than
    /// the context accounted for. It does not independently verify change
    /// counts or semantic equivalence.
    pub fn validate_report(
        &self,
        report: CanonicalizationReport,
    ) -> Result<CanonicalizationReport, CanonicalizationError> {
        if report.work_used != self.work_used {
            return Err(CanonicalizationError::InvalidReport {
                reported_work: report.work_used,
                accounted_work: self.work_used,
            });
        }

        if report.entities_visited != self.entities_visited
            || report.changes_made != self.changes_made
        {
            return Err(CanonicalizationError::InvalidInput {
                code: "canonicalization-report-counters-mismatch",
            });
        }

        Ok(report)
    }
}

/// Contract implemented by the actual canonical HDLIR design representation.
///
/// Implementations must:
///
/// - Normalize only constructs whose ordering or representation is proven
///   semantically irrelevant by the registered HDLIR schema.
/// - Preserve ordered operations, state transitions, memory semantics,
///   connectivity, effects, and required provenance.
/// - Charge work before performing bounded work.
/// - Return `Err` rather than a successful report when required work is
///   incomplete.
/// - Produce deterministic output for equivalent inputs under the same
///   schema, dialect registrations, and configuration.
/// - Be idempotent: canonicalizing an already canonical design must make no
///   further changes.
/// - Avoid panics for malformed or externally supplied IR where the owning
///   IR contract requires recoverable validation.
/// - Never claim formal equivalence unless an appropriate verifier provides
///   that evidence.
///
/// This trait intentionally does not prescribe an IR storage layout.
pub trait Canonicalizable {
    /// Canonicalize this design in place.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the operation cannot be completed.
    /// The implementer must document its mutation guarantee on failure.
    /// Prefer transactional mutation: either commit a complete canonical
    /// result or leave the original design unchanged.
    fn canonicalize(
        &mut self,
        context: &mut CanonicalizationContext,
    ) -> Result<CanonicalizationReport, CanonicalizationError>;
}

/// Execute canonicalization with the supplied operational budget.
///
/// This function validates the implementation's accounting before returning
/// its report. It does not perform independent verification or prove
/// semantic preservation; those obligations belong to the HDLIR verifier
/// and the existing pass-preservation framework.
///
/// # Errors
///
/// Returns an error if canonicalization fails, exceeds its budget, or
/// reports inconsistent counters.
pub fn run_canonicalization<T: Canonicalizable>(
    design: &mut T,
    budget: CanonicalizationBudget,
) -> Result<CanonicalizationReport, CanonicalizationError> {
    let mut context = CanonicalizationContext::new(budget);
    let report = design.canonicalize(&mut context)?;
    context.validate_report(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn limited_budget(value: u64) -> CanonicalizationBudget {
        CanonicalizationBudget::limited(
            NonZeroU64::new(value).expect("test budget must be nonzero"),
        )
    }

    #[test]
    fn unlimited_budget_has_no_configured_work_limit() {
        assert_eq!(CanonicalizationBudget::unlimited().max_work(), None);
    }

    #[test]
    fn limited_budget_exposes_its_limit() {
        assert_eq!(limited_budget(7).max_work().map(NonZeroU64::get), Some(7));
    }

    #[test]
    fn work_is_charged_with_checked_accounting() {
        let mut context = CanonicalizationContext::new(limited_budget(3));

        context.charge_work(1).expect("first charge should fit");
        context.charge_work(2).expect("second charge should fit");

        assert_eq!(context.work_used(), 3);
        assert_eq!(context.report().work_used, 3);
    }

    #[test]
    fn exceeding_budget_returns_an_error_without_charging_excess_work() {
        let mut context = CanonicalizationContext::new(limited_budget(2));

        context.charge_work(2).expect("initial charge should fit");

        let error = context
            .charge_work(1)
            .expect_err("additional work must exceed budget");

        assert_eq!(
            error,
            CanonicalizationError::WorkBudgetExceeded {
                work_used: 2,
                requested: 1,
                limit: 2,
            }
        );
        assert_eq!(context.work_used(), 2);
    }

    #[test]
    fn zero_work_charge_is_valid() {
        let mut context = CanonicalizationContext::new(limited_budget(1));

        context
            .charge_work(0)
            .expect("zero work must not exceed budget");

        assert_eq!(context.work_used(), 0);
    }

    #[test]
    fn report_matches_accounted_work() {
        let mut context =
            CanonicalizationContext::new(CanonicalizationBudget::unlimited());

        context.charge_work(4).expect("unlimited budget");
        context.record_entity().expect("entity counter fits");
        context.record_change().expect("change counter fits");

        let report = context.report();

        assert_eq!(report.work_used, 4);
        assert_eq!(report.entities_visited, 1);
        assert_eq!(report.changes_made, 1);
        assert_eq!(context.validate_report(report), Ok(report));
    }

    #[test]
    fn inconsistent_work_report_is_rejected() {
        let context =
            CanonicalizationContext::new(CanonicalizationBudget::unlimited());

        let error = context
            .validate_report(CanonicalizationReport {
                work_used: 1,
                entities_visited: 0,
                changes_made: 0,
            })
            .expect_err("unaccounted work must be rejected");

        assert_eq!(
            error,
            CanonicalizationError::InvalidReport {
                reported_work: 1,
                accounted_work: 0,
            }
        );
    }

    #[test]
    fn changes_are_counted_only_when_explicitly_recorded() {
        let mut context =
            CanonicalizationContext::new(CanonicalizationBudget::unlimited());

        context.record_entity().expect("entity counter fits");

        assert_eq!(context.report().entities_visited, 1);
        assert_eq!(context.report().changes_made, 0);
    }

    #[test]
    fn context_is_independent_per_invocation() {
        let mut first =
            CanonicalizationContext::new(CanonicalizationBudget::unlimited());
        let second =
            CanonicalizationContext::new(CanonicalizationBudget::unlimited());

        first.charge_work(5).expect("first context accepts work");

        assert_eq!(first.work_used(), 5);
        assert_eq!(second.work_used(), 0);
    }
}
