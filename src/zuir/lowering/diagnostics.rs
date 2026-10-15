
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # ZUIR Lowering Diagnostics Adapter
//!
//! Path: `src/zuir/lowering/diagnostics.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Adapt lowering-local diagnostic notes and canonical ZUIR errors to the
//! repository-wide diagnostic representation.
//!
//! ## STATUS
//!
//! Proposed implementation. This file has not been compiled or validated
//! against the complete repository in this environment.
//!
//! ## AUTHORITY
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/execution-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `src/zuir/diagnostics.rs`
//! - `src/zuir/errors.rs`
//! - `src/zuir/source_map.rs`
//! - `src/zuir/lowering/context.rs`
//!
//! ## OWNS
//!
//! - Conversion of `LoweringNote` into canonical `Diagnostic`.
//! - Conversion of canonical lowering errors into diagnostics.
//! - Stable metadata keys for lowering-specific context.
//! - Deterministic collection of diagnostics from one lowering invocation.
//!
//! ## DOES_NOT_OWN
//!
//! - Canonical diagnostic codes, severity definitions, or rendering.
//! - Source identity allocation, source spans, or source-map validation.
//! - Lowering legality, target feasibility, or semantic preservation.
//! - Diagnostic retention limits or global compiler configuration.
//! - Parser, AST, semantic analysis, IR verification, or runtime behavior.
//!
//! ## INTEGRATION
//!
//! `context.rs` owns `LoweringNote` and `LoweringContext`.
//! `src/zuir/diagnostics.rs` owns `Diagnostic`, `DiagnosticPhase`,
//! `DiagnosticSeverity`, and `DiagnosticBuildError`.
//! `src/zuir/errors.rs` owns `ZuirError` and its stable error categories.
//! `src/zuir/source_map.rs` owns canonical `SourceSpan` values.
//!
//! Lowering `EntityKey` values are not interchangeable with canonical typed
//! ZUIR IDs. This adapter therefore never invents a `SourceSpan` from an
//! entity namespace or identity. A caller that has an authoritative mapping
//! may supply the resolved span explicitly.
//!
//! ## SCALABILITY
//!
//! This module defines no fixed diagnostic count, operation count, qubit
//! count, tensor rank, node count, or device count. Collection conversion
//! uses dynamically sized storage. Memory and execution remain subject to
//! actual process resources and any explicit upstream diagnostic policy.
//!
//! ## POCO-REAF
//!
//! Diagnostics describe lowering decisions without changing program
//! semantics. They do not claim that an unavailable target can execute a
//! program, and they do not authorize dropping unsupported semantics.
//!
//! ## FREEZE CRITERIA
//!
//! - The parent lowering module declares this file exactly once.
//! - Canonical diagnostic APIs and context APIs compile as used here.
//! - Codes, messages, entity metadata, and ordering are tested.
//! - Missing source locations remain absent rather than fabricated.
//! - Lowering errors retain canonical error codes and phase information.
//! - Formatting, linting, and repository tests pass.

use core::fmt;
use std::error::Error as StdError;

use crate::zuir::diagnostics::{
    Diagnostic, DiagnosticBuildError, DiagnosticPhase, DiagnosticSeverity,
};
use crate::zuir::errors::ZuirError;
use crate::zuir::source_map::SourceSpan;

use super::context::{
    EntityKey, EntityKind, LoweringContext, LoweringNote,
};

/// Canonical fallback code used for lowering-context notes.
///
/// The original lowering-local code is retained separately as structured
/// diagnostic data. This avoids assuming that every local code has already
/// been registered as a canonical ZUIR diagnostic code.
pub const LOWERING_NOTE_DIAGNOSTIC_CODE: &str = "ZMN-IR-LOWERING-NOTE";

/// Stable metadata key containing the original lowering-local diagnostic
/// code.
pub const DATA_LOWERING_CODE: &str = "lowering.code";

/// Stable metadata key containing the lowering entity category.
pub const DATA_ENTITY_KIND: &str = "lowering.entity.kind";

/// Stable metadata key containing the lowering entity namespace.
pub const DATA_ENTITY_NAMESPACE: &str = "lowering.entity.namespace";

/// Stable metadata key containing the opaque lowering entity identity.
pub const DATA_ENTITY_IDENTITY: &str = "lowering.entity.identity";

/// Stable metadata key containing the name of an extension entity category.
pub const DATA_ENTITY_EXTENSION: &str = "lowering.entity.extension";

/// Failure while adapting a lowering note into a canonical diagnostic.
///
/// This error represents an adapter or diagnostic-construction failure. It
/// does not imply that the source program or lowering decision is invalid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoweringDiagnosticError {
    source: DiagnosticBuildError,
}

impl LoweringDiagnosticError {
    /// Return the underlying canonical diagnostic construction error.
    #[must_use]
    pub const fn source_error(&self) -> DiagnosticBuildError {
        self.source
    }
}

impl fmt::Display for LoweringDiagnosticError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "failed to construct canonical lowering diagnostic: {}",
            self.source
        )
    }
}

impl StdError for LoweringDiagnosticError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.source)
    }
}

impl From<DiagnosticBuildError> for LoweringDiagnosticError {
    fn from(source: DiagnosticBuildError) -> Self {
        Self { source }
    }
}

/// Convert a lowering-local note into the canonical ZUIR diagnostic type.
///
/// The note's original code is stored in [`DATA_LOWERING_CODE`]. The
/// canonical outer code identifies the adapter-level diagnostic category;
/// it does not falsely claim that a lowering-local code has been registered
/// in the repository-wide diagnostic registry.
///
/// Since `LoweringNote` does not carry severity, this conversion uses
/// `DiagnosticSeverity::Note`. Callers must report actual failures using
/// canonical `ZuirError` values instead of inferring severity from a message.
pub fn lowering_note_to_diagnostic(
    note: &LoweringNote,
) -> Result<Diagnostic, LoweringDiagnosticError> {
    lowering_note_to_diagnostic_with_span(note, None)
}

/// Convert a lowering-local note using an optional, already-resolved span.
///
/// `primary_span` must originate from the canonical source-map contract or
/// another authoritative span-producing subsystem. It must not be
/// synthesized from `EntityKey::namespace()` or `EntityKey::value()`.
///
/// The caller is responsible for validating the final diagnostic against
/// the relevant `SourceMap` when source-map validation is required.
pub fn lowering_note_to_diagnostic_with_span(
    note: &LoweringNote,
    primary_span: Option<SourceSpan>,
) -> Result<Diagnostic, LoweringDiagnosticError> {
    let mut diagnostic = Diagnostic::new(
        LOWERING_NOTE_DIAGNOSTIC_CODE,
        DiagnosticSeverity::Note,
        DiagnosticPhase::Lowering,
        note.message(),
        primary_span,
    )?;

    diagnostic = diagnostic.with_data(DATA_LOWERING_CODE, note.code())?;

    if let Some(subject) = note.subject() {
        add_entity_metadata(&mut diagnostic, subject)?;
    }

    Ok(diagnostic)
}

/// Convert a canonical ZUIR error into the canonical diagnostic type.
///
/// The canonical error subsystem remains authoritative for the stable error
/// code, error category, and message. An optional span may be supplied only
/// when the caller has resolved the relevant source location.
#[must_use]
pub fn lowering_error_to_diagnostic(
    error: &ZuirError,
    primary_span: Option<SourceSpan>,
) -> Diagnostic {
    Diagnostic::from_zui_error(error, DiagnosticPhase::Lowering, primary_span)
}

/// Convert all notes recorded in one lowering context.
///
/// The context already stores notes in a deterministic ordered collection.
/// The resulting diagnostics are also sorted and deduplicated to make this
/// guarantee explicit at the adapter boundary.
///
/// This function does not clear or mutate the context. If one note cannot be
/// converted, the function returns an error instead of silently dropping it.
pub fn lowering_context_diagnostics(
    context: &LoweringContext,
) -> Result<Vec<Diagnostic>, LoweringDiagnosticError> {
    let mut diagnostics = context
        .diagnostics()
        .map(lowering_note_to_diagnostic)
        .collect::<Result<Vec<_>, _>>()?;

    diagnostics.sort();
    diagnostics.dedup();

    Ok(diagnostics)
}

/// Add stable metadata describing a lowering entity to a diagnostic.
///
/// This helper deliberately stores the entity identity as opaque metadata.
/// It does not attempt to parse the identity into a typed ZUIR ID.
fn add_entity_metadata(
    diagnostic: &mut Diagnostic,
    entity: &EntityKey,
) -> Result<(), LoweringDiagnosticError> {
    let (kind_name, extension_name) = entity_kind_metadata(entity.kind());

    *diagnostic = diagnostic
        .clone()
        .with_data(DATA_ENTITY_KIND, kind_name)?
        .with_data(DATA_ENTITY_NAMESPACE, entity.namespace())?
        .with_data(DATA_ENTITY_IDENTITY, entity.value())?;

    if let Some(extension_name) = extension_name {
        *diagnostic = diagnostic
            .clone()
            .with_data(DATA_ENTITY_EXTENSION, extension_name)?;
    }

    Ok(())
}

/// Return stable metadata for an entity category.
///
/// The category strings are diagnostic metadata, not serialized entity IDs
/// and not a substitute for the canonical `ErrorSubject` type.
fn entity_kind_metadata(kind: &EntityKind) -> (&'static str, Option<&str>) {
    match kind {
        EntityKind::Program => ("program", None),
        EntityKind::Module => ("module", None),
        EntityKind::Function => ("function", None),
        EntityKind::Region => ("region", None),
        EntityKind::Block => ("block", None),
        EntityKind::Operation => ("operation", None),
        EntityKind::Value => ("value", None),
        EntityKind::Type => ("type", None),
        EntityKind::Symbol => ("symbol", None),
        EntityKind::Dialect => ("dialect", None),
        EntityKind::Source => ("source", None),
        EntityKind::Extension(name) => ("extension", Some(name.as_str())),
        _ => ("unknown", None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::errors::{ZuirErrorKind};
    use crate::zuir::lowering::context::{
        EntityKey, EntityKind, LoweringContext, LoweringNote, LoweringPolicy,
    };

    fn sample_note() -> LoweringNote {
        let entity = EntityKey::new(
            EntityKind::Operation,
            "test.source",
            "operation-17",
        )
        .expect("valid entity key");

        LoweringNote::new(
            "LOWERING-UNSUPPORTED-SEMANTICS",
            "The destination cannot preserve this operation's semantics.",
            Some(entity),
        )
        .expect("valid lowering note")
    }

    #[test]
    fn converts_note_using_canonical_diagnostic_type() {
        let diagnostic =
            lowering_note_to_diagnostic(&sample_note()).expect("conversion succeeds");

        assert_eq!(diagnostic.code().as_str(), LOWERING_NOTE_DIAGNOSTIC_CODE);
        assert_eq!(diagnostic.severity(), DiagnosticSeverity::Note);
        assert_eq!(diagnostic.phase(), &DiagnosticPhase::Lowering);
        assert_eq!(
            diagnostic.message(),
            "The destination cannot preserve this operation's semantics."
        );
        assert_eq!(
            diagnostic.data().get(DATA_LOWERING_CODE).map(String::as_str),
            Some("LOWERING-UNSUPPORTED-SEMANTICS")
        );
    }

    #[test]
    fn preserves_entity_identity_as_structured_metadata() {
        let diagnostic =
            lowering_note_to_diagnostic(&sample_note()).expect("conversion succeeds");

        assert_eq!(
            diagnostic.data().get(DATA_ENTITY_KIND).map(String::as_str),
            Some("operation")
        );
        assert_eq!(
            diagnostic
                .data()
                .get(DATA_ENTITY_NAMESPACE)
                .map(String::as_str),
            Some("test.source")
        );
        assert_eq!(
            diagnostic
                .data()
                .get(DATA_ENTITY_IDENTITY)
                .map(String::as_str),
            Some("operation-17")
        );
    }

    #[test]
    fn does_not_invent_a_source_span() {
        let diagnostic =
            lowering_note_to_diagnostic(&sample_note()).expect("conversion succeeds");

        assert!(diagnostic.primary_span().is_none());
    }

    #[test]
    fn converts_canonical_lowering_errors_without_replacing_their_code() {
        let error = ZuirError::new(
            ZuirErrorKind::LoweringUnsupported,
            "The requested conversion is not supported.",
        );

        let diagnostic = lowering_error_to_diagnostic(&error, None);

        assert_eq!(diagnostic.code().as_str(), error.code().as_str());
        assert_eq!(diagnostic.phase(), &DiagnosticPhase::Lowering);
        assert_eq!(diagnostic.message(), error.message());
    }

    #[test]
    fn converts_an_empty_context_to_an_empty_collection() {
        let context = LoweringContext::new(LoweringPolicy::default());

        let diagnostics =
            lowering_context_diagnostics(&context).expect("conversion succeeds");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn context_conversion_is_deterministic() {
        let mut context = LoweringContext::new(LoweringPolicy::default());

        let first = sample_note();

        let second_entity = EntityKey::new(
            EntityKind::Value,
            "test.source",
            "value-9",
        )
        .expect("valid entity key");

        let second = LoweringNote::new(
            "LOWERING-CONDITIONAL",
            "The destination requires additional checks.",
            Some(second_entity),
        )
        .expect("valid lowering note");

        context.add_diagnostic(first);
        context.add_diagnostic(second);

        let a = lowering_context_diagnostics(&context).expect("first conversion");
        let b = lowering_context_diagnostics(&context).expect("second conversion");

        assert_eq!(a, b);
    }

    #[test]
    fn extension_entity_categories_remain_descriptive() {
        let entity = EntityKey::new(
            EntityKind::Extension("future.entity".to_owned()),
            "vendor.domain",
            "entity-1",
        )
        .expect("valid extension entity");

        let note = LoweringNote::new(
            "LOWERING-EXTENSION-NOTE",
            "An extension entity was encountered.",
            Some(entity),
        )
        .expect("valid lowering note");

        let diagnostic =
            lowering_note_to_diagnostic(&note).expect("conversion succeeds");

        assert_eq!(
            diagnostic
                .data()
                .get(DATA_ENTITY_KIND)
                .map(String::as_str),
            Some("extension")
        );
        assert_eq!(
            diagnostic
                .data()
                .get(DATA_ENTITY_EXTENSION)
                .map(String::as_str),
            Some("future.entity")
        );
    }
}
