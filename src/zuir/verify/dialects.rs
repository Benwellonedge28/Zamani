
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Dialect Verification
//!
//! Path: `src/zuir/verify/dialects.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: safe Rust only.
//!
//! PURPOSE
//!
//! Verify that operations in a ZUIR view refer to registered dialects,
//! declared operation exports, and available, valid operation schemas.
//! Check schema-level operand/result/region cardinality, attributes, and
//! declared effects wherever the current IR exposes enough information.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/dialect/definition.rs`
//! - `src/zuir/dialect/operation.rs`
//! - `src/zuir/dialect/registry.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `src/zuir/schema/types.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/dialects/dialect.g4`
//! - `grammar/specification/semantic-model.md`
//!
//! OWNS
//!
//! - Cross-checking operation names against the dialect registry.
//! - Cross-checking manifest exports against operation schemas.
//! - Schema-level operation conformance checks.
//! - Structured, deterministic findings for dialect verification.
//!
//! DOES NOT OWN
//!
//! - Parsing, AST construction, or source name resolution.
//! - Dialect registration, manifest dependency resolution, or trust approval.
//! - Semantic type inference, SSA, dominance, or global def-use verification.
//! - Loading or executing extension code.
//! - Physical target feasibility, resource allocation, routing, or scheduling.
//! - Runtime availability or proof that a lowering implementation exists.
//!
//! INTEGRATION
//!
//! `DialectRegistry` owns registered manifests. `OperationDefinition` owns
//! operation schemas. These are distinct repositories of information, so
//! this verifier receives both explicitly.
//!
//! The operation's `QualifiedName` is interpreted as the concatenation of
//! the registered dialect name segments and the export name segments.
//! The longest registered dialect prefix wins. This permits qualified
//! dialect names and qualified export names without a hard-coded namespace
//! list.
//!
//! `verify_structural` should run before this verifier. Type, ownership,
//! control-flow, effect, and whole-program checks remain separate stages.
//!
//! SCALABILITY AND POCO-REAF
//!
//! No universal number of dialects, operations, operands, results, regions,
//! devices, qubits, processors, nodes, or tensor dimensions is imposed here.
//! Collections scale with input and available resources. Arithmetic that
//! combines cardinalities is checked. Finite memory and execution time are
//! still finite; resource exhaustion must be handled by the caller's
//! configured compilation budget.
//!
//! UNKNOWN INFORMATION
//!
//! An unknown dialect, undeclared operation, or missing required schema is
//! an error. An unknown effect contract or an attribute type whose symbolic
//! constraints cannot be resolved is reported as Deferred, never as proof
//! of conformance. A deferred result is not safe authorization to execute.
//!
//! FREEZE CRITERIA
//!
//! - Registry manifests and detailed schemas remain separate authorities.
//! - Unknown operations cannot pass as verified operations.
//! - No operation list or hardware capacity is hard-coded.
//! - Findings use stable codes and deterministic traversal.
//! - The caller can distinguish invalid from incomplete verification.
//! - No unsafe Rust is used.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::dialect::definition::{
    DialectExportName, DialectName, ExportKind,
};
use crate::zuir::dialect::operation::OperationDefinition;
use crate::zuir::dialect::registry::{
    DialectRegistry, ExportReference,
};
use crate::zuir::ir::operation::ZuirOperation;
use crate::zuir::schema::attributes::{
    AttributeKey, UnknownAttributePolicy,
};

/// Severity of one dialect-verification finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DialectIssueSeverity {
    /// The operation or schema violates a known contract.
    Error,
    /// Verification cannot establish conformance with the available data.
    Deferred,
}

/// One actionable dialect-verification finding.
///
/// `code` is a stable machine-readable identifier. Consumers should not parse
/// the explanatory message as a stable API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialectIssue {
    severity: DialectIssueSeverity,
    code: &'static str,
    operation: Option<String>,
    message: String,
}

impl DialectIssue {
    fn new(
        severity: DialectIssueSeverity,
        code: &'static str,
        operation: Option<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code,
            operation,
            message: message.into(),
        }
    }

    /// Return the issue severity.
    #[must_use]
    pub const fn severity(&self) -> DialectIssueSeverity {
        self.severity
    }

    /// Return the stable issue code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.code
    }

    /// Return the canonical operation name, if this finding concerns one.
    #[must_use]
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// Return the explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for DialectIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

/// Outcome of verifying operations against registered dialect contracts.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DialectVerificationReport {
    issues: Vec<DialectIssue>,
    operations_checked: usize,
    schemas_checked: usize,
}

impl DialectVerificationReport {
    /// Return all findings in deterministic verification order.
    #[must_use]
    pub fn issues(&self) -> &[DialectIssue] {
        &self.issues
    }

    /// Return true when no known contract violation was found.
    ///
    /// A valid report may still be incomplete. Use `is_complete` before
    /// treating verification as fully resolved.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self
            .issues
            .iter()
            .any(|issue| issue.severity == DialectIssueSeverity::Error)
    }

    /// Return true only when there are neither errors nor deferred findings.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.issues.is_empty()
    }

    /// Return the number of findings.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Return the number of error findings.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == DialectIssueSeverity::Error)
            .count()
    }

    /// Return the number of deferred findings.
    #[must_use]
    pub fn deferred_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == DialectIssueSeverity::Deferred)
            .count()
    }

    /// Return the number of operation instances inspected.
    #[must_use]
    pub const fn operations_checked(&self) -> usize {
        self.operations_checked
    }

    /// Return the number of valid operation schemas inspected.
    #[must_use]
    pub const fn schemas_checked(&self) -> usize {
        self.schemas_checked
    }

    fn push(
        &mut self,
        severity: DialectIssueSeverity,
        code: &'static str,
        operation: Option<String>,
        message: impl Into<String>,
    ) {
        self.issues.push(DialectIssue::new(
            severity,
            code,
            operation,
            message,
        ));
    }

    fn error(
        &mut self,
        code: &'static str,
        operation: Option<String>,
        message: impl Into<String>,
    ) {
        self.push(DialectIssueSeverity::Error, code, operation, message);
    }

    fn deferred(
        &mut self,
        code: &'static str,
        operation: Option<String>,
        message: impl Into<String>,
    ) {
        self.push(
            DialectIssueSeverity::Deferred,
            code,
            operation,
            message,
        );
    }
}

/// Verify operations against an immutable registry and an explicit schema set.
///
/// `schemas` contains detailed operation definitions. It is intentionally
/// separate from `registry`: the current registry owns dialect manifests and
/// export declarations, not executable or detailed operation schemas.
///
/// The verifier is read-only. It neither mutates the IR nor registers
/// dialects, loads extension code, or authorizes execution.
///
/// # Operation-name convention
///
/// An operation name is a `QualifiedName` whose leading segments identify a
/// registered dialect and whose remaining segments identify an operation
/// export. The longest matching registered dialect prefix is selected.
///
/// For example, a dialect named `quantum::standard` may own an export named
/// `gate::apply`, represented by `quantum::standard::gate::apply`.
///
/// # Completeness
///
/// A report with errors is invalid. A report with deferred findings is
/// incomplete. Neither condition authorizes execution or semantic-changing
/// optimization.
#[must_use]
pub fn verify_dialects(
    operations: &[ZuirOperation],
    registry: &DialectRegistry,
    schemas: &[OperationDefinition],
) -> DialectVerificationReport {
    let mut report = DialectVerificationReport::default();

    // Build a deterministic schema index. Invalid and duplicate schemas are
    // reported rather than silently replacing another definition.
    let schema_index = index_schemas(schemas, registry, &mut report);

    for operation in operations {
        report.operations_checked += 1;
        let operation_name = operation.name().canonical_string();

        if let Err(error) = operation.validate() {
            report.error(
                "ZUIR-DIALECT-INVALID-OPERATION",
                Some(operation_name.clone()),
                format!("operation-local validation failed: {error}"),
            );
            // Local corruption can make subsequent checks misleading.
            continue;
        }

        let Some((dialect_name, export_name)) =
            resolve_operation_identity(operation, registry)
        else {
            report.error(
                "ZUIR-DIALECT-UNKNOWN-DIALECT",
                Some(operation_name.clone()),
                format!(
                    "operation `{operation_name}` has no registered dialect prefix"
                ),
            );
            continue;
        };

        let export_reference = ExportReference::new(
            dialect_name.clone(),
            export_name.clone(),
            ExportKind::Operation,
        );

        if let Err(error) = registry.resolve_export(&export_reference) {
            report.error(
                "ZUIR-DIALECT-UNDECLARED-OPERATION",
                Some(operation_name.clone()),
                format!(
                    "operation `{operation_name}` is not declared by dialect \
                     `{dialect_name}`: {error}"
                ),
            );
            continue;
        }

        let key = (dialect_name.clone(), export_name.clone());
        let Some(schema) = schema_index.get(&key).copied() else {
            report.error(
                "ZUIR-DIALECT-MISSING-OPERATION-SCHEMA",
                Some(operation_name.clone()),
                format!(
                    "dialect `{dialect_name}` declares operation \
                     `{export_name}`, but its detailed schema is unavailable"
                ),
            );
            continue;
        };

        verify_operation_against_schema(
            operation,
            schema,
            &mut report,
        );
    }

    report
}

/// Index and validate all supplied schemas.
///
/// A schema must correspond to a declared operation export. Duplicate schema
/// identities are errors; the first definition is retained only to keep
/// subsequent diagnostics deterministic. A report containing such an error
/// is not valid.
fn index_schemas<'a>(
    schemas: &'a [OperationDefinition],
    registry: &DialectRegistry,
    report: &mut DialectVerificationReport,
) -> BTreeMap<(DialectName, DialectExportName), &'a OperationDefinition> {
    let mut index = BTreeMap::new();

    for schema in schemas {
        let key = (schema.dialect().clone(), schema.name().clone());
        let display_name = format!("{}::{}", schema.dialect(), schema.name());

        if let Err(error) = schema.validate() {
            report.error(
                "ZUIR-DIALECT-INVALID-SCHEMA",
                Some(display_name),
                format!("operation schema failed validation: {error}"),
            );
            continue;
        }

        if !registry.has_export(
            schema.dialect(),
            schema.name(),
            ExportKind::Operation,
        ) {
            report.error(
                "ZUIR-DIALECT-SCHEMA-WITHOUT-EXPORT",
                Some(display_name.clone()),
                format!(
                    "schema `{display_name}` has no matching operation export \
                     in the registered dialect manifest"
                ),
            );
            continue;
        }

        if index.contains_key(&key) {
            report.error(
                "ZUIR-DIALECT-DUPLICATE-SCHEMA",
                Some(display_name.clone()),
                format!("more than one schema was supplied for `{display_name}`"),
            );
            continue;
        }

        index.insert(key, schema);
        report.schemas_checked += 1;
    }

    index
}

/// Resolve an operation's qualified name to the longest registered dialect
/// prefix and its remaining export name.
///
/// This avoids hard-coded dialect names and supports multi-segment dialect
/// and export names. Invalid prefixes or empty suffixes do not resolve.
fn resolve_operation_identity(
    operation: &ZuirOperation,
    registry: &DialectRegistry,
) -> Option<(DialectName, DialectExportName)> {
    let segments = operation.name().segments();
    let mut best: Option<(usize, DialectName)> = None;

    for dialect in registry.names() {
        let dialect_segments: Vec<&str> =
            dialect.to_string().split("::").collect();

        if dialect_segments.len() >= segments.len() {
            continue;
        }

        let matches = dialect_segments
            .iter()
            .zip(segments.iter())
            .all(|(expected, actual)| *expected == actual);

        if matches
            && best
                .as_ref()
                .is_none_or(|(best_len, _)| dialect_segments.len() > *best_len)
        {
            best = Some((dialect_segments.len(), dialect.clone()));
        }
    }

    let (prefix_len, dialect) = best?;
    let export_text = segments[prefix_len..].join("::");

    let export = DialectExportName::parse(&export_text).ok()?;
    Some((dialect, export))
}

fn verify_operation_against_schema(
    operation: &ZuirOperation,
    schema: &OperationDefinition,
    report: &mut DialectVerificationReport,
) {
    let operation_name = operation.name().canonical_string();

    if let Err(error) = schema.validate_operand_count(operation.operand_count()) {
        report.error(
            "ZUIR-DIALECT-OPERAND-CARDINALITY",
            Some(operation_name.clone()),
            format!("operand count violates the schema: {error}"),
        );
    }

    if let Err(error) = schema.validate_result_count(operation.result_count()) {
        report.error(
            "ZUIR-DIALECT-RESULT-CARDINALITY",
            Some(operation_name.clone()),
            format!("result count violates the schema: {error}"),
        );
    }

    verify_region_cardinality(operation, schema, report);
    verify_effects(operation, schema, report);
    verify_attributes(operation, schema, report);
}

/// Verify aggregate region cardinality.
///
/// The current canonical operation representation stores a flat list of
/// region IDs and does not retain the schema's region-group assignment.
/// Consequently, this check can enforce the aggregate minimum and maximum,
/// but it cannot prove which region belongs to which named group. That
/// stronger check requires group-role metadata in the IR or builder.
fn verify_region_cardinality(
    operation: &ZuirOperation,
    schema: &OperationDefinition,
    report: &mut DialectVerificationReport,
) {
    let operation_name = operation.name().canonical_string();
    let mut minimum = 0usize;
    let mut maximum: Option<usize> = Some(0);

    for group in schema.regions() {
        let cardinality = group.cardinality();

        let Some(next_minimum) = minimum.checked_add(cardinality.minimum()) else {
            report.error(
                "ZUIR-DIALECT-REGION-CARDINALITY-OVERFLOW",
                Some(operation_name),
                "sum of minimum region cardinalities overflows usize",
            );
            return;
        };
        minimum = next_minimum;

        maximum = match (maximum, cardinality.maximum()) {
            (Some(current), Some(additional)) => {
                match current.checked_add(additional) {
                    Some(total) => Some(total),
                    None => {
                        report.error(
                            "ZUIR-DIALECT-REGION-CARDINALITY-OVERFLOW",
                            Some(operation_name),
                            "sum of maximum region cardinalities overflows usize",
                        );
                        return;
                    }
                }
            }
            _ => None,
        };
    }

    let actual = operation.region_count();

    if actual < minimum || maximum.is_some_and(|limit| actual > limit) {
        report.error(
            "ZUIR-DIALECT-REGION-CARDINALITY",
            Some(operation_name.clone()),
            format!(
                "operation has {actual} regions; schema permits {minimum}..={maximum:?}"
            ),
        );
    }
}

fn verify_effects(
    operation: &ZuirOperation,
    schema: &OperationDefinition,
    report: &mut DialectVerificationReport,
) {
    let operation_name = operation.name().canonical_string();

    match schema.effects().effects() {
        Some(expected) if expected == operation.effects() => {}
        Some(expected) => {
            report.error(
                "ZUIR-DIALECT-EFFECT-MISMATCH",
                Some(operation_name),
                format!(
                    "operation effect set {:?} does not match schema effect set {:?}",
                    operation.effects(),
                    expected
                ),
            );
        }
        None => {
            report.deferred(
                "ZUIR-DIALECT-EFFECTS-UNKNOWN",
                Some(operation_name),
                "schema does not establish a complete effect contract; \
                 this operation must not be assumed pure",
            );
        }
    }
}

fn verify_attributes(
    operation: &ZuirOperation,
    schema: &OperationDefinition,
    report: &mut DialectVerificationReport,
) {
    let operation_name = operation.name().canonical_string();
    let attributes = schema.attributes();

    // Validate present attributes against their declarations.
    for (key, value) in operation.attributes() {
        match attributes.get(key) {
            Some(definition) => match value.conforms_to(definition.value_type()) {
                Ok(true) => {}
                Ok(false) => report.error(
                    "ZUIR-DIALECT-ATTRIBUTE-TYPE",
                    Some(operation_name.clone()),
                    format!(
                        "attribute `{key}` does not conform to its declared schema type"
                    ),
                ),
                Err(error) => report.deferred(
                    "ZUIR-DIALECT-ATTRIBUTE-TYPE-UNRESOLVED",
                    Some(operation_name.clone()),
                    format!(
                        "cannot establish the type of attribute `{key}`: {error}"
                    ),
                ),
            },
            None => match attributes.unknown_policy() {
                UnknownAttributePolicy::Reject => report.error(
                    "ZUIR-DIALECT-UNKNOWN-ATTRIBUTE",
                    Some(operation_name.clone()),
                    format!("attribute `{key}` is not declared by the operation schema"),
                ),
                UnknownAttributePolicy::Preserve | UnknownAttributePolicy::Ignore => {
                    // The policy explicitly permits unknown attributes.
                }
            },
        }
    }

    // Required attributes must be present.
    for (key, definition) in attributes.iter() {
        if definition.required() && !operation.attributes().contains_key(key) {
            report.error(
                "ZUIR-DIALECT-MISSING-ATTRIBUTE",
                Some(operation_name.clone()),
                format!("required attribute `{key}` is missing"),
            );
        }
    }

    // The current IR stores one value per attribute key. If a schema permits
    // repeated values, this representation cannot encode that multiplicity.
    // A future IR/schema integration must either represent repeated values or
    // reject multi-valued schema contracts explicitly.
    for (key, definition) in attributes.iter() {
        if operation.attributes().contains_key(key)
            && definition.multiplicity()
                == crate::zuir::schema::attributes::AttributeMultiplicity::Repeated
        {
            report.deferred(
                "ZUIR-DIALECT-ATTRIBUTE-MULTIPLICITY-UNREPRESENTABLE",
                Some(operation_name.clone()),
                format!(
                    "schema attribute `{key}` permits repeated values, but \
                     this IR operation representation stores only one value per key"
                ),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::dialect::operation::Cardinality;

    #[test]
    fn cardinality_uses_checked_arithmetic() {
        let cardinalities = [
            Cardinality::new(1, Some(2)).expect("valid cardinality"),
            Cardinality::new(0, None).expect("valid cardinality"),
        ];

        let mut minimum = 0usize;
        let mut maximum: Option<usize> = Some(0);

        for cardinality in cardinalities {
            minimum = minimum
                .checked_add(cardinality.minimum())
                .expect("small minimum does not overflow");

            maximum = match (maximum, cardinality.maximum()) {
                (Some(current), Some(additional)) => {
                    Some(current.checked_add(additional).expect("no overflow"))
                }
                _ => None,
            };
        }

        assert_eq!(minimum, 1);
        assert_eq!(maximum, None);
    }

    #[test]
    fn report_is_incomplete_when_a_finding_is_deferred() {
        let mut report = DialectVerificationReport::default();

        report.deferred(
            "ZUIR-DIALECT-TEST-DEFERRED",
            None,
            "test finding",
        );

        assert!(report.is_valid());
        assert!(!report.is_complete());
        assert_eq!(report.deferred_count(), 1);
        assert_eq!(report.error_count(), 0);
    }
}
