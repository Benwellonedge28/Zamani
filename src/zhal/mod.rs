
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer (ZHAL)
//!
//! Path: `src/zhal/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! Primary architectural objective:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).
//!
//! ## 1. PURPOSE
//!
//! ZHAL is the common physical hardware integration boundary of Zamani.
//!
//! It connects the existing compiler, backend, runtime and computational
//! domain authorities to independently implemented hardware providers.
//!
//! It is designed to accommodate:
//!
//! - Classical processors and computing systems.
//! - Memory and storage hardware.
//! - Quantum processors and quantum-control systems.
//! - Quantum simulators and emulators.
//! - FPGA and ASIC realization.
//! - GPUs, TPUs, NPUs, DSPs and other accelerators.
//! - Embedded systems and microcontrollers.
//! - Distributed and remote hardware.
//! - Hybrid and heterogeneous systems.
//! - Neuromorphic, photonic, analog and future technologies.
//!
//! These are supported architectural domains, not claims that every
//! hardware implementation or provider already exists.
//!
//! ## 2. ARCHITECTURAL OWNERSHIP
//!
//! ZHAL owns the common physical hardware abstraction boundary.
//!
//! It does not replace:
//!
//! - Source-language grammar.
//! - Frontend or semantic analysis.
//! - Classical IR.
//! - Quantum IR.
//! - HDLIR.
//! - ZUIR.
//! - Backend code generation.
//! - Runtime scheduling.
//! - Runtime resource allocation.
//! - Distributed execution.
//! - Hybrid computation.
//! - Domain-specific execution semantics.
//!
//! The authoritative repository boundaries remain:
//!
//! Grammar:
//!     crate::grammar (external specification)
//!
//! Standard-library provider contracts:
//!     crate::stdlib::provider
//!
//! Standard-library capability contracts:
//!     crate::stdlib::capability
//!
//! Standard-library operation context:
//!     crate::stdlib::context
//!
//! Generic byte-oriented device interfaces:
//!     crate::stdlib::io::devices
//!
//! Quantum hardware:
//!     crate::quantum::hardware
//!
//! HDL realization:
//!     crate::hdl::realization
//!
//! Host-platform descriptions:
//!     crate::zur::platforms
//!
//! Runtime and resource orchestration:
//!     crate::zur
//!     crate::runtime
//!
//! Distributed computation:
//!     crate::distributed
//!
//! Hybrid computation:
//!     crate::hybrid
//!
//! Classical computation:
//!     crate::classical
//!
//! Backend implementations:
//!     crate::backend
//!
//! ## 3. RESPONSIBILITIES
//!
//! The ZHAL architecture must support:
//!
//! 1. Hardware-provider integration.
//! 2. Hardware identification.
//! 3. Hardware description and observation.
//! 4. Hardware capability evidence.
//! 5. Device discovery.
//! 6. Hardware interface descriptions.
//! 7. Provider-backed device access.
//! 8. Hardware resource observations.
//! 9. Hardware operation contracts.
//! 10. Device session integration.
//! 11. Hardware health observations.
//! 12. Provider lifecycle integration.
//! 13. Hardware failure reporting.
//! 14. Hardware provenance.
//! 15. Domain-specific hardware adapters.
//! 16. Hardware simulation and conformance.
//!
//! These responsibilities will be implemented in independently owned
//! child modules rather than accumulated in this composition root.
//!
//! ## 4. EXISTING CONTRACT REUSE
//!
//! ZHAL reuses the repository's existing contracts.
//!
//! It MUST NOT define competing versions of:
//!
//! - ProviderId.
//! - ProviderDescriptor.
//! - ProviderStatus.
//! - ProviderFuture.
//! - ProviderLifecycle.
//! - CapabilityId.
//! - CapabilitySnapshot.
//! - OperationContext.
//! - CancellationSignal.
//! - DeviceDescriptor.
//! - DeviceConnection.
//! - DeviceProvider.
//!
//! The owning standard-library modules remain authoritative.
//!
//! ## 5. PROVIDER ARCHITECTURE
//!
//! The common provider authority is:
//!
//!     crate::stdlib::provider
//!
//! Specialized hardware providers extend the common provider contract
//! with domain-appropriate interfaces.
//!
//! A hardware provider may expose:
//!
//! - Device discovery.
//! - Hardware observations.
//! - Capability evidence.
//! - Hardware-specific sessions.
//! - Hardware interfaces.
//! - Domain-specific operations.
//! - Health and diagnostic observations.
//!
//! A byte-oriented device provider may additionally implement:
//!
//!     crate::stdlib::io::devices::DeviceProvider
//!
//! However, ZHAL MUST NOT require every hardware technology to implement
//! byte-oriented transport.
//!
//! Quantum circuits, FPGA realization, processor execution, accelerator
//! operations and physical-device control require their own typed
//! domain-specific interfaces.
//!
//! Provider-specific implementations belong behind adapter boundaries.
//!
//! ## 6. CAPABILITY AND SEMANTIC BOUNDARY
//!
//! Capability identifiers and snapshots are reused from:
//!
//!     crate::stdlib::capability
//!
//! The following concepts MUST remain distinct:
//!
//! Requirement:
//!     What a workload requires.
//!
//! Capability:
//!     What a provider or hardware environment can supply.
//!
//! Preference:
//!     What is desirable but not mandatory.
//!
//! Constraint:
//!     What an acceptable realization must obey.
//!
//! Hint:
//!     Non-binding implementation guidance.
//!
//! Feasibility:
//!     Whether available evidence establishes that a realization is
//!     possible.
//!
//! ZHAL does not establish source-level semantic validity.
//!
//! ZHAL does not interpret source-language requirements independently.
//!
//! ZHAL supplies physical hardware information to the appropriate
//! compiler and runtime authorities.
//!
//! Unknown capability evidence MUST NOT be treated as available.
//!
//! Provider declarations MUST NOT be confused with current availability.
//!
//! A discovery result MUST NOT be interpreted as a reservation.
//!
//! ## 7. POCO-REAF
//!
//! The same Zamani program must retain its defined semantics when
//! compatible implementations use different physical hardware.
//!
//! The architecture must accommodate:
//!
//! - Different processor architectures.
//! - Different memory configurations.
//! - Different accelerator implementations.
//! - Different quantum technologies.
//! - Different programmable hardware.
//! - Different embedded environments.
//! - Different network topologies.
//! - Different distributed deployments.
//! - Future computational technologies.
//!
//! A target that cannot satisfy mandatory requirements must produce
//! an explicit incompatibility or infeasibility result.
//!
//! Hardware adaptation MUST NOT silently change program semantics.
//!
//! ## 8. SCALABILITY
//!
//! ZHAL MUST NOT impose arbitrary universal limits on:
//!
//! - Hardware devices.
//! - Providers.
//! - CPUs.
//! - Processor cores.
//! - Threads.
//! - GPUs.
//! - FPGAs.
//! - ASICs.
//! - QPUs.
//! - Qubits.
//! - Memory.
//! - Storage.
//! - Accelerators.
//! - Distributed participants.
//! - Hardware interfaces.
//! - Hardware resources.
//! - Computational domains.
//! - Provider operations.
//!
//! In particular, this composition root MUST NOT introduce constants
//! such as:
//!
//!     MAX_DEVICES
//!     MAX_CPUS
//!     MAX_GPUS
//!     MAX_QPUS
//!     MAX_QUBITS
//!     MAX_MEMORY
//!     MAX_REGISTER_WIDTH
//!     MAX_PROVIDERS
//!     MAX_NODES
//!
//! Collections must grow according to available resources and explicitly
//! configured operational budgets.
//!
//! Operational budgets are not universal hardware capability ceilings.
//!
//! Physical computation remains subject to memory, processing capacity,
//! address-space limits, provider capabilities, operating-system
//! constraints and physical reality.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## 9. RESOURCE OWNERSHIP
//!
//! ZHAL exposes physical hardware information and provider-facing
//! resource interaction contracts.
//!
//! The canonical runtime owns:
//!
//! - Resource allocation.
//! - Resource scheduling.
//! - Execution placement.
//! - Resource orchestration.
//! - Operational leases.
//! - Runtime recovery.
//!
//! ZHAL MUST NOT introduce an independent resource allocator.
//!
//! A hardware observation does not guarantee future availability.
//!
//! A reservation must be confirmed by the responsible provider.
//!
//! ## 10. DOMAIN INTEGRATION
//!
//! Classical:
//!     crate::classical
//!     crate::backend
//!
//! Quantum:
//!     crate::quantum::hardware
//!     crate::quantum::ir
//!
//! HDL:
//!     crate::hdl::realization
//!     crate::hdl::hdlir
//!
//! AI and accelerators:
//!     crate::ai
//!     crate::classical::collections
//!
//! Hybrid:
//!     crate::hybrid
//!
//! Distributed:
//!     crate::distributed
//!     crate::zur::distributed
//!
//! Embedded and physical I/O:
//!     crate::stdlib::io::devices
//!
//! Domain adapters MUST preserve the semantics and ownership of their
//! respective subsystems.
//!
//! ## 11. SECURITY
//!
//! Provider registration does not establish trust.
//!
//! Hardware discovery does not authorize hardware access.
//!
//! Provider metadata and device descriptors MUST NOT contain:
//!
//! - Passwords.
//! - API keys.
//! - Private keys.
//! - Bearer tokens.
//! - Cookies.
//! - Authorization headers.
//! - Other authentication secrets.
//!
//! Authentication, authorization, provider trust and isolation remain
//! subject to the existing security authorities.
//!
//! ## 12. INITIALIZATION
//!
//! Importing ZHAL MUST NOT:
//!
//! - Discover physical hardware.
//! - Initialize hardware providers.
//! - Allocate physical resources.
//! - Start background threads.
//! - Open network connections.
//! - Initialize quantum processors.
//! - Initialize AI models.
//! - Create global mutable registries.
//! - Execute user programs.
//!
//! Initialization and shutdown must be explicit operations coordinated
//! by the appropriate runtime and provider lifecycle authorities.
//!
//! ## 13. ERROR HANDLING
//!
//! Existing structured errors are reused.
//!
//! Hardware integration must preserve distinctions between:
//!
//! - Invalid input.
//! - Unsupported functionality.
//! - Unknown capability.
//! - Unavailable hardware.
//! - Inaccessible hardware.
//! - Provider failure.
//! - Hardware communication failure.
//! - Resource exhaustion.
//! - Authorization failure.
//! - Cancellation.
//! - Timeout.
//! - Partial completion.
//! - Unknown remote outcome.
//! - Physical hardware failure.
//!
//! No provider failure may be silently represented as success.
//!
//! ## 14. CONCURRENCY
//!
//! ZHAL introduces no global mutable state.
//!
//! Provider concurrency guarantees must be documented by their
//! implementations.
//!
//! Operation context and cancellation are delegated to:
//!
//!     crate::stdlib::context
//!
//! Cancellation remains cooperative.
//!
//! A cancellation request MUST NOT be reported as proof that a physical
//! operation has stopped.
//!
//! ## 15. API AND VERSIONING
//!
//! The schema identity identifies the ZHAL composition boundary.
//!
//! It is distinct from:
//!
//! - Cargo package version.
//! - Zamani language version.
//! - Grammar version.
//! - Provider implementation version.
//! - Provider API version.
//! - Hardware firmware version.
//! - Hardware protocol version.
//!
//! Child modules must maintain their own documented ownership and
//! compatibility contracts.
//!
//! ## 16. INTEGRATION POLICY
//!
//! This file is the ZHAL composition root.
//!
//! It intentionally exposes existing canonical contracts and does not
//! declare child modules until their implementation files exist.
//!
//! The future child modules must be added in dependency order:
//!
//! version, identity, errors, config, context,
//! model, capabilities, providers, devices,
//! resources, operations, security, observability,
//! adapters and simulation.
//!
//! Each child module must be independently validated before its
//! declaration is added here.
//!
//! ## 17. SAFETY
//!
//! Unsafe Rust is prohibited.
//!
//! Provider implementations must preserve the crate-wide safety policy.
//!
//! ## 18. PRODUCTION ACCEPTANCE
//!
//! This composition root is complete when:
//!
//! 1. The crate root exposes `pub mod zhal;`.
//! 2. Existing standard-library contracts are reused.
//! 3. Public paths are stable and documented.
//! 4. No competing provider or capability authority is introduced.
//! 5. No nonexistent child module is declared.
//! 6. No arbitrary hardware ceiling is introduced.
//! 7. No implicit initialization or hardware discovery occurs.
//! 8. No unsafe Rust is introduced.
//! 9. Composition tests pass.
//!
//! Full ZHAL production readiness additionally requires implementation,
//! conformance and integration testing of its child modules and adapters.

// ============================================================================
// SCHEMA IDENTITY
// ============================================================================

/// Stable ZHAL schema identity.
pub const ZHAL_SCHEMA_ID: &str = "zamani.zhal";

/// Major version of the ZHAL composition schema.
pub const ZHAL_SCHEMA_MAJOR: u16 = 1;

/// Minor version of the ZHAL composition schema.
pub const ZHAL_SCHEMA_MINOR: u16 = 0;

/// Patch version of the ZHAL composition schema.
pub const ZHAL_SCHEMA_PATCH: u16 = 0;

/// Rust edition used by ZHAL.
pub const ZHAL_RUST_EDITION: &str = "2021";

/// Minimum Rust compiler version required by ZHAL.
pub const ZHAL_MIN_RUST_VERSION: &str = "1.97";

/// Returns the stable ZHAL schema identifier.
#[must_use]
pub const fn schema_id() -> &'static str {
    ZHAL_SCHEMA_ID
}

/// Returns the ZHAL schema version as major, minor and patch.
#[must_use]
pub const fn schema_version() -> (u16, u16, u16) {
    (
        ZHAL_SCHEMA_MAJOR,
        ZHAL_SCHEMA_MINOR,
        ZHAL_SCHEMA_PATCH,
    )
}

// ============================================================================
// ARCHITECTURAL INVARIANTS
// ============================================================================

/// Machine-checkable names for the principal ZHAL architectural rules.
pub mod invariants {
    /// Hardware providers use extensible provider identities.
    pub const OPEN_WORLD_PROVIDERS: &str = "open-world-providers";

    /// Hardware capability evidence is observational.
    pub const EVIDENCE_IS_OBSERVATIONAL: &str =
        "capability-evidence-is-observational";

    /// Hardware discovery does not allocate physical resources.
    pub const DISCOVERY_DOES_NOT_ALLOCATE: &str =
        "discovery-does-not-allocate";

    /// Source-language semantics remain authoritative.
    pub const SEMANTICS_ARE_PRESERVED: &str =
        "source-semantics-are-preserved";

    /// Runtime owns resource allocation and orchestration.
    pub const RUNTIME_OWNS_ALLOCATION: &str =
        "runtime-owns-resource-allocation";

    /// Domain-specific authorities retain their semantics.
    pub const DOMAIN_AUTHORITIES_PRESERVED: &str =
        "domain-authorities-are-preserved";

    /// No arbitrary universal hardware ceilings are introduced.
    pub const NO_HARDWARE_CEILINGS: &str =
        "no-arbitrary-hardware-ceilings";

    /// Hardware discovery is never implicitly initialized.
    pub const NO_IMPLICIT_DISCOVERY: &str =
        "no-implicit-hardware-discovery";

    /// Provider metadata must not contain authentication secrets.
    pub const NO_SECRETS_IN_METADATA: &str =
        "no-authentication-secrets-in-metadata";

    /// ZHAL does not require unsafe Rust.
    pub const SAFE_RUST_ONLY: &str = "safe-rust-only";

    /// New providers must not require unrelated core changes.
    pub const PROVIDER_EXTENSIBILITY: &str =
        "provider-additions-are-non-breaking";
}

// ============================================================================
// EXISTING CANONICAL CONTRACTS
// ============================================================================

/// Standard-library capability authority.
///
/// This module owns capability identities, descriptions, evidence,
/// availability and immutable snapshots.
pub use crate::stdlib::capability;

/// Standard-library operation context and cancellation authority.
pub use crate::stdlib::context;

/// Standard-library structured error authority.
pub use crate::stdlib::error;

/// Standard-library common provider authority.
pub use crate::stdlib::provider;

/// Standard-library generic byte-oriented device I/O authority.
///
/// This is not a universal hardware execution interface.
pub use crate::stdlib::io::devices as device_io;

// ============================================================================
// CONTROLLED PUBLIC TYPE RE-EXPORTS
// ============================================================================

/// Canonical capability contracts.
pub use crate::stdlib::capability::{
    Capability,
    CapabilityId,
    CapabilityKind,
    CapabilitySnapshot,
    CapabilityState,
    CapabilityStatus,
};

/// Canonical operation context and cancellation contracts.
pub use crate::stdlib::context::{
    CancellationSignal,
    NeverCancelled,
    OperationContext,
};

/// Canonical structured error types.
pub use crate::stdlib::error::{
    Error,
    ErrorKind,
    Result,
};

/// Canonical provider identities, descriptors and lifecycle contracts.
pub use crate::stdlib::provider::{
    Provider,
    ProviderContractId,
    ProviderDescriptor,
    ProviderFuture,
    ProviderId,
    ProviderKind,
    ProviderLifecycle,
    ProviderOperationDescriptor,
    ProviderOperationId,
    ProviderStatus,
};

/// Canonical generic device-access contracts.
pub use crate::stdlib::io::devices::{
    DeviceAccess,
    DeviceCapabilityId,
    DeviceConnection,
    DeviceDescriptor,
    DeviceId,
    DeviceKindId,
    DeviceProvider,
    DeviceTransport,
};

// ============================================================================
// CONTROLLED PUBLIC PRELUDE
// ============================================================================

/// Common imports for applications integrating with ZHAL.
///
/// Domain-specific hardware APIs should be imported from their
/// authoritative modules rather than flattened into this namespace.
pub mod prelude {
    pub use super::{
        Capability,
        CapabilityId,
        CapabilityKind,
        CapabilitySnapshot,
        CapabilityState,
        CapabilityStatus,
        CancellationSignal,
        DeviceAccess,
        DeviceCapabilityId,
        DeviceConnection,
        DeviceDescriptor,
        DeviceId,
        DeviceKindId,
        DeviceProvider,
        DeviceTransport,
        Error,
        ErrorKind,
        NeverCancelled,
        OperationContext,
        Provider,
        ProviderContractId,
        ProviderDescriptor,
        ProviderFuture,
        ProviderId,
        ProviderKind,
        ProviderLifecycle,
        ProviderOperationDescriptor,
        ProviderOperationId,
        ProviderStatus,
        Result,
    };

    pub use super::{
        capability,
        context,
        device_io,
        error,
        invariants,
        provider,
    };
}

// ============================================================================
// COMPOSITION CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_identity_is_stable() {
        assert_eq!(schema_id(), "zamani.zhal");
    }

    #[test]
    fn schema_version_is_valid() {
        let (major, minor, patch) = schema_version();

        assert_eq!(major, 1);
        assert_eq!(minor, 0);
        assert_eq!(patch, 0);
    }

    #[test]
    fn rust_compatibility_is_explicit() {
        assert_eq!(ZHAL_RUST_EDITION, "2021");
        assert_eq!(ZHAL_MIN_RUST_VERSION, "1.97");
    }

    #[test]
    fn architectural_invariants_are_declared() {
        let values = [
            invariants::OPEN_WORLD_PROVIDERS,
            invariants::EVIDENCE_IS_OBSERVATIONAL,
            invariants::DISCOVERY_DOES_NOT_ALLOCATE,
            invariants::SEMANTICS_ARE_PRESERVED,
            invariants::RUNTIME_OWNS_ALLOCATION,
            invariants::DOMAIN_AUTHORITIES_PRESERVED,
            invariants::NO_HARDWARE_CEILINGS,
            invariants::NO_IMPLICIT_DISCOVERY,
            invariants::NO_SECRETS_IN_METADATA,
            invariants::SAFE_RUST_ONLY,
            invariants::PROVIDER_EXTENSIBILITY,
        ];

        for invariant in values {
            assert!(!invariant.is_empty());
        }
    }

    #[test]
    fn existing_provider_contracts_are_reachable() {
        let _: Option<ProviderId> = None;
        let _: Option<ProviderDescriptor> = None;
        let _: Option<ProviderStatus> = None;
        let _: Option<CapabilitySnapshot> = None;
        let _: Option<OperationContext> = None;
    }

    #[test]
    fn existing_device_contracts_are_reachable() {
        let _: Option<DeviceId> = None;
        let _: Option<DeviceDescriptor> = None;
        let _: Option<DeviceConnection> = None;
        let _: Option<&dyn DeviceProvider> = None;
    }

    #[test]
    fn provider_contract_remains_object_safe() {
        let _: Option<&dyn Provider> = None;
    }

    #[test]
    fn future_provider_identities_remain_representable() {
        let id = ProviderId::new(
            "future.photonic.neuromorphic.provider",
        )
        .expect("future provider identity should be accepted");

        assert_eq!(
            id.as_str(),
            "future.photonic.neuromorphic.provider",
        );
    }

    #[test]
    fn future_capabilities_remain_representable() {
        let capability = CapabilityId::new(
            "future.molecular.computation",
        )
        .expect("future capability should be accepted");

        assert_eq!(
            capability.as_str(),
            "future.molecular.computation",
        );
    }

    #[test]
    fn prelude_exposes_canonical_contracts() {
        use crate::zhal::prelude::*;

        let _: Option<ProviderId> = None;
        let _: Option<CapabilityId> = None;
        let _: Option<DeviceId> = None;
        let _: Option<OperationContext> = None;
    }
}
