#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani semantic diagnostics
//!
//! File: `src/frontend/semantic/diagnostics/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//!
//! ## PURPOSE
//!
//! This module is the public orchestration boundary for semantic diagnostics.
//! It brings together:
//!
//! - `diagnostic`: structured diagnostic records and their presentation;
//! - `codes`: stable built-in diagnostic identifiers and registry metadata;
//! - `labels`: structured primary and secondary source-label collections;
//! - `ordering`: deterministic comparison, sorting, and deduplication.
//!
//! It provides one explicit public API without requiring callers to import
//! implementation modules individually.
//!
//! ## AUTHORITY
//!
//! Normative diagnostic contract:
//! `grammar/spec/diagnostics.md`
//!
//! Normative semantic contract:
//! `grammar/spec/semantics.md`
//!
//! Canonical syntax composition:
//! `grammar/Zamani.g4`
//!
//! Canonical source locations:
//! `crate::frontend::ast::node::source::span`
//!
//! These specifications govern diagnostic behavior. This module does not
//! define language syntax, semantic validity, or target feasibility.
//!
//! ## OWNS
//!
//! - Diagnostic module composition and public exports.
//! - Unambiguous names for overlapping diagnostic types.
//! - Explicit conversions between the code registry's classifications and
//!   the structured diagnostic record's classifications.
//! - The public entry point for ordering and deduplication.
//!
//! ## DOES NOT OWN
//!
//! - Diagnostic record storage or rendering algorithms.
//! - The built-in diagnostic-code catalogue.
//! - Source maps, source text, or source-span construction.
//! - Lexical or parser diagnostic implementations.
//! - Semantic rules, type checking, or domain-specific validation.
//! - Diagnostic accumulation budgets or analysis-session state.
//! - Target selection, resource allocation, scheduling, or IR generation.
//!
//! ## INTEGRATION CONTRACT
//!
//! `diagnostic.rs` remains the compatibility-facing record implementation.
//! `codes.rs` remains the registry-facing code catalogue.
//! `labels.rs` remains the richer role-aware label collection.
//! `ordering.rs` remains the canonical ordering and deduplication algorithm.
//!
//! These files must not introduce competing implementations of sorting,
//! report identity, source coordinates, or diagnostic collection.
//!
//! Because the repository currently contains overlapping diagnostic types,
//! this module exports the record-facing types under their direct names and
//! gives the registry and role-aware label types explicit aliases.
//!
//! Conversions are provided where the concepts have compatible meanings.
//! Conversion does not make an unknown code registered, prove a diagnostic
//! correct, or validate a source span.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This module introduces no hardware-capacity limits, computational-domain
//! limits, fixed diagnostic-count ceiling, or source-size ceiling.
//!
//! Actual memory and processing limits are determined by available resources
//! and by explicit operational budgets enforced by the owning analysis
//! session. Budget exhaustion must be represented as incomplete analysis;
//! it must not be interpreted as successful semantic validation.
//!
//! Open-world diagnostic codes remain possible through the registry API.
//! Adding a computational domain must not require a new diagnostics module
//! or a change to the diagnostic orchestration boundary.
//!
//! ## DETERMINISM
//!
//! All public ordering functions delegate to `ordering.rs`.
//! Ordered metadata and stable diagnostic identities are preserved.
//! This module does not introduce global state, runtime-dependent ordering,
//! or hash-map iteration into diagnostic reporting.
//!
//! ## SAFETY AND ERROR HANDLING
//!
//! Safe Rust only. No unsafe code is permitted.
//! Conversions are explicit and non-panicking.
//! Invalid code registration remains an error handled by `codes.rs`.
//! Diagnostic identity and ordering remain separate concepts.
//!
//! ## FREEZE CRITERIA
//!
//! - Every diagnostic implementation module is declared exactly once.
//! - Public exports are explicit and free from duplicate-name ambiguity.
//! - Registry codes can be used to construct structured diagnostics.
//! - Registry categories and severities convert deterministically.
//! - Ordering functions have one canonical implementation.
//! - Unit tests pass with the repository's supported toolchain.
//! - The frontend and workspace integration builds successfully.

// Keep modules public so advanced consumers can access module-specific APIs.
// Prefer the explicit re-exports below for normal use.
pub mod codes;
pub mod diagnostic;
pub mod labels;
pub mod ordering;

// ---------------------------------------------------------------------------
// Canonical structured diagnostic record API
// ---------------------------------------------------------------------------

pub use diagnostic::{
    Diagnostic,
    DiagnosticCategory,
    DiagnosticCode,
    DiagnosticLabel,
    DiagnosticPhase,
    DiagnosticSeverity,
    DiagnosticValue,
    FixIt,
    FixItEdit,
    RecoveryClassification,
};

// ---------------------------------------------------------------------------
// Registry API
//
// These aliases are intentional. `codes.rs` currently owns types with names
// that overlap types in `diagnostic.rs`; exposing both under the same names
// would make the public API ambiguous.
// ---------------------------------------------------------------------------

pub use codes::{
    BuiltinDiagnosticMetadata,
    CategoryError,
    DiagnosticCategory as RegisteredDiagnosticCategory,
    DiagnosticCode as RegisteredDiagnosticCode,
    DiagnosticCodeError,
    DiagnosticSeverity as RegisteredDiagnosticSeverity,
    BUILTIN_DIAGNOSTICS,
};

// ---------------------------------------------------------------------------
// Role-aware source-label API
//
// `diagnostic.rs` has a compatibility label type that stores a span and
// message. `labels.rs` has a richer role-aware label and collection. Keep
// both available without conflating their invariants.
// ---------------------------------------------------------------------------

pub use labels::{
    DiagnosticLabel as SourceDiagnosticLabel,
    DiagnosticLabels,
    LabelInsertError,
    LabelRole,
};

// ---------------------------------------------------------------------------
// Canonical reporting API
//
// Do not implement a second comparator or deduplicator here.
// ---------------------------------------------------------------------------

pub use ordering::{
    compare_diagnostics,
    deduplicate_diagnostics,
    sort_diagnostics,
};

// ---------------------------------------------------------------------------
// Explicit conversions from registry concepts to record concepts
// ---------------------------------------------------------------------------

impl From<RegisteredDiagnosticCode> for DiagnosticCode {
    fn from(code: RegisteredDiagnosticCode) -> Self {
        Self::new(code.as_str())
    }
}

impl From<RegisteredDiagnosticSeverity> for DiagnosticSeverity {
    fn from(severity: RegisteredDiagnosticSeverity) -> Self {
        match severity {
            RegisteredDiagnosticSeverity::Note => Self::Note,
            RegisteredDiagnosticSeverity::Warning => Self::Warning,
            RegisteredDiagnosticSeverity::Error => Self::Error,
            RegisteredDiagnosticSeverity::Fatal => Self::Fatal,
            _ => Self::Error,
        }
    }
}

impl From<RegisteredDiagnosticCategory> for DiagnosticCategory {
    fn from(category: RegisteredDiagnosticCategory) -> Self {
        match category {
            RegisteredDiagnosticCategory::Structural => Self::Syntax,
            RegisteredDiagnosticCategory::NameResolution => Self::NameResolution,
            RegisteredDiagnosticCategory::Type => Self::Type,
            RegisteredDiagnosticCategory::Ownership => Self::Ownership,
            RegisteredDiagnosticCategory::Effect => Self::Effect,
            RegisteredDiagnosticCategory::Capability => Self::Capability,
            RegisteredDiagnosticCategory::Resource => Self::Resource,
            RegisteredDiagnosticCategory::Contract => Self::Contract,
            RegisteredDiagnosticCategory::Policy => {
                Self::Extension(String::from("policy"))
            }
            RegisteredDiagnosticCategory::Provenance => {
                Self::Extension(String::from("provenance"))
            }
            RegisteredDiagnosticCategory::Domain => Self::Domain,
            RegisteredDiagnosticCategory::Extension => {
                Self::Extension(String::from("extension"))
            }
            RegisteredDiagnosticCategory::Evaluation => {
                Self::Extension(String::from("evaluation"))
            }
            RegisteredDiagnosticCategory::Analysis => Self::OperationalLimit,
            RegisteredDiagnosticCategory::Internal => Self::Internal,
            RegisteredDiagnosticCategory::ExtensionCategory(name) => {
                Self::Extension(name)
            }
            _ => Self::Extension(String::from("unclassified")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_code_converts_into_record_code() {
        let registered = RegisteredDiagnosticCode::TYPE_MISMATCH;
        let record_code: DiagnosticCode = registered.clone().into();

        assert_eq!(record_code.as_str(), registered.as_str());
    }

    #[test]
    fn registry_severities_convert_without_losing_error_status() {
        assert_eq!(
            DiagnosticSeverity::from(RegisteredDiagnosticSeverity::Note),
            DiagnosticSeverity::Note,
        );
        assert_eq!(
            DiagnosticSeverity::from(RegisteredDiagnosticSeverity::Warning),
            DiagnosticSeverity::Warning,
        );
        assert_eq!(
            DiagnosticSeverity::from(RegisteredDiagnosticSeverity::Error),
            DiagnosticSeverity::Error,
        );
        assert_eq!(
            DiagnosticSeverity::from(RegisteredDiagnosticSeverity::Fatal),
            DiagnosticSeverity::Fatal,
        );
    }

    #[test]
    fn registry_categories_convert_deterministically() {
        assert_eq!(
            DiagnosticCategory::from(RegisteredDiagnosticCategory::Type),
            DiagnosticCategory::Type,
        );

        assert_eq!(
            DiagnosticCategory::from(
                RegisteredDiagnosticCategory::ExtensionCategory(
                    String::from("example-domain"),
                ),
            ),
            DiagnosticCategory::Extension(String::from("example-domain")),
        );
    }

    #[test]
    fn registry_values_can_construct_a_structured_diagnostic() {
        let diagnostic = Diagnostic::new(
            RegisteredDiagnosticCode::TYPE_MISMATCH,
            DiagnosticPhase::TypeAnalysis,
            RegisteredDiagnosticCategory::Type,
            RegisteredDiagnosticSeverity::Error,
            "incompatible types",
            None,
        );

        assert_eq!(diagnostic.code().as_str(), "ZM-SEM-TYPE-001");
        assert_eq!(diagnostic.category(), &DiagnosticCategory::Type);
        assert_eq!(diagnostic.severity(), DiagnosticSeverity::Error);
        assert!(diagnostic.is_error());
    }
}