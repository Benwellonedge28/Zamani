//! Zamani Quantum IR — Canonical Hashing
//!
//! Production-grade, deterministic, streaming-capable SHA-256 hashing for
//! canonical Zamani Quantum IR representations.
//!
//! # Architectural role
//!
//! This module owns the *canonical hashing contract*.
//!
//! It converts already-canonical IR bytes into stable cryptographic content
//! identities:
//!
//! ```text
//! semantic IR
//!     │
//!     ▼
//! serialization::IrEncode
//!     │
//!     ▼
//! canonical serialized representation
//!     │
//!     ▼
//! hashing::canonical_hash
//!     │
//!     ▼
//! SHA-256 content identity
//! ```
//!
//! This module deliberately does NOT own:
//!
//! - IR semantics;
//! - gate definitions;
//! - qubit semantics;
//! - program structure;
//! - serialization semantics;
//! - validation;
//! - routing;
//! - scheduling;
//! - optimization;
//! - hardware;
//! - simulation;
//! - QEC;
//! - backend execution;
//! - digital signatures;
//! - encryption;
//! - key management.
//!
//! Those responsibilities belong to their respective IR/compiler layers.
//!
//! # Canonical identity rule
//!
//! The fundamental invariant is:
//!
//! ```text
//! same canonical bytes
//! + same hash schema
//! + same hash domain
//! + same algorithm
//! --------------------------------
//! = same IrHash
//! ```
//!
//! A semantic change MUST change canonical bytes. Therefore, after canonical
//! serialization, a semantic change MUST produce a different content hash
//! except for the mathematically negligible collision probability inherent in
//! SHA-256.
//!
//! # Important distinction
//!
//! A hash is a content identity, not an identity allocator.
//!
//! ```text
//! ProgramId       = stable object identity
//! QubitId         = logical qubit identity
//! PhysicalQubitId = physical-qubit identity vocabulary
//! IrHash          = content identity
//! ```
//!
//! The canonical qubit types remain owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module never defines replacement qubit types.
//!
//! # Scaling
//!
//! There is no fixed quantum-machine size encoded here.
//!
//! In particular, this module does not impose:
//!
//! - maximum qubits;
//! - maximum registers;
//! - maximum operations;
//! - maximum circuit depth;
//! - maximum topology size;
//! - maximum hardware size.
//!
//! A one-qubit program and an arbitrarily large finite program use exactly the
//! same hashing algorithm.
//!
//! The practical limit is determined by:
//!
//! - the size of the canonical representation;
//! - available memory when serialization materializes bytes;
//! - available storage;
//! - stream availability;
//! - caller-selected resource policies.
//!
//! Hashing itself is streaming-capable and does not require the complete
//! canonical byte representation to exist in one allocation when the caller
//! already has a byte stream.
//!
//! # Determinism
//!
//! This module never hashes:
//!
//! - pointer addresses;
//! - process IDs;
//! - thread IDs;
//! - allocator state;
//! - wall-clock time;
//! - random values;
//! - Rust `Debug` output;
//! - `HashMap` iteration order;
//! - host-dependent `usize` encodings.
//!
//! Hashing is performed only over explicitly defined canonical bytes.
//!
//! # Security
//!
//! SHA-256 is used as the canonical content-addressing algorithm.
//!
//! A hash does NOT provide authenticity:
//!
//! ```text
//! canonical IR
//!     ↓
//! SHA-256
//!     ↓
//! IrHash
//! ```
//!
//! Authenticity requires a separate signature system:
//!
//! ```text
//! canonical IR
//!     ↓
//! IrHash
//!     ↓
//! signing subsystem
//!     ↓
//! signature
//! ```
//!
//! This module intentionally does not manage keys or signatures.
//!
//! # Hash-domain separation
//!
//! Hashes are domain-separated so that identical canonical payloads used for
//! different semantic purposes do not accidentally become interchangeable
//! identities.
//!
//! The hashed framing is:
//!
//! ```text
//! HASH_DOMAIN_PREFIX
//! hash-schema-version
//! algorithm-id
//! domain-id
//! payload-length
//! canonical-payload
//! ```
//!
//! All integer fields in this framing use little-endian encoding.
//!
//! The payload length is included before the payload. This prevents ambiguous
//! concatenation between independently defined fields and makes the hashing
//! contract explicit.
//!
//! # Serialization boundary
//!
//! The existing serialization subsystem owns canonical IR serialization.
//!
//! This module therefore consumes:
//!
//! ```text
//! serialization::IrEncode
//! serialization::serialize
//! serialization::SerializedIr
//! ```
//!
//! It does not reproduce serialization logic.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! Requirements:
//!
//! - no nightly features;
//! - no unsafe code;
//! - no architecture-specific code;
//! - no fixed quantum-machine limit.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.

#![forbid(unsafe_code)]

use std::fmt;
use std::io::{self, Read};

use sha2::{Digest, Sha256};

use super::super::identity::IrVersion;
use super::super::qubit::{PhysicalQubitId, QubitId};
use super::super::serialization::{serialize, IrEncode, SerializationError, SerializedIr};

// =============================================================================
// Contract constants
// =============================================================================

/// Version of the canonical hashing wire/identity contract.
///
/// Increment this value whenever the bytes fed into SHA-256 change in a
/// backward-incompatible way.
pub const HASH_SCHEMA_VERSION: u16 = 1;

/// Number of bytes in a SHA-256 digest.
pub const HASH_BYTES: usize = 32;

/// Number of hexadecimal characters in a SHA-256 digest.
pub const HASH_HEX_BYTES: usize = HASH_BYTES * 2;

/// Stable domain-separation prefix.
pub const HASH_DOMAIN_PREFIX: &[u8] = b"ZAMANI-QIR-HASH";

/// Current hashing algorithm.
pub const HASH_ALGORITHM: HashAlgorithm = HashAlgorithm::Sha256;

// =============================================================================
// Hash algorithm
// =============================================================================

/// Algorithm used for canonical Quantum IR content hashing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HashAlgorithm {
    /// SHA-256.
    Sha256,
}

impl HashAlgorithm {
    /// Stable numeric identifier used in the canonical hash framing.
    #[must_use]
    pub const fn id(self) -> u8 {
        match self {
            Self::Sha256 => 1,
        }
    }

    /// Stable textual identifier.
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
// Hash domains
// =============================================================================

/// Semantic namespace used for hash domain separation.
///
/// Numeric identifiers are permanent parts of the hashing contract.
/// Existing identifiers MUST NOT be reused for another semantic meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HashDomain {
    /// Complete canonical Quantum IR document.
    Ir,

    /// Complete program.
    Program,

    /// Circuit representation.
    Circuit,

    /// Individual operation.
    Operation,

    /// Logical qubit identity/content.
    LogicalQubit,

    /// Physical qubit identity/content.
    PhysicalQubit,

    /// Generic IR value.
    Value,

    /// Symbolic parameter.
    Parameter,

    /// Pulse representation.
    Pulse,

    /// Waveform representation.
    Waveform,

    /// Channel representation.
    Channel,

    /// Frame representation.
    Frame,

    /// Schedule representation.
    Schedule,

    /// Resource representation.
    Resource,

    /// Capability representation.
    Capability,

    /// Mapping representation.
    Mapping,

    /// Provenance representation.
    Provenance,

    /// Extension representation.
    Extension,

    /// Generic canonical byte stream.
    Raw,
}

impl HashDomain {
    /// Returns the stable wire identifier.
    #[must_use]
    pub const fn id(self) -> u16 {
        match self {
            Self::Ir => 1,
            Self::Program => 2,
            Self::Circuit => 3,
            Self::Operation => 4,
            Self::LogicalQubit => 5,
            Self::PhysicalQubit => 6,
            Self::Value => 7,
            Self::Parameter => 8,
            Self::Pulse => 9,
            Self::Waveform => 10,
            Self::Channel => 11,
            Self::Frame => 12,
            Self::Schedule => 13,
            Self::Resource => 14,
            Self::Capability => 15,
            Self::Mapping => 16,
            Self::Provenance => 17,
            Self::Extension => 18,
            Self::Raw => 19,
        }
    }

    /// Returns the stable textual identifier.
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
// Hash errors
// =============================================================================

/// Errors produced by canonical hashing.
#[derive(Debug)]
pub enum HashError {
    /// Canonical IR serialization failed.
    Serialization(SerializationError),

    /// Reading a canonical byte stream failed.
    Io(io::Error),

    /// The supplied hexadecimal representation has the wrong length.
    InvalidHexLength {
        /// Number of hexadecimal characters supplied.
        length: usize,
    },

    /// A hexadecimal representation contains an invalid character.
    InvalidHexCharacter {
        /// Character position.
        index: usize,

        /// Invalid character.
        character: char,
    },

    /// The caller supplied a payload larger than the representable framing
    /// length.
    PayloadLengthOverflow,

    /// A hash builder has already been finalized.
    BuilderFinalized,
}

impl fmt::Display for HashError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serialization(error) => {
                write!(
                    formatter,
                    "canonical Quantum IR serialization failed during hashing: {error}"
                )
            }

            Self::Io(error) => {
                write!(
                    formatter,
                    "canonical Quantum IR hashing stream failed: {error}"
                )
            }

            Self::InvalidHexLength { length } => {
                write!(
                    formatter,
                    "invalid SHA-256 hash length {length}; expected {HASH_HEX_BYTES}"
                )
            }

            Self::InvalidHexCharacter { index, character } => {
                write!(
                    formatter,
                    "invalid hexadecimal hash character `{character}` at position {index}"
                )
            }

            Self::PayloadLengthOverflow => {
                formatter.write_str(
                    "canonical hash payload length cannot be represented by u64",
                )
            }

            Self::BuilderFinalized => {
                formatter.write_str(
                    "canonical hash builder has already been finalized",
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
            Self::InvalidHexLength { .. }
            | Self::InvalidHexCharacter { .. }
            | Self::PayloadLengthOverflow
            | Self::BuilderFinalized => None,
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

/// Fixed-size canonical SHA-256 content identity.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IrHash([u8; HASH_BYTES]);

impl IrHash {
    /// Creates a hash from an exact SHA-256 digest.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; HASH_BYTES]) -> Self {
        Self(bytes)
    }

    /// Returns the raw digest without allocation.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; HASH_BYTES] {
        &self.0
    }

    /// Copies the raw digest.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; HASH_BYTES] {
        self.0
    }

    /// Encodes the digest as lowercase hexadecimal.
    #[must_use]
    pub fn to_hex(self) -> String {
        let mut output = String::with_capacity(HASH_HEX_BYTES);

        for byte in self.0 {
            output.push(hex_digit(byte >> 4));
            output.push(hex_digit(byte & 0x0f));
        }

        output
    }

    /// Parses a lowercase or uppercase hexadecimal SHA-256 digest.
    pub fn from_hex(value: &str) -> Result<Self, HashError> {
        if value.len() != HASH_HEX_BYTES {
            return Err(HashError::InvalidHexLength {
                length: value.len(),
            });
        }

        let bytes = value.as_bytes();
        let mut digest = [0u8; HASH_BYTES];

        let mut index = 0usize;

        while index < HASH_BYTES {
            let high = hex_value(bytes[index * 2], index * 2)?;
            let low = hex_value(bytes[index * 2 + 1], index * 2 + 1)?;

            digest[index] = (high << 4) | low;
            index += 1;
        }

        Ok(Self(digest))
    }

    /// Returns whether this hash is all zeroes.
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.0.iter().all(|byte| *byte == 0)
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
// Semantic hash aliases
// =============================================================================

/// Content hash of a complete Quantum IR program/document.
pub type ProgramHash = IrHash;

/// Content hash of a circuit representation.
pub type CircuitHash = IrHash;

/// Content hash of an individual IR operation.
pub type OperationHash = IrHash;

// =============================================================================
// Canonical hash builder
// =============================================================================

/// Incremental canonical SHA-256 builder.
///
/// The builder hashes the canonical domain framing followed by payload bytes.
///
/// It is intentionally streaming-capable so callers with already-streamed IR
/// representations do not need to materialize the complete representation in
/// memory.
///
/// A builder is single-use: after [`CanonicalHashBuilder::finalize`] it cannot
/// be appended to again.
pub struct CanonicalHashBuilder {
    hasher: Sha256,
    finalized: bool,
}

impl CanonicalHashBuilder {
    /// Creates a canonical hash builder for the supplied semantic domain.
    #[must_use]
    pub fn new(domain: HashDomain) -> Self {
        Self::new_with_algorithm(
            HASH_ALGORITHM,
            domain,
        )
    }

    /// Creates a builder using an explicitly selected supported algorithm.
    #[must_use]
    pub fn new_with_algorithm(
        algorithm: HashAlgorithm,
        domain: HashDomain,
    ) -> Self {
        let mut hasher = Sha256::new();

        hasher.update(HASH_DOMAIN_PREFIX);
        hasher.update(HASH_SCHEMA_VERSION.to_le_bytes());
        hasher.update([algorithm.id()]);
        hasher.update(domain.id().to_le_bytes());

        Self {
            hasher,
            finalized: false,
        }
    }

    /// Appends a canonical payload-length field.
    ///
    /// This method should normally be used once, immediately before the
    /// payload. It is separate from `new` so that stream callers can determine
    /// or validate their payload size explicitly.
    pub fn write_payload_length(
        &mut self,
        length: u64,
    ) -> Result<(), HashError> {
        self.ensure_not_finalized()?;

        self.hasher.update(length.to_le_bytes());

        Ok(())
    }

    /// Appends canonical bytes to the digest.
    pub fn update(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), HashError> {
        self.ensure_not_finalized()?;

        self.hasher.update(bytes);

        Ok(())
    }

    /// Appends one canonical byte.
    pub fn update_byte(
        &mut self,
        byte: u8,
    ) -> Result<(), HashError> {
        self.ensure_not_finalized()?;

        self.hasher.update([byte]);

        Ok(())
    }

    /// Appends a canonical `u16`.
    pub fn update_u16(
        &mut self,
        value: u16,
    ) -> Result<(), HashError> {
        self.ensure_not_finalized()?;

        self.hasher.update(value.to_le_bytes());

        Ok(())
    }

    /// Appends a canonical `u32`.
    pub fn update_u32(
        &mut self,
        value: u32,
    ) -> Result<(), HashError> {
        self.ensure_not_finalized()?;

        self.hasher.update(value.to_le_bytes());

        Ok(())
    }

    /// Appends a canonical `u64`.
    pub fn update_u64(
        &mut self,
        value: u64,
    ) -> Result<(), HashError> {
        self.ensure_not_finalized()?;

        self.hasher.update(value.to_le_bytes());

        Ok(())
    }

    /// Appends a canonical `i64`.
    pub fn update_i64(
        &mut self,
        value: i64,
    ) -> Result<(), HashError> {
        self.ensure_not_finalized()?;

        self.hasher.update(value.to_le_bytes());

        Ok(())
    }

    /// Finalizes the digest.
    ///
    /// Finalization consumes the builder and therefore makes accidental
    /// post-finalization mutation impossible.
    #[must_use]
    pub fn finalize(self) -> IrHash {
        let digest = self.hasher.finalize();

        let mut bytes = [0u8; HASH_BYTES];
        bytes.copy_from_slice(&digest);

        IrHash::from_bytes(bytes)
    }

    fn ensure_not_finalized(&self) -> Result<(), HashError> {
        if self.finalized {
            Err(HashError::BuilderFinalized)
        } else {
            Ok(())
        }
    }
}

// =============================================================================
// Canonical byte hashing
// =============================================================================

/// Hashes canonical bytes using the supplied semantic domain.
///
/// The bytes MUST already be canonical according to the caller's IR contract.
pub fn hash_bytes(
    domain: HashDomain,
    bytes: &[u8],
) -> Result<IrHash, HashError> {
    let length = u64::try_from(bytes.len())
        .map_err(|_| HashError::PayloadLengthOverflow)?;

    let mut builder = CanonicalHashBuilder::new(domain);

    builder.write_payload_length(length)?;
    builder.update(bytes)?;

    Ok(builder.finalize())
}

/// Hashes a canonical byte reader without loading the complete input into
/// memory.
///
/// The reader's complete byte stream becomes the hash payload.
///
/// This function is suitable for very large IR artifacts when canonical bytes
/// are available through a streaming abstraction.
pub fn hash_reader<R>(
    domain: HashDomain,
    mut reader: R,
) -> Result<IrHash, HashError>
where
    R: Read,
{
    let mut builder = CanonicalHashBuilder::new(domain);

    /*
     * The payload length is part of the framing and must appear before the
     * payload. A generic Read implementation does not necessarily expose its
     * length, so we use a two-pass requirement only when the reader itself can
     * be recreated by the caller.
     *
     * Because a generic Read is single-pass, we instead use a length-neutral
     * stream framing for this API.
     *
     * To avoid introducing a second, incompatible hash contract, this method
     * does NOT call write_payload_length(0). Instead, it uses the explicit
     * streaming domain marker below.
     *
     * This distinction is intentionally permanent and domain-separated.
     */
    builder.update(b"STREAM")?;

    let mut buffer = [0u8; 64 * 1024];
    let mut total = 0u64;

    loop {
        let read = reader.read(&mut buffer)?;

        if read == 0 {
            break;
        }

        total = total
            .checked_add(
                u64::try_from(read)
                    .map_err(|_| HashError::PayloadLengthOverflow)?,
            )
            .ok_or(HashError::PayloadLengthOverflow)?;

        builder.update(&buffer[..read])?;
    }

    builder.update_u64(total)?;

    Ok(builder.finalize())
}

/// Hashes a canonical serialized IR object.
///
/// The complete serialization framing returned by `serialize` is hashed. This
/// means the semantic IR version and serialization format remain part of the
/// content identity.
pub fn hash_ir<T>(
    value: &T,
) -> Result<IrHash, HashError>
where
    T: IrEncode,
{
    let serialized = serialize(value)?;

    hash_serialized(HashDomain::Ir, &serialized)
}

/// Hashes an explicitly serialized IR document.
///
/// The document's canonical serialization bytes are treated as the payload.
pub fn hash_serialized(
    domain: HashDomain,
    serialized: &SerializedIr,
) -> Result<IrHash, HashError> {
    hash_bytes(
        domain,
        serialized.as_bytes(),
    )
}

// =============================================================================
// Version-aware hashing
// =============================================================================

/// Hashes canonical payload bytes while explicitly binding the semantic IR
/// version into the domain framing.
///
/// This is useful when a caller has canonical payload bytes but has not yet
/// constructed a complete `SerializedIr`.
pub fn hash_versioned_bytes(
    domain: HashDomain,
    ir_version: IrVersion,
    bytes: &[u8],
) -> Result<IrHash, HashError> {
    let length = u64::try_from(bytes.len())
        .map_err(|_| HashError::PayloadLengthOverflow)?;

    let mut builder = CanonicalHashBuilder::new(domain);

    builder.update_u16(ir_version.major())?;
    builder.update_u16(ir_version.minor())?;
    builder.update_u16(ir_version.patch())?;
    builder.write_payload_length(length)?;
    builder.update(bytes)?;

    Ok(builder.finalize())
}

// =============================================================================
// Canonical identity helpers
// =============================================================================

/// Hashes a canonical logical-qubit identity.
///
/// `QubitId` remains owned by `quantum::ir::qubit`.
pub fn hash_logical_qubit_id(
    qubit: QubitId,
) -> Result<IrHash, HashError> {
    hash_u64_domain(
        HashDomain::LogicalQubit,
        qubit.index() as u64,
    )
}

/// Hashes a canonical physical-qubit identity.
///
/// `PhysicalQubitId` remains owned by `quantum::ir::qubit`.
pub fn hash_physical_qubit_id(
    qubit: PhysicalQubitId,
) -> Result<IrHash, HashError> {
    hash_u64_domain(
        HashDomain::PhysicalQubit,
        qubit.index() as u64,
    )
}

/// Hashes a stable numeric IR identity.
///
/// This helper is intended for identity types whose canonical wire identity is
/// already explicitly defined as `u64`.
pub fn hash_identity(
    domain: HashDomain,
    identity: u64,
) -> Result<IrHash, HashError> {
    hash_u64_domain(domain, identity)
}

fn hash_u64_domain(
    domain: HashDomain,
    value: u64,
) -> Result<IrHash, HashError> {
    let bytes = value.to_le_bytes();

    hash_bytes(domain, &bytes)
}

// =============================================================================
// Equality / difference helpers
// =============================================================================

/// Returns whether two canonical hashes are equal.
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
// Hexadecimal helpers
// =============================================================================

fn hex_digit(value: u8) -> char {
    match value {
        0..=9 => (b'0' + value) as char,
        10..=15 => (b'a' + (value - 10)) as char,
        _ => unreachable!(),
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
        other => Err(HashError::InvalidHexCharacter {
            index,
            character: other as char,
        }),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_fixed_size() {
        let hash = hash_bytes(
            HashDomain::Raw,
            b"zamani",
        )
        .expect("hashing must succeed");

        assert_eq!(
            hash.as_bytes().len(),
            HASH_BYTES
        );
    }

    #[test]
    fn hash_is_deterministic() {
        let first = hash_bytes(
            HashDomain::Raw,
            b"zamani quantum ir",
        )
        .expect("first hash must succeed");

        let second = hash_bytes(
            HashDomain::Raw,
            b"zamani quantum ir",
        )
        .expect("second hash must succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn different_payloads_have_different_hashes() {
        let first = hash_bytes(
            HashDomain::Raw,
            b"one",
        )
        .expect("first hash must succeed");

        let second = hash_bytes(
            HashDomain::Raw,
            b"two",
        )
        .expect("second hash must succeed");

        assert_ne!(first, second);
    }

    #[test]
    fn domains_are_separated() {
        let raw = hash_bytes(
            HashDomain::Raw,
            b"same",
        )
        .expect("raw hash must succeed");

        let program = hash_bytes(
            HashDomain::Program,
            b"same",
        )
        .expect("program hash must succeed");

        assert_ne!(raw, program);
    }

    #[test]
    fn algorithm_is_explicit() {
        assert_eq!(
            HASH_ALGORITHM,
            HashAlgorithm::Sha256
        );

        assert_eq!(
            HASH_ALGORITHM.id(),
            1
        );

        assert_eq!(
            HASH_ALGORITHM.name(),
            "sha256"
        );
    }

    #[test]
    fn domain_ids_are_stable() {
        assert_eq!(HashDomain::Ir.id(), 1);
        assert_eq!(HashDomain::Program.id(), 2);
        assert_eq!(HashDomain::Circuit.id(), 3);
        assert_eq!(HashDomain::Operation.id(), 4);
        assert_eq!(HashDomain::LogicalQubit.id(), 5);
        assert_eq!(HashDomain::PhysicalQubit.id(), 6);
        assert_eq!(HashDomain::Raw.id(), 19);
    }

    #[test]
    fn hash_hex_round_trip() {
        let original = hash_bytes(
            HashDomain::Raw,
            b"round trip",
        )
        .expect("hashing must succeed");

        let encoded = original.to_hex();

        let decoded =
            IrHash::from_hex(&encoded)
                .expect("hex decoding must succeed");

        assert_eq!(
            original,
            decoded
        );
    }

    #[test]
    fn uppercase_hex_is_accepted() {
        let original = hash_bytes(
            HashDomain::Raw,
            b"uppercase",
        )
        .expect("hashing must succeed");

        let uppercase = original.to_hex().to_uppercase();

        let decoded =
            IrHash::from_hex(&uppercase)
                .expect("uppercase hex must decode");

        assert_eq!(
            original,
            decoded
        );
    }

    #[test]
    fn invalid_hex_length_is_rejected() {
        let error =
            IrHash::from_hex("abcd")
                .expect_err("invalid length must fail");

        assert!(matches!(
            error,
            HashError::InvalidHexLength { .. }
        ));
    }

    #[test]
    fn invalid_hex_character_is_rejected() {
        let mut value =
            "000000000000000000000000000000000000000000000000000000000000000g"
                .to_owned();

        value.replace_range(
            63..64,
            "g",
        );

        let error =
            IrHash::from_hex(&value)
                .expect_err("invalid character must fail");

        assert!(matches!(
            error,
            HashError::InvalidHexCharacter { .. }
        ));
    }

    #[test]
    fn zero_hash_is_detected() {
        assert!(
            IrHash::default().is_zero()
        );

        let hash = hash_bytes(
            HashDomain::Raw,
            b"nonzero",
        )
        .expect("hashing must succeed");

        assert!(!hash.is_zero());
    }

    #[test]
    fn logical_qubit_hashes_are_domain_separated() {
        let logical =
            hash_logical_qubit_id(QubitId::new(7))
                .expect("logical qubit hashing must succeed");

        let physical =
            hash_physical_qubit_id(
                PhysicalQubitId::new(7)
            )
            .expect("physical qubit hashing must succeed");

        assert_ne!(
            logical,
            physical
        );
    }

    #[test]
    fn logical_qubit_identity_is_stable() {
        let first =
            hash_logical_qubit_id(QubitId::new(42))
                .expect("hashing must succeed");

        let second =
            hash_logical_qubit_id(QubitId::new(42))
                .expect("hashing must succeed");

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn different_qubit_ids_differ() {
        let first =
            hash_logical_qubit_id(QubitId::new(1))
                .expect("hashing must succeed");

        let second =
            hash_logical_qubit_id(QubitId::new(2))
                .expect("hashing must succeed");

        assert_ne!(
            first,
            second
        );
    }

    #[test]
    fn builder_matches_byte_hashing() {
        let payload = b"builder equivalence";

        let direct =
            hash_bytes(
                HashDomain::Raw,
                payload,
            )
            .expect("direct hash must succeed");

        let mut builder =
            CanonicalHashBuilder::new(
                HashDomain::Raw
            );

        builder
            .write_payload_length(
                payload.len() as u64
            )
            .expect("length write must succeed");

        builder
            .update(payload)
            .expect("payload write must succeed");

        let incremental =
            builder.finalize();

        assert_eq!(
            direct,
            incremental
        );
    }

    #[test]
    fn reader_hash_is_deterministic() {
        let first =
            hash_reader(
                HashDomain::Raw,
                &b"streamed data"[..],
            )
            .expect("first stream hash must succeed");

        let second =
            hash_reader(
                HashDomain::Raw,
                &b"streamed data"[..],
            )
            .expect("second stream hash must succeed");

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn empty_payload_is_hashable() {
        let hash =
            hash_bytes(
                HashDomain::Raw,
                &[],
            )
            .expect("empty payload must be hashable");

        assert!(!hash.is_zero());
    }

    #[test]
    fn versioned_hash_changes_with_version() {
        let first =
            hash_versioned_bytes(
                HashDomain::Program,
                IrVersion::new(1, 0, 0),
                b"program",
            )
            .expect("first hash must succeed");

        let second =
            hash_versioned_bytes(
                HashDomain::Program,
                IrVersion::new(1, 0, 1),
                b"program",
            )
            .expect("second hash must succeed");

        assert_ne!(
            first,
            second
        );
    }

    #[test]
    fn hashes_equal_and_differ_are_consistent() {
        let first =
            hash_bytes(
                HashDomain::Raw,
                b"a",
            )
            .expect("hashing must succeed");

        let second =
            hash_bytes(
                HashDomain::Raw,
                b"a",
            )
            .expect("hashing must succeed");

        let third =
            hash_bytes(
                HashDomain::Raw,
                b"b",
            )
            .expect("hashing must succeed");

        assert!(
            hashes_equal(
                &first,
                &second
            )
        );

        assert!(
            hashes_differ(
                &first,
                &third
            )
        );
    }
}