
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Registry Validation
//!
//! Path: src/toolchain/registries/validation.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the authoritative local validation boundary for configured
//! registry instances and their selected provider implementations.
//!
//! Validation establishes whether:
//!
//! - Registry configuration is structurally valid.
//! - The configured provider identity matches the implementation.
//! - Registry and provider protocol ranges overlap.
//! - Required protocol versions are supported.
//! - Required operations are declared by the provider.
//! - Registry lifecycle state permits the requested operation.
//! - The registry has a usable provider-interpreted location.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Registry/provider compatibility validation.
//! - Registry lifecycle admission validation.
//! - Protocol compatibility validation.
//! - Required operation validation.
//! - Structured validation reports.
//! - Aggregated validation diagnostics.
//!
//! This module does NOT own:
//!
//! - Registry identity allocation.
//! - Provider registration.
//! - Registry discovery.
//! - Network communication.
//! - Authentication.
//! - Authorization.
//! - Protocol serialization.
//! - Dependency resolution.
//! - Package downloading.
//! - Package installation.
//! - Package integrity verification.
//! - Cryptographic signatures.
//! - Publisher trust.
//! - Registry federation.
//! - Runtime or hardware execution.
//!
//! These responsibilities remain with their canonical subsystems.
//!
//! ## Canonical authorities
//!
//! Registry configuration:
//!     super::registry
//!
//! Provider implementation contracts:
//!     super::provider
//!
//! Registry identity:
//!     crate::toolchain::ids
//!
//! Structured errors:
//!     crate::toolchain::errors
//!
//! Protocol versions:
//!     crate::toolchain::version
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Registry instances.
//! - Provider implementations.
//! - Protocol ranges.
//! - Operations.
//! - Computational domains.
//!
//! Collections grow according to available memory.
//!
//! Operational resource budgets belong to the caller.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## Important
//!
//! A successful validation establishes local contract compatibility.
//! It does not prove that a remote registry is reachable, authenticated,
//! authorized, trusted or operational.

use std::fmt;

use crate::toolchain::errors::{
    Result,
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::ids::{
    ProviderId,
    RegistryId,
};

use crate::toolchain::version::{
    RegistryProtocolVersion,
    VersionRange,
};

use super::provider::{
    RegistryOperationId,
    RegistryProviderDescriptor,
};

use super::registry::{
    Registry,
    RegistrySnapshot,
    RegistryStatus,
};

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const OPERATION_VALIDATE: &str = "registry.validation";
const OPERATION_VALIDATE_CONFIGURATION: &str =
    "registry.validation.configuration";
const OPERATION_VALIDATE_PROVIDER: &str =
    "registry.validation.provider";
const OPERATION_VALIDATE_PROTOCOL: &str =
    "registry.validation.protocol";
const OPERATION_VALIDATE_OPERATION: &str =
    "registry.validation.operation";
const OPERATION_VALIDATE_LIFECYCLE: &str =
    "registry.validation.lifecycle";

// ============================================================================
// VALIDATION REQUIREMENTS
// ============================================================================

/// Requirements used when validating a registry configuration.
///
/// Requirements are supplied by the caller according to the operation
/// that intends to use the registry.
///
/// An empty requirement set does not imply universal compatibility.
/// It means that no additional operation-specific requirements were
/// supplied.
///
/// The provider must still satisfy the registry's configured binding.
#[derive(Debug, Clone, Default)]
pub struct RegistryValidationRequirements {
    required_protocol: Option<RegistryProtocolVersion>,
    required_operation: Option<RegistryOperationId>,
    require_enabled: bool,
}

impl RegistryValidationRequirements {
    /// Create the default validation requirements.
    ///
    /// Registries must be enabled to pass lifecycle validation.
    #[must_use]
    pub fn new() -> Self {
        Self {
            required_protocol: None,
            required_operation: None,
            require_enabled: true,
        }
    }

    /// Require a particular registry protocol version.
    #[must_use]
    pub fn protocol(
        mut self,
        protocol: RegistryProtocolVersion,
    ) -> Self {
        self.required_protocol = Some(protocol);
        self
    }

    /// Require a particular registry operation.
    #[must_use]
    pub fn operation(
        mut self,
        operation: RegistryOperationId,
    ) -> Self {
        self.required_operation = Some(operation);
        self
    }

    /// Configure whether the registry must be enabled.
    ///
    /// Retired registries are always rejected.
    ///
    /// Setting this to false permits validation of disabled registry
    /// configurations for administrative inspection. It does not make
    /// disabled registries selectable.
    #[must_use]
    pub fn require_enabled(
        mut self,
        required: bool,
    ) -> Self {
        self.require_enabled = required;
        self
    }

    /// Return the required protocol, if supplied.
    #[must_use]
    pub const fn required_protocol(
        &self,
    ) -> Option<RegistryProtocolVersion> {
        self.required_protocol
    }

    /// Return the required operation, if supplied.
    #[must_use]
    pub fn required_operation(
        &self,
    ) -> Option<&RegistryOperationId> {
        self.required_operation.as_ref()
    }

    /// Return whether an enabled registry is required.
    #[must_use]
    pub const fn requires_enabled(&self) -> bool {
        self.require_enabled
    }
}

// ============================================================================
// VALIDATION ISSUE CLASSIFICATION
// ============================================================================

/// Classification of a registry validation failure.
///
/// These classifications are independent of the explanatory diagnostic
/// message and remain suitable for machine-readable reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum RegistryValidationIssueKind {
    /// Invalid registry configuration.
    InvalidConfiguration,

    /// The provider identity does not match the registry binding.
    ProviderIdentityMismatch,

    /// The configured provider has no overlapping protocol range.
    ProtocolIncompatible,

    /// The requested protocol version is unsupported.
    RequiredProtocolUnsupported,

    /// The requested operation is not declared by the provider.
    OperationUnsupported,

    /// The registry is disabled.
    RegistryDisabled,

    /// The registry has been permanently retired.
    RegistryRetired,

    /// The registry location is invalid.
    InvalidLocation,

    /// A provider descriptor failed validation.
    InvalidProviderDescriptor,
}

impl RegistryValidationIssueKind {
    /// Return the stable classification identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidConfiguration =>
                "registry.invalid_configuration",

            Self::ProviderIdentityMismatch =>
                "registry.provider_identity_mismatch",

            Self::ProtocolIncompatible =>
                "registry.protocol_incompatible",

            Self::RequiredProtocolUnsupported =>
                "registry.required_protocol_unsupported",

            Self::OperationUnsupported =>
                "registry.operation_unsupported",

            Self::RegistryDisabled =>
                "registry.disabled",

            Self::RegistryRetired =>
                "registry.retired",

            Self::InvalidLocation =>
                "registry.invalid_location",

            Self::InvalidProviderDescriptor =>
                "registry.invalid_provider_descriptor",
        }
    }
}

impl fmt::Display for RegistryValidationIssueKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// VALIDATION ISSUE
// ============================================================================

/// One structured registry validation diagnostic.
///
/// The underlying ToolchainError preserves the canonical toolchain
/// error classification and causal context.
#[derive(Debug, Clone)]
pub struct RegistryValidationIssue {
    kind: RegistryValidationIssueKind,
    error: ToolchainError,
}

impl RegistryValidationIssue {
    fn new(
        kind: RegistryValidationIssueKind,
        error: ToolchainError,
    ) -> Self {
        Self { kind, error }
    }

    /// Return the validation issue classification.
    #[must_use]
    pub const fn kind(&self) -> RegistryValidationIssueKind {
        self.kind
    }

    /// Return the underlying structured toolchain error.
    #[must_use]
    pub fn error(&self) -> &ToolchainError {
        &self.error
    }

    /// Return the stable classification identifier.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.kind.as_str()
    }
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Complete result of validating one registry/provider relationship.
///
/// A report may contain multiple independent issues so that callers
/// can correct several configuration problems in one iteration.
///
/// A valid report establishes local compatibility only.
///
/// It does not establish remote availability or trust.
#[derive(Debug, Clone)]
pub struct RegistryValidationReport {
    registry_id: RegistryId,
    provider_id: ProviderId,
    issues: Vec<RegistryValidationIssue>,
}

impl RegistryValidationReport {
    fn new(
        registry_id: RegistryId,
        provider_id: ProviderId,
        issues: Vec<RegistryValidationIssue>,
    ) -> Self {
        Self {
            registry_id,
            provider_id,
            issues,
        }
    }

    /// Return the validated registry identity.
    #[must_use]
    pub fn registry_id(&self) -> &RegistryId {
        &self.registry_id
    }

    /// Return the provider identity examined during validation.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Return all validation issues.
    #[must_use]
    pub fn issues(&self) -> &[RegistryValidationIssue] {
        &self.issues
    }

    /// Return whether the registry/provider relationship is valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.issues.is_empty()
    }

    /// Return whether validation found any issues.
    #[must_use]
    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }

    /// Return the number of validation issues.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Consume the report and return success or the first validation error.
    ///
    /// Use `issues()` when all diagnostics must be inspected.
    pub fn into_result(self) -> Result<Self> {
        if let Some(issue) = self.issues.first() {
            return Err(issue.error.clone());
        }

        Ok(self)
    }

    /// Return the first validation error, if one exists.
    #[must_use]
    pub fn first_error(&self) -> Option<&ToolchainError> {
        self.issues.first().map(|issue| &issue.error)
    }
}

// ============================================================================
// VALIDATION ENTRY POINTS
// ============================================================================

/// Validate a registry configuration against a provider descriptor.
///
/// This is the principal validation entry point.
///
/// The method validates:
///
/// 1. Registry identity and provider binding.
/// 2. Provider descriptor invariants.
/// 3. Protocol-range compatibility.
/// 4. Required protocol compatibility.
/// 5. Required operation support.
/// 6. Registry lifecycle state.
/// 7. Generic registry location validity.
///
/// The method does not execute provider code or communicate with a
/// registry.
///
/// Multiple independent failures are collected into one report.
#[must_use]
pub fn validate_registry(
    registry: &Registry,
    descriptor: &RegistryProviderDescriptor,
    requirements: &RegistryValidationRequirements,
) -> RegistryValidationReport {
    let registry_id = registry.id().clone();
    let provider_id = registry.provider().provider_id().clone();

    let mut issues = Vec::new();

    validate_location(registry, &mut issues);

    validate_lifecycle(
        registry,
        requirements,
        &mut issues,
    );

    validate_provider_descriptor(
        descriptor,
        &mut issues,
    );

    validate_provider_identity(
        registry,
        descriptor,
        &mut issues,
    );

    validate_protocol_compatibility(
        registry,
        descriptor,
        &mut issues,
    );

    validate_required_protocol(
        registry,
        descriptor,
        requirements,
        &mut issues,
    );

    validate_required_operation(
        descriptor,
        requirements,
        &mut issues,
    );

    RegistryValidationReport::new(
        registry_id,
        provider_id,
        issues,
    )
}

/// Validate an immutable registry snapshot against a provider descriptor.
///
/// This convenience entry point ensures callers can validate the exact
/// configuration revision obtained from RegistryCatalog.
#[must_use]
pub fn validate_registry_snapshot(
    snapshot: &RegistrySnapshot,
    descriptor: &RegistryProviderDescriptor,
    requirements: &RegistryValidationRequirements,
) -> RegistryValidationReport {
    validate_registry(
        snapshot.registry(),
        descriptor,
        requirements,
    )
}

/// Validate a registry/provider relationship and require success.
///
/// Unlike `validate_registry`, this function returns the first structured
/// error rather than an aggregated report.
///
/// Use this when a caller must reject an operation immediately.
pub fn ensure_registry_valid(
    registry: &Registry,
    descriptor: &RegistryProviderDescriptor,
    requirements: &RegistryValidationRequirements,
) -> Result<()> {
    let report = validate_registry(
        registry,
        descriptor,
        requirements,
    );

    if let Some(issue) = report.first_error() {
        return Err(issue.clone());
    }

    Ok(())
}

// ============================================================================
// LOCATION VALIDATION
// ============================================================================

fn validate_location(
    registry: &Registry,
    issues: &mut Vec<RegistryValidationIssue>,
) {
    let location = registry.location().as_str();

    if location.trim().is_empty()
        || location.chars().any(char::is_control)
    {
        push_issue(
            issues,
            RegistryValidationIssueKind::InvalidLocation,
            ToolchainErrorKind::InvalidConfiguration,
            OPERATION_VALIDATE_CONFIGURATION,
            registry,
            "Registry location is empty or contains control characters.",
        );
    }
}

// ============================================================================
// LIFECYCLE VALIDATION
// ============================================================================

fn validate_lifecycle(
    registry: &Registry,
    requirements: &RegistryValidationRequirements,
    issues: &mut Vec<RegistryValidationIssue>,
) {
    match registry.status() {
        RegistryStatus::Retired => {
            push_issue(
                issues,
                RegistryValidationIssueKind::RegistryRetired,
                ToolchainErrorKind::ValidationFailed,
                OPERATION_VALIDATE_LIFECYCLE,
                registry,
                "A retired registry cannot participate in operations.",
            );
        }

        RegistryStatus::Disabled
            if requirements.requires_enabled() =>
        {
            push_issue(
                issues,
                RegistryValidationIssueKind::RegistryDisabled,
                ToolchainErrorKind::ValidationFailed,
                OPERATION_VALIDATE_LIFECYCLE,
                registry,
                "The registry is disabled and cannot be selected.",
            );
        }

        RegistryStatus::Disabled
        | RegistryStatus::Enabled => {}
    }
}

// ============================================================================
// PROVIDER DESCRIPTOR VALIDATION
// ============================================================================

fn validate_provider_descriptor(
    descriptor: &RegistryProviderDescriptor,
    issues: &mut Vec<RegistryValidationIssue>,
) {
    if let Err(error) = descriptor.validate() {
        issues.push(RegistryValidationIssue::new(
            RegistryValidationIssueKind::InvalidProviderDescriptor,
            error
                .with_operation(OPERATION_VALIDATE_PROVIDER)
                .with_subject(
                    descriptor.provider_id().to_string(),
                )
                .with_context(
                    "The provider descriptor failed its canonical validation.",
                ),
        ));
    }
}

// ============================================================================
// PROVIDER IDENTITY VALIDATION
// ============================================================================

fn validate_provider_identity(
    registry: &Registry,
    descriptor: &RegistryProviderDescriptor,
    issues: &mut Vec<RegistryValidationIssue>,
) {
    let configured = registry.provider().provider_id();
    let actual = descriptor.provider_id();

    if configured != actual {
        push_issue(
            issues,
            RegistryValidationIssueKind::ProviderIdentityMismatch,
            ToolchainErrorKind::ValidationFailed,
            OPERATION_VALIDATE_PROVIDER,
            registry,
            &format!(
                "Configured provider '{}' does not match registered provider '{}'.",
                configured,
                actual,
            ),
        );
    }
}

// ============================================================================
// PROTOCOL RANGE VALIDATION
// ============================================================================

fn validate_protocol_compatibility(
    registry: &Registry,
    descriptor: &RegistryProviderDescriptor,
    issues: &mut Vec<RegistryValidationIssue>,
) {
    let configured_ranges =
        registry.provider().supported_protocols();

    let provider_ranges =
        descriptor.supported_protocols();

    if configured_ranges.is_empty() || provider_ranges.is_empty() {
        push_issue(
            issues,
            RegistryValidationIssueKind::ProtocolIncompatible,
            ToolchainErrorKind::IncompatibleVersion,
            OPERATION_VALIDATE_PROTOCOL,
            registry,
            "Registry and provider must declare protocol version ranges.",
        );

        return;
    }

    let compatible = configured_ranges.iter().any(
        |configured| {
            provider_ranges.iter().any(
                |provided| ranges_overlap(configured, provided),
            )
        },
    );

    if !compatible {
        push_issue(
            issues,
            RegistryValidationIssueKind::ProtocolIncompatible,
            ToolchainErrorKind::IncompatibleVersion,
            OPERATION_VALIDATE_PROTOCOL,
            registry,
            "Registry and provider have no overlapping protocol version ranges.",
        );
    }
}

/// Determine whether two inclusive protocol version ranges overlap.
///
/// Unbounded minimum and maximum values are supported.
///
/// This function does not assume a fixed protocol version catalogue.
fn ranges_overlap(
    left: &VersionRange<RegistryProtocolVersion>,
    right: &VersionRange<RegistryProtocolVersion>,
) -> bool {
    let left_before_right =
        match (left.maximum(), right.minimum()) {
            (Some(left_max), Some(right_min)) => {
                left_max < right_min
            }
            _ => false,
        };

    let right_before_left =
        match (right.maximum(), left.minimum()) {
            (Some(right_max), Some(left_min)) => {
                right_max < left_min
            }
            _ => false,
        };

    !left_before_right && !right_before_left
}

// ============================================================================
// REQUIRED PROTOCOL VALIDATION
// ============================================================================

fn validate_required_protocol(
    registry: &Registry,
    descriptor: &RegistryProviderDescriptor,
    requirements: &RegistryValidationRequirements,
    issues: &mut Vec<RegistryValidationIssue>,
) {
    let Some(required) = requirements.required_protocol() else {
        return;
    };

    let registry_supports =
        registry.provider().supports_protocol(required);

    let provider_supports =
        descriptor.supports_protocol(required);

    if !registry_supports || !provider_supports {
        push_issue(
            issues,
            RegistryValidationIssueKind::RequiredProtocolUnsupported,
            ToolchainErrorKind::IncompatibleVersion,
            OPERATION_VALIDATE_PROTOCOL,
            registry,
            &format!(
                "Required registry protocol version '{}' is not supported by both the registry binding and provider.",
                required,
            ),
        );
    }
}

// ============================================================================
// REQUIRED OPERATION VALIDATION
// ============================================================================

fn validate_required_operation(
    descriptor: &RegistryProviderDescriptor,
    requirements: &RegistryValidationRequirements,
    issues: &mut Vec<RegistryValidationIssue>,
) {
    let Some(required) = requirements.required_operation() else {
        return;
    };

    if !descriptor.supports_operation(required) {
        issues.push(RegistryValidationIssue::new(
            RegistryValidationIssueKind::OperationUnsupported,
            ToolchainError::new(
                ToolchainErrorKind::UnsupportedOperation,
                format!(
                    "Registry provider does not declare support for operation '{}'.",
                    required.as_str(),
                ),
            )
            .with_operation(OPERATION_VALIDATE_OPERATION)
            .with_subject(
                descriptor.provider_id().to_string(),
            )
            .with_context(
                format!("Required operation: {}", required.as_str()),
            ),
        ));
    }
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn push_issue(
    issues: &mut Vec<RegistryValidationIssue>,
    issue_kind: RegistryValidationIssueKind,
    error_kind: ToolchainErrorKind,
    operation: &str,
    registry: &Registry,
    message: &str,
) {
    issues.push(RegistryValidationIssue::new(
        issue_kind,
        ToolchainError::new(error_kind, message)
            .with_operation(operation)
            .with_subject(registry.id().to_string())
            .with_context(
                format!(
                    "Registry validation classification: {}",
                    issue_kind.as_str(),
                ),
            ),
    ));
}

// ============================================================================
// DISPLAY
// ============================================================================

impl fmt::Display for RegistryValidationReport {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if self.is_valid() {
            return write!(
                formatter,
                "Registry '{}' is locally compatible with provider '{}'.",
                self.registry_id,
                self.provider_id,
            );
        }

        writeln!(
            formatter,
            "Registry '{}' failed validation against provider '{}':",
            self.registry_id,
            self.provider_id,
        )?;

        for issue in &self.issues {
            writeln!(
                formatter,
                "- {}: {}",
                issue.code(),
                issue.error(),
            )?;
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

    use std::collections::BTreeSet;

    use crate::toolchain::ids::{
        IdNamespace,
        ProviderId,
        RegistryId,
    };

    use crate::toolchain::version::{
        ProviderApiVersion,
        RegistryProtocolVersion,
    };

    use super::super::provider::{
        RegistryOperationId,
        RegistryProviderDescriptor,
    };

    use super::super::registry::{
        RegistryLocation,
        RegistryName,
        RegistryProviderBinding,
    };

    const TEST_NAMESPACE: &str = "zamani.tests";

    fn registry_id(value: u128) -> RegistryId {
        RegistryId::new(
            IdNamespace::new(TEST_NAMESPACE)
                .expect("valid namespace"),
            value,
        )
        .expect("valid registry identity")
    }

    fn provider_id(value: u128) -> ProviderId {
        ProviderId::new(
            IdNamespace::new(TEST_NAMESPACE)
                .expect("valid namespace"),
            value,
        )
        .expect("valid provider identity")
    }

    fn protocol(
        major: u32,
        minor: u32,
        patch: u32,
    ) -> RegistryProtocolVersion {
        RegistryProtocolVersion::new(
            major,
            minor,
            patch,
        )
    }

    fn operation(name: &str) -> RegistryOperationId {
        RegistryOperationId::new(name)
            .expect("valid operation")
    }

    fn descriptor(
        id: ProviderId,
        protocols: Vec<
            VersionRange<RegistryProtocolVersion>,
        >,
        operations: BTreeSet<RegistryOperationId>,
    ) -> RegistryProviderDescriptor {
        RegistryProviderDescriptor::new(
            id,
            "Test registry provider",
            ProviderApiVersion::new(1, 0, 0),
            protocols,
            operations,
            BTreeSet::new(),
        )
        .expect("valid provider descriptor")
    }

    fn registry(
        id: u128,
        provider: u128,
        protocols: Vec<
            VersionRange<RegistryProtocolVersion>,
        >,
    ) -> Registry {
        let binding = RegistryProviderBinding::new(
            provider_id(provider),
            protocols,
        )
        .expect("valid provider binding");

        Registry::new(
            registry_id(id),
            RegistryName::new(format!("Registry {id}"))
                .expect("valid name"),
            RegistryLocation::new(
                format!("local:registry/{id}"),
            )
            .expect("valid location"),
            binding,
        )
    }

    fn enable(registry: Registry) -> Registry {
        let catalog = crate::toolchain::registries::registry::RegistryCatalog::new();

        let snapshot = catalog
            .register(registry)
            .expect("registration");

        catalog
            .set_status(
                snapshot.registry().id(),
                snapshot.registry().revision(),
                RegistryStatus::Enabled,
            )
            .expect("enable")
            .registry()
            .clone()
    }

    fn any_protocol() -> VersionRange<RegistryProtocolVersion> {
        VersionRange::any()
    }

    // ------------------------------------------------------------------------
    // Identity
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_matching_provider_identity() {
        let registry = enable(registry(
            1,
            10,
            vec![any_protocol()],
        ));

        let descriptor = descriptor(
            provider_id(10),
            vec![any_protocol()],
            BTreeSet::from([
                operation("packages.query"),
            ]),
        );

        let report = validate_registry(
            &registry,
            &descriptor,
            &RegistryValidationRequirements::new(),
        );

        assert!(report.is_valid());
    }

    #[test]
    fn rejects_provider_identity_mismatch() {
        let registry = enable(registry(
            1,
            10,
            vec![any_protocol()],
        ));

        let descriptor = descriptor(
            provider_id(20),
            vec![any_protocol()],
            BTreeSet::from([
                operation("packages.query"),
            ]),
        );

        let report = validate_registry(
            &registry,
            &descriptor,
            &RegistryValidationRequirements::new(),
        );

        assert!(report.issues().iter().any(
            |issue| issue.kind()
                == RegistryValidationIssueKind::ProviderIdentityMismatch
        ));
    }

    // ------------------------------------------------------------------------
    // Protocol compatibility
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_overlapping_protocol_ranges() {
        let registry = enable(registry(
            1,
            10,
            vec![
                VersionRange::new(
                    protocol(1, 0, 0),
                    protocol(3, 0, 0),
                )
                .expect("valid range"),
            ],
        ));

        let descriptor = descriptor(
            provider_id(10),
            vec![
                VersionRange::new(
                    protocol(2, 0, 0),
                    protocol(4, 0, 0),
                )
                .expect("valid range"),
            ],
            BTreeSet::from([
                operation("packages.query"),
            ]),
        );

        let report = validate_registry(
            &registry,
            &descriptor,
            &RegistryValidationRequirements::new(),
        );

        assert!(report.is_valid());
    }

    #[test]
    fn rejects_non_overlapping_protocol_ranges() {
        let registry = enable(registry(
            1,
            10,
            vec![
                VersionRange::new(
                    protocol(1, 0, 0),
                    protocol(1, 9, 9),
                )
                .expect("valid range"),
            ],
        ));

        let descriptor = descriptor(
            provider_id(10),
            vec![
                VersionRange::new(
                    protocol(2, 0, 0),
                    protocol(3, 0, 0),
                )
                .expect("valid range"),
            ],
            BTreeSet::from([
                operation("packages.query"),
            ]),
        );

        let report = validate_registry(
            &registry,
            &descriptor,
            &RegistryValidationRequirements::new(),
        );

        assert!(report.issues().iter().any(
            |issue| issue.kind()
                == RegistryValidationIssueKind::ProtocolIncompatible
        ));
    }

    #[test]
    fn accepts_unbounded_protocol_ranges() {
        let registry = enable(registry(
            1,
            10,
            vec![VersionRange::any()],
        ));

        let descriptor = descriptor(
            provider_id(10),
            vec![VersionRange::any()],
            BTreeSet::from([
                operation("packages.query"),
            ]),
        );

        let report = validate_registry(
            &registry,
            &descriptor,
            &RegistryValidationRequirements::new(),
        );

        assert!(report.is_valid());
    }

    #[test]
    fn rejects_unsupported_required_protocol() {
        let registry = enable(registry(
            1,
            10,
            vec![
                VersionRange::new(
                    protocol(1, 0, 0),
                    protocol(1, 9, 9),
                )
                .expect("valid range"),
            ],
        ));

        let descriptor = descriptor(
            provider_id(10),
            vec![VersionRange::any()],
            BTreeSet::from([
                operation("packages.query"),
            ]),
        );

        let report = validate_registry(
            &registry,
            &descriptor,
            &RegistryValidationRequirements::new()
                .protocol(protocol(2, 0, 0)),
        );

        assert!(report.issues().iter().any(
            |issue| issue.kind()
                == RegistryValidationIssueKind::RequiredProtocolUnsupported
        ));
    }

    // ------------------------------------------------------------------------
    // Operation compatibility
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_declared_operation() {
        let registry = enable(registry(
            1,
            10,
            vec![any_protocol()],
        ));

        let descriptor = descriptor(
            provider_id(10),
            vec![any_protocol()],
            BTreeSet::from([
                operation("packages.publish"),
                operation("packages.query"),
            ]),
        );

        let report = validate_registry(
            &registry,
            &descriptor,
            &RegistryValidationRequirements::new()
                .operation(operation("packages.publish")),
        );

        assert!(report.is_valid());
    }

    #[test]
    fn rejects_undeclared_operation() {
        let registry = enable(registry(
            1,
            10,
            vec![any_protocol()],
        ));

        let descriptor = descriptor(
            provider_id(10),
            vec![any_protocol()],
            BTreeSet::from([
                operation("packages.query"),
            ]),
        );

        let report = validate_registry(
            &registry,
            &descriptor,
            &RegistryValidationRequirements::new()
                .operation(operation("packages.publish")),
        );

        assert!(report.issues().iter().any(
            |issue| issue.kind()
                == RegistryValidationIssueKind::OperationUnsupported
        ));
    }

    // ------------------------------------------------------------------------
    // Lifecycle
    // ------------------------------------------------------------------------

    #[test]
    fn rejects_disabled_registry_by_default() {
        let registry = registry(
            1,
            10,
            vec![any_protocol()],
        );

        let descriptor = descriptor(
            provider_id(10),
            vec![any_protocol()],
            BTreeSet::from([
                operation("packages.query"),
            ]),
        );

        let report = validate_registry(
            &registry,
            &descriptor,
            &RegistryValidationRequirements::new(),
        );

        assert!(report.issues().iter().any(
            |issue| issue.kind()
                == RegistryValidationIssueKind::RegistryDisabled
        ));
    }

    #[test]
    fn permits_disabled_registry_for_administrative_validation() {
        let registry = registry(
            1,
            10,
            vec![any_protocol()],
        );

        let descriptor = descriptor(
            provider_id(10),
            vec![any_protocol()],
            BTreeSet::from([
                operation("packages.query"),
            ]),
        );

        let report = validate_registry(
            &registry,
            &descriptor,
            &RegistryValidationRequirements::new()
                .require_enabled(false),
        );

        assert!(report.is_valid());
    }

    #[test]
    fn always_rejects_retired_registry() {
        let catalog =
            crate::toolchain::registries::registry::RegistryCatalog::new();

        let snapshot = catalog
            .register(registry(
                1,
                10,
                vec![any_protocol()],
            ))
            .expect("registration");

        let retired = catalog
            .set_status(
                snapshot.registry().id(),
                snapshot.registry().revision(),
                RegistryStatus::Retired,
            )
            .expect("retirement");

        let descriptor = descriptor(
            provider_id(10),
            vec![any_protocol()],
            BTreeSet::from([
                operation("packages.query"),
            ]),
        );

        let report = validate_registry(
            retired.registry(),
            &descriptor,
            &RegistryValidationRequirements::new()
                .require_enabled(false),
        );

        assert!(report.issues().iter().any(
            |issue| issue.kind()
                == RegistryValidationIssueKind::RegistryRetired
        ));
    }

    // ------------------------------------------------------------------------
    // Aggregated diagnostics
    // ------------------------------------------------------------------------

    #[test]
    fn collects_multiple_independent_validation_issues() {
        let registry = registry(
            1,
            10,
            vec![
                VersionRange::new(
                    protocol(1, 0, 0),
                    protocol(1, 9, 9),
                )
                .expect("valid range"),
            ],
        );

        let descriptor = descriptor(
            provider_id(20),
            vec![
                VersionRange::new(
                    protocol(2, 0, 0),
                    protocol(3, 0, 0),
                )
                .expect("valid range"),
            ],
            BTreeSet::new(),
        );

        let report = validate_registry(
            &registry,
            &descriptor,
            &RegistryValidationRequirements::new()
                .protocol(protocol(4, 0, 0))
                .operation(operation("packages.publish")),
        );

        assert!(report.issue_count() >= 4);
        assert!(!report.is_valid());
    }

    #[test]
    fn report_preserves_registry_and_provider_identities() {
        let registry = enable(registry(
            1,
            10,
            vec![any_protocol()],
        ));

        let descriptor = descriptor(
            provider_id(10),
            vec![any_protocol()],
            BTreeSet::from([
                operation("packages.query"),
            ]),
        );

        let report = validate_registry(
            &registry,
            &descriptor,
            &RegistryValidationRequirements::new(),
        );

        assert_eq!(
            report.registry_id(),
            registry.id(),
        );

        assert_eq!(
            report.provider_id(),
            descriptor.provider_id(),
        );
    }

    #[test]
    fn ensure_registry_valid_returns_success_for_valid_registry() {
        let registry = enable(registry(
            1,
            10,
            vec![any_protocol()],
        ));

        let descriptor = descriptor(
            provider_id(10),
            vec![any_protocol()],
            BTreeSet::from([
                operation("packages.query"),
            ]),
        );

        assert!(
            ensure_registry_valid(
                &registry,
                &descriptor,
                &RegistryValidationRequirements::new(),
            )
            .is_ok()
        );
    }

    #[test]
    fn ensure_registry_valid_returns_structured_failure() {
        let registry = registry(
            1,
            10,
            vec![any_protocol()],
        );

        let descriptor = descriptor(
            provider_id(10),
            vec![any_protocol()],
            BTreeSet::from([
                operation("packages.query"),
            ]),
        );

        let result = ensure_registry_valid(
            &registry,
            &descriptor,
            &RegistryValidationRequirements::new(),
        );

        assert!(result.is_err());
    }
}
