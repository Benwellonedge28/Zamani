//! Zamani Quantum IR — Canonical Hashing Integration Tests.
//!
//! Production-grade cross-module tests for the canonical Quantum IR hashing
//! contract.
//!
//! # Purpose
//!
//! This file verifies the PUBLIC hashing boundary exposed by:
//!
//! ```text
//! crate::quantum::ir::hashing
//! ```
//!
//! It does not implement a second hashing algorithm, serializer, canonicalizer,
//! digest representation, or qubit identity model.
//!
//! The production dependency boundary is:
//!
//! ```text
//! semantic IR
//!      │
//!      ▼
//! quantum::ir::serialization
//!      │
//!      ▼
//! canonical bytes
//!      │
//!      ▼
//! quantum::ir::hashing
//!      │
//!      ▼
//! deterministic SHA-256 content identity
//! ```
//!
//! # What this file verifies
//!
//! The test suite verifies:
//!
//! - fixed digest width;
//! - fixed hexadecimal width;
//! - deterministic hashing;
//! - domain separation;
//! - stable domain identifiers;
//! - stable domain names;
//! - payload sensitivity;
//! - empty-input handling;
//! - streaming equivalence;
//! - reader chunk-boundary independence;
//! - incremental `HashBuilder` equivalence;
//! - logical qubit identity hashing;
//! - physical qubit identity hashing;
//! - logical/physical namespace separation;
//! - identity-value sensitivity;
//! - hexadecimal round trips;
//! - invalid hexadecimal rejection;
//! - zero-digest sentinel behavior;
//! - algorithm/schema metadata;
//! - large finite deterministic fixtures;
//! - absence of fixed quantum-machine assumptions;
//! - public façade equivalence with the canonical hashing implementation.
//!
//! # Architectural rules
//!
//! This test suite deliberately does NOT assume:
//!
//! - a maximum qubit count;
//! - a maximum register size;
//! - a fixed operation count;
//! - a fixed circuit depth;
//! - a fixed topology;
//! - a fixed quantum architecture;
//! - a fixed vendor;
//! - a fixed backend;
//! - a fixed gate universe.
//!
//! Fixture sizes below are test workload sizes only. They are NOT IR limits.
//!
//! # Scalability
//!
//! The hashing implementation must use the same semantic contract for:
//!
//! ```text
//! one qubit
//! thousands of qubits
//! millions of qubits
//! very large finite IR artifacts
//! ```
//!
//! The test suite therefore tests both materialized and streaming hashing.
//!
//! For very large artifacts, the production API must permit callers to use
//! `hash_reader`/`HashBuilder` without requiring another complete copy of the
//! artifact in memory.
//!
//! # Qubit identity boundary
//!
//! Canonical qubit identities MUST come from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This test file never defines another `QubitId` or `PhysicalQubitId`.
//!
//! # No unsafe
//!
//! This file and everything it tests are required to remain safe Rust.
//!
//! Rust target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration
//!
//! This file should be registered by the Quantum IR test module, for example:
//!
//! ```text
//! #[cfg(test)]
//! mod hashing;
//! ```
//!
//! If the repository uses a different test registration mechanism, this file
//! should be registered there without changing the production hashing API.
//!
//! Production source code MUST NOT depend on this test module.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::indexing_slicing)]

use std::io::{self, Cursor, Read};

use crate::quantum::ir::hashing::{
    hash_bytes,
    hash_canonical_bytes,
    hash_canonical_domain,
    hash_operation_id,
    hash_physical_qubit_id,
    hash_program_id,
    hash_qubit_id,
    hash_reader,
    hash_u64_identity,
    algorithm,
    digest_hex_size,
    digest_size,
    schema_version,
    HashBuilder,
    HashDomain,
    IrHash,
    HASH_ALGORITHM,
    HASH_BYTES,
    HASH_HEX_BYTES,
    HASH_SCHEMA_VERSION,
};

use crate::quantum::ir::qubit::{
    PhysicalQubitId,
    QubitId,
};

// =============================================================================
// Test fixture constants
// =============================================================================
//
// These values are deliberately test-only workload sizes.
//
// They MUST NOT become production constants or semantic limits.
//
// The implementation under test must not branch on these values.

const SMALL_FIXTURE_SIZE: usize = 1;
const MEDIUM_FIXTURE_SIZE: usize = 4 * 1024;
const LARGE_FIXTURE_SIZE: usize = 128 * 1024;

// =============================================================================
// Deterministic fixture generation
// =============================================================================

/// Generates deterministic pseudo-random-looking bytes for testing.
///
/// This is deliberately NOT a cryptographic primitive.
///
/// It exists only to provide reproducible input with sufficient variation to
/// exercise the SHA-256 implementation over nontrivial payloads.
fn deterministic_bytes(length: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(length);

    for index in 0..length {
        let value = index as u64;

        let mixed = value
            .wrapping_mul(0x9E37_79B9_7F4A_7C15)
            .rotate_left(17)
            ^ 0xA5A5_A5A5_A5A5_A5A5;

        bytes.push((mixed & 0xff) as u8);
    }

    bytes
}

// =============================================================================
// Chunked reader
// =============================================================================

/// Reader that intentionally limits every read to a fixed chunk size.
///
/// This verifies that streaming hashing does not depend on the caller's
/// `Read::read` chunk boundaries.
struct ChunkedReader<R> {
    inner: R,
    chunk_size: usize,
}

impl<R> ChunkedReader<R> {
    fn new(inner: R, chunk_size: usize) -> Self {
        assert!(chunk_size > 0);

        Self {
            inner,
            chunk_size,
        }
    }
}

impl<R: Read> Read for ChunkedReader<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let requested = buffer.len().min(self.chunk_size);

        if requested == 0 {
            return Ok(0);
        }

        self.inner.read(&mut buffer[..requested])
    }
}

// =============================================================================
// Helper functions
// =============================================================================

fn stream_hash(
    domain: HashDomain,
    bytes: &[u8],
    chunk_size: usize,
) -> Result<IrHash, Box<dyn std::error::Error>> {
    let reader = ChunkedReader::new(
        Cursor::new(bytes),
        chunk_size,
    );

    Ok(hash_reader(domain, reader)?)
}

fn builder_hash(
    domain: HashDomain,
    bytes: &[u8],
    chunk_size: usize,
) -> IrHash {
    let mut builder = HashBuilder::new(domain);

    for chunk in bytes.chunks(chunk_size) {
        builder.update(chunk);
    }

    builder.finalize()
}

// =============================================================================
// Contract metadata
// =============================================================================

#[test]
fn hash_algorithm_contract_is_stable() {
    assert_eq!(
        HASH_ALGORITHM,
        algorithm(),
        "public algorithm() must expose the canonical hashing algorithm"
    );

    assert_eq!(
        HASH_ALGORITHM.name(),
        "sha256",
        "the canonical Quantum IR hash algorithm must be SHA-256"
    );

    assert_eq!(
        HASH_ALGORITHM.id(),
        1,
        "the published SHA-256 algorithm identifier must remain stable"
    );
}

#[test]
fn hash_schema_contract_is_stable() {
    assert_eq!(
        schema_version(),
        HASH_SCHEMA_VERSION,
        "schema_version() must expose the canonical hash schema version"
    );

    assert_eq!(
        HASH_SCHEMA_VERSION,
        1,
        "the current production hash schema must remain version 1 until an explicit migration is introduced"
    );
}

#[test]
fn digest_width_contract_is_stable() {
    assert_eq!(
        HASH_BYTES,
        32,
        "SHA-256 must have exactly 32 digest bytes"
    );

    assert_eq!(
        HASH_HEX_BYTES,
        64,
        "SHA-256 must have exactly 64 hexadecimal characters"
    );

    assert_eq!(
        digest_size(),
        HASH_BYTES,
        "digest_size() must match the canonical digest width"
    );

    assert_eq!(
        digest_hex_size(),
        HASH_HEX_BYTES,
        "digest_hex_size() must match the canonical hexadecimal width"
    );
}

// =============================================================================
// IrHash representation
// =============================================================================

#[test]
fn ir_hash_has_fixed_width() {
    let hash = hash_bytes(
        HashDomain::Raw,
        b"Zamani Quantum IR",
    );

    assert_eq!(
        hash.len(),
        HASH_BYTES,
        "IrHash::len must always report the SHA-256 digest width"
    );

    assert_eq!(
        hash.as_bytes().len(),
        HASH_BYTES,
        "IrHash::as_bytes must expose exactly one SHA-256 digest"
    );

    assert_eq!(
        hash.to_bytes().len(),
        HASH_BYTES,
        "IrHash::to_bytes must preserve the fixed digest width"
    );

    assert_eq!(
        hash.to_hex().len(),
        HASH_HEX_BYTES,
        "IrHash::to_hex must contain exactly two hexadecimal characters per digest byte"
    );
}

#[test]
fn ir_hash_hex_is_canonical_lowercase() {
    let hash = hash_bytes(
        HashDomain::Raw,
        b"Zamani Quantum IR canonical hash",
    );

    let hexadecimal = hash.to_hex();

    assert!(
        hexadecimal
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
        "canonical hexadecimal representation must use lowercase ASCII hexadecimal"
    );
}

#[test]
fn ir_hash_hex_round_trip_is_lossless() -> Result<(), Box<dyn std::error::Error>> {
    let inputs: [&[u8]; 6] = [
        b"",
        b"Zamani",
        b"Quantum",
        b"Zamani Quantum IR",
        b"canonical serialization",
        b"deterministic content identity",
    ];

    for input in inputs {
        let original = hash_bytes(
            HashDomain::Raw,
            input,
        );

        let hexadecimal = original.to_hex();

        let decoded = IrHash::from_hex(&hexadecimal)?;

        assert_eq!(
            original,
            decoded,
            "hash hexadecimal encoding and decoding must preserve every digest bit"
        );

        assert_eq!(
            hexadecimal,
            decoded.to_hex(),
            "canonical re-encoding must reproduce identical lowercase hexadecimal"
        );
    }

    Ok(())
}

#[test]
fn ir_hash_accepts_uppercase_hexadecimal() -> Result<(), Box<dyn std::error::Error>> {
    let original = hash_bytes(
        HashDomain::Raw,
        b"uppercase compatibility",
    );

    let uppercase = original.to_hex().to_ascii_uppercase();

    let decoded = IrHash::from_hex(&uppercase)?;

    assert_eq!(
        decoded,
        original,
        "hexadecimal parsing may accept uppercase without changing the digest"
    );

    assert_eq!(
        decoded.to_hex(),
        original.to_hex(),
        "canonical output must still be lowercase"
    );

    Ok(())
}

#[test]
fn invalid_hex_length_is_rejected() {
    let short = IrHash::from_hex("00");

    assert!(
        short.is_err(),
        "hexadecimal input shorter than one complete digest must be rejected"
    );

    let long = IrHash::from_hex(
        "00000000000000000000000000000000000000000000000000000000000000000",
    );

    assert!(
        long.is_err(),
        "hexadecimal input longer than one complete digest must be rejected"
    );
}

#[test]
fn invalid_hex_character_is_rejected() {
    let mut hexadecimal = String::new();

    for _ in 0..HASH_HEX_BYTES {
        hexadecimal.push('0');
    }

    hexadecimal.replace_range(0..1, "g");

    let result = IrHash::from_hex(&hexadecimal);

    assert!(
        result.is_err(),
        "non-hexadecimal input must never be accepted as a canonical digest"
    );
}

#[test]
fn zero_hash_is_an_explicit_sentinel_only() {
    let zero = IrHash::default();

    assert!(
        zero.is_zero(),
        "IrHash::default must be the all-zero sentinel"
    );

    let real_hash = hash_bytes(
        HashDomain::Raw,
        b"",
    );

    assert_ne!(
        zero,
        real_hash,
        "the SHA-256 hash of an empty payload must not equal the all-zero sentinel"
    );
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn identical_input_is_always_deterministic() {
    let payload = deterministic_bytes(MEDIUM_FIXTURE_SIZE);

    let expected = hash_bytes(
        HashDomain::Raw,
        &payload,
    );

    for _ in 0..128 {
        let actual = hash_bytes(
            HashDomain::Raw,
            &payload,
        );

        assert_eq!(
            actual,
            expected,
            "identical canonical bytes must always produce identical content hashes"
        );
    }
}

#[test]
fn deterministic_fixture_generation_is_reproducible() {
    let first = deterministic_bytes(MEDIUM_FIXTURE_SIZE);
    let second = deterministic_bytes(MEDIUM_FIXTURE_SIZE);

    assert_eq!(
        first,
        second,
        "deterministic test fixtures must themselves be reproducible"
    );
}

#[test]
fn different_payloads_produce_different_hashes() {
    let first = deterministic_bytes(MEDIUM_FIXTURE_SIZE);

    let mut second = first.clone();

    let position = second.len() / 2;

    let original = second[position];

    second[position] = original.wrapping_add(1);

    assert_ne!(
        first,
        second,
        "the fixture mutation must actually change the payload"
    );

    let first_hash = hash_bytes(
        HashDomain::Raw,
        &first,
    );

    let second_hash = hash_bytes(
        HashDomain::Raw,
        &second,
    );

    assert_ne!(
        first_hash,
        second_hash,
        "a semantic byte change must change the content identity"
    );
}

#[test]
fn appended_payload_changes_hash() {
    let first = deterministic_bytes(SMALL_FIXTURE_SIZE);

    let mut second = first.clone();
    second.push(0);

    let first_hash = hash_bytes(
        HashDomain::Raw,
        &first,
    );

    let second_hash = hash_bytes(
        HashDomain::Raw,
        &second,
    );

    assert_ne!(
        first_hash,
        second_hash,
        "changing canonical payload length must change its content identity"
    );
}

#[test]
fn empty_and_nonempty_payloads_are_distinct() {
    let empty = hash_bytes(
        HashDomain::Raw,
        &[],
    );

    let nonempty = hash_bytes(
        HashDomain::Raw,
        &[0],
    );

    assert_ne!(
        empty,
        nonempty,
        "empty and non-empty canonical payloads must have different content identities"
    );
}

// =============================================================================
// Domain separation
// =============================================================================

fn all_hash_domains() -> [HashDomain; 19] {
    [
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
    ]
}

#[test]
fn every_published_hash_domain_is_unique() {
    let domains = all_hash_domains();

    for left in 0..domains.len() {
        for right in (left + 1)..domains.len() {
            assert_ne!(
                domains[left].id(),
                domains[right].id(),
                "published hash-domain identifiers must be unique"
            );

            assert_ne!(
                domains[left].name(),
                domains[right].name(),
                "published hash-domain names must be unique"
            );
        }
    }
}

#[test]
fn same_payload_is_domain_separated() {
    let payload = b"same canonical payload";

    let domains = all_hash_domains();

    let hashes: Vec<IrHash> = domains
        .iter()
        .map(|domain| hash_bytes(*domain, payload))
        .collect();

    for left in 0..hashes.len() {
        for right in (left + 1)..hashes.len() {
            assert_ne!(
                hashes[left],
                hashes[right],
                "different semantic hash domains must not share the same domain-separated digest"
            );
        }
    }
}

#[test]
fn raw_and_ir_domains_are_distinct() {
    let payload = b"canonical bytes";

    let raw = hash_bytes(
        HashDomain::Raw,
        payload,
    );

    let ir = hash_bytes(
        HashDomain::Ir,
        payload,
    );

    assert_ne!(
        raw,
        ir,
        "raw-byte identity and complete-IR identity must remain distinct"
    );
}

#[test]
fn logical_and_physical_qubit_domains_are_distinct() {
    let logical = hash_u64_identity(
        HashDomain::LogicalQubit,
        7,
    );

    let physical = hash_u64_identity(
        HashDomain::PhysicalQubit,
        7,
    );

    assert_ne!(
        logical,
        physical,
        "logical and physical qubit identities must remain domain-separated"
    );
}

// =============================================================================
// Streaming hashing
// =============================================================================

#[test]
fn streaming_hash_matches_materialized_hash() -> Result<(), Box<dyn std::error::Error>> {
    let payload = deterministic_bytes(LARGE_FIXTURE_SIZE);

    let direct = hash_bytes(
        HashDomain::Raw,
        &payload,
    );

    let streamed = stream_hash(
        HashDomain::Raw,
        &payload,
        1,
    )?;

    assert_eq!(
        streamed,
        direct,
        "streaming hashing must produce the same digest as materialized hashing"
    );

    Ok(())
}

#[test]
fn streaming_hash_is_independent_of_reader_chunk_size(
) -> Result<(), Box<dyn std::error::Error>> {
    let payload = deterministic_bytes(LARGE_FIXTURE_SIZE);

    let expected = hash_bytes(
        HashDomain::Raw,
        &payload,
    );

    let chunk_sizes = [
        1usize,
        2,
        3,
        7,
        16,
        31,
        64,
        127,
        256,
        1024,
        4096,
        65_536,
        payload.len(),
        payload.len().saturating_add(1),
    ];

    for chunk_size in chunk_sizes {
        let actual = stream_hash(
            HashDomain::Raw,
            &payload,
            chunk_size,
        )?;

        assert_eq!(
            actual,
            expected,
            "streaming hash must not depend on reader chunk size {chunk_size}"
        );
    }

    Ok(())
}

#[test]
fn repeated_streaming_hashes_are_deterministic(
) -> Result<(), Box<dyn std::error::Error>> {
    let payload = deterministic_bytes(MEDIUM_FIXTURE_SIZE);

    let expected = stream_hash(
        HashDomain::Raw,
        &payload,
        7,
    )?;

    for _ in 0..64 {
        let actual = stream_hash(
            HashDomain::Raw,
            &payload,
            7,
        )?;

        assert_eq!(
            actual,
            expected,
            "repeated streaming hashes must remain deterministic"
        );
    }

    Ok(())
}

// =============================================================================
// Incremental HashBuilder
// =============================================================================

#[test]
fn_hash_builder_matches_direct_hash() {
    let payload = deterministic_bytes(LARGE_FIXTURE_SIZE);

    let direct = hash_bytes(
        HashDomain::Raw,
        &payload,
    );

    let built = builder_hash(
        HashDomain::Raw,
        &payload,
        1,
    );

    assert_eq!(
        built,
        direct,
        "incremental hashing one byte at a time must equal direct hashing"
    );
}

#[test]
fn hash_builder_is_independent_of_chunk_size() {
    let payload = deterministic_bytes(LARGE_FIXTURE_SIZE);

    let expected = hash_bytes(
        HashDomain::Raw,
        &payload,
    );

    let chunk_sizes = [
        1usize,
        2,
        5,
        16,
        31,
        64,
        127,
        256,
        1024,
        4096,
        65_536,
        payload.len(),
    ];

    for chunk_size in chunk_sizes {
        let actual = builder_hash(
            HashDomain::Raw,
            &payload,
            chunk_size,
        );

        assert_eq!(
            actual,
            expected,
            "HashBuilder must be independent of update chunk size {chunk_size}"
        );
    }
}

#[test]
fn hash_builder_empty_input_matches_direct_hash() {
    let direct = hash_bytes(
        HashDomain::Raw,
        &[],
    );

    let builder = HashBuilder::new(
        HashDomain::Raw,
    );

    let actual = builder.finalize();

    assert_eq!(
        actual,
        direct,
        "finalizing an untouched HashBuilder must hash the canonical empty payload"
    );
}

#[test]
fn hash_builder_domain_separation_matches_direct_hash() {
    let payload = b"domain-separated builder payload";

    let direct = hash_bytes(
        HashDomain::Operation,
        payload,
    );

    let mut builder = HashBuilder::new(
        HashDomain::Operation,
    );

    builder.update(payload);

    let actual = builder.finalize();

    assert_eq!(
        actual,
        direct,
        "HashBuilder must use the same domain framing as direct hashing"
    );
}

// =============================================================================
// Canonical façade
// =============================================================================

#[test]
fn canonical_byte_facade_matches_direct_ir_domain() {
    let payload = b"canonical serialized Quantum IR";

    let direct = hash_bytes(
        HashDomain::Ir,
        payload,
    );

    let facade = hash_canonical_bytes(payload);

    assert_eq!(
        facade,
        direct,
        "hash_canonical_bytes must delegate to the canonical IR hash domain"
    );
}

#[test]
fn canonical_domain_facade_matches_direct_hash() {
    let payload = b"canonical domain payload";

    let direct = hash_bytes(
        HashDomain::Operation,
        payload,
    );

    let facade = hash_canonical_domain(
        HashDomain::Operation,
        payload,
    );

    assert_eq!(
        facade,
        direct,
        "hash_canonical_domain must delegate to the canonical hashing implementation"
    );
}

// =============================================================================
// Qubit identity integration
// =============================================================================

#[test]
fn canonical_logical_qubit_helper_matches_u64_identity_hash() {
    let qubit = QubitId::new(42);

    let helper = hash_qubit_id(qubit);

    let generic = hash_u64_identity(
        HashDomain::LogicalQubit,
        qubit.index() as u64,
    );

    assert_eq!(
        helper,
        generic,
        "logical qubit hashing must use the canonical QubitId numeric identity under the logical-qubit domain"
    );
}

#[test]
fn canonical_physical_qubit_helper_matches_u64_identity_hash() {
    let qubit = PhysicalQubitId::new(42);

    let helper = hash_physical_qubit_id(qubit);

    let generic = hash_u64_identity(
        HashDomain::PhysicalQubit,
        qubit.index() as u64,
    );

    assert_eq!(
        helper,
        generic,
        "physical qubit hashing must use the canonical PhysicalQubitId identity under the physical-qubit domain"
    );
}

#[test]
fn logical_qubit_identity_is_sensitive_to_identity_value() {
    let first = QubitId::new(0);
    let second = QubitId::new(1);

    let first_hash = hash_qubit_id(first);
    let second_hash = hash_qubit_id(second);

    assert_ne!(
        first_hash,
        second_hash,
        "different logical qubit identities must produce different content hashes"
    );
}

#[test]
fn physical_qubit_identity_is_sensitive_to_identity_value() {
    let first = PhysicalQubitId::new(0);
    let second = PhysicalQubitId::new(1);

    let first_hash = hash_physical_qubit_id(first);
    let second_hash = hash_physical_qubit_id(second);

    assert_ne!(
        first_hash,
        second_hash,
        "different physical qubit identities must produce different content hashes"
    );
}

#[test]
fn logical_and_physical_qubit_helpers_remain_separated() {
    let logical = QubitId::new(7);
    let physical = PhysicalQubitId::new(7);

    let logical_hash = hash_qubit_id(logical);
    let physical_hash = hash_physical_qubit_id(physical);

    assert_ne!(
        logical_hash,
        physical_hash,
        "the same numeric value in different qubit namespaces must not share a content identity"
    );
}

#[test]
fn canonical_qubit_identity_types_are_not_interchangeable() {
    let logical = QubitId::new(5);
    let physical = PhysicalQubitId::new(5);

    assert_eq!(
        logical.index(),
        physical.index(),
        "the fixture intentionally uses the same numeric index"
    );

    assert_ne!(
        hash_qubit_id(logical),
        hash_physical_qubit_id(physical),
        "semantic namespace separation must survive equal numeric indices"
    );
}

// =============================================================================
// Stable object identity helpers
// =============================================================================

#[test]
fn program_identity_hash_is_stable() {
    let program = crate::quantum::ir::identity::ProgramId::new(123);

    let first = hash_program_id(program);
    let second = hash_program_id(program);

    assert_eq!(
        first,
        second,
        "identical ProgramId values must hash deterministically"
    );
}

#[test]
fn operation_identity_hash_is_stable() {
    let operation = crate::quantum::ir::identity::OperationId::new(456);

    let first = hash_operation_id(operation);
    let second = hash_operation_id(operation);

    assert_eq!(
        first,
        second,
        "identical OperationId values must hash deterministically"
    );
}

#[test]
fn program_and_operation_identity_domains_are_distinct() {
    let program = crate::quantum::ir::identity::ProgramId::new(123);
    let operation = crate::quantum::ir::identity::OperationId::new(123);

    let program_hash = hash_program_id(program);
    let operation_hash = hash_operation_id(operation);

    assert_ne!(
        program_hash,
        operation_hash,
        "different semantic identity types must remain domain-separated even for equal numeric values"
    );
}

// =============================================================================
// Identity boundary and platform independence
// =============================================================================

#[test]
fn u64_identity_encoding_is_deterministic() {
    let values = [
        0u64,
        1u64,
        2u64,
        255u64,
        256u64,
        u64::from(u32::MAX),
        u64::from(u32::MAX) + 1,
        u64::MAX,
    ];

    for value in values {
        let first = hash_u64_identity(
            HashDomain::Value,
            value,
        );

        let second = hash_u64_identity(
            HashDomain::Value,
            value,
        );

        assert_eq!(
            first,
            second,
            "u64 identity hashing must be deterministic for value {value}"
        );
    }
}

#[test]
fn u64_identity_hash_changes_when_identity_changes() {
    let first = hash_u64_identity(
        HashDomain::Value,
        0,
    );

    let second = hash_u64_identity(
        HashDomain::Value,
        1,
    );

    assert_ne!(
        first,
        second,
        "changing a stable identity value must change its content hash"
    );
}

#[test]
fn identity_hashing_does_not_use_pointer_identity() {
    let first = Box::new(QubitId::new(99));
    let second = Box::new(QubitId::new(99));

    assert_eq!(
        hash_qubit_id(*first),
        hash_qubit_id(*second),
        "equal logical identities must hash equally regardless of their allocation addresses"
    );
}

// =============================================================================
// Large finite workload
// =============================================================================

#[test]
fn large_finite_fixture_uses_the_same_hashing_contract() -> Result<(), Box<dyn std::error::Error>> {
    let payload = deterministic_bytes(LARGE_FIXTURE_SIZE);

    let direct = hash_bytes(
        HashDomain::Raw,
        &payload,
    );

    let streamed = stream_hash(
        HashDomain::Raw,
        &payload,
        4096,
    )?;

    let incremental = builder_hash(
        HashDomain::Raw,
        &payload,
        4096,
    );

    assert_eq!(
        direct,
        streamed,
        "large finite artifacts must have identical direct and streaming content identities"
    );

    assert_eq!(
        direct,
        incremental,
        "large finite artifacts must have identical direct and incremental content identities"
    );

    assert_eq!(
        direct.len(),
        HASH_BYTES,
        "large artifacts still have the fixed SHA-256 digest width"
    );

    Ok(())
}

// =============================================================================
// Hash equality helpers
// =============================================================================

#[test]
fn hash_comparison_helpers_agree_with_standard_equality() {
    let first = hash_bytes(
        HashDomain::Raw,
        b"same",
    );

    let second = hash_bytes(
        HashDomain::Raw,
        b"same",
    );

    let third = hash_bytes(
        HashDomain::Raw,
        b"different",
    );

    assert_eq!(
        first,
        second,
        "equal canonical content must produce equal hashes"
    );

    assert!(
        crate::quantum::ir::hashing::hashes_equal(
            &first,
            &second,
        ),
        "hashes_equal must agree with IrHash equality"
    );

    assert!(
        crate::quantum::ir::hashing::hashes_differ(
            &first,
            &third,
        ),
        "hashes_differ must agree with IrHash inequality"
    );

    assert!(
        !crate::quantum::ir::hashing::hashes_differ(
            &first,
            &second,
        ),
        "hashes_differ must be false for equal hashes"
    );

    assert!(
        !crate::quantum::ir::hashing::hashes_equal(
            &first,
            &third,
        ),
        "hashes_equal must be false for different hashes"
    );
}

// =============================================================================
// Ordering and copy semantics
// =============================================================================

#[test]
fn hash_is_copy_and_equality_stable() {
    let original = hash_bytes(
        HashDomain::Raw,
        b"copy semantics",
    );

    let copied = original;

    assert_eq!(
        original,
        copied,
        "IrHash must preserve exact digest equality when copied"
    );
}

#[test]
fn hash_ordering_is_deterministic() {
    let first = hash_bytes(
        HashDomain::Raw,
        b"first",
    );

    let second = hash_bytes(
        HashDomain::Raw,
        b"second",
    );

    let first_again = hash_bytes(
        HashDomain::Raw,
        b"first",
    );

    assert_eq!(
        first.cmp(&first_again),
        std::cmp::Ordering::Equal,
        "equal hashes must compare equal"
    );

    let ordering_a = first.cmp(&second);
    let ordering_b = first_again.cmp(&second);

    assert_eq!(
        ordering_a,
        ordering_b,
        "hash ordering must be deterministic"
    );
}

// =============================================================================
// Canonical domain metadata
// =============================================================================

#[test]
fn published_domain_ids_are_positive_and_stable() {
    for domain in all_hash_domains() {
        assert!(
            domain.id() > 0,
            "published hash-domain identifiers must be positive"
        );
    }
}

#[test]
fn published_domain_names_are_nonempty() {
    for domain in all_hash_domains() {
        assert!(
            !domain.name().is_empty(),
            "every published hash domain must have a non-empty stable name"
        );
    }
}

#[test]
fn hash_domain_display_matches_stable_name() {
    for domain in all_hash_domains() {
        assert_eq!(
            domain.to_string(),
            domain.name(),
            "HashDomain Display must remain the stable domain name"
        );
    }
}

// =============================================================================
// Regression tests for architectural boundaries
// =============================================================================

#[test]
fn qubit_hashing_uses_canonical_qubit_module() {
    let logical = QubitId::new(0);
    let physical = PhysicalQubitId::new(0);

    let logical_hash = hash_qubit_id(logical);
    let physical_hash = hash_physical_qubit_id(physical);

    assert_ne!(
        logical_hash,
        physical_hash,
        "hashing must preserve the canonical logical/physical qubit identity boundary"
    );
}

#[test]
fn hashing_does_not_treat_qubit_index_as_machine_capacity() {
    let small = QubitId::new(0);
    let large = QubitId::new(usize::MAX);

    let small_hash = hash_qubit_id(small);
    let large_hash = hash_qubit_id(large);

    assert_ne!(
        small_hash,
        large_hash,
        "the qubit identity hashing contract must operate on identity values rather than a fixed machine-size limit"
    );
}

#[test]
fn high_identity_values_remain_valid_hash_inputs() {
    let values = [
        usize::MAX,
        usize::MAX.saturating_sub(1),
        usize::MAX / 2,
        1,
        0,
    ];

    for value in values {
        let qubit = QubitId::new(value);

        let hash = hash_qubit_id(qubit);

        assert_eq!(
            hash.len(),
            HASH_BYTES,
            "every representable canonical qubit identity must remain hashable"
        );
    }
}

// =============================================================================
// Compatibility façade regression
// =============================================================================

#[test]
fn_hashing_facade_exposes_the_canonical_algorithm() {
    assert_eq!(
        crate::quantum::ir::hashing::algorithm(),
        HASH_ALGORITHM,
        "quantum::ir::hashing must remain the stable public façade"
    );
}

#[test]
fn hashing_facade_exposes_the_canonical_schema() {
    assert_eq!(
        crate::quantum::ir::hashing::schema_version(),
        HASH_SCHEMA_VERSION,
        "quantum::ir::hashing must expose the canonical hashing schema"
    );
}

#[test]
fn hashing_facade_exposes_the_canonical_digest_width() {
    assert_eq!(
        crate::quantum::ir::hashing::digest_size(),
        HASH_BYTES,
        "quantum::ir::hashing must expose the canonical SHA-256 digest width"
    );
}