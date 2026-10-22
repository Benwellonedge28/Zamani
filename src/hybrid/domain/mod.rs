
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Domain Subsystem
//!
//! Path: `src/hybrid/domain/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Compose the public module boundary for hybrid computational domains.
//!
//! This module connects the existing domain identity, interface contract,
//! registry, and composition modules. It contains no domain-specific
//! implementation logic.
//!
//! A computational domain describes a semantic class of computation.
//! It does not identify a physical device, compiler backend, runtime
//! session, hardware allocation, or execution location.
//!
//! ## Canonical ownership
//!
//! The child modules retain exclusive ownership of their respective APIs:
//!
//! - `identity`: validated, extensible semantic domain identities.
//! - `interface`: domain participation contracts and interface metadata.
//! - `registry`: registration and lookup of domain contracts.
//! - `composition`: structural composition of domain instances.
//!
//! Related concepts remain owned by their existing modules:
//!
//! - `crate::hybrid::contract`: canonical domain contracts, interface
//!   references, dependencies, inputs, outputs, and guarantees.
//! - `crate::hybrid::version`: contract and format versions.
//! - `crate::hybrid::ids`: graph-scoped hybrid and domain-instance IDs.
//! - `crate::hybrid::config`: explicit operational budgets and policies.
//! - `crate::hybrid::errors`: canonical hybrid errors.
//! - `crate::quantum::ir`: canonical quantum computation semantics.
//! - The canonical semantic and classical facilities: source-level types,
//!   effects, ownership, and domain-specific computation semantics.
//!
//! This module must not create competing definitions of those concepts.
//!
//! ## Architectural boundary
//!
//! Domain declarations are descriptive contracts. They do not, by
//! themselves, prove that an implementation is registered, available,
//! compatible, trustworthy, or executable.
//!
//! This module does not:
//!
//! - parse Zamani source or define another AST;
//! - implement semantic analysis or a second type/effect system;
//! - define a universal IR or duplicate quantum IR;
//! - discover or allocate physical resources;
//! - select targets or implement target-specific lowering;
//! - schedule, route, calibrate, or execute computation;
//! - depend on runtime sessions, hardware SDKs, or vendor clients.
//!
//! ## Extensibility and scalability
//!
//! Domain identity is extensible rather than restricted to a closed enum.
//! Additional domains can be introduced through validated identifiers and
//! registered contracts without modifying this module.
//!
//! This module imposes no fixed limit on the number of domains, interfaces,
//! interface versions, or composition instances. Actual storage and
//! processing remain subject to available resources and any explicit
//! operational policies enforced by the owning APIs.
//!
//! The absence of an architectural ceiling is not a promise of infinite
//! physical memory, execution time, or target capability.
//!
//! ## Determinism
//!
//! Determinism is governed by the child modules' contracts. This module
//! introduces no registration order, hidden global state, initialization
//! side effects, or implicit fallback behavior.
//!
//! The registry remains an explicitly owned value. It is not a global
//! mutable singleton.
//!
//! ## Integration
//!
//! - `identity.rs` provides `DomainId` and validated domain metadata.
//! - `interface.rs` combines domain identity with the canonical
//!   `DomainContract` and additional participation declarations.
//! - `registry.rs` stores and resolves canonical contract references.
//! - `composition.rs` describes participating domain instances and
//!   structural containment.
//! - The hybrid graph, control, synchronization, and resource layers
//!   consume these contracts without taking over their ownership.
//! - Compiler integration uses validated contracts and compositions;
//!   target realization remains downstream.
//! - The frontend and grammar remain authoritative for source syntax.
//! - `grammar/spec/hybrid.md` remains the normative hybrid specification.
//! - `grammar/hybrid/` remains the hybrid grammar architecture.
//!
//! ## API stability
//!
//! Child modules are declared explicitly, and root-level re-exports are
//! selective. Glob re-exports are intentionally avoided to prevent
//! accidental name collisions and ambiguous ownership as the subsystem
//! evolves.
//!
//! Adding a new domain implementation should normally require no change
//! to this module. Add a child module here only when it represents a real,
//! implemented subsystem with a defined ownership contract.
//!
//! ## Completion criteria
//!
//! - Every declared child module exists.
//! - Every re-export resolves to a public item in its owning module.
//! - Canonical contract and identity ownership remain unambiguous.
//! - Public API additions do not require unrelated child modules to change.
//! - The module introduces no hardware-size ceilings, unsafe code,
//!   global mutable registry, or duplicate semantic implementation.

// -----------------------------------------------------------------------------
// Canonical child modules
// -----------------------------------------------------------------------------

/// Validated, extensible identities for semantic computational domains.
///
/// This module does not register domains or assert implementation
/// availability.
pub mod identity;

/// Contracts describing how a domain participates in hybrid computation.
///
/// The underlying port and domain contract definitions remain owned by
/// `crate::hybrid::contract`.
pub mod interface;

/// Explicit registry of validated domain interface contracts.
///
/// Registry lifetime and shared access are controlled by the caller.
pub mod registry;

/// Structural composition of participating domain instances.
///
/// Structural containment does not imply execution order, data dependency,
/// synchronization, or causality.
pub mod composition;

// -----------------------------------------------------------------------------
// Selective public re-exports
// -----------------------------------------------------------------------------

/// Canonical identity and descriptive metadata for a computational domain.
pub use identity::{DomainId, DomainIdError, DomainIdentity, DomainIdentityError};

/// Interface-level declarations for domain participation.
pub use interface::{
    CapabilityRequirement,
    CapabilityStrength,
    CompletionBehavior,
    DomainInterface,
    EffectRef,
    FailureBehavior,
    PortabilityDeclaration,
    PortabilityLevel,
    VersionCompatibilityPolicy,
};

/// Registry for canonical domain interface contracts.
pub use registry::DomainRegistry;

/// Structural domain composition and participating instances.
pub use composition::{DomainComposition, DomainInstance};
