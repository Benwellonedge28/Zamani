//! Zamani Quantum Benchmarking — QEC Resource Overhead
//!
//! Production resource-accounting and overhead analysis for quantum
//! error-correction (QEC) and fault-tolerant quantum-computing benchmarks.
//!
//! # Responsibility
//!
//! This module owns the mathematical representation and calculation of
//! physical-to-logical resource overhead.
//!
//! It covers:
//!
//! - physical-qubit overhead;
//! - physical-gate overhead;
//! - two-qubit-gate overhead;
//! - circuit-depth overhead;
//! - two-qubit-depth overhead;
//! - T-gate overhead;
//! - generic-operation overhead;
//! - measurement overhead;
//! - correction-cycle overhead;
//! - execution-time overhead;
//! - circuit-count overhead;
//! - shot-count overhead;
//! - physical/logical resource ratios;
//! - qubit overhead per logical qubit;
//! - gate overhead per logical gate;
//! - physical depth per logical depth;
//! - physical time per logical time;
//! - space-time volume;
//! - space-time overhead;
//! - physical-to-logical resource summaries;
//! - safe aggregation of resource measurements.
//!
//! This module deliberately does NOT:
//!
//! - generate QEC circuits;
//! - execute QEC circuits;
//! - implement a QEC code;
//! - implement a decoder;
//! - determine whether a logical computation is correct;
//! - calculate physical error rates;
//! - calculate logical error rates;
//! - fit threshold curves;
//! - discover hardware capabilities;
//! - access calibration data;
//! - submit jobs to hardware;
//! - own the universal `BenchmarkResult` envelope;
//! - own benchmark registry logic;
//! - own report formatting.
//!
//! Those responsibilities belong to the corresponding QEC, execution,
//! hardware, statistics, result, registry, and reporting layers.
//!
//! # Architectural position
//!
//! ```text
//! QEC execution / compiler / scheduler / decoder
//!                         │
//!                         │ resource observations
//!                         ▼
//!             qec::resource_overhead
//!                         │
//!             ┌───────────┼───────────┐
//!             ▼           ▼           ▼
//!         qubit       gate/depth     time
//!         overhead    overhead       overhead
//!             │           │           │
//!             └───────────┼───────────┘
//!                         ▼
//!                  ResourceMetrics
//!                         │
//!             ┌───────────┴───────────┐
//!             ▼                       ▼
//!       core::metric              core::result
//!             │                       │
//!             ▼                       ▼
//!         analysis                reporting
//! ```
//!
//! # Dependency direction
//!
//! This file depends only on the foundational metric representation:
//!
//! ```text
//! qec::resource_overhead
//!          │
//!          └──> benchmarking::core::metric
//! ```
//!
//! It must never be inverted:
//!
//! ```text
//! core::metric
//!      │
//!      └──> qec::resource_overhead   // forbidden
//! ```
//!
//! # Scientific meaning
//!
//! Resource overhead is always relative to an explicitly supplied logical
//! reference. For example:
//!
//! ```text
//! qubit overhead = physical qubits / logical qubits
//! gate overhead  = physical gates  / logical gates
//! ```
//!
//! These ratios are not universal properties of a QEC code or hardware
//! platform. They depend on:
//!
//! - code family;
//! - code distance;
//! - logical gate implementation;
//! - syndrome-extraction schedule;
//! - decoder;
//! - compiler;
//! - routing;
//! - lattice/topology;
//! - circuit structure;
//! - fault-tolerant decomposition;
//! - measurement strategy;
//! - execution model.
//!
//! Consequently this module never infers missing denominators and never
//! substitutes one resource definition for another.
//!
//! # Space-time volume
//!
//! A generic discrete space-time volume is represented as:
//!
//! ```text
//! space_time_volume = physical_qubits × physical_time_steps
//! ```
//!
//! where the caller explicitly defines what one time step means.
//!
//! For wall-clock resource accounting, a second form is available:
//!
//! ```text
//! physical_qubits × execution_time_seconds
//! ```
//!
//! The unit and interpretation are preserved in the returned structure.
//!
//! This distinction is important: "space-time volume" must not silently mix
//! circuit cycles with seconds.
//!
//! # Production invariants
//!
//! 1. No NaN or infinity enters the module.
//! 2. No division by zero is permitted.
//! 3. No unsigned integer operation is allowed to wrap.
//! 4. No floating-point quantity is silently clamped.
//! 5. Negative durations are rejected.
//! 6. Zero logical resources are rejected when used as denominators.
//! 7. Resource ratios are always derived from explicitly supplied counts.
//! 8. Exact integer resource counts remain available in the result.
//! 9. Resource multiplication uses checked arithmetic.
//! 10. Large integer products are calculated in `u128` before conversion.
//! 11. Metric construction uses the canonical benchmarking `Metric` type.
//! 12. No library diagnostic printing occurs.
//! 13. No hardware/network/global state is accessed.
//! 14. The module is deterministic for identical inputs.
//! 15. Resource categories are never silently combined.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! The module is intentionally complete before the following modules exist:
//!
//! - `qec/physical.rs`;
//! - `qec/logical.rs`;
//! - `qec/decoder.rs`;
//! - `qec/threshold.rs`;
//! - `analysis/*`;
//! - `reporting/*`;
//! - `core/result.rs`;
//! - `registry/*`.
//!
//! Those modules may consume the types defined here without changing their
//! semantics.
//!
//! Typical integration is:
//!
//! ```text
//! quantum::error_correction
//!          │
//!          ▼
//! QEC execution/resource observations
//!          │
//!          ▼
//! ResourceSnapshot::new(...)
//!          │
//!          ▼
//! ResourceOverhead::between(...)
//!          │
//!          ├──> qubit_overhead_metric()
//!          ├──> gate_overhead_metric()
//!          ├──> depth_overhead_metric()
//!          ├──> time_overhead_metric()
//!          └──> space_time_overhead_metric()
//! ```
//!
//! No later modification of this file is required merely because those
//! downstream integrations are added.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;

use serde::{Deserialize, Serialize};

use crate::quantum::benchmarking::core::metric::{
    Metric,
    MetricKind,
    MetricResult,
    MetricUnit,
};

// ============================================================================
// Public constants
// ============================================================================

/// Stable benchmark-analysis identifier.
pub const QEC_RESOURCE_OVERHEAD_BENCHMARK_ID: &str = "qec.resource_overhead";

/// Version of the resource-overhead mathematical contract.
///
/// Increment this only when the meaning of an existing field or calculation
/// changes incompatibly.
pub const QEC_RESOURCE_OVERHEAD_VERSION: u32 = 1;

/// Maximum length accepted for a user-defined resource label.
pub const MAX_RESOURCE_LABEL_BYTES: usize = 128;

/// Maximum supported code distance.
///
/// This is a validation/safety limit for benchmark metadata, not a limit on
/// what a QEC implementation may theoretically support.
pub const MAX_CODE_DISTANCE: u64 = 1_000_000;

/// Maximum number of correction cycles represented by one resource snapshot.
pub const MAX_CORRECTION_CYCLES: u64 = 1_000_000_000_000;

/// Maximum number of time steps represented by one snapshot.
pub const MAX_TIME_STEPS: u64 = 1_000_000_000_000;

/// Maximum physical/logical resource count accepted by one snapshot.
///
/// The limit prevents malformed benchmark input from becoming an accidental
/// integer-arithmetic stressor.
pub const MAX_RESOURCE_COUNT: u64 = 1_000_000_000_000_000;

/// Maximum number of circuits represented by one snapshot.
pub const MAX_CIRCUITS: u64 = 1_000_000_000_000;

/// Maximum number of shots represented by one snapshot.
pub const MAX_SHOTS: u64 = 1_000_000_000_000_000;

// ============================================================================
// Result aliases
// ============================================================================

/// Result type for resource-overhead operations.
pub type ResourceOverheadResult<T> = Result<T, ResourceOverheadError>;

// ============================================================================
// Resource kinds
// ============================================================================

/// Resource category used in overhead calculations.
///
/// Keeping resource categories explicit prevents accidental comparison of
/// unrelated quantities such as gates versus seconds.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ResourceKind {
    /// Number of physical qubits.
    Qubits,

    /// Number of logical qubits.
    LogicalQubits,

    /// Number of physical gates.
    Gates,

    /// Number of logical gates.
    LogicalGates,

    /// Number of physical two-qubit gates.
    TwoQubitGates,

    /// Number of logical two-qubit gates.
    LogicalTwoQubitGates,

    /// Circuit depth.
    Depth,

    /// Two-qubit circuit depth.
    TwoQubitDepth,

    /// T-gate count.
    TGates,

    /// Logical T-gate count.
    LogicalTGates,

    /// Generic operation count.
    Operations,

    /// Logical operation count.
    LogicalOperations,

    /// Measurement count.
    Measurements,

    /// Logical measurement count.
    LogicalMeasurements,

    /// Error-correction cycle count.
    CorrectionCycles,

    /// Circuit count.
    Circuits,

    /// Shot count.
    Shots,

    /// Execution time.
    ExecutionTime,

    /// Custom resource category.
    Custom,
}

impl ResourceKind {
    /// Stable machine-readable identifier.
    pub const fn id(self) -> &'static str {
        match self {
            Self::Qubits => "qubits",
            Self::LogicalQubits => "logical_qubits",
            Self::Gates => "gates",
            Self::LogicalGates => "logical_gates",
            Self::TwoQubitGates => "two_qubit_gates",
            Self::LogicalTwoQubitGates => "logical_two_qubit_gates",
            Self::Depth => "depth",
            Self::TwoQubitDepth => "two_qubit_depth",
            Self::TGates => "t_gates",
            Self::LogicalTGates => "logical_t_gates",
            Self::Operations => "operations",
            Self::LogicalOperations => "logical_operations",
            Self::Measurements => "measurements",
            Self::LogicalMeasurements => "logical_measurements",
            Self::CorrectionCycles => "correction_cycles",
            Self::Circuits => "circuits",
            Self::Shots => "shots",
            Self::ExecutionTime => "execution_time",
            Self::Custom => "custom",
        }
    }
}

// ============================================================================
// Resource snapshot
// ============================================================================

/// Exact resource counts for one physical or logical execution layer.
///
/// A snapshot contains both physical and logical resources. This allows one
/// object to describe the complete basis from which overhead is calculated.
///
/// Every integer resource remains exact in this structure. Floating-point
/// conversion occurs only when constructing a `Metric`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceSnapshot {
    /// Physical qubit count.
    pub physical_qubits: u64,

    /// Logical qubit count.
    pub logical_qubits: u64,

    /// Physical gate count.
    pub physical_gates: u64,

    /// Logical gate count.
    pub logical_gates: u64,

    /// Physical two-qubit gate count.
    pub physical_two_qubit_gates: u64,

    /// Logical two-qubit gate count.
    pub logical_two_qubit_gates: u64,

    /// Physical circuit depth.
    pub physical_depth: u64,

    /// Logical circuit depth.
    pub logical_depth: u64,

    /// Physical two-qubit depth.
    pub physical_two_qubit_depth: u64,

    /// Logical two-qubit depth.
    pub logical_two_qubit_depth: u64,

    /// Physical T-gate count.
    pub physical_t_gates: u64,

    /// Logical T-gate count.
    pub logical_t_gates: u64,

    /// Physical operation count.
    pub physical_operations: u64,

    /// Logical operation count.
    pub logical_operations: u64,

    /// Physical measurement count.
    pub physical_measurements: u64,

    /// Logical measurement count.
    pub logical_measurements: u64,

    /// Number of error-correction cycles.
    pub correction_cycles: u64,

    /// Number of circuits.
    pub circuits: u64,

    /// Number of shots.
    pub shots: u64,

    /// Physical execution time in seconds.
    pub physical_execution_time_seconds: f64,

    /// Logical/reference execution time in seconds.
    pub logical_execution_time_seconds: f64,

    /// Optional discrete physical time-step count.
    ///
    /// This is useful for space-time volume where the time axis is measured in
    /// circuit cycles rather than wall-clock seconds.
    pub physical_time_steps: Option<u64>,

    /// Optional logical/reference time-step count.
    pub logical_time_steps: Option<u64>,

    /// Optional QEC code distance.
    pub code_distance: Option<u64>,

    /// Optional human-readable code identifier.
    pub code_id: Option<String>,

    /// Optional decoder identifier.
    pub decoder_id: Option<String>,

    /// Optional backend identifier.
    pub backend_id: Option<String>,
}

impl Default for ResourceSnapshot {
    fn default() -> Self {
        Self {
            physical_qubits: 0,
            logical_qubits: 0,
            physical_gates: 0,
            logical_gates: 0,
            physical_two_qubit_gates: 0,
            logical_two_qubit_gates: 0,
            physical_depth: 0,
            logical_depth: 0,
            physical_two_qubit_depth: 0,
            logical_two_qubit_depth: 0,
            physical_t_gates: 0,
            logical_t_gates: 0,
            physical_operations: 0,
            logical_operations: 0,
            physical_measurements: 0,
            logical_measurements: 0,
            correction_cycles: 0,
            circuits: 0,
            shots: 0,
            physical_execution_time_seconds: 0.0,
            logical_execution_time_seconds: 0.0,
            physical_time_steps: None,
            logical_time_steps: None,
            code_distance: None,
            code_id: None,
            decoder_id: None,
            backend_id: None,
        }
    }
}

impl ResourceSnapshot {
    /// Creates a validated resource snapshot.
    ///
    /// The constructor intentionally requires all primary counts explicitly.
    /// This prevents the overhead layer from inventing missing logical
    /// resources.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        physical_qubits: u64,
        logical_qubits: u64,
        physical_gates: u64,
        logical_gates: u64,
        physical_two_qubit_gates: u64,
        logical_two_qubit_gates: u64,
        physical_depth: u64,
        logical_depth: u64,
        physical_two_qubit_depth: u64,
        logical_two_qubit_depth: u64,
        physical_t_gates: u64,
        logical_t_gates: u64,
        physical_operations: u64,
        logical_operations: u64,
        physical_measurements: u64,
        logical_measurements: u64,
        correction_cycles: u64,
        circuits: u64,
        shots: u64,
        physical_execution_time_seconds: f64,
        logical_execution_time_seconds: f64,
    ) -> ResourceOverheadResult<Self> {
        let snapshot = Self {
            physical_qubits,
            logical_qubits,
            physical_gates,
            logical_gates,
            physical_two_qubit_gates,
            logical_two_qubit_gates,
            physical_depth,
            logical_depth,
            physical_two_qubit_depth,
            logical_two_qubit_depth,
            physical_t_gates,
            logical_t_gates,
            physical_operations,
            logical_operations,
            physical_measurements,
            logical_measurements,
            correction_cycles,
            circuits,
            shots,
            physical_execution_time_seconds,
            logical_execution_time_seconds,
            physical_time_steps: None,
            logical_time_steps: None,
            code_distance: None,
            code_id: None,
            decoder_id: None,
            backend_id: None,
        };

        snapshot.validate()?;
        Ok(snapshot)
    }

    /// Creates a zero-initialized snapshot that can subsequently be populated
    /// by a trusted adapter.
    ///
    /// This is useful for execution adapters that receive resource categories
    /// incrementally. The snapshot is not valid for overhead calculation until
    /// `validate()` succeeds.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Validates all resource fields.
    pub fn validate(&self) -> ResourceOverheadResult<()> {
        validate_resource_count(self.physical_qubits, "physical_qubits")?;
        validate_resource_count(self.logical_qubits, "logical_qubits")?;
        validate_resource_count(self.physical_gates, "physical_gates")?;
        validate_resource_count(self.logical_gates, "logical_gates")?;
        validate_resource_count(
            self.physical_two_qubit_gates,
            "physical_two_qubit_gates",
        )?;
        validate_resource_count(
            self.logical_two_qubit_gates,
            "logical_two_qubit_gates",
        )?;
        validate_resource_count(self.physical_depth, "physical_depth")?;
        validate_resource_count(self.logical_depth, "logical_depth")?;
        validate_resource_count(
            self.physical_two_qubit_depth,
            "physical_two_qubit_depth",
        )?;
        validate_resource_count(
            self.logical_two_qubit_depth,
            "logical_two_qubit_depth",
        )?;
        validate_resource_count(self.physical_t_gates, "physical_t_gates")?;
        validate_resource_count(self.logical_t_gates, "logical_t_gates")?;
        validate_resource_count(
            self.physical_operations,
            "physical_operations",
        )?;
        validate_resource_count(
            self.logical_operations,
            "logical_operations",
        )?;
        validate_resource_count(
            self.physical_measurements,
            "physical_measurements",
        )?;
        validate_resource_count(
            self.logical_measurements,
            "logical_measurements",
        )?;
        validate_resource_count(
            self.correction_cycles,
            "correction_cycles",
        )?;
        validate_circuit_count(self.circuits)?;
        validate_shot_count(self.shots)?;

        validate_non_negative_finite(
            self.physical_execution_time_seconds,
            "physical_execution_time_seconds",
        )?;

        validate_non_negative_finite(
            self.logical_execution_time_seconds,
            "logical_execution_time_seconds",
        )?;

        if let Some(steps) = self.physical_time_steps {
            validate_time_steps(steps, "physical_time_steps")?;
        }

        if let Some(steps) = self.logical_time_steps {
            validate_time_steps(steps, "logical_time_steps")?;
        }

        if let Some(distance) = self.code_distance {
            validate_code_distance(distance)?;
        }

        validate_optional_identifier(
            self.code_id.as_deref(),
            "code_id",
        )?;

        validate_optional_identifier(
            self.decoder_id.as_deref(),
            "decoder_id",
        )?;

        validate_optional_identifier(
            self.backend_id.as_deref(),
            "backend_id",
        )?;

        Ok(())
    }

    /// Adds discrete physical and logical time-step counts.
    pub fn with_time_steps(
        mut self,
        physical_time_steps: u64,
        logical_time_steps: u64,
    ) -> ResourceOverheadResult<Self> {
        validate_time_steps(
            physical_time_steps,
            "physical_time_steps",
        )?;

        validate_time_steps(
            logical_time_steps,
            "logical_time_steps",
        )?;

        self.physical_time_steps = Some(physical_time_steps);
        self.logical_time_steps = Some(logical_time_steps);

        Ok(self)
    }

    /// Adds QEC code distance.
    pub fn with_code_distance(
        mut self,
        code_distance: u64,
    ) -> ResourceOverheadResult<Self> {
        validate_code_distance(code_distance)?;
        self.code_distance = Some(code_distance);
        Ok(self)
    }

    /// Adds a code identifier.
    pub fn with_code_id(
        mut self,
        code_id: impl Into<String>,
    ) -> ResourceOverheadResult<Self> {
        let code_id = validate_identifier(
            code_id.into(),
            "code_id",
        )?;

        self.code_id = Some(code_id);
        Ok(self)
    }

    /// Adds a decoder identifier.
    pub fn with_decoder_id(
        mut self,
        decoder_id: impl Into<String>,
    ) -> ResourceOverheadResult<Self> {
        let decoder_id = validate_identifier(
            decoder_id.into(),
            "decoder_id",
        )?;

        self.decoder_id = Some(decoder_id);
        Ok(self)
    }

    /// Adds a backend identifier.
    pub fn with_backend_id(
        mut self,
        backend_id: impl Into<String>,
    ) -> ResourceOverheadResult<Self> {
        let backend_id = validate_identifier(
            backend_id.into(),
            "backend_id",
        )?;

        self.backend_id = Some(backend_id);
        Ok(self)
    }

    /// Returns the physical qubit count.
    pub const fn physical_qubits(&self) -> u64 {
        self.physical_qubits
    }

    /// Returns the logical qubit count.
    pub const fn logical_qubits(&self) -> u64 {
        self.logical_qubits
    }

    /// Returns the physical gate count.
    pub const fn physical_gates(&self) -> u64 {
        self.physical_gates
    }

    /// Returns the logical gate count.
    pub const fn logical_gates(&self) -> u64 {
        self.logical_gates
    }

    /// Returns the physical execution time in seconds.
    pub const fn physical_execution_time_seconds(&self) -> f64 {
        self.physical_execution_time_seconds
    }

    /// Returns the logical/reference execution time in seconds.
    pub const fn logical_execution_time_seconds(&self) -> f64 {
        self.logical_execution_time_seconds
    }
}

// ============================================================================
// Overhead calculation
// ============================================================================

/// Complete physical-to-logical resource overhead calculation.
///
/// Every ratio is independently calculated from its own resource pair.
///
/// A missing denominator is represented by `None` rather than being silently
/// substituted with another quantity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceOverhead {
    /// Original resource snapshot.
    pub snapshot: ResourceSnapshot,

    /// Physical qubits per logical qubit.
    pub qubit_overhead: Option<f64>,

    /// Physical gates per logical gate.
    pub gate_overhead: Option<f64>,

    /// Physical two-qubit gates per logical two-qubit gate.
    pub two_qubit_gate_overhead: Option<f64>,

    /// Physical depth per logical depth.
    pub depth_overhead: Option<f64>,

    /// Physical two-qubit depth per logical two-qubit depth.
    pub two_qubit_depth_overhead: Option<f64>,

    /// Physical T gates per logical T gate.
    pub t_gate_overhead: Option<f64>,

    /// Physical operations per logical operation.
    pub operation_overhead: Option<f64>,

    /// Physical measurements per logical measurement.
    pub measurement_overhead: Option<f64>,

    /// Physical execution time per logical/reference execution time.
    pub execution_time_overhead: Option<f64>,

    /// Physical time steps per logical time step.
    pub time_step_overhead: Option<f64>,

    /// Physical space-time volume in discrete units.
    pub physical_space_time_volume: Option<u128>,

    /// Logical/reference space-time volume in discrete units.
    pub logical_space_time_volume: Option<u128>,

    /// Physical discrete space-time overhead.
    pub space_time_overhead: Option<f64>,

    /// Code distance, when supplied.
    pub code_distance: Option<u64>,
}

impl ResourceOverhead {
    /// Calculates all available resource-overhead ratios.
    ///
    /// This function never substitutes a missing denominator and never
    /// silently treats zero as one.
    pub fn between(
        snapshot: ResourceSnapshot,
    ) -> ResourceOverheadResult<Self> {
        snapshot.validate()?;

        let qubit_overhead = ratio_if_nonzero(
            snapshot.physical_qubits,
            snapshot.logical_qubits,
            "qubit_overhead",
        )?;

        let gate_overhead = ratio_if_nonzero(
            snapshot.physical_gates,
            snapshot.logical_gates,
            "gate_overhead",
        )?;

        let two_qubit_gate_overhead = ratio_if_nonzero(
            snapshot.physical_two_qubit_gates,
            snapshot.logical_two_qubit_gates,
            "two_qubit_gate_overhead",
        )?;

        let depth_overhead = ratio_if_nonzero(
            snapshot.physical_depth,
            snapshot.logical_depth,
            "depth_overhead",
        )?;

        let two_qubit_depth_overhead = ratio_if_nonzero(
            snapshot.physical_two_qubit_depth,
            snapshot.logical_two_qubit_depth,
            "two_qubit_depth_overhead",
        )?;

        let t_gate_overhead = ratio_if_nonzero(
            snapshot.physical_t_gates,
            snapshot.logical_t_gates,
            "t_gate_overhead",
        )?;

        let operation_overhead = ratio_if_nonzero(
            snapshot.physical_operations,
            snapshot.logical_operations,
            "operation_overhead",
        )?;

        let measurement_overhead = ratio_if_nonzero(
            snapshot.physical_measurements,
            snapshot.logical_measurements,
            "measurement_overhead",
        )?;

        let execution_time_overhead =
            positive_f64_ratio_if_nonzero(
                snapshot.physical_execution_time_seconds,
                snapshot.logical_execution_time_seconds,
                "execution_time_overhead",
            )?;

        let time_step_overhead = match (
            snapshot.physical_time_steps,
            snapshot.logical_time_steps,
        ) {
            (Some(physical), Some(logical)) => {
                ratio_if_nonzero(
                    physical,
                    logical,
                    "time_step_overhead",
                )?
            }
            _ => None,
        };

        let physical_space_time_volume =
            match snapshot.physical_time_steps {
                Some(steps) => checked_mul_u128(
                    snapshot.physical_qubits as u128,
                    steps as u128,
                    "physical_space_time_volume",
                )?,
                None => None,
            };

        let logical_space_time_volume =
            match snapshot.logical_time_steps {
                Some(steps) => checked_mul_u128(
                    snapshot.logical_qubits as u128,
                    steps as u128,
                    "logical_space_time_volume",
                )?,
                None => None,
            };

        let space_time_overhead =
            match (
                physical_space_time_volume,
                logical_space_time_volume,
            ) {
                (Some(physical), Some(logical)) => {
                    ratio_u128_if_nonzero(
                        physical,
                        logical,
                        "space_time_overhead",
                    )?
                }
                _ => None,
            };

        Ok(Self {
            code_distance: snapshot.code_distance,
            snapshot,
            qubit_overhead,
            gate_overhead,
            two_qubit_gate_overhead,
            depth_overhead,
            two_qubit_depth_overhead,
            t_gate_overhead,
            operation_overhead,
            measurement_overhead,
            execution_time_overhead,
            time_step_overhead,
            physical_space_time_volume,
            logical_space_time_volume,
            space_time_overhead,
        })
    }

    /// Calculates overhead directly from a validated snapshot.
    pub fn from_snapshot(
        snapshot: ResourceSnapshot,
    ) -> ResourceOverheadResult<Self> {
        Self::between(snapshot)
    }

    /// Returns qubit overhead.
    pub const fn qubit_overhead(&self) -> Option<f64> {
        self.qubit_overhead
    }

    /// Returns gate overhead.
    pub const fn gate_overhead(&self) -> Option<f64> {
        self.gate_overhead
    }

    /// Returns two-qubit-gate overhead.
    pub const fn two_qubit_gate_overhead(&self) -> Option<f64> {
        self.two_qubit_gate_overhead
    }

    /// Returns circuit-depth overhead.
    pub const fn depth_overhead(&self) -> Option<f64> {
        self.depth_overhead
    }

    /// Returns execution-time overhead.
    pub const fn execution_time_overhead(&self) -> Option<f64> {
        self.execution_time_overhead
    }

    /// Returns discrete space-time overhead.
    pub const fn space_time_overhead(&self) -> Option<f64> {
        self.space_time_overhead
    }

    /// Constructs the canonical qubit-overhead metric.
    pub fn qubit_overhead_metric(&self) -> ResourceOverheadResult<Metric> {
        self.ratio_metric(
            self.qubit_overhead,
            "physical qubits per logical qubit",
        )
    }

    /// Constructs the canonical gate-overhead metric.
    pub fn gate_overhead_metric(&self) -> ResourceOverheadResult<Metric> {
        self.ratio_metric(
            self.gate_overhead,
            "physical gates per logical gate",
        )
    }

    /// Constructs the canonical two-qubit-gate-overhead metric.
    pub fn two_qubit_gate_overhead_metric(
        &self,
    ) -> ResourceOverheadResult<Metric> {
        self.ratio_metric(
            self.two_qubit_gate_overhead,
            "physical two-qubit gates per logical two-qubit gate",
        )
    }

    /// Constructs the canonical circuit-depth-overhead metric.
    pub fn depth_overhead_metric(&self) -> ResourceOverheadResult<Metric> {
        self.ratio_metric(
            self.depth_overhead,
            "physical depth per logical depth",
        )
    }

    /// Constructs the canonical two-qubit-depth-overhead metric.
    pub fn two_qubit_depth_overhead_metric(
        &self,
    ) -> ResourceOverheadResult<Metric> {
        self.ratio_metric(
            self.two_qubit_depth_overhead,
            "physical two-qubit depth per logical two-qubit depth",
        )
    }

    /// Constructs the canonical T-gate-overhead metric.
    pub fn t_gate_overhead_metric(&self) -> ResourceOverheadResult<Metric> {
        self.ratio_metric(
            self.t_gate_overhead,
            "physical T gates per logical T gate",
        )
    }

    /// Constructs the canonical operation-overhead metric.
    pub fn operation_overhead_metric(
        &self,
    ) -> ResourceOverheadResult<Metric> {
        self.ratio_metric(
            self.operation_overhead,
            "physical operations per logical operation",
        )
    }

    /// Constructs the canonical measurement-overhead metric.
    pub fn measurement_overhead_metric(
        &self,
    ) -> ResourceOverheadResult<Metric> {
        self.ratio_metric(
            self.measurement_overhead,
            "physical measurements per logical measurement",
        )
    }

    /// Constructs the canonical execution-time-overhead metric.
    pub fn execution_time_overhead_metric(
        &self,
    ) -> ResourceOverheadResult<Metric> {
        self.ratio_metric(
            self.execution_time_overhead,
            "physical execution time per logical execution time",
        )
    }

    /// Constructs the canonical discrete space-time-overhead metric.
    pub fn space_time_overhead_metric(
        &self,
    ) -> ResourceOverheadResult<Metric> {
        self.ratio_metric(
            self.space_time_overhead,
            "physical space-time volume per logical space-time volume",
        )
    }

    /// Returns all available canonical metrics in a deterministic order.
    ///
    /// Metrics whose denominator was not supplied are omitted rather than
    /// fabricated.
    pub fn metrics(&self) -> ResourceOverheadResult<Vec<Metric>> {
        let mut metrics = Vec::with_capacity(11);

        if self.qubit_overhead.is_some() {
            metrics.push(self.qubit_overhead_metric()?);
        }

        if self.gate_overhead.is_some() {
            metrics.push(self.gate_overhead_metric()?);
        }

        if self.two_qubit_gate_overhead.is_some() {
            metrics.push(self.two_qubit_gate_overhead_metric()?);
        }

        if self.depth_overhead.is_some() {
            metrics.push(self.depth_overhead_metric()?);
        }

        if self.two_qubit_depth_overhead.is_some() {
            metrics.push(self.two_qubit_depth_overhead_metric()?);
        }

        if self.t_gate_overhead.is_some() {
            metrics.push(self.t_gate_overhead_metric()?);
        }

        if self.operation_overhead.is_some() {
            metrics.push(self.operation_overhead_metric()?);
        }

        if self.measurement_overhead.is_some() {
            metrics.push(self.measurement_overhead_metric()?);
        }

        if self.execution_time_overhead.is_some() {
            metrics.push(self.execution_time_overhead_metric()?);
        }

        if self.time_step_overhead.is_some() {
            metrics.push(self.ratio_metric(
                self.time_step_overhead,
                "physical time steps per logical time step",
            )?);
        }

        if self.space_time_overhead.is_some() {
            metrics.push(self.space_time_overhead_metric()?);
        }

        Ok(metrics)
    }

    fn ratio_metric(
        &self,
        value: Option<f64>,
        description: &'static str,
    ) -> ResourceOverheadResult<Metric> {
        let value = value.ok_or(
            ResourceOverheadError::MissingDenominator {
                metric: description,
            },
        )?;

        Metric::new(
            MetricKind::ResourceOverhead,
            MetricUnit::Dimensionless,
            value,
        )
        .map_err(ResourceOverheadError::metric_error)
        .and_then(|metric| {
            metric
                .with_description(description)
                .map_err(ResourceOverheadError::metric_error)
        })
    }
}

// ============================================================================
// Resource metric helpers
// ============================================================================

/// Creates a metric for the exact physical qubit count.
pub fn physical_qubit_count_metric(
    snapshot: &ResourceSnapshot,
) -> ResourceOverheadResult<Metric> {
    count_metric(
        MetricKind::PhysicalQubitCount,
        MetricUnit::PhysicalQubits,
        snapshot.physical_qubits,
        "physical qubit count",
    )
}

/// Creates a metric for the exact logical qubit count.
pub fn logical_qubit_count_metric(
    snapshot: &ResourceSnapshot,
) -> ResourceOverheadResult<Metric> {
    count_metric(
        MetricKind::LogicalQubitCount,
        MetricUnit::LogicalQubits,
        snapshot.logical_qubits,
        "logical qubit count",
    )
}

/// Creates a metric for the exact physical gate count.
pub fn physical_gate_count_metric(
    snapshot: &ResourceSnapshot,
) -> ResourceOverheadResult<Metric> {
    count_metric(
        MetricKind::GateCount,
        MetricUnit::Gates,
        snapshot.physical_gates,
        "physical gate count",
    )
}

/// Creates a metric for the exact physical two-qubit gate count.
pub fn physical_two_qubit_gate_count_metric(
    snapshot: &ResourceSnapshot,
) -> ResourceOverheadResult<Metric> {
    count_metric(
        MetricKind::TwoQubitGateCount,
        MetricUnit::TwoQubitGates,
        snapshot.physical_two_qubit_gates,
        "physical two-qubit gate count",
    )
}

/// Creates a metric for physical circuit depth.
pub fn physical_depth_metric(
    snapshot: &ResourceSnapshot,
) -> ResourceOverheadResult<Metric> {
    count_metric(
        MetricKind::CircuitDepth,
        MetricUnit::Layers,
        snapshot.physical_depth,
        "physical circuit depth",
    )
}

/// Creates a metric for physical T-gate count.
pub fn physical_t_gate_count_metric(
    snapshot: &ResourceSnapshot,
) -> ResourceOverheadResult<Metric> {
    count_metric(
        MetricKind::TGateCount,
        MetricUnit::TGates,
        snapshot.physical_t_gates,
        "physical T-gate count",
    )
}

/// Creates a metric for physical measurement count.
pub fn physical_measurement_count_metric(
    snapshot: &ResourceSnapshot,
) -> ResourceOverheadResult<Metric> {
    count_metric(
        MetricKind::MeasurementCount,
        MetricUnit::Operations,
        snapshot.physical_measurements,
        "physical measurement count",
    )
}

/// Creates a metric for the number of QEC correction cycles.
pub fn correction_cycle_metric(
    snapshot: &ResourceSnapshot,
) -> ResourceOverheadResult<Metric> {
    count_metric(
        MetricKind::Custom(
            "qec_correction_cycles".to_owned(),
        ),
        MetricUnit::Layers,
        snapshot.correction_cycles,
        "QEC correction-cycle count",
    )
}

/// Creates a canonical discrete space-time-volume metric.
///
/// The value is converted to `f64` only at the final metric boundary. The
/// exact `u128` value remains available through
/// `ResourceOverhead::physical_space_time_volume`.
pub fn physical_space_time_volume_metric(
    snapshot: &ResourceSnapshot,
) -> ResourceOverheadResult<Metric> {
    let steps = snapshot
        .physical_time_steps
        .ok_or(ResourceOverheadError::MissingResource {
            resource: "physical_time_steps",
        })?;

    let volume = checked_mul_u128(
        snapshot.physical_qubits as u128,
        steps as u128,
        "physical_space_time_volume",
    )?
    .ok_or(ResourceOverheadError::MissingResource {
        resource: "physical_space_time_volume",
    })?;

    let value = u128_to_f64(
        volume,
        "physical_space_time_volume",
    )?;

    Metric::new(
        MetricKind::SpaceTimeVolume,
        MetricUnit::SpaceTimeVolume,
        value,
    )
    .map_err(ResourceOverheadError::metric_error)
    .and_then(|metric| {
        metric
            .with_description(
                "physical qubits multiplied by physical time steps",
            )
            .map_err(ResourceOverheadError::metric_error)
    })
}

/// Creates a canonical metric for the code distance.
pub fn code_distance_metric(
    snapshot: &ResourceSnapshot,
) -> ResourceOverheadResult<Metric> {
    let distance = snapshot
        .code_distance
        .ok_or(ResourceOverheadError::MissingResource {
            resource: "code_distance",
        })?;

    count_metric(
        MetricKind::Custom(
            "qec_code_distance".to_owned(),
        ),
        MetricUnit::Dimensionless,
        distance,
        "QEC code distance",
    )
}

/// Creates a canonical metric for physical execution time.
pub fn physical_execution_time_metric(
    snapshot: &ResourceSnapshot,
) -> ResourceOverheadResult<Metric> {
    duration_metric(
        MetricKind::ExecutionTime,
        snapshot.physical_execution_time_seconds,
        "physical execution time",
    )
}

// ============================================================================
// Aggregation
// ============================================================================

/// Adds two resource snapshots component-wise.
///
/// This is useful when a complete benchmark consists of multiple independent
/// circuits or experiment batches.
///
/// Identifiers are retained only when they agree. Conflicting identifiers
/// cause an explicit error instead of silently selecting one.
pub fn aggregate_snapshots(
    left: &ResourceSnapshot,
    right: &ResourceSnapshot,
) -> ResourceOverheadResult<ResourceSnapshot> {
    left.validate()?;
    right.validate()?;

    let code_id = merge_identifier(
        left.code_id.as_deref(),
        right.code_id.as_deref(),
        "code_id",
    )?;

    let decoder_id = merge_identifier(
        left.decoder_id.as_deref(),
        right.decoder_id.as_deref(),
        "decoder_id",
    )?;

    let backend_id = merge_identifier(
        left.backend_id.as_deref(),
        right.backend_id.as_deref(),
        "backend_id",
    )?;

    let physical_time_steps =
        checked_optional_add(
            left.physical_time_steps,
            right.physical_time_steps,
            "physical_time_steps",
        )?;

    let logical_time_steps =
        checked_optional_add(
            left.logical_time_steps,
            right.logical_time_steps,
            "logical_time_steps",
        )?;

    let code_distance =
        merge_numeric_metadata(
            left.code_distance,
            right.code_distance,
            "code_distance",
        )?;

    let snapshot = ResourceSnapshot {
        physical_qubits: checked_add(
            left.physical_qubits,
            right.physical_qubits,
            "physical_qubits",
        )?,
        logical_qubits: checked_add(
            left.logical_qubits,
            right.logical_qubits,
            "logical_qubits",
        )?,
        physical_gates: checked_add(
            left.physical_gates,
            right.physical_gates,
            "physical_gates",
        )?,
        logical_gates: checked_add(
            left.logical_gates,
            right.logical_gates,
            "logical_gates",
        )?,
        physical_two_qubit_gates: checked_add(
            left.physical_two_qubit_gates,
            right.physical_two_qubit_gates,
            "physical_two_qubit_gates",
        )?,
        logical_two_qubit_gates: checked_add(
            left.logical_two_qubit_gates,
            right.logical_two_qubit_gates,
            "logical_two_qubit_gates",
        )?,
        physical_depth: checked_add(
            left.physical_depth,
            right.physical_depth,
            "physical_depth",
        )?,
        logical_depth: checked_add(
            left.logical_depth,
            right.logical_depth,
            "logical_depth",
        )?,
        physical_two_qubit_depth: checked_add(
            left.physical_two_qubit_depth,
            right.physical_two_qubit_depth,
            "physical_two_qubit_depth",
        )?,
        logical_two_qubit_depth: checked_add(
            left.logical_two_qubit_depth,
            right.logical_two_qubit_depth,
            "logical_two_qubit_depth",
        )?,
        physical_t_gates: checked_add(
            left.physical_t_gates,
            right.physical_t_gates,
            "physical_t_gates",
        )?,
        logical_t_gates: checked_add(
            left.logical_t_gates,
            right.logical_t_gates,
            "logical_t_gates",
        )?,
        physical_operations: checked_add(
            left.physical_operations,
            right.physical_operations,
            "physical_operations",
        )?,
        logical_operations: checked_add(
            left.logical_operations,
            right.logical_operations,
            "logical_operations",
        )?,
        physical_measurements: checked_add(
            left.physical_measurements,
            right.physical_measurements,
            "physical_measurements",
        )?,
        logical_measurements: checked_add(
            left.logical_measurements,
            right.logical_measurements,
            "logical_measurements",
        )?,
        correction_cycles: checked_add(
            left.correction_cycles,
            right.correction_cycles,
            "correction_cycles",
        )?,
        circuits: checked_add(
            left.circuits,
            right.circuits,
            "circuits",
        )?,
        shots: checked_add(
            left.shots,
            right.shots,
            "shots",
        )?,
        physical_execution_time_seconds:
            checked_add_f64(
                left.physical_execution_time_seconds,
                right.physical_execution_time_seconds,
                "physical_execution_time_seconds",
            )?,
        logical_execution_time_seconds:
            checked_add_f64(
                left.logical_execution_time_seconds,
                right.logical_execution_time_seconds,
                "logical_execution_time_seconds",
            )?,
        physical_time_steps,
        logical_time_steps,
        code_distance,
        code_id,
        decoder_id,
        backend_id,
    };

    snapshot.validate()?;
    Ok(snapshot)
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by resource-overhead calculations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceOverheadError {
    /// A numeric value is invalid.
    InvalidValue {
        /// Field containing the invalid value.
        field: &'static str,

        /// Human-readable reason.
        reason: &'static str,
    },

    /// A floating-point value was NaN or infinite.
    NonFiniteValue {
        /// Field containing the invalid value.
        field: &'static str,
    },

    /// A resource exceeded the safety bound.
    ResourceLimitExceeded {
        /// Resource field.
        field: &'static str,

        /// Supplied value.
        value: u64,

        /// Maximum permitted value.
        maximum: u64,
    },

    /// A denominator was zero.
    ZeroDenominator {
        /// Metric being calculated.
        metric: &'static str,
    },

    /// A required resource was not supplied.
    MissingResource {
        /// Resource name.
        resource: &'static str,
    },

    /// A ratio cannot be produced because its denominator is absent.
    MissingDenominator {
        /// Metric requiring the denominator.
        metric: &'static str,
    },

    /// Integer arithmetic would overflow.
    ArithmeticOverflow {
        /// Operation being performed.
        operation: &'static str,
    },

    /// A floating-point conversion would lose the representable numeric
    /// contract required by this metric.
    NumericConversionFailure {
        /// Quantity being converted.
        field: &'static str,
    },

    /// Two resource snapshots cannot be aggregated because their metadata
    /// conflicts.
    IncompatibleMetadata {
        /// Metadata field.
        field: &'static str,
    },

    /// The canonical metric layer rejected a constructed metric.
    MetricConstruction {
        /// Error string returned by `core::metric`.
        message: String,
    },
}

impl ResourceOverheadError {
    fn metric_error<E: fmt::Display>(error: E) -> Self {
        Self::MetricConstruction {
            message: error.to_string(),
        }
    }
}

impl fmt::Display for ResourceOverheadError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidValue { field, reason } => {
                write!(
                    formatter,
                    "invalid resource-overhead value `{field}`: {reason}"
                )
            }

            Self::NonFiniteValue { field } => {
                write!(
                    formatter,
                    "resource-overhead value `{field}` is not finite"
                )
            }

            Self::ResourceLimitExceeded {
                field,
                value,
                maximum,
            } => {
                write!(
                    formatter,
                    "resource `{field}` has value {value}, \
                     exceeding maximum {maximum}"
                )
            }

            Self::ZeroDenominator { metric } => {
                write!(
                    formatter,
                    "cannot calculate `{metric}` because its denominator is zero"
                )
            }

            Self::MissingResource { resource } => {
                write!(
                    formatter,
                    "required resource `{resource}` was not supplied"
                )
            }

            Self::MissingDenominator { metric } => {
                write!(
                    formatter,
                    "required denominator for `{metric}` was not supplied"
                )
            }

            Self::ArithmeticOverflow { operation } => {
                write!(
                    formatter,
                    "integer overflow during `{operation}`"
                )
            }

            Self::NumericConversionFailure { field } => {
                write!(
                    formatter,
                    "numeric conversion failed for `{field}`"
                )
            }

            Self::IncompatibleMetadata { field } => {
                write!(
                    formatter,
                    "resource snapshots contain incompatible `{field}` metadata"
                )
            }

            Self::MetricConstruction { message } => {
                write!(
                    formatter,
                    "canonical metric construction failed: {message}"
                )
            }
        }
    }
}

impl std::error::Error for ResourceOverheadError {}

// ============================================================================
// Validation helpers
// ============================================================================

fn validate_resource_count(
    value: u64,
    field: &'static str,
) -> ResourceOverheadResult<()> {
    if value > MAX_RESOURCE_COUNT {
        return Err(
            ResourceOverheadError::ResourceLimitExceeded {
                field,
                value,
                maximum: MAX_RESOURCE_COUNT,
            },
        );
    }

    Ok(())
}

fn validate_circuit_count(
    value: u64,
) -> ResourceOverheadResult<()> {
    if value > MAX_CIRCUITS {
        return Err(
            ResourceOverheadError::ResourceLimitExceeded {
                field: "circuits",
                value,
                maximum: MAX_CIRCUITS,
            },
        );
    }

    Ok(())
}

fn validate_shot_count(
    value: u64,
) -> ResourceOverheadResult<()> {
    if value > MAX_SHOTS {
        return Err(
            ResourceOverheadError::ResourceLimitExceeded {
                field: "shots",
                value,
                maximum: MAX_SHOTS,
            },
        );
    }

    Ok(())
}

fn validate_time_steps(
    value: u64,
    field: &'static str,
) -> ResourceOverheadResult<()> {
    if value > MAX_TIME_STEPS {
        return Err(
            ResourceOverheadError::ResourceLimitExceeded {
                field,
                value,
                maximum: MAX_TIME_STEPS,
            },
        );
    }

    Ok(())
}

fn validate_code_distance(
    value: u64,
) -> ResourceOverheadResult<()> {
    if value == 0 {
        return Err(ResourceOverheadError::InvalidValue {
            field: "code_distance",
            reason: "code distance must be greater than zero",
        });
    }

    if value > MAX_CODE_DISTANCE {
        return Err(
            ResourceOverheadError::ResourceLimitExceeded {
                field: "code_distance",
                value,
                maximum: MAX_CODE_DISTANCE,
            },
        );
    }

    Ok(())
}

fn validate_non_negative_finite(
    value: f64,
    field: &'static str,
) -> ResourceOverheadResult<()> {
    if !value.is_finite() {
        return Err(ResourceOverheadError::NonFiniteValue {
            field,
        });
    }

    if value < 0.0 {
        return Err(ResourceOverheadError::InvalidValue {
            field,
            reason: "value must not be negative",
        });
    }

    Ok(())
}

fn validate_optional_identifier(
    value: Option<&str>,
    field: &'static str,
) -> ResourceOverheadResult<()> {
    if let Some(value) = value {
        if value.trim().is_empty() {
            return Err(ResourceOverheadError::InvalidValue {
                field,
                reason: "identifier must not be empty",
            });
        }

        if value.len() > MAX_RESOURCE_LABEL_BYTES {
            return Err(ResourceOverheadError::InvalidValue {
                field,
                reason: "identifier exceeds maximum length",
            });
        }
    }

    Ok(())
}

fn validate_identifier(
    value: String,
    field: &'static str,
) -> ResourceOverheadResult<String> {
    let value = value.trim().to_owned();

    validate_optional_identifier(
        Some(value.as_str()),
        field,
    )?;

    Ok(value)
}

// ============================================================================
// Arithmetic helpers
// ============================================================================

fn checked_add(
    left: u64,
    right: u64,
    field: &'static str,
) -> ResourceOverheadResult<u64> {
    left.checked_add(right).ok_or(
        ResourceOverheadError::ArithmeticOverflow {
            operation: field,
        },
    )
}

fn checked_add_f64(
    left: f64,
    right: f64,
    field: &'static str,
) -> ResourceOverheadResult<f64> {
    validate_non_negative_finite(left, field)?;
    validate_non_negative_finite(right, field)?;

    let result = left + right;

    if !result.is_finite() {
        return Err(ResourceOverheadError::NonFiniteValue {
            field,
        });
    }

    Ok(result)
}

fn checked_mul_u128(
    left: u128,
    right: u128,
    operation: &'static str,
) -> ResourceOverheadResult<Option<u128>> {
    left.checked_mul(right)
        .map(Some)
        .ok_or(
            ResourceOverheadError::ArithmeticOverflow {
                operation,
            },
        )
}

fn ratio_if_nonzero(
    numerator: u64,
    denominator: u64,
    metric: &'static str,
) -> ResourceOverheadResult<Option<f64>> {
    if denominator == 0 {
        return Ok(None);
    }

    let value = numerator as f64 / denominator as f64;

    if !value.is_finite() {
        return Err(ResourceOverheadError::NonFiniteValue {
            field: metric,
        });
    }

    if value < 0.0 {
        return Err(ResourceOverheadError::InvalidValue {
            field: metric,
            reason: "ratio must not be negative",
        });
    }

    Ok(Some(value))
}

fn ratio_u128_if_nonzero(
    numerator: u128,
    denominator: u128,
    metric: &'static str,
) -> ResourceOverheadResult<Option<f64>> {
    if denominator == 0 {
        return Ok(None);
    }

    let numerator = u128_to_f64(
        numerator,
        metric,
    )?;

    let denominator = u128_to_f64(
        denominator,
        metric,
    )?;

    let value = numerator / denominator;

    if !value.is_finite() {
        return Err(ResourceOverheadError::NonFiniteValue {
            field: metric,
        });
    }

    Ok(Some(value))
}

fn positive_f64_ratio_if_nonzero(
    numerator: f64,
    denominator: f64,
    metric: &'static str,
) -> ResourceOverheadResult<Option<f64>> {
    validate_non_negative_finite(numerator, metric)?;
    validate_non_negative_finite(denominator, metric)?;

    if denominator == 0.0 {
        return Ok(None);
    }

    let value = numerator / denominator;

    if !value.is_finite() {
        return Err(ResourceOverheadError::NonFiniteValue {
            field: metric,
        });
    }

    Ok(Some(value))
}

fn u128_to_f64(
    value: u128,
    field: &'static str,
) -> ResourceOverheadResult<f64> {
    let converted = value as f64;

    if !converted.is_finite() {
        return Err(
            ResourceOverheadError::NumericConversionFailure {
                field,
            },
        );
    }

    Ok(converted)
}

// ============================================================================
// Metric helpers
// ============================================================================

fn count_metric(
    kind: MetricKind,
    unit: MetricUnit,
    value: u64,
    description: &'static str,
) -> ResourceOverheadResult<Metric> {
    let value = value as f64;

    if !value.is_finite() {
        return Err(ResourceOverheadError::NonFiniteValue {
            field: description,
        });
    }

    Metric::new(kind, unit, value)
        .map_err(ResourceOverheadError::metric_error)
        .and_then(|metric| {
            metric
                .with_description(description)
                .map_err(ResourceOverheadError::metric_error)
        })
}

fn duration_metric(
    kind: MetricKind,
    seconds: f64,
    description: &'static str,
) -> ResourceOverheadResult<Metric> {
    validate_non_negative_finite(
        seconds,
        "execution_time_seconds",
    )?;

    Metric::new(
        kind,
        MetricUnit::Seconds,
        seconds,
    )
    .map_err(ResourceOverheadError::metric_error)
    .and_then(|metric| {
        metric
            .with_description(description)
            .map_err(ResourceOverheadError::metric_error)
    })
}

// ============================================================================
// Metadata helpers
// ============================================================================

fn merge_identifier(
    left: Option<&str>,
    right: Option<&str>,
    field: &'static str,
) -> ResourceOverheadResult<Option<String>> {
    match (left, right) {
        (None, None) => Ok(None),

        (Some(value), None) => Ok(Some(value.to_owned())),

        (None, Some(value)) => Ok(Some(value.to_owned())),

        (Some(left), Some(right)) if left == right => {
            Ok(Some(left.to_owned()))
        }

        _ => Err(
            ResourceOverheadError::IncompatibleMetadata {
                field,
            },
        ),
    }
}

fn merge_numeric_metadata(
    left: Option<u64>,
    right: Option<u64>,
    field: &'static str,
) -> ResourceOverheadResult<Option<u64>> {
    match (left, right) {
        (None, None) => Ok(None),

        (Some(value), None) => Ok(Some(value)),

        (None, Some(value)) => Ok(Some(value)),

        (Some(left), Some(right)) if left == right => {
            Ok(Some(left))
        }

        _ => Err(
            ResourceOverheadError::IncompatibleMetadata {
                field,
            },
        ),
    }
}

fn checked_optional_add(
    left: Option<u64>,
    right: Option<u64>,
    field: &'static str,
) -> ResourceOverheadResult<Option<u64>> {
    match (left, right) {
        (None, None) => Ok(None),

        (Some(value), None) => Ok(Some(value)),

        (None, Some(value)) => Ok(Some(value)),

        (Some(left), Some(right)) => {
            Ok(Some(checked_add(left, right, field)?))
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_snapshot() -> ResourceSnapshot {
        ResourceSnapshot::new(
            100,
            4,
            10_000,
            100,
            4_000,
            50,
            1_000,
            100,
            500,
            50,
            2_000,
            20,
            12_000,
            150,
            1_000,
            10,
            100,
            10,
            10_000,
            2.0,
            1.0,
        )
        .expect("test snapshot must be valid")
    }

    #[test]
    fn validates_normal_snapshot() {
        let snapshot = test_snapshot();

        assert_eq!(snapshot.physical_qubits, 100);
        assert_eq!(snapshot.logical_qubits, 4);
    }

    #[test]
    fn calculates_qubit_overhead() {
        let overhead =
            ResourceOverhead::between(test_snapshot())
                .expect("overhead calculation must succeed");

        assert_eq!(overhead.qubit_overhead, Some(25.0));
    }

    #[test]
    fn calculates_gate_overhead() {
        let overhead =
            ResourceOverhead::between(test_snapshot())
                .expect("overhead calculation must succeed");

        assert_eq!(overhead.gate_overhead, Some(100.0));
    }

    #[test]
    fn calculates_two_qubit_gate_overhead() {
        let overhead =
            ResourceOverhead::between(test_snapshot())
                .expect("overhead calculation must succeed");

        assert_eq!(
            overhead.two_qubit_gate_overhead,
            Some(80.0)
        );
    }

    #[test]
    fn calculates_depth_overhead() {
        let overhead =
            ResourceOverhead::between(test_snapshot())
                .expect("overhead calculation must succeed");

        assert_eq!(overhead.depth_overhead, Some(10.0));
    }

    #[test]
    fn calculates_time_overhead() {
        let overhead =
            ResourceOverhead::between(test_snapshot())
                .expect("overhead calculation must succeed");

        assert_eq!(
            overhead.execution_time_overhead,
            Some(2.0)
        );
    }

    #[test]
    fn zero_denominator_is_not_fabricated() {
        let mut snapshot = test_snapshot();
        snapshot.logical_gates = 0;

        let overhead =
            ResourceOverhead::between(snapshot)
                .expect("zero denominator should not invalidate snapshot");

        assert_eq!(overhead.gate_overhead, None);
    }

    #[test]
    fn zero_logical_qubits_are_not_an_error_until_ratio_is_requested() {
        let mut snapshot = test_snapshot();
        snapshot.logical_qubits = 0;

        let overhead =
            ResourceOverhead::between(snapshot)
                .expect("zero denominator should be represented as unavailable");

        assert_eq!(overhead.qubit_overhead, None);
    }

    #[test]
    fn discrete_space_time_volume_is_exact() {
        let snapshot = test_snapshot()
            .with_time_steps(20, 10)
            .expect("time steps must be valid");

        let overhead =
            ResourceOverhead::between(snapshot)
                .expect("overhead calculation must succeed");

        assert_eq!(
            overhead.physical_space_time_volume,
            Some(2_000)
        );

        assert_eq!(
            overhead.logical_space_time_volume,
            Some(40)
        );

        assert_eq!(
            overhead.space_time_overhead,
            Some(50.0)
        );
    }

    #[test]
    fn physical_space_time_metric_uses_canonical_metric_kind() {
        let snapshot = test_snapshot()
            .with_time_steps(20, 10)
            .expect("time steps must be valid");

        let metric =
            physical_space_time_volume_metric(&snapshot)
                .expect("metric construction must succeed");

        assert_eq!(
            metric.kind,
            MetricKind::SpaceTimeVolume
        );

        assert_eq!(
            metric.unit,
            MetricUnit::SpaceTimeVolume
        );

        assert_eq!(metric.value.get(), 2_000.0);
    }

    #[test]
    fn resource_overhead_metric_uses_canonical_metric_kind() {
        let overhead =
            ResourceOverhead::between(test_snapshot())
                .expect("overhead calculation must succeed");

        let metric = overhead
            .qubit_overhead_metric()
            .expect("metric construction must succeed");

        assert_eq!(
            metric.kind,
            MetricKind::ResourceOverhead
        );

        assert_eq!(
            metric.unit,
            MetricUnit::Dimensionless
        );

        assert_eq!(metric.value.get(), 25.0);
    }

    #[test]
    fn metrics_are_deterministically_ordered() {
        let overhead =
            ResourceOverhead::between(
                test_snapshot()
                    .with_time_steps(20, 10)
                    .expect("time steps must be valid"),
            )
            .expect("overhead calculation must succeed");

        let metrics =
            overhead.metrics().expect("metrics must build");

        let ids: Vec<String> =
            metrics.iter().map(|metric| metric.kind_id()).collect();

        assert_eq!(
            ids,
            vec![
                "resource_overhead",
                "resource_overhead",
                "resource_overhead",
                "resource_overhead",
                "resource_overhead",
                "resource_overhead",
                "resource_overhead",
                "resource_overhead",
                "resource_overhead",
                "resource_overhead",
                "resource_overhead",
            ]
        );
    }

    #[test]
    fn rejects_nan_execution_time() {
        let result = ResourceSnapshot::new(
            10,
            1,
            10,
            1,
            2,
            1,
            10,
            1,
            2,
            1,
            1,
            1,
            10,
            1,
            1,
            1,
            1,
            1,
            1,
            f64::NAN,
            1.0,
        );

        assert!(matches!(
            result,
            Err(ResourceOverheadError::NonFiniteValue {
                field: "physical_execution_time_seconds"
            })
        ));
    }

    #[test]
    fn rejects_negative_execution_time() {
        let result = ResourceSnapshot::new(
            10,
            1,
            10,
            1,
            2,
            1,
            10,
            1,
            2,
            1,
            1,
            1,
            10,
            1,
            1,
            1,
            1,
            1,
            1,
            -1.0,
            1.0,
        );

        assert!(matches!(
            result,
            Err(ResourceOverheadError::InvalidValue {
                field: "physical_execution_time_seconds",
                ..
            })
        ));
    }

    #[test]
    fn rejects_excessive_resource_count() {
        let mut snapshot = test_snapshot();
        snapshot.physical_qubits =
            MAX_RESOURCE_COUNT + 1;

        assert!(matches!(
            snapshot.validate(),
            Err(
                ResourceOverheadError::ResourceLimitExceeded {
                    field: "physical_qubits",
                    ..
                }
            )
        ));
    }

    #[test]
    fn rejects_invalid_code_distance() {
        let result = test_snapshot()
            .with_code_distance(0);

        assert!(matches!(
            result,
            Err(ResourceOverheadError::InvalidValue {
                field: "code_distance",
                ..
            })
        ));
    }

    #[test]
    fn rejects_long_identifier() {
        let identifier =
            "x".repeat(MAX_RESOURCE_LABEL_BYTES + 1);

        let result = test_snapshot()
            .with_code_id(identifier);

        assert!(matches!(
            result,
            Err(ResourceOverheadError::InvalidValue {
                field: "code_id",
                ..
            })
        ));
    }

    #[test]
    fn aggregates_compatible_snapshots() {
        let left = test_snapshot();
        let right = test_snapshot();

        let combined =
            aggregate_snapshots(&left, &right)
                .expect("compatible snapshots must aggregate");

        assert_eq!(
            combined.physical_qubits,
            left.physical_qubits + right.physical_qubits
        );

        assert_eq!(
            combined.physical_gates,
            left.physical_gates + right.physical_gates
        );
    }

    #[test]
    fn rejects_conflicting_metadata() {
        let left = test_snapshot()
            .with_code_id("surface_code")
            .expect("identifier must be valid");

        let right = test_snapshot()
            .with_code_id("color_code")
            .expect("identifier must be valid");

        assert!(matches!(
            aggregate_snapshots(&left, &right),
            Err(
                ResourceOverheadError::IncompatibleMetadata {
                    field: "code_id"
                }
            )
        ));
    }

    #[test]
    fn code_distance_metric_is_available() {
        let snapshot = test_snapshot()
            .with_code_distance(7)
            .expect("distance must be valid");

        let metric =
            code_distance_metric(&snapshot)
                .expect("metric must build");

        assert_eq!(metric.value.get(), 7.0);
    }

    #[test]
    fn physical_qubit_metric_is_available() {
        let snapshot = test_snapshot();

        let metric =
            physical_qubit_count_metric(&snapshot)
                .expect("metric must build");

        assert_eq!(
            metric.kind,
            MetricKind::PhysicalQubitCount
        );

        assert_eq!(
            metric.unit,
            MetricUnit::PhysicalQubits
        );

        assert_eq!(metric.value.get(), 100.0);
    }

    #[test]
    fn execution_time_metric_is_available() {
        let snapshot = test_snapshot();

        let metric =
            physical_execution_time_metric(&snapshot)
                .expect("metric must build");

        assert_eq!(
            metric.kind,
            MetricKind::ExecutionTime
        );

        assert_eq!(
            metric.unit,
            MetricUnit::Seconds
        );

        assert_eq!(metric.value.get(), 2.0);
    }
}