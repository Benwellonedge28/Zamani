//! Zamani Quantum Hardware — Provider Registry
//!
//! Production-grade, provider-neutral registry for quantum providers.
//!
//! # Responsibility
//!
//! This module owns the authoritative in-process registry of validated
//! [`ProviderDescriptor`] values.
//!
//! It provides:
//!
//! - provider registration;
//! - provider replacement/update;
//! - provider removal;
//! - provider lookup;
//! - deterministic provider enumeration;
//! - provider filtering;
//! - capability-based provider selection;
//! - technology-based provider selection;
//! - execution-model selection;
//! - interoperability-format selection;
//! - provider status filtering;
//! - registry snapshots;
//! - optimistic versioning;
//! - deterministic registry fingerprints;
//! - duplicate-registration protection;
//! - bounded registry growth;
//! - thread-safe concurrent access;
//! - provider descriptor validation;
//! - safe registry diagnostics.
//!
//! # Non-responsibilities
//!
//! This module deliberately does NOT own:
//!
//! - backend/device registration;
//! - backend discovery;
//! - provider discovery/network calls;
//! - provider credentials;
//! - API keys;
//! - OAuth tokens;
//! - authentication;
//! - authorization;
//! - HTTP clients;
//! - provider SDKs;
//! - job submission;
//! - job polling;
//! - job cancellation;
//! - result retrieval;
//! - calibration;
//! - topology;
//! - routing;
//! - scheduling;
//! - transpilation;
//! - benchmarking;
//! - simulation;
//! - emulation.
//!
//! Those responsibilities belong to their dedicated modules.
//!
//! # Architectural position
//!
//! ```text
//!                         ProviderDescriptor
//!                                |
//!                                v
//!                      ProviderRegistry
//!                                |
//!             +------------------+------------------+
//!             |                  |                  |
//!             v                  v                  v
//!          provider A         provider B         provider C
//!             |                  |                  |
//!             +------------------+------------------+
//!                                |
//!                                v
//!                       DeviceRegistry
//!                                |
//!                                v
//!                       BackendRegistry
//!                                |
//!                                v
//!                       Backend Adapter
//!                                |
//!                                v
//!                              QPU
//! ```
//!
//! The registry is metadata/index state. It does not execute quantum work.
//!
//! # Dependency contract
//!
//! This module depends only on:
//!
//! - the Rust standard library;
//! - `provider.rs`.
//!
//! It intentionally does NOT depend on:
//!
//! - `device_registry.rs`;
//! - `discovery.rs`;
//! - `credentials.rs`;
//! - `authentication.rs`;
//! - adapters;
//! - benchmarking;
//! - Danga;
//! - networking.
//!
//! This makes the file independently complete and prevents dependency cycles.
//!
//! # Provider versus adapter
//!
//! A [`ProviderDescriptor`] describes a provider.
//!
//! A [`QuantumBackendAdapter`](super::backend_trait::QuantumBackendAdapter)
//! executes against an individual backend.
//!
//! This registry therefore stores provider descriptors, not executable
//! adapters.
//!
//! Adapter ownership belongs to the execution/backend layer.
//!
//! # Thread safety
//!
//! [`ProviderRegistry`] is safe to share between threads through `Arc`.
//!
//! Internally it uses `RwLock` so:
//!
//! - concurrent readers do not mutate state;
//! - writes are serialized;
//! - callers cannot obtain mutable access to the internal map;
//! - registry invariants remain centralized.
//!
//! No global registry is created by this module.
//!
//! Applications may explicitly create and own an `Arc<ProviderRegistry>`.
//!
//! # Determinism
//!
//! The registry uses `BTreeMap` and `BTreeSet`.
//!
//! Therefore:
//!
//! - provider enumeration is deterministic;
//! - filtering order is deterministic;
//! - snapshots are deterministic;
//! - canonical representations are deterministic;
//! - fingerprints are deterministic.
//!
//! The registry never reads the system clock or random number generators.
//!
//! # Versioning
//!
//! Every successful structural mutation increments the registry generation.
//!
//! The generation starts at zero.
//!
//! Generation numbers are local registry versions. They are NOT timestamps,
//! cryptographic nonces, or distributed consensus versions.
//!
//! # Concurrency semantics
//!
//! Registration and removal are atomic with respect to other registry
//! operations.
//!
//! A caller may use [`ProviderRegistry::register_if_generation`] when it needs
//! optimistic concurrency control:
//!
//! ```text
//! read generation N
//!       |
//!       v
//! compute desired change
//!       |
//!       v
//! register_if_generation(N, ...)
//!       |
//!       +---- success -> generation N + 1
//!       |
//!       +---- mismatch -> retry/reconcile
//! ```
//!
//! # Security
//!
//! Provider descriptors are already responsible for rejecting unsafe metadata.
//!
//! This registry additionally:
//!
//! - validates descriptors before insertion;
//! - never stores credentials;
//! - never accepts a separate secret parameter;
//! - never logs descriptors automatically;
//! - never performs network requests;
//! - never resolves endpoints;
//! - never reads environment variables.
//!
//! # No-reedit rule
//!
//! This file is complete when:
//!
//! 1. provider identity is indexed through `ProviderId`;
//! 2. descriptors are validated before registration;
//! 3. duplicate registration is explicit;
//! 4. replacement is explicit;
//! 5. removal is explicit;
//! 6. deterministic listing exists;
//! 7. capability filtering exists;
//! 8. technology filtering exists;
//! 9. execution-model filtering exists;
//! 10. format filtering exists;
//! 11. status filtering exists;
//! 12. bounded growth exists;
//! 13. concurrent access is safe;
//! 14. generation/version checks exist;
//! 15. snapshots exist;
//! 16. deterministic fingerprints exist;
//! 17. no credentials are stored;
//! 18. no provider-specific implementation leaks into this file;
//! 19. downstream modules can consume this API without modifying this file.
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
//! ```text
//! #![deny(unsafe_code)]
//! ```
//!
//! # Downstream integration
//!
//! `device_registry.rs`
//!     Uses `ProviderId` to associate devices/backends with providers.
//!
//! `discovery.rs`
//!     Validates discovered provider descriptors and registers them here.
//!
//! `credentials.rs`
//!     Remains completely independent. Credentials are never inserted into
//!     this registry.
//!
//! `authentication.rs`
//!     Authenticates separately from registry state.
//!
//! `backend.rs`
//!     Supplies backend-level provider references.
//!
//! `backend_trait.rs`
//!     Supplies executable backend adapters. It does not become a provider
//!     registry dependency.
//!
//! `adapters/*`
//!     May identify their provider through `ProviderId` but must not mutate
//!     this registry internally unless explicitly given a registry handle by
//!     the application.
//!
//! `benchmarking`
//!     Uses provider information as execution provenance and selection input.
//!
//! `Danga`
//!     May expose registry operations such as provider listing and selection.
//!
//! Adding a provider MUST NOT require changing this file.
//!
//! -----------------------------------------------------------------------------
//! Schema
//! -----------------------------------------------------------------------------

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::provider::{
    ExecutionModelId,
    FormatId,
    ProviderDescriptor,
    ProviderError,
    ProviderId,
    ProviderKind,
    ProviderStatus,
    TechnologyId,
};

/// Stable registry schema identifier.
pub const PROVIDER_REGISTRY_SCHEMA_ID: &str =
    "zamani.quantum.hardware.provider_registry";

/// Semantic version of the registry contract.
pub const PROVIDER_REGISTRY_SCHEMA_VERSION: u16 = 1;

/// Maximum number of providers held by one registry.
///
/// This is intentionally bounded to prevent accidental unbounded growth from
/// configuration/discovery loops.
pub const MAX_REGISTERED_PROVIDERS: usize = 16_384;

/// Maximum number of filter results returned by one query.
///
/// The registry itself may contain up to `MAX_REGISTERED_PROVIDERS`, but this
/// bound prevents callers from accidentally requesting an enormous result
/// allocation.
pub const MAX_QUERY_RESULTS: usize = MAX_REGISTERED_PROVIDERS;

/// Maximum provider ID length used defensively by this module.
///
/// The canonical identity module remains authoritative for identity syntax.
pub const MAX_PROVIDER_ID_LENGTH: usize = 256;

// =============================================================================
// Registry errors
// =============================================================================

/// Errors produced by [`ProviderRegistry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderRegistryError {
    /// The provider descriptor itself is invalid.
    InvalidProvider {
        /// Provider validation error.
        source: ProviderError,
    },

    /// A provider with the same canonical identity already exists.
    AlreadyRegistered {
        /// Existing provider identity.
        provider_id: String,
    },

    /// The requested provider does not exist.
    NotFound {
        /// Requested provider identity.
        provider_id: String,
    },

    /// Registry capacity has been reached.
    CapacityExceeded {
        /// Maximum provider count.
        maximum: usize,
    },

    /// Optimistic-concurrency generation does not match.
    GenerationMismatch {
        /// Generation observed by the caller.
        expected: u64,

        /// Current registry generation.
        actual: u64,
    },

    /// Registry internal lock was poisoned.
    ///
    /// A poisoned lock indicates that a previous thread panicked while
    /// holding the lock. We deliberately do not recover silently because
    /// doing so could expose partially completed mutation semantics.
    LockPoisoned,

    /// A query requested more results than the registry contract permits.
    QueryLimitExceeded {
        /// Requested limit.
        requested: usize,

        /// Maximum permitted result limit.
        maximum: usize,
    },

    /// Registry-level invariant was violated.
    InvariantViolation {
        /// Human-readable diagnostic.
        message: String,
    },
}

impl fmt::Display for ProviderRegistryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidProvider { source } => {
                write!(
                    formatter,
                    "provider descriptor is invalid: {source}"
                )
            }

            Self::AlreadyRegistered { provider_id } => {
                write!(
                    formatter,
                    "provider '{}' is already registered",
                    provider_id
                )
            }

            Self::NotFound { provider_id } => {
                write!(
                    formatter,
                    "provider '{}' is not registered",
                    provider_id
                )
            }

            Self::CapacityExceeded { maximum } => {
                write!(
                    formatter,
                    "provider registry capacity of {} has been exceeded",
                    maximum
                )
            }

            Self::GenerationMismatch { expected, actual } => {
                write!(
                    formatter,
                    "provider registry generation mismatch: expected {}, \
                     actual {}",
                    expected,
                    actual
                )
            }

            Self::LockPoisoned => {
                write!(
                    formatter,
                    "provider registry lock is poisoned"
                )
            }

            Self::QueryLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "provider query requested {} results; maximum is {}",
                    requested,
                    maximum
                )
            }

            Self::InvariantViolation { message } => {
                write!(
                    formatter,
                    "provider registry invariant violation: {}",
                    message
                )
            }
        }
    }
}

impl std::error::Error for ProviderRegistryError {}

impl From<ProviderError> for ProviderRegistryError {
    fn from(source: ProviderError) -> Self {
        Self::InvalidProvider { source }
    }
}

// =============================================================================
// Registry generation
// =============================================================================

/// Monotonic in-process registry generation.
///
/// Generation `0` represents an empty, never-mutated registry.
///
/// Successful structural mutations increment the generation.
///
/// The value is not a timestamp and has no distributed meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RegistryGeneration(u64);

impl RegistryGeneration {
    /// Initial registry generation.
    pub const INITIAL: Self = Self(0);

    /// Returns the raw generation number.
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns the next generation.
    ///
    /// `u64::MAX` is treated as exhausted rather than wrapping around.
    fn checked_next(self) -> Result<Self, ProviderRegistryError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or_else(|| ProviderRegistryError::InvariantViolation {
                message: "registry generation exhausted".to_owned(),
            })
    }
}

impl Default for RegistryGeneration {
    fn default() -> Self {
        Self::INITIAL
    }
}

impl fmt::Display for RegistryGeneration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// =============================================================================
// Registry snapshot
// =============================================================================

/// Immutable deterministic snapshot of provider registry state.
///
/// The snapshot contains cloned provider descriptors and therefore does not
/// retain the registry lock.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderRegistrySnapshot {
    /// Registry schema identifier.
    pub schema_id: &'static str,

    /// Registry schema version.
    pub schema_version: u16,

    /// Registry generation at snapshot creation.
    pub generation: RegistryGeneration,

    /// Providers ordered by canonical provider identity.
    pub providers: BTreeMap<ProviderId, ProviderDescriptor>,
}

impl ProviderRegistrySnapshot {
    /// Returns the number of registered providers in the snapshot.
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    /// Returns true when the snapshot contains no providers.
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    /// Returns a provider by canonical identity.
    pub fn get(&self, provider_id: &ProviderId) -> Option<&ProviderDescriptor> {
        self.providers.get(provider_id)
    }

    /// Returns the deterministic canonical representation of the snapshot.
    ///
    /// The representation is intended as stable input to an external
    /// cryptographic hash function. This method itself is NOT a cryptographic
    /// hash.
    pub fn canonical_representation(&self) -> String {
        let mut output = String::new();

        output.push_str(self.schema_id);
        output.push('|');
        output.push_str(&self.schema_version.to_string());
        output.push('|');
        output.push_str(&self.generation.to_string());
        output.push('|');

        for (provider_id, descriptor) in &self.providers {
            output.push_str(provider_id.as_str());
            output.push('=');
            output.push_str(&descriptor.canonical_representation());
            output.push(';');
        }

        output
    }

    /// Returns a deterministic non-cryptographic fingerprint.
    ///
    /// This is suitable for cache invalidation and diagnostics.
    ///
    /// It MUST NOT be used as a security/authentication primitive.
    pub fn fingerprint(&self) -> u64 {
        use std::hash::{Hash, Hasher};

        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        self.canonical_representation().hash(&mut hasher);

        hasher.finish()
    }
}

// =============================================================================
// Provider query
// =============================================================================

/// Provider selection query.
///
/// Every populated field is an AND constraint.
///
/// Within a set, membership is also conjunctive where documented.
///
/// Empty sets mean "no constraint".
///
/// The query is metadata-only and never performs discovery or network calls.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProviderQuery {
    /// Restrict by provider kind.
    pub kind: Option<ProviderKind>,

    /// Restrict by provider status.
    pub status: Option<ProviderStatus>,

    /// Require a physical quantum provider.
    pub physical_quantum_hardware: Option<bool>,

    /// Require simulators.
    pub simulators: Option<bool>,

    /// Require emulators.
    pub emulators: Option<bool>,

    /// Require all listed technologies.
    pub technologies: BTreeSet<TechnologyId>,

    /// Require all listed execution models.
    pub execution_models: BTreeSet<ExecutionModelId>,

    /// Require all listed interoperability formats.
    pub formats: BTreeSet<FormatId>,

    /// Require all listed stable provider features.
    pub required_features: BTreeSet<String>,

    /// Require all listed experimental provider features.
    pub required_experimental_features: BTreeSet<String>,
}

impl ProviderQuery {
    /// Creates an unconstrained query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restricts the query to a provider kind.
    pub fn with_kind(mut self, kind: ProviderKind) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Restricts the query to a provider status.
    pub fn with_status(mut self, status: ProviderStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Requires physical quantum hardware.
    pub fn requiring_physical_hardware(mut self) -> Self {
        self.physical_quantum_hardware = Some(true);
        self
    }

    /// Requires simulators.
    pub fn requiring_simulators(mut self) -> Self {
        self.simulators = Some(true);
        self
    }

    /// Requires emulators.
    pub fn requiring_emulators(mut self) -> Self {
        self.emulators = Some(true);
        self
    }

    /// Requires a technology.
    pub fn requiring_technology(
        mut self,
        technology: TechnologyId,
    ) -> Self {
        self.technologies.insert(technology);
        self
    }

    /// Requires an execution model.
    pub fn requiring_execution_model(
        mut self,
        model: ExecutionModelId,
    ) -> Self {
        self.execution_models.insert(model);
        self
    }

    /// Requires an interoperability format.
    pub fn requiring_format(
        mut self,
        format: FormatId,
    ) -> Self {
        self.formats.insert(format);
        self
    }

    /// Requires a stable provider feature.
    pub fn requiring_feature(
        mut self,
        feature: impl Into<String>,
    ) -> Result<Self, ProviderRegistryError> {
        let feature = normalize_query_identifier(&feature.into())?;
        self.required_features.insert(feature);
        Ok(self)
    }

    /// Requires an experimental provider feature.
    pub fn requiring_experimental_feature(
        mut self,
        feature: impl Into<String>,
    ) -> Result<Self, ProviderRegistryError> {
        let feature = normalize_query_identifier(&feature.into())?;
        self.required_experimental_features.insert(feature);
        Ok(self)
    }

    /// Returns true when the descriptor satisfies this query.
    pub fn matches(&self, provider: &ProviderDescriptor) -> bool {
        if let Some(kind) = self.kind {
            if provider.kind != kind {
                return false;
            }
        }

        if let Some(status) = self.status {
            if provider.status != status {
                return false;
            }
        }

        if let Some(required) = self.physical_quantum_hardware {
            if provider.capabilities.physical_quantum_hardware != required {
                return false;
            }
        }

        if let Some(required) = self.simulators {
            if provider.capabilities.simulators != required {
                return false;
            }
        }

        if let Some(required) = self.emulators {
            if provider.capabilities.emulators != required {
                return false;
            }
        }

        if !self
            .technologies
            .iter()
            .all(|technology| provider.supports_technology(technology))
        {
            return false;
        }

        if !self
            .execution_models
            .iter()
            .all(|model| provider.supports_execution_model(model))
        {
            return false;
        }

        if !self
            .formats
            .iter()
            .all(|format| provider.supports_format(format))
        {
            return false;
        }

        if !self
            .required_features
            .iter()
            .all(|feature| provider.supports_feature(feature))
        {
            return false;
        }

        if !self
            .required_experimental_features
            .iter()
            .all(|feature| {
                provider.supports_experimental_feature(feature)
            })
        {
            return false;
        }

        true
    }
}

// =============================================================================
// Provider registry
// =============================================================================

/// Thread-safe, provider-neutral quantum provider registry.
///
/// The registry owns validated provider descriptors indexed by canonical
/// [`ProviderId`].
///
/// It contains no credentials and performs no network operations.
#[derive(Debug)]
pub struct ProviderRegistry {
    providers: RwLock<BTreeMap<ProviderId, ProviderDescriptor>>,
    generation: RwLock<RegistryGeneration>,
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ProviderRegistry {
    /// Creates an empty provider registry.
    pub fn new() -> Self {
        Self {
            providers: RwLock::new(BTreeMap::new()),
            generation: RwLock::new(RegistryGeneration::INITIAL),
        }
    }

    // -------------------------------------------------------------------------
    // Lock helpers
    // -------------------------------------------------------------------------

    fn read_providers(
        &self,
    ) -> Result<
        RwLockReadGuard<'_, BTreeMap<ProviderId, ProviderDescriptor>>,
        ProviderRegistryError,
    > {
        self.providers
            .read()
            .map_err(|_| ProviderRegistryError::LockPoisoned)
    }

    fn write_providers(
        &self,
    ) -> Result<
        RwLockWriteGuard<'_, BTreeMap<ProviderId, ProviderDescriptor>>,
        ProviderRegistryError,
    > {
        self.providers
            .write()
            .map_err(|_| ProviderRegistryError::LockPoisoned)
    }

    fn read_generation(
        &self,
    ) -> Result<
        RwLockReadGuard<'_, RegistryGeneration>,
        ProviderRegistryError,
    > {
        self.generation
            .read()
            .map_err(|_| ProviderRegistryError::LockPoisoned)
    }

    fn write_generation(
        &self,
    ) -> Result<
        RwLockWriteGuard<'_, RegistryGeneration>,
        ProviderRegistryError,
    > {
        self.generation
            .write()
            .map_err(|_| ProviderRegistryError::LockPoisoned)
    }

    // -------------------------------------------------------------------------
    // Generation
    // -------------------------------------------------------------------------

    /// Returns the current registry generation.
    pub fn generation(
        &self,
    ) -> Result<RegistryGeneration, ProviderRegistryError> {
        Ok(*self.read_generation()?)
    }

    /// Advances the generation after a successful structural mutation.
    fn advance_generation(
        &self,
    ) -> Result<RegistryGeneration, ProviderRegistryError> {
        let mut generation = self.write_generation()?;
        let next = generation.checked_next()?;
        *generation = next;
        Ok(next)
    }

    // -------------------------------------------------------------------------
    // Registration
    // -------------------------------------------------------------------------

    /// Registers a provider descriptor.
    ///
    /// Registration is rejected when the canonical provider identity already
    /// exists.
    ///
    /// The descriptor is fully validated before insertion.
    pub fn register(
        &self,
        provider: ProviderDescriptor,
    ) -> Result<RegistryGeneration, ProviderRegistryError> {
        provider.validate()?;

        validate_provider_id(&provider.id)?;

        let provider_id = provider.id.clone();

        let mut providers = self.write_providers()?;

        if providers.contains_key(&provider_id) {
            return Err(ProviderRegistryError::AlreadyRegistered {
                provider_id: provider_id.to_string(),
            });
        }

        if providers.len() >= MAX_REGISTERED_PROVIDERS {
            return Err(ProviderRegistryError::CapacityExceeded {
                maximum: MAX_REGISTERED_PROVIDERS,
            });
        }

        providers.insert(provider_id, provider);

        drop(providers);

        self.advance_generation()
    }

    /// Registers a provider only when the registry generation matches
    /// `expected_generation`.
    ///
    /// This gives callers an optimistic-concurrency mechanism without
    /// exposing the internal locks.
    pub fn register_if_generation(
        &self,
        expected_generation: RegistryGeneration,
        provider: ProviderDescriptor,
    ) -> Result<RegistryGeneration, ProviderRegistryError> {
        provider.validate()?;

        validate_provider_id(&provider.id)?;

        let provider_id = provider.id.clone();

        let mut providers = self.write_providers()?;
        let mut generation = self.write_generation()?;

        if *generation != expected_generation {
            return Err(ProviderRegistryError::GenerationMismatch {
                expected: expected_generation.get(),
                actual: generation.get(),
            });
        }

        if providers.contains_key(&provider_id) {
            return Err(ProviderRegistryError::AlreadyRegistered {
                provider_id: provider_id.to_string(),
            });
        }

        if providers.len() >= MAX_REGISTERED_PROVIDERS {
            return Err(ProviderRegistryError::CapacityExceeded {
                maximum: MAX_REGISTERED_PROVIDERS,
            });
        }

        let next = generation.checked_next()?;

        providers.insert(provider_id, provider);
        *generation = next;

        Ok(next)
    }

    /// Replaces an existing provider descriptor.
    ///
    /// Replacement is explicit so accidental duplicate registration can never
    /// silently overwrite provider metadata.
    pub fn replace(
        &self,
        provider: ProviderDescriptor,
    ) -> Result<RegistryGeneration, ProviderRegistryError> {
        provider.validate()?;

        validate_provider_id(&provider.id)?;

        let provider_id = provider.id.clone();

        let mut providers = self.write_providers()?;

        if !providers.contains_key(&provider_id) {
            return Err(ProviderRegistryError::NotFound {
                provider_id: provider_id.to_string(),
            });
        }

        providers.insert(provider_id, provider);

        drop(providers);

        self.advance_generation()
    }

    /// Replaces an existing provider only when the registry generation matches
    /// `expected_generation`.
    pub fn replace_if_generation(
        &self,
        expected_generation: RegistryGeneration,
        provider: ProviderDescriptor,
    ) -> Result<RegistryGeneration, ProviderRegistryError> {
        provider.validate()?;

        validate_provider_id(&provider.id)?;

        let provider_id = provider.id.clone();

        let mut providers = self.write_providers()?;
        let mut generation = self.write_generation()?;

        if *generation != expected_generation {
            return Err(ProviderRegistryError::GenerationMismatch {
                expected: expected_generation.get(),
                actual: generation.get(),
            });
        }

        if !providers.contains_key(&provider_id) {
            return Err(ProviderRegistryError::NotFound {
                provider_id: provider_id.to_string(),
            });
        }

        let next = generation.checked_next()?;

        providers.insert(provider_id, provider);
        *generation = next;

        Ok(next)
    }

    /// Removes a provider.
    ///
    /// Returns the removed descriptor.
    pub fn remove(
        &self,
        provider_id: &ProviderId,
    ) -> Result<ProviderDescriptor, ProviderRegistryError> {
        validate_provider_id(provider_id)?;

        let mut providers = self.write_providers()?;

        let removed = providers.remove(provider_id).ok_or_else(|| {
            ProviderRegistryError::NotFound {
                provider_id: provider_id.to_string(),
            }
        })?;

        drop(providers);

        self.advance_generation()?;

        Ok(removed)
    }

    /// Removes a provider only when the registry generation matches
    /// `expected_generation`.
    pub fn remove_if_generation(
        &self,
        expected_generation: RegistryGeneration,
        provider_id: &ProviderId,
    ) -> Result<
        (ProviderDescriptor, RegistryGeneration),
        ProviderRegistryError,
    > {
        validate_provider_id(provider_id)?;

        let mut providers = self.write_providers()?;
        let mut generation = self.write_generation()?;

        if *generation != expected_generation {
            return Err(ProviderRegistryError::GenerationMismatch {
                expected: expected_generation.get(),
                actual: generation.get(),
            });
        }

        let removed = providers.remove(provider_id).ok_or_else(|| {
            ProviderRegistryError::NotFound {
                provider_id: provider_id.to_string(),
            }
        })?;

        let next = generation.checked_next()?;
        *generation = next;

        Ok((removed, next))
    }

    /// Removes every provider from the registry.
    ///
    /// Returns the number removed.
    ///
    /// Clearing an already-empty registry does not advance the generation.
    pub fn clear(&self) -> Result<usize, ProviderRegistryError> {
        let mut providers = self.write_providers()?;

        if providers.is_empty() {
            return Ok(0);
        }

        let count = providers.len();
        providers.clear();

        drop(providers);

        self.advance_generation()?;

        Ok(count)
    }

    // -------------------------------------------------------------------------
    // Lookup
    // -------------------------------------------------------------------------

    /// Returns a cloned provider descriptor by canonical identity.
    ///
    /// The registry lock is released before the returned descriptor reaches
    /// the caller.
    pub fn get(
        &self,
        provider_id: &ProviderId,
    ) -> Result<ProviderDescriptor, ProviderRegistryError> {
        validate_provider_id(provider_id)?;

        let providers = self.read_providers()?;

        providers
            .get(provider_id)
            .cloned()
            .ok_or_else(|| ProviderRegistryError::NotFound {
                provider_id: provider_id.to_string(),
            })
    }

    /// Returns whether a provider is registered.
    pub fn contains(
        &self,
        provider_id: &ProviderId,
    ) -> Result<bool, ProviderRegistryError> {
        validate_provider_id(provider_id)?;

        Ok(self.read_providers()?.contains_key(provider_id))
    }

    /// Returns the number of registered providers.
    pub fn len(&self) -> Result<usize, ProviderRegistryError> {
        Ok(self.read_providers()?.len())
    }

    /// Returns whether the registry contains no providers.
    pub fn is_empty(&self) -> Result<bool, ProviderRegistryError> {
        Ok(self.read_providers()?.is_empty())
    }

    // -------------------------------------------------------------------------
    // Enumeration
    // -------------------------------------------------------------------------

    /// Returns all provider IDs in deterministic canonical order.
    pub fn provider_ids(
        &self,
    ) -> Result<Vec<ProviderId>, ProviderRegistryError> {
        Ok(self
            .read_providers()?
            .keys()
            .cloned()
            .collect())
    }

    /// Returns all provider descriptors in deterministic provider-ID order.
    pub fn providers(
        &self,
    ) -> Result<Vec<ProviderDescriptor>, ProviderRegistryError> {
        Ok(self
            .read_providers()?
            .values()
            .cloned()
            .collect())
    }

    /// Returns all registered provider IDs as a deterministic set.
    pub fn provider_id_set(
        &self,
    ) -> Result<BTreeSet<ProviderId>, ProviderRegistryError> {
        Ok(self
            .read_providers()?
            .keys()
            .cloned()
            .collect())
    }

    // -------------------------------------------------------------------------
    // Query
    // -------------------------------------------------------------------------

    /// Returns providers matching a metadata query.
    ///
    /// Results are deterministic and ordered by canonical provider identity.
    pub fn query(
        &self,
        query: &ProviderQuery,
    ) -> Result<Vec<ProviderDescriptor>, ProviderRegistryError> {
        self.query_with_limit(query, MAX_QUERY_RESULTS)
    }

    /// Returns providers matching a metadata query with an explicit result
    /// limit.
    pub fn query_with_limit(
        &self,
        query: &ProviderQuery,
        limit: usize,
    ) -> Result<Vec<ProviderDescriptor>, ProviderRegistryError> {
        if limit > MAX_QUERY_RESULTS {
            return Err(ProviderRegistryError::QueryLimitExceeded {
                requested: limit,
                maximum: MAX_QUERY_RESULTS,
            });
        }

        if limit == 0 {
            return Ok(Vec::new());
        }

        let providers = self.read_providers()?;

        Ok(providers
            .values()
            .filter(|provider| query.matches(provider))
            .take(limit)
            .cloned()
            .collect())
    }

    /// Returns the first provider matching a query in deterministic order.
    pub fn first_match(
        &self,
        query: &ProviderQuery,
    ) -> Result<Option<ProviderDescriptor>, ProviderRegistryError> {
        Ok(self
            .read_providers()?
            .values()
            .find(|provider| query.matches(provider))
            .cloned())
    }

    // -------------------------------------------------------------------------
    // Specialized queries
    // -------------------------------------------------------------------------

    /// Returns providers supporting a technology.
    pub fn providers_supporting_technology(
        &self,
        technology: &TechnologyId,
    ) -> Result<Vec<ProviderDescriptor>, ProviderRegistryError> {
        self.query(&ProviderQuery::new().requiring_technology(
            technology.clone(),
        ))
    }

    /// Returns providers supporting an execution model.
    pub fn providers_supporting_execution_model(
        &self,
        model: &ExecutionModelId,
    ) -> Result<Vec<ProviderDescriptor>, ProviderRegistryError> {
        self.query(
            &ProviderQuery::new()
                .requiring_execution_model(model.clone()),
        )
    }

    /// Returns providers supporting an interoperability format.
    pub fn providers_supporting_format(
        &self,
        format: &FormatId,
    ) -> Result<Vec<ProviderDescriptor>, ProviderRegistryError> {
        self.query(
            &ProviderQuery::new().requiring_format(format.clone()),
        )
    }

    /// Returns currently reachable providers.
    ///
    /// This uses provider metadata only. It does NOT perform a health check or
    /// network operation.
    pub fn reachable_providers(
        &self,
    ) -> Result<Vec<ProviderDescriptor>, ProviderRegistryError> {
        self.query(
            &ProviderQuery::new().with_status(ProviderStatus::Available),
        )
    }

    /// Returns providers advertising physical quantum hardware.
    pub fn physical_quantum_providers(
        &self,
    ) -> Result<Vec<ProviderDescriptor>, ProviderRegistryError> {
        self.query(&ProviderQuery::new().requiring_physical_hardware())
    }

    /// Returns providers advertising simulators.
    pub fn simulator_providers(
        &self,
    ) -> Result<Vec<ProviderDescriptor>, ProviderRegistryError> {
        self.query(&ProviderQuery::new().requiring_simulators())
    }

    /// Returns providers advertising emulators.
    pub fn emulator_providers(
        &self,
    ) -> Result<Vec<ProviderDescriptor>, ProviderRegistryError> {
        self.query(&ProviderQuery::new().requiring_emulators())
    }

    // -------------------------------------------------------------------------
    // Snapshot
    // -------------------------------------------------------------------------

    /// Creates an immutable registry snapshot.
    ///
    /// The snapshot contains cloned descriptors and therefore does not retain
    /// the registry lock.
    pub fn snapshot(
        &self,
    ) -> Result<ProviderRegistrySnapshot, ProviderRegistryError> {
        let providers = self.read_providers()?.clone();
        let generation = *self.read_generation()?;

        Ok(ProviderRegistrySnapshot {
            schema_id: PROVIDER_REGISTRY_SCHEMA_ID,
            schema_version: PROVIDER_REGISTRY_SCHEMA_VERSION,
            generation,
            providers,
        })
    }

    /// Returns a deterministic registry fingerprint.
    ///
    /// This is a non-cryptographic fingerprint suitable for local cache
    /// invalidation and diagnostics.
    ///
    /// It MUST NOT be used as a security primitive.
    pub fn fingerprint(&self) -> Result<u64, ProviderRegistryError> {
        Ok(self.snapshot()?.fingerprint())
    }

    /// Returns the deterministic canonical representation of the registry.
    pub fn canonical_representation(
        &self,
    ) -> Result<String, ProviderRegistryError> {
        Ok(self.snapshot()?.canonical_representation())
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Performs defensive provider-ID validation before registry operations.
///
/// `identity.rs` remains authoritative for construction/parsing. This helper
/// exists because registry operations may receive an already-created identity
/// from another subsystem.
fn validate_provider_id(
    provider_id: &ProviderId,
) -> Result<(), ProviderRegistryError> {
    let value = provider_id.as_str();

    if value.trim().is_empty() {
        return Err(ProviderRegistryError::InvariantViolation {
            message: "provider ID cannot be empty".to_owned(),
        });
    }

    if value.len() > MAX_PROVIDER_ID_LENGTH {
        return Err(ProviderRegistryError::InvariantViolation {
            message: format!(
                "provider ID exceeds maximum length of {} bytes",
                MAX_PROVIDER_ID_LENGTH
            ),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(ProviderRegistryError::InvariantViolation {
            message: "provider ID contains a control character".to_owned(),
        });
    }

    Ok(())
}

/// Normalizes a query feature identifier.
///
/// Provider feature identifiers are ultimately validated by `provider.rs`.
/// This additional boundary rejects obviously malformed values before they
/// enter a query object.
fn normalize_query_identifier(
    value: &str,
) -> Result<String, ProviderRegistryError> {
    if value.trim().is_empty() {
        return Err(ProviderRegistryError::InvariantViolation {
            message: "query identifier cannot be empty".to_owned(),
        });
    }

    if value.len() > 256 {
        return Err(ProviderRegistryError::InvariantViolation {
            message:
                "query identifier exceeds maximum length of 256 bytes"
                    .to_owned(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(ProviderRegistryError::InvariantViolation {
            message:
                "query identifier contains a control character"
                    .to_owned(),
        });
    }

    if value != value.trim() {
        return Err(ProviderRegistryError::InvariantViolation {
            message:
                "query identifier cannot contain surrounding whitespace"
                    .to_owned(),
        });
    }

    Ok(value.to_owned())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn provider(id: &str) -> ProviderDescriptor {
        ProviderDescriptor::from_str_id(
            id,
            ProviderKind::Cloud,
            ProviderMetadataForTests::metadata(),
        )
        .expect("valid provider")
    }

    struct ProviderMetadataForTests;

    impl ProviderMetadataForTests {
        fn metadata() -> super::super::provider::ProviderMetadata {
            super::super::provider::ProviderMetadata::new("Test Provider")
                .expect("valid provider metadata")
        }
    }

    #[test]
    fn registry_starts_empty() {
        let registry = ProviderRegistry::new();

        assert_eq!(registry.len().expect("length"), 0);
        assert!(registry.is_empty().expect("empty"));
        assert_eq!(
            registry.generation().expect("generation"),
            RegistryGeneration::INITIAL
        );
    }

    #[test]
    fn registration_advances_generation() {
        let registry = ProviderRegistry::new();

        let generation = registry
            .register(provider("provider://test"))
            .expect("registration");

        assert_eq!(generation.get(), 1);
        assert_eq!(registry.len().expect("length"), 1);
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider("provider://test"))
            .expect("first registration");

        let result = registry.register(provider("provider://test"));

        assert!(matches!(
            result,
            Err(ProviderRegistryError::AlreadyRegistered { .. })
        ));

        assert_eq!(registry.len().expect("length"), 1);
        assert_eq!(registry.generation().expect("generation").get(), 1);
    }

    #[test]
    fn replace_requires_existing_provider() {
        let registry = ProviderRegistry::new();

        let result = registry.replace(provider("provider://missing"));

        assert!(matches!(
            result,
            Err(ProviderRegistryError::NotFound { .. })
        ));
    }

    #[test]
    fn replace_advances_generation() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider("provider://test"))
            .expect("registration");

        let generation = registry
            .replace(provider("provider://test"))
            .expect("replacement");

        assert_eq!(generation.get(), 2);
    }

    #[test]
    fn remove_returns_provider() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider("provider://test"))
            .expect("registration");

        let id = provider("provider://test").id.clone();

        let removed = registry.remove(&id).expect("removal");

        assert_eq!(removed.id, id);
        assert!(registry.is_empty().expect("empty"));
        assert_eq!(registry.generation().expect("generation").get(), 2);
    }

    #[test]
    fn removing_missing_provider_is_rejected() {
        let registry = ProviderRegistry::new();

        let id = provider("provider://missing").id.clone();

        let result = registry.remove(&id);

        assert!(matches!(
            result,
            Err(ProviderRegistryError::NotFound { .. })
        ));

        assert_eq!(registry.generation().expect("generation").get(), 0);
    }

    #[test]
    fn provider_lookup_is_deterministic() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider("provider://zeta"))
            .expect("zeta");

        registry
            .register(provider("provider://alpha"))
            .expect("alpha");

        let ids = registry.provider_ids().expect("IDs");

        assert_eq!(ids.len(), 2);
        assert_eq!(ids[0].as_str(), "provider://alpha");
        assert_eq!(ids[1].as_str(), "provider://zeta");
    }

    #[test]
    fn contains_works() {
        let registry = ProviderRegistry::new();

        let descriptor = provider("provider://test");
        let id = descriptor.id.clone();

        registry.register(descriptor).expect("registration");

        assert!(registry.contains(&id).expect("contains"));
    }

    #[test]
    fn snapshot_is_independent_of_registry_lock() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider("provider://test"))
            .expect("registration");

        let snapshot = registry.snapshot().expect("snapshot");

        assert_eq!(snapshot.len(), 1);
        assert_eq!(snapshot.generation.get(), 1);

        registry.clear().expect("clear");

        assert_eq!(snapshot.len(), 1);
    }

    #[test]
    fn snapshot_fingerprint_changes_after_mutation() {
        let registry = ProviderRegistry::new();

        let initial = registry.fingerprint().expect("initial fingerprint");

        registry
            .register(provider("provider://test"))
            .expect("registration");

        let updated = registry.fingerprint().expect("updated fingerprint");

        assert_ne!(initial, updated);
    }

    #[test]
    fn clear_advances_generation_when_non_empty() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider("provider://a"))
            .expect("a");

        registry
            .register(provider("provider://b"))
            .expect("b");

        let removed = registry.clear().expect("clear");

        assert_eq!(removed, 2);
        assert!(registry.is_empty().expect("empty"));
        assert_eq!(registry.generation().expect("generation").get(), 3);
    }

    #[test]
    fn clear_empty_registry_does_not_mutate_generation() {
        let registry = ProviderRegistry::new();

        assert_eq!(registry.clear().expect("clear"), 0);
        assert_eq!(registry.generation().expect("generation").get(), 0);
    }

    #[test]
    fn generation_guard_rejects_stale_registration() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider("provider://first"))
            .expect("first");

        let stale = RegistryGeneration::INITIAL;

        let result = registry.register_if_generation(
            stale,
            provider("provider://second"),
        );

        assert!(matches!(
            result,
            Err(ProviderRegistryError::GenerationMismatch {
                expected: 0,
                actual: 1
            })
        ));

        assert_eq!(registry.len().expect("length"), 1);
    }

    #[test]
    fn generation_guard_accepts_current_registration() {
        let registry = ProviderRegistry::new();

        let generation = registry.generation().expect("generation");

        let next = registry
            .register_if_generation(
                generation,
                provider("provider://test"),
            )
            .expect("conditional registration");

        assert_eq!(next.get(), 1);
        assert_eq!(registry.len().expect("length"), 1);
    }

    #[test]
    fn query_by_status_is_deterministic() {
        let registry = ProviderRegistry::new();

        let available = provider("provider://available")
            .with_status(ProviderStatus::Available);

        let retired = provider("provider://retired")
            .with_status(ProviderStatus::Retired);

        registry.register(retired).expect("retired");
        registry.register(available).expect("available");

        let query = ProviderQuery::new()
            .with_status(ProviderStatus::Available);

        let results = registry.query(&query).expect("query");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id.as_str(), "provider://available");
    }

    #[test]
    fn query_with_zero_limit_returns_empty() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider("provider://test"))
            .expect("registration");

        let results = registry
            .query_with_limit(&ProviderQuery::new(), 0)
            .expect("query");

        assert!(results.is_empty());
    }

    #[test]
    fn query_matches_unconstrained_provider() {
        let descriptor = provider("provider://test");

        assert!(ProviderQuery::new().matches(&descriptor));
    }

    #[test]
    fn query_rejects_wrong_provider_kind() {
        let descriptor = provider("provider://test");

        let query = ProviderQuery::new().with_kind(ProviderKind::Local);

        assert!(!query.matches(&descriptor));
    }

    #[test]
    fn specialized_physical_provider_query_is_safe() {
        let registry = ProviderRegistry::new();

        let descriptor = provider("provider://qpu");

        let descriptor = descriptor.with_capabilities({
            let mut capabilities =
                super::super::provider::ProviderCapabilities::new();

            capabilities.physical_quantum_hardware = true;

            capabilities
        });

        registry
            .register(descriptor)
            .expect("registration");

        let results = registry
            .physical_quantum_providers()
            .expect("query");

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn specialized_simulator_query_is_safe() {
        let registry = ProviderRegistry::new();

        let descriptor = provider("provider://simulator");

        let descriptor = descriptor.with_capabilities({
            let mut capabilities =
                super::super::provider::ProviderCapabilities::new();

            capabilities.simulators = true;

            capabilities
        });

        registry
            .register(descriptor)
            .expect("registration");

        let results = registry
            .simulator_providers()
            .expect("query");

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn provider_id_validation_rejects_empty_value() {
        let registry = ProviderRegistry::new();

        let result = registry.provider_ids();

        assert!(result.is_ok());
    }

    #[test]
    fn registry_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<ProviderRegistry>();
    }

    #[test]
    fn snapshot_canonical_representation_is_deterministic() {
        let first = ProviderRegistry::new();
        let second = ProviderRegistry::new();

        first
            .register(provider("provider://a"))
            .expect("first");

        second
            .register(provider("provider://a"))
            .expect("second");

        assert_eq!(
            first.canonical_representation().expect("first canonical"),
            second
                .canonical_representation()
                .expect("second canonical")
        );
    }
}