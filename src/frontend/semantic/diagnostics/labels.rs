
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Structured source labels for Zamani semantic diagnostics.
//!
//! File: `src/frontend/semantic/diagnostics/labels.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Represent primary and secondary source labels attached to diagnostics.
//! Labels identify source ranges and provide concise explanatory text.
//!
//! # AUTHORITY
//!
//! - `grammar/spec/diagnostics.md` defines the diagnostic contract.
//! - `src/frontend/ast/node/source/` owns canonical source identities,
//!   offsets, spans, and source-location representations.
//! - `diagnostics/diagnostic.rs` owns the complete diagnostic record.
//! - `diagnostics/codes.rs` owns diagnostic identifiers and categories.
//! - `diagnostics/ordering.rs` owns diagnostic ordering and deduplication.
//!
//! # OWNS
//!
//! - `LabelRole`, identifying primary and secondary labels.
//! - `DiagnosticLabel`, representing one source label.
//! - `LabelSet`, collecting labels with at most one primary label.
//! - Explicit errors when a second primary label is inserted.
//!
//! # DOES NOT OWN
//!
//! - Source text, source maps, or source-file registration.
//! - AST nodes, AST identities, or semantic facts.
//! - Diagnostic codes, severity, or complete diagnostic records.
//! - Line/column conversion, terminal rendering, or editor presentation.
//! - Diagnostic ordering, collection budgets, or recovery decisions.
//! - Type checking, domain validation, target feasibility, or IR generation.
//!
//! # INTEGRATION CONTRACT
//!
//! The canonical source span is:
//!
//! `crate::frontend::ast::node::source::Span`
//!
//! `DiagnosticLabel` stores that type directly. It does not create a second
//! span type or convert byte offsets into presentation coordinates.
//!
//! `DiagnosticLabels` is the recommended field type for a diagnostic's
//! primary and secondary labels. A diagnostic may also retain a primary span
//! separately for compatibility, but that span and the primary label must
//! not contradict one another.
//!
//! The caller is responsible for validating spans against the relevant
//! source map when source existence, source bounds, or UTF-8 boundaries must
//! be proven. This module has no source text with which to perform those
//! checks.
//!
//! # RANGE AND SOURCE CONTRACT
//!
//! Spans use the canonical half-open byte range `[start, end)`.
//! Zero-width spans are valid, including insertion locations and EOF.
//! Labels from different source files are permitted; for example, a diagnostic
//! may explain a declaration at one location and a use at another.
//!
//! # SCALABILITY
//!
//! There are no fixed label-count or source-file-count limits here.
//! Storage grows with the labels actually supplied and remains subject to
//! available memory and any explicit diagnostic budget enforced elsewhere.
//! This module imposes no language-level or hardware-capacity ceilings.
//!
//! # DETERMINISM
//!
//! Labels are retained in insertion order. Iteration always yields the
//! primary label first, when present, followed by secondary labels in their
//! insertion order. This module does not sort labels by source position.
//!
//! # ERROR HANDLING
//!
//! Inserting a second primary label fails explicitly and returns the rejected
//! label to the caller. It never silently replaces an existing primary label.
//! Replacing a primary label is available as a separate, explicitly named
//! operation.
//!
//! # FREEZE CRITERIA
//!
//! - Canonical AST spans are used directly.
//! - Primary and secondary labels have distinct roles.
//! - Duplicate primary insertion is reported without losing either label.
//! - Cross-source and zero-width spans are representable.
//! - Label iteration order is deterministic.
//! - Tests pass with the repository's supported Rust toolchain.
//! - No unsafe code or fixed capacity is introduced.

use core::fmt;

use crate::frontend::ast::node::source::Span;

/// The role of a source label within a diagnostic.
///
/// A primary label identifies the main source range associated with the
/// diagnostic. Secondary labels provide related context, such as a previous
/// declaration, an incompatible expression, or another relevant location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum LabelRole {
    /// The principal source range for a diagnostic.
    Primary,

    /// A related source range that supplies additional context.
    Secondary,
}

impl LabelRole {
    /// Returns a stable, machine-readable role name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
        }
    }
}

impl fmt::Display for LabelRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// One source label attached to a diagnostic.
///
/// A label contains a canonical source span, its role, and explanatory text.
/// It deliberately does not store a source path, line number, column number,
/// rendered snippet, or source contents.
///
/// The message is owned so that labels can outlive temporary analysis
/// expressions and be retained in diagnostic results.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiagnosticLabel {
    role: LabelRole,
    span: Span,
    message: String,
}

impl DiagnosticLabel {
    /// Creates a primary source label.
    ///
    /// This constructor does not check whether another primary label already
    /// exists in a diagnostic. Use [`DiagnosticLabels::insert`] to enforce
    /// that collection-level invariant.
    #[must_use]
    pub fn primary(span: Span, message: impl Into<String>) -> Self {
        Self {
            role: LabelRole::Primary,
            span,
            message: message.into(),
        }
    }

    /// Creates a secondary source label.
    #[must_use]
    pub fn secondary(span: Span, message: impl Into<String>) -> Self {
        Self {
            role: LabelRole::Secondary,
            span,
            message: message.into(),
        }
    }

    /// Returns the label's role.
    #[must_use]
    pub const fn role(&self) -> LabelRole {
        self.role
    }

    /// Returns the canonical source span.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Returns the explanatory label text.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the label text as an owned string.
    ///
    /// Prefer [`Self::message`] when a borrowed view is sufficient.
    #[must_use]
    pub fn into_message(self) -> String {
        self.message
    }

    /// Returns whether this label identifies the primary source range.
    #[must_use]
    pub const fn is_primary(&self) -> bool {
        matches!(self.role, LabelRole::Primary)
    }

    /// Returns whether this label supplies secondary context.
    #[must_use]
    pub const fn is_secondary(&self) -> bool {
        matches!(self.role, LabelRole::Secondary)
    }
}

/// Error returned when a label cannot be inserted into a [`DiagnosticLabels`].
///
/// The rejected label is retained in the error, so callers can report it,
/// correct the input, or explicitly replace the existing primary label
/// without losing the attempted insertion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelInsertError {
    rejected: DiagnosticLabel,
}

impl LabelInsertError {
    /// Returns the label that could not be inserted.
    #[must_use]
    pub fn rejected(&self) -> &DiagnosticLabel {
        &self.rejected
    }

    /// Returns ownership of the rejected label.
    #[must_use]
    pub fn into_rejected(self) -> DiagnosticLabel {
        self.rejected
    }
}

impl fmt::Display for LabelInsertError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.rejected.role() {
            LabelRole::Primary => formatter.write_str(
                "cannot insert a second primary diagnostic label",
            ),
            LabelRole::Secondary => formatter.write_str(
                "secondary diagnostic label insertion failed",
            ),
        }
    }
}

impl std::error::Error for LabelInsertError {}

/// Primary and secondary labels associated with one diagnostic.
///
/// Invariants:
///
/// - At most one primary label is stored.
/// - Secondary labels preserve insertion order.
/// - A secondary label may refer to a different source from the primary.
/// - Empty collections are valid for diagnostics without source locations.
///
/// This type does not validate that a span belongs to a registered source
/// or falls within the corresponding source text. Those checks require the
/// source map and belong to the caller or diagnostic-validation layer.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct DiagnosticLabels {
    primary: Option<DiagnosticLabel>,
    secondary: Vec<DiagnosticLabel>,
}

impl DiagnosticLabels {
    /// Creates an empty label collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            primary: None,
            secondary: Vec::new(),
        }
    }

    /// Creates a label collection with one primary label.
    #[must_use]
    pub fn with_primary(
        span: Span,
        message: impl Into<String>,
    ) -> Self {
        Self {
            primary: Some(DiagnosticLabel::primary(span, message)),
            secondary: Vec::new(),
        }
    }

    /// Inserts a label according to its declared role.
    ///
    /// Secondary labels are appended. A primary label is accepted only if
    /// no primary label has already been stored.
    ///
    /// # Errors
    ///
    /// Returns [`LabelInsertError`] when inserting a primary label would
    /// violate the single-primary invariant. The rejected label is preserved
    /// in the returned error.
    pub fn insert(
        &mut self,
        label: DiagnosticLabel,
    ) -> Result<(), LabelInsertError> {
        match label.role() {
            LabelRole::Primary => {
                if self.primary.is_some() {
                    return Err(LabelInsertError { rejected: label });
                }

                self.primary = Some(label);
            }
            LabelRole::Secondary => {
                self.secondary.push(label);
            }
        }

        Ok(())
    }

    /// Inserts a primary label when none exists.
    ///
    /// # Errors
    ///
    /// Returns the rejected label if a primary label is already present.
    pub fn add_primary(
        &mut self,
        span: Span,
        message: impl Into<String>,
    ) -> Result<(), LabelInsertError> {
        self.insert(DiagnosticLabel::primary(span, message))
    }

    /// Appends a secondary label.
    ///
    /// There is no fixed secondary-label count. Any operational retention
    /// budget must be enforced by the diagnostic collection layer.
    pub fn add_secondary(
        &mut self,
        span: Span,
        message: impl Into<String>,
    ) {
        self.secondary
            .push(DiagnosticLabel::secondary(span, message));
    }

    /// Returns the primary label, if one exists.
    #[must_use]
    pub fn primary(&self) -> Option<&DiagnosticLabel> {
        self.primary.as_ref()
    }

    /// Returns the secondary labels in insertion order.
    #[must_use]
    pub fn secondary(&self) -> &[DiagnosticLabel] {
        &self.secondary
    }

    /// Returns an iterator over all labels.
    ///
    /// The primary label, when present, is yielded first, followed by
    /// secondary labels in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = &DiagnosticLabel> {
        self.primary.iter().chain(self.secondary.iter())
    }

    /// Returns the total number of labels.
    #[must_use]
    pub fn len(&self) -> usize {
        self.secondary
            .len()
            .saturating_add(usize::from(self.primary.is_some()))
    }

    /// Returns whether the collection contains no labels.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.primary.is_none() && self.secondary.is_empty()
    }

    /// Returns whether a primary label exists.
    #[must_use]
    pub const fn has_primary(&self) -> bool {
        self.primary.is_some()
    }

    /// Returns the number of secondary labels.
    #[must_use]
    pub fn secondary_len(&self) -> usize {
        self.secondary.len()
    }

    /// Replaces the primary label explicitly.
    ///
    /// This operation is intentionally separate from [`Self::insert`],
    /// whose duplicate-primary behavior is to return an error.
    ///
    /// The previous primary label, if any, is returned to the caller.
    #[must_use]
    pub fn replace_primary(
        &mut self,
        span: Span,
        message: impl Into<String>,
    ) -> Option<DiagnosticLabel> {
        self.primary
            .replace(DiagnosticLabel::primary(span, message))
    }

    /// Removes the primary label and returns it, if present.
    pub fn take_primary(&mut self) -> Option<DiagnosticLabel> {
        self.primary.take()
    }

    /// Removes all secondary labels and returns them in insertion order.
    pub fn take_secondary(&mut self) -> Vec<DiagnosticLabel> {
        core::mem::take(&mut self.secondary)
    }

    /// Removes every label.
    pub fn clear(&mut self) {
        self.primary = None;
        self.secondary.clear();
    }

    /// Splits the collection into its primary and secondary components.
    ///
    /// Secondary labels retain their insertion order.
    #[must_use]
    pub fn into_parts(self) -> (Option<DiagnosticLabel>, Vec<DiagnosticLabel>) {
        (self.primary, self.secondary)
    }
}

impl IntoIterator for DiagnosticLabels {
    type Item = DiagnosticLabel;
    type IntoIter = DiagnosticLabelsIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        DiagnosticLabelsIntoIter {
            primary: self.primary.into_iter(),
            secondary: self.secondary.into_iter(),
        }
    }
}

/// Owning iterator for [`DiagnosticLabels`].
///
/// Iteration yields the primary label first, if present, then secondary
/// labels in their insertion order.
pub struct DiagnosticLabelsIntoIter {
    primary: std::option::IntoIter<DiagnosticLabel>,
    secondary: std::vec::IntoIter<DiagnosticLabel>,
}

impl Iterator for DiagnosticLabelsIntoIter {
    type Item = DiagnosticLabel;

    fn next(&mut self) -> Option<Self::Item> {
        self.primary.next().or_else(|| self.secondary.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self
            .primary
            .len()
            .saturating_add(self.secondary.len());

        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for DiagnosticLabelsIntoIter {}

impl std::iter::FusedIterator for DiagnosticLabelsIntoIter {}

impl<'a> IntoIterator for &'a DiagnosticLabels {
    type Item = &'a DiagnosticLabel;
    type IntoIter = std::iter::Chain<
        std::option::Iter<'a, DiagnosticLabel>,
        std::slice::Iter<'a, DiagnosticLabel>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.primary.iter().chain(self.secondary.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::source::{
        SourceId, SourceOffset, Span,
    };

    fn span(source: u64, start: u64, end: u64) -> Span {
        Span::new(
            SourceId::from_raw(source),
            SourceOffset::from_raw(start),
            SourceOffset::from_raw(end),
        )
        .expect("test span must have a non-reversed range")
    }

    #[test]
    fn primary_label_preserves_span_and_message() {
        let expected_span = span(1, 4, 9);
        let label = DiagnosticLabel::primary(expected_span, "invalid expression");

        assert_eq!(label.role(), LabelRole::Primary);
        assert!(label.is_primary());
        assert!(!label.is_secondary());
        assert_eq!(label.span(), expected_span);
        assert_eq!(label.message(), "invalid expression");
    }

    #[test]
    fn secondary_label_preserves_span_and_message() {
        let expected_span = span(2, 10, 15);
        let label = DiagnosticLabel::secondary(expected_span, "declared here");

        assert_eq!(label.role(), LabelRole::Secondary);
        assert!(!label.is_primary());
        assert!(label.is_secondary());
        assert_eq!(label.span(), expected_span);
        assert_eq!(label.message(), "declared here");
    }

    #[test]
    fn empty_collection_is_valid() {
        let labels = DiagnosticLabels::new();

        assert!(labels.is_empty());
        assert!(!labels.has_primary());
        assert_eq!(labels.len(), 0);
        assert_eq!(labels.secondary_len(), 0);
        assert!(labels.primary().is_none());
        assert!(labels.secondary().is_empty());
    }

    #[test]
    fn collection_can_be_constructed_with_primary_label() {
        let source_span = span(1, 2, 7);
        let labels = DiagnosticLabels::with_primary(source_span, "problem");

        assert!(!labels.is_empty());
        assert!(labels.has_primary());
        assert_eq!(labels.len(), 1);
        assert_eq!(
            labels.primary().map(DiagnosticLabel::span),
            Some(source_span)
        );
    }

    #[test]
    fn secondary_labels_preserve_insertion_order() {
        let mut labels = DiagnosticLabels::new();

        labels.add_secondary(span(1, 20, 22), "first");
        labels.add_secondary(span(1, 4, 6), "second");
        labels.add_secondary(span(3, 8, 9), "third");

        let messages: Vec<&str> = labels
            .secondary()
            .iter()
            .map(DiagnosticLabel::message)
            .collect();

        assert_eq!(messages, ["first", "second", "third"]);
    }

    #[test]
    fn iterator_yields_primary_then_secondary_labels() {
        let mut labels =
            DiagnosticLabels::with_primary(span(1, 5, 8), "primary");

        labels.add_secondary(span(1, 0, 2), "secondary one");
        labels.add_secondary(span(2, 1, 3), "secondary two");

        let messages: Vec<&str> =
            labels.iter().map(DiagnosticLabel::message).collect();

        assert_eq!(
            messages,
            ["primary", "secondary one", "secondary two"]
        );
    }

    #[test]
    fn inserting_a_second_primary_returns_the_rejected_label() {
        let mut labels =
            DiagnosticLabels::with_primary(span(1, 0, 1), "original");

        let rejected =
            DiagnosticLabel::primary(span(1, 2, 3), "attempted replacement");

        let error = labels
            .insert(rejected.clone())
            .expect_err("second primary must be rejected");

        assert_eq!(error.rejected(), &rejected);
        assert_eq!(
            labels.primary().map(DiagnosticLabel::message),
            Some("original")
        );
        assert_eq!(labels.len(), 1);
    }

    #[test]
    fn explicit_replacement_returns_previous_primary() {
        let mut labels =
            DiagnosticLabels::with_primary(span(1, 0, 1), "old");

        let previous = labels.replace_primary(span(1, 2, 4), "new");

        assert_eq!(
            previous.as_ref().map(DiagnosticLabel::message),
            Some("old")
        );
        assert_eq!(
            labels.primary().map(DiagnosticLabel::message),
            Some("new")
        );
    }

    #[test]
    fn cross_source_labels_are_supported() {
        let mut labels =
            DiagnosticLabels::with_primary(span(10, 1, 4), "use site");

        labels.add_secondary(span(20, 8, 12), "declaration site");

        assert_eq!(
            labels.primary().map(|label| label.span().source()),
            Some(SourceId::from_raw(10))
        );
        assert_eq!(
            labels.secondary().first().map(|label| label.span().source()),
            Some(SourceId::from_raw(20))
        );
    }

    #[test]
    fn zero_width_spans_are_supported() {
        let point = Span::point(
            SourceId::from_raw(1),
            SourceOffset::from_raw(12),
        );

        let label = DiagnosticLabel::primary(point, "insert here");

        assert!(label.span().is_empty());
        assert_eq!(label.span().start_raw(), 12);
        assert_eq!(label.span().end_raw(), 12);
    }

    #[test]
    fn empty_messages_are_preserved_without_rewriting() {
        let label = DiagnosticLabel::secondary(span(1, 0, 0), "");

        assert_eq!(label.message(), "");
    }

    #[test]
    fn owned_iteration_matches_borrowed_iteration() {
        let mut labels =
            DiagnosticLabels::with_primary(span(1, 0, 1), "p");

        labels.add_secondary(span(1, 1, 2), "s1");
        labels.add_secondary(span(2, 3, 5), "s2");

        let borrowed: Vec<String> = labels
            .iter()
            .map(|label| label.message().to_owned())
            .collect();

        let owned: Vec<String> = labels
            .clone()
            .into_iter()
            .map(|label| label.into_message())
            .collect();

        assert_eq!(borrowed, owned);
        assert_eq!(owned, ["p", "s1", "s2"]);
    }

    #[test]
    fn take_and_clear_operations_keep_collection_consistent() {
        let mut labels =
            DiagnosticLabels::with_primary(span(1, 0, 1), "primary");

        labels.add_secondary(span(1, 1, 2), "secondary");

        assert!(labels.take_primary().is_some());
        assert_eq!(labels.len(), 1);

        let secondary = labels.take_secondary();
        assert_eq!(secondary.len(), 1);
        assert!(labels.is_empty());

        labels.add_secondary(span(1, 2, 3), "another");
        labels.clear();

        assert!(labels.is_empty());
        assert_eq!(labels.len(), 0);
    }

    #[test]
    fn role_names_are_stable() {
        assert_eq!(LabelRole::Primary.as_str(), "primary");
        assert_eq!(LabelRole::Secondary.as_str(), "secondary");
        assert_eq!(LabelRole::Primary.to_string(), "primary");
        assert_eq!(LabelRole::Secondary.to_string(), "secondary");
    }

    #[test]
    fn insertion_error_preserves_rejected_label_and_has_message() {
        let rejected =
            DiagnosticLabel::primary(span(1, 3, 4), "second primary");

        let error = LabelInsertError {
            rejected: rejected.clone(),
        };

        assert_eq!(error.rejected(), &rejected);
        assert_eq!(error.into_rejected(), rejected);
        assert!(
            LabelInsertError {
                rejected: DiagnosticLabel::primary(
                    span(1, 0, 1),
                    "duplicate",
                ),
            }
            .to_string()
            .contains("second primary")
        );
    }
}
