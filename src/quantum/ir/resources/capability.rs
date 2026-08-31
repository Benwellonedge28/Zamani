//! Zamani Quantum IR — Capability Model
//!
//! Production-grade, target-independent capability vocabulary and
//! capability-requirement model for the Zamani Quantum IR.
//!
//! # Architectural role
//!
//! This module answers:
//!
//! > What capabilities does this quantum program require, and what
//! > capabilities can an abstract target declare that it provides?
//!
//! It deliberately does NOT answer:
//!
//! - which hardware device is selected;
//! - which physical qubits are allocated;
//! - how logical qubits are routed;
//! - how operations are scheduled;
//! - which calibration is selected;
//! - how pulses are synthesized;
//! - how a backend is contacted;
//! - how a simulator represents quantum state;
//! - how a QEC decoder works;
//! - how an optimization pass works.
//!
//! Those responsibilities belong to downstream modules.
//!
//! # Design principles
//!
//! This module is designed around the Zamani universal-program principle:
//!
//! ```text
//!                 Zamani program
//!                       │
//!                       ▼
//!                canonical IR
//!                       │
//!                       ▼
//!             capability requirements
//!                       │
//!          ┌────────────┴────────────┐
//!          │                         │
//!          ▼                         ▼
//!     target capability        target resources
//!          │                         │
//!          └────────────┬────────────┘
//!                       ▼
//!                compatibility
//!                       │
//!                       ▼
//!              target-specific lowering
//! ```
//!
//! A capability is therefore a semantic contract, not a vendor identifier.
//!
//! The same capability can be provided by many different technologies:
//!
//! ```text
//! mid_circuit_measurement
//! dynamic_classical_control
//! arbitrary_single_qubit_rotation
//! pulse_control
//! logical_qubits
//! fault_tolerant_execution
//! distributed_quantum_operation
//! ```
//!
//! No capability name is tied to IBM, IonQ, Quantinuum, Rigetti, D-Wave,
//! neutral atoms, superconducting qubits, trapped ions, photonics, or any
//! other particular implementation.
//!
//! # Scalability
//!
//! There is deliberately NO:
//!
//! - maximum capability count;
//! - maximum qubit count;
//! - fixed number of capability kinds;
//! - fixed number of capability properties;
//! - fixed hardware architecture;
//! - fixed topology;
//! - fixed operation universe.
//!
//! Collections grow according to available memory and explicit IR/resource
//! policies.
//!
//! `u64` is used for semantic quantities where a finite integer quantity is
//! required. It is not a machine-size ceiling. Unbounded quantities are
//! represented explicitly through `ResourceQuantity::Unbounded`.
//!
//! # Canonical qubit integration
//!
//! Logical qubit-scoped capabilities use:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! Physical-qubit identities are intentionally NOT required by the canonical
//! capability requirement model. Physical placement belongs to mapping,
//! routing, and hardware layers.
//!
//! # Relationship with `resource.rs`
//!
//! `resource.rs` owns generic resource quantities and resource requirements.
//!
//! This module owns capability semantics.
//!
//! Therefore:
//!
//! ```text
//! resource.rs
//!     ResourceQuantity
//!     ResourceRange
//!     ResourceKind
//!     ResourceRequirement
//!
//! capability.rs
//!     CapabilityId
//!     Capability
//!     CapabilityRequirement
//!     CapabilityRequirements
//!     CapabilitySet
//!     CapabilityScope
//!     CapabilityConstraint
//! ```
//!
//! Capability quantities may reuse `ResourceQuantity` and `ResourceRange`.
//!
//! This prevents two independent definitions of finite/unbounded resource
//! semantics from appearing in the IR.
//!
//! # Dependency boundary
//!
//! Allowed dependencies:
//!
//! ```text
//! quantum::ir::qubit
//! quantum::ir::identity
//! quantum::ir::resource
//! ```
//!
//! This module must not depend on:
//!
//! ```text
//! frontend
//! optimization
//! routing
//! scheduling
//! hardware
//! simulator
//! qec implementation
//! backend execution
//! ```
//!
//! Those systems may consume this module.
//!
//! # Versioning
//!
//! Capability identifiers are stable semantic identifiers.
//!
//! A capability's implementation/version is represented separately from its
//! identifier so that adding a new implementation version does not require
//! renaming the capability.
//!
//! # Serialization
//!
//! The data structures intentionally use deterministic representations:
//!
//! - `BTreeMap` instead of unordered maps;
//! - explicit strings for namespaces and names;
//! - ordered identifiers;
//! - normalized qubit scopes.
//!
//! This allows the canonical serialization layer to serialize this module
//! deterministically without depending on hash-map iteration order.
//!
//! # Hashing
//!
//! All semantic fields are retained in ordinary data structures so the
//! canonical hashing layer can include them in a stable representation.
//!
//! Nondeterministic runtime information is not stored here.
//!
//! # Safety
//!
//! No unsafe code is permitted.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::super::identity::OperationId;
use super::super::qubit::QubitId;
use super::super::resource::{
    ResourceError,
    ResourceQuantity,
    ResourceRange,
};

// =============================================================================
// Capability identifier
// =============================================================================

/// Stable namespace-qualified capability identifier.
///
/// A capability ID is semantic vocabulary, not a hardware identifier.
///
/// Examples:
///
/// ```text
/// zamani.quantum.mid_circuit_measurement
/// zamani.quantum.dynamic_control
/// zamani.quantum.arbitrary_rotation
/// zamani.pulse.control
/// zamani.ft.logical_execution
/// ```
///
/// Vendor-specific capabilities should use their own namespace, for example:
///
/// ```text
/// vendor.example.special_operation
/// ```
///
/// The canonical IR remains independent of that vendor.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CapabilityId {
    namespace: String,
    name: String,
}

impl CapabilityId {
    /// Creates a validated capability identifier.
    ///
    /// Both namespace and name must be non-empty after trimming.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, CapabilityError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_identifier_component(&namespace, "namespace")?;
        validate_identifier_component(&name, "name")?;

        Ok(Self { namespace, name })
    }

    /// Creates an identifier from a single qualified string.
    ///
    /// The final `.` separates namespace and name:
    ///
    /// ```text
    /// zamani.quantum.mid_circuit_measurement
    /// ```
    ///
    /// becomes:
    ///
    /// ```text
    /// namespace = zamani.quantum
    /// name      = mid_circuit_measurement
    /// ```
    pub fn parse(qualified: impl Into<String>) -> Result<Self, CapabilityError> {
        let qualified = qualified.into();

        let separator = qualified
            .rfind('.')
            .ok_or(CapabilityError::MissingNamespaceSeparator)?;

        if separator == 0 || separator + 1 >= qualified.len() {
            return Err(CapabilityError::InvalidIdentifier);
        }

        Self::new(
            qualified[..separator].to_owned(),
            qualified[separator + 1..].to_owned(),
        )
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the unqualified capability name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the fully qualified capability name.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}.{}", self.namespace, self.name)
    }
}

impl fmt::Display for CapabilityId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}", self.namespace, self.name)
    }
}

// =============================================================================
// Capability version
// =============================================================================

/// Semantic capability version.
///
/// Capability identity and capability version are intentionally separate.
///
/// This allows:
///
/// ```text
/// zamani.quantum.dynamic_control
///     version 1
///     version 2
///     version 3
/// ```
///
/// without creating three unrelated capability names.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CapabilityVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl CapabilityVersion {
    /// Creates a capability version.
    #[must_use]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major component.
    #[must_use]
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Returns the minor component.
    #[must_use]
    pub const fn minor(self) -> u16 {
        self.minor
    }

    /// Returns the patch component.
    #[must_use]
    pub const fn patch(self) -> u16 {
        self.patch
    }
}

impl fmt::Display for CapabilityVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

// =============================================================================
// Version constraint
// =============================================================================

/// Constraint applied to a capability version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VersionConstraint {
    /// Any version is accepted.
    Any,

    /// Exact version.
    Exact(CapabilityVersion),

    /// Required version or newer.
    AtLeast(CapabilityVersion),

    /// Required version or older.
    AtMost(CapabilityVersion),

    /// Version must be inside the inclusive interval.
    Between {
        /// Minimum accepted version.
        min: CapabilityVersion,

        /// Maximum accepted version.
        max: CapabilityVersion,
    },
}

impl VersionConstraint {
    /// Creates an inclusive version range.
    pub const fn between(
        min: CapabilityVersion,
        max: CapabilityVersion,
    ) -> Result<Self, CapabilityError> {
        if min > max {
            return Err(CapabilityError::InvalidVersionRange);
        }

        Ok(Self::Between { min, max })
    }

    /// Tests whether a version satisfies this constraint.
    #[must_use]
    pub const fn matches(self, version: CapabilityVersion) -> bool {
        match self {
            Self::Any => true,
            Self::Exact(required) => version == required,
            Self::AtLeast(required) => version >= required,
            Self::AtMost(required) => version <= required,
            Self::Between { min, max } => version >= min && version <= max,
        }
    }
}

// =============================================================================
// Capability scope
// =============================================================================

/// Scope at which a capability applies.
///
/// The scope is semantic and does not perform placement.
///
/// `LogicalQubit` and `LogicalQubits` use the canonical
/// `quantum::ir::qubit::QubitId`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityScope {
    /// Capability applies to the complete target/program context.
    Global,

    /// Capability applies to a specific logical qubit.
    LogicalQubit(QubitId),

    /// Capability applies to a set of logical qubits.
    ///
    /// The constructor normalizes the set deterministically.
    LogicalQubits(Vec<QubitId>),

    /// Capability applies to a particular IR operation.
    Operation(OperationId),
}

impl CapabilityScope {
    /// Creates a normalized logical-qubit-set scope.
    ///
    /// Empty sets are rejected because an empty scope has no semantic target.
    pub fn logical_qubits<I>(qubits: I) -> Result<Self, CapabilityError>
    where
        I: IntoIterator<Item = QubitId>,
    {
        let mut ordered = BTreeSet::new();

        for qubit in qubits {
            ordered.insert(qubit);
        }

        if ordered.is_empty() {
            return Err(CapabilityError::EmptyQubitScope);
        }

        Ok(Self::LogicalQubits(ordered.into_iter().collect()))
    }

    /// Returns whether this scope is global.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        matches!(self, Self::Global)
    }

    /// Returns the explicitly scoped logical qubits, if any.
    #[must_use]
    pub fn logical_qubits(&self) -> Option<&[QubitId]> {
        match self {
            Self::LogicalQubits(qubits) => Some(qubits),
            _ => None,
        }
    }
}

// =============================================================================
// Capability value
// =============================================================================

/// Deterministic, target-independent capability property value.
///
/// Floating-point values are deliberately not included because capability
/// properties should have deterministic equality and hashing semantics.
///
/// Numeric physical values should use an exact integer/rational representation
/// in the appropriate IR domain or an explicitly encoded extension.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityValue {
    /// Boolean property.
    Boolean(bool),

    /// Unsigned integer property.
    Unsigned(u64),

    /// UTF-8 textual property.
    Text(String),

    /// Capability version property.
    Version(CapabilityVersion),

    /// Explicitly present but implementation-defined value.
    Opaque(String),
}

impl CapabilityValue {
    /// Returns the contained boolean when applicable.
    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the contained unsigned integer when applicable.
    #[must_use]
    pub const fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Unsigned(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the contained text when applicable.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Text(value) | Self::Opaque(value) => Some(value),
            _ => None,
        }
    }
}

// =============================================================================
// Capability support state
// =============================================================================

/// Degree of support a target declares for a capability.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilitySupport {
    /// Capability is not provided.
    Unsupported,

    /// Capability is natively/directly provided.
    Supported,

    /// Capability is available only conditionally.
    ///
    /// A strict `Required` requirement does not accept this state unless the
    /// requirement explicitly allows conditional support.
    Conditional,
}

impl CapabilitySupport {
    /// Returns whether support is unconditional.
    #[must_use]
    pub const fn is_unconditional(self) -> bool {
        matches!(self, Self::Supported)
    }

    /// Returns whether any form of support is declared.
    #[must_use]
    pub const fn is_available(self) -> bool {
        !matches!(self, Self::Unsupported)
    }
}

// =============================================================================
// Capability
// =============================================================================

/// A target-independent declaration of one provided capability.
///
/// Hardware adapters may construct these declarations from hardware
/// descriptions, but this type itself has no dependency on hardware code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capability {
    id: CapabilityId,
    version: CapabilityVersion,
    support: CapabilitySupport,
    scope: CapabilityScope,
    quantity: ResourceQuantity,
    properties: BTreeMap<String, CapabilityValue>,
}

impl Capability {
    /// Creates an unquantified supported capability.
    pub fn supported(id: CapabilityId) -> Self {
        Self {
            id,
            version: CapabilityVersion::default(),
            support: CapabilitySupport::Supported,
            scope: CapabilityScope::Global,
            quantity: ResourceQuantity::Unbounded,
            properties: BTreeMap::new(),
        }
    }

    /// Creates an unsupported capability declaration.
    pub fn unsupported(id: CapabilityId) -> Self {
        Self {
            id,
            version: CapabilityVersion::default(),
            support: CapabilitySupport::Unsupported,
            scope: CapabilityScope::Global,
            quantity: ResourceQuantity::Finite(0),
            properties: BTreeMap::new(),
        }
    }

    /// Creates a conditionally supported capability.
    pub fn conditional(id: CapabilityId) -> Self {
        Self {
            id,
            version: CapabilityVersion::default(),
            support: CapabilitySupport::Conditional,
            scope: CapabilityScope::Global,
            quantity: ResourceQuantity::Unbounded,
            properties: BTreeMap::new(),
        }
    }

    /// Sets the capability version.
    #[must_use]
    pub const fn with_version(mut self, version: CapabilityVersion) -> Self {
        self.version = version;
        self
    }

    /// Sets the support state.
    #[must_use]
    pub const fn with_support(mut self, support: CapabilitySupport) -> Self {
        self.support = support;
        self
    }

    /// Sets the semantic scope.
    #[must_use]
    pub fn with_scope(mut self, scope: CapabilityScope) -> Self {
        self.scope = scope;
        self
    }

    /// Sets the finite/unbounded capability quantity.
    #[must_use]
    pub const fn with_quantity(mut self, quantity: ResourceQuantity) -> Self {
        self.quantity = quantity;
        self
    }

    /// Adds or replaces a capability property.
    ///
    /// Empty property names are rejected.
    pub fn with_property(
        mut self,
        name: impl Into<String>,
        value: CapabilityValue,
    ) -> Result<Self, CapabilityError> {
        let name = name.into();

        validate_identifier_component(&name, "property name")?;

        self.properties.insert(name, value);
        Ok(self)
    }

    /// Returns the capability ID.
    #[must_use]
    pub fn id(&self) -> &CapabilityId {
        &self.id
    }

    /// Returns the capability version.
    #[must_use]
    pub const fn version(&self) -> CapabilityVersion {
        self.version
    }

    /// Returns the support state.
    #[must_use]
    pub const fn support(&self) -> CapabilitySupport {
        self.support
    }

    /// Returns the scope.
    #[must_use]
    pub fn scope(&self) -> &CapabilityScope {
        &self.scope
    }

    /// Returns the declared quantity.
    #[must_use]
    pub const fn quantity(&self) -> ResourceQuantity {
        self.quantity
    }

    /// Returns capability properties.
    #[must_use]
    pub fn properties(&self) -> &BTreeMap<String, CapabilityValue> {
        &self.properties
    }

    /// Returns one property.
    #[must_use]
    pub fn property(&self, name: &str) -> Option<&CapabilityValue> {
        self.properties.get(name)
    }
}

// =============================================================================
// Capability constraint
// =============================================================================

/// Constraint applied to one capability.
///
/// This is intentionally more expressive than a boolean because real quantum
/// targets differ in quantitative and versioned capabilities.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CapabilityConstraint {
    /// Capability must be unconditionally supported.
    Required,

    /// Capability may be supported but is not mandatory.
    Preferred,

    /// Capability must not be available.
    Forbidden,

    /// Capability must be available, including conditional support.
    Available,

    /// Capability must provide at least this quantity.
    AtLeast(u64),

    /// Capability must provide no more than this quantity.
    AtMost(u64),

    /// Capability quantity must satisfy the specified range.
    Quantity(ResourceRange),

    /// A specific property must exist.
    PropertyExists(String),

    /// A specific property must equal a value.
    PropertyEquals {
        /// Property name.
        name: String,

        /// Required property value.
        value: CapabilityValue,
    },

    /// Capability must provide the requested version.
    Version(VersionConstraint),
}

impl CapabilityConstraint {
    /// Creates a property-existence constraint.
    pub fn property_exists(name: impl Into<String>) -> Result<Self, CapabilityError> {
        let name = name.into();

        validate_identifier_component(&name, "property name")?;

        Ok(Self::PropertyExists(name))
    }

    /// Creates a property-equality constraint.
    pub fn property_equals(
        name: impl Into<String>,
        value: CapabilityValue,
    ) -> Result<Self, CapabilityError> {
        let name = name.into();

        validate_identifier_component(&name, "property name")?;

        Ok(Self::PropertyEquals { name, value })
    }
}

// =============================================================================
// Capability requirement
// =============================================================================

/// One capability requirement attached to a program, operation, or logical
/// resource.
///
/// A requirement is a semantic statement. It does not select a target.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapabilityRequirement {
    id: CapabilityId,
    constraint: CapabilityConstraint,
    version: VersionConstraint,
    scope: CapabilityScope,
}

impl CapabilityRequirement {
    /// Creates a required global capability.
    #[must_use]
    pub fn required(id: CapabilityId) -> Self {
        Self {
            id,
            constraint: CapabilityConstraint::Required,
            version: VersionConstraint::Any,
            scope: CapabilityScope::Global,
        }
    }

    /// Creates a preferred global capability.
    #[must_use]
    pub fn preferred(id: CapabilityId) -> Self {
        Self {
            id,
            constraint: CapabilityConstraint::Preferred,
            version: VersionConstraint::Any,
            scope: CapabilityScope::Global,
        }
    }

    /// Creates a forbidden global capability.
    #[must_use]
    pub fn forbidden(id: CapabilityId) -> Self {
        Self {
            id,
            constraint: CapabilityConstraint::Forbidden,
            version: VersionConstraint::Any,
            scope: CapabilityScope::Global,
        }
    }

    /// Creates an availability requirement.
    #[must_use]
    pub fn available(id: CapabilityId) -> Self {
        Self {
            id,
            constraint: CapabilityConstraint::Available,
            version: VersionConstraint::Any,
            scope: CapabilityScope::Global,
        }
    }

    /// Creates a quantity requirement.
    #[must_use]
    pub fn quantity(
        id: CapabilityId,
        quantity: ResourceRange,
    ) -> Self {
        Self {
            id,
            constraint: CapabilityConstraint::Quantity(quantity),
            version: VersionConstraint::Any,
            scope: CapabilityScope::Global,
        }
    }

    /// Sets the constraint.
    #[must_use]
    pub const fn with_constraint(
        mut self,
        constraint: CapabilityConstraint,
    ) -> Self {
        self.constraint = constraint;
        self
    }

    /// Sets the required capability version.
    #[must_use]
    pub const fn with_version(
        mut self,
        version: VersionConstraint,
    ) -> Self {
        self.version = version;
        self
    }

    /// Sets the capability scope.
    #[must_use]
    pub fn with_scope(mut self, scope: CapabilityScope) -> Self {
        self.scope = scope;
        self
    }

    /// Returns the capability ID.
    #[must_use]
    pub fn id(&self) -> &CapabilityId {
        &self.id
    }

    /// Returns the constraint.
    #[must_use]
    pub const fn constraint(&self) -> &CapabilityConstraint {
        &self.constraint
    }

    /// Returns the version constraint.
    #[must_use]
    pub const fn version(&self) -> VersionConstraint {
        self.version
    }

    /// Returns the scope.
    #[must_use]
    pub fn scope(&self) -> &CapabilityScope {
        &self.scope
    }
}

// =============================================================================
// Capability requirements collection
// =============================================================================

/// Collection of capability requirements for a program or compilation unit.
///
/// Requirements are retained in declaration order because declaration order
/// can be useful for diagnostics, while matching itself is deterministic and
/// independent of insertion order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CapabilityRequirements {
    requirements: Vec<CapabilityRequirement>,
}

impl CapabilityRequirements {
    /// Creates an empty requirement collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            requirements: Vec::new(),
        }
    }

    /// Adds one requirement.
    pub fn push(
        &mut self,
        requirement: CapabilityRequirement,
    ) {
        self.requirements.push(requirement);
    }

    /// Returns all requirements.
    #[must_use]
    pub fn as_slice(&self) -> &[CapabilityRequirement] {
        &self.requirements
    }

    /// Returns the number of requirements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.requirements.len()
    }

    /// Returns whether there are no requirements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }

    /// Returns an iterator over requirements.
    pub fn iter(&self) -> impl Iterator<Item = &CapabilityRequirement> {
        self.requirements.iter()
    }

    /// Extends this collection from another collection.
    pub fn extend(&mut self, other: &Self) {
        self.requirements
            .extend(other.requirements.iter().cloned());
    }

    /// Checks every requirement against a capability set.
    ///
    /// This method performs only capability compatibility.
    ///
    /// It does not perform:
    ///
    /// - hardware discovery;
    /// - routing;
    /// - allocation;
    /// - scheduling;
    /// - calibration;
    /// - execution.
    #[must_use]
    pub fn satisfied_by(&self, capabilities: &CapabilitySet) -> bool {
        self.requirements
            .iter()
            .all(|requirement| capabilities.satisfies(requirement))
    }

    /// Returns all requirements that are not satisfied.
    ///
    /// This is intended for compiler diagnostics and target-selection logic.
    #[must_use]
    pub fn unsatisfied_by<'a>(
        &'a self,
        capabilities: &'a CapabilitySet,
    ) -> Vec<&'a CapabilityRequirement> {
        self.requirements
            .iter()
            .filter(|requirement| !capabilities.satisfies(requirement))
            .collect()
    }
}

// =============================================================================
// Capability key
// =============================================================================

/// Deterministic key for a scoped capability.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct CapabilityKey {
    id: CapabilityId,
    scope: CapabilityScope,
}

impl CapabilityKey {
    fn new(capability: &Capability) -> Self {
        Self {
            id: capability.id().clone(),
            scope: capability.scope().clone(),
        }
    }
}

// =============================================================================
// Capability set
// =============================================================================

/// Deterministic collection of capabilities supplied by an abstract target.
///
/// A hardware adapter can translate its device description into this structure
/// without importing hardware types into the canonical IR.
///
/// `BTreeMap` is deliberate:
///
/// - deterministic iteration;
/// - deterministic serialization order;
/// - deterministic diagnostics;
/// - deterministic hashing input;
/// - no dependence on randomized hash state.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CapabilitySet {
    capabilities: BTreeMap<CapabilityKey, Capability>,
}

impl CapabilitySet {
    /// Creates an empty capability set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            capabilities: BTreeMap::new(),
        }
    }

    /// Adds or replaces a capability with the same ID and scope.
    ///
    /// Replacing a capability is intentional: hardware discovery or target
    /// refinement may progressively improve a capability declaration.
    pub fn insert(&mut self, capability: Capability) {
        let key = CapabilityKey::new(&capability);

        self.capabilities.insert(key, capability);
    }

    /// Returns the number of capability declarations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.capabilities.len()
    }

    /// Returns whether the set contains no capability declarations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.capabilities.is_empty()
    }

    /// Returns all capabilities in deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = &Capability> {
        self.capabilities.values()
    }

    /// Returns a capability matching an ID and scope.
    #[must_use]
    pub fn get(
        &self,
        id: &CapabilityId,
        scope: &CapabilityScope,
    ) -> Option<&Capability> {
        self.capabilities.get(&CapabilityKey {
            id: id.clone(),
            scope: scope.clone(),
        })
    }

    /// Returns all capabilities with the specified ID.
    pub fn by_id<'a>(
        &'a self,
        id: &'a CapabilityId,
    ) -> impl Iterator<Item = &'a Capability> {
        self.capabilities
            .values()
            .filter(move |capability| capability.id() == id)
    }

    /// Checks whether a capability requirement is satisfied.
    #[must_use]
    pub fn satisfies(
        &self,
        requirement: &CapabilityRequirement,
    ) -> bool {
        let candidates = self
            .capabilities
            .values()
            .filter(|capability| {
                capability.id() == requirement.id()
                    && capability.scope() == requirement.scope()
            });

        match requirement.constraint() {
            CapabilityConstraint::Forbidden => candidates
                .all(|capability| {
                    matches!(
                        capability.support(),
                        CapabilitySupport::Unsupported
                    )
                }),

            CapabilityConstraint::Preferred => candidates
                .any(|capability| capability.support().is_available()),

            _ => candidates.any(|capability| {
                capability_matches_requirement(capability, requirement)
            }),
        }
    }

    /// Returns all capabilities as a deterministic vector.
    #[must_use]
    pub fn to_vec(&self) -> Vec<&Capability> {
        self.capabilities.values().collect()
    }
}

// =============================================================================
// Capability compatibility report
// =============================================================================

/// Result of checking a set of capability requirements.
///
/// This is intentionally lightweight and contains no target object.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapabilityCompatibility {
    satisfied: bool,
    unsatisfied_count: usize,
}

impl CapabilityCompatibility {
    /// Creates a compatibility report.
    #[must_use]
    pub const fn new(
        satisfied: bool,
        unsatisfied_count: usize,
    ) -> Self {
        Self {
            satisfied,
            unsatisfied_count,
        }
    }

    /// Returns whether all requirements are satisfied.
    #[must_use]
    pub const fn is_satisfied(&self) -> bool {
        self.satisfied
    }

    /// Returns the number of unsatisfied requirements.
    #[must_use]
    pub const fn unsatisfied_count(&self) -> usize {
        self.unsatisfied_count
    }
}

impl CapabilitySet {
    /// Produces a deterministic compatibility report.
    #[must_use]
    pub fn compatibility(
        &self,
        requirements: &CapabilityRequirements,
    ) -> CapabilityCompatibility {
        let unsatisfied_count = requirements
            .requirements
            .iter()
            .filter(|requirement| !self.satisfies(requirement))
            .count();

        CapabilityCompatibility::new(
            unsatisfied_count == 0,
            unsatisfied_count,
        )
    }
}

// =============================================================================
// Resource-backed capability helpers
// =============================================================================

/// Creates a quantity-backed capability requirement from a generic resource
/// range.
///
/// This helper deliberately does not create a new resource type. It reuses
/// `ResourceRange` from `resource.rs`.
#[must_use]
pub fn quantity_requirement(
    id: CapabilityId,
    range: ResourceRange,
) -> CapabilityRequirement {
    CapabilityRequirement::quantity(id, range)
}

/// Creates an exact finite quantity requirement.
#[must_use]
pub fn exact_quantity_requirement(
    id: CapabilityId,
    amount: u64,
) -> CapabilityRequirement {
    CapabilityRequirement::quantity(
        id,
        ResourceRange::exact(amount),
    )
}

/// Creates a minimum quantity requirement.
#[must_use]
pub fn minimum_quantity_requirement(
    id: CapabilityId,
    amount: u64,
) -> CapabilityRequirement {
    CapabilityRequirement::quantity(
        id,
        ResourceRange::at_least(amount),
    )
}

// =============================================================================
// Internal matching
// =============================================================================

fn capability_matches_requirement(
    capability: &Capability,
    requirement: &CapabilityRequirement,
) -> bool {
    match requirement.constraint() {
        CapabilityConstraint::Required => {
            capability.support() == CapabilitySupport::Supported
                && requirement.version().matches(capability.version())
        }

        CapabilityConstraint::Preferred => {
            capability.support().is_available()
                && requirement.version().matches(capability.version())
        }

        CapabilityConstraint::Forbidden => {
            capability.support() == CapabilitySupport::Unsupported
        }

        CapabilityConstraint::Available => {
            capability.support().is_available()
                && requirement.version().matches(capability.version())
        }

        CapabilityConstraint::AtLeast(amount) => {
            capability.support().is_available()
                && requirement.version().matches(capability.version())
                && quantity_at_least(capability.quantity(), *amount)
        }

        CapabilityConstraint::AtMost(amount) => {
            capability.support().is_available()
                && requirement.version().matches(capability.version())
                && quantity_at_most(capability.quantity(), *amount)
        }

        CapabilityConstraint::Quantity(range) => {
            capability.support().is_available()
                && requirement.version().matches(capability.version())
                && quantity_satisfies_range(
                    capability.quantity(),
                    *range,
                )
        }

        CapabilityConstraint::PropertyExists(name) => {
            capability.support().is_available()
                && requirement.version().matches(capability.version())
                && capability.property(name).is_some()
        }

        CapabilityConstraint::PropertyEquals { name, value } => {
            capability.support().is_available()
                && requirement.version().matches(capability.version())
                && capability.property(name) == Some(value)
        }

        CapabilityConstraint::Version(version) => {
            capability.support().is_available()
                && version.matches(capability.version())
        }
    }
}

fn quantity_at_least(
    quantity: ResourceQuantity,
    minimum: u64,
) -> bool {
    match quantity {
        ResourceQuantity::Finite(value) => value >= minimum,
        ResourceQuantity::Unbounded => true,
    }
}

fn quantity_at_most(
    quantity: ResourceQuantity,
    maximum: u64,
) -> bool {
    match quantity {
        ResourceQuantity::Finite(value) => value <= maximum,
        ResourceQuantity::Unbounded => false,
    }
}

fn quantity_satisfies_range(
    quantity: ResourceQuantity,
    range: ResourceRange,
) -> bool {
    match quantity {
        ResourceQuantity::Finite(value) => range.accepts(value),
        ResourceQuantity::Unbounded => range.is_unbounded(),
    }
}

fn validate_identifier_component(
    value: &str,
    field: &'static str,
) -> Result<(), CapabilityError> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err(CapabilityError::EmptyIdentifierComponent(field));
    }

    if trimmed != value {
        return Err(CapabilityError::InvalidIdentifier);
    }

    if value.chars().any(char::is_whitespace) {
        return Err(CapabilityError::InvalidIdentifier);
    }

    Ok(())
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the capability model.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CapabilityError {
    /// Namespace/name/property component was empty.
    EmptyIdentifierComponent(&'static str),

    /// Identifier syntax was invalid.
    InvalidIdentifier,

    /// Qualified capability ID did not contain a namespace separator.
    MissingNamespaceSeparator,

    /// Capability version range had min > max.
    InvalidVersionRange,

    /// Logical-qubit capability scope was empty.
    EmptyQubitScope,

    /// A resource operation failed while constructing a capability contract.
    Resource(ResourceError),
}

impl From<ResourceError> for CapabilityError {
    fn from(error: ResourceError) -> Self {
        Self::Resource(error)
    }
}

impl fmt::Display for CapabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifierComponent(field) => {
                write!(formatter, "{field} cannot be empty")
            }

            Self::InvalidIdentifier => {
                formatter.write_str("invalid capability identifier")
            }

            Self::MissingNamespaceSeparator => {
                formatter.write_str(
                    "capability identifier requires a namespace separator",
                )
            }

            Self::InvalidVersionRange => {
                formatter.write_str(
                    "capability version range has minimum greater than maximum",
                )
            }

            Self::EmptyQubitScope => {
                formatter.write_str(
                    "logical-qubit capability scope cannot be empty",
                )
            }

            Self::Resource(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for CapabilityError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn capability_id(name: &str) -> CapabilityId {
        CapabilityId::new("zamani.quantum", name)
            .expect("test capability ID must be valid")
    }

    #[test]
    fn capability_ids_are_namespace_qualified() {
        let id = capability_id("dynamic_control");

        assert_eq!(id.namespace(), "zamani.quantum");
        assert_eq!(id.name(), "dynamic_control");
        assert_eq!(
            id.qualified_name(),
            "zamani.quantum.dynamic_control"
        );
        assert_eq!(
            id.to_string(),
            "zamani.quantum.dynamic_control"
        );
    }

    #[test]
    fn capability_ids_round_trip_through_parse() {
        let original = capability_id("mid_circuit_measurement");
        let parsed = CapabilityId::parse(original.to_string())
            .expect("qualified ID should parse");

        assert_eq!(parsed, original);
    }

    #[test]
    fn invalid_capability_ids_are_rejected() {
        assert!(CapabilityId::new("", "x").is_err());
        assert!(CapabilityId::new("zamani", "").is_err());
        assert!(CapabilityId::new("zamani quantum", "x").is_err());
        assert!(CapabilityId::parse("missing_namespace").is_err());
    }

    #[test]
    fn versions_are_ordered() {
        let v1 = CapabilityVersion::new(1, 0, 0);
        let v2 = CapabilityVersion::new(2, 0, 0);

        assert!(v2 > v1);
        assert!(VersionConstraint::AtLeast(v1).matches(v2));
        assert!(!VersionConstraint::AtMost(v1).matches(v2));
    }

    #[test]
    fn logical_qubit_scopes_are_normalized() {
        let scope = CapabilityScope::logical_qubits([
            QubitId::new(3),
            QubitId::new(1),
            QubitId::new(3),
            QubitId::new(2),
        ])
        .expect("scope should be valid");

        assert_eq!(
            scope.logical_qubits(),
            Some(
                &[
                    QubitId::new(1),
                    QubitId::new(2),
                    QubitId::new(3),
                ][..]
            )
        );
    }

    #[test]
    fn empty_logical_qubit_scope_is_rejected() {
        assert!(
            CapabilityScope::logical_qubits(
                std::iter::empty::<QubitId>()
            )
            .is_err()
        );
    }

    #[test]
    fn required_capability_matches_supported_capability() {
        let id = capability_id("dynamic_control");

        let mut target = CapabilitySet::new();

        target.insert(
            Capability::supported(id.clone())
                .with_version(CapabilityVersion::new(1, 2, 0)),
        );

        let mut requirements = CapabilityRequirements::new();

        requirements.push(
            CapabilityRequirement::required(id)
                .with_version(VersionConstraint::AtLeast(
                    CapabilityVersion::new(1, 0, 0),
                )),
        );

        assert!(requirements.satisfied_by(&target));
    }

    #[test]
    fn_required_capability_rejects_conditional_support() {
        let id = capability_id("dynamic_control");

        let mut target = CapabilitySet::new();

        target.insert(
            Capability::conditional(id.clone())
                .with_version(CapabilityVersion::new(1, 0, 0)),
        );

        let requirement = CapabilityRequirement::required(id);

        let mut requirements = CapabilityRequirements::new();
        requirements.push(requirement);

        assert!(!requirements.satisfied_by(&target));
    }

    #[test]
    fn_available_accepts_conditional_support() {
        let id = capability_id("dynamic_control");

        let mut target = CapabilitySet::new();

        target.insert(
            Capability::conditional(id.clone())
                .with_version(CapabilityVersion::new(1, 0, 0)),
        );

        let mut requirements = CapabilityRequirements::new();

        requirements.push(
            CapabilityRequirement::available(id),
        );

        assert!(requirements.satisfied_by(&target));
    }

    #[test]
    fn quantity_capabilities_scale_without_fixed_limits() {
        let id = capability_id("logical_qubits");

        let mut target = CapabilitySet::new();

        target.insert(
            Capability::supported(id.clone())
                .with_quantity(ResourceQuantity::Finite(1_000_000)),
        );

        let requirement = minimum_quantity_requirement(
            id,
            900_000,
        );

        let mut requirements = CapabilityRequirements::new();
        requirements.push(requirement);

        assert!(requirements.satisfied_by(&target));
    }

    #[test]
    fn unbounded_quantity_satisfies_minimum_requirement() {
        let id = capability_id("logical_qubits");

        let mut target = CapabilitySet::new();

        target.insert(
            Capability::supported(id.clone())
                .with_quantity(ResourceQuantity::Unbounded),
        );

        let requirement = minimum_quantity_requirement(
            id,
            u64::MAX,
        );

        let mut requirements = CapabilityRequirements::new();
        requirements.push(requirement);

        assert!(requirements.satisfied_by(&target));
    }

    #[test]
    fn unbounded_quantity_does_not_claim_finite_upper_bound() {
        let id = capability_id("logical_qubits");

        let mut target = CapabilitySet::new();

        target.insert(
            Capability::supported(id.clone())
                .with_quantity(ResourceQuantity::Unbounded),
        );

        let requirement = CapabilityRequirement::quantity(
            id,
            ResourceRange::between(1, 1_000)
                .expect("range should be valid"),
        );

        let mut requirements = CapabilityRequirements::new();
        requirements.push(requirement);

        assert!(!requirements.satisfied_by(&target));
    }

    #[test]
    fn properties_are_deterministic_and_matchable() {
        let id = capability_id("pulse_control");

        let capability = Capability::supported(id.clone())
            .with_property(
                "granularity",
                CapabilityValue::Unsigned(1),
            )
            .expect("property should be accepted");

        let mut target = CapabilitySet::new();
        target.insert(capability);

        let property_requirement =
            CapabilityRequirement::required(id.clone())
                .with_constraint(
                    CapabilityConstraint::PropertyEquals {
                        name: "granularity".to_owned(),
                        value: CapabilityValue::Unsigned(1),
                    },
                );

        assert!(target.satisfies(&property_requirement));

        let exists_requirement =
            CapabilityRequirement::required(id)
                .with_constraint(
                    CapabilityConstraint::PropertyExists(
                        "granularity".to_owned(),
                    ),
                );

        assert!(target.satisfies(&exists_requirement));
    }

    #[test]
    fn scoped_capabilities_do_not_leak_between_qubits() {
        let id = capability_id("special_operation");

        let mut target = CapabilitySet::new();

        target.insert(
            Capability::supported(id.clone())
                .with_scope(
                    CapabilityScope::LogicalQubit(
                        QubitId::new(0),
                    ),
                ),
        );

        let requirement_for_q0 =
            CapabilityRequirement::required(id.clone())
                .with_scope(
                    CapabilityScope::LogicalQubit(
                        QubitId::new(0),
                    ),
                );

        let requirement_for_q1 =
            CapabilityRequirement::required(id)
                .with_scope(
                    CapabilityScope::LogicalQubit(
                        QubitId::new(1),
                    ),
                );

        assert!(target.satisfies(&requirement_for_q0));
        assert!(!target.satisfies(&requirement_for_q1));
    }

    #[test]
    fn compatibility_report_counts_unsatisfied_requirements() {
        let dynamic = capability_id("dynamic_control");
        let pulse = capability_id("pulse_control");

        let mut target = CapabilitySet::new();

        target.insert(Capability::supported(dynamic.clone()));

        let mut requirements = CapabilityRequirements::new();

        requirements.push(
            CapabilityRequirement::required(dynamic),
        );

        requirements.push(
            CapabilityRequirement::required(pulse),
        );

        let report = target.compatibility(&requirements);

        assert!(!report.is_satisfied());
        assert_eq!(report.unsatisfied_count(), 1);
    }

    #[test]
    fn capability_set_is_deterministically_ordered() {
        let mut target = CapabilitySet::new();

        let b = capability_id("b");
        let a = capability_id("a");

        target.insert(Capability::supported(b));
        target.insert(Capability::supported(a));

        let names: Vec<&str> = target
            .iter()
            .map(|capability| capability.id().name())
            .collect();

        assert_eq!(names, vec!["a", "b"]);
    }

    #[test]
    fn preferred_capability_is_not_a_hard_requirement() {
        let id = capability_id("optional_acceleration");

        let mut requirements = CapabilityRequirements::new();

        requirements.push(
            CapabilityRequirement::preferred(id),
        );

        let target = CapabilitySet::new();

        assert!(!requirements.satisfied_by(&target));
    }

    #[test]
    fn forbidden_capability_accepts_absence() {
        let id = capability_id("forbidden_feature");

        let mut requirements = CapabilityRequirements::new();

        requirements.push(
            CapabilityRequirement::forbidden(id),
        );

        let target = CapabilitySet::new();

        assert!(requirements.satisfied_by(&target));
    }

    #[test]
    fn forbidden_capability_rejects_supported_capability() {
        let id = capability_id("forbidden_feature");

        let mut target = CapabilitySet::new();

        target.insert(Capability::supported(id.clone()));

        let mut requirements = CapabilityRequirements::new();

        requirements.push(
            CapabilityRequirement::forbidden(id),
        );

        assert!(!requirements.satisfied_by(&target));
    }
}