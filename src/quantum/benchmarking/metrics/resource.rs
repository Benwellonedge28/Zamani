//! Zamani Quantum Benchmarking — Resource Metrics
//!
//! Production resource accounting and metric construction for the Zamani
//! quantum benchmarking framework.
//!
//! # Purpose
//!
//! This module converts validated resource observations into the canonical
//! benchmarking [`Metric`] representation.
//!
//! It deliberately does NOT:
//!
//! - execute circuits;
//! - generate circuits;
//! - inspect hardware;
//! - inspect Quantum IR directly;
//! - perform statistical fitting;
//! - perform protocol-specific analysis;
//! - perform reporting;
//! - mutate benchmark state;
//! - allocate memory proportional to benchmark size;
//! - maintain process-global state.
//!
//! The dependency direction is:
//!
//! ```text
//! quantum::ir / algorithms / hardware / execution
//!                         │
//!                         ▼
//!                resource observations
//!                         │
//!                         ▼
//!        benchmarking::metrics::resource
//!                         │
//!                         ▼
//!                core::metric::Metric
//!                         │
//!                         ▼
//!          core::result / reporting / analysis
//! ```
//!
//! # Architectural boundary
//!
//! Resource accounting is intentionally separate from:
//!
//! - `core::dimension` — describes benchmark dimensions/axes;
//! - `metrics::runtime` — describes elapsed timing;
//! - `metrics::throughput` — derives rates;
//! - `metrics::fidelity` — describes quality;
//! - `metrics::gate_error` — describes gate error;
//! - protocol implementations — define why a resource was measured.
//!
//! This module answers a narrower question:
//!
//! > "How many quantum/classical resources did this workload consume or
//! > require?"
//!
//! # Supported resources
//!
//! The canonical resource snapshot supports:
//!
//! - total qubits;
//! - logical qubits;
//! - physical qubits;
//! - circuit depth;
//! - two-qubit depth;
//! - total gates;
//! - two-qubit gates;
//! - measurements;
//! - T gates;
//! - classical operations;
//! - shots;
//! - circuits;
//! - memory;
//! - energy.
//!
//! Optional resources remain absent rather than being fabricated.
//!
//! # Production properties
//!
//! This implementation provides:
//!
//! - checked arithmetic;
//! - overflow detection;
//! - explicit optional-resource semantics;
//! - deterministic aggregation;
//! - immutable resource snapshots;
//! - resource-delta calculation;
//! - canonical [`Metric`] conversion;
//! - machine-readable resource identifiers;
//! - serde serialization;
//! - bounded metadata;
//! - no floating-point use for integer resource counts;
//! - no hidden conversions;
//! - no logging side effects;
//! - no panics in normal library operation;
//! - Rust 1.97 / 1.97.1 compatibility;
//! - Rust 2021 compatibility.
//!
//! # Integration contract
//!
//! This file is designed to be completed before the protocol implementations.
//! Later modules should consume this API without modifying this file.
//!
//! Protocols such as:
//!
//! - Quantum Volume;
//! - randomized benchmarking;
//! - XEB;
//! - mirror circuits;
//! - application benchmarks;
//! - QAOA;
//! - VQE;
//! - QEC;
//! - volumetric benchmarking;
//!
//! can construct a [`ResourceSnapshot`] and call [`ResourceMetrics::from_snapshot`].
//!
//! Execution backends should populate the snapshot from their normalized
//! execution information. They must not introduce backend-specific resource
//! types here.
//!
//! # Important semantic rule
//!
//! Zero and unavailable are different:
//!
//! - `Some(0)` means the measured/required resource count is genuinely zero.
//! - `None` means the resource was not applicable, not measured, or not
//!   available from the backend.
//!
//! This distinction is essential for analog, photonic, annealing, and other
//! non-gate-model quantum systems.
//!
//! # Canonical metric integration
//!
//! The module uses the existing `core::metric` contract. That contract
//! requires finite values, explicit units and semantic validation rather than
//! returning unexplained primitive values.
//!
//! See:
//!
//! `crate::quantum::benchmarking::core::metric`
//!
//! for the authoritative metric model.
//!
//! # No protocol ownership
//!
//! Resource metrics must never contain logic such as:
//!
//! ```text
//! if quantum_volume { ... }
//! if rb { ... }
//! if xeb { ... }
//! ```
//!
//! Protocols decide which resources matter. This module only represents and
//! computes them.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust only
//!
//! No nightly features are required.
//!
//! # Security/resource-safety note
//!
//! All aggregate arithmetic is checked. A malformed or hostile benchmark
//! result cannot silently wrap `u64` resource counts and produce a false
//! benchmark result.
//!
//! This is particularly important when benchmark results originate from:
//!
//! - external hardware providers;
//! - serialized result files;
//! - distributed execution;
//! - user-defined Zamani benchmarks;
//! - QEC sweeps;
//! - volumetric sweeps.
//!
//! The module does not allocate based on any resource count.

use serde::{Deserialize, Serialize};

use crate::quantum::benchmarking::core::metric::{
    Metric,
    MetricKind,
    MetricResult,
    MetricUnit,
};

// =============================================================================
// Public constants
// =============================================================================

/// Stable schema version for resource snapshots.
///
/// Increment this when the serialized resource schema changes incompatibly.
pub const RESOURCE_SCHEMA_VERSION: u32 = 1;

/// Stable identifier for the resource metric family.
pub const RESOURCE_METRIC_FAMILY_ID: &str = "quantum_resource";

/// Maximum number of custom resource entries permitted in one snapshot.
///
/// Resource names are extensible, but the collection is bounded so malformed
/// external input cannot request unbounded metadata allocation.
pub const MAX_CUSTOM_RESOURCES: usize = 128;

/// Maximum UTF-8 byte length of a custom resource identifier.
pub const MAX_CUSTOM_RESOURCE_ID_BYTES: usize = 128;

/// Maximum UTF-8 byte length of a custom resource unit.
pub const MAX_CUSTOM_RESOURCE_UNIT_BYTES: usize = 64;

// =============================================================================
// Resource identifiers
// =============================================================================

/// Stable machine-readable identifier for a resource quantity.
///
/// Built-in identifiers correspond to the universal Zamani quantum
/// benchmarking resource vocabulary. `Custom` provides forward-compatible
/// extension without requiring this module to be modified for every new
/// quantum technology.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceKind {
    /// Total qubits represented by the workload.
    Qubits,

    /// Logical qubits used by a logical/fault-tolerant workload.
    LogicalQubits,

    /// Physical qubits allocated or consumed.
    PhysicalQubits,

    /// Total circuit depth.
    CircuitDepth,

    /// Depth contributed by two-qubit operations.
    TwoQubitDepth,

    /// Total quantum gate count.
    GateCount,

    /// Number of two-qubit gates.
    TwoQubitGateCount,

    /// Number of measurements.
    MeasurementCount,

    /// Number of T gates.
    TGateCount,

    /// Number of classical operations.
    ClassicalOperationCount,

    /// Number of shots/samples.
    ShotCount,

    /// Number of circuits/instances.
    CircuitCount,

    /// Memory consumption.
    MemoryBytes,

    /// Energy consumption.
    EnergyJoules,

    /// User-defined resource quantity.
    Custom(String),
}

impl ResourceKind {
    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub fn id(&self) -> String {
        match self {
            Self::Qubits => "qubits".to_owned(),
            Self::LogicalQubits => "logical_qubits".to_owned(),
            Self::PhysicalQubits => "physical_qubits".to_owned(),
            Self::CircuitDepth => "circuit_depth".to_owned(),
            Self::TwoQubitDepth => "two_qubit_depth".to_owned(),
            Self::GateCount => "gate_count".to_owned(),
            Self::TwoQubitGateCount => "two_qubit_gate_count".to_owned(),
            Self::MeasurementCount => "measurement_count".to_owned(),
            Self::TGateCount => "t_gate_count".to_owned(),
            Self::ClassicalOperationCount => {
                "classical_operation_count".to_owned()
            }
            Self::ShotCount => "shot_count".to_owned(),
            Self::CircuitCount => "circuit_count".to_owned(),
            Self::MemoryBytes => "memory_bytes".to_owned(),
            Self::EnergyJoules => "energy_joules".to_owned(),
            Self::Custom(value) => value.clone(),
        }
    }

    /// Returns whether this resource is a user-defined extension.
    #[must_use]
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }

    /// Creates a validated custom resource identifier.
    pub fn custom<S: Into<String>>(id: S) -> Result<Self, ResourceError> {
        let id = id.into();
        validate_identifier(
            &id,
            MAX_CUSTOM_RESOURCE_ID_BYTES,
            "resource identifier",
        )?;

        Ok(Self::Custom(id))
    }

    /// Returns the canonical built-in resource kinds.
    #[must_use]
    pub fn builtins() -> &'static [Self] {
        static BUILTINS: [ResourceKind; 14] = [
            ResourceKind::Qubits,
            ResourceKind::LogicalQubits,
            ResourceKind::PhysicalQubits,
            ResourceKind::CircuitDepth,
            ResourceKind::TwoQubitDepth,
            ResourceKind::GateCount,
            ResourceKind::TwoQubitGateCount,
            ResourceKind::MeasurementCount,
            ResourceKind::TGateCount,
            ResourceKind::ClassicalOperationCount,
            ResourceKind::ShotCount,
            ResourceKind::CircuitCount,
            ResourceKind::MemoryBytes,
            ResourceKind::EnergyJoules,
        ];

        &BUILTINS
    }
}

// =============================================================================
// Resource values
// =============================================================================

/// A validated non-negative resource quantity.
///
/// Resource counts are represented using `u64` rather than floating-point
/// values. This prevents precision loss for large workloads and makes
/// overflow handling explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResourceCount(u64);

impl ResourceCount {
    /// Creates a resource count.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Checked addition.
    pub fn checked_add(
        self,
        other: Self,
    ) -> Result<Self, ResourceError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(ResourceError::ArithmeticOverflow)
    }

    /// Checked subtraction.
    pub fn checked_sub(
        self,
        other: Self,
    ) -> Result<Self, ResourceError> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or(ResourceError::NegativeResult)
    }

    /// Returns whether the count is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl From<u64> for ResourceCount {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<ResourceCount> for u64 {
    fn from(value: ResourceCount) -> Self {
        value.get()
    }
}

// =============================================================================
// Resource observations
// =============================================================================

/// Canonical resource snapshot.
///
/// Each field is optional because not every quantum technology exposes or
/// even possesses every resource concept.
///
/// For example:
///
/// - a gate-model circuit may have gate count and depth;
/// - an annealing workload may not have gate count;
/// - a photonic workload may use modes rather than qubits;
/// - a simulator may report memory but not physical qubits.
///
/// `None` must never be interpreted as zero.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceSnapshot {
    /// Snapshot schema version.
    pub schema_version: u32,

    /// Logical/abstract qubit count when applicable.
    pub qubits: Option<ResourceCount>,

    /// Logical qubit count when applicable.
    pub logical_qubits: Option<ResourceCount>,

    /// Physical qubit count when applicable.
    pub physical_qubits: Option<ResourceCount>,

    /// Total circuit depth when applicable.
    pub circuit_depth: Option<ResourceCount>,

    /// Two-qubit-operation depth when applicable.
    pub two_qubit_depth: Option<ResourceCount>,

    /// Total quantum gate count.
    pub gate_count: Option<ResourceCount>,

    /// Two-qubit gate count.
    pub two_qubit_gate_count: Option<ResourceCount>,

    /// Measurement count.
    pub measurement_count: Option<ResourceCount>,

    /// T-gate count.
    pub t_gate_count: Option<ResourceCount>,

    /// Classical operation count.
    pub classical_operation_count: Option<ResourceCount>,

    /// Number of shots/samples.
    pub shot_count: Option<ResourceCount>,

    /// Number of distinct circuits/instances.
    pub circuit_count: Option<ResourceCount>,

    /// Peak or otherwise explicitly defined memory consumption in bytes.
    pub memory_bytes: Option<ResourceCount>,

    /// Energy consumption in joules.
    ///
    /// Energy is optional because most backends do not expose trustworthy
    /// energy measurements.
    pub energy_joules: Option<FiniteResourceValue>,

    /// Extensible domain-specific resource values.
    pub custom: Vec<CustomResource>,
}

impl ResourceSnapshot {
    /// Creates an empty snapshot with the current schema version.
    #[must_use]
    pub fn new() -> Self {
        Self {
            schema_version: RESOURCE_SCHEMA_VERSION,
            ..Self::default()
        }
    }

    /// Validates the snapshot.
    pub fn validate(&self) -> Result<(), ResourceError> {
        if self.schema_version == 0 {
            return Err(ResourceError::InvalidSchemaVersion {
                version: self.schema_version,
            });
        }

        if self.custom.len() > MAX_CUSTOM_RESOURCES {
            return Err(ResourceError::TooManyCustomResources {
                count: self.custom.len(),
                maximum: MAX_CUSTOM_RESOURCES,
            });
        }

        for resource in &self.custom {
            resource.validate()?;
        }

        Ok(())
    }

    /// Returns true when no resource value has been supplied.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.qubits.is_none()
            && self.logical_qubits.is_none()
            && self.physical_qubits.is_none()
            && self.circuit_depth.is_none()
            && self.two_qubit_depth.is_none()
            && self.gate_count.is_none()
            && self.two_qubit_gate_count.is_none()
            && self.measurement_count.is_none()
            && self.t_gate_count.is_none()
            && self.classical_operation_count.is_none()
            && self.shot_count.is_none()
            && self.circuit_count.is_none()
            && self.memory_bytes.is_none()
            && self.energy_joules.is_none()
            && self.custom.is_empty()
    }

    /// Returns the number of populated resource fields.
    #[must_use]
    pub fn populated_count(&self) -> usize {
        let mut count = self.custom.len();

        if self.qubits.is_some() {
            count += 1;
        }
        if self.logical_qubits.is_some() {
            count += 1;
        }
        if self.physical_qubits.is_some() {
            count += 1;
        }
        if self.circuit_depth.is_some() {
            count += 1;
        }
        if self.two_qubit_depth.is_some() {
            count += 1;
        }
        if self.gate_count.is_some() {
            count += 1;
        }
        if self.two_qubit_gate_count.is_some() {
            count += 1;
        }
        if self.measurement_count.is_some() {
            count += 1;
        }
        if self.t_gate_count.is_some() {
            count += 1;
        }
        if self.classical_operation_count.is_some() {
            count += 1;
        }
        if self.shot_count.is_some() {
            count += 1;
        }
        if self.circuit_count.is_some() {
            count += 1;
        }
        if self.memory_bytes.is_some() {
            count += 1;
        }
        if self.energy_joules.is_some() {
            count += 1;
        }

        count
    }

    /// Returns the resource count associated with a built-in resource kind.
    #[must_use]
    pub fn get_count(&self, kind: &ResourceKind) -> Option<ResourceCount> {
        match kind {
            ResourceKind::Qubits => self.qubits,
            ResourceKind::LogicalQubits => self.logical_qubits,
            ResourceKind::PhysicalQubits => self.physical_qubits,
            ResourceKind::CircuitDepth => self.circuit_depth,
            ResourceKind::TwoQubitDepth => self.two_qubit_depth,
            ResourceKind::GateCount => self.gate_count,
            ResourceKind::TwoQubitGateCount => self.two_qubit_gate_count,
            ResourceKind::MeasurementCount => self.measurement_count,
            ResourceKind::TGateCount => self.t_gate_count,
            ResourceKind::ClassicalOperationCount => {
                self.classical_operation_count
            }
            ResourceKind::ShotCount => self.shot_count,
            ResourceKind::CircuitCount => self.circuit_count,
            ResourceKind::MemoryBytes => self.memory_bytes,
            ResourceKind::EnergyJoules | ResourceKind::Custom(_) => None,
        }
    }

    /// Returns the energy value.
    #[must_use]
    pub fn energy_joules(&self) -> Option<f64> {
        self.energy_joules.map(FiniteResourceValue::get)
    }

    /// Adds two snapshots using checked arithmetic.
    ///
    /// `None + Some(x)` becomes `Some(x)`.
    ///
    /// `None + None` remains `None`.
    ///
    /// This operation is intended for aggregating independent benchmark
    /// executions.
    pub fn checked_add(
        &self,
        other: &Self,
    ) -> Result<Self, ResourceError> {
        self.validate()?;
        other.validate()?;

        let mut result = Self::new();

        result.qubits = checked_add_optional(
            self.qubits,
            other.qubits,
        )?;

        result.logical_qubits = checked_add_optional(
            self.logical_qubits,
            other.logical_qubits,
        )?;

        result.physical_qubits = checked_add_optional(
            self.physical_qubits,
            other.physical_qubits,
        )?;

        result.circuit_depth = checked_add_optional(
            self.circuit_depth,
            other.circuit_depth,
        )?;

        result.two_qubit_depth = checked_add_optional(
            self.two_qubit_depth,
            other.two_qubit_depth,
        )?;

        result.gate_count = checked_add_optional(
            self.gate_count,
            other.gate_count,
        )?;

        result.two_qubit_gate_count = checked_add_optional(
            self.two_qubit_gate_count,
            other.two_qubit_gate_count,
        )?;

        result.measurement_count = checked_add_optional(
            self.measurement_count,
            other.measurement_count,
        )?;

        result.t_gate_count = checked_add_optional(
            self.t_gate_count,
            other.t_gate_count,
        )?;

        result.classical_operation_count = checked_add_optional(
            self.classical_operation_count,
            other.classical_operation_count,
        )?;

        result.shot_count = checked_add_optional(
            self.shot_count,
            other.shot_count,
        )?;

        result.circuit_count = checked_add_optional(
            self.circuit_count,
            other.circuit_count,
        )?;

        result.memory_bytes = checked_add_optional(
            self.memory_bytes,
            other.memory_bytes,
        )?;

        result.energy_joules = checked_add_optional_float(
            self.energy_joules,
            other.energy_joules,
        )?;

        result.custom = merge_custom_resources(
            &self.custom,
            &other.custom,
        )?;

        Ok(result)
    }

    /// Calculates a non-negative resource delta.
    ///
    /// This is useful when an execution environment provides cumulative
    /// counters and the benchmark needs the resource consumption between two
    /// observations.
    ///
    /// If a resource is present in only one snapshot, the delta is unavailable
    /// rather than being fabricated.
    pub fn checked_delta(
        &self,
        previous: &Self,
    ) -> Result<Self, ResourceError> {
        self.validate()?;
        previous.validate()?;

        let mut result = Self::new();

        result.qubits =
            checked_delta_optional(self.qubits, previous.qubits)?;

        result.logical_qubits = checked_delta_optional(
            self.logical_qubits,
            previous.logical_qubits,
        )?;

        result.physical_qubits = checked_delta_optional(
            self.physical_qubits,
            previous.physical_qubits,
        )?;

        result.circuit_depth = checked_delta_optional(
            self.circuit_depth,
            previous.circuit_depth,
        )?;

        result.two_qubit_depth = checked_delta_optional(
            self.two_qubit_depth,
            previous.two_qubit_depth,
        )?;

        result.gate_count = checked_delta_optional(
            self.gate_count,
            previous.gate_count,
        )?;

        result.two_qubit_gate_count = checked_delta_optional(
            self.two_qubit_gate_count,
            previous.two_qubit_gate_count,
        )?;

        result.measurement_count = checked_delta_optional(
            self.measurement_count,
            previous.measurement_count,
        )?;

        result.t_gate_count = checked_delta_optional(
            self.t_gate_count,
            previous.t_gate_count,
        )?;

        result.classical_operation_count =
            checked_delta_optional(
                self.classical_operation_count,
                previous.classical_operation_count,
            )?;

        result.shot_count = checked_delta_optional(
            self.shot_count,
            previous.shot_count,
        )?;

        result.circuit_count = checked_delta_optional(
            self.circuit_count,
            previous.circuit_count,
        )?;

        result.memory_bytes = checked_delta_optional(
            self.memory_bytes,
            previous.memory_bytes,
        )?;

        result.energy_joules =
            checked_delta_optional_float(
                self.energy_joules,
                previous.energy_joules,
            )?;

        result.custom = delta_custom_resources(
            &self.custom,
            &previous.custom,
        )?;

        Ok(result)
    }
}

// =============================================================================
// Finite resource values
// =============================================================================

/// A finite, non-negative floating-point resource quantity.
///
/// This is currently used for energy because joules may be fractional.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FiniteResourceValue(f64);

impl FiniteResourceValue {
    /// Creates a validated non-negative finite resource value.
    pub fn new(value: f64) -> Result<Self, ResourceError> {
        if !value.is_finite() {
            return Err(ResourceError::NonFiniteValue { value });
        }

        if value < 0.0 {
            return Err(ResourceError::NegativeValue { value });
        }

        Ok(Self(value))
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    /// Checked addition.
    pub fn checked_add(
        self,
        other: Self,
    ) -> Result<Self, ResourceError> {
        let value = self.0 + other.0;

        if !value.is_finite() {
            return Err(ResourceError::ArithmeticOverflow);
        }

        Self::new(value)
    }

    /// Checked subtraction.
    pub fn checked_sub(
        self,
        other: Self,
    ) -> Result<Self, ResourceError> {
        let value = self.0 - other.0;

        Self::new(value)
    }
}

// =============================================================================
// Custom resources
// =============================================================================

/// User-defined resource quantity.
///
/// Custom resources allow future quantum technologies to expose resources
/// without changing the stable built-in vocabulary.
///
/// Example:
///
/// ```text
/// ResourceKind::Custom("optical_modes")
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomResource {
    /// Stable resource identifier.
    pub kind: String,

    /// Numeric value.
    pub value: FiniteResourceValue,

    /// Unit identifier.
    pub unit: String,

    /// Optional human-readable description.
    pub description: Option<String>,
}

impl CustomResource {
    /// Creates a validated custom resource.
    pub fn new(
        kind: impl Into<String>,
        value: f64,
        unit: impl Into<String>,
    ) -> Result<Self, ResourceError> {
        let kind = kind.into();
        let unit = unit.into();

        validate_identifier(
            &kind,
            MAX_CUSTOM_RESOURCE_ID_BYTES,
            "custom resource identifier",
        )?;

        validate_unit(&unit)?;

        Ok(Self {
            kind,
            value: FiniteResourceValue::new(value)?,
            unit,
            description: None,
        })
    }

    /// Adds an optional description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, ResourceError> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(ResourceError::EmptyIdentifier {
                field: "custom resource description",
            });
        }

        self.description = Some(description);

        Ok(self)
    }

    /// Validates the resource.
    pub fn validate(&self) -> Result<(), ResourceError> {
        validate_identifier(
            &self.kind,
            MAX_CUSTOM_RESOURCE_ID_BYTES,
            "custom resource identifier",
        )?;

        validate_unit(&self.unit)?;

        FiniteResourceValue::new(self.value.get())?;

        Ok(())
    }
}

// =============================================================================
// Resource metric construction
// =============================================================================

/// Converts resource snapshots into canonical Zamani metrics.
///
/// This type is stateless. It exists to provide a stable API boundary and to
/// keep metric construction separate from raw resource observation.
#[derive(Debug, Clone, Copy, Default)]
pub struct ResourceMetrics;

impl ResourceMetrics {
    /// Creates the resource metric builder.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Converts every populated built-in resource into a canonical metric.
    ///
    /// The returned vector follows the fixed order of the resource vocabulary,
    /// guaranteeing deterministic output.
    pub fn from_snapshot(
        &self,
        snapshot: &ResourceSnapshot,
    ) -> Result<Vec<Metric>, ResourceMetricError> {
        snapshot.validate()?;

        let mut metrics = Vec::with_capacity(
            snapshot.populated_count(),
        );

        self.push_count_metric(
            &mut metrics,
            snapshot.qubits,
            MetricKind::QubitCount,
            MetricUnit::Qubits,
            "Total qubit resource requirement.",
        )?;

        self.push_count_metric(
            &mut metrics,
            snapshot.logical_qubits,
            MetricKind::LogicalQubitCount,
            MetricUnit::LogicalQubits,
            "Logical qubit resource requirement.",
        )?;

        self.push_count_metric(
            &mut metrics,
            snapshot.physical_qubits,
            MetricKind::PhysicalQubitCount,
            MetricUnit::PhysicalQubits,
            "Physical qubit resource requirement.",
        )?;

        self.push_count_metric(
            &mut metrics,
            snapshot.circuit_depth,
            MetricKind::CircuitDepth,
            MetricUnit::Layers,
            "Total circuit depth.",
        )?;

        self.push_count_metric(
            &mut metrics,
            snapshot.two_qubit_depth,
            MetricKind::TwoQubitDepth,
            MetricUnit::Layers,
            "Two-qubit operation depth.",
        )?;

        self.push_count_metric(
            &mut metrics,
            snapshot.gate_count,
            MetricKind::GateCount,
            MetricUnit::Gates,
            "Total quantum gate count.",
        )?;

        self.push_count_metric(
            &mut metrics,
            snapshot.two_qubit_gate_count,
            MetricKind::TwoQubitGateCount,
            MetricUnit::TwoQubitGates,
            "Total two-qubit gate count.",
        )?;

        self.push_count_metric(
            &mut metrics,
            snapshot.measurement_count,
            MetricKind::MeasurementCount,
            MetricUnit::Operations,
            "Total measurement operation count.",
        )?;

        self.push_count_metric(
            &mut metrics,
            snapshot.t_gate_count,
            MetricKind::TGateCount,
            MetricUnit::TGates,
            "Total T-gate count.",
        )?;

        self.push_count_metric(
            &mut metrics,
            snapshot.classical_operation_count,
            MetricKind::ClassicalOperationCount,
            MetricUnit::Operations,
            "Total classical operation count.",
        )?;

        self.push_count_metric(
            &mut metrics,
            snapshot.shot_count,
            MetricKind::ShotCount,
            MetricUnit::Shots,
            "Total execution shot count.",
        )?;

        self.push_count_metric(
            &mut metrics,
            snapshot.circuit_count,
            MetricKind::CircuitCount,
            MetricUnit::Circuits,
            "Total executed circuit/instance count.",
        )?;

        self.push_count_metric(
            &mut metrics,
            snapshot.memory_bytes,
            MetricKind::Memory,
            MetricUnit::Bytes,
            "Memory resource consumption.",
        )?;

        if let Some(energy) = snapshot.energy_joules {
            let metric = Metric::observed(
                MetricKind::Energy,
                MetricUnit::Joules,
                energy.get(),
            )
            .map_err(ResourceMetricError::Metric)?;

            metrics.push(
                metric
                    .with_description(
                        "Measured energy resource consumption.",
                    )
                    .map_err(ResourceMetricError::Metric)?,
            );
        }

        for custom in &snapshot.custom {
            metrics.push(self.custom_metric(custom)?);
        }

        Ok(metrics)
    }

    /// Creates one metric from a built-in resource count.
    pub fn metric_for_count(
        &self,
        kind: ResourceKind,
        value: ResourceCount,
    ) -> Result<Metric, ResourceMetricError> {
        let (metric_kind, unit, description) =
            metric_definition(&kind)?;

        Metric::observed(
            metric_kind,
            unit,
            value.get() as f64,
        )
        .map_err(ResourceMetricError::Metric)?
        .with_description(description)
        .map_err(ResourceMetricError::Metric)
    }

    fn push_count_metric(
        &self,
        output: &mut Vec<Metric>,
        value: Option<ResourceCount>,
        metric_kind: MetricKind,
        unit: MetricUnit,
        description: &'static str,
    ) -> Result<(), ResourceMetricError> {
        if let Some(value) = value {
            let metric = Metric::observed(
                metric_kind,
                unit,
                value.get() as f64,
            )
            .map_err(ResourceMetricError::Metric)?
            .with_description(description)
            .map_err(ResourceMetricError::Metric)?;

            output.push(metric);
        }

        Ok(())
    }

    fn custom_metric(
        &self,
        resource: &CustomResource,
    ) -> Result<Metric, ResourceMetricError> {
        let kind = MetricKind::Custom(resource.kind.clone());
        let unit = MetricUnit::Custom(resource.unit.clone());

        let mut metric = Metric::observed(
            kind,
            unit,
            resource.value.get(),
        )
        .map_err(ResourceMetricError::Metric)?;

        if let Some(description) = &resource.description {
            metric = metric
                .with_description(description.clone())
                .map_err(ResourceMetricError::Metric)?;
        }

        Ok(metric)
    }
}

// =============================================================================
// Canonical resource definitions
// =============================================================================

fn metric_definition(
    kind: &ResourceKind,
) -> Result<
    (MetricKind, MetricUnit, &'static str),
    ResourceMetricError,
> {
    let definition = match kind {
        ResourceKind::Qubits => (
            MetricKind::QubitCount,
            MetricUnit::Qubits,
            "Total qubit resource requirement.",
        ),

        ResourceKind::LogicalQubits => (
            MetricKind::LogicalQubitCount,
            MetricUnit::LogicalQubits,
            "Logical qubit resource requirement.",
        ),

        ResourceKind::PhysicalQubits => (
            MetricKind::PhysicalQubitCount,
            MetricUnit::PhysicalQubits,
            "Physical qubit resource requirement.",
        ),

        ResourceKind::CircuitDepth => (
            MetricKind::CircuitDepth,
            MetricUnit::Layers,
            "Total circuit depth.",
        ),

        ResourceKind::TwoQubitDepth => (
            MetricKind::TwoQubitDepth,
            MetricUnit::Layers,
            "Two-qubit operation depth.",
        ),

        ResourceKind::GateCount => (
            MetricKind::GateCount,
            MetricUnit::Gates,
            "Total quantum gate count.",
        ),

        ResourceKind::TwoQubitGateCount => (
            MetricKind::TwoQubitGateCount,
            MetricUnit::TwoQubitGates,
            "Total two-qubit gate count.",
        ),

        ResourceKind::MeasurementCount => (
            MetricKind::MeasurementCount,
            MetricUnit::Operations,
            "Total measurement operation count.",
        ),

        ResourceKind::TGateCount => (
            MetricKind::TGateCount,
            MetricUnit::TGates,
            "Total T-gate count.",
        ),

        ResourceKind::ClassicalOperationCount => (
            MetricKind::ClassicalOperationCount,
            MetricUnit::Operations,
            "Total classical operation count.",
        ),

        ResourceKind::ShotCount => (
            MetricKind::Custom("shot_count".to_owned()),
            MetricUnit::Shots,
            "Total execution shot count.",
        ),

        ResourceKind::CircuitCount => (
            MetricKind::Custom("circuit_count".to_owned()),
            MetricUnit::Circuits,
            "Total executed circuit/instance count.",
        ),

        ResourceKind::MemoryBytes => (
            MetricKind::Memory,
            MetricUnit::Bytes,
            "Memory resource consumption.",
        ),

        ResourceKind::EnergyJoules => (
            MetricKind::Energy,
            MetricUnit::Joules,
            "Measured energy resource consumption.",
        ),

        ResourceKind::Custom(id) => (
            MetricKind::Custom(id.clone()),
            MetricUnit::Custom("custom".to_owned()),
            "Custom quantum resource.",
        ),
    };

    Ok(definition)
}

// =============================================================================
// Resource aggregation helpers
// =============================================================================

fn checked_add_optional(
    left: Option<ResourceCount>,
    right: Option<ResourceCount>,
) -> Result<Option<ResourceCount>, ResourceError> {
    match (left, right) {
        (Some(left), Some(right)) => {
            Ok(Some(left.checked_add(right)?))
        }

        (Some(value), None) | (None, Some(value)) => Some(value)
            .map(Some)
            .ok_or(ResourceError::InvalidOptionalState),

        (None, None) => Ok(None),
    }
}

fn checked_delta_optional(
    current: Option<ResourceCount>,
    previous: Option<ResourceCount>,
) -> Result<Option<ResourceCount>, ResourceError> {
    match (current, previous) {
        (Some(current), Some(previous)) => {
            Ok(Some(current.checked_sub(previous)?))
        }

        _ => Ok(None),
    }
}

fn checked_add_optional_float(
    left: Option<FiniteResourceValue>,
    right: Option<FiniteResourceValue>,
) -> Result<Option<FiniteResourceValue>, ResourceError> {
    match (left, right) {
        (Some(left), Some(right)) => {
            Ok(Some(left.checked_add(right)?))
        }

        (Some(value), None) | (None, Some(value)) => Ok(Some(value)),

        (None, None) => Ok(None),
    }
}

fn checked_delta_optional_float(
    current: Option<FiniteResourceValue>,
    previous: Option<FiniteResourceValue>,
) -> Result<Option<FiniteResourceValue>, ResourceError> {
    match (current, previous) {
        (Some(current), Some(previous)) => {
            Ok(Some(current.checked_sub(previous)?))
        }

        _ => Ok(None),
    }
}

fn merge_custom_resources(
    left: &[CustomResource],
    right: &[CustomResource],
) -> Result<Vec<CustomResource>, ResourceError> {
    if left.len() + right.len() > MAX_CUSTOM_RESOURCES {
        return Err(ResourceError::TooManyCustomResources {
            count: left.len() + right.len(),
            maximum: MAX_CUSTOM_RESOURCES,
        });
    }

    let mut result = Vec::with_capacity(
        left.len() + right.len(),
    );

    for resource in left {
        result.push(resource.clone());
    }

    for resource in right {
        if let Some(existing) = result
            .iter_mut()
            .find(|item| item.kind == resource.kind)
        {
            if existing.unit != resource.unit {
                return Err(ResourceError::CustomUnitMismatch {
                    kind: resource.kind.clone(),
                    left_unit: existing.unit.clone(),
                    right_unit: resource.unit.clone(),
                });
            }

            let value = existing
                .value
                .checked_add(resource.value)?;

            existing.value = value;
        } else {
            result.push(resource.clone());
        }
    }

    Ok(result)
}

fn delta_custom_resources(
    current: &[CustomResource],
    previous: &[CustomResource],
) -> Result<Vec<CustomResource>, ResourceError> {
    let mut result = Vec::with_capacity(
        current.len(),
    );

    for resource in current {
        if let Some(previous_resource) = previous
            .iter()
            .find(|item| item.kind == resource.kind)
        {
            if previous_resource.unit != resource.unit {
                return Err(ResourceError::CustomUnitMismatch {
                    kind: resource.kind.clone(),
                    left_unit: resource.unit.clone(),
                    right_unit: previous_resource.unit.clone(),
                });
            }

            result.push(CustomResource {
                kind: resource.kind.clone(),
                value: resource
                    .value
                    .checked_sub(previous_resource.value)?,
                unit: resource.unit.clone(),
                description: resource.description.clone(),
            });
        }
    }

    if result.len() > MAX_CUSTOM_RESOURCES {
        return Err(ResourceError::TooManyCustomResources {
            count: result.len(),
            maximum: MAX_CUSTOM_RESOURCES,
        });
    }

    Ok(result)
}

// =============================================================================
// Validation
// =============================================================================

fn validate_identifier(
    value: &str,
    maximum_bytes: usize,
    field: &'static str,
) -> Result<(), ResourceError> {
    if value.trim().is_empty() {
        return Err(ResourceError::EmptyIdentifier { field });
    }

    if value.len() > maximum_bytes {
        return Err(ResourceError::IdentifierTooLong {
            field,
            length: value.len(),
            maximum: maximum_bytes,
        });
    }

    let bytes = value.as_bytes();

    if !bytes[0].is_ascii_lowercase() {
        return Err(ResourceError::InvalidIdentifier {
            field,
            reason: "identifier must begin with a lowercase ASCII letter",
        });
    }

    for byte in bytes.iter().skip(1) {
        if !(byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || *byte == b'_')
        {
            return Err(ResourceError::InvalidIdentifier {
                field,
                reason: "identifier may contain only lowercase ASCII letters, digits, and underscores",
            });
        }
    }

    Ok(())
}

fn validate_unit(unit: &str) -> Result<(), ResourceError> {
    if unit.trim().is_empty() {
        return Err(ResourceError::EmptyIdentifier {
            field: "custom resource unit",
        });
    }

    if unit.len() > MAX_CUSTOM_RESOURCE_UNIT_BYTES {
        return Err(ResourceError::IdentifierTooLong {
            field: "custom resource unit",
            length: unit.len(),
            maximum: MAX_CUSTOM_RESOURCE_UNIT_BYTES,
        });
    }

    Ok(())
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by resource accounting.
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceError {
    /// Resource schema version is invalid.
    InvalidSchemaVersion {
        /// Invalid schema version.
        version: u32,
    },

    /// Arithmetic exceeded the representable range.
    ArithmeticOverflow,

    /// Subtraction would result in a negative resource.
    NegativeResult,

    /// A floating-point resource was NaN or infinite.
    NonFiniteValue {
        /// Invalid floating-point value.
        value: f64,
    },

    /// A resource value was negative.
    NegativeValue {
        /// Invalid negative value.
        value: f64,
    },

    /// An identifier was empty.
    EmptyIdentifier {
        /// Field containing the invalid identifier.
        field: &'static str,
    },

    /// An identifier exceeded its configured bound.
    IdentifierTooLong {
        /// Field containing the oversized value.
        field: &'static str,

        /// Actual UTF-8 byte length.
        length: usize,

        /// Maximum accepted byte length.
        maximum: usize,
    },

    /// An identifier violates the canonical resource naming rules.
    InvalidIdentifier {
        /// Field containing the invalid identifier.
        field: &'static str,

        /// Human-readable validation reason.
        reason: &'static str,
    },

    /// Too many custom resources were supplied.
    TooManyCustomResources {
        /// Number supplied.
        count: usize,

        /// Maximum accepted.
        maximum: usize,
    },

    /// Two resources with the same identifier use incompatible units.
    CustomUnitMismatch {
        /// Resource identifier.
        kind: String,

        /// Unit of the first resource.
        left_unit: String,

        /// Unit of the second resource.
        right_unit: String,
    },

    /// Internal optional-state invariant was violated.
    InvalidOptionalState,
}

impl std::fmt::Display for ResourceError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::InvalidSchemaVersion { version } => write!(
                formatter,
                "unsupported resource snapshot schema version: {}",
                version
            ),

            Self::ArithmeticOverflow => {
                formatter.write_str(
                    "resource arithmetic overflowed the supported range",
                )
            }

            Self::NegativeResult => {
                formatter.write_str(
                    "resource subtraction would produce a negative result",
                )
            }

            Self::NonFiniteValue { value } => write!(
                formatter,
                "resource value must be finite: {}",
                value
            ),

            Self::NegativeValue { value } => write!(
                formatter,
                "resource value cannot be negative: {}",
                value
            ),

            Self::EmptyIdentifier { field } => write!(
                formatter,
                "{} cannot be empty",
                field
            ),

            Self::IdentifierTooLong {
                field,
                length,
                maximum,
            } => write!(
                formatter,
                "{} is {} bytes; maximum is {} bytes",
                field,
                length,
                maximum
            ),

            Self::InvalidIdentifier {
                field,
                reason,
            } => write!(
                formatter,
                "invalid {}: {}",
                field,
                reason
            ),

            Self::TooManyCustomResources {
                count,
                maximum,
            } => write!(
                formatter,
                "resource snapshot contains {} custom resources; maximum is {}",
                count,
                maximum
            ),

            Self::CustomUnitMismatch {
                kind,
                left_unit,
                right_unit,
            } => write!(
                formatter,
                "custom resource '{}' has incompatible units '{}' and '{}'",
                kind,
                left_unit,
                right_unit
            ),

            Self::InvalidOptionalState => {
                formatter.write_str(
                    "invalid optional resource state",
                )
            }
        }
    }
}

impl std::error::Error for ResourceError {}

/// Errors produced while converting resources into canonical metrics.
#[derive(Debug)]
pub enum ResourceMetricError {
    /// The source resource snapshot was invalid.
    Resource(ResourceError),

    /// The canonical metric rejected the constructed resource metric.
    Metric(crate::quantum::benchmarking::core::metric::MetricError),
}

impl std::fmt::Display for ResourceMetricError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Resource(error) => {
                write!(formatter, "invalid resource data: {}", error)
            }

            Self::Metric(error) => {
                write!(formatter, "invalid resource metric: {}", error)
            }
        }
    }
}

impl std::error::Error for ResourceMetricError {}

impl From<ResourceError> for ResourceMetricError {
    fn from(error: ResourceError) -> Self {
        Self::Resource(error)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_is_valid() {
        let snapshot = ResourceSnapshot::new();

        assert!(snapshot.validate().is_ok());
        assert!(snapshot.is_empty());
        assert_eq!(snapshot.populated_count(), 0);
    }

    #[test]
    fn resource_count_supports_checked_addition() {
        let left = ResourceCount::new(10);
        let right = ResourceCount::new(20);

        assert_eq!(
            left.checked_add(right).unwrap().get(),
            30
        );
    }

    #[test]
    fn resource_count_detects_overflow() {
        let left = ResourceCount::new(u64::MAX);
        let right = ResourceCount::new(1);

        assert_eq!(
            left.checked_add(right),
            Err(ResourceError::ArithmeticOverflow)
        );
    }

    #[test]
    fn resource_count_detects_negative_subtraction() {
        let left = ResourceCount::new(10);
        let right = ResourceCount::new(11);

        assert_eq!(
            left.checked_sub(right),
            Err(ResourceError::NegativeResult)
        );
    }

    #[test]
    fn finite_energy_rejects_nan() {
        assert_eq!(
            FiniteResourceValue::new(f64::NAN),
            Err(ResourceError::NonFiniteValue {
                value: f64::NAN
            })
        );
    }

    #[test]
    fn finite_energy_rejects_infinity() {
        assert_eq!(
            FiniteResourceValue::new(f64::INFINITY),
            Err(ResourceError::NonFiniteValue {
                value: f64::INFINITY
            })
        );
    }

    #[test]
    fn finite_energy_rejects_negative_values() {
        assert_eq!(
            FiniteResourceValue::new(-1.0),
            Err(ResourceError::NegativeValue {
                value: -1.0
            })
        );
    }

    #[test]
    fn zero_and_unavailable_are_distinct() {
        let mut snapshot = ResourceSnapshot::new();

        snapshot.gate_count = Some(ResourceCount::new(0));

        assert!(!snapshot.is_empty());
        assert_eq!(
            snapshot.get_count(&ResourceKind::GateCount),
            Some(ResourceCount::new(0))
        );

        snapshot.gate_count = None;

        assert_eq!(
            snapshot.get_count(&ResourceKind::GateCount),
            None
        );
    }

    #[test]
    fn snapshot_addition_is_checked() {
        let mut left = ResourceSnapshot::new();
        let mut right = ResourceSnapshot::new();

        left.gate_count = Some(ResourceCount::new(10));
        right.gate_count = Some(ResourceCount::new(20));

        let result = left.checked_add(&right).unwrap();

        assert_eq!(
            result.gate_count,
            Some(ResourceCount::new(30))
        );
    }

    #[test]
    fn snapshot_addition_detects_overflow() {
        let mut left = ResourceSnapshot::new();
        let mut right = ResourceSnapshot::new();

        left.gate_count = Some(ResourceCount::new(u64::MAX));
        right.gate_count = Some(ResourceCount::new(1));

        assert_eq!(
            left.checked_add(&right),
            Err(ResourceError::ArithmeticOverflow)
        );
    }

    #[test]
    fn snapshot_delta_is_non_negative() {
        let mut previous = ResourceSnapshot::new();
        let mut current = ResourceSnapshot::new();

        previous.gate_count = Some(ResourceCount::new(10));
        current.gate_count = Some(ResourceCount::new(30));

        let delta = current
            .checked_delta(&previous)
            .unwrap();

        assert_eq!(
            delta.gate_count,
            Some(ResourceCount::new(20))
        );
    }

    #[test]
    fn snapshot_delta_rejects_counter_regression() {
        let mut previous = ResourceSnapshot::new();
        let mut current = ResourceSnapshot::new();

        previous.gate_count = Some(ResourceCount::new(30));
        current.gate_count = Some(ResourceCount::new(10));

        assert_eq!(
            current.checked_delta(&previous),
            Err(ResourceError::NegativeResult)
        );
    }

    #[test]
    fn snapshot_delta_does_not_fabricate_missing_resources() {
        let mut previous = ResourceSnapshot::new();
        let mut current = ResourceSnapshot::new();

        previous.gate_count = Some(ResourceCount::new(10));
        current.gate_count = None;

        let delta = current
            .checked_delta(&previous)
            .unwrap();

        assert_eq!(delta.gate_count, None);
    }

    #[test]
    fn resource_metrics_create_canonical_metrics() {
        let mut snapshot = ResourceSnapshot::new();

        snapshot.qubits = Some(ResourceCount::new(20));
        snapshot.logical_qubits = Some(ResourceCount::new(10));
        snapshot.physical_qubits = Some(ResourceCount::new(100));
        snapshot.gate_count = Some(ResourceCount::new(500));
        snapshot.two_qubit_gate_count =
            Some(ResourceCount::new(200));
        snapshot.shot_count =
            Some(ResourceCount::new(1000));
        snapshot.circuit_count =
            Some(ResourceCount::new(25));

        let metrics = ResourceMetrics::new()
            .from_snapshot(&snapshot)
            .unwrap();

        assert_eq!(metrics.len(), 7);

        assert_eq!(
            metrics[0].kind,
            MetricKind::QubitCount
        );

        assert_eq!(
            metrics[0].unit,
            MetricUnit::Qubits
        );

        assert_eq!(
            metrics[0].value.get(),
            20.0
        );
    }

    #[test]
    fn memory_metric_uses_bytes() {
        let mut snapshot = ResourceSnapshot::new();

        snapshot.memory_bytes =
            Some(ResourceCount::new(4096));

        let metrics = ResourceMetrics::new()
            .from_snapshot(&snapshot)
            .unwrap();

        assert_eq!(
            metrics[0].kind,
            MetricKind::Memory
        );

        assert_eq!(
            metrics[0].unit,
            MetricUnit::Bytes
        );

        assert_eq!(
            metrics[0].value.get(),
            4096.0
        );
    }

    #[test]
    fn energy_metric_uses_joules() {
        let mut snapshot = ResourceSnapshot::new();

        snapshot.energy_joules =
            Some(FiniteResourceValue::new(0.125).unwrap());

        let metrics = ResourceMetrics::new()
            .from_snapshot(&snapshot)
            .unwrap();

        assert_eq!(
            metrics[0].kind,
            MetricKind::Energy
        );

        assert_eq!(
            metrics[0].unit,
            MetricUnit::Joules
        );

        assert_eq!(
            metrics[0].value.get(),
            0.125
        );
    }

    #[test]
    fn custom_resource_is_preserved() {
        let custom = CustomResource::new(
            "optical_modes",
            32.0,
            "modes",
        )
        .unwrap();

        let mut snapshot = ResourceSnapshot::new();
        snapshot.custom.push(custom);

        let metrics = ResourceMetrics::new()
            .from_snapshot(&snapshot)
            .unwrap();

        assert_eq!(metrics.len(), 1);
        assert_eq!(
            metrics[0].kind,
            MetricKind::Custom(
                "optical_modes".to_owned()
            )
        );
        assert_eq!(
            metrics[0].unit,
            MetricUnit::Custom(
                "modes".to_owned()
            )
        );
    }

    #[test]
    fn custom_resource_identifiers_are_bounded() {
        let oversized =
            "a".repeat(MAX_CUSTOM_RESOURCE_ID_BYTES + 1);

        assert!(matches!(
            CustomResource::new(
                oversized,
                1.0,
                "units",
            ),
            Err(ResourceError::IdentifierTooLong { .. })
        ));
    }

    #[test]
    fn custom_resource_units_must_match_when_aggregated() {
        let left = CustomResource::new(
            "optical_modes",
            10.0,
            "modes",
        )
        .unwrap();

        let right = CustomResource::new(
            "optical_modes",
            10.0,
            "photons",
        )
        .unwrap();

        let mut snapshot_left = ResourceSnapshot::new();
        let mut snapshot_right = ResourceSnapshot::new();

        snapshot_left.custom.push(left);
        snapshot_right.custom.push(right);

        assert!(matches!(
            snapshot_left.checked_add(&snapshot_right),
            Err(ResourceError::CustomUnitMismatch { .. })
        ));
    }

    #[test]
    fn resource_kind_ids_are_stable() {
        assert_eq!(
            ResourceKind::Qubits.id(),
            "qubits"
        );

        assert_eq!(
            ResourceKind::LogicalQubits.id(),
            "logical_qubits"
        );

        assert_eq!(
            ResourceKind::PhysicalQubits.id(),
            "physical_qubits"
        );

        assert_eq!(
            ResourceKind::TwoQubitGateCount.id(),
            "two_qubit_gate_count"
        );
    }

    #[test]
    fn resource_metric_output_is_deterministically_ordered() {
        let mut snapshot = ResourceSnapshot::new();

        snapshot.circuit_count =
            Some(ResourceCount::new(2));

        snapshot.qubits =
            Some(ResourceCount::new(4));

        snapshot.gate_count =
            Some(ResourceCount::new(10));

        let metrics = ResourceMetrics::new()
            .from_snapshot(&snapshot)
            .unwrap();

        assert_eq!(
            metrics[0].kind,
            MetricKind::QubitCount
        );

        assert_eq!(
            metrics[1].kind,
            MetricKind::GateCount
        );

        assert_eq!(
            metrics[2].kind,
            MetricKind::CircuitCount
        );
    }
}