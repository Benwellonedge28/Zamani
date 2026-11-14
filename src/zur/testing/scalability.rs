
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Scalability Testing
//!
//! Path:
//!     src/zur/testing/scalability.rs
//!
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//!
//! # PURPOSE
//!
//! Provides a reusable, resource-parametric scalability testing
//! harness for the Zamani Universal Runtime.
//!
//! The harness evaluates whether actual runtime operations continue
//! to preserve their declared execution contracts as workload
//! dimensions increase.
//!
//! It supports:
//!
//! - Minimal and zero-sized workloads.
//! - Progressively increasing workloads.
//! - Arbitrarily parameterized scale values.
//! - Streaming and open-ended workloads.
//! - Multiple independent scalability dimensions.
//! - Custom future scalability dimensions.
//! - Runtime service scaling.
//! - Provider and device population scaling.
//! - Task and execution-session scaling.
//! - Distributed participant scaling.
//! - Data and memory workload scaling.
//! - Hybrid and cross-domain composition scaling.
//! - Explicit resource-admission checks.
//! - Cooperative cancellation.
//! - Operational failure reporting.
//! - Inconclusive execution reporting.
//! - Checked accounting.
//! - Expected workload verification.
//!
//! # ARCHITECTURAL OWNERSHIP
//!
//! THIS MODULE OWNS:
//!
//! - Scalability test orchestration.
//! - Scale-axis identities.
//! - Scale-point descriptions.
//! - Workload execution observations.
//! - Incremental workload accounting.
//! - Aggregate scalability reports.
//! - Scalability-run termination classification.
//! - Failure evidence preservation.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - Source-language grammar.
//! - AST construction.
//! - Semantic analysis.
//! - Resource requirements.
//! - Resource capabilities.
//! - Resource feasibility.
//! - Resource allocation.
//! - Resource reservation.
//! - Resource leases.
//! - Hardware discovery.
//! - Hardware selection.
//! - Backend registration.
//! - Code generation.
//! - Compiler optimization.
//! - Runtime scheduling.
//! - Provider registration.
//! - Provider lifecycle.
//! - Provider execution.
//! - Classical IR.
//! - Quantum IR.
//! - HDLIR.
//! - AI or tensor IR.
//! - ZUIR.
//! - Hybrid execution planning.
//! - Distributed execution planning.
//! - Quantum scheduling.
//! - Routing.
//! - Physical execution.
//!
//! These responsibilities remain with their canonical owners.
//!
//! # EXISTING AUTHORITIES
//!
//! Grammar and scalability specifications:
//!
//! - grammar/specification/scalability-model.md
//! - grammar/specification/poco-reaf.md
//! - grammar/specification/execution-model.md
//! - grammar/specification/semantic-model.md
//! - grammar/resources/scalability.g4
//! - grammar/validation/scalability-rules.md
//! - grammar/validation/scalability.md
//!
//! Runtime:
//!
//! - zur/error.rs
//! - zur/resources/
//! - zur/memory/
//! - zur/concurrency/
//! - zur/scheduling/
//! - zur/execution/
//! - zur/domains/
//! - zur/distributed/
//! - zur/hybrid/
//! - zur/testing/provider_harness.rs
//! - zur/testing/conformance.rs
//! - zur/testing/portability.rs
//!
//! Compiler:
//!
//! - compiler/testing/scalability.rs
//! - compiler/configuration/budget
//! - compiler/planning/
//! - compiler/artifacts/
//!
//! Standard library:
//!
//! - stdlib/context.rs
//!
//! # GRAMMAR BOUNDARY
//!
//! This module consumes already validated runtime operations and
//! their associated scale parameters.
//!
//! It does not parse scalability grammar.
//!
//! It does not construct AST nodes.
//!
//! It does not perform semantic analysis.
//!
//! It does not interpret source-language scalability declarations.
//!
//! grammar/resources/scalability.g4 owns the source syntax for
//! expressing scalability intent.
//!
//! grammar/specification/scalability-model.md owns the normative
//! scalability model.
//!
//! grammar/validation/scalability.md owns scalability validation
//! requirements.
//!
//! This module tests the operational implementation against those
//! established contracts.
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Scalability testing must distinguish:
//!
//! 1. Program semantic validity.
//! 2. Target capability.
//! 3. Actual resource availability.
//! 4. Operational resource budgets.
//! 5. Execution feasibility.
//! 6. Actual execution.
//! 7. Successful execution completion.
//!
//! A successful small-scale execution does not establish that
//! a larger execution is supported.
//!
//! An unavailable resource does not establish a language-level
//! capacity restriction.
//!
//! A successful resource query does not establish reservation.
//!
//! An accepted execution request does not establish completion.
//!
//! # OPEN-WORLD SCALABILITY
//!
//! No universal maximum is imposed on:
//!
//! - Scale points.
//! - Workload items.
//! - Program units.
//! - Tasks.
//! - Services.
//! - Providers.
//! - Devices.
//! - Distributed participants.
//! - Sessions.
//! - Data units.
//! - Memory requirements.
//! - Computational domains.
//! - Domain compositions.
//! - Custom scalability dimensions.
//!
//! The harness uses u128 to represent requested scale quantities.
//!
//! This is a representation choice, not a universal language limit.
//!
//! Workloads are consumed incrementally.
//!
//! No intermediate collection of all workload items is constructed.
//!
//! Reports retain aggregate counters rather than every processed item.
//!
//! This permits open-ended workloads without requiring report memory
//! to grow proportionally to the number of processed test cases.
//!
//! Actual execution remains subject to:
//!
//! - Available physical resources.
//! - Rust and platform representation limits.
//! - Available memory.
//! - Execution time.
//! - Explicit operational policies.
//! - Target capabilities.
//! - Implementation constraints.
//!
//! Literal infinite physical computation is not promised.
//!
//! # SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe.
//! No raw pointers.
//! No global mutable state.
//! No fabricated execution success.
//! No implicit resource allocation.
//! No implicit resource reservation.
//! No implicit hardware discovery.
//! No implicit hardware selection.
//! No silent workload truncation.
//! No silent counter wrapping.
//! No automatic fallback that weakens program semantics.
//!
//! # DETERMINISM
//!
//! Workloads retain caller-supplied iteration order.
//!
//! The harness does not reorder scale points.
//!
//! The harness does not introduce random workload generation.
//!
//! Elapsed time is observational telemetry only.
//!
//! It is not a semantic correctness criterion.
//!
//! # INTEGRATION
//!
//! This module complements:
//!
//! - compiler/testing/scalability.rs
//! - zur/testing/provider_harness.rs
//! - zur/testing/conformance.rs
//! - zur/testing/portability.rs
//!
//! Compiler scalability testing establishes compiler workload behavior.
//!
//! Provider testing establishes provider conformance.
//!
//! Portability testing establishes cross-environment behavioral
//! compatibility.
//!
//! This module coordinates scalable operational test workloads.
//!
//! It does not replace any of those testing authorities.

use core::fmt;

use std::time::{Duration, Instant};

use crate::stdlib::context::OperationContext;

use crate::zur::error::{
    ErrorKind,
    ZurError,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.testing.scalability";

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of the ZUR scalability-testing interface.
///
/// This is independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - AST version.
/// - Semantic-model version.
/// - Compiler version.
/// - IR schema version.
/// - Runtime ABI version.
/// - Artifact version.
pub const SCALABILITY_TESTING_API_VERSION: u16 = 1;

// ============================================================================
// SCALE AXIS
// ============================================================================

/// An extensible identity for a scalability dimension.
///
/// Scale axes use open-world, namespaced identities.
///
/// Examples:
///
/// - `program.units`
/// - `task.instances`
/// - `memory.bytes`
/// - `service.instances`
/// - `provider.instances`
/// - `device.instances`
/// - `distributed.participants`
/// - `data.units`
/// - `session.instances`
/// - `domain.compositions`
/// - `quantum.logical_resources`
/// - `hardware.generated_structures`
///
/// These are examples of test dimensions, not a closed inventory.
///
/// New dimensions may be introduced without modifying this module.
///
/// # Validation
///
/// Namespace segments are separated by dots.
///
/// Each segment accepts ASCII letters, digits, underscores
/// and hyphens.
///
/// No arbitrary identity-length ceiling is imposed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScaleAxis(String);

impl ScaleAxis {
    /// Creates an extensible scale-axis identity.
    ///
    /// # Errors
    ///
    /// Returns a structured error for:
    ///
    /// - Empty identities.
    /// - Surrounding whitespace.
    /// - Empty namespace segments.
    /// - Unsupported characters.
    pub fn new(value: impl Into<String>) -> Result<Self, ZurError> {
        let value = value.into();

        if value.is_empty() {
            return Err(
                ZurError::invalid_input(
                    "scale-axis identity cannot be empty",
                )
                .with_component(COMPONENT)
                .with_operation("create_scale_axis"),
            );
        }

        if value.trim() != value {
            return Err(
                ZurError::invalid_input(
                    "scale-axis identity cannot contain surrounding whitespace",
                )
                .with_component(COMPONENT)
                .with_operation("create_scale_axis"),
            );
        }

        for segment in value.split('.') {
            if segment.is_empty() {
                return Err(
                    ZurError::invalid_input(
                        "scale-axis identity contains an empty namespace segment",
                    )
                    .with_component(COMPONENT)
                    .with_operation("create_scale_axis"),
                );
            }

            if !segment.bytes().all(|byte| {
                byte.is_ascii_alphanumeric()
                    || byte == b'_'
                    || byte == b'-'
            }) {
                return Err(
                    ZurError::invalid_input(
                        "scale-axis identity contains unsupported characters",
                    )
                    .with_component(COMPONENT)
                    .with_operation("create_scale_axis"),
                );
            }
        }

        Ok(Self(value))
    }

    /// Returns the canonical scale-axis identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the axis and returns its identity.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }

    /// Returns the program-unit scalability dimension.
    #[must_use]
    pub fn program_units() -> Self {
        Self("program.units".to_owned())
    }

    /// Returns the task-instance scalability dimension.
    #[must_use]
    pub fn task_instances() -> Self {
        Self("task.instances".to_owned())
    }

    /// Returns the memory-byte scalability dimension.
    #[must_use]
    pub fn memory_bytes() -> Self {
        Self("memory.bytes".to_owned())
    }

    /// Returns the service-instance scalability dimension.
    #[must_use]
    pub fn service_instances() -> Self {
        Self("service.instances".to_owned())
    }

    /// Returns the provider-instance scalability dimension.
    #[must_use]
    pub fn provider_instances() -> Self {
        Self("provider.instances".to_owned())
    }

    /// Returns the device-instance scalability dimension.
    #[must_use]
    pub fn device_instances() -> Self {
        Self("device.instances".to_owned())
    }

    /// Returns the distributed-participant scalability dimension.
    #[must_use]
    pub fn distributed_participants() -> Self {
        Self("distributed.participants".to_owned())
    }

    /// Returns the data-unit scalability dimension.
    #[must_use]
    pub fn data_units() -> Self {
        Self("data.units".to_owned())
    }

    /// Returns the execution-session scalability dimension.
    #[must_use]
    pub fn session_instances() -> Self {
        Self("session.instances".to_owned())
    }

    /// Returns the cross-domain composition scalability dimension.
    #[must_use]
    pub fn domain_compositions() -> Self {
        Self("domain.compositions".to_owned())
    }
}

impl fmt::Display for ScaleAxis {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// SCALE POINT
// ============================================================================

/// A single requested scale point.
///
/// `scale` is the caller-defined requested quantity for the
/// associated axis.
///
/// It is not automatically interpreted as:
///
/// - A hardware capacity.
/// - A resource reservation.
/// - A semantic requirement.
/// - A feasibility result.
/// - An execution result.
///
/// Those meanings belong to their respective authorities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScalePoint {
    axis: ScaleAxis,
    scale: u128,
}

impl ScalePoint {
    /// Creates a scale point.
    ///
    /// Zero is permitted because zero-sized and empty workloads
    /// are valid testing cases.
    #[must_use]
    pub fn new(axis: ScaleAxis, scale: u128) -> Self {
        Self { axis, scale }
    }

    /// Returns the scalability dimension.
    #[must_use]
    pub fn axis(&self) -> &ScaleAxis {
        &self.axis
    }

    /// Returns the requested scale.
    #[must_use]
    pub const fn scale(&self) -> u128 {
        self.scale
    }
}

// ============================================================================
// EXECUTION OBSERVATION
// ============================================================================

/// Actual scale observation returned by a workload operation.
///
/// The caller must report the scale it actually exercised.
///
/// The harness compares that observation with the requested scale.
///
/// A mismatch is a failed scalability case.
///
/// `completed_units` is caller-defined operational evidence.
///
/// It must represent actual successfully completed work, not merely
/// allocated metadata, planned work or submitted operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaleObservation {
    observed_scale: u128,
    completed_units: u128,
}

impl ScaleObservation {
    /// Creates an observation of an executed workload.
    ///
    /// `observed_scale` must identify the scale actually exercised.
    ///
    /// `completed_units` must represent work that actually completed.
    #[must_use]
    pub const fn new(
        observed_scale: u128,
        completed_units: u128,
    ) -> Self {
        Self {
            observed_scale,
            completed_units,
        }
    }

    /// Returns the observed scale.
    #[must_use]
    pub const fn observed_scale(&self) -> u128 {
        self.observed_scale
    }

    /// Returns the number of successfully completed work units.
    #[must_use]
    pub const fn completed_units(&self) -> u128 {
        self.completed_units
    }
}

// ============================================================================
// SCALE ASSESSMENT
// ============================================================================

/// Assessment produced by one actual scalability operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScaleAssessment {
    /// The operation completed and supplied an actual observation.
    Verified(ScaleObservation),

    /// The operation could not establish sufficient evidence.
    ///
    /// An inconclusive assessment is never treated as a pass.
    Inconclusive {
        /// Explanation of why the result remains inconclusive.
        reason: String,
    },
}

impl ScaleAssessment {
    /// Constructs a verified assessment.
    #[must_use]
    pub fn verified(observation: ScaleObservation) -> Self {
        Self::Verified(observation)
    }

    /// Constructs an inconclusive assessment.
    #[must_use]
    pub fn inconclusive(reason: impl Into<String>) -> Self {
        Self::Inconclusive {
            reason: reason.into(),
        }
    }
}

// ============================================================================
// RUN OPTIONS
// ============================================================================

/// Configuration for one scalability-testing run.
///
/// The expected case count is optional because the harness also
/// supports open-ended streaming workloads.
///
/// The expected count is a test assertion, not a universal capacity
/// ceiling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ScalabilityRunOptions {
    expected_cases: Option<u128>,
    continue_after_inconclusive: bool,
}

impl ScalabilityRunOptions {
    /// Creates default scalability-testing options.
    ///
    /// By default, the harness continues after an inconclusive case.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            expected_cases: None,
            continue_after_inconclusive: true,
        }
    }

    /// Sets the expected number of supplied test cases.
    ///
    /// This validates completion; it does not restrict the
    /// language or workload domain.
    #[must_use]
    pub const fn with_expected_cases(mut self, expected: u128) -> Self {
        self.expected_cases = Some(expected);
        self
    }

    /// Configures whether execution continues after an inconclusive case.
    #[must_use]
    pub const fn with_continue_after_inconclusive(
        mut self,
        enabled: bool,
    ) -> Self {
        self.continue_after_inconclusive = enabled;
        self
    }

    /// Returns the expected case count.
    #[must_use]
    pub const fn expected_cases(&self) -> Option<u128> {
        self.expected_cases
    }

    /// Returns whether inconclusive cases permit continued testing.
    #[must_use]
    pub const fn continues_after_inconclusive(&self) -> bool {
        self.continue_after_inconclusive
    }
}

// ============================================================================
// FAILURE PHASE
// ============================================================================

/// Identifies the stage at which a scalability run failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleFailurePhase {
    /// Resource or policy admission failed.
    Admission,

    /// Actual workload execution failed.
    Execution,

    /// An execution observation violated its contract.
    ObservationValidation,

    /// Cooperative cancellation was requested.
    Cancellation,

    /// Internal accounting could not be completed.
    Accounting,

    /// Expected workload completion was not satisfied.
    CompletionValidation,
}

// ============================================================================
// FAILURE EVIDENCE
// ============================================================================

/// Structured evidence describing a scalability-test failure.
///
/// The original error classification, code and message are preserved.
///
/// The original error is not retained because a report must remain
/// independently inspectable without retaining arbitrary error sources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaleFailure {
    phase: ScaleFailurePhase,
    kind: ErrorKind,
    code: String,
    message: String,
    axis: Option<ScaleAxis>,
    requested_scale: Option<u128>,
}

impl ScaleFailure {
    fn from_error(
        phase: ScaleFailurePhase,
        error: ZurError,
        point: Option<&ScalePoint>,
    ) -> Self {
        Self {
            phase,
            kind: error.kind(),
            code: error.code().to_string(),
            message: error.message().to_owned(),
            axis: point.map(|value| value.axis().clone()),
            requested_scale: point.map(ScalePoint::scale),
        }
    }

    /// Returns the failure phase.
    #[must_use]
    pub const fn phase(&self) -> ScaleFailurePhase {
        self.phase
    }

    /// Returns the canonical ZUR error classification.
    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Returns the canonical error code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Returns the diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the associated scalability dimension.
    #[must_use]
    pub fn axis(&self) -> Option<&ScaleAxis> {
        self.axis.as_ref()
    }

    /// Returns the requested scale associated with the failure.
    #[must_use]
    pub const fn requested_scale(&self) -> Option<u128> {
        self.requested_scale
    }
}

// ============================================================================
// TERMINATION
// ============================================================================

/// Explains how a scalability-testing run terminated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleRunTermination {
    /// All supplied workload items were processed.
    Completed,

    /// Cooperative cancellation stopped the run.
    Cancelled,

    /// The resource or policy admission operation rejected a case.
    AdmissionRejected,

    /// Actual workload execution failed.
    ExecutionFailed,

    /// An inconclusive result stopped the run under the selected policy.
    Inconclusive,

    /// Internal checked accounting failed.
    AccountingFailed,

    /// The expected number of test cases was not supplied.
    ExpectedCountMismatch,
}

// ============================================================================
// REPORT
// ============================================================================

/// Aggregate result of a scalability-testing run.
///
/// The report uses aggregate counters rather than retaining every
/// processed test case.
///
/// This keeps harness-owned memory independent of workload length.
///
/// Callers that require individual execution observations may collect
/// them inside their own supplied workload callbacks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScalabilityReport {
    considered_cases: u128,
    executed_cases: u128,
    verified_cases: u128,
    inconclusive_cases: u128,
    failed_cases: u128,
    not_run_cases: u128,
    completed_units: u128,
    termination: ScaleRunTermination,
    failure: Option<ScaleFailure>,
    elapsed: Duration,
}

impl ScalabilityReport {
    fn new() -> Self {
        Self {
            considered_cases: 0,
            executed_cases: 0,
            verified_cases: 0,
            inconclusive_cases: 0,
            failed_cases: 0,
            not_run_cases: 0,
            completed_units: 0,
            termination: ScaleRunTermination::Completed,
            failure: None,
            elapsed: Duration::ZERO,
        }
    }

    /// Returns the number of scale points obtained from the iterator.
    #[must_use]
    pub const fn considered_cases(&self) -> u128 {
        self.considered_cases
    }

    /// Returns the number of actual workload callback invocations.
    #[must_use]
    pub const fn executed_cases(&self) -> u128 {
        self.executed_cases
    }

    /// Returns the number of verified scalability cases.
    #[must_use]
    pub const fn verified_cases(&self) -> u128 {
        self.verified_cases
    }

    /// Returns the number of inconclusive scalability cases.
    #[must_use]
    pub const fn inconclusive_cases(&self) -> u128 {
        self.inconclusive_cases
    }

    /// Returns the number of failed scalability cases.
    #[must_use]
    pub const fn failed_cases(&self) -> u128 {
        self.failed_cases
    }

    /// Returns the number of cases rejected before execution.
    #[must_use]
    pub const fn not_run_cases(&self) -> u128 {
        self.not_run_cases
    }

    /// Returns the total successfully completed work units.
    #[must_use]
    pub const fn completed_units(&self) -> u128 {
        self.completed_units
    }

    /// Returns the run termination classification.
    #[must_use]
    pub const fn termination(&self) -> ScaleRunTermination {
        self.termination
    }

    /// Returns the available failure evidence.
    #[must_use]
    pub fn failure(&self) -> Option<&ScaleFailure> {
        self.failure.as_ref()
    }

    /// Returns observed elapsed time.
    ///
    /// This is telemetry, not a semantic correctness guarantee.
    #[must_use]
    pub const fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// Returns whether the run completed without unresolved cases.
    ///
    /// An empty run is not considered evidence of scalability.
    ///
    /// An inconclusive result is not a passing result.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.termination == ScaleRunTermination::Completed
            && self.considered_cases > 0
            && self.executed_cases > 0
            && self.verified_cases == self.executed_cases
            && self.inconclusive_cases == 0
            && self.failed_cases == 0
            && self.not_run_cases == 0
            && self.failure.is_none()
    }

    /// Returns whether any test case was successfully verified.
    #[must_use]
    pub const fn has_verified_evidence(&self) -> bool {
        self.verified_cases > 0
    }
}

// ============================================================================
// HARNESS
// ============================================================================

/// ZUR scalability-testing harness.
///
/// The harness is stateless.
///
/// It owns no global runtime, provider registry, resource manager,
/// execution engine or hardware inventory.
#[derive(Debug, Clone, Copy, Default)]
pub struct ScalabilityHarness;

impl ScalabilityHarness {
    /// Creates a scalability-testing harness.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Executes a caller-supplied streaming scalability workload.
    ///
    /// The iterator is consumed exactly once and in its supplied order.
    ///
    /// No intermediate workload vector is created.
    ///
    /// The caller supplies two operations:
    ///
    /// `admit`:
    ///
    /// Checks whether the requested case is permitted to proceed.
    ///
    /// It should delegate to the appropriate canonical resource,
    /// authorization, policy or feasibility authority.
    ///
    /// Admission does not itself reserve resources.
    ///
    /// `execute`:
    ///
    /// Performs the actual operation being tested.
    ///
    /// It must return an observation describing the scale actually
    /// exercised, or an inconclusive assessment.
    ///
    /// The harness verifies that the reported observed scale matches
    /// the requested scale.
    ///
    /// # Cancellation
    ///
    /// The supplied OperationContext is checked before every case.
    ///
    /// Cancellation is cooperative. It does not forcibly terminate
    /// an operation already in progress.
    ///
    /// # Failure behavior
    ///
    /// A failed admission prevents execution of that case.
    ///
    /// A failed execution stops the run.
    ///
    /// A mismatched scale observation is a failure.
    ///
    /// An inconclusive result is never converted into success.
    ///
    /// # Open-ended workloads
    ///
    /// The iterator may be finite or open-ended.
    ///
    /// Open-ended workloads should provide a cancellation signal
    /// through OperationContext or otherwise terminate through
    /// their own execution conditions.
    ///
    /// # Errors
    ///
    /// Returns a structured report.
    ///
    /// Operational failures are preserved in the report rather
    /// than converted into successful partial completion.
    pub fn run_streaming<I, A, F>(
        &self,
        context: &OperationContext,
        workload: I,
        options: ScalabilityRunOptions,
        mut admit: A,
        mut execute: F,
    ) -> ScalabilityReport
    where
        I: IntoIterator<Item = ScalePoint>,
        A: FnMut(&ScalePoint) -> Result<(), ZurError>,
        F: FnMut(&ScalePoint) -> Result<ScaleAssessment, ZurError>,
    {
        let started = Instant::now();

        let mut report = ScalabilityReport::new();

        for point in workload {
            // ------------------------------------------------------------
            // COOPERATIVE CANCELLATION
            // ------------------------------------------------------------

            if context.is_cancelled() {
                let reason = context
                    .cancellation_reason()
                    .unwrap_or_else(|| {
                        "scalability testing was cancelled".to_owned()
                    });

                let error = ZurError::cancelled(reason)
                    .with_component(COMPONENT)
                    .with_operation("run_streaming");

                report.failure = Some(
                    ScaleFailure::from_error(
                        ScaleFailurePhase::Cancellation,
                        error,
                        Some(&point),
                    ),
                );

                report.termination = ScaleRunTermination::Cancelled;

                break;
            }

            // ------------------------------------------------------------
            // CASE ACCOUNTING
            // ------------------------------------------------------------

            let Some(next_considered) =
                report.considered_cases.checked_add(1)
            else {
                Self::accounting_failure(
                    &mut report,
                    &point,
                    "considered-case counter overflow",
                );

                break;
            };

            report.considered_cases = next_considered;

            // ------------------------------------------------------------
            // RESOURCE AND POLICY ADMISSION
            // ------------------------------------------------------------

            if let Err(error) = admit(&point) {
                report.not_run_cases =
                    match report.not_run_cases.checked_add(1) {
                        Some(value) => value,
                        None => {
                            Self::accounting_failure(
                                &mut report,
                                &point,
                                "not-run counter overflow",
                            );

                            break;
                        }
                    };

                report.failure = Some(
                    ScaleFailure::from_error(
                        ScaleFailurePhase::Admission,
                        error.with_component(COMPONENT)
                            .with_operation("admit_scale_point"),
                        Some(&point),
                    ),
                );

                report.termination =
                    ScaleRunTermination::AdmissionRejected;

                break;
            }

            // ------------------------------------------------------------
            // EXECUTION ACCOUNTING
            // ------------------------------------------------------------

            let Some(next_executed) =
                report.executed_cases.checked_add(1)
            else {
                Self::accounting_failure(
                    &mut report,
                    &point,
                    "executed-case counter overflow",
                );

                break;
            };

            report.executed_cases = next_executed;

            // ------------------------------------------------------------
            // ACTUAL WORKLOAD EXECUTION
            // ------------------------------------------------------------

            let assessment = match execute(&point) {
                Ok(value) => value,

                Err(error) => {
                    report.failed_cases =
                        match report.failed_cases.checked_add(1) {
                            Some(value) => value,
                            None => {
                                Self::accounting_failure(
                                    &mut report,
                                    &point,
                                    "failed-case counter overflow",
                                );

                                break;
                            }
                        };

                    report.failure = Some(
                        ScaleFailure::from_error(
                            ScaleFailurePhase::Execution,
                            error.with_component(COMPONENT)
                                .with_operation("execute_scale_point"),
                            Some(&point),
                        ),
                    );

                    report.termination =
                        ScaleRunTermination::ExecutionFailed;

                    break;
                }
            };

            // ------------------------------------------------------------
            // ASSESSMENT
            // ------------------------------------------------------------

            match assessment {
                ScaleAssessment::Verified(observation) => {
                    // A successful callback is not sufficient.
                    //
                    // The actual observation must correspond to
                    // the requested scalability point.

                    if observation.observed_scale()
                        != point.scale()
                    {
                        report.failed_cases =
                            match report.failed_cases.checked_add(1) {
                                Some(value) => value,
                                None => {
                                    Self::accounting_failure(
                                        &mut report,
                                        &point,
                                        "failed-case counter overflow",
                                    );

                                    break;
                                }
                            };

                        let error = ZurError::invalid_data(
                            format!(
                                "scale observation mismatch: requested {}, observed {}",
                                point.scale(),
                                observation.observed_scale(),
                            ),
                        )
                        .with_component(COMPONENT)
                        .with_operation("validate_scale_observation");

                        report.failure = Some(
                            ScaleFailure::from_error(
                                ScaleFailurePhase::ObservationValidation,
                                error,
                                Some(&point),
                            ),
                        );

                        report.termination =
                            ScaleRunTermination::ExecutionFailed;

                        break;
                    }

                    // Checked accumulation.
                    //
                    // Never wrap or silently saturate counters.

                    let Some(next_verified) =
                        report.verified_cases.checked_add(1)
                    else {
                        Self::accounting_failure(
                            &mut report,
                            &point,
                            "verified-case counter overflow",
                        );

                        break;
                    };

                    let Some(next_units) =
                        report.completed_units.checked_add(
                            observation.completed_units(),
                        )
                    else {
                        Self::accounting_failure(
                            &mut report,
                            &point,
                            "completed-work counter overflow",
                        );

                        break;
                    };

                    report.verified_cases = next_verified;
                    report.completed_units = next_units;
                }

                ScaleAssessment::Inconclusive { reason } => {
                    let Some(next_inconclusive) =
                        report.inconclusive_cases.checked_add(1)
                    else {
                        Self::accounting_failure(
                            &mut report,
                            &point,
                            "inconclusive-case counter overflow",
                        );

                        break;
                    };

                    report.inconclusive_cases = next_inconclusive;

                    if !options.continues_after_inconclusive() {
                        report.failure = Some(
                            ScaleFailure {
                                phase: ScaleFailurePhase::ObservationValidation,
                                kind: ErrorKind::Other,
                                code: "zur.scalability.inconclusive".to_owned(),
                                message: reason,
                                axis: Some(point.axis().clone()),
                                requested_scale: Some(point.scale()),
                            },
                        );

                        report.termination =
                            ScaleRunTermination::Inconclusive;

                        break;
                    }
                }
            }
        }

        // ----------------------------------------------------------------
        // EXPECTED COMPLETION VALIDATION
        // ----------------------------------------------------------------

        if report.termination == ScaleRunTermination::Completed {
            if let Some(expected) = options.expected_cases() {
                if report.considered_cases != expected {
                    let error = ZurError::invalid_data(
                        format!(
                            "scalability workload count mismatch: expected {}, considered {}",
                            expected,
                            report.considered_cases,
                        ),
                    )
                    .with_component(COMPONENT)
                    .with_operation("validate_completion");

                    report.failure = Some(
                        ScaleFailure::from_error(
                            ScaleFailurePhase::CompletionValidation,
                            error,
                            None,
                        ),
                    );

                    report.termination =
                        ScaleRunTermination::ExpectedCountMismatch;
                }
            }
        }

        report.elapsed = started.elapsed();

        report
    }

    // ====================================================================
    // INTERNAL ACCOUNTING FAILURE
    // ====================================================================

    fn accounting_failure(
        report: &mut ScalabilityReport,
        point: &ScalePoint,
        message: &str,
    ) {
        let error = ZurError::internal(message)
            .with_component(COMPONENT)
            .with_operation("accounting");

        report.failure = Some(
            ScaleFailure::from_error(
                ScaleFailurePhase::Accounting,
                error,
                Some(point),
            ),
        );

        report.termination =
            ScaleRunTermination::AccountingFailed;
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::Cell;

    fn operation_context() -> OperationContext {
        OperationContext::builder()
            .with_component(COMPONENT)
            .expect("valid component")
            .with_operation("scalability_test")
            .expect("valid operation")
            .build()
    }

    fn verified(point: &ScalePoint) -> ScaleAssessment {
        ScaleAssessment::verified(
            ScaleObservation::new(
                point.scale(),
                point.scale(),
            ),
        )
    }

    #[test]
    fn api_version_is_available() {
        assert_ne!(
            SCALABILITY_TESTING_API_VERSION,
            0,
        );
    }

    #[test]
    fn scale_axis_accepts_open_world_namespaces() {
        let axis = ScaleAxis::new(
            "future.photonic.optical_resources",
        )
        .expect("valid extensible axis");

        assert_eq!(
            axis.as_str(),
            "future.photonic.optical_resources",
        );
    }

    #[test]
    fn scale_axis_rejects_invalid_namespaces() {
        assert!(
            ScaleAxis::new("").is_err()
        );

        assert!(
            ScaleAxis::new("alpha..beta").is_err()
        );

        assert!(
            ScaleAxis::new(" alpha.beta").is_err()
        );

        assert!(
            ScaleAxis::new("alpha/beta").is_err()
        );
    }

    #[test]
    fn zero_sized_workload_is_supported() {
        let context = operation_context();

        let harness = ScalabilityHarness::new();

        let report = harness.run_streaming(
            &context,
            [ScalePoint::new(
                ScaleAxis::program_units(),
                0,
            )],
            ScalabilityRunOptions::new()
                .with_expected_cases(1),
            |_| Ok(()),
            verified,
        );

        assert_eq!(report.considered_cases(), 1);
        assert_eq!(report.verified_cases(), 1);
        assert_eq!(report.completed_units(), 0);
        assert!(report.is_successful());
    }

    #[test]
    fn empty_stream_is_not_scalability_evidence() {
        let context = operation_context();

        let report = ScalabilityHarness::new().run_streaming(
            &context,
            std::iter::empty::<ScalePoint>(),
            ScalabilityRunOptions::new()
                .with_expected_cases(0),
            |_| Ok(()),
            verified,
        );

        assert_eq!(report.considered_cases(), 0);
        assert!(!report.is_successful());
    }

    #[test]
    fn progressively_larger_workloads_are_executed() {
        let context = operation_context();

        let executed = Cell::new(0_u128);

        let scales = [
            1_u128,
            2,
            4,
            8,
            16,
            32,
            64,
        ];

        let workload = scales.into_iter().map(|scale| {
            ScalePoint::new(
                ScaleAxis::task_instances(),
                scale,
            )
        });

        let report = ScalabilityHarness::new().run_streaming(
            &context,
            workload,
            ScalabilityRunOptions::new()
                .with_expected_cases(scales.len() as u128),
            |_| Ok(()),
            |point| {
                executed.set(executed.get() + 1);

                Ok(verified(point))
            },
        );

        assert_eq!(
            executed.get(),
            scales.len() as u128,
        );

        assert_eq!(
            report.verified_cases(),
            scales.len() as u128,
        );

        assert!(report.is_successful());
    }

    #[test]
    fn largest_representable_scale_is_not_artificially_rejected() {
        let context = operation_context();

        let point = ScalePoint::new(
            ScaleAxis::memory_bytes(),
            u128::MAX,
        );

        let report = ScalabilityHarness::new().run_streaming(
            &context,
            std::iter::once(point),
            ScalabilityRunOptions::new()
                .with_expected_cases(1),
            |_| Ok(()),
            |point| {
                Ok(
                    ScaleAssessment::verified(
                        ScaleObservation::new(
                            point.scale(),
                            1,
                        ),
                    ),
                )
            },
        );

        assert_eq!(report.verified_cases(), 1);
        assert!(report.is_successful());
    }

    #[test]
    fn workload_order_is_preserved() {
        let context = operation_context();

        let observed = std::cell::RefCell::new(
            Vec::<u128>::new(),
        );

        let scales = [9_u128, 2, 7, 1, 5];

        let workload = scales.into_iter().map(|scale| {
            ScalePoint::new(
                ScaleAxis::data_units(),
                scale,
            )
        });

        let report = ScalabilityHarness::new().run_streaming(
            &context,
            workload,
            ScalabilityRunOptions::new()
                .with_expected_cases(5),
            |_| Ok(()),
            |point| {
                observed.borrow_mut().push(point.scale());

                Ok(verified(point))
            },
        );

        assert_eq!(
            *observed.borrow(),
            scales.to_vec(),
        );

        assert!(report.is_successful());
    }

    #[test]
    fn execution_failure_stops_the_workload() {
        let context = operation_context();

        let executed = Cell::new(0_u128);

        let workload = (0_u128..10).map(|scale| {
            ScalePoint::new(
                ScaleAxis::task_instances(),
                scale,
            )
        });

        let report = ScalabilityHarness::new().run_streaming(
            &context,
            workload,
            ScalabilityRunOptions::new(),
            |_| Ok(()),
            |point| {
                executed.set(executed.get() + 1);

                if point.scale() == 3 {
                    return Err(
                        ZurError::resource_unavailable(
                            "test resource became unavailable",
                        ),
                    );
                }

                Ok(verified(point))
            },
        );

        assert_eq!(executed.get(), 4);

        assert_eq!(
            report.termination(),
            ScaleRunTermination::ExecutionFailed,
        );

        assert_eq!(report.verified_cases(), 3);
        assert_eq!(report.failed_cases(), 1);
        assert!(!report.is_successful());

        assert_eq!(
            report.failure().unwrap().phase(),
            ScaleFailurePhase::Execution,
        );
    }

    #[test]
    fn admission_rejection_prevents_execution() {
        let context = operation_context();

        let executed = Cell::new(0_u128);

        let report = ScalabilityHarness::new().run_streaming(
            &context,
            [
                ScalePoint::new(
                    ScaleAxis::device_instances(),
                    1,
                ),
                ScalePoint::new(
                    ScaleAxis::device_instances(),
                    2,
                ),
            ],
            ScalabilityRunOptions::new(),
            |point| {
                if point.scale() == 2 {
                    return Err(
                        ZurError::budget_exhausted(
                            "test admission budget exhausted",
                        ),
                    );
                }

                Ok(())
            },
            |point| {
                executed.set(executed.get() + 1);

                Ok(verified(point))
            },
        );

        assert_eq!(executed.get(), 1);
        assert_eq!(report.verified_cases(), 1);
        assert_eq!(report.not_run_cases(), 1);

        assert_eq!(
            report.termination(),
            ScaleRunTermination::AdmissionRejected,
        );

        assert!(!report.is_successful());
    }

    #[test]
    fn mismatched_scale_observation_is_rejected() {
        let context = operation_context();

        let report = ScalabilityHarness::new().run_streaming(
            &context,
            [ScalePoint::new(
                ScaleAxis::program_units(),
                100,
            )],
            ScalabilityRunOptions::new(),
            |_| Ok(()),
            |_| {
                Ok(
                    ScaleAssessment::verified(
                        ScaleObservation::new(
                            50,
                            50,
                        ),
                    ),
                )
            },
        );

        assert_eq!(
            report.termination(),
            ScaleRunTermination::ExecutionFailed,
        );

        assert_eq!(report.failed_cases(), 1);
        assert!(!report.is_successful());

        assert_eq!(
            report.failure().unwrap().phase(),
            ScaleFailurePhase::ObservationValidation,
        );
    }

    #[test]
    fn inconclusive_results_are_not_successes() {
        let context = operation_context();

        let report = ScalabilityHarness::new().run_streaming(
            &context,
            [
                ScalePoint::new(
                    ScaleAxis::provider_instances(),
                    1,
                ),
            ],
            ScalabilityRunOptions::new(),
            |_| Ok(()),
            |_| {
                Ok(
                    ScaleAssessment::inconclusive(
                        "provider execution evidence unavailable",
                    ),
                )
            },
        );

        assert_eq!(report.inconclusive_cases(), 1);
        assert!(!report.is_successful());
    }

    #[test]
    fn inconclusive_policy_can_stop_testing() {
        let context = operation_context();

        let executed = Cell::new(0_u128);

        let report = ScalabilityHarness::new().run_streaming(
            &context,
            (0_u128..5).map(|scale| {
                ScalePoint::new(
                    ScaleAxis::session_instances(),
                    scale,
                )
            }),
            ScalabilityRunOptions::new()
                .with_continue_after_inconclusive(false),
            |_| Ok(()),
            |_| {
                executed.set(executed.get() + 1);

                Ok(
                    ScaleAssessment::inconclusive(
                        "insufficient execution evidence",
                    ),
                )
            },
        );

        assert_eq!(executed.get(), 1);

        assert_eq!(
            report.termination(),
            ScaleRunTermination::Inconclusive,
        );

        assert!(!report.is_successful());
    }

    #[test]
    fn inconclusive_cases_can_be_followed_by_verified_cases() {
        let context = operation_context();

        let report = ScalabilityHarness::new().run_streaming(
            &context,
            [
                ScalePoint::new(
                    ScaleAxis::domain_compositions(),
                    1,
                ),
                ScalePoint::new(
                    ScaleAxis::domain_compositions(),
                    2,
                ),
            ],
            ScalabilityRunOptions::new(),
            |_| Ok(()),
            |point| {
                if point.scale() == 1 {
                    return Ok(
                        ScaleAssessment::inconclusive(
                            "test evidence unavailable",
                        ),
                    );
                }

                Ok(verified(point))
            },
        );

        assert_eq!(report.inconclusive_cases(), 1);
        assert_eq!(report.verified_cases(), 1);
        assert!(!report.is_successful());
    }

    #[test]
    fn expected_case_count_is_validated() {
        let context = operation_context();

        let report = ScalabilityHarness::new().run_streaming(
            [
                ScalePoint::new(
                    ScaleAxis::program_units(),
                    1,
                ),
            ],
            &context,
            ScalabilityRunOptions::new()
                .with_expected_cases(2),
            |_| Ok(()),
            verified,
        );

        assert_eq!(
            report.termination(),
            ScaleRunTermination::ExpectedCountMismatch,
        );

        assert!(!report.is_successful());
    }
}
