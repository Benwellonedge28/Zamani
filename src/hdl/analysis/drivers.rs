
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # HDL Driver Analysis
//!
//! Path: `src/hdl/analysis/drivers.rs`
//! Rust: 1.97+
//! Edition: 2021
//! Safety: safe Rust only.
//!
//! ## Purpose
//!
//! Analyze which source-level HDL processes drive each declared signal.
//! Produce deterministic driver summaries and structured findings for
//! multiple drivers, undriven signals, attempts to drive input ports,
//! unresolved targets, and ambiguous inout resolution.
//!
//! ## Authority and ownership
//!
//! - `grammar/spec/hdl.md` owns the normative HDL architecture.
//! - `grammar/hdl/` owns HDL source syntax.
//! - The canonical frontend AST owns parsed source structure.
//! - `crate::frontend::semantic::domains::hdl` owns the normalized HDL
//!   semantic input types and `validate_hdl()`.
//! - `src/hdl/analysis/dependencies.rs` owns dependency graph analysis.
//! - `src/hdl/hdlir/` remains the exclusive future home for HDLIR.
//!
//! This module reuses `HdlProgram`, `HdlSignal`, `HdlProcess`,
//! `HdlAssignment`, `HdlId`, `HdlDirection`, and `HdlProcessKind`. It does
//! not create another AST, signal type, source-span system, semantic
//! validator, dependency graph, or intermediate representation.
//!
//! ## Analysis semantics
//!
//! A driver owner is a distinct process identity, not an assignment.
//! Multiple assignments to one signal within one process therefore count
//! as one owner. This module does not attempt to prove that procedural
//! branches are mutually exclusive; that requires richer control-flow facts.
//!
//! An assignment marked `continuous` is counted separately from procedural
//! assignments for reporting, but its enclosing process identity remains
//! the ownership identity because the current semantic contract does not
//! provide a separate stable assignment identity.
//!
//! Driving an input port is reported as an error. An output with no driver
//! is reported as an error. Undriven internal signals can be reported by
//! configuration. An undriven `inout` is reported as indeterminate.
//!
//! Multiple owners of an ordinary signal are reported as an error.
//! Multiple `inout` owners are reported as indeterminate: the current
//! semantic input does not describe a resolved-net type, tri-state drive
//! conditions, or a resolution function. This module must not guess that
//! such drivers are legal.
//!
//! ## Boundaries
//!
//! This is a read-only analysis, not a repair pass or replacement for
//! `validate_hdl()`. Run semantic validation separately. This module does
//! not infer physical drivers, select target primitives, allocate hardware,
//! prove branch exclusivity, implement tri-state resolution, or implement
//! HDLIR.
//!
//! ## Scalability and incomplete analysis
//!
//! There are no universal signal, process, or assignment ceilings. Optional
//! budgets are caller-configured operational limits. `None` means that
//! this API imposes no limit for that dimension; it does not promise
//! infinite host memory or execution time.
//!
//! A budget stop produces `DriverAnalysisStatus::Incomplete`. The result
//! may contain useful positive findings, but incomplete analysis MUST NOT
//! be used to conclude that a signal is undriven, that no multiple drivers
//! exist, or that a design is safe.
//!
//! Ordered maps and sets provide deterministic output. Traversal is
//! iterative over the supplied program, with storage proportional to the
//! declarations and driver relationships processed.
//!
//! ## Integration
//!
//! Input: the existing normalized `HdlProgram`.
//! Output: one summary per processed, uniquely identified signal, plus
//! ordered findings and an explicit completion status.
//!
//! The future `src/hdl/analysis/mod.rs` should declare `pub mod drivers;`
//! and re-export only the APIs intended to be stable. `src/hdl/mod.rs`
//! should register `analysis` when its composition root is integrated.
//! Do not modify `src/hdl/hdlir/` for this work.

use std::collections::{BTreeMap, BTreeSet};

use crate::frontend::semantic::domains::hdl::{
    HdlDirection, HdlId, HdlProcessKind, HdlProgram, HdlSignal,
    HdlSourceLocation,
};

/// Operational budgets for driver analysis.
///
/// These limits constrain work performed by this analysis, not the legal
/// size or semantics of an HDL program. `None` means no caller-specified
/// limit for that dimension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriverAnalysisConfig {
    /// Maximum number of signal declarations to inspect.
    pub max_signals: Option<usize>,
    /// Maximum number of assignments to inspect.
    pub max_assignments: Option<usize>,
    /// Maximum number of signal, process, and assignment work units.
    pub max_work: Option<usize>,
    /// Whether to emit findings for undriven internal signals.
    pub report_undriven_internal: bool,
}

impl Default for DriverAnalysisConfig {
    fn default() -> Self {
        Self {
            max_signals: None,
            max_assignments: None,
            max_work: None,
            report_undriven_internal: true,
        }
    }
}

/// Why driver analysis could not finish.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DriverAnalysisIncompleteReason {
    /// The configured signal-inspection budget was exhausted.
    SignalBudgetExceeded,
    /// The configured assignment-inspection budget was exhausted.
    AssignmentBudgetExceeded,
    /// The configured total-work budget was exhausted.
    WorkBudgetExceeded,
}

/// Completion state of driver analysis.
///
/// `Complete` means all signals, processes, and assignments in the supplied
/// semantic input were inspected. It does not mean the design is valid;
/// findings must still be examined.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DriverAnalysisStatus {
    /// The requested driver analysis completed.
    Complete,
    /// Analysis stopped before all relevant input was inspected.
    Incomplete(DriverAnalysisIncompleteReason),
}

impl DriverAnalysisStatus {
    /// Returns true only if all relevant input was inspected.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Severity of a driver-analysis finding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DriverFindingSeverity {
    /// The finding identifies a driver relationship that violates the
    /// current source-level contract.
    Error,
    /// The finding identifies a suspicious or potentially unintended
    /// driver relationship.
    Warning,
    /// The available semantic input is insufficient to decide legality.
    Indeterminate,
}

/// Kind of driver-analysis finding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DriverFindingKind {
    /// More than one distinct process drives an ordinary signal.
    MultipleDrivers,
    /// A declared output has no observed driver.
    UndrivenOutput,
    /// An internal signal has no observed driver.
    UndrivenInternal,
    /// An inout signal has no observed driver.
    UndrivenInOut,
    /// An input port is assigned by a process.
    InputDriven,
    /// An assignment target is absent from the supplied declarations.
    UnknownTarget,
    /// The same signal identity occurs more than once.
    DuplicateSignalIdentity,
    /// The same process identity occurs more than once.
    DuplicateProcessIdentity,
    /// Multiple inout owners exist, but resolution semantics are absent.
    InOutResolutionUnspecified,
}

/// One structured finding from driver analysis.
///
/// Locations refer to the existing semantic source-location contract. They
/// are not a new span or source-map representation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DriverFinding {
    /// Stable finding category.
    pub kind: DriverFindingKind,
    /// Severity appropriate to the available evidence.
    pub severity: DriverFindingSeverity,
    /// Signal identity, if the finding is associated with a known signal.
    pub signal: Option<HdlId>,
    /// Process identity, if the finding is associated with one process.
    pub process: Option<HdlId>,
    /// Human-readable explanation.
    pub message: String,
    /// Primary source location, if known.
    pub location: Option<HdlSourceLocation>,
    /// Other relevant assignment or declaration locations.
    pub related: Vec<HdlSourceLocation>,
}

/// A distinct process that contributes assignments to a signal.
///
/// The owner is keyed by the canonical process identity. Assignment counts
/// distinguish continuous and procedural forms without treating repeated
/// assignments in one process as separate process owners.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriverOwner {
    /// Canonical identity of the owning process.
    pub process_id: HdlId,
    /// Kind of the owning process.
    pub process_kind: HdlProcessKind,
    /// Number of assignments marked as continuous in this process.
    pub continuous_assignments: usize,
    /// Number of assignments not marked as continuous in this process.
    pub procedural_assignments: usize,
    /// Source locations of the assignments, in deterministic order.
    pub locations: Vec<HdlSourceLocation>,
}

impl DriverOwner {
    fn new(process_id: HdlId, process_kind: HdlProcessKind) -> Self {
        Self {
            process_id,
            process_kind,
            continuous_assignments: 0,
            procedural_assignments: 0,
            locations: Vec::new(),
        }
    }

    fn record_assignment(
        &mut self,
        continuous: bool,
        location: Option<&HdlSourceLocation>,
    ) {
        if continuous {
            self.continuous_assignments =
                self.continuous_assignments.saturating_add(1);
        } else {
            self.procedural_assignments =
                self.procedural_assignments.saturating_add(1);
        }

        if let Some(location) = location {
            self.locations.push(location.clone());
            self.locations.sort();
            self.locations.dedup();
        }
    }
}

/// Driver ownership classification for one signal.
///
/// `Indeterminate` means the available input is not sufficient to classify
/// the ownership relationship as definitively legal or illegal.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DriverDisposition {
    /// No driver was observed in a complete assignment scan.
    Undriven,
    /// Exactly one distinct process owner was observed.
    SingleDriver,
    /// More than one distinct process owner was observed for an ordinary
    /// signal.
    MultipleDrivers,
    /// The ownership relationship cannot be classified reliably.
    Indeterminate,
}

/// Driver summary for one uniquely identified declared signal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignalDriverSummary {
    /// Signal identity from the canonical semantic model.
    pub signal: HdlId,
    /// Source-level signal name for reporting.
    pub signal_name: String,
    /// Declared port or internal direction.
    pub direction: HdlDirection,
    /// Distinct process owners, ordered by process identity.
    pub owners: Vec<DriverOwner>,
    /// Ownership classification based on inspected input.
    ///
    /// If the overall analysis is incomplete, `Undriven` is not used to
    /// assert that no uninspected assignment exists.
    pub disposition: DriverDisposition,
}

/// Complete result of one driver-analysis invocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriverAnalysis {
    /// Completion status. Always inspect this before relying on absences.
    pub status: DriverAnalysisStatus,
    /// Summaries for declarations whose identities could be represented
    /// uniquely in the processed input.
    pub signals: Vec<SignalDriverSummary>,
    /// Findings ordered deterministically for stable diagnostics and tests.
    pub findings: Vec<DriverFinding>,
    /// Number of input items charged against the total-work budget.
    pub work_units: usize,
    /// Number of assignments inspected.
    pub assignments_inspected: usize,
    /// Number of signal declarations inspected.
    pub signals_inspected: usize,
}

impl DriverAnalysis {
    /// Returns true only when analysis completed and emitted no error
    /// findings. Indeterminate findings are not treated as proof of safety.
    #[must_use]
    pub fn has_no_errors(&self) -> bool {
        self.status.is_complete()
            && !self.findings.iter().any(|finding| {
                finding.severity == DriverFindingSeverity::Error
            })
            && !self.findings.iter().any(|finding| {
                finding.severity == DriverFindingSeverity::Indeterminate
            })
    }
}

/// Analyze source-level driver ownership in a normalized HDL program.
///
/// This function is deterministic for equivalent semantic input. It does
/// not mutate the program and does not replace `validate_hdl()`.
///
/// The analysis uses process identity as the driver-owner identity because
/// the current `HdlAssignment` contract does not provide a separate stable
/// identity for continuous assignments. An assignment targeting an unknown
/// signal is reported, rather than silently ignored.
///
/// If a budget is reached, the result is marked incomplete. In that case,
/// the absence of a finding is not evidence that the corresponding problem
/// does not exist.
#[must_use]
pub fn analyze_drivers(
    program: &HdlProgram,
    config: &DriverAnalysisConfig,
) -> DriverAnalysis {
    let mut signals_by_id = BTreeMap::<HdlId, &HdlSignal>::new();
    let mut findings = Vec::<DriverFinding>::new();
    let mut duplicate_signal_ids = BTreeSet::<HdlId>::new();
    let mut duplicate_process_ids = BTreeSet::<HdlId>::new();

    let mut drivers =
        BTreeMap::<HdlId, BTreeMap<HdlId, DriverOwner>>::new();

    let mut signals_inspected = 0_usize;
    let mut assignments_inspected = 0_usize;
    let mut work_units = 0_usize;
    let mut stop_reason = None;

    // Inspect declarations first so assignment targets can be checked
    // without inventing signal identities or mutating the source program.
    for signal in &program.signals {
        if limit_reached(config.max_signals, signals_inspected) {
            stop_reason =
                Some(DriverAnalysisIncompleteReason::SignalBudgetExceeded);
            break;
        }
        if limit_reached(config.max_work, work_units) {
            stop_reason =
                Some(DriverAnalysisIncompleteReason::WorkBudgetExceeded);
            break;
        }

        signals_inspected = signals_inspected.saturating_add(1);
        work_units = work_units.saturating_add(1);

        if signals_by_id.contains_key(&signal.id) {
            duplicate_signal_ids.insert(signal.id.clone());
            findings.push(DriverFinding {
                kind: DriverFindingKind::DuplicateSignalIdentity,
                severity: DriverFindingSeverity::Error,
                signal: Some(signal.id.clone()),
                process: None,
                message: format!(
                    "signal identity `{}` is declared more than once",
                    signal.id
                ),
                location: signal.location.clone(),
                related: signals_by_id
                    .get(&signal.id)
                    .and_then(|existing| existing.location.clone())
                    .into_iter()
                    .collect(),
            });
        } else {
            signals_by_id.insert(signal.id.clone(), signal);
        }
    }

    // Do not proceed into process assignments if declaration inspection
    // was stopped. Doing so could incorrectly classify unknown targets.
    if stop_reason.is_none() {
        'processes: for process in &program.processes {
            if limit_reached(config.max_work, work_units) {
                stop_reason =
                    Some(DriverAnalysisIncompleteReason::WorkBudgetExceeded);
                break;
            }
            work_units = work_units.saturating_add(1);

            if !duplicate_process_ids.insert(process.id.clone()) {
                findings.push(DriverFinding {
                    kind: DriverFindingKind::DuplicateProcessIdentity,
                    severity: DriverFindingSeverity::Error,
                    signal: None,
                    process: Some(process.id.clone()),
                    message: format!(
                        "process identity `{}` is declared more than once; \
                         driver ownership by identity is ambiguous",
                        process.id
                    ),
                    location: process.location.clone(),
                    related: Vec::new(),
                });
            }

            for assignment in &process.assignments {
                if limit_reached(
                    config.max_assignments,
                    assignments_inspected,
                ) {
                    stop_reason = Some(
                        DriverAnalysisIncompleteReason::AssignmentBudgetExceeded,
                    );
                    break 'processes;
                }
                if limit_reached(config.max_work, work_units) {
                    stop_reason =
                        Some(DriverAnalysisIncompleteReason::WorkBudgetExceeded);
                    break 'processes;
                }

                assignments_inspected =
                    assignments_inspected.saturating_add(1);
                work_units = work_units.saturating_add(1);

                let Some(signal) = signals_by_id.get(&assignment.target) else {
                    findings.push(DriverFinding {
                        kind: DriverFindingKind::UnknownTarget,
                        severity: DriverFindingSeverity::Error,
                        signal: Some(assignment.target.clone()),
                        process: Some(process.id.clone()),
                        message: format!(
                            "process `{}` assigns unknown signal `{}`",
                            process.id, assignment.target
                        ),
                        location: assignment
                            .location
                            .clone()
                            .or_else(|| process.location.clone()),
                        related: Vec::new(),
                    });
                    continue;
                };

                if signal.direction == HdlDirection::Input {
                    findings.push(DriverFinding {
                        kind: DriverFindingKind::InputDriven,
                        severity: DriverFindingSeverity::Error,
                        signal: Some(signal.id.clone()),
                        process: Some(process.id.clone()),
                        message: format!(
                            "process `{}` attempts to drive input signal `{}`",
                            process.id, signal.name
                        ),
                        location: assignment
                            .location
                            .clone()
                            .or_else(|| signal.location.clone()),
                        related: Vec::new(),
                    });
                }

                let owner = drivers
                    .entry(assignment.target.clone())
                    .or_default()
                    .entry(process.id.clone())
                    .or_insert_with(|| {
                        DriverOwner::new(process.id.clone(), process.kind)
                    });

                owner.record_assignment(
                    assignment.continuous,
                    assignment.location.as_ref(),
                );
            }
        }
    }

    let status = match stop_reason {
        Some(reason) => DriverAnalysisStatus::Incomplete(reason),
        None => DriverAnalysisStatus::Complete,
    };

    let mut summaries = Vec::new();

    for (signal_id, signal) in &signals_by_id {
        let owners_by_id = drivers.get(signal_id);
        let owners: Vec<DriverOwner> = owners_by_id
            .into_iter()
            .flat_map(|owners| owners.values().cloned())
            .collect();

        let ambiguous_owner = owners
            .iter()
            .any(|owner| duplicate_process_ids.contains(&owner.process_id));
        let duplicate_signal = duplicate_signal_ids.contains(signal_id);

        let disposition = if ambiguous_owner || duplicate_signal {
            DriverDisposition::Indeterminate
        } else if owners.is_empty() {
            if status.is_complete() {
                DriverDisposition::Undriven
            } else {
                // An incomplete assignment scan cannot establish absence.
                DriverDisposition::Indeterminate
            }
        } else if owners.len() == 1 {
            DriverDisposition::SingleDriver
        } else if signal.direction == HdlDirection::InOut {
            DriverDisposition::Indeterminate
        } else {
            DriverDisposition::MultipleDrivers
        };

        if status.is_complete() && owners.is_empty() {
            match signal.direction {
                HdlDirection::Output => findings.push(DriverFinding {
                    kind: DriverFindingKind::UndrivenOutput,
                    severity: DriverFindingSeverity::Error,
                    signal: Some(signal_id.clone()),
                    process: None,
                    message: format!(
                        "output signal `{}` has no driver",
                        signal.name
                    ),
                    location: signal.location.clone(),
                    related: Vec::new(),
                }),
                HdlDirection::Internal if config.report_undriven_internal => {
                    findings.push(DriverFinding {
                        kind: DriverFindingKind::UndrivenInternal,
                        severity: DriverFindingSeverity::Warning,
                        signal: Some(signal_id.clone()),
                        process: None,
                        message: format!(
                            "internal signal `{}` has no driver",
                            signal.name
                        ),
                        location: signal.location.clone(),
                        related: Vec::new(),
                    });
                }
                HdlDirection::InOut => findings.push(DriverFinding {
                    kind: DriverFindingKind::UndrivenInOut,
                    severity: DriverFindingSeverity::Indeterminate,
                    signal: Some(signal_id.clone()),
                    process: None,
                    message: format!(
                        "inout signal `{}` has no driver; the available \
                         semantic input does not establish whether this is \
                         permitted by its interface contract",
                        signal.name
                    ),
                    location: signal.location.clone(),
                    related: Vec::new(),
                }),
                HdlDirection::Input | HdlDirection::Internal => {}
            }
        }

        if owners.len() > 1 && !ambiguous_owner && !duplicate_signal {
            let related = owner_locations(&owners);
            let primary = related.first().cloned().or_else(|| signal.location.clone());

            if signal.direction == HdlDirection::InOut {
                findings.push(DriverFinding {
                    kind: DriverFindingKind::InOutResolutionUnspecified,
                    severity: DriverFindingSeverity::Indeterminate,
                    signal: Some(signal_id.clone()),
                    process: None,
                    message: format!(
                        "inout signal `{}` has multiple process owners, \
                         but the semantic input does not describe its \
                         resolution or tri-state rules",
                        signal.name
                    ),
                    location: primary,
                    related,
                });
            } else {
                findings.push(DriverFinding {
                    kind: DriverFindingKind::MultipleDrivers,
                    severity: DriverFindingSeverity::Error,
                    signal: Some(signal_id.clone()),
                    process: None,
                    message: format!(
                        "signal `{}` is driven by multiple distinct processes",
                        signal.name
                    ),
                    location: primary,
                    related,
                });
            }
        }

        summaries.push(SignalDriverSummary {
            signal: signal_id.clone(),
            signal_name: signal.name.clone(),
            direction: signal.direction,
            owners,
            disposition,
        });
    }

    findings.sort();
    findings.dedup();

    DriverAnalysis {
        status,
        signals: summaries,
        findings,
        work_units,
        assignments_inspected,
        signals_inspected,
    }
}

fn limit_reached(limit: Option<usize>, current: usize) -> bool {
    limit.is_some_and(|maximum| current >= maximum)
}

fn owner_locations(owners: &[DriverOwner]) -> Vec<HdlSourceLocation> {
    let mut locations: Vec<HdlSourceLocation> = owners
        .iter()
        .flat_map(|owner| owner.locations.iter().cloned())
        .collect();
    locations.sort();
    locations.dedup();
    locations
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::domains::hdl::{
        HdlAssignment, HdlInputError, HdlValueType,
    };

    fn id(value: &str) -> HdlId {
        HdlId::new(value).expect("test identifier must be valid")
    }

    fn signal(name: &str, direction: HdlDirection) -> HdlSignal {
        HdlSignal {
            id: id(name),
            name: name.to_owned(),
            direction,
            value_type: HdlValueType::Bit,
            location: None,
        }
    }

    fn assignment(target: &str) -> HdlAssignment {
        HdlAssignment {
            target: id(target),
            reads: Vec::new(),
            continuous: false,
            location: None,
        }
    }

    fn process(name: &str, assignments: Vec<HdlAssignment>) -> crate::frontend::semantic::domains::hdl::HdlProcess {
        crate::frontend::semantic::domains::hdl::HdlProcess {
            id: id(name),
            kind: HdlProcessKind::Combinational,
            assignments,
            clocks: Vec::new(),
            resets: Vec::new(),
            location: None,
        }
    }

    #[test]
    fn repeated_assignments_in_one_process_are_one_driver() {
        let program = HdlProgram {
            signals: vec![signal("out", HdlDirection::Output)],
            processes: vec![process(
                "p",
                vec![assignment("out"), assignment("out")],
            )],
            ..HdlProgram::default()
        };

        let result =
            analyze_drivers(&program, &DriverAnalysisConfig::default());

        assert_eq!(result.status, DriverAnalysisStatus::Complete);
        assert_eq!(result.signals.len(), 1);
        assert_eq!(
            result.signals[0].disposition,
            DriverDisposition::SingleDriver
        );
        assert_eq!(result.signals[0].owners.len(), 1);
        assert_eq!(
            result.signals[0].owners[0].procedural_assignments,
            2
        );
        assert!(!result.findings.iter().any(|finding| {
            finding.kind == DriverFindingKind::MultipleDrivers
        }));
    }

    #[test]
    fn distinct_processes_driving_one_output_are_reported() {
        let program = HdlProgram {
            signals: vec![signal("out", HdlDirection::Output)],
            processes: vec![
                process("p1", vec![assignment("out")]),
                process("p2", vec![assignment("out")]),
            ],
            ..HdlProgram::default()
        };

        let result =
            analyze_drivers(&program, &DriverAnalysisConfig::default());

        assert_eq!(
            result.signals[0].disposition,
            DriverDisposition::MultipleDrivers
        );
        assert!(result.findings.iter().any(|finding| {
            finding.kind == DriverFindingKind::MultipleDrivers
                && finding.severity == DriverFindingSeverity::Error
        }));
        assert!(!result.has_no_errors());
    }

    #[test]
    fn undriven_output_is_reported_after_complete_scan() {
        let program = HdlProgram {
            signals: vec![signal("out", HdlDirection::Output)],
            ..HdlProgram::default()
        };

        let result =
            analyze_drivers(&program, &DriverAnalysisConfig::default());

        assert!(result.status.is_complete());
        assert!(result.findings.iter().any(|finding| {
            finding.kind == DriverFindingKind::UndrivenOutput
        }));
    }

    #[test]
    fn input_port_cannot_be_driven() {
        let program = HdlProgram {
            signals: vec![signal("input", HdlDirection::Input)],
            processes: vec![process("p", vec![assignment("input")])],
            ..HdlProgram::default()
        };

        let result =
            analyze_drivers(&program, &DriverAnalysisConfig::default());

        assert!(result.findings.iter().any(|finding| {
            finding.kind == DriverFindingKind::InputDriven
                && finding.severity == DriverFindingSeverity::Error
        }));
    }

    #[test]
    fn multiple_inout_owners_are_indeterminate_not_assumed_legal() {
        let program = HdlProgram {
            signals: vec![signal("bus", HdlDirection::InOut)],
            processes: vec![
                process("p1", vec![assignment("bus")]),
                process("p2", vec![assignment("bus")]),
            ],
            ..HdlProgram::default()
        };

        let result =
            analyze_drivers(&program, &DriverAnalysisConfig::default());

        assert_eq!(
            result.signals[0].disposition,
            DriverDisposition::Indeterminate
        );
        assert!(result.findings.iter().any(|finding| {
            finding.kind == DriverFindingKind::InOutResolutionUnspecified
                && finding.severity == DriverFindingSeverity::Indeterminate
        }));
        assert!(!result.has_no_errors());
    }

    #[test]
    fn incomplete_assignment_scan_does_not_claim_outputs_are_undriven() {
        let program = HdlProgram {
            signals: vec![signal("out", HdlDirection::Output)],
            processes: vec![process("p", vec![assignment("out")])],
            ..HdlProgram::default()
        };
        let config = DriverAnalysisConfig {
            max_assignments: Some(0),
            ..DriverAnalysisConfig::default()
        };

        let result = analyze_drivers(&program, &config);

        assert_eq!(
            result.status,
            DriverAnalysisStatus::Incomplete(
                DriverAnalysisIncompleteReason::AssignmentBudgetExceeded
            )
        );
        assert!(!result.findings.iter().any(|finding| {
            finding.kind == DriverFindingKind::UndrivenOutput
        }));
        assert_eq!(
            result.signals[0].disposition,
            DriverDisposition::Indeterminate
        );
    }

    #[test]
    fn zero_work_budget_stops_before_inspecting_signals() {
        let program = HdlProgram {
            signals: vec![signal("out", HdlDirection::Output)],
            ..HdlProgram::default()
        };
        let config = DriverAnalysisConfig {
            max_work: Some(0),
            ..DriverAnalysisConfig::default()
        };

        let result = analyze_drivers(&program, &config);

        assert_eq!(
            result.status,
            DriverAnalysisStatus::Incomplete(
                DriverAnalysisIncompleteReason::WorkBudgetExceeded
            )
        );
        assert_eq!(result.signals_inspected, 0);
        assert!(result.findings.is_empty());
    }

    #[test]
    fn unknown_assignment_target_is_reported() {
        let program = HdlProgram {
            processes: vec![process("p", vec![assignment("missing")])],
            ..HdlProgram::default()
        };

        let result =
            analyze_drivers(&program, &DriverAnalysisConfig::default());

        assert!(result.findings.iter().any(|finding| {
            finding.kind == DriverFindingKind::UnknownTarget
        }));
    }

    #[test]
    fn continuous_and_procedural_assignments_are_counted_separately() {
        let mut continuous = assignment("out");
        continuous.continuous = true;

        let program = HdlProgram {
            signals: vec![signal("out", HdlDirection::Output)],
            processes: vec![process(
                "p",
                vec![continuous, assignment("out")],
            )],
            ..HdlProgram::default()
        };

        let result =
            analyze_drivers(&program, &DriverAnalysisConfig::default());

        let owner = &result.signals[0].owners[0];
        assert_eq!(owner.continuous_assignments, 1);
        assert_eq!(owner.procedural_assignments, 1);
        assert_eq!(result.signals[0].owners.len(), 1);
    }

    // Keep this import referenced so compiler diagnostics in the test module
    // catch accidental changes to the canonical input-error API.
    #[test]
    fn canonical_identifier_contract_remains_in_use() {
        let error = HdlId::new(" ").expect_err("blank IDs must be rejected");
        assert_eq!(error, HdlInputError::EmptyIdentifier);
    }
}
