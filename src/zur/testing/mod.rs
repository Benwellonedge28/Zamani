
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Testing
//!
//! Path: `src/zur/testing/mod.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//!
//! # Purpose
//!
//! This module is the public composition root for ZUR testing infrastructure.
//!
//! It exposes the existing, independently implemented testing systems:
//!
//! - `provider_harness`: Operational provider integration probes.
//! - `conformance`: Aggregate structural and operational conformance.
//! - `portability`: Cross-environment execution portability testing.
//! - `scalability`: Resource-parametric scalability testing.
//! - `fault_injection`: Explicit fault-injection and fault-response testing.
//!
//! This module establishes stable public module paths and selective
//! re-exports. It does not implement, duplicate, or replace the
//! responsibilities of its child modules.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Testing namespace composition.
//! - Public testing-module organization.
//! - Stable public re-export paths.
//! - Testing architecture documentation.
//!
//! The child modules retain exclusive ownership of their implementations.
//!
//! This module does NOT own:
//!
//! - Provider identity or lifecycle.
//! - Provider registration or discovery.
//! - Domain identity or domain semantics.
//! - Capability definitions or capability evidence.
//! - Requirements, preferences, constraints or hints.
//! - Feasibility assessment.
//! - Resource allocation or reservation.
//! - Hardware discovery or selection.
//! - Compiler planning or optimization.
//! - Backend registration or code generation.
//! - Classical, quantum, HDL, AI or tensor IR.
//! - ZUIR.
//! - Hybrid execution planning.
//! - Distributed execution planning.
//! - Scheduling, routing or placement.
//! - Physical execution.
//! - Operational fault classification.
//! - Quantum fault semantics or quantum error correction.
//! - Production recovery execution.
//!
//! These responsibilities remain with their canonical repository owners.
//!
//! # Canonical integration authorities
//!
//! ## ZUR
//!
//! - `crate::zur::error`
//! - `crate::zur::version`
//! - `crate::zur::contracts`
//! - `crate::zur::domains`
//! - `crate::zur::execution`
//! - `crate::zur::reliability`
//!
//! ## Standard library
//!
//! - `crate::stdlib::context`
//! - `crate::stdlib::provider`
//! - `crate::stdlib::capability`
//! - `crate::stdlib::requirement`
//! - `crate::stdlib::feasibility`
//!
//! ## Compiler and execution
//!
//! - `crate::compiler`
//! - `crate::backend`
//! - `crate::zuir`
//! - `crate::classical`
//! - `crate::quantum`
//! - `crate::hdl`
//! - `crate::ai`
//! - `crate::hybrid`
//! - `crate::distributed`
//!
//! The testing modules consume these authorities through their
//! established contracts. They must not introduce competing versions.
//!
//! # Testing-module responsibilities
//!
//! | Module | Responsibility |
//! |---|---|
//! | `provider_harness` | Explicit provider invocation probes and response validation |
//! | `conformance` | Aggregate provider conformance assessment and report validation |
//! | `portability` | Cross-environment execution and semantic comparison |
//! | `scalability` | Scale-axis testing, streaming orchestration and checked accounting |
//! | `fault_injection` | Explicit fault injection and independently verified fault response |
//!
//! The modules are independently usable. A caller can use one testing
//! facility without initializing or executing the others.
//!
//! # Fundamental distinctions
//!
//! Testing results must preserve the following distinctions:
//!
//! - Structural conformance != operational conformance.
//! - Provider registration != provider initialization.
//! - Provider initialization != provider availability.
//! - Provider availability != capability availability.
//! - Capability availability != resource reservation.
//! - Requirement satisfaction != feasibility.
//! - Feasibility != resource reservation.
//! - Resource reservation != execution.
//! - Execution submission != successful completion.
//! - Fault injection != fault-response verification.
//! - Fault observation != recovery.
//! - Recovery request != verified recovery.
//! - Execution completion != semantic equivalence.
//! - A passing test != proof of untested behavior.
//!
//! No testing report may claim evidence that its underlying test did
//! not actually establish.
//!
//! # Grammar and semantic boundary
//!
//! This module does not parse Zamani source code or grammar productions.
//!
//! Source-level intent and normative language behavior remain owned by
//! the canonical grammar and specification hierarchy.
//!
//! Relevant authorities include:
//!
//! - `grammar/specification/execution-model.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/domains.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/execution/resilience.g4`
//! - `grammar/distributed/fault-tolerance.g4`
//! - `grammar/resources/scalability.g4`
//! - `grammar/validation/validation.g4`
//!
//! These files define language-level syntax, semantics and requirements.
//!
//! The ZUR testing infrastructure evaluates implementations of already
//! established contracts. It must not reinterpret source declarations,
//! create AST nodes, or become another semantic-analysis subsystem.
//!
//! # Open-world and scalability requirements
//!
//! The testing composition introduces no fixed maximum for:
//!
//! - Providers.
//! - Computational domains.
//! - Test cases.
//! - Test scenarios.
//! - Test findings.
//! - Scale axes.
//! - Scale points.
//! - Execution environments.
//! - Execution sessions.
//! - Fault models.
//! - Devices.
//! - Distributed participants.
//! - Resource quantities.
//!
//! No closed computational-domain enumeration is introduced.
//!
//! Collections grow subject to actual allocator capacity, representation
//! limits, implementation capabilities and available system resources.
//!
//! Configured operational budgets are permitted when explicitly supplied
//! by the caller. They must not become universal language or hardware
//! capacity limits.
//!
//! This architecture does not claim literal infinite physical storage
//! or computation.
//!
//! # Safety and side-effect requirements
//!
//! All testing implementations must use safe Rust.
//!
//! No unsafe code is permitted.
//!
//! No global mutable testing state is permitted.
//!
//! Testing must not silently:
//!
//! - Initialize providers.
//! - Discover or select hardware.
//! - Reserve physical resources.
//! - Execute arbitrary provider operations.
//! - Access filesystems or networks.
//! - Inject faults into production systems.
//! - Retry operations with unknown side effects.
//! - Treat inconclusive observations as successful.
//! - Treat a cancellation request as proof of termination.
//!
//! Operations with external effects require explicit caller-supplied
//! implementations and authorization in the relevant environment.
//!
//! # Determinism and evidence
//!
//! Each child module defines its own report-ordering and evidence rules.
//!
//! This composition root does not reorder findings or alter observations.
//!
//! A report must retain the distinction between:
//!
//! - Passed.
//! - Failed.
//! - Unsupported.
//! - Inconclusive.
//! - Not run.
//! - Cancelled.
//! - Outcome unknown.
//!
//! Missing evidence must not be converted into a passing result.
//!
//! # Integration contract
//!
//! Upstream:
//!
//! - `zur/error.rs`
//! - `zur/contracts/`
//! - `zur/domains/`
//! - `zur/execution/`
//! - `zur/reliability/`
//! - `stdlib/context.rs`
//! - `stdlib/provider.rs`
//! - `compiler/`
//! - `backend/`
//! - `hybrid/`
//! - `distributed/`
//! - `quantum/`
//!
//! Downstream:
//!
//! - ZUR provider implementations.
//! - Domain-provider conformance suites.
//! - Runtime portability suites.
//! - Compiler scalability tests.
//! - Hybrid integration tests.
//! - Distributed integration tests.
//! - Quantum resilience tests.
//! - Hardware and accelerator provider tests.
//! - CI and external provider conformance suites.
//!
//! New testing facilities should be added as independent child modules
//! only when their ownership and integration contracts are established.
//! They must not be implemented inside this composition root.

// ============================================================================
// TESTING MODULES
// ============================================================================

/// Aggregate structural and operational provider conformance testing.
///
/// Owns conformance-suite orchestration, deterministic report aggregation,
/// duplicate provider identity detection and report validation.
///
/// Structural conformance is delegated to the canonical ZUR domain
/// conformance implementation.
pub mod conformance;

/// Explicit fault-injection scenarios, target contracts, observations,
/// verification contracts and partial-result reports.
///
/// Canonical operational fault classification remains owned by
/// `crate::zur::reliability::faults`.
pub mod fault_injection;

/// Cross-environment portability testing.
///
/// Owns test environments, execution-mode descriptions, execution
/// observations, semantic comparison policies and portability reports.
///
/// Actual execution is delegated to explicitly supplied adapters.
pub mod portability;

/// Explicit provider integration probes.
///
/// Owns probe definitions, operational probe execution and response
/// validation.
///
/// Provider identity, provider lifecycle and structural domain
/// conformance remain owned by their canonical modules.
pub mod provider_harness;

/// Resource-parametric scalability testing.
///
/// Owns scale-axis descriptions, scale-point observations, streaming
/// test orchestration, checked accounting and scalability reports.
///
/// Resource discovery, feasibility, reservation and scheduling remain
/// owned by their canonical subsystems.
pub mod scalability;

// ============================================================================
// PUBLIC CONFORMANCE API
// ============================================================================

/// Public conformance-testing API.
pub use conformance::{
    assess_provider_conformance,
    assess_provider_conformances,
    ConformanceSuite,
    ConformanceSuiteReport,
};

// ============================================================================
// PUBLIC FAULT-INJECTION API
// ============================================================================

/// Public fault-injection testing API.
pub use fault_injection::{
    validate_scenario_id,
    FaultInjectionFailureStage,
    FaultInjectionFinding,
    FaultInjectionPlan,
    FaultInjectionReport,
    FaultInjectionRunError,
    FaultInjectionRunResult,
    FaultInjectionStep,
    FaultInjectionTarget,
    InjectionDisposition,
    InjectionObservation,
    VerificationDisposition,
    VerificationObservation,
    FaultInjectionVerifier,
};

// ============================================================================
// PUBLIC PORTABILITY API
// ============================================================================

/// Public portability-testing API.
pub use portability::{
    validate_portability_identity,
    ExactOutputComparison,
    NeverCancelled,
    PortabilityCancellation,
    PortabilityComparison,
    PortabilityComparisonPolicy,
    PortabilityEnvironment,
    PortabilityEnvironmentId,
    PortabilityError,
    PortabilityExecutionAdapter,
    PortabilityExecutionMode,
    PortabilityExecutionRecord,
    PortabilityExecutionRequest,
    PortabilityExecutionStatus,
    PortabilityFinding,
    PortabilityObservation,
    PortabilityReport,
    PortabilityResult,
    PortabilityTestCase,
    PortabilityTestCaseId,
    PortabilityTester,
};

// ============================================================================
// PUBLIC PROVIDER-HARNESS API
// ============================================================================

/// Public provider integration-testing API.
pub use provider_harness::{
    validate_probe_id,
    ProbeOutcome,
    ProviderHarness,
    ProviderProbe,
    ProviderProbeFinding,
    ProviderProbeReport,
};

// ============================================================================
// PUBLIC SCALABILITY API
// ============================================================================

/// Public scalability-testing API.
pub use scalability::{
    ScaleAssessment,
    ScaleAxis,
    ScaleFailure,
    ScaleFailurePhase,
    ScaleObservation,
    ScalePoint,
    ScaleRunTermination,
    ScalabilityHarness,
    ScalabilityReport,
    ScalabilityRunOptions,
    SCALABILITY_TESTING_API_VERSION,
};

// ============================================================================
// COMPOSITION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_testing_components_are_independently_constructible() {
        let _provider_harness = ProviderHarness::new();
        let _conformance_suite = ConformanceSuite::new();
        let _portability_tester = PortabilityTester::new();
        let _scalability_harness = ScalabilityHarness::new();
        let _fault_injection_plan = FaultInjectionPlan::new();
    }

    #[test]
    fn empty_conformance_report_does_not_claim_success() {
        let report = ConformanceSuiteReport::new();

        assert_eq!(report.provider_count(), 0);
        assert!(!report.is_conformant());
        assert!(report.validate().is_ok());
    }

    #[test]
    fn empty_fault_injection_report_does_not_claim_success() {
        let report = FaultInjectionReport::new();

        assert!(report.is_empty());
        assert!(!report.is_successful());
    }

    #[test]
    fn empty_portability_report_does_not_claim_success() {
        let report = PortabilityReport::new();

        assert!(report.validate().is_ok());
        assert!(!report.is_portable());
    }

    #[test]
    fn testing_modules_have_distinct_responsibilities() {
        assert_eq!(SCALABILITY_TESTING_API_VERSION, 1);

        assert!(matches!(
            ProbeOutcome::NotRun,
            ProbeOutcome::NotRun
        ));

        assert!(matches!(
            InjectionDisposition::OutcomeUnknown,
            InjectionDisposition::OutcomeUnknown
        ));

        assert!(matches!(
            VerificationDisposition::Inconclusive,
            VerificationDisposition::Inconclusive
        ));
    }
}
