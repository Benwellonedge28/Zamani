
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Provider Contracts
//!
//! Path: `src/stdlib/provider.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Defines the provider-neutral contracts used by the Zamani standard
//! library to describe independently implemented computational services.
//!
//! A provider may supply classical, quantum, HDL, hybrid, distributed,
//! AI, scientific, networking, storage, security or future functionality.
//!
//! This module defines the common provider contract. It does not implement
//! any particular computational domain or select a physical execution
//! target.
//!
//! ## OWNS
//!
//! - ProviderId.
//! - ProviderKind.
//! - ProviderContractId.
//! - ProviderOperationId.
//! - ProviderOperationDescriptor.
//! - ProviderDescriptor.
//! - ProviderStatus.
//! - Provider.
//! - ProviderLifecycle.
//! - ProviderFuture.
//! - Provider metadata validation.
//! - Provider capability declarations.
//!
//! ## DOES NOT OWN
//!
//! - Provider registration or registry storage.
//! - Provider discovery or dynamic library loading.
//! - Provider authentication or trust verification.
//! - Provider sandboxing.
//! - Provider admission or authorization.
//! - Compiler extension registration.
//! - Backend registration.
//! - Quantum hardware registration.
//! - Hardware discovery.
//! - Device identity or topology.
//! - Physical resource allocation.
//! - Provider selection.
//! - Scheduling or routing.
//! - Semantic requirement evaluation.
//! - Feasibility assessment.
//! - Runtime execution.
//! - Distributed execution.
//! - Source grammar or AST representation.
//!
//! These responsibilities remain with their canonical repository owners.
//!
//! ## ARCHITECTURAL AUTHORITIES
//!
//! Standard-library operation context:
//!     crate::stdlib::context
//!
//! Standard-library errors:
//!     crate::stdlib::error
//!
//! Standard-library versions:
//!     crate::stdlib::version
//!
//! Standard-library capability descriptions:
//!     crate::stdlib::capability
//!
//! Compiler extension providers:
//!     crate::compiler::extensions::provider
//!
//! Compiler extension registry:
//!     crate::compiler::extensions::registry
//!
//! Quantum hardware providers:
//!     crate::quantum::hardware::provider
//!
//! Quantum hardware registry:
//!     crate::quantum::hardware::provider_registry
//!
//! Runtime:
//!     crate::runtime
//!
//! Distributed execution:
//!     crate::distributed
//!
//! HAL and physical platform integration:
//!     Canonical HAL/provider implementations.
//!
//! ## PROVIDER MODEL
//!
//! A provider is an independently identifiable implementation.
//!
//! A provider descriptor describes:
//!
//! - provider identity;
//! - provider kind;
//! - implementation version;
//! - provider API version;
//! - supported operation contracts;
//! - declared capabilities;
//! - descriptive metadata.
//!
//! A provider capability declaration is not proof that the capability
//! is currently available. The provider must supply an evidence-backed
//! capability snapshot when queried.
//!
//! A capability snapshot is observational. It does not reserve resources
//! or guarantee that they remain available.
//!
//! ## OPERATION CONTRACTS
//!
//! Operation descriptors describe provider-supported operation identities
//! and their request/response contract identifiers.
//!
//! They do not define domain-specific request payloads or execution
//! algorithms. Those belong to specialized provider interfaces.
//!
//! For example, quantum providers retain their quantum operation APIs,
//! compiler providers retain their extension APIs, and distributed
//! providers retain their distributed operation APIs.
//!
//! ## LIFECYCLE
//!
//! Provider initialization and shutdown are asynchronous contracts.
//!
//! The lifecycle manager is responsible for ordering, dependency
//! resolution, readiness tracking, rollback and shutdown coordination.
//!
//! This module does not automatically initialize providers.
//!
//! ## POCO-REAF
//!
//! Provider identities, kinds, operation identities and contract
//! identifiers are open-world validated strings.
//!
//! No closed list of vendors, architectures, devices or computing
//! domains is maintained.
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - providers;
//! - provider operations;
//! - capability declarations;
//! - metadata entries;
//! - identifier lengths;
//! - namespace depth;
//! - computational domains;
//! - hardware resources;
//! - distributed participants.
//!
//! Actual growth is limited by available memory, address-space constraints,
//! operating-system constraints and explicitly configured operational
//! budgets.
//!
//! Literal infinite physical computation or storage is not promised.
//!
//! ## SECURITY
//!
//! Provider descriptors are descriptive metadata, not credentials.
//!
//! Do not store passwords, private keys, access tokens, cookies,
//! authorization headers or other secrets in descriptors.
//!
//! Provider registration does not establish trust.
//!
//! Untrusted provider code must be subject to the applicable admission,
//! verification and isolation requirements before activation.
//!
//! ## DETERMINISM
//!
//! Provider metadata attributes use BTreeMap.
//!
//! Capability and operation declarations preserve insertion order.
//!
//! Provider registration order must not determine semantic meaning.
//!
//! Registry ordering belongs to the registry authority.
//!
//! ## ERROR CONTRACT
//!
//! Invalid identifiers, metadata and duplicate declarations return
//! the existing standard-library Error type.
//!
//! Provider implementation failures are propagated through that same
//! error contract.
//!
//! This module does not silently retry, recover or convert failure
//! into success.
//!
//! ## INTEGRATION
//!
//! The standard-library root must expose this module using:
//!
//!     pub mod provider;
//!
//! Provider registries should consume ProviderDescriptor.
//!
//! Specialized domain providers should implement Provider and retain
//! their domain-specific operation interfaces.
//!
//! Lifecycle managers should invoke ProviderLifecycle.
//!
//! Runtime and HAL adapters must preserve provider identity, operation
//! contracts, capability evidence and failure information.

use core::fmt;
use core::future::Future;
use core::pin::Pin;

use std::collections::BTreeMap;

use crate::stdlib::capability::{
    Capability,
    CapabilityId,
    CapabilitySnapshot,
};

use crate::stdlib::context::OperationContext;

use crate::stdlib::error::{
    Error,
    ErrorKind,
    Result,
};

use crate::stdlib::version::Version;

// ============================================================================
// ASYNCHRONOUS PROVIDER RESULT
// ============================================================================

/// A boxed asynchronous provider operation.
///
/// The future is `Send` so it can be driven by a multithreaded runtime
/// when the selected execution environment supports that model.
///
/// The future lifetime is tied to the borrowed provider and operation
/// context. It cannot outlive those references.
///
/// The standard library does not provide an executor. The canonical
/// runtime owns asynchronous execution.
///
/// Providers targeting a single-threaded environment must implement
/// their integration through a compatible runtime adapter.
pub type ProviderFuture<'a, T> =
    Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;

// ============================================================================
// COMMON VALIDATION
// ============================================================================

const PROVIDER_COMPONENT: &str = "stdlib.provider";

fn provider_error(
    kind: ErrorKind,
    message: impl Into<String>,
) -> Error {
    Error::new(kind, message)
        .in_component(PROVIDER_COMPONENT)
}

fn invalid_input(
    message: impl Into<String>,
) -> Error {
    provider_error(ErrorKind::InvalidInput, message)
}

fn duplicate_entry(
    message: impl Into<String>,
) -> Error {
    provider_error(ErrorKind::AlreadyExists, message)
}

/// Validates an open-world qualified identifier.
///
/// Identifier segments are separated by dots.
///
/// Accepted segment characters:
///
/// - ASCII letters;
/// - ASCII digits;
/// - underscores;
/// - hyphens.
///
/// Identifiers are case-sensitive.
///
/// No arbitrary maximum length or namespace depth is imposed.
fn validate_identifier(
    value: &str,
    field: &'static str,
) -> Result<()> {
    if value.is_empty() {
        return Err(invalid_input(
            format!("{field} must not be empty"),
        ));
    }

    if value.trim() != value {
        return Err(invalid_input(
            format!("{field} must not contain surrounding whitespace"),
        ));
    }

    let valid = value.split('.').all(|segment| {
        !segment.is_empty()
            && segment.bytes().all(|byte| {
                byte.is_ascii_alphanumeric()
                    || byte == b'_'
                    || byte == b'-'
            })
    });

    if !valid {
        return Err(invalid_input(
            format!(
                "{field} must contain non-empty dot-separated \
                 ASCII identifier segments"
            ),
        ));
    }

    Ok(())
}

/// Validates descriptive text without imposing an arbitrary length limit.
///
/// Unicode text is permitted. Control characters and empty values
/// are rejected.
fn validate_description(
    value: &str,
    field: &'static str,
) -> Result<()> {
    if value.trim().is_empty() {
        return Err(invalid_input(
            format!("{field} must not be empty"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_input(
            format!("{field} must not contain control characters"),
        ));
    }

    Ok(())
}

// ============================================================================
// PROVIDER IDENTITY
// ============================================================================

/// Stable, open-world identity of a provider.
///
/// Examples:
///
/// - `org.example.classical`
/// - `org.example.quantum`
/// - `vendor.hardware.fpga`
/// - `community.distributed.execution`
/// - `future.computing.provider`
///
/// These are examples, not reserved identities.
///
/// Provider identity is independent of:
///
/// - hardware vendor;
/// - physical device;
/// - operating system;
/// - execution location;
/// - provider implementation language;
/// - computational domain;
/// - endpoint URL;
/// - credentials.
///
/// The identity is case-sensitive.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProviderId(String);

impl ProviderId {
    /// Constructs a validated provider identity.
    ///
    /// No arbitrary maximum identifier length is imposed.
    ///
    /// # Errors
    ///
    /// Returns a standard-library error for malformed identifiers.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "provider identifier",
        )?;

        Ok(Self(value))
    }

    /// Returns the exact provider identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProviderId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl TryFrom<&str> for ProviderId {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value)
    }
}

impl TryFrom<String> for ProviderId {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Self::new(value)
    }
}

// ============================================================================
// PROVIDER KIND
// ============================================================================

/// Open-world classification of a provider.
///
/// Examples:
///
/// - `computation`
/// - `classical`
/// - `quantum`
/// - `hdl`
/// - `hybrid`
/// - `distributed`
/// - `ai`
/// - `storage`
/// - `networking`
/// - `security`
/// - `scientific`
/// - `custom`
///
/// These examples do not define a closed list.
///
/// A provider may support several computational domains. Its kind
/// is descriptive metadata, not a restriction on the capabilities
/// it may declare.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProviderKind(String);

impl ProviderKind {
    /// Constructs a validated provider kind.
    ///
    /// # Errors
    ///
    /// Returns a standard-library error for malformed identifiers.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "provider kind",
        )?;

        Ok(Self(value))
    }

    /// Returns the exact provider-kind identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProviderKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// OPERATION IDENTITY
// ============================================================================

/// Stable identity of an operation exposed by a provider.
///
/// Operation identities are open-world and independent of any
/// particular computing domain.
///
/// Examples:
///
/// - `classical.matrix_multiplication`
/// - `quantum.circuit_execution`
/// - `hdl.synthesis`
/// - `distributed.collective_operation`
/// - `ai.model_inference`
/// - `future.domain.operation`
///
/// This identity describes an operation contract. It is not a
/// source-language function name or an IR operation identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProviderOperationId(String);

impl ProviderOperationId {
    /// Constructs a validated operation identity.
    ///
    /// # Errors
    ///
    /// Returns a standard-library error for malformed identifiers.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "provider operation identifier",
        )?;

        Ok(Self(value))
    }

    /// Returns the exact operation identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProviderOperationId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// OPERATION CONTRACT IDENTITY
// ============================================================================

/// Identity of a request or response contract.
///
/// Contract identifiers name schemas or typed contract definitions
/// owned by the corresponding domain or integration subsystem.
///
/// This type does not parse, validate or interpret the schema itself.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProviderContractId(String);

impl ProviderContractId {
    /// Constructs a validated provider contract identity.
    ///
    /// # Errors
    ///
    /// Returns a standard-library error for malformed identifiers.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "provider contract identifier",
        )?;

        Ok(Self(value))
    }

    /// Returns the exact contract identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProviderContractId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// OPERATION DESCRIPTOR
// ============================================================================

/// Describes one operation contract exposed by a provider.
///
/// The descriptor identifies the operation and optionally identifies
/// the request and response contracts.
///
/// Actual request and response types remain owned by the specialized
/// computing-domain interface.
///
/// For example, a quantum provider may use its canonical quantum
/// circuit and result types. A distributed provider may use its
/// canonical distributed work and result types.
///
/// This type does not create a competing universal request model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderOperationDescriptor {
    id: ProviderOperationId,
    version: Version,
    description: Option<String>,
    request_contract: Option<ProviderContractId>,
    response_contract: Option<ProviderContractId>,
}

impl ProviderOperationDescriptor {
    /// Creates an operation descriptor.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation identity is invalid.
    pub fn new(
        id: ProviderOperationId,
        version: Version,
    ) -> Self {
        Self {
            id,
            version,
            description: None,
            request_contract: None,
            response_contract: None,
        }
    }

    /// Creates an operation descriptor from a string identity.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation identity is malformed.
    pub fn from_id(
        id: impl Into<String>,
        version: Version,
    ) -> Result<Self> {
        Ok(Self::new(
            ProviderOperationId::new(id)?,
            version,
        ))
    }

    /// Sets the descriptive operation text.
    ///
    /// # Errors
    ///
    /// Returns an error for empty text or control characters.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self> {
        let description = description.into();

        validate_description(
            &description,
            "operation description",
        )?;

        self.description = Some(description);

        Ok(self)
    }

    /// Sets the request and response contract identities.
    ///
    /// These identities refer to contracts owned by another subsystem.
    /// No schema parsing or interpretation is performed here.
    #[must_use]
    pub fn with_contracts(
        mut self,
        request: ProviderContractId,
        response: ProviderContractId,
    ) -> Self {
        self.request_contract = Some(request);
        self.response_contract = Some(response);
        self
    }

    /// Returns the operation identity.
    #[must_use]
    pub fn id(&self) -> &ProviderOperationId {
        &self.id
    }

    /// Returns the operation contract version.
    #[must_use]
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the optional request contract identity.
    #[must_use]
    pub fn request_contract(&self) -> Option<&ProviderContractId> {
        self.request_contract.as_ref()
    }

    /// Returns the optional response contract identity.
    #[must_use]
    pub fn response_contract(&self) -> Option<&ProviderContractId> {
        self.response_contract.as_ref()
    }
}

// ============================================================================
// PROVIDER STATUS
// ============================================================================

/// Observed lifecycle or availability status of a provider.
///
/// Status is an observation, not a capability declaration.
///
/// In particular:
///
/// - `Unknown` does not mean unavailable.
/// - `Degraded` does not mean every operation is unavailable.
/// - `Ready` does not guarantee that every declared capability
///   is currently available.
/// - `Unavailable` does not prove that the provider is permanently
///   unsupported.
///
/// Capability-specific availability must be represented by the
/// capability snapshot.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ProviderStatus {
    /// The provider's current status is not known.
    Unknown,

    /// The provider is being initialized.
    Initializing,

    /// The provider is ready to accept supported operations.
    Ready,

    /// The provider is operational with reported degradation.
    Degraded,

    /// The provider is currently unavailable.
    Unavailable,

    /// The provider is shutting down.
    Stopping,

    /// The provider has completed shutdown.
    Stopped,

    /// The provider has entered a failed state.
    Failed,
}

impl ProviderStatus {
    /// Returns a stable textual status identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Initializing => "initializing",
            Self::Ready => "ready",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
            Self::Stopping => "stopping",
            Self::Stopped => "stopped",
            Self::Failed => "failed",
        }
    }

    /// Returns whether the status indicates that the provider
    /// may accept operations, subject to capability and policy checks.
    #[must_use]
    pub const fn may_accept_operations(self) -> bool {
        matches!(
            self,
            Self::Ready | Self::Degraded
        )
    }
}

impl fmt::Display for ProviderStatus {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// PROVIDER DESCRIPTOR
// ============================================================================

/// Immutable descriptive metadata for a provider.
///
/// A descriptor is suitable for registry storage, inspection,
/// compatibility checks and provider discovery metadata.
///
/// It contains no credentials, live handles, physical allocations,
/// or mutable runtime state.
///
/// Declared capabilities describe what the provider claims to implement.
/// They do not establish current availability.
///
/// Current capability evidence must be obtained through the Provider
/// interface.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderDescriptor {
    id: ProviderId,
    kind: ProviderKind,
    display_name: String,
    implementation_version: Version,
    api_version: Version,
    capabilities: Vec<Capability>,
    operations: Vec<ProviderOperationDescriptor>,
    attributes: BTreeMap<String, String>,
}

impl ProviderDescriptor {
    /// Creates a provider descriptor.
    ///
    /// Both implementation and API versions are explicit because they
    /// identify different compatibility contracts.
    ///
    /// # Errors
    ///
    /// Returns an error if the display name is invalid.
    pub fn new(
        id: ProviderId,
        kind: ProviderKind,
        display_name: impl Into<String>,
        implementation_version: Version,
        api_version: Version,
    ) -> Result<Self> {
        let display_name = display_name.into();

        validate_description(
            &display_name,
            "provider display name",
        )?;

        Ok(Self {
            id,
            kind,
            display_name,
            implementation_version,
            api_version,
            capabilities: Vec::new(),
            operations: Vec::new(),
            attributes: BTreeMap::new(),
        })
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn id(&self) -> &ProviderId {
        &self.id
    }

    /// Returns the provider kind.
    #[must_use]
    pub fn kind(&self) -> &ProviderKind {
        &self.kind
    }

    /// Returns the human-readable display name.
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Returns the implementation version.
    #[must_use]
    pub fn implementation_version(&self) -> &Version {
        &self.implementation_version
    }

    /// Returns the provider API version.
    ///
    /// This version is distinct from the implementation version,
    /// language version, grammar version, compiler version and
    /// IR schema versions.
    #[must_use]
    pub fn api_version(&self) -> &Version {
        &self.api_version
    }

    /// Returns declared capability descriptions in insertion order.
    ///
    /// These declarations are not current availability evidence.
    #[must_use]
    pub fn capabilities(&self) -> &[Capability] {
        &self.capabilities
    }

    /// Returns declared operation contracts in insertion order.
    #[must_use]
    pub fn operations(&self) -> &[ProviderOperationDescriptor] {
        &self.operations
    }

    /// Returns provider attributes in deterministic key order.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }

    /// Returns one provider attribute.
    #[must_use]
    pub fn attribute(&self, key: &str) -> Option<&str> {
        self.attributes.get(key).map(String::as_str)
    }

    /// Finds a declared capability by identity.
    #[must_use]
    pub fn capability(
        &self,
        id: &CapabilityId,
    ) -> Option<&Capability> {
        self.capabilities
            .iter()
            .find(|capability| capability.id() == id)
    }

    /// Finds a declared operation by identity.
    #[must_use]
    pub fn operation(
        &self,
        id: &ProviderOperationId,
    ) -> Option<&ProviderOperationDescriptor> {
        self.operations
            .iter()
            .find(|operation| operation.id() == id)
    }

    /// Adds a declared capability.
    ///
    /// Duplicate capability identities are rejected. An explicit
    /// replacement API is provided for deliberate metadata updates.
    ///
    /// This does not assert that the capability is currently available.
    ///
    /// # Errors
    ///
    /// Returns an error if the capability identity is already declared.
    pub fn insert_capability(
        &mut self,
        capability: Capability,
    ) -> Result<()> {
        if self
            .capabilities
            .iter()
            .any(|existing| existing.id() == capability.id())
        {
            return Err(duplicate_entry(
                format!(
                    "provider {} already declares capability {}",
                    self.id,
                    capability.id()
                ),
            ));
        }

        self.capabilities.push(capability);

        Ok(())
    }

    /// Replaces a declared capability with the same identity.
    ///
    /// # Errors
    ///
    /// Returns an error if the capability has not previously been declared.
    pub fn replace_capability(
        &mut self,
        capability: Capability,
    ) -> Result<()> {
        let Some(existing) = self
            .capabilities
            .iter_mut()
            .find(|existing| existing.id() == capability.id())
        else {
            return Err(provider_error(
                ErrorKind::NotFound,
                format!(
                    "provider {} does not declare capability {}",
                    self.id,
                    capability.id()
                ),
            ));
        };

        *existing = capability;

        Ok(())
    }

    /// Adds an operation contract.
    ///
    /// Duplicate operation identities are rejected.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation identity already exists.
    pub fn insert_operation(
        &mut self,
        operation: ProviderOperationDescriptor,
    ) -> Result<()> {
        if self
            .operations
            .iter()
            .any(|existing| existing.id() == operation.id())
        {
            return Err(duplicate_entry(
                format!(
                    "provider {} already declares operation {}",
                    self.id,
                    operation.id()
                ),
            ));
        }

        self.operations.push(operation);

        Ok(())
    }

    /// Replaces an operation contract with the same identity.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation has not previously been declared.
    pub fn replace_operation(
        &mut self,
        operation: ProviderOperationDescriptor,
    ) -> Result<()> {
        let Some(existing) = self
            .operations
            .iter_mut()
            .find(|existing| existing.id() == operation.id())
        else {
            return Err(provider_error(
                ErrorKind::NotFound,
                format!(
                    "provider {} does not declare operation {}",
                    self.id,
                    operation.id()
                ),
            ));
        };

        *existing = operation;

        Ok(())
    }

    /// Inserts a descriptive metadata attribute.
    ///
    /// Keys are validated identifiers. Values may contain Unicode text
    /// but may not be empty or contain control characters.
    ///
    /// Duplicate keys are rejected rather than silently overwritten.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid metadata or duplicate keys.
    pub fn insert_attribute(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<()> {
        let key = key.into();
        let value = value.into();

        validate_identifier(
            &key,
            "provider attribute key",
        )?;

        validate_description(
            &value,
            "provider attribute value",
        )?;

        if self.attributes.contains_key(&key) {
            return Err(duplicate_entry(
                format!(
                    "provider attribute {key} is already declared"
                ),
            ));
        }

        self.attributes.insert(key, value);

        Ok(())
    }

    /// Explicitly replaces a descriptive metadata attribute.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid metadata or an absent key.
    pub fn replace_attribute(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<()> {
        let key = key.into();
        let value = value.into();

        validate_identifier(
            &key,
            "provider attribute key",
        )?;

        validate_description(
            &value,
            "provider attribute value",
        )?;

        let Some(existing) = self.attributes.get_mut(&key) else {
            return Err(provider_error(
                ErrorKind::NotFound,
                format!("provider attribute {key} does not exist"),
            ));
        };

        *existing = value;

        Ok(())
    }
}

// ============================================================================
// PROVIDER CONTRACT
// ============================================================================

/// Common provider metadata and capability-observation interface.
///
/// This trait is intentionally object-safe so registries and lifecycle
/// managers can hold `Arc<dyn Provider>` without knowing the provider's
/// computational domain.
///
/// Specialized providers extend this trait with their own typed
/// operation interfaces.
///
/// For example:
///
/// - compiler extension providers implement compiler extension contracts;
/// - quantum providers implement quantum hardware operation contracts;
/// - distributed providers implement distributed execution contracts;
/// - backend providers implement backend-specific contracts.
///
/// This trait does not replace those interfaces.
pub trait Provider: Send + Sync {
    /// Returns immutable provider metadata.
    ///
    /// The descriptor must remain stable for the provider instance's
    /// lifetime. A changed implementation or API identity requires a
    /// new provider instance and registration lifecycle.
    fn descriptor(&self) -> &ProviderDescriptor;

    /// Returns the provider's current observed status.
    ///
    /// This is an observation, not a guarantee that an operation will
    /// succeed. A provider may become unavailable immediately afterward.
    fn status(&self) -> ProviderStatus;

    /// Obtains an evidence-backed snapshot of current capabilities.
    ///
    /// The snapshot describes the provider's current observable state.
    ///
    /// This method must not reserve resources, allocate hardware,
    /// choose a target, or guarantee future availability.
    ///
    /// Implementations must:
    ///
    /// - observe the supplied operation context;
    /// - honor cooperative cancellation;
    /// - preserve `Unknown` when evidence is insufficient;
    /// - distinguish unavailable from unsupported;
    /// - avoid treating descriptor declarations as live evidence;
    /// - return structured errors for failed observation.
    ///
    /// # Cancellation
    ///
    /// Implementations should check cancellation before and during
    /// potentially long-running discovery operations.
    fn capability_snapshot<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, CapabilitySnapshot>;
}

// ============================================================================
// PROVIDER LIFECYCLE
// ============================================================================

/// Lifecycle operations for providers managed by a lifecycle authority.
///
/// The lifecycle manager owns ordering, dependency resolution,
/// activation policy, readiness tracking, rollback and shutdown.
///
/// This trait supplies provider-specific lifecycle hooks only.
///
/// Providers that require no explicit initialization or shutdown
/// may implement these hooks as successful no-op operations.
///
/// Lifecycle calls must be safe to coordinate according to the
/// lifecycle manager's documented state machine.
///
/// In particular, providers must not assume that registration itself
/// means they have been initialized or admitted for execution.
pub trait ProviderLifecycle: Provider {
    /// Initializes the provider.
    ///
    /// The lifecycle manager must call this only after the required
    /// admission, compatibility, dependency and security checks.
    ///
    /// Implementations should make repeated initialization safe or
    /// return a structured error describing the current state.
    ///
    /// The operation must observe cooperative cancellation.
    fn initialize<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, ()>;

    /// Shuts down the provider and releases provider-owned resources.
    ///
    /// The lifecycle manager determines when shutdown is appropriate.
    ///
    /// Implementations should make repeated shutdown safe or return
    /// a structured error describing the current state.
    ///
    /// A successful return means the provider has completed its
    /// declared shutdown contract. It does not prove that a remote
    /// device or external process has terminated unless the provider
    /// can establish that fact.
    fn shutdown<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, ()>;
}

// ============================================================================
// CAPABILITY SNAPSHOT VALIDATION
// ============================================================================

/// Checks whether a provider snapshot contains an available state
/// for a particular capability.
///
/// This helper performs only an exact identity lookup.
///
/// It does not:
///
/// - evaluate semantic requirements;
/// - evaluate constraints;
/// - score preferences;
/// - interpret hints;
/// - determine feasibility;
/// - reserve resources;
/// - select a provider.
///
/// An absent entry returns `None`, preserving the distinction between
/// unknown information and established unavailability.
#[must_use]
pub fn capability_status(
    snapshot: &CapabilitySnapshot,
    id: &CapabilityId,
) -> Option<crate::stdlib::capability::CapabilityStatus> {
    snapshot.get(id).map(|state| state.status())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::capability::{
        CapabilityKind,
        CapabilityState,
        CapabilityStatus,
    };

    fn version() -> Version {
        Version::parse("1.0.0")
            .expect("test version must be valid")
    }

    fn provider_descriptor() -> ProviderDescriptor {
        ProviderDescriptor::new(
            ProviderId::new("org.example.classical")
                .expect("provider identity must be valid"),
            ProviderKind::new("classical")
                .expect("provider kind must be valid"),
            "Example Classical Provider",
            version(),
            version(),
        )
        .expect("provider descriptor must be valid")
    }

    fn capability(id: &str) -> Capability {
        Capability::new(
            CapabilityId::new(id)
                .expect("capability identity must be valid"),
            CapabilityKind::new("computation")
                .expect("capability kind must be valid"),
        )
    }

    #[test]
    fn provider_identity_accepts_open_world_namespaces() {
        let id = ProviderId::new(
            "organization.future.architecture.provider",
        )
        .expect("open-world identity should be accepted");

        assert_eq!(
            id.as_str(),
            "organization.future.architecture.provider",
        );
    }

    #[test]
    fn provider_identity_rejects_invalid_segments() {
        assert!(ProviderId::new("").is_err());
        assert!(ProviderId::new(".provider").is_err());
        assert!(ProviderId::new("provider.").is_err());
        assert!(ProviderId::new("provider..extension").is_err());
        assert!(ProviderId::new("provider name").is_err());
    }

    #[test]
    fn provider_identity_is_case_sensitive() {
        let lower = ProviderId::new("org.example")
            .expect("valid identity");

        let upper = ProviderId::new("Org.example")
            .expect("valid identity");

        assert_ne!(lower, upper);
    }

    #[test]
    fn provider_kind_is_open_world() {
        let kind = ProviderKind::new(
            "future.neuromorphic.computation",
        )
        .expect("future provider kinds must be representable");

        assert_eq!(
            kind.as_str(),
            "future.neuromorphic.computation",
        );
    }

    #[test]
    fn descriptor_preserves_separate_versions() {
        let descriptor = provider_descriptor();

        assert_eq!(
            descriptor.implementation_version().to_string(),
            "1.0.0",
        );

        assert_eq!(
            descriptor.api_version().to_string(),
            "1.0.0",
        );
    }

    #[test]
    fn descriptor_rejects_empty_display_name() {
        let result = ProviderDescriptor::new(
            ProviderId::new("org.example")
                .expect("valid identity"),
            ProviderKind::new("computation")
                .expect("valid kind"),
            "   ",
            version(),
            version(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn descriptor_accepts_unicode_display_name() {
        let descriptor = ProviderDescriptor::new(
            ProviderId::new("org.example")
                .expect("valid identity"),
            ProviderKind::new("computation")
                .expect("valid kind"),
            "提供者 — Provider",
            version(),
            version(),
        )
        .expect("Unicode display name should be valid");

        assert_eq!(
            descriptor.display_name(),
            "提供者 — Provider",
        );
    }

    #[test]
    fn descriptor_rejects_duplicate_capability_declarations() {
        let mut descriptor = provider_descriptor();

        descriptor
            .insert_capability(capability("classical.matrix"))
            .expect("first declaration should succeed");

        let result = descriptor
            .insert_capability(capability("classical.matrix"));

        assert!(result.is_err());
        assert_eq!(descriptor.capabilities().len(), 1);
    }

    #[test]
    fn descriptor_replaces_capability_explicitly() {
        let mut descriptor = provider_descriptor();

        descriptor
            .insert_capability(capability("classical.matrix"))
            .expect("initial declaration should succeed");

        descriptor
            .replace_capability(capability("classical.matrix"))
            .expect("explicit replacement should succeed");

        assert_eq!(descriptor.capabilities().len(), 1);
    }

    #[test]
    fn descriptor_rejects_replacement_of_missing_capability() {
        let mut descriptor = provider_descriptor();

        assert!(
            descriptor
                .replace_capability(capability("quantum.measurement"))
                .is_err()
        );
    }

    #[test]
    fn descriptor_rejects_duplicate_operation_declarations() {
        let mut descriptor = provider_descriptor();

        let operation = ProviderOperationDescriptor::from_id(
            "classical.matrix_multiplication",
            version(),
        )
        .expect("operation should be valid");

        descriptor
            .insert_operation(operation.clone())
            .expect("first operation should succeed");

        assert!(
            descriptor
                .insert_operation(operation)
                .is_err()
        );

        assert_eq!(descriptor.operations().len(), 1);
    }

    #[test]
    fn operation_descriptor_preserves_contract_identities() {
        let operation = ProviderOperationDescriptor::from_id(
            "quantum.circuit_execution",
            version(),
        )
        .expect("operation should be valid")
        .with_contracts(
            ProviderContractId::new("quantum.circuit.request")
                .expect("request contract should be valid"),
            ProviderContractId::new("quantum.circuit.result")
                .expect("response contract should be valid"),
        );

        assert_eq!(
            operation.id().as_str(),
            "quantum.circuit_execution",
        );

        assert_eq!(
            operation
                .request_contract()
                .expect("request contract should exist")
                .as_str(),
            "quantum.circuit.request",
        );

        assert_eq!(
            operation
                .response_contract()
                .expect("response contract should exist")
                .as_str(),
            "quantum.circuit.result",
        );
    }

    #[test]
    fn operation_descriptor_rejects_invalid_identity() {
        assert!(
            ProviderOperationId::new("quantum..execution").is_err()
        );

        assert!(
            ProviderContractId::new("contract name").is_err()
        );
    }

    #[test]
    fn descriptor_attributes_are_deterministically_ordered() {
        let mut descriptor = provider_descriptor();

        descriptor
            .insert_attribute("zeta", "last")
            .expect("attribute should be valid");

        descriptor
            .insert_attribute("alpha", "first")
            .expect("attribute should be valid");

        let keys: Vec<&str> = descriptor
            .attributes()
            .keys()
            .map(String::as_str)
            .collect();

        assert_eq!(keys, vec!["alpha", "zeta"]);
    }

    #[test]
    fn duplicate_attributes_are_rejected() {
        let mut descriptor = provider_descriptor();

        descriptor
            .insert_attribute("region", "example")
            .expect("initial attribute should succeed");

        assert!(
            descriptor
                .insert_attribute("region", "replacement")
                .is_err()
        );
    }

    #[test]
    fn attributes_can_be_explicitly_replaced() {
        let mut descriptor = provider_descriptor();

        descriptor
            .insert_attribute("region", "first")
            .expect("initial attribute should succeed");

        descriptor
            .replace_attribute("region", "second")
            .expect("explicit replacement should succeed");

        assert_eq!(
            descriptor.attribute("region"),
            Some("second"),
        );
    }

    #[test]
    fn provider_status_is_not_capability_status() {
        assert!(ProviderStatus::Ready.may_accept_operations());
        assert!(ProviderStatus::Degraded.may_accept_operations());

        assert!(!ProviderStatus::Unknown.may_accept_operations());
        assert!(!ProviderStatus::Unavailable.may_accept_operations());
        assert!(!ProviderStatus::Stopped.may_accept_operations());
        assert!(!ProviderStatus::Failed.may_accept_operations());
    }

    #[test]
    fn absent_capability_remains_unknown() {
        let snapshot = CapabilitySnapshot::new();

        let id = CapabilityId::new("future.computation")
            .expect("capability identity should be valid");

        assert_eq!(
            capability_status(&snapshot, &id),
            None,
        );
    }

    #[test]
    fn available_capability_is_reported_without_resource_reservation() {
        let mut snapshot = CapabilitySnapshot::new();

        let capability = capability("classical.matrix");

        snapshot
            .insert(CapabilityState::new(
                capability,
                CapabilityStatus::Available,
            ))
            .expect("capability should be unique");

        let id = CapabilityId::new("classical.matrix")
            .expect("capability identity should be valid");

        assert_eq!(
            capability_status(&snapshot, &id),
            Some(CapabilityStatus::Available),
        );
    }

    #[test]
    fn unavailable_is_not_unsupported() {
        let mut snapshot = CapabilitySnapshot::new();

        snapshot
            .insert(CapabilityState::new(
                capability("quantum.measurement"),
                CapabilityStatus::Unavailable,
            ))
            .expect("capability should be unique");

        let id = CapabilityId::new("quantum.measurement")
            .expect("capability identity should be valid");

        assert_eq!(
            capability_status(&snapshot, &id),
            Some(CapabilityStatus::Unavailable),
        );
    }

    #[test]
    fn provider_descriptor_contains_no_physical_resource_assumptions() {
        let descriptor = provider_descriptor();

        assert!(descriptor.attributes().is_empty());
        assert!(descriptor.capabilities().is_empty());
        assert!(descriptor.operations().is_empty());
    }
}
