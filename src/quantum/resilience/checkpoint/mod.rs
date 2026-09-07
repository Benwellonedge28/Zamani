//! Zamani Quantum Resilience — Checkpoint Subsystem
//!
//! Path:
//!     src/quantum/resilience/checkpoint/mod.rs
//!
//! Purpose:
//!     Composition boundary for the production checkpoint subsystem of
//!     `crate::quantum::resilience`.
//!
//! # Architectural role
//!
//! This module is intentionally a composition root. It owns:
//!
//! - checkpoint child-module declarations;
//! - public API boundaries;
//! - carefully selected compatibility re-exports;
//! - subsystem-level documentation;
//! - compile-time safety policy.
//!
//! It does NOT own:
//!
//! - checkpoint state semantics;
//! - manifest implementation;
//! - snapshot implementation;
//! - storage implementation;
//! - cryptographic implementation;
//! - compatibility implementation;
//! - recovery orchestration;
//! - serialization algorithms;
//! - quantum-state capture;
//! - hardware selection;
//! - routing;
//! - scheduling;
//! - QEC;
//! - vendor/provider logic.
//!
//! Those responsibilities belong to the appropriate child subsystem.
//!
//! # Module architecture
//!
//! ```text
//! quantum::resilience::checkpoint
//! │
//! ├── checkpoint
//! │     Canonical checkpoint semantic model and lifecycle
//! │
//! ├── manifest
//! │     Manifest identity, artifact enumeration and ordering
//! │
//! ├── snapshot
//! │     Runtime/provider snapshot semantics
//! │
//! ├── storage
//! │     Provider-neutral persistence contracts
//! │
//! ├── integrity
//! │     Integrity/authenticity contracts
//! │
//! └── compatibility
//!       Restore/schema/target compatibility contracts
//! ```
//!
//! The resulting dependency direction is:
//!
//! ```text
//!                         checkpoint
//!                              │
//!             ┌────────────────┼────────────────┐
//!             │                │                │
//!             ▼                ▼                ▼
//!         manifest          snapshot         storage
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              │
//!                    ┌─────────┴─────────┐
//!                    ▼                   ▼
//!                integrity        compatibility
//!                    │                   │
//!                    └─────────┬─────────┘
//!                              ▼
//!                         recovery::*
//!                              │
//!                              ▼
//!                         verification
//! ```
//!
//! The exact implementation dependency graph is defined by the child
//! contracts. This file must remain free of operational logic.
//!
//! # Canonical quantum identity
//!
//! The checkpoint subsystem MUST NOT define a competing qubit identity.
//!
//! When a checkpoint API requires a quantum qubit identifier, the canonical
//! identity is:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! Physical/logical distinctions must be represented using the canonical types
//! exposed by the surrounding quantum architecture rather than creating a
//! checkpoint-local `QubitId`.
//!
//! # Write once, scale everywhere
//!
//! A checkpoint is a semantic execution artifact, not a representation of one
//! particular machine size.
//!
//! This module therefore imposes no architectural maximum on:
//!
//! - qubits;
//! - logical qubits;
//! - physical qubits;
//! - checkpoint count;
//! - artifact count;
//! - artifact size;
//! - payload size;
//! - devices;
//! - backends;
//! - execution regions;
//! - checkpoint history.
//!
//! Concrete limits are supplied by:
//!
//! - storage capabilities;
//! - runtime resource policies;
//! - hardware capabilities;
//! - memory availability;
//! - execution policy;
//! - security policy;
//! - provider limits;
//! - caller-selected budgets.
//!
//! The subsystem must never introduce constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_CHECKPOINTS
//! MAX_ARTIFACTS
//! MAX_CHECKPOINT_BYTES
//! DEFAULT_QUBIT_COUNT
//! ```
//!
//! merely to make implementation easier.
//!
//! "Infinity" means that the semantic API does not impose an artificial finite
//! ceiling. Every concrete execution remains bounded by resources actually
//! available to that execution.
//!
//! # Quantum-state correctness
//!
//! A checkpoint does NOT automatically make an arbitrary quantum state
//! restorable.
//!
//! In particular, this composition boundary must not imply that:
//!
//! ```text
//! unknown quantum state
//! +
//! serialized bytes
//! =
//! restorable quantum state
//! ```
//!
//! Restoration is valid only when the selected checkpoint boundary and runtime,
//! provider, QEC system, or reconstruction procedure explicitly support it.
//!
//! Supported semantic categories are defined by `checkpoint.rs`, including:
//!
//! - program-start reconstruction;
//! - classical execution state;
//! - measurement boundaries;
//! - logical/QEC boundaries;
//! - provider-supported snapshots;
//! - reconstructible runtime state;
//! - replayable/compiled execution state.
//!
//! # Persistence boundary
//!
//! Checkpoint metadata and semantic identity are distinct from physical
//! persistence.
//!
//! ```text
//! checkpoint
//!     │
//!     ├── semantic identity
//!     ├── lifecycle
//!     ├── boundary
//!     └── payload descriptors
//!
//! storage
//!     │
//!     ├── object addressing
//!     ├── streaming
//!     ├── conditional writes
//!     ├── versions
//!     └── persistence
//! ```
//!
//! This separation allows the same checkpoint model to work with:
//!
//! - local storage;
//! - filesystem storage;
//! - database storage;
//! - object storage;
//! - distributed storage;
//! - encrypted storage;
//! - provider-managed storage;
//! - future storage systems.
//!
//! No storage provider is hard-coded here.
//!
//! # Integrity boundary
//!
//! Cryptographic operations are not implemented by this composition root.
//!
//! `integrity.rs` owns the integrity/authenticity contract.
//!
//! This module therefore does not:
//!
//! - implement hash functions;
//! - implement signatures;
//! - embed cryptographic keys;
//! - trust provider names;
//! - silently accept unverifiable data.
//!
//! Recovery must verify checkpoint integrity before accepting a checkpoint as
//! authoritative.
//!
//! # Compatibility boundary
//!
//! `compatibility.rs` owns compatibility negotiation.
//!
//! Compatibility can involve:
//!
//! - checkpoint schema;
//! - manifest schema;
//! - program identity;
//! - IR schema;
//! - resilience schema;
//! - target capabilities;
//! - hardware generation;
//! - QEC configuration;
//! - runtime capabilities;
//! - backend interface.
//!
//! This module does not duplicate those rules.
//!
//! # Recovery integration
//!
//! The checkpoint subsystem is consumed by:
//!
//! ```text
//! crate::quantum::resilience::recovery
//! ```
//!
//! Recovery owns orchestration such as:
//!
//! ```text
//! locate checkpoint
//!       ↓
//! validate metadata
//!       ↓
//! validate integrity
//!       ↓
//! validate compatibility
//!       ↓
//! restore supported state
//!       ↓
//! reconstruct/recompile when required
//!       ↓
//! verify semantics
//!       ↓
//! resume execution
//! ```
//!
//! Checkpoint itself does not decide whether recovery should happen.
//!
//! # State integration
//!
//! `crate::quantum::resilience::state` records execution/recovery state and
//! coordinates lifecycle information.
//!
//! It must not assume that persisted execution metadata is itself a quantum
//! state snapshot.
//!
//! The checkpoint subsystem remains the authority for checkpoint semantics.
//!
//! # Verification integration
//!
//! Restored state must ultimately be verified by the resilience verification
//! subsystem before a recovered result is accepted.
//!
//! The architectural rule is:
//!
//! ```text
//! persisted
//!     ≠
//! trusted
//!
//! restored
//!     ≠
//! accepted
//! ```
//!
//! A successful storage read is therefore not sufficient for recovery.
//!
//! # Serialization integration
//!
//! Serialization belongs to the dedicated resilience serialization subsystem
//! and to semantic objects that own their wire representation.
//!
//! This module does not define a second checkpoint encoding.
//!
//! The composition boundary only exposes the typed checkpoint structures that
//! serialization implementations may encode/decode.
//!
//! # Determinism
//!
//! This composition root performs no stochastic operations.
//!
//! It owns no:
//!
//! - global RNG;
//! - wall-clock-driven policy;
//! - hidden mutable state;
//! - random identifiers;
//! - implicit ordering decisions.
//!
//! Child modules must provide deterministic behavior where their contracts
//! require it.
//!
//! Manifest ordering is explicitly represented by artifact sequence metadata.
//!
//! # Concurrency
//!
//! This module owns no global mutable state.
//!
//! Child implementations may provide `Send`/`Sync` types where appropriate.
//!
//! No unnecessary synchronization primitive is introduced here.
//!
//! `Arc`-based immutable structures may be used by child contracts where that
//! improves sharing without introducing mutable global state.
//!
//! # Security
//!
//! Checkpointing is security-sensitive because a compromised checkpoint can
//! cause:
//!
//! - execution-state corruption;
//! - rollback to stale state;
//! - execution on an unintended target;
//! - semantic divergence;
//! - replay attacks;
//! - provenance loss;
//! - availability attacks.
//!
//! Therefore:
//!
//! 1. checkpoint identity must remain opaque;
//! 2. storage references must remain provider-neutral;
//! 3. integrity must be verified by the integrity subsystem;
//! 4. compatibility must be checked before restoration;
//! 5. authorization must be enforced by the recovery/security boundary;
//! 6. restored results must pass semantic verification;
//! 7. no child module may silently weaken a caller's safety policy.
//!
//! # No unsafe Rust
//!
//! The entire checkpoint subsystem is required to use safe Rust.
//!
//! This file explicitly forbids unsafe code.
//!
//! Child modules independently enforce the same rule.
//!
//! No:
//!
//! - raw-pointer manipulation;
//! - `unsafe` blocks;
//! - `unsafe fn`;
//! - unsafe trait implementations;
//! - unchecked FFI;
//! - backend-specific unsafe primitives;
//!
//! belong in this subsystem.
//!
//! # Public API policy
//!
//! The preferred API is namespace-oriented:
//!
//! ```text
//! crate::quantum::resilience::checkpoint::checkpoint
//! crate::quantum::resilience::checkpoint::manifest
//! crate::quantum::resilience::checkpoint::snapshot
//! crate::quantum::resilience::checkpoint::storage
//! crate::quantum::resilience::checkpoint::integrity
//! crate::quantum::resilience::checkpoint::compatibility
//! ```
//!
//! Selected canonical types are re-exported below for ergonomic use.
//!
//! Wildcard re-exports are deliberately forbidden:
//!
//! ```text
//! pub use checkpoint::*;
//! pub use manifest::*;
//! pub use storage::*;
//! ```
//!
//! Such exports create API collisions and make ownership ambiguous.
//!
//! # Compatibility
//!
//! Re-exports in this file are compatibility conveniences only.
//!
//! They must always refer to the canonical implementation in exactly one child
//! module.
//!
//! This file must never create duplicate semantic types.
//!
//! # Module completion rule
//!
//! A child module is declared here only when its implementation exists in the
//! repository and is intended to be part of the checkpoint subsystem.
//!
//! The current checkpoint directory contains:
//!
//! - `checkpoint.rs`;
//! - `compatibility.rs`;
//! - `integrity.rs`;
//! - `manifest.rs`;
//! - `snapshot.rs`;
//! - `storage.rs`.
//!
//! This file intentionally declares exactly those completed modules.
//!
//! Future modules may be added here when their files and contracts are
//! independently complete.
//!
//! # Rust compatibility
//!
//! Required toolchain:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! The implementation intentionally avoids newer language features that would
//! unnecessarily raise the minimum compiler version.
//!
//! # Integration contract
//!
//! The checkpoint composition root has the following dependency responsibilities:
//!
//! ```text
//! checkpoint/mod.rs
//!     │
//!     ├── declares checkpoint semantic module
//!     ├── declares manifest module
//!     ├── declares snapshot module
//!     ├── declares storage module
//!     ├── declares integrity module
//!     └── declares compatibility module
//!
//! checkpoint.rs
//!     │
//!     ├── canonical checkpoint model
//!     └── checkpoint lifecycle
//!
//! manifest.rs
//!     │
//!     └── checkpoint artifact manifest
//!
//! snapshot.rs
//!     │
//!     └── supported snapshot semantics
//!
//! storage.rs
//!     │
//!     └── persistence contracts
//!
//! integrity.rs
//!     │
//!     └── integrity/authenticity contracts
//!
//! compatibility.rs
//!     │
//!     └── compatibility contracts
//! ```
//!
//! Downstream integration is intentionally one-way:
//!
//! ```text
//! resilience::recovery
//!        │
//!        ▼
//! resilience::checkpoint
//!        │
//!        ├────► state
//!        ├────► serialization
//!        └────► verification
//! ```
//!
//! The checkpoint composition root must not depend on the concrete recovery
//! implementation. This prevents a circular dependency between persistence
//! contracts and recovery orchestration.
//!
//! # Testing
//!
//! Integration tests should exercise this module through the public namespace:
//!
//! ```text
//! quantum::resilience::checkpoint
//! ```
//!
//! Tests must cover:
//!
//! - all child modules are reachable;
//! - canonical types are available;
//! - no duplicate qubit identity exists;
//! - checkpoint construction remains provider-independent;
//! - manifest composition remains dynamically sized;
//! - storage contracts remain backend-neutral;
//! - integrity remains separated from storage;
//! - compatibility remains separated from restoration;
//! - no fixed machine-size assumptions exist.
//!
//! Large-scale tests belong to the checkpoint test suite rather than this
//! composition root.
//!
//! # Example
//!
//! ```rust
//! use crate::quantum::resilience::checkpoint::{
//!     Checkpoint,
//!     CheckpointId,
//! };
//!
//! let _checkpoint_id = CheckpointId::new("example-checkpoint")?;
//! # let _ = core::result::Result::<(), Box<dyn std::error::Error>>::Ok(());
//! ```
//!
//! The concrete constructor requirements of `Checkpoint` remain owned by
//! `checkpoint.rs`; this composition root does not wrap or duplicate them.

// ============================================================================
// Safety policy
// ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// ============================================================================
// Child modules
// ============================================================================

/// Canonical checkpoint semantic model, identity, lifecycle and validation.
///
/// This is the authoritative owner of checkpoint semantics.
pub mod checkpoint;

/// Checkpoint compatibility negotiation.
///
/// This module determines whether a checkpoint can be interpreted/restored
/// against a requested schema, runtime, QEC configuration, or target.
pub mod compatibility;

/// Checkpoint integrity and authenticity contracts.
///
/// Cryptographic implementation remains behind the integrity abstraction.
pub mod integrity;

/// Canonical checkpoint manifest and artifact enumeration.
///
/// A manifest describes artifacts; it does not contain their payload bytes.
pub mod manifest;

/// Provider/runtime snapshot semantics.
///
/// This module describes explicitly supported snapshot forms and does not
/// imply that arbitrary unknown quantum states are serializable.
pub mod snapshot;

/// Provider-neutral physical checkpoint persistence.
///
/// This module abstracts object storage, streaming, conditional operations,
/// versions, namespaces and storage capabilities.
pub mod storage;

// ============================================================================
// Canonical public re-exports
// ============================================================================
//
// Re-export only types whose ownership is unambiguous and whose use is
// expected to be common across checkpoint consumers.
//
// Do not replace these with wildcard exports. Explicit exports make API
// ownership visible and prevent accidental collisions when child modules grow.

// -----------------------------------------------------------------------------
// checkpoint.rs
// -----------------------------------------------------------------------------

pub use self::checkpoint::{
    ArtifactId,
    Checkpoint,
    CheckpointBoundary,
    CheckpointError,
    CheckpointId,
    CheckpointPayloadLocation,
    CheckpointResourceScope,
    CheckpointStateKind,
    CheckpointTimestamp,
    ExecutionId,
    OperationId,
    ProgramId,
    TargetId,
    CHECKPOINT_SCHEMA_ID,
    CHECKPOINT_SCHEMA_VERSION,
};

// -----------------------------------------------------------------------------
// manifest.rs
// -----------------------------------------------------------------------------

pub use self::manifest::{
    CheckpointManifest,
    ManifestArtifact,
    ManifestArtifactRole,
    ManifestError,
    ManifestId,
    ManifestLineage,
    ManifestMetadata,
    ManifestSchemaVersion,
    ManifestState,
    CHECKPOINT_MANIFEST_SCHEMA_ID,
    CHECKPOINT_MANIFEST_SCHEMA_MAJOR,
    CHECKPOINT_MANIFEST_SCHEMA_MINOR,
    CHECKPOINT_MANIFEST_SCHEMA_PATCH,
};

// -----------------------------------------------------------------------------
// snapshot.rs
// -----------------------------------------------------------------------------

pub use self::snapshot::{
    SnapshotError,
};

// -----------------------------------------------------------------------------
// storage.rs
// -----------------------------------------------------------------------------

pub use self::storage::{
    StorageBackendIdentity,
    StorageCapabilities,
    StorageConsistency,
    StorageDurability,
    StorageError,
    StorageLimits,
    StorageNamespace,
    StorageObjectId,
    StorageObjectMetadata,
    StorageObjectReference,
    StorageReadRequest,
    StorageVersion,
    WriteCondition,
    CHECKPOINT_STORAGE_SCHEMA_ID,
    CHECKPOINT_STORAGE_SCHEMA_VERSION,
};

// -----------------------------------------------------------------------------
// Integrity / compatibility exports
// -----------------------------------------------------------------------------
//
// These are intentionally limited to the stable contract surface. If a child
// module exposes many implementation details, consumers should access those
// through the child namespace rather than widening this root indefinitely.
//
// The child modules remain the authoritative owners of their APIs.

// ============================================================================
// Compile-time namespace invariants
// ============================================================================

/// Returns the stable checkpoint schema namespace.
///
/// This small function gives integration code a dependency-free way to obtain
/// the checkpoint contract identity without reaching into implementation
/// details.
///
/// The value is static and does not depend on machine size or provider.
#[must_use]
pub const fn schema_id() -> &'static str {
    CHECKPOINT_SCHEMA_ID
}

/// Returns the current checkpoint semantic schema version.
///
/// The value is owned by `checkpoint.rs`; this function merely exposes the
/// canonical value through the checkpoint namespace.
#[must_use]
pub const fn schema_version() -> u16 {
    CHECKPOINT_SCHEMA_VERSION
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_identity_is_stable_and_non_empty() {
        assert!(!CHECKPOINT_SCHEMA_ID.is_empty());
        assert!(!schema_id().is_empty());
        assert_eq!(schema_id(), CHECKPOINT_SCHEMA_ID);
        assert!(CHECKPOINT_SCHEMA_VERSION > 0);
        assert_eq!(schema_version(), CHECKPOINT_SCHEMA_VERSION);
    }

    #[test]
    fn checkpoint_modules_are_reachable() {
        let _ = checkpoint::CHECKPOINT_SCHEMA_ID;
        let _ = manifest::CHECKPOINT_MANIFEST_SCHEMA_ID;
        let _ = storage::CHECKPOINT_STORAGE_SCHEMA_ID;
    }

    #[test]
    fn canonical_checkpoint_types_are_reexported() {
        let checkpoint_id =
            CheckpointId::new("checkpoint-mod-test")
                .expect("non-empty checkpoint ID must be valid");

        assert_eq!(checkpoint_id.as_str(), "checkpoint-mod-test");
    }

    #[test]
    fn canonical_qubit_identity_is_owned_by_ir() {
        // This test intentionally uses the canonical IR type directly.
        //
        // The checkpoint subsystem must never introduce another QubitId.
        let _qubit: crate::quantum::ir::qubit::QubitId =
            crate::quantum::ir::qubit::QubitId::new(0);
    }

    #[test]
    fn execution_scope_does_not_require_machine_size() {
        let scope = CheckpointResourceScope::execution();

        assert!(matches!(
            scope,
            CheckpointResourceScope::Execution
        ));
    }

    #[test]
    fn storage_limits_can_be_unbounded() {
        let limits = StorageLimits::unbounded();

        assert!(limits.maximum_object_bytes.is_none());
        assert!(limits.maximum_list_page_objects.is_none());
        assert!(limits.maximum_range_bytes.is_none());
    }
}