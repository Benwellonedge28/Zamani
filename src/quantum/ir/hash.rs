//! Zamani Quantum IR — Canonical Hashing
//!
//! Production-grade, deterministic, cryptographic hashing infrastructure for
//! the Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `hash.rs` provides stable content identity for canonical Quantum IR.
//!
//! The fundamental pipeline is:
//!
//! ```text
//! semantic IR
//!     │
//!     ▼
//! serialization.rs
//!     │
//!     ▼
//! canonical bytes
//!     │
//!     ▼
//! hash.rs
//!     │
//!     ▼
//! cryptographic content identity
//! ```
//!
//! `hash.rs` deliberately does NOT define the semantics of:
//!
//! - gates;
//! - measurements;
//! - pulses;
//! - waveforms;
//! - channels;
//! - frames;
//! - scheduling;
//! - routing;
//! - hardware;
//! - optimization;
//! - simulation;
//! - QEC;
//! - frontend syntax;
//! - backend execution.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Canonical identity principle
//!
//! A Quantum IR hash is a hash of canonical serialized representation.
//!
//! Therefore:
//!
//! ```text
//! same semantic IR
//! + same IR version
//! + same serialization format
//! + same canonical encoding
//! --------------------------------
//! = same hash
//! ```
//!
//! Conversely, a semantic change must produce different canonical bytes and,
//! consequently, a different hash.
//!
//! # Important distinction
//!
//! This module distinguishes:
//!
//! - `IrHash`: canonical SHA-256 content identity;
//! - `ProgramHash`: semantic alias for a complete-program hash;
//! - `CircuitHash`: semantic alias for a circuit hash;
//! - `OperationHash`: semantic alias for an operation hash;
//! - `HashBuilder`: incremental canonical-byte hashing;
//! - `HashDomain`: domain separation for different IR object categories.
//!
//! These hashes are content identities.
//!
//! They are NOT:
//!
//! - cryptographic signatures;
//! - authentication tokens;
//! - encryption keys;
//! - authorization credentials;
//! - hardware identifiers;
//! - calibration identifiers;
//! - random identifiers.
//!
//! A hash alone does not prove who created an IR object.
//!
//! # Hash algorithm
//!
//! The production algorithm is SHA-256.
//!
//! SHA-256 is already a direct dependency of the Zamani repository, so this
//! module does not introduce another hashing dependency.
//!
//! The algorithm is deliberately explicit rather than using Rust's standard
//! `Hash`/`Hasher` infrastructure because the standard-library hashers are not
//! a canonical persistent serialization contract.
//!
//! # Scalability
//!
//! There is no fixed quantum-machine size encoded here.
//!
//! In particular, this module does NOT define:
//!
//! ```text
//! 63
//! 64
//! 4096
//! 1_000_000
//! ```
//!
//! as qubit or machine limits.
//!
//! A program containing one qubit and a program containing an arbitrarily large
//! finite number of qubits use exactly the same hashing model.
//!
//! The practical maximum is determined by:
//!
//! - available memory when canonical serialization materializes bytes;
//! - available storage;
//! - the selected serialization policy;
//! - operating-system/process limits;
//! - the size of the actual IR document.
//!
//! For extremely large artifacts, `HashBuilder` and `hash_reader` permit
//! incremental hashing without requiring the complete input to be held in one
//! allocation.
//!
//! # Determinism
//!
//! This module never hashes:
//!
//! - pointer addresses;
//! - memory addresses;
//! - process identifiers;
//! - thread identifiers;
//! - random values;
//! - wall-clock time;
//! - allocator state;
//! - `usize` representations;
//! - Rust debug output.
//!
//! Hashes are therefore suitable for:
//!
//! - reproducible compilation;
//! - content-addressed storage;
//! - compilation caching;
//! - distributed compilation;
//! - job identity;
//! - benchmark reproducibility;
//! - provenance;
//! - artifact deduplication;
//! - incremental compilation;
//! - transformation tracking.
//!
//! # Domain separation
//!
//! The same bytes interpreted as different semantic object categories must not
//! accidentally produce the same logical identity.
//!
//! Therefore object hashes use an explicit domain prefix:
//!
//! ```text
//! ZAMANI-QIR-HASH
//! + hash schema version
//! + object domain
//! + canonical bytes
//! ```
//!
//! For complete serialized IR documents, the serialized document already
//! contains its own serialization and IR-version framing. Domain separation is
//! still applied to prevent accidental cross-category reuse.
//!
//! # Quantum identity boundary
//!
//! Canonical logical and physical qubit identities remain owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module imports those exact types when hashing qubit identities.
//! It never defines duplicate qubit identity types.
//!
//! # Serialization integration
//!
//! `serialization.rs` defines:
//!
//! ```text
//! IrEncode
//! IrDecode
//! serialize()
//! SerializedIr
//! ```
//!
//! Any future IR type that implements `IrEncode` automatically becomes
//! hashable through:
//!
//! ```text
//! hash_ir(&value)
//! ```
//!
//! This intentionally allows `hash.rs` to be completed before:
//!
//! - program.rs;
//! - operation.rs;
//! - region.rs;
//! - pulse.rs;
//! - waveform.rs;
//! - channel.rs;
//! - frame.rs;
//! - mapping.rs;
//! - capability.rs;
//! - provenance.rs;
//!
//! are completed.
//!
//! Those modules only need to implement the already-established
//! `serialization::IrEncode` contract.
//!
//! # Security boundary
//!
//! SHA-256 provides cryptographic collision resistance appropriate for content
//! identity, but a hash is not a signature.
//!
//! For authenticity:
//!
//! ```text
//! IR bytes
//!     ↓
//! hash.rs
//!     ↓
//! SHA-256 digest
//!     ↓
//! signing subsystem
//!     ↓
//! digital signature
//! ```
//!
//! Signing remains outside this module.
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

use super::identity::{
    IrVersion,
    OperationId,
    ProgramId,
};
use super::qubit::{
    PhysicalQubitId,
    QubitId,
};
use super::serialization::{
    serialize,
    IrEncode,
    SerializationError,
    SerializedIr,
};

// =============================================================================
// Hash schema
// =============================================================================

/// Version of the Zamani Quantum IR hashing contract.
///
/// This is deliberately independent from:
///
/// - Quantum IR semantic version;
/// - serialization format version;
/// - Zamani language version;
/// - compiler version;
/// - hardware version.
///
/// A breaking change to hash-domain construction or digest interpretation must
/// increment this version.
pub const HASH_SCHEMA_VERSION: u16 = 1;

/// SHA-256 digest length in bytes.
pub const HASH_BYTES: usize = 32;

/// SHA-256 digest length in hexadecimal characters.
pub const HASH_HEX_BYTES: usize = HASH_BYTES * 2;

/// Domain-separation prefix for all Zamani Quantum IR hashes.
pub const HASH_DOMAIN_PREFIX: &[u8] = b"ZAMANI-QIR-HASH";

/// Current hash algorithm identifier.
pub const HASH_ALGORITHM: HashAlgorithm = HashAlgorithm::Sha256;

// =============================================================================
// Hash algorithm
// =============================================================================

/// Cryptographic hash algorithm used by the canonical Quantum IR hashing
/// contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HashAlgorithm {
    /// SHA-256.
    Sha256,
}

impl HashAlgorithm {
    /// Returns the stable numeric algorithm identifier.
    ///
    /// The numeric identifier is part of the hash contract.
    #[must_use]
    pub const fn id(self) -> u8 {
        match self {
            Self::Sha256 => 1,
        }
    }

    /// Returns the stable algorithm name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Sha256 => "sha256",
        }
    }
}

impl fmt::Display for HashAlgorithm {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Hash domain
// =============================================================================

/// Semantic domain used to separate different classes of IR content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HashDomain {
    /// Complete Quantum IR document/program.
    Ir,

    /// Complete quantum program.
    Program,

    /// Quantum circuit.
    Circuit,

    /// Individual IR operation.
    Operation,

    /// Logical qubit identity.
    LogicalQubit,

    /// Physical qubit identity.
    PhysicalQubit,

    /// Generic IR value.
    Value,

    /// IR parameter.
    Parameter,

    /// IR pulse.
    Pulse,

    /// IR waveform.
    Waveform,

    /// IR channel.
    Channel,

    /// IR frame.
    Frame,

    /// IR schedule.
    Schedule,

    /// IR resource requirement.
    Resource,

    /// IR capability requirement.
    Capability,

    /// IR mapping.
    Mapping,

    /// IR provenance.
    Provenance,

    /// IR extension.
    Extension,

    /// Generic canonical bytes.
    Raw,
}

impl HashDomain {
    /// Returns the stable domain identifier.
    ///
    /// These identifiers are part of the hash contract and therefore must not
    /// be reordered or reused for another semantic purpose.
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

    /// Returns a stable textual domain name.
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
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Hash error
// =============================================================================

/// Errors produced by Quantum IR hashing.
#[derive(Debug)]
pub enum HashError {
    /// Canonical serialization failed.
    Serialization(SerializationError),

    /// An incremental hash stream failed while reading its input.
    Io(io::Error),

    /// A supplied hexadecimal hash string had an invalid length.
    InvalidHexLength {
        /// Number of characters supplied.
        length: usize,
    },

    /// A supplied hexadecimal hash string contained an invalid character.
    InvalidHexCharacter {
        /// Character position.
        index: usize,

        /// Invalid character.
        character: char,
    },
}

impl fmt::Display for HashError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Serialization(error) => {
                write!(
                    formatter,
                    "cannot hash Quantum IR because canonical serialization failed: {error}"
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
                    "invalid Quantum IR hash hexadecimal length {length}; expected {HASH_HEX_BYTES}"
                )
            }

            Self::InvalidHexCharacter {
                index,
                character,
            } => {
                write!(
                    formatter,
                    "invalid Quantum IR hash hexadecimal character `{character}` at position {index}"
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

/// Canonical 256-bit content identity for Zamani Quantum IR.
///
/// `IrHash` is deliberately represented as exactly 32 bytes rather than as a
/// variable-length vector.
///
/// This gives the type:
///
/// - stable size;
/// - stable serialization;
/// - stable equality semantics;
/// - no allocation;
/// - no architecture-dependent representation.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IrHash([u8; HASH_BYTES]);

impl IrHash {
    /// Creates a hash from its exact 32-byte digest.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; HASH_BYTES]) -> Self {
        Self(bytes)
    }

    /// Returns the raw digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; HASH_BYTES] {
        &self.0
    }

    /// Copies the digest into a fixed-size array.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; HASH_BYTES] {
        self.0
    }

    /// Returns the digest as lowercase hexadecimal.
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
    pub fn from_hex(input: &str) -> Result<Self, HashError> {
        if input.len() != HASH_HEX_BYTES {
            return Err(HashError::InvalidHexLength {
                length: input.len(),
            });
        }

        let bytes = input.as_bytes();
        let mut output = [0u8; HASH_BYTES];

        let mut index = 0usize;

        while index < HASH_BYTES {
            let high = hex_value(bytes[index * 2], index * 2)?;
            let low = hex_value(bytes[index * 2 + 1], index * 2 + 1)?;

            output[index] = (high << 4) | low;

            index += 1;
        }

        Ok(Self(output))
    }

    /// Returns the cryptographic algorithm used by this hash.
    #[must_use]
    pub const fn algorithm(self) -> HashAlgorithm {
        HASH_ALGORITHM
    }

    /// Returns the digest length.
    #[must_use]
    pub const fn len(self) -> usize {
        HASH_BYTES
    }

    /// Returns whether the digest is all zero.
    ///
    /// This is mostly useful for defensive validation and tests. A real
    /// SHA-256 digest can theoretically be zero, so this function MUST NOT be
    /// interpreted as proof of invalidity.
    #[must_use]
    pub fn is_zero(self) -> bool {
        let mut result = true;

        for byte in self.0 {
            if byte != 0 {
                result = false;
                break;
            }
        }

        result
    }
}

impl fmt::Debug for IrHash {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_tuple("IrHash")
            .field(&self.to_hex())
            .finish()
    }
}

impl fmt::Display for IrHash {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.to_hex())
    }
}

// =============================================================================
// Semantic aliases
// =============================================================================

/// Canonical hash identifying a complete Zamani quantum program.
///
/// This is a semantic alias of [`IrHash`] and intentionally has the same
/// representation.
pub type ProgramHash = IrHash;

/// Canonical hash identifying a quantum circuit.
///
/// This is a semantic alias of [`IrHash`].
pub type CircuitHash = IrHash;

/// Canonical hash identifying an individual IR operation.
///
/// This is a semantic alias of [`IrHash`].
pub type OperationHash = IrHash;

/// Canonical hash identifying a logical qubit identity.
///
/// This is a semantic alias of [`IrHash`].
pub type QubitHash = IrHash;

/// Canonical hash identifying a physical qubit identity.
///
/// This is a semantic alias of [`IrHash`].
pub type PhysicalQubitHash = IrHash;

// =============================================================================
// Hash builder
// =============================================================================

/// Incremental canonical hash builder.
///
/// This type is intended for very large IR documents and streaming pipelines.
///
/// It does not accumulate the entire input in memory.
///
/// Example:
///
/// ```text
/// let mut builder = HashBuilder::new(HashDomain::Program);
/// builder.update(chunk_a);
/// builder.update(chunk_b);
/// let hash = builder.finalize();
/// ```
///
/// The domain separator is included before the first user-provided byte.
pub struct HashBuilder {
    hasher: Sha256,
    finalized: bool,
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

        Self {
            hasher,
            finalized: false,
        }
    }

    /// Adds canonical bytes to the digest.
    ///
    /// Calling this method after `finalize` is prevented by the builder's
    /// internal state and is treated as a no-op.
    ///
    /// This design avoids panics in long-running compiler pipelines.
    pub fn update(
        &mut self,
        bytes: &[u8],
    ) {
        if self.finalized {
            return;
        }

        self.hasher.update(bytes);
    }

    /// Adds a single byte.
    pub fn update_byte(
        &mut self,
        byte: u8,
    ) {
        self.update(&[byte]);
    }

    /// Adds a little-endian `u16`.
    pub fn update_u16(
        &mut self,
        value: u16,
    ) {
        self.update(&value.to_le_bytes());
    }

    /// Adds a little-endian `u32`.
    pub fn update_u32(
        &mut self,
        value: u32,
    ) {
        self.update(&value.to_le_bytes());
    }

    /// Adds a little-endian `u64`.
    pub fn update_u64(
        &mut self,
        value: u64,
    ) {
        self.update(&value.to_le_bytes());
    }

    /// Adds a little-endian `u128`.
    pub fn update_u128(
        &mut self,
        value: u128,
    ) {
        self.update(&value.to_le_bytes());
    }

    /// Adds a length-prefixed byte sequence.
    ///
    /// The length is encoded as `u64` to prevent architecture-dependent
    /// hashing caused by `usize`.
    pub fn update_length_prefixed_bytes(
        &mut self,
        bytes: &[u8],
    ) {
        let length = bytes.len() as u64;

        self.update_u64(length);
        self.update(bytes);
    }

    /// Adds a UTF-8 string using canonical length-prefixed encoding.
    pub fn update_str(
        &mut self,
        value: &str,
    ) {
        self.update_length_prefixed_bytes(
            value.as_bytes(),
        );
    }

    /// Returns the final digest.
    ///
    /// Finalization consumes the builder so the resulting digest cannot be
    /// accidentally modified.
    #[must_use]
    pub fn finalize(
        mut self,
    ) -> IrHash {
        self.finalized = true;

        let digest = self.hasher.finalize();

        let mut bytes = [0u8; HASH_BYTES];

        bytes.copy_from_slice(&digest);

        IrHash::from_bytes(bytes)
    }
}

// =============================================================================
// Hashing canonical IR
// =============================================================================

/// Hashes a complete canonical IR object.
///
/// The object is first serialized through the canonical `serialization.rs`
/// contract and then hashed.
///
/// This is the primary API future IR modules should use.
///
/// Example:
///
/// ```text
/// let program_hash = hash_ir(&program)?;
/// ```
pub fn hash_ir<T>(
    value: &T,
) -> Result<IrHash, HashError>
where
    T: IrEncode,
{
    let serialized = serialize(value)?;

    hash_serialized(&serialized)
}

/// Hashes a canonical IR object using an explicit semantic IR version.
///
/// The supplied version is embedded by `serialization.rs` in the canonical
/// document before hashing.
pub fn hash_ir_with_version<T>(
    value: &T,
    version: IrVersion,
) -> Result<IrHash, HashError>
where
    T: IrEncode,
{
    use super::serialization::serialize_with_version;

    let serialized = serialize_with_version(
        value,
        version,
    )?;

    hash_serialized(&serialized)
}

/// Hashes the exact canonical serialized document returned by
/// `serialization::serialize`.
///
/// This function deliberately hashes the complete document rather than merely
/// its payload, so the serialization format and semantic IR version are part
/// of the content identity.
pub fn hash_serialized(
    serialized: &SerializedIr,
) -> Result<IrHash, HashError> {
    hash_bytes(
        HashDomain::Ir,
        serialized.as_bytes(),
    )
}

/// Hashes a byte slice using the supplied semantic domain.
///
/// This is useful for downstream modules that already possess canonical bytes.
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

/// Hashes a byte stream incrementally.
///
/// This API is intended for very large artifacts that should not be loaded
/// completely into memory.
///
/// The reader is consumed in bounded chunks.
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
// Qubit hashing
// =============================================================================

/// Hashes a canonical logical-qubit identity.
///
/// The exact canonical type from `quantum::ir::qubit::QubitId` is used.
///
/// The logical and physical domains are deliberately distinct.
#[must_use]
pub fn hash_qubit_id(
    qubit: QubitId,
) -> QubitHash {
    let mut builder =
        HashBuilder::new(HashDomain::LogicalQubit);

    builder.update_u64(
        qubit.index() as u64,
    );

    builder.finalize()
}

/// Hashes a canonical physical-qubit identity.
///
/// The exact canonical type from `quantum::ir::qubit::PhysicalQubitId` is
/// used.
///
/// This does NOT claim that the physical qubit exists on a particular
/// hardware target.
#[must_use]
pub fn hash_physical_qubit_id(
    qubit: PhysicalQubitId,
) -> PhysicalQubitHash {
    let mut builder =
        HashBuilder::new(HashDomain::PhysicalQubit);

    builder.update_u64(
        qubit.index() as u64,
    );

    builder.finalize()
}

// =============================================================================
// Identity hashing
// =============================================================================

/// Hashes a canonical `ProgramId`.
///
/// This function hashes the identity token itself, not the program contents.
///
/// For content identity use [`hash_ir`] instead.
#[must_use]
pub fn hash_program_id(
    id: ProgramId,
) -> IrHash {
    hash_u64_identity(
        HashDomain::Program,
        id.value(),
    )
}

/// Hashes a canonical `OperationId`.
///
/// This is an identity-token hash, not an operation-content hash.
#[must_use]
pub fn hash_operation_id(
    id: OperationId,
) -> IrHash {
    hash_u64_identity(
        HashDomain::Operation,
        id.value(),
    )
}

/// Hashes a stable `u64` identity in a specified domain.
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
// Hash comparison helpers
// =============================================================================

/// Returns whether two hashes are equal.
///
/// This helper exists primarily to give callers a semantic API and avoid
/// coupling them to the internal representation of `IrHash`.
#[must_use]
pub const fn hashes_equal(
    left: &IrHash,
    right: &IrHash,
) -> bool {
    left.0 == right.0
}

/// Returns whether two hashes differ.
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

/// Metadata describing the canonical hashing contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HashMetadata {
    /// Hash schema version.
    pub schema_version: u16,

    /// Cryptographic algorithm.
    pub algorithm: HashAlgorithm,

    /// Digest length.
    pub digest_bytes: usize,

    /// Domain used for the digest.
    pub domain: HashDomain,
}

impl HashMetadata {
    /// Creates metadata for a hash domain.
    #[must_use]
    pub const fn new(
        domain: HashDomain,
    ) -> Self {
        Self {
            schema_version: HASH_SCHEMA_VERSION,
            algorithm: HASH_ALGORITHM,
            digest_bytes: HASH_BYTES,
            domain,
        }
    }
}

// =============================================================================
// Internal domain framing
// =============================================================================

/// Writes the canonical domain separator into a SHA-256 hasher.
///
/// The framing is:
///
/// ```text
/// prefix length  u16 LE
/// prefix         bytes
/// schema         u16 LE
/// algorithm      u8
/// domain         u16 LE
/// ```
///
/// Every field is explicit and architecture-independent.
fn write_domain_header(
    hasher: &mut Sha256,
    domain: HashDomain,
) {
    hasher.update(
        &(HASH_DOMAIN_PREFIX.len() as u16).to_le_bytes(),
    );

    hasher.update(HASH_DOMAIN_PREFIX);

    hasher.update(
        &HASH_SCHEMA_VERSION.to_le_bytes(),
    );

    hasher.update(&[HASH_ALGORITHM.id()]);

    hasher.update(
        &domain.id().to_le_bytes(),
    );
}

// =============================================================================
// Hex helpers
// =============================================================================

fn hex_digit(
    value: u8,
) -> char {
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
            },
        ),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_exactly_32_bytes() {
        let hash = hash_bytes(
            HashDomain::Raw,
            b"zamani",
        );

        assert_eq!(
            hash.as_bytes().len(),
            HASH_BYTES
        );

        assert_eq!(
            hash.len(),
            HASH_BYTES
        );
    }

    #[test]
    fn same_bytes_same_domain_produce_same_hash() {
        let first = hash_bytes(
            HashDomain::Program,
            b"zamani",
        );

        let second = hash_bytes(
            HashDomain::Program,
            b"zamani",
        );

        assert_eq!(first, second);
    }

    #[test]
    fn different_bytes_produce_different_hashes() {
        let first = hash_bytes(
            HashDomain::Program,
            b"zamani-a",
        );

        let second = hash_bytes(
            HashDomain::Program,
            b"zamani-b",
        );

        assert_ne!(first, second);
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

        assert_ne!(program, circuit);
    }

    #[test]
    fn hash_is_deterministic() {
        let first = hash_bytes(
            HashDomain::Raw,
            b"deterministic",
        );

        let second = hash_bytes(
            HashDomain::Raw,
            b"deterministic",
        );

        assert_eq!(first, second);
        assert_eq!(
            first.to_hex(),
            second.to_hex()
        );
    }

    #[test]
    fn hex_round_trip() {
        let original = hash_bytes(
            HashDomain::Program,
            b"round-trip",
        );

        let encoded = original.to_hex();

        assert_eq!(
            encoded.len(),
            HASH_HEX_BYTES
        );

        let decoded =
            IrHash::from_hex(&encoded)
                .expect("valid hash must parse");

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
        );

        let uppercase =
            original.to_hex().to_uppercase();

        let decoded =
            IrHash::from_hex(&uppercase)
                .expect("uppercase hexadecimal must parse");

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
            Err(HashError::InvalidHexLength {
                length: 2
            })
        ));
    }

    #[test]
    fn invalid_hex_character_is_rejected() {
        let mut input =
            "000000000000000000000000000000000000000000000000000000000000000g"
                .to_string();

        let result =
            IrHash::from_hex(&input);

        assert!(matches!(
            result,
            Err(HashError::InvalidHexCharacter { .. })
        ));

        input.clear();
    }

    #[test]
    fn incremental_hash_matches_single_update() {
        let complete = hash_bytes(
            HashDomain::Raw,
            b"abcdef",
        );

        let mut builder =
            HashBuilder::new(HashDomain::Raw);

        builder.update(b"ab");
        builder.update(b"cd");
        builder.update(b"ef");

        let incremental =
            builder.finalize();

        assert_eq!(
            complete,
            incremental
        );
    }

    #[test]
    fn incremental_integer_encoding_is_deterministic() {
        let mut first =
            HashBuilder::new(HashDomain::Raw);

        first.update_u16(0x1234);
        first.update_u32(0x12345678);
        first.update_u64(0x123456789abcdef0);
        first.update_u128(
            0x123456789abcdef00123456789abcdef,
        );

        let first = first.finalize();

        let mut second =
            HashBuilder::new(HashDomain::Raw);

        second.update_u16(0x1234);
        second.update_u32(0x12345678);
        second.update_u64(0x123456789abcdef0);
        second.update_u128(
            0x123456789abcdef00123456789abcdef,
        );

        let second = second.finalize();

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn logical_and_physical_qubits_are_domain_separated() {
        let logical =
            hash_qubit_id(QubitId::new(17));

        let physical =
            hash_physical_qubit_id(
                PhysicalQubitId::new(17),
            );

        assert_ne!(
            logical,
            physical
        );
    }

    #[test]
    fn qubit_identity_hash_does_not_confuse_index_with_domain() {
        let logical_zero =
            hash_qubit_id(QubitId::new(0));

        let logical_one =
            hash_qubit_id(QubitId::new(1));

        assert_ne!(
            logical_zero,
            logical_one
        );
    }

    #[test]
    fn program_id_hash_is_not_content_hash() {
        let first =
            hash_program_id(
                ProgramId::new(1),
            );

        let second =
            hash_program_id(
                ProgramId::new(2),
            );

        assert_ne!(
            first,
            second
        );
    }

    #[test]
    fn zero_hash_detection_is_only_a_diagnostic() {
        let hash = hash_bytes(
            HashDomain::Raw,
            b"not-zero",
        );

        assert!(!hash.is_zero());
    }

    #[test]
    fn hash_metadata_is_stable() {
        let metadata =
            HashMetadata::new(
                HashDomain::Program,
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
    fn hash_algorithm_has_stable_identifier() {
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
    fn hash_domain_names_are_stable() {
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
    fn hash_display_is_lowercase_hex() {
        let hash = hash_bytes(
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
                .chars()
                .all(|character| {
                    character.is_ascii_hexdigit()
                        && !character.is_ascii_uppercase()
                })
        );
    }

    #[test]
    fn builder_after_finalize_cannot_change_result() {
        let mut builder =
            HashBuilder::new(HashDomain::Raw);

        builder.update(b"before");

        let hash =
            builder.finalize();

        let mut second =
            HashBuilder::new(HashDomain::Raw);

        second.update(b"before");

        let expected =
            second.finalize();

        assert_eq!(
            hash,
            expected
        );
    }

    #[test]
    fn length_prefixed_encoding_distinguishes_boundaries() {
        let mut first =
            HashBuilder::new(HashDomain::Raw);

        first.update_str("ab");
        first.update_str("c");

        let first =
            first.finalize();

        let mut second =
            HashBuilder::new(HashDomain::Raw);

        second.update_str("a");
        second.update_str("bc");

        let second =
            second.finalize();

        assert_ne!(
            first,
            second
        );
    }
}