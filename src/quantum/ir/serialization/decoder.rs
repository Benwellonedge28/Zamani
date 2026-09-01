//! Zamani Quantum IR — Canonical Decoder
//!
//! Production-grade, deterministic, bounded decoder for the canonical Zamani
//! Quantum IR payload representation.
//!
//! # Architectural role
//!
//! This module owns:
//!
//! - bounded byte consumption;
//! - canonical primitive decoding;
//! - checked integer conversion;
//! - checked collection/string/byte decoding;
//! - nesting-budget enforcement;
//! - canonical logical/physical qubit identity decoding;
//! - decoder cursor management;
//! - detection of unread payload bytes.
//!
//! This module does NOT own:
//!
//! - IR semantic validation;
//! - quantum-machine limits;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - optimization;
//! - simulation;
//! - QEC;
//! - frontend parsing;
//! - backend execution;
//! - serialization framing;
//! - checksum calculation;
//! - IR version policy.
//!
//! Those responsibilities remain outside this file.
//!
//! # Universal-program principle
//!
//! The decoder contains no architectural quantum-machine size.
//!
//! In particular, it does NOT define:
//!
//! - maximum qubits;
//! - maximum gates;
//! - maximum registers;
//! - maximum circuit depth;
//! - maximum topology size;
//! - maximum operation count.
//!
//! All finite limits are supplied through [`DecodeLimits`].
//!
//! A larger machine can therefore use a larger decode policy without changing
//! the IR wire schema.
//!
//! # Security boundary
//!
//! Serialized IR is untrusted input.
//!
//! Every operation which can cause an allocation or cursor movement therefore
//! follows:
//!
//! ```text
//! untrusted length
//!       │
//!       ▼
//! checked u64 → usize conversion
//!       │
//!       ▼
//! explicit policy check
//!       │
//!       ▼
//! remaining-input check
//!       │
//!       ▼
//! allocation
//! ```
//!
//! No allocation is performed from an unchecked wire length.
//!
//! This module contains no `unsafe` code and explicitly forbids unsafe code.
//!
//! # Canonical encoding assumptions
//!
//! The enclosing serialization module owns the document frame:
//!
//! ```text
//! magic
//! format version
//! IR version
//! payload length
//! checksum
//! payload
//! ```
//!
//! This decoder operates only on the already-isolated canonical payload.
//!
//! Framing validation belongs to `serialization::mod`.
//!
//! # Qubit identity boundary
//!
//! Logical and physical qubit identifiers are decoded using the canonical
//! definitions from:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module deliberately does not define replacement qubit types.
//!
//! # Integration contract
//!
//! `serialization/mod.rs` supplies:
//!
//! - [`DecodeLimits`];
//! - [`SerializationError`];
//! - [`IrDecode`];
//!
//! `quantum::ir::qubit` supplies:
//!
//! - [`QubitId`];
//! - [`PhysicalQubitId`];
//! - [`QubitRef`];
//!
//! Other IR modules consume [`Decoder`] through their implementations of
//! [`IrDecode`].
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
//! - no external dependencies;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the last requirement compiler-enforced.

#![forbid(unsafe_code)]

use std::convert::TryFrom;

use super::{DecodeLimits, SerializationError};
use super::IrDecode;

use crate::quantum::ir::qubit::{
    PhysicalQubitId,
    QubitId,
    QubitRef,
};

// =============================================================================
// Decoder
// =============================================================================

/// Cursor over an already-isolated canonical Quantum IR payload.
///
/// `Decoder` is intentionally small and stateful. Higher-level IR decoders
/// should consume values through this API rather than manipulating byte
/// offsets themselves.
///
/// # Invariants
///
/// At all times:
///
/// ```text
/// 0 <= position <= bytes.len()
/// ```
///
/// The decoder never advances the cursor unless the complete requested value
/// has been proven to exist.
///
/// Allocation-producing operations validate their lengths before allocation.
///
/// The decoder does not perform semantic IR validation. A successfully
/// decoded object must still be passed through the canonical IR validator.
#[derive(Debug)]
pub struct Decoder<'a> {
    bytes: &'a [u8],
    position: usize,
    limits: DecodeLimits,
    nesting_depth: u64,
}

impl<'a> Decoder<'a> {
    /// Creates a decoder using the default serialization policy.
    #[must_use]
    pub fn new(bytes: &'a [u8]) -> Self {
        Self::with_limits(bytes, DecodeLimits::default())
    }

    /// Creates a decoder using an explicit decoding policy.
    ///
    /// This constructor is infallible because the policy itself is validated
    /// when decoding begins through [`Self::validate_limits`].
    #[must_use]
    pub fn with_limits(
        bytes: &'a [u8],
        limits: DecodeLimits,
    ) -> Self {
        Self {
            bytes,
            position: 0,
            limits,
            nesting_depth: 0,
        }
    }

    /// Returns the complete input payload.
    #[must_use]
    pub fn input(&self) -> &'a [u8] {
        self.bytes
    }

    /// Returns the current cursor position.
    #[must_use]
    pub fn position(&self) -> usize {
        self.position
    }

    /// Returns the number of unread bytes.
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.position)
    }

    /// Returns whether the payload has been completely consumed.
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.position == self.bytes.len()
    }

    /// Returns the active decode limits.
    #[must_use]
    pub const fn limits(&self) -> DecodeLimits {
        self.limits
    }

    /// Returns the current recursive nesting depth.
    #[must_use]
    pub const fn nesting_depth(&self) -> u64 {
        self.nesting_depth
    }

    /// Validates the configured decode policy.
    pub fn validate_limits(&self) -> Result<(), SerializationError> {
        self.limits.validate()
    }

    // -------------------------------------------------------------------------
    // Completion
    // -------------------------------------------------------------------------

    /// Finishes decoding and requires complete payload consumption.
    ///
    /// This prevents malformed payloads from hiding arbitrary trailing data
    /// after a successfully decoded object.
    pub fn finish(&self) -> Result<(), SerializationError> {
        self.validate_limits()?;

        if self.position != self.bytes.len() {
            return Err(SerializationError::TrailingBytes {
                count: self.bytes.len() - self.position,
            });
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Bounds
    // -------------------------------------------------------------------------

    /// Ensures that `count` bytes remain without advancing the cursor.
    fn ensure_remaining(
        &self,
        count: usize,
    ) -> Result<(), SerializationError> {
        let remaining = self.remaining();

        if count > remaining {
            return Err(SerializationError::UnexpectedEnd {
                needed: count,
                available: remaining,
            });
        }

        Ok(())
    }

    /// Advances the cursor by exactly `count` bytes.
    ///
    /// The returned slice is borrowed directly from the input payload.
    fn take_exact(
        &mut self,
        count: usize,
    ) -> Result<&'a [u8], SerializationError> {
        self.ensure_remaining(count)?;

        let start = self.position;

        let end = start
            .checked_add(count)
            .ok_or(SerializationError::ArithmeticOverflow {
                context: "decoder cursor advancement",
            })?;

        self.position = end;

        Ok(&self.bytes[start..end])
    }

    // -------------------------------------------------------------------------
    // Nesting
    // -------------------------------------------------------------------------

    /// Executes a decoding operation under one additional nesting level.
    ///
    /// A closure is used rather than a manually managed guard so that the
    /// nesting counter is restored on both success and error paths without
    /// requiring unsafe aliasing tricks.
    pub fn with_nesting<T, F>(
        &mut self,
        operation: F,
    ) -> Result<T, SerializationError>
    where
        F: FnOnce(&mut Self) -> Result<T, SerializationError>,
    {
        let next_depth = self
            .nesting_depth
            .checked_add(1)
            .ok_or(SerializationError::ArithmeticOverflow {
                context: "decoder nesting depth",
            })?;

        if next_depth > self.limits.max_nesting_depth {
            return Err(SerializationError::NestingLimitExceeded {
                requested: next_depth,
                maximum: self.limits.max_nesting_depth,
            });
        }

        self.nesting_depth = next_depth;

        let result = operation(self);

        // This cannot underflow because this method incremented the value
        // immediately before invoking the callback.
        self.nesting_depth -= 1;

        result
    }

    // -------------------------------------------------------------------------
    // Raw byte primitives
    // -------------------------------------------------------------------------

    /// Reads one byte.
    pub fn read_u8(&mut self) -> Result<u8, SerializationError> {
        Ok(self.take_exact(1)?[0])
    }

    /// Reads a canonical boolean.
    ///
    /// Only `0` and `1` are valid.
    pub fn read_bool(&mut self) -> Result<bool, SerializationError> {
        match self.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            value => Err(SerializationError::InvalidBoolean { value }),
        }
    }

    /// Reads an exact borrowed byte slice.
    pub fn read_raw(
        &mut self,
        count: usize,
    ) -> Result<&'a [u8], SerializationError> {
        self.take_exact(count)
    }

    /// Reads a fixed-size array.
    pub fn read_array<const N: usize>(
        &mut self,
    ) -> Result<[u8; N], SerializationError> {
        let bytes = self.take_exact(N)?;

        let mut result = [0u8; N];

        result.copy_from_slice(bytes);

        Ok(result)
    }

    // -------------------------------------------------------------------------
    // Unsigned integers
    // -------------------------------------------------------------------------

    /// Reads a little-endian `u16`.
    pub fn read_u16(&mut self) -> Result<u16, SerializationError> {
        Ok(u16::from_le_bytes(self.read_array()?))
    }

    /// Reads a little-endian `u32`.
    pub fn read_u32(&mut self) -> Result<u32, SerializationError> {
        Ok(u32::from_le_bytes(self.read_array()?))
    }

    /// Reads a little-endian `u64`.
    pub fn read_u64(&mut self) -> Result<u64, SerializationError> {
        Ok(u64::from_le_bytes(self.read_array()?))
    }

    /// Reads a little-endian `u128`.
    pub fn read_u128(&mut self) -> Result<u128, SerializationError> {
        Ok(u128::from_le_bytes(self.read_array()?))
    }

    // -------------------------------------------------------------------------
    // Signed integers
    // -------------------------------------------------------------------------

    /// Reads a little-endian `i8`.
    pub fn read_i8(&mut self) -> Result<i8, SerializationError> {
        Ok(i8::from_le_bytes(self.read_array()?))
    }

    /// Reads a little-endian `i16`.
    pub fn read_i16(&mut self) -> Result<i16, SerializationError> {
        Ok(i16::from_le_bytes(self.read_array()?))
    }

    /// Reads a little-endian `i32`.
    pub fn read_i32(&mut self) -> Result<i32, SerializationError> {
        Ok(i32::from_le_bytes(self.read_array()?))
    }

    /// Reads a little-endian `i64`.
    pub fn read_i64(&mut self) -> Result<i64, SerializationError> {
        Ok(i64::from_le_bytes(self.read_array()?))
    }

    /// Reads a little-endian `i128`.
    pub fn read_i128(&mut self) -> Result<i128, SerializationError> {
        Ok(i128::from_le_bytes(self.read_array()?))
    }

    // -------------------------------------------------------------------------
    // Floating-point values
    // -------------------------------------------------------------------------

    /// Reads an IEEE-754 `f32` encoded as little-endian bits.
    pub fn read_f32(&mut self) -> Result<f32, SerializationError> {
        Ok(f32::from_bits(self.read_u32()?))
    }

    /// Reads an IEEE-754 `f64` encoded as little-endian bits.
    pub fn read_f64(&mut self) -> Result<f64, SerializationError> {
        Ok(f64::from_bits(self.read_u64()?))
    }

    // -------------------------------------------------------------------------
    // Host-size conversion
    // -------------------------------------------------------------------------

    /// Reads a wire `u64` and converts it to `usize`.
    ///
    /// The wire format remains platform-independent because it stores `u64`.
    /// Conversion to the host representation is performed only at the point
    /// where the Rust API requires a `usize`.
    pub fn read_usize(
        &mut self,
        context: &'static str,
    ) -> Result<usize, SerializationError> {
        let value = self.read_u64()?;

        usize::try_from(value).map_err(|_| {
            SerializationError::LengthOverflow {
                context,
                value,
            }
        })
    }

    /// Reads a wire collection length and validates it against the configured
    /// collection policy.
    pub fn read_collection_len(
        &mut self,
        context: &'static str,
    ) -> Result<usize, SerializationError> {
        let count = self.read_u64()?;

        if count > self.limits.max_collection_elements {
            return Err(
                SerializationError::CollectionLimitExceeded {
                    context,
                    requested: count,
                    maximum: self.limits.max_collection_elements,
                },
            );
        }

        usize::try_from(count).map_err(|_| {
            SerializationError::LengthOverflow {
                context,
                value: count,
            }
        })
    }

    /// Reads a wire byte/string field length and validates it against the
    /// configured field policy.
    pub fn read_field_len(
        &mut self,
        context: &'static str,
    ) -> Result<usize, SerializationError> {
        let length = self.read_u64()?;

        if length > self.limits.max_field_bytes {
            return Err(SerializationError::FieldLimitExceeded {
                context,
                requested: length,
                maximum: self.limits.max_field_bytes,
            });
        }

        usize::try_from(length).map_err(|_| {
            SerializationError::LengthOverflow {
                context,
                value: length,
            }
        })
    }

    // -------------------------------------------------------------------------
    // Byte and string fields
    // -------------------------------------------------------------------------

    /// Reads a length-prefixed byte vector.
    ///
    /// The wire length is checked against both the field policy and the
    /// remaining payload before allocation.
    pub fn read_bytes(
        &mut self,
        context: &'static str,
    ) -> Result<Vec<u8>, SerializationError> {
        let length = self.read_field_len(context)?;

        self.ensure_remaining(length)?;

        let bytes = self.take_exact(length)?;

        // Allocation occurs only after:
        //
        // 1. u64 -> usize conversion;
        // 2. policy validation;
        // 3. remaining-input validation.
        Ok(bytes.to_vec())
    }

    /// Reads a UTF-8 string.
    pub fn read_string(
        &mut self,
        context: &'static str,
    ) -> Result<String, SerializationError> {
        let bytes = self.read_bytes(context)?;

        String::from_utf8(bytes)
            .map_err(|_| SerializationError::InvalidUtf8)
    }

    /// Reads an optional UTF-8 string encoded as:
    ///
    /// ```text
    /// 0 = None
    /// 1 = Some(length + bytes)
    /// ```
    pub fn read_optional_string(
        &mut self,
        context: &'static str,
    ) -> Result<Option<String>, SerializationError> {
        match self.read_bool()? {
            false => Ok(None),
            true => Ok(Some(self.read_string(context)?)),
        }
    }

    /// Reads an optional byte vector.
    pub fn read_optional_bytes(
        &mut self,
        context: &'static str,
    ) -> Result<Option<Vec<u8>>, SerializationError> {
        match self.read_bool()? {
            false => Ok(None),
            true => Ok(Some(self.read_bytes(context)?)),
        }
    }

    // -------------------------------------------------------------------------
    // Generic collections
    // -------------------------------------------------------------------------

    /// Reads a length-prefixed vector.
    ///
    /// The element count is validated before the vector is allocated.
    pub fn read_vec<T, F>(
        &mut self,
        context: &'static str,
        mut decode_element: F,
    ) -> Result<Vec<T>, SerializationError>
    where
        F: FnMut(&mut Self) -> Result<T, SerializationError>,
    {
        self.with_nesting(|decoder| {
            let count = decoder.read_collection_len(context)?;

            // `count` is already policy-checked and host-convertible.
            //
            // We still avoid blindly allocating a huge vector. Rust's
            // `Vec::with_capacity` may fail by aborting on allocation failure
            // rather than returning a Result, so the decoder deliberately
            // starts empty and grows only as elements are successfully decoded.
            let mut values = Vec::new();

            for _ in 0..count {
                values.push(decode_element(decoder)?);
            }

            Ok(values)
        })
    }

    /// Reads an optional value encoded as a canonical boolean followed by the
    /// value when present.
    pub fn read_option<T, F>(
        &mut self,
        decode_value: F,
    ) -> Result<Option<T>, SerializationError>
    where
        F: FnOnce(&mut Self) -> Result<T, SerializationError>,
    {
        if self.read_bool()? {
            Ok(Some(decode_value(self)?))
        } else {
            Ok(None)
        }
    }

    // -------------------------------------------------------------------------
    // Discriminants
    // -------------------------------------------------------------------------

    /// Reads an unsigned discriminant.
    ///
    /// The caller remains responsible for interpreting the discriminant as
    /// its concrete enum.
    pub fn read_discriminant(
        &mut self,
        _type_name: &'static str,
    ) -> Result<u64, SerializationError> {
        self.read_u64()
    }

    /// Reads a `u8` discriminant and validates it against the supplied maximum
    /// inclusive value.
    pub fn read_u8_discriminant(
        &mut self,
        type_name: &'static str,
        maximum: u8,
    ) -> Result<u8, SerializationError> {
        let value = self.read_u8()?;

        if value > maximum {
            return Err(SerializationError::InvalidDiscriminant {
                type_name,
                value: u64::from(value),
            });
        }

        Ok(value)
    }

    // -------------------------------------------------------------------------
    // Canonical qubit identities
    // -------------------------------------------------------------------------

    /// Decodes the canonical logical qubit identity.
    ///
    /// The wire representation is always `u64`, while the canonical
    /// `QubitId` currently stores a host `usize`. The conversion is therefore
    /// checked and never silently truncates on narrower targets.
    pub fn read_qubit_id(
        &mut self,
    ) -> Result<QubitId, SerializationError> {
        let index = self.read_usize("logical qubit identity")?;

        Ok(QubitId::new(index))
    }

    /// Decodes the canonical physical qubit identity.
    pub fn read_physical_qubit_id(
        &mut self,
    ) -> Result<PhysicalQubitId, SerializationError> {
        let index = self.read_usize("physical qubit identity")?;

        Ok(PhysicalQubitId::new(index))
    }

    /// Decodes an explicitly tagged logical/physical qubit reference.
    ///
    /// Canonical encoding:
    ///
    /// ```text
    /// 0 = Logical(QubitId)
    /// 1 = Physical(PhysicalQubitId)
    /// ```
    pub fn read_qubit_ref(
        &mut self,
    ) -> Result<QubitRef, SerializationError> {
        match self.read_u8()? {
            0 => Ok(QubitRef::Logical(self.read_qubit_id()?)),

            1 => Ok(QubitRef::Physical(
                self.read_physical_qubit_id()?,
            )),

            value => Err(SerializationError::InvalidDiscriminant {
                type_name: "QubitRef",
                value: u64::from(value),
            }),
        }
    }

    // -------------------------------------------------------------------------
    // Generic IR decoding
    // -------------------------------------------------------------------------

    /// Delegates decoding to an [`IrDecode`] implementation.
    ///
    /// This method exists to keep higher-level IR decoders uniform and to
    /// provide one canonical entry point for nested semantic objects.
    pub fn decode<T>(&mut self) -> Result<T, SerializationError>
    where
        T: IrDecode,
    {
        T::decode(self)
    }

    // -------------------------------------------------------------------------
    // Cursor utilities
    // -------------------------------------------------------------------------

    /// Skips exactly `count` bytes after validating their existence.
    ///
    /// This should only be used for fields whose semantics are explicitly
    /// defined as opaque bytes.
    pub fn skip(
        &mut self,
        count: usize,
    ) -> Result<(), SerializationError> {
        let _ = self.take_exact(count)?;

        Ok(())
    }

    /// Returns a borrowed view of the unread payload.
    #[must_use]
    pub fn remaining_bytes(&self) -> &'a [u8] {
        &self.bytes[self.position..]
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> DecodeLimits {
        DecodeLimits::new(
            1024 * 1024,
            1024 * 1024,
            64 * 1024,
            1024,
            64,
        )
    }

    #[test]
    fn reads_little_endian_integers() {
        let bytes = [
            0x34, 0x12, //
            0x78, 0x56, 0x34, 0x12, //
            0xef, 0xcd, 0xab, 0x89,
            0x67, 0x45, 0x23, 0x01,
            0x00, 0x00,
            0x00, 0x00,
        ];

        let mut decoder = Decoder::with_limits(&bytes, limits());

        assert_eq!(decoder.read_u16().unwrap(), 0x1234);
        assert_eq!(decoder.read_u32().unwrap(), 0x1234_5678);
        assert_eq!(
            decoder.read_u64().unwrap(),
            0x0123_4567_89ab_cdef
        );

        assert!(decoder.is_finished());
    }

    #[test]
    fn rejects_noncanonical_boolean() {
        let bytes = [2u8];

        let mut decoder = Decoder::with_limits(&bytes, limits());

        assert_eq!(
            decoder.read_bool(),
            Err(SerializationError::InvalidBoolean { value: 2 })
        );
    }

    #[test]
    fn rejects_truncated_value() {
        let bytes = [1u8, 2u8, 3u8];

        let mut decoder = Decoder::with_limits(&bytes, limits());

        let result = decoder.read_u64();

        assert!(matches!(
            result,
            Err(SerializationError::UnexpectedEnd {
                needed: 8,
                available: 3
            })
        ));
    }

    #[test]
    fn rejects_field_above_policy_before_allocation() {
        let bytes = [
            0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let policy = DecodeLimits::new(
            1024,
            1024,
            1024,
            1024,
            64,
        );

        let mut decoder =
            Decoder::with_limits(&bytes, policy);

        assert!(matches!(
            decoder.read_bytes("test field"),
            Err(SerializationError::FieldLimitExceeded {
                context: "test field",
                requested: 4096,
                maximum: 1024
            })
        ));
    }

    #[test]
    fn rejects_collection_above_policy() {
        let bytes = [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f,
        ];

        let policy = DecodeLimits::new(
            1024,
            1024,
            1024,
            16,
            64,
        );

        let mut decoder =
            Decoder::with_limits(&bytes, policy);

        assert!(matches!(
            decoder.read_collection_len("test collection"),
            Err(SerializationError::CollectionLimitExceeded {
                context: "test collection",
                requested: _,
                maximum: 16
            })
        ));
    }

    #[test]
    fn rejects_trailing_payload() {
        let bytes = [42u8, 99u8];

        let mut decoder = Decoder::with_limits(&bytes, limits());

        assert_eq!(decoder.read_u8().unwrap(), 42);

        assert!(matches!(
            decoder.finish(),
            Err(SerializationError::TrailingBytes { count: 1 })
        ));
    }

    #[test]
    fn decodes_logical_qubit_using_canonical_type() {
        let bytes = 7u64.to_le_bytes();

        let mut decoder =
            Decoder::with_limits(&bytes, limits());

        let qubit = decoder.read_qubit_id().unwrap();

        assert_eq!(qubit, QubitId::new(7));
    }

    #[test]
    fn decodes_physical_qubit_using_canonical_type() {
        let bytes = 13u64.to_le_bytes();

        let mut decoder =
            Decoder::with_limits(&bytes, limits());

        let qubit =
            decoder.read_physical_qubit_id().unwrap();

        assert_eq!(
            qubit,
            PhysicalQubitId::new(13)
        );
    }

    #[test]
    fn decodes_logical_qubit_ref() {
        let mut bytes = Vec::new();

        bytes.push(0);
        bytes.extend_from_slice(&5u64.to_le_bytes());

        let mut decoder =
            Decoder::with_limits(&bytes, limits());

        assert_eq!(
            decoder.read_qubit_ref().unwrap(),
            QubitRef::Logical(QubitId::new(5))
        );
    }

    #[test]
    fn decodes_physical_qubit_ref() {
        let mut bytes = Vec::new();

        bytes.push(1);
        bytes.extend_from_slice(&9u64.to_le_bytes());

        let mut decoder =
            Decoder::with_limits(&bytes, limits());

        assert_eq!(
            decoder.read_qubit_ref().unwrap(),
            QubitRef::Physical(
                PhysicalQubitId::new(9)
            )
        );
    }

    #[test]
    fn rejects_invalid_qubit_ref_discriminant() {
        let bytes = [2u8];

        let mut decoder =
            Decoder::with_limits(&bytes, limits());

        assert!(matches!(
            decoder.read_qubit_ref(),
            Err(SerializationError::InvalidDiscriminant {
                type_name: "QubitRef",
                value: 2
            })
        ));
    }

    #[test]
    fn reads_utf8_string() {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&5u64.to_le_bytes());
        bytes.extend_from_slice(b"Zamani");

        // Correct the length to six bytes.
        bytes[0..8].copy_from_slice(&6u64.to_le_bytes());

        let mut decoder =
            Decoder::with_limits(&bytes, limits());

        assert_eq!(
            decoder.read_string("name").unwrap(),
            "Zamani"
        );
    }

    #[test]
    fn rejects_invalid_utf8() {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&2u64.to_le_bytes());
        bytes.extend_from_slice(&[0xff, 0xff]);

        let mut decoder =
            Decoder::with_limits(&bytes, limits());

        assert_eq!(
            decoder.read_string("name"),
            Err(SerializationError::InvalidUtf8)
        );
    }

    #[test]
    fn optional_values_are_canonical() {
        let none = [0u8];

        let mut decoder =
            Decoder::with_limits(&none, limits());

        assert_eq!(
            decoder
                .read_option(|_| Ok::<u64, SerializationError>(7))
                .unwrap(),
            None
        );

        let some = [1u8, 7u8];

        let mut decoder =
            Decoder::with_limits(&some, limits());

        assert_eq!(
            decoder
                .read_option(|decoder| decoder.read_u8())
                .unwrap(),
            Some(7)
        );
    }

    #[test]
    fn nesting_limit_is_enforced() {
        let policy = DecodeLimits::new(
            1024,
            1024,
            1024,
            1024,
            1,
        );

        let bytes = [0u8];

        let mut decoder =
            Decoder::with_limits(&bytes, policy);

        let result = decoder.with_nesting(|decoder| {
            decoder.with_nesting(|_| Ok::<(), SerializationError>(()))
        });

        assert!(matches!(
            result,
            Err(SerializationError::NestingLimitExceeded {
                requested: 2,
                maximum: 1
            })
        ));
    }

    #[test]
    fn nesting_depth_is_restored_after_error() {
        let policy = DecodeLimits::new(
            1024,
            1024,
            1024,
            1024,
            8,
        );

        let bytes = [0u8];

        let mut decoder =
            Decoder::with_limits(&bytes, policy);

        let result = decoder.with_nesting(|_| {
            Err::<(), SerializationError>(
                SerializationError::Codec {
                    message: "expected test failure".to_owned(),
                },
            )
        });

        assert!(result.is_err());
        assert_eq!(decoder.nesting_depth(), 0);
    }

    #[test]
    fn vector_decoder_does_not_preallocate_from_untrusted_length() {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&3u64.to_le_bytes());
        bytes.push(1);
        bytes.push(2);
        bytes.push(3);

        let mut decoder =
            Decoder::with_limits(&bytes, limits());

        let values = decoder
            .read_vec("bytes", |decoder| {
                decoder.read_u8()
            })
            .unwrap();

        assert_eq!(values, vec![1, 2, 3]);
        assert!(decoder.is_finished());
    }
}