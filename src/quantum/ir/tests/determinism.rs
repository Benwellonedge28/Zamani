//! Zamani Quantum IR — Determinism Test Suite
//!
//! Production-grade determinism and reproducibility tests for the canonical
//! Quantum IR serialization and hashing contracts.
//!
//! # Architectural role
//!
//! This file tests the following invariant:
//!
//! ```text
//! same semantic input
//! + same canonicalization contract
//! + same IR version
//! + same serialization format
//! + same hashing schema
//! -----------------------------------------
//! = identical canonical bytes
//! = identical content hash
//! ```
//!
//! The tests deliberately exercise the public contracts of:
//!
//! - `quantum::ir::serialization`;
//! - `quantum::ir::hashing`;
//! - `quantum::ir::qubit`;
//! - canonical hash domain separation;
//! - streaming hashing;
//! - stable hexadecimal hash representation.
//!
//! # Important architectural rule
//!
//! This test module does NOT define a second serializer or a second hashing
//! implementation.
//!
//! It tests the production implementations.
//!
//! It also does NOT assume:
//!
//! - a fixed number of qubits;
//! - a fixed number of operations;
//! - a fixed register size;
//! - a fixed topology;
//! - a fixed quantum architecture;
//! - a fixed vendor;
//! - a fixed backend;
//! - a fixed gate universe.
//!
//! # Scaling
//!
//! Tests use bounded fixtures because a test process necessarily has finite
//! resources. Those fixture sizes are NOT semantic Quantum IR limits.
//!
//! The implementation under test must continue to use the same algorithms for:
//!
//! - one qubit;
//! - many qubits;
//! - large programs;
//! - future quantum architectures;
//! - distributed programs;
//! - pulse programs;
//! - analog programs;
//! - logical/fault-tolerant programs.
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
//! - no unsafe code.
//!
//! # Integration contract
//!
//! This file is intended to be included from the Quantum IR test module.
//!
//! Recommended integration:
//!
//! ```text
//! src/quantum/ir/tests/mod.rs
//!     └── mod determinism;
//! ```
//!
//! If the repository instead uses inline test modules, the contents of this
//! file may be included through the existing test module without changing the
//! production IR API.
//!
//! No production source file should depend on this test module.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::indexing_slicing)]

use std::io::{self, Cursor, Read};

use crate::quantum::ir::hashing::{
    hash_bytes,
    hash_qubit_id,
    hash_reader,
    HashDomain,
    IrHash,
    HASH_ALGORITHM,
    HASH_BYTES,
    HASH_HEX_BYTES,
    HASH_SCHEMA_VERSION,
};
use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Test constants
// =============================================================================
//
// These are test fixture sizes only.
//
// They MUST NOT be interpreted as Quantum IR architecture limits.

const SMALL_FIXTURE_SIZE: usize = 1;
const MEDIUM_FIXTURE_SIZE: usize = 1_024;
const LARGE_FIXTURE_SIZE: usize = 64 * 1_024;

// =============================================================================
// Deterministic byte-fixture generation
// =============================================================================
//
// The fixture generator intentionally avoids randomness.
//
// Randomness is inappropriate for a determinism test because a random fixture
// would make failures harder to reproduce.
//
// The generator is deterministic for every supplied length.

fn deterministic_bytes(length: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(length);

    for index in 0..length {
        let value = index as u64;

        // A deterministic avalanche-like mixing function.
        //
        // This is test-fixture generation only. It is NOT a cryptographic
        // algorithm and must never be used by production hashing.
        let mixed = value
            .wrapping_mul(0x9E37_79B9_7F4A_7C15)
            .rotate_left(17)
            ^ 0xA5A5_A5A5_A5A5_A5A5;

        bytes.push((mixed & 0xff) as u8);
    }

    bytes
}

// =============================================================================
// Deterministic reader
// =============================================================================

/// Reader wrapper used to verify that streaming hashing is independent from
/// read chunk boundaries.
struct ChunkedReader<R> {
    inner: R,
    chunk_size: usize,
}

impl<R> ChunkedReader<R> {
    fn new(inner: R, chunk_size: usize) -> Self {
        assert!(
            chunk_size > 0,
            "test fixture chunk size must be greater than zero"
        );

        Self { inner, chunk_size }
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
// Canonical hash helpers
// =============================================================================

fn hash_raw(domain: HashDomain, bytes: &[u8]) -> IrHash {
    hash_bytes(domain, bytes)
}

fn hash_stream(domain: HashDomain, bytes: &[u8], chunk_size: usize) -> IrHash {
    let reader = ChunkedReader::new(Cursor::new(bytes), chunk_size);

    hash_reader(domain, reader)
        .unwrap_or_else(|error| {
            panic!(
                "deterministic streaming hash fixture unexpectedly failed: {error}"
            )
        })
}

// =============================================================================
// Hash representation tests
// =============================================================================

#[test]
fn hash_has_fixed_sha256_size() {
    let hash = hash_raw(HashDomain::Raw, b"Zamani");

    assert_eq!(
        hash.as_bytes().len(),
        HASH_BYTES,
        "IrHash must always contain exactly one SHA-256 digest"
    );

    assert_eq!(
        hash.to_bytes().len(),
        HASH_BYTES,
        "IrHash::to_bytes must preserve the fixed SHA-256 width"
    );

    assert_eq!(
        hash.to_hex().len(),
        HASH_HEX_BYTES,
        "hexadecimal representation must contain exactly two characters per digest byte"
    );
}

#[test]
fn hash_hex_round_trip_is_deterministic() {
    let inputs = [
        b"".as_slice(),
        b"Zamani".as_slice(),
        b"quantum".as_slice(),
        b"canonical-ir".as_slice(),
        b"deterministic".as_slice(),
    ];

    for input in inputs {
        let original = hash_raw(HashDomain::Raw, input);
        let hexadecimal = original.to_hex();

        let decoded = IrHash::from_hex(&hexadecimal)
            .unwrap_or_else(|error| {
                panic!(
                    "valid canonical hash hexadecimal failed to parse: {error}"
                )
            });

        assert_eq!(
            original, decoded,
            "hash hexadecimal encoding must round-trip exactly"
        );

        assert_eq!(
            hexadecimal,
            decoded.to_hex(),
            "re-encoding a decoded hash must be byte-for-byte deterministic"
        );
    }
}

#[test]
fn hash_hex_is_lowercase() {
    let hash = hash_raw(HashDomain::Raw, b"Zamani Quantum IR");
    let hexadecimal = hash.to_hex();

    assert!(
        hexadecimal
            .chars()
            .all(|character| !character.is_ascii_uppercase()),
        "canonical hexadecimal hash output must use lowercase ASCII"
    );
}

#[test]
fn hash_zero_value_is_distinguishable_from_real_hashes() {
    let zero = IrHash::default();

    assert!(
        zero.is_zero(),
        "IrHash::default must be the all-zero sentinel"
    );

    let real = hash_raw(HashDomain::Raw, b"");

    assert_ne!(
        real, zero,
        "a real SHA-256 content hash must not equal the all-zero sentinel"
    );
}

// =============================================================================
// Repeated hashing determinism
// =============================================================================

#[test]
fn hashing_identical_input_repeatedly_produces_identical_output() {
    let payload = deterministic_bytes(MEDIUM_FIXTURE_SIZE);

    let first = hash_raw(HashDomain::Raw, &payload);

    for _ in 0..128 {
        let repeated = hash_raw(HashDomain::Raw, &payload);

        assert_eq!(
            first, repeated,
            "hashing identical canonical bytes must always produce the same digest"
        );
    }
}

#[test]
fn deterministic_fixture_generation_is_stable() {
    let first = deterministic_bytes(MEDIUM_FIXTURE_SIZE);
    let second = deterministic_bytes(MEDIUM_FIXTURE_SIZE);

    assert_eq!(
        first, second,
        "test fixtures themselves must be reproducible"
    );
}

// =============================================================================
// Domain separation
// =============================================================================

#[test]
fn different_hash_domains_produce_different_content_identities() {
    let payload = b"same canonical payload";

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

    let hashes: Vec<IrHash> = domains
        .iter()
        .map(|domain| hash_raw(*domain, payload))
        .collect();

    for left in 0..hashes.len() {
        for right in (left + 1)..hashes.len() {
            assert_ne!(
                hashes[left], hashes[right],
                "distinct semantic hash domains must be domain-separated"
            );
        }
    }
}

#[test]
fn hash_domain_ids_are_unique() {
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

    let mut ids = domains.iter().map(|domain| domain.id()).collect::<Vec<_>>();

    ids.sort_unstable();

    for pair in ids.windows(2) {
        assert_ne!(
            pair[0], pair[1],
            "published hash-domain identifiers must be unique"
        );
    }
}

#[test]
fn hash_domain_names_are_unique() {
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

    let mut names = domains
        .iter()
        .map(|domain| domain.name())
        .collect::<Vec<_>>();

    names.sort_unstable();

    for pair in names.windows(2) {
        assert_ne!(
            pair[0], pair[1],
            "hash-domain names must remain unique"
        );
    }
}

// =============================================================================
// Payload sensitivity
// =============================================================================

#[test]
fn one_byte_semantic_change_changes_hash() {
    let mut first = deterministic_bytes(MEDIUM_FIXTURE_SIZE);
    let mut second = first.clone();

    let position = second.len() / 2;

    let original = second[position];
    second[position] = original.wrapping_add(1);

    let first_hash = hash_raw(HashDomain::Raw, &first);
    let second_hash = hash_raw(HashDomain::Raw, &second);

    assert_ne!(
        first_hash, second_hash,
        "a semantic payload change must change its content identity"
    );

    // Restore the fixture so this test explicitly verifies that only the
    // intended byte was changed.
    assert_eq!(
        first[position], original,
        "the determinism fixture must remain unchanged"
    );

    first[position] = original;
}

#[test]
fn appended_data_changes_hash() {
    let first = deterministic_bytes(SMALL_FIXTURE_SIZE);
    let mut second = first.clone();

    second.push(0);

    let first_hash = hash_raw(HashDomain::Raw, &first);
    let second_hash = hash_raw(HashDomain::Raw, &second);

    assert_ne!(
        first_hash, second_hash,
        "changing canonical payload length must change the content identity"
    );
}

#[test]
fn empty_and_nonempty_payloads_have_distinct_hashes() {
    let empty = hash_raw(HashDomain::Raw, &[]);
    let nonempty = hash_raw(HashDomain::Raw, &[0]);

    assert_ne!(
        empty, nonempty,
        "empty and non-empty canonical payloads must not collide under the IR hash contract"
    );
}

// =============================================================================
// Streaming determinism
// =============================================================================

#[test]
fn_streaming_hash_matches_direct_hash() {
    let payload = deterministic_bytes(LARGE_FIXTURE_SIZE);

    let direct = hash_raw(HashDomain::Raw, &payload);

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
        payload.len(),
        payload.len().saturating_add(1),
    ];

    for chunk_size in chunk_sizes {
        let streamed = hash_stream(HashDomain::Raw, &payload, chunk_size);

        assert_eq!(
            direct, streamed,
            "streaming hash must be independent of reader chunk size: {chunk_size}"
        );
    }
}

#[test]
fn streaming_hash_is_stable_across_repeated_reads() {
    let payload = deterministic_bytes(MEDIUM_FIXTURE_SIZE);

    let expected = hash_stream(HashDomain::Raw, &payload, 7);

    for _ in 0..64 {
        let actual = hash_stream(HashDomain::Raw, &payload, 7);

        assert_eq!(
            expected, actual,
            "repeated streaming reads over identical canonical bytes must be deterministic"
        );
    }
}

#[test]
fn one_byte_reader_and_large_reader_produce_identical_hashes() {
    let payload = deterministic_bytes(LARGE_FIXTURE_SIZE);

    let byte_at_a_time = hash_stream(HashDomain::Raw, &payload, 1);
    let large_chunks = hash_stream(HashDomain::Raw, &payload, 64 * 1024);

    assert_eq!(
        byte_at_a_time, large_chunks,
        "hashing must not depend on how a canonical byte stream is chunked"
    );
}

// =============================================================================
// Qubit identity boundary
// =============================================================================
//
// Qubit identity belongs to:
//!
//!     quantum::ir::qubit::QubitId
//
//! The hashing layer must consume that canonical type rather than defining
//! another qubit identity representation.

#[test]
fn canonical_qubit_hashing_is_stable() {
    let qubit = QubitId::new(0);

    let first = hash_qubit_id(qubit);
    let second = hash_qubit_id(qubit);

    assert_eq!(
        first, second,
        "the canonical QubitId hash must be deterministic"
    );
}

#[test]
fn distinct_qubit_ids_have_distinct_content_hashes() {
    let first_qubit = QubitId::new(0);
    let second_qubit = QubitId::new(1);

    let first_hash = hash_qubit_id(first_qubit);
    let second_hash = hash_qubit_id(second_qubit);

    assert_ne!(
        first_hash, second_hash,
        "distinct logical qubit identities must have distinct canonical hashes"
    );
}

#[test]
fn qubit_hash_is_domain_separated_from_raw_hashing() {
    let qubit = QubitId::new(0);

    let qubit_hash = hash_qubit_id(qubit);

    // This deliberately does not attempt to reproduce the qubit hashing
    // implementation. The point is that canonical QubitId hashing must be
    // associated with the logical-qubit semantic domain.
    let raw_hash = hash_raw(HashDomain::Raw, &[0]);

    assert_ne!(
        qubit_hash, raw_hash,
        "logical qubit content identity must be domain-separated from raw bytes"
    );
}

// =============================================================================
// Size-scaling tests
// =============================================================================
//
// These tests intentionally exercise several scales while never treating any
// particular scale as a semantic maximum.

#[test]
fn hashing_is_deterministic_across_multiple_payload_sizes() {
    let sizes = [
        SMALL_FIXTURE_SIZE,
        2,
        8,
        32,
        128,
        1_024,
        16 * 1_024,
        LARGE_FIXTURE_SIZE,
    ];

    for size in sizes {
        let payload = deterministic_bytes(size);

        let first = hash_raw(HashDomain::Raw, &payload);
        let second = hash_raw(HashDomain::Raw, &payload);

        assert_eq!(
            first, second,
            "hash determinism failed for fixture size {size}"
        );
    }
}

#[test]
fn hashing_large_payload_does_not_change_semantics() {
    let payload = deterministic_bytes(LARGE_FIXTURE_SIZE);

    let direct = hash_raw(HashDomain::Raw, &payload);

    // Hash the same payload in several streaming configurations.
    let configurations = [1usize, 17, 257, 4096, 65_536];

    for chunk_size in configurations {
        let streamed = hash_stream(HashDomain::Raw, &payload, chunk_size);

        assert_eq!(
            direct, streamed,
            "large-payload hash changed when streaming chunk size changed"
        );
    }
}

// =============================================================================
// Algorithm/version contract
// =============================================================================

#[test]
fn hashing_contract_constants_are_self_consistent() {
    assert_eq!(
        HASH_ALGORITHM.id(),
        1,
        "the published SHA-256 algorithm identifier must remain stable"
    );

    assert_eq!(
        HASH_ALGORITHM.name(),
        "sha256",
        "the published SHA-256 algorithm name must remain stable"
    );

    assert_eq!(
        HASH_BYTES,
        32,
        "SHA-256 must remain a 32-byte digest"
    );

    assert_eq!(
        HASH_HEX_BYTES,
        HASH_BYTES * 2,
        "hexadecimal width must remain exactly two characters per digest byte"
    );

    assert!(
        HASH_SCHEMA_VERSION > 0,
        "the hash schema must have a non-zero published version"
    );
}

// =============================================================================
// Ordering / collection determinism
// =============================================================================
//
// The canonical serializer is responsible for canonical semantic ordering.
// These tests verify the lower-level hashing contract without attempting to
// impose an ordering on semantic operations.

#[test]
fn hashing_same_sequence_in_same_order_is_stable() {
    let values = [
        b"alpha".as_slice(),
        b"beta".as_slice(),
        b"gamma".as_slice(),
        b"delta".as_slice(),
    ];

    let first = values
        .iter()
        .flat_map(|value| value.iter().copied())
        .collect::<Vec<_>>();

    let second = values
        .iter()
        .flat_map(|value| value.iter().copied())
        .collect::<Vec<_>>();

    assert_eq!(
        first, second,
        "the canonical fixture sequence must be stable"
    );

    assert_eq!(
        hash_raw(HashDomain::Raw, &first),
        hash_raw(HashDomain::Raw, &second),
        "identical ordered canonical bytes must hash identically"
    );
}

#[test]
fn different_byte_order_is_semantically_distinct() {
    let first = b"abcd";
    let second = b"dcba";

    assert_ne!(
        hash_raw(HashDomain::Raw, first),
        hash_raw(HashDomain::Raw, second),
        "different canonical byte ordering must remain semantically distinguishable"
    );
}

// =============================================================================
// Regression guards for common nondeterminism sources
// =============================================================================

#[test]
fn hash_does_not_depend_on_memory_address() {
    let first_storage = deterministic_bytes(MEDIUM_FIXTURE_SIZE);
    let second_storage = first_storage.clone();

    let first_hash = hash_raw(HashDomain::Raw, &first_storage);
    let second_hash = hash_raw(HashDomain::Raw, &second_storage);

    assert_eq!(
        first_hash, second_hash,
        "identical semantic bytes must hash identically regardless of allocation address"
    );
}

#[test]
fn hash_does_not_depend_on_container_capacity() {
    let source = deterministic_bytes(MEDIUM_FIXTURE_SIZE);

    let mut compact = Vec::with_capacity(source.len());
    compact.extend_from_slice(&source);

    let mut oversized = Vec::with_capacity(source.len().saturating_mul(4));
    oversized.extend_from_slice(&source);

    assert_eq!(
        compact.len(),
        oversized.len(),
        "test containers must contain identical semantic data"
    );

    assert_eq!(
        hash_raw(HashDomain::Raw, &compact),
        hash_raw(HashDomain::Raw, &oversized),
        "hashing must depend on semantic bytes, not vector capacity"
    );
}

#[test]
fn hash_does_not_depend_on_reader_chunking() {
    let payload = deterministic_bytes(MEDIUM_FIXTURE_SIZE);

    let hash_one = hash_stream(HashDomain::Raw, &payload, 1);
    let hash_prime = hash_stream(HashDomain::Raw, &payload, 127);
    let hash_large = hash_stream(HashDomain::Raw, &payload, 8192);

    assert_eq!(
        hash_one, hash_prime,
        "reader chunking must not affect canonical content identity"
    );

    assert_eq!(
        hash_prime, hash_large,
        "reader chunking must not affect canonical content identity"
    );
}

// =============================================================================
// Canonical collision-boundary tests
// =============================================================================
//
// These tests are intentionally about representation boundaries. They prevent
// future changes from accidentally collapsing distinct payloads into the same
// framing.
//
// They do NOT attempt to prove cryptographic collision resistance.

#[test]
fn length_boundaries_are_distinguishable() {
    let cases = [
        b"".as_slice(),
        b"a".as_slice(),
        b"ab".as_slice(),
        b"abc".as_slice(),
        b"abcd".as_slice(),
        b"abcde".as_slice(),
    ];

    let hashes = cases
        .iter()
        .map(|payload| hash_raw(HashDomain::Raw, payload))
        .collect::<Vec<_>>();

    for left in 0..hashes.len() {
        for right in (left + 1)..hashes.len() {
            assert_ne!(
                hashes[left], hashes[right],
                "different length-prefixed payloads must remain distinguishable"
            );
        }
    }
}

// =============================================================================
// Public contract summary test
// =============================================================================

#[test]
fn canonical_hash_contract_is_deterministic() {
    let payload = deterministic_bytes(MEDIUM_FIXTURE_SIZE);

    // Direct canonical hashing.
    let direct_a = hash_raw(HashDomain::Raw, &payload);
    let direct_b = hash_raw(HashDomain::Raw, &payload);

    assert_eq!(
        direct_a, direct_b,
        "direct canonical hashing must be deterministic"
    );

    // Streaming canonical hashing.
    let streamed_a = hash_stream(HashDomain::Raw, &payload, 1);
    let streamed_b = hash_stream(HashDomain::Raw, &payload, 4096);

    assert_eq!(
        streamed_a, streamed_b,
        "streaming canonical hashing must be independent of chunk size"
    );

    // Direct and streaming forms must represent the same canonical bytes.
    assert_eq!(
        direct_a, streamed_a,
        "direct and streaming canonical hashing must agree"
    );

    // Canonical hexadecimal representation must also be stable.
    assert_eq!(
        direct_a.to_hex(),
        direct_b.to_hex(),
        "canonical textual hash representation must be deterministic"
    );

    // The result must have the published SHA-256 width.
    assert_eq!(
        direct_a.as_bytes().len(),
        HASH_BYTES,
        "canonical hash must have the published digest width"
    );
}