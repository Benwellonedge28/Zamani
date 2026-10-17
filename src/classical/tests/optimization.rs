
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Contract and regression tests for Zamani's classical optimization subsystem.
//!
//! File: `src/classical/tests/optimization.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Authority and ownership
//!
//! Normative semantic authority:
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! Implementation under test:
//! - `src/classical/optimization/passes.rs`
//! - `src/classical/optimization/legality.rs`
//! - `src/classical/optimization/cost_model.rs`
//! - `src/classical/optimization/vectorization.rs`
//! - `src/classical/optimization/loop_transform.rs`
//! - `src/classical/optimization/parallelization.rs`
//! - `src/classical/optimization/memory_planning.rs`
//!
//! The grammar owns syntax, semantic analysis owns source-language meaning,
//! and the selected canonical IR owns its representation and invariants.
//! These tests do not introduce an AST, IR, scheduler, or optimizer of their
//! own. Test targets are small adapters for exercising the existing contracts.
//!
//! # Invariants tested
//!
//! - Missing or disproven legality evidence never authorizes a transformation.
//! - Cost and feasibility are independent of semantic legality.
//! - Candidate selection is deterministic and respects explicit budgets.
//! - Pass registration order is execution order.
//! - Duplicate passes, invalid targets, failed passes, and budget exhaustion
//!   are explicit errors rather than silent success.
//! - Memory plans preserve symbolic sizes and validate evidence revisions.
//! - Target-specific vectorization and parallelization decisions are plans,
//!   not claims that a transformation has already been applied.
//!
//! Test workload sizes are examples, not universal language or hardware limits.
//! Operational budgets are explicit policy. Physical resource availability,
//! target discovery, IR transformation, and execution are outside the scope
//! of these unit tests.
//!
//! # Registration
//!
//! Register this file exactly once in the repository's intended test module.
//! For example, an owning test module may use:
//!
//! `#[path = "optimization.rs"] mod optimization;`
//!
//! Adjust that path only if the chosen parent module has a different location.
//! Do not create duplicate production modules to satisfy test imports.

use crate::classical::optimization::cost_model::{
    select_best, CandidateFeasibility, ComparisonBasis, CostDimension, CostEstimate,
    CostModelError, CostModelPolicy, OptimizationCandidate, RejectionReason,
};
use crate::classical::optimization::legality::{
    assess_legality, LegalityEvidence, LegalityProperty, LegalityStatus, ProofEvidence,
    TransformationContract,
};
use crate::classical::optimization::memory_planning::{
    plan_memory, MemoryBudgetAssessment, MemoryObject, MemoryPlanningError,
    MemoryPlanningPolicy, MemorySize,
};
use crate::classical::optimization::passes::{
    OptimizationBudget, OptimizationError, OptimizationPass, OptimizationResult,
    OptimizationTarget, PassContext, PassId, PassManager, PassManagerConfig, PassOutcome,
    VerificationPolicy,
};

// ---------------------------------------------------------------------------
// Test-only canonical-IR adapter and pass implementations
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
struct TestTarget {
    valid: bool,
    revision: u64,
}

impl TestTarget {
    fn valid() -> Self {
        Self {
            valid: true,
            revision: 0,
        }
    }

    fn invalid() -> Self {
        Self {
            valid: false,
            revision: 0,
        }
    }
}

impl OptimizationTarget for TestTarget {
    fn verify(&self) -> Result<(), String> {
        if self.valid {
            Ok(())
        } else {
            Err("test target violates its invariant".to_owned())
        }
    }
}

struct CountingPass {
    id: PassId,
    changes: u64,
    calls: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl CountingPass {
    fn new(id: &str, changes: u64) -> Self {
        Self {
            id: PassId::new(id).expect("test pass identifier must be valid"),
            changes,
            calls: std::sync::Arc::default(),
        }
    }

    fn with_counter(
        id: &str,
        changes: u64,
        calls: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    ) -> Self {
        Self {
            id: PassId::new(id).expect("test pass identifier must be valid"),
            changes,
            calls,
        }
    }
}

impl OptimizationPass for CountingPass {
    fn id(&self) -> &PassId {
        &self.id
    }

    fn description(&self) -> &str {
        "Test-only optimization pass"
    }

    fn run(
        &self,
        _target: &mut dyn OptimizationTarget,
        _context: &PassContext,
    ) -> OptimizationResult<PassOutcome> {
        self.calls
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        Ok(PassOutcome::changed(self.changes))
    }
}

struct FailingPass {
    id: PassId,
}

impl FailingPass {
    fn new(id: &str) -> Self {
        Self {
            id: PassId::new(id).expect("test pass identifier must be valid"),
        }
    }
}

impl OptimizationPass for FailingPass {
    fn id(&self) -> &PassId {
        &self.id
    }

    fn description(&self) -> &str {
        "Test-only failing optimization pass"
    }

    fn run(
        &self,
        _target: &mut dyn OptimizationTarget,
        _context: &PassContext,
    ) -> OptimizationResult<PassOutcome> {
        Err(OptimizationError::InvalidOptionKey {
            key: String::new(),
        })
    }
}

fn legal_candidate(id: &str, estimate: u128) -> OptimizationCandidate {
    let latency = CostDimension::new("latency", "ns")
        .expect("test cost dimension must be valid");

    let mut candidate = OptimizationCandidate::new(
        id,
        LegalityStatus::Legal,
        CandidateFeasibility::feasible(),
    )
    .expect("test candidate must be valid");

    candidate.set_estimate(latency, CostEstimate::exact(estimate));
    candidate
}

fn pass_manager_config() -> PassManagerConfig {
    PassManagerConfig {
        budget: OptimizationBudget::unlimited(),
        verification: VerificationPolicy::EveryPass,
    }
}

// ---------------------------------------------------------------------------
// Pass identity, context, registration, and execution
// ---------------------------------------------------------------------------

#[test]
fn pass_ids_reject_empty_whitespace_and_control_characters() {
    assert!(PassId::new("").is_err());
    assert!(PassId::new(" \t ").is_err());
    assert!(PassId::new("invalid\nidentifier").is_err());

    assert!(PassId::new("classical.constant-folding").is_ok());
    assert!(PassId::new("extension.optimization.v2").is_ok());
}

#[test]
fn pass_context_validates_keys_and_keeps_options_in_key_order() {
    let mut context = PassContext::new();

    context
        .set_option("numeric.mode", "strict")
        .expect("valid option should be accepted");

    context
        .set_option("loop.strategy", "conservative")
        .expect("valid option should be accepted");

    assert_eq!(context.option("numeric.mode"), Some("strict"));
    assert_eq!(context.option("missing"), None);

    let keys: Vec<_> = context.options().keys().map(String::as_str).collect();
    assert_eq!(keys, vec!["loop.strategy", "numeric.mode"]);

    assert!(context.set_option("", "value").is_err());
    assert!(context.set_option("invalid\nkey", "value").is_err());
}

#[test]
fn pass_manager_preserves_registration_order() {
    let mut manager = PassManager::new(pass_manager_config());

    manager
        .register(Box::new(CountingPass::new("first", 1)))
        .expect("first pass should register");

    manager
        .register(Box::new(CountingPass::new("second", 2)))
        .expect("second pass should register");

    assert_eq!(manager.len(), 2);
    assert!(!manager.is_empty());

    let ids: Vec<_> = manager
        .pass_ids()
        .iter()
        .map(|id| id.as_str())
        .collect();

    assert_eq!(ids, vec!["first", "second"]);
}

#[test]
fn pass_manager_rejects_duplicate_ids() {
    let mut manager = PassManager::default();

    manager
        .register(Box::new(CountingPass::new("duplicate", 0)))
        .expect("first registration should succeed");

    assert!(matches!(
        manager.register(Box::new(CountingPass::new("duplicate", 0))),
        Err(OptimizationError::DuplicatePass { .. })
    ));

    assert_eq!(manager.len(), 1);
}

#[test]
fn pass_manager_verifies_target_and_reports_accounting() {
    let mut manager = PassManager::new(pass_manager_config());

    manager
        .register(Box::new(CountingPass::new("fold", 3)))
        .expect("pass should register");

    manager
        .register(Box::new(CountingPass::new("simplify", 2)))
        .expect("pass should register");

    let mut target = TestTarget::valid();

    let report = manager
        .run(&mut target, &PassContext::new())
        .expect("valid target and successful passes should complete");

    assert_eq!(report.pass_invocations, 2);
    assert_eq!(report.total_changes, 5);
    assert_eq!(report.passes.len(), 2);
    assert!(!report.is_unchanged());
}

#[test]
fn pass_manager_rejects_invalid_initial_target() {
    let manager = PassManager::default();
    let mut target = TestTarget::invalid();

    assert!(matches!(
        manager.run(&mut target, &PassContext::new()),
        Err(OptimizationError::InitialVerificationFailed { .. })
    ));
}

#[test]
fn pass_manager_enforces_invocation_budget_before_running_next_pass() {
    let calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let config = PassManagerConfig {
        budget: OptimizationBudget::new(Some(1), None),
        verification: VerificationPolicy::EveryPass,
    };

    let mut manager = PassManager::new(config);

    manager
        .register(Box::new(CountingPass::with_counter(
            "first",
            0,
            std::sync::Arc::clone(&calls),
        )))
        .expect("first pass should register");

    manager
        .register(Box::new(CountingPass::with_counter(
            "second",
            0,
            std::sync::Arc::clone(&calls),
        )))
        .expect("second pass should register");

    let mut target = TestTarget::valid();

    assert!(matches!(
        manager.run(&mut target, &PassContext::new()),
        Err(OptimizationError::InvocationBudgetExceeded { limit: 1, .. })
    ));

    assert_eq!(
        calls.load(std::sync::atomic::Ordering::SeqCst),
        1,
        "the pass beyond the configured invocation budget must not run"
    );
}

#[test]
fn pass_manager_reports_change_budget_exhaustion() {
    let config = PassManagerConfig {
        budget: OptimizationBudget::new(None, Some(1)),
        verification: VerificationPolicy::EveryPass,
    };

    let mut manager = PassManager::new(config);

    manager
        .register(Box::new(CountingPass::new("changes-two", 2)))
        .expect("pass should register");

    let mut target = TestTarget::valid();

    assert!(matches!(
        manager.run(&mut target, &PassContext::new()),
        Err(OptimizationError::ChangeBudgetExceeded { limit: 1, .. })
    ));
}

#[test]
fn pass_manager_reports_pass_failure_without_claiming_success() {
    let mut manager = PassManager::default();

    manager
        .register(Box::new(FailingPass::new("deliberate-failure")))
        .expect("pass should register");

    let mut target = TestTarget::valid();

    assert!(matches!(
        manager.run(&mut target, &PassContext::new()),
        Err(OptimizationError::PassFailed { .. })
    ));
}

// ---------------------------------------------------------------------------
// Conservative legality assessment
// ---------------------------------------------------------------------------

#[test]
fn semantic_preservation_is_mandatory_even_for_empty_extra_contracts() {
    let pass_id = PassId::new("test.transformation").expect("valid pass id");
    let contract = TransformationContract::new(pass_id);

    assert!(
        contract
            .obligations()
            .contains(&LegalityProperty::SemanticPreservation)
    );

    let report = assess_legality(&contract, &LegalityEvidence::new());

    assert_eq!(report.status(), LegalityStatus::Unproven);
    assert!(report.require_legal().is_err());
}

#[test]
fn missing_legality_evidence_fails_closed() {
    let pass_id = PassId::new("test.vectorize").expect("valid pass id");
    let mut contract = TransformationContract::new(pass_id);

    contract.require(LegalityProperty::Aliasing);
    contract.require(LegalityProperty::FloatingPointSemantics);

    let mut evidence = LegalityEvidence::new();
    evidence.record(
        LegalityProperty::SemanticPreservation,
        ProofEvidence::proven("test proof for semantic preservation"),
    );

    let report = assess_legality(&contract, &evidence);

    assert_eq!(report.status(), LegalityStatus::Unproven);
    assert_eq!(report.blocking_findings().len(), 2);
    assert!(report.require_legal().is_err());
}

#[test]
fn disproven_legality_evidence_rejects_transformation() {
    let pass_id = PassId::new("test.reassociate").expect("valid pass id");
    let mut contract = TransformationContract::new(pass_id);
    contract.require(LegalityProperty::FloatingPointSemantics);

    let mut evidence = LegalityEvidence::new();
    evidence.record(
        LegalityProperty::SemanticPreservation,
        ProofEvidence::proven("source meaning is preserved"),
    );
    evidence.record(
        LegalityProperty::FloatingPointSemantics,
        ProofEvidence::disproven("reassociation changes required rounding behavior"),
    );

    let report = assess_legality(&contract, &evidence);

    assert_eq!(report.status(), LegalityStatus::Illegal);
    assert!(report.require_legal().is_err());
}

#[test]
fn only_complete_proof_evidence_authorizes_legality() {
    let pass_id = PassId::new("test.safe-transform").expect("valid pass id");
    let mut contract = TransformationContract::new(pass_id);
    contract.require_all([
        LegalityProperty::TypeCorrectness,
        LegalityProperty::Effects,
        LegalityProperty::Concurrency,
    ]);

    let mut evidence = LegalityEvidence::new();

    for property in contract.obligations() {
        evidence.record(
            property.clone(),
            ProofEvidence::proven("provided by test fixture"),
        );
    }

    let report = assess_legality(&contract, &evidence);

    assert_eq!(report.status(), LegalityStatus::Legal);
    assert!(report.blocking_findings().is_empty());
    assert!(report.require_legal().is_ok());
}

#[test]
fn extension_legality_properties_are_open_world_but_validated() {
    let custom =
        LegalityProperty::custom("vendor.example.preserve-device-ordering")
            .expect("namespaced extension property should be accepted");

    assert_eq!(
        custom.as_str(),
        "vendor.example.preserve-device-ordering"
    );

    assert!(LegalityProperty::custom("").is_err());
    assert!(LegalityProperty::custom("invalid\nproperty").is_err());

    assert!(LegalityProperty::custom("semantic-preservation").is_err());
}

// ---------------------------------------------------------------------------
// Cost estimation, feasibility, budgets, and deterministic selection
// ---------------------------------------------------------------------------

#[test]
fn cost_dimensions_include_units_and_reject_invalid_identifiers() {
    let latency = CostDimension::new("latency", "ns")
        .expect("valid cost dimension");

    let memory = CostDimension::new("memory", "bytes")
        .expect("valid cost dimension");

    assert_ne!(latency, memory);
    assert_eq!(latency.name(), "latency");
    assert_eq!(latency.unit(), "ns");

    assert!(CostDimension::new("", "bytes").is_err());
    assert!(CostDimension::new("latency", "").is_err());
}

#[test]
fn cost_estimate_ranges_are_checked() {
    assert!(CostEstimate::range(1, 2, 3).is_ok());
    assert!(CostEstimate::range(3, 2, 4).is_err());
    assert!(CostEstimate::range(1, 4, 3).is_err());

    let symbolic = CostEstimate::symbolic("work(n)")
        .expect("valid symbolic cost expression");

    assert_eq!(symbolic.as_symbolic(), Some("work(n)"));
    assert!(!symbolic.is_numeric());
    assert_eq!(CostEstimate::exact(u128::MAX).as_exact(), Some(u128::MAX));
}

#[test]
fn cost_selection_chooses_lowest_legal_feasible_candidate() {
    let latency = CostDimension::new("latency", "ns")
        .expect("valid cost dimension");

    let policy = CostModelPolicy::new(
        [latency],
        ComparisonBasis::Expected,
    )
    .expect("at least one objective is provided");

    let candidates = [
        legal_candidate("candidate.slower", 30),
        legal_candidate("candidate.faster", 10),
        legal_candidate("candidate.middle", 20),
    ];

    let report = select_best(candidates, &policy)
        .expect("all candidates have comparable numeric estimates");

    assert!(report.has_selection());
    assert_eq!(
        report.selected.as_ref().map(OptimizationCandidate::id),
        Some("candidate.faster")
    );

    assert_eq!(
        report
            .ranking
            .iter()
            .map(|candidate| candidate.candidate_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "candidate.faster",
            "candidate.middle",
            "candidate.slower"
        ]
    );
}

#[test]
fn cost_selection_does_not_treat_low_cost_as_legality_proof() {
    let latency = CostDimension::new("latency", "ns")
        .expect("valid cost dimension");

    let policy = CostModelPolicy::new(
        [latency.clone()],
        ComparisonBasis::Expected,
    )
    .expect("valid policy");

    let mut illegal = OptimizationCandidate::new(
        "candidate.illegal",
        LegalityStatus::Unproven,
        CandidateFeasibility::feasible(),
    )
    .expect("candidate is structurally valid");

    illegal.set_estimate(latency, CostEstimate::exact(0));

    let report = select_best([illegal], &policy)
        .expect("unproven candidates are rejected, not selected");

    assert!(!report.has_selection());
    assert_eq!(report.rejected.len(), 1);

    assert!(matches!(
        report.rejected[0].reason,
        RejectionReason::LegalityNotProven {
            status: LegalityStatus::Unproven
        }
    ));
}

#[test]
fn cost_selection_does_not_assume_unknown_target_feasibility() {
    let latency = CostDimension::new("latency", "ns")
        .expect("valid cost dimension");

    let policy = CostModelPolicy::new(
        [latency.clone()],
        ComparisonBasis::Expected,
    )
    .expect("valid policy");

    let mut candidate = OptimizationCandidate::new(
        "candidate.unknown-feasibility",
        LegalityStatus::Legal,
        CandidateFeasibility::unknown("target support has not been established"),
    )
    .expect("candidate is structurally valid");

    candidate.set_estimate(latency, CostEstimate::exact(1));

    let report = select_best([candidate], &policy)
        .expect("unknown feasibility must not cause a panic");

    assert!(!report.has_selection());
    assert_eq!(report.rejected.len(), 1);

    assert!(matches!(
        report.rejected[0].reason,
        RejectionReason::FeasibilityNotEstablished { .. }
    ));
}

#[test]
fn explicit_cost_budget_is_separate_from_program_semantics() {
    let latency = CostDimension::new("latency", "ns")
        .expect("valid cost dimension");

    let policy = CostModelPolicy::new(
        [latency.clone()],
        ComparisonBasis::Expected,
    )
    .expect("valid policy")
    .with_budget(latency, 10);

    let report = select_best(
        [legal_candidate("candidate.over-budget", 11)],
        &policy,
    )
    .expect("over-budget candidates should be reported");

    assert!(!report.has_selection());
    assert!(matches!(
        report.rejected[0].reason,
        RejectionReason::BudgetExceeded {
            upper_bound: 11,
            maximum: 10,
            ..
        }
    ));
}

#[test]
fn cost_selection_rejects_duplicate_candidate_ids() {
    let latency = CostDimension::new("latency", "ns")
        .expect("valid cost dimension");

    let policy = CostModelPolicy::new(
        [latency],
        ComparisonBasis::Expected,
    )
    .expect("valid policy");

    assert!(matches!(
        select_best(
            [
                legal_candidate("same-id", 1),
                legal_candidate("same-id", 2)
            ],
            &policy
        ),
        Err(CostModelError::DuplicateCandidate { .. })
    ));
}

#[test]
fn cost_selection_is_stable_for_equal_objective_values() {
    let latency = CostDimension::new("latency", "ns")
        .expect("valid cost dimension");

    let policy = CostModelPolicy::new(
        [latency],
        ComparisonBasis::Expected,
    )
    .expect("valid policy");

    let first = select_best(
        [
            legal_candidate("zeta", 5),
            legal_candidate("alpha", 5),
            legal_candidate("mu", 5),
        ],
        &policy,
    )
    .expect("candidates have comparable costs");

    let second = select_best(
        [
            legal_candidate("mu", 5),
            legal_candidate("zeta", 5),
            legal_candidate("alpha", 5),
        ],
        &policy,
    )
    .expect("candidates have comparable costs");

    assert_eq!(
        first
            .ranking
            .iter()
            .map(|candidate| candidate.candidate_id.as_str())
            .collect::<Vec<_>>(),
        second
            .ranking
            .iter()
            .map(|candidate| candidate.candidate_id.as_str())
            .collect::<Vec<_>>()
    );
}

// ---------------------------------------------------------------------------
// Symbolic memory planning, budgets, and evidence revision safety
// ---------------------------------------------------------------------------

fn memory_object(
    id: &str,
    size: MemorySize,
    revision: &str,
) -> MemoryObject {
    MemoryObject::new(
        id,
        size,
        1,
        "logical.default",
        "general",
        id,
        revision,
    )
    .expect("test memory object must be valid")
}

#[test]
fn symbolic_memory_sizes_are_not_silently_treated_as_zero() {
    let symbolic = MemorySize::symbolic("element_count * element_size")
        .expect("valid symbolic expression");

    assert!(!symbolic.is_exact());
    assert_eq!(symbolic.exact_bytes(), None);

    let object = memory_object("buffer", symbolic, "analysis.1");

    let plan = plan_memory(
        "analysis.1",
        [object],
        MemoryPlanningPolicy::new().with_max_reserved_bytes(1024),
    )
    .expect("symbolic memory size should remain representable");

    assert!(!plan.all_sizes_known());
    assert_eq!(plan.unknown_size_slots(), 1);
    assert_eq!(plan.budget_assessment(), MemoryBudgetAssessment::Unresolved);
}

#[test]
fn memory_planning_rejects_stale_analysis_evidence() {
    let object = memory_object(
        "buffer",
        MemorySize::Exact(64),
        "analysis.old",
    );

    assert!(matches!(
        plan_memory(
            "analysis.current",
            [object],
            MemoryPlanningPolicy::new()
        ),
        Err(MemoryPlanningError::StaleAnalysisEvidence { .. })
    ));
}

#[test]
fn memory_planning_rejects_duplicate_object_ids() {
    let first = memory_object(
        "duplicate",
        MemorySize::Exact(8),
        "analysis.1",
    );

    let second = memory_object(
        "duplicate",
        MemorySize::Exact(16),
        "analysis.1",
    );

    assert!(matches!(
        plan_memory(
            "analysis.1",
            [first, second],
            MemoryPlanningPolicy::new()
        ),
        Err(MemoryPlanningError::DuplicateObjectId { .. })
    ));
}

#[test]
fn memory_planning_rejects_zero_alignment() {
    assert!(matches!(
        MemoryObject::new(
            "buffer",
            MemorySize::Exact(8),
            0,
            "logical.default",
            "general",
            "lifetime.1",
            "analysis.1",
        ),
        Err(MemoryPlanningError::InvalidAlignment { .. })
    ));
}

#[test]
fn memory_planning_accepts_large_exact_sizes_without_machine_size_casts() {
    let object = memory_object(
        "large.logical.buffer",
        MemorySize::Exact(u128::MAX),
        "analysis.1",
    );

    let plan = plan_memory(
        "analysis.1",
        [object],
        MemoryPlanningPolicy::new(),
    )
    .expect("the planner should preserve representable u128 size");

    assert!(plan.all_sizes_known());
    assert_eq!(plan.known_reserved_bytes(), u128::MAX);
}

#[test]
fn memory_planning_reports_an_explicit_slot_budget_separately() {
    let first = memory_object(
        "buffer.a",
        MemorySize::Exact(8),
        "analysis.1",
    );

    let second = memory_object(
        "buffer.b",
        MemorySize::Exact(16),
        "analysis.1",
    );

    let plan = plan_memory(
        "analysis.1",
        [first, second],
        MemoryPlanningPolicy::new()
            .with_slot_reuse(false)
            .with_max_storage_slots(1),
    )
    .expect("planning should return a plan even when its slot budget is exceeded");

    assert_eq!(plan.slots().len(), 2);
    // The current planner's assessment concerns its statically known byte
    // budget; slot-count policy must not be misreported as a byte assessment.
}

// ---------------------------------------------------------------------------
// Integration boundary smoke tests
// ---------------------------------------------------------------------------

#[test]
fn optimization_modules_keep_distinct_responsibilities() {
    // This is a compile-time API contract: pass orchestration consumes an
    // opaque OptimizationTarget, while legality and cost selection operate
    // on supplied evidence and estimates. Neither test helper defines an IR.
    let target: &dyn OptimizationTarget = &TestTarget::valid();

    assert!(target.verify().is_ok());

    let pass_id = PassId::new("integration.boundary").expect("valid pass id");
    let contract = TransformationContract::new(pass_id);
    let report = assess_legality(&contract, &LegalityEvidence::new());

    assert_eq!(report.status(), LegalityStatus::Unproven);
}
