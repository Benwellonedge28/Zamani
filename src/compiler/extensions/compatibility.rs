
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler Extension Compatibility
//!
//! Path: `src/compiler/extensions/compatibility.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Evaluates whether an extension provider's declared contract is
//! compatible with the host compiler and the available provider
//! environment.
//!
//! This module is the canonical compiler-extension compatibility
//! boundary.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Provider compatibility requests.
//! - Provider API compatibility evaluation.
//! - Dependency compatibility evaluation.
//! - Contribution compatibility evaluation.
//! - Extension-point compatibility evaluation.
//! - Compatibility findings.
//! - Deterministic compatibility reports.
//! - Provider compatibility decisions.
//!
//! This module does not own:
//!
//! - Provider identity.
//! - Provider version definitions.
//! - Provider registration.
//! - Provider discovery.
//! - Provider lifecycle.
//! - Manifest parsing.
//! - Backend compatibility.
//! - Hardware capability definitions.
//! - Target discovery.
//! - Resource allocation.
//! - Scheduling.
//! - Runtime execution.
//! - Security admission.
//! - Executable extension loading.
//!
//! ## Canonical authorities
//!
//! Provider contracts:
//! `crate::compiler::extensions::provider`
//!
//! Provider descriptors:
//! `crate::compiler::extensions::provider::ProviderDescriptor`
//!
//! Manifest declarations:
//! `crate::compiler::extensions::manifest`
//!
//! Extension discovery:
//! `crate::compiler::extensions::discovery`
//!
//! Extension registration:
//! `crate::compiler::extensions::registry`
//!
//! Extension validation:
//! `crate::compiler::extensions::validation`
//!
//! Backend compatibility:
//! `crate::backend::compatibility`
//!
//! Backend integration:
//! `crate::compiler::backends::compatibility`
//!
//! Security admission:
//! `crate::compiler::security`
//!
//! ## Compatibility semantics
//!
//! Compatibility has three possible outcomes:
//!
//! - Compatible: all requested compatibility conditions are met.
//! - Incompatible: an explicit compatibility condition is violated.
//! - Indeterminate: available evidence cannot establish compatibility.
//!
//! Indeterminate is never equivalent to Compatible.
//!
//! ## Scalability
//!
//! No arbitrary universal limits are imposed on:
//!
//! - Providers.
//! - Dependencies.
//! - Contributions.
//! - Extension points.
//! - Computing domains.
//! - Computing technologies.
//! - Provider versions.
//!
//! Operational budgets belong to the host configuration and
//! extension-admission infrastructure.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## Security
//!
//! Compatibility is not authentication.
//!
//! Compatibility is not authorization.
//!
//! Compatibility is not sandboxing.
//!
//! Compatibility is not proof that provider code implements its
//! declared contract.
//!
//! Security admission must independently succeed before an
//! executable provider is activated.
//!
//! ## Determinism
//!
//! Findings are stored in deterministic order.
//!
//! Provider registration order does not determine compatibility.
//!
//! ## Safety
//!
//! This module uses safe Rust exclusively.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use super::provider::{
    api_major,
    api_minor,
    validate_provider_compatibility,
    ContributionKind,
    ExtensionPointId,
    ProviderDescriptor,
    ProviderError,
    ProviderId,
    ProviderVersion,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable identity of the extension compatibility subsystem.
pub const EXTENSION_COMPATIBILITY_MODULE_NAME: &str =
    "zamani.compiler.extensions.compatibility";

/// Public API version of extension compatibility.
pub const EXTENSION_COMPATIBILITY_API_VERSION: u16 = 1;

/// Returns the module identity.
#[must_use]
pub const fn module_name() -> &'static str {
    EXTENSION_COMPATIBILITY_MODULE_NAME
}

/// Returns the compatibility API version.
#[must_use]
pub const fn api_version() -> u16 {
    EXTENSION_COMPATIBILITY_API_VERSION
}

// ============================================================================
// HOST API VERSION
// ============================================================================

/// Identifies the provider API version exposed by the host compiler.
///
/// This version is independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Provider implementation version.
/// - ZUIR schema version.
/// - Backend API version.
/// - Runtime version.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct HostProviderApiVersion {
    major: u64,
    minor: u64,
}

impl HostProviderApiVersion {
    /// Creates a host provider API version.
    #[must_use]
    pub const fn new(major: u64, minor: u64) -> Self {
        Self { major, minor }
    }

    /// Returns the major version.
    #[must_use]
    pub const fn major(self) -> u64 {
        self.major
    }

    /// Returns the minor version.
    #[must_use]
    pub const fn minor(self) -> u64 {
        self.minor
    }

    /// Returns the current canonical provider API version.
    #[must_use]
    pub const fn current() -> Self {
        Self {
            major: api_major(),
            minor: api_minor(),
        }
    }
}

// ============================================================================
// PROVIDER INVENTORY COMPLETENESS
// ============================================================================

/// Describes the completeness of the provider inventory supplied
/// to a compatibility evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderInventoryCompleteness {
    /// The supplied inventory may omit installed providers.
    ///
    /// An unresolved dependency therefore remains indeterminate.
    Partial,

    /// The supplied inventory is authoritative and exhaustive
    /// for the evaluation environment.
    ///
    /// A missing dependency is explicitly unavailable.
    Complete,
}

// ============================================================================
// PROVIDER INVENTORY
// ============================================================================

/// Snapshot of provider versions available to the host.
///
/// This is an immutable evaluation input.
///
/// It does not implement provider registration or discovery.
///
/// The authoritative registry supplies the snapshot.
///
/// Provider identity and version remain owned by `provider.rs`.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProviderInventory {
    providers: BTreeMap<ProviderId, ProviderVersion>,
    completeness: ProviderInventoryCompleteness,
}

impl Default for ProviderInventoryCompleteness {
    fn default() -> Self {
        Self::Partial
    }
}

impl ProviderInventory {
    /// Creates an empty, partial inventory.
    ///
    /// An empty partial inventory does not establish that
    /// dependencies are unavailable.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an inventory from an authoritative collection.
    ///
    /// Duplicate provider identities are rejected.
    pub fn from_versions(
        versions: impl IntoIterator<Item = (ProviderId, ProviderVersion)>,
        completeness: ProviderInventoryCompleteness,
    ) -> Result<Self, CompatibilityError> {
        let mut inventory = Self {
            providers: BTreeMap::new(),
            completeness,
        };

        for (provider_id, version) in versions {
            if inventory
                .providers
                .insert(provider_id.clone(), version)
                .is_some()
            {
                return Err(
                    CompatibilityError::DuplicateInventoryProvider {
                        provider_id,
                    },
                );
            }
        }

        Ok(inventory)
    }

    /// Adds a provider to this inventory.
    ///
    /// Existing identities cannot be silently overwritten.
    pub fn insert(
        &mut self,
        provider_id: ProviderId,
        version: ProviderVersion,
    ) -> Result<(), CompatibilityError> {
        if self.providers.contains_key(&provider_id) {
            return Err(
                CompatibilityError::DuplicateInventoryProvider {
                    provider_id,
                },
            );
        }

        self.providers.insert(provider_id, version);

        Ok(())
    }

    /// Sets the inventory completeness declaration.
    pub fn set_completeness(
        &mut self,
        completeness: ProviderInventoryCompleteness,
    ) {
        self.completeness = completeness;
    }

    /// Returns the inventory completeness.
    #[must_use]
    pub const fn completeness(
        &self,
    ) -> ProviderInventoryCompleteness {
        self.completeness
    }

    /// Returns the version of an available provider.
    #[must_use]
    pub fn version(
        &self,
        provider_id: &ProviderId,
    ) -> Option<ProviderVersion> {
        self.providers.get(provider_id).copied()
    }

    /// Returns the number of inventory entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    /// Returns whether the inventory is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    /// Iterates over providers in deterministic identity order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&ProviderId, &ProviderVersion)> {
        self.providers.iter()
    }
}

// ============================================================================
// COMPATIBILITY REQUEST
// ============================================================================

/// Requirements supplied to an extension compatibility evaluation.
///
/// The request describes host expectations.
///
/// It does not redefine provider capabilities or source semantics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtensionCompatibilityRequest {
    host_api: HostProviderApiVersion,
    inventory: ProviderInventory,
    required_contributions: BTreeSet<ContributionKind>,
    required_extension_points: BTreeSet<ExtensionPointId>,
}

impl ExtensionCompatibilityRequest {
    /// Creates a request using an explicit host API version.
    #[must_use]
    pub fn new(host_api: HostProviderApiVersion) -> Self {
        Self {
            host_api,
            inventory: ProviderInventory::new(),
            required_contributions: BTreeSet::new(),
            required_extension_points: BTreeSet::new(),
        }
    }

    /// Creates a request using the current host provider API.
    #[must_use]
    pub fn current() -> Self {
        Self::new(HostProviderApiVersion::current())
    }

    /// Associates a provider inventory with the request.
    #[must_use]
    pub fn with_inventory(
        mut self,
        inventory: ProviderInventory,
    ) -> Self {
        self.inventory = inventory;
        self
    }

    /// Requires a contribution category.
    #[must_use]
    pub fn requiring_contribution(
        mut self,
        contribution: ContributionKind,
    ) -> Self {
        self.required_contributions.insert(contribution);
        self
    }

    /// Requires an extension point.
    #[must_use]
    pub fn requiring_extension_point(
        mut self,
        extension_point: ExtensionPointId,
    ) -> Self {
        self.required_extension_points
            .insert(extension_point);

        self
    }

    /// Returns the host API version.
    #[must_use]
    pub const fn host_api(&self) -> HostProviderApiVersion {
        self.host_api
    }

    /// Returns the provider inventory.
    #[must_use]
    pub fn inventory(&self) -> &ProviderInventory {
        &self.inventory
    }

    /// Returns required contribution categories.
    #[must_use]
    pub fn required_contributions(
        &self,
    ) -> &BTreeSet<ContributionKind> {
        &self.required_contributions
    }

    /// Returns required extension points.
    #[must_use]
    pub fn required_extension_points(
        &self,
    ) -> &BTreeSet<ExtensionPointId> {
        &self.required_extension_points
    }
}

// ============================================================================
// COMPATIBILITY STATUS
// ============================================================================

/// Overall result of an extension compatibility evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ExtensionCompatibilityStatus {
    /// All requested compatibility conditions are satisfied.
    Compatible,

    /// Compatibility cannot be established from available evidence.
    Indeterminate,

    /// At least one explicit compatibility condition is violated.
    Incompatible,
}

impl ExtensionCompatibilityStatus {
    /// Whether compatibility has been established.
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        matches!(self, Self::Compatible)
    }

    /// Whether compatibility is indeterminate.
    #[must_use]
    pub const fn is_indeterminate(self) -> bool {
        matches!(self, Self::Indeterminate)
    }

    /// Whether incompatibility has been established.
    #[must_use]
    pub const fn is_incompatible(self) -> bool {
        matches!(self, Self::Incompatible)
    }
}

// ============================================================================
// FINDING CODES
// ============================================================================

/// Machine-readable compatibility finding categories.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[non_exhaustive]
pub enum CompatibilityFindingCode {
    /// Provider API major or minor version is unsupported.
    IncompatibleHostApi,

    /// A required provider dependency is unavailable.
    MissingDependency,

    /// Dependency inventory is incomplete.
    DependencyEvidenceUnavailable,

    /// A dependency version is outside its declared range.
    DependencyVersionMismatch,

    /// A required contribution category is not declared.
    MissingContribution,

    /// A required extension point is not declared.
    MissingExtensionPoint,
}

// ============================================================================
// FINDINGS
// ============================================================================

/// One compatibility finding.
///
/// Findings preserve the identity of the affected provider
/// or extension contract.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CompatibilityFinding {
    code: CompatibilityFindingCode,
    subject: String,
    message: String,
    blocking: bool,
    indeterminate: bool,
}

impl CompatibilityFinding {
    /// Constructs a compatibility finding.
    fn new(
        code: CompatibilityFindingCode,
        subject: impl Into<String>,
        message: impl Into<String>,
        blocking: bool,
        indeterminate: bool,
    ) -> Self {
        Self {
            code,
            subject: subject.into(),
            message: message.into(),
            blocking,
            indeterminate,
        }
    }

    /// Returns the finding code.
    #[must_use]
    pub const fn code(&self) -> CompatibilityFindingCode {
        self.code
    }

    /// Returns the affected subject.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Returns the diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Whether the finding establishes incompatibility.
    #[must_use]
    pub const fn is_blocking(&self) -> bool {
        self.blocking
    }

    /// Whether the finding represents insufficient evidence.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        self.indeterminate
    }
}

// ============================================================================
// COMPATIBILITY REPORT
// ============================================================================

/// Complete, deterministic extension compatibility report.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtensionCompatibilityReport {
    provider_id: ProviderId,
    provider_version: ProviderVersion,
    host_api: HostProviderApiVersion,
    status: ExtensionCompatibilityStatus,
    findings: Vec<CompatibilityFinding>,
}

impl ExtensionCompatibilityReport {
    /// Returns the evaluated provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the provider implementation version.
    #[must_use]
    pub const fn provider_version(&self) -> ProviderVersion {
        self.provider_version
    }

    /// Returns the host API version.
    #[must_use]
    pub const fn host_api(&self) -> HostProviderApiVersion {
        self.host_api
    }

    /// Returns the overall compatibility status.
    #[must_use]
    pub const fn status(&self) -> ExtensionCompatibilityStatus {
        self.status
    }

    /// Returns all findings in deterministic order.
    #[must_use]
    pub fn findings(&self) -> &[CompatibilityFinding] {
        &self.findings
    }

    /// Returns whether compatibility has been established.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        self.status.is_compatible()
    }

    /// Returns whether the result is indeterminate.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        self.status.is_indeterminate()
    }

    /// Returns whether the provider is incompatible.
    #[must_use]
    pub const fn is_incompatible(&self) -> bool {
        self.status.is_incompatible()
    }

    /// Returns the number of findings.
    #[must_use]
    pub fn finding_count(&self) -> usize {
        self.findings.len()
    }

    /// Returns the number of blocking findings.
    #[must_use]
    pub fn blocking_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.is_blocking())
            .count()
    }

    /// Returns the number of indeterminate findings.
    #[must_use]
    pub fn indeterminate_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.is_indeterminate())
            .count()
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors caused by invalid compatibility evaluation inputs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompatibilityError {
    /// The provider inventory contains a duplicate identity.
    DuplicateInventoryProvider {
        /// Duplicate provider identity.
        provider_id: ProviderId,
    },
}

impl fmt::Display for CompatibilityError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::DuplicateInventoryProvider { provider_id } => {
                write!(
                    formatter,
                    "provider `{provider_id}` occurs more than once \
                     in the compatibility inventory"
                )
            }
        }
    }
}

impl Error for CompatibilityError {}

// ============================================================================
// COMPATIBILITY EVALUATOR
// ============================================================================

/// Evaluates provider descriptors against host compatibility contracts.
///
/// This evaluator is stateless and can be shared across compilation
/// sessions.
///
/// It does not own a registry or activate providers.
#[derive(Clone, Copy, Debug, Default)]
pub struct ExtensionCompatibilityEvaluator;

impl ExtensionCompatibilityEvaluator {
    /// Creates the compatibility evaluator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Evaluates one provider descriptor.
    ///
    /// The provider descriptor must already have passed structural
    /// validation through its canonical provider API.
    ///
    /// The resulting report never treats unknown dependency evidence
    /// as successful compatibility.
    #[must_use]
    pub fn evaluate(
        &self,
        descriptor: &ProviderDescriptor,
        request: &ExtensionCompatibilityRequest,
    ) -> ExtensionCompatibilityReport {
        let mut findings = Vec::new();

        self.evaluate_host_api(
            descriptor,
            request,
            &mut findings,
        );

        self.evaluate_dependencies(
            descriptor,
            request,
            &mut findings,
        );

        self.evaluate_contributions(
            descriptor,
            request,
            &mut findings,
        );

        self.evaluate_extension_points(
            descriptor,
            request,
            &mut findings,
        );

        findings.sort();

        let status = determine_status(&findings);

        ExtensionCompatibilityReport {
            provider_id: descriptor.id().clone(),
            provider_version: descriptor.version(),
            host_api: request.host_api(),
            status,
            findings,
        }
    }

    fn evaluate_host_api(
        &self,
        descriptor: &ProviderDescriptor,
        request: &ExtensionCompatibilityRequest,
        findings: &mut Vec<CompatibilityFinding>,
    ) {
        let host = request.host_api();

        if let Err(error) = validate_provider_compatibility(
            descriptor,
            host.major(),
            host.minor(),
        ) {
            findings.push(
                CompatibilityFinding::new(
                    CompatibilityFindingCode::IncompatibleHostApi,
                    descriptor.id().to_string(),
                    error.to_string(),
                    true,
                    false,
                ),
            );
        }
    }

    fn evaluate_dependencies(
        &self,
        descriptor: &ProviderDescriptor,
        request: &ExtensionCompatibilityRequest,
        findings: &mut Vec<CompatibilityFinding>,
    ) {
        let inventory = request.inventory();

        for (dependency_id, requirement) in descriptor.dependencies() {
            match inventory.version(dependency_id) {
                Some(available_version) => {
                    if !requirement.accepts(available_version) {
                        findings.push(
                            CompatibilityFinding::new(
                                CompatibilityFindingCode::DependencyVersionMismatch,
                                dependency_id.to_string(),
                                format!(
                                    "provider `{}` requires dependency \
                                     `{}` in its declared version range, \
                                     but version `{}` is available",
                                    descriptor.id(),
                                    dependency_id,
                                    available_version,
                                ),
                                true,
                                false,
                            ),
                        );
                    }
                }

                None => {
                    match inventory.completeness() {
                        ProviderInventoryCompleteness::Complete => {
                            findings.push(
                                CompatibilityFinding::new(
                                    CompatibilityFindingCode::MissingDependency,
                                    dependency_id.to_string(),
                                    format!(
                                        "required provider dependency \
                                         `{dependency_id}` is absent \
                                         from the authoritative inventory"
                                    ),
                                    true,
                                    false,
                                ),
                            );
                        }

                        ProviderInventoryCompleteness::Partial => {
                            findings.push(
                                CompatibilityFinding::new(
                                    CompatibilityFindingCode::DependencyEvidenceUnavailable,
                                    dependency_id.to_string(),
                                    format!(
                                        "availability of required provider \
                                         dependency `{dependency_id}` \
                                         cannot be established from the \
                                         partial inventory"
                                    ),
                                    false,
                                    true,
                                ),
                            );
                        }
                    }
                }
            }
        }
    }

    fn evaluate_contributions(
        &self,
        descriptor: &ProviderDescriptor,
        request: &ExtensionCompatibilityRequest,
        findings: &mut Vec<CompatibilityFinding>,
    ) {
        for contribution in request.required_contributions() {
            if !descriptor.supports_contribution(contribution) {
                findings.push(
                    CompatibilityFinding::new(
                        CompatibilityFindingCode::MissingContribution,
                        contribution.to_string(),
                        format!(
                            "provider `{}` does not declare required \
                             contribution `{contribution}`",
                            descriptor.id()
                        ),
                        true,
                        false,
                    ),
                );
            }
        }
    }

    fn evaluate_extension_points(
        &self,
        descriptor: &ProviderDescriptor,
        request: &ExtensionCompatibilityRequest,
        findings: &mut Vec<CompatibilityFinding>,
    ) {
        for extension_point in request.required_extension_points() {
            if !descriptor.supports_extension_point(extension_point) {
                findings.push(
                    CompatibilityFinding::new(
                        CompatibilityFindingCode::MissingExtensionPoint,
                        extension_point.to_string(),
                        format!(
                            "provider `{}` does not declare required \
                             extension point `{extension_point}`",
                            descriptor.id()
                        ),
                        true,
                        false,
                    ),
                );
            }
        }
    }
}

// ============================================================================
// STATUS RESOLUTION
// ============================================================================

/// Determines the overall result from the complete finding set.
///
/// Explicit incompatibility takes precedence over indeterminate
/// evidence because a known violated requirement cannot be repaired
/// merely by discovering additional providers.
fn determine_status(
    findings: &[CompatibilityFinding],
) -> ExtensionCompatibilityStatus {
    if findings.iter().any(CompatibilityFinding::is_blocking) {
        return ExtensionCompatibilityStatus::Incompatible;
    }

    if findings.iter().any(CompatibilityFinding::is_indeterminate) {
        return ExtensionCompatibilityStatus::Indeterminate;
    }

    ExtensionCompatibilityStatus::Compatible
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::extensions::provider::{
        ContributionKind,
        ExtensionPointId,
        ProviderApiCompatibility,
        ProviderDependency,
        ProviderId,
        ProviderVersion,
        ProviderVersionRequirement,
    };

    fn provider_id(value: &str) -> ProviderId {
        ProviderId::new(value).expect("valid provider identity")
    }

    fn contribution(value: &str) -> ContributionKind {
        ContributionKind::new(value)
            .expect("valid contribution")
    }

    fn extension_point(value: &str) -> ExtensionPointId {
        ExtensionPointId::new(value)
            .expect("valid extension point")
    }

    fn descriptor() -> ProviderDescriptor {
        ProviderDescriptor::new(
            provider_id("example.compiler"),
            ProviderVersion::new(1, 0, 0),
            ProviderApiCompatibility::new(
                api_major(),
                0,
                None,
            )
            .expect("valid API range"),
            "Example provider",
            "Example extension provider",
            [contribution("compiler.pass")],
        )
        .expect("valid descriptor")
        .with_extension_point(extension_point("compiler.optimization"))
    }

    fn evaluator() -> ExtensionCompatibilityEvaluator {
        ExtensionCompatibilityEvaluator::new()
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            module_name(),
            "zamani.compiler.extensions.compatibility"
        );
    }

    #[test]
    fn api_version_is_nonzero() {
        assert_ne!(api_version(), 0);
    }

    #[test]
    fn current_host_api_uses_provider_authority() {
        let host = HostProviderApiVersion::current();

        assert_eq!(host.major(), api_major());
        assert_eq!(host.minor(), api_minor());
    }

    #[test]
    fn compatible_provider_is_accepted() {
        let report = evaluator().evaluate(
            &descriptor(),
            &ExtensionCompatibilityRequest::current(),
        );

        assert!(report.is_compatible());
        assert_eq!(report.finding_count(), 0);
    }

    #[test]
    fn incompatible_host_major_is_rejected() {
        let request = ExtensionCompatibilityRequest::new(
            HostProviderApiVersion::new(
                api_major() + 1,
                0,
            ),
        );

        let report = evaluator().evaluate(
            &descriptor(),
            &request,
        );

        assert!(report.is_incompatible());
        assert_eq!(report.blocking_count(), 1);
    }

    #[test]
    fn missing_dependency_in_complete_inventory_is_incompatible() {
        let dependency_id = provider_id("example.dependency");

        let provider = descriptor()
            .requiring(
                ProviderDependency::new(
                    dependency_id,
                    ProviderVersionRequirement::exact(
                        ProviderVersion::new(1, 0, 0),
                    ),
                ),
            )
            .expect("valid dependency");

        let inventory = ProviderInventory::new();

        let request = ExtensionCompatibilityRequest::current()
            .with_inventory(
                ProviderInventory {
                    providers: inventory.providers,
                    completeness:
                        ProviderInventoryCompleteness::Complete,
                },
            );

        let report = evaluator().evaluate(&provider, &request);

        assert!(report.is_incompatible());
        assert!(report.findings().iter().any(|finding| {
            finding.code()
                == CompatibilityFindingCode::MissingDependency
        }));
    }

    #[test]
    fn missing_dependency_in_partial_inventory_is_indeterminate() {
        let dependency_id = provider_id("example.dependency");

        let provider = descriptor()
            .requiring(
                ProviderDependency::new(
                    dependency_id,
                    ProviderVersionRequirement::exact(
                        ProviderVersion::new(1, 0, 0),
                    ),
                ),
            )
            .expect("valid dependency");

        let report = evaluator().evaluate(
            &provider,
            &ExtensionCompatibilityRequest::current(),
        );

        assert!(report.is_indeterminate());
        assert!(!report.is_compatible());
    }

    #[test]
    fn duplicate_inventory_entries_are_rejected() {
        let id = provider_id("example.provider");
        let version = ProviderVersion::new(1, 0, 0);

        let result = ProviderInventory::from_versions(
            [
                (id.clone(), version),
                (id, version),
            ],
            ProviderInventoryCompleteness::Complete,
        );

        assert!(result.is_err());
    }

    #[test]
    fn required_contribution_is_checked() {
        let request = ExtensionCompatibilityRequest::current()
            .requiring_contribution(
                contribution("future.custom.contribution"),
            );

        let report = evaluator().evaluate(
            &descriptor(),
            &request,
        );

        assert!(report.is_incompatible());
    }

    #[test]
    fn required_extension_point_is_checked() {
        let request = ExtensionCompatibilityRequest::current()
            .requiring_extension_point(
                extension_point("future.custom.extension"),
            );

        let report = evaluator().evaluate(
            &descriptor(),
            &request,
        );

        assert!(report.is_incompatible());
    }

    #[test]
    fn existing_extension_point_is_accepted() {
        let request = ExtensionCompatibilityRequest::current()
            .requiring_extension_point(
                extension_point("compiler.optimization"),
            );

        let report = evaluator().evaluate(
            &descriptor(),
            &request,
        );

        assert!(report.is_compatible());
    }

    #[test]
    fn unknown_dependency_evidence_never_becomes_compatible() {
        let provider = descriptor()
            .requiring(
                ProviderDependency::new(
                    provider_id("future.provider"),
                    ProviderVersionRequirement::exact(
                        ProviderVersion::new(1, 0, 0),
                    ),
                ),
            )
            .expect("valid dependency");

        let report = evaluator().evaluate(
            &provider,
            &ExtensionCompatibilityRequest::current(),
        );

        assert_eq!(
            report.status(),
            ExtensionCompatibilityStatus::Indeterminate
        );

        assert!(!report.is_compatible());
    }

    #[test]
    fn findings_are_deterministically_ordered() {
        let request = ExtensionCompatibilityRequest::current()
            .requiring_contribution(
                contribution("zeta.contribution"),
            )
            .requiring_contribution(
                contribution("alpha.contribution"),
            );

        let first = evaluator().evaluate(
            &descriptor(),
            &request,
        );

        let second = evaluator().evaluate(
            &descriptor(),
            &request,
        );

        assert_eq!(first.findings(), second.findings());
    }

    #[test]
    fn incompatible_result_takes_precedence_over_unknown_evidence() {
        let provider = descriptor()
            .requiring(
                ProviderDependency::new(
                    provider_id("future.provider"),
                    ProviderVersionRequirement::exact(
                        ProviderVersion::new(1, 0, 0),
                    ),
                ),
            )
            .expect("valid dependency");

        let request = ExtensionCompatibilityRequest::new(
            HostProviderApiVersion::new(
                api_major() + 1,
                0,
            ),
        );

        let report = evaluator().evaluate(
            &provider,
            &request,
        );

        assert!(report.is_incompatible());
        assert!(report.indeterminate_count() > 0);
    }

    #[test]
    fn evaluator_is_stateless_and_shareable() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<ExtensionCompatibilityEvaluator>();
    }

    #[test]
    fn provider_identity_is_preserved() {
        let report = evaluator().evaluate(
            &descriptor(),
            &ExtensionCompatibilityRequest::current(),
        );

        assert_eq!(
            report.provider_id().as_str(),
            "example.compiler"
        );
    }

    #[test]
    fn report_preserves_provider_version() {
        let report = evaluator().evaluate(
            &descriptor(),
            &ExtensionCompatibilityRequest::current(),
        );

        assert_eq!(
            report.provider_version(),
            ProviderVersion::new(1, 0, 0)
        );
    }
}
