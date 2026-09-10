//! # Zamani Frontend AST — Source-Range Validation
//!
//! `src/frontend/ast/node/validation/source_ranges.rs`
//!
//! ## Purpose
//!
//! This module validates source-range invariants for the native Zamani AST.
//!
//! It deliberately validates **source coordinates**, not program semantics.
//!
//! The module is responsible for detecting malformed source locations such as:
//!
//! - reversed ranges;
//! - offsets outside the corresponding source unit;
//! - invalid UTF-8 boundaries when source text is available;
//! - invalid parent/child source containment when that policy is explicitly
//!   requested;
//! - invalid cross-source relationships when a caller explicitly requires
//!   same-source containment;
//! - arithmetic/conversion failures;
//! - inconsistent range relationships.
//!
//! It does **not** perform:
//!
//! - type checking;
//! - name resolution;
//! - generic resolution;
//! - resource checking;
//! - quantum validation;
//! - qubit-count validation;
//! - topology validation;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - backend validation;
//! - hardware validation;
//! - ZUIR validation.
//!
//! Those responsibilities belong to later compiler layers.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! lexer / parser
//!      │
//!      ▼
//! native Zamani AST
//!      │
//!      ├── NodeId
//!      ├── NodeKind
//!      ├── Span
//!      └── metadata
//!      │
//!      ▼
//! structural validation
//!      │
//!      ├── source_ranges.rs  ← this module
//!      ├── structural.rs
//!      └── invariants.rs
//!      │
//!      ▼
//! semantic analysis
//!      │
//!      ▼
//! semantic model
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ▼
//! domain / target / hardware lowering
//! ```
//!
//! ## POCO-REAF
//!
//! Source ranges contain no machine-size assumptions.
//!
//! There is no:
//!
//! - maximum qubit count;
//! - maximum AST node count;
//! - maximum source-file count;
//! - maximum program size;
//! - maximum register size;
//! - maximum operation count.
//!
//! The implementation therefore does not introduce an artificial scalability
//! boundary into Zamani's source representation.
//!
//! Operational compiler limits remain caller-controlled policy.
//!
//! ## Coordinate model
//!
//! The canonical `Span` supplied by the native AST uses:
//!
//! ```text
//! SourceId
//! start: SourceOffset
//! end: SourceOffset
//! ```
//!
//! with the half-open interval:
//!
//! ```text
//! [start, end)
//! ```
//!
//! `SourceOffset` is a UTF-8 byte offset.
//!
//! This module does not replace `Span` and does not define a second span type.
//! It consumes the canonical `Span` from:
//!
//! ```text
//! crate::frontend::ast::node::source::Span
//! ```
//!
//! ## Source-length validation
//!
//! A `Span` knows its coordinate values but intentionally does not own source
//! text. Consequently, the span type can guarantee:
//!
//! ```text
//! start <= end
//! ```
//!
//! but cannot by itself prove:
//!
//! ```text
//! end <= source.len()
//! ```
//!
//! This module supplies the missing validation when source text or an abstract
//! source-length provider is available.
//!
//! ## UTF-8 boundary validation
//!
//! Byte offsets are only valid slicing boundaries when they occur at UTF-8
//! character boundaries.
//!
//! This module can therefore validate boundaries against `&str` without
//! duplicating source-map functionality.
//!
//! Importantly, this does not slice the source string during validation.
//! Boundary checking uses `str::is_char_boundary`, avoiding unnecessary
//! allocations and avoiding accidental construction of large substrings.
//!
//! ## Cross-source nodes
//!
//! AST nodes can legitimately originate from different source units because of:
//!
//! - generated nodes;
//! - macro expansion;
//! - imported source;
//! - external-format import;
//! - tooling;
//! - future extension mechanisms.
//!
//! Therefore this module does **not** universally require every parent and child
//! to share a `SourceId`.
//!
//! Same-source containment is a configurable validation policy.
//!
//! ## Parent/child containment
//!
//! A normal parser-produced syntax tree generally has source ranges where a
//! parent contains the ranges of its syntactic children.
//!
//! However, generated/imported/desugared nodes can have different provenance.
//!
//! Therefore containment is not silently imposed as an unconditional language
//! invariant.
//!
//! The caller chooses one of:
//!
//! - no relationship validation;
//! - same-source containment;
//! - same-source non-strict containment;
//! - same-source strict containment.
//!
//! ## Dependency contract
//!
//! This file depends only on:
//!
//! - the canonical AST `Span`;
//! - the Rust standard library;
//! - `serde` indirectly through the canonical span types when serialized.
//!
//! It must not depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - routing;
//! - scheduling;
//! - QEC;
//! - resilience;
//! - runtime;
//! - backend implementations.
//!
//! ## Safety
//!
//! This module contains no `unsafe` code.
//!
//! `#![forbid(unsafe_code)]` prevents accidental introduction of unsafe Rust.
//!
//! ## Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use super::super::source::Span;

// =============================================================================
// Source length abstraction
// =============================================================================

/// Read-only source information required for source-range validation.
///
/// The AST validator deliberately depends on this tiny abstraction rather than
/// on a concrete source-map implementation.
///
/// This keeps the validation layer independent from:
//!
//! - filesystem access;
//! - virtual files;
//! - editors;
//! - network sources;
//! - parser implementations;
//! - source-map storage.
//!
//! A source provider only needs to answer two questions:
//!
//! 1. What is the byte length of a source?
//! 2. Is a particular byte offset a valid UTF-8 boundary?
///
/// The latter can be implemented efficiently when the provider already owns
/// the source text.
///
/// Implementations must be deterministic for the lifetime of a validation
/// operation.
pub trait SourceRangeProvider {
    /// Provider-specific error type.
    type Error;

    /// Returns the byte length of the identified source unit.
    ///
    /// The returned value is the length in bytes, not characters.
    fn byte_len(
        &self,
        source: super::super::source::SourceId,
    ) -> Result<Option<u64>, Self::Error>;

    /// Returns whether `offset` is a valid UTF-8 character boundary in the
    /// identified source unit.
    ///
    /// Returning `None` means that the source unit is unavailable to the
    /// provider.
    ///
    /// A provider that does not retain UTF-8 source text may return `None`.
    fn is_utf8_boundary(
        &self,
        source: super::super::source::SourceId,
        offset: u64,
    ) -> Result<Option<bool>, Self::Error>;
}

/// Convenience implementation for a single source string.
///
/// This is useful for parser tests, diagnostics, and small standalone
/// validation operations.
///
/// The supplied source is treated as belonging to `source_id`.
pub struct SingleSource<'a> {
    source_id: super::super::source::SourceId,
    source: &'a str,
}

impl<'a> SingleSource<'a> {
    /// Creates a provider for one source unit.
    #[must_use]
    pub const fn new(
        source_id: super::super::source::SourceId,
        source: &'a str,
    ) -> Self {
        Self {
            source_id,
            source,
        }
    }

    /// Returns the source identity represented by this provider.
    #[must_use]
    pub const fn source_id(&self) -> super::super::source::SourceId {
        self.source_id
    }

    /// Returns the source text.
    ///
    /// This accessor is intended for diagnostics and tests. Validation itself
    /// does not require exposing the source text.
    #[must_use]
    pub const fn source(&self) -> &'a str {
        self.source
    }
}

impl<'a> SourceRangeProvider for SingleSource<'a> {
    type Error = core::convert::Infallible;

    fn byte_len(
        &self,
        source: super::super::source::SourceId,
    ) -> Result<Option<u64>, Self::Error> {
        if source == self.source_id {
            Ok(Some(self.source.len() as u64))
        } else {
            Ok(None)
        }
    }

    fn is_utf8_boundary(
        &self,
        source: super::super::source::SourceId,
        offset: u64,
    ) -> Result<Option<bool>, Self::Error> {
        if source != self.source_id {
            return Ok(None);
        }

        let offset = match usize::try_from(offset) {
            Ok(value) => value,
            Err(_) => return Ok(Some(false)),
        };

        Ok(Some(self.source.is_char_boundary(offset)))
    }
}

// =============================================================================
// Validation policy
// =============================================================================

/// Policy controlling how source ranges are validated.
///
/// The defaults are deliberately structural and conservative without imposing
/// a language-specific parent/child policy.
///
/// In particular, the default does **not** require parent and child nodes to
/// originate from the same source file because generated and imported nodes
/// are valid architectural possibilities.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SourceRangePolicy {
    /// Whether source length should be checked when a provider is available.
    pub check_bounds: bool,

    /// Whether UTF-8 character boundaries should be checked when source text is
    /// available.
    pub check_utf8_boundaries: bool,

    /// Relationship required between a parent span and child span.
    pub containment: SpanContainmentPolicy,

    /// Whether the source identity itself must be considered when validating
    /// relationships.
    ///
    /// This is useful for ASTs whose nodes are known to originate from one
    /// source unit.
    pub require_same_source: bool,
}

impl Default for SourceRangePolicy {
    fn default() -> Self {
        Self {
            check_bounds: true,
            check_utf8_boundaries: true,
            containment: SpanContainmentPolicy::Ignore,
            require_same_source: false,
        }
    }
}

impl SourceRangePolicy {
    /// Returns the most permissive structural policy.
    ///
    /// This still validates the canonical `Span` invariants represented by the
    /// public span API, but does not require an external source provider.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            check_bounds: false,
            check_utf8_boundaries: false,
            containment: SpanContainmentPolicy::Ignore,
            require_same_source: false,
        }
    }

    /// Enables/disables source-length validation.
    #[must_use]
    pub const fn with_bounds_check(mut self, enabled: bool) -> Self {
        self.check_bounds = enabled;
        self
    }

    /// Enables/disables UTF-8 boundary validation.
    #[must_use]
    pub const fn with_utf8_boundary_check(mut self, enabled: bool) -> Self {
        self.check_utf8_boundaries = enabled;
        self
    }

    /// Sets the parent/child containment policy.
    #[must_use]
    pub const fn with_containment(
        mut self,
        policy: SpanContainmentPolicy,
    ) -> Self {
        self.containment = policy;
        self
    }

    /// Enables or disables same-source requirements for relationships.
    #[must_use]
    pub const fn with_same_source_requirement(
        mut self,
        enabled: bool,
    ) -> Self {
        self.require_same_source = enabled;
        self
    }
}

/// Parent/child source-span relationship policy.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SpanContainmentPolicy {
    /// Do not validate parent/child containment.
    ///
    /// This is the default because generated and imported AST nodes can have
    /// independent source provenance.
    #[default]
    Ignore,

    /// Require the parent to contain the complete child span.
    ///
    /// The spans must belong to the same source.
    Contained,

    /// Require strict containment.
    ///
    /// Equal spans are rejected.
    StrictlyContained,
}

// =============================================================================
// Validation errors
// =============================================================================

/// Errors produced by source-range validation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SourceRangeError {
    /// The source range itself is reversed.
    ///
    /// This should normally be impossible for spans constructed through the
    /// canonical `Span::new` constructor, but the validator retains this error
    /// category so deserialized/untrusted data can be diagnosed explicitly.
    Reversed {
        /// Start byte offset.
        start: u64,

        /// End byte offset.
        end: u64,
    },

    /// The range extends beyond the known source length.
    OutOfBounds {
        /// Source identity.
        source: super::super::source::SourceId,

        /// Node/range start.
        start: u64,

        /// Node/range end.
        end: u64,

        /// Known source byte length.
        source_len: u64,
    },

    /// The start offset is not a UTF-8 boundary.
    InvalidUtf8Boundary {
        /// Source identity.
        source: super::super::source::SourceId,

        /// Invalid byte offset.
        offset: u64,

        /// Whether the invalid offset was the range start or end.
        position: RangePosition,
    },

    /// The end offset is not a UTF-8 boundary.
    ///
    /// This variant is retained separately for compatibility with callers
    /// wanting to classify start/end failures without inspecting fields.
    InvalidEndUtf8Boundary {
        /// Source identity.
        source: super::super::source::SourceId,

        /// Invalid byte offset.
        offset: u64,
    },

    /// A parent and child do not have the required source identity.
    DifferentSources {
        /// Parent source identity.
        parent_source: super::super::source::SourceId,

        /// Child source identity.
        child_source: super::super::source::SourceId,
    },

    /// A child span is not contained by its parent.
    ChildNotContained {
        /// Parent span.
        parent: Span,

        /// Child span.
        child: Span,
    },

    /// A child span is equal to its parent when strict containment was
    /// requested.
    ChildNotStrictlyContained {
        /// Parent span.
        parent: Span,

        /// Child span.
        child: Span,
    },

    /// The source provider could not answer a requested query.
    Provider(String),
}

impl fmt::Display for SourceRangeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reversed { start, end } => {
                write!(
                    formatter,
                    "invalid source range: start {start} is greater than end {end}"
                )
            }

            Self::OutOfBounds {
                source,
                start,
                end,
                source_len,
            } => {
                write!(
                    formatter,
                    "source range [{start}, {end}) is outside {source} \
                     with byte length {source_len}"
                )
            }

            Self::InvalidUtf8Boundary {
                source,
                offset,
                position,
            } => {
                write!(
                    formatter,
                    "source range {position} offset {offset} is not a UTF-8 \
                     character boundary in {source}"
                )
            }

            Self::InvalidEndUtf8Boundary { source, offset } => {
                write!(
                    formatter,
                    "source range end offset {offset} is not a UTF-8 \
                     character boundary in {source}"
                )
            }

            Self::DifferentSources {
                parent_source,
                child_source,
            } => {
                write!(
                    formatter,
                    "parent span belongs to {parent_source}, but child span \
                     belongs to {child_source}"
                )
            }

            Self::ChildNotContained { parent, child } => {
                write!(
                    formatter,
                    "child source span {child} is not contained in parent \
                     source span {parent}"
                )
            }

            Self::ChildNotStrictlyContained { parent, child } => {
                write!(
                    formatter,
                    "child source span {child} is not strictly contained in \
                     parent source span {parent}"
                )
            }

            Self::Provider(error) => {
                write!(
                    formatter,
                    "source provider failed during range validation: {error}"
                )
            }
        }
    }
}

impl std::error::Error for SourceRangeError {}

/// Identifies which end of a span failed UTF-8 boundary validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RangePosition {
    /// Start of `[start, end)`.
    Start,

    /// End of `[start, end)`.
    End,
}

impl fmt::Display for RangePosition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start => formatter.write_str("start"),
            Self::End => formatter.write_str("end"),
        }
    }
}

// =============================================================================
// Validation report
// =============================================================================

/// Result statistics for a source-range validation operation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SourceRangeReport {
    /// Number of individual spans validated.
    pub spans_checked: usize,

    /// Number of parent/child relationships checked.
    pub relationships_checked: usize,

    /// Number of source-length checks actually performed.
    pub bounds_checks: usize,

    /// Number of UTF-8 boundary checks actually performed.
    pub utf8_boundary_checks: usize,
}

impl SourceRangeReport {
    /// Returns an empty report.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            spans_checked: 0,
            relationships_checked: 0,
            bounds_checks: 0,
            utf8_boundary_checks: 0,
        }
    }

    fn increment_spans(&mut self) -> Result<(), SourceRangeError> {
        self.spans_checked = self
            .spans_checked
            .checked_add(1)
            .ok_or_else(|| {
                SourceRangeError::Provider(
                    "source-range validation statistics overflowed"
                        .to_owned(),
                )
            })?;

        Ok(())
    }

    fn increment_relationships(&mut self) -> Result<(), SourceRangeError> {
        self.relationships_checked = self
            .relationships_checked
            .checked_add(1)
            .ok_or_else(|| {
                SourceRangeError::Provider(
                    "source-range validation statistics overflowed"
                        .to_owned(),
                )
            })?;

        Ok(())
    }

    fn increment_bounds(&mut self) -> Result<(), SourceRangeError> {
        self.bounds_checks = self
            .bounds_checks
            .checked_add(1)
            .ok_or_else(|| {
                SourceRangeError::Provider(
                    "source-range validation statistics overflowed"
                        .to_owned(),
                )
            })?;

        Ok(())
    }

    fn increment_utf8(&mut self) -> Result<(), SourceRangeError> {
        self.utf8_boundary_checks = self
            .utf8_boundary_checks
            .checked_add(1)
            .ok_or_else(|| {
                SourceRangeError::Provider(
                    "source-range validation statistics overflowed"
                        .to_owned(),
                )
            })?;

        Ok(())
    }
}

// =============================================================================
// Individual span validation
// =============================================================================

/// Validates one canonical AST source span.
///
/// This is the primary low-level entry point.
///
/// It validates the intrinsic coordinate ordering and, when a source provider
/// is supplied and the policy enables it:
///
/// - source bounds;
/// - UTF-8 boundaries.
///
/// It does not require source text when only structural coordinate validation
/// is desired.
///
/// # Errors
///
/// Returns the first structural source-range error encountered.
///
/// # Scalability
///
/// Complexity is `O(1)` apart from provider operations.
///
/// No AST traversal, allocation proportional to source size, or fixed-size
/// machine assumptions are introduced.
pub fn validate_span<P>(
    span: &Span,
    provider: Option<&P>,
    policy: SourceRangePolicy,
) -> Result<SourceRangeReport, SourceRangeError>
where
    P: SourceRangeProvider,
    P::Error: fmt::Display,
{
    let mut report = SourceRangeReport::empty();

    validate_span_with_report(span, provider, policy, &mut report)?;

    Ok(report)
}

/// Validates one span while accumulating into an existing report.
///
/// This function is useful to structural validation code that already walks
/// the AST and wants source-range validation to contribute to one aggregate
/// validation report.
pub fn validate_span_with_report<P>(
    span: &Span,
    provider: Option<&P>,
    policy: SourceRangePolicy,
    report: &mut SourceRangeReport,
) -> Result<(), SourceRangeError>
where
    P: SourceRangeProvider,
    P::Error: fmt::Display,
{
    let start = span.start_raw();
    let end = span.end_raw();

    report.increment_spans()?;

    // `Span::new` already guarantees this invariant for normally constructed
    // values. Keeping the check here makes the validator resilient to values
    // obtained from deserialization or future representations.
    if start > end {
        return Err(SourceRangeError::Reversed { start, end });
    }

    let Some(provider) = provider else {
        return Ok(());
    };

    if policy.check_bounds {
        report.increment_bounds()?;

        if let Some(source_len) = provider
            .byte_len(span.source())
            .map_err(|error| SourceRangeError::Provider(error.to_string()))?
        {
            if end > source_len {
                return Err(SourceRangeError::OutOfBounds {
                    source: span.source(),
                    start,
                    end,
                    source_len,
                });
            }
        }
    }

    if policy.check_utf8_boundaries {
        validate_utf8_boundary(
            span,
            provider,
            start,
            RangePosition::Start,
            report,
        )?;

        validate_utf8_boundary(
            span,
            provider,
            end,
            RangePosition::End,
            report,
        )?;
    }

    Ok(())
}

fn validate_utf8_boundary<P>(
    span: &Span,
    provider: &P,
    offset: u64,
    position: RangePosition,
    report: &mut SourceRangeReport,
) -> Result<(), SourceRangeError>
where
    P: SourceRangeProvider,
    P::Error: fmt::Display,
{
    report.increment_utf8()?;

    let boundary = provider
        .is_utf8_boundary(span.source(), offset)
        .map_err(|error| SourceRangeError::Provider(error.to_string()))?;

    match boundary {
        None | Some(true) => Ok(()),

        Some(false) => match position {
            RangePosition::Start => Err(SourceRangeError::InvalidUtf8Boundary {
                source: span.source(),
                offset,
                position,
            }),

            RangePosition::End => {
                Err(SourceRangeError::InvalidEndUtf8Boundary {
                    source: span.source(),
                    offset,
                })
            }
        },
    }
}

// =============================================================================
// Parent / child validation
// =============================================================================

/// Validates the source relationship between a parent and child span.
///
/// This function does not assume that all AST nodes must come from the same
/// source unit. That requirement is controlled by [`SourceRangePolicy`].
///
/// This is important for:
///
/// - macro expansion;
/// - generated code;
/// - imported syntax;
/// - external-format frontends;
/// - future source transformation systems.
///
/// The function is independent from AST node kinds and therefore works for
/// classical, quantum, hybrid, HDL, accelerator, AI, distributed, and future
/// source constructs.
pub fn validate_parent_child(
    parent: &Span,
    child: &Span,
    policy: SourceRangePolicy,
) -> Result<SourceRangeReport, SourceRangeError> {
    let mut report = SourceRangeReport::empty();

    validate_parent_child_with_report(parent, child, policy, &mut report)?;

    Ok(report)
}

/// Validates a parent/child relationship while accumulating statistics.
pub fn validate_parent_child_with_report(
    parent: &Span,
    child: &Span,
    policy: SourceRangePolicy,
    report: &mut SourceRangeReport,
) -> Result<(), SourceRangeError> {
    report.increment_relationships()?;

    if policy.containment == SpanContainmentPolicy::Ignore {
        return Ok(());
    }

    if policy.require_same_source && parent.source() != child.source() {
        return Err(SourceRangeError::DifferentSources {
            parent_source: parent.source(),
            child_source: child.source(),
        });
    }

    // Containment can only be meaningfully compared within the same source.
    // If the caller did not explicitly require same-source validation, a
    // cross-source pair is left alone rather than incorrectly comparing two
    // unrelated coordinate spaces.
    if parent.source() != child.source() {
        return Ok(());
    }

    match policy.containment {
        SpanContainmentPolicy::Ignore => Ok(()),

        SpanContainmentPolicy::Contained => {
            if parent.contains(*child) {
                Ok(())
            } else {
                Err(SourceRangeError::ChildNotContained {
                    parent: *parent,
                    child: *child,
                })
            }
        }

        SpanContainmentPolicy::StrictlyContained => {
            if parent.strictly_contains(*child) {
                Ok(())
            } else {
                Err(SourceRangeError::ChildNotStrictlyContained {
                    parent: *parent,
                    child: *child,
                })
            }
        }
    }
}

// =============================================================================
// Batch validation
// =============================================================================

/// Validates an iterator of source spans.
///
/// The iterator is consumed exactly once.
///
/// No intermediate collection is created, so memory usage remains independent
/// of the number of spans supplied, apart from the caller's iterator itself.
pub fn validate_spans<'a, I, P>(
    spans: I,
    provider: Option<&P>,
    policy: SourceRangePolicy,
) -> Result<SourceRangeReport, SourceRangeError>
where
    I: IntoIterator<Item = &'a Span>,
    P: SourceRangeProvider,
    P::Error: fmt::Display,
{
    let mut report = SourceRangeReport::empty();

    for span in spans {
        validate_span_with_report(span, provider, policy, &mut report)?;
    }

    Ok(report)
}

/// Validates an iterator of parent/child span pairs.
///
/// This function is deliberately generic over the iterator and does not create
/// a graph or AST abstraction of its own.
pub fn validate_relationships<'a, I>(
    relationships: I,
    policy: SourceRangePolicy,
) -> Result<SourceRangeReport, SourceRangeError>
where
    I: IntoIterator<Item = (&'a Span, &'a Span)>,
{
    let mut report = SourceRangeReport::empty();

    for (parent, child) in relationships {
        validate_parent_child_with_report(
            parent,
            child,
            policy,
            &mut report,
        )?;
    }

    Ok(report)
}

// =============================================================================
// Direct source-string helpers
// =============================================================================

/// Validates a span directly against one source string.
///
/// This is a convenience API around [`SingleSource`].
pub fn validate_span_in_source(
    span: &Span,
    source_id: super::super::source::SourceId,
    source: &str,
    policy: SourceRangePolicy,
) -> Result<SourceRangeReport, SourceRangeError> {
    let provider = SingleSource::new(source_id, source);

    validate_span(span, Some(&provider), policy)
}

/// Validates multiple spans against one source string without allocating an
/// intermediate collection.
pub fn validate_spans_in_source<'a, I>(
    spans: I,
    source_id: super::super::source::SourceId,
    source: &str,
    policy: SourceRangePolicy,
) -> Result<SourceRangeReport, SourceRangeError>
where
    I: IntoIterator<Item = &'a Span>,
{
    let provider = SingleSource::new(source_id, source);

    validate_spans(spans, Some(&provider), policy)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::source::{
        SourceId,
        SourceOffset,
    };

    fn source_id(value: u64) -> SourceId {
        SourceId::from_raw(value)
    }

    fn offset(value: u64) -> SourceOffset {
        SourceOffset::from_raw(value)
    }

    fn span(source: u64, start: u64, end: u64) -> Span {
        Span::new(
            source_id(source),
            offset(start),
            offset(end),
        )
        .expect("test span must be ordered")
    }

    #[test]
    fn validates_empty_span() {
        let value = span(1, 0, 0);

        let report = validate_span(
            &value,
            Option::<&SingleSource<'_>>::None,
            SourceRangePolicy::default(),
        )
        .expect("empty span is valid");

        assert_eq!(report.spans_checked, 1);
    }

    #[test]
    fn validates_half_open_range() {
        let value = span(1, 2, 7);

        assert_eq!(value.start_raw(), 2);
        assert_eq!(value.end_raw(), 7);
        assert_eq!(value.len(), 5);
    }

    #[test]
    fn rejects_out_of_bounds_end() {
        let value = span(1, 0, 8);

        let error = validate_span_in_source(
            &value,
            source_id(1),
            "abc",
            SourceRangePolicy::default(),
        )
        .expect_err("range must exceed source");

        assert!(matches!(
            error,
            SourceRangeError::OutOfBounds { .. }
        ));
    }

    #[test]
    fn accepts_end_at_eof() {
        let value = span(1, 0, 3);

        validate_span_in_source(
            &value,
            source_id(1),
            "abc",
            SourceRangePolicy::default(),
        )
        .expect("EOF is a valid half-open end");
    }

    #[test]
    fn accepts_unicode_character_boundaries() {
        let source = "aλb";

        // UTF-8:
        // a = byte 0
        // λ = bytes 1..3
        // b = byte 3
        let value = span(1, 1, 3);

        validate_span_in_source(
            &value,
            source_id(1),
            source,
            SourceRangePolicy::default(),
        )
        .expect("range covers the complete UTF-8 character");
    }

    #[test]
    fn rejects_non_character_boundary() {
        let source = "aλb";

        // Byte offset 2 lies inside λ.
        let value = span(1, 1, 2);

        let error = validate_span_in_source(
            &value,
            source_id(1),
            source,
            SourceRangePolicy::default(),
        )
        .expect_err("offset 2 is not a UTF-8 boundary");

        assert!(matches!(
            error,
            SourceRangeError::InvalidEndUtf8Boundary {
                source: _,
                offset: 2,
            }
        ));
    }

    #[test]
    fn accepts_parent_containment() {
        let parent = span(1, 0, 10);
        let child = span(1, 2, 7);

        let policy = SourceRangePolicy::default()
            .with_containment(SpanContainmentPolicy::Contained);

        validate_parent_child(&parent, &child, policy)
            .expect("child is contained");
    }

    #[test]
    fn rejects_parent_containment() {
        let parent = span(1, 0, 5);
        let child = span(1, 2, 7);

        let policy = SourceRangePolicy::default()
            .with_containment(SpanContainmentPolicy::Contained);

        let error = validate_parent_child(&parent, &child, policy)
            .expect_err("child exceeds parent");

        assert!(matches!(
            error,
            SourceRangeError::ChildNotContained { .. }
        ));
    }

    #[test]
    fn strict_containment_rejects_equal_spans() {
        let parent = span(1, 0, 5);
        let child = span(1, 0, 5);

        let policy = SourceRangePolicy::default()
            .with_containment(SpanContainmentPolicy::StrictlyContained);

        let error = validate_parent_child(&parent, &child, policy)
            .expect_err("equal spans are not strict containment");

        assert!(matches!(
            error,
            SourceRangeError::ChildNotStrictlyContained { .. }
        ));
    }

    #[test]
    fn cross_source_relationship_can_be_ignored() {
        let parent = span(1, 0, 5);
        let child = span(2, 0, 5);

        let policy = SourceRangePolicy::default()
            .with_containment(SpanContainmentPolicy::Contained);

        validate_parent_child(&parent, &child, policy)
            .expect("different provenance is not automatically invalid");
    }

    #[test]
    fn cross_source_relationship_can_be_rejected() {
        let parent = span(1, 0, 5);
        let child = span(2, 0, 5);

        let policy = SourceRangePolicy::default()
            .with_containment(SpanContainmentPolicy::Contained)
            .with_same_source_requirement(true);

        let error = validate_parent_child(&parent, &child, policy)
            .expect_err("same-source policy must reject mismatch");

        assert!(matches!(
            error,
            SourceRangeError::DifferentSources { .. }
        ));
    }

    #[test]
    fn unrestricted_policy_does_not_require_source_provider() {
        let value = span(999, 100, 200);

        validate_span(
            &value,
            Option::<&SingleSource<'_>>::None,
            SourceRangePolicy::unrestricted(),
        )
        .expect("coordinate ordering is sufficient for unrestricted mode");
    }

    #[test]
    fn batch_validation_is_single_pass() {
        let first = span(1, 0, 1);
        let second = span(1, 1, 2);
        let third = span(1, 2, 3);

        let spans = [&first, &second, &third];

        let report = validate_spans(
            spans,
            Option::<&SingleSource<'_>>::None,
            SourceRangePolicy::default(),
        )
        .expect("all spans are valid");

        assert_eq!(report.spans_checked, 3);
    }

    #[test]
    fn direct_source_validation_reports_checks() {
        let value = span(1, 0, 3);

        let report = validate_span_in_source(
            &value,
            source_id(1),
            "abc",
            SourceRangePolicy::default(),
        )
        .expect("valid source range");

        assert_eq!(report.spans_checked, 1);
        assert_eq!(report.bounds_checks, 1);
        assert_eq!(report.utf8_boundary_checks, 2);
    }

    #[test]
    fn validation_does_not_require_non_empty_source() {
        let value = span(1, 0, 0);

        validate_span_in_source(
            &value,
            source_id(1),
            "",
            SourceRangePolicy::default(),
        )
        .expect("empty source has valid EOF span");
    }

    #[test]
    fn source_identity_is_part_of_range_relationship() {
        let parent = span(10, 0, 10);
        let child = span(11, 0, 10);

        let policy = SourceRangePolicy::default()
            .with_containment(SpanContainmentPolicy::Contained)
            .with_same_source_requirement(true);

        let error = validate_parent_child(&parent, &child, policy)
            .expect_err("source identity must be checked");

        match error {
            SourceRangeError::DifferentSources {
                parent_source,
                child_source,
            } => {
                assert_eq!(parent_source, source_id(10));
                assert_eq!(child_source, source_id(11));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}