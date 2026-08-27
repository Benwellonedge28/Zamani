//! Zamani Quantum Benchmarking — Reporting Boundary
//!
//! Production-grade module boundary for all quantum-benchmark reporting.
//!
//! # Architectural role
//!
//! This module is the presentation and serialization boundary of the Zamani
//! quantum-benchmarking subsystem.
//!
//! The reporting layer consumes already-computed, validated benchmark data and
//! converts it into:
//!
//! - canonical JSON;
//! - CSV;
//! - Markdown;
//! - deterministic human-readable summaries;
//! - structured tables;
//! - complete benchmark reports.
//!
//! It does NOT:
//!
//! - execute quantum circuits;
//! - generate circuits;
//! - compile or route circuits;
//! - select hardware;
//! - access hardware;
//! - calculate benchmark metrics;
//! - perform statistical inference;
//! - perform benchmark protocol logic;
//! - modify benchmark results;
//! - own benchmark state;
//! - maintain global mutable state;
//! - silently discard warnings/errors;
//! - silently reinterpret measurements;
//! - make scientific claims not already represented by the result model.
//!
//! # Canonical dependency direction
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! benchmark workload / experiment
//!      │
//!      ▼
//! execution
//!      │
//!      ▼
//! observations
//!      │
//!      ▼
//! statistics
//!      │
//!      ▼
//! metrics
//!      │
//!      ▼
//! core::BenchmarkResult
//!      │
//!      ▼
//! reporting
//!   ┌──┼───────────┬──────────┬──────────┐
//!   ▼  ▼           ▼          ▼          ▼
//! JSON CSV     Markdown     Table     Summary
//!   │
//!   ▼
//! external storage / CI / tooling / Zamani stdlib
//! ```
//!
//! Reporting must therefore depend on the core result model, never the other
//! way around.
//!
//! # Public API policy
//!
//! The module exposes two levels of API:
//!
//! 1. **module APIs**, for consumers that need format-specific functionality;
//! 2. **stable reporting aliases**, for common reporting types that should be
//!    discoverable from `quantum::benchmarking::reporting`.
//!
//! Format-specific implementation details remain owned by their respective
//! modules.
//!
//! # Canonical interchange format
//!
//! JSON is the canonical lossless interchange representation of a
//! `BenchmarkResult`.
//!
//! CSV and Markdown are presentation/export formats and must not become
//! alternative authoritative representations of benchmark truth.
//!
//! In particular:
//!
//! ```text
//! BenchmarkResult
//!       │
//!       ├──► JSON      lossless/canonical
//!       │
//!       └──► Summary
//!               ├──► CSV
//!               └──► Markdown
//! ```
//!
//! # Trust boundary
//!
//! Serialized benchmark data must be treated as untrusted until it has passed
//! the validation contract of `core::BenchmarkResult`.
//!
//! Reporting must never turn malformed serialized data into a trusted
//! scientific result.
//!
//! # Determinism
//!
//! Reporting is required to be deterministic for the same validated input and
//! the same explicitly supplied reporting configuration.
//!
//! It must not depend on:
//!
//! - wall-clock time;
//! - process ID;
//! - thread scheduling;
//! - hash-map iteration order;
//! - host-specific locale;
//! - environment variables;
//! - global mutable state.
//!
//! Any timestamp included in a report must originate from the benchmark
//! provenance/result itself, not be generated merely because a report is being
//! rendered.
//!
//! # Bounded output
//!
//! The reporting layer must honor the bounded result/report contracts defined
//! by the benchmarking core. A caller must not be able to create an
//! unbounded-memory reporting operation simply by supplying malformed or
//! adversarial benchmark data.
//!
//! Format modules are responsible for their format-specific limits; this
//! module is responsible for keeping the module boundary explicit and avoiding
//! accidental cross-format recursion.
//!
//! # Rust compatibility
//!
//! Target Rust: 1.97 / 1.97.1.
//! Edition: Rust 2021.
//! No nightly features.
//! No unsafe code.
//!
//! # Integration contract
//!
//! This module integrates with:
//!
//! - `super::core` for canonical benchmark result types;
//! - `super::analysis` indirectly through already-computed result data;
//! - `super::validation` indirectly through validated result data;
//! - `super::registry` indirectly through benchmark metadata;
//! - `super::protocols` indirectly through protocol-produced results;
//! - `super::execution` indirectly through completed observations;
//! - `super::stdlib` only through the public benchmarking boundary;
//! - external CI/storage/tooling through JSON/CSV/Markdown output.
//!
//! It must NOT create reverse dependencies from any of those layers into
//! reporting merely to obtain a formatting type.
//!
//! # Module ownership
//!
//! ```text
//! reporting/
//! │
//! ├── mod.rs       ← this file; module/public API boundary
//! ├── report.rs    ← complete report/orchestration model
//! ├── summary.rs   ← deterministic presentation summary
//! ├── table.rs     ← structured tabular representation
//! ├── json.rs      ← canonical JSON interchange
//! ├── csv.rs       ← tabular CSV export
//! └── markdown.rs  ← human/GitHub-friendly Markdown export
//! ```
//!
//! `mod.rs` intentionally contains no serialization implementation and no
//! report-generation algorithm.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

/// Complete benchmark-report model and report-level orchestration.
///
/// This source file currently exists in the repository with a trailing space
/// in its filename (`report.rs `). The explicit `#[path]` is intentional and
/// prevents this module from silently referring to a different file.
///
/// The filename should eventually be normalized to `report.rs`; until that
/// repository change is made, this declaration is the exact integration
/// contract for the current tree.
#[path = "report.rs "]
pub mod report;

/// Deterministic presentation-level benchmark summary.
///
/// This is the common source consumed by CSV and presentation-oriented
/// reporting.
pub mod summary;

/// Structured table representation used by reporting consumers.
///
/// Table construction remains separate from the complete report model so that
/// callers can consume benchmark data without selecting a textual format.
pub mod table;

/// Canonical lossless JSON serialization/deserialization.
///
/// JSON is the machine-readable interchange boundary for complete
/// `BenchmarkResult` values.
pub mod json;

/// Deterministic CSV export.
///
/// CSV is a tabular export representation and is not the authoritative
/// benchmark-result schema.
pub mod csv;

/// Deterministic Markdown export.
///
/// Markdown is intended for humans, GitHub, documentation, and CI summaries;
/// it is not the canonical machine interchange representation.
pub mod markdown;

// =============================================================================
// Stable reporting surface
// =============================================================================

/// Common reporting result type.
///
/// This is deliberately an alias rather than a second error hierarchy. The
/// concrete reporting modules remain authoritative for their format-specific
/// errors.
///
/// Consumers needing format-specific error handling should use the concrete
/// module API, for example `reporting::json::*`.
///
/// NOTE:
/// We intentionally do not manufacture a synthetic cross-format error type
/// here because doing so would force every serializer to depend on this
/// module's error model and would create an unnecessary coupling point.
pub use report::Report;

/// Human/machine presentation summary.
///
/// Re-exported because summaries are the common input to CSV/Markdown/table
/// reporting and are part of the stable reporting vocabulary.
pub use summary::BenchmarkSummary;

// =============================================================================
// API documentation and architectural invariants
// =============================================================================

/// Stable identifier for the reporting subsystem.
///
/// This is intentionally a constant rather than a runtime-generated value.
/// It can be used by registry/diagnostic code to identify the reporting
/// capability without depending on a concrete formatter.
pub const REPORTING_SUBSYSTEM_ID: &str = "zamani.quantum.benchmarking.reporting";

/// Current reporting module contract version.
///
/// This version describes the *module-level reporting contract*, not the
/// `BenchmarkResult` serialization schema. The JSON module owns the canonical
/// result-schema version.
///
/// Increment this only when the public reporting contract itself changes
/// incompatibly.
pub const REPORTING_API_VERSION: u16 = 1;

/// Canonical interchange format identifier.
///
/// JSON is the lossless representation of `BenchmarkResult`.
pub const CANONICAL_INTERCHANGE_FORMAT: &str = "json";

/// Returns the stable reporting subsystem identifier.
///
/// This small accessor provides a const-friendly API for callers that prefer a
/// function over direct constant access.
#[inline]
#[must_use]
pub const fn subsystem_id() -> &'static str {
    REPORTING_SUBSYSTEM_ID
}

/// Returns the reporting API contract version.
#[inline]
#[must_use]
pub const fn api_version() -> u16 {
    REPORTING_API_VERSION
}

/// Returns the canonical interchange format identifier.
#[inline]
#[must_use]
pub const fn canonical_interchange_format() -> &'static str {
    CANONICAL_INTERCHANGE_FORMAT
}

// =============================================================================
// Architectural smoke tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reporting_subsystem_identity_is_stable() {
        assert_eq!(
            subsystem_id(),
            "zamani.quantum.benchmarking.reporting"
        );
    }

    #[test]
    fn reporting_api_version_is_nonzero() {
        assert!(api_version() >= 1);
    }

    #[test]
    fn_json_is_the_canonical_interchange_format() {
        assert_eq!(canonical_interchange_format(), "json");
    }

    #[test]
    fn reporting_modules_are_reachable() {
        // These references intentionally exercise the module boundary without
        // constructing heavyweight benchmark objects.
        let _ = std::any::TypeId::of::<BenchmarkSummary>();
        let _ = std::any::TypeId::of::<Report>();
    }
}