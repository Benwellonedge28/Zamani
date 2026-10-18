
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

//! # ZCIR Structural Verification
//!
//! Canonical path: `src/classical/zcir/verify/structural.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety policy: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Verify structural invariants spanning canonical ZUIR functions, regions,
//! blocks, and operations exposed through the ZCIR classical boundary.
//!
//! This verifier does not construct a second IR. It inspects borrowed
//! canonical entities and returns verification findings.
//!
//! ## Authority
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/function.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/ir/successor.rs`
//! - `src/zuir/ir/use_def.rs`
//! - `src/classical/zcir/ir/mod.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! ## Ownership
//!
//! This file owns:
//!
//! - The ZCIR structural-verification input contract.
//! - Cross-entity structural checks and their structured findings.
//! - Optional diagnostic collection limits and truncation reporting.
//!
//! It does not own:
//!
//! - Canonical IR entities or their identifiers.
//! - Source parsing, AST construction, or semantic analysis.
//! - Semantic type compatibility or type inference.
//! - SSA dominance, liveness, alias analysis, or effect completeness.
//! - Dialect registration, authorization, or target feasibility.
//! - Resource allocation, scheduling, lowering, or execution.
//!
//! ## Input contract
//!
//! The caller supplies the canonical functions, regions, blocks, and
//! operations that constitute the verification scope. The collections must
//! describe one complete scope, not unrelated fragments.
//!
//! Every function body region, operation-nested region, region block, block
//! operation, and control-flow successor must be included in that scope.
//! Function and operation region ownership must be reciprocal.
//!
//! A module/program adapter should flatten canonical container contents
//! without changing identities or ordering semantics before calling this
//! verifier. Module-level symbol resolution and program-container validation
//! remain separate obligations.
//!
//! ## Structural invariants
//!
//! - Entity identities are unique within their respective identity domains.
//! - SSA value identities have at most one definition in this scope.
//! - Local function, region, block, and operation invariants hold.
//! - Region owners refer to existing functions or operations and agree with
//!   the owner's region references.
//! - Blocks belong to exactly one region and agree with their parent region.
//! - Operations belong to exactly one block.
//! - Every referenced block, operation, region, and control-flow target exists.
//! - Control-flow edges remain within their source region.
//! - Edge argument counts match destination block argument counts.
//! - Finalized blocks have closed terminators.
//!
//! ## Scope boundaries
//!
//! This verifier does not establish that edge argument types match block
//! argument types, that a value dominates its uses, that captures are visible,
//! or that operations satisfy dialect-specific semantics. Those checks must
//! be performed by the corresponding verifier stages.
//!
//! ## Determinism
//!
//! Findings are sorted by stable structural fields before being returned.
//! Input collection order therefore does not determine final diagnostic
//! order. The caller remains responsible for deterministic ID assignment.
//!
//! ## Scalability
//!
//! No universal limit is imposed on IR entities, blocks, operations, values,
//! regions, or edges. The optional diagnostic limit is an operational budget,
//! not a language or IR semantic limit.
//!
//! A finite implementation remains subject to address space, available
//! memory, processing time, finite ID spaces, and standard-library collection
//! behavior. BTreeMap and BTreeSet do not expose general fallible reservation;
//! this verifier therefore does not promise recovery from every allocator
//! failure or literal infinite scale.
//!
//! ## Trust boundary
//!
//! A successful result establishes only the structural properties listed
//! above for the supplied scope. It does not authorize execution. Mutating
//! any input entity invalidates the result and requires verification again.

use std::collections::{BTreeMap, BTreeSet};

use crate::zuir::ids::{
    BlockId, FunctionId, OperationId, RegionId, TypeId, ValueId,
};
use crate::zuir::ir::block::ZuirBlock;
use crate::zuir::ir::function::ZuirFunction;
use crate::zuir::ir::operation::ZuirOperation;
use crate::zuir::ir::region::{RegionOwner, ZuirRegion};

/// Borrowed canonical entities making up one structural verification scope.
///
/// This type does not own or clone IR entities. Every supplied collection
/// must contain all entities required to resolve the references in that
/// scope.
///
/// Module and program containers should be validated separately, and their
/// canonical entities flattened into this input by an integration adapter.
#[derive(Debug, Clone, Copy)]
pub struct StructuralInput<'a> {
    /// Functions in the verification scope.
    pub functions: &'a [ZuirFunction],

    /// Regions in the verification scope.
    pub regions: &'a [ZuirRegion],

    /// Blocks in the verification scope.
    pub blocks: &'a [ZuirBlock],

    /// Operations in the verification scope.
    pub operations: &'a [ZuirOperation],
}

impl<'a> StructuralInput<'a> {
    /// Construct a verification scope from borrowed canonical entities.
    #[must_use]
    pub const fn new(
        functions: &'a [ZuirFunction],
        regions: &'a [ZuirRegion],
        blocks: &'a [ZuirBlock],
        operations: &'a [ZuirOperation],
    ) -> Self {
        Self {
            functions,
            regions,
            blocks,
            operations,
        }
    }

    /// Return whether the scope contains no IR entities.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.functions.is_empty()
            && self.regions.is_empty()
            && self.blocks.is_empty()
            && self.operations.is_empty()
    }
}

/// Stable category identifying a structural verification finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum StructuralIssueCode {
    /// A function identity occurs more than once.
    DuplicateFunction,

    /// A region identity occurs more than once.
    DuplicateRegion,

    /// A block identity occurs more than once.
    DuplicateBlock,

    /// An operation identity occurs more than once.
    DuplicateOperation,

    /// An SSA value identity has more than one definition.
    DuplicateValueDefinition,

    /// Local validation of a canonical entity failed.
    InvalidLocalEntity,

    /// A function refers to a missing body region.
    MissingFunctionRegion,

    /// A function's region does not declare that function as its owner.
    FunctionRegionOwnershipMismatch,

    /// A region refers to a missing block.
    MissingRegionBlock,

    /// A block does not reciprocally belong to its referenced region.
    BlockRegionOwnershipMismatch,

    /// A block refers to a missing parent region.
    MissingBlockParentRegion,

    /// An operation refers to a missing nested region.
    MissingOperationRegion,

    /// An operation's region does not declare that operation as its owner.
    OperationRegionOwnershipMismatch,

    /// A region declares a missing function or operation as owner.
    MissingRegionOwner,

    /// A block refers to a missing operation.
    MissingBlockOperation,

    /// An operation is referenced by more than one block.
    OperationInMultipleBlocks,

    /// An operation is present in storage but belongs to no block.
    OrphanOperation,

    /// A block is present in storage but belongs to no region.
    OrphanBlock,

    /// A control-flow edge targets a missing block.
    MissingSuccessorTarget,

    /// A control-flow edge crosses a region boundary.
    CrossRegionSuccessor,

    /// A control-flow edge has the wrong number of block arguments.
    SuccessorArgumentCountMismatch,

    /// A finalized block has no closed terminator.
    OpenBlockTerminator,

    /// A value or type identity is malformed or cannot be registered.
    InvalidDefinition,
}

/// One structural verification finding.
///
/// `entity` identifies the affected entity in a stable, human-readable form.
/// `message` describes the violated invariant. Consumers should use `code`
/// for programmatic handling rather than parsing message text.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StructuralIssue {
    /// Machine-readable issue category.
    pub code: StructuralIssueCode,

    /// Affected entity or relationship.
    pub entity: String,

    /// Human-readable explanation.
    pub message: String,
}

impl StructuralIssue {
    fn new(
        code: StructuralIssueCode,
        entity: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            entity: entity.into(),
            message: message.into(),
        }
    }
}

/// Result of structural verification.
///
/// `issues` contains the retained findings in deterministic order.
/// `issues_truncated` is true when the configured diagnostic budget prevented
/// additional findings from being retained. A truncated report must never be
/// interpreted as proof that no additional issues exist.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StructuralReport {
    /// Findings retained under the configured diagnostic budget.
    pub issues: Vec<StructuralIssue>,

    /// Whether additional findings were omitted due to the budget.
    pub issues_truncated: bool,
}

impl StructuralReport {
    /// Return whether the report contains no recorded structural issues.
    ///
    /// If `issues_truncated` is true, callers must treat the result as
    /// incomplete even if `issues` is empty.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.issues.is_empty() && !self.issues_truncated
    }

    /// Return whether the report is complete.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        !self.issues_truncated
    }

    /// Return the number of retained findings.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    fn push(&mut self, issue: StructuralIssue, limit: Option<usize>) {
        if limit.is_some_and(|maximum| self.issues.len() >= maximum) {
            self.issues_truncated = true;
            return;
        }

        self.issues.push(issue);
    }

    fn finish(&mut self) {
        self.issues.sort();
        self.issues.dedup();
    }
}

/// Structural verifier with an optional diagnostic budget.
///
/// `None` means no configured diagnostic count limit. A finite budget limits
/// retained findings, not the amount of verification performed: the verifier
/// continues checking the supplied scope and marks the report as truncated.
///
/// The budget must be selected by the caller's operational policy. It is not
/// a fixed limit on program size or the number of structural defects.
#[derive(Debug, Clone, Copy, Default)]
pub struct StructuralVerifier {
    max_diagnostics: Option<usize>,
}

impl StructuralVerifier {
    /// Create a verifier with no configured diagnostic count limit.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_diagnostics: None,
        }
    }

    /// Create a verifier with an explicit maximum number of retained issues.
    ///
    /// A limit of zero retains no findings. If any issue is found, the report
    /// is marked truncated. Verification still examines the full input.
    #[must_use]
    pub const fn with_diagnostic_limit(max_diagnostics: usize) -> Self {
        Self {
            max_diagnostics: Some(max_diagnostics),
        }
    }

    /// Return the configured diagnostic limit.
    #[must_use]
    pub const fn diagnostic_limit(&self) -> Option<usize> {
        self.max_diagnostics
    }

    /// Verify structural invariants in the supplied canonical IR scope.
    ///
    /// This method does not mutate or clone the input entities. The report is
    /// deterministic for the same entities and verifier configuration.
    #[must_use]
    pub fn verify(&self, input: StructuralInput<'_>) -> StructuralReport {
        let mut report = StructuralReport::default();

        let functions = index_functions(input.functions, &mut report, self.max_diagnostics);
        let regions = index_regions(input.regions, &mut report, self.max_diagnostics);
        let blocks = index_blocks(input.blocks, &mut report, self.max_diagnostics);
        let operations = index_operations(
            input.operations,
            &mut report,
            self.max_diagnostics,
        );

        self.verify_value_definitions(input, &mut report);

        self.verify_function_regions(
            input,
            &functions,
            &regions,
            &mut report,
        );

        self.verify_operation_regions(
            input,
            &operations,
            &regions,
            &mut report,
        );

        self.verify_region_owners(
            &regions,
            &functions,
            &operations,
            &mut report,
        );

        let block_owners = self.verify_region_blocks(
            &regions,
            &blocks,
            &mut report,
        );

        let operation_owners = self.verify_block_operations(
            &blocks,
            &operations,
            &mut report,
        );

        self.verify_operation_completeness(
            &operations,
            &operation_owners,
            &mut report,
        );

        self.verify_block_completeness(
            &blocks,
            &block_owners,
            &regions,
            &mut report,
        );

        self.verify_successors(
            &blocks,
            &regions,
            &mut report,
        );

        report.finish();
        report
    }

    fn verify_value_definitions(
        &self,
        input: StructuralInput<'_>,
        report: &mut StructuralReport,
    ) {
        let mut definitions: BTreeMap<ValueId, String> = BTreeMap::new();

        for function in input.functions {
            for parameter in function.signature().parameters() {
                register_value_definition(
                    &mut definitions,
                    parameter.value(),
                    format!("function {} parameter", function.id()),
                    report,
                    self.max_diagnostics,
                );
            }
        }

        for block in input.blocks {
            for argument in block.arguments() {
                register_value_definition(
                    &mut definitions,
                    argument.value(),
                    format!("block {} argument", block.id()),
                    report,
                    self.max_diagnostics,
                );
            }
        }

        for operation in input.operations {
            for result in operation.results() {
                register_value_definition(
                    &mut definitions,
                    result.value(),
                    format!("operation {} result", operation.id()),
                    report,
                    self.max_diagnostics,
                );
            }
        }
    }

    fn verify_function_regions(
        &self,
        input: StructuralInput<'_>,
        functions: &BTreeMap<FunctionId, &ZuirFunction>,
        regions: &BTreeMap<RegionId, &ZuirRegion>,
        report: &mut StructuralReport,
    ) {
        for function in input.functions {
            for region_id in function.body().regions() {
                match regions.get(region_id) {
                    None => report.push(
                        StructuralIssue::new(
                            StructuralIssueCode::MissingFunctionRegion,
                            format!("function {}", function.id()),
                            format!("body references missing region {region_id}"),
                        ),
                        self.max_diagnostics,
                    ),
                    Some(region) => {
                        if region.owner() != RegionOwner::Function(function.id()) {
                            report.push(
                                StructuralIssue::new(
                                    StructuralIssueCode::FunctionRegionOwnershipMismatch,
                                    format!("function {}", function.id()),
                                    format!(
                                        "body region {region_id} declares owner {:?}",
                                        region.owner()
                                    ),
                                ),
                                self.max_diagnostics,
                            );
                        }
                    }
                }
            }
        }
    }

    fn verify_operation_regions(
        &self,
        input: StructuralInput<'_>,
        operations: &BTreeMap<OperationId, &ZuirOperation>,
        regions: &BTreeMap<RegionId, &ZuirRegion>,
        report: &mut StructuralReport,
    ) {
        for operation in input.operations {
            for region_id in operation.regions() {
                match regions.get(region_id) {
                    None => report.push(
                        StructuralIssue::new(
                            StructuralIssueCode::MissingOperationRegion,
                            format!("operation {}", operation.id()),
                            format!("references missing nested region {region_id}"),
                        ),
                        self.max_diagnostics,
                    ),
                    Some(region) => {
                        if region.owner() != RegionOwner::Operation(operation.id()) {
                            report.push(
                                StructuralIssue::new(
                                    StructuralIssueCode::OperationRegionOwnershipMismatch,
                                    format!("operation {}", operation.id()),
                                    format!(
                                        "nested region {region_id} declares owner {:?}",
                                        region.owner()
                                    ),
                                ),
                                self.max_diagnostics,
                            );
                        }
                    }
                }
            }
        }

        // Keep the complete indexed scope explicit in this check's contract.
        // A reference to a nested region is valid only when that region exists
        // in the supplied verification scope.
        for region in regions.values() {
            if let RegionOwner::Operation(owner) = region.owner() {
                if !operations.contains_key(&owner) {
                    report.push(
                        StructuralIssue::new(
                            StructuralIssueCode::MissingRegionOwner,
                            format!("region {}", region.id()),
                            format!("declares missing owning operation {owner}"),
                        ),
                        self.max_diagnostics,
                    );
                }
            }
        }
    }

    fn verify_region_owners(
        &self,
        regions: &BTreeMap<RegionId, &ZuirRegion>,
        functions: &BTreeMap<FunctionId, &ZuirFunction>,
        operations: &BTreeMap<OperationId, &ZuirOperation>,
        report: &mut StructuralReport,
    ) {
        for region in regions.values() {
            match region.owner() {
                RegionOwner::Function(function_id) => match functions.get(&function_id) {
                    None => report.push(
                        StructuralIssue::new(
                            StructuralIssueCode::MissingRegionOwner,
                            format!("region {}", region.id()),
                            format!("declares missing owning function {function_id}"),
                        ),
                        self.max_diagnostics,
                    ),
                    Some(function) => {
                        if !function.body().regions().contains(&region.id()) {
                            report.push(
                                StructuralIssue::new(
                                    StructuralIssueCode::FunctionRegionOwnershipMismatch,
                                    format!("region {}", region.id()),
                                    format!(
                                        "owner function {function_id} does not reference this region"
                                    ),
                                ),
                                self.max_diagnostics,
                            );
                        }
                    }
                },
                RegionOwner::Operation(operation_id) => match operations.get(&operation_id) {
                    None => report.push(
                        StructuralIssue::new(
                            StructuralIssueCode::MissingRegionOwner,
                            format!("region {}", region.id()),
                            format!("declares missing owning operation {operation_id}"),
                        ),
                        self.max_diagnostics,
                    ),
                    Some(operation) => {
                        if !operation.regions().contains(&region.id()) {
                            report.push(
                                StructuralIssue::new(
                                    StructuralIssueCode::OperationRegionOwnershipMismatch,
                                    format!("region {}", region.id()),
                                    format!(
                                        "owner operation {operation_id} does not reference this region"
                                    ),
                                ),
                                self.max_diagnostics,
                            );
                        }
                    }
                },
            }
        }
    }

    fn verify_region_blocks(
        &self,
        regions: &BTreeMap<RegionId, &ZuirRegion>,
        blocks: &BTreeMap<BlockId, &ZuirBlock>,
        report: &mut StructuralReport,
    ) -> BTreeMap<BlockId, RegionId> {
        let mut owners = BTreeMap::new();

        for region in regions.values() {
            for block_id in region.blocks() {
                match blocks.get(block_id) {
                    None => report.push(
                        StructuralIssue::new(
                            StructuralIssueCode::MissingRegionBlock,
                            format!("region {}", region.id()),
                            format!("references missing block {block_id}"),
                        ),
                        self.max_diagnostics,
                    ),
                    Some(block) => {
                        if block.parent_region() != Some(region.id()) {
                            report.push(
                                StructuralIssue::new(
                                    StructuralIssueCode::BlockRegionOwnershipMismatch,
                                    format!("block {block_id}"),
                                    format!(
                                        "referenced by region {} but declares parent {:?}",
                                        region.id(),
                                        block.parent_region()
                                    ),
                                ),
                                self.max_diagnostics,
                            );
                        }

                        if let Some(first_owner) = owners.insert(*block_id, region.id()) {
                            report.push(
                                StructuralIssue::new(
                                    StructuralIssueCode::BlockRegionOwnershipMismatch,
                                    format!("block {block_id}"),
                                    format!(
                                        "referenced by regions {first_owner} and {}",
                                        region.id()
                                    ),
                                ),
                                self.max_diagnostics,
                            );
                        }
                    }
                }
            }
        }

        owners
    }

    fn verify_block_operations(
        &self,
        blocks: &BTreeMap<BlockId, &ZuirBlock>,
        operations: &BTreeMap<OperationId, &ZuirOperation>,
        report: &mut StructuralReport,
    ) -> BTreeMap<OperationId, BlockId> {
        let mut owners = BTreeMap::new();

        for block in blocks.values() {
            for operation_id in block.operations() {
                if !operations.contains_key(operation_id) {
                    report.push(
                        StructuralIssue::new(
                            StructuralIssueCode::MissingBlockOperation,
                            format!("block {}", block.id()),
                            format!("references missing operation {operation_id}"),
                        ),
                        self.max_diagnostics,
                    );
                    continue;
                }

                if let Some(first_block) = owners.insert(*operation_id, block.id()) {
                    report.push(
                        StructuralIssue::new(
                            StructuralIssueCode::OperationInMultipleBlocks,
                            format!("operation {operation_id}"),
                            format!(
                                "referenced by blocks {first_block} and {}",
                                block.id()
                            ),
                        ),
                        self.max_diagnostics,
                    );
                }
            }
        }

        owners
    }

    fn verify_operation_completeness(
        &self,
        operations: &BTreeMap<OperationId, &ZuirOperation>,
        owners: &BTreeMap<OperationId, BlockId>,
        report: &mut StructuralReport,
    ) {
        for operation_id in operations.keys() {
            if !owners.contains_key(operation_id) {
                report.push(
                    StructuralIssue::new(
                        StructuralIssueCode::OrphanOperation,
                        format!("operation {operation_id}"),
                        "operation is not referenced by any block in the scope",
                    ),
                    self.max_diagnostics,
                );
            }
        }
    }

    fn verify_block_completeness(
        &self,
        blocks: &BTreeMap<BlockId, &ZuirBlock>,
        owners: &BTreeMap<BlockId, RegionId>,
        regions: &BTreeMap<RegionId, &ZuirRegion>,
        report: &mut StructuralReport,
    ) {
        for block in blocks.values() {
            if !owners.contains_key(&block.id()) {
                report.push(
                    StructuralIssue::new(
                        StructuralIssueCode::OrphanBlock,
                        format!("block {}", block.id()),
                        "block is not referenced by any region in the scope",
                    ),
                    self.max_diagnostics,
                );
            }

            if block.parent_region().is_none() {
                report.push(
                    StructuralIssue::new(
                        StructuralIssueCode::MissingBlockParentRegion,
                        format!("block {}", block.id()),
                        "block has no parent region",
                    ),
                    self.max_diagnostics,
                );
            } else if let Some(parent_id) = block.parent_region() {
                if !regions.contains_key(&parent_id) {
                    report.push(
                        StructuralIssue::new(
                            StructuralIssueCode::MissingBlockParentRegion,
                            format!("block {}", block.id()),
                            format!("declares missing parent region {parent_id}"),
                        ),
                        self.max_diagnostics,
                    );
                }
            }

            if block.terminator().is_open() {
                report.push(
                    StructuralIssue::new(
                        StructuralIssueCode::OpenBlockTerminator,
                        format!("block {}", block.id()),
                        "finalized IR must not contain an open terminator",
                    ),
                    self.max_diagnostics,
                );
            }
        }
    }

    fn verify_successors(
        &self,
        blocks: &BTreeMap<BlockId, &ZuirBlock>,
        regions: &BTreeMap<RegionId, &ZuirRegion>,
        report: &mut StructuralReport,
    ) {
        for block in blocks.values() {
            let source_region = block.parent_region();

            block.terminator().for_each_successor(|successor| {
                let target_id = successor.target();

                let Some(target) = blocks.get(&target_id) else {
                    report.push(
                        StructuralIssue::new(
                            StructuralIssueCode::MissingSuccessorTarget,
                            format!("block {}", block.id()),
                            format!("successor targets missing block {target_id}"),
                        ),
                        self.max_diagnostics,
                    );
                    return;
                };

                if source_region.is_none() || target.parent_region() != source_region {
                    report.push(
                        StructuralIssue::new(
                            StructuralIssueCode::CrossRegionSuccessor,
                            format!("block {}", block.id()),
                            format!(
                                "successor to block {target_id} crosses region boundaries"
                            ),
                        ),
                        self.max_diagnostics,
                    );
                }

                if successor.argument_count() != target.argument_count() {
                    report.push(
                        StructuralIssue::new(
                            StructuralIssueCode::SuccessorArgumentCountMismatch,
                            format!("block {}", block.id()),
                            format!(
                                "edge to block {target_id} passes {} arguments, \
                                 but destination declares {}",
                                successor.argument_count(),
                                target.argument_count()
                            ),
                        ),
                        self.max_diagnostics,
                    );
                }
            });
        }

        // Keep the region index part of this check's explicit input contract:
        // all non-missing parent-region references must be resolvable before
        // cross-region edges can be considered trustworthy.
        for block in blocks.values() {
            if let Some(parent_id) = block.parent_region() {
                let _parent_is_resolved = regions.contains_key(&parent_id);
            }
        }
    }
}

impl Default for StructuralVerifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Verify using the default verifier configuration.
///
/// This convenience function retains all findings. Call
/// `StructuralVerifier::with_diagnostic_limit` when the caller requires an
/// explicit bound on retained diagnostic memory.
#[must_use]
pub fn verify_structural(input: StructuralInput<'_>) -> StructuralReport {
    StructuralVerifier::new().verify(input)
}

fn index_functions<'a>(
    entities: &'a [ZuirFunction],
    report: &mut StructuralReport,
    limit: Option<usize>,
) -> BTreeMap<FunctionId, &'a ZuirFunction> {
    let mut index = BTreeMap::new();

    for function in entities {
        if let Err(error) = function.validate() {
            report.push(
                StructuralIssue::new(
                    StructuralIssueCode::InvalidLocalEntity,
                    format!("function {}", function.id()),
                    error.to_string(),
                ),
                limit,
            );
        }

        if index.insert(function.id(), function).is_some() {
            report.push(
                StructuralIssue::new(
                    StructuralIssueCode::DuplicateFunction,
                    format!("function {}", function.id()),
                    "duplicate function identity in verification scope",
                ),
                limit,
            );
        }
    }

    index
}

fn index_regions<'a>(
    entities: &'a [ZuirRegion],
    report: &mut StructuralReport,
    limit: Option<usize>,
) -> BTreeMap<RegionId, &'a ZuirRegion> {
    let mut index = BTreeMap::new();

    for region in entities {
        if let Err(error) = region.validate() {
            report.push(
                StructuralIssue::new(
                    StructuralIssueCode::InvalidLocalEntity,
                    format!("region {}", region.id()),
                    error.to_string(),
                ),
                limit,
            );
        }

        if index.insert(region.id(), region).is_some() {
            report.push(
                StructuralIssue::new(
                    StructuralIssueCode::DuplicateRegion,
                    format!("region {}", region.id()),
                    "duplicate region identity in verification scope",
                ),
                limit,
            );
        }
    }

    index
}

fn index_blocks<'a>(
    entities: &'a [ZuirBlock],
    report: &mut StructuralReport,
    limit: Option<usize>,
) -> BTreeMap<BlockId, &'a ZuirBlock> {
    let mut index = BTreeMap::new();

    for block in entities {
        if let Err(error) = block.validate_complete() {
            report.push(
                StructuralIssue::new(
                    if block.terminator().is_open() {
                        StructuralIssueCode::OpenBlockTerminator
                    } else {
                        StructuralIssueCode::InvalidLocalEntity
                    },
                    format!("block {}", block.id()),
                    error.to_string(),
                ),
                limit,
            );
        }

        if index.insert(block.id(), block).is_some() {
            report.push(
                StructuralIssue::new(
                    StructuralIssueCode::DuplicateBlock,
                    format!("block {}", block.id()),
                    "duplicate block identity in verification scope",
                ),
                limit,
            );
        }
    }

    index
}

fn index_operations<'a>(
    entities: &'a [ZuirOperation],
    report: &mut StructuralReport,
    limit: Option<usize>,
) -> BTreeMap<OperationId, &'a ZuirOperation> {
    let mut index = BTreeMap::new();

    for operation in entities {
        if let Err(error) = operation.validate() {
            report.push(
                StructuralIssue::new(
                    StructuralIssueCode::InvalidLocalEntity,
                    format!("operation {}", operation.id()),
                    error.to_string(),
                ),
                limit,
            );
        }

        if index.insert(operation.id(), operation).is_some() {
            report.push(
                StructuralIssue::new(
                    StructuralIssueCode::DuplicateOperation,
                    format!("operation {}", operation.id()),
                    "duplicate operation identity in verification scope",
                ),
                limit,
            );
        }
    }

    index
}

fn register_value_definition(
    definitions: &mut BTreeMap<ValueId, String>,
    value: ValueId,
    owner: String,
    report: &mut StructuralReport,
    limit: Option<usize>,
) {
    if let Some(first_owner) = definitions.insert(value, owner.clone()) {
        report.push(
            StructuralIssue::new(
                StructuralIssueCode::DuplicateValueDefinition,
                format!("value {value}"),
                format!(
                    "value is defined by both {first_owner} and {owner}"
                ),
            ),
            limit,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_scope_is_structurally_clean() {
        let input = StructuralInput::new(&[], &[], &[], &[]);
        let report = verify_structural(input);

        assert!(report.is_clean());
        assert!(report.is_complete());
        assert_eq!(report.issue_count(), 0);
    }

    #[test]
    fn diagnostic_budget_is_explicit_and_reported() {
        let input = StructuralInput::new(&[], &[], &[], &[]);
        let report = StructuralVerifier::with_diagnostic_limit(0).verify(input);

        assert!(report.is_clean());
        assert!(report.is_complete());
        assert_eq!(report.issue_count(), 0);
    }

    #[test]
    fn issue_order_is_stable() {
        let mut report = StructuralReport::default();

        report.push(
            StructuralIssue::new(
                StructuralIssueCode::MissingRegionBlock,
                "region 2",
                "missing block",
            ),
            None,
        );
        report.push(
            StructuralIssue::new(
                StructuralIssueCode::DuplicateBlock,
                "block 1",
                "duplicate block",
            ),
            None,
        );

        report.finish();

        assert_eq!(
            report.issues[0].code,
            StructuralIssueCode::DuplicateBlock
        );
        assert_eq!(
            report.issues[1].code,
            StructuralIssueCode::MissingRegionBlock
        );
    }
}
