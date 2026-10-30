
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Compatibility
//!
//! Path: src/ai/compatibility.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Evaluate whether an AI contract, model description, operation set,
//! or AI artifact can be consumed under an explicitly declared AI
//! compatibility profile.
//!
//! OWNERSHIP
//!
//! This module owns:
//! - AI compatibility descriptors;
//! - AI consumer compatibility profiles;
//! - AI feature identities;
//! - deterministic compatibility reports;
//! - AI compatibility aggregation;
//! - compatibility-specific validation.
//!
//! It does not own:
//! - version-number representation;
//! - language versioning;
//! - grammar versioning;
//! - AST versioning;
//! - semantic versioning;
//! - ZUIR schema or format versioning;
//! - dialect versioning;
//! - quantum IR versioning;
//! - backend or runtime versioning;
//! - target discovery;
//! - hardware capability negotiation;
//! - resource allocation;
//! - artifact serialization;
//! - migration execution.
//!
//! VERSION AUTHORITY
//!
//! Version identities and version policies are owned by:
//! src/ai/version.rs
//!
//! Canonical version representations and compatibility classifications
//! are owned by:
//! src/zuir/version.rs
//!
//! ZUIR dialect compatibility is owned by:
//! src/zuir/dialect/versioning.rs
//!
//! COMPATIBILITY RULE
//!
//! Version equality does not prove complete semantic compatibility.
//!
//! Compatibility must be evaluated independently for every AI
//! contract version domain.
//!
//! An unsupported future version must never be silently accepted.
//!
//! An older version requiring migration must never be interpreted
//! as a current representation before migration.
//!
//! FEATURE RULE
//!
//! Required features must be supported by the consumer.
//!
//! Unknown required features fail closed.
//!
//! Optional extensions do not automatically become required features.
//!
//! SCALABILITY
//!
//! No language-level limits are imposed on:
//! - models;
//! - parameters;
//! - operations;
//! - tensor rank;
//! - datasets;
//! - agents;
//! - pipeline stages;
//! - accelerators;
//! - execution nodes;
//! - hardware resources.
//!
//! Collection sizes are determined by available address space and
//! configured operational budgets.
//!
//! This module does not promise literal infinite computation.
//!
//! PORTABILITY
//!
//! Compatibility is independent of physical target size.
//!
//! A target's ability to execute a compatible AI program is assessed
//! separately by capability negotiation and resource feasibility.
//!
//! MIGRATION
//!
//! This module reports migration requirements.
//!
//! It does not execute migrations or silently transform artifacts.
//!
//! DETERMINISM
//!
//! Feature identities and compatibility reports use ordered collections.
//!
//! Identical descriptors and profiles produce identical reports.
//!
//! FREEZE CRITERIA
//!
//! - AI version types are reused.
//! - Canonical ZUIR compatibility classifications are reused.
//! - All five AI version domains are evaluated independently.
//! - Required features are checked deterministically.
//! - Unknown required features are rejected.
//! - Migration requirements remain explicit.
//! - Deserialized profiles can be revalidated.
//! - No target-specific assumptions are introduced.
//! - All tests pass.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::zuir::version::Compatibility;

use crate::ai::version::{
    AiApiVersion,
    AiArtifactFormatVersion,
    AiModelSchemaVersion,
    AiOperationSetVersion,
    AiSemanticVersion,
    AiVersionDecision,
    AiVersionPolicy,
    VersionIdentity,
    VersionPolicyError,
};

// ============================================================================
// FEATURE IDENTITY
// ============================================================================

/// Stable, namespaced identity of an AI feature.
///
/// Feature names are open-world identifiers.
///
/// New AI capabilities do not require extending a closed Rust enum.
///
/// Examples:
///
/// - ai.tensor.sparse
/// - ai.reasoning.probabilistic
/// - ai.learning.federated
/// - ai.differentiation.reverse
/// - ai.agent.coordination
/// - ai.quantum.hybrid
///
/// Feature identity does not prove that a particular implementation
/// can execute the feature on a particular target.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct AiFeatureId {
    namespace: String,
    name: String,
}

impl AiFeatureId {
    /// Construct a validated feature identity.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, AiCompatibilityError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_identifier(&namespace, "feature namespace")?;
        validate_identifier(&name, "feature name")?;

        Ok(Self {
            namespace,
            name,
        })
    }

    /// Return the namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the feature name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the fully qualified feature identity.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}.{}", self.namespace, self.name)
    }
}

impl fmt::Display for AiFeatureId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}.{}",
            self.namespace,
            self.name
        )
    }
}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn validate_identifier(
    value: &str,
    field: &'static str,
) -> Result<(), AiCompatibilityError> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(
            AiCompatibilityError::InvalidIdentifier {
                field,
            }
        );
    }

    Ok(())
}

// ============================================================================
// AI COMPATIBILITY DESCRIPTOR
// ============================================================================

/// Version information and required features of an AI artifact.
///
/// This descriptor contains logical compatibility information.
///
/// It does not contain physical device requirements or resource
/// allocation decisions.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct AiCompatibilityDescriptor {
    /// AI public API contract version.
    pub api_version: AiApiVersion,

    /// AI semantic contract version.
    pub semantic_version: AiSemanticVersion,

    /// Logical model schema version.
    pub model_schema_version: AiModelSchemaVersion,

    /// AI operation-set version.
    pub operation_set_version: AiOperationSetVersion,

    /// Serialized AI artifact format version.
    pub artifact_format_version: AiArtifactFormatVersion,

    /// Features necessary to preserve artifact meaning.
    #[serde(default)]
    pub required_features: BTreeSet<AiFeatureId>,
}

impl AiCompatibilityDescriptor {
    /// Construct a descriptor from its independently versioned contracts.
    #[must_use]
    pub fn new(
        api_version: AiApiVersion,
        semantic_version: AiSemanticVersion,
        model_schema_version: AiModelSchemaVersion,
        operation_set_version: AiOperationSetVersion,
        artifact_format_version: AiArtifactFormatVersion,
    ) -> Self {
        Self {
            api_version,
            semantic_version,
            model_schema_version,
            operation_set_version,
            artifact_format_version,
            required_features: BTreeSet::new(),
        }
    }

    /// Add a required feature.
    #[must_use]
    pub fn with_required_feature(
        mut self,
        feature: AiFeatureId,
    ) -> Self {
        self.required_features.insert(feature);
        self
    }

    /// Add multiple required features.
    #[must_use]
    pub fn with_required_features(
        mut self,
        features: impl IntoIterator<Item = AiFeatureId>,
    ) -> Self {
        self.required_features.extend(features);
        self
    }

    /// Validate descriptor invariants.
    ///
    /// Deserialization must not be treated as validation.
    pub fn validate(&self) -> Result<(), AiCompatibilityError> {
        for feature in &self.required_features {
            validate_identifier(
                &feature.namespace,
                "feature namespace",
            )?;

            validate_identifier(
                &feature.name,
                "feature name",
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// AI COMPATIBILITY POLICIES
// ============================================================================

/// Collection of independently validated AI version policies.
///
/// Each field has its own strongly typed version domain.
///
/// Consequently, an API version cannot accidentally be supplied
/// where an artifact-format version is required.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct AiCompatibilityPolicies {
    /// Public API compatibility.
    pub api: AiVersionPolicy<AiApiVersion>,

    /// Semantic contract compatibility.
    pub semantics: AiVersionPolicy<AiSemanticVersion>,

    /// Model schema compatibility.
    pub model_schema: AiVersionPolicy<AiModelSchemaVersion>,

    /// Operation-set compatibility.
    pub operations: AiVersionPolicy<AiOperationSetVersion>,

    /// Artifact-format compatibility.
    pub artifact_format: AiVersionPolicy<AiArtifactFormatVersion>,
}

impl AiCompatibilityPolicies {
    /// Construct a complete AI compatibility policy.
    #[must_use]
    pub fn new(
        api: AiVersionPolicy<AiApiVersion>,
        semantics: AiVersionPolicy<AiSemanticVersion>,
        model_schema: AiVersionPolicy<AiModelSchemaVersion>,
        operations: AiVersionPolicy<AiOperationSetVersion>,
        artifact_format: AiVersionPolicy<AiArtifactFormatVersion>,
    ) -> Self {
        Self {
            api,
            semantics,
            model_schema,
            operations,
            artifact_format,
        }
    }

    /// Validate every policy, including deserialized policies.
    pub fn validate(&self) -> Result<(), AiCompatibilityError> {
        self.api.validate().map_err(|_| {
            AiCompatibilityError::InvalidVersionPolicy {
                domain: "AI API",
            }
        })?;

        self.semantics.validate().map_err(|_| {
            AiCompatibilityError::InvalidVersionPolicy {
                domain: "AI semantics",
            }
        })?;

        self.model_schema.validate().map_err(|_| {
            AiCompatibilityError::InvalidVersionPolicy {
                domain: "AI model schema",
            }
        })?;

        self.operations.validate().map_err(|_| {
            AiCompatibilityError::InvalidVersionPolicy {
                domain: "AI operation set",
            }
        })?;

        self.artifact_format.validate().map_err(|_| {
            AiCompatibilityError::InvalidVersionPolicy {
                domain: "AI artifact format",
            }
        })?;

        Ok(())
    }

    /// Evaluate all five version dimensions.
    ///
    /// Each dimension is assessed independently.
    #[must_use]
    pub fn evaluate_versions(
        &self,
        descriptor: &AiCompatibilityDescriptor,
    ) -> AiVersionCompatibilityReport {
        AiVersionCompatibilityReport {
            api: self.api.check(descriptor.api_version),
            semantics: self.semantics.check(
                descriptor.semantic_version
            ),
            model_schema: self.model_schema.check(
                descriptor.model_schema_version
            ),
            operations: self.operations.check(
                descriptor.operation_set_version
            ),
            artifact_format: self.artifact_format.check(
                descriptor.artifact_format_version
            ),
        }
    }
}

// ============================================================================
// CONSUMER PROFILE
// ============================================================================

/// AI compatibility profile of a consumer.
///
/// A consumer may be a compiler component, artifact reader,
/// model loader, AI library, or another explicitly identified
/// AI subsystem participant.
///
/// The profile does not assert that the consumer has sufficient
/// physical resources to execute a model.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct AiCompatibilityProfile {
    /// Identity of the consumer's compatibility policy.
    pub identity: String,

    /// Version policies.
    pub policies: AiCompatibilityPolicies,

    /// Features understood by this consumer.
    pub supported_features: BTreeSet<AiFeatureId>,
}

impl AiCompatibilityProfile {
    /// Construct a consumer profile.
    pub fn new(
        identity: impl Into<String>,
        policies: AiCompatibilityPolicies,
    ) -> Result<Self, AiCompatibilityError> {
        let identity = identity.into();

        validate_identifier(
            &identity,
            "consumer identity",
        )?;

        policies.validate()?;

        Ok(Self {
            identity,
            policies,
            supported_features: BTreeSet::new(),
        })
    }

    /// Add a supported feature.
    #[must_use]
    pub fn with_supported_feature(
        mut self,
        feature: AiFeatureId,
    ) -> Self {
        self.supported_features.insert(feature);
        self
    }

    /// Add multiple supported features.
    #[must_use]
    pub fn with_supported_features(
        mut self,
        features: impl IntoIterator<Item = AiFeatureId>,
    ) -> Self {
        self.supported_features.extend(features);
        self
    }

    /// Validate the profile.
    ///
    /// This method must be called after deserialization.
    pub fn validate(&self) -> Result<(), AiCompatibilityError> {
        validate_identifier(
            &self.identity,
            "consumer identity",
        )?;

        self.policies.validate()?;

        for feature in &self.supported_features {
            validate_identifier(
                &feature.namespace,
                "feature namespace",
            )?;

            validate_identifier(
                &feature.name,
                "feature name",
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// VERSION COMPATIBILITY REPORT
// ============================================================================

/// Compatibility results for all independent AI version domains.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct AiVersionCompatibilityReport {
    pub api: AiVersionDecision<AiApiVersion>,
    pub semantics: AiVersionDecision<AiSemanticVersion>,
    pub model_schema: AiVersionDecision<AiModelSchemaVersion>,
    pub operations: AiVersionDecision<AiOperationSetVersion>,
    pub artifact_format: AiVersionDecision<AiArtifactFormatVersion>,
}

impl AiVersionCompatibilityReport {
    /// Return true when every version domain is compatible
    /// without migration.
    #[must_use]
    pub const fn is_compatible_without_migration(&self) -> bool {
        self.api.is_compatible_without_migration()
            && self.semantics.is_compatible_without_migration()
            && self.model_schema.is_compatible_without_migration()
            && self.operations.is_compatible_without_migration()
            && self.artifact_format.is_compatible_without_migration()
    }

    /// Return true if at least one version requires migration.
    #[must_use]
    pub const fn requires_migration(&self) -> bool {
        self.api.requires_migration()
            || self.semantics.requires_migration()
            || self.model_schema.requires_migration()
            || self.operations.requires_migration()
            || self.artifact_format.requires_migration()
    }

    /// Return true if at least one version is incompatible.
    #[must_use]
    pub const fn is_incompatible(&self) -> bool {
        self.api.must_reject()
            || self.semantics.must_reject()
            || self.model_schema.must_reject()
            || self.operations.must_reject()
            || self.artifact_format.must_reject()
    }

    /// Aggregate the version decisions.
    ///
    /// Incompatibility has precedence over migration requirements.
    #[must_use]
    pub const fn classification(&self) -> Compatibility {
        if self.is_incompatible() {
            Compatibility::Incompatible
        } else if self.requires_migration() {
            Compatibility::RequiresMigration
        } else {
            Compatibility::Compatible
        }
    }
}

// ============================================================================
// FEATURE COMPATIBILITY
// ============================================================================

/// Features required by an artifact but unavailable in a consumer.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct AiFeatureCompatibilityReport {
    /// Required features supported by the consumer.
    pub supported: BTreeSet<AiFeatureId>,

    /// Required features missing from the consumer.
    pub unsupported: BTreeSet<AiFeatureId>,
}

impl AiFeatureCompatibilityReport {
    /// Whether every required feature is supported.
    #[must_use]
    pub fn is_compatible(&self) -> bool {
        self.unsupported.is_empty()
    }
}

// ============================================================================
// OVERALL COMPATIBILITY REPORT
// ============================================================================

/// Complete compatibility assessment.
///
/// Version compatibility and feature compatibility remain visible
/// independently, so callers can distinguish their failure causes.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct AiCompatibilityReport {
    /// Consumer identity.
    pub consumer_identity: String,

    /// Version decisions.
    pub versions: AiVersionCompatibilityReport,

    /// Feature decisions.
    pub features: AiFeatureCompatibilityReport,
}

impl AiCompatibilityReport {
    /// Overall compatibility classification.
    ///
    /// An unsupported required feature is incompatible.
    #[must_use]
    pub const fn classification(&self) -> Compatibility {
        if self.versions.is_incompatible()
            || !self.features.unsupported.is_empty()
        {
            Compatibility::Incompatible
        } else if self.versions.requires_migration() {
            Compatibility::RequiresMigration
        } else {
            Compatibility::Compatible
        }
    }

    /// Whether the artifact can be consumed without migration.
    #[must_use]
    pub const fn is_compatible_without_migration(&self) -> bool {
        matches!(
            self.classification(),
            Compatibility::Compatible
        )
    }

    /// Whether explicit migration is required.
    #[must_use]
    pub const fn requires_migration(&self) -> bool {
        matches!(
            self.classification(),
            Compatibility::RequiresMigration
        )
    }

    /// Whether the artifact must be rejected.
    #[must_use]
    pub const fn must_reject(&self) -> bool {
        matches!(
            self.classification(),
            Compatibility::Incompatible
        )
    }
}

// ============================================================================
// COMPATIBILITY EVALUATION
// ============================================================================

/// Evaluate an AI artifact against a consumer profile.
///
/// This is a pure operation:
///
/// - no filesystem access;
/// - no network access;
/// - no hardware discovery;
/// - no global mutable state;
/// - no migration execution.
///
/// The result is deterministic for identical inputs.
pub fn evaluate_compatibility(
    descriptor: &AiCompatibilityDescriptor,
    consumer: &AiCompatibilityProfile,
) -> Result<AiCompatibilityReport, AiCompatibilityError> {
    descriptor.validate()?;
    consumer.validate()?;

    let versions = consumer
        .policies
        .evaluate_versions(descriptor);

    let supported: BTreeSet<_> = descriptor
        .required_features
        .intersection(&consumer.supported_features)
        .cloned()
        .collect();

    let unsupported: BTreeSet<_> = descriptor
        .required_features
        .difference(&consumer.supported_features)
        .cloned()
        .collect();

    Ok(AiCompatibilityReport {
        consumer_identity: consumer.identity.clone(),
        versions,
        features: AiFeatureCompatibilityReport {
            supported,
            unsupported,
        },
    })
}

// ============================================================================
// VERSION POLICY ERROR MAPPING
// ============================================================================

impl From<VersionPolicyError> for AiCompatibilityError {
    fn from(_: VersionPolicyError) -> Self {
        Self::InvalidVersionPolicy {
            domain: "AI compatibility",
        }
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors encountered while constructing or evaluating AI
/// compatibility contracts.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    thiserror::Error,
)]
pub enum AiCompatibilityError {
    /// Invalid identity.
    #[error("invalid {field}")]
    InvalidIdentifier {
        field: &'static str,
    },

    /// Invalid version policy.
    #[error("invalid compatibility policy for {domain}")]
    InvalidVersionPolicy {
        domain: &'static str,
    },
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::version::{
        exact_ai_policy,
        VersionRange,
    };

    fn policies() -> AiCompatibilityPolicies {
        AiCompatibilityPolicies::new(
            exact_ai_policy(AiApiVersion::new(1, 0, 0)),
            exact_ai_policy(AiSemanticVersion::new(1, 0, 0)),
            exact_ai_policy(AiModelSchemaVersion::new(1, 0, 0)),
            exact_ai_policy(AiOperationSetVersion::new(1, 0, 0)),
            exact_ai_policy(AiArtifactFormatVersion::new(1, 0, 0)),
        )
    }

    fn descriptor() -> AiCompatibilityDescriptor {
        AiCompatibilityDescriptor::new(
            AiApiVersion::new(1, 0, 0),
            AiSemanticVersion::new(1, 0, 0),
            AiModelSchemaVersion::new(1, 0, 0),
            AiOperationSetVersion::new(1, 0, 0),
            AiArtifactFormatVersion::new(1, 0, 0),
        )
    }

    fn consumer() -> AiCompatibilityProfile {
        AiCompatibilityProfile::new(
            "zamani-ai-consumer",
            policies(),
        )
        .expect("valid consumer")
    }

    #[test]
    fn feature_identity_is_namespaced() {
        let feature = AiFeatureId::new(
            "ai.tensor",
            "sparse",
        )
        .expect("valid feature");

        assert_eq!(
            feature.qualified_name(),
            "ai.tensor.sparse"
        );
    }

    #[test]
    fn empty_feature_identity_is_rejected() {
        assert!(
            AiFeatureId::new("", "sparse").is_err()
        );

        assert!(
            AiFeatureId::new("ai.tensor", "").is_err()
        );
    }

    #[test]
    fn whitespace_and_control_characters_are_rejected() {
        assert!(
            AiFeatureId::new(
                " ai.tensor",
                "sparse"
            ).is_err()
        );

        assert!(
            AiFeatureId::new(
                "ai.tensor",
                "sparse\n"
            ).is_err()
        );
    }

    #[test]
    fn exact_versions_are_compatible() {
        let report = evaluate_compatibility(
            &descriptor(),
            &consumer(),
        )
        .expect("evaluation succeeds");

        assert_eq!(
            report.classification(),
            Compatibility::Compatible
        );

        assert!(
            report.is_compatible_without_migration()
        );
    }

    #[test]
    fn unsupported_required_features_are_rejected() {
        let feature = AiFeatureId::new(
            "ai.learning",
            "federated",
        )
        .expect("valid feature");

        let descriptor = descriptor()
            .with_required_feature(feature.clone());

        let report = evaluate_compatibility(
            &descriptor,
            &consumer(),
        )
        .expect("evaluation succeeds");

        assert_eq!(
            report.classification(),
            Compatibility::Incompatible
        );

        assert!(
            report.features.unsupported.contains(&feature)
        );
    }

    #[test]
    fn supported_required_features_are_accepted() {
        let feature = AiFeatureId::new(
            "ai.reasoning",
            "probabilistic",
        )
        .expect("valid feature");

        let descriptor = descriptor()
            .with_required_feature(feature.clone());

        let consumer = consumer()
            .with_supported_feature(feature);

        let report = evaluate_compatibility(
            &descriptor,
            &consumer,
        )
        .expect("evaluation succeeds");

        assert_eq!(
            report.classification(),
            Compatibility::Compatible
        );

        assert!(report.features.is_compatible());
    }

    #[test]
    fn future_artifact_format_is_rejected() {
        let mut descriptor = descriptor();

        descriptor.artifact_format_version =
            AiArtifactFormatVersion::new(2, 0, 0);

        let report = evaluate_compatibility(
            &descriptor,
            &consumer(),
        )
        .expect("evaluation succeeds");

        assert_eq!(
            report.classification(),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn historical_versions_can_require_migration() {
        let current = AiModelSchemaVersion::new(2, 0, 0);

        let range = VersionRange::new(
            AiModelSchemaVersion::new(1, 0, 0),
            current,
        )
        .expect("valid range");

        let model_policy = AiVersionPolicy::new(
            current,
            range,
            false,
        )
        .expect("valid policy");

        let mut policies = policies();
        policies.model_schema = model_policy;

        let mut descriptor = descriptor();

        descriptor.model_schema_version =
            AiModelSchemaVersion::new(1, 5, 0);

        let consumer = AiCompatibilityProfile::new(
            "historical-reader",
            policies,
        )
        .expect("valid consumer");

        let report = evaluate_compatibility(
            &descriptor,
            &consumer,
        )
        .expect("evaluation succeeds");

        assert_eq!(
            report.classification(),
            Compatibility::RequiresMigration
        );
    }

    #[test]
    fn incompatible_version_takes_precedence_over_migration() {
        let current = AiModelSchemaVersion::new(2, 0, 0);

        let range = VersionRange::new(
            AiModelSchemaVersion::new(1, 0, 0),
            current,
        )
        .expect("valid range");

        let model_policy = AiVersionPolicy::new(
            current,
            range,
            false,
        )
        .expect("valid policy");

        let mut policies = policies();
        policies.model_schema = model_policy;

        let mut descriptor = descriptor();

        descriptor.model_schema_version =
            AiModelSchemaVersion::new(1, 5, 0);

        descriptor.artifact_format_version =
            AiArtifactFormatVersion::new(2, 0, 0);

        let consumer = AiCompatibilityProfile::new(
            "mixed-version-reader",
            policies,
        )
        .expect("valid consumer");

        let report = evaluate_compatibility(
            &descriptor,
            &consumer,
        )
        .expect("evaluation succeeds");

        assert_eq!(
            report.classification(),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn reports_are_deterministic() {
        let feature_a = AiFeatureId::new(
            "ai.tensor",
            "sparse",
        ).expect("valid feature");

        let feature_b = AiFeatureId::new(
            "ai.reasoning",
            "probabilistic",
        ).expect("valid feature");

        let descriptor = descriptor()
            .with_required_features([
                feature_a.clone(),
                feature_b.clone(),
            ]);

        let consumer = consumer()
            .with_supported_feature(feature_b);

        let first = evaluate_compatibility(
            &descriptor,
            &consumer,
        ).expect("evaluation succeeds");

        let second = evaluate_compatibility(
            &descriptor,
            &consumer,
        ).expect("evaluation succeeds");

        assert_eq!(first, second);

        assert!(
            first.features.unsupported.contains(&feature_a)
        );
    }

    #[test]
    fn descriptor_serialization_round_trip() {
        let original = descriptor();

        let encoded = serde_json::to_string(&original)
            .expect("serialization succeeds");

        let decoded: AiCompatibilityDescriptor =
            serde_json::from_str(&encoded)
                .expect("deserialization succeeds");

        assert_eq!(original, decoded);

        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn consumer_profile_serialization_round_trip() {
        let original = consumer();

        let encoded = serde_json::to_string(&original)
            .expect("serialization succeeds");

        let decoded: AiCompatibilityProfile =
            serde_json::from_str(&encoded)
                .expect("deserialization succeeds");

        assert_eq!(original, decoded);

        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn empty_required_feature_set_is_valid() {
        let report = evaluate_compatibility(
            &descriptor(),
            &consumer(),
        )
        .expect("evaluation succeeds");

        assert!(report.features.is_compatible());
    }
}
