
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Compiler Analysis Manager
//!
//! Path: src/compiler/passes/analysis_manager.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Provide compiler-side storage, lookup, dependency tracking, and
//! invalidation for analysis results produced by canonical ZUIR and
//! domain-specific analysis implementations.
//!
//! ARCHITECTURAL AUTHORITY
//!
//! Analysis algorithms and their result semantics remain owned by:
//!
//!     crate::zuir::analysis
//!
//! Canonical pass contracts remain owned by:
//!
//!     crate::zuir::pass::contracts
//!
//! Pass-driven invalidation policy remains owned by:
//!
//!     crate::zuir::pass::invalidation
//!
//! Pass execution remains owned by:
//!
//!     crate::zuir::pass::manager
//!
//! This module owns only compiler-side analysis-result caching,
//! dependency validation, and cache invalidation.
//!
//! DOES NOT OWN
//!
//! - Analysis algorithms.
//! - IR definitions.
//! - Semantic analysis.
//! - Type checking.
//! - IR verification.
//! - Pass execution.
//! - Pass registration.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Runtime execution.
//! - Semantic-preservation proofs.
//!
//! SCALABILITY
//!
//! No fixed limit is imposed on:
//!
//! - Analysis identities.
//! - Analysis subjects.
//! - Cached results.
//! - Dependencies.
//! - Computational domains.
//! - Hardware types.
//! - Program size.
//!
//! Storage grows according to actual resource availability.
//! This module does not promise literal infinite memory or execution.
//!
//! SAFETY
//!
//! Unsafe Rust is forbidden.
//!
//! CONCURRENCY
//!
//! The manager is safe to share between threads.
//! A mutex protects cache state.
//! Analysis result values must be Send + Sync.
//!
//! DETERMINISM
//!
//! Ordered collections are used for identities, dependencies,
//! invalidation results, and deterministic traversal.
//!
//! CACHE CORRECTNESS
//!
//! An analysis result is reusable only when:
//!
//! 1. Its analysis identity matches.
//! 2. Its subject identity matches.
//! 3. Its IR revision matches.
//! 4. Every declared dependency remains available at the revision
//!    recorded when the result was produced.
//!
//! A stale or missing dependency invalidates dependent cached results.
//!
//! An analysis result is not proof that the IR is valid or that the
//! analysis is complete. The caller must respect the analysis contract.

use std::any::Any;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};

// -----------------------------------------------------------------------------
// Analysis identity
// -----------------------------------------------------------------------------

/// Extensible identity for an analysis implementation.
///
/// Examples:
///
/// - core.control_flow
/// - core.def_use
/// - core.dominance
/// - classical.constant_propagation
/// - quantum.logical_liveness
/// - hdl.signal_dependencies
/// - tensor.shape_analysis
/// - ai.tensor_liveness
/// - distributed.communication_dependencies
///
/// This identity does not identify a hardware device.
///
/// Names are intentionally extensible. New computational domains do not
/// require changes to this type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnalysisId(String);

impl AnalysisId {
    /// Create an analysis identity.
    ///
    /// Empty values, surrounding whitespace, whitespace characters,
    /// and control characters are rejected.
    ///
    /// Namespacing is recommended but not mandatory for compatibility
    /// with the canonical analysis identifiers.
    pub fn new(value: impl Into<String>) -> Result<Self, AnalysisManagerError> {
        let value = value.into();

        if value.is_empty()
            || value.trim() != value
            || value.chars().any(|ch| ch.is_whitespace() || ch.is_control())
        {
            return Err(AnalysisManagerError::InvalidAnalysisId {
                value,
            });
        }

        Ok(Self(value))
    }

    /// Return the canonical identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AnalysisId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// -----------------------------------------------------------------------------
// Analysis subject
// -----------------------------------------------------------------------------

/// Stable identity of the IR entity or scope analyzed.
///
/// A subject can identify a module, function, region, block, operation,
/// domain-specific entity, or another canonical analysis scope.
///
/// The identity must be stable for the lifetime of the associated
/// IR revision.
///
/// This type deliberately does not invent a second ZUIR identifier system.
/// Adapters should derive its value from canonical repository identities.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnalysisSubject(String);

impl AnalysisSubject {
    /// Construct a subject identity.
    pub fn new(value: impl Into<String>) -> Result<Self, AnalysisManagerError> {
        let value = value.into();

        if value.is_empty()
            || value.trim() != value
            || value.chars().any(|ch| ch.is_control())
        {
            return Err(AnalysisManagerError::InvalidSubject {
                value,
            });
        }

        Ok(Self(value))
    }

    /// Return the subject identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AnalysisSubject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// -----------------------------------------------------------------------------
// IR revision
// -----------------------------------------------------------------------------

/// Caller-supplied identity for a particular IR revision.
///
/// The compiler does not calculate fingerprints or assume that a numeric
/// revision is globally unique.
///
/// The owner of the canonical IR must provide a revision that changes
/// whenever an IR mutation can affect the relevant analysis.
///
/// A revision is scoped by AnalysisSubject.
///
/// Revision zero is valid. No revision value is reserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnalysisRevision(u128);

impl AnalysisRevision {
    /// Construct a revision identifier.
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Return the underlying revision value.
    #[must_use]
    pub const fn value(self) -> u128 {
        self.0
    }
}

// -----------------------------------------------------------------------------
// Analysis key
// -----------------------------------------------------------------------------

/// Complete identity of one analysis result.
///
/// Two analyses of different subjects are distinct, even when their
/// AnalysisId values are identical.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnalysisKey {
    analysis: AnalysisId,
    subject: AnalysisSubject,
}

impl AnalysisKey {
    /// Construct an analysis key.
    #[must_use]
    pub fn new(
        analysis: AnalysisId,
        subject: AnalysisSubject,
    ) -> Self {
        Self {
            analysis,
            subject,
        }
    }

    /// Return the analysis identity.
    #[must_use]
    pub fn analysis(&self) -> &AnalysisId {
        &self.analysis
    }

    /// Return the subject identity.
    #[must_use]
    pub fn subject(&self) -> &AnalysisSubject {
        &self.subject
    }
}

// -----------------------------------------------------------------------------
// Analysis dependency
// -----------------------------------------------------------------------------

/// A dependency required by an analysis result.
///
/// The dependency records the exact revision consumed by the analysis.
///
/// A dependent result is not reusable if the dependency is absent or
/// available at a different revision.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnalysisDependency {
    key: AnalysisKey,
    revision: AnalysisRevision,
}

impl AnalysisDependency {
    /// Construct a dependency declaration.
    #[must_use]
    pub fn new(
        key: AnalysisKey,
        revision: AnalysisRevision,
    ) -> Self {
        Self {
            key,
            revision,
        }
    }

    /// Return the dependency key.
    #[must_use]
    pub fn key(&self) -> &AnalysisKey {
        &self.key
    }

    /// Return the required dependency revision.
    #[must_use]
    pub const fn revision(&self) -> AnalysisRevision {
        self.revision
    }
}

// -----------------------------------------------------------------------------
// Cache entry
// -----------------------------------------------------------------------------

/// One cached analysis result.
///
/// The result is type-erased internally but is retrieved through
/// checked Arc downcasting. No unchecked casts or unsafe operations
/// are used.
struct CacheEntry {
    revision: AnalysisRevision,
    dependencies: BTreeSet<AnalysisDependency>,
    value: Arc<dyn Any + Send + Sync>,
}

impl fmt::Debug for CacheEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CacheEntry")
            .field("revision", &self.revision)
            .field("dependencies", &self.dependencies)
            .finish_non_exhaustive()
    }
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Analysis-manager error category.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalysisManagerError {
    InvalidAnalysisId {
        value: String,
    },

    InvalidSubject {
        value: String,
    },

    MissingDependency {
        dependency: AnalysisKey,
    },

    DependencyRevisionMismatch {
        dependency: AnalysisKey,
        expected: AnalysisRevision,
        actual: AnalysisRevision,
    },

    DependencyCycle {
        analysis: AnalysisKey,
        dependency: AnalysisKey,
    },

    SelfDependency {
        analysis: AnalysisKey,
    },

    ResultTypeMismatch {
        analysis: AnalysisKey,
    },

    LockPoisoned,

    DuplicateInvalidationKey {
        key: AnalysisKey,
    },
}

impl fmt::Display for AnalysisManagerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAnalysisId { value } => {
                write!(formatter, "invalid analysis identifier: {value:?}")
            }

            Self::InvalidSubject { value } => {
                write!(formatter, "invalid analysis subject: {value:?}")
            }

            Self::MissingDependency { dependency } => {
                write!(
                    formatter,
                    "required analysis dependency is unavailable: \
                     {} for {}",
                    dependency.analysis(),
                    dependency.subject()
                )
            }

            Self::DependencyRevisionMismatch {
                dependency,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "analysis dependency revision mismatch for {}: \
                     expected {}, found {}",
                    dependency.analysis(),
                    expected.value(),
                    actual.value()
                )
            }

            Self::DependencyCycle {
                analysis,
                dependency,
            } => {
                write!(
                    formatter,
                    "analysis dependency cycle: {} depends on {}",
                    analysis.analysis(),
                    dependency.analysis()
                )
            }

            Self::SelfDependency { analysis } => {
                write!(
                    formatter,
                    "analysis cannot depend on itself: {}",
                    analysis.analysis()
                )
            }

            Self::ResultTypeMismatch { analysis } => {
                write!(
                    formatter,
                    "cached result has an unexpected Rust type for {}",
                    analysis.analysis()
                )
            }

            Self::LockPoisoned => {
                formatter.write_str("analysis cache lock is poisoned")
            }

            Self::DuplicateInvalidationKey { key } => {
                write!(
                    formatter,
                    "duplicate analysis invalidation key: {}",
                    key.analysis()
                )
            }
        }
    }
}

impl Error for AnalysisManagerError {}

// -----------------------------------------------------------------------------
// Manager
// -----------------------------------------------------------------------------

/// Thread-safe compiler analysis-result manager.
///
/// This manager stores results produced by analysis implementations.
/// It does not calculate those results.
///
/// The cache has no arbitrary entry-count limit and does not silently
/// discard entries.
///
/// Use explicit invalidation or clear() when the owning compilation
/// lifecycle no longer requires cached results.
#[derive(Default)]
pub struct AnalysisManager {
    entries: Mutex<BTreeMap<AnalysisKey, CacheEntry>>,
}

impl fmt::Debug for AnalysisManager {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.entries.lock() {
            Ok(entries) => formatter
                .debug_struct("AnalysisManager")
                .field("entry_count", &entries.len())
                .finish(),

            Err(_) => formatter
                .debug_struct("AnalysisManager")
                .field("state", &"poisoned")
                .finish(),
        }
    }
}

impl AnalysisManager {
    /// Create an empty analysis manager.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Lock the cache and report poisoning explicitly.
    fn lock(
        &self,
    ) -> Result<MutexGuard<'_, BTreeMap<AnalysisKey, CacheEntry>>, AnalysisManagerError>
    {
        self.entries
            .lock()
            .map_err(|_| AnalysisManagerError::LockPoisoned)
    }

    /// Number of currently stored analysis results.
    pub fn len(&self) -> Result<usize, AnalysisManagerError> {
        Ok(self.lock()?.len())
    }

    /// Whether the cache currently contains no entries.
    pub fn is_empty(&self) -> Result<bool, AnalysisManagerError> {
        Ok(self.lock()?.is_empty())
    }

    /// Store an analysis result.
    ///
    /// Every declared dependency must already exist at exactly the
    /// revision recorded by the caller.
    ///
    /// The operation validates dependencies and cycles before changing
    /// the cache.
    ///
    /// If the existing result for this key has a different revision,
    /// that result and its transitive dependents are invalidated before
    /// the new result is inserted.
    ///
    /// The caller is responsible for ensuring that the supplied revision
    /// accurately identifies the IR used to calculate the result.
    pub fn insert<T>(
        &self,
        key: AnalysisKey,
        revision: AnalysisRevision,
        dependencies: impl IntoIterator<Item = AnalysisDependency>,
        value: T,
    ) -> Result<(), AnalysisManagerError>
    where
        T: Any + Send + Sync,
    {
        let dependencies: BTreeSet<_> = dependencies.into_iter().collect();

        let mut entries = self.lock()?;

        // Validate all dependencies before mutating cache state.
        for dependency in &dependencies {
            if dependency.key == key {
                return Err(AnalysisManagerError::SelfDependency {
                    analysis: key,
                });
            }

            let Some(existing) = entries.get(&dependency.key) else {
                return Err(AnalysisManagerError::MissingDependency {
                    dependency: dependency.key.clone(),
                });
            };

            if existing.revision != dependency.revision {
                return Err(AnalysisManagerError::DependencyRevisionMismatch {
                    dependency: dependency.key.clone(),
                    expected: dependency.revision,
                    actual: existing.revision,
                });
            }

            // A dependency must not already depend transitively on the
            // key being inserted. Otherwise insertion would create a cycle.
            if Self::depends_on(
                &entries,
                &dependency.key,
                &key,
                &mut BTreeSet::new(),
            ) {
                return Err(AnalysisManagerError::DependencyCycle {
                    analysis: key,
                    dependency: dependency.key.clone(),
                });
            }
        }

        // Replacing a result at a different revision invalidates every
        // cached result that relied on the old revision.
        let revision_changed = entries
            .get(&key)
            .is_some_and(|existing| existing.revision != revision);

        if revision_changed {
            Self::invalidate_locked(
                &mut entries,
                std::iter::once(key.clone()),
            );
        }

        entries.insert(
            key,
            CacheEntry {
                revision,
                dependencies,
                value: Arc::new(value),
            },
        );

        Ok(())
    }

    /// Retrieve a cached result if the requested revision is current.
    ///
    /// A missing result returns Ok(None).
    ///
    /// A stale result is invalidated together with its transitive
    /// dependents, then returns Ok(None).
    ///
    /// A stored result with a different Rust type returns an explicit error.
    pub fn get<T>(
        &self,
        key: &AnalysisKey,
        revision: AnalysisRevision,
    ) -> Result<Option<Arc<T>>, AnalysisManagerError>
    where
        T: Any + Send + Sync,
    {
        let mut entries = self.lock()?;

        let Some(entry) = entries.get(key) else {
            return Ok(None);
        };

        if entry.revision != revision {
            Self::invalidate_locked(
                &mut entries,
                std::iter::once(key.clone()),
            );

            return Ok(None);
        }

        // Verify that all recorded dependencies still exist at the
        // exact revisions consumed by this analysis.
        let dependencies_valid = entry.dependencies.iter().all(|dependency| {
            entries
                .get(&dependency.key)
                .is_some_and(|cached| cached.revision == dependency.revision)
        });

        if !dependencies_valid {
            Self::invalidate_locked(
                &mut entries,
                std::iter::once(key.clone()),
            );

            return Ok(None);
        }

        let value = Arc::clone(&entry.value);

        match Arc::downcast::<T>(value) {
            Ok(value) => Ok(Some(value)),

            Err(_) => Err(AnalysisManagerError::ResultTypeMismatch {
                analysis: key.clone(),
            }),
        }
    }

    /// Check whether a result exists at the requested revision.
    ///
    /// This method also performs stale-result cleanup.
    pub fn contains(
        &self,
        key: &AnalysisKey,
        revision: AnalysisRevision,
    ) -> Result<bool, AnalysisManagerError> {
        let mut entries = self.lock()?;

        let Some(entry) = entries.get(key) else {
            return Ok(false);
        };

        let valid = entry.revision == revision
            && entry.dependencies.iter().all(|dependency| {
                entries
                    .get(&dependency.key)
                    .is_some_and(|cached| cached.revision == dependency.revision)
            });

        if !valid {
            Self::invalidate_locked(
                &mut entries,
                std::iter::once(key.clone()),
            );

            return Ok(false);
        }

        Ok(true)
    }

    /// Invalidate one analysis and every cached result that depends
    /// directly or transitively on it.
    ///
    /// Returns the complete set of removed keys.
    pub fn invalidate(
        &self,
        key: &AnalysisKey,
    ) -> Result<BTreeSet<AnalysisKey>, AnalysisManagerError> {
        let mut entries = self.lock()?;

        Ok(Self::invalidate_locked(
            &mut entries,
            std::iter::once(key.clone()),
        ))
    }

    /// Invalidate multiple analyses and all their transitive dependents.
    ///
    /// Duplicate root keys are rejected rather than silently hidden.
    ///
    /// The operation is atomic with respect to other cache operations.
    pub fn invalidate_many(
        &self,
        keys: impl IntoIterator<Item = AnalysisKey>,
    ) -> Result<BTreeSet<AnalysisKey>, AnalysisManagerError> {
        let roots: Vec<_> = keys.into_iter().collect();
        let mut unique = BTreeSet::new();

        for key in roots {
            if !unique.insert(key.clone()) {
                return Err(AnalysisManagerError::DuplicateInvalidationKey {
                    key,
                });
            }
        }

        let mut entries = self.lock()?;

        Ok(Self::invalidate_locked(
            &mut entries,
            unique,
        ))
    }

    /// Invalidate every cached result associated with a subject.
    ///
    /// This is useful when a canonical IR module or region is replaced.
    ///
    /// Every result for the subject is invalidated, followed by all
    /// dependent results, including results for other subjects.
    pub fn invalidate_subject(
        &self,
        subject: &AnalysisSubject,
    ) -> Result<BTreeSet<AnalysisKey>, AnalysisManagerError> {
        let mut entries = self.lock()?;

        let roots: Vec<_> = entries
            .keys()
            .filter(|key| key.subject == *subject)
            .cloned()
            .collect();

        Ok(Self::invalidate_locked(
            &mut entries,
            roots,
        ))
    }

    /// Remove all cached analysis results.
    ///
    /// This is an explicit lifecycle operation, not an automatic
    /// response to an arbitrary cache-size threshold.
    pub fn clear(&self) -> Result<(), AnalysisManagerError> {
        self.lock()?.clear();
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Dependency graph algorithms
    // -------------------------------------------------------------------------

    /// Determine whether `candidate` depends transitively on `target`.
    ///
    /// The visited set prevents repeated traversal of shared dependency
    /// subgraphs. The cache is expected to be acyclic, but this traversal
    /// remains defensive against inconsistent state.
    fn depends_on(
        entries: &BTreeMap<AnalysisKey, CacheEntry>,
        candidate: &AnalysisKey,
        target: &AnalysisKey,
        visited: &mut BTreeSet<AnalysisKey>,
    ) -> bool {
        if candidate == target {
            return true;
        }

        if !visited.insert(candidate.clone()) {
            return false;
        }

        let Some(entry) = entries.get(candidate) else {
            return false;
        };

        entry.dependencies.iter().any(|dependency| {
            Self::depends_on(
                entries,
                &dependency.key,
                target,
                visited,
            )
        })
    }

    /// Remove roots and all transitive dependents.
    ///
    /// A result A depends on B when B is listed in A's dependency set.
    /// Invalidating B therefore also invalidates A.
    ///
    /// The algorithm computes the complete removal set before mutating
    /// the map, so traversal never observes a partially removed graph.
    fn invalidate_locked(
        entries: &mut BTreeMap<AnalysisKey, CacheEntry>,
        roots: impl IntoIterator<Item = AnalysisKey>,
    ) -> BTreeSet<AnalysisKey> {
        let mut removed: BTreeSet<AnalysisKey> =
            roots.into_iter().collect();

        loop {
            let previous_size = removed.len();

            for (candidate, entry) in entries.iter() {
                if removed.contains(candidate) {
                    continue;
                }

                if entry.dependencies.iter().any(|dependency| {
                    removed.contains(&dependency.key)
                }) {
                    removed.insert(candidate.clone());
                }
            }

            if removed.len() == previous_size {
                break;
            }
        }

        for key in &removed {
            entries.remove(key);
        }

        removed
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn analysis(value: &str) -> AnalysisId {
        AnalysisId::new(value).expect("valid analysis ID")
    }

    fn subject(value: &str) -> AnalysisSubject {
        AnalysisSubject::new(value).expect("valid subject")
    }

    fn key(analysis_name: &str, subject_name: &str) -> AnalysisKey {
        AnalysisKey::new(
            analysis(analysis_name),
            subject(subject_name),
        )
    }

    fn revision(value: u128) -> AnalysisRevision {
        AnalysisRevision::new(value)
    }

    #[test]
    fn creates_extensible_analysis_ids() {
        for value in [
            "core.cfg",
            "classical.constant_propagation",
            "quantum.logical_liveness",
            "hdl.signal_dependencies",
            "tensor.shape_analysis",
            "future_domain.custom_analysis",
        ] {
            assert_eq!(
                analysis(value).as_str(),
                value
            );
        }
    }

    #[test]
    fn rejects_invalid_analysis_ids() {
        for value in ["", " ", " core.cfg", "core cfg", "core\ncfg"] {
            assert!(AnalysisId::new(value).is_err());
        }
    }

    #[test]
    fn stores_and_retrieves_typed_results() {
        let manager = AnalysisManager::new();
        let key = key("core.cfg", "module.main");
        let revision = revision(1);

        manager
            .insert(
                key.clone(),
                revision,
                [],
                vec![1_u64, 2, 3],
            )
            .unwrap();

        let result = manager
            .get::<Vec<u64>>(&key, revision)
            .unwrap()
            .unwrap();

        assert_eq!(*result, vec![1, 2, 3]);
    }

    #[test]
    fn returns_none_for_missing_results() {
        let manager = AnalysisManager::new();

        assert!(
            manager
                .get::<u64>(
                    &key("core.cfg", "module.main"),
                    revision(1),
                )
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn rejects_result_type_mismatch() {
        let manager = AnalysisManager::new();
        let key = key("core.cfg", "module.main");

        manager
            .insert(key.clone(), revision(1), [], 42_u64)
            .unwrap();

        assert!(matches!(
            manager.get::<String>(&key, revision(1)),
            Err(AnalysisManagerError::ResultTypeMismatch { .. })
        ));
    }

    #[test]
    fn rejects_missing_dependencies_without_mutation() {
        let manager = AnalysisManager::new();

        let key = key("core.dominance", "module.main");
        let dependency = key("core.cfg", "module.main");

        let result = manager.insert(
            key.clone(),
            revision(1),
            [AnalysisDependency::new(
                dependency,
                revision(1),
            )],
            42_u64,
        );

        assert!(matches!(
            result,
            Err(AnalysisManagerError::MissingDependency { .. })
        ));

        assert_eq!(manager.len().unwrap(), 0);
    }

    #[test]
    fn rejects_dependency_revision_mismatch() {
        let manager = AnalysisManager::new();

        let cfg = key("core.cfg", "module.main");
        let dominance = key("core.dominance", "module.main");

        manager
            .insert(cfg.clone(), revision(2), [], ())
            .unwrap();

        let result = manager.insert(
            dominance,
            revision(2),
            [AnalysisDependency::new(cfg, revision(1))],
            (),
        );

        assert!(matches!(
            result,
            Err(AnalysisManagerError::DependencyRevisionMismatch { .. })
        ));
    }

    #[test]
    fn rejects_self_dependency() {
        let manager = AnalysisManager::new();
        let key = key("core.cfg", "module.main");

        let result = manager.insert(
            key.clone(),
            revision(1),
            [AnalysisDependency::new(key.clone(), revision(1))],
            (),
        );

        assert!(matches!(
            result,
            Err(AnalysisManagerError::SelfDependency { .. })
        ));
    }

    #[test]
    fn invalidation_is_transitive() {
        let manager = AnalysisManager::new();

        let cfg = key("core.cfg", "module.main");
        let dominance = key("core.dominance", "module.main");
        let liveness = key("core.liveness", "module.main");

        manager
            .insert(cfg.clone(), revision(1), [], ())
            .unwrap();

        manager
            .insert(
                dominance.clone(),
                revision(1),
                [AnalysisDependency::new(cfg.clone(), revision(1))],
                (),
            )
            .unwrap();

        manager
            .insert(
                liveness.clone(),
                revision(1),
                [AnalysisDependency::new(
                    dominance.clone(),
                    revision(1),
                )],
                (),
            )
            .unwrap();

        let removed = manager.invalidate(&cfg).unwrap();

        assert_eq!(
            removed,
            BTreeSet::from([
                cfg.clone(),
                dominance.clone(),
                liveness.clone(),
            ])
        );

        assert!(manager.is_empty().unwrap());
    }

    #[test]
    fn replacing_a_revision_invalidates_dependents() {
        let manager = AnalysisManager::new();

        let cfg = key("core.cfg", "module.main");
        let dominance = key("core.dominance", "module.main");

        manager
            .insert(cfg.clone(), revision(1), [], ())
            .unwrap();

        manager
            .insert(
                dominance.clone(),
                revision(1),
                [AnalysisDependency::new(cfg.clone(), revision(1))],
                (),
            )
            .unwrap();

        manager
            .insert(cfg.clone(), revision(2), [], ())
            .unwrap();

        assert!(manager.contains(&cfg, revision(2)).unwrap());
        assert!(!manager.contains(&dominance, revision(1)).unwrap());
    }

    #[test]
    fn stale_lookup_invalidates_dependents() {
        let manager = AnalysisManager::new();

        let cfg = key("core.cfg", "module.main");
        let dominance = key("core.dominance", "module.main");

        manager
            .insert(cfg.clone(), revision(1), [], ())
            .unwrap();

        manager
            .insert(
                dominance.clone(),
                revision(1),
                [AnalysisDependency::new(cfg.clone(), revision(1))],
                (),
            )
            .unwrap();

        assert!(
            manager
                .get::<()>(&cfg, revision(2))
                .unwrap()
                .is_none()
        );

        assert!(manager.is_empty().unwrap());
    }

    #[test]
    fn invalidates_all_results_for_a_subject() {
        let manager = AnalysisManager::new();

        let cfg = key("core.cfg", "module.main");
        let effects = key("core.effects", "module.main");
        let other = key("core.cfg", "module.other");

        manager.insert(cfg.clone(), revision(1), [], ()).unwrap();
        manager.insert(effects.clone(), revision(1), [], ()).unwrap();
        manager.insert(other.clone(), revision(1), [], ()).unwrap();

        let removed = manager
            .invalidate_subject(&subject("module.main"))
            .unwrap();

        assert!(removed.contains(&cfg));
        assert!(removed.contains(&effects));
        assert!(!removed.contains(&other));
    }

    #[test]
    fn clear_removes_every_result() {
        let manager = AnalysisManager::new();

        manager
            .insert(
                key("core.cfg", "module.main"),
                revision(1),
                [],
                (),
            )
            .unwrap();

        manager.clear().unwrap();

        assert!(manager.is_empty().unwrap());
    }

    #[test]
    fn independent_domains_can_share_the_manager() {
        let manager = AnalysisManager::new();

        let classical = key(
            "classical.constant_propagation",
            "module.main",
        );

        let quantum = key(
            "quantum.logical_liveness",
            "module.main",
        );

        let hdl = key(
            "hdl.signal_dependencies",
            "module.main",
        );

        manager.insert(classical.clone(), revision(1), [], 10_u64).unwrap();
        manager.insert(quantum.clone(), revision(1), [], 20_u64).unwrap();
        manager.insert(hdl.clone(), revision(1), [], 30_u64).unwrap();

        assert_eq!(
            *manager.get::<u64>(&classical, revision(1)).unwrap().unwrap(),
            10
        );

        assert_eq!(
            *manager.get::<u64>(&quantum, revision(1)).unwrap().unwrap(),
            20
        );

        assert_eq!(
            *manager.get::<u64>(&hdl, revision(1)).unwrap().unwrap(),
            30
        );
    }
}
