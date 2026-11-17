
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Build Subsystem
//!
//! Path: `src/toolchain/build/mod.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Mission
//!
//! This module is the composition root of the Zamani toolchain build
//! subsystem.
//!
//! It coordinates the public build interfaces exposed by the existing
//! build modules without duplicating their implementations.
//!
//! The build subsystem supports the orchestration of projects involving:
//!
//! - Classical computation.
//! - Quantum computation.
//! - Hardware description and hardware realization.
//! - AI and tensor computation.
//! - Hybrid computation.
//! - Distributed computation.
//! - Networking and cryptography.
//! - Embedded systems and accelerators.
//! - Future registered computational domains.
//!
//! Domain-specific compilation and realization remain owned by their
//! authoritative repository subsystems.
//!
//! ## POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! One logical build request may produce multiple compatible
//! target-specific realizations.
//!
//! The build subsystem preserves:
//!
//! - One logical build identity.
//! - One validated build request.
//! - One immutable build plan.
//! - Target-specific feasibility evidence.
//! - Logical build-input identity.
//! - Target-realization input identities.
//! - Deterministic build dependencies.
//! - Target-specific execution outcomes.
//! - Verified artifact identities.
//! - Structured build reports.
//!
//! Different realizations may produce different artifacts while
//! preserving the applicable program semantics.
//!
//! POCO-REAF does not mean that every program can execute on every
//! physical machine or that physical resources are infinite.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Build subsystem composition.
//! - Public build module declarations.
//! - Deliberate public API re-exports.
//! - The boundary between build subsystem modules.
//!
//! It does not own:
//!
//! - Source grammar, lexing or parsing.
//! - AST construction.
//! - Semantic analysis.
//! - Requirements, capabilities, preferences, constraints or hints.
//! - Project or workspace discovery.
//! - Manifest parsing.
//! - Dependency resolution.
//! - Target registration or discovery.
//! - Capability matching.
//! - Feasibility analysis.
//! - Physical resource planning or allocation.
//! - Compiler implementation.
//! - Backend selection or code generation.
//! - External process execution.
//! - Hardware discovery.
//! - Runtime scheduling or execution.
//! - Package creation or publication.
//! - Deployment execution.
//!
//! These responsibilities remain with their authoritative subsystems.
//!
//! ## Existing repository integrations
//!
//! Validated build request:
//! `request`
//!
//! Build planning and feasibility association:
//! `plan`
//!
//! Build dependency graph:
//! `graph`
//!
//! Compiler invocation and execution orchestration:
//! `executor`
//!
//! Build profile composition:
//! `profiles`
//!
//! Immutable build-input identity:
//! `inputs`
//!
//! Verified build-output collection:
//! `outputs`
//!
//! Incremental build coordination:
//! `incremental`
//!
//! Structured build reporting:
//! `reporting`
//!
//! Public toolchain API:
//! `crate::toolchain::api`
//!
//! Configuration:
//! `crate::toolchain::configuration`
//!
//! Workspace:
//! `crate::toolchain::workspace`
//!
//! Manifest:
//! `crate::toolchain::manifest`
//!
//! Compiler:
//! `crate::compiler`
//!
//! Backend:
//! `crate::backend`
//!
//! Hardware abstraction:
//! `crate::zhal`
//!
//! Runtime:
//! `crate::zur`
//!
//! ## Build lifecycle
//!
//! The intended orchestration lifecycle is:
//!
//! ```text
//! Public BuildRequest
//!        |
//!        v
//! ValidatedBuildRequest
//!        |
//!        v
//! Workspace and manifest validation
//!        |
//!        v
//! Dependency resolution
//!        |
//!        v
//! Build profile resolution
//!        |
//!        v
//! Target resolution
//!        |
//!        v
//! BuildPlan
//!        |
//!        v
//! Compiler feasibility assessment
//!        |
//!        v
//! AssessedBuildPlan
//!        |
//!        v
//! Immutable BuildInputs
//!        |
//!        v
//! BuildGraph
//!        |
//!        v
//! Build execution
//!        |
//!        v
//! CompilerService
//!        |
//!        v
//! Backend artifact production
//!        |
//!        v
//! Artifact verification
//!        |
//!        v
//! BuildOutputs
//!        |
//!        v
//! Incremental state coordination
//!        |
//!        v
//! Structured BuildReport
//! ```
//!
//! This is the intended integration contract. Declaring these modules
//! does not itself implement a complete orchestration service.
//!
//! A dedicated build service must coordinate the lifecycle using the
//! existing module APIs.
//!
//! ## Scalability
//!
//! This composition module introduces no arbitrary universal ceilings
//! on:
//!
//! - Project size.
//! - Workspace members.
//! - Source files.
//! - Dependencies.
//! - Build tasks.
//! - Build graph nodes or edges.
//! - Target realizations.
//! - Computing domains.
//! - Processors or processing cores.
//! - Accelerators.
//! - Quantum resources.
//! - Memory or storage.
//! - Tensor dimensions.
//! - Distributed participants.
//! - Build artifacts.
//!
//! Actual operations remain subject to:
//!
//! - Available physical resources.
//! - Rust representation and address-space limits.
//! - Operating-system constraints.
//! - Provider capabilities.
//! - Semantic compatibility.
//! - Explicit operational budgets.
//! - Security and authorization policies.
//!
//! Resource exhaustion must be reported honestly.
//!
//! Literal infinite computation, memory or storage is not promised.
//!
//! ## Determinism
//!
//! This module introduces:
//!
//! - No global mutable state.
//! - No hidden target selection.
//! - No random identifiers.
//! - No implicit hardware assumptions.
//! - No additional scheduling policy.
//! - No independent fingerprint algorithm.
//!
//! Deterministic ordering and identity remain owned by the respective
//! build modules.
//!
//! ## Security
//!
//! Unsafe Rust is prohibited.
//!
//! Build requests must pass their owning validation boundary.
//!
//! Unresolved or unsupported capabilities must never be interpreted
//! as available.
//!
//! Failed compilation or artifact verification must never be reported
//! as a successful build.
//!
//! ## Versioning
//!
//! This module is an in-process composition boundary.
//!
//! It does not introduce a new serialization format.
//!
//! Persisted build inputs, incremental state, reports and artifacts
//! retain the versioning contracts of their respective modules.
//!
//! ## Public API compatibility
//!
//! The public API request is:
//!
//! `crate::toolchain::api::BuildRequest`
//!
//! The internal validated request is:
//!
//! `crate::toolchain::build::ValidatedBuildRequest`
//!
//! These types deliberately remain distinct.
//!
//! External caller input must not bypass the validated build boundary.


// ============================================================================
// MODULE DECLARATIONS
// ============================================================================

/// Validated internal build-request representation.
///
/// Converts the public API request into an immutable validated request.
pub mod request;

/// Immutable build plans and feasibility association.
///
/// Owns target resolution, build-plan validation and the association
/// between plans and authoritative compiler feasibility evidence.
pub mod plan;

/// Deterministic build-task dependency graph.
///
/// Owns build-task dependency relationships, cycle validation and
/// dependency-first ordering. It does not execute tasks.
pub mod graph;

/// Compiler-facing build execution orchestration.
///
/// Coordinates source acquisition, target-specific compiler
/// configuration, cancellation, operational budgets, compiler
/// invocation and artifact verification.
pub mod executor;

/// Build-profile resolution and configuration composition.
///
/// Resolves the effective configuration using existing toolchain
/// configuration, manifest settings, selected profiles and explicit
/// operation overrides.
pub mod profiles;

/// Immutable build-input snapshots and fingerprints.
///
/// Separates target-independent logical inputs from target-specific
/// realization inputs.
pub mod inputs;

/// Build-output collection and validation.
///
/// Associates verified compiler outputs with their target realizations
/// and canonical artifact representations.
pub mod outputs;

/// Incremental-build coordination.
///
/// Coordinates input-change detection and compiler-owned invalidation.
pub mod incremental;

/// Structured build and failure reporting.
///
/// Produces machine-readable reports for logical builds and
/// target-specific realizations.

pub mod reporting;


// ============================================================================
// VALIDATED REQUEST API
// ============================================================================

/// Internal validated build request.
///
/// This is intentionally different from the public API's
/// `crate::toolchain::api::BuildRequest`.
///
/// Use this type after the caller's request has passed the validation
/// boundary.
pub use request::BuildRequest as ValidatedBuildRequest;


// ============================================================================
// BUILD PLANNING API
// ============================================================================

/// A resolved canonical target associated with a build request.
pub use plan::ResolvedBuildTarget;

/// Result of resolving requested targets.
pub use plan::BuildTargetResolution;

/// Immutable validated logical build plan.
pub use plan::BuildPlan;

/// Build plan associated with feasibility evidence.
pub use plan::AssessedBuildPlan;


// ============================================================================
// BUILD GRAPH API
// ============================================================================

/// Deterministic build-task dependency graph.
pub use graph::BuildGraph;


// ============================================================================
// BUILD EXECUTION API
// ============================================================================

/// Source unit supplied to the compiler.
pub use executor::BuildSourceUnit;

/// Provider responsible for acquiring build source units.
pub use executor::BuildSourceProvider;

/// Target-specific compiler configuration.
pub use executor::TargetCompilerConfiguration;

/// Provider responsible for resolving target compiler configurations.
pub use executor::TargetConfigurationProvider;

/// Contract for authoritative artifact verification.
pub use executor::ArtifactVerifier;

/// Build cancellation observation contract.
pub use executor::CancellationProbe;

/// Cancellation probe that never reports cancellation.
pub use executor::NeverCancelled;

/// Artifact whose verification has succeeded.
pub use executor::VerifiedBuildArtifact;

/// Result of actual build execution.
pub use executor::BuildExecutionResult;

/// Generic compiler-service-backed build executor.
pub use executor::BuildExecutor;

/// Default compiler API-backed executor.
pub use executor::DefaultBuildExecutor;


// ============================================================================
// BUILD PROFILE API
// ============================================================================

/// Origin of a build-profile configuration section.
pub use profiles::BuildProfileSource;

/// Manifest-provided build settings.
pub use profiles::ManifestBuildSettings;

/// Explicit build-operation configuration overrides.
pub use profiles::BuildProfileOverrides;

/// Inputs used for resolving an effective build profile.
pub use profiles::BuildProfileInputs;

/// Origin of the selected build profile.
pub use profiles::ProfileSelectionSource;

/// Selected build profile.
pub use profiles::SelectedBuildProfile;

/// Fully resolved build profile.
pub use profiles::ResolvedBuildProfile;


// ============================================================================
// BUILD INPUT API
// ============================================================================

/// Classification of a build-input scope.
pub use inputs::BuildInputScope;

/// Authority associated with a build input.
pub use inputs::BuildInputAuthority;

/// Fingerprinted build-input record.
pub use inputs::BuildInputRecord;

/// Target-specific realization fingerprint.
pub use inputs::RealizationFingerprint;

/// Immutable build-input snapshot.
pub use inputs::BuildInputs;

/// Builder for immutable build-input snapshots.
pub use inputs::BuildInputsBuilder;

/// Build-input API version.
pub use inputs::BUILD_INPUTS_API_VERSION;

/// Build-input schema version.
pub use inputs::BUILD_INPUTS_SCHEMA_VERSION;


// ============================================================================
// BUILD OUTPUT API
// ============================================================================

/// Adapter to the canonical compiler artifact representation.
pub use outputs::CanonicalArtifactAdapter;

/// Classification of build-output completeness.
pub use outputs::BuildOutputStatus;

/// Target-to-artifact coverage information.
pub use outputs::TargetOutputCoverage;

/// Aggregate build-output statistics.
pub use outputs::BuildOutputSummary;

/// Validated outputs of a logical build.
pub use outputs::BuildOutputs;


// ============================================================================
// INCREMENTAL BUILD API
// ============================================================================

/// Identity of an incremental build artifact.
pub use incremental::IncrementalArtifactId;

/// Scope of an incremental artifact.
pub use incremental::BuildArtifactScope;

/// Classification of incremental input changes.
pub use incremental::IncrementalChangeKind;

/// Target-realization input change.
pub use incremental::RealizationChange;

/// Structured incremental-build report.
pub use incremental::IncrementalBuildReport;

/// Incremental-build coordination service.
pub use incremental::IncrementalBuildCoordinator;

/// Incremental-build API version.
pub use incremental::INCREMENTAL_BUILD_API_VERSION;

/// Incremental-build schema version.
pub use incremental::INCREMENTAL_BUILD_SCHEMA_VERSION;


// ============================================================================
// BUILD REPORTING API
// ============================================================================

/// Completion classification for a build report.
pub use reporting::BuildReportStatus;

/// Classification of a build failure.
pub use reporting::BuildFailureStatus;

/// Logical build-input report.
pub use reporting::BuildInputReport;

/// Target-realization input report.
pub use reporting::RealizationInputReport;

/// Target buildability classification.
pub use reporting::TargetBuildability;

/// Target-specific build report.
pub use reporting::TargetBuildReport;

/// Report for an unresolved target selector.
pub use reporting::UnresolvedTargetReport;

/// Structured build-plan report.
pub use reporting::BuildPlanReport;

/// Structured build-output report.
pub use reporting::BuildOutputReport;

/// Structured logical build report.
pub use reporting::BuildReport;

/// Structured build-failure report.
pub use reporting::BuildFailureReport;

/// Build-report schema identifier.
pub use reporting::BUILD_REPORT_SCHEMA;

/// Build-failure report schema identifier.
pub use reporting::BUILD_FAILURE_REPORT_SCHEMA;


// ============================================================================
// COMPOSITION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the build graph is available through the
    /// canonical build composition boundary.
    #[test]
    fn build_graph_is_available() {
        let graph = BuildGraph::new();

        assert!(graph.is_empty());
        assert_eq!(graph.task_count(), 0);
        assert_eq!(graph.dependency_count(), 0);
    }

    /// Verifies that the validated request has a distinct public
    /// composition name.
    #[test]
    fn validated_request_alias_is_available() {
        fn accepts_validated_request(
            _: Option<ValidatedBuildRequest>,
        ) {
        }

        accepts_validated_request(None);
    }

    /// Verifies that the build-plan composition exports are available.
    #[test]
    fn build_plan_exports_are_available() {
        fn accepts_plan(
            _: Option<BuildPlan>,
            _: Option<AssessedBuildPlan>,
            _: Option<BuildTargetResolution>,
        ) {
        }

        accepts_plan(None, None, None);
    }

    /// Verifies that the profile-resolution composition exports
    /// are available.
    #[test]
    fn build_profile_exports_are_available() {
        fn accepts_profile(
            _: Option<ManifestBuildSettings>,
            _: Option<BuildProfileOverrides>,
            _: Option<ResolvedBuildProfile>,
        ) {
        }

        accepts_profile(None, None, None);
    }

    /// Verifies that the immutable input snapshot exports are available.
    #[test]
    fn build_input_exports_are_available() {
        fn accepts_inputs(
            _: Option<BuildInputs>,
            _: Option<BuildInputsBuilder>,
            _: Option<BuildInputRecord>,
        ) {
        }

        accepts_inputs(None, None, None);
    }

    /// Verifies that the output composition exports are available.
    #[test]
    fn build_output_exports_are_available() {
        fn accepts_outputs(
            _: Option<BuildOutputs>,
            _: Option<BuildOutputSummary>,
        ) {
        }

        accepts_outputs(None, None);
    }

    /// Verifies that incremental-build coordination exports are available.
    #[test]
    fn incremental_build_exports_are_available() {
        fn accepts_incremental(
            _: Option<IncrementalBuildCoordinator>,
            _: Option<IncrementalBuildReport>,
        ) {
        }

        accepts_incremental(None, None);
    }
}
