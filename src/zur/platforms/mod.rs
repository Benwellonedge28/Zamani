
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Platform Integration
//!
//! Path: src/zur/platforms/mod.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! # Purpose
//!
//! This module is the public composition boundary for ZUR platform
//! integration.
//!
//! It connects three independently owned platform components:
//!
//! - `host`: Platform and host-environment descriptions.
//! - `provider`: Explicit platform observation through providers.
//! - `compatibility`: Compatibility evaluation using canonical
//!   backend target and capability contracts.
//!
//! This module exposes those components through one stable module path.
//!
//! It deliberately contains no platform discovery, hardware selection,
//! resource allocation, operating-system calls, executable loading,
//! scheduling, or execution implementation.
//!
//! # Architectural ownership
//!
//! | Responsibility | Authoritative owner |
//! |---|---|
//! | Host environment descriptions | `platforms::host` |
//! | Platform provider observation | `platforms::provider` |
//! | Platform compatibility evaluation | `platforms::compatibility` |
//! | Provider identity and lifecycle | `stdlib::provider` |
//! | Operation context and cancellation | `stdlib::context` |
//! | Execution-provider contracts | `contracts::provider` |
//! | Runtime operational errors | `zur::error` |
//! | Runtime context | `zur::core::context` |
//! | Backend target descriptions | `backend::target` |
//! | Backend capability models | `backend::capabilities` |
//! | Source-language resource intent | Grammar and semantic authorities |
//! | Hardware discovery and realization | Appropriate platform/HAL providers |
//!
//! # Fundamental distinctions
//!
//! The platform integration MUST preserve these distinctions:
//!
//! - Platform identity != physical device identity.
//! - Host description != backend target.
//! - Host facility != runtime service.
//! - Platform provider != execution provider.
//! - Provider registration != provider initialization.
//! - Provider initialization != provider availability.
//! - Platform observation != hardware discovery.
//! - Platform observation != capability verification.
//! - Capability != availability.
//! - Availability != feasibility.
//! - Feasibility != resource reservation.
//! - Resource reservation != execution.
//! - Execution submission != successful completion.
//!
//! An observed host environment is descriptive evidence.
//! It does not independently establish that a workload is executable.
//!
//! # Open-world extensibility
//!
//! Platform identities, host facilities, execution interfaces,
//! architectures, operating systems, ABIs and platform attributes
//! remain extensible.
//!
//! This module MUST NOT introduce closed inventories of:
//!
//! - Operating systems.
//! - Processor architectures.
//! - CPU models.
//! - GPU models.
//! - FPGA models.
//! - QPU models.
//! - Photonic platforms.
//! - Computational domains.
//! - Device vendors.
//! - Hardware resource counts.
//!
//! Future platforms must integrate through the existing provider
//! and host-environment contracts.
//!
//! Adding a new platform must not require modifying this composition
//! module when the existing contracts are sufficient.
//!
//! # Scalability
//!
//! No artificial maximum is imposed on:
//!
//! - Platform providers.
//! - Host facilities.
//! - Execution interfaces.
//! - Platform attributes.
//! - Platform identities.
//! - Computational domains.
//! - Devices.
//! - Resources.
//!
//! The underlying collections, address space, implementation and
//! available physical resources remain finite.
//!
//! POCO-REAF means portable program intent and adaptable execution,
//! not literal infinite physical computation.
//!
//! # Safety
//!
//! This module and its public integration boundary use safe Rust.
//!
//! No unsafe blocks.
//! No unsafe functions.
//! No unsafe implementations.
//! No raw pointers.
//! No global mutable state.
//! No implicit initialization.
//! No implicit hardware discovery.
//! No fabricated availability.
//! No implicit hardware selection.
//!
//! # Grammar integration
//!
//! Relevant source-language authorities include:
//!
//! - `grammar/hybrid/host-device.g4`
//! - `grammar/execution/runtime-capabilities.g4`
//! - `grammar/spec/resources.md`
//! - `grammar/hardware/`
//! - `grammar/resources/`
//!
//! The grammar and semantic layer express program intent.
//!
//! The platform layer consumes validated representations.
//!
//! In particular, `@host` and `@device` represent semantic execution
//! roles. They must not be interpreted as physical platform identities.
//!
//! Runtime capability requirements do not authorize platform discovery,
//! device selection, allocation, reservation or execution.
//!
//! # Integration direction
//!
//! ```text
//! Source grammar
//!       |
//!       v
//! Frontend and semantic authority
//!       |
//!       v
//! Canonical requirements and capabilities
//!       |
//!       v
//! Compiler and backend target descriptions
//!       |
//!       v
//! ZUR platform integration
//!       |
//!       +----------------------+
//!       |                      |
//!       v                      v
//!   Host description      Compatibility
//!       |                      |
//!       v                      v
//!   Platform provider     Compatibility report
//!       |                      |
//!       +-----------+----------+
//!                   |
//!                   v
//!           ZUR execution layer
//!                   |
//!                   v
//!        Appropriate execution provider
//!                   |
//!                   v
//!          HAL and target realization
//! ```
//!
//! This module is a composition boundary, not an additional execution
//! or hardware-abstraction subsystem.
//!
//! # Downstream integration
//!
//! Consumers should use:
//!
//! - `zur::platforms::host` for environment descriptions.
//! - `zur::platforms::provider` for platform observations.
//! - `zur::platforms::compatibility` for compatibility evaluation.
//!
//! Runtime context receives platform information through explicit
//! dependency injection.
//!
//! Execution loading and dispatch consume validated compatibility
//! evidence without transferring platform ownership to the executor.

// =============================================================================
// PLATFORM SUBMODULES
// =============================================================================

/// Platform compatibility evaluation.
///
/// Compares an explicitly supplied host environment with canonical
/// backend target descriptions and compatibility requirements.
///
/// This module does not select targets or reserve resources.
pub mod compatibility;

/// Host-platform and execution-environment descriptions.
///
/// Owns the canonical ZUR representation of descriptive host information.
///
/// This module does not perform operating-system calls or discover hardware.
pub mod host;

/// Platform-provider integration.
///
/// Obtains explicitly supplied platform observations through providers
/// implementing the repository's canonical provider lifecycle.
///
/// This module does not own provider registration or discovery.
pub mod provider;

// =============================================================================
// STABLE PUBLIC HOST API
// =============================================================================

/// Host-platform identity and environment descriptions.
pub use host::{
    ExecutionInterfaceDescriptor,
    ExecutionInterfaceId,
    HostAttributeKey,
    HostAvailability,
    HostEnvironment,
    HostEnvironmentBuilder,
    HostFacilityDescriptor,
    HostFacilityId,
    HostPlatformDescriptor,
    HostPlatformId,
    HostProcessDescriptor,
};

// =============================================================================
// STABLE PUBLIC PROVIDER API
// =============================================================================

/// Explicit platform observation and provider integration.
pub use provider::{
    observe_platform,
    platform_provider_descriptor,
    platform_provider_id,
    platform_provider_status,
    PlatformObservation,
    PlatformProvider,
};

// =============================================================================
// COMPOSITION INVARIANTS
// =============================================================================

/// Returns the architectural identity of this ZUR component.
///
/// This is descriptive metadata, not a provider identity,
/// platform identity or execution capability.
#[must_use]
pub const fn component_name() -> &'static str {
    "zamani.zur.platforms"
}

/// Returns the ZUR platform integration contract generation.
///
/// This identifies the module's integration generation, not the
/// language version, runtime ABI version, platform ABI version,
/// or backend target version.
#[must_use]
pub const fn integration_generation() -> u32 {
    1
}

// =============================================================================
// COMPOSITION TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_platform_component_identity() {
        assert_eq!(
            component_name(),
            "zamani.zur.platforms"
        );
    }

    #[test]
    fn exposes_integration_generation() {
        assert_eq!(
            integration_generation(),
            1
        );
    }

    #[test]
    fn exposes_host_environment_through_stable_api() {
        let platform = HostPlatformDescriptor::from_id(
            "org.zamani.test-platform"
        )
        .expect("valid platform identity");

        let environment = HostEnvironment::builder(platform)
            .build()
            .expect("valid host environment");

        assert_eq!(
            environment.platform().id().as_str(),
            "org.zamani.test-platform"
        );
    }

    #[test]
    fn preserves_unknown_platform_information() {
        let platform = HostPlatformDescriptor::from_id(
            "future.embedded-platform"
        )
        .expect("valid platform identity");

        let environment = HostEnvironment::builder(platform)
            .build()
            .expect("valid host environment");

        assert_eq!(
            environment.platform().architecture(),
            None
        );

        assert_eq!(
            environment.platform().operating_system(),
            None
        );

        assert_eq!(
            environment.platform().abi(),
            None
        );
    }

    #[test]
    fn supports_future_execution_interfaces() {
        let platform = HostPlatformDescriptor::from_id(
            "future.photonic-platform"
        )
        .expect("valid platform identity");

        let interface_id = ExecutionInterfaceId::new(
            "future.photonics.execution"
        )
        .expect("valid interface identity");

        let interface = ExecutionInterfaceDescriptor::new(
            interface_id.clone(),
            HostAvailability::Unknown
        );

        let environment = HostEnvironment::builder(platform)
            .with_execution_interface(interface)
            .expect("interface registration")
            .build()
            .expect("host environment");

        assert_eq!(
            environment.execution_interface_count(),
            1
        );

        assert!(
            !environment.reports_interface_available(
                &interface_id
            )
        );
    }

    #[test]
    fn does_not_treat_unknown_as_available() {
        let platform = HostPlatformDescriptor::from_id(
            "future.test-platform"
        )
        .expect("valid platform identity");

        let environment = HostEnvironment::builder(platform)
            .build()
            .expect("host environment");

        let interface_id = ExecutionInterfaceId::new(
            "future.execution.interface"
        )
        .expect("valid interface identity");

        assert!(
            !environment.reports_interface_available(
                &interface_id
            )
        );
    }
}
