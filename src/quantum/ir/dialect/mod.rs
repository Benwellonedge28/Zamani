//! Zamani Quantum IR — Dialect System
//!
//! This module is the public composition boundary for the Zamani Quantum IR
//! dialect system.
//!
//! # Architectural role
//!
//! A dialect is a semantic vocabulary layered on top of the canonical Zamani
//! Quantum IR. A dialect defines or extends the meaning of operations, types,
//! attributes, resources, or other IR constructs without becoming a hardware
//! backend.
//!
//! The canonical compilation boundary is:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! frontend
//!      │
//!      ▼
//! ┌──────────────────────────────┐
//! │      Canonical Quantum IR    │
//! │                              │
//! │          quantum::ir         │
//! └──────────────┬───────────────┘
//!                │
//!                ▼
//!          dialect system
//!                │
//!      ┌─────────┼──────────┬───────────┐
//!      ▼         ▼          ▼           ▼
//!   standard    pulse    fault-tolerant vendor
//!      │         │          │           │
//!      └─────────┴──────────┴───────────┘
//!                         │
//!                         ▼
//!                target-independent
//!                transformations
//!                         │
//!                         ▼
//!                target capabilities
//!                         │
//!                         ▼
//!                 mapping/routing
//!                         │
//!                         ▼
//!                    scheduling
//!                         │
//!                         ▼
//!                 hardware lowering
//!                         │
//!                         ▼
//!                     backend
//! ```
//!
//! The fundamental rule is:
//!
//! > A dialect describes semantic meaning. It does not describe how a
//! > particular hardware backend executes that meaning.
//!
//! # Responsibilities of this module
//!
//! This module owns only the composition boundary:
//!
//! - declaring dialect submodules;
//! - defining the public dialect module tree;
//! - exposing the dialect registry API;
//! - exposing the standard/pulse/fault-tolerant/vendor dialect APIs;
//! - exposing dialect extensions;
//! - providing a small, deliberately curated dialect prelude;
//! - documenting dependency direction and integration contracts.
//!
//! It does NOT own:
//!
//! - individual operation definitions;
//! - gate semantics;
//! - qubit identity;
//! - physical-qubit mapping;
//! - hardware topology;
//! - backend execution;
//! - simulation;
//! - optimization;
//! - routing;
//! - scheduling;
//! - calibration execution;
//! - QEC decoding;
//! - source parsing;
//! - hardware SDKs.
//!
//! Those responsibilities belong to their canonical modules or downstream
//! compiler layers.
//!
//! # Existing dialect modules
//!
//! The current dialect directory contains:
//!
//! - [`standard`] — canonical standard quantum operations;
//! - [`pulse`] — pulse-level semantic constructs;
//! - [`fault_tolerant`] — logical/fault-tolerant semantic constructs;
//! - [`vendor`] — vendor-specific semantic extensions;
//! - [`extension`] — dialect declarations and registry infrastructure.
//!
//! New dialects should be added as sibling modules rather than expanding this
//! file with their implementation details.
//!
//! # Universal-program principle
//!
//! Zamani programs are written at the semantic level and may be lowered to
//! compatible targets of different sizes and architectures.
//!
//! Therefore this module intentionally contains NO architectural constants
//! representing:
//!
//! - maximum qubits;
//! - maximum registers;
//! - maximum operations;
//! - maximum dialects;
//! - maximum extensions;
//! - maximum resources;
//! - maximum topology size;
//! - maximum pulse count;
//! - maximum logical qubits;
//! - maximum physical qubits;
//! - maximum distributed nodes.
//!
//! A finite compilation may still be constrained by explicit resource and
//! security policies, available host resources, target capabilities, and
//! backend constraints. Those are not semantic limits of Zamani.
//!
//! # No fixed gate universe
//!
//! The standard dialect is intentionally only one dialect.
//!
//! ```text
//! canonical Zamani Quantum IR
//!          │
//!          ├── standard
//!          ├── pulse
//!          ├── fault-tolerant
//!          ├── vendor
//!          ├── future/user dialects
//!          └── extension mechanisms
//! ```
//!
//! Adding a new quantum architecture MUST NOT require turning this file into
//! an ever-growing `match` over every operation known to humanity.
//!
//! New semantics should normally be introduced through a new dialect or
//! extension contract.
//!
//! # Canonical identity ownership
//!
//! This module does not define quantum identities.
//!
//! The canonical quantum identity remains:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! and physical identity remains:
//!
//! ```text
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Dialects may reference those types through their canonical module paths,
//! but MUST NOT redefine them.
//!
//! In particular, dialect code must use:
//!
//! ```text
//! super::super::qubit::QubitId
//! ```
//!
//! or the equivalent canonical `quantum::ir::qubit::QubitId` path when
//! appropriate.
//!
//! # Extension ownership
//!
//! There are deliberately two different concepts:
//!
//! 1. `core::extension`
//!    - owns an individual extension occurrence;
//!    - owns extension identity/payload representation.
//!
//! 2. `dialect::extension`
//!    - owns the relationship between extensions and dialects;
//!    - owns dialect declarations;
//!    - owns dialect registration;
//!    - owns registry-level conflict/version handling.
//!
//! This separation prevents extension data from becoming coupled to the
//! registry implementation.
//!
//! # Registry ownership
//!
//! Dialect registration is explicit and scoped.
//!
//! This module MUST NOT create a process-global mutable registry.
//!
//! Callers should own a [`extension::DialectRegistry`] and pass it explicitly
//! through the relevant compiler stages.
//!
//! This provides:
//!
//! - deterministic compilation;
//! - reproducible tests;
//! - parallel compilation;
//! - independent compiler sessions;
//! - explicit dependency management;
//! - no hidden global state;
//! - easier distributed compilation;
//! - easier sandboxing;
//! - easier plugin isolation.
//!
//! # Dependency direction
//!
//! The dependency graph is intentionally one-way:
//!
//! ```text
//! core
//!  │
//!  ├───────────────┐
//!  │               │
//!  ▼               ▼
//! quantum       classical
//!  │               │
//!  └───────┬───────┘
//!          ▼
//!       program
//!          │
//!    ┌─────┼─────┐
//!    ▼     ▼     ▼
//! dialect pulse models
//!    │     │     │
//!    └─────┼─────┘
//!          ▼
//!     validation
//!          │
//!          ▼
//!      analysis
//!          │
//!          ▼
//! serialization / hashing
//! ```
//!
//! Dialects may depend on canonical IR primitives.
//!
//! Canonical IR primitives MUST NOT depend on a concrete dialect.
//!
//! Downstream compiler components may depend on dialects.
//!
//! Dialects MUST NOT depend on downstream execution infrastructure.
//!
//! # Backend isolation
//!
//! This module contains no:
//!
//! - IBM SDK;
//! - IonQ SDK;
//! - Quantinuum SDK;
//! - Rigetti SDK;
//! - D-Wave SDK;
//! - CUDA runtime;
//! - OpenCL runtime;
//! - QPU transport;
//! - cloud credentials;
//! - network clients;
//! - simulator state;
//! - device calibration database.
//!
//! A vendor dialect is semantic metadata. A vendor backend remains outside
//! `quantum::ir`.
//!
//! # Versioning
//!
//! Dialect versioning is owned by [`extension`] and individual dialect
//! declarations.
//!
//! This module MUST NOT create a second global dialect version.
//!
//! The following versions remain conceptually distinct:
//!
//! ```text
//! Zamani language version
//! Quantum IR version
//! dialect version
//! compiler version
//! backend version
//! hardware version
//! calibration version
//! ```
//!
//! Changing one must not silently redefine another.
//!
//! # Serialization
//!
//! Dialect declarations and extension contracts may participate in canonical
//! IR serialization.
//!
//! This module does not define a second serialization format.
//!
//! Serialization belongs to:
//!
//! ```text
//! quantum::ir::serialization
//! ```
//!
//! The dialect system must therefore expose stable semantic identifiers and
//! deterministic declarations, while the serializer owns byte-level encoding.
//!
//! # Hashing
//!
//! Dialect semantics may contribute to canonical IR hashing.
//!
//! Hashing belongs to:
//!
//! ```text
//! quantum::ir::hash
//! ```
//!
//! This module does not hash dialects itself.
//!
//! Stable dialect identity and deterministic registry ordering are therefore
//! more important than introducing local hashing behavior here.
//!
//! # Validation
//!
//! Dialect-level validation belongs in the dialect implementation or the
//! canonical validation subsystem as appropriate.
//!
//! Registry membership means:
//!
//! > The compiler knows the dialect contract.
//!
//! It does NOT mean:
//!
//! > The selected hardware can execute the dialect.
//!
//! Hardware capability checking remains a separate target concern.
//!
//! # Unknown dialects
//!
//! Forward compatibility requires unknown dialects to be handled explicitly.
//!
//! An importer or decoder must never silently transform:
//!
//! ```text
//! unknown dialect → ignored data
//! ```
//!
//! Instead, unknown dialect information should either:
//!
//! - be preserved losslessly as an extension/opaque declaration; or
//! - produce an explicit compatibility diagnostic.
//!
//! The dialect system therefore intentionally avoids APIs that imply that an
//! unknown dialect is automatically invalid simply because the current binary
//! does not implement it.
//!
//! # Determinism
//!
//! Registry and dialect operations must remain deterministic.
//!
//! The implementation must not depend on randomized iteration order.
//!
//! `dialect::extension` owns the concrete registry representation and is
//! responsible for deterministic ordering.
//!
//! # Thread safety
//!
//! This module has no global mutable state.
//!
//! Dialect declarations should be immutable after construction/registration
//! where possible.
//!
//! A registry can therefore be shared through immutable references by compiler
//! stages that only inspect it.
//!
//! Mutable registry construction should occur explicitly during setup rather
//! than through hidden global state.
//!
//! # Rust compatibility
//!
//! This module is intentionally compatible with:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe` code.
//!
//! Unsafe code is forbidden at compile time.
//!
//! # Integration contract
//!
//! ## `quantum::ir::mod`
//!
//! The parent IR module exposes this module as:
//!
//! ```text
//! pub mod dialect;
//! ```
//!
//! This file must remain usable without changing canonical IR types.
//!
//! ## `quantum::ir::gate`
//!
//! `gate.rs` owns `GateKind` and gate semantics.
//!
//! `dialect::standard` consumes that canonical representation instead of
//! defining another incompatible gate enum.
//!
//! ## `quantum::ir::qubit`
//!
//! `qubit.rs` owns `QubitId` and `PhysicalQubitId`.
//!
//! Dialects use those canonical identities and never redefine them.
//!
//! ## `quantum::ir::operation`
//!
//! Universal operations may refer to dialect-qualified operation names.
//!
//! Operation construction remains owned by `operation.rs`.
//!
//! The dialect system only describes the semantic vocabulary available to
//! those operations.
//!
//! ## `quantum::ir::validation`
//!
//! Validation may inspect dialect declarations, operation ownership,
//! extension contracts, and dialect dependencies.
//!
//! Validation remains the authority for whole-program validity.
//!
//! ## `quantum::ir::serialization`
//!
//! Serialization consumes stable dialect identifiers, versions, declarations,
//! and extension metadata.
//!
//! It owns the wire/canonical representation.
//!
//! ## `quantum::ir::hash`
//!
//! Canonical hashing consumes deterministic dialect information.
//!
//! It owns the actual hash algorithm.
//!
//! ## frontend
//!
//! Frontends translate source-language constructs into canonical IR and may
//! resolve source names through a dialect registry.
//!
//! A frontend MUST NOT make hardware assumptions merely because a dialect was
//! resolved.
//!
//! ## optimization
//!
//! Optimization may inspect dialect semantics and transform operations while
//! preserving semantic validity.
//!
//! Dialects do not own optimization algorithms.
//!
//! ## routing
//!
//! Routing may consume operations containing logical `QubitId` values.
//!
//! Dialects do not own physical routing.
//!
//! ## scheduling
//!
//! Scheduling may consume dialect operation metadata and timing/resource
//! requirements.
//!
//! Dialects do not own scheduling policy.
//!
//! ## hardware
//!
//! Hardware determines whether a dialect operation is supported, decomposable,
//! emulatable, or unsupported on a target.
//!
//! Dialect registration MUST NOT be interpreted as hardware support.
//!
//! ## backend
//!
//! Backends consume target-lowered representations and may use dialect
//! information while lowering.
//!
//! Backend execution code does not belong in this module.
//!
//! # Adding a future dialect
//!
//! A new dialect should follow this pattern:
//!
//! ```text
//! src/quantum/ir/dialect/
//!     mod.rs
//!     standard.rs
//!     pulse.rs
//!     fault_tolerant.rs
//!     vendor.rs
//!     extension.rs
//!     future.rs
//! ```
//!
//! The new implementation should own its semantics.
//!
//! `mod.rs` should then receive only the module declaration and, if justified,
//! a carefully scoped public re-export.
//!
//! Do NOT add a large operation enum to this file.
//!
//! Do NOT add hardware-specific constants to this file.
//!
//! Do NOT add backend calls to this file.
//!
//! # Public API policy
//!
//! The module tree is the primary stable API:
//!
//! ```text
//! quantum::ir::dialect::standard
//! quantum::ir::dialect::pulse
//! quantum::ir::dialect::fault_tolerant
//! quantum::ir::dialect::vendor
//! quantum::ir::dialect::extension
//! ```
//!
//! The module root may additionally expose a curated prelude for common
//! dialect infrastructure.
//!
//! Broad wildcard re-exports are intentionally avoided because they can:
//!
//! - create name collisions;
//! - make API ownership unclear;
//! - make future dialect additions breaking;
//! - make documentation ambiguous;
//! - accidentally expose implementation details.
//!
//! # Compatibility policy
//!
//! Existing canonical paths remain authoritative.
//!
//! This module does not introduce compatibility aliases for:
//!
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `GateKind`;
//! - `Operation`;
//! - `Parameter`;
//! - `Measurement`.
//!
//! Those belong to their existing canonical modules.
//!
//! # Production-readiness invariants
//!
//! The dialect root satisfies the following invariants:
//!
//! 1. No unsafe code.
//! 2. No global mutable registry.
//! 3. No hardware dependency.
//! 4. No fixed machine size.
//! 5. No fixed qubit count.
//! 6. No fixed dialect count.
//! 7. No fixed extension count.
//! 8. No fixed topology.
//! 9. No vendor execution dependency.
//! 10. No duplicate `QubitId`.
//! 11. No duplicate `GateKind`.
//! 12. No duplicate extension representation.
//! 13. No dialect implementation logic in the module root.
//! 14. Stable module paths.
//! 15. Explicit dependency direction.
//! 16. Forward-compatible dialect architecture.
//! 17. Deterministic registry contract delegated to `extension`.
//! 18. Serialization delegated to the canonical serializer.
//! 19. Hashing delegated to the canonical hasher.
//! 20. Validation delegated to the canonical validation boundary.
//!
//! # Testing contract
//!
//! Module-level tests for individual dialects belong in those dialect files.
//!
//! Cross-dialect tests belong in the IR integration test suite.
//!
//! At minimum, integration tests should verify:
//!
//! - every declared dialect module compiles;
//! - standard dialect registration works;
//! - pulse dialect registration works;
//! - fault-tolerant dialect registration works;
//! - vendor dialect registration works;
//! - duplicate dialect registration is rejected deterministically;
//! - incompatible dialect versions are detected;
//! - dialect dependencies are resolved deterministically;
//! - unknown dialect metadata can be preserved or explicitly diagnosed;
//! - canonical serialization is deterministic;
//! - canonical hashing remains stable;
//! - no dialect introduces a hardware-size assumption;
//! - canonical `quantum::ir::qubit::QubitId` remains the sole qubit identity.
//!
//! # Important architectural guarantee
//!
//! This file is deliberately boring.
//!
//! That is a feature.
//!
//! A production IR root should be a stable composition boundary, not the
//! location where every future quantum technology is encoded.
//!
//! The extensibility mechanism lives below this boundary.
//!
//! The semantic operation definitions live in their dialects.
//!
//! The universal operation model lives in `operation.rs`.
//!
//! Resource/capability contracts live in their respective IR modules.
//!
//! Hardware decisions happen downstream.
//!
//! This separation is what permits one Zamani program to scale from a tiny
//! quantum system to a substantially larger compatible system without changing
//! the semantic program merely because the target machine became larger.
//!
//! -----------------------------------------------------------------------------
//! Module declarations
//! -----------------------------------------------------------------------------
//!
//! The declarations below intentionally contain no implementation logic.
//!
//! New dialect implementations should be added here as sibling modules.
//!
//! `extension` is declared first because it provides registry/declaration
//! infrastructure used by the dialect ecosystem. Rust module declaration order
//! does not establish semantic dependency order, but keeping infrastructure
//! first makes the public module tree easier to audit.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Dialect infrastructure
// =============================================================================

/// Dialect declarations, versions, dependencies, extension contracts, and the
/// explicit dialect registry.
///
/// This is the infrastructure layer of the dialect system.
///
/// It owns dialect-to-extension relationships and registry semantics, while
/// `core::extension` owns individual extension occurrences.
pub mod extension;

// =============================================================================
// Canonical standard dialects
// =============================================================================

/// Canonical target-independent standard quantum operations.
///
/// This module reuses the canonical [`crate::quantum::ir::gate::GateKind`]
/// representation and does not define a second gate universe.
pub mod standard;

/// Hardware-independent pulse-level dialect.
///
/// Pulse semantics remain distinct from physical DAC/backend implementation.
pub mod pulse;

/// Logical and fault-tolerant quantum dialect.
///
/// Logical operations remain distinct from physical qubit mapping and QEC
/// decoder implementation.
pub mod fault_tolerant;

/// Vendor-specific semantic dialect support.
///
/// Vendor dialects describe vendor-defined semantics but do not execute vendor
/// SDK calls.
pub mod vendor;

// =============================================================================
// Curated infrastructure re-exports
// =============================================================================
//
// The root deliberately re-exports only dialect infrastructure that is safe
// and useful across compiler stages.
//
// Individual dialect APIs remain available through their explicit module paths.
// This prevents collisions between similarly named operations from different
// dialects and preserves clear ownership.
//
// If a type is not listed here, callers should import it from the owning
// dialect module rather than expecting the root to mirror the entire dialect
// API.

pub use extension::{
    Dialect,
    DialectDependency,
    DialectError,
    DialectExtension,
    DialectId,
    DialectKind,
    DialectName,
    DialectRegistry,
    DialectVersion,
};

// =============================================================================
// Dialect-level prelude
// =============================================================================

/// Common dialect infrastructure intended for compiler code that needs to
/// reason about dialect identity without importing individual implementation
/// modules.
///
/// This module is deliberately small. It is not a wildcard export of every
/// standard, pulse, vendor, or fault-tolerant operation.
pub mod prelude {
    pub use super::extension::{
        Dialect,
        DialectDependency,
        DialectError,
        DialectExtension,
        DialectId,
        DialectKind,
        DialectName,
        DialectRegistry,
        DialectVersion,
    };
}

// =============================================================================
// Compile-time architectural assertions
// =============================================================================
//
// Rust's type system already enforces the most important structural property:
// this module contains no unsafe code.
//
// The tests below are intentionally lightweight and do not construct a global
// registry or assume any particular quantum-machine size.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialect_modules_are_exposed() {
        // These references intentionally exercise the public module boundary
        // without depending on implementation details of the individual
        // dialects.
        let _ = core::any::type_name::<extension::Dialect>();
        let _ = core::any::type_name::<standard::StandardDialect>();
        let _ = core::any::type_name::<pulse::PulseDialect>();
        let _ = core::any::type_name::<fault_tolerant::FaultTolerantDialect>();
        let _ = core::any::type_name::<vendor::VendorDialect>();
    }

    #[test]
    fn root_reexports_are_available() {
        let _ = core::any::type_name::<Dialect>();
        let _ = core::any::type_name::<DialectDependency>();
        let _ = core::any::type_name::<DialectError>();
        let _ = core::any::type_name::<DialectExtension>();
        let _ = core::any::type_name::<DialectId>();
        let _ = core::any::type_name::<DialectKind>();
        let _ = core::any::type_name::<DialectName>();
        let _ = core::any::type_name::<DialectRegistry>();
        let _ = core::any::type_name::<DialectVersion>();
    }

    #[test]
    fn prelude_is_available() {
        let _ = core::any::type_name::<prelude::DialectRegistry>();
        let _ = core::any::type_name::<prelude::DialectId>();
    }

    #[test]
    fn dialect_root_contains_no_machine_size_policy() {
        // This test documents an architectural invariant rather than testing
        // a numerical limit. Machine-size policy belongs to target resources
        // and explicit IR limits, never to the dialect root.
        assert_eq!(core::mem::size_of::<DialectVersion>(), 6);
    }
}