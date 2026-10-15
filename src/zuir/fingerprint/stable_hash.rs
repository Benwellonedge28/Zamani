
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Stable Content Fingerprints
//!
//! Path: `src/zuir/fingerprint/stable_hash.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide deterministic, versioned, domain-separated SHA-256 fingerprints
//! for canonical ZUIR content.
//!
//! This module owns the fingerprint algorithm and its canonical byte-framing
//! contract. It does not decide which fields of a ZUIR entity are semantic.
//! The corresponding entity or serialization adapter owns that decision.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/version.rs`
//! - `src/zuir/serialization/canonical.rs`
//! - `src/zuir/serialization/schema.rs`
//! - `src/zuir/schema/metadata.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//!
//! ## OWNS
//!
//! - The stable fingerprint algorithm identifier.
//! - Fingerprint representation and hexadecimal display.
//! - Domain-separated, length-framed hashing.
//! - Checked accounting for input bytes and field counts.
//! - A minimal trait for explicit canonical encoding.
//! - Stable encoding for basic scalar and ordered collection types.
//!
//! ## DOES_NOT_OWN
//!
//! - ZUIR program, module, operation, type, or dialect semantics.
//! - The choice of semantic versus descriptive fields.
//! - Source parsing, grammar interpretation, or name resolution.
//! - Canonical JSON rendering or serialized document envelopes.
//! - Cache storage, cache invalidation, or incremental dependency graphs.
//! - Digital signatures, authentication, or authorization.
//! - Target discovery, physical allocation, or runtime execution.
//!
//! ## STABILITY CONTRACT
//!
//! The digest is defined over a byte stream constructed by this module.
//! Field boundaries and the fingerprint domain are part of that contract.
//!
//! The same canonical field sequence, domain, and algorithm version produce
//! the same digest across runs and platforms.
//!
//! A change to field framing, field order, integer encoding, semantic field
//! selection, or canonical collection ordering can change fingerprints.
//! Such changes require an explicit fingerprint schema/version decision.
//!
//! `StableEncode` implementations MUST NOT depend on `std::hash::Hash`,
//! `DefaultHasher`, memory addresses, randomized iteration, thread order,
//! locale, platform endianness, or process-local identity allocation.
//!
//! ## SECURITY CONTRACT
//!
//! SHA-256 is a cryptographic hash, but a fingerprint is not a signature.
//! It does not establish who produced an artifact or whether that artifact
//! is authorized. Authenticity requires a separate signature or trust policy.
//!
//! A digest has finite length. Collisions are theoretically possible.
//! A fingerprint MUST NOT replace full semantic verification or be treated
//! as proof of equivalence when the input encoding is not canonical.
//!
//! ## SCALABILITY CONTRACT
//!
//! Hashing is incremental. The input does not need to be accumulated into
//! one complete byte buffer by this module.
//!
//! There is no universal maximum program size, operation count, qubit count,
//! tensor rank, device count, or node count. The implementation remains
//! subject to finite process memory, execution time, and platform limits.
//!
//! Byte and field counters use checked arithmetic. Overflow is an explicit
//! error rather than silent wraparound.
//!
//! ## INTEGRATION CONTRACT
//!
//! - `fingerprint/mod.rs` declares this module and re-exports the public API.
//! - The parent ZUIR module exposes `fingerprint`.
//! - ZUIR entity adapters implement `StableEncode` for their canonical,
//!   semantically relevant fields.
//! - `serialization/canonical.rs` remains the owner of canonical JSON bytes.
//! - Cache keys must include the appropriate algorithm/schema version and
//!   all compilation inputs that affect the cached result.
//! - Source maps and descriptive metadata are excluded from semantic
//!   fingerprints unless a particular fingerprint domain explicitly includes
//!   them.
//!
//! This file intentionally does not import the ZUIR IR, serializer facade,
//! dialect registry, or incremental subsystem. That prevents dependency
//! cycles and lets this contract be frozen independently.
//!
//! ## FREEZE CRITERIA
//!
//! - Stable domain separation and field framing are tested.
//! - Integer encoding is fixed-width and endian-independent.
//! - Ordered collections encode in their documented order.
//! - Hashing does not depend on Rust's `Hash` trait or default hasher.
//! - Input counters cannot wrap silently.
//! - Known test vectors and repeatability tests pass.
//! - The module compiles with `#![forbid(unsafe_code)]`.
//!
//! Dependencies: the repository's existing `sha2` crate.

use core::fmt;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Stable identifier for this fingerprint algorithm.
///
/// This identifies SHA-256 with ZUIR's framed encoding. It does not, by
/// itself, identify the schema or semantic fields of a particular entity.
pub const FINGERPRINT_ALGORITHM: &str = "zamani-zuir-sha256-framed-v1";

/// Number of bytes in a SHA-256 fingerprint.
pub const FINGERPRINT_BYTES: usize = 32;

/// Fingerprint schema version used by the framed hashing protocol.
///
/// Bump this only when the protocol changes incompatibly. Entity-specific
/// canonical encoding changes must also be versioned by their owning schema.
pub const FINGERPRINT_FORMAT_VERSION: u32 = 1;

/// A stable, fixed-size content fingerprint.
///
/// This value is suitable for deterministic cache keys and content identity
/// within a documented fingerprint domain. It is not an entity allocator ID,
/// digital signature, or proof that two programs are semantically equivalent.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fingerprint([u8; FINGERPRINT_BYTES]);

impl Fingerprint {
    /// Construct a fingerprint from its complete fixed-size digest.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; FINGERPRINT_BYTES]) -> Self {
        Self(bytes)
    }

    /// Return the digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; FINGERPRINT_BYTES] {
        &self.0
    }

    /// Consume the fingerprint and return its digest bytes.
    #[must_use]
    pub const fn into_bytes(self) -> [u8; FINGERPRINT_BYTES] {
        self.0
    }

    /// Encode the digest as lowercase hexadecimal.
    ///
    /// This method allocates a string proportional to the fixed digest size.
    #[must_use]
    pub fn to_hex(self) -> String {
        let mut output = String::with_capacity(FINGERPRINT_BYTES * 2);
        const HEX: &[u8; 16] = b"0123456789abcdef";

        for byte in self.0 {
            output.push(HEX[(byte >> 4) as usize] as char);
            output.push(HEX[(byte & 0x0f) as usize] as char);
        }

        output
    }
}

impl fmt::Display for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Fingerprint")
            .field(&self.to_hex())
            .finish()
    }
}

/// Errors raised by canonical fingerprint construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FingerprintError {
    /// The supplied domain is empty or contains forbidden control characters.
    InvalidDomain,

    /// A field name is empty or contains forbidden control characters.
    InvalidFieldName,

    /// A byte, field, or item counter exceeded its representable range.
    CounterOverflow {
        counter: &'static str,
    },

    /// A value cannot be represented by the specified encoding contract.
    InvalidEncoding {
        detail: &'static str,
    },
}

impl fmt::Display for FingerprintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDomain => {
                f.write_str("fingerprint domain must be nonempty and contain no control characters")
            }
            Self::InvalidFieldName => {
                f.write_str("fingerprint field name must be nonempty and contain no control characters")
            }
            Self::CounterOverflow { counter } => {
                write!(f, "fingerprint {counter} counter overflowed")
            }
            Self::InvalidEncoding { detail } => {
                write!(f, "invalid canonical fingerprint encoding: {detail}")
            }
        }
    }
}

impl std::error::Error for FingerprintError {}

/// Canonical, incremental fingerprint builder.
///
/// Every field is encoded as:
///
/// `field-marker || name-length(u64 BE) || name || value-length(u64 BE) || value`
///
/// The initial stream contains a fixed protocol marker, the protocol version,
/// and the length-prefixed domain. Length prefixes prevent concatenation
/// ambiguity: `("ab", "c")` differs from `("a", "bc")`.
///
/// Callers must provide fields in the canonical order specified by the
/// corresponding entity schema. This builder does not sort arbitrary fields,
/// because silently sorting could change a schema's declared semantics.
pub struct StableHashBuilder {
    hasher: Sha256,
    bytes_hashed: u64,
    fields_hashed: u64,
}

impl StableHashBuilder {
    /// Start a fingerprint for a named semantic domain.
    ///
    /// Examples of domains include `zuir.program.semantic`,
    /// `zuir.operation.semantic`, and `zuir.cache.codegen`.
    ///
    /// A domain name is part of the digest and MUST remain stable for the
    /// lifetime of its encoding contract.
    pub fn new(domain: &str) -> Result<Self, FingerprintError> {
        validate_name(domain).map_err(|_| FingerprintError::InvalidDomain)?;

        let mut builder = Self {
            hasher: Sha256::new(),
            bytes_hashed: 0,
            fields_hashed: 0,
        };

        builder.write_raw(b"ZAMANI-ZUIR-FINGERPRINT\0")?;
        builder.write_raw(&FINGERPRINT_FORMAT_VERSION.to_be_bytes())?;
        builder.write_len_prefixed(domain.as_bytes())?;

        Ok(builder)
    }

    /// Append a named field whose value is a byte string.
    ///
    /// The field name and field order are part of the canonical encoding.
    pub fn field_bytes(
        &mut self,
        name: &str,
        value: &[u8],
    ) -> Result<&mut Self, FingerprintError> {
        validate_name(name).map_err(|_| FingerprintError::InvalidFieldName)?;

        let next_fields = self
            .fields_hashed
            .checked_add(1)
            .ok_or(FingerprintError::CounterOverflow {
                counter: "field",
            })?;

        self.write_raw(&[0xF1])?;
        self.write_len_prefixed(name.as_bytes())?;
        self.write_len_prefixed(value)?;

        self.fields_hashed = next_fields;
        Ok(self)
    }

    /// Append a UTF-8 string field.
    pub fn field_str(
        &mut self,
        name: &str,
        value: &str,
    ) -> Result<&mut Self, FingerprintError> {
        self.field_bytes(name, value.as_bytes())
    }

    /// Append a boolean as exactly one byte: zero or one.
    pub fn field_bool(
        &mut self,
        name: &str,
        value: bool,
    ) -> Result<&mut Self, FingerprintError> {
        self.field_bytes(name, &[u8::from(value)])
    }

    /// Append a u8 field.
    pub fn field_u8(
        &mut self,
        name: &str,
        value: u8,
    ) -> Result<&mut Self, FingerprintError> {
        self.field_bytes(name, &[value])
    }

    /// Append a u32 field using big-endian byte order.
    pub fn field_u32(
        &mut self,
        name: &str,
        value: u32,
    ) -> Result<&mut Self, FingerprintError> {
        self.field_bytes(name, &value.to_be_bytes())
    }

    /// Append a u64 field using big-endian byte order.
    pub fn field_u64(
        &mut self,
        name: &str,
        value: u64,
    ) -> Result<&mut Self, FingerprintError> {
        self.field_bytes(name, &value.to_be_bytes())
    }

    /// Append a u128 field using big-endian byte order.
    pub fn field_u128(
        &mut self,
        name: &str,
        value: u128,
    ) -> Result<&mut Self, FingerprintError> {
        self.field_bytes(name, &value.to_be_bytes())
    }

    /// Append an i64 field using its fixed-width two's-complement encoding.
    pub fn field_i64(
        &mut self,
        name: &str,
        value: i64,
    ) -> Result<&mut Self, FingerprintError> {
        self.field_bytes(name, &value.to_be_bytes())
    }

    /// Append a nested fingerprint as its raw 32-byte digest.
    pub fn field_fingerprint(
        &mut self,
        name: &str,
        value: Fingerprint,
    ) -> Result<&mut Self, FingerprintError> {
        self.field_bytes(name, value.as_bytes())
    }

    /// Return the number of bytes incorporated into the hash stream.
    ///
    /// This includes protocol framing and field framing.
    #[must_use]
    pub const fn bytes_hashed(&self) -> u64 {
        self.bytes_hashed
    }

    /// Return the number of fields incorporated into the hash stream.
    #[must_use]
    pub const fn fields_hashed(&self) -> u64 {
        self.fields_hashed
    }

    /// Finalize the stream and return its digest.
    ///
    /// Finalization consumes the builder; appending after finalization is
    /// impossible through this API.
    #[must_use]
    pub fn finish(self) -> Fingerprint {
        let digest = self.hasher.finalize();
        let mut output = [0_u8; FINGERPRINT_BYTES];
        output.copy_from_slice(&digest);
        Fingerprint(output)
    }

    fn write_len_prefixed(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), FingerprintError> {
        let length = u64::try_from(bytes.len()).map_err(|_| {
            FingerprintError::CounterOverflow {
                counter: "field-length",
            }
        })?;

        self.write_raw(&length.to_be_bytes())?;
        self.write_raw(bytes)
    }

    fn write_raw(&mut self, bytes: &[u8]) -> Result<(), FingerprintError> {
        let length = u64::try_from(bytes.len()).map_err(|_| {
            FingerprintError::CounterOverflow {
                counter: "byte",
            }
        })?;

        let next = self
            .bytes_hashed
            .checked_add(length)
            .ok_or(FingerprintError::CounterOverflow {
                counter: "byte",
            })?;

        self.hasher.update(bytes);
        self.bytes_hashed = next;
        Ok(())
    }
}

fn validate_name(value: &str) -> Result<(), ()> {
    if value.is_empty() || value.chars().any(char::is_control) {
        return Err(());
    }
    Ok(())
}

/// Implement this trait to provide a versioned canonical encoding.
///
/// Implementations MUST:
///
/// - Use a stable field order.
/// - Include every field that belongs to the selected fingerprint domain.
/// - Exclude fields declared non-semantic by that domain.
/// - Encode numeric values explicitly, not via host-memory layout.
/// - Encode maps in canonical key order.
/// - Include relevant schema/dialect versions.
/// - Never depend on addresses, thread order, randomized hashing, or local IDs
///   unless those IDs are explicitly semantic and stably encoded.
///
/// A trait implementation is a schema contract. Changing its output may
/// invalidate persisted cache keys and must be reviewed accordingly.
pub trait StableEncode {
    /// Append this value's canonical representation to a fingerprint builder.
    fn stable_encode(
        &self,
        builder: &mut StableHashBuilder,
    ) -> Result<(), FingerprintError>;
}

/// Compute a fingerprint using an explicit domain and canonical encoder.
pub fn fingerprint<T: StableEncode + ?Sized>(
    domain: &str,
    value: &T,
) -> Result<Fingerprint, FingerprintError> {
    let mut builder = StableHashBuilder::new(domain)?;
    value.stable_encode(&mut builder)?;
    Ok(builder.finish())
}

impl StableEncode for str {
    fn stable_encode(
        &self,
        builder: &mut StableHashBuilder,
    ) -> Result<(), FingerprintError> {
        builder.field_str("value", self)?;
        Ok(())
    }
}

impl StableEncode for String {
    fn stable_encode(
        &self,
        builder: &mut StableHashBuilder,
    ) -> Result<(), FingerprintError> {
        self.as_str().stable_encode(builder)
    }
}

impl StableEncode for [u8] {
    fn stable_encode(
        &self,
        builder: &mut StableHashBuilder,
    ) -> Result<(), FingerprintError> {
        builder.field_bytes("value", self)?;
        Ok(())
    }
}

impl StableEncode for Vec<u8> {
    fn stable_encode(
        &self,
        builder: &mut StableHashBuilder,
    ) -> Result<(), FingerprintError> {
        self.as_slice().stable_encode(builder)
    }
}

impl StableEncode for bool {
    fn stable_encode(
        &self,
        builder: &mut StableHashBuilder,
    ) -> Result<(), FingerprintError> {
        builder.field_bool("value", *self)?;
        Ok(())
    }
}

impl StableEncode for u8 {
    fn stable_encode(
        &self,
        builder: &mut StableHashBuilder,
    ) -> Result<(), FingerprintError> {
        builder.field_u8("value", *self)?;
        Ok(())
    }
}

impl StableEncode for u32 {
    fn stable_encode(
        &self,
        builder: &mut StableHashBuilder,
    ) -> Result<(), FingerprintError> {
        builder.field_u32("value", *self)?;
        Ok(())
    }
}

impl StableEncode for u64 {
    fn stable_encode(
        &self,
        builder: &mut StableHashBuilder,
    ) -> Result<(), FingerprintError> {
        builder.field_u64("value", *self)?;
        Ok(())
    }
}

impl StableEncode for u128 {
    fn stable_encode(
        &self,
        builder: &mut StableHashBuilder,
    ) -> Result<(), FingerprintError> {
        builder.field_u128("value", *self)?;
        Ok(())
    }
}

impl StableEncode for i64 {
    fn stable_encode(
        &self,
        builder: &mut StableHashBuilder,
    ) -> Result<(), FingerprintError> {
        builder.field_i64("value", *self)?;
        Ok(())
    }
}

impl<T: StableEncode> StableEncode for Option<T> {
    fn stable_encode(
        &self,
        builder: &mut StableHashBuilder,
    ) -> Result<(), FingerprintError> {
        match self {
            None => {
                builder.field_u8("option.tag", 0)?;
            }
            Some(value) => {
                builder.field_u8("option.tag", 1)?;
                value.stable_encode(builder)?;
            }
        }
        Ok(())
    }
}

/// Encode a sequence in iteration order.
///
/// The sequence length and each item boundary are explicit. The iterator
/// must already yield items in the canonical semantic order.
pub fn encode_sequence<T: StableEncode>(
    name: &str,
    values: &[T],
    builder: &mut StableHashBuilder,
) -> Result<(), FingerprintError> {
    builder.field_u64(
        &format!("{name}.length"),
        u64::try_from(values.len()).map_err(|_| {
            FingerprintError::CounterOverflow {
                counter: "sequence-length",
            }
        })?,
    )?;

    for (index, value) in values.iter().enumerate() {
        // Decimal index is unambiguous, stable, and independent of host width.
        builder.field_u64(
            &format!("{name}.index"),
            u64::try_from(index).map_err(|_| {
                FingerprintError::CounterOverflow {
                    counter: "sequence-index",
                }
            })?,
        )?;
        value.stable_encode(builder)?;
    }

    Ok(())
}

/// Encode a string-keyed ordered map in key order.
///
/// BTreeMap iteration is deterministic for the same keys and comparator.
/// The caller remains responsible for defining whether key ordering matches
/// the semantic canonicalization contract.
pub fn encode_string_map<T: StableEncode>(
    name: &str,
    values: &BTreeMap<String, T>,
    builder: &mut StableHashBuilder,
) -> Result<(), FingerprintError> {
    builder.field_u64(
        &format!("{name}.length"),
        u64::try_from(values.len()).map_err(|_| {
            FingerprintError::CounterOverflow {
                counter: "map-length",
            }
        })?,
    )?;

    for (key, value) in values {
        builder.field_str(&format!("{name}.key"), key)?;
        value.stable_encode(builder)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_domain_and_fields_produce_same_fingerprint() {
        let mut first =
            StableHashBuilder::new("zuir.test").expect("valid domain");
        first
            .field_str("name", "add")
            .expect("field should encode");
        first
            .field_u64("arity", 2)
            .expect("field should encode");

        let mut second =
            StableHashBuilder::new("zuir.test").expect("valid domain");
        second
            .field_str("name", "add")
            .expect("field should encode");
        second
            .field_u64("arity", 2)
            .expect("field should encode");

        assert_eq!(first.finish(), second.finish());
    }

    #[test]
    fn domain_separation_changes_fingerprint() {
        let mut first =
            StableHashBuilder::new("zuir.operation").expect("valid domain");
        first.field_str("name", "add").expect("field");

        let mut second =
            StableHashBuilder::new("zuir.type").expect("valid domain");
        second.field_str("name", "add").expect("field");

        assert_ne!(first.finish(), second.finish());
    }

    #[test]
    fn field_boundaries_prevent_concatenation_ambiguity() {
        let mut first =
            StableHashBuilder::new("zuir.test").expect("valid domain");
        first.field_str("a", "ab").expect("field");
        first.field_str("b", "c").expect("field");

        let mut second =
            StableHashBuilder::new("zuir.test").expect("valid domain");
        second.field_str("a", "a").expect("field");
        second.field_str("b", "bc").expect("field");

        assert_ne!(first.finish(), second.finish());
    }

    #[test]
    fn field_order_is_significant() {
        let mut first =
            StableHashBuilder::new("zuir.test").expect("valid domain");
        first.field_u8("a", 1).expect("field");
        first.field_u8("b", 2).expect("field");

        let mut second =
            StableHashBuilder::new("zuir.test").expect("valid domain");
        second.field_u8("b", 2).expect("field");
        second.field_u8("a", 1).expect("field");

        assert_ne!(first.finish(), second.finish());
    }

    #[test]
    fn integer_encoding_is_explicit_and_stable() {
        let mut first =
            StableHashBuilder::new("zuir.integer").expect("valid domain");
        first
            .field_u64("number", 0x0102_0304_0506_0708)
            .expect("field");

        let mut second =
            StableHashBuilder::new("zuir.integer").expect("valid domain");
        second
            .field_bytes("number", &[1, 2, 3, 4, 5, 6, 7, 8])
            .expect("field");

        // The outer field framing is identical, and the numeric payload is
        // exactly the specified big-endian byte sequence.
        assert_eq!(first.finish(), second.finish());
    }

    #[test]
    fn rejects_invalid_domains_and_field_names() {
        assert!(matches!(
            StableHashBuilder::new(""),
            Err(FingerprintError::InvalidDomain)
        ));

        assert!(matches!(
            StableHashBuilder::new("bad\ndomain"),
            Err(FingerprintError::InvalidDomain)
        ));

        let mut builder =
            StableHashBuilder::new("zuir.test").expect("valid domain");

        assert!(matches!(
            builder.field_str("", "value"),
            Err(FingerprintError::InvalidFieldName)
        ));
    }

    #[test]
    fn scalar_encoding_is_deterministic() {
        assert_eq!(
            fingerprint("zuir.string", "zamani").expect("fingerprint"),
            fingerprint("zuir.string", "zamani").expect("fingerprint")
        );

        assert_ne!(
            fingerprint("zuir.string", "zamani").expect("fingerprint"),
            fingerprint("zuir.string", "Zamani").expect("fingerprint")
        );
    }

    #[test]
    fn reports_accounting_information() {
        let mut builder =
            StableHashBuilder::new("zuir.test").expect("valid domain");

        builder.field_str("name", "value").expect("field");
        assert_eq!(builder.fields_hashed(), 1);
        assert!(builder.bytes_hashed() > 0);
    }

    #[test]
    fn digest_hex_is_fixed_length_lowercase() {
        let fingerprint =
            fingerprint("zuir.test", "value").expect("fingerprint");
        let hex = fingerprint.to_hex();

        assert_eq!(hex.len(), FINGERPRINT_BYTES * 2);
        assert!(hex.bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert_eq!(hex, hex.to_ascii_lowercase());
        assert_eq!(hex, fingerprint.to_string());
    }

    #[test]
    fn fingerprint_is_not_tied_to_runtime_identity() {
        let first = fingerprint("zuir.operation", "core.add")
            .expect("fingerprint");
        let second = fingerprint("zuir.operation", "core.add")
            .expect("fingerprint");

        assert_eq!(first, second);
    }

    #[test]
    fn fingerprints_are_fixed_size_values() {
        let value = fingerprint("zuir.test", "value")
            .expect("fingerprint");
        let bytes = value.into_bytes();

        assert_eq!(bytes.len(), FINGERPRINT_BYTES);
    }
}
