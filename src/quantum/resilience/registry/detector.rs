//! Zamani Quantum Resilience — Detector Registry
//!
//! Path:
//!     src/quantum/resilience/registry/detector.rs
//!
//! # Purpose
//!
//! This module owns explicit registration and deterministic discovery of
//! heterogeneous resilience detectors.
//!
//! The registry is intentionally:
//!
//! - provider-neutral;
//! - hardware-size independent;
//! - deterministic;
//! - explicit rather than globally mutable;
//! - object-safe at the detector boundary;
//! - compatible with streaming detector execution;
//! - free of unsafe Rust;
//! - independent of concrete detector implementations;
//! - independent of hardware providers;
//! - independent of QEC implementations;
//! - independent of diagnosis, planning, recovery and verification.
//!
//! The registry answers:
//!
//! > "Which detector implementations are explicitly registered in this
//! > resilience execution context, and can they be invoked deterministically?"
//!
//! It does NOT decide:
//!
//! - whether a detector should be used;
//! - which detector is more important;
//! - root cause;
//! - severity;
//! - recovery;
//! - mitigation;
//! - backend selection;
//! - routing;
//! - scheduling;
//! - QEC configuration;
//! - semantic result acceptance.
//!
//! Those responsibilities belong to their owning subsystems.
//!
//! # Architectural position
//!
//! ```text
//! quantum::hardware ───────┐
//! quantum::zqn ────────────┤
//! quantum::qec ────────────┤
//! runtime / telemetry ─────┤
//! benchmarking ────────────┤
//! simulation ──────────────┤
//!                             │
//!                             ▼
//!                    DetectionObservation
//!                             │
//!                             ▼
//!                    DetectionContext
//!                             │
//!                             ▼
//!                 ┌──────────────────────┐
//!                 │   DetectorRegistry   │
//!                 │                      │
//!                 │ deterministic index  │
//!                 │ explicit ownership   │
//!                 │ lifecycle management │
//!                 └──────────┬───────────┘
//!                            │
//!              ┌─────────────┼─────────────┐
//!              ▼             ▼             ▼
//!          detector A    detector B    detector N
//!              │             │             │
//!              └─────────────┼─────────────┘
//!                            ▼
//!                    DetectionOutput
//!                            │
//!                            ▼
//!                        diagnosis
//! ```
//!
//! # Write once, scale everywhere
//!
//! This registry deliberately contains no machine-size constants.
//!
//! It does not assume:
//!
//! - a fixed number of qubits;
//! - a fixed number of physical qubits;
//! - a fixed number of detectors;
//! - a fixed number of backends;
//! - a fixed number of devices;
//! - a fixed number of observations;
//! - a fixed number of signals;
//! - a fixed topology;
//! - a fixed execution width;
//! - a fixed retry count.
//!
//! "Infinity" in the architectural sense means that the registry introduces
//! no artificial finite ceiling. Actual memory, CPU, execution-policy and
//! security limits remain external resource constraints.
//!
//! # Important scalability rule
//!
//! The registry stores detector implementations, not observations.
//!
//! Observations remain caller-owned.
//!
//! ```text
//! registry
//!     └── detector implementations
//!
//! caller
//!     └── observations
//! ```
//!
//! Therefore registering a detector does not retain telemetry history.
//!
//! The streaming execution API additionally permits callers to process a
//! detector set without requiring the registry to materialize all outputs.
//!
//! # Determinism
//!
//! Detector registration and discovery use `BTreeMap`.
//!
//! Therefore detector ordering is determined by explicit detector identity:
//!
//! ```text
//! detector name
//! detector version
//! ```
//!
//! No ordering depends on:
//!
//! - hash-map iteration;
//! - memory addresses;
//! - thread scheduling;
//! - process IDs;
//! - timestamps;
//! - randomness;
//! - environment variables.
//!
//! The registry does not generate detector IDs.
//!
//! Detector identity is supplied by the detector implementation.
//!
//! # Explicit ownership
//!
//! The registry owns registered detector objects.
//!
//! There is intentionally no:
//!
//! ```text
//! static GLOBAL_REGISTRY
//! lazy_static!
//! OnceLock<...>
//! thread_local!
//! ```
//!
//! A caller creates and owns a `DetectorRegistry` explicitly.
//!
//! This prevents hidden global state and allows independent resilience
//! execution contexts, tests, simulations and distributed workers to maintain
//! independent detector sets.
//!
//! # Concurrency
//!
//! `DetectorObject` intentionally does not require `Send` or `Sync` in the
//! existing detection contract.
//!
//! Consequently this registry does not artificially add those bounds.
//!
//! Parallel execution should be implemented by an owning execution layer using
//! independent registry instances or an explicitly synchronized integration
//! boundary.
//!
//! The registry itself is deterministic and synchronous.
//!
//! # Detector identity
//!
//! The existing detector contract exposes:
//!
//! ```text
//! DetectorIdentity
//!     name
//!     version
//! ```
//!
//! The registry uses the complete identity as its registration key.
//!
//! This permits multiple versions of the same detector family to coexist:
//!
//! ```text
//! anomaly@1.0.0
//! anomaly@2.0.0
//! ```
//!
//! while preventing accidental duplicate registration of the exact same
//! detector identity.
//!
//! # Version handling
//!
//! The registry treats detector versions as opaque identity strings.
//!
//! It deliberately does not parse semantic versions because doing so would
//! duplicate compatibility policy.
//!
//! Version compatibility belongs to the resilience compatibility subsystem.
//!
//! # Lifecycle
//!
//! ```text
//! create registry
//!       │
//!       ▼
//! register detector
//!       │
//!       ▼
//! validate identity
//!       │
//!       ▼
//! deterministic registry
//!       │
//!       ├──────────────► lookup
//!       │
//!       ├──────────────► execution
//!       │
//!       ├──────────────► reset
//!       │
//!       └──────────────► unregister
//! ```
//!
//! # Detector identity stability
//!
//! The underlying `Detector` trait returns a reference to `DetectorIdentity`.
//!
//! The registry therefore snapshots the identity at registration time.
//!
//! Before and after detector execution, the registry verifies that the
//! implementation still reports the registered identity.
//!
//! If a detector changes its identity while registered, the registry reports
//! `ComponentIncompatible` instead of silently continuing with ambiguous
//! provenance.
//!
//! This protects:
//!
//! - deterministic provenance;
//! - telemetry;
//! - serialization;
//! - diagnosis;
//! - auditability.
//!
//! # Error ownership
//!
//! All fallible operations use:
//!
//! ```text
//! crate::quantum::resilience::errors
//! ```
//!
//! The registry does not define another error enum.
//!
//! Relevant canonical error codes include:
//!
//! ```text
//! InvalidArgument
//! InvalidIdentifier
//! InvalidState
//! ComponentFailure
//! ComponentUnavailable
//! ComponentIncompatible
//! ```
//!
//! # Canonical quantum identity
//!
//! This registry does not model qubits.
//!
//! It therefore intentionally does not import or redefine:
//!
//! ```text
//! QubitId
//! PhysicalQubitId
//! ```
//!
//! When a detector operates on quantum resources, the detector implementation
//! must use the canonical:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! through the appropriate detection/model integration.
//!
//! The registry must remain unaware of those resource details.
//!
//! # Integration with detection
//!
//! The registry consumes the existing stable contract:
//!
//! ```text
//! crate::quantum::resilience::detection::detector::DetectorObject
//! crate::quantum::resilience::detection::detector::DetectionContext
//! crate::quantum::resilience::detection::detector::DetectionObservation
//! crate::quantum::resilience::detection::detector::DetectionOutput
//! ```
//!
//! Concrete detectors implement `Detector` and are automatically usable
//! through `DetectorObject`.
//!
//! The registry therefore requires no changes when adding:
//!
//! ```text
//! threshold
//! anomaly
//! statistical
//! drift
//! timeout
//! execution_failure
//! qec_signal
//! hardware_signal
//! ```
//!
//! # Integration with diagnosis
//!
//! The registry only produces `DetectionOutput`.
//!
//! Diagnosis consumes those outputs.
//!
//! ```text
//! DetectorRegistry
//!       │
//!       ▼
//! DetectionOutput
//!       │
//!       ▼
//! diagnosis::diagnostician
//! ```
//!
//! No diagnosis implementation is imported here.
//!
//! # Integration with policy
//!
//! Registry membership is not policy.
//!
//! A policy may determine which registered detectors should participate in a
//! particular execution, but that decision must occur outside this registry.
//!
//! This prevents the registry from embedding hard-coded resilience policy.
//!
//! # Integration with telemetry
//!
//! Telemetry is an observation producer.
//!
//! The registry does not collect telemetry itself.
//!
//! ```text
//! telemetry
//!     │
//!     ▼
//! DetectionObservation
//!     │
//!     ▼
//! DetectorRegistry
//! ```
//!
//! # Integration with hardware
//!
//! Hardware providers remain behind `quantum::hardware`.
//!
//! A hardware detector can be registered without exposing provider SDK types
//! to this registry.
//!
//! ```text
//! provider adapter
//!       │
//!       ▼
//! quantum::hardware
//!       │
//!       ▼
//! hardware observation
//!       │
//!       ▼
//! hardware detector
//!       │
//!       ▼
//! DetectorRegistry
//! ```
//!
//! # Integration with QEC
//!
//! QEC detectors consume QEC-produced observations.
//!
//! The registry does not implement QEC and does not know the selected code,
//! decoder or logical layout.
//!
//! # Integration with ZQN
//!
//! ZQN remains the canonical quantum fault/noise semantic owner.
//!
//! A ZQN-aware detector may consume ZQN observations, but the registry does
//! not define another fault ontology.
//!
//! # Integration with simulation
//!
//! Simulators may construct independent registries containing the same detector
//! implementations as hardware execution.
//!
//! This makes resilience behavior testable without requiring a QPU.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may provide observations to registered detectors and may
//! compare detector outputs across target configurations.
//!
//! The registry does not own benchmark state.
//!
//! # Integration with serialization
//!
//! The registry does not serialize executable detector objects.
//!
//! Serialization of detector identity/configuration belongs to the appropriate
//! serialization/configuration subsystem.
//!
//! This avoids pretending that arbitrary Rust trait objects can be safely
//! persisted and reconstructed without an explicit implementation contract.
//!
//! # Plugin boundary
//!
//! A future plugin system can construct detector implementations and register
//! them here.
//!
//! The registry does not load dynamic libraries itself.
//!
//! Plugin loading, authentication, authorization, compatibility and isolation
//! belong to the plugin/security infrastructure.
//!
//! # No unsafe Rust
//!
//! This file forbids unsafe Rust at compile time.
//!
//! Required compatibility:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! =============================================================================
//! Compiler-enforced safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::btree_map::{BTreeMap, Entry};

use crate::quantum::resilience::detection::detector::{
    DetectionContext,
    DetectionObservation,
    DetectionOutput,
    DetectorIdentity,
    DetectorObject,
};
use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

// =============================================================================
// Public schema
// =============================================================================

/// Stable schema identifier for the detector registry.
pub const DETECTOR_REGISTRY_SCHEMA_ID: &str =
    "zamani.quantum.resilience.registry.detector";

/// Semantic version of the detector registry contract.
pub const DETECTOR_REGISTRY_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Detector key
// =============================================================================

/// Immutable registry key derived from a detector's complete identity.
///
/// The key deliberately contains both name and version so different detector
/// versions can coexist without overwriting one another.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DetectorKey {
    name: String,
    version: String,
}

impl DetectorKey {
    /// Creates a registry key from a detector identity.
    ///
    /// The identity is validated before the key is constructed.
    pub fn from_identity(identity: &DetectorIdentity) -> ResilienceResult<Self> {
        if identity.name().trim().is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidIdentifier,
                "detector name must not be empty",
            ));
        }

        if identity.version().trim().is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidIdentifier,
                "detector version must not be empty",
            ));
        }

        Ok(Self {
            name: identity.name().to_owned(),
            version: identity.version().to_owned(),
        })
    }

    /// Returns the detector name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the detector version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
}

impl std::fmt::Display for DetectorKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}@{}", self.name, self.version)
    }
}

// =============================================================================
// Registered detector
// =============================================================================

/// One detector owned by the registry.
///
/// The registration identity is intentionally stored independently from the
/// trait object's current identity so that identity mutation by a faulty
/// implementation can be detected.
struct RegisteredDetector {
    identity: DetectorIdentity,
    detector: Box<dyn DetectorObject>,
}

impl RegisteredDetector {
    /// Creates a registered detector after validating identity consistency.
    fn new(detector: Box<dyn DetectorObject>) -> ResilienceResult<Self> {
        let identity = detector.identity().clone();

        DetectorKey::from_identity(&identity)?;

        if detector.identity() != &identity {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ComponentIncompatible,
                "detector identity changed during registration",
            ));
        }

        Ok(Self {
            identity,
            detector,
        })
    }

    /// Verifies that the implementation still reports its registered identity.
    fn verify_identity(&self) -> ResilienceResult<()> {
        if self.detector.identity() != &self.identity {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ComponentIncompatible,
                "registered detector identity changed after registration",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Registration result
// =============================================================================

/// Result of registering a detector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DetectorRegistrationResult {
    /// A new detector was inserted.
    Registered,

    /// An exact detector identity was already registered.
    AlreadyRegistered,
}

impl DetectorRegistrationResult {
    /// Returns whether a new detector was inserted.
    #[must_use]
    pub const fn is_registered(self) -> bool {
        matches!(self, Self::Registered)
    }

    /// Returns whether the identity already existed.
    #[must_use]
    pub const fn is_already_registered(self) -> bool {
        matches!(self, Self::AlreadyRegistered)
    }
}

// =============================================================================
// Unregistration result
// =============================================================================

/// Result of removing a detector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DetectorUnregistrationResult {
    /// A detector was removed.
    Removed,

    /// No detector with that identity was registered.
    NotRegistered,
}

impl DetectorUnregistrationResult {
    /// Returns whether a detector was removed.
    #[must_use]
    pub const fn is_removed(self) -> bool {
        matches!(self, Self::Removed)
    }
}

// =============================================================================
// Detector registry
// =============================================================================

/// Explicit, deterministic registry of heterogeneous detector implementations.
///
/// # Ownership
///
/// The registry owns registered detector objects.
///
/// # Ordering
///
/// Iteration and execution are ordered by [`DetectorKey`].
///
/// # Threading
///
/// The registry does not introduce `Send`/`Sync` requirements beyond those of
/// the existing `DetectorObject` contract.
///
/// # Global state
///
/// No global registry exists. Every registry is an explicit value.
#[derive(Default)]
pub struct DetectorRegistry {
    detectors: BTreeMap<DetectorKey, RegisteredDetector>,
}

impl DetectorRegistry {
    /// Creates an empty detector registry.
    ///
    /// No detector implementations are implicitly registered.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of registered detector identities.
    #[must_use]
    pub fn len(&self) -> usize {
        self.detectors.len()
    }

    /// Returns whether the registry contains no detectors.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.detectors.is_empty()
    }

    /// Registers a detector if its exact identity is not already present.
    ///
    /// Registration is atomic from the caller's perspective: the detector is
    /// validated before the registry is modified.
    ///
    /// The registry does not replace an existing detector implicitly.
    pub fn register(
        &mut self,
        detector: Box<dyn DetectorObject>,
    ) -> ResilienceResult<DetectorRegistrationResult> {
        let registered = RegisteredDetector::new(detector)?;
        let key = DetectorKey::from_identity(&registered.identity)?;

        match self.detectors.entry(key) {
            Entry::Vacant(entry) => {
                entry.insert(registered);
                Ok(DetectorRegistrationResult::Registered)
            }
            Entry::Occupied(_) => Ok(DetectorRegistrationResult::AlreadyRegistered),
        }
    }

    /// Registers a detector and replaces an existing detector with the same
    /// complete identity.
    ///
    /// Replacement is explicit so that accidental duplicate registration can
    /// never silently alter an existing execution environment.
    pub fn register_or_replace(
        &mut self,
        detector: Box<dyn DetectorObject>,
    ) -> ResilienceResult<Option<Box<dyn DetectorObject>>> {
        let registered = RegisteredDetector::new(detector)?;
        let key = DetectorKey::from_identity(&registered.identity)?;

        let previous = self
            .detectors
            .insert(key, registered)
            .map(|old| old.detector);

        Ok(previous)
    }

    /// Removes a detector with the exact supplied identity.
    pub fn unregister(
        &mut self,
        identity: &DetectorIdentity,
    ) -> ResilienceResult<DetectorUnregistrationResult> {
        let key = DetectorKey::from_identity(identity)?;

        Ok(match self.detectors.remove(&key) {
            Some(_) => DetectorUnregistrationResult::Removed,
            None => DetectorUnregistrationResult::NotRegistered,
        })
    }

    /// Removes a detector by its exact name and version.
    pub fn unregister_by_key(
        &mut self,
        key: &DetectorKey,
    ) -> DetectorUnregistrationResult {
        match self.detectors.remove(key) {
            Some(_) => DetectorUnregistrationResult::Removed,
            None => DetectorUnregistrationResult::NotRegistered,
        }
    }

    /// Removes all registered detectors.
    ///
    /// This operation affects only this explicit registry instance.
    pub fn clear(&mut self) {
        self.detectors.clear();
    }

    /// Returns whether a detector with the exact identity is registered.
    pub fn contains(
        &self,
        identity: &DetectorIdentity,
    ) -> ResilienceResult<bool> {
        let key = DetectorKey::from_identity(identity)?;
        Ok(self.detectors.contains_key(&key))
    }

    /// Returns whether a detector with the supplied key is registered.
    #[must_use]
    pub fn contains_key(&self, key: &DetectorKey) -> bool {
        self.detectors.contains_key(key)
    }

    /// Returns an immutable detector reference by exact identity.
    ///
    /// The returned object remains owned by the registry.
    pub fn get(
        &self,
        identity: &DetectorIdentity,
    ) -> ResilienceResult<Option<&dyn DetectorObject>> {
        let key = DetectorKey::from_identity(identity)?;

        Ok(self
            .detectors
            .get(&key)
            .map(|registered| registered.detector.as_ref()))
    }

    /// Returns a mutable detector reference by exact identity.
    ///
    /// Mutable access is explicit because detectors may maintain detector-local
    /// state.
    pub fn get_mut(
        &mut self,
        identity: &DetectorIdentity,
    ) -> ResilienceResult<Option<&mut dyn DetectorObject>> {
        let key = DetectorKey::from_identity(identity)?;

        Ok(self
            .detectors
            .get_mut(&key)
            .map(|registered| registered.detector.as_mut()))
    }

    /// Returns a deterministic snapshot of all registered detector keys.
    ///
    /// The returned vector is a snapshot only. It does not expose or retain
    /// detector implementations.
    #[must_use]
    pub fn keys(&self) -> Vec<DetectorKey> {
        self.detectors.keys().cloned().collect()
    }

    /// Returns a deterministic snapshot of all detector identities.
    #[must_use]
    pub fn identities(&self) -> Vec<DetectorIdentity> {
        self.detectors
            .values()
            .map(|registered| registered.identity.clone())
            .collect()
    }

    /// Returns the detector identity at a registry key.
    #[must_use]
    pub fn identity_for_key(&self, key: &DetectorKey) -> Option<&DetectorIdentity> {
        self.detectors
            .get(key)
            .map(|registered| &registered.identity)
    }

    /// Resets all registered detectors in deterministic registry order.
    ///
    /// Resetting a detector only resets detector-local state. It does not
    /// modify resilience state, hardware state, execution state, policy or
    /// recovery state.
    pub fn reset_all(&mut self) -> ResilienceResult<()> {
        for registered in self.detectors.values_mut() {
            registered.verify_identity()?;
            registered.detector.reset();
            registered.verify_identity()?;
        }

        Ok(())
    }

    /// Resets one detector identified by its complete identity.
    pub fn reset(
        &mut self,
        identity: &DetectorIdentity,
    ) -> ResilienceResult<bool> {
        let key = DetectorKey::from_identity(identity)?;

        let Some(registered) = self.detectors.get_mut(&key) else {
            return Ok(false);
        };

        registered.verify_identity()?;
        registered.detector.reset();
        registered.verify_identity()?;

        Ok(true)
    }

    /// Returns whether every registered detector currently reports itself as
    /// available.
    ///
    /// An empty registry is considered available because it contains no
    /// unavailable component.
    pub fn all_available(&self) -> ResilienceResult<bool> {
        for registered in self.detectors.values() {
            registered.verify_identity()?;

            if !registered.detector.is_available() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Returns the number of currently available detectors.
    ///
    /// No detector is removed merely because it is unavailable.
    #[must_use]
    pub fn available_count(&self) -> usize {
        self.detectors
            .values()
            .filter(|registered| registered.detector.is_available())
            .count()
    }

    /// Returns the number of currently unavailable detectors.
    #[must_use]
    pub fn unavailable_count(&self) -> usize {
        self.detectors
            .values()
            .filter(|registered| !registered.detector.is_available())
            .count()
    }

    /// Detects using one exact detector identity.
    ///
    /// An unavailable detector is reported as `ComponentUnavailable` rather
    /// than silently producing an empty detection result.
    pub fn detect(
        &mut self,
        identity: &DetectorIdentity,
        context: &DetectionContext,
        observations: &[DetectionObservation],
    ) -> ResilienceResult<DetectionOutput> {
        let key = DetectorKey::from_identity(identity)?;

        let Some(registered) = self.detectors.get_mut(&key) else {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ComponentUnavailable,
                format!("detector `{key}` is not registered"),
            ));
        };

        registered.verify_identity()?;

        if !registered.detector.is_available() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ComponentUnavailable,
                format!("detector `{key}` is currently unavailable"),
            ));
        }

        let output = registered
            .detector
            .detect_slice(context, observations)
            .map_err(|error| {
                ResilienceError::new(
                    ResilienceErrorCode::ComponentFailure,
                    format!("detector `{key}` failed: {error}"),
                )
            })?;

        registered.verify_identity()?;

        Ok(output)
    }

    /// Executes all currently available detectors in deterministic registry
    /// order.
    ///
    /// The callback receives each output immediately, avoiding a requirement to
    /// materialize all detector outputs in memory.
    ///
    /// This is the preferred API for large detector sets.
    ///
    /// The callback must not attempt to mutably access this same registry.
    pub fn detect_each<F>(
        &mut self,
        context: &DetectionContext,
        observations: &[DetectionObservation],
        mut consume: F,
    ) -> ResilienceResult<()>
    where
        F: FnMut(&DetectorKey, DetectionOutput) -> ResilienceResult<()>,
    {
        for (key, registered) in &mut self.detectors {
            registered.verify_identity()?;

            if !registered.detector.is_available() {
                continue;
            }

            let output = registered
                .detector
                .detect_slice(context, observations)
                .map_err(|error| {
                    ResilienceError::new(
                        ResilienceErrorCode::ComponentFailure,
                        format!("detector `{key}` failed: {error}"),
                    )
                })?;

            registered.verify_identity()?;

            consume(key, output)?;
        }

        Ok(())
    }

    /// Executes all currently available detectors and collects their outputs.
    ///
    /// This is a convenience API. For very large detector populations, prefer
    /// [`Self::detect_each`] so output retention is controlled by the caller.
    pub fn detect_all(
        &mut self,
        context: &DetectionContext,
        observations: &[DetectionObservation],
    ) -> ResilienceResult<Vec<(DetectorKey, DetectionOutput)>> {
        let mut outputs = Vec::new();

        self.detect_each(context, observations, |key, output| {
            outputs.push((key.clone(), output));
            Ok(())
        })?;

        Ok(outputs)
    }

    /// Executes one detector using a caller-provided observation slice without
    /// changing detector registration state.
    ///
    /// This is an explicit convenience wrapper around [`Self::detect`].
    pub fn detect_key(
        &mut self,
        key: &DetectorKey,
        context: &DetectionContext,
        observations: &[DetectionObservation],
    ) -> ResilienceResult<DetectionOutput> {
        let identity = self
            .identity_for_key(key)
            .cloned()
            .ok_or_else(|| {
                ResilienceError::new(
                    ResilienceErrorCode::ComponentUnavailable,
                    format!("detector `{key}` is not registered"),
                )
            })?;

        self.detect(&identity, context, observations)
    }

    /// Validates all registered detector identities.
    ///
    /// This can be used at an execution boundary before detector execution or
    /// before provenance capture.
    pub fn validate(&self) -> ResilienceResult<()> {
        for (key, registered) in &self.detectors {
            registered.verify_identity()?;

            let derived = DetectorKey::from_identity(&registered.identity)?;

            if &derived != key {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::InvariantViolation,
                    format!(
                        "registry key `{key}` does not match registered detector identity `{}`",
                        registered.identity
                    ),
                ));
            }
        }

        Ok(())
    }
}

// =============================================================================
// IntoIterator
// =============================================================================

/// Deterministic immutable iteration over registered detector identities.
///
/// Implemented as an identity iterator rather than an iterator over executable
/// trait objects so callers cannot accidentally depend on internal ownership.
impl<'a> IntoIterator for &'a DetectorRegistry {
    type Item = &'a DetectorIdentity;
    type IntoIter = std::vec::IntoIter<&'a DetectorIdentity>;

    fn into_iter(self) -> Self::IntoIter {
        self.detectors
            .values()
            .map(|registered| &registered.identity)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use core::num::NonZeroU64;

    use crate::quantum::resilience::detection::detector::{
        DetectionClassification,
        DetectionConfidence,
        DetectionInput,
        DetectionMetadata,
        DetectionSignal,
        ObservationFreshness,
        ObservationPayload,
        ObservationSource,
        ObservationTrust,
        ObservationId,
        DetectionSequence,
        Detector,
    };

    fn sequence(value: u64) -> DetectionSequence {
        DetectionSequence::new(
            NonZeroU64::new(value).expect("test sequence must be non-zero"),
        )
    }

    fn observation_id(value: u64) -> ObservationId {
        ObservationId::new(
            NonZeroU64::new(value).expect("test observation ID must be non-zero"),
        )
    }

    fn signal_id(value: u64) -> crate::quantum::resilience::detection::detector::SignalId {
        crate::quantum::resilience::detection::detector::SignalId::new(
            NonZeroU64::new(value).expect("test signal ID must be non-zero"),
        )
    }

    fn observation(value: u64) -> DetectionObservation {
        DetectionObservation::new(
            observation_id(value),
            sequence(1),
            ObservationSource::Runtime,
            ObservationTrust::Verified,
            ObservationFreshness::Fresh,
            ObservationPayload::Unsigned(value as u128),
        )
        .expect("test observation must be valid")
    }

    #[derive(Debug)]
    struct TestDetector {
        identity: DetectorIdentity,
        available: bool,
    }

    impl TestDetector {
        fn new(name: &str, version: &str) -> Self {
            Self {
                identity: DetectorIdentity::new(name, version)
                    .expect("test detector identity must be valid"),
                available: true,
            }
        }
    }

    impl Detector for TestDetector {
        fn identity(&self) -> &DetectorIdentity {
            &self.identity
        }

        fn detect<'a, I>(
            &mut self,
            input: DetectionInput<'a, I>,
        ) -> ResilienceResult<DetectionOutput>
        where
            I: Iterator<Item = &'a DetectionObservation>,
        {
            let context = input.context();
            let mut examined = 0_u64;

            for _ in input.observations() {
                examined = examined
                    .checked_add(1)
                    .ok_or_else(|| {
                        ResilienceError::new(
                            ResilienceErrorCode::ArithmeticOverflow,
                            "test detector observation counter overflowed",
                        )
                    })?;
            }

            let signal = DetectionSignal::new(
                signal_id(1),
                self.identity.clone(),
                DetectionClassification::Anomaly,
                DetectionConfidence::full(),
                None,
                context.sequence(),
            );

            Ok(DetectionOutput::new(
                DetectionMetadata::new(
                    self.identity.clone(),
                    context.sequence(),
                    examined,
                ),
                vec![signal],
            ))
        }

        fn is_available(&self) -> bool {
            self.available
        }
    }

    #[test]
    fn empty_registry_is_valid() {
        let registry = DetectorRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
        assert_eq!(registry.available_count(), 0);
        assert_eq!(registry.unavailable_count(), 0);
        assert!(registry.validate().is_ok());
    }

    #[test]
    fn detector_can_be_registered() {
        let mut registry = DetectorRegistry::new();

        let detector = Box::new(TestDetector::new("test", "1.0.0"));

        let result = registry.register(detector);

        assert_eq!(
            result.expect("registration should succeed"),
            DetectorRegistrationResult::Registered
        );
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn duplicate_exact_identity_is_not_registered_twice() {
        let mut registry = DetectorRegistry::new();

        let first = Box::new(TestDetector::new("test", "1.0.0"));
        let second = Box::new(TestDetector::new("test", "1.0.0"));

        assert_eq!(
            registry
                .register(first)
                .expect("first registration should succeed"),
            DetectorRegistrationResult::Registered
        );

        assert_eq!(
            registry
                .register(second)
                .expect("duplicate registration should be handled"),
            DetectorRegistrationResult::AlreadyRegistered
        );

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn different_versions_can_coexist() {
        let mut registry = DetectorRegistry::new();

        let first = Box::new(TestDetector::new("test", "1.0.0"));
        let second = Box::new(TestDetector::new("test", "2.0.0"));

        registry
            .register(first)
            .expect("first registration should succeed");

        registry
            .register(second)
            .expect("second version should succeed");

        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn identities_are_deterministically_ordered() {
        let mut registry = DetectorRegistry::new();

        registry
            .register(Box::new(TestDetector::new("zeta", "1.0.0")))
            .expect("zeta registration should succeed");

        registry
            .register(Box::new(TestDetector::new("alpha", "1.0.0")))
            .expect("alpha registration should succeed");

        registry
            .register(Box::new(TestDetector::new("beta", "1.0.0")))
            .expect("beta registration should succeed");

        let identities = registry.identities();

        assert_eq!(identities[0].name(), "alpha");
        assert_eq!(identities[1].name(), "beta");
        assert_eq!(identities[2].name(), "zeta");
    }

    #[test]
    fn exact_identity_can_be_found() {
        let mut registry = DetectorRegistry::new();

        let identity =
            DetectorIdentity::new("test", "1.0.0").expect("identity should be valid");

        registry
            .register(Box::new(TestDetector::new("test", "1.0.0")))
            .expect("registration should succeed");

        assert!(
            registry
                .contains(&identity)
                .expect("contains should succeed")
        );

        assert!(
            registry
                .get(&identity)
                .expect("get should succeed")
                .is_some()
        );
    }

    #[test]
    fn missing_detector_returns_component_unavailable() {
        let mut registry = DetectorRegistry::new();

        let identity =
            DetectorIdentity::new("missing", "1.0.0").expect("identity should be valid");

        let context = DetectionContext::new(sequence(1), false, false);

        let result = registry.detect(&identity, &context, &[]);

        let error = result.expect_err("missing detector must fail");

        assert_eq!(
            error.code(),
            ResilienceErrorCode::ComponentUnavailable
        );
    }

    #[test]
    fn unavailable_detector_returns_component_unavailable() {
        struct UnavailableDetector {
            identity: DetectorIdentity,
        }

        impl Detector for UnavailableDetector {
            fn identity(&self) -> &DetectorIdentity {
                &self.identity
            }

            fn detect<'a, I>(
                &mut self,
                input: DetectionInput<'a, I>,
            ) -> ResilienceResult<DetectionOutput>
            where
                I: Iterator<Item = &'a DetectionObservation>,
            {
                let context = input.context();

                Ok(DetectionOutput::new(
                    DetectionMetadata::new(
                        self.identity.clone(),
                        context.sequence(),
                        0,
                    ),
                    Vec::new(),
                ))
            }

            fn is_available(&self) -> bool {
                false
            }
        }

        let mut registry = DetectorRegistry::new();

        let identity =
            DetectorIdentity::new("unavailable", "1.0.0")
                .expect("identity should be valid");

        registry
            .register(Box::new(UnavailableDetector {
                identity: identity.clone(),
            }))
            .expect("registration should succeed");

        let context = DetectionContext::new(sequence(1), false, false);

        let error = registry
            .detect(&identity, &context, &[])
            .expect_err("unavailable detector must fail");

        assert_eq!(
            error.code(),
            ResilienceErrorCode::ComponentUnavailable
        );
    }

    #[test]
    fn detector_execution_is_supported() {
        let mut registry = DetectorRegistry::new();

        let identity =
            DetectorIdentity::new("test", "1.0.0").expect("identity should be valid");

        registry
            .register(Box::new(TestDetector::new("test", "1.0.0")))
            .expect("registration should succeed");

        let context = DetectionContext::new(sequence(1), false, false);

        let observations = vec![observation(1), observation(2)];

        let output = registry
            .detect(&identity, &context, &observations)
            .expect("detection should succeed");

        assert_eq!(output.metadata().observations_examined(), 2);
        assert_eq!(output.len(), 1);
    }

    #[test]
    fn detect_all_is_deterministic() {
        let mut registry = DetectorRegistry::new();

        registry
            .register(Box::new(TestDetector::new("zeta", "1.0.0")))
            .expect("zeta registration should succeed");

        registry
            .register(Box::new(TestDetector::new("alpha", "1.0.0")))
            .expect("alpha registration should succeed");

        let context = DetectionContext::new(sequence(1), false, false);

        let outputs = registry
            .detect_all(&context, &[])
            .expect("detection should succeed");

        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0].0.name(), "alpha");
        assert_eq!(outputs[1].0.name(), "zeta");
    }

    #[test]
    fn detect_each_can_stream_outputs_to_caller() {
        let mut registry = DetectorRegistry::new();

        registry
            .register(Box::new(TestDetector::new("alpha", "1.0.0")))
            .expect("registration should succeed");

        registry
            .register(Box::new(TestDetector::new("beta", "1.0.0")))
            .expect("registration should succeed");

        let context = DetectionContext::new(sequence(1), false, false);

        let mut names = Vec::new();

        registry
            .detect_each(&context, &[], |key, _output| {
                names.push(key.name().to_owned());
                Ok(())
            })
            .expect("streamed detection should succeed");

        assert_eq!(names, vec!["alpha", "beta"]);
    }

    #[test]
    fn reset_does_not_remove_detector() {
        let mut registry = DetectorRegistry::new();

        let identity =
            DetectorIdentity::new("test", "1.0.0").expect("identity should be valid");

        registry
            .register(Box::new(TestDetector::new("test", "1.0.0")))
            .expect("registration should succeed");

        assert!(
            registry
                .reset(&identity)
                .expect("reset should succeed")
        );

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn unregister_removes_only_exact_identity() {
        let mut registry = DetectorRegistry::new();

        let first =
            DetectorIdentity::new("test", "1.0.0").expect("identity should be valid");

        let second =
            DetectorIdentity::new("test", "2.0.0").expect("identity should be valid");

        registry
            .register(Box::new(TestDetector::new("test", "1.0.0")))
            .expect("first registration should succeed");

        registry
            .register(Box::new(TestDetector::new("test", "2.0.0")))
            .expect("second registration should succeed");

        assert_eq!(
            registry
                .unregister(&first)
                .expect("unregister should succeed"),
            DetectorUnregistrationResult::Removed
        );

        assert!(
            !registry
                .contains(&first)
                .expect("contains should succeed")
        );

        assert!(
            registry
                .contains(&second)
                .expect("contains should succeed")
        );
    }

    #[test]
    fn register_or_replace_is_explicit() {
        let mut registry = DetectorRegistry::new();

        let identity =
            DetectorIdentity::new("test", "1.0.0").expect("identity should be valid");

        let previous = registry
            .register_or_replace(Box::new(TestDetector::new("test", "1.0.0")))
            .expect("initial replacement should succeed");

        assert!(previous.is_none());

        let previous = registry
            .register_or_replace(Box::new(TestDetector::new("test", "1.0.0")))
            .expect("replacement should succeed");

        assert!(previous.is_some());

        assert!(
            registry
                .contains(&identity)
                .expect("contains should succeed")
        );
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn registry_can_be_cleared() {
        let mut registry = DetectorRegistry::new();

        registry
            .register(Box::new(TestDetector::new("test", "1.0.0")))
            .expect("registration should succeed");

        registry.clear();

        assert!(registry.is_empty());
    }

    #[test]
    fn all_available_is_true_for_available_detectors() {
        let mut registry = DetectorRegistry::new();

        registry
            .register(Box::new(TestDetector::new("test", "1.0.0")))
            .expect("registration should succeed");

        assert!(
            registry
                .all_available()
                .expect("availability check should succeed")
        );
    }
}