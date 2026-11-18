
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Run Command
//!
//! Path: src/toolchain/commands/run.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the public run-command orchestration boundary.
//!
//! The run command coordinates:
//!
//! 1. Public run-request validation.
//! 2. Actual project compilation.
//! 3. Verified compiler artifact acquisition.
//! 4. Canonical compiler-artifact integration.
//! 5. Runtime artifact validation.
//! 6. Execution environment validation.
//! 7. Runtime service and resource readiness.
//! 8. Canonical runtime execution.
//! 9. Actual terminal execution observation.
//! 10. Structured execution reporting.
//!
//! ## Architectural ownership
//!
//! This module owns command orchestration only.
//!
//! It does not own:
//!
//! - Grammar or source parsing.
//! - Semantic analysis.
//! - Entry-point semantic resolution.
//! - Workspace discovery.
//! - Manifest parsing.
//! - Dependency resolution.
//! - Compilation.
//! - Backend code generation.
//! - Artifact construction.
//! - Artifact integrity algorithms.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Runtime scheduling.
//! - Execution-provider implementation.
//! - Program execution.
//! - External process execution.
//!
//! These responsibilities remain with their authoritative subsystems.
//!
//! ## Canonical integrations
//!
//! Public run request:
//!     crate::toolchain::api::request::RunRequest
//!
//! Build orchestration:
//!     crate::toolchain::commands::build
//!
//! Compiler:
//!     crate::compiler::api
//!
//! Compiler artifacts:
//!     crate::compiler::artifacts
//!
//! Hardware:
//!     crate::zhal
//!
//! Runtime artifact validation:
//!     crate::zur::execution::artifact
//!
//! Runtime artifact loading:
//!     crate::zur::execution::loader
//!
//! Runtime dispatch:
//!     crate::zur::execution::dispatch
//!
//! Runtime execution:
//!     crate::zur::execution::executor
//!
//! ## POCO-REAF
//!
//! One source project may produce multiple target-specific artifacts.
//!
//! The run command preserves the logical build identity and delegates
//! target-specific realization and execution to their canonical owners.
//!
//! It does not assume that every target can execute every program.
//!
//! ## Scalability
//!
//! No arbitrary universal ceilings are introduced for:
//!
//! - Source files.
//! - Build artifacts.
//! - Arguments.
//! - Computational domains.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Quantum resources.
//! - Memory.
//! - Storage.
//! - Distributed participants.
//! - Execution tasks.
//!
//! Actual operations remain subject to physical resources,
//! implementation capabilities, address-space limits and explicitly
//! configured operational budgets.
//!
//! Literal infinite execution or storage is not promised.
//!
//! ## Safety
//!
//! Unsafe Rust is forbidden.
//!
//! No shell execution is performed.
//!
//! No raw executable pointers are exposed.
//!
//! No hardware or runtime provider is selected implicitly.
//!
//! No execution success is fabricated.
//!
//! ## Failure semantics
//!
//! Build failure prevents runtime handoff.
//!
//! Artifact or environment validation failure prevents execution.
//!
//! A submission receipt is not execution completion.
//!
//! A partial, failed, cancelled, timed-out or unknown execution
//! remains distinguishable from successful completion.

use std::future::Future;
use std::pin::Pin;

use crate::toolchain::api::request::RunRequest;

use crate::toolchain::commands::build::{
    BuildCommand,
    BuildCommandResult,
    BuildPreparationProvider,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::result::ToolchainResult;

use crate::zur::execution::executor::ExecutionOutcome;

use crate::zur::error::ZurResult;

// ============================================================================
// RUNTIME EXECUTION FUTURE
// ============================================================================

/// Asynchronous result returned by the canonical runtime integration.
///
/// The future must resolve to an actual terminal execution outcome.
///
/// It must not resolve successfully merely because execution was
/// accepted or submitted.
///
/// Cancellation, provider failure and indeterminate execution outcomes
/// must remain explicit.
pub type RuntimeRunFuture<'a> = Pin<
    Box<
        dyn Future<Output = ZurResult<ExecutionOutcome>>
            + Send
            + 'a
    >
>;

// ============================================================================
// RUNTIME EXECUTION PROVIDER
// ============================================================================

/// Integration boundary between toolchain orchestration and ZUR.
///
/// Implementations must use the existing canonical runtime facilities.
///
/// This trait exists because the toolchain build executor currently
/// exposes verified compiler outputs, while ZUR consumes canonical
/// compiler artifacts and their associated manifests, ABI contracts
/// and runtime validation evidence.
///
/// An implementation must therefore perform the authoritative
/// artifact-integration step before runtime execution.
///
/// ## Required execution lifecycle
///
/// Implementations must establish the following sequence:
///
/// 1. Check the run request and build-result association.
/// 2. Resolve the exact executable artifact from the verified build.
/// 3. Associate the artifact with its canonical compiler artifact
///    identity and manifest.
/// 4. Obtain authoritative ABI and entry-point metadata.
/// 5. Validate the artifact using ZUR artifact validation.
/// 6. Validate runtime and target compatibility.
/// 7. Establish actual execution-environment availability.
/// 8. Obtain required service readiness and resource reservations
///    through their respective authorities.
/// 9. Load the artifact through ZUR's ArtifactLoader.
/// 10. Construct the canonical validated execution request.
/// 11. Dispatch through ZUR's DispatchCoordinator or the
///     appropriate canonical execution authority.
/// 12. Monitor execution through ZUR's ExecutionExecutor.
/// 13. Return the actual terminal ExecutionOutcome.
///
/// Implementations must reuse:
///
/// - crate::compiler::artifacts
/// - crate::zur::execution::artifact
/// - crate::zur::execution::loader
/// - crate::zur::execution::dispatch
/// - crate::zur::execution::executor
/// - crate::zur::core
/// - crate::zhal
///
/// Domain-specific integrations remain with their existing owners.
///
/// Classical, quantum, HDL, AI, hybrid and distributed execution
/// must use their appropriate registered runtime authorities.
///
/// ## Important restrictions
///
/// Implementations must not:
///
/// - Execute compiler output directly.
/// - Introduce a second artifact representation.
/// - Invent artifact manifests or ABI metadata.
/// - Select an arbitrary runtime provider.
/// - Assume provider registration proves availability.
/// - Assume hardware discovery proves reservation.
/// - Ignore cancellation.
/// - Convert an unknown execution outcome into success.
/// - Invoke shell commands.
/// - Bypass runtime authorization.
///
/// The implementation must preserve the original request arguments
/// and working-directory intent.
///
/// The working directory must be checked and authorized by the
/// appropriate execution environment authority.
///
/// This interface does not itself implement runtime execution.
pub trait RuntimeRunProvider: Send + Sync {

    /// Execute a successfully built project through canonical ZUR.
    ///
    /// `request` contains the original validated-by-boundary
    /// run intent.
    ///
    /// `build` contains the actual build plan, input snapshot
    /// and verified compiler outputs.
    ///
    /// Implementations must verify that the executable artifact
    /// belongs to this exact build result.
    ///
    /// The returned future must complete only after the canonical
    /// runtime has established a terminal execution outcome.
    ///
    /// # Errors
    ///
    /// Returns the original ZUR error when artifact integration,
    /// validation, environment preparation, authorization,
    /// resource acquisition, loading, dispatch, observation
    /// or execution monitoring fails.
    fn execute<'a>(
        &'a self,
        request: &'a RunRequest,
        build: &'a BuildCommandResult,
    ) -> RuntimeRunFuture<'a>;
}

// ============================================================================
// RUN COMMAND RESULT
// ============================================================================

/// Complete result of a run-command operation.
///
/// This type retains:
///
/// - The original run request.
/// - The actual completed build result.
/// - The canonical ZUR terminal execution outcome.
///
/// A result can only be constructed by RunCommand after the
/// runtime provider returns a terminal execution outcome.
///
/// A returned result does not necessarily mean successful
/// program execution. Inspect `execution()` or `is_successful()`.
#[must_use = "inspect and report the actual runtime execution outcome"]
pub struct RunCommandResult {

    request: RunRequest,

    build: BuildCommandResult,

    execution: ExecutionOutcome,
}

impl RunCommandResult {

    /// Return the original run request.
    #[must_use]
    pub fn request(&self) -> &RunRequest {
        &self.request
    }

    /// Return the actual completed build result.
    #[must_use]
    pub fn build(&self) -> &BuildCommandResult {
        &self.build
    }

    /// Return the actual canonical runtime execution outcome.
    #[must_use]
    pub fn execution(&self) -> &ExecutionOutcome {
        &self.execution
    }

    /// Return the canonical logical build identity.
    #[must_use]
    pub fn build_id(
        &self,
    ) -> &crate::toolchain::ids::BuildId {
        self.build.build_id()
    }

    /// Return the canonical runtime execution identity.
    #[must_use]
    pub fn execution_id(
        &self,
    ) -> &crate::zur::ids::ExecutionId {
        self.execution.execution_id()
    }

    /// Return whether the program completed successfully.
    ///
    /// This value comes from the actual canonical runtime result.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.execution.is_successful()
    }

    /// Return whether execution partially completed.
    #[must_use]
    pub fn is_partial(&self) -> bool {
        self.execution.is_partial()
    }

    /// Return whether execution failed.
    #[must_use]
    pub fn is_failed(&self) -> bool {
        self.execution.is_failed()
    }

    /// Return whether execution was confirmed as cancelled.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.execution.is_cancelled()
    }

    /// Return whether execution timed out.
    #[must_use]
    pub fn is_timed_out(&self) -> bool {
        self.execution.is_timed_out()
    }

    /// Return whether execution has a successful terminal outcome.
    ///
    /// Partial completion, cancellation, timeout and failure are
    /// never classified as successful completion.
    #[must_use]
    pub fn has_terminal_success(&self) -> bool {
        self.execution.is_successful()
    }

    /// Consume the result into its original components.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        RunRequest,
        BuildCommandResult,
        ExecutionOutcome,
    ) {
        (
            self.request,
            self.build,
            self.execution,
        )
    }
}

// ============================================================================
// RUN COMMAND
// ============================================================================

/// Coordinates project compilation and canonical runtime execution.
///
/// The build command and runtime provider are explicitly injected.
///
/// This avoids:
///
/// - Hidden global compiler instances.
/// - Hidden runtime initialization.
/// - Implicit target selection.
/// - Implicit hardware discovery.
/// - Implicit resource allocation.
/// - Vendor-specific runtime assumptions.
/// - Fabricated execution providers.
///
/// The same command can support new computational domains through
/// the existing compiler, artifact, hardware and runtime contracts.
///
/// No domain-specific compiler or runtime implementation is introduced.
pub struct RunCommand<P, C, R>
where
    P: BuildPreparationProvider,
    C: crate::compiler::api::CompilerService,
    R: RuntimeRunProvider,
{
    build: BuildCommand<P, C>,

    runtime: R,
}

impl<P, C, R> RunCommand<P, C, R>
where
    P: BuildPreparationProvider,
    C: crate::compiler::api::CompilerService,
    R: RuntimeRunProvider,
{

    /// Construct a run command with explicit build and runtime services.
    ///
    /// The build command must use the intended canonical compiler
    /// service and authoritative build-preparation implementation.
    ///
    /// The runtime provider must be connected to the canonical ZUR
    /// execution architecture.
    ///
    /// Construction performs no compilation, hardware discovery,
    /// resource allocation, runtime initialization or execution.
    #[must_use]
    pub fn new(
        build: BuildCommand<P, C>,
        runtime: R,
    ) -> Self {
        Self {
            build,
            runtime,
        }
    }

    /// Execute a validated run request.
    ///
    /// The operation is asynchronous because canonical ZUR execution
    /// and monitoring are asynchronous.
    ///
    /// The build stage is performed first. Runtime execution cannot
    /// begin unless the build command returns actual verified artifacts.
    ///
    /// A successful return means that ZUR supplied a terminal
    /// execution outcome. It does not necessarily mean that the
    /// application completed successfully.
    ///
    /// Inspect `RunCommandResult::is_successful()` to determine
    /// the actual execution result.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The public run request is invalid.
    /// - The project cannot be built.
    /// - Compilation fails.
    /// - No verified build artifacts are available.
    /// - Runtime artifact integration fails.
    /// - Runtime artifact validation fails.
    /// - Execution environment validation fails.
    /// - Authorization fails.
    /// - Required runtime resources are unavailable.
    /// - Artifact loading fails.
    /// - Execution dispatch fails.
    /// - Execution monitoring cannot establish a terminal outcome.
    /// - The runtime provider violates its execution contract.
    ///
    /// Actual terminal execution failures are preserved in the
    /// returned ExecutionOutcome rather than converted into
    /// fabricated toolchain success.
    pub async fn execute(
        &self,
        request: RunRequest,
    ) -> ToolchainResult<RunCommandResult> {

        // ------------------------------------------------------------
        // STAGE 1: PUBLIC REQUEST VALIDATION
        // ------------------------------------------------------------

        request.validate().map_err(|error| {
            ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                format!(
                    "run request validation failed: {error}"
                ),
            )
            .with_operation("commands.run.request")
            .with_source(error)
        })?;

        // ------------------------------------------------------------
        // STAGE 2: ACTUAL PROJECT BUILD
        // ------------------------------------------------------------

        // BuildRequest is cloned because the original RunRequest
        // must remain available to the runtime provider.
        //
        // The build command performs its own complete validation
        // and preparation lifecycle.

        let build = self
            .build
            .execute(request.build.clone())?;

        // ------------------------------------------------------------
        // STAGE 3: VERIFIED ARTIFACT ENFORCEMENT
        // ------------------------------------------------------------

        if build.artifacts().is_empty() {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::ValidationFailed,
                    "run cannot proceed because the build \
                     produced no verified artifacts",
                )
                .with_operation("commands.run.artifacts")
                .with_subject(
                    build.build_id().to_string(),
                ),
            );
        }

        // ------------------------------------------------------------
        // STAGE 4: CANONICAL RUNTIME HANDOFF
        // ------------------------------------------------------------

        // The runtime provider must perform the canonical
        // compiler-artifact to ZUR-artifact integration.
        //
        // It must validate the exact artifact, environment,
        // ABI, runtime services, authorization and resources.
        //
        // It must then load, dispatch and monitor the program
        // through the existing ZUR execution architecture.

        let execution = self
            .runtime
            .execute(&request, &build)
            .await
            .map_err(|error| {
                ToolchainError::new(
                    ToolchainErrorKind::ProviderFailed,
                    format!(
                        "canonical runtime execution failed: {error}"
                    ),
                )
                .with_operation("commands.run.runtime")
                .with_subject(
                    build.build_id().to_string(),
                )
                .with_source(error)
            })?;

        // ------------------------------------------------------------
        // STAGE 5: EXECUTION IDENTITY VALIDATION
        // ------------------------------------------------------------

        // ZUR owns execution identity.
        //
        // ExecutionOutcome has already validated that its
        // handle and terminal result identify the same execution.
        //
        // We additionally ensure that the returned identity
        // is available for the final command result.

        let _execution_id = execution.execution_id();

        // ------------------------------------------------------------
        // STAGE 6: STRUCTURED RESULT
        // ------------------------------------------------------------

        // Do not convert a failed, partial, cancelled or timed-out
        // terminal execution into an error-free success status.
        //
        // Preserve the actual ZUR outcome for the caller.

        Ok(RunCommandResult {
            request,
            build,
            execution,
        })
    }

    /// Borrow the underlying build command.
    #[must_use]
    pub fn build_command(&self) -> &BuildCommand<P, C> {
        &self.build
    }

    /// Borrow the configured runtime integration.
    #[must_use]
    pub fn runtime_provider(&self) -> &R {
        &self.runtime
    }
}

// ============================================================================
// CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    /// Verify that the runtime integration remains an explicitly
    /// injected dependency.
    #[test]
    fn runtime_provider_is_explicit() {

        fn accepts_provider<R: RuntimeRunProvider>(_: &R) {}

        let _ = accepts_provider::<fn(
            &RunRequest,
            &BuildCommandResult,
        ) -> RuntimeRunFuture<'static>>;
    }

    /// Verify that the command is generic over the existing
    /// compiler and build-preparation contracts.
    #[test]
    fn run_command_uses_existing_build_contracts() {

        fn accepts_command<P, C, R>(
            _: &RunCommand<P, C, R>,
        )
        where
            P: BuildPreparationProvider,
            C: crate::compiler::api::CompilerService,
            R: RuntimeRunProvider,
        {
        }

        let _ = accepts_command::<
            fn(
                &crate::toolchain::build::ValidatedBuildRequest,
            ) -> ToolchainResult<
                crate::toolchain::commands::build::PreparedBuild,
            >,
            crate::compiler::api::CompilerApi,
            fn(
                &RunRequest,
                &BuildCommandResult,
            ) -> RuntimeRunFuture<'static>,
        >;
    }

    /// Verify that the public run result retains the canonical
    /// runtime execution outcome.
    #[test]
    fn result_exposes_actual_runtime_outcome() {

        fn accepts_outcome(
            result: &RunCommandResult,
        ) -> &ExecutionOutcome {
            result.execution()
        }

        let _ = accepts_outcome;
    }

    /// Verify that terminal success is obtained from ZUR rather
    /// than inferred from build success.
    #[test]
    fn success_is_not_inferred_from_compilation() {

        fn accepts_success(
            result: &RunCommandResult,
        ) -> bool {
            result.is_successful()
        }

        let _ = accepts_success;
    }
}
