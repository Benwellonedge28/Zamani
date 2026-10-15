
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Structural Verification
//!
//! Path: `src/zuir/verify/structural.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.1
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Verify local IR invariants and cross-entity structural relationships
//! before semantic verification, optimization, serialization, or lowering.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/`
//! - `src/zuir/errors.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/domains.md`
//!
//! OWNS
//!
//! - Cross-entity structural integrity.
//! - Canonical identity uniqueness within the supplied verification view.
//! - Module/function, function/region, region/block, block/operation,
//!   operation/region, and control-flow reference integrity.
//! - Structured structural-verification findings.
//!
//! DOES NOT OWN
//!
//! - Parsing or AST construction.
//! - Name resolution or semantic type checking.
//! - Dialect registration or extension authorization.
//! - SSA dominance, complete use-def correctness, or effect inference.
//! - Quantum gate legality, hardware allocation, routing, or scheduling.
//! - Target feasibility or runtime execution.
//!
//! SCALABILITY
//!
//! There are no fixed node, edge, qubit, processor, memory, tensor-rank,
//! device-count, or topology limits. Work and indexes scale with the supplied
//! IR and the resources available to the process.
//!
//! The verifier does not promise literal infinity. Standard collection
//! allocation may abort on some platforms when memory is exhausted.
//!
//! INTEGRATION
//!
//! The caller supplies a complete structural view. All referenced entities
//! must be included, including entities referenced by nested regions.
//! Run this verifier before semantic verification and lowering.
//!
//! FREEZE CRITERIA
//!
//! - Uses canonical IDs and IR entity types.
//! - Never silently ignores missing references.
//! - Reports duplicate IDs instead of overwriting index entries.
//! - Validates reciprocal containment relationships.
//! - Does not introduce target-specific assumptions.
//! - Uses no unsafe code.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::ids::{
    BlockId, FunctionId, ModuleId, OperationId, RegionId,
};
use crate::zuir::ir::{
    BlockTerminator, RegionOwner, ZuirBlock, ZuirFunction, ZuirModule,
    ZuirOperation, ZuirProgram, ZuirRegion,
};

// =============================================================================
// Public API
// =============================================================================

/// One independently actionable structural-verification finding.
///
/// `code` is a stable machine-readable identifier. `message` is explanatory
/// text and must not be used as a stable machine interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralIssue {
    code: &'static str,
    message: String,
}

impl StructuralIssue {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    /// Return the stable structural issue code.
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

impl fmt::Display for StructuralIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

/// The outcome of structural verification.
///
/// A report may contain multiple independent findings. A report with no
/// issues means only that the checks implemented here passed; it is not a
/// proof of semantic correctness, type correctness, or target compatibility.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StructuralReport {
    issues: Vec<StructuralIssue>,
    modules_checked: usize,
    functions_checked: usize,
    regions_checked: usize,
    blocks_checked: usize,
    operations_checked: usize,
}

impl StructuralReport {
    /// Return all findings in deterministic verification order.
    #[must_use]
    pub fn issues(&self) -> &[StructuralIssue] {
        &self.issues
    }

    /// Return whether structural verification found no issues.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.issues.is_empty()
    }

    /// Return the number of findings.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Return the number of modules inspected.
    #[must_use]
    pub const fn modules_checked(&self) -> usize {
        self.modules_checked
    }

    /// Return the number of functions inspected.
    #[must_use]
    pub const fn functions_checked(&self) -> usize {
        self.functions_checked
    }

    /// Return the number of regions inspected.
    #[must_use]
    pub const fn regions_checked(&self) -> usize {
        self.regions_checked
    }

    /// Return the number of blocks inspected.
    #[must_use]
    pub const fn blocks_checked(&self) -> usize {
        self.blocks_checked
    }

    /// Return the number of operations inspected.
    #[must_use]
    pub const fn operations_checked(&self) -> usize {
        self.operations_checked
    }

    fn push(&mut self, code: &'static str, message: impl Into<String>) {
        self.issues.push(StructuralIssue::new(code, message));
    }
}

/// Complete set of canonical entities needed for structural verification.
///
/// The program owns its module/function tree. Regions, blocks, and operations
/// are supplied separately because the current IR represents their
/// relationships through typed references rather than one enclosing arena.
///
/// Every entity reachable through these references must be supplied.
/// Duplicate identities are reported instead of silently replacing entities.
pub struct StructuralInput<'a> {
    /// Program and canonical module/function hierarchy.
    pub program: &'a ZuirProgram<ZuirModule<ZuirFunction>>,

    /// All regions in the representation being verified.
    pub regions: &'a [ZuirRegion],

    /// All blocks in the representation being verified.
    pub blocks: &'a [ZuirBlock],

    /// All operations in the representation being verified.
    pub operations: &'a [ZuirOperation],
}

/// Verify structural integrity of the supplied representation.
///
/// This function does not mutate the IR. It traverses collections in their
/// supplied order and uses ordered indexes so that cross-reference findings
/// are deterministic for a given input.
///
/// Callers must not treat a successful result as authorization to execute
/// unknown dialect operations or as proof of semantic validity.
#[must_use]
pub fn verify_structural(input: &StructuralInput<'_>) -> StructuralReport {
    let mut report = StructuralReport::default();

    verify_program_tree(input.program, &mut report);

    let regions = index_regions(input.regions, &mut report);
    let blocks = index_blocks(input.blocks, &mut report);
    let operations = index_operations(input.operations, &mut report);

    verify_region_relationships(
        input,
        &regions,
        &blocks,
        &operations,
        &mut report,
    );

    verify_block_relationships(
        input,
        &regions,
        &blocks,
        &operations,
        &mut report,
    );

    verify_operation_relationships(
        input,
        &regions,
        &operations,
        &mut report,
    );

    report
}

// =============================================================================
// Identity indexes
// =============================================================================

fn index_regions<'a>(
    entities: &'a [ZuirRegion],
    report: &mut StructuralReport,
) -> BTreeMap<RegionId, &'a ZuirRegion> {
    let mut index = BTreeMap::new();

    for region in entities {
        report.regions_checked += 1;

        if index.insert(region.id(), region).is_some() {
            report.push(
                "ZUIR-STRUCT-DUPLICATE-REGION-ID",
                format!("region identity {} occurs more than once", region.id()),
            );
        }

        if let Err(error) = region.validate() {
            report.push(
                "ZUIR-STRUCT-INVALID-REGION",
                format!("region {} failed local validation: {error}", region.id()),
            );
        }
    }

    index
}

fn index_blocks<'a>(
    entities: &'a [ZuirBlock],
    report: &mut StructuralReport,
) -> BTreeMap<BlockId, &'a ZuirBlock> {
    let mut index = BTreeMap::new();

    for block in entities {
        report.blocks_checked += 1;

        if index.insert(block.id(), block).is_some() {
            report.push(
                "ZUIR-STRUCT-DUPLICATE-BLOCK-ID",
                format!("block identity {} occurs more than once", block.id()),
            );
        }

        if let Err(error) = block.validate() {
            report.push(
                "ZUIR-STRUCT-INVALID-BLOCK",
                format!("block {} failed local validation: {error}", block.id()),
            );
        }
    }

    index
}

fn index_operations<'a>(
    entities: &'a [ZuirOperation],
    report: &mut StructuralReport,
) -> BTreeMap<OperationId, &'a ZuirOperation> {
    let mut index = BTreeMap::new();

    for operation in entities {
        report.operations_checked += 1;

        if index.insert(operation.id(), operation).is_some() {
            report.push(
                "ZUIR-STRUCT-DUPLICATE-OPERATION-ID",
                format!(
                    "operation identity {} occurs more than once",
                    operation.id()
                ),
            );
        }

        if let Err(error) = operation.validate() {
            report.push(
                "ZUIR-STRUCT-INVALID-OPERATION",
                format!(
                    "operation {} failed local validation: {error}",
                    operation.id()
                ),
            );
        }
    }

    index
}

// =============================================================================
// Program, module, and function hierarchy
// =============================================================================

fn verify_program_tree(
    program: &ZuirProgram<ZuirModule<ZuirFunction>>,
    report: &mut StructuralReport,
) {
    if let Err(error) = program.validate() {
        report.push(
            "ZUIR-STRUCT-INVALID-PROGRAM",
            format!("program {} failed local validation: {error}", program.id()),
        );
    }

    let mut function_ids = BTreeSet::<FunctionId>::new();
    let mut module_names = BTreeMap::<String, ModuleId>::new();

    for (module_key, module) in program.modules() {
        report.modules_checked += 1;

        if module_key != module.id() {
            report.push(
                "ZUIR-STRUCT-MODULE-KEY-MISMATCH",
                format!(
                    "program module key {module_key} does not match contained module ID {}",
                    module.id()
                ),
            );
        }

        if let Err(error) = module.validate() {
            report.push(
                "ZUIR-STRUCT-INVALID-MODULE",
                format!("module {} failed local validation: {error}", module.id()),
            );
        }

        if let Some(previous) =
            module_names.insert(module.name().as_str().to_owned(), module.id())
        {
            report.push(
                "ZUIR-STRUCT-DUPLICATE-MODULE-NAME",
                format!(
                    "module name {} is shared by module IDs {} and {}",
                    module.name(),
                    previous,
                    module.id()
                ),
            );
        }

        for (function_key, function) in module.functions() {
            report.functions_checked += 1;

            if function_key != function.id() {
                report.push(
                    "ZUIR-STRUCT-FUNCTION-KEY-MISMATCH",
                    format!(
                        "module {} function key {function_key} does not match contained function ID {}",
                        module.id(),
                        function.id()
                    ),
                );
            }

            if !function_ids.insert(function.id()) {
                report.push(
                    "ZUIR-STRUCT-DUPLICATE-FUNCTION-ID",
                    format!(
                        "function identity {} appears in more than one declaration",
                        function.id()
                    ),
                );
            }

            if let Err(error) = function.validate() {
                report.push(
                    "ZUIR-STRUCT-INVALID-FUNCTION",
                    format!(
                        "function {} failed local validation: {error}",
                        function.id()
                    ),
                );
            }
        }
    }
}

// =============================================================================
// Region relationships
// =============================================================================

fn verify_region_relationships(
    input: &StructuralInput<'_>,
    regions: &BTreeMap<RegionId, &ZuirRegion>,
    blocks: &BTreeMap<BlockId, &ZuirBlock>,
    operations: &BTreeMap<OperationId, &ZuirOperation>,
    report: &mut StructuralReport,
) {
    for region in input.regions {
        match region.owner() {
            RegionOwner::Function(function_id) => {
                let found = input.program.modules().any(|(_, module)| {
                    module
                        .functions()
                        .any(|(_, function)| {
                            function.id() == function_id
                                && function.body().regions().contains(&region.id())
                        })
                });

                if !found {
                    report.push(
                        "ZUIR-STRUCT-ORPHAN-FUNCTION-REGION",
                        format!(
                            "region {} declares function owner {} but is not referenced by that function",
                            region.id(),
                            function_id
                        ),
                    );
                }
            }
            RegionOwner::Operation(operation_id) => {
                match operations.get(&operation_id) {
                    Some(operation) if operation.regions().contains(&region.id()) => {}
                    Some(_) => report.push(
                        "ZUIR-STRUCT-REGION-OWNER-MISMATCH",
                        format!(
                            "region {} declares operation owner {}, but that operation does not reference the region",
                            region.id(),
                            operation_id
                        ),
                    ),
                    None => report.push(
                        "ZUIR-STRUCT-MISSING-REGION-OWNER",
                        format!(
                            "region {} references missing owner operation {}",
                            region.id(),
                            operation_id
                        ),
                    ),
                }
            }
        }

        for block_id in region.blocks() {
            match blocks.get(block_id) {
                Some(block) if block.parent_region() == Some(region.id()) => {}
                Some(block) => report.push(
                    "ZUIR-STRUCT-BLOCK-REGION-MISMATCH",
                    format!(
                        "region {} references block {}, whose parent region is {:?}",
                        region.id(),
                        block.id(),
                        block.parent_region()
                    ),
                ),
                None => report.push(
                    "ZUIR-STRUCT-MISSING-BLOCK",
                    format!(
                        "region {} references missing block {}",
                        region.id(),
                        block_id
                    ),
                ),
            }
        }
    }

    // Check the reverse direction: each block must appear in its parent region.
    for block in input.blocks {
        match block.parent_region() {
            Some(region_id) => match regions.get(&region_id) {
                Some(region) if region.blocks().contains(&block.id()) => {}
                Some(_) => report.push(
                    "ZUIR-STRUCT-REGION-OMITS-BLOCK",
                    format!(
                        "block {} declares parent region {}, but that region does not list the block",
                        block.id(),
                        region_id
                    ),
                ),
                None => report.push(
                    "ZUIR-STRUCT-MISSING-PARENT-REGION",
                    format!(
                        "block {} references missing parent region {}",
                        block.id(),
                        region_id
                    ),
                ),
            },
            None => report.push(
                "ZUIR-STRUCT-UNOWNED-BLOCK",
                format!("block {} has no parent region", block.id()),
            ),
        }
    }
}

// =============================================================================
// Block and control-flow relationships
// =============================================================================

fn verify_block_relationships(
    input: &StructuralInput<'_>,
    regions: &BTreeMap<RegionId, &ZuirRegion>,
    blocks: &BTreeMap<BlockId, &ZuirBlock>,
    operations: &BTreeMap<OperationId, &ZuirOperation>,
    report: &mut StructuralReport,
) {
    let mut operation_membership = BTreeMap::<OperationId, BlockId>::new();

    for block in input.blocks {
        for operation_id in block.operations() {
            if let Some(previous_block) =
                operation_membership.insert(*operation_id, block.id())
            {
                report.push(
                    "ZUIR-STRUCT-OPERATION-MULTIPLE-BLOCKS",
                    format!(
                        "operation {} is listed in blocks {} and {}",
                        operation_id, previous_block, block.id()
                    ),
                );
            }

            match operations.get(operation_id) {
                Some(_) => {}
                None => report.push(
                    "ZUIR-STRUCT-MISSING-OPERATION",
                    format!(
                        "block {} references missing operation {}",
                        block.id(),
                        operation_id
                    ),
                ),
            }
        }

        if let Err(error) = block.validate_complete() {
            report.push(
                "ZUIR-STRUCT-INCOMPLETE-BLOCK",
                format!(
                    "block {} is not structurally complete: {error}",
                    block.id()
                ),
            );
        }

        block.terminator().for_each_successor(|successor| {
            match blocks.get(&successor.target()) {
                Some(target) => {
                    let source_region = block.parent_region();
                    let target_region = target.parent_region();

                    if source_region != target_region {
                        report.push(
                            "ZUIR-STRUCT-CROSS-REGION-BRANCH",
                            format!(
                                "block {} branches to block {} across region boundaries",
                                block.id(),
                                target.id()
                            ),
                        );
                    }

                    if let Some(target_region_id) = target_region {
                        if !regions.contains_key(&target_region_id) {
                            report.push(
                                "ZUIR-STRUCT-MISSING-TARGET-REGION",
                                format!(
                                    "branch from block {} targets block {} whose region {} is absent",
                                    block.id(),
                                    target.id(),
                                    target_region_id
                                ),
                            );
                        }
                    }

                    if successor.argument_count() != target.argument_count() {
                        report.push(
                            "ZUIR-STRUCT-BRANCH-ARGUMENT-COUNT",
                            format!(
                                "branch from block {} to block {} supplies {} arguments, but the target declares {}",
                                block.id(),
                                target.id(),
                                successor.argument_count(),
                                target.argument_count()
                            ),
                        );
                    }
                }
                None => report.push(
                    "ZUIR-STRUCT-MISSING-BRANCH-TARGET",
                    format!(
                        "terminator in block {} targets missing block {}",
                        block.id(),
                        successor.target()
                    ),
                ),
            }
        });
    }

    // Every operation in the supplied complete view must have exactly one
    // containing block. This is structural containment, not execution order.
    for operation in input.operations {
        if !operation_membership.contains_key(&operation.id()) {
            report.push(
                "ZUIR-STRUCT-ORPHAN-OPERATION",
                format!(
                    "operation {} is not contained in any block",
                    operation.id()
                ),
            );
        }
    }
}

// =============================================================================
// Operation and nested-region relationships
// =============================================================================

fn verify_operation_relationships(
    input: &StructuralInput<'_>,
    regions: &BTreeMap<RegionId, &ZuirRegion>,
    operations: &BTreeMap<OperationId, &ZuirOperation>,
    report: &mut StructuralReport,
) {
    for operation in input.operations {
        let mut seen_regions = BTreeSet::new();

        for region_id in operation.regions() {
            if !seen_regions.insert(*region_id) {
                report.push(
                    "ZUIR-STRUCT-DUPLICATE-NESTED-REGION",
                    format!(
                        "operation {} references region {} more than once",
                        operation.id(),
                        region_id
                    ),
                );
            }

            match regions.get(region_id) {
                Some(region) => {
                    if region.owner() != RegionOwner::Operation(operation.id()) {
                        report.push(
                            "ZUIR-STRUCT-NESTED-REGION-OWNER-MISMATCH",
                            format!(
                                "operation {} references region {}, but its declared owner is {:?}",
                                operation.id(),
                                region_id,
                                region.owner()
                            ),
                        );
                    }
                }
                None => report.push(
                    "ZUIR-STRUCT-MISSING-NESTED-REGION",
                    format!(
                        "operation {} references missing region {}",
                        operation.id(),
                        region_id
                    ),
                ),
            }
        }
    }

    // A region owned by an operation must be listed by that operation.
    // The reverse relationship is checked above.
    for region in input.regions {
        if let RegionOwner::Operation(owner_id) = region.owner() {
            if !operations.contains_key(&owner_id) {
                report.push(
                    "ZUIR-STRUCT-MISSING-REGION-OWNER",
                    format!(
                        "region {} is owned by missing operation {}",
                        region.id(),
                        owner_id
                    ),
                );
            }
        }
    }
}

// =============================================================================
// Scope and limitations
// =============================================================================

/// Return whether a block terminator is still in its construction state.
///
/// This helper deliberately does not decide whether an extension-defined
/// terminator is semantically legal. That decision belongs to its registered
/// dialect contract.
#[must_use]
pub fn is_open_terminator(terminator: &BlockTerminator) -> bool {
    terminator.is_open()
}
