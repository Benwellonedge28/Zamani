
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Connectivity Analysis
//!
//! Path: `src/hdl/analysis/connectivity.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Analyze signal connectivity represented by the existing HDL semantic
//! input contract. Report declared signals, process readers and drivers,
//! source-level dataflow connections, unresolved signal references, and
//! completed-analysis findings for unused or undriven declarations.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/signals.g4`
//! - `grammar/hdl/processes.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/analysis/dependencies.rs`
//!
//! Grammar and the canonical frontend own syntax and source structure.
//! `frontend::semantic::domains::hdl` owns the semantic input contract.
//! `analysis::dependencies` owns combinational dependency-cycle analysis.
//! This module owns connectivity reporting over the available semantic facts.
//!
//! ## Does not own
//!
//! - A replacement AST, parser, semantic model, or source-span system.
//! - Hierarchical module instances or port bindings not present in `HdlProgram`.
//! - A second combinational-cycle detector.
//! - Physical wires, pin assignments, placement, routing, or target selection.
//! - Hardware allocation, synthesis, simulation, or realization.
//! - HDLIR structures, builders, verification, passes, or lowering.
//!
//! ## Edge semantics
//!
//! A `DataflowConnection` records a relationship from a signal read by an
//! assignment to the signal targeted by that assignment. Its kind preserves
//! whether the assignment is continuous, combinational, sequential, or
//! extension-defined. It does not assert that a physical wire exists.
//!
//! Reader and driver sets identify processes, not individual assignments.
//! Multiple assignments in one process therefore count as one process reader
//! or driver for a given signal. Detailed assignment legality and multiple
//! driver validation remain the responsibility of `validate_hdl()`.
//!
//! ## Interpretation
//!
//! Connectivity is computed only from the declarations and assignments
//! supplied in `HdlProgram`. External connections are not represented by
//! this input contract. In particular, an input port is not reported as
//! undriven merely because no internal process drives it, and an output port
//! is not reported as unread merely because its external consumer is absent.
//!
//! `InOut` signals are not diagnosed as unconnected based solely on missing
//! internal readers or drivers because their external relationships are not
//! represented here.
//!
//! Findings about unused or undriven declarations are emitted only after the
//! analysis completes. A partial graph must not be used to infer that a
//! declaration is unused or undriven.
//!
//! ## Scalability and determinism
//!
//! There are no universal signal, process, or connection ceilings. Optional
//! budgets limit analysis work, not the language's semantics. Budget
//! exhaustion is reported explicitly, and a partial result is never marked
//! complete. Ordered collections provide deterministic output ordering.
//!
//! Finite execution remains constrained by available memory, execution time,
//! representable collection sizes, and any caller-configured budgets.
//!
//! ## Integration
//!
//! Input: `crate::frontend::semantic::domains::hdl::HdlProgram`.
//! Output: `ConnectivityReport`.
//!
//! The semantic pipeline should run `validate_hdl()` for source-semantic
//! legality and may run `analyze_connectivity()` for supplementary
//! connectivity facts. Consumers must inspect `report.status` before
//! treating absence of a finding as evidence of connectivity.
//!
//! `analysis/dependencies.rs` remains the owner of the combinational
//! dependency graph and cycle witnesses. This file does not call or duplicate
//! its cycle-finding algorithm.
//!
//! ## Freeze criteria
//!
//! - Public contracts are registered by `analysis/mod.rs`.
//! - Tests cover port-direction interpretation, readers, drivers, unresolved
//!   references, deterministic output, and all operational budgets.
//! - Incomplete analysis cannot produce derived unused/undriven findings.
//! - The native frontend and crate root expose the canonical semantic path.
//! - Formatting, tests, linting, and workspace integration pass.

use std::collections::{BTreeMap, BTreeSet};

use crate::frontend::semantic::domains::hdl::{
    HdlDirection, HdlId, HdlProcessKind, HdlProgram, HdlSourceLocation,
};

/// Configurable operational budgets for connectivity analysis.
///
/// A `None` budget means that this analysis imposes no limit for that
/// dimension. It does not imply that the host has unlimited resources.
///
/// These settings do not constrain legal HDL program sizes.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConnectivityConfig {
    /// Maximum number of distinct signal identities admitted to the report.
    pub max_signals: Option<usize>,

    /// Maximum number of distinct dataflow connections retained.
    pub max_connections: Option<usize>,

    /// Maximum number of deterministic work units consumed.
    ///
    /// Work is charged for each signal declaration, assignment, and signal
    /// reference examined. The same input and configuration use the same
    /// accounting order.
    pub max_work: Option<usize>,
}

/// Why connectivity analysis could not complete.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConnectivityIncompleteReason {
    /// The configured distinct-signal budget was reached.
    SignalBudgetExceeded,

    /// The configured distinct-connection budget was reached.
    ConnectionBudgetExceeded,

    /// The configured work budget was reached.
    WorkBudgetExceeded,
}

/// Completion status of connectivity analysis.
///
/// `Complete` means all declarations and assignment relationships in the
/// supplied input were processed. `Incomplete` means the report may be
/// partial and must not be treated as proof that no other findings exist.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConnectivityStatus {
    /// All requested connectivity analysis completed.
    Complete,

    /// Analysis stopped because an operational budget was reached.
    Incomplete(ConnectivityIncompleteReason),
}

impl ConnectivityStatus {
    /// Returns true only if the analysis completed.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Classification of an assignment-level dataflow connection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConnectionKind {
    /// The source assignment is explicitly marked continuous.
    Continuous,

    /// The connection originates in a combinational process.
    Combinational,

    /// The connection originates in a sequential process.
    Sequential,

    /// The process semantics are supplied by an extension.
    Extension,
}

/// A source-level relationship between a read signal and an assignment target.
///
/// This is a semantic dataflow fact, not a physical wire or routing edge.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DataflowConnection {
    /// Signal read by the assignment expression.
    pub source: HdlId,

    /// Signal assigned by the expression.
    pub target: HdlId,

    /// Process containing the assignment.
    pub process: HdlId,

    /// Assignment relationship kind.
    pub kind: ConnectionKind,

    /// Assignment source location, when available.
    pub location: Option<HdlSourceLocation>,
}

/// Connectivity facts collected for one declared signal.
///
/// `readers` and `drivers` contain distinct process identities. They do not
/// count assignments, bit-level uses, external ports, or physical endpoints.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignalConnectivity {
    /// Canonical semantic identity of the signal.
    pub id: HdlId,

    /// Source-level name retained for diagnostics and reports.
    pub name: String,

    /// Declared source-level direction.
    pub direction: HdlDirection,

    /// Processes that assign this signal.
    pub drivers: BTreeSet<HdlId>,

    /// Processes whose assignment expressions read this signal.
    pub readers: BTreeSet<HdlId>,

    /// Declaration source location, when available.
    pub location: Option<HdlSourceLocation>,
}

impl SignalConnectivity {
    /// Returns whether any represented internal process reads or drives the
    /// signal.
    ///
    /// This method does not account for external connections or physical
    /// endpoints, which are not represented by the current semantic input.
    #[must_use]
    pub fn has_internal_activity(&self) -> bool {
        !self.readers.is_empty() || !self.drivers.is_empty()
    }
}

/// Severity of a connectivity finding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConnectivitySeverity {
    /// A structurally suspicious condition that should be reviewed.
    Warning,

    /// A reference cannot be matched to a declared signal identity.
    Error,

    /// A duplicate identity was encountered in the supplied input.
    ErrorInput,
}

/// Kind of finding emitted by connectivity analysis.
///
/// These findings supplement rather than replace diagnostics from
/// `frontend::semantic::domains::hdl::validate_hdl()`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConnectivityIssueKind {
    /// More than one declaration uses the same semantic signal identity.
    DuplicateSignalIdentity,

    /// An assignment targets a signal not declared in the input.
    UnresolvedAssignmentTarget,

    /// An assignment reads a signal not declared in the input.
    UnresolvedReadSignal,

    /// An output has no represented internal process driver.
    UndrivenOutput,

    /// An internal signal has no represented internal process driver.
    UndrivenInternalSignal,

    /// An input is not read by any represented internal process.
    UnusedInput,

    /// An internal signal is neither read nor driven by a represented process.
    UnusedInternalSignal,
}

/// One structured connectivity finding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConnectivityIssue {
    /// Stable finding category.
    pub kind: ConnectivityIssueKind,

    /// Finding severity.
    pub severity: ConnectivitySeverity,

    /// Signal identity, when known.
    pub signal: Option<HdlId>,

    /// Process identity, when the finding concerns a process.
    pub process: Option<HdlId>,

    /// Best available source location.
    pub location: Option<HdlSourceLocation>,

    /// Human-readable explanation.
    pub message: String,
}

/// Result of connectivity analysis over an `HdlProgram`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConnectivityReport {
    /// Facts for each distinct signal identity admitted to the report.
    ///
    /// When the status is incomplete, this map may be partial.
    pub signals: BTreeMap<HdlId, SignalConnectivity>,

    /// Distinct dataflow connections, in deterministic order.
    ///
    /// When the status is incomplete, this vector may be partial.
    pub connections: Vec<DataflowConnection>,

    /// Findings discovered during analysis.
    ///
    /// If the analysis is incomplete, findings derived from a partial graph
    /// are deliberately omitted. Findings directly observed before stopping
    /// may remain present.
    pub issues: Vec<ConnectivityIssue>,

    /// Whether the report represents a complete analysis.
    pub status: ConnectivityStatus,

    /// Number of work units consumed.
    pub work_units: usize,
}

impl ConnectivityReport {
    /// Returns true only if the analysis completed.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.status.is_complete()
    }

    /// Returns true when analysis completed and emitted no findings.
    ///
    /// This is not a substitute for semantic validation. It means only that
    /// this connectivity analysis completed without emitting its own issues.
    #[must_use]
    pub fn is_clean_and_complete(&self) -> bool {
        self.is_complete() && self.issues.is_empty()
    }
}

/// Analyze signal connectivity represented by the canonical HDL semantic
/// input contract.
///
/// The analysis:
///
/// - Preserves declared signals, including isolated declarations.
/// - Records distinct process readers and drivers.
/// - Records dataflow relationships from each known read signal to a known
///   assignment target.
/// - Reports unresolved assignment targets and read references.
/// - Reports unused inputs, undriven outputs, and unused/undriven internal
///   signals only after a complete traversal.
///
/// Sequential assignments are included as sequential dataflow facts. They
/// are not converted into combinational dependency edges.
///
/// This function does not mutate the input, resolve names, infer missing
/// declarations, or replace `validate_hdl()`. Unknown references remain
/// explicit findings. If a budget is reached, the returned status is
/// incomplete and no derived unused/undriven findings are generated.
#[must_use]
pub fn analyze_connectivity(
    program: &HdlProgram,
    config: &ConnectivityConfig,
) -> ConnectivityReport {
    let mut signals = BTreeMap::<HdlId, SignalConnectivity>::new();
    let mut connections = BTreeSet::<DataflowConnection>::new();
    let mut issues = Vec::<ConnectivityIssue>::new();
    let mut work_units = 0_usize;

    // First admit declarations, retaining isolated signals.
    for signal in &program.signals {
        if budget_reached(config.max_work, work_units) {
            return incomplete_report(
                signals,
                connections,
                issues,
                work_units,
                ConnectivityIncompleteReason::WorkBudgetExceeded,
            );
        }

        if !signals.contains_key(&signal.id)
            && budget_reached(config.max_signals, signals.len())
        {
            return incomplete_report(
                signals,
                connections,
                issues,
                work_units,
                ConnectivityIncompleteReason::SignalBudgetExceeded,
            );
        }

        work_units = work_units.saturating_add(1);

        if signals.contains_key(&signal.id) {
            issues.push(ConnectivityIssue {
                kind: ConnectivityIssueKind::DuplicateSignalIdentity,
                severity: ConnectivitySeverity::ErrorInput,
                signal: Some(signal.id.clone()),
                process: None,
                location: signal.location.clone(),
                message: format!(
                    "duplicate signal identity `{}` in connectivity input",
                    signal.id
                ),
            });

            // Keep the first declaration deterministically. The semantic
            // validator remains responsible for reporting invalid input.
            continue;
        }

        signals.insert(
            signal.id.clone(),
            SignalConnectivity {
                id: signal.id.clone(),
                name: signal.name.clone(),
                direction: signal.direction,
                drivers: BTreeSet::new(),
                readers: BTreeSet::new(),
                location: signal.location.clone(),
            },
        );
    }

    // Collect process-level drivers, readers, and assignment dataflow.
    for process in &program.processes {
        for assignment in &process.assignments {
            if budget_reached(config.max_work, work_units) {
                return incomplete_report(
                    signals,
                    connections,
                    issues,
                    work_units,
                    ConnectivityIncompleteReason::WorkBudgetExceeded,
                );
            }
            work_units = work_units.saturating_add(1);

            let target_is_known = if let Some(target) = signals.get_mut(&assignment.target) {
                target.drivers.insert(process.id.clone());
                true
            } else {
                issues.push(ConnectivityIssue {
                    kind: ConnectivityIssueKind::UnresolvedAssignmentTarget,
                    severity: ConnectivitySeverity::Error,
                    signal: Some(assignment.target.clone()),
                    process: Some(process.id.clone()),
                    location: assignment
                        .location
                        .clone()
                        .or_else(|| process.location.clone()),
                    message: format!(
                        "process `{}` assigns undeclared signal `{}`",
                        process.id, assignment.target
                    ),
                });
                false
            };

            for read_id in &assignment.reads {
                if budget_reached(config.max_work, work_units) {
                    return incomplete_report(
                        signals,
                        connections,
                        issues,
                        work_units,
                        ConnectivityIncompleteReason::WorkBudgetExceeded,
                    );
                }
                work_units = work_units.saturating_add(1);

                if let Some(reader) = signals.get_mut(read_id) {
                    reader.readers.insert(process.id.clone());
                } else {
                    issues.push(ConnectivityIssue {
                        kind: ConnectivityIssueKind::UnresolvedReadSignal,
                        severity: ConnectivitySeverity::Error,
                        signal: Some(read_id.clone()),
                        process: Some(process.id.clone()),
                        location: assignment
                            .location
                            .clone()
                            .or_else(|| process.location.clone()),
                        message: format!(
                            "assignment in process `{}` reads undeclared signal `{read_id}`",
                            process.id
                        ),
                    });
                    continue;
                }

                if !target_is_known {
                    continue;
                }

                let kind = if assignment.continuous {
                    ConnectionKind::Continuous
                } else {
                    match process.kind {
                        HdlProcessKind::Combinational => ConnectionKind::Combinational,
                        HdlProcessKind::Sequential => ConnectionKind::Sequential,
                        HdlProcessKind::Extension => ConnectionKind::Extension,
                    }
                };

                let connection = DataflowConnection {
                    source: read_id.clone(),
                    target: assignment.target.clone(),
                    process: process.id.clone(),
                    kind,
                    location: assignment.location.clone(),
                };

                if !connections.contains(&connection)
                    && budget_reached(config.max_connections, connections.len())
                {
                    return incomplete_report(
                        signals,
                        connections,
                        issues,
                        work_units,
                        ConnectivityIncompleteReason::ConnectionBudgetExceeded,
                    );
                }

                connections.insert(connection);
            }
        }
    }

    // Only completed analysis may infer absence of internal activity.
    for signal in signals.values() {
        match signal.direction {
            HdlDirection::Input => {
                if signal.readers.is_empty() {
                    issues.push(ConnectivityIssue {
                        kind: ConnectivityIssueKind::UnusedInput,
                        severity: ConnectivitySeverity::Warning,
                        signal: Some(signal.id.clone()),
                        process: None,
                        location: signal.location.clone(),
                        message: format!(
                            "input signal `{}` is not read by any represented process",
                            signal.name
                        ),
                    });
                }
            }
            HdlDirection::Output => {
                if signal.drivers.is_empty() {
                    issues.push(ConnectivityIssue {
                        kind: ConnectivityIssueKind::UndrivenOutput,
                        severity: ConnectivitySeverity::Warning,
                        signal: Some(signal.id.clone()),
                        process: None,
                        location: signal.location.clone(),
                        message: format!(
                            "output signal `{}` has no represented internal driver",
                            signal.name
                        ),
                    });
                }
            }
            HdlDirection::Internal => {
                if signal.drivers.is_empty() {
                    issues.push(ConnectivityIssue {
                        kind: ConnectivityIssueKind::UndrivenInternalSignal,
                        severity: ConnectivitySeverity::Warning,
                        signal: Some(signal.id.clone()),
                        process: None,
                        location: signal.location.clone(),
                        message: format!(
                            "internal signal `{}` has no represented internal driver",
                            signal.name
                        ),
                    });
                }

                if signal.readers.is_empty() {
                    issues.push(ConnectivityIssue {
                        kind: ConnectivityIssueKind::UnusedInternalSignal,
                        severity: ConnectivitySeverity::Warning,
                        signal: Some(signal.id.clone()),
                        process: None,
                        location: signal.location.clone(),
                        message: format!(
                            "internal signal `{}` is not read by any represented process",
                            signal.name
                        ),
                    });
                }
            }
            // External inout connectivity cannot be established from this
            // semantic input alone. Avoid claiming it is disconnected.
            HdlDirection::InOut => {}
        }
    }

    issues.sort_by(|left, right| {
        (
            left.signal.as_ref(),
            left.process.as_ref(),
            left.kind,
            left.location.as_ref(),
            &left.message,
        )
            .cmp(&(
                right.signal.as_ref(),
                right.process.as_ref(),
                right.kind,
                right.location.as_ref(),
                &right.message,
            ))
    });

    ConnectivityReport {
        signals,
        connections: connections.into_iter().collect(),
        issues,
        status: ConnectivityStatus::Complete,
        work_units,
    }
}

fn budget_reached(limit: Option<usize>, current: usize) -> bool {
    limit.is_some_and(|maximum| current >= maximum)
}

fn incomplete_report(
    signals: BTreeMap<HdlId, SignalConnectivity>,
    connections: BTreeSet<DataflowConnection>,
    issues: Vec<ConnectivityIssue>,
    work_units: usize,
    reason: ConnectivityIncompleteReason,
) -> ConnectivityReport {
    let mut issues = issues;
    issues.sort_by(|left, right| {
        (
            left.signal.as_ref(),
            left.process.as_ref(),
            left.kind,
            left.location.as_ref(),
            &left.message,
        )
            .cmp(&(
                right.signal.as_ref(),
                right.process.as_ref(),
                right.kind,
                right.location.as_ref(),
                &right.message,
            ))
    });

    ConnectivityReport {
        signals,
        connections: connections.into_iter().collect(),
        issues,
        status: ConnectivityStatus::Incomplete(reason),
        work_units,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::domains::hdl::{
        HdlAssignment, HdlId, HdlSignal, HdlValueType, HdlProcess,
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

    fn assignment(target: &str, reads: &[&str]) -> HdlAssignment {
        HdlAssignment {
            target: id(target),
            reads: reads.iter().map(|value| id(value)).collect(),
            continuous: false,
            location: None,
        }
    }

    fn process(kind: HdlProcessKind, assignments: Vec<HdlAssignment>) -> HdlProcess {
        HdlProcess {
            id: id("process"),
            kind,
            assignments,
            clocks: Vec::new(),
            resets: Vec::new(),
            location: None,
        }
    }

    #[test]
    fn records_readers_drivers_and_dataflow() {
        let program = HdlProgram {
            signals: vec![
                signal("input", HdlDirection::Input),
                signal("output", HdlDirection::Output),
            ],
            processes: vec![process(
                HdlProcessKind::Combinational,
                vec![assignment("output", &["input"])],
            )],
            ..HdlProgram::default()
        };

        let report = analyze_connectivity(&program, &ConnectivityConfig::default());

        assert!(report.is_complete());
        assert_eq!(
            report.signals[&id("input")].readers,
            BTreeSet::from([id("process")])
        );
        assert_eq!(
            report.signals[&id("output")].drivers,
            BTreeSet::from([id("process")])
        );
        assert_eq!(report.connections.len(), 1);
        assert_eq!(report.connections[0].source, id("input"));
        assert_eq!(report.connections[0].target, id("output"));
        assert_eq!(report.connections[0].kind, ConnectionKind::Combinational);
        assert!(report.issues.is_empty());
    }

    #[test]
    fn reports_unused_inputs_and_undriven_outputs() {
        let program = HdlProgram {
            signals: vec![
                signal("unused_input", HdlDirection::Input),
                signal("undriven_output", HdlDirection::Output),
            ],
            ..HdlProgram::default()
        };

        let report = analyze_connectivity(&program, &ConnectivityConfig::default());

        assert!(report.is_complete());
        assert!(report.issues.iter().any(|issue| {
            issue.kind == ConnectivityIssueKind::UnusedInput
                && issue.signal == Some(id("unused_input"))
        }));
        assert!(report.issues.iter().any(|issue| {
            issue.kind == ConnectivityIssueKind::UndrivenOutput
                && issue.signal == Some(id("undriven_output"))
        }));
    }

    #[test]
    fn reports_unknown_read_and_assignment_target() {
        let program = HdlProgram {
            signals: vec![signal("known", HdlDirection::Internal)],
            processes: vec![process(
                HdlProcessKind::Combinational,
                vec![assignment("missing_target", &["missing_read", "known"])],
            )],
            ..HdlProgram::default()
        };

        let report = analyze_connectivity(&program, &ConnectivityConfig::default());

        assert!(report.issues.iter().any(|issue| {
            issue.kind == ConnectivityIssueKind::UnresolvedAssignmentTarget
                && issue.signal == Some(id("missing_target"))
        }));
        assert!(report.issues.iter().any(|issue| {
            issue.kind == ConnectivityIssueKind::UnresolvedReadSignal
                && issue.signal == Some(id("missing_read"))
        }));
        assert!(report.connections.is_empty());
    }

    #[test]
    fn incomplete_analysis_does_not_infer_unused_or_undriven_findings() {
        let program = HdlProgram {
            signals: vec![signal("output", HdlDirection::Output)],
            ..HdlProgram::default()
        };
        let config = ConnectivityConfig {
            max_work: Some(0),
            ..ConnectivityConfig::default()
        };

        let report = analyze_connectivity(&program, &config);

        assert_eq!(
            report.status,
            ConnectivityStatus::Incomplete(
                ConnectivityIncompleteReason::WorkBudgetExceeded
            )
        );
        assert!(!report.issues.iter().any(|issue| {
            issue.kind == ConnectivityIssueKind::UndrivenOutput
        }));
    }

    #[test]
    fn signal_budget_is_explicit() {
        let program = HdlProgram {
            signals: vec![
                signal("a", HdlDirection::Internal),
                signal("b", HdlDirection::Internal),
            ],
            ..HdlProgram::default()
        };
        let config = ConnectivityConfig {
            max_signals: Some(1),
            ..ConnectivityConfig::default()
        };

        let report = analyze_connectivity(&program, &config);

        assert_eq!(
            report.status,
            ConnectivityStatus::Incomplete(
                ConnectivityIncompleteReason::SignalBudgetExceeded
            )
        );
        assert_eq!(report.signals.len(), 1);
    }

    #[test]
    fn connection_budget_is_explicit() {
        let program = HdlProgram {
            signals: vec![
                signal("a", HdlDirection::Input),
                signal("b", HdlDirection::Input),
                signal("out", HdlDirection::Output),
            ],
            processes: vec![process(
                HdlProcessKind::Combinational,
                vec![assignment("out", &["a", "b"])],
            )],
            ..HdlProgram::default()
        };
        let config = ConnectivityConfig {
            max_connections: Some(1),
            ..ConnectivityConfig::default()
        };

        let report = analyze_connectivity(&program, &config);

        assert_eq!(
            report.status,
            ConnectivityStatus::Incomplete(
                ConnectivityIncompleteReason::ConnectionBudgetExceeded
            )
        );
        assert_eq!(report.connections.len(), 1);
        assert!(!report.issues.iter().any(|issue| {
            issue.kind == ConnectivityIssueKind::UnusedInput
                || issue.kind == ConnectivityIssueKind::UndrivenOutput
        }));
    }

    #[test]
    fn sequential_connections_remain_sequential() {
        let program = HdlProgram {
            signals: vec![
                signal("data", HdlDirection::Input),
                signal("state", HdlDirection::Output),
            ],
            processes: vec![process(
                HdlProcessKind::Sequential,
                vec![assignment("state", &["data"])],
            )],
            ..HdlProgram::default()
        };

        let report = analyze_connectivity(&program, &ConnectivityConfig::default());

        assert_eq!(report.connections.len(), 1);
        assert_eq!(report.connections[0].kind, ConnectionKind::Sequential);
    }

    #[test]
    fn equivalent_inputs_produce_identical_reports() {
        let program = HdlProgram {
            signals: vec![
                signal("a", HdlDirection::Input),
                signal("y", HdlDirection::Output),
            ],
            processes: vec![process(
                HdlProcessKind::Combinational,
                vec![assignment("y", &["a"])],
            )],
            ..HdlProgram::default()
        };

        let first = analyze_connectivity(&program, &ConnectivityConfig::default());
        let second = analyze_connectivity(&program, &ConnectivityConfig::default());

        assert_eq!(first, second);
    }
}
