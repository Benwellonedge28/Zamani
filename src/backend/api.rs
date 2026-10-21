#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Public API for the Zamani compilation backend.
//!
//! Path: `src/backend/api.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: safe Rust only; `unsafe` is forbidden.
//!
//! # Purpose
//!
//! Provide a stable, documented entry point for applications that need to
//! validate backend configuration, verify the existing legacy ZIR input,
//! and invoke an explicitly registered backend through the canonical backend
//! pipeline.
//!
//! # Architectural authority
//!
//! This API follows the ownership boundaries defined by:
//!
//! - `grammar/Zamani.g4`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/compiler/mod.rs`
//! - `src/compiler_types.rs`
//! - `src/ir_gen.rs`
//! - `src/ir_verify.rs`
//! - `src/backend.rs`
//! - `src/backend/config.rs`
//! - `src/backend/errors.rs`
//! - `src/backend/registry.rs`
//! - `src/backend/pipeline.rs`
//!
//! # Responsibilities
//!
//! This module:
//!
//! - exposes the approved backend-facing API;
//! - validates backend configuration;
//! - offers explicit verification of the existing legacy ZIR representation;
//! - invokes the canonical backend pipeline using an explicit backend ID;
//! - preserves structured backend errors and verification status;
//! - re-exports selected existing types instead of defining competing types.
//!
//! # Non-responsibilities
//!
//! This module does not:
//!
//! - parse Zamani source or define grammar rules;
//! - perform semantic analysis or establish source-program meaning;
//! - introduce a second compiler configuration;
//! - define another backend trait or backend registry;
//! - invent target capabilities or infer a target from the host machine;
//! - lower or rewrite IR;
//! - allocate physical resources or schedule computation;
//! - implement quantum routing, calibration, or error correction;
//! - implement HDL synthesis or timing closure;
//! - write artifacts to disk or execute external processes;
//! - claim that an artifact is portable to every target;
//! - impose universal hardware-capacity limits.
//!
//! # POCO-REAF contract
//!
//! The same source program can be compiled for different targets when the
//! applicable language semantics, IR contracts, target capabilities, and
//! backend implementations permit it.
//!
//! This API never silently replaces an explicitly requested backend with
//! another backend. Unsupported targets, invalid inputs, verification
//! failures, and operational-budget exhaustion remain explicit failures.
//!
//! POCO-REAF is an end-to-end compiler and runtime property; this facade alone
//! cannot establish that every target supports every program.
//!
//! # Integration prerequisites
//!
//! The canonical backend module root must expose the existing backend
//! interface and declare the modules used below. In particular, resolve the
//! competing `src/backend.rs` and `src/backend/mod.rs` module roots before
//! integrating this file into the crate.
//!
//! The legacy `crate::ir_gen::IrModule` remains the current pipeline input.
//! A future canonical-IR migration must introduce an explicit adapter at the
//! relevant integration boundary instead of silently changing this API's
//! input semantics.
//!
//! # Completion boundary
//!
//! This file is complete against its intended facade contract when its
//! imports resolve, its tests pass, the canonical module root exports it,
//! and the existing public compiler API remains compatible. Repository-wide
//! production readiness additionally requires successful compilation,
//! target-specific verification, and integration tests.

use crate::compiler_types::CompilerConfig;
use crate::ir_gen::IrModule;

use super::config::{BackendConfig, ConfigError};
use super::errors::{BackendError, BackendResult};
use super::pipeline::{
    ArtifactVerificationStatus,
    BackendPipeline,
    PipelineOutput,
};

/// The principal types exposed by the backend API.
///
/// These are re-exports of the authoritative implementations. This module
/// does not define competing copies of configuration, errors, or pipeline
/// output types.
pub use super::config::{
    DebugInfoPolicy,
    ExternalToolPolicy,
    OptimizationPolicy,
    OutputDestination,
    OutputPolicy,
    ReproducibilityPolicy,
    ResourceBudgets,
    ToolchainConfig,
    ToolchainKey,
    VerificationPolicy,
};

/// Structured backend error and result types.
pub use super::errors::{
    BackendErrorKind,
    BackendResult as Result,
};

/// Canonical pipeline and verification interfaces.
pub use super::pipeline::{
    ArtifactVerifier,
    BackendPipeline as CompilationPipeline,
    PipelineOutput as CompilationOutput,
};

/// Backend registry types.
///
/// Registration and lookup remain owned by `registry.rs`.
pub use super::registry::{
    BackendDescriptor,
    BackendFactory,
    BackendId,
    BackendRegistry,
    RegistryError,
};

/// Target-selection request and report types.
///
/// Actual selection policy remains owned by `selection.rs`.
pub use super::selection::{
    CandidateEvaluation,
    RejectionReason,
    SelectedTarget,
    SelectionReport,
    SelectionRequest,
    TargetCandidate,
};

/// Target descriptor and identifier types.
///
/// Target metadata remains owned by `target.rs`.
pub use super::target::{
    ArtifactFormat,
    TargetDescriptor,
    TargetId,
};

/// Validate backend-specific configuration.
///
/// This checks the internal consistency of the supplied configuration. It
/// does not claim that an executable exists, that a target supports a
/// capability, or that the filesystem can accept an artifact.
///
/// The embedded `CompilerConfig` remains the source of truth for the existing
/// compiler target and optimization settings.
///
/// # Errors
///
/// Returns a structured backend configuration error if the configuration is
/// internally inconsistent.
pub fn validate_configuration(config: &BackendConfig) -> BackendResult<()> {
    config.validate().map_err(map_config_error)
}

/// Construct backend configuration from the existing compiler configuration.
///
/// This is a convenience adapter, not a second configuration authority.
/// Backend-specific policies receive their documented defaults.
///
/// # Examples
///
/// ```ignore
/// use zamani_compiler::backend::api::{
///     configuration_from_compiler,
///     validate_configuration,
/// };
/// use zamani_compiler::compiler_types::CompilerConfig;
///
/// let compiler_config = CompilerConfig::default();
/// let backend_config = configuration_from_compiler(&compiler_config);
/// validate_configuration(&backend_config)?;
/// # Ok::<(), zamani_compiler::backend::errors::BackendError>(())
/// ```
pub fn configuration_from_compiler(compiler: &CompilerConfig) -> BackendConfig {
    BackendConfig::from_compiler_config(compiler)
}

/// Verify the structural invariants of a legacy ZIR module.
///
/// Verification delegates to the repository's authoritative
/// `crate::ir_verify::verify_module` implementation. No backend is selected
/// and no artifact is generated.
///
/// This check establishes structural verification only; it is not a proof of
/// semantic equivalence, target compatibility, or generated-artifact validity.
///
/// # Errors
///
/// Returns a structured verification error containing the available
/// verifier findings when the module is invalid.
pub fn verify_input_ir(module: &IrModule) -> BackendResult<()> {
    crate::ir_verify::verify_module(module).map_err(|errors| {
        let details = errors
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n");

        BackendError::verification_failed(
            "input ZIR module failed structural verification",
        )
        .with_operation("backend.api.verify_input_ir")
        .with_context(details)
    })
}

/// Compile a legacy ZIR module using an explicitly registered backend.
///
/// This is the principal compilation entry point exposed by this facade.
///
/// The pipeline validates configuration and input IR, enforces configured
/// budgets, resolves the exact requested backend, generates output, and
/// applies the configured artifact-verification policy.
///
/// The `backend_id` must identify a backend registered in the supplied
/// pipeline. It is not inferred from the host architecture, a display label,
/// or a guessed mapping from `CompilationTarget`.
///
/// # Arguments
///
/// - `pipeline`: The configured pipeline and authoritative backend registry.
/// - `module`: The existing legacy ZIR module to compile.
/// - `config`: Compiler settings plus backend policies.
/// - `backend_id`: The exact registered backend identifier to invoke.
///
/// # Errors
///
/// Returns a structured error if configuration or input verification fails,
/// the requested backend cannot be resolved, a configured budget is
/// exhausted, code generation fails, or the requested artifact verification
/// cannot be completed successfully.
///
/// # Verification
///
/// A successful result includes an explicit verification status. Callers
/// must not interpret `NotVerified` or `DisabledByPolicy` as `Verified`.
pub fn compile_with_backend(
    pipeline: &BackendPipeline,
    module: &IrModule,
    config: &BackendConfig,
    backend_id: &str,
) -> BackendResult<PipelineOutput> {
    if backend_id.trim().is_empty() {
        return Err(
            BackendError::invalid_input(
                "backend identifier must not be empty",
            )
            .with_operation("backend.api.compile"),
        );
    }

    validate_configuration(config)?;

    // The pipeline performs authoritative input verification again as part
    // of its complete execution contract. Keeping that check inside the
    // pipeline prevents callers from bypassing it by using the lower-level
    // pipeline API directly.
    pipeline
        .compile(module, config, backend_id)
        .map_err(|error| {
            error.with_context("invoked through backend::api::compile_with_backend")
        })
}

/// Compile using the target explicitly specified in the existing compiler
/// configuration, provided that the caller supplies its deliberate mapping
/// to a registered backend ID.
///
/// The mapping is a caller decision because `CompilationTarget` variants and
/// registered backend IDs are separate contracts. This function does not
/// guess mappings and does not perform silent fallback.
///
/// This helper is intentionally not provided as a function that automatically
/// converts every `CompilationTarget` variant to a backend ID. Such a mapping
/// must be implemented and tested as part of the target-registration and
/// migration work.
pub fn compile_with_registered_backend(
    pipeline: &BackendPipeline,
    module: &IrModule,
    config: &BackendConfig,
    registered_backend_id: &str,
) -> BackendResult<PipelineOutput> {
    compile_with_backend(
        pipeline,
        module,
        config,
        registered_backend_id,
    )
}

/// Return whether the compilation output was verified by the configured
/// artifact verifier.
///
/// This helper intentionally distinguishes successful generation from
/// successful artifact verification.
pub const fn output_is_verified(output: &PipelineOutput) -> bool {
    matches!(
        output.verification,
        ArtifactVerificationStatus::Verified
    )
}

/// Adapt a configuration error to the backend's stable structured error
/// model.
///
/// The original error remains available as a causal description.
fn map_config_error(error: ConfigError) -> BackendError {
    BackendError::invalid_configuration(
        "backend configuration failed validation",
    )
    .with_operation("backend.api.validate_configuration")
    .with_cause(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiler_configuration_adapter_preserves_the_existing_target() {
        let compiler = CompilerConfig::default();
        let backend = configuration_from_compiler(&compiler);

        assert_eq!(
            backend.compiler.target,
            compiler.target,
        );
    }

    #[test]
    fn compiler_configuration_adapter_preserves_existing_settings() {
        let compiler = CompilerConfig::default();
        let backend = configuration_from_compiler(&compiler);

        assert_eq!(
            backend.compiler.debug_info,
            compiler.debug_info,
        );
        assert_eq!(
            backend.compiler.emit_ir,
            compiler.emit_ir,
        );
        assert_eq!(
            backend.compiler.parallel,
            compiler.parallel,
        );
        assert_eq!(
            backend.compiler.verify,
            compiler.verify,
        );
    }

    #[test]
    fn default_backend_configuration_is_valid() {
        let config =
            configuration_from_compiler(&CompilerConfig::default());

        assert!(validate_configuration(&config).is_ok());
    }

    #[test]
    fn empty_backend_identifier_is_rejected_before_pipeline_invocation() {
        let config =
            configuration_from_compiler(&CompilerConfig::default());

        let module = IrModule {
            name: "api_empty_backend_id_test".to_owned(),
            functions: Vec::new(),
            globals: Vec::new(),
            string_literals: Default::default(),
        };

        let pipeline = BackendPipeline::default();

        let result = compile_with_backend(
            &pipeline,
            &module,
            &config,
            "   ",
        );

        assert!(result.is_err());
        let error = result.expect_err("empty backend ID must be rejected");
        assert_eq!(
            error.kind(),
            BackendErrorKind::InvalidInput,
        );
    }

    #[test]
    fn verification_status_is_not_collapsed_to_a_boolean_success() {
        let verified = ArtifactVerificationStatus::Verified;
        let unverified = ArtifactVerificationStatus::NotVerified;
        let disabled = ArtifactVerificationStatus::DisabledByPolicy;

        assert_ne!(verified, unverified);
        assert_ne!(verified, disabled);
        assert_ne!(unverified, disabled);
    }
}