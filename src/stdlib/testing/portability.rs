
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Portability Testing
//!
//! Path: `src/stdlib/testing/portability.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides reusable, target-independent portability testing for
//! Zamani programs and computational operations.
//!
//! Portability testing verifies that independently supplied execution
//! environments preserve the observable semantics of the same program.
//!
//! Supported testing domains are open-ended and include:
//!
//! - Classical computation.
//! - Quantum computation.
//! - HDL and hardware-description computation.
//! - Classical/quantum hybrid computation.
//! - Distributed computation.
//! - AI and machine learning.
//! - Scientific computation.
//! - Embedded computation.
//! - Networking.
//! - Cryptography.
//! - Future computational domains.
//!
//! ## SEMANTIC PRINCIPLE
//!
//! Portability means semantic preservation across compatible realizations.
//!
//! It does not mean that every program can execute on every environment.
//!
//! An unsupported environment MUST remain distinguishable from a
//! successful execution.
//!
//! ## OWNS
//!
//! - Execution-environment descriptions.
//! - Portability test registration.
//! - Execution comparison policies.
//! - Canonical observation comparison.
//! - Cross-environment execution records.
//! - Portability reports.
//! - Portability statistics.
//! - Portability acceptance decisions.
//!
//! ## DOES NOT OWN
//!
//! - Hardware discovery.
//! - Hardware allocation.
//! - Provider registration.
//! - Provider selection.
//! - Runtime scheduling.
//! - Compiler lowering.
//! - Optimization.
//! - AST definitions.
//! - Grammar definitions.
//! - Semantic analysis.
//! - Classical IR.
//! - Quantum IR.
//! - HDLIR.
//! - Distributed execution.
//! - Physical hardware control.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ## INTEGRATION
//!
//! Conformance infrastructure:
//!
//!     crate::stdlib::testing::conformance
//!
//! Canonical grammar:
//!
//!     grammar/Zamani.g4
//!
//! Normative portability specification:
//!
//!     grammar/spec/portability.md
//!
//! Human-readable portability specification:
//!
//!     grammar/specification/portability.md
//!
//! Portability validation:
//!
//!     grammar/validation/portability.md
//!
//! Semantic authority:
//!
//!     src/frontend/semantic/
//!
//! Compiler:
//!
//!     src/compiler/
//!
//! Classical computation:
//!
//!     src/classical/
//!
//! Quantum computation:
//!
//!     src/quantum/
//!
//! Hardware description:
//!
//!     src/hdl/
//!
//! Hybrid computation:
//!
//!     src/hybrid/
//!
//! Distributed computation:
//!
//!     src/distributed/
//!
//! Universal IR:
//!
//!     src/zuir/
//!
//! Runtime:
//!
//!     src/runtime/
//!
//! ## SCALABILITY
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Execution environments.
//! - Portability cases.
//! - Computational domains.
//! - Environment attributes.
//! - Observation sizes.
//! - Comparison results.
//! - Diagnostic lengths.
//! - Providers.
//! - Computational resources.
//!
//! Storage and computation remain bounded by actual available resources.
//!
//! Comparison strategy is explicit because all-pairs comparison has
//! quadratic comparison complexity.
//!
//! ## SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe code.
//! No raw pointers.
//! No global mutable state.
//! No implicit filesystem access.
//! No implicit network access.
//! No implicit hardware discovery.
//! No implicit provider selection.
//! No implicit execution scheduling.
//!
//! ## DETERMINISM
//!
//! Registration uses ordered maps.
//!
//! Execution order is deterministic.
//!
//! Reports preserve deterministic ordering.
//!
//! A custom comparison policy must itself provide deterministic results
//! if deterministic reports are required.
//!
//! ## ACCEPTANCE RULE
//!
//! A portability claim requires:
//!
//! 1. At least two tested environments.
//! 2. Successful observations from the compared environments.
//! 3. Completed semantic comparisons.
//! 4. Equivalent observations under the declared comparison policy.
//! 5. No unsupported, skipped, failed, cancelled, or inconclusive
//!    execution that invalidates the requested portability claim.
//!
//! A missing observation MUST NEVER be treated as a passing comparison.
//!
//! ## FREEZE CRITERIA
//!
//! 1. Environment identities are validated.
//! 2. Duplicate registrations are rejected.
//! 3. Exact comparison is the default.
//! 4. Custom equivalence policies are supported.
//! 5. Unsupported execution is never successful.
//! 6. Incomplete comparisons never pass.
//! 7. Cancellation is explicit.
//! 8. Panics are isolated when unwinding is enabled.
//! 9. Reports are deterministic.
//! 10. No arbitrary universal capacity ceiling exists.
//! 11. All public contracts are documented.
//! 12. Rust 1.97+ compatibility is maintained.

// =============================================================================
// IMPORTS
// =============================================================================

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};

use super::conformance::{
    ConformanceCancellation,
    ConformanceContext,
    ConformanceDomain,
    ConformanceId,
};

// =============================================================================
// IDENTIFIER VALIDATION
// =============================================================================

/// Errors encountered while constructing an environment identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnvironmentIdError {
    /// The identifier is empty.
    Empty,

    /// The identifier contains surrounding whitespace.
    SurroundingWhitespace,

    /// The identifier contains an empty qualified component.
    EmptyComponent,

    /// The identifier contains unsupported characters.
    InvalidCharacter,
}

impl fmt::Display for EnvironmentIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                formatter.write_str("environment identifier cannot be empty")
            }
            Self::SurroundingWhitespace => {
                formatter.write_str(
                    "environment identifier contains surrounding whitespace",
                )
            }
            Self::EmptyComponent => {
                formatter.write_str(
                    "environment identifier contains an empty component",
                )
            }
            Self::InvalidCharacter => {
                formatter.write_str(
                    "environment identifier contains invalid characters",
                )
            }
        }
    }
}

impl Error for EnvironmentIdError {}

fn validate_identifier(value: &str) -> Result<(), EnvironmentIdError> {
    if value.is_empty() {
        return Err(EnvironmentIdError::Empty);
    }

    if value.trim() != value {
        return Err(EnvironmentIdError::SurroundingWhitespace);
    }

    if value.split('.').any(str::is_empty) {
        return Err(EnvironmentIdError::EmptyComponent);
    }

    if value.chars().any(|character| {
        !(character.is_ascii_alphanumeric()
            || character == '_'
            || character == '-'
            || character == '.')
    }) {
        return Err(EnvironmentIdError::InvalidCharacter);
    }

    Ok(())
}

// =============================================================================
// ENVIRONMENT IDENTITY
// =============================================================================

/// Extensible identity for a tested execution environment.
///
/// The identity describes a test environment, not a physical device ID.
///
/// Examples:
///
/// - `classical.local`
— `quantum.simulator`
/// - `hardware.synthesis`
/// - `distributed.cluster`
/// - `future.execution.environment`
///
/// Identifiers are not restricted to a closed hardware enumeration.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnvironmentId(String);

impl EnvironmentId {
    /// Creates a validated environment identity.
    ///
    /// # Errors
    ///
    /// Returns an error for empty or malformed identifiers.
    pub fn new(value: impl Into<String>) -> Result<Self, EnvironmentIdError> {
        let value = value.into();

        validate_identifier(&value)?;

        Ok(Self(value))
    }

    /// Returns the environment identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EnvironmentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// ENVIRONMENT DESCRIPTION
// =============================================================================

/// Descriptive information about an execution environment.
///
/// The structure deliberately does not perform environment discovery.
///
/// Its fields describe information supplied by an external authority.
///
/// Environment attributes are extensible and may describe:
///
/// - Implementation identity.
/// - Runtime version.
/// - Compiler version.
/// - Grammar version.
/// - Semantic specification version.
/// - Architecture properties.
/// - Execution capabilities.
/// - Deployment characteristics.
///
/// Hardware-specific information must remain descriptive metadata,
/// not a universal assumption imposed on portable source semantics.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionEnvironment {
    id: EnvironmentId,
    implementation: String,
    attributes: BTreeMap<String, String>,
}

impl ExecutionEnvironment {
    /// Creates an execution-environment description.
    pub fn new(
        id: EnvironmentId,
        implementation: impl Into<String>,
    ) -> Self {
        Self {
            id,
            implementation: implementation.into(),
            attributes: BTreeMap::new(),
        }
    }

    /// Returns the environment identity.
    #[must_use]
    pub fn id(&self) -> &EnvironmentId {
        &self.id
    }

    /// Returns the implementation description.
    #[must_use]
    pub fn implementation(&self) -> &str {
        &self.implementation
    }

    /// Adds an extensible descriptive attribute.
    ///
    /// Attribute names use the same qualified identifier convention
    /// as environment identities.
    ///
    /// Existing values are replaced deliberately.
    ///
    /// # Errors
    ///
    /// Returns an error when the attribute name is malformed.
    pub fn with_attribute(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, EnvironmentIdError> {
        let name = name.into();

        validate_identifier(&name)?;

        self.attributes.insert(name, value.into());

        Ok(self)
    }

    /// Returns all descriptive attributes.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }

    /// Returns an attribute by name.
    #[must_use]
    pub fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(String::as_str)
    }
}

// =============================================================================
// PORTABILITY OBSERVATION
// =============================================================================

/// Canonical observable result produced by an execution environment.
///
/// The bytes must represent the semantic observation being compared.
///
/// Callers are responsible for producing canonical representations.
///
/// Examples:
///
/// - Canonical serialized classical values.
/// - Canonical quantum measurement distributions.
/// - HDL verification results.
/// - Hybrid computation outputs.
/// - Distributed consistency observations.
/// - Scientific numerical results.
///
/// Hardware addresses, timestamps, provider identifiers, and other
/// incidental metadata should not be included unless they are
/// semantically observable under the tested contract.
///
/// Exact byte equality is used by the default comparison policy.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PortabilityObservation {
    bytes: Vec<u8>,
}

impl PortabilityObservation {
    /// Creates an observation from canonical bytes.
    #[must_use]
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            bytes: bytes.into(),
        }
    }

    /// Creates an observation from UTF-8 text.
    #[must_use]
    pub fn from_text(value: impl Into<String>) -> Self {
        Self::new(value.into().into_bytes())
    }

    /// Returns the canonical observation bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns whether the observation is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns the observation length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
}

// =============================================================================
// EXECUTION RESULTS
// =============================================================================

/// Result of executing a portability test on one environment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PortabilityExecution {
    /// Execution completed and produced an observation.
    Observed(PortabilityObservation),

    /// The required operation is unsupported.
    Unsupported {
        /// Explanation of the unsupported operation.
        reason: String,
    },

    /// Execution could not establish a definitive observation.
    Inconclusive {
        /// Explanation of incomplete verification.
        reason: String,
    },

    /// The test was deliberately skipped.
    Skipped {
        /// Explanation of the skip.
        reason: String,
    },

    /// The execution failed.
    Failed {
        /// Explanation of the failure.
        reason: String,
    },

    /// Execution was cancelled.
    Cancelled,

    /// An infrastructure error occurred.
    InfrastructureError {
        /// Explanation of the infrastructure failure.
        reason: String,
    },
}

impl PortabilityExecution {
    /// Creates a successful observed execution.
    #[must_use]
    pub fn observed(observation: PortabilityObservation) -> Self {
        Self::Observed(observation)
    }

    /// Creates an unsupported result.
    #[must_use]
    pub fn unsupported(reason: impl Into<String>) -> Self {
        Self::Unsupported {
            reason: reason.into(),
        }
    }

    /// Creates an inconclusive result.
    #[must_use]
    pub fn inconclusive(reason: impl Into<String>) -> Self {
        Self::Inconclusive {
            reason: reason.into(),
        }
    }

    /// Creates a skipped result.
    #[must_use]
    pub fn skipped(reason: impl Into<String>) -> Self {
        Self::Skipped {
            reason: reason.into(),
        }
    }

    /// Creates a failed result.
    #[must_use]
    pub fn failed(reason: impl Into<String>) -> Self {
        Self::Failed {
            reason: reason.into(),
        }
    }

    /// Creates an infrastructure-error result.
    #[must_use]
    pub fn infrastructure_error(reason: impl Into<String>) -> Self {
        Self::InfrastructureError {
            reason: reason.into(),
        }
    }

    /// Returns whether an actual observation was produced.
    #[must_use]
    pub const fn has_observation(&self) -> bool {
        matches!(self, Self::Observed(_))
    }

    /// Returns whether execution produced a successful observation.
    #[must_use]
    pub const fn is_successful(&self) -> bool {
        self.has_observation()
    }

    /// Returns the observation when execution succeeded.
    #[must_use]
    pub fn observation(&self) -> Option<&PortabilityObservation> {
        match self {
            Self::Observed(observation) => Some(observation),
            _ => None,
        }
    }
}

// =============================================================================
// COMPARISON POLICY
// =============================================================================

/// Result of comparing two semantic observations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EquivalenceResult {
    /// The observations are semantically equivalent.
    Equivalent,

    /// The observations are not equivalent.
    Different {
        /// Explanation of the semantic difference.
        reason: String,
    },

    /// The comparison could not establish equivalence.
    Inconclusive {
        /// Explanation of incomplete comparison.
        reason: String,
    },
}

impl EquivalenceResult {
    /// Creates an equivalent result.
    #[must_use]
    pub const fn equivalent() -> Self {
        Self::Equivalent
    }

    /// Creates a non-equivalent result.
    #[must_use]
    pub fn different(reason: impl Into<String>) -> Self {
        Self::Different {
            reason: reason.into(),
        }
    }

    /// Creates an inconclusive result.
    #[must_use]
    pub fn inconclusive(reason: impl Into<String>) -> Self {
        Self::Inconclusive {
            reason: reason.into(),
        }
    }

    /// Returns whether equivalence was established.
    #[must_use]
    pub const fn is_equivalent(&self) -> bool {
        matches!(self, Self::Equivalent)
    }
}

/// Comparison policy for portability observations.
///
/// Implementations may define exact, numerical, probabilistic,
/// quantum-distribution, or other domain-specific equivalence.
///
/// The policy must compare semantic observations, not incidental
/// hardware implementation metadata.
pub trait EquivalencePolicy: Send + Sync {
    /// Compares two canonical observations.
    fn compare(
        &self,
        reference: &PortabilityObservation,
        candidate: &PortabilityObservation,
    ) -> EquivalenceResult;
}

/// Default exact-byte comparison policy.
#[derive(Clone, Copy, Debug, Default)]
pub struct ExactEquivalence;

impl EquivalencePolicy for ExactEquivalence {
    fn compare(
        &self,
        reference: &PortabilityObservation,
        candidate: &PortabilityObservation,
    ) -> EquivalenceResult {
        if reference == candidate {
            EquivalenceResult::Equivalent
        } else {
            EquivalenceResult::different(
                "canonical observations differ",
            )
        }
    }
}

// =============================================================================
// COMPARISON STRATEGY
// =============================================================================

/// Determines how execution environments are compared.
///
/// Reference comparison is generally preferable when the test suite
/// has a trusted reference environment.
///
/// All-pairs comparison evaluates every pair of successful observations
/// and therefore has quadratic comparison complexity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComparisonStrategy {
    /// Compare every environment against one explicitly selected reference.
    Reference(EnvironmentId),

    /// Compare every successful environment pair.
    AllPairs,
}

// =============================================================================
// PORTABILITY CASE
// =============================================================================

/// One portability test that can execute on different environments.
pub struct PortabilityCase {
    id: ConformanceId,
    domain: ConformanceDomain,
    description: String,
    execute: Box<
        dyn Fn(
                &ExecutionEnvironment,
                &ConformanceContext,
            ) -> PortabilityExecution
            + Send
            + Sync
            + 'static,
    >,
}

impl fmt::Debug for PortabilityCase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PortabilityCase")
            .field("id", &self.id)
            .field("domain", &self.domain)
            .field("description", &self.description)
            .finish_non_exhaustive()
    }
}

impl PortabilityCase {
    /// Creates a portability test case.
    ///
    /// The callback receives an environment description and the
    /// shared conformance context.
    ///
    /// It must return an explicit execution result.
    ///
    /// # Panics
    ///
    /// Panics are isolated by the suite and converted into
    /// infrastructure-error outcomes when unwinding is enabled.
    pub fn new<F>(
        id: ConformanceId,
        domain: ConformanceDomain,
        description: impl Into<String>,
        execute: F,
    ) -> Self
    where
        F: Fn(
                &ExecutionEnvironment,
                &ConformanceContext,
            ) -> PortabilityExecution
            + Send
            + Sync
            + 'static,
    {
        Self {
            id,
            domain,
            description: description.into(),
            execute: Box::new(execute),
        }
    }

    /// Returns the portability case identity.
    #[must_use]
    pub fn id(&self) -> &ConformanceId {
        &self.id
    }

    /// Returns the computational domain.
    #[must_use]
    pub fn domain(&self) -> &ConformanceDomain {
        &self.domain
    }

    /// Returns the description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

// =============================================================================
// REGISTRATION ERRORS
// =============================================================================

/// Errors encountered while constructing a portability suite.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PortabilityRegistrationError {
    /// An environment identity has already been registered.
    DuplicateEnvironment(EnvironmentId),

    /// A portability case identity has already been registered.
    DuplicateCase(ConformanceId),
}

impl fmt::Display for PortabilityRegistrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateEnvironment(id) => {
                write!(formatter, "duplicate environment: {id}")
            }
            Self::DuplicateCase(id) => {
                write!(formatter, "duplicate portability case: {id}")
            }
        }
    }
}

impl Error for PortabilityRegistrationError {}

// =============================================================================
// EXECUTION ERRORS
// =============================================================================

/// Errors encountered before portability execution can begin.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PortabilityError {
    /// No execution environments have been registered.
    NoEnvironments,

    /// No portability cases have been registered.
    NoCases,

    /// The selected reference environment is absent.
    MissingReference(EnvironmentId),
}

impl fmt::Display for PortabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoEnvironments => {
                formatter.write_str(
                    "portability suite has no execution environments",
                )
            }
            Self::NoCases => {
                formatter.write_str(
                    "portability suite has no test cases",
                )
            }
            Self::MissingReference(id) => {
                write!(
                    formatter,
                    "reference environment is not registered: {id}"
                )
            }
        }
    }
}

impl Error for PortabilityError {}

// =============================================================================
// PORTABILITY SUITE
// =============================================================================

/// Collection of portability cases and execution environments.
///
/// Registration is independent of hardware discovery.
///
/// Execution is sequential and deterministic in this implementation.
///
/// Parallel execution must be supplied by an explicit execution adapter
/// integrated with the canonical runtime.
#[derive(Default)]
pub struct PortabilitySuite {
    environments: BTreeMap<EnvironmentId, ExecutionEnvironment>,
    cases: BTreeMap<ConformanceId, PortabilityCase>,
}

impl fmt::Debug for PortabilitySuite {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PortabilitySuite")
            .field("environment_count", &self.environments.len())
            .field("case_count", &self.cases.len())
            .finish()
    }
}

impl PortabilitySuite {
    /// Creates an empty portability suite.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers an execution environment.
    ///
    /// Duplicate identities are rejected.
    pub fn register_environment(
        &mut self,
        environment: ExecutionEnvironment,
    ) -> Result<(), PortabilityRegistrationError> {
        let id = environment.id().clone();

        if self.environments.contains_key(&id) {
            return Err(
                PortabilityRegistrationError::DuplicateEnvironment(id),
            );
        }

        self.environments.insert(id, environment);

        Ok(())
    }

    /// Registers a portability test case.
    ///
    /// Duplicate identities are rejected.
    pub fn register_case(
        &mut self,
        case: PortabilityCase,
    ) -> Result<(), PortabilityRegistrationError> {
        let id = case.id().clone();

        if self.cases.contains_key(&id) {
            return Err(
                PortabilityRegistrationError::DuplicateCase(id),
            );
        }

        self.cases.insert(id, case);

        Ok(())
    }

    /// Returns the number of registered environments.
    #[must_use]
    pub fn environment_count(&self) -> usize {
        self.environments.len()
    }

    /// Returns the number of registered portability cases.
    #[must_use]
    pub fn case_count(&self) -> usize {
        self.cases.len()
    }

    /// Returns whether no environments have been registered.
    #[must_use]
    pub fn environments_are_empty(&self) -> bool {
        self.environments.is_empty()
    }

    /// Returns whether no portability cases have been registered.
    #[must_use]
    pub fn cases_are_empty(&self) -> bool {
        self.cases.is_empty()
    }

    /// Returns registered environments in deterministic order.
    pub fn environments(
        &self,
    ) -> impl Iterator<Item = &ExecutionEnvironment> {
        self.environments.values()
    }

    /// Returns registered portability cases in deterministic order.
    pub fn cases(
        &self,
    ) -> impl Iterator<Item = &PortabilityCase> {
        self.cases.values()
    }

    /// Executes the suite using exact semantic observation comparison.
    ///
    /// The selected strategy determines which environments are compared.
    ///
    /// # Errors
    ///
    /// Returns an error when required suite configuration is missing.
    pub fn run(
        &self,
        context: &ConformanceContext,
        strategy: ComparisonStrategy,
    ) -> Result<PortabilityReport, PortabilityError> {
        self.run_with_policy(
            context,
            strategy,
            ExactEquivalence,
        )
    }

    /// Executes the suite using a caller-supplied equivalence policy.
    ///
    /// This method supports numerical tolerances, probabilistic
    /// distributions, quantum measurement comparisons and other
    /// domain-specific semantic equivalence contracts.
    ///
    /// The policy must not silently weaken the program's declared
    /// correctness requirements.
    ///
    /// # Errors
    ///
    /// Returns an error when required suite configuration is missing.
    pub fn run_with_policy<P>(
        &self,
        context: &ConformanceContext,
        strategy: ComparisonStrategy,
        policy: P,
    ) -> Result<PortabilityReport, PortabilityError>
    where
        P: EquivalencePolicy,
    {
        if self.environments.is_empty() {
            return Err(PortabilityError::NoEnvironments);
        }

        if self.cases.is_empty() {
            return Err(PortabilityError::NoCases);
        }

        if let ComparisonStrategy::Reference(reference) = &strategy {
            if !self.environments.contains_key(reference) {
                return Err(
                    PortabilityError::MissingReference(reference.clone()),
                );
            }
        }

        let mut executions = Vec::new();

        // Execute each case against each supplied environment.
        //
        // Both collections are ordered maps, so execution order is
        // deterministic and independent of registration order.

        for case in self.cases.values() {
            for environment in self.environments.values() {
                let outcome = if context.is_cancelled() {
                    PortabilityExecution::Cancelled
                } else {
                    match catch_unwind(AssertUnwindSafe(|| {
                        (case.execute)(environment, context)
                    })) {
                        Ok(outcome) => outcome,

                        Err(payload) => {
                            let reason = if let Some(message) =
                                payload.downcast_ref::<String>()
                            {
                                message.clone()
                            } else if let Some(message) =
                                payload.downcast_ref::<&str>()
                            {
                                (*message).to_owned()
                            } else {
                                "portability test panicked".to_owned()
                            };

                            PortabilityExecution::infrastructure_error(reason)
                        }
                    }
                };

                executions.push(ExecutionRecord {
                    case_id: case.id.clone(),
                    domain: case.domain.clone(),
                    environment_id: environment.id().clone(),
                    description: case.description.clone(),
                    outcome,
                });
            }
        }

        let comparisons = build_comparisons(
            &executions,
            &strategy,
            &policy,
        );

        Ok(PortabilityReport::new(
            executions,
            comparisons,
        ))
    }
}

// =============================================================================
// EXECUTION RECORD
// =============================================================================

/// Complete result of executing one portability case on one environment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionRecord {
    case_id: ConformanceId,
    domain: ConformanceDomain,
    environment_id: EnvironmentId,
    description: String,
    outcome: PortabilityExecution,
}

impl ExecutionRecord {
    /// Returns the case identity.
    #[must_use]
    pub fn case_id(&self) -> &ConformanceId {
        &self.case_id
    }

    /// Returns the computational domain.
    #[must_use]
    pub fn domain(&self) -> &ConformanceDomain {
        &self.domain
    }

    /// Returns the environment identity.
    #[must_use]
    pub fn environment_id(&self) -> &EnvironmentId {
        &self.environment_id
    }

    /// Returns the case description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the execution outcome.
    #[must_use]
    pub fn outcome(&self) -> &PortabilityExecution {
        &self.outcome
    }

    /// Returns whether execution produced a valid observation.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.outcome.is_successful()
    }
}

// =============================================================================
// COMPARISON RECORD
// =============================================================================

/// Result of comparing two execution environments for one case.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComparisonRecord {
    case_id: ConformanceId,
    reference_environment: EnvironmentId,
    candidate_environment: EnvironmentId,
    result: EquivalenceResult,
}

impl ComparisonRecord {
    /// Returns the case identity.
    #[must_use]
    pub fn case_id(&self) -> &ConformanceId {
        &self.case_id
    }

    /// Returns the reference environment.
    #[must_use]
    pub fn reference_environment(&self) -> &EnvironmentId {
        &self.reference_environment
    }

    /// Returns the candidate environment.
    #[must_use]
    pub fn candidate_environment(&self) -> &EnvironmentId {
        &self.candidate_environment
    }

    /// Returns the equivalence result.
    #[must_use]
    pub fn result(&self) -> &EquivalenceResult {
        &self.result
    }

    /// Returns whether semantic equivalence was established.
    #[must_use]
    pub fn is_equivalent(&self) -> bool {
        self.result.is_equivalent()
    }
}

// =============================================================================
// COMPARISON CONSTRUCTION
// =============================================================================

fn build_comparisons<P: EquivalencePolicy>(
    executions: &[ExecutionRecord],
    strategy: &ComparisonStrategy,
    policy: &P,
) -> Vec<ComparisonRecord> {
    let mut comparisons = Vec::new();

    // Group execution records by case identity.
    //
    // The original execution ordering is deterministic.

    let mut by_case: BTreeMap<
        ConformanceId,
        Vec<&ExecutionRecord>,
    > = BTreeMap::new();

    for execution in executions {
        by_case
            .entry(execution.case_id.clone())
            .or_default()
            .push(execution);
    }

    for (case_id, records) in by_case {
        match strategy {
            ComparisonStrategy::Reference(reference_id) => {
                let reference = records
                    .iter()
                    .find(|record| {
                        record.environment_id == *reference_id
                    });

                let Some(reference) = reference else {
                    continue;
                };

                for candidate in &records {
                    if candidate.environment_id == *reference_id {
                        continue;
                    }

                    let result = match (
                        reference.outcome.observation(),
                        candidate.outcome.observation(),
                    ) {
                        (Some(expected), Some(actual)) => {
                            policy.compare(expected, actual)
                        }

                        (None, _) => {
                            EquivalenceResult::inconclusive(
                                format!(
                                    "reference environment `{}` produced no observation",
                                    reference_id
                                ),
                            )
                        }

                        (_, None) => {
                            EquivalenceResult::inconclusive(
                                format!(
                                    "candidate environment `{}` produced no observation",
                                    candidate.environment_id
                                ),
                            )
                        }
                    };

                    comparisons.push(ComparisonRecord {
                        case_id: case_id.clone(),
                        reference_environment: reference_id.clone(),
                        candidate_environment: candidate.environment_id.clone(),
                        result,
                    });
                }
            }

            ComparisonStrategy::AllPairs => {
                for (index, reference) in records.iter().enumerate() {
                    for candidate in records.iter().skip(index + 1) {
                        let result = match (
                            reference.outcome.observation(),
                            candidate.outcome.observation(),
                        ) {
                            (Some(expected), Some(actual)) => {
                                policy.compare(expected, actual)
                            }

                            (None, _) => {
                                EquivalenceResult::inconclusive(
                                    format!(
                                        "environment `{}` produced no observation",
                                        reference.environment_id
                                    ),
                                )
                            }

                            (_, None) => {
                                EquivalenceResult::inconclusive(
                                    format!(
                                        "environment `{}` produced no observation",
                                        candidate.environment_id
                                    ),
                                )
                            }
                        };

                        comparisons.push(ComparisonRecord {
                            case_id: case_id.clone(),
                            reference_environment: reference.environment_id.clone(),
                            candidate_environment: candidate.environment_id.clone(),
                            result,
                        });
                    }
                }
            }
        }
    }

    comparisons
}

// =============================================================================
// STATISTICS
// =============================================================================

/// Aggregate portability execution statistics.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PortabilityStatistics {
    /// Number of executions.
    pub executions: usize,

    /// Number of executions producing observations.
    pub observed: usize,

    /// Number of unsupported executions.
    pub unsupported: usize,

    /// Number of inconclusive executions.
    pub inconclusive: usize,

    /// Number of skipped executions.
    pub skipped: usize,

    /// Number of failed executions.
    pub failed: usize,

    /// Number of cancelled executions.
    pub cancelled: usize,

    /// Number of infrastructure errors.
    pub infrastructure_errors: usize,

    /// Number of semantic comparisons.
    pub comparisons: usize,

    /// Number of equivalent comparisons.
    pub equivalent: usize,

    /// Number of different observations.
    pub different: usize,

    /// Number of inconclusive comparisons.
    pub comparison_inconclusive: usize,
}

impl PortabilityStatistics {
    /// Returns whether every execution produced an observation.
    #[must_use]
    pub fn all_executions_observed(&self) -> bool {
        self.executions > 0
            && self.observed == self.executions
    }

    /// Returns whether every comparison established equivalence.
    #[must_use]
    pub fn all_comparisons_equivalent(&self) -> bool {
        self.comparisons > 0
            && self.equivalent == self.comparisons
    }

    /// Returns the number of non-successful executions.
    #[must_use]
    pub fn non_successful_executions(&self) -> usize {
        self.executions.saturating_sub(self.observed)
    }

    /// Returns whether all comparisons and executions are complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.executions > 0
            && self.inconclusive == 0
            && self.cancelled == 0
            && self.infrastructure_errors == 0
            && self.comparison_inconclusive == 0
    }
}

// =============================================================================
// PORTABILITY REPORT
// =============================================================================

/// Complete report of cross-environment portability testing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PortabilityReport {
    executions: Vec<ExecutionRecord>,
    comparisons: Vec<ComparisonRecord>,
    statistics: PortabilityStatistics,
}

impl PortabilityReport {
    fn new(
        executions: Vec<ExecutionRecord>,
        comparisons: Vec<ComparisonRecord>,
    ) -> Self {
        let mut statistics = PortabilityStatistics::default();

        for execution in &executions {
            statistics.executions =
                statistics.executions.saturating_add(1);

            match execution.outcome {
                PortabilityExecution::Observed(_) => {
                    statistics.observed =
                        statistics.observed.saturating_add(1);
                }

                PortabilityExecution::Unsupported { .. } => {
                    statistics.unsupported =
                        statistics.unsupported.saturating_add(1);
                }

                PortabilityExecution::Inconclusive { .. } => {
                    statistics.inconclusive =
                        statistics.inconclusive.saturating_add(1);
                }

                PortabilityExecution::Skipped { .. } => {
                    statistics.skipped =
                        statistics.skipped.saturating_add(1);
                }

                PortabilityExecution::Failed { .. } => {
                    statistics.failed =
                        statistics.failed.saturating_add(1);
                }

                PortabilityExecution::Cancelled => {
                    statistics.cancelled =
                        statistics.cancelled.saturating_add(1);
                }

                PortabilityExecution::InfrastructureError { .. } => {
                    statistics.infrastructure_errors =
                        statistics.infrastructure_errors.saturating_add(1);
                }
            }
        }

        for comparison in &comparisons {
            statistics.comparisons =
                statistics.comparisons.saturating_add(1);

            match comparison.result {
                EquivalenceResult::Equivalent => {
                    statistics.equivalent =
                        statistics.equivalent.saturating_add(1);
                }

                EquivalenceResult::Different { .. } => {
                    statistics.different =
                        statistics.different.saturating_add(1);
                }

                EquivalenceResult::Inconclusive { .. } => {
                    statistics.comparison_inconclusive =
                        statistics.comparison_inconclusive.saturating_add(1);
                }
            }
        }

        Self {
            executions,
            comparisons,
            statistics,
        }
    }

    /// Returns all execution records in deterministic order.
    #[must_use]
    pub fn executions(&self) -> &[ExecutionRecord] {
        &self.executions
    }

    /// Returns all comparison records in deterministic order.
    #[must_use]
    pub fn comparisons(&self) -> &[ComparisonRecord] {
        &self.comparisons
    }

    /// Returns aggregate statistics.
    #[must_use]
    pub const fn statistics(&self) -> &PortabilityStatistics {
        &self.statistics
    }

    /// Returns whether all requested portability comparisons passed.
    ///
    /// A portability claim requires at least one completed comparison.
    ///
    /// One environment alone cannot establish cross-environment
    /// portability.
    ///
    /// Unsupported, skipped, failed, cancelled, inconclusive and
    /// infrastructure-error outcomes never establish portability.
    #[must_use]
    pub fn is_portable(&self) -> bool {
        self.statistics.all_executions_observed()
            && self.statistics.all_comparisons_equivalent()
            && self.statistics.is_complete()
    }

    /// Returns whether all executions and comparisons completed.
    ///
    /// Completed failures and unsupported results remain complete
    /// reports, but they do not establish portability.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.statistics.is_complete()
    }

    /// Returns whether any execution failed.
    #[must_use]
    pub fn has_execution_failures(&self) -> bool {
        self.statistics.failed > 0
    }

    /// Returns whether any environment lacks the required capability.
    #[must_use]
    pub fn has_unsupported_executions(&self) -> bool {
        self.statistics.unsupported > 0
    }

    /// Returns whether any semantic comparison found a difference.
    #[must_use]
    pub fn has_semantic_differences(&self) -> bool {
        self.statistics.different > 0
    }

    /// Returns executions belonging to a particular case.
    #[must_use]
    pub fn case_executions(
        &self,
        id: &ConformanceId,
    ) -> Vec<&ExecutionRecord> {
        self.executions
            .iter()
            .filter(|record| record.case_id() == id)
            .collect()
    }

    /// Returns comparisons belonging to a particular case.
    #[must_use]
    pub fn case_comparisons(
        &self,
        id: &ConformanceId,
    ) -> Vec<&ComparisonRecord> {
        self.comparisons
            .iter()
            .filter(|record| record.case_id() == id)
            .collect()
    }

    /// Returns executions belonging to a particular environment.
    #[must_use]
    pub fn environment_executions(
        &self,
        id: &EnvironmentId,
    ) -> Vec<&ExecutionRecord> {
        self.executions
            .iter()
            .filter(|record| record.environment_id() == id)
            .collect()
    }
}

impl fmt::Display for PortabilityReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "Zamani Portability Report")?;

        writeln!(
            formatter,
            "Executions: {}",
            self.statistics.executions
        )?;

        writeln!(
            formatter,
            "Observed: {}",
            self.statistics.observed
        )?;

        writeln!(
            formatter,
            "Unsupported: {}",
            self.statistics.unsupported
        )?;

        writeln!(
            formatter,
            "Inconclusive: {}",
            self.statistics.inconclusive
        )?;

        writeln!(
            formatter,
            "Skipped: {}",
            self.statistics.skipped
        )?;

        writeln!(
            formatter,
            "Failed: {}",
            self.statistics.failed
        )?;

        writeln!(
            formatter,
            "Cancelled: {}",
            self.statistics.cancelled
        )?;

        writeln!(
            formatter,
            "Infrastructure errors: {}",
            self.statistics.infrastructure_errors
        )?;

        writeln!(
            formatter,
            "Comparisons: {}",
            self.statistics.comparisons
        )?;

        writeln!(
            formatter,
            "Equivalent: {}",
            self.statistics.equivalent
        )?;

        writeln!(
            formatter,
            "Different: {}",
            self.statistics.different
        )?;

        writeln!(
            formatter,
            "Comparison inconclusive: {}",
            self.statistics.comparison_inconclusive
        )?;

        writeln!(
            formatter,
            "Complete: {}",
            self.is_complete()
        )?;

        writeln!(
            formatter,
            "Portable: {}",
            self.is_portable()
        )
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn environment_id(value: &str) -> EnvironmentId {
        EnvironmentId::new(value).expect("valid environment identity")
    }

    fn case_id(value: &str) -> ConformanceId {
        ConformanceId::new(value).expect("valid case identity")
    }

    fn domain(value: &str) -> ConformanceDomain {
        ConformanceDomain::new(value).expect("valid domain")
    }

    fn environment(id: &str) -> ExecutionEnvironment {
        ExecutionEnvironment::new(
            environment_id(id),
            "test implementation",
        )
    }

    fn observation(value: &str) -> PortabilityExecution {
        PortabilityExecution::observed(
            PortabilityObservation::from_text(value),
        )
    }

    fn basic_suite() -> PortabilitySuite {
        let mut suite = PortabilitySuite::new();

        suite
            .register_environment(environment("classical.first"))
            .unwrap();

        suite
            .register_environment(environment("classical.second"))
            .unwrap();

        suite
    }

    fn passing_case() -> PortabilityCase {
        PortabilityCase::new(
            case_id("classical.numeric"),
            domain("classical"),
            "Portable numerical operation",
            |_, _| observation("42"),
        )
    }

    #[test]
    fn accepts_extensible_environment_namespaces() {
        for value in [
            "classical.local",
            "quantum.simulator",
            "hdl.synthesis",
            "hybrid.execution",
            "distributed.cluster",
            "future.neuromorphic",
        ] {
            assert_eq!(
                environment_id(value).as_str(),
                value
            );
        }
    }

    #[test]
    fn rejects_invalid_environment_identifiers() {
        assert_eq!(
            EnvironmentId::new(""),
            Err(EnvironmentIdError::Empty)
        );

        assert_eq!(
            EnvironmentId::new(" classical"),
            Err(EnvironmentIdError::SurroundingWhitespace)
        );

        assert_eq!(
            EnvironmentId::new("quantum..execution"),
            Err(EnvironmentIdError::EmptyComponent)
        );

        assert_eq!(
            EnvironmentId::new("quantum/execution"),
            Err(EnvironmentIdError::InvalidCharacter)
        );
    }

    #[test]
    fn environment_attributes_are_extensible() {
        let environment = environment("quantum.simulator")
            .with_attribute("runtime.version", "1.0")
            .unwrap()
            .with_attribute("grammar.version", "4.0")
            .unwrap();

        assert_eq!(
            environment.attribute("runtime.version"),
            Some("1.0")
        );

        assert_eq!(
            environment.attribute("grammar.version"),
            Some("4.0")
        );
    }

    #[test]
    fn observation_preserves_canonical_bytes() {
        let value = PortabilityObservation::from_text("42");

        assert_eq!(value.as_bytes(), b"42");
        assert_eq!(value.len(), 2);
        assert!(!value.is_empty());
    }

    #[test]
    fn exact_comparison_accepts_equal_observations() {
        let first = PortabilityObservation::from_text("42");
        let second = PortabilityObservation::from_text("42");

        assert!(
            ExactEquivalence
                .compare(&first, &second)
                .is_equivalent()
        );
    }

    #[test]
    fn exact_comparison_rejects_different_observations() {
        let first = PortabilityObservation::from_text("42");
        let second = PortabilityObservation::from_text("43");

        assert!(matches!(
            ExactEquivalence.compare(&first, &second),
            EquivalenceResult::Different { .. }
        ));
    }

    #[test]
    fn rejects_duplicate_environments() {
        let mut suite = PortabilitySuite::new();

        suite
            .register_environment(environment("classical.local"))
            .unwrap();

        let result = suite.register_environment(
            environment("classical.local"),
        );

        assert!(matches!(
            result,
            Err(
                PortabilityRegistrationError::DuplicateEnvironment(_)
            )
        ));
    }

    #[test]
    fn rejects_duplicate_cases() {
        let mut suite = basic_suite();

        suite.register_case(passing_case()).unwrap();

        let result = suite.register_case(passing_case());

        assert!(matches!(
            result,
            Err(PortabilityRegistrationError::DuplicateCase(_))
        ));
    }

    #[test]
    fn empty_environment_suite_is_rejected() {
        let mut suite = PortabilitySuite::new();

        suite.register_case(passing_case()).unwrap();

        assert!(matches!(
            suite.run(
                &ConformanceContext::independent(),
                ComparisonStrategy::AllPairs,
            ),
            Err(PortabilityError::NoEnvironments)
        ));
    }

    #[test]
    fn empty_case_suite_is_rejected() {
        let suite = basic_suite();

        assert!(matches!(
            suite.run(
                &ConformanceContext::independent(),
                ComparisonStrategy::AllPairs,
            ),
            Err(PortabilityError::NoCases)
        ));
    }

    #[test]
    fn missing_reference_is_rejected() {
        let mut suite = basic_suite();

        suite.register_case(passing_case()).unwrap();

        assert!(matches!(
            suite.run(
                &ConformanceContext::independent(),
                ComparisonStrategy::Reference(
                    environment_id("quantum.missing"),
                ),
            ),
            Err(PortabilityError::MissingReference(_))
        ));
    }

    #[test]
    fn equal_observations_establish_portability() {
        let mut suite = basic_suite();

        suite.register_case(passing_case()).unwrap();

        let report = suite
            .run(
                &ConformanceContext::independent(),
                ComparisonStrategy::AllPairs,
            )
            .unwrap();

        assert!(report.is_portable());
        assert!(report.is_complete());

        assert_eq!(report.statistics().executions, 2);
        assert_eq!(report.statistics().observed, 2);
        assert_eq!(report.statistics().comparisons, 1);
        assert_eq!(report.statistics().equivalent, 1);
    }

    #[test]
    fn different_observations_reject_portability() {
        let mut suite = basic_suite();

        suite
            .register_case(PortabilityCase::new(
                case_id("classical.numeric"),
                domain("classical"),
                "Different results",
                |environment, _| {
                    if environment.id().as_str() == "classical.first" {
                        observation("42")
                    } else {
                        observation("43")
                    }
                },
            ))
            .unwrap();

        let report = suite
            .run(
                &ConformanceContext::independent(),
                ComparisonStrategy::AllPairs,
            )
            .unwrap();

        assert!(!report.is_portable());
        assert!(report.is_complete());
        assert!(report.has_semantic_differences());
    }

    #[test]
    fn unsupported_execution_never_passes() {
        let mut suite = basic_suite();

        suite
            .register_case(PortabilityCase::new(
                case_id("quantum.measurement"),
                domain("quantum"),
                "Quantum measurement",
                |environment, _| {
                    if environment.id().as_str() == "classical.first" {
                        observation("measurement")
                    } else {
                        PortabilityExecution::unsupported(
                            "Required quantum measurement capability unavailable",
                        )
                    }
                },
            ))
            .unwrap();

        let report = suite
            .run(
                &ConformanceContext::independent(),
                ComparisonStrategy::AllPairs,
            )
            .unwrap();

        assert!(!report.is_portable());
        assert!(report.has_unsupported_executions());
        assert_eq!(report.statistics().unsupported, 1);
    }

    #[test]
    fn skipped_execution_never_passes() {
        let mut suite = basic_suite();

        suite
            .register_case(PortabilityCase::new(
                case_id("hdl.synthesis"),
                domain("hdl"),
                "Synthesis test",
                |_, _| {
                    PortabilityExecution::skipped(
                        "Synthesis adapter unavailable",
                    )
                },
            ))
            .unwrap();

        let report = suite
            .run(
                &ConformanceContext::independent(),
                ComparisonStrategy::AllPairs,
            )
            .unwrap();

        assert!(!report.is_portable());
    }

    #[test]
    fn inconclusive_execution_never_passes() {
        let mut suite = basic_suite();

        suite
            .register_case(PortabilityCase::new(
                case_id("distributed.consistency"),
                domain("distributed"),
                "Consistency test",
                |_, _| {
                    PortabilityExecution::inconclusive(
                        "Insufficient evidence",
                    )
                },
            ))
            .unwrap();

        let report = suite
            .run(
                &ConformanceContext::independent(),
                ComparisonStrategy::AllPairs,
            )
            .unwrap();

        assert!(!report.is_portable());
        assert!(!report.is_complete());
    }

    #[test]
    fn cancellation_is_preserved() {
        let mut suite = basic_suite();

        suite.register_case(passing_case()).unwrap();

        let cancellation = ConformanceCancellation::new();
        cancellation.cancel();

        let context = ConformanceContext::new(cancellation);

        let report = suite
            .run(&context, ComparisonStrategy::AllPairs)
            .unwrap();

        assert!(!report.is_portable());
        assert_eq!(report.statistics().cancelled, 2);
    }

    #[test]
    fn panics_become_infrastructure_errors() {
        let mut suite = basic_suite();

        suite
            .register_case(PortabilityCase::new(
                case_id("classical.panicking"),
                domain("classical"),
                "Panicking test",
                |_, _| panic!("simulated failure"),
            ))
            .unwrap();

        let report = suite
            .run(
                &ConformanceContext::independent(),
                ComparisonStrategy::AllPairs,
            )
            .unwrap();

        assert!(!report.is_portable());

        assert_eq!(
            report.statistics().infrastructure_errors,
            2
        );
    }

    #[test]
    fn reference_comparison_uses_selected_environment() {
        let mut suite = basic_suite();

        suite.register_case(passing_case()).unwrap();

        let report = suite
            .run(
                &ConformanceContext::independent(),
                ComparisonStrategy::Reference(
                    environment_id("classical.first"),
                ),
            )
            .unwrap();

        assert!(report.is_portable());

        assert_eq!(report.comparisons().len(), 1);

        assert_eq!(
            report.comparisons()[0]
                .reference_environment()
                .as_str(),
            "classical.first"
        );
    }

    #[test]
    fn custom_comparison_supports_numerical_tolerance() {
        struct NumericalEquivalence;

        impl EquivalencePolicy for NumericalEquivalence {
            fn compare(
                &self,
                reference: &PortabilityObservation,
                candidate: &PortabilityObservation,
            ) -> EquivalenceResult {
                let expected = std::str::from_utf8(reference.as_bytes())
                    .ok()
                    .and_then(|value| value.parse::<f64>().ok());

                let actual = std::str::from_utf8(candidate.as_bytes())
                    .ok()
                    .and_then(|value| value.parse::<f64>().ok());

                match (expected, actual) {
                    (Some(expected), Some(actual))
                        if expected.is_finite()
                            && actual.is_finite()
                            && (expected - actual).abs() <= 0.001 =>
                    {
                        EquivalenceResult::Equivalent
                    }

                    (Some(_), Some(_)) => {
                        EquivalenceResult::different(
                            "numerical tolerance exceeded",
                        )
                    }

                    _ => {
                        EquivalenceResult::inconclusive(
                            "invalid numerical observation",
                        )
                    }
                }
            }
        }

        let mut suite = basic_suite();

        suite
            .register_case(PortabilityCase::new(
                case_id("scientific.numerical"),
                domain("scientific"),
                "Numerical portability",
                |environment, _| {
                    if environment.id().as_str() == "classical.first" {
                        observation("42.000")
                    } else {
                        observation("42.0005")
                    }
                },
            ))
            .unwrap();

        let report = suite
            .run_with_policy(
                &ConformanceContext::independent(),
                ComparisonStrategy::AllPairs,
                NumericalEquivalence,
            )
            .unwrap();

        assert!(report.is_portable());
    }

    #[test]
    fn invalid_custom_comparison_is_inconclusive() {
        struct InconclusivePolicy;

        impl EquivalencePolicy for InconclusivePolicy {
            fn compare(
                &self,
                _: &PortabilityObservation,
                _: &PortabilityObservation,
            ) -> EquivalenceResult {
                EquivalenceResult::inconclusive(
                    "insufficient comparison evidence",
                )
            }
        }

        let mut suite = basic_suite();

        suite.register_case(passing_case()).unwrap();

        let report = suite
            .run_with_policy(
                &ConformanceContext::independent(),
                ComparisonStrategy::AllPairs,
                InconclusivePolicy,
            )
            .unwrap();

        assert!(!report.is_portable());
        assert!(!report.is_complete());
    }

    #[test]
    fn one_environment_cannot_establish_portability() {
        let mut suite = PortabilitySuite::new();

        suite
            .register_environment(environment("classical.local"))
            .unwrap();

        suite.register_case(passing_case()).unwrap();

        let report = suite
            .run(
                &ConformanceContext::independent(),
                ComparisonStrategy::AllPairs,
            )
            .unwrap();

        assert_eq!(report.statistics().observed, 1);
        assert_eq!(report.statistics().comparisons, 0);
        assert!(!report.is_portable());
    }

    #[test]
    fn all_pairs_compares_every_environment_pair() {
        let mut suite = basic_suite();

        suite
            .register_environment(environment("classical.third"))
            .unwrap();

        suite.register_case(passing_case()).unwrap();

        let report = suite
            .run(
                &ConformanceContext::independent(),
                ComparisonStrategy::AllPairs,
            )
            .unwrap();

        assert_eq!(report.comparisons().len(), 3);
        assert!(report.is_portable());
    }

    #[test]
    fn reports_are_deterministically_ordered() {
        let mut suite = PortabilitySuite::new();

        suite
            .register_environment(environment("quantum.second"))
            .unwrap();

        suite
            .register_environment(environment("classical.first"))
            .unwrap();

        suite
            .register_case(PortabilityCase::new(
                case_id("quantum.execution"),
                domain("quantum"),
                "Quantum operation",
                |_, _| observation("result"),
            ))
            .unwrap();

        suite
            .register_case(PortabilityCase::new(
                case_id("classical.numeric"),
                domain("classical"),
                "Classical operation",
                |_, _| observation("result"),
            ))
            .unwrap();

        let report = suite
            .run(
                &ConformanceContext::independent(),
                ComparisonStrategy::AllPairs,
            )
            .unwrap();

        assert_eq!(
            report.executions()[0].case_id().as_str(),
            "classical.numeric"
        );

        assert_eq!(
            report.executions()[0].environment_id().as_str(),
            "classical.first"
        );
    }

    #[test]
    fn report_filters_by_case_and_environment() {
        let mut suite = basic_suite();

        suite.register_case(passing_case()).unwrap();

        let report = suite
            .run(
                &ConformanceContext::independent(),
                ComparisonStrategy::AllPairs,
            )
            .unwrap();

        assert_eq!(
            report
                .case_executions(&case_id("classical.numeric"))
                .len(),
            2
        );

        assert_eq!(
            report
                .environment_executions(
                    &environment_id("classical.first")
                )
                .len(),
            1
        );

        assert_eq!(
            report
                .case_comparisons(&case_id("classical.numeric"))
                .len(),
            1
        );
    }

    #[test]
    fn statistics_account_for_execution_outcomes() {
        let mut suite = basic_suite();

        suite
            .register_case(PortabilityCase::new(
                case_id("classical.mixed"),
                domain("classical"),
                "Mixed outcomes",
                |environment, _| {
                    if environment.id().as_str() == "classical.first" {
                        observation("42")
                    } else {
                        PortabilityExecution::failed(
                            "execution failed",
                        )
                    }
                },
            ))
            .unwrap();

        let report = suite
            .run(
                &ConformanceContext::independent(),
                ComparisonStrategy::AllPairs,
            )
            .unwrap();

        assert_eq!(report.statistics().executions, 2);
        assert_eq!(report.statistics().observed, 1);
        assert_eq!(report.statistics().failed, 1);

        assert!(!report.is_portable());
    }

    #[test]
    fn report_display_contains_acceptance_status() {
        let mut suite = basic_suite();

        suite.register_case(passing_case()).unwrap();

        let report = suite
            .run(
                &ConformanceContext::independent(),
                ComparisonStrategy::AllPairs,
            )
            .unwrap();

        let output = report.to_string();

        assert!(output.contains("Zamani Portability Report"));
        assert!(output.contains("Portable: true"));
    }
}
