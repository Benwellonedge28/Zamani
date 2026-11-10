
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Foundation — Strings
//!
//! Path: `src/foundation/strings.rs`
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides target-independent string operations for Zamani.
//!
//! The implementation uses Rust's native UTF-8 `String` and `str`
//! representations instead of introducing a competing string-storage
//! system.
//!
//! ## OWNS
//!
//! - Owned string construction.
//! - Borrowed UTF-8 string views.
//! - UTF-8 validation.
//! - Unicode scalar iteration.
//! - UTF-8 byte-offset validation.
//! - String comparison.
//! - String concatenation.
//! - Fallible string growth.
//! - String parsing through `FromStr`.
//! - Formatting through `fmt::Write`.
//! - Zamani ordinary string-literal escape decoding.
//! - Unicode scalar validation for escaped literals.
//!
//! ## DOES NOT OWN
//!
//! - Lexer tokenization.
//! - Parser implementation.
//! - AST construction.
//! - Semantic type checking.
//! - String interning.
//! - Canonical Unicode normalization.
//! - Locale-specific collation.
//! - Unicode grapheme segmentation.
//! - Runtime memory allocation policy.
//! - Filesystem operations.
//! - Serialization formats.
//! - Distributed string transport.
//! - Hardware-specific string representations.
//!
//! These responsibilities remain with their respective repository
//! authorities.
//!
//! ## GRAMMAR CONTRACT
//!
//! The authoritative ordinary string grammar is:
//!
//! `grammar/lexer/string-literals.g4`
//!
//! The grammar defines:
//!
//! - Double-quoted strings.
//! - Escaped apostrophes.
//! - Escaped quotation marks.
//! - Escaped backslashes.
//! - Backspace.
//! - Form feed.
//! - Line feed.
//! - Carriage return.
//! - Horizontal tab.
//! - Vertical tab.
//! - NUL.
//! - Four-digit Unicode escapes.
//! - Braced Unicode escapes.
//!
//! This module decodes already-recognized literal source text.
//! It does not replace the lexer or parser.
//!
//! ## UTF-8 CONTRACT
//!
//! All owned and borrowed strings are valid UTF-8.
//!
//! Byte offsets and Unicode scalar positions are different concepts.
//!
//! Byte offsets must always be UTF-8 character boundaries.
//!
//! Unicode scalar iteration uses Rust's `char` representation.
//!
//! Grapheme clusters require a separate Unicode segmentation
//! implementation and are deliberately not approximated here.
//!
//! ## SCALABILITY
//!
//! This module introduces no arbitrary maximum for:
//!
//! - String byte length.
//! - String scalar count.
//! - String capacity.
//! - String concatenation length.
//! - String builder operations.
//! - Formatting output.
//!
//! Actual growth is subject to available memory, Rust's allocation
//! behavior, and platform representation limits.
//!
//! Fallible reservation is provided where Rust exposes it.
//!
//! This does not promise literally infinite memory.
//!
//! ## POCO-REAF
//!
//! String meaning is independent of:
//!
//! - CPU architecture.
//! - Processor count.
//! - GPU architecture.
//! - Quantum processor architecture.
//! - FPGA or ASIC implementation.
//! - Memory capacity.
//! - Operating system.
//! - Distributed topology.
//!
//! The same valid UTF-8 string has the same textual meaning
//! across compatible Zamani implementations.
//!
//! ## ERROR CONTRACT
//!
//! Uses:
//!
//! `crate::stdlib::error::{Error, ErrorKind, Result}`
//!
//! Ordinary invalid inputs return structured errors.
//!
//! Allocation reservation failures are propagated.
//!
//! No ordinary malformed input causes a panic.
//!
//! ## SAFETY
//!
//! No unsafe Rust.
//!
//! No unchecked UTF-8 conversion.
//!
//! No unchecked indexing.
//!
//! No global mutable state.
//!
//! No implicit logging.
//!
//! No hidden truncation.
//!
//! No silent replacement of invalid Unicode.

use core::cmp::Ordering;
use core::fmt;
use core::str::FromStr;

use crate::stdlib::error::{Error, ErrorKind, Result};

// ============================================================================
// ERROR HELPERS
// ============================================================================

const COMPONENT: &str = "stdlib.foundation.strings";

fn invalid_input(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::InvalidInput, message)
        .with_component(COMPONENT)
}

fn invalid_data(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::InvalidData, message)
        .with_component(COMPONENT)
}

fn allocation_error(
    operation: &'static str,
    source: std::collections::TryReserveError,
) -> Error {
    Error::new(
        ErrorKind::ResourceUnavailable,
        "Unable to reserve memory for string operation",
    )
    .with_component(COMPONENT)
    .with_operation(operation)
    .with_source(source)
}

// ============================================================================
// UTF-8 STRING VIEW
// ============================================================================

/// A borrowed, validated UTF-8 string view.
///
/// `StringView` does not allocate or own the underlying string.
///
/// It is a lightweight abstraction over Rust's native `str`.
///
/// Byte offsets and scalar positions are explicitly distinguished.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringView<'a> {
    value: &'a str,
}

impl<'a> StringView<'a> {
    /// Constructs a view over an existing UTF-8 string.
    #[must_use]
    pub const fn new(value: &'a str) -> Self {
        Self { value }
    }

    /// Returns the underlying string slice.
    #[must_use]
    pub const fn as_str(self) -> &'a str {
        self.value
    }

    /// Returns the UTF-8 byte length.
    #[must_use]
    pub const fn byte_len(self) -> usize {
        self.value.len()
    }

    /// Returns the number of Unicode scalar values.
    #[must_use]
    pub fn scalar_len(self) -> usize {
        self.value.chars().count()
    }

    /// Returns whether the view is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.value.is_empty()
    }

    /// Returns the Unicode scalar iterator.
    pub fn chars(self) -> std::str::Chars<'a> {
        self.value.chars()
    }

    /// Returns the byte iterator.
    pub fn bytes(self) -> std::str::Bytes<'a> {
        self.value.bytes()
    }

    /// Returns the character indices.
    ///
    /// Each index is a UTF-8 byte offset.
    pub fn char_indices(self) -> std::str::CharIndices<'a> {
        self.value.char_indices()
    }

    /// Returns a validated UTF-8 byte offset.
    ///
    /// # Errors
    ///
    /// Returns an error if the offset is outside the string or
    /// is not a UTF-8 character boundary.
    pub fn byte_offset(self, offset: usize) -> Result<Utf8ByteOffset> {
        if !self.value.is_char_boundary(offset) {
            return Err(invalid_input(
                "Byte offset is not a valid UTF-8 character boundary",
            ));
        }

        Ok(Utf8ByteOffset(offset))
    }

    /// Returns the substring beginning at a validated byte offset.
    ///
    /// # Errors
    ///
    /// Returns an error if the offset is invalid.
    pub fn slice_from(self, offset: Utf8ByteOffset) -> Result<&'a str> {
        self.value
            .get(offset.0..)
            .ok_or_else(|| invalid_input("Invalid UTF-8 byte offset"))
    }

    /// Returns a substring using validated byte offsets.
    ///
    /// # Errors
    ///
    /// Returns an error when either offset is invalid or the
    /// requested range is reversed.
    pub fn slice(
        self,
        start: Utf8ByteOffset,
        end: Utf8ByteOffset,
    ) -> Result<&'a str> {
        if start.0 > end.0 {
            return Err(invalid_input(
                "String slice start exceeds its end",
            ));
        }

        self.value
            .get(start.0..end.0)
            .ok_or_else(|| invalid_input("Invalid UTF-8 string slice"))
    }

    /// Returns whether the string contains another string.
    #[must_use]
    pub fn contains(self, needle: &str) -> bool {
        self.value.contains(needle)
    }

    /// Returns whether the string starts with the given prefix.
    #[must_use]
    pub fn starts_with(self, prefix: &str) -> bool {
        self.value.starts_with(prefix)
    }

    /// Returns whether the string ends with the given suffix.
    #[must_use]
    pub fn ends_with(self, suffix: &str) -> bool {
        self.value.ends_with(suffix)
    }

    /// Compares two views lexicographically by UTF-8 string contents.
    #[must_use]
    pub fn compare(self, other: StringView<'_>) -> Ordering {
        self.value.cmp(other.value)
    }
}

impl<'a> From<&'a str> for StringView<'a> {
    fn from(value: &'a str) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for StringView<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.value)
    }
}

// ============================================================================
// UTF-8 BYTE OFFSETS
// ============================================================================

/// A UTF-8 byte offset.
///
/// This is not a Unicode scalar index.
///
/// Instances created through [`StringView::byte_offset`] are
/// validated against the associated string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Utf8ByteOffset(usize);

impl Utf8ByteOffset {
    /// Returns the underlying byte offset.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}

// ============================================================================
// OWNED STRING
// ============================================================================

/// An owned UTF-8 string.
///
/// This is a thin domain-oriented wrapper over Rust's standard
/// `String`, not a competing string representation.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZamaniString {
    value: String,
}

impl ZamaniString {
    /// Creates an empty string.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    /// Creates an empty string with fallibly reserved capacity.
    ///
    /// # Errors
    ///
    /// Returns a structured resource error when reservation fails.
    pub fn try_with_capacity(capacity: usize) -> Result<Self> {
        let mut value = String::new();

        value
            .try_reserve_exact(capacity)
            .map_err(|error| allocation_error(
                "try_with_capacity",
                error,
            ))?;

        Ok(Self { value })
    }

    /// Constructs an owned string by copying a UTF-8 slice.
    ///
    /// # Errors
    ///
    /// Returns a structured error if memory reservation fails.
    pub fn try_from_str(value: &str) -> Result<Self> {
        let mut output = Self::new();
        output.try_push_str(value)?;
        Ok(output)
    }

    /// Returns the underlying UTF-8 string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    /// Returns a borrowed string view.
    #[must_use]
    pub fn view(&self) -> StringView<'_> {
        StringView::new(self.as_str())
    }

    /// Returns the UTF-8 byte length.
    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.value.len()
    }

    /// Returns the Unicode scalar count.
    #[must_use]
    pub fn scalar_len(&self) -> usize {
        self.value.chars().count()
    }

    /// Returns the current allocation capacity.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.value.capacity()
    }

    /// Returns whether the string is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Clears the string while retaining its allocation.
    pub fn clear(&mut self) {
        self.value.clear();
    }

    /// Requests additional capacity without changing string contents.
    ///
    /// The requested amount is additional capacity relative to length.
    ///
    /// # Errors
    ///
    /// Returns a structured error if reservation fails.
    pub fn try_reserve(&mut self, additional: usize) -> Result<()> {
        self.value
            .try_reserve(additional)
            .map_err(|error| allocation_error(
                "try_reserve",
                error,
            ))
    }

    /// Requests exact additional capacity.
    ///
    /// # Errors
    ///
    /// Returns a structured error if reservation fails.
    pub fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> Result<()> {
        self.value
            .try_reserve_exact(additional)
            .map_err(|error| allocation_error(
                "try_reserve_exact",
                error,
            ))
    }

    /// Appends a UTF-8 string.
    ///
    /// Capacity is reserved before modifying the string.
    ///
    /// # Errors
    ///
    /// Returns a structured error if capacity reservation fails.
    pub fn try_push_str(&mut self, value: &str) -> Result<()> {
        self.try_reserve(value.len())?;
        self.value.push_str(value);
        Ok(())
    }

    /// Appends a Unicode scalar value.
    ///
    /// # Errors
    ///
    /// Returns a structured error if capacity reservation fails.
    pub fn try_push(&mut self, value: char) -> Result<()> {
        self.try_reserve(value.len_utf8())?;
        self.value.push(value);
        Ok(())
    }

    /// Appends another owned string.
    ///
    /// # Errors
    ///
    /// Returns a structured error if capacity reservation fails.
    pub fn try_append(&mut self, other: &Self) -> Result<()> {
        self.try_push_str(other.as_str())
    }

    /// Removes all trailing and leading Unicode whitespace.
    ///
    /// This returns a new allocation-free borrowed view.
    #[must_use]
    pub fn trimmed(&self) -> StringView<'_> {
        StringView::new(self.value.trim())
    }

    /// Returns whether the string contains the given substring.
    #[must_use]
    pub fn contains(&self, needle: &str) -> bool {
        self.value.contains(needle)
    }

    /// Returns the owned Rust string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.value
    }

    /// Parses a value using its standard `FromStr` implementation.
    ///
    /// # Errors
    ///
    /// Returns a structured invalid-input error when parsing fails.
    pub fn parse<T>(&self) -> Result<T>
    where
        T: FromStr,
        T::Err: fmt::Display,
    {
        self.value.parse::<T>().map_err(|error| {
            invalid_input(format!(
                "String parsing failed: {error}"
            ))
        })
    }

    /// Constructs a string from a validated UTF-8 byte vector.
    ///
    /// Rust `String::from_utf8` performs the validation.
    ///
    /// # Errors
    ///
    /// Returns an error when the supplied bytes are not valid UTF-8.
    pub fn from_utf8(bytes: Vec<u8>) -> Result<Self> {
        String::from_utf8(bytes)
            .map(|value| Self { value })
            .map_err(|error| {
                invalid_data(format!(
                    "Invalid UTF-8 input: {error}"
                ))
            })
    }
}

impl From<String> for ZamaniString {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<ZamaniString> for String {
    fn from(value: ZamaniString) -> Self {
        value.value
    }
}

impl From<&str> for ZamaniString {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}

impl fmt::Display for ZamaniString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.value)
    }
}

impl AsRef<str> for ZamaniString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl core::ops::Deref for ZamaniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

// ============================================================================
// FALLIBLE STRING BUILDER
// ============================================================================

/// A fallible incremental string builder.
///
/// This builder reserves capacity before each append operation.
///
/// It does not impose a fixed output length.
///
/// The caller remains responsible for any explicit operational
/// budget or output policy.
#[derive(Debug, Clone, Default)]
pub struct StringBuilder {
    value: String,
}

impl StringBuilder {
    /// Creates an empty builder.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    /// Creates a builder with fallibly reserved capacity.
    ///
    /// # Errors
    ///
    /// Returns a resource error when reservation fails.
    pub fn try_with_capacity(capacity: usize) -> Result<Self> {
        let mut value = String::new();

        value
            .try_reserve_exact(capacity)
            .map_err(|error| allocation_error(
                "builder_try_with_capacity",
                error,
            ))?;

        Ok(Self { value })
    }

    /// Returns the current contents.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Returns the current byte length.
    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.value.len()
    }

    /// Returns whether the builder is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Appends a string slice.
    ///
    /// # Errors
    ///
    /// Returns a resource error if capacity reservation fails.
    pub fn push_str(&mut self, value: &str) -> Result<&mut Self> {
        self.value
            .try_reserve(value.len())
            .map_err(|error| allocation_error(
                "builder_push_str",
                error,
            ))?;

        self.value.push_str(value);

        Ok(self)
    }

    /// Appends a Unicode scalar.
    ///
    /// # Errors
    ///
    /// Returns a resource error if capacity reservation fails.
    pub fn push(&mut self, value: char) -> Result<&mut Self> {
        self.value
            .try_reserve(value.len_utf8())
            .map_err(|error| allocation_error(
                "builder_push",
                error,
            ))?;

        self.value.push(value);

        Ok(self)
    }

    /// Appends a line terminator.
    ///
    /// # Errors
    ///
    /// Returns a resource error if capacity reservation fails.
    pub fn push_line(&mut self, value: &str) -> Result<&mut Self> {
        self.push_str(value)?;
        self.push('\n')
    }

    /// Appends a display-formatted value.
    ///
    /// Formatting occurs into a temporary string first.
    ///
    /// # Errors
    ///
    /// Returns an error if formatting or capacity reservation fails.
    pub fn push_display<T: fmt::Display>(
        &mut self,
        value: &T,
    ) -> Result<&mut Self> {
        let formatted = value.to_string();
        self.push_str(&formatted)
    }

    /// Appends a debug-formatted value.
    ///
    /// # Errors
    ///
    /// Returns an error if formatting or capacity reservation fails.
    pub fn push_debug<T: fmt::Debug>(
        &mut self,
        value: &T,
    ) -> Result<&mut Self> {
        let formatted = format!("{value:?}");
        self.push_str(&formatted)
    }

    /// Clears the builder while retaining capacity.
    pub fn clear(&mut self) {
        self.value.clear();
    }

    /// Consumes the builder and returns the resulting string.
    #[must_use]
    pub fn finish(self) -> ZamaniString {
        ZamaniString { value: self.value }
    }

    /// Consumes the builder and returns its Rust string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.value
    }
}

// ============================================================================
// FORMATTING
// ============================================================================

/// Writes formatted output into an existing formatting destination.
///
/// This function does not allocate its own output buffer.
///
/// # Errors
///
/// Returns the destination's formatting error.
pub fn write_formatted(
    destination: &mut impl fmt::Write,
    arguments: fmt::Arguments<'_>,
) -> fmt::Result {
    destination.write_fmt(arguments)
}

/// Creates a string from a display-formatted value.
///
/// # Errors
///
/// Returns a resource error if the resulting string cannot be
/// reserved by the builder.
pub fn try_format<T: fmt::Display>(value: &T) -> Result<ZamaniString> {
    let mut builder = StringBuilder::new();
    builder.push_display(value)?;
    Ok(builder.finish())
}

// ============================================================================
// GENERIC PARSING
// ============================================================================

/// Parses a UTF-8 string into a requested type.
///
/// The target type defines its own parsing semantics.
///
/// # Errors
///
/// Returns a structured invalid-input error if parsing fails.
pub fn parse<T>(value: &str) -> Result<T>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    value.parse::<T>().map_err(|error| {
        invalid_input(format!(
            "Unable to parse string value: {error}"
        ))
    })
}

// ============================================================================
// ZAMANI STRING LITERAL DECODING
// ============================================================================

/// Decodes an ordinary Zamani double-quoted string literal.
///
/// This function accepts the lexical contract established by:
///
/// `grammar/lexer/string-literals.g4`
///
/// It does not tokenize source code.
///
/// It expects the complete source spelling, including both
/// double-quote delimiters.
///
/// Supported escapes:
///
/// - `\'`
/// - `\"`
/// - `\\`
/// - `\b`
/// - `\f`
/// - `\n`
/// - `\r`
/// - `\t`
/// - `\v`
/// - `\0`
/// - `\uXXXX`
/// - `\u{HEX_DIGITS}`
///
/// Unicode escapes must resolve to valid Unicode scalar values.
///
/// Unknown escapes, malformed delimiters, invalid hexadecimal
/// digits, surrogate values, and out-of-range code points are
/// rejected.
///
/// No normalization or lossy replacement is performed.
///
/// # Errors
///
/// Returns an error when the source spelling is malformed or
/// contains an invalid Unicode scalar value.
pub fn decode_zamani_literal(source: &str) -> Result<ZamaniString> {
    let bytes = source.as_bytes();

    if bytes.len() < 2
        || bytes.first() != Some(&b'"')
        || bytes.last() != Some(&b'"')
    {
        return Err(invalid_data(
            "Expected a complete double-quoted Zamani string literal",
        ));
    }

    let body = &source[1..source.len() - 1];

    let mut builder = StringBuilder::new();
    let mut characters = body.chars();

    while let Some(character) = characters.next() {
        if character != '\\' {
            if character == '\n' || character == '\r' {
                return Err(invalid_data(
                    "Raw line breaks are not permitted in ordinary Zamani string literals",
                ));
            }

            builder.push(character)?;
            continue;
        }

        let escaped = characters.next().ok_or_else(|| {
            invalid_data("Incomplete string escape sequence")
        })?;

        match escaped {
            '\'' => builder.push('\'')?,
            '"' => builder.push('"')?,
            '\\' => builder.push('\\')?,
            'b' => builder.push('\u{0008}')?,
            'f' => builder.push('\u{000C}')?,
            'n' => builder.push('\n')?,
            'r' => builder.push('\r')?,
            't' => builder.push('\t')?,
            'v' => builder.push('\u{000B}')?,
            '0' => builder.push('\0')?,
            'u' => {
                let scalar = decode_unicode_escape(&mut characters)?;
                builder.push(scalar)?;
            }
            _ => {
                return Err(invalid_data(format!(
                    "Unsupported Zamani string escape: \\{escaped}"
                )));
            }
        }
    }

    Ok(builder.finish())
}

/// Decodes the Unicode portion of a Zamani escape.
///
/// Supports:
///
/// - Four hexadecimal digits.
/// - Braced hexadecimal digits.
///
/// Braced escapes have no arbitrary digit-count ceiling.
///
/// Numerical overflow is detected explicitly.
fn decode_unicode_escape(
    characters: &mut impl Iterator<Item = char>,
) -> Result<char> {
    let first = characters.next().ok_or_else(|| {
        invalid_data("Incomplete Unicode escape")
    })?;

    let code_point = if first == '{' {
        let mut value: u32 = 0;
        let mut found_digit = false;

        loop {
            let character = characters.next().ok_or_else(|| {
                invalid_data("Unterminated braced Unicode escape")
            })?;

            if character == '}' {
                break;
            }

            let digit = character.to_digit(16).ok_or_else(|| {
                invalid_data("Invalid hexadecimal digit in Unicode escape")
            })?;

            found_digit = true;

            value = value
                .checked_mul(16)
                .and_then(|current| current.checked_add(digit))
                .ok_or_else(|| {
                    invalid_data("Unicode escape value overflow")
                })?;
        }

        if !found_digit {
            return Err(invalid_data(
                "Braced Unicode escape must contain hexadecimal digits",
            ));
        }

        value
    } else {
        let mut value = first.to_digit(16).ok_or_else(|| {
            invalid_data("Invalid hexadecimal digit in Unicode escape")
        })?;

        for _ in 0..3 {
            let character = characters.next().ok_or_else(|| {
                invalid_data("Incomplete four-digit Unicode escape")
            })?;

            let digit = character.to_digit(16).ok_or_else(|| {
                invalid_data("Invalid hexadecimal digit in Unicode escape")
            })?;

            value = value * 16 + digit;
        }

        value
    };

    char::from_u32(code_point).ok_or_else(|| {
        invalid_data(format!(
            "Unicode escape does not represent a valid Unicode scalar: U+{code_point:X}"
        ))
    })
}

// ============================================================================
// UNICODE OPERATIONS
// ============================================================================

/// Returns the number of Unicode scalar values in a string.
///
/// This is not a grapheme-cluster count.
#[must_use]
pub fn scalar_count(value: &str) -> usize {
    value.chars().count()
}

/// Returns the UTF-8 byte length of a string.
#[must_use]
pub const fn byte_count(value: &str) -> usize {
    value.len()
}

/// Returns whether a byte position is a valid UTF-8 boundary.
#[must_use]
pub fn is_utf8_boundary(value: &str, offset: usize) -> bool {
    value.is_char_boundary(offset)
}

/// Returns an iterator over Unicode scalar values.
pub fn unicode_scalars(value: &str) -> std::str::Chars<'_> {
    value.chars()
}

/// Returns an iterator over Unicode scalars and their byte positions.
pub fn unicode_scalar_indices(
    value: &str,
) -> std::str::CharIndices<'_> {
    value.char_indices()
}

/// Returns whether two strings have identical UTF-8 contents.
#[must_use]
pub fn equals(left: &str, right: &str) -> bool {
    left == right
}

/// Returns the lexicographical comparison of two strings.
#[must_use]
pub fn compare(left: &str, right: &str) -> Ordering {
    left.cmp(right)
}

/// Returns a lowercase Unicode transformation.
///
/// This uses Rust's Unicode-aware character mapping.
///
/// It is not locale-specific case conversion.
pub fn lowercase(value: &str) -> String {
    value.chars().flat_map(char::to_lowercase).collect()
}

/// Returns an uppercase Unicode transformation.
///
/// This uses Rust's Unicode-aware character mapping.
///
/// It is not locale-specific case conversion.
pub fn uppercase(value: &str) -> String {
    value.chars().flat_map(char::to_uppercase).collect()
}

/// Returns a whitespace-trimmed borrowed view.
#[must_use]
pub fn trim(value: &str) -> &str {
    value.trim()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_is_supported() {
        let value = ZamaniString::new();

        assert!(value.is_empty());
        assert_eq!(value.byte_len(), 0);
        assert_eq!(value.scalar_len(), 0);
    }

    #[test]
    fn utf8_byte_and_scalar_lengths_are_distinct() {
        let value = ZamaniString::from("Zamani λ 😀");

        assert_eq!(value.scalar_len(), 10);
        assert!(value.byte_len() > value.scalar_len());
    }

    #[test]
    fn borrowed_view_does_not_change_contents() {
        let value = "Zamani";

        let view = StringView::new(value);

        assert_eq!(view.as_str(), "Zamani");
        assert_eq!(view.byte_len(), 6);
        assert_eq!(view.scalar_len(), 6);
    }

    #[test]
    fn unicode_iteration_works() {
        let value = "Aλ😀";

        let scalars: Vec<char> = unicode_scalars(value).collect();

        assert_eq!(scalars, vec!['A', 'λ', '😀']);
    }

    #[test]
    fn utf8_offsets_are_validated() {
        let view = StringView::new("AλB");

        assert!(view.byte_offset(0).is_ok());
        assert!(view.byte_offset(1).is_ok());
        assert!(view.byte_offset(2).is_err());
        assert!(view.byte_offset(3).is_ok());
        assert!(view.byte_offset(4).is_ok());
    }

    #[test]
    fn utf8_slicing_is_checked() {
        let view = StringView::new("AλB");

        let start = view.byte_offset(1).unwrap();
        let end = view.byte_offset(3).unwrap();

        assert_eq!(view.slice(start, end).unwrap(), "λ");
    }

    #[test]
    fn reversed_slices_are_rejected() {
        let view = StringView::new("Zamani");

        let start = view.byte_offset(4).unwrap();
        let end = view.byte_offset(2).unwrap();

        assert!(view.slice(start, end).is_err());
    }

    #[test]
    fn string_builder_appends_unicode() {
        let mut builder = StringBuilder::new();

        builder.push_str("Zamani").unwrap();
        builder.push(' ').unwrap();
        builder.push('λ').unwrap();

        assert_eq!(builder.as_str(), "Zamani λ");
    }

    #[test]
    fn string_builder_preserves_existing_contents() {
        let mut builder = StringBuilder::new();

        builder.push_str("first").unwrap();

        let before = builder.as_str().to_owned();

        assert!(builder.push_str("second").is_ok());
        assert_eq!(before, "first");
        assert_eq!(builder.as_str(), "firstsecond");
    }

    #[test]
    fn fallible_capacity_construction_works() {
        let builder = StringBuilder::try_with_capacity(64).unwrap();

        assert!(builder.is_empty());
    }

    #[test]
    fn owned_string_round_trip_works() {
        let original = String::from("Zamani λ");

        let value = ZamaniString::try_from_str(&original).unwrap();

        assert_eq!(value.as_str(), original);
        assert_eq!(value.into_string(), original);
    }

    #[test]
    fn utf8_validation_accepts_valid_data() {
        let bytes = "Zamani λ".as_bytes().to_vec();

        let value = ZamaniString::from_utf8(bytes).unwrap();

        assert_eq!(value.as_str(), "Zamani λ");
    }

    #[test]
    fn utf8_validation_rejects_invalid_data() {
        let bytes = vec![0xFF, 0xFE];

        assert!(ZamaniString::from_utf8(bytes).is_err());
    }

    #[test]
    fn generic_integer_parsing_works() {
        let value: i128 = parse("12345678901234567890").unwrap();

        assert_eq!(value, 12345678901234567890_i128);
    }

    #[test]
    fn invalid_integer_parsing_returns_error() {
        assert!(parse::<i64>("not-a-number").is_err());
    }

    #[test]
    fn generic_float_parsing_works() {
        let value: f64 = parse("3.1415926535").unwrap();

        assert!((value - 3.1415926535).abs() < 0.0000000001);
    }

    #[test]
    fn display_formatting_works() {
        let value = 12345;

        let formatted = try_format(&value).unwrap();

        assert_eq!(formatted.as_str(), "12345");
    }

    #[test]
    fn zamani_empty_literal_decodes() {
        let value = decode_zamani_literal("\"\"").unwrap();

        assert_eq!(value.as_str(), "");
    }

    #[test]
    fn zamani_ordinary_literal_decodes() {
        let value = decode_zamani_literal("\"Hello Zamani\"").unwrap();

        assert_eq!(value.as_str(), "Hello Zamani");
    }

    #[test]
    fn zamani_escape_sequences_decode() {
        let value = decode_zamani_literal(
            r#""\' \" \\ \b \f \n \r \t \v \0""#,
        )
        .unwrap();

        assert_eq!(
            value.as_str(),
            "' \" \\ \u{0008} \u{000C} \n \r \t \u{000B} \0"
        );
    }

    #[test]
    fn fixed_unicode_escape_decodes() {
        let value = decode_zamani_literal(r#""\u03BB""#).unwrap();

        assert_eq!(value.as_str(), "λ");
    }

    #[test]
    fn braced_unicode_escape_decodes() {
        let value = decode_zamani_literal(r#""\u{1F600}""#).unwrap();

        assert_eq!(value.as_str(), "😀");
    }

    #[test]
    fn supplementary_unicode_scalars_decode() {
        let value = decode_zamani_literal(r#""\u{10FFFF}""#).unwrap();

        assert_eq!(value.as_str(), "\u{10FFFF}");
    }

    #[test]
    fn invalid_unicode_surrogates_are_rejected() {
        assert!(decode_zamani_literal(r#""\u{D800}""#).is_err());
    }

    #[test]
    fn unicode_outside_scalar_range_is_rejected() {
        assert!(decode_zamani_literal(r#""\u{110000}""#).is_err());
    }

    #[test]
    fn malformed_unicode_escape_is_rejected() {
        assert!(decode_zamani_literal(r#""\u{XYZ}""#).is_err());
        assert!(decode_zamani_literal(r#""\u{}""#).is_err());
        assert!(decode_zamani_literal(r#""\u12""#).is_err());
    }

    #[test]
    fn unknown_escape_is_rejected() {
        assert!(decode_zamani_literal(r#""\q""#).is_err());
    }

    #[test]
    fn unterminated_literal_is_rejected() {
        assert!(decode_zamani_literal("\"unterminated").is_err());
    }

    #[test]
    fn raw_newline_is_rejected() {
        assert!(decode_zamani_literal("\"first\nsecond\"").is_err());
    }

    #[test]
    fn lowercase_unicode_conversion_works() {
        assert_eq!(lowercase("ZAMANI"), "zamani");
    }

    #[test]
    fn uppercase_unicode_conversion_works() {
        assert_eq!(uppercase("zamani"), "ZAMANI");
    }

    #[test]
    fn trim_handles_unicode_whitespace() {
        assert_eq!(trim("  Zamani λ  "), "Zamani λ");
    }

    #[test]
    fn comparison_is_deterministic() {
        assert_eq!(compare("alpha", "beta"), Ordering::Less);
        assert!(equals("Zamani", "Zamani"));
    }
}
