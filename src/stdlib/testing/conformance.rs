
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Conformance Testing
//!
//! Path: `src/stdlib/testing/conformance.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides an extensible, target-independent conformance testing
//! infrastructure for Zamani.
//!
//! The infrastructure validates observable behavior and integration
//! contracts across:
//!
//! - Classical computation.
//! - Quantum computation.
//! - Hardware description and hardware-oriented computation.
//! - Classical/quantum hybrid computation.
//! - Distributed computation.
//! - Artificial intelligence and machine learning.
//! - Scientific computation.
//! - Networking.
//! - Cryptography and security.
//! - Embedded computation.
//! - Concurrency and asynchronous execution.
//! - Future computational domains.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! Grammar:
//!     grammar/
//!
//! AST:
//!     src/frontend/ast/
//!
//! Semantic analysis:
//!     src/frontend/semantic/
//!
//! Classical computation:
//!     src/classical/
//!
//! Quantum computation:
//!     src/quantum/
//!
//! Hardware description:
//!     src/hdl/
//!
//! AI:
//!     src/ai/
//!
//! Hybrid computation:
//!     src/hybrid/
//!
//! Distributed computation:
//!     src/distributed/
//!
//! Universal IR:
//!     src/zuir/
//!
//! Compiler:
//!     src/compiler/
//!
//! Backend:
//!     src/backend/
//!
//! Runtime:
//!     src/runtime/
//!
//! Standard-library assertions:
//!     src/stdlib/testing/assertions.rs
//!
//! ## OWNS
//!
//! - Conformance case metadata.
//! - Conformance case registration.
//! - Conformance dimensions.
//! - Conformance outcomes.
//! - Conformance execution coordination.
//! - Structured conformance reports.
//! - Conformance acceptance decisions.
//!
//! ## DOES NOT OWN
//!
//! - Grammar or parser implementations.
//! - AST or semantic models.
//! - IR representations.
//! - Compiler passes.
//! - Runtime scheduling.
//! - Hardware discovery or allocation.
//! - Provider implementations.
//! - Property-based generation.
//! - Fuzzing infrastructure.
//! - Physical device execution.
//!
//! ## IMPORTANT SEMANTIC RULE
//!
//! Unsupported, incomplete, cancelled, or inconclusive tests must
//! never be reported as successful.
//!
//! A target that cannot satisfy a program's semantic requirements
//! must report that inability explicitly.
//!
//! ## POCO-REAF
//!
//! Conformance tests must establish that portable program semantics
//! are preserved across compatible execution environments.
//!
//! Different implementations are permitted.
//!
//! Silent changes to observable program meaning are not.
//!
//! ## SCALABILITY
//!
//! This module introduces no arbitrary universal limits on:
//!
//! - Number of test cases.
//! - Number of domains.
//! - Number of conformance dimensions.
//! - Number of diagnostics.
//! - Number of execution environments.
//! - Number of providers.
//! - Number of tested operations.
//! - Size of test identifiers.
//! - Size of diagnostic messages.
//!
//! Actual capacity depends on available resources.
//!
//! Explicit operational budgets belong to the caller.
//!
//! No finite implementation promises literal infinite computation.
//!
//! ## SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No raw pointers.
//! No global mutable state.
//! No implicit filesystem access.
//! No implicit network access.
//! No implicit hardware access.
//!
//! ## INTEGRATION
//!
//! Public module path:
//!
//!     crate::stdlib::testing::conformance
//!
//! Assertions are provided by:
//!
//!     crate::stdlib::testing::assertions
//!
//! Compiler, grammar, semantic, IR and runtime adapters register
//! their own conformance cases through ConformanceSuite.
//!
//! ## FREEZE CRITERIA
//!
//! 1. All public contracts are documented.
//! 2. Registration rejects duplicate identities.
//! 3. Execution preserves deterministic report ordering.
//! 4. Failures remain distinguishable from unsupported operations.
//! 5. Incomplete verification never passes.
//! 6. Cancellation is explicitly reported.
//! 7. Panics are converted into failure outcomes when unwinding
//!    is enabled.
//! 8. Reports preserve all executed case outcomes.
//! 9. No arbitrary universal workload ceilings are introduced.
//! 10. All tests pass under the declared Rust baseline.

// =============================================================================
// Imports
// =============================================================================

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

// =============================================================================
// Shared assertion authority
// =============================================================================

use crate::stdlib::testing::assertions::AssertionFailure;

// =============================================================================
// Identifier validation
// =============================================================================

/// Validates an extensible, qualified conformance identifier.
///
/// Identifiers use dot-separated non-empty components.
///
/// Examples:
///
/// - `classical.numeric`
— `quantum.execution`
/// - `hardware.synthesis`
/// - `distributed.consistency`
/// - `future.custom_domain`
///
/// Identifiers are not restricted to a closed domain enumeration.
fn validate_identifier(value: &str) -> Result<(), IdentifierError> {
    if value.trim().is_empty() {
        return Err(IdentifierError::Empty);
    }

    if value.trim() != value {
        return Err(IdentifierError::SurroundingWhitespace);
    }

    if value.split('.').any(str::is_empty) {
        return Err(IdentifierError::EmptyComponent);
    }

    if value
        .chars()
        .any(|character| {
            !(character.is_ascii_alphanumeric()
                || character == '_'
                || character == '-'
                || character == '.')
        })
    {
        return Err(IdentifierError::InvalidCharacter);
    }

    Ok(())
}

/// Errors encountered while validating identifiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentifierError {
    /// The identifier is empty.
    Empty,

    /// The identifier contains surrounding whitespace.
    SurroundingWhitespace,

    /// A qualified identifier contains an empty component.
    EmptyComponent,

    /// The identifier contains an unsupported character.
    InvalidCharacter,
}

impl fmt::Display for IdentifierError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(formatter, "identifier cannot be empty"),
            Self::SurroundingWhitespace => {
                write!(formatter, "identifier contains surrounding whitespace")
            }
            Self::EmptyComponent => {
                write!(formatter, "identifier contains an empty component")
            }
            Self::InvalidCharacter => {
                write!(formatter, "identifier contains an invalid character")
            }
        }
    }
}

impl Error for IdentifierError {}

// =============================================================================
// Conformance identifiers
// =============================================================================

/// Globally meaningful identity within a conformance suite.
///
/// The identifier is independent of hardware, providers and execution
/// environments.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConformanceId(String);

impl ConformanceId {
    /// Creates a validated conformance identity.
    pub fn new(value: impl Into<String>) -> Result<Self, IdentifierError> {
        let value = value.into();
        validate_identifier(&value)?;
        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ConformanceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Extensible computational domain identity.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConformanceDomain(String);

impl ConformanceDomain {
    /// Creates a validated domain identity.
    pub fn new(value: impl Into<String>) -> Result<Self, IdentifierError> {
        let value = value.into();
        validate_identifier(&value)?;
        Ok(Self(value))
    }

    /// Returns the domain identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ConformanceDomain {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Extensible conformance dimension.
///
/// Dimensions describe what a test verifies without imposing a
/// permanently closed list of testing categories.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConformanceDimension(String);

impl ConformanceDimension {
    /// Creates a validated dimension.
    pub fn new(value: impl Into<String>) -> Result<Self, IdentifierError> {
        let value = value.into();
        validate_identifier(&value)?;
        Ok(Self(value))
    }

    /// Returns the dimension identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Verifies externally observable behavior.
    pub fn behavior() -> Self {
        Self("behavior".to_owned())
    }

    /// Verifies deterministic behavior.
    pub fn determinism() -> Self {
        Self("determinism".to_owned())
    }

    /// Verifies serialization compatibility.
    pub fn serialization() -> Self {
        Self("serialization".to_owned())
    }

    /// Verifies error semantics.
    pub fn errors() -> Self {
        Self("errors".to_owned())
    }

    /// Verifies cancellation behavior.
    pub fn cancellation() -> Self {
        Self("cancellation".to_owned())
    }

    /// Verifies resource exhaustion handling.
    pub fn resource_exhaustion() -> Self {
        Self("resource_exhaustion".to_owned())
    }

    /// Verifies unsupported-capability reporting.
    pub fn capability_negotiation() -> Self {
        Self("capability_negotiation".to_owned())
    }

    /// Verifies provider substitution.
    pub fn provider_interoperability() -> Self {
        Self("provider_interoperability".to_owned())
    }

    /// Verifies portability across environments.
    pub fn portability() -> Self {
        Self("portability".to_owned())
    }

    /// Verifies reproducibility.
    pub fn reproducibility() -> Self {
        Self("reproducibility".to_owned())
    }

    /// Verifies compatibility between versions.
    pub fn version_compatibility() -> Self {
        Self("version_compatibility".to_owned())
    }

    /// Verifies preservation of source semantics.
    pub fn semantic_preservation() -> Self {
        Self("semantic_preservation".to_owned())
    }

    /// Verifies integration between architectural boundaries.
    pub fn integration() -> Self {
        Self("integration".to_owned())
    }
}

impl fmt::Display for ConformanceDimension {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Cancellation
// =============================================================================

/// Cooperative cancellation signal for conformance execution.
///
/// Cancellation does not forcibly terminate an executing test.
/// Individual cases must check the context when performing
/// potentially long-running operations.
#[derive(Clone, Debug, Default)]
pub struct ConformanceCancellation {
    cancelled: Arc<AtomicBool>,
}

impl ConformanceCancellation {
    /// Creates a new cancellation signal.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Requests cancellation.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

// =============================================================================
// Execution context
// =============================================================================

/// Context supplied to every conformance case.
///
/// This context does not allocate hardware, select providers,
/// schedule workloads or change semantic requirements.
#[derive(Clone, Debug, Default)]
pub struct ConformanceContext {
    cancellation: ConformanceCancellation,
}

impl ConformanceContext {
    /// Creates a context using the supplied cancellation signal.
    #[must_use]
    pub fn new(cancellation: ConformanceCancellation) -> Self {
        Self { cancellation }
    }

    /// Creates a context with a new cancellation signal.
    #[must_use]
    pub fn independent() -> Self {
        Self::default()
    }

    /// Returns the cancellation signal.
    #[must_use]
    pub fn cancellation(&self) -> &ConformanceCancellation {
        &self.cancellation
    }

    /// Returns whether the current operation should stop.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancellation.is_cancelled()
    }
}

// =============================================================================
// Test outcomes
// =============================================================================

/// Outcome of one conformance case.
///
/// Only `Passed` represents successful verification.
///
/// Unsupported and inconclusive outcomes are never successful.
#[derive(Debug)]
pub enum ConformanceOutcome {
    /// The conformance requirements were verified successfully.
    Passed,

    /// One or more conformance assertions failed.
    Failed(Vec<AssertionFailure>),

    /// The required operation is not supported by the tested environment.
    Unsupported {
        /// Explanation of the unsupported operation.
        reason: String,
    },

    /// Verification could not establish a definitive result.
    Inconclusive {
        /// Explanation of incomplete verification.
        reason: String,
    },

    /// The case was explicitly skipped.
    Skipped {
        /// Explanation of the skip.
        reason: String,
    },

    /// The case was cancelled.
    Cancelled,

    /// An execution infrastructure failure occurred.
    InfrastructureError {
        /// Explanation of the infrastructure failure.
        reason: String,
    },
}

impl ConformanceOutcome {
    /// Creates a successful outcome.
    #[must_use]
    pub const fn passed() -> Self {
        Self::Passed
    }

    /// Creates a failed outcome.
    #[must_use]
    pub fn failed(
        name: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::Failed(vec![
            AssertionFailure::new(name, message)
        ])
    }

    /// Creates an unsupported outcome.
    #[must_use]
    pub fn unsupported(reason: impl Into<String>) -> Self {
        Self::Unsupported {
            reason: reason.into(),
        }
    }

    /// Creates an inconclusive outcome.
    #[must_use]
    pub fn inconclusive(reason: impl Into<String>) -> Self {
        Self::Inconclusive {
            reason: reason.into(),
        }
    }

    /// Creates a skipped outcome.
    #[must_use]
    pub fn skipped(reason: impl Into<String>) -> Self {
        Self::Skipped {
            reason: reason.into(),
        }
    }

    /// Creates an infrastructure-error outcome.
    #[must_use]
    pub fn infrastructure_error(reason: impl Into<String>) -> Self {
        Self::InfrastructureError {
            reason: reason.into(),
        }
    }

    /// Returns whether verification genuinely succeeded.
    #[must_use]
    pub const fn is_successful(&self) -> bool {
        matches!(self, Self::Passed)
    }
}

// =============================================================================
// Case definitions
// =============================================================================

/// One independently executable conformance case.
pub struct ConformanceCase {
    id: ConformanceId,
    domain: ConformanceDomain,
    dimension: ConformanceDimension,
    description: String,
    execute: Box<
        dyn Fn(&ConformanceContext) -> ConformanceOutcome
            + Send
            + Sync
            + 'static,
    >,
}

impl fmt::Debug for ConformanceCase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ConformanceCase")
            .field("id", &self.id)
            .field("domain", &self.domain)
            .field("dimension", &self.dimension)
            .field("description", &self.description)
            .finish_non_exhaustive()
    }
}

impl ConformanceCase {
    /// Creates a conformance case.
    ///
    /// The callback must return an explicit outcome.
    ///
    /// Panics are caught and reported as infrastructure failures
    /// when the process uses unwinding.
    pub fn new<F>(
        id: ConformanceId,
        domain: ConformanceDomain,
        dimension: ConformanceDimension,
        description: impl Into<String>,
        execute: F,
    ) -> Self
    where
        F: Fn(&ConformanceContext) -> ConformanceOutcome
            + Send
            + Sync
            + 'static,
    {
        Self {
            id,
            domain,
            dimension,
            description: description.into(),
            execute: Box::new(execute),
        }
    }

    /// Returns the case identity.
    #[must_use]
    pub fn id(&self) -> &ConformanceId {
        &self.id
    }

    /// Returns the computational domain.
    #[must_use]
    pub fn domain(&self) -> &ConformanceDomain {
        &self.domain
    }

    /// Returns the verified dimension.
    #[must_use]
    pub fn dimension(&self) -> &ConformanceDimension {
        &self.dimension
    }

    /// Returns the case description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

// =============================================================================
// Registration errors
// =============================================================================

/// Errors encountered while constructing a conformance suite.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegistrationError {
    /// A case with the same identity is already registered.
    DuplicateCase(ConformanceId),
}

impl fmt::Display for RegistrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateCase(id) => {
                write!(formatter, "duplicate conformance case: {id}")
            }
        }
    }
}

impl Error for RegistrationError {}

// =============================================================================
// Conformance suite
// =============================================================================

/// Collection of registered conformance cases.
///
/// Cases are stored by identity, giving deterministic execution and
/// reporting order independent of registration order.
///
/// Execution is sequential in this implementation.
///
/// Parallel execution belongs to a future explicitly integrated
/// execution adapter, not to an implicit global scheduler.
#[derive(Default)]
pub struct ConformanceSuite {
    cases: BTreeMap<ConformanceId, ConformanceCase>,
}

impl fmt::Debug for ConformanceSuite {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ConformanceSuite")
            .field("case_count", &self.cases.len())
            .field(
                "case_ids",
                &self.cases.keys().collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl ConformanceSuite {
    /// Creates an empty suite.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a conformance case.
    ///
    /// Duplicate identities are rejected instead of silently
    /// replacing previously registered cases.
    pub fn register(
        &mut self,
        case: ConformanceCase,
    ) -> Result<(), RegistrationError> {
        let id = case.id.clone();

        if self.cases.contains_key(&id) {
            return Err(RegistrationError::DuplicateCase(id));
        }

        self.cases.insert(id, case);

        Ok(())
    }

    /// Returns the number of registered cases.
    #[must_use]
    pub fn len(&self) -> usize {
        self.cases.len()
    }

    /// Returns whether the suite contains no cases.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cases.is_empty()
    }

    /// Returns the registered case identities in deterministic order.
    #[must_use]
    pub fn case_ids(&self) -> Vec<&ConformanceId> {
        self.cases.keys().collect()
    }

    /// Executes all registered cases.
    ///
    /// Every registered case receives the same execution context.
    ///
    /// Cancellation prevents subsequent cases from executing.
    ///
    /// An empty suite does not establish conformance.
    #[must_use]
    pub fn run(
        &self,
        context: &ConformanceContext,
    ) -> ConformanceReport {
        let mut results = Vec::with_capacity(self.cases.len());

        for case in self.cases.values() {
            if context.is_cancelled() {
                results.push(CaseResult {
                    id: case.id.clone(),
                    domain: case.domain.clone(),
                    dimension: case.dimension.clone(),
                    description: case.description.clone(),
                    outcome: ConformanceOutcome::Cancelled,
                });

                continue;
            }

            let outcome = match catch_unwind(AssertUnwindSafe(|| {
                (case.execute)(context)
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
                        "conformance case panicked with a non-string payload"
                            .to_owned()
                    };

                    ConformanceOutcome::infrastructure_error(reason)
                }
            };

            results.push(CaseResult {
                id: case.id.clone(),
                domain: case.domain.clone(),
                dimension: case.dimension.clone(),
                description: case.description.clone(),
                outcome,
            });
        }

        ConformanceReport::new(results)
    }
}

// =============================================================================
// Individual case result
// =============================================================================

/// Complete result of one executed conformance case.
#[derive(Debug)]
pub struct CaseResult {
    id: ConformanceId,
    domain: ConformanceDomain,
    dimension: ConformanceDimension,
    description: String,
    outcome: ConformanceOutcome,
}

impl CaseResult {
    /// Returns the case identity.
    #[must_use]
    pub fn id(&self) -> &ConformanceId {
        &self.id
    }

    /// Returns the computational domain.
    #[must_use]
    pub fn domain(&self) -> &ConformanceDomain {
        &self.domain
    }

    /// Returns the tested dimension.
    #[must_use]
    pub fn dimension(&self) -> &ConformanceDimension {
        &self.dimension
    }

    /// Returns the description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the execution outcome.
    #[must_use]
    pub fn outcome(&self) -> &ConformanceOutcome {
        &self.outcome
    }

    /// Returns whether this case passed.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.outcome.is_successful()
    }
}

// =============================================================================
// Aggregate statistics
// =============================================================================

/// Aggregate conformance execution statistics.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ConformanceStatistics {
    /// Number of registered and executed cases.
    pub executed: usize,

    /// Number of successful cases.
    pub passed: usize,

    /// Number of failed cases.
    pub failed: usize,

    /// Number of unsupported cases.
    pub unsupported: usize,

    /// Number of inconclusive cases.
    pub inconclusive: usize,

    /// Number of skipped cases.
    pub skipped: usize,

    /// Number of cancelled cases.
    pub cancelled: usize,

    /// Number of infrastructure errors.
    pub infrastructure_errors: usize,
}

impl ConformanceStatistics {
    /// Returns whether all executed cases passed.
    #[must_use]
    pub fn all_passed(&self) -> bool {
        self.executed > 0
            && self.passed == self.executed
    }

    /// Returns the number of non-passing cases.
    #[must_use]
    pub fn non_passing(&self) -> usize {
        self.failed
            + self.unsupported
            + self.inconclusive
            + self.skipped
            + self.cancelled
            + self.infrastructure_errors
    }
}

// =============================================================================
// Conformance report
// =============================================================================

/// Complete report for a conformance execution.
#[derive(Debug)]
pub struct ConformanceReport {
    results: Vec<CaseResult>,
    statistics: ConformanceStatistics,
}

impl ConformanceReport {
    fn new(results: Vec<CaseResult>) -> Self {
        let mut statistics = ConformanceStatistics::default();

        for result in &results {
            statistics.executed += 1;

            match &result.outcome {
                ConformanceOutcome::Passed => {
                    statistics.passed += 1;
                }

                ConformanceOutcome::Failed(_) => {
                    statistics.failed += 1;
                }

                ConformanceOutcome::Unsupported { .. } => {
                    statistics.unsupported += 1;
                }

                ConformanceOutcome::Inconclusive { .. } => {
                    statistics.inconclusive += 1;
                }

                ConformanceOutcome::Skipped { .. } => {
                    statistics.skipped += 1;
                }

                ConformanceOutcome::Cancelled => {
                    statistics.cancelled += 1;
                }

                ConformanceOutcome::InfrastructureError { .. } => {
                    statistics.infrastructure_errors += 1;
                }
            }
        }

        Self {
            results,
            statistics,
        }
    }

    /// Returns all case results in deterministic order.
    #[must_use]
    pub fn results(&self) -> &[CaseResult] {
        &self.results
    }

    /// Returns aggregate statistics.
    #[must_use]
    pub const fn statistics(&self) -> &ConformanceStatistics {
        &self.statistics
    }

    /// Returns whether every executed case passed.
    ///
    /// An empty suite never establishes conformance.
    #[must_use]
    pub fn is_conformant(&self) -> bool {
        self.statistics.all_passed()
    }

    /// Returns whether any case failed.
    #[must_use]
    pub fn has_failures(&self) -> bool {
        self.statistics.failed > 0
    }

    /// Returns whether the report is complete.
    ///
    /// A report is incomplete if execution was cancelled or
    /// any case was inconclusive.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.statistics.cancelled == 0
            && self.statistics.inconclusive == 0
    }

    /// Returns whether any case is unsupported.
    #[must_use]
    pub fn has_unsupported_cases(&self) -> bool {
        self.statistics.unsupported > 0
    }

    /// Returns whether infrastructure errors occurred.
    #[must_use]
    pub fn has_infrastructure_errors(&self) -> bool {
        self.statistics.infrastructure_errors > 0
    }

    /// Returns assertion failures from all failed cases.
    #[must_use]
    pub fn failures(&self) -> Vec<&AssertionFailure> {
        self.results
            .iter()
            .filter_map(|result| {
                if let ConformanceOutcome::Failed(failures) =
                    &result.outcome
                {
                    Some(failures.iter())
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }

    /// Returns results belonging to a particular domain.
    #[must_use]
    pub fn domain_results(
        &self,
        domain: &ConformanceDomain,
    ) -> Vec<&CaseResult> {
        self.results
            .iter()
            .filter(|result| &result.domain == domain)
            .collect()
    }

    /// Returns results belonging to a particular dimension.
    #[must_use]
    pub fn dimension_results(
        &self,
        dimension: &ConformanceDimension,
    ) -> Vec<&CaseResult> {
        self.results
            .iter()
            .filter(|result| &result.dimension == dimension)
            .collect()
    }
}

impl fmt::Display for ConformanceReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "Zamani Conformance Report")?;

        writeln!(
            formatter,
            "Executed: {}",
            self.statistics.executed
        )?;

        writeln!(
            formatter,
            "Passed: {}",
            self.statistics.passed
        )?;

        writeln!(
            formatter,
            "Failed: {}",
            self.statistics.failed
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
            "Conformant: {}",
            self.is_conformant()
        )
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> ConformanceId {
        ConformanceId::new(value).expect("valid identity")
    }

    fn domain(value: &str) -> ConformanceDomain {
        ConformanceDomain::new(value).expect("valid domain")
    }

    fn dimension(value: &str) -> ConformanceDimension {
        ConformanceDimension::new(value).expect("valid dimension")
    }

    fn passing_case(name: &str) -> ConformanceCase {
        ConformanceCase::new(
            id(name),
            domain("classical"),
            ConformanceDimension::behavior(),
            "A passing conformance case",
            |_| ConformanceOutcome::passed(),
        )
    }

    #[test]
    fn identifiers_accept_extensible_domains() {
        assert_eq!(
            domain("future.neuromorphic").as_str(),
            "future.neuromorphic"
        );

        assert_eq!(
            domain("quantum.new_architecture").as_str(),
            "quantum.new_architecture"
        );
    }

    #[test]
    fn identifiers_reject_empty_values() {
        assert_eq!(
            ConformanceId::new(""),
            Err(IdentifierError::Empty)
        );
    }

    #[test]
    fn identifiers_reject_empty_components() {
        assert_eq!(
            ConformanceDomain::new("classical..numeric"),
            Err(IdentifierError::EmptyComponent)
        );
    }

    #[test]
    fn identifiers_reject_whitespace() {
        assert_eq!(
            ConformanceId::new(" classical"),
            Err(IdentifierError::SurroundingWhitespace)
        );
    }

    #[test]
    fn identifiers_reject_invalid_characters() {
        assert_eq!(
            ConformanceId::new("classical/numeric"),
            Err(IdentifierError::InvalidCharacter)
        );
    }

    #[test]
    fn empty_suite_does_not_establish_conformance() {
        let suite = ConformanceSuite::new();

        let report = suite.run(&ConformanceContext::independent());

        assert!(suite.is_empty());
        assert!(!report.is_conformant());
        assert!(!report.is_complete());
    }

    #[test]
    fn suite_registers_cases() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(passing_case("classical.numeric"))
            .expect("registration succeeds");

        assert_eq!(suite.len(), 1);
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(passing_case("classical.numeric"))
            .expect("first registration succeeds");

        let result = suite.register(
            passing_case("classical.numeric")
        );

        assert!(matches!(
            result,
            Err(RegistrationError::DuplicateCase(_))
        ));
    }

    #[test]
    fn passing_cases_establish_conformance() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(passing_case("classical.numeric"))
            .expect("registration succeeds");

        let report = suite.run(&ConformanceContext::independent());

        assert!(report.is_conformant());
        assert!(report.is_complete());
        assert_eq!(report.statistics().passed, 1);
    }

    #[test]
    fn failed_cases_do_not_establish_conformance() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(ConformanceCase::new(
                id("classical.failure"),
                domain("classical"),
                dimension("behavior"),
                "Expected failure",
                |_| {
                    ConformanceOutcome::failed(
                        "example",
                        "expected failure",
                    )
                },
            ))
            .expect("registration succeeds");

        let report = suite.run(&ConformanceContext::independent());

        assert!(!report.is_conformant());
        assert!(report.has_failures());
        assert_eq!(report.statistics().failed, 1);
        assert_eq!(report.failures().len(), 1);
    }

    #[test]
    fn unsupported_cases_never_pass() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(ConformanceCase::new(
                id("quantum.execution"),
                domain("quantum"),
                dimension("capability_negotiation"),
                "Quantum capability test",
                |_| ConformanceOutcome::unsupported(
                    "No compatible quantum provider",
                ),
            ))
            .expect("registration succeeds");

        let report = suite.run(&ConformanceContext::independent());

        assert!(!report.is_conformant());
        assert!(report.has_unsupported_cases());
        assert_eq!(report.statistics().unsupported, 1);
    }

    #[test]
    fn inconclusive_cases_are_not_complete() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(ConformanceCase::new(
                id("distributed.consistency"),
                domain("distributed"),
                dimension("semantic_preservation"),
                "Distributed consistency verification",
                |_| ConformanceOutcome::inconclusive(
                    "Verification evidence is incomplete",
                ),
            ))
            .expect("registration succeeds");

        let report = suite.run(&ConformanceContext::independent());

        assert!(!report.is_conformant());
        assert!(!report.is_complete());
    }

    #[test]
    fn skipped_cases_never_establish_conformance() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(ConformanceCase::new(
                id("hardware.synthesis"),
                domain("hardware"),
                dimension("integration"),
                "Hardware synthesis test",
                |_| ConformanceOutcome::skipped(
                    "Required synthesis adapter unavailable",
                ),
            ))
            .expect("registration succeeds");

        let report = suite.run(&ConformanceContext::independent());

        assert!(!report.is_conformant());
        assert_eq!(report.statistics().skipped, 1);
    }

    #[test]
    fn cancellation_prevents_later_cases() {
        let cancellation = ConformanceCancellation::new();
        cancellation.cancel();

        let context = ConformanceContext::new(cancellation);

        let mut suite = ConformanceSuite::new();

        suite
            .register(passing_case("classical.first"))
            .expect("registration succeeds");

        suite
            .register(passing_case("classical.second"))
            .expect("registration succeeds");

        let report = suite.run(&context);

        assert!(!report.is_conformant());
        assert_eq!(report.statistics().cancelled, 2);
        assert_eq!(report.statistics().passed, 0);
    }

    #[test]
    fn panic_becomes_infrastructure_failure() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(ConformanceCase::new(
                id("classical.panicking"),
                domain("classical"),
                dimension("behavior"),
                "Panicking test",
                |_| panic!("simulated infrastructure failure"),
            ))
            .expect("registration succeeds");

        let report = suite.run(&ConformanceContext::independent());

        assert!(!report.is_conformant());
        assert!(report.has_infrastructure_errors());
    }

    #[test]
    fn reports_are_ordered_by_case_identity() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(passing_case("quantum.execution"))
            .expect("registration succeeds");

        suite
            .register(passing_case("classical.numeric"))
            .expect("registration succeeds");

        let report = suite.run(&ConformanceContext::independent());

        assert_eq!(
            report.results()[0].id().as_str(),
            "classical.numeric"
        );

        assert_eq!(
            report.results()[1].id().as_str(),
            "quantum.execution"
        );
    }

    #[test]
    fn domain_filtering_works() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(passing_case("classical.numeric"))
            .expect("registration succeeds");

        suite
            .register(ConformanceCase::new(
                id("quantum.execution"),
                domain("quantum"),
                dimension("behavior"),
                "Quantum execution",
                |_| ConformanceOutcome::passed(),
            ))
            .expect("registration succeeds");

        let report = suite.run(&ConformanceContext::independent());

        assert_eq!(
            report.domain_results(&domain("quantum")).len(),
            1
        );

        assert_eq!(
            report.domain_results(&domain("classical")).len(),
            1
        );
    }

    #[test]
    fn dimension_filtering_works() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(passing_case("classical.numeric"))
            .expect("registration succeeds");

        let report = suite.run(&ConformanceContext::independent());

        assert_eq!(
            report
                .dimension_results(
                    &ConformanceDimension::behavior()
                )
                .len(),
            1
        );
    }

    #[test]
    fn statistics_account_for_all_outcomes() {
        let mut suite = ConformanceSuite::new();

        let cases = [
            ("case.passed", ConformanceOutcome::passed()),
            (
                "case.failed",
                ConformanceOutcome::failed("test", "failure"),
            ),
            (
                "case.unsupported",
                ConformanceOutcome::unsupported("unsupported"),
            ),
            (
                "case.inconclusive",
                ConformanceOutcome::inconclusive("incomplete"),
            ),
            (
                "case.skipped",
                ConformanceOutcome::skipped("skipped"),
            ),
            (
                "case.error",
                ConformanceOutcome::infrastructure_error("error"),
            ),
        ];

        for (name, outcome) in cases {
            suite
                .register(ConformanceCase::new(
                    id(name),
                    domain("classical"),
                    dimension("behavior"),
                    "Statistics test",
                    move |_| match &outcome {
                        ConformanceOutcome::Passed => {
                            ConformanceOutcome::passed()
                        }

                        ConformanceOutcome::Failed(_) => {
                            ConformanceOutcome::failed(
                                "test",
                                "failure",
                            )
                        }

                        ConformanceOutcome::Unsupported { .. } => {
                            ConformanceOutcome::unsupported("unsupported")
                        }

                        ConformanceOutcome::Inconclusive { .. } => {
                            ConformanceOutcome::inconclusive("incomplete")
                        }

                        ConformanceOutcome::Skipped { .. } => {
                            ConformanceOutcome::skipped("skipped")
                        }

                        ConformanceOutcome::InfrastructureError { .. } => {
                            ConformanceOutcome::infrastructure_error("error")
                        }

                        ConformanceOutcome::Cancelled => {
                            ConformanceOutcome::cancelled_for_test()
                        }
                    },
                ))
                .expect("registration succeeds");
        }

        let report = suite.run(&ConformanceContext::independent());

        assert_eq!(report.statistics().executed, 6);
        assert_eq!(report.statistics().passed, 1);
        assert_eq!(report.statistics().failed, 1);
        assert_eq!(report.statistics().unsupported, 1);
        assert_eq!(report.statistics().inconclusive, 1);
        assert_eq!(report.statistics().skipped, 1);
        assert_eq!(report.statistics().infrastructure_errors, 1);
    }

    #[test]
    fn report_display_contains_acceptance_status() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(passing_case("classical.numeric"))
            .expect("registration succeeds");

        let report = suite.run(&ConformanceContext::independent());

        let output = report.to_string();

        assert!(output.contains("Zamani Conformance Report"));
        assert!(output.contains("Conformant: true"));
    }
}
