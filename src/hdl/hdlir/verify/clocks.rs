
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Clock and Reset Verification
//!
//! Path: `src/hdl/hdlir/verify/clocks.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Verify logical clock-domain declarations, clock and reset source
//! references, parent-domain relationships, and process clocking contracts
//! against the containing HDLIR graph.
//!
//! ## Authority
//!
//! This verifier follows:
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/clocks.g4`
//! - `grammar/hdl/clocking.g4`
//! - `grammar/hdl/processes.g4`
//! - `grammar/specification/semantic-model.md`
//! - `src/hdl/api.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/hdl/hdlir/ir/clocks.rs`
//! - `src/hdl/hdlir/ir/processes.rs`
//! - `src/hdl/hdlir/errors.rs`
//!
//! The grammar owns source syntax. The canonical frontend owns source-level
//! semantic validation. ZUIR owns universal computational meaning. HDLIR
//! owns hardware-domain clock representation and verification.
//!
//! ## Ownership
//!
//! This module owns the orchestration of clock-domain verification and its
//! completed-work report. It does not own a second clock model, graph,
//! source-span model, type system, schema registry, or diagnostic system.
//!
//! It does not implement clock-tree synthesis, physical clock allocation,
//! scheduling, timing closure, synchronizer insertion, or CDC repair.
//!
//! ## Verification contract
//!
//! Local structure validation is necessary but insufficient. A graph adapter
//! must establish that clock/reset sources exist in the correct namespace,
//! have legal clock/reset semantics, and are visible in the appropriate
//! module or design scope. It must also validate parent relationships and
//! process clocking semantics using the actual graph and registered dialect
//! contracts.
//!
//! This verifier returns a report only when all requested checks complete.
//! A missing reference, invalid relationship, unsupported contract, or
//! exhausted work budget returns an error, never a success-shaped partial
//! report.
//!
//! ## Scalability
//!
//! No universal maximum is imposed on clock domains, processes, or events.
//! The optional work budget limits one verification invocation; it does not
//! limit language expressiveness or hardware capacity. Available memory,
//! representable collection sizes, and adapter-specific work budgets still
//! apply.
//!
//! Traversal is iterative. Work counters use checked arithmetic. Traversal
//! order is deterministic for the ordered clock-domain collection and the
//! process table's declared iteration order.
//!
//! ## Integration
//!
//! `verify/mod.rs` must register this module. The top-level graph verifier
//! must supply a concrete `ClockVerificationContext` backed by the actual
//! design, module, signal, value, process, and dialect registries.
//!
//! A successful result proves only the checks performed here. It does not
//! establish synthesis feasibility, physical timing closure, or target
//! compatibility.

use crate::hdl::hdlir::errors::{
    HdlirBudgetKind,
    HdlirError,
    HdlirResult,
};
use crate::hdl::hdlir::ids::HdlirClockDomainId;
use crate::hdl::hdlir::ir::clocks::{
    HdlirClockDomain,
    HdlirClockDomains,
    HdlirClockSource,
};
use crate::hdl::hdlir::ir::processes::{
    HdlirProcess,
    HdlirProcessTable,
};

/// Identifies the semantic role in which a clock source is referenced.
///
/// A signal may be valid in one role but invalid in another. The graph
/// adapter must apply the actual type, visibility, and dialect rules for
/// each role rather than assuming clock and reset sources are interchangeable.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ClockSourceRole {
    /// Source that activates a logical clock domain.
    Clock,

    /// Source that asserts a domain's declared reset.
    Reset,
}

/// Required graph-aware services for clock verification.
///
/// Implement this trait at the graph-verification boundary. Implementations
/// must consult the actual graph and registered contracts. There are
/// intentionally no default implementations: omitting a required check must
/// not silently turn into successful verification.
pub trait ClockVerificationContext {
    /// Validate the domain's graph ownership and scope.
    ///
    /// Establish that the domain belongs to the intended design/module scope,
    /// that its identity is legal in that scope, and that the containing
    /// graph does not define conflicting ownership.
    ///
    /// # Errors
    ///
    /// Return a structured error for missing ownership information, invalid
    /// scope, conflicting ownership, or malformed graph relationships.
    fn validate_domain_scope(
        &self,
        domain: &HdlirClockDomain,
    ) -> HdlirResult<()>;

    /// Validate a clock or reset source in its declared role.
    ///
    /// Establish that the source exists in the correct identity namespace,
    /// is visible from the domain's scope, has a legal type and value
    /// semantics for the specified role, and satisfies the applicable
    /// registered dialect contract.
    ///
    /// A typed ID alone does not prove that a value can legally be used as a
    /// clock or reset. The adapter must not invent missing edge, polarity,
    /// timing, or source semantics.
    ///
    /// # Errors
    ///
    /// Return a structured error for a missing source, invalid type, invalid
    /// scope, illegal clock/reset use, or unsupported semantics.
    fn validate_source(
        &self,
        domain: &HdlirClockDomain,
        source: HdlirClockSource,
        role: ClockSourceRole,
    ) -> HdlirResult<()>;

    /// Validate a declared parent-domain relationship.
    ///
    /// The parent reference has already been checked for existence and
    /// structural acyclicity by the canonical clock-domain collection.
    /// This method must validate graph ownership and any generated-clock or
    /// dialect-specific relationship required by the actual representation.
    ///
    /// It must not infer an unstated frequency, phase, duty cycle, or timing
    /// relationship.
    ///
    /// # Errors
    ///
    /// Return a structured error for incompatible ownership, an illegal
    /// relationship, or unsupported generated-clock semantics.
    fn validate_parent_relationship(
        &self,
        child: &HdlirClockDomain,
        parent: &HdlirClockDomain,
    ) -> HdlirResult<()>;

    /// Validate one process's relationship to its clocking context.
    ///
    /// `domain` is `Some` when the process declares an explicit clock-domain
    /// association. It is `None` when no explicit association is present.
    ///
    /// The adapter must inspect the process kind and event declarations,
    /// resolve event sources against the graph, and validate edge,
    /// qualifier, clock-domain, and reset relationships under the registered
    /// process/dialect contracts. It must distinguish a legal unclocked
    /// process from an invalid or unsupported clocking declaration.
    ///
    /// This method is called for every process, including processes without
    /// an explicit domain, so that missing clock associations cannot be
    /// silently accepted by omission.
    ///
    /// # Errors
    ///
    /// Return a structured error for an invalid clock association, missing
    /// event source, incompatible edge, invalid reset relationship, invalid
    /// scope, or unsupported clocking semantics.
    fn validate_process_clocking(
        &self,
        process: &HdlirProcess,
        domain: Option<&HdlirClockDomain>,
    ) -> HdlirResult<()>;
}

/// Operational work limit for clock verification.
///
/// One unit is charged for each top-level validation stage, domain, clock
/// source, reset source, parent relationship, process, and process event
/// accounted for by this verifier. Context implementations are responsible
/// for applying their own budgets to substantial internal graph traversals.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClockVerificationBudget {
    /// Maximum accounted work items, or `None` for no configured item cap.
    pub max_items: Option<u128>,
}

impl ClockVerificationBudget {
    /// Construct a budget with an explicit maximum work-item count.
    #[must_use]
    pub const fn new(max_items: u128) -> Self {
        Self {
            max_items: Some(max_items),
        }
    }

    /// Construct a budget without a configured work-item cap.
    ///
    /// This does not promise unlimited memory, execution time, or host
    /// resources.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self { max_items: None }
    }
}

impl Default for ClockVerificationBudget {
    fn default() -> Self {
        Self::unbounded()
    }
}

/// Counts produced by a fully completed clock verification.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ClockVerificationReport {
    /// Number of clock-domain declarations checked.
    pub clock_domains_checked: u128,

    /// Number of clock-source references checked.
    pub clock_sources_checked: u128,

    /// Number of reset-source references checked.
    pub reset_sources_checked: u128,

    /// Number of parent-domain relationships checked.
    pub parent_relationships_checked: u128,

    /// Number of process clocking relationships checked.
    pub process_relationships_checked: u128,

    /// Number of process event declarations accounted for.
    pub process_events_accounted: u128,

    /// Total work items accounted for by this verifier.
    pub work_items: u128,
}

/// Verify clock domains and process clocking contracts against the graph.
///
/// This function validates local clock-domain structure and parent-cycle
/// invariants, resolves every clock/reset source through the supplied
/// context, validates parent relationships, and checks every process's
/// clocking context.
///
/// The caller must provide the canonical process table and clock-domain
/// collection from the same design snapshot. The context must validate their
/// shared graph scope and all references that require information beyond
/// these collections.
///
/// # Errors
///
/// Returns an error if local structures are invalid, a required source or
/// parent is missing, a relationship is invalid or unsupported, the adapter
/// rejects a process's clocking semantics, arithmetic overflows, or the
/// configured work budget is exhausted.
pub fn verify_clocks<C: ClockVerificationContext>(
    clock_domains: &HdlirClockDomains,
    processes: &HdlirProcessTable,
    context: &C,
    budget: ClockVerificationBudget,
) -> HdlirResult<ClockVerificationReport> {
    let mut verifier = ClockVerifier {
        budget,
        report: ClockVerificationReport::default(),
    };

    verifier.consume_work("validating HDLIR clock-domain structure")?;

    // Charge for the domain records before invoking the collection's
    // structural validation. The collection performs local checks and an
    // iterative parent-cycle check; it does not resolve graph source IDs.
    verifier.consume_many(
        u128::try_from(clock_domains.len()).map_err(|_| {
            HdlirError::invalid_invariant(
                "clock_verification.domain_count_representable",
                "clock-domain count cannot be represented by the verifier counter",
            )
        })?,
        "validating clock-domain records and parent relationships",
    )?;

    clock_domains.validate_structure()?;

    for (_, domain) in clock_domains.iter() {
        verifier.consume_work("validating clock-domain graph ownership")?;
        context.validate_domain_scope(domain)?;
        verifier.increment_domains()?;

        verifier.consume_work("validating a clock source")?;
        domain.source().validate_structure()?;
        context.validate_source(
            domain,
            domain.source(),
            ClockSourceRole::Clock,
        )?;
        verifier.increment_clock_sources()?;

        if let Some(reset) = domain.reset() {
            verifier.consume_work("validating a reset source")?;
            reset.source().validate_structure()?;
            context.validate_source(
                domain,
                reset.source(),
                ClockSourceRole::Reset,
            )?;
            verifier.increment_reset_sources()?;
        }

        if let Some(parent_id) = domain.parent() {
            verifier.consume_work("validating a parent clock relationship")?;

            let parent = clock_domains.get(parent_id).ok_or_else(|| {
                HdlirError::MissingEntity {
                    entity_kind: "parent clock domain",
                    identity: parent_id.to_string(),
                    scope: Some(format!("clock domain {}", domain.id())),
                }
            })?;

            context.validate_parent_relationship(domain, parent)?;
            verifier.increment_parent_relationships()?;
        }
    }

    verifier.consume_work("validating the HDLIR process table")?;
    processes.validate_structure()?;

    for process in processes.iter() {
        verifier.consume_work("validating a process clocking relationship")?;

        let domain = match process.clock_domain() {
            Some(domain_id) => {
                let domain = clock_domains.get(domain_id).ok_or_else(|| {
                    HdlirError::MissingEntity {
                        entity_kind: "HDLIR clock domain",
                        identity: domain_id.to_string(),
                        scope: Some(format!("process {}", process.id())),
                    }
                })?;

                Some(domain)
            }
            None => None,
        };

        context.validate_process_clocking(process, domain)?;
        verifier.increment_process_relationships()?;

        for _event in process.events() {
            verifier.consume_work("accounting for a process event")?;
            verifier.increment_process_events()?;
        }
    }

    Ok(verifier.report)
}

struct ClockVerifier {
    budget: ClockVerificationBudget,
    report: ClockVerificationReport,
}

impl ClockVerifier {
    fn consume_work(&mut self, operation: &'static str) -> HdlirResult<()> {
        self.consume_many(1, operation)
    }

    fn consume_many(
        &mut self,
        amount: u128,
        operation: &'static str,
    ) -> HdlirResult<()> {
        let next = self.report.work_items.checked_add(amount).ok_or_else(|| {
            HdlirError::budget_exceeded(
                HdlirBudgetKind::Verification,
                self.budget.max_items,
                None,
                "clock verification work counter overflow",
            )
        })?;

        if self.budget.max_items.is_some_and(|limit| next > limit) {
            return Err(HdlirError::budget_exceeded(
                HdlirBudgetKind::Verification,
                self.budget.max_items,
                Some(next),
                operation,
            ));
        }

        self.report.work_items = next;
        Ok(())
    }

    fn increment_domains(&mut self) -> HdlirResult<()> {
        self.report.clock_domains_checked = self
            .report
            .clock_domains_checked
            .checked_add(1)
            .ok_or_else(|| {
                HdlirError::budget_exceeded(
                    HdlirBudgetKind::Verification,
                    self.budget.max_items,
                    None,
                    "clock-domain report counter overflow",
                )
            })?;
        Ok(())
    }

    fn increment_clock_sources(&mut self) -> HdlirResult<()> {
        self.report.clock_sources_checked = self
            .report
            .clock_sources_checked
            .checked_add(1)
            .ok_or_else(|| {
                HdlirError::budget_exceeded(
                    HdlirBudgetKind::Verification,
                    self.budget.max_items,
                    None,
                    "clock-source report counter overflow",
                )
            })?;
        Ok(())
    }

    fn increment_reset_sources(&mut self) -> HdlirResult<()> {
        self.report.reset_sources_checked = self
            .report
            .reset_sources_checked
            .checked_add(1)
            .ok_or_else(|| {
                HdlirError::budget_exceeded(
                    HdlirBudgetKind::Verification,
                    self.budget.max_items,
                    None,
                    "reset-source report counter overflow",
                )
            })?;
        Ok(())
    }

    fn increment_parent_relationships(&mut self) -> HdlirResult<()> {
        self.report.parent_relationships_checked = self
            .report
            .parent_relationships_checked
            .checked_add(1)
            .ok_or_else(|| {
                HdlirError::budget_exceeded(
                    HdlirBudgetKind::Verification,
                    self.budget.max_items,
                    None,
                    "parent-relationship report counter overflow",
                )
            })?;
        Ok(())
    }

    fn increment_process_relationships(&mut self) -> HdlirResult<()> {
        self.report.process_relationships_checked = self
            .report
            .process_relationships_checked
            .checked_add(1)
            .ok_or_else(|| {
                HdlirError::budget_exceeded(
                    HdlirBudgetKind::Verification,
                    self.budget.max_items,
                    None,
                    "process-relationship report counter overflow",
                )
            })?;
        Ok(())
    }

    fn increment_process_events(&mut self) -> HdlirResult<()> {
        self.report.process_events_accounted = self
            .report
            .process_events_accounted
            .checked_add(1)
            .ok_or_else(|| {
                HdlirError::budget_exceeded(
                    HdlirBudgetKind::Verification,
                    self.budget.max_items,
                    None,
                    "process-event report counter overflow",
                )
            })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ClockVerificationBudget,
        ClockVerificationReport,
    };

    #[test]
    fn default_budget_has_no_configured_work_cap() {
        assert_eq!(
            ClockVerificationBudget::default(),
            ClockVerificationBudget::unbounded()
        );
        assert_eq!(
            ClockVerificationBudget::default().max_items,
            None
        );
    }

    #[test]
    fn explicit_zero_budget_is_preserved() {
        assert_eq!(
            ClockVerificationBudget::new(0).max_items,
            Some(0)
        );
    }

    #[test]
    fn report_starts_empty() {
        let report = ClockVerificationReport::default();

        assert_eq!(report.clock_domains_checked, 0);
        assert_eq!(report.clock_sources_checked, 0);
        assert_eq!(report.reset_sources_checked, 0);
        assert_eq!(report.parent_relationships_checked, 0);
        assert_eq!(report.process_relationships_checked, 0);
        assert_eq!(report.process_events_accounted, 0);
        assert_eq!(report.work_items, 0);
    }
}
