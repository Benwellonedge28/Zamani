//! Zamani Quantum Scheduling — Plugin Boundary
//!
//! Path:
//!     src/quantum/scheduling/plugins/mod.rs
//!
//! # Purpose
//!
//! This module is the public composition boundary for the Zamani quantum
//! scheduling plugin subsystem.
//!
//! It exposes:
//!
//! - safe in-process scheduler-plugin contracts;
//! - scheduler-plugin registration;
//! - scheduler-plugin discovery;
//! - scheduler-plugin construction;
//! - stable plugin metadata;
//! - plugin-registry errors;
//! - compatibility inspection.
//!
//! It does NOT implement scheduling algorithms.
//!
//! Concrete scheduling behavior belongs to:
//!
//! ```text
//! crate::quantum::scheduling::algorithms
//! crate::quantum::scheduling::planners
//! ```
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                       quantum::frontend
//!                              │
//!                              ▼
//!                         quantum::ir
//!                              │
//!                              ▼
//!                         optimization
//!                              │
//!                              ▼
//!                           routing
//!                              │
//!                              ▼
//!                    SchedulingContext
//!                              │
//!                              ▼
//!                     scheduling::plugins
//!                              │
//!                 ┌────────────┴────────────┐
//!                 │                         │
//!                 ▼                         ▼
//!           registry.rs               scheduler.rs
//!                 │                         │
//!                 └────────────┬────────────┘
//!                              ▼
//!                    SchedulingPlanner
//!                              │
//!                              ▼
//!                     SchedulingResult
//!                              │
//!             ┌────────────────┼────────────────┐
//!             ▼                ▼                ▼
//!        verification    transformations    diagnostics
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              ▼
//!                       hardware/runtime
//! ```
//!
//! # Responsibility boundary
//!
//! This module answers:
//!
//! > What scheduler-plugin APIs are publicly available?
//!
//! `registry.rs` answers:
//!
//! > Which registered plugin can be instantiated?
//!
//! `scheduler.rs` answers:
//!
//! > What is the safe plugin implementation contract?
//!
//! `planners::planner` answers:
//!
//! > What is the canonical scheduling-planner contract?
//!
//! `algorithms` and `planners` answer:
//!
//! > How is a schedule actually produced?
//!
//! This file must never become a fourth scheduler implementation.
//!
//! # Canonical planner contract
//!
//! The authoritative scheduler implementation contract is:
//!
//! ```text
//! crate::quantum::scheduling::planners::planner::SchedulingPlanner
//! ```
//!
//! This module deliberately does not define another scheduler trait.
//!
//! Consequently, a plugin implementation follows this dependency direction:
//!
//! ```text
//! custom plugin
//!      │
//!      ▼
//! SchedulingPlanner
//!      │
//!      ▼
//! SchedulingContext
//!      │
//!      ▼
//! SchedulingResult
//! ```
//!
//! # Write once, scale everywhere
//!
//! Nothing in this module encodes:
//!
//! - qubit count;
//! - physical-qubit count;
//! - logical-qubit count;
//! - operation count;
//! - gate count;
//! - gate arity;
//! - resource count;
//! - channel count;
//! - topology dimensions;
//! - timing units;
//! - schedule depth;
//! - QEC distance;
//! - hardware technology;
//! - provider;
//! - vendor;
//! - machine generation.
//!
//! A plugin therefore remains applicable to any target for which its declared
//! planner capabilities and the supplied `SchedulingContext` are compatible.
//!
//! "Infinity" means that this module introduces no artificial quantum-machine
//! ceiling. Actual compilation remains bounded by the available address space,
//! memory, compute resources, explicit resource/deployment limits, and the
//! target itself.
//!
//! # Canonical qubit identity
//!
//! This module does not create scheduler-specific qubit identities.
//!
//! When a plugin implementation needs qubit identities, it MUST use the
//! canonical quantum IR types:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No plugin API exported from this module may introduce an alternative
//! `QubitId` or `PhysicalQubitId`.
//!
//! # Separation from routing
//!
//! Routing owns:
//!
//! ```text
//! logical qubit
//!      │
//!      ▼
//! physical placement
//! ```
//!
//! Scheduling owns:
//!
//! ```text
//! mapped operation
//!      │
//!      ▼
//! execution time
//! ```
//!
//! Therefore a plugin must consume mapped/target-aware information through
//! `SchedulingContext` rather than performing logical-to-physical routing.
//!
//! # Separation from hardware
//!
//! Plugins do not:
//!
//! - discover hardware;
//! - authenticate;
//! - access credentials;
//! - open provider connections;
//! - execute jobs;
//! - query live hardware;
//! - invoke vendor SDKs;
//! - mutate hardware state.
//!
//! Target information enters through the scheduler context and the appropriate
//! hardware adapter.
//!
//! # Separation from QEC
//!
//! A QEC-specific scheduler may be implemented as a plugin, but it must consume
//! QEC requirements through explicit scheduling/QEC contracts.
//!
//! This module does not implement:
//!
//! - syndrome extraction;
//! - decoder logic;
//! - stabilizer semantics;
//! - surface-code construction;
//! - QEC rounds.
//!
//! # Separation from optimization
//!
//! A plugin may optimize scheduling objectives, but it must not silently perform
//! unrelated quantum-compiler transformations.
//!
//! Gate synthesis, decomposition, algebraic optimization, and equivalent
//! circuit rewriting belong to their respective compiler subsystems.
//!
//! # Safe extensibility
//!
//! This module exposes safe in-process Rust plugin support.
//!
//! It intentionally does not provide:
//!
//! - `unsafe`;
//! - native ABI loading;
//! - `dlopen`;
//! - `LoadLibrary`;
//! - raw function pointers;
//! - FFI symbol discovery;
//! - implicit dynamic-library loading.
//!
//! Independently deployed plugins should eventually use an explicitly specified
//! safe boundary such as:
//!
//! - a process/IPC protocol;
//! - WASM;
//! - another validated serialized execution boundary.
//!
//! Such a mechanism should be introduced as a separate subsystem rather than
//! weakening this safe Rust boundary.
//!
//! # Ownership
//!
//! There is no process-global mutable plugin registry.
//!
//! The registry is explicitly caller-owned:
//!
//! ```text
//! application/compiler
//!        │
//!        ▼
//! SchedulerPluginRegistry
//!        │
//!        ├── registration
//!        ├── inspection
//!        └── construction
//! ```
//!
//! This prevents hidden cross-compilation state.
//!
//! Applications that need shared mutable access may place their registry behind
//! their own synchronization primitive. The scheduling plugin module does not
//! impose a global synchronization strategy.
//!
//! # Factory isolation
//!
//! Registered plugins are represented by factories rather than long-lived
//! scheduler instances.
//!
//! ```text
//! registry
//!    │
//!    ├── metadata
//!    │
//!    └── factory
//!          │
//!          ▼
//!       new planner
//!          │
//!          ▼
//!       one request
//! ```
//!
//! A factory must return a fresh planner instance for each construction request.
//!
//! This prevents scheduler state from one compilation leaking into another.
//!
//! # Determinism
//!
//! This module provides deterministic API exposure and deterministic registry
//! ordering through the underlying registry implementation.
//!
//! It does not impose a scheduling algorithm's tie-breaking semantics.
//!
//! A deterministic scheduler invocation remains the responsibility of the
//! concrete planner and `SchedulingContext`.
//!
//! When deterministic scheduling is requested, a plugin must respect the
//! planner contract's deterministic requirements.
//!
//! # Explicit plugin selection
//!
//! An explicitly requested plugin must either:
//!
//! ```text
//! requested plugin
//!       │
//!       ├── registered ──► instantiate
//!       │
//!       └── absent ──────► explicit error
//! ```
//!
//! There must be no silent replacement by another scheduler.
//!
//! Automatic algorithm selection belongs to a higher-level planner-selection
//! policy, not this public module.
//!
//! # Versioning
//!
//! Plugin versioning is intentionally separated into layers:
//!
//! ```text
//! Zamani package version
//!          │
//!          ├── plugin registry API version
//!          │
//!          ├── planner contract version
//!          │
//!          └── concrete plugin implementation version
//! ```
//!
//! A concrete plugin version must not be interpreted as a hardware version.
//!
//! # Thread safety
//!
//! Factories exposed by the plugin implementation contract are safe Rust
//! values. The registry owns them according to the registry contract.
//!
//! This module itself contains no mutable global state.
//!
//! # Rust compatibility
//!
//! Required:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! =============================================================================
//! Module declarations
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

/// Safe scheduler-plugin implementation contracts.
///
/// This module contains the plugin-facing implementation abstraction and
/// plugin metadata/factory contracts. It delegates actual scheduling behavior
/// to the canonical `SchedulingPlanner`.
pub mod scheduler;

/// Caller-owned scheduler-plugin registry.
///
/// This module owns registration, deterministic discovery, compatibility
/// inspection, and fresh planner construction.
pub mod registry;

// =============================================================================
// Stable public exports
// =============================================================================
//
// Re-export only the public plugin API.
// Keep implementation modules accessible through their canonical module paths
// as well, but make common use ergonomic from:
//!
//!     quantum::scheduling::plugins::*
//!
//! No algorithm implementation is re-exported here.
// =============================================================================

pub use registry::{
    RegisteredSchedulerPlugin,
    SchedulerPluginDescriptor,
    SchedulerPluginFactory,
    SchedulerPluginFactoryProvider,
    SchedulerPluginId,
    SchedulerPluginRegistry,
    SchedulerPluginRegistryError,
    SCHEDULER_PLUGIN_REGISTRY_API_VERSION,
};

pub use scheduler::{
    SchedulerPlugin,
    SchedulerPluginError,
    SchedulerPluginMetadata,
    SCHEDULER_PLUGIN_API_VERSION,
};

// =============================================================================
// Compatibility aliases
// =============================================================================
//
// These aliases are deliberately limited to semantic names already owned by
// the plugin modules. They do not create alternate scheduler contracts.
//
// Do not add algorithm-specific aliases here.
// =============================================================================

/// Canonical registry type used by scheduler clients.
///
/// This alias exists only as an ergonomic public API name and does not create
/// a second registry implementation.
pub type PluginRegistry = SchedulerPluginRegistry;

/// Canonical plugin descriptor name for callers that prefer the shorter term.
pub type PluginDescriptor = SchedulerPluginDescriptor;

// =============================================================================
// Public API documentation
// =============================================================================

/// Returns the scheduler-plugin subsystem API version.
///
/// This is the version of the plugin boundary, not the version of a concrete
/// scheduler implementation and not the version of the planner behavioral
/// contract.
///
/// # Stability
///
/// This function is intentionally trivial and stable so applications can
/// inspect the plugin API without constructing a registry or planner.
#[must_use]
pub const fn plugin_api_version() -> u32 {
    SCHEDULER_PLUGIN_API_VERSION
}

/// Returns the scheduler-plugin registry API version.
///
/// This is intentionally separate from `plugin_api_version()` because the
/// implementation contract and registry contract may evolve independently.
#[must_use]
pub const fn registry_api_version() -> u32 {
    SCHEDULER_PLUGIN_REGISTRY_API_VERSION
}

// =============================================================================
// Integration contract
// =============================================================================
//
// The following dependency direction is intentional:
//
//     quantum::ir
//          │
//          ▼
//     optimization
//          │
//          ▼
//       routing
//          │
//          ▼
// SchedulingContext
//          │
//          ▼
// scheduling::plugins
//          │
//          ├── scheduler::SchedulerPlugin
//          │          │
//          │          ▼
//          │   SchedulingPlanner
//          │
//          └── registry::SchedulerPluginRegistry
//                     │
//                     ▼
//                 fresh planner
//
// No reverse dependency is introduced from IR, routing, hardware, QEC, or
// optimization into this module merely because a plugin exists.
//
// =============================================================================

// =============================================================================
// Usage contract
// =============================================================================
//
// A normal caller should conceptually perform:
//
//     let mut registry = SchedulerPluginRegistry::new();
//
//     registry.register(...)?;
//
//     let planner = registry.create(...)?;
//
//     let result = planner.plan(context)?;
//
// The exact constructor/registration arguments remain defined by
// `registry.rs` so this composition root does not duplicate them.
//
// =============================================================================

// =============================================================================
// Production invariants
// =============================================================================
//
// The plugin boundary is considered valid only while all of the following hold:
//
// 1. No unsafe code.
//
// 2. No global mutable registry.
//
// 3. No scheduler algorithm implementation.
//
// 4. No hardware-provider implementation.
//
// 5. No routing implementation.
//
// 6. No QEC implementation.
//
// 7. No competing `SchedulingPlanner` trait.
//
// 8. No competing qubit identity type.
//
// 9. No fixed quantum-machine size.
//
// 10. No fixed qubit count.
//
// 11. No fixed operation count.
//
// 12. No fixed resource count.
//
// 13. No fixed timing unit.
//
// 14. No fixed topology.
//
// 15. No fixed QEC distance.
//
// 16. No implicit fallback when an explicitly requested plugin is unavailable.
//
// 17. Fresh planner construction for independent scheduling requests.
//
// 18. Plugin compatibility must be explicit.
//
// 19. Plugin metadata must remain deterministic.
//
// 20. Public API changes must be deliberate and versioned.
//
// =============================================================================

// =============================================================================
// Future extension rule
// =============================================================================
//
// If a future feature requires adding:
//
//     - a new scheduling algorithm;
//     - a new scheduling heuristic;
//     - a new hardware technology;
//     - a new resource type;
//     - a new timing model;
//     - a new QEC strategy;
//     - a new routing algorithm;
//     - a new optimization objective;
//
// DO NOT modify this file merely to accommodate the feature.
//
// Instead:
//
//     algorithm      -> scheduling::algorithms
//     planner        -> scheduling::planners
//     resource       -> scheduling::resources
//     timing         -> scheduling::timing
//     constraint     -> scheduling::constraints
//     QEC             -> scheduling::qec
//     routing adapter -> scheduling::adapters::routing
//     hardware adapter-> scheduling::adapters::hardware
//
// Modify this file only when the public plugin-boundary composition itself
// genuinely changes.
//
// =============================================================================

// =============================================================================
// API migration rule
// =============================================================================
//
// `stabilizer_scheduler.rs` is not imported here deliberately.
//
// It is a concrete/legacy scheduler concern and must not become part of the
// plugin composition boundary merely because it is scheduler-related.
//
// The intended architecture is:
//
//     stabilizer/QEC scheduling
//             │
//             ▼
//     qec scheduling constraints
//             │
//             ▼
//     generic SchedulingPlanner
//             │
//             ▼
//     SchedulingResult
//
// This prevents the plugin subsystem from becoming coupled to one QEC code.
//
// =============================================================================

// =============================================================================
// Canonical qubit integration reminder
// =============================================================================
//
// Plugin implementations that need qubit identities must import:
//
//     use crate::quantum::ir::qubit::QubitId;
//     use crate::quantum::ir::qubit::PhysicalQubitId;
//
// They must not import a scheduler-local qubit identity because this module
// does not define one.
//
// =============================================================================

// =============================================================================
// Compile-time safety posture
// =============================================================================
//
// The module-level attributes above deliberately make unsafe Rust unavailable
// here. The same restriction should be maintained by all scheduler plugin
// implementations intended for production use.
//
// =============================================================================