
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Deployment Validation
//!
//! Path: src/toolchain/deployment/validation.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provides evidence-based validation of deployment requests before
//! deployment planning, infrastructure operations or runtime handoff.
//!
//! The validator coordinates authoritative validation providers for:
//!
//! - Artifact integrity and compatibility.
//! - Authentication and authorization.
//! - Environment compatibility.
//! - Deployment-provider prerequisites.
//! - Security policy.
//! - Artifact and deployment provenance.
//! - Future deployment validation scopes.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - DeploymentValidationScope.
//! - DeploymentValidationRequest.
//! - DeploymentValidationContext.
//! - DeploymentValidationProvider.
//! - DeploymentValidationReport.
//! - DeploymentValidator.
//! - Provider preflight.
//! - Validation-scope coverage.
//! - Canonical feasibility aggregation.
//!
//! It does NOT own:
//!
//! - Source grammar or AST.
//! - Semantic requirements or capability definitions.
//! - Target registration or target selection.
//! - Artifact generation or storage.
//! - Artifact integrity algorithms.
//! - Cryptographic signing or verification.
//! - Authentication or authorization implementation.
//! - Environment discovery.
//! - Hardware discovery.
//! - Resource allocation or reservation.
//! - Physical placement.
//! - Quantum routing or QEC.
//! - HDL synthesis or timing closure.
//! - Distributed scheduling.
//! - Infrastructure provisioning.
//! - Runtime execution.
//!
//! ## Canonical authorities
//!
//! Deployment request:
//!     crate::toolchain::api::request::DeployRequest
//!
//! Deployment request adapter:
//!     crate::toolchain::deployment::request
//!
//! Environment configuration:
//!     crate::toolchain::environments::configuration
//!
//! Environment validation:
//!     crate::toolchain::environments::validation
//!
//! Artifact and backend validation:
//!     crate::backend
//!
//! Security and cryptographic verification:
//!     Canonical security and cryptographic providers.
//!
//! Hardware capabilities:
//!     crate::zhal
//!
//! Runtime:
//!     crate::zur
//!
//! Distributed infrastructure:
//!     crate::distributed
//!
//! Feasibility:
//!     crate::stdlib::feasibility
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! ## Fundamental distinctions
//!
//! Request validity is not deployment feasibility.
//!
//! Provider registration is not provider trust.
//!
//! Provider trust is not authorization.
//!
//! Artifact existence is not artifact integrity.
//!
//! Artifact integrity is not artifact compatibility.
//!
//! Capability declaration is not resource availability.
//!
//! Feasibility is not resource reservation.
//!
//! Resource reservation is not deployment completion.
//!
//! Deployment submission is not successful execution.
//!
//! Missing evidence is Unknown.
//!
//! Explicit unsupportedness is Unsupported.
//!
//! A proven mandatory violation is Infeasible.
//!
//! ## POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This module supports open-ended deployment domains and providers.
//!
//! It imposes no arbitrary limits on:
//!
//! - Deployment targets.
//! - Providers.
//! - Artifacts.
//! - Computational domains.
//! - Hardware devices.
//! - CPUs or accelerators.
//! - Quantum resources.
//! - Distributed participants.
//! - Memory or storage.
//! - Validation findings.
//!
//! Actual collection growth is subject to available resources and
//! configured operational budgets.
//!
//! Literal infinite physical resources or computation are not promised.
//!
//! ## Side-effect boundary
//!
//! Validation is read-only with respect to deployment infrastructure.
//!
//! It does not:
//!
//! - Provision infrastructure.
//! - Reserve physical resources.
//! - Upload or install artifacts.
//! - Start applications.
//! - Change deployment state.
//! - Modify provider configuration.
//!
//! These operations belong to deployment providers and runtime services.
//!
//! ## Failure semantics
//!
//! Invalid requests return errors.
//!
//! Provider operational failures preserve their causal errors.
//!
//! Missing required validation providers produce Unknown.
//!
//! Empty or incomplete provider reports cannot establish feasibility.
//!
//! An empty overall assessment is Unknown.
//!
//! Only complete, positive, authoritative evidence can establish Feasible.
//!
//! An Unknown result must never be silently promoted to Feasible.

use std::collections::BTreeSet;

use crate::stdlib::feasibility::{
    FeasibilityEvidence,
    FeasibilityFinding,
    FeasibilityImpact,
    FeasibilityReport,
    FeasibilityStatus,
};

use crate::toolchain::deployment::request::{
    DeploymentRequest,
};

use crate::toolchain::environments::validation::{
    EnvironmentValidationReport,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::result::Result;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "toolchain.deployment.validation";

const OPERATION_VALIDATE: &str =
    "toolchain.deployment.validation.validate";

const OPERATION_PREFLIGHT: &str =
    "toolchain.deployment.validation.provider_preflight";

const EVIDENCE_SOURCE: &str =
    "toolchain.deployment.validation";

// ============================================================================
// CANONICAL VALIDATION SCOPE IDENTITIES
// ============================================================================

/// Required artifact-validation scope.
pub const SCOPE_ARTIFACTS: &str = "artifacts.integrity";

/// Required authorization-validation scope.
pub const SCOPE_AUTHORIZATION: &str = "authorization";

/// Required environment-validation scope.
pub const SCOPE_ENVIRONMENT: &str = "environment.compatibility";

/// Required provider-prerequisite scope.
pub const SCOPE_PROVIDER_PREREQUISITES: &str =
    "provider.prerequisites";

/// Required deployment security scope.
pub const SCOPE_SECURITY: &str = "security.policy";

/// Required provenance-validation scope.
pub const SCOPE_PROVENANCE: &str = "provenance";

// ============================================================================
// VALIDATION TEXT
// ============================================================================

fn validate_identifier(
    value: &str,
    description: &str,
) -> Result<()> {
    if value.trim().is_empty() {
        return Err(invalid_input(format!(
            "{description} must not be empty"
        )));
    }

    if value.trim() != value {
        return Err(invalid_input(format!(
            "{description} must not contain surrounding whitespace"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_input(format!(
            "{description} must not contain control characters"
        )));
    }

    Ok(())
}

// ============================================================================
// ERROR ADAPTATION
// ============================================================================

fn invalid_input(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::InvalidInput,
        message,
    )
    .with_operation(OPERATION_VALIDATE)
    .with_context(COMPONENT)
}

fn validation_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ValidationFailed,
        message,
    )
    .with_operation(OPERATION_VALIDATE)
    .with_context(COMPONENT)
}

fn internal_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::Internal,
        message,
    )
    .with_operation(OPERATION_VALIDATE)
    .with_context(COMPONENT)
}

fn provider_error(
    identifier: &str,
    scope: &str,
    source: ToolchainError,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ProviderFailed,
        format!(
            "deployment validation provider `{identifier}` \
             failed while validating scope `{scope}`"
        ),
    )
    .with_operation(OPERATION_VALIDATE)
    .with_subject(identifier)
    .with_context(format!("validation scope: {scope}"))
    .with_context(COMPONENT)
    .with_source(source)
}

// ============================================================================
// DEPLOYMENT VALIDATION SCOPE
// ============================================================================

/// An extensible deployment-validation scope.
///
/// Scope identifiers describe validation responsibilities, not
/// computational domains or physical hardware categories.
///
/// Built-in scopes cover common deployment obligations. Additional
/// providers may introduce new namespaced scopes without changing
/// this module.
///
/// Examples:
///
/// - `artifacts.integrity`
/// - `authorization`
/// - `environment.compatibility`
/// - `provider.prerequisites`
/// - `security.policy`
/// - `provenance`
/// - `quantum.calibration`
/// - `hdl.timing_requirements`
/// - `distributed.network_policy`
/// - `vendor.extension.validation`
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct DeploymentValidationScope(String);

impl DeploymentValidationScope {
    /// Constructs and validates an open-ended scope identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "deployment validation scope",
        )?;

        Ok(Self(value))
    }

    /// Returns the exact scope identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the artifact-integrity scope.
    #[must_use]
    pub fn artifacts() -> Self {
        Self(SCOPE_ARTIFACTS.to_owned())
    }

    /// Returns the authorization scope.
    #[must_use]
    pub fn authorization() -> Self {
        Self(SCOPE_AUTHORIZATION.to_owned())
    }

    /// Returns the environment-compatibility scope.
    #[must_use]
    pub fn environment() -> Self {
        Self(SCOPE_ENVIRONMENT.to_owned())
    }

    /// Returns the provider-prerequisite scope.
    #[must_use]
    pub fn provider_prerequisites() -> Self {
        Self(SCOPE_PROVIDER_PREREQUISITES.to_owned())
    }

    /// Returns the deployment-security scope.
    #[must_use]
    pub fn security() -> Self {
        Self(SCOPE_SECURITY.to_owned())
    }

    /// Returns the provenance scope.
    #[must_use]
    pub fn provenance() -> Self {
        Self(SCOPE_PROVENANCE.to_owned())
    }
}

impl std::fmt::Display for DeploymentValidationScope {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// VALIDATION REQUEST
// ============================================================================

/// Immutable input to a deployment-validation operation.
///
/// The request borrows the canonical deployment request and optionally
/// an existing environment-validation report.
///
/// The canonical deployment request remains owned by the public API.
///
/// No artifact, capability, resource or authorization model is
/// reimplemented here.
///
/// The required validation scopes are explicit and extensible.
#[derive(Debug, Clone)]
pub struct DeploymentValidationRequest<'a> {
    deployment: &'a DeploymentRequest,

    environment: Option<&'a EnvironmentValidationReport>,

    required_scopes: Vec<DeploymentValidationScope>,
}

impl<'a> DeploymentValidationRequest<'a> {
    /// Constructs a deployment-validation request.
    ///
    /// The standard deployment obligations are required by default:
    ///
    /// - Artifact integrity.
    /// - Authorization.
    /// - Environment compatibility.
    /// - Provider prerequisites.
    /// - Security policy.
    /// - Provenance.
    ///
    /// Additional requirements can be supplied through
    /// `with_required_scopes`.
    #[must_use]
    pub fn new(
        deployment: &'a DeploymentRequest,
    ) -> Self {
        Self {
            deployment,
            environment: None,
            required_scopes: vec![
                DeploymentValidationScope::artifacts(),
                DeploymentValidationScope::authorization(),
                DeploymentValidationScope::environment(),
                DeploymentValidationScope::provider_prerequisites(),
                DeploymentValidationScope::security(),
                DeploymentValidationScope::provenance(),
            ],
        }
    }

    /// Supplies a report produced by the canonical environment validator.
    ///
    /// Its original findings are preserved during aggregation.
    ///
    /// This report does not independently establish deployment-provider
    /// readiness or authorization.
    #[must_use]
    pub fn with_environment(
        mut self,
        environment: &'a EnvironmentValidationReport,
    ) -> Self {
        self.environment = Some(environment);
        self
    }

    /// Replaces the required validation scopes.
    ///
    /// This method is useful when a deployment contract defines
    /// additional provider-specific obligations.
    ///
    /// An empty scope list does not imply that the deployment is
    /// automatically valid. The resulting assessment remains Unknown
    /// unless substantive authoritative evidence is supplied.
    ///
    /// Duplicate scopes are rejected.
    pub fn with_required_scopes(
        mut self,
        scopes: Vec<DeploymentValidationScope>,
    ) -> Result<Self> {
        let mut seen = BTreeSet::new();

        for scope in &scopes {
            validate_identifier(
                scope.as_str(),
                "required deployment validation scope",
            )?;

            if !seen.insert(scope.as_str().to_owned()) {
                return Err(invalid_input(format!(
                    "duplicate required deployment validation scope `{}`",
                    scope.as_str()
                )));
            }
        }

        self.required_scopes = scopes;

        Ok(self)
    }

    /// Returns the canonical deployment request.
    #[must_use]
    pub fn deployment(&self) -> &'a DeploymentRequest {
        self.deployment
    }

    /// Returns the optional environment-validation report.
    #[must_use]
    pub fn environment(
        &self,
    ) -> Option<&'a EnvironmentValidationReport> {
        self.environment
    }

    /// Returns all required validation scopes.
    #[must_use]
    pub fn required_scopes(
        &self,
    ) -> &[DeploymentValidationScope] {
        &self.required_scopes
    }

    /// Performs structural validation of the validation request.
    ///
    /// This does not assess physical deployment feasibility.
    pub fn validate(&self) -> Result<()> {
        self.deployment.validate().map_err(|source| {
            ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                "canonical deployment request failed structural validation",
            )
            .with_operation(OPERATION_VALIDATE)
            .with_context(COMPONENT)
            .with_source(source)
        })?;

        let mut seen = BTreeSet::new();

        for scope in &self.required_scopes {
            validate_identifier(
                scope.as_str(),
                "required deployment validation scope",
            )?;

            if !seen.insert(scope.as_str()) {
                return Err(invalid_input(format!(
                    "duplicate required deployment validation scope `{}`",
                    scope.as_str()
                )));
            }
        }

        Ok(())
    }
}

// ============================================================================
// READ-ONLY VALIDATION CONTEXT
// ============================================================================

/// Read-only context supplied to deployment-validation providers.
///
/// Providers receive the canonical deployment request and any available
/// environment-validation evidence.
///
/// Providers must obtain artifact, authorization, provenance, security,
/// target and resource evidence from their own authoritative services.
///
/// This context grants no permission to allocate or execute anything.
#[derive(Debug, Clone, Copy)]
pub struct DeploymentValidationContext<'a> {
    deployment: &'a DeploymentRequest,

    environment: Option<&'a EnvironmentValidationReport>,
}

impl<'a> DeploymentValidationContext<'a> {
    fn from_request(
        request: &'a DeploymentValidationRequest<'a>,
    ) -> Self {
        Self {
            deployment: request.deployment,
            environment: request.environment,
        }
    }

    /// Returns the canonical deployment request.
    #[must_use]
    pub fn deployment(&self) -> &'a DeploymentRequest {
        self.deployment
    }

    /// Returns the existing environment-validation evidence.
    #[must_use]
    pub fn environment(
        &self,
    ) -> Option<&'a EnvironmentValidationReport> {
        self.environment
    }
}

// ============================================================================
// VALIDATION PROVIDER CONTRACT
// ============================================================================

/// Authoritative deployment-validation provider.
///
/// Each provider validates one declared scope.
///
/// Implementations belong to the subsystem that owns the relevant
/// evidence and validation rules.
///
/// Examples include:
///
/// - Backend artifact-integrity adapters.
/// - Security authorization adapters.
/// - Environment compatibility adapters.
/// - Deployment-provider prerequisite adapters.
/// - Cryptographic provenance adapters.
/// - Domain-specific quantum or HDL validation adapters.
///
/// Provider identities and scopes are open-ended.
///
/// A provider MUST:
///
/// - Return a stable, non-empty identifier.
/// - Return a valid, non-empty scope identifier.
/// - Validate against canonical evidence.
/// - Preserve the distinction between requirements and preferences.
/// - Report insufficient evidence as Unknown.
/// - Report Unsupported only when authoritative evidence establishes it.
/// - Report Infeasible only when a mandatory condition is demonstrably
///   violated.
/// - Return substantive findings for every mandatory condition in scope.
/// - Avoid claiming that resources have been reserved.
/// - Avoid claiming that deployment or execution has succeeded.
///
/// Providers MUST NOT:
///
/// - Trust caller-supplied assertions as verified evidence.
/// - Treat provider registration as authentication.
/// - Treat artifact existence as integrity.
/// - Treat capability declaration as resource availability.
/// - Allocate physical resources.
/// - Perform deployment or runtime execution.
///
/// Providers must be safe to call through a shared reference.
///
/// Implementations must not rely on undocumented mutable global state.
pub trait DeploymentValidationProvider: Send + Sync {
    /// Returns the stable, open-ended provider identity.
    fn identifier(&self) -> &str;

    /// Returns the validation scope owned by this provider.
    fn scope(&self) -> &str;

    /// Validates the provider's declared scope.
    ///
    /// An empty or incomplete report is not accepted as proof
    /// of feasibility.
    fn validate(
        &self,
        context: &DeploymentValidationContext<'_>,
    ) -> Result<FeasibilityReport>;
}

// ============================================================================
// DEPLOYMENT VALIDATION REPORT
// ============================================================================

/// Result of a coordinated deployment-validation operation.
///
/// The canonical FeasibilityReport owns aggregate status and finding
/// semantics.
///
/// Completed scopes record which providers returned successfully.
/// They do not independently establish that the checks passed.
///
/// Required scopes record the obligations against which completeness
/// must be assessed.
#[derive(Debug, Clone, Default)]
pub struct DeploymentValidationReport {
    feasibility: FeasibilityReport,

    completed_scopes: BTreeSet<String>,

    required_scopes: BTreeSet<String>,

    provider_count: usize,
}

impl DeploymentValidationReport {
    /// Returns the canonical feasibility report.
    #[must_use]
    pub fn feasibility(&self) -> &FeasibilityReport {
        &self.feasibility
    }

    /// Returns the aggregate feasibility status.
    #[must_use]
    pub fn status(&self) -> FeasibilityStatus {
        self.feasibility.status()
    }

    /// Returns whether all available evidence establishes feasibility.
    ///
    /// This does not reserve resources or guarantee future execution.
    #[must_use]
    pub fn is_feasible(&self) -> bool {
        self.is_complete()
            && self.feasibility.is_feasible()
    }

    /// Returns whether a mandatory deployment condition is infeasible.
    #[must_use]
    pub fn is_infeasible(&self) -> bool {
        self.feasibility.is_infeasible()
    }

    /// Returns whether a required operation is explicitly unsupported.
    #[must_use]
    pub fn is_unsupported(&self) -> bool {
        self.feasibility.is_unsupported()
    }

    /// Returns whether feasibility remains unresolved.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.status() == FeasibilityStatus::Unknown
    }

    /// Returns whether every required validation scope completed
    /// with a substantive complete assessment.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        !self.required_scopes.is_empty()
            && self.required_scopes
                .iter()
                .all(|scope| self.completed_scopes.contains(scope))
            && self.feasibility.is_complete()
    }

    /// Returns whether additional evidence is required.
    #[must_use]
    pub fn needs_more_evidence(&self) -> bool {
        self.is_unknown()
            || self.completed_scopes.len()
                < self.required_scopes.len()
    }

    /// Returns whether a proven mandatory violation exists.
    #[must_use]
    pub fn has_blocking_findings(&self) -> bool {
        self.feasibility.has_blocking_findings()
    }

    /// Returns whether an explicit unsupported finding exists.
    #[must_use]
    pub fn has_unsupported_findings(&self) -> bool {
        self.feasibility.has_unsupported_findings()
    }

    /// Returns the number of findings.
    #[must_use]
    pub fn finding_count(&self) -> usize {
        self.feasibility.len()
    }

    /// Returns the number of providers invoked successfully.
    #[must_use]
    pub const fn provider_count(&self) -> usize {
        self.provider_count
    }

    /// Returns successfully completed validation scopes.
    #[must_use]
    pub fn completed_scopes(&self) -> &BTreeSet<String> {
        &self.completed_scopes
    }

    /// Returns required validation scopes.
    #[must_use]
    pub fn required_scopes(&self) -> &BTreeSet<String> {
        &self.required_scopes
    }

    /// Returns the canonical feasibility report by value.
    #[must_use]
    pub fn into_feasibility(self) -> FeasibilityReport {
        self.feasibility
    }

    fn record_scope(
        &mut self,
        scope: &str,
    ) {
        self.completed_scopes.insert(scope.to_owned());
    }

    fn push_finding(
        &mut self,
        category: &'static str,
        subject: Option<String>,
        impact: FeasibilityImpact,
        message: impl Into<String>,
        evidence_description: impl Into<String>,
    ) -> Result<()> {
        let finding = FeasibilityFinding::new(
            category,
            subject,
            impact,
            message,
        )
        .map_err(|source| {
            internal_error(
                "failed to construct deployment-validation finding",
            )
            .with_source(source)
        })?;

        let evidence = FeasibilityEvidence::new(
            EVIDENCE_SOURCE,
            evidence_description,
        )
        .map_err(|source| {
            internal_error(
                "failed to construct deployment-validation evidence",
            )
            .with_source(source)
        })?;

        self.feasibility.push(
            finding.with_evidence(evidence)
        );

        Ok(())
    }

    fn merge_provider_report(
        &mut self,
        provider: &str,
        scope: &str,
        provider_report: FeasibilityReport,
    ) -> Result<()> {
        let complete = provider_report.is_complete();

        self.feasibility.merge(provider_report);

        if !complete {
            self.push_finding(
                "deployment.validation.incomplete_provider",
                Some(provider.to_owned()),
                FeasibilityImpact::Unknown,
                format!(
                    "provider `{provider}` returned an empty or incomplete \
                     assessment for scope `{scope}`"
                ),
                "The provider did not establish all mandatory conditions \
                 in its declared validation scope",
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// DEPLOYMENT VALIDATOR
// ============================================================================

/// Stateless coordinator for deployment-validation providers.
///
/// This service does not maintain a global registry.
///
/// Providers must be explicitly supplied by the caller.
///
/// No filesystem, network, process, hardware, infrastructure or runtime
/// side effects are performed by the coordinator itself.
#[derive(Debug, Clone, Copy, Default)]
pub struct DeploymentValidator;

impl DeploymentValidator {
    /// Constructs a stateless deployment validator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Validates one deployment request against explicitly supplied
    /// authoritative providers.
    ///
    /// The operation:
    ///
    /// 1. Validates the canonical deployment request.
    /// 2. Validates provider identities and scopes.
    /// 3. Records all required validation scopes.
    /// 4. Merges supplied canonical environment evidence.
    /// 5. Identifies missing validation providers.
    /// 6. Executes each provider's read-only validation operation.
    /// 7. Aggregates canonical feasibility findings.
    /// 8. Returns a complete deployment-validation report.
    ///
    /// Providers are invoked in caller-supplied order.
    ///
    /// A valid negative or unresolved feasibility result is returned
    /// as a report, not as an operational error.
    ///
    /// Provider operational errors are returned immediately with their
    /// causal source preserved.
    ///
    /// # Errors
    ///
    /// Returns an error for:
    ///
    /// - Invalid deployment requests.
    /// - Invalid provider identifiers.
    /// - Invalid validation scopes.
    /// - Duplicate provider identifiers.
    /// - Provider operational failures.
    /// - Internal feasibility-report construction failures.
    ///
    /// Missing required providers do not cause an operational error.
    /// They produce Unknown findings.
    pub fn validate(
        &self,
        request: &DeploymentValidationRequest<'_>,
        providers: &[&dyn DeploymentValidationProvider],
    ) -> Result<DeploymentValidationReport> {
        request.validate()?;

        self.preflight_providers(providers)?;

        let mut report = DeploymentValidationReport::default();

        for scope in request.required_scopes() {
            report.required_scopes.insert(
                scope.as_str().to_owned()
            );
        }

        // A supplied environment report is additional canonical evidence.
        // It is never interpreted as authorization or artifact verification.
        if let Some(environment) = request.environment() {
            report.feasibility.merge(
                environment.feasibility().clone()
            );
        }

        let supplied_scopes: BTreeSet<String> = providers
            .iter()
            .map(|provider| provider.scope().to_owned())
            .collect();

        // Missing providers must not be treated as successful checks.
        for required in &report.required_scopes {
            if !supplied_scopes.contains(required) {
                report.push_finding(
                    "deployment.validation.missing_provider",
                    Some(required.clone()),
                    FeasibilityImpact::Unknown,
                    format!(
                        "no authoritative validation provider was supplied \
                         for required scope `{required}`"
                    ),
                    "The deployment validator could not obtain an \
                     authoritative assessment for this mandatory scope",
                )?;
            }
        }

        let context =
            DeploymentValidationContext::from_request(request);

        for provider in providers {
            let identifier = provider.identifier();
            let scope = provider.scope();

            let provider_report = provider
                .validate(&context)
                .map_err(|source| {
                    provider_error(
                        identifier,
                        scope,
                        source,
                    )
                })?;

            report.merge_provider_report(
                identifier,
                scope,
                provider_report,
            )?;

            report.record_scope(scope);
            report.provider_count += 1;
        }

        Ok(report)
    }

    fn preflight_providers(
        &self,
        providers: &[&dyn DeploymentValidationProvider],
    ) -> Result<()> {
        let mut identifiers = BTreeSet::new();

        for provider in providers {
            let identifier = provider.identifier();
            let scope = provider.scope();

            validate_identifier(
                identifier,
                "deployment validation-provider identifier",
            )
            .map_err(|error| {
                error
                    .with_operation(OPERATION_PREFLIGHT)
                    .with_context(COMPONENT)
            })?;

            validate_identifier(
                scope,
                "deployment validation-provider scope",
            )
            .map_err(|error| {
                error
                    .with_operation(OPERATION_PREFLIGHT)
                    .with_subject(identifier)
                    .with_context(COMPONENT)
            })?;

            if !identifiers.insert(identifier.to_owned()) {
                return Err(
                    invalid_input(format!(
                        "duplicate deployment validation-provider \
                         identifier `{identifier}`"
                    ))
                    .with_operation(OPERATION_PREFLIGHT)
                    .with_subject(identifier)
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::api::request::{
        ProjectLocator,
        ProjectScope,
        TargetSelection,
    };

    use crate::toolchain::errors::{
        ToolchainError,
        ToolchainErrorKind,
    };

    use crate::toolchain::ids::{
        IdNamespace,
        ProjectId,
    };

    struct TestProvider {
        identifier: String,
        scope: String,
        report: FeasibilityReport,
    }

    impl TestProvider {
        fn new(
            identifier: &str,
            scope: &str,
            impact: FeasibilityImpact,
        ) -> Self {
            let finding = FeasibilityFinding::new(
                "deployment.test.assessment",
                Some(scope.to_owned()),
                impact,
                "test provider assessment",
            )
            .expect("valid finding");

            let evidence = FeasibilityEvidence::new(
                "test.provider",
                "Evidence supplied by an authoritative test provider",
            )
            .expect("valid evidence");

            Self {
                identifier: identifier.to_owned(),
                scope: scope.to_owned(),
                report: FeasibilityReport::new()
                    .with_finding(finding.with_evidence(evidence)),
            }
        }
    }

    impl DeploymentValidationProvider for TestProvider {
        fn identifier(&self) -> &str {
            &self.identifier
        }

        fn scope(&self) -> &str {
            &self.scope
        }

        fn validate(
            &self,
            _context: &DeploymentValidationContext<'_>,
        ) -> Result<FeasibilityReport> {
            Ok(self.report.clone())
        }
    }

    struct FailingProvider {
        identifier: String,
        scope: String,
    }

    impl DeploymentValidationProvider for FailingProvider {
        fn identifier(&self) -> &str {
            &self.identifier
        }

        fn scope(&self) -> &str {
            &self.scope
        }

        fn validate(
            &self,
            _context: &DeploymentValidationContext<'_>,
        ) -> Result<FeasibilityReport> {
            Err(ToolchainError::new(
                ToolchainErrorKind::ProviderFailed,
                "simulated provider failure",
            ))
        }
    }

    fn deployment_request() -> DeploymentRequest {
        let namespace = IdNamespace::new("test")
            .expect("valid namespace");

        let project_id = ProjectId::new(namespace, 1)
            .expect("valid project ID");

        let scope = ProjectScope::new(
            ProjectLocator::Id(project_id)
        )
        .expect("valid project scope");

        DeploymentRequest::new(
            scope,
            TargetSelection::resolve_from_capabilities(
                Vec::new()
            )
            .expect("valid target selection"),
        )
        .expect("valid deployment request")
    }

    fn provider(
        identifier: &str,
        scope: &str,
    ) -> TestProvider {
        TestProvider::new(
            identifier,
            scope,
            FeasibilityImpact::Satisfied,
        )
    }

    fn required_providers() -> Vec<TestProvider> {
        vec![
            provider("artifacts", SCOPE_ARTIFACTS),
            provider("authorization", SCOPE_AUTHORIZATION),
            provider("environment", SCOPE_ENVIRONMENT),
            provider("prerequisites", SCOPE_PROVIDER_PREREQUISITES),
            provider("security", SCOPE_SECURITY),
            provider("provenance", SCOPE_PROVENANCE),
        ]
    }

    #[test]
    fn default_required_scopes_are_present() {
        let deployment = deployment_request();

        let request = DeploymentValidationRequest::new(
            &deployment
        );

        assert_eq!(request.required_scopes().len(), 6);

        assert!(request.required_scopes().iter().any(
            |scope| scope.as_str() == SCOPE_ARTIFACTS
        ));

        assert!(request.required_scopes().iter().any(
            |scope| scope.as_str() == SCOPE_AUTHORIZATION
        ));
    }

    #[test]
    fn custom_validation_scopes_are_open_ended() {
        let deployment = deployment_request();

        let scope = DeploymentValidationScope::new(
            "future.computing.validation"
        )
        .expect("valid open-ended scope");

        let request = DeploymentValidationRequest::new(
            &deployment
        )
        .with_required_scopes(vec![scope])
        .expect("valid custom scope");

        assert_eq!(
            request.required_scopes()[0].as_str(),
            "future.computing.validation"
        );
    }

    #[test]
    fn duplicate_required_scopes_are_rejected() {
        let deployment = deployment_request();

        let scope = DeploymentValidationScope::artifacts();

        let result = DeploymentValidationRequest::new(
            &deployment
        )
        .with_required_scopes(vec![
            scope.clone(),
            scope,
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn all_required_positive_providers_establish_feasibility() {
        let deployment = deployment_request();

        let request = DeploymentValidationRequest::new(
            &deployment
        );

        let providers = required_providers();

        let references: Vec<&dyn DeploymentValidationProvider> =
            providers
                .iter()
                .map(|provider| {
                    provider as &dyn DeploymentValidationProvider
                })
                .collect();

        let report = DeploymentValidator::new()
            .validate(&request, &references)
            .expect("validation should succeed");

        assert!(report.is_feasible());
        assert!(report.is_complete());
        assert_eq!(report.provider_count(), 6);
        assert_eq!(report.status(), FeasibilityStatus::Feasible);
    }

    #[test]
    fn missing_provider_produces_unknown() {
        let deployment = deployment_request();

        let request = DeploymentValidationRequest::new(
            &deployment
        );

        let report = DeploymentValidator::new()
            .validate(&request, &[])
            .expect("missing providers are a valid unresolved result");

        assert!(report.is_unknown());
        assert!(!report.is_feasible());
        assert!(!report.is_complete());
        assert!(report.needs_more_evidence());
    }

    #[test]
    fn incomplete_provider_report_cannot_establish_feasibility() {
        let deployment = deployment_request();

        let request = DeploymentValidationRequest::new(
            &deployment
        );

        let empty = TestProvider {
            identifier: "empty".to_owned(),
            scope: SCOPE_ARTIFACTS.to_owned(),
            report: FeasibilityReport::new(),
        };

        let report = DeploymentValidator::new()
            .validate(&request, &[&empty])
            .expect("incomplete report must remain unresolved");

        assert!(report.is_unknown());
        assert!(!report.is_feasible());
    }

    #[test]
    fn blocking_provider_establishes_infeasibility() {
        let deployment = deployment_request();

        let request = DeploymentValidationRequest::new(
            &deployment
        );

        let providers = required_providers();

        let mut references: Vec<&dyn DeploymentValidationProvider> =
            providers
                .iter()
                .map(|provider| {
                    provider as &dyn DeploymentValidationProvider
                })
                .collect();

        let blocked = TestProvider::new(
            "blocked",
            SCOPE_SECURITY,
            FeasibilityImpact::Blocking,
        );

        references.push(&blocked);

        let report = DeploymentValidator::new()
            .validate(&request, &references)
            .expect("negative feasibility is not an operational error");

        assert!(report.is_infeasible());
        assert!(!report.is_feasible());
    }

    #[test]
    fn unsupported_provider_remains_unsupported() {
        let deployment = deployment_request();

        let request = DeploymentValidationRequest::new(
            &deployment
        );

        let providers = vec![
            TestProvider::new(
                "future-target",
                SCOPE_ARTIFACTS,
                FeasibilityImpact::Unsupported,
            ),
        ];

        let report = DeploymentValidator::new()
            .validate(&request, &[&providers[0]])
            .expect("unsupported is a valid assessment");

        assert!(report.is_unsupported());
        assert!(!report.is_feasible());
    }

    #[test]
    fn unknown_provider_evidence_is_not_promoted() {
        let deployment = deployment_request();

        let request = DeploymentValidationRequest::new(
            &deployment
        );

        let unknown = TestProvider::new(
            "unknown",
            SCOPE_ARTIFACTS,
            FeasibilityImpact::Unknown,
        );

        let report = DeploymentValidator::new()
            .validate(&request, &[&unknown])
            .expect("unknown is a valid assessment");

        assert!(report.is_unknown());
        assert!(!report.is_feasible());
    }

    #[test]
    fn duplicate_provider_identifiers_are_rejected() {
        let deployment = deployment_request();

        let request = DeploymentValidationRequest::new(
            &deployment
        );

        let first = provider("duplicate", SCOPE_ARTIFACTS);

        let second = provider(
            "duplicate",
            SCOPE_AUTHORIZATION,
        );

        let result = DeploymentValidator::new()
            .validate(&request, &[&first, &second]);

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_scope_providers_with_distinct_ids_are_allowed() {
        let deployment = deployment_request();

        let request = DeploymentValidationRequest::new(
            &deployment
        );

        let first = provider("artifact-a", SCOPE_ARTIFACTS);
        let second = provider("artifact-b", SCOPE_ARTIFACTS);

        let report = DeploymentValidator::new()
            .validate(&request, &[&first, &second])
            .expect("composable providers are allowed");

        assert_eq!(report.provider_count(), 2);
        assert!(report.completed_scopes().contains(
            SCOPE_ARTIFACTS
        ));

        assert!(!report.is_complete());
    }

    #[test]
    fn invalid_provider_scope_is_rejected() {
        let deployment = deployment_request();

        let request = DeploymentValidationRequest::new(
            &deployment
        );

        let invalid = provider("provider", " ");

        assert!(
            DeploymentValidator::new()
                .validate(&request, &[&invalid])
                .is_err()
        );
    }

    #[test]
    fn provider_operational_errors_are_preserved() {
        let deployment = deployment_request();

        let request = DeploymentValidationRequest::new(
            &deployment
        );

        let failing = FailingProvider {
            identifier: "failing".to_owned(),
            scope: SCOPE_ARTIFACTS.to_owned(),
        };

        let result = DeploymentValidator::new()
            .validate(&request, &[&failing]);

        assert!(result.is_err());

        let error = result.expect_err("provider must fail");

        assert_eq!(
            error.kind(),
            ToolchainErrorKind::ProviderFailed
        );
    }

    #[test]
    fn canonical_deployment_request_is_validated() {
        let mut deployment = deployment_request();

        // The request constructor already establishes its structural
        // contract. The validator rechecks it at the service boundary.
        assert!(deployment.validate().is_ok());

        let _ = &mut deployment;
    }

    #[test]
    fn scope_identity_is_case_sensitive() {
        let first = DeploymentValidationScope::new(
            "security.policy"
        )
        .expect("valid scope");

        let second = DeploymentValidationScope::new(
            "Security.Policy"
        )
        .expect("valid scope");

        assert_ne!(first, second);
    }

    #[test]
    fn future_computational_domains_need_no_enum_changes() {
        for scope in [
            "classical.native_validation",
            "quantum.qpu_calibration",
            "hdl.timing_validation",
            "ai.accelerator_compatibility",
            "hybrid.cross_domain_validation",
            "distributed.cluster_validation",
            "networking.transport_validation",
            "future.domain.validation",
        ] {
            assert!(
                DeploymentValidationScope::new(scope).is_ok()
            );
        }
    }

    #[test]
    fn validation_does_not_claim_deployment_completion() {
        let deployment = deployment_request();

        let request = DeploymentValidationRequest::new(
            &deployment
        );

        let providers = required_providers();

        let references: Vec<&dyn DeploymentValidationProvider> =
            providers
                .iter()
                .map(|provider| {
                    provider as &dyn DeploymentValidationProvider
                })
                .collect();

        let report = DeploymentValidator::new()
            .validate(&request, &references)
            .expect("validation should succeed");

        assert!(report.is_feasible());

        // Feasibility is not deployment or execution completion.
        // No deployment operation is exposed by this validator.
        assert_eq!(report.provider_count(), 6);
    }
}
