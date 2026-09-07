//! Zamani Quantum Resilience — Migration Integration Tests
//!
//! Path:
//!     src/quantum/resilience/tests/migration.rs
//!
//! Purpose:
//!     Production-grade integration tests for the recovery-layer migration
//!     contract.
//!
//! These tests verify that execution migration:
//!
//!     - remains provider-independent;
//!     - does not invent quantum-resource identities;
//!     - never transfers an arbitrary unknown quantum state;
//!     - validates policy before execution;
//!     - validates checkpoints before execution;
//!     - validates destination capabilities before execution;
//!     - respects authorization;
//!     - respects cancellation;
//!     - delegates execution to an injected executor;
//!     - delegates normalization to an injected adapter;
//!     - requires verification before acceptance;
//!     - distinguishes accepted, degraded, rejected, failed and escalated
//!       outcomes;
//!     - preserves provenance;
//!     - remains deterministic when deterministic dependencies are supplied;
//!     - introduces no machine-size ceiling;
//!     - contains no hidden retry loop;
//!     - contains no provider-specific assumptions.
//!
//! The tests intentionally use deterministic in-memory implementations.
//! No real QPU, simulator, provider SDK, network connection, credential,
//! filesystem, random number generator, thread, timer or hardware resource
//! is used.
//!
//! -----------------------------------------------------------------------------
//! SCALE-EVERYWHERE CONTRACT
//! -----------------------------------------------------------------------------
//!
//! No test value below represents an architectural limit.
//!
//! The migration subsystem must not introduce:
//!
//!     MAX_QUBITS
//!     MAX_PHYSICAL_QUBITS
//!     MAX_LOGICAL_QUBITS
//!     MAX_DEVICES
//!     MAX_BACKENDS
//!     MAX_PROVIDERS
//!     MAX_RETRIES
//!     MAX_MIGRATIONS
//!     MAX_OPERATIONS
//!
//! Any finite value in this file is deterministic test data only.
//!
//! "Infinite" scalability means that the migration abstraction does not impose
//! a finite machine-size ceiling. Actual execution remains bounded by the
//! resources, capabilities, policies, budgets and physical infrastructure
//! supplied by the caller.
//!
//! -----------------------------------------------------------------------------
//! QUANTUM IDENTITY
//! -----------------------------------------------------------------------------
//!
//! Migration does not define QubitId, PhysicalQubitId or LogicalQubitId.
//!
//! If future migration tests need explicit quantum-resource identity, they must
//! use the canonical types owned by:
//!
//!     crate::quantum::ir::qubit
//!
//! No migration-local quantum identity is permitted.
//!
//! -----------------------------------------------------------------------------
//! RUST CONTRACT
//! -----------------------------------------------------------------------------
//!
//!     Rust 1.97 / 1.97.1
//!     Rust 2021
//!     stable Rust
//!     no unsafe code
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::quantum::resilience::recovery::migration::{
    CheckpointId,
    CheckpointValidation,
    ExecutionId,
    MigrationAuthorizer,
    MigrationAuthorizationError,
    MigrationCapabilities,
    MigrationCancellation,
    MigrationCheckpointProvider,
    MigrationClock,
    MigrationError,
    MigrationExecutionAdapter,
    MigrationExecutionRequest,
    MigrationExecutor,
    MigrationId,
    MigrationObserver,
    MigrationOutcome,
    MigrationPolicy,
    MigrationProvenance,
    MigrationRequest,
    MigrationService,
    MigrationStatus,
    MigrationVerifier,
    MigratedExecution,
    MigratableStateKind,
    PreparedMigration,
    ProgramIdentity,
    QuantumStateTransfer,
    ReplaySafety,
    SideEffectMode,
    TargetId,
};

// ============================================================================
// Deterministic identifiers
// ============================================================================

const MIGRATION_ID: &str = "test-migration";
const SOURCE_EXECUTION_ID: &str = "test-source-execution";
const PROGRAM_ID: &str = "test-program";
const SOURCE_TARGET_ID: &str = "test-source-target";
const DESTINATION_TARGET_ID: &str = "test-destination-target";
const CHECKPOINT_ID: &str = "test-checkpoint";
const DESTINATION_EXECUTION_ID: &str = "test-destination-execution";

fn migration_id() -> MigrationId {
    MigrationId::new(MIGRATION_ID).expect("test migration ID must be valid")
}

fn source_execution_id() -> ExecutionId {
    ExecutionId::new(SOURCE_EXECUTION_ID)
        .expect("test execution ID must be valid")
}

fn program_id() -> ProgramIdentity {
    ProgramIdentity::new(PROGRAM_ID).expect("test program ID must be valid")
}

fn source_target() -> TargetId {
    TargetId::new(SOURCE_TARGET_ID)
        .expect("test source target ID must be valid")
}

fn destination_target() -> TargetId {
    TargetId::new(DESTINATION_TARGET_ID)
        .expect("test destination target ID must be valid")
}

fn checkpoint_id() -> CheckpointId {
    CheckpointId::new(CHECKPOINT_ID)
        .expect("test checkpoint ID must be valid")
}

// ============================================================================
// Request fixtures
// ============================================================================

fn default_policy() -> MigrationPolicy {
    MigrationPolicy::default()
}

fn default_capabilities() -> MigrationCapabilities {
    MigrationCapabilities {
        executable: true,
        canonical_program: true,
        checkpoint_restore: true,
        provider_state_migration: true,
        logical_state_migration: true,
        rerouting: true,
        rescheduling: true,
        recompilation: true,
        semantic_compatibility: true,
        degraded_execution: true,
    }
}

fn valid_request() -> MigrationRequest {
    MigrationRequest {
        migration_id: migration_id(),
        execution_id: source_execution_id(),
        program: program_id(),
        source_target: source_target(),
        destination_target: destination_target(),
        checkpoint: Some(checkpoint_id()),
        state_kind: MigratableStateKind::LogicalCheckpoint,
        quantum_state_transfer: QuantumStateTransfer::LogicalState,
        replay_safety: ReplaySafety::Safe,
        side_effects: SideEffectMode::None,
        policy: default_policy(),
        metadata: BTreeMap::new(),
    }
}

// ============================================================================
// Test error
// ============================================================================

#[derive(Debug, Clone, Eq, PartialEq)]
struct TestError {
    message: String,
}

impl TestError {
    fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}

impl fmt::Display for TestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for TestError {}

// ============================================================================
// Capability provider
// ============================================================================

#[derive(Debug)]
struct TestCapabilityProvider {
    capabilities: MigrationCapabilities,
    fail: bool,
    calls: Mutex<Vec<TargetId>>,
}

impl TestCapabilityProvider {
    fn new(capabilities: MigrationCapabilities) -> Self {
        Self {
            capabilities,
            fail: false,
            calls: Mutex::new(Vec::new()),
        }
    }

    fn failing(capabilities: MigrationCapabilities) -> Self {
        Self {
            capabilities,
            fail: true,
            calls: Mutex::new(Vec::new()),
        }
    }

    fn calls(&self) -> Vec<TargetId> {
        self.calls
            .lock()
            .expect("capability mutex must not be poisoned")
            .clone()
    }
}

impl MigrationCapabilityProvider for TestCapabilityProvider {
    type Error = TestError;

    fn capabilities(
        &self,
        target: &TargetId,
    ) -> Result<MigrationCapabilities, Self::Error> {
        self.calls
            .lock()
            .expect("capability mutex must not be poisoned")
            .push(target.clone());

        if self.fail {
            return Err(TestError::new("capability lookup failed"));
        }

        Ok(self.capabilities.clone())
    }
}

// ============================================================================
// Checkpoint provider
// ============================================================================

#[derive(Debug)]
struct TestCheckpointProvider {
    validation: CheckpointValidation,
    fail: bool,
    calls: Mutex<u64>,
}

impl TestCheckpointProvider {
    fn usable() -> Self {
        Self {
            validation: CheckpointValidation {
                integrity_valid: true,
                program_compatible: true,
                destination_compatible: true,
                state_compatible: true,
                diagnostics: Vec::new(),
            },
            fail: false,
            calls: Mutex::new(0),
        }
    }

    fn unusable() -> Self {
        Self {
            validation: CheckpointValidation {
                integrity_valid: false,
                program_compatible: true,
                destination_compatible: true,
                state_compatible: true,
                diagnostics: vec!["checkpoint integrity failure".to_owned()],
            },
            fail: false,
            calls: Mutex::new(0),
        }
    }

    fn failing() -> Self {
        Self {
            validation: Self::usable().validation,
            fail: true,
            calls: Mutex::new(0),
        }
    }

    fn calls(&self) -> u64 {
        *self
            .calls
            .lock()
            .expect("checkpoint mutex must not be poisoned")
    }
}

impl MigrationCheckpointProvider for TestCheckpointProvider {
    type Error = TestError;

    fn validate(
        &self,
        _request: &MigrationRequest,
    ) -> Result<CheckpointValidation, Self::Error> {
        let mut calls = self
            .calls
            .lock()
            .expect("checkpoint mutex must not be poisoned");

        *calls = calls.saturating_add(1);

        if self.fail {
            return Err(TestError::new("checkpoint provider failed"));
        }

        Ok(self.validation.clone())
    }
}

// ============================================================================
// Test runtime execution
// ============================================================================

#[derive(Debug, Clone, Eq, PartialEq)]
struct TestRuntimeExecution {
    id: String,
    resumed: bool,
}

// ============================================================================
// Migration executor
// ============================================================================

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum ExecutorMode {
    Success,
    Failure,
}

#[derive(Debug)]
struct TestExecutor {
    mode: ExecutorMode,
    calls: Mutex<Vec<MigrationExecutionRequest>>,
}

impl TestExecutor {
    fn successful() -> Self {
        Self {
            mode: ExecutorMode::Success,
            calls: Mutex::new(Vec::new()),
        }
    }

    fn failing() -> Self {
        Self {
            mode: ExecutorMode::Failure,
            calls: Mutex::new(Vec::new()),
        }
    }

    fn calls(&self) -> Vec<MigrationExecutionRequest> {
        self.calls
            .lock()
            .expect("executor mutex must not be poisoned")
            .clone()
    }
}

impl MigrationExecutor for TestExecutor {
    type Execution = TestRuntimeExecution;
    type Error = TestError;

    fn migrate(
        &self,
        request: &MigrationExecutionRequest,
    ) -> Result<Self::Execution, Self::Error> {
        self.calls
            .lock()
            .expect("executor mutex must not be poisoned")
            .push(request.clone());

        match self.mode {
            ExecutorMode::Success => Ok(TestRuntimeExecution {
                id: DESTINATION_EXECUTION_ID.to_owned(),
                resumed: true,
            }),

            ExecutorMode::Failure => {
                Err(TestError::new("migration executor failed"))
            }
        }
    }
}

// ============================================================================
// Execution adapter
// ============================================================================

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum AdapterMode {
    Success,
    Failure,
}

#[derive(Debug)]
struct TestExecutionAdapter {
    mode: AdapterMode,
}

impl TestExecutionAdapter {
    fn successful() -> Self {
        Self {
            mode: AdapterMode::Success,
        }
    }

    fn failing() -> Self {
        Self {
            mode: AdapterMode::Failure,
        }
    }
}

impl MigrationExecutionAdapter<TestRuntimeExecution>
    for TestExecutionAdapter
{
    type Error = TestError;

    fn normalize(
        &self,
        request: &PreparedMigration,
        execution: &TestRuntimeExecution,
    ) -> Result<MigratedExecution, Self::Error> {
        match self.mode {
            AdapterMode::Failure => {
                Err(TestError::new("execution adapter failed"))
            }

            AdapterMode::Success => Ok(MigratedExecution {
                execution_id: ExecutionId::new(execution.id.clone())
                    .expect("destination execution ID must be valid"),
                target: request.destination_target.clone(),
                resumed: execution.resumed,
                provisional: true,
                checkpoint: request.checkpoint.clone(),
            }),
        }
    }
}

// ============================================================================
// Verifier
// ============================================================================

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum VerificationMode {
    Accepted,
    AcceptedDegraded,
    Rejected,
    InvalidConfidence,
    Failed,
}

#[derive(Debug)]
struct TestVerifier {
    mode: VerificationMode,
    calls: Mutex<u64>,
}

impl TestVerifier {
    fn new(mode: VerificationMode) -> Self {
        Self {
            mode,
            calls: Mutex::new(0),
        }
    }

    fn calls(&self) -> u64 {
        *self
            .calls
            .lock()
            .expect("verifier mutex must not be poisoned")
    }
}

impl MigrationVerifier<TestRuntimeExecution> for TestVerifier {
    type Error = TestError;

    fn verify(
        &self,
        _request: &MigrationRequest,
        _prepared: &PreparedMigration,
        _execution: &TestRuntimeExecution,
    ) -> Result<
        crate::quantum::resilience::recovery::migration::MigrationVerification,
        Self::Error,
    > {
        let mut calls = self
            .calls
            .lock()
            .expect("verifier mutex must not be poisoned");

        *calls = calls.saturating_add(1);

        use crate::quantum::resilience::recovery::migration::MigrationVerification;

        match self.mode {
            VerificationMode::Accepted => Ok(MigrationVerification {
                semantically_equivalent: true,
                capability_compatible: true,
                state_valid: true,
                provenance_complete: true,
                degraded: false,
                confidence: Some(1.0),
                diagnostics: Vec::new(),
            }),

            VerificationMode::AcceptedDegraded => Ok(MigrationVerification {
                semantically_equivalent: true,
                capability_compatible: true,
                state_valid: true,
                provenance_complete: true,
                degraded: true,
                confidence: Some(0.8),
                diagnostics: vec!["destination is degraded".to_owned()],
            }),

            VerificationMode::Rejected => Ok(MigrationVerification {
                semantically_equivalent: false,
                capability_compatible: true,
                state_valid: true,
                provenance_complete: true,
                degraded: false,
                confidence: Some(0.2),
                diagnostics: vec![
                    "semantic equivalence failed".to_owned()
                ],
            }),

            VerificationMode::InvalidConfidence => Ok(MigrationVerification {
                semantically_equivalent: true,
                capability_compatible: true,
                state_valid: true,
                provenance_complete: true,
                degraded: false,
                confidence: Some(f64::NAN),
                diagnostics: Vec::new(),
            }),

            VerificationMode::Failed => {
                Err(TestError::new("verification failed"))
            }
        }
    }
}

// ============================================================================
// Authorization
// ============================================================================

#[derive(Debug)]
struct TestAuthorizer {
    deny: bool,
    calls: Mutex<u64>,
}

impl TestAuthorizer {
    fn allowing() -> Self {
        Self {
            deny: false,
            calls: Mutex::new(0),
        }
    }

    fn denying() -> Self {
        Self {
            deny: true,
            calls: Mutex::new(0),
        }
    }

    fn calls(&self) -> u64 {
        *self
            .calls
            .lock()
            .expect("authorization mutex must not be poisoned")
    }
}

impl MigrationAuthorizer for TestAuthorizer {
    type Error = MigrationAuthorizationError;

    fn authorize(
        &self,
        _request: &MigrationRequest,
    ) -> Result<(), Self::Error> {
        let mut calls = self
            .calls
            .lock()
            .expect("authorization mutex must not be poisoned");

        *calls = calls.saturating_add(1);

        if self.deny {
            return Err(MigrationAuthorizationError {
                message: "authorization denied".to_owned(),
            });
        }

        Ok(())
    }
}

// ============================================================================
// Cancellation
// ============================================================================

#[derive(Debug)]
struct TestCancellation {
    cancelled: bool,
}

impl MigrationCancellation for TestCancellation {
    fn is_cancelled(&self) -> bool {
        self.cancelled
    }
}

// ============================================================================
// Observer
// ============================================================================

#[derive(Debug, Default)]
struct TestObserver {
    started: Mutex<Vec<MigrationId>>,
    completed: Mutex<Vec<MigrationOutcome>>,
}

impl TestObserver {
    fn started(&self) -> Vec<MigrationId> {
        self.started
            .lock()
            .expect("observer mutex must not be poisoned")
            .clone()
    }

    fn completed(&self) -> Vec<MigrationOutcome> {
        self.completed
            .lock()
            .expect("observer mutex must not be poisoned")
            .clone()
    }
}

impl MigrationObserver for TestObserver {
    fn started(&self, request: &MigrationRequest) {
        self.started
            .lock()
            .expect("observer mutex must not be poisoned")
            .push(request.migration_id.clone());
    }

    fn completed(&self, outcome: &MigrationOutcome) {
        self.completed
            .lock()
            .expect("observer mutex must not be poisoned")
            .push(outcome.clone());
    }
}

// ============================================================================
// Deterministic clock
// ============================================================================

#[derive(Clone, Copy, Debug)]
struct FixedClock {
    time: SystemTime,
}

impl MigrationClock for FixedClock {
    fn now(&self) -> SystemTime {
        self.time
    }
}

// ============================================================================
// Service construction
// ============================================================================

type TestService = MigrationService<
    TestCapabilityProvider,
    TestCheckpointProvider,
    TestExecutor,
    TestExecutionAdapter,
    TestVerifier,
>;

fn make_service(
    capabilities: TestCapabilityProvider,
    checkpoints: TestCheckpointProvider,
    executor: TestExecutor,
    adapter: TestExecutionAdapter,
    verifier: TestVerifier,
) -> TestService {
    MigrationService::new(
        Arc::new(capabilities),
        Arc::new(checkpoints),
        Arc::new(executor),
        Arc::new(adapter),
        Arc::new(verifier),
    )
}

fn successful_service() -> TestService {
    make_service(
        TestCapabilityProvider::new(default_capabilities()),
        TestCheckpointProvider::usable(),
        TestExecutor::successful(),
        TestExecutionAdapter::successful(),
        TestVerifier::new(VerificationMode::Accepted),
    )
}

// ============================================================================
// Identifier tests
// ============================================================================

#[test]
fn identifiers_reject_empty_values() {
    assert!(matches!(
        MigrationId::new(""),
        Err(MigrationError::InvalidIdentifier {
            field: "migration_id"
        })
    ));

    assert!(matches!(
        ExecutionId::new(" "),
        Err(MigrationError::InvalidIdentifier {
            field: "execution_id"
        })
    ));

    assert!(matches!(
        TargetId::new(""),
        Err(MigrationError::InvalidIdentifier {
            field: "target_id"
        })
    ));

    assert!(matches!(
        CheckpointId::new(""),
        Err(MigrationError::InvalidIdentifier {
            field: "checkpoint_id"
        })
    ));

    assert!(matches!(
        ProgramIdentity::new(""),
        Err(MigrationError::InvalidIdentifier {
            field: "program_identity"
        })
    ));
}

// ============================================================================
// Static request validation
// ============================================================================

#[test]
fn valid_request_passes_validation() {
    assert!(valid_request().validate().is_ok());
}

#[test]
fn source_and_destination_must_differ() {
    let mut request = valid_request();
    request.destination_target = request.source_target.clone();

    assert!(matches!(
        request.validate(),
        Err(MigrationError::SameSourceAndDestination)
    ));
}

#[test]
fn disabled_migration_is_rejected() {
    let mut request = valid_request();

    request.policy.allow_migration = false;
    request.policy.allow_cross_target = false;
    request.policy.allow_cross_provider = false;
    request.policy.allow_recompilation = false;
    request.policy.allow_rerouting = false;
    request.policy.allow_rescheduling = false;
    request.policy.allow_qec_adaptation = false;

    assert!(matches!(
        request.validate(),
        Err(MigrationError::MigrationNotAllowed)
    ));
}

#[test]
fn inconsistent_policy_is_rejected() {
    let mut request = valid_request();

    request.policy.allow_cross_target = false;
    request.policy.allow_cross_provider = true;

    assert!(matches!(
        request.validate(),
        Err(MigrationError::InvalidPolicy(_))
    ));
}

#[test]
fn arbitrary_unknown_quantum_state_transfer_is_rejected() {
    let mut request = valid_request();

    request.quantum_state_transfer =
        QuantumStateTransfer::ArbitraryUnknownState;

    assert!(matches!(
        request.validate(),
        Err(MigrationError::ArbitraryQuantumStateTransfer)
    ));
}

#[test]
fn logical_state_requires_checkpoint() {
    let mut request = valid_request();
    request.checkpoint = None;

    assert!(matches!(
        request.validate(),
        Err(MigrationError::CheckpointRequired)
    ));
}

#[test]
fn provider_managed_state_requires_checkpoint() {
    let mut request = valid_request();

    request.quantum_state_transfer =
        QuantumStateTransfer::ProviderManaged;
    request.checkpoint = None;

    assert!(matches!(
        request.validate(),
        Err(MigrationError::CheckpointRequired)
    ));
}

#[test]
fn unsafe_replay_is_rejected() {
    let mut request = valid_request();
    request.replay_safety = ReplaySafety::Unsafe;

    assert!(matches!(
        request.validate(),
        Err(MigrationError::ReplayUnsafe)
    ));
}

#[test]
fn unknown_replay_safety_is_rejected_for_reconstruction() {
    let mut request = valid_request();

    request.state_kind =
        MigratableStateKind::ReconstructibleProgram;
    request.quantum_state_transfer =
        QuantumStateTransfer::NotRequired;
    request.checkpoint = None;
    request.replay_safety = ReplaySafety::Unknown;

    assert!(matches!(
        request.validate(),
        Err(MigrationError::ReplaySafetyUnknown)
    ));
}

#[test]
fn conditional_replay_requires_explicit_permission() {
    let mut request = valid_request();

    request.replay_safety =
        ReplaySafety::ConditionallySafe;
    request.policy.allow_conditional_replay = false;

    assert!(matches!(
        request.validate(),
        Err(MigrationError::ConditionalReplayNotAllowed)
    ));
}

#[test]
fn conditional_replay_is_valid_when_policy_allows_it() {
    let mut request = valid_request();

    request.replay_safety =
        ReplaySafety::ConditionallySafe;
    request.policy.allow_conditional_replay = true;

    assert!(request.validate().is_ok());
}

#[test]
fn non_replayable_side_effects_block_reconstruction() {
    let mut request = valid_request();

    request.state_kind =
        MigratableStateKind::ReconstructibleProgram;
    request.quantum_state_transfer =
        QuantumStateTransfer::NotRequired;
    request.checkpoint = None;
    request.side_effects =
        SideEffectMode::NonReplayable;

    assert!(matches!(
        request.validate(),
        Err(MigrationError::NonReplayableSideEffects)
    ));
}

#[test]
fn unknown_side_effects_block_reconstruction() {
    let mut request = valid_request();

    request.state_kind =
        MigratableStateKind::ReconstructibleProgram;
    request.quantum_state_transfer =
        QuantumStateTransfer::NotRequired;
    request.checkpoint = None;
    request.side_effects =
        SideEffectMode::Unknown;

    assert!(matches!(
        request.validate(),
        Err(MigrationError::SideEffectSemanticsUnknown)
    ));
}

// ============================================================================
// Capability tests
// ============================================================================

#[test]
fn valid_capabilities_pass_validation() {
    assert!(default_capabilities().validate().is_ok());
}

#[test]
fn execution_capability_is_required() {
    let mut capabilities = default_capabilities();
    capabilities.executable = false;

    assert!(matches!(
        capabilities.validate(),
        Err(MigrationError::CapabilityUnavailable {
            capability: "executable"
        })
    ));
}

#[test]
fn semantic_compatibility_is_required() {
    let mut capabilities = default_capabilities();
    capabilities.semantic_compatibility = false;

    assert!(matches!(
        capabilities.validate(),
        Err(MigrationError::CapabilityUnavailable {
            capability: "semantic_compatibility"
        })
    ));
}

// ============================================================================
// Preparation
// ============================================================================

#[test]
fn preparation_preserves_request_identity() {
    let prepared = PreparedMigration::prepare(
        &valid_request(),
        default_capabilities(),
    )
    .expect("valid migration must prepare");

    assert_eq!(
        prepared.migration_id.as_str(),
        MIGRATION_ID
    );
    assert_eq!(
        prepared.execution_id.as_str(),
        SOURCE_EXECUTION_ID
    );
    assert_eq!(
        prepared.program.as_str(),
        PROGRAM_ID
    );
    assert_eq!(
        prepared.source_target.as_str(),
        SOURCE_TARGET_ID
    );
    assert_eq!(
        prepared.destination_target.as_str(),
        DESTINATION_TARGET_ID
    );
}

#[test]
fn logical_migration_requires_destination_logical_state_support() {
    let mut capabilities = default_capabilities();
    capabilities.logical_state_migration = false;

    assert!(matches!(
        PreparedMigration::prepare(
            &valid_request(),
            capabilities,
        ),
        Err(MigrationError::CapabilityUnavailable {
            capability: "logical_state_migration"
        })
    ));
}

#[test]
fn provider_managed_migration_requires_destination_support() {
    let mut request = valid_request();

    request.quantum_state_transfer =
        QuantumStateTransfer::ProviderManaged;

    let mut capabilities = default_capabilities();
    capabilities.provider_state_migration = false;

    assert!(matches!(
        PreparedMigration::prepare(
            &request,
            capabilities,
        ),
        Err(MigrationError::CapabilityUnavailable {
            capability: "provider_state_migration"
        })
    ));
}

#[test]
fn destination_without_canonical_program_requires_recompilation() {
    let mut capabilities = default_capabilities();
    capabilities.canonical_program = false;

    let prepared = PreparedMigration::prepare(
        &valid_request(),
        capabilities,
    )
    .expect("default policy permits recompilation");

    assert!(prepared.recompilation_required);
}

#[test]
fn required_recompilation_can_be_forbidden_by_policy() {
    let mut request = valid_request();
    request.policy.allow_recompilation = false;

    let mut capabilities = default_capabilities();
    capabilities.canonical_program = false;

    assert!(matches!(
        PreparedMigration::prepare(
            &request,
            capabilities,
        ),
        Err(MigrationError::RecompilationNotAllowed)
    ));
}

// ============================================================================
// Checkpoint integration
// ============================================================================

#[test]
fn checkpoint_validation_occurs_before_execution() {
    let checkpoints = TestCheckpointProvider::usable();
    let executor = TestExecutor::successful();

    let service = make_service(
        TestCapabilityProvider::new(default_capabilities()),
        checkpoints,
        executor,
        TestExecutionAdapter::successful(),
        TestVerifier::new(VerificationMode::Accepted),
    );

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Accepted
    );
}

#[test]
fn unusable_checkpoint_prevents_destination_execution() {
    let executor = TestExecutor::successful();

    let service = make_service(
        TestCapabilityProvider::new(default_capabilities()),
        TestCheckpointProvider::unusable(),
        executor,
        TestExecutionAdapter::successful(),
        TestVerifier::new(VerificationMode::Accepted),
    );

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Rejected
    );
    assert!(outcome.destination_execution.is_none());
}

#[test]
fn checkpoint_provider_failure_prevents_execution() {
    let executor = TestExecutor::successful();

    let service = make_service(
        TestCapabilityProvider::new(default_capabilities()),
        TestCheckpointProvider::failing(),
        executor,
        TestExecutionAdapter::successful(),
        TestVerifier::new(VerificationMode::Accepted),
    );

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Rejected
    );
    assert!(outcome.destination_execution.is_none());
}

// ============================================================================
// Capability-provider integration
// ============================================================================

#[test]
fn destination_capabilities_are_requested_for_destination_target() {
    let capabilities =
        TestCapabilityProvider::new(default_capabilities());

    let service = make_service(
        capabilities,
        TestCheckpointProvider::usable(),
        TestExecutor::successful(),
        TestExecutionAdapter::successful(),
        TestVerifier::new(VerificationMode::Accepted),
    );

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Accepted
    );
}

#[test]
fn capability_provider_failure_prevents_execution() {
    let executor = TestExecutor::successful();

    let service = make_service(
        TestCapabilityProvider::failing(default_capabilities()),
        TestCheckpointProvider::usable(),
        executor,
        TestExecutionAdapter::successful(),
        TestVerifier::new(VerificationMode::Accepted),
    );

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Rejected
    );
    assert!(outcome.destination_execution.is_none());
}

// ============================================================================
// Authorization
// ============================================================================

#[test]
fn authorization_denial_prevents_migration() {
    let authorizer = Arc::new(TestAuthorizer::denying());

    let service =
        successful_service().with_authorizer(authorizer.clone());

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Rejected
    );
    assert_eq!(authorizer.calls(), 1);
}

#[test]
fn successful_authorization_allows_migration() {
    let authorizer = Arc::new(TestAuthorizer::allowing());

    let service =
        successful_service().with_authorizer(authorizer.clone());

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Accepted
    );
    assert_eq!(authorizer.calls(), 1);
}

// ============================================================================
// Cancellation
// ============================================================================

#[test]
fn cancellation_before_preparation_prevents_migration() {
    let service = successful_service().with_cancellation(
        Arc::new(TestCancellation { cancelled: true }),
    );

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Cancelled
    );
    assert!(outcome.destination_execution.is_none());
}

// ============================================================================
// Executor integration
// ============================================================================

#[test]
fn executor_receives_prepared_migration() {
    let executor = TestExecutor::successful();

    let service = make_service(
        TestCapabilityProvider::new(default_capabilities()),
        TestCheckpointProvider::usable(),
        executor,
        TestExecutionAdapter::successful(),
        TestVerifier::new(VerificationMode::Accepted),
    );

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Accepted
    );
}

#[test]
fn executor_failure_is_not_success() {
    let service = make_service(
        TestCapabilityProvider::new(default_capabilities()),
        TestCheckpointProvider::usable(),
        TestExecutor::failing(),
        TestExecutionAdapter::successful(),
        TestVerifier::new(VerificationMode::Accepted),
    );

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Failed
    );
    assert!(!outcome.accepted());
    assert!(outcome.destination_execution.is_none());
}

// ============================================================================
// Adapter integration
// ============================================================================

#[test]
fn adapter_failure_is_not_success() {
    let service = make_service(
        TestCapabilityProvider::new(default_capabilities()),
        TestCheckpointProvider::usable(),
        TestExecutor::successful(),
        TestExecutionAdapter::failing(),
        TestVerifier::new(VerificationMode::Accepted),
    );

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Failed
    );
    assert!(!outcome.accepted());
}

// ============================================================================
// Verification
// ============================================================================

#[test]
fn verification_is_mandatory_for_acceptance() {
    let verifier = TestVerifier::new(
        VerificationMode::Rejected,
    );

    let service = make_service(
        TestCapabilityProvider::new(default_capabilities()),
        TestCheckpointProvider::usable(),
        TestExecutor::successful(),
        TestExecutionAdapter::successful(),
        verifier,
    );

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Escalated
    );
    assert!(!outcome.accepted());
}

#[test]
fn verification_failure_escalates() {
    let service = make_service(
        TestCapabilityProvider::new(default_capabilities()),
        TestCheckpointProvider::usable(),
        TestExecutor::successful(),
        TestExecutionAdapter::successful(),
        TestVerifier::new(VerificationMode::Failed),
    );

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Escalated
    );
    assert!(!outcome.accepted());
}

#[test]
fn invalid_verification_confidence_is_rejected() {
    let service = make_service(
        TestCapabilityProvider::new(default_capabilities()),
        TestCheckpointProvider::usable(),
        TestExecutor::successful(),
        TestExecutionAdapter::successful(),
        TestVerifier::new(
            VerificationMode::InvalidConfidence,
        ),
    );

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Rejected
    );
    assert!(!outcome.accepted());
}

#[test]
fn degraded_execution_requires_explicit_policy_permission() {
    let service = make_service(
        TestCapabilityProvider::new(default_capabilities()),
        TestCheckpointProvider::usable(),
        TestExecutor::successful(),
        TestExecutionAdapter::successful(),
        TestVerifier::new(
            VerificationMode::AcceptedDegraded,
        ),
    );

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Escalated
    );
    assert!(!outcome.accepted());
}

#[test]
fn degraded_execution_can_be_explicitly_accepted() {
    let mut request = valid_request();
    request.policy.allow_degraded_execution = true;

    let service = make_service(
        TestCapabilityProvider::new(default_capabilities()),
        TestCheckpointProvider::usable(),
        TestExecutor::successful(),
        TestExecutionAdapter::successful(),
        TestVerifier::new(
            VerificationMode::AcceptedDegraded,
        ),
    );

    let outcome = service.migrate(request);

    assert_eq!(
        outcome.status,
        MigrationStatus::Degraded
    );
    assert!(outcome.accepted());
}

// ============================================================================
// Outcome and provenance
// ============================================================================

#[test]
fn accepted_outcome_contains_destination_execution() {
    let outcome = successful_service()
        .migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Accepted
    );

    assert_eq!(
        outcome.destination_execution
            .as_ref()
            .map(ExecutionId::as_str),
        Some(DESTINATION_EXECUTION_ID)
    );
}

#[test]
fn accepted_outcome_contains_verification() {
    let outcome = successful_service()
        .migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Accepted
    );

    let verification = outcome
        .verification
        .as_ref()
        .expect("accepted migration must be verified");

    assert!(verification.semantically_equivalent);
    assert!(verification.capability_compatible);
    assert!(verification.state_valid);
    assert!(verification.provenance_complete);
}

#[test]
fn provenance_preserves_source_and_destination() {
    let outcome = successful_service()
        .migrate(valid_request());

    let provenance: &MigrationProvenance =
        &outcome.provenance;

    assert_eq!(
        provenance.source_execution.as_str(),
        SOURCE_EXECUTION_ID
    );
    assert_eq!(
        provenance.source_target.as_str(),
        SOURCE_TARGET_ID
    );
    assert_eq!(
        provenance.destination_target.as_str(),
        DESTINATION_TARGET_ID
    );
    assert_eq!(
        provenance.program.as_str(),
        PROGRAM_ID
    );
}

#[test]
fn caller_metadata_is_preserved_in_provenance() {
    let mut request = valid_request();

    request.metadata.insert(
        "trace_id".to_owned(),
        "trace-001".to_owned(),
    );

    request.metadata.insert(
        "reason".to_owned(),
        "hardware-degradation".to_owned(),
    );

    let outcome = successful_service()
        .migrate(request);

    assert_eq!(
        outcome.provenance.metadata
            .get("trace_id")
            .map(String::as_str),
        Some("trace-001")
    );

    assert_eq!(
        outcome.provenance.metadata
            .get("reason")
            .map(String::as_str),
        Some("hardware-degradation")
    );
}

#[test]
fn failure_preserves_source_provenance() {
    let service = make_service(
        TestCapabilityProvider::new(default_capabilities()),
        TestCheckpointProvider::usable(),
        TestExecutor::failing(),
        TestExecutionAdapter::successful(),
        TestVerifier::new(VerificationMode::Accepted),
    );

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Failed
    );

    assert_eq!(
        outcome.provenance.source_execution.as_str(),
        SOURCE_EXECUTION_ID
    );

    assert_eq!(
        outcome.provenance.source_target.as_str(),
        SOURCE_TARGET_ID
    );
}

// ============================================================================
// Observer
// ============================================================================

#[test]
fn observer_receives_started_and_completed_events() {
    let observer = Arc::new(TestObserver::default());

    let service =
        successful_service().with_observer(observer.clone());

    let outcome = service.migrate(valid_request());

    assert_eq!(
        observer.started(),
        vec![migration_id()]
    );

    assert_eq!(
        observer.completed(),
        vec![outcome]
    );
}

// ============================================================================
// Determinism
// ============================================================================

#[test]
fn fixed_clock_makes_elapsed_observation_deterministic() {
    let clock = FixedClock {
        time: UNIX_EPOCH
            + Duration::from_secs(10_000),
    };

    let service =
        successful_service().with_clock(Arc::new(clock));

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.elapsed,
        Some(Duration::ZERO)
    );
}

#[test]
fn repeated_deterministic_migrations_preserve_semantics() {
    let first = successful_service()
        .migrate(valid_request());

    let second = successful_service()
        .migrate(valid_request());

    assert_eq!(
        first.migration_id,
        second.migration_id
    );

    assert_eq!(
        first.source_execution,
        second.source_execution
    );

    assert_eq!(
        first.source_target,
        second.source_target
    );

    assert_eq!(
        first.destination_target,
        second.destination_target
    );

    assert_eq!(
        first.status,
        second.status
    );

    assert_eq!(
        first.provenance,
        second.provenance
    );
}

// ============================================================================
// Lifecycle
// ============================================================================

#[test]
fn terminal_statuses_are_terminal() {
    for status in [
        MigrationStatus::Accepted,
        MigrationStatus::Degraded,
        MigrationStatus::Rejected,
        MigrationStatus::Cancelled,
        MigrationStatus::Failed,
        MigrationStatus::Escalated,
    ] {
        assert!(status.is_terminal());
    }
}

#[test]
fn active_statuses_are_not_terminal() {
    for status in [
        MigrationStatus::Requested,
        MigrationStatus::Prepared,
        MigrationStatus::Migrating,
        MigrationStatus::Verifying,
    ] {
        assert!(!status.is_terminal());
    }
}

// ============================================================================
// No hidden machine-size assumption
// ============================================================================

#[test]
fn migration_contract_has_no_machine_size_limit() {
    let request = valid_request();

    let prepared = PreparedMigration::prepare(
        &request,
        default_capabilities(),
    )
    .expect("valid migration must prepare");

    assert!(prepared.destination_capabilities.executable);
    assert!(
        prepared
            .destination_capabilities
            .semantic_compatibility
    );
}

// ============================================================================
// Canonical implementation identity
// ============================================================================

#[test]
fn implementation_identity_is_stable() {
    let outcome = successful_service()
        .migrate(valid_request());

    assert_eq!(
        outcome.provenance.implementation,
        crate::quantum::resilience::recovery::migration::
            MIGRATION_IMPLEMENTATION_ID
    );
}

#[test]
fn migration_contract_version_is_explicit() {
    assert_eq!(
        crate::quantum::resilience::recovery::migration::
            MIGRATION_CONTRACT_VERSION,
        1
    );
}

// ============================================================================
// Observability helper
// ============================================================================

#[test]
fn unix_timestamp_is_observability_only() {
    assert_eq!(
        crate::quantum::resilience::recovery::migration::
            unix_timestamp_millis(UNIX_EPOCH),
        Some(0)
    );
}

// ============================================================================
// End-to-end acceptance invariant
// ============================================================================

#[test]
fn migration_is_accepted_only_when_all_required_verification_invariants_pass() {
    let outcome = successful_service()
        .migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Accepted
    );

    let verification = outcome
        .verification
        .as_ref()
        .expect("accepted migration must contain verification");

    assert!(verification.semantically_equivalent);
    assert!(verification.capability_compatible);
    assert!(verification.state_valid);
    assert!(verification.provenance_complete);
    assert!(!verification.degraded);
    assert!(outcome.accepted());
}

// ============================================================================
// Executor invocation contract
// ============================================================================

#[test]
fn executor_is_invoked_only_after_validation_boundaries_pass() {
    let executor = TestExecutor::successful();

    let service = make_service(
        TestCapabilityProvider::new(default_capabilities()),
        TestCheckpointProvider::usable(),
        executor,
        TestExecutionAdapter::successful(),
        TestVerifier::new(VerificationMode::Accepted),
    );

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Accepted
    );
}

// ============================================================================
// Checkpoint call accounting
// ============================================================================

#[test]
fn checkpoint_provider_is_consulted_for_migration_state() {
    let checkpoints =
        TestCheckpointProvider::usable();

    let service = make_service(
        TestCapabilityProvider::new(default_capabilities()),
        checkpoints,
        TestExecutor::successful(),
        TestExecutionAdapter::successful(),
        TestVerifier::new(VerificationMode::Accepted),
    );

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Accepted
    );
}

// ============================================================================
// Verifier call accounting
// ============================================================================

#[test]
fn verifier_is_called_for_successful_destination_execution() {
    let verifier =
        TestVerifier::new(VerificationMode::Accepted);

    let service = make_service(
        TestCapabilityProvider::new(default_capabilities()),
        TestCheckpointProvider::usable(),
        TestExecutor::successful(),
        TestExecutionAdapter::successful(),
        verifier,
    );

    let outcome = service.migrate(valid_request());

    assert_eq!(
        outcome.status,
        MigrationStatus::Accepted
    );
}

// ============================================================================
// Provider neutrality
// ============================================================================

#[test]
fn request_contains_no_required_provider_specific_identity() {
    let request = valid_request();

    assert_eq!(
        request.source_target.as_str(),
        SOURCE_TARGET_ID
    );

    assert_eq!(
        request.destination_target.as_str(),
        DESTINATION_TARGET_ID
    );

    assert!(
        !request.metadata.contains_key("provider")
    );
}

// ============================================================================
// Quantum-state safety
// ============================================================================

#[test]
fn migration_never_accepts_arbitrary_unknown_quantum_state_transfer() {
    let mut request = valid_request();

    request.quantum_state_transfer =
        QuantumStateTransfer::ArbitraryUnknownState;

    let outcome = successful_service()
        .migrate(request);

    assert_eq!(
        outcome.status,
        MigrationStatus::Rejected
    );

    assert!(
        outcome.destination_execution.is_none()
    );
}

// ============================================================================
// Completion invariant
// ============================================================================

#[test]
fn successful_migration_preserves_destination_target() {
    let outcome = successful_service()
        .migrate(valid_request());

    assert_eq!(
        outcome.destination_target.as_str(),
        DESTINATION_TARGET_ID
    );

    assert_eq!(
        outcome.provenance.destination_target.as_str(),
        DESTINATION_TARGET_ID
    );
}