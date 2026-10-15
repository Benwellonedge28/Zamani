
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Universal Intermediate Representation — Incremental Compilation
//!
//! Path: `src/zuir/incremental/mod.rs`
//!
//! PURPOSE
//!
//! Provide the public module boundary for ZUIR incremental compilation.
//! This module connects dependency tracking, invalidation planning, and
//! deterministic cache-key derivation without duplicating their algorithms.
//!
//! ARCHITECTURAL AUTHORITY
//!
//! - `dependencies.rs` owns logical artifact identities and dependency graphs.
//! - `invalidation.rs` owns invalidation plans and graph-freshness checks.
//! - `cache_keys.rs` owns cache-key input contracts and key derivation.
//! - `fingerprint/stable_hash.rs` owns stable content fingerprinting.
//! - `version.rs` owns ZUIR schema and serialization format versions.
//! - `ZUIR-SPEC.md` owns normative ZUIR semantics.
//! - The grammar specification owns source-language and grammar semantics.
//!
//! RESPONSIBILITIES
//!
//! - Declare the incremental-compilation modules.
//! - Publish their supported public types.
//! - Preserve clear ownership and dependency direction.
//! - Keep graph mutation, invalidation, and cache identity separate.
//!
//! DOES NOT OWN
//!
//! - Dependency graph algorithms.
//! - Cache-key hashing or content fingerprint algorithms.
//! - Cache storage, eviction, persistence, or synchronization.
//! - Source parsing, semantic analysis, IR verification, or lowering.
//! - Hardware discovery, resource allocation, or runtime execution.
//!
//! DEPENDENCY DIRECTION
//!
//! `dependencies` is the foundational graph component.
//! `invalidation` consumes `dependencies`.
//! `cache_keys` consumes dependency identities and stable fingerprints.
//! The compilation coordinator integrates all three.
//!
//! No component in this module may depend on a concrete hardware limit,
//! assume every target supports every capability, or silently reinterpret
//! a missing dependency as a valid dependency-free artifact.
//!
//! SCALABILITY
//!
//! This module imposes no fixed limit on artifacts, dependency edges,
//! operations, qubits, devices, nodes, or tensor dimensions.
//! Practical scale depends on available resources and the algorithms in
//! the underlying modules. Resource exhaustion must be handled explicitly.
//!
//! POCO-REAF
//!
//! The module boundary is target-independent. Target-specific artifact
//! identity and feasibility belong in the inputs and downstream target
//! adapters, not in a universal hardware-specific cache policy.
//!
//! INTEGRATION
//!
//! The parent ZUIR module must declare `pub mod incremental;`.
//! Consumers should prefer the public re-exports below over reaching into
//! implementation details. Add new incremental components only when their
//! responsibilities and integration contracts are independently defined.
//!
//! FREEZE CRITERIA
//!
//! - Every declared module exists.
//! - All re-exports match real public definitions.
//! - No duplicate owners or circular module dependencies are introduced.
//! - The module compiles with the complete repository.
//! - Public API and incremental integration tests pass.

/// Logical artifact identities, dependency edges, and graph traversal.
pub mod dependencies;

/// Stable cache-key input contracts and key derivation.
pub mod cache_keys;

/// Deterministic invalidation planning and freshness validation.
pub mod invalidation;

// ---------------------------------------------------------------------------
// Public cache-key API
// ---------------------------------------------------------------------------

pub use cache_keys::{
    CacheKey,
    CacheKeyError,
    CacheKeyInput,
    DependencyFingerprint,
    CACHE_KEY_DOMAIN,
};

// ---------------------------------------------------------------------------
// Public dependency-graph API
// ---------------------------------------------------------------------------

pub use dependencies::{
    DependencyEdge,
    DependencyError,
    DependencyGraph,
    DependencyKey,
    DependencyKind,
    DependencyResult,
};

// ---------------------------------------------------------------------------
// Public invalidation API
// ---------------------------------------------------------------------------

pub use invalidation::{
    InvalidationCause,
    InvalidationError,
    InvalidationPlan,
    InvalidationPlanner,
    InvalidationResult,
};
