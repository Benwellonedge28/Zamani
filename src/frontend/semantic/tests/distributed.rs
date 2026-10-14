
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Integration-oriented conformance tests for distributed semantic analysis.
//!
//! Path: src/frontend/semantic/tests/distributed.rs
//!
//! PURPOSE
//!
//! Exercise the existing distributed semantic model, shared semantic-context
//! contract, communication validation, dependency analysis, failure-handling
//! intent, symbolic requirements, deterministic diagnostics, and grammar
//! ownership boundaries.
//!
//! AUTHORITY
//!
//! - grammar/Zamani.g4
//! - grammar/antlr/ZamaniParser.g4
//! - grammar/distributed/distributed.g4
//! - grammar/distributed/actors.g4
//! - grammar/distributed/channels.g4
//! - grammar/distributed/communication.g4
//! - grammar/distributed/consistency.g4
//! - grammar/distributed/topology.g4
//! - grammar/distributed/placement.g4
//! - grammar/distributed/replication.g4
//! - grammar/distributed/fault-tolerance.g4
//! - grammar/distributed/contracts.g4
//! - grammar/distributed/deployment.g4
//! - grammar/distributed/processes.g4
//! - grammar/distributed/tasks.g4
//! - grammar/networking/messages.g4
//! - grammar/spec/distributed.md
//! - src/frontend/semantic/domains/distributed.rs
//!
//! OWNERSHIP
//!
//! This file tests production APIs. It does not define a second distributed
//! semantic implementation, parser, AST, target model, or runtime.
//!
//! SCALABILITY
//!
//! Test collection sizes are workload samples, not language limits. No test
//! treats a sample size as a maximum participant, process, channel, node,
//! device, resource, or topology count. All tests remain bounded by the
//! test process's available memory and execution time.
//!
//! SAFETY
//!
//! Safe Rust only. No hardware discovery, network access, or execution of
//! distributed work is performed.
//!
//! INTEGRATION
//!
//! Register this module with `mod distributed;` in the semantic test module.
//! The parent test module must itself be registered by the semantic module.
//!
//! A complete context is explicitly implemented below so that tests can
//! distinguish local structural validation from missing shared-service
//! integration. This context is test evidence, not a production service.

use std::collections::{BTreeMap, BTreeSet};

use crate::frontend::semantic::domains::distributed::{
    validate_distributed,
    Communication,
    ConsistencyModel,
    ContextStatus,
    DeliverySemantics,
    Dependency,
    DependencyKind,
    DistributedId,
    DistributedProgram,
    DistributedRequirement,
    DistributedSemanticContext,
    DistributedValidationReport,
    Endpoint,
    FailureHandling,
    FailurePolicy,
    OrderingSemantics,
    Participant,
    ParticipantRole,
    QualifiedId,
    Severity,
    SourceOrigin,
    SymbolicExpr,
    SymbolicValue,
};

// ---------------------------------------------------------------------------
// Deterministic semantic-service fixtures
// ---------------------------------------------------------------------------

/// Supplies affirmative evidence for the semantic checks delegated to shared
/// services. It deliberately does not claim anything about a physical target.
struct CompleteContext;

impl DistributedSemanticContext for CompleteContext {
    fn validate_participant(&self, _: &Participant) -> ContextStatus {
        ContextStatus::Valid
    }

    fn validate_communication(&self, _: &Communication) -> ContextStatus {
        ContextStatus::Valid
    }

    fn validate_dependency(&self, _: &Dependency) -> ContextStatus {
        ContextStatus::Valid
    }

    fn validate_failure_handling(&self, _: &FailureHandling) -> ContextStatus {
        ContextStatus::Valid
    }

    fn validate_requirement(&self, _: &DistributedRequirement) -> ContextStatus {
        ContextStatus::Valid
    }
}

/// Represents an analyzer that has not yet connected the shared semantic
/// services. Missing checks must not be treated as successful validation.
struct IncompleteContext;

impl DistributedSemanticContext for IncompleteContext {}

/// Represents a shared service that has rejected a declaration.
struct RejectingContext;

impl DistributedSemanticContext for RejectingContext {
    fn validate_participant(&self, _: &Participant) -> ContextStatus {
        ContextStatus::Invalid {
            code: "TEST-PARTICIPANT-REJECTED".to_owned(),
            message: "deliberate test rejection".to_owned(),
        }
    }

    fn validate_communication(&self, _: &Communication) -> ContextStatus {
        ContextStatus::Valid
    }

    fn validate_dependency(&self, _: &Dependency) -> ContextStatus {
        ContextStatus::Valid
    }

    fn validate_failure_handling(&self, _: &FailureHandling) -> ContextStatus {
        ContextStatus::Valid
    }

    fn validate_requirement(&self, _: &DistributedRequirement) -> ContextStatus {
        ContextStatus::Valid
    }
}

/// Leaves custom dependency cycle policy unresolved to verify conservative
/// handling of unregistered semantic rules.
struct UnknownCustomCyclePolicy;

impl DistributedSemanticContext for UnknownCustomCyclePolicy {
    fn validate_participant(&self, _: &Participant) -> ContextStatus {
        ContextStatus::Valid
    }

    fn validate_communication(&self, _: &Communication) -> ContextStatus {
        ContextStatus::Valid
    }

    fn validate_dependency(&self, _: &Dependency) -> ContextStatus {
        ContextStatus::Valid
    }

    fn validate_failure_handling(&self, _: &FailureHandling) -> ContextStatus {
        ContextStatus::Valid
    }

    fn validate_requirement(&self, _: &DistributedRequirement) -> ContextStatus {
        ContextStatus::Valid
    }

    fn dependency_cycle_is_forbidden(&self, dependency: &Dependency) -> Option<bool> {
        match dependency.kind {
            DependencyKind::Custom => None,
            DependencyKind::Feedback => Some(false),
            DependencyKind::Data | DependencyKind::Control | DependencyKind::Ordering => {
                Some(true)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Construction helpers using the existing production types
// ---------------------------------------------------------------------------

fn id(value: impl Into<String>) -> DistributedId {
    DistributedId::new(value).expect("test identifier must be non-empty")
}

fn qid(namespace: impl Into<String>, name: impl Into<String>) -> QualifiedId {
    QualifiedId::new(namespace, name).expect("test qualified identifier must be valid")
}

fn origin(source: &str, node_id: &str) -> SourceOrigin {
    SourceOrigin::new(Some(source.to_owned()), Some(node_id.to_owned()))
}

fn participant(name: &str) -> Participant {
    Participant {
        id: id(name),
        role: ParticipantRole::Worker,
        kind: qid("zamani.distributed", "participant"),
        cardinality: None,
        required_capabilities: BTreeSet::new(),
        attributes: BTreeMap::new(),
        origin: SourceOrigin::default(),
    }
}

fn participant_with_cardinality(name: &str, cardinality: SymbolicExpr) -> Participant {
    Participant {
        cardinality: Some(cardinality),
        ..participant(name)
    }
}

fn communication(name: &str, from: &str, to: &str) -> Communication {
    Communication {
        id: id(name),
        kind: qid("zamani.distributed", "message-flow"),
        from: Endpoint {
            participant: id(from),
            port: None,
        },
        to: Endpoint {
            participant: id(to),
            port: None,
        },
        delivery: DeliverySemantics::AtLeastOnce,
        ordering: OrderingSemantics::Causal,
        consistency: Some(ConsistencyModel::Causal),
        required_capabilities: BTreeSet::new(),
        effects: BTreeSet::new(),
        attributes: BTreeMap::new(),
        origin: SourceOrigin::default(),
    }
}

fn dependency(from: &str, to: &str, kind: DependencyKind) -> Dependency {
    Dependency {
        from: id(from),
        to: id(to),
        kind,
        rule: None,
        origin: SourceOrigin::default(),
    }
}

fn requirement(name: &str, quantity: SymbolicExpr) -> DistributedRequirement {
    DistributedRequirement {
        kind: qid("zamani.resource", name),
        quantity,
        unit: None,
        required: true,
        origin: SourceOrigin::default(),
    }
}

fn report(program: &DistributedProgram) -> DistributedValidationReport {
    validate_distributed(program, &CompleteContext)
}

fn has_code(report: &DistributedValidationReport, expected: &str) -> bool {
    report.issues.iter().any(|issue| issue.code == expected)
}

// ---------------------------------------------------------------------------
// Identity and input-model contracts
// ---------------------------------------------------------------------------

#[test]
fn identifiers_reject_empty_or_whitespace_only_values() {
    assert!(DistributedId::new("").is_err());
    assert!(DistributedId::new(" \t\n ").is_err());
    assert!(QualifiedId::new("", "participant").is_err());
    assert!(QualifiedId::new("distributed", "").is_err());
    assert!(QualifiedId::new("  ", "send").is_err());
}

#[test]
fn identifiers_preserve_spelling_without_implicit_normalization() {
    let lower = id("Worker");
    let upper = id("worker");

    assert_ne!(lower, upper);
    assert_eq!(lower.as_str(), "Worker");
    assert_eq!(upper.as_str(), "worker");

    let qualified = qid("example.distributed", "send");
    assert_eq!(qualified.namespace(), "example.distributed");
    assert_eq!(qualified.name(), "send");
}

#[test]
fn empty_program_is_valid_when_no_semantic_service_is_needed() {
    let result = validate_distributed(&DistributedProgram::default(), &CompleteContext);

    assert!(result.complete);
    assert!(result.is_valid());
    assert!(!result.has_errors());
    assert!(result.issues.is_empty());
}

#[test]
fn missing_shared_service_evidence_is_not_reported_as_valid() {
    let program = DistributedProgram {
        participants: vec![participant("worker-a")],
        ..DistributedProgram::default()
    };

    let result = validate_distributed(&program, &IncompleteContext);

    assert!(!result.is_valid());
    assert!(!result.complete);
    assert!(result.has_errors());
    assert!(has_code(&result, "DIST-ANALYSIS-INCOMPLETE"));
}

#[test]
fn rejected_shared_semantic_evidence_produces_an_error() {
    let program = DistributedProgram {
        participants: vec![participant("worker-a")],
        ..DistributedProgram::default()
    };

    let result = validate_distributed(&program, &RejectingContext);

    assert!(result.has_errors());
    assert!(!result.is_valid());
    assert!(has_code(&result, "DIST-CONTEXT-INVALID"));
}

// ---------------------------------------------------------------------------
// Participant and communication validation
// ---------------------------------------------------------------------------

#[test]
fn communication_with_declared_endpoints_is_valid() {
    let program = DistributedProgram {
        participants: vec![participant("source"), participant("destination")],
        communications: vec![communication("flow", "source", "destination")],
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(result.complete);
    assert!(result.is_valid(), "{:#?}", result.issues);
}

#[test]
fn undeclared_source_and_destination_are_reported() {
    let program = DistributedProgram {
        communications: vec![communication("flow", "missing-source", "missing-target")],
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(result.has_errors());
    assert!(has_code(&result, "DIST-UNKNOWN-ENDPOINT"));
}

#[test]
fn duplicate_participant_identity_is_reported() {
    let program = DistributedProgram {
        participants: vec![participant("worker"), participant("worker")],
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(result.has_errors());
    assert!(has_code(&result, "DIST-DUPLICATE-PARTICIPANT"));
}

#[test]
fn duplicate_communication_identity_is_reported() {
    let program = DistributedProgram {
        participants: vec![participant("source"), participant("destination")],
        communications: vec![
            communication("flow", "source", "destination"),
            communication("flow", "source", "destination"),
        ],
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(result.has_errors());
    assert!(has_code(&result, "DIST-DUPLICATE-COMMUNICATION"));
}

#[test]
fn self_communication_is_a_warning_not_an_automatic_semantic_error() {
    let program = DistributedProgram {
        participants: vec![participant("worker")],
        communications: vec![communication("loopback", "worker", "worker")],
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(has_code(&result, "DIST-SELF-COMMUNICATION"));
    assert!(result
        .issues
        .iter()
        .filter(|issue| issue.code == "DIST-SELF-COMMUNICATION")
        .all(|issue| issue.severity == Severity::Warning));
    assert!(!result.has_errors());
    assert!(result.is_valid());
}

#[test]
fn exactly_once_intent_without_consistency_contract_is_reported_as_warning() {
    let mut flow = communication("flow", "source", "destination");
    flow.delivery = DeliverySemantics::ExactlyOnce;
    flow.consistency = None;

    let program = DistributedProgram {
        participants: vec![participant("source"), participant("destination")],
        communications: vec![flow],
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(has_code(&result, "DIST-EXACTLY-ONCE-CONTRACT"));
    assert!(result
        .issues
        .iter()
        .filter(|issue| issue.code == "DIST-EXACTLY-ONCE-CONTRACT")
        .all(|issue| issue.severity == Severity::Warning));
}

// ---------------------------------------------------------------------------
// Dependency and cycle semantics
// ---------------------------------------------------------------------------

#[test]
fn acyclic_ordering_dependencies_are_accepted() {
    let program = DistributedProgram {
        participants: vec![
            participant("stage-a"),
            participant("stage-b"),
            participant("stage-c"),
        ],
        dependencies: vec![
            dependency("stage-a", "stage-b", DependencyKind::Ordering),
            dependency("stage-b", "stage-c", DependencyKind::Ordering),
        ],
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(result.is_valid(), "{:#?}", result.issues);
    assert!(!has_code(&result, "DIST-DEPENDENCY-CYCLE"));
}

#[test]
fn forbidden_ordering_cycle_is_rejected() {
    let program = DistributedProgram {
        participants: vec![participant("a"), participant("b")],
        dependencies: vec![
            dependency("a", "b", DependencyKind::Ordering),
            dependency("b", "a", DependencyKind::Ordering),
        ],
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(result.has_errors());
    assert!(has_code(&result, "DIST-DEPENDENCY-CYCLE"));
}

#[test]
fn feedback_cycle_is_not_mistaken_for_an_ordering_cycle() {
    let program = DistributedProgram {
        participants: vec![participant("a"), participant("b")],
        dependencies: vec![
            dependency("a", "b", DependencyKind::Feedback),
            dependency("b", "a", DependencyKind::Feedback),
        ],
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(!has_code(&result, "DIST-DEPENDENCY-CYCLE"));
    assert!(result.is_valid(), "{:#?}", result.issues);
}

#[test]
fn unknown_dependency_endpoints_are_reported() {
    let program = DistributedProgram {
        participants: vec![participant("known")],
        dependencies: vec![dependency(
            "missing-source",
            "missing-target",
            DependencyKind::Ordering,
        )],
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(result.has_errors());
    assert!(has_code(&result, "DIST-DEPENDENCY-SOURCE"));
    assert!(has_code(&result, "DIST-DEPENDENCY-TARGET"));
}

#[test]
fn unregistered_custom_cycle_policy_makes_analysis_incomplete() {
    let program = DistributedProgram {
        participants: vec![participant("a"), participant("b")],
        dependencies: vec![dependency("a", "b", DependencyKind::Custom)],
        ..DistributedProgram::default()
    };

    let result = validate_distributed(&program, &UnknownCustomCyclePolicy);

    assert!(!result.is_valid());
    assert!(!result.complete);
    assert!(has_code(&result, "DIST-CYCLE-POLICY-UNKNOWN"));
}

#[test]
fn duplicate_dependency_edges_do_not_create_a_false_cycle() {
    let program = DistributedProgram {
        participants: vec![participant("a"), participant("b")],
        dependencies: vec![
            dependency("a", "b", DependencyKind::Ordering),
            dependency("a", "b", DependencyKind::Ordering),
        ],
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(!has_code(&result, "DIST-DEPENDENCY-CYCLE"));
    assert!(result.is_valid(), "{:#?}", result.issues);
}

// ---------------------------------------------------------------------------
// Symbolic quantities, resources, and checked arithmetic
// ---------------------------------------------------------------------------

#[test]
fn symbolic_expression_keeps_unknown_values_symbolic() {
    let expression = SymbolicExpr::add(
        SymbolicExpr::known(4),
        SymbolicExpr::symbol(id("worker_count")),
    );

    assert_eq!(expression.evaluate(), SymbolicValue::Symbolic);
}

#[test]
fn known_symbolic_expression_can_be_evaluated() {
    let expression = SymbolicExpr::multiply(
        SymbolicExpr::known(12),
        SymbolicExpr::known(9),
    );

    assert_eq!(expression.evaluate(), SymbolicValue::Known(108));
}

#[test]
fn checked_symbolic_arithmetic_reports_overflow() {
    let addition = SymbolicExpr::add(
        SymbolicExpr::known(u128::MAX),
        SymbolicExpr::known(1),
    );

    let multiplication = SymbolicExpr::multiply(
        SymbolicExpr::known(u128::MAX),
        SymbolicExpr::known(2),
    );

    assert_eq!(addition.evaluate(), SymbolicValue::Overflow);
    assert_eq!(multiplication.evaluate(), SymbolicValue::Overflow);
}

#[test]
fn symbolic_resource_requirements_are_not_target_feasibility_checks() {
    let symbolic_workers = SymbolicExpr::symbol(id("required_workers"));
    let program = DistributedProgram {
        participants: vec![participant_with_cardinality(
            "worker-pool",
            symbolic_workers.clone(),
        )],
        requirements: vec![requirement("compute.units", symbolic_workers)],
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(result.is_valid(), "{:#?}", result.issues);
    assert!(result.complete);
    // Valid semantic intent is not proof that a target can provide it.
}

#[test]
fn overflowing_participant_cardinality_is_reported() {
    let program = DistributedProgram {
        participants: vec![participant_with_cardinality(
            "worker-pool",
            SymbolicExpr::add(
                SymbolicExpr::known(u128::MAX),
                SymbolicExpr::known(1),
            ),
        )],
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(result.has_errors());
    assert!(has_code(&result, "DIST-SYMBOLIC-OVERFLOW"));
}

#[test]
fn overflowing_required_resource_quantity_is_reported() {
    let program = DistributedProgram {
        requirements: vec![requirement(
            "memory.bytes",
            SymbolicExpr::multiply(
                SymbolicExpr::known(u128::MAX),
                SymbolicExpr::known(2),
            ),
        )],
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(result.has_errors());
    assert!(has_code(&result, "DIST-SYMBOLIC-OVERFLOW"));
}

// ---------------------------------------------------------------------------
// Failure-handling intent
// ---------------------------------------------------------------------------

#[test]
fn zero_retry_attempts_are_rejected() {
    let program = DistributedProgram {
        participants: vec![participant("worker")],
        failure_handling: vec![FailureHandling {
            subject: id("worker"),
            policy: FailurePolicy::Retry {
                attempts: SymbolicExpr::known(0),
            },
            required_capabilities: BTreeSet::new(),
            effects: BTreeSet::new(),
            origin: SourceOrigin::default(),
        }],
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(result.has_errors());
    assert!(has_code(&result, "DIST-RETRY-ZERO"));
}

#[test]
fn retry_overflow_is_rejected() {
    let program = DistributedProgram {
        participants: vec![participant("worker")],
        failure_handling: vec![FailureHandling {
            subject: id("worker"),
            policy: FailurePolicy::Retry {
                attempts: SymbolicExpr::add(
                    SymbolicExpr::known(u128::MAX),
                    SymbolicExpr::known(1),
                ),
            },
            required_capabilities: BTreeSet::new(),
            effects: BTreeSet::new(),
            origin: SourceOrigin::default(),
        }],
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(result.has_errors());
    assert!(has_code(&result, "DIST-SYMBOLIC-OVERFLOW"));
}

#[test]
fn failure_handling_for_unknown_subject_is_rejected() {
    let program = DistributedProgram {
        failure_handling: vec![FailureHandling {
            subject: id("missing"),
            policy: FailurePolicy::Propagate,
            required_capabilities: BTreeSet::new(),
            effects: BTreeSet::new(),
            origin: SourceOrigin::default(),
        }],
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(result.has_errors());
    assert!(has_code(&result, "DIST-FAILURE-SUBJECT"));
}

#[test]
fn custom_failure_policy_is_preserved_for_shared_validation() {
    let handling = FailureHandling {
        subject: id("worker"),
        policy: FailurePolicy::Custom(qid("example.failure", "recover")),
        required_capabilities: BTreeSet::new(),
        effects: BTreeSet::new(),
        origin: SourceOrigin::default(),
    };

    let program = DistributedProgram {
        participants: vec![participant("worker")],
        failure_handling: vec![handling],
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(result.is_valid(), "{:#?}", result.issues);
}

// ---------------------------------------------------------------------------
// Source provenance and deterministic diagnostics
// ---------------------------------------------------------------------------

#[test]
fn issue_origin_is_preserved_for_unknown_communication_endpoint() {
    let mut flow = communication("flow", "source", "missing");
    flow.origin = origin("fixtures/distributed.zm", "communication-17");

    let program = DistributedProgram {
        participants: vec![participant("source")],
        communications: vec![flow],
        ..DistributedProgram::default()
    };

    let result = report(&program);
    let issue = result
        .issues
        .iter()
        .find(|issue| issue.code == "DIST-UNKNOWN-ENDPOINT")
        .expect("unknown endpoint must produce a diagnostic");

    assert_eq!(
        issue.origin.source.as_deref(),
        Some("fixtures/distributed.zm")
    );
    assert_eq!(issue.origin.node_id.as_deref(), Some("communication-17"));
}

#[test]
fn repeated_analysis_produces_identical_reports() {
    let program = DistributedProgram {
        participants: vec![participant("worker")],
        communications: vec![
            communication("flow-z", "worker", "missing-z"),
            communication("flow-a", "worker", "missing-a"),
        ],
        dependencies: vec![dependency(
            "missing-source",
            "missing-target",
            DependencyKind::Ordering,
        )],
        ..DistributedProgram::default()
    };

    let first = report(&program);
    let second = report(&program);
    let third = report(&program);

    assert_eq!(first, second);
    assert_eq!(second, third);
}

#[test]
fn diagnostic_order_is_stable_for_a_fixed_input() {
    let program = DistributedProgram {
        participants: vec![participant("worker")],
        communications: vec![
            communication("flow-z", "worker", "missing-z"),
            communication("flow-a", "worker", "missing-a"),
        ],
        ..DistributedProgram::default()
    };

    let result = report(&program);
    let keys: Vec<_> = result
        .issues
        .iter()
        .map(|issue| {
            (
                issue.subject.clone(),
                issue.code,
                issue.severity,
                issue.message.clone(),
            )
        })
        .collect();

    let mut sorted = keys.clone();
    sorted.sort();

    assert_eq!(keys, sorted);
}

// ---------------------------------------------------------------------------
// Bounded stress tests: workload samples, never architectural limits
// ---------------------------------------------------------------------------

#[test]
fn large_participant_collection_is_validated_without_fixed_capacity_assumptions() {
    let participant_count = 2_048usize;
    let participants: Vec<_> = (0..participant_count)
        .map(|index| participant(&format!("participant-{index}")))
        .collect();

    let program = DistributedProgram {
        participants,
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(result.complete);
    assert!(result.is_valid(), "{:#?}", result.issues);
}

#[test]
fn large_acyclic_dependency_graph_is_processed_iteratively() {
    let node_count = 2_048usize;

    let participants: Vec<_> = (0..node_count)
        .map(|index| participant(&format!("stage-{index}")))
        .collect();

    let dependencies: Vec<_> = (1..node_count)
        .map(|index| {
            dependency(
                &format!("stage-{}", index - 1),
                &format!("stage-{index}"),
                DependencyKind::Ordering,
            )
        })
        .collect();

    let program = DistributedProgram {
        participants,
        dependencies,
        ..DistributedProgram::default()
    };

    let result = report(&program);

    assert!(result.complete);
    assert!(result.is_valid(), "{:#?}", result.issues);
    assert!(!has_code(&result, "DIST-DEPENDENCY-CYCLE"));
}

#[test]
fn many_communications_keep_diagnostics_deterministic() {
    let communication_count = 512usize;

    let participants = vec![participant("source"), participant("destination")];
    let communications: Vec<_> = (0..communication_count)
        .map(|index| communication(
            &format!("flow-{index}"),
            "source",
            "destination",
        ))
        .collect();

    let program = DistributedProgram {
        participants,
        communications,
        ..DistributedProgram::default()
    };

    let first = report(&program);
    let second = report(&program);

    assert!(first.is_valid(), "{:#?}", first.issues);
    assert_eq!(first, second);
}

// ---------------------------------------------------------------------------
// Grammar and specification integration guards
// ---------------------------------------------------------------------------

#[test]
fn canonical_distributed_grammar_exposes_the_domain_boundary() {
    let grammar = include_str!("../../../../grammar/distributed/distributed.g4");

    assert!(grammar.contains("parser grammar Distributed;"));
    assert!(grammar.contains("distributedDeclaration"));
    assert!(grammar.contains("distributedStatement"));
    assert!(grammar.contains("distributedExpression"));
}

#[test]
fn distributed_grammar_reuses_canonical_lexer_vocabulary() {
    let grammar = include_str!("../../../../grammar/distributed/distributed.g4");

    assert!(grammar.contains("tokenVocab = ZamaniLexer;"));
    assert!(!grammar.contains("lexer grammar DistributedLexer;"));
}

#[test]
fn distributed_leaf_grammars_are_present_and_non_empty() {
    let leaves: &[(&str, &str)] = &[
        (
            "actors",
            include_str!("../../../../grammar/distributed/actors.g4"),
        ),
        (
            "channels",
            include_str!("../../../../grammar/distributed/channels.g4"),
        ),
        (
            "communication",
            include_str!("../../../../grammar/distributed/communication.g4"),
        ),
        (
            "consistency",
            include_str!("../../../../grammar/distributed/consistency.g4"),
        ),
        (
            "topology",
            include_str!("../../../../grammar/distributed/topology.g4"),
        ),
        (
            "placement",
            include_str!("../../../../grammar/distributed/placement.g4"),
        ),
        (
            "replication",
            include_str!("../../../../grammar/distributed/replication.g4"),
        ),
        (
            "fault-tolerance",
            include_str!("../../../../grammar/distributed/fault-tolerance.g4"),
        ),
        (
            "contracts",
            include_str!("../../../../grammar/distributed/contracts.g4"),
        ),
        (
            "deployment",
            include_str!("../../../../grammar/distributed/deployment.g4"),
        ),
        (
            "processes",
            include_str!("../../../../grammar/distributed/processes.g4"),
        ),
        (
            "tasks",
            include_str!("../../../../grammar/distributed/tasks.g4"),
        ),
        (
            "networking messages",
            include_str!("../../../../grammar/networking/messages.g4"),
        ),
    ];

    for (name, content) in leaves {
        assert!(
            !content.trim().is_empty(),
            "the {name} grammar must not be empty"
        );
    }
}

#[test]
fn distributed_specification_defines_portability_and_semantic_boundaries() {
    let specification = include_str!("../../../../grammar/spec/distributed.md");

    assert!(specification.contains("POCO-REAF"));
    assert!(specification.contains("placement"));
    assert!(specification.contains("scheduling"));
    assert!(specification.contains("resource"));
}

#[test]
fn canonical_parser_composition_root_exists() {
    let parser_root = include_str!("../../../../grammar/antlr/ZamaniParser.g4");
    let grammar_root = include_str!("../../../../grammar/Zamani.g4");

    assert!(parser_root.contains("parser grammar ZamaniParser;"));
    assert!(grammar_root.contains("grammar/Zamani.g4"));
}
