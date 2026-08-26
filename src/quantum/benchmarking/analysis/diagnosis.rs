//! Zamani Quantum Benchmarking — Diagnostic Analysis
//!
//! Production-grade interpretation of benchmark-analysis findings.
//!
//! # Purpose
//!
//! `diagnosis.rs` converts lower-level benchmark findings into structured,
//! deterministic diagnostic conclusions suitable for:
//!
//! - reporting;
//! - CI/CD;
//! - regression dashboards;
//! - the Zamani standard library;
//! - the Zamani programming language;
//! - human-readable engineering diagnosis;
//! - automated remediation planning;
//! - benchmark comparison;
//! - future observability systems.
//!
//! It answers:
//!
//! > "What performance condition does the measured benchmark evidence indicate,
//! > and what should an engineer investigate next?"
//!
//! # Critical scientific boundary
//!
//! Diagnosis is interpretation, NOT causal proof.
//!
//! For example:
//!
//! ```text
//! high execution latency
//!        ↓
//! diagnosis
//!        ↓
//! "execution latency is a significant performance concern"
//! ```
//!
//! It must NOT automatically become:
//!
//! ```text
//! "the scheduler caused the latency"
//! ```
//!
//! Causal attribution belongs to `analysis::attribution` and requires stronger
//! evidence than a single metric finding.
//!
//! # Architectural position
//!
//! ```text
//! benchmark execution
//!        │
//!        ▼
//! core::result::BenchmarkResult
//!        │
//!        ▼
//! analysis::bottleneck
//!        │
//!        ├── bottleneck findings
//!        ├── severity
//!        ├── confidence
//!        └── metric quality
//!        │
//!        ▼
//! analysis::diagnosis
//!        │
//!        ├── diagnostic findings
//!        ├── evidence
//!        ├── investigation priority
//!        ├── confidence
//!        └── recommended investigation areas
//!        │
//!        ├───────────────┐
//!        ▼               ▼
//! reporting          future attribution
//! ```
//!
//! # Dependency boundary
//!
//! This module intentionally depends ONLY on:
//!
//! ```text
//! analysis::bottleneck
//! ```
//!
//! It does NOT depend on:
//!
//! - benchmark protocols;
//! - execution;
//! - hardware;
//! - Quantum IR;
//! - frontend;
//! - algorithms;
//! - routing;
//! - scheduling;
//! - calibration;
//! - reporting;
//! - registry;
//! - baseline;
//! - regression;
//! - attribution.
//!
//! This allows `diagnosis.rs` to be completed before those modules are
//! integrated.
//!
//! # Integration contract
//!
//! The intended integration is:
//!
//! ```text
//! BenchmarkResult
//!       │
//!       ▼
//! BottleneckAnalyzer
//!       │
//!       ▼
//! BottleneckAnalysis
//!       │
//!       ▼
//! DiagnosisAnalyzer
//!       │
//!       ▼
//! DiagnosticReport
//!       │
//!       ├── reporting
//!       ├── CI
//!       ├── baseline/regression
//!       └── Zamani stdlib
//! ```
//!
//! `diagnosis.rs` does not need to be modified when those consumers are added,
//! provided they consume the public API defined here.
//!
//! # Diagnostic philosophy
//!
//! The analyzer follows five rules:
//!
//! 1. Never invent a performance threshold.
//! 2. Never claim causality from correlation alone.
//! 3. Never silently ignore uncertainty.
//! 4. Never silently discard invalid input.
//! 5. Always preserve the original metric evidence.
//!
//! # Diagnostic classes
//!
//! The analyzer can identify conditions including:
//!
//! - critical quality degradation;
//! - significant bottlenecks;
//! - watch-level pressure;
//! - uncertainty requiring more measurement;
//! - missing benchmark policy;
//! - measurement-quality concerns;
//! - error-rate pressure;
//! - fidelity degradation;
//! - latency pressure;
//! - throughput pressure;
//! - resource pressure;
//! - coherence concerns;
//! - leakage concerns;
//! - crosstalk concerns;
//! - drift/stability concerns;
//! - logical/QEC concerns;
//! - scaling concerns;
//! - application-quality concerns.
//!
//! These are interpretations of metric semantics, not universal hardware
//! diagnoses.
//!
//! # Security/resource safety
//!
//! The analyzer:
//!
//! - uses no unsafe code;
//! - performs no network access;
//! - performs no filesystem access;
//! - performs no process execution;
//! - has bounded input/output;
//! - rejects malformed diagnostic identifiers;
//! - rejects non-finite diagnostic scores;
//! - uses deterministic ordering;
//! - uses no global mutable state;
//! - uses no recursion;
//! - does not dynamically execute user supplied expressions.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//!
//! ---------------------------------------------------------------------------

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::fmt;

use super::bottleneck::{
    BottleneckAnalysis,
    BottleneckClassification,
    BottleneckConfidence,
    BottleneckFinding,
};

// =============================================================================
// Public constants
// =============================================================================

/// Stable component identifier.
pub const DIAGNOSIS_ANALYSIS_COMPONENT_ID: &str =
    "zamani.quantum.benchmark.analysis.diagnosis";

/// Stable diagnosis API/schema version.
pub const DIAGNOSIS_ANALYSIS_VERSION: &str = "1.0.0";

/// Maximum number of bottleneck findings accepted.
pub const MAX_DIAGNOSIS_INPUT_FINDINGS: usize = 16_384;

/// Maximum number of diagnostic findings returned.
pub const MAX_DIAGNOSTIC_FINDINGS: usize = 16_384;

/// Maximum number of evidence items attached to one diagnostic finding.
pub const MAX_EVIDENCE_ITEMS: usize = 64;

/// Maximum number of investigation recommendations.
pub const MAX_RECOMMENDATIONS: usize = 64;

/// Maximum diagnostic identifier length in bytes.
pub const MAX_DIAGNOSTIC_ID_LENGTH: usize = 256;

/// Maximum diagnostic code length in bytes.
pub const MAX_DIAGNOSTIC_CODE_LENGTH: usize = 256;

/// Maximum diagnostic title length in bytes.
pub const MAX_DIAGNOSTIC_TITLE_LENGTH: usize = 512;

/// Maximum diagnostic explanation length in bytes.
pub const MAX_DIAGNOSTIC_EXPLANATION_LENGTH: usize = 8_192;

/// Maximum recommendation length in bytes.
pub const MAX_RECOMMENDATION_LENGTH: usize = 1_024;

/// Maximum evidence text length in bytes.
pub const MAX_EVIDENCE_LENGTH: usize = 1_024;

/// Numeric tolerance used only for stable boundary comparisons.
const NUMERIC_EPSILON: f64 = 1.0e-12;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by diagnostic analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosisError {
    /// No bottleneck analysis was supplied.
    EmptyAnalysis,

    /// Too many findings were supplied.
    TooManyFindings {
        /// Number supplied.
        count: usize,

        /// Maximum allowed.
        maximum: usize,
    },

    /// A diagnostic identifier is invalid.
    InvalidIdentifier {
        /// Invalid identifier.
        identifier: String,
    },

    /// A diagnostic title is invalid.
    InvalidTitle,

    /// A diagnostic explanation is invalid.
    InvalidExplanation,

    /// A recommendation is invalid.
    InvalidRecommendation,

    /// Evidence is invalid.
    InvalidEvidence,

    /// A score became non-finite.
    NonFiniteScore,

    /// A priority value is invalid.
    InvalidPriority,

    /// Too many evidence entries were requested.
    TooManyEvidenceItems {
        /// Number requested.
        count: usize,

        /// Maximum allowed.
        maximum: usize,
    },

    /// Too many recommendations were requested.
    TooManyRecommendations {
        /// Number requested.
        count: usize,

        /// Maximum allowed.
        maximum: usize,
    },
}

impl fmt::Display for DiagnosisError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAnalysis => {
                formatter.write_str(
                    "diagnosis requires a non-empty bottleneck analysis",
                )
            }

            Self::TooManyFindings { count, maximum } => {
                write!(
                    formatter,
                    "diagnosis received {} findings; maximum is {}",
                    count,
                    maximum
                )
            }

            Self::InvalidIdentifier { identifier } => {
                write!(
                    formatter,
                    "invalid diagnostic identifier `{}`",
                    identifier
                )
            }

            Self::InvalidTitle => {
                formatter.write_str("diagnostic title is empty or too long")
            }

            Self::InvalidExplanation => {
                formatter.write_str(
                    "diagnostic explanation is empty or too long",
                )
            }

            Self::InvalidRecommendation => {
                formatter.write_str(
                    "diagnostic recommendation is empty or too long",
                )
            }

            Self::InvalidEvidence => {
                formatter.write_str(
                    "diagnostic evidence is empty or too long",
                )
            }

            Self::NonFiniteScore => {
                formatter.write_str(
                    "diagnostic score must be finite",
                )
            }

            Self::InvalidPriority => {
                formatter.write_str(
                    "diagnostic priority must be finite and in [0, 1]",
                )
            }

            Self::TooManyEvidenceItems { count, maximum } => {
                write!(
                    formatter,
                    "diagnostic finding contains {} evidence items; maximum is {}",
                    count,
                    maximum
                )
            }

            Self::TooManyRecommendations { count, maximum } => {
                write!(
                    formatter,
                    "diagnostic report contains {} recommendations; maximum is {}",
                    count,
                    maximum
                )
            }
        }
    }
}

impl std::error::Error for DiagnosisError {}

// =============================================================================
// Diagnostic domain
// =============================================================================

/// High-level domain to which a diagnosis belongs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticDomain {
    /// Fidelity/error-quality concern.
    Quality,

    /// Runtime/latency concern.
    Performance,

    /// Throughput concern.
    Throughput,

    /// Quantum-resource concern.
    Resources,

    /// Hardware/device-characterization concern.
    Device,

    /// Error-correction/logical-computation concern.
    FaultTolerance,

    /// Scaling/width/depth concern.
    Scaling,

    /// Application-level solution-quality concern.
    Application,

    /// Measurement/statistical confidence concern.
    Measurement,

    /// Benchmark configuration concern.
    Configuration,

    /// General benchmark concern.
    General,
}

impl DiagnosticDomain {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Quality => "quality",
            Self::Performance => "performance",
            Self::Throughput => "throughput",
            Self::Resources => "resources",
            Self::Device => "device",
            Self::FaultTolerance => "fault_tolerance",
            Self::Scaling => "scaling",
            Self::Application => "application",
            Self::Measurement => "measurement",
            Self::Configuration => "configuration",
            Self::General => "general",
        }
    }
}

impl fmt::Display for DiagnosticDomain {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Diagnostic severity
// =============================================================================

/// Severity of a diagnostic conclusion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticSeverity {
    /// Informational observation.
    Info,

    /// Something should be monitored.
    Notice,

    /// Engineering investigation is recommended.
    Warning,

    /// Significant performance concern.
    High,

    /// Immediate engineering attention is warranted.
    Critical,

    /// Evidence is insufficient for a reliable conclusion.
    Uncertain,
}

impl DiagnosticSeverity {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Notice => "notice",
            Self::Warning => "warning",
            Self::High => "high",
            Self::Critical => "critical",
            Self::Uncertain => "uncertain",
        }
    }

    /// Numeric ranking used only for deterministic ordering.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Info => 1,
            Self::Notice => 2,
            Self::Warning => 3,
            Self::High => 4,
            Self::Critical => 5,
            Self::Uncertain => 0,
        }
    }

    /// Returns whether this severity requires investigation.
    pub const fn requires_investigation(self) -> bool {
        matches!(
            self,
            Self::Warning | Self::High | Self::Critical
        )
    }
}

impl fmt::Display for DiagnosticSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Diagnostic confidence
// =============================================================================

/// Confidence in the diagnostic interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticConfidence {
    /// No meaningful confidence conclusion is available.
    Unspecified,

    /// Evidence is strong and classification is stable.
    High,

    /// Evidence supports the conclusion but has limitations.
    Moderate,

    /// Evidence is weak or approximate.
    Low,

    /// Evidence crosses an important boundary.
    BoundaryCrossing,
}

impl DiagnosticConfidence {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unspecified => "unspecified",
            Self::High => "high",
            Self::Moderate => "moderate",
            Self::Low => "low",
            Self::BoundaryCrossing => "boundary_crossing",
        }
    }
}

impl fmt::Display for DiagnosticConfidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Investigation priority
// =============================================================================

/// Priority of an engineering investigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvestigationPriority {
    /// No action required.
    None,

    /// Observe on subsequent runs.
    Low,

    /// Investigate during normal engineering work.
    Medium,

    /// Investigate before relying on the benchmark result operationally.
    High,

    /// Treat as an immediate engineering blocker.
    Critical,
}

impl InvestigationPriority {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }

    /// Numeric ranking used for deterministic ordering.
    pub const fn rank(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Low => 1,
            Self::Medium => 2,
            Self::High => 3,
            Self::Critical => 4,
        }
    }
}

impl fmt::Display for InvestigationPriority {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Evidence
// =============================================================================

/// A single piece of evidence supporting a diagnosis.
#[derive(Debug, Clone, PartialEq)]
pub struct DiagnosticEvidence {
    /// Metric identifier from the underlying bottleneck analysis.
    pub metric_id: String,

    /// Human-readable evidence statement.
    pub statement: String,

    /// Point severity supplied by the bottleneck analyzer.
    pub severity: f64,

    /// Worst-case severity.
    pub worst_case_severity: f64,

    /// Original bottleneck classification.
    pub classification: BottleneckClassification,

    /// Original bottleneck confidence state.
    pub confidence: BottleneckConfidence,
}

impl DiagnosticEvidence {
    /// Creates validated evidence.
    pub fn new(
        metric_id: impl Into<String>,
        statement: impl Into<String>,
        severity: f64,
        worst_case_severity: f64,
        classification: BottleneckClassification,
        confidence: BottleneckConfidence,
    ) -> Result<Self, DiagnosisError> {
        let metric_id = metric_id.into();
        let statement = statement.into();

        validate_identifier(
            &metric_id,
            MAX_DIAGNOSTIC_ID_LENGTH,
        )?;

        validate_text(
            &statement,
            MAX_EVIDENCE_LENGTH,
            DiagnosisError::InvalidEvidence,
        )?;

        validate_unit_score(severity)?;
        validate_unit_score(worst_case_severity)?;

        Ok(Self {
            metric_id,
            statement,
            severity,
            worst_case_severity,
            classification,
            confidence,
        })
    }
}

// =============================================================================
// Recommendation
// =============================================================================

/// Structured engineering investigation recommendation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticRecommendation {
    /// Stable recommendation identifier.
    pub id: String,

    /// Human-readable recommendation.
    pub action: String,

    /// Domain to investigate.
    pub domain: DiagnosticDomain,
}

impl DiagnosticRecommendation {
    /// Creates a recommendation.
    pub fn new(
        id: impl Into<String>,
        action: impl Into<String>,
        domain: DiagnosticDomain,
    ) -> Result<Self, DiagnosisError> {
        let id = id.into();
        let action = action.into();

        validate_identifier(
            &id,
            MAX_DIAGNOSTIC_ID_LENGTH,
        )?;

        validate_text(
            &action,
            MAX_RECOMMENDATION_LENGTH,
            DiagnosisError::InvalidRecommendation,
        )?;

        Ok(Self {
            id,
            action,
            domain,
        })
    }
}

// =============================================================================
// Diagnostic finding
// =============================================================================

/// Complete diagnostic conclusion.
#[derive(Debug, Clone, PartialEq)]
pub struct DiagnosticFinding {
    /// Stable diagnostic identifier.
    pub id: String,

    /// Diagnostic domain.
    pub domain: DiagnosticDomain,

    /// Diagnostic severity.
    pub severity: DiagnosticSeverity,

    /// Confidence in the interpretation.
    pub confidence: DiagnosticConfidence,

    /// Deterministic priority for engineering investigation.
    pub priority: InvestigationPriority,

    /// Normalized diagnostic score in `[0, 1]`.
    ///
    /// This is a prioritization signal, not a benchmark score.
    pub score: f64,

    /// Short human-readable title.
    pub title: String,

    /// Detailed explanation.
    pub explanation: String,

    /// Evidence supporting this diagnosis.
    pub evidence: Vec<DiagnosticEvidence>,

    /// Recommended investigation areas.
    pub recommendations: Vec<DiagnosticRecommendation>,

    /// Whether the diagnosis may be interpreted as an actionable issue.
    pub actionable: bool,
}

impl DiagnosticFinding {
    /// Returns whether the finding requires engineering investigation.
    #[must_use]
    pub const fn requires_investigation(&self) -> bool {
        self.actionable
    }

    /// Returns the highest evidence severity.
    #[must_use]
    pub fn maximum_evidence_severity(&self) -> f64 {
        self.evidence
            .iter()
            .map(|evidence| evidence.worst_case_severity)
            .fold(0.0, f64::max)
    }
}

// =============================================================================
// Diagnosis summary
// =============================================================================

/// Summary of a diagnosis invocation.
#[derive(Debug, Clone, PartialEq)]
pub struct DiagnosisSummary {
    /// Number of input bottleneck findings.
    pub input_finding_count: usize,

    /// Number of diagnostic findings.
    pub diagnostic_count: usize,

    /// Number of critical findings.
    pub critical_count: usize,

    /// Number of high findings.
    pub high_count: usize,

    /// Number of warning findings.
    pub warning_count: usize,

    /// Number of uncertain findings.
    pub uncertain_count: usize,

    /// Number of actionable findings.
    pub actionable_count: usize,

    /// Number of measurement-quality/configuration findings.
    pub meta_diagnostic_count: usize,

    /// Highest diagnostic score.
    pub maximum_score: f64,

    /// Index of the highest-priority diagnostic finding.
    pub primary_diagnostic_index: Option<usize>,
}

impl DiagnosisSummary {
    /// Returns whether the report contains an actionable diagnosis.
    #[must_use]
    pub const fn has_actionable_diagnosis(&self) -> bool {
        self.actionable_count > 0
    }
}

// =============================================================================
// Complete report
// =============================================================================

/// Complete diagnostic analysis report.
#[derive(Debug, Clone, PartialEq)]
pub struct DiagnosticReport {
    /// Analyzer version.
    pub analyzer_version: &'static str,

    /// Diagnostic findings ordered by deterministic priority.
    pub findings: Vec<DiagnosticFinding>,

    /// Summary.
    pub summary: DiagnosisSummary,
}

impl DiagnosticReport {
    /// Returns the primary diagnostic finding.
    pub fn primary(&self) -> Option<&DiagnosticFinding> {
        self.summary
            .primary_diagnostic_index
            .and_then(|index| self.findings.get(index))
    }

    /// Returns actionable findings.
    pub fn actionable(
        &self,
    ) -> impl Iterator<Item = &DiagnosticFinding> {
        self.findings
            .iter()
            .filter(|finding| finding.actionable)
    }

    /// Returns critical findings.
    pub fn critical(
        &self,
    ) -> impl Iterator<Item = &DiagnosticFinding> {
        self.findings
            .iter()
            .filter(|finding| {
                finding.severity == DiagnosticSeverity::Critical
            })
    }

    /// Returns whether at least one actionable finding exists.
    #[must_use]
    pub const fn has_actionable_diagnosis(&self) -> bool {
        self.summary.has_actionable_diagnosis()
    }
}

// =============================================================================
// Analyzer
// =============================================================================

/// Stateless diagnostic analyzer.
///
/// The analyzer contains no global state and can be reused safely across
/// benchmark runs.
#[derive(Debug, Clone, Copy, Default)]
pub struct DiagnosisAnalyzer;

impl DiagnosisAnalyzer {
    /// Creates a diagnosis analyzer.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Converts a bottleneck analysis into a diagnostic report.
    ///
    /// This is the primary integration point.
    pub fn analyze(
        &self,
        analysis: &BottleneckAnalysis,
    ) -> Result<DiagnosticReport, DiagnosisError> {
        if analysis.findings.is_empty() {
            return Err(DiagnosisError::EmptyAnalysis);
        }

        if analysis.findings.len()
            > MAX_DIAGNOSIS_INPUT_FINDINGS
        {
            return Err(DiagnosisError::TooManyFindings {
                count: analysis.findings.len(),
                maximum: MAX_DIAGNOSIS_INPUT_FINDINGS,
            });
        }

        let mut findings = Vec::new();

        for finding in &analysis.findings {
            let diagnostic = self.diagnose_finding(finding)?;

            if findings.len() < MAX_DIAGNOSTIC_FINDINGS {
                findings.push(diagnostic);
            }
        }

        findings.sort_by(compare_diagnostics);

        let summary = build_summary(
            analysis.findings.len(),
            &findings,
        )?;

        Ok(DiagnosticReport {
            analyzer_version: DIAGNOSIS_ANALYSIS_VERSION,
            findings,
            summary,
        })
    }

    /// Diagnoses one bottleneck finding.
    ///
    /// This API is useful for streaming/incremental analysis.
    pub fn analyze_one(
        &self,
        finding: &BottleneckFinding,
    ) -> Result<DiagnosticFinding, DiagnosisError> {
        self.diagnose_finding(finding)
    }

    fn diagnose_finding(
        &self,
        finding: &BottleneckFinding,
    ) -> Result<DiagnosticFinding, DiagnosisError> {
        let metric_id = finding.metric_id.as_str();

        let domain = domain_for_metric(metric_id);

        let confidence =
            diagnostic_confidence(finding.confidence);

        let severity =
            diagnostic_severity(finding);

        let score =
            diagnostic_score(finding)?;

        let priority =
            investigation_priority(
                severity,
                finding,
            );

        let actionable =
            is_actionable(finding, severity);

        let title =
            diagnostic_title(
                finding,
                domain,
                severity,
            );

        let explanation =
            diagnostic_explanation(
                finding,
                domain,
                severity,
                confidence,
            );

        let evidence =
            vec![build_evidence(finding)?];

        let recommendations =
            build_recommendations(
                finding,
                domain,
            )?;

        Ok(DiagnosticFinding {
            id: diagnostic_id(finding),
            domain,
            severity,
            confidence,
            priority,
            score,
            title,
            explanation,
            evidence,
            recommendations,
            actionable,
        })
    }
}

// =============================================================================
// Diagnostic domain mapping
// =============================================================================

fn domain_for_metric(metric_id: &str) -> DiagnosticDomain {
    match metric_id {
        "fidelity"
        | "state_fidelity"
        | "process_fidelity"
        | "average_gate_fidelity"
        | "entanglement_fidelity"
        | "hellinger_fidelity"
        | "classical_fidelity"
        | "success_probability"
        | "solution_quality"
        | "approximation_ratio"
        | "objective_value"
        | "energy_error"
        | "observable_error"
        | "estimation_error" => DiagnosticDomain::Quality,

        "error_rate"
        | "error_per_gate"
        | "error_per_clifford"
        | "cycle_error"
        | "gate_infidelity"
        | "process_infidelity"
        | "readout_error"
        | "state_preparation_error"
        | "spam_error"
        | "leakage_rate" => DiagnosticDomain::Quality,

        "runtime"
        | "compilation_time"
        | "queue_time"
        | "submission_time"
        | "execution_time"
        | "readout_time"
        | "analysis_time"
        | "total_wall_time"
        | "latency"
        | "time_to_solution" => DiagnosticDomain::Performance,

        "throughput"
        | "shots_per_second"
        | "circuits_per_second"
        | "gates_per_second"
        | "two_qubit_gates_per_second"
        | "layers_per_second" => DiagnosticDomain::Throughput,

        "qubit_count"
        | "logical_qubit_count"
        | "physical_qubit_count"
        | "gate_count"
        | "two_qubit_gate_count"
        | "measurement_count"
        | "t_gate_count"
        | "classical_operation_count"
        | "memory"
        | "energy"
        | "resource_overhead"
        | "space_time_volume" => DiagnosticDomain::Resources,

        "t1"
        | "t2"
        | "t2_star"
        | "t_phi"
        | "readout_fidelity"
        | "physical_error_rate"
        | "crosstalk"
        | "drift"
        | "stability" => DiagnosticDomain::Device,

        "logical_error_rate"
        | "logical_fidelity"
        | "decoder_failure_probability"
        | "threshold" => DiagnosticDomain::FaultTolerance,

        "quantum_volume"
        | "circuit_depth"
        | "two_qubit_depth" => DiagnosticDomain::Scaling,

        _ => DiagnosticDomain::General,
    }
}

// =============================================================================
// Severity
// =============================================================================

fn diagnostic_severity(
    finding: &BottleneckFinding,
) -> DiagnosticSeverity {
    match finding.classification {
        BottleneckClassification::Critical => {
            DiagnosticSeverity::Critical
        }

        BottleneckClassification::Bottleneck => {
            DiagnosticSeverity::High
        }

        BottleneckClassification::Watch => {
            DiagnosticSeverity::Warning
        }

        BottleneckClassification::Healthy => {
            DiagnosticSeverity::Info
        }

        BottleneckClassification::Uncertain => {
            DiagnosticSeverity::Uncertain
        }

        BottleneckClassification::Unconfigured => {
            DiagnosticSeverity::Notice
        }
    }
}

// =============================================================================
// Confidence
// =============================================================================

fn diagnostic_confidence(
    confidence: BottleneckConfidence,
) -> DiagnosticConfidence {
    match confidence {
        BottleneckConfidence::Unspecified => {
            DiagnosticConfidence::Unspecified
        }

        BottleneckConfidence::Robust => {
            DiagnosticConfidence::High
        }

        BottleneckConfidence::BoundaryCrossing => {
            DiagnosticConfidence::BoundaryCrossing
        }

        BottleneckConfidence::LowQuality => {
            DiagnosticConfidence::Low
        }
    }
}

// =============================================================================
// Score
// =============================================================================

fn diagnostic_score(
    finding: &BottleneckFinding,
) -> Result<f64, DiagnosisError> {
    let point =
        finding.severity.get();

    let worst =
        finding.worst_case_severity.get();

    if !point.is_finite()
        || !worst.is_finite()
    {
        return Err(DiagnosisError::NonFiniteScore);
    }

    let base =
        point.max(worst);

    let confidence_multiplier =
        match finding.confidence {
            BottleneckConfidence::Robust => 1.0,

            BottleneckConfidence::Unspecified => 0.9,

            BottleneckConfidence::BoundaryCrossing => 0.75,

            BottleneckConfidence::LowQuality => 0.5,
        };

    let score =
        (base * confidence_multiplier)
            .clamp(0.0, 1.0);

    if !score.is_finite() {
        return Err(DiagnosisError::NonFiniteScore);
    }

    Ok(score)
}

// =============================================================================
// Priority
// =============================================================================

fn investigation_priority(
    severity: DiagnosticSeverity,
    finding: &BottleneckFinding,
) -> InvestigationPriority {
    match severity {
        DiagnosticSeverity::Critical => {
            InvestigationPriority::Critical
        }

        DiagnosticSeverity::High => {
            InvestigationPriority::High
        }

        DiagnosticSeverity::Warning => {
            InvestigationPriority::Medium
        }

        DiagnosticSeverity::Notice => {
            if finding.classification
                == BottleneckClassification::Unconfigured
            {
                InvestigationPriority::Low
            } else {
                InvestigationPriority::None
            }
        }

        DiagnosticSeverity::Info => {
            InvestigationPriority::None
        }

        DiagnosticSeverity::Uncertain => {
            InvestigationPriority::Medium
        }
    }
}

// =============================================================================
// Actionability
// =============================================================================

fn is_actionable(
    finding: &BottleneckFinding,
    severity: DiagnosticSeverity,
) -> bool {
    match finding.classification {
        BottleneckClassification::Critical
        | BottleneckClassification::Bottleneck
        | BottleneckClassification::Watch => {
            severity.requires_investigation()
        }

        BottleneckClassification::Uncertain => true,

        BottleneckClassification::Unconfigured => false,

        BottleneckClassification::Healthy => false,
    }
}

// =============================================================================
// Diagnostic IDs
// =============================================================================

fn diagnostic_id(
    finding: &BottleneckFinding,
) -> String {
    let sanitized =
        sanitize_component(
            &finding.metric_id,
        );

    format!(
        "diagnosis.{}",
        sanitized
    )
}

// =============================================================================
// Titles
// =============================================================================

fn diagnostic_title(
    finding: &BottleneckFinding,
    domain: DiagnosticDomain,
    severity: DiagnosticSeverity,
) -> String {
    match finding.classification {
        BottleneckClassification::Critical => {
            format!(
                "Critical {} pressure in `{}`",
                domain,
                finding.metric_id
            )
        }

        BottleneckClassification::Bottleneck => {
            format!(
                "Significant {} bottleneck in `{}`",
                domain,
                finding.metric_id
            )
        }

        BottleneckClassification::Watch => {
            format!(
                "Monitor {} pressure in `{}`",
                domain,
                finding.metric_id
            )
        }

        BottleneckClassification::Uncertain => {
            format!(
                "Uncertain {} condition in `{}`",
                domain,
                finding.metric_id
            )
        }

        BottleneckClassification::Unconfigured => {
            format!(
                "No diagnostic policy configured for `{}`",
                finding.metric_id
            )
        }

        BottleneckClassification::Healthy => {
            format!(
                "No significant {} pressure in `{}`",
                domain,
                finding.metric_id
            )
        }
    }
    .chars()
    .take(MAX_DIAGNOSTIC_TITLE_LENGTH)
    .collect::<String>()
    .pipe(|title| {
        let _ = severity;
        title
    })
}

// =============================================================================
// Explanation
// =============================================================================

fn diagnostic_explanation(
    finding: &BottleneckFinding,
    domain: DiagnosticDomain,
    severity: DiagnosticSeverity,
    confidence: DiagnosticConfidence,
) -> String {
    let classification =
        finding.classification.as_str();

    let quality =
        match finding.quality {
            super::super::core::metric::MetricQuality::Observed => {
                "observed"
            }

            super::super::core::metric::MetricQuality::Derived => {
                "derived"
            }

            super::super::core::metric::MetricQuality::Estimated => {
                "estimated"
            }

            super::super::core::metric::MetricQuality::Fitted => {
                "fitted"
            }

            super::super::core::metric::MetricQuality::Approximate => {
                "approximate"
            }

            super::super::core::metric::MetricQuality::Uncertain => {
                "uncertain"
            }

            super::super::core::metric::MetricQuality::Invalid => {
                "invalid"
            }
        };

    let mut explanation = format!(
        "The metric `{}` is classified as {} in the {} domain. \
         The measured value is {} {}, with point severity {:.6} \
         and worst-case severity {:.6}. \
         Diagnostic severity is {} and diagnostic confidence is {}. \
         The source metric quality is {}.",
        finding.metric_id,
        classification,
        domain,
        finding.value.get(),
        finding.unit.id(),
        finding.severity.get(),
        finding.worst_case_severity.get(),
        severity,
        confidence,
        quality,
    );

    match finding.classification {
        BottleneckClassification::Critical => {
            explanation.push_str(
                " The configured critical boundary has been reached or exceeded. \
                 Engineering investigation is warranted.",
            );
        }

        BottleneckClassification::Bottleneck => {
            explanation.push_str(
                " The configured bottleneck threshold has been reached. \
                 The metric should be investigated as a significant limiting factor.",
            );
        }

        BottleneckClassification::Watch => {
            explanation.push_str(
                " The metric shows measurable pressure but has not crossed \
                 the configured bottleneck threshold.",
            );
        }

        BottleneckClassification::Uncertain => {
            explanation.push_str(
                " The available evidence does not support a sufficiently \
                 stable classification. Additional or better-quality measurements \
                 should be considered before making a strong engineering conclusion.",
            );
        }

        BottleneckClassification::Unconfigured => {
            explanation.push_str(
                " No acceptable/critical policy was supplied for this metric, \
                 so no performance-limit conclusion is made.",
            );
        }

        BottleneckClassification::Healthy => {
            explanation.push_str(
                " The metric is within its configured acceptable region.",
            );
        }
    }

    explanation.push_str(
        " This diagnosis describes measured evidence and does not by itself \
         establish causal attribution.",
    );

    explanation
        .chars()
        .take(MAX_DIAGNOSTIC_EXPLANATION_LENGTH)
        .collect()
}

// =============================================================================
// Evidence
// =============================================================================

fn build_evidence(
    finding: &BottleneckFinding,
) -> Result<DiagnosticEvidence, DiagnosisError> {
    let statement = format!(
        "`{}` measured at {} {} with classification `{}`; \
         point severity {:.6}, worst-case severity {:.6}.",
        finding.metric_id,
        finding.value.get(),
        finding.unit.id(),
        finding.classification.as_str(),
        finding.severity.get(),
        finding.worst_case_severity.get(),
    );

    DiagnosticEvidence::new(
        finding.metric_id.clone(),
        statement,
        finding.severity.get(),
        finding.worst_case_severity.get(),
        finding.classification,
        finding.confidence,
    )
}

// =============================================================================
// Recommendations
// =============================================================================

fn build_recommendations(
    finding: &BottleneckFinding,
    domain: DiagnosticDomain,
) -> Result<Vec<DiagnosticRecommendation>, DiagnosisError> {
    let mut recommendations =
        Vec::with_capacity(8);

    match domain {
        DiagnosticDomain::Quality => {
            push_recommendation(
                &mut recommendations,
                "inspect_quality_characterization",
                "Inspect the relevant fidelity, error, success-probability, or application-quality measurements and verify that the acceptance criterion and statistical method are appropriate.",
                domain,
            )?;

            push_recommendation(
                &mut recommendations,
                "repeat_quality_measurement",
                "Repeat the measurement with sufficient samples and preserve the confidence interval so that statistical uncertainty is not mistaken for deterministic degradation.",
                domain,
            )?;
        }

        DiagnosticDomain::Performance => {
            push_recommendation(
                &mut recommendations,
                "decompose_latency",
                "Decompose end-to-end latency into compilation, queue, submission, execution, readout, analysis, and other available timing components before attributing the delay.",
                domain,
            )?;

            push_recommendation(
                &mut recommendations,
                "compare_execution_conditions",
                "Compare the same workload under controlled execution conditions to distinguish persistent performance pressure from transient timing variation.",
                domain,
            )?;
        }

        DiagnosticDomain::Throughput => {
            push_recommendation(
                &mut recommendations,
                "inspect_throughput_components",
                "Inspect shots-per-second, circuits-per-second, gate/layer throughput, batching, and execution timing separately where those measurements are available.",
                domain,
            )?;

            push_recommendation(
                &mut recommendations,
                "check_quality_throughput_tradeoff",
                "Check whether increased throughput is being achieved at the expense of benchmark quality; throughput should not be interpreted independently of correctness or fidelity.",
                domain,
            )?;
        }

        DiagnosticDomain::Resources => {
            push_recommendation(
                &mut recommendations,
                "inspect_resource_growth",
                "Inspect qubit, gate, depth, memory, energy, and space-time resource growth against workload size rather than treating one resource metric as universally limiting.",
                domain,
            )?;

            push_recommendation(
                &mut recommendations,
                "inspect_compilation_overhead",
                "Compare logical workload resources with compiled/routed resources to identify avoidable resource expansion.",
                domain,
            )?;
        }

        DiagnosticDomain::Device => {
            push_recommendation(
                &mut recommendations,
                "inspect_device_characterization",
                "Inspect device characterization, calibration snapshots, coherence, readout, crosstalk, and stability measurements associated with the benchmark.",
                domain,
            )?;

            push_recommendation(
                &mut recommendations,
                "compare_calibration_windows",
                "Compare measurements across calibration windows before concluding that the observed condition is persistent.",
                domain,
            )?;
        }

        DiagnosticDomain::FaultTolerance => {
            push_recommendation(
                &mut recommendations,
                "inspect_logical_scaling",
                "Inspect logical error rates, code distance, syndrome quality, decoder behavior, and physical-to-logical resource overhead together.",
                domain,
            )?;

            push_recommendation(
                &mut recommendations,
                "verify_threshold_experiment",
                "Verify that threshold conclusions use comparable code distances, workloads, physical error models, and statistical procedures.",
                domain,
            )?;
        }

        DiagnosticDomain::Scaling => {
            push_recommendation(
                &mut recommendations,
                "inspect_width_depth_surface",
                "Inspect performance across both circuit width and depth instead of relying on a single scalar benchmark value.",
                domain,
            )?;

            push_recommendation(
                &mut recommendations,
                "inspect_scaling_frontier",
                "Determine the workload-size frontier at which the required quality criterion ceases to be satisfied.",
                domain,
            )?;
        }

        DiagnosticDomain::Application => {
            push_recommendation(
                &mut recommendations,
                "inspect_application_quality",
                "Inspect solution quality, approximation quality, estimation error, convergence, and time-to-solution together for the application workload.",
                domain,
            )?;

            push_recommendation(
                &mut recommendations,
                "separate_quantum_classical_cost",
                "Separate quantum execution cost from classical optimization and post-processing cost before making an end-to-end performance conclusion.",
                domain,
            )?;
        }

        DiagnosticDomain::Measurement => {
            push_recommendation(
                &mut recommendations,
                "improve_measurement_confidence",
                "Repeat the benchmark or increase the appropriate sample count so that the diagnostic classification is statistically defensible.",
                domain,
            )?;
        }

        DiagnosticDomain::Configuration => {
            push_recommendation(
                &mut recommendations,
                "configure_metric_policy",
                "Provide an explicit metric policy defining acceptable and critical boundaries before treating the metric as a bottleneck.",
                domain,
            )?;
        }

        DiagnosticDomain::General => {
            push_recommendation(
                &mut recommendations,
                "inspect_metric_context",
                "Inspect the metric together with its workload, execution conditions, provenance, confidence information, and resource context before drawing a stronger conclusion.",
                domain,
            )?;
        }
    }

    // For uncertainty, always add a measurement-oriented recommendation.
    if matches!(
        finding.classification,
        BottleneckClassification::Uncertain
    ) {
        push_recommendation(
            &mut recommendations,
            "resolve_uncertainty",
            "Resolve the uncertainty before treating the diagnosis as a confirmed engineering condition.",
            DiagnosticDomain::Measurement,
        )?;
    }

    // For every actionable finding, explicitly preserve the causal boundary.
    if finding.is_bottleneck() {
        push_recommendation(
            &mut recommendations,
            "perform_causal_attribution_separately",
            "Use controlled experiments or the attribution subsystem before claiming that a specific hardware, compiler, routing, scheduling, calibration, or runtime component caused the observed degradation.",
            DiagnosticDomain::General,
        )?;
    }

    if recommendations.len()
        > MAX_RECOMMENDATIONS
    {
        recommendations.truncate(MAX_RECOMMENDATIONS);
    }

    Ok(recommendations)
}

fn push_recommendation(
    recommendations: &mut Vec<DiagnosticRecommendation>,
    id: &str,
    action: &str,
    domain: DiagnosticDomain,
) -> Result<(), DiagnosisError> {
    if recommendations.len()
        >= MAX_RECOMMENDATIONS
    {
        return Err(
            DiagnosisError::TooManyRecommendations {
                count: recommendations.len() + 1,
                maximum: MAX_RECOMMENDATIONS,
            },
        );
    }

    recommendations.push(
        DiagnosticRecommendation::new(
            id,
            action,
            domain,
        )?,
    );

    Ok(())
}

// =============================================================================
// Summary
// =============================================================================

fn build_summary(
    input_count: usize,
    findings: &[DiagnosticFinding],
) -> Result<DiagnosisSummary, DiagnosisError> {
    let mut critical_count = 0usize;
    let mut high_count = 0usize;
    let mut warning_count = 0usize;
    let mut uncertain_count = 0usize;
    let mut actionable_count = 0usize;
    let mut meta_diagnostic_count = 0usize;

    let mut maximum_score = 0.0f64;

    for finding in findings {
        if !finding.score.is_finite() {
            return Err(DiagnosisError::NonFiniteScore);
        }

        maximum_score =
            maximum_score.max(finding.score);

        match finding.severity {
            DiagnosticSeverity::Critical => {
                critical_count += 1;
            }

            DiagnosticSeverity::High => {
                high_count += 1;
            }

            DiagnosticSeverity::Warning => {
                warning_count += 1;
            }

            DiagnosticSeverity::Uncertain => {
                uncertain_count += 1;
            }

            DiagnosticSeverity::Info
            | DiagnosticSeverity::Notice => {}
        }

        if finding.actionable {
            actionable_count += 1;
        }

        if matches!(
            finding.domain,
            DiagnosticDomain::Measurement
                | DiagnosticDomain::Configuration
        ) {
            meta_diagnostic_count += 1;
        }
    }

    let primary_diagnostic_index =
        findings
            .iter()
            .enumerate()
            .max_by(|(_, left), (_, right)| {
                compare_diagnostics(left, right)
            })
            .map(|(index, _)| index);

    Ok(DiagnosisSummary {
        input_finding_count: input_count,
        diagnostic_count: findings.len(),
        critical_count,
        high_count,
        warning_count,
        uncertain_count,
        actionable_count,
        meta_diagnostic_count,
        maximum_score,
        primary_diagnostic_index,
    })
}

// =============================================================================
// Deterministic ordering
// =============================================================================

fn compare_diagnostics(
    left: &DiagnosticFinding,
    right: &DiagnosticFinding,
) -> Ordering {
    right
        .priority
        .rank()
        .cmp(&left.priority.rank())
        .then_with(|| {
            right
                .severity
                .rank()
                .cmp(&left.severity.rank())
        })
        .then_with(|| {
            right
                .score
                .partial_cmp(&left.score)
                .unwrap_or(Ordering::Equal)
        })
        .then_with(|| {
            right
                .maximum_evidence_severity()
                .partial_cmp(
                    &left.maximum_evidence_severity(),
                )
                .unwrap_or(Ordering::Equal)
        })
        .then_with(|| {
            left.id.cmp(&right.id)
        })
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_identifier(
    identifier: &str,
    maximum: usize,
) -> Result<(), DiagnosisError> {
    if identifier.trim().is_empty()
        || identifier.len() > maximum
    {
        return Err(
            DiagnosisError::InvalidIdentifier {
                identifier: identifier.to_owned(),
            },
        );
    }

    Ok(())
}

fn validate_text<E>(
    text: &str,
    maximum: usize,
    error: E,
) -> Result<(), E> {
    if text.trim().is_empty()
        || text.len() > maximum
    {
        return Err(error);
    }

    Ok(())
}

fn validate_unit_score(
    value: f64,
) -> Result<(), DiagnosisError> {
    if !value.is_finite()
        || !(0.0..=1.0).contains(&value)
    {
        return Err(DiagnosisError::NonFiniteScore);
    }

    Ok(())
}

fn sanitize_component(
    identifier: &str,
) -> String {
    let mut output =
        String::with_capacity(
            identifier.len().min(
                MAX_DIAGNOSTIC_ID_LENGTH,
            ),
        );

    for character in identifier
        .chars()
        .take(MAX_DIAGNOSTIC_ID_LENGTH)
    {
        if character.is_ascii_alphanumeric()
            || character == '_'
            || character == '-'
            || character == '.'
        {
            output.push(character);
        } else {
            output.push('_');
        }
    }

    if output.is_empty() {
        "metric".to_owned()
    } else {
        output
    }
}

// =============================================================================
// Compatibility helper
// =============================================================================
//
// Rust does not provide a standard `.pipe()` method. The implementation above
// deliberately avoids depending on an external crate. This tiny local helper
// exists solely to keep the title-building expression readable while keeping
// the module dependency-free.

trait Pipe: Sized {
    fn pipe<F, R>(self, function: F) -> R
    where
        F: FnOnce(Self) -> R;
}

impl<T> Pipe for T {
    fn pipe<F, R>(self, function: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        function(self)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::bottleneck::{
        BottleneckClassification,
        BottleneckConfidence,
        BottleneckFinding,
    };

    use super::super::super::core::metric::{
        FiniteF64,
        MetricKind,
        MetricQuality,
        MetricUnit,
        MetricDirection,
    };

    fn finite(value: f64) -> FiniteF64 {
        FiniteF64::new(value)
            .expect("test value must be finite")
    }

    fn runtime_finding(
        classification: BottleneckClassification,
        severity: f64,
        worst_case: f64,
        confidence: BottleneckConfidence,
    ) -> BottleneckFinding {
        BottleneckFinding {
            metric_id: "runtime".to_owned(),
            kind: MetricKind::Runtime,
            unit: MetricUnit::Seconds,
            value: finite(2.0),
            severity: finite(severity),
            best_case_severity: finite(0.0),
            worst_case_severity: finite(worst_case),
            weighted_pressure: finite(severity),
            classification,
            confidence,
            quality: MetricQuality::Observed,
            direction: MetricDirection::LowerIsBetter,
            acceptable: finite(1.0),
            critical: finite(2.0),
            weight: finite(1.0),
            diagnostic_code: "bottleneck.runtime".to_owned(),
            explanation: "test".to_owned(),
            sample_count: Some(100),
            shot_count: Some(1000),
            circuit_count: Some(10),
        }
    }

    #[test]
    fn analyzer_is_constructible() {
        let analyzer = DiagnosisAnalyzer::new();

        let _ = analyzer;
    }

    #[test]
    fn critical_bottleneck_becomes_critical_diagnosis() {
        let analyzer =
            DiagnosisAnalyzer::new();

        let finding =
            runtime_finding(
                BottleneckClassification::Critical,
                1.0,
                1.0,
                BottleneckConfidence::Robust,
            );

        let diagnosis =
            analyzer
                .analyze_one(&finding)
                .expect("diagnosis should succeed");

        assert_eq!(
            diagnosis.severity,
            DiagnosticSeverity::Critical
        );

        assert_eq!(
            diagnosis.priority,
            InvestigationPriority::Critical
        );

        assert!(diagnosis.actionable);
        assert!(diagnosis.score > 0.99);
    }

    #[test]
    fn bottleneck_becomes_high_diagnosis() {
        let analyzer =
            DiagnosisAnalyzer::new();

        let finding =
            runtime_finding(
                BottleneckClassification::Bottleneck,
                0.75,
                0.80,
                BottleneckConfidence::Robust,
            );

        let diagnosis =
            analyzer
                .analyze_one(&finding)
                .expect("diagnosis should succeed");

        assert_eq!(
            diagnosis.severity,
            DiagnosticSeverity::High
        );

        assert_eq!(
            diagnosis.priority,
            InvestigationPriority::High
        );

        assert!(diagnosis.actionable);
    }

    #[test]
    fn watch_becomes_warning() {
        let analyzer =
            DiagnosisAnalyzer::new();

        let finding =
            runtime_finding(
                BottleneckClassification::Watch,
                0.30,
                0.40,
                BottleneckConfidence::Robust,
            );

        let diagnosis =
            analyzer
                .analyze_one(&finding)
                .expect("diagnosis should succeed");

        assert_eq!(
            diagnosis.severity,
            DiagnosticSeverity::Warning
        );

        assert_eq!(
            diagnosis.priority,
            InvestigationPriority::Medium
        );
    }

    #[test]
    fn uncertainty_is_preserved() {
        let analyzer =
            DiagnosisAnalyzer::new();

        let finding =
            runtime_finding(
                BottleneckClassification::Uncertain,
                0.60,
                0.90,
                BottleneckConfidence::BoundaryCrossing,
            );

        let diagnosis =
            analyzer
                .analyze_one(&finding)
                .expect("diagnosis should succeed");

        assert_eq!(
            diagnosis.severity,
            DiagnosticSeverity::Uncertain
        );

        assert_eq!(
            diagnosis.confidence,
            DiagnosticConfidence::BoundaryCrossing
        );

        assert!(diagnosis.actionable);
    }

    #[test]
    fn unconfigured_metric_is_not_claimed_as_bottleneck() {
        let analyzer =
            DiagnosisAnalyzer::new();

        let finding =
            runtime_finding(
                BottleneckClassification::Unconfigured,
                0.0,
                0.0,
                BottleneckConfidence::Unspecified,
            );

        let diagnosis =
            analyzer
                .analyze_one(&finding)
                .expect("diagnosis should succeed");

        assert_eq!(
            diagnosis.severity,
            DiagnosticSeverity::Notice
        );

        assert_eq!(
            diagnosis.priority,
            InvestigationPriority::Low
        );

        assert!(!diagnosis.actionable);
    }

    #[test]
    fn healthy_metric_is_information_only() {
        let analyzer =
            DiagnosisAnalyzer::new();

        let finding =
            runtime_finding(
                BottleneckClassification::Healthy,
                0.0,
                0.0,
                BottleneckConfidence::Robust,
            );

        let diagnosis =
            analyzer
                .analyze_one(&finding)
                .expect("diagnosis should succeed");

        assert_eq!(
            diagnosis.severity,
            DiagnosticSeverity::Info
        );

        assert_eq!(
            diagnosis.priority,
            InvestigationPriority::None
        );

        assert!(!diagnosis.actionable);
    }

    #[test]
    fn diagnostic_score_is_bounded() {
        let analyzer =
            DiagnosisAnalyzer::new();

        let finding =
            runtime_finding(
                BottleneckClassification::Critical,
                1.0,
                1.0,
                BottleneckConfidence::Robust,
            );

        let diagnosis =
            analyzer
                .analyze_one(&finding)
                .expect("diagnosis should succeed");

        assert!(
            (0.0..=1.0).contains(
                &diagnosis.score
            )
        );
    }

    #[test]
    fn report_orders_critical_before_lower_priority() {
        let analyzer =
            DiagnosisAnalyzer::new();

        let critical =
            runtime_finding(
                BottleneckClassification::Critical,
                1.0,
                1.0,
                BottleneckConfidence::Robust,
            );

        let watch =
            runtime_finding(
                BottleneckClassification::Watch,
                0.25,
                0.25,
                BottleneckConfidence::Robust,
            );

        let analysis =
            BottleneckAnalysis {
                analyzer_version:
                    "1.0.0",
                findings: vec![
                    watch,
                    critical,
                ],
                summary:
                    super::super::bottleneck::BottleneckSummary {
                        metric_count: 2,
                        configured_metric_count: 2,
                        bottleneck_count: 1,
                        critical_count: 1,
                        healthy_count: 0,
                        watch_count: 1,
                        uncertain_count: 0,
                        unconfigured_count: 0,
                        aggregate_pressure:
                            finite(0.625),
                        primary_bottleneck_index:
                            Some(1),
                    },
            };

        let report =
            analyzer
                .analyze(&analysis)
                .expect("report should succeed");

        assert_eq!(
            report.findings.len(),
            2
        );

        assert_eq!(
            report.findings[0].severity,
            DiagnosticSeverity::Critical
        );

        assert_eq!(
            report.findings[1].severity,
            DiagnosticSeverity::Warning
        );
    }

    #[test]
    fn runtime_maps_to_performance_domain() {
        assert_eq!(
            domain_for_metric("runtime"),
            DiagnosticDomain::Performance
        );

        assert_eq!(
            domain_for_metric("execution_time"),
            DiagnosticDomain::Performance
        );
    }

    #[test]
    fn error_rate_maps_to_quality_domain() {
        assert_eq!(
            domain_for_metric("error_rate"),
            DiagnosticDomain::Quality
        );

        assert_eq!(
            domain_for_metric("logical_error_rate"),
            DiagnosticDomain::FaultTolerance
        );
    }

    #[test]
    fn quantum_volume_maps_to_scaling_domain() {
        assert_eq!(
            domain_for_metric("quantum_volume"),
            DiagnosticDomain::Scaling
        );
    }

    #[test]
    fn throughput_maps_to_throughput_domain() {
        assert_eq!(
            domain_for_metric("throughput"),
            DiagnosticDomain::Throughput
        );
    }

    #[test]
    fn evidence_preserves_original_metric_identity() {
        let analyzer =
            DiagnosisAnalyzer::new();

        let finding =
            runtime_finding(
                BottleneckClassification::Bottleneck,
                0.7,
                0.8,
                BottleneckConfidence::Robust,
            );

        let diagnosis =
            analyzer
                .analyze_one(&finding)
                .expect("diagnosis should succeed");

        assert_eq!(
            diagnosis.evidence[0].metric_id,
            "runtime"
        );
    }

    #[test]
    fn actionable_diagnosis_contains_recommendations() {
        let analyzer =
            DiagnosisAnalyzer::new();

        let finding =
            runtime_finding(
                BottleneckClassification::Bottleneck,
                0.8,
                0.9,
                BottleneckConfidence::Robust,
            );

        let diagnosis =
            analyzer
                .analyze_one(&finding)
                .expect("diagnosis should succeed");

        assert!(
            !diagnosis.recommendations.is_empty()
        );
    }

    #[test]
    fn causal_claim_is_not_automatically_made() {
        let analyzer =
            DiagnosisAnalyzer::new();

        let finding =
            runtime_finding(
                BottleneckClassification::Critical,
                1.0,
                1.0,
                BottleneckConfidence::Robust,
            );

        let diagnosis =
            analyzer
                .analyze_one(&finding)
                .expect("diagnosis should succeed");

        let joined =
            diagnosis
                .recommendations
                .iter()
                .map(|recommendation| {
                    recommendation.action.as_str()
                })
                .collect::<Vec<_>>()
                .join(" ");

        assert!(
            joined.contains("causal")
        );
    }

    #[test]
    fn empty_analysis_is_rejected() {
        let analyzer =
            DiagnosisAnalyzer::new();

        let analysis =
            BottleneckAnalysis {
                analyzer_version:
                    "1.0.0",
                findings: Vec::new(),
                summary:
                    super::super::bottleneck::BottleneckSummary {
                        metric_count: 0,
                        configured_metric_count: 0,
                        bottleneck_count: 0,
                        critical_count: 0,
                        healthy_count: 0,
                        watch_count: 0,
                        uncertain_count: 0,
                        unconfigured_count: 0,
                        aggregate_pressure:
                            finite(0.0),
                        primary_bottleneck_index:
                            None,
                    },
            };

        let result =
            analyzer.analyze(&analysis);

        assert_eq!(
            result,
            Err(DiagnosisError::EmptyAnalysis)
        );
    }

    #[test]
    fn identifiers_are_sanitized_deterministically() {
        assert_eq!(
            sanitize_component(
                "two qubit/error"
            ),
            "two_qubit_error"
        );
    }
}