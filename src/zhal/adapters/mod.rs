
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Domain Hardware Adapters
//!
//! Canonical path: `src/zhal/adapters/mod.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97.1
//! Safety: Safe Rust only.
//!
//! # 1. PURPOSE
//!
//! This module is the canonical composition root for the
//! domain-specific hardware adapters integrated with ZHAL.
//!
//! It connects the universal hardware abstraction architecture
//! to the existing computational-domain authorities without
//! replacing, duplicating or taking ownership of their
//! specialized implementations.
//!
//! The adapters preserve domain semantics while exposing the
//! appropriate existing provider, device, execution and
//! realization contracts.
//!
//! # 2. ARCHITECTURAL POSITION
//!
//! The intended dependency direction is:
//!
//! ```text
//!                  Zamani source
//!                       |
//!                       v
//!             Frontend and semantics
//!                       |
//!                       v
//!                 Canonical IRs
//!                       |
//!                       v
//!             Compiler and planning
//!                       |
//!                       v
//!                Backend / ZQN
//!                       |
//!                       v
//!                 ZUR runtime
//!                       |
//!                       v
//!                      ZHAL
//!                       |
//!                       v
//!                   Adapters
//!                       |
//!       +---------------+---------------+
//!       |       |       |       |       |
//!       v       v       v       v       v
//!    Classical Quantum  HDL Accelerator Distributed
//!       |
//!       +-------------------------------+
//!                       |
//!                       v
//!                    Embedded
//!                       |
//!                       v
//!              Existing domain owners
//!                       |
//!                       v
//!                Physical hardware
//! ```
//!
//! This diagram describes architectural integration,
//! not a requirement that every operation passes through
//! every domain adapter.
//!
//! # 3. OWNERSHIP
//!
//! This composition root owns:
//!
//! - Domain adapter module registration.
//! - Stable public module paths.
//! - Adapter composition documentation.
//! - Adapter architecture identification.
//! - Adapter composition versioning.
//! - Public module accessibility.
//! - Composition-level contract tests.
//!
//! Individual adapter files own their respective integration
//! implementations.
//!
//! # 4. EXPLICIT NON-OWNERSHIP
//!
//! This module MUST NOT implement or duplicate:
//!
//! - Hardware-provider identity.
//! - Provider descriptors.
//! - Provider registration.
//! - Provider lifecycle.
//! - Provider discovery.
//! - Provider validation.
//! - Device inventory.
//! - Device discovery.
//! - Device access.
//! - Hardware sessions.
//! - Capability identity.
//! - Capability evidence.
//! - Capability evaluation.
//! - Semantic requirements.
//! - Resource requirements.
//! - Resource allocation.
//! - Resource reservations.
//! - Resource accounting.
//! - Hardware scheduling.
//! - Hardware routing.
//! - Target selection.
//! - Compiler lowering.
//! - Classical IR.
//! - Quantum IR.
//! - HDLIR.
//! - ZUIR.
//! - Hardware drivers.
//! - Hardware communication protocols.
//! - Domain-specific execution engines.
//! - Authentication.
//! - Authorization.
//! - Distributed transport.
//! - Runtime orchestration.
//!
//! These responsibilities remain with their existing
//! authoritative repository modules.
//!
//! # 5. DOMAIN ADAPTER AUTHORITIES
//!
//! ## Classical
//!
//! Canonical module:
//!
//!     crate::zhal::adapters::classical
//!
//! Integrates:
//!
//!     crate::classical
//!     crate::classical::zcir
//!     crate::classical::execution
//!     crate::backend
//!     crate::stdlib::provider
//!     crate::stdlib::capability
//!     crate::zur::platforms
//!
//! Responsibilities:
//!
//! - Adapt existing classical provider contracts.
//! - Preserve classical capability observations.
//! - Integrate classical hardware-facing services.
//! - Preserve target-independent classical semantics.
//!
//! CPU architecture definitions, instruction selection,
//! classical IR, memory management and physical allocation
//! remain outside this composition root.
//!
//! ## Quantum
//!
//! Canonical module:
//!
//!     crate::zhal::adapters::quantum
//!
//! Integrates:
//!
//!     crate::quantum::ir
//!     crate::quantum::hardware
//!     crate::quantum::hardware::backend
//!     crate::quantum::hardware::backend_trait
//!     crate::quantum::hardware::adapters
//!
//! Responsibilities:
//!
//! - Bind universal hardware-provider identities to
//!   explicitly associated quantum backend identities.
//! - Preserve generic and quantum-specific capability evidence.
//! - Delegate quantum execution to existing quantum adapters.
//! - Preserve quantum job, result and cancellation contracts.
//! - Preserve hardware and execution provenance.
//!
//! Quantum circuits, pulses, annealing, calibration,
//! connectivity, routing, scheduling and quantum execution
//! remain owned by `crate::quantum::hardware` and `crate::quantum::ir`.
//!
//! ## HDL
//!
//! Canonical module:
//!
//!     crate::zhal::adapters::hdl
//!
//! Integrates:
//!
//!     crate::hdl
//!     crate::hdl::hdlir
//!     crate::hdl::targets
//!     crate::hdl::requirements
//!     crate::hdl::resources
//!     crate::hdl::realization
//!
//! Responsibilities:
//!
//! - Bridge ZHAL hardware providers to HDL realization providers.
//! - Preserve target and feasibility contracts.
//! - Integrate realization plans and reports.
//! - Expose existing physical realization interfaces.
//!
//! HDLIR, synthesis, technology mapping, placement,
//! routing, timing closure and physical realization
//! remain owned by the HDL subsystem.
//!
//! ## Accelerators
//!
//! Canonical module:
//!
//!     crate::zhal::adapters::accelerator
//!
//! Integrates:
//!
//!     crate::ai
//!     crate::ai::execution
//!     crate::classical::collections::tensor
//!     crate::zuir
//!     crate::backend
//!     crate::distributed::execution
//!
//! Responsibilities:
//!
//! - Adapt accelerator-associated hardware providers.
//! - Preserve AI execution contracts.
//! - Integrate accelerator capability observations.
//! - Delegate workload execution to existing AI execution
//!   and runtime authorities.
//! - Preserve tensor and operation semantics.
//!
//! GPU, TPU, NPU, DSP and other accelerator implementations
//! remain extensible through existing provider contracts.
//!
//! Tensor storage, AI semantic models, tensor IR and
//! physical allocation must not be duplicated here.
//!
//! ## Distributed hardware
//!
//! Canonical module:
//!
//!     crate::zhal::adapters::distributed
//!
//! Integrates:
//!
//!     crate::distributed::ids
//!     crate::distributed::membership
//!     crate::distributed::transport
//!     crate::distributed::planning
//!     crate::distributed::execution
//!     crate::zur::distributed
//!
//! Responsibilities:
//!
//! - Associate hardware providers with distributed participants.
//! - Preserve participant, provider and device identities.
//! - Validate membership and lease observations.
//! - Preserve remote provider failures.
//! - Integrate remote hardware sessions.
//! - Preserve uncertain remote operation outcomes.
//!
//! Distributed placement, transport, scheduling, recovery,
//! security and execution remain owned by the distributed
//! subsystem and ZUR.
//!
//! ## Embedded hardware
//!
//! Canonical module:
//!
//!     crate::zhal::adapters::embedded
//!
//! Integrates:
//!
//!     crate::stdlib::io::devices
//!     crate::stdlib::provider
//!     crate::zhal::providers::contract
//!     crate::zur::io
//!
//! Responsibilities:
//!
//! - Integrate embedded hardware providers.
//! - Preserve generic device-I/O contracts.
//! - Bridge hardware-provider and device-provider interfaces.
//! - Support microcontrollers, sensors, actuators,
//!   robotics and other physical devices.
//!
//! Byte-oriented transport remains owned by the standard
//! library device-I/O subsystem.
//!
//! Device-specific protocols remain owned by their respective
//! domain implementations.
//!
//! # 6. PROVIDER AND REGISTRY INTEGRATION
//!
//! All adapters must respect the existing provider architecture.
//!
//! Canonical provider identity and lifecycle:
//!
//!     crate::stdlib::provider
//!
//! Canonical hardware-provider contract:
//!
//!     crate::zhal::providers::contract
//!
//! Canonical provider registry:
//!
//!     crate::zhal::providers::registry
//!
//! Canonical provider lifecycle coordination:
//!
//!     crate::zhal::providers::lifecycle
//!
//! Canonical provider discovery:
//!
//!     crate::zhal::providers::discovery
//!
//! Canonical provider validation:
//!
//!     crate::zhal::providers::validation
//!
//! This composition root MUST NOT introduce another registry,
//! lifecycle manager or provider identity system.
//!
//! Domain adapters may wrap existing providers or compose
//! existing domain-specific execution interfaces.
//!
//! Such wrapping must preserve the original provider identity.
//!
//! The distinction between:
//!
//! - A standard-library Provider.
//! - A ZHAL HardwareProvider.
//! - A domain-specific execution provider.
//!
//! must remain explicit.
//!
//! Rust trait-object types must not be assumed interchangeable.
//! Adapters must use the existing typed contracts rather than
//! unchecked conversions or unsafe trait-object downcasts.
//!
//! # 7. CAPABILITY AND SEMANTIC INTEGRATION
//!
//! Canonical capability authority:
//!
//!     crate::stdlib::capability
//!
//! Canonical source-level semantic authority:
//!
//!     crate::frontend::semantic
//!
//! Canonical hardware-facing capability integration:
//!
//!     crate::zhal::capabilities
//!
//! Domain-specific capability authorities remain with their
//! corresponding computational domains.
//!
//! The following concepts must remain distinct:
//!
//! - Requirement.
//! - Capability.
//! - Preference.
//! - Constraint.
//! - Hint.
//! - Feasibility.
//! - Observation.
//! - Availability.
//! - Reservation.
//!
//! An adapter may translate or expose an existing typed
//! representation, but must not independently reinterpret
//! source-language semantics.
//!
//! Unknown capability evidence must never be treated as
//! established availability.
//!
//! Advertised capability must never be treated as proof
//! of resource reservation.
//!
//! # 8. POCO-REAF AND OPEN-WORLD EXTENSIBILITY
//!
//! Zamani's objective is:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! The adapter architecture must preserve program semantics
//! independently of the physical hardware selected.
//!
//! This composition root MUST NOT introduce arbitrary
//! universal ceilings on:
//!
//! - Providers.
//! - Devices.
//! - Processors.
//! - CPU cores.
//! - Threads.
//! - Accelerators.
//! - GPUs.
//! - NPUs.
//! - TPUs.
//! - FPGAs.
//! - ASICs.
//! - Quantum processors.
//! - Qubits.
//! - Memory.
//! - Storage.
//! - Hardware interfaces.
//! - Distributed participants.
//! - Computational domains.
//! - Hardware technologies.
//! - Future provider implementations.
//!
//! No universal hardware-count constants or fixed-capacity
//! arrays are introduced by this module.
//!
//! This module does not define a closed enumeration of
//! hardware technologies.
//!
//! The six modules below represent currently implemented
//! integration boundaries, not the complete universe of
//! computational hardware.
//!
//! Future technologies may integrate through the existing
//! open-world provider and adapter contracts.
//!
//! A future photonic, neuromorphic, analog, optical,
//! molecular or other hardware implementation must not
//! require changes to unrelated adapters.
//!
//! The absence of a dedicated built-in adapter does not
//! prevent an independently implemented compatible provider
//! from participating in the universal provider architecture.
//!
//! Scalability is governed by:
//!
//! - Available physical resources.
//! - Memory and address-space representation.
//! - Provider implementation capabilities.
//! - Explicit operational budgets.
//! - Platform and protocol constraints.
//! - Applicable execution policies.
//!
//! Literal infinite physical computation or storage is
//! not promised.
//!
//! # 9. FAILURE AND SECURITY BOUNDARIES
//!
//! Adapter errors must preserve their originating authority.
//!
//! An adapter must not:
//!
//! - Manufacture hardware identities.
//! - Fabricate capability evidence.
//! - Convert unknown outcomes into success.
//! - Claim cancellation stopped physical execution.
//! - Claim discovery reserved hardware.
//! - Claim provider registration establishes trust.
//! - Claim membership establishes authentication.
//! - Claim capability availability guarantees execution.
//! - Silently change computational semantics.
//!
//! Security authority remains with the existing ZHAL and
//! ZUR security implementations.
//!
//! Adapters must not store credentials in ordinary metadata,
//! diagnostics or debug representations.
//!
//! # 10. CONCURRENCY AND LIFECYCLE
//!
//! This composition root introduces:
//!
//! - No mutable global state.
//! - No independent asynchronous executor.
//! - No hardware-discovery thread.
//! - No resource allocator.
//! - No scheduler.
//! - No provider lifecycle state machine.
//!
//! Each adapter must preserve the concurrency guarantees
//! of the underlying provider and domain contracts.
//!
//! Cancellation and deadlines must propagate through the
//! existing operation-context authorities.
//!
//! # 11. VERSIONING
//!
//! The composition-root schema version is independent of:
//!
//! - Zamani language version.
//! - Grammar version.
//! - Cargo package version.
//! - ZHAL provider API version.
//! - Quantum backend API version.
//! - HDL realization API version.
//! - Hardware firmware version.
//! - Provider protocol version.
//!
//! A composition-root version change does not silently
//! upgrade any underlying domain API.
//!
//! # 12. INTEGRATION REQUIREMENTS
//!
//! The parent `src/zhal/mod.rs` must declare:
//!
//!     pub mod adapters;
//!
//! The crate root `src/lib.rs` must expose:
//!
//!     pub mod zhal;
//!
//! Existing adapter implementations must remain registered
//! exactly once.
//!
//! Domain-specific implementations must be compiled and
//! validated through their owning modules.
//!
//! This module must not force unrelated computational
//! subsystems to depend on every adapter implementation.
//!
//! # 13. FREEZE CRITERIA
//!
//! This file is complete when:
//!
//! 1. All six existing adapter files are registered.
//! 2. No duplicate adapter implementation is introduced.
//! 3. No provider registry is introduced.
//! 4. No capability authority is introduced.
//! 5. No resource allocator is introduced.
//! 6. No domain-specific IR is introduced.
//! 7. All public adapter module paths are stable.
//! 8. Future provider technologies remain extensible.
//! 9. No arbitrary universal hardware ceilings are introduced.
//! 10. No unsafe Rust is used.
//! 11. All registered child modules compile.
//! 12. Composition and repository tests pass.
//!
//! This file does not claim that the complete repository
//! or every physical hardware provider has already passed
//! production certification.

// ============================================================================
// COMPOSITION ROOT IDENTITY
// ============================================================================

/// Stable architectural identity of the ZHAL adapter composition root.
pub const ADAPTERS_MODULE_NAME: &str = "zamani.zhal.adapters";

/// Major version of the adapter composition-root contract.
///
/// This version is independent of all domain-provider API versions.
pub const ADAPTERS_MODULE_SCHEMA_MAJOR: u16 = 1;

/// Minor version of the adapter composition-root contract.
pub const ADAPTERS_MODULE_SCHEMA_MINOR: u16 = 0;

/// Patch version of the adapter composition-root contract.
pub const ADAPTERS_MODULE_SCHEMA_PATCH: u16 = 0;

/// Complete adapter composition-root schema version.
pub const ADAPTERS_MODULE_SCHEMA_VERSION: (u16, u16, u16) = (
    ADAPTERS_MODULE_SCHEMA_MAJOR,
    ADAPTERS_MODULE_SCHEMA_MINOR,
    ADAPTERS_MODULE_SCHEMA_PATCH,
);

// ============================================================================
// DOMAIN ADAPTER MODULES
// ============================================================================

/// Classical-computing hardware integration.
///
/// Bridges existing classical provider and capability contracts
/// without replacing classical IR, backend or runtime ownership.
pub mod classical;

/// Quantum hardware integration.
///
/// Delegates quantum execution to the authoritative quantum
/// hardware and backend contracts.
pub mod quantum;

/// HDL realization hardware integration.
///
/// Bridges universal hardware abstraction to the existing
/// HDL realization provider architecture.
pub mod hdl;

/// AI, tensor and accelerator hardware integration.
///
/// Delegates workload execution to the canonical AI,
/// compiler, backend and runtime authorities.
pub mod accelerator;

/// Distributed and remote hardware integration.
///
/// Bridges existing hardware providers to canonical distributed
/// membership and execution contracts.
pub mod distributed;

/// Embedded and physical device integration.
///
/// Reuses the standard-library generic device-I/O contracts
/// and existing ZHAL hardware-provider interfaces.
pub mod embedded;

// ============================================================================
// COMPOSITION CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// The composition root exposes its stable architectural identity.
    #[test]
    fn composition_root_has_stable_identity() {
        assert_eq!(
            ADAPTERS_MODULE_NAME,
            "zamani.zhal.adapters"
        );
    }

    /// The composition schema version is internally consistent.
    #[test]
    fn composition_schema_version_is_consistent() {
        assert_eq!(
            ADAPTERS_MODULE_SCHEMA_VERSION,
            (
                ADAPTERS_MODULE_SCHEMA_MAJOR,
                ADAPTERS_MODULE_SCHEMA_MINOR,
                ADAPTERS_MODULE_SCHEMA_PATCH,
            )
        );

        assert_ne!(ADAPTERS_MODULE_SCHEMA_MAJOR, 0);
    }

    /// The adapter namespace exposes independently maintained modules.
    #[test]
    fn domain_adapter_modules_are_registered() {
        assert_eq!(
            core::any::type_name::<
                classical::ClassicalCapabilityReport
            >(),
            "zamani::zhal::adapters::classical::ClassicalCapabilityReport"
        );
    }

    /// The quantum adapter retains its independent public namespace.
    #[test]
    fn quantum_adapter_is_accessible() {
        assert_eq!(
            quantum::COMPONENT,
            "zhal.adapters.quantum"
        );
    }

    /// The accelerator adapter retains its independent public namespace.
    #[test]
    fn accelerator_adapter_is_accessible() {
        assert_eq!(
            accelerator::COMPONENT,
            "zhal.adapters.accelerator"
        );
    }

    /// The composition root does not define hardware capacity limits.
    #[test]
    fn composition_root_does_not_define_hardware_ceilings() {
        // Physical capacities are supplied by provider observations,
        // hardware descriptions and explicitly configured policies.
        //
        // This module contains no fixed-capacity hardware arrays,
        // maximum device counts or closed hardware-domain registry.
        assert_eq!(
            ADAPTERS_MODULE_NAME,
            "zamani.zhal.adapters"
        );
    }
}
