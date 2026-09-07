//! Zamani Quantum Resilience — Backend Adapter Registry
//!
//! Production-grade registry for executable quantum backend adapters.
//!
//! # Responsibility
//!
//! This module provides the resilience subsystem with a deterministic,
//! provider-neutral catalogue of executable backend adapters.
//!
//! It owns:
//!
//! - adapter registration;
//! - adapter replacement;
//! - adapter removal;
//! - backend-identity lookup;
//! - deterministic enumeration;
//! - availability/status filtering;
//! - adapter identity snapshots;
//! - duplicate detection;
//! - registry consistency validation;
//! - immutable adapter sharing;
//! - resilience-facing backend discovery.
//!
//! It deliberately does NOT own:
//!
//! - the canonical `QuantumBackend` model;
//! - provider authentication;
//! - credentials;
//! - provider SDKs;
//! - network communication;
//! - execution;
//! - job polling;
//! - routing;
//! - scheduling;
//! - calibration;
//! - topology algorithms;
//! - QEC;
//! - mitigation;
//! - recovery planning;
//! - provider-specific capability interpretation.
//!
//! Those responsibilities remain in the hardware subsystem and the
//! corresponding resilience modules.
//!
//! # Architectural boundary
//!
//! ```text
//!                         Zamani Quantum
//!                              |
//!                    +---------+---------+
//!                    |                   |
//!                Hardware             Resilience
//!                    |                   |
//!          QuantumBackend          BackendRegistry
//!                    |                   |
//!          QuantumBackendAdapter <------+
//!                    |
//!        +-----------+-----------+
//!        |           |           |
//!      Local       QPU       Simulator
//!      adapter    adapter     adapter
//! ```
//!
//! The important distinction is:
//!
//! ```text
//! QuantumBackend
//!     = description of an execution target
//!
//! QuantumBackendAdapter
//!     = executable implementation for that target
//!
//! BackendRegistry
//!     = deterministic resilience-facing catalogue of adapters
//! ```
//!
//! # Why the registry belongs in resilience
//!
//! Resilience may need to discover alternative execution targets after:
//!
//! - hardware degradation;
//! - backend unavailability;
//! - topology degradation;
//! - calibration drift;
//! - execution failure;
//! - provider outage;
//! - capability changes;
//! - migration;
//! - recovery planning.
//!
//! The registry provides discovery only.
//!
//! It must never decide that a particular backend is semantically preferable.
//! That decision belongs to planning/policy.
//!
//! # Scalability
//!
//! There are no fixed backend counts, qubit counts, provider lists, or
//! retry counts in this module.
//!
//! The registry scales with the number of registered adapters and uses
//! `BTreeMap` for deterministic ordering.
//!
//! It does not allocate a table proportional to physical qubit count.
//!
//! Qubit-level information remains inside `QuantumBackend`, topology,
//! capabilities, routing, and hardware resource models.
//!
//! # Determinism
//!
//! Registry enumeration is deterministic because the primary index is a
//! `BTreeMap`.
//!
//! No system clock, randomness, network state, or global mutable state is
//! consulted by registry operations.
//!
//! # Concurrency
//!
//! Adapters are stored behind `Arc<dyn QuantumBackendAdapter>`.
//!
//! `QuantumBackendAdapter` is already defined by the hardware layer as a
//! `Send + Sync` object-safe contract, allowing registry entries to be shared
//! between resilience controllers, planners, execution engines, and
//! concurrent workers.
//!
//! The registry itself intentionally remains an ordinary owned value rather
//! than introducing a global singleton.
//!
//! A concurrent owner may place the registry behind an appropriate application
//! synchronization primitive without this module imposing one.
//!
//! # Security
//!
//! This module never stores credentials.
//!
//! Adapter objects remain responsible for authentication and secure transport.
//!
//! Registry diagnostics expose stable identities only. Provider credentials,
//! tokens, authorization headers, private keys, cookies, and other secret
//! material must never be encoded into backend identifiers.
//!
//! # Compatibility
//!
//! This module consumes the existing hardware contracts:
//!
//! - `crate::quantum::hardware::backend::QuantumBackend`;
//! - `crate::quantum::hardware::backend_trait::QuantumBackendAdapter`.
//!
//! It does not introduce another backend trait.
//!
//! # Integration contract
//!
//! `registry/backend.rs` is consumed by:
//!
//! - `resilience/api/controller.rs`
//! - `resilience/planning/planner.rs`
//! - `resilience/planning/feasibility.rs`
//! - `resilience/adaptation/backend_selection.rs`
//! - `resilience/recovery/migration.rs`
//! - `resilience/verification/provenance.rs`
//! - `resilience/telemetry/*`
//! - `resilience/registry/mod.rs`
//! - `quantum::hardware::provider_registry`
//! - `quantum::hardware::device_registry`
//!
//! The resilience planner must use this registry for discovery and then apply
//! policy, capability, feasibility, cost, security, and verification rules
//! before selecting an execution target.
//!
//! # Important architectural rule
//!
//! ```text
//! Registry discovers.
//! Policy constrains.
//! Planner decides.
//! Adapter executes.
//! Verification accepts or rejects.
//! ```
//!
//! The registry MUST NOT become a hidden scheduler, load balancer,
//! retry engine, or backend-selection policy.
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
//! # Safety
//!
//! Unsafe Rust is explicitly forbidden.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use crate::quantum::hardware::backend_trait::QuantumBackendAdapter;
use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the resilience backend registry.
pub const BACKEND_REGISTRY_SCHEMA_ID: &str =
    "zamani.quantum.resilience.registry.backend";

/// Semantic schema version.
///
/// Increment only when the meaning of the public registry contract changes
/// incompatibly.
pub const BACKEND_REGISTRY_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Backend registry key
// =============================================================================

/// Stable registry key for a backend adapter.
///
/// The backend identity is supplied by the canonical hardware descriptor
/// exposed through `QuantumBackendAdapter::backend()`.
///
/// The adapter identity is supplied separately by the hardware adapter
/// contract. Keeping both identities allows one physical/logical backend
/// descriptor to be associated with an explicit implementation identity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BackendRegistryKey {
    backend_id: String,
    adapter_id: String,
}

impl BackendRegistryKey {
    /// Creates a validated registry key.
    ///
    /// Validation deliberately rejects empty or whitespace-only identifiers.
    /// Length is not artificially constrained here because the authoritative
    /// hardware contracts already validate their own identifiers.
    pub fn new(
        backend_id: impl Into<String>,
        adapter_id: impl Into<String>,
    ) -> ResilienceResult<Self> {
        let backend_id = backend_id.into();
        let adapter_id = adapter_id.into();

        validate_identifier("backend_id", &backend_id)?;
        validate_identifier("adapter_id", &adapter_id)?;

        Ok(Self {
            backend_id,
            adapter_id,
        })
    }

    /// Returns the canonical backend identifier.
    pub fn backend_id(&self) -> &str {
        &self.backend_id
    }

    /// Returns the executable adapter identifier.
    pub fn adapter_id(&self) -> &str {
        &self.adapter_id
    }
}

impl fmt::Display for BackendRegistryKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .write_str(&self.backend_id)
            .and_then(|_| formatter.write_str("::"))
            .and_then(|_| formatter.write_str(&self.adapter_id))
    }
}

// =============================================================================
// Adapter snapshot
// =============================================================================

/// Immutable registry metadata captured when an adapter is registered.
///
/// This snapshot prevents registry indexing from depending on mutable
/// provider state while still allowing the adapter itself to expose live
/// hardware information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendRegistration {
    key: BackendRegistryKey,
}

impl BackendRegistration {
    fn from_adapter(
        adapter: &dyn QuantumBackendAdapter,
    ) -> ResilienceResult<Self> {
        let backend_id = adapter.backend().id().to_string();

        let adapter_id = adapter
            .adapter_info()
            .adapter_id
            .trim()
            .to_string();

        let key = BackendRegistryKey::new(backend_id, adapter_id)?;

        Ok(Self { key })
    }

    /// Returns the stable backend identifier.
    pub fn backend_id(&self) -> &str {
        self.key.backend_id()
    }

    /// Returns the stable adapter identifier.
    pub fn adapter_id(&self) -> &str {
        self.key.adapter_id()
    }

    /// Returns the complete registry key.
    pub fn key(&self) -> &BackendRegistryKey {
        &self.key
    }
}

// =============================================================================
// Registry entry
// =============================================================================

/// Internal immutable registry entry.
///
/// The adapter is shared using `Arc` because hardware adapters are explicitly
/// designed as `Send + Sync` objects by the hardware execution contract.
#[derive(Clone)]
struct BackendRegistryEntry {
    registration: BackendRegistration,
    adapter: Arc<dyn QuantumBackendAdapter>,
}

impl BackendRegistryEntry {
    fn new(
        adapter: Arc<dyn QuantumBackendAdapter>,
    ) -> ResilienceResult<Self> {
        let registration =
            BackendRegistration::from_adapter(adapter.as_ref())?;

        Ok(Self {
            registration,
            adapter,
        })
    }

    fn key(&self) -> &BackendRegistryKey {
        self.registration.key()
    }
}

// =============================================================================
// Registry
// =============================================================================

/// Deterministic registry of executable quantum backend adapters.
///
/// # Ownership
///
/// The registry owns an `Arc` reference to each adapter but does not own the
/// underlying provider credentials, network clients, or external resources
/// conceptually associated with the adapter.
///
/// # Selection semantics
///
/// This type intentionally does not contain a `select_best()` operation.
///
/// A resilience registry must not silently turn discovery into policy.
///
/// Consumers should perform:
///
/// ```text
/// registry discovery
///       ↓
/// capability filtering
///       ↓
/// policy filtering
///       ↓
/// security filtering
///       ↓
/// feasibility analysis
///       ↓
/// cost/risk ranking
///       ↓
/// planner decision
/// ```
///
/// # Determinism
///
/// All public enumeration methods use the deterministic ordering of the
/// underlying `BTreeMap`.
#[derive(Default, Clone)]
pub struct BackendRegistry {
    entries: BTreeMap<BackendRegistryKey, BackendRegistryEntry>,
}

impl fmt::Debug for BackendRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let keys: Vec<&BackendRegistryKey> =
            self.entries.keys().collect();

        formatter
            .debug_struct("BackendRegistry")
            .field("schema_id", &BACKEND_REGISTRY_SCHEMA_ID)
            .field("schema_version", &BACKEND_REGISTRY_SCHEMA_VERSION)
            .field("entry_count", &self.entries.len())
            .field("keys", &keys)
            .finish()
    }
}

impl BackendRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of registered adapter entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true when no adapters are registered.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Registers a new backend adapter.
    ///
    /// Registration is atomic from the registry's perspective: validation
    /// happens before the entry is inserted.
    ///
    /// Duplicate `(backend_id, adapter_id)` keys are rejected.
    ///
    /// Use [`Self::replace`] when intentional replacement is required.
    pub fn register<A>(&mut self, adapter: A) -> ResilienceResult<BackendRegistration>
    where
        A: QuantumBackendAdapter + 'static,
    {
        self.register_arc(Arc::new(adapter))
    }

    /// Registers an already shared backend adapter.
    ///
    /// This is useful when the hardware layer, execution layer, and resilience
    /// layer intentionally share the same adapter instance.
    pub fn register_arc(
        &mut self,
        adapter: Arc<dyn QuantumBackendAdapter>,
    ) -> ResilienceResult<BackendRegistration> {
        let entry = BackendRegistryEntry::new(adapter)?;
        let key = entry.key().clone();
        let registration = entry.registration.clone();

        if self.entries.contains_key(&key) {
            return Err(ResilienceError::new(
                ResilienceErrorCode::AlreadyExists,
                format!(
                    "backend adapter '{}' is already registered",
                    key
                ),
            ));
        }

        self.entries.insert(key, entry);

        Ok(registration)
    }

    /// Replaces an existing adapter with the same registry key.
    ///
    /// Replacement is explicit so that accidental provider/adapter
    /// substitution cannot occur silently.
    pub fn replace<A>(
        &mut self,
        adapter: A,
    ) -> ResilienceResult<BackendRegistration>
    where
        A: QuantumBackendAdapter + 'static,
    {
        self.replace_arc(Arc::new(adapter))
    }

    /// Replaces an existing shared adapter.
    pub fn replace_arc(
        &mut self,
        adapter: Arc<dyn QuantumBackendAdapter>,
    ) -> ResilienceResult<BackendRegistration> {
        let entry = BackendRegistryEntry::new(adapter)?;
        let key = entry.key().clone();
        let registration = entry.registration.clone();

        if self.entries.insert(key.clone(), entry).is_none() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::NotFound,
                format!(
                    "cannot replace unregistered backend adapter '{}'",
                    key
                ),
            ));
        }

        Ok(registration)
    }

    /// Removes an adapter by its complete registry key.
    ///
    /// The returned adapter remains alive as long as the caller retains the
    /// returned `Arc`.
    pub fn remove(
        &mut self,
        key: &BackendRegistryKey,
    ) -> ResilienceResult<Arc<dyn QuantumBackendAdapter>> {
        self.entries
            .remove(key)
            .map(|entry| entry.adapter)
            .ok_or_else(|| {
                ResilienceError::new(
                    ResilienceErrorCode::NotFound,
                    format!(
                        "backend adapter '{}' is not registered",
                        key
                    ),
                )
            })
    }

    /// Removes an adapter using backend and adapter identifiers.
    pub fn remove_by_id(
        &mut self,
        backend_id: &str,
        adapter_id: &str,
    ) -> ResilienceResult<Arc<dyn QuantumBackendAdapter>> {
        let key = BackendRegistryKey::new(
            backend_id.to_string(),
            adapter_id.to_string(),
        )?;

        self.remove(&key)
    }

    /// Returns an adapter by its complete registry key.
    pub fn get(
        &self,
        key: &BackendRegistryKey,
    ) -> Option<Arc<dyn QuantumBackendAdapter>> {
        self.entries
            .get(key)
            .map(|entry| Arc::clone(&entry.adapter))
    }

    /// Returns an adapter by backend and adapter identifier.
    pub fn get_by_id(
        &self,
        backend_id: &str,
        adapter_id: &str,
    ) -> ResilienceResult<Option<Arc<dyn QuantumBackendAdapter>>> {
        let key = BackendRegistryKey::new(
            backend_id.to_string(),
            adapter_id.to_string(),
        )?;

        Ok(self.get(&key))
    }

    /// Returns all registered keys in deterministic order.
    pub fn keys(
        &self,
    ) -> impl Iterator<Item = &BackendRegistryKey> {
        self.entries.keys()
    }

    /// Returns all registered adapters in deterministic order.
    ///
    /// The order is determined by `(backend_id, adapter_id)`.
    pub fn adapters(
        &self,
    ) -> impl Iterator<Item = (&BackendRegistryKey, Arc<dyn QuantumBackendAdapter>)>
    {
        self.entries
            .iter()
            .map(|(key, entry)| (key, Arc::clone(&entry.adapter)))
    }

    /// Returns all registered adapters for one backend identity.
    ///
    /// Multiple adapters may legitimately exist for the same backend descriptor
    /// when, for example, different adapter implementations or protocol
    /// versions are intentionally registered.
    pub fn adapters_for_backend(
        &self,
        backend_id: &str,
    ) -> ResilienceResult<
        impl Iterator<Item = (&BackendRegistryKey, Arc<dyn QuantumBackendAdapter>)>,
    > {
        validate_identifier("backend_id", backend_id)?;

        Ok(self
            .entries
            .iter()
            .filter(move |(key, _)| key.backend_id() == backend_id)
            .map(|(key, entry)| (key, Arc::clone(&entry.adapter))))
    }

    /// Returns true if the exact adapter key is registered.
    pub fn contains(&self, key: &BackendRegistryKey) -> bool {
        self.entries.contains_key(key)
    }

    /// Returns true if the backend identity has at least one adapter.
    pub fn contains_backend(
        &self,
        backend_id: &str,
    ) -> ResilienceResult<bool> {
        validate_identifier("backend_id", backend_id)?;

        Ok(self
            .entries
            .keys()
            .any(|key| key.backend_id() == backend_id))
    }

    /// Returns the number of registered adapters for one backend.
    pub fn adapter_count_for_backend(
        &self,
        backend_id: &str,
    ) -> ResilienceResult<usize> {
        validate_identifier("backend_id", backend_id)?;

        Ok(self
            .entries
            .keys()
            .filter(|key| key.backend_id() == backend_id)
            .count())
    }

    /// Removes all adapters.
    ///
    /// This operation is explicit and returns the number of removed entries.
    ///
    /// It is intended for lifecycle teardown, test isolation, or complete
    /// registry replacement. It is not used by automatic resilience recovery.
    pub fn clear(&mut self) -> usize {
        let count = self.entries.len();
        self.entries.clear();
        count
    }

    /// Validates internal registry invariants.
    ///
    /// This is intentionally inexpensive enough to be used at composition
    /// boundaries, while avoiding network calls or provider operations.
    pub fn validate(&self) -> ResilienceResult<()> {
        for (map_key, entry) in &self.entries {
            let derived = BackendRegistration::from_adapter(
                entry.adapter.as_ref(),
            )?;

            if map_key != derived.key() {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::StateConflict,
                    format!(
                        "backend registry identity drift detected for '{}'",
                        map_key
                    ),
                ));
            }

            if entry.registration.key() != derived.key() {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::StateConflict,
                    format!(
                        "backend registration snapshot drift detected for '{}'",
                        map_key
                    ),
                ));
            }
        }

        Ok(())
    }

    /// Produces a deterministic snapshot of registry registrations.
    ///
    /// This is useful for resilience provenance, diagnostics, testing, and
    /// deterministic planning inputs without exposing adapter internals.
    pub fn registrations(
        &self,
    ) -> impl Iterator<Item = &BackendRegistration> {
        self.entries
            .values()
            .map(|entry| &entry.registration)
    }
}

// =============================================================================
// Convenience conversions
// =============================================================================

impl From<&BackendRegistryKey> for String {
    fn from(value: &BackendRegistryKey) -> Self {
        value.to_string()
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates an externally supplied identifier.
///
/// The registry intentionally has no arbitrary identifier length constant.
/// The authoritative hardware contracts already validate their identifiers.
/// This helper therefore checks semantic safety only.
fn validate_identifier(
    field: &str,
    value: &str,
) -> ResilienceResult<()> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err(ResilienceError::new(
            ResilienceErrorCode::InvalidIdentifier,
            format!("{field} must not be empty"),
        ));
    }

    if trimmed != value {
        return Err(ResilienceError::new(
            ResilienceErrorCode::InvalidIdentifier,
            format!("{field} must not contain leading or trailing whitespace"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(ResilienceError::new(
            ResilienceErrorCode::InvalidIdentifier,
            format!("{field} must not contain control characters"),
        ));
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_key_rejects_empty_backend_id() {
        let result = BackendRegistryKey::new("", "adapter");

        assert!(result.is_err());
    }

    #[test]
    fn registry_key_rejects_empty_adapter_id() {
        let result = BackendRegistryKey::new("backend", "");

        assert!(result.is_err());
    }

    #[test]
    fn registry_key_is_deterministic() {
        let a = BackendRegistryKey::new("backend-a", "adapter-a")
            .expect("valid key");

        let b = BackendRegistryKey::new("backend-b", "adapter-a")
            .expect("valid key");

        assert!(a < b);
        assert_eq!(a.backend_id(), "backend-a");
        assert_eq!(a.adapter_id(), "adapter-a");
    }

    #[test]
    fn identifier_rejects_control_characters() {
        let result = BackendRegistryKey::new(
            "backend\ninvalid",
            "adapter",
        );

        assert!(result.is_err());
    }

    #[test]
    fn empty_registry_is_valid() {
        let registry = BackendRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
        assert!(registry.validate().is_ok());
    }
}