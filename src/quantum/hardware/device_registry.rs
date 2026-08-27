//! Zamani Quantum Hardware — Device Registry
//!
//! Production-grade, provider-neutral registry for quantum execution devices.
//!
//! # Responsibility
//!
//! `DeviceRegistry` is the authoritative in-process index of discovered,
//! configured, or otherwise explicitly registered quantum execution targets.
//!
//! A registered device is represented by the canonical
//! `hardware::backend::QuantumBackend` descriptor. The registry does not
//! redefine backend identity, capabilities, limits, topology, calibration,
//! execution, provider authentication, or provider communication.
//!
//! The registry owns:
//!
//! - registration and replacement of complete backend snapshots;
//! - removal of devices;
//! - deterministic device lookup;
//! - provider-to-device indexing;
//! - status-aware queries;
//! - capability-aware queries;
//! - resource-aware queries;
//! - instruction-aware queries;
//! - region/metadata filtering;
//! - compatibility preselection;
//! - atomic snapshot replacement;
//! - registry generation/version tracking;
//! - deterministic enumeration;
//! - registry integrity validation;
//! - bounded registry capacity;
//! - concurrent read access;
//! - safe concurrent registration/replacement/removal;
//! - registry-level audit metadata;
//! - no-secret metadata validation;
//! - stable error reporting.
//!
//! # Explicit non-responsibilities
//!
//! This module does NOT:
//!
//! - perform provider network I/O;
//! - authenticate with providers;
//! - store credentials;
//! - store API keys;
//! - submit jobs;
//! - poll jobs;
//! - cancel jobs;
//! - retrieve results;
//! - acquire calibration;
//! - mutate topology;
//! - perform routing;
//! - perform scheduling;
//! - transpile circuits;
//! - execute benchmarks;
//! - perform statistical analysis;
//! - implement provider-specific APIs;
//! - implement provider discovery protocols.
//!
//! Those responsibilities belong to:
//!
//! ```text
//! authentication.rs
//! credentials.rs
//! discovery.rs
//! backend_trait.rs
//! execution.rs
//! routing.rs
//! scheduling.rs
//! benchmarking/
//! adapters/
//! ```
//!
//! # Architectural position
//!
//! ```text
//!                    Provider / Local Discovery
//!                              │
//!                              ▼
//!                         QuantumBackend
//!                              │
//!                              ▼
//!                       DeviceRegistry
//!                    ┌─────────┼─────────┐
//!                    │         │         │
//!                    ▼         ▼         ▼
//!                 lookup     filter    snapshot
//!                    │         │         │
//!                    └─────────┼─────────┘
//!                              ▼
//!                    Compatibility / Selection
//!                              │
//!                              ▼
//!                       Routing / Scheduling
//!                              │
//!                              ▼
//!                          Execution
//! ```
//!
//! # Critical architectural distinction
//!
//! A "device" in this registry is an execution-target snapshot, represented by
//! `QuantumBackend`.
//!
//! The registry does not create a second `Device` structure containing copies
//! of backend capabilities or topology. Doing so would create two competing
//! sources of truth.
//!
//! Therefore:
//!
//! ```text
//! DeviceRegistry
//!       │
//!       └── QuantumBackend
//!             ├── BackendMetadata
//!             ├── BackendCapabilities
//!             ├── BackendLimits
//!             └── HardwareTopology
//! ```
//!
//! `QuantumBackend` remains authoritative for backend semantics.
//!
//! # Snapshot semantics
//!
//! Registered backends are immutable snapshots from the registry's
//! perspective.
//!
//! A discovery/adapter subsystem should construct a fully validated
//! `QuantumBackend` and atomically replace the previous snapshot.
//!
//! This prevents readers from observing a partially updated backend such as:
//!
//! ```text
//! new capabilities
//! old topology
//! new metadata
//! old limits
//! ```
//!
//! Instead readers observe either the old complete snapshot or the new
//! complete snapshot.
//!
//! # Concurrency
//!
//! The registry uses `std::sync::RwLock` and `Arc` from the standard library.
//!
//! - many readers may query concurrently;
//! - writes are serialized;
//! - individual backend snapshots are immutable while held by readers;
//! - no async runtime is required;
//! - no global registry exists;
//! - callers own the registry instance.
//!
//! # Determinism
//!
//! All device identifiers are indexed in `BTreeMap`.
//!
//! Provider indexes use `BTreeSet`.
//!
//! Enumeration is therefore deterministic and independent of hash-map
//! iteration order.
//!
//! Query results are always returned in canonical backend-ID order.
//!
//! # Generation semantics
//!
//! Every successful mutation increments a monotonically increasing registry
//! generation.
//!
//! Generation `0` is the initial empty registry state.
//!
//! A generation does not identify a device. It identifies the registry state.
//!
//! This is useful for:
//!
//! - cache invalidation;
//! - compiler snapshots;
//! - discovery refreshes;
//! - reproducibility;
//! - diagnostics;
//! - tests;
//! - Danga;
//! - benchmarking.
//!
//! # Security
//!
//! This registry never stores credentials.
//!
//! Backend metadata itself is already responsible for rejecting secret-like
//! metadata. The registry additionally validates provider/backend identifiers
//! before indexing them.
//!
//! The registry never logs secrets because it has no credential API.
//!
//! # Rust compatibility
//!
//! Target:
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
//! This file is intentionally complete before the following modules are
//! integrated:
//!
//! - `provider_registry.rs` may use provider IDs to select devices;
//! - `discovery.rs` may replace/register complete snapshots;
//! - `compatibility.rs` may query candidate devices;
//! - `validation.rs` may validate workloads against selected devices;
//! - `execution.rs` may resolve a backend by ID;
//! - `backend_trait.rs` may execute a selected backend;
//! - adapters may populate the registry through discovery;
//! - benchmarking may obtain deterministic candidate devices;
//! - Danga may expose registry operations through its CLI/API.
//!
//! None of those consumers need to modify this file when implemented.
//!
//! # No-reedit rule
//!
//! This file is considered complete when:
//!
//! 1. registration is atomic;
//! 2. duplicate registration is rejected unless explicitly replaced;
//! 3. replacement is atomic;
//! 4. removal is deterministic;
//! 5. lookup is deterministic;
//! 6. provider indexing is maintained automatically;
//! 7. stale provider indexes cannot survive a mutation;
//! 8. queries are deterministic;
//! 9. registry capacity is bounded;
//! 10. generation tracking is deterministic;
//! 11. concurrent access is safe;
//! 12. backend snapshots cannot be partially observed;
//! 13. credentials cannot be registered through this API;
//! 14. registry integrity can be independently validated;
//! 15. no provider-specific implementation leaks into the registry;
//! 16. downstream modules can consume this API without modifying it.
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
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::backend::{
    BackendKind,
    BackendStatus,
    QuantumBackend,
};

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the device registry.
pub const DEVICE_REGISTRY_SCHEMA_ID: &str =
    "zamani.quantum.hardware.device_registry";

/// Semantic version of the device-registry contract.
pub const DEVICE_REGISTRY_SCHEMA_VERSION: u16 = 1;

/// Default maximum number of registered devices.
pub const DEFAULT_MAX_DEVICES: usize = 16_384;

/// Maximum permitted registry capacity.
pub const MAX_MAX_DEVICES: usize = 1_000_000;

/// Maximum provider identifier length accepted by the registry.
///
/// Backend metadata has its own authoritative validation. This limit protects
/// the registry's indexing layer from pathological inputs.
pub const MAX_PROVIDER_ID_LENGTH: usize = 512;

/// Maximum backend/device identifier length accepted by the registry.
pub const MAX_DEVICE_ID_LENGTH: usize = 512;

/// Maximum number of query results returned by a single bounded query.
pub const MAX_QUERY_RESULTS: usize = 1_000_000;

// =============================================================================
// Internal state
// =============================================================================

/// Internal mutable registry state.
///
/// This type is intentionally private. Callers interact only through
/// `DeviceRegistry`.
#[derive(Debug)]
struct RegistryState {
    /// Backend snapshots indexed by canonical backend ID.
    devices: BTreeMap<String, Arc<QuantumBackend>>,

    /// Reverse provider index:
    ///
    /// provider ID -> backend IDs.
    ///
    /// `BTreeSet` guarantees deterministic ordering.
    provider_index: BTreeMap<String, BTreeSet<String>>,

    /// Monotonically increasing registry state generation.
    generation: u64,
}

impl RegistryState {
    fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            provider_index: BTreeMap::new(),
            generation: 0,
        }
    }

    fn rebuild_provider_index(&mut self) -> Result<(), DeviceRegistryError> {
        let mut provider_index: BTreeMap<String, BTreeSet<String>> =
            BTreeMap::new();

        for (device_id, backend) in &self.devices {
            validate_device_id(device_id)?;
            validate_provider_id(backend.provider())?;

            provider_index
                .entry(backend.provider().to_owned())
                .or_default()
                .insert(device_id.clone());
        }

        self.provider_index = provider_index;

        Ok(())
    }

    fn increment_generation(&mut self) -> Result<u64, DeviceRegistryError> {
        self.generation = self
            .generation
            .checked_add(1)
            .ok_or(DeviceRegistryError::GenerationOverflow)?;

        Ok(self.generation)
    }
}

// =============================================================================
// Registry errors
// =============================================================================

/// Errors produced by device-registry operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceRegistryError {
    /// Registry capacity is invalid.
    InvalidCapacity {
        /// Requested capacity.
        capacity: usize,

        /// Maximum supported capacity.
        maximum: usize,
    },

    /// Device/backend ID is invalid.
    InvalidDeviceId {
        /// Invalid identifier.
        id: String,
    },

    /// Provider ID is invalid.
    InvalidProviderId {
        /// Invalid provider identifier.
        id: String,
    },

    /// Device ID supplied to registration does not match the backend ID.
    DeviceIdMismatch {
        /// Registry key.
        requested: String,

        /// Backend's canonical ID.
        backend: String,
    },

    /// Device already exists.
    AlreadyRegistered {
        /// Existing device ID.
        device_id: String,
    },

    /// Device was not found.
    NotFound {
        /// Requested device ID.
        device_id: String,
    },

    /// Provider was not found.
    ProviderNotFound {
        /// Requested provider ID.
        provider_id: String,
    },

    /// Registry lock could not be acquired for reading.
    ReadLockPoisoned,

    /// Registry lock could not be acquired for writing.
    WriteLockPoisoned,

    /// Registry generation counter overflowed.
    GenerationOverflow,

    /// Registry capacity was reached.
    CapacityExceeded {
        /// Current number of devices.
        current: usize,

        /// Maximum permitted devices.
        maximum: usize,
    },

    /// Backend descriptor failed registry validation.
    InvalidBackend {
        /// Device/backend ID.
        device_id: String,

        /// Human-readable reason.
        reason: String,
    },

    /// Internal registry index is inconsistent.
    IntegrityViolation {
        /// Human-readable reason.
        reason: String,
    },

    /// Query result would exceed the registry API safety bound.
    QueryResultLimitExceeded {
        /// Requested result limit.
        requested: usize,

        /// Maximum allowed result limit.
        maximum: usize,
    },
}

impl fmt::Display for DeviceRegistryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCapacity {
                capacity,
                maximum,
            } => write!(
                formatter,
                "invalid device-registry capacity {capacity}; maximum is {maximum}"
            ),

            Self::InvalidDeviceId { id } => {
                write!(formatter, "invalid quantum device ID '{id}'")
            }

            Self::InvalidProviderId { id } => {
                write!(formatter, "invalid quantum provider ID '{id}'")
            }

            Self::DeviceIdMismatch {
                requested,
                backend,
            } => write!(
                formatter,
                "registry device ID '{requested}' does not match backend ID '{backend}'"
            ),

            Self::AlreadyRegistered { device_id } => write!(
                formatter,
                "quantum device '{device_id}' is already registered"
            ),

            Self::NotFound { device_id } => write!(
                formatter,
                "quantum device '{device_id}' is not registered"
            ),

            Self::ProviderNotFound { provider_id } => write!(
                formatter,
                "quantum provider '{provider_id}' has no registered devices"
            ),

            Self::ReadLockPoisoned => {
                formatter.write_str("quantum device registry read lock is poisoned")
            }

            Self::WriteLockPoisoned => {
                formatter.write_str(
                    "quantum device registry write lock is poisoned",
                )
            }

            Self::GenerationOverflow => {
                formatter.write_str(
                    "quantum device registry generation counter overflowed",
                )
            }

            Self::CapacityExceeded { current, maximum } => write!(
                formatter,
                "quantum device registry capacity exceeded: {current} devices; maximum is {maximum}"
            ),

            Self::InvalidBackend { device_id, reason } => write!(
                formatter,
                "registered backend '{device_id}' is invalid: {reason}"
            ),

            Self::IntegrityViolation { reason } => write!(
                formatter,
                "quantum device registry integrity violation: {reason}"
            ),

            Self::QueryResultLimitExceeded {
                requested,
                maximum,
            } => write!(
                formatter,
                "query result limit {requested} exceeds maximum {maximum}"
            ),
        }
    }
}

impl Error for DeviceRegistryError {}

// =============================================================================
// Registry configuration
// =============================================================================

/// Immutable configuration for a device registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceRegistryConfig {
    /// Maximum number of devices that may be registered.
    pub max_devices: usize,
}

impl DeviceRegistryConfig {
    /// Creates a validated registry configuration.
    pub fn new(max_devices: usize) -> Result<Self, DeviceRegistryError> {
        if max_devices == 0 || max_devices > MAX_MAX_DEVICES {
            return Err(DeviceRegistryError::InvalidCapacity {
                capacity: max_devices,
                maximum: MAX_MAX_DEVICES,
            });
        }

        Ok(Self { max_devices })
    }

    /// Returns the production default configuration.
    pub const fn default_const() -> Self {
        Self {
            max_devices: DEFAULT_MAX_DEVICES,
        }
    }
}

impl Default for DeviceRegistryConfig {
    fn default() -> Self {
        Self::default_const()
    }
}

// =============================================================================
// Device selection query
// =============================================================================

/// Provider-neutral device selection query.
///
/// A query is a filter, not a reservation and not an execution request.
///
/// All specified predicates must match.
///
/// Results are returned in deterministic backend-ID order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DeviceQuery {
    /// Restrict to one provider.
    pub provider_id: Option<String>,

    /// Restrict to one backend kind.
    pub backend_kind: Option<BackendKind>,

    /// Restrict to one operational status.
    pub status: Option<BackendStatus>,

    /// Require at least this many physical quantum resources.
    pub minimum_qubits: Option<usize>,

    /// Require at least this many logical qubits.
    pub minimum_logical_qubits: Option<usize>,

    /// Require all listed stable capabilities.
    pub required_capabilities: BTreeSet<String>,

    /// Require all listed native instructions.
    pub required_instructions: BTreeSet<String>,

    /// Require a metadata property with this exact value.
    pub required_properties: BTreeMap<String, String>,

    /// Require a region with this exact value.
    pub region: Option<String>,

    /// Include unavailable/retired devices when true.
    ///
    /// If false, `status` remains authoritative when explicitly specified,
    /// while the default query excludes retired devices.
    pub include_retired: bool,

    /// Maximum number of results.
    pub limit: Option<usize>,
}

impl DeviceQuery {
    /// Creates an empty query that matches all registered non-retired
    /// devices.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restricts the query to a provider.
    pub fn with_provider(
        mut self,
        provider_id: impl Into<String>,
    ) -> Result<Self, DeviceRegistryError> {
        let provider_id = provider_id.into();

        validate_provider_id(&provider_id)?;

        self.provider_id = Some(provider_id);

        Ok(self)
    }

    /// Restricts the query to a backend kind.
    pub fn with_backend_kind(mut self, kind: BackendKind) -> Self {
        self.backend_kind = Some(kind);
        self
    }

    /// Restricts the query to an exact status.
    pub fn with_status(mut self, status: BackendStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Requires a minimum number of physical qubits/resources.
    pub fn with_minimum_qubits(mut self, count: usize) -> Self {
        self.minimum_qubits = Some(count);
        self
    }

    /// Requires a minimum number of logical qubits.
    pub fn with_minimum_logical_qubits(
        mut self,
        count: usize,
    ) -> Self {
        self.minimum_logical_qubits = Some(count);
        self
    }

    /// Requires one stable backend capability.
    pub fn require_capability(
        mut self,
        capability: impl Into<String>,
    ) -> Self {
        let capability = normalize_name(&capability.into());

        if !capability.is_empty() {
            self.required_capabilities.insert(capability);
        }

        self
    }

    /// Requires one native instruction.
    pub fn require_instruction(
        mut self,
        instruction: impl Into<String>,
    ) -> Self {
        let instruction = normalize_name(&instruction.into());

        if !instruction.is_empty() {
            self.required_instructions.insert(instruction);
        }

        self
    }

    /// Requires a non-secret backend metadata property.
    pub fn require_property(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, DeviceRegistryError> {
        let key = key.into();
        let value = value.into();

        validate_property(&key, &value)?;

        self.required_properties.insert(key, value);

        Ok(self)
    }

    /// Restricts the query to an exact region.
    pub fn with_region(
        mut self,
        region: impl Into<String>,
    ) -> Result<Self, DeviceRegistryError> {
        let region = region.into();

        if region.trim().is_empty() || region.chars().any(char::is_control) {
            return Err(DeviceRegistryError::InvalidBackend {
                device_id: "<query>".to_owned(),
                reason: "region cannot be empty or contain control characters"
                    .to_owned(),
            });
        }

        self.region = Some(region);

        Ok(self)
    }

    /// Includes retired devices.
    pub fn including_retired(mut self) -> Self {
        self.include_retired = true;
        self
    }

    /// Sets a maximum result count.
    pub fn with_limit(
        mut self,
        limit: usize,
    ) -> Result<Self, DeviceRegistryError> {
        validate_query_limit(limit)?;

        self.limit = Some(limit);

        Ok(self)
    }

    /// Validates query invariants.
    pub fn validate(&self) -> Result<(), DeviceRegistryError> {
        if let Some(provider_id) = &self.provider_id {
            validate_provider_id(provider_id)?;
        }

        if let Some(region) = &self.region {
            if region.trim().is_empty()
                || region.chars().any(char::is_control)
            {
                return Err(DeviceRegistryError::InvalidBackend {
                    device_id: "<query>".to_owned(),
                    reason: "region cannot be empty or contain control characters"
                        .to_owned(),
                });
            }
        }

        if let Some(limit) = self.limit {
            validate_query_limit(limit)?;
        }

        for key in &self.required_capabilities {
            validate_name(key, "capability")?;
        }

        for instruction in &self.required_instructions {
            validate_name(instruction, "instruction")?;
        }

        for (key, value) in &self.required_properties {
            validate_property(key, value)?;
        }

        Ok(())
    }
}

// =============================================================================
// Registry snapshot
// =============================================================================

/// Immutable snapshot of registry state metadata.
///
/// The actual backend snapshots remain owned by `DeviceRegistry`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceRegistrySnapshot {
    /// Registry schema identifier.
    pub schema_id: &'static str,

    /// Registry schema version.
    pub schema_version: u16,

    /// Current registry generation.
    pub generation: u64,

    /// Number of registered devices.
    pub device_count: usize,

    /// Configured maximum capacity.
    pub max_devices: usize,
}

// =============================================================================
// Device registry
// =============================================================================

/// Thread-safe registry of canonical quantum backend/device snapshots.
///
/// The registry owns no global state and performs no network I/O.
///
/// # Example
///
/// ```ignore
/// let registry = DeviceRegistry::new()?;
///
/// registry.register(backend)?;
///
/// let backend = registry.get("provider://example/device")?;
///
/// let devices = registry.query(
///     DeviceQuery::new()
///         .with_minimum_qubits(20)
///         .require_capability("measurement"),
/// )?;
/// # Ok::<(), DeviceRegistryError>(())
/// ```
///
/// # Atomicity
///
/// Registration, replacement and removal each update the device map and
/// provider reverse index under one write lock. A reader therefore cannot
/// observe a partially updated index.
#[derive(Debug)]
pub struct DeviceRegistry {
    config: DeviceRegistryConfig,

    state: RwLock<RegistryState>,
}

impl DeviceRegistry {
    /// Creates an empty registry using the production default configuration.
    pub fn new() -> Self {
        Self::with_config(DeviceRegistryConfig::default())
            .expect("default device registry configuration is valid")
    }

    /// Creates an empty registry with explicit capacity.
    pub fn with_config(
        config: DeviceRegistryConfig,
    ) -> Result<Self, DeviceRegistryError> {
        if config.max_devices == 0
            || config.max_devices > MAX_MAX_DEVICES
        {
            return Err(DeviceRegistryError::InvalidCapacity {
                capacity: config.max_devices,
                maximum: MAX_MAX_DEVICES,
            });
        }

        Ok(Self {
            config,
            state: RwLock::new(RegistryState::new()),
        })
    }

    /// Returns registry configuration.
    pub const fn config(&self) -> DeviceRegistryConfig {
        self.config
    }

    /// Returns the current registry generation.
    pub fn generation(&self) -> Result<u64, DeviceRegistryError> {
        Ok(self.read_state()?.generation)
    }

    /// Returns an immutable registry snapshot.
    pub fn snapshot(
        &self,
    ) -> Result<DeviceRegistrySnapshot, DeviceRegistryError> {
        let state = self.read_state()?;

        Ok(DeviceRegistrySnapshot {
            schema_id: DEVICE_REGISTRY_SCHEMA_ID,
            schema_version: DEVICE_REGISTRY_SCHEMA_VERSION,
            generation: state.generation,
            device_count: state.devices.len(),
            max_devices: self.config.max_devices,
        })
    }

    // =========================================================================
    // Registration
    // =========================================================================

    /// Registers a complete backend snapshot.
    ///
    /// Registration fails if the backend ID already exists.
    ///
    /// The backend is validated before it enters the registry.
    pub fn register(
        &self,
        backend: QuantumBackend,
    ) -> Result<u64, DeviceRegistryError> {
        validate_backend(&backend)?;

        let device_id = backend.id().to_owned();
        let provider_id = backend.provider().to_owned();

        let mut state = self.write_state()?;

        if state.devices.contains_key(&device_id) {
            return Err(DeviceRegistryError::AlreadyRegistered {
                device_id,
            });
        }

        if state.devices.len() >= self.config.max_devices {
            return Err(DeviceRegistryError::CapacityExceeded {
                current: state.devices.len(),
                maximum: self.config.max_devices,
            });
        }

        state
            .devices
            .insert(device_id.clone(), Arc::new(backend));

        state
            .provider_index
            .entry(provider_id)
            .or_default()
            .insert(device_id);

        state.increment_generation()
    }

    /// Atomically replaces an existing backend snapshot.
    ///
    /// Replacement is permitted only when the backend ID already exists.
    pub fn replace(
        &self,
        backend: QuantumBackend,
    ) -> Result<u64, DeviceRegistryError> {
        validate_backend(&backend)?;

        let device_id = backend.id().to_owned();
        let provider_id = backend.provider().to_owned();

        let mut state = self.write_state()?;

        if !state.devices.contains_key(&device_id) {
            return Err(DeviceRegistryError::NotFound { device_id });
        }

        let old_provider = state
            .devices
            .get(&device_id)
            .map(|backend| backend.provider().to_owned())
            .ok_or_else(|| DeviceRegistryError::NotFound {
                device_id: device_id.clone(),
            })?;

        state
            .devices
            .insert(device_id.clone(), Arc::new(backend));

        if old_provider != provider_id {
            remove_from_provider_index(
                &mut state.provider_index,
                &old_provider,
                &device_id,
            );

            state
                .provider_index
                .entry(provider_id)
                .or_default()
                .insert(device_id);
        }

        state.increment_generation()
    }

    /// Registers a backend if absent, otherwise atomically replaces it.
    ///
    /// This is the preferred operation for discovery refresh loops where the
    /// caller does not want a race between "check" and "insert/replace".
    pub fn upsert(
        &self,
        backend: QuantumBackend,
    ) -> Result<u64, DeviceRegistryError> {
        validate_backend(&backend)?;

        let device_id = backend.id().to_owned();
        let provider_id = backend.provider().to_owned();

        let mut state = self.write_state()?;

        let old_provider = state
            .devices
            .get(&device_id)
            .map(|existing| existing.provider().to_owned());

        if old_provider.is_none()
            && state.devices.len() >= self.config.max_devices
        {
            return Err(DeviceRegistryError::CapacityExceeded {
                current: state.devices.len(),
                maximum: self.config.max_devices,
            });
        }

        state
            .devices
            .insert(device_id.clone(), Arc::new(backend));

        if let Some(old_provider) = old_provider {
            if old_provider != provider_id {
                remove_from_provider_index(
                    &mut state.provider_index,
                    &old_provider,
                    &device_id,
                );
            }
        }

        state
            .provider_index
            .entry(provider_id)
            .or_default()
            .insert(device_id);

        state.increment_generation()
    }

    /// Removes a registered backend/device.
    ///
    /// Returns the removed immutable backend snapshot.
    pub fn remove(
        &self,
        device_id: &str,
    ) -> Result<Arc<QuantumBackend>, DeviceRegistryError> {
        validate_device_id(device_id)?;

        let mut state = self.write_state()?;

        let backend = state.devices.remove(device_id).ok_or_else(|| {
            DeviceRegistryError::NotFound {
                device_id: device_id.to_owned(),
            }
        })?;

        remove_from_provider_index(
            &mut state.provider_index,
            backend.provider(),
            device_id,
        );

        state.increment_generation()?;

        Ok(backend)
    }

    /// Removes all devices belonging to one provider.
    ///
    /// Returns the number of removed devices and the resulting generation.
    pub fn remove_provider(
        &self,
        provider_id: &str,
    ) -> Result<(usize, u64), DeviceRegistryError> {
        validate_provider_id(provider_id)?;

        let mut state = self.write_state()?;

        let device_ids = state
            .provider_index
            .get(provider_id)
            .cloned()
            .ok_or_else(|| DeviceRegistryError::ProviderNotFound {
                provider_id: provider_id.to_owned(),
            })?;

        let count = device_ids.len();

        for device_id in &device_ids {
            state.devices.remove(device_id);
        }

        state.provider_index.remove(provider_id);

        let generation = state.increment_generation()?;

        Ok((count, generation))
    }

    /// Clears the registry.
    ///
    /// This operation is intentionally explicit and cannot accidentally be
    /// triggered by a discovery refresh.
    pub fn clear(&self) -> Result<u64, DeviceRegistryError> {
        let mut state = self.write_state()?;

        if state.devices.is_empty() {
            return Ok(state.generation);
        }

        state.devices.clear();
        state.provider_index.clear();

        state.increment_generation()
    }

    // =========================================================================
    // Lookup
    // =========================================================================

    /// Returns an immutable backend snapshot by ID.
    pub fn get(
        &self,
        device_id: &str,
    ) -> Result<Arc<QuantumBackend>, DeviceRegistryError> {
        validate_device_id(device_id)?;

        let state = self.read_state()?;

        state
            .devices
            .get(device_id)
            .cloned()
            .ok_or_else(|| DeviceRegistryError::NotFound {
                device_id: device_id.to_owned(),
            })
    }

    /// Returns an immutable backend snapshot when present.
    pub fn get_optional(
        &self,
        device_id: &str,
    ) -> Result<Option<Arc<QuantumBackend>>, DeviceRegistryError> {
        validate_device_id(device_id)?;

        Ok(self.read_state()?.devices.get(device_id).cloned())
    }

    /// Returns whether a device is registered.
    pub fn contains(
        &self,
        device_id: &str,
    ) -> Result<bool, DeviceRegistryError> {
        validate_device_id(device_id)?;

        Ok(self.read_state()?.devices.contains_key(device_id))
    }

    /// Returns the number of registered devices.
    pub fn len(&self) -> Result<usize, DeviceRegistryError> {
        Ok(self.read_state()?.devices.len())
    }

    /// Returns true when no devices are registered.
    pub fn is_empty(&self) -> Result<bool, DeviceRegistryError> {
        Ok(self.read_state()?.devices.is_empty())
    }

    /// Returns all registered device IDs in deterministic order.
    pub fn device_ids(&self) -> Result<Vec<String>, DeviceRegistryError> {
        Ok(self.read_state()?.devices.keys().cloned().collect())
    }

    /// Returns all registered backend snapshots in deterministic order.
    pub fn devices(
        &self,
    ) -> Result<Vec<Arc<QuantumBackend>>, DeviceRegistryError> {
        Ok(self
            .read_state()?
            .devices
            .values()
            .cloned()
            .collect())
    }

    // =========================================================================
    // Provider indexing
    // =========================================================================

    /// Returns all device IDs belonging to a provider.
    pub fn device_ids_for_provider(
        &self,
        provider_id: &str,
    ) -> Result<Vec<String>, DeviceRegistryError> {
        validate_provider_id(provider_id)?;

        let state = self.read_state()?;

        match state.provider_index.get(provider_id) {
            Some(device_ids) => Ok(device_ids.iter().cloned().collect()),

            None => Err(DeviceRegistryError::ProviderNotFound {
                provider_id: provider_id.to_owned(),
            }),
        }
    }

    /// Returns all devices belonging to a provider.
    pub fn devices_for_provider(
        &self,
        provider_id: &str,
    ) -> Result<Vec<Arc<QuantumBackend>>, DeviceRegistryError> {
        let device_ids = self.device_ids_for_provider(provider_id)?;

        let state = self.read_state()?;

        let mut result = Vec::with_capacity(device_ids.len());

        for device_id in device_ids {
            let backend = state.devices.get(&device_id).ok_or_else(|| {
                DeviceRegistryError::IntegrityViolation {
                    reason: format!(
                        "provider index references missing device '{device_id}'"
                    ),
                }
            })?;

            result.push(Arc::clone(backend));
        }

        Ok(result)
    }

    /// Returns all provider IDs represented in the registry.
    pub fn provider_ids(&self) -> Result<Vec<String>, DeviceRegistryError> {
        Ok(self
            .read_state()?
            .provider_index
            .keys()
            .cloned()
            .collect())
    }

    /// Returns the number of registered devices for a provider.
    pub fn provider_device_count(
        &self,
        provider_id: &str,
    ) -> Result<usize, DeviceRegistryError> {
        validate_provider_id(provider_id)?;

        Ok(self
            .read_state()?
            .provider_index
            .get(provider_id)
            .map(BTreeSet::len)
            .unwrap_or(0))
    }

    // =========================================================================
    // Query
    // =========================================================================

    /// Finds devices matching all query predicates.
    ///
    /// The returned list is deterministic and bounded by `query.limit`.
    pub fn query(
        &self,
        query: DeviceQuery,
    ) -> Result<Vec<Arc<QuantumBackend>>, DeviceRegistryError> {
        query.validate()?;

        let state = self.read_state()?;

        let mut result = Vec::new();

        let candidates: Box<dyn Iterator<Item = (&String, &Arc<QuantumBackend>)>> =
            if let Some(provider_id) = &query.provider_id {
                match state.provider_index.get(provider_id) {
                    Some(ids) => Box::new(
                        ids.iter().filter_map(|device_id| {
                            state.devices.get_key_value(device_id)
                        }),
                    ),

                    None => return Ok(result),
                }
            } else {
                Box::new(state.devices.iter())
            };

        for (_, backend) in candidates {
            if !matches_query(backend.as_ref(), &query) {
                continue;
            }

            result.push(Arc::clone(backend));

            if let Some(limit) = query.limit {
                if result.len() >= limit {
                    break;
                }
            }
        }

        Ok(result)
    }

    /// Returns the first deterministic device matching a query.
    pub fn find_one(
        &self,
        query: DeviceQuery,
    ) -> Result<Option<Arc<QuantumBackend>>, DeviceRegistryError> {
        let mut query = query;
        query.limit = Some(1);

        Ok(self.query(query)?.into_iter().next())
    }

    /// Finds all available devices with at least the requested number of
    /// qubits and required capabilities.
    pub fn find_compatible_candidates(
        &self,
        minimum_qubits: usize,
        required_capabilities: &[&str],
    ) -> Result<Vec<Arc<QuantumBackend>>, DeviceRegistryError> {
        let mut query =
            DeviceQuery::new().with_minimum_qubits(minimum_qubits);

        for capability in required_capabilities {
            query = query.require_capability(*capability);
        }

        self.query(query)
    }

    // =========================================================================
    // Status helpers
    // =========================================================================

    /// Returns all currently usable devices.
    ///
    /// `Available` and `Degraded` are considered usable according to the
    /// backend status contract. Degraded devices remain visible so higher
    /// layers can make an explicit policy decision.
    pub fn available_devices(
        &self,
    ) -> Result<Vec<Arc<QuantumBackend>>, DeviceRegistryError> {
        self.query(
            DeviceQuery::new().with_status(BackendStatus::Available),
        )
    }

    /// Returns all operational devices, including busy, maintenance and
    /// degraded targets.
    pub fn operational_devices(
        &self,
    ) -> Result<Vec<Arc<QuantumBackend>>, DeviceRegistryError> {
        let state = self.read_state()?;

        Ok(state
            .devices
            .values()
            .filter(|backend| backend.status().is_operational())
            .cloned()
            .collect())
    }

    /// Returns all degraded devices.
    pub fn degraded_devices(
        &self,
    ) -> Result<Vec<Arc<QuantumBackend>>, DeviceRegistryError> {
        self.query(
            DeviceQuery::new().with_status(BackendStatus::Degraded),
        )
    }

    /// Returns all retired devices.
    pub fn retired_devices(
        &self,
    ) -> Result<Vec<Arc<QuantumBackend>>, DeviceRegistryError> {
        self.query(
            DeviceQuery::new()
                .with_status(BackendStatus::Retired)
                .including_retired(),
        )
    }

    // =========================================================================
    // Capability helpers
    // =========================================================================

    /// Returns devices supporting one stable capability.
    pub fn devices_supporting_capability(
        &self,
        capability: &str,
    ) -> Result<Vec<Arc<QuantumBackend>>, DeviceRegistryError> {
        let capability = normalize_name(capability);

        validate_name(&capability, "capability")?;

        self.query(DeviceQuery::new().require_capability(capability))
    }

    /// Returns devices supporting one native instruction.
    pub fn devices_supporting_instruction(
        &self,
        instruction: &str,
    ) -> Result<Vec<Arc<QuantumBackend>>, DeviceRegistryError> {
        let instruction = normalize_name(instruction);

        validate_name(&instruction, "instruction")?;

        self.query(DeviceQuery::new().require_instruction(instruction))
    }

    // =========================================================================
    // Integrity
    // =========================================================================

    /// Validates all registry invariants.
    ///
    /// This performs a full consistency check between the primary device map
    /// and the provider reverse index.
    pub fn validate_integrity(&self) -> Result<(), DeviceRegistryError> {
        let state = self.read_state()?;

        if state.devices.len() > self.config.max_devices {
            return Err(DeviceRegistryError::IntegrityViolation {
                reason: format!(
                    "device count {} exceeds configured capacity {}",
                    state.devices.len(),
                    self.config.max_devices
                ),
            });
        }

        let mut expected_provider_index: BTreeMap<
            String,
            BTreeSet<String>,
        > = BTreeMap::new();

        for (device_id, backend) in &state.devices {
            validate_backend(backend)?;

            if backend.id() != device_id {
                return Err(DeviceRegistryError::IntegrityViolation {
                    reason: format!(
                        "device map key '{}' does not match backend ID '{}'",
                        device_id,
                        backend.id()
                    ),
                });
            }

            expected_provider_index
                .entry(backend.provider().to_owned())
                .or_default()
                .insert(device_id.clone());
        }

        if expected_provider_index != state.provider_index {
            return Err(DeviceRegistryError::IntegrityViolation {
                reason: "provider reverse index does not match device map"
                    .to_owned(),
            });
        }

        for (provider_id, device_ids) in &state.provider_index {
            if device_ids.is_empty() {
                return Err(DeviceRegistryError::IntegrityViolation {
                    reason: format!(
                        "provider index contains empty provider '{provider_id}'"
                    ),
                });
            }

            for device_id in device_ids {
                let backend =
                    state.devices.get(device_id).ok_or_else(|| {
                        DeviceRegistryError::IntegrityViolation {
                            reason: format!(
                                "provider '{provider_id}' references missing device '{device_id}'"
                            ),
                        }
                    })?;

                if backend.provider() != provider_id {
                    return Err(DeviceRegistryError::IntegrityViolation {
                        reason: format!(
                            "provider index says '{}' owns '{}' but backend reports '{}'",
                            provider_id,
                            device_id,
                            backend.provider()
                        ),
                    });
                }
            }
        }

        Ok(())
    }

    /// Creates a complete deterministic snapshot of all registered backend
    /// references.
    ///
    /// The returned `Arc`s keep their backend snapshots alive even if the
    /// registry is subsequently modified.
    pub fn snapshot_devices(
        &self,
    ) -> Result<DeviceRegistryDeviceSnapshot, DeviceRegistryError> {
        let state = self.read_state()?;

        Ok(DeviceRegistryDeviceSnapshot {
            generation: state.generation,
            devices: state.devices.values().cloned().collect(),
        })
    }

    // =========================================================================
    // Lock helpers
    // =========================================================================

    fn read_state(
        &self,
    ) -> Result<RwLockReadGuard<'_, RegistryState>, DeviceRegistryError> {
        self.state
            .read()
            .map_err(|_| DeviceRegistryError::ReadLockPoisoned)
    }

    fn write_state(
        &self,
    ) -> Result<RwLockWriteGuard<'_, RegistryState>, DeviceRegistryError> {
        self.state
            .write()
            .map_err(|_| DeviceRegistryError::WriteLockPoisoned)
    }
}

impl Default for DeviceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Immutable device snapshot
// =============================================================================

/// Immutable collection of backend snapshots captured at one registry
/// generation.
#[derive(Debug, Clone)]
pub struct DeviceRegistryDeviceSnapshot {
    /// Registry generation from which the snapshot was taken.
    pub generation: u64,

    /// Deterministically ordered backend snapshots.
    pub devices: Vec<Arc<QuantumBackend>>,
}

impl DeviceRegistryDeviceSnapshot {
    /// Number of devices in the snapshot.
    pub fn len(&self) -> usize {
        self.devices.len()
    }

    /// Returns true when the snapshot contains no devices.
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Finds a device by canonical ID inside this immutable snapshot.
    pub fn get(&self, device_id: &str) -> Option<Arc<QuantumBackend>> {
        self.devices
            .binary_search_by(|backend| {
                backend.id().cmp(device_id)
            })
            .ok()
            .map(|index| Arc::clone(&self.devices[index]))
    }
}

// =============================================================================
// Matching
// =============================================================================

fn matches_query(
    backend: &QuantumBackend,
    query: &DeviceQuery,
) -> bool {
    if !query.include_retired
        && query.status.is_none()
        && backend.status() == BackendStatus::Retired
    {
        return false;
    }

    if let Some(provider_id) = &query.provider_id {
        if backend.provider() != provider_id {
            return false;
        }
    }

    if let Some(kind) = query.backend_kind {
        if backend.kind() != kind {
            return false;
        }
    }

    if let Some(status) = query.status {
        if backend.status() != status {
            return false;
        }
    }

    if let Some(minimum_qubits) = query.minimum_qubits {
        if backend.qubit_count() < minimum_qubits {
            return false;
        }
    }

    if let Some(minimum_logical_qubits) =
        query.minimum_logical_qubits
    {
        if backend
            .limits
            .max_logical_qubits
            != 0
            && backend.limits.max_logical_qubits
                < minimum_logical_qubits
        {
            return false;
        }

        if !backend.capabilities.logical_qubits
            && minimum_logical_qubits > 0
        {
            return false;
        }
    }

    for capability in &query.required_capabilities {
        if !backend.capabilities.supports_capability(capability) {
            return false;
        }
    }

    for instruction in &query.required_instructions {
        if !backend.capabilities.supports_gate(instruction) {
            return false;
        }
    }

    for (key, expected_value) in &query.required_properties {
        match backend.metadata.properties.get(key) {
            Some(actual_value) if actual_value == expected_value => {}

            _ => return false,
        }
    }

    if let Some(region) = &query.region {
        match &backend.metadata.region {
            Some(actual_region) if actual_region == region => {}

            _ => return false,
        }
    }

    true
}

// =============================================================================
// Validation
// =============================================================================

fn validate_backend(
    backend: &QuantumBackend,
) -> Result<(), DeviceRegistryError> {
    validate_device_id(backend.id())?;
    validate_provider_id(backend.provider())?;

    if backend.metadata.name.trim().is_empty() {
        return Err(DeviceRegistryError::InvalidBackend {
            device_id: backend.id().to_owned(),
            reason: "backend name cannot be empty".to_owned(),
        });
    }

    if backend.qubit_count() == 0 {
        return Err(DeviceRegistryError::InvalidBackend {
            device_id: backend.id().to_owned(),
            reason: "backend topology must contain at least one resource"
                .to_owned(),
        });
    }

    backend
        .topology()
        .validate()
        .map_err(|error| DeviceRegistryError::InvalidBackend {
            device_id: backend.id().to_owned(),
            reason: error.to_string(),
        })?;

    Ok(())
}

fn validate_device_id(
    value: &str,
) -> Result<(), DeviceRegistryError> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().count() > MAX_DEVICE_ID_LENGTH
        || value.chars().any(char::is_control)
    {
        return Err(DeviceRegistryError::InvalidDeviceId {
            id: value.to_owned(),
        });
    }

    Ok(())
}

fn validate_provider_id(
    value: &str,
) -> Result<(), DeviceRegistryError> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().count() > MAX_PROVIDER_ID_LENGTH
        || value.chars().any(char::is_control)
    {
        return Err(DeviceRegistryError::InvalidProviderId {
            id: value.to_owned(),
        });
    }

    Ok(())
}

fn validate_name(
    value: &str,
    field: &'static str,
) -> Result<(), DeviceRegistryError> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(DeviceRegistryError::InvalidBackend {
            device_id: "<query>".to_owned(),
            reason: format!("{field} identifier is invalid"),
        });
    }

    Ok(())
}

fn validate_property(
    key: &str,
    value: &str,
) -> Result<(), DeviceRegistryError> {
    validate_name(key, "property key")?;

    if value.trim().is_empty() || value.chars().any(char::is_control) {
        return Err(DeviceRegistryError::InvalidBackend {
            device_id: "<query>".to_owned(),
            reason: "property value cannot be empty or contain control characters"
                .to_owned(),
        });
    }

    Ok(())
}

fn validate_query_limit(
    limit: usize,
) -> Result<(), DeviceRegistryError> {
    if limit == 0 || limit > MAX_QUERY_RESULTS {
        return Err(DeviceRegistryError::QueryResultLimitExceeded {
            requested: limit,
            maximum: MAX_QUERY_RESULTS,
        });
    }

    Ok(())
}

fn normalize_name(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn remove_from_provider_index(
    provider_index: &mut BTreeMap<String, BTreeSet<String>>,
    provider_id: &str,
    device_id: &str,
) {
    let should_remove_provider =
        if let Some(device_ids) = provider_index.get_mut(provider_id) {
            device_ids.remove(device_id);
            device_ids.is_empty()
        } else {
            false
        };

    if should_remove_provider {
        provider_index.remove(provider_id);
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn registry() -> DeviceRegistry {
        DeviceRegistry::new()
    }

    #[test]
    fn default_registry_is_empty() {
        let registry = registry();

        assert!(registry.is_empty().unwrap());
        assert_eq!(registry.len().unwrap(), 0);
        assert_eq!(registry.generation().unwrap(), 0);
    }

    #[test]
    fn configuration_rejects_zero_capacity() {
        let result = DeviceRegistryConfig::new(0);

        assert!(matches!(
            result,
            Err(DeviceRegistryError::InvalidCapacity {
                capacity: 0,
                ..
            })
        ));
    }

    #[test]
    fn configuration_rejects_excessive_capacity() {
        let result =
            DeviceRegistryConfig::new(MAX_MAX_DEVICES + 1);

        assert!(matches!(
            result,
            Err(DeviceRegistryError::InvalidCapacity { .. })
        ));
    }

    #[test]
    fn device_id_validation_rejects_empty() {
        let result = validate_device_id("");

        assert!(matches!(
            result,
            Err(DeviceRegistryError::InvalidDeviceId { .. })
        ));
    }

    #[test]
    fn provider_id_validation_rejects_empty() {
        let result = validate_provider_id("");

        assert!(matches!(
            result,
            Err(DeviceRegistryError::InvalidProviderId { .. })
        ));
    }

    #[test]
    fn query_limit_zero_is_rejected() {
        let result =
            DeviceQuery::new().with_limit(0);

        assert!(matches!(
            result,
            Err(DeviceRegistryError::QueryResultLimitExceeded {
                requested: 0,
                ..
            })
        ));
    }

    #[test]
    fn query_limit_above_bound_is_rejected() {
        let result =
            DeviceQuery::new().with_limit(MAX_QUERY_RESULTS + 1);

        assert!(matches!(
            result,
            Err(DeviceRegistryError::QueryResultLimitExceeded {
                ..
            })
        ));
    }

    #[test]
    fn normalization_is_deterministic() {
        assert_eq!(
            normalize_name("  CX  "),
            "cx"
        );

        assert_eq!(
            normalize_name("RZ"),
            "rz"
        );
    }

    #[test]
    fn provider_index_removal_removes_empty_provider() {
        let mut index = BTreeMap::new();

        index
            .entry("provider://test".to_owned())
            .or_insert_with(BTreeSet::new)
            .insert("provider://test/device".to_owned());

        remove_from_provider_index(
            &mut index,
            "provider://test",
            "provider://test/device",
        );

        assert!(index.is_empty());
    }

    #[test]
    fn snapshot_empty_state_is_consistent() {
        let registry = registry();

        let snapshot =
            registry.snapshot().unwrap();

        assert_eq!(
            snapshot.schema_id,
            DEVICE_REGISTRY_SCHEMA_ID
        );

        assert_eq!(
            snapshot.schema_version,
            DEVICE_REGISTRY_SCHEMA_VERSION
        );

        assert_eq!(snapshot.generation, 0);
        assert_eq!(snapshot.device_count, 0);
        assert_eq!(
            snapshot.max_devices,
            DEFAULT_MAX_DEVICES
        );
    }

    #[test]
    fn default_query_is_valid() {
        assert!(DeviceQuery::new().validate().is_ok());
    }

    #[test]
    fn device_snapshot_empty_state_is_consistent() {
        let registry = registry();

        let snapshot =
            registry.snapshot_devices().unwrap();

        assert_eq!(snapshot.generation, 0);
        assert!(snapshot.is_empty());
        assert_eq!(snapshot.len(), 0);
    }

    #[test]
    fn empty_registry_integrity_is_valid() {
        let registry = registry();

        assert!(registry.validate_integrity().is_ok());
    }
}