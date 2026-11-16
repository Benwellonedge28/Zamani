
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Simulation
//!
//! Canonical path: `src/zhal/simulation/mod.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # 1. PURPOSE
//!
//! This module is the canonical public composition root for
//! ZHAL hardware simulation.
//!
//! It composes:
//!
//! - The configurable simulated hardware provider.
//! - The simulated hardware inventory.
//! - Simulation-specific fault injection.
//! - Simulated operation outcome reporting.
//! - Hardware-provider conformance testing.
//!
//! It provides one stable public API for simulation functionality
//! without introducing another hardware abstraction system.
//!
//! # 2. ARCHITECTURAL POSITION
//!
//! ```text
//!                 Zamani source
//!                       |
//!                       v
//!               Frontend / Semantic
//!                       |
//!                       v
//!                Compiler / Domain IR
//!                       |
//!                       v
//!                 Backend / ZQN
//!                       |
//!                       v
//!                  ZUR Runtime
//!                       |
//!                       v
//!                      ZHAL
//!                       |
//!             +---------+---------+
//!             |                   |
//!             v                   v
//!        Real Providers      Simulation
//!                                 |
//!                       +---------+---------+
//!                       |                   |
//!                       v                   v
//!                   Provider           Conformance
//!                   Simulation          Testing
//!                       |
//!                       v
//!                Simulated Devices
//! ```
//!
//! Simulation is an alternative provider implementation.
//!
//! It implements the same canonical hardware-provider contract
//! as other ZHAL providers.
//!
//! # 3. ARCHITECTURAL AUTHORITIES
//!
//! The following repository modules remain authoritative.
//!
//! Provider identity and lifecycle:
//!     crate::stdlib::provider
//!
//! Capability identity and snapshots:
//!     crate::stdlib::capability
//!
//! Operation context and cancellation:
//!     crate::stdlib::context
//!
//! Hardware-provider contract:
//!     crate::zhal::providers::contract
//!
//! Hardware identity:
//!     crate::zhal::identity
//!
//! Hardware device models:
//!     crate::zhal::model::device
//!
//! Hardware interface models:
//!     crate::zhal::model::interface
//!
//! Hardware resource models:
//!     crate::zhal::model::resource
//!
//! Hardware errors:
//!     crate::zhal::errors
//!
//! Runtime resource management:
//!     crate::zur
//!
//! Domain-specific simulation:
//!     crate::quantum
//!     crate::hdl
//!     crate::hybrid
//!     crate::distributed
//!     crate::ai
//!
//! # 4. OWNERSHIP
//!
//! This module owns:
//!
//! - Simulation module composition.
//! - Stable simulation API paths.
//! - Simulation API re-exports.
//! - Simulation architecture documentation.
//! - Simulation module composition tests.
//!
//! The provider module owns:
//!
//! - SimulationProvider.
//! - SimulationProviderConfig.
//! - SimulatedDevice.
//! - SimulationFault.
//! - SimulationFaultPoint.
//! - SimulatedOutcome.
//! - SimulatedOperationReport.
//! - Simulated provider state.
//! - Simulated device inventory.
//! - Simulated provider sessions.
//! - Simulation-specific fault injection.
//! - Modeled simulation latency.
//!
//! The conformance module owns:
//!
//! - ConformanceOptions.
//! - ConformanceCheckStatus.
//! - ConformanceStatus.
//! - ConformanceFinding.
//! - ConformanceReport.
//! - Provider conformance execution.
//!
//! This module must not duplicate any of those implementations.
//!
//! # 5. WHAT THIS MODULE DOES NOT OWN
//!
//! This module does not implement:
//!
//! - A second hardware-provider trait.
//! - A second provider registry.
//! - Provider identity.
//! - Provider lifecycle.
//! - Hardware discovery infrastructure.
//! - Hardware inventory infrastructure.
//! - Hardware capability semantics.
//! - Semantic resource requirements.
//! - Hardware feasibility.
//! - Physical resource allocation.
//! - Physical resource reservation.
//! - Runtime scheduling.
//! - Quantum simulation algorithms.
//! - Quantum state-vector engines.
//! - Quantum routing.
//! - HDL waveform engines.
//! - HDL event kernels.
//! - HDL synthesis.
//! - Numerical simulation solvers.
//! - Distributed transport.
//! - Hybrid execution.
//! - AI model execution.
//! - Source-language parsing.
//! - AST construction.
//! - ZUIR or domain-specific IR.
//! - Physical hardware execution.
//!
//! Domain-specific simulation engines remain owned by their
//! respective computational subsystems.
//!
//! # 6. SIMULATION AND PHYSICAL EXECUTION
//!
//! Simulation and physical execution are different execution
//! realities.
//!
//! A successful simulated operation:
//!
//! - Describes a simulated scenario.
//! - Does not prove physical hardware availability.
//! - Does not prove physical hardware compatibility.
//! - Does not prove physical resource reservation.
//! - Does not prove physical execution.
//! - Does not constitute a physical measurement.
//!
//! Simulated outcomes must retain their simulated provenance.
//!
//! Unknown and partially completed outcomes must remain distinct
//! from successful completion.
//!
//! # 7. POCO-REAF AND EXTENSIBILITY
//!
//! Simulation must support open-ended hardware descriptions.
//!
//! Applicable computational domains include:
//!
//! - Classical.
//! - Quantum.
//! - HDL.
//! - AI and tensor accelerators.
//! - Hybrid.
//! - Distributed.
//! - Embedded.
//! - Neuromorphic.
//! - Photonic.
//! - Analog.
//! - Future computational technologies.
//!
//! This module must not enumerate or restrict those technologies.
//!
//! It must not impose fixed maximums for:
//!
//! - Simulated devices.
//! - Providers.
//! - CPUs.
//! - GPUs.
//! - NPUs.
//! - FPGAs.
//! - ASICs.
//! - QPUs.
//! - Qubits.
//! - Memory.
//! - Storage.
//! - Interfaces.
//! - Hardware resources.
//! - Computational domains.
//! - Discovery pages.
//! - Simulation operations.
//!
//! The provider supports dynamic inventory growth and paginated
//! discovery through its canonical provider contract.
//!
//! Conformance budgets are optional operational policies.
//! They are not hardware capability ceilings.
//!
//! Actual growth is constrained by available memory, processing
//! capacity, address-space representation and explicit operational
//! budgets.
//!
//! Literal infinite computation or storage is not promised.
//!
//! # 8. GRAMMAR BOUNDARY
//!
//! This module must remain consistent with:
//!
//! - grammar/execution/simulation.g4
//! - grammar/policies/simulation.g4
//! - grammar/hdl/simulation.g4
//! - grammar/effects/simulation.g4
//! - grammar/resources/requirements.g4
//! - grammar/types/resource.g4
//!
//! These grammars describe source-level intent and semantic
//! composition.
//!
//! This module exposes runtime simulation infrastructure.
//!
//! No grammar parser or semantic evaluator belongs here.
//!
//! # 9. THREAD SAFETY
//!
//! Thread-safety guarantees are provided by the underlying
//! simulation provider implementation.
//!
//! This composition module introduces:
//!
//! - No mutable global state.
//! - No global simulator singleton.
//! - No global device inventory.
//! - No global random-number generator.
//! - No background execution threads.
//! - No hidden initialization.
//!
//! Provider instances are independently constructed.
//!
//! # 10. PUBLIC API STABILITY
//!
//! Canonical implementation paths remain available:
//!
//!     crate::zhal::simulation::provider
//!     crate::zhal::simulation::conformance
//!
//! Convenience re-exports are provided at:
//!
//!     crate::zhal::simulation
//!
//! The implementation modules remain the authoritative
//! definitions of their respective public types.

// ============================================================================
// COMPONENT MODULES
// ============================================================================

/// Configurable simulated hardware provider.
///
/// Implements the canonical standard-library provider contracts
/// and the ZHAL hardware-provider interface.
///
/// Supports configurable device inventories, capabilities,
/// resource observations, availability, modeled latency and
/// simulation-specific fault injection.
///
/// Simulation does not imply physical hardware execution.
pub mod provider;

/// Reusable hardware-provider conformance testing.
///
/// Provides asynchronous conformance checks for implementations
/// of the canonical ZHAL HardwareProvider contract.
///
/// Session testing is optional because enabling it against a
/// physical provider may cause external effects.
///
/// Conformance testing does not allocate resources, select
/// hardware or guarantee future provider availability.
pub mod conformance;

// ============================================================================
// SIMULATED PROVIDER PUBLIC API
// ============================================================================

/// Canonical simulation-provider types and functionality.
///
/// These are convenience re-exports. Their implementations
/// remain owned by [`provider`].
pub use provider::{
    SimulatedDevice,
    SimulatedOperationReport,
    SimulatedOutcome,
    SimulationFault,
    SimulationFaultPoint,
    SimulationProvider,
    SimulationProviderConfig,
};

/// Stable simulation-provider component identifier.
pub use provider::COMPONENT as PROVIDER_COMPONENT;

/// Simulation-provider schema identifier.
pub use provider::SIMULATION_PROVIDER_SCHEMA_ID;

/// Simulation-provider schema version.
pub use provider::SIMULATION_PROVIDER_SCHEMA_VERSION;

// ============================================================================
// CONFORMANCE PUBLIC API
// ============================================================================

/// Provider conformance configuration and reporting types.
///
/// Their implementations remain owned by [`conformance`].
pub use conformance::{
    ConformanceCheckStatus,
    ConformanceFinding,
    ConformanceOptions,
    ConformanceReport,
    ConformanceStatus,
};

/// Executes the reusable asynchronous provider conformance suite.
///
/// This is an alias-free re-export of the canonical implementation
/// in [`conformance`].
pub use conformance::run_provider_conformance;

// ============================================================================
// CONVENIENCE PRELUDE
// ============================================================================

/// Commonly used ZHAL simulation APIs.
///
/// Importing this prelude does not initialize a simulator,
/// register a provider, discover hardware or allocate resources.
///
/// Example:
///
/// ```ignore
/// use zamani_compiler::zhal::simulation::prelude::*;
///
/// let simulated_device = SimulatedDevice::new(device_descriptor);
/// ```
///
/// The example is illustrative; device descriptors must be
/// constructed using their canonical ZHAL model constructors.
pub mod prelude {
    pub use super::{
        ConformanceCheckStatus,
        ConformanceFinding,
        ConformanceOptions,
        ConformanceReport,
        ConformanceStatus,
        SimulatedDevice,
        SimulatedOperationReport,
        SimulatedOutcome,
        SimulationFault,
        SimulationFaultPoint,
        SimulationProvider,
        SimulationProviderConfig,
        run_provider_conformance,
    };
}

// ============================================================================
// COMPOSITION CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensures that the provider implementation is reachable through
    /// both its canonical path and the stable public re-export.
    #[test]
    fn provider_public_api_is_composed() {
        fn assert_provider_type(
            _: Option<&SimulationProvider>,
        ) {
        }

        fn assert_device_type(
            _: Option<&SimulatedDevice>,
        ) {
        }

        fn assert_config_type(
            _: Option<&SimulationProviderConfig>,
        ) {
        }

        assert_provider_type(None);
        assert_device_type(None);
        assert_config_type(None);

        let _: Option<&provider::SimulationProvider> = None;
        let _: Option<&provider::SimulatedDevice> = None;
        let _: Option<&provider::SimulationProviderConfig> = None;
    }

    /// Ensures that conformance APIs are reachable through the
    /// stable simulation namespace.
    #[test]
    fn conformance_public_api_is_composed() {
        let _: Option<ConformanceOptions> = None;
        let _: Option<ConformanceReport> = None;
        let _: Option<ConformanceFinding> = None;

        let _: Option<ConformanceCheckStatus> = None;
        let _: Option<ConformanceStatus> = None;

        let _: Option<conformance::ConformanceOptions> = None;
        let _: Option<conformance::ConformanceReport> = None;
    }

    /// Ensures that the provider's simulation identity remains
    /// available from the composition root.
    #[test]
    fn simulation_schema_identity_is_available() {
        assert_eq!(
            PROVIDER_COMPONENT,
            provider::COMPONENT
        );

        assert_eq!(
            SIMULATION_PROVIDER_SCHEMA_ID,
            provider::SIMULATION_PROVIDER_SCHEMA_ID
        );

        assert_eq!(
            SIMULATION_PROVIDER_SCHEMA_VERSION,
            provider::SIMULATION_PROVIDER_SCHEMA_VERSION
        );

        assert!(!SIMULATION_PROVIDER_SCHEMA_ID.is_empty());
        assert!(!SIMULATION_PROVIDER_SCHEMA_VERSION.is_empty());
    }

    /// Ensures that the canonical asynchronous conformance
    /// runner remains exposed without another implementation.
    #[test]
    fn conformance_runner_is_the_canonical_runner() {
        let _: fn(
            &dyn crate::zhal::providers::contract::HardwareProvider,
            &crate::zhal::context::ZhalContext,
            &ConformanceOptions,
        ) -> std::pin::Pin<
            Box<
                dyn std::future::Future<Output = ConformanceReport>
                    + Send
                    + '_,
            >,
        > = |provider, context, options| {
            Box::pin(run_provider_conformance(
                provider,
                context,
                options,
            ))
        };
    }
}
