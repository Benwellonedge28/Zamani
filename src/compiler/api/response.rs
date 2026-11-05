
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Compilation Responses.
//!
//! Path: src/compiler/api/response.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Defines the stable response contract for one compilation operation.
//!
//! A CompilationResponse carries:
//!
//! - the actual result returned by the compiler;
//! - structured diagnostics produced during compilation;
//! - convenient read-only access to success and failure information.
//!
//! The response does not implement compilation.
//!
//! It does not parse source code, construct ASTs, perform semantic analysis,
//! generate IR, optimize programs, select hardware, invoke backends,
//! link artifacts, or execute programs.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - the compiler API response envelope;
//! - the success/failure outcome representation;
//! - response-level structured diagnostic association;
//! - response inspection and result extraction.
//!
//! This module does not own:
//!
//! - CompilationOutput;
//! - CompilerError;
//! - diagnostic definitions;
//! - compiler configuration;
//! - compilation requests;
//! - compilation sessions;
//! - source-language semantics;
//! - canonical intermediate representations;
//! - target discovery;
//! - hardware allocation;
//! - backend implementation;
//! - runtime execution.
//!
//! # Existing repository authorities
//!
//! CompilationOutput and CompilerError are defined by:
//!
//!     crate::compiler
//!
//! Diagnostic and DiagnosticLevel are defined by:
//!
//!     crate::compiler::diagnostics
//!
//! This module reuses those types.
//!
//! It deliberately does not define another artifact, error, or diagnostic
//! system.
//!
//! # POCO-REAF
//!
//! A successful response means that the selected compiler implementation
//! returned a successful CompilationOutput.
//!
//! It does not independently certify:
//!
//! - universal hardware compatibility;
//! - portability to every target;
//! - availability of every requested capability;
//! - successful deployment;
//! - successful runtime execution;
//! - semantic equivalence across all possible backends.
//!
//! Those claims require verification by their authoritative compiler,
//! artifact, backend, and runtime subsystems.
//!
//! The response does not impose source-size, operation-count, device-count,
//! processor-count, memory-capacity, or hardware-inventory limits.
//!
//! # Error handling
//!
//! The original CompilerError is preserved without converting it into a
//! string. Callers can inspect the original error variant.
//!
//! Structured diagnostics remain available independently of the outcome.
//!
//! # Integration
//!
//! Upstream:
//!
//! - api/compiler.rs;
//! - api/session.rs;
//! - compilation pipeline;
//! - diagnostic engine;
//! - command-line consumers;
//! - IDE and language-server consumers;
//! - build systems;
//! - tests and automation.
//!
//! Downstream:
//!
//! - api/mod.rs;
//! - external compiler API consumers;
//! - response formatting and presentation adapters.
//!
//! The response type is intentionally independent of request.rs.
//! This prevents response construction from depending on request ownership
//! or source-input representation.
//!
//! # Compatibility
//!
//! Existing compiler entry points remain unchanged.
//!
//! Existing CompilationOutput and CompilerError remain authoritative.
//!
//! This is an additive API.

use crate::compiler::{CompilationOutput, CompilerError};
use crate::compiler::diagnostics::Diagnostic;

/// The outcome of one compilation operation.
///
/// The original compiler output or compiler error is retained.
///
/// No error is converted into a string, and no successful output is
/// fabricated when compilation fails.
#[derive(Debug)]
pub enum CompilationOutcome {
    /// Compilation completed successfully and returned an output artifact.
    Succeeded(CompilationOutput),

    /// Compilation failed.
    ///
    /// The original structured compiler error is preserved.
    Failed(CompilerError),
}

impl CompilationOutcome {
    /// Returns true when compilation succeeded.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        matches!(self, Self::Succeeded(_))
    }

    /// Returns true when compilation failed.
    #[must_use]
    pub const fn is_failure(&self) -> bool {
        matches!(self, Self::Failed(_))
    }

    /// Returns the successful compilation output, if available.
    #[must_use]
    pub fn output(&self) -> Option<&CompilationOutput> {
        match self {
            Self::Succeeded(output) => Some(output),
            Self::Failed(_) => None,
        }
    }

    /// Returns the original compiler error, if compilation failed.
    #[must_use]
    pub fn error(&self) -> Option<&CompilerError> {
        match self {
            Self::Succeeded(_) => None,
            Self::Failed(error) => Some(error),
        }
    }

    /// Consumes the outcome and returns the underlying compiler result.
    ///
    /// This preserves the original CompilerError and CompilationOutput.
    #[must_use]
    pub fn into_result(self) -> Result<CompilationOutput, CompilerError> {
        match self {
            Self::Succeeded(output) => Ok(output),
            Self::Failed(error) => Err(error),
        }
    }
}

impl From<Result<CompilationOutput, CompilerError>> for CompilationOutcome {
    fn from(result: Result<CompilationOutput, CompilerError>) -> Self {
        match result {
            Ok(output) => Self::Succeeded(output),
            Err(error) => Self::Failed(error),
        }
    }
}

/// Response returned by a compiler API operation.
///
/// A response contains the actual compilation outcome and all structured
/// diagnostics explicitly attached by the caller.
///
/// Diagnostics are kept separate from the outcome because:
///
/// - successful compilation may produce warnings, notes, or help messages;
/// - failed compilation may produce multiple diagnostics;
/// - the existing CompilerError may summarize a failure that has several
///   associated diagnostics.
///
/// This type does not invent diagnostic information or infer success from
/// the absence of diagnostics.
///
/// The outcome is authoritative for success or failure.
#[derive(Debug)]
pub struct CompilationResponse {
    outcome: CompilationOutcome,
    diagnostics: Vec<Diagnostic>,
}

impl CompilationResponse {
    /// Constructs a response from an existing compiler result.
    ///
    /// This is the primary integration constructor.
    ///
    /// It preserves the original CompilationOutput or CompilerError.
    ///
    /// Diagnostics may be empty when the underlying compilation pipeline
    /// does not expose structured diagnostics for the operation.
    #[must_use]
    pub fn new(
        result: Result<CompilationOutput, CompilerError>,
    ) -> Self {
        Self {
            outcome: CompilationOutcome::from(result),
            diagnostics: Vec::new(),
        }
    }

    /// Constructs a successful compilation response.
    ///
    /// The supplied output must be an actual output returned by a compiler
    /// implementation. This constructor does not fabricate artifacts.
    #[must_use]
    pub fn success(output: CompilationOutput) -> Self {
        Self {
            outcome: CompilationOutcome::Succeeded(output),
            diagnostics: Vec::new(),
        }
    }

    /// Constructs a failed compilation response.
    ///
    /// The original compiler error is preserved.
    #[must_use]
    pub fn failure(error: CompilerError) -> Self {
        Self {
            outcome: CompilationOutcome::Failed(error),
            diagnostics: Vec::new(),
        }
    }

    /// Attaches structured diagnostics to this response.
    ///
    /// Existing diagnostics are preserved and new diagnostics are appended
    /// in the order supplied.
    ///
    /// This method does not suppress duplicates or impose a diagnostic
    /// count limit. Those policies belong to DiagnosticEngine.
    #[must_use]
    pub fn with_diagnostics(
        mut self,
        diagnostics: impl IntoIterator<Item = Diagnostic>,
    ) -> Self {
        self.diagnostics.extend(diagnostics);
        self
    }

    /// Appends one structured diagnostic.
    ///
    /// Diagnostic ordering is preserved.
    pub fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Returns the compilation outcome.
    #[must_use]
    pub fn outcome(&self) -> &CompilationOutcome {
        &self.outcome
    }

    /// Returns the attached structured diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Returns the successful compilation output, if available.
    #[must_use]
    pub fn output(&self) -> Option<&CompilationOutput> {
        self.outcome.output()
    }

    /// Returns the original compiler error, if compilation failed.
    #[must_use]
    pub fn error(&self) -> Option<&CompilerError> {
        self.outcome.error()
    }

    /// Returns true when compilation succeeded.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.outcome.is_success()
    }

    /// Returns true when compilation failed.
    #[must_use]
    pub const fn is_failure(&self) -> bool {
        self.outcome.is_failure()
    }

    /// Returns the number of attached diagnostics.
    ///
    /// This reports the actual number retained by this response.
    /// It is not a configured or universal diagnostic limit.
    #[must_use]
    pub fn diagnostic_count(&self) -> usize {
        self.diagnostics.len()
    }

    /// Returns true when at least one attached diagnostic has error severity.
    ///
    /// This does not replace the authoritative compilation outcome.
    ///
    /// A response can contain an error-level diagnostic for explanatory
    /// purposes even when a caller is inspecting a successful result.
    /// The compiler pipeline remains responsible for deciding whether
    /// such a diagnostic prevents successful compilation.
    #[must_use]
    pub fn has_error_diagnostics(&self) -> bool {
        self.diagnostics.iter().any(Diagnostic::is_error)
    }

    /// Consumes the response and returns the underlying compiler result.
    ///
    /// Attached diagnostics are discarded by this operation.
    ///
    /// Use into_parts() when diagnostics must also be retained.
    #[must_use]
    pub fn into_result(self) -> Result<CompilationOutput, CompilerError> {
        self.outcome.into_result()
    }

    /// Consumes the response and returns its outcome and diagnostics.
    ///
    /// This is useful for presentation layers and integrations that need
    /// to preserve all response information.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (CompilationOutcome, Vec<Diagnostic>) {
        (self.outcome, self.diagnostics)
    }
}

impl From<Result<CompilationOutput, CompilerError>> for CompilationResponse {
    fn from(result: Result<CompilationOutput, CompilerError>) -> Self {
        Self::new(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_output() -> CompilationOutput {
        let bytes = b"portable-test-artifact".to_vec();

        CompilationOutput {
            size_bytes: bytes.len(),
            bytes,
            target: "test-target".to_string(),
        }
    }

    fn test_diagnostic(
        level: crate::compiler::diagnostics::DiagnosticLevel,
        code: &str,
        message: &str,
    ) -> Diagnostic {
        Diagnostic::new(level, code, message)
    }

    #[test]
    fn successful_response_preserves_compilation_output() {
        let output = test_output();

        let response = CompilationResponse::success(output);

        assert!(response.is_success());
        assert!(!response.is_failure());
        assert!(response.error().is_none());

        let returned = response
            .output()
            .expect("successful response must expose output");

        assert_eq!(returned.bytes, b"portable-test-artifact");
        assert_eq!(returned.size_bytes, returned.bytes.len());
        assert_eq!(returned.target, "test-target");
    }

    #[test]
    fn failed_response_preserves_original_compiler_error() {
        let response = CompilationResponse::failure(
            CompilerError::Parse {
                message: "invalid source".to_string(),
            },
        );

        assert!(response.is_failure());
        assert!(!response.is_success());
        assert!(response.output().is_none());

        assert!(matches!(
            response.error(),
            Some(CompilerError::Parse { message })
                if message == "invalid source"
        ));
    }

    #[test]
    fn new_preserves_successful_result() {
        let response = CompilationResponse::new(Ok(test_output()));

        assert!(response.is_success());
        assert!(response.output().is_some());
    }

    #[test]
    fn new_preserves_failed_result() {
        let response = CompilationResponse::new(
            Err(CompilerError::Semantic {
                message: "invalid semantic construct".to_string(),
            }),
        );

        assert!(response.is_failure());

        assert!(matches!(
            response.error(),
            Some(CompilerError::Semantic { .. })
        ));
    }

    #[test]
    fn successful_response_can_contain_warnings() {
        use crate::compiler::diagnostics::DiagnosticLevel;

        let response = CompilationResponse::success(test_output())
            .with_diagnostics([test_diagnostic(
                DiagnosticLevel::Warning,
                "W0001",
                "test warning",
            )]);

        assert!(response.is_success());
        assert_eq!(response.diagnostic_count(), 1);
        assert!(!response.has_error_diagnostics());
    }

    #[test]
    fn failed_response_can_retain_multiple_diagnostics() {
        use crate::compiler::diagnostics::DiagnosticLevel;

        let response = CompilationResponse::failure(
            CompilerError::Semantic {
                message: "semantic validation failed".to_string(),
            },
        )
        .with_diagnostics([
            test_diagnostic(
                DiagnosticLevel::Error,
                "E1001",
                "first semantic error",
            ),
            test_diagnostic(
                DiagnosticLevel::Error,
                "E1002",
                "second semantic error",
            ),
        ]);

        assert!(response.is_failure());
        assert_eq!(response.diagnostic_count(), 2);
        assert!(response.has_error_diagnostics());
    }

    #[test]
    fn diagnostics_preserve_insertion_order() {
        use crate::compiler::diagnostics::DiagnosticLevel;

        let response = CompilationResponse::success(test_output())
            .with_diagnostics([
                test_diagnostic(
                    DiagnosticLevel::Warning,
                    "W0001",
                    "first",
                ),
                test_diagnostic(
                    DiagnosticLevel::Note,
                    "N0001",
                    "second",
                ),
                test_diagnostic(
                    DiagnosticLevel::Help,
                    "H0001",
                    "third",
                ),
            ]);

        let diagnostics = response.diagnostics();

        assert_eq!(diagnostics.len(), 3);
        assert_eq!(diagnostics[0].code, "W0001");
        assert_eq!(diagnostics[1].code, "N0001");
        assert_eq!(diagnostics[2].code, "H0001");
    }

    #[test]
    fn push_diagnostic_appends_without_replacing_existing_items() {
        use crate::compiler::diagnostics::DiagnosticLevel;

        let mut response = CompilationResponse::success(test_output());

        response.push_diagnostic(test_diagnostic(
            DiagnosticLevel::Warning,
            "W0001",
            "first warning",
        ));

        response.push_diagnostic(test_diagnostic(
            DiagnosticLevel::Note,
            "N0001",
            "additional note",
        ));

        assert_eq!(response.diagnostic_count(), 2);
        assert_eq!(response.diagnostics()[0].code, "W0001");
        assert_eq!(response.diagnostics()[1].code, "N0001");
    }

    #[test]
    fn outcome_preserves_success_and_failure_information() {
        let success = CompilationResponse::success(test_output());

        assert!(success.outcome().is_success());
        assert!(success.outcome().output().is_some());
        assert!(success.outcome().error().is_none());

        let failure = CompilationResponse::failure(
            CompilerError::Verification {
                message: "invalid IR".to_string(),
            },
        );

        assert!(failure.outcome().is_failure());
        assert!(failure.outcome().output().is_none());
        assert!(failure.outcome().error().is_some());
    }

    #[test]
    fn into_result_returns_the_original_successful_output() {
        let response = CompilationResponse::success(test_output());

        let output = response
            .into_result()
            .expect("successful response must return output");

        assert_eq!(output.bytes, b"portable-test-artifact");
        assert_eq!(output.target, "test-target");
    }

    #[test]
    fn into_result_returns_the_original_error_variant() {
        let response = CompilationResponse::failure(
            CompilerError::CodeGeneration {
                message: "backend failed".to_string(),
            },
        );

        let result = response.into_result();

        assert!(matches!(
            result,
            Err(CompilerError::CodeGeneration { .. })
        ));
    }

    #[test]
    fn into_parts_preserves_outcome_and_diagnostics() {
        use crate::compiler::diagnostics::DiagnosticLevel;

        let response = CompilationResponse::success(test_output())
            .with_diagnostics([test_diagnostic(
                DiagnosticLevel::Warning,
                "W0001",
                "retained warning",
            )]);

        let (outcome, diagnostics) = response.into_parts();

        assert!(outcome.is_success());
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, "W0001");
    }

    #[test]
    fn empty_diagnostics_are_valid() {
        let response = CompilationResponse::success(test_output());

        assert!(response.diagnostics().is_empty());
        assert_eq!(response.diagnostic_count(), 0);
        assert!(!response.has_error_diagnostics());
    }

    #[test]
    fn conversion_from_result_preserves_failure_classification() {
        let result: Result<CompilationOutput, CompilerError> =
            Err(CompilerError::Configuration {
                message: "invalid configuration".to_string(),
            });

        let response = CompilationResponse::from(result);

        assert!(response.is_failure());

        assert!(matches!(
            response.error(),
            Some(CompilerError::Configuration { .. })
        ));
    }

    #[test]
    fn diagnostic_error_presence_does_not_change_outcome() {
        use crate::compiler::diagnostics::DiagnosticLevel;

        let response = CompilationResponse::success(test_output())
            .with_diagnostics([test_diagnostic(
                DiagnosticLevel::Error,
                "E0001",
                "diagnostic supplied by an external reporting layer",
            )]);

        // The outcome remains the actual compiler result.
        // The response does not independently redefine pipeline policy.
        assert!(response.is_success());
        assert!(response.has_error_diagnostics());
    }
}
