//! Zamani Quantum Resilience — Resource and Execution Limits
//!
//! Path:
//!     src/quantum/resilience/limits/limits.rs
//!
//! Purpose:
//!     Defines provider-independent, machine-size-independent resource limits
//!     used by the quantum resilience subsystem.
//!
//! Design contract:
//!     - Rust 2021
//!     - Rust 1.97 / 1.97.1
//!     - `unsafe` is forbidden
//!     - no fixed quantum-machine size
//!     - no provider-specific assumptions
//!     - no hard-coded retry/qubit/incident ceilings
//!     - no competing quantum identity types
//!     - limits are constraints, not quantum semantics
//!     - `None` means "no limit imposed by this layer"
//!     - concrete environmental limits come from callers, policies,
//!       target capabilities, runtime resources, security policy, or
//!       deployment configuration
//!
//! Architectural ownership:
//!
//!     quantum::ir::qubit
//!         └── owns canonical logical/physical qubit identities
//!
//!     quantum::hardware
//!         └── owns target capabilities and hardware limits
//!
//!     quantum::resilience::policy
//!         └── owns operational policy and budgets
//!
//!     quantum::resilience::limits
//!         └── represents/evaluates explicit resilience limits
//!
//!     quantum::resilience::planning
//!         └── consumes limits when constructing plans
//!
//!     quantum::resilience::recovery
//!         └── consumes limits when executing recovery actions
//!
//!     quantum::resilience::telemetry
//!         └── may use limits for bounded collection/resource usage
//!
//! This module deliberately does not perform routing, scheduling, QEC,
//! optimization, hardware discovery, or recovery itself.
//!
//! -----------------------------------------------------------------------------
//! INVARIANTS
//! -----------------------------------------------------------------------------
//!
//! 1. There is no architectural `MAX_QUBITS`.
//! 2. There is no architectural `MAX_PHYSICAL_QUBITS`.
//! 3. There is no architectural `MAX_RECOVERY_ATTEMPTS`.
//! 4. There is no architectural `MAX_INCIDENTS`.
//! 5. There is no architectural `MAX_MEMORY`.
//! 6. A limit may only constrain an operation when an explicit limit exists.
//! 7. Resource availability is distinct from semantic quantum limits.
//! 8. A limit cannot manufacture a capability that the target does not have.
//! 9. Limits must be composable.
//! 10. Limits must be deterministic when their inputs are deterministic.
//! 11. Arithmetic must detect overflow instead of silently wrapping.
//! 12. Unlimited values are represented explicitly.
//! 13. Zero is a valid explicit limit and means "none permitted".
//! 14. Unknown/unavailable information must not be interpreted as unlimited
//!     when doing safety-critical validation.
//! 15. This module contains no `unsafe` code.
//!
//! -----------------------------------------------------------------------------
//! SCALABILITY MODEL
//! -----------------------------------------------------------------------------
//!
//! "Infinity" in Zamani means:
//!
//!     no artificial ceiling imposed by this module.
//!
//! It does NOT mean:
//!
//!     physically infinite hardware
//!
//! A workload can therefore scale according to the intersection of:
//!
//!     requested workload
//!     ∩ target capability
//!     ∩ caller policy
//!     ∩ execution budget
//!     ∩ security policy
//!     ∩ available runtime resources
//!
//! No quantity in this module is tied to a particular QPU size.
//!
//! -----------------------------------------------------------------------------
//! INTEGRATION
//! -----------------------------------------------------------------------------
//!
//! `limits/resource.rs` should build resource-specific requirements and use
//! these generic limit primitives rather than defining another limit system.
//!
//! `limits/validation.rs` should use `LimitSet::check_*` methods or equivalent
//! comparisons exposed here and should translate failures into the resilience
//! error taxonomy.
//!
//! `policy/budgets.rs` should be the normal source of caller-selected budgets.
//!
//! `hardware::capabilities` should provide target-derived capacity.
//!
//! `planning::feasibility` should combine target capabilities and these limits.
//!
//! `telemetry::collector` may use the bounded limits for resource-safe
//! collection, but must not infer quantum-machine size from telemetry limits.
//!
//! `recovery::*` must consume explicit attempt/time/resource budgets rather than
//! embedding retry counts.
//!
//! `serialization::*` may serialize these values. The representation is kept
//! explicit and stable so that deterministic decision provenance can include
//! the complete limit configuration.
//!
//! -----------------------------------------------------------------------------
//! NOTE ON `QubitId`
//! -----------------------------------------------------------------------------
//!
//! This file intentionally does not need to manipulate qubit IDs directly.
//! When a future limit/resource API needs to identify a quantum resource, it
//! MUST use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! for logical identity and:
//!
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! for physical identity.
//!
//! Do not add `ResilienceQubitId`, `LogicalQubitId`, or another competing
//! quantum identity here.
//!
//! -----------------------------------------------------------------------------
//! NUMERIC POLICY
//! -----------------------------------------------------------------------------
//!
//! Unsigned quantities represent counts/capacities.
//!
//! Signed quantities are intentionally avoided for resource counts.
//!
//! `u64` is used for portable serialized resource counts and durations where a
//! bounded integer representation is appropriate.
//!
//! `usize` is used only when an API must interact with an in-memory collection
//! or allocation whose indexing type is `usize`.
//!
//! Conversion from `u64` to `usize` is therefore checked.
//!
//! No silent integer truncation is permitted.
//! -----------------------------------------------------------------------------


use core::fmt;
use core::num::NonZeroU64;


/// A resource quantity that can either be explicitly bounded or unlimited.
///
/// `Bounded(n)` means the resource may not exceed `n`.
///
/// `Unlimited` means this resilience layer imposes no upper bound.
///
/// This type is intentionally independent of any particular quantum resource.
///
/// # Examples
///
/// ```
/// use crate::quantum::resilience::limits::limits::Limit;
///
/// let bounded = Limit::bounded(128);
/// let unlimited = Limit::unlimited();
///
/// assert!(bounded.is_bounded());
/// assert!(unlimited.is_unlimited());
/// assert!(bounded.allows(128));
/// assert!(!bounded.allows(129));
/// assert!(unlimited.allows(u64::MAX));
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Limit {
    /// An explicit finite upper bound.
    Bounded(u64),

    /// No upper bound is imposed by this layer.
    Unlimited,
}

impl Limit {
    /// Creates a bounded limit.
    ///
    /// Zero is valid and means that the resource is not permitted.
    #[must_use]
    pub const fn bounded(value: u64) -> Self {
        Self::Bounded(value)
    }

    /// Creates an unlimited limit.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self::Unlimited
    }

    /// Returns the configured finite value, if bounded.
    #[must_use]
    pub const fn value(self) -> Option<u64> {
        match self {
            Self::Bounded(value) => Some(value),
            Self::Unlimited => None,
        }
    }

    /// Returns whether this limit is explicitly finite.
    #[must_use]
    pub const fn is_bounded(self) -> bool {
        matches!(self, Self::Bounded(_))
    }

    /// Returns whether this limit is unlimited.
    #[must_use]
    pub const fn is_unlimited(self) -> bool {
        matches!(self, Self::Unlimited)
    }

    /// Returns whether a requested quantity is permitted.
    #[must_use]
    pub const fn allows(self, requested: u64) -> bool {
        match self {
            Self::Bounded(limit) => requested <= limit,
            Self::Unlimited => true,
        }
    }

    /// Returns the smaller of two independently imposed limits.
    ///
    /// This is the correct operation when two constraints must both be
    /// satisfied.
    ///
    /// Examples:
    ///
    ///     policy limit = 100
    ///     runtime limit = 80
    ///     effective limit = 80
    ///
    /// If either side is unlimited, the bounded side wins.
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        match (self, other) {
            (Self::Bounded(a), Self::Bounded(b)) => Self::Bounded(if a < b { a } else { b }),
            (Self::Bounded(a), Self::Unlimited) => Self::Bounded(a),
            (Self::Unlimited, Self::Bounded(b)) => Self::Bounded(b),
            (Self::Unlimited, Self::Unlimited) => Self::Unlimited,
        }
    }

    /// Returns whether this limit is at least as permissive as `other`.
    #[must_use]
    pub const fn permits_limit(self, other: Self) -> bool {
        match (self, other) {
            (Self::Unlimited, _) => true,
            (Self::Bounded(_), Self::Unlimited) => false,
            (Self::Bounded(a), Self::Bounded(b)) => a >= b,
        }
    }

    /// Checks a requested quantity and returns a structured result.
    pub const fn check(self, requested: u64) -> Result<(), LimitViolation> {
        match self {
            Self::Bounded(limit) if requested > limit => Err(LimitViolation {
                resource: ResourceKind::Generic,
                requested,
                limit: Some(limit),
            }),
            Self::Bounded(_) | Self::Unlimited => Ok(()),
        }
    }

    /// Converts a bounded value into `usize` without truncation.
    ///
    /// Returns `None` for `Unlimited` or when the value cannot be represented
    /// by `usize` on the current target.
    #[must_use]
    pub fn as_usize(self) -> Option<usize> {
        match self {
            Self::Bounded(value) => usize::try_from(value).ok(),
            Self::Unlimited => None,
        }
    }

    /// Returns the finite limit as `NonZeroU64` when it is positive.
    #[must_use]
    pub const fn as_non_zero(self) -> Option<NonZeroU64> {
        match self {
            Self::Bounded(value) => NonZeroU64::new(value),
            Self::Unlimited => None,
        }
    }
}

impl Default for Limit {
    /// Default is unlimited because this module must not introduce an
    /// artificial resource ceiling.
    fn default() -> Self {
        Self::Unlimited
    }
}

impl fmt::Display for Limit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bounded(value) => write!(formatter, "{value}"),
            Self::Unlimited => formatter.write_str("unlimited"),
        }
    }
}


/// A named category of resource constrained by the resilience subsystem.
///
/// This is deliberately generic and does not encode a hardware topology or
/// provider-specific resource model.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResourceKind {
    /// Generic resource used when no more specific category is applicable.
    Generic,

    /// Logical qubits required by a workload.
    LogicalQubits,

    /// Physical qubits required by a realization.
    PhysicalQubits,

    /// Quantum operations/instructions.
    Operations,

    /// Circuit depth.
    CircuitDepth,

    /// Shots/samples.
    Shots,

    /// Recovery attempts.
    RecoveryAttempts,

    /// Mitigation executions or equivalent repetitions.
    MitigationExecutions,

    /// Checkpoint count.
    Checkpoints,

    /// Bytes of persistent/transient storage.
    StorageBytes,

    /// CPU work units or explicitly bounded CPU capacity.
    CpuUnits,

    /// GPU work units or explicitly bounded GPU capacity.
    GpuUnits,

    /// Network bytes.
    NetworkBytes,

    /// Telemetry events.
    TelemetryEvents,

    /// Concurrent tasks/executions.
    ConcurrentExecutions,

    /// Execution duration in nanoseconds.
    ExecutionTimeNanos,

    /// Compilation duration in nanoseconds.
    CompilationTimeNanos,

    /// Queue/wait duration in nanoseconds.
    QueueTimeNanos,

    /// Recovery duration in nanoseconds.
    RecoveryTimeNanos,

    /// Cost units supplied by the caller/deployment.
    CostUnits,

    /// Energy units supplied by the target/deployment.
    EnergyUnits,
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Generic => "generic",
            Self::LogicalQubits => "logical_qubits",
            Self::PhysicalQubits => "physical_qubits",
            Self::Operations => "operations",
            Self::CircuitDepth => "circuit_depth",
            Self::Shots => "shots",
            Self::RecoveryAttempts => "recovery_attempts",
            Self::MitigationExecutions => "mitigation_executions",
            Self::Checkpoints => "checkpoints",
            Self::StorageBytes => "storage_bytes",
            Self::CpuUnits => "cpu_units",
            Self::GpuUnits => "gpu_units",
            Self::NetworkBytes => "network_bytes",
            Self::TelemetryEvents => "telemetry_events",
            Self::ConcurrentExecutions => "concurrent_executions",
            Self::ExecutionTimeNanos => "execution_time_nanos",
            Self::CompilationTimeNanos => "compilation_time_nanos",
            Self::QueueTimeNanos => "queue_time_nanos",
            Self::RecoveryTimeNanos => "recovery_time_nanos",
            Self::CostUnits => "cost_units",
            Self::EnergyUnits => "energy_units",
        };

        formatter.write_str(name)
    }
}


/// A structured violation produced when a requested quantity exceeds an
/// explicit limit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LimitViolation {
    /// Resource category that was exceeded.
    pub resource: ResourceKind,

    /// Requested quantity.
    pub requested: u64,

    /// Explicit configured limit.
    pub limit: Option<u64>,
}

impl LimitViolation {
    /// Creates a violation for a specific resource.
    #[must_use]
    pub const fn new(resource: ResourceKind, requested: u64, limit: u64) -> Self {
        Self {
            resource,
            requested,
            limit: Some(limit),
        }
    }

    /// Returns whether the violation is associated with a finite limit.
    #[must_use]
    pub const fn has_finite_limit(self) -> bool {
        self.limit.is_some()
    }
}

impl fmt::Display for LimitViolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.limit {
            Some(limit) => write!(
                formatter,
                "resource '{}' requested {} but limit is {}",
                self.resource, self.requested, limit
            ),
            None => write!(
                formatter,
                "resource '{}' requested {} but no finite limit is available",
                self.resource, self.requested
            ),
        }
    }
}

impl std::error::Error for LimitViolation {}


/// Explicit resilience limits.
///
/// Every field is optional in the sense that `Limit::Unlimited` means this
/// layer imposes no bound.
///
/// This structure represents policy/resource constraints; it does not claim
/// that the target can actually supply unlimited resources.
///
/// Target capability validation MUST still happen elsewhere.
///
/// The fields intentionally cover both quantum and supporting execution
/// resources because resilience can fail through either domain.
///
/// Importantly, these are not machine-size constants. They are values supplied
/// by the current execution context.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LimitSet {
    /// Maximum logical qubits permitted by this constraint set.
    pub logical_qubits: Limit,

    /// Maximum physical qubits permitted by this constraint set.
    pub physical_qubits: Limit,

    /// Maximum number of operations permitted.
    pub operations: Limit,

    /// Maximum circuit depth permitted.
    pub circuit_depth: Limit,

    /// Maximum number of shots/samples permitted.
    pub shots: Limit,

    /// Maximum recovery attempts permitted.
    pub recovery_attempts: Limit,

    /// Maximum mitigation executions permitted.
    pub mitigation_executions: Limit,

    /// Maximum checkpoints permitted.
    pub checkpoints: Limit,

    /// Maximum storage in bytes.
    pub storage_bytes: Limit,

    /// Maximum CPU resource units.
    pub cpu_units: Limit,

    /// Maximum GPU resource units.
    pub gpu_units: Limit,

    /// Maximum network transfer in bytes.
    pub network_bytes: Limit,

    /// Maximum telemetry events.
    pub telemetry_events: Limit,

    /// Maximum concurrently active executions/tasks.
    pub concurrent_executions: Limit,

    /// Maximum total execution time in nanoseconds.
    pub execution_time_nanos: Limit,

    /// Maximum compilation time in nanoseconds.
    pub compilation_time_nanos: Limit,

    /// Maximum queue/wait time in nanoseconds.
    pub queue_time_nanos: Limit,

    /// Maximum recovery time in nanoseconds.
    pub recovery_time_nanos: Limit,

    /// Maximum caller-defined cost units.
    pub cost_units: Limit,

    /// Maximum caller/target-defined energy units.
    pub energy_units: Limit,
}

impl Default for LimitSet {
    /// Creates a limit set with no artificial restrictions.
    fn default() -> Self {
        Self::unlimited()
    }
}

impl LimitSet {
    /// Creates an unrestricted limit set.
    ///
    /// This means only that this layer contributes no additional limit.
    /// It does not override hardware capability, runtime capacity, policy,
    /// security constraints, or operating-system limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            logical_qubits: Limit::Unlimited,
            physical_qubits: Limit::Unlimited,
            operations: Limit::Unlimited,
            circuit_depth: Limit::Unlimited,
            shots: Limit::Unlimited,
            recovery_attempts: Limit::Unlimited,
            mitigation_executions: Limit::Unlimited,
            checkpoints: Limit::Unlimited,
            storage_bytes: Limit::Unlimited,
            cpu_units: Limit::Unlimited,
            gpu_units: Limit::Unlimited,
            network_bytes: Limit::Unlimited,
            telemetry_events: Limit::Unlimited,
            concurrent_executions: Limit::Unlimited,
            execution_time_nanos: Limit::Unlimited,
            compilation_time_nanos: Limit::Unlimited,
            queue_time_nanos: Limit::Unlimited,
            recovery_time_nanos: Limit::Unlimited,
            cost_units: Limit::Unlimited,
            energy_units: Limit::Unlimited,
        }
    }

    /// Returns the effective intersection of two independent limit sets.
    ///
    /// This operation is commutative and associative because each field is
    /// combined using `Limit::intersection`.
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self {
            logical_qubits: self.logical_qubits.intersection(other.logical_qubits),
            physical_qubits: self.physical_qubits.intersection(other.physical_qubits),
            operations: self.operations.intersection(other.operations),
            circuit_depth: self.circuit_depth.intersection(other.circuit_depth),
            shots: self.shots.intersection(other.shots),
            recovery_attempts: self.recovery_attempts.intersection(other.recovery_attempts),
            mitigation_executions: self
                .mitigation_executions
                .intersection(other.mitigation_executions),
            checkpoints: self.checkpoints.intersection(other.checkpoints),
            storage_bytes: self.storage_bytes.intersection(other.storage_bytes),
            cpu_units: self.cpu_units.intersection(other.cpu_units),
            gpu_units: self.gpu_units.intersection(other.gpu_units),
            network_bytes: self.network_bytes.intersection(other.network_bytes),
            telemetry_events: self.telemetry_events.intersection(other.telemetry_events),
            concurrent_executions: self
                .concurrent_executions
                .intersection(other.concurrent_executions),
            execution_time_nanos: self
                .execution_time_nanos
                .intersection(other.execution_time_nanos),
            compilation_time_nanos: self
                .compilation_time_nanos
                .intersection(other.compilation_time_nanos),
            queue_time_nanos: self
                .queue_time_nanos
                .intersection(other.queue_time_nanos),
            recovery_time_nanos: self
                .recovery_time_nanos
                .intersection(other.recovery_time_nanos),
            cost_units: self.cost_units.intersection(other.cost_units),
            energy_units: self.energy_units.intersection(other.energy_units),
        }
    }

    /// Returns the strictest limit among all supplied sets.
    #[must_use]
    pub const fn min(self, other: Self) -> Self {
        self.intersection(other)
    }

    /// Checks logical-qubit demand.
    pub const fn check_logical_qubits(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(
            ResourceKind::LogicalQubits,
            self.logical_qubits,
            requested,
        )
    }

    /// Checks physical-qubit demand.
    pub const fn check_physical_qubits(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(
            ResourceKind::PhysicalQubits,
            self.physical_qubits,
            requested,
        )
    }

    /// Checks operation count.
    pub const fn check_operations(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(ResourceKind::Operations, self.operations, requested)
    }

    /// Checks circuit depth.
    pub const fn check_circuit_depth(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(ResourceKind::CircuitDepth, self.circuit_depth, requested)
    }

    /// Checks shot/sample count.
    pub const fn check_shots(self, requested: u64) -> Result<(), LimitViolation> {
        check(ResourceKind::Shots, self.shots, requested)
    }

    /// Checks recovery attempts.
    pub const fn check_recovery_attempts(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(
            ResourceKind::RecoveryAttempts,
            self.recovery_attempts,
            requested,
        )
    }

    /// Checks mitigation executions.
    pub const fn check_mitigation_executions(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(
            ResourceKind::MitigationExecutions,
            self.mitigation_executions,
            requested,
        )
    }

    /// Checks checkpoint count.
    pub const fn check_checkpoints(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(ResourceKind::Checkpoints, self.checkpoints, requested)
    }

    /// Checks storage bytes.
    pub const fn check_storage_bytes(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(ResourceKind::StorageBytes, self.storage_bytes, requested)
    }

    /// Checks CPU units.
    pub const fn check_cpu_units(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(ResourceKind::CpuUnits, self.cpu_units, requested)
    }

    /// Checks GPU units.
    pub const fn check_gpu_units(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(ResourceKind::GpuUnits, self.gpu_units, requested)
    }

    /// Checks network bytes.
    pub const fn check_network_bytes(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(ResourceKind::NetworkBytes, self.network_bytes, requested)
    }

    /// Checks telemetry events.
    pub const fn check_telemetry_events(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(ResourceKind::TelemetryEvents, self.telemetry_events, requested)
    }

    /// Checks concurrent executions.
    pub const fn check_concurrent_executions(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(
            ResourceKind::ConcurrentExecutions,
            self.concurrent_executions,
            requested,
        )
    }

    /// Checks execution duration in nanoseconds.
    pub const fn check_execution_time_nanos(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(
            ResourceKind::ExecutionTimeNanos,
            self.execution_time_nanos,
            requested,
        )
    }

    /// Checks compilation duration in nanoseconds.
    pub const fn check_compilation_time_nanos(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(
            ResourceKind::CompilationTimeNanos,
            self.compilation_time_nanos,
            requested,
        )
    }

    /// Checks queue/wait duration in nanoseconds.
    pub const fn check_queue_time_nanos(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(
            ResourceKind::QueueTimeNanos,
            self.queue_time_nanos,
            requested,
        )
    }

    /// Checks recovery duration in nanoseconds.
    pub const fn check_recovery_time_nanos(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(
            ResourceKind::RecoveryTimeNanos,
            self.recovery_time_nanos,
            requested,
        )
    }

    /// Checks caller-defined cost units.
    pub const fn check_cost_units(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(ResourceKind::CostUnits, self.cost_units, requested)
    }

    /// Checks caller/target-defined energy units.
    pub const fn check_energy_units(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(ResourceKind::EnergyUnits, self.energy_units, requested)
    }

    /// Returns the limit associated with a resource category.
    #[must_use]
    pub const fn get(self, resource: ResourceKind) -> Limit {
        match resource {
            ResourceKind::Generic => Limit::Unlimited,
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

    /// Returns whether the specified resource demand is allowed.
    #[must_use]
    pub const fn allows(self, resource: ResourceKind, requested: u64) -> bool {
        self.get(resource).allows(requested)
    }

    /// Checks an arbitrary resource category.
    pub const fn check(
        self,
        resource: ResourceKind,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(resource, self.get(resource), requested)
    }
}


/// Performs a common limit check.
const fn check(
    resource: ResourceKind,
    limit: Limit,
    requested: u64,
) -> Result<(), LimitViolation> {
    match limit {
        Limit::Bounded(value) if requested > value => {
            Err(LimitViolation::new(resource, requested, value))
        }
        Limit::Bounded(_) | Limit::Unlimited => Ok(()),
    }
}


/// Combines multiple independently sourced limit sets.
///
/// The effective constraint is the intersection of all supplied constraints.
///
/// This helper exists so callers do not need to duplicate the semantics:
///
///     target capability
///         ∩ deployment resource policy
///         ∩ caller policy
///         ∩ security policy
///         ∩ runtime budget
///
/// No source is considered authoritative for all dimensions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectiveLimits {
    limits: LimitSet,
}

impl EffectiveLimits {
    /// Creates effective limits from one explicit set.
    #[must_use]
    pub const fn new(limits: LimitSet) -> Self {
        Self { limits }
    }

    /// Returns unrestricted effective limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            limits: LimitSet::unlimited(),
        }
    }

    /// Adds another constraint using intersection.
    #[must_use]
    pub const fn intersect(self, other: LimitSet) -> Self {
        Self {
            limits: self.limits.intersection(other),
        }
    }

    /// Returns the resulting limit set.
    #[must_use]
    pub const fn as_limit_set(self) -> LimitSet {
        self.limits
    }

    /// Returns a single effective resource limit.
    #[must_use]
    pub const fn get(self, resource: ResourceKind) -> Limit {
        self.limits.get(resource)
    }

    /// Checks an effective resource constraint.
    pub const fn check(
        self,
        resource: ResourceKind,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        self.limits.check(resource, requested)
    }
}

impl Default for EffectiveLimits {
    fn default() -> Self {
        Self::unlimited()
    }
}


/// A requirement for a resource.
///
/// Unlike `Limit`, which describes an upper bound, `Requirement` describes the
/// amount a workload needs.
///
/// Keeping these concepts separate prevents the common architectural error of
/// confusing "the program needs N" with "the machine supports N".
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Requirement {
    /// Resource category.
    pub resource: ResourceKind,

    /// Requested quantity.
    pub quantity: u64,
}

impl Requirement {
    /// Creates a resource requirement.
    #[must_use]
    pub const fn new(resource: ResourceKind, quantity: u64) -> Self {
        Self {
            resource,
            quantity,
        }
    }

    /// Checks this requirement against a limit.
    pub const fn check(self, limit: Limit) -> Result<(), LimitViolation> {
        check(self.resource, limit, self.quantity)
    }
}


/// A collection of resource requirements.
///
/// The structure is intentionally represented as a fixed semantic set rather
/// than a fixed-size machine representation. It can be extended in future
/// versions without imposing a qubit-count ceiling.
///
/// This type currently contains the most important aggregate requirements
/// directly because those are the dimensions that are frequently evaluated by
/// resilience planning.
///
/// For arbitrary future resources, `Requirement` remains the generic contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceRequirements {
    /// Required logical qubits.
    pub logical_qubits: u64,

    /// Required physical qubits when already known by a lower layer.
    ///
    /// Resilience should normally leave physical realization to routing and
    /// hardware. A value of zero therefore means "no physical requirement
    /// supplied by this layer", not "the program uses zero physical qubits".
    pub physical_qubits: u64,

    /// Required operation count.
    pub operations: u64,

    /// Required circuit depth.
    pub circuit_depth: u64,

    /// Required shots/samples.
    pub shots: u64,

    /// Required storage bytes.
    pub storage_bytes: u64,

    /// Required CPU units.
    pub cpu_units: u64,

    /// Required GPU units.
    pub gpu_units: u64,

    /// Required network bytes.
    pub network_bytes: u64,

    /// Required concurrent executions.
    pub concurrent_executions: u64,

    /// Required execution time in nanoseconds.
    pub execution_time_nanos: u64,

    /// Required compilation time in nanoseconds.
    pub compilation_time_nanos: u64,

    /// Required queue time in nanoseconds.
    pub queue_time_nanos: u64,

    /// Required recovery time in nanoseconds.
    pub recovery_time_nanos: u64,

    /// Required cost units.
    pub cost_units: u64,

    /// Required energy units.
    pub energy_units: u64,
}

impl ResourceRequirements {
    /// Creates empty requirements.
    ///
    /// Empty requirements do not mean that a quantum program contains no
    /// resources. They mean that this particular resilience layer has not
    /// supplied a requirement for those dimensions.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            logical_qubits: 0,
            physical_qubits: 0,
            operations: 0,
            circuit_depth: 0,
            shots: 0,
            storage_bytes: 0,
            cpu_units: 0,
            gpu_units: 0,
            network_bytes: 0,
            concurrent_executions: 0,
            execution_time_nanos: 0,
            compilation_time_nanos: 0,
            queue_time_nanos: 0,
            recovery_time_nanos: 0,
            cost_units: 0,
            energy_units: 0,
        }
    }

    /// Checks every explicitly represented requirement against a limit set.
    ///
    /// The returned error is the first violation in a stable semantic order.
    ///
    /// This ordering is intentional so deterministic planning does not depend
    /// on hash-map iteration order.
    pub const fn check(self, limits: LimitSet) -> Result<(), LimitViolation> {
        macro_rules! check_field {
            ($field:ident, $kind:expr) => {
                match check($kind, limits.$field, self.$field) {
                    Ok(()) => {}
                    Err(error) => return Err(error),
                }
            };
        }

        check_field!(logical_qubits, ResourceKind::LogicalQubits);
        check_field!(physical_qubits, ResourceKind::PhysicalQubits);
        check_field!(operations, ResourceKind::Operations);
        check_field!(circuit_depth, ResourceKind::CircuitDepth);
        check_field!(shots, ResourceKind::Shots);
        check_field!(storage_bytes, ResourceKind::StorageBytes);
        check_field!(cpu_units, ResourceKind::CpuUnits);
        check_field!(gpu_units, ResourceKind::GpuUnits);
        check_field!(network_bytes, ResourceKind::NetworkBytes);
        check_field!(
            concurrent_executions,
            ResourceKind::ConcurrentExecutions
        );
        check_field!(
            execution_time_nanos,
            ResourceKind::ExecutionTimeNanos
        );
        check_field!(
            compilation_time_nanos,
            ResourceKind::CompilationTimeNanos
        );
        check_field!(queue_time_nanos, ResourceKind::QueueTimeNanos);
        check_field!(recovery_time_nanos, ResourceKind::RecoveryTimeNanos);
        check_field!(cost_units, ResourceKind::CostUnits);
        check_field!(energy_units, ResourceKind::EnergyUnits);

        Ok(())
    }
}

impl Default for ResourceRequirements {
    fn default() -> Self {
        Self::empty()
    }
}


/// Checked resource accounting.
///
/// This type is useful for recovery/planning code that progressively consumes
/// a finite budget.
///
/// It never performs wrapping arithmetic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceBudget {
    /// Remaining resource capacity.
    pub remaining: LimitSet,
}

impl ResourceBudget {
    /// Creates a budget.
    #[must_use]
    pub const fn new(limits: LimitSet) -> Self {
        Self { remaining: limits }
    }

    /// Returns an unrestricted budget.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            remaining: LimitSet::unlimited(),
        }
    }

    /// Checks whether a requirement can be consumed.
    pub const fn can_consume(
        self,
        requirements: ResourceRequirements,
    ) -> Result<(), LimitViolation> {
        requirements.check(self.remaining)
    }

    /// Consumes a requirement and returns the remaining budget.
    ///
    /// Unlimited dimensions remain unlimited.
    ///
    /// This operation is checked and cannot underflow.
    pub const fn consume(
        self,
        requirements: ResourceRequirements,
    ) -> Result<Self, LimitViolation> {
        requirements.check(self.remaining)?;

        Ok(Self {
            remaining: LimitSet {
                logical_qubits: subtract_limit(
                    self.remaining.logical_qubits,
                    requirements.logical_qubits,
                ),
                physical_qubits: subtract_limit(
                    self.remaining.physical_qubits,
                    requirements.physical_qubits,
                ),
                operations: subtract_limit(
                    self.remaining.operations,
                    requirements.operations,
                ),
                circuit_depth: subtract_limit(
                    self.remaining.circuit_depth,
                    requirements.circuit_depth,
                ),
                shots: subtract_limit(self.remaining.shots, requirements.shots),
                recovery_attempts: self.remaining.recovery_attempts,
                mitigation_executions: self.remaining.mitigation_executions,
                checkpoints: self.remaining.checkpoints,
                storage_bytes: subtract_limit(
                    self.remaining.storage_bytes,
                    requirements.storage_bytes,
                ),
                cpu_units: subtract_limit(
                    self.remaining.cpu_units,
                    requirements.cpu_units,
                ),
                gpu_units: subtract_limit(
                    self.remaining.gpu_units,
                    requirements.gpu_units,
                ),
                network_bytes: subtract_limit(
                    self.remaining.network_bytes,
                    requirements.network_bytes,
                ),
                telemetry_events: self.remaining.telemetry_events,
                concurrent_executions: subtract_limit(
                    self.remaining.concurrent_executions,
                    requirements.concurrent_executions,
                ),
                execution_time_nanos: subtract_limit(
                    self.remaining.execution_time_nanos,
                    requirements.execution_time_nanos,
                ),
                compilation_time_nanos: subtract_limit(
                    self.remaining.compilation_time_nanos,
                    requirements.compilation_time_nanos,
                ),
                queue_time_nanos: subtract_limit(
                    self.remaining.queue_time_nanos,
                    requirements.queue_time_nanos,
                ),
                recovery_time_nanos: subtract_limit(
                    self.remaining.recovery_time_nanos,
                    requirements.recovery_time_nanos,
                ),
                cost_units: subtract_limit(
                    self.remaining.cost_units,
                    requirements.cost_units,
                ),
                energy_units: subtract_limit(
                    self.remaining.energy_units,
                    requirements.energy_units,
                ),
            },
        })
    }
}


/// Subtracts a consumed quantity from an explicit limit.
///
/// Unlimited remains unlimited.
const fn subtract_limit(limit: Limit, consumed: u64) -> Limit {
    match limit {
        Limit::Bounded(value) => Limit::Bounded(value - consumed),
        Limit::Unlimited => Limit::Unlimited,
    }
}


/// A recovery-specific budget.
///
/// This is kept separate from the generic `ResourceBudget` because recovery
/// attempts and mitigation executions have semantics that should not be
/// confused with physical resource capacity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecoveryBudget {
    /// Maximum recovery attempts.
    pub attempts: Limit,

    /// Maximum recovery time in nanoseconds.
    pub time_nanos: Limit,

    /// Maximum mitigation executions.
    pub mitigation_executions: Limit,

    /// Maximum checkpoint creations.
    pub checkpoints: Limit,
}

impl Default for RecoveryBudget {
    fn default() -> Self {
        Self::unlimited()
    }
}

impl RecoveryBudget {
    /// Creates an unrestricted recovery budget.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            attempts: Limit::Unlimited,
            time_nanos: Limit::Unlimited,
            mitigation_executions: Limit::Unlimited,
            checkpoints: Limit::Unlimited,
        }
    }

    /// Creates a recovery budget.
    #[must_use]
    pub const fn new(
        attempts: Limit,
        time_nanos: Limit,
        mitigation_executions: Limit,
        checkpoints: Limit,
    ) -> Self {
        Self {
            attempts,
            time_nanos,
            mitigation_executions,
            checkpoints,
        }
    }

    /// Checks recovery attempts.
    pub const fn check_attempts(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(ResourceKind::RecoveryAttempts, self.attempts, requested)
    }

    /// Checks recovery time.
    pub const fn check_time(
        self,
        requested_nanos: u64,
    ) -> Result<(), LimitViolation> {
        check(
            ResourceKind::RecoveryTimeNanos,
            self.time_nanos,
            requested_nanos,
        )
    }

    /// Checks mitigation executions.
    pub const fn check_mitigation(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(
            ResourceKind::MitigationExecutions,
            self.mitigation_executions,
            requested,
        )
    }

    /// Checks checkpoints.
    pub const fn check_checkpoints(
        self,
        requested: u64,
    ) -> Result<(), LimitViolation> {
        check(
            ResourceKind::Checkpoints,
            self.checkpoints,
            requested,
        )
    }

    /// Converts the recovery budget into a general limit set.
    #[must_use]
    pub const fn as_limit_set(self) -> LimitSet {
        let mut limits = LimitSet::unlimited();

        limits.recovery_attempts = self.attempts;
        limits.recovery_time_nanos = self.time_nanos;
        limits.mitigation_executions = self.mitigation_executions;
        limits.checkpoints = self.checkpoints;

        limits
    }
}


/// Represents why a concrete limit exists.
///
/// This is useful for provenance and deterministic auditing.
///
/// A limit should not be treated as a machine semantic merely because its
/// source is hardware; the source identifies authority for the constraint.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LimitSource {
    /// Supplied by the target's currently discovered capabilities.
    TargetCapability,

    /// Supplied by caller policy.
    CallerPolicy,

    /// Supplied by deployment configuration.
    Deployment,

    /// Supplied by runtime resource availability.
    RuntimeResource,

    /// Supplied by security policy.
    SecurityPolicy,

    /// Supplied by an execution deadline/budget.
    ExecutionBudget,

    /// Supplied by a provider/backend capability contract.
    BackendCapability,

    /// Supplied by a composition of several sources.
    Composite,
}

impl fmt::Display for LimitSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::TargetCapability => "target_capability",
            Self::CallerPolicy => "caller_policy",
            Self::Deployment => "deployment",
            Self::RuntimeResource => "runtime_resource",
            Self::SecurityPolicy => "security_policy",
            Self::ExecutionBudget => "execution_budget",
            Self::BackendCapability => "backend_capability",
            Self::Composite => "composite",
        };

        formatter.write_str(value)
    }
}


/// An explicitly sourced set of limits.
///
/// The source is metadata; the actual constraint values remain in `LimitSet`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourcedLimits {
    /// Limits contributed by the source.
    pub limits: LimitSet,

    /// Authority from which these limits originate.
    pub source: LimitSource,
}

impl SourcedLimits {
    /// Creates a sourced limit set.
    #[must_use]
    pub const fn new(limits: LimitSet, source: LimitSource) -> Self {
        Self { limits, source }
    }
}


/// Combines independently sourced limits.
///
/// This function is intentionally deterministic: source order does not affect
/// the resulting values because intersection is commutative.
#[must_use]
pub fn combine<I>(sources: I) -> LimitSet
where
    I: IntoIterator<Item = SourcedLimits>,
{
    sources
        .into_iter()
        .fold(LimitSet::unlimited(), |effective, sourced| {
            effective.intersection(sourced.limits)
        })
}


/// Checks that a proposed limit is compatible with a capability limit.
///
/// A resilience policy may tighten a hardware capability, but must never claim
/// more capacity than the target exposes.
///
/// Therefore:
///
///     policy <= capability       valid
///     policy > capability        invalid
///
/// `Unlimited` capability permits any explicit policy limit, but does not mean
/// that the hardware is physically infinite; it means that this capability
/// source did not impose a finite bound.
#[must_use]
pub const fn limit_is_compatible_with_capability(
    proposed: Limit,
    capability: Limit,
) -> bool {
    capability.permits_limit(proposed)
}


/// Validates that a complete proposed limit set does not exceed the target
/// capability set.
///
/// This is a capability compatibility check, not an execution check.
#[must_use]
pub const fn limits_are_compatible_with_capabilities(
    proposed: LimitSet,
    capability: LimitSet,
) -> bool {
    proposed.logical_qubits.permits_limit(capability.logical_qubits)
        && proposed
            .physical_qubits
            .permits_limit(capability.physical_qubits)
        && proposed.operations.permits_limit(capability.operations)
        && proposed.circuit_depth.permits_limit(capability.circuit_depth)
        && proposed.shots.permits_limit(capability.shots)
        && proposed
            .recovery_attempts
            .permits_limit(capability.recovery_attempts)
        && proposed
            .mitigation_executions
            .permits_limit(capability.mitigation_executions)
        && proposed.checkpoints.permits_limit(capability.checkpoints)
        && proposed.storage_bytes.permits_limit(capability.storage_bytes)
        && proposed.cpu_units.permits_limit(capability.cpu_units)
        && proposed.gpu_units.permits_limit(capability.gpu_units)
        && proposed.network_bytes.permits_limit(capability.network_bytes)
        && proposed
            .telemetry_events
            .permits_limit(capability.telemetry_events)
        && proposed
            .concurrent_executions
            .permits_limit(capability.concurrent_executions)
        && proposed
            .execution_time_nanos
            .permits_limit(capability.execution_time_nanos)
        && proposed
            .compilation_time_nanos
            .permits_limit(capability.compilation_time_nanos)
        && proposed
            .queue_time_nanos
            .permits_limit(capability.queue_time_nanos)
        && proposed
            .recovery_time_nanos
            .permits_limit(capability.recovery_time_nanos)
        && proposed.cost_units.permits_limit(capability.cost_units)
        && proposed.energy_units.permits_limit(capability.energy_units)
}


/// A compact, deterministic representation of a limit decision.
///
/// This is useful to feed into provenance/serialization without making
/// operational identifiers part of the decision itself.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LimitDecision {
    /// The requested amount is within the effective limit.
    Allowed,

    /// The requested amount exceeds an explicit finite limit.
    Rejected(LimitViolation),
}

impl LimitDecision {
    /// Evaluates a request.
    #[must_use]
    pub const fn evaluate(
        resource: ResourceKind,
        limit: Limit,
        requested: u64,
    ) -> Self {
        match check(resource, limit, requested) {
            Ok(()) => Self::Allowed,
            Err(error) => Self::Rejected(error),
        }
    }

    /// Returns whether the request was allowed.
    #[must_use]
    pub const fn is_allowed(self) -> bool {
        matches!(self, Self::Allowed)
    }

    /// Returns the violation if rejected.
    #[must_use]
    pub const fn violation(self) -> Option<LimitViolation> {
        match self {
            Self::Allowed => None,
            Self::Rejected(error) => Some(error),
        }
    }
}


/// Converts a resource count into `usize` only when the conversion is safe.
///
/// This helper is intentionally fallible because `usize` is host-width
/// dependent while serialized/resource values are represented by `u64`.
pub fn checked_usize(value: u64) -> Result<usize, ResourceConversionError> {
    usize::try_from(value).map_err(|_| ResourceConversionError::UsizeOverflow {
        value,
    })
}


/// Errors produced when converting resource quantities between representations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceConversionError {
    /// The `u64` value cannot be represented as `usize` on this target.
    UsizeOverflow {
        /// Value that could not be represented.
        value: u64,
    },
}

impl fmt::Display for ResourceConversionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UsizeOverflow { value } => {
                write!(formatter, "resource value {value} cannot fit into usize")
            }
        }
    }
}

impl std::error::Error for ResourceConversionError {}


// -----------------------------------------------------------------------------
// TESTS
// -----------------------------------------------------------------------------
//
// These tests are deliberately local to the contract. Integration tests should
// additionally validate interaction with hardware capabilities, policy,
// planning, recovery, telemetry and serialization.
//
// No test relies on a particular QPU size.
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unlimited_allows_arbitrary_u64_value() {
        let limit = Limit::Unlimited;

        assert!(limit.allows(0));
        assert!(limit.allows(1));
        assert!(limit.allows(u64::MAX));
    }

    #[test]
    fn bounded_limit_is_inclusive() {
        let limit = Limit::bounded(10);

        assert!(limit.allows(0));
        assert!(limit.allows(10));
        assert!(!limit.allows(11));
    }

    #[test]
    fn zero_is_a_valid_limit() {
        let limit = Limit::bounded(0);

        assert!(limit.allows(0));
        assert!(!limit.allows(1));
    }

    #[test]
    fn intersection_is_strictest_bound() {
        assert_eq!(
            Limit::bounded(10).intersection(Limit::bounded(20)),
            Limit::bounded(10)
        );

        assert_eq!(
            Limit::unlimited().intersection(Limit::bounded(20)),
            Limit::bounded(20)
        );

        assert_eq!(
            Limit::unlimited().intersection(Limit::unlimited()),
            Limit::unlimited()
        );
    }

    #[test]
    fn intersection_is_commutative() {
        let a = Limit::bounded(10);
        let b = Limit::bounded(20);

        assert_eq!(a.intersection(b), b.intersection(a));
    }

    #[test]
    fn limit_set_default_has_no_artificial_ceiling() {
        let limits = LimitSet::default();

        assert!(limits.logical_qubits.is_unlimited());
        assert!(limits.physical_qubits.is_unlimited());
        assert!(limits.operations.is_unlimited());
        assert!(limits.recovery_attempts.is_unlimited());
    }

    #[test]
    fn limit_set_checks_logical_qubits() {
        let limits = LimitSet {
            logical_qubits: Limit::bounded(100),
            ..LimitSet::unlimited()
        };

        assert!(limits.check_logical_qubits(100).is_ok());
        assert!(limits.check_logical_qubits(101).is_err());
    }

    #[test]
    fn limit_set_checks_physical_qubits() {
        let limits = LimitSet {
            physical_qubits: Limit::bounded(200),
            ..LimitSet::unlimited()
        };

        assert!(limits.check_physical_qubits(200).is_ok());
        assert!(limits.check_physical_qubits(201).is_err());
    }

    #[test]
    fn limit_set_intersection_is_deterministic() {
        let a = LimitSet {
            logical_qubits: Limit::bounded(100),
            operations: Limit::bounded(10_000),
            ..LimitSet::unlimited()
        };

        let b = LimitSet {
            logical_qubits: Limit::bounded(80),
            operations: Limit::bounded(20_000),
            ..LimitSet::unlimited()
        };

        let first = a.intersection(b);
        let second = b.intersection(a);

        assert_eq!(first, second);
        assert_eq!(first.logical_qubits, Limit::bounded(80));
        assert_eq!(first.operations, Limit::bounded(10_000));
    }

    #[test]
    fn requirements_are_checked_in_stable_order() {
        let limits = LimitSet {
            logical_qubits: Limit::bounded(2),
            operations: Limit::bounded(3),
            ..LimitSet::unlimited()
        };

        let requirements = ResourceRequirements {
            logical_qubits: 3,
            operations: 4,
            ..ResourceRequirements::empty()
        };

        let violation = requirements.check(limits).expect_err("must fail");

        assert_eq!(violation.resource, ResourceKind::LogicalQubits);
        assert_eq!(violation.requested, 3);
        assert_eq!(violation.limit, Some(2));
    }

    #[test]
    fn requirements_pass_when_all_limits_allow_them() {
        let limits = LimitSet {
            logical_qubits: Limit::bounded(100),
            physical_qubits: Limit::bounded(200),
            operations: Limit::bounded(1_000),
            circuit_depth: Limit::bounded(500),
            shots: Limit::bounded(10_000),
            ..LimitSet::unlimited()
        };

        let requirements = ResourceRequirements {
            logical_qubits: 10,
            physical_qubits: 20,
            operations: 100,
            circuit_depth: 100,
            shots: 1_000,
            ..ResourceRequirements::empty()
        };

        assert!(requirements.check(limits).is_ok());
    }

    #[test]
    fn resource_budget_consumes_without_underflow() {
        let budget = ResourceBudget::new(LimitSet {
            operations: Limit::bounded(100),
            ..LimitSet::unlimited()
        });

        let requirements = ResourceRequirements {
            operations: 40,
            ..ResourceRequirements::empty()
        };

        let remaining = budget.consume(requirements).expect("must fit");

        assert_eq!(
            remaining.remaining.operations,
            Limit::bounded(60)
        );
    }

    #[test]
    fn resource_budget_rejects_overconsumption() {
        let budget = ResourceBudget::new(LimitSet {
            operations: Limit::bounded(100),
            ..LimitSet::unlimited()
        });

        let requirements = ResourceRequirements {
            operations: 101,
            ..ResourceRequirements::empty()
        };

        assert!(budget.consume(requirements).is_err());
    }

    #[test]
    fn unlimited_budget_stays_unlimited_after_consumption() {
        let budget = ResourceBudget::unlimited();

        let requirements = ResourceRequirements {
            operations: u64::MAX,
            logical_qubits: u64::MAX,
            ..ResourceRequirements::empty()
        };

        let remaining = budget.consume(requirements).expect("unlimited");

        assert!(remaining.remaining.operations.is_unlimited());
        assert!(remaining.remaining.logical_qubits.is_unlimited());
    }

    #[test]
    fn recovery_budget_has_no_implicit_retry_count() {
        let budget = RecoveryBudget::default();

        assert!(budget.attempts.is_unlimited());
        assert!(budget.time_nanos.is_unlimited());
    }

    #[test]
    fn proposed_limit_may_tighten_capability() {
        let capability = Limit::bounded(100);
        let proposed = Limit::bounded(50);

        assert!(limit_is_compatible_with_capability(
            proposed,
            capability
        ));
    }

    #[test]
    fn proposed_limit_must_not_exceed_capability() {
        let capability = Limit::bounded(100);
        let proposed = Limit::bounded(101);

        assert!(!limit_is_compatible_with_capability(
            proposed,
            capability
        ));
    }

    #[test]
    fn unlimited_proposed_limit_requires_unlimited_capability() {
        assert!(!limit_is_compatible_with_capability(
            Limit::Unlimited,
            Limit::bounded(100)
        ));

        assert!(limit_is_compatible_with_capability(
            Limit::Unlimited,
            Limit::Unlimited
        ));
    }

    #[test]
    fn explicit_policy_does_not_create_hardware_capacity() {
        let capability = LimitSet {
            logical_qubits: Limit::bounded(50),
            ..LimitSet::unlimited()
        };

        let proposed = LimitSet {
            logical_qubits: Limit::bounded(100),
            ..LimitSet::unlimited()
        };

        assert!(!limits_are_compatible_with_capabilities(
            proposed,
            capability
        ));
    }

    #[test]
    fn effective_limits_are_intersection_of_sources() {
        let capability = SourcedLimits::new(
            LimitSet {
                logical_qubits: Limit::bounded(100),
                operations: Limit::bounded(10_000),
                ..LimitSet::unlimited()
            },
            LimitSource::TargetCapability,
        );

        let policy = SourcedLimits::new(
            LimitSet {
                logical_qubits: Limit::bounded(80),
                operations: Limit::bounded(5_000),
                ..LimitSet::unlimited()
            },
            LimitSource::CallerPolicy,
        );

        let runtime = SourcedLimits::new(
            LimitSet {
                logical_qubits: Limit::bounded(90),
                operations: Limit::bounded(8_000),
                ..LimitSet::unlimited()
            },
            LimitSource::RuntimeResource,
        );

        let effective = combine([capability, policy, runtime]);

        assert_eq!(
            effective.logical_qubits,
            Limit::bounded(80)
        );
        assert_eq!(
            effective.operations,
            Limit::bounded(5_000)
        );
    }

    #[test]
    fn combine_is_order_independent() {
        let a = SourcedLimits::new(
            LimitSet {
                logical_qubits: Limit::bounded(100),
                ..LimitSet::unlimited()
            },
            LimitSource::TargetCapability,
        );

        let b = SourcedLimits::new(
            LimitSet {
                logical_qubits: Limit::bounded(50),
                ..LimitSet::unlimited()
            },
            LimitSource::CallerPolicy,
        );

        let first = combine([a, b]);
        let second = combine([b, a]);

        assert_eq!(first, second);
    }

    #[test]
    fn decision_is_deterministic() {
        let first = LimitDecision::evaluate(
            ResourceKind::LogicalQubits,
            Limit::bounded(10),
            11,
        );

        let second = LimitDecision::evaluate(
            ResourceKind::LogicalQubits,
            Limit::bounded(10),
            11,
        );

        assert_eq!(first, second);
    }

    #[test]
    fn checked_usize_conversion_accepts_valid_values() {
        assert_eq!(checked_usize(0).expect("valid"), 0);
        assert_eq!(checked_usize(1).expect("valid"), 1);
    }

    #[test]
    fn limit_as_usize_is_none_for_unlimited() {
        assert_eq!(Limit::Unlimited.as_usize(), None);
    }

    #[test]
    fn limit_as_non_zero_rejects_zero() {
        assert_eq!(Limit::bounded(0).as_non_zero(), None);
        assert_eq!(
            Limit::bounded(1).as_non_zero(),
            NonZeroU64::new(1)
        );
    }

    #[test]
    fn resource_kind_display_is_stable() {
        assert_eq!(
            ResourceKind::LogicalQubits.to_string(),
            "logical_qubits"
        );
        assert_eq!(
            ResourceKind::PhysicalQubits.to_string(),
            "physical_qubits"
        );
        assert_eq!(
            ResourceKind::RecoveryAttempts.to_string(),
            "recovery_attempts"
        );
    }
}