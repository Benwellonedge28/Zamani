//! # Quantum Hardware Resource Estimation
//!
//! Production-grade, provider-neutral resource estimation for Zamani Quantum.
//!
//! ## Responsibility
//!
//! This module estimates the resources required by a quantum workload before
//! submission to a hardware backend. It deliberately does **not** execute
//! workloads, submit jobs, communicate with providers, perform routing, or
//! perform final backend validation.
//!
//! The estimator can answer:
//!
//! - How many physical qubits are required?
//! - How many logical qubits are required?
//! - How many quantum operations are required?
//! - What is the circuit depth?
//! - How many measurements/resets/classical bits are required?
//! - How many shots are requested?
//! - How much execution time can be estimated?
//! - How much provider cost can be estimated?
//! - Does a workload fit within known backend limits?
//! - Which required capabilities are missing?
//! - Which estimates are exact, bounded, or unavailable?
//!
//! ## Architectural boundary
//!
//! ```text
//! Zamani Quantum IR
//!        |
//!        v
//! ResourceEstimator
//!        |
//!        +----> resource requirements
//!        |
//!        +----> duration estimate
//!        |
//!        +----> cost estimate
//!        |
//!        +----> limit preflight
//!        |
//!        +----> capability preflight
//!        |
//!        v
//! compatibility / validation / routing / scheduling / execution
//! ```
//!
//! This module intentionally does not depend on those downstream modules.
//!
//! ## Integration contract
//!
//! Later modules may consume [`ResourceEstimate`] and [`ResourceRequirement`]
//! without modifying this file.
//!
//! In particular:
//!
//! - `backend.rs` supplies backend limits and capabilities.
//! - `timing.rs` can later supply measured instruction timing.
//! - `calibration.rs` can later supply calibration-aware timing.
//! - `routing.rs` can add routing overhead before final submission.
//! - `scheduling.rs` can refine parallel execution duration.
//! - `compatibility.rs` can consume capability requirements.
//! - `execution.rs` can use the estimate for preflight.
//! - `queue.rs` can combine duration with queue information.
//! - `job.rs` can attach the estimate to a job.
//! - benchmarking can record estimates alongside actual execution.
//! - Danga can expose this functionality without implementing its own
//!   quantum resource model.
//!
//! ## Important correctness rule
//!
//! An unknown timing or pricing input is never silently converted into a
//! fabricated number. The estimator reports the estimate as unavailable and
//! records a diagnostic instead.
//!
//! ## Stability
//!
//! The public types in this module are provider-neutral. Provider-specific
//! identifiers, pricing models, API objects, and credentials must never be
//! introduced here.
//!
//! ## Rust compatibility
//!
//! This implementation intentionally uses stable Rust features compatible with
//! Rust 1.97 / 1.97.1.
//!
//! ## Security
//!
//! This module never stores credentials, tokens, secrets, private keys, or
//! provider authentication material.
//!
//! No unsafe code is permitted.
//!
//! ```text
//! Public API stability: stable provider-neutral contract
//! Dependency direction: foundational -> consumers
//! Side effects: none
//! Network access: none
//! Credential access: none
//! ```
//!
//! [`BackendLimits`] from `backend.rs` can be integrated by constructing a
//! [`BackendResourceProfile`] from the existing backend limits. This module
//! does not require `backend.rs`, allowing the estimator to remain independently
//! testable and avoiding circular dependencies.

#![forbid(unsafe_code)]

use std::cmp::Ordering;
use std::fmt;

// =============================================================================
// Constants
// =============================================================================

/// Stable module schema version.
///
/// This identifies the semantic schema of serialized resource estimates,
/// not the Zamani compiler version.
pub const RESOURCE_ESTIMATOR_SCHEMA_VERSION: u16 = 1;

/// Stable module API version.
pub const RESOURCE_ESTIMATOR_API_VERSION: &str = "1.0";

/// Number of nanoseconds in one microsecond.
const NANOS_PER_MICROSECOND: u64 = 1_000;

/// Number of nanoseconds in one millisecond.
const NANOS_PER_MILLISECOND: u64 = 1_000_000;

/// Number of nanoseconds in one second.
const NANOS_PER_SECOND: u64 = 1_000_000_000;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing or calculating resource estimates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceEstimationError {
    /// An input quantity was invalid.
    InvalidInput {
        /// Stable field identifier.
        field: &'static str,
        /// Human-readable reason.
        reason: &'static str,
    },

    /// Integer arithmetic would overflow.
    ArithmeticOverflow {
        /// Stable operation identifier.
        operation: &'static str,
    },

    /// A requested resource cannot be represented.
    Unrepresentable {
        /// Stable resource identifier.
        resource: &'static str,
    },

    /// A batch contains an invalid number of workloads.
    EmptyBatch,
}

impl fmt::Display for ResourceEstimationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput { field, reason } => {
                write!(formatter, "invalid resource-estimation input `{field}`: {reason}")
            }
            Self::ArithmeticOverflow { operation } => {
                write!(formatter, "resource-estimation arithmetic overflow: {operation}")
            }
            Self::Unrepresentable { resource } => {
                write!(formatter, "resource cannot be represented: {resource}")
            }
            Self::EmptyBatch => {
                formatter.write_str("resource-estimation batch cannot be empty")
            }
        }
    }
}

impl std::error::Error for ResourceEstimationError {}

// =============================================================================
// Diagnostic model
// =============================================================================

/// Severity of an estimation diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EstimateDiagnosticSeverity {
    /// Informational condition.
    Info,

    /// Estimate is usable but has uncertainty.
    Warning,

    /// Estimate cannot be fully calculated.
    Error,
}

impl fmt::Display for EstimateDiagnosticSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        })
    }
}

/// Stable diagnostic codes.
///
/// These codes are suitable for CLI output, telemetry, testing, and future
/// machine-readable APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EstimateDiagnosticCode {
    /// Timing data was unavailable.
    TimingUnavailable,

    /// Cost data was unavailable.
    CostUnavailable,

    /// Backend limit was unspecified.
    BackendLimitUnavailable,

    /// Backend capability was not supplied.
    CapabilityUnavailable,

    /// Estimate is based on sequential execution.
    SequentialExecutionAssumption,

    /// Parallel execution information was incomplete.
    ParallelismUnknown,

    /// Routing overhead was not supplied.
    RoutingOverheadUnknown,

    /// Scheduling overhead was not supplied.
    SchedulingOverheadUnknown,

    /// Calibration-dependent timing was not supplied.
    CalibrationTimingUnavailable,

    /// A requirement exceeds a known backend limit.
    ResourceLimitExceeded,

    /// Required capability is absent.
    CapabilityMissing,

    /// Estimate uses a user-supplied approximation.
    ApproximationUsed,

    /// Cost is a model rather than an observed provider bill.
    ModeledCost,

    /// Shot count is zero.
    ZeroShots,

    /// The workload has no operations.
    EmptyWorkload,
}

impl EstimateDiagnosticCode {
    /// Returns a stable machine-readable code.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TimingUnavailable => "timing_unavailable",
            Self::CostUnavailable => "cost_unavailable",
            Self::BackendLimitUnavailable => "backend_limit_unavailable",
            Self::CapabilityUnavailable => "capability_unavailable",
            Self::SequentialExecutionAssumption => {
                "sequential_execution_assumption"
            }
            Self::ParallelismUnknown => "parallelism_unknown",
            Self::RoutingOverheadUnknown => "routing_overhead_unknown",
            Self::SchedulingOverheadUnknown => "scheduling_overhead_unknown",
            Self::CalibrationTimingUnavailable => {
                "calibration_timing_unavailable"
            }
            Self::ResourceLimitExceeded => "resource_limit_exceeded",
            Self::CapabilityMissing => "capability_missing",
            Self::ApproximationUsed => "approximation_used",
            Self::ModeledCost => "modeled_cost",
            Self::ZeroShots => "zero_shots",
            Self::EmptyWorkload => "empty_workload",
        }
    }
}

impl fmt::Display for EstimateDiagnosticCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A structured estimation diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EstimateDiagnostic {
    /// Severity.
    pub severity: EstimateDiagnosticSeverity,

    /// Stable code.
    pub code: EstimateDiagnosticCode,

    /// Stable field/resource identifier where applicable.
    pub subject: &'static str,

    /// Human-readable explanation.
    pub message: String,
}

impl EstimateDiagnostic {
    /// Creates a diagnostic.
    pub fn new(
        severity: EstimateDiagnosticSeverity,
        code: EstimateDiagnosticCode,
        subject: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code,
            subject,
            message: message.into(),
        }
    }

    /// Creates an informational diagnostic.
    pub fn info(
        code: EstimateDiagnosticCode,
        subject: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self::new(
            EstimateDiagnosticSeverity::Info,
            code,
            subject,
            message,
        )
    }

    /// Creates a warning diagnostic.
    pub fn warning(
        code: EstimateDiagnosticCode,
        subject: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self::new(
            EstimateDiagnosticSeverity::Warning,
            code,
            subject,
            message,
        )
    }

    /// Creates an error diagnostic.
    pub fn error(
        code: EstimateDiagnosticCode,
        subject: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self::new(
            EstimateDiagnosticSeverity::Error,
            code,
            subject,
            message,
        )
    }

    /// Returns the stable machine-readable code.
    pub const fn code(&self) -> EstimateDiagnosticCode {
        self.code
    }

    /// Returns whether the diagnostic prevents a complete estimate.
    pub const fn is_error(&self) -> bool {
        matches!(self.severity, EstimateDiagnosticSeverity::Error)
    }
}

// =============================================================================
// Resource quantities
// =============================================================================

/// Physical quantum resources required by a workload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PhysicalResourceRequirement {
    /// Maximum number of simultaneously live physical qubits.
    pub qubits: u64,

    /// Number of quantum modes required by a non-qubit workload, where known.
    ///
    /// This is separate from `qubits` because photonic/bosonic/continuous
    /// variable workloads are not necessarily naturally represented as qubits.
    pub modes: u64,
}

impl PhysicalResourceRequirement {
    /// Creates a qubit resource requirement.
    pub const fn qubits(qubits: u64) -> Self {
        Self { qubits, modes: 0 }
    }

    /// Creates a mode resource requirement.
    pub const fn modes(modes: u64) -> Self {
        Self { qubits: 0, modes }
    }

    /// Returns the larger physical resource count.
    pub const fn maximum_count(self) -> u64 {
        if self.qubits > self.modes {
            self.qubits
        } else {
            self.modes
        }
    }
}

/// Logical quantum resources required by a workload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LogicalResourceRequirement {
    /// Logical qubits required.
    pub logical_qubits: u64,

    /// Optional code distance.
    pub code_distance: Option<u32>,
}

impl LogicalResourceRequirement {
    /// Creates a logical-qubit requirement.
    pub const fn new(logical_qubits: u64) -> Self {
        Self {
            logical_qubits,
            code_distance: None,
        }
    }

    /// Adds a logical error-correction code distance.
    pub const fn with_code_distance(mut self, distance: u32) -> Self {
        self.code_distance = Some(distance);
        self
    }
}

/// Classical resources required by a workload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ClassicalResourceRequirement {
    /// Classical bits/register elements required.
    pub bits: u64,

    /// Number of classical result values produced.
    pub result_values: u64,
}

/// Quantum operation/resource counters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OperationResourceRequirement {
    /// Total quantum operations.
    pub total_operations: u64,

    /// One-qubit operations.
    pub single_qubit_operations: u64,

    /// Two-qubit operations.
    pub two_qubit_operations: u64,

    /// Three-qubit operations.
    pub three_qubit_operations: u64,

    /// Operations with four or more quantum operands.
    pub multi_qubit_operations: u64,

    /// Measurements.
    pub measurements: u64,

    /// Resets.
    pub resets: u64,

    /// Barriers/synchronization operations.
    pub barriers: u64,

    /// Delays.
    pub delays: u64,

    /// Conditional/control-flow operations.
    pub control_flow_operations: u64,

    /// Pulse operations.
    pub pulse_operations: u64,

    /// Analog operations.
    pub analog_operations: u64,

    /// Annealing operations.
    pub annealing_operations: u64,
}

impl OperationResourceRequirement {
    /// Returns the number of counted quantum operations.
    pub const fn total_quantum_operations(self) -> u64 {
        self.total_operations
    }

    /// Returns the maximum operation arity observed.
    pub const fn maximum_arity(self) -> u8 {
        if self.multi_qubit_operations > 0 {
            4
        } else if self.three_qubit_operations > 0 {
            3
        } else if self.two_qubit_operations > 0 {
            2
        } else if self.single_qubit_operations > 0 {
            1
        } else {
            0
        }
    }
}

// =============================================================================
// Workload shape
// =============================================================================

/// General workload shape used by the estimator.
///
/// This is intentionally more general than a gate-model circuit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkloadKind {
    /// Ordinary gate-model circuit.
    Circuit,

    /// Circuit with measurement-dependent classical control.
    DynamicCircuit,

    /// Pulse-level workload.
    Pulse,

    /// Analog Hamiltonian/control workload.
    Analog,

    /// Quantum annealing/QUBO/Ising workload.
    Annealing,

    /// Logical/fault-tolerant workload.
    Logical,

    /// Generic sampling workload.
    Sampling,

    /// Simulator workload.
    Simulation,

    /// Hardware-emulator workload.
    Emulation,
}

impl WorkloadKind {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Circuit => "circuit",
            Self::DynamicCircuit => "dynamic_circuit",
            Self::Pulse => "pulse",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Logical => "logical",
            Self::Sampling => "sampling",
            Self::Simulation => "simulation",
            Self::Emulation => "emulation",
        }
    }
}

impl fmt::Display for WorkloadKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Shape information about a workload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkloadShape {
    /// Workload category.
    pub kind: WorkloadKind,

    /// Number of logical qubits explicitly required by the program.
    pub logical_qubits: u64,

    /// Number of physical qubits/modes explicitly required before routing.
    pub physical_qubits: u64,

    /// Circuit depth, if meaningful.
    pub depth: u64,

    /// Operation counters.
    pub operations: OperationResourceRequirement,

    /// Classical resource requirements.
    pub classical: ClassicalResourceRequirement,

    /// Number of shots.
    pub shots: u64,

    /// Number of independent program executions, excluding shots.
    pub executions: u64,

    /// Whether the workload requires dynamic control flow.
    pub requires_dynamic_control: bool,

    /// Whether the workload requires pulse-level access.
    pub requires_pulse_control: bool,

    /// Whether the workload requires analog control.
    pub requires_analog_control: bool,

    /// Whether the workload requires annealing.
    pub requires_annealing: bool,

    /// Whether the workload requires logical qubits.
    pub requires_logical_qubits: bool,

    /// Whether the workload requires deterministic seeding.
    pub requires_deterministic_seed: bool,
}

impl Default for WorkloadShape {
    fn default() -> Self {
        Self {
            kind: WorkloadKind::Circuit,
            logical_qubits: 0,
            physical_qubits: 0,
            depth: 0,
            operations: OperationResourceRequirement::default(),
            classical: ClassicalResourceRequirement::default(),
            shots: 1,
            executions: 1,
            requires_dynamic_control: false,
            requires_pulse_control: false,
            requires_analog_control: false,
            requires_annealing: false,
            requires_logical_qubits: false,
            requires_deterministic_seed: false,
        }
    }
}

impl WorkloadShape {
    /// Creates a circuit workload.
    pub fn circuit(qubits: u64, depth: u64, operations: u64, shots: u64) -> Self {
        Self {
            kind: WorkloadKind::Circuit,
            logical_qubits: qubits,
            physical_qubits: qubits,
            depth,
            operations: OperationResourceRequirement {
                total_operations: operations,
                ..OperationResourceRequirement::default()
            },
            shots,
            ..Self::default()
        }
    }

    /// Validates the workload shape.
    pub fn validate(&self) -> Result<(), ResourceEstimationError> {
        if self.logical_qubits == 0 && self.physical_qubits == 0 {
            return Err(ResourceEstimationError::InvalidInput {
                field: "qubits",
                reason: "at least one quantum resource must be specified",
            });
        }

        if self.shots == 0 {
            return Err(ResourceEstimationError::InvalidInput {
                field: "shots",
                reason: "shots must be greater than zero",
            });
        }

        if self.executions == 0 {
            return Err(ResourceEstimationError::InvalidInput {
                field: "executions",
                reason: "executions must be greater than zero",
            });
        }

        if self.operations.total_operations > 0
            && self.depth > self.operations.total_operations
        {
            return Err(ResourceEstimationError::InvalidInput {
                field: "depth",
                reason: "circuit depth cannot exceed total operation count",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Timing
// =============================================================================

/// Exact duration represented as nanoseconds.
///
/// Nanoseconds are used internally to avoid floating-point accumulation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DurationNanos(u64);

impl DurationNanos {
    /// Zero duration.
    pub const ZERO: Self = Self(0);

    /// Creates a duration from nanoseconds.
    pub const fn from_nanos(nanos: u64) -> Self {
        Self(nanos)
    }

    /// Creates a duration from microseconds.
    pub fn from_micros(micros: u64) -> Result<Self, ResourceEstimationError> {
        micros
            .checked_mul(NANOS_PER_MICROSECOND)
            .map(Self)
            .ok_or(ResourceEstimationError::ArithmeticOverflow {
                operation: "microseconds_to_nanoseconds",
            })
    }

    /// Creates a duration from milliseconds.
    pub fn from_millis(millis: u64) -> Result<Self, ResourceEstimationError> {
        millis
            .checked_mul(NANOS_PER_MILLISECOND)
            .map(Self)
            .ok_or(ResourceEstimationError::ArithmeticOverflow {
                operation: "milliseconds_to_nanoseconds",
            })
    }

    /// Creates a duration from seconds.
    pub fn from_secs(seconds: u64) -> Result<Self, ResourceEstimationError> {
        seconds
            .checked_mul(NANOS_PER_SECOND)
            .map(Self)
            .ok_or(ResourceEstimationError::ArithmeticOverflow {
                operation: "seconds_to_nanoseconds",
            })
    }

    /// Returns nanoseconds.
    pub const fn as_nanos(self) -> u64 {
        self.0
    }

    /// Returns seconds as a floating-point convenience value.
    ///
    /// This is only for presentation. Internal calculations remain integer
    /// based.
    pub fn as_secs_f64(self) -> f64 {
        self.0 as f64 / NANOS_PER_SECOND as f64
    }

    /// Checked addition.
    pub fn checked_add(self, other: Self) -> Result<Self, ResourceEstimationError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(ResourceEstimationError::ArithmeticOverflow {
                operation: "duration_addition",
            })
    }

    /// Checked multiplication.
    pub fn checked_mul(self, factor: u64) -> Result<Self, ResourceEstimationError> {
        self.0
            .checked_mul(factor)
            .map(Self)
            .ok_or(ResourceEstimationError::ArithmeticOverflow {
                operation: "duration_multiplication",
            })
    }

    /// Returns the larger duration.
    pub const fn max(self, other: Self) -> Self {
        if self.0 >= other.0 {
            self
        } else {
            other
        }
    }
}

impl fmt::Display for DurationNanos {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 >= NANOS_PER_SECOND {
            write!(formatter, "{:.9}s", self.as_secs_f64())
        } else if self.0 >= NANOS_PER_MILLISECOND {
            write!(
                formatter,
                "{:.6}ms",
                self.0 as f64 / NANOS_PER_MILLISECOND as f64
            )
        } else if self.0 >= NANOS_PER_MICROSECOND {
            write!(
                formatter,
                "{:.3}µs",
                self.0 as f64 / NANOS_PER_MICROSECOND as f64
            )
        } else {
            write!(formatter, "{}ns", self.0)
        }
    }
}

/// Timing information for one operation class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationTiming {
    /// One operation duration.
    pub duration: DurationNanos,

    /// Optional additional measurement/readout latency.
    pub latency: DurationNanos,
}

impl OperationTiming {
    /// Creates timing with no additional latency.
    pub const fn new(duration: DurationNanos) -> Self {
        Self {
            duration,
            latency: DurationNanos::ZERO,
        }
    }

    /// Creates timing with explicit latency.
    pub const fn with_latency(
        duration: DurationNanos,
        latency: DurationNanos,
    ) -> Self {
        Self { duration, latency }
    }

    /// Returns operation duration plus latency.
    pub fn total(&self) -> Result<DurationNanos, ResourceEstimationError> {
        self.duration.checked_add(self.latency)
    }
}

/// Provider-neutral timing profile.
///
/// Missing fields mean that a trustworthy timing estimate cannot be produced
/// for that operation class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TimingProfile {
    /// Generic single-qubit operation.
    pub single_qubit: Option<OperationTiming>,

    /// Generic two-qubit operation.
    pub two_qubit: Option<OperationTiming>,

    /// Generic three-qubit operation.
    pub three_qubit: Option<OperationTiming>,

    /// Generic multi-qubit operation.
    pub multi_qubit: Option<OperationTiming>,

    /// Measurement.
    pub measurement: Option<OperationTiming>,

    /// Reset.
    pub reset: Option<OperationTiming>,

    /// Barrier.
    pub barrier: Option<OperationTiming>,

    /// Delay operation.
    pub delay: Option<OperationTiming>,

    /// Classical feed-forward latency.
    pub feed_forward: Option<DurationNanos>,

    /// Routing overhead per inserted operation.
    pub routing_operation: Option<OperationTiming>,

    /// Fixed submission/execution overhead.
    pub fixed_overhead: Option<DurationNanos>,
}

impl TimingProfile {
    /// Creates an empty timing profile.
    pub const fn new() -> Self {
        Self {
            single_qubit: None,
            two_qubit: None,
            three_qubit: None,
            multi_qubit: None,
            measurement: None,
            reset: None,
            barrier: None,
            delay: None,
            feed_forward: None,
            routing_operation: None,
            fixed_overhead: None,
        }
    }

    fn timing_for_arity(&self, arity: u8) -> Option<OperationTiming> {
        match arity {
            1 => self.single_qubit,
            2 => self.two_qubit,
            3 => self.three_qubit,
            _ => self.multi_qubit,
        }
    }
}

// =============================================================================
// Cost
// =============================================================================

/// Currency-independent cost unit.
///
/// Providers may later map this to USD, EUR, ZWL, credits, etc. The estimator
/// itself does not assume a particular currency.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CostAmount {
    /// Numeric amount.
    pub amount: f64,

    /// Stable currency/unit identifier.
    pub unit: &'static str,
}

impl CostAmount {
    /// Creates a cost amount.
    pub const fn new(amount: f64, unit: &'static str) -> Self {
        Self { amount, unit }
    }
}

/// Provider-neutral pricing profile.
///
/// All rates are optional. Missing pricing information produces an explicit
/// unavailable cost estimate rather than a fabricated value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PricingProfile {
    /// Cost per shot.
    pub cost_per_shot: Option<f64>,

    /// Cost per execution.
    pub cost_per_execution: Option<f64>,

    /// Cost per quantum operation.
    pub cost_per_operation: Option<f64>,

    /// Cost per second of execution.
    pub cost_per_second: Option<f64>,

    /// Cost unit/currency.
    pub unit: &'static str,
}

impl Default for PricingProfile {
    fn default() -> Self {
        Self {
            cost_per_shot: None,
            cost_per_execution: None,
            cost_per_operation: None,
            cost_per_second: None,
            unit: "unknown",
        }
    }
}

impl PricingProfile {
    /// Creates an empty pricing profile.
    pub const fn new(unit: &'static str) -> Self {
        Self {
            cost_per_shot: None,
            cost_per_execution: None,
            cost_per_operation: None,
            cost_per_second: None,
            unit,
        }
    }

    /// Returns whether at least one pricing component exists.
    pub fn has_any_rate(&self) -> bool {
        self.cost_per_shot.is_some()
            || self.cost_per_execution.is_some()
            || self.cost_per_operation.is_some()
            || self.cost_per_second.is_some()
    }

    fn validate(&self) -> Result<(), ResourceEstimationError> {
        let rates = [
            ("cost_per_shot", self.cost_per_shot),
            ("cost_per_execution", self.cost_per_execution),
            ("cost_per_operation", self.cost_per_operation),
            ("cost_per_second", self.cost_per_second),
        ];

        for (field, value) in rates {
            if let Some(rate) = value {
                if !rate.is_finite() || rate < 0.0 {
                    return Err(ResourceEstimationError::InvalidInput {
                        field,
                        reason: "pricing rates must be finite and non-negative",
                    });
                }
            }
        }

        if self.unit.trim().is_empty() {
            return Err(ResourceEstimationError::InvalidInput {
                field: "pricing.unit",
                reason: "pricing unit must not be empty",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Backend profile
// =============================================================================

/// Provider-neutral resource limits.
///
/// This deliberately mirrors the semantic fields already established by
/// Zamani's `BackendLimits`, while using `u64` to avoid architecture-dependent
/// `usize` semantics in a public estimation result.
///
/// A value of zero means that the corresponding provider limit is unknown or
/// unspecified, not that the backend has zero capacity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendResourceProfile {
    /// Maximum physical qubits.
    pub max_qubits: u64,

    /// Maximum logical qubits.
    pub max_logical_qubits: u64,

    /// Maximum circuit depth.
    pub max_circuit_depth: u64,

    /// Maximum operation count.
    pub max_operations: u64,

    /// Maximum shots.
    pub max_shots: u64,

    /// Maximum classical bits.
    pub max_classical_bits: u64,

    /// Maximum concurrent jobs.
    pub max_concurrent_jobs: u64,

    /// Maximum batch size.
    pub max_batch_size: u64,

    /// Whether the backend can provide timing information.
    pub timing_available: bool,

    /// Whether the backend exposes pricing information.
    pub pricing_available: bool,
}

impl Default for BackendResourceProfile {
    fn default() -> Self {
        Self::unknown()
    }
}

impl BackendResourceProfile {
    /// Creates a profile in which all numeric limits are unspecified.
    pub const fn unknown() -> Self {
        Self {
            max_qubits: 0,
            max_logical_qubits: 0,
            max_circuit_depth: 0,
            max_operations: 0,
            max_shots: 0,
            max_classical_bits: 0,
            max_concurrent_jobs: 0,
            max_batch_size: 0,
            timing_available: false,
            pricing_available: false,
        }
    }

    /// Returns whether a numeric limit is known and finite.
    pub const fn has_limit(value: u64) -> bool {
        value != 0
    }
}

// =============================================================================
// Capabilities
// =============================================================================

/// Provider-neutral capabilities required by a workload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RequiredCapabilities {
    /// Measurement required.
    pub measurement: bool,

    /// Reset required.
    pub reset: bool,

    /// Mid-circuit measurement required.
    pub mid_circuit_measurement: bool,

    /// Classical control required.
    pub classical_control: bool,

    /// Dynamic circuits required.
    pub dynamic_circuits: bool,

    /// Pulse control required.
    pub pulse_control: bool,

    /// Analog control required.
    pub analog_control: bool,

    /// Annealing required.
    pub annealing: bool,

    /// Logical qubits required.
    pub logical_qubits: bool,

    /// Fault tolerance required.
    pub fault_tolerance: bool,

    /// Deterministic seed required.
    pub deterministic_seeding: bool,

    /// State-vector results required.
    pub state_vector_results: bool,

    /// Density-matrix results required.
    pub density_matrix_results: bool,

    /// Expectation-value results required.
    pub expectation_value_results: bool,

    /// Native instruction set required.
    pub native_instruction_set: bool,
}

/// Capability availability supplied to the estimator.
///
/// The field names intentionally mirror the provider-neutral backend
/// capability vocabulary already established in Zamani.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AvailableCapabilities {
    pub measurement: bool,
    pub reset: bool,
    pub mid_circuit_measurement: bool,
    pub classical_control: bool,
    pub dynamic_circuits: bool,
    pub pulse_control: bool,
    pub analog_control: bool,
    pub annealing: bool,
    pub logical_qubits: bool,
    pub fault_tolerance: bool,
    pub deterministic_seeding: bool,
    pub state_vector_results: bool,
    pub density_matrix_results: bool,
    pub expectation_value_results: bool,
    pub native_instruction_set: bool,
}

// =============================================================================
// Estimate confidence
// =============================================================================

/// Confidence classification of an estimate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EstimateConfidence {
    /// Exact under the supplied model and inputs.
    Exact,

    /// Deterministic but based on an explicit approximation/model.
    Modeled,

    /// A lower/upper bound or incomplete hardware information is involved.
    Bounded,

    /// A requested quantity cannot be estimated reliably.
    Unavailable,
}

impl fmt::Display for EstimateConfidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Exact => "exact",
            Self::Modeled => "modeled",
            Self::Bounded => "bounded",
            Self::Unavailable => "unavailable",
        })
    }
}

// =============================================================================
// Duration estimate
// =============================================================================

/// Execution-duration estimate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurationEstimate {
    /// Estimated duration when available.
    pub duration: Option<DurationNanos>,

    /// Confidence classification.
    pub confidence: EstimateConfidence,

    /// Whether the calculation assumes sequential execution.
    pub assumes_sequential_execution: bool,

    /// Whether shot multiplication was applied.
    pub includes_shots: bool,
}

impl DurationEstimate {
    /// Creates an unavailable duration estimate.
    pub const fn unavailable() -> Self {
        Self {
            duration: None,
            confidence: EstimateConfidence::Unavailable,
            assumes_sequential_execution: false,
            includes_shots: false,
        }
    }

    /// Creates an exact estimate.
    pub const fn exact(duration: DurationNanos) -> Self {
        Self {
            duration: Some(duration),
            confidence: EstimateConfidence::Exact,
            assumes_sequential_execution: false,
            includes_shots: false,
        }
    }
}

// =============================================================================
// Cost estimate
// =============================================================================

/// Execution-cost estimate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CostEstimate {
    /// Estimated cost when available.
    pub cost: Option<CostAmount>,

    /// Confidence classification.
    pub confidence: EstimateConfidence,

    /// Whether the value is based on a pricing model.
    pub modeled: bool,
}

impl CostEstimate {
    /// Creates an unavailable cost estimate.
    pub const fn unavailable() -> Self {
        Self {
            cost: None,
            confidence: EstimateConfidence::Unavailable,
            modeled: false,
        }
    }
}

// =============================================================================
// Requirement checks
// =============================================================================

/// Result of checking a workload against a backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequirementStatus {
    /// Requirement definitely fits known backend information.
    Satisfied,

    /// Requirement cannot be decided because backend information is missing.
    Unknown,

    /// Requirement definitely cannot be satisfied.
    Exceeded,
}

impl RequirementStatus {
    /// Returns true if the requirement is satisfied.
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied)
    }

    /// Returns true if the requirement definitely fails.
    pub const fn is_exceeded(self) -> bool {
        matches!(self, Self::Exceeded)
    }
}

/// One resource-limit check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceLimitCheck {
    /// Resource name.
    pub resource: &'static str,

    /// Required amount.
    pub required: u64,

    /// Backend limit; zero means unknown.
    pub available: u64,

    /// Check status.
    pub status: RequirementStatus,
}

impl ResourceLimitCheck {
    fn new(
        resource: &'static str,
        required: u64,
        available: u64,
    ) -> Self {
        let status = if available == 0 {
            RequirementStatus::Unknown
        } else if required <= available {
            RequirementStatus::Satisfied
        } else {
            RequirementStatus::Exceeded
        };

        Self {
            resource,
            required,
            available,
            status,
        }
    }
}

/// One capability check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityCheck {
    /// Stable capability name.
    pub capability: &'static str,

    /// Required by workload.
    pub required: bool,

    /// Available on backend.
    pub available: bool,

    /// Whether backend capability information was known.
    pub known: bool,
}

impl CapabilityCheck {
    fn evaluate(
        capability: &'static str,
        required: bool,
        available: bool,
        known: bool,
    ) -> Self {
        Self {
            capability,
            required,
            available,
            known,
        }
    }

    /// Returns the status.
    pub const fn status(self) -> RequirementStatus {
        if !self.required {
            RequirementStatus::Satisfied
        } else if !self.known {
            RequirementStatus::Unknown
        } else if self.available {
            RequirementStatus::Satisfied
        } else {
            RequirementStatus::Exceeded
        }
    }
}

/// Complete backend preflight result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourcePreflight {
    /// Resource checks.
    pub resource_limits: Vec<ResourceLimitCheck>,

    /// Capability checks.
    pub capabilities: Vec<CapabilityCheck>,

    /// Diagnostics.
    pub diagnostics: Vec<EstimateDiagnostic>,
}

impl ResourcePreflight {
    /// Returns true when no requirement definitely fails.
    pub fn is_not_exceeded(&self) -> bool {
        !self
            .resource_limits
            .iter()
            .any(|check| check.status.is_exceeded())
            && !self
                .capabilities
                .iter()
                .any(|check| check.status().is_exceeded())
    }

    /// Returns true only when every required item is known and satisfied.
    pub fn is_fully_satisfied(&self) -> bool {
        self.resource_limits
            .iter()
            .all(|check| check.status.is_satisfied())
            && self
                .capabilities
                .iter()
                .all(|check| check.status().is_satisfied())
    }

    /// Returns true if any error diagnostic exists.
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(EstimateDiagnostic::is_error)
    }
}

// =============================================================================
// Complete estimate
// =============================================================================

/// Complete provider-neutral resource estimate.
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceEstimate {
    /// Schema version.
    pub schema_version: u16,

    /// Workload kind.
    pub workload_kind: WorkloadKind,

    /// Physical resource requirement.
    pub physical: PhysicalResourceRequirement,

    /// Logical resource requirement.
    pub logical: LogicalResourceRequirement,

    /// Classical resource requirement.
    pub classical: ClassicalResourceRequirement,

    /// Operation requirement.
    pub operations: OperationResourceRequirement,

    /// Circuit depth.
    pub depth: u64,

    /// Shots.
    pub shots: u64,

    /// Independent executions.
    pub executions: u64,

    /// Estimated duration.
    pub duration: DurationEstimate,

    /// Estimated cost.
    pub cost: CostEstimate,

    /// Backend preflight.
    pub preflight: Option<ResourcePreflight>,

    /// Diagnostics.
    pub diagnostics: Vec<EstimateDiagnostic>,
}

impl ResourceEstimate {
    /// Returns true when the estimate contains no fatal error diagnostics.
    pub fn is_usable(&self) -> bool {
        !self.diagnostics.iter().any(EstimateDiagnostic::is_error)
    }

    /// Returns true if duration is available.
    pub const fn has_duration(&self) -> bool {
        self.duration.duration.is_some()
    }

    /// Returns true if cost is available.
    pub const fn has_cost(&self) -> bool {
        self.cost.cost.is_some()
    }

    /// Returns the highest diagnostic severity.
    pub fn highest_diagnostic_severity(&self) -> Option<EstimateDiagnosticSeverity> {
        self.diagnostics
            .iter()
            .map(|diagnostic| diagnostic.severity)
            .max()
    }
}

// =============================================================================
// Estimation options
// =============================================================================

/// Parallelism model used for duration estimation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParallelismModel {
    /// Treat every operation as sequential.
    Sequential,

    /// Use circuit depth as the number of parallel layers.
    DepthBased,

    /// Use an explicitly supplied layer duration.
    ExplicitLayerTiming,
}

impl Default for ParallelismModel {
    fn default() -> Self {
        Self::DepthBased
    }
}

/// Controls how the estimator handles incomplete information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EstimationOptions {
    /// Parallelism strategy.
    pub parallelism: ParallelismModel,

    /// Include routing overhead if routing timing is supplied.
    pub include_routing_overhead: bool,

    /// Include fixed provider overhead if supplied.
    pub include_fixed_overhead: bool,

    /// Multiply execution time by shots.
    pub multiply_by_shots: bool,

    /// Multiply execution time by independent executions.
    pub multiply_by_executions: bool,

    /// Calculate cost when any usable pricing component exists.
    pub calculate_cost: bool,
}

impl Default for EstimationOptions {
    fn default() -> Self {
        Self {
            parallelism: ParallelismModel::DepthBased,
            include_routing_overhead: true,
            include_fixed_overhead: true,
            multiply_by_shots: true,
            multiply_by_executions: true,
            calculate_cost: true,
        }
    }
}

// =============================================================================
// Estimator
// =============================================================================

/// Production resource estimator.
///
/// The estimator is immutable after construction and therefore safe to share
/// between callers when wrapped in the appropriate application-level
/// concurrency primitive.
#[derive(Debug, Clone, Copy)]
pub struct ResourceEstimator {
    /// Backend resource profile.
    backend: Option<BackendResourceProfile>,

    /// Timing profile.
    timing: TimingProfile,

    /// Pricing profile.
    pricing: PricingProfile,

    /// Available capabilities.
    capabilities: Option<AvailableCapabilities>,

    /// Estimation options.
    options: EstimationOptions,
}

impl Default for ResourceEstimator {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceEstimator {
    /// Creates an estimator without backend-specific information.
    pub const fn new() -> Self {
        Self {
            backend: None,
            timing: TimingProfile::new(),
            pricing: PricingProfile::new("unknown"),
            capabilities: None,
            options: EstimationOptions {
                parallelism: ParallelismModel::DepthBased,
                include_routing_overhead: true,
                include_fixed_overhead: true,
                multiply_by_shots: true,
                multiply_by_executions: true,
                calculate_cost: true,
            },
        }
    }

    /// Attaches a backend resource profile.
    pub const fn with_backend_profile(
        mut self,
        backend: BackendResourceProfile,
    ) -> Self {
        self.backend = Some(backend);
        self
    }

    /// Attaches timing information.
    pub const fn with_timing_profile(
        mut self,
        timing: TimingProfile,
    ) -> Self {
        self.timing = timing;
        self
    }

    /// Attaches pricing information.
    pub const fn with_pricing_profile(
        mut self,
        pricing: PricingProfile,
    ) -> Self {
        self.pricing = pricing;
        self
    }

    /// Attaches capability information.
    pub const fn with_capabilities(
        mut self,
        capabilities: AvailableCapabilities,
    ) -> Self {
        self.capabilities = Some(capabilities);
        self
    }

    /// Changes estimation options.
    pub const fn with_options(
        mut self,
        options: EstimationOptions,
    ) -> Self {
        self.options = options;
        self
    }

    /// Estimates a workload.
    pub fn estimate(
        &self,
        workload: &WorkloadShape,
    ) -> Result<ResourceEstimate, ResourceEstimationError> {
        workload.validate()?;

        let mut diagnostics = Vec::new();

        if workload.operations.total_operations == 0 {
            diagnostics.push(EstimateDiagnostic::info(
                EstimateDiagnosticCode::EmptyWorkload,
                "operations",
                "workload contains no quantum operations",
            ));
        }

        if workload.shots == 0 {
            diagnostics.push(EstimateDiagnostic::warning(
                EstimateDiagnosticCode::ZeroShots,
                "shots",
                "workload requests zero shots",
            ));
        }

        let physical = PhysicalResourceRequirement::qubits(
            workload.physical_qubits.max(workload.logical_qubits),
        );

        let logical = LogicalResourceRequirement::new(workload.logical_qubits);

        let duration = self.estimate_duration(workload, &mut diagnostics)?;

        let cost = if self.options.calculate_cost {
            self.estimate_cost(workload, duration, &mut diagnostics)?
        } else {
            CostEstimate::unavailable()
        };

        let preflight = self.preflight(workload);

        if let Some(ref result) = preflight {
            diagnostics.extend(result.diagnostics.iter().cloned());
        }

        Ok(ResourceEstimate {
            schema_version: RESOURCE_ESTIMATOR_SCHEMA_VERSION,
            workload_kind: workload.kind,
            physical,
            logical,
            classical: workload.classical,
            operations: workload.operations,
            depth: workload.depth,
            shots: workload.shots,
            executions: workload.executions,
            duration,
            cost,
            preflight,
            diagnostics,
        })
    }

    /// Estimates execution duration.
    pub fn estimate_duration(
        &self,
        workload: &WorkloadShape,
        diagnostics: &mut Vec<EstimateDiagnostic>,
    ) -> Result<DurationEstimate, ResourceEstimationError> {
        if workload.operations.total_operations == 0 {
            if let Some(fixed) = self.timing.fixed_overhead {
                return Ok(DurationEstimate {
                    duration: Some(fixed),
                    confidence: EstimateConfidence::Modeled,
                    assumes_sequential_execution: false,
                    includes_shots: false,
                });
            }

            return Ok(DurationEstimate::unavailable());
        }

        let base = match self.options.parallelism {
            ParallelismModel::Sequential => {
                diagnostics.push(EstimateDiagnostic::warning(
                    EstimateDiagnosticCode::SequentialExecutionAssumption,
                    "parallelism",
                    "duration uses a sequential-operation model",
                ));

                self.sequential_duration(workload)?
            }

            ParallelismModel::DepthBased => {
                self.depth_based_duration(workload, diagnostics)?
            }

            ParallelismModel::ExplicitLayerTiming => {
                diagnostics.push(EstimateDiagnostic::warning(
                    EstimateDiagnosticCode::ParallelismUnknown,
                    "parallelism",
                    "explicit layer timing was requested but no layer timing model exists; falling back to depth-based timing",
                ));

                self.depth_based_duration(workload, diagnostics)?
            }
        };

        let mut total = base;

        if self.options.include_fixed_overhead {
            if let Some(fixed) = self.timing.fixed_overhead {
                total = total.checked_add(fixed)?;
            }
        }

        let mut confidence = if self.has_complete_timing_for(workload) {
            EstimateConfidence::Modeled
        } else {
            EstimateConfidence::Bounded
        };

        if self.options.multiply_by_shots {
            total = total.checked_mul(workload.shots)?;
        }

        if self.options.multiply_by_executions {
            total = total.checked_mul(workload.executions)?;
        }

        if !self.has_complete_timing_for(workload) {
            diagnostics.push(EstimateDiagnostic::warning(
                EstimateDiagnosticCode::TimingUnavailable,
                "timing",
                "one or more operation classes lack trusted timing information; duration is not fully exact",
            ));

            confidence = EstimateConfidence::Bounded;
        }

        Ok(DurationEstimate {
            duration: Some(total),
            confidence,
            assumes_sequential_execution: matches!(
                self.options.parallelism,
                ParallelismModel::Sequential
            ),
            includes_shots: self.options.multiply_by_shots,
        })
    }

    fn sequential_duration(
        &self,
        workload: &WorkloadShape,
    ) -> Result<DurationNanos, ResourceEstimationError> {
        let mut total = DurationNanos::ZERO;

        total = total.checked_add(
            self.class_duration(
                workload.operations.single_qubit_operations,
                self.timing.single_qubit,
            )?,
        )?;

        total = total.checked_add(
            self.class_duration(
                workload.operations.two_qubit_operations,
                self.timing.two_qubit,
            )?,
        )?;

        total = total.checked_add(
            self.class_duration(
                workload.operations.three_qubit_operations,
                self.timing.three_qubit,
            )?,
        )?;

        total = total.checked_add(
            self.class_duration(
                workload.operations.multi_qubit_operations,
                self.timing.multi_qubit,
            )?,
        )?;

        total = total.checked_add(
            self.class_duration(
                workload.operations.measurements,
                self.timing.measurement,
            )?,
        )?;

        total = total.checked_add(
            self.class_duration(
                workload.operations.resets,
                self.timing.reset,
            )?,
        )?;

        total = total.checked_add(
            self.class_duration(
                workload.operations.barriers,
                self.timing.barrier,
            )?,
        )?;

        total = total.checked_add(
            self.class_duration(
                workload.operations.delays,
                self.timing.delay,
            )?,
        )?;

        if workload.requires_dynamic_control {
            if let Some(latency) = self.timing.feed_forward {
                total = total.checked_add(latency)?;
            }
        }

        Ok(total)
    }

    fn depth_based_duration(
        &self,
        workload: &WorkloadShape,
        diagnostics: &mut Vec<EstimateDiagnostic>,
    ) -> Result<DurationNanos, ResourceEstimationError> {
        let timing = self.depth_layer_duration(workload, diagnostics)?;

        if self.options.include_routing_overhead
            && workload.operations.two_qubit_operations > 0
        {
            if let Some(routing) = self.timing.routing_operation {
                let routing_count =
                    workload.operations.two_qubit_operations;

                let routing_time =
                    routing.total()?.checked_mul(routing_count)?;

                return timing.checked_add(routing_time);
            }
        }

        Ok(timing)
    }

    fn depth_layer_duration(
        &self,
        workload: &WorkloadShape,
        diagnostics: &mut Vec<EstimateDiagnostic>,
    ) -> Result<DurationNanos, ResourceEstimationError> {
        if workload.depth == 0 {
            return self.sequential_duration(workload);
        }

        let max_timing = [
            self.timing.single_qubit,
            self.timing.two_qubit,
            self.timing.three_qubit,
            self.timing.multi_qubit,
        ]
        .iter()
        .flatten()
        .filter_map(|timing| timing.total().ok())
        .max();

        let measurement_time = workload
            .operations
            .measurements
            .checked_add(workload.operations.resets)
            .ok_or(ResourceEstimationError::ArithmeticOverflow {
                operation: "measurement_reset_count",
            })?;

        let mut duration = match max_timing {
            Some(timing) => timing.checked_mul(workload.depth)?,
            None => {
                diagnostics.push(EstimateDiagnostic::warning(
                    EstimateDiagnosticCode::TimingUnavailable,
                    "gate_timing",
                    "no gate timing information is available",
                ));

                return Err(ResourceEstimationError::Unrepresentable {
                    resource: "circuit_duration",
                });
            }
        };

        if measurement_time > 0 {
            let terminal = self
                .timing
                .measurement
                .or(self.timing.reset)
                .map(|timing| timing.total())
                .transpose()?;

            if let Some(terminal) = terminal {
                duration = duration.checked_add(
                    terminal.checked_mul(measurement_time)?,
                )?;
            } else {
                diagnostics.push(EstimateDiagnostic::warning(
                    EstimateDiagnosticCode::TimingUnavailable,
                    "measurement_reset_timing",
                    "measurement/reset timing is unavailable",
                ));
            }
        }

        if workload.requires_dynamic_control {
            if let Some(latency) = self.timing.feed_forward {
                duration = duration.checked_add(latency)?;
            } else {
                diagnostics.push(EstimateDiagnostic::warning(
                    EstimateDiagnosticCode::TimingUnavailable,
                    "feed_forward",
                    "dynamic-control feed-forward latency is unavailable",
                ));
            }
        }

        Ok(duration)
    }

    fn class_duration(
        &self,
        count: u64,
        timing: Option<OperationTiming>,
    ) -> Result<DurationNanos, ResourceEstimationError> {
        if count == 0 {
            return Ok(DurationNanos::ZERO);
        }

        let timing = timing.ok_or(ResourceEstimationError::Unrepresentable {
            resource: "operation_timing",
        })?;

        timing.total()?.checked_mul(count)
    }

    fn has_complete_timing_for(&self, workload: &WorkloadShape) -> bool {
        let operation_timing_complete =
            (workload.operations.single_qubit_operations == 0
                || self.timing.single_qubit.is_some())
                && (workload.operations.two_qubit_operations == 0
                    || self.timing.two_qubit.is_some())
                && (workload.operations.three_qubit_operations == 0
                    || self.timing.three_qubit.is_some())
                && (workload.operations.multi_qubit_operations == 0
                    || self.timing.multi_qubit.is_some())
                && (workload.operations.measurements == 0
                    || self.timing.measurement.is_some())
                && (workload.operations.resets == 0
                    || self.timing.reset.is_some());

        operation_timing_complete
    }

    /// Estimates cost from the supplied provider-neutral pricing model.
    pub fn estimate_cost(
        &self,
        workload: &WorkloadShape,
        duration: DurationEstimate,
        diagnostics: &mut Vec<EstimateDiagnostic>,
    ) -> Result<CostEstimate, ResourceEstimationError> {
        self.pricing.validate()?;

        if !self.pricing.has_any_rate() {
            diagnostics.push(EstimateDiagnostic::warning(
                EstimateDiagnosticCode::CostUnavailable,
                "pricing",
                "no pricing information was supplied",
            ));

            return Ok(CostEstimate::unavailable());
        }

        let mut amount = 0.0;

        if let Some(rate) = self.pricing.cost_per_shot {
            amount += rate * workload.shots as f64;
        }

        if let Some(rate) = self.pricing.cost_per_execution {
            amount += rate * workload.executions as f64;
        }

        if let Some(rate) = self.pricing.cost_per_operation {
            amount +=
                rate * workload.operations.total_operations as f64;
        }

        if let Some(rate) = self.pricing.cost_per_second {
            if let Some(duration) = duration.duration {
                amount += rate * duration.as_secs_f64();
            } else {
                diagnostics.push(EstimateDiagnostic::warning(
                    EstimateDiagnosticCode::CostUnavailable,
                    "duration",
                    "time-based pricing exists but execution duration is unavailable",
                ));
            }
        }

        if !amount.is_finite() || amount < 0.0 {
            return Err(ResourceEstimationError::InvalidInput {
                field: "cost",
                reason: "calculated cost is not finite and non-negative",
            });
        }

        diagnostics.push(EstimateDiagnostic::info(
            EstimateDiagnosticCode::ModeledCost,
            "pricing",
            "cost is a provider-neutral model estimate, not an observed provider invoice",
        ));

        Ok(CostEstimate {
            cost: Some(CostAmount::new(amount, self.pricing.unit)),
            confidence: EstimateConfidence::Modeled,
            modeled: true,
        })
    }

    /// Performs backend limit and capability preflight.
    pub fn preflight(
        &self,
        workload: &WorkloadShape,
    ) -> Option<ResourcePreflight> {
        let backend = self.backend?;

        let mut resource_limits = Vec::new();
        let mut capabilities_checks = Vec::new();
        let mut diagnostics = Vec::new();

        resource_limits.push(ResourceLimitCheck::new(
            "max_qubits",
            workload.physical_qubits,
            backend.max_qubits,
        ));

        resource_limits.push(ResourceLimitCheck::new(
            "max_logical_qubits",
            workload.logical_qubits,
            backend.max_logical_qubits,
        ));

        resource_limits.push(ResourceLimitCheck::new(
            "max_circuit_depth",
            workload.depth,
            backend.max_circuit_depth,
        ));

        resource_limits.push(ResourceLimitCheck::new(
            "max_operations",
            workload.operations.total_operations,
            backend.max_operations,
        ));

        resource_limits.push(ResourceLimitCheck::new(
            "max_shots",
            workload.shots,
            backend.max_shots,
        ));

        resource_limits.push(ResourceLimitCheck::new(
            "max_classical_bits",
            workload.classical.bits,
            backend.max_classical_bits,
        ));

        for check in &resource_limits {
            match check.status {
                RequirementStatus::Exceeded => {
                    diagnostics.push(EstimateDiagnostic::error(
                        EstimateDiagnosticCode::ResourceLimitExceeded,
                        check.resource,
                        format!(
                            "workload requires {} but backend limit is {}",
                            check.required, check.available
                        ),
                    ));
                }

                RequirementStatus::Unknown => {
                    diagnostics.push(EstimateDiagnostic::warning(
                        EstimateDiagnosticCode::BackendLimitUnavailable,
                        check.resource,
                        format!(
                            "backend limit for {} is unspecified",
                            check.resource
                        ),
                    ));
                }

                RequirementStatus::Satisfied => {}
            }
        }

        if let Some(capabilities) = self.capabilities {
            capabilities_checks.push(CapabilityCheck::evaluate(
                "measurement",
                workload.operations.measurements > 0,
                capabilities.measurement,
                true,
            ));

            capabilities_checks.push(CapabilityCheck::evaluate(
                "reset",
                workload.operations.resets > 0,
                capabilities.reset,
                true,
            ));

            capabilities_checks.push(CapabilityCheck::evaluate(
                "mid_circuit_measurement",
                workload.requires_dynamic_control,
                capabilities.mid_circuit_measurement,
                true,
            ));

            capabilities_checks.push(CapabilityCheck::evaluate(
                "classical_control",
                workload.requires_dynamic_control,
                capabilities.classical_control,
                true,
            ));

            capabilities_checks.push(CapabilityCheck::evaluate(
                "dynamic_circuits",
                workload.requires_dynamic_control,
                capabilities.dynamic_circuits,
                true,
            ));

            capabilities_checks.push(CapabilityCheck::evaluate(
                "pulse_control",
                workload.requires_pulse_control,
                capabilities.pulse_control,
                true,
            ));

            capabilities_checks.push(CapabilityCheck::evaluate(
                "analog_control",
                workload.requires_analog_control,
                capabilities.analog_control,
                true,
            ));

            capabilities_checks.push(CapabilityCheck::evaluate(
                "annealing",
                workload.requires_annealing,
                capabilities.annealing,
                true,
            ));

            capabilities_checks.push(CapabilityCheck::evaluate(
                "logical_qubits",
                workload.requires_logical_qubits,
                capabilities.logical_qubits,
                true,
            ));

            capabilities_checks.push(CapabilityCheck::evaluate(
                "deterministic_seeding",
                workload.requires_deterministic_seed,
                capabilities.deterministic_seeding,
                true,
            ));

            for check in &capabilities_checks {
                if check.status().is_exceeded() {
                    diagnostics.push(EstimateDiagnostic::error(
                        EstimateDiagnosticCode::CapabilityMissing,
                        check.capability,
                        format!(
                            "workload requires capability `{}` but backend does not advertise it",
                            check.capability
                        ),
                    ));
                }
            }
        } else {
            for capability in [
                "measurement",
                "reset",
                "mid_circuit_measurement",
                "classical_control",
                "dynamic_circuits",
                "pulse_control",
                "analog_control",
                "annealing",
                "logical_qubits",
                "deterministic_seeding",
            ] {
                capabilities_checks.push(CapabilityCheck::evaluate(
                    capability,
                    false,
                    false,
                    false,
                ));
            }

            diagnostics.push(EstimateDiagnostic::warning(
                EstimateDiagnosticCode::CapabilityUnavailable,
                "capabilities",
                "backend capabilities were not supplied; capability preflight is incomplete",
            ));
        }

        Some(ResourcePreflight {
            resource_limits,
            capabilities: capabilities_checks,
            diagnostics,
        })
    }

    /// Estimates several workloads independently.
    ///
    /// No workload is allowed to influence another workload's estimate.
    pub fn estimate_batch(
        &self,
        workloads: &[WorkloadShape],
    ) -> Result<Vec<ResourceEstimate>, ResourceEstimationError> {
        if workloads.is_empty() {
            return Err(ResourceEstimationError::EmptyBatch);
        }

        workloads.iter().map(|workload| self.estimate(workload)).collect()
    }

    /// Estimates the aggregate resource requirements of a batch.
    ///
    /// Physical and logical qubit counts use the maximum concurrent requirement
    /// rather than summing resources, because batch jobs may execute
    /// sequentially. Operation counts, shots and executions are summed.
    pub fn aggregate_batch(
        &self,
        workloads: &[WorkloadShape],
    ) -> Result<ResourceEstimate, ResourceEstimationError> {
        if workloads.is_empty() {
            return Err(ResourceEstimationError::EmptyBatch);
        }

        let estimates = self.estimate_batch(workloads)?;

        let mut physical_qubits = 0_u64;
        let mut logical_qubits = 0_u64;
        let mut depth = 0_u64;
        let mut shots = 0_u64;
        let mut executions = 0_u64;
        let mut classical_bits = 0_u64;

        let mut operations = OperationResourceRequirement::default();

        for estimate in &estimates {
            physical_qubits =
                physical_qubits.max(estimate.physical.qubits);

            logical_qubits =
                logical_qubits.max(estimate.logical.logical_qubits);

            depth = depth
                .checked_add(estimate.depth)
                .ok_or(ResourceEstimationError::ArithmeticOverflow {
                    operation: "batch_depth",
                })?;

            shots = shots
                .checked_add(estimate.shots)
                .ok_or(ResourceEstimationError::ArithmeticOverflow {
                    operation: "batch_shots",
                })?;

            executions = executions
                .checked_add(estimate.executions)
                .ok_or(ResourceEstimationError::ArithmeticOverflow {
                    operation: "batch_executions",
                })?;

            classical_bits = classical_bits
                .checked_add(estimate.classical.bits)
                .ok_or(ResourceEstimationError::ArithmeticOverflow {
                    operation: "batch_classical_bits",
                })?;

            operations.total_operations = operations
                .total_operations
                .checked_add(estimate.operations.total_operations)
                .ok_or(ResourceEstimationError::ArithmeticOverflow {
                    operation: "batch_operations",
                })?;

            operations.single_qubit_operations = operations
                .single_qubit_operations
                .checked_add(
                    estimate.operations.single_qubit_operations,
                )
                .ok_or(ResourceEstimationError::ArithmeticOverflow {
                    operation: "batch_single_qubit_operations",
                })?;

            operations.two_qubit_operations = operations
                .two_qubit_operations
                .checked_add(
                    estimate.operations.two_qubit_operations,
                )
                .ok_or(ResourceEstimationError::ArithmeticOverflow {
                    operation: "batch_two_qubit_operations",
                })?;

            operations.three_qubit_operations = operations
                .three_qubit_operations
                .checked_add(
                    estimate.operations.three_qubit_operations,
                )
                .ok_or(ResourceEstimationError::ArithmeticOverflow {
                    operation: "batch_three_qubit_operations",
                })?;

            operations.multi_qubit_operations = operations
                .multi_qubit_operations
                .checked_add(
                    estimate.operations.multi_qubit_operations,
                )
                .ok_or(ResourceEstimationError::ArithmeticOverflow {
                    operation: "batch_multi_qubit_operations",
                })?;

            operations.measurements = operations
                .measurements
                .checked_add(estimate.operations.measurements)
                .ok_or(ResourceEstimationError::ArithmeticOverflow {
                    operation: "batch_measurements",
                })?;

            operations.resets = operations
                .resets
                .checked_add(estimate.operations.resets)
                .ok_or(ResourceEstimationError::ArithmeticOverflow {
                    operation: "batch_resets",
                })?;
        }

        let duration = estimates
            .iter()
            .filter_map(|estimate| estimate.duration.duration)
            .try_fold(DurationNanos::ZERO, |acc, duration| {
                acc.checked_add(duration)
            })
            .map(|duration| DurationEstimate {
                duration: Some(duration),
                confidence: EstimateConfidence::Bounded,
                assumes_sequential_execution: true,
                includes_shots: true,
            })
            .unwrap_or_else(|_| DurationEstimate::unavailable());

        let mut diagnostics = Vec::new();

        if estimates
            .iter()
            .any(|estimate| !estimate.has_duration())
        {
            diagnostics.push(EstimateDiagnostic::warning(
                EstimateDiagnosticCode::TimingUnavailable,
                "batch.duration",
                "one or more workloads has unavailable duration information",
            ));
        }

        Ok(ResourceEstimate {
            schema_version: RESOURCE_ESTIMATOR_SCHEMA_VERSION,
            workload_kind: WorkloadKind::Sampling,
            physical: PhysicalResourceRequirement::qubits(physical_qubits),
            logical: LogicalResourceRequirement::new(logical_qubits),
            classical: ClassicalResourceRequirement {
                bits: classical_bits,
                result_values: 0,
            },
            operations,
            depth,
            shots,
            executions,
            duration,
            cost: CostEstimate::unavailable(),
            preflight: None,
            diagnostics,
        })
    }
}

// =============================================================================
// Convenience helpers
// =============================================================================

/// Converts the repository's `usize`-based backend limit semantics into the
/// estimator's architecture-independent `u64` representation.
///
/// A zero input remains zero and therefore retains the repository convention:
/// unspecified/unbounded.
pub const fn backend_limit_from_usize(value: usize) -> u64 {
    value as u64
}

/// Creates an estimator backend profile from Zamani's existing backend-limit
/// semantic fields.
///
/// This helper deliberately accepts primitive values rather than importing
/// `backend.rs`, preventing a dependency cycle and allowing the foundation
/// module to remain independently testable.
///
/// Integration in `backend.rs` can therefore be a one-line conversion at the
/// call site without modifying this file.
pub const fn backend_profile_from_limits(
    max_qubits: usize,
    max_logical_qubits: usize,
    max_circuit_depth: usize,
    max_operations: usize,
    max_shots: usize,
    max_classical_bits: usize,
    max_concurrent_jobs: usize,
    max_batch_size: usize,
) -> BackendResourceProfile {
    BackendResourceProfile {
        max_qubits: max_qubits as u64,
        max_logical_qubits: max_logical_qubits as u64,
        max_circuit_depth: max_circuit_depth as u64,
        max_operations: max_operations as u64,
        max_shots: max_shots as u64,
        max_classical_bits: max_classical_bits as u64,
        max_concurrent_jobs: max_concurrent_jobs as u64,
        max_batch_size: max_batch_size as u64,
        timing_available: false,
        pricing_available: false,
    }
}

// =============================================================================
// Capability-check compatibility helper
// =============================================================================

impl CapabilityCheck {
    /// Returns the status of this capability requirement.
    pub const fn status(self) -> RequirementStatus {
        if !self.required {
            RequirementStatus::Satisfied
        } else if !self.known {
            RequirementStatus::Unknown
        } else if self.available {
            RequirementStatus::Satisfied
        } else {
            RequirementStatus::Exceeded
        }
    }
}

// =============================================================================
// Ordering helpers
// =============================================================================

impl PartialOrd for EstimateDiagnostic {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.severity.cmp(&other.severity))
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_timing() -> TimingProfile {
        TimingProfile {
            single_qubit: Some(OperationTiming::new(
                DurationNanos::from_micros(1).unwrap(),
            )),
            two_qubit: Some(OperationTiming::new(
                DurationNanos::from_micros(2).unwrap(),
            )),
            three_qubit: Some(OperationTiming::new(
                DurationNanos::from_micros(3).unwrap(),
            )),
            multi_qubit: Some(OperationTiming::new(
                DurationNanos::from_micros(4).unwrap(),
            )),
            measurement: Some(OperationTiming::new(
                DurationNanos::from_micros(5).unwrap(),
            )),
            reset: Some(OperationTiming::new(
                DurationNanos::from_micros(6).unwrap(),
            )),
            barrier: Some(OperationTiming::new(
                DurationNanos::from_micros(1).unwrap(),
            )),
            delay: Some(OperationTiming::new(
                DurationNanos::from_micros(1).unwrap(),
            )),
            feed_forward: Some(
                DurationNanos::from_micros(7).unwrap(),
            ),
            routing_operation: Some(OperationTiming::new(
                DurationNanos::from_micros(1).unwrap(),
            )),
            fixed_overhead: Some(
                DurationNanos::from_micros(10).unwrap(),
            ),
        }
    }

    #[test]
    fn duration_units_are_exact() {
        assert_eq!(
            DurationNanos::from_micros(1).unwrap().as_nanos(),
            1_000
        );

        assert_eq!(
            DurationNanos::from_millis(1).unwrap().as_nanos(),
            1_000_000
        );

        assert_eq!(
            DurationNanos::from_secs(1).unwrap().as_nanos(),
            1_000_000_000
        );
    }

    #[test]
    fn duration_overflow_is_rejected() {
        let result = DurationNanos::from_secs(u64::MAX);

        assert!(matches!(
            result,
            Err(ResourceEstimationError::ArithmeticOverflow { .. })
        ));
    }

    #[test]
    fn invalid_zero_qubit_workload_is_rejected() {
        let workload = WorkloadShape {
            shots: 1,
            ..WorkloadShape::default()
        };

        assert!(workload.validate().is_err());
    }

    #[test]
    fn invalid_zero_shots_are_rejected() {
        let workload = WorkloadShape {
            logical_qubits: 1,
            physical_qubits: 1,
            shots: 0,
            ..WorkloadShape::default()
        };

        assert!(workload.validate().is_err());
    }

    #[test]
    fn circuit_constructor_is_valid() {
        let workload =
            WorkloadShape::circuit(4, 10, 20, 100);

        assert!(workload.validate().is_ok());
        assert_eq!(workload.logical_qubits, 4);
        assert_eq!(workload.depth, 10);
        assert_eq!(workload.operations.total_operations, 20);
        assert_eq!(workload.shots, 100);
    }

    #[test]
    fn sequential_duration_is_deterministic() {
        let workload = WorkloadShape {
            logical_qubits: 2,
            physical_qubits: 2,
            depth: 3,
            operations: OperationResourceRequirement {
                total_operations: 3,
                single_qubit_operations: 2,
                two_qubit_operations: 1,
                ..OperationResourceRequirement::default()
            },
            shots: 1,
            executions: 1,
            ..WorkloadShape::default()
        };

        let estimator = ResourceEstimator::new()
            .with_timing_profile(basic_timing())
            .with_options(EstimationOptions {
                parallelism: ParallelismModel::Sequential,
                include_routing_overhead: false,
                include_fixed_overhead: false,
                multiply_by_shots: false,
                multiply_by_executions: false,
                calculate_cost: false,
            });

        let result = estimator.estimate(&workload).unwrap();

        assert_eq!(
            result.duration.duration.unwrap().as_nanos(),
            4_000
        );
    }

    #[test]
    fn depth_based_duration_uses_parallel_layers() {
        let workload = WorkloadShape {
            logical_qubits: 2,
            physical_qubits: 2,
            depth: 3,
            operations: OperationResourceRequirement {
                total_operations: 4,
                single_qubit_operations: 2,
                two_qubit_operations: 2,
                ..OperationResourceRequirement::default()
            },
            shots: 1,
            executions: 1,
            ..WorkloadShape::default()
        };

        let estimator = ResourceEstimator::new()
            .with_timing_profile(basic_timing())
            .with_options(EstimationOptions {
                parallelism: ParallelismModel::DepthBased,
                include_routing_overhead: false,
                include_fixed_overhead: false,
                multiply_by_shots: false,
                multiply_by_executions: false,
                calculate_cost: false,
            });

        let result = estimator.estimate(&workload).unwrap();

        assert_eq!(
            result.duration.duration.unwrap().as_nanos(),
            6_000
        );
    }

    #[test]
    fn shots_scale_duration_when_enabled() {
        let workload = WorkloadShape {
            logical_qubits: 1,
            physical_qubits: 1,
            depth: 1,
            operations: OperationResourceRequirement {
                total_operations: 1,
                single_qubit_operations: 1,
                ..OperationResourceRequirement::default()
            },
            shots: 100,
            executions: 1,
            ..WorkloadShape::default()
        };

        let estimator = ResourceEstimator::new()
            .with_timing_profile(TimingProfile {
                single_qubit: Some(OperationTiming::new(
                    DurationNanos::from_micros(1).unwrap(),
                )),
                ..TimingProfile::new()
            })
            .with_options(EstimationOptions {
                parallelism: ParallelismModel::Sequential,
                include_routing_overhead: false,
                include_fixed_overhead: false,
                multiply_by_shots: true,
                multiply_by_executions: false,
                calculate_cost: false,
            });

        let result = estimator.estimate(&workload).unwrap();

        assert_eq!(
            result.duration.duration.unwrap().as_nanos(),
            100_000
        );
    }

    #[test]
    fn executions_scale_duration_when_enabled() {
        let workload = WorkloadShape {
            logical_qubits: 1,
            physical_qubits: 1,
            depth: 1,
            operations: OperationResourceRequirement {
                total_operations: 1,
                single_qubit_operations: 1,
                ..OperationResourceRequirement::default()
            },
            shots: 1,
            executions: 10,
            ..WorkloadShape::default()
        };

        let estimator = ResourceEstimator::new()
            .with_timing_profile(TimingProfile {
                single_qubit: Some(OperationTiming::new(
                    DurationNanos::from_micros(1).unwrap(),
                )),
                ..TimingProfile::new()
            })
            .with_options(EstimationOptions {
                parallelism: ParallelismModel::Sequential,
                include_routing_overhead: false,
                include_fixed_overhead: false,
                multiply_by_shots: false,
                multiply_by_executions: true,
                calculate_cost: false,
            });

        let result = estimator.estimate(&workload).unwrap();

        assert_eq!(
            result.duration.duration.unwrap().as_nanos(),
            10_000
        );
    }

    #[test]
    fn fixed_overhead_is_included() {
        let workload = WorkloadShape {
            logical_qubits: 1,
            physical_qubits: 1,
            depth: 1,
            operations: OperationResourceRequirement {
                total_operations: 1,
                single_qubit_operations: 1,
                ..OperationResourceRequirement::default()
            },
            shots: 1,
            executions: 1,
            ..WorkloadShape::default()
        };

        let estimator = ResourceEstimator::new()
            .with_timing_profile(basic_timing())
            .with_options(EstimationOptions {
                parallelism: ParallelismModel::Sequential,
                include_routing_overhead: false,
                include_fixed_overhead: true,
                multiply_by_shots: false,
                multiply_by_executions: false,
                calculate_cost: false,
            });

        let result = estimator.estimate(&workload).unwrap();

        assert_eq!(
            result.duration.duration.unwrap().as_nanos(),
            11_000
        );
    }

    #[test]
    fn missing_timing_is_explicit() {
        let workload = WorkloadShape {
            logical_qubits: 1,
            physical_qubits: 1,
            depth: 1,
            operations: OperationResourceRequirement {
                total_operations: 1,
                single_qubit_operations: 1,
                ..OperationResourceRequirement::default()
            },
            shots: 1,
            executions: 1,
            ..WorkloadShape::default()
        };

        let estimator = ResourceEstimator::new()
            .with_options(EstimationOptions {
                parallelism: ParallelismModel::Sequential,
                include_routing_overhead: false,
                include_fixed_overhead: false,
                multiply_by_shots: false,
                multiply_by_executions: false,
                calculate_cost: false,
            });

        let result = estimator.estimate(&workload);

        assert!(result.is_err());
    }

    #[test]
    fn cost_per_shot_is_calculated() {
        let workload = WorkloadShape {
            logical_qubits: 1,
            physical_qubits: 1,
            depth: 1,
            operations: OperationResourceRequirement {
                total_operations: 1,
                single_qubit_operations: 1,
                ..OperationResourceRequirement::default()
            },
            shots: 100,
            executions: 1,
            ..WorkloadShape::default()
        };

        let pricing = PricingProfile {
            cost_per_shot: Some(0.01),
            cost_per_execution: None,
            cost_per_operation: None,
            cost_per_second: None,
            unit: "USD",
        };

        let estimator = ResourceEstimator::new()
            .with_pricing_profile(pricing)
            .with_options(EstimationOptions {
                calculate_cost: true,
                ..EstimationOptions::default()
            });

        let result = estimator.estimate(&workload).unwrap();

        assert_eq!(
            result.cost.cost.unwrap().amount,
            1.0
        );

        assert_eq!(
            result.cost.cost.unwrap().unit,
            "USD"
        );
    }

    #[test]
    fn missing_pricing_does_not_fabricate_cost() {
        let workload =
            WorkloadShape::circuit(2, 2, 2, 10);

        let estimator = ResourceEstimator::new();

        let result = estimator.estimate(&workload).unwrap();

        assert!(!result.has_cost());

        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == EstimateDiagnosticCode::CostUnavailable
        }));
    }

    #[test]
    fn backend_limit_preflight_detects_excess_qubits() {
        let workload =
            WorkloadShape::circuit(10, 2, 4, 100);

        let backend = BackendResourceProfile {
            max_qubits: 5,
            max_logical_qubits: 5,
            max_circuit_depth: 100,
            max_operations: 1_000,
            max_shots: 10_000,
            max_classical_bits: 100,
            max_concurrent_jobs: 10,
            max_batch_size: 10,
            timing_available: true,
            pricing_available: false,
        };

        let estimator = ResourceEstimator::new()
            .with_backend_profile(backend);

        let result = estimator.estimate(&workload).unwrap();

        let preflight = result.preflight.unwrap();

        assert!(!preflight.is_not_exceeded());

        assert!(preflight.resource_limits.iter().any(|check| {
            check.resource == "max_qubits"
                && check.status == RequirementStatus::Exceeded
        }));
    }

    #[test]
    fn unknown_backend_limit_is_not_reported_as_zero_capacity() {
        let workload =
            WorkloadShape::circuit(2, 2, 2, 10);

        let backend = BackendResourceProfile::unknown();

        let estimator = ResourceEstimator::new()
            .with_backend_profile(backend);

        let result = estimator.estimate(&workload).unwrap();

        let preflight = result.preflight.unwrap();

        assert!(preflight.is_not_exceeded());
        assert!(!preflight.is_fully_satisfied());
    }

    #[test]
    fn capability_failure_is_detected() {
        let workload = WorkloadShape {
            logical_qubits: 2,
            physical_qubits: 2,
            depth: 4,
            operations: OperationResourceRequirement {
                total_operations: 5,
                measurements: 1,
                ..OperationResourceRequirement::default()
            },
            shots: 10,
            executions: 1,
            requires_dynamic_control: true,
            ..WorkloadShape::default()
        };

        let capabilities = AvailableCapabilities {
            measurement: true,
            reset: true,
            mid_circuit_measurement: false,
            classical_control: false,
            dynamic_circuits: false,
            ..AvailableCapabilities::default()
        };

        let estimator = ResourceEstimator::new()
            .with_backend_profile(BackendResourceProfile {
                max_qubits: 10,
                max_logical_qubits: 10,
                max_circuit_depth: 100,
                max_operations: 1_000,
                max_shots: 10_000,
                max_classical_bits: 100,
                max_concurrent_jobs: 10,
                max_batch_size: 10,
                timing_available: false,
                pricing_available: false,
            })
            .with_capabilities(capabilities);

        let result = estimator.estimate(&workload).unwrap();

        let preflight = result.preflight.unwrap();

        assert!(!preflight.is_not_exceeded());

        assert!(preflight.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == EstimateDiagnosticCode::CapabilityMissing
        }));
    }

    #[test]
    fn capability_not_required_is_satisfied() {
        let check = CapabilityCheck::evaluate(
            "pulse_control",
            false,
            false,
            true,
        );

        assert_eq!(
            check.status(),
            RequirementStatus::Satisfied
        );
    }

    #[test]
    fn batch_estimation_preserves_independence() {
        let timing = TimingProfile {
            single_qubit: Some(OperationTiming::new(
                DurationNanos::from_micros(1).unwrap(),
            )),
            ..TimingProfile::new()
        };

        let workloads = [
            WorkloadShape {
                logical_qubits: 1,
                physical_qubits: 1,
                depth: 1,
                operations: OperationResourceRequirement {
                    total_operations: 1,
                    single_qubit_operations: 1,
                    ..OperationResourceRequirement::default()
                },
                shots: 1,
                executions: 1,
                ..WorkloadShape::default()
            },
            WorkloadShape {
                logical_qubits: 2,
                physical_qubits: 2,
                depth: 2,
                operations: OperationResourceRequirement {
                    total_operations: 2,
                    single_qubit_operations: 2,
                    ..OperationResourceRequirement::default()
                },
                shots: 2,
                executions: 1,
                ..WorkloadShape::default()
            },
        ];

        let estimator = ResourceEstimator::new()
            .with_timing_profile(timing);

        let results =
            estimator.estimate_batch(&workloads).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(
            results[0].physical.qubits,
            1
        );
        assert_eq!(
            results[1].physical.qubits,
            2
        );
    }

    #[test]
    fn empty_batch_is_rejected() {
        let estimator = ResourceEstimator::new();

        assert_eq!(
            estimator.estimate_batch(&[]).unwrap_err(),
            ResourceEstimationError::EmptyBatch
        );
    }

    #[test]
    fn aggregate_batch_uses_maximum_concurrent_qubits() {
        let timing = TimingProfile {
            single_qubit: Some(OperationTiming::new(
                DurationNanos::from_micros(1).unwrap(),
            )),
            ..TimingProfile::new()
        };

        let workloads = [
            WorkloadShape::circuit(4, 2, 2, 10),
            WorkloadShape::circuit(8, 3, 3, 10),
        ];

        let estimator = ResourceEstimator::new()
            .with_timing_profile(timing);

        let aggregate =
            estimator.aggregate_batch(&workloads).unwrap();

        assert_eq!(
            aggregate.physical.qubits,
            8
        );

        assert_eq!(
            aggregate.operations.total_operations,
            5
        );
    }

    #[test]
    fn workload_kind_has_stable_names() {
        assert_eq!(
            WorkloadKind::Circuit.as_str(),
            "circuit"
        );

        assert_eq!(
            WorkloadKind::DynamicCircuit.as_str(),
            "dynamic_circuit"
        );

        assert_eq!(
            WorkloadKind::Analog.as_str(),
            "analog"
        );

        assert_eq!(
            WorkloadKind::Annealing.as_str(),
            "annealing"
        );
    }

    #[test]
    fn diagnostics_have_stable_codes() {
        assert_eq!(
            EstimateDiagnosticCode::TimingUnavailable.as_str(),
            "timing_unavailable"
        );

        assert_eq!(
            EstimateDiagnosticCode::CapabilityMissing.as_str(),
            "capability_missing"
        );
    }

    #[test]
    fn pricing_rejects_nan() {
        let pricing = PricingProfile {
            cost_per_shot: Some(f64::NAN),
            ..PricingProfile::new("USD")
        };

        assert!(pricing.validate().is_err());
    }

    #[test]
    fn pricing_rejects_negative_values() {
        let pricing = PricingProfile {
            cost_per_shot: Some(-1.0),
            ..PricingProfile::new("USD")
        };

        assert!(pricing.validate().is_err());
    }

    #[test]
    fn duration_display_is_human_readable() {
        assert_eq!(
            DurationNanos::from_nanos(500).to_string(),
            "500ns"
        );

        assert_eq!(
            DurationNanos::from_micros(10).unwrap().to_string(),
            "10.000µs"
        );
    }

    #[test]
    fn backend_profile_conversion_preserves_limits() {
        let profile = backend_profile_from_limits(
            127,
            32,
            1_000,
            100_000,
            10_000,
            1_000,
            4,
            16,
        );

        assert_eq!(profile.max_qubits, 127);
        assert_eq!(profile.max_logical_qubits, 32);
        assert_eq!(profile.max_circuit_depth, 1_000);
        assert_eq!(profile.max_operations, 100_000);
        assert_eq!(profile.max_shots, 10_000);
        assert_eq!(profile.max_classical_bits, 1_000);
        assert_eq!(profile.max_concurrent_jobs, 4);
        assert_eq!(profile.max_batch_size, 16);
    }

    #[test]
    fn zero_backend_limit_means_unknown() {
        assert_eq!(
            ResourceLimitCheck::new("max_qubits", 10, 0).status,
            RequirementStatus::Unknown
        );
    }

    #[test]
    fn exact_backend_limit_is_satisfied() {
        assert_eq!(
            ResourceLimitCheck::new("max_qubits", 10, 10).status,
            RequirementStatus::Satisfied
        );
    }

    #[test]
    fn one_over_backend_limit_is_exceeded() {
        assert_eq!(
            ResourceLimitCheck::new("max_qubits", 11, 10).status,
            RequirementStatus::Exceeded
        );
    }

    #[test]
    fn logical_resource_can_record_code_distance() {
        let resource =
            LogicalResourceRequirement::new(20)
                .with_code_distance(7);

        assert_eq!(
            resource.logical_qubits,
            20
        );

        assert_eq!(
            resource.code_distance,
            Some(7)
        );
    }

    #[test]
    fn physical_resources_distinguish_qubits_and_modes() {
        let resource =
            PhysicalResourceRequirement::modes(12);

        assert_eq!(resource.qubits, 0);
        assert_eq!(resource.modes, 12);
        assert_eq!(resource.maximum_count(), 12);
    }

    #[test]
    fn result_is_provider_neutral() {
        let workload =
            WorkloadShape::circuit(4, 10, 20, 100);

        let result =
            ResourceEstimator::new()
                .with_timing_profile(basic_timing())
                .estimate(&workload)
                .unwrap();

        assert_eq!(
            result.schema_version,
            RESOURCE_ESTIMATOR_SCHEMA_VERSION
        );
    }
}