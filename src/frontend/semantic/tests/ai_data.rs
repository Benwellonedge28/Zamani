#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI/Data Semantic Integration Tests
//!
//! Path: `src/frontend/semantic/tests/ai_data.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Verify AI/data semantic validation against the existing AI-domain API,
//! including symbolic tensor dimensions, operation metadata, value-flow
//! integrity, shared semantic checks, deterministic diagnostics, extensibility,
//! and explicit incomplete-analysis behavior.
//!
//! AUTHORITY
//!
//! - `grammar/Zamani.g4`
//! - `grammar/ai/ai.g4`
//! - `grammar/ai/tensors.g4`
//! - `grammar/ai/datasets.g4`
//! - `grammar/ai/learning.g4`
//! - `grammar/ai/adaptation.g4`
//! - `grammar/data/data.g4`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/specification/scalability-model.md`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/domains/ai.rs`
//! - `src/frontend/semantic/domains/data.rs`
//!
//! The grammar owns source syntax. The canonical AST owns source structure.
//! Shared semantic services own canonical types, effects, capabilities,
//! resource constraints, authorization, and provenance. These tests exercise
//! the AI validator's adapter-facing contract; they do not claim to test a
//! source-to-AST adapter that has not been integrated.
//!
//! OWNS
//!
//! - AI semantic API contract tests.
//! - Shared-context delegation tests.
//! - Tensor shape and symbolic-dimension tests.
//! - Operation and value-flow integrity tests.
//! - Determinism, extensibility, and grammar-authority tests.
//!
//! DOES_NOT_OWN
//!
//! - A replacement AST, type system, or AI intermediate representation.
//! - Parsing or AST construction.
//! - Hardware discovery, allocation, placement, or target feasibility.
//! - Training, inference, tensor execution, or accelerator scheduling.
//! - Authorization policy implementation.
//!
//! SCALABILITY
//!
//! No language-level tensor-rank, dataset-size, model-size, or device-count
//! ceiling is introduced here. Test data remains finite because the test
//! process has finite resources. Symbolic dimensions must remain symbolic
//! unless the shared semantic context establishes a definitive result.
//!
//! INTEGRATION
//!
//! Register this module using `mod ai_data;` in the semantic test module.
//! The parent semantic test module must be registered by the frontend semantic
//! module. Do not add another AI validator or duplicate production types.
//!
//! FREEZE CRITERIA
//!
//! These tests must compile against the actual public AI semantic API, pass
//! alongside the domain's unit tests, and run with the checked-in grammar and
//! specification files. Full production readiness additionally requires a
//! real canonical-AST adapter and pipeline integration test.

use std::cell::Cell;

use crate::frontend::semantic::domains::ai::{
    AiId, AiOperation, AiOperationKind, AiOrigin, AiProgram, AiSemanticContext,
    AiSeverity, AiValue, AiValueFlow, AiValueKind, CheckStatus, ContextCheck,
    DimensionExpr, QualifiedName, TensorShape,
};

// These paths are relative to src/frontend/semantic/tests/.
// include_str! makes missing or renamed authoritative grammar/specification
// files a compile-time failure instead of silently testing copied syntax.
const ROOT_GRAMMAR: &str = include_str!("../../../../grammar/Zamani.g4");
const AI_GRAMMAR: &str = include_str!("../../../../grammar/ai/ai.g4");
const TENSOR_GRAMMAR: &str = include_str!("../../../../grammar/ai/tensors.g4");
const DATASET_GRAMMAR: &str = include_str!("../../../../grammar/ai/datasets.g4");
const LEARNING_GRAMMAR: &str = include_str!("../../../../grammar/ai/learning.g4");
const ADAPTATION_GRAMMAR: &str =
    include_str!("../../../../grammar/ai/adaptation.g4");
const DATA_GRAMMAR: &str = include_str!("../../../../grammar/data/data.g4");
const POCO_REAF_SPEC: &str =
    include_str!("../../../../grammar/specification/poco-reaf.md");
const SCALABILITY_SPEC: &str =
    include_str!("../../../../grammar/specification/scalability-model.md");

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

fn id(value: &str) -> AiId {
    AiId::new(value).expect("test AI identity must be valid")
}

fn qualified(namespace: &str, name: &str) -> QualifiedName {
    QualifiedName::new(namespace, name).expect("test qualified name must be valid")
}

fn tensor_value(value_id: &str, shape: Option<TensorShape>) -> AiValue {
    AiValue {
        id: id(value_id),
        kind: AiValueKind::Tensor,
        type_identity: "core.tensor".to_owned(),
        shape,
        origin: AiOrigin::default(),
    }
}

fn model_value(value_id: &str) -> AiValue {
    AiValue {
        id: id(value_id),
        kind: AiValueKind::Model,
        type_identity: "ai.model".to_owned(),
        shape: None,
        origin: AiOrigin::default(),
    }
}

fn operation(
    operation_id: &str,
    kind: AiOperationKind,
    inputs: &[&str],
    outputs: &[&str],
) -> AiOperation {
    AiOperation {
        id: id(operation_id),
        operation: qualified("ai", "test_operation"),
        kind,
        inputs: inputs.iter().map(|value| id(value)).collect(),
        outputs: outputs.iter().map(|value| id(value)).collect(),
        origin: AiOrigin::default(),
    }
}

fn tensor_operation(
    operation_id: &str,
    inputs: &[&str],
    outputs: &[&str],
) -> AiOperation {
    operation(
        operation_id,
        AiOperationKind::TensorOperation,
        inputs,
        outputs,
    )
}

fn flow(producer: &str, consumer: &str, value: &str) -> AiValueFlow {
    AiValueFlow {
        producer: id(producer),
        consumer: id(consumer),
        value: id(value),
        expected_type_identity: Some("core.tensor".to_owned()),
        origin: AiOrigin::default(),
    }
}

fn has_code(program: &AiProgram, context: &TestContext, code: &str) -> bool {
    program
        .validate(context)
        .issues
        .iter()
        .any(|issue| issue.code == code)
}

fn has_report_code(report: &crate::frontend::semantic::domains::ai::AiValidationReport, code: &str) -> bool {
    report.issues.iter().any(|issue| issue.code == code)
}

fn has_error_code(
    report: &crate::frontend::semantic::domains::ai::AiValidationReport,
    code: &str,
) -> bool {
    report
        .issues
        .iter()
        .any(|issue| issue.code == code && issue.severity == AiSeverity::Error)
}

// ---------------------------------------------------------------------------
// Deterministic shared semantic context
// ---------------------------------------------------------------------------

/// A configurable test double for services owned by the shared semantic
/// infrastructure. It does not discover hardware or implement type checking.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Response {
    Valid,
    Invalid,
    Incomplete,
}

impl Response {
    fn check(self, code: &'static str, message: &'static str) -> ContextCheck {
        match self {
            Self::Valid => ContextCheck::valid(),
            Self::Invalid => ContextCheck::invalid(code, message),
            Self::Incomplete => ContextCheck::incomplete(code, message),
        }
    }
}

struct TestContext {
    operation: Response,
    value_type: Response,
    dimension: Response,
    flow: Response,
    obligations: Response,
    operation_calls: Cell<usize>,
    value_type_calls: Cell<usize>,
    dimension_calls: Cell<usize>,
    flow_calls: Cell<usize>,
    obligation_calls: Cell<usize>,
}

impl Default for TestContext {
    fn default() -> Self {
        Self {
            operation: Response::Valid,
            value_type: Response::Valid,
            dimension: Response::Valid,
            flow: Response::Valid,
            obligations: Response::Valid,
            operation_calls: Cell::new(0),
            value_type_calls: Cell::new(0),
            dimension_calls: Cell::new(0),
            flow_calls: Cell::new(0),
            obligation_calls: Cell::new(0),
        }
    }
}

impl TestContext {
    fn with_dimension_response(response: Response) -> Self {
        Self {
            dimension: response,
            ..Self::default()
        }
    }

    fn with_operation_response(response: Response) -> Self {
        Self {
            operation: response,
            ..Self::default()
        }
    }

    fn with_type_response(response: Response) -> Self {
        Self {
            value_type: response,
            ..Self::default()
        }
    }

    fn with_flow_response(response: Response) -> Self {
        Self {
            flow: response,
            ..Self::default()
        }
    }

    fn with_obligation_response(response: Response) -> Self {
        Self {
            obligations: response,
            ..Self::default()
        }
    }
}

impl AiSemanticContext for TestContext {
    fn validate_operation(&self, _: &AiOperation) -> ContextCheck {
        self.operation_calls
            .set(self.operation_calls.get().saturating_add(1));
        self.operation.check(
            "test.ai.operation",
            "The test context rejected the operation.",
        )
    }

    fn validate_value_type(&self, _: &AiValue) -> ContextCheck {
        self.value_type_calls
            .set(self.value_type_calls.get().saturating_add(1));
        self.value_type.check(
            "test.ai.value_type",
            "The test context rejected the canonical value type.",
        )
    }

    fn validate_dimension(&self, _: &DimensionExpr) -> ContextCheck {
        self.dimension_calls
            .set(self.dimension_calls.get().saturating_add(1));
        self.dimension.check(
            "test.ai.dimension",
            "The test context could not validate a dimension.",
        )
    }

    fn validate_flow(
        &self,
        _: &AiValueFlow,
        _: &AiValue,
        _: &AiOperation,
        _: &AiOperation,
    ) -> ContextCheck {
        self.flow_calls
            .set(self.flow_calls.get().saturating_add(1));
        self.flow.check(
            "test.ai.flow",
            "The test context rejected the value flow.",
        )
    }

    fn validate_obligations(&self, _: &AiOperation) -> ContextCheck {
        self.obligation_calls
            .set(self.obligation_calls.get().saturating_add(1));
        self.obligations.check(
            "test.ai.obligations",
            "The test context rejected an operation obligation.",
        )
    }
}

// ---------------------------------------------------------------------------
// Result validity and delegation
// ---------------------------------------------------------------------------

#[test]
fn empty_program_is_valid_when_shared_context_has_no_obligations() {
    let report = AiProgram::default().validate(&TestContext::default());

    assert!(report.complete);
    assert!(report.is_valid());
    assert!(!report.has_errors());
    assert!(report.issues.is_empty());
}

#[test]
fn valid_program_delegates_operations_types_dimensions_flows_and_obligations() {
    let program = AiProgram {
        values: vec![
            tensor_value(
                "input",
                Some(TensorShape::new(vec![
                    DimensionExpr::Constant(16),
                    DimensionExpr::Symbol(id("batch")),
                ])),
            ),
            tensor_value("output", Some(TensorShape::new(vec![
                DimensionExpr::Symbol(id("batch")),
                DimensionExpr::Constant(4),
            ]))),
        ],
        operations: vec![
            tensor_operation("produce", &["input"], &["output"]),
            tensor_operation("consume", &["output"], &[]),
        ],
        flows: vec![flow("produce", "consume", "output")],
    };
    let context = TestContext::default();

    let report = program.validate(&context);

    assert!(report.is_valid(), "{:#?}", report.issues);
    assert_eq!(context.operation_calls.get(), 2);
    assert_eq!(context.value_type_calls.get(), 2);
    assert_eq!(context.dimension_calls.get(), 4);
    assert_eq!(context.flow_calls.get(), 1);
    assert_eq!(context.obligation_calls.get(), 2);
}

#[test]
fn shared_type_failure_is_reported_as_an_error() {
    let program = AiProgram {
        values: vec![tensor_value("x", None)],
        ..AiProgram::default()
    };
    let report = program.validate(&TestContext::with_type_response(Response::Invalid));

    assert!(has_error_code(&report, "test.ai.value_type"));
    assert!(!report.is_valid());
    assert!(report.complete);
}

#[test]
fn unresolved_type_or_dimension_cannot_produce_certified_success() {
    let program = AiProgram {
        values: vec![tensor_value(
            "x",
            Some(TensorShape::new(vec![DimensionExpr::Symbol(id("unknown_dim"))])),
        )],
        ..AiProgram::default()
    };
    let report =
        program.validate(&TestContext::with_dimension_response(Response::Incomplete));

    assert!(!report.complete);
    assert!(!report.is_valid());
    assert!(report.has_errors());
    assert!(has_error_code(&report, "test.ai.dimension"));
}

#[test]
fn invalid_operation_is_reported_by_shared_operation_registry() {
    let program = AiProgram {
        operations: vec![tensor_operation("op", &[], &[])],
        ..AiProgram::default()
    };
    let report =
        program.validate(&TestContext::with_operation_response(Response::Invalid));

    assert!(has_error_code(&report, "test.ai.operation"));
    assert!(!report.is_valid());
}

#[test]
fn unresolved_operation_obligations_make_analysis_incomplete() {
    let program = AiProgram {
        operations: vec![tensor_operation("op", &[], &[])],
        ..AiProgram::default()
    };
    let report =
        program.validate(&TestContext::with_obligation_response(Response::Incomplete));

    assert!(!report.complete);
    assert!(!report.is_valid());
    assert!(has_error_code(&report, "test.ai.obligations"));
}

#[test]
fn invalid_value_flow_is_delegated_to_shared_semantic_services() {
    let program = AiProgram {
        values: vec![tensor_value("x", None)],
        operations: vec![
            tensor_operation("producer", &[], &["x"]),
            tensor_operation("consumer", &["x"], &[]),
        ],
        flows: vec![flow("producer", "consumer", "x")],
    };
    let report = program.validate(&TestContext::with_flow_response(Response::Invalid));

    assert!(has_error_code(&report, "test.ai.flow"));
    assert!(!report.is_valid());
}

// ---------------------------------------------------------------------------
// Symbolic dimensions and scalable shapes
// ---------------------------------------------------------------------------

#[test]
fn symbolic_dimensions_are_preserved_without_eager_evaluation() {
    let expression = DimensionExpr::Multiply(
        Box::new(DimensionExpr::Symbol(id("batch"))),
        Box::new(DimensionExpr::Constant(4)),
    );
    let shape = TensorShape::new(vec![
        DimensionExpr::Constant(128),
        DimensionExpr::Symbol(id("sequence")),
        expression.clone(),
    ]);

    assert_eq!(shape.rank(), 3);
    assert_eq!(shape.dimensions[1], DimensionExpr::Symbol(id("sequence")));
    assert_eq!(shape.dimensions[2], expression);
    assert!(!shape.is_scalar());
}

#[test]
fn scalar_shape_is_distinguished_from_unknown_or_symbolic_shape() {
    let scalar = TensorShape::new(Vec::new());
    let symbolic = TensorShape::new(vec![DimensionExpr::Symbol(id("n"))]);

    assert!(scalar.is_scalar());
    assert_eq!(scalar.rank(), 0);
    assert!(!symbolic.is_scalar());
    assert_eq!(symbolic.rank(), 1);

    let unspecified = tensor_value("unspecified", None);
    assert!(unspecified.shape.is_none());
}

#[test]
fn shape_rank_is_not_limited_by_a_test_defined_hardware_ceiling() {
    // This is a finite stress sample, not a language-level maximum.
    let dimensions = (0..256)
        .map(|index| DimensionExpr::Symbol(id(&format!("dimension_{index}"))))
        .collect::<Vec<_>>();
    let shape = TensorShape::new(dimensions);

    assert_eq!(shape.rank(), 256);
    assert!(shape
        .dimensions
        .iter()
        .all(|dimension| matches!(dimension, DimensionExpr::Symbol(_))));

    let program = AiProgram {
        values: vec![tensor_value("wide_shape", Some(shape))],
        ..AiProgram::default()
    };
    let report = program.validate(&TestContext::default());

    assert!(report.is_valid(), "{:#?}", report.issues);
}

#[test]
fn large_representable_constant_dimension_is_not_a_machine_capacity_claim() {
    let dimension = DimensionExpr::Constant(u128::MAX);
    let program = AiProgram {
        values: vec![tensor_value(
            "large_symbolic_program_value",
            Some(TensorShape::new(vec![dimension])),
        )],
        ..AiProgram::default()
    };
    let report = program.validate(&TestContext::default());

    // This validator delegates dimension meaning to shared semantic services.
    // It must not reject a value merely because it resembles a large capacity.
    assert!(report.is_valid(), "{:#?}", report.issues);
}

#[test]
fn custom_dimension_constraint_identity_is_delegated_to_shared_solver() {
    let program = AiProgram {
        values: vec![tensor_value(
            "constrained",
            Some(TensorShape::new(vec![DimensionExpr::Constraint(id(
                "shape_constraint",
            ))])),
        )],
        ..AiProgram::default()
    };
    let context = TestContext::default();

    let report = program.validate(&context);

    assert!(report.is_valid(), "{:#?}", report.issues);
    assert_eq!(context.dimension_calls.get(), 1);
}

// ---------------------------------------------------------------------------
// Identity and graph integrity
// ---------------------------------------------------------------------------

#[test]
fn ai_identifiers_reject_empty_values_and_control_characters() {
    assert!(AiId::new("").is_err());
    assert!(AiId::new("bad\nidentity").is_err());
    assert!(AiId::new("tensor_λ").is_ok());
}

#[test]
fn qualified_operation_names_are_open_world_and_namespace_qualified() {
    assert!(QualifiedName::new("ai.tensor", "custom_transform").is_ok());
    assert!(QualifiedName::new("", "operation").is_err());
    assert!(QualifiedName::new("ai", "").is_err());
}

#[test]
fn duplicate_value_identity_is_reported() {
    let program = AiProgram {
        values: vec![tensor_value("duplicate", None), tensor_value("duplicate", None)],
        ..AiProgram::default()
    };
    let report = program.validate(&TestContext::default());

    assert!(has_error_code(&report, "AI001"));
    assert!(!report.is_valid());
}

#[test]
fn duplicate_operation_identity_is_reported() {
    let program = AiProgram {
        operations: vec![
            tensor_operation("duplicate", &[], &[]),
            tensor_operation("duplicate", &[], &[]),
        ],
        ..AiProgram::default()
    };
    let report = program.validate(&TestContext::default());

    assert!(has_error_code(&report, "AI002"));
    assert!(!report.is_valid());
}

#[test]
fn repeated_input_and_output_ids_are_reported() {
    let program = AiProgram {
        values: vec![tensor_value("x", None)],
        operations: vec![
            tensor_operation("repeated_input", &["x", "x"], &[]),
            tensor_operation("repeated_output", &[], &["x", "x"]),
        ],
        ..AiProgram::default()
    };
    let report = program.validate(&TestContext::default());

    assert!(has_error_code(&report, "AI003"));
    assert!(has_error_code(&report, "AI005"));
    assert!(!report.is_valid());
}

#[test]
fn undeclared_operation_inputs_and_outputs_are_reported() {
    let program = AiProgram {
        operations: vec![tensor_operation("op", &["missing_input"], &["missing_output"])],
        ..AiProgram::default()
    };
    let report = program.validate(&TestContext::default());

    assert!(has_error_code(&report, "AI004"));
    assert!(has_error_code(&report, "AI006"));
    assert!(!report.is_valid());
}

#[test]
fn unknown_flow_value_and_operation_endpoints_are_reported() {
    let program = AiProgram {
        flows: vec![flow("missing_producer", "missing_consumer", "missing_value")],
        ..AiProgram::default()
    };
    let report = program.validate(&TestContext::default());

    // Unknown value is diagnosed first because endpoints cannot be validated
    // without an established value identity.
    assert!(has_error_code(&report, "AI008"));
    assert!(!report.is_valid());
}

#[test]
fn producer_must_declare_the_flowed_value_as_an_output() {
    let program = AiProgram {
        values: vec![tensor_value("x", None)],
        operations: vec![
            tensor_operation("producer", &[], &[]),
            tensor_operation("consumer", &["x"], &[]),
        ],
        flows: vec![flow("producer", "consumer", "x")],
    };
    let report = program.validate(&TestContext::default());

    assert!(has_error_code(&report, "AI011"));
    assert!(!report.is_valid());
}

#[test]
fn consumer_must_declare_the_flowed_value_as_an_input() {
    let program = AiProgram {
        values: vec![tensor_value("x", None)],
        operations: vec![
            tensor_operation("producer", &[], &["x"]),
            tensor_operation("consumer", &[], &[]),
        ],
        flows: vec![flow("producer", "consumer", "x")],
    };
    let report = program.validate(&TestContext::default());

    assert!(has_error_code(&report, "AI012"));
    assert!(!report.is_valid());
}

#[test]
fn duplicate_value_flow_is_reported() {
    let program = AiProgram {
        values: vec![tensor_value("x", None)],
        operations: vec![
            tensor_operation("producer", &[], &["x"]),
            tensor_operation("consumer", &["x"], &[]),
        ],
        flows: vec![
            flow("producer", "consumer", "x"),
            flow("producer", "consumer", "x"),
        ],
    };
    let report = program.validate(&TestContext::default());

    assert!(has_error_code(&report, "AI007"));
    assert!(!report.is_valid());
}

#[test]
fn one_value_cannot_have_distinct_producers() {
    let program = AiProgram {
        values: vec![tensor_value("x", None)],
        operations: vec![
            tensor_operation("producer_a", &[], &["x"]),
            tensor_operation("producer_b", &[], &["x"]),
        ],
        ..AiProgram::default()
    };
    let report = program.validate(&TestContext::default());

    assert!(has_error_code(&report, "AI013"));
    assert!(!report.is_valid());
}

// ---------------------------------------------------------------------------
// Extensible operations and cross-domain semantic obligations
// ---------------------------------------------------------------------------

#[test]
fn custom_ai_operation_kind_is_supported_without_a_new_core_variant() {
    let custom = AiOperationKind::Custom(
        qualified("vendor_independent.ai", "future_operation"),
    );
    let program = AiProgram {
        operations: vec![operation("custom", custom, &[], &[])],
        ..AiProgram::default()
    };
    let report = program.validate(&TestContext::default());

    assert!(report.is_valid(), "{:#?}", report.issues);
}

#[test]
fn learning_and_inference_remain_distinct_operation_kinds() {
    let learning = operation("learning", AiOperationKind::Training, &["data"], &["model"]);
    let inference = operation("inference", AiOperationKind::Inference, &["model"], &["prediction"]);

    assert_ne!(learning.kind, inference.kind);
    assert_eq!(learning.kind, AiOperationKind::Training);
    assert_eq!(inference.kind, AiOperationKind::Inference);
}

#[test]
fn model_and_tensor_values_keep_distinct_domain_roles() {
    let tensor = tensor_value("tensor", None);
    let model = model_value("model");

    assert_eq!(tensor.kind, AiValueKind::Tensor);
    assert_eq!(model.kind, AiValueKind::Model);
    assert_ne!(tensor.type_identity, model.type_identity);
}

#[test]
fn adaptation_obligations_are_checked_through_shared_context() {
    let program = AiProgram {
        operations: vec![operation(
            "adapt",
            AiOperationKind::Custom(qualified("ai", "adaptation")),
            &["model"],
            &["adapted_model"],
        )],
        values: vec![model_value("model"), model_value("adapted_model")],
        ..AiProgram::default()
    };
    let context = TestContext::with_obligation_response(Response::Invalid);
    let report = program.validate(&context);

    assert!(has_error_code(&report, "test.ai.obligations"));
    assert_eq!(context.obligation_calls.get(), 1);
    assert!(!report.is_valid());
}

// ---------------------------------------------------------------------------
// Determinism and grammar/specification integration
// ---------------------------------------------------------------------------

#[test]
fn equivalent_invalid_inputs_produce_identically_ordered_issues() {
    let first = AiProgram {
        values: vec![tensor_value("duplicate", None), tensor_value("duplicate", None)],
        operations: vec![tensor_operation("op", &["missing"], &[])],
        ..AiProgram::default()
    };
    let second = AiProgram {
        values: vec![tensor_value("duplicate", None), tensor_value("duplicate", None)],
        operations: vec![tensor_operation("op", &["missing"], &[])],
        ..AiProgram::default()
    };

    let first_report = first.validate(&TestContext::default());
    let second_report = second.validate(&TestContext::default());

    assert_eq!(first_report, second_report);
    assert!(first_report.issues.windows(2).all(|pair| pair[0] <= pair[1]));
}

#[test]
fn grammar_files_are_the_checked_in_authorities() {
    assert!(ROOT_GRAMMAR.contains("Zamani"));
    assert!(AI_GRAMMAR.contains("AITensors"));
    assert!(AI_GRAMMAR.contains("AIDatasets"));
    assert!(AI_GRAMMAR.contains("AILearning"));
    assert!(AI_GRAMMAR.contains("AIAdaptation"));

    assert!(TENSOR_GRAMMAR.contains("parser grammar AITensors;"));
    assert!(TENSOR_GRAMMAR.contains("tensorConstruct"));
    assert!(TENSOR_GRAMMAR.contains("symbolic"));

    assert!(DATASET_GRAMMAR.contains("parser grammar AIDatasets;"));
    assert!(DATASET_GRAMMAR.contains("datasetConstruct"));

    assert!(LEARNING_GRAMMAR.contains("aiLearningConstruct"));
    assert!(ADAPTATION_GRAMMAR.contains("aiAdaptationConstruct"));
    assert!(DATA_GRAMMAR.contains("data"));

    assert!(POCO_REAF_SPEC.contains("POCO-REAF"));
    assert!(POCO_REAF_SPEC.contains("target"));
    assert!(SCALABILITY_SPEC.contains("symbolic"));
    assert!(SCALABILITY_SPEC.contains("tensor"));
}

#[test]
fn grammar_keeps_tensor_and_dataset_syntax_at_domain_boundaries() {
    assert!(TENSOR_GRAMMAR.contains("tensorConstruct"));
    assert!(DATASET_GRAMMAR.contains("datasetConstruct"));

    // The AI composition grammar integrates leaf grammars; it must not replace
    // the leaf grammars with a second tensor or dataset syntax authority.
    assert!(AI_GRAMMAR.contains("AITensors"));
    assert!(AI_GRAMMAR.contains("AIDatasets"));
}

#[test]
fn specifications_separate_portable_meaning_from_target_realization() {
    assert!(POCO_REAF_SPEC.contains("target-specific realization"));
    assert!(POCO_REAF_SPEC.contains("resource"));
    assert!(SCALABILITY_SPEC.contains("operational"));
    assert!(SCALABILITY_SPEC.contains("Target adapters"));
}