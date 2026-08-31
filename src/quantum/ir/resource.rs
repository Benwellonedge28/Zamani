//! Universal quantum resource requirements for the Zamani Quantum IR.
//!
//! # Architectural role
//!
//! This module describes the resources a quantum program, region, operation,
//! pulse program, logical computation, or execution plan requires.
//!
//! It does NOT:
//! - discover hardware;
//! - allocate hardware;
//! - perform qubit routing;
//! - perform scheduling;
//! - select a backend;
//! - inspect calibration data;
//! - execute a program;
//! - decide whether a particular QPU exists.
//!
//! Those responsibilities belong to the corresponding layers under
//! `quantum::hardware`, `quantum::routing`, `quantum::scheduling`, and
//! backend/runtime infrastructure.
//!
//! The resource layer answers one question:
//!
//!     "What resources does this IR object require?"
//!
//! # Scalability
//!
//! Zamani Quantum IR has no architectural fixed qubit-count ceiling.
//!
//! A resource requirement may be:
//!
//! - exactly N;
//! - at least N;
//! - at most N;
//! - bounded between two values;
//! - unbounded.
//!
//! "Unbounded" is represented explicitly rather than by using `usize::MAX`
//! or another sentinel. This prevents an implementation limit from becoming
//! a semantic quantum-machine limit.
//!
//! Concrete safety/resource policies are supplied by higher layers such as
//! `quantum::ir::limits`.
//!
//! # Dependency boundary
//!
//! This file may depend on canonical identity types from the IR, especially
//! `quantum::ir::qubit::{QubitId, PhysicalQubitId}`.
//!
//! It must not depend on hardware topology, routing, scheduling, optimization,
//! simulation, frontend parsing, QEC implementation, or backend execution.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.
//!
//! All arithmetic that can overflow is checked.
//!
//! # Rust compatibility
//!
//! Designed for Rust 1.97 / Rust 1.97.1.

use std::fmt;

use super::qubit::{PhysicalQubitId, QubitId};

/// A non-negative resource quantity.
///
/// `Unbounded` means that the semantic requirement has no finite upper bound.
/// It is intentionally different from a very large finite integer.
///
/// This is critical for the Zamani "write once, scale everywhere" model:
/// `usize::MAX` is an implementation value; `Unbounded` is an IR meaning.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ResourceQuantity {
    /// A finite resource quantity.
    Finite(u64),

    /// No finite upper bound exists.
    Unbounded,
}

impl ResourceQuantity {
    /// Creates a finite quantity.
    #[must_use]
    pub const fn finite(value: u64) -> Self {
        Self::Finite(value)
    }

    /// Creates an unbounded quantity.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self::Unbounded
    }

    /// Returns `true` when this quantity is unbounded.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        matches!(self, Self::Unbounded)
    }

    /// Returns the finite value, if one exists.
    #[must_use]
    pub const fn as_finite(self) -> Option<u64> {
        match self {
            Self::Finite(value) => Some(value),
            Self::Unbounded => None,
        }
    }

    /// Checked addition.
    ///
    /// Any finite overflow is reported. An unbounded operand produces an
    /// unbounded result.
    pub const fn checked_add(self, rhs: Self) -> Result<Self, ResourceError> {
        match (self, rhs) {
            (Self::Unbounded, _) | (_, Self::Unbounded) => Ok(Self::Unbounded),
            (Self::Finite(lhs), Self::Finite(rhs)) => match lhs.checked_add(rhs) {
                Some(value) => Ok(Self::Finite(value)),
                None => Err(ResourceError::ArithmeticOverflow),
            },
        }
    }

    /// Checked subtraction.
    ///
    /// Subtracting a larger finite value is rejected instead of wrapping.
    ///
    /// An unbounded right-hand side cannot be subtracted from a finite
    /// quantity because the result cannot be represented meaningfully as a
    /// non-negative resource quantity.
    pub const fn checked_sub(self, rhs: Self) -> Result<Self, ResourceError> {
        match (self, rhs) {
            (Self::Unbounded, Self::Finite(_)) => Ok(Self::Unbounded),
            (Self::Unbounded, Self::Unbounded) => Err(ResourceError::IndeterminateOperation),
            (Self::Finite(_), Self::Unbounded) => Err(ResourceError::InsufficientQuantity),
            (Self::Finite(lhs), Self::Finite(rhs)) => match lhs.checked_sub(rhs) {
                Some(value) => Ok(Self::Finite(value)),
                None => Err(ResourceError::InsufficientQuantity),
            },
        }
    }

    /// Checked multiplication by a finite factor.
    pub const fn checked_mul(self, factor: u64) -> Result<Self, ResourceError> {
        match self {
            Self::Unbounded => Ok(Self::Unbounded),
            Self::Finite(value) => match value.checked_mul(factor) {
                Some(result) => Ok(Self::Finite(result)),
                None => Err(ResourceError::ArithmeticOverflow),
            },
        }
    }

    /// Returns whether this quantity is greater than the supplied finite
    /// value.
    #[must_use]
    pub const fn exceeds(self, value: u64) -> bool {
        match self {
            Self::Finite(quantity) => quantity > value,
            Self::Unbounded => true,
        }
    }

    /// Returns whether this quantity is at least the supplied finite value.
    #[must_use]
    pub const fn at_least(self, value: u64) -> bool {
        match self {
            Self::Finite(quantity) => quantity >= value,
            Self::Unbounded => true,
        }
    }

    /// Returns the maximum of two quantities.
    ///
    /// `Unbounded` dominates because it has no finite upper bound.
    #[must_use]
    pub const fn max(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Unbounded, _) | (_, Self::Unbounded) => Self::Unbounded,
            (Self::Finite(lhs), Self::Finite(rhs)) => Self::Finite(if lhs >= rhs {
                lhs
            } else {
                rhs
            }),
        }
    }

    /// Returns the minimum of two quantities.
    ///
    /// An unbounded quantity does not constrain the minimum.
    #[must_use]
    pub const fn min(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Unbounded, finite) | (finite, Self::Unbounded) => finite,
            (Self::Finite(lhs), Self::Finite(rhs)) => Self::Finite(if lhs <= rhs {
                lhs
            } else {
                rhs
            }),
        }
    }
}

impl Default for ResourceQuantity {
    fn default() -> Self {
        Self::Finite(0)
    }
}

impl fmt::Display for ResourceQuantity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Finite(value) => write!(formatter, "{value}"),
            Self::Unbounded => formatter.write_str("unbounded"),
        }
    }
}

/// A closed interval describing an acceptable resource quantity.
///
/// `min <= actual <= max` when both bounds are finite.
///
/// `max = Unbounded` represents an open upper side:
///
///     min <= actual
///
/// This type is useful for resource requirements whose exact amount is not
/// known until compilation, specialization, routing, scheduling, or runtime.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ResourceRange {
    min: u64,
    max: ResourceQuantity,
}

impl ResourceRange {
    /// Creates an exact finite requirement.
    #[must_use]
    pub const fn exact(value: u64) -> Self {
        Self {
            min: value,
            max: ResourceQuantity::Finite(value),
        }
    }

    /// Creates a minimum-only requirement.
    #[must_use]
    pub const fn at_least(min: u64) -> Self {
        Self {
            min,
            max: ResourceQuantity::Unbounded,
        }
    }

    /// Creates a finite inclusive range.
    ///
    /// Returns an error if `min > max`.
    pub const fn between(min: u64, max: u64) -> Result<Self, ResourceError> {
        if min > max {
            return Err(ResourceError::InvalidRange);
        }

        Ok(Self {
            min,
            max: ResourceQuantity::Finite(max),
        })
    }

    /// Returns the minimum required quantity.
    #[must_use]
    pub const fn min(self) -> u64 {
        self.min
    }

    /// Returns the maximum required quantity.
    #[must_use]
    pub const fn max(self) -> ResourceQuantity {
        self.max
    }

    /// Returns `true` when the range has no finite upper bound.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        self.max.is_unbounded()
    }

    /// Returns whether a finite capacity satisfies the range.
    #[must_use]
    pub const fn accepts(self, capacity: u64) -> bool {
        if capacity < self.min {
            return false;
        }

        match self.max {
            ResourceQuantity::Finite(max) => capacity <= max,
            ResourceQuantity::Unbounded => true,
        }
    }

    /// Intersects two ranges.
    ///
    /// Returns an error if their intersection is empty.
    pub const fn intersect(self, other: Self) -> Result<Self, ResourceError> {
        let min = if self.min >= other.min {
            self.min
        } else {
            other.min
        };

        let max = self.max.min(other.max);

        match max {
            ResourceQuantity::Finite(maximum) => {
                if min > maximum {
                    Err(ResourceError::EmptyIntersection)
                } else {
                    Ok(Self {
                        min,
                        max: ResourceQuantity::Finite(maximum),
                    })
                }
            }
            ResourceQuantity::Unbounded => Ok(Self {
                min,
                max: ResourceQuantity::Unbounded,
            }),
        }
    }
}

impl Default for ResourceRange {
    fn default() -> Self {
        Self::exact(0)
    }
}

/// Classification of a quantum resource.
///
/// The enum is intentionally technology-neutral. It describes what a
/// computation requires without claiming how a particular machine implements
/// it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    /// Logical qubits.
    LogicalQubits,

    /// Physical qubits.
    PhysicalQubits,

    /// Classical bits/register storage.
    ClassicalBits,

    /// Generic classical integer/word resources.
    ClassicalWords,

    /// Quantum operations.
    QuantumOperations,

    /// Gate operations.
    GateOperations,

    /// Two-or-more-qubit operations.
    MultiQubitOperations,

    /// Measurement operations.
    Measurements,

    /// Mid-circuit/dynamic measurements.
    MidCircuitMeasurements,

    /// Pulse operations.
    PulseOperations,

    /// Waveforms.
    Waveforms,

    /// Control channels.
    Channels,

    /// Pulse frames.
    Frames,

    /// Schedule slots/events.
    ScheduleEvents,

    /// Program parameters.
    Parameters,

    /// Control-flow nesting.
    ControlFlowDepth,

    /// Circuit/program depth.
    CircuitDepth,

    /// Total semantic execution duration.
    ExecutionTime,

    /// Classical memory bytes.
    ClassicalMemoryBytes,

    /// Waveform/sample storage bytes.
    WaveformMemoryBytes,

    /// Program/IR serialized bytes.
    ProgramBytes,

    /// Generic backend-independent memory.
    MemoryBytes,

    /// Technology-specific resource.
    Custom,
}

impl ResourceKind {
    /// Returns a stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LogicalQubits => "logical_qubits",
            Self::PhysicalQubits => "physical_qubits",
            Self::ClassicalBits => "classical_bits",
            Self::ClassicalWords => "classical_words",
            Self::QuantumOperations => "quantum_operations",
            Self::GateOperations => "gate_operations",
            Self::MultiQubitOperations => "multi_qubit_operations",
            Self::Measurements => "measurements",
            Self::MidCircuitMeasurements => "mid_circuit_measurements",
            Self::PulseOperations => "pulse_operations",
            Self::Waveforms => "waveforms",
            Self::Channels => "channels",
            Self::Frames => "frames",
            Self::ScheduleEvents => "schedule_events",
            Self::Parameters => "parameters",
            Self::ControlFlowDepth => "control_flow_depth",
            Self::CircuitDepth => "circuit_depth",
            Self::ExecutionTime => "execution_time",
            Self::ClassicalMemoryBytes => "classical_memory_bytes",
            Self::WaveformMemoryBytes => "waveform_memory_bytes",
            Self::ProgramBytes => "program_bytes",
            Self::MemoryBytes => "memory_bytes",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A technology-neutral resource requirement.
///
/// The optional label allows a requirement to distinguish resources within
/// the same broad class without forcing hardware-specific semantics into the
/// IR.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResourceRequirement {
    kind: ResourceKind,
    range: ResourceRange,
    label: Option<String>,
}

impl ResourceRequirement {
    /// Creates an exact resource requirement.
    #[must_use]
    pub fn exact(kind: ResourceKind, amount: u64) -> Self {
        Self {
            kind,
            range: ResourceRange::exact(amount),
            label: None,
        }
    }

    /// Creates a minimum resource requirement.
    #[must_use]
    pub fn at_least(kind: ResourceKind, amount: u64) -> Self {
        Self {
            kind,
            range: ResourceRange::at_least(amount),
            label: None,
        }
    }

    /// Creates a bounded resource requirement.
    pub fn between(
        kind: ResourceKind,
        min: u64,
        max: u64,
    ) -> Result<Self, ResourceError> {
        Ok(Self {
            kind,
            range: ResourceRange::between(min, max)?,
            label: None,
        })
    }

    /// Adds a semantic label.
    ///
    /// Labels are descriptive and must not be interpreted as hardware
    /// identifiers.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Returns the resource kind.
    #[must_use]
    pub const fn kind(&self) -> ResourceKind {
        self.kind
    }

    /// Returns the accepted range.
    #[must_use]
    pub const fn range(&self) -> ResourceRange {
        self.range
    }

    /// Returns the optional semantic label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Returns whether a finite capacity satisfies this requirement.
    #[must_use]
    pub const fn accepts(&self, capacity: u64) -> bool {
        self.range.accepts(capacity)
    }
}

/// A logical-to-physical resource association.
///
/// This is intentionally only a declaration/reference. It does not perform
/// routing or allocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct QubitResourceBinding {
    logical: QubitId,
    physical: PhysicalQubitId,
}

impl QubitResourceBinding {
    /// Creates a logical/physical association.
    #[must_use]
    pub const fn new(logical: QubitId, physical: PhysicalQubitId) -> Self {
        Self { logical, physical }
    }

    /// Returns the logical qubit.
    #[must_use]
    pub const fn logical(self) -> QubitId {
        self.logical
    }

    /// Returns the physical qubit.
    #[must_use]
    pub const fn physical(self) -> PhysicalQubitId {
        self.physical
    }
}

/// A collection of logical qubit references required by an IR object.
///
/// The collection contains identities only. It does not imply allocation,
/// placement, topology, or routing.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LogicalQubitResources {
    qubits: Vec<QubitId>,
}

impl LogicalQubitResources {
    /// Creates an empty resource set.
    #[must_use]
    pub const fn new() -> Self {
        Self { qubits: Vec::new() }
    }

    /// Creates a resource set from an existing vector.
    ///
    /// The caller is responsible for avoiding duplicates.
    #[must_use]
    pub fn from_vec(qubits: Vec<QubitId>) -> Self {
        Self { qubits }
    }

    /// Adds one logical qubit.
    ///
    /// Duplicate identities are rejected.
    pub fn insert(&mut self, qubit: QubitId) -> Result<(), ResourceError> {
        if self.qubits.iter().any(|existing| *existing == qubit) {
            return Err(ResourceError::DuplicateQubit);
        }

        self.qubits.push(qubit);
        Ok(())
    }

    /// Returns the number of explicitly represented logical qubits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.qubits.len()
    }

    /// Returns whether no logical qubits are explicitly represented.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.qubits.is_empty()
    }

    /// Returns an iterator over logical qubits.
    pub fn iter(&self) -> impl Iterator<Item = &QubitId> {
        self.qubits.iter()
    }

    /// Returns the backing slice.
    #[must_use]
    pub fn as_slice(&self) -> &[QubitId] {
        &self.qubits
    }

    /// Consumes the set and returns its vector.
    #[must_use]
    pub fn into_vec(self) -> Vec<QubitId> {
        self.qubits
    }
}

/// A collection of explicit physical qubit references.
///
/// This does not imply that the physical qubits exist on a target machine.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PhysicalQubitResources {
    qubits: Vec<PhysicalQubitId>,
}

impl PhysicalQubitResources {
    /// Creates an empty resource set.
    #[must_use]
    pub const fn new() -> Self {
        Self { qubits: Vec::new() }
    }

    /// Creates a resource set from a vector.
    ///
    /// The caller is responsible for avoiding duplicates.
    #[must_use]
    pub fn from_vec(qubits: Vec<PhysicalQubitId>) -> Self {
        Self { qubits }
    }

    /// Adds one physical qubit.
    ///
    /// Duplicate identities are rejected.
    pub fn insert(&mut self, qubit: PhysicalQubitId) -> Result<(), ResourceError> {
        if self.qubits.iter().any(|existing| *existing == qubit) {
            return Err(ResourceError::DuplicateQubit);
        }

        self.qubits.push(qubit);
        Ok(())
    }

    /// Returns the number of explicitly represented physical qubits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.qubits.len()
    }

    /// Returns whether no physical qubits are explicitly represented.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.qubits.is_empty()
    }

    /// Returns an iterator over physical qubits.
    pub fn iter(&self) -> impl Iterator<Item = &PhysicalQubitId> {
        self.qubits.iter()
    }

    /// Returns the backing slice.
    #[must_use]
    pub fn as_slice(&self) -> &[PhysicalQubitId] {
        &self.qubits
    }

    /// Consumes the set and returns its vector.
    #[must_use]
    pub fn into_vec(self) -> Vec<PhysicalQubitId> {
        self.qubits
    }
}

/// Resource requirements for a quantum program or subprogram.
///
/// This structure intentionally separates:
///
/// 1. scalar requirements;
/// 2. explicit logical-qubit references;
/// 3. explicit physical-qubit references;
/// 4. logical-to-physical associations.
///
/// This lets small programs remain inexpensive while large programs can
/// express requirements without inventing fixed-size machine limits.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct QuantumResourceRequirements {
    requirements: Vec<ResourceRequirement>,
    logical_qubits: LogicalQubitResources,
    physical_qubits: PhysicalQubitResources,
    bindings: Vec<QubitResourceBinding>,
}

impl QuantumResourceRequirements {
    /// Creates an empty requirement set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            requirements: Vec::new(),
            logical_qubits: LogicalQubitResources::new(),
            physical_qubits: PhysicalQubitResources::new(),
            bindings: Vec::new(),
        }
    }

    /// Adds a scalar resource requirement.
    pub fn add_requirement(
        &mut self,
        requirement: ResourceRequirement,
    ) -> Result<(), ResourceError> {
        self.requirements.push(requirement);
        Ok(())
    }

    /// Adds an exact requirement.
    pub fn require_exact(
        &mut self,
        kind: ResourceKind,
        amount: u64,
    ) -> Result<(), ResourceError> {
        self.add_requirement(ResourceRequirement::exact(kind, amount))
    }

    /// Adds a minimum requirement.
    pub fn require_at_least(
        &mut self,
        kind: ResourceKind,
        amount: u64,
    ) -> Result<(), ResourceError> {
        self.add_requirement(ResourceRequirement::at_least(kind, amount))
    }

    /// Adds a finite range requirement.
    pub fn require_between(
        &mut self,
        kind: ResourceKind,
        min: u64,
        max: u64,
    ) -> Result<(), ResourceError> {
        self.add_requirement(ResourceRequirement::between(kind, min, max)?)
    }

    /// Adds a logical qubit reference.
    pub fn add_logical_qubit(
        &mut self,
        qubit: QubitId,
    ) -> Result<(), ResourceError> {
        self.logical_qubits.insert(qubit)
    }

    /// Adds a physical qubit reference.
    pub fn add_physical_qubit(
        &mut self,
        qubit: PhysicalQubitId,
    ) -> Result<(), ResourceError> {
        self.physical_qubits.insert(qubit)
    }

    /// Adds a logical-to-physical association.
    ///
    /// This records an already-known mapping. It does not calculate one.
    pub fn add_binding(
        &mut self,
        binding: QubitResourceBinding,
    ) -> Result<(), ResourceError> {
        if self.bindings.iter().any(|existing| {
            existing.logical() == binding.logical()
                || existing.physical() == binding.physical()
        }) {
            return Err(ResourceError::ConflictingQubitBinding);
        }

        self.bindings.push(binding);
        Ok(())
    }

    /// Returns scalar requirements.
    #[must_use]
    pub fn requirements(&self) -> &[ResourceRequirement] {
        &self.requirements
    }

    /// Returns explicitly referenced logical qubits.
    #[must_use]
    pub fn logical_qubits(&self) -> &LogicalQubitResources {
        &self.logical_qubits
    }

    /// Returns explicitly referenced physical qubits.
    #[must_use]
    pub fn physical_qubits(&self) -> &PhysicalQubitResources {
        &self.physical_qubits
    }

    /// Returns recorded logical-to-physical bindings.
    #[must_use]
    pub fn bindings(&self) -> &[QubitResourceBinding] {
        &self.bindings
    }

    /// Returns whether no resources have been declared.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
            && self.logical_qubits.is_empty()
            && self.physical_qubits.is_empty()
            && self.bindings.is_empty()
    }

    /// Returns the number of scalar requirements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.requirements.len()
    }

    /// Combines two requirement sets.
    ///
    /// The operation is additive for scalar resources and union-like for
    /// explicit qubit references/bindings.
    ///
    /// Arithmetic is checked and duplicate/conflicting identities are rejected.
    pub fn merge(&mut self, other: &Self) -> Result<(), ResourceError> {
        for requirement in &other.requirements {
            self.requirements.push(requirement.clone());
        }

        for qubit in other.logical_qubits.iter().copied() {
            self.add_logical_qubit(qubit)?;
        }

        for qubit in other.physical_qubits.iter().copied() {
            self.add_physical_qubit(qubit)?;
        }

        for binding in other.bindings.iter().copied() {
            self.add_binding(binding)?;
        }

        Ok(())
    }

    /// Computes the aggregate minimum for one resource kind.
    ///
    /// Multiple requirements of the same kind are additive because they may
    /// originate from independent regions or operations.
    ///
    /// An unbounded requirement makes the aggregate unbounded.
    pub fn minimum_required(
        &self,
        kind: ResourceKind,
    ) -> Result<ResourceQuantity, ResourceError> {
        let mut total = ResourceQuantity::Finite(0);

        for requirement in &self.requirements {
            if requirement.kind() == kind {
                total = total.checked_add(ResourceQuantity::Finite(
                    requirement.range().min(),
                ))?;
            }
        }

        Ok(total)
    }

    /// Returns whether a finite capacity can satisfy all requirements of a
    /// particular resource kind.
    ///
    /// This method checks only the scalar requirements stored in this object.
    /// Hardware compatibility remains outside the IR resource layer.
    #[must_use]
    pub fn accepts_capacity(&self, kind: ResourceKind, capacity: u64) -> bool {
        let mut minimum = 0u64;
        let mut maximum: Option<u64> = None;

        for requirement in &self.requirements {
            if requirement.kind() != kind {
                continue;
            }

            let next_minimum = match minimum.checked_add(requirement.range().min()) {
                Some(value) => value,
                None => return false,
            };

            minimum = next_minimum;

            match requirement.range().max() {
                ResourceQuantity::Finite(value) => {
                    maximum = match maximum {
                        Some(current) => current.checked_add(value),
                        None => Some(value),
                    };

                    if maximum.is_none() {
                        return false;
                    }
                }
                ResourceQuantity::Unbounded => {
                    maximum = None;
                }
            }
        }

        if capacity < minimum {
            return false;
        }

        match maximum {
            Some(maximum) => capacity <= maximum,
            None => true,
        }
    }
}

impl ResourceRequirement {
    /// Returns the resource kind.
    #[must_use]
    pub const fn kind(&self) -> ResourceKind {
        self.kind
    }

    /// Returns the requirement range.
    #[must_use]
    pub const fn range(&self) -> ResourceRange {
        self.range
    }

    /// Returns the optional semantic label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

/// A target-independent resource capacity.
///
/// This type belongs to IR compatibility contracts. A concrete hardware
/// description can later translate its target capabilities into capacities
/// understood by this type without making the IR depend on hardware types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceCapacity {
    kind: ResourceKind,
    capacity: ResourceQuantity,
    label: Option<String>,
}

impl ResourceCapacity {
    /// Creates a finite capacity.
    #[must_use]
    pub fn finite(kind: ResourceKind, capacity: u64) -> Self {
        Self {
            kind,
            capacity: ResourceQuantity::Finite(capacity),
            label: None,
        }
    }

    /// Creates an unbounded capacity.
    ///
    /// This is primarily useful for abstract simulators, symbolic compilation,
    /// or capability models where the concrete limit is deliberately deferred.
    #[must_use]
    pub fn unbounded(kind: ResourceKind) -> Self {
        Self {
            kind,
            capacity: ResourceQuantity::Unbounded,
            label: None,
        }
    }

    /// Adds a descriptive label.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Returns the resource kind.
    #[must_use]
    pub const fn kind(&self) -> ResourceKind {
        self.kind
    }

    /// Returns the capacity.
    #[must_use]
    pub const fn capacity(&self) -> ResourceQuantity {
        self.capacity
    }

    /// Returns the optional label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Checks whether a requirement can be satisfied by this capacity.
    #[must_use]
    pub fn satisfies(&self, requirement: &ResourceRequirement) -> bool {
        if self.kind != requirement.kind() {
            return false;
        }

        match self.capacity {
            ResourceQuantity::Unbounded => true,
            ResourceQuantity::Finite(capacity) => requirement.accepts(capacity),
        }
    }
}

/// An error produced by resource-model operations.
///
/// These errors are intentionally local to the resource contract so this file
/// can be completed independently. The central `quantum::ir::errors` module
/// can later map these variants into the canonical IR error taxonomy without
/// changing the resource semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceError {
    /// A checked arithmetic operation overflowed.
    ArithmeticOverflow,

    /// A subtraction would produce a negative resource quantity.
    InsufficientQuantity,

    /// An operation involving unbounded values has no determinate result.
    IndeterminateOperation,

    /// A range had an invalid lower/upper relationship.
    InvalidRange,

    /// Two ranges had no intersection.
    EmptyIntersection,

    /// A logical or physical qubit was inserted twice.
    DuplicateQubit,

    /// A logical qubit or physical qubit was already bound elsewhere.
    ConflictingQubitBinding,
}

impl fmt::Display for ResourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::ArithmeticOverflow => "resource arithmetic overflow",
            Self::InsufficientQuantity => "resource quantity is insufficient",
            Self::IndeterminateOperation => {
                "resource operation has no determinate result"
            }
            Self::InvalidRange => "resource range is invalid",
            Self::EmptyIntersection => "resource ranges have no intersection",
            Self::DuplicateQubit => "duplicate qubit resource",
            Self::ConflictingQubitBinding => {
                "logical or physical qubit already has a conflicting binding"
            }
        };

        formatter.write_str(message)
    }
}

impl std::error::Error for ResourceError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_quantities_are_checked() {
        assert_eq!(
            ResourceQuantity::finite(10)
                .checked_add(ResourceQuantity::finite(20))
                .expect("checked addition should succeed"),
            ResourceQuantity::finite(30)
        );

        assert_eq!(
            ResourceQuantity::finite(u64::MAX)
                .checked_add(ResourceQuantity::finite(1)),
            Err(ResourceError::ArithmeticOverflow)
        );
    }

    #[test]
    fn unbounded_is_not_a_numeric_sentinel() {
        assert!(ResourceQuantity::unbounded().is_unbounded());
        assert!(ResourceQuantity::unbounded().at_least(u64::MAX));
        assert!(ResourceQuantity::unbounded().exceeds(u64::MAX));
    }

    #[test]
    fn exact_range_accepts_only_exact_capacity() {
        let range = ResourceRange::exact(100);

        assert!(range.accepts(100));
        assert!(!range.accepts(99));
        assert!(!range.accepts(101));
    }

    #[test]
    fn minimum_range_scales_without_fixed_machine_limit() {
        let range = ResourceRange::at_least(1_000_000_000);

        assert!(range.accepts(1_000_000_000));
        assert!(range.accepts(u64::MAX));
    }

    #[test]
    fn bounded_range_rejects_invalid_bounds() {
        assert_eq!(
            ResourceRange::between(10, 9),
            Err(ResourceError::InvalidRange)
        );
    }

    #[test]
    fn unbounded_range_has_no_architectural_ceiling() {
        let range = ResourceRange::at_least(u64::MAX);

        assert!(range.is_unbounded());
        assert!(range.accepts(u64::MAX));
    }

    #[test]
    fn resource_requirement_is_target_independent() {
        let requirement =
            ResourceRequirement::at_least(ResourceKind::LogicalQubits, 1000);

        assert_eq!(requirement.kind(), ResourceKind::LogicalQubits);
        assert_eq!(requirement.range().min(), 1000);
        assert!(requirement.range().is_unbounded());
    }

    #[test]
    fn capacity_can_satisfy_requirement() {
        let requirement =
            ResourceRequirement::at_least(ResourceKind::LogicalQubits, 100);

        let capacity =
            ResourceCapacity::finite(ResourceKind::LogicalQubits, 128);

        assert!(capacity.satisfies(&requirement));
    }

    #[test]
    fn insufficient_capacity_is_rejected() {
        let requirement =
            ResourceRequirement::at_least(ResourceKind::LogicalQubits, 100);

        let capacity =
            ResourceCapacity::finite(ResourceKind::LogicalQubits, 99);

        assert!(!capacity.satisfies(&requirement));
    }

    #[test]
    fn unbounded_capacity_satisfies_finite_requirement() {
        let requirement =
            ResourceRequirement::exact(ResourceKind::LogicalQubits, u64::MAX);

        let capacity =
            ResourceCapacity::unbounded(ResourceKind::LogicalQubits);

        assert!(capacity.satisfies(&requirement));
    }

    #[test]
    fn logical_qubit_resources_reject_duplicates() {
        let mut resources = LogicalQubitResources::new();

        let qubit = QubitId::new(7);

        resources
            .insert(qubit)
            .expect("first insertion should succeed");

        assert_eq!(
            resources.insert(qubit),
            Err(ResourceError::DuplicateQubit)
        );
    }

    #[test]
    fn physical_qubit_resources_reject_duplicates() {
        let mut resources = PhysicalQubitResources::new();

        let qubit = PhysicalQubitId::new(42);

        resources
            .insert(qubit)
            .expect("first insertion should succeed");

        assert_eq!(
            resources.insert(qubit),
            Err(ResourceError::DuplicateQubit)
        );
    }

    #[test]
    fn bindings_reject_conflicting_logical_qubits() {
        let mut resources = QuantumResourceRequirements::new();

        let logical = QubitId::new(1);
        let physical_a = PhysicalQubitId::new(10);
        let physical_b = PhysicalQubitId::new(11);

        resources
            .add_binding(QubitResourceBinding::new(logical, physical_a))
            .expect("first binding should succeed");

        assert_eq!(
            resources.add_binding(QubitResourceBinding::new(logical, physical_b)),
            Err(ResourceError::ConflictingQubitBinding)
        );
    }

    #[test]
    fn bindings_reject_conflicting_physical_qubits() {
        let mut resources = QuantumResourceRequirements::new();

        let logical_a = QubitId::new(1);
        let logical_b = QubitId::new(2);
        let physical = PhysicalQubitId::new(10);

        resources
            .add_binding(QubitResourceBinding::new(logical_a, physical))
            .expect("first binding should succeed");

        assert_eq!(
            resources.add_binding(QubitResourceBinding::new(logical_b, physical)),
            Err(ResourceError::ConflictingQubitBinding)
        );
    }

    #[test]
    fn scalar_requirements_are_aggregated_with_checked_arithmetic() {
        let mut resources = QuantumResourceRequirements::new();

        resources
            .require_at_least(ResourceKind::LogicalQubits, 100)
            .expect("requirement should succeed");

        resources
            .require_at_least(ResourceKind::LogicalQubits, 200)
            .expect("requirement should succeed");

        assert_eq!(
            resources
                .minimum_required(ResourceKind::LogicalQubits)
                .expect("aggregation should succeed"),
            ResourceQuantity::finite(300)
        );
    }

    #[test]
    fn capacity_check_is_resource_kind_specific() {
        let mut resources = QuantumResourceRequirements::new();

        resources
            .require_at_least(ResourceKind::LogicalQubits, 100)
            .expect("requirement should succeed");

        assert!(resources.accepts_capacity(ResourceKind::LogicalQubits, 100));
        assert!(resources.accepts_capacity(ResourceKind::LogicalQubits, 1000));

        assert!(!resources.accepts_capacity(ResourceKind::PhysicalQubits, 1000));
    }

    #[test]
    fn no_63_qubit_architectural_boundary_exists() {
        let requirement =
            ResourceRequirement::at_least(ResourceKind::LogicalQubits, 64);

        assert!(requirement.accepts(u64::MAX));
    }

    #[test]
    fn no_4096_qubit_architectural_boundary_exists() {
        let requirement =
            ResourceRequirement::at_least(ResourceKind::LogicalQubits, 4097);

        assert!(requirement.accepts(u64::MAX));
    }

    #[test]
    fn merge_preserves_independent_requirements() {
        let mut first = QuantumResourceRequirements::new();
        let mut second = QuantumResourceRequirements::new();

        first
            .require_at_least(ResourceKind::LogicalQubits, 10)
            .expect("requirement should succeed");

        second
            .require_at_least(ResourceKind::LogicalQubits, 20)
            .expect("requirement should succeed");

        first.merge(&second).expect("merge should succeed");

        assert_eq!(
            first
                .minimum_required(ResourceKind::LogicalQubits)
                .expect("aggregation should succeed"),
            ResourceQuantity::finite(30)
        );
    }
}