//! Zamani Quantum IR — Core Foundation
//!
//! This module defines and exposes the dependency-lowest foundation of the
//! canonical Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `quantum::ir::core` contains the primitives upon which the remainder of
//! the Quantum IR is built:
//!
//! ```text
//!                         quantum::ir::core
//!                                │
//!        ┌───────────────────────┼────────────────────────┐
//!        │           │           │           │            │
//!        ▼           ▼           ▼           ▼            ▼
//!     identity    version      types      values      parameters
//!        │           │           │           │            │
//!        └───────────┴───────────┴───────────┴────────────┘
//!                                │
//!                                ▼
//!                    higher-level Quantum IR
//!                                │
//!              ┌─────────────────┼─────────────────┐
//!              ▼                 ▼                 ▼
//!           qubit              gate             operation
//!              │                 │                 │
//!              └─────────────────┼─────────────────┘
//!                                ▼
//!                         quantum::ir::program
//! ```
//!
//! The core layer is deliberately below:
//!
//! - quantum gates;
//! - circuits;
//! - operations;
//! - control flow;
//! - pulse semantics;
//! - scheduling;
//! - routing;
//! - hardware;
//! - optimization;
//! - simulation;
//! - QEC;
//! - backend execution.
//!
//! # Responsibilities
//!
//! This module owns only the module boundary for:
//!
//! - stable IR identities;
//! - IR versioning;
//! - canonical IR types;
//! - canonical IR values;
//! - symbolic parameters;
//! - typed extensible attributes;
//! - forward-compatible extensions;
//! - canonical IR error primitives;
//! - explicit compilation/security limits.
//!
//! It does not implement quantum-computing algorithms.
//!
//! # Universal-program principle
//!
//! The core layer contains **no quantum-machine-size constant**.
//!
//! In particular, it must never define or imply architectural limits such as:
//!
//! ```text
//! 32 qubits
//! 63 qubits
//! 64 qubits
//! 128 qubits
//! 4096 qubits
//! 1_000_000 qubits
//! ```
//!
//! Such values may occur in tests, target descriptions, or explicit
//! per-compilation resource policies, but they are never semantic limits of
//! Zamani.
//!
//! A program is therefore allowed to describe any finite computation that can
//! be represented and processed by the available compiler/runtime resources.
//!
//! The practical execution boundary is determined by:
//!
//! ```text
//! source program
//!      │
//!      ▼
//! compiler/resource policy
//!      │
//!      ▼
//! host/platform resources
//!      │
//!      ▼
//! target capabilities
//!      │
//!      ▼
//! target hardware
//! ```
//!
//! None of those concrete constraints belong in the semantic core.
//!
//! # Logical versus physical quantum identity
//!
//! The canonical quantum identity boundary is deliberately outside this
//! module:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! `core` must never define another `QubitId`, `PhysicalQubitId`, or equivalent
//! quantum-resource identity.
//!
//! This prevents the historical inconsistency where some code referred to a
//! `qubits` module while the canonical module was `qubit`.
//!
//! New code must use:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! # Dependency direction
//!
//! The canonical dependency direction is strictly downward toward this
//! foundation:
//!
//! ```text
//!                    frontend
//!                       │
//!                       ▼
//!                quantum::ir
//!                       │
//!       ┌───────────────┴────────────────┐
//!       │                                │
//!       ▼                                ▼
//! quantum semantic IR              quantum::ir::core
//!       │                                ▲
//!       │                                │
//!       └────────────────────────────────┘
//! ```
//!
//! More precisely:
//!
//! ```text
//! core
//!  ▲
//!  │
//! quantum / classical / program / pulse / timing / models / resources
//!  ▲
//!  │
//! validation / analysis / serialization / hashing / dialects
//!  ▲
//!  │
//! frontend / optimization / routing / scheduling / hardware / backend
//! ```
//!
//! `core` must never depend on those higher-level layers.
//!
//! # Forbidden dependencies
//!
//! The core module must never depend on:
//!
//! - `crate::quantum::frontend`;
//! - `crate::quantum::optimization`;
//! - `crate::quantum::routing`;
//! - `crate::quantum::scheduling`;
//! - `crate::quantum::hardware`;
//! - `crate::quantum::simulator`;
//! - `crate::quantum::qec`;
//! - backend implementations;
//! - vendor SDKs;
//! - target-specific calibration databases;
//! - network clients;
//! - filesystem execution;
//! - source-language ASTs.
//!
//! This keeps the semantic foundation reusable by every downstream compiler
//! and execution technology.
//!
//! # Module ownership
//!
//! ```text
//! core/
//! ├── mod.rs
//! ├── attribute.rs
//! ├── errors.rs
//! ├── extension.rs
//! ├── identity.rs
//! ├── limits.rs
//! ├── parameter.rs
//! ├── types.rs
//! ├── value.rs
//! └── version.rs
//! ```
//!
//! Each file has one authoritative responsibility.
//!
//! ```text
//! attribute.rs
//!     Typed extensible attributes and metadata values.
//!
//! errors.rs
//!     Core IR error vocabulary and result contracts.
//!
//! extension.rs
//!     Forward-compatible extension representation.
//!
//! identity.rs
//!     Stable identifiers for IR objects and namespaces.
//!
//! limits.rs
//!     Explicit per-compilation/resource/security limits.
//!
//! parameter.rs
//!     Symbolic and parameterized computation values.
//!
//! types.rs
//!     Canonical IR type vocabulary.
//!
//! value.rs
//!     Canonical typed IR values.
//!
//! version.rs
//!     IR schema/semantic version contract.
//! ```
//!
//! `mod.rs` owns none of those data structures itself.
//!
//! # Identity boundary
//!
//! Identity definitions belong exclusively to `identity.rs`, except for
//! quantum logical/physical qubit identity, which belongs exclusively to
//! `quantum::ir::qubit`.
//!
//! This distinction is intentional:
//!
//! ```text
//! core::identity
//!     ProgramId
//!     ModuleId
//!     RegionId
//!     BlockId
//!     OperationId
//!     ValueId
//!     ParameterId
//!     ...
//!
//! quantum::ir::qubit
//!     QubitId
//!     PhysicalQubitId
//! ```
//!
//! Do not add aliases here merely for convenience if they create another
//! authority for a semantic type.
//!
//! # Version boundary
//!
//! `version.rs` owns the canonical IR version.
//!
//! `mod.rs` must not introduce:
//!
//! - another version number;
//! - another compatibility policy;
//! - another schema version type;
//! - compiler-version semantics;
//! - hardware-version semantics.
//!
//! Downstream modules consume the canonical version contract from
//! `version.rs`.
//!
//! # Limits boundary
//!
//! `limits.rs` owns explicit operational and security limits.
//!
//! These limits are **policy**, not architecture.
//!
//! Correct:
//!
//! ```text
//! this compilation permits at most N operations
//! ```
//!
//! Incorrect:
//!
//! ```text
//! Zamani supports at most N operations
//! ```
//!
//! Correct:
//!
//! ```text
//! this compiler invocation permits at most N logical qubits
//! ```
//!
//! Incorrect:
//!
//! ```text
//! Zamani supports at most N logical qubits
//! ```
//!
//! The distinction is essential for the "write once, scale anywhere"
//! requirement.
//!
//! # Type/value separation
//!
//! `types.rs` defines what a value *is*.
//!
//! `value.rs` defines a concrete canonical value.
//!
//! `parameter.rs` defines symbolic/parameterized computation.
//!
//! These concepts must not be collapsed into one universal enum merely for
//! convenience.
//!
//! Conceptually:
//!
//! ```text
//! Type
//!   │
//!   ├── describes valid representation
//!   │
//!   ▼
//! Value
//!   │
//!   └── concrete semantic data
//!
//! Parameter
//!   │
//!   └── symbolic/later-bound computation
//! ```
//!
//! # Attribute/extension separation
//!
//! Attributes are typed semantic metadata attached to known IR objects.
//!
//! Extensions represent forward-compatible structures that are not necessarily
//! known to the current implementation.
//!
//! Therefore:
//!
//! ```text
//! attribute.rs
//!     = known typed metadata
//!
//! extension.rs
//!     = extensibility boundary
//! ```
//!
//! Unknown extension data must never be silently discarded merely because the
//! current compiler does not understand it.
//!
//! # Error boundary
//!
//! `errors.rs` is the canonical error foundation for the core layer.
//!
//! Higher-level modules may define domain-specific error variants, but those
//! errors must integrate with the canonical IR error contract rather than
//! creating unrelated error hierarchies.
//!
//! # Serialization
//!
//! The core layer does not own the complete serialization format.
//!
//! Canonical persistence belongs to the higher-level IR serialization layer.
//!
//! Core types should nevertheless remain suitable for deterministic encoding:
//!
//! - stable identifiers;
//! - explicit values;
//! - deterministic ordering where semantic ordering is required;
//! - no memory-address identity;
//! - no process-local identity;
//! - no hidden global state.
//!
//! # Hashing
//!
//! The core layer does not calculate complete program hashes.
//!
//! Canonical hashing belongs to the IR hashing layer.
//!
//! Core primitives must therefore avoid nondeterministic semantic state.
//!
//! # Thread safety
//!
//! This module intentionally contains no global mutable state and no global
//! identity allocator.
//!
//! Identity allocation must be controlled by the owning program/compiler
//! construction context.
//!
//! This makes core primitives suitable for:
//!
//! - parallel compilation;
//! - incremental compilation;
//! - deterministic builds;
//! - distributed compilation;
//! - caching;
//! - reproducible builds.
//!
//! # Security
//!
//! The core boundary must not use `unsafe`.
//!
//! It must also avoid introducing implicit resource allocation or unbounded
//! global state.
//!
//! Resource-sensitive construction belongs to explicit APIs such as the
//! `QuantumIrLimits` contract.
//!
//! # Rust compatibility
//!
//! Required toolchain:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! The module explicitly forbids unsafe code.
//!
//! # Integration contract
//!
//! The parent `quantum::ir` module should expose this module with:
//!
//! ```text
//! pub mod core;
//! ```
//!
//! Higher-level IR modules should then depend on the canonical paths:
//!
//! ```text
//! quantum::ir::core::attribute
//! quantum::ir::core::errors
//! quantum::ir::core::extension
//! quantum::ir::core::identity
//! quantum::ir::core::limits
//! quantum::ir::core::parameter
//! quantum::ir::core::types
//! quantum::ir::core::value
//! quantum::ir::core::version
//! ```
//!
//! The parent `quantum::ir` module may selectively re-export stable public
//! types for compatibility, but `core::mod.rs` must not create duplicate
//! definitions simply to support root-level convenience imports.
//!
//! # Integration with quantum::ir::qubit
//!
//! This module intentionally does not import `quantum::ir::qubit`.
//!
//! That is deliberate.
//!
//! `core` is lower in the dependency graph than the quantum-specific semantic
//! layer. If a core abstraction needs to refer to a qubit identity, that
//! dependency should be designed explicitly in the higher-level module rather
//! than introducing a circular foundation dependency.
//!
//! Therefore downstream code should use:
//!
//! ```text
//! use crate::quantum::ir::core::identity::OperationId;
//! use crate::quantum::ir::qubit::QubitId;
//! ```
//!
//! rather than attempting to manufacture a qubit identity from core.
//!
//! # Integration with existing IR files
//!
//! The current repository contains both the established flat IR modules and
//! the newer canonical `core/` foundation files.
//!
//! The intended final architecture is:
//!
//! ```text
//! quantum::ir
//!     │
//!     ├── core
//!     │   ├── attribute
//!     │   ├── errors
//!     │   ├── extension
//!     │   ├── identity
//!     │   ├── limits
//!     │   ├── parameter
//!     │   ├── types
//!     │   ├── value
//!     │   └── version
//!     │
//!     ├── qubit
//!     ├── gate
//!     ├── operation
//!     ├── program
//!     ├── classical
//!     ├── control_flow
//!     ├── pulse
//!     ├── timing
//!     ├── ...
//!     └── compatibility
//! ```
//!
//! During migration, old flat modules may remain available, but they must not
//! become competing authorities for the same semantic type.
//!
//! In particular:
//!
//! ```text
//! canonical qubit identity
//!     = quantum::ir::qubit::QubitId
//!
//! not
//!     = quantum::ir::core::QubitId
//!
//! not
//!     = quantum::ir::qubits::QubitId
//! ```
//!
//! # Public API philosophy
//!
//! This module deliberately does not perform broad glob re-exports such as:
//!
//! ```text
//! pub use attribute::*;
//! pub use errors::*;
//! pub use types::*;
//! ```
//!
//! Broad glob exports make API ownership ambiguous, increase the probability
//! of name collisions, and make future additions capable of breaking unrelated
//! code.
//!
//! The canonical public boundary is therefore the module path itself.
//!
//! Higher-level modules can explicitly import the exact symbols they require.
//!
//! # Dependency ordering
//!
//! The conceptual implementation order of the core files is:
//!
//! ```text
//! version
//!    │
//!    ├── identity
//!    │
//!    ├── types
//!    │
//!    ├── value
//!    │
//!    ├── parameter
//!    │
//!    ├── attribute
//!    │
//!    ├── extension
//!    │
//!    ├── errors
//!    │
//!    └── limits
//! ```
//!
//! The physical Rust module declarations do not impose this implementation
//! order. They merely expose the already-separated contracts.
//!
//! # What belongs here in the future
//!
//! A new module may be added under `core/` only if its responsibility is a
//! genuinely foundational, hardware-independent semantic primitive required by
//! multiple independent IR layers.
//!
//! Examples that may belong here:
//!
//! - canonical units;
//! - identifier namespaces;
//! - generic immutable semantic references;
//! - schema-neutral primitive representations.
//!
//! Examples that do NOT belong here:
//!
//! - gate decomposition;
//! - routing;
//! - scheduling;
//! - topology;
//! - calibration;
//! - pulse synthesis;
//! - hardware drivers;
//! - simulators;
//! - QEC decoders;
//! - optimization algorithms.
//!
//! # Production-readiness invariants
//!
//! This module guarantees the following architectural properties:
//!
//! 1. No unsafe code.
//! 2. No global mutable state.
//! 3. No hardware dependency.
//! 4. No fixed quantum-machine size.
//! 5. No fixed gate universe.
//! 6. No duplicate canonical qubit identity.
//! 7. No source-language AST dependency.
//! 8. No backend dependency.
//! 9. No optimizer dependency.
//! 10. No router dependency.
//! 11. No scheduler dependency.
//! 12. No simulator dependency.
//! 13. No QEC dependency.
//! 14. No vendor dependency.
//! 15. Explicit limits remain policy rather than architecture.
//! 16. Versioning remains centralized.
//! 17. Identity remains centralized.
//! 18. Type/value/parameter responsibilities remain separated.
//! 19. Extension handling remains explicit.
//! 20. Higher-level IR remains free to evolve independently.
//!
//! # Module declarations
//!
//! These are the only declarations that belong in this file.
//!
//! No semantic implementation should be placed in `mod.rs`.
//!
//! -----------------------------------------------------------------------------
//! Compiler-enforced safety boundary
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Core foundation modules
// =============================================================================

/// Typed, namespaced, extensible IR attributes.
///
/// Owns metadata attached to semantic IR objects.
///
/// Does not own arbitrary forward-compatible extension objects.
pub mod attribute;

/// Canonical errors used by the Quantum IR foundation.
///
/// Owns the error vocabulary shared by core primitives.
pub mod errors;

/// Forward-compatible IR extensions.
///
/// Owns representations required to preserve or explicitly handle IR
/// constructs unknown to the current implementation.
pub mod extension;

/// Stable identities and IR identity/version primitives.
///
/// IMPORTANT:
/// Logical and physical qubit identities remain owned by
/// `quantum::ir::qubit`.
pub mod identity;

/// Explicit resource and security limits.
///
/// These are per-compilation/per-operation policies and never architectural
/// limits on the size of quantum machines Zamani can describe.
pub mod limits;

/// Symbolic and parameterized computation primitives.
///
/// Owns constants, symbolic parameters, expressions, and parameter binding
/// contracts at the foundation level.
pub mod parameter;

/// Canonical hardware-independent IR type vocabulary.
///
/// Does not contain hardware-specific target types.
pub mod types;

/// Canonical typed semantic values.
///
/// Does not contain simulator state or physical quantum amplitudes.
pub mod value;

/// Canonical Quantum IR schema and semantic version contract.
///
/// Owns the single authoritative IR version definition.
pub mod version;

// =============================================================================
// Deliberately absent modules
// =============================================================================
//
// The following are intentionally NOT declared here.
//
// They belong to higher semantic layers:
//
// quantum::ir::qubit
// quantum::ir::gate
// quantum::ir::operation
// quantum::ir::program
// quantum::ir::circuit
// quantum::ir::classical
// quantum::ir::control_flow
// quantum::ir::pulse
// quantum::ir::timing
// quantum::ir::models
// quantum::ir::resources
// quantum::ir::scheduling
// quantum::ir::analysis
// quantum::ir::validation
// quantum::ir::serialization
// quantum::ir::hashing
// quantum::ir::dialect
//
// Keeping those layers outside `core` is what prevents the foundation from
// becoming coupled to a particular quantum-computing execution model.
//
// =============================================================================
// No root-level glob re-exports
// =============================================================================
//
// Intentionally do NOT write:
//
// pub use attribute::*;
// pub use errors::*;
// pub use extension::*;
// pub use identity::*;
// pub use limits::*;
// pub use parameter::*;
// pub use types::*;
// pub use value::*;
// pub use version::*;
//
// Explicit module paths preserve ownership and prevent future name collisions.
//
// Consumers should use:
//
// use crate::quantum::ir::core::identity::OperationId;
// use crate::quantum::ir::core::types::...;
// use crate::quantum::ir::core::value::...;
// use crate::quantum::ir::qubit::QubitId;
//
// =============================================================================
// End of canonical core module boundary
// =============================================================================