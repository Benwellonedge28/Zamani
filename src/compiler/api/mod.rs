
```rust
//! Zamani Universal Meta-Compiler — Public API
//!
//! Path:
//!     src/compiler/api/mod.rs
//!
//! Purpose:
//!     Provide the stable public API boundary for the Zamani compiler.
//!
//! Responsibilities:
//!     - Expose the existing canonical compiler pipeline.
//!     - Represent compilation requests.
//!     - Represent compilation responses.
//!     - Provide a stable compiler service interface.
//!     - Support in-memory source compilation.
//!     - Support source-file compilation.
//!     - Preserve existing compiler configuration and error types.
//!     - Maintain compatibility with existing compiler entry points.
//!
//! Non-responsibilities:
//!     - Parsing implementation.
//!     - AST ownership.
//!     - Semantic analysis.
//!     - Ownership and borrow analysis.
//!     - IR ownership.
//!     - Optimization implementation.
//!     - Hardware discovery.
//!     - Resource allocation.
//!     - Backend implementation.
//!     - Runtime execution.
//!
//! Architecture:
//!
//!     Public API
//!         |
//!         v
//!     CompilationRequest
//!         |
//!         v
//!     CompilerService
//!         |
//!         v
//!     Existing compiler pipeline
//!         |
//!         +--> Lexer
//!         +--> Parser
//!         +--> Semantic analysis
//!         +--> Ownership integration
//!         +--> IR generation
//!         +--> Optimization
//!         +--> Verification
//!         +--> Security inspection
//!         +--> Backend
//!         |
//!         v
//!     CompilationResponse
//!
//! Compatibility:
//!     Existing compile(), compile_source(), and
//!     compile_with_config() remain authoritative.
//!
//! Scalability:
//!     This module introduces no universal hardware-capacity
//!     restrictions. Actual resource limits belong to the
//!     relevant compiler, target, policy, and runtime layers.
//!
//! Rust:
//!     Rust 2021 edition.
//!     Rust 1.97 or later.
//!     Safe Rust only.
//!
//! POCO-REAF:
//!     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.

use std::fmt;
use std::path::{Path, PathBuf};

// ============================================================================
// EXISTING COMPILER AUTHORITY
// ============================================================================

/// Existing canonical compiler configuration.
///
/// The configuration is owned by compiler_types.rs.
///
/// This API must not introduce a competing configuration structure.
pub use crate::compiler_types::CompilerConfig;

/// Existing canonical compiler optimization configuration.
pub use crate::compiler_types::OptimizationLevel;

/// Existing canonical compilation target type.
pub use crate::compiler_types::CompilationTarget;

/// Existing compiler error type.
pub use super::CompilerError;

/// Existing compiler output type.
pub use super::CompilationOutput;

/// Existing production configuration factory.
pub use super::production_config;

/// Existing source compilation entry point.
pub use super::compile_source;

/// Existing source-file compilation entry point.
pub use super::compile_with_config;

/// Existing compatibility entry point.
pub use super::compile;

// ============================================================================
// RESULT
// ============================================================================

/// Standard result returned by the public compiler API.
///
/// All errors preserve the existing CompilerError type.
pub type CompilerResult<T> = Result<T, CompilerError>;

// ============================================================================
// SOURCE INPUT
// ============================================================================

/// Identifies the source supplied to a compilation request.
///
/// SourceInput supports both in-memory source and filesystem source.
///
/// The API does not impose a universal source-size limit.
///
/// Actual memory and operating-system limitations are handled
/// by the underlying implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceInput {
    /// Source supplied directly as text.
    ///
    /// `name` identifies the logical source file and is used
    /// for diagnostics and source tracking.
    Text {
        source: String,
        name: String,
    },

    /// Source supplied through a filesystem path.
    File {
        path: PathBuf,
    },
}

impl SourceInput {
    /// Creates an in-memory source input.
    pub fn text(
        source: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self::Text {
            source: source.into(),
            name: name.into(),
        }
    }

    /// Creates a filesystem source input.
    pub fn file(path: impl Into<PathBuf>) -> Self {
        Self::File {
            path: path.into(),
        }
    }

    /// Returns the logical source name.
    ///
    /// For filesystem sources, the path is converted to a
    /// display representation.
    pub fn source_name(&self) -> &str {
        match self {
            Self::Text { name, .. } => name,

            // Filesystem paths are not necessarily UTF-8.
            // Their display representation is therefore
            // generated by source_name_owned().
            Self::File { .. } => "",
        }
    }

    /// Returns an owned source-name representation.
    pub fn source_name_owned(&self) -> String {
        match self {
            Self::Text { name, .. } => name.clone(),

            Self::File { path } => path.display().to_string(),
        }
    }

    /// Returns the source text when the input is in-memory.
    ///
    /// Filesystem inputs must be loaded by the compiler service.
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text { source, .. } => Some(source),

            Self::File { .. } => None,
        }
    }

    /// Returns the filesystem path when the input is file-based.
    pub fn as_path(&self) -> Option<&Path> {
        match self {
            Self::File { path } => Some(path.as_path()),

            Self::Text { .. } => None,
        }
    }

    /// Indicates whether the source is in-memory.
    pub fn is_in_memory(&self) -> bool {
        matches!(self, Self::Text { .. })
    }

    /// Indicates whether the source is filesystem-based.
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File { .. })
    }
}

// ============================================================================
// COMPILATION REQUEST
// ============================================================================

/// Represents one complete compilation request.
///
/// A request contains:
///
/// - source input;
/// - explicit compiler configuration.
///
/// Compiler configuration remains owned by compiler_types.rs.
///
/// This structure does not perform parsing, semantic analysis,
/// hardware discovery, or code generation.
#[derive(Debug, Clone)]
pub struct CompilationRequest {
    /// Input source.
    pub source: SourceInput,

    /// Existing canonical compiler configuration.
    pub config: CompilerConfig,
}

impl CompilationRequest {
    /// Creates a compilation request.
    ///
    /// Configuration must be supplied explicitly.
    ///
    /// This avoids silently imposing a particular hardware target
    /// through an API-level default.
    pub fn new(
        source: SourceInput,
        config: CompilerConfig,
    ) -> Self {
        Self {
            source,
            config,
        }
    }

    /// Creates a request from source text.
    pub fn from_source(
        source: impl Into<String>,
        source_name: impl Into<String>,
        config: CompilerConfig,
    ) -> Self {
        Self::new(
            SourceInput::text(source, source_name),
            config,
        )
    }

    /// Creates a request from a filesystem path.
    pub fn from_file(
        path: impl Into<PathBuf>,
        config: CompilerConfig,
    ) -> Self {
        Self::new(
            SourceInput::file(path),
            config,
        )
    }

    /// Returns the logical source name.
    pub fn source_name(&self) -> String {
        self.source.source_name_owned()
    }

    /// Returns whether this request contains in-memory source.
    pub fn contains_source_text(&self) -> bool {
        self.source.is_in_memory()
    }

    /// Returns whether this request contains a filesystem path.
    pub fn contains_source_file(&self) -> bool {
        self.source.is_file()
    }

    /// Returns a reference to the compiler configuration.
    pub fn configuration(&self) -> &CompilerConfig {
        &self.config
    }
}

// ============================================================================
// COMPILATION RESPONSE
// ============================================================================

/// Represents the successful result of a compilation request.
///
/// The actual generated artifact is owned by the existing
/// CompilationOutput structure.
///
/// This response does not duplicate artifact storage or backend logic.
#[derive(Debug)]
pub struct CompilationResponse {
    /// Original logical source name.
    pub source_name: String,

    /// Existing canonical compiler output.
    pub output: CompilationOutput,
}

impl CompilationResponse {
    /// Creates a compilation response.
    pub fn new(
        source_name: impl Into<String>,
        output: CompilationOutput,
    ) -> Self {
        Self {
            source_name: source_name.into(),
            output,
        }
    }

    /// Returns the generated artifact.
    pub fn output(&self) -> &CompilationOutput {
        &self.output
    }

    /// Returns the generated artifact bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.output.bytes
    }

    /// Returns the selected target identifier.
    pub fn target(&self) -> &str {
        &self.output.target
    }

    /// Returns the generated artifact size.
    pub fn size_bytes(&self) -> usize {
        self.output.size_bytes
    }

    /// Consumes the response and returns the existing output.
    pub fn into_output(self) -> CompilationOutput {
        self.output
    }
}

// ============================================================================
// COMPILER SERVICE CONTRACT
// ============================================================================

/// Stable compiler service interface.
///
/// Implementations must delegate to the canonical compiler
/// pipeline rather than creating independent compilation pipelines.
///
/// The trait is object-safe and can therefore be used behind
/// trait objects by applications and tooling.
///
/// Implementations must preserve compiler errors.
pub trait CompilerService {
    /// Compiles one request.
    fn compile(
        &self,
        request: CompilationRequest,
    ) -> CompilerResult<CompilationResponse>;

    /// Compiles in-memory source.
    fn compile_text(
        &self,
        source: &str,
        source_name: &str,
        config: CompilerConfig,
    ) -> CompilerResult<CompilationResponse> {
        self.compile(
            CompilationRequest::from_source(
                source,
                source_name,
                config,
            ),
        )
    }

    /// Compiles a filesystem source.
    fn compile_file(
        &self,
        path: &Path,
        config: CompilerConfig,
    ) -> CompilerResult<CompilationResponse> {
        self.compile(
            CompilationRequest::from_file(
                path.to_path_buf(),
                config,
            ),
        )
    }
}

// ============================================================================
// COMPILER API
// ============================================================================

/// Default stateless public compiler API.
///
/// CompilerApi owns no global mutable compilation state.
///
/// Each compilation delegates to the existing canonical
/// compiler pipeline.
///
/// This makes the API suitable for:
///
/// - command-line applications;
/// - IDE integrations;
/// - language servers;
/// - build systems;
/// - testing tools;
/// - embedded compiler consumers;
/// - future orchestration services.
#[derive(Debug, Clone, Copy, Default)]
pub struct CompilerApi;

impl CompilerApi {
    /// Creates a compiler API instance.
    pub const fn new() -> Self {
        Self
    }

    /// Compiles one request.
    pub fn compile(
        &self,
        request: CompilationRequest,
    ) -> CompilerResult<CompilationResponse> {
        <Self as CompilerService>::compile(self, request)
    }

    /// Compiles source text using explicit configuration.
    pub fn compile_text(
        &self,
        source: &str,
        source_name: &str,
        config: CompilerConfig,
    ) -> CompilerResult<CompilationResponse> {
        <Self as CompilerService>::compile_text(
            self,
            source,
            source_name,
            config,
        )
    }

    /// Compiles a source file using explicit configuration.
    pub fn compile_file(
        &self,
        path: &Path,
        config: CompilerConfig,
    ) -> CompilerResult<CompilationResponse> {
        <Self as CompilerService>::compile_file(
            self,
            path,
            config,
        )
    }

    /// Returns the existing compiler production configuration.
    ///
    /// Important:
    /// This method delegates to production_config().
    /// It does not independently choose a target.
    ///
    /// The existing production_config() currently selects
    /// X86_64Linux and must be corrected in compiler/mod.rs
    /// before this default can be considered target-independent.
    pub fn production_config(&self) -> CompilerConfig {
        super::production_config()
    }
}

impl CompilerService for CompilerApi {
    fn compile(
        &self,
        request: CompilationRequest,
    ) -> CompilerResult<CompilationResponse> {
        let source_name = request.source.source_name_owned();

        let source_code = match request.source {
            SourceInput::Text {
                source,
                ..
            } => source,

            SourceInput::File {
                path,
            } => {
                std::fs::read_to_string(&path)
                    .map_err(|error| CompilerError::Io {
                        path: path.display().to_string(),
                        message: error.to_string(),
                    })?
            }
        };

        let output = super::compile_source(
            &source_code,
            &source_name,
            request.config,
        )?;

        Ok(CompilationResponse::new(
            source_name,
            output,
        ))
    }
}

// ============================================================================
// DISPLAY AND DEBUG SUPPORT
// ============================================================================

impl fmt::Display for SourceInput {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Text {
                name,
                ..
            } => write!(
                formatter,
                "in-memory source `{name}`"
            ),

            Self::File {
                path,
            } => write!(
                formatter,
                "source file `{}`",
                path.display()
            ),
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> CompilerConfig {
        super::production_config()
    }

    #[test]
    fn api_can_be_constructed_without_global_initialization() {
        let api = CompilerApi::new();

        let _ = api;
    }

    #[test]
    fn api_is_default_constructible() {
        let api = CompilerApi::default();

        let _ = api;
    }

    #[test]
    fn source_input_accepts_in_memory_source() {
        let source = SourceInput::text(
            "fn main() {}",
            "main.zm",
        );

        assert!(source.is_in_memory());
        assert!(!source.is_file());

        assert_eq!(
            source.as_text(),
            Some("fn main() {}")
        );

        assert_eq!(
            source.source_name_owned(),
            "main.zm"
        );
    }

    #[test]
    fn source_input_accepts_filesystem_paths() {
        let source = SourceInput::file("main.zm");

        assert!(source.is_file());
        assert!(!source.is_in_memory());

        assert_eq!(
            source.as_path(),
            Some(Path::new("main.zm"))
        );
    }

    #[test]
    fn source_input_display_is_available() {
        let source = SourceInput::text(
            "fn main() {}",
            "main.zm",
        );

        assert_eq!(
            source.to_string(),
            "in-memory source `main.zm`"
        );
    }

    #[test]
    fn compilation_request_accepts_source_text() {
        let request = CompilationRequest::from_source(
            "fn main() {}",
            "main.zm",
            test_config(),
        );

        assert!(request.contains_source_text());
        assert!(!request.contains_source_file());

        assert_eq!(
            request.source_name(),
            "main.zm"
        );
    }

    #[test]
    fn compilation_request_accepts_source_file() {
        let request = CompilationRequest::from_file(
            "main.zm",
            test_config(),
        );

        assert!(request.contains_source_file());
        assert!(!request.contains_source_text());

        assert_eq!(
            request.source_name(),
            "main.zm"
        );
    }

    #[test]
    fn compilation_request_preserves_configuration() {
        let config = test_config();

        let request = CompilationRequest::new(
            SourceInput::text(
                "fn main() {}",
                "main.zm",
            ),
            config.clone(),
        );

        assert_eq!(
            request.config.opt_level,
            config.opt_level
        );

        assert_eq!(
            request.config.debug_info,
            config.debug_info
        );
    }

    #[test]
    fn compilation_request_exposes_configuration_reference() {
        let request = CompilationRequest::from_source(
            "fn main() {}",
            "main.zm",
            test_config(),
        );

        let config = request.configuration();

        assert!(config.debug_info);
    }

    #[test]
    fn compiler_api_provides_production_configuration() {
        let api = CompilerApi::new();

        let config = api.production_config();

        assert!(config.verify);
    }

    #[test]
    fn compiler_service_is_object_safe() {
        let api = CompilerApi::new();

        let service: &dyn CompilerService = &api;

        let _ = service;
    }

    #[test]
    fn compiler_result_uses_existing_error_type() {
        fn accepts_compiler_result(
            result: CompilerResult<()>,
        ) -> CompilerResult<()> {
            result
        }

        assert!(accepts_compiler_result(Ok(())).is_ok());
    }

    #[test]
    fn response_preserves_source_name() {
        let response = CompilationResponse {
            source_name: "main.zm".to_string(),
            output: CompilationOutput {
                bytes: vec![1, 2, 3],
                target: "test-target".to_string(),
                size_bytes: 3,
            },
        };

        assert_eq!(
            response.source_name,
            "main.zm"
        );

        assert_eq!(
            response.bytes(),
            &[1, 2, 3]
        );

        assert_eq!(
            response.target(),
            "test-target"
        );

        assert_eq!(
            response.size_bytes(),
            3
        );
    }

    #[test]
    fn response_can_return_existing_compilation_output() {
        let response = CompilationResponse {
            source_name: "main.zm".to_string(),
            output: CompilationOutput {
                bytes: vec![1, 2, 3],
                target: "test-target".to_string(),
                size_bytes: 3,
            },
        };

        let output = response.into_output();

        assert_eq!(output.bytes, vec![1, 2, 3]);
    }
}
```
