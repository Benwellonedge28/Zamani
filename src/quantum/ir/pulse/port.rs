//! Zamani Quantum IR — Pulse Port Semantics
//!
//! Canonical, hardware-independent representation of pulse/control I/O ports.
//!
//! # Architectural role
//!
//! `quantum::ir::pulse::port` defines the semantic meaning of an abstract
//! pulse-control or acquisition endpoint.
//!
//! A port answers:
//!
//! > Where, semantically, may a pulse/control/acquisition signal enter or
//! > leave the quantum-control environment?
//!
//! It does NOT answer:
//!
//! - which physical DAC is used;
//! - which ADC is used;
//! - which FPGA/controller owns the endpoint;
//! - which cable is connected;
//! - which rack/controller/channel number is used;
//! - which oscillator is used;
//! - which laser is used;
//! - which physical wiring exists;
//! - which calibration is selected;
//! - which hardware device implements the port;
//! - which logical qubit is routed to which physical resource;
//! - when the port is scheduled;
//! - how a waveform is sampled;
//! - how a pulse is synthesized;
//! - how a QPU is contacted.
//!
//! Those responsibilities belong to target/hardware, mapping, calibration,
//! scheduling, pulse lowering, and backend subsystems.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! frontend
//!      |
//!      v
//! canonical Zamani Quantum IR
//!      |
//!      +----------------------------+
//!      |                            |
//!      v                            v
//!   pulse.rs                    pulse::port.rs
//!      |                            |
//!      | waveform/frame/channel     | abstract endpoint
//!      | references                 |
//!      +-------------+--------------+
//!                    |
//!                    v
//!              target-independent
//!                 compilation
//!                    |
//!          +---------+----------+
//!          |         |          |
//!          v         v          v
//!       mapping  scheduling  optimization
//!          |         |          |
//!          +---------+----------+
//!                    |
//!                    v
//!                 hardware
//!                    |
//!                    v
//!                 backend
//!                    |
//!                    v
//!                   QPU
//! ```
//!
//! # Universal-program principle
//!
//! A Zamani program is written once at the semantic level and may be lowered
//! to any compatible target for which the required capabilities and resources
//! exist.
//!
//! Therefore this file contains NO architectural machine-size limit.
//!
//! In particular, this file does not define:
//!
//! - a maximum number of ports;
//! - a maximum number of qubits;
//! - a maximum number of channels;
//! - a maximum number of targets;
//! - a maximum machine size;
//! - a particular controller topology;
//! - a particular hardware port numbering scheme.
//!
//! Any finite limit used during construction or validation is an explicit
//! resource/security policy supplied by the caller.
//!
//! # Port versus channel
//!
//! The distinction is intentional:
//!
//! ```text
//! Port
//!     = abstract semantic I/O endpoint
//!
//! Channel
//!     = abstract control/acquisition resource or signal family
//!
//! Frame
//!     = phase/frequency reference
//!
//! Waveform
//!     = time-dependent signal definition
//!
//! Pulse
//!     = operation that uses waveform/frame/channel/port semantics
//!
//! Hardware
//!     = physical implementation of all of the above
//! ```
//!
//! A port may optionally be associated with a canonical `ChannelId`, but the
//! two concepts are not collapsed into one type.
//!
//! # Qubit integration
//!
//! Ports may be associated with:
//!
//! ```rust
//! quantum::ir::qubit::QubitRef
//! ```
//!
//! This preserves the distinction between:
//!
//! - logical qubit;
//! - physical qubit.
//!
//! A `QubitRef::Physical` inside a port is only a semantic reference.
//! It does NOT prove that the physical qubit exists or that the selected
//! hardware supports the port.
//!
//! Logical-to-physical mapping remains a downstream responsibility.
//!
//! # Naming and identity
//!
//! The current canonical `identity.rs` contains `ChannelId`, `FrameId`,
//! `WaveformId`, `PulseId`, etc., but does not yet define a `PortId`.
//!
//! This file therefore deliberately uses:
//!
//! ```text
//! PortNamespace + PortName
//! ```
//!
//! as the stable semantic port identity.
//!
//! This is preferable to introducing a second independent numeric identity
//! implementation here that would later conflict with `identity.rs`.
//!
//! The combination:
//!
//! ```text
//! namespace + name
//! ```
//!
//! is deterministic, serializable, human-readable and suitable for distributed
//! compilation.
//!
//! It also permits arbitrary hierarchical namespaces without embedding vendor
//! hardware numbering into the canonical IR.
//!
//! # Determinism
//!
//! Metadata uses `BTreeMap` rather than `HashMap` so iteration order is
//! deterministic.
//!
//! Target collections preserve insertion order because target order may carry
//! semantic meaning for multi-target operations.
//!
//! # Resource safety
//!
//! This module provides explicit validation policies.
//!
//! Default limits are defensive construction limits only. They are NOT
//! architectural quantum-computer limits.
//!
//! Callers compiling larger programs may supply larger policies.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe.
//!
//! The module enforces the no-unsafe requirement at compile time.
//!
//! # Integration contracts
//!
//! `super::super::channel`
//!     Owns abstract channel semantics. This file stores only `ChannelId`
//!     associations and does not redefine channel semantics.
//!
//! `super::super::identity`
//!     Owns stable channel identities and IR identities.
//!
//! `super::super::qubit`
//!     Owns canonical `QubitRef`, `QubitId`, and `PhysicalQubitId`.
//!
//! `super::super::pulse`
//!     May reference this module when representing pulse-level endpoint
//!     requirements.
//!
//! `super::super::frame`
//!     May associate frames with ports during downstream integration.
//!
//! `super::super::waveform`
//!     Remains independent from port semantics.
//!
//! `super::super::timing`
//!     Owns program-wide timing semantics.
//!
//! `super::super::schedule`
//!     Determines when a port is occupied.
//!
//! `super::super::mapping`
//!     Determines logical-to-physical placement.
//!
//! `super::super::validation`
//!     Performs whole-program validation and target compatibility checking.
//!
//! `super::super::serialization`
//!     Owns canonical persistence/encoding.
//!
//! `super::super::hash`
//!     Owns canonical content hashing.
//!
//! `quantum::hardware`
//!     Resolves semantic ports to actual hardware endpoints.
//!
//! `backend`
//!     Converts target-specific port information into executable instructions.
//!
//! # Completion guarantee
//!
//! This file owns the complete semantic contract for an abstract pulse port:
//!
//! - stable namespace/name identity;
//! - port kind;
//! - direction;
//! - access mode;
//! - scope;
//! - qubit target association;
//! - optional channel association;
//! - deterministic metadata;
//! - validation policy;
//! - checked construction;
//! - checked target management;
//! - semantic equality;
//! - deterministic ordering;
//! - safe display formatting;
//! - no hardware assumptions;
//! - no fixed machine-size ceiling;
//! - no unsafe code;
//! - local tests.
//!
//! Future implementation of waveform, frame, pulse, hardware, routing,
//! scheduling, serialization or backend modules should consume this contract
//! rather than requiring the port semantic model to be redesigned.
//!
//! -----------------------------------------------------------------------------
//! No domain logic belongs outside the port contract below.
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use super::super::identity::ChannelId;
use super::super::qubit::QubitRef;

// =============================================================================
// Schema
// =============================================================================

/// Stable semantic schema identifier for pulse-port IR.
pub const PORT_SCHEMA_ID: &str = "zamani.quantum.ir.pulse.port";

/// Stable semantic schema version.
///
/// Breaking semantic changes require a new major IR contract.
pub const PORT_SCHEMA_VERSION: u16 = 1;

/// Default maximum namespace length in UTF-8 bytes.
///
/// This is an input/resource policy, not a machine-size limit.
pub const DEFAULT_MAX_NAMESPACE_BYTES: usize = 256;

/// Default maximum port-name length in UTF-8 bytes.
///
/// This is an input/resource policy, not a machine-size limit.
pub const DEFAULT_MAX_NAME_BYTES: usize = 256;

/// Default maximum metadata key length in UTF-8 bytes.
pub const DEFAULT_MAX_METADATA_KEY_BYTES: usize = 256;

/// Default maximum metadata value length in UTF-8 bytes.
pub const DEFAULT_MAX_METADATA_VALUE_BYTES: usize = 4096;

/// Default maximum number of metadata entries.
///
/// This is a defensive local construction policy only.
pub const DEFAULT_MAX_METADATA_FIELDS: usize = 4096;

/// Default maximum number of explicit port targets.
///
/// This is not a machine-wide target limit. Callers may supply a larger
/// policy when the target and host resources permit it.
pub const DEFAULT_MAX_TARGETS: usize = 4096;

// =============================================================================
// Result
// =============================================================================

/// Result type used by port construction and local validation.
pub type PortResult<T> = Result<T, PortError>;

// =============================================================================
// Port namespace
// =============================================================================

/// Stable semantic namespace for a port.
///
/// A namespace prevents unrelated modules, libraries or compilation units from
/// accidentally treating identically named ports as the same semantic object.
///
/// Example:
///
/// ```text
/// zamani.control
/// zamani.readout
/// experiment.calibration
/// vendor_extension.foo
/// ```
///
/// The namespace is semantic metadata. It is never interpreted as a hardware
/// vendor, device, controller or physical location.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PortNamespace(String);

impl PortNamespace {
    /// Creates a validated namespace.
    pub fn new<S: Into<String>>(value: S) -> PortResult<Self> {
        let value = value.into();

        validate_text(
            &value,
            DEFAULT_MAX_NAMESPACE_BYTES,
            "port namespace",
        )?;

        if value.is_empty() {
            return Err(PortError::EmptyNamespace);
        }

        Ok(Self(value))
    }

    /// Returns the namespace as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the namespace and returns its string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for PortNamespace {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PortNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PortNamespace {
    type Err = PortError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

// =============================================================================
// Port name
// =============================================================================

/// Stable semantic name of a port.
///
/// Port names are intentionally not physical controller numbers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PortName(String);

impl PortName {
    /// Creates a validated port name.
    pub fn new<S: Into<String>>(value: S) -> PortResult<Self> {
        let value = value.into();

        validate_text(
            &value,
            DEFAULT_MAX_NAME_BYTES,
            "port name",
        )?;

        if value.is_empty() {
            return Err(PortError::EmptyName);
        }

        Ok(Self(value))
    }

    /// Returns the port name as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the name and returns its string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for PortName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PortName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PortName {
    type Err = PortError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

// =============================================================================
// Port path
// =============================================================================

/// Deterministic fully-qualified semantic port name.
///
/// A port path is:
///
/// ```text
/// namespace:name
/// ```
///
/// No hardware meaning is implied by either component.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PortPath {
    namespace: PortNamespace,
    name: PortName,
}

impl PortPath {
    /// Creates a port path from a namespace and name.
    pub fn new(
        namespace: PortNamespace,
        name: PortName,
    ) -> Self {
        Self { namespace, name }
    }

    /// Creates a port path directly from strings.
    pub fn from_strings<N, P>(
        namespace: N,
        name: P,
    ) -> PortResult<Self>
    where
        N: Into<String>,
        P: Into<String>,
    {
        Ok(Self::new(
            PortNamespace::new(namespace)?,
            PortName::new(name)?,
        ))
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &PortNamespace {
        &self.namespace
    }

    /// Returns the name.
    #[must_use]
    pub fn name(&self) -> &PortName {
        &self.name
    }

    /// Returns the namespace string.
    #[must_use]
    pub fn namespace_str(&self) -> &str {
        self.namespace.as_str()
    }

    /// Returns the name string.
    #[must_use]
    pub fn name_str(&self) -> &str {
        self.name.as_str()
    }

    /// Returns a deterministic qualified string.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}:{}", self.namespace, self.name)
    }
}

impl fmt::Display for PortPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}",
            self.namespace,
            self.name
        )
    }
}

// =============================================================================
// Port kind
// =============================================================================

/// Semantic family of a pulse/control I/O port.
///
/// These variants describe what the endpoint means, not what hardware
/// implements it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PortKind {
    /// General quantum-control output.
    Control,

    /// Measurement/readout output from the program toward a target.
    Measure,

    /// Acquisition input from a measurement system.
    Acquire,

    /// Readout endpoint.
    Readout,

    /// Analog-control endpoint.
    Analog,

    /// Microwave/RF endpoint.
    Microwave,

    /// Flux/control-voltage endpoint.
    Flux,

    /// Optical endpoint.
    Optical,

    /// Laser endpoint.
    Laser,

    /// Digital/synchronization marker endpoint.
    Digital,

    /// Trigger endpoint.
    Trigger,

    /// General synchronization endpoint.
    Synchronization,

    /// User-defined semantic port kind.
    Custom(String),
}

impl PortKind {
    /// Creates a validated custom port kind.
    pub fn custom<S: Into<String>>(name: S) -> PortResult<Self> {
        let name = name.into();

        validate_non_empty_name(
            &name,
            "custom port kind",
        )?;

        Ok(Self::Custom(name))
    }

    /// Returns the stable semantic name.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Control => "control",
            Self::Measure => "measure",
            Self::Acquire => "acquire",
            Self::Readout => "readout",
            Self::Analog => "analog",
            Self::Microwave => "microwave",
            Self::Flux => "flux",
            Self::Optical => "optical",
            Self::Laser => "laser",
            Self::Digital => "digital",
            Self::Trigger => "trigger",
            Self::Synchronization => "synchronization",
            Self::Custom(value) => value.as_str(),
        }
    }

    /// Returns whether this is a custom port kind.
    #[must_use]
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }
}

impl fmt::Display for PortKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Direction
// =============================================================================

/// Semantic signal direction of a port.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PortDirection {
    /// Program/control system sends information through the port.
    Output,

    /// Program/control system receives information through the port.
    Input,

    /// Both directions are semantically permitted.
    Bidirectional,

    /// Direction is not applicable to the semantic resource.
    Unspecified,
}

impl Default for PortDirection {
    fn default() -> Self {
        Self::Output
    }
}

impl fmt::Display for PortDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Output => "output",
            Self::Input => "input",
            Self::Bidirectional => "bidirectional",
            Self::Unspecified => "unspecified",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Access mode
// =============================================================================

/// Semantic access policy for a port.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PortAccess {
    /// A single compatible operation conceptually owns the port at a time.
    Exclusive,

    /// Compatible operations may share the port.
    Shared,

    /// Port is only read from.
    ReadOnly,

    /// Port is only written to.
    WriteOnly,

    /// Access policy is supplied by the target.
    TargetDefined,
}

impl Default for PortAccess {
    fn default() -> Self {
        Self::Exclusive
    }
}

impl fmt::Display for PortAccess {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Exclusive => "exclusive",
            Self::Shared => "shared",
            Self::ReadOnly => "read_only",
            Self::WriteOnly => "write_only",
            Self::TargetDefined => "target_defined",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Scope
// =============================================================================

/// Semantic scope of a port.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PortScope {
    /// Endpoint is independent of a specific quantum target.
    Global,

    /// Endpoint is associated with one semantic target.
    PerTarget,

    /// Endpoint can serve multiple semantic targets.
    MultiTarget,

    /// Endpoint represents a relationship between targets.
    Pairwise,

    /// Endpoint has an architecture-defined scope.
    Custom,
}

impl Default for PortScope {
    fn default() -> Self {
        Self::Global
    }
}

impl fmt::Display for PortScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Global => "global",
            Self::PerTarget => "per_target",
            Self::MultiTarget => "multi_target",
            Self::Pairwise => "pairwise",
            Self::Custom => "custom",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Metadata
// =============================================================================

/// Deterministic metadata attached to a port.
///
/// Metadata is semantic/diagnostic information. It is not a hardware
/// credential store and must never contain authentication secrets.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PortMetadata {
    fields: BTreeMap<String, String>,
}

impl PortMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts one metadata field using the default validation policy.
    pub fn insert<K, V>(
        &mut self,
        key: K,
        value: V,
    ) -> PortResult<Option<String>>
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.insert_with_limits(
            key,
            value,
            DEFAULT_MAX_METADATA_FIELDS,
            DEFAULT_MAX_METADATA_KEY_BYTES,
            DEFAULT_MAX_METADATA_VALUE_BYTES,
        )
    }

    /// Inserts one metadata field using explicit resource limits.
    pub fn insert_with_limits<K, V>(
        &mut self,
        key: K,
        value: V,
        max_fields: usize,
        max_key_bytes: usize,
        max_value_bytes: usize,
    ) -> PortResult<Option<String>>
    where
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();
        let value = value.into();

        validate_text(
            &key,
            max_key_bytes,
            "metadata key",
        )?;

        validate_text(
            &value,
            max_value_bytes,
            "metadata value",
        )?;

        if key.is_empty() {
            return Err(PortError::EmptyMetadataKey);
        }

        if !self.fields.contains_key(&key)
            && self.fields.len() >= max_fields
        {
            return Err(PortError::MetadataFieldLimitExceeded {
                limit: max_fields,
            });
        }

        Ok(self.fields.insert(key, value))
    }

    /// Returns a metadata value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.fields.get(key).map(String::as_str)
    }

    /// Returns whether a key exists.
    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        self.fields.contains_key(key)
    }

    /// Returns the number of metadata fields.
    #[must_use]
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Returns whether metadata is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Iterates deterministically in key order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&str, &str)> {
        self.fields
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    /// Removes one metadata entry.
    pub fn remove(
        &mut self,
        key: &str,
    ) -> Option<String> {
        self.fields.remove(key)
    }

    /// Removes all metadata.
    pub fn clear(&mut self) {
        self.fields.clear();
    }
}

// =============================================================================
// Port validation policy
// =============================================================================

/// Explicit local validation/resource policy for ports.
///
/// These limits protect compiler processes from malformed or unexpectedly
/// large input. They do NOT define the maximum size of a quantum computer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PortValidationLimits {
    /// Maximum namespace length in bytes.
    pub max_namespace_bytes: usize,

    /// Maximum name length in bytes.
    pub max_name_bytes: usize,

    /// Maximum metadata fields.
    pub max_metadata_fields: usize,

    /// Maximum metadata key length.
    pub max_metadata_key_bytes: usize,

    /// Maximum metadata value length.
    pub max_metadata_value_bytes: usize,

    /// Maximum number of explicit targets.
    pub max_targets: usize,
}

impl Default for PortValidationLimits {
    fn default() -> Self {
        Self {
            max_namespace_bytes: DEFAULT_MAX_NAMESPACE_BYTES,
            max_name_bytes: DEFAULT_MAX_NAME_BYTES,
            max_metadata_fields: DEFAULT_MAX_METADATA_FIELDS,
            max_metadata_key_bytes: DEFAULT_MAX_METADATA_KEY_BYTES,
            max_metadata_value_bytes: DEFAULT_MAX_METADATA_VALUE_BYTES,
            max_targets: DEFAULT_MAX_TARGETS,
        }
    }
}

impl PortValidationLimits {
    /// Creates a policy with no artificial local limits other than the host's
    /// own memory/address-space constraints.
    ///
    /// `usize::MAX` does not allocate memory by itself. It simply means this
    /// validation layer does not impose a smaller explicit policy.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_namespace_bytes: usize::MAX,
            max_name_bytes: usize::MAX,
            max_metadata_fields: usize::MAX,
            max_metadata_key_bytes: usize::MAX,
            max_metadata_value_bytes: usize::MAX,
            max_targets: usize::MAX,
        }
    }
}

// =============================================================================
// Port
// =============================================================================

/// Canonical semantic pulse/control I/O port.
///
/// A `Port` is an abstract endpoint. It is not a physical device endpoint.
///
/// # Identity
///
/// The stable semantic identity is:
///
/// ```text
/// PortPath {
///     namespace,
///     name,
/// }
/// ```
///
/// # Hardware independence
///
/// A port may eventually be mapped to:
///
/// - a DAC;
/// - ADC;
/// - controller;
/// - FPGA;
/// - optical modulator;
/// - microwave source;
/// - laser;
/// - readout electronics;
/// - future quantum-control technology.
///
/// None of those details are stored as mandatory fields here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Port {
    path: PortPath,
    kind: PortKind,
    direction: PortDirection,
    access: PortAccess,
    scope: PortScope,
    targets: Vec<QubitRef>,
    channel: Option<ChannelId>,
    metadata: PortMetadata,
}

impl Port {
    /// Creates a port with the canonical default semantics:
    ///
    /// - output;
    /// - exclusive;
    /// - global;
    /// - no targets;
    /// - no channel association;
    /// - empty metadata.
    pub fn new(
        namespace: PortNamespace,
        name: PortName,
        kind: PortKind,
    ) -> Self {
        Self {
            path: PortPath::new(namespace, name),
            kind,
            direction: PortDirection::default(),
            access: PortAccess::default(),
            scope: PortScope::default(),
            targets: Vec::new(),
            channel: None,
            metadata: PortMetadata::new(),
        }
    }

    /// Creates a port directly from strings.
    pub fn try_new<N, P>(
        namespace: N,
        name: P,
        kind: PortKind,
    ) -> PortResult<Self>
    where
        N: Into<String>,
        P: Into<String>,
    {
        Ok(Self::new(
            PortNamespace::new(namespace)?,
            PortName::new(name)?,
            kind,
        ))
    }

    /// Creates a port using explicit validation limits.
    pub fn try_new_with_limits<N, P>(
        namespace: N,
        name: P,
        kind: PortKind,
        limits: PortValidationLimits,
    ) -> PortResult<Self>
    where
        N: Into<String>,
        P: Into<String>,
    {
        let namespace = namespace.into();
        let name = name.into();

        validate_text(
            &namespace,
            limits.max_namespace_bytes,
            "port namespace",
        )?;

        validate_text(
            &name,
            limits.max_name_bytes,
            "port name",
        )?;

        if namespace.is_empty() {
            return Err(PortError::EmptyNamespace);
        }

        if name.is_empty() {
            return Err(PortError::EmptyName);
        }

        let mut port = Self {
            path: PortPath::new(
                PortNamespace(namespace),
                PortName(name),
            ),
            kind,
            direction: PortDirection::default(),
            access: PortAccess::default(),
            scope: PortScope::default(),
            targets: Vec::new(),
            channel: None,
            metadata: PortMetadata::new(),
        };

        port.validate_with_limits(limits)?;
        Ok(port)
    }

    /// Returns the stable semantic port path.
    #[must_use]
    pub fn path(&self) -> &PortPath {
        &self.path
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &PortNamespace {
        self.path.namespace()
    }

    /// Returns the port name.
    #[must_use]
    pub fn name(&self) -> &PortName {
        self.path.name()
    }

    /// Returns the fully-qualified semantic name.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        self.path.qualified_name()
    }

    /// Returns the semantic port kind.
    #[must_use]
    pub fn kind(&self) -> &PortKind {
        &self.kind
    }

    /// Returns the signal direction.
    #[must_use]
    pub const fn direction(&self) -> PortDirection {
        self.direction
    }

    /// Returns the semantic access mode.
    #[must_use]
    pub const fn access(&self) -> PortAccess {
        self.access
    }

    /// Returns the semantic scope.
    #[must_use]
    pub const fn scope(&self) -> PortScope {
        self.scope
    }

    /// Returns the associated abstract channel, if any.
    #[must_use]
    pub const fn channel(&self) -> Option<ChannelId> {
        self.channel
    }

    /// Returns all semantic targets.
    #[must_use]
    pub fn targets(&self) -> &[QubitRef] {
        &self.targets
    }

    /// Returns the number of targets.
    #[must_use]
    pub fn target_count(&self) -> usize {
        self.targets.len()
    }

    /// Returns whether no explicit targets are attached.
    #[must_use]
    pub fn is_untargeted(&self) -> bool {
        self.targets.is_empty()
    }

    /// Returns metadata.
    #[must_use]
    pub const fn metadata(&self) -> &PortMetadata {
        &self.metadata
    }

    /// Sets the port direction.
    pub fn set_direction(
        &mut self,
        direction: PortDirection,
    ) {
        self.direction = direction;
    }

    /// Sets the port access mode.
    pub fn set_access(
        &mut self,
        access: PortAccess,
    ) {
        self.access = access;
    }

    /// Sets the port scope.
    pub fn set_scope(
        &mut self,
        scope: PortScope,
    ) {
        self.scope = scope;
    }

    /// Associates this port with an abstract channel.
    ///
    /// This does not resolve the channel to hardware.
    pub fn set_channel(
        &mut self,
        channel: Option<ChannelId>,
    ) {
        self.channel = channel;
    }

    /// Adds a semantic target using the default validation policy.
    pub fn add_target(
        &mut self,
        target: QubitRef,
    ) -> PortResult<()> {
        self.add_target_with_limits(
            target,
            PortValidationLimits::default(),
        )
    }

    /// Adds a semantic target using an explicit policy.
    pub fn add_target_with_limits(
        &mut self,
        target: QubitRef,
        limits: PortValidationLimits,
    ) -> PortResult<()> {
        if self.targets.len() >= limits.max_targets {
            return Err(PortError::TargetLimitExceeded {
                limit: limits.max_targets,
            });
        }

        if self.targets.contains(&target) {
            return Err(PortError::DuplicateTarget(target));
        }

        self.targets.push(target);
        Ok(())
    }

    /// Adds multiple semantic targets.
    ///
    /// Validation occurs before mutation so a failed operation does not leave
    /// the port partially modified.
    pub fn add_targets<I>(
        &mut self,
        targets: I,
    ) -> PortResult<()>
    where
        I: IntoIterator<Item = QubitRef>,
    {
        self.add_targets_with_limits(
            targets,
            PortValidationLimits::default(),
        )
    }

    /// Adds multiple semantic targets under an explicit policy.
    ///
    /// The incoming collection is first materialized so the operation can be
    /// validated transactionally before modifying `self`.
    pub fn add_targets_with_limits<I>(
        &mut self,
        targets: I,
        limits: PortValidationLimits,
    ) -> PortResult<()>
    where
        I: IntoIterator<Item = QubitRef>,
    {
        let incoming: Vec<QubitRef> =
            targets.into_iter().collect();

        let new_count = self
            .targets
            .len()
            .checked_add(incoming.len())
            .ok_or(PortError::TargetCountOverflow)?;

        if new_count > limits.max_targets {
            return Err(PortError::TargetLimitExceeded {
                limit: limits.max_targets,
            });
        }

        for (index, target) in incoming.iter().enumerate() {
            if incoming[..index].contains(target)
                || self.targets.contains(target)
            {
                return Err(PortError::DuplicateTarget(*target));
            }
        }

        self.targets.extend(incoming);
        Ok(())
    }

    /// Removes one semantic target.
    pub fn remove_target(
        &mut self,
        target: QubitRef,
    ) -> bool {
        if let Some(index) =
            self.targets.iter().position(|value| *value == target)
        {
            self.targets.remove(index);
            true
        } else {
            false
        }
    }

    /// Removes all semantic targets.
    pub fn clear_targets(&mut self) {
        self.targets.clear();
    }

    /// Adds or replaces a metadata field using the default policy.
    pub fn insert_metadata<K, V>(
        &mut self,
        key: K,
        value: V,
    ) -> PortResult<Option<String>>
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.metadata.insert(key, value)
    }

    /// Adds or replaces a metadata field under an explicit policy.
    pub fn insert_metadata_with_limits<K, V>(
        &mut self,
        key: K,
        value: V,
        limits: PortValidationLimits,
    ) -> PortResult<Option<String>>
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.metadata.insert_with_limits(
            key,
            value,
            limits.max_metadata_fields,
            limits.max_metadata_key_bytes,
            limits.max_metadata_value_bytes,
        )
    }

    /// Returns whether this port is compatible with an operation direction.
    ///
    /// This checks only semantic direction. It does not check hardware
    /// capability.
    #[must_use]
    pub const fn accepts_direction(
        &self,
        requested: PortDirection,
    ) -> bool {
        match (self.direction, requested) {
            (
                PortDirection::Bidirectional,
                _,
            )
            | (
                _,
                PortDirection::Unspecified,
            ) => true,

            (
                PortDirection::Output,
                PortDirection::Output,
            )
            | (
                PortDirection::Input,
                PortDirection::Input,
            )
            | (
                PortDirection::Bidirectional,
                PortDirection::Input,
            )
            | (
                PortDirection::Bidirectional,
                PortDirection::Output,
            )
            | (
                PortDirection::Bidirectional,
                PortDirection::Bidirectional,
            ) => true,

            _ => false,
        }
    }

    /// Returns whether a target is associated with this port.
    #[must_use]
    pub fn has_target(
        &self,
        target: QubitRef,
    ) -> bool {
        self.targets.contains(&target)
    }

    /// Returns whether the port semantically permits the supplied target.
    ///
    /// A global port accepts any target because it has no explicit target
    /// restriction.
    #[must_use]
    pub fn accepts_target(
        &self,
        target: QubitRef,
    ) -> bool {
        match self.scope {
            PortScope::Global => true,

            PortScope::PerTarget
            | PortScope::MultiTarget
            | PortScope::Pairwise
            | PortScope::Custom => {
                self.targets.contains(&target)
            }
        }
    }

    /// Performs structural and semantic validation with default limits.
    pub fn validate(&self) -> PortResult<()> {
        self.validate_with_limits(
            PortValidationLimits::default(),
        )
    }

    /// Performs structural and semantic validation with explicit limits.
    pub fn validate_with_limits(
        &self,
        limits: PortValidationLimits,
    ) -> PortResult<()> {
        validate_text(
            self.namespace().as_str(),
            limits.max_namespace_bytes,
            "port namespace",
        )?;

        validate_text(
            self.name().as_str(),
            limits.max_name_bytes,
            "port name",
        )?;

        if self.namespace().as_str().is_empty() {
            return Err(PortError::EmptyNamespace);
        }

        if self.name().as_str().is_empty() {
            return Err(PortError::EmptyName);
        }

        if self.targets.len() > limits.max_targets {
            return Err(PortError::TargetLimitExceeded {
                limit: limits.max_targets,
            });
        }

        for (index, target) in self.targets.iter().enumerate() {
            if self.targets[..index].contains(target) {
                return Err(PortError::DuplicateTarget(*target));
            }
        }

        for (key, value) in self.metadata.iter() {
            validate_text(
                key,
                limits.max_metadata_key_bytes,
                "metadata key",
            )?;

            validate_text(
                value,
                limits.max_metadata_value_bytes,
                "metadata value",
            )?;
        }

        if self.metadata.len() > limits.max_metadata_fields {
            return Err(
                PortError::MetadataFieldLimitExceeded {
                    limit: limits.max_metadata_fields,
                },
            );
        }

        validate_scope_invariants(
            self.scope,
            self.targets.len(),
        )?;

        validate_direction_invariants(
            self.direction,
            self.kind(),
        )?;

        validate_access_invariants(
            self.access,
            self.direction,
        )?;

        Ok(())
    }

    /// Returns a deterministic semantic fingerprint input.
    ///
    /// This is NOT a cryptographic hash. `hash.rs` remains responsible for
    /// canonical cryptographic content hashing.
    ///
    /// The returned representation is intentionally simple and deterministic.
    #[must_use]
    pub fn canonical_identity(&self) -> String {
        let mut result = String::new();

        result.push_str(PORT_SCHEMA_ID);
        result.push('|');
        result.push_str(&PORT_SCHEMA_VERSION.to_string());
        result.push('|');
        result.push_str(self.namespace().as_str());
        result.push('|');
        result.push_str(self.name().as_str());
        result.push('|');
        result.push_str(self.kind().name());
        result.push('|');
        result.push_str(&self.direction().to_string());
        result.push('|');
        result.push_str(&self.access().to_string());
        result.push('|');
        result.push_str(&self.scope().to_string());
        result.push('|');

        match self.channel() {
            Some(channel) => {
                result.push_str(&channel.value().to_string());
            }
            None => {
                result.push('-');
            }
        }

        result.push('|');

        for target in self.targets() {
            result.push_str(&target.to_string());
            result.push(',');
        }

        result.push('|');

        for (key, value) in self.metadata.iter() {
            result.push_str(key);
            result.push('=');
            result.push_str(value);
            result.push(';');
        }

        result
    }
}

impl fmt::Display for Port {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} [{}; {}; {}; {}]",
            self.qualified_name(),
            self.kind,
            self.direction,
            self.access,
            self.scope
        )
    }
}

// =============================================================================
// Port builder
// =============================================================================

/// Builder for deterministic port construction.
///
/// The builder validates the final structure before returning a `Port`.
#[derive(Debug, Clone)]
pub struct PortBuilder {
    port: Port,
    limits: PortValidationLimits,
}

impl PortBuilder {
    /// Creates a builder using default validation limits.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
        kind: PortKind,
    ) -> PortResult<Self> {
        Self::with_limits(
            namespace,
            name,
            kind,
            PortValidationLimits::default(),
        )
    }

    /// Creates a builder using explicit validation limits.
    pub fn with_limits(
        namespace: impl Into<String>,
        name: impl Into<String>,
        kind: PortKind,
        limits: PortValidationLimits,
    ) -> PortResult<Self> {
        let port = Port::try_new_with_limits(
            namespace,
            name,
            kind,
            limits,
        )?;

        Ok(Self { port, limits })
    }

    /// Sets direction.
    #[must_use]
    pub fn direction(
        mut self,
        direction: PortDirection,
    ) -> Self {
        self.port.set_direction(direction);
        self
    }

    /// Sets access mode.
    #[must_use]
    pub fn access(
        mut self,
        access: PortAccess,
    ) -> Self {
        self.port.set_access(access);
        self
    }

    /// Sets scope.
    #[must_use]
    pub fn scope(
        mut self,
        scope: PortScope,
    ) -> Self {
        self.port.set_scope(scope);
        self
    }

    /// Associates an abstract channel.
    #[must_use]
    pub fn channel(
        mut self,
        channel: ChannelId,
    ) -> Self {
        self.port.set_channel(Some(channel));
        self
    }

    /// Adds one target.
    pub fn target(
        mut self,
        target: QubitRef,
    ) -> PortResult<Self> {
        self.port
            .add_target_with_limits(target, self.limits)?;

        Ok(self)
    }

    /// Adds multiple targets.
    pub fn targets<I>(
        mut self,
        targets: I,
    ) -> PortResult<Self>
    where
        I: IntoIterator<Item = QubitRef>,
    {
        self.port
            .add_targets_with_limits(targets, self.limits)?;

        Ok(self)
    }

    /// Adds metadata.
    pub fn metadata<K, V>(
        mut self,
        key: K,
        value: V,
    ) -> PortResult<Self>
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.port.insert_metadata_with_limits(
            key,
            value,
            self.limits,
        )?;

        Ok(self)
    }

    /// Finishes construction after complete validation.
    pub fn build(self) -> PortResult<Port> {
        self.port.validate_with_limits(self.limits)?;
        Ok(self.port)
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Error vocabulary for port construction and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortError {
    /// Namespace was empty.
    EmptyNamespace,

    /// Name was empty.
    EmptyName,

    /// A textual field exceeded its configured limit.
    TextTooLong {
        /// Semantic field name.
        field: &'static str,

        /// Actual UTF-8 byte length.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// Metadata key was empty.
    EmptyMetadataKey,

    /// Metadata field limit was exceeded.
    MetadataFieldLimitExceeded {
        /// Configured maximum.
        limit: usize,
    },

    /// Target limit was exceeded.
    TargetLimitExceeded {
        /// Configured maximum.
        limit: usize,
    },

    /// Target collection count overflowed `usize`.
    TargetCountOverflow,

    /// The same target was supplied more than once.
    DuplicateTarget(QubitRef),

    /// A port scope requires a target but no target was supplied.
    MissingTargetForScope(PortScope),

    /// A port scope does not permit the supplied target cardinality.
    InvalidTargetCardinality {
        /// Scope being validated.
        scope: PortScope,

        /// Number of supplied targets.
        count: usize,
    },

    /// A direction/access combination is semantically invalid.
    InvalidAccessForDirection {
        /// Access mode.
        access: PortAccess,

        /// Port direction.
        direction: PortDirection,
    },

    /// A semantic kind cannot use the selected direction.
    InvalidDirectionForKind {
        /// Port kind.
        kind: PortKind,

        /// Direction.
        direction: PortDirection,
    },
}

impl fmt::Display for PortError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyNamespace => {
                formatter.write_str("port namespace must not be empty")
            }

            Self::EmptyName => {
                formatter.write_str("port name must not be empty")
            }

            Self::TextTooLong {
                field,
                actual,
                maximum,
            } => write!(
                formatter,
                "{field} is {actual} bytes but the configured maximum is {maximum} bytes"
            ),

            Self::EmptyMetadataKey => {
                formatter.write_str("port metadata key must not be empty")
            }

            Self::MetadataFieldLimitExceeded { limit } => write!(
                formatter,
                "port metadata field limit exceeded: maximum {limit}"
            ),

            Self::TargetLimitExceeded { limit } => write!(
                formatter,
                "port target limit exceeded: maximum {limit}"
            ),

            Self::TargetCountOverflow => {
                formatter.write_str(
                    "port target count overflowed the host usize representation",
                )
            }

            Self::DuplicateTarget(target) => write!(
                formatter,
                "duplicate port target: {target}"
            ),

            Self::MissingTargetForScope(scope) => write!(
                formatter,
                "port scope {scope} requires at least one target"
            ),

            Self::InvalidTargetCardinality { scope, count } => write!(
                formatter,
                "port scope {scope} does not permit {count} targets"
            ),

            Self::InvalidAccessForDirection {
                access,
                direction,
            } => write!(
                formatter,
                "port access mode {access} is incompatible with direction {direction}"
            ),

            Self::InvalidDirectionForKind {
                kind,
                direction,
            } => write!(
                formatter,
                "port kind {kind} is incompatible with direction {direction}"
            ),
        }
    }
}

impl std::error::Error for PortError {}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_text(
    value: &str,
    maximum: usize,
    field: &'static str,
) -> PortResult<()> {
    let actual = value.len();

    if actual > maximum {
        return Err(PortError::TextTooLong {
            field,
            actual,
            maximum,
        });
    }

    Ok(())
}

fn validate_non_empty_name(
    value: &str,
    field: &'static str,
) -> PortResult<()> {
    if value.is_empty() {
        return Err(PortError::EmptyName);
    }

    if value.trim().is_empty() {
        return Err(PortError::EmptyName);
    }

    let _ = field;

    Ok(())
}

fn validate_scope_invariants(
    scope: PortScope,
    target_count: usize,
) -> PortResult<()> {
    match scope {
        PortScope::Global => {
            // Global ports may optionally carry target information as
            // descriptive metadata. No target is required.
            Ok(())
        }

        PortScope::PerTarget => {
            if target_count == 1 {
                Ok(())
            } else if target_count == 0 {
                Err(PortError::MissingTargetForScope(scope))
            } else {
                Err(PortError::InvalidTargetCardinality {
                    scope,
                    count: target_count,
                })
            }
        }

        PortScope::MultiTarget => {
            if target_count >= 1 {
                Ok(())
            } else {
                Err(PortError::MissingTargetForScope(scope))
            }
        }

        PortScope::Pairwise => {
            if target_count == 2 {
                Ok(())
            } else {
                Err(PortError::InvalidTargetCardinality {
                    scope,
                    count: target_count,
                })
            }
        }

        PortScope::Custom => {
            // Custom architectures define their own cardinality semantics.
            Ok(())
        }
    }
}

fn validate_direction_invariants(
    direction: PortDirection,
    kind: &PortKind,
) -> PortResult<()> {
    match kind {
        PortKind::Acquire
        | PortKind::Readout => match direction {
            PortDirection::Input
            | PortDirection::Bidirectional
            | PortDirection::Unspecified => Ok(()),

            PortDirection::Output => {
                Err(PortError::InvalidDirectionForKind {
                    kind: kind.clone(),
                    direction,
                })
            }
        },

        PortKind::Control
        | PortKind::Measure
        | PortKind::Analog
        | PortKind::Microwave
        | PortKind::Flux
        | PortKind::Optical
        | PortKind::Laser
        | PortKind::Digital
        | PortKind::Trigger
        | PortKind::Synchronization
        | PortKind::Custom(_) => match direction {
            PortDirection::Output
            | PortDirection::Bidirectional
            | PortDirection::Unspecified => Ok(()),

            PortDirection::Input => {
                // Custom and general control families may legitimately be
                // input ports on future architectures. The semantic model
                // therefore rejects this only for specifically input-oriented
                // kinds above, not here.
                Ok(())
            }
        },
    }
}

fn validate_access_invariants(
    access: PortAccess,
    direction: PortDirection,
) -> PortResult<()> {
    match access {
        PortAccess::ReadOnly => match direction {
            PortDirection::Output
            | PortDirection::Input
            | PortDirection::Bidirectional
            | PortDirection::Unspecified => Ok(()),
        },

        PortAccess::WriteOnly => match direction {
            PortDirection::Input => {
                Err(PortError::InvalidAccessForDirection {
                    access,
                    direction,
                })
            }

            PortDirection::Output
            | PortDirection::Bidirectional
            | PortDirection::Unspecified => Ok(()),
        },

        PortAccess::Exclusive
        | PortAccess::Shared
        | PortAccess::TargetDefined => Ok(()),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::qubit::{
        PhysicalQubitId,
        QubitId,
        QubitRef,
    };

    #[test]
    fn namespace_and_name_are_deterministic() {
        let path = PortPath::from_strings(
            "zamani.control",
            "drive",
        )
        .expect("valid port path");

        assert_eq!(
            path.qualified_name(),
            "zamani.control:drive"
        );
    }

    #[test]
    fn port_is_hardware_independent() {
        let port = Port::try_new(
            "zamani.control",
            "drive",
            PortKind::Control,
        )
        .expect("valid port");

        assert_eq!(
            port.kind(),
            &PortKind::Control
        );

        assert!(port.channel().is_none());
        assert!(port.is_untargeted());
    }

    #[test]
    fn logical_qubit_target_uses_canonical_qubit_type() {
        let mut port = Port::try_new(
            "zamani.control",
            "q0-drive",
            PortKind::Control,
        )
        .expect("valid port");

        port.set_scope(PortScope::PerTarget);

        port.add_target(QubitRef::Logical(
            QubitId::new(0),
        ))
        .expect("valid target");

        assert_eq!(
            port.target_count(),
            1
        );

        assert!(
            port.has_target(
                QubitRef::Logical(QubitId::new(0))
            )
        );

        assert!(port.validate().is_ok());
    }

    #[test]
    fn physical_qubit_reference_remains_explicit() {
        let mut port = Port::try_new(
            "zamani.control",
            "physical-drive",
            PortKind::Control,
        )
        .expect("valid port");

        port.set_scope(PortScope::PerTarget);

        port.add_target(QubitRef::Physical(
            PhysicalQubitId::new(17),
        ))
        .expect("valid target");

        assert!(
            port.accepts_target(
                QubitRef::Physical(
                    PhysicalQubitId::new(17)
                )
            )
        );

        assert!(port.validate().is_ok());
    }

    #[test]
    fn pairwise_scope_requires_exactly_two_targets() {
        let mut port = Port::try_new(
            "zamani.control",
            "pair",
            PortKind::Control,
        )
        .expect("valid port");

        port.set_scope(PortScope::Pairwise);

        assert!(port.validate().is_err());

        port.add_targets([
            QubitRef::Logical(QubitId::new(0)),
            QubitRef::Logical(QubitId::new(1)),
        ])
        .expect("two unique targets");

        assert!(port.validate().is_ok());
    }

    #[test]
    fn duplicate_targets_are_rejected() {
        let mut port = Port::try_new(
            "zamani.control",
            "drive",
            PortKind::Control,
        )
        .expect("valid port");

        let target =
            QubitRef::Logical(QubitId::new(3));

        port.add_target(target)
            .expect("first target");

        assert_eq!(
            port.add_target(target),
            Err(PortError::DuplicateTarget(target))
        );
    }

    #[test]
    fn target_limit_is_explicit_policy() {
        let limits = PortValidationLimits {
            max_targets: 2,
            ..PortValidationLimits::unrestricted()
        };

        let mut port = Port::try_new_with_limits(
            "zamani.control",
            "drive",
            PortKind::Control,
            limits,
        )
        .expect("valid port");

        port.add_target_with_limits(
            QubitRef::Logical(QubitId::new(0)),
            limits,
        )
        .expect("first target");

        port.add_target_with_limits(
            QubitRef::Logical(QubitId::new(1)),
            limits,
        )
        .expect("second target");

        assert_eq!(
            port.add_target_with_limits(
                QubitRef::Logical(QubitId::new(2)),
                limits,
            ),
            Err(PortError::TargetLimitExceeded {
                limit: 2,
            })
        );
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let mut metadata = PortMetadata::new();

        metadata
            .insert("z", "last")
            .expect("valid metadata");

        metadata
            .insert("a", "first")
            .expect("valid metadata");

        let fields: Vec<_> =
            metadata.iter().collect();

        assert_eq!(
            fields,
            vec![
                ("a", "first"),
                ("z", "last"),
            ]
        );
    }

    #[test]
    fn metadata_limits_are_enforced() {
        let mut metadata = PortMetadata::new();

        assert_eq!(
            metadata.insert_with_limits(
                "a",
                "12345",
                1,
                1,
                4,
            ),
            Err(PortError::TextTooLong {
                field: "metadata value",
                actual: 5,
                maximum: 4,
            })
        );
    }

    #[test]
    fn builder_produces_valid_port() {
        let port = PortBuilder::new(
            "zamani.control",
            "drive",
            PortKind::Control,
        )
        .expect("builder")
        .direction(PortDirection::Output)
        .access(PortAccess::Exclusive)
        .scope(PortScope::PerTarget)
        .target(QubitRef::Logical(QubitId::new(5)))
        .expect("target")
        .channel(ChannelId::new(42))
        .metadata("purpose", "single-qubit-control")
        .expect("metadata")
        .build()
        .expect("valid port");

        assert_eq!(
            port.qualified_name(),
            "zamani.control:drive"
        );

        assert_eq!(
            port.channel(),
            Some(ChannelId::new(42))
        );

        assert_eq!(
            port.metadata().get("purpose"),
            Some("single-qubit-control")
        );
    }

    #[test]
    fn acquire_port_is_input_or_bidirectional() {
        let input = PortBuilder::new(
            "zamani.readout",
            "acquire",
            PortKind::Acquire,
        )
        .expect("builder")
        .direction(PortDirection::Input)
        .build();

        assert!(input.is_ok());

        let output = PortBuilder::new(
            "zamani.readout",
            "invalid-acquire",
            PortKind::Acquire,
        )
        .expect("builder")
        .direction(PortDirection::Output)
        .build();

        assert!(matches!(
            output,
            Err(PortError::InvalidDirectionForKind {
                kind: PortKind::Acquire,
                direction: PortDirection::Output,
            })
        ));
    }

    #[test]
    fn write_only_input_is_rejected() {
        let result = PortBuilder::new(
            "zamani.control",
            "invalid",
            PortKind::Control,
        )
        .expect("builder")
        .direction(PortDirection::Input)
        .access(PortAccess::WriteOnly)
        .build();

        assert!(matches!(
            result,
            Err(PortError::InvalidAccessForDirection {
                access: PortAccess::WriteOnly,
                direction: PortDirection::Input,
            })
        ));
    }

    #[test]
    fn global_port_does_not_require_targets() {
        let port = Port::try_new(
            "zamani.control",
            "global-drive",
            PortKind::Control,
        )
        .expect("valid port");

        assert!(port.is_untargeted());
        assert!(port.validate().is_ok());
    }

    #[test]
    fn canonical_identity_is_deterministic() {
        let mut first = PortBuilder::new(
            "zamani.control",
            "drive",
            PortKind::Control,
        )
        .expect("builder")
        .target(QubitRef::Logical(QubitId::new(0)))
        .expect("target")
        .metadata("b", "2")
        .expect("metadata")
        .metadata("a", "1")
        .expect("metadata")
        .build()
        .expect("port");

        let mut second = PortBuilder::new(
            "zamani.control",
            "drive",
            PortKind::Control,
        )
        .expect("builder")
        .target(QubitRef::Logical(QubitId::new(0)))
        .expect("target")
        .metadata("a", "1")
        .expect("metadata")
        .metadata("b", "2")
        .expect("metadata")
        .build()
        .expect("port");

        first.set_channel(Some(ChannelId::new(7)));
        second.set_channel(Some(ChannelId::new(7)));

        assert_eq!(
            first.canonical_identity(),
            second.canonical_identity()
        );
    }

    #[test]
    fn unrestricted_policy_has_no_artificial_limits() {
        let limits =
            PortValidationLimits::unrestricted();

        assert_eq!(
            limits.max_targets,
            usize::MAX
        );

        assert_eq!(
            limits.max_metadata_fields,
            usize::MAX
        );
    }

    #[test]
    fn custom_port_kind_is_supported() {
        let kind =
            PortKind::custom("future.quantum.interface")
                .expect("valid custom kind");

        assert!(kind.is_custom());
        assert_eq!(
            kind.name(),
            "future.quantum.interface"
        );
    }

    #[test]
    fn custom_scope_can_be_used_for_future_architectures() {
        let mut port = Port::try_new(
            "future",
            "resource",
            PortKind::Custom(
                "future.interface".to_string()
            ),
        )
        .expect("valid port");

        port.set_scope(PortScope::Custom);

        assert!(port.validate().is_ok());
    }
}