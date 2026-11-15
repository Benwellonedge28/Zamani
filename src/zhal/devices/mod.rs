
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Universal Hardware Devices
//!
//! Path: src/zhal/devices/mod.rs
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module is the canonical composition root for universal hardware
//! device discovery, inventory, access, sessions and health within Zamani.
//!
//! It provides the stable public device-management boundary of ZHAL.
//!
//! It composes independently implemented device subsystems without
//! duplicating their implementation or ownership.
//!
//! Supported computational hardware is open-ended and includes:
//!
//! - Classical processors and memory systems.
//! - Quantum processors and quantum control systems.
//! - Quantum simulators and emulators.
//! - FPGA and ASIC devices.
//! - GPUs, TPUs, NPUs and DSPs.
//! - AI and tensor accelerators.
//! - Embedded processors and microcontrollers.
//! - Sensors and actuators.
//! - Distributed and remotely accessible hardware.
//! - Hybrid computational systems.
//! - Neuromorphic hardware.
//! - Photonic and optical computing.
//! - Analog and in-memory computing.
//! - Future computational technologies.
//!
//! No closed hardware-category enumeration is imposed here.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! THIS MODULE OWNS:
//!
//! - Device subsystem composition.
//! - Public device-module visibility.
//! - Stable module paths.
//! - Device API organization.
//! - Device subsystem documentation.
//! - Device subsystem architectural boundaries.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - Provider identity.
//! - Hardware device identity.
//! - Hardware device descriptors.
//! - Hardware capability semantics.
//! - Hardware resource allocation.
//! - Hardware resource reservations.
//! - Provider registration.
//! - Provider lifecycle.
//! - Provider discovery coordination.
//! - Device discovery algorithms.
//! - Device inventory storage.
//! - Device access authorization.
//! - Device communication.
//! - Hardware session implementation.
//! - Hardware health observation.
//! - Quantum hardware execution.
//! - HDL realization.
//! - Distributed execution.
//! - Hybrid execution.
//! - Runtime orchestration.
//! - Hardware scheduling.
//! - Hardware routing.
//! - Source-language semantic analysis.
//!
//! Those responsibilities belong to their respective canonical modules.
//!
//! ============================================================================
//! SUBSYSTEM COMPOSITION
//! ============================================================================
//!
//! The device subsystem consists of five independently owned components.
//!
//! 1. discovery
//!
//!    Observes hardware devices through the provider-discovery authority.
//!
//!    Discovery is observational. It does not authorize access, allocate
//!    resources, reserve hardware or create device sessions.
//!
//! 2. inventory
//!
//!    Maintains validated, versioned observations of discovered devices.
//!
//!    It provides deterministic queries and inventory snapshots.
//!
//!    It does not replace quantum-specific or domain-specific registries.
//!
//! 3. access
//!
//!    Coordinates explicit hardware-device access requests.
//!
//!    It integrates the existing authorization and hardware-provider
//!    authorities.
//!
//!    Discovery alone never establishes permission to access hardware.
//!
//! 4. sessions
//!
//!    Coordinates provider-managed hardware-session ownership and lifecycle.
//!
//!    A session does not automatically represent a resource reservation.
//!
//! 5. health
//!
//!    Provides provider-independent hardware health observations.
//!
//!    It does not replace quantum-specific health or distributed
//!    participant-health authorities.
//!
//! ============================================================================
//! CANONICAL REPOSITORY AUTHORITIES
//! ============================================================================
//!
//! Provider identity:
//!     crate::stdlib::provider
//!
//! Generic device identity and device categories:
//!     crate::stdlib::io::devices
//!
//! Hardware device identity:
//!     crate::zhal::identity
//!
//! Hardware device descriptors:
//!     crate::zhal::model::device
//!
//! Hardware provider contracts:
//!     crate::zhal::providers::contract
//!
//! Hardware provider registration:
//!     crate::zhal::providers::registry
//!
//! Provider discovery:
//!     crate::zhal::providers::discovery
//!
//! Hardware capability contracts:
//!     crate::stdlib::capability
//!
//! Hardware operation context:
//!     crate::zhal::context
//!
//! Hardware error contracts:
//!     crate::zhal::errors
//!
//! Hardware device discovery:
//!     crate::zhal::devices::discovery
//!
//! Hardware device inventory:
//!     crate::zhal::devices::inventory
//!
//! Hardware device access:
//!     crate::zhal::devices::access
//!
//! Hardware device sessions:
//!     crate::zhal::devices::sessions
//!
//! Hardware device health:
//!     crate::zhal::devices::health
//!
//! Runtime resource management:
//!     crate::zur
//!
//! ============================================================================
//! DOMAIN INTEGRATION
//! ============================================================================
//!
//! This module provides a domain-independent composition boundary.
//!
//! Existing domain-specific hardware implementations remain authoritative.
//!
//! Classical hardware:
//!     crate::classical
//!
//! Quantum hardware:
//!     crate::quantum::hardware
//!
//! HDL and programmable hardware:
//!     crate::hdl::realization
//!
//! Accelerator hardware:
//!     crate::backend
//!
//! Distributed hardware:
//!     crate::distributed
//!
//! Hybrid computing:
//!     crate::hybrid
//!
//! Host platform observations:
//!     crate::zur::platforms
//!
//! Generic device communication:
//!     crate::stdlib::io::devices
//!
//! Domain adapters must translate between these existing contracts
//! and ZHAL. This module must not duplicate their implementations.
//!
//! ============================================================================
//! GRAMMAR BOUNDARY
//! ============================================================================
//!
//! Relevant grammar components:
//!
//! - grammar/hardware/devices.g4
//! - grammar/hardware/hardware.g4
//! - grammar/effects/hardware.g4
//! - grammar/execution/execution-context.g4
//! - grammar/execution/distributed-execution.g4
//!
//! Relevant specifications:
//!
//! - grammar/specification/semantic-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/poco-reaf.md
//! - grammar/validation/semantic-boundaries.md
//!
//! The grammar describes logical hardware declarations and computational
//! intent. It does not discover physical hardware.
//!
//! The semantic layer owns:
//!
//! - Requirements.
//! - Capabilities.
//! - Preferences.
//! - Constraints.
//! - Hints.
//! - Feasibility.
//!
//! This device subsystem exposes physical hardware observations and
//! provider-managed device interactions.
//!
//! It must never reinterpret or replace the semantic model.
//!
//! ============================================================================
//! SCALABILITY AND POCO-REAF
//! ============================================================================
//!
//! This composition root imposes no arbitrary universal ceilings on:
//!
//! - Hardware devices.
//! - Hardware providers.
//! - Device categories.
//! - Hardware technologies.
//! - Hardware interfaces.
//! - Hardware sessions.
//! - Hardware observations.
//! - Distributed participants.
//! - Hardware generations.
//!
//! The individual subsystems use incremental operations, provider-backed
//! discovery, versioned observations and resource-aware storage.
//!
//! Operational budgets may be configured by their owning authorities.
//!
//! Such budgets must never become semantic hardware-capacity ceilings.
//!
//! Literal infinite storage, computation or physical capacity is not
//! promised. Scalability is bounded only by available resources,
//! provider capabilities and applicable operational policies.
//!
//! ============================================================================
//! CONCURRENCY
//! ============================================================================
//!
//! Thread-safety and synchronization are implemented by the owning
//! subsystems.
//!
//! This composition root introduces no additional global mutable state,
//! global locks, singleton registries or synchronization authorities.
//!
//! ============================================================================
//! ERROR HANDLING
//! ============================================================================
//!
//! Each subsystem uses the canonical ZHAL error contract.
//!
//! This module does not introduce a competing error hierarchy.
//!
//! Errors must preserve their originating subsystem, relevant hardware
//! identity, provider context and underlying cause where available.
//!
//! ============================================================================
//! PUBLIC API POLICY
//! ============================================================================
//!
//! Each subsystem is exposed through its own stable module path.
//!
//! This deliberately avoids flattening unrelated APIs into one namespace.
//!
//! Callers can explicitly select the functionality they require.
//!
//! Examples:
//!
//!     crate::zhal::devices::discovery
//!     crate::zhal::devices::inventory
//!     crate::zhal::devices::access
//!     crate::zhal::devices::sessions
//!     crate::zhal::devices::health
//!
//! Existing type ownership and module paths are preserved.
//!
//! ============================================================================
//! SAFETY
//! ============================================================================
//!
//! Unsafe Rust is forbidden throughout this module and its descendants.
//!
//! Hardware interaction must occur through safe provider interfaces.
//!
//! No unsafe downcasting, pointer manipulation or hardware-memory access
//! is introduced by this composition root.
//!
//! ============================================================================
//! MODULE DECLARATIONS
//! ============================================================================

/// Hardware-device discovery and incremental observation.
///
/// Provides filtered discovery sessions and provider-backed discovery pages.
///
/// Discovery does not authorize hardware access or allocate resources.
pub mod discovery;

/// Canonical inventory of observed hardware devices.
///
/// Provides validated inventory snapshots, deterministic queries,
/// provider-scoped refresh operations and inventory consistency.
pub mod inventory;

/// Explicit hardware-device access coordination.
///
/// Integrates hardware-provider access with the existing authorization
/// authority and provider-managed device sessions.
pub mod access;

/// Provider-managed hardware-device session lifecycle.
///
/// Provides session ownership, lifecycle coordination and safe session
/// handle management.
pub mod sessions;

/// Provider-independent hardware health observations.
///
/// Provides extensible health checks, health reports, diagnostic evidence
/// and observation-freshness assessment.
pub mod health;

// ============================================================================
// ARCHITECTURAL COMPLETION CONTRACT
// ============================================================================
//
// This file is complete when:
//
// 1. Every device subsystem has one authoritative implementation.
// 2. Every existing device module is exposed through this composition root.
// 3. Existing module paths are preserved.
// 4. No competing provider registry is introduced.
// 5. No competing device inventory is introduced.
// 6. No competing capability model is introduced.
// 7. No competing authorization authority is introduced.
// 8. No competing resource allocator is introduced.
// 9. No competing hardware-session implementation is introduced.
// 10. Domain-specific hardware ownership is preserved.
// 11. Grammar and semantic ownership boundaries are preserved.
// 12. No arbitrary universal hardware ceilings are introduced.
// 13. Unsafe Rust is prohibited.
// 14. The parent ZHAL module exposes this module.
// 15. Repository compilation and test validation succeeds.
//
// Future hardware technologies must be integrated through the appropriate
// provider and adapter contracts, not by modifying this composition root.
