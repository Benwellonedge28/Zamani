
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain API — Capability Discovery
//!
//! Path: `src/toolchain/api/capabilities.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the versioned capability-discovery contract exposed by the
//! Zamani Toolchain API.
//!
//! API clients use this contract to discover:
//!
//! - Which toolchain operations are advertised.
//! - Which API features are available.
//! - Which features are experimental, unavailable, or conditional.
//! - Which features require additional configuration.
//! - Which optional extensions are advertised.
//!
//! ## Ownership boundaries
//!
//! This module owns API-level capability descriptions only.
//!
//! It does not own:
//!
//! - Source-language semantic capabilities.
//! - Semantic requirements, preferences, constraints, or hints.
//! - Computational-domain capability analysis.
//! - CPU, GPU, FPGA, QPU, accelerator, or device discovery.
//! - Physical resource availability or allocation.
//! - Backend registration or backend feasibility.
//! - Runtime scheduling or execution.
//! - Deployment-provider discovery.
//!
//! Those responsibilities remain with their existing authoritative
//! repository subsystems.
//!
//! In particular, an API feature being advertised does not prove that
//! a particular program can be compiled or executed on a particular
//! physical target.
//!
//! ## POCO-REAF
//!
//! The API catalogue is open-ended. Third-party integrations and future
//! toolchain versions can advertise new operation and feature identifiers
//! without changing a closed enum of computing domains.
//!
//! The catalogue does not impose a maximum number of operations, features,
//! extensions, providers, targets, devices, or computational resources.
//!
//! Collections are limited only by available memory and the representational
//! limits of the executing process.
//!
//! ## Validation
//!
//! Deserialization is not validation. Data received from an untrusted
//! transport must pass `ToolchainApiCapabilities::validate()` before use.
//!
//! Validation here is structural. Authorization, provider trust, target
//! compatibility, semantic feasibility, and physical resource availability
//! must be established by their respective authorities.
//!
//! ## Integration
//!
//! This module consumes:
//!
//! - `crate::toolchain::version::ToolchainApiVersion`
//! - `crate::toolchain::api::request` for operation naming conventions
//! - `crate::toolchain::api::response` for the shared API transport boundary
//!
//! The API module should expose this contract through `api/mod.rs`.
//!
//! The toolchain root should expose the API through `toolchain/mod.rs`.
//!
//! ## Security
//!
//! Capability descriptions are untrusted metadata when received remotely.
//! They must not contain credentials, private keys, access tokens, or
//! other secrets.
//!
//! Extension values are descriptive data, not executable instructions.
//! Consumers must not execute extension values or treat them as trusted
//! configuration without separate validation and authorization.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::toolchain::version::ToolchainApiVersion;

// ============================================================================
// VALIDATION
// ============================================================================

/// Structural validation failure for an API capability catalogue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityValidationError {
    field: String,
    message: String,
}

impl CapabilityValidationError {
    fn new(
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Return the field associated with the validation failure.
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

impl fmt::Display for CapabilityValidationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "invalid API capability field '{}': {}",
            self.field,
            self.message,
        )
    }
}

impl std::error::Error for CapabilityValidationError {}

/// Result of API capability validation.
pub type CapabilityValidationResult<T> =
    std::result::Result<T, CapabilityValidationError>;

/// Validate an open-ended API identifier.
///
/// Identifiers are deliberately not restricted to a built-in list.
/// Third-party clients and future API extensions may introduce new
/// identifiers without requiring changes to this module.
///
/// Surrounding whitespace, empty values, and control characters are
/// rejected. No arbitrary identifier-length ceiling is imposed.
fn validate_identifier(
    field: &str,
    value: &str,
) -> CapabilityValidationResult<()> {
    if value.is_empty() || value.trim().is_empty() {
        return Err(CapabilityValidationError::new(
            field,
            "identifier must not be empty",
        ));
    }

    if value != value.trim() {
        return Err(CapabilityValidationError::new(
            field,
            "identifier must not contain surrounding whitespace",
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(CapabilityValidationError::new(
            field,
            "identifier must not contain control characters",
        ));
    }

    Ok(())
}

/// Validate descriptive text.
///
/// Empty descriptions are permitted when the API producer intentionally
/// has no description. Control characters are rejected to prevent
/// accidental terminal and log-control injection.
fn validate_description(
    field: &str,
    value: &str,
) -> CapabilityValidationResult<()> {
    if value.chars().any(char::is_control) {
        return Err(CapabilityValidationError::new(
            field,
            "description must not contain control characters",
        ));
    }

    Ok(())
}

// ============================================================================
// OPEN IDENTIFIERS
// ============================================================================

/// Define a strongly typed, open-ended API identifier.
macro_rules! define_api_identifier {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
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
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Construct and validate an identifier.
            pub fn new(
                value: impl Into<String>,
            ) -> CapabilityValidationResult<Self> {
                let identifier = Self(value.into());

                identifier.validate()?;

                Ok(identifier)
            }

            /// Return the identifier as a string slice.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Validate the identifier.
            pub fn validate(
                &self,
            ) -> CapabilityValidationResult<()> {
                validate_identifier(
                    stringify!($name),
                    &self.0,
                )
            }
        }

        impl fmt::Display for $name {
            fn fmt(
                &self,
                formatter: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

define_api_identifier!(
    /// Identifies a toolchain API operation.
    ///
    /// Examples include `build`, `workspace.discover`,
    /// `dependency.resolve`, and third-party operation identifiers.
    ///
    /// The identifier describes an API operation. It is not a compiler
    /// pass, backend identity, runtime task identity, or hardware target.
    ApiOperationId
);

define_api_identifier!(
    /// Identifies an advertised toolchain API feature.
    ///
    /// Examples include `build.incremental`, `package.signing`,
    /// `registry.federation`, and third-party feature identifiers.
    ///
    /// Feature identifiers describe API-level functionality. They must
    /// not be used as substitutes for semantic or physical capabilities.
    ApiFeatureId
);

// ============================================================================
// ADVERTISEMENT STATUS
// ============================================================================

/// Describes the availability state of an API operation or API feature.
///
/// This status is an advertisement by the toolchain API implementation.
/// It is not a statement about whether a particular program, target,
/// provider, or physical device is feasible.
///
/// Unknown states are represented explicitly so that clients can fail
/// closed when they encounter a future status value.
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
pub enum ApiCapabilityStatus {
    /// The feature or operation is available under the current API
    /// implementation and its normal authorization rules.
    Available,

    /// The feature or operation is available but its contract may evolve.
    Experimental,

    /// The feature or operation is not available.
    Unavailable,

    /// The feature or operation is available only after additional
    /// configuration or prerequisites have been satisfied.
    RequiresConfiguration,

    /// The producer cannot establish the availability state.
    ///
    /// Consumers must not infer support from this value.
    Unknown,
}

impl ApiCapabilityStatus {
    /// Return whether the producer advertises this capability as usable
    /// without additional configuration.
    ///
    /// Experimental functionality is considered invocable only when the
    /// client explicitly accepts experimental features through its own
    /// policy. This method therefore returns `false` for Experimental.
    #[must_use]
    pub const fn is_available(&self) -> bool {
        matches!(self, Self::Available)
    }

    /// Return whether the status is explicitly unavailable.
    #[must_use]
    pub const fn is_unavailable(&self) -> bool {
        matches!(self, Self::Unavailable)
    }

    /// Return whether additional configuration is required.
    #[must_use]
    pub const fn requires_configuration(&self) -> bool {
        matches!(self, Self::RequiresConfiguration)
    }

    /// Return whether the status is known.
    #[must_use]
    pub const fn is_known(&self) -> bool {
        !matches!(self, Self::Unknown)
    }
}

// ============================================================================
// OPERATION DESCRIPTORS
// ============================================================================

/// Describes one operation exposed by the toolchain API.
///
/// This is metadata only. It does not execute the operation, grant
/// permission to execute it, or prove that its downstream providers
/// are available.
///
/// `required_features` references feature identifiers advertised by
/// the same capability catalogue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiOperationDescriptor {
    /// Stable, open-ended operation identifier.
    pub id: ApiOperationId,

    /// Current advertisement status.
    pub status: ApiCapabilityStatus,

    /// Human-readable explanation of the operation.
    #[serde(default)]
    pub description: String,

    /// API features required by this operation.
    ///
    /// These are API-level prerequisites, not program semantic
    /// requirements or physical hardware requirements.
    #[serde(default)]
    pub required_features: BTreeSet<ApiFeatureId>,
}

impl ApiOperationDescriptor {
    /// Construct an operation descriptor.
    pub fn new(
        id: ApiOperationId,
        status: ApiCapabilityStatus,
    ) -> Self {
        Self {
            id,
            status,
            description: String::new(),
            required_features: BTreeSet::new(),
        }
    }

    /// Set the human-readable description.
    #[must_use]
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = description.into();
        self
    }

    /// Declare an API feature required by this operation.
    #[must_use]
    pub fn requiring_feature(
        mut self,
        feature: ApiFeatureId,
    ) -> Self {
        self.required_features.insert(feature);
        self
    }

    /// Validate the descriptor independently of its catalogue.
    pub fn validate(
        &self,
    ) -> CapabilityValidationResult<()> {
        self.id.validate()?;

        validate_description(
            "operation.description",
            &self.description,
        )?;

        for feature in &self.required_features {
            feature.validate()?;
        }

        Ok(())
    }
}

// ============================================================================
// FEATURE DESCRIPTORS
// ============================================================================

/// Describes one feature exposed by the toolchain API.
///
/// The optional `owner` is descriptive metadata identifying the service
/// or provider responsible for the feature. It does not register that
/// provider or establish its trustworthiness.
///
/// A feature descriptor must not claim physical target support. For
/// example, advertising `quantum.build` means only that the corresponding
/// API feature is advertised; it does not prove that a QPU or simulator
/// is installed or available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiFeatureDescriptor {
    /// Stable, open-ended feature identifier.
    pub id: ApiFeatureId,

    /// Current advertisement status.
    pub status: ApiCapabilityStatus,

    /// Human-readable explanation.
    #[serde(default)]
    pub description: String,

    /// Optional descriptive owner, subsystem, or provider identifier.
    ///
    /// This is not an authenticated provider identity.
    pub owner: Option<String>,
}

impl ApiFeatureDescriptor {
    /// Construct a feature descriptor.
    pub fn new(
        id: ApiFeatureId,
        status: ApiCapabilityStatus,
    ) -> Self {
        Self {
            id,
            status,
            description: String::new(),
            owner: None,
        }
    }

    /// Set the human-readable description.
    #[must_use]
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = description.into();
        self
    }

    /// Set the descriptive owner.
    ///
    /// The value is metadata only. Provider registration and trust
    /// verification remain with the canonical provider registry.
    #[must_use]
    pub fn with_owner(
        mut self,
        owner: impl Into<String>,
    ) -> Self {
        self.owner = Some(owner.into());
        self
    }

    /// Validate the feature descriptor independently of its catalogue.
    pub fn validate(
        &self,
    ) -> CapabilityValidationResult<()> {
        self.id.validate()?;

        validate_description(
            "feature.description",
            &self.description,
        )?;

        if let Some(owner) = &self.owner {
            validate_identifier("feature.owner", owner)?;
        }

        Ok(())
    }
}

// ============================================================================
// API CAPABILITY CATALOGUE
// ============================================================================

/// Versioned description of the capabilities exposed by one toolchain API
/// implementation.
///
/// This is the API-level discovery document returned to clients.
///
/// It deliberately does not contain a closed list of computing domains,
/// architectures, hardware types, providers, or physical resources.
///
/// Operations and features are indexed by stable identifiers in ordered
/// maps. This gives deterministic serialization and lookup without
/// requiring a fixed-size catalogue.
///
/// Unknown extension data is carried in the explicit `extensions` map.
/// Extension keys must be namespaced to avoid collisions.
///
/// Deserialized catalogues must pass `validate()` before being trusted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolchainApiCapabilities {
    /// Toolchain API version described by this catalogue.
    pub api_version: ToolchainApiVersion,

    /// Advertised API operations, keyed by their canonical identifiers.
    #[serde(default)]
    pub operations: BTreeMap<ApiOperationId, ApiOperationDescriptor>,

    /// Advertised API features, keyed by their canonical identifiers.
    #[serde(default)]
    pub features: BTreeMap<ApiFeatureId, ApiFeatureDescriptor>,

    /// Namespaced, non-executable extension metadata.
    ///
    /// Extension values must not contain credentials or secrets.
    #[serde(default)]
    pub extensions: BTreeMap<String, Value>,
}

impl ToolchainApiCapabilities {
    /// Construct an empty capability catalogue for an API version.
    ///
    /// An empty catalogue makes no operation or feature availability
    /// claims. It is not equivalent to a catalogue in which every
    /// possible operation is supported.
    #[must_use]
    pub fn new(api_version: ToolchainApiVersion) -> Self {
        Self {
            api_version,
            operations: BTreeMap::new(),
            features: BTreeMap::new(),
            extensions: BTreeMap::new(),
        }
    }

    /// Return the API version described by this catalogue.
    #[must_use]
    pub fn api_version(&self) -> &ToolchainApiVersion {
        &self.api_version
    }

    /// Advertise an operation.
    ///
    /// Duplicate operation identifiers are rejected. The catalogue is
    /// unchanged when insertion fails.
    pub fn register_operation(
        &mut self,
        descriptor: ApiOperationDescriptor,
    ) -> CapabilityValidationResult<()> {
        descriptor.validate()?;

        if self.operations.contains_key(&descriptor.id) {
            return Err(CapabilityValidationError::new(
                "operations",
                format!(
                    "operation '{}' is already registered",
                    descriptor.id,
                ),
            ));
        }

        self.operations
            .insert(descriptor.id.clone(), descriptor);

        Ok(())
    }

    /// Advertise a feature.
    ///
    /// Duplicate feature identifiers are rejected. The catalogue is
    /// unchanged when insertion fails.
    pub fn register_feature(
        &mut self,
        descriptor: ApiFeatureDescriptor,
    ) -> CapabilityValidationResult<()> {
        descriptor.validate()?;

        if self.features.contains_key(&descriptor.id) {
            return Err(CapabilityValidationError::new(
                "features",
                format!(
                    "feature '{}' is already registered",
                    descriptor.id,
                ),
            ));
        }

        self.features
            .insert(descriptor.id.clone(), descriptor);

        Ok(())
    }

    /// Add or replace namespaced extension metadata.
    ///
    /// Extension data is descriptive only and must not be interpreted
    /// as executable code or trusted configuration.
    pub fn set_extension(
        &mut self,
        key: impl Into<String>,
        value: Value,
    ) -> CapabilityValidationResult<Option<Value>> {
        let key = key.into();

        validate_identifier("extensions.key", &key)?;

        if !key.contains('.') {
            return Err(CapabilityValidationError::new(
                "extensions.key",
                "extension keys must be namespace-qualified",
            ));
        }

        Ok(self.extensions.insert(key, value))
    }

    /// Return an operation descriptor by identifier.
    #[must_use]
    pub fn operation(
        &self,
        id: &ApiOperationId,
    ) -> Option<&ApiOperationDescriptor> {
        self.operations.get(id)
    }

    /// Return a feature descriptor by identifier.
    #[must_use]
    pub fn feature(
        &self,
        id: &ApiFeatureId,
    ) -> Option<&ApiFeatureDescriptor> {
        self.features.get(id)
    }

    /// Return whether an operation is explicitly advertised as available.
    ///
    /// This does not check authorization, configuration, provider health,
    /// target feasibility, or runtime resource availability.
    #[must_use]
    pub fn is_operation_available(
        &self,
        id: &ApiOperationId,
    ) -> bool {
        self.operation(id)
            .is_some_and(|descriptor| descriptor.status.is_available())
    }

    /// Return whether a feature is explicitly advertised as available.
    ///
    /// This does not prove that a specific operation can be completed.
    #[must_use]
    pub fn is_feature_available(
        &self,
        id: &ApiFeatureId,
    ) -> bool {
        self.feature(id)
            .is_some_and(|descriptor| descriptor.status.is_available())
    }

    /// Return operation descriptors in deterministic identifier order.
    pub fn operations(
        &self,
    ) -> impl Iterator<Item = &ApiOperationDescriptor> {
        self.operations.values()
    }

    /// Return feature descriptors in deterministic identifier order.
    pub fn features(
        &self,
    ) -> impl Iterator<Item = &ApiFeatureDescriptor> {
        self.features.values()
    }

    /// Validate the complete capability catalogue.
    ///
    /// This method must be called after deserialization and before
    /// accepting a catalogue from an untrusted transport.
    ///
    /// Every operation's required features must be present in the
    /// catalogue. A required feature need not be available: the operation
    /// may be advertised as conditional or unavailable. Consumers must
    /// inspect the status of both the operation and its prerequisites.
    pub fn validate(
        &self,
    ) -> CapabilityValidationResult<()> {
        for (key, descriptor) in &self.operations {
            key.validate()?;
            descriptor.validate()?;

            if key != &descriptor.id {
                return Err(CapabilityValidationError::new(
                    "operations",
                    format!(
                        "operation map key '{}' does not match descriptor identifier '{}'",
                        key,
                        descriptor.id,
                    ),
                ));
            }

            for required in &descriptor.required_features {
                if !self.features.contains_key(required) {
                    return Err(CapabilityValidationError::new(
                        "operations.required_features",
                        format!(
                            "operation '{}' references undeclared feature '{}'",
                            descriptor.id,
                            required,
                        ),
                    ));
                }
            }
        }

        for (key, descriptor) in &self.features {
            key.validate()?;
            descriptor.validate()?;

            if key != &descriptor.id {
                return Err(CapabilityValidationError::new(
                    "features",
                    format!(
                        "feature map key '{}' does not match descriptor identifier '{}'",
                        key,
                        descriptor.id,
                    ),
                ));
            }
        }

        for key in self.extensions.keys() {
            validate_identifier("extensions.key", key)?;

            if !key.contains('.') {
                return Err(CapabilityValidationError::new(
                    "extensions.key",
                    format!(
                        "extension key '{}' is not namespace-qualified",
                        key,
                    ),
                ));
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

    use crate::toolchain::version::INITIAL_TOOLCHAIN_API_VERSION;

    fn api_version() -> ToolchainApiVersion {
        INITIAL_TOOLCHAIN_API_VERSION.clone()
    }

    fn operation_id(value: &str) -> ApiOperationId {
        ApiOperationId::new(value).expect("valid operation identifier")
    }

    fn feature_id(value: &str) -> ApiFeatureId {
        ApiFeatureId::new(value).expect("valid feature identifier")
    }

    #[test]
    fn empty_catalogue_is_valid_and_makes_no_support_claims() {
        let catalogue = ToolchainApiCapabilities::new(api_version());

        assert!(catalogue.validate().is_ok());
        assert!(catalogue.operations.is_empty());
        assert!(catalogue.features.is_empty());
    }

    #[test]
    fn operation_can_be_advertised() {
        let mut catalogue =
            ToolchainApiCapabilities::new(api_version());

        let operation = ApiOperationDescriptor::new(
            operation_id("build"),
            ApiCapabilityStatus::Available,
        )
        .with_description("Build a Zamani project");

        catalogue
            .register_operation(operation)
            .expect("operation registration");

        assert!(
            catalogue.is_operation_available(&operation_id("build"))
        );

        assert!(catalogue.validate().is_ok());
    }

    #[test]
    fn feature_can_be_advertised() {
        let mut catalogue =
            ToolchainApiCapabilities::new(api_version());

        let feature = ApiFeatureDescriptor::new(
            feature_id("build.incremental"),
            ApiCapabilityStatus::Available,
        )
        .with_description("Incremental project builds");

        catalogue
            .register_feature(feature)
            .expect("feature registration");

        assert!(
            catalogue.is_feature_available(
                &feature_id("build.incremental"),
            )
        );

        assert!(catalogue.validate().is_ok());
    }

    #[test]
    fn operation_prerequisites_must_be_declared() {
        let mut catalogue =
            ToolchainApiCapabilities::new(api_version());

        let operation = ApiOperationDescriptor::new(
            operation_id("package.publish"),
            ApiCapabilityStatus::Available,
        )
        .requiring_feature(feature_id("package.signing"));

        catalogue
            .register_operation(operation)
            .expect("operation registration itself is valid");

        assert!(catalogue.validate().is_err());
    }

    #[test]
    fn declared_operation_prerequisites_validate() {
        let mut catalogue =
            ToolchainApiCapabilities::new(api_version());

        catalogue
            .register_feature(
                ApiFeatureDescriptor::new(
                    feature_id("package.signing"),
                    ApiCapabilityStatus::RequiresConfiguration,
                )
                .with_description("Package signing support"),
            )
            .expect("feature registration");

        catalogue
            .register_operation(
                ApiOperationDescriptor::new(
                    operation_id("package.publish"),
                    ApiCapabilityStatus::RequiresConfiguration,
                )
                .requiring_feature(feature_id("package.signing")),
            )
            .expect("operation registration");

        assert!(catalogue.validate().is_ok());
    }

    #[test]
    fn duplicate_operation_registration_is_rejected_without_replacement() {
        let mut catalogue =
            ToolchainApiCapabilities::new(api_version());

        catalogue
            .register_operation(
                ApiOperationDescriptor::new(
                    operation_id("build"),
                    ApiCapabilityStatus::Available,
                ),
            )
            .expect("first registration");

        let duplicate = catalogue.register_operation(
            ApiOperationDescriptor::new(
                operation_id("build"),
                ApiCapabilityStatus::Unavailable,
            ),
        );

        assert!(duplicate.is_err());

        assert_eq!(
            catalogue
                .operation(&operation_id("build"))
                .expect("original operation")
                .status,
            ApiCapabilityStatus::Available,
        );
    }

    #[test]
    fn duplicate_feature_registration_is_rejected_without_replacement() {
        let mut catalogue =
            ToolchainApiCapabilities::new(api_version());

        catalogue
            .register_feature(
                ApiFeatureDescriptor::new(
                    feature_id("build.incremental"),
                    ApiCapabilityStatus::Available,
                ),
            )
            .expect("first registration");

        let duplicate = catalogue.register_feature(
            ApiFeatureDescriptor::new(
                feature_id("build.incremental"),
                ApiCapabilityStatus::Unavailable,
            ),
        );

        assert!(duplicate.is_err());

        assert_eq!(
            catalogue
                .feature(&feature_id("build.incremental"))
                .expect("original feature")
                .status,
            ApiCapabilityStatus::Available,
        );
    }

    #[test]
    fn experimental_is_not_reported_as_stably_available() {
        let mut catalogue =
            ToolchainApiCapabilities::new(api_version());

        catalogue
            .register_feature(
                ApiFeatureDescriptor::new(
                    feature_id("future.experimental"),
                    ApiCapabilityStatus::Experimental,
                ),
            )
            .expect("feature registration");

        assert!(
            !catalogue.is_feature_available(
                &feature_id("future.experimental"),
            )
        );
    }

    #[test]
    fn unavailable_and_unknown_are_not_reported_as_available() {
        for status in [
            ApiCapabilityStatus::Unavailable,
            ApiCapabilityStatus::Unknown,
            ApiCapabilityStatus::RequiresConfiguration,
        ] {
            let mut catalogue =
                ToolchainApiCapabilities::new(api_version());

            catalogue
                .register_feature(
                    ApiFeatureDescriptor::new(
                        feature_id("provider.feature"),
                        status,
                    ),
                )
                .expect("feature registration");

            assert!(
                !catalogue.is_feature_available(
                    &feature_id("provider.feature"),
                )
            );
        }
    }

    #[test]
    fn third_party_identifiers_are_supported() {
        let mut catalogue =
            ToolchainApiCapabilities::new(api_version());

        catalogue
            .register_feature(
                ApiFeatureDescriptor::new(
                    feature_id("third_party.example.quantum.workflow"),
                    ApiCapabilityStatus::Available,
                )
                .with_owner("third_party.example"),
            )
            .expect("third-party feature");

        catalogue
            .register_operation(
                ApiOperationDescriptor::new(
                    operation_id("third_party.example.execute_workflow"),
                    ApiCapabilityStatus::Available,
                )
                .requiring_feature(
                    feature_id("third_party.example.quantum.workflow"),
                ),
            )
            .expect("third-party operation");

        assert!(catalogue.validate().is_ok());
    }

    #[test]
    fn extension_keys_must_be_namespaced() {
        let mut catalogue =
            ToolchainApiCapabilities::new(api_version());

        assert!(
            catalogue
                .set_extension(
                    "vendor",
                    serde_json::json!({"enabled": true}),
                )
                .is_err()
        );

        assert!(
            catalogue
                .set_extension(
                    "vendor.example",
                    serde_json::json!({"enabled": true}),
                )
                .is_ok()
        );

        assert!(catalogue.validate().is_ok());
    }

    #[test]
    fn extension_values_round_trip_as_json() {
        let mut catalogue =
            ToolchainApiCapabilities::new(api_version());

        catalogue
            .set_extension(
                "vendor.example",
                serde_json::json!({
                    "protocol": "v2",
                    "metadata": {
                        "supports_streaming": true
                    }
                }),
            )
            .expect("extension insertion");

        let encoded =
            serde_json::to_string(&catalogue).expect("serialize");

        let decoded: ToolchainApiCapabilities =
            serde_json::from_str(&encoded).expect("deserialize");

        assert_eq!(decoded, catalogue);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn malformed_identifiers_are_rejected() {
        assert!(ApiOperationId::new("").is_err());
        assert!(ApiOperationId::new(" build").is_err());
        assert!(ApiFeatureId::new("feature\ninvalid").is_err());
    }

    #[test]
    fn descriptions_reject_control_characters() {
        let descriptor = ApiFeatureDescriptor::new(
            feature_id("build.incremental"),
            ApiCapabilityStatus::Available,
        )
        .with_description("invalid\nterminal text");

        assert!(descriptor.validate().is_err());
    }

    #[test]
    fn map_keys_must_match_descriptor_identifiers() {
        let mut catalogue =
            ToolchainApiCapabilities::new(api_version());

        catalogue.operations.insert(
            operation_id("build"),
            ApiOperationDescriptor::new(
                operation_id("package"),
                ApiCapabilityStatus::Available,
            ),
        );

        assert!(catalogue.validate().is_err());
    }

    #[test]
    fn serialization_order_is_deterministic() {
        let mut catalogue =
            ToolchainApiCapabilities::new(api_version());

        catalogue
            .register_feature(
                ApiFeatureDescriptor::new(
                    feature_id("z.feature"),
                    ApiCapabilityStatus::Available,
                ),
            )
            .expect("feature registration");

        catalogue
            .register_feature(
                ApiFeatureDescriptor::new(
                    feature_id("a.feature"),
                    ApiCapabilityStatus::Available,
                ),
            )
            .expect("feature registration");

        let first =
            serde_json::to_string(&catalogue).expect("serialize");

        let second =
            serde_json::to_string(&catalogue).expect("serialize");

        assert_eq!(first, second);
    }

    #[test]
    fn large_open_ended_catalogues_have_no_application_count_limit() {
        let mut catalogue =
            ToolchainApiCapabilities::new(api_version());

        for index in 0_u64..1_000 {
            catalogue
                .register_feature(
                    ApiFeatureDescriptor::new(
                        feature_id(&format!("third_party.example.feature.{index}")),
                        ApiCapabilityStatus::Available,
                    ),
                )
                .expect("feature registration");
        }

        assert_eq!(catalogue.features.len(), 1_000);
        assert!(catalogue.validate().is_ok());
    }
}
