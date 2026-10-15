
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Ownership Verification
//!
//! Canonical path: `src/zuir/verify/ownership.rs`
//!
//! PURPOSE
//!
//! Verify ownership-index consistency, ownership contracts, operation and
//! block references, declared capabilities, and region capture declarations.
//!
//! AUTHORITY
//!
//! - `src/zuir/ir/ownership.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/ir/operand.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ids.rs`
//! - `grammar/ownership.g4` and the applicable semantic specifications.
//!
//! OWNS
//!
//! - Ownership verification input, findings, and report.
//! - Cross-reference checks for recorded ownership events.
//! - Local validation of ownership capabilities and region captures.
//!
//! DOES NOT OWN
//!
//! - Ownership contract storage or mutation.
//! - Global SSA definition resolution.
//! - Path-sensitive linearity and affine-use analysis.
//! - Complete borrow lifetime, aliasing, or escape analysis.
//! - Dialect-specific ownership semantics.
//!
//! LIMITATIONS
//!
//! A successful report proves only the properties implemented here.
//! Full ownership safety requires CFG-aware dataflow analysis and
//! operation-specific semantics. Unknown extension ownership events are
//! reported rather than interpreted as core-language behavior.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::ids::{BlockId, OperationId, RegionId, ValueId};
use crate::zuir::ir::{
    CaptureMode, OwnershipCapabilities, OwnershipIndex, OwnershipKind,
    OwnershipUse, OwnershipUseKind, RegionIsolation, ZuirBlock,
    ZuirOperation, ZuirRegion,
};

// =============================================================================
// Public API
// =============================================================================

/// All entities required for ownership-index verification.
///
/// The caller supplies the canonical ownership index and the corresponding
/// region, block, and operation collections. This verifier does not mutate
/// any of these entities.
pub struct OwnershipVerificationInput<'a> {
    /// Ownership contracts and recorded ownership events.
    pub ownership: &'a OwnershipIndex,

    /// Regions referenced by the representation being verified.
    pub regions: &'a [ZuirRegion],

    /// Blocks referenced by those regions.
    pub blocks: &'a [ZuirBlock],

    /// Operations referenced by those blocks.
    pub operations: &'a [ZuirOperation],
}

/// One independently actionable ownership-verification finding.
///
/// The diagnostic code is the stable machine-readable interface. The
/// message is intended for people and may evolve.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnershipIssue {
    code: &'static str,
    message: String,
}

impl OwnershipIssue {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    /// Return the stable diagnostic code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.code
    }

    /// Return the human-readable explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for OwnershipIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

/// Summary of one ownership-verification run.
///
/// A report can contain several findings. A valid report means that the
/// checks implemented in this module passed; it does not prove path-sensitive
/// ownership correctness or the validity of unregistered extension semantics.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OwnershipVerificationReport {
    issues: Vec<OwnershipIssue>,
    contracts_checked: usize,
    uses_checked: usize,
    regions_checked: usize,
    blocks_checked: usize,
    operations_checked: usize,
}

impl OwnershipVerificationReport {
    /// Borrow all findings in deterministic verification order.
    #[must_use]
    pub fn issues(&self) -> &[OwnershipIssue] {
        &self.issues
    }

    /// Whether this verifier found no issues.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.issues.is_empty()
    }

    /// Number of findings.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Number of ownership contracts inspected.
    #[must_use]
    pub const fn contracts_checked(&self) -> usize {
        self.contracts_checked
    }

    /// Number of recorded ownership events inspected.
    #[must_use]
    pub const fn uses_checked(&self) -> usize {
        self.uses_checked
    }

    /// Number of regions inspected.
    #[must_use]
    pub const fn regions_checked(&self) -> usize {
        self.regions_checked
    }

    /// Number of blocks inspected.
    #[must_use]
    pub const fn blocks_checked(&self) -> usize {
        self.blocks_checked
    }

    /// Number of operations inspected.
    #[must_use]
    pub const fn operations_checked(&self) -> usize {
        self.operations_checked
    }

    fn push(&mut self, code: &'static str, message: impl Into<String>) {
        self.issues.push(OwnershipIssue::new(code, message));
    }
}

// =============================================================================
// Entry point
// =============================================================================

/// Verify ownership contracts, recorded events, and their IR references.
///
/// Entity indexes are ordered by canonical ID, making duplicate-identity
/// detection and cross-reference lookup deterministic. Findings are emitted
/// in deterministic phases and collection traversal order.
///
/// This function does not perform path-sensitive ownership-flow analysis.
#[must_use]
pub fn verify_ownership(
    input: &OwnershipVerificationInput<'_>,
) -> OwnershipVerificationReport {
    let mut report = OwnershipVerificationReport::default();

    let regions = index_regions(input.regions, &mut report);
    let blocks = index_blocks(input.blocks, &mut report);
    let operations = index_operations(input.operations, &mut report);

    let operation_blocks =
        index_operation_owners(input.blocks, &blocks, &mut report);

    verify_index(input.ownership, &mut report);
    verify_contracts(input.ownership, &regions, &mut report);
    verify_region_captures(input.ownership, &regions, &mut report);

    verify_recorded_uses(
        input.ownership,
        &regions,
        &blocks,
        &operations,
        &operation_blocks,
        &mut report,
    );

    report
}

// =============================================================================
// Entity indexes
// =============================================================================

fn index_regions<'a>(
    entities: &'a [ZuirRegion],
    report: &mut OwnershipVerificationReport,
) -> BTreeMap<RegionId, &'a ZuirRegion> {
    let mut index = BTreeMap::new();

    for region in entities {
        report.regions_checked += 1;

        if index.insert(region.id(), region).is_some() {
            report.push(
                "ZUIR-OWN-DUPLICATE-REGION",
                format!("region {} is defined more than once", region.id()),
            );
        }

        if let Err(error) = region.validate() {
            report.push(
                "ZUIR-OWN-INVALID-REGION",
                format!("region {} failed local validation: {error}", region.id()),
            );
        }
    }

    index
}

fn index_blocks<'a>(
    entities: &'a [ZuirBlock],
    report: &mut OwnershipVerificationReport,
) -> BTreeMap<BlockId, &'a ZuirBlock> {
    let mut index = BTreeMap::new();

    for block in entities {
        report.blocks_checked += 1;

        if index.insert(block.id(), block).is_some() {
            report.push(
                "ZUIR-OWN-DUPLICATE-BLOCK",
                format!("block {} is defined more than once", block.id()),
            );
        }

        if let Err(error) = block.validate() {
            report.push(
                "ZUIR-OWN-INVALID-BLOCK",
                format!("block {} failed local validation: {error}", block.id()),
            );
        }
    }

    index
}

fn index_operations<'a>(
    entities: &'a [ZuirOperation],
    report: &mut OwnershipVerificationReport,
) -> BTreeMap<OperationId, &'a ZuirOperation> {
    let mut index = BTreeMap::new();

    for operation in entities {
        report.operations_checked += 1;

        if index.insert(operation.id(), operation).is_some() {
            report.push(
                "ZUIR-OWN-DUPLICATE-OPERATION",
                format!(
                    "operation {} is defined more than once",
                    operation.id()
                ),
            );
        }

        if let Err(error) = operation.validate() {
            report.push(
                "ZUIR-OWN-INVALID-OPERATION",
                format!(
                    "operation {} failed local validation: {error}",
                    operation.id()
                ),
            );
        }
    }

    index
}

/// Build an operation-to-containing-block index.
///
/// An operation referenced by multiple distinct blocks is reported as an
/// ownership/containment inconsistency rather than silently assigned to one.
fn index_operation_owners(
    blocks: &[ZuirBlock],
    block_index: &BTreeMap<BlockId, &ZuirBlock>,
    report: &mut OwnershipVerificationReport,
) -> BTreeMap<OperationId, BlockId> {
    let mut owners = BTreeMap::new();

    for block in blocks {
        // Skip duplicate block identities after their duplicate has been
        // reported. The first canonical occurrence remains authoritative for
        // lookup during this verification run.
        if block_index.get(&block.id()).copied() != Some(block) {
            continue;
        }

        for operation in block.operations() {
            if let Some(previous) = owners.insert(*operation, block.id()) {
                report.push(
                    "ZUIR-OWN-OPERATION-MULTIPLE-BLOCKS",
                    format!(
                        "operation {operation} is referenced by blocks \
                         {previous} and {}",
                        block.id()
                    ),
                );
            }
        }
    }

    owners
}

// =============================================================================
// Index and contract checks
// =============================================================================

fn verify_index(
    ownership: &OwnershipIndex,
    report: &mut OwnershipVerificationReport,
) {
    if let Err(error) = ownership.validate() {
        report.push(
            "ZUIR-OWN-INVALID-INDEX",
            format!("ownership index failed validation: {error}"),
        );
    }

    report.contracts_checked = ownership.contract_count();
    report.uses_checked = ownership.use_count();
}

fn verify_contracts(
    ownership: &OwnershipIndex,
    regions: &BTreeMap<RegionId, &ZuirRegion>,
    report: &mut OwnershipVerificationReport,
) {
    for (value, contract) in ownership.contracts() {
        if contract.value != *value {
            report.push(
                "ZUIR-OWN-CONTRACT-KEY-MISMATCH",
                format!(
                    "ownership index key {value} differs from contract value {}",
                    contract.value
                ),
            );
        }

        if let Err(error) = contract.validate() {
            report.push(
                "ZUIR-OWN-INVALID-CONTRACT",
                format!("ownership contract for {value} is invalid: {error}"),
            );
        }

        if let Some(region_id) = contract.declared_in {
            if !regions.contains_key(&region_id) {
                report.push(
                    "ZUIR-OWN-CONTRACT-REGION-MISSING",
                    format!(
                        "ownership contract for {value} refers to missing \
                         declaration region {region_id}"
                    ),
                );
            }
        }
    }
}

// =============================================================================
// Capture checks
// =============================================================================

fn verify_region_captures(
    ownership: &OwnershipIndex,
    regions: &BTreeMap<RegionId, &ZuirRegion>,
    report: &mut OwnershipVerificationReport,
) {
    for region in regions.values() {
        for (value, mode) in region.captures() {
            let Some(contract) = ownership.contract(value) else {
                report.push(
                    "ZUIR-OWN-CAPTURE-CONTRACT-MISSING",
                    format!(
                        "region {} captures value {value} without an \
                         ownership contract",
                        region.id()
                    ),
                );
                continue;
            };

            if region.isolation() == RegionIsolation::Isolated {
                report.push(
                    "ZUIR-OWN-CAPTURE-IN-ISOLATED-REGION",
                    format!(
                        "isolated region {} captures value {value}",
                        region.id()
                    ),
                );
            }

            if !capture_mode_is_allowed(mode, contract.capabilities) {
                report.push(
                    "ZUIR-OWN-CAPTURE-CAPABILITY",
                    format!(
                        "region {} capture of value {value} using mode \
                         {mode:?} conflicts with its ownership capabilities",
                        region.id()
                    ),
                );
            }
        }
    }
}

fn capture_mode_is_allowed(
    mode: CaptureMode,
    capabilities: OwnershipCapabilities,
) -> bool {
    match mode {
        CaptureMode::Read => {
            capabilities.may_borrow_shared
                || capabilities.may_copy
                || capabilities.may_move
        }
        CaptureMode::Write => {
            capabilities.may_borrow_exclusive || capabilities.may_move
        }
        CaptureMode::Move => capabilities.may_move,
        CaptureMode::BorrowShared => capabilities.may_borrow_shared,
        CaptureMode::BorrowExclusive => capabilities.may_borrow_exclusive,
    }
}

// =============================================================================
// Recorded use verification
// =============================================================================

fn verify_recorded_uses(
    ownership: &OwnershipIndex,
    regions: &BTreeMap<RegionId, &ZuirRegion>,
    blocks: &BTreeMap<BlockId, &ZuirBlock>,
    operations: &BTreeMap<OperationId, &ZuirOperation>,
    operation_blocks: &BTreeMap<OperationId, BlockId>,
    report: &mut OwnershipVerificationReport,
) {
    for (indexed_value, uses) in ownership.all_uses() {
        for usage in uses {
            if usage.value != *indexed_value {
                report.push(
                    "ZUIR-OWN-USE-KEY-MISMATCH",
                    format!(
                        "use-index key {indexed_value} differs from event value {}",
                        usage.value
                    ),
                );
            }

            let Some(contract) = ownership.contract(usage.value) else {
                report.push(
                    "ZUIR-OWN-USE-CONTRACT-MISSING",
                    format!(
                        "ownership event for value {} has no contract",
                        usage.value
                    ),
                );
                continue;
            };

            verify_event_capability(usage, contract.kind.clone(), contract.capabilities, report);

            let Some(block) = blocks.get(&usage.block).copied() else {
                report.push(
                    "ZUIR-OWN-USE-BLOCK-MISSING",
                    format!(
                        "ownership event for value {} refers to missing block {}",
                        usage.value, usage.block
                    ),
                );
                continue;
            };

            let Some(operation) = operations.get(&usage.operation).copied() else {
                report.push(
                    "ZUIR-OWN-USE-OPERATION-MISSING",
                    format!(
                        "ownership event for value {} refers to missing operation {}",
                        usage.value, usage.operation
                    ),
                );
                continue;
            };

            if !block.operations().contains(&usage.operation) {
                report.push(
                    "ZUIR-OWN-USE-OPERATION-NOT-IN-BLOCK",
                    format!(
                        "ownership event operation {} is not listed in block {}",
                        usage.operation, usage.block
                    ),
                );
            }

            if operation_blocks.get(&usage.operation).copied()
                != Some(usage.block)
            {
                report.push(
                    "ZUIR-OWN-USE-BLOCK-MISMATCH",
                    format!(
                        "ownership event for operation {} names block {}, \
                         but the operation's indexed block is {:?}",
                        usage.operation,
                        usage.block,
                        operation_blocks.get(&usage.operation)
                    ),
                );
            }

            verify_event_has_ir_evidence(
                usage,
                operation,
                regions,
                report,
            );
        }
    }
}

fn verify_event_capability(
    usage: &OwnershipUse,
    kind: OwnershipKind,
    capabilities: OwnershipCapabilities,
    report: &mut OwnershipVerificationReport,
) {
    let permitted = match &usage.kind {
        OwnershipUseKind::Read => {
            capabilities.may_borrow_shared || capabilities.may_copy
        }
        OwnershipUseKind::Write => capabilities.may_borrow_exclusive,
        OwnershipUseKind::Move => capabilities.may_move,
        OwnershipUseKind::BorrowShared
        | OwnershipUseKind::CaptureShared => capabilities.may_borrow_shared,
        OwnershipUseKind::BorrowExclusive
        | OwnershipUseKind::CaptureExclusive => {
            capabilities.may_borrow_exclusive
        }
        OwnershipUseKind::Drop => capabilities.may_drop,
        OwnershipUseKind::Extension(_) => {
            report.push(
                "ZUIR-OWN-EXTENSION-SEMANTICS-REQUIRED",
                format!(
                    "ownership event for value {} uses extension semantics; \
                     a registered extension verifier must authorize it",
                    usage.value
                ),
            );
            true
        }
    };

    if !permitted {
        report.push(
            "ZUIR-OWN-CAPABILITY-VIOLATION",
            format!(
                "event {:?} for value {} is not permitted by its capabilities",
                usage.kind, usage.value
            ),
        );
    }

    // Keep ownership classification visible in diagnostics for cases where
    // a classification is stricter than the event capability flags alone.
    match kind {
        OwnershipKind::Linear if matches!(usage.kind, OwnershipUseKind::Drop) => {
            report.push(
                "ZUIR-OWN-LINEAR-DROP",
                format!("linear value {} cannot be explicitly dropped", usage.value),
            );
        }
        OwnershipKind::Borrowed { .. }
            if matches!(usage.kind, OwnershipUseKind::Move) =>
        {
            report.push(
                "ZUIR-OWN-MOVE-BORROWED",
                format!("borrowed value {} cannot be moved as an owned value", usage.value),
            );
        }
        _ => {}
    }
}

fn verify_event_has_ir_evidence(
    usage: &OwnershipUse,
    operation: &ZuirOperation,
    regions: &BTreeMap<RegionId, &ZuirRegion>,
    report: &mut OwnershipVerificationReport,
) {
    let is_capture_event = matches!(
        &usage.kind,
        OwnershipUseKind::CaptureShared | OwnershipUseKind::CaptureExclusive
    );

    let operand_evidence = operation
        .operands()
        .iter()
        .any(|operand| operand.value() == usage.value);

    let capture_evidence = operation.regions().iter().any(|region_id| {
        regions
            .get(region_id)
            .is_some_and(|region| region.capture_mode(usage.value).is_some())
    });

    let has_evidence = if is_capture_event {
        capture_evidence
    } else {
        operand_evidence
    };

    if !has_evidence {
        report.push(
            "ZUIR-OWN-USE-NO-IR-EVIDENCE",
            format!(
                "ownership event {:?} for value {} is not supported by an \
                 operand of operation {} or by a capture of its nested region",
                usage.kind, usage.value, usage.operation
            ),
        );
    }

    if is_capture_event && !capture_evidence {
        report.push(
            "ZUIR-OWN-CAPTURE-EVENT-NOT-DECLARED",
            format!(
                "operation {} records a capture event for value {}, but none \
                 of its nested regions declares that capture",
                usage.operation, usage.value
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_ownership_index_is_valid_for_empty_entity_sets() {
        let ownership = OwnershipIndex::new();
        let regions = [];
        let blocks = [];
        let operations = [];

        let input = OwnershipVerificationInput {
            ownership: &ownership,
            regions: &regions,
            blocks: &blocks,
            operations: &operations,
        };

        let report = verify_ownership(&input);

        assert!(report.is_valid(), "{:?}", report.issues());
        assert_eq!(report.contracts_checked(), 0);
        assert_eq!(report.uses_checked(), 0);
    }
}
