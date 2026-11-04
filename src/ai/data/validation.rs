
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Dataset Validation
//!
//! Path: `src/ai/data/validation.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides structural validation for logical AI datasets and their
//! associated schemas, together with an incremental record-validation
//! boundary.
//!
//! Validation is deliberately separated into three levels:
//!
//! 1. Dataset-definition validation.
//! 2. Dataset-schema structural validation.
//! 3. Record validation delegated to a caller-provided validator.
//!
//! This module does not claim to validate canonical field types,
//! expressions, or field constraints independently. Those obligations
//! belong to the canonical semantic type and constraint authorities.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Dataset-definition validation.
//! - Dataset/schema reference consistency.
//! - Dataset field-role consistency.
//! - Deterministic validation reports.
//! - Validation completeness reporting.
//! - Incremental record-validation coordination.
//!
//! This module does not own:
//!
//! - Dataset identity allocation.
//! - Dataset definitions.
//! - Schema definitions.
//! - Canonical type semantics.
//! - Constraint expression evaluation.
//! - Dataset storage.
//! - Dataset loading.
//! - Stream transport.
//! - Batch construction.
//! - Preprocessing execution.
//! - Tensor storage.
//! - Hardware discovery.
//! - Scheduling or placement.
//!
//! ## Canonical dependencies
//!
//! - `crate::ai::data::datasets`
//! - `crate::ai::data::schemas`
//! - `crate::ai::data::streaming`
//! - `crate::ai::error`
//! - `crate::ai::ids`
//!
//! ## POCO-REAF
//!
//! This module imposes no universal maximum on:
//!
//! - Dataset count.
//! - Dataset fields.
//! - Field roles.
//! - Schema fields.
//! - Dataset records.
//! - Record sequence length.
//! - Validation issues.
//! - Tensor rank.
//! - Tensor dimensions.
//! - Dataset duration.
//! - Dataset size.
//!
//! Record validation is incremental. It does not collect the complete
//! dataset or require a fixed-size record buffer.
//!
//! Practical allocation and execution limitations remain explicit.
//! Literal infinite computation or storage is not promised.
//!
//! ## Validation completeness
//!
//! A report distinguishes:
//!
//! - Valid: all checks requested and available to this validator passed.
//! - Invalid: at least one known structural error was found.
//! - Incomplete: additional authoritative analysis or information is needed.
//!
//! Missing schema information is not silently treated as successful
//! schema validation.
//!
//! ## Grammar integration
//!
//! Dataset validation intent is declared through:
//!
//! - `grammar/data/datasets.g4`
//! - `grammar/ai/datasets.g4`
//!
//! The canonical frontend AST and semantic system must resolve source
//! expressions and semantic constraints before invoking this module.
//!
//! This module does not parse grammar or evaluate source expressions.
//!
//! ## Integration
//!
//! - `data/datasets.rs` supplies `DatasetDefinition`.
//! - `data/schemas.rs` supplies `DatasetSchema`.
//! - `data/streaming.rs` supplies `DatasetRecordSource<T>`.
//! - `data/batching.rs` may consume records after validation.
//! - `data/preprocessing.rs` may require validated dataset definitions.
//! - `learning/training.rs` may require validated training datasets.
//! - `learning/federated.rs` may validate locally held dataset streams.
//! - The frontend semantic adapter resolves canonical type and constraint
//!   obligations before claiming complete source-level semantic validity.
//!
//! ## Failure behavior
//!
//! Structural errors are represented in validation reports.
//!
//! Provider and record-validator failures are returned as structured AI
//! errors. A failed record is never silently skipped.
//!
//! Record validation stops at the first operational error. The caller
//! retains ownership of the source and decides whether to retry, resume,
//! or discard it.
//!
//! ## Freeze criteria
//!
//! - Uses existing dataset and schema authorities.
//! - Uses existing streaming abstractions.
//! - No duplicate type system.
//! - No duplicate schema system.
//! - No unsafe code.
//! - No arbitrary data-size ceiling.
//! - No complete-dataset materialization.
//! - Deterministic structural reports.
//! - Incomplete validation cannot be reported as valid.
//! - Record failures are propagated.
//! - All public APIs are documented.

use std::collections::BTreeSet;

use crate::ai::data::datasets::{
    DatasetDefinition,
    DatasetSchemaRef,
};

use crate::ai::data::schemas::DatasetSchema;

use crate::ai::data::streaming::DatasetRecordSource;

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiErrorPhase,
    AiResult,
};

use crate::ai::ids::AiDatasetId;

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn validation_error(
    kind: AiErrorKind,
    message: impl Into<String>,
    dataset_id: Option<AiDatasetId>,
) -> AiError {
    let error = AiError::new(kind, message)
        .with_phase(AiErrorPhase::DataProcessing);

    match dataset_id {
        Some(id) => error.with_dataset(id.to_string()),
        None => error,
    }
}

// ============================================================================
// VALIDATION STATUS
// ============================================================================

/// Overall outcome of a validation operation.
///
/// An incomplete report is not a successful validation certificate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationStatus {
    /// All checks available to this validator passed.
    Valid,

    /// At least one known validation error was found.
    Invalid,

    /// Additional information or authoritative analysis is required.
    Incomplete,
}

// ============================================================================
// VALIDATION SEVERITY
// ============================================================================

/// Severity assigned to a validation issue.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ValidationSeverity {
    /// Informational observation.
    Information,

    /// A known issue that does not independently invalidate the definition.
    Warning,

    /// A structural error that makes the validated definition invalid.
    Error,
}

// ============================================================================
// VALIDATION ISSUE
// ============================================================================

/// One deterministic, machine-readable validation finding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationIssue {
    code: String,
    severity: ValidationSeverity,
    message: String,
    subject: Option<String>,
}

impl ValidationIssue {
    /// Construct a validation issue.
    ///
    /// # Errors
    ///
    /// Returns an input error if the code or message is empty.
    pub fn new(
        code: impl Into<String>,
        severity: ValidationSeverity,
        message: impl Into<String>,
        subject: Option<String>,
    ) -> AiResult<Self> {
        let code = code.into();
        let message = message.into();

        if code.trim().is_empty() {
            return Err(validation_error(
                AiErrorKind::InvalidInput,
                "validation issue code cannot be empty",
                None,
            ));
        }

        if message.trim().is_empty() {
            return Err(validation_error(
                AiErrorKind::InvalidInput,
                "validation issue message cannot be empty",
                None,
            ));
        }

        if code.chars().any(char::is_control)
            || message.chars().any(char::is_control)
        {
            return Err(validation_error(
                AiErrorKind::InvalidInput,
                "validation issue contains control characters",
                None,
            ));
        }

        if let Some(value) = &subject {
            if value.trim().is_empty()
                || value.chars().any(char::is_control)
            {
                return Err(validation_error(
                    AiErrorKind::InvalidInput,
                    "validation issue subject is invalid",
                    None,
                ));
            }
        }

        Ok(Self {
            code,
            severity,
            message,
            subject,
        })
    }

    /// Return the stable issue code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Return the issue severity.
    #[must_use]
    pub const fn severity(&self) -> ValidationSeverity {
        self.severity
    }

    /// Return the human-readable explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return the optional subject reference.
    #[must_use]
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Deterministic report produced by dataset validation.
///
/// Issues retain insertion order. Dataset fields and roles are traversed
/// through their canonical deterministic collections.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationReport {
    dataset_id: AiDatasetId,
    status: ValidationStatus,
    issues: Vec<ValidationIssue>,
}

impl ValidationReport {
    /// Create an empty report for a dataset.
    #[must_use]
    pub fn new(dataset_id: AiDatasetId) -> Self {
        Self {
            dataset_id,
            status: ValidationStatus::Valid,
            issues: Vec::new(),
        }
    }

    /// Return the validated dataset identity.
    #[must_use]
    pub const fn dataset_id(&self) -> AiDatasetId {
        self.dataset_id
    }

    /// Return the overall validation status.
    #[must_use]
    pub const fn status(&self) -> ValidationStatus {
        self.status
    }

    /// Return all validation issues.
    #[must_use]
    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }

    /// Return whether the report is valid.
    ///
    /// Incomplete validation is never considered valid.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        matches!(self.status, ValidationStatus::Valid)
    }

    /// Return whether the report contains errors.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|issue| {
            issue.severity() == ValidationSeverity::Error
        })
    }

    /// Return whether the report contains warnings.
    #[must_use]
    pub fn has_warnings(&self) -> bool {
        self.issues.iter().any(|issue| {
            issue.severity() == ValidationSeverity::Warning
        })
    }

    /// Return whether further validation is required.
    #[must_use]
    pub const fn is_incomplete(&self) -> bool {
        matches!(self.status, ValidationStatus::Incomplete)
    }

    /// Return the number of reported issues.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Add a validation issue and update the report status.
    ///
    /// Allocation is attempted before the report is mutated.
    ///
    /// # Errors
    ///
    /// Returns a resource-exhaustion error if issue storage cannot grow.
    pub fn push(
        &mut self,
        issue: ValidationIssue,
    ) -> AiResult<()> {
        self.issues.try_reserve(1).map_err(|error| {
            AiError::from_source(
                AiErrorKind::ResourceExhausted,
                "unable to grow dataset validation report",
                error,
            )
            .with_phase(AiErrorPhase::DataProcessing)
            .with_dataset(self.dataset_id.to_string())
        })?;

        match issue.severity() {
            ValidationSeverity::Error => {
                self.status = ValidationStatus::Invalid;
            }

            ValidationSeverity::Warning => {
                if self.status == ValidationStatus::Valid {
                    self.status = ValidationStatus::Incomplete;
                }
            }

            ValidationSeverity::Information => {}
        }

        self.issues.push(issue);

        Ok(())
    }

    /// Explicitly mark validation as incomplete.
    ///
    /// This does not overwrite an already-invalid status.
    pub fn mark_incomplete(&mut self) {
        if self.status == ValidationStatus::Valid {
            self.status = ValidationStatus::Incomplete;
        }
    }
}

// ============================================================================
// VALIDATOR
// ============================================================================

/// Stateless dataset validator.
///
/// The validator holds no global mutable state and can be reused for
/// different dataset definitions.
#[derive(Clone, Copy, Debug, Default)]
pub struct DatasetValidator;

impl DatasetValidator {
    /// Create a dataset validator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Validate a dataset definition and its optional resolved schema.
    ///
    /// When a schema reference is present but the resolved schema is not
    /// supplied, the report is marked incomplete.
    ///
    /// This method validates only structural relationships. Canonical
    /// type resolution and field-constraint evaluation remain external
    /// semantic obligations.
    ///
    /// # Errors
    ///
    /// Returns an error if the report cannot grow due to resource
    /// exhaustion.
    pub fn validate_definition(
        &self,
        dataset: &DatasetDefinition,
        schema: Option<&DatasetSchema>,
    ) -> AiResult<ValidationReport> {
        let mut report = ValidationReport::new(dataset.id());

        self.validate_name(dataset, &mut report)?;

        self.validate_sources(dataset, &mut report)?;

        self.validate_schema_reference(
            dataset,
            schema,
            &mut report,
        )?;

        self.validate_field_roles(
            dataset,
            schema,
            &mut report,
        )?;

        Ok(report)
    }

    fn validate_name(
        &self,
        dataset: &DatasetDefinition,
        report: &mut ValidationReport,
    ) -> AiResult<()> {
        if dataset.name().trim().is_empty() {
            report.push(ValidationIssue::new(
                "AI-DATA-001",
                ValidationSeverity::Error,
                "dataset name is empty",
                Some(dataset.id().to_string()),
            )?)?;
        }

        if dataset.name().chars().any(char::is_control) {
            report.push(ValidationIssue::new(
                "AI-DATA-002",
                ValidationSeverity::Error,
                "dataset name contains control characters",
                Some(dataset.id().to_string()),
            )?)?;
        }

        Ok(())
    }

    fn validate_sources(
        &self,
        dataset: &DatasetDefinition,
        report: &mut ValidationReport,
    ) -> AiResult<()> {
        let mut seen = BTreeSet::new();

        for source in dataset.sources() {
            if !seen.insert(source) {
                report.push(ValidationIssue::new(
                    "AI-DATA-003",
                    ValidationSeverity::Error,
                    format!("duplicate dataset source: {source}"),
                    Some(dataset.id().to_string()),
                )?)?;
            }
        }

        // An empty source list is not automatically invalid.
        //
        // A dataset may be supplied by a later binding, generated
        // computation, or another semantic integration.
        if dataset.sources().is_empty() {
            report.push(ValidationIssue::new(
                "AI-DATA-004",
                ValidationSeverity::Warning,
                "dataset has no declared source; source resolution remains outstanding",
                Some(dataset.id().to_string()),
            )?)?;
        }

        Ok(())
    }

    fn validate_schema_reference(
        &self,
        dataset: &DatasetDefinition,
        schema: Option<&DatasetSchema>,
        report: &mut ValidationReport,
    ) -> AiResult<()> {
        match (dataset.schema(), schema) {
            (None, None) => {
                report.push(ValidationIssue::new(
                    "AI-DATA-005",
                    ValidationSeverity::Warning,
                    "dataset has no schema reference",
                    Some(dataset.id().to_string()),
                )?)?;
            }

            (Some(reference), None) => {
                report.push(ValidationIssue::new(
                    "AI-DATA-006",
                    ValidationSeverity::Warning,
                    format!(
                        "schema '{}' has not been resolved",
                        reference
                    ),
                    Some(dataset.id().to_string()),
                )?)?;
            }

            (None, Some(resolved)) => {
                report.push(ValidationIssue::new(
                    "AI-DATA-007",
                    ValidationSeverity::Error,
                    format!(
                        "resolved schema '{}' was supplied for a dataset without a schema reference",
                        resolved.reference()
                    ),
                    Some(dataset.id().to_string()),
                )?)?;
            }

            (Some(reference), Some(resolved)) => {
                self.validate_resolved_schema(
                    dataset.id(),
                    reference,
                    resolved,
                    report,
                )?;
            }
        }

        Ok(())
    }

    fn validate_resolved_schema(
        &self,
        dataset_id: AiDatasetId,
        reference: &DatasetSchemaRef,
        schema: &DatasetSchema,
        report: &mut ValidationReport,
    ) -> AiResult<()> {
        if reference != schema.reference() {
            report.push(ValidationIssue::new(
                "AI-DATA-008",
                ValidationSeverity::Error,
                format!(
                    "dataset schema reference '{}' does not match resolved schema '{}'",
                    reference,
                    schema.reference()
                ),
                Some(dataset_id.to_string()),
            )?)?;

            return Ok(());
        }

        if let Err(error) = schema.validate_structure() {
            report.push(ValidationIssue::new(
                "AI-DATA-009",
                ValidationSeverity::Error,
                format!(
                    "schema '{}' failed structural validation: {}",
                    reference,
                    error
                ),
                Some(dataset_id.to_string()),
            )?)?;
        }

        Ok(())
    }

    fn validate_field_roles(
        &self,
        dataset: &DatasetDefinition,
        schema: Option<&DatasetSchema>,
        report: &mut ValidationReport,
    ) -> AiResult<()> {
        if dataset.field_roles().is_empty() {
            return Ok(());
        }

        let Some(schema) = schema else {
            report.mark_incomplete();

            report.push(ValidationIssue::new(
                "AI-DATA-010",
                ValidationSeverity::Warning,
                "field roles cannot be checked until the dataset schema is resolved",
                Some(dataset.id().to_string()),
            )?)?;

            return Ok(());
        };

        if dataset.schema() != Some(schema.reference()) {
            // A schema mismatch has already been reported by the
            // schema-reference check. Do not validate roles against
            // an unrelated schema.
            return Ok(());
        }

        for field in dataset.field_roles().keys() {
            if schema.field(field.as_str()).is_none() {
                report.push(ValidationIssue::new(
                    "AI-DATA-011",
                    ValidationSeverity::Error,
                    format!(
                        "dataset field role refers to undeclared schema field '{}'",
                        field
                    ),
                    Some(dataset.id().to_string()),
                )?)?;
            }
        }

        Ok(())
    }
}

// ============================================================================
// CONVENIENCE API
// ============================================================================

/// Validate a dataset definition using the stateless validator.
///
/// # Errors
///
/// Returns a resource error if report allocation fails.
pub fn validate_dataset(
    dataset: &DatasetDefinition,
    schema: Option<&DatasetSchema>,
) -> AiResult<ValidationReport> {
    DatasetValidator::new()
        .validate_definition(dataset, schema)
}

// ============================================================================
// INCREMENTAL RECORD VALIDATION
// ============================================================================

/// Summary of an incrementally validated record stream.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordValidationReport {
    dataset_id: AiDatasetId,
    records_validated: u128,
}

impl RecordValidationReport {
    /// Return the dataset identity.
    #[must_use]
    pub const fn dataset_id(&self) -> AiDatasetId {
        self.dataset_id
    }

    /// Return the number of successfully validated records.
    #[must_use]
    pub const fn records_validated(&self) -> u128 {
        self.records_validated
    }
}

/// Validate records from an existing dataset record source.
///
/// The source is consumed incrementally. No complete-dataset collection,
/// fixed-size buffer, or record cloning is required.
///
/// The caller supplies the actual record validator because record values
/// and canonical field semantics are owned by the relevant data and
/// semantic adapters.
///
/// A record-validator error stops processing immediately. The failed
/// record is not silently skipped.
///
/// # Type parameters
///
/// `T` is the record representation.
/// `S` is the existing record-source implementation.
/// `F` is the caller-provided record validator.
///
/// # Errors
///
/// Returns the source error or record-validation error.
///
/// Returns a resource error if the record counter exceeds its
/// representable range.
pub fn validate_record_source<T, S, F>(
    dataset_id: AiDatasetId,
    source: &mut S,
    mut validate_record: F,
) -> AiResult<RecordValidationReport>
where
    S: DatasetRecordSource<T>,
    F: FnMut(u128, &T) -> AiResult<()>,
{
    let mut records_validated = 0_u128;

    loop {
        let record = source.next_record()?;

        let Some(record) = record else {
            break;
        };

        validate_record(records_validated, &record)?;

        records_validated = records_validated.checked_add(1)
            .ok_or_else(|| {
                validation_error(
                    AiErrorKind::ResourceExhausted,
                    "validated record counter exhausted its representable range",
                    Some(dataset_id),
                )
            })?;
    }

    Ok(RecordValidationReport {
        dataset_id,
        records_validated,
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::data::datasets::{
        DatasetFieldRef,
        DatasetFieldRole,
        DatasetSchemaRef,
        DatasetSourceRef,
        DatasetSourceKind,
    };

    use crate::ai::data::schemas::{
        CanonicalTypeRef,
        DatasetSchema,
        SchemaField,
        SchemaVersion,
    };

    use crate::ai::ids::AiDatasetId;

    fn dataset_id(value: u128) -> AiDatasetId {
        AiDatasetId::new(value)
            .expect("test dataset ID must be nonzero")
    }

    fn schema_reference() -> DatasetSchemaRef {
        DatasetSchemaRef::new(
            "test.data",
            "training",
        )
        .expect("valid schema reference")
    }

    fn schema() -> DatasetSchema {
        let mut schema = DatasetSchema::new(
            schema_reference(),
            SchemaVersion::new("1")
                .expect("valid schema version"),
        );

        let field = SchemaField::new(
            "features",
            CanonicalTypeRef::new(
                "zamani.types",
                "Tensor",
            )
            .expect("valid canonical type reference"),
            false,
        )
        .expect("valid schema field");

        schema.add_field(field)
            .expect("field insertion");

        schema
    }

    fn dataset() -> DatasetDefinition {
        let source = DatasetSourceRef::new(
            DatasetSourceKind::generated(),
            "test.generator",
            "training",
        )
        .expect("valid source reference");

        let mut dataset = DatasetDefinition::new(
            dataset_id(1),
            "training",
        )
        .expect("valid dataset");

        dataset.set_schema(schema_reference());

        dataset.add_source(source)
            .expect("source insertion");

        dataset
    }

    #[test]
    fn valid_dataset_and_schema_pass_structural_validation() {
        let dataset = dataset();
        let schema = schema();

        let report = validate_dataset(
            &dataset,
            Some(&schema),
        )
        .expect("validation must run");

        assert_eq!(report.status(), ValidationStatus::Valid);
        assert!(report.is_valid());
        assert!(report.issues().is_empty());
    }

    #[test]
    fn missing_schema_is_incomplete_not_valid() {
        let dataset = dataset();

        let report = validate_dataset(
            &dataset,
            None,
        )
        .expect("validation must run");

        assert_eq!(report.status(), ValidationStatus::Incomplete);
        assert!(!report.is_valid());
        assert!(report.has_warnings());
    }

    #[test]
    fn missing_dataset_schema_is_incomplete() {
        let dataset = DatasetDefinition::new(
            dataset_id(1),
            "unresolved",
        )
        .expect("valid dataset");

        let report = validate_dataset(
            &dataset,
            None,
        )
        .expect("validation must run");

        assert!(report.is_incomplete());
    }

    #[test]
    fn schema_reference_mismatch_is_invalid() {
        let mut dataset = dataset();

        dataset.set_schema(
            DatasetSchemaRef::new(
                "other.namespace",
                "different",
            )
            .expect("valid reference"),
        );

        let report = validate_dataset(
            &dataset,
            Some(&schema()),
        )
        .expect("validation must run");

        assert_eq!(report.status(), ValidationStatus::Invalid);
        assert!(report.has_errors());
    }

    #[test]
    fn schema_without_dataset_reference_is_invalid() {
        let dataset = DatasetDefinition::new(
            dataset_id(1),
            "unbound",
        )
        .expect("valid dataset");

        let report = validate_dataset(
            &dataset,
            Some(&schema()),
        )
        .expect("validation must run");

        assert_eq!(report.status(), ValidationStatus::Invalid);
    }

    #[test]
    fn undeclared_field_role_is_invalid() {
        let mut dataset = dataset();

        dataset.assign_role(
            DatasetFieldRef::new("missing")
                .expect("valid field reference"),
            DatasetFieldRole::feature(),
        )
        .expect("role insertion");

        let report = validate_dataset(
            &dataset,
            Some(&schema()),
        )
        .expect("validation must run");

        assert_eq!(report.status(), ValidationStatus::Invalid);

        assert!(report.issues().iter().any(|issue| {
            issue.code() == "AI-DATA-011"
        }));
    }

    #[test]
    fn declared_field_role_is_valid() {
        let mut dataset = dataset();

        dataset.assign_role(
            DatasetFieldRef::new("features")
                .expect("valid field reference"),
            DatasetFieldRole::feature(),
        )
        .expect("role insertion");

        let report = validate_dataset(
            &dataset,
            Some(&schema()),
        )
        .expect("validation must run");

        assert!(report.is_valid());
    }

    #[test]
    fn unresolved_schema_does_not_certify_field_roles() {
        let mut dataset = dataset();

        dataset.assign_role(
            DatasetFieldRef::new("features")
                .expect("valid field reference"),
            DatasetFieldRole::feature(),
        )
        .expect("role insertion");

        let report = validate_dataset(
            &dataset,
            None,
        )
        .expect("validation must run");

        assert!(report.is_incomplete());
        assert!(!report.is_valid());
    }

    #[test]
    fn empty_source_list_is_reported_as_incomplete() {
        let dataset = DatasetDefinition::new(
            dataset_id(1),
            "generated_later",
        )
        .expect("valid dataset");

        let report = validate_dataset(
            &dataset,
            None,
        )
        .expect("validation must run");

        assert!(report.is_incomplete());
        assert!(report.has_warnings());
    }

    #[test]
    fn validation_issue_rejects_empty_codes() {
        assert!(
            ValidationIssue::new(
                "",
                ValidationSeverity::Error,
                "invalid",
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn validation_report_never_promotes_invalid_to_valid() {
        let mut report = ValidationReport::new(dataset_id(1));

        report.push(
            ValidationIssue::new(
                "TEST-ERROR",
                ValidationSeverity::Error,
                "invalid structure",
                None,
            )
            .expect("valid issue"),
        )
        .expect("report insertion");

        report.mark_incomplete();

        assert_eq!(report.status(), ValidationStatus::Invalid);
        assert!(!report.is_valid());
    }

    #[test]
    fn record_source_is_validated_incrementally() {
        let dataset_id = dataset_id(7);

        let mut records = vec![10_u64, 20, 30].into_iter();

        let mut source = crate::ai::data::streaming::IteratorSource::new(
            records.by_ref(),
        );

        let report = validate_record_source(
            dataset_id,
            &mut source,
            |ordinal, value| {
                if *value == 0 {
                    return Err(AiError::new(
                        AiErrorKind::DataValidationFailure,
                        format!("record {ordinal} is zero"),
                    )
                    .with_phase(AiErrorPhase::DataProcessing)
                    .with_dataset(dataset_id.to_string()));
                }

                Ok(())
            },
        )
        .expect("all records must validate");

        assert_eq!(report.dataset_id(), dataset_id);
        assert_eq!(report.records_validated(), 3);
    }

    #[test]
    fn record_validation_stops_on_first_invalid_record() {
        let id = dataset_id(8);

        let mut source = crate::ai::data::streaming::IteratorSource::new(
            vec![1_u64, 0, 3].into_iter(),
        );

        let result = validate_record_source(
            id,
            &mut source,
            |ordinal, value| {
                if *value == 0 {
                    return Err(AiError::new(
                        AiErrorKind::DataValidationFailure,
                        format!("invalid record at {ordinal}"),
                    ));
                }

                Ok(())
            },
        );

        assert!(result.is_err());

        // The source is not silently resumed or drained.
        assert_eq!(source.next_record().unwrap(), Some(3));
    }

    #[test]
    fn supports_large_logical_dataset_identity() {
        let id = dataset_id(u128::MAX);

        let report = ValidationReport::new(id);

        assert_eq!(report.dataset_id(), id);
    }
}
