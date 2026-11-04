
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani AI Integration Tests
//!
//! Path: src/ai/tests/integration.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Verify the integration contracts between the existing AI domain
//! components and the execution boundary.
//!
//! These tests exercise:
//!
//! - AI context construction;
//! - exact, versioned AI operation registration;
//! - canonical AI operation names and classifications;
//! - AI executable workload contracts;
//! - requirement completeness enforcement;
//! - execution eligibility assessment;
//! - explicit infeasibility handling;
//! - indeterminate capability handling;
//! - cancellation before execution;
//! - actual dispatcher invocation;
//! - execution result and provenance propagation;
//! - canonical frontend AI semantic validation.
//!
//! ## Architectural authorities
//!
//! AI context:
//!     crate::ai::context
//!
//! AI configuration:
//!     crate::ai::config
//!
//! AI errors:
//!     crate::ai::error
//!
//! AI operation registry:
//!     crate::ai::registry
//!
//! AI requirements:
//!     crate::ai::requirements
//!
//! AI execution:
//!     crate::ai::execution
//!
//! Canonical AI semantic model:
//!     crate::frontend::semantic::domains::ai
//!
//! Execution control:
//!     crate::distributed::execution::executor
//!
//! AI source grammar:
//!     grammar/ai/ai.g4
//!
//! AI grammar architecture:
//!     grammar/ai/README.md
//!
//! ## Ownership
//!
//! This file owns test fixtures and integration assertions only.
//!
//! It does not implement:
//!
//! - another AI semantic model;
//! - another AI operation registry;
//! - another tensor representation;
//! - another intermediate representation;
//! - another execution engine;
//! - hardware discovery;
//! - physical resource allocation;
//! - device placement;
//! - scheduling;
//! - distributed transport;
//! - quantum execution;
//! - hybrid execution.
//!
//! ## POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! AI computation descriptions must remain independent of the physical
//! target that eventually realizes them.
//!
//! A target must establish its own capability and resource feasibility.
//! An unsupported or infeasible workload must not be silently accepted.
//!
//! Test workloads are finite samples, not language-level capacity limits.
//! No universal model, tensor, device, worker, accelerator or memory
//! ceiling is established by this file.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## Important integration boundary
//!
//! The test dispatcher below is deliberately a test fixture.
//! It verifies the AI execution protocol without claiming that a real
//! CPU, GPU, QPU, FPGA, accelerator, hybrid runtime or distributed
//! provider has executed the workload.
//!
//! Full source-to-AST-to-semantic-to-ZUIR-to-runtime execution requires
//! the owning frontend, ZUIR, compiler, hybrid, distributed and runtime
//! implementations to be completed and tested independently.

use std::sync::atomic::{
    AtomicUsize,
    Ordering,
};
use std::sync::Arc;

use crate::ai::context::{
    AiContext,
    AiContextBuilder,
};

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiResult,
};

use crate::ai::execution::{
    AiExecutable,
    AiExecutionAssessment,
    AiExecutionDispatcher,
    AiExecutionEngine,
    AiExecutionId,
    AiExecutionProvenance,
    AiExecutionReceipt,
    AiExecutionRequest,
};

use crate::ai::registry::{
    AiOperationDescriptor,
    AiOperationRegistry,
};

use crate::ai::requirements::AiRequirements;

use crate::ai::version::AiOperationSetVersion;

use crate::distributed::execution::executor::{
    CancellationSignal,
    ExecutionControl,
};

use crate::frontend::semantic::domains::ai::{
    AiOperationKind,
    QualifiedName,
};

// ============================================================================
// TEST IDENTITIES
// ============================================================================

fn inference_name() -> QualifiedName {
    QualifiedName::new(
        "zamani.ai",
        "inference",
    )
    .expect("the integration operation identity must be valid")
}

fn inference_version() -> AiOperationSetVersion {
    AiOperationSetVersion::new(1, 0, 0)
}

fn inference_descriptor() -> AiOperationDescriptor {
    AiOperationDescriptor::new(
        inference_name(),
        AiOperationKind::Inference,
        inference_version(),
    )
    .expect("the inference operation declaration must be valid")
}

// ============================================================================
// TEST CONTEXT
// ============================================================================

/// Constructs an AI context with an explicitly registered inference
/// operation.
///
/// The production context receives an immutable snapshot of the
/// prepared operation registry.
///
/// No operation implementation or hardware provider is registered here.
fn inference_context() -> AiContext<'static> {
    let mut registry = AiOperationRegistry::new();

    registry
        .register(inference_descriptor())
        .expect("the inference operation must register successfully");

    AiContextBuilder::new()
        .registry(registry)
        .build()
        .expect("the AI context configuration must be valid")
}

// ============================================================================
// TEST WORKLOAD
// ============================================================================

/// Test-only workload implementing the existing AI execution contract.
///
/// This is not a second production model or IR.
///
/// Its identity and operation metadata refer to the canonical
/// frontend AI semantic name and AI operation-set version.
struct TestInferenceWorkload {
    operation: QualifiedName,
    version: AiOperationSetVersion,
    requirements: AiRequirements,
    valid: bool,
}

impl TestInferenceWorkload {
    /// Creates a valid test inference workload.
    fn new() -> Self {
        Self {
            operation: inference_name(),
            version: inference_version(),
            requirements: AiRequirements::new(),
            valid: true,
        }
    }

    /// Creates a workload whose requirement analysis is incomplete.
    fn with_incomplete_requirements() -> Self {
        let mut requirements = AiRequirements::new();

        requirements
            .mark_incomplete(
                "test requirement source has not been analyzed",
            )
            .expect("the incomplete-analysis reason must be valid");

        Self {
            operation: inference_name(),
            version: inference_version(),
            requirements,
            valid: true,
        }
    }

    /// Creates a workload that fails its own validation.
    fn invalid() -> Self {
        Self {
            operation: inference_name(),
            version: inference_version(),
            requirements: AiRequirements::new(),
            valid: false,
        }
    }
}

impl AiExecutable for TestInferenceWorkload {
    fn operation_name(&self) -> &QualifiedName {
        &self.operation
    }

    fn operation_version(&self) -> AiOperationSetVersion {
        self.version
    }

    fn requirements(&self) -> &AiRequirements {
        &self.requirements
    }

    fn model_reference(&self) -> Option<&str> {
        Some("test.model.inference")
    }

    fn validate(&self) -> AiResult<()> {
        if self.valid {
            Ok(())
        } else {
            Err(
                AiError::new(
                    AiErrorKind::InvalidInput,
                    "test workload validation failed",
                ),
            )
        }
    }
}

// ============================================================================
// TEST DISPATCHER
// ============================================================================

/// Controls the result of the test dispatcher's feasibility assessment.
#[derive(Clone, Debug)]
enum TestAssessment {
    Eligible,
    Ineligible,
    Indeterminate,
}

/// A test-only dispatcher for verifying the AI execution protocol.
///
/// The dispatcher does not claim to execute physical hardware.
/// It returns a deterministic test result after the execution engine
/// has completed its validation and eligibility checks.
struct TestDispatcher {
    assessment: TestAssessment,
    assessment_calls: Arc<AtomicUsize>,
    execution_calls: Arc<AtomicUsize>,
}

impl TestDispatcher {
    /// Constructs a dispatcher with the requested assessment behavior.
    fn new(assessment: TestAssessment) -> Self {
        Self {
            assessment,
            assessment_calls: Arc::new(AtomicUsize::new(0)),
            execution_calls: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Returns the number of assessment calls.
    fn assessment_calls(&self) -> usize {
        self.assessment_calls.load(Ordering::SeqCst)
    }

    /// Returns the number of execution calls.
    fn execution_calls(&self) -> usize {
        self.execution_calls.load(Ordering::SeqCst)
    }
}

impl AiExecutionDispatcher<TestInferenceWorkload> for TestDispatcher {
    type Output = String;

    fn assess(
        &self,
        workload: &TestInferenceWorkload,
        operation: &AiOperationDescriptor,
    ) -> AiResult<AiExecutionAssessment> {
        self.assessment_calls
            .fetch_add(1, Ordering::SeqCst);

        if operation.name() != &workload.operation {
            return Err(
                AiError::new(
                    AiErrorKind::InvalidOperation,
                    "resolved operation does not match the workload",
                ),
            );
        }

        if operation.version() != workload.version {
            return Err(
                AiError::new(
                    AiErrorKind::InvalidOperation,
                    "resolved operation version does not match the workload",
                ),
            );
        }

        match self.assessment {
            TestAssessment::Eligible => {
                Ok(AiExecutionAssessment::eligible(
                    "test dispatcher established eligibility",
                ))
            }

            TestAssessment::Ineligible => {
                Ok(AiExecutionAssessment::ineligible(
                    "test target cannot satisfy the required computation",
                ))
            }

            TestAssessment::Indeterminate => {
                Ok(AiExecutionAssessment::indeterminate(
                    "test target has insufficient capability evidence",
                ))
            }
        }
    }

    fn execute(
        &self,
        _workload: &TestInferenceWorkload,
        operation: &AiOperationDescriptor,
        control: &ExecutionControl,
    ) -> AiResult<AiExecutionReceipt<Self::Output>> {
        control.check().map_err(|error| {
            AiError::from_source(
                AiErrorKind::Cancelled,
                "test execution was cancelled",
                error,
            )
        })?;

        self.execution_calls
            .fetch_add(1, Ordering::SeqCst);

        let output = format!(
            "test-result:{}:{}",
            operation.name().namespace,
            operation.name().name,
        );

        let provenance = AiExecutionProvenance {
            computation_reference: Some(
                "test.computation".to_owned(),
            ),
            execution_reference: Some(
                "test.execution".to_owned(),
            ),
            model_reference: Some(
                "test.model.inference".to_owned(),
            ),
            data_reference: Some(
                "test.dataset".to_owned(),
            ),
        };

        Ok(
            AiExecutionReceipt::new(
                output,
                provenance,
            )
            .with_realization_reference(
                "test.realization",
            ),
        )
    }
}

// ============================================================================
// CANCELLATION FIXTURE
// ============================================================================

/// A cancellation signal that always reports cancellation.
///
/// This is test-only and does not depend on operating-system facilities.
struct AlwaysCancelled;

impl CancellationSignal for AlwaysCancelled {
    fn is_cancelled(&self) -> bool {
        true
    }
}

// ============================================================================
// SUCCESSFUL EXECUTION
// ============================================================================

/// A valid workload must pass through exact operation resolution,
/// eligibility assessment and the injected execution dispatcher.
#[test]
fn valid_inference_workload_reaches_execution_dispatcher() {
    let context = inference_context();
    let workload = TestInferenceWorkload::new();

    let dispatcher = TestDispatcher::new(
        TestAssessment::Eligible,
    );

    let request = AiExecutionRequest::new(
        AiExecutionId::new("integration.inference.001")
            .expect("valid execution identity"),
        &workload,
    );

    let result = AiExecutionEngine::execute(
        &context,
        &request,
        &dispatcher,
        &ExecutionControl::new(),
    )
    .expect("eligible inference execution must succeed");

    assert_eq!(
        result.id().as_str(),
        "integration.inference.001",
    );

    assert_eq!(
        result.operation(),
        "zamani.ai::inference",
    );

    assert_eq!(
        result.operation_version(),
        inference_version(),
    );

    assert_eq!(
        result.output(),
        "test-result:zamani.ai:inference",
    );

    assert_eq!(
        result.realization_reference(),
        Some("test.realization"),
    );

    assert!(
        result.provenance().is_recorded(),
        "the actual execution reference must be preserved",
    );

    assert_eq!(dispatcher.assessment_calls(), 1);
    assert_eq!(dispatcher.execution_calls(), 1);
}

// ============================================================================
// FEASIBILITY ENFORCEMENT
// ============================================================================

/// A workload that is explicitly infeasible must not be dispatched.
///
/// Infeasibility is a target-realization result, not a change to the
/// source program's computational meaning.
#[test]
fn infeasible_workload_is_rejected_before_execution() {
    let context = inference_context();
    let workload = TestInferenceWorkload::new();

    let dispatcher = TestDispatcher::new(
        TestAssessment::Ineligible,
    );

    let request = AiExecutionRequest::new(
        AiExecutionId::new("integration.inference.infeasible")
            .expect("valid execution identity"),
        &workload,
    );

    let result = AiExecutionEngine::execute(
        &context,
        &request,
        &dispatcher,
        &ExecutionControl::new(),
    );

    assert!(
        result.is_err(),
        "an infeasible workload must not produce a successful result",
    );

    assert_eq!(dispatcher.assessment_calls(), 1);
    assert_eq!(
        dispatcher.execution_calls(),
        0,
        "infeasible workloads must never reach execution",
    );
}

/// An indeterminate capability assessment must not be treated as
/// successful eligibility.
#[test]
fn indeterminate_workload_is_not_dispatched() {
    let context = inference_context();
    let workload = TestInferenceWorkload::new();

    let dispatcher = TestDispatcher::new(
        TestAssessment::Indeterminate,
    );

    let request = AiExecutionRequest::new(
        AiExecutionId::new("integration.inference.indeterminate")
            .expect("valid execution identity"),
        &workload,
    );

    let result = AiExecutionEngine::execute(
        &context,
        &request,
        &dispatcher,
        &ExecutionControl::new(),
    );

    assert!(
        result.is_err(),
        "unknown feasibility must not be promoted to success",
    );

    assert_eq!(dispatcher.assessment_calls(), 1);
    assert_eq!(dispatcher.execution_calls(), 0);
}

// ============================================================================
// REQUIREMENT COMPLETENESS
// ============================================================================

/// Incomplete requirement analysis must prevent execution.
///
/// The execution boundary must not treat missing requirement evidence
/// as proof that the workload is safe or feasible.
#[test]
fn incomplete_requirements_prevent_execution() {
    let context = inference_context();

    let workload =
        TestInferenceWorkload::with_incomplete_requirements();

    let dispatcher = TestDispatcher::new(
        TestAssessment::Eligible,
    );

    let request = AiExecutionRequest::new(
        AiExecutionId::new("integration.inference.incomplete")
            .expect("valid execution identity"),
        &workload,
    );

    let result = AiExecutionEngine::execute(
        &context,
        &request,
        &dispatcher,
        &ExecutionControl::new(),
    );

    assert!(
        result.is_err(),
        "incomplete requirement analysis must prevent execution",
    );

    assert_eq!(
        dispatcher.assessment_calls(),
        0,
        "feasibility must not be assessed using incomplete requirements",
    );

    assert_eq!(
        dispatcher.execution_calls(),
        0,
        "incomplete requirements must never reach execution",
    );
}

// ============================================================================
// WORKLOAD VALIDATION
// ============================================================================

/// Invalid workloads must be rejected before operation resolution
/// or provider assessment.
#[test]
fn invalid_workload_is_rejected_before_assessment() {
    let context = inference_context();
    let workload = TestInferenceWorkload::invalid();

    let dispatcher = TestDispatcher::new(
        TestAssessment::Eligible,
    );

    let request = AiExecutionRequest::new(
        AiExecutionId::new("integration.inference.invalid")
            .expect("valid execution identity"),
        &workload,
    );

    let result = AiExecutionEngine::execute(
        &context,
        &request,
        &dispatcher,
        &ExecutionControl::new(),
    );

    assert!(result.is_err());

    assert_eq!(dispatcher.assessment_calls(), 0);
    assert_eq!(dispatcher.execution_calls(), 0);
}

// ============================================================================
// EXACT OPERATION VERSIONING
// ============================================================================

/// The execution engine must resolve the exact operation version.
///
/// It must not silently substitute another registered version.
#[test]
fn unknown_operation_version_is_rejected() {
    let context = inference_context();

    let mut workload = TestInferenceWorkload::new();

    workload.version = AiOperationSetVersion::new(
        2,
        0,
        0,
    );

    let dispatcher = TestDispatcher::new(
        TestAssessment::Eligible,
    );

    let request = AiExecutionRequest::new(
        AiExecutionId::new("integration.inference.unknown-version")
            .expect("valid execution identity"),
        &workload,
    );

    let result = AiExecutionEngine::execute(
        &context,
        &request,
        &dispatcher,
        &ExecutionControl::new(),
    );

    assert!(
        result.is_err(),
        "an unregistered exact operation version must be rejected",
    );

    assert_eq!(dispatcher.assessment_calls(), 0);
    assert_eq!(dispatcher.execution_calls(), 0);
}

// ============================================================================
// CANCELLATION
// ============================================================================

/// Cancellation before execution must prevent assessment and dispatch.
#[test]
fn cancelled_request_never_reaches_assessment_or_execution() {
    let context = inference_context();
    let workload = TestInferenceWorkload::new();

    let dispatcher = TestDispatcher::new(
        TestAssessment::Eligible,
    );

    let request = AiExecutionRequest::new(
        AiExecutionId::new("integration.inference.cancelled")
            .expect("valid execution identity"),
        &workload,
    );

    let control = ExecutionControl::new()
        .with_cancellation(
            Arc::new(AlwaysCancelled),
        );

    let result = AiExecutionEngine::execute(
        &context,
        &request,
        &dispatcher,
        &control,
    );

    assert!(
        result.is_err(),
        "a cancelled request must not succeed",
    );

    assert_eq!(dispatcher.assessment_calls(), 0);
    assert_eq!(dispatcher.execution_calls(), 0);
}

// ============================================================================
// CANONICAL CONTEXT AND REGISTRY INTEGRATION
// ============================================================================

/// The AI context must preserve its explicit operation registry.
///
/// Registration is descriptive metadata; it does not itself imply
/// that an execution provider exists.
#[test]
fn context_exposes_exact_registered_operation() {
    let context = inference_context();

    assert_eq!(
        context.registered_operation_count(),
        1,
    );

    let operation = context
        .registry()
        .resolve(
            &inference_name(),
            inference_version(),
        )
        .expect("the exact operation must be registered");

    assert_eq!(
        operation.name(),
        &inference_name(),
    );

    assert_eq!(
        operation.kind(),
        &AiOperationKind::Inference,
    );

    assert_eq!(
        operation.version(),
        inference_version(),
    );
}

/// An independent context snapshot must preserve the registry
/// without sharing mutable registration state.
#[test]
fn context_clone_preserves_immutable_registry_snapshot() {
    let original = inference_context();
    let cloned = original.clone();

    assert_eq!(
        original.registered_operation_count(),
        cloned.registered_operation_count(),
    );

    assert!(
        cloned.registry().contains(
            &inference_name(),
            inference_version(),
        ),
    );
}

// ============================================================================
// TEST SCOPE
// ============================================================================
//
// The following complete production pathways are not certified by
// this file:
//
// - grammar/ai/ai.g4 -> canonical parser -> AST;
// - AST -> frontend semantic AI adapter;
// - semantic AI -> ZUIR AI dialect;
// - ZUIR -> optimization and lowering;
// - ZUIR -> classical or quantum IR;
// - AI -> hybrid graph and validated hybrid execution plan;
// - AI -> distributed planning and execution;
// - execution dispatcher -> actual runtime/backend provider.
//
// These require real implementations and integration tests owned by
// the corresponding repository subsystems.
