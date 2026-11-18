
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Build Command
//!
//! Path: src/toolchain/commands/build.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the public build-command orchestration boundary.
//!
//! This module accepts a public toolchain build request, validates it,
//! coordinates authoritative project/build preparation, invokes the
//! existing compiler-facing build executor, and returns structured
//! execution evidence.
//!
//! ## POCO-REAF
//!
//! One logical build request can produce multiple target-specific
//! realizations.
//!
//! The command preserves:
//!
//! - One validated logical build request.
//! - One immutable build plan.
//! - One logical build identity.
//! - One complete build-input snapshot.
//! - Authoritative target-feasibility evidence.
//! - Target-specific compiler configurations.
//! - Verified compiler artifacts.
//! - Structured execution outcomes.
//!
//! Different target realizations may produce different artifacts
//! while preserving applicable program semantics.
//!
//! ## Architectural ownership
//!
//! This module owns command orchestration only.
//!
//! It does not own:
//!
//! - Source grammar, parsing or semantic analysis.
//! - Workspace discovery implementation.
//! - Manifest parsing.
//! - Dependency resolution.
//! - Target discovery or registration.
//! - Capability matching.
//! - Feasibility analysis.
//! - Physical resource planning or allocation.
//! - Compiler implementation.
//! - Backend implementation.
//! - External process execution.
//! - Hardware discovery.
//! - Runtime scheduling or execution.
//! - Artifact storage or verification algorithms.
//! - Package creation.
//! - Deployment.
//!
//! Those responsibilities remain with their authoritative subsystems.
//!
//! ## Canonical integrations
//!
//! Public request:
//!     crate::toolchain::api::request
//!
//! Validated request:
//!     crate::toolchain::build::request
//!
//! Build plan and feasibility:
//!     crate::toolchain::build::plan
//!
//! Immutable build inputs:
//!     crate::toolchain::build::inputs
//!
//! Compiler execution:
//!     crate::toolchain::build::executor
//!
//! Operational budgets:
//!     crate::toolchain::configuration::budget
//!
//! Structured errors:
//!     crate::toolchain::errors
//!
//! Build preparation implementations must integrate:
//!
//!     crate::toolchain::workspace
//!     crate::toolchain::manifest
//!     crate::toolchain::dependencies
//!     crate::toolchain::build
//!     crate::compiler
//!     crate::backend
//!
//! ## Scalability
//!
//! This module imposes no arbitrary universal ceilings on:
//!
//! - Projects.
//! - Workspace members.
//! - Source files.
//! - Dependencies.
//! - Targets.
//! - Computing domains.
//! - Build artifacts.
//! - Tasks.
//! - Processors.
//! - Accelerators.
//! - Quantum resources.
//! - Memory.
//! - Storage.
//! - Distributed participants.
//!
//! Actual execution remains subject to available resources,
//! representational limits, provider capabilities and explicitly
//! configured operational budgets.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## Failure semantics
//!
//! A successful command result requires actual compiler execution
//! and successfully verified output artifacts.
//!
//! Preparation failure, cancellation, compiler failure, configuration
//! failure, budget exhaustion and artifact-verification failure must
//! never be converted into successful build results.
//!
//! A partially completed compilation that subsequently fails must
//! not be reported as a completed logical build.
//!
//! ## Concurrency
//!
//! This command does not create a worker pool or choose a fixed
//! concurrency level.
//!
//! The existing BuildExecutor provides resource-conservative
//! compiler invocation. Task-level parallel execution belongs to
//! the canonical task execution subsystem.
//!
//! ## Production integration
//!
//! No fake default workspace, manifest, dependency, target or
//! artifact provider is supplied.
//!
//! A real BuildPreparationProvider must connect the canonical
//! repository services before the command can be deployed.

use std::sync::Arc;

use crate::compiler::api::CompilerService;

use crate::toolchain::api::request::BuildRequest as ApiBuildRequest;

use crate::toolchain::build::executor::{
    ArtifactVerifier,
    BuildExecutionResult,
    BuildExecutor,
    BuildSourceProvider,
    CancellationProbe,
    TargetConfigurationProvider,
};

use crate::toolchain::build::inputs::BuildInputs;

use crate::toolchain::build::plan::AssessedBuildPlan;

use crate::toolchain::build::request::BuildRequest
    as ValidatedBuildRequest;

use crate::toolchain::configuration::budget::BudgetTracker;

use crate::toolchain::errors::ToolchainError;

use crate::toolchain::result::ToolchainResult;

// ============================================================================
// BUILD PREPARATION
// ============================================================================

/// Supplies the authoritative preparation services required by
/// a logical build operation.
///
/// Implementations must coordinate the existing repository
/// authorities rather than implementing competing subsystems.
///
/// The preparation lifecycle must establish:
///
/// 1. Project and workspace discovery.
/// 2. Canonical manifest loading.
/// 3. Manifest validation.
/// 4. Dependency resolution.
/// 5. Feature resolution.
/// 6. Effective build-profile resolution.
/// 7. Target-selection resolution.
/// 8. Compiler feasibility assessment.
/// 9. Complete immutable build-input construction.
/// 10. Target-specific source and compiler configuration providers.
///
/// The returned build plan must preserve the original validated
/// request.
///
/// The build-input snapshot must represent the complete inspected
/// input inventory. An incomplete inventory must never be presented
/// as a reproducible complete build.
///
/// This trait does not authorize a provider to invent target
/// feasibility or bypass compiler validation.
pub trait BuildPreparationProvider: Send + Sync {
    /// Prepare one validated build request.
    ///
    /// The returned preparation must be internally consistent and
    /// must originate from the canonical project, manifest,
    /// dependency, target and compiler-planning authorities.
    ///
    /// # Errors
    ///
    /// Returns the original structured toolchain error when project
    /// discovery, manifest validation, dependency resolution,
    /// target resolution, feasibility assessment or input
    /// construction fails.
    fn prepare(
        &self,
        request: &ValidatedBuildRequest,
    ) -> ToolchainResult<PreparedBuild>;
}

// ============================================================================
// PREPARED BUILD
// ============================================================================

/// Complete immutable preparation required for compiler execution.
///
/// All providers are injected explicitly.
///
/// This avoids hidden global registries, implicit host-target
/// selection and fabricated default implementations.
pub struct PreparedBuild {
    plan: AssessedBuildPlan,
    inputs: BuildInputs,

    sources: Arc<dyn BuildSourceProvider>,

    configurations: Arc<dyn TargetConfigurationProvider>,

    verifier: Arc<dyn ArtifactVerifier>,

    budget: BudgetTracker,

    cancellation: Arc<dyn CancellationProbe>,
}

impl PreparedBuild {
    /// Construct a prepared build.
    ///
    /// The plan must correspond exactly to the validated request.
    ///
    /// All providers must be authoritative implementations or
    /// explicitly controlled test implementations.
    ///
    /// # Errors
    ///
    /// Returns an error when the plan does not correspond to the
    /// original request or cannot establish a buildable target.
    pub fn new(
        request: &ValidatedBuildRequest,
        plan: AssessedBuildPlan,
        inputs: BuildInputs,
        sources: Arc<dyn BuildSourceProvider>,
        configurations: Arc<dyn TargetConfigurationProvider>,
        verifier: Arc<dyn ArtifactVerifier>,
        budget: BudgetTracker,
        cancellation: Arc<dyn CancellationProbe>,
    ) -> ToolchainResult<Self> {
        if plan.plan().request() != request {
            return Err(
                ToolchainError::validation_failed(
                    "Prepared build plan does not correspond to \
                     the validated build request",
                )
                .with_operation("commands.build.prepare"),
            );
        }

        if !plan.can_attempt_build() {
            return Err(
                ToolchainError::unsupported_operation(
                    "No requested target has established \
                     sufficient feasibility evidence for building",
                )
                .with_operation("commands.build.prepare"),
            );
        }

        Ok(Self {
            plan,
            inputs,
            sources,
            configurations,
            verifier,
            budget,
            cancellation,
        })
    }

    /// Borrow the immutable assessed build plan.
    #[must_use]
    pub fn plan(&self) -> &AssessedBuildPlan {
        &self.plan
    }

    /// Borrow the complete immutable build-input snapshot.
    #[must_use]
    pub fn inputs(&self) -> &BuildInputs {
        &self.inputs
    }
}

// ============================================================================
// BUILD COMMAND RESULT
// ============================================================================

/// Structured result of one completed logical build command.
///
/// A result can only be constructed by the command after the
/// compiler executor returns successfully and at least one
/// verified artifact exists.
///
/// The result preserves the assessed plan and complete input
/// snapshot for downstream reporting, incremental compilation,
/// reproducibility and artifact association.
#[must_use = "inspect and report the completed build result"]
pub struct BuildCommandResult {
    request: ValidatedBuildRequest,

    plan: AssessedBuildPlan,

    inputs: BuildInputs,

    execution: BuildExecutionResult,
}

impl BuildCommandResult {
    /// Return the validated original request.
    #[must_use]
    pub fn request(&self) -> &ValidatedBuildRequest {
        &self.request
    }

    /// Return the canonical logical build identity.
    #[must_use]
    pub fn build_id(
        &self,
    ) -> &crate::toolchain::ids::BuildId {
        self.plan.plan().id()
    }

    /// Borrow the assessed build plan.
    #[must_use]
    pub fn plan(&self) -> &AssessedBuildPlan {
        &self.plan
    }

    /// Borrow the immutable build-input snapshot.
    #[must_use]
    pub fn inputs(&self) -> &BuildInputs {
        &self.inputs
    }

    /// Borrow the actual compiler execution result.
    #[must_use]
    pub fn execution(&self) -> &BuildExecutionResult {
        &self.execution
    }

    /// Borrow all successfully verified compiler artifacts.
    #[must_use]
    pub fn artifacts(
        &self,
    ) -> &[crate::toolchain::build::executor::VerifiedBuildArtifact] {
        self.execution.artifacts()
    }

    /// Return the number of verified artifacts.
    #[must_use]
    pub fn artifact_count(&self) -> usize {
        self.execution.artifact_count()
    }

    /// Return whether the requested build contains skipped targets.
    #[must_use]
    pub fn has_skipped_targets(&self) -> bool {
        self.execution.has_skipped_targets()
    }

    /// Consume the result and return its principal components.
    ///
    /// The components remain independently available for the
    /// canonical output adapter and reporting subsystem.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        ValidatedBuildRequest,
        AssessedBuildPlan,
        BuildInputs,
        BuildExecutionResult,
    ) {
        (
            self.request,
            self.plan,
            self.inputs,
            self.execution,
        )
    }
}

// ============================================================================
// BUILD COMMAND SERVICE
// ============================================================================

/// Coordinates validated project builds.
///
/// The compiler implementation is injected through the existing
/// public CompilerService contract.
///
/// Preparation is injected through BuildPreparationProvider.
///
/// No global compiler singleton, target registry, worker pool
/// or hidden execution environment is created.
pub struct BuildCommand<P, C>
where
    P: BuildPreparationProvider,
    C: CompilerService,
{
    preparation: P,
    executor: BuildExecutor<C>,
}

impl<P, C> BuildCommand<P, C>
where
    P: BuildPreparationProvider,
    C: CompilerService,
{
    /// Construct a build command with explicit preparation and
    /// compiler services.
    ///
    /// The supplied compiler must be the intended compiler
    /// implementation for the deployment.
    #[must_use]
    pub fn new(
        preparation: P,
        compiler: C,
    ) -> Self {
        Self {
            preparation,
            executor: BuildExecutor::new(compiler),
        }
    }

    /// Execute a public build request.
    ///
    /// The request is structurally validated before any preparation
    /// or filesystem-dependent operation is attempted.
    ///
    /// # Errors
    ///
    /// Returns an error if request validation, preparation,
    /// feasibility enforcement, compilation, cancellation,
    /// operational-budget accounting or artifact verification fails.
    ///
    /// A successful result always contains at least one verified
    /// compiler artifact.
    pub fn execute(
        &self,
        request: ApiBuildRequest,
    ) -> ToolchainResult<BuildCommandResult> {
        let validated = ValidatedBuildRequest::try_from_api(request)
            .map_err(|error| {
                ToolchainError::validation_failed(format!(
                    "Build request validation failed: {error}"
                ))
                .with_operation("commands.build.request")
            })?;

        self.execute_validated(validated)
    }

    /// Execute an already constructed validated build request.
    ///
    /// Validation is repeated at this boundary to prevent callers
    /// from bypassing structural request validation.
    ///
    /// The preparation provider must return a plan corresponding
    /// exactly to this request.
    ///
    /// # Errors
    ///
    /// Returns the original structured error from preparation or
    /// compilation, with no fabricated successful result.
    pub fn execute_validated(
        &self,
        request: ValidatedBuildRequest,
    ) -> ToolchainResult<BuildCommandResult> {
        request.validate().map_err(|error| {
            ToolchainError::validation_failed(format!(
                "Validated build request failed revalidation: {error}"
            ))
            .with_operation("commands.build.request")
        })?;

        // --------------------------------------------------------
        // Stage 1: Authoritative build preparation
        // --------------------------------------------------------

        let prepared = self.preparation.prepare(&request)?;

        // Recheck the preparation boundary even when the provider
        // used PreparedBuild::new.
        if prepared.plan.plan().request() != &request {
            return Err(
                ToolchainError::validation_failed(
                    "Build preparation returned a plan for a \
                     different request",
                )
                .with_operation("commands.build.prepare"),
            );
        }

        if !prepared.plan.can_attempt_build() {
            return Err(
                ToolchainError::unsupported_operation(
                    "The assessed build plan does not authorize \
                     compiler execution",
                )
                .with_operation("commands.build.feasibility"),
            );
        }

        // --------------------------------------------------------
        // Stage 2: Preserve preparation evidence
        // --------------------------------------------------------

        let PreparedBuild {
            plan,
            inputs,
            sources,
            configurations,
            verifier,
            budget,
            cancellation,
        } = prepared;

        // --------------------------------------------------------
        // Stage 3: Actual compiler execution
        // --------------------------------------------------------

        let execution = self.executor.execute(
            &plan,
            sources.as_ref(),
            configurations.as_ref(),
            verifier.as_ref(),
            &budget,
            cancellation.as_ref(),
        )?;

        // --------------------------------------------------------
        // Stage 4: Successful-output enforcement
        // --------------------------------------------------------

        if !execution.has_artifacts() {
            return Err(
                ToolchainError::validation_failed(
                    "Compiler execution completed without producing \
                     any verified build artifacts",
                )
                .with_operation("commands.build.outputs"),
            );
        }

        // --------------------------------------------------------
        // Stage 5: Structured command result
        // --------------------------------------------------------

        Ok(BuildCommandResult {
            request,
            plan,
            inputs,
            execution,
        })
    }

    /// Borrow the preparation provider.
    #[must_use]
    pub fn preparation(&self) -> &P {
        &self.preparation
    }

    /// Borrow the configured compiler service.
    #[must_use]
    pub fn compiler(&self) -> &C {
        self.executor.compiler()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensure the command's preparation boundary remains explicit.
    ///
    /// A compiler service cannot be constructed from an absent
    /// preparation provider.
    #[test]
    fn preparation_is_an_explicit_dependency() {
        fn accepts_provider<P: BuildPreparationProvider>(_: &P) {}

        let _ = accepts_provider::<
            fn(&ValidatedBuildRequest) -> ToolchainResult<PreparedBuild>,
        >;
    }

    /// Ensure the structured result exposes the canonical build ID
    /// rather than inventing another command-level identity.
    #[test]
    fn result_uses_canonical_build_identity() {
        fn accepts_result(
            result: &BuildCommandResult,
        ) -> &crate::toolchain::ids::BuildId {
            result.build_id()
        }

        let _ = accepts_result;
    }

    /// Ensure the command accepts the canonical compiler-service
    /// abstraction rather than a vendor-specific implementation.
    #[test]
    fn compiler_service_is_generic() {
        fn accepts_command<P, C>(
            _: &BuildCommand<P, C>,
        )
        where
            P: BuildPreparationProvider,
            C: CompilerService,
        {
        }

        let _ = accepts_command::<
            fn(&ValidatedBuildRequest) -> ToolchainResult<PreparedBuild>,
            crate::compiler::api::CompilerApi,
        >;
    }
}
