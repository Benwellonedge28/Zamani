
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Incremental Compilation Cache
//!
//! Path: src/compiler/incremental/cache.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Provide the compiler-level integration boundary for incremental
//! compilation caching.
//!
//! This module connects compiler stages to the repository's existing
//! content-addressed backend artifact cache.
//!
//! It does not implement another cache-storage engine.
//!
//! ============================================================================
//! ARCHITECTURAL AUTHORITY
//! ============================================================================
//!
//! Existing authoritative implementations:
//!
//! - src/backend/cache.rs
//!     Owns thread-safe artifact storage, integrity checking,
//!     eviction, resource accounting, and cache statistics.
//!
//! - src/frontend/semantic/incremental/cache.rs
//!     Owns semantic-fact caching and AST NodeId-based validity.
//!
//! - src/frontend/semantic/incremental/dependencies.rs
//!     Owns semantic dependency relationships.
//!
//! - src/frontend/semantic/incremental/invalidation.rs
//!     Owns semantic invalidation planning.
//!
//! - src/zuir/incremental/cache_keys.rs
//!     Owns stable ZUIR artifact cache-key derivation.
//!
//! - src/zuir/incremental/dependencies.rs
//!     Owns ZUIR artifact dependencies.
//!
//! - src/zuir/incremental/invalidation.rs
//!     Owns ZUIR invalidation planning.
//!
//! This module coordinates compiler-stage cache access without
//! duplicating those responsibilities.
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! THIS MODULE OWNS:
//!
//! - Compiler-stage cache-key input contracts.
//! - Compiler-stage cache-key derivation.
//! - Compiler-facing cache type exports.
//! - Compiler-facing cache integration.
//! - Documentation of compiler-stage cache correctness.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - Cache storage implementation.
//! - Cache eviction algorithms.
//! - Cache locking.
//! - Cache payload integrity algorithms.
//! - AST node identity.
//! - Semantic dependency relationships.
//! - Dependency invalidation algorithms.
//! - ZUIR fingerprint algorithms.
//! - Source parsing.
//! - Semantic analysis.
//! - IR generation.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Backend generation.
//! - Runtime execution.
//!
//! ============================================================================
//! CACHE CORRECTNESS
//! ============================================================================
//!
//! A cached compilation result may only be reused when all inputs capable
//! of changing the result are represented in its cache identity.
//!
//! The caller must supply:
//!
//! - Cache schema version.
//! - Compiler-stage identity.
//! - Compiler-stage implementation version.
//! - Input fingerprint.
//! - Compiler fingerprint.
//! - Configuration fingerprint.
//! - Dependency fingerprint.
//! - Optional toolchain fingerprint.
//! - Optional target fingerprint.
//! - Artifact-kind identity.
//!
//! Missing fingerprints must never be replaced with fabricated defaults.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! Target-independent compilation stages must not be unnecessarily bound
//! to the machine on which the compiler happens to execute.
//!
//! Target-specific compilation stages must include the complete target
//! identity in their cache keys.
//!
//! A portable cache entry may be reused across targets only when its
//! producing stage and all other artifact-affecting inputs are equivalent.
//!
//! Cache eviction, cache misses, cache clearing, and process restarts
//! must never change program semantics.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! This module introduces no universal limits on:
//!
//! - Program size.
//! - IR operation count.
//! - Functions.
//! - Modules.
//! - Dependencies.
//! - Compilation stages.
//! - Devices.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Distributed nodes.
//! - Memory.
//! - Tensor dimensions.
//! - Quantum resources.
//!
//! Operational cache budgets are delegated to CacheConfig.
//!
//! Actual memory consumption remains constrained by available resources,
//! the process address space, and the operating system.
//!
//! Literal infinity is not a realizable implementation guarantee.
//!
//! ============================================================================
//! CONCURRENCY
//! ============================================================================
//!
//! BackendCache provides thread-safe storage.
//!
//! Cloned CompilationCache instances share the same underlying cache.
//!
//! Concurrent misses may independently execute compilation work.
//! This module does not promise single-flight compilation.
//!
//! No compilation callback is executed while the cache storage mutex
//! is held.
//!
//! ============================================================================
//! INTEGRATION
//! ============================================================================
//!
//! Compiler orchestration:
//!
//! 1. Obtain verified compilation inputs.
//! 2. Collect complete input and dependency fingerprints.
//! 3. Construct StageCacheKeyParts.
//! 4. Derive the cache key.
//! 5. Look up the existing artifact cache.
//! 6. Validate the cached artifact's format and compatibility.
//! 7. Recompile on a cache miss.
//! 8. Verify the newly generated result.
//! 9. Insert only successful, validated results.
//!
//! Dependency invalidation remains the responsibility of the
//! incremental coordinator.
//!
//! ============================================================================

use std::fmt;
use std::sync::Arc;

use crate::backend::cache::{
    BackendCache,
    CacheConfig,
    CacheKey,
    CacheKeyError,
    CacheKeyParts,
    CacheStats,
    CacheUsage,
    InsertOutcome,
};

/// Compiler incremental-cache schema version.
///
/// Changing the schema requires a deliberate cache-identity migration.
pub const COMPILER_CACHE_SCHEMA_VERSION: &str =
    "zamani.compiler.incremental-cache.v1";

/// Canonical identifier for a target-independent compilation result.
///
/// This is a cache identity marker, not a hardware architecture.
pub const TARGET_INDEPENDENT_ID: &str =
    "zamani.target-independent";

/// Compiler-facing cache-key type.
pub type CompilationCacheKey = CacheKey;

/// Compiler-facing cache configuration.
pub type CompilationCacheConfig = CacheConfig;

/// Compiler-facing cache statistics.
pub type CompilationCacheStats = CacheStats;

/// Compiler-facing cache occupancy.
pub type CompilationCacheUsage = CacheUsage;

/// Compiler-facing insertion result.
pub type CompilationCacheInsertOutcome = InsertOutcome;

/// Compiler-facing cache implementation.
///
/// This is an alias of the existing authoritative backend artifact cache.
/// It does not introduce a second storage implementation.
pub type CompilationCache = BackendCache;

// ============================================================================
// STAGE CACHE KEY INPUT
// ============================================================================

/// Complete identity inputs for a compiler-stage cache entry.
///
/// Every field must be supplied explicitly.
///
/// Fingerprints must be generated by the corresponding authoritative
/// subsystem using deterministic encoding.
///
/// Do not use:
///
/// - Debug formatting.
/// - Pointer addresses.
/// - Randomized hashers.
/// - Wall-clock timestamps.
/// - Process-local identifiers.
/// - Unordered map iteration.
///
/// `target_fingerprint: None` identifies a target-independent stage result.
///
/// `Some(fingerprint)` identifies a result whose output depends on a
/// particular target realization.
///
/// `toolchain_fingerprint: None` means no external toolchain identity
/// applies to this compilation stage.
#[derive(Debug, Clone, Copy)]
pub struct StageCacheKeyParts<'a> {
    /// Stable compiler-stage identifier.
    ///
    /// Examples:
    ///
    /// `semantic-analysis`
    /// `zuir-verification`
    /// `classical-lowering`
    /// `quantum-lowering`
    /// `hdl-lowering`
    /// `ai-lowering`
    /// `wasm-emission`
    /// `llvm-emission`
    ///
    /// These are extensible identifiers, not a closed enum.
    pub stage_id: &'a str,

    /// Version of the stage implementation.
    ///
    /// Changing behavior in a way that can change its output must
    /// change this version or the relevant compiler fingerprint.
    pub stage_version: &'a str,

    /// Fingerprint of the complete direct input.
    ///
    /// This may identify source-independent IR, a verified module,
    /// a compilation unit, or another canonical stage input.
    pub input_fingerprint: &'a str,

    /// Compiler implementation fingerprint.
    ///
    /// Must identify all compiler implementation details that can
    /// change the resulting artifact.
    pub compiler_fingerprint: &'a str,

    /// Fingerprint of all artifact-affecting stage configuration.
    pub configuration_fingerprint: &'a str,

    /// Fingerprint of the complete relevant dependency set.
    ///
    /// An empty dependency set must still have a canonical,
    /// nonempty fingerprint.
    pub dependency_fingerprint: &'a str,

    /// Optional external toolchain fingerprint.
    ///
    /// For example, the identity of an external LLVM toolchain.
    pub toolchain_fingerprint: Option<&'a str>,

    /// Optional target fingerprint.
    ///
    /// None means the stage result is target-independent.
    pub target_fingerprint: Option<&'a str>,

    /// Stable artifact-kind identifier.
    ///
    /// Examples:
    ///
    /// `verified-zuir`
    /// `classical-ir`
    /// `quantum-ir`
    /// `hdl-ir`
    /// `wasm-module`
    /// `llvm-ir`
    ///
    /// Custom registered artifact kinds are supported.
    pub artifact_kind: &'a str,
}

// ============================================================================
// KEY DERIVATION
// ============================================================================

/// Derive a deterministic compiler-stage cache key.
///
/// The underlying cache-key implementation performs:
///
/// - Domain-separated hashing.
/// - Length-prefixed component encoding.
/// - Optional-field presence encoding.
/// - Empty-component validation.
/// - Deterministic digest construction.
///
/// The compiler stage is represented as the producing provider identity.
///
/// The stage version is incorporated into the versioned cache schema
/// component so that changing the stage implementation changes the key.
///
/// Target-independent stages use TARGET_INDEPENDENT_ID.
///
/// Target-specific stages use their supplied target fingerprint.
///
/// # Errors
///
/// Returns CacheKeyError when a required component is empty or a
/// present optional component is empty.
///
/// # Correctness
///
/// This function cannot compensate for incomplete fingerprints.
///
/// The caller must include every input that can change the result.
pub fn derive_stage_cache_key(
    parts: StageCacheKeyParts<'_>,
) -> Result<CompilationCacheKey, CacheKeyError> {
    let schema_version = format!(
        "{}:{}",
        COMPILER_CACHE_SCHEMA_VERSION,
        parts.stage_version
    );

    let target_id = parts
        .target_fingerprint
        .unwrap_or(TARGET_INDEPENDENT_ID);

    CacheKey::from_parts(CacheKeyParts {
        schema_version: &schema_version,

        input_fingerprint: parts.input_fingerprint,

        compiler_fingerprint: parts.compiler_fingerprint,

        // The backend cache's provider identity is used here to identify
        // the compiler stage that produced this cached result.
        backend_id: parts.stage_id,

        target_id,

        config_fingerprint: parts.configuration_fingerprint,

        toolchain_fingerprint: parts.toolchain_fingerprint,

        dependency_fingerprint: parts.dependency_fingerprint,

        artifact_kind: parts.artifact_kind,
    })
}

// ============================================================================
// CACHE CONSTRUCTION
// ============================================================================

/// Create a compiler incremental cache.
///
/// The supplied policy controls cache admission and retention.
///
/// A disabled cache remains a valid configuration. Compilation must
/// continue normally when caching is disabled.
///
/// # Example
///
/// ```
/// use zamani_compiler::compiler::incremental::cache::{
///     CompilationCache,
///     CompilationCacheConfig,
/// };
///
/// let cache = CompilationCache::new(
///     CompilationCacheConfig::bounded(
///         1000,
///         512 * 1024 * 1024,
///         64 * 1024 * 1024,
///     )
/// );
///
/// assert_eq!(cache.usage().entries, 0);
/// ```
#[must_use]
pub fn new_compilation_cache(
    config: CompilationCacheConfig,
) -> CompilationCache {
    CompilationCache::new(config)
}

// ============================================================================
// CACHE LOOKUP
// ============================================================================

/// Retrieve a cached compilation result.
///
/// A successful result is returned as immutable shared bytes.
///
/// A cache miss returns None.
///
/// The caller must validate the artifact format, version,
/// integrity requirements, and target compatibility before using it.
///
/// Payload-integrity checking is performed by BackendCache.
///
/// This function does not certify semantic correctness.
#[must_use]
pub fn get_cached_result(
    cache: &CompilationCache,
    key: &CompilationCacheKey,
) -> Option<Arc<[u8]>> {
    cache.get(key)
}

// ============================================================================
// CACHE INSERTION
// ============================================================================

/// Insert a successfully compiled and validated result.
///
/// Only verified successful results should reach this function.
///
/// Do not insert:
///
/// - Failed compilation results.
/// - Incomplete compilation results.
/// - Diagnostics-only results.
/// - Unsupported-target results.
/// - Invalid IR.
/// - Unverified generated artifacts.
///
/// A rejected cache insertion does not mean compilation failed.
///
/// The original successful compilation result remains available to
/// the caller even when the cache cannot retain it.
pub fn insert_compiled_result(
    cache: &CompilationCache,
    key: CompilationCacheKey,
    payload: Vec<u8>,
) -> CompilationCacheInsertOutcome {
    cache.insert(key, payload)
}

// ============================================================================
// CACHE INVALIDATION
// ============================================================================

/// Invalidate one compiler-stage cache entry.
///
/// Dependency-aware invalidation planning remains with the incremental
/// coordinator.
///
/// This function only removes the specified cache identity.
///
/// The coordinator must determine all affected keys before invoking it.
pub fn invalidate_compilation_result(
    cache: &CompilationCache,
    key: &CompilationCacheKey,
) -> bool {
    cache.invalidate(key)
}

/// Clear all compiler-stage cache entries.
///
/// This is an operational cache-management operation.
///
/// It does not modify source programs, dependency graphs, semantic facts,
/// IR, or program meaning.
pub fn clear_compilation_cache(cache: &CompilationCache) {
    cache.clear();
}

// ============================================================================
// CACHE OBSERVABILITY
// ============================================================================

/// Obtain current cache occupancy.
#[must_use]
pub fn compilation_cache_usage(
    cache: &CompilationCache,
) -> CompilationCacheUsage {
    cache.usage()
}

/// Obtain a consistent cache statistics snapshot.
#[must_use]
pub fn compilation_cache_stats(
    cache: &CompilationCache,
) -> CompilationCacheStats {
    cache.stats()
}

// ============================================================================
// ERRORS AND FORMATTING
// ============================================================================

/// Compiler-level cache integration error.
///
/// Key derivation errors are preserved without converting them into
/// successful cache misses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompilationCacheError {
    /// Invalid cache identity.
    InvalidKey(CacheKeyError),
}

impl fmt::Display for CompilationCacheError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidKey(error) => {
                write!(
                    formatter,
                    "compiler incremental cache key error: {error}"
                )
            }
        }
    }
}

impl std::error::Error for CompilationCacheError {}

impl From<CacheKeyError> for CompilationCacheError {
    fn from(error: CacheKeyError) -> Self {
        Self::InvalidKey(error)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn parts<'a>(
        input: &'a str,
    ) -> StageCacheKeyParts<'a> {
        StageCacheKeyParts {
            stage_id: "classical-lowering",
            stage_version: "1",
            input_fingerprint: input,
            compiler_fingerprint: "compiler-build-v1",
            configuration_fingerprint: "configuration-v1",
            dependency_fingerprint: "dependencies-v1",
            toolchain_fingerprint: None,
            target_fingerprint: None,
            artifact_kind: "classical-ir",
        }
    }

    fn key(input: &str) -> CompilationCacheKey {
        derive_stage_cache_key(parts(input))
            .expect("test cache key must be valid")
    }

    #[test]
    fn identical_inputs_produce_identical_keys() {
        assert_eq!(key("input-a"), key("input-a"));
    }

    #[test]
    fn different_inputs_produce_different_keys() {
        assert_ne!(key("input-a"), key("input-b"));
    }

    #[test]
    fn different_stages_produce_different_keys() {
        let first = derive_stage_cache_key(parts("input"))
            .expect("first key");

        let second = derive_stage_cache_key(
            StageCacheKeyParts {
                stage_id: "quantum-lowering",
                ..parts("input")
            },
        )
        .expect("second key");

        assert_ne!(first, second);
    }

    #[test]
    fn different_stage_versions_produce_different_keys() {
        let first = derive_stage_cache_key(parts("input"))
            .expect("first key");

        let second = derive_stage_cache_key(
            StageCacheKeyParts {
                stage_version: "2",
                ..parts("input")
            },
        )
        .expect("second key");

        assert_ne!(first, second);
    }

    #[test]
    fn target_independent_and_target_specific_keys_differ() {
        let portable = derive_stage_cache_key(parts("input"))
            .expect("portable key");

        let targeted = derive_stage_cache_key(
            StageCacheKeyParts {
                target_fingerprint: Some("target-fingerprint-a"),
                ..parts("input")
            },
        )
        .expect("target-specific key");

        assert_ne!(portable, targeted);
    }

    #[test]
    fn different_targets_produce_different_keys() {
        let first = derive_stage_cache_key(
            StageCacheKeyParts {
                target_fingerprint: Some("target-a"),
                ..parts("input")
            },
        )
        .expect("first target");

        let second = derive_stage_cache_key(
            StageCacheKeyParts {
                target_fingerprint: Some("target-b"),
                ..parts("input")
            },
        )
        .expect("second target");

        assert_ne!(first, second);
    }

    #[test]
    fn different_dependencies_produce_different_keys() {
        let first = derive_stage_cache_key(parts("input"))
            .expect("first key");

        let second = derive_stage_cache_key(
            StageCacheKeyParts {
                dependency_fingerprint: "dependencies-v2",
                ..parts("input")
            },
        )
        .expect("second key");

        assert_ne!(first, second);
    }

    #[test]
    fn different_configurations_produce_different_keys() {
        let first = derive_stage_cache_key(parts("input"))
            .expect("first key");

        let second = derive_stage_cache_key(
            StageCacheKeyParts {
                configuration_fingerprint: "configuration-v2",
                ..parts("input")
            },
        )
        .expect("second key");

        assert_ne!(first, second);
    }

    #[test]
    fn missing_required_fingerprint_is_rejected() {
        let result = derive_stage_cache_key(
            StageCacheKeyParts {
                input_fingerprint: "",
                ..parts("input")
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn empty_present_toolchain_fingerprint_is_rejected() {
        let result = derive_stage_cache_key(
            StageCacheKeyParts {
                toolchain_fingerprint: Some(""),
                ..parts("input")
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn empty_present_target_fingerprint_is_rejected() {
        let result = derive_stage_cache_key(
            StageCacheKeyParts {
                target_fingerprint: Some(""),
                ..parts("input")
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn cache_insert_and_retrieval_work() {
        let cache = new_compilation_cache(
            CompilationCacheConfig::bounded(
                10,
                1024,
                512,
            ),
        );

        let cache_key = key("input");

        let result = insert_compiled_result(
            &cache,
            cache_key,
            b"verified-ir".to_vec(),
        );

        assert_eq!(
            result,
            InsertOutcome::Inserted
        );

        let retrieved = get_cached_result(
            &cache,
            &cache_key,
        )
        .expect("cache entry must exist");

        assert_eq!(
            retrieved.as_ref(),
            b"verified-ir"
        );
    }

    #[test]
    fn cache_miss_returns_none() {
        let cache = new_compilation_cache(
            CompilationCacheConfig::bounded(
                10,
                1024,
                512,
            ),
        );

        assert!(get_cached_result(
            &cache,
            &key("missing"),
        ).is_none());
    }

    #[test]
    fn disabled_cache_does_not_retain_entries() {
        let cache = new_compilation_cache(
            CompilationCacheConfig::default(),
        );

        let result = insert_compiled_result(
            &cache,
            key("input"),
            b"artifact".to_vec(),
        );

        assert_eq!(
            result,
            InsertOutcome::RejectedDisabled
        );

        assert_eq!(cache.usage().entries, 0);
    }

    #[test]
    fn invalidation_removes_only_requested_entry() {
        let cache = new_compilation_cache(
            CompilationCacheConfig::bounded(
                10,
                1024,
                512,
            ),
        );

        let first = key("first");
        let second = key("second");

        cache.insert(first, b"first".to_vec());
        cache.insert(second, b"second".to_vec());

        assert!(invalidate_compilation_result(
            &cache,
            &first,
        ));

        assert!(get_cached_result(
            &cache,
            &first,
        ).is_none());

        assert!(get_cached_result(
            &cache,
            &second,
        ).is_some());
    }

    #[test]
    fn cloned_cache_handles_share_state() {
        let first = new_compilation_cache(
            CompilationCacheConfig::bounded(
                10,
                1024,
                512,
            ),
        );

        let second = first.clone();

        let cache_key = key("shared");

        first.insert(
            cache_key,
            b"shared-artifact".to_vec(),
        );

        assert!(
            get_cached_result(&second, &cache_key)
                .is_some()
        );
    }

    #[test]
    fn clearing_cache_does_not_change_key_identity() {
        let cache = new_compilation_cache(
            CompilationCacheConfig::bounded(
                10,
                1024,
                512,
            ),
        );

        let cache_key = key("input");

        cache.insert(
            cache_key,
            b"artifact".to_vec(),
        );

        clear_compilation_cache(&cache);

        assert_eq!(cache.usage().entries, 0);

        assert_eq!(cache_key, key("input"));
    }
}
