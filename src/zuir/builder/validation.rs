
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Builder Validation Framework
//!
//! Canonical path: `src/zuir/builder/validation.rs`
//! Edition: Rust 2021
//! Minimum intended Rust version: 1.97 or later.
//! Safety: unsafe Rust is forbidden.
//!
//! STATUS
//!
//! Proposed foundational module. This file was not present on the inspected
//! repository default branch and must be introduced as a new module.
//!
//! PURPOSE
//!
//! Provide a deterministic, extensible framework for validation performed
//! during construction of ZUIR structures. Builder modules register focused
//! rules and run them against inputs whose types are chosen by the caller.
//!
//! OWNS
//!
//! - ValidationContext, ValidationPhase, and ValidationMode.
//! - ValidationSeverity and ValidationIssue.
//! - ValidationReport.
//! - The ValidationRule extension contract.
//! - ValidationPipeline and its rule-registration order.
//! - Framework-level errors, including allocation and rule failures.
//!
//! DOES NOT OWN
//!
//! - Canonical IR data structures or identifier allocation.
//! - Operation, block, region, type, effect, ownership, or CFG semantics.
//! - Dialect registration or extension-code execution.
//! - Source parsing, AST construction, name resolution, or type inference.
//! - The authoritative whole-IR verifier.
//! - Target capability discovery, physical resource allocation, or lowering.
//! - Diagnostic rendering, terminal output, or logging policy.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/dialect/operation.rs`
//! - `src/zuir/dialect/registry.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/compatibility.md`
//!
//! INPUT CONTRACT
//!
//! A validation rule receives an immutable input and a context. It may report
//! zero or more issues, or return a rule-execution failure. Rules must not
//! mutate the input or rely on rule execution order for semantic correctness.
//!
//! OUTPUT CONTRACT
//!
//! A successful pipeline execution returns a report, which may contain
//! errors and warnings. `Ok(report)` does NOT mean the input is valid unless
//! `report.has_errors()` is false. Framework failures are returned separately.
//!
//! DETERMINISM
//!
//! Rules execute sequentially in registration order. Issues preserve rule
//! order and each rule's issue order. This module does not run rules in
//! parallel or reorder diagnostics.
//!
//! SCALABILITY
//!
//! There is no universal maximum rule count, issue count, operation count,
//! region count, qubit count, processor count, device count, tensor rank, or
//! graph size. Dynamic collections grow subject to available resources.
//! Principal framework vectors use fallible reservation before appending.
//! This does not make every allocation in Rust fallible or promise literal
//! infinity. Resource exhaustion is reported where the framework can detect it.
//!
//! TRUST BOUNDARY
//!
//! A builder pipeline is not a substitute for whole-IR verification. Local
//! checks may establish construction preconditions, but reference resolution,
//! global ID uniqueness, SSA, dominance, CFG consistency, effect legality,
//! type compatibility, ownership, and dialect-specific validity remain the
//! responsibility of their authoritative verifiers.
//!
//! FREEZE CRITERIA
//!
//! - This module is declared by `builder/mod.rs`.
//! - Public rule and report contracts are used consistently by builders.
//! - Unit and integration tests pass.
//! - No duplicate authoritative verifier or diagnostic type is introduced.
//! - Public APIs and error behavior are documented and reviewed.

use std::error::Error;
use std::fmt;
use std::sync::Arc;

/// Result type for validation framework operations.
pub type ValidationResult<T> = Result<T, ValidationFrameworkError>;

/// Stage at which a validation pipeline is being used.
///
/// The phase is descriptive policy context. It does not imply that the
/// framework itself implements any particular semantic verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValidationPhase {
    /// Checks needed while constructing one local IR object.
    BuilderLocal,

    /// Checks against a declarative schema or registered contract.
    Schema,

    /// Checks that require relationships among multiple IR objects.
    WholeIr,

    /// Checks performed before converting to another representation.
    Lowering,

    /// Checks performed when importing or decoding an artifact.
    Interchange,
}

/// Controls whether validation stops after the first error.
///
/// Warnings and notes do not trigger fail-fast termination.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationMode {
    /// Stop processing further rules after an error is reported.
    FailFast,

    /// Run all registered rules and collect their issues.
    CollectAll,
}

/// Immutable context shared by rules in one pipeline invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValidationContext {
    phase: ValidationPhase,
    mode: ValidationMode,
}

impl ValidationContext {
    /// Construct a validation context.
    #[must_use]
    pub const fn new(phase: ValidationPhase, mode: ValidationMode) -> Self {
        Self { phase, mode }
    }

    /// Return the current validation phase.
    #[must_use]
    pub const fn phase(self) -> ValidationPhase {
        self.phase
    }

    /// Return the selected issue-collection mode.
    #[must_use]
    pub const fn mode(self) -> ValidationMode {
        self.mode
    }
}

impl Default for ValidationContext {
    fn default() -> Self {
        Self::new(
            ValidationPhase::BuilderLocal,
            ValidationMode::CollectAll,
        )
    }
}

/// Severity of a validation issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValidationSeverity {
    /// The input violates a required contract.
    Error,

    /// The input is accepted by this rule, but the caller should review it.
    Warning,

    /// Informational validation output.
    Note,
}

/// One structured validation issue.
///
/// `code` is a stable machine-readable identifier owned by the rule author.
/// `path` is a sequence of logical path segments, not a filesystem path.
/// The framework does not interpret either field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    severity: ValidationSeverity,
    code: String,
    message: String,
    path: Vec<String>,
    rule: Option<String>,
}

impl ValidationIssue {
    /// Create an issue with a stable code and human-readable message.
    #[must_use]
    pub fn new(
        severity: ValidationSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code: code.into(),
            message: message.into(),
            path: Vec::new(),
            rule: None,
        }
    }

    /// Create an error issue.
    #[must_use]
    pub fn error(
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(ValidationSeverity::Error, code, message)
    }

    /// Create a warning issue.
    #[must_use]
    pub fn warning(
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(ValidationSeverity::Warning, code, message)
    }

    /// Create an informational issue.
    #[must_use]
    pub fn note(
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(ValidationSeverity::Note, code, message)
    }

    /// Attach a logical path to this issue.
    #[must_use]
    pub fn with_path<I, S>(mut self, segments: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.path = segments.into_iter().map(Into::into).collect();
        self
    }

    /// Return issue severity.
    #[must_use]
    pub const fn severity(&self) -> ValidationSeverity {
        self.severity
    }

    /// Return the stable machine-readable issue code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Return the human-readable issue message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return logical path segments.
    #[must_use]
    pub fn path(&self) -> &[String] {
        &self.path
    }

    /// Return the registered rule name, if attached by the pipeline.
    #[must_use]
    pub fn rule(&self) -> Option<&str> {
        self.rule.as_deref()
    }

    fn attach_rule(&mut self, rule_name: &str) {
        self.rule = Some(rule_name.to_owned());
    }
}

/// Collected issues from one pipeline invocation.
///
/// A report can contain errors and still be a successful framework result.
/// Callers must inspect `has_errors()` before accepting the validated input.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ValidationReport {
    issues: Vec<ValidationIssue>,
}

impl ValidationReport {
    /// Construct an empty report.
    #[must_use]
    pub const fn new() -> Self {
        Self { issues: Vec::new() }
    }

    /// Return all issues in deterministic encounter order.
    #[must_use]
    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }

    /// Return the total number of issues.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Return whether the report contains an error.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.severity() == ValidationSeverity::Error)
    }

    /// Return the number of errors.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity() == ValidationSeverity::Error)
            .count()
    }

    /// Return the number of warnings.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity() == ValidationSeverity::Warning)
            .count()
    }

    /// Return the number of informational notes.
    #[must_use]
    pub fn note_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity() == ValidationSeverity::Note)
            .count()
    }

    /// Return whether the report has no issues of any severity.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.issues.is_empty()
    }

    /// Reserve space for additional issues.
    ///
    /// This is an optimization and an explicit way to observe reservation
    /// failure before appending. It is not a semantic limit on issue count.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> ValidationResult<()> {
        self.issues
            .try_reserve(additional)
            .map_err(|_| ValidationFrameworkError::AllocationFailure {
                collection: "validation report issues",
            })
    }

    /// Append one issue, reserving capacity before mutation.
    pub fn try_push(
        &mut self,
        issue: ValidationIssue,
    ) -> ValidationResult<()> {
        self.try_reserve(1)?;
        self.issues.push(issue);
        Ok(())
    }

    /// Append issues in the supplied order.
    ///
    /// If reservation fails, no issue from this batch is appended.
    pub fn try_extend<I>(&mut self, issues: I) -> ValidationResult<()>
    where
        I: IntoIterator<Item = ValidationIssue>,
    {
        let iterator = issues.into_iter();
        let (lower_bound, _) = iterator.size_hint();

        self.try_reserve(lower_bound)?;

        for issue in iterator {
            // Iterators are not required to yield exactly their lower-bound
            // count. Each push reserves as necessary before mutation.
            self.try_push(issue)?;
        }

        Ok(())
    }
}

/// Failure of the validation framework itself.
///
/// This is distinct from a validation issue: a rule can report that an
/// input is invalid without failing to execute.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValidationFrameworkError {
    /// A framework-owned collection could not reserve capacity.
    AllocationFailure {
        collection: &'static str,
    },

    /// A rule name is empty or contains only whitespace.
    EmptyRuleName,

    /// A rule name has already been registered in this pipeline.
    DuplicateRuleName(String),

    /// A registered rule could not complete its validation.
    RuleFailed {
        rule: String,
        reason: String,
    },
}

impl fmt::Display for ValidationFrameworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllocationFailure { collection } => {
                write!(f, "unable to reserve memory for {collection}")
            }
            Self::EmptyRuleName => {
                f.write_str("validation rule name must not be empty")
            }
            Self::DuplicateRuleName(name) => {
                write!(f, "validation rule `{name}` is already registered")
            }
            Self::RuleFailed { rule, reason } => {
                write!(f, "validation rule `{rule}` failed to execute: {reason}")
            }
        }
    }
}

impl Error for ValidationFrameworkError {}

/// A reusable validation rule.
///
/// Rules should be deterministic, side-effect-free with respect to `input`,
/// and limited to the responsibility identified by `name()`.
///
/// The returned vector preserves the rule's desired issue order. A rule
/// execution failure is not an ordinary validation issue and is propagated
/// as a framework error.
pub trait ValidationRule<T>: Send + Sync {
    /// Stable, nonempty name unique within one pipeline.
    fn name(&self) -> &str;

    /// Validate an immutable input and return issues in deterministic order.
    fn validate(
        &self,
        input: &T,
        context: &ValidationContext,
    ) -> Result<Vec<ValidationIssue>, String>;
}

struct RegisteredRule<T> {
    name: String,
    rule: Arc<dyn ValidationRule<T>>,
}

/// Ordered validation pipeline for an input type.
///
/// The pipeline owns rule registrations, not the input being validated.
/// Rules execute sequentially in registration order to keep diagnostics
/// deterministic and avoid requiring global mutable state.
pub struct ValidationPipeline<T> {
    rules: Vec<RegisteredRule<T>>,
}

impl<T> fmt::Debug for ValidationPipeline<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValidationPipeline")
            .field(
                "rules",
                &self
                    .rules
                    .iter()
                    .map(|registered| registered.name.as_str())
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl<T> Default for ValidationPipeline<T> {
    fn default() -> Self {
        Self { rules: Vec::new() }
    }
}

impl<T> ValidationPipeline<T> {
    /// Construct an empty pipeline.
    #[must_use]
    pub const fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Return the number of registered rules.
    #[must_use]
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Return whether no rules are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// Return registered rule names in execution order.
    #[must_use]
    pub fn rule_names(&self) -> Vec<&str> {
        self.rules
            .iter()
            .map(|registered| registered.name.as_str())
            .collect()
    }

    /// Reserve capacity for additional rule registrations.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> ValidationResult<()> {
        self.rules
            .try_reserve(additional)
            .map_err(|_| ValidationFrameworkError::AllocationFailure {
                collection: "validation pipeline rules",
            })
    }

    /// Register a rule, preserving insertion order.
    ///
    /// Duplicate names are rejected because they make diagnostic provenance
    /// ambiguous. Rule names should be stable identifiers, not localized text.
    pub fn try_add_rule<R>(&mut self, rule: R) -> ValidationResult<()>
    where
        R: ValidationRule<T> + 'static,
    {
        let name = rule.name().trim();

        if name.is_empty() {
            return Err(ValidationFrameworkError::EmptyRuleName);
        }

        if self.rules.iter().any(|existing| existing.name == name) {
            return Err(ValidationFrameworkError::DuplicateRuleName(
                name.to_owned(),
            ));
        }

        self.try_reserve(1)?;

        self.rules.push(RegisteredRule {
            name: name.to_owned(),
            rule: Arc::new(rule),
        });

        Ok(())
    }

    /// Register a shared rule implementation.
    ///
    /// This is useful when multiple pipelines intentionally share an
    /// immutable rule instance. The rule's name must remain stable.
    pub fn try_add_shared_rule(
        &mut self,
        rule: Arc<dyn ValidationRule<T>>,
    ) -> ValidationResult<()> {
        let name = rule.name().trim();

        if name.is_empty() {
            return Err(ValidationFrameworkError::EmptyRuleName);
        }

        if self.rules.iter().any(|existing| existing.name == name) {
            return Err(ValidationFrameworkError::DuplicateRuleName(
                name.to_owned(),
            ));
        }

        self.try_reserve(1)?;

        self.rules.push(RegisteredRule {
            name: name.to_owned(),
            rule,
        });

        Ok(())
    }

    /// Run registered rules against an immutable input.
    ///
    /// `Ok(report)` means every executed rule completed. The report may
    /// still contain errors. Callers must reject the input when the report
    /// contains an error.
    ///
    /// In `FailFast` mode, processing stops as soon as an error issue is
    /// appended. In `CollectAll` mode, every registered rule is executed.
    pub fn validate(
        &self,
        input: &T,
        context: &ValidationContext,
    ) -> ValidationResult<ValidationReport> {
        let mut report = ValidationReport::new();

        for registered in &self.rules {
            let issues = registered
                .rule
                .validate(input, context)
                .map_err(|reason| ValidationFrameworkError::RuleFailed {
                    rule: registered.name.clone(),
                    reason,
                })?;

            for mut issue in issues {
                issue.attach_rule(&registered.name);
                let is_error = issue.severity() == ValidationSeverity::Error;

                report.try_push(issue)?;

                if is_error && context.mode() == ValidationMode::FailFast {
                    return Ok(report);
                }
            }
        }

        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysValid;

    impl ValidationRule<u32> for AlwaysValid {
        fn name(&self) -> &str {
            "test.always_valid"
        }

        fn validate(
            &self,
            _input: &u32,
            _context: &ValidationContext,
        ) -> Result<Vec<ValidationIssue>, String> {
            Ok(Vec::new())
        }
    }

    struct MinimumValue {
        minimum: u32,
    }

    impl ValidationRule<u32> for MinimumValue {
        fn name(&self) -> &str {
            "test.minimum_value"
        }

        fn validate(
            &self,
            input: &u32,
            _context: &ValidationContext,
        ) -> Result<Vec<ValidationIssue>, String> {
            if *input >= self.minimum {
                return Ok(Vec::new());
            }

            Ok(vec![
                ValidationIssue::error(
                    "ZUIR.TEST.VALUE_BELOW_MINIMUM",
                    format!("value {input} is below {}", self.minimum),
                )
                .with_path(["input"]),
            ])
        }
    }

    struct AlwaysWarn;

    impl ValidationRule<u32> for AlwaysWarn {
        fn name(&self) -> &str {
            "test.always_warn"
        }

        fn validate(
            &self,
            _input: &u32,
            _context: &ValidationContext,
        ) -> Result<Vec<ValidationIssue>, String> {
            Ok(vec![ValidationIssue::warning(
                "ZUIR.TEST.WARNING",
                "test warning",
            )])
        }
    }

    struct RuleFailure;

    impl ValidationRule<u32> for RuleFailure {
        fn name(&self) -> &str {
            "test.rule_failure"
        }

        fn validate(
            &self,
            _input: &u32,
            _context: &ValidationContext,
        ) -> Result<Vec<ValidationIssue>, String> {
            Err("synthetic rule failure".to_owned())
        }
    }

    fn context(mode: ValidationMode) -> ValidationContext {
        ValidationContext::new(ValidationPhase::BuilderLocal, mode)
    }

    #[test]
    fn empty_pipeline_returns_empty_report() {
        let pipeline = ValidationPipeline::<u32>::new();
        let report = pipeline
            .validate(&5, &context(ValidationMode::CollectAll))
            .expect("empty pipeline should execute");

        assert!(report.is_empty());
        assert!(!report.has_errors());
    }

    #[test]
    fn valid_input_has_no_errors() {
        let mut pipeline = ValidationPipeline::new();
        pipeline
            .try_add_rule(AlwaysValid)
            .expect("rule registration");

        let report = pipeline
            .validate(&10, &context(ValidationMode::CollectAll))
            .expect("validation should execute");

        assert!(!report.has_errors());
        assert_eq!(report.issue_count(), 0);
    }

    #[test]
    fn errors_include_the_originating_rule() {
        let mut pipeline = ValidationPipeline::new();
        pipeline
            .try_add_rule(MinimumValue { minimum: 10 })
            .expect("rule registration");

        let report = pipeline
            .validate(&3, &context(ValidationMode::CollectAll))
            .expect("rule should report an issue");

        assert!(report.has_errors());
        assert_eq!(report.error_count(), 1);
        assert_eq!(
            report.issues()[0].rule(),
            Some("test.minimum_value")
        );
        assert_eq!(
            report.issues()[0].path(),
            &["input".to_owned()]
        );
    }

    #[test]
    fn collect_all_runs_later_rules_after_an_error() {
        let mut pipeline = ValidationPipeline::new();
        pipeline
            .try_add_rule(MinimumValue { minimum: 10 })
            .expect("first rule");
        pipeline
            .try_add_rule(AlwaysWarn)
            .expect("second rule");

        let report = pipeline
            .validate(&3, &context(ValidationMode::CollectAll))
            .expect("pipeline should complete");

        assert_eq!(report.error_count(), 1);
        assert_eq!(report.warning_count(), 1);
        assert_eq!(report.issues()[0].rule(), Some("test.minimum_value"));
        assert_eq!(report.issues()[1].rule(), Some("test.always_warn"));
    }

    #[test]
    fn fail_fast_stops_after_the_first_error() {
        let mut pipeline = ValidationPipeline::new();
        pipeline
            .try_add_rule(MinimumValue { minimum: 10 })
            .expect("first rule");
        pipeline
            .try_add_rule(AlwaysWarn)
            .expect("second rule");

        let report = pipeline
            .validate(&3, &context(ValidationMode::FailFast))
            .expect("pipeline should return its partial report");

        assert_eq!(report.error_count(), 1);
        assert_eq!(report.warning_count(), 0);
    }

    #[test]
    fn warnings_do_not_trigger_fail_fast() {
        let mut pipeline = ValidationPipeline::new();
        pipeline
            .try_add_rule(AlwaysWarn)
            .expect("warning rule");
        pipeline
            .try_add_rule(MinimumValue { minimum: 10 })
            .expect("error rule");

        let report = pipeline
            .validate(&3, &context(ValidationMode::FailFast))
            .expect("pipeline should execute");

        assert_eq!(report.warning_count(), 1);
        assert_eq!(report.error_count(), 1);
    }

    #[test]
    fn duplicate_rule_names_are_rejected() {
        let mut pipeline = ValidationPipeline::new();
        pipeline
            .try_add_rule(AlwaysValid)
            .expect("first registration");

        assert!(matches!(
            pipeline.try_add_rule(AlwaysValid),
            Err(ValidationFrameworkError::DuplicateRuleName(_))
        ));
    }

    #[test]
    fn rule_execution_failure_is_not_misreported_as_invalid_input() {
        let mut pipeline = ValidationPipeline::new();
        pipeline
            .try_add_rule(RuleFailure)
            .expect("rule registration");

        let result = pipeline.validate(
            &5,
            &context(ValidationMode::CollectAll),
        );

        assert!(matches!(
            result,
            Err(ValidationFrameworkError::RuleFailed { .. })
        ));
    }

    #[test]
    fn registration_order_is_preserved() {
        let mut pipeline = ValidationPipeline::new();
        pipeline
            .try_add_rule(MinimumValue { minimum: 10 })
            .expect("first rule");
        pipeline
            .try_add_rule(AlwaysWarn)
            .expect("second rule");

        assert_eq!(
            pipeline.rule_names(),
            vec!["test.minimum_value", "test.always_warn"]
        );
    }

    #[test]
    fn report_counts_each_severity() {
        let mut report = ValidationReport::new();

        report
            .try_push(ValidationIssue::error("TEST.ERROR", "error"))
            .expect("append error");
        report
            .try_push(ValidationIssue::warning("TEST.WARNING", "warning"))
            .expect("append warning");
        report
            .try_push(ValidationIssue::note("TEST.NOTE", "note"))
            .expect("append note");

        assert_eq!(report.issue_count(), 3);
        assert_eq!(report.error_count(), 1);
        assert_eq!(report.warning_count(), 1);
        assert_eq!(report.note_count(), 1);
        assert!(report.has_errors());
    }
}
