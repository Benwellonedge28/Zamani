
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Fault Injection Testing
//!
//! Path: src/zur/testing/fault_injection.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97+
//!
//! PURPOSE
//!
//! Provides deterministic, provider-independent fault-injection testing
//! across all computational domains supported by Zamani.
//!
//! Supports:
//!
//! - Classical execution.
//! - Quantum execution.
//! - HDL and hardware.
//! - AI and tensor execution.
//! - Hybrid execution.
//! - Distributed execution.
//! - Accelerators.
//! - Embedded execution.
//! - Networking.
//! - Cryptographic computation.
//! - Future computational domains.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Fault-injection scenario definitions.
//! - Fault-injection plans.
//! - Fault-injection target contracts.
//! - Fault-injection observations.
//! - Fault-verification contracts.
//! - Verification observations.
//! - Fault-injection findings.
//! - Fault-injection reports.
//! - Partial-result preservation.
//! - Deterministic scenario execution.
//!
//! This module does NOT own:
//!
//! - Canonical fault classification.
//! - Quantum fault semantics.
//! - Hardware fault models.
//! - Quantum error correction.
//! - Provider registration.
//! - Provider discovery.
//! - Hardware selection.
//! - Physical resource allocation.
//! - Distributed membership.
//! - Distributed scheduling.
//! - Hybrid execution planning.
//! - Runtime recovery execution.
//! - Checkpoint serialization.
//! - Security authorization.
//! - Production execution orchestration.
//! - Grammar parsing.
//!
//! Those responsibilities remain with their authoritative subsystems.
//!
//! CANONICAL INTEGRATION
//!
//! Operational fault infrastructure:
//!     crate::zur::reliability::faults
//!
//! Operational errors:
//!     crate::zur::error
//!
//! Operation context and cancellation:
//!     crate::stdlib::context
//!
//! Quantum fault semantics:
//!     crate::quantum::zqn::fault
//!
//! Quantum resilience:
//!     crate::quantum::resilience
//!
//! Distributed resilience:
//!     crate::distributed::resilience
//!
//! Hybrid execution:
//!     crate::hybrid
//!
//! Provider testing:
//!     crate::zur::testing::provider_harness
//!
//! GRAMMAR BOUNDARY
//!
//! This module does not parse resilience or fault-tolerance declarations.
//!
//! It tests implementations of already-established operational contracts.
//!
//! Relevant specifications:
//!
//! - grammar/execution/resilience.g4
//! - grammar/distributed/fault-tolerance.g4
//! - grammar/specification/execution-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/poco-reaf.md
//! - grammar/specification/domains.md
//!
//! SAFETY
//!
//! Safe Rust only.
//! No unsafe.
//! No raw pointers.
//! No global mutable state.
//! No implicit hardware access.
//! No implicit provider selection.
//! No fabricated injection success.
//! No fabricated recovery success.
//!
//! SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Scenarios.
//! - Fault models.
//! - Providers.
//! - Computational domains.
//! - Fault records.
//! - Verification findings.
//! - Resource quantities.
//! - Execution sessions.
//!
//! Collection growth is constrained by actual allocator and system
//! resource availability.
//!
//! No finite machine promises literal infinite computation.
//!
//! INTEGRATION
//!
//! This module integrates with:
//!
//! - zur/reliability/faults.rs
//! - zur/reliability/recovery.rs
//! - zur/reliability/checkpoint.rs
//! - zur/testing/provider_harness.rs
//! - zur/testing/conformance.rs
//! - zur/testing/portability.rs
//! - stdlib/context.rs
//! - quantum/zqn/fault/
//! - quantum/resilience/
//! - distributed/resilience/
//! - hybrid/
//!
//! It does not implement any of those systems a second time.

use core::fmt;

use std::error::Error as StdError;

use crate::stdlib::context::OperationContext;

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::reliability::faults::{
    ExecutionEffect,
    FailureModel,
    FaultCategory,
    FaultOrigin,
    FaultRecord,
    Severity,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.testing.fault_injection";

const FAULT_ORIGIN: &str = "zamani.testing.fault_injection";

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

/// Validates a fault-injection scenario identifier.
///
/// Identifiers are open-world namespaced strings.
///
/// Accepted characters:
///
/// - ASCII letters.
/// - ASCII digits.
/// - Underscores.
/// - Hyphens.
/// - Dots.
/// - Colons.
///
/// Whitespace and empty namespace segments are rejected.
///
/// No arbitrary identifier-length ceiling is imposed.
///
/// # Errors
///
/// Returns an error when the identifier is empty, contains whitespace,
/// or contains an invalid namespace boundary.
pub fn validate_scenario_id(value: &str) -> ZurResult<()> {
    if value.is_empty() || value.trim() != value {
        return Err(
            ZurError::invalid_input(
                "fault-injection scenario identity is invalid",
            )
            .with_component(COMPONENT)
            .with_operation("validate_scenario_id"),
        );
    }

    let mut previous_separator = true;

    let bytes = value.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        let byte = bytes[index];

        if byte == b':' {
            if index + 1 >= bytes.len()
                || bytes[index + 1] != b':'
                || previous_separator
            {
                return Err(
                    ZurError::invalid_input(
                        "invalid fault-injection namespace separator",
                    )
                    .with_component(COMPONENT),
                );
            }

            previous_separator = true;
            index += 2;
            continue;
        }

        if byte == b'.' {
            if previous_separator {
                return Err(
                    ZurError::invalid_input(
                        "empty fault-injection namespace segment",
                    )
                    .with_component(COMPONENT),
                );
            }

            previous_separator = true;
            index += 1;
            continue;
        }

        let valid = byte.is_ascii_alphanumeric()
            || byte == b'_'
            || byte == b'-';

        if !valid {
            return Err(
                ZurError::invalid_input(
                    "unsupported character in scenario identity",
                )
                .with_component(COMPONENT),
            );
        }

        if previous_separator
            && (byte == b'_' || byte == b'-')
        {
            return Err(
                ZurError::invalid_input(
                    "invalid scenario identity boundary",
                )
                .with_component(COMPONENT),
            );
        }

        previous_separator = false;
        index += 1;
    }

    if previous_separator {
        return Err(
            ZurError::invalid_input(
                "scenario identity cannot end with a separator",
            )
            .with_component(COMPONENT),
        );
    }

    Ok(())
}

// ============================================================================
// FAULT-INJECTION STEP
// ============================================================================

/// One explicitly requested fault-injection scenario.
///
/// This describes a test operation. It does not itself inject a fault.
///
/// The actual operation is delegated to an explicitly supplied
/// FaultInjectionTarget.
#[derive(Debug, Clone)]
pub struct FaultInjectionStep {
    id: String,

    model: FailureModel,

    category: FaultCategory,

    severity: Severity,

    message: String,
}

impl FaultInjectionStep {
    /// Constructs a validated fault-injection step.
    ///
    /// The failure model and category use the existing canonical
    /// ZUR operational fault vocabulary.
    ///
    /// Domain-specific failure models remain open-world.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid identifiers, empty messages,
    /// or messages containing control characters.
    pub fn new(
        id: impl Into<String>,
        model: FailureModel,
        category: FaultCategory,
        severity: Severity,
        message: impl Into<String>,
    ) -> ZurResult<Self> {
        let id = id.into();
        let message = message.into();

        validate_scenario_id(&id)?;

        if message.trim().is_empty()
            || message.chars().any(char::is_control)
        {
            return Err(
                ZurError::invalid_input(
                    "fault-injection message is invalid",
                )
                .with_component(COMPONENT),
            );
        }

        Ok(Self {
            id,
            model,
            category,
            severity,
            message,
        })
    }

    /// Returns the scenario identity.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the canonical open-world failure model.
    #[must_use]
    pub fn model(&self) -> &FailureModel {
        &self.model
    }

    /// Returns the canonical operational fault category.
    #[must_use]
    pub const fn category(&self) -> FaultCategory {
        self.category
    }

    /// Returns the configured severity.
    #[must_use]
    pub const fn severity(&self) -> Severity {
        self.severity
    }

    /// Returns the descriptive fault message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// INJECTION DISPOSITION
// ============================================================================

/// Actual disposition reported by a fault-injection target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum InjectionDisposition {
    /// The target established that the fault was injected.
    Applied,

    /// The target established that injection did not occur.
    NotApplied,

    /// The target explicitly rejected the injection request.
    Rejected,

    /// The target cannot establish whether injection occurred.
    OutcomeUnknown,

    /// The target reported an operational failure.
    Failed,
}

impl InjectionDisposition {
    /// Returns whether injection was established as applied.
    #[must_use]
    pub const fn is_applied(self) -> bool {
        matches!(self, Self::Applied)
    }

    /// Returns whether injection remains uncertain.
    #[must_use]
    pub const fn is_uncertain(self) -> bool {
        matches!(self, Self::OutcomeUnknown)
    }
}

// ============================================================================
// INJECTION OBSERVATION
// ============================================================================

/// Actual observation returned by a fault-injection target.
///
/// This is an observation, not an assumed result.
///
/// In particular, OutcomeUnknown must not be interpreted as NotApplied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InjectionObservation {
    disposition: InjectionDisposition,

    execution_effect: ExecutionEffect,

    detail: String,
}

impl InjectionObservation {
    /// Constructs an injection observation.
    ///
    /// The target must report what it actually knows.
    ///
    /// # Errors
    ///
    /// Returns an error when the descriptive detail is empty
    /// or contains control characters.
    pub fn new(
        disposition: InjectionDisposition,
        execution_effect: ExecutionEffect,
        detail: impl Into<String>,
    ) -> ZurResult<Self> {
        let detail = detail.into();

        if detail.trim().is_empty()
            || detail.chars().any(char::is_control)
        {
            return Err(
                ZurError::invalid_input(
                    "injection observation detail is invalid",
                )
                .with_component(COMPONENT),
            );
        }

        Ok(Self {
            disposition,
            execution_effect,
            detail,
        })
    }

    /// Returns the observed injection disposition.
    #[must_use]
    pub const fn disposition(&self) -> InjectionDisposition {
        self.disposition
    }

    /// Returns the observed execution effect.
    #[must_use]
    pub const fn execution_effect(&self) -> ExecutionEffect {
        self.execution_effect
    }

    /// Returns the descriptive observation.
    #[must_use]
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

// ============================================================================
// INJECTION TARGET CONTRACT
// ============================================================================

/// Explicit implementation of a fault-injection target.
///
/// Implementations may represent:
///
/// - A test executor.
/// - A simulated device.
/// - A quantum simulator.
/// - A hardware test environment.
/// - A distributed test participant.
/// - A hybrid execution test environment.
/// - An independently implemented future-domain provider.
///
/// The target owns the actual injection mechanism.
///
/// The harness never accesses hardware directly.
///
/// Implementations must:
///
/// - Respect supplied execution context.
/// - Observe cooperative cancellation where appropriate.
/// - Report actual injection outcomes.
/// - Preserve uncertainty.
/// - Avoid claiming that a remote or physical fault occurred without
///   authoritative evidence.
///
/// A target must return OutcomeUnknown when it cannot establish
/// whether an operation took effect.
pub trait FaultInjectionTarget: Send + Sync {
    /// Injects one explicitly supplied fault scenario.
    ///
    /// # Errors
    ///
    /// Returns the actual operational error if injection could not
    /// be completed or its outcome could not be represented.
    ///
    /// An uncertain side effect should be represented using
    /// InjectionDisposition::OutcomeUnknown rather than silently
    /// treating the operation as not applied.
    fn inject(
        &self,
        context: &OperationContext,
        step: &FaultInjectionStep,
    ) -> ZurResult<InjectionObservation>;
}

// ============================================================================
// VERIFICATION DISPOSITION
// ============================================================================

/// Result of verifying the system response to an injected fault.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VerificationDisposition {
    /// The required behavior was actually verified.
    Verified,

    /// The required behavior was tested and failed.
    Failed,

    /// Available evidence was insufficient.
    Inconclusive,
}

impl VerificationDisposition {
    /// Returns whether verification established success.
    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::Verified)
    }
}

// ============================================================================
// VERIFICATION OBSERVATION
// ============================================================================

/// Actual result of a fault-response verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationObservation {
    disposition: VerificationDisposition,

    detail: String,
}

impl VerificationObservation {
    /// Constructs a verification observation.
    ///
    /// # Errors
    ///
    /// Returns an error for empty or invalid descriptive details.
    pub fn new(
        disposition: VerificationDisposition,
        detail: impl Into<String>,
    ) -> ZurResult<Self> {
        let detail = detail.into();

        if detail.trim().is_empty()
            || detail.chars().any(char::is_control)
        {
            return Err(
                ZurError::invalid_input(
                    "verification observation detail is invalid",
                )
                .with_component(COMPONENT),
            );
        }

        Ok(Self {
            disposition,
            detail,
        })
    }

    /// Returns the verification disposition.
    #[must_use]
    pub const fn disposition(&self) -> VerificationDisposition {
        self.disposition
    }

    /// Returns the descriptive verification result.
    #[must_use]
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

// ============================================================================
// VERIFICATION CONTRACT
// ============================================================================

/// Verifies the actual response to an injected fault.
///
/// Injection and verification are deliberately separate.
///
/// A successful injection does not establish that the application,
/// runtime, provider or recovery subsystem handled the fault correctly.
///
/// The verifier must inspect actual evidence from the relevant test
/// environment.
///
/// It must not manufacture successful verification results.
pub trait FaultInjectionVerifier: Send + Sync {
    /// Verifies the response to one injection.
    ///
    /// The verifier receives the original scenario and actual
    /// injection observation.
    ///
    /// For uncertain injection outcomes, the verifier may perform
    /// authoritative reconciliation before reaching a conclusion.
    ///
    /// # Errors
    ///
    /// Returns an operational error when verification itself
    /// cannot be completed.
    fn verify(
        &self,
        context: &OperationContext,
        step: &FaultInjectionStep,
        injection: &InjectionObservation,
    ) -> ZurResult<VerificationObservation>;
}

// ============================================================================
// FINDING
// ============================================================================

/// One complete fault-injection test finding.
///
/// A canonical FaultRecord is retained whenever the target establishes
/// that a synthetic fault was applied or its application is uncertain.
///
/// The original canonical ZUR fault model is not redefined.
#[derive(Debug)]
pub struct FaultInjectionFinding {
    scenario_id: String,

    injection: InjectionObservation,

    verification: Option<VerificationObservation>,

    fault: Option<FaultRecord>,

    diagnostic: Option<String>,
}

impl FaultInjectionFinding {
    /// Returns the scenario identity.
    #[must_use]
    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }

    /// Returns the actual injection observation.
    #[must_use]
    pub fn injection(&self) -> &InjectionObservation {
        &self.injection
    }

    /// Returns the verification observation, if verification ran.
    #[must_use]
    pub fn verification(&self) -> Option<&VerificationObservation> {
        self.verification.as_ref()
    }

    /// Returns the canonical operational fault record, if applicable.
    #[must_use]
    pub fn fault(&self) -> Option<&FaultRecord> {
        self.fault.as_ref()
    }

    /// Returns an operational diagnostic, if one occurred.
    #[must_use]
    pub fn diagnostic(&self) -> Option<&str> {
        self.diagnostic.as_deref()
    }

    /// Returns whether this finding established successful verification.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.injection.disposition().is_applied()
            && self
                .verification
                .as_ref()
                .is_some_and(|value| {
                    value.disposition().is_verified()
                })
    }
}

// ============================================================================
// REPORT
// ============================================================================

/// Aggregate report from an executed fault-injection plan.
///
/// The report preserves the actual order of execution.
///
/// It does not claim that untested scenarios passed.
#[derive(Debug, Default)]
pub struct FaultInjectionReport {
    findings: Vec<FaultInjectionFinding>,
}

impl FaultInjectionReport {
    /// Creates an empty report.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            findings: Vec::new(),
        }
    }

    /// Returns the number of recorded scenarios.
    #[must_use]
    pub fn len(&self) -> usize {
        self.findings.len()
    }

    /// Returns whether the report contains no findings.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.findings.is_empty()
    }

    /// Returns all recorded findings in execution order.
    #[must_use]
    pub fn findings(&self) -> &[FaultInjectionFinding] {
        &self.findings
    }

    /// Returns the number of verified scenarios.
    #[must_use]
    pub fn verified_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| {
                finding.is_successful()
            })
            .count()
    }

    /// Returns the number of scenarios whose verification failed.
    #[must_use]
    pub fn failed_verification_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| {
                finding.verification()
                    .is_some_and(|verification| {
                        verification.disposition()
                            == VerificationDisposition::Failed
                    })
            })
            .count()
    }

    /// Returns the number of uncertain injection outcomes.
    #[must_use]
    pub fn uncertain_injection_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| {
                finding.injection().disposition().is_uncertain()
            })
            .count()
    }

    /// Returns whether every recorded scenario was successfully
    /// injected and verified.
    ///
    /// An empty report is not considered successful.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        !self.findings.is_empty()
            && self.findings.iter().all(|finding| {
                finding.is_successful()
            })
    }

    fn try_push(
        &mut self,
        finding: FaultInjectionFinding,
    ) -> ZurResult<()> {
        self.findings.try_reserve(1).map_err(|_| {
            ZurError::resource_unavailable(
                "unable to allocate fault-injection report storage",
            )
            .with_component(COMPONENT)
        })?;

        self.findings.push(finding);

        Ok(())
    }
}

// ============================================================================
// FAILURE STAGE
// ============================================================================

/// Stage at which a fault-injection test could not continue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FaultInjectionFailureStage {
    /// Execution context was cancelled.
    Cancellation,

    /// The target could not perform the injection.
    Injection,

    /// The verifier could not complete verification.
    Verification,

    /// Report storage could not be allocated.
    Reporting,
}

impl fmt::Display for FaultInjectionFailureStage {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(match self {
            Self::Cancellation => "cancellation",
            Self::Injection => "injection",
            Self::Verification => "verification",
            Self::Reporting => "reporting",
        })
    }
}

// ============================================================================
// PARTIAL EXECUTION FAILURE
// ============================================================================

/// Failure returned by fault-injection plan execution.
///
/// Crucially, the partial report is preserved.
///
/// A test that injects several faults before encountering an error
/// must not lose evidence from its previously completed scenarios.
#[derive(Debug)]
pub struct FaultInjectionRunError {
    stage: FaultInjectionFailureStage,

    report: FaultInjectionReport,

    cause: ZurError,
}

impl FaultInjectionRunError {
    /// Returns the stage at which execution stopped.
    #[must_use]
    pub const fn stage(&self) -> FaultInjectionFailureStage {
        self.stage
    }

    /// Returns the partial report.
    #[must_use]
    pub fn report(&self) -> &FaultInjectionReport {
        &self.report
    }

    /// Returns ownership of the partial report.
    #[must_use]
    pub fn into_report(self) -> FaultInjectionReport {
        self.report
    }

    /// Returns the underlying operational error.
    #[must_use]
    pub fn cause(&self) -> &ZurError {
        &self.cause
    }
}

impl fmt::Display for FaultInjectionRunError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "fault-injection execution stopped during {}: {}",
            self.stage,
            self.cause,
        )
    }
}

impl StdError for FaultInjectionRunError {
    fn source(
        &self,
    ) -> Option<&(dyn StdError + 'static)> {
        Some(&self.cause)
    }
}

/// Result of executing a fault-injection plan.
pub type FaultInjectionRunResult =
    Result<FaultInjectionReport, FaultInjectionRunError>;

// ============================================================================
// PLAN
// ============================================================================

/// An explicitly constructed fault-injection plan.
///
/// Steps execute in insertion order.
///
/// The plan does not automatically discover providers, allocate resources,
/// choose hardware, or activate fault injection.
///
/// There is no fixed maximum number of scenarios.
///
/// Storage grows subject to available memory.
#[derive(Debug, Default)]
pub struct FaultInjectionPlan {
    steps: Vec<FaultInjectionStep>,
}

impl FaultInjectionPlan {
    /// Creates an empty plan.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            steps: Vec::new(),
        }
    }

    /// Adds a scenario to the plan.
    ///
    /// Allocation failure is reported explicitly.
    ///
    /// Repeated failure models are permitted: they represent repeated
    /// injection experiments, not duplicate scenario identities.
    ///
    /// # Errors
    ///
    /// Returns an error when storage cannot be allocated.
    pub fn try_add(
        &mut self,
        step: FaultInjectionStep,
    ) -> ZurResult<()> {
        self.steps.try_reserve(1).map_err(|_| {
            ZurError::resource_unavailable(
                "unable to allocate fault-injection plan storage",
            )
            .with_component(COMPONENT)
        })?;

        self.steps.push(step);

        Ok(())
    }

    /// Returns the number of scenarios.
    #[must_use]
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Returns whether the plan contains no scenarios.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Returns the configured scenarios.
    #[must_use]
    pub fn steps(&self) -> &[FaultInjectionStep] {
        &self.steps
    }

    /// Executes the plan using explicitly supplied injection
    /// and verification implementations.
    ///
    /// Execution is deterministic and sequential.
    ///
    /// This method does not create threads, initialize providers,
    /// reserve resources or perform hardware discovery.
    ///
    /// The supplied OperationContext is checked before each scenario.
    ///
    /// Injection and verification are separate operations.
    ///
    /// A failed injection or verification operation stops the plan
    /// and preserves the partial report.
    ///
    /// A target reporting an uncertain outcome is passed to the
    /// verifier for reconciliation.
    ///
    /// # Errors
    ///
    /// Returns FaultInjectionRunError with the partial report and
    /// original operational error.
    pub fn run(
        &self,
        context: &OperationContext,
        target: &dyn FaultInjectionTarget,
        verifier: &dyn FaultInjectionVerifier,
    ) -> FaultInjectionRunResult {
        let mut report = FaultInjectionReport::new();

        if self.steps.is_empty() {
            return Ok(report);
        }

        let origin = FaultOrigin::new(FAULT_ORIGIN)
            .map_err(|_| {
                ZurError::internal(
                    "invalid internal fault-injection origin",
                )
                .with_component(COMPONENT)
            });

        let origin = match origin {
            Ok(value) => value,
            Err(cause) => {
                return Err(FaultInjectionRunError {
                    stage: FaultInjectionFailureStage::Reporting,
                    report,
                    cause,
                });
            }
        };

        for step in &self.steps {
            // ------------------------------------------------------------
            // CANCELLATION
            // ------------------------------------------------------------

            if context.is_cancelled() {
                return Err(FaultInjectionRunError {
                    stage: FaultInjectionFailureStage::Cancellation,
                    report,
                    cause: ZurError::cancelled(
                        "fault-injection plan was cancelled",
                    )
                    .with_component(COMPONENT),
                });
            }

            // ------------------------------------------------------------
            // RESERVE REPORT CAPACITY BEFORE EXTERNAL SIDE EFFECTS
            // ------------------------------------------------------------

            if let Err(cause) = report.findings.try_reserve(1) {
                return Err(FaultInjectionRunError {
                    stage: FaultInjectionFailureStage::Reporting,
                    report,
                    cause: ZurError::resource_unavailable(
                        format!(
                            "unable to reserve report storage: {cause}"
                        ),
                    )
                    .with_component(COMPONENT),
                });
            }

            // ------------------------------------------------------------
            // INJECTION
            // ------------------------------------------------------------

            let observation = match target.inject(context, step) {
                Ok(observation) => observation,

                Err(cause) => {
                    let diagnostic = cause.to_string();

                    report.findings.push(
                        FaultInjectionFinding {
                            scenario_id: step.id.clone(),

                            injection: InjectionObservation {
                                disposition: InjectionDisposition::Failed,
                                execution_effect: ExecutionEffect::Unknown,
                                detail: "injection operation failed".to_owned(),
                            },

                            verification: None,

                            fault: None,

                            diagnostic: Some(diagnostic),
                        },
                    );

                    return Err(FaultInjectionRunError {
                        stage: FaultInjectionFailureStage::Injection,
                        report,
                        cause,
                    });
                }
            };

            // ------------------------------------------------------------
            // CANONICAL OPERATIONAL FAULT RECORD
            // ------------------------------------------------------------

            let fault = match observation.disposition() {
                InjectionDisposition::Applied
                | InjectionDisposition::OutcomeUnknown => {
                    match FaultRecord::new(
                        step.category(),
                        step.model().clone(),
                        origin.clone(),
                        step.severity(),
                        step.message(),
                    ) {
                        Ok(record) => Some(
                            record.with_execution_effect(
                                observation.execution_effect(),
                            ),
                        ),

                        Err(cause) => {
                            return Err(FaultInjectionRunError {
                                stage: FaultInjectionFailureStage::Reporting,
                                report,
                                cause,
                            });
                        }
                    }
                }

                InjectionDisposition::NotApplied
                | InjectionDisposition::Rejected
                | InjectionDisposition::Failed => None,
            };

            // ------------------------------------------------------------
            // RECORD INJECTION BEFORE VERIFICATION
            //
            // This preserves the injected event even if verification
            // subsequently fails.
            // ------------------------------------------------------------

            report.findings.push(
                FaultInjectionFinding {
                    scenario_id: step.id.clone(),

                    injection: observation.clone(),

                    verification: None,

                    fault,

                    diagnostic: None,
                },
            );

            // ------------------------------------------------------------
            // VERIFICATION
            // ------------------------------------------------------------

            match observation.disposition() {
                InjectionDisposition::Applied
                | InjectionDisposition::OutcomeUnknown => {
                    if context.is_cancelled() {
                        return Err(FaultInjectionRunError {
                            stage: FaultInjectionFailureStage::Cancellation,
                            report,
                            cause: ZurError::cancelled(
                                "verification was cancelled after injection",
                            )
                            .with_component(COMPONENT),
                        });
                    }

                    match verifier.verify(
                        context,
                        step,
                        &observation,
                    ) {
                        Ok(verification) => {
                            if let Some(finding) =
                                report.findings.last_mut()
                            {
                                finding.verification =
                                    Some(verification);
                            }
                        }

                        Err(cause) => {
                            if let Some(finding) =
                                report.findings.last_mut()
                            {
                                finding.diagnostic =
                                    Some(cause.to_string());
                            }

                            return Err(FaultInjectionRunError {
                                stage: FaultInjectionFailureStage::Verification,
                                report,
                                cause,
                            });
                        }
                    }
                }

                InjectionDisposition::NotApplied
                | InjectionDisposition::Rejected
                | InjectionDisposition::Failed => {
                    // Verification is deliberately not claimed.
                }
            }
        }

        Ok(report)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{
        AtomicUsize,
        Ordering,
    };

    fn context() -> OperationContext {
        OperationContext::builder().build()
    }

    fn step(id: &str) -> FaultInjectionStep {
        FaultInjectionStep::new(
            id,
            FailureModel::new("provider_failure")
                .expect("valid failure model"),
            FaultCategory::Provider,
            Severity::Error,
            "Synthetic provider failure",
        )
        .expect("valid scenario")
    }

    struct AppliedTarget {
        calls: AtomicUsize,
    }

    impl FaultInjectionTarget for AppliedTarget {
        fn inject(
            &self,
            _context: &OperationContext,
            _step: &FaultInjectionStep,
        ) -> ZurResult<InjectionObservation> {
            self.calls.fetch_add(1, Ordering::SeqCst);

            InjectionObservation::new(
                InjectionDisposition::Applied,
                ExecutionEffect::NotStarted,
                "fault injection established",
            )
        }
    }

    struct SuccessfulVerifier {
        calls: AtomicUsize,
    }

    impl FaultInjectionVerifier for SuccessfulVerifier {
        fn verify(
            &self,
            _context: &OperationContext,
            _step: &FaultInjectionStep,
            _injection: &InjectionObservation,
        ) -> ZurResult<VerificationObservation> {
            self.calls.fetch_add(1, Ordering::SeqCst);

            VerificationObservation::new(
                VerificationDisposition::Verified,
                "expected fault response verified",
            )
        }
    }

    // ------------------------------------------------------------------------
    // IDENTIFIERS
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_namespaced_scenario_identifiers() {
        assert!(
            validate_scenario_id("zamani.runtime.memory_failure")
                .is_ok()
        );

        assert!(
            validate_scenario_id("vendor::photonics.device_failure")
                .is_ok()
        );

        assert!(
            validate_scenario_id("future.domain.failure")
                .is_ok()
        );
    }

    #[test]
    fn rejects_invalid_scenario_identifiers() {
        assert!(validate_scenario_id("").is_err());

        assert!(validate_scenario_id(" ").is_err());

        assert!(validate_scenario_id(".invalid").is_err());

        assert!(validate_scenario_id("invalid.").is_err());

        assert!(validate_scenario_id("invalid..name").is_err());

        assert!(validate_scenario_id("invalid::").is_err());

        assert!(validate_scenario_id("invalid:name").is_err());

        assert!(validate_scenario_id("invalid name").is_err());
    }

    #[test]
    fn accepts_long_open_world_identifiers() {
        let value = format!(
            "future.{}.failure",
            "domain".repeat(256),
        );

        assert!(validate_scenario_id(&value).is_ok());
    }

    // ------------------------------------------------------------------------
    // STEP CONSTRUCTION
    // ------------------------------------------------------------------------

    #[test]
    fn creates_valid_injection_step() {
        let scenario = step("provider.disconnect");

        assert_eq!(
            scenario.id(),
            "provider.disconnect",
        );

        assert_eq!(
            scenario.model().as_str(),
            "provider_failure",
        );

        assert_eq!(
            scenario.category(),
            FaultCategory::Provider,
        );
    }

    #[test]
    fn rejects_empty_scenario_message() {
        let result = FaultInjectionStep::new(
            "scenario",
            FailureModel::new("provider_failure").unwrap(),
            FaultCategory::Provider,
            Severity::Error,
            "",
        );

        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // OBSERVATION
    // ------------------------------------------------------------------------

    #[test]
    fn preserves_uncertain_injection_outcomes() {
        let observation = InjectionObservation::new(
            InjectionDisposition::OutcomeUnknown,
            ExecutionEffect::Unknown,
            "remote execution outcome is unknown",
        )
        .unwrap();

        assert!(
            observation.disposition().is_uncertain()
        );

        assert_eq!(
            observation.execution_effect(),
            ExecutionEffect::Unknown,
        );
    }

    #[test]
    fn rejects_invalid_injection_observation() {
        assert!(
            InjectionObservation::new(
                InjectionDisposition::Applied,
                ExecutionEffect::NotStarted,
                "",
            )
            .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // PLAN
    // ------------------------------------------------------------------------

    #[test]
    fn plan_preserves_insertion_order() {
        let mut plan = FaultInjectionPlan::new();

        plan.try_add(step("first")).unwrap();

        plan.try_add(step("second")).unwrap();

        plan.try_add(step("third")).unwrap();

        assert_eq!(plan.len(), 3);

        assert_eq!(plan.steps()[0].id(), "first");

        assert_eq!(plan.steps()[1].id(), "second");

        assert_eq!(plan.steps()[2].id(), "third");
    }

    #[test]
    fn plan_accepts_repeated_failure_models() {
        let mut plan = FaultInjectionPlan::new();

        plan.try_add(step("provider.first")).unwrap();

        plan.try_add(step("provider.second")).unwrap();

        assert_eq!(plan.len(), 2);

        assert_eq!(
            plan.steps()[0].model(),
            plan.steps()[1].model(),
        );
    }

    // ------------------------------------------------------------------------
    // ACTUAL EXECUTION
    // ------------------------------------------------------------------------

    #[test]
    fn executes_injection_and_verification() {
        let mut plan = FaultInjectionPlan::new();

        plan.try_add(step("provider.disconnect")).unwrap();

        let target = AppliedTarget {
            calls: AtomicUsize::new(0),
        };

        let verifier = SuccessfulVerifier {
            calls: AtomicUsize::new(0),
        };

        let report = plan
            .run(&context(), &target, &verifier)
            .unwrap();

        assert_eq!(report.len(), 1);

        assert_eq!(report.verified_count(), 1);

        assert!(report.is_successful());

        assert_eq!(
            target.calls.load(Ordering::SeqCst),
            1,
        );

        assert_eq!(
            verifier.calls.load(Ordering::SeqCst),
            1,
        );
    }

    #[test]
    fn report_preserves_canonical_fault_record() {
        let mut plan = FaultInjectionPlan::new();

        plan.try_add(step("provider.failure")).unwrap();

        let target = AppliedTarget {
            calls: AtomicUsize::new(0),
        };

        let verifier = SuccessfulVerifier {
            calls: AtomicUsize::new(0),
        };

        let report = plan
            .run(&context(), &target, &verifier)
            .unwrap();

        let finding = &report.findings()[0];

        let fault = finding
            .fault()
            .expect("applied fault must have a canonical record");

        assert_eq!(
            fault.category(),
            FaultCategory::Provider,
        );

        assert_eq!(
            fault.model().as_str(),
            "provider_failure",
        );

        assert_eq!(
            fault.origin().as_str(),
            FAULT_ORIGIN,
        );

        assert_eq!(
            fault.execution_effect(),
            ExecutionEffect::NotStarted,
        );
    }

    #[test]
    fn empty_report_is_not_successful() {
        let report = FaultInjectionReport::new();

        assert!(report.is_empty());

        assert!(!report.is_successful());
    }

    // ------------------------------------------------------------------------
    // PARTIAL FAILURE PRESERVATION
    // ------------------------------------------------------------------------

    struct FailingTarget;

    impl FaultInjectionTarget for FailingTarget {
        fn inject(
            &self,
            _context: &OperationContext,
            _step: &FaultInjectionStep,
        ) -> ZurResult<InjectionObservation> {
            Err(
                ZurError::invalid_input(
                    "injected target failure",
                )
                .with_component(COMPONENT),
            )
        }
    }

    #[test]
    fn injection_failure_preserves_partial_report() {
        let mut plan = FaultInjectionPlan::new();

        plan.try_add(step("provider.first")).unwrap();

        let result = plan.run(
            &context(),
            &FailingTarget,
            &SuccessfulVerifier {
                calls: AtomicUsize::new(0),
            },
        );

        let failure = result.unwrap_err();

        assert_eq!(
            failure.stage(),
            FaultInjectionFailureStage::Injection,
        );

        assert_eq!(
            failure.report().len(),
            1,
        );

        assert_eq!(
            failure.report().findings()[0]
                .injection()
                .disposition(),
            InjectionDisposition::Failed,
        );
    }

    // ------------------------------------------------------------------------
    // VERIFICATION FAILURE
    // ------------------------------------------------------------------------

    struct FailingVerifier;

    impl FaultInjectionVerifier for FailingVerifier {
        fn verify(
            &self,
            _context: &OperationContext,
            _step: &FaultInjectionStep,
            _injection: &InjectionObservation,
        ) -> ZurResult<VerificationObservation> {
            Err(
                ZurError::invalid_input(
                    "verification infrastructure failed",
                )
                .with_component(COMPONENT),
            )
        }
    }

    #[test]
    fn verification_failure_preserves_injected_fault() {
        let mut plan = FaultInjectionPlan::new();

        plan.try_add(step("provider.disconnect")).unwrap();

        let target = AppliedTarget {
            calls: AtomicUsize::new(0),
        };

        let result = plan.run(
            &context(),
            &target,
            &FailingVerifier,
        );

        let failure = result.unwrap_err();

        assert_eq!(
            failure.stage(),
            FaultInjectionFailureStage::Verification,
        );

        assert_eq!(
            failure.report().len(),
            1,
        );

        assert!(
            failure.report().findings()[0]
                .fault()
                .is_some()
        );

        assert!(
            failure.report().findings()[0]
                .verification()
                .is_none()
        );
    }

    // ------------------------------------------------------------------------
    // VERIFICATION HONESTY
    // ------------------------------------------------------------------------

    struct UnavailableTarget;

    impl FaultInjectionTarget for UnavailableTarget {
        fn inject(
            &self,
            _context: &OperationContext,
            _step: &FaultInjectionStep,
        ) -> ZurResult<InjectionObservation> {
            InjectionObservation::new(
                InjectionDisposition::NotApplied,
                ExecutionEffect::NotStarted,
                "target rejected the requested test operation",
            )
        }
    }

    #[test]
    fn unsuccessful_injection_is_not_verified() {
        let mut plan = FaultInjectionPlan::new();

        plan.try_add(step("provider.unavailable")).unwrap();

        let verifier = SuccessfulVerifier {
            calls: AtomicUsize::new(0),
        };

        let report = plan
            .run(&context(), &UnavailableTarget, &verifier)
            .unwrap();

        assert!(!report.is_successful());

        assert_eq!(
            verifier.calls.load(Ordering::SeqCst),
            0,
        );

        assert!(
            report.findings()[0].verification().is_none()
        );
    }

    // ------------------------------------------------------------------------
    // SCALABILITY
    // ------------------------------------------------------------------------

    #[test]
    fn plan_grows_without_an_artificial_scenario_ceiling() {
        let mut plan = FaultInjectionPlan::new();

        for index in 0..10_000 {
            plan.try_add(step(
                &format!("scenario.failure_{index}"),
            ))
            .unwrap();
        }

        assert_eq!(plan.len(), 10_000);
    }
}
