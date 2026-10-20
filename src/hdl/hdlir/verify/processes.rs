
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Behavioral Process Verification
//!
//! Path: `src/hdl/hdlir/verify/processes.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Verify process declarations against the complete HDLIR graph and registered
//! process/dialect semantics. Local process validation is necessary but does
//! not establish that body regions, event sources, or clock domains exist or
//! are legally related.
//!
//! ## Authority and ownership
//!
//! Source syntax is defined by the canonical grammar, including
//! `grammar/hdl/processes.g4`, `combinational.g4`, `sequential.g4`,
//! `clocking.g4`, and the normative `grammar/spec/hdl.md`.
//! The canonical frontend owns source-level semantic truth. ZUIR owns the
//! universal computational representation. HDLIR owns hardware-domain
//! process declarations and graph verification.
//!
//! This module reuses `ir::processes`, `ir::regions`, `ir::clocks`, and the
//! shared HDLIR error and ID contracts. It does not define another process
//! model, clock model, source-span model, operation registry, or type system.
//!
//! ## Scope
//!
//! This verifier checks local process invariants, process identity uniqueness,
//! exclusive process-body ownership within the supplied table, registered
//! process-kind rules, body-region graph and dialect rules, event-source and
//! event-edge legality, and clock-domain references/compatibility.
//!
//! The graph adapter must check assignment legality, combinational completeness,
//! state-transition semantics, region ownership and membership, source/value
//! visibility, clock/reset semantics, and extension-specific rules. A no-op
//! adapter is not a valid production adapter.
//!
//! This module does not implement simulation, scheduling, clock-tree synthesis,
//! CDC repair, synthesis, physical allocation, placement, routing, or timing
//! closure.
//!
//! ## Scalability
//!
//! No fixed maximum process count, event count, clock count, or region count is
//! imposed. An optional work budget limits verifier effort, not language
//! semantics. Work-counter overflow and budget exhaustion are explicit errors.
//! The context adapter must account for any substantial internal work or apply
//! its own documented operational budget.
//!
//! A finite implementation cannot process literally infinite input. Processing
//! is limited by available resources, supported semantics, representable data,
//! and explicitly configured operational budgets.
//!
//! ## Integration
//!
//! `verify/mod.rs` must register this module. The graph-level verification
//! entry point must construct a concrete `ProcessVerificationContext` backed
//! by the actual design/module/region/block/operation/value/clock collections.
//! It must not treat a successful process check as proof that the complete
//! design is synthesizable or feasible on a target.

use std::collections::BTreeMap;

use crate::hdl::hdlir::errors::{
    HdlirBudgetKind,
    HdlirError,
    HdlirResult,
};
use crate::hdl::hdlir::ids::{
    HdlirClockDomainId,
    HdlirProcessId,
    HdlirRegionId,
};
use crate::hdl::hdlir::ir::clocks::HdlirClockDomains;
use crate::hdl::hdlir::ir::processes::{
    HdlirProcess,
    HdlirProcessEvent,
    HdlirProcessTable,
};

/// Required graph- and schema-aware process verification services.
///
/// Implement this trait at the graph-verification boundary. Each method must
/// resolve and validate the relationship described by its contract against
/// canonical graph records and registered schema/dialect definitions.
///
/// Methods deliberately have no default implementations: silently skipping a
/// required stage must not be possible by inheriting a permissive default.
pub trait ProcessVerificationContext {
    /// Validate that the process kind is known and legal in its declared
    /// scope.
    ///
    /// This includes resolving custom process kinds against the registered,
    /// version-compatible dialect contract and validating kind-specific
    /// attributes. A qualified name alone does not register an extension.
    ///
    /// # Errors
    ///
    /// Return a structured error for an unknown process kind, invalid
    /// attributes, incompatible schema versions, or unsupported semantics.
    fn validate_process_kind(
        &self,
        process: &HdlirProcess,
    ) -> HdlirResult<()>;

    /// Validate the process's body region against the complete graph.
    ///
    /// The implementation must establish that the body region exists, has the
    /// required region kind, is owned by this process in the current graph,
    /// and contains valid block/operation relationships. It must validate
    /// process-body semantics, including assignment ordering and assignment
    /// class, combinational completeness/latch implications, state-transition
    /// behavior, operation effects, and dialect-specific body constraints
    /// wherever those properties are part of the registered contract.
    ///
    /// The implementation must not claim a rule was checked when the
    /// underlying representation does not provide enough information to
    /// establish it.
    ///
    /// # Errors
    ///
    /// Return a structured error for missing or incorrectly owned regions,
    /// malformed blocks, illegal assignments, invalid process semantics, or
    /// unsupported body constructs.
    fn validate_body_region(
        &self,
        process: &HdlirProcess,
    ) -> HdlirResult<()>;

    /// Validate one event's source, edge, qualifiers, and process relationship.
    ///
    /// The implementation must resolve signal/value identity in the correct
    /// namespace and scope; establish that the source may be observed as an
    /// event source; validate edge and qualifier combinations; and verify
    /// clocking relationships required by the process/dialect contract.
    ///
    /// For a sequential process, an edge event must be interpreted using the
    /// canonical event/clock semantics. The verifier must not infer that any
    /// arbitrary value is a valid clock merely because it has a value ID.
    ///
    /// # Errors
    ///
    /// Return a structured error for a missing source, invalid source kind,
    /// illegal edge/qualifier combination, invalid scope, or unsupported
    /// event semantics.
    fn validate_event(
        &self,
        process: &HdlirProcess,
        event: &HdlirProcessEvent,
    ) -> HdlirResult<()>;

    /// Validate a process's explicit clock-domain reference.
    ///
    /// The implementation must establish that the domain exists, is visible
    /// in the correct design/module scope, and is compatible with the process.
    /// It must also validate relevant clock/reset source contracts through
    /// the canonical clock-domain representation. This is logical verification,
    /// not physical timing closure or CDC repair.
    ///
    /// # Errors
    ///
    /// Return a structured error for a missing domain, invalid ownership,
    /// incompatible process/domain relationship, or unsupported clocking
    /// semantics.
    fn validate_clock_domain(
        &self,
        process: &HdlirProcess,
        clock_domain: HdlirClockDomainId,
    ) -> HdlirResult<()>;
}

/// Operational work limit for process verification.
///
/// Each process, event, body-region validation, registered-kind validation,
/// and explicit clock-domain validation consumes a work item. Adapter work
/// must also be bounded or accounted for by the adapter's own contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProcessVerificationBudget {
    /// Maximum accounted verifier work, or `None` for no configured item cap.
    pub max_items: Option<u128>,
}

impl ProcessVerificationBudget {
    /// Create a budget with an explicit maximum number of work items.
    #[must_use]
    pub const fn new(max_items: u128) -> Self {
        Self {
            max_items: Some(max_items),
        }
    }

    /// Create a budget without a configured work-item cap.
    ///
    /// Available memory, representable collection sizes, adapter budgets, and
    /// execution resources still apply.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self { max_items: None }
    }
}

impl Default for ProcessVerificationBudget {
    fn default() -> Self {
        Self::unbounded()
    }
}

/// Counts produced by a fully completed process verification.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProcessVerificationReport {
    /// Number of process declarations checked.
    pub processes_checked: u128,

    /// Number of process events checked.
    pub events_checked: u128,

    /// Number of process body-region checks completed.
    pub body_regions_checked: u128,

    /// Number of explicit clock-domain references checked.
    pub clock_domain_references_checked: u128,

    /// Number of unique process-body regions indexed.
    pub distinct_body_regions_checked: u128,

    /// Total accounted verifier work items.
    pub work_items: u128,
}

/// Verify process declarations against graph and dialect contracts.
///
/// This function first validates the process table and clock-domain collection
/// locally, then validates every process against the supplied context. A
/// process body region may be owned by only one process in this input. The
/// concrete context remains responsible for validating region ownership in the
/// complete graph and for all body, assignment, event-source, and clocking
/// semantics.
///
/// A report is returned only if all requested checks complete successfully.
/// Missing graph information, invalid process semantics, or budget exhaustion
/// returns an error rather than a success-shaped partial report.
///
/// # Arguments
///
/// * `processes` — canonical HDLIR process table.
/// * `clock_domains` — canonical logical clock-domain collection.
/// * `context` — required graph/schema adapter.
/// * `budget` — optional operational work budget.
///
/// # Errors
///
/// Returns an HDLIR error if local process/clock structures are malformed,
/// process IDs or body ownership are duplicated, a required reference or
/// contract is invalid, a context check fails, arithmetic overflows, or the
/// operational budget is exhausted.
pub fn verify_processes<C: ProcessVerificationContext>(
    processes: &HdlirProcessTable,
    clock_domains: &HdlirClockDomains,
    context: &C,
    budget: ProcessVerificationBudget,
) -> HdlirResult<ProcessVerificationReport> {
    let mut verifier = ProcessVerifier {
        budget,
        report: ProcessVerificationReport::default(),
    };

    verifier.consume_work("validating the HDLIR process table")?;
    processes.validate_structure()?;

    verifier.consume_work("validating HDLIR clock-domain structures")?;
    clock_domains.validate_structure()?;

    let mut body_region_owners: BTreeMap<HdlirRegionId, HdlirProcessId> =
        BTreeMap::new();

    for process in processes.iter() {
        verifier.consume_work("checking an HDLIR process")?;

        process.validate_structure()?;
        verifier.increment_processes()?;

        if let Some(previous) =
            body_region_owners.insert(process.body_region(), process.id())
        {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR process body-region ownership",
                identity: process.body_region().to_string(),
                scope: Some(format!(
                    "processes {previous} and {}",
                    process.id()
                )),
            });
        }

        verifier.consume_work("validating a registered process kind")?;
        context.validate_process_kind(process)?;

        verifier.consume_work("validating a process body region")?;
        context.validate_body_region(process)?;
        verifier.increment_body_regions()?;

        for event in process.events() {
            verifier.consume_work("checking an HDLIR process event")?;
            event.validate_structure()?;
            context.validate_event(process, event)?;
            verifier.increment_events()?;
        }

        if let Some(clock_domain) = process.clock_domain() {
            verifier.consume_work("checking a process clock-domain reference")?;

            if clock_domains.get(clock_domain).is_none() {
                return Err(HdlirError::MissingEntity {
                    entity_kind: "HDLIR clock domain",
                    identity: clock_domain.to_string(),
                    scope: Some(format!("process {}", process.id())),
                });
            }

            context.validate_clock_domain(process, clock_domain)?;
            verifier.increment_clock_domains()?;
        }
    }

    verifier.report.distinct_body_regions_checked =
        u128::try_from(body_region_owners.len()).map_err(|_| {
            HdlirError::invalid_invariant(
                "process_verification.counter_range",
                "body-region count cannot be represented as u128",
            )
        })?;

    Ok(verifier.report)
}

struct ProcessVerifier {
    budget: ProcessVerificationBudget,
    report: ProcessVerificationReport,
}

impl ProcessVerifier {
    fn consume_work(&mut self, operation: &'static str) -> HdlirResult<()> {
        let next = self.report.work_items.checked_add(1).ok_or_else(|| {
            HdlirError::invalid_invariant(
                "process_verification.counter_range",
                "work counter overflowed u128",
            )
        })?;

        if let Some(limit) = self.budget.max_items {
            if next > limit {
                return Err(HdlirError::budget_exceeded(
                    HdlirBudgetKind::Verification,
                    Some(limit),
                    Some(next),
                    operation,
                ));
            }
        }

        self.report.work_items = next;
        Ok(())
    }

    fn increment_processes(&mut self) -> HdlirResult<()> {
        self.report.processes_checked = self
            .report
            .processes_checked
            .checked_add(1)
            .ok_or_else(|| {
                HdlirError::invalid_invariant(
                    "process_verification.counter_range",
                    "process counter overflowed u128",
                )
            })?;
        Ok(())
    }

    fn increment_events(&mut self) -> HdlirResult<()> {
        self.report.events_checked = self
            .report
            .events_checked
            .checked_add(1)
            .ok_or_else(|| {
                HdlirError::invalid_invariant(
                    "process_verification.counter_range",
                    "event counter overflowed u128",
                )
            })?;
        Ok(())
    }

    fn increment_body_regions(&mut self) -> HdlirResult<()> {
        self.report.body_regions_checked = self
            .report
            .body_regions_checked
            .checked_add(1)
            .ok_or_else(|| {
                HdlirError::invalid_invariant(
                    "process_verification.counter_range",
                    "body-region counter overflowed u128",
                )
            })?;
        Ok(())
    }

    fn increment_clock_domains(&mut self) -> HdlirResult<()> {
        self.report.clock_domain_references_checked = self
            .report
            .clock_domain_references_checked
            .checked_add(1)
            .ok_or_else(|| {
                HdlirError::invalid_invariant(
                    "process_verification.counter_range",
                    "clock-domain reference counter overflowed u128",
                )
            })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ProcessVerificationBudget,
        ProcessVerificationReport,
    };

    #[test]
    fn unbounded_budget_has_no_semantic_process_ceiling() {
        assert_eq!(ProcessVerificationBudget::unbounded().max_items, None);
    }

    #[test]
    fn explicit_budget_is_an_operational_limit() {
        assert_eq!(
            ProcessVerificationBudget::new(23).max_items,
            Some(23)
        );
    }

    #[test]
    fn report_starts_with_zero_counts() {
        assert_eq!(
            ProcessVerificationReport::default(),
            ProcessVerificationReport {
                processes_checked: 0,
                events_checked: 0,
                body_regions_checked: 0,
                clock_domain_references_checked: 0,
                distinct_body_regions_checked: 0,
                work_items: 0,
            }
        );
    }
}
