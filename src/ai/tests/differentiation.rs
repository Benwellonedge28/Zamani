
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI Differentiation Integration Tests
//!
//! Path: src/ai/tests/differentiation.rs
//!
//! Rust baseline: 1.97+
//! Edition: 2021
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Integration and conformance tests for the AI differentiation
//! subsystem.
//!
//! These tests exercise the existing differentiation authorities
//! together rather than implementing another differentiation engine.
//!
//! # Tested components
//!
//! - Differentiation rule registry.
//! - Forward-mode automatic differentiation.
//! - Reverse-mode automatic differentiation.
//! - Differentiation dependency graphs.
//! - Differentiation validation.
//! - Checkpoint planning.
//! - Gradient request interoperability.
//! - Mathematical correctness.
//! - Deterministic behavior.
//! - Dynamic workload handling.
//!
//! # Ownership boundaries
//!
//! This file owns tests only.
//!
//! It does not implement:
//!
//! - A second differentiation engine.
//! - A second computational graph.
//! - A tensor storage implementation.
//! - An AI semantic system.
//! - A competing intermediate representation.
//! - Hardware selection.
//! - Resource allocation.
//! - Runtime scheduling.
//!
//! # POCO-REAF
//!
//! Tests use dynamically constructed workloads.
//!
//! No universal maximum is introduced for:
//!
//! - Differentiation variables.
//! - Operations.
//! - Graph nodes.
//! - Graph edges.
//! - Model parameters.
//! - Tensor rank.
//! - Tensor dimensions.
//! - Checkpoints.
//! - Differentiation rules.
//! - Execution targets.
//!
//! Literal infinite computation is not promised.
//!
//! Tests verify that arbitrary architectural ceilings are not
//! introduced by the differentiation test infrastructure.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::ai::differentiation::checkpoints::{
    CheckpointAction,
    CheckpointEligibility,
    CheckpointOperation,
    CheckpointPlan,
    CheckpointPolicy,
    CheckpointValue,
    RecomputeEligibility,
};

use crate::ai::differentiation::forward::{
    ForwardModeEngine,
    ForwardModeError,
    ForwardModeExecutor,
    ForwardValue,
};

use crate::ai::differentiation::graph::{
    DifferentiationDependencyKind,
    DifferentiationGraph,
    DifferentiationNode,
    DifferentiationNodeKind,
};

use crate::ai::differentiation::reverse::{
    ReverseModeEngine,
    ReverseModeError,
    ReverseModeExecutor,
    ReverseOperation,
    ReverseTape,
};

use crate::ai::differentiation::rules::{
    Differentiability,
    DifferentiableOperationId,
    DifferentiationDomain,
    DifferentiationMode,
    DifferentiationRegularity,
    DifferentiationRule,
    DifferentiationRuleId,
    DifferentiationRuleProvenance,
    DifferentiationRuleRegistry,
    DifferentiationRuleVersion,
    DerivativeOperationRef,
};

use crate::ai::differentiation::validation::{
    DifferentiationOperationResolver,
    DifferentiationOutput,
    DifferentiationValidationCompleteness,
    DifferentiationValidationInput,
    DifferentiationValidationPolicy,
    DifferentiationValidator,
    DifferentiationVariable,
    OperationDifferentiabilityInfo,
};

use crate::ai::learning::gradients::{
    GradientRequest,
    GradientResultKind,
    GradientStrategyRequest,
    StrategyIntent,
};

use crate::ai::learning::training::TrainingValueRef;

// ============================================================================
// COMMON TEST UTILITIES
// ============================================================================

/// Test differentiation rule version.
///
/// This is test metadata, not a production version ceiling.
fn version() -> DifferentiationRuleVersion {
    DifferentiationRuleVersion::new(1, 0, 0)
}

/// Creates a canonical operation identity.
fn operation_id(value: &str) -> DifferentiableOperationId {
    DifferentiableOperationId::new(value)
        .expect("test operation identity must be valid")
}

/// Creates a rule identity.
fn rule_id(value: &str) -> DifferentiationRuleId {
    DifferentiationRuleId::new(value)
        .expect("test rule identity must be valid")
}

/// Creates a derivative operation reference.
fn derivative_ref(value: &str) -> DerivativeOperationRef {
    DerivativeOperationRef::new(value)
        .expect("test derivative reference must be valid")
}

/// Creates test rule provenance.
fn provenance() -> DifferentiationRuleProvenance {
    DifferentiationRuleProvenance::new(
        "zamani::tests::differentiation",
        "Zamani differentiation conformance tests",
        "Mathematical integration test rule",
    )
    .expect("test provenance must be valid")
}

/// Constructs a mathematical differentiation rule.
fn make_rule(
    operation: &str,
    derivative: &str,
    mode: DifferentiationMode,
    rule_version: DifferentiationRuleVersion,
) -> DifferentiationRule {
    DifferentiationRule::new(
        rule_id(&format!("test::rule::{operation}")),
        operation_id(operation),
        derivative_ref(derivative),
        rule_version,
        vec![mode],
        vec![DifferentiationDomain::Classical],
        Differentiability::Differentiable,
        DifferentiationRegularity::Smooth,
        provenance(),
    )
    .expect("test differentiation rule must be valid")
}

/// Creates a registry containing a multiplication rule for forward
/// and reverse differentiation.
fn multiplication_registry() -> DifferentiationRuleRegistry {
    let registry = DifferentiationRuleRegistry::new();

    registry
        .register(make_rule(
            "test::multiply",
            "test::multiply::jvp",
            DifferentiationMode::Forward,
            version(),
        ))
        .expect("forward multiplication rule registration");

    registry
        .register(make_rule(
            "test::multiply",
            "test::multiply::vjp",
            DifferentiationMode::Reverse,
            version(),
        ))
        .expect("reverse multiplication rule registration");

    registry
}

// ============================================================================
// NUMERICAL TEST PROVIDER
// ============================================================================

/// Provider error used by numerical differentiation tests.
#[derive(Debug, Clone, PartialEq, Eq)]
struct NumericalProviderError {
    message: String,
}

impl NumericalProviderError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for NumericalProviderError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for NumericalProviderError {}

/// Forward-mode scalar multiplication provider.
///
/// Implements:
///
/// f(x, y) = x * y
///
/// JVP:
///
/// df = dx * y + x * dy
struct ScalarMultiplicationJvp;

impl ForwardModeExecutor<f64, f64> for ScalarMultiplicationJvp {
    type Error = NumericalProviderError;

    fn apply_jvp(
        &mut self,
        rule: &DifferentiationRule,
        inputs: &[ForwardValue<f64, f64>],
    ) -> Result<Vec<ForwardValue<f64, f64>>, Self::Error> {
        if rule.derivative().as_str() != "test::multiply::jvp" {
            return Err(NumericalProviderError::new(
                "unexpected JVP derivative operation",
            ));
        }

        if inputs.len() != 2 {
            return Err(NumericalProviderError::new(
                "multiplication requires two inputs",
            ));
        }

        let left = *inputs[0].primal();
        let right = *inputs[1].primal();

        let left_tangent = *inputs[0].tangent();
        let right_tangent = *inputs[1].tangent();

        let primal = left * right;

        let tangent =
            left_tangent * right + left * right_tangent;

        if !primal.is_finite() || !tangent.is_finite() {
            return Err(NumericalProviderError::new(
                "non-finite numerical result",
            ));
        }

        Ok(vec![ForwardValue::new(primal, tangent)])
    }
}

// ============================================================================
// REVERSE-MODE NUMERICAL PROVIDER
// ============================================================================

/// Reverse-mode scalar provider.
///
/// The provider owns the primal values needed to evaluate local
/// vector-Jacobian products.
///
/// It does not own the differentiation tape or rule registry.
struct ScalarReverseProvider {
    values: BTreeMap<String, f64>,
}

impl ScalarReverseProvider {
    fn new(
        values: impl IntoIterator<Item = (String, f64)>,
    ) -> Self {
        Self {
            values: values.into_iter().collect(),
        }
    }

    fn value(
        &self,
        identity: &str,
    ) -> Result<f64, NumericalProviderError> {
        self.values
            .get(identity)
            .copied()
            .ok_or_else(|| {
                NumericalProviderError::new(format!(
                    "missing primal value: {identity}"
                ))
            })
    }
}

impl ReverseModeExecutor<String, f64> for ScalarReverseProvider {
    type Error = NumericalProviderError;

    fn apply_vjp(
        &mut self,
        rule: &DifferentiationRule,
        inputs: &[String],
        _outputs: &[String],
        output_cotangents: &[Option<&f64>],
    ) -> Result<Vec<Option<f64>>, Self::Error> {
        let incoming = output_cotangents
            .iter()
            .filter_map(|value| *value)
            .copied()
            .sum::<f64>();

        match rule.operation().as_str() {
            "test::multiply" => {
                if inputs.len() != 2 {
                    return Err(NumericalProviderError::new(
                        "multiplication requires two inputs",
                    ));
                }

                let left = self.value(&inputs[0])?;
                let right = self.value(&inputs[1])?;

                Ok(vec![
                    Some(incoming * right),
                    Some(incoming * left),
                ])
            }

            "test::add" => {
                Ok(inputs.iter().map(|_| Some(incoming)).collect())
            }

            _ => Err(NumericalProviderError::new(
                "unsupported reverse derivative operation",
            )),
        }
    }

    fn accumulate(
        &mut self,
        accumulated: &mut f64,
        contribution: f64,
    ) -> Result<(), Self::Error> {
        let result = *accumulated + contribution;

        if !result.is_finite() {
            return Err(NumericalProviderError::new(
                "cotangent accumulation overflow",
            ));
        }

        *accumulated = result;

        Ok(())
    }
}

// ============================================================================
// 1. FORWARD-MODE MATHEMATICAL CORRECTNESS
// ============================================================================

/// Verifies the actual Jacobian-vector product for multiplication.
///
/// For:
///
/// f(x, y) = x * y
///
/// At:
///
/// x = 3
/// y = 4
///
/// With tangent direction:
///
/// dx = 1
/// dy = 0
///
/// Expected:
///
/// f(x, y) = 12
///
/// JVP = 4
#[test]
fn forward_mode_computes_correct_jacobian_vector_product() {
    let engine = ForwardModeEngine::new(
        multiplication_registry(),
        DifferentiationDomain::Classical,
    );

    let inputs = vec![
        ForwardValue::new(3.0, 1.0),
        ForwardValue::new(4.0, 0.0),
    ];

    let result = engine
        .propagate(
            operation_id("test::multiply"),
            &inputs,
            &mut ScalarMultiplicationJvp,
        )
        .expect("forward differentiation must succeed");

    assert_eq!(result.outputs().len(), 1);

    assert_eq!(
        *result.outputs()[0].primal(),
        12.0,
    );

    assert_eq!(
        *result.outputs()[0].tangent(),
        4.0,
    );

    assert_eq!(
        result.derivative_operation().as_str(),
        "test::multiply::jvp",
    );
}

/// Verifies that the tangent calculation supports simultaneous
/// differentiation directions.
#[test]
fn forward_mode_supports_multiple_nonzero_tangents() {
    let engine = ForwardModeEngine::new(
        multiplication_registry(),
        DifferentiationDomain::Classical,
    );

    let inputs = vec![
        ForwardValue::new(3.0, 2.0),
        ForwardValue::new(4.0, 5.0),
    ];

    let result = engine
        .propagate(
            operation_id("test::multiply"),
            &inputs,
            &mut ScalarMultiplicationJvp,
        )
        .expect("forward differentiation must succeed");

    // d(x*y) = dx*y + x*dy
    //
    // = 2*4 + 3*5
    // = 23

    assert_eq!(
        *result.outputs()[0].tangent(),
        23.0,
    );
}

/// Verifies that missing derivative rules fail explicitly.
#[test]
fn forward_mode_rejects_missing_derivative_rules() {
    let engine = ForwardModeEngine::new(
        DifferentiationRuleRegistry::new(),
        DifferentiationDomain::Classical,
    );

    let result = engine.propagate(
        operation_id("test::unknown"),
        &[],
        &mut ScalarMultiplicationJvp,
    );

    assert!(matches!(
        result,
        Err(ForwardModeError::RuleNotFound { .. })
    ));
}

/// Verifies exact rule-version compatibility.
#[test]
fn forward_mode_rejects_unavailable_rule_versions() {
    let engine = ForwardModeEngine::new(
        multiplication_registry(),
        DifferentiationDomain::Classical,
    );

    let result = engine.propagate_with_version(
        operation_id("test::multiply"),
        DifferentiationRuleVersion::new(9, 0, 0),
        &[],
        &mut ScalarMultiplicationJvp,
    );

    assert!(matches!(
        result,
        Err(ForwardModeError::VersionUnavailable { .. })
    ));
}

/// Verification-required execution must fail closed.
#[test]
fn forward_mode_requires_verified_rules_when_configured() {
    let engine = ForwardModeEngine::new(
        multiplication_registry(),
        DifferentiationDomain::Classical,
    )
    .requiring_verified_rules();

    let result = engine.propagate(
        operation_id("test::multiply"),
        &[],
        &mut ScalarMultiplicationJvp,
    );

    assert!(matches!(
        result,
        Err(ForwardModeError::VerificationRequired { .. })
    ));
}

// ============================================================================
// 2. REVERSE-MODE MATHEMATICAL CORRECTNESS
// ============================================================================

fn reverse_registry() -> DifferentiationRuleRegistry {
    let registry = DifferentiationRuleRegistry::new();

    for (operation, derivative) in [
        ("test::multiply", "test::multiply::vjp"),
        ("test::add", "test::add::vjp"),
    ] {
        registry
            .register(make_rule(
                operation,
                derivative,
                DifferentiationMode::Reverse,
                version(),
            ))
            .expect("reverse rule registration");
    }

    registry
}

/// Verifies reverse-mode differentiation through a computation chain.
///
/// Computation:
///
/// y = x * x
///
/// z = y + x
///
/// Therefore:
///
/// dz/dx = 2*x + 1
///
/// At x = 3:
///
/// dz/dx = 7
#[test]
fn reverse_mode_computes_correct_gradient_through_chain() {
    let tape = ReverseTape::new(vec![
        ReverseOperation::new(
            operation_id("test::multiply"),
            vec![
                "x".to_owned(),
                "x".to_owned(),
            ],
            vec!["y".to_owned()],
        ),
        ReverseOperation::new(
            operation_id("test::add"),
            vec![
                "y".to_owned(),
                "x".to_owned(),
            ],
            vec!["z".to_owned()],
        ),
    ])
    .expect("valid reverse tape");

    let engine = ReverseModeEngine::new(
        reverse_registry(),
        DifferentiationDomain::Classical,
    );

    let mut provider = ScalarReverseProvider::new([
        ("x".to_owned(), 3.0),
        ("y".to_owned(), 9.0),
        ("z".to_owned(), 12.0),
    ]);

    let mut seeds = BTreeMap::new();
    seeds.insert("z".to_owned(), 1.0);

    let requested = vec!["x".to_owned()];

    let result = engine
        .differentiate(
            &tape,
            seeds,
            &requested,
            &mut provider,
        )
        .expect("reverse differentiation must succeed");

    let gradient = result
        .gradient_for(&"x".to_owned())
        .expect("gradient must exist");

    assert_eq!(*gradient, 7.0);

    assert_eq!(result.executed_rule_count(), 2);
}

/// Verifies accumulation of contributions from repeated input use.
///
/// Computation:
///
/// y = x * x
///
/// At x = 5:
///
/// dy/dx = 10
#[test]
fn reverse_mode_accumulates_repeated_input_contributions() {
    let tape = ReverseTape::new(vec![
        ReverseOperation::new(
            operation_id("test::multiply"),
            vec![
                "x".to_owned(),
                "x".to_owned(),
            ],
            vec!["y".to_owned()],
        ),
    ])
    .expect("valid reverse tape");

    let engine = ReverseModeEngine::new(
        reverse_registry(),
        DifferentiationDomain::Classical,
    );

    let mut provider = ScalarReverseProvider::new([
        ("x".to_owned(), 5.0),
        ("y".to_owned(), 25.0),
    ]);

    let seeds = BTreeMap::from([
        ("y".to_owned(), 1.0),
    ]);

    let result = engine
        .differentiate(
            &tape,
            seeds,
            &["x".to_owned()],
            &mut provider,
        )
        .expect("reverse differentiation must succeed");

    assert_eq!(
        *result.gradient_for(&"x".to_owned()).unwrap(),
        10.0,
    );
}

/// Verifies that reverse differentiation does not execute
/// operations outside the active output dependency path.
#[test]
fn reverse_mode_skips_inactive_branches() {
    let tape = ReverseTape::new(vec![
        ReverseOperation::new(
            operation_id("test::multiply"),
            vec![
                "x".to_owned(),
                "x".to_owned(),
            ],
            vec!["active".to_owned()],
        ),
        ReverseOperation::new(
            operation_id("test::multiply"),
            vec![
                "a".to_owned(),
                "b".to_owned(),
            ],
            vec!["inactive".to_owned()],
        ),
    ])
    .expect("valid reverse tape");

    let engine = ReverseModeEngine::new(
        reverse_registry(),
        DifferentiationDomain::Classical,
    );

    let mut provider = ScalarReverseProvider::new([
        ("x".to_owned(), 3.0),
        ("a".to_owned(), 2.0),
        ("b".to_owned(), 4.0),
    ]);

    let seeds = BTreeMap::from([
        ("active".to_owned(), 1.0),
    ]);

    let result = engine
        .differentiate(
            &tape,
            seeds,
            &["x".to_owned()],
            &mut provider,
        )
        .expect("inactive branches must not be executed");

    assert_eq!(
        *result.gradient_for(&"x".to_owned()).unwrap(),
        6.0,
    );

    assert_eq!(result.executed_rule_count(), 1);
}

/// Verifies that reverse differentiation rejects unknown output seeds.
#[test]
fn reverse_mode_rejects_unknown_output_seed() {
    let tape = ReverseTape::new(vec![
        ReverseOperation::new(
            operation_id("test::multiply"),
            vec!["x".to_owned(), "x".to_owned()],
            vec!["y".to_owned()],
        ),
    ])
    .expect("valid reverse tape");

    let engine = ReverseModeEngine::new(
        reverse_registry(),
        DifferentiationDomain::Classical,
    );

    let mut provider = ScalarReverseProvider::new([
        ("x".to_owned(), 2.0),
    ]);

    let seeds = BTreeMap::from([
        ("unknown".to_owned(), 1.0),
    ]);

    let result = engine.differentiate(
        &tape,
        seeds,
        &["x".to_owned()],
        &mut provider,
    );

    assert!(matches!(
        result,
        Err(ReverseModeError::UnknownOutputSeed)
    ));
}

/// Verifies that duplicate differentiation variables are rejected.
#[test]
fn reverse_mode_rejects_duplicate_requested_variables() {
    let tape = ReverseTape::new(vec![
        ReverseOperation::new(
            operation_id("test::multiply"),
            vec!["x".to_owned(), "x".to_owned()],
            vec!["y".to_owned()],
        ),
    ])
    .expect("valid reverse tape");

    let engine = ReverseModeEngine::new(
        reverse_registry(),
        DifferentiationDomain::Classical,
    );

    let mut provider = ScalarReverseProvider::new([
        ("x".to_owned(), 2.0),
    ]);

    let result = engine.differentiate(
        &tape,
        BTreeMap::from([("y".to_owned(), 1.0)]),
        &["x".to_owned(), "x".to_owned()],
        &mut provider,
    );

    assert!(matches!(
        result,
        Err(ReverseModeError::DuplicateRequestedInput)
    ));
}

// ============================================================================
// 3. DIFFERENTIATION GRAPH INTEGRITY
// ============================================================================

/// Verifies dependency ordering.
#[test]
fn differentiation_graph_preserves_topological_order() {
    let mut graph =
        DifferentiationGraph::<u64, String>::new();

    for (id, kind) in [
        (1, DifferentiationNodeKind::Input),
        (2, DifferentiationNodeKind::Intermediate),
        (3, DifferentiationNodeKind::Operation(
            "test::multiply".to_owned(),
        )),
        (4, DifferentiationNodeKind::Output),
    ] {
        graph
            .add_node(DifferentiationNode::new(id, kind))
            .expect("node insertion");
    }

    for (source, target) in [
        (1, 2),
        (2, 3),
        (3, 4),
    ] {
        graph
            .add_dependency(
                source,
                target,
                DifferentiationDependencyKind::Value,
            )
            .expect("dependency insertion");
    }

    let report = graph
        .validate()
        .expect("graph must be valid");

    assert_eq!(report.node_count(), 4);
    assert_eq!(report.edge_count(), 3);

    assert_eq!(
        report.topological_order(),
        &[1, 2, 3, 4],
    );
}

/// Verifies that dependency cycles are rejected.
#[test]
fn differentiation_graph_rejects_cycles() {
    let mut graph =
        DifferentiationGraph::<u64, String>::new();

    for id in [1, 2, 3] {
        graph
            .add_node(DifferentiationNode::new(
                id,
                DifferentiationNodeKind::Intermediate,
            ))
            .expect("node insertion");
    }

    for (source, target) in [
        (1, 2),
        (2, 3),
    ] {
        graph
            .add_dependency(
                source,
                target,
                DifferentiationDependencyKind::Value,
            )
            .expect("valid dependency");
    }

    let result = graph.add_dependency(
        3,
        1,
        DifferentiationDependencyKind::Value,
    );

    assert!(result.is_err());

    assert_eq!(graph.edge_count(), 2);
}

// ============================================================================
// 4. DIFFERENTIATION SEMANTIC VALIDATION
// ============================================================================

struct TestOperationResolver;

impl DifferentiationOperationResolver<String>
    for TestOperationResolver
{
    fn resolve(
        &self,
        operation: &String,
        mode: &DifferentiationMode,
    ) -> OperationDifferentiabilityInfo {
        if operation == "test::multiply"
            && *mode == DifferentiationMode::Reverse
        {
            OperationDifferentiabilityInfo::differentiable(
                "test::multiply::reverse",
            )
        } else {
            OperationDifferentiabilityInfo::unknown(
                "operation semantics or derivative rule unavailable",
            )
        }
    }
}

/// Verifies differentiation validation using the actual graph
/// and operation resolver contracts.
#[test]
fn differentiation_validation_accepts_resolved_operation() {
    let mut graph =
        DifferentiationGraph::<u64, String>::new();

    graph
        .add_node(DifferentiationNode::new(
            1,
            DifferentiationNodeKind::Input,
        ))
        .unwrap();

    graph
        .add_node(DifferentiationNode::new(
            2,
            DifferentiationNodeKind::Operation(
                "test::multiply".to_owned(),
            ),
        ))
        .unwrap();

    graph
        .add_dependency(
            1,
            2,
            DifferentiationDependencyKind::Value,
        )
        .unwrap();

    let request = DifferentiationValidationInput {
        mode: DifferentiationMode::Reverse,
        result_kind: GradientResultKind::Gradient,
        variables: vec![
            DifferentiationVariable {
                node: 1,
                semantic_identity: Some("model.x".to_owned()),
            },
        ],
        outputs: vec![
            DifferentiationOutput {
                node: 2,
                semantic_identity: Some("model.y".to_owned()),
            },
        ],
        derivative_order: Some("1".to_owned()),
    };

    let report = DifferentiationValidator::default()
        .validate(
            &graph,
            &request,
            &TestOperationResolver,
        )
        .expect("validation must complete");

    assert!(report.is_valid());
    assert!(report.is_complete());

    assert_eq!(
        report.completeness(),
        DifferentiationValidationCompleteness::Complete,
    );
}

/// Unknown operation semantics must not be interpreted as proof
/// of differentiability.
#[test]
fn differentiation_validation_rejects_incomplete_semantics() {
    let mut graph =
        DifferentiationGraph::<u64, String>::new();

    graph
        .add_node(DifferentiationNode::new(
            1,
            DifferentiationNodeKind::Input,
        ))
        .unwrap();

    graph
        .add_node(DifferentiationNode::new(
            2,
            DifferentiationNodeKind::Operation(
                "test::unknown".to_owned(),
            ),
        ))
        .unwrap();

    graph
        .add_dependency(
            1,
            2,
            DifferentiationDependencyKind::Value,
        )
        .unwrap();

    let request = DifferentiationValidationInput {
        mode: DifferentiationMode::Reverse,
        result_kind: GradientResultKind::Gradient,
        variables: vec![
            DifferentiationVariable {
                node: 1,
                semantic_identity: None,
            },
        ],
        outputs: vec![
            DifferentiationOutput {
                node: 2,
                semantic_identity: None,
            },
        ],
        derivative_order: Some("1".to_owned()),
    };

    let report = DifferentiationValidator::default()
        .validate(
            &graph,
            &request,
            &TestOperationResolver,
        )
        .expect("validator should return an incomplete report");

    assert!(!report.is_valid());
    assert!(!report.is_complete());
}

// ============================================================================
// 5. CHECKPOINTING
// ============================================================================

/// Verifies checkpoint retention and recomputation planning.
#[test]
fn checkpoint_plan_supports_retention_and_recomputation() {
    let mut plan =
        CheckpointPlan::<String, String>::new(
            CheckpointPolicy::RetainAll,
        )
        .expect("checkpoint policy");

    plan.add_value(CheckpointValue {
        id: "activation".to_owned(),
        eligibility: CheckpointEligibility::eligible(
            Some(128),
        ),
    })
    .expect("checkpoint value");

    plan.add_operation(CheckpointOperation {
        id: "forward_operation".to_owned(),
        eligibility: RecomputeEligibility::eligible(
            Some(10),
        ),
    })
    .expect("recomputable operation");

    plan.set_value_action(
        "activation".to_owned(),
        CheckpointAction::Retain,
    )
    .expect("retention decision");

    plan.set_operation_recompute(
        "forward_operation".to_owned(),
        true,
        Some("memory-aware recomputation".to_owned()),
    )
    .expect("recomputation decision");

    assert!(plan.validate().is_ok());

    assert_eq!(plan.retained_value_count(), 1);
    assert_eq!(plan.recomputation_count(), 1);

    assert_eq!(plan.known_retained_bytes(), Some(128));
}

/// A checkpoint policy must reject retaining a value whose
/// authoritative owner declares it ineligible.
#[test]
fn checkpoint_plan_rejects_ineligible_values() {
    let mut plan =
        CheckpointPlan::<String, String>::new(
            CheckpointPolicy::RetainAll,
        )
        .unwrap();

    plan.add_value(CheckpointValue {
        id: "external_state".to_owned(),
        eligibility: CheckpointEligibility::ineligible(
            "external state cannot be retained by this planner",
        ),
    })
    .unwrap();

    let result = plan.set_value_action(
        "external_state".to_owned(),
        CheckpointAction::Retain,
    );

    assert!(result.is_err());
}

/// Checkpoint policies must preserve unknown sizes rather than
/// inventing resource estimates.
#[test]
fn checkpoint_plan_preserves_unknown_resource_sizes() {
    let mut plan =
        CheckpointPlan::<String, String>::new(
            CheckpointPolicy::RetainAll,
        )
        .unwrap();

    plan.add_value(CheckpointValue {
        id: "symbolic_activation".to_owned(),
        eligibility: CheckpointEligibility::eligible(None),
    })
    .unwrap();

    plan.set_value_action(
        "symbolic_activation".to_owned(),
        CheckpointAction::Retain,
    )
    .unwrap();

    assert_eq!(plan.known_retained_bytes(), None);
}

// ============================================================================
// 6. GRADIENT REQUEST INTEROPERABILITY
// ============================================================================

fn training_reference(value: &str) -> TrainingValueRef {
    TrainingValueRef::new(value)
        .expect("valid training reference")
}

/// Verifies that the gradient contract supports extensible strategies
/// and symbolic derivative order.
#[test]
fn gradient_request_supports_symbolic_order_and_custom_strategy() {
    let request = GradientRequest::new(
        training_reference("training.loss"),
        vec![
            training_reference("model.weights"),
            training_reference("model.bias"),
        ],
        GradientResultKind::Gradient,
    )
    .expect("valid gradient request")
    .with_order(
        training_reference("derivative.order"),
    )
    .expect("symbolic order")
    .with_strategy(
        GradientStrategyRequest::new(
            training_reference("future.differentiation.strategy"),
            StrategyIntent::Preferred,
        )
        .expect("custom strategy"),
    )
    .expect("strategy registration");

    assert!(request.validate().is_ok());

    assert_eq!(request.variables().len(), 2);

    assert_eq!(
        request.order().unwrap().as_str(),
        "derivative.order",
    );

    assert_eq!(request.strategies().len(), 1);
}

// ============================================================================
// 7. RULE REGISTRY AND DETERMINISM
// ============================================================================

/// Verifies that rule registration is shared across engine clones.
#[test]
fn differentiation_registry_is_shared_between_engines() {
    let registry = DifferentiationRuleRegistry::new();

    let first = ForwardModeEngine::new(
        registry.clone(),
        DifferentiationDomain::Classical,
    );

    let second = first.clone();

    first
        .registry()
        .register(make_rule(
            "test::multiply",
            "test::multiply::jvp",
            DifferentiationMode::Forward,
            version(),
        ))
        .unwrap();

    assert_eq!(
        second.registry().len().unwrap(),
        1,
    );
}

/// Verifies that rule metadata preserves exact versions.
#[test]
fn differentiation_rule_versions_are_preserved() {
    let registry = DifferentiationRuleRegistry::new();

    let requested_version =
        DifferentiationRuleVersion::new(3, 7, 11);

    registry
        .register(make_rule(
            "test::multiply",
            "test::multiply::jvp",
            DifferentiationMode::Forward,
            requested_version,
        ))
        .unwrap();

    let engine = ForwardModeEngine::new(
        registry,
        DifferentiationDomain::Classical,
    );

    let result = engine
        .propagate_with_version(
            operation_id("test::multiply"),
            requested_version,
            &[
                ForwardValue::new(2.0, 1.0),
                ForwardValue::new(3.0, 0.0),
            ],
            &mut ScalarMultiplicationJvp,
        )
        .expect("exact version must resolve");

    assert_eq!(
        result.rule_version(),
        requested_version,
    );
}

// ============================================================================
// 8. DYNAMIC WORKLOAD AND SCALABILITY
// ============================================================================

/// Verifies graph construction using dynamically generated collections.
///
/// The chosen test size is a workload fixture, not a language-level
/// or architectural capacity limit.
#[test]
fn differentiation_graph_supports_dynamically_sized_workloads() {
    let workload_size = std::env::var("ZAMANI_AI_TEST_GRAPH_SIZE")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(257);

    let mut graph =
        DifferentiationGraph::<usize, String>::new();

    for id in 0..=workload_size {
        let kind = if id == 0 {
            DifferentiationNodeKind::Input
        } else {
            DifferentiationNodeKind::Intermediate
        };

        graph
            .add_node(DifferentiationNode::new(id, kind))
            .expect("dynamic node insertion");
    }

    for id in 0..workload_size {
        graph
            .add_dependency(
                id,
                id + 1,
                DifferentiationDependencyKind::Value,
            )
            .expect("dynamic dependency insertion");
    }

    let report = graph
        .validate()
        .expect("dynamic graph validation");

    assert_eq!(
        report.node_count(),
        workload_size.saturating_add(1),
    );

    assert_eq!(
        report.edge_count(),
        workload_size,
    );

    assert_eq!(
        report.topological_order().len(),
        workload_size.saturating_add(1),
    );
}

/// Verifies that symbolic checkpoint sizes are supported without
/// assuming that every value is materialized.
#[test]
fn differentiation_checkpointing_supports_symbolic_values() {
    let mut plan =
        CheckpointPlan::<String, String>::new(
            CheckpointPolicy::RetainAll,
        )
        .unwrap();

    plan.add_value(CheckpointValue {
        id: "dynamic_tensor".to_owned(),
        eligibility: CheckpointEligibility::eligible(None),
    })
    .unwrap();

    plan.set_value_action(
        "dynamic_tensor".to_owned(),
        CheckpointAction::Deferred {
            provider: "target.checkpoint.provider".to_owned(),
        },
    )
    .unwrap();

    assert!(plan.validate().is_ok());
}

// ============================================================================
// 9. FAILURE TRANSPARENCY
// ============================================================================

/// Verifies that provider errors cannot be mistaken for successful
/// differentiation.
#[test]
fn differentiation_provider_failure_is_preserved() {
    struct FailingProvider;

    impl ForwardModeExecutor<f64, f64> for FailingProvider {
        type Error = NumericalProviderError;

        fn apply_jvp(
            &mut self,
            _rule: &DifferentiationRule,
            _inputs: &[ForwardValue<f64, f64>],
        ) -> Result<Vec<ForwardValue<f64, f64>>, Self::Error> {
            Err(NumericalProviderError::new(
                "intentional provider failure",
            ))
        }
    }

    let engine = ForwardModeEngine::new(
        multiplication_registry(),
        DifferentiationDomain::Classical,
    );

    let result = engine.propagate(
        operation_id("test::multiply"),
        &[],
        &mut FailingProvider,
    );

    assert!(matches!(
        result,
        Err(ForwardModeError::Provider(_))
    ));
}
