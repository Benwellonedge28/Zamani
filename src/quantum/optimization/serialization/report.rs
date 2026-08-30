//! Zamani Quantum Optimization — Optimization Report Serialization
//!
//! Production serialization boundary for optimization results.
//!
//! # Architectural role
//!
//! This module converts the canonical
//! `crate::quantum::optimization::result::OptimizationResult` into a stable,
//! versioned, machine-readable JSON report.
//!
//! The dependency direction is:
//!
//! ```text
//!                    quantum::ir
//!                        │
//!                        ▼
//!                  optimization
//!                        │
//!                        ▼
//!                OptimizationResult
//!                        │
//!                        ▼
//!             serialization::report
//!                        │
//!                  ┌─────┴─────┐
//!                  ▼           ▼
//!                JSON      JSON Pretty
//! ```
//!
//! This module does NOT:
//!
//! - optimize circuits;
//! - mutate Quantum IR;
//! - perform verification;
//! - perform routing;
//! - perform scheduling;
//! - access hardware;
//! - access a QPU;
//! - access the filesystem unless a caller explicitly supplies a writer;
//! - access the network;
//! - execute external programs;
//! - define another quantum circuit representation;
//! - define another qubit identifier;
//! - depend on optimization passes;
//! - depend on benchmarking;
//! - depend on hardware backends.
//!
//! # Canonical representation
//!
//! The canonical semantic circuit remains:
//!
//! ```text
//! crate::quantum::ir::QuantumCircuit
//! ```
//!
//! `OptimizationResult` owns that circuit. This report intentionally does not
//! duplicate or reinterpret the circuit because the canonical Quantum IR is
//! owned by `quantum::ir`.
//!
//! The report instead serializes the observational result metadata:
//!
//! - optimization status;
//! - summary;
//! - resource metrics;
//! - cost snapshots;
//! - verification state;
//! - pass results;
//! - diagnostics;
//! - provenance;
//! - stable schema metadata.
//!
//! A separate circuit serializer/exporter should serialize the actual circuit
//! when required.
//!
//! # Stable schema
//!
//! Every report contains:
//!
//! - `schema`;
//! - `schema_version`;
//! - `format_version`;
//! - `report_kind`;
//! - `circuit_representation`;
//! - `status`;
//! - `summary`;
//! - `metrics`;
//! - `cost`;
//! - `verification`;
//! - `passes`;
//! - `diagnostics`;
//! - `provenance`.
//!
//! Schema versioning is deliberately independent from the optimizer version.
//! A new optimization pass does not require a schema change unless the
//! externally serialized report contract itself changes.
//!
//! # Scalability
//!
//! Zamani optimization is intended to scale from tiny circuits to workloads
//! constrained only by available resources.
//!
//! This serializer therefore provides two important properties:
//!
//! 1. `write_json` and `write_json_pretty` serialize directly into a caller-
//!    supplied `Write` implementation.
//! 2. The serializer does not construct an intermediate `serde_json::Value`
//!    containing the complete report.
//!
//! Consequently, report serialization does not require a second complete copy
//! of the report in memory.
//!
//! The caller remains responsible for choosing an appropriate output sink.
//! For example, a caller may supply a buffered file writer, network stream,
//! memory buffer, or another streaming sink.
//!
//! The size of the pass/diagnostic arrays is still determined by the
//! `OptimizationResult` supplied to this module. This module does not impose
//! an artificial maximum on them.
//!
//! # Numeric safety
//!
//! Optimization counters are represented by the result subsystem using `u128`.
//! They are serialized without narrowing to `u64` or `usize`.
//!
//! Floating-point values are passed through Serde's JSON serializer. JSON does
//! not represent NaN or infinity. If an invalid non-finite value reaches the
//! report, serialization fails rather than silently emitting an invalid or
//! misleading value.
//!
//! # Determinism
//!
//! The report serializer itself introduces no timestamps, random values,
//! process IDs, memory addresses, filesystem paths, or environment variables.
//!
//! Report ordering follows the ordering already present in `OptimizationResult`.
//! In particular, pass and diagnostic arrays preserve their recorded order.
//!
//! # Thread safety
//!
//! This module contains no global mutable state.
//!
//! `OptimizationReport` only borrows an `OptimizationResult`, so independent
//! reports can be serialized concurrently when their underlying results are
//! independently accessible.
//!
//! # Security
//!
//! This module:
//!
//! - contains no `unsafe` code;
//! - forbids unsafe code explicitly;
//! - performs no external I/O by itself;
//! - does not execute arbitrary code;
//! - does not deserialize executable content;
//! - does not interpret optimization rules;
//! - does not trust report text as compiler instructions.
//!
//! A caller serializing untrusted optimization metadata should still apply
//! external output-size policies appropriate for its environment.
//!
//! # Quantum IR / Qubit boundary
//!
//! This module intentionally does NOT import:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! because this report does not serialize individual qubit identities.
//!
//! Qubit counts are already represented by `OptimizationMetrics`.
//!
//! If a future report field genuinely needs an individual qubit identity, it
//! MUST use the canonical:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! and must not introduce a report-local qubit identifier.
//!
//! # Integration contract
//!
//! `result.rs`
//!     owns the optimization result model.
//!
//! `pipeline.rs`
//!     produces the result and populates its fields.
//!
//! `provenance.rs`
//!     supplies provenance information.
//!
//! `verification/*`
//!     supplies verification information.
//!
//! `statistics.rs`
//!     supplies detailed optimization accounting that is summarized in the
//!     result.
//!
//! `serialization::config`
//!     serializes optimization configuration independently.
//!
//! `serialization::provenance`
//!     may serialize provenance independently.
//!
//! `serialization::report`
//!     serializes the terminal optimization result.
//!
//! `benchmarking`
//!     may consume this report but is not a dependency of this module.
//!
//! `routing`, `scheduling`, and `hardware`
//!     may consume the optimized circuit separately and do not become
//!     dependencies of this serializer.
//!
//! New optimization passes do not need to modify this file.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! Existing Cargo dependencies are sufficient:
//!
//! - `serde`;
//! - `serde_json`.
//!
//! No Cargo.toml modification is required for this file.
//!
//! # Public API
//!
//! ```text
//! OptimizationReport::new
//! OptimizationReport::result
//! OptimizationReport::to_json_string
//! OptimizationReport::to_json_pretty_string
//! OptimizationReport::write_json
//! OptimizationReport::write_json_pretty
//! serialize_json
//! serialize_json_pretty
//! write_json
//! write_json_pretty
//! ```
//!
//! The string-returning APIs are convenient for small reports.
//!
//! The writer APIs are the preferred APIs for very large optimization runs.

#![forbid(unsafe_code)]

use serde::ser::{
    Serialize,
    SerializeStruct,
};
use serde::Serializer;
use serde_json;
use std::fmt;
use std::io::{self, Write};

use crate::quantum::optimization::result::{
    OptimizationCostSnapshot,
    OptimizationDiagnostic,
    OptimizationMetrics,
    OptimizationResult,
    OptimizationStatus,
    OptimizationSummary,
    PassOutcome,
    PassPhase,
    PassResult,
    ProvenanceSnapshot,
    VerificationStatus,
    VerificationSummary,
};

// =============================================================================
// Stable schema constants
// =============================================================================

/// Stable identifier for optimization result reports.
pub const REPORT_SCHEMA: &str = "zamani.quantum.optimization.report";

/// Current report schema version.
///
/// Increment only when the externally visible report structure or semantics
/// change in a way that requires a compatibility decision.
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

/// Stable serializer format version.
///
/// This is intentionally independent from the optimization algorithm version.
pub const FORMAT_VERSION: u32 = 1;

/// Stable report kind.
pub const REPORT_KIND: &str = "optimization_result";

/// Canonical circuit representation named by this report.
pub const CIRCUIT_REPRESENTATION: &str =
    "crate::quantum::ir::QuantumCircuit";

// =============================================================================
// Serialization error
// =============================================================================

/// Errors produced while serializing an optimization report.
#[derive(Debug)]
pub enum ReportSerializationError {
    /// The destination writer rejected output.
    Io(io::Error),

    /// Serde/JSON serialization failed.
    Json(serde_json::Error),
}

impl fmt::Display for ReportSerializationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => {
                write!(
                    formatter,
                    "optimization report I/O error: {error}"
                )
            }

            Self::Json(error) => {
                write!(
                    formatter,
                    "optimization report JSON serialization error: {error}"
                )
            }
        }
    }
}

impl std::error::Error for ReportSerializationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Json(error) => Some(error),
        }
    }
}

impl From<io::Error> for ReportSerializationError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for ReportSerializationError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

// =============================================================================
// Public report wrapper
// =============================================================================

/// Borrowed, serializable view of an [`OptimizationResult`].
///
/// This type deliberately borrows the result rather than cloning it.
///
/// The optimized `QuantumCircuit` itself remains owned by the canonical
/// `OptimizationResult` and is not duplicated into this report representation.
#[derive(Debug, Clone, Copy)]
pub struct OptimizationReport<'a> {
    result: &'a OptimizationResult,
}

impl<'a> OptimizationReport<'a> {
    /// Creates a report view over an optimization result.
    #[must_use]
    pub const fn new(result: &'a OptimizationResult) -> Self {
        Self { result }
    }

    /// Returns the underlying optimization result.
    #[must_use]
    pub const fn result(&self) -> &'a OptimizationResult {
        self.result
    }

    /// Serializes this report as compact JSON.
    ///
    /// This convenience method is appropriate for small to moderate reports.
    ///
    /// For very large optimization results, prefer [`Self::write_json`] so
    /// the caller can control the output sink.
    pub fn to_json_string(
        &self,
    ) -> Result<String, ReportSerializationError> {
        serde_json::to_string(self)
            .map_err(ReportSerializationError::Json)
    }

    /// Serializes this report as human-readable JSON.
    ///
    /// For very large reports, prefer [`Self::write_json_pretty`] so the caller
    /// can stream directly to an output sink.
    pub fn to_json_pretty_string(
        &self,
    ) -> Result<String, ReportSerializationError> {
        serde_json::to_string_pretty(self)
            .map_err(ReportSerializationError::Json)
    }

    /// Streams compact JSON into the supplied writer.
    ///
    /// No complete intermediate JSON `Value` is constructed.
    pub fn write_json<W: Write>(
        &self,
        writer: W,
    ) -> Result<(), ReportSerializationError> {
        serde_json::to_writer(writer, self)
            .map_err(ReportSerializationError::Json)
    }

    /// Streams human-readable JSON into the supplied writer.
    ///
    /// No complete intermediate JSON `Value` is constructed.
    pub fn write_json_pretty<W: Write>(
        &self,
        writer: W,
    ) -> Result<(), ReportSerializationError> {
        serde_json::to_writer_pretty(writer, self)
            .map_err(ReportSerializationError::Json)
    }
}

// =============================================================================
// Serde implementation
// =============================================================================

impl Serialize for OptimizationReport<'_> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let result = self.result;

        let mut state =
            serializer.serialize_struct("OptimizationReport", 13)?;

        state.serialize_field("schema", REPORT_SCHEMA)?;
        state.serialize_field(
            "schema_version",
            &CURRENT_SCHEMA_VERSION,
        )?;
        state.serialize_field("format_version", &FORMAT_VERSION)?;
        state.serialize_field("report_kind", REPORT_KIND)?;
        state.serialize_field(
            "circuit_representation",
            CIRCUIT_REPRESENTATION,
        )?;
        state.serialize_field("status", &StatusView(result.status()))?;
        state.serialize_field(
            "summary",
            &SummaryView(result.summary()),
        )?;
        state.serialize_field(
            "metrics",
            &MetricsView(result.metrics()),
        )?;
        state.serialize_field(
            "cost",
            &CostView(result.cost()),
        )?;
        state.serialize_field(
            "verification",
            &VerificationView(result.verification()),
        )?;
        state.serialize_field(
            "passes",
            &PassesView(result.passes()),
        )?;
        state.serialize_field(
            "diagnostics",
            &DiagnosticsView(result.diagnostics()),
        )?;
        state.serialize_field(
            "provenance",
            &ProvenanceView(result.provenance()),
        )?;

        state.end()
    }
}

// =============================================================================
// Top-level enum view
// =============================================================================

struct StatusView(OptimizationStatus);

impl Serialize for StatusView {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self.0 {
            OptimizationStatus::Optimized => "optimized",
            OptimizationStatus::Unchanged => "unchanged",
            OptimizationStatus::PartiallyOptimized => {
                "partially_optimized"
            }
            OptimizationStatus::LimitReached => "limit_reached",
            OptimizationStatus::VerificationFailed => {
                "verification_failed"
            }
            OptimizationStatus::Failed => "failed",
        })
    }
}

// =============================================================================
// Summary
// =============================================================================

struct SummaryView<'a>(&'a OptimizationSummary);

impl Serialize for SummaryView<'_> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = self.0;

        let mut state =
            serializer.serialize_struct("OptimizationSummary", 14)?;

        state.serialize_field(
            "passes_requested",
            &value.passes_requested,
        )?;
        state.serialize_field(
            "passes_executed",
            &value.passes_executed,
        )?;
        state.serialize_field(
            "passes_changed",
            &value.passes_changed,
        )?;
        state.serialize_field(
            "passes_skipped",
            &value.passes_skipped,
        )?;
        state.serialize_field(
            "rewrites_applied",
            &value.rewrites_applied,
        )?;
        state.serialize_field(
            "operations_before",
            &value.operations_before,
        )?;
        state.serialize_field(
            "operations_after",
            &value.operations_after,
        )?;
        state.serialize_field(
            "operations_removed",
            &value.operations_removed(),
        )?;
        state.serialize_field(
            "two_qubit_operations_before",
            &value.two_qubit_operations_before,
        )?;
        state.serialize_field(
            "two_qubit_operations_after",
            &value.two_qubit_operations_after,
        )?;
        state.serialize_field(
            "two_qubit_operations_removed",
            &value.two_qubit_operations_removed(),
        )?;
        state.serialize_field("depth_before", &value.depth_before)?;
        state.serialize_field("depth_after", &value.depth_after)?;
        state.serialize_field(
            "depth_reduction",
            &value.depth_reduction(),
        )?;

        state.end()
    }
}

// =============================================================================
// Metrics
// =============================================================================

struct MetricsView<'a>(&'a OptimizationMetrics);

impl Serialize for MetricsView<'_> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = self.0;

        let mut state =
            serializer.serialize_struct("OptimizationMetrics", 16)?;

        state.serialize_field(
            "operations_removed",
            &value.operations_removed,
        )?;
        state.serialize_field(
            "operations_added",
            &value.operations_added,
        )?;
        state.serialize_field(
            "operations_replaced",
            &value.operations_replaced,
        )?;
        state.serialize_field(
            "two_qubit_operations_removed",
            &value.two_qubit_operations_removed,
        )?;
        state.serialize_field(
            "t_gates_removed",
            &value.t_gates_removed,
        )?;
        state.serialize_field(
            "t_gates_added",
            &value.t_gates_added,
        )?;
        state.serialize_field("depth_before", &value.depth_before)?;
        state.serialize_field("depth_after", &value.depth_after)?;
        state.serialize_field(
            "depth_reduction",
            &value.depth_reduction(),
        )?;
        state.serialize_field(
            "t_depth_before",
            &value.t_depth_before,
        )?;
        state.serialize_field(
            "t_depth_after",
            &value.t_depth_after,
        )?;
        state.serialize_field(
            "t_depth_reduction",
            &value.t_depth_reduction(),
        )?;
        state.serialize_field("qubits_before", &value.qubits_before)?;
        state.serialize_field("qubits_after", &value.qubits_after)?;
        state.serialize_field("improved", &value.improved())?;
        state.serialize_field(
            "elapsed",
            &DurationView(value.elapsed),
        )?;

        state.end()
    }
}

// =============================================================================
// Duration
// =============================================================================

struct DurationView(std::time::Duration);

impl Serialize for DurationView {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state =
            serializer.serialize_struct("Duration", 3)?;

        state.serialize_field("seconds", &self.0.as_secs())?;
        state.serialize_field("nanoseconds", &self.0.subsec_nanos())?;
        state.serialize_field(
            "total_nanoseconds",
            &self.0.as_nanos(),
        )?;

        state.end()
    }
}

// =============================================================================
// Cost
// =============================================================================

struct CostView<'a>(&'a OptimizationCostSnapshot);

impl Serialize for CostView<'_> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = self.0;

        let mut state =
            serializer.serialize_struct("OptimizationCostSnapshot", 12)?;

        state.serialize_field("has_initial", &value.has_initial)?;
        state.serialize_field("has_final", &value.has_final)?;
        state.serialize_field(
            "initial_total",
            &value.initial_total,
        )?;
        state.serialize_field(
            "final_total",
            &value.final_total,
        )?;
        state.serialize_field(
            "initial_two_qubit",
            &value.initial_two_qubit,
        )?;
        state.serialize_field(
            "final_two_qubit",
            &value.final_two_qubit,
        )?;
        state.serialize_field(
            "initial_depth",
            &value.initial_depth,
        )?;
        state.serialize_field(
            "final_depth",
            &value.final_depth,
        )?;
        state.serialize_field(
            "initial_t_cost",
            &value.initial_t_cost,
        )?;
        state.serialize_field(
            "final_t_cost",
            &value.final_t_cost,
        )?;
        state.serialize_field(
            "has_comparable_totals",
            &value.has_comparable_totals(),
        )?;
        state.serialize_field(
            "total_delta",
            &value.total_delta(),
        )?;

        state.end()
    }
}

// =============================================================================
// Verification
// =============================================================================

struct VerificationView<'a>(&'a VerificationSummary);

impl Serialize for VerificationView<'_> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = self.0;

        let mut state =
            serializer.serialize_struct("VerificationSummary", 6)?;

        state.serialize_field(
            "status",
            &VerificationStatusView(value.status),
        )?;
        state.serialize_field("checks", &value.checks)?;
        state.serialize_field("confidence", &value.confidence)?;
        state.serialize_field("tolerance", &value.tolerance)?;
        state.serialize_field("fidelity", &value.fidelity)?;
        state.serialize_field(
            "successful",
            &value.is_successful(),
        )?;

        state.end()
    }
}

struct VerificationStatusView(VerificationStatus);

impl Serialize for VerificationStatusView {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self.0 {
            VerificationStatus::NotPerformed => "not_performed",
            VerificationStatus::Unavailable => "unavailable",
            VerificationStatus::Exact => "exact",
            VerificationStatus::UpToGlobalPhase => {
                "up_to_global_phase"
            }
            VerificationStatus::MeasurementEquivalent => {
                "measurement_equivalent"
            }
            VerificationStatus::Statistical => "statistical",
            VerificationStatus::Approximate => "approximate",
            VerificationStatus::Failed => "failed",
        })
    }
}

// =============================================================================
// Passes
// =============================================================================

struct PassesView<'a>(&'a [PassResult]);

impl Serialize for PassesView<'_> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut sequence =
            serializer.serialize_seq(Some(self.0.len()))?;

        for pass in self.0 {
            sequence.serialize_element(&PassView(pass))?;
        }

        sequence.end()
    }
}

struct PassView<'a>(&'a PassResult);

impl Serialize for PassView<'_> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = self.0;

        let mut state =
            serializer.serialize_struct("PassResult", 11)?;

        state.serialize_field("pass_id", &value.pass_id)?;
        state.serialize_field(
            "phase",
            &PassPhaseView(value.phase),
        )?;
        state.serialize_field(
            "outcome",
            &PassOutcomeView(value.outcome),
        )?;
        state.serialize_field(
            "operations_before",
            &value.operations_before,
        )?;
        state.serialize_field(
            "operations_after",
            &value.operations_after,
        )?;
        state.serialize_field("rewrites", &value.rewrites)?;
        state.serialize_field(
            "operations_removed",
            &value.operations_removed,
        )?;
        state.serialize_field(
            "operations_added",
            &value.operations_added,
        )?;
        state.serialize_field(
            "verification_checks",
            &value.verification_checks,
        )?;
        state.serialize_field(
            "changed",
            &value.outcome.changed(),
        )?;
        state.serialize_field(
            "elapsed",
            &DurationView(value.elapsed),
        )?;

        state.end()
    }
}

struct PassPhaseView(PassPhase);

impl Serialize for PassPhaseView {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self.0 {
            PassPhase::Validation => "validation",
            PassPhase::Normalization => "normalization",
            PassPhase::Local => "local",
            PassPhase::Algebraic => "algebraic",
            PassPhase::Parameter => "parameter",
            PassPhase::Clifford => "clifford",
            PassPhase::PhasePolynomial => {
                "phase_polynomial"
            }
            PassPhase::FaultTolerant => "fault_tolerant",
            PassPhase::Synthesis => "synthesis",
            PassPhase::Structural => "structural",
            PassPhase::TargetAware => "target_aware",
            PassPhase::Search => "search",
            PassPhase::Verification => "verification",
            PassPhase::Other => "other",
        })
    }
}

struct PassOutcomeView(PassOutcome);

impl Serialize for PassOutcomeView {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self.0 {
            PassOutcome::Changed => "changed",
            PassOutcome::Unchanged => "unchanged",
            PassOutcome::Skipped => "skipped",
            PassOutcome::LimitReached => "limit_reached",
            PassOutcome::PartiallyCompleted => {
                "partially_completed"
            }
            PassOutcome::Failed => "failed",
        })
    }
}

// =============================================================================
// Diagnostics
// =============================================================================

struct DiagnosticsView<'a>(&'a [OptimizationDiagnostic]);

impl Serialize for DiagnosticsView<'_> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut sequence =
            serializer.serialize_seq(Some(self.0.len()))?;

        for diagnostic in self.0 {
            sequence.serialize_element(
                &DiagnosticView(diagnostic),
            )?;
        }

        sequence.end()
    }
}

struct DiagnosticView<'a>(&'a OptimizationDiagnostic);

impl Serialize for DiagnosticView<'_> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = self.0;

        let mut state =
            serializer.serialize_struct("OptimizationDiagnostic", 6)?;

        state.serialize_field("code", &value.code)?;
        state.serialize_field(
            "severity",
            &DiagnosticSeverityView(value.severity),
        )?;
        state.serialize_field("message", &value.message)?;
        state.serialize_field("pass_id", &value.pass_id)?;
        state.serialize_field("rule_id", &value.rule_id)?;
        state.serialize_field(
            "operation_id",
            &value.operation_id,
        )?;

        state.end()
    }
}

struct DiagnosticSeverityView(
    crate::quantum::optimization::result::DiagnosticSeverity,
);

impl Serialize for DiagnosticSeverityView {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use crate::quantum::optimization::result::DiagnosticSeverity;

        serializer.serialize_str(match self.0 {
            DiagnosticSeverity::Info => "info",
            DiagnosticSeverity::Warning => "warning",
            DiagnosticSeverity::Error => "error",
        })
    }
}

// =============================================================================
// Provenance
// =============================================================================

struct ProvenanceView<'a>(&'a ProvenanceSnapshot);

impl Serialize for ProvenanceView<'_> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = self.0;

        let mut state =
            serializer.serialize_struct("ProvenanceSnapshot", 8)?;

        state.serialize_field(
            "optimizer_version",
            &value.optimizer_version,
        )?;
        state.serialize_field("profile", &value.profile)?;
        state.serialize_field("target", &value.target)?;
        state.serialize_field(
            "random_seed",
            &value.random_seed,
        )?;
        state.serialize_field(
            "input_hash",
            &value.input_hash,
        )?;
        state.serialize_field(
            "output_hash",
            &value.output_hash,
        )?;
        state.serialize_field(
            "pipeline",
            &value.pipeline,
        )?;
        state.serialize_field(
            "deterministic",
            &value.deterministic,
        )?;

        state.end()
    }
}

// =============================================================================
// Free-function API
// =============================================================================

/// Serializes an optimization result as compact JSON.
///
/// This is equivalent to:
///
/// ```text
/// OptimizationReport::new(result).to_json_string()
/// ```
///
/// For large results, prefer [`write_json`] to avoid requiring the complete
/// JSON document to remain in memory.
pub fn serialize_json(
    result: &OptimizationResult,
) -> Result<String, ReportSerializationError> {
    OptimizationReport::new(result).to_json_string()
}

/// Serializes an optimization result as pretty JSON.
pub fn serialize_json_pretty(
    result: &OptimizationResult,
) -> Result<String, ReportSerializationError> {
    OptimizationReport::new(result).to_json_pretty_string()
}

/// Streams an optimization result as compact JSON into a caller-supplied
/// writer.
///
/// This is the preferred API for large reports.
pub fn write_json<W: Write>(
    result: &OptimizationResult,
    writer: W,
) -> Result<(), ReportSerializationError> {
    OptimizationReport::new(result).write_json(writer)
}

/// Streams an optimization result as pretty JSON into a caller-supplied
/// writer.
///
/// This is useful for human inspection while remaining streaming-oriented.
pub fn write_json_pretty<W: Write>(
    result: &OptimizationResult,
    writer: W,
) -> Result<(), ReportSerializationError> {
    OptimizationReport::new(result).write_json_pretty(writer)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn minimal_result() -> OptimizationResult {
        let circuit = crate::quantum::ir::QuantumCircuit::new();
        OptimizationResult::new(circuit)
    }

    #[test]
    fn report_has_stable_schema_constants() {
        assert_eq!(
            REPORT_SCHEMA,
            "zamani.quantum.optimization.report"
        );
        assert_eq!(CURRENT_SCHEMA_VERSION, 1);
        assert_eq!(FORMAT_VERSION, 1);
        assert_eq!(REPORT_KIND, "optimization_result");
    }

    #[test]
    fn compact_report_is_valid_json() {
        let result = minimal_result();

        let json = serialize_json(&result)
            .expect("minimal result must serialize");

        let value: serde_json::Value =
            serde_json::from_str(&json)
                .expect("serialized report must be valid JSON");

        assert_eq!(
            value["schema"],
            serde_json::Value::String(
                REPORT_SCHEMA.to_owned()
            )
        );

        assert_eq!(
            value["schema_version"],
            serde_json::Value::from(
                CURRENT_SCHEMA_VERSION
            )
        );

        assert_eq!(
            value["format_version"],
            serde_json::Value::from(FORMAT_VERSION)
        );

        assert_eq!(
            value["report_kind"],
            serde_json::Value::String(
                REPORT_KIND.to_owned()
            )
        );

        assert_eq!(
            value["circuit_representation"],
            serde_json::Value::String(
                CIRCUIT_REPRESENTATION.to_owned()
            )
        );

        assert_eq!(
            value["status"],
            serde_json::Value::String(
                "unchanged".to_owned()
            )
        );
    }

    #[test]
    fn pretty_report_is_valid_json() {
        let result = minimal_result();

        let json = serialize_json_pretty(&result)
            .expect("minimal result must serialize");

        assert!(
            json.contains('\n'),
            "pretty JSON should contain line breaks"
        );

        serde_json::from_str::<serde_json::Value>(&json)
            .expect("pretty report must be valid JSON");
    }

    #[test]
    fn writer_api_matches_string_api() {
        let result = minimal_result();

        let string_output = serialize_json(&result)
            .expect("string serialization must succeed");

        let mut buffer = Vec::new();

        write_json(&result, &mut buffer)
            .expect("writer serialization must succeed");

        let writer_output = String::from_utf8(buffer)
            .expect("JSON output must be UTF-8");

        assert_eq!(string_output, writer_output);
    }

    #[test]
    fn pretty_writer_api_produces_valid_json() {
        let result = minimal_result();

        let mut buffer = Vec::new();

        write_json_pretty(&result, &mut buffer)
            .expect("pretty writer serialization must succeed");

        let output = String::from_utf8(buffer)
            .expect("JSON output must be UTF-8");

        serde_json::from_str::<serde_json::Value>(&output)
            .expect("pretty writer output must be valid JSON");
    }

    #[test]
    fn duration_serialization_is_explicit() {
        let duration = Duration::new(12, 345);

        let json = serde_json::to_string(
            &DurationView(duration),
        )
        .expect("duration serialization must succeed");

        let value: serde_json::Value =
            serde_json::from_str(&json)
                .expect("duration JSON must be valid");

        assert_eq!(
            value["seconds"],
            serde_json::Value::from(12_u64)
        );

        assert_eq!(
            value["nanoseconds"],
            serde_json::Value::from(345_u32)
        );

        assert_eq!(
            value["total_nanoseconds"],
            serde_json::Value::from(12_000_000_345_u128)
        );
    }

    #[test]
    fn status_serialization_is_stable() {
        let statuses = [
            (
                OptimizationStatus::Optimized,
                "optimized",
            ),
            (
                OptimizationStatus::Unchanged,
                "unchanged",
            ),
            (
                OptimizationStatus::PartiallyOptimized,
                "partially_optimized",
            ),
            (
                OptimizationStatus::LimitReached,
                "limit_reached",
            ),
            (
                OptimizationStatus::VerificationFailed,
                "verification_failed",
            ),
            (
                OptimizationStatus::Failed,
                "failed",
            ),
        ];

        for (status, expected) in statuses {
            let json = serde_json::to_string(
                &StatusView(status),
            )
            .expect("status serialization must succeed");

            assert_eq!(json, format!("\"{expected}\""));
        }
    }

    #[test]
    fn verification_status_serialization_is_stable() {
        let statuses = [
            (
                VerificationStatus::NotPerformed,
                "not_performed",
            ),
            (
                VerificationStatus::Unavailable,
                "unavailable",
            ),
            (
                VerificationStatus::Exact,
                "exact",
            ),
            (
                VerificationStatus::UpToGlobalPhase,
                "up_to_global_phase",
            ),
            (
                VerificationStatus::MeasurementEquivalent,
                "measurement_equivalent",
            ),
            (
                VerificationStatus::Statistical,
                "statistical",
            ),
            (
                VerificationStatus::Approximate,
                "approximate",
            ),
            (
                VerificationStatus::Failed,
                "failed",
            ),
        ];

        for (status, expected) in statuses {
            let json = serde_json::to_string(
                &VerificationStatusView(status),
            )
            .expect("verification status serialization must succeed");

            assert_eq!(json, format!("\"{expected}\""));
        }
    }

    #[test]
    fn report_does_not_require_qubit_id_representation() {
        // The report summarizes canonical IR metrics rather than introducing
        // a second qubit identifier type.
        //
        // This test intentionally documents the architectural boundary:
        // individual QubitId values belong to quantum::ir::qubit.
        let result = minimal_result();

        let json = serialize_json(&result)
            .expect("report serialization must succeed");

        assert!(
            !json.contains("\"QubitId\""),
            "report must not invent a report-local qubit type"
        );
    }
}