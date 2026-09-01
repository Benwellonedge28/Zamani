//! Zamani Quantum IR — Vendor Dialect
//!
//! Production-grade vendor-dialect contract for the Zamani Quantum IR.
//!
//! # Purpose
//!
//! This module provides a target-independent representation for vendor- or
//! provider-specific quantum IR constructs without contaminating the canonical
//! Zamani semantic IR with backend implementation details.
//!
//! A vendor dialect may describe:
//!
//! - vendor-specific operations;
//! - vendor-specific types;
//! - vendor-specific attributes;
//! - vendor-specific resources;
//! - vendor-specific operation properties;
//! - vendor-specific calibration references;
//! - vendor-specific compilation hints;
//! - vendor-specific opaque payloads;
//! - vendor-specific qubit/resource references;
//! - vendor-specific declarations;
//! - vendor-specific compatibility information.
//!
//! It MUST NOT:
//!
//! - execute vendor operations;
//! - contact a vendor service;
//! - contain credentials;
//! - contain network clients;
//! - select hardware;
//! - route qubits;
//! - schedule operations;
//! - perform optimization;
//! - perform calibration;
//! - simulate quantum states;
//! - depend on a vendor SDK;
//! - depend on a backend implementation.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! canonical Zamani IR
//!      │
//!      ├──────────────────────────────┐
//!      │                              │
//!      ▼                              ▼
//! standard dialect              vendor dialect
//!      │                              │
//!      │                              └── opaque/vendor semantics
//!      │
//!      └──────────────┬───────────────┘
//!                     ▼
//!              target compatibility
//!                     │
//!                     ▼
//!                 lowering
//!                     │
//!                     ▼
//!                  backend
//! ```
//!
//! The vendor dialect is therefore an *IR description boundary*, not a
//! hardware execution boundary.
//!
//! # Vendor neutrality
//!
//! Vendor names are data, not Rust types.
//!
//! This module intentionally does not contain structures such as:
//!
//! ```text
//! IbmOperation
//! IonQOperation
//! RigettiOperation
//! QuantinuumOperation
//! GoogleOperation
//! DWaveOperation
//! ```
//!
//! Instead, a vendor is identified by a validated namespace/name pair.
//! This permits arbitrary future providers without modifying this file.
//!
//! # Scalability
//!
//! There is no architectural maximum for:
//!
//! - vendors;
//! - dialects;
//! - operations;
//! - operands;
//! - results;
//! - attributes;
//! - resources;
//! - qubits;
//! - declarations;
//! - payload fields;
//! - payload nesting;
//! - extensions.
//!
//! Collection sizes are constrained only by available resources and explicit
//! resource/security policies imposed by the surrounding IR/compiler layers.
//!
//! No quantum-machine size is encoded here.
//!
//! In particular, this module contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_VENDOR_ATTRIBUTES
//! MAX_VENDOR_RESOURCES
//! ```
//!
//! # Qubit identity
//!
//! Whenever a vendor declaration needs to refer to a Zamani quantum resource,
//! it uses the canonical:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! This module deliberately does not define another qubit identifier.
//!
//! Physical qubit identity remains distinct and is represented with:
//!
//! ```text
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Vendor-specific physical semantics should normally be represented as
//! vendor metadata rather than replacing canonical Zamani identities.
//!
//! # Lossless unknown-vendor handling
//!
//! A compiler that does not understand a vendor dialect must be able to:
//!
//! 1. identify the dialect;
//! 2. inspect its version;
//! 3. preserve its declaration;
//! 4. preserve its attributes;
//! 5. preserve its operands/results;
//! 6. preserve its opaque payload;
//! 7. serialize it;
//! 8. hash it;
//! 9. round-trip it;
//! 10. explicitly reject execution when semantics are unavailable.
//!
//! Unknown vendor data must never silently disappear.
//!
//! # Determinism
//!
//! Vendor IR participates in canonical serialization and hashing.
//!
//! Therefore:
//!
//! - maps use `BTreeMap`;
//! - sets use `BTreeSet`;
//! - collections retain semantic ordering where ordering is meaningful;
//! - identifiers are strongly typed;
//! - payload equality is structural;
//! - formatting is deterministic.
//!
//! # Security
//!
//! Vendor data is treated as untrusted IR data.
//!
//! This module performs structural validation only.
//!
//! It does not establish that a vendor operation is safe or executable.
//! Target/hardware compatibility belongs downstream.
//!
//! Vendor payloads also cannot contain executable Rust objects, function
//! pointers, raw pointers, file handles, sockets, credentials, or backend
//! objects.
//!
//! # Versioning
//!
//! Vendor dialect versions are independent from:
//!
//! - Zamani language version;
//! - Zamani Quantum IR version;
//! - compiler version;
//! - backend version;
//! - firmware version;
//! - calibration version.
//!
//! A future incompatible vendor contract must use a new major version.
//!
//! # Integration contract
//!
//! This module may be consumed by:
//!
//! - `program`;
//! - `operation`;
//! - `region`;
//! - `quantum`;
//! - `resources`;
//! - `validation`;
//! - `serialization`;
//! - `hashing`;
//! - `provenance`;
//! - target compatibility/lowering layers.
//!
//! This module must never depend on:
//!
//! - frontend;
//! - optimization;
//! - routing;
//! - scheduling;
//! - hardware execution;
//! - simulator;
//! - QEC implementation;
//! - backend transport.
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
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::hash::{Hash, Hasher};

use super::super::identity::OperationId;
use super::super::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Constants
// =============================================================================

/// Maximum semantic identifier length is deliberately not encoded as a
/// compile-time constant.
///
/// Resource/security policies belong to the surrounding IR validation layer.
///
/// This constant is only the canonical vendor namespace prefix and therefore
/// is not a resource limit.
pub const VENDOR_NAMESPACE_PREFIX: &str = "vendor";

// =============================================================================
// Vendor error
// =============================================================================

/// Error returned when vendor-dialect data violates structural invariants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VendorError {
    /// A required identifier was empty.
    EmptyIdentifier {
        /// Human-readable identifier category.
        kind: &'static str,
    },

    /// An identifier contains an invalid character.
    InvalidIdentifier {
        /// Human-readable identifier category.
        kind: &'static str,

        /// Invalid value.
        value: String,
    },

    /// A namespace contains an invalid segment.
    InvalidNamespace {
        /// Invalid namespace.
        value: String,
    },

    /// A vendor dialect version is invalid.
    InvalidVersion,

    /// A duplicate operation was inserted.
    DuplicateOperation {
        /// Conflicting operation name.
        name: String,
    },

    /// A duplicate attribute was inserted.
    DuplicateAttribute {
        /// Conflicting attribute name.
        name: String,
    },

    /// A duplicate resource was inserted.
    DuplicateResource {
        /// Conflicting resource name.
        name: String,
    },

    /// A duplicate declaration was inserted.
    DuplicateDeclaration {
        /// Conflicting declaration name.
        name: String,
    },

    /// An invalid vendor operation was supplied.
    InvalidOperation {
        /// Reason.
        reason: String,
    },

    /// An invalid vendor dialect was supplied.
    InvalidDialect {
        /// Reason.
        reason: String,
    },

    /// A required vendor name is missing.
    MissingVendorName,

    /// A vendor dialect version is incompatible.
    IncompatibleVersion {
        /// Producer version.
        producer: VendorVersion,

        /// Consumer version.
        consumer: VendorVersion,
    },
}

impl fmt::Display for VendorError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { kind } => {
                write!(formatter, "{kind} cannot be empty")
            }

            Self::InvalidIdentifier { kind, value } => {
                write!(
                    formatter,
                    "invalid {kind} identifier: {value:?}"
                )
            }

            Self::InvalidNamespace { value } => {
                write!(
                    formatter,
                    "invalid vendor namespace: {value:?}"
                )
            }

            Self::InvalidVersion => {
                formatter.write_str("invalid vendor dialect version")
            }

            Self::DuplicateOperation { name } => {
                write!(
                    formatter,
                    "duplicate vendor operation: {name:?}"
                )
            }

            Self::DuplicateAttribute { name } => {
                write!(
                    formatter,
                    "duplicate vendor attribute: {name:?}"
                )
            }

            Self::DuplicateResource { name } => {
                write!(
                    formatter,
                    "duplicate vendor resource: {name:?}"
                )
            }

            Self::DuplicateDeclaration { name } => {
                write!(
                    formatter,
                    "duplicate vendor declaration: {name:?}"
                )
            }

            Self::InvalidOperation { reason } => {
                write!(
                    formatter,
                    "invalid vendor operation: {reason}"
                )
            }

            Self::InvalidDialect { reason } => {
                write!(
                    formatter,
                    "invalid vendor dialect: {reason}"
                )
            }

            Self::MissingVendorName => {
                formatter.write_str("vendor name is required")
            }

            Self::IncompatibleVersion {
                producer,
                consumer,
            } => {
                write!(
                    formatter,
                    "vendor dialect version {producer} is incompatible with consumer version {consumer}"
                )
            }
        }
    }
}

impl std::error::Error for VendorError {}

// =============================================================================
// Vendor version
// =============================================================================

/// Version of a vendor dialect contract.
///
/// This version belongs to the vendor dialect and is independent from the
/// Zamani Quantum IR version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VendorVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl VendorVersion {
    /// Creates a vendor dialect version.
    #[must_use]
    pub const fn new(
        major: u16,
        minor: u16,
        patch: u16,
    ) -> Self {
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

    /// Returns whether this is exactly the same version.
    #[must_use]
    pub const fn is_exactly(
        self,
        other: Self,
    ) -> bool {
        self == other
    }

    /// Returns whether both versions belong to the same major contract.
    #[must_use]
    pub const fn same_major(
        self,
        other: Self,
    ) -> bool {
        self.major == other.major
    }

    /// Returns whether this version can consume `other` under the
    /// conservative compatibility policy.
    #[must_use]
    pub const fn supports(
        self,
        other: Self,
    ) -> bool {
        other.major == self.major
            && other.minor <= self.minor
            && !(other.minor == self.minor
                && other.patch > self.patch)
    }
}

impl Default for VendorVersion {
    fn default() -> Self {
        Self::new(1, 0, 0)
    }
}

impl fmt::Display for VendorVersion {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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
// Vendor identifier
// =============================================================================

/// Validated vendor identifier.
///
/// The identifier is intentionally a string rather than a closed enum so that
/// adding a new quantum vendor never requires modifying this file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VendorId(String);

impl VendorId {
    /// Creates a validated vendor identifier.
    pub fn new<S>(
        value: S,
    ) -> Result<Self, VendorError>
    where
        S: Into<String>,
    {
        let value = value.into();

        validate_identifier(
            &value,
            "vendor",
        )?;

        Ok(Self(value))
    }

    /// Returns the vendor identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for VendorId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Vendor namespace
// =============================================================================

/// Validated vendor dialect namespace.
///
/// Examples:
///
/// ```text
/// vendor.ibm
/// vendor.ionq
/// vendor.quantinuum
/// vendor.rigetti
/// vendor.example
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VendorNamespace(String);

impl VendorNamespace {
    /// Creates a vendor namespace.
    pub fn new<S>(
        value: S,
    ) -> Result<Self, VendorError>
    where
        S: Into<String>,
    {
        let value = value.into();

        validate_namespace(&value)?;

        Ok(Self(value))
    }

    /// Creates a canonical namespace from a vendor identifier.
    pub fn from_vendor(
        vendor: &VendorId,
    ) -> Result<Self, VendorError> {
        Self::new(format!(
            "{VENDOR_NAMESPACE_PREFIX}.{}",
            vendor.as_str()
        ))
    }

    /// Returns the namespace.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the namespace.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for VendorNamespace {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Vendor operation name
// =============================================================================

/// Validated vendor operation name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VendorOperationName(String);

impl VendorOperationName {
    /// Creates an operation name.
    pub fn new<S>(
        value: S,
    ) -> Result<Self, VendorError>
    where
        S: Into<String>,
    {
        let value = value.into();

        validate_identifier(
            &value,
            "vendor operation",
        )?;

        Ok(Self(value))
    }

    /// Returns the operation name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for VendorOperationName {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Vendor attribute name
// =============================================================================

/// Validated vendor attribute name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VendorAttributeName(String);

impl VendorAttributeName {
    /// Creates an attribute name.
    pub fn new<S>(
        value: S,
    ) -> Result<Self, VendorError>
    where
        S: Into<String>,
    {
        let value = value.into();

        validate_identifier(
            &value,
            "vendor attribute",
        )?;

        Ok(Self(value))
    }

    /// Returns the attribute name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for VendorAttributeName {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Vendor resource name
// =============================================================================

/// Validated vendor resource name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VendorResourceName(String);

impl VendorResourceName {
    /// Creates a resource name.
    pub fn new<S>(
        value: S,
    ) -> Result<Self, VendorError>
    where
        S: Into<String>,
    {
        let value = value.into();

        validate_identifier(
            &value,
            "vendor resource",
        )?;

        Ok(Self(value))
    }

    /// Returns the resource name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for VendorResourceName {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Vendor declaration name
// =============================================================================

/// Validated vendor declaration name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VendorDeclarationName(String);

impl VendorDeclarationName {
    /// Creates a declaration name.
    pub fn new<S>(
        value: S,
    ) -> Result<Self, VendorError>
    where
        S: Into<String>,
    {
        let value = value.into();

        validate_identifier(
            &value,
            "vendor declaration",
        )?;

        Ok(Self(value))
    }

    /// Returns the declaration name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for VendorDeclarationName {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Vendor dialect identifier
// =============================================================================

/// Globally meaningful vendor dialect identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VendorDialectId {
    vendor: VendorId,
    namespace: VendorNamespace,
    version: VendorVersion,
}

impl VendorDialectId {
    /// Creates a vendor dialect identifier.
    pub fn new(
        vendor: VendorId,
        version: VendorVersion,
    ) -> Result<Self, VendorError> {
        let namespace =
            VendorNamespace::from_vendor(&vendor)?;

        Ok(Self {
            vendor,
            namespace,
            version,
        })
    }

    /// Returns the vendor.
    #[must_use]
    pub fn vendor(&self) -> &VendorId {
        &self.vendor
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &VendorNamespace {
        &self.namespace
    }

    /// Returns the version.
    #[must_use]
    pub const fn version(&self) -> VendorVersion {
        self.version
    }

    /// Returns the canonical textual identity.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!(
            "{}@{}",
            self.namespace,
            self.version
        )
    }
}

impl fmt::Display for VendorDialectId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}@{}",
            self.namespace,
            self.version
        )
    }
}

// =============================================================================
// Vendor value
// =============================================================================

/// Vendor-dialect value.
///
/// This deliberately contains only data. It cannot contain executable code,
/// pointers, handles, sockets, credentials, or backend objects.
///
/// `Bytes` and `Opaque` permit lossless preservation of vendor-defined data
/// whose semantic schema is not known to the canonical IR.
#[derive(Debug, Clone, PartialEq)]
pub enum VendorValue {
    /// Null value.
    Null,

    /// Boolean value.
    Bool(bool),

    /// Signed integer.
    Signed(i128),

    /// Unsigned integer.
    Unsigned(u128),

    /// Floating-point value.
    ///
    /// Floating-point values are structural IR values. Canonical
    /// serialization/hashing layers must define their exact encoding.
    Float(f64),

    /// UTF-8 string.
    String(String),

    /// Raw byte sequence.
    Bytes(Vec<u8>),

    /// Ordered sequence.
    Array(Vec<Self>),

    /// Deterministic map.
    Map(BTreeMap<String, Self>),

    /// Canonical logical qubit reference.
    Qubit(QubitId),

    /// Canonical physical qubit reference.
    PhysicalQubit(PhysicalQubitId),

    /// Canonical operation reference.
    Operation(OperationId),

    /// Opaque vendor-defined scalar or structured value.
    Opaque {
        /// Vendor-defined type name.
        type_name: String,

        /// Vendor-defined bytes.
        data: Vec<u8>,
    },
}

impl Eq for VendorValue {}

impl Hash for VendorValue {
    fn hash<H: Hasher>(
        &self,
        state: &mut H,
    ) {
        match self {
            Self::Null => {
                0u8.hash(state);
            }

            Self::Bool(value) => {
                1u8.hash(state);
                value.hash(state);
            }

            Self::Signed(value) => {
                2u8.hash(state);
                value.hash(state);
            }

            Self::Unsigned(value) => {
                3u8.hash(state);
                value.hash(state);
            }

            Self::Float(value) => {
                4u8.hash(state);
                value.to_bits().hash(state);
            }

            Self::String(value) => {
                5u8.hash(state);
                value.hash(state);
            }

            Self::Bytes(value) => {
                6u8.hash(state);
                value.hash(state);
            }

            Self::Array(values) => {
                7u8.hash(state);
                values.hash(state);
            }

            Self::Map(values) => {
                8u8.hash(state);

                for (key, value) in values {
                    key.hash(state);
                    value.hash(state);
                }
            }

            Self::Qubit(value) => {
                9u8.hash(state);
                value.hash(state);
            }

            Self::PhysicalQubit(value) => {
                10u8.hash(state);
                value.hash(state);
            }

            Self::Operation(value) => {
                11u8.hash(state);
                value.hash(state);
            }

            Self::Opaque {
                type_name,
                data,
            } => {
                12u8.hash(state);
                type_name.hash(state);
                data.hash(state);
            }
        }
    }
}

impl VendorValue {
    /// Creates a deterministic vendor map.
    #[must_use]
    pub fn map() -> BTreeMap<String, Self> {
        BTreeMap::new()
    }

    /// Returns whether this value is structurally opaque.
    #[must_use]
    pub const fn is_opaque(&self) -> bool {
        matches!(self, Self::Opaque { .. })
    }

    /// Returns whether this value contains a canonical qubit reference.
    #[must_use]
    pub const fn contains_qubit_reference(&self) -> bool {
        matches!(
            self,
            Self::Qubit(_) | Self::PhysicalQubit(_)
        )
    }
}

// =============================================================================
// Vendor attribute
// =============================================================================

/// Vendor-defined attribute.
///
/// Attribute names are kept separate from values so that the vendor schema
/// can evolve without modifying the canonical IR.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VendorAttribute {
    name: VendorAttributeName,
    value: VendorValue,
}

impl VendorAttribute {
    /// Creates an attribute.
    pub fn new<S>(
        name: S,
        value: VendorValue,
    ) -> Result<Self, VendorError>
    where
        S: Into<String>,
    {
        Ok(Self {
            name: VendorAttributeName::new(name)?,
            value,
        })
    }

    /// Returns the attribute name.
    #[must_use]
    pub fn name(&self) -> &VendorAttributeName {
        &self.name
    }

    /// Returns the attribute value.
    #[must_use]
    pub fn value(&self) -> &VendorValue {
        &self.value
    }
}

// =============================================================================
// Vendor operand
// =============================================================================

/// Operand accepted by a vendor operation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VendorOperand {
    /// Logical qubit operand.
    Qubit(QubitId),

    /// Physical qubit reference.
    PhysicalQubit(PhysicalQubitId),

    /// Operation result/reference.
    Operation(OperationId),

    /// Symbolic/value identifier represented by the vendor dialect.
    Value(String),

    /// Named vendor resource.
    Resource(VendorResourceName),
}

// =============================================================================
// Vendor result
// =============================================================================

/// Result produced by a vendor operation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VendorResult {
    name: Option<String>,
    type_name: String,
}

impl VendorResult {
    /// Creates an unnamed result.
    pub fn unnamed<S>(
        type_name: S,
    ) -> Result<Self, VendorError>
    where
        S: Into<String>,
    {
        Self::new(None, type_name)
    }

    /// Creates a named result.
    pub fn named<N, T>(
        name: N,
        type_name: T,
    ) -> Result<Self, VendorError>
    where
        N: Into<String>,
        T: Into<String>,
    {
        Self::new(Some(name.into()), type_name)
    }

    fn new(
        name: Option<String>,
        type_name: impl Into<String>,
    ) -> Result<Self, VendorError> {
        let type_name = type_name.into();

        validate_identifier(
            &type_name,
            "vendor result type",
        )?;

        if let Some(name) = &name {
            validate_identifier(
                name,
                "vendor result",
            )?;
        }

        Ok(Self {
            name,
            type_name,
        })
    }

    /// Returns the optional result name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the result type name.
    #[must_use]
    pub fn type_name(&self) -> &str {
        &self.type_name
    }
}

// =============================================================================
// Vendor operation
// =============================================================================

/// Vendor-specific semantic operation.
///
/// This is intentionally declarative.
///
/// It does not contain executable backend code.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VendorOperation {
    name: VendorOperationName,
    operands: Vec<VendorOperand>,
    results: Vec<VendorResult>,
    attributes: BTreeMap<VendorAttributeName, VendorValue>,
    resources: BTreeSet<VendorResourceName>,
    opaque_payload: Option<Vec<u8>>,
}

impl VendorOperation {
    /// Creates an operation with no operands, results, attributes or payload.
    pub fn new<S>(
        name: S,
    ) -> Result<Self, VendorError>
    where
        S: Into<String>,
    {
        Ok(Self {
            name: VendorOperationName::new(name)?,
            operands: Vec::new(),
            results: Vec::new(),
            attributes: BTreeMap::new(),
            resources: BTreeSet::new(),
            opaque_payload: None,
        })
    }

    /// Returns the operation name.
    #[must_use]
    pub fn name(&self) -> &VendorOperationName {
        &self.name
    }

    /// Returns the operation operands.
    #[must_use]
    pub fn operands(&self) -> &[VendorOperand] {
        &self.operands
    }

    /// Returns the operation results.
    #[must_use]
    pub fn results(&self) -> &[VendorResult] {
        &self.results
    }

    /// Returns vendor attributes.
    #[must_use]
    pub fn attributes(
        &self,
    ) -> &BTreeMap<VendorAttributeName, VendorValue> {
        &self.attributes
    }

    /// Returns required vendor resources.
    #[must_use]
    pub fn resources(
        &self,
    ) -> &BTreeSet<VendorResourceName> {
        &self.resources
    }

    /// Returns the opaque payload.
    #[must_use]
    pub fn opaque_payload(&self) -> Option<&[u8]> {
        self.opaque_payload.as_deref()
    }

    /// Adds an operand.
    pub fn push_operand(
        &mut self,
        operand: VendorOperand,
    ) {
        self.operands.push(operand);
    }

    /// Adds a result.
    pub fn push_result(
        &mut self,
        result: VendorResult,
    ) {
        self.results.push(result);
    }

    /// Adds or replaces an attribute.
    ///
    /// Replacement is explicit and deterministic. The operation's attribute
    /// map remains structurally unique by name.
    pub fn set_attribute(
        &mut self,
        name: VendorAttributeName,
        value: VendorValue,
    ) {
        self.attributes.insert(name, value);
    }

    /// Adds a required vendor resource.
    pub fn require_resource(
        &mut self,
        resource: VendorResourceName,
    ) {
        self.resources.insert(resource);
    }

    /// Sets an opaque vendor payload.
    pub fn set_opaque_payload(
        &mut self,
        payload: Vec<u8>,
    ) {
        self.opaque_payload = Some(payload);
    }

    /// Validates the operation structurally.
    pub fn validate(&self) -> Result<(), VendorError> {
        if self.name.as_str().is_empty() {
            return Err(VendorError::InvalidOperation {
                reason: "operation name cannot be empty".to_owned(),
            });
        }

        for result in &self.results {
            validate_identifier(
                result.type_name(),
                "vendor result type",
            )?;
        }

        for name in self.attributes.keys() {
            validate_identifier(
                name.as_str(),
                "vendor attribute",
            )?;
        }

        for resource in &self.resources {
            validate_identifier(
                resource.as_str(),
                "vendor resource",
            )?;
        }

        Ok(())
    }
}

// =============================================================================
// Vendor resource
// =============================================================================

/// Vendor-specific resource declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VendorResource {
    name: VendorResourceName,
    attributes: BTreeMap<VendorAttributeName, VendorValue>,
    opaque_payload: Option<Vec<u8>>,
}

impl VendorResource {
    /// Creates a resource declaration.
    pub fn new<S>(
        name: S,
    ) -> Result<Self, VendorError>
    where
        S: Into<String>,
    {
        Ok(Self {
            name: VendorResourceName::new(name)?,
            attributes: BTreeMap::new(),
            opaque_payload: None,
        })
    }

    /// Returns the resource name.
    #[must_use]
    pub fn name(&self) -> &VendorResourceName {
        &self.name
    }

    /// Returns resource attributes.
    #[must_use]
    pub fn attributes(
        &self,
    ) -> &BTreeMap<VendorAttributeName, VendorValue> {
        &self.attributes
    }

    /// Returns opaque resource data.
    #[must_use]
    pub fn opaque_payload(&self) -> Option<&[u8]> {
        self.opaque_payload.as_deref()
    }

    /// Sets a resource attribute.
    pub fn set_attribute(
        &mut self,
        name: VendorAttributeName,
        value: VendorValue,
    ) {
        self.attributes.insert(name, value);
    }

    /// Sets an opaque resource payload.
    pub fn set_opaque_payload(
        &mut self,
        payload: Vec<u8>,
    ) {
        self.opaque_payload = Some(payload);
    }
}

// =============================================================================
// Vendor declaration
// =============================================================================

/// Vendor-defined declaration.
///
/// Declarations allow a vendor dialect to preserve definitions that do not
/// belong in the canonical Zamani semantic vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VendorDeclaration {
    name: VendorDeclarationName,
    type_name: String,
    attributes: BTreeMap<VendorAttributeName, VendorValue>,
    opaque_payload: Option<Vec<u8>>,
}

impl VendorDeclaration {
    /// Creates a declaration.
    pub fn new<N, T>(
        name: N,
        type_name: T,
    ) -> Result<Self, VendorError>
    where
        N: Into<String>,
        T: Into<String>,
    {
        let type_name = type_name.into();

        validate_identifier(
            &type_name,
            "vendor declaration type",
        )?;

        Ok(Self {
            name: VendorDeclarationName::new(name)?,
            type_name,
            attributes: BTreeMap::new(),
            opaque_payload: None,
        })
    }

    /// Returns the declaration name.
    #[must_use]
    pub fn name(&self) -> &VendorDeclarationName {
        &self.name
    }

    /// Returns the declaration type.
    #[must_use]
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Returns attributes.
    #[must_use]
    pub fn attributes(
        &self,
    ) -> &BTreeMap<VendorAttributeName, VendorValue> {
        &self.attributes
    }

    /// Returns opaque payload.
    #[must_use]
    pub fn opaque_payload(&self) -> Option<&[u8]> {
        self.opaque_payload.as_deref()
    }

    /// Sets an attribute.
    pub fn set_attribute(
        &mut self,
        name: VendorAttributeName,
        value: VendorValue,
    ) {
        self.attributes.insert(name, value);
    }

    /// Sets opaque declaration data.
    pub fn set_opaque_payload(
        &mut self,
        payload: Vec<u8>,
    ) {
        self.opaque_payload = Some(payload);
    }
}

// =============================================================================
// Vendor dialect
// =============================================================================

/// Complete declarative vendor dialect.
///
/// A `VendorDialect` is a container for vendor-specific semantic declarations.
/// It is not a backend and contains no executable vendor implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VendorDialect {
    id: VendorDialectId,

    /// Human-readable description.
    description: Option<String>,

    /// Vendor operations keyed by stable semantic name.
    operations: BTreeMap<VendorOperationName, VendorOperation>,

    /// Vendor resources.
    resources: BTreeMap<VendorResourceName, VendorResource>,

    /// Vendor declarations.
    declarations:
        BTreeMap<VendorDeclarationName, VendorDeclaration>,

    /// Dialect-level attributes.
    attributes: BTreeMap<VendorAttributeName, VendorValue>,

    /// Unknown/future vendor-defined payload.
    opaque_payload: Option<Vec<u8>>,
}

impl VendorDialect {
    /// Creates an empty vendor dialect.
    pub fn new(
        vendor: VendorId,
        version: VendorVersion,
    ) -> Result<Self, VendorError> {
        Ok(Self {
            id: VendorDialectId::new(
                vendor,
                version,
            )?,
            description: None,
            operations: BTreeMap::new(),
            resources: BTreeMap::new(),
            declarations: BTreeMap::new(),
            attributes: BTreeMap::new(),
            opaque_payload: None,
        })
    }

    /// Returns the dialect identifier.
    #[must_use]
    pub fn id(&self) -> &VendorDialectId {
        &self.id
    }

    /// Returns the vendor identifier.
    #[must_use]
    pub fn vendor(&self) -> &VendorId {
        self.id.vendor()
    }

    /// Returns the vendor namespace.
    #[must_use]
    pub fn namespace(&self) -> &VendorNamespace {
        self.id.namespace()
    }

    /// Returns the dialect version.
    #[must_use]
    pub const fn version(&self) -> VendorVersion {
        self.id.version()
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Sets the description.
    pub fn set_description<S>(
        &mut self,
        description: S,
    ) where
        S: Into<String>,
    {
        self.description = Some(description.into());
    }

    /// Returns all operations.
    #[must_use]
    pub fn operations(
        &self,
    ) -> &BTreeMap<VendorOperationName, VendorOperation> {
        &self.operations
    }

    /// Returns all resources.
    #[must_use]
    pub fn resources(
        &self,
    ) -> &BTreeMap<VendorResourceName, VendorResource> {
        &self.resources
    }

    /// Returns all declarations.
    #[must_use]
    pub fn declarations(
        &self,
    ) -> &BTreeMap<
        VendorDeclarationName,
        VendorDeclaration,
    > {
        &self.declarations
    }

    /// Returns dialect-level attributes.
    #[must_use]
    pub fn attributes(
        &self,
    ) -> &BTreeMap<VendorAttributeName, VendorValue> {
        &self.attributes
    }

    /// Returns opaque dialect data.
    #[must_use]
    pub fn opaque_payload(&self) -> Option<&[u8]> {
        self.opaque_payload.as_deref()
    }

    /// Adds a vendor operation.
    ///
    /// Duplicate operation names are rejected instead of silently replacing
    /// semantic definitions.
    pub fn add_operation(
        &mut self,
        operation: VendorOperation,
    ) -> Result<(), VendorError> {
        operation.validate()?;

        let name = operation.name.clone();

        if self.operations.contains_key(&name) {
            return Err(
                VendorError::DuplicateOperation {
                    name: name.to_string(),
                },
            );
        }

        self.operations.insert(
            name,
            operation,
        );

        Ok(())
    }

    /// Adds a vendor resource.
    pub fn add_resource(
        &mut self,
        resource: VendorResource,
    ) -> Result<(), VendorError> {
        let name = resource.name.clone();

        if self.resources.contains_key(&name) {
            return Err(
                VendorError::DuplicateResource {
                    name: name.to_string(),
                },
            );
        }

        self.resources.insert(
            name,
            resource,
        );

        Ok(())
    }

    /// Adds a vendor declaration.
    pub fn add_declaration(
        &mut self,
        declaration: VendorDeclaration,
    ) -> Result<(), VendorError> {
        let name = declaration.name.clone();

        if self.declarations.contains_key(&name) {
            return Err(
                VendorError::DuplicateDeclaration {
                    name: name.to_string(),
                },
            );
        }

        self.declarations.insert(
            name,
            declaration,
        );

        Ok(())
    }

    /// Adds a dialect-level attribute.
    ///
    /// Duplicate attributes are rejected.
    pub fn add_attribute(
        &mut self,
        attribute: VendorAttribute,
    ) -> Result<(), VendorError> {
        let name = attribute.name.clone();

        if self.attributes.contains_key(&name) {
            return Err(
                VendorError::DuplicateAttribute {
                    name: name.to_string(),
                },
            );
        }

        self.attributes.insert(
            name,
            attribute.value.clone(),
        );

        Ok(())
    }

    /// Sets an opaque future/vendor payload.
    pub fn set_opaque_payload(
        &mut self,
        payload: Vec<u8>,
    ) {
        self.opaque_payload = Some(payload);
    }

    /// Validates the complete dialect structurally.
    pub fn validate(&self) -> Result<(), VendorError> {
        validate_identifier(
            self.vendor().as_str(),
            "vendor",
        )?;

        for operation in self.operations.values() {
            operation.validate()?;
        }

        for resource in self.resources.values() {
            validate_identifier(
                resource.name().as_str(),
                "vendor resource",
            )?;
        }

        for declaration in self.declarations.values() {
            validate_identifier(
                declaration.name().as_str(),
                "vendor declaration",
            )?;
            validate_identifier(
                declaration.type_name(),
                "vendor declaration type",
            )?;
        }

        for name in self.attributes.keys() {
            validate_identifier(
                name.as_str(),
                "vendor attribute",
            )?;
        }

        Ok(())
    }

    /// Returns whether this dialect can be consumed by a consumer supporting
    /// the supplied version.
    #[must_use]
    pub fn is_compatible_with(
        &self,
        consumer: VendorVersion,
    ) -> bool {
        consumer.supports(self.version())
    }

    /// Validates compatibility and returns a useful error on failure.
    pub fn require_compatible_with(
        &self,
        consumer: VendorVersion,
    ) -> Result<(), VendorError> {
        if self.is_compatible_with(consumer) {
            Ok(())
        } else {
            Err(
                VendorError::IncompatibleVersion {
                    producer: self.version(),
                    consumer,
                },
            )
        }
    }
}

// =============================================================================
// Vendor dialect set
// =============================================================================

/// Deterministic collection of vendor dialects.
///
/// This type is useful at program/module level when multiple vendor dialects
/// coexist in one IR artifact.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct VendorDialectSet {
    dialects: BTreeMap<
        VendorDialectId,
        VendorDialect,
    >,
}

impl VendorDialectSet {
    /// Creates an empty dialect set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the dialects.
    #[must_use]
    pub fn dialects(
        &self,
    ) -> &BTreeMap<VendorDialectId, VendorDialect> {
        &self.dialects
    }

    /// Inserts a dialect.
    ///
    /// A duplicate dialect identity is rejected.
    pub fn insert(
        &mut self,
        dialect: VendorDialect,
    ) -> Result<(), VendorError> {
        dialect.validate()?;

        let id = dialect.id.clone();

        if self.dialects.contains_key(&id) {
            return Err(
                VendorError::InvalidDialect {
                    reason: format!(
                        "dialect {id} already exists"
                    ),
                },
            );
        }

        self.dialects.insert(
            id,
            dialect,
        );

        Ok(())
    }

    /// Returns a dialect by identity.
    #[must_use]
    pub fn get(
        &self,
        id: &VendorDialectId,
    ) -> Option<&VendorDialect> {
        self.dialects.get(id)
    }

    /// Returns the number of registered dialects.
    ///
    /// This is an informational collection size, not an architectural limit.
    #[must_use]
    pub fn len(&self) -> usize {
        self.dialects.len()
    }

    /// Returns whether the set contains no dialects.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dialects.is_empty()
    }

    /// Validates every dialect.
    pub fn validate(&self) -> Result<(), VendorError> {
        for dialect in self.dialects.values() {
            dialect.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Vendor operation reference
// =============================================================================

/// Stable semantic reference to a vendor operation.
///
/// The reference contains no executable implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VendorOperationRef {
    dialect: VendorDialectId,
    operation: VendorOperationName,
}

impl VendorOperationRef {
    /// Creates an operation reference.
    pub fn new<S>(
        dialect: VendorDialectId,
        operation: S,
    ) -> Result<Self, VendorError>
    where
        S: Into<String>,
    {
        Ok(Self {
            dialect,
            operation: VendorOperationName::new(
                operation,
            )?,
        })
    }

    /// Returns the dialect identity.
    #[must_use]
    pub fn dialect(&self) -> &VendorDialectId {
        &self.dialect
    }

    /// Returns the operation name.
    #[must_use]
    pub fn operation(&self) -> &VendorOperationName {
        &self.operation
    }

    /// Returns a deterministic qualified operation name.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!(
            "{}::{}",
            self.dialect,
            self.operation
        )
    }
}

// =============================================================================
// Vendor resource reference
// =============================================================================

/// Stable semantic reference to a vendor resource.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VendorResourceRef {
    dialect: VendorDialectId,
    resource: VendorResourceName,
}

impl VendorResourceRef {
    /// Creates a resource reference.
    pub fn new<S>(
        dialect: VendorDialectId,
        resource: S,
    ) -> Result<Self, VendorError>
    where
        S: Into<String>,
    {
        Ok(Self {
            dialect,
            resource: VendorResourceName::new(
                resource,
            )?,
        })
    }

    /// Returns the dialect.
    #[must_use]
    pub fn dialect(&self) -> &VendorDialectId {
        &self.dialect
    }

    /// Returns the resource name.
    #[must_use]
    pub fn resource(&self) -> &VendorResourceName {
        &self.resource
    }

    /// Returns a deterministic qualified resource name.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!(
            "{}::{}",
            self.dialect,
            self.resource
        )
    }
}

// =============================================================================
// Structural helper functions
// =============================================================================

fn validate_identifier(
    value: &str,
    kind: &'static str,
) -> Result<(), VendorError> {
    if value.is_empty() {
        return Err(
            VendorError::EmptyIdentifier { kind },
        );
    }

    let mut characters =
        value.chars();

    let first =
        match characters.next() {
            Some(value) => value,
            None => {
                return Err(
                    VendorError::EmptyIdentifier {
                        kind,
                    },
                );
            }
        };

    if !(first == '_'
        || first.is_ascii_alphabetic())
    {
        return Err(
            VendorError::InvalidIdentifier {
                kind,
                value: value.to_owned(),
            },
        );
    }

    if !characters.all(|character| {
        character == '_'
            || character == '-'
            || character.is_ascii_alphanumeric()
    }) {
        return Err(
            VendorError::InvalidIdentifier {
                kind,
                value: value.to_owned(),
            },
        );
    }

    Ok(())
}

fn validate_namespace(
    value: &str,
) -> Result<(), VendorError> {
    if value.is_empty() {
        return Err(
            VendorError::InvalidNamespace {
                value: value.to_owned(),
            },
        );
    }

    for segment in value.split('.') {
        if segment.is_empty() {
            return Err(
                VendorError::InvalidNamespace {
                    value: value.to_owned(),
                },
            );
        }

        validate_identifier(
            segment,
            "vendor namespace segment",
        )?;
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_compatibility_is_conservative() {
        let consumer =
            VendorVersion::new(2, 3, 4);

        assert!(
            consumer.supports(
                VendorVersion::new(2, 3, 4)
            )
        );

        assert!(
            consumer.supports(
                VendorVersion::new(2, 2, 9)
            )
        );

        assert!(
            !consumer.supports(
                VendorVersion::new(3, 0, 0)
            )
        );

        assert!(
            !consumer.supports(
                VendorVersion::new(2, 4, 0)
            )
        );

        assert!(
            !consumer.supports(
                VendorVersion::new(2, 3, 5)
            )
        );
    }

    #[test]
    fn vendor_namespace_is_deterministic() {
        let vendor =
            VendorId::new("example")
                .expect("valid vendor");

        let namespace =
            VendorNamespace::from_vendor(
                &vendor
            )
            .expect("valid namespace");

        assert_eq!(
            namespace.as_str(),
            "vendor.example"
        );
    }

    #[test]
    fn vendor_dialect_accepts_arbitrary_vendor_names() {
        let vendor =
            VendorId::new("future_quantum_provider")
                .expect("valid vendor");

        let dialect =
            VendorDialect::new(
                vendor,
                VendorVersion::default(),
            )
            .expect("valid dialect");

        assert_eq!(
            dialect.vendor().as_str(),
            "future_quantum_provider"
        );
    }

    #[test]
    fn duplicate_operations_are_rejected() {
        let vendor =
            VendorId::new("example")
                .expect("valid vendor");

        let mut dialect =
            VendorDialect::new(
                vendor,
                VendorVersion::default(),
            )
            .expect("valid dialect");

        dialect
            .add_operation(
                VendorOperation::new(
                    "native_gate"
                )
                .expect("valid operation"),
            )
            .expect("first operation");

        let result =
            dialect.add_operation(
                VendorOperation::new(
                    "native_gate"
                )
                .expect("valid operation"),
            );

        assert!(matches!(
            result,
            Err(
                VendorError::DuplicateOperation { .. }
            )
        ));
    }

    #[test]
    fn unknown_payload_is_preserved_structurally() {
        let value =
            VendorValue::Opaque {
                type_name:
                    "future.vendor.value"
                        .to_owned(),
                data: vec![
                    1, 2, 3, 4,
                ],
            };

        assert!(value.is_opaque());
    }

    #[test]
    fn logical_qubits_use_canonical_qubit_id() {
        fn accepts_qubit(
            _qubit: QubitId,
        ) {
        }

        let _ = accepts_qubit;
    }

    #[test]
    fn physical_qubits_use_canonical_qubit_id() {
        fn accepts_physical_qubit(
            _qubit: PhysicalQubitId,
        ) {
        }

        let _ = accepts_physical_qubit;
    }

    #[test]
    fn dialect_set_is_deterministic() {
        let first =
            VendorId::new("first")
                .expect("valid vendor");

        let second =
            VendorId::new("second")
                .expect("valid vendor");

        let first_dialect =
            VendorDialect::new(
                first,
                VendorVersion::default(),
            )
            .expect("valid dialect");

        let second_dialect =
            VendorDialect::new(
                second,
                VendorVersion::default(),
            )
            .expect("valid dialect");

        let mut set =
            VendorDialectSet::new();

        set.insert(second_dialect)
            .expect("insert second");

        set.insert(first_dialect)
            .expect("insert first");

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn operation_reference_is_deterministic() {
        let vendor =
            VendorId::new("example")
                .expect("valid vendor");

        let dialect =
            VendorDialect::new(
                vendor,
                VendorVersion::default(),
            )
            .expect("valid dialect");

        let reference =
            VendorOperationRef::new(
                dialect.id().clone(),
                "native_operation",
            )
            .expect("valid reference");

        assert_eq!(
            reference.qualified_name(),
            "vendor.example@1.0.0::native_operation"
        );
    }
}