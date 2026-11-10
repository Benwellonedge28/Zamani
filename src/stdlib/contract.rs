
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Standard Library — Public API Contracts
//!
//! Path: `src/stdlib/contract.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Defines the public compatibility and stability contracts governing
//! the Zamani standard library.
//!
//! This module provides a common way for standard-library components
//! to declare:
//!
//! - API identity;
//! - API version;
//! - stability guarantees;
//! - compatibility policy;
//! - compatibility dimensions;
//! - change-impact classification;
//! - deprecation information;
//! - portability guarantees;
//! - error-handling conventions;
//! - extension metadata.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns standard-library API compatibility policy.
//!
//! It does not own:
//!
//! - Zamani language semantics;
//! - grammar compatibility;
//! - compiler API compatibility;
//! - IR schema compatibility;
//! - serialization format compatibility;
//! - runtime compatibility decisions;
//! - hardware compatibility;
//! - platform discovery;
//! - semantic contracts;
//! - resource requirements;
//! - capability discovery.
//!
//! Those responsibilities remain with their authoritative subsystems.
//!
//! ## VERSION IDENTITY
//!
//! API contract versions use:
//!
//! `crate::stdlib::version::Version`
//!
//! They must not be confused with:
//!
//! - Cargo package version;
//! - Zamani language version;
//! - grammar version;
//! - compiler version;
//! - IR version;
//! - runtime version;
//! - serialization schema version.
//!
//! ## SCALABILITY
//!
//! This module introduces no universal hardware or resource ceilings.
//!
//! Contract metadata is extensible.
//!
//! Component identifiers are not restricted to a predefined list.
//!
//! Compatibility dimensions can be extended.
//!
//! Operational metadata storage is limited only by available resources
//! and any explicitly configured operational budgets.
//!
//! ## DEPENDENCIES
//!
//! This module depends only on:
//!
//! - `crate::stdlib::version::Version`;
//! - Rust's standard library.
//!
//! It must remain independent of domain-specific implementations.

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use super::version::Version;

// ============================================================================
// CONTRACT IDENTITY
// ============================================================================

/// Identifies a standard-library API contract.
///
/// Identifiers are extensible and are not restricted to a predefined
/// list of computing domains.
///
/// Examples:
///
/// - `stdlib.foundation.collections`
/// - `stdlib.foundation.numeric`
/// - `stdlib.io.filesystem`
/// - `stdlib.quantum`
/// - `stdlib.hdl`
/// - `stdlib.distributed`
///
/// An identifier describes an API contract, not a hardware device.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContractId(String);

impl ContractId {
    /// Creates a validated contract identifier.
    ///
    /// Identifiers must:
    ///
    /// - be non-empty;
    /// - not contain leading or trailing whitespace;
    /// - not contain control characters.
    ///
    /// No fixed identifier-length ceiling is imposed.
    pub fn new(value: impl Into<String>) -> Result<Self, ContractError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ContractError::EmptyIdentifier);
        }

        if value.trim() != value {
            return Err(ContractError::InvalidIdentifier);
        }

        if value.chars().any(char::is_control) {
            return Err(ContractError::InvalidIdentifier);
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ContractId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// API STABILITY
// ============================================================================

/// Public API stability classification.
///
/// Stability describes the strength of the API compatibility promise.
///
/// It does not describe implementation maturity or runtime availability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stability {
    /// API is experimental and may change without a stable compatibility
    /// guarantee.
    Experimental,

    /// API is under active development.
    ///
    /// Changes should be documented, but compatibility is not guaranteed.
    Development,

    /// API is available for evaluation and adoption.
    ///
    /// Compatibility guarantees are limited by its documented policy.
    Preview,

    /// API is stable and follows its declared compatibility policy.
    Stable,

    /// API is deprecated and should no longer be used for new development.
    Deprecated,

    /// API is retained for compatibility but is no longer actively developed.
    Legacy,
}

impl Stability {
    /// Returns whether the API promises stable compatibility.
    #[must_use]
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// Returns whether the API is deprecated.
    #[must_use]
    pub const fn is_deprecated(self) -> bool {
        matches!(self, Self::Deprecated)
    }

    /// Returns whether the API is experimental.
    #[must_use]
    pub const fn is_experimental(self) -> bool {
        matches!(self, Self::Experimental)
    }
}

impl fmt::Display for Stability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Experimental => "experimental",
            Self::Development => "development",
            Self::Preview => "preview",
            Self::Stable => "stable",
            Self::Deprecated => "deprecated",
            Self::Legacy => "legacy",
        };

        f.write_str(value)
    }
}

// ============================================================================
// COMPATIBILITY DIMENSIONS
// ============================================================================

/// Compatibility dimension.
///
/// Each dimension represents a distinct compatibility promise.
///
/// New dimensions can be introduced without changing the existing
/// standard dimensions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompatibilityDimension {
    /// Compatibility of public source-level APIs.
    Source,

    /// Compatibility of compiled binary interfaces where applicable.
    Binary,

    /// Compatibility of documented observable behavior.
    Behavioral,

    /// Compatibility of semantic meaning.
    Semantic,

    /// Compatibility of public data representations.
    Data,

    /// Compatibility of serialized representations.
    Serialization,

    /// Compatibility of documented portability guarantees.
    Portability,

    /// Compatibility of public extension interfaces.
    Extension,

    /// Compatibility of documentation guarantees where normative.
    Documentation,

    /// A future or organization-specific compatibility dimension.
    Custom(String),
}

impl CompatibilityDimension {
    /// Creates an extensible custom compatibility dimension.
    pub fn custom(value: impl Into<String>) -> Result<Self, ContractError> {
        let value = value.into();

        if value.is_empty()
            || value.trim() != value
            || value.chars().any(char::is_control)
        {
            return Err(ContractError::InvalidIdentifier);
        }

        Ok(Self::Custom(value))
    }

    /// Returns the canonical dimension name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Source => "source",
            Self::Binary => "binary",
            Self::Behavioral => "behavioral",
            Self::Semantic => "semantic",
            Self::Data => "data",
            Self::Serialization => "serialization",
            Self::Portability => "portability",
            Self::Extension => "extension",
            Self::Documentation => "documentation",
            Self::Custom(value) => value,
        }
    }
}

impl fmt::Display for CompatibilityDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// COMPATIBILITY POLICY
// ============================================================================

/// Policy governing compatibility guarantees.
///
/// Every dimension has an explicit policy.
///
/// No assumption is made that all APIs must provide binary compatibility.
///
/// A component must declare the dimensions relevant to its actual API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityPolicy {
    /// Breaking changes are prohibited under the declared contract.
    ///
    /// An intentional breaking change requires a new compatibility
    /// contract or an explicitly approved migration.
    Strict,

    /// Breaking changes require explicit versioning and migration
    /// documentation.
    Versioned,

    /// Breaking changes are permitted with documented migration guidance.
    Managed,

    /// No compatibility guarantee is provided.
    Unspecified,
}

impl CompatibilityPolicy {
    /// Returns whether this policy permits breaking changes.
    #[must_use]
    pub const fn permits_breaking_changes(self) -> bool {
        matches!(self, Self::Versioned | Self::Managed | Self::Unspecified)
    }

    /// Returns whether a breaking change requires migration documentation.
    #[must_use]
    pub const fn requires_migration(self) -> bool {
        matches!(self, Self::Versioned | Self::Managed)
    }
}

impl Default for CompatibilityPolicy {
    fn default() -> Self {
        Self::Strict
    }
}

// ============================================================================
// CHANGE IMPACT
// ============================================================================

/// Classification of an API change.
///
/// This is an explicit declaration by the change author.
///
/// It is not automatically inferred from a version number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChangeImpact {
    /// No externally observable compatibility change.
    None,

    /// A change that preserves documented compatibility.
    Compatible,

    /// A change that adds functionality without invalidating existing
    /// documented behavior.
    Additive,

    /// A change that requires callers to modify their usage.
    Breaking,

    /// A change that modifies documented observable semantics.
    Semantic,

    /// A change that modifies a data representation or format.
    DataFormat,

    /// A change that affects portability guarantees.
    Portability,

    /// A change that introduces a security-relevant compatibility change.
    Security,
}

impl ChangeImpact {
    /// Returns whether the change is potentially breaking.
    #[must_use]
    pub const fn is_breaking(self) -> bool {
        matches!(
            self,
            Self::Breaking
                | Self::Semantic
                | Self::DataFormat
                | Self::Portability
                | Self::Security
        )
    }
}

// ============================================================================
// COMPATIBILITY ASSESSMENT
// ============================================================================

/// Result of evaluating a change against a declared compatibility policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatibilityAssessment {
    /// The change satisfies the declared compatibility policy.
    Compatible,

    /// The change is permitted only with an explicit migration.
    MigrationRequired,

    /// The change violates the declared compatibility policy.
    Incompatible,

    /// The contract does not provide a guarantee for this dimension.
    Unspecified,
}

impl CompatibilityAssessment {
    /// Returns whether the change is accepted without migration.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        matches!(self, Self::Compatible)
    }

    /// Returns whether migration is required.
    #[must_use]
    pub const fn requires_migration(&self) -> bool {
        matches!(self, Self::MigrationRequired)
    }
}

// ============================================================================
// DEPRECATION CONTRACT
// ============================================================================

/// Deprecation information for an API contract.
///
/// Deprecation is a documented lifecycle state, not an immediate removal.
///
/// A removal version is optional because some APIs may remain available
/// indefinitely for compatibility.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deprecation {
    since: Version,
    removal_version: Option<Version>,
    replacement: Option<ContractId>,
    reason: String,
    migration_guidance: Option<String>,
}

impl Deprecation {
    /// Creates deprecation metadata.
    ///
    /// When a removal version is supplied, it must have strictly greater
    /// semantic precedence than the deprecation version.
    pub fn new(
        since: Version,
        removal_version: Option<Version>,
        replacement: Option<ContractId>,
        reason: impl Into<String>,
        migration_guidance: Option<String>,
    ) -> Result<Self, ContractError> {
        let reason = reason.into();

        if reason.trim().is_empty() {
            return Err(ContractError::MissingDeprecationReason);
        }

        if let Some(removal) = &removal_version {
            if removal.precedence_cmp(&since) != Ordering::Greater {
                return Err(ContractError::InvalidRemovalVersion);
            }
        }

        Ok(Self {
            since,
            removal_version,
            replacement,
            reason,
            migration_guidance,
        })
    }

    /// Returns the version in which deprecation began.
    #[must_use]
    pub fn since(&self) -> &Version {
        &self.since
    }

    /// Returns the planned removal version.
    #[must_use]
    pub fn removal_version(&self) -> Option<&Version> {
        self.removal_version.as_ref()
    }

    /// Returns the replacement API, if one exists.
    #[must_use]
    pub fn replacement(&self) -> Option<&ContractId> {
        self.replacement.as_ref()
    }

    /// Returns the deprecation reason.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }

    /// Returns migration guidance.
    #[must_use]
    pub fn migration_guidance(&self) -> Option<&str> {
        self.migration_guidance.as_deref()
    }
}

// ============================================================================
// PORTABILITY GUARANTEES
// ============================================================================

/// Portability guarantee provided by a standard-library API.
///
/// Portability guarantees describe API behavior.
///
/// They do not promise that every physical target can execute every
/// possible operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortabilityGuarantee {
    /// The API is independent of a particular hardware architecture.
    ///
    /// Target-specific execution requirements may still exist.
    ArchitectureIndependent,

    /// The API provides consistent source-level behavior across supported
    /// platforms.
    SourcePortable,

    /// The API provides documented observable behavior across supported
    /// implementations.
    BehaviorPortable,

    /// The API requires an explicitly declared platform capability.
    CapabilityDependent,

    /// The API is tied to a specific implementation environment.
    ImplementationSpecific,

    /// The API makes no portability guarantee.
    Unspecified,
}

// ============================================================================
// ERROR CONVENTIONS
// ============================================================================

/// Standard-library error-handling convention.
///
/// This describes how an API communicates failure.
///
/// It does not replace the shared structured error implementation
/// planned for `error.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorConvention {
    /// Errors are returned through Rust Result.
    Result,

    /// Absence is represented through Option where absence is expected.
    Option,

    /// The API explicitly documents infallibility.
    Infallible,

    /// Both absence and operational failure are represented distinctly.
    ResultWithOptionalValue,

    /// Failure is represented through an explicitly documented
    /// asynchronous completion mechanism.
    AsynchronousResult,
}

// ============================================================================
// COMPATIBILITY GUARANTEE
// ============================================================================

/// Compatibility guarantee for one API dimension.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityGuarantee {
    dimension: CompatibilityDimension,
    policy: CompatibilityPolicy,
}

impl CompatibilityGuarantee {
    /// Creates a compatibility guarantee.
    #[must_use]
    pub fn new(
        dimension: CompatibilityDimension,
        policy: CompatibilityPolicy,
    ) -> Self {
        Self { dimension, policy }
    }

    /// Returns the compatibility dimension.
    #[must_use]
    pub fn dimension(&self) -> &CompatibilityDimension {
        &self.dimension
    }

    /// Returns the compatibility policy.
    #[must_use]
    pub const fn policy(&self) -> CompatibilityPolicy {
        self.policy
    }

    /// Evaluates a declared change against this guarantee.
    #[must_use]
    pub fn assess(
        &self,
        impact: ChangeImpact,
    ) -> CompatibilityAssessment {
        if !impact.is_breaking() {
            return CompatibilityAssessment::Compatible;
        }

        match self.policy {
            CompatibilityPolicy::Strict => {
                CompatibilityAssessment::Incompatible
            }

            CompatibilityPolicy::Versioned => {
                CompatibilityAssessment::MigrationRequired
            }

            CompatibilityPolicy::Managed => {
                CompatibilityAssessment::MigrationRequired
            }

            CompatibilityPolicy::Unspecified => {
                CompatibilityAssessment::Unspecified
            }
        }
    }
}

// ============================================================================
// CONTRACT ERROR
// ============================================================================

/// Standard-library contract validation errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractError {
    /// Contract identifier is empty.
    EmptyIdentifier,

    /// Contract identifier is malformed.
    InvalidIdentifier,

    /// Required contract information is missing.
    MissingRequiredField(&'static str),

    /// A compatibility dimension was declared more than once.
    DuplicateCompatibilityDimension(String),

    /// A compatibility dimension was not declared.
    UnknownCompatibilityDimension(String),

    /// Deprecation reason is missing.
    MissingDeprecationReason,

    /// Removal version does not follow the deprecation version.
    InvalidRemovalVersion,

    /// Deprecation metadata conflicts with the declared stability state.
    InvalidDeprecationState,

    /// The contract contains inconsistent policy information.
    InconsistentContract,
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => {
                f.write_str("contract identifier cannot be empty")
            }

            Self::InvalidIdentifier => {
                f.write_str("contract identifier is invalid")
            }

            Self::MissingRequiredField(field) => {
                write!(f, "missing required contract field: {field}")
            }

            Self::DuplicateCompatibilityDimension(dimension) => {
                write!(
                    f,
                    "duplicate compatibility dimension: {dimension}"
                )
            }

            Self::UnknownCompatibilityDimension(dimension) => {
                write!(
                    f,
                    "unknown compatibility dimension: {dimension}"
                )
            }

            Self::MissingDeprecationReason => {
                f.write_str("deprecation reason cannot be empty")
            }

            Self::InvalidRemovalVersion => {
                f.write_str(
                    "removal version must follow the deprecation version"
                )
            }

            Self::InvalidDeprecationState => {
                f.write_str(
                    "deprecation metadata conflicts with API stability"
                )
            }

            Self::InconsistentContract => {
                f.write_str("standard-library contract is inconsistent")
            }
        }
    }
}

impl Error for ContractError {}

// ============================================================================
// API CONTRACT
// ============================================================================

/// Complete public contract for a standard-library API.
///
/// A contract describes guarantees, not implementation details.
///
/// The contract is immutable after construction. To publish a revised
/// contract, construct a new value with an updated version and metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiContract {
    id: ContractId,
    version: Version,
    stability: Stability,
    guarantees: BTreeMap<
        CompatibilityDimension,
        CompatibilityPolicy,
    >,
    portability: PortabilityGuarantee,
    error_convention: ErrorConvention,
    deprecation: Option<Deprecation>,
    metadata: BTreeMap<String, String>,
}

impl ApiContract {
    /// Returns a builder for a new API contract.
    #[must_use]
    pub fn builder(
        id: ContractId,
        version: Version,
    ) -> ApiContractBuilder {
        ApiContractBuilder::new(id, version)
    }

    /// Returns the contract identifier.
    #[must_use]
    pub fn id(&self) -> &ContractId {
        &self.id
    }

    /// Returns the API contract version.
    #[must_use]
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Returns the stability classification.
    #[must_use]
    pub const fn stability(&self) -> Stability {
        self.stability
    }

    /// Returns the portability guarantee.
    #[must_use]
    pub const fn portability(&self) -> PortabilityGuarantee {
        self.portability
    }

    /// Returns the error convention.
    #[must_use]
    pub const fn error_convention(&self) -> ErrorConvention {
        self.error_convention
    }

    /// Returns deprecation information.
    #[must_use]
    pub fn deprecation(&self) -> Option<&Deprecation> {
        self.deprecation.as_ref()
    }

    /// Returns all declared compatibility guarantees.
    #[must_use]
    pub fn guarantees(
        &self,
    ) -> &BTreeMap<CompatibilityDimension, CompatibilityPolicy> {
        &self.guarantees
    }

    /// Returns one compatibility policy.
    #[must_use]
    pub fn guarantee(
        &self,
        dimension: &CompatibilityDimension,
    ) -> Option<CompatibilityPolicy> {
        self.guarantees.get(dimension).copied()
    }

    /// Returns extension metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Returns one metadata value.
    #[must_use]
    pub fn metadata_value(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(String::as_str)
    }

    /// Assesses a proposed change against a declared compatibility
    /// dimension.
    ///
    /// An undeclared dimension is reported as unspecified rather than
    /// being assumed compatible.
    #[must_use]
    pub fn assess_change(
        &self,
        dimension: &CompatibilityDimension,
        impact: ChangeImpact,
    ) -> CompatibilityAssessment {
        match self.guarantees.get(dimension) {
            Some(policy) => {
                CompatibilityGuarantee::new(
                    dimension.clone(),
                    *policy,
                )
                .assess(impact)
            }

            None => CompatibilityAssessment::Unspecified,
        }
    }

    /// Returns whether this contract declares a guarantee for a dimension.
    #[must_use]
    pub fn declares(&self, dimension: &CompatibilityDimension) -> bool {
        self.guarantees.contains_key(dimension)
    }
}

// ============================================================================
// API CONTRACT BUILDER
// ============================================================================

/// Builder for constructing validated API contracts.
///
/// Builders permit incremental configuration but produce immutable
/// contracts only after validation.
#[derive(Debug, Clone)]
pub struct ApiContractBuilder {
    id: ContractId,
    version: Version,
    stability: Stability,
    guarantees: BTreeMap<
        CompatibilityDimension,
        CompatibilityPolicy,
    >,
    portability: PortabilityGuarantee,
    error_convention: ErrorConvention,
    deprecation: Option<Deprecation>,
    metadata: BTreeMap<String, String>,
}

impl ApiContractBuilder {
    /// Creates a new builder.
    #[must_use]
    pub fn new(id: ContractId, version: Version) -> Self {
        Self {
            id,
            version,
            stability: Stability::Experimental,
            guarantees: BTreeMap::new(),
            portability: PortabilityGuarantee::Unspecified,
            error_convention: ErrorConvention::Result,
            deprecation: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Sets API stability.
    #[must_use]
    pub fn stability(mut self, stability: Stability) -> Self {
        self.stability = stability;
        self
    }

    /// Declares a compatibility guarantee.
    ///
    /// Repeated declarations for the same dimension are rejected.
    pub fn guarantee(
        &mut self,
        dimension: CompatibilityDimension,
        policy: CompatibilityPolicy,
    ) -> Result<&mut Self, ContractError> {
        let key = dimension.to_string();

        if self.guarantees.contains_key(&dimension) {
            return Err(
                ContractError::DuplicateCompatibilityDimension(key)
            );
        }

        self.guarantees.insert(dimension, policy);
        Ok(self)
    }

    /// Sets the portability guarantee.
    #[must_use]
    pub fn portability(
        mut self,
        portability: PortabilityGuarantee,
    ) -> Self {
        self.portability = portability;
        self
    }

    /// Sets the error convention.
    #[must_use]
    pub fn error_convention(
        mut self,
        convention: ErrorConvention,
    ) -> Self {
        self.error_convention = convention;
        self
    }

    /// Sets deprecation metadata.
    #[must_use]
    pub fn deprecation(mut self, deprecation: Deprecation) -> Self {
        self.deprecation = Some(deprecation);
        self
    }

    /// Inserts extension metadata.
    ///
    /// Keys must be non-empty and must not contain control characters.
    ///
    /// Existing values are replaced intentionally.
    pub fn metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<&mut Self, ContractError> {
        let key = key.into();

        if key.is_empty()
            || key.trim() != key
            || key.chars().any(char::is_control)
        {
            return Err(ContractError::InvalidIdentifier);
        }

        self.metadata.insert(key, value.into());
        Ok(self)
    }

    /// Validates and builds the API contract.
    pub fn build(self) -> Result<ApiContract, ContractError> {
        if self.stability == Stability::Deprecated
            && self.deprecation.is_none()
        {
            return Err(
                ContractError::MissingRequiredField("deprecation")
            );
        }

        if self.stability != Stability::Deprecated
            && self.deprecation.is_some()
        {
            return Err(ContractError::InvalidDeprecationState);
        }

        Ok(ApiContract {
            id: self.id,
            version: self.version,
            stability: self.stability,
            guarantees: self.guarantees,
            portability: self.portability,
            error_convention: self.error_convention,
            deprecation: self.deprecation,
            metadata: self.metadata,
        })
    }
}

// ============================================================================
// CONTRACT VALIDATION
// ============================================================================

/// Validates the relationship between two API contract versions.
///
/// Version precedence alone cannot establish compatibility.
///
/// A version change must be accompanied by an explicit change-impact
/// assessment for every relevant compatibility dimension.
#[must_use]
pub fn compare_contract_versions(
    previous: &ApiContract,
    current: &ApiContract,
) -> Ordering {
    current.version.precedence_cmp(&previous.version)
}

/// Evaluates whether a contract change is allowed by its declared policy.
#[must_use]
pub fn assess_contract_change(
    contract: &ApiContract,
    dimension: &CompatibilityDimension,
    impact: ChangeImpact,
) -> CompatibilityAssessment {
    contract.assess_change(dimension, impact)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn version(value: &str) -> Version {
        Version::parse(value).expect("valid test version")
    }

    fn contract_id(value: &str) -> ContractId {
        ContractId::new(value).expect("valid test identifier")
    }

    #[test]
    fn contract_identifier_accepts_extensible_names() {
        let id = contract_id("stdlib.quantum.future_provider");

        assert_eq!(
            id.as_str(),
            "stdlib.quantum.future_provider"
        );
    }

    #[test]
    fn contract_identifier_rejects_empty_values() {
        assert_eq!(
            ContractId::new(""),
            Err(ContractError::EmptyIdentifier)
        );
    }

    #[test]
    fn contract_identifier_rejects_control_characters() {
        assert_eq!(
            ContractId::new("stdlib\nquantum"),
            Err(ContractError::InvalidIdentifier)
        );
    }

    #[test]
    fn compatibility_dimension_supports_extensions() {
        let dimension =
            CompatibilityDimension::custom("future_protocol")
                .expect("valid custom dimension");

        assert_eq!(dimension.as_str(), "future_protocol");
    }

    #[test]
    fn strict_policy_rejects_breaking_changes() {
        let guarantee = CompatibilityGuarantee::new(
            CompatibilityDimension::Source,
            CompatibilityPolicy::Strict,
        );

        assert_eq!(
            guarantee.assess(ChangeImpact::Breaking),
            CompatibilityAssessment::Incompatible
        );
    }

    #[test]
    fn versioned_policy_requires_migration() {
        let guarantee = CompatibilityGuarantee::new(
            CompatibilityDimension::Source,
            CompatibilityPolicy::Versioned,
        );

        assert_eq!(
            guarantee.assess(ChangeImpact::Breaking),
            CompatibilityAssessment::MigrationRequired
        );
    }

    #[test]
    fn additive_changes_preserve_compatibility() {
        let guarantee = CompatibilityGuarantee::new(
            CompatibilityDimension::Source,
            CompatibilityPolicy::Strict,
        );

        assert_eq!(
            guarantee.assess(ChangeImpact::Additive),
            CompatibilityAssessment::Compatible
        );
    }

    #[test]
    fn undeclared_dimension_is_unspecified() {
        let contract = ApiContract::builder(
            contract_id("stdlib.foundation"),
            version("1.0.0"),
        )
        .build()
        .expect("valid contract");

        assert_eq!(
            contract.assess_change(
                &CompatibilityDimension::Binary,
                ChangeImpact::Breaking,
            ),
            CompatibilityAssessment::Unspecified
        );
    }

    #[test]
    fn contract_builder_rejects_duplicate_dimensions() {
        let mut builder = ApiContract::builder(
            contract_id("stdlib.foundation"),
            version("1.0.0"),
        );

        builder
            .guarantee(
                CompatibilityDimension::Source,
                CompatibilityPolicy::Strict,
            )
            .expect("first declaration");

        assert!(matches!(
            builder.guarantee(
                CompatibilityDimension::Source,
                CompatibilityPolicy::Managed,
            ),
            Err(ContractError::DuplicateCompatibilityDimension(_))
        ));
    }

    #[test]
    fn contract_builder_accepts_multiple_dimensions() {
        let mut builder = ApiContract::builder(
            contract_id("stdlib.foundation.collections"),
            version("1.0.0"),
        )
        .stability(Stability::Stable)
        .portability(
            PortabilityGuarantee::ArchitectureIndependent,
        );

        builder
            .guarantee(
                CompatibilityDimension::Source,
                CompatibilityPolicy::Strict,
            )
            .expect("source guarantee");

        builder
            .guarantee(
                CompatibilityDimension::Behavioral,
                CompatibilityPolicy::Strict,
            )
            .expect("behavioral guarantee");

        let contract = builder.build().expect("valid contract");

        assert!(contract.declares(
            &CompatibilityDimension::Source
        ));

        assert!(contract.declares(
            &CompatibilityDimension::Behavioral
        ));
    }

    #[test]
    fn deprecated_contract_requires_deprecation_information() {
        let result = ApiContract::builder(
            contract_id("stdlib.legacy"),
            version("1.0.0"),
        )
        .stability(Stability::Deprecated)
        .build();

        assert_eq!(
            result.err(),
            Some(ContractError::MissingRequiredField("deprecation"))
        );
    }

    #[test]
    fn deprecation_requires_later_removal_version() {
        let result = Deprecation::new(
            version("2.0.0"),
            Some(version("2.0.0")),
            None,
            "Replaced by a newer API",
            None,
        );

        assert_eq!(
            result.err(),
            Some(ContractError::InvalidRemovalVersion)
        );
    }

    #[test]
    fn deprecation_accepts_valid_removal_version() {
        let deprecation = Deprecation::new(
            version("2.0.0"),
            Some(version("3.0.0")),
            None,
            "Replaced by a newer API",
            Some("Use the new API".to_owned()),
        )
        .expect("valid deprecation");

        assert_eq!(
            deprecation.removal_version(),
            Some(&version("3.0.0"))
        );
    }

    #[test]
    fn deprecated_contract_requires_consistent_state() {
        let deprecation = Deprecation::new(
            version("1.0.0"),
            None,
            None,
            "Legacy API",
            None,
        )
        .expect("valid deprecation");

        let result = ApiContract::builder(
            contract_id("stdlib.legacy"),
            version("1.0.0"),
        )
        .deprecation(deprecation)
        .build();

        assert_eq!(
            result.err(),
            Some(ContractError::InvalidDeprecationState)
        );
    }

    #[test]
    fn metadata_is_extensible_and_deterministic() {
        let mut builder = ApiContract::builder(
            contract_id("stdlib.foundation"),
            version("1.0.0"),
        );

        builder
            .metadata("owner", "foundation")
            .expect("valid metadata");

        builder
            .metadata("migration", "documented")
            .expect("valid metadata");

        let contract = builder.build().expect("valid contract");

        assert_eq!(
            contract.metadata_value("owner"),
            Some("foundation")
        );

        assert_eq!(
            contract.metadata().len(),
            2
        );
    }

    #[test]
    fn contract_version_comparison_uses_semver_precedence() {
        let previous = ApiContract::builder(
            contract_id("stdlib.foundation"),
            version("1.0.0"),
        )
        .build()
        .expect("previous contract");

        let current = ApiContract::builder(
            contract_id("stdlib.foundation"),
            version("2.0.0"),
        )
        .build()
        .expect("current contract");

        assert_eq!(
            compare_contract_versions(&previous, &current),
            Ordering::Greater
        );
    }

    #[test]
    fn contract_versions_do_not_imply_compatibility() {
        let contract = ApiContract::builder(
            contract_id("stdlib.foundation"),
            version("2.0.0"),
        )
        .build()
        .expect("valid contract");

        assert_eq!(
            contract.assess_change(
                &CompatibilityDimension::Source,
                ChangeImpact::Breaking,
            ),
            CompatibilityAssessment::Unspecified
        );
    }

    #[test]
    fn custom_dimension_and_large_version_components_are_supported() {
        let dimension =
            CompatibilityDimension::custom("future_execution_model")
                .expect("valid dimension");

        let contract = ApiContract::builder(
            contract_id("stdlib.future"),
            version(
                "999999999999999999999999999999999999999999.0.0"
            ),
        )
        .build()
        .expect("valid contract");

        assert_eq!(
            dimension.as_str(),
            "future_execution_model"
        );

        assert_eq!(
            contract.version().major(),
            "999999999999999999999999999999999999999999"
        );
    }
}
