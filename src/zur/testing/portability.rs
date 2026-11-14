
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Portability Testing
//!
//! Path: src/zur/testing/portability.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Provides a reusable execution-portability testing boundary for ZUR.
//!
//! Portability testing evaluates whether the same declared computation
//! preserves its required observable semantics across independently
//! supplied execution environments.
//!
//! The module supports:
//!
//! - Runtime-free execution.
//! - Runtime-enabled execution.
//! - Hybrid execution.
//! - AOT-generated artifacts.
//! - JIT-enabled execution where explicitly supported.
//! - Classical computation.
//! - Quantum computation.
//! - HDL and hardware execution.
//! - AI and tensor execution.
//! - Hybrid and distributed computation.
//! - Future computational domains.
//!
//! This module does not implement those execution engines.
//!
//! It tests execution through explicitly supplied adapters.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Portability test cases.
//! - Test environment identities.
//! - Environment metadata.
//! - Execution-mode descriptions.
//! - Execution requests.
//! - Execution observations.
//! - Observation comparison contracts.
//! - Cross-environment findings.
//! - Portability reports.
//! - Portability acceptance decisions.
//!
//! This module does NOT own:
//!
//! - Source-language semantics.
//! - Grammar.
//! - AST.
//! - Semantic analysis.
//! - Resource requirements.
//! - Capability definitions.
//! - Feasibility.
//! - Resource reservation.
//! - Hardware discovery.
//! - Hardware selection.
//! - Compiler optimization.
//! - Backend registration.
//! - Artifact construction.
//! - Artifact portability classification.
//! - Domain-specific IR.
//! - Provider registration.
//! - Provider lifecycle.
//! - Scheduling.
//! - Routing.
//! - Physical execution.
//! - Distributed coordination.
//! - Hybrid planning.
//!
//! Existing repository authorities retain these responsibilities.
//!
//! GRAMMAR AUTHORITY
//!
//! Relevant specifications:
//!
//! - grammar/spec/portability.md
//! - grammar/spec/compatibility.md
//! - grammar/spec/resources.md
//! - grammar/spec/semantics.md
//! - grammar/specification/portability.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/poco-reaf.md
//! - grammar/specification/execution-model.md
//! - grammar/specification/domains.md
//! - grammar/validation/semantic-boundaries.md
//!
//! This module consumes established contracts.
//!
//! It does not parse or interpret grammar productions.
//!
//! ARCHITECTURAL INVARIANTS
//!
//! Same program intent != same physical realization.
//! Same artifact != proof of semantic portability.
//! Artifact portability declaration != verified portability.
//! Target compatibility != resource availability.
//! Resource availability != resource reservation.
//! Submission != execution completion.
//! Execution completion != semantic equivalence.
//! Successful execution on one environment != universal portability.
//!
//! A portability claim requires actual execution evidence and
//! successful comparison under an explicitly declared policy.
//!
//! SAFETY
//!
//! Safe Rust only.
//! No unsafe code.
//! No raw pointers.
//! No global mutable state.
//! No implicit filesystem access.
//! No implicit network access.
//! No implicit hardware discovery.
//! No implicit provider selection.
//! No fabricated execution success.
//! No silent semantic downgrade.
//!
//! SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Environments.
//! - Test cases.
//! - Domains.
//! - Attributes.
//! - Observations.
//! - Comparison findings.
//! - Execution records.
//! - Artifact references.
//! - Diagnostic messages.
//!
//! Collections grow according to actual implementation capacity
//! and available system resources.
//!
//! The module does not promise physically infinite computation.
//!
//! DETERMINISM
//!
//! Environment registration uses ordered maps.
//! Test cases preserve their supplied order.
//! Environment comparisons use deterministic ordering.
//! Reports preserve deterministic ordering.
//!
//! A custom comparison policy must be deterministic if deterministic
//! portability results are required.
//!
//! INTEGRATION
//!
//! Upstream:
//!
//! - compiler/artifacts/portability.rs
//! - compiler/artifacts/manifest.rs
//! - stdlib/testing/portability.rs
//! - hybrid/planning/portability.rs
//! - zur/contracts/provider.rs
//! - zur/domains/provider.rs
//! - zur/error.rs
//!
//! Downstream:
//!
//! - zur/testing/conformance.rs
//! - zur/testing/provider_harness.rs
//! - compiler artifact integration tests
//! - runtime-free execution tests
//! - runtime-enabled execution tests
//! - hybrid integration tests
//! - distributed integration tests
//! - CI portability suites

use std::collections::BTreeMap;
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.testing.portability";

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

/// Validates an extensible portability identity.
///
/// Identity components are separated by dots.
///
/// Supported characters:
///
/// - ASCII letters.
/// - ASCII digits.
/// - Underscores.
/// - Hyphens.
///
/// No arbitrary identity-length limit is imposed.
///
/// # Errors
///
/// Returns a structured error for malformed identities.
pub fn validate_portability_identity(
    value: &str,
) -> Result<(), PortabilityError> {

    if value.is_empty() {
        return Err(PortabilityError::invalid_identity(
            "identity cannot be empty",
        ));
    }

    if value.trim() != value {
        return Err(PortabilityError::invalid_identity(
            "identity cannot contain surrounding whitespace",
        ));
    }

    for segment in value.split('.') {

        if segment.is_empty() {
            return Err(PortabilityError::invalid_identity(
                "identity contains an empty namespace segment",
            ));
        }

        if !segment.bytes().all(|byte| {
            byte.is_ascii_alphanumeric()
                || byte == b'_'
                || byte == b'-'
        }) {
            return Err(PortabilityError::invalid_identity(
                "identity contains unsupported characters",
            ));
        }
    }

    Ok(())
}

// ============================================================================
// ERROR MODEL
// ============================================================================

/// Portability testing error.
///
/// This type describes errors in the portability testing machinery.
///
/// Execution failures are preserved as execution findings rather
/// than being incorrectly reported as successful comparisons.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PortabilityError {

    /// An identity is invalid.
    InvalidIdentity {
        /// Diagnostic description.
        reason: String,
    },

    /// A duplicate environment was registered.
    DuplicateEnvironment {
        /// Duplicated environment identity.
        environment: String,
    },

    /// A duplicate test case was registered.
    DuplicateTestCase {
        /// Duplicated test case identity.
        test_case: String,
    },

    /// An execution environment does not exist.
    UnknownEnvironment {
        /// Requested environment.
        environment: String,
    },

    /// An execution environment is already registered.
    EnvironmentAlreadyRegistered {
        /// Environment identity.
        environment: String,
    },

    /// A comparison policy rejected an observation.
    ComparisonRejected {
        /// Description of the rejection.
        reason: String,
    },

    /// A report failed internal validation.
    InvalidReport {
        /// Description of the inconsistency.
        reason: String,
    },

    /// No valid portability comparison could be established.
    InsufficientEvidence {
        /// Description of missing evidence.
        reason: String,
    },
}

impl PortabilityError {

    /// Creates an invalid-identity error.
    #[must_use]
    pub fn invalid_identity(
        reason: impl Into<String>,
    ) -> Self {

        Self::InvalidIdentity {
            reason: reason.into(),
        }
    }

    /// Creates an invalid-report error.
    #[must_use]
    pub fn invalid_report(
        reason: impl Into<String>,
    ) -> Self {

        Self::InvalidReport {
            reason: reason.into(),
        }
    }
}

impl fmt::Display for PortabilityError {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        match self {

            Self::InvalidIdentity { reason } => {
                write!(
                    formatter,
                    "invalid portability identity: {reason}"
                )
            }

            Self::DuplicateEnvironment { environment } => {
                write!(
                    formatter,
                    "duplicate environment: {environment}"
                )
            }

            Self::DuplicateTestCase { test_case } => {
                write!(
                    formatter,
                    "duplicate portability test case: {test_case}"
                )
            }

            Self::UnknownEnvironment { environment } => {
                write!(
                    formatter,
                    "unknown execution environment: {environment}"
                )
            }

            Self::EnvironmentAlreadyRegistered { environment } => {
                write!(
                    formatter,
                    "environment already registered: {environment}"
                )
            }

            Self::ComparisonRejected { reason } => {
                write!(
                    formatter,
                    "portability comparison rejected: {reason}"
                )
            }

            Self::InvalidReport { reason } => {
                write!(
                    formatter,
                    "invalid portability report: {reason}"
                )
            }

            Self::InsufficientEvidence { reason } => {
                write!(
                    formatter,
                    "insufficient portability evidence: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for PortabilityError {}

/// Result type for portability testing.
pub type PortabilityResult<T> =
    Result<T, PortabilityError>;

// ============================================================================
// ENVIRONMENT IDENTITY
// ============================================================================

/// Open-world identity for an execution environment.
///
/// This is a logical testing identity, not a physical device identifier.
///
/// Examples:
///
/// - `classical.local`
/// - `classical.embedded`
/// - `quantum.simulator`
/// - `quantum.hardware`
/// - `hdl.synthesis`
/// - `hybrid.execution`
/// - `distributed.cluster`
/// - `future.photonics.execution`
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct PortabilityEnvironmentId(String);

impl PortabilityEnvironmentId {

    /// Creates a validated environment identity.
    pub fn new(
        value: impl Into<String>,
    ) -> PortabilityResult<Self> {

        let value = value.into();

        validate_portability_identity(&value)?;

        Ok(Self(value))
    }

    /// Returns the identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PortabilityEnvironmentId {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        formatter.write_str(&self.0)
    }
}

// ============================================================================
// EXECUTION MODE
// ============================================================================

/// Execution configuration used by a portability test.
///
/// These are execution strategies, not computational domains.
///
/// A domain may use more than one execution mode.
///
/// JIT is represented separately because it is an optional execution
/// capability that may coexist with runtime-enabled execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PortabilityExecutionMode {

    /// Native or freestanding execution without a required ZUR runtime.
    RuntimeFree,

    /// Execution using explicitly available ZUR runtime services.
    RuntimeEnabled,

    /// Combination of direct and runtime-managed execution.
    Hybrid,

    /// Runtime-enabled execution using JIT where explicitly supported.
    RuntimeEnabledJit,
}

impl fmt::Display for PortabilityExecutionMode {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        formatter.write_str(match self {

            Self::RuntimeFree => "runtime_free",

            Self::RuntimeEnabled => "runtime_enabled",

            Self::Hybrid => "hybrid",

            Self::RuntimeEnabledJit => "runtime_enabled_jit",
        })
    }
}

// ============================================================================
// ENVIRONMENT DESCRIPTION
// ============================================================================

/// Describes an execution environment for portability testing.
///
/// Attributes are supplied by an external authoritative source.
///
/// This type does not discover physical devices or authenticate
/// capability evidence.
///
/// Attribute names use extensible namespaced identities.
///
/// Example attributes:
///
/// - `platform.architecture`
/// - `platform.operating_system`
/// - `runtime.version`
/// - `compiler.target`
/// - `provider.identity`
/// - `domain.identity`
/// - `execution.mode`
///
/// Values remain descriptive metadata, not proof of availability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortabilityEnvironment {

    id: PortabilityEnvironmentId,

    mode: PortabilityExecutionMode,

    attributes: BTreeMap<String, String>,
}

impl PortabilityEnvironment {

    /// Creates an execution environment description.
    pub fn new(
        id: PortabilityEnvironmentId,
        mode: PortabilityExecutionMode,
    ) -> Self {

        Self {
            id,
            mode,
            attributes: BTreeMap::new(),
        }
    }

    /// Adds a descriptive environment attribute.
    ///
    /// Duplicate keys are rejected instead of silently replacing
    /// existing metadata.
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> PortabilityResult<Self> {

        let key = key.into();
        let value = value.into();

        validate_portability_identity(&key)?;

        if value.chars().any(char::is_control) {

            return Err(
                PortabilityError::invalid_identity(
                    "environment attribute contains control characters",
                )
            );
        }

        if self.attributes.insert(
            key.clone(),
            value,
        ).is_some() {

            return Err(
                PortabilityError::invalid_identity(
                    format!(
                        "duplicate environment attribute: {key}"
                    ),
                )
            );
        }

        Ok(self)
    }

    /// Returns the environment identity.
    #[must_use]
    pub fn id(&self) -> &PortabilityEnvironmentId {
        &self.id
    }

    /// Returns the execution mode.
    #[must_use]
    pub const fn mode(&self) -> PortabilityExecutionMode {
        self.mode
    }

    /// Returns all environment attributes.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }

    /// Returns one environment attribute.
    #[must_use]
    pub fn attribute(
        &self,
        key: &str,
    ) -> Option<&str> {

        self.attributes
            .get(key)
            .map(String::as_str)
    }
}

// ============================================================================
// PORTABILITY TEST CASE
// ============================================================================

/// A portability test case identity.
///
/// The identity is independent of source-language syntax and
/// computational domain.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct PortabilityTestCaseId(String);

impl PortabilityTestCaseId {

    /// Creates a validated test case identity.
    pub fn new(
        value: impl Into<String>,
    ) -> PortabilityResult<Self> {

        let value = value.into();

        validate_portability_identity(&value)?;

        Ok(Self(value))
    }

    /// Returns the identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PortabilityTestCaseId {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        formatter.write_str(&self.0)
    }
}

/// A reusable portability test case.
///
/// `program_identity` identifies the same semantic program across
/// environments.
///
/// `contract_identity` identifies the applicable semantic contract.
///
/// Input bytes are supplied to the execution adapter without being
/// interpreted by the portability subsystem.
///
/// The adapter is responsible for passing them to the correct
/// execution implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortabilityTestCase {

    id: PortabilityTestCaseId,

    program_identity: String,

    contract_identity: String,

    input: Vec<u8>,
}

impl PortabilityTestCase {

    /// Constructs a portability test case.
    pub fn new(
        id: PortabilityTestCaseId,
        program_identity: impl Into<String>,
        contract_identity: impl Into<String>,
        input: Vec<u8>,
    ) -> PortabilityResult<Self> {

        let program_identity = program_identity.into();
        let contract_identity = contract_identity.into();

        validate_portability_identity(&program_identity)?;
        validate_portability_identity(&contract_identity)?;

        Ok(Self {
            id,
            program_identity,
            contract_identity,
            input,
        })
    }

    /// Returns the test case identity.
    #[must_use]
    pub fn id(&self) -> &PortabilityTestCaseId {
        &self.id
    }

    /// Returns the program identity.
    #[must_use]
    pub fn program_identity(&self) -> &str {
        &self.program_identity
    }

    /// Returns the semantic contract identity.
    #[must_use]
    pub fn contract_identity(&self) -> &str {
        &self.contract_identity
    }

    /// Returns the supplied input.
    #[must_use]
    pub fn input(&self) -> &[u8] {
        &self.input
    }
}

// ============================================================================
// EXECUTION REQUEST
// ============================================================================

/// A request to execute one portability test case.
///
/// The request is immutable and contains no hardware-selection decision.
#[derive(Debug, Clone)]
pub struct PortabilityExecutionRequest {

    case_id: PortabilityTestCaseId,

    program_identity: String,

    contract_identity: String,

    input: Vec<u8>,

    mode: PortabilityExecutionMode,
}

impl PortabilityExecutionRequest {

    /// Creates a request from a test case and execution mode.
    #[must_use]
    pub fn new(
        test_case: &PortabilityTestCase,
        mode: PortabilityExecutionMode,
    ) -> Self {

        Self {
            case_id: test_case.id.clone(),
            program_identity: test_case.program_identity.clone(),
            contract_identity: test_case.contract_identity.clone(),
            input: test_case.input.clone(),
            mode,
        }
    }

    /// Returns the test case identity.
    #[must_use]
    pub fn case_id(&self) -> &PortabilityTestCaseId {
        &self.case_id
    }

    /// Returns the program identity.
    #[must_use]
    pub fn program_identity(&self) -> &str {
        &self.program_identity
    }

    /// Returns the semantic contract identity.
    #[must_use]
    pub fn contract_identity(&self) -> &str {
        &self.contract_identity
    }

    /// Returns the execution input.
    #[must_use]
    pub fn input(&self) -> &[u8] {
        &self.input
    }

    /// Returns the requested execution mode.
    #[must_use]
    pub const fn mode(&self) -> PortabilityExecutionMode {
        self.mode
    }
}

// ============================================================================
// EXECUTION OBSERVATION
// ============================================================================

/// Actual outcome of one environment execution.
///
/// Unsupported and failed execution are never converted into
/// successful observations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PortabilityExecutionStatus {

    /// Execution completed successfully.
    Completed,

    /// Execution completed with a valid partial result.
    PartiallyCompleted,

    /// Execution is not supported by this environment.
    Unsupported,

    /// Execution failed.
    Failed,

    /// Execution was cancelled.
    Cancelled,

    /// Execution timed out.
    TimedOut,

    /// The available evidence is insufficient.
    Inconclusive,
}

impl PortabilityExecutionStatus {

    /// Returns whether execution completed fully.
    #[must_use]
    pub const fn is_completed(self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Returns whether execution permits a portability comparison.
    #[must_use]
    pub const fn permits_comparison(self) -> bool {
        matches!(self, Self::Completed)
    }
}

impl fmt::Display for PortabilityExecutionStatus {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        formatter.write_str(match self {

            Self::Completed => "completed",

            Self::PartiallyCompleted => "partially_completed",

            Self::Unsupported => "unsupported",

            Self::Failed => "failed",

            Self::Cancelled => "cancelled",

            Self::TimedOut => "timed_out",

            Self::Inconclusive => "inconclusive",
        })
    }
}

/// Actual observation produced by an execution environment.
///
/// Output bytes are opaque to ZUR portability testing.
///
/// The comparison policy defines how their semantic equivalence
/// is established.
///
/// Metadata is descriptive and must not be used as a substitute
/// for output validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortabilityObservation {

    status: PortabilityExecutionStatus,

    output: Vec<u8>,

    diagnostic: Option<String>,

    evidence_reference: Option<String>,
}

impl PortabilityObservation {

    /// Creates an observation from an actual execution outcome.
    pub fn new(
        status: PortabilityExecutionStatus,
        output: Vec<u8>,
        diagnostic: Option<String>,
        evidence_reference: Option<String>,
    ) -> PortabilityResult<Self> {

        if let Some(reference) = &evidence_reference {
            validate_portability_identity(reference)?;
        }

        if let Some(message) = &diagnostic {

            if message.chars().any(char::is_control) {

                return Err(
                    PortabilityError::invalid_identity(
                        "execution diagnostic contains control characters",
                    )
                );
            }
        }

        if status != PortabilityExecutionStatus::Completed
            && !output.is_empty()
            && status == PortabilityExecutionStatus::Unsupported
        {
            return Err(
                PortabilityError::invalid_identity(
                    "unsupported execution cannot provide a completed output",
                )
            );
        }

        Ok(Self {
            status,
            output,
            diagnostic,
            evidence_reference,
        })
    }

    /// Returns the actual execution status.
    #[must_use]
    pub const fn status(&self) -> PortabilityExecutionStatus {
        self.status
    }

    /// Returns the actual output.
    #[must_use]
    pub fn output(&self) -> &[u8] {
        &self.output
    }

    /// Returns an optional diagnostic.
    #[must_use]
    pub fn diagnostic(&self) -> Option<&str> {
        self.diagnostic.as_deref()
    }

    /// Returns the external execution evidence reference.
    #[must_use]
    pub fn evidence_reference(&self) -> Option<&str> {
        self.evidence_reference.as_deref()
    }
}

// ============================================================================
// EXECUTION ADAPTER
// ============================================================================

/// Explicit execution adapter for a portability environment.
///
/// Implementations bridge this testing module to actual execution
/// systems.
///
/// They may delegate to:
///
/// - A runtime-free executable.
/// - A runtime-enabled executor.
/// - A hybrid executor.
/// - A quantum simulator.
/// - A quantum hardware provider.
/// - An HDL or hardware provider.
/// - A distributed execution provider.
/// - A future computational-domain provider.
///
/// The adapter must execute the supplied program and return the
/// actual observed result.
///
/// It must not fabricate successful completion.
///
/// It must preserve the semantic contract supplied in the request.
pub trait PortabilityExecutionAdapter: Send + Sync {

    /// Executes one portability test request.
    ///
    /// An error indicates that the adapter could not produce a
    /// trustworthy execution observation.
    ///
    /// A supported execution that fails should return an observation
    /// with the appropriate failure status when possible.
    fn execute(
        &self,
        request: &PortabilityExecutionRequest,
    ) -> PortabilityResult<PortabilityObservation>;
}

// ============================================================================
// COMPARISON POLICY
// ============================================================================

/// Result of comparing two execution observations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PortabilityComparison {

    /// Observations satisfy the declared semantic equivalence policy.
    Equivalent,

    /// Observations violate the declared semantic equivalence policy.
    Different,

    /// Available evidence cannot establish equivalence or difference.
    Inconclusive,
}

impl fmt::Display for PortabilityComparison {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        formatter.write_str(match self {

            Self::Equivalent => "equivalent",

            Self::Different => "different",

            Self::Inconclusive => "inconclusive",
        })
    }
}

/// Defines semantic observation equivalence.
///
/// Exact output comparison is provided by the default implementation.
///
/// Domain-specific policies may implement this trait for:
///
/// - Floating-point tolerance.
/// - Numerical equivalence.
/// - Quantum measurement distributions.
/// - Nondeterministic results.
/// - Distributed consistency.
/// - Hardware timing contracts.
/// - Domain-specific observable behavior.
///
/// The policy must be justified by the applicable semantic contract.
///
/// It must not silently weaken program guarantees.
pub trait PortabilityComparisonPolicy: Send + Sync {

    /// Returns a stable comparison policy identity.
    fn id(&self) -> &str;

    /// Compares two actual execution observations.
    fn compare(
        &self,
        left: &PortabilityObservation,
        right: &PortabilityObservation,
    ) -> PortabilityResult<PortabilityComparison>;
}

/// Exact byte-for-byte output comparison.
///
/// This is the default policy.
///
/// It does not assume that all computational domains can use
/// byte equality as their semantic equivalence relation.
#[derive(Debug, Default, Clone, Copy)]
pub struct ExactOutputComparison;

impl PortabilityComparisonPolicy for ExactOutputComparison {

    fn id(&self) -> &str {
        "zamani.portability.exact_output"
    }

    fn compare(
        &self,
        left: &PortabilityObservation,
        right: &PortabilityObservation,
    ) -> PortabilityResult<PortabilityComparison> {

        if !left.status().permits_comparison()
            || !right.status().permits_comparison()
        {
            return Ok(PortabilityComparison::Inconclusive);
        }

        Ok(
            if left.output() == right.output() {
                PortabilityComparison::Equivalent
            } else {
                PortabilityComparison::Different
            }
        )
    }
}

// ============================================================================
// CANCELLATION
// ============================================================================

/// Cooperative cancellation contract for portability testing.
///
/// Cancellation is supplied explicitly by the caller.
///
/// No global cancellation state is used.
pub trait PortabilityCancellation: Send + Sync {

    /// Returns whether the current test should stop.
    fn is_cancelled(&self) -> bool;
}

/// Cancellation implementation that never requests cancellation.
#[derive(Debug, Default, Clone, Copy)]
pub struct NeverCancelled;

impl PortabilityCancellation for NeverCancelled {

    fn is_cancelled(&self) -> bool {
        false
    }
}

// ============================================================================
// EXECUTION RECORD
// ============================================================================

/// Recorded execution in one environment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortabilityExecutionRecord {

    environment: PortabilityEnvironmentId,

    case_id: PortabilityTestCaseId,

    mode: PortabilityExecutionMode,

    observation: Option<PortabilityObservation>,

    status: PortabilityExecutionStatus,
}

impl PortabilityExecutionRecord {

    /// Returns the environment identity.
    #[must_use]
    pub fn environment(&self) -> &PortabilityEnvironmentId {
        &self.environment
    }

    /// Returns the test case identity.
    #[must_use]
    pub fn case_id(&self) -> &PortabilityTestCaseId {
        &self.case_id
    }

    /// Returns the requested execution mode.
    #[must_use]
    pub const fn mode(&self) -> PortabilityExecutionMode {
        self.mode
    }

    /// Returns the actual execution status.
    #[must_use]
    pub const fn status(&self) -> PortabilityExecutionStatus {
        self.status
    }

    /// Returns the observation when one was produced.
    #[must_use]
    pub fn observation(&self) -> Option<&PortabilityObservation> {
        self.observation.as_ref()
    }
}

// ============================================================================
// COMPARISON FINDING
// ============================================================================

/// Result of comparing two execution environments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortabilityFinding {

    case_id: PortabilityTestCaseId,

    left: PortabilityEnvironmentId,

    right: PortabilityEnvironmentId,

    comparison: PortabilityComparison,

    message: String,
}

impl PortabilityFinding {

    /// Returns the test case identity.
    #[must_use]
    pub fn case_id(&self) -> &PortabilityTestCaseId {
        &self.case_id
    }

    /// Returns the first environment.
    #[must_use]
    pub fn left(&self) -> &PortabilityEnvironmentId {
        &self.left
    }

    /// Returns the second environment.
    #[must_use]
    pub fn right(&self) -> &PortabilityEnvironmentId {
        &self.right
    }

    /// Returns the actual comparison result.
    #[must_use]
    pub const fn comparison(&self) -> PortabilityComparison {
        self.comparison
    }

    /// Returns the diagnostic.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// PORTABILITY REPORT
// ============================================================================

/// Complete portability testing report.
///
/// A report is successful only when:
///
/// - At least two distinct environments were tested.
/// - Every required execution completed.
/// - Every required comparison completed.
/// - Every comparison established equivalence.
/// - No execution or comparison failed.
/// - No required evidence is missing.
///
/// An empty report never establishes portability.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PortabilityReport {

    policy_identity: String,

    executions: Vec<PortabilityExecutionRecord>,

    findings: Vec<PortabilityFinding>,

    required_environments: usize,

    required_test_cases: usize,
}

impl PortabilityReport {

    /// Creates an empty report.
    ///
    /// An empty report is not portable.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the comparison policy identity.
    #[must_use]
    pub fn policy_identity(&self) -> &str {
        &self.policy_identity
    }

    /// Returns all execution records.
    #[must_use]
    pub fn executions(&self) -> &[PortabilityExecutionRecord] {
        &self.executions
    }

    /// Returns all comparison findings.
    #[must_use]
    pub fn findings(&self) -> &[PortabilityFinding] {
        &self.findings
    }

    /// Returns the number of required environments.
    #[must_use]
    pub fn required_environment_count(&self) -> usize {
        self.required_environments
    }

    /// Returns the number of required test cases.
    #[must_use]
    pub fn required_test_case_count(&self) -> usize {
        self.required_test_cases
    }

    /// Returns the number of successful executions.
    #[must_use]
    pub fn completed_execution_count(&self) -> usize {

        self.executions
            .iter()
            .filter(|record| {
                record.status().is_completed()
            })
            .count()
    }

    /// Returns the number of failed executions.
    #[must_use]
    pub fn failed_execution_count(&self) -> usize {

        self.executions
            .iter()
            .filter(|record| {
                record.status()
                    == PortabilityExecutionStatus::Failed
            })
            .count()
    }

    /// Returns the number of unsupported executions.
    #[must_use]
    pub fn unsupported_execution_count(&self) -> usize {

        self.executions
            .iter()
            .filter(|record| {
                record.status()
                    == PortabilityExecutionStatus::Unsupported
            })
            .count()
    }

    /// Returns the number of cancelled executions.
    #[must_use]
    pub fn cancelled_execution_count(&self) -> usize {

        self.executions
            .iter()
            .filter(|record| {
                record.status()
                    == PortabilityExecutionStatus::Cancelled
            })
            .count()
    }

    /// Returns the number of equivalent comparisons.
    #[must_use]
    pub fn equivalent_count(&self) -> usize {

        self.findings
            .iter()
            .filter(|finding| {
                finding.comparison()
                    == PortabilityComparison::Equivalent
            })
            .count()
    }

    /// Returns the number of different comparisons.
    #[must_use]
    pub fn different_count(&self) -> usize {

        self.findings
            .iter()
            .filter(|finding| {
                finding.comparison()
                    == PortabilityComparison::Different
            })
            .count()
    }

    /// Returns the number of inconclusive comparisons.
    #[must_use]
    pub fn inconclusive_count(&self) -> usize {

        self.findings
            .iter()
            .filter(|finding| {
                finding.comparison()
                    == PortabilityComparison::Inconclusive
            })
            .count()
    }

    /// Returns whether portability has been established.
    ///
    /// This method deliberately rejects incomplete evidence.
    #[must_use]
    pub fn is_portable(&self) -> bool {

        if self.required_environments < 2
            || self.required_test_cases == 0
        {
            return false;
        }

        let expected_executions =
            self.required_environments
                .checked_mul(self.required_test_cases);

        let Some(expected_executions) = expected_executions else {
            return false;
        };

        if self.executions.len() != expected_executions {
            return false;
        }

        if self.executions.iter().any(|record| {
            !record.status().is_completed()
                || record.observation().is_none()
        }) {
            return false;
        }

        self.findings.iter().all(|finding| {
            finding.comparison()
                == PortabilityComparison::Equivalent
        }) && self.findings.len()
            == self.expected_comparison_count()
    }

    /// Returns whether any comparison established a semantic difference.
    #[must_use]
    pub fn has_semantic_differences(&self) -> bool {
        self.different_count() > 0
    }

    /// Returns whether evidence is incomplete.
    #[must_use]
    pub fn has_incomplete_evidence(&self) -> bool {

        self.executions.iter().any(|record| {
            !record.status().is_completed()
        }) || self.findings.iter().any(|finding| {
            finding.comparison()
                == PortabilityComparison::Inconclusive
        })
    }

    /// Returns the expected number of pairwise comparisons.
    ///
    /// The calculation uses checked arithmetic.
    #[must_use]
    pub fn expected_comparison_count(&self) -> usize {

        self.required_environments
            .checked_mul(
                self.required_environments.saturating_sub(1)
            )
            .and_then(|value| value.checked_div(2))
            .and_then(|pairs| {
                pairs.checked_mul(self.required_test_cases)
            })
            .unwrap_or(usize::MAX)
    }

    /// Validates report internal consistency.
    ///
    /// # Errors
    ///
    /// Returns an error when report records contain inconsistent
    /// identities or duplicated execution entries.
    pub fn validate(&self) -> PortabilityResult<()> {

        let mut executions = std::collections::BTreeSet::new();

        for record in &self.executions {

            if !executions.insert((
                record.environment().clone(),
                record.case_id().clone(),
            )) {
                return Err(
                    PortabilityError::invalid_report(
                        "duplicate environment/test-case execution",
                    )
                );
            }

            if record.status().is_completed()
                != record.observation()
                    .map(|observation| {
                        observation.status().is_completed()
                    })
                    .unwrap_or(false)
            {
                return Err(
                    PortabilityError::invalid_report(
                        "execution record and observation status disagree",
                    )
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// PORTABILITY TESTER
// ============================================================================

/// Reusable portability testing engine.
///
/// This component does not select targets or providers.
///
/// Environments and execution adapters are explicitly supplied by
/// the caller.
///
/// Comparisons are deterministic.
///
/// The tester does not own provider lifecycle.
#[derive(Debug, Default)]
pub struct PortabilityTester {

    environments: BTreeMap<
        PortabilityEnvironmentId,
        PortabilityEnvironment,
    >,
}

impl PortabilityTester {

    /// Creates an empty portability tester.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers an execution environment.
    ///
    /// Duplicate identities are rejected.
    pub fn register_environment(
        &mut self,
        environment: PortabilityEnvironment,
    ) -> PortabilityResult<()> {

        let id = environment.id().clone();

        if self.environments.contains_key(&id) {

            return Err(
                PortabilityError::DuplicateEnvironment {
                    environment: id.to_string(),
                }
            );
        }

        self.environments.insert(id, environment);

        Ok(())
    }

    /// Returns registered environments in deterministic order.
    #[must_use]
    pub fn environments(
        &self,
    ) -> impl Iterator<Item = &PortabilityEnvironment> {

        self.environments.values()
    }

    /// Returns the number of registered environments.
    #[must_use]
    pub fn environment_count(&self) -> usize {
        self.environments.len()
    }

    /// Runs one portability test case against explicitly supplied
    /// execution adapters.
    ///
    /// The caller must supply an adapter for every registered
    /// environment.
    ///
    /// No environment is automatically selected.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - fewer than two environments are registered;
    /// - the test case collection is empty;
    /// - an adapter is missing;
    /// - duplicate test case identities are supplied;
    /// - a comparison policy identity is invalid.
    ///
    /// Individual execution failures are preserved in the report.
    pub fn run<P>(
        &self,
        test_cases: &[PortabilityTestCase],
        adapters: &BTreeMap<
            PortabilityEnvironmentId,
            &dyn PortabilityExecutionAdapter,
        >,
        policy: &P,
        cancellation: &dyn PortabilityCancellation,
    ) -> PortabilityResult<PortabilityReport>
    where
        P: PortabilityComparisonPolicy + ?Sized,
    {

        if self.environments.len() < 2 {

            return Err(
                PortabilityError::InsufficientEvidence {
                    reason: String::from(
                        "at least two distinct execution environments \
                         are required",
                    ),
                }
            );
        }

        if test_cases.is_empty() {

            return Err(
                PortabilityError::InsufficientEvidence {
                    reason: String::from(
                        "at least one portability test case is required",
                    ),
                }
            );
        }

        validate_portability_identity(policy.id())?;

        let mut case_ids =
            std::collections::BTreeSet::new();

        for test_case in test_cases {

            if !case_ids.insert(test_case.id().clone()) {

                return Err(
                    PortabilityError::DuplicateTestCase {
                        test_case: test_case.id().to_string(),
                    }
                );
            }
        }

        for environment in self.environments.values() {

            if !adapters.contains_key(environment.id()) {

                return Err(
                    PortabilityError::UnknownEnvironment {
                        environment: environment.id().to_string(),
                    }
                );
            }
        }

        let mut report = PortabilityReport {
            policy_identity: policy.id().to_owned(),
            executions: Vec::new(),
            findings: Vec::new(),
            required_environments: self.environments.len(),
            required_test_cases: test_cases.len(),
        };

        // --------------------------------------------------------------------
        // EXECUTION
        // --------------------------------------------------------------------

        for test_case in test_cases {

            let mut observations = BTreeMap::<
                PortabilityEnvironmentId,
                PortabilityObservation,
            >::new();

            for environment in self.environments.values() {

                if cancellation.is_cancelled() {

                    report.executions.push(
                        PortabilityExecutionRecord {
                            environment: environment.id().clone(),
                            case_id: test_case.id().clone(),
                            mode: environment.mode(),
                            observation: None,
                            status: PortabilityExecutionStatus::Cancelled,
                        }
                    );

                    continue;
                }

                let adapter = adapters
                    .get(environment.id())
                    .ok_or_else(|| {
                        PortabilityError::UnknownEnvironment {
                            environment: environment.id().to_string(),
                        }
                    })?;

                let request = PortabilityExecutionRequest::new(
                    test_case,
                    environment.mode(),
                );

                // Catch adapter panics when unwinding is enabled.
                //
                // A panic is never converted into successful execution.

                let execution = catch_unwind(
                    AssertUnwindSafe(|| {
                        adapter.execute(&request)
                    })
                );

                let observation = match execution {

                    Ok(Ok(observation)) => observation,

                    Ok(Err(error)) => {

                        let _ = error;

                        report.executions.push(
                            PortabilityExecutionRecord {
                                environment: environment.id().clone(),
                                case_id: test_case.id().clone(),
                                mode: environment.mode(),
                                observation: None,
                                status: PortabilityExecutionStatus::Failed,
                            }
                        );

                        continue;
                    }

                    Err(_) => {

                        report.executions.push(
                            PortabilityExecutionRecord {
                                environment: environment.id().clone(),
                                case_id: test_case.id().clone(),
                                mode: environment.mode(),
                                observation: None,
                                status: PortabilityExecutionStatus::Failed,
                            }
                        );

                        continue;
                    }
                };

                let status = observation.status();

                if status.is_completed() {

                    observations.insert(
                        environment.id().clone(),
                        observation.clone(),
                    );
                }

                report.executions.push(
                    PortabilityExecutionRecord {
                        environment: environment.id().clone(),
                        case_id: test_case.id().clone(),
                        mode: environment.mode(),
                        observation: Some(observation),
                        status,
                    }
                );
            }

            // ----------------------------------------------------------------
            // SEMANTIC COMPARISON
            // ----------------------------------------------------------------

            let completed = observations.iter();

            let mut completed_iter = completed;

            let mut previous = completed_iter.next();

            while let Some((right_id, right_observation)) =
                completed_iter.next()
            {

                let Some((left_id, left_observation)) = previous else {
                    previous = Some((right_id, right_observation));
                    continue;
                };

                let comparison = catch_unwind(
                    AssertUnwindSafe(|| {
                        policy.compare(
                            left_observation,
                            right_observation,
                        )
                    })
                );

                let (result, message) = match comparison {

                    Ok(Ok(result)) => {

                        let message = match result {

                            PortabilityComparison::Equivalent =>
                                "observations satisfy the declared \
                                 semantic equivalence policy",

                            PortabilityComparison::Different =>
                                "observations differ under the declared \
                                 semantic equivalence policy",

                            PortabilityComparison::Inconclusive =>
                                "available evidence cannot establish \
                                 semantic equivalence",
                        };

                        (result, message.to_owned())
                    }

                    Ok(Err(error)) => {

                        let _ = error;

                        (
                            PortabilityComparison::Inconclusive,
                            String::from(
                                "comparison policy could not establish \
                                 a trustworthy result",
                            ),
                        )
                    }

                    Err(_) => {

                        (
                            PortabilityComparison::Inconclusive,
                            String::from(
                                "comparison policy terminated unexpectedly",
                            ),
                        )
                    }
                };

                report.findings.push(
                    PortabilityFinding {
                        case_id: test_case.id().clone(),
                        left: left_id.clone(),
                        right: right_id.clone(),
                        comparison: result,
                        message,
                    }
                );

                previous = Some((right_id, right_observation));
            }

            // The adjacent comparison chain is supplemented by the
            // complete pairwise comparison pass below when required.
            //
            // The report must contain every unordered pair.
        }

        // --------------------------------------------------------------------
        // COMPLETE PAIRWISE COMPARISON
        // --------------------------------------------------------------------

        // The previous loop deliberately records adjacent comparisons
        // during streaming traversal. Rebuild the complete deterministic
        // pair set from the actual execution records so every environment
        // pair is independently evaluated.

        report.findings.clear();

        for test_case in test_cases {

            let completed: Vec<_> = report.executions
                .iter()
                .filter(|record| {
                    record.case_id() == test_case.id()
                        && record.status().is_completed()
                })
                .filter_map(|record| {
                    record.observation().map(|observation| {
                        (
                            record.environment().clone(),
                            observation,
                        )
                    })
                })
                .collect();

            for left_index in 0..completed.len() {

                for right_index in (left_index + 1)..completed.len() {

                    let (left_id, left_observation) =
                        &completed[left_index];

                    let (right_id, right_observation) =
                        &completed[right_index];

                    let comparison = catch_unwind(
                        AssertUnwindSafe(|| {
                            policy.compare(
                                left_observation,
                                right_observation,
                            )
                        })
                    );

                    let (result, message) = match comparison {

                        Ok(Ok(result)) => {

                            let message = match result {

                                PortabilityComparison::Equivalent =>
                                    "observations are equivalent",

                                PortabilityComparison::Different =>
                                    "observations are different",

                                PortabilityComparison::Inconclusive =>
                                    "comparison is inconclusive",
                            };

                            (result, message.to_owned())
                        }

                        Ok(Err(_)) => (
                            PortabilityComparison::Inconclusive,
                            String::from(
                                "comparison policy returned an error",
                            ),
                        ),

                        Err(_) => (
                            PortabilityComparison::Inconclusive,
                            String::from(
                                "comparison policy panicked",
                            ),
                        ),
                    };

                    report.findings.push(
                        PortabilityFinding {
                            case_id: test_case.id().clone(),
                            left: left_id.clone(),
                            right: right_id.clone(),
                            comparison: result,
                            message,
                        }
                    );
                }
            }
        }

        report.validate()?;

        Ok(report)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    struct StaticAdapter {
        output: Vec<u8>,
        status: PortabilityExecutionStatus,
    }

    impl PortabilityExecutionAdapter for StaticAdapter {

        fn execute(
            &self,
            _request: &PortabilityExecutionRequest,
        ) -> PortabilityResult<PortabilityObservation> {

            PortabilityObservation::new(
                self.status,
                self.output.clone(),
                None,
                None,
            )
        }
    }

    struct FailingAdapter;

    impl PortabilityExecutionAdapter for FailingAdapter {

        fn execute(
            &self,
            _request: &PortabilityExecutionRequest,
        ) -> PortabilityResult<PortabilityObservation> {

            Err(
                PortabilityError::invalid_identity(
                    "test execution failed",
                )
            )
        }
    }

    struct PanickingAdapter;

    impl PortabilityExecutionAdapter for PanickingAdapter {

        fn execute(
            &self,
            _request: &PortabilityExecutionRequest,
        ) -> PortabilityResult<PortabilityObservation> {

            panic!("simulated adapter failure");
        }
    }

    fn environment(
        name: &str,
    ) -> PortabilityEnvironment {

        PortabilityEnvironment::new(
            PortabilityEnvironmentId::new(name).unwrap(),
            PortabilityExecutionMode::RuntimeFree,
        )
    }

    fn test_case() -> PortabilityTestCase {

        PortabilityTestCase::new(
            PortabilityTestCaseId::new(
                "classical.basic",
            )
            .unwrap(),
            "program.example",
            "semantic.contract.v1",
            vec![1, 2, 3],
        )
        .unwrap()
    }

    #[test]
    fn environment_identity_is_open_world() {

        let id = PortabilityEnvironmentId::new(
            "future.photonics.execution",
        )
        .unwrap();

        assert_eq!(
            id.as_str(),
            "future.photonics.execution",
        );
    }

    #[test]
    fn invalid_environment_identity_is_rejected() {

        assert!(
            PortabilityEnvironmentId::new("").is_err()
        );

        assert!(
            PortabilityEnvironmentId::new(".provider").is_err()
        );

        assert!(
            PortabilityEnvironmentId::new("provider.").is_err()
        );

        assert!(
            PortabilityEnvironmentId::new("provider..execution").is_err()
        );
    }

    #[test]
    fn environment_metadata_is_extensible() {

        let environment = environment(
            "classical.local",
        )
        .with_attribute(
            "platform.architecture",
            "test-architecture",
        )
        .unwrap();

        assert_eq!(
            environment.attribute("platform.architecture"),
            Some("test-architecture"),
        );
    }

    #[test]
    fn duplicate_environment_attribute_is_rejected() {

        let result = environment(
            "classical.local",
        )
        .with_attribute(
            "platform.architecture",
            "first",
        )
        .unwrap()
        .with_attribute(
            "platform.architecture",
            "second",
        );

        assert!(result.is_err());
    }

    #[test]
    fn empty_tester_does_not_claim_portability() {

        let tester = PortabilityTester::new();

        assert_eq!(
            tester.environment_count(),
            0,
        );

        assert!(
            !PortabilityReport::new().is_portable()
        );
    }

    #[test]
    fn duplicate_environment_registration_is_rejected() {

        let mut tester = PortabilityTester::new();

        tester.register_environment(
            environment("classical.local"),
        ).unwrap();

        assert!(
            tester.register_environment(
                environment("classical.local"),
            ).is_err()
        );
    }

    #[test]
    fn exact_comparison_accepts_equal_outputs() {

        let left = PortabilityObservation::new(
            PortabilityExecutionStatus::Completed,
            vec![1, 2, 3],
            None,
            None,
        ).unwrap();

        let right = left.clone();

        let result = ExactOutputComparison.compare(
            &left,
            &right,
        ).unwrap();

        assert_eq!(
            result,
            PortabilityComparison::Equivalent,
        );
    }

    #[test]
    fn exact_comparison_rejects_different_outputs() {

        let left = PortabilityObservation::new(
            PortabilityExecutionStatus::Completed,
            vec![1],
            None,
            None,
        ).unwrap();

        let right = PortabilityObservation::new(
            PortabilityExecutionStatus::Completed,
            vec![2],
            None,
            None,
        ).unwrap();

        let result = ExactOutputComparison.compare(
            &left,
            &right,
        ).unwrap();

        assert_eq!(
            result,
            PortabilityComparison::Different,
        );
    }

    #[test]
    fn unsupported_execution_is_not_comparable() {

        let unsupported = PortabilityObservation::new(
            PortabilityExecutionStatus::Unsupported,
            Vec::new(),
            Some(String::from("not supported")),
            None,
        ).unwrap();

        let completed = PortabilityObservation::new(
            PortabilityExecutionStatus::Completed,
            vec![1],
            None,
            None,
        ).unwrap();

        assert_eq!(
            ExactOutputComparison.compare(
                &unsupported,
                &completed,
            ).unwrap(),
            PortabilityComparison::Inconclusive,
        );
    }

    #[test]
    fn successful_cross_environment_execution_establishes_portability() {

        let mut tester = PortabilityTester::new();

        let first = environment("classical.local");
        let second = environment("classical.embedded");

        tester.register_environment(first.clone()).unwrap();
        tester.register_environment(second.clone()).unwrap();

        let first_adapter = StaticAdapter {
            output: vec![1, 2, 3],
            status: PortabilityExecutionStatus::Completed,
        };

        let second_adapter = StaticAdapter {
            output: vec![1, 2, 3],
            status: PortabilityExecutionStatus::Completed,
        };

        let mut adapters = BTreeMap::new();

        adapters.insert(
            first.id().clone(),
            &first_adapter as &dyn PortabilityExecutionAdapter,
        );

        adapters.insert(
            second.id().clone(),
            &second_adapter as &dyn PortabilityExecutionAdapter,
        );

        let report = tester.run(
            &[test_case()],
            &adapters,
            &ExactOutputComparison,
            &NeverCancelled,
        ).unwrap();

        assert!(report.is_portable());

        assert_eq!(
            report.completed_execution_count(),
            2,
        );

        assert_eq!(
            report.equivalent_count(),
            1,
        );
    }

    #[test]
    fn different_outputs_do_not_establish_portability() {

        let mut tester = PortabilityTester::new();

        let first = environment("classical.local");
        let second = environment("classical.embedded");

        tester.register_environment(first.clone()).unwrap();
        tester.register_environment(second.clone()).unwrap();

        let first_adapter = StaticAdapter {
            output: vec![1],
            status: PortabilityExecutionStatus::Completed,
        };

        let second_adapter = StaticAdapter {
            output: vec![2],
            status: PortabilityExecutionStatus::Completed,
        };

        let mut adapters = BTreeMap::new();

        adapters.insert(
            first.id().clone(),
            &first_adapter as &dyn PortabilityExecutionAdapter,
        );

        adapters.insert(
            second.id().clone(),
            &second_adapter as &dyn PortabilityExecutionAdapter,
        );

        let report = tester.run(
            &[test_case()],
            &adapters,
            &ExactOutputComparison,
            &NeverCancelled,
        ).unwrap();

        assert!(!report.is_portable());

        assert_eq!(
            report.different_count(),
            1,
        );
    }

    #[test]
    fn failed_execution_does_not_claim_portability() {

        let mut tester = PortabilityTester::new();

        let first = environment("classical.local");
        let second = environment("classical.embedded");

        tester.register_environment(first.clone()).unwrap();
        tester.register_environment(second.clone()).unwrap();

        let first_adapter = StaticAdapter {
            output: vec![1],
            status: PortabilityExecutionStatus::Completed,
        };

        let second_adapter = FailingAdapter;

        let mut adapters = BTreeMap::new();

        adapters.insert(
            first.id().clone(),
            &first_adapter as &dyn PortabilityExecutionAdapter,
        );

        adapters.insert(
            second.id().clone(),
            &second_adapter as &dyn PortabilityExecutionAdapter,
        );

        let report = tester.run(
            &[test_case()],
            &adapters,
            &ExactOutputComparison,
            &NeverCancelled,
        ).unwrap();

        assert!(!report.is_portable());

        assert_eq!(
            report.failed_execution_count(),
            1,
        );
    }

    #[test]
    fn adapter_panic_does_not_claim_success() {

        let mut tester = PortabilityTester::new();

        let first = environment("classical.local");
        let second = environment("classical.embedded");

        tester.register_environment(first.clone()).unwrap();
        tester.register_environment(second.clone()).unwrap();

        let first_adapter = StaticAdapter {
            output: vec![1],
            status: PortabilityExecutionStatus::Completed,
        };

        let second_adapter = PanickingAdapter;

        let mut adapters = BTreeMap::new();

        adapters.insert(
            first.id().clone(),
            &first_adapter as &dyn PortabilityExecutionAdapter,
        );

        adapters.insert(
            second.id().clone(),
            &second_adapter as &dyn PortabilityExecutionAdapter,
        );

        let report = tester.run(
            &[test_case()],
            &adapters,
            &ExactOutputComparison,
            &NeverCancelled,
        ).unwrap();

        assert!(!report.is_portable());

        assert_eq!(
            report.failed_execution_count(),
            1,
        );
    }

    #[test]
    fn cancellation_does_not_claim_portability() {

        struct CancelImmediately;

        impl PortabilityCancellation for CancelImmediately {

            fn is_cancelled(&self) -> bool {
                true
            }
        }

        let mut tester = PortabilityTester::new();

        let first = environment("classical.local");
        let second = environment("classical.embedded");

        tester.register_environment(first.clone()).unwrap();
        tester.register_environment(second.clone()).unwrap();

        let first_adapter = StaticAdapter {
            output: vec![1],
            status: PortabilityExecutionStatus::Completed,
        };

        let second_adapter = StaticAdapter {
            output: vec![1],
            status: PortabilityExecutionStatus::Completed,
        };

        let mut adapters = BTreeMap::new();

        adapters.insert(
            first.id().clone(),
            &first_adapter as &dyn PortabilityExecutionAdapter,
        );

        adapters.insert(
            second.id().clone(),
            &second_adapter as &dyn PortabilityExecutionAdapter,
        );

        let report = tester.run(
            &[test_case()],
            &adapters,
            &ExactOutputComparison,
            &CancelImmediately,
        ).unwrap();

        assert!(!report.is_portable());

        assert_eq!(
            report.cancelled_execution_count(),
            2,
        );
    }

    #[test]
    fn multiple_environment_comparisons_are_pairwise() {

        let mut tester = PortabilityTester::new();

        let environments = [
            environment("classical.local"),
            environment("classical.embedded"),
            environment("classical.server"),
        ];

        let adapters_owned = [
            StaticAdapter {
                output: vec![1],
                status: PortabilityExecutionStatus::Completed,
            },
            StaticAdapter {
                output: vec![1],
                status: PortabilityExecutionStatus::Completed,
            },
            StaticAdapter {
                output: vec![1],
                status: PortabilityExecutionStatus::Completed,
            },
        ];

        let mut adapters = BTreeMap::new();

        for (environment, adapter) in
            environments.iter().zip(adapters_owned.iter())
        {
            tester.register_environment(
                environment.clone(),
            ).unwrap();

            adapters.insert(
                environment.id().clone(),
                adapter as &dyn PortabilityExecutionAdapter,
            );
        }

        let report = tester.run(
            &[test_case()],
            &adapters,
            &ExactOutputComparison,
            &NeverCancelled,
        ).unwrap();

        assert_eq!(
            report.findings().len(),
            3,
        );

        assert_eq!(
            report.equivalent_count(),
            3,
        );

        assert!(report.is_portable());
    }

    #[test]
    fn different_execution_modes_can_be_compared() {

        let mut tester = PortabilityTester::new();

        let first = PortabilityEnvironment::new(
            PortabilityEnvironmentId::new(
                "classical.native",
            ).unwrap(),
            PortabilityExecutionMode::RuntimeFree,
        );

        let second = PortabilityEnvironment::new(
            PortabilityEnvironmentId::new(
                "classical.managed",
            ).unwrap(),
            PortabilityExecutionMode::RuntimeEnabled,
        );

        tester.register_environment(first.clone()).unwrap();
        tester.register_environment(second.clone()).unwrap();

        let first_adapter = StaticAdapter {
            output: vec![1],
            status: PortabilityExecutionStatus::Completed,
        };

        let second_adapter = StaticAdapter {
            output: vec![1],
            status: PortabilityExecutionStatus::Completed,
        };

        let mut adapters = BTreeMap::new();

        adapters.insert(
            first.id().clone(),
            &first_adapter as &dyn PortabilityExecutionAdapter,
        );

        adapters.insert(
            second.id().clone(),
            &second_adapter as &dyn PortabilityExecutionAdapter,
        );

        let report = tester.run(
            &[test_case()],
            &adapters,
            &ExactOutputComparison,
            &NeverCancelled,
        ).unwrap();

        assert!(report.is_portable());
    }

    #[test]
    fn report_validation_rejects_nothing_for_empty_report() {

        let report = PortabilityReport::new();

        assert!(report.validate().is_ok());

        assert!(!report.is_portable());
    }

    #[test]
    fn execution_modes_are_not_domain_identifiers() {

        assert_ne!(
            PortabilityExecutionMode::RuntimeFree,
            PortabilityExecutionMode::RuntimeEnabled,
        );

        let future_domain =
            PortabilityEnvironmentId::new(
                "future.photonics.execution",
            );

        assert!(future_domain.is_ok());
    }
}
