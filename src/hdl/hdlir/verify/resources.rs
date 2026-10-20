
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Resource-Contract Verification
//!
//! Path: `src/hdl/hdlir/verify/resources.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Orchestrate verification of logical resource requirements, symbolic
//! resource expressions, and resource constraints associated with an HDLIR
//! design. The verifier uses an adapter over the canonical resource model;
//! it deliberately does not introduce another resource type system.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/resources/resource.g4`
//! - `grammar/resources/resource-expressions.g4`
//! - `grammar/hdl/processes.g4`
//! - `grammar/hdl/sequential.g4`
//! - `src/frontend/semantic/resources/model.rs`
//! - `src/frontend/semantic/resources/requirements.rs`
//! - `src/frontend/semantic/resources/constraints.rs`
//! - `src/hdl/resources.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/config.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ir/`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns syntax. The canonical frontend owns source-level meaning
//! and the semantic resource model. ZUIR owns universal computational
//! representation. HDLIR owns hardware-domain representation and this
//! verification orchestration.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Explicit resource-verification work budgets.
//! - Checked work accounting shared with a resource adapter.
//! - Resource-verification result aggregation.
//! - Deterministic traversal and duplicate-subject detection.
//! - The distinction between completed checks and unresolved symbolic facts.
//!
//! This module does not own:
//!
//! - Resource quantities, units, symbolic expressions, or constraint syntax.
//! - Source parsing, source-level semantic validation, or name resolution.
//! - Target inventories, capability discovery, or target selection.
//! - Physical resource allocation, scheduling, mapping, placement, or routing.
//! - Synthesis, physical timing closure, or proof of implementability.
//!
//! ## Verification contract
//!
//! The adapter MUST check the canonical contracts for each subject. It MUST
//! reject malformed requirements and contradictory or invalid constraints.
//! It MUST preserve unresolved symbolic facts as unresolved; it MUST NOT
//! invent bindings or treat an unevaluated expression as proven satisfied.
//!
//! A completed report means that the adapter completed the requested
//! resource-contract checks. Unresolved symbolic facts are counted separately
//! and do not, by themselves, mean the contract is invalid. Callers requiring
//! concrete resolution must check that no unresolved facts remain.
//!
//! Target feasibility is a separate downstream question. A logically
//! consistent requirement can still be unsupported by a target.
//!
//! ## Scalability and safety
//!
//! No universal hardware maximum is imposed. `None` for a work limit means
//! that this invocation has no configured item-count budget; actual work
//! remains limited by available memory, execution resources, the adapter,
//! and representable values.
//!
//! All verifier counters use checked arithmetic. Budget exhaustion returns a
//! structured error and never returns a successful partial report.
//! Adapter implementations MUST charge the shared meter for additional
//! resource-dependent work they perform.
//!
//! ## Integration
//!
//! `verify/mod.rs` must register `pub mod resources;` and expose the intended
//! public API. The graph-level verifier should invoke this module after the
//! relevant design references and canonical resource facts are available.
//! The adapter should consume the existing semantic resource model through
//! `src/hdl/resources.rs` or the appropriate canonical semantic APIs.
//!
//! Do not manufacture a resource inventory from HDLIR counts or treat a
//! logical estimate as an allocated physical resource.
//!
//! ## Freeze criteria
//!
//! - The adapter is implemented against actual canonical resource APIs.
//! - Every relevant design subject is visited exactly once.
//! - Malformed and contradictory contracts return explicit errors.
//! - Symbolic unknowns are counted, not silently accepted as proven.
//! - Budget exhaustion and counter overflow are tested.
//! - Repository formatting, compilation, tests, and Clippy pass.

use std::collections::BTreeSet;
use std::fmt::{self, Display};

use crate::hdl::hdlir::errors::{HdlirBudgetKind, HdlirError, HdlirResult};

/// Explicit work budget for one resource-verification invocation.
///
/// This budget measures verifier work, not the number or size of hardware
/// resources described by the design. An absent limit imposes no configured
/// item-count ceiling for this invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceVerificationBudget {
    /// Maximum number of work items, or `None` for no configured limit.
    pub max_work_items: Option<u128>,
}

impl ResourceVerificationBudget {
    /// Construct a budget with the given maximum number of work items.
    #[must_use]
    pub const fn new(max_work_items: u128) -> Self {
        Self {
            max_work_items: Some(max_work_items),
        }
    }

    /// Construct a budget without a configured item-count limit.
    ///
    /// This does not promise infinite memory or execution time.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_work_items: None,
        }
    }
}

impl Default for ResourceVerificationBudget {
    fn default() -> Self {
        Self::unbounded()
    }
}

/// Checked work meter shared with the canonical resource adapter.
///
/// The verifier charges at least one item per subject. Adapter implementations
/// must also charge this meter for expression nodes, constraints, bindings, or
/// other potentially unbounded work performed while checking a subject.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceWorkMeter {
    budget: ResourceVerificationBudget,
    consumed: u128,
}

impl ResourceWorkMeter {
    /// Construct a fresh meter for one verification invocation.
    #[must_use]
    pub const fn new(budget: ResourceVerificationBudget) -> Self {
        Self {
            budget,
            consumed: 0,
        }
    }

    /// Charge the specified number of work items.
    ///
    /// A charge is atomic: if it would exceed the configured budget or
    /// overflow the counter, the meter remains unchanged.
    ///
    /// # Errors
    ///
    /// Returns a structured budget-exhaustion or arithmetic-overflow error.
    pub fn charge(&mut self, amount: u128, operation: &'static str) -> HdlirResult<()> {
        let next = self
            .consumed
            .checked_add(amount)
            .ok_or_else(|| HdlirError::ArithmeticOverflow {
                operation: "accounting for HDLIR resource-verification work",
                context: Some(operation.to_owned()),
            })?;

        if let Some(limit) = self.budget.max_work_items {
            if next > limit {
                return Err(HdlirError::budget_exceeded(
                    HdlirBudgetKind::Verification,
                    Some(limit),
                    Some(self.consumed),
                    operation,
                ));
            }
        }

        self.consumed = next;
        Ok(())
    }

    /// Return the number of work items successfully charged.
    #[must_use]
    pub const fn consumed(&self) -> u128 {
        self.consumed
    }

    /// Return the budget associated with this meter.
    #[must_use]
    pub const fn budget(&self) -> ResourceVerificationBudget {
        self.budget
    }
}

/// Completed resource checks for one verification subject.
///
/// Counts are supplied by the adapter only after the corresponding checks
/// have actually completed. Unresolved expressions are reported separately
/// from invalid or contradictory contracts.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ResourceSubjectReport {
    /// Number of logical resource requirements checked.
    pub requirements_checked: u128,

    /// Number of resource constraints checked for internal consistency.
    pub constraints_checked: u128,

    /// Number of symbolic facts that remain unresolved.
    ///
    /// These facts have not been proven true or false. They must not be
    /// interpreted as satisfied requirements.
    pub unresolved_symbolic_facts: u128,
}

/// Aggregate report from a completed resource-verification invocation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ResourceVerificationReport {
    /// Number of unique subjects successfully checked.
    pub subjects_checked: u128,

    /// Total logical resource requirements checked.
    pub requirements_checked: u128,

    /// Total resource constraints checked.
    pub constraints_checked: u128,

    /// Total unresolved symbolic facts retained by the checks.
    pub unresolved_symbolic_facts: u128,

    /// Total work items charged to the shared meter.
    pub work_items: u128,
}

impl ResourceVerificationReport {
    /// Return whether every symbolic fact reported by the adapter was resolved.
    ///
    /// This says nothing about target feasibility or physical implementation.
    #[must_use]
    pub const fn all_symbolic_facts_resolved(&self) -> bool {
        self.unresolved_symbolic_facts == 0
    }
}

/// Adapter to the canonical resource model and the enclosing HDLIR graph.
///
/// `Subject` should be a stable, typed identity for a design entity whose
/// logical resource contracts need checking. The adapter decides which
/// subjects are relevant; it must not omit a subject merely because its
/// requirements are symbolic or difficult to resolve.
///
/// Implementations must return subjects in deterministic order. Duplicate
/// subjects are rejected by the verifier rather than counted twice.
///
/// The generic associated iterator avoids forcing the adapter to allocate a
/// second vector of every subject just for verification.
pub trait HdlirResourceVerificationContext {
    /// Stable identity of a resource-bearing design subject.
    type Subject: Clone + Ord + Display;

    /// Iterator returned by [`Self::subjects`].
    type SubjectIter<'a>: Iterator<Item = Self::Subject>
    where
        Self: 'a;

    /// Enumerate every subject whose resource contracts require verification.
    fn subjects(&self) -> Self::SubjectIter<'_>;

    /// Verify one subject against the canonical semantic resource contracts.
    ///
    /// Implementations must validate malformed requirements, illegal
    /// quantities, inconsistent units or constraints, and invalid expression
    /// structure. Symbolic facts that cannot yet be evaluated should be
    /// counted as unresolved instead of being declared satisfied.
    ///
    /// Charge `meter` for additional work performed. In particular, recursive
    /// or iterative symbolic-expression traversal and constraint evaluation
    /// must not bypass the invocation's configured work budget.
    ///
    /// This method must not query physical hardware, allocate resources, or
    /// turn a logical estimate into a physical allocation.
    ///
    /// # Errors
    ///
    /// Returns an error if a required contract is malformed, contradictory,
    /// unsupported by the active semantic contract, or cannot be checked
    /// without violating the requested policy. Budget exhaustion is explicit.
    fn verify_subject(
        &self,
        subject: &Self::Subject,
        meter: &mut ResourceWorkMeter,
    ) -> HdlirResult<ResourceSubjectReport>;
}

/// Verify logical resource contracts for all subjects exposed by a context.
///
/// This function verifies resource-contract consistency only. It does not
/// determine whether any target can satisfy the requirements and does not
/// perform resource allocation.
///
/// # Errors
///
/// Returns an error for duplicate subjects, invalid or contradictory
/// contracts, budget exhaustion, or counter overflow. No report is returned
/// when any required subject fails or processing stops before completion.
///
/// # Determinism
///
/// Deterministic output requires the context to enumerate subjects in a stable
/// order and the canonical resource adapter to produce deterministic results.
pub fn verify_resources<C: HdlirResourceVerificationContext>(
    context: &C,
    budget: ResourceVerificationBudget,
) -> HdlirResult<ResourceVerificationReport> {
    let mut meter = ResourceWorkMeter::new(budget);
    let mut seen = BTreeSet::new();
    let mut report = ResourceVerificationReport::default();

    for subject in context.subjects() {
        meter.charge(1, "visiting an HDLIR resource-verification subject")?;

        if !seen.insert(subject.clone()) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR resource-verification subject",
                identity: subject.to_string(),
                scope: Some("one resource-verification invocation".to_owned()),
            });
        }

        let subject_report = context.verify_subject(&subject, &mut meter)?;

        report.subjects_checked = checked_add(
            report.subjects_checked,
            1,
            "counting resource-verification subjects",
        )?;
        report.requirements_checked = checked_add(
            report.requirements_checked,
            subject_report.requirements_checked,
            "counting checked resource requirements",
        )?;
        report.constraints_checked = checked_add(
            report.constraints_checked,
            subject_report.constraints_checked,
            "counting checked resource constraints",
        )?;
        report.unresolved_symbolic_facts = checked_add(
            report.unresolved_symbolic_facts,
            subject_report.unresolved_symbolic_facts,
            "counting unresolved symbolic resource facts",
        )?;
    }

    report.work_items = meter.consumed();
    Ok(report)
}

/// Add two report counters without allowing silent wraparound.
fn checked_add(lhs: u128, rhs: u128, operation: &'static str) -> HdlirResult<u128> {
    lhs.checked_add(rhs)
        .ok_or_else(|| HdlirError::ArithmeticOverflow {
            operation,
            context: None,
        })
}

impl Display for ResourceVerificationReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "resource verification: {} subjects, {} requirements, \
             {} constraints, {} unresolved symbolic facts, {} work items",
            self.subjects_checked,
            self.requirements_checked,
            self.constraints_checked,
            self.unresolved_symbolic_facts,
            self.work_items
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct Subject(u32);

    impl Display for Subject {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "subject-{}", self.0)
        }
    }

    struct TestContext {
        subjects: Vec<Subject>,
        result: ResourceSubjectReport,
        extra_work: u128,
    }

    impl HdlirResourceVerificationContext for TestContext {
        type Subject = Subject;
        type SubjectIter<'a> = std::vec::IntoIter<Subject>;

        fn subjects(&self) -> Self::SubjectIter<'_> {
            self.subjects.clone().into_iter()
        }

        fn verify_subject(
            &self,
            _subject: &Self::Subject,
            meter: &mut ResourceWorkMeter,
        ) -> HdlirResult<ResourceSubjectReport> {
            meter.charge(self.extra_work, "checking test resource expressions")?;
            Ok(self.result)
        }
    }

    #[test]
    fn default_budget_has_no_configured_item_limit() {
        assert_eq!(ResourceVerificationBudget::default().max_work_items, None);
    }

    #[test]
    fn meter_charges_work_and_rejects_exhaustion_atomically() {
        let mut meter = ResourceWorkMeter::new(ResourceVerificationBudget::new(2));

        meter.charge(2, "test work").unwrap();
        assert_eq!(meter.consumed(), 2);

        let result = meter.charge(1, "excess test work");

        assert!(matches!(result, Err(HdlirError::BudgetExceeded { .. })));
        assert_eq!(meter.consumed(), 2);
    }

    #[test]
    fn reports_completed_checks_and_unresolved_symbolic_facts() {
        let context = TestContext {
            subjects: vec![Subject(1), Subject(2)],
            result: ResourceSubjectReport {
                requirements_checked: 3,
                constraints_checked: 2,
                unresolved_symbolic_facts: 1,
            },
            extra_work: 4,
        };

        let report = verify_resources(&context, ResourceVerificationBudget::new(20)).unwrap();

        assert_eq!(report.subjects_checked, 2);
        assert_eq!(report.requirements_checked, 6);
        assert_eq!(report.constraints_checked, 4);
        assert_eq!(report.unresolved_symbolic_facts, 2);
        assert_eq!(report.work_items, 10);
        assert!(!report.all_symbolic_facts_resolved());
    }

    #[test]
    fn rejects_duplicate_subjects() {
        let context = TestContext {
            subjects: vec![Subject(7), Subject(7)],
            result: ResourceSubjectReport::default(),
            extra_work: 0,
        };

        assert!(matches!(
            verify_resources(&context, ResourceVerificationBudget::unbounded()),
            Err(HdlirError::DuplicateEntity { .. })
        ));
    }

    #[test]
    fn budget_exhaustion_never_returns_a_successful_report() {
        let context = TestContext {
            subjects: vec![Subject(1)],
            result: ResourceSubjectReport::default(),
            extra_work: 1,
        };

        assert!(matches!(
            verify_resources(&context, ResourceVerificationBudget::new(1)),
            Err(HdlirError::BudgetExceeded { .. })
        ));
    }

    #[test]
    fn empty_design_resource_context_is_valid() {
        let context = TestContext {
            subjects: Vec::new(),
            result: ResourceSubjectReport::default(),
            extra_work: 0,
        };

        let report = verify_resources(&context, ResourceVerificationBudget::new(0)).unwrap();

        assert_eq!(report, ResourceVerificationReport::default());
        assert!(report.all_symbolic_facts_resolved());
    }
}
