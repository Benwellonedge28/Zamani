
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani ZUIR Fingerprinting
//!
//! Path: `src/zuir/fingerprint/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later
//! Safety: Unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Define the public module boundary for deterministic, versioned ZUIR
//! content fingerprints.
//!
//! This module exposes the canonical hashing API implemented by
//! [`stable_hash`]. It does not implement hashing itself and does not
//! decide which fields of a program, module, type, operation, or dialect
//! belong in a semantic fingerprint.
//!
//! ## AUTHORITY
//!
//! The integration boundary must remain consistent with:
//!
//! - `DESIGN.md` and the repository's applicable architecture contracts.
//! - `ZAMANI_IR_SPEC.md` and the authoritative ZUIR specification,
//!   once the applicable specification paths are reconciled.
//! - `grammar/DESIGN.md`.
//! - `grammar/Zamani.g4`.
//! - `grammar/antlr/ZamaniLexer.g4`.
//! - `grammar/antlr/ZamaniParser.g4`.
//! - `grammar/specification/semantic-model.md`.
//! - `grammar/specification/scalability-model.md`.
//! - `src/zuir/serialization/canonical.rs`.
//! - `src/zuir/serialization/schema.rs`.
//! - `src/zuir/schema/metadata.rs`.
//!
//! The grammar defines source syntax, semantic analysis establishes program
//! meaning, and ZUIR represents that meaning. Fingerprinting consumes an
//! explicitly defined canonical representation; it does not infer semantics
//! from grammar rules or source spelling.
//!
//! ## OWNS
//!
//! - The public fingerprint module boundary.
//! - The public re-exports of the stable hashing contract.
//! - The API-level integration and conformance tests for those re-exports.
//!
//! ## DOES NOT OWN
//!
//! - The hashing algorithm or byte-framing implementation.
//! - Canonical JSON serialization or serialized-document schemas.
//! - Semantic field selection for programs, operations, or types.
//! - IR construction, verification, or transformation.
//! - Source parsing, grammar interpretation, or name resolution.
//! - Incremental compilation, cache storage, or cache invalidation.
//! - Digital signatures, authentication, or authorization.
//! - Target discovery, resource allocation, or runtime execution.
//!
//! ## DEPENDENCY CONTRACT
//!
//! This facade depends only on its child module, `stable_hash`.
//!
//! Child modules and downstream implementation modules should import the
//! defining module directly when doing so avoids circular dependencies.
//! This facade must not become a dependency of the hashing implementation.
//!
//! ## PUBLIC API CONTRACT
//!
//! The facade exposes:
//!
//! - `Fingerprint`: a fixed-size digest value.
//! - `FingerprintError`: structured fingerprint-construction failures.
//! - `StableEncode`: the explicit canonical-encoding extension trait.
//! - `StableHashBuilder`: the incremental, domain-separated hash builder.
//! - `fingerprint`: the generic fingerprint entry point.
//! - `encode_sequence`: deterministic sequence encoding.
//! - `encode_string_map`: deterministic string-keyed ordered-map encoding.
//! - Algorithm, digest-size, and format-version constants.
//!
//! Re-exports must correspond to actual public items in `stable_hash.rs`.
//! Adding a new public item to that implementation does not automatically
//! make it part of this facade's supported public API.
//!
//! ## CANONICALIZATION CONTRACT
//!
//! This module does not canonicalize arbitrary ZUIR entities.
//!
//! Callers must use a canonical field order and the appropriate fingerprint
//! domain. Entity-specific `StableEncode` implementations must include all
//! fields relevant to their declared domain, exclude fields that are
//! explicitly non-semantic, and encode all values without relying on
//! platform-specific memory representations.
//!
//! Ordered collections must follow their owning schema's canonical order.
//! Unordered collections must be canonicalized before encoding.
//!
//! Semantic fingerprints and complete artifact fingerprints are different
//! contracts. Source maps, descriptive metadata, and tooling metadata must
//! not accidentally influence a semantic fingerprint. Conversely, a cache
//! key must include every semantic and configuration input that can affect
//! the cached result.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! The facade introduces no fixed limit on program size, operation count,
//! qubit count, processor count, tensor rank, register width, device count,
//! node count, or topology size.
//!
//! Actual capacity remains bounded by available memory, address space,
//! execution time, the hashing implementation, and explicitly configured
//! resource budgets. No finite implementation can promise literal infinity.
//!
//! Fingerprint construction must report representational counter overflow
//! instead of silently wrapping. Resource exhaustion must not be interpreted
//! as semantic invalidity or successful compilation.
//!
//! ## SECURITY CONTRACT
//!
//! A content fingerprint is not a digital signature, an authorization
//! decision, or proof that an input program is valid.
//!
//! SHA-256 has a finite digest space, so collisions are theoretically
//! possible. Consumers must not use a fingerprint as a substitute for
//! verification, authorization, or validation of untrusted artifacts.
//!
//! ## INTEGRATION CONTRACT
//!
//! - `stable_hash.rs` owns hashing, framing, digest representation, and
//!   primitive canonical encoders.
//! - `serialization/canonical.rs` owns canonical JSON rendering.
//! - `serialization/schema.rs` owns serialized schema and version contracts.
//! - `schema/metadata.rs` owns metadata classification and fingerprint policy.
//! - ZUIR entity adapters own semantic field selection and field ordering.
//! - `incremental/cache_keys.rs` must include all compilation inputs that
//!   can affect a cached result.
//! - `incremental/dependencies.rs` and `incremental/invalidation.rs` own
//!   dependency tracking and safe cache invalidation.
//! - The parent ZUIR module may expose this module using `pub mod fingerprint;`.
//!
//! This facade does not assume that all of those integration modules already
//! exist or are fully implemented.
//!
//! ## FREEZE CRITERIA
//!
//! - Every re-export resolves to a public item in `stable_hash.rs`.
//! - The parent ZUIR module exposes the fingerprint module where required.
//! - No hashing logic or duplicate algorithm constants are introduced here.
//! - The facade tests compile against the actual implementation.
//! - The repository's formatting, compilation, test, and lint checks pass.
//! - No unsafe code or universal hardware-capacity limits are introduced.

// =============================================================================
// Canonical implementation boundary
// =============================================================================

/// Stable, domain-separated content fingerprinting.
///
/// This module owns the implementation. The parent facade intentionally
/// re-exports only the API selected below.
pub mod stable_hash;

// =============================================================================
// Stable public API
// =============================================================================

pub use self::stable_hash::{
    encode_sequence,
    encode_string_map,
    fingerprint,
    Fingerprint,
    FingerprintError,
    StableEncode,
    StableHashBuilder,
    FINGERPRINT_ALGORITHM,
    FINGERPRINT_BYTES,
    FINGERPRINT_FORMAT_VERSION,
};

// =============================================================================
// Facade conformance tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn public_fingerprint_api_is_available_through_facade() {
        let value = String::from("zamani");

        let result = fingerprint("zuir.facade.test", &value)
            .expect("a valid domain and string should be fingerprintable");

        assert_eq!(result.as_bytes().len(), FINGERPRINT_BYTES);
        assert_eq!(result.to_hex().len(), FINGERPRINT_BYTES * 2);
        assert!(!FINGERPRINT_ALGORITHM.is_empty());
        assert!(FINGERPRINT_FORMAT_VERSION > 0);
    }

    #[test]
    fn facade_preserves_domain_separation() {
        let value = String::from("same-content");

        let first = fingerprint("zuir.module.semantic", &value)
            .expect("first domain should be valid");

        let second = fingerprint("zuir.operation.semantic", &value)
            .expect("second domain should be valid");

        assert_ne!(first, second);
    }

    #[test]
    fn facade_exposes_incremental_builder() {
        let mut builder = StableHashBuilder::new("zuir.facade.builder")
            .expect("domain should be valid");

        builder
            .field_str("operation", "add")
            .expect("operation field should encode");

        builder
            .field_u64("operand_count", 2)
            .expect("operand count should encode");

        assert_eq!(builder.fields_hashed(), 2);
        assert!(builder.bytes_hashed() > 0);

        let result = builder.finish();

        assert_eq!(result.as_bytes().len(), FINGERPRINT_BYTES);
    }

    #[test]
    fn facade_exposes_deterministic_sequence_encoding() {
        let values = vec![11_u64, 22_u64, 33_u64];

        let mut first = StableHashBuilder::new("zuir.sequence.test")
            .expect("domain should be valid");

        encode_sequence("operands", &values, &mut first)
            .expect("sequence should encode");

        let mut second = StableHashBuilder::new("zuir.sequence.test")
            .expect("domain should be valid");

        encode_sequence("operands", &values, &mut second)
            .expect("sequence should encode");

        assert_eq!(first.finish(), second.finish());
    }

    #[test]
    fn facade_exposes_ordered_map_encoding() {
        let mut values = BTreeMap::new();
        values.insert(String::from("alpha"), 1_u64);
        values.insert(String::from("beta"), 2_u64);

        let mut first = StableHashBuilder::new("zuir.map.test")
            .expect("domain should be valid");

        encode_string_map("attributes", &values, &mut first)
            .expect("map should encode");

        let mut second_values = BTreeMap::new();
        second_values.insert(String::from("beta"), 2_u64);
        second_values.insert(String::from("alpha"), 1_u64);

        let mut second = StableHashBuilder::new("zuir.map.test")
            .expect("domain should be valid");

        encode_string_map("attributes", &second_values, &mut second)
            .expect("map should encode");

        assert_eq!(first.finish(), second.finish());
    }

    #[test]
    fn facade_exposes_structured_errors() {
        assert!(matches!(
            StableHashBuilder::new(""),
            Err(FingerprintError::InvalidDomain)
        ));

        assert!(matches!(
            StableHashBuilder::new("invalid\ndomain"),
            Err(FingerprintError::InvalidDomain)
        ));
    }

    #[test]
    fn facade_does_not_confuse_digest_identity_with_encoding_identity() {
        let value = String::from("example");

        let first = fingerprint("zuir.semantic.v1", &value)
            .expect("first fingerprint should succeed");

        let second = fingerprint("zuir.artifact.v1", &value)
            .expect("second fingerprint should succeed");

        // A digest is meaningful only within its declared fingerprint domain.
        assert_ne!(first, second);
    }
}
