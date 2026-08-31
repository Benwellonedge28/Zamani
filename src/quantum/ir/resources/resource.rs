//! Zamani Quantum IR — Universal Resource Requirements
//!
//! This module defines the target-independent resource contract of the
//! canonical Zamani Quantum IR.
//!
//! # Architectural role
//!
//! This module answers:
//!
//! > What resources does this semantic quantum program require?
//!
//! It deliberately does NOT answer:
//!
//! - which physical machine provides the resources;
//! - where logical qubits are placed;
//! - how qubits are routed;
//! - when operations execute;
//! - how resources are scheduled;
//! - which calibration is used;
//! - which backend is selected;
//! - how pulses are generated;
//! - how a quantum program is simulated;
//! - how a QEC decoder operates;
//! - how a QPU is contacted.
//!
//! Those responsibilities belong to downstream subsystems.
//!
//! # Universal scalability contract
//!
//! Zamani programs are written once and may be lowered to any compatible
//! target for which sufficient resources and capabilities exist.
//!
//! Consequently, this module contains NO architectural constants such as:
//!
//! ```text
//! MAX_QUBITS = 64
//! MAX_QUBITS = 4096
//! MAX_GATES = 1_000_000
//! ```
//!
//! A resource quantity is data, not architecture.
//!
//! Finite quantities use `u64`. An explicitly unbounded quantity is represented
//! by `ResourceQuantity::Unbounded`; it is never represented by `u64::MAX`.
//!
//! `Unbounded` means:
//!
//! > this semantic requirement has no finite upper bound known at this IR
//! > boundary.
//!
//! It does NOT mean that a physical computer has literally infinite capacity.
//!
//! Actual finite limits are established by:
//!
//! ```text
//! quantum::ir::limits
//! quantum::hardware
//! quantum::hardware::capabilities
//! quantum::hardware::topology
//! backend/runtime policies
//! host/execution constraints
//! ```
//!
//! # Ownership
//!
//! This module owns:
//!
//! - resource quantities;
//! - resource ranges;
//! - resource kinds;
//! - extensible resource identifiers;
//! - resource requirements;
//! - resource requirement collections;
//! - resource capacities used for target-independent compatibility checks;
//! - resource satisfaction diagnostics.
//!
//! This module does NOT own:
//!
//! - logical-to-physical mapping;
//! - routing;
//! - topology;
//! - scheduling;
//! - calibration;
//! - device discovery;
//! - hardware allocation;
//! - execution.
//!
//! # Qubit integration
//!
//! Logical qubit identity remains owned by:
//!
//! `quantum::ir::qubit::QubitId`
//!
//! This module may reference `QubitId` when a resource requirement needs to be
//! scoped to a particular logical qubit.
//!
//! It must never redefine `QubitId` or `PhysicalQubitId`.
//!
//! Physical placement belongs outside this module.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::ir::qubit
//!         │
//!         ▼
//! resources::resource
//!         │
//!         ├── validation
//!         ├── analysis
//!         ├── scheduling
//!         ├── routing
//!         └── hardware integration
//! ```
//!
//! No dependency points back from this file to those downstream systems.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! The module explicitly forbids unsafe code.
//!
//! # Determinism
//!
//! Resource identifiers and collections must have deterministic equality,
//! hashing and iteration semantics where the API promises ordering.
//!
//! The resource contract itself does not depend on `HashMap` ordering.
//!
//! # Serialization
//!
//! Serialization is owned by the IR serialization subsystem.
//!
//! This file therefore defines semantic values only and does not introduce a
//! second serialization format.
//!
//! # Hashing
//!
//! All types implement semantic equality/hash where appropriate.
//!
//! Canonical IR hashing remains owned by `quantum::ir::hash`.
//!
//! # Versioning
//!
//! Resource semantics are part of the canonical IR contract. Breaking changes
//! must be handled by the canonical IR version/migration system.
//!
//! This file must not introduce a second independent versioning system.
//!
//! # Important design rule
//!
//! A resource requirement is a statement of intent.
//!
//! A resource capacity is a statement of available capacity.
//!
//! Neither object performs allocation.
//!
//! ```text
//! Requirement
//!     │
//!     │ compatibility check
//!     ▼
//! Capacity
//!     │
//!     ▼
//! downstream target resolution
//!     │
//!     ▼
//! allocation / mapping / scheduling
//! ```
//!
//! This separation keeps the canonical IR hardware-independent.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use super::super::qubit::QubitId;

// =============================================================================
// Resource quantity
// =============================================================================

/// A non-negative semantic resource quantity.
///
/// `Finite(n)` represents an explicitly known finite amount.
///
/// `Unbounded` represents the absence of a finite semantic upper bound.
///
/// `Unbounded` must never be confused with `u64::MAX`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceQuantity {
    /// A finite quantity.
    Finite(u64),

    /// No finite semantic upper bound is specified.
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

    /// Returns whether this quantity is unbounded.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        matches!(self, Self::Unbounded)
    }

    /// Returns the finite value, if this quantity is finite.
    #[must_use]
    pub const fn as_finite(self) -> Option<u64> {
        match self {
            Self::Finite(value) => Some(value),
            Self::Unbounded => None,
        }
    }

    /// Checked addition.
    ///
    /// If either operand is unbounded, the result is unbounded.
    pub const fn checked_add(self, rhs: Self) -> Result<Self, ResourceError> {
        match (self, rhs) {
            (Self::Unbounded, _) | (_, Self::Unbounded) => Ok(Self::Unbounded),
            (Self::Finite(lhs), Self::Finite(rhs)) => match lhs.checked_add(rhs) {
                Some(value) => Ok(Self::Finite(value)),
                None => Err(ResourceError::ArithmeticOverflow),
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

    /// Returns whether this quantity is at least `value`.
    #[must_use]
    pub const fn at_least(self, value: u64) -> bool {
        match self {
            Self::Finite(quantity) => quantity >= value,
            Self::Unbounded => true,
        }
    }

    /// Returns whether this quantity exceeds `value`.
    #[must_use]
    pub const fn exceeds(self, value: u64) -> bool {
        match self {
            Self::Finite(quantity) => quantity > value,
            Self::Unbounded => true,
        }
    }

    /// Returns the semantic maximum of two quantities.
    #[must_use]
    pub const fn max(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Unbounded, _) | (_, Self::Unbounded) => Self::Unbounded,
            (Self::Finite(lhs), Self::Finite(rhs)) => {
                if lhs >= rhs {
                    Self::Finite(lhs)
                } else {
                    Self::Finite(rhs)
                }
            }
        }
    }

    /// Returns the semantic minimum of two quantities.
    ///
    /// `Unbounded` does not constrain the minimum.
    #[must_use]
    pub const fn min(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Unbounded, finite) | (finite, Self::Unbounded) => finite,
            (Self::Finite(lhs), Self::Finite(rhs)) => {
                if lhs <= rhs {
                    Self::Finite(lhs)
                } else {
                    Self::Finite(rhs)
                }
            }
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

// =============================================================================
// Resource range
// =============================================================================

/// An inclusive semantic resource range.
///
/// The requirement is satisfied when:
///
/// ```text
/// minimum <= capacity <= maximum
/// ```
///
/// when `maximum` is finite.
///
/// When `maximum` is `Unbounded`, the requirement is:
///
/// ```text
/// minimum <= capacity
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ResourceRange {
    minimum: u64,
    maximum: ResourceQuantity,
}

impl ResourceRange {
    /// Creates an exact requirement.
    #[must_use]
    pub const fn exact(value: u64) -> Self {
        Self {
            minimum: value,
            maximum: ResourceQuantity::Finite(value),
        }
    }

    /// Creates a minimum-only requirement.
    #[must_use]
    pub const fn at_least(minimum: u64) -> Self {
        Self {
            minimum,
            maximum: ResourceQuantity::Unbounded,
        }
    }

    /// Creates a finite inclusive range.
    pub const fn between(minimum: u64, maximum: u64) -> Result<Self, ResourceError> {
        if minimum > maximum {
            return Err(ResourceError::InvalidRange);
        }

        Ok(Self {
            minimum,
            maximum: ResourceQuantity::Finite(maximum),
        })
    }

    /// Returns the minimum.
    #[must_use]
    pub const fn minimum(self) -> u64 {
        self.minimum
    }

    /// Returns the maximum.
    #[must_use]
    pub const fn maximum(self) -> ResourceQuantity {
        self.maximum
    }

    /// Compatibility alias for callers using `min()`.
    #[must_use]
    pub const fn min(self) -> u64 {
        self.minimum()
    }

    /// Compatibility alias for callers using `max()`.
    #[must_use]
    pub const fn max(self) -> ResourceQuantity {
        self.maximum()
    }

    /// Returns whether the upper bound is unbounded.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        self.maximum.is_unbounded()
    }

    /// Checks whether a finite capacity satisfies this range.
    #[must_use]
    pub const fn accepts(self, capacity: u64) -> bool {
        if capacity < self.minimum {
            return false;
        }

        match self.maximum {
            ResourceQuantity::Finite(maximum) => capacity <= maximum,
            ResourceQuantity::Unbounded => true,
        }
    }

    /// Intersects this range with another range.
    ///
    /// This operation is useful when independent compilation policies impose
    /// multiple constraints on the same resource.
    pub const fn intersect(self, other: Self) -> Result<Self, ResourceError> {
        let minimum = if self.minimum >= other.minimum {
            self.minimum
        } else {
            other.minimum
        };

        let maximum = self.maximum.min(other.maximum);

        match maximum {
            ResourceQuantity::Finite(value) => {
                if minimum > value {
                    Err(ResourceError::EmptyIntersection)
                } else {
                    Ok(Self {
                        minimum,
                        maximum: ResourceQuantity::Finite(value),
                    })
                }
            }
            ResourceQuantity::Unbounded => Ok(Self {
                minimum,
                maximum: ResourceQuantity::Unbounded,
            }),
        }
    }
}

impl Default for ResourceRange {
    fn default() -> Self {
        Self::exact(0)
    }
}

// =============================================================================
// Resource namespace
// =============================================================================

/// Stable namespace for an extensible resource kind.
///
/// Built-in resources use the canonical `zamani` namespace.
///
/// Custom resources may use another namespace, allowing new quantum
/// technologies to introduce resource kinds without modifying this file.
///
/// Examples:
///
/// ```text
/// zamani/logical_qubits
/// zamani/physical_qubits
/// zamani/measurements
/// neutral_atom/atom_sites
/// photonic/optical_modes
/// vendor_x/custom_control_resource
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceNamespace(String);

impl ResourceNamespace {
    /// Canonical Zamani namespace.
    #[must_use]
    pub fn zamani() -> Self {
        Self(String::from("zamani"))
    }

    /// Creates a custom namespace.
    ///
    /// Empty namespaces are rejected because they cannot provide a stable
    /// identity.
    pub fn new(namespace: impl Into<String>) -> Result<Self, ResourceError> {
        let namespace = namespace.into();

        if namespace.trim().is_empty() {
            return Err(ResourceError::EmptyResourceNamespace);
        }

        Ok(Self(namespace))
    }

    /// Returns the namespace string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for ResourceNamespace {
    fn default() -> Self {
        Self::zamani()
    }
}

impl fmt::Display for ResourceNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Stable name of an extensible resource kind.
///
/// Unlike a closed enum, this allows Zamani to represent resources introduced
/// by future quantum technologies without changing the canonical resource
/// structure.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceName(String);

impl ResourceName {
    /// Creates a resource name.
    ///
    /// Empty names are rejected.
    pub fn new(name: impl Into<String>) -> Result<Self, ResourceError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(ResourceError::EmptyResourceName);
        }

        Ok(Self(name))
    }

    /// Returns the resource name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ResourceName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Fully qualified resource kind.
///
/// The `(namespace, name)` pair is the extensibility boundary.
///
/// A new resource does not require changing this source file.
///
/// Examples:
///
/// ```text
/// zamani/logical_qubits
/// zamani/pulse_operations
/// zamani/measurement_results
/// neutral_atom/atom_sites
/// photonic/optical_modes
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceKind {
    namespace: ResourceNamespace,
    name: ResourceName,
}

impl ResourceKind {
    /// Creates a fully qualified resource kind.
    pub fn new(
        namespace: ResourceNamespace,
        name: ResourceName,
    ) -> Self {
        Self { namespace, name }
    }

    /// Creates a built-in Zamani resource kind.
    pub fn zamani(name: impl Into<String>) -> Result<Self, ResourceError> {
        Ok(Self {
            namespace: ResourceNamespace::zamani(),
            name: ResourceName::new(name)?,
        })
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &ResourceNamespace {
        &self.namespace
    }

    /// Returns the resource name.
    #[must_use]
    pub fn name(&self) -> &ResourceName {
        &self.name
    }

    /// Returns the fully qualified stable name.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}/{}", self.namespace, self.name)
    }

    /// Logical qubit resources.
    #[must_use]
    pub fn logical_qubits() -> Self {
        Self::builtin("logical_qubits")
    }

    /// Physical qubit capacity.
    #[must_use]
    pub fn physical_qubits() -> Self {
        Self::builtin("physical_qubits")
    }

    /// Classical bits.
    #[must_use]
    pub fn classical_bits() -> Self {
        Self::builtin("classical_bits")
    }

    /// Classical words.
    #[must_use]
    pub fn classical_words() -> Self {
        Self::builtin("classical_words")
    }

    /// Quantum operations.
    #[must_use]
    pub fn quantum_operations() -> Self {
        Self::builtin("quantum_operations")
    }

    /// Gate operations.
    #[must_use]
    pub fn gate_operations() -> Self {
        Self::builtin("gate_operations")
    }

    /// Multi-qubit operations.
    #[must_use]
    pub fn multi_qubit_operations() -> Self {
        Self::builtin("multi_qubit_operations")
    }

    /// Measurements.
    #[must_use]
    pub fn measurements() -> Self {
        Self::builtin("measurements")
    }

    /// Mid-circuit measurements.
    #[must_use]
    pub fn mid_circuit_measurements() -> Self {
        Self::builtin("mid_circuit_measurements")
    }

    /// Pulse operations.
    #[must_use]
    pub fn pulse_operations() -> Self {
        Self::builtin("pulse_operations")
    }

    /// Waveforms.
    #[must_use]
    pub fn waveforms() -> Self {
        Self::builtin("waveforms")
    }

    /// Control channels.
    #[must_use]
    pub fn channels() -> Self {
        Self::builtin("channels")
    }

    /// Frames.
    #[must_use]
    pub fn frames() -> Self {
        Self::builtin("frames")
    }

    /// Schedule events.
    #[must_use]
    pub fn schedule_events() -> Self {
        Self::builtin("schedule_events")
    }

    /// Parameters.
    #[must_use]
    pub fn parameters() -> Self {
        Self::builtin("parameters")
    }

    /// Control-flow depth.
    #[must_use]
    pub fn control_flow_depth() -> Self {
        Self::builtin("control_flow_depth")
    }

    /// Circuit/program depth.
    #[must_use]
    pub fn circuit_depth() -> Self {
        Self::builtin("circuit_depth")
    }

    /// Execution time.
    #[must_use]
    pub fn execution_time() -> Self {
        Self::builtin("execution_time")
    }

    /// Classical memory bytes.
    #[must_use]
    pub fn classical_memory_bytes() -> Self {
        Self::builtin("classical_memory_bytes")
    }

    /// Waveform memory bytes.
    #[must_use]
    pub fn waveform_memory_bytes() -> Self {
        Self::builtin("waveform_memory_bytes")
    }

    /// Serialized program bytes.
    #[must_use]
    pub fn program_bytes() -> Self {
        Self::builtin("program_bytes")
    }

    /// Generic memory bytes.
    #[must_use]
    pub fn memory_bytes() -> Self {
        Self::builtin("memory_bytes")
    }

    fn builtin(name: &'static str) -> Self {
        Self {
            namespace: ResourceNamespace::zamani(),
            name: ResourceName(String::from(name)),
        }
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}", self.namespace, self.name)
    }
}

// =============================================================================
// Resource scope
// =============================================================================

/// Scope to which a resource requirement applies.
///
/// `Global` means the requirement applies to the complete IR object.
///
/// `LogicalQubit` allows a resource to be associated with a semantic logical
/// qubit without transferring ownership of qubit identity to this module.
///
/// `Named` supports extensible semantic scopes without introducing hardware
/// identifiers.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceScope {
    /// Requirement applies globally.
    Global,

    /// Requirement applies to one canonical logical qubit.
    LogicalQubit(QubitId),

    /// Named semantic scope.
    Named(String),
}

impl ResourceScope {
    /// Creates a logical-qubit scope.
    #[must_use]
    pub const fn logical_qubit(qubit: QubitId) -> Self {
        Self::LogicalQubit(qubit)
    }

    /// Creates a named semantic scope.
    pub fn named(name: impl Into<String>) -> Result<Self, ResourceError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(ResourceError::EmptyResourceScope);
        }

        Ok(Self::Named(name))
    }

    /// Returns whether this is a global scope.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        matches!(self, Self::Global)
    }

    /// Returns the logical qubit when scoped to one.
    #[must_use]
    pub const fn logical_qubit_id(&self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(qubit) => Some(*qubit),
            _ => None,
        }
    }
}

impl Default for ResourceScope {
    fn default() -> Self {
        Self::Global
    }
}

// =============================================================================
// Resource requirement
// =============================================================================

/// One target-independent resource requirement.
///
/// A requirement expresses semantic intent. It does not allocate anything.
///
/// Examples:
///
/// ```text
/// at least 100 logical qubits
/// at least 1 measurement resource
/// between 1 and 4 control channels
/// unbounded semantic operation capacity
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResourceRequirement {
    kind: ResourceKind,
    range: ResourceRange,
    scope: ResourceScope,
    label: Option<String>,
}

impl ResourceRequirement {
    /// Creates an exact global requirement.
    #[must_use]
    pub fn exact(kind: ResourceKind, amount: u64) -> Self {
        Self {
            kind,
            range: ResourceRange::exact(amount),
            scope: ResourceScope::Global,
            label: None,
        }
    }

    /// Creates a minimum global requirement.
    #[must_use]
    pub fn at_least(kind: ResourceKind, amount: u64) -> Self {
        Self {
            kind,
            range: ResourceRange::at_least(amount),
            scope: ResourceScope::Global,
            label: None,
        }
    }

    /// Creates a bounded global requirement.
    pub fn between(
        kind: ResourceKind,
        minimum: u64,
        maximum: u64,
    ) -> Result<Self, ResourceError> {
        Ok(Self {
            kind,
            range: ResourceRange::between(minimum, maximum)?,
            scope: ResourceScope::Global,
            label: None,
        })
    }

    /// Associates the requirement with a semantic scope.
    #[must_use]
    pub fn with_scope(mut self, scope: ResourceScope) -> Self {
        self.scope = scope;
        self
    }

    /// Associates the requirement with one logical qubit.
    #[must_use]
    pub const fn for_logical_qubit(mut self, qubit: QubitId) -> Self {
        self.scope = ResourceScope::LogicalQubit(qubit);
        self
    }

    /// Adds a descriptive label.
    ///
    /// Labels are semantic metadata, not hardware identifiers.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Returns the resource kind.
    #[must_use]
    pub fn kind(&self) -> &ResourceKind {
        &self.kind
    }

    /// Returns the required range.
    #[must_use]
    pub const fn range(&self) -> ResourceRange {
        self.range
    }

    /// Returns the semantic scope.
    #[must_use]
    pub fn scope(&self) -> &ResourceScope {
        &self.scope
    }

    /// Returns the optional label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Checks whether a finite capacity satisfies this requirement.
    #[must_use]
    pub fn accepts(&self, capacity: u64) -> bool {
        self.range.accepts(capacity)
    }
}

// =============================================================================
// Resource capacity
// =============================================================================

/// Target-independent description of available resource capacity.
///
/// This is intentionally not a hardware device type.
///
/// Hardware integration code may translate device capabilities into this
/// representation before performing resource compatibility checks.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResourceCapacity {
    kind: ResourceKind,
    capacity: ResourceQuantity,
    scope: ResourceScope,
    label: Option<String>,
}

impl ResourceCapacity {
    /// Creates a finite global capacity.
    #[must_use]
    pub fn finite(kind: ResourceKind, capacity: u64) -> Self {
        Self {
            kind,
            capacity: ResourceQuantity::Finite(capacity),
            scope: ResourceScope::Global,
            label: None,
        }
    }

    /// Creates an unbounded global capacity.
    #[must_use]
    pub fn unbounded(kind: ResourceKind) -> Self {
        Self {
            kind,
            capacity: ResourceQuantity::Unbounded,
            scope: ResourceScope::Global,
            label: None,
        }
    }

    /// Associates the capacity with a semantic scope.
    #[must_use]
    pub fn with_scope(mut self, scope: ResourceScope) -> Self {
        self.scope = scope;
        self
    }

    /// Adds a descriptive label.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Returns the resource kind.
    #[must_use]
    pub fn kind(&self) -> &ResourceKind {
        &self.kind
    }

    /// Returns the capacity.
    #[must_use]
    pub const fn capacity(&self) -> ResourceQuantity {
        self.capacity
    }

    /// Returns the scope.
    #[must_use]
    pub fn scope(&self) -> &ResourceScope {
        &self.scope
    }

    /// Returns the optional label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Checks whether this capacity can satisfy a requirement.
    ///
    /// Resource kind and scope must match.
    #[must_use]
    pub fn satisfies(&self, requirement: &ResourceRequirement) -> bool {
        if self.kind != *requirement.kind() {
            return false;
        }

        if self.scope != *requirement.scope() {
            return false;
        }

        match self.capacity {
            ResourceQuantity::Unbounded => true,
            ResourceQuantity::Finite(capacity) => requirement.accepts(capacity),
        }
    }
}

// =============================================================================
// Resource requirement set
// =============================================================================

/// A collection of semantic resource requirements.
///
/// Requirements are intentionally stored independently rather than collapsed
/// into one fixed schema.
///
/// This allows future resource classes to coexist without changing the
/// `QuantumResourceRequirements` structure.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct QuantumResourceRequirements {
    requirements: Vec<ResourceRequirement>,
}

impl QuantumResourceRequirements {
    /// Creates an empty requirement collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            requirements: Vec::new(),
        }
    }

    /// Creates a requirement collection with pre-existing requirements.
    #[must_use]
    pub fn from_vec(requirements: Vec<ResourceRequirement>) -> Self {
        Self { requirements }
    }

    /// Adds one requirement.
    pub fn add(
        &mut self,
        requirement: ResourceRequirement,
    ) -> Result<(), ResourceError> {
        self.requirements.push(requirement);
        Ok(())
    }

    /// Adds an exact global requirement.
    pub fn require_exact(
        &mut self,
        kind: ResourceKind,
        amount: u64,
    ) -> Result<(), ResourceError> {
        self.add(ResourceRequirement::exact(kind, amount))
    }

    /// Adds a minimum global requirement.
    pub fn require_at_least(
        &mut self,
        kind: ResourceKind,
        amount: u64,
    ) -> Result<(), ResourceError> {
        self.add(ResourceRequirement::at_least(kind, amount))
    }

    /// Adds a finite bounded global requirement.
    pub fn require_between(
        &mut self,
        kind: ResourceKind,
        minimum: u64,
        maximum: u64,
    ) -> Result<(), ResourceError> {
        self.add(ResourceRequirement::between(
            kind,
            minimum,
            maximum,
        )?)
    }

    /// Returns all requirements.
    #[must_use]
    pub fn requirements(&self) -> &[ResourceRequirement] {
        &self.requirements
    }

    /// Returns the number of requirements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.requirements.len()
    }

    /// Returns whether no requirements exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }

    /// Returns an iterator over requirements.
    pub fn iter(&self) -> impl Iterator<Item = &ResourceRequirement> {
        self.requirements.iter()
    }

    /// Merges another requirement collection.
    ///
    /// Requirements remain separate. This is important because two ranges
    /// may represent independent resource consumers rather than one merged
    /// mathematical interval.
    pub fn merge(&mut self, other: &Self) -> Result<(), ResourceError> {
        self.requirements
            .try_reserve(other.requirements.len())
            .map_err(|_| ResourceError::AllocationFailure)?;

        self.requirements
            .extend(other.requirements.iter().cloned());

        Ok(())
    }

    /// Returns all requirements matching a resource kind.
    pub fn for_kind<'a>(
        &'a self,
        kind: &'a ResourceKind,
    ) -> impl Iterator<Item = &'a ResourceRequirement> + 'a {
        self.requirements
            .iter()
            .filter(move |requirement| requirement.kind() == kind)
    }

    /// Computes the aggregate minimum requirement for a kind and scope.
    ///
    /// Independent requirements are additive.
    ///
    /// Checked arithmetic prevents wraparound.
    pub fn minimum_required(
        &self,
        kind: &ResourceKind,
        scope: &ResourceScope,
    ) -> Result<ResourceQuantity, ResourceError> {
        let mut total = ResourceQuantity::Finite(0);

        for requirement in &self.requirements {
            if requirement.kind() == kind && requirement.scope() == scope {
                total = total.checked_add(ResourceQuantity::Finite(
                    requirement.range().minimum(),
                ))?;
            }
        }

        Ok(total)
    }

    /// Determines whether a finite capacity can satisfy all matching
    /// requirements.
    ///
    /// This method is intentionally conservative.
    ///
    /// Independent requirements are additive. If any matching requirement is
    /// unbounded above, no finite upper capacity is required by the semantic
    /// requirement and only the aggregate minimum is checked.
    #[must_use]
    pub fn accepts_capacity(
        &self,
        kind: &ResourceKind,
        scope: &ResourceScope,
        capacity: u64,
    ) -> bool {
        let mut minimum = 0u64;
        let mut has_unbounded_upper_bound = false;
        let mut maximum = 0u64;

        for requirement in &self.requirements {
            if requirement.kind() != kind || requirement.scope() != scope {
                continue;
            }

            minimum = match minimum.checked_add(requirement.range().minimum()) {
                Some(value) => value,
                None => return false,
            };

            match requirement.range().maximum() {
                ResourceQuantity::Finite(value) => {
                    maximum = match maximum.checked_add(value) {
                        Some(result) => result,
                        None => return false,
                    };
                }
                ResourceQuantity::Unbounded => {
                    has_unbounded_upper_bound = true;
                }
            }
        }

        if capacity < minimum {
            return false;
        }

        if has_unbounded_upper_bound {
            return true;
        }

        capacity <= maximum
    }

    /// Validates the internal semantic requirements.
    ///
    /// This is deliberately lightweight; whole-program structural validation
    /// remains owned by `quantum::ir::validation`.
    pub fn validate(&self) -> Result<(), ResourceError> {
        for requirement in &self.requirements {
            if requirement.label().is_some_and(|label| label.trim().is_empty()) {
                return Err(ResourceError::EmptyResourceLabel);
            }

            if let ResourceScope::Named(name) = requirement.scope() {
                if name.trim().is_empty() {
                    return Err(ResourceError::EmptyResourceScope);
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Resource satisfaction
// =============================================================================

/// Result of checking one resource requirement against a capacity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceSatisfaction {
    /// The capacity satisfies the requirement.
    Satisfied,

    /// The capacity is insufficient.
    Insufficient {
        /// Required minimum.
        required_minimum: u64,

        /// Available finite capacity.
        available: u64,
    },

    /// Resource kinds do not match.
    KindMismatch,

    /// Resource scopes do not match.
    ScopeMismatch,

    /// The requirement cannot be represented by the supplied finite capacity
    /// because of an incompatible upper constraint.
    RangeMismatch {
        /// Required maximum, when finite.
        required_maximum: ResourceQuantity,

        /// Available finite capacity.
        available: u64,
    },
}

impl ResourceSatisfaction {
    /// Returns whether the result is satisfied.
    #[must_use]
    pub const fn is_satisfied(&self) -> bool {
        matches!(self, Self::Satisfied)
    }
}

// =============================================================================
// Resource errors
// =============================================================================

/// Errors produced by resource-model operations.
///
/// The central IR error layer may translate these errors into its canonical
/// error taxonomy. This file deliberately keeps its own precise semantic
/// errors so it remains independently implementable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceError {
    /// Checked resource arithmetic overflowed.
    ArithmeticOverflow,

    /// A finite subtraction would produce a negative quantity.
    InsufficientQuantity,

    /// An operation involving unbounded quantities has no determinate result.
    IndeterminateOperation,

    /// A range has invalid bounds.
    InvalidRange,

    /// Two ranges have no intersection.
    EmptyIntersection,

    /// Resource namespace is empty.
    EmptyResourceNamespace,

    /// Resource name is empty.
    EmptyResourceName,

    /// Resource scope name is empty.
    EmptyResourceScope,

    /// Resource label is empty.
    EmptyResourceLabel,

    /// Memory allocation/reservation failed.
    AllocationFailure,
}

impl fmt::Display for ResourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArithmeticOverflow => {
                formatter.write_str("resource arithmetic overflow")
            }
            Self::InsufficientQuantity => {
                formatter.write_str("resource quantity is insufficient")
            }
            Self::IndeterminateOperation => {
                formatter.write_str(
                    "resource operation has no determinate result",
                )
            }
            Self::InvalidRange => {
                formatter.write_str("resource range is invalid")
            }
            Self::EmptyIntersection => {
                formatter.write_str(
                    "resource ranges have no non-empty intersection",
                )
            }
            Self::EmptyResourceNamespace => {
                formatter.write_str("resource namespace cannot be empty")
            }
            Self::EmptyResourceName => {
                formatter.write_str("resource name cannot be empty")
            }
            Self::EmptyResourceScope => {
                formatter.write_str("resource scope cannot be empty")
            }
            Self::EmptyResourceLabel => {
                formatter.write_str("resource label cannot be empty")
            }
            Self::AllocationFailure => {
                formatter.write_str("resource collection allocation failed")
            }
        }
    }
}

impl std::error::Error for ResourceError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_quantity_is_exact() {
        let quantity = ResourceQuantity::finite(42);

        assert_eq!(quantity.as_finite(), Some(42));
        assert!(!quantity.is_unbounded());
    }

    #[test]
    fn unbounded_is_semantic_not_numeric() {
        let quantity = ResourceQuantity::unbounded();

        assert!(quantity.is_unbounded());
        assert!(quantity.at_least(u64::MAX));
        assert!(quantity.exceeds(u64::MAX));
    }

    #[test]
    fn finite_addition_is_checked() {
        assert_eq!(
            ResourceQuantity::finite(10)
                .checked_add(ResourceQuantity::finite(20))
                .expect("addition should succeed"),
            ResourceQuantity::finite(30)
        );
    }

    #[test]
    fn finite_overflow_is_rejected() {
        assert_eq!(
            ResourceQuantity::finite(u64::MAX)
                .checked_add(ResourceQuantity::finite(1)),
            Err(ResourceError::ArithmeticOverflow)
        );
    }

    #[test]
    fn unbounded_addition_remains_unbounded() {
        assert_eq!(
            ResourceQuantity::unbounded()
                .checked_add(ResourceQuantity::finite(1))
                .expect("unbounded addition should succeed"),
            ResourceQuantity::Unbounded
        );
    }

    #[test]
    fn exact_range_accepts_only_exact_capacity() {
        let range = ResourceRange::exact(100);

        assert!(!range.accepts(99));
        assert!(range.accepts(100));
        assert!(!range.accepts(101));
    }

    #[test]
    fn minimum_range_has_no_semantic_upper_limit() {
        let range = ResourceRange::at_least(100);

        assert!(range.accepts(100));
        assert!(range.accepts(u64::MAX));
        assert!(range.is_unbounded());
    }

    #[test]
    fn invalid_range_is_rejected() {
        assert_eq!(
            ResourceRange::between(10, 9),
            Err(ResourceError::InvalidRange)
        );
    }

    #[test]
    fn ranges_intersect_correctly() {
        let first =
            ResourceRange::between(10, 100).expect("valid range");
        let second =
            ResourceRange::between(50, 200).expect("valid range");

        let intersection =
            first.intersect(second).expect("ranges should intersect");

        assert_eq!(intersection.minimum(), 50);
        assert_eq!(
            intersection.maximum(),
            ResourceQuantity::Finite(100)
        );
    }

    #[test]
    fn resource_kinds_are_extensible() {
        let custom_namespace =
            ResourceNamespace::new("neutral_atom")
                .expect("namespace should be valid");

        let custom_name =
            ResourceName::new("atom_sites")
                .expect("resource name should be valid");

        let kind = ResourceKind::new(
            custom_namespace,
            custom_name,
        );

        assert_eq!(
            kind.qualified_name(),
            "neutral_atom/atom_sites"
        );
    }

    #[test]
    fn builtin_logical_qubits_are_stable() {
        let kind = ResourceKind::logical_qubits();

        assert_eq!(
            kind.qualified_name(),
            "zamani/logical_qubits"
        );
    }

    #[test]
    fn logical_qubit_scope_uses_canonical_ir_identity() {
        let qubit = QubitId::new(7);
        let scope = ResourceScope::logical_qubit(qubit);

        assert_eq!(scope.logical_qubit_id(), Some(qubit));
    }

    #[test]
    fn requirement_accepts_sufficient_capacity() {
        let requirement =
            ResourceRequirement::at_least(
                ResourceKind::logical_qubits(),
                100,
            );

        assert!(requirement.accepts(100));
        assert!(requirement.accepts(u64::MAX));
        assert!(!requirement.accepts(99));
    }

    #[test]
    fn finite_capacity_satisfies_requirement() {
        let requirement =
            ResourceRequirement::at_least(
                ResourceKind::logical_qubits(),
                100,
            );

        let capacity =
            ResourceCapacity::finite(
                ResourceKind::logical_qubits(),
                128,
            );

        assert!(capacity.satisfies(&requirement));
    }

    #[test]
    fn insufficient_capacity_is_rejected() {
        let requirement =
            ResourceRequirement::at_least(
                ResourceKind::logical_qubits(),
                100,
            );

        let capacity =
            ResourceCapacity::finite(
                ResourceKind::logical_qubits(),
                99,
            );

        assert!(!capacity.satisfies(&requirement));
    }

    #[test]
    fn mismatched_resource_kinds_are_rejected() {
        let requirement =
            ResourceRequirement::exact(
                ResourceKind::logical_qubits(),
                100,
            );

        let capacity =
            ResourceCapacity::finite(
                ResourceKind::physical_qubits(),
                100,
            );

        assert!(!capacity.satisfies(&requirement));
    }

    #[test]
    fn mismatched_scopes_are_rejected() {
        let qubit = QubitId::new(3);

        let requirement =
            ResourceRequirement::exact(
                ResourceKind::logical_qubits(),
                1,
            )
            .for_logical_qubit(qubit);

        let capacity =
            ResourceCapacity::finite(
                ResourceKind::logical_qubits(),
                1,
            );

        assert!(!capacity.satisfies(&requirement));
    }

    #[test]
    fn matching_logical_qubit_scope_is_accepted() {
        let qubit = QubitId::new(3);

        let requirement =
            ResourceRequirement::exact(
                ResourceKind::logical_qubits(),
                1,
            )
            .for_logical_qubit(qubit);

        let capacity =
            ResourceCapacity::finite(
                ResourceKind::logical_qubits(),
                1,
            )
            .with_scope(ResourceScope::logical_qubit(qubit));

        assert!(capacity.satisfies(&requirement));
    }

    #[test]
    fn requirements_are_additive() {
        let mut requirements =
            QuantumResourceRequirements::new();

        requirements
            .require_at_least(
                ResourceKind::logical_qubits(),
                100,
            )
            .expect("requirement should be added");

        requirements
            .require_at_least(
                ResourceKind::logical_qubits(),
                200,
            )
            .expect("requirement should be added");

        let minimum = requirements
            .minimum_required(
                &ResourceKind::logical_qubits(),
                &ResourceScope::Global,
            )
            .expect("minimum should calculate");

        assert_eq!(
            minimum,
            ResourceQuantity::Finite(300)
        );
    }

    #[test]
    fn aggregate_capacity_is_checked_without_fixed_machine_limit() {
        let mut requirements =
            QuantumResourceRequirements::new();

        requirements
            .require_at_least(
                ResourceKind::logical_qubits(),
                4097,
            )
            .expect("requirement should be added");

        assert!(
            requirements.accepts_capacity(
                &ResourceKind::logical_qubits(),
                &ResourceScope::Global,
                u64::MAX,
            )
        );
    }

    #[test]
    fn unbounded_requirement_has_no_architectural_ceiling() {
        let requirement =
            ResourceRequirement::at_least(
                ResourceKind::logical_qubits(),
                u64::MAX,
            );

        assert!(requirement.accepts(u64::MAX));
    }

    #[test]
    fn custom_resources_do_not_require_core_ir_changes() {
        let namespace =
            ResourceNamespace::new("future_quantum_architecture")
                .expect("namespace should be valid");

        let name =
            ResourceName::new("future_resource")
                .expect("name should be valid");

        let kind = ResourceKind::new(namespace, name);

        let requirement =
            ResourceRequirement::at_least(kind.clone(), 1);

        let capacity =
            ResourceCapacity::finite(kind, 1);

        assert!(capacity.satisfies(&requirement));
    }

    #[test]
    fn empty_collection_is_valid() {
        let requirements =
            QuantumResourceRequirements::new();

        assert!(requirements.is_empty());
        assert_eq!(requirements.len(), 0);
        assert!(requirements.validate().is_ok());
    }

    #[test]
    fn merging_requirements_preserves_semantics() {
        let mut first =
            QuantumResourceRequirements::new();

        let mut second =
            QuantumResourceRequirements::new();

        first
            .require_at_least(
                ResourceKind::logical_qubits(),
                10,
            )
            .expect("requirement should be added");

        second
            .require_at_least(
                ResourceKind::logical_qubits(),
                20,
            )
            .expect("requirement should be added");

        first
            .merge(&second)
            .expect("merge should succeed");

        let minimum = first
            .minimum_required(
                &ResourceKind::logical_qubits(),
                &ResourceScope::Global,
            )
            .expect("minimum should calculate");

        assert_eq!(
            minimum,
            ResourceQuantity::Finite(30)
        );
    }
}