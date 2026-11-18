
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Lint Command
//!
//! Path: `src/toolchain/commands/lint.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the toolchain-facing lint command.
//!
//! This module coordinates a registered or injected lint-analysis
//! implementation and publishes its findings through the existing
//! Zamani compiler diagnostic infrastructure.
//!
//! It is an orchestration and reporting boundary. It is not a
//! second lexer, parser, semantic analyzer, compiler, diagnostic
//! engine, formatter, source editor, or backend.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Lint request validation.
//! - Lint command orchestration.
//! - The lint-provider integration contract.
//! - Streaming diagnostic collection.
//! - Lint-specific outcome and report metadata.
//! - Configurable warning enforcement.
//! - Lint report serialization.
//!
//! This module does not own:
//!
//! - Lexical analysis.
//! - Parsing or grammar interpretation.
//! - AST construction.
//! - Name resolution or type analysis.
//! - Semantic validation rules.
//! - Domain-specific lint rules.
//! - Diagnostic identity, severity, categories, or source spans.
//! - Diagnostic ordering or deduplication algorithms.
//! - Diagnostic rendering algorithms.
//! - Formatting or source modification.
//! - Compilation, code generation, linking, or artifact publication.
//! - Target discovery, hardware allocation, or runtime execution.
//!
//! Those responsibilities remain with their canonical repository
//! authorities.
//!
//! ## Canonical integrations
//!
//! Source request:
//!     crate::compiler::api
//!
//! Canonical diagnostics:
//!     crate::frontend::semantic::diagnostics
//!
//! Diagnostic aggregation:
//!     crate::compiler::diagnostics::aggregation
//!
//! Diagnostic JSON representation:
//!     crate::compiler::diagnostics::reporting
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! Toolchain results:
//!     crate::toolchain::result
//!
//! Normative language and diagnostic contracts:
//!     grammar/DESIGN.md
//!     grammar/Zamani.g4
//!     grammar/spec/diagnostics.md
//!     grammar/spec/source-spans.md
//!     grammar/specification/syntax-model.md
//!     grammar/validation/ambiguity.md
//!     grammar/validation/precedence.md
//!
//! ## Analysis contract
//!
//! A LintService implementation must use the canonical Zamani frontend
//! and the appropriate semantic/domain analysis services.
//!
//! It must not independently reinterpret the grammar or duplicate
//! semantic rules.
//!
//! A provider may report diagnostics from any registered phase or
//! computational domain through the canonical Diagnostic type.
//!
//! A provider must explicitly report incomplete analysis. Failure to
//! complete a required analysis must never be presented as a clean
//! lint result.
//!
//! ## Report-only contract
//!
//! This command may report:
//!
//! - Diagnostic messages.
//! - Source locations and labels.
//! - Notes and help.
//! - Proposed fix-its.
//! - Analysis completeness.
//! - Diagnostic aggregation statistics.
//!
//! It must never:
//!
//! - Apply a fix-it.
//! - Rewrite or format source.
//! - Modify project files.
//! - Modify compiler configuration.
//! - Execute suggested commands.
//! - Change IR or generated artifacts.
//! - Allocate hardware.
//! - Schedule or execute a program.
//!
//! ## POCO-REAF and scalability
//!
//! Linting is independent of a particular physical target.
//!
//! The provider architecture supports diagnostics from classical,
//! quantum, HDL, AI, tensor, hybrid, distributed, networking,
//! cryptographic, embedded, accelerator, and future domains.
//!
//! No closed enumeration of supported domains is required.
//!
//! No universal maximum is imposed on source size, diagnostic count,
//! domain count, target count, processor count, accelerator count,
//! qubit count, tensor rank, or distributed participant count.
//!
//! Diagnostic retention may use an explicit configurable operational
//! budget. If diagnostics are dropped, the resulting report is marked
//! incomplete and cannot be reported as a successful clean analysis.
//!
//! Actual execution remains subject to available memory, processing
//! resources, provider capabilities, and configured operational policy.
//! Literal infinite computation or storage is not promised.
//!
//! ## Concurrency
//!
//! This module does not create a worker pool or impose a fixed worker
//! count. A provider may perform analysis using its own approved
//! concurrency model.
//!
//! Providers must transfer diagnostics through the supplied sink.
//! The canonical aggregator is owned by the command invocation and
//! is not internally shared between workers.
//!
//! Parallel providers should synchronize their own diagnostic emission
//! before calling the sink.
//!
//! ## Failure semantics
//!
//! Invalid requests, invalid diagnostic records, provider failures,
//! and diagnostic aggregation failures remain errors.
//!
//! An incomplete analysis or incomplete diagnostic aggregate is not
//! a clean result.
//!
//! A warning is a finding. It becomes a lint violation only when the
//! request's warning policy requires warnings to be denied.
//!
//! No successful result is fabricated when a provider fails.

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use std::io::Write;

use crate::compiler::api::{
    CompilationRequest,
    CompilerConfig,
    SourceInput,
};

use crate::compiler::diagnostics::{
    AggregatedDiagnostics,
    AggregationError,
    AggregationPolicy,
    Diagnostic,
    DiagnosticAggregator,
    DiagnosticReport,
    DiagnosticSeverity,
    ReportSummary,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::result::ToolchainResult;

/// Stable schema identifier for a toolchain lint report.
pub const LINT_REPORT_SCHEMA: &str =
    "zamani.toolchain.lint-report.v1";

// ============================================================================
// REQUEST
// ============================================================================

/// A validated request to lint one source input.
///
/// The request reuses the canonical compiler request and configuration.
/// It does not introduce another source representation or compiler
/// configuration model.
#[derive(Debug)]
pub struct LintRequest {
    compilation: CompilationRequest,
    options: LintOptions,
}

impl LintRequest {
    /// Construct a lint request from a source input and compiler
    /// configuration.
    ///
    /// The source name or path must be non-empty.
    ///
    /// Empty source text is permitted. The canonical frontend decides
    /// whether that source is syntactically valid.
    ///
    /// # Errors
    ///
    /// Returns an invalid-input error when the source identity is empty.
    pub fn new(
        source: SourceInput,
        config: CompilerConfig,
    ) -> ToolchainResult<Self> {
        validate_source_input(&source)?;

        Ok(Self {
            compilation: CompilationRequest::new(source, config),
            options: LintOptions::default(),
        })
    }

    /// Construct a lint request from an existing canonical compilation
    /// request.
    ///
    /// This allows callers to reuse an already prepared source and
    /// configuration without creating a second request model.
    ///
    /// # Errors
    ///
    /// Returns an invalid-input error when the source identity is empty.
    pub fn from_compilation(
        compilation: CompilationRequest,
    ) -> ToolchainResult<Self> {
        validate_source_input(&compilation.source)?;

        Ok(Self {
            compilation,
            options: LintOptions::default(),
        })
    }

    /// Replace the lint options.
    #[must_use]
    pub fn with_options(
        mut self,
        options: LintOptions,
    ) -> Self {
        self.options = options;
        self
    }

    /// Borrow the canonical compilation request.
    #[must_use]
    pub fn compilation(&self) -> &CompilationRequest {
        &self.compilation
    }

    /// Borrow the lint options.
    #[must_use]
    pub const fn options(&self) -> &LintOptions {
        &self.options
    }

    /// Return the logical source name.
    #[must_use]
    pub fn source_name(&self) -> String {
        self.compilation.source.source_name_owned()
    }

    /// Consume the lint request and return its canonical compilation
    /// request.
    #[must_use]
    pub fn into_compilation(self) -> CompilationRequest {
        self.compilation
    }
}

/// Validate source identity without interpreting source contents.
fn validate_source_input(
    source: &SourceInput,
) -> ToolchainResult<()> {
    match source {
        SourceInput::Text { name, .. } => {
            if name.trim().is_empty() {
                return Err(
                    ToolchainError::invalid_input(
                        "lint source name must not be empty",
                    )
                    .with_operation("lint.request.validate"),
                );
            }
        }

        SourceInput::File { path } => {
            if path.as_os_str().is_empty() {
                return Err(
                    ToolchainError::invalid_input(
                        "lint source path must not be empty",
                    )
                    .with_operation("lint.request.validate"),
                );
            }
        }
    }

    Ok(())
}

// ============================================================================
// OPTIONS
// ============================================================================

/// Immutable policy for one lint operation.
///
/// The aggregation policy is the existing compiler diagnostic
/// retention policy. This module does not define a competing
/// diagnostic collection budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LintOptions {
    aggregation_policy: AggregationPolicy,
    deny_warnings: bool,
}

impl Default for LintOptions {
    fn default() -> Self {
        Self {
            aggregation_policy: AggregationPolicy::unlimited(),
            deny_warnings: false,
        }
    }
}

impl LintOptions {
    /// Create default lint options.
    ///
    /// The default retains diagnostics without an explicitly configured
    /// diagnostic-count ceiling. This does not promise infinite memory.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            aggregation_policy: AggregationPolicy::unlimited(),
            deny_warnings: false,
        }
    }

    /// Replace the diagnostic aggregation policy.
    ///
    /// A retention budget is an operational safeguard. If the budget
    /// causes diagnostics to be dropped, the report becomes incomplete.
    #[must_use]
    pub const fn with_aggregation_policy(
        mut self,
        policy: AggregationPolicy,
    ) -> Self {
        self.aggregation_policy = policy;
        self
    }

    /// Configure whether warning diagnostics count as lint violations.
    ///
    /// Warnings remain visible in the report regardless of this setting.
    #[must_use]
    pub const fn with_deny_warnings(
        mut self,
        enabled: bool,
    ) -> Self {
        self.deny_warnings = enabled;
        self
    }

    /// Return the configured aggregation policy.
    #[must_use]
    pub const fn aggregation_policy(self) -> AggregationPolicy {
        self.aggregation_policy
    }

    /// Whether warnings are treated as violations.
    #[must_use]
    pub const fn denies_warnings(self) -> bool {
        self.deny_warnings
    }
}

// ============================================================================
// PROVIDER CONTRACT
// ============================================================================

/// Completion state returned by the lint-analysis provider.
///
/// `Incomplete` means that one or more required analysis operations
/// could not be completed. A report with this state must never be
/// treated as a successful clean lint result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LintAnalysisStatus {
    /// All analysis required by the provider's declared contract completed.
    Complete,

    /// Required analysis could not be completed.
    Incomplete,
}

impl LintAnalysisStatus {
    /// Return the stable machine-readable status.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Incomplete => "incomplete",
        }
    }

    /// Whether the provider completed its required analysis.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Receives canonical diagnostic records from lint-analysis providers.
///
/// Implementations must not modify source code or execute fix-it
/// suggestions. The sink accepts diagnostic data only.
pub trait DiagnosticSink {
    /// Submit one canonical diagnostic.
    ///
    /// The diagnostic must have a valid, non-empty stable code.
    ///
    /// # Errors
    ///
    /// Returns an error if the diagnostic is invalid or the
    /// diagnostic infrastructure cannot account for the submission.
    fn emit(
        &mut self,
        diagnostic: Diagnostic,
    ) -> ToolchainResult<()>;
}

/// Contract for an implementation that performs Zamani lint analysis.
///
/// This is an integration boundary, not an implementation of the
/// language's lint rules.
///
/// A production implementation must:
///
/// - Use the canonical frontend and semantic analysis authorities.
/// - Preserve the normative grammar and language specifications.
/// - Use canonical source spans.
/// - Emit canonical frontend Diagnostic records.
/// - Preserve stable diagnostic codes and provenance.
/// - Report incomplete analysis explicitly.
/// - Propagate failures from required analysis stages.
/// - Avoid duplicating parser, semantic, or domain validation rules.
/// - Avoid compiling or publishing artifacts merely to produce lint data.
/// - Never modify source files or execute proposed fixes.
///
/// The contract is open to new diagnostic phases and computational
/// domains. It does not enumerate classical, quantum, HDL, AI, hybrid,
/// or distributed rules in a closed list.
pub trait LintService: Send + Sync {
    /// Perform lint analysis and stream diagnostics into the supplied sink.
    ///
    /// The provider receives the canonical source and configuration
    /// request. It is responsible for invoking the appropriate existing
    /// frontend and analysis services.
    ///
    /// The provider must return `Incomplete` if required analysis could
    /// not be completed, even when it has already emitted diagnostics.
    ///
    /// # Errors
    ///
    /// Returns the original structured toolchain error when required
    /// analysis or provider infrastructure fails.
    fn lint(
        &self,
        request: &CompilationRequest,
        options: &LintOptions,
        diagnostics: &mut dyn DiagnosticSink,
    ) -> ToolchainResult<LintAnalysisStatus>;
}

// ============================================================================
// DIAGNOSTIC SINK ADAPTER
// ============================================================================

/// Adapts the canonical compiler aggregator to the lint provider contract.
///
/// This adapter is invocation-local. It does not use global state or
/// introduce a second diagnostic collection.
struct AggregatingDiagnosticSink<'a> {
    aggregator: &'a mut DiagnosticAggregator,
    failure: Option<ToolchainError>,
}

impl<'a> AggregatingDiagnosticSink<'a> {
    /// Create a sink over one invocation's diagnostic aggregator.
    fn new(aggregator: &'a mut DiagnosticAggregator) -> Self {
        Self {
            aggregator,
            failure: None,
        }
    }
}

impl DiagnosticSink for AggregatingDiagnosticSink<'_> {
    fn emit(
        &mut self,
        diagnostic: Diagnostic,
    ) -> ToolchainResult<()> {
        // Once the sink has failed, preserve and return the original
        // failure instead of accepting further diagnostic submissions.
        if let Some(error) = self.failure.as_ref() {
            return Err(error.clone());
        }

        if !diagnostic.has_valid_code() {
            let error = ToolchainError::validation_failed(
                "lint provider emitted a diagnostic with an empty code",
            )
            .with_operation("lint.diagnostics.emit");

            self.failure = Some(error.clone());

            return Err(error);
        }

        match self.aggregator.push(diagnostic) {
            Ok(_) => Ok(()),

            Err(source) => {
                let error = aggregation_error(source);

                self.failure = Some(error.clone());

                Err(error)
            }
        }
    }
}

/// Convert a compiler aggregation infrastructure failure into a
/// toolchain-level error while preserving its original cause.
fn aggregation_error(
    source: AggregationError,
) -> ToolchainError {
    ToolchainError::resource_exhausted(
        "lint diagnostic aggregation could not complete",
    )
    .with_operation("lint.diagnostics.aggregate")
    .with_context(
        "The diagnostic aggregation infrastructure failed; \
         the lint result must not be treated as complete.",
    )
    .with_source(source)
}

// ============================================================================
// REPORT
// ============================================================================

/// Final status of one lint operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LintStatus {
    /// Analysis completed and no diagnostics were retained.
    Clean,

    /// Analysis completed and one or more diagnostics were retained.
    Findings,

    /// Required analysis or diagnostic aggregation was incomplete.
    Incomplete,
}

impl LintStatus {
    /// Return the stable machine-readable status.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::Findings => "findings",
            Self::Incomplete => "incomplete",
        }
    }
}

/// Machine-readable result of one lint operation.
///
/// The report retains the canonical aggregated diagnostic records.
/// It does not create a second diagnostic model.
///
/// JSON serialization embeds the compiler's canonical diagnostic
/// report, including its schema version, completeness status,
/// summary, source spans, labels, notes, help and fix-it suggestions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintReport {
    source_name: String,
    analysis_status: LintAnalysisStatus,
    deny_warnings: bool,
    aggregate: AggregatedDiagnostics,
}

impl LintReport {
    /// Construct a report from a completed command invocation.
    fn new(
        source_name: String,
        analysis_status: LintAnalysisStatus,
        deny_warnings: bool,
        aggregate: AggregatedDiagnostics,
    ) -> Self {
        Self {
            source_name,
            analysis_status,
            deny_warnings,
            aggregate,
        }
    }

    /// Return the lint report schema identifier.
    #[must_use]
    pub const fn schema() -> &'static str {
        LINT_REPORT_SCHEMA
    }

    /// Return the source name.
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    /// Return the provider's analysis completion state.
    #[must_use]
    pub const fn analysis_status(&self) -> LintAnalysisStatus {
        self.analysis_status
    }

    /// Return the final lint status.
    ///
    /// Incomplete analysis or incomplete aggregation always takes
    /// precedence over the presence or absence of findings.
    #[must_use]
    pub fn status(&self) -> LintStatus {
        if !self.analysis_status.is_complete()
            || !self.aggregate.is_complete()
        {
            return LintStatus::Incomplete;
        }

        if self.aggregate.is_empty() {
            LintStatus::Clean
        } else {
            LintStatus::Findings
        }
    }

    /// Whether the diagnostic aggregate is complete.
    #[must_use]
    pub const fn diagnostics_complete(&self) -> bool {
        self.aggregate.is_complete()
    }

    /// Whether the provider completed required analysis.
    #[must_use]
    pub const fn analysis_complete(&self) -> bool {
        self.analysis_status.is_complete()
    }

    /// Whether at least one diagnostic was retained.
    #[must_use]
    pub fn has_findings(&self) -> bool {
        !self.aggregate.is_empty()
    }

    /// Whether the report contains a lint violation.
    ///
    /// Error and fatal diagnostics are always violations.
    /// Warning diagnostics are violations only when `deny_warnings`
    /// is enabled.
    ///
    /// Notes, hints and informational diagnostics remain reportable
    /// but are not violations under this policy.
    #[must_use]
    pub fn has_violations(&self) -> bool {
        self.aggregate.iter().any(|diagnostic| {
            diagnostic.is_error()
                || (self.deny_warnings
                    && diagnostic.severity()
                        == DiagnosticSeverity::Warning)
        })
    }

    /// Whether the lint operation completed successfully.
    ///
    /// Success requires complete analysis, complete diagnostic
    /// aggregation and no violations under the configured policy.
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.analysis_complete()
            && self.diagnostics_complete()
            && !self.has_violations()
    }

    /// Whether the report contains no diagnostics and is complete.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.status() == LintStatus::Clean
    }

    /// Borrow diagnostics in canonical reporting order.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        self.aggregate.diagnostics()
    }

    /// Borrow the complete canonical aggregate.
    #[must_use]
    pub const fn aggregate(&self) -> &AggregatedDiagnostics {
        &self.aggregate
    }

    /// Return the canonical diagnostic summary.
    #[must_use]
    pub fn summary(&self) -> ReportSummary {
        DiagnosticReport::new(&self.aggregate).summary()
    }

    /// Return a borrowed view of the canonical diagnostic report.
    ///
    /// This allows consumers to use the existing compiler reporting
    /// API without copying the diagnostic collection.
    #[must_use]
    pub fn diagnostic_report(&self) -> DiagnosticReport<'_> {
        DiagnosticReport::new(&self.aggregate)
    }

    /// Serialize the report directly to a caller-provided writer.
    ///
    /// This is the preferred method for large diagnostic reports because
    /// it avoids constructing an additional complete JSON string.
    ///
    /// If serialization fails, the writer may contain partial output.
    /// The caller must not treat partial output as a complete report.
    ///
    /// # Errors
    ///
    /// Returns a structured toolchain error when serialization or
    /// writing fails.
    pub fn write_json<W>(
        &self,
        writer: W,
    ) -> ToolchainResult<()>
    where
        W: Write,
    {
        serde_json::to_writer(writer, self).map_err(|source| {
            ToolchainError::new(
                ToolchainErrorKind::Io,
                "failed to write the lint report",
            )
            .with_operation("lint.report.write_json")
            .with_source(source)
        })
    }

    /// Serialize the report into an owned JSON string.
    ///
    /// This allocates memory proportional to the serialized report.
    /// Prefer `write_json` when processing large reports.
    ///
    /// # Errors
    ///
    /// Returns a structured toolchain error when serialization fails.
    pub fn to_json_string(&self) -> ToolchainResult<String> {
        serde_json::to_string(self).map_err(|source| {
            ToolchainError::provider_failed(
                "failed to serialize the lint report",
            )
            .with_operation("lint.report.serialize")
            .with_source(source)
        })
    }
}

/// Stable serialization of a lint report.
///
/// The canonical diagnostic records are serialized by
/// `compiler::diagnostics::reporting::DiagnosticReport`.
///
/// This implementation only adds lint-command metadata around that
/// existing report. It does not serialize diagnostic fields independently.
impl Serialize for LintReport {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state =
            serializer.serialize_struct("LintReport", 7)?;

        state.serialize_field(
            "schema_version",
            LINT_REPORT_SCHEMA,
        )?;

        state.serialize_field(
            "source_name",
            &self.source_name,
        )?;

        state.serialize_field(
            "status",
            self.status().as_str(),
        )?;

        state.serialize_field(
            "analysis_status",
            self.analysis_status.as_str(),
        )?;

        state.serialize_field(
            "success",
            &self.is_success(),
        )?;

        state.serialize_field(
            "has_violations",
            &self.has_violations(),
        )?;

        state.serialize_field(
            "diagnostics",
            &DiagnosticReport::new(&self.aggregate),
        )?;

        state.end()
    }
}

// ============================================================================
// COMMAND
// ============================================================================

/// Toolchain lint command.
///
/// The lint implementation is injected through `LintService`.
/// This keeps the command independent of a concrete frontend,
/// semantic pipeline, or domain-specific lint implementation.
pub struct LintCommand<L> {
    service: L,
}

impl<L> LintCommand<L>
where
    L: LintService,
{
    /// Create a lint command using the supplied analysis service.
    #[must_use]
    pub const fn new(service: L) -> Self {
        Self { service }
    }

    /// Borrow the lint-analysis service.
    #[must_use]
    pub const fn service(&self) -> &L {
        &self.service
    }

    /// Lint one source request.
    ///
    /// The command:
    ///
    /// 1. Uses the validated canonical compilation request.
    /// 2. Creates the existing compiler diagnostic aggregator.
    /// 3. Supplies a streaming diagnostic sink to the provider.
    /// 4. Preserves provider and aggregation failures.
    /// 5. Finalizes the canonical diagnostic aggregate.
    /// 6. Returns a structured lint report.
    ///
    /// This method does not invoke `CompilerService::compile`.
    /// It therefore does not claim to compile, generate, link,
    /// publish, or execute an artifact.
    ///
    /// # Errors
    ///
    /// Returns the original provider error, an invalid diagnostic
    /// error, or a diagnostic aggregation infrastructure error.
    pub fn lint(
        &self,
        request: LintRequest,
    ) -> ToolchainResult<LintReport> {
        let source_name = request.source_name();
        let options = request.options;

        let mut aggregator =
            DiagnosticAggregator::new(options.aggregation_policy());

        let provider_status = {
            let mut sink =
                AggregatingDiagnosticSink::new(&mut aggregator);

            let provider_result = self.service.lint(
                &request.compilation,
                &options,
                &mut sink,
            );

            // A provider must not be able to hide a sink failure by
            // ignoring the error returned from DiagnosticSink::emit.
            if let Some(error) = sink.failure.take() {
                return Err(error);
            }

            provider_result.map_err(|error| {
                error
                    .with_operation("lint.analyze")
                    .with_context(
                        "The lint provider did not complete its \
                         requested analysis successfully.",
                    )
            })?
        };

        let aggregate = aggregator.finish();

        Ok(LintReport::new(
            source_name,
            provider_status,
            options.denies_warnings(),
            aggregate,
        ))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::semantic::diagnostics::{
        DiagnosticCategory,
        DiagnosticPhase,
    };

    use std::num::NonZeroUsize;

    /// Legacy compiler configuration is used only to construct
    /// isolated command-contract tests. Production callers should
    /// supply their canonical explicit compiler configuration.
    #[allow(deprecated)]
    fn test_config() -> CompilerConfig {
        CompilerConfig::default()
    }

    fn request() -> LintRequest {
        LintRequest::new(
            SourceInput::text(
                "fn main() {}",
                "main.zm",
            ),
            test_config(),
        )
        .expect("test lint request should be valid")
    }

    fn warning(code: &str) -> Diagnostic {
        Diagnostic::new(
            code,
            DiagnosticPhase::DomainAnalysis,
            DiagnosticCategory::Extension(
                String::from("lint"),
            ),
            DiagnosticSeverity::Warning,
            "test lint warning",
            None,
        )
    }

    fn error(code: &str) -> Diagnostic {
        Diagnostic::new(
            code,
            DiagnosticPhase::TypeAnalysis,
            DiagnosticCategory::Type,
            DiagnosticSeverity::Error,
            "test lint error",
            None,
        )
    }

    #[derive(Debug)]
    struct MockLintService {
        diagnostics: Vec<Diagnostic>,
        status: LintAnalysisStatus,
        fail: bool,
    }

    impl MockLintService {
        fn new(
            diagnostics: Vec<Diagnostic>,
        ) -> Self {
            Self {
                diagnostics,
                status: LintAnalysisStatus::Complete,
                fail: false,
            }
        }

        fn incomplete(mut self) -> Self {
            self.status = LintAnalysisStatus::Incomplete;
            self
        }

        fn failing(mut self) -> Self {
            self.fail = true;
            self
        }
    }

    impl LintService for MockLintService {
        fn lint(
            &self,
            _request: &CompilationRequest,
            _options: &LintOptions,
            diagnostics: &mut dyn DiagnosticSink,
        ) -> ToolchainResult<LintAnalysisStatus> {
            if self.fail {
                return Err(ToolchainError::provider_failed(
                    "mock lint provider failure",
                ));
            }

            for diagnostic in self.diagnostics.iter().cloned() {
                diagnostics.emit(diagnostic)?;
            }

            Ok(self.status)
        }
    }

    #[test]
    fn accepts_empty_source_text_with_a_name() {
        let request = LintRequest::new(
            SourceInput::text("", "empty.zm"),
            test_config(),
        );

        assert!(request.is_ok());
    }

    #[test]
    fn rejects_empty_source_name() {
        let result = LintRequest::new(
            SourceInput::text("source", ""),
            test_config(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_whitespace_source_name() {
        let result = LintRequest::new(
            SourceInput::text("source", "   "),
            test_config(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_file_path() {
        let result = LintRequest::new(
            SourceInput::file(std::path::PathBuf::new()),
            test_config(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn accepts_nonempty_file_path() {
        let result = LintRequest::new(
            SourceInput::file("main.zm"),
            test_config(),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn empty_complete_analysis_is_clean() {
        let command =
            LintCommand::new(MockLintService::new(Vec::new()));

        let report = command
            .lint(request())
            .expect("lint operation should succeed");

        assert_eq!(report.status(), LintStatus::Clean);
        assert!(report.is_clean());
        assert!(report.is_success());
        assert!(!report.has_findings());
        assert!(!report.has_violations());
    }

    #[test]
    fn warning_is_reported_without_being_denied_by_default() {
        let command = LintCommand::new(
            MockLintService::new(vec![
                warning("test.warning"),
            ]),
        );

        let report = command
            .lint(request())
            .expect("lint operation should succeed");

        assert_eq!(report.status(), LintStatus::Findings);
        assert!(report.has_findings());
        assert!(!report.has_violations());
        assert!(report.is_success());
    }

    #[test]
    fn warning_can_be_configured_as_a_violation() {
        let command = LintCommand::new(
            MockLintService::new(vec![
                warning("test.warning"),
            ]),
        );

        let request = request().with_options(
            LintOptions::new().with_deny_warnings(true),
        );

        let report = command
            .lint(request)
            .expect("lint analysis should complete");

        assert!(report.has_violations());
        assert!(!report.is_success());
    }

    #[test]
    fn error_diagnostic_is_always_a_violation() {
        let command = LintCommand::new(
            MockLintService::new(vec![
                error("test.error"),
            ]),
        );

        let report = command
            .lint(request())
            .expect("lint analysis should complete");

        assert!(report.has_violations());
        assert!(!report.is_success());
    }

    #[test]
    fn incomplete_provider_analysis_is_never_clean() {
        let command = LintCommand::new(
            MockLintService::new(Vec::new()).incomplete(),
        );

        let report = command
            .lint(request())
            .expect("provider may report incomplete analysis");

        assert_eq!(report.status(), LintStatus::Incomplete);
        assert!(!report.analysis_complete());
        assert!(!report.is_success());
        assert!(!report.is_clean());
    }

    #[test]
    fn diagnostic_retention_budget_marks_report_incomplete() {
        let policy = AggregationPolicy::with_limit(
            NonZeroUsize::new(1)
                .expect("one is nonzero"),
        );

        let command = LintCommand::new(
            MockLintService::new(vec![
                warning("test.warning.one"),
                warning("test.warning.two"),
            ]),
        );

        let request = request().with_options(
            LintOptions::new()
                .with_aggregation_policy(policy),
        );

        let report = command
            .lint(request)
            .expect("budget exhaustion is reported in the aggregate");

        assert_eq!(report.status(), LintStatus::Incomplete);
        assert!(!report.diagnostics_complete());
        assert_eq!(report.diagnostics().len(), 1);
        assert_eq!(
            report.aggregate().statistics().dropped(),
            1
        );
        assert!(!report.is_success());
    }

    #[test]
    fn provider_failure_is_not_converted_into_a_clean_report() {
        let command = LintCommand::new(
            MockLintService::new(Vec::new()).failing(),
        );

        let result = command.lint(request());

        assert!(result.is_err());
    }

    #[test]
    fn invalid_diagnostic_code_is_rejected() {
        let invalid = Diagnostic::new(
            "",
            DiagnosticPhase::DomainAnalysis,
            DiagnosticCategory::Extension(
                String::from("lint"),
            ),
            DiagnosticSeverity::Warning,
            "invalid diagnostic",
            None,
        );

        let command = LintCommand::new(
            MockLintService::new(vec![invalid]),
        );

        let result = command.lint(request());

        assert!(result.is_err());
    }

    #[test]
    fn report_schema_is_versioned() {
        assert_eq!(
            LINT_REPORT_SCHEMA,
            "zamani.toolchain.lint-report.v1"
        );
    }

    #[test]
    fn report_serializes_lint_and_canonical_diagnostic_metadata() {
        let command = LintCommand::new(
            MockLintService::new(vec![
                warning("test.warning"),
            ]),
        );

        let report = command
            .lint(request())
            .expect("lint operation should succeed");

        let json = report
            .to_json_string()
            .expect("lint report should serialize");

        assert!(json.contains(LINT_REPORT_SCHEMA));
        assert!(json.contains("main.zm"));
        assert!(json.contains("diagnostics"));
        assert!(json.contains("test.warning"));
        assert!(json.contains("findings"));
    }

    #[test]
    fn report_exposes_canonical_summary() {
        let command = LintCommand::new(
            MockLintService::new(vec![
                warning("test.warning.one"),
                warning("test.warning.two"),
            ]),
        );

        let report = command
            .lint(request())
            .expect("lint operation should succeed");

        assert_eq!(report.summary().warnings(), 2);
        assert_eq!(report.summary().errors(), 0);
        assert_eq!(report.summary().fatal(), 0);
    }

    #[test]
    fn diagnostic_report_reuses_compiler_reporting_authority() {
        let command = LintCommand::new(
            MockLintService::new(vec![
                warning("test.warning"),
            ]),
        );

        let report = command
            .lint(request())
            .expect("lint operation should succeed");

        assert_eq!(
            report.diagnostic_report().diagnostics().len(),
            1
        );

        assert!(report.diagnostic_report().is_complete());
    }
}
