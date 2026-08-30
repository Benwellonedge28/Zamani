//! Zamani Quantum Optimization — Canonical Error Model
//!
//! This module defines the stable, backend-independent error vocabulary for
//! `quantum::optimization`.
//!
//! # Architectural role
//!
//! `optimization::errors` is the lowest-level error contract of the
//! optimization subsystem. It is intentionally independent from:
//!
//! - optimization configuration;
//! - optimization profiles;
//! - optimization passes;
//! - pass registries;
//! - optimization pipelines;
//! - analyses;
//! - rewrite engines;
//! - synthesis;
//! - target descriptions;
//! - verification;
//! - serialization;
//! - routing;
//! - scheduling;
//! - hardware backends.
//!
//! Those modules may depend on this module. This module must not depend on
//! them.
//!
//! The dependency direction is therefore:
//!
//! ```text
//!                    quantum::ir
//!                       │
//!                       ▼
//!             optimization::errors
//!                       ▲
//!                       │
//!       ┌───────────────┼────────────────┐
//!       │               │                │
//!       ▼               ▼                ▼
//!     passes         pipeline         synthesis
//!       │               │                │
//!       └───────────────┼────────────────┘
//!                       ▼
//!                  verification
//! ```
//!
//! The canonical semantic representation remains `quantum::ir`. This file
//! does not define another Quantum IR and deliberately does not contain
//! optimizer-specific gate representations.
//!
//! # Design goals
//!
//! The error model provides:
//!
//! - one stable top-level `OptimizationError`;
//! - a typed `OptimizationErrorKind`;
//! - stable machine-readable error codes;
//! - severity classification;
//! - optimization-stage classification;
//! - pass/rule/operation context without depending on future types;
//! - source/circuit location information;
//! - resource-limit reporting;
//! - invalid configuration reporting;
//! - invalid target reporting;
//! - invalid rewrite reporting;
//! - analysis failures;
//! - synthesis failures;
//! - equivalence/verification failures;
//! - convergence failures;
//! - timeout reporting;
//! - serialization failures;
//! - integration failures;
//! - invariant failures;
//! - deterministic `Display` output;
//! - standard-library `Error` integration;
//! - error chaining without external dependencies;
//! - Rust 1.97 / 1.97.1 compatibility;
//! - no `unsafe` code.
//!
//! # Important ownership rule
//!
//! This module reports optimization failures. It does not own the semantics
//! that caused them.
//!
//! For example:
//!
//! - a malformed Quantum IR remains an `quantum::ir` concern;
//! - a hardware topology failure remains a routing/hardware concern;
//! - a backend execution failure remains a hardware/runtime concern;
//! - a QEC-code failure remains an error-correction concern.
//!
//! Such errors may be translated into an optimization integration error at
//! subsystem boundaries without making this module depend on those modules.
//!
//! # Stability rule
//!
//! Future optimization modules should prefer the existing variants and
//! context structures in this file instead of introducing local public error
//! enums. Local private implementation errors may exist internally, but
//! public optimization APIs should converge on `OptimizationError`.
//!
//! This file is intended to be frozen before higher-level optimization files
//! are implemented.

use std::error::Error;
use std::fmt;

// =============================================================================
// Result aliases
// =============================================================================

/// Canonical result type for public optimization operations.
pub type OptimizationResult<T> = Result<T, OptimizationError>;

/// Backwards/ergonomic alias for APIs that prefer a shorter name.
pub type OptResult<T> = Result<T, OptimizationError>;

// =============================================================================
// Stable identifiers
// =============================================================================

/// Identifies an optimization pass without depending on the future `PassId`
/// type.
///
/// A string is used deliberately so this error contract can be frozen before
/// `pass.rs` and `registry.rs` exist.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PassIdentifier(String);

impl PassIdentifier {
    /// Creates a pass identifier.
    ///
    /// Empty identifiers are rejected because they make diagnostics and
    /// provenance ambiguous.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidIdentifierError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(InvalidIdentifierError::Empty {
                kind: "optimization pass",
            });
        }

        Ok(Self(value))
    }

    /// Creates a pass identifier from a compile-time constant.
    ///
    /// This constructor is useful for pass implementations whose identifier is
    /// statically known.
    pub fn from_static(value: &'static str) -> Result<Self, InvalidIdentifierError> {
        Self::new(value)
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for PassIdentifier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PassIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Identifies an optimization rewrite rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RuleIdentifier(String);

impl RuleIdentifier {
    /// Creates a rule identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidIdentifierError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(InvalidIdentifierError::Empty {
                kind: "optimization rule",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for RuleIdentifier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RuleIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Identifies an optimization analysis.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnalysisIdentifier(String);

impl AnalysisIdentifier {
    /// Creates an analysis identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidIdentifierError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(InvalidIdentifierError::Empty {
                kind: "optimization analysis",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for AnalysisIdentifier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AnalysisIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Error code
// =============================================================================

/// Stable machine-readable optimization error code.
///
/// These codes are intended for compiler diagnostics, logs, telemetry,
/// testing, IDE integrations, and serialized reports.
///
/// Do not parse `Display` strings to determine error categories. Use this type
/// or `OptimizationError::code()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptimizationErrorCode {
    /// Input/output Quantum IR is invalid for optimization.
    InvalidInput,

    /// An optimization invariant was violated.
    InvalidInvariant,

    /// Optimization configuration is invalid.
    InvalidConfiguration,

    /// An optimization target is invalid.
    InvalidTarget,

    /// A requested operation is unsupported.
    UnsupportedOperation,

    /// A gate/operation is unsupported by the selected optimizer.
    UnsupportedGate,

    /// A requested optimization pass is unavailable.
    UnknownPass,

    /// A requested optimization analysis is unavailable.
    UnknownAnalysis,

    /// A rewrite rule is invalid.
    InvalidRewrite,

    /// A rewrite precondition was not satisfied.
    RewritePreconditionFailed,

    /// A rewrite produced an invalid result.
    RewritePostconditionFailed,

    /// Pattern matching failed.
    PatternMatchFailed,

    /// A required analysis failed.
    AnalysisFailed,

    /// A synthesis operation failed.
    SynthesisFailed,

    /// Parameter processing failed.
    ParameterError,

    /// Semantic equivalence could not be established.
    EquivalenceCheckFailed,

    /// Post-optimization verification failed.
    VerificationFailed,

    /// Optimization failed to reach a fixed point within its budget.
    NonConvergent,

    /// An explicit iteration/rewrite/pass budget was exceeded.
    IterationLimitExceeded,

    /// A configured resource limit was exceeded.
    ResourceLimitExceeded,

    /// Optimization exceeded its execution deadline.
    Timeout,

    /// The requested transformation cannot preserve semantics under the
    /// selected equivalence policy.
    SemanticViolation,

    /// Serialization/deserialization failed.
    SerializationFailed,

    /// Integration with another compiler subsystem failed.
    IntegrationFailed,

    /// An internal optimizer invariant was violated.
    InternalInvariant,

    /// A requested feature is intentionally unavailable in the current
    /// implementation.
    NotImplemented,

    /// A generic internal failure that does not fit a more specific category.
    Internal,
}

impl OptimizationErrorCode {
    /// Returns the stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidInput => "QOPT-0001",
            Self::InvalidInvariant => "QOPT-0002",
            Self::InvalidConfiguration => "QOPT-0003",
            Self::InvalidTarget => "QOPT-0004",
            Self::UnsupportedOperation => "QOPT-0005",
            Self::UnsupportedGate => "QOPT-0006",
            Self::UnknownPass => "QOPT-0007",
            Self::UnknownAnalysis => "QOPT-0008",
            Self::InvalidRewrite => "QOPT-0009",
            Self::RewritePreconditionFailed => "QOPT-0010",
            Self::RewritePostconditionFailed => "QOPT-0011",
            Self::PatternMatchFailed => "QOPT-0012",
            Self::AnalysisFailed => "QOPT-0013",
            Self::SynthesisFailed => "QOPT-0014",
            Self::ParameterError => "QOPT-0015",
            Self::EquivalenceCheckFailed => "QOPT-0016",
            Self::VerificationFailed => "QOPT-0017",
            Self::NonConvergent => "QOPT-0018",
            Self::IterationLimitExceeded => "QOPT-0019",
            Self::ResourceLimitExceeded => "QOPT-0020",
            Self::Timeout => "QOPT-0021",
            Self::SemanticViolation => "QOPT-0022",
            Self::SerializationFailed => "QOPT-0023",
            Self::IntegrationFailed => "QOPT-0024",
            Self::InternalInvariant => "QOPT-0025",
            Self::NotImplemented => "QOPT-0026",
            Self::Internal => "QOPT-0099",
        }
    }

    /// Returns the broad category represented by the code.
    pub const fn kind(self) -> OptimizationErrorKind {
        match self {
            Self::InvalidInput
            | Self::InvalidInvariant
            | Self::SemanticViolation
            | Self::InternalInvariant => OptimizationErrorKind::Validation,

            Self::InvalidConfiguration => OptimizationErrorKind::Configuration,

            Self::InvalidTarget => OptimizationErrorKind::Target,

            Self::UnsupportedOperation
            | Self::UnsupportedGate
            | Self::NotImplemented => OptimizationErrorKind::Unsupported,

            Self::UnknownPass
            | Self::UnknownAnalysis
            | Self::InvalidRewrite
            | Self::RewritePreconditionFailed
            | Self::RewritePostconditionFailed
            | Self::PatternMatchFailed => OptimizationErrorKind::Rewrite,

            Self::AnalysisFailed => OptimizationErrorKind::Analysis,

            Self::SynthesisFailed => OptimizationErrorKind::Synthesis,

            Self::ParameterError => OptimizationErrorKind::Parameter,

            Self::EquivalenceCheckFailed | Self::VerificationFailed => {
                OptimizationErrorKind::Verification
            }

            Self::NonConvergent | Self::IterationLimitExceeded => {
                OptimizationErrorKind::Convergence
            }

            Self::ResourceLimitExceeded | Self::Timeout => {
                OptimizationErrorKind::Resource
            }

            Self::SerializationFailed => OptimizationErrorKind::Serialization,

            Self::IntegrationFailed => OptimizationErrorKind::Integration,

            Self::Internal => OptimizationErrorKind::Internal,
        }
    }
}

impl fmt::Display for OptimizationErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Error kind
// =============================================================================

/// Stable high-level classification of an optimization error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptimizationErrorKind {
    /// Input or output semantic/structural validation failed.
    Validation,

    /// Configuration failed validation.
    Configuration,

    /// Target/gate-set constraints are invalid.
    Target,

    /// Requested operation is unsupported.
    Unsupported,

    /// Rewrite or pattern transformation failed.
    Rewrite,

    /// An analysis required by optimization failed.
    Analysis,

    /// Circuit synthesis failed.
    Synthesis,

    /// Parameter processing failed.
    Parameter,

    /// Semantic equivalence or verification failed.
    Verification,

    /// Fixed-point/convergence requirements failed.
    Convergence,

    /// Time or resource budget was exceeded.
    Resource,

    /// Serialization failed.
    Serialization,

    /// Integration with another subsystem failed.
    Integration,

    /// Internal optimizer failure.
    Internal,
}

impl fmt::Display for OptimizationErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Validation => "validation",
            Self::Configuration => "configuration",
            Self::Target => "target",
            Self::Unsupported => "unsupported",
            Self::Rewrite => "rewrite",
            Self::Analysis => "analysis",
            Self::Synthesis => "synthesis",
            Self::Parameter => "parameter",
            Self::Verification => "verification",
            Self::Convergence => "convergence",
            Self::Resource => "resource",
            Self::Serialization => "serialization",
            Self::Integration => "integration",
            Self::Internal => "internal",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Severity
// =============================================================================

/// Severity of an optimization diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OptimizationErrorSeverity {
    /// Informational diagnostic; normally not returned as an error.
    Info,

    /// Recoverable condition that may permit partial optimization.
    Warning,

    /// Optimization could not satisfy the requested operation.
    Error,

    /// An internal invariant was violated and compilation must not continue
    /// using the affected result.
    Fatal,
}

impl fmt::Display for OptimizationErrorSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Fatal => "fatal",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Optimization stage
// =============================================================================

/// Stage in the optimization lifecycle where an error occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptimizationStage {
    /// Input validation before any transformation.
    InputValidation,

    /// Canonicalization/normalization.
    Canonicalization,

    /// Analysis construction or invalidation.
    Analysis,

    /// Local optimization.
    LocalOptimization,

    /// Algebraic optimization.
    AlgebraicOptimization,

    /// Parameter optimization.
    ParameterOptimization,

    /// Structural/control-flow optimization.
    StructuralOptimization,

    /// Rewrite/template optimization.
    Rewrite,

    /// Equality saturation/e-graph optimization.
    EqualitySaturation,

    /// Gate/unitary/isometry synthesis.
    Synthesis,

    /// Fault-tolerant optimization.
    FaultTolerantOptimization,

    /// Target-aware optimization.
    TargetOptimization,

    /// Composite optimization pipeline.
    Pipeline,

    /// Cost evaluation.
    CostEvaluation,

    /// Post-optimization semantic verification.
    Verification,

    /// Final output validation.
    OutputValidation,

    /// Serialization of optimization artifacts.
    Serialization,

    /// Integration with another subsystem.
    Integration,

    /// Unknown or not-applicable stage.
    Unknown,
}

impl fmt::Display for OptimizationStage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::InputValidation => "input-validation",
            Self::Canonicalization => "canonicalization",
            Self::Analysis => "analysis",
            Self::LocalOptimization => "local-optimization",
            Self::AlgebraicOptimization => "algebraic-optimization",
            Self::ParameterOptimization => "parameter-optimization",
            Self::StructuralOptimization => "structural-optimization",
            Self::Rewrite => "rewrite",
            Self::EqualitySaturation => "equality-saturation",
            Self::Synthesis => "synthesis",
            Self::FaultTolerantOptimization => "fault-tolerant-optimization",
            Self::TargetOptimization => "target-optimization",
            Self::Pipeline => "pipeline",
            Self::CostEvaluation => "cost-evaluation",
            Self::Verification => "verification",
            Self::OutputValidation => "output-validation",
            Self::Serialization => "serialization",
            Self::Integration => "integration",
            Self::Unknown => "unknown",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Location
// =============================================================================

/// Logical location of an optimization failure.
///
/// This is deliberately independent of `quantum::ir` operation-ID types so
/// that the error contract remains frozen before the optimization circuit
/// adapter is implemented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OptimizationLocation {
    /// Optional zero-based operation index.
    pub operation_index: Option<usize>,

    /// Optional zero-based qubit index.
    pub qubit_index: Option<usize>,

    /// Optional zero-based classical-bit index.
    pub classical_bit_index: Option<usize>,

    /// Optional zero-based region index.
    pub region_index: Option<usize>,

    /// Optional zero-based block index.
    pub block_index: Option<usize>,
}

impl OptimizationLocation {
    /// Creates an empty location.
    pub const fn new() -> Self {
        Self {
            operation_index: None,
            qubit_index: None,
            classical_bit_index: None,
            region_index: None,
            block_index: None,
        }
    }

    /// Sets the operation index.
    pub const fn operation(mut self, index: usize) -> Self {
        self.operation_index = Some(index);
        self
    }

    /// Sets the qubit index.
    pub const fn qubit(mut self, index: usize) -> Self {
        self.qubit_index = Some(index);
        self
    }

    /// Sets the classical-bit index.
    pub const fn classical_bit(mut self, index: usize) -> Self {
        self.classical_bit_index = Some(index);
        self
    }

    /// Sets the region index.
    pub const fn region(mut self, index: usize) -> Self {
        self.region_index = Some(index);
        self
    }

    /// Sets the block index.
    pub const fn block(mut self, index: usize) -> Self {
        self.block_index = Some(index);
        self
    }

    /// Returns true when no location information is present.
    pub const fn is_empty(&self) -> bool {
        self.operation_index.is_none()
            && self.qubit_index.is_none()
            && self.classical_bit_index.is_none()
            && self.region_index.is_none()
            && self.block_index.is_none()
    }
}

impl fmt::Display for OptimizationLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut wrote = false;

        if let Some(index) = self.operation_index {
            write!(formatter, "operation={index}")?;
            wrote = true;
        }

        if let Some(index) = self.qubit_index {
            if wrote {
                formatter.write_str(", ")?;
            }
            write!(formatter, "qubit=q{index}")?;
            wrote = true;
        }

        if let Some(index) = self.classical_bit_index {
            if wrote {
                formatter.write_str(", ")?;
            }
            write!(formatter, "classical-bit=c{index}")?;
            wrote = true;
        }

        if let Some(index) = self.region_index {
            if wrote {
                formatter.write_str(", ")?;
            }
            write!(formatter, "region={index}")?;
            wrote = true;
        }

        if let Some(index) = self.block_index {
            if wrote {
                formatter.write_str(", ")?;
            }
            write!(formatter, "block={index}")?;
            wrote = true;
        }

        if !wrote {
            formatter.write_str("unknown")?;
        }

        Ok(())
    }
}

// =============================================================================
// Resource limits
// =============================================================================

/// Resource dimension associated with an optimization limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptimizationResource {
    /// Maximum number of pipeline passes.
    Passes,

    /// Maximum fixed-point iterations.
    Iterations,

    /// Maximum rewrite applications.
    Rewrites,

    /// Maximum operations in an optimized circuit.
    Operations,

    /// Maximum logical qubits.
    Qubits,

    /// Maximum analysis nodes/entries.
    AnalysisEntries,

    /// Maximum e-graph nodes.
    EGraphNodes,

    /// Maximum e-graph equivalence classes.
    EGraphClasses,

    /// Maximum synthesis operations.
    SynthesisOperations,

    /// Maximum verification operations.
    VerificationOperations,

    /// Maximum verification qubits.
    VerificationQubits,

    /// Maximum configured execution time.
    ExecutionTime,

    /// Maximum configured memory/resource budget.
    Memory,

    /// Generic implementation-defined resource.
    Other,
}

impl fmt::Display for OptimizationResource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Passes => "passes",
            Self::Iterations => "iterations",
            Self::Rewrites => "rewrites",
            Self::Operations => "operations",
            Self::Qubits => "qubits",
            Self::AnalysisEntries => "analysis-entries",
            Self::EGraphNodes => "e-graph-nodes",
            Self::EGraphClasses => "e-graph-classes",
            Self::SynthesisOperations => "synthesis-operations",
            Self::VerificationOperations => "verification-operations",
            Self::VerificationQubits => "verification-qubits",
            Self::ExecutionTime => "execution-time",
            Self::Memory => "memory",
            Self::Other => "other",
        };

        formatter.write_str(value)
    }
}

/// Detailed resource-limit violation.
///
/// The optimization `limits.rs` module will consume this type rather than
/// defining a second public error vocabulary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizationLimitError {
    /// Resource dimension.
    pub resource: OptimizationResource,

    /// Stable name of the configured limit.
    pub limit_name: String,

    /// Actual/requested value when represented as an integer.
    pub actual: Option<u64>,

    /// Maximum permitted value when represented as an integer.
    pub maximum: Option<u64>,

    /// Optional human-readable measured quantity.
    pub actual_display: Option<String>,

    /// Optional human-readable configured quantity.
    pub maximum_display: Option<String>,
}

impl OptimizationLimitError {
    /// Creates an integer-based limit violation.
    pub fn integer(
        resource: OptimizationResource,
        limit_name: impl Into<String>,
        actual: u64,
        maximum: u64,
    ) -> Self {
        Self {
            resource,
            limit_name: limit_name.into(),
            actual: Some(actual),
            maximum: Some(maximum),
            actual_display: None,
            maximum_display: None,
        }
    }

    /// Creates a display-based limit violation.
    pub fn displayed(
        resource: OptimizationResource,
        limit_name: impl Into<String>,
        actual: impl Into<String>,
        maximum: impl Into<String>,
    ) -> Self {
        Self {
            resource,
            limit_name: limit_name.into(),
            actual: None,
            maximum: None,
            actual_display: Some(actual.into()),
            maximum_display: Some(maximum.into()),
        }
    }
}

impl fmt::Display for OptimizationLimitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let (Some(actual), Some(maximum)) = (self.actual, self.maximum) {
            write!(
                formatter,
                "optimization limit `{}` for {} exceeded: actual={}, maximum={}",
                self.limit_name, self.resource, actual, maximum
            )
        } else {
            write!(
                formatter,
                "optimization limit `{}` for {} exceeded: actual={}, maximum={}",
                self.limit_name,
                self.resource,
                self.actual_display.as_deref().unwrap_or("unknown"),
                self.maximum_display.as_deref().unwrap_or("unknown"),
            )
        }
    }
}

impl Error for OptimizationLimitError {}

// =============================================================================
// Invalid identifier errors
// =============================================================================

/// Failure while constructing a stable optimization identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidIdentifierError {
    /// Identifier is empty or whitespace-only.
    Empty {
        /// Kind of identifier.
        kind: &'static str,
    },
}

impl fmt::Display for InvalidIdentifierError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { kind } => {
                write!(formatter, "{kind} identifier cannot be empty")
            }
        }
    }
}

impl Error for InvalidIdentifierError {}

// =============================================================================
// Error context
// =============================================================================

/// Structured context attached to an `OptimizationError`.
///
/// The context is intentionally additive. Future modules can populate the
/// fields relevant to them without changing the fundamental error hierarchy.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OptimizationErrorContext {
    /// Optimization pass responsible for the failure.
    pub pass: Option<PassIdentifier>,

    /// Rewrite rule responsible for the failure.
    pub rule: Option<RuleIdentifier>,

    /// Analysis responsible for the failure.
    pub analysis: Option<AnalysisIdentifier>,

    /// Operation name, when available.
    pub operation_name: Option<String>,

    /// Gate name, when available.
    pub gate_name: Option<String>,

    /// Circuit identifier/name, when available.
    pub circuit_name: Option<String>,

    /// Target identifier/name, when available.
    pub target_name: Option<String>,

    /// Optimization profile identifier/name, when available.
    pub profile_name: Option<String>,

    /// Location within the circuit.
    pub location: Option<OptimizationLocation>,

    /// Human-readable operation details.
    pub operation_details: Option<String>,

    /// Human-readable expected condition.
    pub expected: Option<String>,

    /// Human-readable actual condition.
    pub actual: Option<String>,
}

impl OptimizationErrorContext {
    /// Creates an empty context.
    pub const fn new() -> Self {
        Self {
            pass: None,
            rule: None,
            analysis: None,
            operation_name: None,
            gate_name: None,
            circuit_name: None,
            target_name: None,
            profile_name: None,
            location: None,
            operation_details: None,
            expected: None,
            actual: None,
        }
    }

    /// Attaches a pass identifier.
    pub fn with_pass(mut self, pass: PassIdentifier) -> Self {
        self.pass = Some(pass);
        self
    }

    /// Attaches a rewrite rule identifier.
    pub fn with_rule(mut self, rule: RuleIdentifier) -> Self {
        self.rule = Some(rule);
        self
    }

    /// Attaches an analysis identifier.
    pub fn with_analysis(mut self, analysis: AnalysisIdentifier) -> Self {
        self.analysis = Some(analysis);
        self
    }

    /// Attaches an operation name.
    pub fn with_operation_name(mut self, name: impl Into<String>) -> Self {
        self.operation_name = Some(name.into());
        self
    }

    /// Attaches a gate name.
    pub fn with_gate_name(mut self, name: impl Into<String>) -> Self {
        self.gate_name = Some(name.into());
        self
    }

    /// Attaches a circuit name.
    pub fn with_circuit_name(mut self, name: impl Into<String>) -> Self {
        self.circuit_name = Some(name.into());
        self
    }

    /// Attaches a target name.
    pub fn with_target_name(mut self, name: impl Into<String>) -> Self {
        self.target_name = Some(name.into());
        self
    }

    /// Attaches an optimization profile name.
    pub fn with_profile_name(mut self, name: impl Into<String>) -> Self {
        self.profile_name = Some(name.into());
        self
    }

    /// Attaches a circuit location.
    pub fn with_location(mut self, location: OptimizationLocation) -> Self {
        self.location = Some(location);
        self
    }

    /// Attaches operation details.
    pub fn with_operation_details(mut self, details: impl Into<String>) -> Self {
        self.operation_details = Some(details.into());
        self
    }

    /// Attaches an expected condition.
    pub fn with_expected(mut self, expected: impl Into<String>) -> Self {
        self.expected = Some(expected.into());
        self
    }

    /// Attaches an actual condition.
    pub fn with_actual(mut self, actual: impl Into<String>) -> Self {
        self.actual = Some(actual.into());
        self
    }

    /// Returns true when the context contains no information.
    pub fn is_empty(&self) -> bool {
        self.pass.is_none()
            && self.rule.is_none()
            && self.analysis.is_none()
            && self.operation_name.is_none()
            && self.gate_name.is_none()
            && self.circuit_name.is_none()
            && self.target_name.is_none()
            && self.profile_name.is_none()
            && self.location.is_none()
            && self.operation_details.is_none()
            && self.expected.is_none()
            && self.actual.is_none()
    }
}

// =============================================================================
// Detailed error payload
// =============================================================================

/// Detailed payload associated with an optimization failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptimizationErrorDetail {
    /// Input/output validation failure.
    Validation {
        /// Description of the invalid condition.
        reason: String,
    },

    /// Configuration failure.
    Configuration {
        /// Configuration field.
        field: String,

        /// Description of the invalid value.
        reason: String,
    },

    /// Target failure.
    Target {
        /// Target field or capability.
        field: String,

        /// Description of the invalid target condition.
        reason: String,
    },

    /// Unsupported operation.
    Unsupported {
        /// Name of the unsupported operation/feature.
        feature: String,

        /// Optional explanation.
        reason: Option<String>,
    },

    /// Invalid rewrite rule.
    Rewrite {
        /// Rule identifier, when available.
        rule: Option<RuleIdentifier>,

        /// Description of the rewrite failure.
        reason: String,
    },

    /// Analysis failure.
    Analysis {
        /// Analysis identifier, when available.
        analysis: Option<AnalysisIdentifier>,

        /// Description of the analysis failure.
        reason: String,
    },

    /// Synthesis failure.
    Synthesis {
        /// Synthesis operation/method.
        method: String,

        /// Description of the synthesis failure.
        reason: String,
    },

    /// Parameter failure.
    Parameter {
        /// Description of the parameter failure.
        reason: String,
    },

    /// Equivalence failure.
    Equivalence {
        /// Equivalence policy/method.
        method: String,

        /// Description of why equivalence failed.
        reason: String,
    },

    /// Verification failure.
    Verification {
        /// Verification method.
        method: String,

        /// Description of the verification failure.
        reason: String,
    },

    /// Fixed-point/convergence failure.
    Convergence {
        /// Number of iterations performed.
        iterations: Option<u64>,

        /// Configured maximum iterations.
        maximum_iterations: Option<u64>,

        /// Description of the convergence condition.
        reason: String,
    },

    /// Resource limit failure.
    Limit(OptimizationLimitError),

    /// Timeout.
    Timeout {
        /// Configured timeout.
        timeout: String,

        /// Optional elapsed time.
        elapsed: Option<String>,
    },

    /// Serialization failure.
    Serialization {
        /// Serialization format.
        format: String,

        /// Description of the serialization failure.
        reason: String,
    },

    /// Integration failure.
    Integration {
        /// Subsystem being integrated with.
        subsystem: String,

        /// Description of the integration failure.
        reason: String,
    },

    /// Internal invariant failure.
    Invariant {
        /// Description of the violated invariant.
        invariant: String,

        /// Optional diagnostic details.
        details: Option<String>,
    },

    /// Generic internal failure.
    Internal {
        /// Description of the internal failure.
        reason: String,
    },
}

impl fmt::Display for OptimizationErrorDetail {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation { reason } => {
                write!(formatter, "validation failed: {reason}")
            }

            Self::Configuration { field, reason } => {
                write!(formatter, "invalid configuration `{field}`: {reason}")
            }

            Self::Target { field, reason } => {
                write!(formatter, "invalid target `{field}`: {reason}")
            }

            Self::Unsupported { feature, reason } => {
                write!(formatter, "unsupported feature `{feature}`")?;

                if let Some(reason) = reason {
                    write!(formatter, ": {reason}")?;
                }

                Ok(())
            }

            Self::Rewrite { rule, reason } => {
                if let Some(rule) = rule {
                    write!(formatter, "rewrite `{rule}` failed: {reason}")
                } else {
                    write!(formatter, "rewrite failed: {reason}")
                }
            }

            Self::Analysis { analysis, reason } => {
                if let Some(analysis) = analysis {
                    write!(formatter, "analysis `{analysis}` failed: {reason}")
                } else {
                    write!(formatter, "analysis failed: {reason}")
                }
            }

            Self::Synthesis { method, reason } => {
                write!(formatter, "synthesis `{method}` failed: {reason}")
            }

            Self::Parameter { reason } => {
                write!(formatter, "parameter processing failed: {reason}")
            }

            Self::Equivalence { method, reason } => {
                write!(
                    formatter,
                    "equivalence check `{method}` failed: {reason}"
                )
            }

            Self::Verification { method, reason } => {
                write!(
                    formatter,
                    "verification `{method}` failed: {reason}"
                )
            }

            Self::Convergence {
                iterations,
                maximum_iterations,
                reason,
            } => {
                write!(formatter, "optimization did not converge: {reason}")?;

                if let (Some(iterations), Some(maximum)) =
                    (iterations, maximum_iterations)
                {
                    write!(
                        formatter,
                        " (iterations={iterations}, maximum={maximum})"
                    )?;
                }

                Ok(())
            }

            Self::Limit(limit) => limit.fmt(formatter),

            Self::Timeout { timeout, elapsed } => {
                write!(formatter, "optimization timed out after {timeout}")?;

                if let Some(elapsed) = elapsed {
                    write!(formatter, " (elapsed={elapsed})")?;
                }

                Ok(())
            }

            Self::Serialization { format, reason } => {
                write!(
                    formatter,
                    "serialization using `{format}` failed: {reason}"
                )
            }

            Self::Integration { subsystem, reason } => {
                write!(
                    formatter,
                    "integration with `{subsystem}` failed: {reason}"
                )
            }

            Self::Invariant {
                invariant,
                details,
            } => {
                write!(formatter, "optimization invariant violated: {invariant}")?;

                if let Some(details) = details {
                    write!(formatter, ": {details}")?;
                }

                Ok(())
            }

            Self::Internal { reason } => {
                write!(formatter, "internal optimization failure: {reason}")
            }
        }
    }
}

// =============================================================================
// Canonical top-level error
// =============================================================================

/// Canonical error returned by the Zamani quantum optimization subsystem.
///
/// All public optimization APIs should converge on this type.
#[derive(Debug)]
pub struct OptimizationError {
    /// Stable machine-readable error code.
    code: OptimizationErrorCode,

    /// High-level error category.
    kind: OptimizationErrorKind,

    /// Severity.
    severity: OptimizationErrorSeverity,

    /// Lifecycle stage where the failure occurred.
    stage: OptimizationStage,

    /// Detailed typed payload.
    detail: OptimizationErrorDetail,

    /// Structured contextual information.
    context: OptimizationErrorContext,

    /// Optional source error.
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl OptimizationError {
    /// Creates an error from a stable code, stage and detail.
    ///
    /// The supplied code should correspond to the detail. Debug assertions
    /// intentionally are not used here because production callers may create
    /// errors dynamically; `code.kind()` remains authoritative for category
    /// classification.
    pub fn new(
        code: OptimizationErrorCode,
        stage: OptimizationStage,
        detail: OptimizationErrorDetail,
    ) -> Self {
        let severity = match code {
            OptimizationErrorCode::InternalInvariant
            | OptimizationErrorCode::InvalidInvariant => {
                OptimizationErrorSeverity::Fatal
            }

            OptimizationErrorCode::InvalidInput
            | OptimizationErrorCode::InvalidConfiguration
            | OptimizationErrorCode::InvalidTarget
            | OptimizationErrorCode::UnsupportedOperation
            | OptimizationErrorCode::UnsupportedGate
            | OptimizationErrorCode::UnknownPass
            | OptimizationErrorCode::UnknownAnalysis
            | OptimizationErrorCode::InvalidRewrite
            | OptimizationErrorCode::RewritePreconditionFailed
            | OptimizationErrorCode::RewritePostconditionFailed
            | OptimizationErrorCode::PatternMatchFailed
            | OptimizationErrorCode::AnalysisFailed
            | OptimizationErrorCode::SynthesisFailed
            | OptimizationErrorCode::ParameterError
            | OptimizationErrorCode::EquivalenceCheckFailed
            | OptimizationErrorCode::VerificationFailed
            | OptimizationErrorCode::NonConvergent
            | OptimizationErrorCode::IterationLimitExceeded
            | OptimizationErrorCode::ResourceLimitExceeded
            | OptimizationErrorCode::Timeout
            | OptimizationErrorCode::SemanticViolation
            | OptimizationErrorCode::SerializationFailed
            | OptimizationErrorCode::IntegrationFailed => {
                OptimizationErrorSeverity::Error
            }

            OptimizationErrorCode::NotImplemented
            | OptimizationErrorCode::Internal => OptimizationErrorSeverity::Error,
        };

        Self {
            code,
            kind: code.kind(),
            severity,
            stage,
            detail,
            context: OptimizationErrorContext::default(),
            source: None,
        }
    }

    /// Creates an input validation error.
    pub fn invalid_input(
        stage: OptimizationStage,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::InvalidInput,
            stage,
            OptimizationErrorDetail::Validation {
                reason: reason.into(),
            },
        )
    }

    /// Creates an invalid configuration error.
    pub fn invalid_configuration(
        field: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::InvalidConfiguration,
            OptimizationStage::InputValidation,
            OptimizationErrorDetail::Configuration {
                field: field.into(),
                reason: reason.into(),
            },
        )
    }

    /// Creates an invalid target error.
    pub fn invalid_target(
        field: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::InvalidTarget,
            OptimizationStage::TargetOptimization,
            OptimizationErrorDetail::Target {
                field: field.into(),
                reason: reason.into(),
            },
        )
    }

    /// Creates an unsupported-operation error.
    pub fn unsupported(
        stage: OptimizationStage,
        feature: impl Into<String>,
        reason: Option<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::UnsupportedOperation,
            stage,
            OptimizationErrorDetail::Unsupported {
                feature: feature.into(),
                reason,
            },
        )
    }

    /// Creates an unsupported-gate error.
    pub fn unsupported_gate(
        stage: OptimizationStage,
        gate: impl Into<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::UnsupportedGate,
            stage,
            OptimizationErrorDetail::Unsupported {
                feature: gate.into(),
                reason: Some("the selected optimization target does not support this gate".into()),
            },
        )
    }

    /// Creates an unknown-pass error.
    pub fn unknown_pass(pass: impl Into<String>) -> Self {
        Self::new(
            OptimizationErrorCode::UnknownPass,
            OptimizationStage::Pipeline,
            OptimizationErrorDetail::Unsupported {
                feature: pass.into(),
                reason: Some("optimization pass is not registered".into()),
            },
        )
    }

    /// Creates an unknown-analysis error.
    pub fn unknown_analysis(analysis: impl Into<String>) -> Self {
        Self::new(
            OptimizationErrorCode::UnknownAnalysis,
            OptimizationStage::Analysis,
            OptimizationErrorDetail::Analysis {
                analysis: None,
                reason: format!("analysis `{}` is not registered", analysis.into()),
            },
        )
    }

    /// Creates an invalid-rewrite error.
    pub fn invalid_rewrite(
        stage: OptimizationStage,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::InvalidRewrite,
            stage,
            OptimizationErrorDetail::Rewrite {
                rule: None,
                reason: reason.into(),
            },
        )
    }

    /// Creates a rewrite-precondition error.
    pub fn rewrite_precondition_failed(
        rule: Option<RuleIdentifier>,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::RewritePreconditionFailed,
            OptimizationStage::Rewrite,
            OptimizationErrorDetail::Rewrite {
                rule,
                reason: reason.into(),
            },
        )
    }

    /// Creates a rewrite-postcondition error.
    pub fn rewrite_postcondition_failed(
        rule: Option<RuleIdentifier>,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::RewritePostconditionFailed,
            OptimizationStage::Rewrite,
            OptimizationErrorDetail::Rewrite {
                rule,
                reason: reason.into(),
            },
        )
    }

    /// Creates an analysis failure.
    pub fn analysis_failed(
        analysis: Option<AnalysisIdentifier>,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::AnalysisFailed,
            OptimizationStage::Analysis,
            OptimizationErrorDetail::Analysis {
                analysis,
                reason: reason.into(),
            },
        )
    }

    /// Creates a synthesis failure.
    pub fn synthesis_failed(
        method: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::SynthesisFailed,
            OptimizationStage::Synthesis,
            OptimizationErrorDetail::Synthesis {
                method: method.into(),
                reason: reason.into(),
            },
        )
    }

    /// Creates a parameter-processing error.
    pub fn parameter_error(
        stage: OptimizationStage,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::ParameterError,
            stage,
            OptimizationErrorDetail::Parameter {
                reason: reason.into(),
            },
        )
    }

    /// Creates an equivalence-check failure.
    pub fn equivalence_failed(
        method: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::EquivalenceCheckFailed,
            OptimizationStage::Verification,
            OptimizationErrorDetail::Equivalence {
                method: method.into(),
                reason: reason.into(),
            },
        )
    }

    /// Creates a verification failure.
    pub fn verification_failed(
        method: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::VerificationFailed,
            OptimizationStage::Verification,
            OptimizationErrorDetail::Verification {
                method: method.into(),
                reason: reason.into(),
            },
        )
    }

    /// Creates a non-convergence error.
    pub fn non_convergent(
        iterations: Option<u64>,
        maximum_iterations: Option<u64>,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::NonConvergent,
            OptimizationStage::Pipeline,
            OptimizationErrorDetail::Convergence {
                iterations,
                maximum_iterations,
                reason: reason.into(),
            },
        )
    }

    /// Creates an iteration-limit error.
    pub fn iteration_limit(
        iterations: u64,
        maximum_iterations: u64,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::IterationLimitExceeded,
            OptimizationStage::Pipeline,
            OptimizationErrorDetail::Convergence {
                iterations: Some(iterations),
                maximum_iterations: Some(maximum_iterations),
                reason: "configured iteration limit exceeded".into(),
            },
        )
    }

    /// Creates a resource-limit error.
    pub fn resource_limit(error: OptimizationLimitError) -> Self {
        Self::new(
            OptimizationErrorCode::ResourceLimitExceeded,
            OptimizationStage::Pipeline,
            OptimizationErrorDetail::Limit(error),
        )
    }

    /// Creates a timeout error.
    pub fn timeout(
        timeout: impl Into<String>,
        elapsed: Option<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::Timeout,
            OptimizationStage::Pipeline,
            OptimizationErrorDetail::Timeout {
                timeout: timeout.into(),
                elapsed,
            },
        )
    }

    /// Creates a semantic-violation error.
    pub fn semantic_violation(
        stage: OptimizationStage,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::SemanticViolation,
            stage,
            OptimizationErrorDetail::Validation {
                reason: reason.into(),
            },
        )
    }

    /// Creates a serialization error.
    pub fn serialization_failed(
        format: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::SerializationFailed,
            OptimizationStage::Serialization,
            OptimizationErrorDetail::Serialization {
                format: format.into(),
                reason: reason.into(),
            },
        )
    }

    /// Creates an integration error.
    pub fn integration_failed(
        subsystem: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::IntegrationFailed,
            OptimizationStage::Integration,
            OptimizationErrorDetail::Integration {
                subsystem: subsystem.into(),
                reason: reason.into(),
            },
        )
    }

    /// Creates an invariant error.
    pub fn invariant(
        stage: OptimizationStage,
        invariant: impl Into<String>,
        details: Option<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::InternalInvariant,
            stage,
            OptimizationErrorDetail::Invariant {
                invariant: invariant.into(),
                details,
            },
        )
    }

    /// Creates an internal error.
    pub fn internal(
        stage: OptimizationStage,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            OptimizationErrorCode::Internal,
            stage,
            OptimizationErrorDetail::Internal {
                reason: reason.into(),
            },
        )
    }

    /// Attaches structured context.
    pub fn with_context(mut self, context: OptimizationErrorContext) -> Self {
        self.context = context;
        self
    }

    /// Attaches a pass identifier.
    pub fn with_pass(mut self, pass: PassIdentifier) -> Self {
        self.context.pass = Some(pass);
        self
    }

    /// Attaches a rewrite rule identifier.
    pub fn with_rule(mut self, rule: RuleIdentifier) -> Self {
        self.context.rule = Some(rule);
        self
    }

    /// Attaches an analysis identifier.
    pub fn with_analysis(mut self, analysis: AnalysisIdentifier) -> Self {
        self.context.analysis = Some(analysis);
        self
    }

    /// Attaches a circuit location.
    pub fn with_location(mut self, location: OptimizationLocation) -> Self {
        self.context.location = Some(location);
        self
    }

    /// Attaches an operation name.
    pub fn with_operation_name(
        mut self,
        operation_name: impl Into<String>,
    ) -> Self {
        self.context.operation_name = Some(operation_name.into());
        self
    }

    /// Attaches a gate name.
    pub fn with_gate_name(mut self, gate_name: impl Into<String>) -> Self {
        self.context.gate_name = Some(gate_name.into());
        self
    }

    /// Attaches a circuit name.
    pub fn with_circuit_name(
        mut self,
        circuit_name: impl Into<String>,
    ) -> Self {
        self.context.circuit_name = Some(circuit_name.into());
        self
    }

    /// Attaches a target name.
    pub fn with_target_name(mut self, target_name: impl Into<String>) -> Self {
        self.context.target_name = Some(target_name.into());
        self
    }

    /// Attaches an optimization profile.
    pub fn with_profile_name(
        mut self,
        profile_name: impl Into<String>,
    ) -> Self {
        self.context.profile_name = Some(profile_name.into());
        self
    }

    /// Attaches an underlying source error.
    ///
    /// The source must be thread-safe so the canonical error can safely cross
    /// compiler worker boundaries.
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    /// Returns the stable machine-readable code.
    pub const fn code(&self) -> OptimizationErrorCode {
        self.code
    }

    /// Returns the high-level category.
    pub const fn kind(&self) -> OptimizationErrorKind {
        self.kind
    }

    /// Returns the severity.
    pub const fn severity(&self) -> OptimizationErrorSeverity {
        self.severity
    }

    /// Returns the optimization stage.
    pub const fn stage(&self) -> OptimizationStage {
        self.stage
    }

    /// Returns structured context.
    pub const fn context(&self) -> &OptimizationErrorContext {
        &self.context
    }

    /// Returns the detailed typed payload.
    pub const fn detail(&self) -> &OptimizationErrorDetail {
        &self.detail
    }

    /// Returns true when the error is fatal.
    pub const fn is_fatal(&self) -> bool {
        matches!(self.severity, OptimizationErrorSeverity::Fatal)
    }

    /// Returns true when the error represents a resource exhaustion condition.
    pub const fn is_resource_exhaustion(&self) -> bool {
        matches!(
            self.code,
            OptimizationErrorCode::ResourceLimitExceeded
                | OptimizationErrorCode::IterationLimitExceeded
                | OptimizationErrorCode::Timeout
        )
    }

    /// Returns true when callers may reasonably retry with different
    /// configuration.
    pub const fn is_configuration_related(&self) -> bool {
        matches!(
            self.code,
            OptimizationErrorCode::InvalidConfiguration
                | OptimizationErrorCode::InvalidTarget
                | OptimizationErrorCode::UnsupportedOperation
                | OptimizationErrorCode::UnsupportedGate
        )
    }

    /// Returns true when semantic preservation could not be established.
    pub const fn is_verification_failure(&self) -> bool {
        matches!(
            self.code,
            OptimizationErrorCode::EquivalenceCheckFailed
                | OptimizationErrorCode::VerificationFailed
                | OptimizationErrorCode::SemanticViolation
        )
    }
}

impl fmt::Display for OptimizationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "[{}] {}: {}",
            self.code,
            self.kind,
            self.detail
        )?;

        write!(formatter, " (stage={})", self.stage)?;

        if !self.context.is_empty() {
            write!(formatter, " [")?;

            let mut wrote = false;

            if let Some(pass) = &self.context.pass {
                write!(formatter, "pass={pass}")?;
                wrote = true;
            }

            if let Some(rule) = &self.context.rule {
                if wrote {
                    write!(formatter, ", ")?;
                }
                write!(formatter, "rule={rule}")?;
                wrote = true;
            }

            if let Some(analysis) = &self.context.analysis {
                if wrote {
                    write!(formatter, ", ")?;
                }
                write!(formatter, "analysis={analysis}")?;
                wrote = true;
            }

            if let Some(circuit) = &self.context.circuit_name {
                if wrote {
                    write!(formatter, ", ")?;
                }
                write!(formatter, "circuit={circuit}")?;
                wrote = true;
            }

            if let Some(operation) = &self.context.operation_name {
                if wrote {
                    write!(formatter, ", ")?;
                }
                write!(formatter, "operation={operation}")?;
                wrote = true;
            }

            if let Some(gate) = &self.context.gate_name {
                if wrote {
                    write!(formatter, ", ")?;
                }
                write!(formatter, "gate={gate}")?;
                wrote = true;
            }

            if let Some(target) = &self.context.target_name {
                if wrote {
                    write!(formatter, ", ")?;
                }
                write!(formatter, "target={target}")?;
                wrote = true;
            }

            if let Some(profile) = &self.context.profile_name {
                if wrote {
                    write!(formatter, ", ")?;
                }
                write!(formatter, "profile={profile}")?;
                wrote = true;
            }

            if let Some(location) = &self.context.location {
                if wrote {
                    write!(formatter, ", ")?;
                }
                write!(formatter, "location={location}")?;
            }

            write!(formatter, "]")?;
        }

        Ok(())
    }
}

impl Error for OptimizationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn Error + 'static))
    }
}

// =============================================================================
// Standard conversions
// =============================================================================

impl From<OptimizationLimitError> for OptimizationError {
    fn from(error: OptimizationLimitError) -> Self {
        Self::resource_limit(error)
    }
}

impl From<std::io::Error> for OptimizationError {
    fn from(error: std::io::Error) -> Self {
        Self::new(
            OptimizationErrorCode::SerializationFailed,
            OptimizationStage::Serialization,
            OptimizationErrorDetail::Serialization {
                format: "I/O".into(),
                reason: error.to_string(),
            },
        )
        .with_source(error)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_codes_are_stable() {
        assert_eq!(
            OptimizationErrorCode::InvalidInput.as_str(),
            "QOPT-0001"
        );
        assert_eq!(
            OptimizationErrorCode::VerificationFailed.as_str(),
            "QOPT-0017"
        );
        assert_eq!(
            OptimizationErrorCode::Internal.as_str(),
            "QOPT-0099"
        );
    }

    #[test]
    fn code_category_is_consistent() {
        assert_eq!(
            OptimizationErrorCode::InvalidConfiguration.kind(),
            OptimizationErrorKind::Configuration
        );

        assert_eq!(
            OptimizationErrorCode::AnalysisFailed.kind(),
            OptimizationErrorKind::Analysis
        );

        assert_eq!(
            OptimizationErrorCode::SynthesisFailed.kind(),
            OptimizationErrorKind::Synthesis
        );

        assert_eq!(
            OptimizationErrorCode::VerificationFailed.kind(),
            OptimizationErrorKind::Verification
        );

        assert_eq!(
            OptimizationErrorCode::Timeout.kind(),
            OptimizationErrorKind::Resource
        );
    }

    #[test]
    fn pass_identifier_rejects_empty_values() {
        assert!(PassIdentifier::new("").is_err());
        assert!(PassIdentifier::new("   ").is_err());

        let pass =
            PassIdentifier::new("local.cancellation")
                .expect("valid pass identifier");

        assert_eq!(pass.as_str(), "local.cancellation");
        assert_eq!(pass.to_string(), "local.cancellation");
    }

    #[test]
    fn rule_identifier_rejects_empty_values() {
        assert!(RuleIdentifier::new("").is_err());

        let rule =
            RuleIdentifier::new("identity.self_inverse")
                .expect("valid rule identifier");

        assert_eq!(rule.as_str(), "identity.self_inverse");
    }

    #[test]
    fn analysis_identifier_rejects_empty_values() {
        assert!(AnalysisIdentifier::new("").is_err());

        let analysis =
            AnalysisIdentifier::new("dependency")
                .expect("valid analysis identifier");

        assert_eq!(analysis.as_str(), "dependency");
    }

    #[test]
    fn location_builder_is_deterministic() {
        let location = OptimizationLocation::new()
            .operation(12)
            .qubit(3)
            .classical_bit(1)
            .region(2)
            .block(4);

        assert_eq!(location.operation_index, Some(12));
        assert_eq!(location.qubit_index, Some(3));
        assert_eq!(location.classical_bit_index, Some(1));
        assert_eq!(location.region_index, Some(2));
        assert_eq!(location.block_index, Some(4));

        assert_eq!(
            location.to_string(),
            "operation=12, qubit=q3, classical-bit=c1, region=2, block=4"
        );
    }

    #[test]
    fn empty_location_is_detected() {
        assert!(OptimizationLocation::new().is_empty());
        assert!(!OptimizationLocation::new().qubit(0).is_empty());
    }

    #[test]
    fn integer_limit_error_is_structured() {
        let limit = OptimizationLimitError::integer(
            OptimizationResource::Rewrites,
            "max_rewrites",
            101,
            100,
        );

        assert_eq!(limit.actual, Some(101));
        assert_eq!(limit.maximum, Some(100));
        assert!(limit.to_string().contains("max_rewrites"));
        assert!(limit.to_string().contains("101"));
        assert!(limit.to_string().contains("100"));
    }

    #[test]
    fn displayed_limit_error_is_structured() {
        let limit = OptimizationLimitError::displayed(
            OptimizationResource::ExecutionTime,
            "max_runtime",
            "10.5s",
            "10s",
        );

        let text = limit.to_string();

        assert!(text.contains("execution-time"));
        assert!(text.contains("10.5s"));
        assert!(text.contains("10s"));
    }

    #[test]
    fn invalid_input_error_has_expected_category() {
        let error = OptimizationError::invalid_input(
            OptimizationStage::InputValidation,
            "invalid circuit",
        );

        assert_eq!(
            error.code(),
            OptimizationErrorCode::InvalidInput
        );

        assert_eq!(
            error.kind(),
            OptimizationErrorKind::Validation
        );

        assert_eq!(
            error.stage(),
            OptimizationStage::InputValidation
        );

        assert_eq!(
            error.severity(),
            OptimizationErrorSeverity::Error
        );
    }

    #[test]
    fn configuration_error_is_retryable_class() {
        let error = OptimizationError::invalid_configuration(
            "optimization_level",
            "unknown level",
        );

        assert!(error.is_configuration_related());
        assert!(!error.is_fatal());
    }

    #[test]
    fn verification_failure_is_classified_correctly() {
        let error = OptimizationError::verification_failed(
            "exact-unitary",
            "optimized circuit is not equivalent to input",
        );

        assert!(error.is_verification_failure());
        assert_eq!(
            error.stage(),
            OptimizationStage::Verification
        );
    }

    #[test]
    fn resource_errors_are_classified_correctly() {
        let error = OptimizationError::iteration_limit(101, 100);

        assert!(error.is_resource_exhaustion());
        assert_eq!(
            error.code(),
            OptimizationErrorCode::IterationLimitExceeded
        );
    }

    #[test]
    fn invariant_errors_are_fatal() {
        let error = OptimizationError::invariant(
            OptimizationStage::OutputValidation,
            "operation identifiers must remain unique",
            None,
        );

        assert!(error.is_fatal());
        assert_eq!(
            error.severity(),
            OptimizationErrorSeverity::Fatal
        );
    }

    #[test]
    fn context_is_attached_without_dependency_on_future_modules() {
        let pass =
            PassIdentifier::new("local.cancellation")
                .expect("valid pass identifier");

        let error = OptimizationError::invalid_input(
            OptimizationStage::InputValidation,
            "test failure",
        )
        .with_pass(pass)
        .with_gate_name("cx")
        .with_circuit_name("bell_state")
        .with_location(
            OptimizationLocation::new()
                .operation(3)
                .qubit(0),
        );

        let text = error.to_string();

        assert!(text.contains("pass=local.cancellation"));
        assert!(text.contains("gate=cx"));
        assert!(text.contains("circuit=bell_state"));
        assert!(text.contains("operation=3"));
        assert!(text.contains("qubit=q0"));
    }

    #[test]
    fn rewrite_error_can_carry_rule_context() {
        let rule =
            RuleIdentifier::new("h.h.identity")
                .expect("valid rule identifier");

        let error = OptimizationError::rewrite_precondition_failed(
            Some(rule),
            "pattern does not match",
        );

        assert_eq!(
            error.code(),
            OptimizationErrorCode::RewritePreconditionFailed
        );

        assert!(error.to_string().contains("h.h.identity"));
    }

    #[test]
    fn analysis_error_can_carry_analysis_context() {
        let analysis =
            AnalysisIdentifier::new("dependency")
                .expect("valid analysis identifier");

        let error = OptimizationError::analysis_failed(
            Some(analysis),
            "dependency graph is inconsistent",
        );

        assert_eq!(
            error.code(),
            OptimizationErrorCode::AnalysisFailed
        );

        assert!(error.to_string().contains("dependency"));
    }

    #[test]
    fn source_errors_are_preserved() {
        let io_error =
            std::io::Error::other("test I/O failure");

        let error = OptimizationError::from(io_error);

        assert_eq!(
            error.code(),
            OptimizationErrorCode::SerializationFailed
        );

        assert!(error.source().is_some());
    }

    #[test]
    fn display_is_machine_code_first() {
        let error = OptimizationError::invalid_target(
            "gate_set",
            "missing required entangling operation",
        );

        let text = error.to_string();

        assert!(text.starts_with("[QOPT-0004]"));
        assert!(text.contains("target"));
        assert!(text.contains("gate_set"));
    }

    #[test]
    fn error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<OptimizationError>();
        assert_send_sync::<OptimizationErrorContext>();
        assert_send_sync::<OptimizationLimitError>();
    }

    #[test]
    fn resource_limit_converts_into_canonical_error() {
        let limit = OptimizationLimitError::integer(
            OptimizationResource::EGraphNodes,
            "max_egraph_nodes",
            1_001,
            1_000,
        );

        let error: OptimizationError = limit.into();

        assert_eq!(
            error.code(),
            OptimizationErrorCode::ResourceLimitExceeded
        );

        assert!(error.is_resource_exhaustion());
    }
}