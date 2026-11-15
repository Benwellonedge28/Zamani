
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Models
//!
//! Canonical path: `src/zhal/model/mod.rs`
//!
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97 or later.
//! Safety: Safe Rust only.
//!
//! # 1. PURPOSE
//!
//! This module is the canonical composition root for the
//! provider-independent physical hardware models used by ZHAL.
//!
//! It composes the existing hardware model implementations
//! into one stable public namespace.
//!
//! It does not introduce another hardware model, identity
//! system, capability system, resource allocator, provider
//! registry, execution engine, or intermediate representation.
//!
//! The model architecture supports hardware descriptions
//! originating from:
//!
//! - Classical computing.
//! - Quantum computing.
//! - Hardware description and programmable logic.
//! - GPUs and other accelerators.
//! - AI and tensor computing.
//! - Embedded and edge computing.
//! - Hybrid computing.
//! - Distributed computing.
//! - Neuromorphic computing.
//! - Photonic computing.
//! - Analog computing.
//! - In-memory computing.
//! - Future computational technologies.
//!
//! Hardware technologies are not restricted to this list.
//!
//! # 2. OWNERSHIP
//!
//! This composition root owns:
//!
//! - The canonical hardware-model module hierarchy.
//! - Stable public paths to hardware-model implementations.
//! - Selective model API re-exports.
//! - Model composition documentation.
//! - Model-level architectural invariants.
//! - Model composition tests.
//!
//! The individual child modules own their respective
//! structures, validation, serialization and behavior.
//!
//! # 3. DOES NOT OWN
//!
//! This module does not own:
//!
//! - Source-language grammar.
//! - AST construction.
//! - Semantic analysis.
//! - Source-level resource requirements.
//! - Source-level capability requirements.
//! - Hardware discovery.
//! - Provider registration.
//! - Provider lifecycle.
//! - Device inventory.
//! - Physical device access.
//! - Hardware communication.
//! - Device sessions.
//! - Physical resource allocation.
//! - Resource reservation.
//! - Resource scheduling.
//! - Hardware routing.
//! - Quantum routing.
//! - Quantum scheduling.
//! - HDL synthesis.
//! - FPGA or ASIC realization.
//! - Distributed execution.
//! - Hybrid execution.
//! - Compiler target selection.
//! - Intermediate representation.
//! - Runtime orchestration.
//!
//! These responsibilities remain with their canonical
//! repository authorities.
//!
//! # 4. CANONICAL MODEL AUTHORITIES
//!
//! Hardware metadata:
//!     crate::zhal::model::metadata
//!
//! Physical hardware resources:
//!     crate::zhal::model::resource
//!
//! Hardware interfaces:
//!     crate::zhal::model::interface
//!
//! Physical hardware devices:
//!     crate::zhal::model::device
//!
//! Physical hardware topology:
//!     crate::zhal::model::topology
//!
//! Hardware identities:
//!     crate::zhal::identity
//!
//! Provider identities and contracts:
//!     crate::stdlib::provider
//!
//! Capability descriptions and snapshots:
//!     crate::stdlib::capability
//!
//! Generic device identity and byte-oriented I/O:
//!     crate::stdlib::io::devices
//!
//! Source-level semantic requirements:
//!     crate::frontend::semantic
//!
//! Runtime resource management:
//!     crate::zur
//!
//! Quantum hardware:
//!     crate::quantum::hardware
//!
//! HDL realization:
//!     crate::hdl::realization
//!
//! Distributed execution:
//!     crate::distributed
//!
//! # 5. MODEL COMPOSITION
//!
//! The five existing model modules are independent public
//! namespaces with explicit relationships.
//!
//! ```text
//!                 zhal::model
//!                      |
//!          +-----------+-----------+
//!          |           |           |
//!       metadata   resource   interface
//!          |           |           |
//!          +-----------+-----------+
//!                      |
//!                 device
//!                      |
//!                  topology
//! ```
//!
//! This is an ownership and conceptual relationship diagram.
//! It does not impose an execution dependency ordering.
//!
//! Device descriptions reference the canonical hardware
//! identities, resource identities, interface identities,
//! metadata and externally owned capability observations.
//!
//! Topology describes relationships between observed physical
//! hardware components.
//!
//! Individual modules remain responsible for their own
//! internal validation and data integrity.
//!
//! # 6. GRAMMAR BOUNDARY
//!
//! The model layer must remain consistent with:
//!
//! - grammar/hardware/devices.g4
//! - grammar/hardware/resources.g4
//! - grammar/hardware/topology.g4
//! - grammar/hardware/capabilities.g4
//! - grammar/specification/semantic-model.md
//! - grammar/specification/execution-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/poco-reaf.md
//!
//! The grammar describes logical hardware intent.
//!
//! The semantic layer validates that intent.
//!
//! ZHAL models represent hardware facts reported by providers.
//!
//! The runtime and compiler consume these descriptions through
//! explicit integration contracts.
//!
//! A logical source-level device must never automatically
//! become a physical device identity.
//!
//! A logical topology must never automatically become a
//! discovered physical topology.
//!
//! A resource requirement must never automatically become
//! a physical resource reservation.
//!
//! # 7. POCO-REAF AND SCALABILITY
//!
//! This module introduces no universal numerical limits.
//!
//! In particular, it must not define:
//!
//! - MAX_DEVICES.
//! - MAX_CPUS.
//! - MAX_GPUS.
//! - MAX_QPUS.
//! - MAX_QUBITS.
//! - MAX_MEMORY.
//! - MAX_RESOURCES.
//! - MAX_INTERFACES.
//! - MAX_TOPOLOGY_NODES.
//! - MAX_TOPOLOGY_LINKS.
//! - MAX_HARDWARE_CATEGORIES.
//! - MAX_COMPUTATIONAL_DOMAINS.
//!
//! No closed enumeration of hardware technologies is
//! introduced by this composition root.
//!
//! Collections and model representations must grow according
//! to actual input, available memory and explicitly configured
//! operational budgets.
//!
//! Actual implementation, address-space, hardware and
//! representation constraints must be reported honestly.
//!
//! Literal infinite physical computation or storage is not
//! promised.
//!
//! The architectural objective is to avoid artificial
//! universal ceilings that prevent a program from scaling
//! when compatible resources become available.
//!
//! # 8. IMMUTABILITY AND OBSERVATION
//!
//! Model descriptions represent hardware facts or observations.
//!
//! They must not be interpreted as permanent guarantees.
//!
//! Device availability may change.
//!
//! Resource availability may change.
//!
//! Hardware interfaces may become inaccessible.
//!
//! Topology may change.
//!
//! Firmware and hardware revisions may change.
//!
//! Updated observations must preserve their provenance and
//! appropriate versioning.
//!
//! Discovery and inventory authorities determine when
//! observations are refreshed.
//!
//! # 9. SECURITY
//!
//! Hardware models are descriptive data structures.
//!
//! They do not independently establish:
//!
//! - Provider trust.
//! - Device authenticity.
//! - Authorization.
//! - Physical access permissions.
//! - Credential validity.
//! - Resource ownership rights.
//! - Successful hardware execution.
//!
//! Descriptive metadata must not become a storage location
//! for credentials, private keys, tokens or session secrets.
//!
//! Trust, authorization and isolation belong to the
//! established security authorities.
//!
//! # 10. DEPENDENCY DIRECTION
//!
//! The intended integration direction is:
//!
//! ```text
//! Grammar
//!    |
//!    v
//! Frontend
//!    |
//!    v
//! Semantic model
//!    |
//!    v
//! Compiler / planning
//!    |
//!    v
//! ZUR / resource orchestration
//!    |
//!    v
//! ZHAL
//!    |
//!    +--> Hardware models
//!    +--> Hardware providers
//!    +--> Device access
//!    +--> Domain adapters
//!    |
//!    +--> Classical hardware
//!    +--> Quantum hardware
//!    +--> HDL realization
//!    +--> Accelerators
//!    +--> Embedded hardware
//!    +--> Distributed hardware
//! ```
//!
//! This file does not implement those integrations.
//!
//! It provides their stable model namespace.
//!
//! # 11. EXTENSIBILITY
//!
//! New hardware-model functionality must be introduced
//! through independently owned modules.
//!
//! A new model module must:
//!
//! 1. Have one clearly defined responsibility.
//! 2. Identify its canonical data ownership.
//! 3. Reuse existing identity and capability authorities.
//! 4. Preserve open-world hardware extensibility.
//! 5. Validate externally supplied information.
//! 6. Define serialization behavior where applicable.
//! 7. Preserve provenance.
//! 8. Avoid arbitrary hardware ceilings.
//! 9. Define its integration contracts.
//! 10. Include independent tests.
//!
//! A new module must be registered here only after its
//! implementation and public contract are complete.
//!
//! Adding a new hardware vendor or provider must not require
//! modifying this composition root.
//!
//! # 12. PUBLIC API POLICY
//!
//! The module exposes both:
//!
//! - Stable named child namespaces.
//! - Explicitly selected commonly used model types.
//!
//! Wildcard re-exports are intentionally prohibited.
//!
//! Consumers may use the specialized module paths when
//! additional types or implementation details are required.
//!
//! # 13. SAFETY
//!
//! No unsafe Rust is permitted.
//!
//! This composition root performs no:
//!
//! - Hardware access.
//! - Filesystem access.
//! - Network access.
//! - Device discovery.
//! - Global initialization.
//! - Thread creation.
//! - Resource allocation.
//! - Mutable global-state management.
//!
//! All operations remain explicit and owned by the
//! appropriate subsystem.
//!
//! # 14. FREEZE CRITERIA
//!
//! This module is complete when:
//!
//! - All declared child modules exist.
//! - All child modules compile.
//! - All public re-exports resolve.
//! - No competing identity or capability types are introduced.
//! - The grammar/model ownership boundary is preserved.
//! - No universal hardware ceiling is introduced.
//! - No unsafe Rust is introduced.
//! - All model tests pass.
//! - The complete crate passes formatting, checking,
//!   testing and Clippy validation.

/// Hardware metadata and firmware descriptions.
///
/// Owns descriptive hardware metadata, metadata validation,
/// metadata serialization and hardware version information.
///
/// Metadata is not evidence of authenticity or capability.
pub mod metadata;

/// Physical hardware resource descriptions and observations.
///
/// Owns extensible resource identities, units, exact resource
/// quantities, capacity descriptions, availability observations
/// and resource provenance.
///
/// Does not allocate, reserve or schedule physical resources.
pub mod resource;

/// Physical and provider-backed hardware interface descriptions.
///
/// Owns interface kinds, communication characteristics,
/// supported operation descriptions, availability and
/// interface validation.
///
/// Does not implement physical communication.
pub mod interface;

/// Provider-independent physical hardware device descriptions.
///
/// Owns device descriptors, availability observations,
/// device provenance and references to canonical hardware
/// identities, capabilities, resources and interfaces.
///
/// Does not discover or access physical devices.
pub mod device;

/// Observed physical hardware topology.
///
/// Owns topology identities, physical topology nodes,
/// interconnect descriptions, hierarchy relationships,
/// topology construction, validation and structural queries.
///
/// Does not own source-level topology intent, placement,
/// routing or scheduling.
pub mod topology;

// ============================================================================
// METADATA PUBLIC API
// ============================================================================

/// Commonly used hardware metadata types.
///
/// The complete API remains available through
/// `crate::zhal::model::metadata`.
pub use metadata::{
    FirmwareDescriptor,
    HardwareMetadata,
    HardwareVersionInfo,
    MetadataError,
    MetadataErrorKind,
    MetadataKey,
    MetadataResult,
    MetadataValue,
};

// ============================================================================
// RESOURCE PUBLIC API
// ============================================================================

/// Commonly used physical hardware resource types.
///
/// The complete API remains available through
/// `crate::zhal::model::resource`.
pub use resource::{
    HardwareResource,
    ResourceAmount,
    ResourceAvailability,
    ResourceCapacity,
    ResourceError,
    ResourceErrorKind,
    ResourceKindId,
    ResourceObservation,
    ResourceOwnerReference,
    ResourceProvenance,
    ResourceResult,
    ResourceUnitId,
};

// ============================================================================
// INTERFACE PUBLIC API
// ============================================================================

/// Commonly used hardware interface types.
///
/// The complete API remains available through
/// `crate::zhal::model::interface`.
pub use interface::{
    CommunicationCharacteristics,
    HardwareInterfaceDescriptor,
    HardwareInterfaceKind,
    InterfaceAvailability,
    InterfaceAvailabilityState,
    InterfaceDirection,
    InterfaceError,
    InterfaceErrorKind,
    InterfaceResult,
    SupportedOperation,
};

// ============================================================================
// DEVICE PUBLIC API
// ============================================================================

/// Commonly used hardware device types.
///
/// Canonical device and provider identities are deliberately
/// not redefined or re-exported as new model-owned identities.
///
/// Their authorities remain `crate::zhal::identity`,
/// `crate::stdlib::provider` and `crate::stdlib::io::devices`.
pub use device::{
    DeviceAvailability,
    DeviceModelError,
    DeviceModelErrorKind,
    DeviceModelResult,
    HardwareDeviceDescriptor,
    HardwareDeviceProvenance,
};

// ============================================================================
// TOPOLOGY PUBLIC API
// ============================================================================

/// Commonly used physical hardware topology types.
///
/// Topology identifiers remain distinct from device identities,
/// resource identities and source-level logical topology names.
pub use topology::{
    HardwareTopology,
    HardwareTopologyBuilder,
    LinkDirection,
    TopologyError,
    TopologyErrorKind,
    TopologyLink,
    TopologyLinkId,
    TopologyLinkKind,
    TopologyNode,
    TopologyNodeId,
    TopologyNodeKind,
    TopologyResult,
    TopologyRevision,
    TraversalMode,
};

// ============================================================================
// COMPOSITION IDENTITY
// ============================================================================

/// Stable identity of the ZHAL hardware-model composition root.
pub const COMPONENT: &str = "zamani.zhal.model";

/// Returns the stable hardware-model composition identity.
#[must_use]
pub const fn component_id() -> &'static str {
    COMPONENT
}

/// Returns the hardware-model composition generation.
///
/// This identifies the composition contract generation.
/// It is not the language version, Cargo version, hardware
/// firmware version or model schema version.
#[must_use]
pub const fn composition_generation() -> u32 {
    1
}

/// Returns the model scalability policy identifier.
///
/// This is descriptive architecture metadata, not a
/// numerical resource limit.
#[must_use]
pub const fn scalability_policy() -> &'static str {
    "resource-bounded-open-world-hardware-models"
}

/// Returns the model safety policy identifier.
#[must_use]
pub const fn safety_policy() -> &'static str {
    "safe-rust-no-unsafe"
}

// ============================================================================
// COMPOSITION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_stable_component_identity() {
        assert_eq!(
            component_id(),
            "zamani.zhal.model"
        );
    }

    #[test]
    fn exposes_composition_generation() {
        assert_eq!(
            composition_generation(),
            1
        );
    }

    #[test]
    fn exposes_open_world_scalability_policy() {
        assert_eq!(
            scalability_policy(),
            "resource-bounded-open-world-hardware-models"
        );
    }

    #[test]
    fn exposes_safe_rust_policy() {
        assert_eq!(
            safety_policy(),
            "safe-rust-no-unsafe"
        );
    }

    #[test]
    fn exposes_all_existing_model_namespaces() {
        let metadata_module =
            core::any::type_name::<metadata::HardwareMetadata>();

        let resource_module =
            core::any::type_name::<resource::ResourceAmount>();

        let interface_module =
            core::any::type_name::<interface::HardwareInterfaceDescriptor>();

        let device_module =
            core::any::type_name::<device::HardwareDeviceDescriptor>();

        let topology_module =
            core::any::type_name::<topology::HardwareTopology>();

        assert!(metadata_module.contains("metadata"));
        assert!(resource_module.contains("resource"));
        assert!(interface_module.contains("interface"));
        assert!(device_module.contains("device"));
        assert!(topology_module.contains("topology"));
    }

    #[test]
    fn common_types_are_available_through_stable_exports() {
        fn accepts_metadata(_: &HardwareMetadata) {}
        fn accepts_resource(_: &HardwareResource) {}
        fn accepts_interface(_: &HardwareInterfaceDescriptor) {}
        fn accepts_device(_: &HardwareDeviceDescriptor) {}
        fn accepts_topology(_: &HardwareTopology) {}

        let _ = (
            accepts_metadata,
            accepts_resource,
            accepts_interface,
            accepts_device,
            accepts_topology,
        );
    }

    #[test]
    fn does_not_define_hardware_capacity_constants() {
        assert_eq!(
            COMPONENT,
            "zamani.zhal.model"
        );

        // Hardware capacities belong to observed models
        // and actual provider/runtime resource information.
        // This composition root introduces no capacity limits.
    }
}
