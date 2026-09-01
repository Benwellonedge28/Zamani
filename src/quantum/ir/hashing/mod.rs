//! Zamani Quantum IR — Hashing subsystem façade.
//!
//! This module is the stable public boundary for canonical Quantum IR hashing.
//! The hashing implementation remains deliberately separated from the semantic
//! IR modules and from target-specific quantum hardware.
//!
//! # Architectural contract
//!
//! ```text
//! Zamani semantic IR
//!        │
//!        ▼
//! canonical serialization
//!        │
//!        ▼
//! quantum::ir::hashing
//!        │
//!        ▼
//! deterministic cryptographic content identity
//! ```
//!
//! The hashing layer MUST NOT contain or depend on:
//!
//! - quantum hardware implementations;
//! - backend execution;
//! - routing algorithms;
//! - schedulers;
//! - optimizers;
//! - simulators;
//! - QEC decoders;
//! - frontend syntax;
//! - network transports;
//! - credentials or authentication state.
//!
//! It hashes already-defined canonical data. It does not define quantum
//! semantics.
//!
//! # Single ownership rule
//!
//! `quantum::ir::hash` is the compatibility implementation currently present in
//! the repository. This module deliberately re-exports that implementation
//! instead of copying `IrHash`, `HashBuilder`, SHA-256 construction, or qubit
//! identity hashing.
//!
//! This is important because there must be exactly one authority for each of:
//!
//! - `IrHash`;
//! - `HashBuilder`;
//! - `HashDomain`;
//! - hash schema constants;
//! - canonical hash framing;
//! - logical qubit hashing;
//! - physical qubit hashing.
//!
//! Future internal files such as `canonical_hash.rs` and `fingerprints.rs` may
//! be introduced without changing this public boundary. They must delegate to
//! the same canonical implementation rather than creating a second hashing
//! contract.
//!
//! # Canonical identity versus object identity
//!
//! A content hash answers:
//!
//! > Are these canonical semantic bytes identical?
//!
//! It is NOT:
//!
//! - a random identifier;
//! - a database primary key;
//! - a hardware identifier;
//! - a signature;
//! - an encryption key;
//! - an authorization credential.
//!
//! `ProgramId`, `OperationId`, `QubitId`, and `PhysicalQubitId` remain identity
//! concepts owned by their respective IR modules. Their hash helpers hash the
//! identity value; they do not replace the identity type.
//!
//! # Logical and physical qubits
//!
//! The authoritative quantum identity types are always:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! New code MUST NOT introduce `hashing::QubitId`, `hashing::PhysicalQubitId`,
//! or another local qubit representation.
//!
//! # Determinism
//!
//! Hashing is valid as persistent content identity only when its input is the
//! canonical representation. Callers therefore use:
//!
//! ```text
//! semantic object
//!     ↓
//! quantum::ir::serialization
//!     ↓
//! canonical bytes
//!     ↓
//! quantum::ir::hashing
//! ```
//!
//! The hashing layer must never use Rust's process-randomized `Hash`/`Hasher`
//! implementations as the persistent IR hash contract, and must never include
//! pointer addresses, process IDs, thread IDs, allocator state, wall-clock
//! time, or other nondeterministic execution state.
//!
//! # Scalability
//!
//! There is intentionally NO semantic maximum for:
//!
//! - qubit count;
//! - register size;
//! - operation count;
//! - circuit depth;
//! - number of regions;
//! - number of parameters;
//! - number of resources.
//!
//! A one-qubit and a very large finite IR document use the same hashing model.
//! Practical limits come only from the actual representation, available
//! resources, and explicit security/resource policies elsewhere in the IR.
//!
//! For large canonical byte streams, prefer [`HashBuilder`] or [`hash_reader`]
//! so the entire document does not have to be materialized in an additional
//! allocation.
//!
//! "Infinite" is not a literal runtime value: every concrete artifact is
//! finite and bounded by available representation and execution resources.
//! The architecture itself contains no fixed quantum-machine size ceiling.
//!
//! # Serialization integration
//!
//! `quantum::ir::serialization` is the canonical encoding authority.
//! `hash_ir` hashes an object through that serialization contract; it does not
//! invent a second serialization format.
//!
//! Therefore the integration rule is:
//!
//! ```text
//! IrEncode
//!     │
//!     ▼
//! serialize / serialize_with_version
//!     │
//!     ▼
//! SerializedIr
//!     │
//!     ▼
//! hash_serialized / hash_ir
//! ```
//!
//! If serialization changes semantically, the canonical serialization and IR
//! version contracts must be updated there. This façade must not compensate by
//! implementing a second byte encoding.
//!
//! # Streaming integration
//!
//! `HashBuilder` provides incremental hashing over already-canonical bytes.
//! `hash_reader` provides bounded streaming input. These APIs are intended for
//! large artifacts and distributed/content-addressed pipelines.
//!
//! A stream must already be canonical for the selected [`HashDomain`]. This
//! module does not attempt to canonicalize arbitrary byte streams.
//!
//! # Domain separation
//!
//! Hashes for different semantic object classes use [`HashDomain`]. A caller
//! must select the domain matching the semantic object being hashed. The
//! complete serialized IR document uses the IR domain through
//! [`hash_serialized`].
//!
//! Domain identifiers are part of the persistent hash contract. They therefore
//! must not be silently renumbered or reused for another meaning.
//!
//! # Versioning
//!
//! There are deliberately separate version concepts:
//!
//! ```text
//! Zamani language version
//!         ≠
//! Quantum IR semantic version
//!         ≠
//! serialization format version
//!         ≠
//! hashing schema version
//!         ≠
//! compiler version
//!         ≠
//! hardware version
//! ```
//!
//! `HASH_SCHEMA_VERSION` belongs to the hashing implementation. `IrVersion`
//! belongs to the canonical IR identity/version contract. Neither is redefined
//! here.
//!
//! A breaking change to hash framing, domain separation, digest interpretation,
//! or canonical hash semantics MUST result in an explicit hashing-schema
//! compatibility decision. Existing hashes must never silently acquire a new
//! meaning.
//!
//! # Security
//!
//! SHA-256 content hashing provides integrity-oriented content identity, not
//! authenticity. A hash alone does not prove who produced an artifact.
//!
//! Authenticity belongs to a signing layer:
//!
//! ```text
//! canonical IR bytes
//!       ↓
//! SHA-256 content hash
//!       ↓
//! signing subsystem
//!       ↓
//! digital signature
//! ```
//!
//! This module contains no private keys, credentials, authentication state, or
//! authorization logic.
//!
//! # Rust contract
//!
//! Targeted Rust versions:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust only;
//! - no nightly features;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced
//! for this module.
//!
//! # Integration contract for every future hashing file
//!
//! Any future file added below `quantum::ir::hashing` MUST obey all of these
//! rules:
//!
//! 1. It MUST use the canonical hash implementation exposed here.
//! 2. It MUST NOT define another digest type for the same semantic hash.
//! 3. It MUST NOT define another SHA-256 framing contract.
//! 4. It MUST NOT duplicate `QubitId` or `PhysicalQubitId`.
//! 5. It MUST use `quantum::ir::qubit` for qubit identities.
//! 6. It MUST NOT depend on hardware or backend modules.
//! 7. It MUST NOT introduce a fixed qubit/machine-size constant.
//! 8. It MUST preserve deterministic ordering supplied by canonical
//!    serialization.
//! 9. It MUST document whether metadata is semantic or non-semantic before
//!    including it in a persistent hash.
//! 10. It MUST preserve compatibility with existing hash consumers unless an
//!     explicit schema/version migration is introduced.
//!
//! # Why this façade exists
//!
//! The repository currently has the hashing implementation at
//! `quantum::ir::hash`. The production architecture calls for a maintained
//! `quantum::ir::hashing/` subsystem. Introducing this façade first gives the
//! repository a stable package boundary without requiring every downstream
//! caller to be rewritten at the same time.
//!
//! Once the implementation is split into dedicated files, the public exports
//! below remain the compatibility boundary. Callers should therefore prefer:
//!
//! ```text
//! quantum::ir::hashing::IrHash
//! quantum::ir::hashing::hash_ir
//! quantum::ir::hashing::hash_serialized
//! ```
//!
//! rather than depending on internal hashing implementation paths.
//!
//! # Parent-module integration
//!
//! The parent `quantum::ir::mod.rs` should expose this subsystem with:
//!
//! ```text
//! pub mod hashing;
//! ```
//!
//! It may retain `pub mod hash;` temporarily for source compatibility with
//! existing repository code. The two modules MUST refer to the same hashing
//! implementation and MUST NOT evolve into independent hash contracts.

#![forbid(unsafe_code)]

// -----------------------------------------------------------------------------
// Canonical implementation re-export
// -----------------------------------------------------------------------------
//
// `hash.rs` is currently the implementation authority. Re-exporting it here
// prevents a duplicate implementation and allows the new directory boundary
// to be integrated independently from the rest of the IR migration.
pub use super::hash::{
    hash_bytes,
    hash_ir,
    hash_ir_with_version,
    hash_operation_id,
    hash_physical_qubit_id,
    hash_program_id,
    hash_qubit_id,
    hash_reader,
    hash_serialized,
    hash_u64_identity,
    hashes_differ,
    hashes_equal,
    CircuitHash,
    HASH_ALGORITHM,
    HASH_BYTES,
    HASH_DOMAIN_PREFIX,
    HASH_HEX_BYTES,
    HASH_SCHEMA_VERSION,
    HashAlgorithm,
    HashBuilder,
    HashDomain,
    HashError,
    IrHash,
    OperationHash,
    PhysicalQubitHash,
    ProgramHash,
    QubitHash,
};

// -----------------------------------------------------------------------------
// Stable semantic aliases
// -----------------------------------------------------------------------------

/// Stable name for a canonical Quantum IR content hash.
///
/// This is intentionally an alias rather than a second hash representation.
pub type ContentHash = IrHash;

/// Stable name for a canonical artifact hash.
///
/// This is intentionally an alias rather than a second hash representation.
pub type ArtifactHash = IrHash;

/// Stable name for the canonical hash result used by compilation artifacts.
///
/// This alias allows callers to describe the role of the digest without
/// creating another incompatible type.
pub type CanonicalHash = IrHash;

// -----------------------------------------------------------------------------
// Contract helpers
// -----------------------------------------------------------------------------

/// Returns the hashing schema version used by the canonical implementation.
#[must_use]
pub const fn schema_version() -> u16 {
    HASH_SCHEMA_VERSION
}

/// Returns the digest size of the canonical hashing algorithm in bytes.
#[must_use]
pub const fn digest_size() -> usize {
    HASH_BYTES
}

/// Returns the digest size of the canonical hashing algorithm in hexadecimal
/// characters.
#[must_use]
pub const fn digest_hex_size() -> usize {
    HASH_HEX_BYTES
}

/// Returns the canonical hashing algorithm identifier.
#[must_use]
pub const fn algorithm() -> HashAlgorithm {
    HASH_ALGORITHM
}

/// Hashes canonical bytes as a complete IR artifact.
///
/// This is an explicit semantic alias for [`hash_serialized`] intended for
/// content-addressed artifact stores. The input MUST be the exact canonical
/// serialization produced by `quantum::ir::serialization`.
#[must_use]
pub fn hash_canonical_bytes(bytes: &[u8]) -> IrHash {
    hash_bytes(HashDomain::Ir, bytes)
}

/// Hashes canonical bytes under an explicit semantic domain.
///
/// This function does not sort, parse, normalize, or otherwise transform the
/// supplied bytes. Canonicalization is owned by
/// `quantum::ir::serialization`.
#[must_use]
pub fn hash_canonical_domain(
    domain: HashDomain,
    bytes: &[u8],
) -> IrHash {
    hash_bytes(domain, bytes)
}

// -----------------------------------------------------------------------------
// Tests: public-boundary invariants only
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn digest_contract_is_stable() {
        assert_eq!(digest_size(), HASH_BYTES);
        assert_eq!(digest_hex_size(), HASH_HEX_BYTES);
        assert_eq!(algorithm(), HASH_ALGORITHM);
        assert_eq!(schema_version(), HASH_SCHEMA_VERSION);
    }

    #[test]
    fn identical_canonical_bytes_have_identical_hashes() {
        let bytes = b"zamani-canonical-ir";

        assert_eq!(
            hash_canonical_bytes(bytes),
            hash_canonical_bytes(bytes)
        );
    }

    #[test]
    fn different_domains_are_separated() {
        let bytes = b"same-canonical-bytes";

        assert_ne!(
            hash_canonical_domain(HashDomain::Ir, bytes),
            hash_canonical_domain(HashDomain::Program, bytes),
        );
    }

    #[test]
    fn streaming_hash_matches_materialized_hash() {
        let bytes = b"streamed-zamani-ir-content";

        let expected = hash_canonical_bytes(bytes);

        let mut reader = Cursor::new(bytes.as_slice());

        let actual = hash_reader(
            HashDomain::Ir,
            &mut reader,
        )
        .expect("in-memory reader cannot fail");

        assert_eq!(actual, expected);
    }

    #[test]
    fn hash_is_fixed_width() {
        let hash = hash_canonical_bytes(b"fixed-width");

        assert_eq!(
            hash.as_bytes().len(),
            HASH_BYTES
        );

        assert_eq!(
            hash.to_hex().len(),
            HASH_HEX_BYTES
        );
    }

    #[test]
    fn logical_and_physical_domains_are_distinct() {
        let bytes = b"1";

        assert_ne!(
            hash_canonical_domain(
                HashDomain::LogicalQubit,
                bytes,
            ),
            hash_canonical_domain(
                HashDomain::PhysicalQubit,
                bytes,
            ),
        );
    }
}