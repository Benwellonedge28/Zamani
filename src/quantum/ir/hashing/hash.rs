//! Zamani Quantum IR — Canonical Content Hashing
//!
//! Production-grade, deterministic, streaming-capable content hashing for the
//! canonical Zamani Quantum Intermediate Representation.
//!
//! # Purpose
//!
//! This module owns the cryptographic content-identity boundary of
//! `quantum::ir`.
//!
//! The canonical pipeline is:
//!
//! ```text
//! semantic IR
//!     │
//!     ▼
//! canonical serialization
//!     │
//!     ▼
//! canonical bytes
//!     │
//!     ▼
//! this module
//!     │
//!     ▼
//! SHA-256 content digest
//! ```
//!
//! # Architectural boundary
//!
//! This module DOES:
//!
//! - provide deterministic SHA-256 hashing;
//! - provide explicit domain separation;
//! - hash canonical serialized IR;
//! - support incremental/streaming hashing;
//! - hash canonical logical and physical qubit identities;
//! - provide stable hexadecimal conversion;
//! - provide stable hash metadata;
//! - provide content-hash comparison helpers.
//!
//! This module DOES NOT:
//!
//! - define quantum semantics;
//! - define gate semantics;
//! - define program semantics;
//! - perform optimization;
//! - perform routing;
//! - perform scheduling;
//! - inspect hardware;
//! - select physical qubits;
//! - execute quantum programs;
//! - simulate quantum states;
//! - perform QEC;
//! - sign artifacts;
//! - encrypt artifacts;
//! - authenticate artifact authorship;
//! - define serialization semantics.
//!
//! Those responsibilities belong to other IR or compiler layers.
//!
//! # Canonical hashing invariant
//!
//! For a canonical serialized object:
//!
//! ```text
//! same hash schema
//! + same algorithm
//! + same domain
//! + same canonical bytes
//! --------------------------------
//! = same digest
//! ```
//!
//! A semantic change that changes canonical serialization MUST change the
//! resulting digest, subject to the normal cryptographic properties of
//! SHA-256.
//!
//! # Important distinction: content identity vs object identity
//!
//! `ProgramId`, `OperationId`, and similar identifiers identify IR objects.
//!
//! `IrHash` identifies canonical content.
//!
//! They are deliberately different concepts:
//!
//! ```text
//! ProgramId
//!     = identity token assigned by the IR owner
//!
//! IrHash
//!     = deterministic identity derived from canonical content
//! ```
//!
//! A content hash MUST NOT be used as a replacement for object identifiers
//! unless a higher-level subsystem explicitly defines content-addressed
//! identity semantics.
//!
//! # Quantum identity boundary
//!
//! Logical and physical qubit identities are owned exclusively by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module imports and hashes those exact types. It never defines duplicate
//! qubit identity types.
//!
//! # Scalability
//!
//! No quantum-machine size is encoded here.
//!
//! There is deliberately no:
//!
//! - maximum qubit count;
//! - maximum register size;
//! - fixed gate count;
//! - fixed topology;
//! - fixed hardware architecture.
//!
//! A one-qubit program and a very large finite program use the same hashing
//! contract.
//!
//! The practical limit is determined by the size of the canonical artifact and
//! by resources available to the process.
//!
//! `HashBuilder` and `hash_reader` permit large artifacts to be hashed without
//! requiring the entire artifact to exist in one memory allocation.
//!
//! The architectural phrase "infinity" therefore means:
//!
//! > no fixed quantum-computation size is encoded into this module; every
//! > finite artifact representable by the surrounding IR, host environment and
//! > selected resource policies follows the same hashing contract.
//!
//! # Determinism
//!
//! This module never hashes:
//!
//! - pointer addresses;
//! - memory addresses;
//! - process IDs;
//! - thread IDs;
//! - allocator state;
//! - wall-clock time;
//! - random values;
//! - Rust debug formatting;
//! - platform-dependent `usize` encodings.
//!
//! All integer framing emitted by this module uses explicit fixed-width
//! little-endian encodings.
//!
//! # Security
//!
//! SHA-256 provides a cryptographic digest suitable for content identity.
//!
//! A digest does NOT provide:
//!
//! - authenticity;
//! - authorization;
//! - authorship;
//! - confidentiality;
//! - proof that a particular machine executed an artifact.
//!
//! Authentication must be implemented by a separate signing/verification
//! subsystem:
//!
//! ```text
//! canonical IR
//!     │
//!     ▼
//! SHA-256
//!     │
//!     ▼
//! signature subsystem
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
//! The module explicitly forbids unsafe code.
//!
//! # Integration contract
//!
//! `quantum::ir::serialization` owns canonical serialization.
//!
//! Any semantic IR type implementing `serialization::IrEncode` can be hashed
//! with [`hash_ir`].
//!
//! The serialization layer remains independent of this module:
//!
//! ```text
//! serialization ───────► hash
//! ```
//!
//! and never:
//!
//! ```text
//! hash ───────► serialization semantics
//! ```
//!
//! This prevents a circular semantic dependency.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]

use std::fmt;
use std::io::{self, Read};

use sha2::{Digest, Sha256};

use super::super::identity::{IrVersion, OperationId, ProgramId};
use super::super::qubit::{PhysicalQubitId, QubitId};
use super::super::serialization::{
    serialize,
    serialize_with_version,
    IrEncode,
    SerializationError,
    SerializedIr,
};

// =============================================================================
// Contract constants
// =============================================================================

/// Version of the canonical hashing contract.
///
/// This version is independent of the semantic IR version and serialization
/// format version.
///
/// A breaking change to the hashing domain framing or digest interpretation
/// MUST increment this value.
pub const HASH_SCHEMA_VERSION: u16 = 1;

/// SHA-256 digest size in bytes.
pub const HASH_BYTES: usize = 32;

/// SHA-256 digest size in hexadecimal characters.
pub const HASH_HEX_BYTES: usize = HASH_BYTES * 2;

/// Stable domain-separation prefix.
///
/// This value is part of the persistent hashing contract and MUST NOT be
/// changed without a hashing-schema version change.
pub const HASH_DOMAIN_PREFIX: &[u8] = b"ZAMANI-QIR-HASH";

/// The canonical hashing algorithm.
pub const HASH_ALGORITHM: HashAlgorithm = HashAlgorithm::Sha256;

// =============================================================================
// Hash algorithm
// =============================================================================

/// Cryptographic algorithm used by the canonical Quantum IR hash contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HashAlgorithm {
    /// SHA-256.
    Sha256,
}

impl HashAlgorithm {
    /// Returns the stable wire-level algorithm identifier.
    #[must_use]
    pub const fn id(self) -> u8 {
        match self {
            Self::Sha256 => 1,
        }
    }

    /// Returns the stable human-readable algorithm name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Sha256 => "sha256",
        }
    }
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Hash domain
// =============================================================================

/// Semantic domain used for domain-separated hashing.
///
/// Domain IDs are persistent protocol values. Once published, an existing
/// numeric identifier MUST NOT be reused for a different semantic domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum HashDomain {
    /// Complete canonical IR document.
    Ir = 1,

    /// Complete quantum program.
    Program = 2,

    /// Quantum circuit.
    Circuit = 3,

    /// Individual IR operation.
    Operation = 4,

    /// Logical qubit identity.
    LogicalQubit = 5,

    /// Physical qubit identity.
    PhysicalQubit = 6,

    /// Generic canonical value.
    Value = 7,

    /// Parameter/expression content.
    Parameter = 8,

    /// Pulse content.
    Pulse = 9,

    /// Waveform content.
    Waveform = 10,

    /// Abstract channel content.
    Channel = 11,

    /// Abstract frame content.
    Frame = 12,

    /// Schedule content.
    Schedule = 13,

    /// Resource requirement content.
    Resource = 14,

    /// Capability requirement content.
    Capability = 15,

    /// Mapping content.
    Mapping = 16,

    /// Provenance content.
    Provenance = 17,

    /// Extension content.
    Extension = 18,

    /// Raw canonical bytes.
    Raw = 19,
}

impl HashDomain {
    /// Returns the stable persistent numeric domain identifier.
    #[must_use]
    pub const fn id(self) -> u16 {
        self as u16
    }

    /// Returns the stable domain name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Ir => "ir",
            Self::Program => "program",
            Self::Circuit => "circuit",
            Self::Operation => "operation",
            Self::LogicalQubit => "logical-qubit",
            Self::PhysicalQubit => "physical-qubit",
            Self::Value => "value",
            Self::Parameter => "parameter",
            Self::Pulse => "pulse",
            Self::Waveform => "waveform",
            Self::Channel => "channel",
            Self::Frame => "frame",
            Self::Schedule => "schedule",
            Self::Resource => "resource",
            Self::Capability => "capability",
            Self::Mapping => "mapping",
            Self::Provenance => "provenance",
            Self::Extension => "extension",
            Self::Raw => "raw",
        }
    }
}

impl fmt::Display for HashDomain {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the canonical hashing subsystem.
#[derive(Debug)]
pub enum HashError {
    /// Canonical IR serialization failed.
    Serialization(SerializationError),

    /// Reading a streaming input failed.
    Io(io::Error),

    /// A hexadecimal digest has the wrong number of characters.
    InvalidHexLength {
        /// Number of supplied characters.
        length: usize,
    },

    /// A hexadecimal digest contains an invalid character.
    InvalidHexCharacter {
        /// Character offset in the hexadecimal input.
        index: usize,

        /// Invalid character.
        character: char,
    },
}

impl fmt::Display for HashError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serialization(error) => {
                write!(
                    formatter,
                    "cannot hash canonical Quantum IR: {error}"
                )
            }

            Self::Io(error) => {
                write!(
                    formatter,
                    "cannot hash Quantum IR stream: {error}"
                )
            }

            Self::InvalidHexLength { length } => {
                write!(
                    formatter,
                    "invalid Quantum IR hash length {length}; \
                     expected {HASH_HEX_BYTES} hexadecimal characters"
                )
            }

            Self::InvalidHexCharacter { index, character } => {
                write!(
                    formatter,
                    "invalid Quantum IR hash character `{character}` \
                     at hexadecimal position {index}"
                )
            }
        }
    }
}

impl std::error::Error for HashError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serialization(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::InvalidHexLength { .. } => None,
            Self::InvalidHexCharacter { .. } => None,
        }
    }
}

impl From<SerializationError> for HashError {
    fn from(error: SerializationError) -> Self {
        Self::Serialization(error)
    }
}

impl From<io::Error> for HashError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

// =============================================================================
// IrHash
// =============================================================================

/// Fixed-size SHA-256 content identity.
///
/// The digest is exactly 32 bytes and therefore has no allocation and no
/// architecture-dependent representation.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IrHash([u8; HASH_BYTES]);

impl IrHash {
    /// Creates a hash from an exact SHA-256 digest.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; HASH_BYTES]) -> Self {
        Self(bytes)
    }

    /// Returns the raw digest.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; HASH_BYTES] {
        &self.0
    }

    /// Returns the digest as an owned fixed-size array.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; HASH_BYTES] {
        self.0
    }

    /// Returns the digest as lowercase hexadecimal.
    #[must_use]
    pub fn to_hex(self) -> String {
        let mut result = String::with_capacity(HASH_HEX_BYTES);

        for byte in self.0 {
            result.push(hex_digit(byte >> 4));
            result.push(hex_digit(byte & 0x0f));
        }

        result
    }

    /// Parses a 64-character hexadecimal SHA-256 digest.
    ///
    /// Both lowercase and uppercase hexadecimal are accepted.
    pub fn from_hex(input: &str) -> Result<Self, HashError> {
        if input.len() != HASH_HEX_BYTES {
            return Err(HashError::InvalidHexLength {
                length: input.len(),
            });
        }

        let bytes = input.as_bytes();
        let mut digest = [0u8; HASH_BYTES];

        let mut index = 0usize;

        while index < HASH_BYTES {
            let high = hex_value(bytes[index * 2], index * 2)?;
            let low = hex_value(
                bytes[index * 2 + 1],
                index * 2 + 1,
            )?;

            digest[index] = (high << 4) | low;
            index += 1;
        }

        Ok(Self(digest))
    }

    /// Returns the algorithm represented by this hash.
    #[must_use]
    pub const fn algorithm(self) -> HashAlgorithm {
        HASH_ALGORITHM
    }

    /// Returns the digest length in bytes.
    #[must_use]
    pub const fn len(self) -> usize {
        HASH_BYTES
    }

    /// Returns whether every digest byte is zero.
    ///
    /// A zero digest is not intrinsically invalid. This method is only a
    /// diagnostic helper.
    #[must_use]
    pub fn is_zero(self) -> bool {
        let mut index = 0usize;

        while index < HASH_BYTES {
            if self.0[index] != 0 {
                return false;
            }

            index += 1;
        }

        true
    }
}

impl Default for IrHash {
    fn default() -> Self {
        Self([0u8; HASH_BYTES])
    }
}

impl fmt::Debug for IrHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("IrHash")
            .field(&self.to_hex())
            .finish()
    }
}

impl fmt::Display for IrHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_hex())
    }
}

// =============================================================================
// Semantic aliases
// =============================================================================

/// Content hash of a complete quantum program.
pub type ProgramHash = IrHash;

/// Content hash of a quantum circuit.
pub type CircuitHash = IrHash;

/// Content hash of an individual IR operation.
pub type OperationHash = IrHash;

/// Content hash of a logical qubit identity.
pub type QubitHash = IrHash;

/// Content hash of a physical qubit identity.
pub type PhysicalQubitHash = IrHash;

// =============================================================================
// Incremental builder
// =============================================================================

/// Incremental domain-separated SHA-256 builder.
///
/// The builder never accumulates all supplied bytes.
///
/// This is suitable for large canonical artifacts and streaming compilation
/// infrastructure.
///
/// # Finalization
///
/// `finalize` consumes the builder. Consequently, there is no mutable
/// post-finalization state and no silently ignored updates.
pub struct HashBuilder {
    hasher: Sha256,
}

impl HashBuilder {
    /// Creates a new domain-separated hash builder.
    #[must_use]
    pub fn new(domain: HashDomain) -> Self {
        let mut hasher = Sha256::new();

        write_domain_header(
            &mut hasher,
            domain,
        );

        Self { hasher }
    }

    /// Appends canonical bytes.
    pub fn update(&mut self, bytes: &[u8]) {
        self.hasher.update(bytes);
    }

    /// Appends one byte.
    pub fn update_byte(&mut self, byte: u8) {
        self.hasher.update([byte]);
    }

    /// Appends a canonical little-endian `u16`.
    pub fn update_u16(&mut self, value: u16) {
        self.hasher.update(value.to_le_bytes());
    }

    /// Appends a canonical little-endian `u32`.
    pub fn update_u32(&mut self, value: u32) {
        self.hasher.update(value.to_le_bytes());
    }

    /// Appends a canonical little-endian `u64`.
    pub fn update_u64(&mut self, value: u64) {
        self.hasher.update(value.to_le_bytes());
    }

    /// Appends a canonical little-endian `u128`.
    pub fn update_u128(&mut self, value: u128) {
        self.hasher.update(value.to_le_bytes());
    }

    /// Appends a canonical little-endian signed `i64`.
    pub fn update_i64(&mut self, value: i64) {
        self.hasher.update(value.to_le_bytes());
    }

    /// Appends a canonical little-endian signed `i128`.
    pub fn update_i128(&mut self, value: i128) {
        self.hasher.update(value.to_le_bytes());
    }

    /// Appends an explicitly encoded boolean.
    pub fn update_bool(&mut self, value: bool) {
        self.update_byte(u8::from(value));
    }

    /// Appends a canonical length-prefixed byte sequence.
    ///
    /// The length is represented by `u64`, never by the host `usize`.
    ///
    /// A conversion failure is returned rather than silently truncating an
    /// oversized platform collection.
    pub fn update_length_prefixed_bytes(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), HashError> {
        let length = u64::try_from(bytes.len())
            .map_err(|_| HashError::Serialization(
                SerializationError::LengthOverflow {
                    context: "hash length prefix",
                    value: u64::MAX,
                },
            ))?;

        self.update_u64(length);
        self.update(bytes);

        Ok(())
    }

    /// Appends a canonical UTF-8 string.
    pub fn update_str(
        &mut self,
        value: &str,
    ) -> Result<(), HashError> {
        self.update_length_prefixed_bytes(value.as_bytes())
    }

    /// Finalizes the digest.
    #[must_use]
    pub fn finalize(self) -> IrHash {
        let digest = self.hasher.finalize();

        let mut bytes = [0u8; HASH_BYTES];
        bytes.copy_from_slice(&digest);

        IrHash(bytes)
    }
}

// =============================================================================
// Canonical IR hashing
// =============================================================================

/// Hashes an object using its canonical IR serialization.
pub fn hash_ir<T>(
    value: &T,
) -> Result<IrHash, HashError>
where
    T: IrEncode,
{
    let serialized = serialize(value)?;
    hash_serialized(&serialized)
}

/// Hashes an object using an explicitly selected semantic IR version.
pub fn hash_ir_with_version<T>(
    value: &T,
    version: IrVersion,
) -> Result<IrHash, HashError>
where
    T: IrEncode,
{
    let serialized = serialize_with_version(
        value,
        version,
    )?;

    hash_serialized(&serialized)
}

/// Hashes an already serialized canonical IR document.
///
/// The complete serialization framing is included in the digest.
#[must_use]
pub fn hash_serialized(
    serialized: &SerializedIr,
) -> Result<IrHash, HashError> {
    Ok(hash_bytes(
        HashDomain::Ir,
        serialized.as_bytes(),
    ))
}

/// Hashes arbitrary canonical bytes under an explicit semantic domain.
///
/// The caller is responsible for ensuring that the bytes are canonical for
/// the selected domain.
#[must_use]
pub fn hash_bytes(
    domain: HashDomain,
    bytes: &[u8],
) -> IrHash {
    let mut builder = HashBuilder::new(domain);
    builder.update(bytes);
    builder.finalize()
}

/// Hashes a stream incrementally.
///
/// The stream is consumed in bounded chunks so that the complete input does
/// not need to be resident in memory.
pub fn hash_reader<R>(
    domain: HashDomain,
    reader: &mut R,
) -> Result<IrHash, HashError>
where
    R: Read,
{
    const BUFFER_SIZE: usize = 64 * 1024;

    let mut builder = HashBuilder::new(domain);
    let mut buffer = [0u8; BUFFER_SIZE];

    loop {
        let count = reader.read(&mut buffer)?;

        if count == 0 {
            break;
        }

        builder.update(&buffer[..count]);
    }

    Ok(builder.finalize())
}

// =============================================================================
// Canonical identity hashing
// =============================================================================

/// Hashes a logical qubit identity.
///
/// The canonical identity type is
/// `quantum::ir::qubit::QubitId`.
///
/// The identity value is framed as a fixed-width `u64`.
#[must_use]
pub fn hash_qubit_id(
    qubit: QubitId,
) -> QubitHash {
    let mut builder =
        HashBuilder::new(HashDomain::LogicalQubit);

    builder.update_u64(
        qubit.index(),
    );

    builder.finalize()
}

/// Hashes a physical qubit identity.
///
/// The canonical identity type is
/// `quantum::ir::qubit::PhysicalQubitId`.
#[must_use]
pub fn hash_physical_qubit_id(
    qubit: PhysicalQubitId,
) -> PhysicalQubitHash {
    let mut builder =
        HashBuilder::new(HashDomain::PhysicalQubit);

    builder.update_u64(
        qubit.index(),
    );

    builder.finalize()
}

/// Hashes a `ProgramId` identity token.
///
/// This hashes the identity token itself, not the program's semantic content.
///
/// Use [`hash_ir`] for program content.
#[must_use]
pub fn hash_program_id(
    id: ProgramId,
) -> IrHash {
    hash_u64_identity(
        HashDomain::Program,
        id.value(),
    )
}

/// Hashes an `OperationId` identity token.
///
/// This hashes the identity token itself, not operation contents.
#[must_use]
pub fn hash_operation_id(
    id: OperationId,
) -> IrHash {
    hash_u64_identity(
        HashDomain::Operation,
        id.value(),
    )
}

/// Hashes an explicit stable `u64` identity under a domain.
#[must_use]
pub fn hash_u64_identity(
    domain: HashDomain,
    value: u64,
) -> IrHash {
    let mut builder = HashBuilder::new(domain);
    builder.update_u64(value);
    builder.finalize()
}

// =============================================================================
// Hash comparison
// =============================================================================

/// Compares two canonical hashes.
#[must_use]
pub const fn hashes_equal(
    left: &IrHash,
    right: &IrHash,
) -> bool {
    left.0 == right.0
}

/// Returns whether two canonical hashes differ.
#[must_use]
pub const fn hashes_differ(
    left: &IrHash,
    right: &IrHash,
) -> bool {
    left.0 != right.0
}

// =============================================================================
// Hash metadata
// =============================================================================

/// Metadata describing the active hashing contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HashMetadata {
    /// Hash schema version.
    pub schema_version: u16,

    /// Cryptographic algorithm.
    pub algorithm: HashAlgorithm,

    /// Digest length in bytes.
    pub digest_bytes: usize,

    /// Semantic domain.
    pub domain: HashDomain,
}

impl HashMetadata {
    /// Creates metadata for a semantic hash domain.
    #[must_use]
    pub const fn new(domain: HashDomain) -> Self {
        Self {
            schema_version: HASH_SCHEMA_VERSION,
            algorithm: HASH_ALGORITHM,
            digest_bytes: HASH_BYTES,
            domain,
        }
    }
}

// =============================================================================
// Domain framing
// =============================================================================

/// Writes the stable domain-separation header.
///
/// The exact framing is:
///
/// ```text
/// prefix_length  u16 LE
/// prefix         bytes
/// schema         u16 LE
/// algorithm      u8
/// domain         u16 LE
/// ```
///
/// The framing is explicitly width-defined and therefore independent of host
/// architecture.
fn write_domain_header(
    hasher: &mut Sha256,
    domain: HashDomain,
) {
    let prefix_length =
        u16::try_from(HASH_DOMAIN_PREFIX.len())
            .expect("the fixed hash-domain prefix must fit in u16");

    hasher.update(prefix_length.to_le_bytes());
    hasher.update(HASH_DOMAIN_PREFIX);
    hasher.update(
        HASH_SCHEMA_VERSION.to_le_bytes()
    );
    hasher.update([HASH_ALGORITHM.id()]);
    hasher.update(domain.id().to_le_bytes());
}

// =============================================================================
// Hexadecimal helpers
// =============================================================================

fn hex_digit(value: u8) -> char {
    match value & 0x0f {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        _ => 'f',
    }
}

fn hex_value(
    value: u8,
    index: usize,
) -> Result<u8, HashError> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        b'A'..=b'F' => Ok(value - b'A' + 10),

        _ => Err(
            HashError::InvalidHexCharacter {
                index,
                character: value as char,
            }
        ),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn digest_has_exact_sha256_size() {
        let hash = hash_bytes(
            HashDomain::Raw,
            b"zamani",
        );

        assert_eq!(
            hash.len(),
            HASH_BYTES
        );

        assert_eq!(
            hash.as_bytes().len(),
            HASH_BYTES
        );
    }

    #[test]
    fn same_bytes_and_domain_are_deterministic() {
        let first = hash_bytes(
            HashDomain::Program,
            b"zamani",
        );

        let second = hash_bytes(
            HashDomain::Program,
            b"zamani",
        );

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn different_bytes_change_digest() {
        let first = hash_bytes(
            HashDomain::Program,
            b"zamani-a",
        );

        let second = hash_bytes(
            HashDomain::Program,
            b"zamani-b",
        );

        assert_ne!(
            first,
            second
        );
    }

    #[test]
    fn different_domains_are_separated() {
        let program = hash_bytes(
            HashDomain::Program,
            b"same",
        );

        let circuit = hash_bytes(
            HashDomain::Circuit,
            b"same",
        );

        assert_ne!(
            program,
            circuit
        );
    }

    #[test]
    fn logical_and_physical_qubits_are_separated() {
        let logical =
            hash_qubit_id(
                QubitId::new(17)
            );

        let physical =
            hash_physical_qubit_id(
                PhysicalQubitId::new(17)
            );

        assert_ne!(
            logical,
            physical
        );
    }

    #[test]
    fn different_qubit_ids_change_digest() {
        let first =
            hash_qubit_id(
                QubitId::new(0)
            );

        let second =
            hash_qubit_id(
                QubitId::new(1)
            );

        assert_ne!(
            first,
            second
        );
    }

    #[test]
    fn incremental_hash_equals_single_update() {
        let complete =
            hash_bytes(
                HashDomain::Raw,
                b"abcdef",
            );

        let mut builder =
            HashBuilder::new(
                HashDomain::Raw
            );

        builder.update(b"ab");
        builder.update(b"cd");
        builder.update(b"ef");

        assert_eq!(
            complete,
            builder.finalize()
        );
    }

    #[test]
    fn reader_hash_equals_byte_hash() {
        let input =
            b"streamed quantum ir";

        let direct =
            hash_bytes(
                HashDomain::Raw,
                input,
            );

        let mut reader =
            Cursor::new(input.as_slice());

        let streamed =
            hash_reader(
                HashDomain::Raw,
                &mut reader,
            )
            .expect("stream hashing must succeed");

        assert_eq!(
            direct,
            streamed
        );
    }

    #[test]
    fn integer_encoding_is_deterministic() {
        let mut first =
            HashBuilder::new(
                HashDomain::Raw
            );

        first.update_u16(0x1234);
        first.update_u32(0x12345678);
        first.update_u64(0x123456789abcdef0);
        first.update_u128(
            0x123456789abcdef00123456789abcdef,
        );
        first.update_i64(-123456);
        first.update_i128(-987654321);

        let first =
            first.finalize();

        let mut second =
            HashBuilder::new(
                HashDomain::Raw
            );

        second.update_u16(0x1234);
        second.update_u32(0x12345678);
        second.update_u64(0x123456789abcdef0);
        second.update_u128(
            0x123456789abcdef00123456789abcdef,
        );
        second.update_i64(-123456);
        second.update_i128(-987654321);

        let second =
            second.finalize();

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn length_prefix_prevents_boundary_ambiguity() {
        let mut first =
            HashBuilder::new(
                HashDomain::Raw
            );

        first
            .update_str("ab")
            .expect("string hashing must succeed");

        first
            .update_str("c")
            .expect("string hashing must succeed");

        let first =
            first.finalize();

        let mut second =
            HashBuilder::new(
                HashDomain::Raw
            );

        second
            .update_str("a")
            .expect("string hashing must succeed");

        second
            .update_str("bc")
            .expect("string hashing must succeed");

        let second =
            second.finalize();

        assert_ne!(
            first,
            second
        );
    }

    #[test]
    fn hexadecimal_round_trip_is_exact() {
        let original =
            hash_bytes(
                HashDomain::Program,
                b"round-trip",
            );

        let encoded =
            original.to_hex();

        assert_eq!(
            encoded.len(),
            HASH_HEX_BYTES
        );

        let decoded =
            IrHash::from_hex(
                &encoded
            )
            .expect("valid hexadecimal must decode");

        assert_eq!(
            original,
            decoded
        );
    }

    #[test]
    fn uppercase_hexadecimal_is_accepted() {
        let original =
            hash_bytes(
                HashDomain::Raw,
                b"uppercase",
            );

        let encoded =
            original
                .to_hex()
                .to_uppercase();

        let decoded =
            IrHash::from_hex(
                &encoded
            )
            .expect("uppercase hexadecimal must decode");

        assert_eq!(
            original,
            decoded
        );
    }

    #[test]
    fn invalid_hex_length_is_rejected() {
        let result =
            IrHash::from_hex("00");

        assert!(matches!(
            result,
            Err(
                HashError::InvalidHexLength {
                    length: 2
                }
            )
        ));
    }

    #[test]
    fn invalid_hex_character_is_rejected() {
        let input =
            "000000000000000000000000000000000000000000000000000000000000000g";

        let result =
            IrHash::from_hex(input);

        assert!(matches!(
            result,
            Err(
                HashError::InvalidHexCharacter {
                    ..
                }
            )
        ));
    }

    #[test]
    fn all_domains_have_unique_ids() {
        let domains = [
            HashDomain::Ir,
            HashDomain::Program,
            HashDomain::Circuit,
            HashDomain::Operation,
            HashDomain::LogicalQubit,
            HashDomain::PhysicalQubit,
            HashDomain::Value,
            HashDomain::Parameter,
            HashDomain::Pulse,
            HashDomain::Waveform,
            HashDomain::Channel,
            HashDomain::Frame,
            HashDomain::Schedule,
            HashDomain::Resource,
            HashDomain::Capability,
            HashDomain::Mapping,
            HashDomain::Provenance,
            HashDomain::Extension,
            HashDomain::Raw,
        ];

        let mut index = 0usize;

        while index < domains.len() {
            let mut other = index + 1;

            while other < domains.len() {
                assert_ne!(
                    domains[index].id(),
                    domains[other].id()
                );

                other += 1;
            }

            index += 1;
        }
    }

    #[test]
    fn algorithm_identifier_is_stable() {
        assert_eq!(
            HashAlgorithm::Sha256.id(),
            1
        );

        assert_eq!(
            HashAlgorithm::Sha256.name(),
            "sha256"
        );
    }

    #[test]
    fn domain_names_are_stable() {
        assert_eq!(
            HashDomain::Program.name(),
            "program"
        );

        assert_eq!(
            HashDomain::LogicalQubit.name(),
            "logical-qubit"
        );

        assert_eq!(
            HashDomain::PhysicalQubit.name(),
            "physical-qubit"
        );
    }

    #[test]
    fn display_is_lowercase_hex() {
        let hash =
            hash_bytes(
                HashDomain::Raw,
                b"display",
            );

        let display =
            hash.to_string();

        assert_eq!(
            display.len(),
            HASH_HEX_BYTES
        );

        assert!(
            display
                .bytes()
                .all(|byte| {
                    matches!(
                        byte,
                        b'0'..=b'9'
                            | b'a'..=b'f'
                    )
                })
        );
    }

    #[test]
    fn zero_detection_is_diagnostic_only() {
        let hash =
            hash_bytes(
                HashDomain::Raw,
                b"not-zero",
            );

        assert!(
            !hash.is_zero()
        );
    }

    #[test]
    fn program_identity_hashes_differ() {
        let first =
            hash_program_id(
                ProgramId::new(1)
            );

        let second =
            hash_program_id(
                ProgramId::new(2)
            );

        assert_ne!(
            first,
            second
        );
    }

    #[test]
    fn operation_identity_hashes_differ() {
        let first =
            hash_operation_id(
                OperationId::new(1)
            );

        let second =
            hash_operation_id(
                OperationId::new(2)
            );

        assert_ne!(
            first,
            second
        );
    }

    #[test]
    fn metadata_matches_contract() {
        let metadata =
            HashMetadata::new(
                HashDomain::Program
            );

        assert_eq!(
            metadata.schema_version,
            HASH_SCHEMA_VERSION
        );

        assert_eq!(
            metadata.algorithm,
            HashAlgorithm::Sha256
        );

        assert_eq!(
            metadata.digest_bytes,
            HASH_BYTES
        );

        assert_eq!(
            metadata.domain,
            HashDomain::Program
        );
    }

    #[test]
    fn default_hash_is_exactly_zero_bytes() {
        let hash = IrHash::default();

        assert!(
            hash.is_zero()
        );

        assert_eq!(
            hash.to_bytes(),
            [0u8; HASH_BYTES]
        );
    }
}