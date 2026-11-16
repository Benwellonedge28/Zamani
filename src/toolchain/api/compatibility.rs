
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Cross-Contract Compatibility
//!
//! Path: `src/toolchain/api/compatibility.rs`
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Evaluates and aggregates compatibility decisions at the public toolchain
//! API boundary.
//!
//! This module coordinates the independently versioned contracts already
//! defined by `crate::toolchain::version`:
//!
//! - Toolchain API.
//! - Project manifest schema.
//! - Package archive format.
//! - Registry protocol.
//! - Third-party provider API.
//! - Task execution protocol.
//!
//! It can also aggregate decisions made by other authoritative repository
//! subsystems without taking ownership of their version types or policies.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Toolchain-level compatibility check results.
//! - Aggregation of compatibility decisions.
//! - Evaluation of configured toolchain version policies.
//! - Explicit representation of unconfigured compatibility policies.
//! - Conversion of canonical compatibility decisions into API results.
//! - Fail-closed compatibility summaries.
//!
//! It does not own:
//!
//! - Language or grammar version policy.
//! - Frontend, AST, semantic-model, or dialect compatibility.
//! - Compiler, IR, ABI, or artifact compatibility policy.
//! - Backend or external executable version validation.
//! - Hardware, device, topology, or resource compatibility.
//! - Runtime or distributed execution compatibility.
//! - Manifest or package migration implementation.
//! - Provider discovery, authentication, or authorization.
//!
//! Those decisions remain with their authoritative subsystems.
//!
//! ## Critical compatibility rule
//!
//! A version number alone does not prove semantic compatibility.
//!
//! A compatibility result is valid only for the named contract and the
//! policy that evaluates that contract.
//!
//! Unknown, unconfigured, future, incompatible, and migration-required
//! contracts must never be silently treated as compatible.
//!
//! This module never performs a migration.
//!
//! ## POCO-REAF and scalability
//!
//! This module introduces no fixed limits on:
//!
//! - Projects, packages, dependencies, registries, or providers.
//! - Targets, devices, processors, accelerators, or quantum resources.
//! - Distributed participants, tasks, or workers.
//! - Computational domains or future provider types.
//!
//! The number of compatibility checks is determined by the supplied input
//! and available memory. No universal count ceiling is imposed.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## Integration
//!
//! - `version.rs` owns the toolchain version types and policies.
//! - `errors.rs` owns toolchain error classification.
//! - `result.rs` owns the canonical toolchain result alias.
//! - `api/request.rs` supplies versioned request contracts.
//! - `api/capabilities.rs` describes advertised API capabilities.
//! - `zamani_project_config` owns manifest parsing and schema meaning.
//! - `compiler`, `backend`, `zhal`, and `zur` retain their own compatibility
//!   authorities.
//! - `crate::zuir::version` supplies the canonical compatibility decision.
//!
//! ## Security
//!
//! Compatibility metadata is untrusted input when deserialized or received
//! from another process. Callers must validate it before relying on it.
//!
//! This module does not accept credentials, secrets, executable paths, or
//! arbitrary migration instructions.
//!
//! ## Freeze criteria
//!
//! - Every toolchain version category remains strongly typed.
//! - Each version domain is evaluated independently.
//! - Missing policies fail closed.
//! - Future versions fail closed through the canonical policy.
//! - Migration-required results are not treated as compatible.
//! - External authorities retain ownership of their own version contracts.
//! - Reports can be validated after deserialization.
//! - No arbitrary capacity ceilings are introduced.
//! - All tests pass.

use core::fmt;

use serde::{Deserialize, Serialize};

use crate::toolchain::errors::{ToolchainError, ToolchainErrorKind};
use crate::toolchain::result::ToolchainResult;
use crate::toolchain::version::{
    initial_manifest_schema_policy,
    initial_package_format_policy,
    initial_provider_api_policy,
    initial_registry_protocol_policy,
    initial_task_protocol_policy,
    initial_toolchain_api_policy,
    ManifestSchemaVersion,
    PackageFormatVersion,
    ProviderApiVersion,
    RegistryProtocolVersion,
    TaskProtocolVersion,
    ToolchainApiVersion,
    VersionPolicy,
};
use crate::zuir::version::{
    Compatibility as CanonicalCompatibility,
    CompatibilityDecision,
    CompatibilityReason as CanonicalCompatibilityReason,
    VersionIdentity,
};

// ============================================================================
// CONTRACT IDENTITIES
// ============================================================================

/// One independently versioned contract owned by the toolchain.
///
/// These identifiers describe contract categories, not implementation
/// vendors, hardware types, programming-language domains, or target devices.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ContractKind {
    /// Public toolchain API request and response contract.
    ToolchainApi,

    /// Project manifest schema consumed by the toolchain.
    ManifestSchema,

    /// Zamani package archive format.
    PackageFormat,

    /// Registry communication protocol.
    RegistryProtocol,

    /// Third-party toolchain provider API.
    ProviderApi,

    /// Toolchain task request and result protocol.
    TaskProtocol,
}

impl ContractKind {
    /// Return the stable machine-readable contract identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ToolchainApi => "toolchain_api",
            Self::ManifestSchema => "manifest_schema",
            Self::PackageFormat => "package_format",
            Self::RegistryProtocol => "registry_protocol",
            Self::ProviderApi => "provider_api",
            Self::TaskProtocol => "task_protocol",
        }
    }
}

impl fmt::Display for ContractKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A version supplied for one toolchain-owned contract.
///
/// The enum preserves the distinct version types defined in `version.rs`.
/// Equal numeric versions in different categories are not interchangeable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractVersion {
    /// Toolchain API version.
    ToolchainApi(ToolchainApiVersion),

    /// Manifest schema version.
    ManifestSchema(ManifestSchemaVersion),

    /// Package archive format version.
    PackageFormat(PackageFormatVersion),

    /// Registry protocol version.
    RegistryProtocol(RegistryProtocolVersion),

    /// Provider API version.
    ProviderApi(ProviderApiVersion),

    /// Task protocol version.
    TaskProtocol(TaskProtocolVersion),
}

impl ContractVersion {
    /// Return the contract category associated with this version.
    #[must_use]
    pub const fn kind(self) -> ContractKind {
        match self {
            Self::ToolchainApi(_) => ContractKind::ToolchainApi,
            Self::ManifestSchema(_) => ContractKind::ManifestSchema,
            Self::PackageFormat(_) => ContractKind::PackageFormat,
            Self::RegistryProtocol(_) => ContractKind::RegistryProtocol,
            Self::ProviderApi(_) => ContractKind::ProviderApi,
            Self::TaskProtocol(_) => ContractKind::TaskProtocol,
        }
    }

    /// Return the canonical version string.
    #[must_use]
    pub fn version_string(self) -> String {
        match self {
            Self::ToolchainApi(version) => version.to_string(),
            Self::ManifestSchema(version) => version.to_string(),
            Self::PackageFormat(version) => version.to_string(),
            Self::RegistryProtocol(version) => version.to_string(),
            Self::ProviderApi(version) => version.to_string(),
            Self::TaskProtocol(version) => version.to_string(),
        }
    }
}

// ============================================================================
// COMPATIBILITY STATUS
// ============================================================================

/// The outcome of evaluating one compatibility contract.
///
/// Only `Compatible` permits interpretation without migration under the
/// policy that produced the result.
///
/// `PolicyUnavailable` is deliberately not equivalent to compatibility.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CompatibilityStatus {
    /// The supplied contract is accepted without migration.
    Compatible,

    /// An explicit migration must complete before normal interpretation.
    RequiresMigration,

    /// The supplied contract cannot safely be interpreted under the policy.
    Incompatible,

    /// No policy is configured for this contract.
    ///
    /// Consumers must fail closed and must not infer compatibility.
    PolicyUnavailable,
}

impl CompatibilityStatus {
    /// Return whether the contract may be interpreted without migration.
    #[must_use]
    pub const fn permits_interpretation(self) -> bool {
        matches!(self, Self::Compatible)
    }

    /// Return whether the contract requires migration.
    #[must_use]
    pub const fn requires_migration(self) -> bool {
        matches!(self, Self::RequiresMigration)
    }

    /// Return whether this result prevents normal interpretation.
    #[must_use]
    pub const fn blocks_interpretation(self) -> bool {
        !matches!(self, Self::Compatible)
    }
}

/// The reason for a compatibility decision.
///
/// The variants correspond to the canonical version-policy reasons, with
/// an additional reason for a toolchain contract that has no configured
/// policy.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CompatibilityReason {
    /// The supplied version exactly matches the current version.
    ExactMatch,

    /// The supplied older version is explicitly supported without migration.
    SupportedOlderVersion,

    /// The supplied older version requires migration.
    MigrationRequired,

    /// The supplied version is newer than the current implementation.
    FutureVersion,

    /// The supplied version has a different major version.
    MajorVersionMismatch,

    /// The supplied version is below the supported historical range.
    BelowMinimumSupported,

    /// The supplied version is outside the allowed version range.
    OutsideAllowedRange,

    /// No policy is configured for the requested contract.
    PolicyNotConfigured,
}

impl CompatibilityReason {
    fn from_canonical(reason: CanonicalCompatibilityReason) -> Self {
        match reason {
            CanonicalCompatibilityReason::ExactMatch => Self::ExactMatch,
            CanonicalCompatibilityReason::SupportedOlderVersion => {
                Self::SupportedOlderVersion
            }
            CanonicalCompatibilityReason::MigrationRequired => {
                Self::MigrationRequired
            }
            CanonicalCompatibilityReason::FutureVersion => {
                Self::FutureVersion
            }
            CanonicalCompatibilityReason::MajorVersionMismatch => {
                Self::MajorVersionMismatch
            }
            CanonicalCompatibilityReason::BelowMinimumSupported => {
                Self::BelowMinimumSupported
            }
            CanonicalCompatibilityReason::OutsideAllowedRange => {
                Self::OutsideAllowedRange
            }
        }
    }
}

impl fmt::Display for CompatibilityReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::ExactMatch => "exact_match",
            Self::SupportedOlderVersion => "supported_older_version",
            Self::MigrationRequired => "migration_required",
            Self::FutureVersion => "future_version",
            Self::MajorVersionMismatch => "major_version_mismatch",
            Self::BelowMinimumSupported => "below_minimum_supported",
            Self::OutsideAllowedRange => "outside_allowed_range",
            Self::PolicyNotConfigured => "policy_not_configured",
        };

        formatter.write_str(value)
    }
}

// ============================================================================
// COMPATIBILITY CONTRACT REFERENCE
// ============================================================================

/// Identifies the authority responsible for a compatibility contract.
///
/// Toolchain-owned contracts use `Toolchain`.
///
/// Contracts owned by other repository subsystems use `External`. The
/// authority and contract names are descriptive identifiers; they do not
/// transfer ownership of the corresponding version or compatibility policy.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum CompatibilityContract {
    /// A contract whose version policy is owned by the toolchain.
    Toolchain(ContractKind),

    /// A contract evaluated by another authoritative subsystem.
    External {
        /// Stable identifier of the owning authority.
        authority: String,

        /// Stable identifier of the contract within that authority.
        contract: String,
    },
}

// ============================================================================
// SUPPORTED VERSION RANGE SNAPSHOT
// ============================================================================

/// A display-only snapshot of a policy's inclusive supported version range.
///
/// This snapshot is for reporting. The authoritative range remains the
/// `VersionRange<V>` stored by `version.rs`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportedVersionRange {
    /// Inclusive minimum supported version.
    pub minimum: String,

    /// Inclusive maximum supported version.
    pub maximum: String,
}

impl SupportedVersionRange {
    fn new<V: fmt::Display>(
        minimum: V,
        maximum: V,
    ) -> Self {
        Self {
            minimum: minimum.to_string(),
            maximum: maximum.to_string(),
        }
    }

    /// Validate the range snapshot.
    ///
    /// The strings must be non-empty. Ordering is not re-evaluated here:
    /// the authoritative typed policy performs that validation.
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_metadata("supported.minimum", &self.minimum)?;
        validate_metadata("supported.maximum", &self.maximum)?;
        Ok(())
    }
}

// ============================================================================
// INDIVIDUAL COMPATIBILITY RESULT
// ============================================================================

/// The result of evaluating one versioned contract.
///
/// Version strings are snapshots for reporting. They do not replace the
/// canonical strongly typed version values or policy that produced the
/// decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompatibilityCheck {
    contract: CompatibilityContract,
    supplied_version: String,
    current_version: Option<String>,
    supported_versions: Option<SupportedVersionRange>,
    status: CompatibilityStatus,
    reason: CompatibilityReason,
}

impl CompatibilityCheck {
    fn from_policy<V>(
        kind: ContractKind,
        supplied: V,
        policy: Option<&VersionPolicy<V>>,
    ) -> Self
    where
        V: VersionIdentity + fmt::Display,
    {
        let contract = CompatibilityContract::Toolchain(kind);
        let supplied_version = supplied.to_string();

        let Some(policy) = policy else {
            return Self {
                contract,
                supplied_version,
                current_version: None,
                supported_versions: None,
                status: CompatibilityStatus::PolicyUnavailable,
                reason: CompatibilityReason::PolicyNotConfigured,
            };
        };

        let decision = policy.check(supplied);

        let status = match decision.compatibility() {
            CanonicalCompatibility::Compatible => {
                CompatibilityStatus::Compatible
            }
            CanonicalCompatibility::RequiresMigration => {
                CompatibilityStatus::RequiresMigration
            }
            CanonicalCompatibility::Incompatible => {
                CompatibilityStatus::Incompatible
            }
        };

        Self {
            contract,
            supplied_version,
            current_version: Some(policy.current().to_string()),
            supported_versions: Some(SupportedVersionRange::new(
                policy.supported().minimum(),
                policy.supported().maximum(),
            )),
            status,
            reason: CompatibilityReason::from_canonical(
                decision.reason(),
            ),
        }
    }

    /// Create a result from a decision made by another canonical authority.
    ///
    /// This adapter does not reinterpret the decision. It records the
    /// supplied authority, contract, source version, target version, and
    /// canonical compatibility outcome.
    ///
    /// Use this for language, compiler, IR, ABI, backend, runtime, hardware,
    /// or other contracts whose policies are owned outside the toolchain.
    pub fn from_authority(
        authority: impl Into<String>,
        contract: impl Into<String>,
        source_version: impl Into<String>,
        target_version: impl Into<String>,
        decision: CompatibilityDecision,
    ) -> ToolchainResult<Self> {
        let authority = authority.into();
        let contract_name = contract.into();
        let source_version = source_version.into();
        let target_version = target_version.into();

        validate_metadata("authority", &authority)?;
        validate_metadata("contract", &contract_name)?;
        validate_metadata("source_version", &source_version)?;
        validate_metadata("target_version", &target_version)?;

        let status = match decision.compatibility() {
            CanonicalCompatibility::Compatible => {
                CompatibilityStatus::Compatible
            }
            CanonicalCompatibility::RequiresMigration => {
                CompatibilityStatus::RequiresMigration
            }
            CanonicalCompatibility::Incompatible => {
                CompatibilityStatus::Incompatible
            }
        };

        Ok(Self {
            contract: CompatibilityContract::External {
                authority,
                contract: contract_name,
            },
            supplied_version: source_version,
            current_version: Some(target_version),
            supported_versions: None,
            status,
            reason: CompatibilityReason::from_canonical(
                decision.reason(),
            ),
        })
    }

    /// Return the contract reference.
    #[must_use]
    pub fn contract(&self) -> &CompatibilityContract {
        &self.contract
    }

    /// Return the supplied version string.
    #[must_use]
    pub fn supplied_version(&self) -> &str {
        &self.supplied_version
    }

    /// Return the current or target version, if known.
    #[must_use]
    pub fn current_version(&self) -> Option<&str> {
        self.current_version.as_deref()
    }

    /// Return the reported supported range, if one is available.
    #[must_use]
    pub fn supported_versions(&self) -> Option<&SupportedVersionRange> {
        self.supported_versions.as_ref()
    }

    /// Return the compatibility status.
    #[must_use]
    pub const fn status(&self) -> CompatibilityStatus {
        self.status
    }

    /// Return the reason for the decision.
    #[must_use]
    pub const fn reason(&self) -> CompatibilityReason {
        self.reason
    }

    /// Return whether this check permits interpretation without migration.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        self.status.permits_interpretation()
    }

    /// Validate a check received from an external serialization boundary.
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_metadata("supplied_version", &self.supplied_version)?;

        if let Some(current) = &self.current_version {
            validate_metadata("current_version", current)?;
        }

        if let Some(range) = &self.supported_versions {
            range.validate()?;
        }

        match &self.contract {
            CompatibilityContract::Toolchain(_) => {
                if self.status == CompatibilityStatus::PolicyUnavailable
                    && self.reason != CompatibilityReason::PolicyNotConfigured
                {
                    return Err(invalid_compatibility_data(
                        "an unconfigured toolchain policy must use \
                         policy_not_configured as its reason",
                    ));
                }

                if self.status != CompatibilityStatus::PolicyUnavailable
                    && self.reason == CompatibilityReason::PolicyNotConfigured
                {
                    return Err(invalid_compatibility_data(
                        "policy_not_configured requires an unavailable policy",
                    ));
                }
            }
            CompatibilityContract::External {
                authority,
                contract,
            } => {
                validate_metadata("authority", authority)?;
                validate_metadata("contract", contract)?;

                if self.status == CompatibilityStatus::PolicyUnavailable {
                    return Err(invalid_compatibility_data(
                        "external authority evidence cannot use \
                         policy_unavailable; report unavailable evidence \
                         through the owning integration boundary",
                    ));
                }

                if self.reason == CompatibilityReason::PolicyNotConfigured {
                    return Err(invalid_compatibility_data(
                        "external authority evidence cannot claim that a \
                         toolchain policy is unconfigured",
                    ));
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// COMPATIBILITY REPORT
// ============================================================================

/// An aggregate of independently evaluated compatibility checks.
///
/// An empty report is not considered compatible. A consumer must have
/// evidence for the contracts required by its operation before proceeding.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompatibilityReport {
    checks: Vec<CompatibilityCheck>,
}

impl CompatibilityReport {
    /// Create an empty report.
    ///
    /// An empty report is incomplete and does not prove compatibility.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            checks: Vec::new(),
        }
    }

    /// Construct a report from supplied checks.
    ///
    /// Every check is validated before the report is returned.
    pub fn from_checks(
        checks: Vec<CompatibilityCheck>,
    ) -> ToolchainResult<Self> {
        let report = Self { checks };
        report.validate()?;
        Ok(report)
    }

    /// Append one compatibility check.
    pub fn push(
        &mut self,
        check: CompatibilityCheck,
    ) -> ToolchainResult<()> {
        check.validate()?;
        self.checks.push(check);
        Ok(())
    }

    /// Append checks from another report.
    ///
    /// The checks are validated before being appended.
    pub fn extend(
        &mut self,
        other: CompatibilityReport,
    ) -> ToolchainResult<()> {
        other.validate()?;
        self.checks.extend(other.checks);
        Ok(())
    }

    /// Return all compatibility checks in evaluation order.
    #[must_use]
    pub fn checks(&self) -> &[CompatibilityCheck] {
        &self.checks
    }

    /// Return the number of checks in this report.
    #[must_use]
    pub fn len(&self) -> usize {
        self.checks.len()
    }

    /// Return whether the report contains no checks.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.checks.is_empty()
    }

    /// Return whether every recorded check is compatible.
    ///
    /// An empty report returns `false`: absence of evidence is not proof
    /// of compatibility.
    #[must_use]
    pub fn is_compatible(&self) -> bool {
        !self.checks.is_empty()
            && self.checks.iter().all(CompatibilityCheck::is_compatible)
    }

    /// Return whether any check blocks normal interpretation.
    ///
    /// An empty report returns `true` because no compatibility evidence
    /// has been supplied.
    #[must_use]
    pub fn has_blockers(&self) -> bool {
        self.checks.is_empty()
            || self.checks.iter().any(|check| {
                check.status().blocks_interpretation()
            })
    }

    /// Return whether at least one check requires migration.
    #[must_use]
    pub fn requires_migration(&self) -> bool {
        self.checks.iter().any(|check| {
            check.status().requires_migration()
        })
    }

    /// Return whether at least one policy is unavailable.
    #[must_use]
    pub fn has_unconfigured_policies(&self) -> bool {
        self.checks.iter().any(|check| {
            check.status() == CompatibilityStatus::PolicyUnavailable
        })
    }

    /// Return whether at least one check is explicitly incompatible.
    #[must_use]
    pub fn has_incompatible_contracts(&self) -> bool {
        self.checks.iter().any(|check| {
            check.status() == CompatibilityStatus::Incompatible
        })
    }

    /// Validate every check in the report.
    ///
    /// This must be called after deserializing an untrusted report.
    pub fn validate(&self) -> ToolchainResult<()> {
        for check in &self.checks {
            check.validate()?;
        }

        Ok(())
    }
}

// ============================================================================
// POLICY SET
// ============================================================================

/// Configured policies for the six toolchain-owned version domains.
///
/// Each field is optional because a toolchain deployment may not implement
/// every service. An absent policy is reported as `PolicyUnavailable` and
/// never treated as a compatible contract.
///
/// The policies are independent: changing a registry protocol policy does
/// not implicitly change the package format, provider API, or language
/// version policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompatibilityPolicies {
    /// Toolchain API compatibility policy.
    pub toolchain_api: Option<VersionPolicy<ToolchainApiVersion>>,

    /// Manifest schema compatibility policy.
    pub manifest_schema: Option<VersionPolicy<ManifestSchemaVersion>>,

    /// Package archive format compatibility policy.
    pub package_format: Option<VersionPolicy<PackageFormatVersion>>,

    /// Registry protocol compatibility policy.
    pub registry_protocol: Option<VersionPolicy<RegistryProtocolVersion>>,

    /// Provider API compatibility policy.
    pub provider_api: Option<VersionPolicy<ProviderApiVersion>>,

    /// Task protocol compatibility policy.
    pub task_protocol: Option<VersionPolicy<TaskProtocolVersion>>,
}

impl CompatibilityPolicies {
    /// Construct conservative initial policies.
    ///
    /// The existing `version.rs` initial policies accept only their exact
    /// initial contract versions. Historical compatibility must be added
    /// only when the corresponding reader and conformance tests exist.
    #[must_use]
    pub fn conservative_initial() -> Self {
        Self {
            toolchain_api: Some(initial_toolchain_api_policy()),
            manifest_schema: Some(initial_manifest_schema_policy()),
            package_format: Some(initial_package_format_policy()),
            registry_protocol: Some(initial_registry_protocol_policy()),
            provider_api: Some(initial_provider_api_policy()),
            task_protocol: Some(initial_task_protocol_policy()),
        }
    }

    /// Validate every configured version policy.
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_policy("toolchain_api", self.toolchain_api.as_ref())?;
        validate_policy("manifest_schema", self.manifest_schema.as_ref())?;
        validate_policy("package_format", self.package_format.as_ref())?;
        validate_policy(
            "registry_protocol",
            self.registry_protocol.as_ref(),
        )?;
        validate_policy("provider_api", self.provider_api.as_ref())?;
        validate_policy("task_protocol", self.task_protocol.as_ref())?;

        Ok(())
    }
}

impl Default for CompatibilityPolicies {
    fn default() -> Self {
        Self::conservative_initial()
    }
}

fn validate_policy<V>(
    name: &str,
    policy: Option<&VersionPolicy<V>>,
) -> ToolchainResult<()>
where
    V: VersionIdentity,
{
    if let Some(policy) = policy {
        policy.validate().map_err(|error| {
            ToolchainError::new(
                ToolchainErrorKind::InvalidConfiguration,
                format!("invalid compatibility policy for {name}"),
            )
            .with_context(error.to_string())
        })?;
    }

    Ok(())
}

// ============================================================================
// COMPATIBILITY CHECKER
// ============================================================================

/// Evaluates toolchain-owned version policies and aggregates external
/// compatibility evidence.
///
/// The checker is immutable after construction. Applications can share it
/// across operations when their concurrency model permits shared references.
#[derive(Debug, Clone)]
pub struct CompatibilityChecker {
    policies: CompatibilityPolicies,
}

impl CompatibilityChecker {
    /// Construct a checker from explicit policies.
    ///
    /// Invalid policy configurations are rejected before the checker can
    /// be used.
    pub fn new(
        policies: CompatibilityPolicies,
    ) -> ToolchainResult<Self> {
        policies.validate()?;

        Ok(Self { policies })
    }

    /// Construct a checker using conservative initial policies.
    ///
    /// Every toolchain contract accepts only its current initial version.
    #[must_use]
    pub fn conservative_initial() -> Self {
        Self {
            policies: CompatibilityPolicies::conservative_initial(),
        }
    }

    /// Return the configured policy set.
    #[must_use]
    pub fn policies(&self) -> &CompatibilityPolicies {
        &self.policies
    }

    /// Evaluate one toolchain-owned version contract.
    #[must_use]
    pub fn check(
        &self,
        version: ContractVersion,
    ) -> CompatibilityCheck {
        match version {
            ContractVersion::ToolchainApi(supplied) => {
                CompatibilityCheck::from_policy(
                    ContractKind::ToolchainApi,
                    supplied,
                    self.policies.toolchain_api.as_ref(),
                )
            }
            ContractVersion::ManifestSchema(supplied) => {
                CompatibilityCheck::from_policy(
                    ContractKind::ManifestSchema,
                    supplied,
                    self.policies.manifest_schema.as_ref(),
                )
            }
            ContractVersion::PackageFormat(supplied) => {
                CompatibilityCheck::from_policy(
                    ContractKind::PackageFormat,
                    supplied,
                    self.policies.package_format.as_ref(),
                )
            }
            ContractVersion::RegistryProtocol(supplied) => {
                CompatibilityCheck::from_policy(
                    ContractKind::RegistryProtocol,
                    supplied,
                    self.policies.registry_protocol.as_ref(),
                )
            }
            ContractVersion::ProviderApi(supplied) => {
                CompatibilityCheck::from_policy(
                    ContractKind::ProviderApi,
                    supplied,
                    self.policies.provider_api.as_ref(),
                )
            }
            ContractVersion::TaskProtocol(supplied) => {
                CompatibilityCheck::from_policy(
                    ContractKind::TaskProtocol,
                    supplied,
                    self.policies.task_protocol.as_ref(),
                )
            }
        }
    }

    /// Evaluate multiple toolchain-owned version contracts.
    ///
    /// The iterator determines the number of checks. No fixed target,
    /// provider, package, or contract count is imposed.
    pub fn check_all<I>(
        &self,
        versions: I,
    ) -> CompatibilityReport
    where
        I: IntoIterator<Item = ContractVersion>,
    {
        let checks = versions
            .into_iter()
            .map(|version| self.check(version))
            .collect();

        CompatibilityReport { checks }
    }

    /// Evaluate the toolchain API version supplied by a request.
    #[must_use]
    pub fn check_toolchain_api(
        &self,
        supplied: ToolchainApiVersion,
    ) -> CompatibilityCheck {
        self.check(ContractVersion::ToolchainApi(supplied))
    }

    /// Evaluate a manifest schema version.
    #[must_use]
    pub fn check_manifest_schema(
        &self,
        supplied: ManifestSchemaVersion,
    ) -> CompatibilityCheck {
        self.check(ContractVersion::ManifestSchema(supplied))
    }

    /// Evaluate a package archive format version.
    #[must_use]
    pub fn check_package_format(
        &self,
        supplied: PackageFormatVersion,
    ) -> CompatibilityCheck {
        self.check(ContractVersion::PackageFormat(supplied))
    }

    /// Evaluate a registry protocol version.
    #[must_use]
    pub fn check_registry_protocol(
        &self,
        supplied: RegistryProtocolVersion,
    ) -> CompatibilityCheck {
        self.check(ContractVersion::RegistryProtocol(supplied))
    }

    /// Evaluate a provider API version.
    #[must_use]
    pub fn check_provider_api(
        &self,
        supplied: ProviderApiVersion,
    ) -> CompatibilityCheck {
        self.check(ContractVersion::ProviderApi(supplied))
    }

    /// Evaluate a task protocol version.
    #[must_use]
    pub fn check_task_protocol(
        &self,
        supplied: TaskProtocolVersion,
    ) -> CompatibilityCheck {
        self.check(ContractVersion::TaskProtocol(supplied))
    }

    /// Aggregate a decision produced by another authoritative subsystem.
    ///
    /// This is the integration point for language, grammar, frontend,
    /// semantic, compiler, IR, artifact, ABI, backend, hardware, runtime,
    /// and distributed compatibility authorities.
    ///
    /// The supplied `CompatibilityDecision` remains authoritative.
    pub fn check_external(
        &self,
        authority: impl Into<String>,
        contract: impl Into<String>,
        source_version: impl Into<String>,
        target_version: impl Into<String>,
        decision: CompatibilityDecision,
    ) -> ToolchainResult<CompatibilityCheck> {
        CompatibilityCheck::from_authority(
            authority,
            contract,
            source_version,
            target_version,
            decision,
        )
    }
}

impl Default for CompatibilityChecker {
    fn default() -> Self {
        Self::conservative_initial()
    }
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn validate_metadata(
    field: &str,
    value: &str,
) -> ToolchainResult<()> {
    if value.is_empty() || value.trim().is_empty() {
        return Err(invalid_compatibility_data(format!(
            "{field} must not be empty",
        )));
    }

    if value != value.trim() {
        return Err(invalid_compatibility_data(format!(
            "{field} must not contain surrounding whitespace",
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_compatibility_data(format!(
            "{field} must not contain control characters",
        )));
    }

    Ok(())
}

fn invalid_compatibility_data(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ValidationFailed,
        message,
    )
    .with_operation("toolchain.compatibility.validate")
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::version::{
        VersionRange,
        INITIAL_TOOLCHAIN_API_VERSION,
    };
    use crate::zuir::version::{
        Compatibility,
        CompatibilityReason as CanonicalReason,
        CompatibilityDecision,
        VersionTriple,
    };

    fn checker() -> CompatibilityChecker {
        CompatibilityChecker::conservative_initial()
    }

    #[test]
    fn initial_api_version_is_compatible() {
        let result = checker().check_toolchain_api(
            INITIAL_TOOLCHAIN_API_VERSION,
        );

        assert_eq!(result.status(), CompatibilityStatus::Compatible);
        assert_eq!(result.reason(), CompatibilityReason::ExactMatch);
        assert!(result.is_compatible());
    }

    #[test]
    fn future_api_version_is_rejected() {
        let result = checker().check_toolchain_api(
            ToolchainApiVersion::new(2, 0, 0),
        );

        assert_eq!(result.status(), CompatibilityStatus::Incompatible);
        assert_eq!(result.reason(), CompatibilityReason::MajorVersionMismatch);
        assert!(!result.is_compatible());
    }

    #[test]
    fn future_minor_version_is_rejected() {
        let result = checker().check_toolchain_api(
            ToolchainApiVersion::new(1, 1, 0),
        );

        assert_eq!(result.status(), CompatibilityStatus::Incompatible);
        assert_eq!(result.reason(), CompatibilityReason::FutureVersion);
    }

    #[test]
    fn older_supported_version_can_require_migration() {
        let current = ToolchainApiVersion::new(1, 2, 0);
        let minimum = ToolchainApiVersion::new(1, 0, 0);

        let policy = VersionPolicy::new(
            current,
            VersionRange::new(minimum, current)
                .expect("valid version range"),
            false,
        )
        .expect("valid version policy");

        let policies = CompatibilityPolicies {
            toolchain_api: Some(policy),
            manifest_schema: None,
            package_format: None,
            registry_protocol: None,
            provider_api: None,
            task_protocol: None,
        };

        let checker = CompatibilityChecker::new(policies)
            .expect("valid checker");

        let result = checker.check_toolchain_api(
            ToolchainApiVersion::new(1, 0, 0),
        );

        assert_eq!(
            result.status(),
            CompatibilityStatus::RequiresMigration,
        );
        assert_eq!(
            result.reason(),
            CompatibilityReason::MigrationRequired,
        );
    }

    #[test]
    fn explicitly_supported_older_version_can_be_accepted() {
        let current = ToolchainApiVersion::new(1, 2, 0);
        let minimum = ToolchainApiVersion::new(1, 0, 0);

        let policy = VersionPolicy::new(
            current,
            VersionRange::new(minimum, current)
                .expect("valid version range"),
            true,
        )
        .expect("valid version policy");

        let policies = CompatibilityPolicies {
            toolchain_api: Some(policy),
            manifest_schema: None,
            package_format: None,
            registry_protocol: None,
            provider_api: None,
            task_protocol: None,
        };

        let checker = CompatibilityChecker::new(policies)
            .expect("valid checker");

        let result = checker.check_toolchain_api(
            ToolchainApiVersion::new(1, 0, 0),
        );

        assert_eq!(result.status(), CompatibilityStatus::Compatible);
        assert_eq!(
            result.reason(),
            CompatibilityReason::SupportedOlderVersion,
        );
    }

    #[test]
    fn missing_policy_fails_closed() {
        let policies = CompatibilityPolicies {
            toolchain_api: None,
            manifest_schema: None,
            package_format: None,
            registry_protocol: None,
            provider_api: None,
            task_protocol: None,
        };

        let checker = CompatibilityChecker::new(policies)
            .expect("empty policy set is structurally valid");

        let result = checker.check_toolchain_api(
            INITIAL_TOOLCHAIN_API_VERSION,
        );

        assert_eq!(
            result.status(),
            CompatibilityStatus::PolicyUnavailable,
        );
        assert_eq!(
            result.reason(),
            CompatibilityReason::PolicyNotConfigured,
        );
        assert!(!result.is_compatible());
    }

    #[test]
    fn every_toolchain_version_domain_is_independent() {
        let checker = checker();

        let checks = checker.check_all([
            ContractVersion::ToolchainApi(
                ToolchainApiVersion::new(1, 0, 0),
            ),
            ContractVersion::ManifestSchema(
                ManifestSchemaVersion::new(1, 0, 0),
            ),
            ContractVersion::PackageFormat(
                PackageFormatVersion::new(1, 0, 0),
            ),
            ContractVersion::RegistryProtocol(
                RegistryProtocolVersion::new(1, 0, 0),
            ),
            ContractVersion::ProviderApi(
                ProviderApiVersion::new(1, 0, 0),
            ),
            ContractVersion::TaskProtocol(
                TaskProtocolVersion::new(1, 0, 0),
            ),
        ]);

        assert_eq!(checks.len(), 6);
        assert!(checks.is_compatible());
    }

    #[test]
    fn empty_report_is_not_compatible() {
        let report = CompatibilityReport::new();

        assert!(report.is_empty());
        assert!(!report.is_compatible());
        assert!(report.has_blockers());
    }

    #[test]
    fn report_blocks_when_any_contract_is_incompatible() {
        let checker = checker();

        let report = checker.check_all([
            ContractVersion::ToolchainApi(
                INITIAL_TOOLCHAIN_API_VERSION,
            ),
            ContractVersion::PackageFormat(
                PackageFormatVersion::new(2, 0, 0),
            ),
        ]);

        assert!(!report.is_compatible());
        assert!(report.has_blockers());
        assert!(report.has_incompatible_contracts());
    }

    #[test]
    fn report_detects_migration_requirements() {
        let current = ToolchainApiVersion::new(1, 2, 0);
        let policy = VersionPolicy::new(
            current,
            VersionRange::new(
                ToolchainApiVersion::new(1, 0, 0),
                current,
            )
            .expect("valid range"),
            false,
        )
        .expect("valid policy");

        let policies = CompatibilityPolicies {
            toolchain_api: Some(policy),
            manifest_schema: None,
            package_format: None,
            registry_protocol: None,
            provider_api: None,
            task_protocol: None,
        };

        let checker = CompatibilityChecker::new(policies)
            .expect("valid checker");

        let report = checker.check_all([
            ContractVersion::ToolchainApi(
                ToolchainApiVersion::new(1, 0, 0),
            ),
        ]);

        assert!(report.requires_migration());
        assert!(!report.is_compatible());
    }

    #[test]
    fn external_authority_decision_is_preserved() {
        let decision = CompatibilityDecision::new(
            Compatibility::Compatible,
            CanonicalReason::ExactMatch,
            VersionTriple::new(1, 0, 0),
            VersionTriple::new(1, 0, 0),
        );

        let check = checker()
            .check_external(
                "compiler",
                "compiler_ir",
                "1.0.0",
                "1.0.0",
                decision,
            )
            .expect("valid external compatibility evidence");

        assert!(check.is_compatible());
        assert_eq!(
            check.status(),
            CompatibilityStatus::Compatible,
        );

        match check.contract() {
            CompatibilityContract::External {
                authority,
                contract,
            } => {
                assert_eq!(authority, "compiler");
                assert_eq!(contract, "compiler_ir");
            }
            _ => panic!("expected external contract"),
        }
    }

    #[test]
    fn external_authority_metadata_is_validated() {
        let decision = CompatibilityDecision::new(
            Compatibility::Compatible,
            CanonicalReason::ExactMatch,
            VersionTriple::new(1, 0, 0),
            VersionTriple::new(1, 0, 0),
        );

        assert!(
            checker()
                .check_external(
                    " ",
                    "compiler_ir",
                    "1.0.0",
                    "1.0.0",
                    decision,
                )
                .is_err()
        );

        assert!(
            checker()
                .check_external(
                    "compiler",
                    "compiler_ir",
                    "1.0.0\n",
                    "1.0.0",
                    decision,
                )
                .is_err()
        );
    }

    #[test]
    fn report_round_trips_through_serde() {
        let report = checker().check_all([
            ContractVersion::ToolchainApi(
                INITIAL_TOOLCHAIN_API_VERSION,
            ),
        ]);

        let serialized = serde_json::to_string(&report)
            .expect("serialize report");

        let decoded: CompatibilityReport =
            serde_json::from_str(&serialized)
                .expect("deserialize report");

        decoded.validate().expect("validate decoded report");

        assert_eq!(decoded, report);
        assert!(decoded.is_compatible());
    }

    #[test]
    fn malformed_deserialized_report_is_rejected_by_validation() {
        let serialized = r#"{
            "checks": [{
                "contract": {
                    "external": {
                        "authority": " ",
                        "contract": "compiler_ir"
                    }
                },
                "supplied_version": "1.0.0",
                "current_version": "1.0.0",
                "supported_versions": null,
                "status": "compatible",
                "reason": "exact_match"
            }]
        }"#;

        let report: CompatibilityReport =
            serde_json::from_str(serialized)
                .expect("structurally valid JSON");

        assert!(report.validate().is_err());
    }

    #[test]
    fn report_accepts_a_large_dynamic_number_of_checks() {
        let mut report = CompatibilityReport::new();

        for _ in 0..1024 {
            let check = checker().check_toolchain_api(
                INITIAL_TOOLCHAIN_API_VERSION,
            );

            report.push(check).expect("valid check");
        }

        assert_eq!(report.len(), 1024);
        assert!(report.is_compatible());
    }
}
