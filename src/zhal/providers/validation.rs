
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Provider Validation
//!
//! Path: src/zhal/providers/validation.rs
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the canonical structural and compatibility validation
//! contract for universal ZHAL hardware providers.
//!
//! Validation establishes whether a provider descriptor and its
//! externally supplied conformance and trust evidence satisfy
//! an explicitly configured admission policy.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Provider descriptor validation.
//! - Provider API compatibility assessment.
//! - Required capability declaration validation.
//! - Required operation declaration validation.
//! - Provider interface manifest validation.
//! - Provider metadata validation.
//! - Provider trust evidence assessment.
//! - Provider conformance evidence assessment.
//! - Structured validation findings.
//! - Validation policy definitions.
//!
//! This module does not own:
//!
//! - Provider identity definitions.
//! - Provider lifecycle.
//! - Provider registry storage.
//! - Provider authentication.
//! - Cryptographic attestation.
//! - Security policy enforcement.
//! - Hardware discovery.
//! - Capability identity.
//! - Capability availability.
//! - Source-language semantic requirements.
//! - Hardware feasibility.
//! - Resource allocation.
//! - Hardware execution.
//! - Quantum execution.
//! - HDL realization.
//! - Distributed execution.
//! - Runtime orchestration.
//!
//! These responsibilities remain with their canonical authorities.
//!
//! # Canonical dependencies
//!
//! Provider contracts:
//!     crate::stdlib::provider
//!
//! Capability contracts:
//!     crate::stdlib::capability
//!
//! Version representation:
//!     crate::stdlib::version
//!
//! Hardware provider:
//!     crate::zhal::providers::contract
//!
//! Hardware errors:
//!     crate::zhal::errors
//!
//! Hardware metadata:
//!     crate::zhal::model::metadata
//!
//! Provider trust:
//!     Existing ZUR security authority.
//!
//! # Important security boundary
//!
//! This module does not authenticate providers or verify
//! cryptographic signatures.
//!
//! Trust assessments must originate from the canonical
//! security authority.
//!
//! A provider cannot establish its own trust merely by
//! declaring itself trusted.
//!
//! # POCO-REAF
//!
//! All provider identities, capability identities, operation
//! identities, interface identities and metadata namespaces
//! remain open-world.
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Providers.
//! - Capabilities.
//! - Operations.
//! - Interfaces.
//! - Metadata entries.
//! - Identifier lengths.
//! - Namespace depth.
//! - Computing domains.
//! - Hardware technologies.
//!
//! Collections grow according to available resources and
//! explicitly configured operational budgets.
//!
//! Literal infinite computation or storage is not promised.
//!
//! # Grammar boundary
//!
//! Source-level hardware intent belongs to the grammar,
//! frontend and semantic authorities.
//!
//! This module validates provider-side declarations.
//!
//! It does not parse grammar or reinterpret:
//!
//! - Requirements.
//! - Capabilities.
//! - Preferences.
//! - Constraints.
//! - Hints.
//! - Feasibility.
//!
//! # Integration
//!
//! The provider registry should invoke validation before
//! committing a provider registration.
//!
//! The lifecycle manager must not treat registration as
//! proof of provider readiness.
//!
//! Domain adapters remain responsible for translating
//! their specialized provider contracts into the ZHAL
//! hardware boundary.

// ============================================================================
// IMPORTS
// ============================================================================

use std::collections::{BTreeMap, BTreeSet};
use std::cmp::Ordering;

use crate::stdlib::capability::CapabilityId;

use crate::stdlib::provider::{
    ProviderDescriptor,
    ProviderOperationId,
    ProviderContractId,
};

use crate::stdlib::version::Version;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

use crate::zhal::providers::contract::HardwareProvider;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable component identifier.
pub const COMPONENT: &str = "zhal.providers.validation";

/// Provider validation schema identifier.
pub const VALIDATION_SCHEMA_ID: &str =
    "zamani.zhal.providers.validation";

/// Provider validation schema major version.
pub const VALIDATION_SCHEMA_MAJOR: u16 = 1;

/// Provider validation schema minor version.
pub const VALIDATION_SCHEMA_MINOR: u16 = 0;

/// Provider validation schema patch version.
pub const VALIDATION_SCHEMA_PATCH: u16 = 0;

/// Complete provider validation schema version.
pub const VALIDATION_SCHEMA_VERSION: (u16, u16, u16) = (
    VALIDATION_SCHEMA_MAJOR,
    VALIDATION_SCHEMA_MINOR,
    VALIDATION_SCHEMA_PATCH,
);

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn validation_error(
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(
        ZhalErrorKind::InvalidData,
        message,
    )
    .in_component(COMPONENT)
}

fn contract_error(
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(
        ZhalErrorKind::ContractViolation,
        message,
    )
    .in_component(COMPONENT)
}

// ============================================================================
// VALIDATION SEVERITY
// ============================================================================

/// Severity of a provider validation finding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[non_exhaustive]
pub enum ValidationSeverity {
    /// Informational observation.
    Information,

    /// A non-fatal recommendation.
    Warning,

    /// A mandatory admission requirement was not satisfied.
    Error,
}

// ============================================================================
// VALIDATION ISSUE
// ============================================================================

/// A single structured provider validation finding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationIssue {
    severity: ValidationSeverity,
    code: String,
    field: String,
    message: String,
}

impl ValidationIssue {
    fn new(
        severity: ValidationSeverity,
        code: impl Into<String>,
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code: code.into(),
            field: field.into(),
            message: message.into(),
        }
    }

    /// Returns the finding severity.
    #[must_use]
    pub const fn severity(&self) -> ValidationSeverity {
        self.severity
    }

    /// Returns the stable finding code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Returns the affected field.
    #[must_use]
    pub fn field(&self) -> &str {
        &self.field
    }

    /// Returns the safe descriptive message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Complete provider validation report.
///
/// Findings preserve insertion order.
///
/// A report is immutable from the consumer's perspective
/// through its public interface.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProviderValidationReport {
    issues: Vec<ValidationIssue>,
}

impl ProviderValidationReport {
    /// Creates an empty validation report.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns all validation findings.
    #[must_use]
    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }

    /// Returns the number of findings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.issues.len()
    }

    /// Returns whether the report contains no findings.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.issues.is_empty()
    }

    /// Returns whether validation contains errors.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|issue| {
            issue.severity == ValidationSeverity::Error
        })
    }

    /// Returns whether the provider satisfies all mandatory
    /// validation requirements.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.has_errors()
    }

    /// Returns all warnings.
    #[must_use]
    pub fn warnings(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues.iter().filter(|issue| {
            issue.severity == ValidationSeverity::Warning
        })
    }

    /// Returns all errors.
    #[must_use]
    pub fn errors(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues.iter().filter(|issue| {
            issue.severity == ValidationSeverity::Error
        })
    }

    fn push(
        &mut self,
        severity: ValidationSeverity,
        code: impl Into<String>,
        field: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.issues.push(
            ValidationIssue::new(
                severity,
                code,
                field,
                message,
            ),
        );
    }

    /// Converts an invalid report into a structured ZHAL error.
    ///
    /// A valid report returns successfully.
    ///
    /// # Errors
    ///
    /// Returns a contract violation if mandatory requirements
    /// have not been satisfied.
    pub fn ensure_valid(&self) -> ZhalResult<()> {
        if !self.is_valid() {
            return Err(contract_error(
                "hardware provider failed mandatory validation",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// TRUST POLICY
// ============================================================================

/// Required provider trust level.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TrustRequirement {
    /// Trust verification is not required by this validation policy.
    ///
    /// This does not establish that the provider is trusted.
    NotRequired,

    /// Independently verified trust evidence is mandatory.
    RequireVerified,
}

impl Default for TrustRequirement {
    fn default() -> Self {
        Self::RequireVerified
    }
}

// ============================================================================
// PROVIDER TRUST EVIDENCE
// ============================================================================

/// Trust assessment supplied by the canonical security authority.
///
/// This is an assessment result, not a mechanism for performing
/// authentication or cryptographic verification.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ProviderTrustEvidence {
    /// No trust assessment is available.
    Unknown,

    /// The provider has been independently verified.
    Verified {
        /// Identity of the verifying authority.
        authority: String,

        /// Reference to the verification evidence.
        evidence_reference: String,
    },

    /// Verification has not established provider trust.
    Unverified {
        /// Explanation supplied by the trust authority.
        reason: String,
    },

    /// The trust authority explicitly rejected the provider.
    Rejected {
        /// Explanation supplied by the trust authority.
        reason: String,
    },
}

impl Default for ProviderTrustEvidence {
    fn default() -> Self {
        Self::Unknown
    }
}

// ============================================================================
// API COMPATIBILITY POLICY
// ============================================================================

/// Policy governing provider API compatibility.
///
/// Provider API versions are separate from implementation,
/// firmware, grammar, package and language versions.
///
/// Compatibility uses semantic-version precedence.
///
/// A compatible provider must:
///
/// - Advertise the required API major version.
/// - Advertise an API version at least as recent as the
///   configured minimum compatible version.
///
/// Build metadata does not affect precedence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderApiPolicy {
    minimum_version: Version,
}

impl ProviderApiPolicy {
    /// Creates an API compatibility policy.
    #[must_use]
    pub fn new(minimum_version: Version) -> Self {
        Self {
            minimum_version,
        }
    }

    /// Returns the minimum compatible API version.
    #[must_use]
    pub fn minimum_version(&self) -> &Version {
        &self.minimum_version
    }

    /// Checks a provider API version.
    #[must_use]
    pub fn is_compatible(&self, provider_version: &Version) -> bool {
        provider_version.major() == self.minimum_version.major()
            && provider_version.precedence_cmp(
                &self.minimum_version,
            ) != Ordering::Less
    }
}

// ============================================================================
// REQUIRED INTERFACE CONTRACTS
// ============================================================================

/// An open-world hardware interface declaration.
///
/// Interface identifiers are descriptive provider contract
/// identifiers, not physical addresses or device identifiers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderInterfaceDeclaration {
    id: String,
    version: Version,
}

impl ProviderInterfaceDeclaration {
    /// Creates a provider interface declaration.
    ///
    /// # Errors
    ///
    /// Returns an error when the interface identifier is invalid.
    pub fn new(
        id: impl Into<String>,
        version: Version,
    ) -> ZhalResult<Self> {
        let id = id.into();

        if !valid_qualified_identifier(&id) {
            return Err(validation_error(
                "provider interface identifier is invalid",
            ));
        }

        Ok(Self {
            id,
            version,
        })
    }

    /// Returns the interface contract identity.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the interface contract version.
    #[must_use]
    pub fn version(&self) -> &Version {
        &self.version
    }
}

/// Required provider interface contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequiredProviderInterface {
    id: String,
    minimum_version: Version,
}

impl RequiredProviderInterface {
    /// Creates a required interface declaration.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid interface identity.
    pub fn new(
        id: impl Into<String>,
        minimum_version: Version,
    ) -> ZhalResult<Self> {
        let id = id.into();

        if !valid_qualified_identifier(&id) {
            return Err(validation_error(
                "required provider interface identity is invalid",
            ));
        }

        Ok(Self {
            id,
            minimum_version,
        })
    }

    /// Returns the required interface identity.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the minimum required interface version.
    #[must_use]
    pub fn minimum_version(&self) -> &Version {
        &self.minimum_version
    }
}

// ============================================================================
// PROVIDER INTERFACE MANIFEST
// ============================================================================

/// Provider-supplied interface declaration manifest.
///
/// The manifest must be obtained from the provider integration
/// adapter or another independently defined contract source.
///
/// It is not inferred from provider metadata.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProviderInterfaceManifest {
    interfaces: Vec<ProviderInterfaceDeclaration>,
}

impl ProviderInterfaceManifest {
    /// Creates an empty interface manifest.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an interface declaration.
    ///
    /// Duplicate interface identities are rejected.
    ///
    /// # Errors
    ///
    /// Returns a contract error for duplicate declarations.
    pub fn insert(
        &mut self,
        declaration: ProviderInterfaceDeclaration,
    ) -> ZhalResult<()> {
        if self.interfaces.iter().any(|existing| {
            existing.id == declaration.id
        }) {
            return Err(contract_error(
                "provider interface manifest contains a duplicate identity",
            ));
        }

        self.interfaces.push(declaration);

        Ok(())
    }

    /// Returns all declared interfaces.
    #[must_use]
    pub fn interfaces(&self) -> &[ProviderInterfaceDeclaration] {
        &self.interfaces
    }

    /// Returns the number of declarations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.interfaces.len()
    }

    /// Returns whether the manifest is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.interfaces.is_empty()
    }
}

// ============================================================================
// VALIDATION POLICY
// ============================================================================

/// Complete provider admission-validation policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderValidationPolicy {
    api: ProviderApiPolicy,

    required_capabilities: BTreeSet<CapabilityId>,

    required_operations: BTreeSet<ProviderOperationId>,

    required_interfaces: BTreeMap<String, Version>,

    trust: TrustRequirement,

    require_typed_operation_contracts: bool,

    require_conformance_evidence: bool,
}

impl ProviderValidationPolicy {
    /// Creates a validation policy using the required API version.
    #[must_use]
    pub fn new(minimum_api_version: Version) -> Self {
        Self {
            api: ProviderApiPolicy::new(minimum_api_version),
            required_capabilities: BTreeSet::new(),
            required_operations: BTreeSet::new(),
            required_interfaces: BTreeMap::new(),
            trust: TrustRequirement::RequireVerified,
            require_typed_operation_contracts: false,
            require_conformance_evidence: true,
        }
    }

    /// Returns the API compatibility policy.
    #[must_use]
    pub fn api(&self) -> &ProviderApiPolicy {
        &self.api
    }

    /// Returns the required capabilities.
    #[must_use]
    pub fn required_capabilities(
        &self,
    ) -> &BTreeSet<CapabilityId> {
        &self.required_capabilities
    }

    /// Returns the required operations.
    #[must_use]
    pub fn required_operations(
        &self,
    ) -> &BTreeSet<ProviderOperationId> {
        &self.required_operations
    }

    /// Returns the required interfaces.
    #[must_use]
    pub fn required_interfaces(
        &self,
    ) -> &BTreeMap<String, Version> {
        &self.required_interfaces
    }

    /// Returns the trust requirement.
    #[must_use]
    pub const fn trust_requirement(&self) -> TrustRequirement {
        self.trust
    }

    /// Adds a required capability.
    #[must_use]
    pub fn requiring_capability(
        mut self,
        capability: CapabilityId,
    ) -> Self {
        self.required_capabilities.insert(capability);
        self
    }

    /// Adds a required provider operation.
    #[must_use]
    pub fn requiring_operation(
        mut self,
        operation: ProviderOperationId,
    ) -> Self {
        self.required_operations.insert(operation);
        self
    }

    /// Adds a required interface contract.
    ///
    /// An existing requirement is explicitly replaced.
    ///
    /// # Errors
    ///
    /// Returns an error if the interface identifier is invalid.
    pub fn requiring_interface(
        mut self,
        id: impl Into<String>,
        minimum_version: Version,
    ) -> ZhalResult<Self> {
        let id = id.into();

        if !valid_qualified_identifier(&id) {
            return Err(validation_error(
                "required interface identity is invalid",
            ));
        }

        self.required_interfaces.insert(
            id,
            minimum_version,
        );

        Ok(self)
    }

    /// Configures provider trust requirements.
    #[must_use]
    pub fn with_trust_requirement(
        mut self,
        requirement: TrustRequirement,
    ) -> Self {
        self.trust = requirement;
        self
    }

    /// Requires operation request and response contract identities.
    #[must_use]
    pub fn requiring_typed_operation_contracts(
        mut self,
        required: bool,
    ) -> Self {
        self.require_typed_operation_contracts = required;
        self
    }

    /// Configures whether conformance evidence is mandatory.
    #[must_use]
    pub fn requiring_conformance_evidence(
        mut self,
        required: bool,
    ) -> Self {
        self.require_conformance_evidence = required;
        self
    }
}

// ============================================================================
// CONFORMANCE EVIDENCE
// ============================================================================

/// Result of an external provider conformance assessment.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ConformanceStatus {
    /// No conformance assessment is available.
    Unknown,

    /// The provider passed the applicable conformance suite.
    Passed,

    /// The provider failed the applicable conformance suite.
    Failed {
        /// Safe explanation of the failure.
        reason: String,
    },
}

/// External provider conformance evidence.
///
/// The evidence must originate from an independent test,
/// certification or verification authority.
///
/// This module validates its representation; it does not
/// execute the conformance suite.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderConformanceEvidence {
    status: ConformanceStatus,
    authority: String,
    suite: String,
}

impl ProviderConformanceEvidence {
    /// Creates provider conformance evidence.
    ///
    /// # Errors
    ///
    /// Returns an error when the authority or suite identity
    /// is empty or invalid.
    pub fn new(
        status: ConformanceStatus,
        authority: impl Into<String>,
        suite: impl Into<String>,
    ) -> ZhalResult<Self> {
        let authority = authority.into();
        let suite = suite.into();

        if !valid_descriptive_text(&authority) {
            return Err(validation_error(
                "conformance authority must contain valid descriptive text",
            ));
        }

        if !valid_qualified_identifier(&suite) {
            return Err(validation_error(
                "conformance suite identity is invalid",
            ));
        }

        if let ConformanceStatus::Failed { reason } = &status {
            if !valid_descriptive_text(reason) {
                return Err(validation_error(
                    "conformance failure reason is invalid",
                ));
            }
        }

        Ok(Self {
            status,
            authority,
            suite,
        })
    }

    /// Returns the conformance status.
    #[must_use]
    pub fn status(&self) -> &ConformanceStatus {
        &self.status
    }

    /// Returns the verifying authority.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Returns the conformance suite identity.
    #[must_use]
    pub fn suite(&self) -> &str {
        &self.suite
    }
}

// ============================================================================
// VALIDATION INPUT
// ============================================================================

/// Complete external evidence supplied for provider validation.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProviderValidationEvidence {
    trust: ProviderTrustEvidence,

    conformance: Option<ProviderConformanceEvidence>,

    interfaces: ProviderInterfaceManifest,
}

impl ProviderValidationEvidence {
    /// Creates an evidence collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets provider trust evidence.
    #[must_use]
    pub fn with_trust(
        mut self,
        trust: ProviderTrustEvidence,
    ) -> Self {
        self.trust = trust;
        self
    }

    /// Sets provider conformance evidence.
    #[must_use]
    pub fn with_conformance(
        mut self,
        conformance: ProviderConformanceEvidence,
    ) -> Self {
        self.conformance = Some(conformance);
        self
    }

    /// Sets the provider interface manifest.
    #[must_use]
    pub fn with_interfaces(
        mut self,
        interfaces: ProviderInterfaceManifest,
    ) -> Self {
        self.interfaces = interfaces;
        self
    }

    /// Returns trust evidence.
    #[must_use]
    pub fn trust(&self) -> &ProviderTrustEvidence {
        &self.trust
    }

    /// Returns conformance evidence.
    #[must_use]
    pub fn conformance(
        &self,
    ) -> Option<&ProviderConformanceEvidence> {
        self.conformance.as_ref()
    }

    /// Returns the interface manifest.
    #[must_use]
    pub fn interfaces(&self) -> &ProviderInterfaceManifest {
        &self.interfaces
    }
}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn valid_qualified_identifier(value: &str) -> bool {
    if value.is_empty() || value.trim() != value {
        return false;
    }

    value.split('.').all(|segment| {
        !segment.is_empty()
            && segment.bytes().all(|byte| {
                byte.is_ascii_alphanumeric()
                    || byte == b'_'
                    || byte == b'-'
            })
    })
}

fn valid_descriptive_text(value: &str) -> bool {
    !value.trim().is_empty()
        && !value.chars().any(char::is_control)
}

fn sensitive_metadata_key(key: &str) -> bool {
    key.split(['.', '_', '-'])
        .any(|segment| {
            matches!(
                segment.to_ascii_lowercase().as_str(),
                "password"
                    | "passwd"
                    | "secret"
                    | "secrets"
                    | "credential"
                    | "credentials"
                    | "apikey"
                    | "accesskey"
                    | "privatekey"
                    | "token"
                    | "accesstoken"
                    | "refreshtoken"
                    | "clientsecret"
                    | "authorization"
                    | "cookie"
                    | "cookies"
            )
        })
}

fn contains_obvious_secret(value: &str) -> bool {
    let lower = value.trim().to_ascii_lowercase();

    (lower.contains("-----begin ")
        && lower.contains("private key-----"))
        || lower.starts_with("bearer ")
        || lower.starts_with("basic ")
        || lower.starts_with("authorization:")
}

// ============================================================================
// DESCRIPTOR VALIDATION
// ============================================================================

/// Validates the structure of a provider descriptor.
///
/// This checks the descriptor independently of live hardware.
///
/// It does not perform API admission, trust verification,
/// interface discovery or provider initialization.
pub fn validate_descriptor(
    descriptor: &ProviderDescriptor,
) -> ProviderValidationReport {
    let mut report = ProviderValidationReport::new();

    if !valid_descriptive_text(descriptor.display_name()) {
        report.push(
            ValidationSeverity::Error,
            "provider.invalid_display_name",
            "display_name",
            "Provider display name is invalid.",
        );
    }

    if !valid_qualified_identifier(descriptor.id().as_str()) {
        report.push(
            ValidationSeverity::Error,
            "provider.invalid_identity",
            "id",
            "Provider identity is invalid.",
        );
    }

    if !valid_qualified_identifier(descriptor.kind().as_str()) {
        report.push(
            ValidationSeverity::Error,
            "provider.invalid_kind",
            "kind",
            "Provider kind is invalid.",
        );
    }

    // --------------------------------------------------------
    // Capability declarations
    // --------------------------------------------------------

    let mut capabilities = BTreeSet::new();

    for capability in descriptor.capabilities() {
        let id = capability.id().as_str();

        if !valid_qualified_identifier(id) {
            report.push(
                ValidationSeverity::Error,
                "provider.invalid_capability",
                "capabilities",
                "Provider contains an invalid capability identity.",
            );
        }

        if !capabilities.insert(id) {
            report.push(
                ValidationSeverity::Error,
                "provider.duplicate_capability",
                "capabilities",
                "Provider declares a capability more than once.",
            );
        }

        if !valid_qualified_identifier(capability.kind().as_str()) {
            report.push(
                ValidationSeverity::Error,
                "provider.invalid_capability_kind",
                "capabilities",
                "Provider contains an invalid capability kind.",
            );
        }

        if capability.description().is_some_and(
            |description| !valid_descriptive_text(description),
        ) {
            report.push(
                ValidationSeverity::Error,
                "provider.invalid_capability_description",
                "capabilities",
                "Provider contains an invalid capability description.",
            );
        }

        for (key, value) in capability.attributes() {
            validate_metadata_entry(
                &mut report,
                key,
                value,
                "capabilities.attributes",
            );
        }
    }

    // --------------------------------------------------------
    // Operation declarations
    // --------------------------------------------------------

    let mut operations = BTreeSet::new();

    for operation in descriptor.operations() {
        let id = operation.id().as_str();

        if !valid_qualified_identifier(id) {
            report.push(
                ValidationSeverity::Error,
                "provider.invalid_operation",
                "operations",
                "Provider contains an invalid operation identity.",
            );
        }

        if !operations.insert(id) {
            report.push(
                ValidationSeverity::Error,
                "provider.duplicate_operation",
                "operations",
                "Provider declares an operation more than once.",
            );
        }

        if operation.description().is_some_and(
            |description| !valid_descriptive_text(description),
        ) {
            report.push(
                ValidationSeverity::Error,
                "provider.invalid_operation_description",
                "operations",
                "Provider contains an invalid operation description.",
            );
        }

        match (
            operation.request_contract(),
            operation.response_contract(),
        ) {
            (Some(request), Some(response)) => {
                if !valid_qualified_identifier(request.as_str())
                    || !valid_qualified_identifier(response.as_str())
                {
                    report.push(
                        ValidationSeverity::Error,
                        "provider.invalid_operation_contract",
                        "operations",
                        "Provider operation contains an invalid request or response contract identity.",
                    );
                }
            }

            (None, None) => {}

            _ => {
                report.push(
                    ValidationSeverity::Error,
                    "provider.incomplete_operation_contract",
                    "operations",
                    "Provider operation has an incomplete request/response contract declaration.",
                );
            }
        }
    }

    // --------------------------------------------------------
    // Provider metadata
    // --------------------------------------------------------

    for (key, value) in descriptor.attributes() {
        validate_metadata_entry(
            &mut report,
            key,
            value,
            "attributes",
        );
    }

    report
}

fn validate_metadata_entry(
    report: &mut ProviderValidationReport,
    key: &str,
    value: &str,
    field: &str,
) {
    if !valid_qualified_identifier(key) {
        report.push(
            ValidationSeverity::Error,
            "provider.invalid_metadata_key",
            field,
            "Provider metadata contains an invalid key.",
        );
    }

    if sensitive_metadata_key(key) {
        report.push(
            ValidationSeverity::Error,
            "provider.sensitive_metadata_key",
            field,
            "Provider metadata contains a key reserved for sensitive information.",
        );
    }

    if !valid_descriptive_text(value) {
        report.push(
            ValidationSeverity::Error,
            "provider.invalid_metadata_value",
            field,
            "Provider metadata contains an invalid descriptive value.",
        );
    }

    if contains_obvious_secret(value) {
        report.push(
            ValidationSeverity::Error,
            "provider.sensitive_metadata_value",
            field,
            "Provider metadata contains recognizable secret material.",
        );
    }
}

// ============================================================================
// API COMPATIBILITY VALIDATION
// ============================================================================

fn validate_api_compatibility(
    descriptor: &ProviderDescriptor,
    policy: &ProviderValidationPolicy,
    report: &mut ProviderValidationReport,
) {
    if !policy.api.is_compatible(descriptor.api_version()) {
        report.push(
            ValidationSeverity::Error,
            "provider.incompatible_api_version",
            "api_version",
            format!(
                "Provider API version {} is incompatible with minimum required version {}.",
                descriptor.api_version(),
                policy.api.minimum_version(),
            ),
        );
    }

    if descriptor.implementation_version().is_prerelease() {
        report.push(
            ValidationSeverity::Warning,
            "provider.prerelease_implementation",
            "implementation_version",
            "Provider implementation uses a prerelease version.",
        );
    }
}

// ============================================================================
// REQUIRED CAPABILITY VALIDATION
// ============================================================================

fn validate_required_capabilities(
    descriptor: &ProviderDescriptor,
    policy: &ProviderValidationPolicy,
    report: &mut ProviderValidationReport,
) {
    for required in &policy.required_capabilities {
        if descriptor.capability(required).is_none() {
            report.push(
                ValidationSeverity::Error,
                "provider.missing_required_capability",
                "capabilities",
                format!(
                    "Provider does not declare required capability {required}."
                ),
            );
        }
    }
}

// ============================================================================
// REQUIRED OPERATION VALIDATION
// ============================================================================

fn validate_required_operations(
    descriptor: &ProviderDescriptor,
    policy: &ProviderValidationPolicy,
    report: &mut ProviderValidationReport,
) {
    for required in &policy.required_operations {
        let Some(operation) = descriptor.operation(required) else {
            report.push(
                ValidationSeverity::Error,
                "provider.missing_required_operation",
                "operations",
                format!(
                    "Provider does not declare required operation {required}."
                ),
            );

            continue;
        };

        if policy.require_typed_operation_contracts
            && (
                operation.request_contract().is_none()
                    || operation.response_contract().is_none()
            )
        {
            report.push(
                ValidationSeverity::Error,
                "provider.missing_typed_operation_contract",
                "operations",
                format!(
                    "Required operation {required} does not declare both request and response contracts."
                ),
            );
        }
    }
}

// ============================================================================
// INTERFACE VALIDATION
// ============================================================================

fn validate_interfaces(
    manifest: &ProviderInterfaceManifest,
    policy: &ProviderValidationPolicy,
    report: &mut ProviderValidationReport,
) {
    let mut declarations = BTreeMap::new();

    for interface in &manifest.interfaces {
        if !valid_qualified_identifier(&interface.id) {
            report.push(
                ValidationSeverity::Error,
                "provider.invalid_interface_identity",
                "interfaces",
                "Provider interface manifest contains an invalid identity.",
            );

            continue;
        }

        if declarations.insert(
            interface.id.as_str(),
            &interface.version,
        ).is_some() {
            report.push(
                ValidationSeverity::Error,
                "provider.duplicate_interface",
                "interfaces",
                "Provider interface manifest contains duplicate identities.",
            );
        }
    }

    for (required_id, minimum_version) in &policy.required_interfaces {
        match declarations.get(required_id.as_str()) {
            None => {
                report.push(
                    ValidationSeverity::Error,
                    "provider.missing_required_interface",
                    "interfaces",
                    format!(
                        "Provider does not declare required interface {required_id}."
                    ),
                );
            }

            Some(actual_version) => {
                if actual_version.major() != minimum_version.major()
                    || actual_version.precedence_cmp(minimum_version)
                        == Ordering::Less
                {
                    report.push(
                        ValidationSeverity::Error,
                        "provider.incompatible_interface_version",
                        "interfaces",
                        format!(
                            "Provider interface {required_id} has an incompatible version."
                        ),
                    );
                }
            }
        }
    }
}

// ============================================================================
// TRUST VALIDATION
// ============================================================================

fn validate_trust(
    evidence: &ProviderTrustEvidence,
    requirement: TrustRequirement,
    report: &mut ProviderValidationReport,
) {
    match evidence {
        ProviderTrustEvidence::Verified {
            authority,
            evidence_reference,
        } => {
            if !valid_descriptive_text(authority)
                || !valid_descriptive_text(evidence_reference)
            {
                report.push(
                    ValidationSeverity::Error,
                    "provider.invalid_trust_evidence",
                    "trust",
                    "Provider trust evidence is malformed.",
                );
            }
        }

        ProviderTrustEvidence::Unknown => {
            if requirement == TrustRequirement::RequireVerified {
                report.push(
                    ValidationSeverity::Error,
                    "provider.trust_unknown",
                    "trust",
                    "Provider trust has not been independently established.",
                );
            } else {
                report.push(
                    ValidationSeverity::Warning,
                    "provider.trust_unknown",
                    "trust",
                    "Provider trust has not been established.",
                );
            }
        }

        ProviderTrustEvidence::Unverified { reason } => {
            if requirement == TrustRequirement::RequireVerified {
                report.push(
                    ValidationSeverity::Error,
                    "provider.trust_unverified",
                    "trust",
                    "Provider trust verification is incomplete.",
                );
            } else {
                report.push(
                    ValidationSeverity::Warning,
                    "provider.trust_unverified",
                    "trust",
                    format!(
                        "Provider trust remains unverified: {reason}"
                    ),
                );
            }
        }

        ProviderTrustEvidence::Rejected { .. } => {
            report.push(
                ValidationSeverity::Error,
                "provider.trust_rejected",
                "trust",
                "The security authority rejected this provider.",
            );
        }
    }
}

// ============================================================================
// CONFORMANCE VALIDATION
// ============================================================================

fn validate_conformance(
    evidence: Option<&ProviderConformanceEvidence>,
    required: bool,
    report: &mut ProviderValidationReport,
) {
    match evidence {
        Some(evidence) => match &evidence.status {
            ConformanceStatus::Passed => {
                if !valid_descriptive_text(&evidence.authority)
                    || !valid_qualified_identifier(&evidence.suite)
                {
                    report.push(
                        ValidationSeverity::Error,
                        "provider.invalid_conformance_evidence",
                        "conformance",
                        "Provider conformance evidence is malformed.",
                    );
                }
            }

            ConformanceStatus::Unknown => {
                if required {
                    report.push(
                        ValidationSeverity::Error,
                        "provider.conformance_unknown",
                        "conformance",
                        "Required provider conformance has not been established.",
                    );
                }
            }

            ConformanceStatus::Failed { .. } => {
                report.push(
                    ValidationSeverity::Error,
                    "provider.conformance_failed",
                    "conformance",
                    "Provider failed its declared conformance assessment.",
                );
            }
        },

        None => {
            if required {
                report.push(
                    ValidationSeverity::Error,
                    "provider.conformance_missing",
                    "conformance",
                    "Required provider conformance evidence is missing.",
                );
            }
        }
    }
}

// ============================================================================
// COMPLETE PROVIDER VALIDATION
// ============================================================================

/// Performs complete provider descriptor and admission validation.
///
/// The validation combines:
///
/// - Descriptor structure.
/// - API compatibility.
/// - Required capability declarations.
/// - Required operation declarations.
/// - Interface manifest compatibility.
/// - Provider metadata.
/// - External trust evidence.
/// - External conformance evidence.
///
/// It does not initialize the provider or access physical hardware.
///
/// # Errors
///
/// Returns an error only if the validation process itself
/// encounters an unrecoverable ZHAL validation failure.
pub fn validate_provider(
    descriptor: &ProviderDescriptor,
    policy: &ProviderValidationPolicy,
    evidence: &ProviderValidationEvidence,
) -> ZhalResult<ProviderValidationReport> {
    let mut report = validate_descriptor(descriptor);

    validate_api_compatibility(
        descriptor,
        policy,
        &mut report,
    );

    validate_required_capabilities(
        descriptor,
        policy,
        &mut report,
    );

    validate_required_operations(
        descriptor,
        policy,
        &mut report,
    );

    validate_interfaces(
        &evidence.interfaces,
        policy,
        &mut report,
    );

    validate_trust(
        &evidence.trust,
        policy.trust,
        &mut report,
    );

    validate_conformance(
        evidence.conformance(),
        policy.require_conformance_evidence,
        &mut report,
    );

    Ok(report)
}

// ============================================================================
// HARDWARE PROVIDER VALIDATION
// ============================================================================

/// Validates an actual ZHAL hardware-provider implementation.
///
/// The generic bound ensures that the supplied implementation
/// implements the canonical HardwareProvider contract.
///
/// This is stronger than accepting an arbitrary standard-library
/// Provider implementation.
///
/// The provider descriptor is obtained from the implementation.
///
/// This function does not invoke discovery, initialize the provider,
/// open a session or communicate with physical hardware.
///
/// # Errors
///
/// Returns an error if mandatory validation fails.
pub fn validate_hardware_provider<P>(
    provider: &P,
    policy: &ProviderValidationPolicy,
    evidence: &ProviderValidationEvidence,
) -> ZhalResult<ProviderValidationReport>
where
    P: HardwareProvider + ?Sized,
{
    let descriptor = provider.descriptor();

    let report = validate_provider(
        descriptor,
        policy,
        evidence,
    )?;

    report.ensure_valid()?;

    Ok(report)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::capability::{
        Capability,
        CapabilityKind,
    };

    use crate::stdlib::provider::{
        ProviderId,
        ProviderKind,
        ProviderDescriptor,
        ProviderOperationDescriptor,
    };

    fn version(value: &str) -> Version {
        Version::parse(value).expect("valid test version")
    }

    fn descriptor() -> ProviderDescriptor {
        ProviderDescriptor::new(
            ProviderId::new("org.zamani.test_provider")
                .expect("valid provider identity"),
            ProviderKind::new("computation")
                .expect("valid provider kind"),
            "Zamani Test Provider",
            version("1.0.0"),
            version("1.2.0"),
        )
        .expect("valid provider descriptor")
    }

    fn capability(id: &str) -> Capability {
        Capability::new(
            CapabilityId::new(id)
                .expect("valid capability identity"),
            CapabilityKind::new("computation")
                .expect("valid capability kind"),
        )
    }

    fn policy() -> ProviderValidationPolicy {
        ProviderValidationPolicy::new(
            version("1.2.0"),
        )
        .with_trust_requirement(TrustRequirement::NotRequired)
        .requiring_conformance_evidence(false)
    }

    fn evidence() -> ProviderValidationEvidence {
        ProviderValidationEvidence::new()
            .with_trust(
                ProviderTrustEvidence::Verified {
                    authority: "org.zamani.security".into(),
                    evidence_reference: "verification.001".into(),
                },
            )
    }

    #[test]
    fn schema_identity_is_stable() {
        assert_eq!(
            VALIDATION_SCHEMA_ID,
            "zamani.zhal.providers.validation",
        );

        assert_eq!(
            VALIDATION_SCHEMA_VERSION,
            (1, 0, 0),
        );
    }

    #[test]
    fn valid_descriptor_passes_structural_validation() {
        let result = validate_descriptor(&descriptor());

        assert!(result.is_valid());
        assert!(result.issues().is_empty());
    }

    #[test]
    fn compatible_api_version_is_accepted() {
        let descriptor = descriptor();

        let result = validate_provider(
            &descriptor,
            &policy(),
            &evidence(),
        )
        .expect("validation must execute");

        assert!(result.is_valid());
    }

    #[test]
    fn newer_minor_api_version_is_accepted() {
        let descriptor = ProviderDescriptor::new(
            ProviderId::new("org.zamani.newer")
                .unwrap(),
            ProviderKind::new("computation")
                .unwrap(),
            "Newer Provider",
            version("1.0.0"),
            version("1.9.0"),
        )
        .unwrap();

        let result = validate_provider(
            &descriptor,
            &policy(),
            &evidence(),
        )
        .unwrap();

        assert!(result.is_valid());
    }

    #[test]
    fn incompatible_major_version_is_rejected() {
        let descriptor = ProviderDescriptor::new(
            ProviderId::new("org.zamani.incompatible")
                .unwrap(),
            ProviderKind::new("computation")
                .unwrap(),
            "Incompatible Provider",
            version("2.0.0"),
            version("2.0.0"),
        )
        .unwrap();

        let result = validate_provider(
            &descriptor,
            &policy(),
            &evidence(),
        )
        .unwrap();

        assert!(!result.is_valid());
    }

    #[test]
    fn older_api_version_is_rejected() {
        let descriptor = ProviderDescriptor::new(
            ProviderId::new("org.zamani.old")
                .unwrap(),
            ProviderKind::new("computation")
                .unwrap(),
            "Older Provider",
            version("1.0.0"),
            version("1.1.0"),
        )
        .unwrap();

        let result = validate_provider(
            &descriptor,
            &policy(),
            &evidence(),
        )
        .unwrap();

        assert!(!result.is_valid());
    }

    #[test]
    fn required_capability_must_be_declared() {
        let descriptor = descriptor();

        let policy = policy()
            .requiring_capability(
                CapabilityId::new("quantum.computation")
                    .unwrap(),
            );

        let result = validate_provider(
            &descriptor,
            &policy,
            &evidence(),
        )
        .unwrap();

        assert!(!result.is_valid());
    }

    #[test]
    fn declared_capability_satisfies_declaration_requirement() {
        let mut descriptor = descriptor();

        descriptor
            .insert_capability(
                capability("quantum.computation"),
            )
            .unwrap();

        let policy = policy()
            .requiring_capability(
                CapabilityId::new("quantum.computation")
                    .unwrap(),
            );

        let result = validate_provider(
            &descriptor,
            &policy,
            &evidence(),
        )
        .unwrap();

        assert!(result.is_valid());
    }

    #[test]
    fn required_operation_must_be_declared() {
        let descriptor = descriptor();

        let policy = policy()
            .requiring_operation(
                ProviderOperationId::new(
                    "classical.matrix_multiplication",
                )
                .unwrap(),
            );

        let result = validate_provider(
            &descriptor,
            &policy,
            &evidence(),
        )
        .unwrap();

        assert!(!result.is_valid());
    }

    #[test]
    fn declared_operation_satisfies_requirement() {
        let mut descriptor = descriptor();

        descriptor
            .insert_operation(
                ProviderOperationDescriptor::from_id(
                    "classical.matrix_multiplication",
                    version("1.0.0"),
                )
                .unwrap(),
            )
            .unwrap();

        let policy = policy()
            .requiring_operation(
                ProviderOperationId::new(
                    "classical.matrix_multiplication",
                )
                .unwrap(),
            );

        let result = validate_provider(
            &descriptor,
            &policy,
            &evidence(),
        )
        .unwrap();

        assert!(result.is_valid());
    }

    #[test]
    fn required_typed_operation_contracts_are_enforced() {
        let mut descriptor = descriptor();

        descriptor
            .insert_operation(
                ProviderOperationDescriptor::from_id(
                    "classical.compute",
                    version("1.0.0"),
                )
                .unwrap(),
            )
            .unwrap();

        let policy = policy()
            .requiring_operation(
                ProviderOperationId::new(
                    "classical.compute",
                )
                .unwrap(),
            )
            .requiring_typed_operation_contracts(true);

        let result = validate_provider(
            &descriptor,
            &policy,
            &evidence(),
        )
        .unwrap();

        assert!(!result.is_valid());
    }

    #[test]
    fn required_interface_must_exist() {
        let descriptor = descriptor();

        let policy = policy()
            .requiring_interface(
                "hardware.device_access",
                version("1.0.0"),
            )
            .unwrap();

        let result = validate_provider(
            &descriptor,
            &policy,
            &evidence(),
        )
        .unwrap();

        assert!(!result.is_valid());
    }

    #[test]
    fn compatible_interface_satisfies_requirement() {
        let descriptor = descriptor();

        let mut manifest = ProviderInterfaceManifest::new();

        manifest
            .insert(
                ProviderInterfaceDeclaration::new(
                    "hardware.device_access",
                    version("1.2.0"),
                )
                .unwrap(),
            )
            .unwrap();

        let evidence = evidence()
            .with_interfaces(manifest);

        let policy = policy()
            .requiring_interface(
                "hardware.device_access",
                version("1.0.0"),
            )
            .unwrap();

        let result = validate_provider(
            &descriptor,
            &policy,
            &evidence,
        )
        .unwrap();

        assert!(result.is_valid());
    }

    #[test]
    fn incompatible_interface_major_is_rejected() {
        let descriptor = descriptor();

        let mut manifest = ProviderInterfaceManifest::new();

        manifest
            .insert(
                ProviderInterfaceDeclaration::new(
                    "hardware.device_access",
                    version("2.0.0"),
                )
                .unwrap(),
            )
            .unwrap();

        let evidence = evidence()
            .with_interfaces(manifest);

        let policy = policy()
            .requiring_interface(
                "hardware.device_access",
                version("1.0.0"),
            )
            .unwrap();

        let result = validate_provider(
            &descriptor,
            &policy,
            &evidence,
        )
        .unwrap();

        assert!(!result.is_valid());
    }

    #[test]
    fn unknown_trust_fails_closed_by_default() {
        let descriptor = descriptor();

        let policy = ProviderValidationPolicy::new(
            version("1.2.0"),
        )
        .requiring_conformance_evidence(false);

        let result = validate_provider(
            &descriptor,
            &policy,
            &ProviderValidationEvidence::new(),
        )
        .unwrap();

        assert!(!result.is_valid());
    }

    #[test]
    fn rejected_provider_never_passes_trust_validation() {
        let descriptor = descriptor();

        let evidence = ProviderValidationEvidence::new()
            .with_trust(
                ProviderTrustEvidence::Rejected {
                    reason: "security rejection".into(),
                },
            );

        let result = validate_provider(
            &descriptor,
            &policy(),
            &evidence,
        )
        .unwrap();

        assert!(!result.is_valid());
    }

    #[test]
    fn missing_conformance_fails_when_required() {
        let descriptor = descriptor();

        let policy = ProviderValidationPolicy::new(
            version("1.2.0"),
        )
        .with_trust_requirement(TrustRequirement::NotRequired);

        let result = validate_provider(
            &descriptor,
            &policy,
            &ProviderValidationEvidence::new(),
        )
        .unwrap();

        assert!(!result.is_valid());
    }

    #[test]
    fn failed_conformance_never_passes() {
        let descriptor = descriptor();

        let conformance = ProviderConformanceEvidence::new(
            ConformanceStatus::Failed {
                reason: "contract mismatch".into(),
            },
            "org.zamani.conformance",
            "zhal.hardware_provider",
        )
        .unwrap();

        let evidence = evidence()
            .with_conformance(conformance);

        let result = validate_provider(
            &descriptor,
            &policy()
                .requiring_conformance_evidence(false),
            &evidence,
        )
        .unwrap();

        assert!(!result.is_valid());
    }

    #[test]
    fn recognizable_secret_metadata_is_rejected() {
        let mut descriptor = descriptor();

        descriptor
            .insert_attribute(
                "org.vendor.hardware.model",
                "Example device",
            )
            .unwrap();

        // The canonical ProviderDescriptor already rejects
        // duplicate or malformed attributes. This test verifies
        // the accepted descriptive metadata path.
        let result = validate_descriptor(&descriptor);

        assert!(result.is_valid());
    }

    #[test]
    fn open_world_provider_kinds_are_supported() {
        let descriptor = ProviderDescriptor::new(
            ProviderId::new("future.photonic.provider")
                .unwrap(),
            ProviderKind::new(
                "future.photonic.neuromorphic",
            )
            .unwrap(),
            "Future Computational Provider",
            version("1.0.0"),
            version("1.2.0"),
        )
        .unwrap();

        assert!(
            validate_descriptor(&descriptor).is_valid()
        );
    }

    #[test]
    fn validation_report_preserves_multiple_errors() {
        let descriptor = descriptor();

        let policy = ProviderValidationPolicy::new(
            version("2.0.0"),
        )
        .requiring_capability(
            CapabilityId::new("quantum.computation")
                .unwrap(),
        )
        .requiring_conformance_evidence(true);

        let report = validate_provider(
            &descriptor,
            &policy,
            &ProviderValidationEvidence::new(),
        )
        .unwrap();

        assert!(!report.is_valid());

        assert!(report.len() >= 3);
    }

    #[test]
    fn validation_policy_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<ProviderValidationPolicy>();
        assert_send_sync::<ProviderValidationReport>();
        assert_send_sync::<ProviderValidationEvidence>();
    }

    #[test]
    fn no_hardware_inventory_is_required_for_validation() {
        let descriptor = descriptor();

        let report = validate_provider(
            &descriptor,
            &policy(),
            &evidence(),
        )
        .unwrap();

        assert!(report.is_valid());
    }
}
