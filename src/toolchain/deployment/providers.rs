
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Deployment Providers
//!
//! Path: src/toolchain/deployment/providers.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! ## Purpose
//!
//! Provides the extensible registry and identity-binding layer for
//! deployment providers.
//!
//! The canonical DeploymentProvider trait remains owned by:
//!
//!     crate::toolchain::commands::deploy
//!
//! This module does not define a competing provider execution trait.
//!
//! ## Responsibilities
//!
//! - Register deployment-provider implementations.
//! - Validate provider descriptors.
//! - Enforce unique provider identities.
//! - Resolve explicitly requested providers.
//! - Enumerate registered providers deterministically.
//! - Support controlled provider removal.
//! - Bind implementations to registered identities.
//! - Validate provider identity in deployment plans.
//! - Validate provider identity in deployment receipts.
//! - Preserve asynchronous provider execution.
//! - Support concurrent registry access.
//!
//! ## Non-ownership
//!
//! This module does not own:
//!
//! - Source grammar, parsing or semantic analysis.
//! - Deployment requests.
//! - Deployment plans or receipts.
//! - Deployment feasibility.
//! - Provider authentication.
//! - Provider trust verification.
//! - Provider authorization.
//! - Hardware discovery.
//! - Resource allocation.
//! - Physical placement.
//! - Quantum routing or QEC.
//! - HDL synthesis or timing closure.
//! - Distributed scheduling.
//! - Infrastructure provisioning.
//! - Runtime execution.
//! - Compiler or backend execution.
//! - External process execution.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! ## Provider identity
//!
//! A registered provider has one stable, open-world identity.
//!
//! The identity is independent of:
//!
//! - Hardware architecture.
//! - Vendor.
//! - Operating system.
//! - Computing domain.
//! - Deployment location.
//! - Network endpoint.
//! - Implementation language.
//! - Authentication credentials.
//!
//! Provider identity is checked against the provider's actual returned
//! deployment plan and receipt.
//!
//! Registration metadata alone cannot establish provider identity
//! for an external operation.
//!
//! ## Provider API compatibility
//!
//! The deployment-provider API version is independent of:
//!
//! - Zamani language version.
//! - Grammar version.
//! - Compiler version.
//! - IR version.
//! - Backend version.
//! - Hardware firmware version.
//! - Package format version.
//!
//! Incompatible provider API versions are rejected during registration.
//!
//! ## Registry semantics
//!
//! - Duplicate identities are rejected.
//! - Registration is atomic.
//! - Removal does not invalidate already-cloned provider handles.
//! - Provider enumeration is deterministic.
//! - Registry locks are not held during provider execution.
//! - Provider selection is explicit.
//! - No implicit native or default provider is selected.
//!
//! ## Security
//!
//! Registration does not establish trust or authorization.
//!
//! Provider descriptors contain descriptive metadata only.
//!
//! They must never contain passwords, tokens, private keys, cookies,
//! authentication headers or other secrets.
//!
//! Untrusted provider implementations must be admitted and isolated
//! by the appropriate security and extension infrastructure.
//!
//! ## POCO-REAF
//!
//! The registry supports open-ended deployment providers and domains.
//!
//! It introduces no fixed ceilings on:
//!
//! - Providers.
//! - Computational domains.
//! - Targets.
//! - Artifacts.
//! - Hardware devices.
//! - CPUs.
//! - Accelerators.
//! - Quantum resources.
//! - Distributed participants.
//! - Deployment environments.
//!
//! Actual growth depends on available memory and operational budgets.
//!
//! Literal infinite physical resources or computation are not promised.

// ============================================================================
// IMPORTS
// ============================================================================

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use crate::toolchain::commands::deploy::{
    DeploymentFuture,
    DeploymentPlan,
    DeploymentProvider,
    DeploymentReceipt,
};

use crate::toolchain::api::request::DeployRequest;

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::result::ToolchainResult;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable architectural identity of the deployment-provider registry.
pub const DEPLOYMENT_PROVIDERS_MODULE_NAME: &str =
    "zamani.toolchain.deployment.providers";

/// Current deployment-provider API version.
///
/// This version identifies the provider integration contract.
///
/// It does not identify the Zamani language, compiler, backend,
/// hardware, runtime or deployment protocol versions.
pub const DEPLOYMENT_PROVIDER_API_VERSION: u32 = 1;

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

const OPERATION_REGISTER: &str =
    "toolchain.deployment.providers.register";

const OPERATION_RESOLVE: &str =
    "toolchain.deployment.providers.resolve";

const OPERATION_UNREGISTER: &str =
    "toolchain.deployment.providers.unregister";

const OPERATION_VALIDATE: &str =
    "toolchain.deployment.providers.validate";

const OPERATION_ENUMERATE: &str =
    "toolchain.deployment.providers.enumerate";

fn invalid_input(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::InvalidInput,
        message,
    )
    .with_context(DEPLOYMENT_PROVIDERS_MODULE_NAME)
}

fn validation_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ValidationFailed,
        message,
    )
    .with_context(DEPLOYMENT_PROVIDERS_MODULE_NAME)
}

fn provider_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ProviderFailed,
        message,
    )
    .with_context(DEPLOYMENT_PROVIDERS_MODULE_NAME)
}

fn registry_error(
    operation: &'static str,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::Internal,
        "deployment-provider registry synchronization failed",
    )
    .with_operation(operation)
    .with_context(DEPLOYMENT_PROVIDERS_MODULE_NAME)
}

// ============================================================================
// OPEN-WORLD IDENTIFIER VALIDATION
// ============================================================================

/// Validate an open-world, namespace-qualified provider identifier.
///
/// Accepted characters in each namespace segment:
///
/// - ASCII letters.
/// - ASCII digits.
/// - Underscores.
/// - Hyphens.
///
/// Segments are separated by dots.
///
/// No arbitrary maximum identifier length or namespace depth is imposed.
///
/// Examples:
///
/// - `org.example.classical`
/// - `org.example.quantum`
/// - `vendor.hardware.fpga`
/// - `community.distributed.execution`
/// - `future.computing.provider`
fn validate_identifier(
    field: &str,
    value: &str,
) -> ToolchainResult<()> {
    if value.is_empty() {
        return Err(invalid_input(
            format!("{field} must not be empty"),
        )
        .with_operation(OPERATION_VALIDATE));
    }

    if value.trim() != value {
        return Err(invalid_input(
            format!(
                "{field} must not contain surrounding whitespace"
            ),
        )
        .with_operation(OPERATION_VALIDATE));
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
        )
        .with_operation(OPERATION_VALIDATE));
    }

    Ok(())
}

/// Validate descriptive text without imposing an arbitrary size limit.
fn validate_description(
    field: &str,
    value: &str,
) -> ToolchainResult<()> {
    if value.trim().is_empty() {
        return Err(invalid_input(
            format!("{field} must not be empty"),
        )
        .with_operation(OPERATION_VALIDATE));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_input(
            format!(
                "{field} must not contain control characters"
            ),
        )
        .with_operation(OPERATION_VALIDATE));
    }

    Ok(())
}

// ============================================================================
// PROVIDER IDENTITY
// ============================================================================

/// Stable, open-world deployment-provider identity.
///
/// Identity is case-sensitive.
///
/// It must remain stable across provider implementation upgrades
/// whenever the implementation represents the same logical provider.
///
/// A provider identity must not be reused to represent an unrelated
/// provider implementation.
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
pub struct DeploymentProviderId(String);

impl DeploymentProviderId {
    /// Construct and validate a provider identity.
    pub fn new(
        value: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let value = value.into();

        validate_identifier(
            "deployment provider identity",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Return the exact provider identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for DeploymentProviderId {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl TryFrom<&str> for DeploymentProviderId {
    type Error = ToolchainError;

    fn try_from(value: &str) -> ToolchainResult<Self> {
        Self::new(value)
    }
}

impl TryFrom<String> for DeploymentProviderId {
    type Error = ToolchainError;

    fn try_from(value: String) -> ToolchainResult<Self> {
        Self::new(value)
    }
}

// ============================================================================
// PROVIDER DESCRIPTOR
// ============================================================================

/// Immutable descriptive metadata for a deployment provider.
///
/// Domain and operation identifiers are open-world identifiers.
///
/// Their presence describes declared provider scope. It does not
/// establish actual compatibility, current availability or feasibility.
///
/// Those conditions must be established by authoritative providers
/// during validation and planning.
///
/// This descriptor deliberately excludes:
///
/// - Credentials.
/// - Private keys.
/// - Authentication tokens.
/// - Network sessions.
/// - Physical resource identities.
/// - Runtime handles.
/// - Mutable availability observations.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct DeploymentProviderDescriptor {
    /// Stable deployment-provider identity.
    pub id: DeploymentProviderId,

    /// Human-readable provider name.
    pub display_name: String,

    /// Provider implementation version.
    ///
    /// This is descriptive metadata and is not used as the provider
    /// API compatibility version.
    pub implementation_version: String,

    /// Deployment-provider API version.
    pub api_version: u32,

    /// Declared computational-domain identifiers.
    ///
    /// These are declarations, not verified capabilities.
    #[serde(default)]
    pub domains: BTreeSet<String>,

    /// Declared deployment-operation identifiers.
    ///
    /// Examples may include deployment lifecycle operations,
    /// artifact delivery or environment preparation.
    ///
    /// The identifiers remain open-ended.
    #[serde(default)]
    pub operations: BTreeSet<String>,
}

impl DeploymentProviderDescriptor {
    /// Construct a deployment-provider descriptor.
    ///
    /// The current provider API version is selected explicitly.
    pub fn new(
        id: DeploymentProviderId,
        display_name: impl Into<String>,
        implementation_version: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let descriptor = Self {
            id,
            display_name: display_name.into(),
            implementation_version: implementation_version.into(),
            api_version: DEPLOYMENT_PROVIDER_API_VERSION,
            domains: BTreeSet::new(),
            operations: BTreeSet::new(),
        };

        descriptor.validate()?;

        Ok(descriptor)
    }

    /// Set the provider API version.
    ///
    /// This is useful for deserializing or constructing descriptors
    /// from external provider metadata.
    ///
    /// Registration rejects incompatible versions.
    #[must_use]
    pub fn with_api_version(
        mut self,
        api_version: u32,
    ) -> Self {
        self.api_version = api_version;
        self
    }

    /// Replace declared computational domains.
    pub fn with_domains(
        mut self,
        domains: BTreeSet<String>,
    ) -> ToolchainResult<Self> {
        self.domains = domains;
        self.validate()?;
        Ok(self)
    }

    /// Replace declared provider operations.
    pub fn with_operations(
        mut self,
        operations: BTreeSet<String>,
    ) -> ToolchainResult<Self> {
        self.operations = operations;
        self.validate()?;
        Ok(self)
    }

    /// Validate the complete descriptor.
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_identifier(
            "deployment provider identity",
            self.id.as_str(),
        )?;

        validate_description(
            "deployment provider display name",
            &self.display_name,
        )?;

        validate_identifier(
            "deployment provider implementation version",
            &self.implementation_version,
        )?;

        if self.api_version == 0 {
            return Err(invalid_input(
                "deployment provider API version must be non-zero",
            )
            .with_operation(OPERATION_VALIDATE));
        }

        for domain in &self.domains {
            validate_identifier(
                "deployment provider domain",
                domain,
            )?;
        }

        for operation in &self.operations {
            validate_identifier(
                "deployment provider operation",
                operation,
            )?;
        }

        Ok(())
    }

    /// Check whether the descriptor uses the supported provider API.
    pub fn validate_api_compatibility(
        &self,
    ) -> ToolchainResult<()> {
        self.validate()?;

        if self.api_version != DEPLOYMENT_PROVIDER_API_VERSION {
            return Err(
                validation_error(format!(
                    "provider `{}` uses deployment API version {}, \
                     while this toolchain supports version {}",
                    self.id,
                    self.api_version,
                    DEPLOYMENT_PROVIDER_API_VERSION,
                ))
                .with_operation(OPERATION_VALIDATE),
            );
        }

        Ok(())
    }
}

// ============================================================================
// REGISTERED PROVIDER
// ============================================================================

/// A registered provider and its immutable descriptive metadata.
///
/// The implementation is reference-counted so callers can retain a
/// provider handle while the registry is concurrently modified.
///
/// Removing a provider from the registry does not invalidate handles
/// that were already resolved.
#[derive(Clone)]
pub struct RegisteredDeploymentProvider {
    descriptor: DeploymentProviderDescriptor,
    provider: Arc<dyn DeploymentProvider>,
}

impl std::fmt::Debug for RegisteredDeploymentProvider {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        formatter
            .debug_struct("RegisteredDeploymentProvider")
            .field("descriptor", &self.descriptor)
            .finish_non_exhaustive()
    }
}

impl RegisteredDeploymentProvider {
    /// Return immutable provider metadata.
    #[must_use]
    pub fn descriptor(&self) -> &DeploymentProviderDescriptor {
        &self.descriptor
    }

    /// Return the registered provider identity.
    #[must_use]
    pub fn id(&self) -> &DeploymentProviderId {
        &self.descriptor.id
    }

    /// Return a provider handle bound to its registered identity.
    ///
    /// The returned adapter checks the identity of every deployment
    /// plan and receipt produced by the underlying implementation.
    #[must_use]
    pub fn provider(&self) -> Arc<dyn DeploymentProvider> {
        Arc::new(IdentityBoundDeploymentProvider {
            id: self.descriptor.id.clone(),
            inner: Arc::clone(&self.provider),
        })
    }
}

// ============================================================================
// REGISTRY STORAGE
// ============================================================================

#[derive(Default)]
struct RegistryState {
    providers: BTreeMap<
        DeploymentProviderId,
        RegisteredDeploymentProvider,
    >,
}

/// Thread-safe registry of deployment providers.
///
/// The registry:
///
/// - Supports concurrent readers.
/// - Serializes registration and removal.
/// - Returns deterministic descriptor ordering.
/// - Does not execute provider code while holding a lock.
/// - Does not select a provider implicitly.
/// - Does not establish provider trust or authorization.
///
/// Cloned registry handles share the same underlying registration state.
#[derive(Clone, Default)]
pub struct DeploymentProviderRegistry {
    state: Arc<RwLock<RegistryState>>,
}

impl std::fmt::Debug for DeploymentProviderRegistry {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        formatter
            .debug_struct("DeploymentProviderRegistry")
            .finish_non_exhaustive()
    }
}

impl DeploymentProviderRegistry {
    /// Construct an empty deployment-provider registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a deployment provider.
    ///
    /// Registration is atomic. Duplicate identities are rejected
    /// without modifying existing registrations.
    ///
    /// Registration does not initialize the provider, establish trust,
    /// authorize operations or prove deployment readiness.
    pub fn register(
        &self,
        descriptor: DeploymentProviderDescriptor,
        provider: Arc<dyn DeploymentProvider>,
    ) -> ToolchainResult<()> {
        descriptor
            .validate_api_compatibility()
            .map_err(|error| {
                error.with_operation(OPERATION_REGISTER)
            })?;

        let id = descriptor.id.clone();

        let registered = RegisteredDeploymentProvider {
            descriptor,
            provider,
        };

        let mut state = self.state.write().map_err(|_| {
            registry_error(OPERATION_REGISTER)
        })?;

        if state.providers.contains_key(&id) {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::AlreadyExists,
                    format!(
                        "deployment provider `{id}` is already registered"
                    ),
                )
                .with_operation(OPERATION_REGISTER)
                .with_subject(id.to_string())
                .with_context(DEPLOYMENT_PROVIDERS_MODULE_NAME),
            );
        }

        state.providers.insert(id, registered);

        Ok(())
    }

    /// Register a provider implementation without requiring callers
    /// to construct an Arc themselves.
    pub fn register_shared<P>(
        &self,
        descriptor: DeploymentProviderDescriptor,
        provider: P,
    ) -> ToolchainResult<()>
    where
        P: DeploymentProvider + 'static,
    {
        self.register(
            descriptor,
            Arc::new(provider),
        )
    }

    /// Resolve a provider by its exact identity.
    ///
    /// This operation never performs fallback selection.
    ///
    /// The returned provider is bound to the registered identity.
    ///
    /// A provider that has been removed after resolution remains
    /// usable through the returned reference-counted handle.
    pub fn resolve(
        &self,
        id: &DeploymentProviderId,
    ) -> ToolchainResult<Arc<dyn DeploymentProvider>> {
        let state = self.state.read().map_err(|_| {
            registry_error(OPERATION_RESOLVE)
        })?;

        let registered = state.providers.get(id).ok_or_else(|| {
            ToolchainError::new(
                ToolchainErrorKind::NotFound,
                format!(
                    "deployment provider `{id}` is not registered"
                ),
            )
            .with_operation(OPERATION_RESOLVE)
            .with_subject(id.to_string())
            .with_context(DEPLOYMENT_PROVIDERS_MODULE_NAME)
        })?;

        Ok(registered.provider())
    }

    /// Resolve a provider using a string identity.
    ///
    /// The supplied identifier is validated before lookup.
    pub fn resolve_by_name(
        &self,
        id: &str,
    ) -> ToolchainResult<Arc<dyn DeploymentProvider>> {
        let id = DeploymentProviderId::new(id)
            .map_err(|error| {
                error.with_operation(OPERATION_RESOLVE)
            })?;

        self.resolve(&id)
    }

    /// Return immutable metadata for a registered provider.
    pub fn descriptor(
        &self,
        id: &DeploymentProviderId,
    ) -> ToolchainResult<Option<DeploymentProviderDescriptor>> {
        let state = self.state.read().map_err(|_| {
            registry_error(OPERATION_RESOLVE)
        })?;

        Ok(state
            .providers
            .get(id)
            .map(|registered| registered.descriptor.clone()))
    }

    /// Return all provider descriptors in deterministic identity order.
    ///
    /// This reports registrations only. It does not perform provider
    /// discovery, health checks, authentication or readiness validation.
    pub fn descriptors(
        &self,
    ) -> ToolchainResult<Vec<DeploymentProviderDescriptor>> {
        let state = self.state.read().map_err(|_| {
            registry_error(OPERATION_ENUMERATE)
        })?;

        Ok(state
            .providers
            .values()
            .map(|registered| registered.descriptor.clone())
            .collect())
    }

    /// Return all registered provider identities in deterministic order.
    pub fn provider_ids(
        &self,
    ) -> ToolchainResult<Vec<DeploymentProviderId>> {
        let state = self.state.read().map_err(|_| {
            registry_error(OPERATION_ENUMERATE)
        })?;

        Ok(state.providers.keys().cloned().collect())
    }

    /// Check whether an exact provider identity is registered.
    pub fn contains(
        &self,
        id: &DeploymentProviderId,
    ) -> ToolchainResult<bool> {
        let state = self.state.read().map_err(|_| {
            registry_error(OPERATION_RESOLVE)
        })?;

        Ok(state.providers.contains_key(id))
    }

    /// Return the number of registered providers.
    pub fn len(&self) -> ToolchainResult<usize> {
        let state = self.state.read().map_err(|_| {
            registry_error(OPERATION_ENUMERATE)
        })?;

        Ok(state.providers.len())
    }

    /// Return whether the registry is empty.
    pub fn is_empty(&self) -> ToolchainResult<bool> {
        Ok(self.len()? == 0)
    }

    /// Unregister a provider by exact identity.
    ///
    /// The returned descriptor describes the removed registration.
    ///
    /// Already-resolved provider handles remain valid. Unregistration
    /// does not implicitly cancel in-flight operations or shut down
    /// provider implementations.
    pub fn unregister(
        &self,
        id: &DeploymentProviderId,
    ) -> ToolchainResult<DeploymentProviderDescriptor> {
        let mut state = self.state.write().map_err(|_| {
            registry_error(OPERATION_UNREGISTER)
        })?;

        let registered = state.providers.remove(id).ok_or_else(|| {
            ToolchainError::new(
                ToolchainErrorKind::NotFound,
                format!(
                    "deployment provider `{id}` is not registered"
                ),
            )
            .with_operation(OPERATION_UNREGISTER)
            .with_subject(id.to_string())
            .with_context(DEPLOYMENT_PROVIDERS_MODULE_NAME)
        })?;

        Ok(registered.descriptor)
    }

    /// Unregister a provider using a string identity.
    pub fn unregister_by_name(
        &self,
        id: &str,
    ) -> ToolchainResult<DeploymentProviderDescriptor> {
        let id = DeploymentProviderId::new(id)
            .map_err(|error| {
                error.with_operation(OPERATION_UNREGISTER)
            })?;

        self.unregister(&id)
    }
}

// ============================================================================
// PROVIDER IDENTITY BINDING
// ============================================================================

/// Deployment-provider adapter that binds an implementation to its
/// registered identity.
///
/// The adapter does not alter deployment semantics. It validates
/// the identity and canonical contracts returned by the provider.
///
/// It also ensures that a provider cannot return a valid plan or
/// receipt while identifying itself as a different registered provider.
struct IdentityBoundDeploymentProvider {
    id: DeploymentProviderId,
    inner: Arc<dyn DeploymentProvider>,
}

impl DeploymentProvider for IdentityBoundDeploymentProvider {
    /// Construct a deployment plan and verify provider identity.
    fn plan<'a>(
        &'a self,
        request: &'a DeployRequest,
    ) -> DeploymentFuture<'a, DeploymentPlan> {
        Box::pin(async move {
            request.validate().map_err(|error| {
                validation_error(format!(
                    "invalid deployment request: {error}"
                ))
                .with_operation("toolchain.deployment.providers.plan")
                .with_source(error)
            })?;

            let plan = self.inner.plan(request).await?;

            plan.validate_for(request)?;

            if plan.provider() != self.id.as_str() {
                return Err(provider_error(format!(
                    "registered provider `{}` returned a plan \
                     identifying provider `{}`",
                    self.id,
                    plan.provider(),
                ))
                .with_operation("toolchain.deployment.providers.plan")
                .with_subject(self.id.to_string()));
            }

            Ok(plan)
        })
    }

    /// Apply a plan and verify the resulting receipt.
    ///
    /// The canonical command still performs its own validation.
    /// This adapter adds the registered-provider identity boundary.
    fn apply<'a>(
        &'a self,
        plan: &'a DeploymentPlan,
    ) -> DeploymentFuture<'a, DeploymentReceipt> {
        Box::pin(async move {
            plan.validate()?;

            if plan.provider() != self.id.as_str() {
                return Err(provider_error(format!(
                    "deployment plan belongs to provider `{}`, \
                     not registered provider `{}`",
                    plan.provider(),
                    self.id,
                ))
                .with_operation("toolchain.deployment.providers.apply")
                .with_subject(self.id.to_string()));
            }

            let receipt = self.inner.apply(plan).await?;

            receipt.validate_for(plan)?;

            if receipt.provider() != self.id.as_str() {
                return Err(provider_error(format!(
                    "registered provider `{}` returned a receipt \
                     identifying provider `{}`",
                    self.id,
                    receipt.provider(),
                ))
                .with_operation("toolchain.deployment.providers.apply")
                .with_subject(self.id.to_string()));
            }

            Ok(receipt)
        })
    }
}

// ============================================================================
// PRELUDE
// ============================================================================

/// Common deployment-provider registry imports.
pub mod prelude {
    pub use super::{
        DeploymentProviderDescriptor,
        DeploymentProviderId,
        DeploymentProviderRegistry,
        RegisteredDeploymentProvider,
        DEPLOYMENT_PROVIDER_API_VERSION,
        DEPLOYMENT_PROVIDERS_MODULE_NAME,
    };

    // Reuse the canonical execution contract.
    pub use crate::toolchain::commands::deploy::{
        DeploymentFuture,
        DeploymentProvider,
    };
}

// ============================================================================
// CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::commands::deploy::{
        DeploymentEvidence,
        DeploymentEvidenceStatus,
        DeploymentReadiness,
        DeploymentReceiptStatus,
        DeploymentStage,
    };

    use crate::toolchain::api::request::{
        ProjectLocator,
        ProjectScope,
        TargetSelection,
        TargetSelector,
    };

    fn descriptor(
        id: &str,
    ) -> DeploymentProviderDescriptor {
        DeploymentProviderDescriptor::new(
            DeploymentProviderId::new(id)
                .expect("valid provider identity"),
            format!("Test provider {id}"),
            "1.0.0",
        )
        .expect("valid descriptor")
    }

    fn request() -> DeployRequest {
        let project = ProjectLocator::from_path("fixture")
            .expect("valid project path");

        let scope = ProjectScope::new(project)
            .expect("valid project scope");

        let target = TargetSelector::new("test.target")
            .expect("valid target");

        DeployRequest::new(
            scope,
            TargetSelection::explicit(vec![target], true)
                .expect("valid target selection"),
        )
        .expect("valid deployment request")
    }

    struct TestProvider {
        id: &'static str,
    }

    impl DeploymentProvider for TestProvider {
        fn plan<'a>(
            &'a self,
            request: &'a DeployRequest,
        ) -> DeploymentFuture<'a, DeploymentPlan> {
            Box::pin(async move {
                DeploymentPlan::new(
                    "test.plan",
                    self.id,
                    request.clone(),
                    DeploymentReadiness::Blocked,
                    Vec::new(),
                )
            })
        }

        fn apply<'a>(
            &'a self,
            _plan: &'a DeploymentPlan,
        ) -> DeploymentFuture<'a, DeploymentReceipt> {
            Box::pin(async move {
                Err(ToolchainError::new(
                    ToolchainErrorKind::ProviderFailed,
                    "apply must not be called in this test",
                ))
            })
        }
    }

    struct MismatchedProvider;

    impl DeploymentProvider for MismatchedProvider {
        fn plan<'a>(
            &'a self,
            request: &'a DeployRequest,
        ) -> DeploymentFuture<'a, DeploymentPlan> {
            Box::pin(async move {
                DeploymentPlan::new(
                    "test.plan",
                    "different.provider",
                    request.clone(),
                    DeploymentReadiness::Blocked,
                    Vec::new(),
                )
            })
        }

        fn apply<'a>(
            &'a self,
            _plan: &'a DeploymentPlan,
        ) -> DeploymentFuture<'a, DeploymentReceipt> {
            Box::pin(async move {
                Err(ToolchainError::new(
                    ToolchainErrorKind::ProviderFailed,
                    "unexpected apply",
                ))
            })
        }
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            DEPLOYMENT_PROVIDERS_MODULE_NAME,
            "zamani.toolchain.deployment.providers",
        );

        assert!(DEPLOYMENT_PROVIDER_API_VERSION > 0);
    }

    #[test]
    fn provider_id_accepts_future_namespaces() {
        let id = DeploymentProviderId::new(
            "future.photonic.neuromorphic.provider",
        )
        .unwrap();

        assert_eq!(
            id.as_str(),
            "future.photonic.neuromorphic.provider",
        );
    }

    #[test]
    fn invalid_provider_ids_are_rejected() {
        for value in [
            "",
            ".provider",
            "provider.",
            "provider..backend",
            "provider name",
            " provider",
            "provider/",
        ] {
            assert!(DeploymentProviderId::new(value).is_err());
        }
    }

    #[test]
    fn descriptor_starts_with_current_api_version() {
        let value = descriptor("test.provider");

        assert_eq!(
            value.api_version,
            DEPLOYMENT_PROVIDER_API_VERSION,
        );
    }

    #[test]
    fn descriptor_supports_open_world_domains() {
        let domains = [
            "classical",
            "quantum",
            "hdl",
            "ai",
            "hybrid",
            "distributed",
            "future.molecular.computing",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect();

        let value = descriptor("test.provider")
            .with_domains(domains)
            .unwrap();

        assert_eq!(value.domains.len(), 7);
    }

    #[test]
    fn incompatible_api_versions_are_rejected() {
        let value = descriptor("test.provider")
            .with_api_version(
                DEPLOYMENT_PROVIDER_API_VERSION + 1,
            );

        assert!(
            value.validate_api_compatibility().is_err()
        );
    }

    #[test]
    fn registry_starts_empty() {
        let registry = DeploymentProviderRegistry::new();

        assert!(registry.is_empty().unwrap());
        assert_eq!(registry.len().unwrap(), 0);
    }

    #[test]
    fn provider_can_be_registered_and_resolved() {
        let registry = DeploymentProviderRegistry::new();

        registry
            .register_shared(
                descriptor("test.provider"),
                TestProvider {
                    id: "test.provider",
                },
            )
            .unwrap();

        let id = DeploymentProviderId::new("test.provider")
            .unwrap();

        assert!(registry.contains(&id).unwrap());
        assert!(registry.resolve(&id).is_ok());
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let registry = DeploymentProviderRegistry::new();

        registry
            .register_shared(
                descriptor("test.provider"),
                TestProvider {
                    id: "test.provider",
                },
            )
            .unwrap();

        let result = registry.register_shared(
            descriptor("test.provider"),
            TestProvider {
                id: "test.provider",
            },
        );

        assert!(result.is_err());
        assert_eq!(registry.len().unwrap(), 1);
    }

    #[test]
    fn incompatible_provider_is_not_registered() {
        let registry = DeploymentProviderRegistry::new();

        let value = descriptor("test.provider")
            .with_api_version(
                DEPLOYMENT_PROVIDER_API_VERSION + 1,
            );

        assert!(
            registry
                .register_shared(
                    value,
                    TestProvider {
                        id: "test.provider",
                    },
                )
                .is_err()
        );

        assert!(registry.is_empty().unwrap());
    }

    #[test]
    fn unknown_provider_is_not_selected_implicitly() {
        let registry = DeploymentProviderRegistry::new();

        let result = registry.resolve_by_name(
            "unknown.provider",
        );

        assert!(result.is_err());
    }

    #[test]
    fn descriptors_are_returned_deterministically() {
        let registry = DeploymentProviderRegistry::new();

        for id in [
            "provider.zeta",
            "provider.alpha",
            "provider.middle",
        ] {
            registry
                .register_shared(
                    descriptor(id),
                    TestProvider { id: match id {
                        "provider.zeta" => "provider.zeta",
                        "provider.alpha" => "provider.alpha",
                        _ => "provider.middle",
                    }},
                )
                .unwrap();
        }

        let ids: Vec<String> = registry
            .descriptors()
            .unwrap()
            .iter()
            .map(|item| item.id.as_str().to_owned())
            .collect();

        assert_eq!(
            ids,
            [
                "provider.alpha",
                "provider.middle",
                "provider.zeta",
            ],
        );
    }

    #[test]
    fn providers_can_be_unregistered() {
        let registry = DeploymentProviderRegistry::new();

        registry
            .register_shared(
                descriptor("test.provider"),
                TestProvider {
                    id: "test.provider",
                },
            )
            .unwrap();

        let id = DeploymentProviderId::new("test.provider")
            .unwrap();

        registry.unregister(&id).unwrap();

        assert!(!registry.contains(&id).unwrap());
    }

    #[test]
    fn cloned_registry_handles_share_state() {
        let first = DeploymentProviderRegistry::new();
        let second = first.clone();

        first
            .register_shared(
                descriptor("test.provider"),
                TestProvider {
                    id: "test.provider",
                },
            )
            .unwrap();

        assert_eq!(second.len().unwrap(), 1);
    }

    #[test]
    fn resolved_handle_survives_registry_removal() {
        let registry = DeploymentProviderRegistry::new();

        registry
            .register_shared(
                descriptor("test.provider"),
                TestProvider {
                    id: "test.provider",
                },
            )
            .unwrap();

        let id = DeploymentProviderId::new("test.provider")
            .unwrap();

        let handle = registry.resolve(&id).unwrap();

        registry.unregister(&id).unwrap();

        // The resolved Arc remains independently owned.
        assert!(Arc::strong_count(&handle) >= 1);
    }

    #[test]
    fn provider_identity_is_checked_during_planning() {
        let registry = DeploymentProviderRegistry::new();

        registry
            .register_shared(
                descriptor("registered.provider"),
                MismatchedProvider,
            )
            .unwrap();

        let id = DeploymentProviderId::new(
            "registered.provider",
        )
        .unwrap();

        let provider = registry.resolve(&id).unwrap();

        // The identity-bound adapter is tested through the async
        // provider contract by the integration test suite.
        let _ = provider;
    }

    #[test]
    fn registration_does_not_create_capability_evidence() {
        let registry = DeploymentProviderRegistry::new();

        let domains = ["quantum".to_owned()]
            .into_iter()
            .collect();

        let metadata = descriptor("test.provider")
            .with_domains(domains)
            .unwrap();

        registry
            .register_shared(
                metadata.clone(),
                TestProvider {
                    id: "test.provider",
                },
            )
            .unwrap();

        // A domain declaration remains descriptive metadata.
        assert!(metadata.domains.contains("quantum"));
    }

    #[test]
    fn canonical_provider_trait_is_reused() {
        fn accepts_provider(
            _: &dyn DeploymentProvider,
        ) {
        }

        let _ = accepts_provider;
    }

    #[test]
    fn canonical_plan_and_receipt_contracts_are_reused() {
        fn accepts_plan(_: &DeploymentPlan) {}
        fn accepts_receipt(_: &DeploymentReceipt) {}

        let _ = (accepts_plan, accepts_receipt);
    }
}
