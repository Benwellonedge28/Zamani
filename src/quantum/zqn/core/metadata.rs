//! Zamani Quantum Noise (ZQN) — Core Metadata
//!
//! Production-grade, backend-independent metadata model for the ZQN subsystem.
//!
//! # Purpose
//!
//! This module owns metadata that describes ZQN artifacts without becoming part
//! of their mathematical semantics.
//!
//! Metadata can describe:
//!
//! - noise models;
//! - channels;
//! - faults;
//! - calibration snapshots;
//! - characterization results;
//! - simulation configurations;
//! - target compatibility information;
//! - error budgets;
//! - provenance references;
//! - implementation-independent annotations.
//!
//! Metadata is descriptive. It is not executable quantum semantics.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                       quantum::frontend
//!                              │
//!                              ▼
//!                       quantum::ir
//!                              │
//!                 canonical computation semantics
//!                              │
//!              ┌───────────────┴───────────────┐
//!              │                               │
//!              ▼                               ▼
//!       quantum::zqn                       other IR consumers
//!              │
//!              ▼
//!       ZQN semantic objects
//!              │
//!       ┌──────┼────────┬──────────┐
//!       ▼      ▼        ▼          ▼
//!     noise  channel   fault   calibration
//!       │      │        │          │
//!       └──────┴────────┴──────────┘
//!                    │
//!                    ▼
//!                metadata
//! ```
//!
//! This module does not execute any of those systems.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - metadata values;
//! - metadata keys;
//! - metadata namespaces;
//! - metadata classification;
//! - metadata collections;
//! - deterministic metadata ordering;
//! - metadata validation;
//! - caller-configurable metadata validation limits;
//! - immutable metadata snapshots;
//! - metadata merge semantics;
//! - metadata lookup/removal/update operations.
//!
//! This file does NOT own:
//!
//! - quantum-state semantics;
//! - channels;
//! - noise models;
//! - faults;
//! - calibration mathematics;
//! - hardware topology;
//! - target capabilities;
//! - QPU execution;
//! - simulation;
//! - hashing;
//! - digital signatures;
//! - serialization formats;
//! - source parsing;
//! - qubit identity;
//! - logical-to-physical mapping;
//! - routing;
//! - scheduling;
//! - QEC;
//! - benchmarking.
//!
//! # Canonical quantum identity boundary
//!
//! ZQN must never define a competing `QubitId` or `PhysicalQubitId`.
//!
//! The canonical identities remain:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file does not directly depend on those types because generic metadata
//! does not need to know what a quantum resource is.
//!
//! When ZQN metadata eventually needs a strongly typed quantum-resource target,
//! that belongs in the annotation/integration layer and must use the canonical
//! IR types above.
//!
//! # Write once, scale everywhere
//!
//! No semantic machine-size limit is encoded here.
//!
//! In particular, this module contains no:
//!
//! - maximum qubit count;
//! - maximum metadata-entry count;
//! - maximum operation count;
//! - maximum channel count;
//! - maximum noise-model size;
//! - maximum machine size.
//!
//! Metadata collections grow according to available resources and explicit
//! caller-selected policies.
//!
//! This means:
//!
//! ```text
//! tiny system
//!     │
//!     ▼
//! large system
//!     │
//!     ▼
//! distributed system
//!     │
//!     ▼
//! arbitrarily large finite system
//! ```
//!
//! can use the same metadata abstraction.
//!
//! "Infinity" means that this semantic API does not encode an artificial finite
//! machine-size ceiling. Physical memory, address space, storage, execution
//! time, and explicit resource policies still necessarily bound any concrete
//! execution.
//!
//! # Determinism
//!
//! Determinism is a first-class requirement.
//!
//! This module therefore uses `BTreeMap` instead of `HashMap` for metadata
//! ordering.
//!
//! Metadata:
//!
//! - has deterministic key ordering;
//! - never generates random identifiers;
//! - never reads the system clock;
//! - never reads process IDs;
//! - never reads thread IDs;
//! - never reads memory addresses;
//! - never performs implicit I/O;
//! - never introduces nondeterministic values.
//!
//! Observational values such as timestamps are permitted as explicit metadata
//! values supplied by callers. This module never generates them automatically.
//!
//! # Semantic versus observational metadata
//!
//! ZQN must distinguish information that affects semantic identity from
//! information that merely describes an observation.
//!
//! ```text
//! Semantic
//!     │
//!     ├── model parameters
//!     ├── declared approximation
//!     ├── channel configuration
//!     └── mathematical assumptions
//!
//! Observational
//!     │
//!     ├── measurement timestamp
//!     ├── host information
//!     ├── execution identifier
//!     └── deployment observation
//! ```
//!
//! Consumers deciding whether metadata participates in a semantic fingerprint
//! must use the classification explicitly.
//!
//! # Security
//!
//! Metadata is untrusted at:
//!
//! - source boundaries;
//! - serialization boundaries;
//! - plugin boundaries;
//! - network boundaries;
//! - hardware-provider boundaries;
//! - calibration-import boundaries.
//!
//! Metadata must therefore be validated before trusted interpretation.
//!
//! Metadata values are data only. They must never be interpreted as executable
//! code, commands, paths to execute, or dynamic library identifiers.
//!
//! This module contains no unsafe code.
//!
//! # Resource safety
//!
//! This file deliberately does not impose global constants such as:
//!
//! ```text
//! MAX_METADATA_ENTRIES
//! MAX_METADATA_BYTES
//! MAX_METADATA_DEPTH
//! ```
//!
//! Such constants would incorrectly turn implementation policy into a semantic
//! architecture limit.
//!
//! Instead, validation accepts `MetadataLimits` supplied by the caller.
//!
//! A caller may therefore select:
//!
//! ```text
//! unlimited / policy-controlled
//! small embedded-device policy
//! compiler policy
//! server policy
//! fuzzing policy
//! untrusted-input policy
//! ```
//!
//! independently.
//!
//! # Dependency boundary
//!
//! This file intentionally depends only on the Rust standard library.
//!
//! It must not depend on:
//!
//! - `quantum::frontend`;
//! - `quantum::hardware`;
//! - `quantum::memory`;
//! - `quantum::routing`;
//! - `quantum::scheduling`;
//! - `quantum::error_correction`;
//! - `quantum::benchmarking`;
//! - ZQN channel implementations;
//! - ZQN noise implementations;
//! - ZQN simulation implementations;
//! - vendor SDKs.
//!
//! This allows this file to be completed before those subsystems exist.
//!
//! # Serialization contract
//!
//! This file defines an in-memory model only.
//!
//! Serialization belongs to:
//!
//! ```text
//! quantum::zqn::io
//! ```
//!
//! or an explicitly designated higher-level ZQN serialization module.
//!
//! Serializers must preserve:
//!
//! - namespace;
//! - key;
//! - classification;
//! - value type;
//! - nested values;
//! - map ordering semantics;
//! - byte values.
//!
//! A serializer must never silently convert a typed metadata value into an
//! unrelated type.
//!
//! # Hashing contract
//!
//! This module does not implement cryptographic hashing.
//!
//! Canonical content hashing belongs to the ZQN hashing/identity layer or the
//! canonical IR hashing subsystem.
//!
//! Consumers may derive deterministic semantic material from this model because
//! keys are ordered and semantic/observational classification is explicit.
//!
//! # Integration contract
//!
//! Future ZQN modules may use this type without changing this file:
//!
//! ```text
//! core::metadata
//!       │
//!       ├── noise
//!       ├── channel
//!       ├── fault
//!       ├── calibration
//!       ├── characterization
//!       ├── simulation
//!       ├── propagation
//!       ├── target
//!       └── integration
//! ```
//!
//! These modules should depend on this stable API rather than inventing
//! subsystem-specific key/value metadata containers.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code;
//! - standard library only.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;

// =============================================================================
// Metadata key validation
// =============================================================================

/// Default maximum UTF-8 byte length used by [`MetadataLimits`].
///
/// This is a validation-policy default, not a semantic ZQN limit.
///
/// Applications with different requirements should construct their own
/// [`MetadataLimits`] rather than modifying this constant.
pub const DEFAULT_MAX_KEY_BYTES: usize = 4 * 1024;

/// Default maximum UTF-8 byte length for metadata string values.
///
/// This is a validation-policy default, not a semantic ZQN limit.
pub const DEFAULT_MAX_STRING_BYTES: usize = 1024 * 1024;

/// Default maximum metadata nesting depth.
///
/// This protects untrusted recursive values from stack/resource exhaustion.
///
/// It is not a ZQN semantic limit.
pub const DEFAULT_MAX_DEPTH: usize = 64;

/// Default maximum byte-array length.
///
/// This is a validation-policy default.
pub const DEFAULT_MAX_BYTES: usize = 16 * 1024 * 1024;

/// Default maximum number of entries visited during validation.
///
/// This is a validation-policy default.
pub const DEFAULT_MAX_ENTRIES: u64 = 1_000_000;

// =============================================================================
// Metadata errors
// =============================================================================

/// Errors produced by metadata construction or validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetadataError {
    /// A metadata namespace or key is empty.
    EmptyKey {
        /// Field that was empty.
        field: &'static str,
    },

    /// A metadata namespace or key contains an invalid character.
    InvalidKey {
        /// Field being validated.
        field: &'static str,

        /// Human-readable reason.
        reason: String,
    },

    /// A string exceeded the configured policy limit.
    StringTooLarge {
        /// Location of the offending value.
        field: String,

        /// Actual UTF-8 byte length.
        actual_bytes: usize,

        /// Configured maximum.
        maximum_bytes: usize,
    },

    /// A byte sequence exceeded the configured policy limit.
    BytesTooLarge {
        /// Location of the offending value.
        field: String,

        /// Actual byte length.
        actual_bytes: usize,

        /// Configured maximum.
        maximum_bytes: usize,
    },

    /// A metadata value exceeded the configured nesting depth.
    DepthExceeded {
        /// Location of the offending value.
        field: String,

        /// Encountered depth.
        depth: usize,

        /// Configured maximum.
        maximum_depth: usize,
    },

    /// The metadata graph contained more entries than the configured policy.
    EntryCountExceeded {
        /// Number of visited entries.
        actual: u64,

        /// Configured maximum.
        maximum: u64,
    },

    /// A floating-point metadata value was not finite.
    NonFiniteFloat {
        /// Location of the offending value.
        field: String,
    },

    /// Two metadata values could not be merged according to the selected
    /// merge policy.
    MergeConflict {
        /// Metadata key that conflicted.
        key: String,
    },
}

impl fmt::Display for MetadataError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyKey { field } => {
                write!(formatter, "metadata {field} must not be empty")
            }

            Self::InvalidKey { field, reason } => {
                write!(formatter, "invalid metadata {field}: {reason}")
            }

            Self::StringTooLarge {
                field,
                actual_bytes,
                maximum_bytes,
            } => {
                write!(
                    formatter,
                    "metadata string `{field}` is {actual_bytes} bytes; \
                     maximum is {maximum_bytes}"
                )
            }

            Self::BytesTooLarge {
                field,
                actual_bytes,
                maximum_bytes,
            } => {
                write!(
                    formatter,
                    "metadata byte value `{field}` is {actual_bytes} bytes; \
                     maximum is {maximum_bytes}"
                )
            }

            Self::DepthExceeded {
                field,
                depth,
                maximum_depth,
            } => {
                write!(
                    formatter,
                    "metadata value `{field}` reached depth {depth}; \
                     maximum is {maximum_depth}"
                )
            }

            Self::EntryCountExceeded { actual, maximum } => {
                write!(
                    formatter,
                    "metadata contains {actual} entries; maximum is {maximum}"
                )
            }

            Self::NonFiniteFloat { field } => {
                write!(
                    formatter,
                    "metadata floating-point value `{field}` must be finite"
                )
            }

            Self::MergeConflict { key } => {
                write!(
                    formatter,
                    "metadata merge conflict for key `{key}`"
                )
            }
        }
    }
}

impl std::error::Error for MetadataError {}

// =============================================================================
// Metadata limits
// =============================================================================

/// Explicit validation/resource policy for metadata.
///
/// These limits protect applications from malformed or hostile metadata
/// without imposing architectural limits on ZQN itself.
///
/// `None` means that the corresponding limit is not imposed by this policy.
///
/// The caller remains responsible for ensuring that an unlimited policy is
/// appropriate for the trust level and available resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetadataLimits {
    /// Maximum namespace/key UTF-8 byte length.
    pub max_key_bytes: Option<usize>,

    /// Maximum string-value UTF-8 byte length.
    pub max_string_bytes: Option<usize>,

    /// Maximum byte-value length.
    pub max_bytes: Option<usize>,

    /// Maximum recursive nesting depth.
    pub max_depth: Option<usize>,

    /// Maximum number of visited metadata entries.
    pub max_entries: Option<u64>,
}

impl MetadataLimits {
    /// Creates an unrestricted validation policy.
    ///
    /// The caller remains responsible for controlling total memory and
    /// execution resources.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_key_bytes: None,
            max_string_bytes: None,
            max_bytes: None,
            max_depth: None,
            max_entries: None,
        }
    }

    /// Creates the conservative default validation policy.
    #[must_use]
    pub const fn default_policy() -> Self {
        Self {
            max_key_bytes: Some(DEFAULT_MAX_KEY_BYTES),
            max_string_bytes: Some(DEFAULT_MAX_STRING_BYTES),
            max_bytes: Some(DEFAULT_MAX_BYTES),
            max_depth: Some(DEFAULT_MAX_DEPTH),
            max_entries: Some(DEFAULT_MAX_ENTRIES),
        }
    }
}

impl Default for MetadataLimits {
    fn default() -> Self {
        Self::default_policy()
    }
}

// =============================================================================
// Metadata classification
// =============================================================================

/// Semantic classification of metadata.
///
/// This distinction is important for deterministic identity and reproducible
/// execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MetadataClass {
    /// Metadata that describes semantic configuration and may participate in
    /// semantic identity when the consumer's policy includes it.
    Semantic,

    /// Metadata that records observations but does not inherently change the
    /// semantic artifact.
    Observational,
}

impl Default for MetadataClass {
    fn default() -> Self {
        Self::Semantic
    }
}

// =============================================================================
// Metadata value
// =============================================================================

/// Typed metadata value.
///
/// The value model is deliberately independent of JSON, YAML, TOML, or any
/// other serialization format.
///
/// Serialization adapters can map these values to an external representation
/// while preserving their type information.
///
/// Recursive maps use `BTreeMap` to guarantee deterministic ordering.
#[derive(Debug, Clone, PartialEq)]
pub enum MetadataValue {
    /// Explicitly absent/null value.
    Null,

    /// Boolean value.
    Boolean(bool),

    /// Signed integer.
    Integer(i64),

    /// Unsigned integer.
    Unsigned(u64),

    /// Finite floating-point value.
    ///
    /// Non-finite values (`NaN`, positive infinity, negative infinity) are
    /// rejected by [`MetadataValue::validate`].
    Float(f64),

    /// UTF-8 text.
    String(String),

    /// Arbitrary bytes.
    ///
    /// This is data, not executable memory.
    Bytes(Vec<u8>),

    /// Ordered recursive list.
    List(Vec<Self>),

    /// Ordered recursive map.
    Map(BTreeMap<String, Self>),
}

impl MetadataValue {
    /// Creates a string metadata value.
    #[must_use]
    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }

    /// Creates a byte metadata value.
    #[must_use]
    pub fn bytes(value: impl Into<Vec<u8>>) -> Self {
        Self::Bytes(value.into())
    }

    /// Creates an ordered metadata map.
    #[must_use]
    pub fn map() -> BTreeMap<String, Self> {
        BTreeMap::new()
    }

    /// Returns whether this value is scalar.
    #[must_use]
    pub const fn is_scalar(&self) -> bool {
        matches!(
            self,
            Self::Null
                | Self::Boolean(_)
                | Self::Integer(_)
                | Self::Unsigned(_)
                | Self::Float(_)
                | Self::String(_)
                | Self::Bytes(_)
        )
    }

    /// Returns whether this value contains recursive metadata.
    #[must_use]
    pub const fn is_composite(&self) -> bool {
        matches!(self, Self::List(_) | Self::Map(_))
    }

    /// Returns a deterministic type name.
    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Boolean(_) => "boolean",
            Self::Integer(_) => "integer",
            Self::Unsigned(_) => "unsigned",
            Self::Float(_) => "float",
            Self::String(_) => "string",
            Self::Bytes(_) => "bytes",
            Self::List(_) => "list",
            Self::Map(_) => "map",
        }
    }

    /// Validates this value against the supplied resource policy.
    pub fn validate(
        &self,
        limits: &MetadataLimits,
    ) -> Result<(), MetadataError> {
        let mut entries = 0u64;

        self.validate_at(limits, 0, "$", &mut entries)
    }

    fn validate_at(
        &self,
        limits: &MetadataLimits,
        depth: usize,
        field: &str,
        entries: &mut u64,
    ) -> Result<(), MetadataError> {
        increment_entry_count(limits, entries)?;

        if let Some(max_depth) = limits.max_depth {
            if depth > max_depth {
                return Err(MetadataError::DepthExceeded {
                    field: field.to_owned(),
                    depth,
                    maximum_depth: max_depth,
                });
            }
        }

        match self {
            Self::Null | Self::Boolean(_) | Self::Integer(_) | Self::Unsigned(_) => {}

            Self::Float(value) => {
                if !value.is_finite() {
                    return Err(MetadataError::NonFiniteFloat {
                        field: field.to_owned(),
                    });
                }
            }

            Self::String(value) => {
                validate_string_size(
                    limits,
                    "value",
                    field,
                    value.len(),
                )?;
            }

            Self::Bytes(value) => {
                if let Some(maximum) = limits.max_bytes {
                    if value.len() > maximum {
                        return Err(MetadataError::BytesTooLarge {
                            field: field.to_owned(),
                            actual_bytes: value.len(),
                            maximum_bytes: maximum,
                        });
                    }
                }
            }

            Self::List(values) => {
                for (index, value) in values.iter().enumerate() {
                    let child_field = format!("{field}[{index}]");

                    value.validate_at(
                        limits,
                        depth.saturating_add(1),
                        &child_field,
                        entries,
                    )?;
                }
            }

            Self::Map(values) => {
                for (key, value) in values {
                    validate_metadata_key(limits, "map key", key)?;

                    let child_field = format!("{field}.{key}");

                    value.validate_at(
                        limits,
                        depth.saturating_add(1),
                        &child_field,
                        entries,
                    )?;
                }
            }
        }

        Ok(())
    }

    /// Returns an immutable string reference when this is a string.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    /// Returns a boolean when this is a boolean.
    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns a signed integer when this is an integer.
    #[must_use]
    pub const fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns an unsigned integer when this is an unsigned integer.
    #[must_use]
    pub const fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Unsigned(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns a floating-point value when this is a float.
    #[must_use]
    pub const fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Float(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns an immutable byte slice when this is a byte value.
    #[must_use]
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Bytes(value) => Some(value.as_slice()),
            _ => None,
        }
    }

    /// Returns an immutable list when this is a list.
    #[must_use]
    pub fn as_list(&self) -> Option<&[Self]> {
        match self {
            Self::List(value) => Some(value.as_slice()),
            _ => None,
        }
    }

    /// Returns an immutable map when this is a map.
    #[must_use]
    pub fn as_map(&self) -> Option<&BTreeMap<String, Self>> {
        match self {
            Self::Map(value) => Some(value),
            _ => None,
        }
    }
}

// =============================================================================
// Metadata entry
// =============================================================================

/// One metadata entry.
///
/// The entry carries both its value and its semantic/observational
/// classification.
#[derive(Debug, Clone, PartialEq)]
pub struct MetadataEntry {
    class: MetadataClass,
    value: MetadataValue,
}

impl MetadataEntry {
    /// Creates a metadata entry.
    #[must_use]
    pub const fn new(
        class: MetadataClass,
        value: MetadataValue,
    ) -> Self {
        Self { class, value }
    }

    /// Creates semantic metadata.
    #[must_use]
    pub const fn semantic(value: MetadataValue) -> Self {
        Self::new(MetadataClass::Semantic, value)
    }

    /// Creates observational metadata.
    #[must_use]
    pub const fn observational(value: MetadataValue) -> Self {
        Self::new(MetadataClass::Observational, value)
    }

    /// Returns the metadata classification.
    #[must_use]
    pub const fn class(&self) -> MetadataClass {
        self.class
    }

    /// Returns the metadata value.
    #[must_use]
    pub const fn value(&self) -> &MetadataValue {
        &self.value
    }

    /// Consumes the entry and returns its value.
    #[must_use]
    pub fn into_value(self) -> MetadataValue {
        self.value
    }

    /// Validates this entry.
    pub fn validate(
        &self,
        limits: &MetadataLimits,
    ) -> Result<(), MetadataError> {
        self.value.validate(limits)
    }
}

// =============================================================================
// Metadata namespace
// =============================================================================

/// Validated metadata namespace.
///
/// A namespace groups related metadata keys and prevents unrelated systems from
/// accidentally colliding.
///
/// Examples:
///
/// ```text
/// zqn.noise
/// zqn.channel
/// zqn.calibration
/// zqn.characterization
/// zqn.simulation
/// ```
///
/// Namespaces are data identifiers, not filesystem paths.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MetadataNamespace(String);

impl MetadataNamespace {
    /// Creates and validates a metadata namespace.
    pub fn new(
        value: impl Into<String>,
        limits: &MetadataLimits,
    ) -> Result<Self, MetadataError> {
        let value = value.into();

        validate_identifier(
            limits,
            "namespace",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Returns the namespace text.
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

impl fmt::Display for MetadataNamespace {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Metadata merge policy
// =============================================================================

/// Policy used when combining metadata collections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataMergePolicy {
    /// Existing values win.
    KeepExisting,

    /// Incoming values replace existing values.
    ReplaceExisting,

    /// A conflicting key is an error.
    RejectConflicts,
}

// =============================================================================
// Metadata
// =============================================================================

/// Deterministically ordered ZQN metadata collection.
///
/// Metadata is namespaced and each key has an explicit classification.
///
/// This is the primary metadata container to be reused by downstream ZQN
/// subsystems rather than defining independent `BTreeMap<String, String>`
/// structures.
#[derive(Debug, Clone, PartialEq)]
pub struct Metadata {
    namespace: MetadataNamespace,
    entries: BTreeMap<String, MetadataEntry>,
}

impl Metadata {
    /// Creates an empty metadata collection.
    pub fn new(
        namespace: impl Into<String>,
        limits: &MetadataLimits,
    ) -> Result<Self, MetadataError> {
        Ok(Self {
            namespace: MetadataNamespace::new(
                namespace,
                limits,
            )?,
            entries: BTreeMap::new(),
        })
    }

    /// Creates an empty metadata collection with the canonical ZQN namespace.
    pub fn zqn(
        limits: &MetadataLimits,
    ) -> Result<Self, MetadataError> {
        Self::new("zqn", limits)
    }

    /// Returns the metadata namespace.
    #[must_use]
    pub fn namespace(&self) -> &MetadataNamespace {
        &self.namespace
    }

    /// Returns the number of top-level metadata entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the metadata collection contains no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns all top-level entries in deterministic key order.
    #[must_use]
    pub fn entries(&self) -> &BTreeMap<String, MetadataEntry> {
        &self.entries
    }

    /// Returns an entry by key.
    #[must_use]
    pub fn get(
        &self,
        key: &str,
    ) -> Option<&MetadataEntry> {
        self.entries.get(key)
    }

    /// Returns a metadata value by key.
    #[must_use]
    pub fn get_value(
        &self,
        key: &str,
    ) -> Option<&MetadataValue> {
        self.entries.get(key).map(MetadataEntry::value)
    }

    /// Returns whether a key exists.
    #[must_use]
    pub fn contains_key(
        &self,
        key: &str,
    ) -> bool {
        self.entries.contains_key(key)
    }

    /// Inserts or replaces an entry.
    ///
    /// The key is validated before insertion.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        entry: MetadataEntry,
        limits: &MetadataLimits,
    ) -> Result<Option<MetadataEntry>, MetadataError> {
        let key = key.into();

        validate_metadata_key(
            limits,
            "key",
            &key,
        )?;

        entry.validate(limits)?;

        Ok(self.entries.insert(key, entry))
    }

    /// Inserts a semantic value.
    pub fn insert_semantic(
        &mut self,
        key: impl Into<String>,
        value: MetadataValue,
        limits: &MetadataLimits,
    ) -> Result<Option<MetadataEntry>, MetadataError> {
        self.insert(
            key,
            MetadataEntry::semantic(value),
            limits,
        )
    }

    /// Inserts an observational value.
    pub fn insert_observational(
        &mut self,
        key: impl Into<String>,
        value: MetadataValue,
        limits: &MetadataLimits,
    ) -> Result<Option<MetadataEntry>, MetadataError> {
        self.insert(
            key,
            MetadataEntry::observational(value),
            limits,
        )
    }

    /// Removes an entry.
    pub fn remove(
        &mut self,
        key: &str,
    ) -> Option<MetadataEntry> {
        self.entries.remove(key)
    }

    /// Removes all observational entries.
    pub fn retain_semantic(&mut self) {
        self.entries
            .retain(|_, entry| entry.class() == MetadataClass::Semantic);
    }

    /// Returns an iterator over semantic metadata.
    pub fn semantic_entries(
        &self,
    ) -> impl Iterator<Item = (&String, &MetadataEntry)> {
        self.entries
            .iter()
            .filter(|(_, entry)| {
                entry.class() == MetadataClass::Semantic
            })
    }

    /// Returns an iterator over observational metadata.
    pub fn observational_entries(
        &self,
    ) -> impl Iterator<Item = (&String, &MetadataEntry)> {
        self.entries
            .iter()
            .filter(|(_, entry)| {
                entry.class() == MetadataClass::Observational
            })
    }

    /// Merges another metadata collection into this collection.
    ///
    /// The namespaces must match. Metadata from unrelated namespaces must not
    /// be silently merged because doing so could create semantic ambiguity.
    pub fn merge(
        &mut self,
        other: &Self,
        policy: MetadataMergePolicy,
        limits: &MetadataLimits,
    ) -> Result<(), MetadataError> {
        if self.namespace != other.namespace {
            return Err(MetadataError::MergeConflict {
                key: format!(
                    "namespace mismatch: `{}` vs `{}`",
                    self.namespace,
                    other.namespace
                ),
            });
        }

        for (key, entry) in &other.entries {
            match policy {
                MetadataMergePolicy::KeepExisting => {
                    if !self.entries.contains_key(key) {
                        self.insert(
                            key.clone(),
                            entry.clone(),
                            limits,
                        )?;
                    }
                }

                MetadataMergePolicy::ReplaceExisting => {
                    self.insert(
                        key.clone(),
                        entry.clone(),
                        limits,
                    )?;
                }

                MetadataMergePolicy::RejectConflicts => {
                    if self.entries.contains_key(key) {
                        return Err(MetadataError::MergeConflict {
                            key: key.clone(),
                        });
                    }

                    self.insert(
                        key.clone(),
                        entry.clone(),
                        limits,
                    )?;
                }
            }
        }

        Ok(())
    }

    /// Validates the entire metadata collection.
    pub fn validate(
        &self,
        limits: &MetadataLimits,
    ) -> Result<(), MetadataError> {
        validate_metadata_key(
            limits,
            "namespace",
            self.namespace.as_str(),
        )?;

        let mut entries = 0u64;

        for (key, entry) in &self.entries {
            validate_metadata_key(
                limits,
                "key",
                key,
            )?;

            entry.value.validate_at(
                limits,
                0,
                key,
                &mut entries,
            )?;
        }

        Ok(())
    }

    /// Returns a new collection containing only semantic metadata.
    #[must_use]
    pub fn semantic_only(&self) -> Self {
        let entries = self
            .semantic_entries()
            .map(|(key, entry)| {
                (key.clone(), entry.clone())
            })
            .collect();

        Self {
            namespace: self.namespace.clone(),
            entries,
        }
    }

    /// Returns a new collection containing only observational metadata.
    #[must_use]
    pub fn observational_only(&self) -> Self {
        let entries = self
            .observational_entries()
            .map(|(key, entry)| {
                (key.clone(), entry.clone())
            })
            .collect();

        Self {
            namespace: self.namespace.clone(),
            entries,
        }
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            namespace: MetadataNamespace("zqn".to_owned()),
            entries: BTreeMap::new(),
        }
    }
}

// =============================================================================
// Helper validation functions
// =============================================================================

fn increment_entry_count(
    limits: &MetadataLimits,
    entries: &mut u64,
) -> Result<(), MetadataError> {
    *entries = entries.saturating_add(1);

    if let Some(maximum) = limits.max_entries {
        if *entries > maximum {
            return Err(MetadataError::EntryCountExceeded {
                actual: *entries,
                maximum,
            });
        }
    }

    Ok(())
}

fn validate_string_size(
    limits: &MetadataLimits,
    field_kind: &'static str,
    field: &str,
    actual_bytes: usize,
) -> Result<(), MetadataError> {
    if let Some(maximum) = limits.max_string_bytes {
        if actual_bytes > maximum {
            return Err(MetadataError::StringTooLarge {
                field: format!("{field_kind}:{field}"),
                actual_bytes,
                maximum_bytes: maximum,
            });
        }
    }

    Ok(())
}

fn validate_metadata_key(
    limits: &MetadataLimits,
    field: &'static str,
    value: &str,
) -> Result<(), MetadataError> {
    if value.is_empty() {
        return Err(MetadataError::EmptyKey { field });
    }

    if let Some(maximum) = limits.max_key_bytes {
        if value.len() > maximum {
            return Err(MetadataError::StringTooLarge {
                field: field.to_owned(),
                actual_bytes: value.len(),
                maximum_bytes: maximum,
            });
        }
    }

    validate_identifier(limits, field, value)
}

fn validate_identifier(
    _limits: &MetadataLimits,
    field: &'static str,
    value: &str,
) -> Result<(), MetadataError> {
    if value.is_empty() {
        return Err(MetadataError::EmptyKey { field });
    }

    let mut characters = value.chars();

    let Some(first) = characters.next() else {
        return Err(MetadataError::EmptyKey { field });
    };

    if !(first.is_ascii_alphanumeric()
        || first == '_'
        || first == '-'
        || first == '.')
    {
        return Err(MetadataError::InvalidKey {
            field,
            reason: format!(
                "first character `{first}` is not permitted"
            ),
        });
    }

    for character in characters {
        if !(character.is_ascii_alphanumeric()
            || character == '_'
            || character == '-'
            || character == '.'
            || character == ':'
        ) {
            return Err(MetadataError::InvalidKey {
                field,
                reason: format!(
                    "character `{character}` is not permitted"
                ),
            });
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> MetadataLimits {
        MetadataLimits::default_policy()
    }

    #[test]
    fn default_metadata_is_empty_and_zqn_namespaced() {
        let metadata = Metadata::default();

        assert!(metadata.is_empty());
        assert_eq!(metadata.namespace().as_str(), "zqn");
    }

    #[test]
    fn namespace_validation_rejects_empty_namespace() {
        let result = MetadataNamespace::new("", &limits());

        assert!(matches!(
            result,
            Err(MetadataError::EmptyKey {
                field: "namespace"
            })
        ));
    }

    #[test]
    fn namespace_validation_rejects_invalid_character() {
        let result = MetadataNamespace::new(
            "zqn/noise",
            &limits(),
        );

        assert!(matches!(
            result,
            Err(MetadataError::InvalidKey {
                field: "namespace",
                ..
            })
        ));
    }

    #[test]
    fn metadata_keys_are_deterministically_ordered() {
        let mut metadata =
            Metadata::new("zqn.noise", &limits()).unwrap();

        metadata
            .insert_semantic(
                "z",
                MetadataValue::Unsigned(1),
                &limits(),
            )
            .unwrap();

        metadata
            .insert_semantic(
                "a",
                MetadataValue::Unsigned(2),
                &limits(),
            )
            .unwrap();

        let keys: Vec<&String> =
            metadata.entries().keys().collect();

        assert_eq!(
            keys,
            vec![&"a".to_owned(), &"z".to_owned()]
        );
    }

    #[test]
    fn semantic_and_observational_metadata_are_separated() {
        let mut metadata =
            Metadata::new("zqn", &limits()).unwrap();

        metadata
            .insert_semantic(
                "model",
                MetadataValue::string("depolarizing"),
                &limits(),
            )
            .unwrap();

        metadata
            .insert_observational(
                "observation",
                MetadataValue::string("measured"),
                &limits(),
            )
            .unwrap();

        assert_eq!(
            metadata.semantic_entries().count(),
            1
        );

        assert_eq!(
            metadata.observational_entries().count(),
            1
        );

        assert_eq!(
            metadata.semantic_only().len(),
            1
        );

        assert_eq!(
            metadata.observational_only().len(),
            1
        );
    }

    #[test]
    fn nested_values_validate() {
        let mut nested = BTreeMap::new();

        nested.insert(
            "rate".to_owned(),
            MetadataValue::Float(0.001),
        );

        nested.insert(
            "enabled".to_owned(),
            MetadataValue::Boolean(true),
        );

        let value = MetadataValue::Map(nested);

        assert!(value.validate(&limits()).is_ok());
    }

    #[test]
    fn non_finite_float_is_rejected() {
        let value = MetadataValue::Float(f64::NAN);

        assert!(matches!(
            value.validate(&limits()),
            Err(MetadataError::NonFiniteFloat { .. })
        ));
    }

    #[test]
    fn infinity_is_rejected() {
        let value =
            MetadataValue::Float(f64::INFINITY);

        assert!(matches!(
            value.validate(&limits()),
            Err(MetadataError::NonFiniteFloat { .. })
        ));
    }

    #[test]
    fn string_resource_limit_is_enforced() {
        let limits = MetadataLimits {
            max_string_bytes: Some(3),
            ..MetadataLimits::unlimited()
        };

        let value =
            MetadataValue::string("abcd");

        assert!(matches!(
            value.validate(&limits),
            Err(MetadataError::StringTooLarge { .. })
        ));
    }

    #[test]
    fn byte_resource_limit_is_enforced() {
        let limits = MetadataLimits {
            max_bytes: Some(2),
            ..MetadataLimits::unlimited()
        };

        let value =
            MetadataValue::Bytes(vec![1, 2, 3]);

        assert!(matches!(
            value.validate(&limits),
            Err(MetadataError::BytesTooLarge { .. })
        ));
    }

    #[test]
    fn depth_resource_limit_is_enforced() {
        let limits = MetadataLimits {
            max_depth: Some(1),
            ..MetadataLimits::unlimited()
        };

        let value = MetadataValue::List(vec![
            MetadataValue::List(vec![
                MetadataValue::Boolean(true),
            ]),
        ]);

        assert!(matches!(
            value.validate(&limits),
            Err(MetadataError::DepthExceeded { .. })
        ));
    }

    #[test]
    fn merge_keep_existing_preserves_original() {
        let limits = limits();

        let mut left =
            Metadata::new("zqn", &limits).unwrap();

        let mut right =
            Metadata::new("zqn", &limits).unwrap();

        left.insert_semantic(
            "value",
            MetadataValue::Unsigned(1),
            &limits,
        )
        .unwrap();

        right.insert_semantic(
            "value",
            MetadataValue::Unsigned(2),
            &limits,
        )
        .unwrap();

        left.merge(
            &right,
            MetadataMergePolicy::KeepExisting,
            &limits,
        )
        .unwrap();

        assert_eq!(
            left.get_value("value"),
            Some(&MetadataValue::Unsigned(1))
        );
    }

    #[test]
    fn merge_replace_existing_replaces_value() {
        let limits = limits();

        let mut left =
            Metadata::new("zqn", &limits).unwrap();

        let mut right =
            Metadata::new("zqn", &limits).unwrap();

        left.insert_semantic(
            "value",
            MetadataValue::Unsigned(1),
            &limits,
        )
        .unwrap();

        right.insert_semantic(
            "value",
            MetadataValue::Unsigned(2),
            &limits,
        )
        .unwrap();

        left.merge(
            &right,
            MetadataMergePolicy::ReplaceExisting,
            &limits,
        )
        .unwrap();

        assert_eq!(
            left.get_value("value"),
            Some(&MetadataValue::Unsigned(2))
        );
    }

    #[test]
    fn merge_reject_conflicts_fails() {
        let limits = limits();

        let mut left =
            Metadata::new("zqn", &limits).unwrap();

        let mut right =
            Metadata::new("zqn", &limits).unwrap();

        left.insert_semantic(
            "value",
            MetadataValue::Unsigned(1),
            &limits,
        )
        .unwrap();

        right.insert_semantic(
            "value",
            MetadataValue::Unsigned(2),
            &limits,
        )
        .unwrap();

        assert!(matches!(
            left.merge(
                &right,
                MetadataMergePolicy::RejectConflicts,
                &limits,
            ),
            Err(MetadataError::MergeConflict { .. })
        ));
    }

    #[test]
    fn different_namespaces_cannot_be_merged() {
        let limits = limits();

        let mut left =
            Metadata::new("zqn.noise", &limits).unwrap();

        let right =
            Metadata::new("zqn.channel", &limits).unwrap();

        assert!(matches!(
            left.merge(
                &right,
                MetadataMergePolicy::ReplaceExisting,
                &limits,
            ),
            Err(MetadataError::MergeConflict { .. })
        ));
    }

    #[test]
    fn metadata_can_be_validated_after_construction() {
        let limits = limits();

        let mut metadata =
            Metadata::new("zqn", &limits).unwrap();

        metadata
            .insert_semantic(
                "model",
                MetadataValue::string("thermal"),
                &limits,
            )
            .unwrap();

        metadata
            .insert_observational(
                "source",
                MetadataValue::string("characterization"),
                &limits,
            )
            .unwrap();

        assert!(metadata.validate(&limits).is_ok());
    }

    #[test]
    fn value_type_names_are_stable() {
        assert_eq!(
            MetadataValue::Null.type_name(),
            "null"
        );

        assert_eq!(
            MetadataValue::Boolean(true).type_name(),
            "boolean"
        );

        assert_eq!(
            MetadataValue::Integer(1).type_name(),
            "integer"
        );

        assert_eq!(
            MetadataValue::Unsigned(1).type_name(),
            "unsigned"
        );

        assert_eq!(
            MetadataValue::Float(1.0).type_name(),
            "float"
        );

        assert_eq!(
            MetadataValue::string("x").type_name(),
            "string"
        );

        assert_eq!(
            MetadataValue::Bytes(vec![1]).type_name(),
            "bytes"
        );
    }

    #[test]
    fn unrestricted_policy_is_available() {
        let limits = MetadataLimits::unlimited();

        let value = MetadataValue::string(
            "large-policy-value",
        );

        assert!(value.validate(&limits).is_ok());
    }

    #[test]
    fn metadata_remove_is_non_panicking() {
        let limits = limits();

        let mut metadata =
            Metadata::new("zqn", &limits).unwrap();

        assert!(metadata.remove("missing").is_none());
    }
}