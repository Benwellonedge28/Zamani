//! Zamani Quantum Resilience — Strategy Registry
//!
//! Path:
//!     src/quantum/resilience/registry/strategy.rs
//!
//! Purpose:
//!     Production-grade registry for provider-independent quantum-resilience
//!     mitigation strategies.
//!
//! Architectural role:
//!
//!     Strategy implementation
//!             |
//!             v
//!     MitigationStrategy
//!             |
//!             v
//!     StrategyRegistry
//!             |
//!       +-----+-----+
//!       |           |
//!       v           v
//!   Selection   Planning
//!       |           |
//!       +-----+-----+
//!             |
//!             v
//!        Executor
//!
//! This registry:
//!
//! - owns registration and lifecycle of strategy implementations;
//! - provides stable identity/version lookup;
//! - prevents accidental duplicate registrations;
//! - supports explicit replacement;
//! - preserves deterministic iteration order;
//! - supports concurrent read access through Arc-backed strategies;
//! - never executes quantum hardware operations;
//! - never performs routing;
//! - never performs scheduling;
//! - never performs compilation;
//! - never performs QEC;
//! - never selects a strategy implicitly;
//! - never contains provider-specific branches;
//! - never introduces fixed machine-size limits;
//! - never introduces a competing qubit identity type;
//! - never uses unsafe Rust.
//!
//! Rust:
//!     1.97 / 1.97.1
//! Edition:
//!     2021
//!
//! Scalability:
//!     The registry has no artificial limit on the number of strategies.
//!     Actual limits are those imposed by available memory/resources and
//!     explicit application policy.
//!
//! Canonical quantum identity:
//!     Strategy implementations and scopes use
//!     crate::quantum::ir::qubit::QubitId where logical qubit identity is
//!     required. This registry itself deliberately remains qubit-agnostic.
//!
//! Integration contracts:
//!
//!     crate::quantum::resilience::mitigation::strategy::MitigationStrategy
//!         Canonical strategy implementation contract.
//!
//!     crate::quantum::resilience::mitigation::strategy::StrategyDescriptor
//!         Immutable strategy metadata and identity.
//!
//!     crate::quantum::resilience::mitigation::strategy::StrategyId
//!         Stable strategy identifier.
//!
//!     crate::quantum::resilience::mitigation::strategy::StrategyVersion
//!         Strategy semantic version.
//!
//!     crate::quantum::resilience::mitigation::selection
//!         Consumes registry entries and performs policy/capability-aware
//!         strategy selection.
//!
//!     crate::quantum::resilience::mitigation::executor
//!         Executes a strategy selected elsewhere.
//!
//!     crate::quantum::resilience::planning
//!         Treats registered strategies as candidate resilience actions.
//!
//!     crate::quantum::resilience::verification
//!         Verifies the result of an executed strategy.
//!
//!     crate::quantum::resilience::telemetry
//!         Records strategy identity/version and execution outcomes.
//!
//!     crate::quantum::resilience::history
//!         Records strategy performance and historical outcomes.
//!
//!     crate::quantum::resilience::serialization
//!         Serializes strategy identity/metadata, not executable trait objects.
//!
//! Important:
//!     The registry is not a plugin loader. Loading dynamic libraries,
//!     authenticating external plugins, sandboxing plugins, and resolving
//!     executable modules belong to a separate integration boundary.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::btree_map::{Entry, Iter, IterMut};
use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use crate::quantum::resilience::errors::{ResilienceError, ResilienceErrorCode, ResilienceResult};

use crate::quantum::resilience::mitigation::strategy::{
    Applicability,
    MitigationStrategy,
    StrategyDescriptor,
    StrategyId,
    StrategyVersion,
    StrategyContext,
    StrategyEvaluation,
};

// ============================================================================
// Stable registry schema
// ============================================================================

/// Stable schema identifier for the strategy registry.
pub const STRATEGY_REGISTRY_SCHEMA_ID: &str =
    "zamani.quantum.resilience.registry.strategy";

/// Semantic version of the registry contract.
///
/// This version changes only when the externally observable registry contract
/// changes.
pub const STRATEGY_REGISTRY_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Strategy key
// ============================================================================

/// Stable key identifying one concrete strategy implementation.
///
/// Strategy identity is deliberately the pair:
///
///     StrategyId + StrategyVersion
///
/// Multiple versions of the same strategy may coexist.
///
/// This is important for:
///
/// - reproducibility;
/// - deterministic replay;
/// - rolling upgrades;
/// - checkpoint compatibility;
/// - distributed execution;
/// - historical provenance.
///
/// The registry never assumes that the newest version is automatically better.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StrategyKey {
    id: StrategyId,
    version: StrategyVersion,
}

impl StrategyKey {
    /// Creates a strategy key from an existing canonical strategy identity.
    #[must_use]
    pub fn new(id: StrategyId, version: StrategyVersion) -> Self {
        Self { id, version }
    }

    /// Creates a strategy key from a strategy descriptor.
    #[must_use]
    pub fn from_descriptor(descriptor: &StrategyDescriptor) -> Self {
        Self::new(descriptor.id.clone(), descriptor.version)
    }

    /// Returns the strategy identifier.
    #[must_use]
    pub fn id(&self) -> &StrategyId {
        &self.id
    }

    /// Returns the strategy version.
    #[must_use]
    pub const fn version(&self) -> StrategyVersion {
        self.version
    }

    /// Returns the string form of the strategy identifier.
    #[must_use]
    pub fn id_str(&self) -> &str {
        self.id.as_str()
    }
}

impl fmt::Display for StrategyKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}@{}", self.id, self.version)
    }
}

// ============================================================================
// Registry
// ============================================================================

/// Production registry of immutable, thread-safe mitigation strategies.
///
/// Strategies are stored behind `Arc` because the canonical
/// `MitigationStrategy` contract is `Send + Sync`.
///
/// The registry itself is intentionally not internally locked.
///
/// This is deliberate:
///
/// - read-only strategy implementations are safe to share;
/// - registry mutation is an administrative operation;
/// - execution/selection should not be serialized behind one global mutex;
/// - distributed deployments can use independent registry instances;
/// - deterministic snapshots can be taken without hidden synchronization.
///
/// If a caller needs concurrent mutation, synchronization belongs at the
/// application/service boundary rather than being hidden inside this domain
/// type.
pub struct StrategyRegistry {
    strategies: BTreeMap<StrategyKey, Arc<dyn MitigationStrategy>>,
}

impl Default for StrategyRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for StrategyRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let keys: Vec<&StrategyKey> = self.strategies.keys().collect();

        formatter
            .debug_struct("StrategyRegistry")
            .field("schema_id", &STRATEGY_REGISTRY_SCHEMA_ID)
            .field("schema_version", &STRATEGY_REGISTRY_SCHEMA_VERSION)
            .field("strategy_count", &self.strategies.len())
            .field("keys", &keys)
            .finish()
    }
}

impl StrategyRegistry {
    /// Creates an empty registry.
    ///
    /// No capacity is preallocated and no artificial strategy limit exists.
    #[must_use]
    pub fn new() -> Self {
        Self {
            strategies: BTreeMap::new(),
        }
    }

    /// Returns the number of registered strategies.
    #[must_use]
    pub fn len(&self) -> usize {
        self.strategies.len()
    }

    /// Returns whether the registry contains no strategies.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.strategies.is_empty()
    }

    /// Registers a strategy.
    ///
    /// Registration is intentionally strict:
    ///
    /// - exact duplicate identity/version is rejected;
    /// - replacing an implementation requires an explicit `replace`;
    /// - the descriptor identity is captured from the implementation itself;
    /// - the registry never permits a caller to register an implementation
    ///   under an identity different from the implementation's descriptor.
    pub fn register<S>(&mut self, strategy: S) -> ResilienceResult<StrategyKey>
    where
        S: MitigationStrategy + 'static,
    {
        self.register_arc(Arc::new(strategy))
    }

    /// Registers an already shared strategy implementation.
    ///
    /// This is useful when a strategy is constructed by a dependency-injection
    /// or application/plugin boundary.
    pub fn register_arc(
        &mut self,
        strategy: Arc<dyn MitigationStrategy>,
    ) -> ResilienceResult<StrategyKey> {
        let descriptor = strategy.descriptor();
        let key = Self::validated_key(descriptor)?;

        match self.strategies.entry(key.clone()) {
            Entry::Vacant(slot) => {
                slot.insert(strategy);
                Ok(key)
            }

            Entry::Occupied(_) => Err(Self::already_exists(&key)),
        }
    }

    /// Explicitly replaces the implementation registered under an exact
    /// strategy identity/version.
    ///
    /// Replacement is never implicit.
    ///
    /// The incoming strategy's descriptor must match the supplied identity
    /// because the key is derived directly from its descriptor.
    ///
    /// Returns the previous implementation when one existed.
    pub fn replace<S>(
        &mut self,
        strategy: S,
    ) -> ResilienceResult<Option<Arc<dyn MitigationStrategy>>>
    where
        S: MitigationStrategy + 'static,
    {
        self.replace_arc(Arc::new(strategy))
    }

    /// Explicitly replaces an already shared strategy.
    pub fn replace_arc(
        &mut self,
        strategy: Arc<dyn MitigationStrategy>,
    ) -> ResilienceResult<Option<Arc<dyn MitigationStrategy>>> {
        let descriptor = strategy.descriptor();
        let key = Self::validated_key(descriptor)?;

        match self.strategies.entry(key.clone()) {
            Entry::Vacant(slot) => {
                slot.insert(strategy);
                Ok(None)
            }

            Entry::Occupied(mut slot) => {
                Self::ensure_identity(&key, slot.get().as_ref())?;

                let previous = slot.insert(strategy);
                Ok(Some(previous))
            }
        }
    }

    /// Removes an exact strategy identity/version.
    ///
    /// The registry validates the currently stored descriptor before removal
    /// so an implementation whose identity has unexpectedly drifted cannot be
    /// silently removed under a false identity.
    pub fn remove(
        &mut self,
        key: &StrategyKey,
    ) -> ResilienceResult<Arc<dyn MitigationStrategy>> {
        match self.strategies.get(key) {
            Some(strategy) => {
                Self::ensure_identity(key, strategy.as_ref())?;
            }

            None => {
                return Err(Self::not_found(key));
            }
        }

        self.strategies
            .remove(key)
            .ok_or_else(|| Self::not_found(key))
    }

    /// Removes every registered strategy.
    ///
    /// Returns the number of entries removed.
    ///
    /// No strategy-specific shutdown is performed because the
    /// `MitigationStrategy` contract intentionally does not own execution
    /// resources or lifecycle side effects.
    pub fn clear(&mut self) -> usize {
        let count = self.strategies.len();
        self.strategies.clear();
        count
    }

    /// Returns whether an exact strategy identity/version is registered.
    #[must_use]
    pub fn contains(&self, key: &StrategyKey) -> bool {
        self.strategies.contains_key(key)
    }

    /// Returns a registered strategy by exact identity/version.
    ///
    /// The returned `Arc` is a cheap shared handle and does not expose mutable
    /// strategy state.
    pub fn get(
        &self,
        key: &StrategyKey,
    ) -> ResilienceResult<Arc<dyn MitigationStrategy>> {
        let strategy = self
            .strategies
            .get(key)
            .ok_or_else(|| Self::not_found(key))?;

        Self::ensure_identity(key, strategy.as_ref())?;

        Ok(Arc::clone(strategy))
    }

    /// Returns all registered keys in deterministic order.
    ///
    /// Ordering is lexicographic by strategy identifier and then semantic
    /// version.
    pub fn keys(&self) -> impl Iterator<Item = &StrategyKey> {
        self.strategies.keys()
    }

    /// Returns all strategy descriptors in deterministic registry order.
    ///
    /// This creates a caller-owned collection. For very large registries,
    /// prefer `for_each_descriptor`.
    pub fn descriptors(&self) -> ResilienceResult<Vec<StrategyDescriptor>> {
        let mut descriptors = Vec::with_capacity(self.strategies.len());

        for (key, strategy) in &self.strategies {
            Self::ensure_identity(key, strategy.as_ref())?;
            descriptors.push(strategy.descriptor().clone());
        }

        Ok(descriptors)
    }

    /// Streams descriptors without constructing one large descriptor vector.
    ///
    /// This is the preferred API when the registry is large.
    pub fn for_each_descriptor<F>(
        &self,
        mut consumer: F,
    ) -> ResilienceResult<()>
    where
        F: FnMut(&StrategyKey, &StrategyDescriptor) -> ResilienceResult<()>,
    {
        for (key, strategy) in &self.strategies {
            Self::ensure_identity(key, strategy.as_ref())?;

            consumer(key, strategy.descriptor())?;
        }

        Ok(())
    }

    /// Returns an immutable iterator over registered strategies.
    ///
    /// Iteration order is deterministic.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&StrategyKey, &Arc<dyn MitigationStrategy>)> {
        self.strategies.iter()
    }

    /// Returns the underlying map iterator.
    ///
    /// This is exposed primarily for internal composition and advanced
    /// registry tooling.
    pub(crate) fn iter_internal(
        &self,
    ) -> Iter<'_, StrategyKey, Arc<dyn MitigationStrategy>> {
        self.strategies.iter()
    }

    /// Returns a mutable map iterator for registry-internal administration.
    ///
    /// This is crate-visible rather than public because mutable access to
    /// strategy implementations would undermine the registry's identity
    /// invariant.
    pub(crate) fn iter_internal_mut(
        &mut self,
    ) -> IterMut<'_, StrategyKey, Arc<dyn MitigationStrategy>> {
        self.strategies.iter_mut()
    }

    /// Evaluates one exact strategy against a context.
    ///
    /// Evaluation is not execution.
    ///
    /// The strategy may determine:
    ///
    /// - applicability;
    /// - required capabilities;
    /// - expected overhead;
    /// - policy requirements;
    /// - determinism properties.
    ///
    /// Actual execution remains the responsibility of the mitigation executor.
    pub fn evaluate(
        &self,
        key: &StrategyKey,
        context: &StrategyContext,
    ) -> ResilienceResult<StrategyEvaluation> {
        let strategy = self
            .strategies
            .get(key)
            .ok_or_else(|| Self::not_found(key))?;

        Self::ensure_identity(key, strategy.as_ref())?;

        let evaluation = strategy.evaluate(context);

        Self::ensure_identity(key, strategy.as_ref())?;

        Ok(evaluation)
    }

    /// Evaluates every registered strategy in deterministic order.
    ///
    /// The callback receives each evaluation immediately, avoiding the need
    /// to retain all evaluations simultaneously.
    ///
    /// This is the preferred API for large registries.
    pub fn evaluate_each<F>(
        &self,
        context: &StrategyContext,
        mut consumer: F,
    ) -> ResilienceResult<()>
    where
        F: FnMut(
            &StrategyKey,
            &StrategyDescriptor,
            &StrategyEvaluation,
        ) -> ResilienceResult<()>,
    {
        for (key, strategy) in &self.strategies {
            Self::ensure_identity(key, strategy.as_ref())?;

            let evaluation = strategy.evaluate(context);

            Self::ensure_identity(key, strategy.as_ref())?;

            consumer(key, strategy.descriptor(), &evaluation)?;
        }

        Ok(())
    }

    /// Evaluates every registered strategy and returns all evaluations.
    ///
    /// This is a convenience API. For very large registries or high-frequency
    /// adaptive execution, prefer `evaluate_each`.
    pub fn evaluate_all(
        &self,
        context: &StrategyContext,
    ) -> ResilienceResult<Vec<(StrategyKey, StrategyEvaluation)>> {
        let mut evaluations = Vec::with_capacity(self.strategies.len());

        for (key, strategy) in &self.strategies {
            Self::ensure_identity(key, strategy.as_ref())?;

            let evaluation = strategy.evaluate(context);

            Self::ensure_identity(key, strategy.as_ref())?;

            evaluations.push((key.clone(), evaluation));
        }

        Ok(evaluations)
    }

    /// Evaluates only strategies belonging to a requested family.
    ///
    /// Family filtering remains a selection concern; this method exists only
    /// as an efficient registry query primitive.
    pub fn evaluate_where<F>(
        &self,
        context: &StrategyContext,
        predicate: F,
    ) -> ResilienceResult<Vec<(StrategyKey, StrategyEvaluation)>>
    where
        F: Fn(&StrategyDescriptor) -> bool,
    {
        let mut evaluations = Vec::new();

        for (key, strategy) in &self.strategies {
            let descriptor = strategy.descriptor();

            if !predicate(descriptor) {
                continue;
            }

            Self::ensure_identity(key, strategy.as_ref())?;

            let evaluation = strategy.evaluate(context);

            Self::ensure_identity(key, strategy.as_ref())?;

            evaluations.push((key.clone(), evaluation));
        }

        Ok(evaluations)
    }

    /// Returns all strategies whose abstract evaluation is a candidate.
    ///
    /// This does NOT select a winner.
    ///
    /// Selection remains owned by `mitigation/selection.rs` and policy/planning.
    pub fn candidates(
        &self,
        context: &StrategyContext,
    ) -> ResilienceResult<Vec<(StrategyKey, StrategyEvaluation)>> {
        self.evaluate_where(context, |descriptor| {
            let _ = descriptor;
            true
        })
        .map(|evaluations| {
            evaluations
                .into_iter()
                .filter(|(_, evaluation)| evaluation.applicability().is_candidate())
                .collect()
        })
    }

    /// Returns all strategies that are statically marked deterministic.
    ///
    /// This is metadata filtering only. It does not guarantee that an
    /// execution is reproducible because runtime state, target state and
    /// external observations can still affect execution.
    pub fn deterministic_keys(&self) -> impl Iterator<Item = &StrategyKey> {
        self.strategies.iter().filter_map(|(key, strategy)| {
            if strategy.descriptor().deterministic {
                Some(key)
            } else {
                None
            }
        })
    }

    /// Returns the number of registered strategies belonging to a family.
    pub fn count_family(
        &self,
        family: crate::quantum::resilience::mitigation::strategy::StrategyFamily,
    ) -> usize {
        self.strategies
            .values()
            .filter(|strategy| strategy.descriptor().family == family)
            .count()
    }

    // ------------------------------------------------------------------------
    // Internal validation
    // ------------------------------------------------------------------------

    fn validated_key(
        descriptor: &StrategyDescriptor,
    ) -> ResilienceResult<StrategyKey> {
        let id = descriptor.id.as_str();

        if id.is_empty() {
            return Err(Self::validation_error(
                "strategy descriptor contains an empty strategy identifier",
            ));
        }

        if id.chars().any(char::is_control) {
            return Err(Self::validation_error(
                "strategy identifier contains a control character",
            ));
        }

        let key = StrategyKey::from_descriptor(descriptor);

        Ok(key)
    }

    fn ensure_identity(
        expected: &StrategyKey,
        strategy: &dyn MitigationStrategy,
    ) -> ResilienceResult<()> {
        let actual = strategy.descriptor();

        let actual_key = StrategyKey::from_descriptor(actual);

        if actual_key != *expected {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::StateConflict,
                    format!(
                        "registered strategy identity changed from '{}' to '{}'",
                        expected, actual_key
                    ),
                )
                .with_context("strategy registry identity validation"),
            );
        }

        Ok(())
    }

    fn already_exists(key: &StrategyKey) -> ResilienceError {
        ResilienceError::new(
            ResilienceErrorCode::AlreadyExists,
            format!("strategy '{}' is already registered", key),
        )
    }

    fn not_found(key: &StrategyKey) -> ResilienceError {
        ResilienceError::new(
            ResilienceErrorCode::NotFound,
            format!("strategy '{}' is not registered", key),
        )
    }

    fn validation_error(message: impl Into<String>) -> ResilienceError {
        ResilienceError::new(
            ResilienceErrorCode::Validation,
            message,
        )
    }
}

// ============================================================================
// Compile-time contract assertions
// ============================================================================

/// Verifies the registry can hold the canonical strategy trait.
///
/// This function intentionally performs no runtime work.
#[allow(dead_code)]
fn assert_strategy_contract<S>()
where
    S: MitigationStrategy + 'static,
{
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;

    use crate::quantum::resilience::mitigation::strategy::{
        ExpectedOverhead,
        OverheadDimension,
        OverheadLevel,
        StrategyFamily,
        StrategyPhase,
        StrategyRequirement,
    };

    #[derive(Debug)]
    struct TestStrategy {
        descriptor: StrategyDescriptor,
    }

    impl TestStrategy {
        fn new(
            id: &str,
            version: StrategyVersion,
            family: StrategyFamily,
        ) -> Self {
            let id = StrategyId::new(id).expect("test strategy id must be valid");

            Self {
                descriptor: StrategyDescriptor {
                    id,
                    version,
                    family,
                    phase: StrategyPhase::PostExecution,
                    description: Arc::from("registry test strategy"),
                    requirements: Arc::from(Vec::<StrategyRequirement>::new()),
                    expected_overhead: Arc::from(
                        Vec::<ExpectedOverhead>::from([
                            ExpectedOverhead::new(
                                OverheadDimension::ClassicalComputation,
                                OverheadLevel::Low,
                            ),
                        ]),
                    ),
                    deterministic: true,
                    requires_explicit_authorization: false,
                },
            }
        }
    }

    impl MitigationStrategy for TestStrategy {
        fn descriptor(&self) -> &StrategyDescriptor {
            &self.descriptor
        }
    }

    #[test]
    fn new_registry_is_empty() {
        let registry = StrategyRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn registration_uses_descriptor_identity() {
        let mut registry = StrategyRegistry::new();

        let strategy = TestStrategy::new(
            "test.strategy",
            StrategyVersion::new(1, 0, 0),
            StrategyFamily::Custom,
        );

        let key = registry.register(strategy).expect("registration must succeed");

        assert_eq!(key.id_str(), "test.strategy");
        assert_eq!(key.version(), StrategyVersion::new(1, 0, 0));
        assert!(registry.contains(&key));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn duplicate_identity_is_rejected() {
        let mut registry = StrategyRegistry::new();

        let first = TestStrategy::new(
            "test.strategy",
            StrategyVersion::new(1, 0, 0),
            StrategyFamily::Custom,
        );

        let second = TestStrategy::new(
            "test.strategy",
            StrategyVersion::new(1, 0, 0),
            StrategyFamily::Custom,
        );

        registry.register(first).expect("first registration");

        let error = registry
            .register(second)
            .expect_err("duplicate must be rejected");

        assert_eq!(error.code, ResilienceErrorCode::AlreadyExists);
    }

    #[test]
    fn multiple_versions_can_coexist() {
        let mut registry = StrategyRegistry::new();

        registry
            .register(TestStrategy::new(
                "test.strategy",
                StrategyVersion::new(1, 0, 0),
                StrategyFamily::Custom,
            ))
            .expect("v1 registration");

        registry
            .register(TestStrategy::new(
                "test.strategy",
                StrategyVersion::new(2, 0, 0),
                StrategyFamily::Custom,
            ))
            .expect("v2 registration");

        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn replacement_is_explicit() {
        let mut registry = StrategyRegistry::new();

        registry
            .register(TestStrategy::new(
                "test.strategy",
                StrategyVersion::new(1, 0, 0),
                StrategyFamily::Custom,
            ))
            .expect("initial registration");

        let replaced = registry
            .replace(TestStrategy::new(
                "test.strategy",
                StrategyVersion::new(1, 0, 0),
                StrategyFamily::Custom,
            ))
            .expect("replacement must succeed");

        assert!(replaced.is_some());
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn iteration_order_is_deterministic() {
        let mut registry = StrategyRegistry::new();

        registry
            .register(TestStrategy::new(
                "z.strategy",
                StrategyVersion::new(1, 0, 0),
                StrategyFamily::Custom,
            ))
            .expect("z registration");

        registry
            .register(TestStrategy::new(
                "a.strategy",
                StrategyVersion::new(1, 0, 0),
                StrategyFamily::Custom,
            ))
            .expect("a registration");

        let ids: Vec<&str> = registry.keys().map(StrategyKey::id_str).collect();

        assert_eq!(ids, vec!["a.strategy", "z.strategy"]);
    }

    #[test]
    fn exact_lookup_returns_shared_strategy() {
        let mut registry = StrategyRegistry::new();

        let key = registry
            .register(TestStrategy::new(
                "test.strategy",
                StrategyVersion::new(1, 0, 0),
                StrategyFamily::Custom,
            ))
            .expect("registration");

        let strategy = registry.get(&key).expect("lookup");

        assert_eq!(strategy.descriptor().id.as_str(), "test.strategy");
        assert_eq!(
            strategy.descriptor().version,
            StrategyVersion::new(1, 0, 0)
        );
    }

    #[test]
    fn removing_unknown_strategy_returns_not_found() {
        let registry = StrategyRegistry::new();

        let key = StrategyKey::new(
            StrategyId::new("missing.strategy").expect("valid test id"),
            StrategyVersion::new(1, 0, 0),
        );

        let mut registry = registry;

        let error = registry
            .remove(&key)
            .expect_err("missing strategy must fail");

        assert_eq!(error.code, ResilienceErrorCode::NotFound);
    }

    #[test]
    fn clear_removes_every_registered_strategy() {
        let mut registry = StrategyRegistry::new();

        registry
            .register(TestStrategy::new(
                "one.strategy",
                StrategyVersion::new(1, 0, 0),
                StrategyFamily::Custom,
            ))
            .expect("registration");

        registry
            .register(TestStrategy::new(
                "two.strategy",
                StrategyVersion::new(1, 0, 0),
                StrategyFamily::Custom,
            ))
            .expect("registration");

        assert_eq!(registry.clear(), 2);
        assert!(registry.is_empty());
    }

    #[test]
    fn family_count_is_dynamic() {
        let mut registry = StrategyRegistry::new();

        registry
            .register(TestStrategy::new(
                "readout.strategy",
                StrategyVersion::new(1, 0, 0),
                StrategyFamily::Readout,
            ))
            .expect("registration");

        registry
            .register(TestStrategy::new(
                "custom.strategy",
                StrategyVersion::new(1, 0, 0),
                StrategyFamily::Custom,
            ))
            .expect("registration");

        assert_eq!(registry.count_family(StrategyFamily::Readout), 1);
        assert_eq!(registry.count_family(StrategyFamily::Custom), 1);
        assert_eq!(registry.count_family(StrategyFamily::Twirling), 0);
    }
}