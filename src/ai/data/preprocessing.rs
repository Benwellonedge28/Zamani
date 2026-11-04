
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Dataset Preprocessing
//!
//! Path: `src/ai/data/preprocessing.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines target-independent preprocessing operations and ordered
//! preprocessing plans for logical AI datasets.
//!
//! A preprocessing plan describes transformations to be applied to a
//! dataset. It does not execute those transformations.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Preprocessing operation references.
//! - Opaque canonical expression references.
//! - Preprocessing step definitions.
//! - Ordered preprocessing plans.
//! - Step metadata.
//! - Declared input and output schema boundaries.
//! - Structural preprocessing validation.
//!
//! This module does not own:
//!
//! - Dataset identity allocation.
//! - Dataset definitions.
//! - Canonical schema definitions.
//! - Canonical expression parsing or evaluation.
//! - Tensor storage.
//! - Dataset storage.
//! - Transformation execution.
//! - Streaming or asynchronous execution.
//! - Physical partitioning.
//! - Distributed scheduling.
//! - Hardware discovery or selection.
//! - General-purpose pipeline execution.
//!
//! ## Canonical dependencies
//!
//! - `crate::ai::error`
//! - `crate::ai::ids`
//! - `crate::ai::data::datasets`
//! - `crate::ai::data::schemas`
//!
//! `DatasetSchemaRef` is owned by `data/datasets.rs`.
//! Schema definitions and schema semantics are owned by `data/schemas.rs`.
//! Dataset identities are owned by `ai/ids.rs`.
//!
//! ## Grammar integration
//!
//! Source-level syntax is owned by:
//!
//! - `grammar/ai/datasets.g4`
//! - `grammar/data/datasets.g4`
//!
//! Relevant AI grammar boundaries include:
//!
//! - `datasetTransformDeclaration`
//! - `datasetProjectionDeclaration`
//! - `datasetFilterDeclaration`
//! - `datasetMapDeclaration`
//! - `datasetJoinDeclaration`
//! - `datasetUnionDeclaration`
//! - `datasetBatchDeclaration`
//! - `datasetWindowDeclaration`
//! - `datasetSamplingDeclaration`
//! - `datasetOrderingDeclaration`
//! - `datasetShuffleDeclaration`
//!
//! The frontend adapter resolves source expressions and operation names.
//! This module accepts only already-resolved opaque references.
//!
//! It does not parse Zamani source text.
//!
//! ## Execution boundary
//!
//! This module describes preprocessing intent.
//!
//! An execution adapter must resolve operation references, validate
//! their implementations, negotiate capabilities and execute the
//! resulting operations through the appropriate runtime.
//!
//! A plan being structurally valid does not mean that an implementation
//! exists or that execution is feasible on a particular target.
//!
//! ## POCO-REAF and scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Preprocessing steps.
//! - Operation arguments.
//! - Metadata entries.
//! - Dataset records.
//! - Dataset size.
//! - Tensor rank.
//! - Tensor dimensions.
//! - Pipeline depth.
//!
//! Collections grow according to available resources.
//!
//! Fallible vector reservations are used where the standard library
//! exposes them. Allocation failure is returned as an AI error.
//!
//! Rust's standard collections and the process allocator cannot guarantee
//! recovery from every possible system-wide out-of-memory condition.
//!
//! Literal infinity is not promised.
//!
//! ## Determinism
//!
//! Step order is declaration order.
//! Metadata entries preserve insertion order.
//! Validation is deterministic.
//!
//! No global mutable state, random values, wall-clock dependency,
//! hardware inspection or implicit execution is introduced.
//!
//! ## Integration
//!
//! - `data/datasets.rs` supplies dataset identity and schema references.
//! - `data/schemas.rs` owns schema definitions and compatibility semantics.
//! - `data/streaming.rs` supplies incremental record sources.
//! - `data/provenance.rs` owns dataset lineage integration.
//! - The frontend semantic adapter resolves grammar expressions.
//! - The AI execution adapter resolves and executes operation references.
//! - `learning/training.rs` may consume validated preprocessing plans.
//! - `src/distributed/` owns distributed execution and coordination.
//! - `src/runtime/` owns runtime lifecycle and resource management.
//!
//! ## Freeze criteria
//!
//! - No unsafe code.
//! - No arbitrary transformation-count limit.
//! - No physical storage or hardware assumptions.
//! - No duplicate schema or expression authority.
//! - Transformation order is preserved.
//! - Duplicate step names are rejected.
//! - Declared schema incompatibilities are detected.
//! - Unresolved schema boundaries are reported rather than guessed.
//! - No transformation is reported as executed merely because a plan
//!   was constructed or validated.

use std::fmt;

use crate::ai::data::datasets::{
    DatasetDefinition,
    DatasetSchemaRef,
};
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

fn preprocessing_error(
    kind: AiErrorKind,
    message: &'static str,
    dataset_id: Option<AiDatasetId>,
) -> AiError {
    let error = AiError::new(kind, message)
        .with_phase(AiErrorPhase::DataProcessing);

    match dataset_id {
        Some(id) => error.with_dataset(id.to_string()),
        None => error,
    }
}

fn validate_text(
    description: &'static str,
    value: &str,
) -> AiResult<()> {
    if value.trim().is_empty() {
        return Err(AiError::new(
            AiErrorKind::InvalidInput,
            description,
        )
        .with_phase(AiErrorPhase::DataProcessing));
    }

    if value.chars().any(char::is_control) {
        return Err(AiError::new(
            AiErrorKind::InvalidInput,
            description,
        )
        .with_phase(AiErrorPhase::DataProcessing));
    }

    Ok(())
}

fn validate_qualified_component(
    description: &'static str,
    value: &str,
) -> AiResult<()> {
    validate_text(description, value)?;

    if value.chars().any(char::is_whitespace) {
        return Err(AiError::new(
            AiErrorKind::InvalidInput,
            description,
        )
        .with_phase(AiErrorPhase::DataProcessing));
    }

    Ok(())
}

fn allocation_error(
    error: std::collections::TryReserveError,
) -> AiError {
    AiError::new(
        AiErrorKind::ResourceExhausted,
        "unable to reserve memory for preprocessing data",
    )
    .with_phase(AiErrorPhase::DataProcessing)
    .with_source(error)
}

// ============================================================================
// OPERATION REFERENCE
// ============================================================================

/// A target-independent reference to a preprocessing operation.
///
/// The namespace identifies the authority providing the operation.
/// The name identifies the operation within that authority.
///
/// Operation names are open-world strings. This module does not impose
/// a closed list of preprocessing algorithms.
///
/// Examples of possible operation references include:
///
/// - `zamani.data::project`
/// - `zamani.data::filter`
/// - `zamani.data::map`
/// - `zamani.data::join`
/// - `zamani.data::normalize`
/// - `extension.example::custom_transform`
///
/// These examples are references, not built-in implementations.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PreprocessingOperationRef {
    namespace: String,
    name: String,
}

impl PreprocessingOperationRef {
    /// Construct a validated operation reference.
    ///
    /// # Errors
    ///
    /// Returns an error if the namespace or operation name is empty,
    /// contains control characters, or violates the qualified-component
    /// whitespace rule.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> AiResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        validate_qualified_component(
            "preprocessing operation namespace is invalid",
            &namespace,
        )?;

        validate_qualified_component(
            "preprocessing operation name is invalid",
            &name,
        )?;

        Ok(Self {
            namespace,
            name,
        })
    }

    /// Return the operation namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the operation name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the canonical display representation.
    ///
    /// This is a display representation, not a serialization protocol.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}::{}", self.namespace, self.name)
    }
}

impl fmt::Display for PreprocessingOperationRef {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}::{}",
            self.namespace,
            self.name
        )
    }
}

// ============================================================================
// CANONICAL EXPRESSION REFERENCE
// ============================================================================

/// An opaque reference to an expression resolved by the canonical frontend.
///
/// This type does not parse, evaluate, type-check or execute expressions.
///
/// The referenced expression must be resolved by the frontend or the
/// appropriate semantic integration before execution.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PreprocessingExpressionRef(String);

impl PreprocessingExpressionRef {
    /// Construct an opaque expression reference.
    ///
    /// The supplied value must identify an expression through the
    /// canonical frontend integration.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty value or control characters.
    pub fn new(
        reference: impl Into<String>,
    ) -> AiResult<Self> {
        let reference = reference.into();

        validate_text(
            "preprocessing expression reference is invalid",
            &reference,
        )?;

        Ok(Self(reference))
    }

    /// Return the opaque expression reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PreprocessingExpressionRef {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// PREPROCESSING STEP
// ============================================================================

/// One ordered preprocessing operation.
///
/// A step describes an operation reference and its canonical expression
/// arguments.
///
/// Optional schema references describe the declared input and output
/// schema boundaries. They do not replace canonical schema definitions.
///
/// Step names are unique within their containing plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreprocessingStep {
    name: String,

    operation: PreprocessingOperationRef,

    arguments: Vec<PreprocessingExpressionRef>,

    input_schema: Option<DatasetSchemaRef>,

    output_schema: Option<DatasetSchemaRef>,

    metadata: Vec<(String, String)>,
}

impl PreprocessingStep {
    /// Construct a preprocessing step.
    ///
    /// # Errors
    ///
    /// Returns an error if the step name is empty or contains control
    /// characters.
    pub fn new(
        name: impl Into<String>,
        operation: PreprocessingOperationRef,
    ) -> AiResult<Self> {
        let name = name.into();

        validate_text(
            "preprocessing step name is invalid",
            &name,
        )?;

        Ok(Self {
            name,
            operation,
            arguments: Vec::new(),
            input_schema: None,
            output_schema: None,
            metadata: Vec::new(),
        })
    }

    /// Return the step name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the operation reference.
    #[must_use]
    pub fn operation(&self) -> &PreprocessingOperationRef {
        &self.operation
    }

    /// Return operation arguments in declaration order.
    #[must_use]
    pub fn arguments(&self) -> &[PreprocessingExpressionRef] {
        &self.arguments
    }

    /// Return the optional declared input schema.
    #[must_use]
    pub fn input_schema(&self) -> Option<&DatasetSchemaRef> {
        self.input_schema.as_ref()
    }

    /// Return the optional declared output schema.
    #[must_use]
    pub fn output_schema(&self) -> Option<&DatasetSchemaRef> {
        self.output_schema.as_ref()
    }

    /// Return metadata in insertion order.
    #[must_use]
    pub fn metadata(&self) -> &[(String, String)] {
        &self.metadata
    }

    /// Add an expression argument.
    ///
    /// Arguments are retained in their original declaration order.
    ///
    /// # Errors
    ///
    /// Returns an error if memory reservation fails.
    pub fn add_argument(
        &mut self,
        argument: PreprocessingExpressionRef,
    ) -> AiResult<()> {
        self.arguments
            .try_reserve(1)
            .map_err(allocation_error)?;

        self.arguments.push(argument);

        Ok(())
    }

    /// Associate an input schema reference.
    ///
    /// This does not establish that the referenced schema exists.
    pub fn set_input_schema(
        &mut self,
        schema: DatasetSchemaRef,
    ) {
        self.input_schema = Some(schema);
    }

    /// Associate an output schema reference.
    ///
    /// This does not establish that the referenced schema exists.
    pub fn set_output_schema(
        &mut self,
        schema: DatasetSchemaRef,
    ) {
        self.output_schema = Some(schema);
    }

    /// Set or replace a metadata entry.
    ///
    /// Metadata keys must be non-empty and contain no control characters.
    ///
    /// Existing keys are replaced without changing their insertion
    /// position. New keys are appended.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid key or if memory reservation fails.
    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> AiResult<()> {
        let key = key.into();
        let value = value.into();

        validate_text(
            "preprocessing metadata key is invalid",
            &key,
        )?;

        if let Some((_, existing_value)) = self
            .metadata
            .iter_mut()
            .find(|(existing_key, _)| existing_key == &key)
        {
            *existing_value = value;
            return Ok(());
        }

        self.metadata
            .try_reserve(1)
            .map_err(allocation_error)?;

        self.metadata.push((key, value));

        Ok(())
    }
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Result of structural preprocessing-plan validation.
///
/// A successful report means that the plan's structural invariants hold.
///
/// It does not mean that operation implementations exist, that referenced
/// expressions are semantically valid, or that a target can execute them.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PreprocessingValidationReport {
    step_count: usize,

    checked_schema_boundaries: usize,

    unresolved_schema_boundaries: usize,
}

impl PreprocessingValidationReport {
    /// Return the number of preprocessing steps.
    #[must_use]
    pub const fn step_count(&self) -> usize {
        self.step_count
    }

    /// Return the number of schema boundaries that could be compared.
    #[must_use]
    pub const fn checked_schema_boundaries(&self) -> usize {
        self.checked_schema_boundaries
    }

    /// Return the number of boundaries for which a schema reference
    /// was missing and compatibility therefore could not be established.
    #[must_use]
    pub const fn unresolved_schema_boundaries(&self) -> usize {
        self.unresolved_schema_boundaries
    }

    /// Return whether every schema boundary was explicitly declared.
    #[must_use]
    pub const fn all_schema_boundaries_resolved(&self) -> bool {
        self.unresolved_schema_boundaries == 0
    }
}

// ============================================================================
// PREPROCESSING PLAN
// ============================================================================

/// An ordered preprocessing plan associated with a logical AI dataset.
///
/// The plan describes transformations in execution order.
///
/// It does not execute them, allocate record buffers, load datasets,
/// choose devices or schedule work.
///
/// An empty plan is valid and represents an identity transformation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreprocessingPlan {
    dataset_id: AiDatasetId,

    input_schema: Option<DatasetSchemaRef>,

    output_schema: Option<DatasetSchemaRef>,

    steps: Vec<PreprocessingStep>,
}

impl PreprocessingPlan {
    /// Construct an empty preprocessing plan for a dataset identity.
    #[must_use]
    pub const fn new(dataset_id: AiDatasetId) -> Self {
        Self {
            dataset_id,
            input_schema: None,
            output_schema: None,
            steps: Vec::new(),
        }
    }

    /// Construct an empty plan associated with a dataset definition.
    #[must_use]
    pub fn for_dataset(
        dataset: &DatasetDefinition,
    ) -> Self {
        Self::new(dataset.id())
    }

    /// Return the associated dataset identity.
    #[must_use]
    pub const fn dataset_id(&self) -> AiDatasetId {
        self.dataset_id
    }

    /// Return the optional declared input schema.
    #[must_use]
    pub fn input_schema(&self) -> Option<&DatasetSchemaRef> {
        self.input_schema.as_ref()
    }

    /// Return the optional declared output schema.
    #[must_use]
    pub fn output_schema(&self) -> Option<&DatasetSchemaRef> {
        self.output_schema.as_ref()
    }

    /// Return preprocessing steps in declaration order.
    #[must_use]
    pub fn steps(&self) -> &[PreprocessingStep] {
        &self.steps
    }

    /// Return the number of declared steps.
    #[must_use]
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Return whether the plan contains no transformations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Associate the plan's input schema.
    pub fn set_input_schema(
        &mut self,
        schema: DatasetSchemaRef,
    ) {
        self.input_schema = Some(schema);
    }

    /// Associate the plan's output schema.
    pub fn set_output_schema(
        &mut self,
        schema: DatasetSchemaRef,
    ) {
        self.output_schema = Some(schema);
    }

    /// Add a step to the end of the plan.
    ///
    /// Step order is semantically significant and is preserved.
    ///
    /// Duplicate step names are rejected.
    ///
    /// The plan is not modified if validation or reservation fails.
    ///
    /// # Errors
    ///
    /// Returns:
    ///
    /// - `Conflict` when a step with the same name already exists.
    /// - `ResourceExhausted` when memory reservation fails.
    pub fn add_step(
        &mut self,
        step: PreprocessingStep,
    ) -> AiResult<()> {
        if self
            .steps
            .iter()
            .any(|existing| existing.name() == step.name())
        {
            return Err(preprocessing_error(
                AiErrorKind::Conflict,
                "duplicate preprocessing step name",
                Some(self.dataset_id),
            ));
        }

        self.steps
            .try_reserve(1)
            .map_err(|error| {
                allocation_error(error)
                    .with_dataset(self.dataset_id.to_string())
            })?;

        self.steps.push(step);

        Ok(())
    }

    /// Find a step by its logical name.
    #[must_use]
    pub fn step(
        &self,
        name: &str,
    ) -> Option<&PreprocessingStep> {
        self.steps
            .iter()
            .find(|step| step.name() == name)
    }

    /// Validate the plan's structural invariants.
    ///
    /// This method checks:
    ///
    /// - Step-name uniqueness.
    /// - Declared input/output schema continuity.
    /// - Plan input schema compatibility with the first declared step.
    /// - Plan output schema compatibility with the final declared step.
    ///
    /// A schema boundary is compared only when both sides provide a
    /// reference. Missing references are reported as unresolved rather
    /// than guessed.
    ///
    /// Actual schema existence, field compatibility, canonical type
    /// compatibility and operation semantics must be checked by their
    /// authoritative semantic services.
    ///
    /// # Errors
    ///
    /// Returns an error when duplicate names or contradictory declared
    /// schema references are detected.
    pub fn validate(
        &self,
    ) -> AiResult<PreprocessingValidationReport> {
        let mut report = PreprocessingValidationReport {
            step_count: self.steps.len(),
            ..PreprocessingValidationReport::default()
        };

        let mut previous_output = self.input_schema.as_ref();

        for step in &self.steps {
            if let Some(input) = step.input_schema() {
                if let Some(previous) = previous_output {
                    report.checked_schema_boundaries += 1;

                    if previous != input {
                        return Err(preprocessing_error(
                            AiErrorKind::ShapeIncompatible,
                            "preprocessing input schema does not match the preceding output schema",
                            Some(self.dataset_id),
                        ));
                    }
                } else {
                    report.unresolved_schema_boundaries += 1;
                }
            } else if previous_output.is_some() {
                report.unresolved_schema_boundaries += 1;
            }

            previous_output = step.output_schema();
        }

        match (
            previous_output,
            self.output_schema.as_ref(),
        ) {
            (Some(previous), Some(output)) => {
                report.checked_schema_boundaries += 1;

                if previous != output {
                    return Err(preprocessing_error(
                        AiErrorKind::ShapeIncompatible,
                        "preprocessing final schema does not match the plan output schema",
                        Some(self.dataset_id),
                    ));
                }
            }

            (None, Some(_)) | (Some(_), None) => {
                report.unresolved_schema_boundaries += 1;
            }

            (None, None) => {}
        }

        Ok(report)
    }

    /// Consume the plan and return its ordered steps.
    #[must_use]
    pub fn into_steps(self) -> Vec<PreprocessingStep> {
        self.steps
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn dataset_id() -> AiDatasetId {
        AiDatasetId::new(1).expect("valid dataset identity")
    }

    fn schema(
        identifier: &str,
    ) -> DatasetSchemaRef {
        DatasetSchemaRef::new(
            "test",
            identifier,
        )
        .expect("valid schema reference")
    }

    fn operation(
        name: &str,
    ) -> PreprocessingOperationRef {
        PreprocessingOperationRef::new(
            "test.data",
            name,
        )
        .expect("valid operation reference")
    }

    fn step(
        name: &str,
        operation_name: &str,
    ) -> PreprocessingStep {
        PreprocessingStep::new(
            name,
            operation(operation_name),
        )
        .expect("valid preprocessing step")
    }

    #[test]
    fn empty_plan_is_a_valid_identity_transformation() {
        let plan = PreprocessingPlan::new(dataset_id());

        assert!(plan.is_empty());
        assert_eq!(plan.len(), 0);

        let report = plan.validate().expect("valid empty plan");

        assert_eq!(report.step_count(), 0);
        assert!(report.all_schema_boundaries_resolved());
    }

    #[test]
    fn plan_preserves_dataset_identity() {
        let plan = PreprocessingPlan::new(dataset_id());

        assert_eq!(plan.dataset_id(), dataset_id());
    }

    #[test]
    fn steps_preserve_declaration_order() {
        let mut plan = PreprocessingPlan::new(dataset_id());

        plan.add_step(step("first", "normalize"))
            .expect("first step");

        plan.add_step(step("second", "project"))
            .expect("second step");

        plan.add_step(step("third", "filter"))
            .expect("third step");

        assert_eq!(plan.steps()[0].name(), "first");
        assert_eq!(plan.steps()[1].name(), "second");
        assert_eq!(plan.steps()[2].name(), "third");
    }

    #[test]
    fn duplicate_step_names_are_rejected() {
        let mut plan = PreprocessingPlan::new(dataset_id());

        plan.add_step(step("transform", "normalize"))
            .expect("first step");

        let error = plan
            .add_step(step("transform", "project"))
            .expect_err("duplicate must fail");

        assert_eq!(error.kind(), AiErrorKind::Conflict);
        assert_eq!(plan.len(), 1);
    }

    #[test]
    fn step_lookup_uses_logical_name() {
        let mut plan = PreprocessingPlan::new(dataset_id());

        plan.add_step(step("normalize", "normalize"))
            .expect("step");

        assert_eq!(
            plan.step("normalize")
                .expect("step exists")
                .operation()
                .name(),
            "normalize"
        );

        assert!(plan.step("missing").is_none());
    }

    #[test]
    fn operation_references_are_open_world() {
        let operation = PreprocessingOperationRef::new(
            "third_party.example",
            "future_transform",
        )
        .expect("extension operation");

        assert_eq!(
            operation.qualified_name(),
            "third_party.example::future_transform"
        );
    }

    #[test]
    fn invalid_operation_names_are_rejected() {
        assert!(
            PreprocessingOperationRef::new(
                "valid.namespace",
                "invalid operation",
            )
            .is_err()
        );

        assert!(
            PreprocessingOperationRef::new(
                "",
                "operation",
            )
            .is_err()
        );
    }

    #[test]
    fn expression_references_are_opaque() {
        let expression = PreprocessingExpressionRef::new(
            "canonical-ast-node:expression-17",
        )
        .expect("valid expression reference");

        assert_eq!(
            expression.as_str(),
            "canonical-ast-node:expression-17"
        );
    }

    #[test]
    fn empty_expression_references_are_rejected() {
        assert!(
            PreprocessingExpressionRef::new("")
                .is_err()
        );
    }

    #[test]
    fn arguments_preserve_declaration_order() {
        let mut step = step("map", "map");

        step.add_argument(
            PreprocessingExpressionRef::new("expression-a")
                .expect("argument"),
        )
        .expect("add argument");

        step.add_argument(
            PreprocessingExpressionRef::new("expression-b")
                .expect("argument"),
        )
        .expect("add argument");

        assert_eq!(
            step.arguments()[0].as_str(),
            "expression-a"
        );

        assert_eq!(
            step.arguments()[1].as_str(),
            "expression-b"
        );
    }

    #[test]
    fn metadata_is_inserted_and_replaced() {
        let mut step = step("normalize", "normalize");

        step.set_metadata("purpose", "training")
            .expect("metadata");

        step.set_metadata("purpose", "inference")
            .expect("replace metadata");

        assert_eq!(step.metadata().len(), 1);
        assert_eq!(step.metadata()[0].0, "purpose");
        assert_eq!(step.metadata()[0].1, "inference");
    }

    #[test]
    fn metadata_preserves_insertion_order() {
        let mut step = step("normalize", "normalize");

        step.set_metadata("first", "1")
            .expect("metadata");

        step.set_metadata("second", "2")
            .expect("metadata");

        assert_eq!(step.metadata()[0].0, "first");
        assert_eq!(step.metadata()[1].0, "second");
    }

    #[test]
    fn matching_schema_boundaries_validate() {
        let input = schema("raw");
        let normalized = schema("normalized");

        let mut plan = PreprocessingPlan::new(dataset_id());

        plan.set_input_schema(input.clone());

        let mut first = step("normalize", "normalize");
        first.set_input_schema(input);
        first.set_output_schema(normalized.clone());

        plan.add_step(first).expect("first step");

        let mut second = step("project", "project");
        second.set_input_schema(normalized.clone());
        second.set_output_schema(normalized.clone());

        plan.add_step(second).expect("second step");

        plan.set_output_schema(normalized);

        let report = plan.validate().expect("compatible schemas");

        assert_eq!(report.step_count(), 2);
        assert_eq!(report.checked_schema_boundaries(), 3);
        assert_eq!(report.unresolved_schema_boundaries(), 0);
    }

    #[test]
    fn incompatible_adjacent_schemas_are_rejected() {
        let mut plan = PreprocessingPlan::new(dataset_id());

        let mut first = step("first", "normalize");
        first.set_output_schema(schema("normalized"));

        plan.add_step(first).expect("first step");

        let mut second = step("second", "project");
        second.set_input_schema(schema("raw"));

        plan.add_step(second).expect("second step");

        let error = plan
            .validate()
            .expect_err("schema mismatch");

        assert_eq!(error.kind(), AiErrorKind::ShapeIncompatible);
    }

    #[test]
    fn incompatible_final_schema_is_rejected() {
        let mut plan = PreprocessingPlan::new(dataset_id());

        let mut final_step = step("final", "project");
        final_step.set_output_schema(schema("projected"));

        plan.add_step(final_step).expect("step");

        plan.set_output_schema(schema("raw"));

        let error = plan
            .validate()
            .expect_err("final schema mismatch");

        assert_eq!(error.kind(), AiErrorKind::ShapeIncompatible);
    }

    #[test]
    fn missing_schema_information_is_reported_as_unresolved() {
        let mut plan = PreprocessingPlan::new(dataset_id());

        plan.set_input_schema(schema("raw"));

        plan.add_step(step("normalize", "normalize"))
            .expect("step");

        let report = plan.validate().expect("structurally valid");

        assert!(
            report.unresolved_schema_boundaries() > 0
        );

        assert!(
            !report.all_schema_boundaries_resolved()
        );
    }

    #[test]
    fn schema_references_remain_canonical_references() {
        let input = schema("input");
        let output = schema("output");

        let mut plan = PreprocessingPlan::new(dataset_id());

        plan.set_input_schema(input.clone());
        plan.set_output_schema(output.clone());

        assert_eq!(plan.input_schema(), Some(&input));
        assert_eq!(plan.output_schema(), Some(&output));
    }

    #[test]
    fn plan_can_be_consumed_without_copying_steps() {
        let mut plan = PreprocessingPlan::new(dataset_id());

        plan.add_step(step("first", "map"))
            .expect("step");

        let steps = plan.into_steps();

        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].name(), "first");
    }

    #[test]
    fn large_logical_step_sequences_have_no_artificial_ceiling() {
        let mut plan = PreprocessingPlan::new(dataset_id());

        for index in 0..10_000 {
            let name = format!("step-{index}");

            plan.add_step(
                PreprocessingStep::new(
                    name,
                    operation("extension_transform"),
                )
                .expect("step"),
            )
            .expect("add step");
        }

        assert_eq!(plan.len(), 10_000);

        assert_eq!(
            plan.validate()
                .expect("valid plan")
                .step_count(),
            10_000
        );
    }
}
