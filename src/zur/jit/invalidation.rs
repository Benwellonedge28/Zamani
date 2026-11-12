
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — JIT Invalidation
//!
//! Path: src/zur/jit/invalidation.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97+
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Coordinates invalidation of generated JIT artifacts when assumptions,
//! dependencies, capabilities, compatibility conditions, or other
//! compilation-relevant facts become invalid.
//!
//! This module provides:
//!
//! - Open-ended invalidation identities.
//! - Dependency-to-artifact indexing.
//! - Assumption invalidation.
//! - Compatibility invalidation.
//! - Capability invalidation.
//! - Generation-based invalidation fencing.
//! - Atomic validation/publication coordination.
//! - Explicit invalidation reports.
//! - Partial-failure preservation.
//! - Cache invalidation integration.
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - JIT invalidation coordination.
//! - Invalidation subject identities.
//! - Invalidation reason identities.
//! - Dependency indexing.
//! - Invalidation generations.
//! - Compilation dependency snapshots.
//! - Cache invalidation coordination.
//! - Publication fencing.
//! - Invalidation reporting.
//!
//! It does NOT own:
//!
//! - Source parsing.
//! - AST construction.
//! - Semantic analysis.
//! - Compiler IR.
//! - Quantum IR.
//! - HDL IR.
//! - ZUIR.
//! - Compiler optimization.
//! - Semantic assumption verification.
//! - Backend registration.
//! - Backend selection.
//! - Code generation.
//! - Executable memory.
//! - Physical execution.
//! - Deoptimization.
//! - Runtime scheduling.
//! - Hardware discovery.
//! - Physical resource reservation.
//! - Security policy.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! ============================================================================
//! CANONICAL INTEGRATION
//! ============================================================================
//!
//! JIT engine:
//!     crate::zur::jit::engine
//!
//!     JitCacheKey
//!     CompiledJitArtifact
//!     JitCodeCache
//!
//! Code cache:
//!     crate::zur::jit::code_cache
//!
//! Specialization:
//!     crate::zur::jit::specialization
//!
//! Deoptimization:
//!     crate::zur::jit::deoptimization
//!
//! Compiler:
//!     crate::compiler
//!     crate::compiler::optimization
//!     crate::compiler::ir
//!     crate::compiler::passes
//!
//! Backend:
//!     crate::backend
//!
//! Security:
//!     crate::zur::security
//!
//! Observability:
//!     crate::zur::observability
//!
//! Errors:
//!     crate::zur::error
//!
//! ============================================================================
//! GRAMMAR BOUNDARY
//! ============================================================================
//!
//! Relevant grammar:
//!
//!     grammar/compile/optimization.g4
//!     grammar/execution/runtime.g4
//!     grammar/execution/runtime-capabilities.g4
//!     grammar/resources/portability.g4
//!
//! Relevant specifications:
//!
//!     grammar/spec/portability.md
//!     grammar/specification/execution-model.md
//!     grammar/specification/semantic-model.md
//!
//! The frontend and semantic authorities establish program meaning.
//!
//! The compiler establishes optimization and compatibility evidence.
//!
//! This module receives invalidation events from trusted integration
//! authorities.
//!
//! It never parses grammar or interprets AST nodes.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! An artifact is reusable only while all compilation-relevant
//! assumptions and dependencies remain valid.
//!
//! Invalidation must not change program semantics.
//!
//! Different hardware, providers, domains, and execution environments
//! may produce different valid artifacts.
//!
//! No arbitrary domain, provider, device, dependency, or hardware
//! capacity limits are imposed.
//!
//! Literal infinite physical computation is not promised.
//!
//! ============================================================================
//! CRITICAL INVARIANTS
//! ============================================================================
//!
//! 1. Invalidated artifacts cannot be newly published using stale snapshots.
//! 2. Invalidation does not imply termination of active executions.
//! 3. Existing Arc references remain owned by their holders.
//! 4. Cache invalidation is delegated to JitCodeCache.
//! 5. Dependency identities are open-ended.
//! 6. Cache keys remain compiler-owned canonical identities.
//! 7. Failed invalidations remain retryable.
//! 8. Partial invalidation failures are never reported as complete.
//! 9. Invalidation never fabricates execution or recovery success.
//! 10. All synchronization is instance-local.
//! 11. No unsafe Rust is permitted.
//! 12. Compiler and semantic authorities remain authoritative.
//!
//! ============================================================================

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::zur::error::{ZurError, ZurResult};

use crate::zur::jit::engine::{
    CompiledJitArtifact,
    JitCacheKey,
    JitCodeCache,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of ZUR JIT invalidation.
pub const JIT_INVALIDATION_API_VERSION: u16 = 1;

/// Component identity used in structured errors.
const COMPONENT: &str = "zur.jit.invalidation";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

/// Creates an invalidation input error.
fn invalidation_input_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::invalid_input(message)
        .with_component(COMPONENT)
        .with_operation("invalidate")
}

/// Creates an invalidation compatibility error.
fn invalidation_compatibility_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::compatibility(message)
        .with_component(COMPONENT)
        .with_operation("validate_publication")
}

/// Creates an invalidation resource error.
fn invalidation_resource_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::internal(message)
        .with_component(COMPONENT)
        .with_operation("allocate_invalidation_state")
}

/// Creates an invalidation internal error.
fn invalidation_internal_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::internal(message)
        .with_component(COMPONENT)
}

// ============================================================================
// IDENTITY VALIDATION
// ============================================================================

/// Validates a nonempty, unpadded identity.
///
/// Identity interpretation belongs to the authority that supplies it.
/// This function validates representation only.
fn validate_identity(
    value: &str,
    description: &str,
) -> ZurResult<()> {
    if value.is_empty() {
        return Err(invalidation_input_error(
            format!("{description} must not be empty"),
        ));
    }

    if value.trim() != value {
        return Err(invalidation_input_error(
            format!(
                "{description} contains surrounding whitespace"
            ),
        ));
    }

    Ok(())
}

/// Validates an open-ended namespaced identity.
///
/// Accepted examples:
///
///     zamani.classical
///     zamani.compiler.optimization
///     zamani.capability.quantum.measurement
///     external.provider.namespace.resource
///
/// This does not establish that the named subsystem or capability
/// exists or is currently available.
fn validate_namespaced_identity(
    value: &str,
    description: &str,
) -> ZurResult<()> {
    validate_identity(value, description)?;

    let mut segments = 0usize;

    for segment in value.split('.') {
        if segment.is_empty() {
            return Err(invalidation_input_error(
                format!(
                    "{description} contains an empty namespace segment"
                ),
            ));
        }

        if !segment.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'-'
                || byte == b'_'
        }) {
            return Err(invalidation_input_error(
                format!(
                    "{description} contains an invalid character"
                ),
            ));
        }

        if segment.starts_with('-')
            || segment.starts_with('_')
            || segment.ends_with('-')
            || segment.ends_with('_')
        {
            return Err(invalidation_input_error(
                format!(
                    "{description} contains an invalid segment boundary"
                ),
            ));
        }

        segments += 1;
    }

    if segments < 2 {
        return Err(invalidation_input_error(
            format!(
                "{description} must contain a namespace"
            ),
        ));
    }

    Ok(())
}

// ============================================================================
// INVALIDATION SUBJECT
// ============================================================================

/// Identifies an assumption, dependency, or compatibility fact whose
/// invalidation may affect one or more generated artifacts.
///
/// The identity is open-ended.
///
/// Examples:
///
/// - `zamani.compiler.optimization.pipeline`
/// - `zamani.specialization.assumption.numeric-range`
/// - `zamani.capability.quantum.measurement`
/// - `zamani.backend.compatibility.target`
/// - `zamani.provider.compatibility.contract`
/// - `external.library.abi`
///
/// These identities are operational dependency identities.
/// They do not replace semantic or compiler-owned identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InvalidationSubject(String);

impl InvalidationSubject {
    /// Creates a validated invalidation subject.
    pub fn new(
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let value = value.into();

        validate_namespaced_identity(
            &value,
            "invalidation subject",
        )?;

        Ok(Self(value))
    }

    /// Returns the subject's canonical identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for InvalidationSubject {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// INVALIDATION REASON
// ============================================================================

/// Identifies why an invalidation was requested.
///
/// Reasons are open-ended so that new compiler, backend, provider,
/// hardware, security, and domain-specific invalidation causes can be
/// introduced without changing this module.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InvalidationReason(String);

impl InvalidationReason {
    /// Creates a validated invalidation reason.
    pub fn new(
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let value = value.into();

        validate_namespaced_identity(
            &value,
            "invalidation reason",
        )?;

        Ok(Self(value))
    }

    /// Returns the canonical invalidation reason.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// INVALIDATION SNAPSHOT
// ============================================================================

/// Captures the generations of all subjects on which a compilation
/// depends.
///
/// A snapshot is created before compilation or specialization.
///
/// It must be checked again before the resulting artifact is published.
///
/// The fields are private so callers cannot fabricate generations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JitInvalidationSnapshot {
    generations: Vec<(InvalidationSubject, u128)>,
}

impl JitInvalidationSnapshot {
    /// Returns the number of tracked subjects.
    #[must_use]
    pub fn subject_count(&self) -> usize {
        self.generations.len()
    }

    /// Returns whether the snapshot contains dependencies.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.generations.is_empty()
    }

    /// Returns the generation captured for a subject.
    #[must_use]
    pub fn generation(
        &self,
        subject: &InvalidationSubject,
    ) -> Option<u128> {
        self.generations
            .binary_search_by(|(candidate, _)| {
                candidate.cmp(subject)
            })
            .ok()
            .map(|index| self.generations[index].1)
    }

    /// Returns the immutable subject identities.
    #[must_use]
    pub fn subjects(
        &self,
    ) -> impl Iterator<Item = &InvalidationSubject> {
        self.generations
            .iter()
            .map(|(subject, _)| subject)
    }
}

// ============================================================================
// INVALIDATION REPORT
// ============================================================================

/// Result of successfully processing one invalidation event.
///
/// `removed_from_cache` counts entries actually removed.
///
/// `already_absent` counts entries whose cache reference had already
/// disappeared.
///
/// Neither count represents termination of active executions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JitInvalidationReport {
    subject: InvalidationSubject,
    reason: InvalidationReason,
    generation: u128,
    affected_artifacts: usize,
    removed_from_cache: usize,
    already_absent: usize,
}

impl JitInvalidationReport {
    /// Returns the invalidated subject.
    #[must_use]
    pub fn subject(&self) -> &InvalidationSubject {
        &self.subject
    }

    /// Returns the invalidation reason.
    #[must_use]
    pub fn reason(&self) -> &InvalidationReason {
        &self.reason
    }

    /// Returns the new subject generation.
    #[must_use]
    pub const fn generation(&self) -> u128 {
        self.generation
    }

    /// Returns the number of affected cache identities.
    #[must_use]
    pub const fn affected_artifacts(&self) -> usize {
        self.affected_artifacts
    }

    /// Returns the number of removed cache entries.
    #[must_use]
    pub const fn removed_from_cache(&self) -> usize {
        self.removed_from_cache
    }

    /// Returns the number of affected entries already absent.
    #[must_use]
    pub const fn already_absent(&self) -> usize {
        self.already_absent
    }
}

// ============================================================================
// PUBLICATION OUTCOME
// ============================================================================

/// Result of attempting to publish a JIT artifact against an
/// invalidation snapshot.
#[derive(Debug)]
pub enum JitInvalidationPublication {
    /// The artifact was accepted and associated with its dependencies.
    Published(Arc<dyn CompiledJitArtifact>),

    /// At least one dependency changed during compilation.
    ///
    /// The artifact was not published by this operation.
    Stale {
        /// Subjects whose generations changed.
        subjects: Vec<InvalidationSubject>,
    },
}

impl JitInvalidationPublication {
    /// Returns whether publication succeeded.
    #[must_use]
    pub const fn is_published(&self) -> bool {
        matches!(self, Self::Published(_))
    }

    /// Returns the published artifact, if successful.
    #[must_use]
    pub fn artifact(
        &self,
    ) -> Option<&Arc<dyn CompiledJitArtifact>> {
        match self {
            Self::Published(artifact) => Some(artifact),
            Self::Stale { .. } => None,
        }
    }
}

// ============================================================================
// INVALIDATION STATE
// ============================================================================

/// Internal coordinator state.
///
/// All fields are protected by one instance-local mutex.
///
/// The synchronization boundary intentionally covers cache publication
/// and invalidation. This prevents a stale compilation from being
/// published between an invalidation check and cache insertion.
#[derive(Debug, Default)]
struct InvalidationState {
    generations: HashMap<InvalidationSubject, u128>,

    dependents: HashMap<
        InvalidationSubject,
        HashSet<JitCacheKey>,
    >,

    key_subjects: HashMap<
        JitCacheKey,
        HashSet<InvalidationSubject>,
    >,
}

impl InvalidationState {
    /// Returns the current generation, defaulting to the initial generation.
    fn generation(
        &self,
        subject: &InvalidationSubject,
    ) -> u128 {
        self.generations
            .get(subject)
            .copied()
            .unwrap_or(0)
    }

    /// Removes one artifact identity from every dependency index.
    fn remove_key(
        &mut self,
        key: &JitCacheKey,
    ) {
        let Some(subjects) = self.key_subjects.remove(key)
        else {
            return;
        };

        for subject in subjects {
            let remove_subject = if let Some(keys) =
                self.dependents.get_mut(&subject)
            {
                keys.remove(key);
                keys.is_empty()
            } else {
                false
            };

            if remove_subject {
                self.dependents.remove(&subject);
            }
        }
    }

    /// Reserves the dependency-index capacity needed for publication.
    fn reserve_publication(
        &mut self,
        key: &JitCacheKey,
        subjects: &[InvalidationSubject],
    ) -> ZurResult<()> {
        self.key_subjects
            .try_reserve(1)
            .map_err(|error| {
                invalidation_resource_error(
                    format!(
                        "unable to reserve artifact dependency index: {error}"
                    ),
                )
            })?;

        for subject in subjects {
            if !self.dependents.contains_key(subject) {
                self.dependents
                    .try_reserve(1)
                    .map_err(|error| {
                        invalidation_resource_error(
                            format!(
                                "unable to reserve subject dependency index: {error}"
                            ),
                        )
                    })?;
            }

            if let Some(keys) = self.dependents.get_mut(subject) {
                keys.try_reserve(1)
                    .map_err(|error| {
                        invalidation_resource_error(
                            format!(
                                "unable to reserve dependent artifact index: {error}"
                            ),
                        )
                    })?;
            }
        }

        if self.key_subjects.contains_key(key) {
            return Err(invalidation_internal_error(
                "artifact already has a dependency registration",
            ));
        }

        Ok(())
    }

    /// Adds the dependency relationships for a published artifact.
    fn insert_dependencies(
        &mut self,
        key: JitCacheKey,
        subjects: &[InvalidationSubject],
    ) {
        let mut registered = HashSet::new();

        for subject in subjects {
            registered.insert(subject.clone());

            self.dependents
                .entry(subject.clone())
                .or_default()
                .insert(key.clone());
        }

        self.key_subjects.insert(key, registered);
    }
}

// ============================================================================
// INVALIDATION COORDINATOR
// ============================================================================

/// Coordinates JIT invalidation and publication fencing.
///
/// Each coordinator owns its own synchronization and dependency index.
///
/// Multiple independent runtime instances may therefore use independent
/// coordinators without sharing mutable global state.
///
/// The associated code cache must be the same cache used by the JIT
/// engine for the affected artifacts.
pub struct JitInvalidationCoordinator {
    state: Mutex<InvalidationState>,
}

impl fmt::Debug for JitInvalidationCoordinator {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("JitInvalidationCoordinator")
            .finish_non_exhaustive()
    }
}

impl Default for JitInvalidationCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

impl JitInvalidationCoordinator {
    /// Creates an independent invalidation coordinator.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Mutex::new(InvalidationState::default()),
        }
    }

    /// Acquires the coordinator state.
    fn lock(&self) -> ZurResult<MutexGuard<'_, InvalidationState>> {
        self.state.lock().map_err(|_| {
            invalidation_internal_error(
                "invalidation coordinator mutex was poisoned",
            )
        })
    }

    /// Captures the current generations of compilation dependencies.
    ///
    /// Call this before compilation or specialization.
    ///
    /// The resulting snapshot must be supplied to
    /// `publish_if_current` after compilation.
    ///
    /// At least one dependency is required. An artifact with no
    /// tracked dependencies cannot receive generation-based
    /// invalidation guarantees.
    pub fn capture(
        &self,
        subjects: impl IntoIterator<Item = InvalidationSubject>,
    ) -> ZurResult<JitInvalidationSnapshot> {
        let mut subjects: Vec<InvalidationSubject> =
            subjects.into_iter().collect();

        if subjects.is_empty() {
            return Err(invalidation_input_error(
                "at least one invalidation dependency is required",
            ));
        }

        subjects.sort_unstable();
        subjects.dedup();

        let state = self.lock()?;

        let mut generations = Vec::new();

        generations
            .try_reserve_exact(subjects.len())
            .map_err(|error| {
                invalidation_resource_error(
                    format!(
                        "unable to reserve invalidation snapshot: {error}"
                    ),
                )
            })?;

        for subject in subjects {
            let generation = state.generation(&subject);

            generations.push((subject, generation));
        }

        Ok(JitInvalidationSnapshot {
            generations,
        })
    }

    /// Checks whether a previously captured snapshot remains current.
    ///
    /// This is an observation only.
    ///
    /// It does not reserve the right to publish an artifact.
    /// Use `publish_if_current` for the atomic publication operation.
    pub fn is_current(
        &self,
        snapshot: &JitInvalidationSnapshot,
    ) -> ZurResult<bool> {
        if snapshot.is_empty() {
            return Err(invalidation_input_error(
                "invalidation snapshot must not be empty",
            ));
        }

        let state = self.lock()?;

        Ok(snapshot.generations.iter().all(
            |(subject, generation)| {
                state.generation(subject) == *generation
            },
        ))
    }

    /// Atomically validates a compilation snapshot and publishes
    /// its artifact into the supplied canonical JIT code cache.
    ///
    /// The coordinator lock is held throughout:
    ///
    /// 1. Generation validation.
    /// 2. Dependency-index reservation.
    /// 3. Cache publication.
    /// 4. Dependency-index registration.
    ///
    /// Invalidation cannot interleave between the validation and
    /// publication steps.
    ///
    /// A stale artifact is never published by this operation.
    ///
    /// The cache must be the same cache used by the JIT engine.
    ///
    /// Cache implementations must not call back into this coordinator
    /// while a cache operation is in progress.
    pub fn publish_if_current(
        &self,
        cache: &dyn JitCodeCache,
        key: &JitCacheKey,
        snapshot: &JitInvalidationSnapshot,
        artifact: Arc<dyn CompiledJitArtifact>,
    ) -> ZurResult<JitInvalidationPublication> {
        if snapshot.is_empty() {
            return Err(invalidation_input_error(
                "invalidation snapshot must not be empty",
            ));
        }

        if artifact.cache_key() != key {
            return Err(invalidation_compatibility_error(
                "artifact cache identity does not match publication key",
            ));
        }

        let mut state = self.lock()?;

        let mut stale_subjects = Vec::new();

        for (subject, generation) in &snapshot.generations {
            if state.generation(subject) != *generation {
                stale_subjects.push(subject.clone());
            }
        }

        if !stale_subjects.is_empty() {
            return Ok(JitInvalidationPublication::Stale {
                subjects: stale_subjects,
            });
        }

        state.reserve_publication(
            key,
            &snapshot
                .generations
                .iter()
                .map(|(subject, _)| subject.clone())
                .collect::<Vec<_>>(),
        )?;

        let canonical = cache.insert_if_absent(
            key,
            artifact,
        )?;

        if canonical.cache_key() != key {
            // Do not leave a cache entry whose identity violates
            // the cache contract.
            let _ = cache.invalidate(key);

            return Err(invalidation_internal_error(
                "cache returned an artifact with an incompatible identity",
            ));
        }

        let subjects: Vec<InvalidationSubject> = snapshot
            .generations
            .iter()
            .map(|(subject, _)| subject.clone())
            .collect();

        state.insert_dependencies(
            key.clone(),
            &subjects,
        );

        Ok(JitInvalidationPublication::Published(
            canonical,
        ))
    }

    /// Invalidates all artifacts depending on a subject.
    ///
    /// The generation advances before cache removal begins.
    ///
    /// This immediately prevents publication using an older
    /// compilation snapshot.
    ///
    /// If cache invalidation fails partway through, already completed
    /// removals remain complete and unprocessed dependencies remain
    /// indexed so that a later invalidation attempt can retry them.
    ///
    /// The same canonical cache used for publication must be supplied.
    pub fn invalidate_subject(
        &self,
        cache: &dyn JitCodeCache,
        subject: InvalidationSubject,
        reason: InvalidationReason,
    ) -> ZurResult<JitInvalidationReport> {
        let mut state = self.lock()?;

        let next_generation = state
            .generation(&subject)
            .checked_add(1)
            .ok_or_else(|| {
                invalidation_internal_error(
                    "invalidation generation exhausted its representable range",
                )
            })?;

        state.generations
            .try_reserve(1)
            .map_err(|error| {
                invalidation_resource_error(
                    format!(
                        "unable to reserve invalidation generation: {error}"
                    ),
                )
            })?;

        state.generations.insert(
            subject.clone(),
            next_generation,
        );

        let mut keys: Vec<JitCacheKey> = state
            .dependents
            .get(&subject)
            .map(|entries| entries.iter().cloned().collect())
            .unwrap_or_default();

        keys.sort_unstable();

        let affected_artifacts = keys.len();

        let mut removed_from_cache = 0usize;
        let mut already_absent = 0usize;

        for key in keys {
            // Cache errors are propagated. The failed key and all
            // remaining keys retain their dependency registrations.
            let removed = cache.invalidate(&key)?;

            if removed {
                removed_from_cache += 1;
            } else {
                already_absent += 1;
            }

            // Remove the completed identity from every subject index.
            state.remove_key(&key);
        }

        Ok(JitInvalidationReport {
            subject,
            reason,
            generation: next_generation,
            affected_artifacts,
            removed_from_cache,
            already_absent,
        })
    }

    /// Invalidates one specific artifact identity.
    ///
    /// This is useful when a compiler or security authority determines
    /// that one artifact is invalid without invalidating every artifact
    /// sharing a broader dependency.
    ///
    /// The artifact is removed from the dependency index after the
    /// cache operation succeeds.
    pub fn invalidate_artifact(
        &self,
        cache: &dyn JitCodeCache,
        key: &JitCacheKey,
    ) -> ZurResult<bool> {
        let mut state = self.lock()?;

        let removed = cache.invalidate(key)?;

        state.remove_key(key);

        Ok(removed)
    }

    /// Returns the number of artifacts currently indexed.
    ///
    /// This is the number of dependency-tracked artifact identities,
    /// not the number of active executions.
    pub fn tracked_artifacts(&self) -> ZurResult<usize> {
        let state = self.lock()?;

        Ok(state.key_subjects.len())
    }

    /// Returns the number of subjects currently indexed.
    pub fn tracked_subjects(&self) -> ZurResult<usize> {
        let state = self.lock()?;

        Ok(state.dependents.len())
    }

    /// Returns the number of artifacts depending on a subject.
    pub fn dependent_artifacts(
        &self,
        subject: &InvalidationSubject,
    ) -> ZurResult<usize> {
        let state = self.lock()?;

        Ok(state
            .dependents
            .get(subject)
            .map(HashSet::len)
            .unwrap_or(0))
    }

    /// Returns the current generation of a subject.
    ///
    /// A subject not previously invalidated has generation zero.
    pub fn current_generation(
        &self,
        subject: &InvalidationSubject,
    ) -> ZurResult<u128> {
        let state = self.lock()?;

        Ok(state.generation(subject))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;
    use std::sync::Mutex;

    // ------------------------------------------------------------------------
    // Test artifact
    // ------------------------------------------------------------------------

    #[derive(Debug)]
    struct TestArtifact {
        key: JitCacheKey,
    }

    impl CompiledJitArtifact for TestArtifact {
        fn cache_key(&self) -> &JitCacheKey {
            &self.key
        }

        fn domain_identity(&self) -> &str {
            "zamani.classical"
        }

        fn provider_identity(&self) -> &str {
            "zamani.test.provider"
        }

        fn format_identity(&self) -> &str {
            "zamani.test.artifact"
        }

        fn compiler_contract_version(&self) -> &str {
            "1"
        }
    }

    // ------------------------------------------------------------------------
    // Test cache
    // ------------------------------------------------------------------------

    #[derive(Default)]
    struct TestCache {
        entries: Mutex<
            HashMap<
                JitCacheKey,
                Arc<dyn CompiledJitArtifact>,
            >,
        >,
    }

    impl JitCodeCache for TestCache {
        fn get(
            &self,
            key: &JitCacheKey,
        ) -> ZurResult<
            Option<Arc<dyn CompiledJitArtifact>>
        > {
            let entries = self.entries.lock().map_err(|_| {
                invalidation_internal_error(
                    "test cache mutex poisoned",
                )
            })?;

            Ok(entries.get(key).cloned())
        }

        fn insert_if_absent(
            &self,
            key: &JitCacheKey,
            artifact: Arc<dyn CompiledJitArtifact>,
        ) -> ZurResult<
            Arc<dyn CompiledJitArtifact>
        > {
            let mut entries = self.entries.lock().map_err(|_| {
                invalidation_internal_error(
                    "test cache mutex poisoned",
                )
            })?;

            if let Some(existing) = entries.get(key) {
                return Ok(existing.clone());
            }

            entries.insert(
                key.clone(),
                artifact.clone(),
            );

            Ok(artifact)
        }

        fn invalidate(
            &self,
            key: &JitCacheKey,
        ) -> ZurResult<bool> {
            let mut entries = self.entries.lock().map_err(|_| {
                invalidation_internal_error(
                    "test cache mutex poisoned",
                )
            })?;

            Ok(entries.remove(key).is_some())
        }
    }

    // ------------------------------------------------------------------------
    // Helpers
    // ------------------------------------------------------------------------

    fn subject(value: &str) -> InvalidationSubject {
        InvalidationSubject::new(value)
            .expect("valid subject")
    }

    fn reason(value: &str) -> InvalidationReason {
        InvalidationReason::new(value)
            .expect("valid reason")
    }

    fn cache_key(value: &str) -> JitCacheKey {
        JitCacheKey::new(value)
            .expect("valid cache key")
    }

    fn artifact(
        key: &JitCacheKey,
    ) -> Arc<dyn CompiledJitArtifact> {
        Arc::new(TestArtifact {
            key: key.clone(),
        })
    }

    fn publish(
        coordinator: &JitInvalidationCoordinator,
        cache: &TestCache,
        key: &JitCacheKey,
        subjects: Vec<InvalidationSubject>,
    ) -> JitInvalidationPublication {
        let snapshot = coordinator
            .capture(subjects)
            .expect("capture dependencies");

        coordinator
            .publish_if_current(
                cache,
                key,
                &snapshot,
                artifact(key),
            )
            .expect("publish artifact")
    }

    // ------------------------------------------------------------------------
    // Identity tests
    // ------------------------------------------------------------------------

    #[test]
    fn rejects_empty_subject() {
        assert!(
            InvalidationSubject::new("").is_err()
        );
    }

    #[test]
    fn rejects_unqualified_subject() {
        assert!(
            InvalidationSubject::new("quantum").is_err()
        );
    }

    #[test]
    fn rejects_invalid_subject_characters() {
        assert!(
            InvalidationSubject::new(
                "zamani.Invalid"
            ).is_err()
        );
    }

    #[test]
    fn accepts_open_ended_subject_namespaces() {
        let value = subject(
            "external.provider.future.domain.resource",
        );

        assert_eq!(
            value.as_str(),
            "external.provider.future.domain.resource"
        );
    }

    #[test]
    fn accepts_open_ended_invalidation_reasons() {
        let value = reason(
            "external.security.policy.changed",
        );

        assert_eq!(
            value.as_str(),
            "external.security.policy.changed"
        );
    }

    // ------------------------------------------------------------------------
    // Snapshot tests
    // ------------------------------------------------------------------------

    #[test]
    fn rejects_empty_dependency_snapshots() {
        let coordinator =
            JitInvalidationCoordinator::new();

        assert!(
            coordinator.capture(Vec::new()).is_err()
        );
    }

    #[test]
    fn deduplicates_snapshot_dependencies() {
        let coordinator =
            JitInvalidationCoordinator::new();

        let dependency =
            subject("zamani.compiler.pipeline");

        let snapshot = coordinator
            .capture(vec![
                dependency.clone(),
                dependency,
            ])
            .expect("snapshot");

        assert_eq!(snapshot.subject_count(), 1);
    }

    #[test]
    fn initial_generation_is_zero() {
        let coordinator =
            JitInvalidationCoordinator::new();

        let dependency =
            subject("zamani.compiler.pipeline");

        let snapshot = coordinator
            .capture(vec![dependency.clone()])
            .expect("snapshot");

        assert_eq!(
            snapshot.generation(&dependency),
            Some(0)
        );
    }

    // ------------------------------------------------------------------------
    // Publication tests
    // ------------------------------------------------------------------------

    #[test]
    fn publishes_artifact_with_current_dependencies() {
        let coordinator =
            JitInvalidationCoordinator::new();

        let cache = TestCache::default();

        let key = cache_key("sha256:artifact-one");

        let result = publish(
            &coordinator,
            &cache,
            &key,
            vec![
                subject("zamani.compiler.pipeline"),
            ],
        );

        assert!(result.is_published());

        assert_eq!(
            coordinator.tracked_artifacts().expect("count"),
            1
        );
    }

    #[test]
    fn rejects_artifact_with_mismatched_identity() {
        let coordinator =
            JitInvalidationCoordinator::new();

        let cache = TestCache::default();

        let key = cache_key("sha256:expected");

        let other = cache_key("sha256:other");

        let snapshot = coordinator
            .capture(vec![
                subject("zamani.compiler.pipeline"),
            ])
            .expect("snapshot");

        let result = coordinator.publish_if_current(
            &cache,
            &key,
            &snapshot,
            artifact(&other),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_publication_after_dependency_invalidation() {
        let coordinator =
            JitInvalidationCoordinator::new();

        let cache = TestCache::default();

        let dependency =
            subject("zamani.compiler.optimization");

        let snapshot = coordinator
            .capture(vec![dependency.clone()])
            .expect("snapshot");

        coordinator
            .invalidate_subject(
                &cache,
                dependency,
                reason("zamani.compiler.assumption.changed"),
            )
            .expect("invalidate");

        let key = cache_key("sha256:stale");

        let result = coordinator
            .publish_if_current(
                &cache,
                &key,
                &snapshot,
                artifact(&key),
            )
            .expect("publication result");

        assert!(
            matches!(
                result,
                JitInvalidationPublication::Stale { .. }
            )
        );

        assert!(
            cache.get(&key).expect("lookup").is_none()
        );
    }

    #[test]
    fn current_snapshot_remains_publishable() {
        let coordinator =
            JitInvalidationCoordinator::new();

        let cache = TestCache::default();

        let dependency =
            subject("zamani.backend.compatibility");

        let snapshot = coordinator
            .capture(vec![dependency.clone()])
            .expect("snapshot");

        assert!(
            coordinator
                .is_current(&snapshot)
                .expect("current")
        );

        let key = cache_key("sha256:current");

        let result = coordinator
            .publish_if_current(
                &cache,
                &key,
                &snapshot,
                artifact(&key),
            )
            .expect("publish");

        assert!(result.is_published());
    }

    // ------------------------------------------------------------------------
    // Invalidation tests
    // ------------------------------------------------------------------------

    #[test]
    fn invalidates_all_dependent_artifacts() {
        let coordinator =
            JitInvalidationCoordinator::new();

        let cache = TestCache::default();

        let dependency =
            subject("zamani.specialization.assumption");

        let first = cache_key("sha256:first");

        let second = cache_key("sha256:second");

        publish(
            &coordinator,
            &cache,
            &first,
            vec![dependency.clone()],
        );

        publish(
            &coordinator,
            &cache,
            &second,
            vec![dependency.clone()],
        );

        let report = coordinator
            .invalidate_subject(
                &cache,
                dependency,
                reason("zamani.specialization.assumption.changed"),
            )
            .expect("invalidation");

        assert_eq!(report.affected_artifacts(), 2);

        assert_eq!(report.removed_from_cache(), 2);

        assert_eq!(report.already_absent(), 0);

        assert!(
            cache.get(&first).expect("first").is_none()
        );

        assert!(
            cache.get(&second).expect("second").is_none()
        );

        assert_eq!(
            coordinator.tracked_artifacts().expect("count"),
            0
        );
    }

    #[test]
    fn invalidating_one_dependency_preserves_unrelated_artifacts() {
        let coordinator =
            JitInvalidationCoordinator::new();

        let cache = TestCache::default();

        let first_dependency =
            subject("zamani.capability.quantum");

        let second_dependency =
            subject("zamani.capability.classical");

        let first = cache_key("sha256:quantum");

        let second = cache_key("sha256:classical");

        publish(
            &coordinator,
            &cache,
            &first,
            vec![first_dependency.clone()],
        );

        publish(
            &coordinator,
            &cache,
            &second,
            vec![second_dependency],
        );

        coordinator
            .invalidate_subject(
                &cache,
                first_dependency,
                reason("zamani.capability.quantum.changed"),
            )
            .expect("invalidate");

        assert!(
            cache.get(&first).expect("first").is_none()
        );

        assert!(
            cache.get(&second).expect("second").is_some()
        );
    }

    #[test]
    fn artifact_with_multiple_dependencies_is_removed_once() {
        let coordinator =
            JitInvalidationCoordinator::new();

        let cache = TestCache::default();

        let first_dependency =
            subject("zamani.compiler.optimization");

        let second_dependency =
            subject("zamani.backend.compatibility");

        let key = cache_key("sha256:multi-dependency");

        publish(
            &coordinator,
            &cache,
            &key,
            vec![
                first_dependency.clone(),
                second_dependency.clone(),
            ],
        );

        let report = coordinator
            .invalidate_subject(
                &cache,
                first_dependency,
                reason("zamani.compiler.optimization.changed"),
            )
            .expect("invalidate");

        assert_eq!(report.removed_from_cache(), 1);

        assert_eq!(
            coordinator.dependent_artifacts(
                &second_dependency
            ).expect("dependency count"),
            0
        );
    }

    #[test]
    fn reports_already_absent_cache_entries() {
        let coordinator =
            JitInvalidationCoordinator::new();

        let cache = TestCache::default();

        let dependency =
            subject("zamani.compiler.optimization");

        let key = cache_key("sha256:already-absent");

        publish(
            &coordinator,
            &cache,
            &key,
            vec![dependency.clone()],
        );

        cache.invalidate(&key).expect("external removal");

        let report = coordinator
            .invalidate_subject(
                &cache,
                dependency,
                reason("zamani.compiler.optimization.changed"),
            )
            .expect("invalidation");

        assert_eq!(report.removed_from_cache(), 0);

        assert_eq!(report.already_absent(), 1);
    }

    #[test]
    fn advances_generation_after_invalidation() {
        let coordinator =
            JitInvalidationCoordinator::new();

        let cache = TestCache::default();

        let dependency =
            subject("zamani.provider.compatibility");

        let report = coordinator
            .invalidate_subject(
                &cache,
                dependency.clone(),
                reason("zamani.provider.contract.changed"),
            )
            .expect("invalidate");

        assert_eq!(report.generation(), 1);

        assert_eq!(
            coordinator.current_generation(&dependency)
                .expect("generation"),
            1
        );
    }

    #[test]
    fn invalidates_artifact_by_exact_cache_identity() {
        let coordinator =
            JitInvalidationCoordinator::new();

        let cache = TestCache::default();

        let key = cache_key("sha256:exact");

        publish(
            &coordinator,
            &cache,
            &key,
            vec![
                subject("zamani.compiler.pipeline"),
            ],
        );

        assert!(
            coordinator
                .invalidate_artifact(&cache, &key)
                .expect("invalidate")
        );

        assert_eq!(
            coordinator.tracked_artifacts().expect("count"),
            0
        );
    }

    // ------------------------------------------------------------------------
    // Independent coordinator tests
    // ------------------------------------------------------------------------

    #[test]
    fn coordinators_have_independent_state() {
        let first =
            JitInvalidationCoordinator::new();

        let second =
            JitInvalidationCoordinator::new();

        let dependency =
            subject("zamani.compiler.optimization");

        let cache = TestCache::default();

        first
            .invalidate_subject(
                &cache,
                dependency.clone(),
                reason("zamani.compiler.optimization.changed"),
            )
            .expect("first invalidation");

        assert_eq!(
            first.current_generation(&dependency)
                .expect("first generation"),
            1
        );

        assert_eq!(
            second.current_generation(&dependency)
                .expect("second generation"),
            0
        );
    }

    // ------------------------------------------------------------------------
    // API tests
    // ------------------------------------------------------------------------

    #[test]
    fn api_version_is_available() {
        assert_eq!(JIT_INVALIDATION_API_VERSION, 1);
    }
}
