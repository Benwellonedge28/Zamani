
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZCIR Resource Verification Facade
//!
//! Path: `src/classical/zcir/verify/resources.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only; `unsafe` is forbidden.
//!
//! ## Purpose
//!
//! Expose canonical ZUIR resource-intent verification through the ZCIR
//! verification API. This module is an integration boundary, not a second
//! resource-verification implementation.
//!
//! ## Authority and ownership
//!
//! `crate::zuir::ir::resources` owns canonical resource-intent data types.
//! `crate::zuir::verify::resources` owns canonical resource verification,
//! findings, consistency checks, and verification reports.
//!
//! This module owns only:
//! - ZCIR-facing names for the canonical API;
//! - the ZCIR verification entry point;
//! - API integration tests.
//!
//! It does not own resource declarations, capability definitions, resource
//! units, hardware inventories, resource estimates, allocation, scheduling,
//! target feasibility, or physical execution.
//!
//! ## Semantic contract
//!
//! - Requirements and constraints retain their canonical mandatory meaning.
//! - Preferences and hints must not be promoted to mandatory requirements.
//! - Resource budgets retain the semantics of the canonical resource model.
//! - Resource quantities and identifiers are not narrowed to machine-sized
//!   integers or closed lists of hardware categories by this facade.
//! - Different units must not be compared without an explicit conversion
//!   contract.
//! - Unknown extension semantics must not be treated as verified.
//! - A valid intent does not prove that a target has the required resources.
//! - Resource feasibility and physical allocation remain downstream tasks.
//!
//! ## Scalability
//!
//! This facade introduces no resource-count, assertion-count, quantity,
//! device-count, or hardware-population ceiling. Scalability is determined
//! by the canonical verifier, available memory and processing time, and
//! explicitly configured operational budgets.
//!
//! This module must not silently truncate input or replace a resource
//! requirement with a weaker one when processing cannot continue.
//!
//! ## Integration
//!
//! The semantic-model-to-ZUIR adapter must supply canonical resource intents
//! and logical scope labels. The full ZUIR verifier remains responsible for
//! invoking the canonical resource-verification stage and aggregating its
//! findings.
//!
//! ZCIR callers should prefer `verify_zcir_resources`. Callers that already
//! use the canonical API can use the re-exported types and function without
//! conversion or copying.
//!
//! The legacy ZCIR-local resource model in `src/classical/zcir/resources.rs`
//! must not be silently treated as canonical ZUIR data. Any migration from
//! that model requires a separately reviewed, explicit adapter preserving
//! dimensions, quantities, units, intent categories, and comparison meaning.
//!
//! ## Freeze conditions
//!
//! This file is ready to freeze only when:
//! - the parent ZCIR module registers `verify`;
//! - `verify/mod.rs` registers this `resources` module;
//! - the repository compiles against the canonical ZUIR API;
//! - API and integration tests pass;
//! - callers do not maintain a competing resource-verification authority.

/// Canonical resource-verification entry type.
///
/// The scope identifies which declarations constrain the same logical
/// resource environment. Scope assignment is the caller's responsibility.
pub use crate::zuir::verify::resources::ResourceIntentEntry;

/// Canonical resource-verification diagnostic.
pub use crate::zuir::verify::resources::ResourceIssue;

/// Canonical resource-verification diagnostic severity.
pub use crate::zuir::verify::resources::ResourceIssueSeverity;

/// Canonical resource-verification report.
pub use crate::zuir::verify::resources::ResourceVerificationReport;

/// Canonical resource-verification implementation.
///
/// Re-exporting this function preserves one authoritative implementation
/// and allows existing ZUIR callers to share the same contract.
pub use crate::zuir::verify::resources::verify_resources;

/// ZCIR-facing alias for the canonical resource-intent entry.
pub type ZcirResourceIntentEntry<'a> = ResourceIntentEntry<'a>;

/// ZCIR-facing alias for the canonical resource diagnostic.
pub type ZcirResourceIssue = ResourceIssue;

/// ZCIR-facing alias for the canonical resource diagnostic severity.
pub type ZcirResourceIssueSeverity = ResourceIssueSeverity;

/// ZCIR-facing alias for the canonical resource-verification report.
pub type ZcirResourceVerificationReport = ResourceVerificationReport;

/// Verify scoped resource intents using the canonical ZUIR verifier.
///
/// This function validates resource-intent structure and the consistency
/// properties implemented by ZUIR. It does not query a target, calculate
/// actual resource consumption, allocate hardware, or establish execution
/// feasibility.
///
/// # Arguments
///
/// * `entries` — borrowed canonical resource intents with caller-defined
///   logical scope labels.
///
/// # Result
///
/// Returns the canonical report, including structured errors and any
/// deferred checks. Callers that require complete verification must inspect
/// both `is_valid()` and `is_complete()`.
///
/// # Determinism
///
/// This facade adds no ordering or transformations of its own. Findings
/// follow the canonical verifier's report contract.
///
/// # Scalability
///
/// No fixed capacity or resource-count limit is introduced here.
#[must_use]
pub fn verify_zcir_resources(
    entries: &[ZcirResourceIntentEntry<'_>],
) -> ZcirResourceVerificationReport {
    verify_resources(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zcir_entry_point_accepts_canonical_resource_entries() {
        let entries: [ZcirResourceIntentEntry<'_>; 0] = [];
        let report = verify_zcir_resources(&entries);

        assert!(report.is_valid());
        assert!(report.is_complete());
        assert_eq!(report.intents_checked(), 0);
        assert_eq!(report.assertions_checked(), 0);
        assert_eq!(report.capabilities_checked(), 0);
        assert_eq!(report.topology_relations_checked(), 0);
        assert!(report.issues().is_empty());
    }

    #[test]
    fn zcir_names_are_exact_aliases_of_canonical_types() {
        let canonical_entry: Option<ResourceIntentEntry<'_>> = None;
        let zcir_entry: Option<ZcirResourceIntentEntry<'_>> = canonical_entry;
        let canonical_issue: Option<ResourceIssue> = None;
        let zcir_issue: Option<ZcirResourceIssue> = canonical_issue;
        let canonical_severity: Option<ResourceIssueSeverity> = None;
        let zcir_severity: Option<ZcirResourceIssueSeverity> =
            canonical_severity;
        let canonical_report: Option<ResourceVerificationReport> = None;
        let zcir_report: Option<ZcirResourceVerificationReport> =
            canonical_report;

        assert!(zcir_entry.is_none());
        assert!(zcir_issue.is_none());
        assert!(zcir_severity.is_none());
        assert!(zcir_report.is_none());
    }

    #[test]
    fn zcir_entry_point_delegates_to_the_canonical_contract() {
        let canonical_entry: fn(
            &[ResourceIntentEntry<'_>],
        ) -> ResourceVerificationReport = verify_resources;

        let zcir_entry: fn(
            &[ZcirResourceIntentEntry<'_>],
        ) -> ZcirResourceVerificationReport = verify_zcir_resources;

        // These assignments are compile-time checks that both entry points
        // expose compatible canonical input and report types.
        let _ = canonical_entry;
        let _ = zcir_entry;
    }
}
