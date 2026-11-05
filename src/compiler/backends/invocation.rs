
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Backend Invocation
//!
//! Path: `src/compiler/backends/invocation.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Invoke an explicitly selected, registered backend using the canonical
//! backend registry and the existing backend generation interface.
//!
//! This module is the final compiler-side invocation adapter. It connects
//! backend selection to actual backend construction and code generation.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - backend invocation request validation;
//! - canonical backend identifier and alias resolution;
//! - backend construction through the canonical registry;
//! - backend implementation metadata consistency checks;
//! - invocation of the existing backend generation interface;
//! - structured invocation results and errors;
//! - explicit distinction between generated text and verified artifacts.
//!
//! This module does not own:
//!
//! - backend registration;
//! - backend discovery;
//! - capability models;
//! - semantic requirement matching;
//! - target compatibility evaluation;
//! - hardware allocation;
//! - physical device discovery;
//! - IR construction or transformation;
//! - IR verification;
//! - target-specific code generation algorithms;
//! - filesystem artifact writing;
//! - linking;
//! - deployment;
//! - runtime execution;
//! - scheduling or routing.
//!
//! ## POCO-REAF
//!
//! The compiler may invoke any registered backend that has been selected
//! through the appropriate compatibility and feasibility processes.
//!
//! Backend invocation does not imply that every backend supports every
//! program or that every physical target is available.
//!
//! A backend failure must be propagated explicitly. This module never
//! substitutes a different backend or silently changes program semantics.
//!
//! ## Scalability
//!
//! No fixed maximum is imposed on:
//!
//! - registered backends;
//! - target architectures;
//! - computational domains;
//! - IR module sizes;
//! - generated output sizes;
//! - hardware resource counts;
//! - accelerator counts;
//! - future backend types.
//!
//! Actual memory, time, and output constraints are determined by the
//! available resources and the surrounding compilation budget policies.
//!
//! ## Output trust boundary
//!
//! The existing backend API returns generated text. Generated text is not
//! automatically a verified executable artifact.
//!
//! This module therefore labels successful output as generated but
//! unverified. Artifact validation, serialization, hashing, linking,
//! packaging, and deployment belong to their respective subsystems.
//!
//! ## Required integration
//!
//! `src/compiler/backends/mod.rs` must declare:
//!
//!     pub mod invocation;
//!
//! `src/backend/mod.rs` must expose the canonical backend registry and
//! resolve its existing legacy backend module migration.
//!
//! `src/compiler/mod.rs` must expose the compiler backend facade.
//!
//! No second Backend trait or registry is introduced.

use std::error::Error;
use std::fmt;

use crate::backend::registry::{
    BackendDescriptor,
    BackendRegistry,
    RegistryError,
};

use crate::ir_gen::IrModule;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable identity of this compiler backend-invocation module.
pub const BACKEND_INVOCATION_MODULE_NAME: &str =
    "zamani.compiler.backends.invocation";

/// Public API version of this module.
///
/// This version is independent of the Zamani language version, backend
/// implementation versions, and IR format versions.
pub const BACKEND_INVOCATION_API_VERSION: u16 = 1;

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

/// Validate a backend identifier supplied to the invocation boundary.
///
/// Canonical identifier syntax and alias uniqueness remain the
/// responsibility of the canonical backend registry.
///
/// This boundary rejects empty identifiers, surrounding whitespace,
/// and control characters.
///
/// No arbitrary identifier length limit is imposed.
fn validate_backend_identifier(
    identifier: &str,
) -> Result<(), BackendInvocationError> {
    if identifier.trim().is_empty() {
        return Err(
            BackendInvocationError::InvalidBackendIdentifier {
                identifier: identifier.to_owned(),
                reason: "identifier must not be empty".to_owned(),
            },
        );
    }

    if identifier.trim() != identifier {
        return Err(
            BackendInvocationError::InvalidBackendIdentifier {
                identifier: identifier.to_owned(),
                reason:
                    "identifier must not contain surrounding whitespace"
                        .to_owned(),
            },
        );
    }

    if identifier.chars().any(char::is_control) {
        return Err(
            BackendInvocationError::InvalidBackendIdentifier {
                identifier: identifier.to_owned(),
                reason:
                    "identifier must not contain control characters"
                        .to_owned(),
            },
        );
    }

    Ok(())
}

// ============================================================================
// INVOCATION REQUEST
// ============================================================================

/// Immutable request to invoke a registered backend.
///
/// The request borrows the canonical IR module. It does not clone or
/// reconstruct the program representation.
///
/// The backend identifier may be either a canonical identifier or an
/// alias registered with the canonical backend registry.
///
/// Compatibility and feasibility must already have been established by
/// the appropriate compiler planning and backend compatibility systems.
///
/// This request does not independently establish target suitability.
pub struct BackendInvocationRequest<'a> {
    backend_identifier: String,
    module: &'a IrModule,
}

impl<'a> BackendInvocationRequest<'a> {
    /// Construct a backend invocation request.
    ///
    /// # Arguments
    ///
    /// - `backend_identifier`: canonical backend identifier or registered
    ///   alias.
    /// - `module`: canonical IR module to pass to the backend.
    ///
    /// # Errors
    ///
    /// Returns an error if the backend identifier is empty, contains
    /// surrounding whitespace, or contains control characters.
    pub fn new(
        backend_identifier: impl Into<String>,
        module: &'a IrModule,
    ) -> Result<Self, BackendInvocationError> {
        let backend_identifier = backend_identifier.into();

        validate_backend_identifier(&backend_identifier)?;

        Ok(Self {
            backend_identifier,
            module,
        })
    }

    /// Return the requested backend identifier.
    ///
    /// This may be an alias. The canonical identifier is available in
    /// the invocation result.
    #[must_use]
    pub fn backend_identifier(&self) -> &str {
        &self.backend_identifier
    }

    /// Return the borrowed IR module.
    #[must_use]
    pub fn module(&self) -> &'a IrModule {
        self.module
    }
}

// ============================================================================
// OUTPUT CLASSIFICATION
// ============================================================================

/// Verification state of output returned by the existing backend API.
///
/// The current backend interface returns a string and does not provide
/// a universal artifact-verification contract.
///
/// Therefore, successful invocation is classified as unverified output.
///
/// Future artifact-validation integration may produce a separate
/// verified-artifact result. It must not silently reinterpret this
/// status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendOutputStatus {
    /// The backend returned output successfully, but this module has not
    /// established that the output is a valid executable artifact.
    GeneratedUnverified,
}

// ============================================================================
// INVOCATION RESULT
// ============================================================================

/// Successful result of invoking a registered backend.
///
/// The result retains the canonical backend descriptor and the output
/// returned by the existing backend interface.
///
/// `generated_output` is deliberately represented as text because that
/// is the existing backend API contract.
///
/// It must not be treated as a binary artifact, executable, bitstream,
/// quantum package, or deployable bundle without additional validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendInvocationResult {
    descriptor: BackendDescriptor,
    generated_output: String,
    status: BackendOutputStatus,
}

impl BackendInvocationResult {
    /// Construct a result after successful backend generation.
    fn new(
        descriptor: BackendDescriptor,
        generated_output: String,
    ) -> Self {
        Self {
            descriptor,
            generated_output,
            status: BackendOutputStatus::GeneratedUnverified,
        }
    }

    /// Return the canonical registered backend descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &BackendDescriptor {
        &self.descriptor
    }

    /// Return the canonical backend identifier.
    #[must_use]
    pub fn backend_id(&self) -> &str {
        &self.descriptor.id
    }

    /// Return the backend's declared output extension.
    ///
    /// This is descriptive registry metadata. It does not prove that
    /// the returned text is valid for that extension.
    #[must_use]
    pub fn file_extension(&self) -> &str {
        &self.descriptor.file_extension
    }

    /// Return the complete generated output.
    ///
    /// No truncation or arbitrary output-size ceiling is imposed.
    #[must_use]
    pub fn generated_output(&self) -> &str {
        &self.generated_output
    }

    /// Consume the result and return its generated output.
    #[must_use]
    pub fn into_generated_output(self) -> String {
        self.generated_output
    }

    /// Return the output verification status.
    #[must_use]
    pub const fn status(&self) -> BackendOutputStatus {
        self.status
    }

    /// Whether this result represents verified executable output.
    ///
    /// The current implementation always returns `false` because the
    /// legacy backend API does not provide artifact verification evidence.
    #[must_use]
    pub const fn is_verified_artifact(&self) -> bool {
        false
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced at the compiler/backend invocation boundary.
#[derive(Debug)]
#[non_exhaustive]
pub enum BackendInvocationError {
    /// The requested backend identifier is malformed.
    InvalidBackendIdentifier {
        /// Identifier supplied by the caller.
        identifier: String,

        /// Explanation of the validation failure.
        reason: String,
    },

    /// The canonical backend registry failed.
    Registry(RegistryError),

    /// The backend implementation reported an unexpected extension.
    ///
    /// The descriptor and implementation must agree before generation
    /// is accepted.
    ImplementationExtensionMismatch {
        /// Canonical backend identifier.
        backend_id: String,

        /// Extension registered in backend metadata.
        registered_extension: String,

        /// Extension reported by the constructed implementation.
        implementation_extension: String,
    },

    /// The backend failed while generating output.
    CodeGenerationFailed {
        /// Canonical backend identifier.
        backend_id: String,

        /// Target identifier reported by the backend error.
        target: String,

        /// Backend-provided error message.
        message: String,
    },
}

impl fmt::Display for BackendInvocationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidBackendIdentifier {
                identifier,
                reason,
            } => {
                write!(
                    formatter,
                    "invalid backend identifier `{identifier}`: {reason}"
                )
            }

            Self::Registry(error) => {
                write!(
                    formatter,
                    "backend invocation registry failure: {error}"
                )
            }

            Self::ImplementationExtensionMismatch {
                backend_id,
                registered_extension,
                implementation_extension,
            } => {
                write!(
                    formatter,
                    "backend `{backend_id}` declares extension \
                     `{registered_extension}` in the registry but its \
                     implementation reports `{implementation_extension}`"
                )
            }

            Self::CodeGenerationFailed {
                backend_id,
                target,
                message,
            } => {
                write!(
                    formatter,
                    "code generation failed for backend `{backend_id}` \
                     and target `{target}`: {message}"
                )
            }
        }
    }
}

impl Error for BackendInvocationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Registry(error) => Some(error),
            Self::InvalidBackendIdentifier { .. }
            | Self::ImplementationExtensionMismatch { .. }
            | Self::CodeGenerationFailed { .. } => None,
        }
    }
}

impl From<RegistryError> for BackendInvocationError {
    fn from(error: RegistryError) -> Self {
        Self::Registry(error)
    }
}

// ============================================================================
// INVOCATION SERVICE
// ============================================================================

/// Stateless backend invocation coordinator.
///
/// The service borrows the canonical backend registry. It does not
/// maintain its own backend inventory.
///
/// The canonical registry is responsible for:
///
/// - canonical identifier resolution;
/// - alias resolution;
/// - backend factory invocation;
/// - backend factory identity verification.
///
/// This service additionally checks that the constructed implementation's
/// output extension agrees with its registered descriptor.
///
/// It then invokes the existing backend generation interface.
pub struct BackendInvocationService<'a> {
    registry: &'a BackendRegistry,
}

impl<'a> BackendInvocationService<'a> {
    /// Construct an invocation service over the canonical registry.
    #[must_use]
    pub fn new(registry: &'a BackendRegistry) -> Self {
        Self { registry }
    }

    /// Return the stable module identity.
    #[must_use]
    pub const fn module_name() -> &'static str {
        BACKEND_INVOCATION_MODULE_NAME
    }

    /// Return the invocation API version.
    #[must_use]
    pub const fn api_version() -> u16 {
        BACKEND_INVOCATION_API_VERSION
    }

    /// Invoke a registered backend with the supplied IR module.
    ///
    /// The method performs the following steps:
    ///
    /// 1. Validate the requested backend identifier.
    /// 2. Resolve the canonical backend descriptor.
    /// 3. Construct the backend through the canonical registry.
    /// 4. Verify implementation and registry extension agreement.
    /// 5. Invoke the backend's existing `generate` method.
    /// 6. Return the generated text with an explicit unverified status.
    ///
    /// The backend factory is invoked by the canonical registry outside
    /// its internal registry lock.
    ///
    /// This method does not:
    ///
    /// - perform capability matching;
    /// - perform hardware allocation;
    /// - run an external compiler or linker;
    /// - write files;
    /// - validate executable formats;
    /// - execute generated output;
    /// - silently fall back to another backend.
    ///
    /// Callers must perform compatibility and feasibility evaluation
    /// before invoking a backend.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - the identifier is malformed;
    /// - the backend is not registered;
    /// - registry access fails;
    /// - backend construction fails its canonical identity check;
    /// - implementation and registry extensions disagree;
    /// - backend code generation fails.
    pub fn invoke(
        &self,
        request: &BackendInvocationRequest<'_>,
    ) -> Result<BackendInvocationResult, BackendInvocationError> {
        validate_backend_identifier(
            request.backend_identifier(),
        )?;

        // Resolve aliases before retaining the descriptor. This ensures
        // the result always reports the canonical backend identity.
        let descriptor = self
            .registry
            .descriptor(request.backend_identifier())?;

        // The canonical registry verifies that the constructed backend
        // reports the registered canonical target identifier.
        let implementation = self
            .registry
            .create(request.backend_identifier())?;

        // The backend implementation and its registered metadata must
        // agree before the generated output is accepted.
        let implementation_extension =
            implementation.file_extension().to_owned();

        if implementation_extension != descriptor.file_extension {
            return Err(
                BackendInvocationError::ImplementationExtensionMismatch {
                    backend_id: descriptor.id.clone(),
                    registered_extension:
                        descriptor.file_extension.clone(),
                    implementation_extension,
                },
            );
        }

        // Invoke the canonical backend implementation.
        //
        // The existing API returns a String. This module preserves that
        // contract and does not claim the output is a verified artifact.
        let generated_output = implementation
            .generate(request.module())
            .map_err(|error| {
                BackendInvocationError::CodeGenerationFailed {
                    backend_id: descriptor.id.clone(),
                    target: error.target.clone(),
                    message: error.message.clone(),
                }
            })?;

        Ok(BackendInvocationResult::new(
            descriptor,
            generated_output,
        ))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            BackendInvocationService::module_name(),
            BACKEND_INVOCATION_MODULE_NAME,
        );

        assert_ne!(
            BackendInvocationService::api_version(),
            0,
        );
    }

    #[test]
    fn empty_backend_identifier_is_rejected() {
        let result = validate_backend_identifier("");

        assert!(matches!(
            result,
            Err(BackendInvocationError::InvalidBackendIdentifier { .. })
        ));
    }

    #[test]
    fn whitespace_only_backend_identifier_is_rejected() {
        let result = validate_backend_identifier("   ");

        assert!(matches!(
            result,
            Err(BackendInvocationError::InvalidBackendIdentifier { .. })
        ));
    }

    #[test]
    fn surrounding_whitespace_is_rejected() {
        let result = validate_backend_identifier(" backend.cpu ");

        assert!(matches!(
            result,
            Err(BackendInvocationError::InvalidBackendIdentifier { .. })
        ));
    }

    #[test]
    fn control_characters_are_rejected() {
        let result = validate_backend_identifier("backend.\nexample");

        assert!(matches!(
            result,
            Err(BackendInvocationError::InvalidBackendIdentifier { .. })
        ));
    }

    #[test]
    fn valid_backend_identifier_is_accepted() {
        assert!(
            validate_backend_identifier("vendor.future-architecture")
                .is_ok()
        );
    }

    #[test]
    fn identifier_validation_has_no_arbitrary_length_ceiling() {
        let identifier = format!(
            "backend.{}",
            "x".repeat(16_384),
        );

        assert!(validate_backend_identifier(&identifier).is_ok());
    }

    #[test]
    fn output_status_never_claims_verification() {
        let status = BackendOutputStatus::GeneratedUnverified;

        assert_eq!(
            status,
            BackendOutputStatus::GeneratedUnverified,
        );
    }

    #[test]
    fn registry_errors_are_preserved_as_error_sources() {
        let error = BackendInvocationError::Registry(
            RegistryError::BackendNotFound {
                identifier: "backend.missing".to_owned(),
            },
        );

        assert!(error.source().is_some());
    }

    #[test]
    fn implementation_extension_mismatch_is_explicit() {
        let error =
            BackendInvocationError::ImplementationExtensionMismatch {
                backend_id: "backend.example".to_owned(),
                registered_extension: ".bin".to_owned(),
                implementation_extension: ".other".to_owned(),
            };

        assert!(
            error
                .to_string()
                .contains("backend.example")
        );

        assert!(
            error
                .to_string()
                .contains(".other")
        );
    }

    #[test]
    fn code_generation_failure_preserves_backend_context() {
        let error = BackendInvocationError::CodeGenerationFailed {
            backend_id: "backend.example".to_owned(),
            target: "target.example".to_owned(),
            message: "generation failed".to_owned(),
        };

        let rendered = error.to_string();

        assert!(rendered.contains("backend.example"));
        assert!(rendered.contains("target.example"));
        assert!(rendered.contains("generation failed"));
    }
}
