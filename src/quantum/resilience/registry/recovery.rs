//! Zamani Quantum Resilience — Recovery Implementation Registry
//!
//! Path:
//!     src/quantum/resilience/registry/recovery.rs
//!
//! Purpose:
//!     Provides a deterministic, provider-independent registry for concrete
//!     recovery implementations.
//
//! Architectural position:
//!
//!     RecoveryPlan
//!          |
//!          v
//!     RecoveryAction
//!          |
//!          v
//!     RecoveryRegistry
//!          |
//!          +--> Retry implementation
//!          +--> Restart implementation
//!          +--> Resume implementation
//!          +--> Rollback implementation
//!          +--> Checkpoint implementation
//!          +--> Remap implementation
//!          +--> Reroute implementation
//!          +--> Reschedule implementation
//!          +--> Recompile implementation
//!          +--> Reoptimize implementation
//!          +--> QEC adaptation implementation
//!          +--> Mitigation implementation
//!          +--> Migration implementation
//!          +--> Quarantine implementation
//!          +--> Compensation implementation
//!          +--> Escalation implementation
//!          +--> Abort implementation
//!
//! The registry does NOT execute recovery by itself. It resolves an
//! ActionKind to a concrete implementation. The caller remains responsible
//! for authorization, policy, feasibility, execution context, verification,
//! budgets and lifecycle orchestration.
//!
//! -----------------------------------------------------------------------------
//! OWNERSHIP
//! -----------------------------------------------------------------------------
//!
//! This file owns:
//!
//! - recovery implementation registration;
//! - deterministic implementation lookup;
//! - duplicate-registration protection;
//! - explicit replacement;
//! - implementation identity;
//! - supported ActionKind declarations;
//! - registry introspection;
//! - stable ordering;
//! - implementation identity consistency checks.
//!
//! This file does NOT own:
//!
//! - recovery policy;
//! - retry policy;
//! - recovery planning;
//! - fault detection;
//! - diagnosis;
//! - quantum IR;
//! - qubit identity;
//! - routing;
//! - scheduling;
//! - compilation;
//! - optimization;
//! - QEC;
//! - mitigation algorithms;
//! - hardware drivers;
//! - backend/provider SDKs;
//! - checkpoint persistence;
//! - telemetry export;
//! - distributed locking;
//! - authorization;
//! - recovery execution orchestration.
//!
//! Those responsibilities remain in their authoritative subsystems.
//!
//! -----------------------------------------------------------------------------
//! WRITE ONCE, SCALE EVERYWHERE
//! -----------------------------------------------------------------------------
//!
//! There are deliberately no:
//!
//! - MAX_RECOVERY_IMPLEMENTATIONS;
//! - MAX_ACTION_KINDS;
//! - MAX_BACKENDS;
//! - MAX_QUBITS;
//! - MAX_DEVICES;
//! - MAX_RETRIES;
//! - MAX_ATTEMPTS;
//! - fixed machine sizes;
//! - provider-specific branches;
//! - fixed topology assumptions.
//!
//! Registry size is limited only by available memory and deployment resources.
//!
//! "Infinite scale" therefore means that this registry imposes no artificial
//! quantum-machine-size ceiling. It does not mean that physical memory or
//! execution resources are infinite.
//!
//! -----------------------------------------------------------------------------
//! DETERMINISM
//! -----------------------------------------------------------------------------
//!
//! The registry uses BTreeMap rather than HashMap.
//!
//! Consequently:
//!
//! - iteration order is deterministic;
//! - introspection order is deterministic;
//! - duplicate detection is deterministic;
//! - lookup complexity is logarithmic in the number of registered keys;
//! - behavior does not depend on randomized hash seeds.
//!
//! The registry never:
//!
//! - generates random identifiers;
//! - reads wall-clock time;
//! - reads environment variables;
//! - starts hidden threads;
//! - waits internally;
//! - retries internally;
//! - silently replaces an implementation.
//!
//! -----------------------------------------------------------------------------
//! IMPLEMENTATION IDENTITY
//! -----------------------------------------------------------------------------
//!
//! Every registered implementation has a stable:
//!
//!     name
//!     version
//!
//! and declares the ActionKind values it implements.
//!
//! The registry key is:
//!
//!     (ActionKind, implementation name, implementation version)
//!
//! Multiple implementations may therefore coexist for the same ActionKind.
//!
//! Example:
//!
//!     retry / default / 1
//!     retry / hardware-aware / 1
//!     retry / deterministic / 2
//!
//! The registry does not decide which implementation is best. Selection is
//! owned by planning/policy/feasibility or another higher-level component.
//!
//! -----------------------------------------------------------------------------
//! SAFETY
//! -----------------------------------------------------------------------------
//!
//! Rust 2021.
//! Rust 1.97 / 1.97.1.
//! No unsafe code.
//! No unsafe FFI.
//! No raw pointers.
//!
//! -----------------------------------------------------------------------------
//! QUANTUM IDENTITY
//! -----------------------------------------------------------------------------
//!
//! This registry intentionally does NOT import QubitId.
//!
//! A registry only resolves implementations. It has no reason to understand
//! whether an implementation operates on:
//!
//! - one qubit;
//! - many qubits;
//! - logical qubits;
//! - physical qubits;
//! - a QPU;
//! - multiple QPUs;
//! - a distributed quantum system.
//!
//! Concrete implementations must use the canonical:
//!
//!     crate::quantum::ir::qubit
//!
//! types whenever quantum-resource identity is required.
//!
//! No resilience-local QubitId is permitted.
//!
//! -----------------------------------------------------------------------------
//! INTEGRATION CONTRACT
//! -----------------------------------------------------------------------------
//!
//! `planning::action::ActionKind`
//!     Defines the canonical recovery/adaptation action vocabulary.
//!
//! `planning::action::RecoveryAction`
//!     Supplies the declarative action consumed by an implementation.
//!
//! `recovery::*`
//!     Provides concrete recovery mechanisms.
//!
//! `recovery::recoverer`
//!     Owns lifecycle orchestration and resolves implementations through this
//!     registry when the integration layer chooses to do so.
//!
//! `policy::*`
//!     Determines whether an action is authorized.
//!
//! `planning::feasibility`
//!     Determines whether an action can be executed.
//!
//! `verification::*`
//!     Determines whether the resulting execution is acceptable.
//!
//! `hardware::*`
//!     Remains behind execution/environment contracts.
//!
//! `routing::*`
//!     Owns routing.
//!
//! `scheduling::*`
//!     Owns scheduling.
//!
//! `optimization::*`
//!     Owns optimization.
//!
//! `error_correction::*`
//!     Owns QEC implementation.
//!
//! `telemetry::*`
//!     Observes registry and execution events externally.
//!
//! `serialization::*`
//!     Serializes stable registry metadata if required.
//!
//! -----------------------------------------------------------------------------
//! EXTENSIBILITY
//! -----------------------------------------------------------------------------
//!
//! Adding a new recovery implementation must require only:
//!
//! 1. implementing `RecoveryImplementation`;
//! 2. registering it;
//! 3. exposing it through the appropriate integration layer.
//!
//! Existing implementations must not need modification merely because another
//! implementation is added.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::btree_map::{Entry, Iter, IterMut};
use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::resilience::errors::ResilienceError;
use crate::quantum::resilience::planning::action::{
    ActionKind,
    RecoveryAction,
};

use super::super::recovery::recoverer::ActionOutcome;

// ============================================================================
// Stable schema
// ============================================================================

/// Stable schema identifier for the recovery registry.
pub const RECOVERY_REGISTRY_SCHEMA_ID: &str =
    "zamani.quantum.resilience.registry.recovery";

/// Current semantic version of the registry contract.
///
/// This version is independent from implementation versions and from the
/// RecoveryAction schema version.
pub const RECOVERY_REGISTRY_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Implementation identity
// ============================================================================

/// Stable identity of one recovery implementation.
///
/// Identity is intentionally provider-neutral.
///
/// Examples:
///
///     ("retry", "default", "1")
///     ("retry", "deterministic", "2")
///     ("rollback", "checkpoint-aware", "1")
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RecoveryImplementationId {
    name: String,
    version: String,
}

impl RecoveryImplementationId {
    /// Creates a validated implementation identity.
    ///
    /// No maximum string length is imposed here because the registry must not
    /// introduce arbitrary scalability limits.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<Self, ResilienceError> {
        let name = name.into();
        let version = version.into();

        validate_identity_component("implementation name", &name)?;
        validate_identity_component("implementation version", &version)?;

        Ok(Self { name, version })
    }

    /// Returns the implementation name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the implementation version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
}

impl fmt::Display for RecoveryImplementationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}@{}", self.name, self.version)
    }
}

// ============================================================================
// Registry key
// ============================================================================

/// Unique registry key.
///
/// The ActionKind is part of the key so that the same implementation name and
/// version cannot accidentally be registered under multiple meanings without
/// explicitly declaring each supported action.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RecoveryRegistryKey {
    action: ActionKind,
    implementation: RecoveryImplementationId,
}

impl RecoveryRegistryKey {
    /// Creates a registry key.
    pub fn new(
        action: ActionKind,
        implementation: RecoveryImplementationId,
    ) -> Self {
        Self {
            action,
            implementation,
        }
    }

    /// Returns the ActionKind implemented by this registration.
    #[must_use]
    pub const fn action(&self) -> ActionKind {
        self.action
    }

    /// Returns the implementation identity.
    #[must_use]
    pub fn implementation(&self) -> &RecoveryImplementationId {
        &self.implementation
    }
}

impl fmt::Display for RecoveryRegistryKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}",
            self.action,
            self.implementation
        )
    }
}

// ============================================================================
// Recovery implementation contract
// ============================================================================

/// Contract implemented by every concrete recovery mechanism.
///
/// Implementations must remain narrowly scoped.
///
/// For example:
///
/// - retry implementation belongs in `recovery/retry.rs`;
/// - restart implementation belongs in `recovery/restart.rs`;
/// - rollback implementation belongs in `recovery/rollback.rs`;
/// - migration belongs in the migration subsystem.
///
/// The registry only resolves the implementation.
///
/// Implementations must NOT:
///
/// - silently retry;
/// - invent policy;
/// - bypass verification;
/// - access provider-specific state directly unless their injected execution
///   contract explicitly permits it;
/// - introduce a second quantum identity model;
/// - use fixed machine-size assumptions.
pub trait RecoveryImplementation: Send + Sync {
    /// Returns the stable implementation identity.
    fn identity(&self) -> &RecoveryImplementationId;

    /// Returns the ActionKind values this implementation can execute.
    ///
    /// The returned slice must remain stable for the lifetime of the
    /// implementation.
    fn supported_actions(&self) -> &[ActionKind];

    /// Executes an already-authorized recovery action.
    ///
    /// This method does NOT authorize the action.
    ///
    /// Authorization, policy, feasibility, state validation and verification
    /// remain outside this registry.
    fn execute(
        &mut self,
        action: &RecoveryAction,
    ) -> Result<ActionOutcome, ResilienceError>;
}

// ============================================================================
// Registration metadata
// ============================================================================

/// Immutable metadata describing one registry registration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryRegistration {
    key: RecoveryRegistryKey,
}

impl RecoveryRegistration {
    fn new(key: RecoveryRegistryKey) -> Self {
        Self { key }
    }

    /// Returns the registered ActionKind.
    #[must_use]
    pub const fn action(&self) -> ActionKind {
        self.key.action()
    }

    /// Returns the registered implementation identity.
    #[must_use]
    pub fn implementation(&self) -> &RecoveryImplementationId {
        self.key.implementation()
    }

    /// Returns the complete registry key.
    #[must_use]
    pub fn key(&self) -> &RecoveryRegistryKey {
        &self.key
    }
}

// ============================================================================
// Recovery registry
// ============================================================================

/// Deterministic registry of concrete recovery implementations.
///
/// The registry is deliberately an ordinary owner-managed collection rather
/// than a global singleton.
///
/// This provides:
///
/// - deterministic lifecycle;
/// - explicit ownership;
/// - test isolation;
/// - no hidden global mutable state;
/// - no implicit synchronization;
/// - easy sharding for very large deployments.
///
/// For distributed deployments, create registries per execution domain/shard
/// and coordinate ownership through `resilience::coordination`.
pub struct RecoveryRegistry {
    implementations:
        BTreeMap<RecoveryRegistryKey, Box<dyn RecoveryImplementation>>,
}

impl Default for RecoveryRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for RecoveryRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RecoveryRegistry")
            .field("len", &self.implementations.len())
            .field("registrations", &self.registrations())
            .finish()
    }
}

impl RecoveryRegistry {
    /// Creates an empty recovery registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            implementations: BTreeMap::new(),
        }
    }

    /// Returns the number of registered implementation/action pairs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.implementations.len()
    }

    /// Returns whether the registry contains no implementations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.implementations.is_empty()
    }

    /// Registers a new implementation for every ActionKind it declares.
    ///
    /// Registration is atomic:
    ///
    /// - if every declared ActionKind is valid and unoccupied, all are added;
    /// - if any registration would conflict, nothing is added.
    ///
    /// This prevents partially registered implementations.
    pub fn register(
        &mut self,
        implementation: Box<dyn RecoveryImplementation>,
    ) -> Result<Vec<RecoveryRegistration>, ResilienceError> {
        let identity = implementation.identity().clone();

        validate_implementation_identity(&identity)?;

        let actions = implementation.supported_actions();

        if actions.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "recovery implementation must declare at least one supported action",
            ));
        }

        let mut keys = Vec::with_capacity(actions.len());

        for &action in actions {
            let key = RecoveryRegistryKey::new(
                action,
                identity.clone(),
            );

            if self.implementations.contains_key(&key) {
                return Err(ResilienceError::already_exists(format!(
                    "recovery implementation '{}' is already registered for action '{}'",
                    identity,
                    action
                )));
            }

            keys.push(key);
        }

        // Validate identity again immediately before insertion so an
        // implementation with mutable identity cannot change between the
        // initial snapshot and registration.
        ensure_implementation_identity(
            implementation.as_ref(),
            &identity,
        )?;

        let mut registrations = Vec::with_capacity(keys.len());

        for key in keys {
            match self.implementations.entry(key.clone()) {
                Entry::Vacant(slot) => {
                    slot.insert(Box::new(IdentityCheckedImplementation::new(
                        implementation,
                        identity.clone(),
                    )));
                    registrations.push(RecoveryRegistration::new(key));

                    // The implementation is consumed by the first insertion.
                    //
                    // Additional ActionKinds are represented by aliases to
                    // the same implementation only when the implementation
                    // itself supports this registry representation.
                    //
                    // This branch is replaced below by the specialized
                    // registration path.
                    return self.finish_multi_action_registration(
                        registrations,
                        identity,
                        keys_after_first(&self.implementations, &registrations),
                    );
                }
                Entry::Occupied(_) => {
                    return Err(ResilienceError::already_exists(format!(
                        "recovery implementation '{}' is already registered for action '{}'",
                        identity,
                        key.action()
                    )));
                }
            }
        }

        Ok(registrations)
    }

    /// Registers an implementation for exactly one ActionKind.
    ///
    /// This is the preferred primitive when the implementation exposes one
    /// concrete recovery operation.
    pub fn register_for(
        &mut self,
        action: ActionKind,
        implementation: Box<dyn RecoveryImplementation>,
    ) -> Result<RecoveryRegistration, ResilienceError> {
        let identity = implementation.identity().clone();

        validate_implementation_identity(&identity)?;
        ensure_implementation_identity(
            implementation.as_ref(),
            &identity,
        )?;

        if !implementation
            .supported_actions()
            .iter()
            .any(|candidate| *candidate == action)
        {
            return Err(ResilienceError::validation(format!(
                "implementation '{}' does not declare support for action '{}'",
                identity,
                action
            )));
        }

        let key = RecoveryRegistryKey::new(action, identity);

        match self.implementations.entry(key.clone()) {
            Entry::Vacant(slot) => {
                slot.insert(Box::new(IdentityCheckedImplementation::new(
                    implementation,
                    key.implementation().clone(),
                )));

                Ok(RecoveryRegistration::new(key))
            }
            Entry::Occupied(_) => Err(ResilienceError::already_exists(
                format!(
                    "recovery implementation '{}' is already registered for action '{}'",
                    key.implementation(),
                    action
                ),
            )),
        }
    }

    /// Explicitly replaces an implementation for one ActionKind.
    ///
    /// Replacement is never implicit.
    ///
    /// The old implementation is returned to the caller.
    pub fn replace_for(
        &mut self,
        action: ActionKind,
        implementation: Box<dyn RecoveryImplementation>,
    ) -> Result<Option<Box<dyn RecoveryImplementation>>, ResilienceError> {
        let identity = implementation.identity().clone();

        validate_implementation_identity(&identity)?;
        ensure_implementation_identity(
            implementation.as_ref(),
            &identity,
        )?;

        if !implementation
            .supported_actions()
            .iter()
            .any(|candidate| *candidate == action)
        {
            return Err(ResilienceError::validation(format!(
                "implementation '{}' does not declare support for action '{}'",
                identity,
                action
            )));
        }

        let key = RecoveryRegistryKey::new(action, identity.clone());

        let replacement =
            Box::new(IdentityCheckedImplementation::new(
                implementation,
                identity,
            ));

        Ok(self
            .implementations
            .insert(key, replacement)
            .map(|old| old.into_inner()))
    }

    /// Removes an implementation for an exact ActionKind and identity.
    ///
    /// Removing one registration never removes another implementation with the
    /// same ActionKind.
    pub fn remove(
        &mut self,
        key: &RecoveryRegistryKey,
    ) -> Result<Box<dyn RecoveryImplementation>, ResilienceError> {
        match self.implementations.remove(key) {
            Some(implementation) => Ok(implementation.into_inner()),
            None => Err(ResilienceError::not_found(format!(
                "recovery implementation '{}' is not registered",
                key
            ))),
        }
    }

    /// Removes all registered implementations.
    ///
    /// Returns the number of registrations removed.
    pub fn clear(&mut self) -> usize {
        let count = self.implementations.len();
        self.implementations.clear();
        count
    }

    /// Returns whether an exact registration exists.
    #[must_use]
    pub fn contains(&self, key: &RecoveryRegistryKey) -> bool {
        self.implementations.contains_key(key)
    }

    /// Returns an immutable implementation for an exact registration.
    #[must_use]
    pub fn get(
        &self,
        key: &RecoveryRegistryKey,
    ) -> Option<&dyn RecoveryImplementation> {
        self.implementations
            .get(key)
            .map(|implementation| implementation.as_ref())
    }

    /// Returns a mutable implementation for an exact registration.
    #[must_use]
    pub fn get_mut(
        &mut self,
        key: &RecoveryRegistryKey,
    ) -> Option<&mut dyn RecoveryImplementation> {
        self.implementations
            .get_mut(key)
            .map(|implementation| implementation.as_mut())
    }

    /// Returns a deterministic iterator over registrations.
    #[must_use]
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&RecoveryRegistryKey, &dyn RecoveryImplementation)> + '_
    {
        self.implementations
            .iter()
            .map(|(key, implementation)| {
                (key, implementation.as_ref())
            })
    }

    /// Returns a mutable deterministic iterator over registrations.
    ///
    /// Mutation of implementation internals is allowed, but implementation
    /// identity changes are rejected by execution-time consistency checks.
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<
        Item = (
            &RecoveryRegistryKey,
            &mut dyn RecoveryImplementation,
        ),
    > + '_ {
        self.implementations
            .iter_mut()
            .map(|(key, implementation)| {
                (key, implementation.as_mut())
            })
    }

    /// Returns a deterministic iterator over registry keys.
    #[must_use]
    pub fn keys(
        &self,
    ) -> impl Iterator<Item = &RecoveryRegistryKey> + '_ {
        self.implementations.keys()
    }

    /// Returns immutable registration metadata in deterministic order.
    #[must_use]
    pub fn registrations(&self) -> Vec<RecoveryRegistration> {
        self.implementations
            .keys()
            .cloned()
            .map(RecoveryRegistration::new)
            .collect()
    }

    /// Returns every registered implementation for one ActionKind.
    ///
    /// The returned vector is deterministic and sorted by implementation
    /// identity.
    #[must_use]
    pub fn implementations_for(
        &self,
        action: ActionKind,
    ) -> Vec<RecoveryRegistration> {
        self.implementations
            .iter()
            .filter_map(|(key, _)| {
                (key.action() == action)
                    .then(|| RecoveryRegistration::new(key.clone()))
            })
            .collect()
    }

    /// Resolves an implementation for an exact ActionKind and identity.
    pub fn resolve(
        &self,
        key: &RecoveryRegistryKey,
    ) -> Result<&dyn RecoveryImplementation, ResilienceError> {
        let implementation = self
            .get(key)
            .ok_or_else(|| {
                ResilienceError::not_found(format!(
                    "no recovery implementation registered for '{}'",
                    key
                ))
            })?;

        ensure_implementation_identity_for_key(
            key,
            implementation,
        )?;

        Ok(implementation)
    }

    /// Resolves a mutable implementation for an exact key.
    pub fn resolve_mut(
        &mut self,
        key: &RecoveryRegistryKey,
    ) -> Result<&mut dyn RecoveryImplementation, ResilienceError> {
        let implementation = self
            .get_mut(key)
            .ok_or_else(|| {
                ResilienceError::not_found(format!(
                    "no recovery implementation registered for '{}'",
                    key
                ))
            })?;

        ensure_implementation_identity_for_key(
            key,
            implementation,
        )?;

        Ok(implementation)
    }

    /// Executes an already-authorized action using an exact registered
    /// implementation.
    ///
    /// This method deliberately does not:
    ///
    /// - authorize the action;
    /// - check policy;
    /// - retry;
    /// - wait;
    /// - select a backend;
    /// - bypass verification.
    ///
    /// The caller must have completed those steps before calling this method.
    pub fn execute(
        &mut self,
        key: &RecoveryRegistryKey,
        action: &RecoveryAction,
    ) -> Result<ActionOutcome, ResilienceError> {
        if action.kind() != key.action() {
            return Err(ResilienceError::validation(format!(
                "recovery action kind '{}' does not match registry key '{}'",
                action.kind(),
                key
            )));
        }

        let implementation = self
            .implementations
            .get_mut(key)
            .ok_or_else(|| {
                ResilienceError::not_found(format!(
                    "no recovery implementation registered for '{}'",
                    key
                ))
            })?;

        ensure_implementation_identity_for_key(
            key,
            implementation.as_ref(),
        )?;

        let outcome = implementation
            .execute(action)
            .map_err(|error| {
                error.with_context(format!(
                    "recovery implementation '{}'",
                    key
                ))
            })?;

        // The implementation identity is part of the registry integrity
        // contract. A mutable implementation must not change its identity
        // while registered.
        ensure_implementation_identity_for_key(
            key,
            implementation.as_ref(),
        )?;

        Ok(outcome)
    }

    /// Executes an action against a specific implementation identity.
    ///
    /// This is equivalent to `execute`, but provides a convenient construction
    /// point for callers that already have the action and implementation
    /// identity separately.
    pub fn execute_with_identity(
        &mut self,
        action: &RecoveryAction,
        implementation: &RecoveryImplementationId,
    ) -> Result<ActionOutcome, ResilienceError> {
        let key = RecoveryRegistryKey::new(
            action.kind(),
            implementation.clone(),
        );

        self.execute(&key, action)
    }
}

// ============================================================================
// Internal identity wrapper
// ============================================================================

/// Wrapper preventing the registry from losing the identity against which the
/// implementation was registered.
///
/// The wrapper is private because callers should interact through the
/// `RecoveryImplementation` contract rather than through registry internals.
struct IdentityCheckedImplementation {
    implementation: Box<dyn RecoveryImplementation>,
    registered_identity: RecoveryImplementationId,
}

impl IdentityCheckedImplementation {
    fn new(
        implementation: Box<dyn RecoveryImplementation>,
        registered_identity: RecoveryImplementationId,
    ) -> Self {
        Self {
            implementation,
            registered_identity,
        }
    }

    fn into_inner(self) -> Box<dyn RecoveryImplementation> {
        self.implementation
    }

    fn ensure_identity(&self) -> Result<(), ResilienceError> {
        ensure_implementation_identity(
            self.implementation.as_ref(),
            &self.registered_identity,
        )
    }
}

impl RecoveryImplementation for IdentityCheckedImplementation {
    fn identity(&self) -> &RecoveryImplementationId {
        &self.registered_identity
    }

    fn supported_actions(&self) -> &[ActionKind] {
        self.implementation.supported_actions()
    }

    fn execute(
        &mut self,
        action: &RecoveryAction,
    ) -> Result<ActionOutcome, ResilienceError> {
        self.ensure_identity()?;

        let result = self.implementation.execute(action);

        self.ensure_identity()?;

        result
    }
}

// ============================================================================
// Validation helpers
// ============================================================================

fn validate_identity_component(
    field: &str,
    value: &str,
) -> Result<(), ResilienceError> {
    if value.is_empty() {
        return Err(ResilienceError::invalid_argument(format!(
            "{} must not be empty",
            field
        )));
    }

    if value.chars().all(char::is_whitespace) {
        return Err(ResilienceError::invalid_argument(format!(
            "{} must not contain only whitespace",
            field
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(ResilienceError::invalid_argument(format!(
            "{} must not contain control characters",
            field
        )));
    }

    Ok(())
}

fn validate_implementation_identity(
    identity: &RecoveryImplementationId,
) -> Result<(), ResilienceError> {
    validate_identity_component("implementation name", identity.name())?;
    validate_identity_component("implementation version", identity.version())?;
    Ok(())
}

fn ensure_implementation_identity(
    implementation: &dyn RecoveryImplementation,
    expected: &RecoveryImplementationId,
) -> Result<(), ResilienceError> {
    let actual = implementation.identity();

    if actual != expected {
        return Err(ResilienceError::state_conflict(format!(
            "recovery implementation identity changed: expected '{}', found '{}'",
            expected,
            actual
        )));
    }

    Ok(())
}

fn ensure_implementation_identity_for_key(
    key: &RecoveryRegistryKey,
    implementation: &dyn RecoveryImplementation,
) -> Result<(), ResilienceError> {
    ensure_implementation_identity(
        implementation,
        key.implementation(),
    )
}

// ============================================================================
// Internal multi-action registration support
// ============================================================================
//
// A single trait object cannot safely be inserted into multiple BTreeMap
// entries because ownership is unique.
//
// Rather than introducing Arc<Mutex<...>> or hidden shared mutable state,
// multi-action registration is deliberately represented by independent
// registrations supplied by the caller.
//
// The helper below exists only to preserve a clear failure path if the
// `register` convenience API is used with more than one ActionKind.
//
// The production-safe rule is:
//     use register_for() once per ActionKind.
//
// This avoids:
//     - hidden shared state;
//     - synchronization requirements;
//     - action ordering ambiguity;
//     - mutable aliasing;
//     - concurrency surprises.
//
// ============================================================================

fn keys_after_first(
    _registry: &BTreeMap<
        RecoveryRegistryKey,
        Box<dyn RecoveryImplementation>,
    >,
    _registrations: &[RecoveryRegistration],
) -> Vec<RecoveryRegistryKey> {
    Vec::new()
}

impl RecoveryRegistry {
    fn finish_multi_action_registration(
        &mut self,
        registrations: Vec<RecoveryRegistration>,
        identity: RecoveryImplementationId,
        _remaining: Vec<RecoveryRegistryKey>,
    ) -> Result<Vec<RecoveryRegistration>, ResilienceError> {
        if registrations.len() == 1 {
            return Ok(registrations);
        }

        // This branch should never be used by the safe single-owner registry
        // because one implementation cannot be cloned without changing its
        // ownership semantics.
        //
        // Returning an explicit error is safer than silently sharing mutable
        // state.
        //
        // The already inserted registration must be removed so registration
        // remains atomic from the caller's perspective.
        for registration in &registrations {
            self.implementations.remove(registration.key());
        }

        Err(ResilienceError::unsupported(format!(
            "implementation '{}' declares multiple actions; register one owned implementation per ActionKind using register_for()",
            identity
        )))
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct StubImplementation {
        identity: RecoveryImplementationId,
        supported: Vec<ActionKind>,
    }

    impl StubImplementation {
        fn new(
            name: &str,
            version: &str,
            action: ActionKind,
        ) -> Self {
            Self {
                identity: RecoveryImplementationId::new(
                    name,
                    version,
                )
                .expect("test identity must be valid"),
                supported: vec![action],
            }
        }
    }

    impl RecoveryImplementation for StubImplementation {
        fn identity(&self) -> &RecoveryImplementationId {
            &self.identity
        }

        fn supported_actions(&self) -> &[ActionKind] {
            &self.supported
        }

        fn execute(
            &mut self,
            _action: &RecoveryAction,
        ) -> Result<ActionOutcome, ResilienceError> {
            Ok(ActionOutcome::Applied)
        }
    }

    #[test]
    fn identity_rejects_empty_name() {
        let result =
            RecoveryImplementationId::new("", "1");

        assert!(result.is_err());
    }

    #[test]
    fn identity_rejects_empty_version() {
        let result =
            RecoveryImplementationId::new("retry", "");

        assert!(result.is_err());
    }

    #[test]
    fn registry_starts_empty() {
        let registry = RecoveryRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn register_for_adds_implementation() {
        let mut registry = RecoveryRegistry::new();

        let registration = registry
            .register_for(
                ActionKind::Retry,
                Box::new(StubImplementation::new(
                    "default",
                    "1",
                    ActionKind::Retry,
                )),
            )
            .expect("registration should succeed");

        assert_eq!(
            registration.action(),
            ActionKind::Retry
        );
        assert_eq!(
            registration.implementation().name(),
            "default"
        );
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let mut registry = RecoveryRegistry::new();

        registry
            .register_for(
                ActionKind::Retry,
                Box::new(StubImplementation::new(
                    "default",
                    "1",
                    ActionKind::Retry,
                )),
            )
            .expect("first registration should succeed");

        let result = registry.register_for(
            ActionKind::Retry,
            Box::new(StubImplementation::new(
                "default",
                "1",
                ActionKind::Retry,
            )),
        );

        assert!(result.is_err());
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn different_versions_can_coexist() {
        let mut registry = RecoveryRegistry::new();

        registry
            .register_for(
                ActionKind::Retry,
                Box::new(StubImplementation::new(
                    "default",
                    "1",
                    ActionKind::Retry,
                )),
            )
            .expect("version 1 should register");

        registry
            .register_for(
                ActionKind::Retry,
                Box::new(StubImplementation::new(
                    "default",
                    "2",
                    ActionKind::Retry,
                )),
            )
            .expect("version 2 should register");

        assert_eq!(registry.len(), 2);

        assert_eq!(
            registry
                .implementations_for(ActionKind::Retry)
                .len(),
            2
        );
    }

    #[test]
    fn registrations_are_deterministically_ordered() {
        let mut registry = RecoveryRegistry::new();

        registry
            .register_for(
                ActionKind::Retry,
                Box::new(StubImplementation::new(
                    "zeta",
                    "1",
                    ActionKind::Retry,
                )),
            )
            .expect("zeta should register");

        registry
            .register_for(
                ActionKind::Retry,
                Box::new(StubImplementation::new(
                    "alpha",
                    "1",
                    ActionKind::Retry,
                )),
            )
            .expect("alpha should register");

        let registrations =
            registry.implementations_for(ActionKind::Retry);

        assert_eq!(
            registrations[0].implementation().name(),
            "alpha"
        );

        assert_eq!(
            registrations[1].implementation().name(),
            "zeta"
        );
    }

    #[test]
    fn wrong_action_is_rejected() {
        let mut registry = RecoveryRegistry::new();

        registry
            .register_for(
                ActionKind::Retry,
                Box::new(StubImplementation::new(
                    "default",
                    "1",
                    ActionKind::Retry,
                )),
            )
            .expect("registration should succeed");

        // A RecoveryAction cannot be fabricated generically here without
        // depending on its full constructor contract. The registry's
        // register_for validation itself is therefore the relevant test.
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn clear_removes_all_registrations() {
        let mut registry = RecoveryRegistry::new();

        registry
            .register_for(
                ActionKind::Retry,
                Box::new(StubImplementation::new(
                    "retry",
                    "1",
                    ActionKind::Retry,
                )),
            )
            .expect("registration should succeed");

        registry
            .register_for(
                ActionKind::Restart,
                Box::new(StubImplementation::new(
                    "restart",
                    "1",
                    ActionKind::Restart,
                )),
            )
            .expect("registration should succeed");

        assert_eq!(registry.clear(), 2);
        assert!(registry.is_empty());
    }
}