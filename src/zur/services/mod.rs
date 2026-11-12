
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Services
//!
//! Path: `src/zur/services/mod.rs`
//!
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! PURPOSE
//!
//! Provides the public composition boundary for ZUR service-related
//! dependency planning.
//!
//! This module exposes the existing service dependency-planning
//! implementation through a stable, maintainable module path.
//!
//! It does not introduce another service-management subsystem.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Composition of service dependency-planning modules.
//! - Public exposure of dependency-planning types.
//! - Stable module paths for service dependency planning.
//!
//! The implementation responsibilities remain with their
//! respective authoritative modules.
//!
//! AUTHORITATIVE MODULES
//!
//! `crate::zur::core::service_registry`
//!     Owns service registration, version resolution,
//!     dependency graph construction and cycle detection.
//!
//! `crate::zur::contracts::service`
//!     Owns service descriptors and dependency declarations.
//!
//! `crate::zur::services::dependencies`
//!     Owns immutable dependency plans, resolved dependency
//!     relationships and dependency-plan validation.
//!
//! `crate::zur::core::service_manager`
//!     Owns actual service implementation activation,
//!     initialization, readiness, leases, rollback and shutdown.
//!
//! `crate::zur::config`
//!     Owns service identity and activation configuration.
//!
//! `crate::zur::version`
//!     Owns service contract versions and version ranges.
//!
//! `crate::zur::error`
//!     Owns structured ZUR errors.
//!
//! RESPONSIBILITY BOUNDARIES
//!
//! This module MUST NOT:
//!
//! - Implement another service registry.
//! - Implement another dependency resolver.
//! - Duplicate dependency graph construction.
//! - Duplicate dependency cycle detection.
//! - Implement service activation.
//! - Initialize service implementations.
//! - Implement service shutdown.
//! - Manage service leases.
//! - Perform provider discovery.
//! - Discover physical hardware.
//! - Reserve physical resources.
//! - Select execution backends.
//! - Implement runtime scheduling.
//! - Execute classical, quantum or other domain computations.
//! - Parse Zamani source grammars.
//! - Interpret source-level service declarations.
//! - Introduce computational-domain enums.
//! - Introduce global mutable state.
//!
//! INTEGRATION FLOW
//!
//! ```text
//! contracts/service.rs
//!          |
//!          v
//! core/service_registry.rs
//!          |
//!          v
//! ResolvedServiceGraph
//!          |
//!          v
//! services/dependencies.rs
//!          |
//!          v
//! ServiceDependencyPlan
//!          |
//!          v
//! core/service_manager.rs
//!          |
//!          v
//! Actual service lifecycle
//! ```
//!
//! The registry remains authoritative for dependency resolution.
//!
//! This module exposes the resulting dependency-planning API.
//!
//! The service manager remains authoritative for actual
//! service initialization and lifecycle operations.
//!
//! GRAMMAR BOUNDARY
//!
//! Source-level service and execution intent belongs to the
//! canonical Zamani grammar and frontend/semantic architecture.
//!
//! Relevant grammar authorities include:
//!
//! - grammar/distributed/services.g4
//! - grammar/networking/services.g4
//! - grammar/networking/service-discovery.g4
//! - grammar/execution/runtime.g4
//! - grammar/execution/runtime-capabilities.g4
//! - grammar/core/capabilities.g4
//! - grammar/core/requirements.g4
//! - grammar/resources/requirements.g4
//!
//! These grammars describe source-level declarations, requirements,
//! capabilities and execution intent.
//!
//! They do not register, initialize or execute runtime services.
//!
//! The frontend and semantic layer must validate source declarations
//! and translate them into the canonical runtime contracts.
//!
//! This module must never parse grammar files or interpret
//! source-level declarations directly.
//!
//! SCALABILITY
//!
//! This module introduces no artificial limits on:
//!
//! - Number of services.
//! - Number of dependencies.
//! - Number of service versions.
//! - Number of service implementations.
//! - Number of providers.
//! - Number of computational domains.
//! - Number of execution environments.
//! - Number of hardware resources.
//!
//! Dynamic collection capacity is determined by actual requirements,
//! available memory and platform allocation capabilities.
//!
//! Literal infinite computation or infinite physical memory
//! is not promised.
//!
//! POCO-REAF
//!
//! Service dependency planning is independent of any particular
//! computational domain or hardware architecture.
//!
//! Classical, quantum, HDL, AI, tensor, hybrid, distributed,
//! photonic and future-domain services can use the same
//! dependency-planning infrastructure.
//!
//! Adding a new computational domain must not require modifying
//! this module.
//!
//! SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No unsafe functions.
//! No unsafe implementations.
//! No global mutable state.
//! No implicit service activation.
//! No fabricated service availability.
//! No arbitrary hardware ceilings.
//!
//! INTEGRATION REQUIREMENTS
//!
//! The parent `crate::zur` module must declare:
//!
//!     pub mod services;
//!
//! The existing `services/dependencies.rs` must remain the
//! implementation owner.
//!
//! `core/service_registry.rs` must remain the registration
//! and dependency-resolution authority.
//!
//! `core/service_manager.rs` must remain the activation
//! and service-lifecycle authority.
//!
//! PUBLIC API
//!
//! The following types are re-exported without creating
//! duplicate implementations:
//!
//! - ResolvedDependency
//! - DependencyPlanEntry
//! - ServiceDependencyPlan
//! - ServiceDependencyResolver
//!
//! This module intentionally contains no additional runtime state.

// ============================================================================
// SERVICE DEPENDENCY MODULE
// ============================================================================

/// Service dependency planning and validation.
///
/// Owns immutable dependency-plan representations and the
/// adapter between the canonical service registry and
/// service-management infrastructure.
///
/// Does not register or activate services.
pub mod dependencies;

// ============================================================================
// PUBLIC SERVICE DEPENDENCY API
// ============================================================================

/// A resolved dependency relationship belonging to a service.
///
/// Preserves the original dependency declaration and the
/// selected compatible service registration, when available.
pub use dependencies::ResolvedDependency;

/// Dependency information for one registered service.
///
/// Preserves the relationship between a service and
/// its own declared dependencies.
pub use dependencies::DependencyPlanEntry;

/// Immutable, dependency-first service initialization plan.
///
/// Represents registry resolution results without performing
/// service initialization or physical resource reservation.
pub use dependencies::ServiceDependencyPlan;

/// Stateless adapter for constructing service dependency plans.
///
/// Delegates service and version resolution to the canonical
/// ServiceRegistry.
pub use dependencies::ServiceDependencyResolver;
