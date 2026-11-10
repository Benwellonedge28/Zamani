
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Foundation — Value Conversion
//!
//! Path: `src/foundation/conversion.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide reusable, target-independent value-conversion utilities for
//! Zamani's Rust implementation and its standard-library integration.
//!
//! This module distinguishes:
//!
//! - infallible conversions;
//! - fallible conversions;
//! - borrowed conversions;
//! - textual parsing;
//! - caller-defined conversions.
//!
//! ## Architectural ownership
//!
//! This module owns concrete value-conversion helpers only.
//!
//! It does not own:
//!
//! - source-language conversion legality;
//! - implicit coercion rules;
//! - type compatibility;
//! - type inference;
//! - semantic type definitions;
//! - AST conversion;
//! - constant evaluation;
//! - serialization formats;
//! - string-literal escape syntax;
//! - runtime scheduling;
//! - hardware representation;
//! - target-specific ABI conversion;
//! - device or memory allocation.
//!
//! Canonical semantic conversion authority:
//!
//! `crate::frontend::semantic::types::coercions`
//!
//! Canonical type compatibility authority:
//!
//! `crate::frontend::semantic::types::compatibility`
//!
//! Canonical semantic type authority:
//!
//! `crate::frontend::semantic::types::ty`
//!
//! Shared standard-library error authority:
//!
//! `crate::stdlib::error`
//!
//! ## Conversion policy
//!
//! `convert` is restricted to Rust conversions implemented through `From`.
//!
//! `try_convert` is restricted to Rust conversions implemented through
//! `TryFrom`.
//!
//! `parse` uses the target type's `FromStr` implementation.
//!
//! These functions do not authorize a conversion in Zamani source code.
//! Semantic validation must establish that a conversion is permitted before
//! the corresponding runtime operation is generated or executed.
//!
//! ## Error policy
//!
//! Fallible conversions return the canonical standard-library `Result<T>`.
//!
//! The original conversion or parsing error is preserved as the causal
//! source whenever its type implements `std::error::Error + Send + Sync`.
//!
//! No conversion failure is silently replaced with a default value.
//!
//! No failed conversion is reported as successful.
//!
//! ## Scalability
//!
//! This module introduces no language-level maximum for:
//!
//! - numeric widths;
//! - collection sizes;
//! - string lengths;
//! - tensor dimensions;
//! - conversion registrations;
//! - computational domains;
//! - hardware resources;
//! - distributed participants.
//!
//! The module does not impose fixed buffers or fixed-capacity conversion
//! tables.
//!
//! The actual source and target types determine their representable values.
//! Fallible conversions must report values that cannot be represented by
//! the target type.
//!
//! This does not promise literally infinite memory or computation. Rust
//! values remain subject to their representation, platform address space,
//! allocator behavior and available resources.
//!
//! ## Portability
//!
//! The functions operate on Rust values and traits. They do not inspect
//! hardware, select execution devices, or assume a particular processor,
//! operating system, accelerator or deployment topology.
//!
//! ## Dependencies
//!
//! - Rust standard library;
//! - `crate::stdlib::error`.
//!
//! No external crate is required.
//!
//! ## Freeze criteria
//!
//! - The module is exported through `crate::foundation`.
//! - The standard library exposes it through a compatibility facade.
//! - Semantic conversion remains owned by the frontend semantic subsystem.
//! - All fallible paths preserve structured errors and causal sources.
//! - No lossy conversion is silently introduced.
//! - Unit tests and workspace checks pass.

use core::any::type_name;
use core::convert::{From, TryFrom};
use core::fmt::Display;
use core::str::FromStr;

use crate::stdlib::error::{Error, ErrorKind, Result};

// =============================================================================
// Error construction
// =============================================================================

/// Constructs a structured error for a failed concrete value conversion.
///
/// The original conversion error is preserved as the causal source.
///
/// This helper is private so all public conversion operations use the
/// same error classification and component identity.
fn conversion_error<E>(
    kind: ErrorKind,
    operation: &'static str,
    source_type: &'static str,
    target_type: &'static str,
    source_error: E,
) -> Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    let message = format!(
        "conversion from `{source_type}` to `{target_type}` failed: \
         {source_error}"
    );

    Error::new(kind, message)
        .in_component("foundation.conversion")
        .during_operation(operation)
        .with_source(source_error)
}

// =============================================================================
// Infallible owned conversion
// =============================================================================

/// Converts an owned value using an implementation of Rust's [`From`] trait.
///
/// This function is only for conversions that the source and target types
/// already define as infallible.
///
/// It does not establish that an implicit conversion is legal in Zamani
/// source code. The semantic conversion subsystem remains authoritative.
///
/// # Examples
///
/// ```
/// use zamani_compiler::foundation::conversion;
///
/// let value: i64 = conversion::convert(42_i32);
///
/// assert_eq!(value, 42_i64);
/// ```
///
/// # Type parameters
///
/// - `Target`: the destination Rust type.
/// - `Source`: the owned source Rust type.
///
/// # Panics
///
/// This function contains no explicit panic path. A custom `From`
/// implementation can nevertheless panic internally; callers must use
/// well-behaved conversion implementations.
///
/// # Scalability
///
/// No intermediate fixed-size buffer or conversion table is created.
#[must_use]
pub fn convert<Target, Source>(source: Source) -> Target
where
    Target: From<Source>,
{
    Target::from(source)
}

// =============================================================================
// Infallible borrowed conversion
// =============================================================================

/// Converts a borrowed value using an implementation of [`From`].
///
/// This is useful when the destination can be constructed from a borrowed
/// source without requiring the caller to transfer ownership.
///
/// # Examples
///
/// ```
/// use zamani_compiler::foundation::conversion;
///
/// let source = "Zamani";
/// let owned: String = conversion::convert_ref(source);
///
/// assert_eq!(owned, "Zamani");
/// ```
///
/// The lifetime of the input borrow is determined by the caller. The
/// returned value is not artificially tied to that lifetime unless the
/// destination type itself requires it.
#[must_use]
pub fn convert_ref<'a, Target, Source>(source: &'a Source) -> Target
where
    Source: ?Sized,
    Target: From<&'a Source>,
{
    Target::from(source)
}

// =============================================================================
// Fallible owned conversion
// =============================================================================

/// Converts an owned value using Rust's [`TryFrom`] trait.
///
/// A conversion failure is returned as the shared Zamani standard-library
/// error type. The original conversion error is preserved as its causal
/// source.
///
/// This function does not truncate, wrap, saturate, clamp, or silently
/// replace an unrepresentable value.
///
/// # Examples
///
/// ```
/// use zamani_compiler::foundation::conversion;
///
/// let converted: i8 = conversion::try_convert(100_i16)
///     .expect("100 is representable as i8");
///
/// assert_eq!(converted, 100);
///
/// let rejected: zamani_compiler::stdlib::error::Result<i8> =
///     conversion::try_convert(1000_i16);
///
/// assert!(rejected.is_err());
/// ```
///
/// # Errors
///
/// Returns an error when the target type rejects the source value.
///
/// The error classification is `ErrorKind::InvalidInput`.
///
/// # Type parameters
///
/// - `Target`: the destination Rust type.
/// - `Source`: the owned source Rust type.
///
/// # Portability
///
/// The conversion is governed by the source and target type contracts,
/// not by a selected hardware provider.
pub fn try_convert<Target, Source>(source: Source) -> Result<Target>
where
    Target: TryFrom<Source>,
    <Target as TryFrom<Source>>::Error:
        std::error::Error + Send + Sync + 'static,
{
    Target::try_from(source).map_err(|error| {
        conversion_error(
            ErrorKind::InvalidInput,
            "try_convert",
            type_name::<Source>(),
            type_name::<Target>(),
            error,
        )
    })
}

// =============================================================================
// Fallible borrowed conversion
// =============================================================================

/// Converts a borrowed value using an implementation of [`TryFrom`].
///
/// The source remains owned by the caller.
///
/// # Errors
///
/// Returns a structured standard-library error when the target type
/// rejects the borrowed value.
///
/// # Example
///
/// ```
/// use zamani_compiler::foundation::conversion;
///
/// let source = "Zamani";
///
/// let converted: String = conversion::try_convert_ref(source)
///     .expect("valid borrowed conversion");
///
/// assert_eq!(converted, "Zamani");
/// ```
pub fn try_convert_ref<'a, Target, Source>(
    source: &'a Source,
) -> Result<Target>
where
    Source: ?Sized,
    Target: TryFrom<&'a Source>,
    <Target as TryFrom<&'a Source>>::Error:
        std::error::Error + Send + Sync + 'static,
{
    Target::try_from(source).map_err(|error| {
        conversion_error(
            ErrorKind::InvalidInput,
            "try_convert_ref",
            type_name::<Source>(),
            type_name::<Target>(),
            error,
        )
    })
}

// =============================================================================
// Text parsing
// =============================================================================

/// Parses text into a destination type using its [`FromStr`] implementation.
///
/// Parsing is distinct from conversion:
///
/// - conversion starts with an existing typed value;
/// - parsing starts with textual input.
///
/// This function does not define Zamani literal syntax, numeric literal
/// grammar, string escape syntax, or semantic literal typing.
///
/// Those remain owned by the grammar and frontend.
///
/// # Examples
///
/// ```
/// use zamani_compiler::foundation::conversion;
///
/// let count: u64 = conversion::parse("1842")
///     .expect("valid unsigned integer");
///
/// assert_eq!(count, 1842);
/// ```
///
/// # Errors
///
/// Returns `ErrorKind::InvalidData` when the destination parser rejects
/// the supplied text.
///
/// The original parsing error is retained as the causal source.
///
/// # Important
///
/// Parsing behavior is determined by the destination type's `FromStr`
/// implementation. This function does not silently trim whitespace,
/// normalize Unicode, change decimal separators, or apply locale rules.
pub fn parse<Target>(source: &str) -> Result<Target>
where
    Target: FromStr,
    <Target as FromStr>::Err:
        std::error::Error + Send + Sync + 'static,
{
    Target::from_str(source).map_err(|error| {
        conversion_error(
            ErrorKind::InvalidData,
            "parse",
            "str",
            type_name::<Target>(),
            error,
        )
    })
}

// =============================================================================
// Caller-defined conversion
// =============================================================================

/// Performs a caller-defined fallible conversion.
///
/// This function provides an integration point for domain-specific
/// implementations whose conversion behavior is not expressed by a
/// standard `TryFrom` implementation.
///
/// The supplied closure owns the actual conversion behavior.
///
/// This function does not register semantic coercion rules, authorize
/// source-language casts, inspect hardware, or choose providers.
///
/// # Arguments
///
/// - `source`: the value to convert.
/// - `converter`: a caller-supplied conversion operation.
///
/// # Errors
///
/// Returns a structured standard-library error if the closure returns
/// an error. The original error is preserved as the causal source.
///
/// # Example
///
/// ```
/// use zamani_compiler::foundation::conversion;
///
/// let result: zamani_compiler::stdlib::error::Result<u64> =
///     conversion::try_convert_with("42", |text| {
///         text.parse::<u64>()
///     });
///
/// assert_eq!(result.expect("valid number"), 42);
/// ```
pub fn try_convert_with<Source, Target, ConvertError, Converter>(
    source: Source,
    converter: Converter,
) -> Result<Target>
where
    ConvertError: std::error::Error + Send + Sync + 'static,
    Converter: FnOnce(Source) -> std::result::Result<Target, ConvertError>,
{
    converter(source).map_err(|error| {
        conversion_error(
            ErrorKind::InvalidInput,
            "try_convert_with",
            type_name::<Source>(),
            type_name::<Target>(),
            error,
        )
    })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use core::convert::Infallible;

    #[test]
    fn converts_owned_values_infallibly() {
        let value: i64 = convert(42_i32);

        assert_eq!(value, 42);
    }

    #[test]
    fn converts_borrowed_values_infallibly() {
        let source = "Zamani";
        let value: String = convert_ref(source);

        assert_eq!(value, "Zamani");
        assert_eq!(source, "Zamani");
    }

    #[test]
    fn converts_values_fallibly_when_representable() {
        let value: i8 = try_convert(120_i16)
            .expect("120 is representable");

        assert_eq!(value, 120);
    }

    #[test]
    fn rejects_values_outside_target_representation() {
        let result: Result<i8> = try_convert(128_i16);

        let error = result.expect_err("128 is outside i8");

        assert_eq!(error.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            error.component(),
            Some("foundation.conversion")
        );
        assert_eq!(
            error.operation(),
            Some("try_convert")
        );
        assert!(error.has_source());
    }

    #[test]
    fn rejects_negative_values_for_unsigned_targets() {
        let result: Result<u8> = try_convert(-1_i16);

        assert!(result.is_err());
    }

    #[test]
    fn supports_borrowed_fallible_conversions() {
        let source = "Zamani";

        let converted: String = try_convert_ref(source)
            .expect("str can be converted to String");

        assert_eq!(converted, "Zamani");
        assert_eq!(source, "Zamani");
    }

    #[test]
    fn parses_valid_text() {
        let value: u64 = parse("1842")
            .expect("valid integer");

        assert_eq!(value, 1842);
    }

    #[test]
    fn rejects_invalid_text_without_defaulting() {
        let result: Result<u64> = parse("not-a-number");

        let error = result.expect_err("invalid integer text");

        assert_eq!(error.kind(), ErrorKind::InvalidData);
        assert_eq!(error.operation(), Some("parse"));
        assert!(error.has_source());
    }

    #[test]
    fn parsing_does_not_silently_trim_whitespace() {
        let result: Result<u64> = parse(" 42 ");

        assert!(result.is_err());
    }

    #[test]
    fn supports_caller_defined_conversions() {
        let result: Result<u64> =
            try_convert_with("42", |text| text.parse::<u64>());

        assert_eq!(result.expect("valid value"), 42);
    }

    #[test]
    fn caller_defined_conversion_errors_are_preserved() {
        let result: Result<u64> =
            try_convert_with("invalid", |text| {
                text.parse::<u64>()
            });

        let error = result.expect_err("invalid text");

        assert_eq!(error.kind(), ErrorKind::InvalidInput);
        assert!(error.has_source());
    }

    #[test]
    fn zero_and_boundary_values_are_handled_by_target_types() {
        let zero: u8 = try_convert(0_i16)
            .expect("zero is representable");

        let maximum: u8 = try_convert(i16::from(u8::MAX))
            .expect("u8 maximum is representable");

        assert_eq!(zero, 0);
        assert_eq!(maximum, u8::MAX);
    }

    #[test]
    fn conversion_does_not_invent_an_infallible_path() {
        let result: Result<i8> = try_convert(1000_i16);

        assert!(result.is_err());
    }

    #[test]
    fn infallible_standard_conversion_remains_available() {
        let source = String::from("Zamani");

        let result: std::result::Result<String, Infallible> =
            Ok(convert(source));

        assert_eq!(
            result.expect("infallible conversion"),
            "Zamani"
        );
    }
}
