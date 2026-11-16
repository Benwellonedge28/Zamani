
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain API — Version Negotiation
//!
//! Path: `src/toolchain/api/versioning.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Defines version negotiation and compatibility enforcement at the
//! public Zamani Toolchain API boundary.
//!
//! Supports:
//! - Client/server API version negotiation.
//! - Explicit supported-version ranges.
//! - Client preferred versions.
//! - Version compatibility checks.
//! - Request-dispatch compatibility gates.
//! - Structured negotiation results.
//! - Validation of deserialized negotiation metadata.
//!
//! ## OWNERSHIP
//!
//! This module owns API-level version negotiation only.
//!
//! It does not own:
//! - Version number representation.
//! - Manifest schema compatibility.
//! - Package archive compatibility.
//! - Registry protocol compatibility.
//! - Provider API compatibility.
//! - Task protocol compatibility.
//! - Language or grammar version compatibility.
//! - AST, semantic-model or IR compatibility.
//! - Compiler, backend, ABI or artifact compatibility.
//! - Hardware, firmware or runtime compatibility.
//! - Migration implementation.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! ## AUTHORITATIVE INTEGRATIONS
//!
//! - crate::toolchain::version
//! - crate::toolchain::api::compatibility
//! - crate::toolchain::errors
//! - crate::toolchain::api::request
//! - crate::toolchain::api::response
//! - crate::zuir::version
//!
//! ## COMPATIBILITY RULES
//!
//! A version number alone does not prove compatibility.
//!
//! Unknown, future, incompatible and migration-required versions
//! must never be silently accepted.
//!
//! A successful negotiation means that both parties have agreed
//! on an API contract version. It does not prove that every API
//! operation or underlying implementation is available.
//!
//! ## MIGRATION
//!
//! This module identifies migration requirements.
//! It never performs a migration.
//!
//! No request may be dispatched as compatible when migration
//! is required.
//!
//! ## POCO-REAF
//!
//! API version negotiation is independent of:
//! - Classical computing.
//! - Quantum computing.
//! - HDL and hardware design.
//! - AI and tensor computation.
//! - Hybrid computation.
//! - Distributed computation.
//! - Future computational domains.
//!
//! Adding a domain must not require changing this module.
//!
//! ## SCALABILITY
//!
//! No arbitrary limits are imposed on API clients, providers,
//! versioned operations or computing domains.
//!
//! Version ranges use the canonical repository representation.
//! Operational memory and processing remain subject to available
//! resources.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## SECURITY
//!
//! Negotiation metadata is untrusted when received externally.
//! It must be structurally validated before being relied upon.
//!
//! This module does not accept credentials or executable instructions.
//!
//! ## FREEZE CRITERIA
//!
//! - Version ownership remains with version.rs.
//! - Compatibility ownership remains with compatibility.rs.
//! - Client/server negotiation is deterministic.
//! - Future versions are rejected.
//! - Missing common versions are explicitly reported.
//! - Migration requirements block dispatch.
//! - Deserialized negotiation results can be validated.
//! - No unsafe code is used.
//! - All applicable tests pass.

use core::fmt;

use serde::{Deserialize, Serialize};

use crate::toolchain::api::compatibility::{
    CompatibilityReason,
    CompatibilityStatus,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
    ToolchainResult,
};

use crate::toolchain::version::{
    ToolchainApiVersion,
    VersionPolicy,
    VersionRange,
};

use crate::zuir::version::{
    Compatibility,
    CompatibilityReason as CanonicalCompatibilityReason,
    CompatibilityDecision,
};

// ============================================================================
// NEGOTIATION ERROR
// ============================================================================

/// Structural or semantic API version negotiation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ApiVersioningError {
    field: String,
    message: String,
}

impl ApiVersioningError {
    fn new(
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Return the invalid field or contract location.
    #[must_use]
    pub fn field(&self) -> &str {
        &self.field
    }

    /// Return the validation explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ApiVersioningError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "API versioning error at '{}': {}",
            self.field,
            self.message
        )
    }
}

impl std::error::Error for ApiVersioningError {}

/// Result of API versioning metadata validation.
pub type ApiVersioningResult<T> =
    core::result::Result<T, ApiVersioningError>;

// ============================================================================
// VERSION IDENTIFIER VALIDATION
// ============================================================================

fn validate_version_range(
    field: &str,
    range: &VersionRange<ToolchainApiVersion>,
) -> ApiVersioningResult<()> {
    range.validate().map_err(|error| {
        ApiVersioningError::new(field, error.to_string())
    })
}

// ============================================================================
// CLIENT API VERSION OFFER
// ============================================================================

/// Describes the API versions a client can consume.
///
/// `supported` is an inclusive version range.
///
/// `preferred` expresses a client preference. It must belong to
/// the declared supported range.
///
/// A preference never overrides server compatibility policy.
///
/// The offer does not authenticate the client or prove that the
/// client implements every operation in the advertised API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiVersionOffer {
    /// Versions the client declares that it supports.
    pub supported: VersionRange<ToolchainApiVersion>,

    /// Optional preferred API version.
    pub preferred: Option<ToolchainApiVersion>,
}

impl ApiVersionOffer {
    /// Construct and validate an API version offer.
    pub fn new(
        supported: VersionRange<ToolchainApiVersion>,
        preferred: Option<ToolchainApiVersion>,
    ) -> ApiVersioningResult<Self> {
        let offer = Self {
            supported,
            preferred,
        };

        offer.validate()?;

        Ok(offer)
    }

    /// Construct an offer for one exact API version.
    pub fn exact(
        version: ToolchainApiVersion,
    ) -> Self {
        Self {
            supported: VersionRange::new(version, version)
                .expect("an exact version range is valid"),
            preferred: Some(version),
        }
    }

    /// Validate the offer, including its preferred version.
    pub fn validate(&self) -> ApiVersioningResult<()> {
        validate_version_range(
            "client.supported",
            &self.supported,
        )?;

        if let Some(preferred) = self.preferred {
            if !self.supported.contains(preferred) {
                return Err(ApiVersioningError::new(
                    "client.preferred",
                    "preferred API version must belong to the supported range",
                ));
            }
        }

        Ok(())
    }

    /// Return whether the offer contains one exact version.
    #[must_use]
    pub fn is_exact(&self) -> bool {
        self.supported.is_exact()
    }
}

// ============================================================================
// API VERSION CHECK
// ============================================================================

/// Result of checking one client API version against server policy.
///
/// This result is descriptive metadata. Its validation does not
/// independently establish trust in the policy that produced it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiVersionCheck {
    requested: ToolchainApiVersion,
    current: ToolchainApiVersion,
    status: CompatibilityStatus,
    reason: CompatibilityReason,
}

impl ApiVersionCheck {
    fn from_decision(
        requested: ToolchainApiVersion,
        current: ToolchainApiVersion,
        decision: CompatibilityDecision,
    ) -> Self {
        let status = match decision.compatibility() {
            Compatibility::Compatible => {
                CompatibilityStatus::Compatible
            }

            Compatibility::RequiresMigration => {
                CompatibilityStatus::RequiresMigration
            }

            Compatibility::Incompatible => {
                CompatibilityStatus::Incompatible
            }
        };

        let reason = map_compatibility_reason(
            decision.reason(),
        );

        Self {
            requested,
            current,
            status,
            reason,
        }
    }

    /// Return the requested API version.
    #[must_use]
    pub const fn requested(&self) -> ToolchainApiVersion {
        self.requested
    }

    /// Return the server's current API version.
    #[must_use]
    pub const fn current(&self) -> ToolchainApiVersion {
        self.current
    }

    /// Return the compatibility classification.
    #[must_use]
    pub const fn status(&self) -> CompatibilityStatus {
        self.status
    }

    /// Return the compatibility reason.
    #[must_use]
    pub const fn reason(&self) -> CompatibilityReason {
        self.reason
    }

    /// Return whether the requested API can be used without migration.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        matches!(
            self.status,
            CompatibilityStatus::Compatible
        )
    }

    /// Return whether migration is required.
    #[must_use]
    pub const fn requires_migration(&self) -> bool {
        matches!(
            self.status,
            CompatibilityStatus::RequiresMigration
        )
    }

    /// Return whether normal API dispatch must be rejected.
    #[must_use]
    pub const fn must_reject_dispatch(&self) -> bool {
        !self.is_compatible()
    }

    /// Validate a deserialized compatibility result.
    pub fn validate(&self) -> ApiVersioningResult<()> {
        validate_status_reason(
            self.status,
            self.reason,
        )?;

        Ok(())
    }
}

fn map_compatibility_reason(
    reason: CanonicalCompatibilityReason,
) -> CompatibilityReason {
    match reason {
        CanonicalCompatibilityReason::ExactMatch => {
            CompatibilityReason::ExactMatch
        }

        CanonicalCompatibilityReason::SupportedOlderVersion => {
            CompatibilityReason::SupportedOlderVersion
        }

        CanonicalCompatibilityReason::MigrationRequired => {
            CompatibilityReason::MigrationRequired
        }

        CanonicalCompatibilityReason::FutureVersion => {
            CompatibilityReason::FutureVersion
        }

        CanonicalCompatibilityReason::MajorVersionMismatch => {
            CompatibilityReason::MajorVersionMismatch
        }

        CanonicalCompatibilityReason::BelowMinimumSupported => {
            CompatibilityReason::BelowMinimumSupported
        }

        CanonicalCompatibilityReason::OutsideAllowedRange => {
            CompatibilityReason::OutsideAllowedRange
        }
    }
}

fn validate_status_reason(
    status: CompatibilityStatus,
    reason: CompatibilityReason,
) -> ApiVersioningResult<()> {
    let valid = match status {
        CompatibilityStatus::Compatible => {
            matches!(
                reason,
                CompatibilityReason::ExactMatch
                    | CompatibilityReason::SupportedOlderVersion
            )
        }

        CompatibilityStatus::RequiresMigration => {
            matches!(
                reason,
                CompatibilityReason::MigrationRequired
            )
        }

        CompatibilityStatus::Incompatible => {
            matches!(
                reason,
                CompatibilityReason::FutureVersion
                    | CompatibilityReason::MajorVersionMismatch
                    | CompatibilityReason::BelowMinimumSupported
                    | CompatibilityReason::OutsideAllowedRange
            )
        }

        CompatibilityStatus::PolicyUnavailable => {
            matches!(
                reason,
                CompatibilityReason::PolicyNotConfigured
            )
        }
    };

    if !valid {
        return Err(ApiVersioningError::new(
            "compatibility",
            "compatibility status and reason contradict each other",
        ));
    }

    Ok(())
}

// ============================================================================
// NEGOTIATION STATUS
// ============================================================================

/// Outcome of client/server API version negotiation.
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
pub enum ApiNegotiationStatus {
    /// A mutually supported version was selected and is compatible.
    Negotiated,

    /// A mutually supported version exists but requires migration.
    MigrationRequired,

    /// No mutually supported API version exists.
    NoCommonVersion,
}

impl ApiNegotiationStatus {
    /// Return whether API negotiation permits dispatch.
    #[must_use]
    pub const fn permits_dispatch(self) -> bool {
        matches!(self, Self::Negotiated)
    }
}

impl fmt::Display for ApiNegotiationStatus {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let value = match self {
            Self::Negotiated => "negotiated",
            Self::MigrationRequired => "migration_required",
            Self::NoCommonVersion => "no_common_version",
        };

        formatter.write_str(value)
    }
}

// ============================================================================
// NEGOTIATION RESULT
// ============================================================================

/// Complete result of client/server API version negotiation.
///
/// The result preserves both parties' effective version ranges.
///
/// A selected version does not prove that a particular operation,
/// provider, compiler, backend or runtime is available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiVersionNegotiation {
    client_supported: VersionRange<ToolchainApiVersion>,
    server_supported: VersionRange<ToolchainApiVersion>,
    server_current: ToolchainApiVersion,
    selected: Option<ToolchainApiVersion>,
    status: ApiNegotiationStatus,
    compatibility: CompatibilityStatus,
    reason: CompatibilityReason,
}

impl ApiVersionNegotiation {
    fn new(
        client_supported: VersionRange<ToolchainApiVersion>,
        server_supported: VersionRange<ToolchainApiVersion>,
        server_current: ToolchainApiVersion,
        selected: Option<ToolchainApiVersion>,
        status: ApiNegotiationStatus,
        compatibility: CompatibilityStatus,
        reason: CompatibilityReason,
    ) -> Self {
        Self {
            client_supported,
            server_supported,
            server_current,
            selected,
            status,
            compatibility,
            reason,
        }
    }

    /// Return the client's advertised supported range.
    #[must_use]
    pub const fn client_supported(
        &self,
    ) -> &VersionRange<ToolchainApiVersion> {
        &self.client_supported
    }

    /// Return the server's effective supported range.
    #[must_use]
    pub const fn server_supported(
        &self,
    ) -> &VersionRange<ToolchainApiVersion> {
        &self.server_supported
    }

    /// Return the server's current API version.
    #[must_use]
    pub const fn server_current(
        &self,
    ) -> ToolchainApiVersion {
        self.server_current
    }

    /// Return the negotiated version, if one was selected.
    #[must_use]
    pub const fn selected(
        &self,
    ) -> Option<ToolchainApiVersion> {
        self.selected
    }

    /// Return the negotiation status.
    #[must_use]
    pub const fn status(&self) -> ApiNegotiationStatus {
        self.status
    }

    /// Return the compatibility classification.
    #[must_use]
    pub const fn compatibility(&self) -> CompatibilityStatus {
        self.compatibility
    }

    /// Return the compatibility reason.
    #[must_use]
    pub const fn reason(&self) -> CompatibilityReason {
        self.reason
    }

    /// Return whether the result permits API request dispatch.
    #[must_use]
    pub const fn permits_dispatch(&self) -> bool {
        self.status.permits_dispatch()
            && matches!(
                self.compatibility,
                CompatibilityStatus::Compatible
            )
            && self.selected.is_some()
    }

    /// Validate negotiation metadata after deserialization.
    ///
    /// This verifies internal consistency. It does not replace
    /// re-negotiation against the authoritative server policy.
    pub fn validate(&self) -> ApiVersioningResult<()> {
        validate_version_range(
            "client_supported",
            &self.client_supported,
        )?;

        validate_version_range(
            "server_supported",
            &self.server_supported,
        )?;

        validate_status_reason(
            self.compatibility,
            self.reason,
        )?;

        if !self.server_supported.contains(self.server_current) {
            return Err(ApiVersioningError::new(
                "server_current",
                "server current version must belong to its effective supported range",
            ));
        }

        match (
            self.status,
            self.compatibility,
            self.selected,
        ) {
            (
                ApiNegotiationStatus::Negotiated,
                CompatibilityStatus::Compatible,
                Some(selected),
            ) => {
                if !self.client_supported.contains(selected)
                    || !self.server_supported.contains(selected)
                {
                    return Err(ApiVersioningError::new(
                        "selected",
                        "negotiated version must belong to both supported ranges",
                    ));
                }
            }

            (
                ApiNegotiationStatus::MigrationRequired,
                CompatibilityStatus::RequiresMigration,
                Some(selected),
            ) => {
                if !self.client_supported.contains(selected)
                    || !self.server_supported.contains(selected)
                {
                    return Err(ApiVersioningError::new(
                        "selected",
                        "migration-required version must belong to both supported ranges",
                    ));
                }
            }

            (
                ApiNegotiationStatus::NoCommonVersion,
                CompatibilityStatus::Incompatible,
                None,
            ) => {
                if self.reason
                    != CompatibilityReason::OutsideAllowedRange
                {
                    return Err(ApiVersioningError::new(
                        "reason",
                        "no-common-version must report an outside-allowed-range reason",
                    ));
                }
            }

            _ => {
                return Err(ApiVersioningError::new(
                    "status",
                    "negotiation status, selected version and compatibility classification contradict each other",
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// NEGOTIATOR
// ============================================================================

/// Negotiates toolchain API versions using an authoritative server policy.
///
/// The negotiator is immutable and can be shared between concurrent
/// requests when its owning service manages the policy lifecycle.
///
/// Policy changes require constructing or publishing a new negotiator
/// through the owning service's configuration mechanism.
#[derive(Debug, Clone, Copy)]
pub struct ApiVersionNegotiator {
    policy: VersionPolicy<ToolchainApiVersion>,
}

impl ApiVersionNegotiator {
    /// Construct a negotiator from an authoritative version policy.
    pub fn new(
        policy: VersionPolicy<ToolchainApiVersion>,
    ) -> ToolchainResult<Self> {
        policy.validate().map_err(|error| {
            ToolchainError::invalid_configuration(
                format!("invalid toolchain API version policy: {error}"),
            )
        })?;

        Ok(Self { policy })
    }

    /// Return the authoritative API version policy.
    #[must_use]
    pub const fn policy(
        &self,
    ) -> &VersionPolicy<ToolchainApiVersion> {
        &self.policy
    }

    /// Check one API version against the authoritative server policy.
    ///
    /// This method does not perform migration.
    pub fn check(
        &self,
        requested: ToolchainApiVersion,
    ) -> ApiVersionCheck {
        let decision = self.policy.check(requested);

        ApiVersionCheck::from_decision(
            requested,
            self.policy.current(),
            decision,
        )
    }

    /// Require a request API version to be compatible before dispatch.
    ///
    /// Migration-required, incompatible and future versions are rejected.
    ///
    /// Call this before executing the operation represented by a
    /// RequestEnvelope.
    pub fn ensure_request_compatible(
        &self,
        requested: ToolchainApiVersion,
    ) -> ToolchainResult<ApiVersionCheck> {
        let check = self.check(requested);

        if check.is_compatible() {
            return Ok(check);
        }

        let message = format!(
            "toolchain API version {} cannot be dispatched against server version {}: {} ({})",
            check.requested(),
            check.current(),
            check.status(),
            check.reason(),
        );

        Err(
            ToolchainError::new(
                ToolchainErrorKind::IncompatibleVersion,
                message,
            )
            .with_operation("toolchain.api.version_check")
            .with_context(format!(
                "requested API version: {}",
                check.requested()
            ))
            .with_context(format!(
                "current API version: {}",
                check.current()
            ))
            .with_context(format!(
                "compatibility reason: {}",
                check.reason()
            )),
        )
    }

    /// Negotiate an API version with a client.
    ///
    /// Selection rules:
    ///
    /// 1. Validate the client offer.
    /// 2. Determine the effective server-supported range.
    /// 3. Find the intersection of both ranges.
    /// 4. Respect a compatible client preference.
    /// 5. Otherwise prefer the server's current version when available.
    /// 6. Otherwise select the highest mutually supported version.
    /// 7. Explicitly report migration requirements.
    ///
    /// Future server versions are never selected.
    pub fn negotiate(
        &self,
        offer: &ApiVersionOffer,
    ) -> ToolchainResult<ApiVersionNegotiation> {
        offer.validate().map_err(|error| {
            ToolchainError::invalid_input(
                format!("invalid API version offer: {error}"),
            )
        })?;

        self.policy.validate().map_err(|error| {
            ToolchainError::invalid_configuration(
                format!("invalid API version policy: {error}"),
            )
        })?;

        let server_current = self.policy.current();

        let server_minimum = self.policy.supported().minimum();

        // A server must never advertise future versions merely because
        // its configured historical range extends beyond its current API.
        let server_maximum = server_current;

        let server_supported = VersionRange::new(
            server_minimum,
            server_maximum,
        )
        .map_err(|error| {
            ToolchainError::invalid_configuration(
                format!("invalid effective server API range: {error}"),
            )
        })?;

        let overlap_minimum = if offer.supported.minimum()
            > server_supported.minimum()
        {
            offer.supported.minimum()
        } else {
            server_supported.minimum()
        };

        let overlap_maximum = if offer.supported.maximum()
            < server_supported.maximum()
        {
            offer.supported.maximum()
        } else {
            server_supported.maximum()
        };

        if overlap_minimum > overlap_maximum {
            return Ok(ApiVersionNegotiation::new(
                offer.supported,
                server_supported,
                server_current,
                None,
                ApiNegotiationStatus::NoCommonVersion,
                CompatibilityStatus::Incompatible,
                CompatibilityReason::OutsideAllowedRange,
            ));
        }

        let overlap = VersionRange::new(
            overlap_minimum,
            overlap_maximum,
        )
        .map_err(|error| {
            ToolchainError::internal(
                format!("failed to construct API version intersection: {error}"),
            )
        })?;

        let preferred = offer.preferred.filter(|version| {
            overlap.contains(*version)
                && matches!(
                    self.policy.check(*version).compatibility(),
                    Compatibility::Compatible
                )
        });

        let selected = preferred
            .or_else(|| {
                if overlap.contains(server_current) {
                    Some(server_current)
                } else {
                    Some(overlap.maximum())
                }
            })
            .ok_or_else(|| {
                ToolchainError::internal(
                    "a non-empty API version intersection did not produce a selection",
                )
            })?;

        let decision = self.policy.check(selected);

        let check = ApiVersionCheck::from_decision(
            selected,
            server_current,
            decision,
        );

        let (status, compatibility, reason) = match check.status() {
            CompatibilityStatus::Compatible => (
                ApiNegotiationStatus::Negotiated,
                CompatibilityStatus::Compatible,
                check.reason(),
            ),

            CompatibilityStatus::RequiresMigration => (
                ApiNegotiationStatus::MigrationRequired,
                CompatibilityStatus::RequiresMigration,
                check.reason(),
            ),

            CompatibilityStatus::Incompatible => {
                return Err(
                    ToolchainError::internal(
                        "the API negotiator selected a version rejected by its own compatibility policy",
                    )
                );
            }

            CompatibilityStatus::PolicyUnavailable => {
                return Err(
                    ToolchainError::incompatible_version(
                        "toolchain API compatibility policy is unavailable",
                    )
                );
            }
        };

        let result = ApiVersionNegotiation::new(
            offer.supported,
            server_supported,
            server_current,
            Some(selected),
            status,
            compatibility,
            reason,
        );

        result.validate().map_err(|error| {
            ToolchainError::internal(
                format!("generated invalid API negotiation result: {error}"),
            )
        })?;

        Ok(result)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::version::{
        initial_toolchain_api_policy,
        INITIAL_TOOLCHAIN_API_VERSION,
    };

    fn version(
        major: u32,
        minor: u32,
        patch: u32,
    ) -> ToolchainApiVersion {
        ToolchainApiVersion::new(major, minor, patch)
    }

    fn policy(
        current: ToolchainApiVersion,
        minimum: ToolchainApiVersion,
        allow_older: bool,
    ) -> VersionPolicy<ToolchainApiVersion> {
        VersionPolicy::new(
            current,
            VersionRange::new(minimum, current)
                .expect("valid version range"),
            allow_older,
        )
        .expect("valid version policy")
    }

    #[test]
    fn exact_offer_is_valid() {
        let offer = ApiVersionOffer::exact(
            INITIAL_TOOLCHAIN_API_VERSION,
        );

        assert!(offer.validate().is_ok());
        assert!(offer.is_exact());
    }

    #[test]
    fn rejects_preference_outside_supported_range() {
        let offer = ApiVersionOffer {
            supported: VersionRange::new(
                version(1, 0, 0),
                version(1, 2, 0),
            )
            .expect("valid range"),
            preferred: Some(version(2, 0, 0)),
        };

        assert!(offer.validate().is_err());
    }

    #[test]
    fn exact_current_version_is_compatible() {
        let negotiator = ApiVersionNegotiator::new(
            initial_toolchain_api_policy(),
        )
        .expect("valid negotiator");

        let result = negotiator.ensure_request_compatible(
            INITIAL_TOOLCHAIN_API_VERSION,
        );

        assert!(result.is_ok());
        assert!(
            result
                .expect("compatible version")
                .is_compatible()
        );
    }

    #[test]
    fn future_api_version_is_rejected() {
        let negotiator = ApiVersionNegotiator::new(
            initial_toolchain_api_policy(),
        )
        .expect("valid negotiator");

        let result = negotiator.ensure_request_compatible(
            version(2, 0, 0),
        );

        assert!(result.is_err());

        assert_eq!(
            result
                .expect_err("future version must be rejected")
                .kind(),
            ToolchainErrorKind::IncompatibleVersion
        );
    }

    #[test]
    fn older_version_requires_migration_by_default() {
        let negotiator = ApiVersionNegotiator::new(
            policy(
                version(2, 0, 0),
                version(1, 0, 0),
                false,
            ),
        )
        .expect("valid negotiator");

        let check = negotiator.check(version(1, 5, 0));

        assert!(check.requires_migration());
        assert!(check.must_reject_dispatch());

        assert!(
            negotiator
                .ensure_request_compatible(version(1, 5, 0))
                .is_err()
        );
    }

    #[test]
    fn explicitly_supported_older_api_can_be_used() {
        let negotiator = ApiVersionNegotiator::new(
            policy(
                version(2, 0, 0),
                version(1, 0, 0),
                true,
            ),
        )
        .expect("valid negotiator");

        let check = negotiator.check(version(1, 5, 0));

        assert!(check.is_compatible());
        assert!(!check.must_reject_dispatch());
    }

    #[test]
    fn client_preference_is_respected_when_compatible() {
        let negotiator = ApiVersionNegotiator::new(
            policy(
                version(2, 0, 0),
                version(1, 0, 0),
                true,
            ),
        )
        .expect("valid negotiator");

        let offer = ApiVersionOffer::new(
            VersionRange::new(
                version(1, 0, 0),
                version(2, 0, 0),
            )
            .expect("valid range"),
            Some(version(1, 5, 0)),
        )
        .expect("valid offer");

        let result = negotiator
            .negotiate(&offer)
            .expect("negotiation succeeds");

        assert_eq!(
            result.selected(),
            Some(version(1, 5, 0))
        );

        assert_eq!(
            result.status(),
            ApiNegotiationStatus::Negotiated
        );

        assert!(result.permits_dispatch());
    }

    #[test]
    fn server_current_is_preferred_when_client_preference_requires_migration() {
        let negotiator = ApiVersionNegotiator::new(
            policy(
                version(2, 0, 0),
                version(1, 0, 0),
                false,
            ),
        )
        .expect("valid negotiator");

        let offer = ApiVersionOffer::new(
            VersionRange::new(
                version(1, 0, 0),
                version(2, 0, 0),
            )
            .expect("valid range"),
            Some(version(1, 5, 0)),
        )
        .expect("valid offer");

        let result = negotiator
            .negotiate(&offer)
            .expect("negotiation succeeds");

        assert_eq!(
            result.selected(),
            Some(version(2, 0, 0))
        );

        assert!(result.permits_dispatch());
    }

    #[test]
    fn no_common_version_is_reported_explicitly() {
        let negotiator = ApiVersionNegotiator::new(
            policy(
                version(2, 0, 0),
                version(2, 0, 0),
                false,
            ),
        )
        .expect("valid negotiator");

        let offer = ApiVersionOffer::exact(
            version(1, 0, 0),
        );

        let result = negotiator
            .negotiate(&offer)
            .expect("negotiation result");

        assert_eq!(
            result.status(),
            ApiNegotiationStatus::NoCommonVersion
        );

        assert_eq!(result.selected(), None);
        assert!(!result.permits_dispatch());
        assert!(result.validate().is_ok());
    }

    #[test]
    fn future_server_range_does_not_advertise_future_versions() {
        let current = version(2, 0, 0);

        let configured_policy = VersionPolicy::new(
            current,
            VersionRange::new(
                version(1, 0, 0),
                version(3, 0, 0),
            )
            .expect("valid range"),
            true,
        )
        .expect("valid policy");

        let negotiator = ApiVersionNegotiator::new(
            configured_policy,
        )
        .expect("valid negotiator");

        let offer = ApiVersionOffer::new(
            VersionRange::new(
                version(2, 0, 0),
                version(3, 0, 0),
            )
            .expect("valid range"),
            None,
        )
        .expect("valid offer");

        let result = negotiator
            .negotiate(&offer)
            .expect("negotiation succeeds");

        assert_eq!(
            result.selected(),
            Some(version(2, 0, 0))
        );
    }

    #[test]
    fn migration_required_negotiation_does_not_permit_dispatch() {
        let negotiator = ApiVersionNegotiator::new(
            policy(
                version(2, 0, 0),
                version(1, 0, 0),
                false,
            ),
        )
        .expect("valid negotiator");

        let offer = ApiVersionOffer::exact(
            version(1, 5, 0),
        );

        let result = negotiator
            .negotiate(&offer)
            .expect("negotiation result");

        assert_eq!(
            result.status(),
            ApiNegotiationStatus::MigrationRequired
        );

        assert_eq!(
            result.compatibility(),
            CompatibilityStatus::RequiresMigration
        );

        assert!(!result.permits_dispatch());
        assert!(result.validate().is_ok());
    }

    #[test]
    fn negotiation_result_survives_serde_round_trip() {
        let negotiator = ApiVersionNegotiator::new(
            initial_toolchain_api_policy(),
        )
        .expect("valid negotiator");

        let offer = ApiVersionOffer::exact(
            INITIAL_TOOLCHAIN_API_VERSION,
        );

        let result = negotiator
            .negotiate(&offer)
            .expect("negotiation succeeds");

        let encoded = serde_json::to_string(&result)
            .expect("serialization succeeds");

        let decoded: ApiVersionNegotiation =
            serde_json::from_str(&encoded)
                .expect("deserialization succeeds");

        assert_eq!(result, decoded);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn incompatible_status_and_reason_are_rejected() {
        let check = ApiVersionCheck {
            requested: version(1, 0, 0),
            current: version(1, 0, 0),
            status: CompatibilityStatus::Compatible,
            reason: CompatibilityReason::FutureVersion,
        };

        assert!(check.validate().is_err());
    }

    #[test]
    fn server_policy_is_validated_on_construction() {
        let valid = initial_toolchain_api_policy();

        assert!(
            ApiVersionNegotiator::new(valid).is_ok()
        );
    }

    #[test]
    fn version_categories_are_not_mixed() {
        let api = version(1, 0, 0);

        assert_eq!(api, INITIAL_TOOLCHAIN_API_VERSION);

        // ToolchainApiVersion is not interchangeable with
        // ManifestSchemaVersion, PackageFormatVersion or
        // any language/IR version.
    }
}
