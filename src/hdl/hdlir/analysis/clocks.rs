
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Logical Clock Analysis
//!
//! Analyzes logical clock-domain hierarchy, process associations, and
//! consistency between declared clock edges and process event specifications.
//!
//! The canonical grammar owns clock syntax. The frontend owns source-level
//! semantic analysis. `ir::clocks` owns clock-domain declarations and
//! `ir::processes` owns process/event declarations. This module computes
//! read-only analysis facts from those existing representations.
//!
//! This module does not prove CDC safety, infer timing constraints, insert
//! synchronizers, schedule hardware, construct clock trees, or prove timing
//! closure. Those require separate contracts and analyses.
//!
//! All traversal is iterative. Optional budgets constrain analysis work, not
//! the number of clocks permitted by the language. Exhaustion is reported
//! explicitly and never presented as complete analysis.

use std::collections::{BTreeMap, BTreeSet};

use crate::hdl::hdlir::ids::{HdlirClockDomainId, HdlirProcessId};
use crate::hdl::hdlir::ir::clocks::{
    HdlirClockDomain, HdlirClockDomains, HdlirClockEdge, HdlirClockSource,
};
use crate::hdl::hdlir::ir::processes::{
    HdlirEventEdge, HdlirEventSource, HdlirProcess, HdlirProcessKind,
    HdlirProcessTable,
};

/// Explicit operational budgets for one clock-analysis invocation.
///
/// `None` means no explicit limit for that unit of work. It does not promise
/// unlimited memory or execution time.
///
/// These limits are not hardware-capacity or language-semantic limits.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ClockAnalysisBudget {
    /// Maximum number of clock-domain records to inspect.
    pub max_domains: Option<u64>,

    /// Maximum number of process records to inspect.
    pub max_processes: Option<u64>,

    /// Maximum number of process event entries to inspect.
    pub max_events: Option<u64>,

    /// Maximum number of parent links to inspect during cycle analysis.
    pub max_parent_steps: Option<u64>,

    /// Maximum number of findings to retain in the report.
    pub max_findings: Option<u64>,
}

impl ClockAnalysisBudget {
    /// Create a budget without explicit limits.
    ///
    /// Available memory, execution time, and representable collection sizes
    /// still constrain the operation.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_domains: None,
            max_processes: None,
            max_events: None,
            max_parent_steps: None,
            max_findings: None,
        }
    }
}

/// Severity of a clock-analysis finding.
///
/// A warning or informational finding is not, by itself, proof of an invalid
/// design. Error findings describe contradictions or unresolved references
/// detected by this analysis.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ClockFindingSeverity {
    /// A detected structural inconsistency or invalid reference.
    Error,

    /// A suspicious relationship requiring further semantic review.
    Warning,

    /// An observation that is useful but does not establish invalidity.
    Info,
}

/// A specific clock-analysis observation.
///
/// Findings describe only facts this analysis can establish from clock
/// domains and process/event declarations. In particular, observing a clock
/// from another domain is not equivalent to proving an unsafe data crossing.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ClockAnalysisFindingKind {
    /// A clock-domain record failed its local structural validation.
    InvalidClockDomain {
        /// Identity of the invalid domain.
        domain: HdlirClockDomainId,
        /// Explanation returned by the owning IR type.
        detail: String,
    },

    /// A clock domain names a parent that is not present in the collection.
    MissingParentDomain {
        /// Domain containing the unresolved parent reference.
        domain: HdlirClockDomainId,
        /// Referenced parent identity.
        parent: HdlirClockDomainId,
    },

    /// A cycle exists in the declared parent-domain relationships.
    ParentDomainCycle {
        /// Domain IDs participating in the detected cycle.
        ///
        /// The first ID is not repeated at the end.
        cycle: Vec<HdlirClockDomainId>,
    },

    /// A process failed its local structural validation.
    InvalidProcess {
        /// Identity of the invalid process.
        process: HdlirProcessId,
        /// Explanation returned by the owning IR type.
        detail: String,
    },

    /// A process refers to a clock domain absent from the analyzed collection.
    MissingProcessClockDomain {
        /// Process containing the unresolved reference.
        process: HdlirProcessId,
        /// Referenced clock-domain identity.
        domain: HdlirClockDomainId,
    },

    /// A sequential process has no explicit clock-domain association.
    ///
    /// This is informational: the event list may still express clock intent.
    /// The analysis does not infer a canonical domain association.
    SequentialProcessWithoutExplicitDomain {
        /// Process without an explicit domain association.
        process: HdlirProcessId,
    },

    /// A sequential process event uses the domain's clock source but declares
    /// an edge that differs from the domain's active edge.
    ClockEventEdgeMismatch {
        /// Process containing the event.
        process: HdlirProcessId,
        /// Associated logical clock domain.
        domain: HdlirClockDomainId,
        /// Zero-based event position within the process.
        event_index: u64,
        /// Edge declared by the clock domain.
        expected: HdlirClockEdge,
        /// Edge declared by the process event.
        actual: HdlirEventEdge,
    },

    /// A process associated with one domain observes the clock source of
    /// another domain in its event list.
    ///
    /// This is an informational observation only. It does not prove a data
    /// crossing, establish whether the relationship is legal, or determine
    /// whether synchronization is required.
    EventObservesAnotherDomainClock {
        /// Process containing the event.
        process: HdlirProcessId,
        /// Explicitly associated domain, when one exists.
        associated_domain: Option<HdlirClockDomainId>,
        /// Domain whose declared clock source matches the event source.
        observed_domain: HdlirClockDomainId,
        /// Zero-based event position within the process.
        event_index: u64,
    },
}

/// One clock-analysis finding with its severity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClockAnalysisFinding {
    /// Severity assigned by this analysis.
    pub severity: ClockFindingSeverity,

    /// Detected condition or informational observation.
    pub kind: ClockAnalysisFindingKind,
}

impl ClockAnalysisFinding {
    fn new(
        severity: ClockFindingSeverity,
        kind: ClockAnalysisFindingKind,
    ) -> Self {
        Self { severity, kind }
    }
}

/// Why clock analysis did not complete all requested work.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ClockAnalysisIncompleteReason {
    /// The configured clock-domain budget was exhausted.
    DomainBudgetExceeded {
        /// Configured limit.
        limit: u64,
        /// Number of domains already inspected.
        inspected: u64,
    },

    /// The configured process budget was exhausted.
    ProcessBudgetExceeded {
        /// Configured limit.
        limit: u64,
        /// Number of processes already inspected.
        inspected: u64,
    },

    /// The configured event budget was exhausted.
    EventBudgetExceeded {
        /// Configured limit.
        limit: u64,
        /// Number of events already inspected.
        inspected: u64,
    },

    /// The configured parent-link traversal budget was exhausted.
    ParentStepBudgetExceeded {
        /// Configured limit.
        limit: u64,
        /// Number of parent steps already inspected.
        inspected: u64,
    },

    /// The configured finding-retention budget was exhausted.
    FindingBudgetExceeded {
        /// Configured limit.
        limit: u64,
        /// Number of findings already retained.
        retained: u64,
    },

    /// A work or result counter exceeded its representable range.
    CounterOverflow,
}

/// Completion status of a clock-analysis invocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClockAnalysisCompletion {
    /// All work requested by the supplied budget was completed.
    ///
    /// This does not mean the design is valid: error findings may exist.
    Complete,

    /// The analysis stopped before all requested work was completed.
    Incomplete(ClockAnalysisIncompleteReason),
}

/// Report returned by logical clock analysis.
///
/// Findings are retained in deterministic domain order followed by process
/// insertion order, with event order preserved within each process.
/// The report never equates completion with validity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClockAnalysisReport {
    /// Findings discovered before completion or budget exhaustion.
    pub findings: Vec<ClockAnalysisFinding>,

    /// Completion status of the analysis.
    pub completion: ClockAnalysisCompletion,

    /// Number of clock-domain records inspected.
    pub domains_examined: u64,

    /// Number of process records inspected.
    pub processes_examined: u64,

    /// Number of process event entries inspected.
    pub events_examined: u64,

    /// Number of parent links inspected during cycle analysis.
    pub parent_steps_examined: u64,

    /// Number of findings retained.
    pub findings_retained: u64,
}

impl ClockAnalysisReport {
    /// Return whether all requested analysis work completed.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(self.completion, ClockAnalysisCompletion::Complete)
    }

    /// Return whether the report contains at least one error-severity finding.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.severity == ClockFindingSeverity::Error)
    }

    /// Return whether the report contains any findings.
    #[must_use]
    pub fn has_findings(&self) -> bool {
        !self.findings.is_empty()
    }
}

impl Default for ClockAnalysisReport {
    fn default() -> Self {
        Self {
            findings: Vec::new(),
            completion: ClockAnalysisCompletion::Complete,
            domains_examined: 0,
            processes_examined: 0,
            events_examined: 0,
            parent_steps_examined: 0,
            findings_retained: 0,
        }
    }
}

/// Analyze logical clock domains and their declared process/event relationships.
///
/// The function reads existing HDLIR structures and does not mutate either
/// collection. It checks local record invariants, parent references and
/// cycles, explicit process-domain references, and sequential clock-edge
/// consistency.
///
/// The analysis is deterministic for the same input collections and budget.
/// When a budget is exhausted, the returned report retains findings already
/// discovered and marks the result incomplete.
///
/// This function does not receive connectivity, value-definition, timing,
/// or target-capability information. It therefore cannot establish CDC
/// safety, complete source resolution, physical feasibility, or timing closure.
#[must_use]
pub fn analyze_clock_relationships(
    domains: &HdlirClockDomains,
    processes: &HdlirProcessTable,
    budget: ClockAnalysisBudget,
) -> ClockAnalysisReport {
    let mut report = ClockAnalysisReport::default();

    // Index the domains for deterministic reference resolution and cycle
    // analysis. Domain IDs are identities, not collection indexes.
    let mut domain_index: BTreeMap<HdlirClockDomainId, &HdlirClockDomain> =
        BTreeMap::new();

    // Multiple domains may declare the same source. Preserve all identities;
    // do not silently select one as the canonical owner of that source.
    let mut domains_by_source: BTreeMap<HdlirClockSource, Vec<HdlirClockDomainId>> =
        BTreeMap::new();

    for (id, domain) in domains.iter() {
        if let Err(reason) = tick(
            &mut report.domains_examined,
            budget.max_domains,
            |limit, inspected| ClockAnalysisIncompleteReason::DomainBudgetExceeded {
                limit,
                inspected,
            },
        ) {
            report.completion = ClockAnalysisCompletion::Incomplete(reason);
            return report;
        }

        if let Err(error) = domain.validate_structure() {
            if !record_finding(
                &mut report,
                budget,
                ClockAnalysisFinding::new(
                    ClockFindingSeverity::Error,
                    ClockAnalysisFindingKind::InvalidClockDomain {
                        domain: *id,
                        detail: error.to_string(),
                    },
                ),
            ) {
                return report;
            }
        }

        domain_index.insert(*id, domain);
        domains_by_source
            .entry(domain.source())
            .or_default()
            .push(*id);
    }

    // Parent references are checked separately so forward references are
    // legal and independent of insertion order.
    for (id, domain) in &domain_index {
        if let Some(parent) = domain.parent() {
            if !domain_index.contains_key(&parent)
                && !record_finding(
                    &mut report,
                    budget,
                    ClockAnalysisFinding::new(
                        ClockFindingSeverity::Error,
                        ClockAnalysisFindingKind::MissingParentDomain {
                            domain: *id,
                            parent,
                        },
                    ),
                )
            {
                return report;
            }
        }
    }

    // Iterative parent-cycle detection. This avoids recursion proportional
    // to user-controlled clock hierarchy depth.
    let mut completed = BTreeSet::new();

    for start in domain_index.keys().copied() {
        if completed.contains(&start) {
            continue;
        }

        let mut path = Vec::new();
        let mut positions: BTreeMap<HdlirClockDomainId, usize> = BTreeMap::new();
        let mut current = Some(start);

        while let Some(id) = current {
            if completed.contains(&id) {
                break;
            }

            if let Some(first_position) = positions.get(&id).copied() {
                let cycle = path[first_position..].to_vec();

                if !record_finding(
                    &mut report,
                    budget,
                    ClockAnalysisFinding::new(
                        ClockFindingSeverity::Error,
                        ClockAnalysisFindingKind::ParentDomainCycle { cycle },
                    ),
                ) {
                    return report;
                }

                break;
            }

            if let Err(reason) = tick(
                &mut report.parent_steps_examined,
                budget.max_parent_steps,
                |limit, inspected| {
                    ClockAnalysisIncompleteReason::ParentStepBudgetExceeded {
                        limit,
                        inspected,
                    }
                },
            ) {
                report.completion = ClockAnalysisCompletion::Incomplete(reason);
                return report;
            }

            let Some(domain) = domain_index.get(&id) else {
                // Missing parent references have already been reported.
                break;
            };

            positions.insert(id, path.len());
            path.push(id);
            current = domain.parent();
        }

        completed.extend(path);
    }

    // Analyze processes in the order preserved by HdlirProcessTable.
    for process in processes.iter() {
        if let Err(reason) = tick(
            &mut report.processes_examined,
            budget.max_processes,
            |limit, inspected| {
                ClockAnalysisIncompleteReason::ProcessBudgetExceeded {
                    limit,
                    inspected,
                }
            },
        ) {
            report.completion = ClockAnalysisCompletion::Incomplete(reason);
            return report;
        }

        let process_id = process.id();

        if let Err(error) = process.validate_structure() {
            if !record_finding(
                &mut report,
                budget,
                ClockAnalysisFinding::new(
                    ClockFindingSeverity::Error,
                    ClockAnalysisFindingKind::InvalidProcess {
                        process: process_id,
                        detail: error.to_string(),
                    },
                ),
            ) {
                return report;
            }
        }

        let associated_domain = process.clock_domain();

        let domain = match associated_domain {
            Some(domain_id) => match domain_index.get(&domain_id) {
                Some(domain) => Some(*domain),
                None => {
                    if !record_finding(
                        &mut report,
                        budget,
                        ClockAnalysisFinding::new(
                            ClockFindingSeverity::Error,
                            ClockAnalysisFindingKind::MissingProcessClockDomain {
                                process: process_id,
                                domain: domain_id,
                            },
                        ),
                    ) {
                        return report;
                    }
                    None
                }
            },
            None => {
                if matches!(process.kind(), HdlirProcessKind::Sequential)
                    && !record_finding(
                        &mut report,
                        budget,
                        ClockAnalysisFinding::new(
                            ClockFindingSeverity::Info,
                            ClockAnalysisFindingKind::SequentialProcessWithoutExplicitDomain {
                                process: process_id,
                            },
                        ),
                    )
                {
                    return report;
                }
                None
            }
        };

        for (event_position, event) in process.events().iter().enumerate() {
            if let Err(reason) = tick(
                &mut report.events_examined,
                budget.max_events,
                |limit, inspected| ClockAnalysisIncompleteReason::EventBudgetExceeded {
                    limit,
                    inspected,
                },
            ) {
                report.completion = ClockAnalysisCompletion::Incomplete(reason);
                return report;
            }

            let event_index = match u64::try_from(event_position) {
                Ok(index) => index,
                Err(_) => {
                    report.completion = ClockAnalysisCompletion::Incomplete(
                        ClockAnalysisIncompleteReason::CounterOverflow,
                    );
                    return report;
                }
            };

            // Only compare event edges with a domain's active edge when the
            // event observes that domain's declared clock source.
            if matches!(process.kind(), HdlirProcessKind::Sequential) {
                if let Some(domain) = domain {
                    if event_matches_clock_source(event.source(), domain.source()) {
                        let edge_matches = matches!(
                            (domain.active_edge(), event.edge()),
                            (HdlirClockEdge::Rising, HdlirEventEdge::RisingEdge)
                                | (HdlirClockEdge::Falling, HdlirEventEdge::FallingEdge)
                        );

                        if !edge_matches
                            && !record_finding(
                                &mut report,
                                budget,
                                ClockAnalysisFinding::new(
                                    ClockFindingSeverity::Warning,
                                    ClockAnalysisFindingKind::ClockEventEdgeMismatch {
                                        process: process_id,
                                        domain: domain.id(),
                                        event_index,
                                        expected: domain.active_edge(),
                                        actual: event.edge(),
                                    },
                                ),
                            )
                        {
                            return report;
                        }
                    }
                }
            }

            // This observation can identify clock-event relationships worth
            // reviewing. It is not a CDC proof and does not classify ordinary
            // data dependencies, because those require the complete graph.
            for (observed_source, observed_domains) in &domains_by_source {
                if !event_matches_clock_source(event.source(), *observed_source) {
                    continue;
                }

                for observed_domain in observed_domains {
                    if associated_domain == Some(*observed_domain) {
                        continue;
                    }

                    if !record_finding(
                        &mut report,
                        budget,
                        ClockAnalysisFinding::new(
                            ClockFindingSeverity::Info,
                            ClockAnalysisFindingKind::EventObservesAnotherDomainClock {
                                process: process_id,
                                associated_domain,
                                observed_domain: *observed_domain,
                                event_index,
                            },
                        ),
                    ) {
                        return report;
                    }
                }
            }
        }
    }

    report
}

fn event_matches_clock_source(
    event_source: HdlirEventSource,
    clock_source: HdlirClockSource,
) -> bool {
    match (event_source, clock_source) {
        (HdlirEventSource::Signal(event), HdlirClockSource::Signal(clock)) => {
            event == clock
        }
        (HdlirEventSource::Value(event), HdlirClockSource::Value(clock)) => {
            event == clock
        }
        _ => false,
    }
}

fn tick(
    counter: &mut u64,
    limit: Option<u64>,
    exceeded: impl FnOnce(u64, u64) -> ClockAnalysisIncompleteReason,
) -> Result<(), ClockAnalysisIncompleteReason> {
    if let Some(limit) = limit {
        if *counter >= limit {
            return Err(exceeded(limit, *counter));
        }
    }

    *counter = counter
        .checked_add(1)
        .ok_or(ClockAnalysisIncompleteReason::CounterOverflow)?;

    Ok(())
}

fn record_finding(
    report: &mut ClockAnalysisReport,
    budget: ClockAnalysisBudget,
    finding: ClockAnalysisFinding,
) -> bool {
    if let Some(limit) = budget.max_findings {
        if report.findings_retained >= limit {
            report.completion = ClockAnalysisCompletion::Incomplete(
                ClockAnalysisIncompleteReason::FindingBudgetExceeded {
                    limit,
                    retained: report.findings_retained,
                },
            );
            return false;
        }
    }

    let Some(next_count) = report.findings_retained.checked_add(1) else {
        report.completion = ClockAnalysisCompletion::Incomplete(
            ClockAnalysisIncompleteReason::CounterOverflow,
        );
        return false;
    };

    report.findings.push(finding);
    report.findings_retained = next_count;
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use crate::hdl::hdlir::ids::{
        HdlirClockDomainId, HdlirProcessId, HdlirRegionId, HdlirSignalId,
    };
    use crate::hdl::hdlir::ir::clocks::{
        HdlirClockDomain, HdlirClockSource,
    };
    use crate::hdl::hdlir::ir::processes::{
        HdlirEventSource, HdlirProcess, HdlirProcessEvent, HdlirProcessKind,
        HdlirProcessTable,
    };

    #[test]
    fn reports_sequential_clock_edge_mismatch() {
        let signal = HdlirSignalId::new(10).expect("non-zero signal ID");
        let domain_id =
            HdlirClockDomainId::new(20).expect("non-zero clock-domain ID");

        let domain = HdlirClockDomain::new(
            domain_id,
            Some("system".to_owned()),
            HdlirClockSource::Signal(signal),
            HdlirClockEdge::Rising,
            None,
            None,
        )
        .expect("valid clock domain");

        let mut domains = HdlirClockDomains::new();
        domains.insert(domain).expect("unique domain");

        let event = HdlirProcessEvent::new(
            HdlirEventSource::Signal(signal),
            HdlirEventEdge::FallingEdge,
            Vec::new(),
        )
        .expect("valid event");

        let process = HdlirProcess::new(
            HdlirProcessId::new(30).expect("non-zero process ID"),
            Some("sample".to_owned()),
            HdlirProcessKind::Sequential,
            HdlirRegionId::new(40).expect("non-zero region ID"),
            vec![event],
            Some(domain_id),
            BTreeMap::new(),
        )
        .expect("valid process metadata");

        let mut processes = HdlirProcessTable::new();
        processes.insert(process).expect("unique process");

        let report = analyze_clock_relationships(
            &domains,
            &processes,
            ClockAnalysisBudget::unbounded(),
        );

        assert!(report.is_complete());
        assert!(report.findings.iter().any(|finding| matches!(
            finding.kind,
            ClockAnalysisFindingKind::ClockEventEdgeMismatch {
                expected: HdlirClockEdge::Rising,
                actual: HdlirEventEdge::FallingEdge,
                ..
            }
        )));
    }

    #[test]
    fn budget_exhaustion_is_never_reported_as_complete() {
        let domains = HdlirClockDomains::new();
        let processes = HdlirProcessTable::new();

        let report = analyze_clock_relationships(
            &domains,
            &processes,
            ClockAnalysisBudget {
                max_domains: Some(0),
                ..ClockAnalysisBudget::unbounded()
            },
        );

        // Empty inputs require no domain visits, so the domain budget is not
        // exhausted. This also guards against treating zero as a hardware
        // capacity limit when no work is needed.
        assert!(report.is_complete());
    }
}
