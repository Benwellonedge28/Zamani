
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Deterministic Fingerprints
//!
//! Path: `src/hdl/hdlir/fingerprint.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//! Status: Foundation implementation; freeze after integration tests pass.
//!
//! ## Purpose
//!
//! Provide deterministic SHA-256 fingerprints for canonical HDLIR content.
//! This module defines the fingerprint protocol and does not depend on the
//! future HDLIR graph, schema registry, serializer, or transformation APIs.
//!
//! ## Authority and ownership
//!
//! This module must conform to:
//! - `grammar/spec/hdl.md`
//! - `grammar/specification/semantic-model.md`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`, once established
//! - `src/hdl/hdlir/compatibility.md`, once established
//!
//! The frontend semantic model owns semantic truth. ZUIR owns universal
//! computational meaning. HDLIR owns hardware-domain representation.
//! This module owns only the stable byte-level fingerprint protocol.
//!
//! It does not own entity IDs, source spans, canonicalization, graph
//! verification, serialization, target selection, or physical allocation.
//!
//! ## Protocol guarantees
//!
//! - SHA-256, identified by `HDLIR_FINGERPRINT_ALGORITHM`.
//! - Domain separation: callers must identify the kind of content hashed.
//! - Length-framed fields prevent concatenation ambiguity.
//! - Field order is significant and must be canonicalized by the caller.
//! - Integer helpers use explicit big-endian encoding.
//! - No pointer values, process-random hash seeds, or host-endian encoding.
//! - The same domain and same ordered fields yield the same fingerprint.
//!
//! ## Important limitations
//!
//! A fingerprint is a content identifier, not an equivalence proof.
//! Different fingerprints indicate different encoded content, not necessarily
//! different behavior. Equal fingerprints provide cryptographic evidence of
//! equal encoded content, subject to SHA-256's collision-resistance assumption.
//!
//! The builder cannot determine whether a caller has omitted a semantic field
//! or supplied fields in a noncanonical order. That obligation belongs to the
//! HDLIR canonical encoding contract and its tests.
//!
//! This module imposes no hardware-capacity limits. Its finite digest length
//! is a fixed protocol representation, not a limit on design size. Callers
//! should stream large designs into the builder one canonical field at a time
//! rather than materializing a second complete byte buffer.
//!
//! ## Integration
//!
//! The future `ir/`, `schema/`, `transform/`, and `serialization/` modules
//! may use this API after specifying their canonical field order and the
//! fields that affect semantic identity. `source_map.rs` provenance should
//! be included only when the fingerprint's documented purpose requires it.
//!
//! The fingerprint algorithm identifier and protocol version must be included
//! in persistent cache/artifact contracts. Changing the framing, domain
//! encoding, or field-count finalization requires a new protocol version.
//!
//! ## Errors
//!
//! Empty domains and field tags are rejected. Lengths that cannot be encoded
//! as `u64` and field-count exhaustion are reported rather than wrapped.
//!
//! ## Freeze criteria
//!
//! - Protocol bytes are documented and covered by known-answer tests.
//! - Field ordering is canonical at every production call site.
//! - Semantic fingerprints exclude incidental provenance unless specified.
//! - Persistent consumers record the algorithm/protocol identity.
//! - Repository formatting, build, test, and Clippy checks pass.

use core::fmt;
use sha2::{Digest, Sha256};

/// Stable identifier for the fingerprint algorithm and framing protocol.
///
/// This identifier is part of the persistent cache and artifact contract.
/// Any incompatible change to the byte protocol must use a new identifier.
pub const HDLIR_FINGERPRINT_ALGORITHM: &str = "zamani.hdlir.sha256-framed.v1";

/// Number of bytes in an SHA-256 digest.
pub const HDLIR_FINGERPRINT_SIZE: usize = 32;

/// Protocol prefix, including a zero byte to separate it from textual input.
const PROTOCOL_PREFIX: &[u8] = b"ZAMANI-HDLIR-FINGERPRINT\0";

/// Marker used to finalize the framed field sequence.
const FINALIZATION_MARKER: &[u8] = b"HDLIR-FIELDS-END\0";

/// A stable SHA-256 fingerprint of canonical HDLIR content.
///
/// Fingerprints can be compared, displayed, and used as deterministic cache
/// keys. They are not design IDs and must not be used as physical resource
/// identities or as proof of behavioral equivalence.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HdlirFingerprint([u8; HDLIR_FINGERPRINT_SIZE]);

impl HdlirFingerprint {
    /// Construct a fingerprint from its exact 32-byte digest.
    ///
    /// This does not verify how the digest was produced. Callers should
    /// normally obtain fingerprints from [`FingerprintBuilder::finish`].
    #[must_use]
    pub const fn from_bytes(bytes: [u8; HDLIR_FINGERPRINT_SIZE]) -> Self {
        Self(bytes)
    }

    /// Return the raw digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; HDLIR_FINGERPRINT_SIZE] {
        &self.0
    }

    /// Return the digest as 64 lowercase hexadecimal characters.
    #[must_use]
    pub fn to_hex(self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut output = String::with_capacity(HDLIR_FINGERPRINT_SIZE * 2);

        for byte in self.0 {
            output.push(char::from(HEX[usize::from(byte >> 4)]));
            output.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }

        output
    }
}

impl AsRef<[u8]> for HdlirFingerprint {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Display for HdlirFingerprint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(formatter, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for HdlirFingerprint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("HdlirFingerprint")
            .field(&self.to_hex())
            .finish()
    }
}

/// Errors returned while constructing a deterministic fingerprint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FingerprintError {
    /// The fingerprint domain was empty.
    EmptyDomain,

    /// A field tag was empty.
    EmptyFieldTag,

    /// A byte length could not be represented by the protocol's `u64` length.
    LengthNotRepresentable,

    /// The number of fields exceeded the protocol's representable field count.
    TooManyFields,
}

impl fmt::Display for FingerprintError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDomain => {
                formatter.write_str("HDLIR fingerprint domain must not be empty")
            }
            Self::EmptyFieldTag => {
                formatter.write_str("HDLIR fingerprint field tag must not be empty")
            }
            Self::LengthNotRepresentable => formatter.write_str(
                "HDLIR fingerprint byte length cannot be represented as u64",
            ),
            Self::TooManyFields => {
                formatter.write_str("HDLIR fingerprint field count overflowed")
            }
        }
    }
}

impl std::error::Error for FingerprintError {}

/// Incremental builder for a domain-separated HDLIR fingerprint.
///
/// Each field is encoded as a field marker followed by the tag length, tag
/// bytes, value length, and value bytes. Lengths are unsigned 64-bit
/// big-endian integers. The final digest also commits to the total field
/// count.
///
/// Field order is significant. Production callers must emit fields in the
/// canonical order specified by their owning HDLIR representation.
pub struct FingerprintBuilder {
    hasher: Sha256,
    field_count: u64,
}

impl FingerprintBuilder {
    /// Create a builder for a non-empty, stable content domain.
    ///
    /// Domain examples include `zamani.hdlir.design.v1`,
    /// `zamani.hdlir.module.v1`, and `zamani.hdlir.schema.v1`.
    ///
    /// The domain is part of the digest, so different domains produce
    /// different fingerprints even when their fields are otherwise identical.
    ///
    /// Domain names are UTF-8 and must be stable protocol identifiers, not
    /// localized labels or values derived from process-specific state.
    ///
    /// # Errors
    ///
    /// Returns [`FingerprintError::EmptyDomain`] when `domain` is empty.
    pub fn new(domain: &str) -> Result<Self, FingerprintError> {
        if domain.is_empty() {
            return Err(FingerprintError::EmptyDomain);
        }

        let domain_len = u64::try_from(domain.len())
            .map_err(|_| FingerprintError::LengthNotRepresentable)?;

        let mut hasher = Sha256::new();
        hasher.update(PROTOCOL_PREFIX);
        hasher.update(HDLIR_FINGERPRINT_ALGORITHM.as_bytes());
        hasher.update([0_u8]);
        hasher.update(domain_len.to_be_bytes());
        hasher.update(domain.as_bytes());

        Ok(Self {
            hasher,
            field_count: 0,
        })
    }

    /// Append a tagged byte field to the fingerprint input.
    ///
    /// The tag must identify the meaning of the field, not its position alone.
    /// Tags and values are length-framed, so fields such as `(ab, c)` and
    /// `(a, bc)` cannot produce the same encoded field sequence.
    ///
    /// This method hashes the supplied slice immediately; it does not retain
    /// a reference to the input.
    ///
    /// # Errors
    ///
    /// Returns an error if the tag is empty, either length cannot be encoded,
    /// or the field count would overflow. On error, the hasher remains
    /// unchanged.
    pub fn write_field(
        &mut self,
        tag: &str,
        value: &[u8],
    ) -> Result<(), FingerprintError> {
        if tag.is_empty() {
            return Err(FingerprintError::EmptyFieldTag);
        }

        let tag_len = u64::try_from(tag.len())
            .map_err(|_| FingerprintError::LengthNotRepresentable)?;
        let value_len = u64::try_from(value.len())
            .map_err(|_| FingerprintError::LengthNotRepresentable)?;
        let next_count = self
            .field_count
            .checked_add(1)
            .ok_or(FingerprintError::TooManyFields)?;

        // Framing is deliberately explicit and independent of host endianness.
        self.hasher.update(b"F");
        self.hasher.update(tag_len.to_be_bytes());
        self.hasher.update(tag.as_bytes());
        self.hasher.update(value_len.to_be_bytes());
        self.hasher.update(value);

        self.field_count = next_count;
        Ok(())
    }

    /// Append a UTF-8 string as a tagged field.
    ///
    /// The exact UTF-8 bytes are hashed. This method does not normalize
    /// Unicode; callers must define normalization where the owning semantic
    /// contract requires it.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::write_field`].
    pub fn write_str(
        &mut self,
        tag: &str,
        value: &str,
    ) -> Result<(), FingerprintError> {
        self.write_field(tag, value.as_bytes())
    }

    /// Append a `u64` using an eight-byte big-endian representation.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::write_field`].
    pub fn write_u64(
        &mut self,
        tag: &str,
        value: u64,
    ) -> Result<(), FingerprintError> {
        self.write_field(tag, &value.to_be_bytes())
    }

    /// Append an `i64` using an eight-byte two's-complement big-endian
    /// representation.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::write_field`].
    pub fn write_i64(
        &mut self,
        tag: &str,
        value: i64,
    ) -> Result<(), FingerprintError> {
        self.write_field(tag, &value.to_be_bytes())
    }

    /// Append a boolean using the single-byte encoding `0` or `1`.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::write_field`].
    pub fn write_bool(
        &mut self,
        tag: &str,
        value: bool,
    ) -> Result<(), FingerprintError> {
        self.write_field(tag, &[u8::from(value)])
    }

    /// Finalize the field sequence and return its fingerprint.
    ///
    /// This consumes the builder. The finalization marker and field count
    /// are included in the digest, distinguishing complete sequences from
    /// prefixes and making the protocol's sequence boundary explicit.
    #[must_use]
    pub fn finish(mut self) -> HdlirFingerprint {
        self.hasher.update(FINALIZATION_MARKER);
        self.hasher.update(self.field_count.to_be_bytes());

        let digest = self.hasher.finalize();
        let mut bytes = [0_u8; HDLIR_FINGERPRINT_SIZE];
        bytes.copy_from_slice(&digest);
        HdlirFingerprint(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fingerprint(domain: &str, fields: &[(&str, &[u8])]) -> HdlirFingerprint {
        let mut builder = FingerprintBuilder::new(domain).expect("valid test domain");

        for (tag, value) in fields {
            builder
                .write_field(tag, value)
                .expect("valid test field");
        }

        builder.finish()
    }

    #[test]
    fn same_domain_and_fields_produce_same_fingerprint() {
        let first = fingerprint("zamani.hdlir.module.v1", &[("name", b"top")]);
        let second = fingerprint("zamani.hdlir.module.v1", &[("name", b"top")]);

        assert_eq!(first, second);
        assert_eq!(first.to_hex(), first.to_string());
        assert_eq!(first.to_hex().len(), HDLIR_FINGERPRINT_SIZE * 2);
    }

    #[test]
    fn domain_separation_changes_fingerprint() {
        let first = fingerprint("zamani.hdlir.module.v1", &[("name", b"top")]);
        let second = fingerprint("zamani.hdlir.design.v1", &[("name", b"top")]);

        assert_ne!(first, second);
    }

    #[test]
    fn field_tag_and_value_are_part_of_the_fingerprint() {
        let first = fingerprint("test.domain", &[("name", b"top")]);
        let second = fingerprint("test.domain", &[("label", b"top")]);
        let third = fingerprint("test.domain", &[("name", b"other")]);

        assert_ne!(first, second);
        assert_ne!(first, third);
    }

    #[test]
    fn field_framing_prevents_concatenation_ambiguity() {
        let first = fingerprint(
            "test.domain",
            &[("first", b"ab"), ("second", b"c")],
        );
        let second = fingerprint(
            "test.domain",
            &[("first", b"a"), ("second", b"bc")],
        );

        assert_ne!(first, second);
    }

    #[test]
    fn field_order_is_significant() {
        let first = fingerprint(
            "test.domain",
            &[("alpha", b"1"), ("beta", b"2")],
        );
        let second = fingerprint(
            "test.domain",
            &[("beta", b"2"), ("alpha", b"1")],
        );

        assert_ne!(first, second);
    }

    #[test]
    fn empty_domain_is_rejected() {
        assert_eq!(
            FingerprintBuilder::new(""),
            Err(FingerprintError::EmptyDomain)
        );
    }

    #[test]
    fn empty_field_tag_is_rejected_without_changing_result() {
        let mut builder = FingerprintBuilder::new("test.domain").expect("valid domain");

        assert_eq!(
            builder.write_field("", b"value"),
            Err(FingerprintError::EmptyFieldTag)
        );

        let after_error = builder.finish();
        let empty_sequence = FingerprintBuilder::new("test.domain")
            .expect("valid domain")
            .finish();

        assert_eq!(after_error, empty_sequence);
    }

    #[test]
    fn integer_helpers_use_stable_encodings() {
        let mut first = FingerprintBuilder::new("test.numeric").expect("valid domain");
        first.write_u64("count", 0x0102_0304_0506_0708).expect("valid field");
        first.write_i64("offset", -1).expect("valid field");
        first.write_bool("enabled", true).expect("valid field");

        let mut second = FingerprintBuilder::new("test.numeric").expect("valid domain");
        second
            .write_field("count", &[1, 2, 3, 4, 5, 6, 7, 8])
            .expect("valid field");
        second
            .write_field("offset", &(-1_i64).to_be_bytes())
            .expect("valid field");
        second.write_field("enabled", &[1]).expect("valid field");

        assert_eq!(first.finish(), second.finish());
    }

    #[test]
    fn empty_and_nonempty_sequences_differ() {
        let empty = FingerprintBuilder::new("test.domain")
            .expect("valid domain")
            .finish();
        let nonempty = fingerprint("test.domain", &[("field", b"")]);

        assert_ne!(empty, nonempty);
    }

    #[test]
    fn fingerprint_bytes_round_trip() {
        let original = fingerprint("test.domain", &[("payload", b"data")]);
        let restored = HdlirFingerprint::from_bytes(*original.as_bytes());

        assert_eq!(original, restored);
        assert_eq!(original.as_ref(), restored.as_ref());
    }

    #[test]
    fn hexadecimal_display_is_lowercase_and_fixed_width() {
        let fingerprint = HdlirFingerprint::from_bytes([0xab; HDLIR_FINGERPRINT_SIZE]);

        assert_eq!(fingerprint.to_string(), "ab".repeat(HDLIR_FINGERPRINT_SIZE));
        assert_eq!(fingerprint.to_hex(), fingerprint.to_string());
    }
}
