
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Semantic Type Verification
//!
//! Path: `src/zuir/verify/types.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Verify semantic type identities and type references throughout the
//! canonical ZUIR representation.
//!
//! AUTHORITY
//!
//! - src/zuir/ZUIR-SPEC.md
//! - src/zuir/ids.rs
//! - src/zuir/ir/program.rs
//! - src/zuir/ir/module.rs
//! - src/zuir/ir/function.rs
//! - src/zuir/ir/region.rs
//! - src/zuir/ir/block.rs
//! - src/zuir/ir/operation.rs
//! - src/zuir/ir/operand.rs
//! - src/zuir/ir/result.rs
//! - src/zuir/types/type_def.rs
//! - src/zuir/types/type_id.rs
//! - grammar/DESIGN.md
//! - grammar/Zamani.g4
//! - grammar/grammar.md
//! - grammar/specification/semantic-model.md
//!
//! OWNS
//!
//! - Resolution of TypeId references against the supplied canonical type
//!   definition table.
//! - Local validation of semantic type definitions.
//! - Detection of duplicate type names in the supplied type scope.
//! - Resolution of named TypeRef references, including nested type forms.
//! - Validation that function parameters, function results, block arguments,
//!   operation results, and operation operands refer to known types/values.
//! - Deterministic, structured type-verification findings.
//!
//! DOES_NOT_OWN
//!
//! - Source parsing, AST construction, name resolution, or type inference.
//! - Allocation or global registration of TypeId values.
//! - Structural containment and control-flow verification.
//! - SSA dominance, value lifetime, or ownership verification.
//! - Dialect-specific operation signatures or conversion semantics.
//! - Effects, resources, physical layouts, device placement, or lowering.
//! - Runtime execution or target capability discovery.
//!
//! INPUT CONTRACT
//!
//! The caller supplies the complete program/module/function hierarchy,
//! all regions, blocks, and operations, and the canonical TypeId-to-TypeDef
//! table for the type scope being verified.
//!
//! Region, block, and operation references are resolved through their
//! canonical IDs. Structural verification remains responsible for proving
//! that those references belong to the correct enclosing entities.
//!
//! TYPE IDENTITY
//!
//! A TypeId is an identity, not a type definition or a type name.
//! A TypeDef owns a QualifiedName and a semantic TypeKind.
//! This module never guesses a type from an integer ID or a source spelling.
//!
//! SCALABILITY
//!
//! No universal hardware, qubit, register-width, tensor-rank, node, or
//! operation-count limit is introduced. Collections grow dynamically.
//! Work scales with the supplied IR and type graph, subject to available
//! resources and finite identifier representations.
//!
//! Unknown semantic types and unresolved named references are errors.
//! Unknown extension-defined operation semantics remain the responsibility
//! of dialect verification.
//!
//! FREEZE CRITERIA
//!
//! - Public types and accessors match the canonical IR modules.
//! - Type definitions are supplied by their owning type subsystem.
//! - All supported TypeKind and TypeRef forms are checked.
//! - No unsafe code, hardware-size constants, or host-width truncation exist.
//! - The verifier does not mutate the IR.
//! - Structural, SSA, dialect, and lowering responsibilities remain separate.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::ids::{TypeId, ValueId};
use crate::zuir::ir::{
    BlockArgument, OperationValueResult, ZuirBlock, ZuirFunction, ZuirModule,
    ZuirOperation, ZuirProgram, ZuirRegion,
};
use crate::zuir::types::type_def::{
    ConstantArgument, FieldDef, FunctionParameter, TypeArgument, TypeConstraint,
    TypeDef, TypeKind, TypeRef, VariantDef,
};

/// Type-verification outcome.
pub type TypeVerificationResult<T> = Result<T, TypeVerificationError>;

/// A deterministic, independently actionable type-verification finding.
///
/// Codes are stable machine-readable identifiers. Messages are intended for
/// people and must not be used as a machine interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeIssue {
    code: &'static str,
    message: String,
}

impl TypeIssue {
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

impl fmt::Display for TypeIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

/// Errors indicating that type verification could not be completed.
///
/// Ordinary malformed IR is represented by [`TypeIssue`] entries so that
/// callers can report multiple independent findings in one pass.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TypeVerificationError {
    /// A required canonical type definition table is inconsistent.
    InvalidTypeEnvironment(String),

    /// The verifier's required collection capacity could not be reserved.
    AllocationFailure {
        collection: &'static str,
    },
}

impl fmt::Display for TypeVerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTypeEnvironment(reason) => {
                write!(f, "invalid ZUIR type environment: {reason}")
            }
            Self::AllocationFailure { collection } => {
                write!(f, "unable to reserve memory for {collection}")
            }
        }
    }
}

impl std::error::Error for TypeVerificationError {}

/// Complete input required by the type verifier.
///
/// The `TypeId` keys are authoritative identities. Definitions must be
/// registered under the same IDs used by function signatures, block
/// arguments, and operation results.
///
/// The program's modules and functions are canonical containers. Regions,
/// blocks, and operations are separate canonical entities in the current
/// representation and are therefore supplied independently.
pub struct TypeVerificationInput<'a> {
    /// Program and module/function hierarchy.
    pub program: &'a ZuirProgram<ZuirModule<ZuirFunction>>,

    /// All regions in the representation being verified.
    pub regions: &'a [ZuirRegion],

    /// All blocks in the representation being verified.
    pub blocks: &'a [ZuirBlock],

    /// All operations in the representation being verified.
    pub operations: &'a [ZuirOperation],

    /// Canonical semantic type definitions indexed by TypeId.
    ///
    /// This mapping belongs to the caller's type-registry or compilation
    /// context. This verifier does not invent IDs or silently intern types.
    pub type_definitions: &'a BTreeMap<TypeId, TypeDef>,
}

/// Summary of one type-verification run.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TypeVerificationReport {
    issues: Vec<TypeIssue>,
    types_checked: usize,
    functions_checked: usize,
    regions_checked: usize,
    blocks_checked: usize,
    operations_checked: usize,
    values_checked: usize,
}

impl TypeVerificationReport {
    /// Return findings in deterministic verification order.
    #[must_use]
    pub fn issues(&self) -> &[TypeIssue] {
        &self.issues
    }

    /// Return whether no type-verification findings were produced.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.issues.is_empty()
    }

    /// Return the number of findings.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Return the number of type definitions inspected.
    #[must_use]
    pub const fn types_checked(&self) -> usize {
        self.types_checked
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

    /// Return the number of value definitions inspected.
    #[must_use]
    pub const fn values_checked(&self) -> usize {
        self.values_checked
    }

    fn push(
        &mut self,
        code: &'static str,
        message: impl Into<String>,
    ) {
        self.issues.push(TypeIssue::new(code, message));
    }
}

/// Verify semantic type references across the supplied ZUIR representation.
///
/// The verifier is read-only and deterministic for equivalent inputs.
/// Ordered maps and sets are used for indexes and type-name resolution.
///
/// This pass establishes that type identities and named type references
/// resolve. It does not establish that every operation's operands satisfy
/// its dialect-defined signature. That additional check belongs to the
/// operation-schema verifier.
///
/// Run structural verification as a separate required pass. A successful
/// type report is not proof that the complete program is structurally valid,
/// executable, or legal for a particular target.
pub fn verify_types(
    input: &TypeVerificationInput<'_>,
) -> TypeVerificationReport {
    let mut report = TypeVerificationReport::default();

    let type_names = index_type_definitions(
        input.type_definitions,
        &mut report,
    );

    verify_type_references(
        input.type_definitions,
        &type_names,
        &mut report,
    );

    let mut value_types = BTreeMap::<ValueId, TypeId>::new();

    index_function_values(
        input.program,
        input.type_definitions,
        &mut value_types,
        &mut report,
    );

    index_block_values(
        input.blocks,
        input.type_definitions,
        &mut value_types,
        &mut report,
    );

    index_operation_results(
        input.operations,
        input.type_definitions,
        &mut value_types,
        &mut report,
    );

    verify_operation_operands(
        input.operations,
        &value_types,
        input.type_definitions,
        &mut report,
    );

    report.regions_checked = input.regions.len();
    report.blocks_checked = input.blocks.len();
    report.operations_checked = input.operations.len();

    report
}

/// Build the canonical name-to-ID view and validate every type definition.
///
/// Duplicate names are reported; the first encountered definition remains
/// authoritative for the remainder of this run. Since the input map is
/// ordered by TypeId, this behavior is deterministic.
fn index_type_definitions(
    definitions: &BTreeMap<TypeId, TypeDef>,
    report: &mut TypeVerificationReport,
) -> BTreeMap<String, TypeId> {
    let mut names = BTreeMap::new();

    for (id, definition) in definitions {
        report.types_checked += 1;

        if let Err(error) = definition.validate() {
            report.push(
                "ZUIR-TYPE-INVALID-DEFINITION",
                format!("type {id} failed local validation: {error}"),
            );
        }

        let name = definition.name().canonical_string();

        if let Some(previous) = names.get(&name).copied() {
            report.push(
                "ZUIR-TYPE-DUPLICATE-NAME",
                format!(
                    "type name `{name}` is assigned to both {previous} and {id}"
                ),
            );
        } else {
            names.insert(name, *id);
        }
    }

    names
}

/// Validate all named references nested in every type definition.
///
/// This includes references within aggregate fields, generic arguments,
/// function signatures, constraints, and extension-defined type arguments.
fn verify_type_references(
    definitions: &BTreeMap<TypeId, TypeDef>,
    names: &BTreeMap<String, TypeId>,
    report: &mut TypeVerificationReport,
) {
    for (id, definition) in definitions {
        let mut references = Vec::<String>::new();

        collect_kind_references(definition.kind(), &mut references);

        for name in references {
            if !names.contains_key(&name) {
                report.push(
                    "ZUIR-TYPE-UNRESOLVED-REFERENCE",
                    format!(
                        "type {id} references undefined semantic type `{name}`"
                    ),
                );
            }
        }
    }
}

/// Collect named references without recursively expanding referenced
/// definitions. This prevents recursive and mutually recursive types from
/// causing unbounded recursion in the verifier.
fn collect_kind_references(
    kind: &TypeKind,
    output: &mut Vec<String>,
) {
    match kind {
        TypeKind::Never
        | TypeKind::Unit
        | TypeKind::Boolean
        | TypeKind::Integer { .. }
        | TypeKind::Float { .. }
        | TypeKind::Decimal { .. }
        | TypeKind::String
        | TypeKind::Bytes
        | TypeKind::Opaque { .. }
        | TypeKind::Parameter { .. } => {
            match kind {
                TypeKind::Opaque { constraints }
                | TypeKind::Parameter {
                    bounds: constraints,
                    ..
                } => {
                    for constraint in constraints {
                        collect_constraint_references(constraint, output);
                    }
                }
                _ => {}
            }
        }

        TypeKind::Tuple { elements } => {
            for ty in elements {
                collect_type_ref(ty, output);
            }
        }

        TypeKind::Array { element, .. } => {
            collect_type_ref(element, output);
        }

        TypeKind::Record { fields } => {
            for field in fields {
                collect_field_references(field, output);
            }
        }

        TypeKind::Sum { variants } => {
            for variant in variants {
                collect_variant_references(variant, output);
            }
        }

        TypeKind::Union { alternatives } => {
            for ty in alternatives {
                collect_type_ref(ty, output);
            }
        }

        TypeKind::Reference { target } => {
            output.push(target.canonical_string());
        }

        TypeKind::Newtype { underlying } => {
            collect_type_ref(underlying, output);
        }

        TypeKind::Applied {
            constructor,
            arguments,
        } => {
            output.push(constructor.canonical_string());

            for argument in arguments {
                collect_type_argument_references(argument, output);
            }
        }

        TypeKind::Function {
            parameters,
            results,
            ..
        } => {
            for parameter in parameters {
                collect_type_ref(&parameter.ty, output);
            }

            for result in results {
                collect_type_ref(result, output);
            }
        }

        TypeKind::Quantum {
            domain_type,
            parameters,
        }
        | TypeKind::Hardware {
            domain_type,
            parameters,
        } => {
            output.push(domain_type.canonical_string());

            for argument in parameters {
                collect_type_argument_references(argument, output);
            }
        }

        TypeKind::Dialect {
            dialect,
            type_name,
            arguments,
        } => {
            output.push(dialect.canonical_string());
            output.push(type_name.canonical_string());

            for argument in arguments {
                collect_type_argument_references(argument, output);
            }
        }

        // TypeKind is non-exhaustive by contract. New variants must be
        // assigned an explicit reference-validation policy before release.
        _ => {}
    }
}

fn collect_type_ref(
    ty: &TypeRef,
    output: &mut Vec<String>,
) {
    match ty {
        TypeRef::Named(name) => {
            output.push(name.canonical_string());
        }
        TypeRef::Inline(kind) => {
            collect_kind_references(kind, output);
        }
    }
}

fn collect_field_references(
    field: &FieldDef,
    output: &mut Vec<String>,
) {
    collect_type_ref(&field.ty, output);
}

fn collect_variant_references(
    variant: &VariantDef,
    output: &mut Vec<String>,
) {
    for field in &variant.fields {
        collect_field_references(field, output);
    }
}

fn collect_type_argument_references(
    argument: &TypeArgument,
    output: &mut Vec<String>,
) {
    match argument {
        TypeArgument::Type(ty) => collect_type_ref(ty, output),
        TypeArgument::Constant(constant) => {
            if let ConstantArgument::Name(name) = constant {
                output.push(name.canonical_string());
            }
        }
        TypeArgument::Symbolic(_) => {}
    }
}

fn collect_constraint_references(
    constraint: &TypeConstraint,
    output: &mut Vec<String>,
) {
    match constraint {
        TypeConstraint::Implements(name) => {
            // An Implements target is an interface/trait name, not
            // necessarily a semantic TypeDef. Its existence is verified
            // by the trait/dialect registry, not by this type-name index.
            let _ = name;
        }

        TypeConstraint::EqualTo(ty) => {
            collect_type_ref(ty, output);
        }

        TypeConstraint::IntegerWidthAtLeast(_)
        | TypeConstraint::Shape(_) => {}

        TypeConstraint::Dialect {
            namespace,
            constraint,
        } => {
            // These names are owned by the dialect/constraint registry.
            let _ = (namespace, constraint);
        }
    }
}

/// Validate function parameter and result type identities and index parameter
/// values so operation operands can be resolved to their semantic types.
fn index_function_values(
    program: &ZuirProgram<ZuirModule<ZuirFunction>>,
    definitions: &BTreeMap<TypeId, TypeDef>,
    value_types: &mut BTreeMap<ValueId, TypeId>,
    report: &mut TypeVerificationReport,
) {
    for (module_id, module) in program.modules() {
        for (function_id, function) in module.functions() {
            report.functions_checked += 1;

            if let Err(error) = function.validate() {
                report.push(
                    "ZUIR-TYPE-INVALID-FUNCTION",
                    format!(
                        "function {function_id} in module {module_id} \
                         failed local validation: {error}"
                    ),
                );
            }

            for parameter in function.signature().parameters() {
                check_type_id(
                    parameter.ty(),
                    definitions,
                    report,
                    "function parameter",
                    function_id.to_string(),
                );

                insert_value_type(
                    parameter.value(),
                    parameter.ty(),
                    value_types,
                    report,
                    "function parameter",
                );
            }

            for ty in function.signature().results() {
                check_type_id(
                    *ty,
                    definitions,
                    report,
                    "function result",
                    function_id.to_string(),
                );
            }
        }
    }
}

/// Validate block argument type identities and index their value types.
fn index_block_values(
    blocks: &[ZuirBlock],
    definitions: &BTreeMap<TypeId, TypeDef>,
    value_types: &mut BTreeMap<ValueId, TypeId>,
    report: &mut TypeVerificationReport,
) {
    for block in blocks {
        for argument in block.arguments() {
            check_block_argument(
                argument,
                block,
                definitions,
                value_types,
                report,
            );
        }
    }
}

fn check_block_argument(
    argument: &BlockArgument,
    block: &ZuirBlock,
    definitions: &BTreeMap<TypeId, TypeDef>,
    value_types: &mut BTreeMap<ValueId, TypeId>,
    report: &mut TypeVerificationReport,
) {
    check_type_id(
        argument.ty(),
        definitions,
        report,
        "block argument",
        block.id().to_string(),
    );

    if let Err(error) = argument.validate() {
        report.push(
            "ZUIR-TYPE-INVALID-BLOCK-ARGUMENT",
            format!(
                "block {} argument {} failed local validation: {error}",
                block.id(),
                argument.value()
            ),
        );
    }

    insert_value_type(
        argument.value(),
        argument.ty(),
        value_types,
        report,
        "block argument",
    );
}

/// Validate operation result types and index result value types.
fn index_operation_results(
    operations: &[ZuirOperation],
    definitions: &BTreeMap<TypeId, TypeDef>,
    value_types: &mut BTreeMap<ValueId, TypeId>,
    report: &mut TypeVerificationReport,
) {
    for operation in operations {
        for result in operation.results() {
            check_result_type(
                result,
                operation,
                definitions,
                value_types,
                report,
            );
        }
    }
}

fn check_result_type(
    result: &OperationValueResult,
    operation: &ZuirOperation,
    definitions: &BTreeMap<TypeId, TypeDef>,
    value_types: &mut BTreeMap<ValueId, TypeId>,
    report: &mut TypeVerificationReport,
) {
    check_type_id(
        result.ty(),
        definitions,
        report,
        "operation result",
        operation.id().to_string(),
    );

    insert_value_type(
        result.value(),
        result.ty(),
        value_types,
        report,
        "operation result",
    );
}

/// Confirm that each operand refers to a value with a known declared type.
///
/// Whether the value dominates the use, is in scope, or is legally owned is
/// checked by the structural, dominance, and ownership verifiers.
fn verify_operation_operands(
    operations: &[ZuirOperation],
    value_types: &BTreeMap<ValueId, TypeId>,
    definitions: &BTreeMap<TypeId, TypeDef>,
    report: &mut TypeVerificationReport,
) {
    for operation in operations {
        for operand in operation.operands() {
            match value_types.get(&operand.value()) {
                Some(ty) => {
                    check_type_id(
                        *ty,
                        definitions,
                        report,
                        "operation operand value",
                        operation.id().to_string(),
                    );
                }

                None => report.push(
                    "ZUIR-TYPE-UNRESOLVED-OPERAND-VALUE",
                    format!(
                        "operation {} references value {} but no declared \
                         type can be resolved for that value",
                        operation.id(),
                        operand.value()
                    ),
                ),
            }
        }
    }
}

/// Validate a TypeId against the supplied canonical definition table.
fn check_type_id(
    ty: TypeId,
    definitions: &BTreeMap<TypeId, TypeDef>,
    report: &mut TypeVerificationReport,
    owner_kind: &'static str,
    owner_id: String,
) {
    if !definitions.contains_key(&ty) {
        report.push(
            "ZUIR-TYPE-UNKNOWN-ID",
            format!(
                "{owner_kind} {owner_id} refers to undefined TypeId {ty}"
            ),
        );
    }
}

/// Index a value's declared type without silently overwriting a previous
/// definition. Global uniqueness remains a structural-verification rule.
fn insert_value_type(
    value: ValueId,
    ty: TypeId,
    value_types: &mut BTreeMap<ValueId, TypeId>,
    report: &mut TypeVerificationReport,
    owner_kind: &'static str,
) {
    report.values_checked += 1;

    if let Some(previous) = value_types.get(&value) {
        if *previous != ty {
            report.push(
                "ZUIR-TYPE-VALUE-TYPE-CONFLICT",
                format!(
                    "{owner_kind} value {value} has conflicting declared \
                     types {previous} and {ty}"
                ),
            );
        } else {
            report.push(
                "ZUIR-TYPE-DUPLICATE-VALUE-DEFINITION",
                format!(
                    "{owner_kind} value {value} has more than one declared \
                     definition with type {ty}"
                ),
            );
        }

        return;
    }

    value_types.insert(value, ty);
}
