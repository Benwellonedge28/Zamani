
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR Type Verification
//!
//! Path: `src/classical/zcir/verify/types.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Verify type identities and type relationships across a supplied scope of
//! canonical ZUIR functions, regions, blocks, and operations.
//!
//! ZCIR is an integration boundary, not a competing IR or semantic type
//! system. This verifier consumes already-resolved canonical TypeIds and
//! TypeDefs. It does not parse source code or perform frontend name
//! resolution, generic inference, or target-specific layout.
//!
//! ## OWNS
//!
//! - Type-verification input and operation-signature contract types.
//! - Cross-entity type-reference checks.
//! - Function, block, operation, return, and successor type checks.
//! - Structured deterministic findings and an optional diagnostic budget.
//!
//! ## DOES NOT OWN
//!
//! - Canonical TypeId, ValueId, TypeDef, or IR entity representations.
//! - Source grammar, AST construction, or semantic name resolution.
//! - SSA dominance, alias analysis, effects, or resource verification.
//! - Operation registration or the authoritative semantics of dialects.
//! - Physical layout, ABI, register allocation, scheduling, or execution.
//!
//! ## INPUT CONTRACT
//!
//! The caller supplies a coherent verification scope containing all relevant
//! functions, regions, blocks, operations, and type definitions. Type IDs
//! must be those already assigned by the canonical semantic/IR pipeline.
//!
//! OperationTypeContract entries are supplied by trusted dialect/schema
//! integration. An operation without a supplied contract is still checked
//! for resolvable operand values and result types, but this file cannot
//! establish its dialect-specific operand/result rules.
//!
//! ## TRUST BOUNDARY
//!
//! This verifier is not a replacement for structural verification. The
//! structural verifier must run as a separate stage. Its findings must be
//! considered together with this report and the reports from other required
//! verifier stages.
//!
//! A diagnostic budget limits retained findings only. Truncation means the
//! report is incomplete and must never be treated as successful verification.
//!
//! ## SCALABILITY
//!
//! No universal limit is imposed on types, values, operands, results,
//! functions, blocks, or operations. Actual processing remains bounded by
//! finite identifier representations, address space, available resources,
//! and any explicitly configured operational budget.
//!
//! This implementation does not promise literal infinite memory or time.
//!
//! ## INTEGRATION
//!
//! - `src/zuir/ids.rs` owns TypeId, ValueId, and other canonical IDs.
//! - `src/zuir/types/type_def.rs` owns TypeDef, TypeRef, and TypeKind.
//! - `src/zuir/types/compatibility.rs` owns semantic type equivalence.
//! - `src/zuir/ir/function.rs` owns function signatures and parameters.
//! - `src/zuir/ir/block.rs` owns block arguments and terminators.
//! - `src/zuir/ir/operation.rs` owns operation operands and results.
//! - `src/zuir/ir/region.rs` owns region membership and region ownership.
//! - `structural.rs` owns structural consistency and reference topology.
//! - Dialect integration supplies operation type contracts.
//!
//! ## FREEZE CRITERIA
//!
//! The module must be registered by `verify/mod.rs` only after the parent
//! module and all sibling exports exist. Run formatting, checking, tests,
//! and Clippy against the complete repository before freezing this file.

use std::collections::{BTreeMap, BTreeSet};

use crate::zuir::ids::{BlockId, FunctionId, OperationId, RegionId, TypeId, ValueId};
use crate::zuir::ir::block::{BlockTerminator, ZuirBlock};
use crate::zuir::ir::function::ZuirFunction;
use crate::zuir::ir::operation::ZuirOperation;
use crate::zuir::ir::region::{RegionOwner, ZuirRegion};
use crate::zuir::schema::types::QualifiedName;
use crate::zuir::types::compatibility::{
    Compatibility, CompatibilityChecker, TypeResolver,
};
use crate::zuir::types::type_def::{TypeDef, TypeRef};

/// A canonical type identity and its authoritative semantic definition.
///
/// This entry is a borrowed view of existing semantic data. It does not
/// allocate a new type identity or replace the canonical type registry.
#[derive(Debug, Clone, Copy)]
pub struct TypeDefinitionEntry<'a> {
    /// Canonical identity of this type.
    pub id: TypeId,

    /// Canonical semantic definition associated with the identity.
    pub definition: &'a TypeDef,
}

impl<'a> TypeDefinitionEntry<'a> {
    /// Create a borrowed type-definition entry.
    #[must_use]
    pub const fn new(id: TypeId, definition: &'a TypeDef) -> Self {
        Self { id, definition }
    }
}

/// A fixed-arity operation type contract supplied by a dialect.
///
/// The contract is keyed by the canonical qualified operation name.
/// Operand and result types are canonical TypeIds in the same type
/// environment as the verification input.
///
/// This structure represents only fixed-arity signatures. A dialect with
/// variadic, dependent, polymorphic, or constraint-based signatures must
/// validate those rules through its own schema verifier rather than
/// pretending that this descriptor expresses them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationTypeContract {
    /// Fully qualified canonical operation name.
    pub name: QualifiedName,

    /// Expected operand types in operand order.
    pub operand_types: Vec<TypeId>,

    /// Expected result types in result order.
    pub result_types: Vec<TypeId>,
}

impl OperationTypeContract {
    /// Create a fixed-arity operation signature contract.
    #[must_use]
    pub fn new(
        name: QualifiedName,
        operand_types: Vec<TypeId>,
        result_types: Vec<TypeId>,
    ) -> Self {
        Self {
            name,
            operand_types,
            result_types,
        }
    }
}

/// Borrowed canonical IR entities and type contracts to verify.
///
/// The slices must represent one coherent scope. Do not combine unrelated
/// modules or type environments unless their identities and references have
/// already been reconciled by the owning integration layer.
#[derive(Debug, Clone, Copy)]
pub struct TypeVerificationInput<'a> {
    /// Function declarations and definitions in the scope.
    pub functions: &'a [ZuirFunction],

    /// Regions referenced by functions and operations.
    pub regions: &'a [ZuirRegion],

    /// Blocks in those regions.
    pub blocks: &'a [ZuirBlock],

    /// Canonical operation definitions.
    pub operations: &'a [ZuirOperation],

    /// Type identities and their semantic definitions.
    pub types: &'a [TypeDefinitionEntry<'a>],

    /// Trusted dialect-provided fixed-arity operation contracts.
    pub operation_contracts: &'a [OperationTypeContract],
}

impl<'a> TypeVerificationInput<'a> {
    /// Construct a verification input without cloning its IR entities.
    #[must_use]
    pub const fn new(
        functions: &'a [ZuirFunction],
        regions: &'a [ZuirRegion],
        blocks: &'a [ZuirBlock],
        operations: &'a [ZuirOperation],
        types: &'a [TypeDefinitionEntry<'a>],
        operation_contracts: &'a [OperationTypeContract],
    ) -> Self {
        Self {
            functions,
            regions,
            blocks,
            operations,
            types,
            operation_contracts,
        }
    }
}

/// Machine-readable category for a type-verification finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum TypeIssueCode {
    /// One TypeId was supplied more than once.
    DuplicateTypeId,

    /// One qualified type name was supplied more than once.
    DuplicateTypeName,

    /// A canonical type definition failed its local validation.
    InvalidTypeDefinition,

    /// An IR entity refers to a TypeId absent from the type environment.
    UnresolvedTypeId,

    /// Two or more type IDs refer to the same nominal definition name.
    AmbiguousTypeIdentity,

    /// A function parameter or result references an unresolved type.
    InvalidFunctionType,

    /// A block argument references an unresolved type.
    InvalidBlockArgumentType,

    /// An operation result references an unresolved type.
    InvalidOperationResultType,

    /// An operation operand references an unknown SSA value.
    UnresolvedOperandValue,

    /// A fixed-arity operation contract has the wrong operand count.
    OperandArityMismatch,

    /// A fixed-arity operation contract has the wrong result count.
    ResultArityMismatch,

    /// An operand does not satisfy the supplied operation type contract.
    OperandTypeMismatch,

    /// A result does not satisfy the supplied operation type contract.
    ResultTypeMismatch,

    /// A successor passes a value of an incompatible type.
    SuccessorArgumentTypeMismatch,

    /// A returned value does not match the function's declared result type.
    ReturnTypeMismatch,

    /// A return terminator has the wrong number of values.
    ReturnArityMismatch,

    /// More than one contract was supplied for the same operation name.
    DuplicateOperationContract,

    /// A supplied operation contract contains an unresolved type.
    InvalidOperationContract,
}

/// One type-verification finding.
///
/// Use `code` for programmatic handling. `entity` and `message` are
/// descriptive and must not be parsed as stable machine-readable fields.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeIssue {
    /// Stable issue category.
    pub code: TypeIssueCode,

    /// Entity or relationship associated with the finding.
    pub entity: String,

    /// Human-readable explanation.
    pub message: String,
}

/// Results of a type-verification run.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TypeVerificationReport {
    /// Retained findings, sorted deterministically.
    pub issues: Vec<TypeIssue>,

    /// Whether the configured diagnostic budget omitted findings.
    pub issues_truncated: bool,
}

impl TypeVerificationReport {
    /// Whether no issues were recorded and the report is complete.
    ///
    /// A truncated report is not a successful verification result.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.issues.is_empty() && !self.issues_truncated
    }

    /// Whether all findings encountered by the verifier could be retained.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        !self.issues_truncated
    }

    /// Number of retained findings.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    fn push(&mut self, issue: TypeIssue, maximum: Option<usize>) {
        if maximum.is_some_and(|limit| self.issues.len() >= limit) {
            self.issues_truncated = true;
        } else {
            self.issues.push(issue);
        }
    }

    fn finish(&mut self) {
        self.issues.sort();
    }
}

/// Configurable whole-scope type verifier.
///
/// No diagnostic limit is applied unless the caller explicitly configures
/// one. This configuration does not change type semantics.
#[derive(Debug, Clone, Copy, Default)]
pub struct TypeVerifier {
    max_diagnostics: Option<usize>,
}

impl TypeVerifier {
    /// Create a verifier with no additional diagnostic count limit.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_diagnostics: None,
        }
    }

    /// Create a verifier retaining at most `maximum` findings.
    ///
    /// Once the budget is exhausted, the verifier marks the report as
    /// truncated. It does not silently claim that remaining checks passed.
    #[must_use]
    pub const fn with_diagnostic_limit(maximum: usize) -> Self {
        Self {
            max_diagnostics: Some(maximum),
        }
    }

    /// Return the configured diagnostic limit, if any.
    #[must_use]
    pub const fn diagnostic_limit(self) -> Option<usize> {
        self.max_diagnostics
    }

    /// Verify the supplied canonical IR and semantic type environment.
    #[must_use]
    pub fn verify(
        &self,
        input: TypeVerificationInput<'_>,
    ) -> TypeVerificationReport {
        let mut report = TypeVerificationReport::default();

        let table = TypeTable::build(input.types, &mut report, self.max_diagnostics);

        let mut value_types = BTreeMap::<ValueId, TypeId>::new();

        self.collect_function_types(
            input.functions,
            &table,
            &mut value_types,
            &mut report,
        );

        self.collect_block_types(
            input.blocks,
            &table,
            &mut value_types,
            &mut report,
        );

        self.collect_operation_result_types(
            input.operations,
            &table,
            &mut value_types,
            &mut report,
        );

        let contracts = build_contract_index(
            input.operation_contracts,
            &table,
            &mut report,
            self.max_diagnostics,
        );

        self.verify_operations(
            input.operations,
            &table,
            &contracts,
            &value_types,
            &mut report,
        );

        self.verify_control_flow_types(
            input.functions,
            input.regions,
            input.blocks,
            &table,
            &value_types,
            &mut report,
        );

        report.finish();
        report
    }

    fn collect_function_types(
        &self,
        functions: &[ZuirFunction],
        table: &TypeTable<'_>,
        value_types: &mut BTreeMap<ValueId, TypeId>,
        report: &mut TypeVerificationReport,
    ) {
        for function in functions {
            for parameter in function.signature().parameters() {
                self.require_type(
                    parameter.ty(),
                    format!("function {} parameter {}", function.id(), parameter.value()),
                    table,
                    report,
                    TypeIssueCode::InvalidFunctionType,
                );

                value_types.entry(parameter.value()).or_insert(parameter.ty());
            }

            for (index, ty) in function.signature().results().iter().enumerate() {
                self.require_type(
                    *ty,
                    format!("function {} result {index}", function.id()),
                    table,
                    report,
                    TypeIssueCode::InvalidFunctionType,
                );
            }
        }
    }

    fn collect_block_types(
        &self,
        blocks: &[ZuirBlock],
        table: &TypeTable<'_>,
        value_types: &mut BTreeMap<ValueId, TypeId>,
        report: &mut TypeVerificationReport,
    ) {
        for block in blocks {
            for argument in block.arguments() {
                self.require_type(
                    argument.ty(),
                    format!("block {} argument {}", block.id(), argument.value()),
                    table,
                    report,
                    TypeIssueCode::InvalidBlockArgumentType,
                );

                value_types.entry(argument.value()).or_insert(argument.ty());
            }
        }
    }

    fn collect_operation_result_types(
        &self,
        operations: &[ZuirOperation],
        table: &TypeTable<'_>,
        value_types: &mut BTreeMap<ValueId, TypeId>,
        report: &mut TypeVerificationReport,
    ) {
        for operation in operations {
            for result in operation.results() {
                self.require_type(
                    result.ty(),
                    format!("operation {} result {}", operation.id(), result.value()),
                    table,
                    report,
                    TypeIssueCode::InvalidOperationResultType,
                );

                value_types.entry(result.value()).or_insert(result.ty());
            }
        }
    }

    fn verify_operations(
        &self,
        operations: &[ZuirOperation],
        table: &TypeTable<'_>,
        contracts: &BTreeMap<QualifiedName, &OperationTypeContract>,
        value_types: &BTreeMap<ValueId, TypeId>,
        report: &mut TypeVerificationReport,
    ) {
        for operation in operations {
            let mut actual_operand_types = Vec::new();

            for operand in operation.operands() {
                match value_types.get(&operand.value()).copied() {
                    Some(ty) => {
                        self.require_type(
                            ty,
                            format!(
                                "operation {} operand value {}",
                                operation.id(),
                                operand.value()
                            ),
                            table,
                            report,
                            TypeIssueCode::UnresolvedTypeId,
                        );
                        actual_operand_types.push(Some(ty));
                    }
                    None => {
                        report.push(
                            TypeIssue {
                                code: TypeIssueCode::UnresolvedOperandValue,
                                entity: format!(
                                    "operation {} operand {}",
                                    operation.id(),
                                    operand.value()
                                ),
                                message: format!(
                                    "operand references value {} with no type-known definition in this scope",
                                    operand.value()
                                ),
                            },
                            self.max_diagnostics,
                        );
                        actual_operand_types.push(None);
                    }
                }
            }

            let Some(contract) = contracts.get(operation.name()) else {
                // The dialect legality/schema stage must report unknown or
                // unsupported operations. Absence of a type contract here
                // is not evidence that the operation is universally legal.
                continue;
            };

            if operation.operands().len() != contract.operand_types.len() {
                report.push(
                    TypeIssue {
                        code: TypeIssueCode::OperandArityMismatch,
                        entity: format!("operation {}", operation.id()),
                        message: format!(
                            "operation `{}` has {} operands but its supplied contract requires {}",
                            operation.name(),
                            operation.operands().len(),
                            contract.operand_types.len()
                        ),
                    },
                    self.max_diagnostics,
                );
            }

            if operation.results().len() != contract.result_types.len() {
                report.push(
                    TypeIssue {
                        code: TypeIssueCode::ResultArityMismatch,
                        entity: format!("operation {}", operation.id()),
                        message: format!(
                            "operation `{}` has {} results but its supplied contract requires {}",
                            operation.name(),
                            operation.results().len(),
                            contract.result_types.len()
                        ),
                    },
                    self.max_diagnostics,
                );
            }

            for (index, (actual, expected)) in actual_operand_types
                .iter()
                .zip(contract.operand_types.iter())
                .enumerate()
            {
                if let Some(actual) = actual {
                    if !self.types_equivalent(*actual, *expected, table) {
                        report.push(
                            TypeIssue {
                                code: TypeIssueCode::OperandTypeMismatch,
                                entity: format!("operation {} operand {index}", operation.id()),
                                message: format!(
                                    "operand type {} is incompatible with required type {}",
                                    actual, expected
                                ),
                            },
                            self.max_diagnostics,
                        );
                    }
                }
            }

            for (index, (actual, expected)) in operation
                .results()
                .iter()
                .zip(contract.result_types.iter())
                .enumerate()
            {
                if !self.types_equivalent(actual.ty(), *expected, table) {
                    report.push(
                        TypeIssue {
                            code: TypeIssueCode::ResultTypeMismatch,
                            entity: format!("operation {} result {index}", operation.id()),
                            message: format!(
                                "result type {} is incompatible with required type {}",
                                actual.ty(), expected
                            ),
                        },
                        self.max_diagnostics,
                    );
                }
            }
        }
    }

    fn verify_control_flow_types(
        &self,
        functions: &[ZuirFunction],
        regions: &[ZuirRegion],
        blocks: &[ZuirBlock],
        table: &TypeTable<'_>,
        value_types: &BTreeMap<ValueId, TypeId>,
        report: &mut TypeVerificationReport,
    ) {
        let region_owners: BTreeMap<RegionId, RegionOwner> = regions
            .iter()
            .map(|region| (region.id(), region.owner().clone()))
            .collect();

        let functions_by_id: BTreeMap<FunctionId, &ZuirFunction> = functions
            .iter()
            .map(|function| (function.id(), function))
            .collect();

        let blocks_by_id: BTreeMap<BlockId, &ZuirBlock> = blocks
            .iter()
            .map(|block| (block.id(), block))
            .collect();

        for block in blocks {
            let Some(parent_region) = block.parent_region() else {
                // Structural verification owns missing parent-region errors.
                continue;
            };

            match block.terminator() {
                BlockTerminator::Return { values } => {
                    let function_id = match region_owners.get(&parent_region) {
                        Some(RegionOwner::Function(id)) => Some(*id),
                        _ => None,
                    };

                    let Some(function_id) = function_id else {
                        // A return inside an operation-owned region is not
                        // automatically a return from its enclosing function.
                        // Its meaning belongs to the region/dialect contract.
                        continue;
                    };

                    let Some(function) = functions_by_id.get(&function_id) else {
                        continue;
                    };

                    let expected = function.signature().results();

                    if values.len() != expected.len() {
                        report.push(
                            TypeIssue {
                                code: TypeIssueCode::ReturnArityMismatch,
                                entity: format!("block {}", block.id()),
                                message: format!(
                                    "return has {} values but function {} declares {} results",
                                    values.len(),
                                    function_id,
                                    expected.len()
                                ),
                            },
                            self.max_diagnostics,
                        );
                    }

                    for (index, (value, expected_type)) in
                        values.iter().zip(expected.iter()).enumerate()
                    {
                        let Some(actual_type) = value_types.get(value).copied() else {
                            report.push(
                                TypeIssue {
                                    code: TypeIssueCode::UnresolvedOperandValue,
                                    entity: format!("block {} return value {value}", block.id()),
                                    message: format!(
                                        "returned value {value} has no type-known definition in this scope"
                                    ),
                                },
                                self.max_diagnostics,
                            );
                            continue;
                        };

                        if !self.types_equivalent(actual_type, *expected_type, table) {
                            report.push(
                                TypeIssue {
                                    code: TypeIssueCode::ReturnTypeMismatch,
                                    entity: format!("block {} return value {index}", block.id()),
                                    message: format!(
                                        "returned value type {} is incompatible with function result type {}",
                                        actual_type, expected_type
                                    ),
                                },
                                self.max_diagnostics,
                            );
                        }
                    }
                }

                _ => {
                    block.terminator().for_each_successor(|successor| {
                        let Some(destination) = blocks_by_id.get(&successor.target()) else {
                            // Structural verification owns missing-target errors.
                            return;
                        };

                        for (index, (value, argument)) in successor
                            .arguments()
                            .iter()
                            .zip(destination.arguments().iter())
                            .enumerate()
                        {
                            let Some(actual_type) = value_types.get(value).copied() else {
                                report.push(
                                    TypeIssue {
                                        code: TypeIssueCode::UnresolvedOperandValue,
                                        entity: format!(
                                            "block {} edge to {} argument {index}",
                                            block.id(),
                                            destination.id()
                                        ),
                                        message: format!(
                                            "successor argument value {value} has no type-known definition in this scope"
                                        ),
                                    },
                                    self.max_diagnostics,
                                );
                                continue;
                            };

                            if !self.types_equivalent(actual_type, argument.ty(), table) {
                                report.push(
                                    TypeIssue {
                                        code: TypeIssueCode::SuccessorArgumentTypeMismatch,
                                        entity: format!(
                                            "block {} edge to {} argument {index}",
                                            block.id(),
                                            destination.id()
                                        ),
                                        message: format!(
                                            "edge value type {} is incompatible with destination argument type {}",
                                            actual_type,
                                            argument.ty()
                                        ),
                                    },
                                    self.max_diagnostics,
                                );
                            }
                        }
                    });
                }
            }
        }
    }

    fn require_type(
        &self,
        id: TypeId,
        entity: String,
        table: &TypeTable<'_>,
        report: &mut TypeVerificationReport,
        code: TypeIssueCode,
    ) {
        if table.ambiguous_ids.contains(&id) {
            report.push(
                TypeIssue {
                    code: TypeIssueCode::AmbiguousTypeIdentity,
                    entity,
                    message: format!(
                        "type identity {id} has multiple conflicting definitions"
                    ),
                },
                self.max_diagnostics,
            );
        } else if !table.by_id.contains_key(&id) {
            report.push(
                TypeIssue {
                    code,
                    entity,
                    message: format!("type identity {id} is not defined in this scope"),
                },
                self.max_diagnostics,
            );
        }
    }

    fn types_equivalent(
        &self,
        left: TypeId,
        right: TypeId,
        table: &TypeTable<'_>,
    ) -> bool {
        if table.ambiguous_ids.contains(&left) || table.ambiguous_ids.contains(&right) {
            return false;
        }

        let (Some(left_definition), Some(right_definition)) =
            (table.by_id.get(&left), table.by_id.get(&right))
        else {
            return false;
        };

        let left_ref = TypeRef::Named(left_definition.name.clone());
        let right_ref = TypeRef::Named(right_definition.name.clone());

        CompatibilityChecker::new(table)
            .equivalent(&left_ref, &right_ref)
            .is_compatible()
    }
}

/// Type table constructed for one verification run.
///
/// Duplicate identities and duplicate qualified names are recorded instead
/// of being resolved through whichever item happened to appear last.
struct TypeTable<'a> {
    by_id: BTreeMap<TypeId, &'a TypeDef>,
    by_name: BTreeMap<QualifiedName, &'a TypeDef>,
    ambiguous_ids: BTreeSet<TypeId>,
    ambiguous_names: BTreeSet<QualifiedName>,
}

impl<'a> TypeTable<'a> {
    fn build(
        entries: &[TypeDefinitionEntry<'a>],
        report: &mut TypeVerificationReport,
        maximum: Option<usize>,
    ) -> Self {
        let mut table = Self {
            by_id: BTreeMap::new(),
            by_name: BTreeMap::new(),
            ambiguous_ids: BTreeSet::new(),
            ambiguous_names: BTreeSet::new(),
        };

        for entry in entries {
            if let Err(error) = entry.definition.validate() {
                report.push(
                    TypeIssue {
                        code: TypeIssueCode::InvalidTypeDefinition,
                        entity: format!("type {}", entry.id),
                        message: error.to_string(),
                    },
                    maximum,
                );
            }

            if table.by_id.contains_key(&entry.id) {
                table.ambiguous_ids.insert(entry.id);
                report.push(
                    TypeIssue {
                        code: TypeIssueCode::DuplicateTypeId,
                        entity: format!("type {}", entry.id),
                        message: "the same TypeId was supplied more than once".to_owned(),
                    },
                    maximum,
                );
            } else {
                table.by_id.insert(entry.id, entry.definition);
            }

            let name = entry.definition.name.clone();

            if table.by_name.contains_key(&name) {
                table.ambiguous_names.insert(name.clone());
                report.push(
                    TypeIssue {
                        code: TypeIssueCode::DuplicateTypeName,
                        entity: name.to_string(),
                        message: "the same qualified type name was supplied more than once".to_owned(),
                    },
                    maximum,
                );
            } else {
                table.by_name.insert(name, entry.definition);
            }
        }

        // A duplicated name must not be treated as resolved by choosing
        // whichever definition happened to be inserted first.
        for name in &table.ambiguous_names {
            table.by_name.remove(name);
        }

        table
    }
}

impl TypeResolver for TypeTable<'_> {
    fn resolve(&self, name: &QualifiedName) -> Option<&TypeDef> {
        if self.ambiguous_names.contains(name) {
            return None;
        }

        self.by_name.get(name).copied()
    }
}

fn build_contract_index<'a>(
    contracts: &'a [OperationTypeContract],
    table: &TypeTable<'_>,
    report: &mut TypeVerificationReport,
    maximum: Option<usize>,
) -> BTreeMap<QualifiedName, &'a OperationTypeContract> {
    let mut index = BTreeMap::new();
    let mut ambiguous = BTreeSet::new();

    for contract in contracts {
        if let Err(error) = contract.name.validate() {
            report.push(
                TypeIssue {
                    code: TypeIssueCode::InvalidOperationContract,
                    entity: contract.name.to_string(),
                    message: error.to_string(),
                },
                maximum,
            );
            continue;
        }

        for (index, ty) in contract
            .operand_types
            .iter()
            .chain(contract.result_types.iter())
            .enumerate()
        {
            if !table.by_id.contains_key(ty) || table.ambiguous_ids.contains(ty) {
                report.push(
                    TypeIssue {
                        code: TypeIssueCode::InvalidOperationContract,
                        entity: contract.name.to_string(),
                        message: format!(
                            "operation type contract references unresolved type {ty} at type position {index}"
                        ),
                    },
                    maximum,
                );
            }
        }

        if index.contains_key(&contract.name) {
            ambiguous.insert(contract.name.clone());
            report.push(
                TypeIssue {
                    code: TypeIssueCode::DuplicateOperationContract,
                    entity: contract.name.to_string(),
                    message: "multiple type contracts were supplied for this operation name".to_owned(),
                },
                maximum,
            );
        } else {
            index.insert(contract.name.clone(), contract);
        }
    }

    for name in ambiguous {
        index.remove(&name);
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ids::{OperationId, TypeId, ValueId};
    use crate::zuir::ir::function::{FunctionBody, FunctionSignature};
    use crate::zuir::types::type_def::{TypeKind, TypeVisibility};

    fn type_id(raw: u128) -> TypeId {
        TypeId::new(raw).expect("test type ID must be nonzero")
    }

    fn value_id(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test value ID must be nonzero")
    }

    fn name(text: &str) -> QualifiedName {
        QualifiedName::parse(text).expect("test qualified name must be valid")
    }

    fn definition(text: &str, kind: TypeKind) -> TypeDef {
        let mut value = TypeDef::new(name(text), kind)
            .expect("test type definition must be valid");
        value.visibility = TypeVisibility::Public;
        value
    }

    fn empty_input<'a>(
        types: &'a [TypeDefinitionEntry<'a>],
    ) -> TypeVerificationInput<'a> {
        TypeVerificationInput::new(&[], &[], &[], &[], types, &[])
    }

    #[test]
    fn empty_scope_is_clean() {
        let types = [];
        let report = TypeVerifier::new().verify(empty_input(&types));

        assert!(report.is_clean());
        assert!(report.is_complete());
    }

    #[test]
    fn unresolved_type_identity_is_reported() {
        let definition = definition("core::Boolean", TypeKind::Boolean);
        let types = [TypeDefinitionEntry::new(type_id(1), &definition)];

        let signature = FunctionSignature::new(
            Vec::new(),
            vec![type_id(2)],
            None,
            false,
        )
        .expect("test signature must be valid");

        let function = ZuirFunction::new(
            crate::zuir::ids::FunctionId::new(1).unwrap(),
            crate::zuir::ids::SymbolId::new(1).unwrap(),
            signature,
            FunctionBody::declaration(),
        )
        .expect("test function must be valid");

        let functions = [function];
        let input = TypeVerificationInput::new(
            &functions,
            &[],
            &[],
            &[],
            &types,
            &[],
        );

        let report = TypeVerifier::new().verify(input);

        assert!(report.issues.iter().any(|issue| {
            issue.code == TypeIssueCode::InvalidFunctionType
        }));
    }

    #[test]
    fn diagnostic_budget_marks_report_incomplete() {
        let definition = definition("core::Boolean", TypeKind::Boolean);
        let types = [TypeDefinitionEntry::new(type_id(1), &definition)];

        let signature = FunctionSignature::new(
            Vec::new(),
            vec![type_id(2)],
            None,
            false,
        )
        .expect("test signature must be valid");

        let function = ZuirFunction::new(
            crate::zuir::ids::FunctionId::new(1).unwrap(),
            crate::zuir::ids::SymbolId::new(1).unwrap(),
            signature,
            FunctionBody::declaration(),
        )
        .expect("test function must be valid");

        let functions = [function];
        let input = TypeVerificationInput::new(
            &functions,
            &[],
            &[],
            &[],
            &types,
            &[],
        );

        let report = TypeVerifier::with_diagnostic_limit(0).verify(input);

        assert!(report.issues_truncated);
        assert!(!report.is_clean());
        assert!(!report.is_complete());
    }

    #[test]
    fn same_type_identity_is_compatible() {
        let definition = definition("core::Boolean", TypeKind::Boolean);
        let types = [TypeDefinitionEntry::new(type_id(1), &definition)];
        let mut report = TypeVerificationReport::default();
        let table = TypeTable::build(&types, &mut report, None);

        assert!(TypeVerifier::new().types_equivalent(
            type_id(1),
            type_id(1),
            &table,
        ));
    }

    #[test]
    fn different_nominal_names_are_not_implicitly_compatible() {
        let left = definition("core::Left", TypeKind::Boolean);
        let right = definition("core::Right", TypeKind::Boolean);
        let types = [
            TypeDefinitionEntry::new(type_id(1), &left),
            TypeDefinitionEntry::new(type_id(2), &right),
        ];

        let mut report = TypeVerificationReport::default();
        let table = TypeTable::build(&types, &mut report, None);

        assert!(!TypeVerifier::new().types_equivalent(
            type_id(1),
            type_id(2),
            &table,
        ));
    }
}
