//! Zamani Quantum IR — Canonical Payload Encoder
//!
//! Production-grade deterministic payload encoder for the canonical Zamani
//! Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! This module owns:
//!
//! - canonical primitive encoding;
//! - canonical length encoding;
//! - canonical logical/physical qubit encoding;
//! - canonical IR-version encoding;
//! - deterministic sequence encoding;
//! - deterministic entry/map-like encoding;
//! - checked output growth;
//! - explicit encoding resource policy;
//! - no-unsafe, fallible allocation boundaries.
//!
//! It does NOT own:
//!
//! - document framing;
//! - magic/version framing;
//! - payload checksum;
//! - decoding;
//! - semantic validation;
//! - hashing;
//! - gate semantics;
//! - operation semantics;
//! - hardware;
//! - routing;
//! - scheduling;
//! - simulation;
//! - QEC;
//! - frontend parsing;
//! - backend execution.
//!
//! Those responsibilities belong to sibling IR modules.
//!
//! # Canonical dependency boundary
//!
//! ```text
//! quantum::ir::core/value
//! quantum::ir::identity
//! quantum::ir::qubit
//!             │
//!             ▼
//!      serialization::encoder
//!             │
//!             ▼
//!      canonical payload
//!             │
//!             ▼
//!      serialization framing
//!             │
//!             ▼
//!      SerializedIr
//! ```
//!
//! The encoder is deliberately below the framing layer.
//!
//! # Universal-program principle
//!
//! This module contains NO fixed quantum-machine size.
//!
//! It does not contain:
//!
//! - maximum qubit count;
//! - maximum register count;
//! - maximum operation count;
//! - maximum gate count;
//! - maximum topology size;
//! - vendor-specific limits.
//!
//! The encoder can represent any finite payload that:
//!
//! 1. can be represented by the wire format;
//! 2. can be represented by the host process;
//! 3. can be allocated by the selected allocator;
//! 4. is permitted by an explicitly supplied [`EncodeLimits`] policy.
//!
//! "Infinite" is therefore not represented literally. The architecture has no
//! semantic finite quantum-machine ceiling; actual serialization is bounded by
//! available resources and the selected execution policy.
//!
//! # Determinism
//!
//! The encoder guarantees deterministic byte order for all primitives:
//!
//! - integers: little-endian;
//! - booleans: `0` / `1`;
//! - floating point: exact IEEE-754 bit representation after semantic
//!   finiteness validation;
//! - lengths: unsigned 64-bit little-endian;
//! - sequences: caller-provided semantic order;
//! - maps/entries: caller-provided canonical order.
//!
//! This module intentionally does NOT sort arbitrary sequences because sequence
//! ordering can be semantically meaningful.
//!
//! # Allocation safety
//!
//! Output allocation is performed through [`Vec::try_reserve`].
//!
//! This means capacity overflow and allocator failure become explicit
//! [`SerializationError::Codec`] failures rather than relying on unchecked
//! `reserve` behavior.
//!
//! The module contains no `unsafe` code.
//!
//! # Qubit identity boundary
//!
//! Logical and physical qubit identifiers are imported exclusively from:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module MUST NOT define duplicate qubit identity types.
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
//! `serialization/mod.rs` or the future serialization parent module should
//! expose this module as:
//!
//! ```text
//! pub mod encoder;
//! pub use encoder::{EncodeLimits, Encoder};
//! ```
//!
//! The parent serialization module owns:
//!
//! ```text
//! SerializationError
//! IrEncode
//! IrDecode
//! SerializedIr
//! ```
//!
//! Higher-level IR types implement `IrEncode` by calling the methods on
//! [`Encoder`].
//!
//! Example:
//!
//! ```ignore
//! impl IrEncode for QubitRef {
//!     fn encode(
//!         &self,
//!         encoder: &mut Encoder,
//!     ) -> Result<(), SerializationError> {
//!         match self {
//!             QubitRef::Logical(id) => {
//!                 encoder.write_u8(0);
//!                 encoder.write_qubit_id(*id)
//!             }
//!             QubitRef::Physical(id) => {
//!                 encoder.write_u8(1);
//!                 encoder.write_physical_qubit_id(*id)
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! # Important integration rule
//!
//! The encoder never imports the complete program, gate, operation, pulse,
//! waveform, or hardware models. Higher-level modules depend on this module,
//! not the reverse.
//!
//! This prevents the serialization layer from becoming coupled to the entire
//! quantum compiler.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::convert::TryFrom;

use super::super::identity::IrVersion;
use super::super::qubit::{PhysicalQubitId, QubitId};
use super::SerializationError;

// =============================================================================
// Encoding policy
// =============================================================================

/// Explicit resource policy for canonical payload encoding.
///
/// This is a compiler/service resource policy, NOT a quantum-machine limit.
///
/// `None` means that the encoder does not impose an application-level payload
/// ceiling. The actual host process and allocator remain finite.
///
/// A caller serving untrusted or multi-tenant workloads should normally supply
/// an explicit limit.
///
/// A trusted local compiler can use [`EncodeLimits::unlimited`] when the
/// enclosing compilation policy already controls resource consumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncodeLimits {
    /// Maximum payload size permitted by this encoder.
    ///
    /// `None` means no application-level payload limit.
    pub max_payload_bytes: Option<u64>,

    /// Maximum size of one byte/string field.
    ///
    /// `None` means no application-level field limit.
    pub max_field_bytes: Option<u64>,

    /// Maximum number of elements in one sequence.
    ///
    /// `None` means no application-level collection limit.
    pub max_collection_elements: Option<u64>,
}

impl EncodeLimits {
    /// Creates an unrestricted encoding policy.
    ///
    /// This does NOT mean infinite memory. The host address space and allocator
    /// remain the actual physical/resource boundaries.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_payload_bytes: None,
            max_field_bytes: None,
            max_collection_elements: None,
        }
    }

    /// Creates an explicit encoding policy.
    #[must_use]
    pub const fn new(
        max_payload_bytes: Option<u64>,
        max_field_bytes: Option<u64>,
        max_collection_elements: Option<u64>,
    ) -> Self {
        Self {
            max_payload_bytes,
            max_field_bytes,
            max_collection_elements,
        }
    }

    /// Creates a conservative policy suitable for service boundaries.
    ///
    /// These values are security/resource defaults only.
    ///
    /// They are NOT:
    ///
    /// - quantum-machine limits;
    /// - qubit limits;
    /// - program semantic limits.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            max_payload_bytes: Some(256 * 1024 * 1024),
            max_field_bytes: Some(16 * 1024 * 1024),
            max_collection_elements: Some(16 * 1024 * 1024),
        }
    }

    /// Validates the policy itself.
    pub fn validate(self) -> Result<(), SerializationError> {
        if let Some(value) = self.max_payload_bytes {
            if value == 0 {
                return Err(SerializationError::InvalidDecodeLimits {
                    field: "max_payload_bytes",
                });
            }
        }

        if let Some(value) = self.max_field_bytes {
            if value == 0 {
                return Err(SerializationError::InvalidDecodeLimits {
                    field: "max_field_bytes",
                });
            }
        }

        if let Some(value) = self.max_collection_elements {
            if value == 0 {
                return Err(SerializationError::InvalidDecodeLimits {
                    field: "max_collection_elements",
                });
            }
        }

        if let (
            Some(max_payload_bytes),
            Some(max_field_bytes),
        ) = (
            self.max_payload_bytes,
            self.max_field_bytes,
        ) {
            if max_field_bytes > max_payload_bytes {
                return Err(SerializationError::InvalidDecodeLimits {
                    field: "max_field_bytes",
                });
            }
        }

        Ok(())
    }
}

impl Default for EncodeLimits {
    fn default() -> Self {
        Self::unlimited()
    }
}

// =============================================================================
// Encoder
// =============================================================================

/// Canonical deterministic Quantum IR payload encoder.
///
/// `Encoder` produces payload bytes only. Document framing belongs to the
/// parent serialization layer.
///
/// The encoder is intentionally independent from:
///
/// - program representation;
/// - operation representation;
/// - hardware;
/// - target mapping;
/// - scheduling;
/// - execution.
///
/// It is therefore reusable by every IR dialect.
#[derive(Debug, Default)]
pub struct Encoder {
    bytes: Vec<u8>,
    limits: EncodeLimits,
}

impl Encoder {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates an unrestricted encoder.
    ///
    /// This is the canonical constructor used by trusted serialization paths.
    #[must_use]
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            limits: EncodeLimits::unlimited(),
        }
    }

    /// Creates an encoder using an explicit resource policy.
    pub fn with_limits(
        limits: EncodeLimits,
    ) -> Result<Self, SerializationError> {
        limits.validate()?;

        Ok(Self {
            bytes: Vec::new(),
            limits,
        })
    }

    /// Creates an encoder with an allocation hint.
    ///
    /// The hint is not a semantic limit and does not affect the wire format.
    ///
    /// Allocation is still performed through fallible reservation.
    pub fn with_capacity(
        capacity: usize,
    ) -> Result<Self, SerializationError> {
        Self::with_capacity_and_limits(
            capacity,
            EncodeLimits::unlimited(),
        )
    }

    /// Creates an encoder with an allocation hint and explicit policy.
    pub fn with_capacity_and_limits(
        capacity: usize,
        limits: EncodeLimits,
    ) -> Result<Self, SerializationError> {
        limits.validate()?;

        let capacity_u64 =
            u64::try_from(capacity).map_err(|_| {
                SerializationError::LengthOverflow {
                    context: "encoder capacity",
                    value: u64::MAX,
                }
            })?;

        if let Some(maximum) = limits.max_payload_bytes {
            if capacity_u64 > maximum {
                return Err(
                    SerializationError::PayloadTooLarge {
                        size: capacity_u64,
                        maximum,
                    },
                );
            }
        }

        let mut bytes = Vec::new();

        bytes.try_reserve_exact(capacity).map_err(|error| {
            SerializationError::Codec {
                message: format!(
                    "unable to reserve {capacity} bytes for encoder: {error}"
                ),
            }
        })?;

        Ok(Self {
            bytes,
            limits,
        })
    }

    // =========================================================================
    // Inspection
    // =========================================================================

    /// Returns the currently encoded payload length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the payload is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns the currently encoded bytes without consuming the encoder.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consumes the encoder and returns the payload.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Returns the active encoding policy.
    #[must_use]
    pub const fn limits(&self) -> EncodeLimits {
        self.limits
    }

    // =========================================================================
    // Capacity management
    // =========================================================================

    /// Checks whether adding `additional` bytes is permitted and representable.
    fn check_growth(
        &self,
        additional: usize,
        context: &'static str,
    ) -> Result<(), SerializationError> {
        let current =
            u64::try_from(self.bytes.len()).map_err(|_| {
                SerializationError::LengthOverflow {
                    context: "encoder current length",
                    value: u64::MAX,
                }
            })?;

        let additional_u64 =
            u64::try_from(additional).map_err(|_| {
                SerializationError::LengthOverflow {
                    context,
                    value: u64::MAX,
                }
            })?;

        let requested =
            current.checked_add(additional_u64).ok_or(
                SerializationError::ArithmeticOverflow {
                    context: "encoder payload length",
                },
            )?;

        if let Some(maximum) = self.limits.max_payload_bytes {
            if requested > maximum {
                return Err(
                    SerializationError::PayloadTooLarge {
                        size: requested,
                        maximum,
                    },
                );
            }
        }

        Ok(())
    }

    /// Reserves exactly enough capacity for the next write.
    fn reserve(
        &mut self,
        additional: usize,
        context: &'static str,
    ) -> Result<(), SerializationError> {
        if additional == 0 {
            return Ok(());
        }

        self.check_growth(additional, context)?;

        self.bytes
            .try_reserve(additional)
            .map_err(|error| SerializationError::Codec {
                message: format!(
                    "unable to reserve {additional} bytes for {context}: {error}"
                ),
            })
    }

    /// Appends raw bytes after applying the encoder's allocation policy.
    fn write_raw(
        &mut self,
        bytes: &[u8],
        context: &'static str,
    ) -> Result<(), SerializationError> {
        self.reserve(bytes.len(), context)?;
        self.bytes.extend_from_slice(bytes);
        Ok(())
    }

    // =========================================================================
    // Primitive integers
    // =========================================================================

    /// Writes one byte.
    pub fn write_u8(
        &mut self,
        value: u8,
    ) -> Result<(), SerializationError> {
        self.write_raw(
            &[value],
            "u8",
        )
    }

    /// Writes a canonical boolean.
    ///
    /// `false` = `0`
    ///
    /// `true` = `1`
    pub fn write_bool(
        &mut self,
        value: bool,
    ) -> Result<(), SerializationError> {
        self.write_u8(if value { 1 } else { 0 })
    }

    /// Writes a little-endian unsigned 16-bit integer.
    pub fn write_u16(
        &mut self,
        value: u16,
    ) -> Result<(), SerializationError> {
        self.write_raw(
            &value.to_le_bytes(),
            "u16",
        )
    }

    /// Writes a little-endian unsigned 32-bit integer.
    pub fn write_u32(
        &mut self,
        value: u32,
    ) -> Result<(), SerializationError> {
        self.write_raw(
            &value.to_le_bytes(),
            "u32",
        )
    }

    /// Writes a little-endian unsigned 64-bit integer.
    pub fn write_u64(
        &mut self,
        value: u64,
    ) -> Result<(), SerializationError> {
        self.write_raw(
            &value.to_le_bytes(),
            "u64",
        )
    }

    /// Writes a little-endian signed 8-bit integer.
    pub fn write_i8(
        &mut self,
        value: i8,
    ) -> Result<(), SerializationError> {
        self.write_u8(value as u8)
    }

    /// Writes a little-endian signed 16-bit integer.
    pub fn write_i16(
        &mut self,
        value: i16,
    ) -> Result<(), SerializationError> {
        self.write_raw(
            &value.to_le_bytes(),
            "i16",
        )
    }

    /// Writes a little-endian signed 32-bit integer.
    pub fn write_i32(
        &mut self,
        value: i32,
    ) -> Result<(), SerializationError> {
        self.write_raw(
            &value.to_le_bytes(),
            "i32",
        )
    }

    /// Writes a little-endian signed 64-bit integer.
    pub fn write_i64(
        &mut self,
        value: i64,
    ) -> Result<(), SerializationError> {
        self.write_raw(
            &value.to_le_bytes(),
            "i64",
        )
    }

    /// Writes a little-endian signed 128-bit integer.
    ///
    /// This is useful for the canonical IR value system, which uses `i128`
    /// semantic integers.
    pub fn write_i128(
        &mut self,
        value: i128,
    ) -> Result<(), SerializationError> {
        self.write_raw(
            &value.to_le_bytes(),
            "i128",
        )
    }

    /// Writes a little-endian unsigned 128-bit integer.
    ///
    /// This is useful for the canonical IR value system, which uses `u128`
    /// semantic integers.
    pub fn write_u128(
        &mut self,
        value: u128,
    ) -> Result<(), SerializationError> {
        self.write_raw(
            &value.to_le_bytes(),
            "u128",
        )
    }

    // =========================================================================
    // Floating point
    // =========================================================================

    /// Writes an IEEE-754 `f32` bit pattern.
    ///
    /// Non-finite values are rejected because the canonical Zamani semantic
    /// value layer uses finite floating-point values.
    pub fn write_f32(
        &mut self,
        value: f32,
    ) -> Result<(), SerializationError> {
        if !value.is_finite() {
            return Err(SerializationError::InvalidObject {
                message: "non-finite f32 values are not canonical IR values",
            });
        }

        self.write_u32(value.to_bits())
    }

    /// Writes an IEEE-754 `f64` bit pattern.
    ///
    /// Non-finite values are rejected.
    ///
    /// Rejecting NaN is important because otherwise multiple NaN bit patterns
    /// could represent semantically equivalent "not-a-number" values while
    /// producing different canonical byte sequences.
    pub fn write_f64(
        &mut self,
        value: f64,
    ) -> Result<(), SerializationError> {
        if !value.is_finite() {
            return Err(SerializationError::InvalidObject {
                message: "non-finite f64 values are not canonical IR values",
            });
        }

        self.write_u64(value.to_bits())
    }

    // =========================================================================
    // Platform-size values
    // =========================================================================

    /// Writes a platform `usize` as a stable wire-format `u64`.
    ///
    /// `usize` is never emitted directly because its width is platform
    /// dependent.
    pub fn write_usize(
        &mut self,
        value: usize,
    ) -> Result<(), SerializationError> {
        let value = u64::try_from(value).map_err(|_| {
            SerializationError::LengthOverflow {
                context: "usize",
                value: u64::MAX,
            }
        })?;

        self.write_u64(value)
    }

    /// Writes a platform `isize` as a stable wire-format `i64`.
    pub fn write_isize(
        &mut self,
        value: isize,
    ) -> Result<(), SerializationError> {
        let value = i64::try_from(value).map_err(|_| {
            SerializationError::LengthOverflow {
                context: "isize",
                value: i64::MAX as u64,
            }
        })?;

        self.write_i64(value)
    }

    // =========================================================================
    // Lengths
    // =========================================================================

    /// Writes a canonical collection length as `u64`.
    pub fn write_length(
        &mut self,
        length: usize,
        context: &'static str,
    ) -> Result<(), SerializationError> {
        let length = u64::try_from(length).map_err(|_| {
            SerializationError::LengthOverflow {
                context,
                value: u64::MAX,
            }
        })?;

        self.check_collection_length(
            length,
            context,
        )?;

        self.write_u64(length)
    }

    /// Writes a caller-supplied `u64` collection length after policy checking.
    pub fn write_collection_length(
        &mut self,
        length: u64,
        context: &'static str,
    ) -> Result<(), SerializationError> {
        self.check_collection_length(
            length,
            context,
        )?;

        self.write_u64(length)
    }

    /// Checks a collection length against the active encoding policy.
    pub fn check_collection_length(
        &self,
        length: u64,
        context: &'static str,
    ) -> Result<(), SerializationError> {
        if let Some(maximum) =
            self.limits.max_collection_elements
        {
            if length > maximum {
                return Err(
                    SerializationError::CollectionLimitExceeded {
                        context,
                        requested: length,
                        maximum,
                    },
                );
            }
        }

        Ok(())
    }

    /// Checks a byte/string field length against the active policy.
    pub fn check_field_length(
        &self,
        length: usize,
        context: &'static str,
    ) -> Result<u64, SerializationError> {
        let length = u64::try_from(length).map_err(|_| {
            SerializationError::LengthOverflow {
                context,
                value: u64::MAX,
            }
        })?;

        if let Some(maximum) =
            self.limits.max_field_bytes
        {
            if length > maximum {
                return Err(
                    SerializationError::FieldLimitExceeded {
                        context,
                        requested: length,
                        maximum,
                    },
                );
            }
        }

        Ok(length)
    }

    // =========================================================================
    // Byte/string values
    // =========================================================================

    /// Writes a length-prefixed byte sequence.
    ///
    /// Wire representation:
    ///
    /// ```text
    /// u64 byte_length
    /// [byte_length bytes]
    /// ```
    pub fn write_bytes(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), SerializationError> {
        let length =
            self.check_field_length(
                bytes.len(),
                "byte field",
            )?;

        self.write_u64(length)?;
        self.write_raw(
            bytes,
            "byte field contents",
        )
    }

    /// Writes a canonical UTF-8 string.
    ///
    /// Rust `str` is already guaranteed to contain valid UTF-8, so no
    /// additional encoding transformation is performed.
    pub fn write_string(
        &mut self,
        value: &str,
    ) -> Result<(), SerializationError> {
        self.write_bytes(value.as_bytes())
    }

    // =========================================================================
    // Quantum identities
    // =========================================================================

    /// Writes the canonical logical qubit identity.
    ///
    /// The identity type is owned by:
    ///
    /// `quantum::ir::qubit::QubitId`
    ///
    /// It is encoded as its stable numeric index in `u64` wire form.
    pub fn write_qubit_id(
        &mut self,
        qubit: QubitId,
    ) -> Result<(), SerializationError> {
        self.write_usize(qubit.index())
    }

    /// Writes the canonical physical qubit identity.
    ///
    /// The identity type is owned by:
    ///
    /// `quantum::ir::qubit::PhysicalQubitId`
    ///
    /// It is encoded as its stable numeric index in `u64` wire form.
    pub fn write_physical_qubit_id(
        &mut self,
        qubit: PhysicalQubitId,
    ) -> Result<(), SerializationError> {
        self.write_usize(qubit.index())
    }

    // =========================================================================
    // IR version
    // =========================================================================

    /// Writes the semantic Quantum IR version.
    ///
    /// The serialization framing version is owned by the parent serialization
    /// layer and MUST NOT be emitted here.
    pub fn write_ir_version(
        &mut self,
        version: IrVersion,
    ) -> Result<(), SerializationError> {
        self.write_u16(version.major())?;
        self.write_u16(version.minor())?;
        self.write_u16(version.patch())
    }

    // =========================================================================
    // Discriminants
    // =========================================================================

    /// Writes a canonical `u64` discriminant.
    ///
    /// Enum semantics remain owned by the calling IR type.
    pub fn write_discriminant(
        &mut self,
        value: u64,
    ) -> Result<(), SerializationError> {
        self.write_u64(value)
    }

    /// Writes a canonical `u8` discriminant.
    ///
    /// This is useful for compact closed enums whose wire contract explicitly
    /// assigns one-byte discriminants.
    pub fn write_discriminant_u8(
        &mut self,
        value: u8,
    ) -> Result<(), SerializationError> {
        self.write_u8(value)
    }

    // =========================================================================
    // Sequences
    // =========================================================================

    /// Writes a deterministic sequence.
    ///
    /// The sequence's existing semantic ordering is preserved.
    ///
    /// The encoder does not sort values.
    pub fn write_sequence<T, F>(
        &mut self,
        values: &[T],
        mut encode: F,
    ) -> Result<(), SerializationError>
    where
        F: FnMut(
            &mut Self,
            &T,
        ) -> Result<(), SerializationError>,
    {
        self.write_length(
            values.len(),
            "sequence",
        )?;

        for value in values {
            encode(self, value)?;
        }

        Ok(())
    }

    /// Writes a sequence using a caller-supplied semantic element count.
    ///
    /// This variant is useful when the caller has an iterator rather than a
    /// materialized slice.
    ///
    /// The iterator MUST yield exactly `length` elements. If it yields fewer
    /// or more elements, the encoder returns an explicit error.
    pub fn write_iter<I, T, F>(
        &mut self,
        length: usize,
        values: I,
        mut encode: F,
    ) -> Result<(), SerializationError>
    where
        I: IntoIterator<Item = T>,
        F: FnMut(
            &mut Self,
            &T,
        ) -> Result<(), SerializationError>,
    {
        self.write_length(
            length,
            "iterator sequence",
        )?;

        let mut count = 0usize;

        for value in values {
            if count >= length {
                return Err(
                    SerializationError::InvalidObject {
                        message:
                            "iterator produced more elements than its declared length",
                    },
                );
            }

            encode(self, &value)?;

            count = count.checked_add(1).ok_or(
                SerializationError::ArithmeticOverflow {
                    context: "iterator sequence count",
                },
            )?;
        }

        if count != length {
            return Err(
                SerializationError::InvalidObject {
                    message:
                        "iterator produced fewer elements than its declared length",
                },
            );
        }

        Ok(())
    }

    // =========================================================================
    // Entry/map-like sequences
    // =========================================================================

    /// Writes a deterministic key/value entry sequence.
    ///
    /// The caller MUST provide entries in canonical semantic order.
    ///
    /// This method intentionally does not sort.
    pub fn write_entries<K, V, F>(
        &mut self,
        entries: &[(K, V)],
        mut encode: F,
    ) -> Result<(), SerializationError>
    where
        F: FnMut(
            &mut Self,
            &K,
            &V,
        ) -> Result<(), SerializationError>,
    {
        self.write_length(
            entries.len(),
            "entry collection",
        )?;

        for (key, value) in entries {
            encode(
                self,
                key,
                value,
            )?;
        }

        Ok(())
    }

    // =========================================================================
    // Optional values
    // =========================================================================

    /// Writes an optional value using canonical discriminants:
    ///
    /// ```text
    /// 0 = None
    /// 1 = Some
    /// ```
    pub fn write_option<T, F>(
        &mut self,
        value: Option<&T>,
        mut encode: F,
    ) -> Result<(), SerializationError>
    where
        F: FnMut(
            &mut Self,
            &T,
        ) -> Result<(), SerializationError>,
    {
        match value {
            None => self.write_u8(0),

            Some(value) => {
                self.write_u8(1)?;
                encode(self, value)
            }
        }
    }

    // =========================================================================
    // Fixed-width raw data
    // =========================================================================

    /// Writes a fixed-width byte array.
    ///
    /// No length prefix is emitted.
    ///
    /// This is intended for fixed protocol fields whose width is already
    /// determined by the semantic type.
    pub fn write_fixed<const N: usize>(
        &mut self,
        value: &[u8; N],
    ) -> Result<(), SerializationError> {
        self.write_raw(
            value,
            "fixed-width field",
        )
    }

    // =========================================================================
    // Alignment/padding
    // =========================================================================

    /// Writes zero padding to reach the requested alignment.
    ///
    /// Alignment is a serialization-layout concern only. It must not be used
    /// to encode semantic information.
    ///
    /// `alignment == 0` is rejected.
    pub fn write_alignment_padding(
        &mut self,
        alignment: usize,
    ) -> Result<(), SerializationError> {
        if alignment == 0 {
            return Err(
                SerializationError::InvalidObject {
                    message:
                        "serialization alignment must be greater than zero",
                },
            );
        }

        let remainder =
            self.bytes.len() % alignment;

        if remainder == 0 {
            return Ok(());
        }

        let padding =
            alignment
                .checked_sub(remainder)
                .ok_or(
                    SerializationError::ArithmeticOverflow {
                        context:
                            "serialization alignment padding",
                    },
                )?;

        let zeros =
            [0u8; 64];

        let mut remaining = padding;

        while remaining > 0 {
            let chunk = remaining.min(zeros.len());

            self.write_raw(
                &zeros[..chunk],
                "alignment padding",
            )?;

            remaining -= chunk;
        }

        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_encoder_is_empty() {
        let encoder = Encoder::new();

        assert!(encoder.is_empty());
        assert_eq!(encoder.len(), 0);
    }

    #[test]
    fn primitive_integer_encoding_is_little_endian() {
        let mut encoder = Encoder::new();

        encoder
            .write_u16(0x1234)
            .expect("u16 encoding");

        encoder
            .write_u32(0x1234_5678)
            .expect("u32 encoding");

        encoder
            .write_u64(0x0123_4567_89ab_cdef)
            .expect("u64 encoding");

        assert_eq!(
            encoder.as_bytes(),
            &[
                0x34,
                0x12,
                0x78,
                0x56,
                0x34,
                0x12,
                0xef,
                0xcd,
                0xab,
                0x89,
                0x67,
                0x45,
                0x23,
                0x01,
            ]
        );
    }

    #[test]
    fn booleans_are_canonical() {
        let mut encoder = Encoder::new();

        encoder
            .write_bool(false)
            .expect("false encoding");

        encoder
            .write_bool(true)
            .expect("true encoding");

        assert_eq!(
            encoder.as_bytes(),
            &[0, 1]
        );
    }

    #[test]
    fn strings_are_length_prefixed() {
        let mut encoder = Encoder::new();

        encoder
            .write_string("Zamani")
            .expect("string encoding");

        let bytes = encoder.as_bytes();

        assert_eq!(
            &bytes[..8],
            &(6u64).to_le_bytes()
        );

        assert_eq!(
            &bytes[8..],
            b"Zamani"
        );
    }

    #[test]
    fn qubit_ids_use_canonical_qubit_module() {
        let mut encoder = Encoder::new();

        let logical =
            QubitId::new(17);

        let physical =
            PhysicalQubitId::new(29);

        encoder
            .write_qubit_id(logical)
            .expect("logical qubit");

        encoder
            .write_physical_qubit_id(physical)
            .expect("physical qubit");

        assert_eq!(
            encoder.as_bytes(),
            &[
                17, 0, 0, 0, 0, 0, 0, 0,
                29, 0, 0, 0, 0, 0, 0, 0,
            ]
        );
    }

    #[test]
    fn ir_version_is_encoded_without_framing_version() {
        let mut encoder = Encoder::new();

        let version =
            IrVersion::new(
                1,
                2,
                3,
            );

        encoder
            .write_ir_version(version)
            .expect("IR version");

        assert_eq!(
            encoder.as_bytes(),
            &[
                1, 0,
                2, 0,
                3, 0,
            ]
        );
    }

    #[test]
    fn sequence_preserves_semantic_order() {
        let mut encoder = Encoder::new();

        encoder
            .write_sequence(
                &[3u64, 1, 2],
                |encoder, value| {
                    encoder.write_u64(*value)
                },
            )
            .expect("sequence encoding");

        let bytes = encoder.as_bytes();

        assert_eq!(
            &bytes[..8],
            &(3u64).to_le_bytes()
        );

        assert_eq!(
            &bytes[8..16],
            &(3u64).to_le_bytes()
        );

        assert_eq!(
            &bytes[16..24],
            &(1u64).to_le_bytes()
        );

        assert_eq!(
            &bytes[24..32],
            &(2u64).to_le_bytes()
        );
    }

    #[test]
    fn iterator_requires_exact_declared_length() {
        let mut encoder = Encoder::new();

        let result =
            encoder.write_iter(
                2,
                vec![1u64, 2, 3],
                |encoder, value| {
                    encoder.write_u64(*value)
                },
            );

        assert!(matches!(
            result,
            Err(SerializationError::InvalidObject { .. })
        ));
    }

    #[test]
    fn iterator_rejects_too_few_elements() {
        let mut encoder = Encoder::new();

        let result =
            encoder.write_iter(
                3,
                vec![1u64, 2],
                |encoder, value| {
                    encoder.write_u64(*value)
                },
            );

        assert!(matches!(
            result,
            Err(SerializationError::InvalidObject { .. })
        ));
    }

    #[test]
    fn non_finite_f64_is_rejected() {
        let mut encoder = Encoder::new();

        assert!(matches!(
            encoder.write_f64(f64::NAN),
            Err(
                SerializationError::InvalidObject { .. }
            )
        ));

        assert!(matches!(
            encoder.write_f64(f64::INFINITY),
            Err(
                SerializationError::InvalidObject { .. }
            )
        ));

        assert!(matches!(
            encoder.write_f64(
                f64::NEG_INFINITY
            ),
            Err(
                SerializationError::InvalidObject { .. }
            )
        ));
    }

    #[test]
    fn finite_f64_is_encoded_deterministically() {
        let mut first = Encoder::new();
        let mut second = Encoder::new();

        first
            .write_f64(0.25)
            .expect("first f64");

        second
            .write_f64(0.25)
            .expect("second f64");

        assert_eq!(
            first.as_bytes(),
            second.as_bytes()
        );
    }

    #[test]
    fn payload_limit_is_enforced() {
        let limits =
            EncodeLimits::new(
                Some(8),
                None,
                None,
            );

        let mut encoder =
            Encoder::with_limits(limits)
                .expect("valid limits");

        assert!(
            encoder
                .write_u64(1)
                .is_ok()
        );

        assert!(matches!(
            encoder.write_u8(1),
            Err(
                SerializationError::PayloadTooLarge {
                    ..
                }
            )
        ));
    }

    #[test]
    fn field_limit_is_enforced_before_payload_write() {
        let limits =
            EncodeLimits::new(
                None,
                Some(4),
                None,
            );

        let mut encoder =
            Encoder::with_limits(limits)
                .expect("valid limits");

        assert!(matches!(
            encoder.write_string("Zamani"),
            Err(
                SerializationError::FieldLimitExceeded {
                    ..
                }
            )
        ));

        assert!(
            encoder.is_empty(),
            "failed field must not partially encode"
        );
    }

    #[test]
    fn collection_limit_is_enforced_before_elements() {
        let limits =
            EncodeLimits::new(
                None,
                None,
                Some(2),
            );

        let mut encoder =
            Encoder::with_limits(limits)
                .expect("valid limits");

        assert!(matches!(
            encoder.write_sequence(
                &[1u64, 2, 3],
                |encoder, value| {
                    encoder.write_u64(*value)
                },
            ),
            Err(
                SerializationError::CollectionLimitExceeded {
                    ..
                }
            )
        ));

        assert!(
            encoder.is_empty(),
            "failed collection must not write its length"
        );
    }

    #[test]
    fn alignment_padding_is_zero_filled() {
        let mut encoder = Encoder::new();

        encoder
            .write_u8(0xaa)
            .expect("byte");

        encoder
            .write_alignment_padding(8)
            .expect("padding");

        assert_eq!(
            encoder.as_bytes(),
            &[
                0xaa,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ]
        );
    }

    #[test]
    fn alignment_requires_nonzero_boundary() {
        let mut encoder = Encoder::new();

        assert!(matches!(
            encoder.write_alignment_padding(0),
            Err(
                SerializationError::InvalidObject {
                    ..
                }
            )
        ));
    }

    #[test]
    fn unlimited_policy_has_no_application_payload_limit() {
        let limits =
            EncodeLimits::unlimited();

        assert_eq!(
            limits.max_payload_bytes,
            None
        );

        assert_eq!(
            limits.max_field_bytes,
            None
        );

        assert_eq!(
            limits.max_collection_elements,
            None
        );
    }

    #[test]
    fn capacity_hint_does_not_change_wire_bytes() {
        let mut first =
            Encoder::new();

        let mut second =
            Encoder::with_capacity(1024)
                .expect("capacity");

        first
            .write_string("Zamani")
            .expect("first");

        second
            .write_string("Zamani")
            .expect("second");

        assert_eq!(
            first.as_bytes(),
            second.as_bytes()
        );
    }
}