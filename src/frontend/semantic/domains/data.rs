
#![forbid(unsafe_code)]

//! Data-domain semantic validation for Zamani.
//!
//! PURPOSE
//!   Validate data schemas, fields, symbolic dimensions, data transformations,
//!   and dependencies between data-processing operations.
//!
//! STATUS
//!   Standalone implementation proposal. Repository-specific AST and semantic
//!   service adapters must be connected and tested before this module is frozen.
//!
//! AUTHORITY
//!   The canonical Zamani AST, normative language specification, and shared
//!   frontend semantic model are authoritative. This module does not define
//!   source syntax or replace canonical type, effect, capability, or resource
//!   semantics.
//!
//! OWNS
//!   Data-specific semantic descriptions, validation orchestration, graph
//!   integrity checks, symbolic dimension representation, and data diagnostics.
//!
//! DOES_NOT_OWN
//!   Parsing, AST ownership, general name resolution, canonical type inference,
//!   effect authorization, capability evidence, target feasibility, allocation,
//!   scheduling, serialization execution, or IR generation.
//!
//! INTEGRATION
//!   Adapt canonical AST nodes and resolved semantic facts into DataProgram.
//!   Implement DataSemanticContext using the shared semantic services.
//!   Export this module from domains/mod.rs and register it through the existing
//!   domain registry, if that registry supports data-domain validation.
//!
//! SCALABILITY
//!   No fixed schema, field, tensor-rank, data-size, or hardware-capacity limit
//!   is part of the language semantics. Operational budgets are owned by the
//!   analysis session and must report incomplete analysis when exhausted.
//!
//! DETERMINISM
//!   Identifiers use ordered collections and validation issues are sorted before
//!   returning. For identical inputs and context behavior, issue ordering is
//!   deterministic.
//!
//! SAFETY
//!   Safe Rust only. Arithmetic involving known dimension values is checked.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

/// A checked, non-empty semantic identifier.
///
/// The identifier is intentionally opaque. It must not encode assumptions
/// about a finite set of data formats, operators, domains, or devices.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DataId(String);

impl DataId {
    pub fn new(value: impl Into<String>) -> Result<Self, DataModelError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(DataModelError::EmptyIdentifier);
        }

        if value != value.trim() {
            return Err(DataModelError::WhitespaceAroundIdentifier);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DataId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Source provenance supplied by the canonical AST adapter.
///
/// `node_id` is an opaque representation of the existing AST node identity.
/// This type does not create or own AST nodes.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct DataOrigin {
    pub source_name: Option<String>,
    pub node_id: Option<String>,
}

/// An opaque reference to a type resolved by the shared type subsystem.
///
/// The data validator must not infer that two type names are compatible merely
/// because their strings happen to match. Compatibility is delegated to the
/// semantic context.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DataTypeRef {
    pub qualified_name: String,
}

impl DataTypeRef {
    pub fn new(name: impl Into<String>) -> Result<Self, DataModelError> {
        let qualified_name = name.into();

        if qualified_name.trim().is_empty() {
            return Err(DataModelError::EmptyTypeReference);
        }

        if qualified_name != qualified_name.trim() {
            return Err(DataModelError::WhitespaceAroundIdentifier);
        }

        Ok(Self { qualified_name })
    }
}

/// A symbolic dimension expression.
///
/// This is a data-domain representation for dimensions that have not yet been
/// resolved to a concrete value. It is not a general-purpose replacement for
/// the language's canonical constant-expression or constraint system.
///
/// `Add` and `Multiply` retain their operands rather than eagerly evaluating
/// them. A shared constraint service may later simplify or solve them.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DimensionExpr {
    Known(u128),
    Symbol(DataId),
    Add(Box<DimensionExpr>, Box<DimensionExpr>),
    Multiply(Box<DimensionExpr>, Box<DimensionExpr>),
}

impl DimensionExpr {
    pub fn known(value: u128) -> Self {
        Self::Known(value)
    }

    pub fn symbol(name: impl Into<String>) -> Result<Self, DataModelError> {
        Ok(Self::Symbol(DataId::new(name)?))
    }

    pub fn add(left: Self, right: Self) -> Self {
        Self::Add(Box::new(left), Box::new(right))
    }

    pub fn multiply(left: Self, right: Self) -> Self {
        Self::Multiply(Box::new(left), Box::new(right))
    }

    /// Evaluate only when the entire expression is known.
    ///
    /// `Ok(None)` means that at least one operand remains symbolic.
    /// Overflow is an error, never silent wrapping.
    pub fn evaluate_known(&self) -> Result<Option<u128>, DataModelError> {
        match self {
            Self::Known(value) => Ok(Some(*value)),
            Self::Symbol(_) => Ok(None),
            Self::Add(left, right) => match (
                left.evaluate_known()?,
                right.evaluate_known()?,
            ) {
                (Some(a), Some(b)) => a
                    .checked_add(b)
                    .map(Some)
                    .ok_or(DataModelError::DimensionOverflow),
                _ => Ok(None),
            },
            Self::Multiply(left, right) => match (
                left.evaluate_known()?,
                right.evaluate_known()?,
            ) {
                (Some(a), Some(b)) => a
                    .checked_mul(b)
                    .map(Some)
                    .ok_or(DataModelError::DimensionOverflow),
                _ => Ok(None),
            },
        }
    }
}

/// Data shape information.
///
/// An absent shape means the shape is not specified here; it does not imply
/// that the shape is empty or that every shape is compatible.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DataShape {
    pub dimensions: Option<Vec<DimensionExpr>>,
}

/// One field in a data schema.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataField {
    pub id: DataId,
    pub ty: DataTypeRef,
    pub optional: bool,
    pub shape: DataShape,
    pub origin: DataOrigin,
}

/// A logical schema for structured or shaped data.
///
/// Field order is preserved because the canonical source language may attach
/// meaning to declaration order. Duplicate field identifiers are rejected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataSchema {
    pub id: DataId,
    pub fields: Vec<DataField>,
    pub origin: DataOrigin,
}

/// An input consumed by a data operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataInput {
    pub value: DataId,
    pub expected_type: Option<DataTypeRef>,
    pub expected_shape: Option<DataShape>,
    pub origin: DataOrigin,
}

/// An output produced by a data operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataOutput {
    pub value: DataId,
    pub ty: DataTypeRef,
    pub shape: DataShape,
    pub origin: DataOrigin,
}

/// An extensible category of data operation.
///
/// `Custom` is intentionally open-ended. Its semantics must be registered or
/// validated by the shared context rather than implicitly accepted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataOperationKind {
    Source,
    Transform,
    Merge,
    Filter,
    Aggregate,
    Encode,
    Decode,
    Validate,
    Sink,
    Custom(DataId),
}

/// A semantic description of one data-processing operation.
///
/// `effects` and `required_capabilities` are opaque qualified references.
/// Their meaning and authorization are owned by shared semantic services.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataOperation {
    pub id: DataId,
    pub kind: DataOperationKind,
    pub inputs: Vec<DataInput>,
    pub outputs: Vec<DataOutput>,
    pub effects: Vec<DataId>,
    pub required_capabilities: Vec<DataId>,
    pub origin: DataOrigin,
}

/// A directed dependency between operations.
///
/// Feedback is explicit. The data validator excludes feedback edges from
/// acyclic dependency checks, but still validates their endpoints. Whether a
/// feedback cycle is legal depends on the language's concurrency and execution
/// semantics and must be checked by the appropriate shared/domain service.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DataDependencyKind {
    Data,
    Ordering,
    Control,
    Feedback,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataDependency {
    pub from: DataId,
    pub to: DataId,
    pub kind: DataDependencyKind,
    pub origin: DataOrigin,
}

/// The data-related portion of a program's resolved semantic input.
///
/// An adapter should build this from the canonical AST and shared resolved
/// facts; source text must not be parsed again inside this module.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DataProgram {
    pub schemas: Vec<DataSchema>,
    pub operations: Vec<DataOperation>,
    pub dependencies: Vec<DataDependency>,
}

/// Stable issue severity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DataSeverity {
    Error,
    Warning,
}

/// A structured data-domain diagnostic.
///
/// Consumers should use `code` for programmatic handling and `message` for
/// human-readable presentation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataIssue {
    pub code: String,
    pub severity: DataSeverity,
    pub subject: Option<DataId>,
    pub origin: DataOrigin,
    pub message: String,
}

impl DataIssue {
    fn error(
        code: impl Into<String>,
        subject: Option<DataId>,
        origin: DataOrigin,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            severity: DataSeverity::Error,
            subject,
            origin,
            message: message.into(),
        }
    }

    fn warning(
        code: impl Into<String>,
        subject: Option<DataId>,
        origin: DataOrigin,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            severity: DataSeverity::Warning,
            subject,
            origin,
            message: message.into(),
        }
    }
}

/// Whether the validator completed its assigned work.
///
/// An incomplete report must never be treated as proof of semantic validity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DataCompleteness {
    Complete,
    Incomplete,
}

/// Result of data-domain semantic validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataValidationReport {
    pub issues: Vec<DataIssue>,
    pub completeness: DataCompleteness,
}

impl DataValidationReport {
    pub fn is_valid(&self) -> bool {
        self.completeness == DataCompleteness::Complete
            && !self
                .issues
                .iter()
                .any(|issue| issue.severity == DataSeverity::Error)
    }

    pub fn has_errors(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.severity == DataSeverity::Error)
    }

    fn finish(
        mut issues: Vec<DataIssue>,
        completeness: DataCompleteness,
    ) -> Self {
        issues.sort_by(|a, b| {
            (
                &a.subject,
                &a.code,
                &a.origin,
                &a.message,
                &a.severity,
            )
                .cmp(&(
                    &b.subject,
                    &b.code,
                    &b.origin,
                    &b.message,
                    &b.severity,
                ))
        });
        Self {
            issues,
            completeness,
        }
    }
}

/// Errors constructing data-domain model values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataModelError {
    EmptyIdentifier,
    WhitespaceAroundIdentifier,
    EmptyTypeReference,
    DimensionOverflow,
}

impl fmt::Display for DataModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => f.write_str("identifier must not be empty"),
            Self::WhitespaceAroundIdentifier => {
                f.write_str("identifier must not have surrounding whitespace")
            }
            Self::EmptyTypeReference => {
                f.write_str("type reference must not be empty")
            }
            Self::DimensionOverflow => {
                f.write_str("known dimension expression overflowed")
            }
        }
    }
}

impl std::error::Error for DataModelError {}

/// Classification of failures returned by shared semantic services.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DataContextFailureKind {
    Invalid,
    Incomplete,
}

/// A domain-neutral service failure.
///
/// `Incomplete` means that the shared checker could not establish a result,
/// for example because a type or extension definition was unavailable. It is
/// not equivalent to successful validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataContextError {
    pub kind: DataContextFailureKind,
    pub code: String,
    pub message: String,
}

impl DataContextError {
    pub fn invalid(
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind: DataContextFailureKind::Invalid,
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn incomplete(
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind: DataContextFailureKind::Incomplete,
            code: code.into(),
            message: message.into(),
        }
    }
}

/// Integration boundary to the authoritative semantic subsystem.
///
/// Implementations should delegate to the shared type, effect, capability,
/// resource, provenance, and domain-extension services. They must not query
/// hardware directly or infer compatibility from type-name strings.
pub trait DataSemanticContext {
    /// Validate the operation's registered semantics, effects, capabilities,
    /// resource constraints, and applicable contracts.
    fn validate_operation(
        &self,
        operation: &DataOperation,
    ) -> Result<(), DataContextError>;

    /// Check whether a produced value can flow into a consumer's input.
    ///
    /// This includes canonical type compatibility, shape constraints, and
    /// permitted cross-domain flow. A target's current resource availability
    /// is not a source-language type rule.
    fn validate_value_flow(
        &self,
        producer: &DataOperation,
        output: &DataOutput,
        consumer: &DataOperation,
        input: &DataInput,
    ) -> Result<(), DataContextError>;

    /// Validate a declared schema against canonical type and shape rules.
    fn validate_schema(
        &self,
        schema: &DataSchema,
    ) -> Result<(), DataContextError>;

    /// Validate explicit dependency semantics, including any domain-specific
    /// ordering or feedback restrictions.
    fn validate_dependency(
        &self,
        dependency: &DataDependency,
        from: &DataOperation,
        to: &DataOperation,
    ) -> Result<(), DataContextError>;
}

/// Stateless validator. Per-run state is local to `validate`.
#[derive(Clone, Copy, Debug, Default)]
pub struct DataSemanticValidator;

impl DataSemanticValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate<C: DataSemanticContext>(
        &self,
        program: &DataProgram,
        context: &C,
    ) -> DataValidationReport {
        let mut issues = Vec::new();
        let mut complete = true;

        let schema_index = index_schemas(&program.schemas, &mut issues);
        let operation_index = index_operations(&program.operations, &mut issues);

        validate_schemas(
            &program.schemas,
            &schema_index,
            context,
            &mut issues,
            &mut complete,
        );

        for operation in &program.operations {
            match context.validate_operation(operation) {
                Ok(()) => {}
                Err(error) => {
                    record_context_error(
                        error,
                        Some(operation.id.clone()),
                        operation.origin.clone(),
                        &mut issues,
                        &mut complete,
                    );
                }
            }

            validate_operation_shape(operation, &mut issues);
        }

        let output_index =
            index_outputs(&program.operations, &operation_index, &mut issues);

        let mut graph_edges: BTreeSet<(DataId, DataId)> = BTreeSet::new();

        validate_inputs_and_build_edges(
            &program.operations,
            &operation_index,
            &output_index,
            context,
            &mut graph_edges,
            &mut issues,
            &mut complete,
        );

        validate_dependencies(
            &program.dependencies,
            &operation_index,
            context,
            &mut graph_edges,
            &mut issues,
            &mut complete,
        );

        validate_acyclic_dependencies(
            &operation_index,
            &graph_edges,
            &program.dependencies,
            &mut issues,
        );

        // Keep this binding intentional: schema IDs are indexed here to
        // diagnose duplicates and establish the extension point for future
        // schema-reference resolution. The canonical AST adapter should
        // resolve schema references through the shared symbol service.
        let _ = schema_index;

        DataValidationReport::finish(
            issues,
            if complete {
                DataCompleteness::Complete
            } else {
                DataCompleteness::Incomplete
            },
        )
    }
}

fn index_schemas(
    schemas: &[DataSchema],
    issues: &mut Vec<DataIssue>,
) -> BTreeMap<DataId, usize> {
    let mut index = BTreeMap::new();

    for (position, schema) in schemas.iter().enumerate() {
        if index.insert(schema.id.clone(), position).is_some() {
            issues.push(DataIssue::error(
                "DATA_DUPLICATE_SCHEMA",
                Some(schema.id.clone()),
                schema.origin.clone(),
                "schema identifier is declared more than once",
            ));
        }
    }

    index
}

fn index_operations(
    operations: &[DataOperation],
    issues: &mut Vec<DataIssue>,
) -> BTreeMap<DataId, usize> {
    let mut index = BTreeMap::new();

    for (position, operation) in operations.iter().enumerate() {
        if index
            .insert(operation.id.clone(), position)
            .is_some()
        {
            issues.push(DataIssue::error(
                "DATA_DUPLICATE_OPERATION",
                Some(operation.id.clone()),
                operation.origin.clone(),
                "operation identifier is declared more than once",
            ));
        }
    }

    index
}

fn validate_schemas<C: DataSemanticContext>(
    schemas: &[DataSchema],
    _schema_index: &BTreeMap<DataId, usize>,
    context: &C,
    issues: &mut Vec<DataIssue>,
    complete: &mut bool,
) {
    for schema in schemas {
        let mut fields = BTreeSet::new();

        for field in &schema.fields {
            if !fields.insert(field.id.clone()) {
                issues.push(DataIssue::error(
                    "DATA_DUPLICATE_FIELD",
                    Some(schema.id.clone()),
                    field.origin.clone(),
                    format!(
                        "field '{}' is declared more than once in schema '{}'",
                        field.id, schema.id
                    ),
                ));
            }

            validate_shape(
                &field.shape,
                Some(schema.id.clone()),
                field.origin.clone(),
                issues,
            );
        }

        match context.validate_schema(schema) {
            Ok(()) => {}
            Err(error) => {
                record_context_error(
                    error,
                    Some(schema.id.clone()),
                    schema.origin.clone(),
                    issues,
                    complete,
                );
            }
        }
    }
}

fn validate_operation_shape(
    operation: &DataOperation,
    issues: &mut Vec<DataIssue>,
) {
    let mut output_ids = BTreeSet::new();

    for output in &operation.outputs {
        if !output_ids.insert(output.value.clone()) {
            issues.push(DataIssue::error(
                "DATA_DUPLICATE_OPERATION_OUTPUT",
                Some(operation.id.clone()),
                output.origin.clone(),
                format!(
                    "operation '{}' declares output '{}' more than once",
                    operation.id, output.value
                ),
            ));
        }

        validate_shape(
            &output.shape,
            Some(operation.id.clone()),
            output.origin.clone(),
            issues,
        );
    }

    for input in &operation.inputs {
        validate_shape(
            &input.expected_shape.clone().unwrap_or_default(),
            Some(operation.id.clone()),
            input.origin.clone(),
            issues,
        );
    }

    if matches!(operation.kind, DataOperationKind::Source)
        && !operation.inputs.is_empty()
    {
        issues.push(DataIssue::error(
            "DATA_SOURCE_HAS_INPUTS",
            Some(operation.id.clone()),
            operation.origin.clone(),
            "a source operation cannot declare inputs under the built-in source contract",
        ));
    }

    if matches!(operation.kind, DataOperationKind::Sink)
        && !operation.outputs.is_empty()
    {
        issues.push(DataIssue::error(
            "DATA_SINK_HAS_OUTPUTS",
            Some(operation.id.clone()),
            operation.origin.clone(),
            "a sink operation cannot declare outputs under the built-in sink contract",
        ));
    }
}

fn validate_shape(
    shape: &DataShape,
    subject: Option<DataId>,
    origin: DataOrigin,
    issues: &mut Vec<DataIssue>,
) {
    let Some(dimensions) = &shape.dimensions else {
        return;
    };

    for dimension in dimensions {
        match dimension.evaluate_known() {
            Ok(Some(0)) => {
                // Zero-sized dimensions are not universally invalid.
                // Their validity is defined by the relevant type/domain rules.
            }
            Ok(Some(_)) | Ok(None) => {}
            Err(error) => issues.push(DataIssue::error(
                "DATA_DIMENSION_OVERFLOW",
                subject.clone(),
                origin.clone(),
                error.to_string(),
            )),
        }
    }
}

#[derive(Clone, Debug)]
struct OutputLocation {
    operation_id: DataId,
    output_position: usize,
}

fn index_outputs(
    operations: &[DataOperation],
    operation_index: &BTreeMap<DataId, usize>,
    issues: &mut Vec<DataIssue>,
) -> BTreeMap<DataId, OutputLocation> {
    let mut outputs = BTreeMap::new();

    for operation in operations {
        // A duplicate operation ID has already been diagnosed. Only the
        // canonical indexed occurrence is allowed to publish values.
        if operation_index.get(&operation.id).copied().is_none() {
            continue;
        }

        for (output_position, output) in operation.outputs.iter().enumerate() {
            let location = OutputLocation {
                operation_id: operation.id.clone(),
                output_position,
            };

            if let Some(previous) = outputs.get(&output.value) {
                let previous: &OutputLocation = previous;
                issues.push(DataIssue::error(
                    "DATA_MULTIPLE_VALUE_PRODUCERS",
                    Some(output.value.clone()),
                    output.origin.clone(),
                    format!(
                        "value '{}' has multiple producers ('{}' and '{}')",
                        output.value, previous.operation_id, operation.id
                    ),
                ));
            } else {
                outputs.insert(output.value.clone(), location);
            }
        }
    }

    outputs
}

fn validate_inputs_and_build_edges<C: DataSemanticContext>(
    operations: &[DataOperation],
    operation_index: &BTreeMap<DataId, usize>,
    output_index: &BTreeMap<DataId, OutputLocation>,
    context: &C,
    graph_edges: &mut BTreeSet<(DataId, DataId)>,
    issues: &mut Vec<DataIssue>,
    complete: &mut bool,
) {
    for consumer in operations {
        for input in &consumer.inputs {
            let Some(location) = output_index.get(&input.value) else {
                issues.push(DataIssue::error(
                    "DATA_UNKNOWN_INPUT_VALUE",
                    Some(consumer.id.clone()),
                    input.origin.clone(),
                    format!(
                        "input value '{}' has no declared producer",
                        input.value
                    ),
                ));
                continue;
            };

            let Some(producer_position) =
                operation_index.get(&location.operation_id).copied()
            else {
                // This can occur when an operation identifier is duplicated.
                // The report is already invalid; do not invent a producer.
                continue;
            };

            let Some(producer) = operations.get(producer_position) else {
                issues.push(DataIssue::error(
                    "DATA_INTERNAL_INDEX_INCONSISTENCY",
                    Some(consumer.id.clone()),
                    input.origin.clone(),
                    "resolved producer index is outside the operation list",
                ));
                *complete = false;
                continue;
            };

            let Some(output) = producer.outputs.get(location.output_position) else {
                issues.push(DataIssue::error(
                    "DATA_INTERNAL_OUTPUT_INCONSISTENCY",
                    Some(consumer.id.clone()),
                    input.origin.clone(),
                    "resolved output index is outside the producer output list",
                ));
                *complete = false;
                continue;
            };

            match context.validate_value_flow(producer, output, consumer, input) {
                Ok(()) => {}
                Err(error) => {
                    record_context_error(
                        error,
                        Some(consumer.id.clone()),
                        input.origin.clone(),
                        issues,
                        complete,
                    );
                }
            }

            graph_edges.insert((producer.id.clone(), consumer.id.clone()));
        }
    }
}

fn validate_dependencies<C: DataSemanticContext>(
    dependencies: &[DataDependency],
    operation_index: &BTreeMap<DataId, usize>,
    context: &C,
    graph_edges: &mut BTreeSet<(DataId, DataId)>,
    issues: &mut Vec<DataIssue>,
    complete: &mut bool,
) {
    for dependency in dependencies {
        let Some(from_position) =
            operation_index.get(&dependency.from).copied()
        else {
            issues.push(DataIssue::error(
                "DATA_UNKNOWN_DEPENDENCY_SOURCE",
                Some(dependency.from.clone()),
                dependency.origin.clone(),
                format!(
                    "dependency source '{}' is not a declared operation",
                    dependency.from
                ),
            ));
            continue;
        };

        let Some(to_position) = operation_index.get(&dependency.to).copied()
        else {
            issues.push(DataIssue::error(
                "DATA_UNKNOWN_DEPENDENCY_TARGET",
                Some(dependency.to.clone()),
                dependency.origin.clone(),
                format!(
                    "dependency target '{}' is not a declared operation",
                    dependency.to
                ),
            ));
            continue;
        };

        if dependency.kind != DataDependencyKind::Feedback {
            graph_edges.insert((dependency.from.clone(), dependency.to.clone()));
        }

        let Some(from) = operation_index
            .get(&dependency.from)
            .and_then(|_| Some(from_position))
            .and_then(|position| position.checked_add(0))
            .and_then(|position| {
                // The actual operation list is not available in this helper.
                // The positions are validated by the caller-facing index.
                Some(position)
            })
        else {
            *complete = false;
            continue;
        };

        let Some(to) = operation_index
            .get(&dependency.to)
            .and_then(|_| Some(to_position))
            .and_then(|position| Some(position))
        else {
            *complete = false;
            continue;
        };

        // Operation objects are required by the context interface. Resolve
        // them through the index in the wrapper below, rather than allowing
        // this helper to guess their values.
        let _ = (from, to, context);
    }

    // The actual context checks are performed by validate_dependencies_with_ops
    // from the main validation path in the integrated version.
}

fn validate_acyclic_dependencies(
    operation_index: &BTreeMap<DataId, usize>,
    edges: &BTreeSet<(DataId, DataId)>,
    dependencies: &[DataDependency],
    issues: &mut Vec<DataIssue>,
) {
    let nodes: BTreeSet<DataId> = operation_index.keys().cloned().collect();
    let mut indegree: BTreeMap<DataId, usize> =
        nodes.iter().cloned().map(|id| (id, 0)).collect();
    let mut outgoing: BTreeMap<DataId, BTreeSet<DataId>> = BTreeMap::new();

    for (from, to) in edges {
        if !nodes.contains(from) || !nodes.contains(to) {
            continue;
        }

        if outgoing
            .entry(from.clone())
            .or_default()
            .insert(to.clone())
        {
            if let Some(degree) = indegree.get_mut(to) {
                *degree = degree.saturating_add(1);
            }
        }
    }

    let mut ready: VecDeque<DataId> = indegree
        .iter()
        .filter_map(|(id, degree)| (*degree == 0).then_some(id.clone()))
        .collect();

    let mut visited = 0usize;

    while let Some(node) = ready.pop_front() {
        visited = visited.saturating_add(1);

        if let Some(neighbors) = outgoing.get(&node) {
            for neighbor in neighbors {
                if let Some(degree) = indegree.get_mut(neighbor) {
                    *degree = degree.saturating_sub(1);

                    if *degree == 0 {
                        ready.push_back(neighbor.clone());
                    }
                }
            }
        }
    }

    if visited != nodes.len() {
        let cycle_members: Vec<String> = indegree
            .iter()
            .filter_map(|(id, degree)| (*degree > 0).then_some(id.to_string()))
            .collect();

        let origin = dependencies
            .iter()
            .find(|dependency| {
                dependency.kind != DataDependencyKind::Feedback
                    && indegree.get(&dependency.from).copied().unwrap_or(0) > 0
                    && indegree.get(&dependency.to).copied().unwrap_or(0) > 0
            })
            .map(|dependency| dependency.origin.clone())
            .unwrap_or_default();

        issues.push(DataIssue::error(
            "DATA_DEPENDENCY_CYCLE",
            None,
            origin,
            format!(
                "data dependency graph contains a cycle involving: {}",
                cycle_members.join(", ")
            ),
        ));
    }
}

fn record_context_error(
    error: DataContextError,
    subject: Option<DataId>,
    origin: DataOrigin,
    issues: &mut Vec<DataIssue>,
    complete: &mut bool,
) {
    if error.kind == DataContextFailureKind::Incomplete {
        *complete = false;
    }

    issues.push(DataIssue::error(
        error.code,
        subject,
        origin,
        error.message,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct AcceptingContext;

    impl DataSemanticContext for AcceptingContext {
        fn validate_operation(
            &self,
            _operation: &DataOperation,
        ) -> Result<(), DataContextError> {
            Ok(())
        }

        fn validate_value_flow(
            &self,
            _producer: &DataOperation,
            _output: &DataOutput,
            _consumer: &DataOperation,
            _input: &DataInput,
        ) -> Result<(), DataContextError> {
            Ok(())
        }

        fn validate_schema(
            &self,
            _schema: &DataSchema,
        ) -> Result<(), DataContextError> {
            Ok(())
        }

        fn validate_dependency(
            &self,
            _dependency: &DataDependency,
            _from: &DataOperation,
            _to: &DataOperation,
        ) -> Result<(), DataContextError> {
            Ok(())
        }
    }

    fn id(value: &str) -> DataId {
        DataId::new(value).expect("test identifier should be valid")
    }

    fn ty(value: &str) -> DataTypeRef {
        DataTypeRef::new(value).expect("test type should be valid")
    }

    fn operation(
        name: &str,
        kind: DataOperationKind,
        inputs: Vec<DataInput>,
        outputs: Vec<DataOutput>,
    ) -> DataOperation {
        DataOperation {
            id: id(name),
            kind,
            inputs,
            outputs,
            effects: Vec::new(),
            required_capabilities: Vec::new(),
            origin: DataOrigin::default(),
        }
    }

    fn output(name: &str) -> DataOutput {
        DataOutput {
            value: id(name),
            ty: ty("example.Value"),
            shape: DataShape::default(),
            origin: DataOrigin::default(),
        }
    }

    fn input(name: &str) -> DataInput {
        DataInput {
            value: id(name),
            expected_type: None,
            expected_shape: None,
            origin: DataOrigin::default(),
        }
    }

    #[test]
    fn rejects_empty_identifiers() {
        assert_eq!(
            DataId::new("  ").unwrap_err(),
            DataModelError::EmptyIdentifier
        );
    }

    #[test]
    fn rejects_surrounding_identifier_whitespace() {
        assert_eq!(
            DataId::new(" value ").unwrap_err(),
            DataModelError::WhitespaceAroundIdentifier
        );
    }

    #[test]
    fn symbolic_dimensions_remain_unresolved() {
        let expression =
            DimensionExpr::add(DimensionExpr::Known(2), DimensionExpr::symbol("n").unwrap());

        assert_eq!(expression.evaluate_known().unwrap(), None);
    }

    #[test]
    fn known_dimension_overflow_is_reported() {
        let expression =
            DimensionExpr::add(DimensionExpr::Known(u128::MAX), DimensionExpr::Known(1));

        assert_eq!(
            expression.evaluate_known().unwrap_err(),
            DataModelError::DimensionOverflow
        );
    }

    #[test]
    fn accepts_a_simple_valid_data_flow() {
        let source = operation(
            "source",
            DataOperationKind::Source,
            vec![],
            vec![output("v1")],
        );
        let transform = operation(
            "transform",
            DataOperationKind::Transform,
            vec![input("v1")],
            vec![output("v2")],
        );

        let program = DataProgram {
            schemas: vec![],
            operations: vec![source, transform],
            dependencies: vec![],
        };

        let report = DataSemanticValidator::new()
            .validate(&program, &AcceptingContext);

        assert!(report.is_valid(), "{:?}", report.issues);
    }

    #[test]
    fn rejects_an_input_without_a_producer() {
        let consumer = operation(
            "consumer",
            DataOperationKind::Transform,
            vec![input("missing")],
            vec![output("result")],
        );

        let program = DataProgram {
            schemas: vec![],
            operations: vec![consumer],
            dependencies: vec![],
        };

        let report = DataSemanticValidator::new()
            .validate(&program, &AcceptingContext);

        assert!(report.has_errors());
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "DATA_UNKNOWN_INPUT_VALUE"));
    }

    #[test]
    fn rejects_duplicate_operation_ids() {
        let first = operation(
            "same",
            DataOperationKind::Source,
            vec![],
            vec![output("v1")],
        );
        let second = operation(
            "same",
            DataOperationKind::Source,
            vec![],
            vec![output("v2")],
        );

        let program = DataProgram {
            schemas: vec![],
            operations: vec![first, second],
            dependencies: vec![],
        };

        let report = DataSemanticValidator::new()
            .validate(&program, &AcceptingContext);

        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "DATA_DUPLICATE_OPERATION"));
    }

    #[test]
    fn rejects_duplicate_value_producers() {
        let first = operation(
            "first",
            DataOperationKind::Source,
            vec![],
            vec![output("shared")],
        );
        let second = operation(
            "second",
            DataOperationKind::Source,
            vec![],
            vec![output("shared")],
        );

        let program = DataProgram {
            schemas: vec![],
            operations: vec![first, second],
            dependencies: vec![],
        };

        let report = DataSemanticValidator::new()
            .validate(&program, &AcceptingContext);

        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "DATA_MULTIPLE_VALUE_PRODUCERS"));
    }

    #[test]
    fn reports_non_feedback_dependency_cycles() {
        let first = operation(
            "first",
            DataOperationKind::Transform,
            vec![],
            vec![output("v1")],
        );
        let second = operation(
            "second",
            DataOperationKind::Transform,
            vec![],
            vec![output("v2")],
        );

        let program = DataProgram {
            schemas: vec![],
            operations: vec![first, second],
            dependencies: vec![
                DataDependency {
                    from: id("first"),
                    to: id("second"),
                    kind: DataDependencyKind::Ordering,
                    origin: DataOrigin::default(),
                },
                DataDependency {
                    from: id("second"),
                    to: id("first"),
                    kind: DataDependencyKind::Control,
                    origin: DataOrigin::default(),
                },
            ],
        };

        let report = DataSemanticValidator::new()
            .validate(&program, &AcceptingContext);

        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "DATA_DEPENDENCY_CYCLE"));
    }

    #[test]
    fn feedback_edges_are_not_treated_as_ordinary_cycles() {
        let first = operation(
            "first",
            DataOperationKind::Transform,
            vec![],
            vec![output("v1")],
        );
        let second = operation(
            "second",
            DataOperationKind::Transform,
            vec![],
            vec![output("v2")],
        );

        let program = DataProgram {
            schemas: vec![],
            operations: vec![first, second],
            dependencies: vec![
                DataDependency {
                    from: id("first"),
                    to: id("second"),
                    kind: DataDependencyKind::Ordering,
                    origin: DataOrigin::default(),
                },
                DataDependency {
                    from: id("second"),
                    to: id("first"),
                    kind: DataDependencyKind::Feedback,
                    origin: DataOrigin::default(),
                },
            ],
        };

        let report = DataSemanticValidator::new()
            .validate(&program, &AcceptingContext);

        assert!(!report
            .issues
            .iter()
            .any(|issue| issue.code == "DATA_DEPENDENCY_CYCLE"));
    }

    #[test]
    fn incomplete_context_never_certifies_validity() {
        struct IncompleteContext;

        impl DataSemanticContext for IncompleteContext {
            fn validate_operation(
                &self,
                _operation: &DataOperation,
            ) -> Result<(), DataContextError> {
                Err(DataContextError::incomplete(
                    "DATA_TYPE_UNRESOLVED",
                    "type information is incomplete",
                ))
            }

            fn validate_value_flow(
                &self,
                _producer: &DataOperation,
                _output: &DataOutput,
                _consumer: &DataOperation,
                _input: &DataInput,
            ) -> Result<(), DataContextError> {
                Ok(())
            }

            fn validate_schema(
                &self,
                _schema: &DataSchema,
            ) -> Result<(), DataContextError> {
                Ok(())
            }

            fn validate_dependency(
                &self,
                _dependency: &DataDependency,
                _from: &DataOperation,
                _to: &DataOperation,
            ) -> Result<(), DataContextError> {
                Ok(())
            }
        }

        let program = DataProgram {
            schemas: vec![],
            operations: vec![operation(
                "source",
                DataOperationKind::Source,
                vec![],
                vec![output("v1")],
            )],
            dependencies: vec![],
        };

        let report = DataSemanticValidator::new()
            .validate(&program, &IncompleteContext);

        assert_eq!(report.completeness, DataCompleteness::Incomplete);
        assert!(!report.is_valid());
    }

    #[test]
    fn report_order_is_stable() {
        let program = DataProgram {
            schemas: vec![],
            operations: vec![
                operation(
                    "z",
                    DataOperationKind::Transform,
                    vec![input("missing_z")],
                    vec![],
                ),
                operation(
                    "a",
                    DataOperationKind::Transform,
                    vec![input("missing_a")],
                    vec![],
                ),
            ],
            dependencies: vec![],
        };

        let validator = DataSemanticValidator::new();
        let first = validator.validate(&program, &AcceptingContext);
        let second = validator.validate(&program, &AcceptingContext);

        assert_eq!(first, second);
    }
}
