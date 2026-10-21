
#![forbid(unsafe_code)]

//! Zamani backend linker integration.
//!
//! This module is the stable backend-facing adapter for the authoritative
//! compiler linker in `crate::compiler::linker`.
//!
//! # Ownership
//!
//! This module owns:
//! - the backend-to-linker integration boundary;
//! - typed adapter errors;
//! - construction and validation of backend linking requests;
//! - explicit portable, WebAssembly, and native-object linking entry points;
//! - access to link results without changing their IR representation.
//!
//! This module does not own:
//! - symbol resolution or duplicate-symbol policy;
//! - IR composition or IR verification algorithms;
//! - ABI selection or target-specific lowering;
//! - native linker command construction;
//! - quantum routing, QEC, HDL synthesis, or runtime execution;
//! - artifact serialization, filesystem publication, or cache management.
//!
//! Those responsibilities belong to the existing compiler linker and the
//! appropriate compiler, domain, toolchain, and artifact subsystems.
//!
//! # Integration
//!
//! `crate::compiler::linker` remains the sole owner of linker behavior.
//! This adapter deliberately delegates to it instead of maintaining a
//! second implementation of symbol resolution.
//!
//! Ordinary IR linking is in-memory and does not intentionally publish files
//! or execute generated programs. Native artifact linking is an explicit
//! operation delegated to the compiler linker.
//!
//! # Scalability
//!
//! No universal hardware-capacity limits are imposed here. Memory and
//! execution costs depend on the input IR, the delegated implementation,
//! the selected target, and the resources available to the process.
//!
//! Rust allocation failure and operating-system resource exhaustion remain
//! possible and are not represented as infinite-capacity guarantees.
//!
//! # Compatibility
//!
//! The adapter consumes the repository's current `crate::ir_gen::IrModule`.
//! It does not claim that this legacy IR is already the final canonical
//! universal IR for every computational domain.

use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

use crate::ir_gen::IrModule;

pub use crate::compiler::linker::{
    LinkResult, LinkTarget, LinkerConfig, LinkerError, LinkingMode, LtoLevel,
    NativeLinkerConfig, ReproducibilityConfig, WasmArchitecture,
};

use crate::compiler::linker::ZamaniLinker;

/// Backend-facing linker adapter.
///
/// The underlying compiler linker remains the authoritative implementation
/// of module composition, symbol validation, and native linking.
///
/// The inner linker is intentionally private so callers use this adapter's
/// validated methods rather than modifying its configuration accidentally.
#[derive(Debug, Clone)]
pub struct BackendLinker {
    inner: ZamaniLinker,
}

/// Errors produced at the backend-linker integration boundary.
#[derive(Debug)]
pub enum BackendLinkError {
    /// The delegated compiler linker rejected the request.
    Linker(LinkerError),

    /// A backend request violated an adapter-level precondition.
    InvalidRequest {
        field: &'static str,
        reason: String,
    },
}

impl fmt::Display for BackendLinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Linker(error) => write!(f, "{error}"),
            Self::InvalidRequest { field, reason } => {
                write!(f, "invalid backend link request for {field}: {reason}")
            }
        }
    }
}

impl Error for BackendLinkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Linker(error) => Some(error),
            Self::InvalidRequest { .. } => None,
        }
    }
}

impl From<LinkerError> for BackendLinkError {
    fn from(error: LinkerError) -> Self {
        Self::Linker(error)
    }
}

impl BackendLinker {
    /// Create a linker using the compiler linker's portable defaults.
    ///
    /// An empty module list is permitted at construction time so the caller
    /// can assemble a request incrementally. Linking an empty list returns
    /// the compiler linker's typed `EmptyInput` error.
    #[must_use]
    pub fn new(modules: Vec<IrModule>) -> Self {
        Self {
            inner: ZamaniLinker::new(modules),
        }
    }

    /// Create a linker for an explicitly selected target.
    ///
    /// Target-specific feasibility is not inferred from the target name.
    /// The compiler linker validates its own target and mode invariants.
    #[must_use]
    pub fn with_target(modules: Vec<IrModule>, target: LinkTarget) -> Self {
        Self {
            inner: ZamaniLinker::with_target(modules, target),
        }
    }

    /// Create a linker from an explicit configuration.
    ///
    /// Configuration validation is delegated to the authoritative compiler
    /// linker. Invalid configurations are returned as typed errors.
    pub fn with_config(
        modules: Vec<IrModule>,
        config: LinkerConfig,
    ) -> Result<Self, BackendLinkError> {
        let inner = ZamaniLinker::with_config(modules, config)?;
        Ok(Self { inner })
    }

    /// Return the number of input modules.
    #[must_use]
    pub fn module_count(&self) -> usize {
        self.inner.module_count()
    }

    /// Return whether the configured target is portable.
    #[must_use]
    pub fn is_portable_target(&self) -> bool {
        self.inner.is_portable_target()
    }

    /// Return the configured target.
    #[must_use]
    pub fn target(&self) -> &LinkTarget {
        &self.inner.config.target
    }

    /// Return the current linking configuration.
    ///
    /// This is a read-only view. Change configuration through the validated
    /// methods exposed by this adapter.
    #[must_use]
    pub fn config(&self) -> &LinkerConfig {
        &self.inner.config
    }

    /// Replace the target while preserving the compiler linker's validation.
    ///
    /// A failed update leaves the existing configuration unchanged.
    pub fn set_target(
        &mut self,
        target: LinkTarget,
    ) -> Result<(), BackendLinkError> {
        self.inner.set_target(target)?;
        Ok(())
    }

    /// Select the compiler linker's link-time normalization level.
    ///
    /// This delegates to the existing implementation. It does not promise
    /// that every LTO level performs a distinct optimization algorithm.
    pub fn set_lto(&mut self, level: LtoLevel) {
        self.inner.set_lto(level);
    }

    /// Compose input modules and return the resulting IR.
    ///
    /// This operation does not invoke a native linker or write an output
    /// artifact. The compiler linker remains responsible for duplicate
    /// declaration checks and module composition.
    pub fn link(&self) -> Result<IrModule, BackendLinkError> {
        Ok(self.inner.link()?)
    }

    /// Compose input modules and return result metadata.
    ///
    /// Metadata includes the selected target and the counts calculated by
    /// the authoritative compiler linker.
    pub fn link_with_result(&self) -> Result<LinkResult, BackendLinkError> {
        Ok(self.inner.link_with_result()?)
    }

    /// Link modules using the portable Zamani IR target.
    ///
    /// This is IR composition, not a promise that the resulting module can
    /// execute without a compatible runtime or further target lowering.
    pub fn link_portable(&self) -> Result<IrModule, BackendLinkError> {
        Ok(self.inner.link_portable()?)
    }

    /// Link modules for a WebAssembly architecture.
    ///
    /// WebAssembly binary generation and validation remain the responsibility
    /// of the WebAssembly backend and its configured toolchain.
    pub fn link_wasm(
        &self,
        architecture: WasmArchitecture,
    ) -> Result<IrModule, BackendLinkError> {
        Ok(self.inner.link_wasm(architecture)?)
    }

    /// Prepare linked IR for a specified native target triple.
    ///
    /// This operation does not itself emit native machine code. The native
    /// backend must subsequently lower the IR for the selected architecture,
    /// operating system, ABI, and target features.
    pub fn link_native(
        &self,
        target_triple: impl Into<String>,
    ) -> Result<IrModule, BackendLinkError> {
        let triple = target_triple.into();

        if triple.trim().is_empty() {
            return Err(BackendLinkError::InvalidRequest {
                field: "target_triple",
                reason: "the target triple must not be empty".to_owned(),
            });
        }

        Ok(self.inner.link_native(triple)?)
    }

    /// Invoke the configured native linker for already-generated object files.
    ///
    /// This is an explicit external-process operation. It is not performed
    /// by `link`, `link_portable`, `link_wasm`, or `link_native`.
    ///
    /// The delegated linker passes arguments without shell interpolation.
    /// Callers must still treat the executable, linker arguments, object
    /// files, library paths, and output location as security-sensitive inputs.
    ///
    /// The output is not considered production-validated merely because the
    /// external process returned success. A higher-level artifact-validation
    /// stage must verify the resulting format and target compatibility.
    pub fn link_native_artifact(
        &self,
        object_files: &[PathBuf],
        output_path: impl AsRef<Path>,
    ) -> Result<(), BackendLinkError> {
        if object_files.is_empty() {
            return Err(BackendLinkError::InvalidRequest {
                field: "object_files",
                reason: "at least one object file is required".to_owned(),
            });
        }

        let output_path = output_path.as_ref();

        if output_path.as_os_str().is_empty() {
            return Err(BackendLinkError::InvalidRequest {
                field: "output_path",
                reason: "the output path must not be empty".to_owned(),
            });
        }

        Ok(self
            .inner
            .link_native_artifact(object_files, output_path)?)
    }
}

/// Compose modules for a target without exposing linker configuration details.
///
/// This convenience function is useful for compiler orchestration code that
/// only needs the linked IR and does not need to retain a linker instance.
pub fn link_modules(
    modules: Vec<IrModule>,
    target: LinkTarget,
) -> Result<LinkResult, BackendLinkError> {
    BackendLinker::with_target(modules, target).link_with_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portable_target_is_selected_by_default() {
        let linker = BackendLinker::new(Vec::new());

        assert!(linker.is_portable_target());
        assert_eq!(linker.target(), &LinkTarget::Portable);
    }

    #[test]
    fn empty_input_returns_a_typed_error() {
        let linker = BackendLinker::new(Vec::new());

        assert!(matches!(
            linker.link(),
            Err(BackendLinkError::Linker(LinkerError::EmptyInput))
        ));
    }

    #[test]
    fn blank_native_triple_is_rejected_before_linking() {
        let linker = BackendLinker::new(Vec::new());

        assert!(matches!(
            linker.link_native("   "),
            Err(BackendLinkError::InvalidRequest {
                field: "target_triple",
                ..
            })
        ));
    }

    #[test]
    fn empty_native_object_list_is_rejected() {
        let linker = BackendLinker::new(Vec::new());

        assert!(matches!(
            linker.link_native_artifact(&[], Path::new("output")),
            Err(BackendLinkError::InvalidRequest {
                field: "object_files",
                ..
            })
        ));
    }

    #[test]
    fn target_change_is_validated_by_the_authoritative_linker() {
        let mut linker = BackendLinker::new(Vec::new());

        assert!(linker
            .set_target(LinkTarget::Native {
                triple: "aarch64-unknown-linux-gnu".to_owned(),
            })
            .is_ok());

        assert_eq!(
            linker.target(),
            &LinkTarget::Native {
                triple: "aarch64-unknown-linux-gnu".to_owned(),
            }
        );
    }

    #[test]
    fn portable_linking_does_not_require_a_native_linker_process() {
        let mut module = IrModule::new("portable_test");
        module.target_triple = None;
        module.data_layout = None;

        let linker = BackendLinker::new(vec![module]);
        let result = linker.link_with_result();

        assert!(result.is_ok());

        let result = result.expect("linking a single module should succeed");
        assert_eq!(result.input_modules, 1);
        assert!(matches!(result.target, LinkTarget::Portable));
    }
}
