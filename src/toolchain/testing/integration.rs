#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Integration Testing
//!
//! Path: `src/toolchain/testing/integration.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides reusable integration-test boundaries for exercising the
//! actual Zamani compiler service and the complete toolchain build
//! command.
//!
//! The module supports two levels of integration:
//!
//! 1. Compiler integration: submit a real compilation request through
//!    the canonical `CompilerService`.
//!
//! 2. Build integration: submit a real public toolchain build request
//!    through `BuildCommand`, which coordinates authoritative build
//!    preparation, compiler execution and artifact verification.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Compiler-service integration evidence.
//! - Build-command integration evidence.
//! - Validation of returned integration metadata.
//! - Reusable integration harnesses.
//! - Structured integration failures.
//!
//! This module does not own:
//!
//! - Source grammar, parsing or semantic analysis.
//! - Project or workspace discovery.
//! - Manifest parsing or validation.
//! - Dependency resolution.
//! - Target registration or discovery.
//! - Capability matching or feasibility analysis.
//! - Build-plan construction.
//! - Physical resource planning or allocation.
//! - Compiler implementation.
//! - Backend selection or code generation.
//! - External process execution.
//! - Artifact storage or cryptographic verification.
//! - Hardware discovery or allocation.
//! - Runtime execution.
//! - Distributed scheduling.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! ## Canonical integration
//!
//! Compiler API:
//!     crate::compiler::api
//!
//! Compiler service:
//!     crate::compiler::api::CompilerService
//!
//! Compiler errors:
//!     crate::compiler::api::CompilerError
//!
//! Compiler output:
//!     crate::compiler::api::CompilationOutput
//!
//! Toolchain build command:
//!     crate::toolchain::commands::build
//!
//! Build preparation:
//!     crate::toolchain::commands::build::BuildPreparationProvider
//!
//! Build executor:
//!     crate::toolchain::build::executor
//!
//! Artifact verification:
//!     crate::toolchain::build::executor::ArtifactVerifier
//!
//! Build identity:
//!     crate::toolchain::ids::BuildId
//!
//! Conformance:
//!     crate::toolchain::testing::conformance
//!
//! Fixtures:
//!     crate::toolchain::testing::fixtures
//!
//! Grammar:
//!     grammar/Zamani.g4
//!     grammar/antlr/ZamaniLexer.g4
//!     grammar/antlr/ZamaniParser.g4
//!     grammar/MANIFEST.yaml
//!
//! Normative portability:
//!     grammar/specification/poco-reaf.md
//!
//! Semantic boundaries:
//!     grammar/validation/semantic-boundaries.md
//!
//! ## Integration lifecycle
//!
//! A compiler integration operation follows:
//!
//! ```text
//! Explicit CompilationRequest
//!           |
//!           v
//! Canonical CompilerService
//!           |
//!           v
//! Existing compiler implementation
//!           |
//!           v
//! CompilationResponse
//!           |
//!           v
//! Integration evidence validation
//! ```
//!
//! A complete toolchain build integration follows:
//!
//! ```text
//! Public BuildRequest
//!           |
//!           v
//! BuildCommand
//!           |
//!           v
//! Authoritative BuildPreparationProvider
//!           |
//!           v
//! Validated and assessed build plan
//!           |
//!           v
//! BuildExecutor
//!           |
//!           v
//! Canonical CompilerService
//!           |
//!           v
//! Backend and artifact production
//!           |
//!           v
//! Authoritative ArtifactVerifier
//!           |
//!           v
//! BuildCommandResult
//!           |
//!           v
//! Integration evidence validation
//! ```
//!
//! This module does not replace either lifecycle.
//!
//! ## Evidence rules
//!
//! - A successful compiler call must return internally consistent output.
//! - A successful build must contain verified artifacts.
//! - Every returned artifact must belong to the logical build being
//!   reported.
//! - Artifact target and source identities must be retained.
//! - Compiler output size metadata must match the actual byte length.
//! - Errors must remain errors; they must never become successful
//!   integration evidence.
//! - A mock compiler or mock verifier can test an isolated contract,
//!   but cannot establish real backend or hardware compatibility.
//! - A successful build does not prove runtime execution.
//!
//! ## POCO-REAF
//!
//! Integration evidence preserves the distinction between one logical
//! build and its target-specific artifacts.
//!
//! It does not assume that different targets produce identical bytes.
//!
//! It does not assume that every target supports every domain.
//!
//! It does not treat a requested target as proof of feasibility.
//!
//! It does not claim that physical resources are infinite.
//!
//! ## Scalability
//!
//! This module introduces no arbitrary maximum for:
//!
//! - Source length.
//! - Build artifacts.
//! - Targets.
//! - Computing domains.
//! - Projects or workspace members.
//! - Dependencies.
//! - Hardware resources.
//! - Quantum resources.
//! - Tensor dimensions.
//! - Distributed participants.
//!
//! Collection allocation is fallible where this module creates
//! variable-sized evidence collections.
//!
//! Actual execution remains subject to available memory, address space,
//! compiler capabilities, operating-system constraints and explicit
//! operational budgets.
//!
//! ## Concurrency
//!
//! This module creates no global mutable state, worker pool, scheduler,
//! background thread or process-wide service.
//!
//! The supplied compiler and build services retain their own concurrency
//! and synchronization contracts.
//!
//! ## Security
//!
//! - Unsafe Rust is prohibited.
//! - Requests are passed to canonical services for validation.
//! - No shell command strings are constructed.
//! - No external process is launched by this module.
//! - No credentials are read or stored.
//! - No provider metadata is treated as authenticated merely because
//!   it appears in a successful response.
//! - Artifact verification remains the responsibility of the injected
//!   authoritative verifier.
//!
//! ## Production-readiness boundary
//!
//! This module verifies integration behavior at its public boundaries.
//! It cannot certify the internal correctness of a compiler driver,
//! backend, artifact verifier, hardware provider or runtime.
//!
//! Each implementation must provide its own conformance evidence.

use std::error::Error;
use std::fmt;

use crate::compiler::api::{
    CompilationRequest,
    CompilationResponse,
    CompilationService as _,
    CompilerError,
    CompilerService,
};

use crate::toolchain::api::request::BuildRequest;

use crate::toolchain::build::executor::VerifiedBuildArtifact;

use crate::toolchain::commands::build::{
    BuildCommand,
    BuildPreparationProvider,
};

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::ids::BuildId;
use crate::toolchain::result::ToolchainResult;

// ============================================================================
// ERROR CONTRACT
// ============================================================================

/// Errors produced while validating compiler or build integration.
///
/// The original compiler and toolchain errors are retained as error
/// sources. Integration validation failures are classified separately
/// from failures originating in the underlying services.
#[derive(Debug)]
pub enum IntegrationError {
    /// The canonical compiler service returned an error.
    Compiler(CompilerError),

    /// The canonical toolchain build command returned an error.
    Build(ToolchainError),

    /// A service returned inconsistent or incomplete result metadata.
    InvalidEvidence {
        /// Integration stage that produced the invalid result.
        stage: &'static str,

        /// Explanation of the violated result contract.
        reason: String,
    },

    /// The integration harness could not reserve memory for its report.
    Allocation {
        /// Integration stage that needed the allocation.
        stage: &'static str,

        /// Allocation failure description.
        reason: String,
    },
}

impl IntegrationError {
    /// Construct an invalid-evidence error.
    fn invalid(
        stage: &'static str,
        reason: impl Into<String>,
    ) -> Self {
        Self::InvalidEvidence {
            stage,
            reason: reason.into(),
        }
    }

    /// Construct an allocation error.
    fn allocation(
        stage: &'static str,
        error: std::collections::TryReserveError,
    ) -> Self {
        Self::Allocation {
            stage,
            reason: error.to_string(),
        }
    }

    /// Return the integration stage associated with this error.
    #[must_use]
    pub const fn stage(&self) -> &'static str {
        match self {
            Self::Compiler(_) => "compiler",
            Self::Build(_) => "build",
            Self::InvalidEvidence { stage, .. } => stage,
            Self::Allocation { stage, .. } => stage,
        }
    }
}

impl fmt::Display for IntegrationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Compiler(error) => {
                write!(formatter, "compiler integration failed: {error}")
            }

            Self::Build(error) => {
                write!(formatter, "toolchain build integration failed: {error}")
            }

            Self::InvalidEvidence { stage, reason } => {
                write!(
                    formatter,
                    "invalid {stage} integration evidence: {reason}"
                )
            }

            Self::Allocation { stage, reason } => {
                write!(
                    formatter,
                    "unable to allocate {stage} integration evidence: {reason}"
                )
            }
        }
    }
}

impl Error for IntegrationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Compiler(error) => Some(error),
            Self::Build(error) => Some(error),
            Self::InvalidEvidence { .. } | Self::Allocation { .. } => None,
        }
    }
}

/// Result returned by integration-test operations.
pub type IntegrationResult<T> = Result<T, IntegrationError>;

// ============================================================================
// COMPILER INTEGRATION EVIDENCE
// ============================================================================

/// Validated evidence from one compiler-service invocation.
///
/// This structure records facts returned by the canonical compiler
/// service. It does not independently prove compiler correctness,
/// backend correctness, artifact portability or runtime compatibility.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilerIntegrationEvidence {
    source_name: String,
    output_target: String,
    output_size_bytes: usize,
}

impl CompilerIntegrationEvidence {
    /// Validate and construct evidence from an actual compiler response.
    ///
    /// The expected source name is captured from the original request.
    /// The response must preserve that identity.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The response source name differs from the request.
    /// - The reported target is empty.
    /// - The reported output size differs from the returned byte length.
    fn from_response(
        expected_source_name: String,
        response: &CompilationResponse,
    ) -> IntegrationResult<Self> {
        if response.source_name != expected_source_name {
            return Err(IntegrationError::invalid(
                "compiler",
                format!(
                    "compiler changed source identity: expected `{expected_source_name}`, \
                     received `{}`",
                    response.source_name
                ),
            ));
        }

        let output = response.output();

        if output.target.trim().is_empty() {
            return Err(IntegrationError::invalid(
                "compiler",
                "compiler returned an empty output target",
            ));
        }

        if output.size_bytes != output.bytes.len() {
            return Err(IntegrationError::invalid(
                "compiler",
                format!(
                    "compiler output size metadata is inconsistent: \
                     reported {}, actual {}",
                    output.size_bytes,
                    output.bytes.len()
                ),
            ));
        }

        Ok(Self {
            source_name: response.source_name.clone(),
            output_target: output.target.clone(),
            output_size_bytes: output.bytes.len(),
        })
    }

    /// Return the logical source name supplied to the compiler.
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    /// Return the target label reported by the compiler.
    ///
    /// This is the compiler's reported output target, not independent
    /// proof that a physical target exists or is compatible.
    #[must_use]
    pub fn output_target(&self) -> &str {
        &self.output_target
    }

    /// Return the actual compiler-output byte length.
    #[must_use]
    pub const fn output_size_bytes(&self) -> usize {
        self.output_size_bytes
    }
}

// ============================================================================
// COMPILER SERVICE INTEGRATION
// ============================================================================

/// Invoke the canonical compiler service and validate its response.
///
/// This function deliberately accepts the existing `CompilerService`
/// abstraction. It does not instantiate a second compiler, choose a
/// target, infer host capabilities or manufacture compiler output.
///
/// Callers must supply an explicitly configured service and request.
///
/// A real production integration should use the repository's intended
/// compiler driver. A mock driver is appropriate for isolated service
/// contract tests only.
///
/// # Errors
///
/// Returns the original compiler error when compilation fails.
///
/// Returns an integration error when the response violates the
/// source-identity, target-identity or output-size contract.
pub fn compile_through_service<S>(
    service: &S,
    request: CompilationRequest,
) -> IntegrationResult<CompilerIntegrationEvidence>
where
    S: CompilerService + ?Sized,
{
    let expected_source_name = request.source.source_name_owned();

    let response = service
        .compile(request)
        .map_err(IntegrationError::Compiler)?;

    CompilerIntegrationEvidence::from_response(
        expected_source_name,
        &response,
    )
}

// ============================================================================
// BUILD ARTIFACT EVIDENCE
// ============================================================================

/// Validated metadata for one artifact returned by a completed build.
///
/// The artifact itself remains owned by the canonical compiler/backend
/// artifact infrastructure. This structure records its identity and
/// output metadata without creating another artifact store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildArtifactEvidence {
    build_id: BuildId,
    target_id: crate::compiler::configuration::target::TargetId,
    source_name: String,
    output_target: String,
    output_size_bytes: usize,
}

impl BuildArtifactEvidence {
    /// Validate and record one artifact returned by the build executor.
    ///
    /// The artifact must belong to the expected logical build.
    ///
    /// # Errors
    ///
    /// Returns an error when the artifact identity or output metadata
    /// is inconsistent.
    fn from_artifact(
        expected_build_id: &BuildId,
        artifact: &VerifiedBuildArtifact,
    ) -> IntegrationResult<Self> {
        if artifact.build_id() != expected_build_id {
            return Err(IntegrationError::invalid(
                "build",
                "artifact belongs to a different logical build",
            ));
        }

        if artifact.source_name().trim().is_empty() {
            return Err(IntegrationError::invalid(
                "build",
                "verified artifact has an empty source identity",
            ));
        }

        let output = artifact.output();

        if output.target.trim().is_empty() {
            return Err(IntegrationError::invalid(
                "build",
                "verified artifact has an empty output target",
            ));
        }

        if output.size_bytes != output.bytes.len() {
            return Err(IntegrationError::invalid(
                "build",
                format!(
                    "artifact output size metadata is inconsistent: \
                     reported {}, actual {}",
                    output.size_bytes,
                    output.bytes.len()
                ),
            ));
        }

        Ok(Self {
            build_id: artifact.build_id().clone(),
            target_id: artifact.target_id().clone(),
            source_name: artifact.source_name().to_owned(),
            output_target: output.target.clone(),
            output_size_bytes: output.bytes.len(),
        })
    }

    /// Return the logical build identity.
    #[must_use]
    pub fn build_id(&self) -> &BuildId {
        &self.build_id
    }

    /// Return the canonical compiler target identity.
    #[must_use]
    pub fn target_id(
        &self,
    ) -> &crate::compiler::configuration::target::TargetId {
        &self.target_id
    }

    /// Return the logical source identity.
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    /// Return the target label reported by the compiler output.
    #[must_use]
    pub fn output_target(&self) -> &str {
        &self.output_target
    }

    /// Return the actual artifact byte length.
    #[must_use]
    pub const fn output_size_bytes(&self) -> usize {
        self.output_size_bytes
    }
}

// ============================================================================
// BUILD INTEGRATION REPORT
// ============================================================================

/// Validated evidence from one completed logical toolchain build.
///
/// The report preserves the single logical build identity and the
/// identities of its target-specific artifacts.
///
/// A report does not claim that the resulting artifacts have been
/// executed, deployed or proven portable to every possible target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildIntegrationReport {
    build_id: BuildId,
    artifacts: Vec<BuildArtifactEvidence>,
    skipped_target_count: usize,
}

impl BuildIntegrationReport {
    /// Return the logical build identity.
    #[must_use]
    pub fn build_id(&self) -> &BuildId {
        &self.build_id
    }

    /// Borrow the validated target-specific artifact evidence.
    #[must_use]
    pub fn artifacts(&self) -> &[BuildArtifactEvidence] {
        &self.artifacts
    }

    /// Return the number of verified artifacts.
    #[must_use]
    pub fn artifact_count(&self) -> usize {
        self.artifacts.len()
    }

    /// Return the number of assessed targets that were not built.
    ///
    /// A skipped target is not counted as a successful realization.
    #[must_use]
    pub const fn skipped_target_count(&self) -> usize {
        self.skipped_target_count
    }

    /// Return whether the build produced at least one verified artifact.
    #[must_use]
    pub fn has_verified_artifacts(&self) -> bool {
        !self.artifacts.is_empty()
    }

    /// Return whether the build completed without skipped targets.
    ///
    /// This reports build-result coverage only. It does not establish
    /// that every possible target or provider was discovered.
    #[must_use]
    pub const fn has_no_skipped_targets(&self) -> bool {
        self.skipped_target_count == 0
    }

    /// Construct a report from an actual completed build result.
    ///
    /// This method is private because reports must originate from the
    /// canonical build-command result path.
    fn from_result(
        result: &crate::toolchain::commands::build::BuildCommandResult,
    ) -> IntegrationResult<Self> {
        let build_id = result.build_id().clone();
        let returned_artifacts = result.artifacts();

        if returned_artifacts.is_empty() {
            return Err(IntegrationError::invalid(
                "build",
                "completed build returned no verified artifacts",
            ));
        }

        let mut artifacts = Vec::new();

        artifacts
            .try_reserve(returned_artifacts.len())
            .map_err(|error| {
                IntegrationError::allocation(
                    "build artifact report",
                    error,
                )
            })?;

        for artifact in returned_artifacts {
            artifacts.push(BuildArtifactEvidence::from_artifact(
                &build_id,
                artifact,
            )?);
        }

        Ok(Self {
            build_id,
            artifacts,
            skipped_target_count: result
                .execution()
                .skipped_targets()
                .len(),
        })
    }
}

// ============================================================================
// BUILD COMMAND INTEGRATION HARNESS
// ============================================================================

/// Reusable integration harness for the canonical toolchain build command.
///
/// The harness owns no compiler or build preparation implementation.
/// Both are supplied through the existing public service contracts.
///
/// The preparation provider must use the repository's authoritative
/// workspace, manifest, dependency, target and compiler-planning
/// implementations.
///
/// The compiler service must use the intended compiler driver.
///
/// The artifact verifier must delegate to the canonical artifact
/// verification authority.
///
/// No fake default providers are supplied.
pub struct BuildIntegrationHarness<P, C>
where
    P: BuildPreparationProvider,
    C: CompilerService,
{
    command: BuildCommand<P, C>,
}

impl<P, C> BuildIntegrationHarness<P, C>
where
    P: BuildPreparationProvider,
    C: CompilerService,
{
    /// Construct an integration harness with explicit production
    /// or test implementations of the canonical service contracts.
    ///
    /// # Arguments
    ///
    /// - `preparation`: authoritative build-preparation integration.
    /// - `compiler`: canonical compiler service.
    ///
    /// A production harness must not use mock preparation, compiler
    /// or artifact-verification implementations as proof of actual
    /// backend or hardware compatibility.
    #[must_use]
    pub fn new(
        preparation: P,
        compiler: C,
    ) -> Self {
        Self {
            command: BuildCommand::new(
                preparation,
                compiler,
            ),
        }
    }

    /// Execute a public build request through the real build-command
    /// integration boundary.
    ///
    /// This operation delegates to `BuildCommand::execute`.
    ///
    /// It therefore exercises the canonical request validation,
    /// preparation, build-plan, compiler-execution and artifact-
    /// verification boundaries supplied to that command.
    ///
    /// # Errors
    ///
    /// Returns the original structured toolchain error when request
    /// validation, preparation, feasibility, compilation, cancellation,
    /// budget accounting or artifact verification fails.
    ///
    /// Returns an integration error if the successful result violates
    /// the build evidence contract.
    pub fn execute(
        &self,
        request: BuildRequest,
    ) -> IntegrationResult<BuildIntegrationReport> {
        let result = self
            .command
            .execute(request)
            .map_err(IntegrationError::Build)?;

        BuildIntegrationReport::from_result(&result)
    }

    /// Borrow the underlying canonical build command.
    ///
    /// This permits integration tests to inspect the configured
    /// preparation and compiler service without creating alternate
    /// implementations.
    #[must_use]
    pub fn command(&self) -> &BuildCommand<P, C> {
        &self.command
    }
}

// ============================================================================
// INTEGRATION CONTRACT ASSERTIONS
// ============================================================================

/// Validate the public invariants of compiler integration evidence.
///
/// This helper is useful when integration reports are passed between
/// test layers or retained for later conformance evaluation.
///
/// # Errors
///
/// Returns an error if required identity or target information is empty.
pub fn validate_compiler_evidence(
    evidence: &CompilerIntegrationEvidence,
) -> IntegrationResult<()> {
    if evidence.source_name.trim().is_empty() {
        return Err(IntegrationError::invalid(
            "compiler",
            "source identity must not be empty",
        ));
    }

    if evidence.output_target.trim().is_empty() {
        return Err(IntegrationError::invalid(
            "compiler",
            "output target must not be empty",
        ));
    }

    Ok(())
}

/// Validate the public invariants of a completed build report.
///
/// At least one verified artifact is required. Every artifact must
/// retain the report's logical build identity and non-empty source
/// and target information.
///
/// # Errors
///
/// Returns an error if the report is incomplete or inconsistent.
pub fn validate_build_report(
    report: &BuildIntegrationReport,
) -> IntegrationResult<()> {
    if report.artifacts.is_empty() {
        return Err(IntegrationError::invalid(
            "build",
            "build report contains no verified artifacts",
        ));
    }

    for artifact in &report.artifacts {
        if artifact.build_id != report.build_id {
            return Err(IntegrationError::invalid(
                "build",
                "artifact build identity differs from report identity",
            ));
        }

        if artifact.source_name.trim().is_empty() {
            return Err(IntegrationError::invalid(
                "build",
                "artifact source identity must not be empty",
            ));
        }

        if artifact.output_target.trim().is_empty() {
            return Err(IntegrationError::invalid(
                "build",
                "artifact output target must not be empty",
            ));
        }
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::api::CompilationOutput;

    fn valid_compiler_response() -> CompilationResponse {
        let bytes = vec![0x5a, 0x4d, 0x01];

        CompilationResponse::new(
            "integration.zm",
            CompilationOutput {
                size_bytes: bytes.len(),
                bytes,
                target: "integration-test-target".to_owned(),
            },
        )
    }

    #[test]
    fn accepts_consistent_compiler_response() {
        let response = valid_compiler_response();

        let evidence =
            CompilerIntegrationEvidence::from_response(
                "integration.zm".to_owned(),
                &response,
            )
            .expect("consistent response must be accepted");

        assert_eq!(evidence.source_name(), "integration.zm");
        assert_eq!(
            evidence.output_target(),
            "integration-test-target"
        );
        assert_eq!(evidence.output_size_bytes(), 3);
        assert!(validate_compiler_evidence(&evidence).is_ok());
    }

    #[test]
    fn rejects_changed_compiler_source_identity() {
        let response = valid_compiler_response();

        let result =
            CompilerIntegrationEvidence::from_response(
                "different.zm".to_owned(),
                &response,
            );

        assert!(matches!(
            result,
            Err(IntegrationError::InvalidEvidence {
                stage: "compiler",
                ..
            })
        ));
    }

    #[test]
    fn rejects_empty_compiler_output_target() {
        let response = CompilationResponse::new(
            "integration.zm",
            CompilationOutput {
                bytes: Vec::new(),
                size_bytes: 0,
                target: String::new(),
            },
        );

        let result =
            CompilerIntegrationEvidence::from_response(
                "integration.zm".to_owned(),
                &response,
            );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_inconsistent_compiler_output_size() {
        let response = CompilationResponse::new(
            "integration.zm",
            CompilationOutput {
                bytes: vec![1, 2, 3],
                size_bytes: 2,
                target: "integration-test-target".to_owned(),
            },
        );

        let result =
            CompilerIntegrationEvidence::from_response(
                "integration.zm".to_owned(),
                &response,
            );

        assert!(result.is_err());
    }

    #[test]
    fn compiler_integration_uses_canonical_service_trait() {
        fn accepts_service<S: CompilerService + ?Sized>(
            service: &S,
            request: CompilationRequest,
        ) -> IntegrationResult<CompilerIntegrationEvidence> {
            compile_through_service(service, request)
        }

        let _ = accepts_service;
    }

    #[test]
    fn build_integration_uses_canonical_build_command() {
        fn accepts_harness<P, C>(
            harness: &BuildIntegrationHarness<P, C>,
            request: BuildRequest,
        ) -> IntegrationResult<BuildIntegrationReport>
        where
            P: BuildPreparationProvider,
            C: CompilerService,
        {
            harness.execute(request)
        }

        let _ = accepts_harness;
    }

    #[test]
    fn integration_errors_preserve_their_stage() {
        let error = IntegrationError::invalid(
            "compiler",
            "invalid test evidence",
        );

        assert_eq!(error.stage(), "compiler");
        assert!(error.to_string().contains("invalid test evidence"));
    }

    #[test]
    fn allocation_errors_are_distinguishable() {
        let error = IntegrationError::Allocation {
            stage: "build artifact report",
            reason: "test allocation failure".to_owned(),
        };

        assert_eq!(error.stage(), "build");
        assert!(error.to_string().contains("allocation"));
    }
}