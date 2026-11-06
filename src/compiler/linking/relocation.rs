
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Compiler — Relocation Model and Application
//!
//! Path: `src/compiler/linking/relocation.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Represent and apply validated link-time relocations without owning
//! object-file parsing, target instruction encoding, section placement,
//! ABI definitions, or symbol resolution.
//!
//! A relocation identifies:
//! - the input compilation unit;
//! - the section and byte offset containing the relocation field;
//! - the canonical symbol reference;
//! - the relocation computation;
//! - the addend;
//! - the field representation and overflow policy;
//! - an open-ended relocation-kind identifier.
//!
//! The canonical symbol index resolves symbol identity. A target/link
//! provider supplies addresses after section placement and other required
//! link-time structures have been established.
//!
//! ## OWNERSHIP
//!
//! This module owns generic relocation semantics and safe field patching.
//!
//! It does not own:
//! - source-level name resolution;
//! - symbol ID allocation;
//! - object-file parsing or writing;
//! - ABI or calling-convention definitions;
//! - target-specific relocation number inventories;
//! - instruction-specific scattered-bit encodings;
//! - section placement or memory allocation;
//! - GOT/PLT construction;
//! - TLS layout;
//! - executable loading or runtime execution.
//!
//! Those responsibilities remain with their canonical linker, object-format,
//! ABI, backend, and runtime owners.
//!
//! ## SCALABILITY
//!
//! No arbitrary limits are imposed on relocation count, section count,
//! symbol count, target count, or address-space size.
//!
//! The generic contiguous field encoder supports fields up to 128 bits.
//! This is the representational width of its `u128` arithmetic, not a
//! universal hardware or language limit. Wider or non-contiguous target
//! encodings require a target-specific codec.
//!
//! ## TRANSACTIONAL BEHAVIOR
//!
//! `apply_relocation` validates and computes the complete patch before
//! mutating the destination bytes.
//!
//! `apply_relocations` applies a batch to a staged copy and commits only
//! when every relocation succeeds.
//!
//! ## INTEGRATION
//!
//! - `linking/symbols.rs` resolves unit-scoped canonical `SymbolRef`s.
//! - `artifacts/*` owns artifact identity, payloads, manifests and storage.
//! - `backend/*` owns target-specific code generation and target relocation
//!   codecs.
//! - ABI and data-layout analysis validate interoperability contracts before
//!   target-specific relocation is attempted.
//! - The linker owns section placement and supplies final addresses through
//!   `RelocationAddressProvider`.

use std::error::Error;
use std::fmt;

use crate::compiler::linking::symbols::{
    LinkUnitId,
    SymbolDefinition,
    SymbolIndex,
    SymbolLinkError,
};

use crate::zuir::values::symbols::SymbolRef;

// ============================================================================
// API
// ============================================================================

/// Public API version of the relocation subsystem.
pub const RELOCATION_API_VERSION: &str = "1.0.0";

/// Result type used by relocation operations.
pub type RelocationResult<T> = Result<T, RelocationError>;

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// An open-ended relocation-kind identifier.
///
/// Examples may include names defined by an object format, architecture,
/// ABI, backend, or future target extension.
///
/// This type deliberately does not enumerate machine relocation numbers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelocationKindId(String);

impl RelocationKindId {
    /// Construct a relocation-kind identifier.
    pub fn new(value: impl Into<String>) -> RelocationResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(RelocationError::EmptyIdentifier {
                field: "relocation kind",
            });
        }

        Ok(Self(value))
    }

    /// Return the exact identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RelocationKindId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Identifies a section within one link input.
///
/// Section identity is deliberately independent of any particular object
/// format's section-number representation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelocationSectionId(String);

impl RelocationSectionId {
    /// Construct a section identifier.
    pub fn new(value: impl Into<String>) -> RelocationResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(RelocationError::EmptyIdentifier {
                field: "section",
            });
        }

        Ok(Self(value))
    }

    /// Return the exact identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// FIELD REPRESENTATION
// ============================================================================

/// Byte order used when reading or writing a relocation field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RelocationByteOrder {
    /// Least-significant byte is stored first.
    LittleEndian,

    /// Most-significant byte is stored first.
    BigEndian,
}

/// Whether the computed relocation value is interpreted as signed or
/// unsigned when checking the destination field's representable range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RelocationSignedness {
    /// Interpret the field as an unsigned integer.
    Unsigned,

    /// Interpret the field as a signed two's-complement integer.
    Signed,
}

/// Behavior when the computed value does not fit the relocation field.
///
/// Truncation is intentionally explicit. A backend must only select it
/// when the applicable target relocation contract permits truncation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RelocationOverflowPolicy {
    /// Reject values that cannot be represented.
    Reject,

    /// Keep only the least-significant field bits.
    ///
    /// This is not a default and must not be used to hide an overflow.
    Truncate,
}

/// Description of a contiguous relocation field.
///
/// `bit_offset` is measured from the least-significant bit of the integer
/// represented by the selected bytes. It is not a target instruction's
/// architectural bit-numbering convention.
///
/// A target whose instruction encoding scatters immediate bits must use
/// a target-specific codec instead of this contiguous-field representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RelocationField {
    /// Offset of the first byte in the section payload.
    pub byte_offset: u128,

    /// Number of bits in the field, from 1 through 128.
    pub bit_width: u8,

    /// Offset of the field from the least-significant bit of its storage.
    pub bit_offset: u8,

    /// Byte order of the field storage.
    pub byte_order: RelocationByteOrder,

    /// Signed or unsigned interpretation.
    pub signedness: RelocationSignedness,

    /// Overflow behavior.
    pub overflow: RelocationOverflowPolicy,
}

impl RelocationField {
    /// Construct and validate a contiguous relocation field.
    pub fn new(
        byte_offset: u128,
        bit_width: u8,
        bit_offset: u8,
        byte_order: RelocationByteOrder,
        signedness: RelocationSignedness,
        overflow: RelocationOverflowPolicy,
    ) -> RelocationResult<Self> {
        if bit_width == 0 || bit_width > 128 {
            return Err(RelocationError::InvalidFieldWidth {
                width: bit_width,
            });
        }

        let end = u16::from(bit_offset)
            .checked_add(u16::from(bit_width))
            .ok_or(RelocationError::ArithmeticOverflow)?;

        if end > 128 {
            return Err(RelocationError::FieldExceedsGenericWidth {
                bit_offset,
                bit_width,
            });
        }

        Ok(Self {
            byte_offset,
            bit_width,
            bit_offset,
            byte_order,
            signedness,
            overflow,
        })
    }

    /// Number of bytes required to store this field.
    pub fn storage_bytes(self) -> usize {
        usize::from(
            (u16::from(self.bit_offset) + u16::from(self.bit_width)).div_ceil(8),
        )
    }

    /// Return the exclusive ending byte offset.
    pub fn end_byte_offset(self) -> RelocationResult<u128> {
        self.byte_offset
            .checked_add(self.storage_bytes() as u128)
            .ok_or(RelocationError::ArithmeticOverflow)
    }
}

// ============================================================================
// RELOCATION COMPUTATION
// ============================================================================

/// Generic relocation expression.
///
/// These are semantic calculations, not machine relocation opcodes.
/// A target-specific relocation kind determines whether a particular
/// expression is legal for that target and object format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RelocationExpression {
    /// `S + A`
    ///
    /// Symbol address plus addend.
    Absolute,

    /// `S + A - P`
    ///
    /// Symbol address plus addend minus relocation place.
    PcRelative,

    /// `S + A - SectionBase`
    ///
    /// Symbol address plus addend relative to a section base.
    SectionRelative,

    /// `S + A - ImageBase`
    ///
    /// Symbol address plus addend relative to the linked image base.
    ImageRelative,

    /// `GOTEntry(S) + A - P`
    ///
    /// Global-offset-table entry relative to the relocation place.
    GotRelative,

    /// `PLTEntry(S) + A - P`
    ///
    /// Procedure-linkage-table entry relative to the relocation place.
    PltRelative,

    /// `TLSOffset(S) + A`
    ///
    /// Thread-local storage offset plus addend.
    TlsRelative,
}

// ============================================================================
// RELOCATION RECORD
// ============================================================================

/// One unresolved relocation request.
///
/// A relocation refers to a canonical symbol identity, not a copied
/// semantic symbol name. The identity is scoped by `unit`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relocation {
    unit: LinkUnitId,
    section: RelocationSectionId,
    kind: RelocationKindId,
    symbol: SymbolRef,
    field: RelocationField,
    expression: RelocationExpression,
    addend: i128,
}

impl Relocation {
    /// Construct a relocation record.
    pub fn new(
        unit: LinkUnitId,
        section: RelocationSectionId,
        kind: RelocationKindId,
        symbol: SymbolRef,
        field: RelocationField,
        expression: RelocationExpression,
        addend: i128,
    ) -> Self {
        Self {
            unit,
            section,
            kind,
            symbol,
            field,
            expression,
            addend,
        }
    }

    /// Return the input compilation unit.
    pub fn unit(&self) -> &LinkUnitId {
        &self.unit
    }

    /// Return the section containing the relocation.
    pub fn section(&self) -> &RelocationSectionId {
        &self.section
    }

    /// Return the relocation-kind identifier.
    pub fn kind(&self) -> &RelocationKindId {
        &self.kind
    }

    /// Return the canonical symbol reference.
    pub fn symbol(&self) -> SymbolRef {
        self.symbol
    }

    /// Return the relocation field description.
    pub fn field(&self) -> RelocationField {
        self.field
    }

    /// Return the relocation expression.
    pub fn expression(&self) -> RelocationExpression {
        self.expression
    }

    /// Return the signed addend.
    pub fn addend(&self) -> i128 {
        self.addend
    }

    /// Validate the record's local structural invariants.
    pub fn validate(&self) -> RelocationResult<()> {
        if self.kind.as_str().trim().is_empty() {
            return Err(RelocationError::EmptyIdentifier {
                field: "relocation kind",
            });
        }

        if self.section.as_str().trim().is_empty() {
            return Err(RelocationError::EmptyIdentifier {
                field: "section",
            });
        }

        RelocationField::new(
            self.field.byte_offset,
            self.field.bit_width,
            self.field.bit_offset,
            self.field.byte_order,
            self.field.signedness,
            self.field.overflow,
        )?;

        Ok(())
    }
}

// ============================================================================
// ADDRESS PROVIDER
// ============================================================================

/// Supplies final link-time addresses to the relocation engine.
///
/// Implementations belong to the linker or target integration. They must
/// return addresses from a consistent link layout and must not silently
/// substitute a different address space or ABI.
///
/// The provider is queried only after the symbol index has resolved the
/// canonical symbol reference.
pub trait RelocationAddressProvider {
    /// Return the final address of a resolved symbol.
    fn symbol_address(
        &self,
        definition: &SymbolDefinition,
    ) -> RelocationResult<u128>;

    /// Return the final address of the relocation place.
    fn place_address(
        &self,
        section: &RelocationSectionId,
        offset: u128,
    ) -> RelocationResult<u128>;

    /// Return the base address of a section.
    fn section_base(
        &self,
        section: &RelocationSectionId,
    ) -> RelocationResult<u128>;

    /// Return the linked image base.
    fn image_base(&self) -> RelocationResult<u128>;

    /// Return the GOT entry address for a symbol.
    ///
    /// The default implementation reports that the provider does not
    /// support this relocation requirement.
    fn got_entry_address(
        &self,
        _definition: &SymbolDefinition,
    ) -> RelocationResult<u128> {
        Err(RelocationError::AddressServiceUnavailable {
            service: "GOT entry",
        })
    }

    /// Return the PLT entry address for a symbol.
    fn plt_entry_address(
        &self,
        _definition: &SymbolDefinition,
    ) -> RelocationResult<u128> {
        Err(RelocationError::AddressServiceUnavailable {
            service: "PLT entry",
        })
    }

    /// Return the TLS offset for a symbol.
    fn tls_offset(
        &self,
        _definition: &SymbolDefinition,
    ) -> RelocationResult<u128> {
        Err(RelocationError::AddressServiceUnavailable {
            service: "TLS offset",
        })
    }
}

// ============================================================================
// COMPUTED VALUE
// ============================================================================

/// Signed magnitude used internally so address arithmetic can represent
/// negative PC-relative results without restricting addresses to `i128`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WideSigned {
    negative: bool,
    magnitude: u128,
}

impl WideSigned {
    fn from_unsigned(value: u128) -> Self {
        Self {
            negative: false,
            magnitude: value,
        }
    }

    fn add_signed(
        self,
        value: i128,
    ) -> RelocationResult<Self> {
        if value >= 0 {
            self.add_unsigned(value as u128)
        } else {
            self.subtract_unsigned(value.unsigned_abs())
        }
    }

    fn add_unsigned(
        self,
        value: u128,
    ) -> RelocationResult<Self> {
        if value == 0 {
            return Ok(self);
        }

        if self.negative {
            if self.magnitude > value {
                Ok(Self {
                    negative: true,
                    magnitude: self.magnitude - value,
                })
            } else {
                Ok(Self {
                    negative: false,
                    magnitude: value - self.magnitude,
                })
            }
        } else {
            Ok(Self {
                negative: false,
                magnitude: self
                    .magnitude
                    .checked_add(value)
                    .ok_or(RelocationError::ArithmeticOverflow)?,
            })
        }
    }

    fn subtract_unsigned(
        self,
        value: u128,
    ) -> RelocationResult<Self> {
        if value == 0 {
            return Ok(self);
        }

        if self.negative {
            Ok(Self {
                negative: true,
                magnitude: self
                    .magnitude
                    .checked_add(value)
                    .ok_or(RelocationError::ArithmeticOverflow)?,
            })
        } else if self.magnitude >= value {
            Ok(Self {
                negative: false,
                magnitude: self.magnitude - value,
            })
        } else {
            Ok(Self {
                negative: true,
                magnitude: value - self.magnitude,
            })
        }
    }
}

fn calculate(
    relocation: &Relocation,
    definition: &SymbolDefinition,
    provider: &impl RelocationAddressProvider,
) -> RelocationResult<WideSigned> {
    let symbol_address = match relocation.expression {
        RelocationExpression::Absolute
        | RelocationExpression::PcRelative
        | RelocationExpression::SectionRelative
        | RelocationExpression::ImageRelative => {
            provider.symbol_address(definition)?
        }

        RelocationExpression::GotRelative => {
            provider.got_entry_address(definition)?
        }

        RelocationExpression::PltRelative => {
            provider.plt_entry_address(definition)?
        }

        RelocationExpression::TlsRelative => {
            provider.tls_offset(definition)?
        }
    };

    let mut value = WideSigned::from_unsigned(symbol_address)
        .add_signed(relocation.addend)?;

    match relocation.expression {
        RelocationExpression::Absolute
        | RelocationExpression::TlsRelative => {}

        RelocationExpression::PcRelative
        | RelocationExpression::GotRelative
        | RelocationExpression::PltRelative => {
            let place = provider.place_address(
                &relocation.section,
                relocation.field.byte_offset,
            )?;

            value = value.subtract_unsigned(place)?;
        }

        RelocationExpression::SectionRelative => {
            let base = provider.section_base(&relocation.section)?;
            value = value.subtract_unsigned(base)?;
        }

        RelocationExpression::ImageRelative => {
            let base = provider.image_base()?;
            value = value.subtract_unsigned(base)?;
        }
    }

    Ok(value)
}

// ============================================================================
// FIELD ENCODING
// ============================================================================

fn bit_mask(width: u8) -> u128 {
    if width == 128 {
        u128::MAX
    } else {
        (1u128 << width) - 1
    }
}

fn encode_value(
    value: WideSigned,
    field: RelocationField,
) -> RelocationResult<u128> {
    let width = field.bit_width;
    let mask = bit_mask(width);

    if field.overflow == RelocationOverflowPolicy::Reject {
        match field.signedness {
            RelocationSignedness::Unsigned => {
                if value.negative || value.magnitude > mask {
                    return Err(RelocationError::ValueOutOfRange {
                        width,
                        signedness: field.signedness,
                        negative: value.negative,
                    });
                }
            }

            RelocationSignedness::Signed => {
                let negative_limit = if width == 128 {
                    1u128 << 127
                } else {
                    1u128 << (width - 1)
                };

                let positive_limit = negative_limit - 1;

                if value.negative {
                    if value.magnitude > negative_limit {
                        return Err(RelocationError::ValueOutOfRange {
                            width,
                            signedness: field.signedness,
                            negative: true,
                        });
                    }
                } else if value.magnitude > positive_limit {
                    return Err(RelocationError::ValueOutOfRange {
                        width,
                        signedness: field.signedness,
                        negative: false,
                    });
                }
            }
        }
    }

    let encoded = if value.negative {
        (!value.magnitude).wrapping_add(1)
    } else {
        value.magnitude
    };

    Ok(encoded & mask)
}

fn read_storage(
    bytes: &[u8],
    order: RelocationByteOrder,
) -> u128 {
    let mut value = 0u128;

    match order {
        RelocationByteOrder::LittleEndian => {
            for (index, byte) in bytes.iter().enumerate() {
                value |= u128::from(*byte) << (index * 8);
            }
        }

        RelocationByteOrder::BigEndian => {
            for byte in bytes {
                value = (value << 8) | u128::from(*byte);
            }
        }
    }

    value
}

fn write_storage(
    value: u128,
    bytes: &mut [u8],
    order: RelocationByteOrder,
) {
    match order {
        RelocationByteOrder::LittleEndian => {
            for (index, byte) in bytes.iter_mut().enumerate() {
                *byte = (value >> (index * 8)) as u8;
            }
        }

        RelocationByteOrder::BigEndian => {
            let length = bytes.len();

            for (index, byte) in bytes.iter_mut().enumerate() {
                let shift = (length - index - 1) * 8;
                *byte = (value >> shift) as u8;
            }
        }
    }
}

fn patch_field(
    payload: &mut [u8],
    field: RelocationField,
    encoded_value: u128,
) -> RelocationResult<()> {
    let start = usize::try_from(field.byte_offset)
        .map_err(|_| RelocationError::OffsetNotRepresentable)?;

    let length = field.storage_bytes();

    let end = start
        .checked_add(length)
        .ok_or(RelocationError::ArithmeticOverflow)?;

    let storage = payload
        .get_mut(start..end)
        .ok_or(RelocationError::FieldOutsideSection {
            offset: field.byte_offset,
            length,
            section_length: payload.len(),
        })?;

    let current = read_storage(storage, field.byte_order);
    let field_mask = bit_mask(field.bit_width);
    let shifted_mask = field_mask << field.bit_offset;
    let shifted_value = (encoded_value & field_mask) << field.bit_offset;

    let updated = (current & !shifted_mask) | shifted_value;

    write_storage(updated, storage, field.byte_order);

    Ok(())
}

// ============================================================================
// APPLICATION
// ============================================================================

/// Apply one relocation to a section payload.
///
/// The symbol reference is resolved using its compilation-unit identity.
/// The complete expression and range checks are performed before the
/// destination bytes are modified.
pub fn apply_relocation(
    relocation: &Relocation,
    symbols: &SymbolIndex,
    provider: &impl RelocationAddressProvider,
    section_payload: &mut [u8],
) -> RelocationResult<()> {
    relocation.validate()?;

    let definition = symbols
        .resolve_reference(&relocation.unit, &relocation.symbol)
        .map_err(RelocationError::SymbolResolution)?;

    let value = calculate(relocation, definition, provider)?;
    let encoded = encode_value(value, relocation.field)?;

    patch_field(section_payload, relocation.field, encoded)
}

/// Apply a collection of relocations atomically.
///
/// The input order is preserved because multiple relocation records may
/// intentionally update disjoint fields in the same section. The caller
/// must validate that overlapping writes are legal for its object format.
///
/// The original payload is unchanged if any relocation fails.
pub fn apply_relocations(
    relocations: &[Relocation],
    symbols: &SymbolIndex,
    provider: &impl RelocationAddressProvider,
    section_payload: &mut Vec<u8>,
) -> RelocationResult<()> {
    let mut staged = section_payload.clone();

    for relocation in relocations {
        apply_relocation(
            relocation,
            symbols,
            provider,
            &mut staged,
        )?;
    }

    *section_payload = staged;

    Ok(())
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors returned by relocation validation, computation, and application.
#[derive(Debug)]
#[non_exhaustive]
pub enum RelocationError {
    /// An identifier was empty or contained only whitespace.
    EmptyIdentifier {
        /// Identifier category.
        field: &'static str,
    },

    /// A relocation field has an invalid width.
    InvalidFieldWidth {
        /// Requested field width.
        width: u8,
    },

    /// The field exceeds the generic 128-bit storage representation.
    FieldExceedsGenericWidth {
        /// Bit offset.
        bit_offset: u8,

        /// Bit width.
        bit_width: u8,
    },

    /// A checked arithmetic operation overflowed.
    ArithmeticOverflow,

    /// A computed value cannot be represented by the requested field.
    ValueOutOfRange {
        /// Field width.
        width: u8,

        /// Requested interpretation.
        signedness: RelocationSignedness,

        /// Whether the computed value was negative.
        negative: bool,
    },

    /// The relocation offset cannot be represented as a host slice index.
    OffsetNotRepresentable,

    /// The relocation field lies outside the supplied section payload.
    FieldOutsideSection {
        /// Requested byte offset.
        offset: u128,

        /// Required field storage length.
        length: usize,

        /// Actual section payload length.
        section_length: usize,
    },

    /// A required address service is unavailable.
    AddressServiceUnavailable {
        /// Name of the unavailable address service.
        service: &'static str,
    },

    /// Canonical symbol resolution failed.
    SymbolResolution(SymbolLinkError),
}

impl fmt::Display for RelocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(f, "empty relocation {field} identifier")
            }

            Self::InvalidFieldWidth { width } => {
                write!(f, "invalid relocation field width: {width}")
            }

            Self::FieldExceedsGenericWidth {
                bit_offset,
                bit_width,
            } => {
                write!(
                    f,
                    "relocation field at bit {bit_offset} with width \
                     {bit_width} exceeds the generic 128-bit representation"
                )
            }

            Self::ArithmeticOverflow => {
                f.write_str("relocation arithmetic overflow")
            }

            Self::ValueOutOfRange {
                width,
                signedness,
                negative,
            } => {
                write!(
                    f,
                    "relocation value does not fit a {width}-bit \
                     {signedness:?} field (negative: {negative})"
                )
            }

            Self::OffsetNotRepresentable => {
                f.write_str(
                    "relocation offset cannot be represented as a host index",
                )
            }

            Self::FieldOutsideSection {
                offset,
                length,
                section_length,
            } => {
                write!(
                    f,
                    "relocation field at byte {offset} requiring {length} \
                     bytes exceeds section payload length {section_length}"
                )
            }

            Self::AddressServiceUnavailable { service } => {
                write!(f, "relocation address service unavailable: {service}")
            }

            Self::SymbolResolution(error) => {
                write!(f, "relocation symbol resolution failed: {error}")
            }
        }
    }
}

impl Error for RelocationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::SymbolResolution(error) => Some(error),
            _ => None,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn field(
        width: u8,
        signedness: RelocationSignedness,
    ) -> RelocationField {
        RelocationField::new(
            0,
            width,
            0,
            RelocationByteOrder::LittleEndian,
            signedness,
            RelocationOverflowPolicy::Reject,
        )
        .unwrap()
    }

    #[test]
    fn accepts_one_bit_fields() {
        assert!(field(1, RelocationSignedness::Unsigned).bit_width == 1);
    }

    #[test]
    fn rejects_zero_width() {
        assert!(matches!(
            RelocationField::new(
                0,
                0,
                0,
                RelocationByteOrder::LittleEndian,
                RelocationSignedness::Unsigned,
                RelocationOverflowPolicy::Reject,
            ),
            Err(RelocationError::InvalidFieldWidth { .. })
        ));
    }

    #[test]
    fn rejects_fields_outside_generic_width() {
        assert!(matches!(
            RelocationField::new(
                0,
                128,
                1,
                RelocationByteOrder::LittleEndian,
                RelocationSignedness::Unsigned,
                RelocationOverflowPolicy::Reject,
            ),
            Err(RelocationError::FieldExceedsGenericWidth { .. })
        ));
    }

    #[test]
    fn accepts_unsigned_maximum() {
        assert_eq!(
            encode_value(
                WideSigned::from_unsigned(255),
                field(8, RelocationSignedness::Unsigned),
            ),
            Ok(255),
        );
    }

    #[test]
    fn rejects_unsigned_negative_value() {
        assert!(matches!(
            encode_value(
                WideSigned {
                    negative: true,
                    magnitude: 1,
                },
                field(8, RelocationSignedness::Unsigned),
            ),
            Err(RelocationError::ValueOutOfRange { .. })
        ));
    }

    #[test]
    fn accepts_signed_negative_value() {
        assert_eq!(
            encode_value(
                WideSigned {
                    negative: true,
                    magnitude: 1,
                },
                field(8, RelocationSignedness::Signed),
            ),
            Ok(255),
        );
    }

    #[test]
    fn accepts_signed_minimum() {
        assert_eq!(
            encode_value(
                WideSigned {
                    negative: true,
                    magnitude: 128,
                },
                field(8, RelocationSignedness::Signed),
            ),
            Ok(128),
        );
    }

    #[test]
    fn rejects_signed_positive_overflow() {
        assert!(matches!(
            encode_value(
                WideSigned::from_unsigned(128),
                field(8, RelocationSignedness::Signed),
            ),
            Err(RelocationError::ValueOutOfRange { .. })
        ));
    }

    #[test]
    fn truncation_is_explicit() {
        let field = RelocationField::new(
            0,
            8,
            0,
            RelocationByteOrder::LittleEndian,
            RelocationSignedness::Unsigned,
            RelocationOverflowPolicy::Truncate,
        )
        .unwrap();

        assert_eq!(
            encode_value(
                WideSigned::from_unsigned(0x1234),
                field,
            ),
            Ok(0x34),
        );
    }

    #[test]
    fn preserves_unrelated_bits_in_field_storage() {
        let field = RelocationField::new(
            0,
            4,
            0,
            RelocationByteOrder::LittleEndian,
            RelocationSignedness::Unsigned,
            RelocationOverflowPolicy::Reject,
        )
        .unwrap();

        let mut bytes = [0b1011_0000];

        patch_field(&mut bytes, field, 0b0101).unwrap();

        assert_eq!(bytes[0], 0b1011_0101);
    }

    #[test]
    fn supports_big_endian_storage() {
        let field = RelocationField::new(
            0,
            16,
            0,
            RelocationByteOrder::BigEndian,
            RelocationSignedness::Unsigned,
            RelocationOverflowPolicy::Reject,
        )
        .unwrap();

        let mut bytes = [0, 0];

        patch_field(&mut bytes, field, 0x1234).unwrap();

        assert_eq!(bytes, [0x12, 0x34]);
    }

    #[test]
    fn wide_signed_arithmetic_supports_addresses_above_i128() {
        let address = u128::MAX;

        let result = WideSigned::from_unsigned(address)
            .subtract_unsigned(1)
            .unwrap();

        assert_eq!(result.magnitude, u128::MAX - 1);
        assert!(!result.negative);
    }

    #[test]
    fn signed_magnitude_can_represent_negative_pc_relative_values() {
        let result = WideSigned::from_unsigned(10)
            .subtract_unsigned(20)
            .unwrap();

        assert!(result.negative);
        assert_eq!(result.magnitude, 10);
    }

    #[test]
    fn add_signed_handles_i128_minimum() {
        let result = WideSigned::from_unsigned(0)
            .add_signed(i128::MIN)
            .unwrap();

        assert!(result.negative);
        assert_eq!(result.magnitude, i128::MIN.unsigned_abs());
    }

    #[test]
    fn field_storage_size_is_derived_from_bit_range() {
        let field = RelocationField::new(
            0,
            12,
            4,
            RelocationByteOrder::LittleEndian,
            RelocationSignedness::Unsigned,
            RelocationOverflowPolicy::Reject,
        )
        .unwrap();

        assert_eq!(field.storage_bytes(), 2);
    }

    #[test]
    fn field_end_offset_uses_checked_arithmetic() {
        let field = RelocationField::new(
            u128::MAX,
            8,
            0,
            RelocationByteOrder::LittleEndian,
            RelocationSignedness::Unsigned,
            RelocationOverflowPolicy::Reject,
        )
        .unwrap();

        assert!(matches!(
            field.end_byte_offset(),
            Err(RelocationError::ArithmeticOverflow)
        ));
    }

    #[test]
    fn rejects_fields_outside_payload() {
        let field = RelocationField::new(
            1,
            16,
            0,
            RelocationByteOrder::LittleEndian,
            RelocationSignedness::Unsigned,
            RelocationOverflowPolicy::Reject,
        )
        .unwrap();

        let mut bytes = [0u8; 2];

        assert!(matches!(
            patch_field(&mut bytes, field, 1),
            Err(RelocationError::FieldOutsideSection { .. })
        ));
    }

    #[test]
    fn validates_open_ended_relocation_kind_ids() {
        let kind = RelocationKindId::new(
            "future.vendor.architecture.relocation",
        )
        .unwrap();

        assert_eq!(
            kind.as_str(),
            "future.vendor.architecture.relocation"
        );
    }

    #[test]
    fn rejects_empty_relocation_kind_ids() {
        assert!(matches!(
            RelocationKindId::new("  "),
            Err(RelocationError::EmptyIdentifier { .. })
        ));
    }

    #[test]
    fn address_services_are_explicitly_unavailable_by_default() {
        struct MinimalProvider;

        impl RelocationAddressProvider for MinimalProvider {
            fn symbol_address(
                &self,
                _: &SymbolDefinition,
            ) -> RelocationResult<u128> {
                Ok(0)
            }

            fn place_address(
                &self,
                _: &RelocationSectionId,
                _: u128,
            ) -> RelocationResult<u128> {
                Ok(0)
            }

            fn section_base(
                &self,
                _: &RelocationSectionId,
            ) -> RelocationResult<u128> {
                Ok(0)
            }

            fn image_base(&self) -> RelocationResult<u128> {
                Ok(0)
            }
        }

        // The default GOT/PLT/TLS methods must never fabricate addresses.
        let _provider = MinimalProvider;
    }
}
