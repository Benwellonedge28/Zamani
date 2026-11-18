#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Environment Validation
//!
//! Path: `src/toolchain/environments/validation.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Coordinate evidence-based validation of explicitly configured
//! development, compilation, build and execution environments.
//!
//! This module joins:
//!
//! - Environment configuration.
//! - Canonical ZUR platform observations.
//! - Canonically discovered external toolchains.
//! - Canonical environment fingerprints.
//! - Registered domain-specific validation providers.
//! - The standard-library feasibility-report contract.
//!
//! It does not create a competing semantic requirement model,
//! capability registry, resource model, target registry, hardware
//! discovery implementation, resource allocator or runtime.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Environment-validation request construction.
//! - Validation-provider coordination.
//! - Validation-provider identity preflight.
//! - Environment configuration precondition checks.
//! - Explicit platform-observation consistency checks.
//! - Explicit external-toolchain presence checks.
//! - Aggregation of canonical feasibility reports.
//! - Validation check completion reporting.
//! - Toolchain-boundary error adaptation.
//!
//! This module does NOT own:
//!
//! - Source grammar parsing.
//! - AST construction or semantic analysis.
//! - Requirement, capability, preference, constraint or hint semantics.
//! - Resource quantities, units or resource feasibility algorithms.
//! - Hardware discovery or device enumeration.
//! - Capability evidence generation.
//! - Provider authentication or authorization.
//! - Physical resource allocation or reservation.
//! - Target selection or target realization.
//! - Backend implementation or executable invocation.
//! - Compiler reproducibility fingerprint algorithms.
//! - Quantum routing, error correction or scheduling.
//! - HDL synthesis, placement, routing or timing closure.
//! - Distributed placement or cluster scheduling.
//! - Runtime execution.
//!
//! ## Canonical authorities
//!
//! Environment configuration:
//!     crate::toolchain::environments::configuration
//!
//! Environment discovery:
//!     crate::toolchain::environments::discovery
//!
//! Environment fingerprints:
//!     crate::toolchain::environments::fingerprints
//!
//! Compiler reproducibility:
//!     crate::compiler::reproducibility::environment
//!
//! External toolchain discovery:
//!     crate::backend::toolchain
//!
//! Backend precondition validation:
//!     crate::backend::validation
//!
//! Semantic capability requirements:
//!     crate::frontend::semantic::capabilities
//!
//! Hardware capability evidence:
//!     crate::zhal::capabilities
//!
//! Aggregate feasibility:
///     crate::stdlib::feasibility
//!
//! Platform observations:
//!     crate::zur::platforms::provider
//!
//! ## Fundamental distinctions
//!
//! Configuration:
//!     Explicit environment intent.
//!
//! Discovery:
//!     An observation obtained from an explicitly selected provider.
//!
//! Validation:
//!     An assessment of supplied facts against supplied contracts.
//!
//! Capability:
//!     A semantic ability or an implementation's declared ability.
//!
//! Evidence:
//!     Information supplied by an authoritative evidence producer.
//!
//! Feasibility:
//!     An evidence-based assessment of mandatory conditions.
//!
//! Allocation:
//!     A separate operation owned by physical resource authorities.
//!
//! Execution:
//!     A separate operation owned by the runtime.
//!
//! None of these stages may be silently substituted for another.
//!
//! ## POCO-REAF and scalability
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This module is open-world. It does not impose fixed maxima on:
//!
//! - Validation providers.
//! - External toolchains.
//! - Environment attributes.
//! - Computing domains.
//! - Capability requirements.
//! - Resource requirements.
//! - Devices or accelerators.
//! - Quantum resources.
//! - Distributed participants.
//! - Memory or storage.
//! - Validation findings.
//!
//! Collection growth is governed by available memory and any explicit
//! operational budgets enforced by their owning services.
//!
//! No literal infinite physical capacity or computation is promised.
//!
//! ## Unknown information
//!
//! Missing evidence is reported as `Unknown`.
//!
//! Unknown is not silently converted into:
//!
//! - Supported.
//! - Available.
//! - Compatible.
//! - Feasible.
//! - Infeasible.
//! - Permanently unsupported.
//!
//! A provider may report `Unsupported` or `Blocking` only when its
//! authoritative assessment establishes that conclusion.
//!
//! ## Integration
//!
//! Upstream:
//!
//! - `EnvironmentConfiguration`.
//! - `PlatformObservation`, when available.
//! - `DiscoveredToolchains`, when available.
//! - `EnvironmentFingerprint`, when available.
//! - Explicit external-tool requirements.
//! - Explicitly selected validation providers.
//!
//! Downstream:
//!
//! - Build planning.
//! - Target compatibility.
//! - External-tool readiness.
//! - Deployment planning.
//! - Environment diagnostics.
//! - Human and AI toolchain clients.
//!
//! Domain-specific validators adapt canonical semantic requirements
//! and domain evidence into `FeasibilityReport` findings.
//!
//! This module aggregates those findings without reinterpreting them.

use std::collections::BTreeSet;

use crate::stdlib::feasibility::{
    FeasibilityEvidence,
    FeasibilityFinding,
    FeasibilityImpact,
    FeasibilityReport,
    FeasibilityStatus,
};

use crate::toolchain::environments::configuration::{
    EnvironmentConfiguration,
};

use crate::toolchain::environments::discovery::{
    DiscoveredToolchains,
};

use crate::toolchain::environments::fingerprints::{
    EnvironmentFingerprint,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::result::Result;

use crate::zur::platforms::provider::PlatformObservation;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "toolchain.environments.validation";

const OPERATION_VALIDATE: &str =
    "toolchain.environments.validation.validate";

const OPERATION_PROVIDER_PREFLIGHT: &str =
    "toolchain.environments.validation.provider_preflight";

// ============================================================================
// FINDING IDENTITIES
// ============================================================================

const CATEGORY_CONFIGURATION: &str =
    "toolchain.environment.configuration";

const CATEGORY_PLATFORM_OBSERVATION: &str =
    "toolchain.environment.platform_observation";

const CATEGORY_EXTERNAL_TOOLCHAIN: &str =
    "toolchain.environment.external_toolchain";

const CATEGORY_VALIDATION_PROVIDER: &str =
    "toolchain.environment.validation_provider";

const CATEGORY_FINGERPRINT: &str =
    "toolchain.environment.fingerprint";

// ============================================================================
// TEXT VALIDATION
// ============================================================================

fn validate_identifier(
    value: &str,
    description: &'static str,
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

fn provider_error(
    identifier: &str,
    source: ToolchainError,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ProviderFailed,
        format!(
            "environment validation provider `{identifier}` failed"
        ),
    )
    .with_operation(OPERATION_VALIDATE)
    .with_subject(identifier)
    .with_context(COMPONENT)
    .with_source(source)
}

// ============================================================================
// VALIDATION REQUEST
// ============================================================================

/// Immutable input to one environment-validation operation.
///
/// The request borrows canonical configuration, discovery and
/// fingerprint objects. It does not copy or redefine their models.
///
/// Optional observations remain optional unless the caller explicitly
/// requires them.
///
/// External-tool requirements are expressed using the exact identifiers
/// used by `backend::toolchain::ToolchainSpec` and
/// `DiscoveredToolchains`.
///
/// This type does not parse source declarations or interpret domain
/// requirements.
#[derive(Debug)]
pub struct EnvironmentValidationRequest<'a> {
    configuration: &'a EnvironmentConfiguration,

    platform_observation: Option<&'a PlatformObservation>,

    discovered_toolchains: Option<&'a DiscoveredToolchains>,

    fingerprint: Option<&'a EnvironmentFingerprint>,

    required_toolchains: Vec<String>,

    platform_observation_required: bool,
}

impl<'a> EnvironmentValidationRequest<'a> {
    /// Creates a validation request for an explicit environment
    /// configuration.
    ///
    /// No observation, discovered toolchain or fingerprint is assumed.
    ///
    /// The caller must explicitly attach those objects when available.
    #[must_use]
    pub fn new(
        configuration: &'a EnvironmentConfiguration,
    ) -> Self {
        Self {
            configuration,
            platform_observation: None,
            discovered_toolchains: None,
            fingerprint: None,
            required_toolchains: Vec::new(),
            platform_observation_required: false,
        }
    }

    /// Supplies an observation obtained from the canonical ZUR
    /// platform-discovery interface.
    ///
    /// This does not independently verify the provider's authenticity
    /// or establish that its reported capabilities are sufficient.
    #[must_use]
    pub fn with_platform_observation(
        mut self,
        observation: &'a PlatformObservation,
    ) -> Self {
        self.platform_observation = Some(observation);
        self
    }

    /// Supplies the result of canonical external-toolchain discovery.
    ///
    /// Descriptors remain owned by `DiscoveredToolchains`.
    ///
    /// A descriptor establishes successful backend discovery and
    /// version validation. It does not establish executable binary
    /// integrity or authenticity.
    #[must_use]
    pub fn with_discovered_toolchains(
        mut self,
        discovered: &'a DiscoveredToolchains,
    ) -> Self {
        self.discovered_toolchains = Some(discovered);
        self
    }

    /// Supplies the canonical toolchain-facing environment fingerprint.
    ///
    /// A fingerprint is content identity evidence. It is not proof of
    /// authenticity, capability support, resource availability or
    /// workload feasibility.
    #[must_use]
    pub fn with_fingerprint(
        mut self,
        fingerprint: &'a EnvironmentFingerprint,
    ) -> Self {
        self.fingerprint = Some(fingerprint);
        self
    }

    /// Declares whether a platform observation is mandatory for this
    /// validation request.
    ///
    /// When `required` is true and no observation is supplied, the
    /// resulting feasibility report contains an `Unknown` finding.
    ///
    /// This does not force every build or compilation to require a
    /// physical execution-host observation.
    #[must_use]
    pub fn require_platform_observation(
        mut self,
        required: bool,
    ) -> Self {
        self.platform_observation_required = required;
        self
    }

    /// Adds one explicitly required external-toolchain identifier.
    ///
    /// The identifier is compared exactly and case-sensitively with
    /// the canonical discovered-toolchain identifier.
    ///
    /// Duplicate identifiers are rejected.
    ///
    /// The request does not resolve executables or inspect the host.
    pub fn require_toolchain(
        mut self,
        identifier: impl Into<String>,
    ) -> Result<Self> {
        let identifier = identifier.into();

        validate_identifier(
            &identifier,
            "required external-toolchain identifier",
        )?;

        if self.required_toolchains
            .iter()
            .any(|existing| existing == &identifier)
        {
            return Err(invalid_input(format!(
                "duplicate required external-toolchain identifier: \
                 {identifier}"
            ))
            .with_subject(identifier));
        }

        self.required_toolchains.push(identifier);

        Ok(self)
    }

    /// Adds multiple explicitly required external-toolchain identifiers.
    ///
    /// The operation is atomic: if any identifier is invalid or duplicated,
    /// no partially updated request is returned.
    pub fn require_toolchains<I, S>(
        mut self,
        identifiers: I,
    ) -> Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut staged = self.required_toolchains.clone();

        for identifier in identifiers {
            let identifier = identifier.into();

            validate_identifier(
                &identifier,
                "required external-toolchain identifier",
            )?;

            if staged.iter().any(|existing| existing == &identifier) {
                return Err(invalid_input(format!(
                    "duplicate required external-toolchain identifier: \
                     {identifier}"
                ))
                .with_subject(identifier));
            }

            staged.push(identifier);
        }

        self.required_toolchains = staged;

        Ok(self)
    }

    /// Returns the environment configuration being validated.
    #[must_use]
    pub fn configuration(&self) -> &'a EnvironmentConfiguration {
        self.configuration
    }

    /// Returns the supplied platform observation, if any.
    #[must_use]
    pub fn platform_observation(
        &self,
    ) -> Option<&'a PlatformObservation> {
        self.platform_observation
    }

    /// Returns the supplied discovered external toolchains, if any.
    #[must_use]
    pub fn discovered_toolchains(
        &self,
    ) -> Option<&'a DiscoveredToolchains> {
        self.discovered_toolchains
    }

    /// Returns the supplied environment fingerprint, if any.
    #[must_use]
    pub fn fingerprint(
        &self,
    ) -> Option<&'a EnvironmentFingerprint> {
        self.fingerprint
    }

    /// Returns required external-toolchain identifiers in request order.
    #[must_use]
    pub fn required_toolchains(&self) -> &[String] {
        &self.required_toolchains
    }

    /// Returns whether a platform observation is mandatory.
    #[must_use]
    pub const fn platform_observation_required(&self) -> bool {
        self.platform_observation_required
    }
}

// ============================================================================
// VALIDATION CONTEXT
// ============================================================================

/// Read-only context supplied to domain-specific environment validators.
///
/// Domain validators use this context to consult canonical environment
/// observations and configuration while evaluating requirements owned
/// by their respective subsystems.
///
/// For example:
///
/// - Classical validation can consult compiler/backend contracts.
/// - Quantum validation can consult quantum capability and QPU evidence.
/// - HDL validation can consult synthesis and hardware-provider evidence.
/// - AI validation can consult tensor and accelerator providers.
/// - Distributed validation can consult distributed capability providers.
/// - Hybrid validation can coordinate the relevant domain validators.
///
/// This context does not expose mutable toolchain state or grant authority
/// to allocate physical resources.
#[derive(Debug, Clone, Copy)]
pub struct EnvironmentValidationContext<'a> {
    configuration: &'a EnvironmentConfiguration,

    platform_observation: Option<&'a PlatformObservation>,

    discovered_toolchains: Option<&'a DiscoveredToolchains>,

    fingerprint: Option<&'a EnvironmentFingerprint>,
}

impl<'a> EnvironmentValidationContext<'a> {
    fn from_request(
        request: &'a EnvironmentValidationRequest<'a>,
    ) -> Self {
        Self {
            configuration: request.configuration,
            platform_observation: request.platform_observation,
            discovered_toolchains: request.discovered_toolchains,
            fingerprint: request.fingerprint,
        }
    }

    /// Returns the canonical environment configuration.
    #[must_use]
    pub fn configuration(&self) -> &'a EnvironmentConfiguration {
        self.configuration
    }

    /// Returns the canonical ZUR platform observation, if supplied.
    #[must_use]
    pub fn platform_observation(
        &self,
    ) -> Option<&'a PlatformObservation> {
        self.platform_observation
    }

    /// Returns the canonical discovered external toolchains, if supplied.
    #[must_use]
    pub fn discovered_toolchains(
        &self,
    ) -> Option<&'a DiscoveredToolchains> {
        self.discovered_toolchains
    }

    /// Returns the canonical environment fingerprint, if supplied.
    #[must_use]
    pub fn fingerprint(
        &self,
    ) -> Option<&'a EnvironmentFingerprint> {
        self.fingerprint
    }
}

// ============================================================================
// DOMAIN VALIDATION PROVIDER CONTRACT
// ============================================================================

/// Contract implemented by an authoritative environment-validation
/// provider.
///
/// Implementations belong to the subsystem that understands the
/// requirements and evidence it validates.
///
/// A provider must:
///
/// - Use canonical semantic requirements.
/// - Use its domain's authoritative capability and resource evidence.
/// - Preserve Requirement / Capability / Preference / Constraint / Hint
///   distinctions.
/// - Return `Unknown` when evidence is insufficient.
/// - Return `Unsupported` only when unsupportedness is established.
/// - Return `Blocking` only when a mandatory violation is established.
/// - Avoid claiming that resources have been reserved.
/// - Avoid claiming that execution has succeeded.
/// - Preserve meaningful provider errors.
///
/// Provider identifiers must be stable, non-empty and unique within
/// one validation request.
///
/// Implementations must be safe to call through a shared reference.
/// They must not rely on this coordinator for hidden mutable global state.
pub trait EnvironmentValidationProvider: Send + Sync {
    /// Returns the stable, open-world identity of this validator.
    fn identifier(&self) -> &str;

    /// Validates the domain-specific environment conditions represented
    /// by this provider.
    ///
    /// The returned report must contain a substantive finding for every
    /// mandatory condition in the provider's declared scope.
    ///
    /// An empty or incomplete report is not accepted as proof of
    /// feasibility by the coordinator.
    fn validate(
        &self,
        context: &EnvironmentValidationContext<'_>,
    ) -> Result<FeasibilityReport>;
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Immutable result of one coordinated environment-validation operation.
///
/// The canonical `FeasibilityReport` owns finding semantics and aggregate
/// feasibility status.
///
/// `completed_checks` records which orchestration checks completed. It
/// does not imply that every check passed.
#[derive(Debug, Clone, Default)]
pub struct EnvironmentValidationReport {
    feasibility: FeasibilityReport,

    completed_checks: BTreeSet<String>,

    provider_count: usize,
}

impl EnvironmentValidationReport {
    /// Returns the canonical aggregate feasibility report.
    #[must_use]
    pub fn feasibility(&self) -> &FeasibilityReport {
        &self.feasibility
    }

    /// Returns the aggregate feasibility status.
    ///
    /// An empty or incomplete assessment remains `Unknown`.
    #[must_use]
    pub fn status(&self) -> FeasibilityStatus {
        self.feasibility.status()
    }

    /// Returns whether the available evidence establishes feasibility.
    ///
    /// This does not reserve resources or guarantee later execution.
    #[must_use]
    pub fn is_feasible(&self) -> bool {
        self.feasibility.is_feasible()
    }

    /// Returns whether evidence establishes a mandatory-condition failure.
    #[must_use]
    pub fn is_infeasible(&self) -> bool {
        self.feasibility.is_infeasible()
    }

    /// Returns whether an authoritative validator established that a
    /// required operation is unsupported in the assessed scope.
    #[must_use]
    pub fn is_unsupported(&self) -> bool {
        self.feasibility.is_unsupported()
    }

    /// Returns whether additional evidence is required.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.feasibility.is_unknown()
    }

    /// Returns whether the report is complete according to the canonical
    /// feasibility-report contract.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.feasibility.is_complete()
    }

    /// Returns whether the report contains a proven blocking finding.
    #[must_use]
    pub fn has_blocking_findings(&self) -> bool {
        self.feasibility.has_blocking_findings()
    }

    /// Returns whether the report contains an explicit unsupported finding.
    #[must_use]
    pub fn has_unsupported_findings(&self) -> bool {
        self.feasibility.has_unsupported_findings()
    }

    /// Returns whether the report requires additional evidence.
    #[must_use]
    pub fn needs_more_evidence(&self) -> bool {
        self.feasibility.needs_more_evidence()
    }

    /// Returns the number of recorded findings.
    #[must_use]
    pub fn finding_count(&self) -> usize {
        self.feasibility.len()
    }

    /// Returns completed orchestration check identifiers in deterministic
    /// order.
    #[must_use]
    pub fn completed_checks(&self) -> &BTreeSet<String> {
        &self.completed_checks
    }

    /// Returns the number of domain validation providers invoked
    /// successfully.
    #[must_use]
    pub const fn provider_count(&self) -> usize {
        self.provider_count
    }

    /// Returns the underlying canonical feasibility report by value.
    #[must_use]
    pub fn into_feasibility(self) -> FeasibilityReport {
        self.feasibility
    }

    fn record_check(
        &mut self,
        identifier: impl Into<String>,
    ) -> Result<()> {
        let identifier = identifier.into();

        validate_identifier(
            &identifier,
            "completed validation check identifier",
        )?;

        self.completed_checks.insert(identifier);

        Ok(())
    }

    fn push_finding(
        &mut self,
        category: &'static str,
        subject: Option<String>,
        impact: FeasibilityImpact,
        message: impl Into<String>,
        evidence_source: &'static str,
        evidence_description: impl Into<String>,
    ) -> Result<()> {
        let finding = FeasibilityFinding::new(
            category,
            subject,
            impact,
            message,
        )
        .map_err(|source| {
            ToolchainError::new(
                ToolchainErrorKind::Internal,
                "failed to construct an environment-validation finding",
            )
            .with_operation(OPERATION_VALIDATE)
            .with_context(COMPONENT)
            .with_source(source)
        })?;

        let evidence = FeasibilityEvidence::new(
            evidence_source,
            evidence_description,
        )
        .map_err(|source| {
            ToolchainError::new(
                ToolchainErrorKind::Internal,
                "failed to construct environment-validation evidence",
            )
            .with_operation(OPERATION_VALIDATE)
            .with_context(COMPONENT)
            .with_source(source)
        })?;

        self.feasibility.push(
            finding.with_evidence(evidence)
        );

        Ok(())
    }

    fn merge_provider_report(
        &mut self,
        identifier: &str,
        provider_report: FeasibilityReport,
    ) -> Result<()> {
        let complete = provider_report.is_complete();

        self.feasibility.merge(provider_report);

        if !complete {
            self.push_finding(
                CATEGORY_VALIDATION_PROVIDER,
                Some(identifier.to_owned()),
                FeasibilityImpact::Unknown,
                format!(
                    "validation provider `{identifier}` returned an \
                     empty or incomplete assessment; feasibility cannot \
                     be established"
                ),
                "toolchain.environments.validation",
                "The provider report did not establish a complete assessment",
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// ENVIRONMENT VALIDATOR
// ============================================================================

/// Stateless coordinator for environment-validation providers.
///
/// The validator performs common configuration and evidence precondition
/// checks, then delegates domain-specific evaluation to explicitly supplied
/// providers.
///
/// No global registry, implicit provider selection, filesystem inspection,
/// hardware discovery, process execution or resource allocation occurs.
#[derive(Debug, Clone, Copy, Default)]
pub struct EnvironmentValidator;

impl EnvironmentValidator {
    /// Creates a stateless environment validator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Validates one environment request using explicitly supplied
    /// domain-specific validation providers.
    ///
    /// Provider identifiers are checked for validity and duplication
    /// before any provider is invoked.
    ///
    /// The operation is fail-fast for malformed input and provider errors.
    /// Valid negative or unresolved feasibility assessments are returned
    /// as successful reports with `Infeasible`, `Unsupported` or `Unknown`
    /// status.
    ///
    /// # Arguments
    ///
    /// * `request` - Explicit configuration and available evidence.
    /// * `providers` - Authoritative validators for the requested domains.
    ///
    /// # Returns
    ///
    /// A report combining common environment checks and all provider
    /// findings, preserving finding order and duplicate findings.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The configuration is structurally invalid.
    /// - A configured platform provider does not match the observation.
    /// - A validation-provider identifier is invalid or duplicated.
    /// - A provider returns an operational error.
    /// - A local report or evidence invariant cannot be constructed.
    ///
    /// A missing observation, missing discovered toolchain or incomplete
    /// domain report is represented as `Unknown`, not as a fabricated
    /// successful validation.
    pub fn validate(
        &self,
        request: &EnvironmentValidationRequest<'_>,
        providers: &[&dyn EnvironmentValidationProvider],
    ) -> Result<EnvironmentValidationReport> {
        self.preflight_providers(providers)?;

        request.configuration.validate().map_err(|source| {
            ToolchainError::new(
                ToolchainErrorKind::InvalidConfiguration,
                "environment configuration failed structural validation",
            )
            .with_operation(OPERATION_VALIDATE)
            .with_context(COMPONENT)
            .with_source(source)
        })?;

        self.validate_platform_provider_consistency(request)?;

        let mut report = EnvironmentValidationReport::default();

        report.record_check("environment.configuration")?;

        report.push_finding(
            CATEGORY_CONFIGURATION,
            Some(
                request.configuration.environment_id().to_string()
            ),
            FeasibilityImpact::Satisfied,
            "environment configuration passed structural validation",
            "toolchain.environments.configuration",
            "The canonical EnvironmentConfiguration::validate operation succeeded",
        )?;

        self.validate_platform_observation(request, &mut report)?;

        self.validate_required_toolchains(request, &mut report)?;

        self.record_fingerprint(request, &mut report)?;

        let context = EnvironmentValidationContext::from_request(request);

        if providers.is_empty() {
            report.push_finding(
                CATEGORY_VALIDATION_PROVIDER,
                None,
                FeasibilityImpact::Unknown,
                "no domain-specific validation providers were supplied; \
                 environment feasibility has not been established",
                "toolchain.environments.validation",
                "No authoritative domain validator was invoked",
            )?;
        }

        for provider in providers {
            let identifier = provider.identifier();

            let provider_report = provider
                .validate(&context)
                .map_err(|source| {
                    provider_error(identifier, source)
                })?;

            report.merge_provider_report(
                identifier,
                provider_report,
            )?;

            report.record_check(format!(
                "environment.provider.{identifier}"
            ))?;

            report.provider_count += 1;
        }

        report.record_check("environment.validation")?;

        Ok(report)
    }

    fn preflight_providers(
        &self,
        providers: &[&dyn EnvironmentValidationProvider],
    ) -> Result<()> {
        let mut identifiers = BTreeSet::new();

        for provider in providers {
            let identifier = provider.identifier();

            validate_identifier(
                identifier,
                "environment validation-provider identifier",
            )
            .map_err(|error| {
                error
                    .with_operation(OPERATION_PROVIDER_PREFLIGHT)
                    .with_context(COMPONENT)
            })?;

            if !identifiers.insert(identifier.to_owned()) {
                return Err(
                    invalid_input(format!(
                        "duplicate environment validation-provider identifier: \
                         {identifier}"
                    ))
                    .with_operation(OPERATION_PROVIDER_PREFLIGHT)
                    .with_subject(identifier)
                );
            }
        }

        Ok(())
    }

    fn validate_platform_provider_consistency(
        &self,
        request: &EnvironmentValidationRequest<'_>,
    ) -> Result<()> {
        let Some(observation) = request.platform_observation else {
            return Ok(());
        };

        let Some(configured_provider) =
            request.configuration.platform_provider()
        else {
            // An observation may be supplied independently of a configured
            // preferred provider. The observation remains explicit evidence.
            return Ok(());
        };

        if configured_provider != observation.provider_id() {
            return Err(
                validation_error(format!(
                    "platform observation provider `{}` does not match \
                     explicitly configured provider `{}`",
                    observation.provider_id(),
                    configured_provider,
                ))
                .with_subject(
                    request.configuration.environment_id().to_string()
                )
            );
        }

        Ok(())
    }

    fn validate_platform_observation(
        &self,
        request: &EnvironmentValidationRequest<'_>,
        report: &mut EnvironmentValidationReport,
    ) -> Result<()> {
        match request.platform_observation {
            Some(observation) => {
                report.record_check("environment.platform_observation")?;

                let impact = if request.platform_observation_required {
                    FeasibilityImpact::Satisfied
                } else {
                    FeasibilityImpact::Advisory
                };

                report.push_finding(
                    CATEGORY_PLATFORM_OBSERVATION,
                    Some(
                        observation.provider_id().to_string()
                    ),
                    impact,
                    if request.platform_observation_required {
                        "the required platform observation was supplied by \
                         the explicitly identified provider"
                    } else {
                        "a platform observation was supplied; workload \
                         compatibility remains the responsibility of \
                         domain validators"
                    },
                    "zur.platforms.provider",
                    "A PlatformObservation was supplied by the caller",
                )?;
            }

            None if request.platform_observation_required => {
                report.push_finding(
                    CATEGORY_PLATFORM_OBSERVATION,
                    None,
                    FeasibilityImpact::Unknown,
                    "a platform observation is required but none was supplied",
                    "toolchain.environments.validation",
                    "No canonical ZUR platform observation was attached",
                )?;
            }

            None => {
                report.push_finding(
                    CATEGORY_PLATFORM_OBSERVATION,
                    None,
                    FeasibilityImpact::Advisory,
                    "no platform observation was requested; this is valid \
                     for target-independent validation stages",
                    "toolchain.environments.validation",
                    "Platform observation is optional for this request",
                )?;
            }
        }

        Ok(())
    }

    fn validate_required_toolchains(
        &self,
        request: &EnvironmentValidationRequest<'_>,
        report: &mut EnvironmentValidationReport,
    ) -> Result<()> {
        if request.required_toolchains.is_empty() {
            report.record_check("environment.external_toolchains.not_required")?;

            return Ok(());
        }

        let Some(discovered) = request.discovered_toolchains else {
            for identifier in &request.required_toolchains {
                report.push_finding(
                    CATEGORY_EXTERNAL_TOOLCHAIN,
                    Some(identifier.clone()),
                    FeasibilityImpact::Unknown,
                    format!(
                        "required external toolchain `{identifier}` has not \
                         been checked because no discovery result was supplied"
                    ),
                    "toolchain.environments.discovery",
                    "No DiscoveredToolchains collection was attached",
                )?;
            }

            return Ok(());
        };

        for identifier in &request.required_toolchains {
            match discovered.get(identifier) {
                Some(descriptor) => {
                    report.push_finding(
                        CATEGORY_EXTERNAL_TOOLCHAIN,
                        Some(identifier.clone()),
                        FeasibilityImpact::Satisfied,
                        format!(
                            "required external toolchain `{identifier}` was \
                             discovered and its configured version requirement \
                             was validated by the backend"
                        ),
                        "backend.toolchain",
                        format!(
                            "Discovered descriptor `{}` reports version {}.{}.{}",
                            descriptor.id(),
                            descriptor.version().major,
                            descriptor.version().minor,
                            descriptor.version().patch,
                        ),
                    )?;
                }

                None => {
                    report.push_finding(
                        CATEGORY_EXTERNAL_TOOLCHAIN,
                        Some(identifier.clone()),
                        FeasibilityImpact::Unknown,
                        format!(
                            "required external toolchain `{identifier}` is \
                             absent from the supplied discovery result"
                        ),
                        "toolchain.environments.discovery",
                        "The supplied discovery collection contains no matching identifier",
                    )?;
                }
            }
        }

        report.record_check("environment.external_toolchains")?;

        Ok(())
    }

    fn record_fingerprint(
        &self,
        request: &EnvironmentValidationRequest<'_>,
        report: &mut EnvironmentValidationReport,
    ) -> Result<()> {
        match request.fingerprint {
            Some(fingerprint) => {
                report.record_check("environment.fingerprint")?;

                report.push_finding(
                    CATEGORY_FINGERPRINT,
                    Some(
                        request.configuration.environment_id().to_string()
                    ),
                    FeasibilityImpact::Advisory,
                    format!(
                        "an environment fingerprint was supplied; \
                         compilation identity is {}",
                        fingerprint.compilation_fingerprint()
                    ),
                    "toolchain.environments.fingerprints",
                    "The canonical compiler-owned compilation environment fingerprint was supplied",
                )?;
            }

            None => {
                report.push_finding(
                    CATEGORY_FINGERPRINT,
                    None,
                    FeasibilityImpact::Advisory,
                    "no environment fingerprint was supplied; fingerprint \
                     availability is not itself a feasibility requirement",
                    "toolchain.environments.validation",
                    "The validation request contains no EnvironmentFingerprint",
                )?;
            }
        }

        Ok(())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::ids::{
        EnvironmentId,
        IdNamespace,
    };

    fn configuration() -> EnvironmentConfiguration {
        let namespace = IdNamespace::new(
            "tests.toolchain.environments",
        )
        .expect("valid namespace");

        let environment_id = EnvironmentId::new(
            namespace,
            1,
        )
        .expect("valid environment identity");

        EnvironmentConfiguration::new(environment_id)
    }

    struct SatisfiedProvider;

    impl EnvironmentValidationProvider for SatisfiedProvider {
        fn identifier(&self) -> &str {
            "tests.classical.validator"
        }

        fn validate(
            &self,
            _context: &EnvironmentValidationContext<'_>,
        ) -> Result<FeasibilityReport> {
            let finding = FeasibilityFinding::new(
                "tests.classical.capability",
                Some("classical.execution".to_owned()),
                FeasibilityImpact::Satisfied,
                "the test provider established its declared condition",
            )
            .expect("valid finding");

            let evidence = FeasibilityEvidence::new(
                "tests.classical.validator",
                "deterministic test evidence",
            )
            .expect("valid evidence");

            Ok(
                FeasibilityReport::new()
                    .with_finding(
                        finding.with_evidence(evidence)
                    )
            )
        }
    }

    struct EmptyProvider;

    impl EnvironmentValidationProvider for EmptyProvider {
        fn identifier(&self) -> &str {
            "tests.empty.validator"
        }

        fn validate(
            &self,
            _context: &EnvironmentValidationContext<'_>,
        ) -> Result<FeasibilityReport> {
            Ok(FeasibilityReport::new())
        }
    }

    struct AdvisoryOnlyProvider;

    impl EnvironmentValidationProvider for AdvisoryOnlyProvider {
        fn identifier(&self) -> &str {
            "tests.advisory.validator"
        }

        fn validate(
            &self,
            _context: &EnvironmentValidationContext<'_>,
        ) -> Result<FeasibilityReport> {
            let finding = FeasibilityFinding::new(
                "tests.advisory",
                None,
                FeasibilityImpact::Advisory,
                "optional information only",
            )
            .expect("valid finding");

            Ok(FeasibilityReport::new().with_finding(finding))
        }
    }

    struct FailingProvider;

    impl EnvironmentValidationProvider for FailingProvider {
        fn identifier(&self) -> &str {
            "tests.failing.validator"
        }

        fn validate(
            &self,
            _context: &EnvironmentValidationContext<'_>,
        ) -> Result<FeasibilityReport> {
            Err(ToolchainError::provider_failed(
                "simulated provider failure",
            ))
        }
    }

    #[test]
    fn no_domain_providers_never_establishes_feasibility() {
        let configuration = configuration();

        let request = EnvironmentValidationRequest::new(
            &configuration,
        );

        let report = EnvironmentValidator::new()
            .validate(&request, &[])
            .expect("structurally valid request");

        assert_eq!(
            report.status(),
            FeasibilityStatus::Unknown,
        );

        assert!(!report.is_feasible());
        assert!(report.needs_more_evidence());
        assert_eq!(report.provider_count(), 0);
    }

    #[test]
    fn a_complete_authoritative_provider_can_establish_feasibility() {
        let configuration = configuration();

        let request = EnvironmentValidationRequest::new(
            &configuration,
        );

        let provider = SatisfiedProvider;

        let providers: [&dyn EnvironmentValidationProvider; 1] = [
            &provider,
        ];

        let report = EnvironmentValidator::new()
            .validate(&request, &providers)
            .expect("validation should complete");

        assert_eq!(
            report.status(),
            FeasibilityStatus::Feasible,
        );

        assert!(report.is_complete());
        assert_eq!(report.provider_count(), 1);
        assert!(
            report.completed_checks()
                .contains("environment.provider.tests.classical.validator")
        );
    }

    #[test]
    fn empty_provider_report_is_not_treated_as_success() {
        let configuration = configuration();

        let request = EnvironmentValidationRequest::new(
            &configuration,
        );

        let provider = EmptyProvider;

        let providers: [&dyn EnvironmentValidationProvider; 1] = [
            &provider,
        ];

        let report = EnvironmentValidator::new()
            .validate(&request, &providers)
            .expect("incomplete report is a valid unresolved result");

        assert_eq!(
            report.status(),
            FeasibilityStatus::Unknown,
        );

        assert!(!report.is_feasible());
        assert!(report.needs_more_evidence());
    }

    #[test]
    fn advisory_only_provider_cannot_establish_feasibility() {
        let configuration = configuration();

        let request = EnvironmentValidationRequest::new(
            &configuration,
        );

        let provider = AdvisoryOnlyProvider;

        let providers: [&dyn EnvironmentValidationProvider; 1] = [
            &provider,
        ];

        let report = EnvironmentValidator::new()
            .validate(&request, &providers)
            .expect("advisory-only report is valid");

        assert_eq!(
            report.status(),
            FeasibilityStatus::Unknown,
        );

        assert!(!report.is_feasible());
    }

    #[test]
    fn required_platform_observation_without_evidence_is_unknown() {
        let configuration = configuration();

        let request = EnvironmentValidationRequest::new(
            &configuration,
        )
        .require_platform_observation(true);

        let provider = SatisfiedProvider;

        let providers: [&dyn EnvironmentValidationProvider; 1] = [
            &provider,
        ];

        let report = EnvironmentValidator::new()
            .validate(&request, &providers)
            .expect("missing observation is an assessment result");

        assert_eq!(
            report.status(),
            FeasibilityStatus::Unknown,
        );
    }

    #[test]
    fn optional_platform_observation_does_not_block_portable_validation() {
        let configuration = configuration();

        let request = EnvironmentValidationRequest::new(
            &configuration,
        );

        let provider = SatisfiedProvider;

        let providers: [&dyn EnvironmentValidationProvider; 1] = [
            &provider,
        ];

        let report = EnvironmentValidator::new()
            .validate(&request, &providers)
            .expect("portable validation should complete");

        assert_eq!(
            report.status(),
            FeasibilityStatus::Feasible,
        );
    }

    #[test]
    fn required_toolchain_without_discovery_is_unknown() {
        let configuration = configuration();

        let request = EnvironmentValidationRequest::new(
            &configuration,
        )
        .require_toolchain("org.example.compiler")
        .expect("valid toolchain identifier");

        let provider = SatisfiedProvider;

        let providers: [&dyn EnvironmentValidationProvider; 1] = [
            &provider,
        ];

        let report = EnvironmentValidator::new()
            .validate(&request, &providers)
            .expect("missing discovery is an unresolved result");

        assert_eq!(
            report.status(),
            FeasibilityStatus::Unknown,
        );
    }

    #[test]
    fn duplicate_required_toolchains_are_rejected() {
        let configuration = configuration();

        let result = EnvironmentValidationRequest::new(
            &configuration,
        )
        .require_toolchain("org.example.compiler")
        .expect("first requirement")
        .require_toolchain("org.example.compiler");

        assert!(result.is_err());

        assert_eq!(
            result
                .expect_err("duplicate must be rejected")
                .kind(),
            ToolchainErrorKind::InvalidInput,
        );
    }

    #[test]
    fn duplicate_provider_identifiers_are_rejected_before_validation() {
        let configuration = configuration();

        let request = EnvironmentValidationRequest::new(
            &configuration,
        );

        let first = SatisfiedProvider;
        let second = SatisfiedProvider;

        let providers: [&dyn EnvironmentValidationProvider; 2] = [
            &first,
            &second,
        ];

        let error = EnvironmentValidator::new()
            .validate(&request, &providers)
            .expect_err("duplicate provider identities must be rejected");

        assert_eq!(
            error.kind(),
            ToolchainErrorKind::InvalidInput,
        );

        assert_eq!(
            error.operation(),
            Some(OPERATION_PROVIDER_PREFLIGHT),
        );
    }

    #[test]
    fn provider_failures_remain_errors() {
        let configuration = configuration();

        let request = EnvironmentValidationRequest::new(
            &configuration,
        );

        let provider = FailingProvider;

        let providers: [&dyn EnvironmentValidationProvider; 1] = [
            &provider,
        ];

        let error = EnvironmentValidator::new()
            .validate(&request, &providers)
            .expect_err("provider failure must not become a report");

        assert_eq!(
            error.kind(),
            ToolchainErrorKind::ProviderFailed,
        );

        assert_eq!(
            error.subject(),
            Some("tests.failing.validator"),
        );

        assert!(error.has_source());
    }

    #[test]
    fn report_preserves_the_canonical_feasibility_contract() {
        let configuration = configuration();

        let request = EnvironmentValidationRequest::new(
            &configuration,
        );

        let provider = SatisfiedProvider;

        let providers: [&dyn EnvironmentValidationProvider; 1] = [
            &provider,
        ];

        let report = EnvironmentValidator::new()
            .validate(&request, &providers)
            .expect("validation should complete");

        assert!(
            report.feasibility()
                .findings()
                .iter()
                .any(|finding| {
                    finding.category()
                        == CATEGORY_CONFIGURATION
                        && finding.impact()
                            == FeasibilityImpact::Satisfied
                })
        );

        assert!(
            report.feasibility()
                .findings()
                .iter()
                .any(|finding| {
                    finding.category()
                        == "tests.classical.capability"
                })
        );
    }

    #[test]
    fn validator_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<EnvironmentValidator>();
    }

    #[test]
    fn validation_context_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<EnvironmentValidationContext<'static>>();
    }
}