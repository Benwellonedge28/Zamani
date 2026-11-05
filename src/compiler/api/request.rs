
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Compilation Requests.
//!
//! Path: src/compiler/api/request.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Defines the owned, immutable input contract for one compilation request.
//!
//! A request contains:
//!
//! - source text or a source-file path;
//! - a logical source name when source text is supplied;
//! - the existing authoritative CompilerConfig.
//!
//! This module does not perform compilation.
//!
//! It does not parse source code, construct an AST, perform semantic
//! analysis, generate IR, optimize programs, select physical hardware,
//! invoke backends, link artifacts or execute programs.
//!
//! Those responsibilities remain with their existing repository owners.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - compilation-request representation;
//! - source-input representation;
//! - request-level configuration association;
//! - immutable request access;
//! - inexpensive request cloning;
//! - source identity at the API boundary.
//!
//! This module does not own:
//!
//! - compiler configuration definitions;
//! - target definitions;
//! - optimization-level definitions;
//! - language semantics;
//! - AST or IR definitions;
//! - resource allocation;
//! - capability discovery;
//! - backend selection;
//! - compilation execution;
//! - runtime execution.
//!
//! # POCO-REAF
//!
//! The request is independent of:
//!
//! - CPU architecture;
//! - processor count;
//! - GPU vendor;
//! - accelerator count;
//! - FPGA architecture;
//! - ASIC implementation;
//! - quantum processor provider;
//! - distributed topology;
//! - operating system;
//! - deployment environment.
//!
//! A request does not imply that the host machine is the compilation target.
//!
//! Target and optimization configuration are supplied explicitly.
//!
//! No arbitrary source-size, module-count, operation-count, hardware-count
//! or resource-capacity limits are introduced here.
//!
//! Actual operational limits belong to configurable compiler budgets,
//! operating-system constraints and the available implementation resources.
//!
//! # Source-input contract
//!
//! SourceInput supports:
//!
//! 1. In-memory UTF-8 source text.
//! 2. A filesystem source path.
//!
//! In-memory source is stored in Arc<str> so cloning a request does not
//! duplicate the entire source buffer.
//!
//! Filesystem paths are stored as PathBuf and are not read by this module.
//!
//! Reading, decoding and reporting filesystem errors belong to the
//! compiler-service input boundary.
//!
//! # Configuration contract
//!
//! CompilationRequest reuses:
//!
//!     crate::compiler_types::CompilerConfig
//!
//! It does not create another configuration system.
//!
//! Configuration is owned by the request so that the request remains
//! independent of later mutations to the caller's original configuration.
//!
//! CompilerConfig currently uses Clone rather than immutable shared fields.
//! The request therefore stores its own configuration value.
//!
//! # Integration
//!
//! Upstream:
//!
//! - CLI and build-system consumers;
//! - language-server and IDE consumers;
//! - embedded compiler consumers;
//! - tests and automation;
//! - existing CompilerService.
//!
//! Downstream:
//!
//! - api/compiler.rs;
//! - api/session.rs;
//! - future pipeline orchestration;
//! - diagnostics and source mapping.
//!
//! The compiler service must dispatch:
//!
//! SourceInput::Text -> CompilerService::compile_source
//! SourceInput::Path -> CompilerService::compile_path
//!
//! The request must not call the legacy compiler pipeline directly.
//!
//! # Compatibility
//!
//! Existing compile(), compile_with_config() and compile_source() functions
//! remain unchanged.
//!
//! This module is an additive API.
//!
//! # Error handling
//!
//! This module intentionally performs no filesystem or compilation work.
//!
//! Consequently, it does not invent a second compiler error type.
//!
//! Invalid source, invalid configuration, file I/O failures, semantic
//! failures and backend failures are reported by their authoritative
//! compiler-service and compiler-pipeline implementations.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::compiler_types::CompilerConfig;

/// Describes the source material supplied to a compilation request.
///
/// This type describes input location and ownership only.
///
/// It does not assign semantic meaning to source code.
///
/// In-memory text is shared through Arc<str>, allowing requests to be
/// cloned and transferred between threads without copying the complete
/// source buffer.
///
/// A filesystem path is retained as a PathBuf and read later by the
/// compiler-service boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceInput {
    /// Source supplied directly as UTF-8 text.
    ///
    /// `source` contains the complete source text.
    ///
    /// `source_name` is a logical name used for diagnostics and source
    /// identification. It does not identify a physical target.
    Text {
        source: Arc<str>,
        source_name: Arc<str>,
    },

    /// Source supplied through a filesystem path.
    ///
    /// The path is not opened or read while constructing the request.
    ///
    /// Filesystem access belongs to CompilerService.
    Path(PathBuf),
}

impl SourceInput {
    /// Creates an in-memory source input.
    ///
    /// The source is converted into Arc<str>.
    ///
    /// If the caller already has an Arc<str>, ownership is shared.
    ///
    /// No source-size limit is imposed.
    #[must_use]
    pub fn text(
        source: impl Into<Arc<str>>,
        source_name: impl Into<Arc<str>>,
    ) -> Self {
        Self::Text {
            source: source.into(),
            source_name: source_name.into(),
        }
    }

    /// Creates a filesystem-backed source input.
    ///
    /// This method does not access the filesystem.
    #[must_use]
    pub fn path(path: impl Into<PathBuf>) -> Self {
        Self::Path(path.into())
    }

    /// Returns the in-memory source text when available.
    ///
    /// Returns None for filesystem-backed inputs.
    #[must_use]
    pub fn source_text(&self) -> Option<&str> {
        match self {
            Self::Text { source, .. } => Some(source.as_ref()),
            Self::Path(_) => None,
        }
    }

    /// Returns the logical source name for in-memory source.
    ///
    /// Returns None for filesystem-backed inputs.
    #[must_use]
    pub fn source_name(&self) -> Option<&str> {
        match self {
            Self::Text { source_name, .. } => {
                Some(source_name.as_ref())
            }
            Self::Path(_) => None,
        }
    }

    /// Returns the filesystem path when the input is path-backed.
    ///
    /// Returns None for in-memory source.
    #[must_use]
    pub fn path_ref(&self) -> Option<&Path> {
        match self {
            Self::Text { .. } => None,
            Self::Path(path) => Some(path.as_path()),
        }
    }

    /// Indicates whether this input contains in-memory source text.
    #[must_use]
    pub const fn is_text(&self) -> bool {
        matches!(self, Self::Text { .. })
    }

    /// Indicates whether this input refers to a filesystem path.
    #[must_use]
    pub const fn is_path(&self) -> bool {
        matches!(self, Self::Path(_))
    }
}

/// Immutable description of one compilation request.
///
/// A request owns its configuration and source-input description.
///
/// Cloning a request:
///
/// - shares in-memory source text;
/// - clones the filesystem path when applicable;
/// - clones the existing compiler configuration.
///
/// A request can therefore be moved into a worker, retained by a session
/// or passed to a compiler service without borrowing the caller's buffers.
///
/// The request does not contain a physical device selection independent
/// of CompilerConfig.
///
/// It does not own a compilation session or pipeline.
#[derive(Debug, Clone)]
pub struct CompilationRequest {
    input: SourceInput,
    config: CompilerConfig,
}

impl CompilationRequest {
    /// Constructs a request from in-memory source text.
    ///
    /// `source_name` is the logical source name used for diagnostics.
    ///
    /// `config` is the authoritative existing compiler configuration.
    ///
    /// No implicit host-target selection is performed.
    ///
    /// Empty source text is preserved rather than rejected here.
    /// The compiler's parser remains responsible for reporting the
    /// appropriate source-language error.
    #[must_use]
    pub fn from_source(
        source: impl Into<Arc<str>>,
        source_name: impl Into<Arc<str>>,
        config: CompilerConfig,
    ) -> Self {
        Self {
            input: SourceInput::text(source, source_name),
            config,
        }
    }

    /// Constructs a request from a filesystem path.
    ///
    /// The file is not opened during request construction.
    ///
    /// UTF-8 decoding and filesystem error handling belong to the
    /// compiler-service boundary.
    ///
    /// No arbitrary source-file size limit is imposed.
    #[must_use]
    pub fn from_path(
        path: impl Into<PathBuf>,
        config: CompilerConfig,
    ) -> Self {
        Self {
            input: SourceInput::path(path),
            config,
        }
    }

    /// Constructs a request from an already-described source input.
    ///
    /// This is useful for integrations that receive source input from
    /// an editor, build system, language server or another service.
    #[must_use]
    pub fn new(
        input: SourceInput,
        config: CompilerConfig,
    ) -> Self {
        Self {
            input,
            config,
        }
    }

    /// Returns the source-input description.
    ///
    /// The returned reference does not transfer ownership.
    #[must_use]
    pub fn input(&self) -> &SourceInput {
        &self.input
    }

    /// Returns the authoritative compiler configuration.
    ///
    /// The returned reference does not transfer ownership.
    #[must_use]
    pub fn config(&self) -> &CompilerConfig {
        &self.config
    }

    /// Returns the in-memory source text when available.
    #[must_use]
    pub fn source_text(&self) -> Option<&str> {
        self.input.source_text()
    }

    /// Returns the logical source name when available.
    #[must_use]
    pub fn source_name(&self) -> Option<&str> {
        self.input.source_name()
    }

    /// Returns the filesystem path when available.
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        self.input.path_ref()
    }

    /// Returns true when the request contains in-memory source.
    #[must_use]
    pub fn is_text(&self) -> bool {
        self.input.is_text()
    }

    /// Returns true when the request refers to a filesystem path.
    #[must_use]
    pub fn is_path(&self) -> bool {
        self.input.is_path()
    }

    /// Returns a cloned compiler configuration.
    ///
    /// This is intended for service adapters that need to pass ownership
    /// of the existing CompilerConfig into an established compiler API.
    ///
    /// No new configuration semantics are introduced.
    #[must_use]
    pub fn cloned_config(&self) -> CompilerConfig {
        self.config.clone()
    }

    /// Returns a cloned source-input description.
    ///
    /// In-memory text remains shared through Arc<str>.
    #[must_use]
    pub fn cloned_input(&self) -> SourceInput {
        self.input.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler_types::{
        CompilationTarget,
        OptimizationLevel,
    };

    fn test_config() -> CompilerConfig {
        CompilerConfig {
            target: CompilationTarget::UniversalIRExport(
                "portable-test-ir".to_string(),
            ),
            opt_level: OptimizationLevel::None,
            debug_info: false,
            verify: true,
            emit_ir: true,
            parallel: false,
        }
    }

    #[test]
    fn source_request_preserves_source_and_name() {
        let request = CompilationRequest::from_source(
            "let answer = 42;",
            "example.zm",
            test_config(),
        );

        assert!(request.is_text());
        assert!(!request.is_path());

        assert_eq!(
            request.source_text(),
            Some("let answer = 42;")
        );

        assert_eq!(
            request.source_name(),
            Some("example.zm")
        );

        assert!(request.path().is_none());
    }

    #[test]
    fn path_request_preserves_path_without_reading_it() {
        let path = PathBuf::from("programs/example.zm");

        let request = CompilationRequest::from_path(
            path.clone(),
            test_config(),
        );

        assert!(!request.is_text());
        assert!(request.is_path());

        assert_eq!(
            request.path(),
            Some(path.as_path())
        );

        assert!(request.source_text().is_none());
        assert!(request.source_name().is_none());
    }

    #[test]
    fn request_preserves_explicit_configuration() {
        let request = CompilationRequest::from_source(
            "let value = 1;",
            "config.zm",
            test_config(),
        );

        assert!(request.config().verify);
        assert!(request.config().emit_ir);

        assert!(matches!(
            request.config().target,
            CompilationTarget::UniversalIRExport(_)
        ));

        assert_eq!(
            request.config().opt_level,
            OptimizationLevel::None
        );
    }

    #[test]
    fn cloning_text_request_shares_source_storage() {
        let source: Arc<str> = Arc::from(
            "let shared_source = 42;"
        );

        let request = CompilationRequest::from_source(
            Arc::clone(&source),
            "shared.zm",
            test_config(),
        );

        let cloned = request.clone();

        match (request.input(), cloned.input()) {
            (
                SourceInput::Text {
                    source: original,
                    ..
                },
                SourceInput::Text {
                    source: copied,
                    ..
                },
            ) => {
                assert!(Arc::ptr_eq(original, copied));
                assert_eq!(original.as_ref(), source.as_ref());
            }
            _ => panic!("expected text-backed requests"),
        }
    }

    #[test]
    fn source_input_accessors_are_consistent() {
        let input = SourceInput::text(
            "source",
            "source.zm",
        );

        assert!(input.is_text());
        assert!(!input.is_path());
        assert_eq!(input.source_text(), Some("source"));
        assert_eq!(input.source_name(), Some("source.zm"));
        assert!(input.path_ref().is_none());
    }

    #[test]
    fn path_input_accessors_are_consistent() {
        let input = SourceInput::path(
            "source.zm"
        );

        assert!(!input.is_text());
        assert!(input.is_path());
        assert!(input.source_text().is_none());
        assert!(input.source_name().is_none());
        assert_eq!(
            input.path_ref(),
            Some(Path::new("source.zm"))
        );
    }

    #[test]
    fn empty_source_is_preserved_for_parser_diagnostics() {
        let request = CompilationRequest::from_source(
            "",
            "empty.zm",
            test_config(),
        );

        assert_eq!(
            request.source_text(),
            Some("")
        );
    }

    #[test]
    fn empty_logical_name_is_not_silently_rewritten() {
        let request = CompilationRequest::from_source(
            "let value = 1;",
            "",
            test_config(),
        );

        assert_eq!(
            request.source_name(),
            Some("")
        );
    }

    #[test]
    fn cloned_configuration_preserves_existing_settings() {
        let request = CompilationRequest::from_source(
            "let value = 1;",
            "clone.zm",
            test_config(),
        );

        let config = request.cloned_config();

        assert!(config.verify);
        assert!(config.emit_ir);

        assert_eq!(
            config.opt_level,
            OptimizationLevel::None
        );
    }

    #[test]
    fn cloned_input_preserves_shared_source() {
        let request = CompilationRequest::from_source(
            "let value = 1;",
            "clone-input.zm",
            test_config(),
        );

        let cloned = request.cloned_input();

        assert_eq!(
            cloned.source_text(),
            Some("let value = 1;")
        );
    }

    #[test]
    fn independently_created_requests_have_independent_inputs() {
        let first = CompilationRequest::from_source(
            "let first = 1;",
            "first.zm",
            test_config(),
        );

        let second = CompilationRequest::from_source(
            "let second = 2;",
            "second.zm",
            test_config(),
        );

        assert_ne!(
            first.source_text(),
            second.source_text()
        );

        assert_ne!(
            first.source_name(),
            second.source_name()
        );
    }
}
