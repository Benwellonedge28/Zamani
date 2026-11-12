
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Core
//!
//! Path: `src/zur/core/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! The core module is the composition boundary for the foundational
//! Zamani Universal Runtime (ZUR).
//!
//! It connects the independently owned runtime components:
//!
//! - Runtime context.
//! - Runtime lifecycle.
//! - Service registry.
//! - Service manager.
//! - Runtime builder and runtime instance.
//!
//! # Architectural responsibility
//!
//! This module owns module composition and stable public paths.
//!
//! It does not implement runtime execution, service registration,
//! service dependency resolution, provider discovery, hardware
//! selection, scheduling, memory allocation, or physical execution.
//!
//! Each responsibility remains with its authoritative module.
//!
//! # Ownership boundaries
//!
//! | Responsibility | Authoritative module |
//! |---|---|
//! | Runtime configuration | `zur::config` |
//! | Runtime errors | `zur::error` |
//! | Runtime identities | `zur::ids` |
//! | Runtime policies | `zur::policy` |
//! | ABI and execution contracts | `zur::contracts` |
//! | Immutable runtime context | `zur::core::context` |
//! | Runtime lifecycle | `zur::core::lifecycle` |
//! | Service descriptors and dependency resolution | `zur::core::service_registry` |
//! | Service implementation activation and shutdown | `zur::core::service_manager` |
//! | Runtime construction and composition | `zur::core::builder` |
//!
//! # Dependency direction
//!
//! ```text
//!                 zur::config
//!                 zur::error
//!                 zur::ids
//!                 zur::policy
//!                 zur::contracts
//!                      |
//!                      v
//!                zur::core
//!                      |
//!          +-----------+-----------+
//!          |           |           |
//!          v           v           v
//!       context    lifecycle   service_registry
//!          |           |           |
//!          +-----------+-----------+
//!                      |
//!                      v
//!                service_manager
//!                      |
//!                      v
//!                   builder
//!                      |
//!                      v
//!               RuntimeInstance
//! ```
//!
//! The dependency graph describes architectural ownership.
//! It does not require the modules to initialize in source-file order.
//!
//! # Service lifecycle separation
//!
//! The runtime lifecycle and individual service lifecycles are distinct.
//!
//! `RuntimeLifecycle` manages the state of one runtime instance.
//!
//! `ServiceRegistry` manages declarative service registrations and
//! dependency resolution.
//!
//! `ServiceManager` manages actual service implementations, activation,
//! readiness, leases, startup rollback, and shutdown.
//!
//! Registering a service does not activate it.
//! Resolving a service does not initialize it.
//! Initializing a service does not reserve physical hardware.
//! A ready service does not guarantee that every requested operation
//! is feasible or will succeed.
//!
//! # Grammar boundary
//!
//! Source-level intent remains owned by the grammar and frontend.
//!
//! Relevant grammar authorities include:
//!
//! - `grammar/networking/services.g4`
//! - `grammar/execution/runtime.g4`
//! - `grammar/execution/runtime-capabilities.g4`
//! - `grammar/resources/requirements.g4`
//! - `grammar/core/hints.g4`
//!
//! These grammars describe declarations and program intent.
//!
//! They do not directly construct or initialize runtime services.
//!
//! The frontend and semantic layer must validate source declarations
//! and translate them into canonical runtime configuration,
//! policies, service descriptors, and execution contracts.
//!
//! This module must never parse grammar files or interpret source
//! declarations as permission to activate a service.
//!
//! # Computational-domain independence
//!
//! The core is intentionally independent of computational domains.
//!
//! Classical, quantum, HDL, AI, tensor, hybrid, distributed,
//! accelerator, photonic, and future-domain integrations must use
//! the established runtime contracts and their respective
//! authoritative subsystem interfaces.
//!
//! Adding a computational domain must not require adding a module
//! to this core composition root.
//!
//! # POCO-REAF and scalability
//!
//! This module imposes no artificial limits on:
//!
//! - Runtime instances.
//! - Service registrations.
//! - Service implementations.
//! - Service dependencies.
//! - Service versions.
//! - Sessions.
//! - Providers.
//! - Computational domains.
//! - Devices.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Memory.
//! - Distributed participants.
//!
//! The number of components is determined by actual application
//! requirements, available resources, and implementation capacity.
//!
//! Rust collections and the underlying platform remain subject to
//! finite address spaces and available memory. Literal infinite
//! computation is not promised.
//!
//! # Safety
//!
//! This module uses safe Rust only.
//!
//! It introduces:
//!
//! - No unsafe blocks.
//! - No unsafe functions.
//! - No unsafe implementations.
//! - No global mutable state.
//! - No implicit runtime initialization.
//! - No implicit hardware discovery.
//! - No fabricated service availability.
//! - No arbitrary hardware ceilings.
//!
//! # Public API stability
//!
//! The submodules remain publicly accessible for advanced integrations.
//!
//! Selected foundational types are also re-exported at the `core`
//! boundary for convenient use.
//!
//! The original owning modules remain authoritative. Re-exports do
//! not create new types, duplicate state, or transfer ownership.
//!
//! # Integration
//!
//! `zur::core` is consumed by the ZUR composition root.
//!
//! The parent `src/zur/mod.rs` must declare:
//!
//! ```rust
//! pub mod core;
//! ```
//!
//! It must also declare the existing sibling modules:
//!
//! ```rust
//! pub mod config;
//! pub mod contracts;
//! pub mod error;
//! pub mod ids;
//! pub mod policy;
//! pub mod version;
//! ```
//!
//! The crate root must subsequently expose `zur` consistently with
//! the repository's runtime migration plan.


// ============================================================================
// CORE MODULES
// ============================================================================

/// Explicitly injected, immutable runtime context.
///
/// Owns runtime identity association, validated effective policy,
/// operation context, cancellation observation, and optional
/// service-registry injection.
///
/// Does not own service registration or service activation.
pub mod context;

/// Runtime-instance lifecycle state machine.
///
/// Owns runtime lifecycle states and validated state transitions.
///
/// Does not own individual service lifecycle callbacks.
pub mod lifecycle;

/// Declarative runtime service registry.
///
/// Owns service registration, identity allocation, version-aware
/// dependency resolution, dependency ordering, and cycle detection.
///
/// Registration does not activate a service.
pub mod service_registry;

/// Runtime service implementation and lifecycle manager.
///
/// Owns implementation installation, activation, readiness,
/// service leases, startup rollback, and orderly shutdown.
///
/// Does not own service descriptor definitions or dependency graph
/// construction.
pub mod service_manager;

/// Explicit runtime construction and instance composition.
///
/// Owns RuntimeBuilder and RuntimeInstance.
///
/// Composes the context, lifecycle, registry, and service manager.
/// It does not own their underlying implementations.
pub mod builder;


// ============================================================================
// FOUNDATIONAL PUBLIC RE-EXPORTS
// ============================================================================

// Runtime context

/// Immutable context for one independently configured ZUR instance.
pub use context::RuntimeContext;

/// Builder for an immutable runtime context.
pub use context::RuntimeContextBuilder;


// Runtime lifecycle

/// Runtime-instance lifecycle coordinator.
pub use lifecycle::RuntimeLifecycle;

/// Runtime-instance lifecycle state.
pub use lifecycle::RuntimeState;


// Service registry

/// Immutable snapshot of a registered service.
pub use service_registry::RegisteredService;

/// Dependency-ordered service resolution result.
pub use service_registry::ResolvedServiceGraph;

/// Authoritative declarative runtime service registry.
pub use service_registry::ServiceRegistry;


// Service manager

/// Asynchronous lifecycle callback future for managed runtime services.
pub use service_manager::ServiceFuture;

/// Operational implementation contract for a managed runtime service.
pub use service_manager::ManagedRuntimeService;

/// Runtime-managed service lifecycle state.
pub use service_manager::ManagedServiceState;

/// Lease retaining references to activated services.
pub use service_manager::ServiceLease;

/// Runtime service activation and lifecycle coordinator.
pub use service_manager::ServiceManager;


// Runtime composition

/// Builder for one independently composed runtime instance.
pub use builder::RuntimeBuilder;

/// Fully composed runtime instance.
pub use builder::RuntimeInstance;
