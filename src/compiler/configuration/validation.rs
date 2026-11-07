
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Configuration Validation.
//!
//! Path: src/compiler/configuration/validation.rs
//! Edition: 2021
//! Minimum Rust version: 1.97.
//!
//! # Purpose
//!
//! Provides the aggregate structural-validation boundary for existing
//! compiler configuration contracts.
//!
//! This module coordinates the validators owned by:
//!
//! - configuration/config.rs
//! - configuration/target.rs
//! - configuration/policy.rs
//!
//! It does not replace those validation authorities.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Aggregate validation requests.
//! - Validation findings and reports.
//! - Validation severity and component classification.
//! - Structured aggregation of underlying validation errors.
//! - Stable aggregate error codes.
//! - Validation of the supplied configuration contracts.
//!
//! It does not own:
//!
//! - CompilerConfig or OptimizationLevel.
//! - TargetDescriptor or target capability discovery.
//! - CompilationBudget accounting.
//! - Policy semantics or authorization.
//! - Source grammar, parsing or AST construction.
//! - Semantic analysis.
//! - IR verification.
//! - Optimization implementation or pass legality.
//! - Backend selection or invocation.
//! - Physical resource allocation.
//! - Runtime execution.
//!
//! # POCO-REAF
//!
//! No arbitrary universal hardware-capacity limits are imposed.
//!
//! This module does not define fixed maximums for:
//!
//! - CPUs, cores, workers or threads.
//! - GPUs, FPGAs, QPUs or accelerators.
//! - Nodes, devices or processing elements.
//! - Memory, storage or address-space capacity.
//! - Qubits or logical qubits.
//! - Tensor dimensions or tensor rank.
//! - IR operations, modules or functions.
//! - Source length or artifact size.
//!
//! Configured operational budgets remain distinct from physical
//! capabilities and availability.
//!
//! An absent budget is not proof of infinite physical resources.
//!
//! # Grammar boundary
//!
//! Source-level compilation-profile syntax:
//!     grammar/compile/profiles.g4
//!
//! Source-level optimization intent:
//!     grammar/compile/optimization.g4
//!
//! Compilation grammar composition:
//!     grammar/compile/compile.g4
//!
//! The frontend owns parsing and AST construction.
//!
//! Semantic analysis owns:
//!
//! - Profile name resolution and inheritance.
//! - Requirement interpretation.
//! - Capability meaning.
//! - Constraint validation.
//! - Preference semantics.
//! - Hint interpretation.
//! - Policy authorization semantics.
//!
//! This module consumes compiler-side configuration contracts.
//! It does not parse grammar or implement a second semantic model.
//!
//! # Validation guarantees
//!
//! A successful result establishes only that the supplied contracts
//! passed the structural validators invoked here.
//!
//! It does not establish:
//!
//! - Target availability.
//! - Capability satisfaction.
//! - Requirement feasibility.
//! - Backend compatibility.
//! - Resource availability.
//! - Authorization.
//! - Semantic correctness.
//! - IR correctness.
//! - Successful compilation.
//! - Successful execution.
//!
//! # Determinism
//!
//! Validation is performed in this order:
//!
//! 1. Compiler configuration.
//! 2. Optional target descriptor.
//! 3. Optional policy document.
//!
//! Findings preserve that order.
//!
//! Each underlying validator currently returns its first error.
//! This module collects errors from all supplied contracts, but does not
//! claim to collect multiple errors from a validator that exposes only
//! a single-error Result.
//!
//! # Failure behavior
//!
//! Invalid contracts produce structured error findings.
//!
//! Warnings and informational findings do not make a report invalid.
//!
//! No invalid configuration is silently repaired, normalized or replaced.
//!
//! # Concurrency and side effects
//!
//! This module is stateless.
//!
//! It performs no:
//!
//! - Global mutation.
//! - Filesystem access.
//! - Network access.
//! - Environment inspection.
//! - Hardware discovery.
//! - Process creation.
//! - Physical resource allocation.
//!
//! Independent compilation sessions can validate independent inputs
//! without shared mutable validation state.

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
/// This is independent of the language, grammar, compiler, IR,
/// target-descriptor and backend ABI versions.
pub const CONFIGURATION_VALIDATION_API_VERSION: u16 = 1;

// ============================================================================
// VALIDATION COMPONENT
// ============================================================================

/// Identifies the contract responsible for a validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationComponent {
    /// Existing canonical compiler configuration.
    CompilerConfiguration,

    /// Optional extensible target descriptor.
    TargetDescriptor,

    /// Optional compiler-side policy document.
    PolicyDocument,

    /// A relationship between multiple configuration contracts.
    CrossContract,

    /// An explicitly established pipeline compatibility observation.
    ///
    /// This component must not be used to infer pipeline behavior
    /// without evidence from the actual pipeline contract.
    PipelineCompatibility,
}

impl fmt::Display for ValidationComponent {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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
// SEVERITY
// ============================================================================

/// Severity of an aggregate validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValidationSeverity {
    /// Informational observation.
    Information,

    /// Accepted configuration with a relevant compatibility observation.
    Warning,

    /// Invalid mandatory configuration contract.
    Error,
}

// ============================================================================
// STRUCTURED UNDERLYING CAUSES
// ============================================================================

/// Original error returned by an authoritative configuration validator.
///
/// Keeping the original error preserves its structured information.
/// The aggregate validator does not need to reconstruct or reinterpret
/// the error from a formatted string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationCause {
    /// Error from configuration/config.rs.
    CompilerConfiguration(ConfigurationError),

    /// Error from configuration/target.rs.
    TargetDescriptor(TargetError),

    /// Error from configuration/policy.rs.
    PolicyDocument(PolicyValidationError),
}

impl fmt::Display for ValidationCause {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::CompilerConfiguration(error) => {
                write!(formatter, "{error}")
            }

            Self::TargetDescriptor(error) => {
                write!(formatter, "{error}")
            }

            Self::PolicyDocument(error) => {
                write!(formatter, "{error}")
            }
        }
    }
}

impl Error for ValidationCause {}

// ============================================================================
// FINDING INPUT ERRORS
// ============================================================================

/// Failure while constructing a validation finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationInputError {
    /// Required text is empty or whitespace-only.
    EmptyText {
        /// Name of the affected field.
        field: &'static str,
    },

    /// Text contains a control character.
    ControlCharacter {
        /// Name of the affected field.
        field: &'static str,
    },
}

impl fmt::Display for ValidationInputError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyText { field } => {
                write!(formatter, "{field} cannot be empty")
            }

            Self::ControlCharacter { field } => {
                write!(
                    formatter,
                    "{field} cannot contain control characters"
                )
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
// VALIDATION FINDING
// ============================================================================

/// One structured configuration-validation finding.
///
/// Finding codes are open-ended strings. They are not restricted to
/// a closed enum, allowing additional validation authorities to add
/// stable identifiers without modifying this module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationFinding {
    component: ValidationComponent,
    severity: ValidationSeverity,
    code: String,
    message: String,
    explanation: Option<String>,
    cause: Option<ValidationCause>,
}

impl ValidationFinding {
    /// Creates a finding without an underlying validator error.
    ///
    /// This constructor is intended for explicit informational,
    /// warning and cross-contract observations.
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
            cause: None,
        })
    }

    /// Adds explanatory information.
    pub fn with_explanation(
        mut self,
        explanation: impl Into<String>,
    ) -> Result<Self, ValidationInputError> {
        let explanation = explanation.into();

        validate_text("finding explanation", &explanation)?;

        self.explanation = Some(explanation);

        Ok(self)
    }

    /// Returns the responsible validation component.
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

    /// Returns the human-readable message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the optional explanation.
    #[must_use]
    pub fn explanation(&self) -> Option<&str> {
        self.explanation.as_deref()
    }

    /// Returns the original structured validator error, if present.
    #[must_use]
    pub fn cause(&self) -> Option<&ValidationCause> {
        self.cause.as_ref()
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

    /// Returns whether this finding is informational.
    #[must_use]
    pub const fn is_information(&self) -> bool {
        matches!(self.severity, ValidationSeverity::Information)
    }

    fn from_cause(
        component: ValidationComponent,
        code: &'static str,
        cause: ValidationCause,
    ) -> Self {
        Self {
            component,
            severity: ValidationSeverity::Error,
            code: code.to_owned(),
            message: cause.to_string(),
            explanation: None,
            cause: Some(cause),
        }
    }
}

impl Error for ValidationFinding {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause
            .as_ref()
            .map(|cause| cause as &(dyn Error + 'static))
    }
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Aggregate report of configuration validation.
///
/// Warnings and informational findings do not make a report invalid.
///
/// A report containing one or more errors is invalid.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConfigurationValidationReport {
    findings: Vec<ValidationFinding>,
}

impl ConfigurationValidationReport {
    /// Creates an empty report.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a finding while preserving insertion order.
    pub fn push(
        &mut self,
        finding: ValidationFinding,
    ) {
        self.findings.push(finding);
    }

    /// Returns all findings in validation order.
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
        self.findings
            .iter()
            .any(ValidationFinding::is_error)
    }

    /// Returns whether the report contains warnings.
    #[must_use]
    pub fn has_warnings(&self) -> bool {
        self.findings
            .iter()
            .any(ValidationFinding::is_warning)
    }

    /// Returns whether the report contains informational findings.
    #[must_use]
    pub fn has_information(&self) -> bool {
        self.findings
            .iter()
            .any(ValidationFinding::is_information)
    }

    /// Returns the total number of findings.
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

    /// Returns the number of informational findings.
    #[must_use]
    pub fn information_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.is_information())
            .count()
    }

    /// Returns findings belonging to one validation component.
    pub fn findings_for(
        &self,
        component: ValidationComponent,
    ) -> impl Iterator<Item = &ValidationFinding> + '_ {
        self.findings
            .iter()
            .filter(move |finding| finding.component() == component)
    }

    /// Converts the report into a result.
    ///
    /// Warnings and informational findings are retained in a successful
    /// report. Any error produces a failure containing the complete report.
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
// AGGREGATE FAILURE
// ============================================================================

/// Aggregate configuration-validation failure.
///
/// The complete report is retained, including warnings and informational
/// findings, so callers do not lose diagnostic information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationValidationFailure {
    report: ConfigurationValidationReport,
}

impl ConfigurationValidationFailure {
    fn from_report(
        report: ConfigurationValidationReport,
    ) -> Self {
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
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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

impl Error for ConfigurationValidationFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.report
            .findings()
            .iter()
            .find(|finding| finding.is_error())
            .and_then(|finding| finding.source())
    }
}

// ============================================================================
// VALIDATION REQUEST
// ============================================================================

/// Aggregate validation request.
///
/// Existing configuration contracts are borrowed. This request does not
/// take ownership, mutate inputs or silently normalize invalid values.
///
/// Target descriptors and policy documents are optional because they
/// may not yet have been resolved or attached at this compilation phase.
///
/// Absence of a target descriptor is not proof of unlimited capabilities.
/// Absence of a policy document is not proof of unrestricted authorization.
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

    /// Supplies an optional target descriptor for structural validation.
    ///
    /// This does not establish compatibility with the legacy target field
    /// or prove that the target is available.
    #[must_use]
    pub fn with_target_descriptor(
        mut self,
        target_descriptor: &'a TargetDescriptor,
    ) -> Self {
        self.target_descriptor = Some(target_descriptor);
        self
    }

    /// Supplies an optional policy document for structural validation.
    ///
    /// Structural validation does not establish authorization or prove
    /// that every policy expression has been semantically evaluated.
    #[must_use]
    pub fn with_policy_document(
        mut self,
        policy_document: &'a PolicyDocument,
    ) -> Self {
        self.policy_document = Some(policy_document);
        self
    }

    /// Returns the selected compiler configuration validation mode.
    #[must_use]
    pub const fn mode(&self) -> ConfigurationValidationMode {
        self.mode
    }

    /// Performs aggregate structural validation.
    ///
    /// All supplied contract validators are invoked in deterministic
    /// order. A failure from one contract does not prevent validation
    /// of the other supplied contracts.
    ///
    /// Each underlying validator currently exposes a single-error
    /// Result, so this method collects one error per failing contract.
    pub fn validate(
        &self,
    ) -> Result<
        ConfigurationValidationReport,
        ConfigurationValidationFailure,
    > {
        let mut report = ConfigurationValidationReport::new();

        self.validate_compiler_configuration(&mut report);
        self.validate_target_descriptor(&mut report);
        self.validate_policy_document(&mut report);

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
                finding_from_configuration_error(error),
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
            report.push(
                finding_from_target_error(error),
            );
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
            report.push(
                finding_from_policy_error(error),
            );
        }
    }
}

// ============================================================================
// ERROR ADAPTATION
// ============================================================================

fn finding_from_configuration_error(
    error: ConfigurationError,
) -> ValidationFinding {
    let code = match &error {
        ConfigurationError::MissingTarget => {
            "compiler.configuration.target_missing"
        }

        ConfigurationError::InvalidTarget { .. } => {
            "compiler.configuration.target_invalid"
        }

        ConfigurationError::UnsupportedOptimization { .. } => {
            "compiler.configuration.optimization_unsupported"
        }

        ConfigurationError::InvalidValue { .. } => {
            "compiler.configuration.value_invalid"
        }
    };

    ValidationFinding::from_cause(
        ValidationComponent::CompilerConfiguration,
        code,
        ValidationCause::CompilerConfiguration(error),
    )
}

fn finding_from_target_error(
    error: TargetError,
) -> ValidationFinding {
    let code = match &error {
        TargetError::EmptyField { .. } => {
            "compiler.target.empty_field"
        }

        TargetError::ControlCharacter { .. } => {
            "compiler.target.control_character"
        }

        TargetError::InvalidIdentifier { .. } => {
            "compiler.target.invalid_identifier"
        }

        TargetError::InvalidDecimal { .. } => {
            "compiler.target.invalid_decimal"
        }

        TargetError::InvalidTargetTriple { .. } => {
            "compiler.target.invalid_triple"
        }

        TargetError::DuplicateProperty { .. } => {
            "compiler.target.duplicate_property"
        }

        TargetError::UnqualifiedExtension { .. } => {
            "compiler.target.unqualified_extension"
        }

        TargetError::UnsupportedApiVersion { .. } => {
            "compiler.target.unsupported_api_version"
        }

        TargetError::InvalidLegacyTarget { .. } => {
            "compiler.target.invalid_legacy_target"
        }
    };

    ValidationFinding::from_cause(
        ValidationComponent::TargetDescriptor,
        code,
        ValidationCause::TargetDescriptor(error),
    )
}

fn finding_from_policy_error(
    error: PolicyValidationError,
) -> ValidationFinding {
    let code = match &error {
        PolicyValidationError::EmptyIdentifier => {
            "compiler.policy.empty_identifier"
        }

        PolicyValidationError::EmptyExpression => {
            "compiler.policy.empty_expression"
        }

        PolicyValidationError::EmptyPropertyKey => {
            "compiler.policy.empty_property_key"
        }

        PolicyValidationError::ControlCharacter { .. } => {
            "compiler.policy.control_character"
        }

        PolicyValidationError::ExpressionRequired { .. } => {
            "compiler.policy.expression_required"
        }

        PolicyValidationError::InvalidRuleAction { .. } => {
            "compiler.policy.invalid_rule_action"
        }

        PolicyValidationError::SelfComposition { .. } => {
            "compiler.policy.self_composition"
        }

        PolicyValidationError::DuplicateCompositionReference { .. } => {
            "compiler.policy.duplicate_composition_reference"
        }
    };

    ValidationFinding::from_cause(
        ValidationComponent::PolicyDocument,
        code,
        ValidationCause::PolicyDocument(error),
    )
}

// ============================================================================
// PUBLIC VALIDATION FUNCTIONS
// ============================================================================

/// Validates the canonical CompilerConfig.
///
/// Target descriptors and policy documents are not required for this
/// basic structural-validation operation.
///
/// The actual CompilerConfig validation remains owned by config.rs.
pub fn validate_compilation_configuration(
    config: &CompilerConfig,
    mode: ConfigurationValidationMode,
) -> Result<
    ConfigurationValidationReport,
    ConfigurationValidationFailure,
> {
    ConfigurationValidationRequest::new(config, mode).validate()
}

/// Validates CompilerConfig together with optional target and policy
/// contracts.
///
/// This function performs structural validation only.
///
/// It does not perform:
///
/// - Target capability matching.
/// - Policy authorization.
/// - Semantic profile resolution.
/// - Requirement feasibility.
/// - Backend compatibility.
/// - Physical resource feasibility.
pub fn validate_compilation_configuration_with_contracts(
    config: &CompilerConfig,
    mode: ConfigurationValidationMode,
    target_descriptor: Option<&TargetDescriptor>,
    policy_document: Option<&PolicyDocument>,
) -> Result<
    ConfigurationValidationReport,
    ConfigurationValidationFailure,
> {
    let mut request =
        ConfigurationValidationRequest::new(config, mode);

    if let Some(target) = target_descriptor {
        request = request.with_target_descriptor(target);
    }

    if let Some(policy) = policy_document {
        request = request.with_policy_document(policy);
    }

    request.validate()
}

/// Returns whether the canonical CompilerConfig validator accepts
/// the supplied configuration in production mode.
///
/// This does not prove that every requested optimization is implemented,
/// available or semantically legal for every IR dialect.
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

    use crate::compiler::configuration::config::CompilerConfigBuilder;
    use crate::compiler::configuration::policy::{
        PolicyAction,
        PolicyDirective,
        PolicyDirectiveKind,
        PolicyDocument,
        PolicyId,
        PolicyExpression,
        PolicyRule,
    };
    use crate::compiler::configuration::target::{
        TargetDescriptor,
        TargetId,
    };
    use crate::compiler_types::{
        CompilationTarget,
        OptimizationLevel,
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
            TargetId::new("test:portable-target")
                .expect("test target ID should be valid"),
            "Test portable target",
        )
        .build()
        .expect("test target descriptor should be valid")
    }

    fn valid_policy_document() -> PolicyDocument {
        PolicyDocument::new(
            PolicyId::new("test::policy")
                .expect("test policy ID should be valid"),
        )
    }

    #[test]
    fn validation_api_version_is_nonzero() {
        assert_ne!(
            CONFIGURATION_VALIDATION_API_VERSION,
            0
        );
    }

    #[test]
    fn empty_report_is_valid() {
        let report = ConfigurationValidationReport::new();

        assert!(report.is_valid());
        assert!(!report.has_errors());
        assert!(!report.has_warnings());
        assert!(!report.has_information());
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
            .expect("warning should be valid"),
        );

        report.push(
            ValidationFinding::new(
                ValidationComponent::TargetDescriptor,
                ValidationSeverity::Error,
                "test.error",
                "test error",
            )
            .expect("error should be valid"),
        );

        report.push(
            ValidationFinding::new(
                ValidationComponent::PolicyDocument,
                ValidationSeverity::Information,
                "test.information",
                "test information",
            )
            .expect("information should be valid"),
        );

        assert!(!report.is_valid());
        assert!(report.has_errors());
        assert!(report.has_warnings());
        assert!(report.has_information());

        assert_eq!(report.error_count(), 1);
        assert_eq!(report.warning_count(), 1);
        assert_eq!(report.information_count(), 1);
        assert_eq!(report.finding_count(), 3);
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
        .expect("first finding should be valid");

        let second = ValidationFinding::new(
            ValidationComponent::TargetDescriptor,
            ValidationSeverity::Information,
            "second",
            "second finding",
        )
        .expect("second finding should be valid");

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
    fn production_validation_preserves_structured_error_cause() {
        let config = CompilerConfigBuilder::for_target(
            CompilationTarget::X86_64Linux,
        )
        .optimization_level(OptimizationLevel::UltraAGI)
        .verify(true)
        .build()
        .expect("standard configuration construction should succeed");

        let failure = validate_compilation_configuration(
            &config,
            ConfigurationValidationMode::Production,
        )
        .expect_err("production validation should reject this setting");

        assert_eq!(failure.report().error_count(), 1);

        let finding = &failure.report().findings()[0];

        assert_eq!(
            finding.code(),
            "compiler.configuration.optimization_unsupported"
        );

        assert!(matches!(
            finding.cause(),
            Some(ValidationCause::CompilerConfiguration(
                ConfigurationError::UnsupportedOptimization { .. }
            ))
        ));
    }

    #[test]
    fn standard_validation_preserves_legacy_optimization_compatibility() {
        let config = CompilerConfigBuilder::for_target(
            CompilationTarget::X86_64Linux,
        )
        .optimization_level(OptimizationLevel::UltraAGI)
        .verify(true)
        .build()
        .expect("standard configuration construction should succeed");

        let report = validate_compilation_configuration(
            &config,
            ConfigurationValidationMode::Standard,
        )
        .expect("standard validation should preserve compatibility");

        assert!(report.is_valid());
    }

    #[test]
    fn disabled_verification_is_not_misreported_as_pipeline_failure() {
        let config = CompilerConfigBuilder::for_target(
            CompilationTarget::X86_64Linux,
        )
        .verify(false)
        .build()
        .expect("test configuration should be valid");

        let report = validate_compilation_configuration(
            &config,
            ConfigurationValidationMode::Production,
        )
        .expect("the configuration validator should not infer pipeline behavior");

        assert!(report.is_valid());
        assert!(!report.has_warnings());
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
    fn policy_validation_preserves_structured_error_cause() {
        let config = valid_config();

        let mut policy = valid_policy_document();

        let expression = PolicyExpression::new("true")
            .expect("test expression should be valid");

        let action = PolicyAction::Directive(
            PolicyDirective::with_expression(
                PolicyDirectiveKind::Requirement,
                expression,
            ),
        );

        let condition = PolicyExpression::new("condition")
            .expect("test condition should be valid");

        policy.add_rule(
            PolicyRule::new(condition, action),
        );

        let failure = ConfigurationValidationRequest::new(
            &config,
            ConfigurationValidationMode::Standard,
        )
        .with_policy_document(&policy)
        .validate()
        .expect_err("a requirement is not a valid conditional rule action");

        let finding = failure
            .report()
            .findings_for(ValidationComponent::PolicyDocument)
            .next()
            .expect("policy finding should exist");

        assert_eq!(
            finding.code(),
            "compiler.policy.invalid_rule_action"
        );

        assert!(matches!(
            finding.cause(),
            Some(ValidationCause::PolicyDocument(
                PolicyValidationError::InvalidRuleAction { .. }
            ))
        ));
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
    fn production_acceptance_helper_uses_canonical_config_validator() {
        let accepted = production_config();

        let rejected = CompilerConfigBuilder::for_target(
            CompilationTarget::X86_64Linux,
        )
        .optimization_level(OptimizationLevel::UltraAGI)
        .build()
        .expect("standard configuration construction should succeed");

        assert!(is_production_configuration_accepted(&accepted));
        assert!(!is_production_configuration_accepted(&rejected));
    }

    #[test]
    fn findings_can_be_filtered_by_component() {
        let mut report = ConfigurationValidationReport::new();

        report.push(
            ValidationFinding::new(
                ValidationComponent::CompilerConfiguration,
                ValidationSeverity::Warning,
                "config.warning",
                "configuration warning",
            )
            .expect("finding should be valid"),
        );

        report.push(
            ValidationFinding::new(
                ValidationComponent::PolicyDocument,
                ValidationSeverity::Information,
                "policy.info",
                "policy information",
            )
            .expect("finding should be valid"),
        );

        assert_eq!(
            report
                .findings_for(ValidationComponent::CompilerConfiguration)
                .count(),
            1
        );

        assert_eq!(
            report
                .findings_for(ValidationComponent::PolicyDocument)
                .count(),
            1
        );
    }
}
