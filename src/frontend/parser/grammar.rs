
#![forbid(unsafe_code)]

//! Shared grammar facade and deterministic extension dispatcher for Zamani.
//!
//! # Purpose
//!
//! Define the common boundary between core grammar productions and registered
//! grammar extensions. This module provides extension dispatch primitives;
//! concrete productions belong in `grammar/` and `domains/`.
//!
//! # Owns
//!
//! - Dispatch at declared grammar extension points.
//! - Extension probe ordering and ambiguity detection.
//! - Dialect and parser-API compatibility checks during dispatch.
//! - Progress checks around extension parsing.
//! - Structured extension-dispatch outcomes.
//!
//! # Does not own
//!
//! - Lexical scanning or token definitions.
//! - Canonical AST definitions or AST construction policy.
//! - Registration, registry storage, or extension loading.
//! - Individual classical, quantum, HDL, or other domain productions.
//! - Name resolution, type checking, effects, capabilities, or contracts.
//! - Hardware discovery, resource negotiation, optimization, or lowering.
//!
//! # Integration contract
//!
//! - `extensions.rs` owns `GrammarExtension`, `ExtensionPoint`,
//!   `ExtensionParseContext`, and extension error types.
//! - `registry.rs` owns extension registration and immutable registry storage.
//! - `session.rs` supplies a budget-aware implementation of
//!   `ExtensionParseContext`.
//! - `grammar/` owns shared syntax productions.
//! - `domains/` owns domain-specific syntax adapters.
//! - `parser.rs` orchestrates complete parse operations.
//! - `ast_adapter.rs` converts intermediate syntax to canonical AST nodes.
//!
//! The dispatcher accepts an already-validated extension collection. It does
//! not depend on the current registry representation, because that registry's
//! API must first be reconciled with `extensions.rs`.
//!
//! # Determinism and safety
//!
//! Probe results must be side-effect-free and deterministic. Conflicting
//! matching extensions are rejected; equal-priority matches are ambiguous.
//! Successful extension parsing must consume input. The caller supplies the
//! lookahead window and active dialect set; this module imposes no fixed
//! extension count, hardware-capacity ceiling, or language-scale limit.
//!
//! Safe Rust only. No global mutable state.

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::frontend::lexer::token_kind::TokenKind;

use super::extensions::{
    ExtensionError,
    ExtensionErrorKind,
    ExtensionPoint,
    GrammarExtension,
    ExtensionParseContext,
};

/// Dispatch an extension at a declared grammar point.
///
/// # Arguments
///
/// - `extensions`: immutable extension handlers from a validated registry.
/// - `point`: the grammar position being parsed.
/// - `context`: the active session's budget-aware extension context.
/// - `lookahead`: a caller-provided lookahead window, starting at the current
///   token. The caller chooses its size according to parser configuration.
/// - `active_dialects`: dialect identifiers enabled for this parse.
/// - `parser_api`: the parser API major and minor versions.
///
/// # Returns
///
/// - `Ok(Some(node))` when exactly one eligible extension parses successfully.
/// - `Ok(None)` when no eligible extension recognizes this grammar position.
/// - `Err(error)` for incompatible extensions, conflicting claims, ambiguity,
///   extension parse failures, or a handler that reports success without
///   consuming input.
///
/// # Dispatch policy
///
/// Higher priority is considered first. Priority does not override an explicit
/// extension conflict. Two matching handlers with equal priority are rejected
/// rather than resolved using registration order.
///
/// Registry construction should reject invalid descriptors before this
/// function is called. Runtime checks remain as defense in depth.
pub fn dispatch_extension<N>(
    extensions: &[Arc<dyn GrammarExtension<N>>],
    point: ExtensionPoint,
    context: &mut dyn ExtensionParseContext,
    lookahead: &[TokenKind],
    active_dialects: &BTreeSet<String>,
    parser_api: (u64, u64),
) -> Result<Option<N>, ExtensionError> {
    let mut matches: Vec<&dyn GrammarExtension<N>> = Vec::new();

    for extension in extensions {
        let descriptor = extension.descriptor();

        if !descriptor.supports_point(point) {
            continue;
        }

        // Extensions requiring a disabled dialect are not candidates.
        if !descriptor
            .required_dialects()
            .iter()
            .all(|dialect| active_dialects.contains(dialect))
        {
            continue;
        }

        // An enabled extension cannot silently execute against an incompatible
        // parser API. Catch this here even if registry validation should have
        // rejected it earlier.
        if !descriptor.supports_parser_api(parser_api.0, parser_api.1) {
            return Err(
                ExtensionError::new(
                    ExtensionErrorKind::IncompatibleParserApi,
                    format!(
                        "extension {} does not support parser API {}.{}",
                        descriptor.id(),
                        parser_api.0,
                        parser_api.1
                    ),
                )
                .with_extension(descriptor.id().clone()),
            );
        }

        match extension.probe(point, lookahead) {
            super::extensions::ExtensionMatch::NoMatch => {}
            super::extensions::ExtensionMatch::Match => matches.push(extension.as_ref()),
        }
    }

    if matches.is_empty() {
        return Ok(None);
    }

    // Stable ordering makes diagnostics and dispatch independent of registry
    // insertion order. The ID tie-breaker is for deterministic inspection;
    // equal-priority claims are still rejected below.
    matches.sort_by(|left, right| {
        right
            .descriptor()
            .priority()
            .cmp(&left.descriptor().priority())
            .then_with(|| left.descriptor().id().cmp(right.descriptor().id()))
    });

    let winner_descriptor = matches[0].descriptor();

    // An explicit conflict invalidates the set of matching candidates even
    // when priorities differ. Priority is not permission to override a
    // declared incompatibility.
    for candidate in matches.iter().skip(1) {
        let candidate_descriptor = candidate.descriptor();

        if winner_descriptor.conflicts_with(candidate_descriptor.id())
            || candidate_descriptor.conflicts_with(winner_descriptor.id())
        {
            return Err(
                ExtensionError::new(
                    ExtensionErrorKind::ConflictingExtensions,
                    format!(
                        "extensions {} and {} both claim {:?} and declare a conflict",
                        winner_descriptor.id(),
                        candidate_descriptor.id(),
                        point
                    ),
                )
                .with_extension(winner_descriptor.id().clone()),
            );
        }
    }

    // Equal-priority matching handlers have no deterministic semantic winner.
    // Do not use registration order or identifier ordering to hide ambiguity.
    let winner_priority = winner_descriptor.priority();

    if matches
        .iter()
        .skip(1)
        .any(|candidate| candidate.descriptor().priority() == winner_priority)
    {
        let competing_ids = matches
            .iter()
            .take_while(|candidate| {
                candidate.descriptor().priority() == winner_priority
            })
            .map(|candidate| candidate.descriptor().id().to_string())
            .collect::<Vec<_>>()
            .join(", ");

        return Err(
            ExtensionError::new(
                ExtensionErrorKind::AmbiguousMatch,
                format!(
                    "multiple extensions with priority {winner_priority} claim \
                     grammar point {point:?}: {competing_ids}"
                ),
            )
            .with_extension(winner_descriptor.id().clone()),
        );
    }

    let start_position = context.position();
    let node = matches[0].parse(point, context)?;
    let end_position = context.position();

    if end_position <= start_position {
        return Err(
            ExtensionError::new(
                ExtensionErrorKind::NoProgress,
                format!(
                    "extension {} reported success at {:?} without consuming input",
                    winner_descriptor.id(),
                    point
                ),
            )
            .with_extension(winner_descriptor.id().clone()),
        );
    }

    Ok(Some(node))
}

/// Shared production-dispatch result used by grammar facades.
///
/// `NotHandled` means the production did not recognize the current syntax;
/// it is not a syntax error. `Handled` means a production successfully
/// produced a node. Errors should use the parser's canonical error and
/// diagnostic contracts rather than being converted to `NotHandled`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProductionOutcome<T> {
    NotHandled,
    Handled(T),
}

impl<T> ProductionOutcome<T> {
    /// Return `true` when a production handled the syntax.
    #[must_use]
    pub const fn is_handled(&self) -> bool {
        matches!(self, Self::Handled(_))
    }

    /// Transform a successfully handled node while preserving `NotHandled`.
    pub fn map<U>(self, map: impl FnOnce(T) -> U) -> ProductionOutcome<U> {
        match self {
            Self::NotHandled => ProductionOutcome::NotHandled,
            Self::Handled(value) => ProductionOutcome::Handled(map(value)),
        }
    }

    /// Consume the outcome and return the node, if handled.
    #[must_use]
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::NotHandled => None,
            Self::Handled(value) => Some(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProductionOutcome;

    #[test]
    fn not_handled_is_not_a_successful_parse() {
        let outcome: ProductionOutcome<u32> = ProductionOutcome::NotHandled;

        assert!(!outcome.is_handled());
        assert_eq!(outcome.into_option(), None);
    }

    #[test]
    fn handled_production_preserves_its_value() {
        let outcome = ProductionOutcome::Handled(21_u32);

        assert!(outcome.is_handled());
        assert_eq!(outcome.map(|value| value * 2), ProductionOutcome::Handled(42));
    }

    #[test]
    fn mapping_does_not_turn_not_handled_into_handled() {
        let outcome: ProductionOutcome<u32> = ProductionOutcome::NotHandled;

        assert_eq!(
            outcome.map(|value| value.to_string()),
            ProductionOutcome::<String>::NotHandled
        );
    }
}
