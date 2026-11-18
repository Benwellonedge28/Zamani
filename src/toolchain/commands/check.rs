
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Source checking command.
//!
//! This module provides a narrow toolchain command that validates a check
//! request and delegates compilation to the canonical compiler service.
//!
//! It does not implement parsing, semantic analysis, IR construction,
//! target discovery, backend selection, workspace resolution, dependency
//! resolution, artifact publication, or program execution.
//!
//! A successful report means that the configured compiler service completed
//! compilation successfully for the supplied source and configuration.
//! It does not claim that the program is executable on every possible
//! target or that an entire workspace has been validated.

use serde::{Deserialize, Serialize};

use crate::compiler::api::{
    CompilationRequest,
    CompilerResult,
    CompilerService,
    SourceInput,
};

/// Version identifier for the machine-readable check report.
pub const CHECK_REPORT_SCHEMA: &str = "zamani.toolchain.check-report.v1";

/// A validated request to check one source input.
///
/// The request retains the canonical compiler request rather than copying
/// or reinterpreting its source and configuration.
#[derive(Debug)]
pub struct CheckRequest {
    compilation: CompilationRequest,
}

impl CheckRequest {
    /// Creates a check request from a source input and compiler config.
    ///
    /// The source name or file path is checked for emptiness. Source text
    /// itself is allowed to be empty because reporting syntax or semantic
    /// errors belongs to the compiler.
    pub fn new(
        source: SourceInput,
        config: crate::compiler::CompilerConfig,
    ) -> Result<Self, CheckRequestError> {
        validate_source_input(&source)?;

        Ok(Self {
            compilation: CompilationRequest::new(source, config),
        })
    }

    /// Creates a check request from an existing compilation request.
    ///
    /// This is useful when another toolchain component has already prepared
    /// the canonical compiler request.
    pub fn from_compilation(
        compilation: CompilationRequest,
    ) -> Result<Self, CheckRequestError> {
        validate_source_input(&compilation.source)?;

        Ok(Self { compilation })
    }

    /// Returns the source name recorded by the compiler request.
    pub fn source_name(&self) -> String {
        self.compilation.source.source_name_owned()
    }

    /// Borrows the underlying canonical compilation request.
    pub fn compilation(&self) -> &CompilationRequest {
        &self.compilation
    }

    /// Consumes this check request and returns its compilation request.
    pub fn into_compilation(self) -> CompilationRequest {
        self.compilation
    }
}

/// Errors detected while constructing a check request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckRequestError {
    /// The in-memory source has an empty or whitespace-only name.
    EmptySourceName,

    /// The file source has an empty path.
    EmptySourcePath,
}

impl std::fmt::Display for CheckRequestError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySourceName => {
                formatter.write_str("source name must not be empty")
            }
            Self::EmptySourcePath => {
                formatter.write_str("source file path must not be empty")
            }
        }
    }
}

impl std::error::Error for CheckRequestError {}

/// Validates the identity of a source input without interpreting its content.
fn validate_source_input(
    source: &SourceInput,
) -> Result<(), CheckRequestError> {
    match source {
        SourceInput::Text { name, .. } => {
            if name.trim().is_empty() {
                return Err(CheckRequestError::EmptySourceName);
            }
        }
        SourceInput::File { path } => {
            if path.as_os_str().is_empty() {
                return Err(CheckRequestError::EmptySourcePath);
            }
        }
    }

    Ok(())
}

/// The outcome of a source check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckStatus {
    /// The configured compiler service completed successfully.
    Passed,
}

impl CheckStatus {
    /// Returns the stable machine-readable status string.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
        }
    }
}

/// Machine-readable result of a successful check operation.
///
/// A report is produced only when the compiler service returns success.
/// Compiler failures are returned as compiler errors and are not converted
/// into successful reports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckReport {
    /// Report schema identifier.
    pub schema: String,

    /// Check outcome.
    pub status: CheckStatus,

    /// Source name reported by the compiler.
    pub source_name: String,

    /// Debug representation of the compiler-selected target.
    ///
    /// This is descriptive metadata, not a portable target identifier.
    pub compiled_target: String,

    /// Size of the compiler output in bytes.
    pub output_size_bytes: u64,
}

impl CheckReport {
    /// Returns the report schema identifier.
    pub fn schema(&self) -> &str {
        &self.schema
    }

    /// Returns the check status.
    pub const fn status(&self) -> CheckStatus {
        self.status
    }

    /// Returns the source name.
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    /// Returns the compiler-selected target description.
    pub fn compiled_target(&self) -> &str {
        &self.compiled_target
    }

    /// Returns the compiler output size in bytes.
    pub const fn output_size_bytes(&self) -> u64 {
        self.output_size_bytes
    }
}

/// Toolchain command that delegates source checking to a compiler service.
///
/// The generic service parameter allows tests and future compiler-service
/// implementations to be injected without coupling this command to a
/// concrete compiler implementation.
#[derive(Debug)]
pub struct CheckCommand<C> {
    compiler: C,
}

impl<C> CheckCommand<C>
where
    C: CompilerService,
{
    /// Creates a check command using the supplied compiler service.
    pub fn new(compiler: C) -> Self {
        Self { compiler }
    }

    /// Borrows the compiler service used by this command.
    pub fn compiler(&self) -> &C {
        &self.compiler
    }

    /// Checks one source input using the supplied compiler configuration.
    ///
    /// The command delegates all language processing and compilation to
    /// `CompilerService`. It does not publish artifacts or execute output.
    ///
    /// # Errors
    ///
    /// Returns the compiler service's error if source loading, parsing,
    /// semantic analysis, lowering, code generation, or another compiler
    /// stage fails.
    pub fn check(
        &self,
        request: CheckRequest,
    ) -> CompilerResult<CheckReport> {
        let response = self.compiler.compile(request.into_compilation())?;

        let report = CheckReport {
            schema: CHECK_REPORT_SCHEMA.to_owned(),
            status: CheckStatus::Passed,
            source_name: response.source_name,
            compiled_target: format!("{:?}", response.target()),
            output_size_bytes: response.size_bytes(),
        };

        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::api::CompilerApi;

    #[test]
    fn rejects_empty_source_name() {
        let source = SourceInput::text("source", "");

        assert_eq!(
            validate_source_input(&source),
            Err(CheckRequestError::EmptySourceName)
        );
    }

    #[test]
    fn rejects_whitespace_source_name() {
        let source = SourceInput::text("source", "   ");

        assert_eq!(
            validate_source_input(&source),
            Err(CheckRequestError::EmptySourceName)
        );
    }

    #[test]
    fn accepts_empty_source_text_with_a_name() {
        let source = SourceInput::text("", "empty.zm");

        assert!(validate_source_input(&source).is_ok());
    }

    #[test]
    fn rejects_empty_file_path() {
        let source = SourceInput::file(std::path::PathBuf::new());

        assert_eq!(
            validate_source_input(&source),
            Err(CheckRequestError::EmptySourcePath)
        );
    }

    #[test]
    fn accepts_nonempty_file_path() {
        let source = SourceInput::file("main.zm");

        assert!(validate_source_input(&source).is_ok());
    }

    #[test]
    fn check_status_has_stable_machine_readable_value() {
        assert_eq!(CheckStatus::Passed.as_str(), "passed");
    }

    #[test]
    fn report_schema_is_versioned() {
        assert_eq!(
            CHECK_REPORT_SCHEMA,
            "zamani.toolchain.check-report.v1"
        );
    }

    #[test]
    fn report_serializes_with_stable_field_names() {
        let report = CheckReport {
            schema: CHECK_REPORT_SCHEMA.to_owned(),
            status: CheckStatus::Passed,
            source_name: "main.zm".to_owned(),
            compiled_target: "TestTarget".to_owned(),
            output_size_bytes: 128,
        };

        let json = serde_json::to_string(&report)
            .expect("check report should serialize");

        assert!(json.contains("\"schema\":"));
        assert!(json.contains("\"status\":\"passed\""));
        assert!(json.contains("\"source_name\":\"main.zm\""));
        assert!(json.contains("\"output_size_bytes\":128"));
    }

    #[test]
    fn report_deserializes() {
        let json = format!(
            concat!(
                "{{",
                "\"schema\":\"{}\",",
                "\"status\":\"passed\",",
                "\"source_name\":\"main.zm\",",
                "\"compiled_target\":\"TestTarget\",",
                "\"output_size_bytes\":128",
                "}}"
            ),
            CHECK_REPORT_SCHEMA
        );

        let report: CheckReport =
            serde_json::from_str(&json).expect("check report should deserialize");

        assert_eq!(report.status(), CheckStatus::Passed);
        assert_eq!(report.source_name(), "main.zm");
        assert_eq!(report.output_size_bytes(), 128);
    }

    #[test]
    fn command_accepts_the_canonical_compiler_api() {
        let command = CheckCommand::new(CompilerApi::new());

        let _compiler = command.compiler();
    }
}
