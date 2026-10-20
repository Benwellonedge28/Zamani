
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Whole-Graph Structural Verification
//!
//! Path: `src/hdl/hdlir/verify/structure.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Verify structural invariants that require examining multiple HDLIR
//! objects together. Local constructors cannot establish graph-wide
//! reference validity, exclusive ownership, or consistency between
//! module, region, block, and operation registries.
//!
//! ## Architectural authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/ports.g4`
//! - `grammar/hdl/processes.g4`
//! - `grammar/specification/semantic-model.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/config.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/ir/design.rs`
//! - `src/hdl/hdlir/ir/module.rs`
//! - `src/hdl/hdlir/ir/regions.rs`
//! - `src/hdl/hdlir/ir/blocks.rs`
//! - `src/hdl/hdlir/ir/operations.rs`
//! - `src/hdl/hdlir/ir/values.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns syntax. The frontend owns source-level semantics.
//! ZUIR owns the universal computational representation. HDLIR owns the
//! hardware-domain graph. This module verifies graph structure only.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Whole-graph structural verification.
//! - Structural verification options and work-budget accounting.
//! - Structured verification issues and completion status.
//! - Cross-object reference and ownership consistency checks.
//!
//! This module does not own:
//!
//! - IR object definitions, ID allocation, or registry mutation.
//! - Source parsing, AST construction, name resolution, or type inference.
//! - Type compatibility, symbolic-width evaluation, or operation semantics.
//! - Clock-domain correctness, process semantics, or memory behavior.
//! - Physical allocation, placement, routing, synthesis, or timing closure.
//! - The universal ZUIR representation.
//!
//! ## Verification boundary
//!
//! This verifier checks the structural relationships expressible through
//! the existing design, module, region, block, and operation APIs.
//! It does not claim that structural validity proves type correctness,
//! behavioral equivalence, synthesizability, or target feasibility.
//!
//! ## Resource policy
//!
//! No universal maximum is imposed on modules, regions, blocks, operations,
//! ports, signals, or hardware resources. A caller may provide an explicit
//! work budget. If the budget is exhausted, verification returns an
//! incomplete report. Incomplete verification is never successful
//! verification.
//!
//! ## Integration contract
//!
//! `verify/mod.rs` must export this module and deliberately expose
//! `verify_structure`, `StructureVerificationOptions`, and
//! `StructureVerificationReport`.
//!
//! The caller supplies the design and the graph-owned region, block, and
//! operation collections. These collections must describe the same graph
//! snapshot. The caller must not mutate the graph while verification runs.
//!
//! The graph owner remains responsible for maintaining canonical storage.
//! This verifier does not silently repair malformed graphs.
//!
//! ## Completion criteria
//!
//! - Missing references are reported explicitly.
//! - Duplicate identities and illegal shared ownership are reported.
//! - Local structural checks are rerun before cross-object checks.
//! - Work-budget exhaustion is explicit and never reported as success.
//! - Output ordering is deterministic for equivalent ordered inputs.
//! - The file is integrated with the actual IR module tree.
//! - Focused and repository-wide checks pass.

use std::collections::{BTreeMap, BTreeSet};

use crate::hdl::hdlir::errors::HdlirError;
use crate::hdl::hdlir::ids::{
    HdlirBlockId, HdlirModuleId, HdlirOperationId, HdlirRegionId,
};
use crate::hdl::hdlir::ir::blocks::HdlirBlock;
use crate::hdl::hdlir::ir::design::{DesignModule, HdlirDesign};
use crate::hdl::hdlir::ir::module::HdlirModule;
use crate::hdl::hdlir::ir::operations::HdlirOperation;
use crate::hdl::hdlir::ir::regions::HdlirRegion;

/// Configuration for whole-graph structural verification.
///
/// `max_work` counts verification work units, not hardware capacity.
/// One work unit is charged for each graph object or cross-object reference
/// inspected. An absent budget imposes no additional verifier-specific
/// work limit.
///
/// The absence of an explicit work limit does not guarantee unlimited
/// execution time or memory.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StructureVerificationOptions {
    /// Maximum number of verification work units.
    ///
    /// `None` means that this configuration does not impose an explicit
    /// work-unit limit. `Some(0)` permits no work.
    pub max_work: Option<u64>,
}

/// Classification of a structural verification issue.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum StructureIssueKind {
    /// A graph object has an identity already used by another object
    /// of the same identity category.
    DuplicateIdentity,

    /// A reference points to an object absent from the supplied graph.
    MissingReference,

    /// An object is referenced by multiple owners where exclusive
    /// ownership is required.
    MultipleOwners,

    /// An object expected to have an owner has no owner.
    UnownedObject,

    /// A local object invariant is violated.
    InvalidLocalStructure,

    /// A registry key, stored identity, or ownership relationship disagrees.
    InconsistentStructure,

    /// The configured verification work budget was exhausted.
    BudgetExhausted,
}

/// A single structural verification finding.
///
/// Fields contain stable classification and descriptive context. Consumers
/// should branch on `kind`, not parse the human-readable `detail`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructureIssue {
    kind: StructureIssueKind,
    entity_kind: &'static str,
    identity: String,
    detail: String,
}

impl StructureIssue {
    fn new(
        kind: StructureIssueKind,
        entity_kind: &'static str,
        identity: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            entity_kind,
            identity: identity.into(),
            detail: detail.into(),
        }
    }

    /// Return the issue classification.
    #[must_use]
    pub const fn kind(&self) -> StructureIssueKind {
        self.kind
    }

    /// Return the affected entity category.
    #[must_use]
    pub const fn entity_kind(&self) -> &'static str {
        self.entity_kind
    }

    /// Return the affected identity as a display string.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// Return the human-readable explanation.
    #[must_use]
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

/// Result of whole-graph structural verification.
///
/// A report is valid only if verification completed and no issues were
/// discovered. `is_valid()` intentionally returns false for incomplete
/// verification, even if no structural violations were found before the
/// work budget was exhausted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructureVerificationReport {
    issues: Vec<StructureIssue>,
    complete: bool,
    work_performed: u64,
}

impl StructureVerificationReport {
    fn new() -> Self {
        Self {
            issues: Vec::new(),
            complete: true,
            work_performed: 0,
        }
    }

    /// Return all issues discovered by the completed portion of verification.
    ///
    /// When `is_complete()` is false, the issues are not an exhaustive
    /// account of the graph.
    #[must_use]
    pub fn issues(&self) -> &[StructureIssue] {
        &self.issues
    }

    /// Return whether all requested structural checks completed.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }

    /// Return whether verification completed without finding any issues.
    ///
    /// An incomplete report is never valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.complete && self.issues.is_empty()
    }

    /// Return the number of verification work units consumed.
    #[must_use]
    pub const fn work_performed(&self) -> u64 {
        self.work_performed
    }

    /// Return the number of issues found.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Return whether any issue has the specified classification.
    #[must_use]
    pub fn contains_kind(&self, kind: StructureIssueKind) -> bool {
        self.issues.iter().any(|issue| issue.kind == kind)
    }

    fn push(&mut self, issue: StructureIssue) {
        self.issues.push(issue);
    }

    fn stop_incomplete(&mut self) {
        self.complete = false;
        self.push(StructureIssue::new(
            StructureIssueKind::BudgetExhausted,
            "verification",
            "structure",
            "structural verification stopped because its work budget was exhausted",
        ));
    }

    fn consume_work(
        &mut self,
        options: StructureVerificationOptions,
    ) -> bool {
        if let Some(limit) = options.max_work {
            if self.work_performed >= limit {
                self.stop_incomplete();
                return false;
            }
        }

        match self.work_performed.checked_add(1) {
            Some(next) => {
                self.work_performed = next;
                true
            }
            None => {
                self.complete = false;
                self.push(StructureIssue::new(
                    StructureIssueKind::BudgetExhausted,
                    "verification",
                    "work_counter",
                    "verification work counter overflowed",
                ));
                false
            }
        }
    }
}

/// Verify the structure of one HDLIR design graph.
///
/// # Inputs
///
/// - `design`: the authoritative logical module registry.
/// - `regions`: the graph's region definitions.
/// - `blocks`: the graph's block definitions.
/// - `operations`: the graph's operation definitions.
/// - `options`: explicit verifier work-budget settings.
///
/// The caller must provide collections from the same graph snapshot.
/// The verifier does not clone or mutate the supplied IR objects.
///
/// # Checks
///
/// - Design-level registry invariants.
/// - Module, region, block, and operation identity uniqueness.
/// - Local structural invariants of modules, regions, blocks, and operations.
/// - Module-to-region references.
/// - Operation-to-region references.
/// - Region-to-block references.
/// - Block-to-operation references.
/// - Exclusive ownership of regions by modules or operations.
/// - Exclusive ownership of blocks by regions.
/// - Exclusive ownership of operations by blocks.
/// - Presence of an owner for every supplied region, block, and operation.
///
/// This function does not replace type, connectivity, process, clock,
/// resource, or dialect-specific verification.
///
/// # Budget behavior
///
/// When the configured work budget is exhausted, the returned report is
/// incomplete and `is_valid()` returns false. The verifier does not
/// truncate the work and then claim success.
///
/// # Errors
///
/// Structural violations are represented in the report rather than returned
/// as a first-error `HdlirError`. This permits callers to display multiple
/// independent findings in a single verification pass.
pub fn verify_structure(
    design: &HdlirDesign<HdlirModule>,
    regions: &[HdlirRegion],
    blocks: &[HdlirBlock],
    operations: &[HdlirOperation],
    options: StructureVerificationOptions,
) -> StructureVerificationReport {
    let mut report = StructureVerificationReport::new();

    if !report.consume_work(options) {
        return report;
    }

    if let Err(error) = design.validate_structure() {
        report.push(issue_from_error("design", design.name().to_string(), error));
    }

    // Build identity indexes from the caller-provided snapshot.
    // These indexes are verification-local and never become authoritative
    // graph state.
    let mut region_by_id = BTreeMap::new();
    let mut block_by_id = BTreeMap::new();
    let mut operation_by_id = BTreeMap::new();

    for region in regions {
        if !report.consume_work(options) {
            return report;
        }

        if region_by_id.insert(region.id(), region).is_some() {
            report.push(StructureIssue::new(
                StructureIssueKind::DuplicateIdentity,
                "region",
                region.id().to_string(),
                "multiple region definitions use the same identity",
            ));
        }

        if let Err(error) = region.validate_structure() {
            report.push(issue_from_error(
                "region",
                region.id().to_string(),
                error,
            ));
        }
    }

    for block in blocks {
        if !report.consume_work(options) {
            return report;
        }

        if block_by_id.insert(block.id(), block).is_some() {
            report.push(StructureIssue::new(
                StructureIssueKind::DuplicateIdentity,
                "block",
                block.id().to_string(),
                "multiple block definitions use the same identity",
            ));
        }

        if let Err(error) = block.validate_structure() {
            report.push(issue_from_error(
                "block",
                block.id().to_string(),
                error,
            ));
        }
    }

    for operation in operations {
        if !report.consume_work(options) {
            return report;
        }

        if operation_by_id.insert(operation.id(), operation).is_some() {
            report.push(StructureIssue::new(
                StructureIssueKind::DuplicateIdentity,
                "operation",
                operation.id().to_string(),
                "multiple operation definitions use the same identity",
            ));
        }

        if let Err(error) = operation.validate_structure() {
            report.push(issue_from_error(
                "operation",
                operation.id().to_string(),
                error,
            ));
        }
    }

    // Ownership maps are derived from explicit references.
    // Counts are used instead of first-owner-wins behavior so ambiguous
    // ownership is never silently accepted.
    let mut region_owners: BTreeMap<HdlirRegionId, Vec<String>> =
        BTreeMap::new();
    let mut block_owners: BTreeMap<HdlirBlockId, Vec<String>> =
        BTreeMap::new();
    let mut operation_owners: BTreeMap<HdlirOperationId, Vec<String>> =
        BTreeMap::new();

    // Module body regions are owned by their declaring module.
    for (module_id, module) in design.modules() {
        if !report.consume_work(options) {
            return report;
        }

        if *module_id != module.id() {
            report.push(StructureIssue::new(
                StructureIssueKind::InconsistentStructure,
                "module",
                module.id().to_string(),
                format!(
                    "design registry key {module_id} disagrees with module identity {}",
                    module.id()
                ),
            ));
        }

        if let Err(error) = module.validate_structure() {
            report.push(issue_from_error(
                "module",
                module.id().to_string(),
                error,
            ));
        }

        let mut seen_regions = BTreeSet::new();

        for region_id in module.body_regions() {
            if !report.consume_work(options) {
                return report;
            }

            if !seen_regions.insert(*region_id) {
                report.push(StructureIssue::new(
                    StructureIssueKind::DuplicateIdentity,
                    "module region reference",
                    region_id.to_string(),
                    format!(
                        "module {} references region {} more than once",
                        module.id(),
                        region_id
                    ),
                ));
                continue;
            }

            if !region_by_id.contains_key(region_id) {
                report.push(StructureIssue::new(
                    StructureIssueKind::MissingReference,
                    "region",
                    region_id.to_string(),
                    format!(
                        "module {} references a region absent from the graph",
                        module.id()
                    ),
                ));
                continue;
            }

            region_owners
                .entry(*region_id)
                .or_default()
                .push(format!("module {}", module.id()));
        }
    }

    // Nested regions are owned by their containing operations.
    for operation in operations {
        for region_id in operation.regions() {
            if !report.consume_work(options) {
                return report;
            }

            if !region_by_id.contains_key(region_id) {
                report.push(StructureIssue::new(
                    StructureIssueKind::MissingReference,
                    "region",
                    region_id.to_string(),
                    format!(
                        "operation {} references a region absent from the graph",
                        operation.id()
                    ),
                ));
                continue;
            }

            region_owners
                .entry(*region_id)
                .or_default()
                .push(format!("operation {}", operation.id()));
        }
    }

    // A region owns its ordered block references.
    for region in regions {
        for block_id in region.blocks() {
            if !report.consume_work(options) {
                return report;
            }

            if !block_by_id.contains_key(block_id) {
                report.push(StructureIssue::new(
                    StructureIssueKind::MissingReference,
                    "block",
                    block_id.to_string(),
                    format!(
                        "region {} references a block absent from the graph",
                        region.id()
                    ),
                ));
                continue;
            }

            block_owners
                .entry(*block_id)
                .or_default()
                .push(format!("region {}", region.id()));
        }
    }

    // A block owns its ordered operation references.
    for block in blocks {
        for operation_id in block.operations() {
            if !report.consume_work(options) {
                return report;
            }

            if !operation_by_id.contains_key(operation_id) {
                report.push(StructureIssue::new(
                    StructureIssueKind::MissingReference,
                    "operation",
                    operation_id.to_string(),
                    format!(
                        "block {} references an operation absent from the graph",
                        block.id()
                    ),
                ));
                continue;
            }

            operation_owners
                .entry(*operation_id)
                .or_default()
                .push(format!("block {}", block.id()));
        }
    }

    // Validate exclusive region ownership.
    for region in regions {
        if !report.consume_work(options) {
            return report;
        }

        match region_owners.get(&region.id()) {
            None => report.push(StructureIssue::new(
                StructureIssueKind::UnownedObject,
                "region",
                region.id().to_string(),
                "region is not referenced by a module or containing operation",
            )),
            Some(owners) if owners.len() > 1 => {
                report.push(StructureIssue::new(
                    StructureIssueKind::MultipleOwners,
                    "region",
                    region.id().to_string(),
                    format!("region has multiple owners: {}", owners.join(", ")),
                ));
            }
            Some(_) => {}
        }
    }

    // Validate exclusive block ownership.
    for block in blocks {
        if !report.consume_work(options) {
            return report;
        }

        match block_owners.get(&block.id()) {
            None => report.push(StructureIssue::new(
                StructureIssueKind::UnownedObject,
                "block",
                block.id().to_string(),
                "block is not referenced by any region",
            )),
            Some(owners) if owners.len() > 1 => {
                report.push(StructureIssue::new(
                    StructureIssueKind::MultipleOwners,
                    "block",
                    block.id().to_string(),
                    format!("block has multiple owners: {}", owners.join(", ")),
                ));
            }
            Some(_) => {}
        }
    }

    // Validate exclusive operation ownership.
    for operation in operations {
        if !report.consume_work(options) {
            return report;
        }

        match operation_owners.get(&operation.id()) {
            None => report.push(StructureIssue::new(
                StructureIssueKind::UnownedObject,
                "operation",
                operation.id().to_string(),
                "operation is not referenced by any block",
            )),
            Some(owners) if owners.len() > 1 => {
                report.push(StructureIssue::new(
                    StructureIssueKind::MultipleOwners,
                    "operation",
                    operation.id().to_string(),
                    format!(
                        "operation has multiple owners: {}",
                        owners.join(", ")
                    ),
                ));
            }
            Some(_) => {}
        }
    }

    report
}

fn issue_from_error(
    entity_kind: &'static str,
    identity: String,
    error: HdlirError,
) -> StructureIssue {
    StructureIssue::new(
        StructureIssueKind::InvalidLocalStructure,
        entity_kind,
        identity,
        error.to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        verify_structure, StructureIssueKind, StructureVerificationOptions,
    };
    use crate::hdl::hdlir::ids::HdlirDesignId;
    use crate::hdl::hdlir::ir::design::HdlirDesign;
    use crate::hdl::hdlir::ir::module::HdlirModule;

    fn empty_design() -> HdlirDesign<HdlirModule> {
        HdlirDesign::new(
            HdlirDesignId::new(1).expect("non-zero design ID"),
            "empty_design",
        )
        .expect("valid design")
    }

    #[test]
    fn zero_work_budget_returns_incomplete_report() {
        let design = empty_design();

        let report = verify_structure(
            &design,
            &[],
            &[],
            &[],
            StructureVerificationOptions {
                max_work: Some(0),
            },
        );

        assert!(!report.is_complete());
        assert!(!report.is_valid());
        assert!(report.contains_kind(StructureIssueKind::BudgetExhausted));
        assert_eq!(report.work_performed(), 0);
    }

    #[test]
    fn empty_design_graph_is_structurally_verifiable() {
        let design = empty_design();

        let report = verify_structure(
            &design,
            &[],
            &[],
            &[],
            StructureVerificationOptions::default(),
        );

        assert!(report.is_complete());
        assert!(report.is_valid());
        assert!(report.issues().is_empty());
        assert!(report.work_performed() > 0);
    }

    #[test]
    fn insufficient_budget_never_reports_success() {
        let design = empty_design();

        let report = verify_structure(
            &design,
            &[],
            &[],
            &[],
            StructureVerificationOptions {
                max_work: Some(1),
            },
        );

        assert!(!report.is_valid());
        assert!(
            report.is_complete()
                || report.contains_kind(StructureIssueKind::BudgetExhausted)
        );
    }
}
