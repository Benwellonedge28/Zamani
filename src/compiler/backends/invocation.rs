#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Backend Invocation
//!
//! Path: src/compiler/backends/invocation.rs
//! Edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Responsibility
//!
//! This module coordinates invocation of an explicitly selected backend
//! through the canonical backend registry.
//!
//! It owns:
//!
//! - invocation request validation;
//! - backend identifier validation;
//! - canonical backend resolution;
//! - consistent descriptor/implementation acquisition;
//! - implementation metadata verification;
//! - backend generation invocation;
//! - structured error propagation;
//! - generated-output classification.
//!
//! It does not own:
//!
//! - backend registration;
//! - backend discovery;
//! - target selection;
//! - semantic requirement evaluation;
//! - capability matching;
//! - physical resource allocation;
//! - IR construction or transformation;
//! - backend implementation algorithms;
//! - artifact verification algorithms;
//! - filesystem operations;
//! - linking;
//! - deployment;
//! - runtime execution.
//!
//! # Architectural authorities
//!
//! Backend interface:
//!     crate::backend::Backend
//!
//! Backend registration:
//!     crate::backend::registry::BackendRegistry
//!
//! Backend metadata:
//!     crate::backend::registry::BackendDescriptor
//!
//! Existing compiler IR input:
//!     crate::ir_gen::IrModule
//!
//! Backend compilation pipeline:
//!     crate::backend::pipeline
//!
//! Artifact verification:
//!     crate::backend::pipeline
//!
//! # POCO-REAF
//!
//! The selected backend must preserve the semantics of the supplied
//! intermediate representation.
//!
//! Invocation never silently substitutes another backend.
//!
//! Backend registration does not establish compatibility.
//!
//! Compatibility does not establish physical availability.
//!
//! Successful generation does not establish artifact validity.
//!
//! # Scalability
//!
//! This module imposes no universal limits on:
//!
//! - backend registrations;
//! - target identifiers;
//! - computational domains;
//! - IR module sizes;
//! - generated output sizes;
//! - resource counts;
//! - hardware capacities;
//! - future backend categories.
//!
//! Actual operational limits belong to the configured compilation
//! pipeline, backend implementation, operating environment and
//! available resources.
//!
//! # Security
//!
//! Backend factories and implementations are extension boundaries.
//!
//! Registry identity checks are necessary but are not proof that a
//! backend is trustworthy.
//!
//! Untrusted implementations must be isolated by the appropriate
//! extension/process security subsystem.
//!
//! This module does not claim to sandbox backend code.
//!
//! # Output trust
//!
//! The current Backend trait returns String output.
//!
//! Generated text is not automatically:
//!
//! - executable;
//! - verified;
//! - portable;
//! - linked;
//! - deployable;
//! - a valid FPGA bitstream;
//! - a valid quantum package.
//!
//! The result therefore retains an explicit unverified status.

use std::error::Error;
use std::fmt;

use crate::backend::{
    Backend,
    CodeGenError,
};

use crate::backend::registry::{
    BackendDescriptor,
    BackendRegistry,
    RegistryError,
};

use crate::ir_gen::IrModule;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable module identity.
pub const BACKEND_INVOCATION_MODULE_NAME: &str =
    "zamani.compiler.backends.invocation";

/// Invocation API version.
///
/// Independent of the Zamani language version, IR version and backend
/// implementation version.
pub const BACKEND_INVOCATION_API_VERSION: u16 = 2;

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

/// Validate a backend identifier at the invocation boundary.
///
/// The canonical registry remains authoritative for identifier syntax,
/// uniqueness, aliases and canonical identity.
///
/// This function rejects malformed invocation input without imposing
/// an arbitrary identifier-length ceiling.
fn validate_backend_identifier(
    identifier: &str,
) -> Result<(), BackendInvocationError> {
    if identifier.is_empty() {
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
                reason: "surrounding whitespace is not permitted"
                    .to_owned(),
            },
        );
    }

    if identifier.chars().any(char::is_control) {
        return Err(
            BackendInvocationError::InvalidBackendIdentifier {
                identifier: identifier.to_owned(),
                reason: "control characters are not permitted"
                    .to_owned(),
            },
        );
    }

    Ok(())
}

// ============================================================================
// REQUEST
// ============================================================================

/// Immutable backend invocation request.
///
/// The IR module is borrowed. The invocation boundary does not clone,
/// reconstruct or reinterpret it.
///
/// The identifier may be a canonical registered identifier or an
/// explicitly registered alias.
///
/// Compatibility and feasibility must have been established by the
/// preceding compiler planning and backend-selection stages.
pub struct BackendInvocationRequest<'a> {
    backend_identifier: String,
    module: &'a IrModule,
}

impl<'a> BackendInvocationRequest<'a> {
    /// Construct a validated invocation request.
    ///
    /// # Errors
    ///
    /// Returns an error if the backend identifier is empty, contains
    /// surrounding whitespace or contains control characters.
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

    /// Return the requested identifier.
    ///
    /// It may be a registered alias.
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
// OUTPUT STATUS
// ============================================================================

/// Trust and verification classification of generated output.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendOutputStatus {
    /// Generation succeeded, but artifact verification has not been
    /// established by the invocation boundary.
    GeneratedUnverified,
}

// ============================================================================
// RESULT
// ============================================================================

/// Successful backend invocation result.
///
/// The output remains text because that is the existing Backend trait
/// contract.
///
/// This type does not pretend that text output is a verified binary
/// artifact or a deployable target package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendInvocationResult {
    descriptor: BackendDescriptor,
    generated_output: String,
    status: BackendOutputStatus,
}

impl BackendInvocationResult {
    /// Create a result from a successful generation operation.
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

    /// Return the advertised output extension.
    ///
    /// This is registry metadata, not proof of output validity.
    #[must_use]
    pub fn file_extension(&self) -> &str {
        &self.descriptor.file_extension
    }

    /// Return the complete generated output.
    #[must_use]
    pub fn generated_output(&self) -> &str {
        &self.generated_output
    }

    /// Consume the result and return generated output.
    #[must_use]
    pub fn into_generated_output(self) -> String {
        self.generated_output
    }

    /// Return the output status.
    #[must_use]
    pub const fn status(&self) -> BackendOutputStatus {
        self.status
    }

    /// Return whether this result is a verified executable artifact.
    ///
    /// This invocation API does not perform artifact verification.
    #[must_use]
    pub const fn is_verified_artifact(&self) -> bool {
        false
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Structured backend invocation failures.
#[derive(Debug)]
#[non_exhaustive]
pub enum BackendInvocationError {
    /// The supplied backend identifier is malformed.
    InvalidBackendIdentifier {
        /// Original identifier.
        identifier: String,

        /// Validation explanation.
        reason: String,
    },

    /// Canonical registry operation failed.
    Registry(RegistryError),

    /// The backend implementation's extension differs from its
    /// registered descriptor.
    ImplementationExtensionMismatch {
        /// Canonical backend identifier.
        backend_id: String,

        /// Extension recorded in the registry.
        registered_extension: String,

        /// Extension reported by the implementation.
        implementation_extension: String,
    },

    /// The backend failed during code generation.
    ///
    /// The original error is preserved rather than flattened into a
    /// string-only representation.
    CodeGenerationFailed {
        /// Canonical backend identifier.
        backend_id: String,

        /// Original backend error.
        source: CodeGenError,
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
            } => write!(
                formatter,
                "invalid backend identifier `{identifier}`: {reason}"
            ),

            Self::Registry(error) => write!(
                formatter,
                "backend registry operation failed: {error}"
            ),

            Self::ImplementationExtensionMismatch {
                backend_id,
                registered_extension,
                implementation_extension,
            } => write!(
                formatter,
                "backend `{backend_id}` reports extension \
                 `{implementation_extension}`, but its registered \
                 descriptor declares `{registered_extension}`"
            ),

            Self::CodeGenerationFailed {
                backend_id,
                source,
            } => write!(
                formatter,
                "code generation failed for backend `{backend_id}`: \
                 {source}"
            ),
        }
    }
}

impl Error for BackendInvocationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Registry(error) => Some(error),

            Self::CodeGenerationFailed { source, .. } => Some(source),

            Self::InvalidBackendIdentifier { .. }
            | Self::ImplementationExtensionMismatch { .. } => None,
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

/// Stateless invocation coordinator over the canonical backend registry.
///
/// It owns no backend registrations and no backend factories.
pub struct BackendInvocationService<'a> {
    registry: &'a BackendRegistry,
}

impl<'a> BackendInvocationService<'a> {
    /// Construct an invocation service over the canonical registry.
    #[must_use]
    pub fn new(registry: &'a BackendRegistry) -> Self {
        Self {
            registry,
        }
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

    /// Invoke the explicitly requested registered backend.
    ///
    /// The operation:
    ///
    /// 1. Validates the identifier.
    /// 2. Obtains a consistent descriptor/factory snapshot.
    /// 3. Constructs the registered backend.
    /// 4. Checks implementation/descriptor extension agreement.
    /// 5. Invokes the existing Backend generation interface.
    /// 6. Returns output classified as unverified.
    ///
    /// The canonical registry executes the backend factory outside its
    /// internal lock.
    ///
    /// No fallback is attempted.
    ///
    /// No target is inferred from the host machine.
    ///
    /// No physical resource is allocated.
    ///
    /// No generated output is executed.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid identifiers, registry failures,
    /// implementation metadata inconsistencies or generation failures.
    pub fn invoke(
        &self,
        request: &BackendInvocationRequest<'_>,
    ) -> Result<BackendInvocationResult, BackendInvocationError> {
        validate_backend_identifier(
            request.backend_identifier(),
        )?;

        // The registry must provide descriptor and implementation
        // from the same registration snapshot.
        let (descriptor, implementation) = self
            .registry
            .create_with_descriptor(
                request.backend_identifier(),
            )?;

        // Do not trust an implementation's advertised output extension
        // when it disagrees with the registered descriptor.
        let implementation_extension =
            implementation.file_extension();

        if implementation_extension != descriptor.file_extension {
            return Err(
                BackendInvocationError::ImplementationExtensionMismatch {
                    backend_id: descriptor.id.clone(),
                    registered_extension: descriptor.file_extension.clone(),
                    implementation_extension:
                        implementation_extension.to_owned(),
                },
            );
        }

        // Invoke the exact selected backend.
        //
        // The existing Backend trait accepts the legacy IR module and
        // returns generated text. This adapter does not claim that the
        // returned text has passed artifact verification.
        let generated_output = implementation
            .generate(request.module())
            .map_err(|source| {
                BackendInvocationError::CodeGenerationFailed {
                    backend_id: descriptor.id.clone(),
                    source,
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
    }

    #[test]
    fn api_version_is_nonzero() {
        assert_ne!(
            BackendInvocationService::api_version(),
            0,
        );
    }

    #[test]
    fn empty_identifier_is_rejected() {
        assert!(matches!(
            validate_backend_identifier(""),
            Err(
                BackendInvocationError::InvalidBackendIdentifier {
                    ..
                }
            )
        ));
    }

    #[test]
    fn whitespace_only_identifier_is_rejected() {
        assert!(matches!(
            validate_backend_identifier("   "),
            Err(
                BackendInvocationError::InvalidBackendIdentifier {
                    ..
                }
            )
        ));
    }

    #[test]
    fn surrounding_whitespace_is_rejected() {
        assert!(matches!(
            validate_backend_identifier(" backend.cpu "),
            Err(
                BackendInvocationError::InvalidBackendIdentifier {
                    ..
                }
            )
        ));
    }

    #[test]
    fn control_characters_are_rejected() {
        assert!(matches!(
            validate_backend_identifier("backend.\nexample"),
            Err(
                BackendInvocationError::InvalidBackendIdentifier {
                    ..
                }
            )
        ));
    }

    #[test]
    fn open_world_identifiers_are_accepted() {
        assert!(
            validate_backend_identifier(
                "future.vendor:heterogeneous-architecture"
            )
            .is_ok()
        );
    }

    #[test]
    fn identifier_validation_has_no_arbitrary_length_limit() {
        let identifier = format!(
            "backend.{}",
            "x".repeat(16_384),
        );

        assert!(
            validate_backend_identifier(&identifier).is_ok()
        );
    }

    #[test]
    fn generated_output_is_not_misrepresented_as_verified() {
        let status = BackendOutputStatus::GeneratedUnverified;

        assert_eq!(
            status,
            BackendOutputStatus::GeneratedUnverified,
        );
    }

    #[test]
    fn registry_errors_preserve_their_source() {
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

        let rendered = error.to_string();

        assert!(rendered.contains("backend.example"));
        assert!(rendered.contains(".bin"));
        assert!(rendered.contains(".other"));
    }

    #[test]
    fn code_generation_error_is_preserved() {
        let source = CodeGenError::new(
            "generation failed",
            "target.example",
        );

        let error = BackendInvocationError::CodeGenerationFailed {
            backend_id: "backend.example".to_owned(),
            source,
        };

        assert!(error.source().is_some());

        let rendered = error.to_string();

        assert!(rendered.contains("backend.example"));
        assert!(rendered.contains("generation failed"));
        assert!(rendered.contains("target.example"));
    }
}