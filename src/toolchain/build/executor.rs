
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Build Executor
//!
//! Path: src/toolchain/build/executor.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Executes an already validated and compiler-assessed logical build plan.
//!
//! The executor coordinates:
//!
//! - Build-plan validation.
//! - Target-feasibility enforcement.
//! - Project source acquisition.
//! - Target-specific compiler configuration.
//! - Shared operational budget accounting.
//! - Cancellation checks.
//! - Public compiler-service invocation.
//! - Compilation-output validation.
//! - Authoritative artifact verification.
//! - Structured build results.
//!
//! ## Architectural ownership
//!
//! This module owns build-execution orchestration only.
//!
//! It does not own:
//!
//! - Source-language semantics.
//! - Lexing or parsing.
//! - AST construction.
//! - Dependency resolution.
//! - Target discovery.
//! - Target registration.
//! - Feasibility analysis.
//! - Physical resource allocation.
//! - Compiler implementation.
//! - Backend implementation.
//! - Artifact storage.
//! - Package creation.
//! - Runtime scheduling.
//! - Physical program execution.
//! - Distributed infrastructure.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ## Canonical integration
//!
//! Build plan:
//!     crate::toolchain::build::plan
//!
//! Build graph:
//!     crate::toolchain::build::graph
//!
//! Build identity:
//!     crate::toolchain::ids
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! Operational budget:
//!     crate::toolchain::configuration::budget
//!
//! Compiler API:
//!     crate::compiler::api
//!
//! Compiler target identity:
//!     crate::compiler::configuration::target
//!
//! Compiler artifacts:
//!     crate::compiler::artifacts
//!
//! Backend:
//!     crate::backend
//!
//! ## POCO-REAF
//!
//! One logical build may produce multiple target-specific artifacts.
//!
//! Each artifact retains its originating logical build identity,
//! canonical target identity and source identity.
//!
//! The executor does not silently replace requested targets.
//!
//! It does not treat an unassessed target as buildable.
//!
//! It does not equate compiler success with runtime compatibility.
//!
//! It does not claim that every computing domain supports every
//! compiler operation.
//!
//! ## Scalability
//!
//! No application-defined maximum is introduced for:
//!
//! - Projects.
//! - Source units.
//! - Targets.
//! - Computing domains.
//! - Build artifacts.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Quantum resources.
//! - Distributed participants.
//! - Memory.
//! - Storage.
//!
//! The executor processes targets and compilation units sequentially.
//!
//! This provides a resource-conservative execution strategy without
//! introducing an arbitrary worker-count ceiling.
//!
//! Parallel execution belongs to the task execution subsystem and
//! must use the same shared BudgetTracker.
//!
//! Memory consumption is proportional to the source and artifact
//! data retained by the caller and the compiler.
//!
//! Rust allocation, address-space and operating-system limitations
//! remain real operational constraints.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## Important integration limitation
//!
//! The current compiler API uses an explicitly transitional legacy
//! compilation driver.
//!
//! This executor invokes the public compiler service, but cannot
//! certify the internal production readiness of that driver.
//!
//! A production deployment must provide a verified compiler pipeline
//! and an authoritative artifact-verification adapter.
//!
//! No successful build is fabricated when a required integration
//! fails.

use std::error::Error;
use std::fmt;

use crate::compiler::api::{
    CompilationResponse,
    CompilerConfig,
    CompilerService,
};

use crate::compiler::configuration::target::TargetId;

use crate::toolchain::build::plan::{
    AssessedBuildPlan,
    ResolvedBuildTarget,
};

use crate::toolchain::configuration::budget::{
    BudgetDimension,
    BudgetError,
    BudgetTracker,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::ids::BuildId;
use crate::toolchain::result::ToolchainResult;

// ============================================================================
// OPERATIONAL ACCOUNTING
// ============================================================================

fn compiler_invocation_dimension(
) -> ToolchainResult<BudgetDimension> {
    BudgetDimension::new(
        "toolchain.build.compiler_invocations",
        "operations",
    )
    .map_err(|error| {
        ToolchainError::invalid_configuration(
            format!(
                "invalid compiler-invocation budget dimension: {error}"
            ),
        )
        .with_operation("build.executor")
        .with_source(error)
    })
}

fn map_budget_error(error: BudgetError) -> ToolchainError {
    let kind = match &error {
        BudgetError::Exceeded { .. }
        | BudgetError::AccountingOverflow { .. } => {
            ToolchainErrorKind::ResourceExhausted
        }

        BudgetError::InvalidDimension { .. } => {
            ToolchainErrorKind::InvalidConfiguration
        }

        BudgetError::LedgerPoisoned => {
            ToolchainErrorKind::Internal
        }
    };

    ToolchainError::new(
        kind,
        format!("build operational budget failure: {error}"),
    )
    .with_operation("build.executor")
    .with_source(error)
}

// ============================================================================
// SOURCE INPUT
// ============================================================================

/// One logical source compilation unit.
///
/// Source identity is represented by its logical source name.
///
/// Source text remains owned by the source-provider boundary.
///
/// This type does not parse, normalize, rewrite or interpret source text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildSourceUnit {
    source_name: String,
    source: String,
}

impl BuildSourceUnit {
    /// Construct one source compilation unit.
    ///
    /// The source name must not be empty.
    ///
    /// Empty source text is permitted. Its validity is determined
    /// by the canonical compiler.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput if the source name is empty.
    pub fn new(
        source_name: impl Into<String>,
        source: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let source_name = source_name.into();

        if source_name.trim().is_empty() {
            return Err(
                ToolchainError::invalid_input(
                    "source compilation unit requires a non-empty logical name",
                )
                .with_operation("build.executor.source"),
            );
        }

        Ok(Self {
            source_name,
            source: source.into(),
        })
    }

    /// Return the logical source name.
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    /// Borrow the source text.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }
}

// ============================================================================
// SOURCE PROVIDER
// ============================================================================

/// Supplies the validated source compilation units for a target.
///
/// Implementations integrate the canonical project, workspace,
/// manifest and source-discovery infrastructure.
///
/// The executor does not assume that a project contains one source
/// file or follows one mandatory directory layout.
///
/// Returned compilation units must represent valid inputs for the
/// selected compiler-service operation.
///
/// Implementations must not silently omit required project sources.
pub trait BuildSourceProvider: Send + Sync {
    /// Load the source compilation units for one target realization.
    ///
    /// # Errors
    ///
    /// Returns the original structured toolchain error if source
    /// discovery or acquisition fails.
    fn load_sources(
        &self,
        plan: &AssessedBuildPlan,
        target: &ResolvedBuildTarget,
    ) -> ToolchainResult<Vec<BuildSourceUnit>>;
}

// ============================================================================
// TARGET COMPILER CONFIGURATION
// ============================================================================

/// Explicit compiler configuration for one target realization.
///
/// The compiler configuration belongs to the existing compiler.
///
/// The expected output target is supplied by the canonical target
/// and backend integration.
///
/// It is not inferred from the host machine.
#[derive(Debug, Clone)]
pub struct TargetCompilerConfiguration {
    /// Existing canonical compiler configuration.
    pub compiler: CompilerConfig,

    /// Expected canonical compiler output target label.
    ///
    /// The backend/target adapter must provide this value.
    ///
    /// It must match the actual compiler output.
    pub expected_output_target: String,
}

impl TargetCompilerConfiguration {
    /// Construct an explicit target compiler configuration.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput when the expected output target is empty.
    pub fn new(
        compiler: CompilerConfig,
        expected_output_target: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let expected_output_target = expected_output_target.into();

        if expected_output_target.trim().is_empty() {
            return Err(
                ToolchainError::invalid_input(
                    "expected compiler output target must not be empty",
                )
                .with_operation("build.executor.configuration"),
            );
        }

        Ok(Self {
            compiler,
            expected_output_target,
        })
    }
}

/// Resolves the compiler configuration for a canonical target.
///
/// Implementations adapt the assessed build request, selected target,
/// profile, features, semantic requirements and applicable compiler
/// policies into the existing CompilerConfig.
///
/// This trait does not own target selection or feasibility.
pub trait TargetConfigurationProvider: Send + Sync {
    /// Resolve explicit compiler configuration for a selected target.
    ///
    /// # Errors
    ///
    /// Returns an error when the target cannot be configured.
    fn configuration_for(
        &self,
        plan: &AssessedBuildPlan,
        target: &ResolvedBuildTarget,
    ) -> ToolchainResult<TargetCompilerConfiguration>;
}

// ============================================================================
// ARTIFACT VERIFICATION
// ============================================================================

/// Authoritative artifact-verification integration.
///
/// Implementations must delegate to the canonical compiler/backend
/// artifact verification infrastructure.
///
/// Verification must establish the applicable properties, including:
///
/// - Artifact structure.
/// - Artifact target compatibility.
/// - Required artifact metadata.
/// - Required integrity checks.
/// - Required provenance checks.
/// - Applicable format and ABI compatibility.
///
/// A successful return means the verifier completed its documented
/// verification contract.
///
/// This interface does not implement cryptographic verification,
/// backend validation or artifact storage.
pub trait ArtifactVerifier: Send + Sync {
    /// Verify one actual compiler output.
    ///
    /// The output must not be modified.
    ///
    /// # Errors
    ///
    /// Returns an error if verification fails or cannot be completed.
    fn verify(
        &self,
        build_id: &BuildId,
        target_id: &TargetId,
        source_name: &str,
        output: &crate::compiler::api::CompilationOutput,
    ) -> ToolchainResult<()>;
}

// ============================================================================
// CANCELLATION
// ============================================================================

/// Read-only cancellation observation.
///
/// The executor checks cancellation between compilation operations.
///
/// An active synchronous compiler invocation cannot be interrupted
/// by this interface. Interruptible compiler execution requires
/// cancellation support from the compiler service itself.
pub trait CancellationProbe: Send + Sync {
    /// Return whether the logical build has been cancelled.
    fn is_cancelled(&self) -> bool;
}

/// Cancellation implementation that never requests cancellation.
///
/// This is useful for explicitly uncancellable operations.
#[derive(Debug, Default, Clone, Copy)]
pub struct NeverCancelled;

impl CancellationProbe for NeverCancelled {
    fn is_cancelled(&self) -> bool {
        false
    }
}

// ============================================================================
// VERIFIED BUILD ARTIFACT
// ============================================================================

/// One compiler output that passed the configured artifact verifier.
///
/// Construction is restricted to the executor's successful
/// verification path.
///
/// The artifact is not a runtime execution result.
#[derive(Debug)]
pub struct VerifiedBuildArtifact {
    build_id: BuildId,
    target_id: TargetId,
    source_name: String,
    output: crate::compiler::api::CompilationOutput,
}

impl VerifiedBuildArtifact {
    /// Return the logical build identity.
    #[must_use]
    pub fn build_id(&self) -> &BuildId {
        &self.build_id
    }

    /// Return the canonical target identity.
    #[must_use]
    pub fn target_id(&self) -> &TargetId {
        &self.target_id
    }

    /// Return the logical source name.
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    /// Borrow the verified compiler output.
    #[must_use]
    pub fn output(
        &self,
    ) -> &crate::compiler::api::CompilationOutput {
        &self.output
    }

    /// Consume the verified artifact and return the compiler output.
    #[must_use]
    pub fn into_output(
        self,
    ) -> crate::compiler::api::CompilationOutput {
        self.output
    }
}

// ============================================================================
// BUILD RESULT
// ============================================================================

/// Result of one logical build execution.
///
/// All artifacts in this result share one BuildId.
///
/// Different target realizations retain their own canonical target
/// identities.
///
/// Skipped targets represent targets that were assessed but not
/// proven feasible when the original request permitted partial
/// target realization.
#[derive(Debug)]
pub struct BuildExecutionResult {
    build_id: BuildId,
    artifacts: Vec<VerifiedBuildArtifact>,
    skipped_targets: Vec<TargetId>,
}

impl BuildExecutionResult {
    /// Return the logical build identity.
    #[must_use]
    pub fn build_id(&self) -> &BuildId {
        &self.build_id
    }

    /// Borrow all verified artifacts.
    #[must_use]
    pub fn artifacts(&self) -> &[VerifiedBuildArtifact] {
        &self.artifacts
    }

    /// Borrow targets that were not attempted because feasibility
    /// was not established.
    #[must_use]
    pub fn skipped_targets(&self) -> &[TargetId] {
        &self.skipped_targets
    }

    /// Return the number of verified artifacts.
    #[must_use]
    pub fn artifact_count(&self) -> usize {
        self.artifacts.len()
    }

    /// Return whether the build produced at least one verified artifact.
    #[must_use]
    pub fn has_artifacts(&self) -> bool {
        !self.artifacts.is_empty()
    }

    /// Return whether some assessed targets were skipped.
    #[must_use]
    pub fn has_skipped_targets(&self) -> bool {
        !self.skipped_targets.is_empty()
    }

    /// Consume the result and return all verified artifacts.
    #[must_use]
    pub fn into_artifacts(self) -> Vec<VerifiedBuildArtifact> {
        self.artifacts
    }
}

// ============================================================================
// BUILD EXECUTOR
// ============================================================================

/// Coordinates execution of assessed build plans.
///
/// The compiler service is injected so that the executor can use
/// the existing public compiler contract without owning or duplicating
/// compilation.
///
/// The default service can be the existing CompilerApi.
///
/// Tests and future compiler implementations may inject another
/// implementation of the same public CompilerService trait.
pub struct BuildExecutor<C: CompilerService> {
    compiler: C,
}

impl<C: CompilerService> BuildExecutor<C> {
    /// Construct an executor using an explicit compiler service.
    #[must_use]
    pub const fn new(compiler: C) -> Self {
        Self {
            compiler,
        }
    }

    /// Borrow the configured compiler service.
    #[must_use]
    pub fn compiler(&self) -> &C {
        &self.compiler
    }

    /// Execute one assessed logical build.
    ///
    /// The same BudgetTracker must be shared with other tasks and
    /// compiler stages contributing to this logical operation.
    ///
    /// Execution order is deterministic:
    ///
    /// 1. Validate build eligibility.
    /// 2. Record non-buildable targets.
    /// 3. Process feasible targets in canonical target-ID order.
    /// 4. Load each target's source compilation units.
    /// 5. Obtain explicit target compiler configuration.
    /// 6. Account for each compiler invocation.
    /// 7. Check cancellation.
    /// 8. Invoke the public compiler service.
    /// 9. Validate the returned compiler output.
    /// 10. Invoke authoritative artifact verification.
    /// 11. Retain only successfully verified artifacts.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The assessed plan cannot attempt a build.
    /// - A required source cannot be obtained.
    /// - A target configuration cannot be resolved.
    /// - A budget operation fails.
    /// - Cancellation is requested.
    /// - Compilation fails.
    /// - Compiler output is inconsistent.
    /// - Artifact verification fails.
    /// - Required memory reservation fails.
    ///
    /// A failed build never returns a successful partial result.
    /// Successfully verified artifacts produced before a later
    /// failure are not falsely represented as a complete build.
    pub fn execute<S, T, V, X>(
        &self,
        plan: &AssessedBuildPlan,
        sources: &S,
        configurations: &T,
        verifier: &V,
        budget: &BudgetTracker,
        cancellation: &X,
    ) -> ToolchainResult<BuildExecutionResult>
    where
        S: BuildSourceProvider + ?Sized,
        T: TargetConfigurationProvider + ?Sized,
        V: ArtifactVerifier + ?Sized,
        X: CancellationProbe + ?Sized,
    {
        // ------------------------------------------------------------
        // BUILD ELIGIBILITY
        // ------------------------------------------------------------

        if !plan.can_attempt_build() {
            return Err(
                ToolchainError::validation_failed(
                    "assessed build plan does not contain sufficient \
                     feasibility evidence to attempt compilation",
                )
                .with_operation("build.execute")
                .with_subject(plan.plan().id().to_string()),
            );
        }

        let build_id = plan.plan().id().clone();

        // ------------------------------------------------------------
        // SHARED OPERATIONAL BUDGET
        // ------------------------------------------------------------

        let invocation_dimension = compiler_invocation_dimension()?;

        // ------------------------------------------------------------
        // RESULT STORAGE
        // ------------------------------------------------------------

        let mut artifacts = Vec::<VerifiedBuildArtifact>::new();
        let mut skipped_targets = Vec::<TargetId>::new();

        let non_buildable = plan.non_buildable_targets();

        for target in non_buildable {
            skipped_targets
                .try_reserve(1)
                .map_err(|error| {
                    allocation_error(
                        "skipped target results",
                        error,
                    )
                })?;

            skipped_targets.push(target.target_id().clone());
        }

        // ------------------------------------------------------------
        // TARGET EXECUTION
        // ------------------------------------------------------------

        for target in plan.buildable_targets() {
            self.check_cancellation(cancellation, &build_id)?;

            let target_id = target.target_id();

            // --------------------------------------------------------
            // SOURCE DISCOVERY
            // --------------------------------------------------------

            let source_units = sources
                .load_sources(plan, target)
                .map_err(|error| {
                    error
                        .with_operation("build.execute.sources")
                        .with_subject(target_id.to_string())
                })?;

            if source_units.is_empty() {
                return Err(
                    ToolchainError::validation_failed(
                        "source provider returned no compilation units \
                         for a buildable target",
                    )
                    .with_operation("build.execute.sources")
                    .with_subject(target_id.to_string()),
                );
            }

            // --------------------------------------------------------
            // EXPLICIT TARGET CONFIGURATION
            // --------------------------------------------------------

            let target_configuration = configurations
                .configuration_for(plan, target)
                .map_err(|error| {
                    error
                        .with_operation(
                            "build.execute.target_configuration",
                        )
                        .with_subject(target_id.to_string())
                })?;

            // --------------------------------------------------------
            // COMPILATION UNITS
            // --------------------------------------------------------

            for source_unit in source_units {
                self.check_cancellation(cancellation, &build_id)?;

                // Account for the invocation before starting work.
                //
                // A failed compilation still consumes operational
                // resources and must remain accounted for.

                budget
                    .consume(
                        invocation_dimension.clone(),
                        1,
                    )
                    .map_err(map_budget_error)?;

                // Invoke the canonical public compiler service.
                //
                // Never call compiler-private passes directly.
                //
                // Never fabricate CompilationOutput.

                let response = self.compiler
                    .compile_text(
                        source_unit.source(),
                        source_unit.source_name(),
                        target_configuration.compiler.clone(),
                    )
                    .map_err(|error| {
                        ToolchainError::build_failed(
                            format!(
                                "compiler failed for source `{}` \
                                 and target `{}`: {error}",
                                source_unit.source_name(),
                                target_id,
                            ),
                        )
                        .with_operation("build.execute.compile")
                        .with_subject(target_id.to_string())
                        .with_context(
                            source_unit.source_name().to_owned(),
                        )
                        .with_source(error)
                    })?;

                self.retain_verified_output(
                    response,
                    &build_id,
                    target_id,
                    source_unit.source_name(),
                    &target_configuration.expected_output_target,
                    verifier,
                    &mut artifacts,
                )?;
            }
        }

        // ------------------------------------------------------------
        // FINAL RESULT VALIDATION
        // ------------------------------------------------------------

        if artifacts.is_empty() {
            return Err(
                ToolchainError::build_failed(
                    "build execution completed without producing \
                     any verified artifacts",
                )
                .with_operation("build.execute")
                .with_subject(build_id.to_string()),
            );
        }

        Ok(BuildExecutionResult {
            build_id,
            artifacts,
            skipped_targets,
        })
    }

    // ====================================================================
    // CANCELLATION
    // ====================================================================

    fn check_cancellation<X: CancellationProbe + ?Sized>(
        &self,
        cancellation: &X,
        build_id: &BuildId,
    ) -> ToolchainResult<()> {
        if cancellation.is_cancelled() {
            return Err(
                ToolchainError::cancelled(
                    "build execution was cancelled",
                )
                .with_operation("build.execute")
                .with_subject(build_id.to_string()),
            );
        }

        Ok(())
    }

    // ====================================================================
    // COMPILATION OUTPUT VALIDATION
    // ====================================================================

    fn validate_output(
        output: &crate::compiler::api::CompilationOutput,
        expected_target: &str,
        source_name: &str,
    ) -> ToolchainResult<()> {
        if output.target != expected_target {
            return Err(
                ToolchainError::validation_failed(
                    format!(
                        "compiler returned target `{}` but target \
                         configuration requires `{}`",
                        output.target,
                        expected_target,
                    ),
                )
                .with_operation("build.execute.output")
                .with_subject(source_name.to_owned()),
            );
        }

        if output.size_bytes != output.bytes.len() {
            return Err(
                ToolchainError::validation_failed(
                    format!(
                        "compiler output size metadata is inconsistent: \
                         declared {}, actual {}",
                        output.size_bytes,
                        output.bytes.len(),
                    ),
                )
                .with_operation("build.execute.output")
                .with_subject(source_name.to_owned()),
            );
        }

        Ok(())
    }

    // ====================================================================
    // ARTIFACT VERIFICATION AND RETENTION
    // ====================================================================

    fn retain_verified_output<V: ArtifactVerifier + ?Sized>(
        &self,
        response: CompilationResponse,
        build_id: &BuildId,
        target_id: &TargetId,
        source_name: &str,
        expected_target: &str,
        verifier: &V,
        artifacts: &mut Vec<VerifiedBuildArtifact>,
    ) -> ToolchainResult<()> {
        // A successful compiler response is necessary but insufficient.
        //
        // The existing compiler output is transferred without copying.

        let output = response.into_output();

        Self::validate_output(
            &output,
            expected_target,
            source_name,
        )?;

        // The authoritative verifier must complete successfully.
        //
        // No artifact is retained as verified before this operation.

        verifier
            .verify(
                build_id,
                target_id,
                source_name,
                &output,
            )
            .map_err(|error| {
                error
                    .with_operation("build.execute.artifact_verification")
                    .with_subject(target_id.to_string())
                    .with_context(source_name.to_owned())
            })?;

        // Fallible capacity reservation precedes ownership transfer.

        artifacts
            .try_reserve(1)
            .map_err(|error| {
                allocation_error(
                    "verified build artifact storage",
                    error,
                )
            })?;

        artifacts.push(VerifiedBuildArtifact {
            build_id: build_id.clone(),
            target_id: target_id.clone(),
            source_name: source_name.to_owned(),
            output,
        });

        Ok(())
    }
}

// ============================================================================
// DEFAULT COMPILER EXECUTOR
// ============================================================================

/// Build executor using the repository's public CompilerApi.
///
/// The underlying compiler pipeline remains responsible for
/// actual compilation.
///
/// This constructor does not imply that the transitional legacy
/// compiler driver has become production-ready.
pub type DefaultBuildExecutor =
    BuildExecutor<crate::compiler::api::CompilerApi>;

impl DefaultBuildExecutor {
    /// Construct an executor using the existing public compiler API.
    #[must_use]
    pub fn with_compiler_api() -> Self {
        Self::new(
            crate::compiler::api::CompilerApi::new(),
        )
    }
}

// ============================================================================
// ERROR HELPERS
// ============================================================================

fn allocation_error(
    subject: &str,
    error: std::collections::TryReserveError,
) -> ToolchainError {
    ToolchainError::resource_exhausted(
        format!(
            "unable to reserve memory for {subject}: {error}",
        ),
    )
    .with_operation("build.execute")
    .with_context(subject.to_owned())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::api::{
        CompilationRequest,
        CompilationResponse,
        CompilerError,
        CompilerService,
    };

    use crate::compiler::CompilationOutput;

    use std::sync::atomic::{
        AtomicUsize,
        Ordering,
    };

    use std::sync::Arc;

    // ------------------------------------------------------------
    // MOCK COMPILER
    // ------------------------------------------------------------

    struct TestCompiler {
        calls: AtomicUsize,
        target: String,
    }

    impl CompilerService for TestCompiler {
        fn compile(
            &self,
            request: CompilationRequest,
        ) -> Result<CompilationResponse, CompilerError> {
            self.calls.fetch_add(
                1,
                Ordering::Relaxed,
            );

            let bytes = request
                .source
                .as_text()
                .unwrap_or_default()
                .as_bytes()
                .to_vec();

            let output = CompilationOutput {
                size_bytes: bytes.len(),
                bytes,
                target: self.target.clone(),
            };

            Ok(CompilationResponse::success(output))
        }
    }

    // ------------------------------------------------------------
    // MOCK VERIFIER
    // ------------------------------------------------------------

    struct TestVerifier;

    impl ArtifactVerifier for TestVerifier {
        fn verify(
            &self,
            _build_id: &BuildId,
            _target_id: &TargetId,
            _source_name: &str,
            _output: &CompilationOutput,
        ) -> ToolchainResult<()> {
            Ok(())
        }
    }

    // ------------------------------------------------------------
    // SOURCE UNIT TESTS
    // ------------------------------------------------------------

    #[test]
    fn source_unit_accepts_valid_source() {
        let unit = BuildSourceUnit::new(
            "main.zm",
            "fn main() {}",
        )
        .expect("valid source unit");

        assert_eq!(
            unit.source_name(),
            "main.zm",
        );

        assert_eq!(
            unit.source(),
            "fn main() {}",
        );
    }

    #[test]
    fn source_unit_rejects_empty_name() {
        assert!(
            BuildSourceUnit::new(
                "",
                "fn main() {}",
            )
            .is_err()
        );
    }

    #[test]
    fn source_unit_permits_empty_source() {
        assert!(
            BuildSourceUnit::new(
                "empty.zm",
                "",
            )
            .is_ok()
        );
    }

    // ------------------------------------------------------------
    // OUTPUT VALIDATION TESTS
    // ------------------------------------------------------------

    #[test]
    fn validates_consistent_compiler_output() {
        let output = CompilationOutput {
            bytes: vec![1, 2, 3],
            size_bytes: 3,
            target: "test-target".to_owned(),
        };

        assert!(
            BuildExecutor::<TestCompiler>::validate_output(
                &output,
                "test-target",
                "main.zm",
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_inconsistent_output_size() {
        let output = CompilationOutput {
            bytes: vec![1, 2, 3],
            size_bytes: 2,
            target: "test-target".to_owned(),
        };

        assert!(
            BuildExecutor::<TestCompiler>::validate_output(
                &output,
                "test-target",
                "main.zm",
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_wrong_output_target() {
        let output = CompilationOutput {
            bytes: vec![1, 2, 3],
            size_bytes: 3,
            target: "wrong-target".to_owned(),
        };

        assert!(
            BuildExecutor::<TestCompiler>::validate_output(
                &output,
                "expected-target",
                "main.zm",
            )
            .is_err()
        );
    }

    // ------------------------------------------------------------
    // COMPILER SERVICE TESTS
    // ------------------------------------------------------------

    #[test]
    fn executor_accepts_injected_compiler_service() {
        let compiler = Arc::new(TestCompiler {
            calls: AtomicUsize::new(0),
            target: "test-target".to_owned(),
        });

        let executor = BuildExecutor::new(
            TestCompiler {
                calls: AtomicUsize::new(0),
                target: "test-target".to_owned(),
            },
        );

        let _ = executor.compiler();

        assert_eq!(
            compiler.calls.load(Ordering::Relaxed),
            0,
        );
    }

    // ------------------------------------------------------------
    // CANCELLATION TESTS
    // ------------------------------------------------------------

    struct CancelImmediately;

    impl CancellationProbe for CancelImmediately {
        fn is_cancelled(&self) -> bool {
            true
        }
    }

    #[test]
    fn cancellation_probe_reports_cancellation() {
        let cancellation = CancelImmediately;

        assert!(cancellation.is_cancelled());

        let never = NeverCancelled;

        assert!(!never.is_cancelled());
    }

    // ------------------------------------------------------------
    // BUDGET INTEGRATION TESTS
    // ------------------------------------------------------------

    #[test]
    fn compiler_invocation_dimension_is_valid() {
        let dimension = compiler_invocation_dimension()
            .expect("valid budget dimension");

        assert_eq!(
            dimension.name(),
            "toolchain.build.compiler_invocations",
        );

        assert_eq!(
            dimension.unit(),
            "operations",
        );
    }

    // ------------------------------------------------------------
    // ARTIFACT VERIFIER CONTRACT TEST
    // ------------------------------------------------------------

    #[test]
    fn verifier_uses_structured_toolchain_result() {
        fn accepts_verifier<V: ArtifactVerifier>(_: &V) {}

        accepts_verifier(&TestVerifier);
    }
}
