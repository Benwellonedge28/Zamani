//! Zamani Quantum Noise (ZQN)
//! Operation Context
//!
//! `src/quantum/zqn/operations/operation.rs`
//!
//! # Purpose
//!
//! This module defines the ZQN-side representation of an operation execution
//! context: the semantic operation identity and the resources/time/parameters
//! that determine where and when a noise model may apply.
//!
//! # Critical ownership boundary
//!
//! The canonical quantum operation remains owned by:
//!
//!     crate::quantum::ir
//!
//! In particular:
//!
//!     crate::quantum::ir::identity::OperationId
//!     crate::quantum::ir::qubit::QubitId
//!
//! are canonical identities.
//!
//! ZQN MUST NOT define another `OperationId` or `QubitId`.
//!
//! This module does not define the ideal gate semantics. It defines the
//! operation context consumed by the noise subsystem.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! quantum::ir
//!       |
//!       | canonical OperationId
//!       v
//! ZQN operations::operation
//!       |
//!       +-------------------+--------------------+
//!       |                   |                    |
//!       v                   v                    v
//! noise              calibration          characterization
//!       |                   |                    |
//!       +-------------------+--------------------+
//!                           |
//!                           v
//!                       simulation
//!                           |
//!                           v
//!                        runtime
//! ```
//!
//! # Design goals
//!
//! - no unsafe code;
//! - Rust 1.97 / 1.97.1 compatible;
//! - Rust 2021 compatible;
//! - no fixed number of qubits;
//! - no fixed gate set;
//! - no vendor-specific operation names;
//! - no hardware-specific limits;
//! - deterministic data representation;
//! - explicit validation;
//! - arbitrary operation arity;
//! - arbitrary resource count;
//! - extensible operation categories;
//! - support for gate, measurement, reset, preparation, idle, pulse,
//!   transport, analog, Hamiltonian, control and future operation classes;
//! - canonical `OperationId` and `QubitId` integration;
//! - no dependency on concrete ZQN channel/noise implementations;
//! - no hidden global state;
//! - no global RNG;
//! - no allocation performed merely by reading an operation;
//! - explicit resource and time semantics;
//! - suitable for serialization by a later ZQN I/O layer;
//! - suitable for deterministic hashing by a later ZQN/IR hashing layer.
//!
//! # Scalability
//!
//! There is intentionally no semantic maximum for:
//!
//! - number of operations;
//! - number of operands;
//! - number of qubits;
//! - number of resources;
//! - number of parameters;
//! - metadata entries;
//! - operation name length;
//! - duration magnitude.
//!
//! Concrete resource limits belong to an explicit runtime/resource policy,
//! not to this semantic type.
//!
//! "Infinity" therefore means that this abstraction does not impose a
//! machine-size ceiling. A concrete execution remains finite and is limited
//! only by the resources and policies of the execution environment.
//!
//! # Determinism
//!
//! All semantic collections in this module use deterministic ordering.
//! Metadata and parameters use `BTreeMap` rather than `HashMap` so that
//! iteration order is stable.
//!
//! This module contains no randomness and no mutable global state.
//!
//! # Security
//!
//! Constructors validate values that would otherwise make the semantic model
//! invalid, including non-finite numeric values and invalid identifiers.
//!
//! Resource exhaustion limits are deliberately NOT embedded here. They must be
//! supplied by higher-level resource policies so that semantic validity and
//! deployment policy remain separate concerns.
//!
//! # Integration contract
//!
//! Consumers:
//!
//! - `zqn::noise` uses operation context to select/apply noise;
//! - `zqn::channel` uses operation context when attaching channels;
//! - `zqn::fault` uses operation locations;
//! - `zqn::calibration` resolves calibration against operation/resource scope;
//! - `zqn::characterization` records operation contexts;
//! - `zqn::simulation` consumes operation contexts;
//! - `zqn::target` checks whether a target can represent the requested context;
//! - routing may inspect resources but owns placement;
//! - scheduling may inspect duration but owns schedule placement;
//! - QEC may convert operation-associated noise into physical faults;
//! - hardware adapters provide target/resource information but do not own this
//!   semantic type.
//!
//! Producers:
//!
//! - canonical quantum IR lowering;
//! - ZQN adapters;
//! - characterization;
//! - scheduling/lowering stages when an execution context is required.
//!
//! Non-owners:
//!
//! - this module does not own gate semantics;
//! - this module does not own routing;
//! - this module does not own scheduling;
//! - this module does not own hardware topology;
//! - this module does not own calibration;
//! - this module does not own noise channels;
//! - this module does not own random sampling;
//! - this module does not own QEC decoding.
//!
//! # Compatibility
//!
//! This module intentionally uses only the standard library plus canonical
//! Zamani IR identity types. It therefore avoids introducing a dependency on
//! future ZQN modules that would otherwise create circular dependencies.
//!
//! # No unsafe
//!
//! `#![forbid(unsafe_code)]` makes the requirement compiler-enforced.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Constants
// =============================================================================

/// Semantic representation version for this ZQN operation contract.
///
/// This is deliberately a local representation marker rather than a machine
/// capacity or execution limit.
///
/// The top-level ZQN version/schema system remains responsible for external
/// compatibility.
pub const OPERATION_MODEL_VERSION: u16 = 1;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing or validating a ZQN operation context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationError {
    /// An operation name was empty after trimming.
    EmptyName,

    /// An operation name was invalid.
    InvalidName,

    /// A parameter name was empty or invalid.
    InvalidParameterName,

    /// A metadata key was empty or invalid.
    InvalidMetadataKey,

    /// A textual value was empty where a non-empty value is required.
    EmptyValue,

    /// A floating-point value was NaN or infinite.
    NonFiniteValue {
        field: &'static str,
    },

    /// A duration was negative.
    NegativeDuration,

    /// A duplicate semantic operand was supplied where uniqueness is required.
    DuplicateResource,

    /// An operation requires at least one resource for the requested
    /// operation category.
    MissingRequiredResource,

    /// An operation ID was not acceptable to the ZQN operation context.
    InvalidOperationId,

    /// A resource identity was structurally invalid.
    InvalidResource,

    /// An operation context violated one of its invariants.
    InvalidOperation {
        reason: &'static str,
    },
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "operation name must not be empty"),
            Self::InvalidName => write!(f, "operation name contains invalid characters"),
            Self::InvalidParameterName => {
                write!(f, "parameter name must not be empty or invalid")
            }
            Self::InvalidMetadataKey => {
                write!(f, "metadata key must not be empty or invalid")
            }
            Self::EmptyValue => write!(f, "value must not be empty"),
            Self::NonFiniteValue { field } => {
                write!(f, "{field} must be finite")
            }
            Self::NegativeDuration => write!(f, "operation duration must not be negative"),
            Self::DuplicateResource => {
                write!(f, "duplicate resource in operation context")
            }
            Self::MissingRequiredResource => {
                write!(f, "required operation resource is missing")
            }
            Self::InvalidOperationId => write!(f, "invalid operation identifier"),
            Self::InvalidResource => write!(f, "invalid operation resource"),
            Self::InvalidOperation { reason } => {
                write!(f, "invalid operation: {reason}")
            }
        }
    }
}

impl std::error::Error for OperationError {}

/// Result type used by this module.
pub type OperationResult<T> = Result<T, OperationError>;

// =============================================================================
// Operation category
// =============================================================================

/// Broad semantic category of an operation.
///
/// This is intentionally NOT a gate set.
///
/// Gate names such as `x`, `h`, `cx`, vendor-native gates, custom gates, etc.
/// are represented by [`OperationName`]. This enum describes the semantic
/// role of an operation so that ZQN can reason about preparation, measurement,
/// idling, transport, etc. without hard-coding a technology's instruction set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperationClass {
    /// A unitary or generally gate-like operation.
    Gate,

    /// State preparation.
    Preparation,

    /// Reset/reinitialization.
    Reset,

    /// Measurement/readout.
    Measurement,

    /// Explicit idle/wait operation.
    Idle,

    /// Pulse/control-level operation.
    Pulse,

    /// Physical or logical quantum transport.
    Transport,

    /// Analog/Hamiltonian evolution.
    Evolution,

    /// Quantum communication/link operation.
    Communication,

    /// Composite operation whose implementation is expanded elsewhere.
    Composite,

    /// A dynamically defined/custom operation category.
    Custom,
}

impl OperationClass {
    /// Returns whether the class is normally associated with an operation
    /// duration.
    #[must_use]
    pub const fn may_have_duration(self) -> bool {
        matches!(
            self,
            Self::Gate
                | Self::Preparation
                | Self::Reset
                | Self::Measurement
                | Self::Idle
                | Self::Pulse
                | Self::Transport
                | Self::Evolution
                | Self::Communication
                | Self::Composite
                | Self::Custom
        )
    }

    /// Returns whether the class semantically represents measurement.
    #[must_use]
    pub const fn is_measurement(self) -> bool {
        matches!(self, Self::Measurement)
    }

    /// Returns whether the class represents an operation that can naturally
    /// produce readout noise.
    #[must_use]
    pub const fn has_readout_semantics(self) -> bool {
        matches!(self, Self::Measurement)
    }
}

// =============================================================================
// Operation name
// =============================================================================

/// Validated operation name.
///
/// Names are intentionally strings rather than an enum containing a fixed
/// universal gate set.
///
/// Examples include:
///
/// - `x`
/// - `h`
/// - `cx`
/// - `measure`
/// - vendor-native operations
/// - user-defined Zamani operations
/// - future operation names
///
/// ZQN does not interpret the gate's mathematical semantics here. The
/// canonical quantum IR owns that responsibility.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperationName(String);

impl OperationName {
    /// Creates a validated operation name.
    pub fn new(value: impl Into<String>) -> OperationResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(OperationError::EmptyName);
        }

        if !Self::is_valid_name(&value) {
            return Err(OperationError::InvalidName);
        }

        Ok(Self(value))
    }

    /// Returns the operation name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the wrapper and returns the underlying string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }

    fn is_valid_name(value: &str) -> bool {
        let mut chars = value.chars();

        let Some(first) = chars.next() else {
            return false;
        };

        if !(first.is_ascii_alphabetic() || first == '_') {
            return false;
        }

        chars.all(|character| {
            character.is_ascii_alphanumeric()
                || matches!(character, '_' | '.' | ':' | '/' | '-' | '$')
        })
    }
}

impl AsRef<str> for OperationName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for OperationName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Parameter values
// =============================================================================

/// A parameter value associated with an operation.
///
/// This intentionally supports both numeric and symbolic values so ZQN does
/// not require every operation to be fully numerically resolved at the point
/// where its noise context is constructed.
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterValue {
    /// Signed integer.
    Integer(i128),

    /// Unsigned integer.
    Unsigned(u128),

    /// Finite floating-point value.
    Real(f64),

    /// Boolean value.
    Boolean(bool),

    /// Textual value.
    Text(String),

    /// Symbolic expression/name resolved by another layer.
    Symbol(String),
}

impl ParameterValue {
    /// Creates a finite real parameter.
    pub fn real(value: f64) -> OperationResult<Self> {
        if !value.is_finite() {
            return Err(OperationError::NonFiniteValue {
                field: "parameter",
            });
        }

        Ok(Self::Real(value))
    }

    /// Returns the contained real value when this is a real parameter.
    #[must_use]
    pub fn as_real(&self) -> Option<f64> {
        match self {
            Self::Real(value) => Some(*value),
            _ => None,
        }
    }

    /// Validates this parameter value.
    pub fn validate(&self) -> OperationResult<()> {
        match self {
            Self::Real(value) if !value.is_finite() => {
                Err(OperationError::NonFiniteValue {
                    field: "parameter",
                })
            }
            Self::Text(value) | Self::Symbol(value) if value.trim().is_empty() => {
                Err(OperationError::EmptyValue)
            }
            _ => Ok(()),
        }
    }
}

// =============================================================================
// Duration
// =============================================================================

/// Non-negative operation duration in seconds.
///
/// Seconds are used as the semantic unit so that this type does not impose a
/// hardware-specific clock resolution.
///
/// A scheduler or hardware target may later lower this into a native timing
/// representation.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OperationDuration(f64);

impl OperationDuration {
    /// Zero duration.
    pub const ZERO: Self = Self(0.0);

    /// Creates a duration from seconds.
    pub fn from_seconds(seconds: f64) -> OperationResult<Self> {
        if !seconds.is_finite() {
            return Err(OperationError::NonFiniteValue {
                field: "duration",
            });
        }

        if seconds < 0.0 {
            return Err(OperationError::NegativeDuration);
        }

        Ok(Self(seconds))
    }

    /// Returns the duration in seconds.
    #[must_use]
    pub const fn as_seconds(self) -> f64 {
        self.0
    }

    /// Returns whether the duration is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0.0
    }
}

// =============================================================================
// Resources
// =============================================================================

/// A quantum resource to which an operation may refer.
///
/// `Qubit` uses Zamani's canonical `quantum::ir::qubit::QubitId`.
///
/// The other variants allow ZQN to remain useful for future quantum
/// modalities without replacing the canonical qubit identity with a ZQN-local
/// type.
///
/// Resource names and indices are semantic references; physical placement and
/// topology remain owned by routing/hardware.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperationResource {
    /// Canonical Zamani quantum-IR qubit identity.
    Qubit(QubitId),

    /// A resource identified by a namespace and an arbitrary stable name.
    Named {
        namespace: String,
        name: String,
    },

    /// A resource identified by namespace and arbitrary unsigned index.
    Indexed {
        namespace: String,
        index: u128,
    },
}

impl OperationResource {
    /// Creates a qubit resource from the canonical IR qubit identity.
    #[must_use]
    pub const fn qubit(qubit: QubitId) -> Self {
        Self::Qubit(qubit)
    }

    /// Creates a named resource.
    pub fn named(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> OperationResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.trim().is_empty() || name.trim().is_empty() {
            return Err(OperationError::InvalidResource);
        }

        Ok(Self::Named { namespace, name })
    }

    /// Creates an indexed resource.
    pub fn indexed(
        namespace: impl Into<String>,
        index: u128,
    ) -> OperationResult<Self> {
        let namespace = namespace.into();

        if namespace.trim().is_empty() {
            return Err(OperationError::InvalidResource);
        }

        Ok(Self::Indexed { namespace, index })
    }

    /// Returns the canonical qubit identity when this resource is a qubit.
    #[must_use]
    pub const fn as_qubit(&self) -> Option<QubitId> {
        match self {
            Self::Qubit(qubit) => Some(*qubit),
            _ => None,
        }
    }

    /// Returns the resource namespace where applicable.
    #[must_use]
    pub fn namespace(&self) -> Option<&str> {
        match self {
            Self::Qubit(_) => Some("quantum.qubit"),
            Self::Named { namespace, .. } | Self::Indexed { namespace, .. } => {
                Some(namespace.as_str())
            }
        }
    }
}

// =============================================================================
// Resource roles
// =============================================================================

/// Semantic role of an operation resource.
///
/// This avoids assuming that every operation resource is simply "a qubit".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResourceRole {
    /// Primary quantum input.
    Input,

    /// Quantum output.
    Output,

    /// Control resource.
    Control,

    /// Target resource.
    Target,

    /// Resource being measured.
    Measured,

    /// Resource being prepared/reset.
    Prepared,

    /// Resource held idle.
    Idle,

    /// Resource being transported.
    Transported,

    /// Resource used by an analog/evolution operation.
    Evolved,

    /// Resource used as an auxiliary or ancilla.
    Auxiliary,

    /// Resource whose exact role is defined by a higher-level dialect.
    Other,
}

// =============================================================================
// Resource binding
// =============================================================================

/// A resource plus its semantic role and deterministic position.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceBinding {
    resource: OperationResource,
    role: ResourceRole,
}

impl ResourceBinding {
    /// Creates a resource binding.
    #[must_use]
    pub const fn new(resource: OperationResource, role: ResourceRole) -> Self {
        Self { resource, role }
    }

    /// Returns the bound resource.
    #[must_use]
    pub const fn resource(&self) -> &OperationResource {
        &self.resource
    }

    /// Returns the resource role.
    #[must_use]
    pub const fn role(&self) -> ResourceRole {
        self.role
    }
}

// =============================================================================
// Operation metadata
// =============================================================================

/// Deterministically ordered metadata attached to an operation.
///
/// Metadata is descriptive and must never silently alter operation semantics.
/// Semantic parameters belong in [`ParameterValue`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OperationMetadata {
    entries: BTreeMap<String, String>,
}

impl OperationMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    /// Inserts metadata.
    ///
    /// Returns the previous value if the key already existed.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> OperationResult<Option<String>> {
        let key = key.into();
        let value = value.into();

        if key.trim().is_empty() {
            return Err(OperationError::InvalidMetadataKey);
        }

        Ok(self.entries.insert(key, value))
    }

    /// Gets metadata.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    /// Returns the number of metadata entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether metadata is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterates in deterministic key order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }
}

// =============================================================================
// Operation parameters
// =============================================================================

/// Deterministically ordered operation parameters.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OperationParameters {
    values: BTreeMap<String, ParameterValue>,
}

impl OperationParameters {
    /// Creates an empty parameter set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    /// Inserts or replaces a parameter.
    pub fn insert(
        &mut self,
        name: impl Into<String>,
        value: ParameterValue,
    ) -> OperationResult<Option<ParameterValue>> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(OperationError::InvalidParameterName);
        }

        value.validate()?;

        Ok(self.values.insert(name, value))
    }

    /// Gets a parameter.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&ParameterValue> {
        self.values.get(name)
    }

    /// Returns the number of parameters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether there are no parameters.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Iterates in deterministic name order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &ParameterValue)> {
        self.values
            .iter()
            .map(|(name, value)| (name.as_str(), value))
    }
}

// =============================================================================
// Operation
// =============================================================================

/// A ZQN operation execution context.
///
/// This is NOT the canonical Zamani quantum-IR operation.
///
/// It is the ZQN-side context needed to associate physical noise, calibration,
/// uncertainty and execution effects with a canonical operation.
///
/// The canonical semantic operation identity is [`OperationId`].
#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    operation_id: OperationId,
    class: OperationClass,
    name: OperationName,
    resources: Vec<ResourceBinding>,
    duration: Option<OperationDuration>,
    parameters: OperationParameters,
    metadata: OperationMetadata,
}

impl Operation {
    /// Creates a new operation context.
    ///
    /// No machine-size limit is imposed.
    pub fn new(
        operation_id: OperationId,
        class: OperationClass,
        name: OperationName,
    ) -> OperationResult<Self> {
        let operation = Self {
            operation_id,
            class,
            name,
            resources: Vec::new(),
            duration: None,
            parameters: OperationParameters::new(),
            metadata: OperationMetadata::new(),
        };

        operation.validate()?;

        Ok(operation)
    }

    /// Creates an operation context from a string name.
    pub fn named(
        operation_id: OperationId,
        class: OperationClass,
        name: impl Into<String>,
    ) -> OperationResult<Self> {
        Self::new(operation_id, class, OperationName::new(name)?)
    }

    /// Returns the canonical IR operation identity.
    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    /// Returns the semantic operation class.
    #[must_use]
    pub const fn class(&self) -> OperationClass {
        self.class
    }

    /// Returns the operation name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the operation name wrapper.
    #[must_use]
    pub const fn operation_name(&self) -> &OperationName {
        &self.name
    }

    /// Adds a resource to the operation.
    ///
    /// Resource order is semantically significant and therefore preserved.
    ///
    /// Duplicate resource bindings are rejected. A resource may occur multiple
    /// times in a larger computation as part of different operations; this
    /// method only prevents accidental duplication inside one operation
    /// context.
    pub fn with_resource(
        mut self,
        resource: OperationResource,
        role: ResourceRole,
    ) -> OperationResult<Self> {
        self.add_resource(resource, role)?;
        Ok(self)
    }

    /// Adds a resource in-place.
    pub fn add_resource(
        &mut self,
        resource: OperationResource,
        role: ResourceRole,
    ) -> OperationResult<()> {
        if self
            .resources
            .iter()
            .any(|binding| binding.resource() == &resource)
        {
            return Err(OperationError::DuplicateResource);
        }

        self.resources.push(ResourceBinding::new(resource, role));

        Ok(())
    }

    /// Adds many resources in one operation.
    ///
    /// Validation is transactional: if any resource is invalid or duplicated,
    /// the operation is left unchanged.
    pub fn with_resources<I>(
        mut self,
        resources: I,
    ) -> OperationResult<Self>
    where
        I: IntoIterator<Item = (OperationResource, ResourceRole)>,
    {
        self.add_resources(resources)?;
        Ok(self)
    }

    /// Adds many resources transactionally.
    pub fn add_resources<I>(
        &mut self,
        resources: I,
    ) -> OperationResult<()>
    where
        I: IntoIterator<Item = (OperationResource, ResourceRole)>,
    {
        let additions: Vec<ResourceBinding> = resources
            .into_iter()
            .map(|(resource, role)| ResourceBinding::new(resource, role))
            .collect();

        for addition in &additions {
            if self
                .resources
                .iter()
                .any(|existing| existing.resource() == addition.resource())
            {
                return Err(OperationError::DuplicateResource);
            }
        }

        for (index, left) in additions.iter().enumerate() {
            if additions[index + 1..]
                .iter()
                .any(|right| left.resource() == right.resource())
            {
                return Err(OperationError::DuplicateResource);
            }
        }

        self.resources.extend(additions);

        Ok(())
    }

    /// Returns all operation resources in semantic order.
    #[must_use]
    pub fn resources(&self) -> &[ResourceBinding] {
        &self.resources
    }

    /// Returns the number of operation resources.
    #[must_use]
    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }

    /// Returns true when no resources are attached.
    #[must_use]
    pub fn has_no_resources(&self) -> bool {
        self.resources.is_empty()
    }

    /// Returns all qubit resources.
    ///
    /// The returned iterator does not allocate.
    pub fn qubits(&self) -> impl Iterator<Item = QubitId> + '_ {
        self.resources
            .iter()
            .filter_map(|binding| binding.resource().as_qubit())
    }

    /// Returns the number of qubit resources.
    #[must_use]
    pub fn qubit_count(&self) -> usize {
        self.qubits().count()
    }

    /// Sets the operation duration.
    pub fn set_duration(
        &mut self,
        duration: OperationDuration,
    ) -> OperationResult<()> {
        self.duration = Some(duration);
        Ok(())
    }

    /// Builder-style duration assignment.
    pub fn with_duration(
        mut self,
        duration: OperationDuration,
    ) -> OperationResult<Self> {
        self.set_duration(duration)?;
        Ok(self)
    }

    /// Returns the optional operation duration.
    #[must_use]
    pub const fn duration(&self) -> Option<OperationDuration> {
        self.duration
    }

    /// Adds or replaces an operation parameter.
    pub fn set_parameter(
        &mut self,
        name: impl Into<String>,
        value: ParameterValue,
    ) -> OperationResult<()> {
        self.parameters.insert(name, value)?;
        Ok(())
    }

    /// Builder-style parameter assignment.
    pub fn with_parameter(
        mut self,
        name: impl Into<String>,
        value: ParameterValue,
    ) -> OperationResult<Self> {
        self.set_parameter(name, value)?;
        Ok(self)
    }

    /// Returns operation parameters.
    #[must_use]
    pub const fn parameters(&self) -> &OperationParameters {
        &self.parameters
    }

    /// Adds or replaces metadata.
    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> OperationResult<()> {
        self.metadata.insert(key, value)?;
        Ok(())
    }

    /// Builder-style metadata assignment.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> OperationResult<Self> {
        self.set_metadata(key, value)?;
        Ok(self)
    }

    /// Returns operation metadata.
    #[must_use]
    pub const fn metadata(&self) -> &OperationMetadata {
        &self.metadata
    }

    /// Returns whether this is a measurement operation.
    #[must_use]
    pub const fn is_measurement(&self) -> bool {
        self.class.is_measurement()
    }

    /// Returns whether this operation has explicit duration information.
    #[must_use]
    pub const fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    /// Returns whether this operation contains at least one qubit.
    #[must_use]
    pub fn touches_qubits(&self) -> bool {
        self.resources
            .iter()
            .any(|binding| binding.resource().as_qubit().is_some())
    }

    /// Returns whether this operation has a specific resource role.
    #[must_use]
    pub fn has_role(&self, role: ResourceRole) -> bool {
        self.resources
            .iter()
            .any(|binding| binding.role() == role)
    }

    /// Returns the first resource with a requested role.
    #[must_use]
    pub fn resource_with_role(
        &self,
        role: ResourceRole,
    ) -> Option<&OperationResource> {
        self.resources
            .iter()
            .find(|binding| binding.role() == role)
            .map(ResourceBinding::resource)
    }

    /// Validates the complete operation context.
    ///
    /// This validates only invariants owned by this module. It does not try to
    /// validate the canonical IR operation itself; that remains the
    /// responsibility of the IR validation layer.
    pub fn validate(&self) -> OperationResult<()> {
        if self.name.as_str().trim().is_empty() {
            return Err(OperationError::EmptyName);
        }

        if self.operation_id_is_invalid() {
            return Err(OperationError::InvalidOperationId);
        }

        if let Some(duration) = self.duration {
            if !duration.as_seconds().is_finite() {
                return Err(OperationError::NonFiniteValue {
                    field: "duration",
                });
            }

            if duration.as_seconds() < 0.0 {
                return Err(OperationError::NegativeDuration);
            }
        }

        for parameter in self.parameters.values.values() {
            parameter.validate()?;
        }

        for (index, left) in self.resources.iter().enumerate() {
            for right in &self.resources[index + 1..] {
                if left.resource() == right.resource() {
                    return Err(OperationError::DuplicateResource);
                }
            }
        }

        self.validate_class_resources()?;

        Ok(())
    }

    fn operation_id_is_invalid(&self) -> bool {
        // OperationId is intentionally opaque. ZQN does not reinterpret its
        // internal representation or impose a machine-size limit.
        false
    }

    fn validate_class_resources(&self) -> OperationResult<()> {
        match self.class {
            OperationClass::Measurement => {
                if self.resources.is_empty() {
                    return Err(OperationError::MissingRequiredResource);
                }

                if !self.has_role(ResourceRole::Measured)
                    && !self.touches_qubits()
                {
                    return Err(OperationError::MissingRequiredResource);
                }
            }

            OperationClass::Preparation | OperationClass::Reset => {
                if self.resources.is_empty() {
                    return Err(OperationError::MissingRequiredResource);
                }
            }

            OperationClass::Idle => {
                if self.resources.is_empty() {
                    return Err(OperationError::MissingRequiredResource);
                }
            }

            _ => {}
        }

        Ok(())
    }
}

// =============================================================================
// Operation references
// =============================================================================

/// A lightweight stable reference to an operation.
///
/// This is useful for noise annotations, calibration records and
/// characterization results that should reference an operation without
/// embedding the complete operation context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperationReference {
    operation_id: OperationId,
}

impl OperationReference {
    /// Creates an operation reference from the canonical operation identity.
    #[must_use]
    pub const fn new(operation_id: OperationId) -> Self {
        Self { operation_id }
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation_id(self) -> OperationId {
        self.operation_id
    }
}

impl From<OperationId> for OperationReference {
    fn from(operation_id: OperationId) -> Self {
        Self::new(operation_id)
    }
}

impl From<&Operation> for OperationReference {
    fn from(operation: &Operation) -> Self {
        Self::new(operation.operation_id())
    }
}

// =============================================================================
// Operation location
// =============================================================================

/// The semantic location at which ZQN may associate an effect.
///
/// This is deliberately broader than a qubit-only location.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperationLocation {
    /// An operation identity.
    Operation(OperationReference),

    /// A specific resource.
    Resource(OperationResource),

    /// An operation/resource pair.
    OperationOnResource {
        operation: OperationReference,
        resource: OperationResource,
    },

    /// An operation over an explicitly ordered set of resources.
    OperationOnResources {
        operation: OperationReference,
        resources: Vec<OperationResource>,
    },
}

impl OperationLocation {
    /// Creates an operation location.
    #[must_use]
    pub const fn operation(operation: OperationReference) -> Self {
        Self::Operation(operation)
    }

    /// Creates a resource location.
    #[must_use]
    pub const fn resource(resource: OperationResource) -> Self {
        Self::Resource(resource)
    }

    /// Creates an operation/resource location.
    #[must_use]
    pub const fn operation_on_resource(
        operation: OperationReference,
        resource: OperationResource,
    ) -> Self {
        Self::OperationOnResource {
            operation,
            resource,
        }
    }

    /// Creates an operation/multiple-resource location.
    #[must_use]
    pub fn operation_on_resources(
        operation: OperationReference,
        resources: Vec<OperationResource>,
    ) -> OperationResult<Self> {
        if resources.is_empty() {
            return Err(OperationError::MissingRequiredResource);
        }

        for (index, left) in resources.iter().enumerate() {
            if resources[index + 1..].contains(left) {
                return Err(OperationError::DuplicateResource);
            }
        }

        Ok(Self::OperationOnResources {
            operation,
            resources,
        })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation_id(value: usize) -> OperationId {
        OperationId::new(value)
    }

    #[test]
    fn operation_name_is_validated() {
        assert!(OperationName::new("cx").is_ok());
        assert!(OperationName::new("vendor.native_gate").is_ok());
        assert!(OperationName::new("_custom").is_ok());
        assert!(OperationName::new("").is_err());
        assert!(OperationName::new("1invalid").is_err());
    }

    #[test]
    fn arbitrary_operation_names_are_supported() {
        let name = OperationName::new("future.quantum.operation").unwrap();

        assert_eq!(name.as_str(), "future.quantum.operation");
    }

    #[test]
    fn operation_uses_canonical_operation_id() {
        let id = operation_id(17);

        let operation =
            Operation::named(id, OperationClass::Gate, "custom_gate").unwrap();

        assert_eq!(operation.operation_id(), id);
    }

    #[test]
    fn operation_uses_canonical_qubit_id() {
        let qubit = QubitId::new(42);

        let operation = Operation::named(
            operation_id(1),
            OperationClass::Gate,
            "x",
        )
        .unwrap()
        .with_resource(
            OperationResource::qubit(qubit),
            ResourceRole::Target,
        )
        .unwrap();

        assert_eq!(operation.qubit_count(), 1);
        assert_eq!(operation.qubits().next(), Some(qubit));
    }

    #[test]
    fn arbitrary_operation_arity_is_supported() {
        let resources = (0usize..64usize).map(|index| {
            (
                OperationResource::qubit(QubitId::new(index)),
                ResourceRole::Target,
            )
        });

        let operation = Operation::named(
            operation_id(2),
            OperationClass::Custom,
            "large_arity_operation",
        )
        .unwrap()
        .with_resources(resources)
        .unwrap();

        assert_eq!(operation.qubit_count(), 64);
    }

    #[test]
    fn duplicate_resources_are_rejected() {
        let qubit = QubitId::new(0);

        let result = Operation::named(
            operation_id(3),
            OperationClass::Gate,
            "x",
        )
        .unwrap()
        .with_resource(
            OperationResource::qubit(qubit),
            ResourceRole::Target,
        )
        .and_then(|operation| {
            operation.with_resource(
                OperationResource::qubit(qubit),
                ResourceRole::Target,
            )
        });

        assert_eq!(result, Err(OperationError::DuplicateResource));
    }

    #[test]
    fn measurement_requires_resources() {
        let operation = Operation::named(
            operation_id(4),
            OperationClass::Measurement,
            "measure",
        );

        assert!(operation.is_err());
    }

    #[test]
    fn measurement_with_qubit_is_valid() {
        let operation = Operation::named(
            operation_id(5),
            OperationClass::Measurement,
            "measure",
        )
        .unwrap()
        .with_resource(
            OperationResource::qubit(QubitId::new(0)),
            ResourceRole::Measured,
        )
        .unwrap();

        assert!(operation.validate().is_ok());
        assert!(operation.is_measurement());
    }

    #[test]
    fn reset_requires_resources() {
        let result = Operation::named(
            operation_id(6),
            OperationClass::Reset,
            "reset",
        );

        assert!(result.is_err());
    }

    #[test]
    fn idle_requires_resources() {
        let result = Operation::named(
            operation_id(7),
            OperationClass::Idle,
            "idle",
        );

        assert!(result.is_err());
    }

    #[test]
    fn durations_must_be_finite_and_non_negative() {
        assert!(OperationDuration::from_seconds(1.0).is_ok());
        assert!(OperationDuration::from_seconds(0.0).is_ok());
        assert!(OperationDuration::from_seconds(-1.0).is_err());
        assert!(OperationDuration::from_seconds(f64::NAN).is_err());
        assert!(OperationDuration::from_seconds(f64::INFINITY).is_err());
    }

    #[test]
    fn parameters_are_deterministically_ordered() {
        let mut parameters = OperationParameters::new();

        parameters
            .insert("z", ParameterValue::Integer(1))
            .unwrap();

        parameters
            .insert("a", ParameterValue::Integer(2))
            .unwrap();

        let names: Vec<&str> =
            parameters.iter().map(|(name, _)| name).collect();

        assert_eq!(names, vec!["a", "z"]);
    }

    #[test]
    fn invalid_real_parameters_are_rejected() {
        assert!(ParameterValue::real(f64::NAN).is_err());
        assert!(ParameterValue::real(f64::INFINITY).is_err());
        assert!(ParameterValue::real(1.5).is_ok());
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let mut metadata = OperationMetadata::new();

        metadata.insert("z", "last").unwrap();
        metadata.insert("a", "first").unwrap();

        let keys: Vec<&str> =
            metadata.iter().map(|(key, _)| key).collect();

        assert_eq!(keys, vec!["a", "z"]);
    }

    #[test]
    fn operation_reference_is_identity_only() {
        let id = operation_id(100);
        let reference = OperationReference::new(id);

        assert_eq!(reference.operation_id(), id);
    }

    #[test]
    fn operation_location_supports_operation_resource_pairs() {
        let operation = OperationReference::new(operation_id(11));
        let resource =
            OperationResource::qubit(QubitId::new(3));

        let location =
            OperationLocation::operation_on_resource(operation, resource);

        match location {
            OperationLocation::OperationOnResource {
                operation: actual_operation,
                resource: actual_resource,
            } => {
                assert_eq!(actual_operation, operation);
                assert_eq!(actual_resource, resource);
            }
            _ => panic!("unexpected operation location"),
        }
    }

    #[test]
    fn operation_can_carry_duration_parameters_and_metadata() {
        let operation = Operation::named(
            operation_id(12),
            OperationClass::Gate,
            "custom_gate",
        )
        .unwrap()
        .with_resource(
            OperationResource::qubit(QubitId::new(0)),
            ResourceRole::Target,
        )
        .unwrap()
        .with_duration(OperationDuration::from_seconds(20e-9).unwrap())
        .unwrap()
        .with_parameter(
            "theta",
            ParameterValue::real(0.25).unwrap(),
        )
        .unwrap()
        .with_metadata("source", "zamani")
        .unwrap();

        assert_eq!(
            operation.duration().unwrap().as_seconds(),
            20e-9
        );

        assert_eq!(
            operation
                .parameters()
                .get("theta")
                .and_then(ParameterValue::as_real),
            Some(0.25)
        );

        assert_eq!(
            operation.metadata().get("source"),
            Some("zamani")
        );
    }

    #[test]
    fn operation_context_is_cloneable_and_comparable() {
        let operation = Operation::named(
            operation_id(13),
            OperationClass::Gate,
            "h",
        )
        .unwrap()
        .with_resource(
            OperationResource::qubit(QubitId::new(0)),
            ResourceRole::Target,
        )
        .unwrap();

        let cloned = operation.clone();

        assert_eq!(operation, cloned);
    }

    #[test]
    fn resource_namespace_is_available() {
        let qubit =
            OperationResource::qubit(QubitId::new(0));

        assert_eq!(
            qubit.namespace(),
            Some("quantum.qubit")
        );
    }
}