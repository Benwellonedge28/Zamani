
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Control-Flow Verification
//!
//! Path: `src/zuir/verify/control_flow.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97.1
//! Safety: unsafe code is forbidden.
//!
//! PURPOSE
//!
//! Validate explicit control-flow structure in ZUIR regions and blocks.
//!
//! OWNS
//!
//! - Validation of finalized block terminators.
//! - Validation of region/block membership relevant to control flow.
//! - Validation of successor targets and destination argument arity.
//! - Deterministic diagnostics for malformed control-flow references.
//!
//! DOES NOT OWN
//!
//! - Source parsing, AST construction, or semantic name resolution.
//! - Canonical ID allocation.
//! - Whole-program containment verification beyond the relationships
//!   necessary to establish a region-local control-flow graph.
//! - SSA dominance, value definition/use correctness, or type compatibility.
//! - Dialect registration or extension-specific execution semantics.
//! - Resource feasibility, hardware placement, routing, scheduling, or runtime.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ir/successor.rs`
//! - `src/zuir/verify/structural.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/domains.md`
//!
//! SEMANTIC BOUNDARIES
//!
//! This verifier uses only explicitly represented successor edges.
//! Extension terminators are checked for their declared local structure,
//! but their full semantics must be checked by the registered extension.
//!
//! The verifier does not infer a region entry block from block ordering.
//! Entry-block semantics must be defined by the authoritative ZUIR contract
//! and represented or supplied explicitly by the relevant IR abstraction.
//!
//! Unreachable blocks are not automatically invalid: reachability policy is
//! separate from basic control-flow integrity.
//!
//! SCALABILITY
//!
//! No fixed block, edge, successor, argument, processor, qubit, device,
//! register-width, tensor-rank, or topology limit is imposed. Collections
//! grow with the input and available resources. Checked semantic validation
//! does not promise literal infinity; address space and allocator resources
//! remain finite.
//!
//! INTEGRATION
//!
//! Run structural verification and this verifier before transformations or
//! lowering. Run dominance, SSA, type, effect, ownership, and dialect
//! verification through their respective modules.
//!
//! A clean report means only that the control-flow checks implemented here
//! passed. It is not a claim that the entire program is semantically valid.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::ids::{BlockId, RegionId};
use crate::zuir::ir::block::{BlockTerminator, ZuirBlock};
use crate::zuir::ir::region::ZuirRegion;

// =============================================================================
// Public API
// =============================================================================

/// One machine-readable control-flow verification finding.
///
/// Diagnostic codes are stable API. Messages are explanatory text and
/// must not be parsed by tools as a substitute for the code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlFlowIssue {
    code: &'static str,
    message: String,
}

impl ControlFlowIssue {
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

    /// Return the explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ControlFlowIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

/// The entities required for control-flow verification.
///
/// The caller must supply every region and block referenced by the
/// representation under verification. Duplicate identities are diagnosed;
/// this verifier does not silently choose between conflicting definitions.
pub struct ControlFlowInput<'a> {
    /// Regions containing blocks.
    pub regions: &'a [ZuirRegion],

    /// Blocks referenced by regions and successor edges.
    pub blocks: &'a [ZuirBlock],
}

/// Result of validating explicit control-flow structure.
///
/// Findings may coexist with partial counts. Consumers must check
/// `is_valid()` before treating the result as evidence that these checks
/// passed for the complete supplied input.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ControlFlowReport {
    issues: Vec<ControlFlowIssue>,
    regions_checked: usize,
    blocks_checked: usize,
    edges_checked: usize,
}

impl ControlFlowReport {
    /// Borrow findings in deterministic verification order.
    #[must_use]
    pub fn issues(&self) -> &[ControlFlowIssue] {
        &self.issues
    }

    /// Whether no control-flow issues were found.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.issues.is_empty()
    }

    /// Return the number of findings.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Return the number of region entities inspected.
    #[must_use]
    pub const fn regions_checked(&self) -> usize {
        self.regions_checked
    }

    /// Return the number of block entities inspected.
    #[must_use]
    pub const fn blocks_checked(&self) -> usize {
        self.blocks_checked
    }

    /// Return the number of explicitly represented successor edges inspected.
    #[must_use]
    pub const fn edges_checked(&self) -> usize {
        self.edges_checked
    }

    fn push(&mut self, code: &'static str, message: impl Into<String>) {
        self.issues.push(ControlFlowIssue::new(code, message));
    }
}

// =============================================================================
// Verification entry point
// =============================================================================

/// Validate explicit control-flow structure without modifying the IR.
///
/// The checks include:
///
/// - duplicate region and block IDs;
/// - invalid region and block-local state;
/// - region/block membership consistency;
/// - missing, open, or locally invalid terminators;
/// - missing or cross-region successor targets;
/// - successor argument count versus destination block arguments.
///
/// This function deliberately does not validate argument types or whether
/// the passed values dominate the terminator. Those are separate verifier
/// responsibilities.
#[must_use]
pub fn verify_control_flow(input: &ControlFlowInput<'_>) -> ControlFlowReport {
    let mut report = ControlFlowReport {
        regions_checked: input.regions.len(),
        blocks_checked: input.blocks.len(),
        ..ControlFlowReport::default()
    };

    let regions_by_id = index_regions(input.regions, &mut report);
    let blocks_by_id = index_blocks(input.blocks, &mut report);

    verify_region_membership(
        input.regions,
        &regions_by_id,
        &blocks_by_id,
        &mut report,
    );

    verify_block_terminators_and_edges(
        input.regions,
        &regions_by_id,
        &blocks_by_id,
        &mut report,
    );

    report
}

// =============================================================================
// Identity indexes
// =============================================================================

fn index_regions<'a>(
    regions: &'a [ZuirRegion],
    report: &mut ControlFlowReport,
) -> BTreeMap<RegionId, &'a ZuirRegion> {
    let mut index = BTreeMap::new();

    for region in regions {
        if index.contains_key(&region.id()) {
            report.push(
                "ZUIR-CFG-DUPLICATE-REGION-ID",
                format!(
                    "region identity {} occurs more than once",
                    region.id()
                ),
            );
            continue;
        }

        index.insert(region.id(), region);

        if let Err(error) = region.validate() {
            report.push(
                "ZUIR-CFG-INVALID-REGION",
                format!(
                    "region {} failed local validation: {error}",
                    region.id()
                ),
            );
        }
    }

    index
}

fn index_blocks<'a>(
    blocks: &'a [ZuirBlock],
    report: &mut ControlFlowReport,
) -> BTreeMap<BlockId, &'a ZuirBlock> {
    let mut index = BTreeMap::new();

    for block in blocks {
        if index.contains_key(&block.id()) {
            report.push(
                "ZUIR-CFG-DUPLICATE-BLOCK-ID",
                format!(
                    "block identity {} occurs more than once",
                    block.id()
                ),
            );
            continue;
        }

        index.insert(block.id(), block);

        if let Err(error) = block.validate() {
            report.push(
                "ZUIR-CFG-INVALID-BLOCK",
                format!(
                    "block {} failed local validation: {error}",
                    block.id()
                ),
            );
        }
    }

    index
}

// =============================================================================
// Region and block membership
// =============================================================================

fn verify_region_membership(
    regions: &[ZuirRegion],
    regions_by_id: &BTreeMap<RegionId, &ZuirRegion>,
    blocks_by_id: &BTreeMap<BlockId, &ZuirBlock>,
    report: &mut ControlFlowReport,
) {
    // Track the first region that claims each block. A block must not be
    // assigned to multiple regions in the same verification view.
    let mut claimed_by = BTreeMap::<BlockId, RegionId>::new();

    for region in regions {
        // Duplicate region IDs were already diagnosed by index_regions.
        // Only the indexed canonical occurrence is used for cross-checks.
        if !regions_by_id
            .get(&region.id())
            .is_some_and(|indexed| std::ptr::eq(*indexed, region))
        {
            continue;
        }

        let mut local_seen = BTreeSet::<BlockId>::new();

        for &block_id in region.blocks() {
            if !local_seen.insert(block_id) {
                report.push(
                    "ZUIR-CFG-DUPLICATE-REGION-BLOCK",
                    format!(
                        "region {} lists block {} more than once",
                        region.id(),
                        block_id
                    ),
                );
                continue;
            }

            match blocks_by_id.get(&block_id) {
                None => {
                    report.push(
                        "ZUIR-CFG-MISSING-BLOCK",
                        format!(
                            "region {} references missing block {}",
                            region.id(),
                            block_id
                        ),
                    );
                }
                Some(block) => {
                    match block.parent_region() {
                        Some(parent) if parent == region.id() => {}
                        actual => report.push(
                            "ZUIR-CFG-BLOCK-REGION-MISMATCH",
                            format!(
                                "region {} lists block {}, but its declared \
                                 parent region is {:?}",
                                region.id(),
                                block_id,
                                actual
                            ),
                        ),
                    }

                    if let Some(previous_region) =
                        claimed_by.insert(block_id, region.id())
                    {
                        if previous_region != region.id() {
                            report.push(
                                "ZUIR-CFG-BLOCK-MULTIPLE-REGIONS",
                                format!(
                                    "block {} is listed by regions {} and {}",
                                    block_id,
                                    previous_region,
                                    region.id()
                                ),
                            );
                        }
                    }
                }
            }
        }
    }

    // Check the reverse relationship. Every supplied block must name an
    // existing parent region and appear in that region's block list.
    for block in blocks_by_id.values() {
        match block.parent_region() {
            None => report.push(
                "ZUIR-CFG-UNOWNED-BLOCK",
                format!("block {} has no parent region", block.id()),
            ),
            Some(region_id) => match regions_by_id.get(&region_id) {
                None => report.push(
                    "ZUIR-CFG-MISSING-PARENT-REGION",
                    format!(
                        "block {} names missing parent region {}",
                        block.id(),
                        region_id
                    ),
                ),
                Some(region) if !region.contains_block(block.id()) => {
                    report.push(
                        "ZUIR-CFG-REGION-OMITS-BLOCK",
                        format!(
                            "block {} names parent region {}, but that region \
                             does not list the block",
                            block.id(),
                            region_id
                        ),
                    );
                }
                Some(_) => {}
            },
        }
    }
}

// =============================================================================
// Terminator and edge validation
// =============================================================================

fn verify_block_terminators_and_edges(
    regions: &[ZuirRegion],
    regions_by_id: &BTreeMap<RegionId, &ZuirRegion>,
    blocks_by_id: &BTreeMap<BlockId, &ZuirBlock>,
    report: &mut ControlFlowReport,
) {
    for block in blocks_by_id.values() {
        let Some(parent_region_id) = block.parent_region() else {
            // Already diagnosed by membership validation. Without a parent,
            // the region-local legality of this block cannot be established.
            continue;
        };

        let Some(parent_region) = regions_by_id.get(&parent_region_id) else {
            // Already diagnosed by membership validation.
            continue;
        };

        if !parent_region.contains_block(block.id()) {
            // Already diagnosed by the reverse membership check.
            continue;
        }

        verify_terminator(block, report);

        block.terminator().for_each_successor(|successor| {
            report.edges_checked += 1;

            let target_id = successor.target();

            let Some(target) = blocks_by_id.get(&target_id).copied() else {
                report.push(
                    "ZUIR-CFG-MISSING-SUCCESSOR",
                    format!(
                        "block {} has a successor to missing block {}",
                        block.id(),
                        target_id
                    ),
                );
                return;
            };

            match target.parent_region() {
                Some(target_region) if target_region == parent_region_id => {}
                actual => {
                    report.push(
                        "ZUIR-CFG-CROSS-REGION-SUCCESSOR",
                        format!(
                            "block {} in region {} targets block {}, whose \
                             declared parent region is {:?}",
                            block.id(),
                            parent_region_id,
                            target_id,
                            actual
                        ),
                    );
                    return;
                }
            }

            if !parent_region.contains_block(target_id) {
                report.push(
                    "ZUIR-CFG-TARGET-NOT-IN-REGION",
                    format!(
                        "block {} targets block {}, but region {} does not \
                         list the target",
                        block.id(),
                        target_id,
                        parent_region_id
                    ),
                );
            }

            if successor.argument_count() != target.argument_count() {
                report.push(
                    "ZUIR-CFG-SUCCESSOR-ARITY",
                    format!(
                        "edge from block {} to block {} passes {} argument(s), \
                         but the destination declares {} block argument(s)",
                        block.id(),
                        target_id,
                        successor.argument_count(),
                        target.argument_count()
                    ),
                );
            }
        });
    }
}

fn verify_terminator(
    block: &ZuirBlock,
    report: &mut ControlFlowReport,
) {
    let terminator = block.terminator();

    if terminator.is_open() {
        report.push(
            "ZUIR-CFG-OPEN-TERMINATOR",
            format!(
                "block {} is not finalized with a closed terminator",
                block.id()
            ),
        );
        return;
    }

    if let Err(error) = terminator.validate() {
        report.push(
            "ZUIR-CFG-INVALID-TERMINATOR",
            format!(
                "block {} has an invalid terminator: {error}",
                block.id()
            ),
        );
    }

    // Terminator-specific edge enumeration is delegated to the canonical
    // BlockTerminator API. This avoids maintaining a competing list of
    // branch, switch, and extension terminator variants here.
    match terminator {
        BlockTerminator::Open
        | BlockTerminator::Return { .. }
        | BlockTerminator::Branch { .. }
        | BlockTerminator::ConditionalBranch { .. }
        | BlockTerminator::Switch { .. }
        | BlockTerminator::Unreachable
        | BlockTerminator::Extension { .. } => {}
    }
}
