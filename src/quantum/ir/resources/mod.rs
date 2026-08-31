//! Zamani Quantum IR — Resource Subsystem
//!
//! Production-grade resource namespace and integration boundary for the
//! canonical Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `quantum::ir::resources` is the namespace through which the Quantum IR
//! exposes resource-related semantic contracts.
//
//! It answers:
//!
//! > What resources, capabilities, mappings, and resource-related
//! > requirements are associated with this quantum program?
//!
//! It does NOT:
//!
//! - discover hardware;
//! - allocate hardware;
//! - select a QPU;
//! - perform routing;
//! - perform scheduling;
//! - inspect live calibration;
//! - execute quantum programs;
//! - communicate with providers;
//! - simulate quantum states;
//! - perform quantum error correction;
//! - implement optimization algorithms;
//! - parse Zamani source code.
//!
//! Those responsibilities belong to downstream subsystems.
//!
//! # Important ownership rule
//!
//! At the current migration stage, the canonical implementations are still
//! owned by the established sibling modules:
//
//! ```text
//! quantum::ir::resource
//! quantum::ir::capability
//! quantum::ir::mapping
//! quantum::ir::qubit
//! ```
//!
//! This module MUST NOT redefine their types.
//!
//! In particular, these types remain canonical:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! quantum::ir::resource::ResourceRequirement
//! quantum::ir::capability::CapabilityRequirement
//! quantum::ir::mapping::QubitMapping
//! ```
//!
//! The `resources` namespace provides a stable grouped API around those
//! implementations while the repository is migrated toward the final
//! directory-oriented architecture.
//!
//! This avoids the most dangerous migration error:
//!
//! ```text
//! quantum::ir::resource::ResourceRequirement
//! quantum::ir::resources::ResourceRequirement
//! ```
//!
//! being two different Rust types.
//!
//! There must be exactly one semantic owner for each concept.
//!
//! # Universal-program principle
//!
//! A Zamani quantum program is written once at the semantic level and may be
//! lowered to any compatible target for which the required resources and
//! capabilities exist.
//!
//! Resource descriptions therefore MUST NOT encode a fixed quantum-machine
//! size.
//!
//! The following must all be representable by the same API:
//!
//! ```text
//! 1 qubit
//! 2 qubits
//! 32 qubits
//! 64 qubits
//! 1_000 qubits
//! 1_000_000 qubits
//! N qubits
//! ```
//!
//! No value in this module is an architectural maximum.
//!
//! Practical limits come from:
//!
//! 1. representable identifier space;
//! 2. available memory;
//! 3. explicit IR security/resource policies;
//! 4. compiler limits;
//! 5. runtime limits;
//! 6. target capacity;
//! 7. target capabilities;
//! 8. backend execution constraints.
//!
//! These are execution constraints, not limits on the Zamani language.
//!
//! # Resource / capability / mapping separation
//!
//! These concepts are intentionally different.
//!
//! ```text
//! RESOURCE
//!     What quantity of something is required?
//!
//! CAPABILITY
//!     What semantic ability is required?
//!
//! MAPPING
//!     Which logical identity is associated with which physical identity?
//!
//! HARDWARE
//!     What actually exists on a target?
//!
//! ROUTING
//!     How should a valid mapping be obtained?
//!
//! SCHEDULING
//!     When should operations execute?
//! ```
//!
//! The dependency direction is therefore:
//!
//! ```text
//!                    canonical IR
//!                         │
//!              ┌──────────┼──────────┐
//!              │          │          │
//!              ▼          ▼          ▼
//!          resources  capabilities  mapping
//!              │          │          │
//!              └──────────┼──────────┘
//!                         │
//!                         ▼
//!                    downstream
//!              ┌──────────┼──────────┐
//!              ▼          ▼          ▼
//!           routing   scheduling   hardware
//! ```
//!
//! The resource namespace MUST NOT depend on any of those downstream
//! implementations.
//!
//! # Canonical qubit identity
//!
//! All qubit-related resource APIs use the authoritative types from:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! New code must use:
//!
//! ```rust
//! use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
//! ```
//!
//! and not create another qubit identifier type.
//!
//! This is particularly important for:
//!
//! - resource declarations;
//! - capability scopes;
//! - logical/physical mappings;
//! - validation;
//! - routing;
//! - hardware compatibility;
//! - serialization;
//! - hashing;
//! - provenance.
//!
//! # Current repository migration strategy
//!
//! The repository currently contains a flat IR layout together with several
//! already-created directory namespaces. The resource directory exists but
//! its specialized child files have not yet replaced the canonical sibling
//! implementations.
//!
//! Therefore this module deliberately uses a compatibility façade:
//
//! ```text
//! quantum::ir::resources
//!          │
//!          ├── resource  ─────► quantum::ir::resource
//!          ├── capability ────► quantum::ir::capability
//!          └── mapping ───────► quantum::ir::mapping
//! ```
//!
//! No implementation is copied.
//!
//! No type is wrapped unnecessarily.
//!
//! No type identity is changed.
//!
//! This allows downstream code to migrate namespace usage without requiring
//! simultaneous rewrites of every existing IR consumer.
//!
//! # Future directory architecture
//!
//! The final resource subsystem can grow into:
//!
//! ```text
//! resources/
//! ├── mod.rs
//! ├── resource.rs
//! ├── requirement.rs
//! ├── capability.rs
//! ├── topology.rs
//! ├── locality.rs
//! ├── mapping.rs
//! └── constraint.rs
//! ```
//!
//! When those files become canonical owners, this façade becomes their public
//! namespace coordinator. Until then, it MUST continue to re-export the
//! existing canonical implementations rather than creating duplicate types.
//!
//! # Resource model
//!
//! The current canonical resource model already supports semantic quantities
//! such as:
//!
//! ```text
//! exact(N)
//! at_least(N)
//! between(N, M)
//! unbounded
//! ```
//!
//! `Unbounded` has semantic meaning and must never be represented by a numeric
//! sentinel such as `usize::MAX`.
//!
//! This distinction is essential:
//!
//! ```text
//! usize::MAX
//!     = implementation-level integer maximum
//!
//! Unbounded
//!     = semantic absence of a finite upper bound
//! ```
//!
//! # Capability model
//!
//! Capability requirements are similarly separate from hardware capability
//! declarations.
//!
//! The IR says:
//!
//! ```text
//! requires: mid-circuit measurement
//! requires: dynamic control
//! requires: pulse control
//! ```
//!
//! Hardware later answers:
//!
//! ```text
//! supports: yes/no
//! ```
//!
//! This module MUST NOT import `quantum::hardware`.
//!
//! # Mapping model
//!
//! Mapping is a semantic record, not a routing algorithm.
//!
//! For example:
//!
//! ```text
//! logical q0 -> physical p17
//! logical q1 -> physical p42
//! ```
//!
//! does not mean that `p17` or `p42` actually exists on a target.
//!
//! Hardware compatibility validates physical identities against the selected
//! target.
//!
//! Routing decides how to obtain the mapping.
//!
//! # Determinism
//!
//! Resource APIs exposed through this module must preserve the deterministic
//! semantics of their canonical implementations.
//!
//! In particular, resource serialization, mapping iteration, capability
//! identifier ordering, and canonical hashing must not depend on hash-map
//! iteration order.
//!
//! The existing mapping implementation uses ordered maps for this purpose.
//!
//! # Scalability
//!
//! This module must remain free of:
//!
//! ```text
//! MAX_QUBITS
//! MAX_RESOURCES
//! MAX_CAPABILITIES
//! MAX_MAPPINGS
//! MAX_CHANNELS
//! MAX_DEVICES
//! ```
//!
//! Such limits belong to explicit policy/configuration objects, not semantic
//! namespace modules.
//!
//! Sparse and large identifiers must remain valid.
//!
//! The implementation must not materialize every identifier between zero and
//! the largest identifier merely because a large identifier is referenced.
//!
//! # Error boundary
//!
//! Resource-specific errors remain owned by the canonical implementation that
//! produces them.
//!
//! This module must not introduce a second incompatible resource error system.
//!
//! For example:
//!
//! ```text
//! quantum::ir::resource::ResourceError
//! ```
//!
//! remains the resource-domain error type.
//!
//! A future central IR error layer may translate it into `IrError`, but this
//! namespace must not silently replace or reinterpret it.
//!
//! # Serialization boundary
//!
//! This module does not define serialization.
//!
//! Canonical serialization remains owned by:
//!
//! ```text
//! quantum::ir::serialization
//! ```
//!
//! Resource objects exposed through this module therefore retain exactly the
//! serialization semantics of their canonical implementations.
//!
//! No second resource serialization format is introduced here.
//!
//! # Hashing boundary
//!
//! This module does not implement hashing.
//!
//! Canonical semantic hashing remains owned by:
//!
//! ```text
//! quantum::ir::hash
//! ```
//!
//! The resource namespace therefore cannot accidentally create a second,
//! incompatible identity representation.
//!
//! # Validation boundary
//!
//! This module does not validate a complete quantum program.
//!
//! Whole-program validation belongs to:
//!
//! ```text
//! quantum::ir::validation
//! ```
//!
//! Resource-domain constructors may still reject locally invalid values,
//! because local invariants belong to the owning type.
//!
//! # Hardware boundary
//!
//! This namespace must remain hardware-independent.
//!
//! It must not contain:
//!
//! ```text
//! IBM
//! IonQ
//! Quantinuum
//! Rigetti
//! D-Wave
//! CUDA
//! QPU APIs
//! provider credentials
//! device addresses
//! hardware calibration databases
//! ```
//!
//! Vendor-specific requirements should use namespaced extension capabilities
//! and resource labels where appropriate.
//!
//! # Integration contract
//!
//! Consumers may use either the canonical legacy path:
//!
//! ```rust
//! use crate::quantum::ir::resource::ResourceRequirement;
//! ```
//!
//! or the grouped resource path introduced by this module:
//!
//! ```rust
//! use crate::quantum::ir::resources::ResourceRequirement;
//! ```
//!
//! Both names resolve to the SAME Rust type.
//!
//! Similarly:
//!
//! ```rust
//! use crate::quantum::ir::resources::CapabilityRequirement;
//! use crate::quantum::ir::resources::QubitMapping;
//! ```
//!
//! resolve to the existing canonical implementations.
//!
//! # No duplicate identity types
//!
//! This module deliberately does NOT define:
//!
//! ```text
//! ResourceQubitId
//! ResourcePhysicalQubitId
//! ResourceMapping
//! ResourceCapabilityId
//! ```
//!
//! unless such a type has a genuinely distinct semantic meaning.
//!
//! Existing canonical identity types remain authoritative.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! This module explicitly forbids unsafe code.
//!
//! # Thread safety
//!
//! This module introduces no global mutable state.
//!
//! The re-exported resource abstractions remain ordinary Rust value types.
//!
//! Thread-safety therefore follows the auto-trait properties of the canonical
//! underlying types.
//!
//! No global registry is introduced here.
//!
//! # Integration with other IR modules
//!
//! ```text
//! quantum::ir::qubit
//!     │
//!     ├──────────────► resources::mapping
//!     │
//!     └──────────────► resources::resource
//!
//! quantum::ir::resource
//!     │
//!     └──────────────► resources
//!
//! quantum::ir::capability
//!     │
//!     └──────────────► resources
//!
//! quantum::ir::mapping
//!     │
//!     └──────────────► resources
//!
//! quantum::ir::program
//!     │
//!     └──────────────► resources
//!
//! quantum::ir::operation
//!     │
//!     └──────────────► resources
//!
//! quantum::ir::validation
//!     │
//!     └──────────────► resources
//!
//! quantum::optimization
//!     │
//!     └──────────────► quantum::ir::resources
//!
//! quantum::routing
//!     │
//!     └──────────────► quantum::ir::resources
//!
//! quantum::hardware
//!     │
//!     └──────────────► quantum::ir::resources
//! ```
//!
//! The arrow direction is always consumer -> semantic contract.
//!
//! `resources` never imports the downstream consumers.
//!
//! # Migration guarantee
//!
//! Existing code such as:
//!
//! ```rust
//! use crate::quantum::ir::resource::ResourceRequirement;
//! ```
//!
//! remains valid.
//!
//! New code may use:
//!
//! ```rust
//! use crate::quantum::ir::resources::ResourceRequirement;
//! ```
//!
//! Because the latter is a re-export of the former, values can cross the
//! namespace boundary without conversion.
//!
//! # Why this file contains no algorithms
//!
//! `mod.rs` is an integration boundary.
//!
//! It owns:
//!
//! 1. namespace organization;
//! 2. canonical re-exports;
//! 3. migration aliases;
//! 4. public API grouping;
//! 5. documentation of ownership boundaries.
//!
//! It does not own:
//!
//! 1. resource arithmetic;
//! 2. capability matching;
//! 3. routing;
//! 4. scheduling;
//! 5. allocation;
//! 6. hardware discovery;
//! 7. optimization;
//! 8. serialization algorithms;
//! 9. hashing algorithms.
//!
//! Keeping this file thin makes it stable while the underlying resource
//! subsystem evolves.
//!
//! # Completion contract
//!
//! This file is considered complete when:
//!
//! - no resource type is duplicated;
//! - canonical `qubit` identities are preserved;
//! - resource requirements remain hardware-independent;
//! - capability requirements remain hardware-independent;
//! - mappings remain routing-independent;
//! - no fixed machine-size constant exists;
//! - legacy paths remain source-compatible;
//! - grouped paths resolve to the same canonical types;
//! - no downstream dependency is introduced;
//! - no unsafe code exists;
//! - Rust 1.97.1 compiles it;
//! - documentation accurately describes ownership;
//! - downstream resource consumers can migrate without changing semantics.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------
//
// This module is intentionally a façade over the currently canonical sibling
// implementations.
//
// IMPORTANT:
// `resource.rs`, `capability.rs`, and `mapping.rs` remain the single semantic
// owners until their eventual directory-specific implementations are created.
//
// Do NOT use `#[path = "../resource.rs"]` here.
// That would compile another copy of the module and create distinct Rust types.
//
// Do NOT copy the definitions here.
// That would violate the one-owner rule.
//
// Instead, re-export the canonical modules directly.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Canonical implementation namespaces
// =============================================================================

/// Canonical resource requirements.
///
/// This is currently backed by [`crate::quantum::ir::resource`].
///
/// The underlying implementation remains the single owner of all resource
/// requirement types and invariants.
pub mod resource {
    pub use super::super::resource::*;
}

/// Canonical capability requirements.
///
/// This is currently backed by [`crate::quantum::ir::capability`].
///
/// Capability semantics remain independent of hardware declarations.
pub mod capability {
    pub use super::super::capability::*;
}

/// Canonical logical-to-physical mapping.
///
/// This is currently backed by [`crate::quantum::ir::mapping`].
///
/// Mapping is a representation of an association, not a routing algorithm.
pub mod mapping {
    pub use super::super::mapping::*;
}

// =============================================================================
// Canonical resource API
// =============================================================================

pub use super::resource::{
    LogicalQubitResources,
    PhysicalQubitResources,
    QubitResourceBinding,
    QuantumResourceRequirements,
    ResourceCapacity,
    ResourceError,
    ResourceKind,
    ResourceQuantity,
    ResourceRange,
    ResourceRequirement,
};

// =============================================================================
// Canonical capability API
// =============================================================================

pub use super::capability::{
    CapabilityCheckReport,
    CapabilityId,
    CapabilityKind,
    CapabilityRequirement,
    CapabilityRequirementSet,
    QuantumCapability,
};

// =============================================================================
// Canonical mapping API
// =============================================================================

pub use super::mapping::{
    MappingDomain,
    MappingEntry,
    MappingError,
    QubitMapping,
    QubitMappingBuilder,
    QubitMappingView,
};

// =============================================================================
// Canonical qubit identity API
// =============================================================================
//
// These are deliberately re-exported from `quantum::ir::qubit`.
//
// `resources` does not define another QubitId.

pub use super::qubit::{
    PhysicalQubitId,
    QubitId,
};

// =============================================================================
// Compile-time API identity tests
// =============================================================================
//
// These tests are deliberately small. Their purpose is to prove that the
// grouped resource namespace does not accidentally create duplicate Rust
// types.
//
// They can be expanded in the dedicated cross-module IR test suite later.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_requirement_is_the_canonical_resource_type() {
        let grouped: ResourceRequirement =
            ResourceRequirement::exact(ResourceKind::LogicalQubits, 1);

        let canonical: crate::quantum::ir::resource::ResourceRequirement = grouped;

        assert_eq!(
            canonical.kind(),
            crate::quantum::ir::resource::ResourceKind::LogicalQubits
        );
        assert!(canonical.accepts(1));
    }

    #[test]
    fn capability_requirement_is_the_canonical_capability_type() {
        let grouped: CapabilityRequirement =
            CapabilityRequirement::new(QuantumCapability::Measurement);

        let _: crate::quantum::ir::capability::CapabilityRequirement = grouped;
    }

    #[test]
    fn mapping_is_the_canonical_mapping_type() {
        let grouped = QubitMapping::new();

        let _: crate::quantum::ir::mapping::QubitMapping = grouped;
    }

    #[test]
    fn qubit_ids_are_owned_by_the_canonical_qubit_module() {
        fn accepts_canonical_qubit(_: crate::quantum::ir::qubit::QubitId) {}

        let logical = QubitId::new(0);

        accepts_canonical_qubit(logical);
    }

    #[test]
    fn physical_qubit_ids_are_owned_by_the_canonical_qubit_module() {
        fn accepts_canonical_physical_qubit(
            _: crate::quantum::ir::qubit::PhysicalQubitId,
        ) {}

        let physical = PhysicalQubitId::new(0);

        accepts_canonical_physical_qubit(physical);
    }

    #[test]
    fn grouped_and_canonical_resource_modules_are_the_same_namespace_contract() {
        let grouped =
            resource::ResourceRequirement::at_least(ResourceKind::LogicalQubits, 1);

        let canonical =
            super::super::resource::ResourceRequirement::at_least(
                ResourceKind::LogicalQubits,
                1,
            );

        assert_eq!(grouped, canonical);
    }

    #[test]
    fn grouped_and_canonical_mapping_modules_are_the_same_namespace_contract() {
        let grouped = mapping::QubitMapping::new();
        let canonical = super::super::mapping::QubitMapping::new();

        assert_eq!(grouped, canonical);
    }

    #[test]
    fn resource_namespace_contains_no_fixed_machine_size() {
        // The test intentionally uses a large sparse semantic identifier
        // rather than allocating a correspondingly large resource array.
        //
        // This proves that the namespace itself does not require a
        // machine-sized contiguous representation.
        let qubit = QubitId::new(u64::MAX);

        assert_eq!(qubit.index(), u64::MAX);
    }
}