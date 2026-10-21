
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Backend compilation pipeline for Zamani.
//!
//! Path: `src/backend/pipeline.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: safe Rust only.
//!
//! # Responsibility
//!
//! Orchestrate the generic backend stages:
//!
//! 1. Validate backend configuration.
//! 2. Verify the incoming legacy ZIR module.
//! 3. Enforce configured input budgets.
//! 4. Resolve an explicitly requested registered backend.
//! 5. Generate target output.
//! 6. Enforce configured output budgets.
//! 7. Verify the generated artifact according to policy.
//! 8. Return output and verification provenance to the caller.
//!
//! # Architectural boundaries
//!
//! This module does not parse Zamani source, perform semantic analysis,
//! select a target from source syntax, allocate physical resources, schedule
//! quantum or distributed work, write files, or execute external processes.
//!
//! The current legacy `Backend` interface accepts `IrModule` and returns a
//! `String`. The pipeline preserves that interface while making its limits
//! explicit. Typed artifacts and context-aware emitters should replace that
//! boundary through a separately reviewed API migration, not through a
//! second competing backend trait.
//!
//! # POCO-REAF and scalability
//!
//! No universal limit is imposed on CPUs, qubits, GPUs, devices, tensor
//! dimensions, nodes, threads, memory, or other target resources. Configured
//! operational budgets are enforced where this pipeline can measure them.
//! The operating system, verifier, emitter, and toolchain may have additional
//! limits; this pipeline never interprets an absent budget as a guarantee of
//! unlimited resources.
//!
//! # Integration prerequisites
//!
//! The canonical `src/backend/mod.rs` must declare this module and expose the
//! existing `Backend` trait, `CodeGenError` compatibility type, and
//! `BackendRegistry`. The repository currently has a competing
//! `src/backend.rs` implementation and a placeholder `src/backend/mod.rs`;
//! that module ownership conflict must be resolved before crate integration.
//!
//! Input verification delegates to `crate::ir_verify::verify_module`.
//! Configuration comes from `config.rs`; backend lookup and construction
//! come from `registry.rs`; structured errors come from `errors.rs`.
//! Artifact-specific validation is injected because the legacy `Backend`
//! trait does not define a verifier.

use std::sync::Arc;

use crate::backend::config::{
    BackendConfig,
    VerificationPolicy,
};
use crate::backend::errors::{
    BackendError,
    BackendErrorKind,
    BackendResult,
};
use crate::backend::registry::{
    BackendRegistry,
    RegistryError,
};
use crate::ir_gen::IrModule;

/// Outcome of validating a generated artifact.
///
/// `Verified` means the configured artifact verifier returned success.
/// `NotVerified` means output generation succeeded but verification was not
/// requested or no verifier was available under `WhenAvailable`.
///
/// `NotVerified` must never be reported as equivalent to `Verified`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactVerificationStatus {
    /// The configured verifier accepted the artifact.
    Verified,

    /// The artifact was generated but was not verified.
    NotVerified,

    /// Verification was explicitly disabled by configuration.
    DisabledByPolicy,
}

/// Metadata and generated content returned by a successful pipeline run.
///
/// The content remains in memory. This type does not write files, link
/// objects, deploy artifacts, or claim that the artifact is executable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PipelineOutput {
    /// Canonical registered backend identifier.
    pub target: String,

    /// Extension reported by the constructed backend, including its leading
    /// dot if that is the convention used by the backend.
    pub extension: String,

    /// Generated artifact content.
    ///
    /// The legacy backend contract uses a string even for formats that may
    /// eventually need binary artifacts. Binary output requires a future
    /// typed-artifact API rather than lossy string conversion.
    pub content: String,

    /// Number of bytes in `content`, measured as UTF-8 bytes.
    pub size_bytes: usize,

    /// Explicit status of artifact verification.
    pub verification: ArtifactVerificationStatus,
}

/// Validates a generated artifact for a specific backend.
///
/// Implementations should use the appropriate authoritative verifier:
/// for example, an LLVM verifier for LLVM IR, a WebAssembly validator for
/// Wasm, a QASM parser for QASM, or an HDL parser for generated HDL.
///
/// A successful implementation must mean that the supplied content was
/// checked according to the declared format and target. It must not return
/// success merely because the output is non-empty.
///
/// Implementations must not silently rewrite artifact semantics.
pub trait ArtifactVerifier: Send + Sync {
    /// Validate generated output.
    ///
    /// `target` is the canonical registered backend identifier.
    /// `extension` is the extension returned by the backend.
    /// `content` is exactly the output returned by that backend.
    fn verify(
        &self,
        target: &str,
        extension: &str,
        content: &str,
    ) -> BackendResult<()>;
}

/// Backend pipeline coordinator.
///
/// The registry and optional verifier are shared safely between independent
/// pipeline calls. This type contains no mutable per-compilation state.
#[derive(Clone, Default)]
pub struct BackendPipeline {
    registry: BackendRegistry,
    verifier: Option<Arc<dyn ArtifactVerifier>>,
}

impl BackendPipeline {
    /// Construct a pipeline without an artifact verifier.
    ///
    /// With `VerificationPolicy::Required`, a run will fail explicitly.
    /// With `WhenAvailable`, generated output will be marked `NotVerified`.
    /// With `Disabled`, verification will be marked `DisabledByPolicy`.
    #[must_use]
    pub fn new(registry: BackendRegistry) -> Self {
        Self {
            registry,
            verifier: None,
        }
    }

    /// Construct a pipeline with an artifact verifier.
    #[must_use]
    pub fn with_verifier(
        registry: BackendRegistry,
        verifier: Arc<dyn ArtifactVerifier>,
    ) -> Self {
        Self {
            registry,
            verifier: Some(verifier),
        }
    }

    /// Return the registry used by this pipeline.
    ///
    /// This permits controlled registration during application setup. A
    /// production application should finish registering its backends before
    /// starting concurrent compilation jobs.
    #[must_use]
    pub fn registry(&self) -> &BackendRegistry {
        &self.registry
    }

    /// Compile a verified legacy ZIR module with an explicitly requested
    /// registered backend.
    ///
    /// The caller chooses `backend_id` explicitly. This method never guesses
    /// a target from the host machine and never substitutes another backend
    /// when the requested one is missing.
    ///
    /// # Errors
    ///
    /// Returns a structured error if configuration is invalid, the module
    /// fails structural verification, a configured budget is exceeded, the
    /// requested backend cannot be resolved or constructed, generation fails,
    /// required artifact verification is unavailable or fails, or output
    /// exceeds the configured output budget.
    pub fn compile(
        &self,
        module: &IrModule,
        config: &BackendConfig,
        backend_id: &str,
    ) -> BackendResult<PipelineOutput> {
        self.validate_config(config)?;

        let verification_policy = config.verification;

        // Do not generate artifacts if the caller requires verification but
        // this pipeline has no verifier capable of establishing that claim.
        if verification_policy == VerificationPolicy::Required
            && self.verifier.is_none()
        {
            return Err(BackendError::new(
                BackendErrorKind::VerificationFailed,
                "artifact verification is required, but no artifact verifier \
                 is configured for this backend pipeline",
            )
            .with_operation("backend.pipeline.preflight")
            .with_context(format!("requested backend: {backend_id}")));
        }

        self.enforce_input_budget(module, config)?;

        // Structural verification is mandatory regardless of the artifact
        // verification policy. Disabling output verification must never
        // disable input IR verification.
        crate::ir_verify::verify_module(module).map_err(|errors| {
            let details = errors
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n");

            BackendError::verification_failed(
                "input ZIR module failed structural verification",
            )
            .with_operation("backend.pipeline.verify_input")
            .with_context(details)
        })?;

        // Resolve and construct the exact requested backend. The registry
        // validates factory identity; no default or fallback target is used.
        let mut backend = self
            .registry
            .create(backend_id)
            .map_err(|error| self.map_registry_error(backend_id, error))?;

        let canonical_target = backend.target_name().to_owned();
        let extension = backend.file_extension().to_owned();

        if canonical_target.trim().is_empty() {
            return Err(BackendError::internal(
                "registered backend returned an empty target identifier",
            )
            .with_operation("backend.pipeline.resolve")
            .with_context(format!("requested backend: {backend_id}")));
        }

        if extension.trim().is_empty() {
            return Err(BackendError::internal(
                "registered backend returned an empty artifact extension",
            )
            .with_operation("backend.pipeline.resolve")
            .with_target(canonical_target));
        }

        // The legacy trait does not accept BackendConfig or a compilation
        // context. Therefore this call cannot itself enforce emitter-specific
        // optimization, debug-info, target flags, or process isolation.
        // Such behavior must be owned by the backend implementation or by a
        // future context-aware emitter interface.
        let content = backend.generate(module).map_err(|error| {
            BackendError::new(
                BackendErrorKind::Legacy,
                "backend code generation failed",
            )
            .with_target(error.target)
            .with_operation("backend.pipeline.generate")
            .with_cause(error.message)
            .with_context(format!("registered backend: {canonical_target}"))
        })?;

        let size_bytes = content.len();

        self.enforce_output_budget(size_bytes, config, &canonical_target)?;

        let verification = match verification_policy {
            VerificationPolicy::Disabled => {
                ArtifactVerificationStatus::DisabledByPolicy
            }

            VerificationPolicy::WhenAvailable => {
                if let Some(verifier) = &self.verifier {
                    verifier
                        .verify(&canonical_target, &extension, &content)
                        .map_err(|error| {
                            error
                                .with_target(canonical_target.clone())
                                .with_operation("backend.pipeline.verify_artifact")
                        })?;

                    ArtifactVerificationStatus::Verified
                } else {
                    ArtifactVerificationStatus::NotVerified
                }
            }

            VerificationPolicy::Required => {
                // The preflight above guarantees that a verifier is present.
                // Retain a defensive check so a future refactor cannot turn a
                // missing verifier into a successful verification claim.
                let verifier = self.verifier.as_ref().ok_or_else(|| {
                    BackendError::new(
                        BackendErrorKind::VerificationFailed,
                        "required artifact verifier is unavailable",
                    )
                    .with_target(canonical_target.clone())
                    .with_operation("backend.pipeline.verify_artifact")
                })?;

                verifier
                    .verify(&canonical_target, &extension, &content)
                    .map_err(|error| {
                        error
                            .with_target(canonical_target.clone())
                            .with_operation("backend.pipeline.verify_artifact")
                    })?;

                ArtifactVerificationStatus::Verified
            }
        };

        Ok(PipelineOutput {
            target: canonical_target,
            extension,
            content,
            size_bytes,
            verification,
        })
    }

    fn validate_config(&self, config: &BackendConfig) -> BackendResult<()> {
        config.validate().map_err(|error| {
            BackendError::invalid_configuration(
                "backend configuration failed validation",
            )
            .with_operation("backend.pipeline.validate_config")
            .with_cause(error.to_string())
        })
    }

    fn enforce_input_budget(
        &self,
        module: &IrModule,
        config: &BackendConfig,
    ) -> BackendResult<()> {
        if let Some(limit) = config.budgets.max_input_bytes {
            // The legacy IR has no reliable allocation-size or encoded-size
            // API. Its canonical textual representation is used here as a
            // deterministic proxy. This is not a measurement of the module's
            // in-memory allocation footprint.
            let serialized = module.to_ir_string();
            let observed = serialized.len() as u128;

            if observed > limit {
                return Err(BackendError::resource_exhausted(format!(
                    "serialized input IR size ({observed} bytes) exceeds \
                     configured input budget ({limit} bytes)"
                ))
                .with_operation("backend.pipeline.input_budget"));
            }
        }

        Ok(())
    }

    fn enforce_output_budget(
        &self,
        size_bytes: usize,
        config: &BackendConfig,
        target: &str,
    ) -> BackendResult<()> {
        if let Some(limit) = config.budgets.max_output_bytes {
            let observed = size_bytes as u128;

            if observed > limit {
                return Err(BackendError::resource_exhausted(format!(
                    "generated output size ({observed} bytes) exceeds \
                     configured output budget ({limit} bytes)"
                ))
                .with_target(target.to_owned())
                .with_operation("backend.pipeline.output_budget"));
            }
        }

        Ok(())
    }

    fn map_registry_error(
        &self,
        requested_id: &str,
        error: RegistryError,
    ) -> BackendError {
        BackendError::unsupported_target(
            requested_id.to_owned(),
            "the requested backend could not be resolved or constructed",
        )
        .with_operation("backend.pipeline.resolve")
        .with_cause(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_size_is_utf8_byte_size() {
        let content = "Zamani λ";
        assert_eq!(content.len(), content.as_bytes().len());
        assert!(content.len() > content.chars().count());
    }

    #[test]
    fn verification_status_distinguishes_unverified_output() {
        assert_ne!(
            ArtifactVerificationStatus::Verified,
            ArtifactVerificationStatus::NotVerified,
        );

        assert_ne!(
            ArtifactVerificationStatus::Verified,
            ArtifactVerificationStatus::DisabledByPolicy,
        );
    }
}
