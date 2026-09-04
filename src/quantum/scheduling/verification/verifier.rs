//! Zamani Quantum Scheduling — Unified Schedule Verification
//!
//! Path:
//!     src/quantum/scheduling/verification/verifier.rs
//!
//! # Purpose
//!
//! This module is the production verification boundary for the Zamani quantum
//! scheduler.
//!
//! It does not implement individual verification algorithms. Instead, it
//! composes independently implemented verification domains into one immutable
//! verification pipeline.
//!
//! The central invariant is:
//!
//! > A schedule may change when an operation executes, where a routed
//! > operation executes, and which legal resources execute it, but it must not
//! > violate the program's structural, dependency, resource, timing, or
//! > semantic contracts.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                             │
//!                             ▼
//!                       routing / mapping
//!                             │
//!                             ▼
//!                         scheduler
//!                             │
//!                             ▼
//!                       ScheduleResult
//!                             │
//!                             ▼
//!                 ┌──────────────────────────┐
//!                 │ Unified ScheduleVerifier│
//!                 └────────────┬─────────────┘
//!                              │
//!          ┌───────────────────┼────────────────────┐
//!          │                   │                    │
//!          ▼                   ▼                    ▼
//!      structural          dependency           resource
//!          │                   │                    │
//!          └───────────────────┼────────────────────┘
//!                              │
//!                    ┌─────────┴─────────┐
//!                    ▼                   ▼
//!                 timing              semantic
//!                    │                   │
//!                    └─────────┬─────────┘
//!                              ▼
//!                     VerificationReport
//!                              │
//!                    ┌─────────┴─────────┐
//!                    ▼                   ▼
//!                 accepted            rejected
//! ```
//!
//! # Responsibilities
//!
//! This file owns:
//!
//! - verification configuration;
//! - verification-domain registration;
//! - verification ordering;
//! - fail-fast policy;
//! - complete-report policy;
//! - aggregation of domain reports;
//! - verification status;
//! - verification summary;
//! - deterministic execution of verification stages;
//! - cross-domain verification invariants;
//! - production acceptance/rejection.
//!
//! This file does NOT own:
//!
//! - structural verification algorithms;
//! - dependency graph algorithms;
//! - resource calendars;
//! - timing arithmetic;
//! - semantic equivalence algorithms;
//! - routing;
//! - hardware discovery;
//! - QEC algorithms;
//! - runtime execution;
//! - optimization;
//! - schedule mutation.
//!
//! Those responsibilities remain in their respective modules.
//!
//! # Verification domains
//!
//! The production scheduler should verify, at minimum:
//!
//! 1. structural integrity;
//! 2. dependency correctness;
//! 3. resource correctness;
//! 4. timing correctness;
//! 5. semantic preservation.
//!
//! The verifier is deliberately extensible so future domains can be added for:
//!
//! - QEC;
//! - distributed execution;
//! - dynamic execution;
//! - communication;
//! - security/capability constraints;
//! - hardware conformance;
//! - provenance;
//! - deterministic reproducibility;
//! - serialization round trips.
//!
//! # No hard-coded machine limits
//!
//! This verifier imposes no architectural limit on:
//!
//! - number of qubits;
//! - number of operations;
//! - number of resources;
//! - number of channels;
//! - circuit depth;
//! - operation arity;
//! - number of dependencies;
//! - number of QEC rounds;
//! - number of distributed nodes;
//! - number of verification domains.
//!
//! Actual limits come from:
//!
//! - the supplied schedule;
//! - the target/resource model;
//! - explicit caller policy;
//! - finite host resources.
//!
//! There is intentionally no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_DEPTH
//! MAX_CHANNELS
//! ```
//!
//! # Scalability
//!
//! The verifier does not construct a global time-slot matrix or a
//! qubit-by-time matrix.
//!
//! Individual verification domains are responsible for their own appropriate
//! data structures.
//!
//! The orchestration layer stores one result per verification domain.
//!
//! Let:
//!
//! - `D` = number of verification domains;
//! - `V` = total verification work performed by those domains.
//!
//! This orchestration layer performs `O(D)` coordination work in addition to
//! the actual domain verification work.
//!
//! # Determinism
//!
//! Verification order is explicitly represented by `VerificationDomain`.
//!
//! Domain execution never depends on:
//!
//! - hash-map iteration order;
//! - pointer addresses;
//! - wall-clock time;
//! - process IDs;
//! - thread scheduling;
//! - hidden randomness.
//!
//! Parallel verification can be added by an adapter, but the resulting report
//! must preserve canonical domain ordering before it is exposed publicly.
//!
//! # Mutation
//!
//! Verification is strictly read-only.
//!
//! A verifier MUST NOT:
//!
//! - modify a schedule;
//! - insert delays;
//! - change operation ordering;
//! - reroute qubits;
//! - change durations;
//! - reserve resources;
//! - mutate hardware state;
//! - modify QEC state.
//!
//! Transformations belong before or after verification according to the
//! scheduling pipeline.
//!
//! # Production acceptance rule
//!
//! A schedule is accepted only when every mandatory verification domain passes.
//!
//! ```text
//! all mandatory domains valid
//!         │
//!         ▼
//!      ACCEPTED
//! ```
//!
//! Any mandatory violation produces:
//!
//! ```text
//! REJECTED
//! ```
//!
//! A warning is never silently promoted to success or silently treated as a
//! failure. Severity is explicit.
//!
//! # Verification pipeline
//!
//! The recommended production sequence is:
//!
//! ```text
//! schedule construction
//!        │
//!        ▼
//! structural verification
//!        │
//!        ▼
//! dependency verification
//!        │
//!        ▼
//! resource verification
//!        │
//!        ▼
//! timing verification
//!        │
//!        ▼
//! semantic verification
//!        │
//!        ▼
//! final cross-domain checks
//!        │
//!        ▼
//! acceptance
//! ```
//!
//! Individual domains may be independently invoked for diagnostics, but
//! production schedule acceptance should use this unified verifier.
//!
//! # Why semantic verification is last
//!
//! Semantic verification is deliberately separated from structural,
//! dependency, resource, and timing verification.
//!
//! Structural errors should be reported before semantic conclusions are drawn
//! from malformed schedule entries.
//!
//! Timing/resource/dependency verification ensures that the schedule is a
//! legal execution arrangement before it is accepted as a complete schedule.
//!
//! The semantic verifier then establishes that scheduling did not change the
//! computation.
//!
//! # Routing boundary
//!
//! Semantic verification must NOT reject a legal logical-to-physical mapping.
//!
//! For example:
//!
//! ```text
//! logical q0 -> physical q17
//! logical q1 -> physical q23
//! ```
//!
//! may be completely valid even though the physical identities differ from an
//! earlier representation.
//!
//! Physical placement belongs to routing/resource/hardware verification.
//!
//! Semantic verification uses canonical logical identities where appropriate,
//! specifically:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! # Error model
//!
//! There are two different classes of failure:
//!
//! 1. verification completed and found violations;
//! 2. verification itself could not execute because its contract was invalid.
//!
//! Class 1 is represented by `VerificationReport`.
//!
//! Class 2 is represented by `ScheduleVerificationError`.
//!
//! This distinction is important for production systems because a rejected
//! schedule and a broken verifier are operationally different events.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Safety
//!
//! This module intentionally contains no unsafe code.
//!
//! ```rust
//! #![forbid(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! ```
//!
//! # Finish-once integration rule
//!
//! This file depends on verification-domain traits rather than concrete
//! scheduler implementations.
//!
//! Consequently:
//!
//! - changing the scheduler's storage does not require changing this file;
//! - adding a new hardware backend does not require changing this file;
//! - adding a new routing algorithm does not require changing this file;
//! - adding a new timing representation does not require changing this file;
//! - adding a new resource type does not require changing this file.
//!
//! New verification domains are integrated by implementing the verifier-domain
//! contract and registering the domain at the composition boundary.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

use std::fmt;

// =============================================================================
// Verification domain
// =============================================================================

/// Canonical verification domains supplied by the scheduler.
///
/// The ordering of these variants is the default production execution order.
/// It is deliberately explicit so diagnostics remain deterministic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerificationDomain {
    /// Verifies schedule membership, operation identity, and structural
    /// integrity.
    Structural,

    /// Verifies operation dependency ordering.
    Dependency,

    /// Verifies resource capacity and reservation correctness.
    Resource,

    /// Verifies temporal constraints, durations, windows, and alignment.
    Timing,

    /// Verifies preservation of program semantics.
    Semantic,
}

impl VerificationDomain {
    /// Returns the canonical production ordering key.
    #[must_use]
    pub const fn order(self) -> u8 {
        match self {
            Self::Structural => 0,
            Self::Dependency => 1,
            Self::Resource => 2,
            Self::Timing => 3,
            Self::Semantic => 4,
        }
    }

    /// Returns the stable domain name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Structural => "structural",
            Self::Dependency => "dependency",
            Self::Resource => "resource",
            Self::Timing => "timing",
            Self::Semantic => "semantic",
        }
    }
}

impl fmt::Display for VerificationDomain {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Severity
// =============================================================================

/// Severity of a verification finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerificationSeverity {
    /// Informational result that does not invalidate the schedule.
    Info,

    /// Non-fatal finding explicitly permitted by policy.
    Warning,

    /// Finding that invalidates a mandatory verification contract.
    Error,
}

impl VerificationSeverity {
    /// Returns whether this severity invalidates production acceptance.
    #[must_use]
    pub const fn is_failure(self) -> bool {
        matches!(self, Self::Error)
    }
}

impl fmt::Display for VerificationSeverity {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Info => formatter.write_str("info"),
            Self::Warning => formatter.write_str("warning"),
            Self::Error => formatter.write_str("error"),
        }
    }
}

// =============================================================================
// Finding
// =============================================================================

/// A normalized verification finding.
///
/// Domain-specific verification modules may retain their native report types.
/// This normalized representation is used by the orchestration layer for
/// cross-domain reporting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationFinding {
    /// Verification domain that produced the finding.
    pub domain: VerificationDomain,

    /// Severity assigned by the verification policy.
    pub severity: VerificationSeverity,

    /// Stable machine-readable finding code.
    pub code: String,

    /// Human-readable explanation.
    pub message: String,

    /// Optional operation identity associated with the finding.
    pub operation: Option<String>,

    /// Optional resource identity associated with the finding.
    pub resource: Option<String>,
}

impl VerificationFinding {
    /// Creates a verification finding.
    #[must_use]
    pub fn new(
        domain: VerificationDomain,
        severity: VerificationSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            domain,
            severity,
            code: code.into(),
            message: message.into(),
            operation: None,
            resource: None,
        }
    }

    /// Associates an operation with the finding.
    #[must_use]
    pub fn with_operation(
        mut self,
        operation: impl Into<String>,
    ) -> Self {
        self.operation = Some(operation.into());
        self
    }

    /// Associates a resource with the finding.
    #[must_use]
    pub fn with_resource(
        mut self,
        resource: impl Into<String>,
    ) -> Self {
        self.resource = Some(resource.into());
        self
    }

    /// Returns whether this finding is fatal.
    #[must_use]
    pub const fn is_failure(&self) -> bool {
        self.severity.is_failure()
    }
}

// =============================================================================
// Domain report
// =============================================================================

/// Result produced by one verification domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationDomainReport {
    /// Domain represented by this report.
    pub domain: VerificationDomain,

    /// Whether this domain passed.
    pub valid: bool,

    /// Number of items examined by the domain.
    pub items_checked: usize,

    /// Number of violations found.
    pub violations: usize,

    /// Domain-specific findings normalized for the unified report.
    pub findings: Vec<VerificationFinding>,
}

impl VerificationDomainReport {
    /// Creates a successful empty domain report.
    #[must_use]
    pub fn success(
        domain: VerificationDomain,
    ) -> Self {
        Self {
            domain,
            valid: true,
            items_checked: 0,
            violations: 0,
            findings: Vec::new(),
        }
    }

    /// Creates a domain report from findings.
    #[must_use]
    pub fn from_findings(
        domain: VerificationDomain,
        items_checked: usize,
        findings: Vec<VerificationFinding>,
    ) -> Self {
        let violations = findings
            .iter()
            .filter(|finding| finding.is_failure())
            .count();

        Self {
            domain,
            valid: violations == 0,
            items_checked,
            violations,
            findings,
        }
    }

    /// Returns whether this domain passed.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.valid
    }

    /// Returns whether this domain failed.
    #[must_use]
    pub const fn has_failures(&self) -> bool {
        !self.valid
    }
}

// =============================================================================
// Overall status
// =============================================================================

/// Final result of unified schedule verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationStatus {
    /// Every mandatory verification domain passed.
    Accepted,

    /// Verification completed but one or more mandatory domains failed.
    Rejected,

    /// Verification was intentionally not completed.
    ///
    /// This status must never be interpreted as successful verification.
    Incomplete,
}

impl VerificationStatus {
    /// Returns whether the status is safe for production execution.
    #[must_use]
    pub const fn is_accepted(self) -> bool {
        matches!(self, Self::Accepted)
    }

    /// Returns whether verification rejected the schedule.
    #[must_use]
    pub const fn is_rejected(self) -> bool {
        matches!(self, Self::Rejected)
    }

    /// Returns whether verification was incomplete.
    #[must_use]
    pub const fn is_incomplete(self) -> bool {
        matches!(self, Self::Incomplete)
    }
}

impl fmt::Display for VerificationStatus {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Accepted => formatter.write_str("accepted"),
            Self::Rejected => formatter.write_str("rejected"),
            Self::Incomplete => formatter.write_str("incomplete"),
        }
    }
}

// =============================================================================
// Verification configuration
// =============================================================================

/// Configuration for unified schedule verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationConfig {
    /// Whether verification should stop after the first fatal finding.
    pub fail_fast: bool,

    /// Whether all standard production domains are required.
    pub require_all_standard_domains: bool,

    /// Whether warnings should be retained in the final report.
    pub include_warnings: bool,

    /// Whether informational findings should be retained.
    pub include_info: bool,

    /// Whether an empty domain result is allowed.
    ///
    /// This controls orchestration only. A domain remains responsible for
    /// determining whether an empty input is valid.
    pub allow_empty_domain_report: bool,
}

impl Default for VerificationConfig {
    fn default() -> Self {
        Self {
            fail_fast: false,
            require_all_standard_domains: true,
            include_warnings: true,
            include_info: true,
            allow_empty_domain_report: true,
        }
    }
}

impl VerificationConfig {
    /// Creates the strict production configuration.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            fail_fast: false,
            require_all_standard_domains: true,
            include_warnings: true,
            include_info: true,
            allow_empty_domain_report: true,
        }
    }

    /// Creates a fail-fast production configuration.
    #[must_use]
    pub const fn production_fail_fast() -> Self {
        Self {
            fail_fast: true,
            require_all_standard_domains: true,
            include_warnings: true,
            include_info: true,
            allow_empty_domain_report: true,
        }
    }
}

// =============================================================================
// Unified report
// =============================================================================

/// Complete unified verification report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationReport {
    /// Final production status.
    pub status: VerificationStatus,

    /// Domain reports in canonical verification order.
    pub domains: Vec<VerificationDomainReport>,

    /// Total number of items examined.
    pub items_checked: usize,

    /// Total fatal violations.
    pub violation_count: usize,

    /// Total warnings retained in the report.
    pub warning_count: usize,

    /// Total informational findings retained in the report.
    pub info_count: usize,

    /// All retained findings in deterministic domain order.
    pub findings: Vec<VerificationFinding>,
}

impl VerificationReport {
    /// Creates an empty incomplete report.
    #[must_use]
    pub fn incomplete() -> Self {
        Self {
            status: VerificationStatus::Incomplete,
            domains: Vec::new(),
            items_checked: 0,
            violation_count: 0,
            warning_count: 0,
            info_count: 0,
            findings: Vec::new(),
        }
    }

    /// Returns whether the schedule passed every required verification stage.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.status.is_accepted()
    }

    /// Returns whether the schedule is safe to submit to execution.
    ///
    /// This intentionally aliases `is_valid()` rather than merely checking
    /// whether verification completed.
    #[must_use]
    pub const fn is_execution_safe(&self) -> bool {
        self.status.is_accepted()
    }

    /// Returns whether verification rejected the schedule.
    #[must_use]
    pub const fn is_rejected(&self) -> bool {
        self.status.is_rejected()
    }

    /// Returns whether verification did not complete.
    #[must_use]
    pub const fn is_incomplete(&self) -> bool {
        self.status.is_incomplete()
    }

    /// Returns whether at least one fatal violation exists.
    #[must_use]
    pub const fn has_violations(&self) -> bool {
        self.violation_count != 0
    }

    /// Returns the report for a particular domain.
    #[must_use]
    pub fn domain(
        &self,
        domain: VerificationDomain,
    ) -> Option<&VerificationDomainReport> {
        self.domains
            .iter()
            .find(|report| report.domain == domain)
    }
}

// =============================================================================
// Verification execution error
// =============================================================================

/// Failure to execute the verification pipeline itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduleVerificationError {
    /// A required verification domain was not registered.
    MissingRequiredDomain {
        /// Domain that was required but unavailable.
        domain: VerificationDomain,
    },

    /// A domain returned a report for a different domain.
    DomainMismatch {
        /// Domain that was requested.
        expected: VerificationDomain,

        /// Domain reported by the implementation.
        observed: VerificationDomain,
    },

    /// A domain returned an invalid report.
    InvalidDomainReport {
        /// Domain that produced the invalid report.
        domain: VerificationDomain,

        /// Explanation of the contract violation.
        reason: String,
    },

    /// A verification implementation could not execute.
    ExecutionFailure {
        /// Domain that failed to execute.
        domain: VerificationDomain,

        /// Stable diagnostic.
        reason: String,
    },

    /// A duplicate verifier domain was registered.
    DuplicateDomain {
        /// Domain registered more than once.
        domain: VerificationDomain,
    },
}

impl fmt::Display for ScheduleVerificationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::MissingRequiredDomain { domain } => {
                write!(
                    formatter,
                    "required verification domain '{domain}' is not registered"
                )
            }

            Self::DomainMismatch {
                expected,
                observed,
            } => {
                write!(
                    formatter,
                    "verification domain mismatch: expected '{expected}', \
                     observed '{observed}'"
                )
            }

            Self::InvalidDomainReport {
                domain,
                reason,
            } => {
                write!(
                    formatter,
                    "verification domain '{domain}' returned an invalid \
                     report: {reason}"
                )
            }

            Self::ExecutionFailure {
                domain,
                reason,
            } => {
                write!(
                    formatter,
                    "verification domain '{domain}' failed to execute: {reason}"
                )
            }

            Self::DuplicateDomain { domain } => {
                write!(
                    formatter,
                    "verification domain '{domain}' was registered more \
                     than once"
                )
            }
        }
    }
}

impl std::error::Error for ScheduleVerificationError {}

// =============================================================================
// Domain verifier trait
// =============================================================================

/// Read-only verification implementation for one verification domain.
///
/// This trait is intentionally generic over the schedule input type.
///
/// The concrete scheduler may therefore use:
///
/// - `ScheduleResult`;
/// - an immutable schedule view;
/// - an arena-backed schedule;
/// - a graph-backed schedule;
/// - a distributed schedule;
/// - another future representation.
///
/// The unified verifier never depends on the storage representation.
///
/// # Contract
///
/// Implementations MUST:
///
/// - be read-only;
/// - return a report whose domain matches `domain()`;
/// - never silently treat invalid input as valid;
/// - avoid architectural machine limits;
/// - avoid hidden randomness;
/// - produce deterministic findings for deterministic input;
/// - not mutate caller-owned state.
///
/// Implementations MAY use internal parallelism, provided their returned
/// findings are deterministic after normalization.
pub trait ScheduleDomainVerifier<S> {
    /// Returns the verification domain implemented by this verifier.
    fn domain(&self) -> VerificationDomain;

    /// Verifies the supplied schedule.
    fn verify(
        &self,
        schedule: &S,
    ) -> Result<VerificationDomainReport, ScheduleVerificationError>;
}

// =============================================================================
// Closure-backed verifier
// =============================================================================

/// Closure-backed domain verifier.
///
/// This adapter makes it possible to integrate an existing verification
/// function without creating a dedicated wrapper type.
///
/// It is particularly useful during incremental construction of the scheduler.
pub struct FnDomainVerifier<S, F>
where
    F: Fn(
        &S,
    ) -> Result<VerificationDomainReport, ScheduleVerificationError>,
{
    domain: VerificationDomain,
    function: F,
    marker: std::marker::PhantomData<fn(&S)>,
}

impl<S, F> FnDomainVerifier<S, F>
where
    F: Fn(
        &S,
    ) -> Result<VerificationDomainReport, ScheduleVerificationError>,
{
    /// Creates a closure-backed domain verifier.
    #[must_use]
    pub fn new(
        domain: VerificationDomain,
        function: F,
    ) -> Self {
        Self {
            domain,
            function,
            marker: std::marker::PhantomData,
        }
    }
}

impl<S, F> ScheduleDomainVerifier<S> for FnDomainVerifier<S, F>
where
    F: Fn(
        &S,
    ) -> Result<VerificationDomainReport, ScheduleVerificationError>,
{
    fn domain(&self) -> VerificationDomain {
        self.domain
    }

    fn verify(
        &self,
        schedule: &S,
    ) -> Result<VerificationDomainReport, ScheduleVerificationError> {
        (self.function)(schedule)
    }
}

// =============================================================================
// Unified verifier
// =============================================================================

/// Unified production schedule verifier.
///
/// `ScheduleVerifier` is the composition root for schedule verification.
///
/// It does not know how individual verification domains work.
///
/// This is the key property that lets Zamani evolve from:
///
/// ```text
/// small single-QPU schedule
/// ```
///
/// to:
///
/// ```text
/// large multi-QPU schedule
/// ```
///
/// without changing the orchestration contract.
pub struct ScheduleVerifier<S> {
    config: VerificationConfig,
    domains: Vec<Box<dyn ScheduleDomainVerifier<S>>>,
}

impl<S> ScheduleVerifier<S> {
    /// Creates an empty verifier with the supplied configuration.
    ///
    /// Domains must be registered before production verification.
    #[must_use]
    pub fn new(
        config: VerificationConfig,
    ) -> Self {
        Self {
            config,
            domains: Vec::new(),
        }
    }

    /// Returns the verifier configuration.
    #[must_use]
    pub const fn config(&self) -> &VerificationConfig {
        &self.config
    }

    /// Returns the number of registered verification domains.
    #[must_use]
    pub fn domain_count(&self) -> usize {
        self.domains.len()
    }

    /// Registers one verification domain.
    ///
    /// Duplicate domains are rejected immediately.
    pub fn register(
        &mut self,
        verifier: Box<dyn ScheduleDomainVerifier<S>>,
    ) -> Result<(), ScheduleVerificationError> {
        let domain = verifier.domain();

        if self
            .domains
            .iter()
            .any(|existing| existing.domain() == domain)
        {
            return Err(ScheduleVerificationError::DuplicateDomain { domain });
        }

        self.domains.push(verifier);

        self.domains.sort_by_key(|entry| entry.domain().order());

        Ok(())
    }

    /// Registers a closure-backed verification domain.
    pub fn register_fn<F>(
        &mut self,
        domain: VerificationDomain,
        function: F,
    ) -> Result<(), ScheduleVerificationError>
    where
        F: Fn(
                &S,
            )
                -> Result<VerificationDomainReport, ScheduleVerificationError>
            + 'static,
    {
        self.register(Box::new(FnDomainVerifier::new(
            domain,
            function,
        )))
    }

    /// Verifies a schedule through every registered verification domain.
    ///
    /// The method is read-only.
    ///
    /// If `fail_fast` is enabled, verification stops after the first fatal
    /// finding.
    ///
    /// Otherwise all registered domains are executed, allowing callers to
    /// receive a complete diagnostic report.
    pub fn verify(
        &self,
        schedule: &S,
    ) -> Result<VerificationReport, ScheduleVerificationError> {
        self.validate_required_domains()?;

        let mut report = VerificationReport::incomplete();

        for verifier in &self.domains {
            let expected_domain = verifier.domain();

            let domain_report = verifier.verify(schedule)?;

            self.validate_domain_report(
                expected_domain,
                &domain_report,
            )?;

            self.merge_domain_report(
                &mut report,
                domain_report,
            );

            if self.config.fail_fast
                && report.violation_count != 0
            {
                break;
            }
        }

        report.status = if report.violation_count != 0 {
            VerificationStatus::Rejected
        } else if self.config.require_all_standard_domains
            && !self.has_all_standard_domains()
        {
            VerificationStatus::Incomplete
        } else {
            VerificationStatus::Accepted
        };

        Ok(report)
    }

    /// Verifies a schedule and returns an error when it is not production-safe.
    ///
    /// This is the preferred entry point immediately before hardware/runtime
    /// submission.
    pub fn verify_for_execution(
        &self,
        schedule: &S,
    ) -> Result<VerificationReport, ScheduleVerificationError> {
        let report = self.verify(schedule)?;

        if !report.is_execution_safe() {
            return Err(
                ScheduleVerificationError::InvalidDomainReport {
                    domain: VerificationDomain::Structural,
                    reason: format!(
                        "schedule verification status is '{}'; execution \
                         requires accepted verification",
                        report.status
                    ),
                },
            );
        }

        Ok(report)
    }

    /// Returns whether all standard production domains are registered.
    #[must_use]
    pub fn has_all_standard_domains(&self) -> bool {
        const STANDARD: [VerificationDomain; 5] = [
            VerificationDomain::Structural,
            VerificationDomain::Dependency,
            VerificationDomain::Resource,
            VerificationDomain::Timing,
            VerificationDomain::Semantic,
        ];

        STANDARD.iter().all(|required| {
            self.domains
                .iter()
                .any(|registered| registered.domain() == *required)
        })
    }

    /// Returns whether a domain is registered.
    #[must_use]
    pub fn has_domain(
        &self,
        domain: VerificationDomain,
    ) -> bool {
        self.domains
            .iter()
            .any(|registered| registered.domain() == domain)
    }

    fn validate_required_domains(
        &self,
    ) -> Result<(), ScheduleVerificationError> {
        if !self.config.require_all_standard_domains {
            return Ok(());
        }

        const STANDARD: [VerificationDomain; 5] = [
            VerificationDomain::Structural,
            VerificationDomain::Dependency,
            VerificationDomain::Resource,
            VerificationDomain::Timing,
            VerificationDomain::Semantic,
        ];

        for domain in STANDARD {
            if !self.has_domain(domain) {
                return Err(
                    ScheduleVerificationError::MissingRequiredDomain {
                        domain,
                    },
                );
            }
        }

        Ok(())
    }

    fn validate_domain_report(
        &self,
        expected: VerificationDomain,
        report: &VerificationDomainReport,
    ) -> Result<(), ScheduleVerificationError> {
        if expected != report.domain {
            return Err(
                ScheduleVerificationError::DomainMismatch {
                    expected,
                    observed: report.domain,
                },
            );
        }

        if !self.config.allow_empty_domain_report
            && report.items_checked == 0
        {
            return Err(
                ScheduleVerificationError::InvalidDomainReport {
                    domain: expected,
                    reason: String::from(
                        "empty verification report is not permitted by policy",
                    ),
                },
            );
        }

        let calculated_violations = report
            .findings
            .iter()
            .filter(|finding| finding.is_failure())
            .count();

        if calculated_violations != report.violations {
            return Err(
                ScheduleVerificationError::InvalidDomainReport {
                    domain: expected,
                    reason: format!(
                        "reported violation count {} does not match \
                         normalized finding count {}",
                        report.violations,
                        calculated_violations
                    ),
                },
            );
        }

        if report.valid && calculated_violations != 0 {
            return Err(
                ScheduleVerificationError::InvalidDomainReport {
                    domain: expected,
                    reason: String::from(
                        "domain reports valid despite containing fatal findings",
                    ),
                },
            );
        }

        if !report.valid && calculated_violations == 0 {
            return Err(
                ScheduleVerificationError::InvalidDomainReport {
                    domain: expected,
                    reason: String::from(
                        "domain reports invalid without a fatal finding",
                    ),
                },
            );
        }

        Ok(())
    }

    fn merge_domain_report(
        &self,
        aggregate: &mut VerificationReport,
        mut domain: VerificationDomainReport,
    ) {
        domain.findings.retain(|finding| {
            match finding.severity {
                VerificationSeverity::Info => self.config.include_info,
                VerificationSeverity::Warning => self.config.include_warnings,
                VerificationSeverity::Error => true,
            }
        });

        aggregate.items_checked = aggregate
            .items_checked
            .saturating_add(domain.items_checked);

        aggregate.violation_count = aggregate
            .violation_count
            .saturating_add(
                domain
                    .findings
                    .iter()
                    .filter(|finding| finding.is_failure())
                    .count(),
            );

        aggregate.warning_count = aggregate
            .warning_count
            .saturating_add(
                domain
                    .findings
                    .iter()
                    .filter(|finding| {
                        finding.severity
                            == VerificationSeverity::Warning
                    })
                    .count(),
            );

        aggregate.info_count = aggregate
            .info_count
            .saturating_add(
                domain
                    .findings
                    .iter()
                    .filter(|finding| {
                        finding.severity
                            == VerificationSeverity::Info
                    })
                    .count(),
            );

        aggregate
            .findings
            .extend(domain.findings);

        aggregate.domains.push(domain);
    }
}

// =============================================================================
// Standard domain adapter helpers
// =============================================================================

/// Converts a boolean verification result into a normalized domain report.
#[must_use]
pub fn boolean_domain_report(
    domain: VerificationDomain,
    items_checked: usize,
    valid: bool,
    failure_code: impl Into<String>,
    failure_message: impl Into<String>,
) -> VerificationDomainReport {
    if valid {
        return VerificationDomainReport {
            domain,
            valid: true,
            items_checked,
            violations: 0,
            findings: Vec::new(),
        };
    }

    let finding = VerificationFinding::new(
        domain,
        VerificationSeverity::Error,
        failure_code,
        failure_message,
    );

    VerificationDomainReport {
        domain,
        valid: false,
        items_checked,
        violations: 1,
        findings: vec![finding],
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestSchedule {
        valid: bool,
    }

    fn successful_domain(
        domain: VerificationDomain,
    ) -> FnDomainVerifier<
        TestSchedule,
        impl Fn(
            &TestSchedule,
        ) -> Result<
            VerificationDomainReport,
            ScheduleVerificationError,
        >,
    > {
        FnDomainVerifier::new(
            domain,
            move |schedule: &TestSchedule| {
                Ok(boolean_domain_report(
                    domain,
                    1,
                    schedule.valid,
                    "TEST_INVALID",
                    "test schedule is invalid",
                ))
            },
        )
    }

    fn register_standard_domains(
        verifier: &mut ScheduleVerifier<TestSchedule>,
    ) {
        verifier
            .register(Box::new(successful_domain(
                VerificationDomain::Structural,
            )))
            .expect("structural registration must succeed");

        verifier
            .register(Box::new(successful_domain(
                VerificationDomain::Dependency,
            )))
            .expect("dependency registration must succeed");

        verifier
            .register(Box::new(successful_domain(
                VerificationDomain::Resource,
            )))
            .expect("resource registration must succeed");

        verifier
            .register(Box::new(successful_domain(
                VerificationDomain::Timing,
            )))
            .expect("timing registration must succeed");

        verifier
            .register(Box::new(successful_domain(
                VerificationDomain::Semantic,
            )))
            .expect("semantic registration must succeed");
    }

    #[test]
    fn production_requires_all_standard_domains() {
        let verifier =
            ScheduleVerifier::<TestSchedule>::new(
                VerificationConfig::production(),
            );

        let result = verifier.verify(&TestSchedule {
            valid: true,
        });

        assert!(matches!(
            result,
            Err(
                ScheduleVerificationError::MissingRequiredDomain {
                    ..
                }
            )
        ));
    }

    #[test]
    fn valid_schedule_is_accepted() {
        let mut verifier =
            ScheduleVerifier::<TestSchedule>::new(
                VerificationConfig::production(),
            );

        register_standard_domains(&mut verifier);

        let report = verifier
            .verify(&TestSchedule {
                valid: true,
            })
            .expect("verification should execute");

        assert_eq!(
            report.status,
            VerificationStatus::Accepted
        );
        assert!(report.is_execution_safe());
        assert_eq!(report.domains.len(), 5);
    }

    #[test]
    fn invalid_schedule_is_rejected() {
        let mut verifier =
            ScheduleVerifier::<TestSchedule>::new(
                VerificationConfig::production(),
            );

        register_standard_domains(&mut verifier);

        let report = verifier
            .verify(&TestSchedule {
                valid: false,
            })
            .expect("verification should execute");

        assert_eq!(
            report.status,
            VerificationStatus::Rejected
        );
        assert!(!report.is_execution_safe());
        assert!(report.violation_count > 0);
    }

    #[test]
    fn duplicate_domains_are_rejected() {
        let mut verifier =
            ScheduleVerifier::<TestSchedule>::new(
                VerificationConfig::production(),
            );

        verifier
            .register(Box::new(successful_domain(
                VerificationDomain::Structural,
            )))
            .expect("first registration should succeed");

        let result = verifier.register(Box::new(
            successful_domain(
                VerificationDomain::Structural,
            ),
        ));

        assert!(matches!(
            result,
            Err(
                ScheduleVerificationError::DuplicateDomain {
                    domain: VerificationDomain::Structural,
                }
            )
        ));
    }

    #[test]
    fn domains_are_sorted_deterministically() {
        let mut verifier =
            ScheduleVerifier::<TestSchedule>::new(
                VerificationConfig {
                    require_all_standard_domains: false,
                    ..VerificationConfig::production()
                },
            );

        verifier
            .register(Box::new(successful_domain(
                VerificationDomain::Semantic,
            )))
            .expect("semantic registration should succeed");

        verifier
            .register(Box::new(successful_domain(
                VerificationDomain::Structural,
            )))
            .expect("structural registration should succeed");

        verifier
            .register(Box::new(successful_domain(
                VerificationDomain::Timing,
            )))
            .expect("timing registration should succeed");

        let report = verifier
            .verify(&TestSchedule {
                valid: true,
            })
            .expect("verification should execute");

        assert_eq!(
            report.domains[0].domain,
            VerificationDomain::Structural
        );
        assert_eq!(
            report.domains[1].domain,
            VerificationDomain::Timing
        );
        assert_eq!(
            report.domains[2].domain,
            VerificationDomain::Semantic
        );
    }

    #[test]
    fn fail_fast_stops_after_failure() {
        let mut verifier =
            ScheduleVerifier::<TestSchedule>::new(
                VerificationConfig {
                    fail_fast: true,
                    require_all_standard_domains: false,
                    ..VerificationConfig::production()
                },
            );

        register_standard_domains(&mut verifier);

        let report = verifier
            .verify(&TestSchedule {
                valid: false,
            })
            .expect("verification should execute");

        assert_eq!(
            report.status,
            VerificationStatus::Rejected
        );

        assert_eq!(report.domains.len(), 1);
        assert_eq!(
            report.domains[0].domain,
            VerificationDomain::Structural
        );
    }

    #[test]
    fn execution_requires_acceptance() {
        let mut verifier =
            ScheduleVerifier::<TestSchedule>::new(
                VerificationConfig::production(),
            );

        register_standard_domains(&mut verifier);

        let result = verifier.verify_for_execution(
            &TestSchedule {
                valid: false,
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn boolean_helper_reports_success() {
        let report = boolean_domain_report(
            VerificationDomain::Timing,
            10,
            true,
            "UNUSED",
            "UNUSED",
        );

        assert!(report.is_valid());
        assert_eq!(report.violations, 0);
        assert!(report.findings.is_empty());
    }

    #[test]
    fn boolean_helper_reports_failure() {
        let report = boolean_domain_report(
            VerificationDomain::Timing,
            10,
            false,
            "TIMING_INVALID",
            "timing is invalid",
        );

        assert!(!report.is_valid());
        assert_eq!(report.violations, 1);
        assert_eq!(
            report.findings[0].code,
            "TIMING_INVALID"
        );
    }
}