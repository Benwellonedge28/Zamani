
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Effect Verification
//!
//! Path: `src/zuir/verify/effects.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.1 or later.
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Verify effect declarations across the canonical ZUIR program,
//! including operation effects, function effect contracts, nested
//! regions, and effect-definition registration.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ir/effects.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/ir/function.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/program.rs`
//! - `src/zuir/ir/module.rs`
//! - `src/zuir/dialect/registry.rs`
//! - `src/zuir/schema/types.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/grammar.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/domains.md`
//!
//! OWNS
//!
//! - Whole-program validation of operation and function effect names.
//! - Validation of registered effect definitions supplied by the caller.
//! - Checking that a function's declared effects cover the effects declared
//!   by operations reachable through its body-region structure.
//! - Detection of missing effect-analysis entities and duplicate identities
//!   in the supplied verification view.
//! - Deterministic effect-verification findings.
//!
//! DOES_NOT_OWN
//!
//! - Source parsing, AST construction, name resolution, or effect inference
//!   in the frontend.
//! - The canonical effect types or their policies.
//! - Dialect registration or trust decisions for executable extension code.
//! - Structural verification, SSA dominance, type verification, ownership,
//!   resource feasibility, or control-flow legality.
//! - Inferring effects of function calls without an authoritative call
//!   resolution and interprocedural summary.
//! - Proving that an implementation's declared effects match its behavior.
//! - Runtime effect enforcement, target selection, or lowering.
//!
//! SEMANTIC RULES
//!
//! - Unknown effects are errors; they are never presumed harmless.
//! - An empty effect set does not prove purity.
//! - A function declaration must cover effects declared by operations in
//!   its body regions, including nested operation-owned regions.
//! - Function effect coverage is a conservative declaration check, not a
//!   complete interprocedural effect analysis.
//! - Effect sets do not encode ordering. Ordering belongs to explicit
//!   dependencies, control flow, and the relevant dialect semantics.
//! - A successful report does not replace structural, type, dialect,
//!   ownership, or full-program verification.
//!
//! SCALABILITY
//!
//! No universal operation, effect, block, region, qubit, processor, device,
//! tensor-rank, node, or topology limit is introduced.
//! Indexes and worklists grow with the supplied IR. Traversal uses an
//! explicit worklist to avoid recursion proportional to region nesting.
//! Actual execution remains bounded by memory, address space, identifier
//! representation, and the caller's configured compilation budgets.
//!
//! INTEGRATION
//!
//! The caller supplies a complete program, all regions, blocks, operations,
//! and a canonical map of registered effect definitions.
//! Run structural verification before relying on region ownership or
//! containment. Run dialect verification to establish that operation effect
//! declarations are permitted by their operation schemas.
//! Run type, ownership, and other verification stages independently.
//!
//! FREEZE CRITERIA
//!
//! - Uses the canonical ZUIR entity and effect types.
//! - Does not mutate IR entities.
//! - Unknown effects and missing references are reported explicitly.
//! - Nested regions are traversed without recursive function calls.
//! - Findings are deterministic for a given ordered input.
//! - No unsafe code or fixed hardware-size assumptions are introduced.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::ids::{BlockId, FunctionId, OperationId, RegionId};
use crate::zuir::ir::{
    EffectDefinition, ZuirBlock, ZuirFunction, ZuirModule, ZuirOperation,
    ZuirProgram, ZuirRegion,
};
use crate::zuir::ir::region::RegionOwner;
use crate::zuir::schema::types::QualifiedName;

// =============================================================================
// Public API
// =============================================================================

/// One independently actionable effect-verification finding.
///
/// `code` is a stable machine-readable identifier. The message is intended
/// for people and must not be used as a stable machine interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectIssue {
    code: &'static str,
    message: String,
}

impl EffectIssue {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    /// Return the stable issue code.
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

impl fmt::Display for EffectIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

/// All entities and effect definitions required by the effect verifier.
///
/// `effect_definitions` is the authoritative effect-definition environment
/// for this verification run. Each map key must equal the definition's
/// canonical name. The dialect/effect registry integration layer is
/// responsible for constructing this view from trusted definitions.
///
/// The verifier does not assume that a qualified name is registered merely
/// because the name is syntactically valid.
pub struct EffectVerificationInput<'a> {
    /// Canonical program and module/function hierarchy.
    pub program: &'a ZuirProgram<ZuirModule<ZuirFunction>>,

    /// All regions referenced by functions or operations.
    pub regions: &'a [ZuirRegion],

    /// All blocks referenced by regions.
    pub blocks: &'a [ZuirBlock],

    /// All operations referenced by blocks.
    pub operations: &'a [ZuirOperation],

    /// Canonical effect definitions, indexed by qualified effect name.
    pub effect_definitions: &'a BTreeMap<QualifiedName, EffectDefinition>,
}

/// Summary of one effect-verification run.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EffectVerificationReport {
    issues: Vec<EffectIssue>,
    functions_checked: usize,
    regions_checked: usize,
    blocks_checked: usize,
    operations_checked: usize,
    effect_definitions_checked: usize,
    effect_declarations_checked: usize,
}

impl EffectVerificationReport {
    /// Return findings in deterministic verification order.
    #[must_use]
    pub fn issues(&self) -> &[EffectIssue] {
        &self.issues
    }

    /// Return whether no effect-verification findings were produced.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.issues.is_empty()
    }

    /// Return the number of findings.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
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

    /// Return the number of registered effect definitions inspected.
    #[must_use]
    pub const fn effect_definitions_checked(&self) -> usize {
        self.effect_definitions_checked
    }

    /// Return the number of effect declarations inspected.
    #[must_use]
    pub const fn effect_declarations_checked(&self) -> usize {
        self.effect_declarations_checked
    }

    fn push(
        &mut self,
        code: &'static str,
        message: impl Into<String>,
    ) {
        self.issues.push(EffectIssue::new(code, message));
    }
}

// =============================================================================
// Entry point
// =============================================================================

/// Verify operation and function effect declarations across the supplied IR.
///
/// This function is read-only. It does not execute dialect code, infer
/// unrecorded effects, or decide whether a target can realize the program.
///
/// Callers must first provide the complete canonical entity view and the
/// registered effect definitions relevant to the program. A report with no
/// findings establishes only the checks implemented here; it is not a
/// substitute for the other required verifier passes.
#[must_use]
pub fn verify_effects(
    input: &EffectVerificationInput<'_>,
) -> EffectVerificationReport {
    let mut report = EffectVerificationReport::default();

    let definitions = index_effect_definitions(
        input.effect_definitions,
        &mut report,
    );

    let regions = index_regions(input.regions, &mut report);
    let blocks = index_blocks(input.blocks, &mut report);
    let operations = index_operations(input.operations, &mut report);

    verify_operation_effect_names(
        input.operations,
        &definitions,
        &mut report,
    );

    verify_function_effect_names(
        input.program,
        &definitions,
        &mut report,
    );

    verify_function_effect_coverage(
        input.program,
        &regions,
        &blocks,
        &operations,
        &mut report,
    );

    report
}

// =============================================================================
// Canonical indexes
// =============================================================================

fn index_effect_definitions<'a>(
    input: &'a BTreeMap<QualifiedName, EffectDefinition>,
    report: &mut EffectVerificationReport,
) -> BTreeMap<QualifiedName, &'a EffectDefinition> {
    let mut result = BTreeMap::new();

    for (key, definition) in input {
        report.effect_definitions_checked += 1;

        if let Err(error) = definition.validate() {
            report.push(
                "ZUIR-EFFECT-INVALID-DEFINITION",
                format!(
                    "effect definition `{key}` failed validation: {error}"
                ),
            );
        }

        if key != definition.name() {
            report.push(
                "ZUIR-EFFECT-DEFINITION-KEY-MISMATCH",
                format!(
                    "effect-definition key `{key}` does not match \
                     canonical definition name `{}`",
                    definition.name()
                ),
            );
            continue;
        }

        result.entry(key.clone()).or_insert(definition);
    }

    result
}

fn index_regions<'a>(
    input: &'a [ZuirRegion],
    report: &mut EffectVerificationReport,
) -> BTreeMap<RegionId, &'a ZuirRegion> {
    let mut result = BTreeMap::new();

    for region in input {
        report.regions_checked += 1;

        if let Err(error) = region.validate() {
            report.push(
                "ZUIR-EFFECT-INVALID-REGION",
                format!(
                    "region {} failed local validation: {error}",
                    region.id()
                ),
            );
        }

        if result.insert(region.id(), region).is_some() {
            report.push(
                "ZUIR-EFFECT-DUPLICATE-REGION-ID",
                format!(
                    "region identity {} occurs more than once",
                    region.id()
                ),
            );
        }
    }

    result
}

fn index_blocks<'a>(
    input: &'a [ZuirBlock],
    report: &mut EffectVerificationReport,
) -> BTreeMap<BlockId, &'a ZuirBlock> {
    let mut result = BTreeMap::new();

    for block in input {
        report.blocks_checked += 1;

        if let Err(error) = block.validate() {
            report.push(
                "ZUIR-EFFECT-INVALID-BLOCK",
                format!(
                    "block {} failed local validation: {error}",
                    block.id()
                ),
            );
        }

        if result.insert(block.id(), block).is_some() {
            report.push(
                "ZUIR-EFFECT-DUPLICATE-BLOCK-ID",
                format!(
                    "block identity {} occurs more than once",
                    block.id()
                ),
            );
        }
    }

    result
}

fn index_operations<'a>(
    input: &'a [ZuirOperation],
    report: &mut EffectVerificationReport,
) -> BTreeMap<OperationId, &'a ZuirOperation> {
    let mut result = BTreeMap::new();

    for operation in input {
        report.operations_checked += 1;

        if let Err(error) = operation.validate() {
            report.push(
                "ZUIR-EFFECT-INVALID-OPERATION",
                format!(
                    "operation {} failed local validation: {error}",
                    operation.id()
                ),
            );
        }

        if result.insert(operation.id(), operation).is_some() {
            report.push(
                "ZUIR-EFFECT-DUPLICATE-OPERATION-ID",
                format!(
                    "operation identity {} occurs more than once",
                    operation.id()
                ),
            );
        }
    }

    result
}

// =============================================================================
// Definition resolution
// =============================================================================

fn verify_one_effect_name(
    name: &QualifiedName,
    definitions: &BTreeMap<QualifiedName, &EffectDefinition>,
    owner_kind: &'static str,
    owner_id: impl fmt::Display,
    report: &mut EffectVerificationReport,
) {
    report.effect_declarations_checked += 1;

    if let Err(error) = name.validate() {
        report.push(
            "ZUIR-EFFECT-INVALID-NAME",
            format!(
                "{owner_kind} {owner_id} declares malformed effect \
                 `{name}`: {error}"
            ),
        );
        return;
    }

    match definitions.get(name) {
        Some(definition) => {
            if let Err(error) = definition.validate() {
                report.push(
                    "ZUIR-EFFECT-INVALID-DEFINITION-REFERENCE",
                    format!(
                        "{owner_kind} {owner_id} references effect `{name}`, \
                         whose registered definition is invalid: {error}"
                    ),
                );
            }
        }
        None => {
            report.push(
                "ZUIR-EFFECT-UNKNOWN-EFFECT",
                format!(
                    "{owner_kind} {owner_id} declares unregistered effect \
                     `{name}`"
                ),
            );
        }
    }
}

fn verify_operation_effect_names(
    operations: &[ZuirOperation],
    definitions: &BTreeMap<QualifiedName, &EffectDefinition>,
    report: &mut EffectVerificationReport,
) {
    for operation in operations {
        for effect in operation.effects() {
            verify_one_effect_name(
                effect,
                definitions,
                "operation",
                operation.id(),
                report,
            );
        }
    }
}

fn verify_function_effect_names(
    program: &ZuirProgram<ZuirModule<ZuirFunction>>,
    definitions: &BTreeMap<QualifiedName, &EffectDefinition>,
    report: &mut EffectVerificationReport,
) {
    for (_, module) in program.modules() {
        for (_, function) in module.functions() {
            for effect in function.effects() {
                verify_one_effect_name(
                    effect,
                    definitions,
                    "function",
                    function.id(),
                    report,
                );
            }
        }
    }
}

// =============================================================================
// Function effect coverage
// =============================================================================

/// Check that each function's declared effects cover the effects explicitly
/// declared by all operations reachable from its body-region references.
///
/// The walk follows region -> block -> operation -> nested region references.
/// It uses an explicit worklist and visited sets, so malformed cyclic region
/// references cannot cause unbounded recursive calls.
///
/// Structural verification remains responsible for proving that ownership
/// relationships are reciprocal and that the supplied IR is well-formed.
fn verify_function_effect_coverage(
    program: &ZuirProgram<ZuirModule<ZuirFunction>>,
    regions: &BTreeMap<RegionId, &ZuirRegion>,
    blocks: &BTreeMap<BlockId, &ZuirBlock>,
    operations: &BTreeMap<OperationId, &ZuirOperation>,
    report: &mut EffectVerificationReport,
) {
    for (_, module) in program.modules() {
        for (_, function) in module.functions() {
            report.functions_checked += 1;

            let declared = function.effects_set();
            let mut encountered = BTreeSet::<QualifiedName>::new();

            let mut pending_regions: Vec<RegionId> =
                function.body().regions().to_vec();

            let mut visited_regions = BTreeSet::<RegionId>::new();
            let mut visited_operations = BTreeSet::<OperationId>::new();

            while let Some(region_id) = pending_regions.pop() {
                if !visited_regions.insert(region_id) {
                    continue;
                }

                let Some(region) = regions.get(&region_id).copied() else {
                    report.push(
                        "ZUIR-EFFECT-MISSING-FUNCTION-REGION",
                        format!(
                            "function {} references missing body region {}",
                            function.id(),
                            region_id
                        ),
                    );
                    continue;
                };

                match region.owner() {
                    RegionOwner::Function(owner)
                        if owner == function.id() => {}
                    other => {
                        report.push(
                            "ZUIR-EFFECT-REGION-OWNER-MISMATCH",
                            format!(
                                "function {} references region {}, but its \
                                 declared owner is {:?}",
                                function.id(),
                                region_id,
                                other
                            ),
                        );
                    }
                }

                for block_id in region.blocks() {
                    let Some(block) = blocks.get(block_id).copied() else {
                        report.push(
                            "ZUIR-EFFECT-MISSING-BLOCK",
                            format!(
                                "region {} references missing block {}",
                                region_id,
                                block_id
                            ),
                        );
                        continue;
                    };

                    for operation_id in block.operations() {
                        if !visited_operations.insert(*operation_id) {
                            continue;
                        }

                        let Some(operation) =
                            operations.get(operation_id).copied()
                        else {
                            report.push(
                                "ZUIR-EFFECT-MISSING-OPERATION",
                                format!(
                                    "block {} references missing operation {}",
                                    block.id(),
                                    operation_id
                                ),
                            );
                            continue;
                        };

                        encountered.extend(
                            operation.effects().iter().cloned(),
                        );

                        // Nested regions belong to this operation according
                        // to the canonical region-owner contract.
                        for nested_region_id in operation.regions() {
                            pending_regions.push(*nested_region_id);
                        }
                    }
                }
            }

            for effect in encountered {
                if !declared.contains(&effect) {
                    report.push(
                        "ZUIR-EFFECT-FUNCTION-DECLARATION-INCOMPLETE",
                        format!(
                            "function {} does not declare effect `{effect}`, \
                             which is declared by an operation in its body",
                            function.id()
                        ),
                    );
                }
            }
        }
    }
}
