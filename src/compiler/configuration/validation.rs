
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Configuration Validation
//!
//! Path:
//!     src/compiler/configuration/validation.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version 1.97
//!
//! # Purpose
//!
//! This module provides the aggregate validation boundary for compiler
//! configuration contracts.
//!
//! It coordinates the existing validation authorities:
//!
//! - configuration/config.rs: CompilerConfig validation;
//! - configuration/target.rs: TargetDescriptor validation;
//! - configuration/policy.rs: PolicyDocument validation;
//! - configuration/budget.rs: CompilationBudget construction and
//!   structurally valid budget dimensions and limits.
//!
//! It does not replace or duplicate those authorities.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Aggregate configuration validation;
//! - Validation modes and options;
//! - Structured validation findings;
//! - Validation reports;
//! - Validation failure aggregation;
//! - Validation of combinations of existing configuration contracts;
//! - Explicit warnings for configuration behavior that differs from
//!   the current compiler pipeline.
//!
//! This module does not own:
//!
//! - CompilerConfig or OptimizationLevel;
//! - TargetDescriptor or hardware capability discovery;
//! - CompilationBudget accounting;
//! - Policy semantics or authorization;
//! - Source-level profile syntax;
//! - Optimization algorithms or pass legality;
//! - Semantic analysis;
//! - IR verification;
//! - Backend selection;
//! - Physical resource allocation;
//! - Runtime execution.
//!
//! # POCO-REAF
//!
//! Validation must not impose arbitrary hardware capacity ceilings.
//!
//! This module does not define maximum values for:
//!
//! - CPUs, cores, workers, or threads;
//! - GPUs, FPGAs, QPUs, or accelerators;
//! - Nodes, devices, or processing elements;
//! - Memory, storage, or address-space capacity;
//! - Qubits or logical qubits;
//! - Tensor dimensions or rank;
//! - IR operations, modules, or functions;
//! - Source length or artifact size.
//!
//! Configured budgets remain operational policies. An absent budget is
//! not interpreted as infinite physical capacity.
//!
//! # Grammar boundary
//!
//! grammar/compile/profiles.g4 owns source-level compilation-profile
//! syntax.
//!
//! grammar/compile/optimization.g4 owns source-level optimization intent.
//!
//! This module does not parse either grammar, construct AST nodes, resolve
//! profile inheritance, or implement optimization behavior.
//!
//! The frontend and semantic subsystem must resolve source declarations
//! before passing their validated compiler-side representations downstream.
//!
//! # Determinism
//!
//! Validation order is stable:
//!
//! 1. Compiler configuration;
//! 2. Optional target descriptor;
//! 3. Optional policy document;
//! 4. Cross-contract observations.
//!
//! The report preserves that order.
//!
//! # Failure behavior
//!
//! Invalid mandatory configuration contracts produce an error.
//!
//! Non-fatal observations are returned as warnings.
//!
//! No invalid configuration is silently repaired or rewritten.
//!
//! # Concurrency
//!
//! This module is stateless. It performs no global mutation, filesystem
//! access, network access, environment inspection, or hardware discovery.
//!
//! # Integration
//!
//! - configuration/mod.rs exposes this module;
//! - config.rs remains the canonical CompilerConfig validator;
//! - target.rs remains the canonical target descriptor validator;
//! - policy.rs remains the canonical policy document validator;
//! - budget.rs remains the canonical budget representation and accounting
//!   implementation;
//! - compiler API and pipeline call this module before compilation;
//! - semantic profile resolution remains in the frontend/semantic boundary.
//!
//! # Completion contract
//!
//! This file is ready to freeze when:
//!
//! - Its module is exported;
//! - The existing configuration validators are integrated;
//! - The unit tests pass;
//! - The compiler pipeline invokes aggregate validation at the appropriate
//!   boundary;
//! - No competing validation implementation is introduced.

use core::fmt;
use std::error::Error;

use crate::compiler_types::CompilerConfig;

use super::config::{
    validate_compiler_config,
    ConfigurationError,
    ConfigurationValidationMode,
};

use super::policy::{
    PolicyDocument,
    PolicyValidationError,
};

use super::target::{
    TargetDescriptor,
    TargetError,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Version of the aggregate configuration-validation API.
///
/// This is not the Zamani language version, grammar version, compiler
/// version, target descriptor version, or artifact version.
pub const CONFIGURATION_VALIDATION_API_VERSION: u16 = 1;

// ============================================================================
// VALIDATION COMPONENT
// ============================================================================

/// Identifies the contract that produced a validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationComponent {
    /// Existing compiler configuration.
    CompilerConfiguration,

    /// Optional target descriptor.
    TargetDescriptor,

    /// Optional policy document.
    PolicyDocument,

    /// A relationship between existing configuration contracts.
    CrossContract,

    /// Configuration behavior that depends on the current pipeline.
    PipelineCompatibility,
}

impl fmt::Display for ValidationComponent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::CompilerConfiguration => "compiler configuration",
            Self::TargetDescriptor => "target descriptor",
            Self::PolicyDocument => "policy document",
            Self::CrossContract => "cross-contract validation",
            Self::PipelineCompatibility => "pipeline compatibility",
        };

        formatter.write_str(name)
    }
}

// ============================================================================
// FINDING SEVERITY
// ============================================================================

/// Severity of an aggregate validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValidationSeverity {
    /// Informational observation. It does not indicate invalid configuration.
    Information,

    /// Configuration is accepted, but the caller should be aware of a
    /// compatibility or behavior distinction.
    Warning,

    /// A mandatory configuration contract is invalid.
    Error,
}

// ============================================================================
// VALIDATION FINDING
// ============================================================================

/// One structured configuration validation finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationFinding {
    component: ValidationComponent,
    severity: ValidationSeverity,
    code: String,
    message: String,
    explanation: Option<String>,
}

impl ValidationFinding {
    /// Creates a validation finding.
    ///
    /// Codes are open-ended identifiers. They are not restricted to a
    /// closed enum so additional validation authorities can introduce
    /// stable codes without modifying this module.
    pub fn new(
        component: ValidationComponent,
        severity: ValidationSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<Self, ValidationInputError> {
        let code = code.into();
        let message = message.into();

        validate_text("finding code", &code)?;
        validate_text("finding message", &message)?;

        Ok(Self {
            component,
            severity,
            code,
            message,
            explanation: None,
        })
    }

    /// Adds an explanatory detail.
    pub fn with_explanation(
        mut self,
        explanation: impl Into<String>,
    ) -> Result<Self, ValidationInputError> {
        let explanation = explanation.into();

        validate_text("finding explanation", &explanation)?;

        self.explanation = Some(explanation);
        Ok(self)
    }

    /// Returns the component that produced this finding.
    #[must_use]
    pub const fn component(&self) -> ValidationComponent {
        self.component
    }

    /// Returns the severity.
    #[must_use]
    pub const fn severity(&self) -> ValidationSeverity {
        self.severity
    }

    /// Returns the stable finding code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Returns the finding message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the optional explanation.
    #[must_use]
    pub fn explanation(&self) -> Option<&str> {
        self.explanation.as_deref()
    }

    /// Returns whether this finding is an error.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        matches!(self.severity, ValidationSeverity::Error)
    }

    /// Returns whether this finding is a warning.
    #[must_use]
    pub const fn is_warning(&self) -> bool {
        matches!(self.severity, ValidationSeverity::Warning)
    }
}

// ============================================================================
// INPUT VALIDATION
// ============================================================================

/// Failure while constructing aggregate validation input or findings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationInputError {
    /// A required text value was empty or whitespace-only.
    EmptyText {
        field: &'static str,
    },

    /// A text value contained a control character.
    ControlCharacter {
        field: &'static str,
    },
}

impl fmt::Display for ValidationInputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText { field } => {
                write!(formatter, "{field} cannot be empty")
            }

            Self::ControlCharacter { field } => {
                write!(formatter, "{field} cannot contain control characters")
            }
        }
    }
}

impl Error for ValidationInputError {}

fn validate_text(
    field: &'static str,
    value: &str,
) -> Result<(), ValidationInputError> {
    if value.trim().is_empty() {
        return Err(ValidationInputError::EmptyText { field });
    }

    if value.chars().any(char::is_control) {
        return Err(ValidationInputError::ControlCharacter { field });
    }

    Ok(())
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Result of aggregate configuration validation.
///
/// A report can contain warnings while remaining valid.
///
/// It cannot be considered valid if any error finding exists.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConfigurationValidationReport {
    findings: Vec<ValidationFinding>,
}

impl ConfigurationValidationReport {
    /// Creates an empty validation report.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a finding to the report.
    pub fn push(&mut self, finding: ValidationFinding) {
        self.findings.push(finding);
    }

    /// Returns all findings in deterministic validation order.
    #[must_use]
    pub fn findings(&self) -> &[ValidationFinding] {
        &self.findings
    }

    /// Returns whether the report contains no errors.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.has_errors()
    }

    /// Returns whether the report contains an error.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.findings.iter().any(ValidationFinding::is_error)
    }

    /// Returns whether the report contains a warning.
    #[must_use]
    pub fn has_warnings(&self) -> bool {
        self.findings
            .iter()
            .any(ValidationFinding::is_warning)
    }

    /// Returns the number of findings.
    #[must_use]
    pub fn finding_count(&self) -> usize {
        self.findings.len()
    }

    /// Returns the number of errors.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.is_error())
            .count()
    }

    /// Returns the number of warnings.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.is_warning())
            .count()
    }

    /// Returns findings belonging to a component.
    pub fn findings_for(
        &self,
        component: ValidationComponent,
    ) -> impl Iterator<Item = &ValidationFinding> {
        self.findings
            .iter()
            .filter(move |finding| finding.component() == component)
    }

    /// Returns the report as a result.
    ///
    /// Warnings and informational findings do not cause failure.
    pub fn into_result(
        self,
    ) -> Result<Self, ConfigurationValidationFailure> {
        if self.has_errors() {
            Err(ConfigurationValidationFailure::from_report(self))
        } else {
            Ok(self)
        }
    }
}

// ============================================================================
// VALIDATION FAILURE
// ============================================================================

/// Aggregate validation failure.
///
/// The complete report is retained so callers do not lose diagnostic
/// information when validation fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationValidationFailure {
    report: ConfigurationValidationReport,
}

impl ConfigurationValidationFailure {
    fn from_report(report: ConfigurationValidationReport) -> Self {
        Self { report }
    }

    /// Returns the complete validation report.
    #[must_use]
    pub fn report(&self) -> &ConfigurationValidationReport {
        &self.report
    }

    /// Consumes the failure and returns its report.
    #[must_use]
    pub fn into_report(self) -> ConfigurationValidationReport {
        self.report
    }
}

impl fmt::Display for ConfigurationValidationFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "compiler configuration validation failed with {} error(s)",
            self.report.error_count()
        )?;

        for finding in self.report.findings() {
            if finding.is_error() {
                write!(
                    formatter,
                    "; [{}] {}: {}",
                    finding.component(),
                    finding.code(),
                    finding.message()
                )?;
            }
        }

        Ok(())
    }
}

impl Error for ConfigurationValidationFailure {}

// ============================================================================
// VALIDATION REQUEST
// ============================================================================

/// Aggregate validation request.
///
/// Existing contract objects are borrowed. This validator does not take
/// ownership, mutate them, or silently replace invalid values.
///
/// Target descriptors and policy documents are optional because not every
/// compilation request necessarily has a resolved target descriptor or
/// an attached policy document at this stage.
///
/// A missing optional descriptor is not treated as proof of unlimited
/// capabilities or unrestricted authorization.
pub struct ConfigurationValidationRequest<'a> {
    compiler_config: &'a CompilerConfig,
    mode: ConfigurationValidationMode,
    target_descriptor: Option<&'a TargetDescriptor>,
    policy_document: Option<&'a PolicyDocument>,
}

impl<'a> ConfigurationValidationRequest<'a> {
    /// Creates an aggregate validation request.
    #[must_use]
    pub fn new(
        compiler_config: &'a CompilerConfig,
        mode: ConfigurationValidationMode,
    ) -> Self {
        Self {
            compiler_config,
            mode,
            target_descriptor: None,
            policy_document: None,
        }
    }

    /// Adds an optional target descriptor for structural validation.
    ///
    /// This does not claim that the descriptor is compatible with the
    /// legacy CompilationTarget field. That mapping belongs to target
    /// resolution and backend compatibility validation.
    #[must_use]
    pub fn with_target_descriptor(
        mut self,
        target_descriptor: &'a TargetDescriptor,
    ) -> Self {
        self.target_descriptor = Some(target_descriptor);
        self
    }

    /// Adds an optional policy document for structural validation.
    ///
    /// Structural validity does not establish authorization or prove
    /// that every policy expression has been semantically evaluated.
    #[must_use]
    pub fn with_policy_document(
        mut self,
        policy_document: &'a PolicyDocument,
    ) -> Self {
        self.policy_document = Some(policy_document);
        self
    }

    /// Returns the requested validation mode.
    #[must_use]
    pub const fn mode(&self) -> ConfigurationValidationMode {
        self.mode
    }

    /// Performs aggregate validation.
    pub fn validate(
        &self,
    ) -> Result<ConfigurationValidationReport, ConfigurationValidationFailure> {
        let mut report = ConfigurationValidationReport::new();

        self.validate_compiler_configuration(&mut report);
        self.validate_target_descriptor(&mut report);
        self.validate_policy_document(&mut report);
        self.validate_pipeline_compatibility(&mut report);

        report.into_result()
    }

    fn validate_compiler_configuration(
        &self,
        report: &mut ConfigurationValidationReport,
    ) {
        if let Err(error) = validate_compiler_config(
            self.compiler_config,
            self.mode,
        ) {
            report.push(
                finding_from_configuration_error(error)
            );
        }
    }

    fn validate_target_descriptor(
        &self,
        report: &mut ConfigurationValidationReport,
    ) {
        let Some(target) = self.target_descriptor else {
            return;
        };

        if let Err(error) = target.validate() {
            report.push(finding_from_target_error(error));
        }
    }

    fn validate_policy_document(
        &self,
        report: &mut ConfigurationValidationReport,
    ) {
        let Some(policy) = self.policy_document else {
            return;
        };

        if let Err(error) = policy.validate() {
            report.push(finding_from_policy_error(error));
        }
    }

    fn validate_pipeline_compatibility(
        &self,
        report: &mut ConfigurationValidationReport,
    ) {
        // The current compiler pipeline invokes IR verification
        // unconditionally. CompilerConfig.verify therefore must not be
        // interpreted as proof that verification can be disabled.
        //
        // Report the mismatch as a warning instead of silently changing
        // the caller's configuration or rejecting legacy configurations.
        if !self.compiler_config.verify {
            report.push(
                ValidationFinding {
                    component: ValidationComponent::PipelineCompatibility,
                    severity: ValidationSeverity::Warning,
                    code: "compiler.verification.flag_not_enabled".to_owned(),
                    message:
                        "CompilerConfig.verify is false, but the current compiler pipeline performs IR verification unconditionally"
                            .to_owned(),
                    explanation: Some(
                        "The configuration value is preserved for compatibility. \
                         The pipeline must define and document the effective \
                         verification policy before this flag can control \
                         verification behavior."
                            .to_owned(),
                    ),
                }
            );
        }

        // The legacy target enum is a compatibility selector, not a complete
        // capability declaration. Target feasibility must be evaluated by
        // the target/backend planning boundary.
        //
        // No assumption about physical hardware availability is made here.
    }
}

// ============================================================================
// ERROR ADAPTATION
// ============================================================================

fn finding_from_configuration_error(
    error: ConfigurationError,
) -> ValidationFinding {
    ValidationFinding {
        component: ValidationComponent::CompilerConfiguration,
        severity: ValidationSeverity::Error,
        code: "compiler.configuration.invalid".to_owned(),
        message: error.to_string(),
        explanation: None,
    }
}

fn finding_from_target_error(
    error: TargetError,
) -> ValidationFinding {
    ValidationFinding {
        component: ValidationComponent::TargetDescriptor,
        severity: ValidationSeverity::Error,
        code: "compiler.target_descriptor.invalid".to_owned(),
        message: error.to_string(),
        explanation: None,
    }
}

fn finding_from_policy_error(
    error: PolicyValidationError,
) -> ValidationFinding {
    ValidationFinding {
        component: ValidationComponent::PolicyDocument,
        severity: ValidationSeverity::Error,
        code: "compiler.policy_document.invalid".to_owned(),
        message: error.to_string(),
        explanation: None,
    }
}

// ============================================================================
// PUBLIC VALIDATION FUNCTIONS
// ============================================================================

/// Validates the existing canonical CompilerConfig.
///
/// This is the simple entry point for callers that only need to validate
/// CompilerConfig and do not yet have resolved target or policy contracts.
///
/// The actual validation remains owned by configuration/config.rs.
pub fn validate_compilation_configuration(
    config: &CompilerConfig,
    mode: ConfigurationValidationMode,
) -> Result<ConfigurationValidationReport, ConfigurationValidationFailure> {
    ConfigurationValidationRequest::new(config, mode).validate()
}

/// Validates CompilerConfig together with optional resolved target and
/// policy contracts.
///
/// This function performs structural validation only. It does not perform
/// target capability matching, authorization, semantic profile resolution,
/// or backend feasibility analysis.
pub fn validate_compilation_configuration_with_contracts(
    config: &CompilerConfig,
    mode: ConfigurationValidationMode,
    target_descriptor: Option<&TargetDescriptor>,
    policy_document: Option<&PolicyDocument>,
) -> Result<ConfigurationValidationReport, ConfigurationValidationFailure> {
    let mut request = ConfigurationValidationRequest::new(config, mode);

    if let Some(target) = target_descriptor {
        request = request.with_target_descriptor(target);
    }

    if let Some(policy) = policy_document {
        request = request.with_policy_document(policy);
    }

    request.validate()
}

/// Returns whether the supplied configuration is accepted by the existing
/// production optimization validation contract.
///
/// This is a convenience adapter. It does not establish that every
/// optimization pass is implemented or verified for every IR dialect.
#[must_use]
pub fn is_production_configuration_accepted(
    config: &CompilerConfig,
) -> bool {
    validate_compiler_config(
        config,
        ConfigurationValidationMode::Production,
    )
    .is_ok()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler_types::{
        CompilationTarget,
        OptimizationLevel,
    };

    use crate::compiler::configuration::config::CompilerConfigBuilder;
    use crate::compiler::configuration::policy::{
        PolicyDocument,
        PolicyId,
    };
    use crate::compiler::configuration::target::{
        TargetDescriptor,
    };

    fn valid_config() -> CompilerConfig {
        CompilerConfigBuilder::for_target(
            CompilationTarget::X86_64Linux,
        )
        .verify(true)
        .build()
        .expect("test configuration should be valid")
    }

    fn production_config() -> CompilerConfig {
        CompilerConfigBuilder::for_target(
            CompilationTarget::X86_64Linux,
        )
        .optimization_level(OptimizationLevel::Basic)
        .verify(true)
        .build()
        .expect("test production configuration should be valid")
    }

    fn valid_target_descriptor() -> TargetDescriptor {
        TargetDescriptor::builder(
            "test:portable-target",
            "Test portable target",
        )
        .build()
        .expect("test target descriptor should build")
    }

    fn valid_policy_document() -> PolicyDocument {
        PolicyDocument::new(
            PolicyId::new("test::policy")
                .expect("policy ID should be valid")
        )
    }

    #[test]
    fn validation_api_version_is_nonzero() {
        assert_ne!(CONFIGURATION_VALIDATION_API_VERSION, 0);
    }

    #[test]
    fn empty_report_is_valid() {
        let report = ConfigurationValidationReport::new();

        assert!(report.is_valid());
        assert!(!report.has_errors());
        assert!(!report.has_warnings());
        assert_eq!(report.finding_count(), 0);
    }

    #[test]
    fn report_counts_findings_by_severity() {
        let mut report = ConfigurationValidationReport::new();

        report.push(
            ValidationFinding::new(
                ValidationComponent::CompilerConfiguration,
                ValidationSeverity::Warning,
                "test.warning",
                "test warning",
            )
            .expect("finding should be valid")
        );

        report.push(
            ValidationFinding::new(
                ValidationComponent::TargetDescriptor,
                ValidationSeverity::Error,
                "test.error",
                "test error",
            )
            .expect("finding should be valid")
        );

        assert!(!report.is_valid());
        assert!(report.has_errors());
        assert!(report.has_warnings());
        assert_eq!(report.error_count(), 1);
        assert_eq!(report.warning_count(), 1);
        assert_eq!(report.finding_count(), 2);
    }

    #[test]
    fn report_preserves_finding_order() {
        let mut report = ConfigurationValidationReport::new();

        let first = ValidationFinding::new(
            ValidationComponent::CompilerConfiguration,
            ValidationSeverity::Information,
            "first",
            "first finding",
        )
        .expect("finding should be valid");

        let second = ValidationFinding::new(
            ValidationComponent::TargetDescriptor,
            ValidationSeverity::Information,
            "second",
            "second finding",
        )
        .expect("finding should be valid");

        report.push(first);
        report.push(second);

        assert_eq!(report.findings()[0].code(), "first");
        assert_eq!(report.findings()[1].code(), "second");
    }

    #[test]
    fn finding_rejects_empty_code() {
        let result = ValidationFinding::new(
            ValidationComponent::CompilerConfiguration,
            ValidationSeverity::Error,
            " ",
            "invalid configuration",
        );

        assert!(matches!(
            result,
            Err(ValidationInputError::EmptyText {
                field: "finding code"
            })
        ));
    }

    #[test]
    fn finding_rejects_control_characters() {
        let result = ValidationFinding::new(
            ValidationComponent::CompilerConfiguration,
            ValidationSeverity::Error,
            "invalid\ncode",
            "invalid configuration",
        );

        assert!(matches!(
            result,
            Err(ValidationInputError::ControlCharacter {
                field: "finding code"
            })
        ));
    }

    #[test]
    fn standard_configuration_validation_accepts_basic_config() {
        let config = valid_config();

        let report = validate_compilation_configuration(
            &config,
            ConfigurationValidationMode::Standard,
        )
        .expect("standard configuration should validate");

        assert!(report.is_valid());
    }

    #[test]
    fn production_configuration_validation_accepts_basic_config() {
        let config = production_config();

        let report = validate_compilation_configuration(
            &config,
            ConfigurationValidationMode::Production,
        )
        .expect("production configuration should validate");

        assert!(report.is_valid());
    }

    #[test]
    fn production_validation_rejects_experimental_optimization() {
        let config = CompilerConfigBuilder::for_target(
            CompilationTarget::X86_64Linux,
        )
        .optimization_level(OptimizationLevel::UltraAGI)
        .verify(true)
        .build()
        .expect("configuration construction should succeed");

        let result = validate_compilation_configuration(
            &config,
            ConfigurationValidationMode::Production,
        );

        assert!(result.is_err());

        let failure = result.expect_err("production validation must fail");

        assert_eq!(failure.report().error_count(), 1);
        assert_eq!(
            failure.report().findings()[0].component(),
            ValidationComponent::CompilerConfiguration
        );
    }

    #[test]
    fn standard_validation_preserves_experimental_compatibility() {
        let config = CompilerConfigBuilder::for_target(
            CompilationTarget::X86_64Linux,
        )
        .optimization_level(OptimizationLevel::UltraAGI)
        .verify(true)
        .build()
        .expect("configuration construction should succeed");

        let report = validate_compilation_configuration(
            &config,
            ConfigurationValidationMode::Standard,
        )
        .expect("standard validation should preserve compatibility");

        assert!(report.is_valid());
    }

    #[test]
    fn disabled_verification_flag_produces_a_warning() {
        let config = CompilerConfigBuilder::for_target(
            CompilationTarget::X86_64Linux,
        )
        .verify(false)
        .build()
        .expect("configuration construction should succeed");

        let report = validate_compilation_configuration(
            &config,
            ConfigurationValidationMode::Production,
        )
        .expect("verification flag mismatch is currently a warning");

        assert!(report.is_valid());
        assert!(report.has_warnings());

        assert_eq!(
            report.findings()[0].code(),
            "compiler.verification.flag_not_enabled"
        );
    }

    #[test]
    fn target_descriptor_is_validated_when_supplied() {
        let config = valid_config();
        let target = valid_target_descriptor();

        let report = ConfigurationValidationRequest::new(
            &config,
            ConfigurationValidationMode::Standard,
        )
        .with_target_descriptor(&target)
        .validate()
        .expect("valid target descriptor should pass");

        assert!(report.is_valid());
    }

    #[test]
    fn policy_document_is_validated_when_supplied() {
        let config = valid_config();
        let policy = valid_policy_document();

        let report = ConfigurationValidationRequest::new(
            &config,
            ConfigurationValidationMode::Standard,
        )
        .with_policy_document(&policy)
        .validate()
        .expect("valid policy document should pass");

        assert!(report.is_valid());
    }

    #[test]
    fn all_supplied_contracts_are_validated_together() {
        let config = valid_config();
        let target = valid_target_descriptor();
        let policy = valid_policy_document();

        let report = validate_compilation_configuration_with_contracts(
            &config,
            ConfigurationValidationMode::Production,
            Some(&target),
            Some(&policy),
        )
        .expect("all valid contracts should pass");

        assert!(report.is_valid());
    }

    #[test]
    fn missing_optional_target_is_not_a_validation_error() {
        let config = valid_config();

        let report = validate_compilation_configuration(
            &config,
            ConfigurationValidationMode::Standard,
        )
        .expect("target descriptor is optional at this phase");

        assert!(report.is_valid());
    }

    #[test]
    fn validation_does_not_mutate_compiler_configuration() {
        let config = valid_config();
        let original_target = config.target.clone();
        let original_level = config.opt_level.clone();
        let original_verify = config.verify;

        let _ = validate_compilation_configuration(
            &config,
            ConfigurationValidationMode::Standard,
        )
        .expect("configuration should validate");

        assert_eq!(config.target, original_target);
        assert_eq!(config.opt_level, original_level);
        assert_eq!(config.verify, original_verify);
    }

    #[test]
    fn production_acceptance_helper_uses_existing_config_authority() {
        let accepted = production_config();

        let rejected = CompilerConfigBuilder::for_target(
            CompilationTarget::X86_64Linux,
        )
        .optimization_level(OptimizationLevel::UltraAGI)
        .build()
        .expect("configuration construction should succeed");

        assert!(is_production_configuration_accepted(&accepted));
        assert!(!is_production_configuration_accepted(&rejected));
    }
}
