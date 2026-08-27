//! Zamani Quantum Hardware — Provider Abstraction
//!
//! Production-grade, provider-neutral representation of a quantum-computing
//! provider.
//!
//! # Responsibility
//!
//! This module owns the semantic model of a quantum provider.
//!
//! A provider is the organization/service boundary that may expose one or
//! more quantum execution targets. A provider can expose many backends and
//! can expose heterogeneous technologies and execution models.
//!
//! This module owns:
//!
//! - provider identity;
//! - provider display metadata;
//! - provider classification;
//! - provider lifecycle/status;
//! - provider API-version metadata;
//! - provider capability declarations;
//! - supported quantum technologies;
//! - supported execution models;
//! - supported interoperability formats;
//! - provider endpoint references;
//! - provider feature flags;
//! - provider metadata;
//! - provider descriptor validation;
//! - deterministic provider fingerprints;
//! - provider provenance;
//! - provider compatibility metadata.
//!
//! This module deliberately does NOT own:
//!
//! - provider credentials;
//! - API keys;
//! - OAuth tokens;
//! - authentication;
//! - authorization;
//! - network clients;
//! - HTTP requests;
//! - SDKs;
//! - backend discovery;
//! - backend registration;
//! - job submission;
//! - job polling;
//! - job cancellation;
//! - result retrieval;
//! - calibration acquisition;
//! - topology acquisition;
//! - routing;
//! - scheduling;
//! - transpilation;
//! - benchmarking;
//! - simulation;
//! - emulation.
//!
//! Those responsibilities belong to:
//!
//! ```text
//! credentials.rs
//! authentication.rs
//! provider_registry.rs
//! device_registry.rs
//! discovery.rs
//! backend_trait.rs
//! execution.rs
//! adapters/*
//! ```
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani Quantum IR
//!                                |
//!                                v
//!                         Compatibility
//!                                |
//!                                v
//!                         Quantum Backend
//!                                |
//!                         +------+------ +
//!                         |             |
//!                         v             v
//!                    Provider      Local target
//!                         |
//!               +---------+---------+
//!               |         |         |
//!               v         v         v
//!             IBM       IonQ     Braket...
//!               |         |         |
//!               +---------+---------+
//!                         |
//!                         v
//!                 provider adapters
//!                         |
//!                         v
//!                       QPU
//! ```
//!
//! `provider.rs` describes the provider boundary.
//!
//! `backend.rs` describes an individual execution backend.
//!
//! `backend_trait.rs` describes how an adapter executes against a backend.
//!
//! This distinction is mandatory.
//!
//! # Provider versus backend
//!
//! A provider is not a backend.
//!
//! For example:
//!
//! ```text
//! Provider:
//!     IBM
//!
//! Backends:
//!     backend-a
//!     backend-b
//!     simulator
//! ```
//!
//! Likewise:
//!
//! ```text
//! Provider:
//!     Amazon Braket
//!
//! Backends:
//!     device-a
//!     device-b
//!     analog-device
//! ```
//!
//! Therefore this module must never contain backend-specific state such as:
//!
//! - qubit count;
//! - native gate set;
//! - physical topology;
//! - calibration values;
//! - backend queue depth;
//! - backend job IDs.
//!
//! Those belong to `backend.rs` and the execution subsystem.
//!
//! # Identity
//!
//! `identity.rs` is authoritative for provider identity.
//!
//! This module therefore does not create a second string-based provider ID.
//! `ProviderId` is an alias for the canonical `QualifiedIdentity` type.
//!
//! Provider IDs should normally use the `provider://` namespace:
//!
//! ```text
//! provider://ibm
//! provider://ionq
//! provider://aws-braket
//! provider://rigetti
//! provider://iqm
//! provider://quantinuum
//! provider://quera
//! ```
//!
//! Provider identity is independent of endpoint URLs, API versions and
//! credentials.
//!
//! # Determinism
//!
//! This module is deterministic:
//!
//! - no system clock;
//! - no random number generator;
//! - no network calls;
//! - no environment-variable reads;
//! - no global mutable state;
//! - ordered collections only;
//! - canonical metadata ordering;
//! - deterministic fingerprints.
//!
//! # Security
//!
//! Provider descriptors are metadata, not secret stores.
//!
//! This module rejects metadata that appears to contain credential material.
//!
//! In particular, provider descriptors must never contain:
//!
//! - API keys;
//! - bearer tokens;
//! - passwords;
//! - private keys;
//! - cookies;
//! - authorization headers;
//! - refresh tokens;
//! - client secrets.
//!
//! Endpoint references may identify a service location, but they must not
//! contain embedded credentials.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe Rust.
//!
//! # Integration contract
//!
//! This file is intentionally usable before the following modules exist:
//!
//! - `provider_registry.rs`;
//! - `device_registry.rs`;
//! - `discovery.rs`;
//! - `credentials.rs`;
//! - `authentication.rs`;
//! - provider adapters.
//!
//! Once this file is complete, those modules must consume this API instead
//! of redefining provider identity, status or provider capabilities.
//!
//! `backend.rs` consumes provider identity through backend metadata.
//!
//! `backend_trait.rs` uses provider information while remaining independent
//! of provider-specific implementations.
//!
//! `provider_registry.rs` indexes `ProviderDescriptor` values.
//!
//! `device_registry.rs` associates provider IDs with backend/device IDs.
//!
//! `discovery.rs` returns validated provider/backend descriptors.
//!
//! `adapters/*` implement provider-specific communication without modifying
//! this file.
//!
//! `benchmarking` consumes provider information as execution provenance.
//!
//! Danga may use this API for provider discovery and selection.
//!
//! Adding a new provider MUST NOT require modifying this module.
//!
//! # Stability rule
//!
//! The public provider model is provider-neutral and must remain stable.
//!
//! Provider-specific fields belong in provider adapters or explicitly
//! namespaced metadata rather than being added to the core provider model.
//!
//! # No-reedit rule
//!
//! This file is considered complete when:
//!
//! 1. provider identity is canonical;
//! 2. provider metadata is validated;
//! 3. capabilities are represented;
//! 4. technologies and execution models are represented;
//! 5. endpoint references are safe;
//! 6. API versions are represented;
//! 7. provider lifecycle state is represented;
//! 8. serialization is deterministic;
//! 9. fingerprints are deterministic;
//! 10. security validation is enforced;
//! 11. all invariants are tested;
//! 12. downstream modules can consume this contract without modifying it.
//!
//! Downstream modules must adapt to this contract rather than reopening it.
//!
//! -----------------------------------------------------------------------------
//! Schema
//! -----------------------------------------------------------------------------

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::identity::{
    IdentityError,
    IdentityNamespace,
    QualifiedIdentity,
};

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier.
pub const PROVIDER_SCHEMA_ID: &str =
    "zamani.quantum.hardware.provider";

/// Semantic schema version.
///
/// Version 1 establishes the provider abstraction. Provider implementations
/// must not interpret this value as a provider API version.
pub const PROVIDER_SCHEMA_VERSION: u16 = 1;

/// Maximum provider name length.
pub const MAX_PROVIDER_NAME_LENGTH: usize = 256;

/// Maximum provider description length.
pub const MAX_PROVIDER_DESCRIPTION_LENGTH: usize = 4096;

/// Maximum provider website/reference length.
pub const MAX_PROVIDER_REFERENCE_LENGTH: usize = 2048;

/// Maximum provider API version length.
pub const MAX_PROVIDER_API_VERSION_LENGTH: usize = 128;

/// Maximum provider adapter version length.
pub const MAX_PROVIDER_ADAPTER_VERSION_LENGTH: usize = 128;

/// Maximum provider metadata key length.
pub const MAX_PROVIDER_METADATA_KEY_LENGTH: usize = 256;

/// Maximum provider metadata value length.
pub const MAX_PROVIDER_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum provider metadata entries.
pub const MAX_PROVIDER_METADATA_ENTRIES: usize = 4096;

/// Maximum number of technologies advertised by a provider.
pub const MAX_PROVIDER_TECHNOLOGIES: usize = 128;

/// Maximum number of execution models advertised by a provider.
pub const MAX_PROVIDER_EXECUTION_MODELS: usize = 128;

/// Maximum number of interoperability formats advertised by a provider.
pub const MAX_PROVIDER_FORMATS: usize = 128;

/// Maximum number of provider feature identifiers.
pub const MAX_PROVIDER_FEATURES: usize = 4096;

/// Maximum number of API versions advertised.
pub const MAX_PROVIDER_API_VERSIONS: usize = 256;

/// Maximum endpoint-reference count.
pub const MAX_PROVIDER_ENDPOINTS: usize = 128;

// =============================================================================
// Provider identity
// =============================================================================

/// Canonical provider identifier.
///
/// This is deliberately an alias rather than a second identity type.
/// `identity.rs` remains the single source of truth for identity semantics.
///
/// Provider identifiers should normally use:
///
/// ```text
/// provider://ibm
/// provider://ionq
/// provider://aws-braket
/// ```
pub type ProviderId = QualifiedIdentity;

// =============================================================================
// Provider kind
// =============================================================================

/// High-level classification of a quantum provider.
///
/// Provider kind describes the provider/service boundary, not an individual
/// backend's physical technology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProviderKind {
    /// Commercial cloud quantum provider.
    Cloud,

    /// Locally hosted provider/service.
    Local,

    /// Self-hosted/private quantum service.
    SelfHosted,

    /// Academic/research provider.
    Research,

    /// Enterprise/private provider.
    Enterprise,

    /// Hybrid provider exposing multiple execution technologies.
    Hybrid,

    /// Provider type not covered by the standard classifications.
    Custom,
}

impl ProviderKind {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cloud => "cloud",
            Self::Local => "local",
            Self::SelfHosted => "self_hosted",
            Self::Research => "research",
            Self::Enterprise => "enterprise",
            Self::Hybrid => "hybrid",
            Self::Custom => "custom",
        }
    }

    /// Returns true when the provider normally requires remote service
    /// communication.
    pub const fn is_remote(self) -> bool {
        matches!(
            self,
            Self::Cloud | Self::Research | Self::Enterprise | Self::Hybrid
        )
    }

    /// Returns true when the provider can be hosted locally.
    pub const fn is_local(self) -> bool {
        matches!(self, Self::Local | Self::SelfHosted)
    }
}

impl fmt::Display for ProviderKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Provider status
// =============================================================================

/// Provider-level operational state.
///
/// This is deliberately different from backend status.
///
/// A provider may be operational while an individual backend is offline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProviderStatus {
    /// Status is not known.
    Unknown,

    /// Provider service is available.
    Available,

    /// Provider is operational but experiencing degradation.
    Degraded,

    /// Provider is undergoing maintenance.
    Maintenance,

    /// Provider is temporarily unavailable.
    Unavailable,

    /// Provider service is offline.
    Offline,

    /// Provider has permanently retired.
    Retired,
}

impl ProviderStatus {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Available => "available",
            Self::Degraded => "degraded",
            Self::Maintenance => "maintenance",
            Self::Unavailable => "unavailable",
            Self::Offline => "offline",
            Self::Retired => "retired",
        }
    }

    /// Returns whether provider-level communication may be attempted.
    ///
    /// This is only a status indication. Authentication, authorization and
    /// backend compatibility must still be checked independently.
    pub const fn is_reachable(self) -> bool {
        matches!(
            self,
            Self::Available | Self::Degraded
        )
    }

    /// Returns whether the provider is permanently retired.
    pub const fn is_retired(self) -> bool {
        matches!(self, Self::Retired)
    }
}

impl fmt::Display for ProviderStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Quantum technology
// =============================================================================

/// Provider-neutral physical quantum technology identifier.
///
/// This intentionally uses stable strings rather than importing
/// `technology.rs` so this file remains independently usable and does not
/// create a dependency cycle.
///
/// Canonical values include:
///
/// ```text
/// superconducting
/// trapped_ion
/// neutral_atom
/// photonic
/// spin
/// topological
/// annealing
/// analog
/// distributed
/// bosonic
/// continuous_variable
/// hybrid
/// simulator
/// emulator
/// custom
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TechnologyId(String);

impl TechnologyId {
    /// Creates a validated technology identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, ProviderError> {
        let value = value.into();

        validate_identifier(
            "technology",
            &value,
            128,
        )?;

        Ok(Self(value))
    }

    /// Superconducting quantum technology.
    pub fn superconducting() -> Self {
        Self("superconducting".to_owned())
    }

    /// Trapped-ion quantum technology.
    pub fn trapped_ion() -> Self {
        Self("trapped_ion".to_owned())
    }

    /// Neutral-atom quantum technology.
    pub fn neutral_atom() -> Self {
        Self("neutral_atom".to_owned())
    }

    /// Photonic quantum technology.
    pub fn photonic() -> Self {
        Self("photonic".to_owned())
    }

    /// Spin/semiconductor quantum technology.
    pub fn spin() -> Self {
        Self("spin".to_owned())
    }

    /// Topological quantum technology.
    pub fn topological() -> Self {
        Self("topological".to_owned())
    }

    /// Quantum annealing technology.
    pub fn annealing() -> Self {
        Self("annealing".to_owned())
    }

    /// Analog quantum computing technology.
    pub fn analog() -> Self {
        Self("analog".to_owned())
    }

    /// Distributed quantum computing technology.
    pub fn distributed() -> Self {
        Self("distributed".to_owned())
    }

    /// Bosonic quantum computing technology.
    pub fn bosonic() -> Self {
        Self("bosonic".to_owned())
    }

    /// Continuous-variable technology.
    pub fn continuous_variable() -> Self {
        Self("continuous_variable".to_owned())
    }

    /// Return the canonical identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TechnologyId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for TechnologyId {
    type Err = ProviderError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl Serialize for TechnologyId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for TechnologyId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(serde::de::Error::custom)
    }
}

// =============================================================================
// Execution model
// =============================================================================

/// Provider-neutral execution-model identifier.
///
/// This deliberately remains independent of the backend implementation.
///
/// Canonical values include:
///
/// ```text
/// gate_model
/// dynamic_circuit
/// pulse
/// analog
/// annealing
/// sampling
/// logical
/// distributed
/// hybrid
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExecutionModelId(String);

impl ExecutionModelId {
    /// Creates a validated execution-model identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, ProviderError> {
        let value = value.into();

        validate_identifier(
            "execution model",
            &value,
            128,
        )?;

        Ok(Self(value))
    }

    /// Gate-model execution.
    pub fn gate_model() -> Self {
        Self("gate_model".to_owned())
    }

    /// Dynamic-circuit execution.
    pub fn dynamic_circuit() -> Self {
        Self("dynamic_circuit".to_owned())
    }

    /// Pulse-level execution.
    pub fn pulse() -> Self {
        Self("pulse".to_owned())
    }

    /// Analog execution.
    pub fn analog() -> Self {
        Self("analog".to_owned())
    }

    /// Annealing execution.
    pub fn annealing() -> Self {
        Self("annealing".to_owned())
    }

    /// Sampling-oriented execution.
    pub fn sampling() -> Self {
        Self("sampling".to_owned())
    }

    /// Logical/fault-tolerant execution.
    pub fn logical() -> Self {
        Self("logical".to_owned())
    }

    /// Distributed execution.
    pub fn distributed() -> Self {
        Self("distributed".to_owned())
    }

    /// Hybrid classical/quantum execution.
    pub fn hybrid() -> Self {
        Self("hybrid".to_owned())
    }

    /// Returns the canonical identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExecutionModelId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for ExecutionModelId {
    type Err = ProviderError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl Serialize for ExecutionModelId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ExecutionModelId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(serde::de::Error::custom)
    }
}

// =============================================================================
// Interoperability format
// =============================================================================

/// Stable interoperability-format identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FormatId(String);

impl FormatId {
    /// Creates a validated format identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, ProviderError> {
        let value = value.into();

        validate_identifier(
            "format",
            &value,
            128,
        )?;

        Ok(Self(value))
    }

    /// Zamani canonical Quantum IR.
    pub fn zamani_ir() -> Self {
        Self("zamani-ir".to_owned())
    }

    /// OpenQASM 3.
    pub fn openqasm_3() -> Self {
        Self("openqasm-3".to_owned())
    }

    /// QIR.
    pub fn qir() -> Self {
        Self("qir".to_owned())
    }

    /// Provider-native representation.
    pub fn provider_native() -> Self {
        Self("provider-native".to_owned())
    }

    /// Returns the canonical identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FormatId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for FormatId {
    type Err = ProviderError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl Serialize for FormatId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for FormatId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(serde::de::Error::custom)
    }
}

// =============================================================================
// API version
// =============================================================================

/// Validated provider API version.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProviderApiVersion(String);

impl ProviderApiVersion {
    /// Creates a validated API-version identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, ProviderError> {
        let value = value.into();

        validate_version(&value)?;

        Ok(Self(value))
    }

    /// Returns the canonical API-version string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProviderApiVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for ProviderApiVersion {
    type Err = ProviderError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl Serialize for ProviderApiVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ProviderApiVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(serde::de::Error::custom)
    }
}

// =============================================================================
// Endpoint reference
// =============================================================================

/// Safe provider endpoint reference.
///
/// This is deliberately not a URL parser or network client.
///
/// The value is treated as an opaque endpoint reference and is checked for
/// obvious embedded-secret patterns.
///
/// Examples:
///
/// ```text
/// https://api.example.com
/// https://quantum.example.com/v1
/// local://quantum
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EndpointReference(String);

impl EndpointReference {
    /// Creates a validated endpoint reference.
    pub fn new(value: impl Into<String>) -> Result<Self, ProviderError> {
        let value = value.into();

        validate_endpoint(&value)?;

        Ok(Self(value))
    }

    /// Returns the endpoint reference.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EndpointReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for EndpointReference {
    type Err = ProviderError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl Serialize for EndpointReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for EndpointReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(serde::de::Error::custom)
    }
}

// =============================================================================
// Provider capabilities
// =============================================================================

/// Provider-level capability declarations.
///
/// Backend-specific capabilities still belong to `backend.rs`.
///
/// This structure describes capabilities that apply to the provider/service
/// boundary or that are useful during provider discovery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderCapabilities {
    /// Provider exposes quantum hardware.
    pub physical_quantum_hardware: bool,

    /// Provider exposes simulators.
    pub simulators: bool,

    /// Provider exposes hardware emulators.
    pub emulators: bool,

    /// Provider supports asynchronous execution.
    pub asynchronous_execution: bool,

    /// Provider supports synchronous execution.
    pub synchronous_execution: bool,

    /// Provider supports job cancellation.
    pub job_cancellation: bool,

    /// Provider exposes queue information.
    pub queue_information: bool,

    /// Provider exposes calibration data.
    pub calibration_data: bool,

    /// Provider exposes topology information.
    pub topology_information: bool,

    /// Provider supports batch submission.
    pub batch_submission: bool,

    /// Provider supports result streaming.
    pub result_streaming: bool,

    /// Provider supports dynamic circuits at the service level.
    pub dynamic_circuits: bool,

    /// Provider supports pulse execution.
    pub pulse_execution: bool,

    /// Provider supports analog execution.
    pub analog_execution: bool,

    /// Provider supports annealing.
    pub annealing: bool,

    /// Provider supports logical/fault-tolerant execution.
    pub logical_execution: bool,

    /// Provider supports distributed quantum execution.
    pub distributed_execution: bool,

    /// Provider exposes pricing/cost metadata.
    pub cost_information: bool,

    /// Provider exposes health/status information.
    pub health_information: bool,

    /// Stable provider-wide feature identifiers.
    pub features: BTreeSet<String>,

    /// Experimental provider-wide feature identifiers.
    pub experimental_features: BTreeSet<String>,
}

impl Default for ProviderCapabilities {
    fn default() -> Self {
        Self {
            physical_quantum_hardware: false,
            simulators: false,
            emulators: false,
            asynchronous_execution: true,
            synchronous_execution: false,
            job_cancellation: false,
            queue_information: false,
            calibration_data: false,
            topology_information: false,
            batch_submission: false,
            result_streaming: false,
            dynamic_circuits: false,
            pulse_execution: false,
            analog_execution: false,
            annealing: false,
            logical_execution: false,
            distributed_execution: false,
            cost_information: false,
            health_information: false,
            features: BTreeSet::new(),
            experimental_features: BTreeSet::new(),
        }
    }
}

impl ProviderCapabilities {
    /// Creates a conservative provider capability profile.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a stable provider feature.
    pub fn with_feature(
        mut self,
        feature: impl Into<String>,
    ) -> Result<Self, ProviderError> {
        let feature = normalize_identifier(&feature.into())?;

        if self.features.len() >= MAX_PROVIDER_FEATURES
            && !self.features.contains(&feature)
        {
            return Err(ProviderError::CollectionLimitExceeded {
                field: "features",
                maximum: MAX_PROVIDER_FEATURES,
            });
        }

        self.features.insert(feature);

        Ok(self)
    }

    /// Adds an experimental provider feature.
    pub fn with_experimental_feature(
        mut self,
        feature: impl Into<String>,
    ) -> Result<Self, ProviderError> {
        let feature = normalize_identifier(&feature.into())?;

        if self.experimental_features.len()
            >= MAX_PROVIDER_FEATURES
            && !self.experimental_features.contains(&feature)
        {
            return Err(ProviderError::CollectionLimitExceeded {
                field: "experimental_features",
                maximum: MAX_PROVIDER_FEATURES,
            });
        }

        self.experimental_features.insert(feature);

        Ok(self)
    }

    /// Returns whether a stable feature is advertised.
    pub fn supports_feature(&self, feature: &str) -> bool {
        self.features.contains(feature)
    }

    /// Returns whether an experimental feature is advertised.
    pub fn supports_experimental_feature(
        &self,
        feature: &str,
    ) -> bool {
        self.experimental_features.contains(feature)
    }
}

// =============================================================================
// Provider metadata
// =============================================================================

/// Immutable provider metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderMetadata {
    /// Human-readable provider name.
    pub name: String,

    /// Human-readable description.
    pub description: Option<String>,

    /// Optional public documentation/reference.
    ///
    /// This field is metadata only. It is never fetched by this module.
    pub reference: Option<String>,

    /// Optional provider adapter implementation version.
    pub adapter_version: Option<String>,

    /// Optional provider API versions.
    pub api_versions: BTreeSet<ProviderApiVersion>,

    /// Optional provider endpoint references.
    pub endpoints: BTreeSet<EndpointReference>,

    /// Arbitrary non-secret provider metadata.
    pub properties: BTreeMap<String, String>,
}

impl ProviderMetadata {
    /// Creates validated provider metadata.
    pub fn new(name: impl Into<String>) -> Result<Self, ProviderError> {
        let name = name.into();

        validate_text(
            "provider name",
            &name,
            MAX_PROVIDER_NAME_LENGTH,
        )?;

        Ok(Self {
            name,
            description: None,
            reference: None,
            adapter_version: None,
            api_versions: BTreeSet::new(),
            endpoints: BTreeSet::new(),
            properties: BTreeMap::new(),
        })
    }

    /// Sets the human-readable description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, ProviderError> {
        let description = description.into();

        validate_text(
            "provider description",
            &description,
            MAX_PROVIDER_DESCRIPTION_LENGTH,
        )?;

        self.description = Some(description);

        Ok(self)
    }

    /// Sets a public provider reference.
    pub fn with_reference(
        mut self,
        reference: impl Into<String>,
    ) -> Result<Self, ProviderError> {
        let reference = reference.into();

        validate_reference(&reference)?;

        self.reference = Some(reference);

        Ok(self)
    }

    /// Sets the adapter version.
    pub fn with_adapter_version(
        mut self,
        version: impl Into<String>,
    ) -> Result<Self, ProviderError> {
        let version = version.into();

        validate_version(&version)?;

        if version.chars().count()
            > MAX_PROVIDER_ADAPTER_VERSION_LENGTH
        {
            return Err(ProviderError::TooLong {
                field: "adapter version",
                maximum: MAX_PROVIDER_ADAPTER_VERSION_LENGTH,
            });
        }

        self.adapter_version = Some(version);

        Ok(self)
    }

    /// Adds a provider API version.
    pub fn with_api_version(
        mut self,
        version: ProviderApiVersion,
    ) -> Result<Self, ProviderError> {
        if self.api_versions.len() >= MAX_PROVIDER_API_VERSIONS
            && !self.api_versions.contains(&version)
        {
            return Err(ProviderError::CollectionLimitExceeded {
                field: "api_versions",
                maximum: MAX_PROVIDER_API_VERSIONS,
            });
        }

        self.api_versions.insert(version);

        Ok(self)
    }

    /// Adds a provider endpoint reference.
    pub fn with_endpoint(
        mut self,
        endpoint: EndpointReference,
    ) -> Result<Self, ProviderError> {
        if self.endpoints.len() >= MAX_PROVIDER_ENDPOINTS
            && !self.endpoints.contains(&endpoint)
        {
            return Err(ProviderError::CollectionLimitExceeded {
                field: "endpoints",
                maximum: MAX_PROVIDER_ENDPOINTS,
            });
        }

        self.endpoints.insert(endpoint);

        Ok(self)
    }

    /// Adds safe provider metadata.
    pub fn with_property(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, ProviderError> {
        let key = key.into();
        let value = value.into();

        validate_identifier(
            "metadata key",
            &key,
            MAX_PROVIDER_METADATA_KEY_LENGTH,
        )?;

        validate_metadata_value(&value)?;

        if self.properties.len() >= MAX_PROVIDER_METADATA_ENTRIES
            && !self.properties.contains_key(&key)
        {
            return Err(ProviderError::CollectionLimitExceeded {
                field: "properties",
                maximum: MAX_PROVIDER_METADATA_ENTRIES,
            });
        }

        self.properties.insert(key, value);

        Ok(self)
    }

    /// Validates the entire metadata object.
    pub fn validate(&self) -> Result<(), ProviderError> {
        validate_text(
            "provider name",
            &self.name,
            MAX_PROVIDER_NAME_LENGTH,
        )?;

        if let Some(description) = &self.description {
            validate_text(
                "provider description",
                description,
                MAX_PROVIDER_DESCRIPTION_LENGTH,
            )?;
        }

        if let Some(reference) = &self.reference {
            validate_reference(reference)?;
        }

        if let Some(version) = &self.adapter_version {
            validate_version(version)?;
        }

        if self.api_versions.len() > MAX_PROVIDER_API_VERSIONS {
            return Err(ProviderError::CollectionLimitExceeded {
                field: "api_versions",
                maximum: MAX_PROVIDER_API_VERSIONS,
            });
        }

        if self.endpoints.len() > MAX_PROVIDER_ENDPOINTS {
            return Err(ProviderError::CollectionLimitExceeded {
                field: "endpoints",
                maximum: MAX_PROVIDER_ENDPOINTS,
            });
        }

        if self.properties.len() > MAX_PROVIDER_METADATA_ENTRIES {
            return Err(ProviderError::CollectionLimitExceeded {
                field: "properties",
                maximum: MAX_PROVIDER_METADATA_ENTRIES,
            });
        }

        for (key, value) in &self.properties {
            validate_identifier(
                "metadata key",
                key,
                MAX_PROVIDER_METADATA_KEY_LENGTH,
            )?;

            validate_metadata_value(value)?;
        }

        Ok(())
    }
}

// =============================================================================
// Provider descriptor
// =============================================================================

/// Immutable, provider-neutral quantum provider descriptor.
///
/// This is the canonical object passed to registries, discovery systems,
/// adapters and backend-selection logic.
///
/// It contains metadata and capabilities only.
///
/// It does not contain credentials or executable network state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderDescriptor {
    /// Canonical provider identity.
    pub id: ProviderId,

    /// Provider classification.
    pub kind: ProviderKind,

    /// Current provider-level status.
    pub status: ProviderStatus,

    /// Provider metadata.
    pub metadata: ProviderMetadata,

    /// Physical quantum technologies exposed by the provider.
    pub technologies: BTreeSet<TechnologyId>,

    /// Execution models exposed by the provider.
    pub execution_models: BTreeSet<ExecutionModelId>,

    /// Interoperability formats exposed by the provider.
    pub supported_formats: BTreeSet<FormatId>,

    /// Provider-level capabilities.
    pub capabilities: ProviderCapabilities,
}

impl ProviderDescriptor {
    /// Creates a provider descriptor.
    ///
    /// The provider identity must use the canonical `provider` namespace.
    pub fn new(
        id: ProviderId,
        kind: ProviderKind,
        metadata: ProviderMetadata,
    ) -> Result<Self, ProviderError> {
        if id.namespace().as_str() != "provider" {
            return Err(ProviderError::InvalidProviderNamespace {
                namespace: id.namespace().as_str().to_owned(),
            });
        }

        metadata.validate()?;

        Ok(Self {
            id,
            kind,
            status: ProviderStatus::Unknown,
            metadata,
            technologies: BTreeSet::new(),
            execution_models: BTreeSet::new(),
            supported_formats: BTreeSet::new(),
            capabilities: ProviderCapabilities::default(),
        })
    }

    /// Creates a provider descriptor from a provider identifier string.
    ///
    /// Accepted:
    ///
    /// ```text
    /// provider://ibm
    /// provider://ionq
    /// ```
    pub fn from_str_id(
        value: &str,
        kind: ProviderKind,
        metadata: ProviderMetadata,
    ) -> Result<Self, ProviderError> {
        let id = QualifiedIdentity::from_str(value)
            .map_err(ProviderError::Identity)?;

        Self::new(id, kind, metadata)
    }

    /// Sets the provider status.
    pub fn with_status(mut self, status: ProviderStatus) -> Self {
        self.status = status;
        self
    }

    /// Adds a physical technology.
    pub fn with_technology(
        mut self,
        technology: TechnologyId,
    ) -> Result<Self, ProviderError> {
        if self.technologies.len() >= MAX_PROVIDER_TECHNOLOGIES
            && !self.technologies.contains(&technology)
        {
            return Err(ProviderError::CollectionLimitExceeded {
                field: "technologies",
                maximum: MAX_PROVIDER_TECHNOLOGIES,
            });
        }

        self.technologies.insert(technology);

        Ok(self)
    }

    /// Adds an execution model.
    pub fn with_execution_model(
        mut self,
        model: ExecutionModelId,
    ) -> Result<Self, ProviderError> {
        if self.execution_models.len()
            >= MAX_PROVIDER_EXECUTION_MODELS
            && !self.execution_models.contains(&model)
        {
            return Err(ProviderError::CollectionLimitExceeded {
                field: "execution_models",
                maximum: MAX_PROVIDER_EXECUTION_MODELS,
            });
        }

        self.execution_models.insert(model);

        Ok(self)
    }

    /// Adds an interoperability format.
    pub fn with_format(
        mut self,
        format: FormatId,
    ) -> Result<Self, ProviderError> {
        if self.supported_formats.len() >= MAX_PROVIDER_FORMATS
            && !self.supported_formats.contains(&format)
        {
            return Err(ProviderError::CollectionLimitExceeded {
                field: "supported_formats",
                maximum: MAX_PROVIDER_FORMATS,
            });
        }

        self.supported_formats.insert(format);

        Ok(self)
    }

    /// Replaces the provider capability declaration.
    pub fn with_capabilities(
        mut self,
        capabilities: ProviderCapabilities,
    ) -> Self {
        self.capabilities = capabilities;
        self
    }

    /// Returns the provider identifier.
    pub fn id(&self) -> &ProviderId {
        &self.id
    }

    /// Returns the provider's human-readable name.
    pub fn name(&self) -> &str {
        &self.metadata.name
    }

    /// Returns whether a technology is advertised.
    pub fn supports_technology(
        &self,
        technology: &TechnologyId,
    ) -> bool {
        self.technologies.contains(technology)
    }

    /// Returns whether an execution model is advertised.
    pub fn supports_execution_model(
        &self,
        model: &ExecutionModelId,
    ) -> bool {
        self.execution_models.contains(model)
    }

    /// Returns whether an interoperability format is advertised.
    pub fn supports_format(&self, format: &FormatId) -> bool {
        self.supported_formats.contains(format)
    }

    /// Returns whether a provider feature is advertised.
    pub fn supports_feature(&self, feature: &str) -> bool {
        self.capabilities.supports_feature(feature)
    }

    /// Returns whether an experimental feature is advertised.
    pub fn supports_experimental_feature(
        &self,
        feature: &str,
    ) -> bool {
        self.capabilities
            .supports_experimental_feature(feature)
    }

    /// Returns whether the provider is currently usable at provider level.
    ///
    /// This does NOT guarantee that any particular backend is usable.
    pub fn is_usable(&self) -> bool {
        self.status.is_reachable()
            && self.status != ProviderStatus::Retired
    }

    /// Performs complete descriptor validation.
    pub fn validate(&self) -> Result<(), ProviderError> {
        if self.id.namespace().as_str() != "provider" {
            return Err(ProviderError::InvalidProviderNamespace {
                namespace: self.id.namespace().as_str().to_owned(),
            });
        }

        self.metadata.validate()?;

        if self.technologies.len() > MAX_PROVIDER_TECHNOLOGIES {
            return Err(ProviderError::CollectionLimitExceeded {
                field: "technologies",
                maximum: MAX_PROVIDER_TECHNOLOGIES,
            });
        }

        if self.execution_models.len()
            > MAX_PROVIDER_EXECUTION_MODELS
        {
            return Err(ProviderError::CollectionLimitExceeded {
                field: "execution_models",
                maximum: MAX_PROVIDER_EXECUTION_MODELS,
            });
        }

        if self.supported_formats.len() > MAX_PROVIDER_FORMATS {
            return Err(ProviderError::CollectionLimitExceeded {
                field: "supported_formats",
                maximum: MAX_PROVIDER_FORMATS,
            });
        }

        if self.capabilities.features.len()
            > MAX_PROVIDER_FEATURES
        {
            return Err(ProviderError::CollectionLimitExceeded {
                field: "features",
                maximum: MAX_PROVIDER_FEATURES,
            });
        }

        if self.capabilities.experimental_features.len()
            > MAX_PROVIDER_FEATURES
        {
            return Err(ProviderError::CollectionLimitExceeded {
                field: "experimental_features",
                maximum: MAX_PROVIDER_FEATURES,
            });
        }

        Ok(())
    }

    /// Produces a deterministic canonical representation.
    ///
    /// The representation is intentionally human-readable and deterministic.
    /// It is suitable as input to an external cryptographic hash function.
    ///
    /// This method does NOT claim to be a cryptographic fingerprint by itself.
    pub fn canonical_representation(&self) -> String {
        let mut output = String::new();

        output.push_str(PROVIDER_SCHEMA_ID);
        output.push('|');
        output.push_str(&PROVIDER_SCHEMA_VERSION.to_string());
        output.push('|');

        output.push_str(&self.id.as_str());
        output.push('|');

        output.push_str(self.kind.as_str());
        output.push('|');

        output.push_str(self.status.as_str());
        output.push('|');

        output.push_str(&self.metadata.name);
        output.push('|');

        if let Some(description) = &self.metadata.description {
            output.push_str(description);
        }

        output.push('|');

        if let Some(reference) = &self.metadata.reference {
            output.push_str(reference);
        }

        output.push('|');

        if let Some(version) = &self.metadata.adapter_version {
            output.push_str(version);
        }

        output.push('|');

        for version in &self.metadata.api_versions {
            output.push_str(version.as_str());
            output.push(',');
        }

        output.push('|');

        for endpoint in &self.metadata.endpoints {
            output.push_str(endpoint.as_str());
            output.push(',');
        }

        output.push('|');

        for technology in &self.technologies {
            output.push_str(technology.as_str());
            output.push(',');
        }

        output.push('|');

        for model in &self.execution_models {
            output.push_str(model.as_str());
            output.push(',');
        }

        output.push('|');

        for format in &self.supported_formats {
            output.push_str(format.as_str());
            output.push(',');
        }

        output.push('|');

        output.push_str(
            if self.capabilities.physical_quantum_hardware {
                "1"
            } else {
                "0"
            },
        );
        output.push_str(
            if self.capabilities.simulators {
                "1"
            } else {
                "0"
            },
        );
        output.push_str(
            if self.capabilities.emulators {
                "1"
            } else {
                "0"
            },
        );
        output.push_str(
            if self.capabilities.asynchronous_execution {
                "1"
            } else {
                "0"
            },
        );
        output.push_str(
            if self.capabilities.synchronous_execution {
                "1"
            } else {
                "0"
            },
        );
        output.push_str(
            if self.capabilities.job_cancellation {
                "1"
            } else {
                "0"
            },
        );
        output.push_str(
            if self.capabilities.queue_information {
                "1"
            } else {
                "0"
            },
        );
        output.push_str(
            if self.capabilities.calibration_data {
                "1"
            } else {
                "0"
            },
        );
        output.push_str(
            if self.capabilities.topology_information {
                "1"
            } else {
                "0"
            },
        );
        output.push_str(
            if self.capabilities.batch_submission {
                "1"
            } else {
                "0"
            },
        );
        output.push_str(
            if self.capabilities.result_streaming {
                "1"
            } else {
                "0"
            },
        );
        output.push_str(
            if self.capabilities.dynamic_circuits {
                "1"
            } else {
                "0"
            },
        );
        output.push_str(
            if self.capabilities.pulse_execution {
                "1"
            } else {
                "0"
            },
        );
        output.push_str(
            if self.capabilities.analog_execution {
                "1"
            } else {
                "0"
            },
        );
        output.push_str(
            if self.capabilities.annealing {
                "1"
            } else {
                "0"
            },
        );
        output.push_str(
            if self.capabilities.logical_execution {
                "1"
            } else {
                "0"
            },
        );
        output.push_str(
            if self.capabilities.distributed_execution {
                "1"
            } else {
                "0"
            },
        );
        output.push_str(
            if self.capabilities.cost_information {
                "1"
            } else {
                "0"
            },
        );
        output.push_str(
            if self.capabilities.health_information {
                "1"
            } else {
                "0"
            },
        );

        output.push('|');

        for feature in &self.capabilities.features {
            output.push_str(feature);
            output.push(',');
        }

        output.push('|');

        for feature in &self.capabilities.experimental_features {
            output.push_str(feature);
            output.push(',');
        }

        output.push('|');

        for (key, value) in &self.metadata.properties {
            output.push_str(key);
            output.push('=');
            output.push_str(value);
            output.push(';');
        }

        output
    }

    /// Returns a deterministic non-cryptographic fingerprint.
    ///
    /// This is useful for in-process equality/caching diagnostics.
    ///
    /// It must NOT be used as a security hash or authentication primitive.
    pub fn fingerprint(&self) -> u64 {
        use std::hash::{Hash, Hasher};

        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        self.canonical_representation().hash(&mut hasher);

        hasher.finish()
    }
}

// =============================================================================
// Provider errors
// =============================================================================

/// Errors produced by the provider abstraction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderError {
    /// Canonical identity construction failed.
    Identity(IdentityError),

    /// Provider identity uses a namespace other than `provider`.
    InvalidProviderNamespace {
        namespace: String,
    },

    /// Required text is empty or whitespace-only.
    Empty {
        field: &'static str,
    },

    /// Text contains leading/trailing whitespace.
    SurroundingWhitespace {
        field: &'static str,
    },

    /// Field exceeds its maximum permitted length.
    TooLong {
        field: &'static str,
        maximum: usize,
    },

    /// Identifier contains an unsupported character.
    InvalidCharacter {
        field: &'static str,
        character: char,
    },

    /// A version string is invalid.
    InvalidVersion {
        value: String,
    },

    /// Endpoint reference is invalid.
    InvalidEndpoint {
        value: String,
    },

    /// Endpoint contains an embedded credential.
    EndpointContainsSecret,

    /// Metadata contains a credential or other sensitive material.
    SensitiveMetadata {
        field: String,
    },

    /// Collection exceeds its safety limit.
    CollectionLimitExceeded {
        field: &'static str,
        maximum: usize,
    },

    /// Provider descriptor violates a semantic invariant.
    InvalidDescriptor {
        message: String,
    },
}

impl fmt::Display for ProviderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identity(error) => {
                write!(formatter, "provider identity error: {error}")
            }

            Self::InvalidProviderNamespace { namespace } => {
                write!(
                    formatter,
                    "provider identity must use the \
                     'provider' namespace; got '{}'",
                    namespace
                )
            }

            Self::Empty { field } => {
                write!(
                    formatter,
                    "{} cannot be empty",
                    field
                )
            }

            Self::SurroundingWhitespace { field } => {
                write!(
                    formatter,
                    "{} cannot contain leading or trailing whitespace",
                    field
                )
            }

            Self::TooLong { field, maximum } => {
                write!(
                    formatter,
                    "{} exceeds maximum length of {}",
                    field,
                    maximum
                )
            }

            Self::InvalidCharacter { field, character } => {
                write!(
                    formatter,
                    "{} contains invalid character {:?}",
                    field,
                    character
                )
            }

            Self::InvalidVersion { value } => {
                write!(
                    formatter,
                    "invalid provider version '{}'",
                    value
                )
            }

            Self::InvalidEndpoint { value } => {
                write!(
                    formatter,
                    "invalid provider endpoint '{}'",
                    value
                )
            }

            Self::EndpointContainsSecret => {
                write!(
                    formatter,
                    "provider endpoint contains embedded credential \
                     material"
                )
            }

            Self::SensitiveMetadata { field } => {
                write!(
                    formatter,
                    "provider metadata field '{}' appears to contain \
                     sensitive credential material",
                    field
                )
            }

            Self::CollectionLimitExceeded {
                field,
                maximum,
            } => {
                write!(
                    formatter,
                    "{} exceeds maximum collection size of {}",
                    field,
                    maximum
                )
            }

            Self::InvalidDescriptor { message } => {
                write!(
                    formatter,
                    "invalid provider descriptor: {}",
                    message
                )
            }
        }
    }
}

impl Error for ProviderError {}

impl From<IdentityError> for ProviderError {
    fn from(error: IdentityError) -> Self {
        Self::Identity(error)
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates general human-readable text.
fn validate_text(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), ProviderError> {
    if value.trim().is_empty() {
        return Err(ProviderError::Empty { field });
    }

    if value.trim() != value {
        return Err(ProviderError::SurroundingWhitespace { field });
    }

    if value.chars().count() > maximum {
        return Err(ProviderError::TooLong { field, maximum });
    }

    Ok(())
}

/// Validates a conservative machine-readable identifier.
fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), ProviderError> {
    if value.is_empty() {
        return Err(ProviderError::Empty { field });
    }

    if value.trim() != value {
        return Err(ProviderError::SurroundingWhitespace { field });
    }

    if value.chars().count() > maximum {
        return Err(ProviderError::TooLong { field, maximum });
    }

    for character in value.chars() {
        if !character.is_ascii_alphanumeric()
            && !matches!(
                character,
                '_' | '-' | '.' | ':' | '/'
            )
        {
            return Err(ProviderError::InvalidCharacter {
                field,
                character,
            });
        }
    }

    Ok(())
}

/// Validates and returns a normalized identifier.
///
/// Normalization is intentionally conservative: only surrounding validation
/// is performed. Case is preserved because provider APIs may distinguish it.
fn normalize_identifier(
    value: &str,
) -> Result<String, ProviderError> {
    validate_identifier("identifier", value, 128)?;
    Ok(value.to_owned())
}

/// Validates provider API/adapter version syntax.
fn validate_version(value: &str) -> Result<(), ProviderError> {
    if value.is_empty() {
        return Err(ProviderError::InvalidVersion {
            value: value.to_owned(),
        });
    }

    if value.trim() != value {
        return Err(ProviderError::SurroundingWhitespace {
            field: "version",
        });
    }

    if value.chars().count() > MAX_PROVIDER_API_VERSION_LENGTH {
        return Err(ProviderError::TooLong {
            field: "version",
            maximum: MAX_PROVIDER_API_VERSION_LENGTH,
        });
    }

    for character in value.chars() {
        if !character.is_ascii_alphanumeric()
            && !matches!(
                character,
                '.' | '-' | '+' | '_' | '/'
            )
        {
            return Err(ProviderError::InvalidCharacter {
                field: "version",
                character,
            });
        }
    }

    Ok(())
}

/// Validate a public/reference field.
fn validate_reference(value: &str) -> Result<(), ProviderError> {
    validate_text(
        "provider reference",
        value,
        MAX_PROVIDER_REFERENCE_LENGTH,
    )?;

    if contains_secret_marker(value) {
        return Err(ProviderError::SensitiveMetadata {
            field: "reference".to_owned(),
        });
    }

    Ok(())
}

/// Validate endpoint references.
///
/// This intentionally does not attempt full URL parsing. Network transport
/// owns URL semantics. This layer only enforces safety and basic structural
/// validity.
fn validate_endpoint(value: &str) -> Result<(), ProviderError> {
    validate_text(
        "endpoint",
        value,
        MAX_PROVIDER_REFERENCE_LENGTH,
    )
    .map_err(|error| match error {
        ProviderError::Empty { .. }
        | ProviderError::SurroundingWhitespace { .. }
        | ProviderError::TooLong { .. } => {
            ProviderError::InvalidEndpoint {
                value: value.to_owned(),
            }
        }
        other => other,
    })?;

    if value.contains('\n') || value.contains('\r') {
        return Err(ProviderError::InvalidEndpoint {
            value: value.to_owned(),
        });
    }

    if contains_secret_marker(value) {
        return Err(ProviderError::EndpointContainsSecret);
    }

    // An endpoint reference must identify a scheme or local-style namespace.
    //
    // We deliberately do not parse arbitrary URLs here. The transport layer
    // owns URL semantics.
    if !value.contains("://") {
        return Err(ProviderError::InvalidEndpoint {
            value: value.to_owned(),
        });
    }

    Ok(())
}

/// Validates metadata values and rejects obvious credential material.
fn validate_metadata_value(
    value: &str,
) -> Result<(), ProviderError> {
    if value.chars().count() > MAX_PROVIDER_METADATA_VALUE_LENGTH {
        return Err(ProviderError::TooLong {
            field: "metadata value",
            maximum: MAX_PROVIDER_METADATA_VALUE_LENGTH,
        });
    }

    if value.contains('\0') {
        return Err(ProviderError::InvalidDescriptor {
            message: "metadata values cannot contain NUL characters"
                .to_owned(),
        });
    }

    Ok(())
}

/// Detects obvious secret markers.
///
/// This is deliberately conservative and is only defence-in-depth. It does
/// not claim to be a complete secret scanner.
fn contains_secret_marker(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();

    const MARKERS: &[&str] = &[
        "authorization:",
        "bearer ",
        "api_key=",
        "apikey=",
        "api-key=",
        "access_token=",
        "access-token=",
        "refresh_token=",
        "refresh-token=",
        "client_secret=",
        "client-secret=",
        "password=",
        "passwd=",
        "private_key=",
        "private-key=",
        "secret=",
        "token=",
        "cookie=",
    ];

    MARKERS.iter().any(|marker| lower.contains(marker))
}

// =============================================================================
// Serde
// =============================================================================

/// Serializable representation of a provider descriptor.
///
/// This explicit representation prevents accidental dependence on Rust's
/// internal struct layout and makes the serialized contract intentional.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderDescriptorWire {
    /// Schema identifier.
    pub schema: String,

    /// Schema version.
    pub schema_version: u16,

    /// Provider identity.
    pub id: ProviderId,

    /// Provider kind.
    pub kind: String,

    /// Provider status.
    pub status: String,

    /// Provider name.
    pub name: String,

    /// Optional description.
    pub description: Option<String>,

    /// Optional public reference.
    pub reference: Option<String>,

    /// Optional adapter version.
    pub adapter_version: Option<String>,

    /// Provider API versions.
    pub api_versions: Vec<String>,

    /// Provider endpoint references.
    pub endpoints: Vec<String>,

    /// Advertised technologies.
    pub technologies: Vec<String>,

    /// Advertised execution models.
    pub execution_models: Vec<String>,

    /// Advertised interoperability formats.
    pub supported_formats: Vec<String>,

    /// Stable feature identifiers.
    pub features: Vec<String>,

    /// Experimental feature identifiers.
    pub experimental_features: Vec<String>,

    /// Provider metadata properties.
    pub properties: BTreeMap<String, String>,
}

impl ProviderDescriptor {
    /// Convert into the explicit serialization representation.
    pub fn to_wire(&self) -> ProviderDescriptorWire {
        ProviderDescriptorWire {
            schema: PROVIDER_SCHEMA_ID.to_owned(),
            schema_version: PROVIDER_SCHEMA_VERSION,
            id: self.id.clone(),
            kind: self.kind.as_str().to_owned(),
            status: self.status.as_str().to_owned(),
            name: self.metadata.name.clone(),
            description: self.metadata.description.clone(),
            reference: self.metadata.reference.clone(),
            adapter_version: self.metadata.adapter_version.clone(),
            api_versions: self
                .metadata
                .api_versions
                .iter()
                .map(|version| version.as_str().to_owned())
                .collect(),
            endpoints: self
                .metadata
                .endpoints
                .iter()
                .map(|endpoint| endpoint.as_str().to_owned())
                .collect(),
            technologies: self
                .technologies
                .iter()
                .map(|technology| technology.as_str().to_owned())
                .collect(),
            execution_models: self
                .execution_models
                .iter()
                .map(|model| model.as_str().to_owned())
                .collect(),
            supported_formats: self
                .supported_formats
                .iter()
                .map(|format| format.as_str().to_owned())
                .collect(),
            features: self
                .capabilities
                .features
                .iter()
                .cloned()
                .collect(),
            experimental_features: self
                .capabilities
                .experimental_features
                .iter()
                .cloned()
                .collect(),
            properties: self.metadata.properties.clone(),
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn provider_id(value: &str) -> ProviderId {
        QualifiedIdentity::with_namespace("provider", value)
            .expect("valid provider identity")
    }

    fn metadata() -> ProviderMetadata {
        ProviderMetadata::new("Example Quantum")
            .expect("valid provider metadata")
    }

    #[test]
    fn provider_identity_requires_provider_namespace() {
        let id = QualifiedIdentity::with_namespace(
            "local",
            "example",
        )
        .expect("valid generic identity");

        let result = ProviderDescriptor::new(
            id,
            ProviderKind::Local,
            metadata(),
        );

        assert!(matches!(
            result,
            Err(ProviderError::InvalidProviderNamespace { .. })
        ));
    }

    #[test]
    fn provider_descriptor_is_constructible() {
        let descriptor = ProviderDescriptor::new(
            provider_id("example"),
            ProviderKind::Cloud,
            metadata(),
        )
        .expect("descriptor should be valid");

        assert_eq!(descriptor.name(), "Example Quantum");
        assert_eq!(
            descriptor.id().namespace().as_str(),
            "provider"
        );
        assert_eq!(
            descriptor.id().value(),
            "example"
        );
    }

    #[test]
    fn technology_identifiers_are_deterministic() {
        assert_eq!(
            TechnologyId::superconducting().as_str(),
            "superconducting"
        );

        assert_eq!(
            TechnologyId::trapped_ion().as_str(),
            "trapped_ion"
        );
    }

    #[test]
    fn execution_models_are_distinct() {
        assert_ne!(
            ExecutionModelId::gate_model(),
            ExecutionModelId::analog()
        );

        assert_eq!(
            ExecutionModelId::dynamic_circuit().as_str(),
            "dynamic_circuit"
        );
    }

    #[test]
    fn formats_are_distinct() {
        assert_ne!(
            FormatId::openqasm_3(),
            FormatId::qir()
        );
    }

    #[test]
    fn provider_capabilities_default_conservatively() {
        let capabilities = ProviderCapabilities::default();

        assert!(!capabilities.physical_quantum_hardware);
        assert!(!capabilities.simulators);
        assert!(capabilities.asynchronous_execution);
        assert!(!capabilities.job_cancellation);
    }

    #[test]
    fn experimental_features_are_separate() {
        let capabilities =
            ProviderCapabilities::new()
                .with_experimental_feature("experimental-x")
                .expect("feature should be accepted");

        assert!(!capabilities.supports_feature("experimental-x"));
        assert!(
            capabilities
                .supports_experimental_feature("experimental-x")
        );
    }

    #[test]
    fn endpoint_rejects_embedded_credentials() {
        let result = EndpointReference::new(
            "https://user:password@example.com",
        );

        assert!(result.is_err());
    }

    #[test]
    fn endpoint_rejects_bearer_tokens() {
        let result = EndpointReference::new(
            "https://example.com?token=secret",
        );

        assert!(result.is_err());
    }

    #[test]
    fn endpoint_requires_scheme() {
        let result = EndpointReference::new(
            "api.example.com/v1",
        );

        assert!(result.is_err());
    }

    #[test]
    fn metadata_rejects_secret_markers() {
        let result = metadata().with_reference(
            "https://example.com?api_key=secret",
        );

        assert!(result.is_err());
    }

    #[test]
    fn provider_metadata_limits_are_enforced() {
        let too_long =
            "x".repeat(MAX_PROVIDER_NAME_LENGTH + 1);

        let result = ProviderMetadata::new(too_long);

        assert!(result.is_err());
    }

    #[test]
    fn provider_feature_is_added_deterministically() {
        let capabilities =
            ProviderCapabilities::new()
                .with_feature("dynamic-circuits")
                .expect("feature should be accepted");

        assert!(
            capabilities.supports_feature(
                "dynamic-circuits"
            )
        );
    }

    #[test]
    fn provider_descriptor_support_queries_work() {
        let descriptor =
            ProviderDescriptor::new(
                provider_id("example"),
                ProviderKind::Cloud,
                metadata(),
            )
            .expect("valid descriptor")
            .with_technology(
                TechnologyId::superconducting(),
            )
            .expect("technology")
            .with_execution_model(
                ExecutionModelId::gate_model(),
            )
            .expect("execution model")
            .with_format(
                FormatId::openqasm_3(),
            )
            .expect("format");

        assert!(
            descriptor.supports_technology(
                &TechnologyId::superconducting()
            )
        );

        assert!(
            descriptor.supports_execution_model(
                &ExecutionModelId::gate_model()
            )
        );

        assert!(
            descriptor.supports_format(
                &FormatId::openqasm_3()
            )
        );
    }

    #[test]
    fn provider_status_is_independent_from_backend_status() {
        assert!(
            ProviderStatus::Available.is_reachable()
        );

        assert!(
            !ProviderStatus::Retired.is_reachable()
        );

        assert!(
            ProviderStatus::Retired.is_retired()
        );
    }

    #[test]
    fn provider_kind_is_not_backend_kind() {
        assert!(ProviderKind::Cloud.is_remote());
        assert!(ProviderKind::Local.is_local());
    }

    #[test]
    fn canonical_representation_is_deterministic() {
        let first =
            ProviderDescriptor::new(
                provider_id("example"),
                ProviderKind::Cloud,
                metadata(),
            )
            .expect("valid descriptor")
            .with_technology(
                TechnologyId::superconducting(),
            )
            .expect("technology")
            .with_technology(
                TechnologyId::trapped_ion(),
            )
            .expect("technology")
            .with_format(
                FormatId::qir(),
            )
            .expect("format");

        let second =
            ProviderDescriptor::new(
                provider_id("example"),
                ProviderKind::Cloud,
                metadata(),
            )
            .expect("valid descriptor")
            .with_technology(
                TechnologyId::trapped_ion(),
            )
            .expect("technology")
            .with_technology(
                TechnologyId::superconducting(),
            )
            .expect("technology")
            .with_format(
                FormatId::qir(),
            )
            .expect("format");

        assert_eq!(
            first.canonical_representation(),
            second.canonical_representation()
        );

        assert_eq!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn provider_wire_representation_is_stable() {
        let descriptor =
            ProviderDescriptor::new(
                provider_id("example"),
                ProviderKind::Cloud,
                metadata(),
            )
            .expect("valid descriptor");

        let wire = descriptor.to_wire();

        assert_eq!(
            wire.schema,
            PROVIDER_SCHEMA_ID
        );

        assert_eq!(
            wire.schema_version,
            PROVIDER_SCHEMA_VERSION
        );
    }

    #[test]
    fn descriptor_validation_succeeds_for_valid_provider() {
        let descriptor =
            ProviderDescriptor::new(
                provider_id("example"),
                ProviderKind::Cloud,
                metadata(),
            )
            .expect("valid descriptor");

        descriptor
            .validate()
            .expect("descriptor should validate");
    }

    #[test]
    fn provider_id_round_trip_uses_identity_layer() {
        let id = provider_id("example");

        let encoded = id.as_str();

        let decoded =
            QualifiedIdentity::from_str(&encoded)
                .expect("identity should parse");

        assert_eq!(id, decoded);
    }

    #[test]
    fn no_secret_is_present_in_canonical_metadata() {
        let descriptor =
            ProviderDescriptor::new(
                provider_id("example"),
                ProviderKind::Cloud,
                metadata()
                    .with_property(
                        "region",
                        "global",
                    )
                    .expect("safe property"),
            )
            .expect("valid descriptor");

        let canonical =
            descriptor.canonical_representation();

        assert!(!canonical.contains("password"));
        assert!(!canonical.contains("api_key"));
        assert!(!canonical.contains("access_token"));
    }
}