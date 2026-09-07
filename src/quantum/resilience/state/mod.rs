//! Zamani Quantum Resilience — state subsystem.
//!
//! Path:
//!     src/quantum/resilience/state/mod.rs
//!
//! # Purpose
//!
//! This module is the composition boundary for all resilience execution-state
//! representations.
//!
//! The state subsystem records and exposes the state required to observe,
//! coordinate, recover, verify, persist, and resume resilient quantum
//! execution.
//!
//! It does NOT own:
//!
//! - canonical quantum semantics;
//! - canonical qubit identity;
//! - quantum fault/noise semantics;
//! - QEC algorithms;
//! - routing algorithms;
//! - scheduling algorithms;
//! - optimization algorithms;
//! - hardware discovery;
//! - backend execution;
//! - checkpoint storage engines;
//! - telemetry collection;
//! - authentication;
//! - authorization;
//! - provider-specific behavior.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani Program
//!                              |
//!                              v
//!                      quantum::frontend
//!                              |
//!                              v
//!                        quantum::ir
//!                              |
//!                 +------------+-------------+
//!                 |                          |
//!                 v                          v
//!             compilation                resilience
//!                 |                          |
//!                 |              +-----------+-----------+
//!                 |              |           |           |
//!                 |              v           v           v
//!                 |          detection   diagnosis    policy
//!                 |              |           |           |
//!                 |              +-----------+-----------+
//!                 |                          |
//!                 |                          v
//!                 |                       planning
//!                 |                          |
//!                 |                          v
//!                 |                       recovery
//!                 |                          |
//!                 +------------+-------------+
//!                              |
//!                              v
//!                       state subsystem
//!                              |
//!          +-------------------+-------------------+
//!          |                   |                   |
//!          v                   v                   v
//!      execution            logical            physical
//!          |                   |                   |
//!          +-------------------+-------------------+
//!                              |
//!                              v
//!                         persistence
//! ```
//!
//! # State ownership
//!
//! The state subsystem deliberately separates five different concerns.
//!
//! ## `machine`
//!
//! Owns the global resilience lifecycle state machine.
//!
//! It answers:
//!
//! > Which resilience phase is active?
//!
//! Its state is conceptually:
//!
//! ```text
//! Idle
//! Detecting
//! Diagnosing
//! Planning
//! Adapting
//! Recovering
//! Verifying
//! Completed
//! Escalated
//! Failed
//! ```
//!
//! It does not represent physical machine health or execution progress.
//!
//! ## `execution`
//!
//! Owns execution-local facts.
//!
//! It answers:
//!
//! > What is this particular execution currently doing?
//!
//! It tracks execution identity, status, attempts, generations, progress,
//! logical resources, physical resources, target/resource epochs, semantic
//! identity, and result identity.
//!
//! It is deliberately distinct from `machine`.
//!
//! ## `logical`
//!
//! Owns logical quantum-resource state.
//!
//! It answers:
//!
//! > What is the resilience state of each logical quantum resource?
//!
//! Logical resource state must use the canonical logical identity from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! It must never invent a competing resilience-specific `QubitId`.
//!
//! ## `physical`
//!
//! Owns physical-resource state as observed by resilience.
//!
//! It answers:
//!
//! > What is the current resilience state of physical resources?
//!
//! Physical resources must use the canonical physical identity from:
//!
//! ```text
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! It must not become a replacement for `quantum::hardware`.
//!
//! The hardware subsystem remains authoritative for hardware capability,
//! topology, calibration, device identity, and provider-specific execution
//! concerns.
//!
//! ## `persistence`
//!
//! Owns persistence-facing state operations.
//!
//! It does not make this module dependent on a particular database,
//! filesystem, object store, cloud service, or serialization implementation.
//!
//! Persistence is an integration boundary.
//!
//! # Canonical qubit identity
//!
//! The state subsystem MUST use the canonical quantum IR identity types:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No state module may introduce:
//!
//! ```text
//! ResilienceQubitId
//! StateQubitId
//! LogicalQubitId
//! PhysicalQubit
//! ```
//!
//! as competing replacements for the canonical identities.
//!
//! The distinction between logical and physical resources is semantic even
//! when the underlying representation happens to be integer-like.
//!
//! In particular, the following assumption is forbidden:
//!
//! ```text
//! logical q7 == physical p7
//! ```
//!
//! A logical-to-physical relationship belongs to routing/placement.
//!
//! The state subsystem records that relationship only when another subsystem
//! explicitly supplies it.
//!
//! # Write once / scale everywhere
//!
//! This module imposes no machine-size ceiling.
//!
//! It MUST NOT define or depend on constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PHYSICAL_QUBITS
//! MAX_LOGICAL_QUBITS
//! MAX_OPERATIONS
//! MAX_SHOTS
//! MAX_DEVICES
//! DEFAULT_QUBIT_COUNT
//! DEFAULT_DEVICE_SIZE
//! ```
//!
//! State collections must be dynamically sized and resource-aware.
//!
//! The architecture therefore supports the same state model for:
//!
//! ```text
//! one qubit
//!     |
//! small QPU
//!     |
//! large QPU
//!     |
//! fault-tolerant logical machine
//!     |
//! multi-QPU system
//!     |
//! distributed quantum system
//! ```
//!
//! "Infinite scale" means that the resilience state abstraction does not
//! impose an artificial finite machine-size limit. Every concrete execution
//! remains bounded by available memory, CPU, storage, device capacity,
//! configured resource policies, and representable values.
//!
//! Those limits belong to resource-policy and capability layers, not to this
//! module boundary.
//!
//! # Separation of state domains
//!
//! The following distinctions are mandatory:
//!
//! ```text
//! resilience lifecycle state
//!         !=
//! execution status
//!         !=
//! logical-resource state
//!         !=
//! physical-resource state
//!         !=
//! persistence state
//! ```
//!
//! This prevents one enum or state object from becoming a universal mutable
//! object that couples unrelated parts of the resilience architecture.
//!
//! # Dependency direction
//!
//! The intended dependency graph is:
//!
//! ```text
//!                     quantum::ir::qubit
//!                              |
//!                              v
//!                  +-----------------------+
//!                  |     state subsystem   |
//!                  +-----------------------+
//!                    ^        ^        ^
//!                    |        |        |
//!                    |        |        |
//!              recovery    telemetry   api
//!                    |        |        |
//!                    +--------+--------+
//!                             |
//!                             v
//!                        persistence
//! ```
//!
//! More precisely:
//!
//! ```text
//! state::machine
//!     -> lifecycle semantics
//!
//! state::execution
//!     -> execution-local state
//!
//! state::logical
//!     -> logical-resource state
//!
//! state::physical
//!     -> physical-resource state
//!
//! state::persistence
//!     -> persistence coordination
//! ```
//!
//! Higher-level resilience components may consume these state modules.
//!
//! State modules must not depend on concrete implementations of:
//!
//! ```text
//! recovery
//! planning
//! routing
//! scheduling
//! optimization
//! hardware providers
//! telemetry exporters
//! database engines
//! network clients
//! ```
//!
//! This keeps state foundational and prevents circular dependencies.
//!
//! # Relationship with the rest of `quantum`
//!
//! ## `quantum::ir`
//!
//! The canonical IR defines quantum-program semantics and canonical resource
//! identity.
//!
//! State records execution/resilience information about that program.
//!
//! State MUST NOT redefine IR semantics.
//!
//! ## `quantum::zqn`
//!
//! ZQN owns canonical quantum noise and fault semantics.
//!
//! State may record references, observations, classifications, or derived
//! resilience state, but must not create a competing noise ontology.
//!
//! ## `quantum::hardware`
//!
//! Hardware owns authoritative device and target information.
//!
//! Physical resilience state is an observation/coordination layer and must not
//! become another hardware abstraction layer.
//!
//! ## `quantum::routing`
//!
//! Routing determines logical-to-physical placement.
//!
//! State records placement-related execution facts only when supplied by
//! routing or execution.
//!
//! ## `quantum::scheduling`
//!
//! Scheduling determines operation order and timing.
//!
//! State records execution progress and scheduling-related facts but does not
//! implement scheduling.
//!
//! ## `quantum::optimization`
//!
//! Optimization transforms canonical quantum representations.
//!
//! State may record implementation/provenance identities but does not perform
//! optimization.
//!
//! ## `quantum::error_correction`
//!
//! QEC owns encoding, syndrome processing, decoding, correction and logical
//! fault-tolerance mechanisms.
//!
//! State may record QEC-related resource and execution state but does not
//! implement QEC.
//!
//! ## `quantum::resilience`
//!
//! The resilience layer consumes this state to detect, diagnose, plan, adapt,
//! recover and verify execution.
//!
//! # Public API policy
//!
//! This module intentionally exposes child namespaces rather than performing
//! wildcard re-exports.
//!
//! Preferred usage:
//!
//! ```text
//! crate::quantum::resilience::state::machine
//! crate::quantum::resilience::state::execution
//! crate::quantum::resilience::state::logical
//! crate::quantum::resilience::state::physical
//! crate::quantum::resilience::state::persistence
//! ```
//!
//! Wildcard exports such as:
//!
//! ```text
//! pub use machine::*;
//! pub use execution::*;
//! ```
//!
//! are intentionally avoided.
//!
//! This prevents unrelated public symbols from becoming part of the state
//! composition boundary and avoids name collisions as Zamani grows.
//!
//! Stable, carefully selected convenience re-exports may be added later only
//! when there is a demonstrated compatibility requirement.
//!
//! # State mutation policy
//!
//! Mutable state must remain owned by the state object that semantically owns
//! it.
//!
//! This module must not introduce:
//!
//! ```text
//! static mut
//! global Mutex<State>
//! global RwLock<State>
//! global OnceLock<State>
//! process-wide current execution
//! process-wide current machine
//! ```
//!
//! Resilience state is execution-scoped and resource-scoped.
//!
//! Multiple quantum executions must be able to coexist without accidentally
//! sharing mutable state.
//!
//! # Determinism
//!
//! The composition module itself performs no stochastic work.
//!
//! It must not:
//!
//! - read the system clock;
//! - generate random values;
//! - access environment-dependent state;
//! - perform network access;
//! - perform filesystem I/O;
//! - depend on hash-map iteration order;
//! - initialize global resources;
//! - create threads.
//!
//! Deterministic state transitions are implemented by the owning state module.
//!
//! If a state object contains an ordering-sensitive collection, the owning
//! implementation must choose an explicitly deterministic representation or
//! provide an explicit ordering contract.
//!
//! # Overflow safety
//!
//! State counters such as:
//!
//! ```text
//! revision
//! generation
//! attempt
//! sequence
//! epoch
//! ```
//!
//! must never silently wrap.
//!
//! The owning module must use checked arithmetic and return a structured error
//! when representational capacity is exhausted.
//!
//! This is preferable to silently corrupting resilience history.
//!
//! # Persistence and serialization
//!
//! This module does not select a serialization format.
//!
//! Persistence and serialization are separate concerns:
//!
//! ```text
//! state
//!   |
//!   +--> serialization
//!   |
//!   +--> persistence
//!   |
//!   +--> checkpoint
//! ```
//!
//! A state object must therefore not contain:
//!
//! - database handles;
//! - filesystem handles;
//! - sockets;
//! - cloud SDK clients;
//! - credentials;
//! - provider sessions.
//!
//! State snapshots should remain representationally portable.
//!
//! # Security
//!
//! State can contain operationally sensitive information.
//!
//! State implementations MUST NOT use arbitrary diagnostic strings as a place
//! to store:
//!
//! - passwords;
//! - access tokens;
//! - private keys;
//! - provider credentials;
//! - authentication material;
//! - secret configuration;
//! - raw memory addresses;
//! - unrelated private program data.
//!
//! Security-sensitive metadata must have explicit ownership and access-control
//! semantics elsewhere in the architecture.
//!
//! The state composition boundary itself performs no authentication or
//! authorization.
//!
//! # Recovery safety
//!
//! State must never imply that recovery is successful merely because a state
//! transition succeeded.
//!
//! For example:
//!
//! ```text
//! Recovering -> Completed
//! ```
//!
//! is not equivalent to semantic correctness.
//!
//! Higher-level verification must independently establish whether the recovered
//! computation is acceptable.
//!
//! Therefore the conceptual flow is:
//!
//! ```text
//! recovery action
//!      |
//!      v
//! state update
//!      |
//!      v
//! verification
//!      |
//!      v
//! acceptance decision
//! ```
//!
//! # Checkpoint safety
//!
//! State is not equivalent to a quantum-state snapshot.
//!
//! In particular, this module must not imply that an arbitrary unknown quantum
//! state can be serialized and reconstructed merely because execution metadata
//! was persisted.
//!
//! Checkpoint semantics are owned by `quantum::resilience::checkpoint`.
//!
//! The state subsystem records the execution metadata required to coordinate
//! those checkpoints.
//!
//! # Lifecycle semantics
//!
//! `machine` owns the resilience lifecycle.
//!
//! `execution` owns execution status.
//!
//! These state machines must remain independent.
//!
//! For example:
//!
//! ```text
//! resilience = Recovering
//! execution  = Suspended
//! ```
//!
//! is a valid combination while recovery reconstructs a valid continuation.
//!
//! Likewise:
//!
//! ```text
//! resilience = Verifying
//! execution  = Verifying
//! ```
//!
//! is valid but not required to be the only possible representation.
//!
//! The exact legal combinations are governed by the owning modules and higher
//! level resilience orchestration contracts.
//!
//! # Versioning
//!
//! This composition module intentionally owns no independent semantic version
//! for the entire resilience state subsystem.
//!
//! Individual state schemas own their compatibility versions.
//!
//! This prevents an unrelated change to one state representation from forcing
//! a global version change for every state type.
//!
//! # Rust contract
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! The `unsafe_code` lint is explicitly forbidden at this composition boundary.
//!
//! # Portability requirement
//!
//! All module filenames MUST use portable filesystem names.
//!
//! In particular, the lifecycle module MUST be physically named:
//!
//! ```text
//! machine.rs
//! ```
//!
//! and NOT:
//!
//! ```text
//! machine.rs<space>
//! ```
//!
//! A trailing-space filename is not an acceptable production portability
//! contract because it can behave differently across filesystems and tooling.
//!
//! The current repository contains such a filename and it must be renamed
//! before this module can be considered fully integrated and production-ready.
//!
//! This `mod.rs` intentionally uses the normal Rust module declaration:
//!
//! ```text
//! pub mod machine;
//! ```
//!
//! rather than hiding a non-portable filename with `#[path = "..."]`.
//!
//! # Module declaration contract
//!
//! The state subsystem consists of:
//!
//! ```text
//! execution.rs
//! logical.rs
//! machine.rs
//! persistence.rs
//! physical.rs
//! recovery.rs
//! ```
//!
//! Each child module owns one semantic responsibility.
//!
//! The declaration order below is intentionally stable and follows dependency
//! conceptuality:
//!
//! 1. lifecycle;
//! 2. execution;
//! 3. logical resources;
//! 4. physical resources;
//! 5. persistence;
//! 6. recovery-state coordination.
//!
//! Rust's module compilation does not depend on this declaration order.
//!
//! # Integration contract for future state modules
//!
//! A future state child module MUST:
//!
//! 1. own one clearly defined state domain;
//! 2. use canonical IR identities where quantum identities are required;
//! 3. avoid machine-size constants;
//! 4. avoid process-global mutable state;
//! 5. avoid unsafe Rust;
//! 6. avoid vendor-specific semantics;
//! 7. avoid direct persistence-engine dependencies;
//! 8. avoid direct telemetry-exporter dependencies;
//! 9. document its invariants;
//! 10. document its error semantics;
//! 11. document its versioning contract;
//! 12. document its deterministic ordering behavior;
//! 13. document its integration points;
//! 14. avoid duplicating another quantum subsystem's authority.
//!
//! # Testing contract
//!
//! The state subsystem should be tested at multiple levels.
//!
//! Unit tests belong primarily in the owning child modules.
//!
//! Cross-module state tests should verify:
//!
//! - lifecycle/execution separation;
//! - logical/physical identity separation;
//! - deterministic state transitions;
//! - counter overflow rejection;
//! - invalid transition rejection;
//! - snapshot stability;
//! - persistence round-trip behavior;
//! - checkpoint compatibility;
//! - concurrent independent executions;
//! - large dynamically sized resource collections;
//! - zero-resource edge cases where semantically valid;
//! - arbitrary resource cardinality;
//! - no accidental process-global state.
//!
//! Fault-injection tests should verify that state remains internally
//! consistent when:
//!
//! - detection fails;
//! - diagnosis is uncertain;
//! - planning fails;
//! - recovery fails;
//! - hardware becomes unavailable;
//! - a target changes generation;
//! - a checkpoint cannot be restored;
//! - verification rejects a recovered result.
//!
//! # Compile-time boundary
//!
//! This file intentionally contains no runtime logic.
//!
//! Its responsibilities are limited to:
//!
//! - documenting ownership;
//! - declaring child modules;
//! - establishing the public namespace;
//! - enforcing the no-unsafe composition boundary.
//!
//! Keeping this file thin is deliberate. It means a future state implementation
//! can be added without turning this composition root into another business
//! logic layer.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// ============================================================================
// STATE MODULES
// ============================================================================

/// Global resilience lifecycle state machine.
///
/// Owns the transition system for:
///
/// `Idle -> Detecting -> Diagnosing -> Planning -> Adapting -> Recovering ->
/// Verifying -> Completed/Failed/Escalated`.
///
/// This module does not own execution-local progress or hardware health.
pub mod machine;

/// Execution-local resilience state.
///
/// Owns execution identity, status, attempts, generations, progress,
/// logical/physical resource references and semantic/result identities.
pub mod execution;

/// Logical quantum-resource resilience state.
///
/// Uses canonical `crate::quantum::ir::qubit::QubitId` identities.
pub mod logical;

/// Physical quantum-resource resilience state.
///
/// Uses canonical
/// `crate::quantum::ir::qubit::PhysicalQubitId` identities.
pub mod physical;

/// Persistence-facing state coordination.
///
/// Does not select a persistence engine or serialization format.
pub mod persistence;

/// Recovery-specific state.
///
/// Owns recovery-state representation and recovery-local state semantics.
/// It remains distinct from the global resilience lifecycle in `machine`.
pub mod recovery;

// ============================================================================
// NAMESPACE POLICY
// ============================================================================
//
// Intentionally no wildcard re-exports.
//
// Consumers should use:
//
// crate::quantum::resilience::state::machine
// crate::quantum::resilience::state::execution
// crate::quantum::resilience::state::logical
// crate::quantum::resilience::state::physical
// crate::quantum::resilience::state::persistence
// crate::quantum::resilience::state::recovery
//
// This prevents this composition boundary from becoming a dumping ground for
// unrelated public symbols and preserves explicit ownership of every state
// type.