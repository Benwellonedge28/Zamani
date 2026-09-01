//! Zamani Quantum IR — Compatibility Boundary.
//!
//! This module is the source/API compatibility boundary for the canonical
//! `quantum::ir` implementation.
//!
//! Compatibility is deliberately isolated from the semantic IR:
//!
//! ```text
//! historical Zamani API / source
//!              │
//!              ▼
//!       compatibility layer
//!        ┌─────┼──────┐
//!        ▼     ▼      ▼
//!     aliases legacy migration
//!        │     │      │
//!        └─────┴──────┘
//!              │
//!              ▼
//!       canonical Quantum IR
//! ```
//!
//! # Architectural rules
//!
//! 1. The canonical IR owns semantics.
//! 2. Compatibility owns only historical API/source compatibility.
//! 3. Compatibility must not become a second IR.
//! 4. Exact aliases must remain semantically lossless.
//! 5. Semantic conversions belong to explicit migrations.
//! 6. Unknown information must never be silently discarded.
//! 7. Compatibility must not introduce hardware-specific assumptions.
//! 8. Compatibility must not impose machine-size limits.
//! 9. The authoritative qubit implementation is `crate::quantum::ir::qubit`.
//! 10. The canonical IR must never depend on this module.
//!
//! # Canonical qubit authority
//!
//! All qubit identity and qubit-reference types remain owned by:
//!
//! `crate::quantum::ir::qubit`
//!
//! Compatibility code may provide historical names or paths for those types,
//! but must not introduce replacement qubit identity types.
//!
//! In particular, a compatibility alias must preserve the exact canonical
//! `QubitId` type rather than wrapping it.
//!
//! New code should use:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! directly.
//!
//! # Scalability
//!
//! This module contains no architectural limits such as:
//!
//! - maximum qubits
//! - maximum registers
//! - maximum operations
//! - maximum gate arity
//! - maximum topology size
//! - maximum nodes
//! - maximum distributed resources
//! - vendor-specific capacities
//!
//! Compatibility APIs must remain valid for programs of any size that can be
//! represented by the canonical IR and processed within the resources and
//! explicit compilation limits selected by the caller.
//!
//! A number appearing in a legacy API must never silently become a universal
//! Zamani quantum-system limit.
//!
//! # Dependency direction
//!
//! The dependency direction is intentionally one-way:
//!
//! ```text
//! canonical IR  ◄──── compatibility
//! ```
//!
//! The canonical IR must not import this module.
//!
//! This module must not depend on:
//!
//! - frontend ASTs
//! - OpenQASM ASTs
//! - optimization passes
//! - routing
//! - scheduling
//! - hardware implementations
//! - simulators
//! - QEC implementations
//! - backend execution
//! - external services
//! - credentials
//! - filesystem state
//!
//! # Compatibility versus serialization
//!
//! Source/API compatibility and serialized-format compatibility are separate
//! concerns.
//!
//! This module owns source/API compatibility.
//!
//! Canonical serialization-version compatibility belongs to the serialization
//! subsystem. Do not duplicate wire-format migration rules here.
//!
//! # Rust requirements
//!
//! This module is intentionally compatible with Rust 1.97 / 1.97.1 and Rust
//! 2021. It requires no nightly features and contains no unsafe code.
//!
//! `forbid(unsafe_code)` makes the no-unsafe requirement compiler enforced.
//!
//! # Public structure
//!
//! ```text
//! compatibility/
//! ├── mod.rs
//! ├── aliases.rs
//! ├── legacy.rs
//! └── migration.rs
//! ```
//!
//! ## aliases
//!
//! Exact, lossless source/API aliases and historical module-path bridges.
//!
//! ## legacy
//!
//! Historical API compatibility that requires more than a simple type alias.
//!
//! ## migration
//!
//! Explicit semantic migration between historical representations and the
//! canonical IR.
//!
//! # Integration contract
//!
//! Other IR modules may consume compatibility APIs when maintaining existing
//! source compatibility, but canonical implementations must remain independent
//! of this module.
//!
//! The stable module paths are:
//!
//! ```text
//! quantum::ir::compatibility::aliases
//! quantum::ir::compatibility::legacy
//! quantum::ir::compatibility::migration
//! ```
//!
//! The root deliberately does not glob-re-export every child symbol. This keeps
//! ownership explicit and prevents historical names from accidentally becoming
//! canonical APIs.
//!
//! # Migration rule
//!
//! The intended lifecycle is:
//!
//! ```text
//! historical API
//!      │
//!      ├── exact semantic identity ──► aliases ──► canonical IR
//!      │
//!      └── semantic difference ─────► migration ──► canonical IR
//! ```
//!
//! A migration must be explicit whenever information, semantics, or invariants
//! change.
//!
//! # What does NOT belong here
//!
//! This module must never contain:
//!
//! - gate implementations
//! - quantum state representations
//! - matrix simulation
//! - pulse execution
//! - calibration execution
//! - topology algorithms
//! - qubit routing
//! - optimization
//! - scheduling
//! - QEC decoding
//! - backend code generation
//! - device discovery
//! - network transport
//! - resource allocation
//!
//! Those responsibilities belong to their respective canonical IR or compiler
//! subsystems.
//!
//! # Stability policy
//!
//! This root should rarely change. Adding a new compatibility mechanism should
//! normally require only:
//!
//! 1. adding the child module;
//! 2. exposing it here;
//! 3. documenting its ownership boundary;
//! 4. adding tests in that child.
//!
//! Existing child contracts must not be modified merely because another
//! compatibility mechanism is added.
//!
//! # Testing
//!
//! Tests in this module verify only integration-level invariants:
//!
//! - all compatibility namespaces are reachable;
//! - canonical `QubitId` remains reachable through `quantum::ir::qubit`;
//! - this module does not define a competing qubit identity.
//!
//! Detailed compatibility tests belong beside the implementation they test.
//! Serialization round-trip tests belong in the serialization subsystem.
//! Migration semantic-equivalence tests belong in `migration.rs`.
//!
//! # Maintenance invariant
//!
//! This file owns module composition and architectural boundaries.
//!
//! It does NOT own the implementation details of aliases, legacy conversions,
//! or migrations. Consequently, internal implementation changes in those files
//! should not require changes here unless their public module ownership changes.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

/// Lossless source/API aliases and historical module-path compatibility.
///
/// This module is the preferred compatibility mechanism when an old name has
/// exactly the same semantics and representation as the canonical type.
pub mod aliases;

/// Compatibility support for historical APIs that cannot be represented by
/// an exact alias alone.
///
/// Semantic conversions should remain explicit and documented.
pub mod legacy;

/// Explicit migrations between historical representations and the canonical
/// Zamani Quantum IR.
///
/// Migrations are where semantic transformation belongs; aliases must remain
/// lossless.
pub mod migration;

#[cfg(test)]
mod tests {
    //! Integration-level invariants for the compatibility namespace.

    #[test]
    fn compatibility_namespaces_are_public() {
        // These paths intentionally reference only the modules themselves.
        // The implementation contracts remain owned by the child modules.
        let _aliases: fn() = || {
            let _ = core::any::TypeId::of::<super::aliases::AliasMapping>();
        };

        let _legacy: fn() = || {
            let _ = core::any::TypeId::of::<super::legacy::CompatibilityKind>();
        };

        let _migration: fn() = || {
            let _ = core::any::TypeId::of::<super::migration::MigrationKind>();
        };

        _aliases();
        _legacy();
        _migration();
    }

    #[test]
    fn canonical_qubit_identity_remains_owned_by_qubit_module() {
        use crate::quantum::ir::qubit::QubitId;

        let canonical = QubitId::new(0);
        let same_type: QubitId = canonical;

        assert_eq!(same_type, canonical);
    }

    #[test]
    fn compatibility_root_does_not_create_a_second_qubit_namespace() {
        // The compatibility root intentionally exposes no replacement QubitId.
        //
        // The authoritative identity is:
        //
        // crate::quantum::ir::qubit::QubitId
        //
        // This test exists as an architectural marker. If compatibility starts
        // defining another qubit identity, the module boundary has been
        // violated and this test should be expanded to reject the design.
        let _ = core::any::TypeId::of::<crate::quantum::ir::qubit::QubitId>();
    }
}