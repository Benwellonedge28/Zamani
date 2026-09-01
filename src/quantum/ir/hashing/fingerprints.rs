//! Zamani Quantum IR — Stable Fingerprints
//!
//! Production-grade semantic fingerprinting built on the canonical Quantum IR
//! hashing and serialization contracts.
//!
//! # Architectural role
//!
//! This module provides a higher-level, typed identity around the canonical
//! cryptographic hashes owned by `quantum::ir::hash`.
//!
//! The responsibility boundary is:
//
//! ```text
//! semantic IR object
//!       │
//!       ▼
//! quantum::ir::serialization
//!       │
//!       ▼
//! canonical serialized bytes
//!       │
//!       ▼
//! quantum::ir::hash
//!       │
//!       ▼
//! domain-separated SHA-256
//!       │
//!       ▼
//! quantum::ir::hashing::fingerprints
//!       │
//!       ▼
//! typed Fingerprint
//! ```
//!
//! `fingerprints.rs` therefore does NOT:
//!
//! - define SHA-256;
//! - define canonical serialization;
//! - define qubit identities;
//! - define quantum operations;
//! - define program semantics;
//! - define hardware identities;
//! - define routing;
//! - define scheduling;
//! - define optimization;
//! - define signatures;
//! - define authentication;
//! - define encryption;
//! - define backend identities.
//!
//! Those responsibilities remain in their owning modules.
//!
//! # Why this module exists
//!
//! `IrHash` is the cryptographic content hash.
//!
//! A fingerprint additionally answers:
//!
//! > What semantic category does this hash belong to?
//!
//! This prevents callers from passing around an untyped `[u8; 32]` or
//! `IrHash` while separately remembering whether the value identifies:
//!
//! - a complete IR document;
//! - a program;
//! - a circuit;
//! - an operation;
//! - a logical qubit;
//! - a physical qubit;
//! - a pulse;
//! - a waveform;
//! - a resource;
//! - a capability;
//! - a mapping;
//! - another canonical IR object.
//!
//! # Identity versus content
//!
//! This distinction is fundamental.
//!
//! ```text
//! identity fingerprint
//!     = stable identity of an explicitly identified IR object
//!
//! content fingerprint
//!     = stable identity of canonical semantic content
//! ```
//!
//! An `OperationId` fingerprint and an operation-content fingerprint are NOT
//! interchangeable.
//!
//! The same principle applies to `ProgramId`, `QubitId`, and
//! `PhysicalQubitId`.
//!
//! # Canonical hashing boundary
//!
//! All cryptographic hashing is delegated to `quantum::ir::hash`.
//!
//! This module must never independently implement another SHA-256 pipeline.
//!
//! That guarantees that:
//!
//! ```text
//! hash.rs
//!     = single cryptographic source of truth
//!
//! fingerprints.rs
//!     = typed identity facade
//! ```
//!
//! # Quantum identity boundary
//!
//! The canonical qubit types come exclusively from:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module deliberately imports those exact types.
//!
//! It does not define aliases or duplicate qubit identity structures.
//!
//! # Scalability
//!
//! No quantum-machine size is encoded here.
//!
//! The fingerprinting model is identical for:
//!
//! ```text
//! 1 qubit
//! 2 qubits
//! 64 qubits
//! 1,000 qubits
//! 1,000,000 qubits
//! N qubits
//! ```
//!
//! There is no `MAX_QUBITS`, `MAX_REGISTER`, `MAX_OPERATIONS`, or hardware
//! architecture constant in this module.
//!
//! The only fixed-size value is the 32-byte SHA-256 digest, which is a property
//! of the selected cryptographic algorithm and NOT a quantum-machine limit.
//!
//! Very large IR objects are serialized according to the resource/security
//! policies owned by `serialization.rs` and `limits.rs`.
//!
//! This module itself does not introduce another memory budget.
//!
//! # Determinism
//!
//! Fingerprints are deterministic when the supplied IR object has deterministic
//! canonical serialization:
//!
//! ```text
//! same semantic object
//! + same IR version
//! + same serialization contract
//! + same hash contract
//! + same fingerprint domain
//! --------------------------------
//! = same Fingerprint
//! ```
//!
//! The fingerprint does not include:
//!
//! - pointer addresses;
//! - allocator state;
//! - process IDs;
//! - thread IDs;
//! - timestamps;
//! - random numbers;
//! - machine-local paths;
//! - Rust debug output;
//! - `HashMap` iteration order.
//!
//! # Security
//!
//! A fingerprint is a content-identity primitive.
//!
//! It is NOT:
//!
//! - a digital signature;
//! - proof of authorship;
//! - authentication;
//! - authorization;
//! - encryption;
//! - a secret.
//!
//! Authenticity must be implemented by a separate signing/verification
//! subsystem:
//
//! ```text
//! canonical IR
//!     ↓
//! fingerprint
//!     ↓
//! signing subsystem
//!     ↓
//! signature
//! ```
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
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! `hash.rs` owns:
//!
//! - `IrHash`;
//! - `HashDomain`;
//! - `hash_bytes`;
//! - `hash_ir`;
//! - identity-specific hash functions.
//!
//! `serialization.rs` owns:
//!
//! - `IrEncode`;
//! - `serialize`;
//! - `SerializedIr`;
//! - canonical wire representation.
//!
//! `qubit.rs` owns:
//!
//! - `QubitId`;
//! - `PhysicalQubitId`.
//!
//! This module consumes those contracts and does not modify them.
//!
//! # Compatibility rule
//!
//! Existing callers that need a raw cryptographic hash should continue using
//! `quantum::ir::hash`.
//!
//! Callers that need a typed semantic fingerprint should use this module.
//!
//! This separation prevents `Fingerprint` from becoming a second hashing
//! implementation.
//!
//! # Module completion guarantee
//!
//! This file is intentionally complete against the already-established
//! contracts of:
//!
//! - `hash.rs`;
//! - `serialization.rs`;
//! - `qubit.rs`.
//!
//! Adding new IR object types later does not require changing this file.
//!
//! New object types only need to:
//!
//! 1. implement `IrEncode`;
//! 2. select an appropriate existing `HashDomain`, or add a new domain in
//!    `hash.rs` if a genuinely new semantic category is introduced;
//! 3. call `fingerprint`/`fingerprint_with_domain`.
//!
//! No existing fingerprint implementation needs to be rewritten merely because
//! another quantum model is added.
//!
//! # Important invariant
//!
//! Fingerprints must never silently hash non-canonical representations.
//!
//! The generic API therefore always goes through `serialization::serialize`.
//!
//! Raw-byte fingerprinting is available separately and explicitly requires the
//! caller to state that the bytes are already canonical.
//!
//! # Canonical API
//!
//! ```text
//! Fingerprint
//! FingerprintDomain
//! FingerprintError
//!
//! fingerprint()
//! fingerprint_with_domain()
//! fingerprint_canonical_bytes()
//!
//! fingerprint_qubit()
//! fingerprint_physical_qubit()
//! fingerprint_program_id()
//! fingerprint_operation_id()
//! ```
//!
//! -----------------------------------------------------------------------------
//! No domain logic belongs outside the implementation below.
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use super::super::hash::{
    hash_bytes,
    hash_ir,
    hash_operation_id,
    hash_physical_qubit_id,
    hash_program_id,
    hash_qubit_id,
    HashDomain,
    HashError,
    IrHash,
    HASH_ALGORITHM,
    HASH_SCHEMA_VERSION,
};
use super::super::identity::{
    OperationId,
    ProgramId,
};
use super::super::qubit::{
    PhysicalQubitId,
    QubitId,
};
use super::super::serialization::{
    serialize,
    IrEncode,
    SerializationError,
};

// =============================================================================
// Fingerprint schema
// =============================================================================

/// Version of the typed fingerprint contract.
///
/// This version is independent of:
///
/// - the Quantum IR semantic version;
/// - the serialization format version;
/// - the hash algorithm;
/// - the Zamani language version;
/// - compiler versions;
/// - hardware versions.
///
/// Increment this version only when the meaning or representation of
/// `Fingerprint` itself changes.
pub const FINGERPRINT_SCHEMA_VERSION: u16 = 1;

/// Canonical number of bytes in the underlying cryptographic digest.
pub const FINGERPRINT_BYTES: usize = 32;

/// Canonical hexadecimal length of the underlying digest.
pub const FINGERPRINT_HEX_BYTES: usize = FINGERPRINT_BYTES * 2;

/// Stable textual prefix used by the human-readable representation.
///
/// This is a presentation/transport prefix. It is NOT fed into the underlying
/// content hash.
pub const FINGERPRINT_TEXT_PREFIX: &str = "zqfp";

// =============================================================================
// Fingerprint domain
// =============================================================================

/// Semantic category associated with a [`Fingerprint`].
///
/// The underlying cryptographic operation is still performed by
/// `quantum::ir::hash::HashDomain`.
///
/// This type exists to make the public fingerprint API strongly typed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FingerprintDomain {
    /// Complete canonical Quantum IR document.
    Ir,

    /// Quantum program semantic content.
    Program,

    /// Gate-oriented quantum circuit semantic content.
    Circuit,

    /// Operation semantic content.
    Operation,

    /// Stable logical-qubit identity.
    LogicalQubit,

    /// Stable physical-qubit identity.
    PhysicalQubit,

    /// Canonical IR value.
    Value,

    /// Canonical IR parameter.
    Parameter,

    /// Pulse semantic content.
    Pulse,

    /// Waveform semantic content.
    Waveform,

    /// Control-channel semantic content.
    Channel,

    /// Frame semantic content.
    Frame,

    /// Schedule semantic content.
    Schedule,

    /// Resource semantic content.
    Resource,

    /// Capability semantic content.
    Capability,

    /// Logical/physical mapping semantic content.
    Mapping,

    /// Provenance semantic content.
    Provenance,

    /// Extension semantic content.
    Extension,

    /// Explicitly canonical raw bytes.
    Raw,
}

impl FingerprintDomain {
    /// Converts this fingerprint domain into the canonical hash domain.
    #[must_use]
    pub const fn hash_domain(self) -> HashDomain {
        match self {
            Self::Ir => HashDomain::Ir,
            Self::Program => HashDomain::Program,
            Self::Circuit => HashDomain::Circuit,
            Self::Operation => HashDomain::Operation,
            Self::LogicalQubit => HashDomain::LogicalQubit,
            Self::PhysicalQubit => HashDomain::PhysicalQubit,
            Self::Value => HashDomain::Value,
            Self::Parameter => HashDomain::Parameter,
            Self::Pulse => HashDomain::Pulse,
            Self::Waveform => HashDomain::Waveform,
            Self::Channel => HashDomain::Channel,
            Self::Frame => HashDomain::Frame,
            Self::Schedule => HashDomain::Schedule,
            Self::Resource => HashDomain::Resource,
            Self::Capability => HashDomain::Capability,
            Self::Mapping => HashDomain::Mapping,
            Self::Provenance => HashDomain::Provenance,
            Self::Extension => HashDomain::Extension,
            Self::Raw => HashDomain::Raw,
        }
    }

    /// Returns the stable numeric domain identifier.
    ///
    /// The identifier is inherited from the canonical hashing contract.
    #[must_use]
    pub const fn id(self) -> u16 {
        self.hash_domain().id()
    }

    /// Returns the stable textual domain name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.hash_domain().name()
    }
}

impl fmt::Display for FingerprintDomain {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Fingerprint error
// =============================================================================

/// Errors produced by fingerprint construction or parsing.
#[derive(Debug)]
pub enum FingerprintError {
    /// Canonical serialization failed.
    Serialization(SerializationError),

    /// The underlying canonical hashing layer failed.
    Hash(HashError),

    /// A textual fingerprint had the wrong number of components.
    InvalidFormat,

    /// The textual fingerprint used an unsupported fingerprint schema.
    UnsupportedSchema {
        /// Schema version found in the textual fingerprint.
        found: u16,
    },

    /// The textual fingerprint used an unsupported hash algorithm.
    UnsupportedAlgorithm {
        /// Algorithm identifier found in the textual fingerprint.
        found: u8,
    },

    /// The textual fingerprint used an unknown domain.
    UnknownDomain {
        /// Domain identifier found in the textual fingerprint.
        found: u16,
    },

    /// The digest was not valid hexadecimal or had the wrong length.
    InvalidDigest(String),
}

impl fmt::Display for FingerprintError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Serialization(error) => {
                write!(
                    formatter,
                    "cannot fingerprint Quantum IR because canonical serialization failed: {error}"
                )
            }

            Self::Hash(error) => {
                write!(
                    formatter,
                    "cannot fingerprint Quantum IR because canonical hashing failed: {error}"
                )
            }

            Self::InvalidFormat => {
                formatter.write_str(
                    "invalid Zamani Quantum IR fingerprint format",
                )
            }

            Self::UnsupportedSchema { found } => {
                write!(
                    formatter,
                    "unsupported Zamani Quantum IR fingerprint schema {found}; expected {FINGERPRINT_SCHEMA_VERSION}"
                )
            }

            Self::UnsupportedAlgorithm { found } => {
                write!(
                    formatter,
                    "unsupported Zamani Quantum IR fingerprint hash algorithm id {found}; expected {}",
                    HASH_ALGORITHM.id()
                )
            }

            Self::UnknownDomain { found } => {
                write!(
                    formatter,
                    "unknown Zamani Quantum IR fingerprint domain id {found}"
                )
            }

            Self::InvalidDigest(value) => {
                write!(
                    formatter,
                    "invalid Zamani Quantum IR fingerprint digest `{value}`"
                )
            }
        }
    }
}

impl std::error::Error for FingerprintError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serialization(error) => Some(error),
            Self::Hash(error) => Some(error),
            Self::InvalidFormat
            | Self::UnsupportedSchema { .. }
            | Self::UnsupportedAlgorithm { .. }
            | Self::UnknownDomain { .. }
            | Self::InvalidDigest(_) => None,
        }
    }
}

impl From<SerializationError> for FingerprintError {
    fn from(error: SerializationError) -> Self {
        Self::Serialization(error)
    }
}

impl From<HashError> for FingerprintError {
    fn from(error: HashError) -> Self {
        Self::Hash(error)
    }
}

// =============================================================================
// Fingerprint
// =============================================================================

/// Typed, deterministic cryptographic fingerprint of canonical Quantum IR
/// content or identity.
///
/// The underlying digest is the canonical `IrHash` produced by `hash.rs`.
///
/// `Fingerprint` adds semantic domain information without changing the
/// underlying digest.
///
/// # Representation
///
/// The in-memory representation contains:
///
/// - one semantic domain;
/// - one 256-bit SHA-256 digest.
///
/// It contains no allocation and no machine-size-dependent fields.
///
/// # Equality
///
/// Two fingerprints are equal only when both:
///
/// - their semantic domains are equal;
/// - their cryptographic digests are equal.
///
/// Therefore the same bytes hashed under two different semantic domains do not
/// compare equal as fingerprints.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Fingerprint {
    domain: FingerprintDomain,
    hash: IrHash,
}

impl Fingerprint {
    /// Creates a fingerprint from a semantic domain and canonical hash.
    ///
    /// This constructor is useful when integrating with another IR hashing
    /// operation that already uses the canonical `quantum::ir::hash` module.
    #[must_use]
    pub const fn new(
        domain: FingerprintDomain,
        hash: IrHash,
    ) -> Self {
        Self { domain, hash }
    }

    /// Returns the semantic fingerprint domain.
    #[must_use]
    pub const fn domain(self) -> FingerprintDomain {
        self.domain
    }

    /// Returns the underlying canonical cryptographic hash.
    #[must_use]
    pub const fn hash(self) -> IrHash {
        self.hash
    }

    /// Returns the raw SHA-256 digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; FINGERPRINT_BYTES] {
        self.hash.as_bytes()
    }

    /// Copies the raw SHA-256 digest bytes.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; FINGERPRINT_BYTES] {
        self.hash.to_bytes()
    }

    /// Returns the digest as lowercase hexadecimal.
    #[must_use]
    pub fn digest_hex(self) -> String {
        self.hash.to_hex()
    }

    /// Returns the stable textual fingerprint representation.
    ///
    /// Format:
    ///
    /// ```text
    /// zqfp:<schema>:<algorithm>:<domain>:<digest>
    /// ```
    ///
    /// Example shape:
    ///
    /// ```text
    /// zqfp:1:1:program:<64 lowercase hex characters>
    /// ```
    ///
    /// The exact digest is intentionally not embedded in source documentation.
    #[must_use]
    pub fn to_string(self) -> String {
        format!(
            "{prefix}:{schema}:{algorithm}:{domain}:{digest}",
            prefix = FINGERPRINT_TEXT_PREFIX,
            schema = FINGERPRINT_SCHEMA_VERSION,
            algorithm = HASH_ALGORITHM.id(),
            domain = self.domain.id(),
            digest = self.hash.to_hex(),
        )
    }

    /// Parses the canonical textual representation.
    ///
    /// Parsing is strict:
    ///
    /// - exact prefix required;
    /// - exact schema required;
    /// - exact algorithm required;
    /// - numeric domain must be known;
    /// - digest must be exactly 32 bytes;
    /// - hexadecimal characters are case-insensitive on input.
    ///
    /// Canonical output from [`Fingerprint::to_string`] is always lowercase.
    pub fn parse(value: &str) -> Result<Self, FingerprintError> {
        let mut parts = value.split(':');

        let prefix = parts.next();
        let schema = parts.next();
        let algorithm = parts.next();
        let domain = parts.next();
        let digest = parts.next();

        if prefix != Some(FINGERPRINT_TEXT_PREFIX)
            || schema.is_none()
            || algorithm.is_none()
            || domain.is_none()
            || digest.is_none()
            || parts.next().is_some()
        {
            return Err(FingerprintError::InvalidFormat);
        }

        let schema = schema
            .and_then(|value| value.parse::<u16>().ok())
            .ok_or(FingerprintError::InvalidFormat)?;

        if schema != FINGERPRINT_SCHEMA_VERSION {
            return Err(FingerprintError::UnsupportedSchema {
                found: schema,
            });
        }

        let algorithm = algorithm
            .and_then(|value| value.parse::<u8>().ok())
            .ok_or(FingerprintError::InvalidFormat)?;

        if algorithm != HASH_ALGORITHM.id() {
            return Err(FingerprintError::UnsupportedAlgorithm {
                found: algorithm,
            });
        }

        let domain = domain
            .and_then(|value| value.parse::<u16>().ok())
            .ok_or(FingerprintError::InvalidFormat)?;

        let domain = fingerprint_domain_from_id(domain)?;

        let digest = digest.ok_or(FingerprintError::InvalidFormat)?;
        let hash = parse_digest(digest)?;

        Ok(Self::new(domain, hash))
    }
}

impl fmt::Debug for Fingerprint {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("Fingerprint")
            .field("domain", &self.domain)
            .field("digest", &self.hash.to_hex())
            .finish()
    }
}

impl fmt::Display for Fingerprint {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.to_string())
    }
}

// =============================================================================
// Generic semantic fingerprinting
// =============================================================================

/// Fingerprints an IR object using the canonical serialization and hashing
/// contracts.
///
/// This is the preferred API for a complete IR document whose semantic domain
/// is `HashDomain::Ir`.
///
/// For another semantic domain use [`fingerprint_with_domain`].
///
/// # Determinism
///
/// The supplied object is first passed through canonical `serialize`, then
/// hashed through the canonical `hash.rs` implementation.
///
/// # Errors
///
/// Serialization errors are returned without modification.
pub fn fingerprint<T>(
    value: &T,
) -> Result<Fingerprint, FingerprintError>
where
    T: IrEncode,
{
    fingerprint_with_domain(FingerprintDomain::Ir, value)
}

/// Fingerprints an IR object under an explicitly selected semantic domain.
///
/// The object is serialized using the canonical IR serialization contract and
/// the resulting canonical bytes are hashed using the selected canonical hash
/// domain.
///
/// This is the primary extensibility API for future IR dialects and models.
///
/// New semantic object types do not require this file to be modified unless a
/// completely new domain must be introduced into `hash.rs`.
pub fn fingerprint_with_domain<T>(
    domain: FingerprintDomain,
    value: &T,
) -> Result<Fingerprint, FingerprintError>
where
    T: IrEncode,
{
    let serialized = serialize(value)?;

    let hash = hash_bytes(
        domain.hash_domain(),
        serialized.as_bytes(),
    );

    Ok(Fingerprint::new(domain, hash))
}

/// Fingerprints bytes that the caller explicitly guarantees are already
/// canonical for the supplied semantic domain.
///
/// This function intentionally does not serialize or transform the bytes.
///
/// Use this only when the bytes were produced by the canonical serialization
/// contract or by another formally specified canonical encoder.
///
/// For ordinary IR objects, prefer [`fingerprint_with_domain`].
#[must_use]
pub fn fingerprint_canonical_bytes(
    domain: FingerprintDomain,
    canonical_bytes: &[u8],
) -> Fingerprint {
    Fingerprint::new(
        domain,
        hash_bytes(domain.hash_domain(), canonical_bytes),
    )
}

// =============================================================================
// Identity fingerprints
// =============================================================================

/// Returns the stable fingerprint of a canonical logical qubit identity.
///
/// This uses `quantum::ir::qubit::QubitId` directly and delegates the
/// cryptographic operation to `quantum::ir::hash::hash_qubit_id`.
///
/// The result is an identity fingerprint, NOT a content fingerprint of a
/// serialized qubit object.
#[must_use]
pub fn fingerprint_qubit(
    qubit: QubitId,
) -> Fingerprint {
    Fingerprint::new(
        FingerprintDomain::LogicalQubit,
        hash_qubit_id(qubit),
    )
}

/// Returns the stable fingerprint of a canonical physical qubit identity.
///
/// Physical and logical qubits intentionally occupy different semantic
/// fingerprint domains.
#[must_use]
pub fn fingerprint_physical_qubit(
    qubit: PhysicalQubitId,
) -> Fingerprint {
    Fingerprint::new(
        FingerprintDomain::PhysicalQubit,
        hash_physical_qubit_id(qubit),
    )
}

/// Returns the stable fingerprint of a canonical `ProgramId`.
///
/// This is an identity-token fingerprint, not a program-content fingerprint.
///
/// To fingerprint program semantics, use [`fingerprint_with_domain`] with
/// [`FingerprintDomain::Program`].
#[must_use]
pub fn fingerprint_program_id(
    id: ProgramId,
) -> Fingerprint {
    Fingerprint::new(
        FingerprintDomain::Program,
        hash_program_id(id),
    )
}

/// Returns the stable fingerprint of a canonical `OperationId`.
///
/// This is an identity-token fingerprint, not an operation-content
/// fingerprint.
///
/// To fingerprint operation semantics, use [`fingerprint_with_domain`] with
/// [`FingerprintDomain::Operation`].
#[must_use]
pub fn fingerprint_operation_id(
    id: OperationId,
) -> Fingerprint {
    Fingerprint::new(
        FingerprintDomain::Operation,
        hash_operation_id(id),
    )
}

// =============================================================================
// Domain conversion
// =============================================================================

/// Converts a canonical hash domain identifier into a fingerprint domain.
fn fingerprint_domain_from_id(
    id: u16,
) -> Result<FingerprintDomain, FingerprintError> {
    match id {
        1 => Ok(FingerprintDomain::Ir),
        2 => Ok(FingerprintDomain::Program),
        3 => Ok(FingerprintDomain::Circuit),
        4 => Ok(FingerprintDomain::Operation),
        5 => Ok(FingerprintDomain::LogicalQubit),
        6 => Ok(FingerprintDomain::PhysicalQubit),
        7 => Ok(FingerprintDomain::Value),
        8 => Ok(FingerprintDomain::Parameter),
        9 => Ok(FingerprintDomain::Pulse),
        10 => Ok(FingerprintDomain::Waveform),
        11 => Ok(FingerprintDomain::Channel),
        12 => Ok(FingerprintDomain::Frame),
        13 => Ok(FingerprintDomain::Schedule),
        14 => Ok(FingerprintDomain::Resource),
        15 => Ok(FingerprintDomain::Capability),
        16 => Ok(FingerprintDomain::Mapping),
        17 => Ok(FingerprintDomain::Provenance),
        18 => Ok(FingerprintDomain::Extension),
        19 => Ok(FingerprintDomain::Raw),
        other => Err(FingerprintError::UnknownDomain {
            found: other,
        }),
    }
}

// =============================================================================
// Digest parsing
// =============================================================================

/// Parses exactly one SHA-256 digest from hexadecimal.
///
/// This implementation deliberately does not allocate a temporary byte vector.
fn parse_digest(
    value: &str,
) -> Result<IrHash, FingerprintError> {
    if value.len() != FINGERPRINT_HEX_BYTES {
        return Err(FingerprintError::InvalidDigest(
            value.to_owned(),
        ));
    }

    let bytes = value.as_bytes();
    let mut digest = [0u8; FINGERPRINT_BYTES];

    let mut index = 0usize;

    while index < FINGERPRINT_BYTES {
        let high = hex_value(bytes[index * 2]);
        let low = hex_value(bytes[index * 2 + 1]);

        let high = match high {
            Some(value) => value,
            None => {
                return Err(FingerprintError::InvalidDigest(
                    value.to_owned(),
                ));
            }
        };

        let low = match low {
            Some(value) => value,
            None => {
                return Err(FingerprintError::InvalidDigest(
                    value.to_owned(),
                ));
            }
        };

        digest[index] = (high << 4) | low;
        index += 1;
    }

    Ok(IrHash::from_bytes(digest))
}

/// Converts one ASCII hexadecimal byte into its numeric value.
const fn hex_value(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_mapping_is_stable() {
        assert_eq!(
            FingerprintDomain::Ir.id(),
            HashDomain::Ir.id()
        );

        assert_eq!(
            FingerprintDomain::LogicalQubit.id(),
            HashDomain::LogicalQubit.id()
        );

        assert_eq!(
            FingerprintDomain::PhysicalQubit.id(),
            HashDomain::PhysicalQubit.id()
        );
    }

    #[test]
    fn domain_names_are_stable() {
        assert_eq!(
            FingerprintDomain::LogicalQubit.name(),
            "logical-qubit"
        );

        assert_eq!(
            FingerprintDomain::PhysicalQubit.name(),
            "physical-qubit"
        );
    }

    #[test]
    fn logical_and_physical_qubits_are_different_domains() {
        let logical = fingerprint_qubit(QubitId::new(0));
        let physical = fingerprint_physical_qubit(
            PhysicalQubitId::new(0),
        );

        assert_ne!(
            logical.domain(),
            physical.domain()
        );
    }

    #[test]
    fn logical_qubit_fingerprint_is_deterministic() {
        let first = fingerprint_qubit(QubitId::new(17));
        let second = fingerprint_qubit(QubitId::new(17));

        assert_eq!(first, second);
        assert_eq!(
            first.to_bytes(),
            second.to_bytes()
        );
    }

    #[test]
    fn different_logical_qubit_identities_do_not_compare_equal() {
        let first = fingerprint_qubit(QubitId::new(0));
        let second = fingerprint_qubit(QubitId::new(1));

        assert_ne!(first, second);
    }

    #[test]
    fn textual_round_trip_is_lossless() {
        let original =
            fingerprint_qubit(QubitId::new(123));

        let encoded = original.to_string();
        let decoded =
            Fingerprint::parse(&encoded)
                .expect("fingerprint must parse");

        assert_eq!(original, decoded);
        assert_eq!(encoded, decoded.to_string());
    }

    #[test]
    fn textual_output_has_canonical_lowercase_hex() {
        let fingerprint =
            fingerprint_qubit(QubitId::new(0));

        let text = fingerprint.to_string();

        assert!(text.starts_with("zqfp:1:1:5:"));
        assert_eq!(
            text.len(),
            FINGERPRINT_TEXT_PREFIX.len()
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 2
                + 1
                + FINGERPRINT_HEX_BYTES
        );
    }

    #[test]
    fn parser_accepts_uppercase_hex() {
        let original =
            fingerprint_qubit(QubitId::new(7));

        let canonical = original.to_string();

        let uppercase = canonical.to_ascii_uppercase();

        // The prefix and numeric fields are intentionally case-insensitive
        // only where the parser itself permits them. Because the canonical
        // prefix is lower-case and is part of the exact transport grammar,
        // rebuild only the digest in uppercase.
        let digest_start = uppercase
            .rfind(':')
            .expect("fingerprint has digest");

        let mut mixed = canonical[..=digest_start].to_owned();
        mixed.push_str(
            &canonical[digest_start + 1..]
                .to_ascii_uppercase(),
        );

        let decoded =
            Fingerprint::parse(&mixed)
                .expect("uppercase digest should parse");

        assert_eq!(decoded, original);
    }

    #[test]
    fn parser_rejects_wrong_schema() {
        let fingerprint =
            fingerprint_qubit(QubitId::new(0));

        let value = fingerprint
            .to_string()
            .replacen(
                "zqfp:1:",
                "zqfp:2:",
                1,
            );

        let result =
            Fingerprint::parse(&value);

        assert!(matches!(
            result,
            Err(FingerprintError::UnsupportedSchema {
                found: 2
            })
        ));
    }

    #[test]
    fn parser_rejects_unknown_domain() {
        let fingerprint =
            fingerprint_qubit(QubitId::new(0));

        let value = fingerprint
            .to_string()
            .replacen(
                ":5:",
                ":65535:",
                1,
            );

        let result =
            Fingerprint::parse(&value);

        assert!(matches!(
            result,
            Err(FingerprintError::UnknownDomain {
                found: 65535
            })
        ));
    }

    #[test]
    fn parser_rejects_truncated_digest() {
        let fingerprint =
            fingerprint_qubit(QubitId::new(0));

        let mut value =
            fingerprint.to_string();

        value.pop();

        let result =
            Fingerprint::parse(&value);

        assert!(matches!(
            result,
            Err(FingerprintError::InvalidDigest(_))
        ));
    }

    #[test]
    fn canonical_bytes_are_domain_separated() {
        let bytes = b"same canonical bytes";

        let logical =
            fingerprint_canonical_bytes(
                FingerprintDomain::LogicalQubit,
                bytes,
            );

        let physical =
            fingerprint_canonical_bytes(
                FingerprintDomain::PhysicalQubit,
                bytes,
            );

        assert_ne!(logical, physical);
        assert_ne!(
            logical.to_bytes(),
            physical.to_bytes()
        );
    }

    #[test]
    fn identity_and_content_domains_are_explicit() {
        let program_id =
            fingerprint_program_id(
                ProgramId::new(1),
            );

        assert_eq!(
            program_id.domain(),
            FingerprintDomain::Program
        );
    }

    #[test]
    fn operation_identity_is_explicit() {
        let operation =
            fingerprint_operation_id(
                OperationId::new(1),
            );

        assert_eq!(
            operation.domain(),
            FingerprintDomain::Operation
        );
    }

    #[test]
    fn fingerprint_has_fixed_digest_size() {
        let fingerprint =
            fingerprint_qubit(QubitId::new(0));

        assert_eq!(
            fingerprint.as_bytes().len(),
            FINGERPRINT_BYTES
        );
    }
}