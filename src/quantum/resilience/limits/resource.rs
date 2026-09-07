//! Zamani Quantum Resilience — Resource Model
//!
//! Path:
//!     src/quantum/resilience/limits/resource.rs
//!
//! Purpose:
//!     Provides the provider-independent resource model consumed by the
//!     resilience limits and validation layers.
//!
//! Architectural position:
//!
//!     quantum::ir::qubit
//!         └── owns logical/physical qubit identity
//!
//!     quantum::hardware
//!         └── owns actual target capabilities and machine resources
//!
//!     quantum::routing
//!         └── determines physical realization requirements
//!
//!     quantum::scheduling
//!         └── determines temporal/resource scheduling requirements
//!
//!     quantum::optimization
//!         └── may change resource demand
//!
//!     quantum::resilience::limits::resource
//!         └── models demand, availability and reservations
//!
//!     quantum::resilience::limits::limits
//!         └── models explicit resilience constraints
//!
//!     quantum::resilience::limits::validation
//!         └── combines requirements, availability and limits
//!
//!     quantum::resilience::planning
//!         └── uses resource feasibility when ranking plans
//!
//! This module does NOT:
//!
//!     - discover hardware;
//!     - perform routing;
//!     - perform scheduling;
//!     - perform optimization;
//!     - implement QEC;
//!     - implement error mitigation;
//!     - execute quantum operations;
//!     - define provider-specific resources;
//!     - define another qubit identity system;
//!     - assume a fixed number of qubits;
//!     - impose artificial machine-size limits.
//!
//! # Scalability
//!
//! The resource model is deliberately independent of machine size.
//!
//! A resource quantity is represented using `u64` because resource quantities
//! are serialized and compared as portable non-negative integers. `Unlimited`
//! is represented by the `Limit` type in `limits.rs`; it is never represented
//! by `u64::MAX`.
//!
//! This distinction is important:
//!
//!     u64::MAX
//!         = a very large finite quantity
//!
//!     Limit::Unlimited
//!         = no artificial limit imposed by this layer
//!
//! Actual hardware capacity is always supplied by the hardware/runtime
//! capability layer. `Unlimited` must never be interpreted as physically
//! infinite hardware.
//!
//! # Unknown resources
//!
//! Resource availability and resource requirements are different concepts.
//!
//! A missing hardware observation is represented as `Unknown`, not as
//! `Unlimited`. Safety-critical validation must not silently convert unknown
//! capacity into available capacity.
//!
//! # Canonical qubit identity
//!
//! This file intentionally models qubit resources primarily as quantities.
//! It does not create a second identity type.
//!
//! Whenever an API outside this file needs to identify a specific quantum
//! resource, it MUST use the canonical types:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! Do not introduce:
//!
//!     ResilienceQubitId
//!     ResourceQubitId
//!     LogicalQubitId
//!     PhysicalResourceId
//!
//! as replacements for the canonical IR identities.
//!
//! # Integration contract
//!
//! `limits.rs`
//!     Owns `Limit`, `LimitSet`, `ResourceKind`, and `LimitViolation`.
//!
//! `validation.rs`
//!     Should use `ResourceDemand`, `ResourceAvailability`, and
//!     `ResourceAssessment` from this module.
//!
//! `policy/budgets.rs`
//!     Supplies policy-level limits which become `LimitSet` values.
//!
//! `hardware`
//!     Supplies target-derived availability.
//!
//! `routing`
//!     Supplies physical-qubit and coupling requirements.
//!
//! `scheduling`
//!     Supplies concurrency and temporal requirements.
//!
//! `optimization`
//!     May alter operation/depth/qubit/resource requirements.
//!
//! `planning`
//!     Uses resource assessment when determining whether a recovery/adaptation
//!     plan is feasible.
//!
//! `recovery`
//!     Must reserve/check resources before starting an action whose resource
//!     requirements are known.
//!
//! `telemetry`
//!     May update observations, but observations must not be treated as
//!     authoritative capabilities unless the hardware capability layer says
//!     so.
//!
//! `serialization`
//!     May serialize these structures for deterministic provenance and replay.
//!
//! `verification`
//!     May use the original and effective resource vectors to verify that an
//!     adaptation did not exceed declared resource constraints.
//!
//! # Determinism
//!
//! All operations in this file are deterministic.
//!
//! No:
//!
//!     - wall-clock reads;
//!     - random numbers;
//!     - provider-specific ordering;
//!     - hash-map iteration;
//!     - floating-point comparisons;
//!     - implicit environmental discovery
//!
//! are used.
//!
//! # Safety
//!
//! This file contains no `unsafe` code.

use core::fmt;

use super::limits::{Limit, LimitSet, LimitViolation, ResourceKind};


/// A resource quantity with explicit knowledge state.
///
/// `Known(value)` means the quantity is known exactly.
///
/// `Unknown` means the value is not currently known.
///
/// Unknown is deliberately distinct from zero and from unlimited.
///
/// This type is used primarily for availability because treating an unknown
/// resource capacity as zero would cause false failures while treating it as
/// unlimited would create an unsafe fail-open condition.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResourceValue {
    /// The quantity is known.
    Known(u64),

    /// The quantity is currently unknown.
    Unknown,
}

impl ResourceValue {
    /// Creates a known resource quantity.
    #[must_use]
    pub const fn known(value: u64) -> Self {
        Self::Known(value)
    }

    /// Creates an unknown resource quantity.
    #[must_use]
    pub const fn unknown() -> Self {
        Self::Unknown
    }

    /// Returns the known quantity, if available.
    #[must_use]
    pub const fn value(self) -> Option<u64> {
        match self {
            Self::Known(value) => Some(value),
            Self::Unknown => None,
        }
    }

    /// Returns whether the value is known.
    #[must_use]
    pub const fn is_known(self) -> bool {
        matches!(self, Self::Known(_))
    }

    /// Returns whether the value is unknown.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Returns whether the known value is zero.
    ///
    /// Unknown is never interpreted as zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        matches!(self, Self::Known(0))
    }

    /// Returns whether the value can satisfy a requested quantity.
    ///
    /// Unknown returns `false` because safety-critical resource validation
    /// cannot prove feasibility from missing information.
    #[must_use]
    pub const fn can_supply(self, requested: u64) -> bool {
        match self {
            Self::Known(value) => requested <= value,
            Self::Unknown => false,
        }
    }

    /// Returns the minimum of two known values.
    ///
    /// If either value is unknown, the result is unknown.
    #[must_use]
    pub const fn min(self, other: Self) -> Self {
        match (self, other) {
            (Self::Known(a), Self::Known(b)) => {
                Self::Known(if a < b { a } else { b })
            }
            _ => Self::Unknown,
        }
    }

    /// Returns a checked subtraction.
    ///
    /// Unknown remains unknown.
    pub const fn checked_sub(self, amount: u64) -> Option<Self> {
        match self {
            Self::Known(value) => match value.checked_sub(amount) {
                Some(remaining) => Some(Self::Known(remaining)),
                None => None,
            },
            Self::Unknown => Some(Self::Unknown),
        }
    }

    /// Returns a checked addition.
    ///
    /// Overflow is reported as `None`.
    pub const fn checked_add(self, amount: u64) -> Option<Self> {
        match self {
            Self::Known(value) => match value.checked_add(amount) {
                Some(total) => Some(Self::Known(total)),
                None => None,
            },
            Self::Unknown => Some(Self::Unknown),
        }
    }
}

impl Default for ResourceValue {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for ResourceValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Known(value) => write!(formatter, "{value}"),
            Self::Unknown => formatter.write_str("unknown"),
        }
    }
}


/// Resource demand for one resilience operation, execution, or plan.
///
/// All quantities are requirements, not capabilities.
///
/// A value of zero means that the operation requires none of that resource.
///
/// This structure intentionally mirrors the resource dimensions defined by
/// `LimitSet` so that `limits.rs` remains the single owner of limit semantics.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceDemand {
    /// Logical qubits required.
    pub logical_qubits: u64,

    /// Physical qubits required.
    pub physical_qubits: u64,

    /// Quantum operations/instructions required.
    pub operations: u64,

    /// Circuit depth required.
    pub circuit_depth: u64,

    /// Shots/samples required.
    pub shots: u64,

    /// Recovery attempts consumed.
    pub recovery_attempts: u64,

    /// Mitigation executions consumed.
    pub mitigation_executions: u64,

    /// Checkpoints consumed/created.
    pub checkpoints: u64,

    /// Storage bytes required.
    pub storage_bytes: u64,

    /// CPU resource units required.
    pub cpu_units: u64,

    /// GPU resource units required.
    pub gpu_units: u64,

    /// Network bytes required.
    pub network_bytes: u64,

    /// Telemetry events generated/retained.
    pub telemetry_events: u64,

    /// Concurrent executions/tasks required.
    pub concurrent_executions: u64,

    /// Execution time in nanoseconds.
    pub execution_time_nanos: u64,

    /// Compilation time in nanoseconds.
    pub compilation_time_nanos: u64,

    /// Queue/wait time in nanoseconds.
    pub queue_time_nanos: u64,

    /// Recovery time in nanoseconds.
    pub recovery_time_nanos: u64,

    /// Caller-defined cost units.
    pub cost_units: u64,

    /// Caller/target-defined energy units.
    pub energy_units: u64,
}

impl ResourceDemand {
    /// Creates a zero-demand resource vector.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            logical_qubits: 0,
            physical_qubits: 0,
            operations: 0,
            circuit_depth: 0,
            shots: 0,
            recovery_attempts: 0,
            mitigation_executions: 0,
            checkpoints: 0,
            storage_bytes: 0,
            cpu_units: 0,
            gpu_units: 0,
            network_bytes: 0,
            telemetry_events: 0,
            concurrent_executions: 0,
            execution_time_nanos: 0,
            compilation_time_nanos: 0,
            queue_time_nanos: 0,
            recovery_time_nanos: 0,
            cost_units: 0,
            energy_units: 0,
        }
    }

    /// Returns a demand vector with one resource dimension populated.
    #[must_use]
    pub const fn single(resource: ResourceKind, amount: u64) -> Self {
        let mut demand = Self::zero();

        match resource {
            ResourceKind::Generic => {}
            ResourceKind::LogicalQubits => demand.logical_qubits = amount,
            ResourceKind::PhysicalQubits => demand.physical_qubits = amount,
            ResourceKind::Operations => demand.operations = amount,
            ResourceKind::CircuitDepth => demand.circuit_depth = amount,
            ResourceKind::Shots => demand.shots = amount,
            ResourceKind::RecoveryAttempts => demand.recovery_attempts = amount,
            ResourceKind::MitigationExecutions => {
                demand.mitigation_executions = amount;
            }
            ResourceKind::Checkpoints => demand.checkpoints = amount,
            ResourceKind::StorageBytes => demand.storage_bytes = amount,
            ResourceKind::CpuUnits => demand.cpu_units = amount,
            ResourceKind::GpuUnits => demand.gpu_units = amount,
            ResourceKind::NetworkBytes => demand.network_bytes = amount,
            ResourceKind::TelemetryEvents => demand.telemetry_events = amount,
            ResourceKind::ConcurrentExecutions => {
                demand.concurrent_executions = amount;
            }
            ResourceKind::ExecutionTimeNanos => {
                demand.execution_time_nanos = amount;
            }
            ResourceKind::CompilationTimeNanos => {
                demand.compilation_time_nanos = amount;
            }
            ResourceKind::QueueTimeNanos => {
                demand.queue_time_nanos = amount;
            }
            ResourceKind::RecoveryTimeNanos => {
                demand.recovery_time_nanos = amount;
            }
            ResourceKind::CostUnits => demand.cost_units = amount,
            ResourceKind::EnergyUnits => demand.energy_units = amount,
        }

        demand
    }

    /// Returns the requested quantity for a resource kind.
    #[must_use]
    pub const fn get(self, resource: ResourceKind) -> u64 {
        match resource {
            ResourceKind::Generic => 0,
            ResourceKind::LogicalQubits => self.logical_qubits,
            ResourceKind::PhysicalQubits => self.physical_qubits,
            ResourceKind::Operations => self.operations,
            ResourceKind::CircuitDepth => self.circuit_depth,
            ResourceKind::Shots => self.shots,
            ResourceKind::RecoveryAttempts => self.recovery_attempts,
            ResourceKind::MitigationExecutions => self.mitigation_executions,
            ResourceKind::Checkpoints => self.checkpoints,
            ResourceKind::StorageBytes => self.storage_bytes,
            ResourceKind::CpuUnits => self.cpu_units,
            ResourceKind::GpuUnits => self.gpu_units,
            ResourceKind::NetworkBytes => self.network_bytes,
            ResourceKind::TelemetryEvents => self.telemetry_events,
            ResourceKind::ConcurrentExecutions => self.concurrent_executions,
            ResourceKind::ExecutionTimeNanos => self.execution_time_nanos,
            ResourceKind::CompilationTimeNanos => self.compilation_time_nanos,
            ResourceKind::QueueTimeNanos => self.queue_time_nanos,
            ResourceKind::RecoveryTimeNanos => self.recovery_time_nanos,
            ResourceKind::CostUnits => self.cost_units,
            ResourceKind::EnergyUnits => self.energy_units,
        }
    }

    /// Returns true when every resource quantity is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.logical_qubits == 0
            && self.physical_qubits == 0
            && self.operations == 0
            && self.circuit_depth == 0
            && self.shots == 0
            && self.recovery_attempts == 0
            && self.mitigation_executions == 0
            && self.checkpoints == 0
            && self.storage_bytes == 0
            && self.cpu_units == 0
            && self.gpu_units == 0
            && self.network_bytes == 0
            && self.telemetry_events == 0
            && self.concurrent_executions == 0
            && self.execution_time_nanos == 0
            && self.compilation_time_nanos == 0
            && self.queue_time_nanos == 0
            && self.recovery_time_nanos == 0
            && self.cost_units == 0
            && self.energy_units == 0
    }

    /// Adds another demand vector with overflow detection.
    ///
    /// This is essential when combining resource requirements from multiple
    /// execution stages.
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        Some(Self {
            logical_qubits: self.logical_qubits.checked_add(other.logical_qubits)?,
            physical_qubits: self.physical_qubits.checked_add(other.physical_qubits)?,
            operations: self.operations.checked_add(other.operations)?,
            circuit_depth: self.circuit_depth.checked_add(other.circuit_depth)?,
            shots: self.shots.checked_add(other.shots)?,
            recovery_attempts: self.recovery_attempts.checked_add(other.recovery_attempts)?,
            mitigation_executions: self
                .mitigation_executions
                .checked_add(other.mitigation_executions)?,
            checkpoints: self.checkpoints.checked_add(other.checkpoints)?,
            storage_bytes: self.storage_bytes.checked_add(other.storage_bytes)?,
            cpu_units: self.cpu_units.checked_add(other.cpu_units)?,
            gpu_units: self.gpu_units.checked_add(other.gpu_units)?,
            network_bytes: self.network_bytes.checked_add(other.network_bytes)?,
            telemetry_events: self.telemetry_events.checked_add(other.telemetry_events)?,
            concurrent_executions: self
                .concurrent_executions
                .checked_add(other.concurrent_executions)?,
            execution_time_nanos: self
                .execution_time_nanos
                .checked_add(other.execution_time_nanos)?,
            compilation_time_nanos: self
                .compilation_time_nanos
                .checked_add(other.compilation_time_nanos)?,
            queue_time_nanos: self
                .queue_time_nanos
                .checked_add(other.queue_time_nanos)?,
            recovery_time_nanos: self
                .recovery_time_nanos
                .checked_add(other.recovery_time_nanos)?,
            cost_units: self.cost_units.checked_add(other.cost_units)?,
            energy_units: self.energy_units.checked_add(other.energy_units)?,
        })
    }

    /// Adds another demand vector, saturating only at the integer boundary.
    ///
    /// This method is intended for telemetry/estimation where preserving
    /// monotonicity is more useful than failing the entire calculation.
    ///
    /// Safety-critical validation must use `checked_add` instead.
    #[must_use]
    pub const fn saturating_add(self, other: Self) -> Self {
        Self {
            logical_qubits: self.logical_qubits.saturating_add(other.logical_qubits),
            physical_qubits: self.physical_qubits.saturating_add(other.physical_qubits),
            operations: self.operations.saturating_add(other.operations),
            circuit_depth: self.circuit_depth.saturating_add(other.circuit_depth),
            shots: self.shots.saturating_add(other.shots),
            recovery_attempts: self.recovery_attempts.saturating_add(other.recovery_attempts),
            mitigation_executions: self
                .mitigation_executions
                .saturating_add(other.mitigation_executions),
            checkpoints: self.checkpoints.saturating_add(other.checkpoints),
            storage_bytes: self.storage_bytes.saturating_add(other.storage_bytes),
            cpu_units: self.cpu_units.saturating_add(other.cpu_units),
            gpu_units: self.gpu_units.saturating_add(other.gpu_units),
            network_bytes: self.network_bytes.saturating_add(other.network_bytes),
            telemetry_events: self.telemetry_events.saturating_add(other.telemetry_events),
            concurrent_executions: self
                .concurrent_executions
                .saturating_add(other.concurrent_executions),
            execution_time_nanos: self
                .execution_time_nanos
                .saturating_add(other.execution_time_nanos),
            compilation_time_nanos: self
                .compilation_time_nanos
                .saturating_add(other.compilation_time_nanos),
            queue_time_nanos: self
                .queue_time_nanos
                .saturating_add(other.queue_time_nanos),
            recovery_time_nanos: self
                .recovery_time_nanos
                .saturating_add(other.recovery_time_nanos),
            cost_units: self.cost_units.saturating_add(other.cost_units),
            energy_units: self.energy_units.saturating_add(other.energy_units),
        }
    }

    /// Returns the demand for a single resource as a `Limit`-compatible value.
    #[must_use]
    pub const fn limit_value(self, resource: ResourceKind) -> Limit {
        Limit::Bounded(self.get(resource))
    }

    /// Checks this demand against a resilience limit set.
    ///
    /// The first deterministic violation is returned according to the order
    /// of `ResourceKind` dimensions used by this module.
    pub const fn check_limits(self, limits: LimitSet) -> Result<(), LimitViolation> {
        limits.check(ResourceKind::LogicalQubits, self.logical_qubits)?;
        limits.check(ResourceKind::PhysicalQubits, self.physical_qubits)?;
        limits.check(ResourceKind::Operations, self.operations)?;
        limits.check(ResourceKind::CircuitDepth, self.circuit_depth)?;
        limits.check(ResourceKind::Shots, self.shots)?;
        limits.check(ResourceKind::RecoveryAttempts, self.recovery_attempts)?;
        limits.check(
            ResourceKind::MitigationExecutions,
            self.mitigation_executions,
        )?;
        limits.check(ResourceKind::Checkpoints, self.checkpoints)?;
        limits.check(ResourceKind::StorageBytes, self.storage_bytes)?;
        limits.check(ResourceKind::CpuUnits, self.cpu_units)?;
        limits.check(ResourceKind::GpuUnits, self.gpu_units)?;
        limits.check(ResourceKind::NetworkBytes, self.network_bytes)?;
        limits.check(ResourceKind::TelemetryEvents, self.telemetry_events)?;
        limits.check(
            ResourceKind::ConcurrentExecutions,
            self.concurrent_executions,
        )?;
        limits.check(
            ResourceKind::ExecutionTimeNanos,
            self.execution_time_nanos,
        )?;
        limits.check(
            ResourceKind::CompilationTimeNanos,
            self.compilation_time_nanos,
        )?;
        limits.check(ResourceKind::QueueTimeNanos, self.queue_time_nanos)?;
        limits.check(ResourceKind::RecoveryTimeNanos, self.recovery_time_nanos)?;
        limits.check(ResourceKind::CostUnits, self.cost_units)?;
        limits.check(ResourceKind::EnergyUnits, self.energy_units)?;

        Ok(())
    }

    /// Returns whether the complete demand is permitted by the supplied limits.
    #[must_use]
    pub const fn fits_within(self, limits: LimitSet) -> bool {
        self.check_limits(limits).is_ok()
    }
}

impl Default for ResourceDemand {
    fn default() -> Self {
        Self::zero()
    }
}


/// Current resource availability.
///
/// Unlike `ResourceDemand`, availability can be unknown.
///
/// Unknown capacity must not be treated as unlimited.
///
/// Hardware implementations should construct this from their capability and
/// resource-state contracts rather than creating provider-specific versions
/// inside resilience.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceAvailability {
    /// Available logical qubits.
    pub logical_qubits: ResourceValue,

    /// Available physical qubits.
    pub physical_qubits: ResourceValue,

    /// Available operation capacity.
    pub operations: ResourceValue,

    /// Available circuit-depth budget.
    pub circuit_depth: ResourceValue,

    /// Available shot/sample capacity.
    pub shots: ResourceValue,

    /// Available recovery-attempt capacity.
    pub recovery_attempts: ResourceValue,

    /// Available mitigation-execution capacity.
    pub mitigation_executions: ResourceValue,

    /// Available checkpoint capacity.
    pub checkpoints: ResourceValue,

    /// Available storage bytes.
    pub storage_bytes: ResourceValue,

    /// Available CPU resource units.
    pub cpu_units: ResourceValue,

    /// Available GPU resource units.
    pub gpu_units: ResourceValue,

    /// Available network bytes.
    pub network_bytes: ResourceValue,

    /// Available telemetry-event capacity.
    pub telemetry_events: ResourceValue,

    /// Available concurrent executions.
    pub concurrent_executions: ResourceValue,

    /// Available execution time.
    pub execution_time_nanos: ResourceValue,

    /// Available compilation time.
    pub compilation_time_nanos: ResourceValue,

    /// Available queue/wait time.
    pub queue_time_nanos: ResourceValue,

    /// Available recovery time.
    pub recovery_time_nanos: ResourceValue,

    /// Available cost budget.
    pub cost_units: ResourceValue,

    /// Available energy budget.
    pub energy_units: ResourceValue,
}

impl ResourceAvailability {
    /// Creates an availability vector where every resource is unknown.
    #[must_use]
    pub const fn unknown() -> Self {
        Self {
            logical_qubits: ResourceValue::Unknown,
            physical_qubits: ResourceValue::Unknown,
            operations: ResourceValue::Unknown,
            circuit_depth: ResourceValue::Unknown,
            shots: ResourceValue::Unknown,
            recovery_attempts: ResourceValue::Unknown,
            mitigation_executions: ResourceValue::Unknown,
            checkpoints: ResourceValue::Unknown,
            storage_bytes: ResourceValue::Unknown,
            cpu_units: ResourceValue::Unknown,
            gpu_units: ResourceValue::Unknown,
            network_bytes: ResourceValue::Unknown,
            telemetry_events: ResourceValue::Unknown,
            concurrent_executions: ResourceValue::Unknown,
            execution_time_nanos: ResourceValue::Unknown,
            compilation_time_nanos: ResourceValue::Unknown,
            queue_time_nanos: ResourceValue::Unknown,
            recovery_time_nanos: ResourceValue::Unknown,
            cost_units: ResourceValue::Unknown,
            energy_units: ResourceValue::Unknown,
        }
    }

    /// Creates an availability vector with all quantities known to be zero.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            logical_qubits: ResourceValue::Known(0),
            physical_qubits: ResourceValue::Known(0),
            operations: ResourceValue::Known(0),
            circuit_depth: ResourceValue::Known(0),
            shots: ResourceValue::Known(0),
            recovery_attempts: ResourceValue::Known(0),
            mitigation_executions: ResourceValue::Known(0),
            checkpoints: ResourceValue::Known(0),
            storage_bytes: ResourceValue::Known(0),
            cpu_units: ResourceValue::Known(0),
            gpu_units: ResourceValue::Known(0),
            network_bytes: ResourceValue::Known(0),
            telemetry_events: ResourceValue::Known(0),
            concurrent_executions: ResourceValue::Known(0),
            execution_time_nanos: ResourceValue::Known(0),
            compilation_time_nanos: ResourceValue::Known(0),
            queue_time_nanos: ResourceValue::Known(0),
            recovery_time_nanos: ResourceValue::Known(0),
            cost_units: ResourceValue::Known(0),
            energy_units: ResourceValue::Known(0),
        }
    }

    /// Returns the availability for one resource category.
    #[must_use]
    pub const fn get(self, resource: ResourceKind) -> ResourceValue {
        match resource {
            ResourceKind::Generic => ResourceValue::Unknown,
            ResourceKind::LogicalQubits => self.logical_qubits,
            ResourceKind::PhysicalQubits => self.physical_qubits,
            ResourceKind::Operations => self.operations,
            ResourceKind::CircuitDepth => self.circuit_depth,
            ResourceKind::Shots => self.shots,
            ResourceKind::RecoveryAttempts => self.recovery_attempts,
            ResourceKind::MitigationExecutions => self.mitigation_executions,
            ResourceKind::Checkpoints => self.checkpoints,
            ResourceKind::StorageBytes => self.storage_bytes,
            ResourceKind::CpuUnits => self.cpu_units,
            ResourceKind::GpuUnits => self.gpu_units,
            ResourceKind::NetworkBytes => self.network_bytes,
            ResourceKind::TelemetryEvents => self.telemetry_events,
            ResourceKind::ConcurrentExecutions => self.concurrent_executions,
            ResourceKind::ExecutionTimeNanos => self.execution_time_nanos,
            ResourceKind::CompilationTimeNanos => self.compilation_time_nanos,
            ResourceKind::QueueTimeNanos => self.queue_time_nanos,
            ResourceKind::RecoveryTimeNanos => self.recovery_time_nanos,
            ResourceKind::CostUnits => self.cost_units,
            ResourceKind::EnergyUnits => self.energy_units,
        }
    }

    /// Returns whether all dimensions are known.
    #[must_use]
    pub const fn is_fully_known(self) -> bool {
        self.logical_qubits.is_known()
            && self.physical_qubits.is_known()
            && self.operations.is_known()
            && self.circuit_depth.is_known()
            && self.shots.is_known()
            && self.recovery_attempts.is_known()
            && self.mitigation_executions.is_known()
            && self.checkpoints.is_known()
            && self.storage_bytes.is_known()
            && self.cpu_units.is_known()
            && self.gpu_units.is_known()
            && self.network_bytes.is_known()
            && self.telemetry_events.is_known()
            && self.concurrent_executions.is_known()
            && self.execution_time_nanos.is_known()
            && self.compilation_time_nanos.is_known()
            && self.queue_time_nanos.is_known()
            && self.recovery_time_nanos.is_known()
            && self.cost_units.is_known()
            && self.energy_units.is_known()
    }

    /// Returns the number of resources available for a given kind, if known.
    #[must_use]
    pub const fn known_amount(self, resource: ResourceKind) -> Option<u64> {
        self.get(resource).value()
    }

    /// Returns whether the requested demand can be supplied.
    ///
    /// Unknown capacity returns `false`.
    #[must_use]
    pub const fn can_supply(self, demand: ResourceDemand) -> bool {
        self.logical_qubits.can_supply(demand.logical_qubits)
            && self.physical_qubits.can_supply(demand.physical_qubits)
            && self.operations.can_supply(demand.operations)
            && self.circuit_depth.can_supply(demand.circuit_depth)
            && self.shots.can_supply(demand.shots)
            && self.recovery_attempts.can_supply(demand.recovery_attempts)
            && self
                .mitigation_executions
                .can_supply(demand.mitigation_executions)
            && self.checkpoints.can_supply(demand.checkpoints)
            && self.storage_bytes.can_supply(demand.storage_bytes)
            && self.cpu_units.can_supply(demand.cpu_units)
            && self.gpu_units.can_supply(demand.gpu_units)
            && self.network_bytes.can_supply(demand.network_bytes)
            && self.telemetry_events.can_supply(demand.telemetry_events)
            && self
                .concurrent_executions
                .can_supply(demand.concurrent_executions)
            && self
                .execution_time_nanos
                .can_supply(demand.execution_time_nanos)
            && self
                .compilation_time_nanos
                .can_supply(demand.compilation_time_nanos)
            && self
                .queue_time_nanos
                .can_supply(demand.queue_time_nanos)
            && self
                .recovery_time_nanos
                .can_supply(demand.recovery_time_nanos)
            && self.cost_units.can_supply(demand.cost_units)
            && self.energy_units.can_supply(demand.energy_units)
    }

    /// Returns a resource value after reserving the requested amount.
    ///
    /// Returns `None` if the resource is known but insufficient or if integer
    /// arithmetic cannot represent the operation.
    ///
    /// Unknown remains unknown.
    pub const fn reserve(
        self,
        demand: ResourceDemand,
    ) -> Option<Self> {
        if !self.can_supply(demand) {
            return None;
        }

        Some(Self {
            logical_qubits: self.logical_qubits.checked_sub(demand.logical_qubits)?,
            physical_qubits: self.physical_qubits.checked_sub(demand.physical_qubits)?,
            operations: self.operations.checked_sub(demand.operations)?,
            circuit_depth: self.circuit_depth.checked_sub(demand.circuit_depth)?,
            shots: self.shots.checked_sub(demand.shots)?,
            recovery_attempts: self
                .recovery_attempts
                .checked_sub(demand.recovery_attempts)?,
            mitigation_executions: self
                .mitigation_executions
                .checked_sub(demand.mitigation_executions)?,
            checkpoints: self.checkpoints.checked_sub(demand.checkpoints)?,
            storage_bytes: self.storage_bytes.checked_sub(demand.storage_bytes)?,
            cpu_units: self.cpu_units.checked_sub(demand.cpu_units)?,
            gpu_units: self.gpu_units.checked_sub(demand.gpu_units)?,
            network_bytes: self.network_bytes.checked_sub(demand.network_bytes)?,
            telemetry_events: self
                .telemetry_events
                .checked_sub(demand.telemetry_events)?,
            concurrent_executions: self
                .concurrent_executions
                .checked_sub(demand.concurrent_executions)?,
            execution_time_nanos: self
                .execution_time_nanos
                .checked_sub(demand.execution_time_nanos)?,
            compilation_time_nanos: self
                .compilation_time_nanos
                .checked_sub(demand.compilation_time_nanos)?,
            queue_time_nanos: self
                .queue_time_nanos
                .checked_sub(demand.queue_time_nanos)?,
            recovery_time_nanos: self
                .recovery_time_nanos
                .checked_sub(demand.recovery_time_nanos)?,
            cost_units: self.cost_units.checked_sub(demand.cost_units)?,
            energy_units: self.energy_units.checked_sub(demand.energy_units)?,
        })
    }

    /// Returns availability after releasing resources.
    ///
    /// Overflow is reported as `None`.
    pub const fn release(
        self,
        demand: ResourceDemand,
    ) -> Option<Self> {
        Some(Self {
            logical_qubits: self.logical_qubits.checked_add(demand.logical_qubits)?,
            physical_qubits: self.physical_qubits.checked_add(demand.physical_qubits)?,
            operations: self.operations.checked_add(demand.operations)?,
            circuit_depth: self.circuit_depth.checked_add(demand.circuit_depth)?,
            shots: self.shots.checked_add(demand.shots)?,
            recovery_attempts: self
                .recovery_attempts
                .checked_add(demand.recovery_attempts)?,
            mitigation_executions: self
                .mitigation_executions
                .checked_add(demand.mitigation_executions)?,
            checkpoints: self.checkpoints.checked_add(demand.checkpoints)?,
            storage_bytes: self.storage_bytes.checked_add(demand.storage_bytes)?,
            cpu_units: self.cpu_units.checked_add(demand.cpu_units)?,
            gpu_units: self.gpu_units.checked_add(demand.gpu_units)?,
            network_bytes: self.network_bytes.checked_add(demand.network_bytes)?,
            telemetry_events: self
                .telemetry_events
                .checked_add(demand.telemetry_events)?,
            concurrent_executions: self
                .concurrent_executions
                .checked_add(demand.concurrent_executions)?,
            execution_time_nanos: self
                .execution_time_nanos
                .checked_add(demand.execution_time_nanos)?,
            compilation_time_nanos: self
                .compilation_time_nanos
                .checked_add(demand.compilation_time_nanos)?,
            queue_time_nanos: self
                .queue_time_nanos
                .checked_add(demand.queue_time_nanos)?,
            recovery_time_nanos: self
                .recovery_time_nanos
                .checked_add(demand.recovery_time_nanos)?,
            cost_units: self.cost_units.checked_add(demand.cost_units)?,
            energy_units: self.energy_units.checked_add(demand.energy_units)?,
        })
    }
}

impl Default for ResourceAvailability {
    fn default() -> Self {
        Self::unknown()
    }
}


/// Result of comparing one resource demand with current availability.
///
/// This type deliberately distinguishes:
///
///     Satisfied
///     Insufficient
///     Unknown
///
/// so callers cannot accidentally treat incomplete resource information as
/// successful validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceAssessment {
    /// The requested quantity is known to be available.
    Satisfied,

    /// The requested quantity is known to exceed available capacity.
    Insufficient {
        /// Requested quantity.
        requested: u64,

        /// Available quantity.
        available: u64,
    },

    /// Capacity is not known sufficiently to prove feasibility.
    Unknown,
}

impl ResourceAssessment {
    /// Returns true only when availability is proven sufficient.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied)
    }

    /// Returns true when the request is proven impossible.
    #[must_use]
    pub const fn is_insufficient(self) -> bool {
        matches!(self, Self::Insufficient { .. })
    }

    /// Returns true when more information is required.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}


/// Result of validating a complete demand against availability and resilience
/// limits.
///
/// This is intentionally richer than `Result<(), LimitViolation>` because
/// resource validation has two distinct failure domains:
///
///     1. explicit resilience limits;
///     2. actual/current resource availability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceValidation {
    /// The demand satisfies both the supplied limits and known availability.
    Satisfied,

    /// The demand exceeds an explicit resilience limit.
    LimitExceeded(LimitViolation),

    /// The target is known not to have enough of a resource.
    ResourceUnavailable {
        /// Resource category.
        resource: ResourceKind,

        /// Requested amount.
        requested: u64,

        /// Currently available amount.
        available: u64,
    },

    /// Availability is unknown for a required resource.
    ResourceUnknown {
        /// Resource category whose availability is unknown.
        resource: ResourceKind,

        /// Requested amount.
        requested: u64,
    },
}

impl ResourceValidation {
    /// Returns whether the resource request is fully validated.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied)
    }

    /// Returns whether the failure was caused by an explicit limit.
    #[must_use]
    pub const fn is_limit_exceeded(self) -> bool {
        matches!(self, Self::LimitExceeded(_))
    }

    /// Returns whether the target is known to be insufficient.
    #[must_use]
    pub const fn is_resource_unavailable(self) -> bool {
        matches!(self, Self::ResourceUnavailable { .. })
    }

    /// Returns whether additional capability/resource information is required.
    #[must_use]
    pub const fn requires_information(self) -> bool {
        matches!(self, Self::ResourceUnknown { .. })
    }
}

impl fmt::Display for ResourceValidation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Satisfied => formatter.write_str("resource requirements satisfied"),

            Self::LimitExceeded(violation) => {
                write!(formatter, "resilience limit exceeded: {violation}")
            }

            Self::ResourceUnavailable {
                resource,
                requested,
                available,
            } => write!(
                formatter,
                "resource '{}' requires {} but only {} is available",
                resource, requested, available
            ),

            Self::ResourceUnknown {
                resource,
                requested,
            } => write!(
                formatter,
                "resource '{}' requires {} but availability is unknown",
                resource, requested
            ),
        }
    }
}


/// Assesses one resource quantity against availability.
#[must_use]
pub const fn assess_resource(
    availability: ResourceValue,
    requested: u64,
) -> ResourceAssessment {
    match availability {
        ResourceValue::Known(available) if requested <= available => {
            ResourceAssessment::Satisfied
        }

        ResourceValue::Known(available) => ResourceAssessment::Insufficient {
            requested,
            available,
        },

        ResourceValue::Unknown => ResourceAssessment::Unknown,
    }
}


/// Validates a demand against one resource category.
#[must_use]
pub const fn validate_resource(
    resource: ResourceKind,
    requested: u64,
    availability: ResourceValue,
    limits: LimitSet,
) -> ResourceValidation {
    match limits.check(resource, requested) {
        Err(violation) => ResourceValidation::LimitExceeded(violation),
        Ok(()) => match assess_resource(availability, requested) {
            ResourceAssessment::Satisfied => ResourceValidation::Satisfied,

            ResourceAssessment::Insufficient {
                requested,
                available,
            } => ResourceValidation::ResourceUnavailable {
                resource,
                requested,
                available,
            },

            ResourceAssessment::Unknown => {
                ResourceValidation::ResourceUnknown {
                    resource,
                    requested,
                }
            }
        },
    }
}


/// Validates a complete demand against explicit limits and current
/// availability.
///
/// The order is deterministic.
///
/// Explicit resilience limits are checked before actual availability because
/// a caller-supplied limit is a hard constraint even when the target has more
/// resources available.
#[must_use]
pub const fn validate(
    demand: ResourceDemand,
    availability: ResourceAvailability,
    limits: LimitSet,
) -> ResourceValidation {
    macro_rules! validate_dimension {
        ($resource:expr, $demand:expr, $availability:expr) => {
            match validate_resource(
                $resource,
                $demand,
                $availability,
                limits,
            ) {
                ResourceValidation::Satisfied => {}
                other => return other,
            }
        };
    }

    validate_dimension!(
        ResourceKind::LogicalQubits,
        demand.logical_qubits,
        availability.logical_qubits
    );

    validate_dimension!(
        ResourceKind::PhysicalQubits,
        demand.physical_qubits,
        availability.physical_qubits
    );

    validate_dimension!(
        ResourceKind::Operations,
        demand.operations,
        availability.operations
    );

    validate_dimension!(
        ResourceKind::CircuitDepth,
        demand.circuit_depth,
        availability.circuit_depth
    );

    validate_dimension!(
        ResourceKind::Shots,
        demand.shots,
        availability.shots
    );

    validate_dimension!(
        ResourceKind::RecoveryAttempts,
        demand.recovery_attempts,
        availability.recovery_attempts
    );

    validate_dimension!(
        ResourceKind::MitigationExecutions,
        demand.mitigation_executions,
        availability.mitigation_executions
    );

    validate_dimension!(
        ResourceKind::Checkpoints,
        demand.checkpoints,
        availability.checkpoints
    );

    validate_dimension!(
        ResourceKind::StorageBytes,
        demand.storage_bytes,
        availability.storage_bytes
    );

    validate_dimension!(
        ResourceKind::CpuUnits,
        demand.cpu_units,
        availability.cpu_units
    );

    validate_dimension!(
        ResourceKind::GpuUnits,
        demand.gpu_units,
        availability.gpu_units
    );

    validate_dimension!(
        ResourceKind::NetworkBytes,
        demand.network_bytes,
        availability.network_bytes
    );

    validate_dimension!(
        ResourceKind::TelemetryEvents,
        demand.telemetry_events,
        availability.telemetry_events
    );

    validate_dimension!(
        ResourceKind::ConcurrentExecutions,
        demand.concurrent_executions,
        availability.concurrent_executions
    );

    validate_dimension!(
        ResourceKind::ExecutionTimeNanos,
        demand.execution_time_nanos,
        availability.execution_time_nanos
    );

    validate_dimension!(
        ResourceKind::CompilationTimeNanos,
        demand.compilation_time_nanos,
        availability.compilation_time_nanos
    );

    validate_dimension!(
        ResourceKind::QueueTimeNanos,
        demand.queue_time_nanos,
        availability.queue_time_nanos
    );

    validate_dimension!(
        ResourceKind::RecoveryTimeNanos,
        demand.recovery_time_nanos,
        availability.recovery_time_nanos
    );

    validate_dimension!(
        ResourceKind::CostUnits,
        demand.cost_units,
        availability.cost_units
    );

    validate_dimension!(
        ResourceKind::EnergyUnits,
        demand.energy_units,
        availability.energy_units
    );

    ResourceValidation::Satisfied
}


/// A resource reservation.
///
/// A reservation records the resources that have been committed to an
/// operation without owning or manipulating the underlying hardware resource.
///
/// Actual reservation coordination belongs to the execution/coordination
/// layers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceReservation {
    /// Resources committed by the reservation.
    pub demand: ResourceDemand,
}

impl ResourceReservation {
    /// Creates a reservation.
    #[must_use]
    pub const fn new(demand: ResourceDemand) -> Self {
        Self { demand }
    }

    /// Returns true when no resources are reserved.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.demand.is_zero()
    }
}

impl Default for ResourceReservation {
    fn default() -> Self {
        Self {
            demand: ResourceDemand::zero(),
        }
    }
}


/// A deterministic resource ledger operation.
///
/// The ledger is intentionally a pure value operation. It does not perform
/// locking, I/O, distributed coordination, or hardware manipulation.
///
/// Those concerns belong to `coordination` and runtime/execution layers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceLedgerOperation {
    /// Reserve resources.
    Reserve(ResourceDemand),

    /// Release resources.
    Release(ResourceDemand),
}


/// Applies a ledger operation to an availability vector.
///
/// Returns `None` on insufficient capacity or arithmetic overflow.
pub const fn apply_ledger_operation(
    availability: ResourceAvailability,
    operation: ResourceLedgerOperation,
) -> Option<ResourceAvailability> {
    match operation {
        ResourceLedgerOperation::Reserve(demand) => availability.reserve(demand),
        ResourceLedgerOperation::Release(demand) => availability.release(demand),
    }
}


/// Returns the effective resource limit produced by intersecting an explicit
/// resilience limit with known current availability.
///
/// Unknown availability remains unknown at the resource level.
///
/// This helper does not mutate or replace the original `LimitSet`.
#[must_use]
pub const fn effective_limit(
    limit: Limit,
    availability: ResourceValue,
) -> ResourceValue {
    match (limit, availability) {
        (Limit::Bounded(limit), ResourceValue::Known(available)) => {
            ResourceValue::Known(if limit < available {
                limit
            } else {
                available
            })
        }

        (Limit::Bounded(limit), ResourceValue::Unknown) => {
            // The policy limit is known, but actual capacity is not.
            //
            // Returning Unknown is deliberate: the resource system must not
            // claim that the target can provide the policy limit.
            let _ = limit;
            ResourceValue::Unknown
        }

        (Limit::Unlimited, ResourceValue::Known(available)) => {
            ResourceValue::Known(available)
        }

        (Limit::Unlimited, ResourceValue::Unknown) => {
            ResourceValue::Unknown
        }
    }
}


/// Computes the effective available quantity for one resource.
#[must_use]
pub const fn effective_available(
    resource: ResourceKind,
    availability: ResourceAvailability,
    limits: LimitSet,
) -> ResourceValue {
    effective_limit(limits.get(resource), availability.get(resource))
}


/// Returns the effective availability vector produced by intersecting
/// availability with explicit resilience limits.
///
/// Unknown remains unknown.
#[must_use]
pub const fn effective_availability(
    availability: ResourceAvailability,
    limits: LimitSet,
) -> ResourceAvailability {
    ResourceAvailability {
        logical_qubits: effective_limit(
            limits.logical_qubits,
            availability.logical_qubits,
        ),

        physical_qubits: effective_limit(
            limits.physical_qubits,
            availability.physical_qubits,
        ),

        operations: effective_limit(
            limits.operations,
            availability.operations,
        ),

        circuit_depth: effective_limit(
            limits.circuit_depth,
            availability.circuit_depth,
        ),

        shots: effective_limit(
            limits.shots,
            availability.shots,
        ),

        recovery_attempts: effective_limit(
            limits.recovery_attempts,
            availability.recovery_attempts,
        ),

        mitigation_executions: effective_limit(
            limits.mitigation_executions,
            availability.mitigation_executions,
        ),

        checkpoints: effective_limit(
            limits.checkpoints,
            availability.checkpoints,
        ),

        storage_bytes: effective_limit(
            limits.storage_bytes,
            availability.storage_bytes,
        ),

        cpu_units: effective_limit(
            limits.cpu_units,
            availability.cpu_units,
        ),

        gpu_units: effective_limit(
            limits.gpu_units,
            availability.gpu_units,
        ),

        network_bytes: effective_limit(
            limits.network_bytes,
            availability.network_bytes,
        ),

        telemetry_events: effective_limit(
            limits.telemetry_events,
            availability.telemetry_events,
        ),

        concurrent_executions: effective_limit(
            limits.concurrent_executions,
            availability.concurrent_executions,
        ),

        execution_time_nanos: effective_limit(
            limits.execution_time_nanos,
            availability.execution_time_nanos,
        ),

        compilation_time_nanos: effective_limit(
            limits.compilation_time_nanos,
            availability.compilation_time_nanos,
        ),

        queue_time_nanos: effective_limit(
            limits.queue_time_nanos,
            availability.queue_time_nanos,
        ),

        recovery_time_nanos: effective_limit(
            limits.recovery_time_nanos,
            availability.recovery_time_nanos,
        ),

        cost_units: effective_limit(
            limits.cost_units,
            availability.cost_units,
        ),

        energy_units: effective_limit(
            limits.energy_units,
            availability.energy_units,
        ),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_is_not_unlimited() {
        assert!(ResourceValue::Unknown.is_unknown());
        assert!(!ResourceValue::Unknown.can_supply(1));
    }

    #[test]
    fn known_capacity_can_supply_exact_demand() {
        assert!(ResourceValue::Known(10).can_supply(10));
        assert!(ResourceValue::Known(10).can_supply(0));
        assert!(!ResourceValue::Known(10).can_supply(11));
    }

    #[test]
    fn zero_demand_is_empty() {
        assert!(ResourceDemand::zero().is_zero());
    }

    #[test]
    fn demand_addition_is_checked() {
        let left = ResourceDemand::single(
            ResourceKind::Operations,
            10,
        );

        let right = ResourceDemand::single(
            ResourceKind::Operations,
            20,
        );

        let combined = left.checked_add(right).expect("addition should fit");

        assert_eq!(combined.operations, 30);
    }

    #[test]
    fn demand_addition_detects_overflow() {
        let left = ResourceDemand::single(
            ResourceKind::Operations,
            u64::MAX,
        );

        let right = ResourceDemand::single(
            ResourceKind::Operations,
            1,
        );

        assert!(left.checked_add(right).is_none());
    }

    #[test]
    fn unlimited_limits_do_not_create_hardware_capacity() {
        let availability = ResourceAvailability::empty();
        let limits = LimitSet::unlimited();

        let demand = ResourceDemand::single(
            ResourceKind::PhysicalQubits,
            1,
        );

        assert!(!availability.can_supply(demand));
        assert!(matches!(
            validate(demand, availability, limits),
            ResourceValidation::ResourceUnavailable {
                resource: ResourceKind::PhysicalQubits,
                requested: 1,
                available: 0,
            }
        ));
    }

    #[test]
    fn unknown_capacity_is_not_accepted() {
        let availability = ResourceAvailability::unknown();
        let limits = LimitSet::unlimited();

        let demand = ResourceDemand::single(
            ResourceKind::LogicalQubits,
            1,
        );

        assert!(matches!(
            validate(demand, availability, limits),
            ResourceValidation::ResourceUnknown {
                resource: ResourceKind::LogicalQubits,
                requested: 1,
            }
        ));
    }

    #[test]
    fn explicit_limit_is_checked_before_availability() {
        let availability = ResourceAvailability {
            operations: ResourceValue::Known(100),
            ..ResourceAvailability::unknown()
        };

        let limits = LimitSet {
            operations: Limit::Bounded(10),
            ..LimitSet::unlimited()
        };

        let demand = ResourceDemand::single(
            ResourceKind::Operations,
            20,
        );

        assert!(matches!(
            validate(demand, availability, limits),
            ResourceValidation::LimitExceeded(
                LimitViolation {
                    resource: ResourceKind::Operations,
                    requested: 20,
                    limit: Some(10),
                }
            )
        ));
    }

    #[test]
    fn reservation_subtracts_resources() {
        let availability = ResourceAvailability {
            physical_qubits: ResourceValue::Known(100),
            ..ResourceAvailability::unknown()
        };

        let demand = ResourceDemand::single(
            ResourceKind::PhysicalQubits,
            25,
        );

        let remaining = availability
            .reserve(demand)
            .expect("reservation should succeed");

        assert_eq!(
            remaining.physical_qubits,
            ResourceValue::Known(75)
        );
    }

    #[test]
    fn release_restores_resources() {
        let availability = ResourceAvailability {
            physical_qubits: ResourceValue::Known(75),
            ..ResourceAvailability::unknown()
        };

        let demand = ResourceDemand::single(
            ResourceKind::PhysicalQubits,
            25,
        );

        let restored = availability
            .release(demand)
            .expect("release should succeed");

        assert_eq!(
            restored.physical_qubits,
            ResourceValue::Known(100)
        );
    }

    #[test]
    fn effective_capacity_is_intersection() {
        let availability = ResourceAvailability {
            physical_qubits: ResourceValue::Known(100),
            ..ResourceAvailability::unknown()
        };

        let limits = LimitSet {
            physical_qubits: Limit::Bounded(80),
            ..LimitSet::unlimited()
        };

        assert_eq!(
            effective_available(
                ResourceKind::PhysicalQubits,
                availability,
                limits,
            ),
            ResourceValue::Known(80)
        );
    }

    #[test]
    fn unlimited_policy_uses_known_hardware_capacity() {
        let availability = ResourceAvailability {
            physical_qubits: ResourceValue::Known(512),
            ..ResourceAvailability::unknown()
        };

        let limits = LimitSet::unlimited();

        assert_eq!(
            effective_available(
                ResourceKind::PhysicalQubits,
                availability,
                limits,
            ),
            ResourceValue::Known(512)
        );
    }

    #[test]
    fn unknown_hardware_capacity_remains_unknown() {
        let availability = ResourceAvailability::unknown();
        let limits = LimitSet::unlimited();

        assert_eq!(
            effective_available(
                ResourceKind::PhysicalQubits,
                availability,
                limits,
            ),
            ResourceValue::Unknown
        );
    }

    #[test]
    fn zero_is_a_real_requirement() {
        let demand = ResourceDemand::single(
            ResourceKind::PhysicalQubits,
            0,
        );

        let availability = ResourceAvailability {
            physical_qubits: ResourceValue::Known(0),
            ..ResourceAvailability::unknown()
        };

        assert!(matches!(
            validate(
                demand,
                availability,
                LimitSet::unlimited(),
            ),
            ResourceValidation::Satisfied
        ));
    }

    #[test]
    fn resource_kind_mapping_is_consistent() {
        let demand = ResourceDemand {
            logical_qubits: 1,
            physical_qubits: 2,
            operations: 3,
            circuit_depth: 4,
            shots: 5,
            recovery_attempts: 6,
            mitigation_executions: 7,
            checkpoints: 8,
            storage_bytes: 9,
            cpu_units: 10,
            gpu_units: 11,
            network_bytes: 12,
            telemetry_events: 13,
            concurrent_executions: 14,
            execution_time_nanos: 15,
            compilation_time_nanos: 16,
            queue_time_nanos: 17,
            recovery_time_nanos: 18,
            cost_units: 19,
            energy_units: 20,
        };

        assert_eq!(
            demand.get(ResourceKind::LogicalQubits),
            1
        );

        assert_eq!(
            demand.get(ResourceKind::PhysicalQubits),
            2
        );

        assert_eq!(
            demand.get(ResourceKind::Operations),
            3
        );

        assert_eq!(
            demand.get(ResourceKind::EnergyUnits),
            20
        );
    }

    #[test]
    fn effective_availability_never_exceeds_hardware_availability() {
        let availability = ResourceAvailability {
            logical_qubits: ResourceValue::Known(1000),
            physical_qubits: ResourceValue::Known(2000),
            ..ResourceAvailability::unknown()
        };

        let limits = LimitSet {
            logical_qubits: Limit::Bounded(500),
            physical_qubits: Limit::Unlimited,
            ..LimitSet::unlimited()
        };

        let effective = effective_availability(
            availability,
            limits,
        );

        assert_eq!(
            effective.logical_qubits,
            ResourceValue::Known(500)
        );

        assert_eq!(
            effective.physical_qubits,
            ResourceValue::Known(2000)
        );
    }
}