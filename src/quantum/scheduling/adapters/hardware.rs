//! Zamani Quantum Scheduling — Hardware Adapter
//!
//! Production-grade, provider-neutral integration boundary between
//! `quantum::hardware` and `quantum::scheduling`.
//!
//! # Responsibility
//!
//! This module translates authoritative hardware information into a
//! scheduler-consumable, immutable view without duplicating hardware
//! semantics inside the scheduler.
//!
//! The adapter answers:
//!
//! > "What hardware target information is available to the scheduler for this
//! > scheduling invocation?"
//!
//! It does NOT answer:
//!
//! - which provider API should be called;
//! - how a workload should be routed;
//! - how a schedule should be constructed;
//! - how calibration should be acquired;
//! - how QEC should be performed;
//! - how a quantum program should be optimized;
//! - how a QPU should be executed.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir
//!      |
//!      v
//! routing
//!      |
//!      v
//! quantum::scheduling
//!      |
//!      +-----------------------------+
//!      |                             |
//!      v                             v
//! adapters::ir                 adapters::hardware
//!      |                             |
//!      |                             v
//!      |                    quantum::hardware
//!      |                             |
//!      |                +------------+------------+
//!      |                |            |            |
//!      |                v            v            v
//!      |          capabilities    topology      timing
//!      |                |            |            |
//!      +----------------+------------+------------+
//!                       |
//!                       v
//!                SchedulingContext
//!                       |
//!                       v
//!                    planner
//! ```
//!
//! # Critical ownership rule
//!
//! `quantum::hardware` remains authoritative for:
//!
//! - backend identity;
//! - provider identity;
//! - backend kind;
//! - backend status;
//! - capabilities;
//! - limits;
//! - topology.
//!
//! This adapter MUST NOT create replacement versions of those concepts.
//!
//! The adapter only exposes them to the scheduling subsystem through a stable
//! scheduling boundary.
//!
//! # Resource identity
//!
//! Physical and logical qubit identities remain owned by the canonical quantum
//! IR:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Scheduler resource identity remains owned by:
//!
//! ```text
//! crate::quantum::ir::core::identity::ResourceId
//! ```
//!
//! This file therefore never invents a second `QubitId` or `ResourceId`.
//!
//! # Universal-program principle
//!
//! A Zamani program must not contain:
//!
//! - a fixed physical-qubit count;
//! - a fixed channel count;
//! - a fixed topology;
//! - a fixed hardware clock;
//! - a fixed gate duration;
//! - a fixed provider;
//! - a fixed machine size.
//!
//! The same program is specialized against a target at compilation or
//! execution time.
//!
//! ```text
//! one Zamani program
//!       |
//!       +---- tiny target
//!       |
//!       +---- medium target
//!       |
//!       +---- large target
//!       |
//!       +---- distributed target
//!       |
//!       +---- future target
//! ```
//!
//! The adapter therefore deliberately contains no `MAX_QUBITS`,
//! `MAX_CHANNELS`, fixed topology, fixed clock period, or provider-specific
//! constants.
//!
//! # Important distinction: limits versus inventory
//!
//! `BackendLimits` describes limits such as the maximum number of physical
//! resources or operations accepted by a backend.
//!
//! It does NOT necessarily enumerate the currently schedulable resources.
//!
//! Therefore:
//!
//! ```text
//! BackendLimits
//!      !=
//! Resource inventory
//! ```
//!
//! This adapter never converts `max_qubits` into synthetic resource IDs.
//! Doing so would silently create incorrect schedules for machines whose
//! resource identifiers are sparse, hierarchical, distributed, dynamically
//! allocated, or provider-defined.
//!
//! Actual resource bindings are supplied explicitly through
//! `HardwareResourceBinding`.
//!
//! # Timing ownership
//!
//! Timing information remains owned by the hardware timing/calibration layers.
//!
//! This adapter does not introduce a default gate duration.
//!
//! A missing duration is represented as missing information and must be dealt
//! with by the scheduler's timing policy or a target-specific timing adapter.
//!
//! # Status ownership
//!
//! Backend operational status is read from `BackendDescriptor`.
//!
//! This module does not poll hardware, contact providers, or mutate status.
//!
//! # Thread safety
//!
//! The adapter contains no global mutable state and performs no I/O.
//!
//! `HardwareSchedulingView` borrows the authoritative backend descriptor and
//! is therefore as thread-safe as the descriptor reference supplied to it.
//!
//! `HardwareSchedulingSnapshot` owns only copied scheduler-facing metadata and
//! is suitable for independent scheduling contexts.
//!
//! # Determinism
//!
//! All collections exposed by this module use deterministic ordering.
//!
//! Resource bindings are stored in `BTreeMap`.
//!
//! The adapter performs no random selection and never reads the system clock.
//!
//! # Error policy
//!
//! Adapter errors are provider-neutral and structured.
//!
//! Provider SDK errors must never cross this boundary.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! # Integration contract
//!
//! This file is intentionally designed so that later additions to:
//!
//! - planners;
//! - policies;
//! - QEC;
//! - distributed scheduling;
//! - hardware providers;
//! - diagnostics;
//! - serialization;
//! - runtime;
//! - benchmarking
//!
//! do not require this file to be modified merely because those components
//! were added.
//!
//! If a new subsystem requires information that belongs to hardware, it must
//! obtain that information from `quantum::hardware` and translate it at its
//! own boundary rather than adding provider-specific semantics here.
//!
//! # File completion invariant
//!
//! This file is complete when:
//!
//! 1. hardware identity can be exposed without duplication;
//! 2. backend capabilities can be exposed without duplication;
//! 3. backend limits can be exposed without duplication;
//! 4. backend topology can be exposed without duplication;
//! 5. explicit resource bindings can be represented;
//! 6. canonical physical qubit identities can be retained;
//! 7. no artificial machine-size limit is introduced;
//! 8. no timing constant is invented;
//! 9. no provider SDK type leaks through the API;
//! 10. no network operation is performed;
//! 11. no global state exists;
//! 12. scheduling can consume an immutable hardware view;
//! 13. snapshots remain deterministic;
//! 14. malformed resource bindings are rejected;
//! 15. Rust 1.97/1.97.1 compatibility is maintained;
//! 16. unsafe Rust is impossible in this file.
//!
//! Adding a new scheduler algorithm or hardware provider must not require
//! reopening this file merely to make the architecture work.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::quantum::hardware::backend::{
    BackendCapabilities,
    BackendDescriptor,
    BackendKind,
    BackendLimits,
    BackendStatus,
};
use crate::quantum::hardware::topology::HardwareTopology;
use crate::quantum::ir::core::identity::ResourceId;
use crate::quantum::ir::qubit::PhysicalQubitId;
use crate::quantum::scheduling::resources::resource::{
    ResourceCapacity,
    ResourceKind,
};

// =============================================================================
// Schema
// =============================================================================

/// Stable adapter schema identifier.
pub const HARDWARE_ADAPTER_SCHEMA_ID: &str =
    "zamani.quantum.scheduling.adapters.hardware";

/// Semantic schema version.
///
/// Increment only when the meaning of the serialized/public adapter contract
/// changes incompatibly.
pub const HARDWARE_ADAPTER_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Error model
// =============================================================================

/// Error returned by the scheduling/hardware integration boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HardwareAdapterError {
    /// A required backend descriptor was absent.
    MissingBackendDescriptor,

    /// A required target identifier was empty.
    EmptyTargetId,

    /// A resource binding used an invalid resource identity.
    InvalidResourceId {
        /// The invalid identifier.
        resource_id: ResourceId,
    },

    /// A resource binding used zero capacity.
    ZeroResourceCapacity {
        /// Resource whose capacity was invalid.
        resource_id: ResourceId,
    },

    /// A resource was registered more than once.
    DuplicateResource {
        /// Resource that was duplicated.
        resource_id: ResourceId,
    },

    /// One physical qubit was associated with multiple resources where the
    /// adapter was configured to require one-to-one physical-resource mapping.
    DuplicatePhysicalQubit {
        /// Physical qubit involved in the conflict.
        qubit: PhysicalQubitId,

        /// First resource.
        first_resource: ResourceId,

        /// Second resource.
        second_resource: ResourceId,
    },

    /// A scheduling operation requires a resource that is not present in the
    /// supplied inventory.
    MissingResource {
        /// Missing resource.
        resource_id: ResourceId,
    },

    /// The target is not usable according to the requested policy.
    BackendUnavailable {
        /// Current backend status.
        status: BackendStatus,
    },

    /// The target is known but does not expose topology information.
    TopologyUnavailable,

    /// A caller supplied an invalid adapter configuration.
    InvalidConfiguration {
        /// Human-readable but stable diagnostic.
        reason: String,
    },
}

impl fmt::Display for HardwareAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingBackendDescriptor => {
                formatter.write_str("hardware adapter requires a backend descriptor")
            }
            Self::EmptyTargetId => {
                formatter.write_str("hardware adapter target identifier is empty")
            }
            Self::InvalidResourceId { resource_id } => {
                write!(formatter, "invalid resource identifier: {resource_id:?}")
            }
            Self::ZeroResourceCapacity { resource_id } => {
                write!(
                    formatter,
                    "resource {resource_id:?} has zero capacity"
                )
            }
            Self::DuplicateResource { resource_id } => {
                write!(
                    formatter,
                    "resource {resource_id:?} was registered more than once"
                )
            }
            Self::DuplicatePhysicalQubit {
                qubit,
                first_resource,
                second_resource,
            } => {
                write!(
                    formatter,
                    "physical qubit {qubit:?} is bound to both \
                     resource {first_resource:?} and resource {second_resource:?}"
                )
            }
            Self::MissingResource { resource_id } => {
                write!(
                    formatter,
                    "required scheduling resource {resource_id:?} \
                     is absent from the hardware inventory"
                )
            }
            Self::BackendUnavailable { status } => {
                write!(
                    formatter,
                    "backend is not available for scheduling: {status}"
                )
            }
            Self::TopologyUnavailable => {
                formatter.write_str("hardware topology information is unavailable")
            }
            Self::InvalidConfiguration { reason } => {
                write!(formatter, "invalid hardware adapter configuration: {reason}")
            }
        }
    }
}

impl Error for HardwareAdapterError {}

// =============================================================================
// Adapter configuration
// =============================================================================

/// Controls how hardware information is exposed to the scheduler.
///
/// This configuration deliberately contains policy only. It does not contain
/// provider credentials, provider URLs, hardware addresses, or fixed machine
/// sizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HardwareAdapterConfig {
    /// Require the backend status to be schedulable.
    ///
    /// When false, an offline/maintenance target can still be converted into
    /// an offline compilation target. This is useful for precompilation and
    /// reproducibility.
    pub require_available_backend: bool,

    /// Require topology information to be present.
    ///
    /// Gate-model physical scheduling normally requires topology information,
    /// but abstract, logical, analog, or provider-managed workloads may not.
    pub require_topology: bool,

    /// Require physical qubit bindings to be unique.
    ///
    /// This is normally true for physical-qubit resources.
    pub enforce_unique_physical_qubits: bool,
}

impl Default for HardwareAdapterConfig {
    fn default() -> Self {
        Self {
            require_available_backend: false,
            require_topology: false,
            enforce_unique_physical_qubits: true,
        }
    }
}

impl HardwareAdapterConfig {
    /// Creates the conservative production configuration.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            require_available_backend: true,
            require_topology: false,
            enforce_unique_physical_qubits: true,
        }
    }

    /// Creates a configuration suitable for offline compilation.
    #[must_use]
    pub const fn offline_compilation() -> Self {
        Self {
            require_available_backend: false,
            require_topology: false,
            enforce_unique_physical_qubits: true,
        }
    }
}

// =============================================================================
// Resource binding
// =============================================================================

/// Explicit binding between a scheduler resource and an authoritative
/// hardware resource.
///
/// The adapter deliberately requires the mapping to be supplied rather than
/// deriving it from a numeric qubit count.
///
/// This permits:
///
/// - sparse physical identifiers;
/// - non-contiguous identifiers;
/// - chiplets;
/// - modules;
/// - distributed QPUs;
/// - provider-defined resource IDs;
/// - future resource types.
///
/// `physical_qubit` is optional because not every hardware resource is a
/// physical qubit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareResourceBinding {
    /// Canonical scheduler resource identity.
    resource_id: ResourceId,

    /// Generic scheduler resource category.
    kind: ResourceKind,

    /// Schedulable resource capacity.
    capacity: ResourceCapacity,

    /// Canonical physical qubit identity when this resource represents one.
    physical_qubit: Option<PhysicalQubitId>,
}

impl HardwareResourceBinding {
    /// Creates a generic hardware resource binding.
    ///
    /// No machine-specific assumptions are made.
    #[must_use]
    pub const fn new(
        resource_id: ResourceId,
        kind: ResourceKind,
        capacity: ResourceCapacity,
    ) -> Self {
        Self {
            resource_id,
            kind,
            capacity,
            physical_qubit: None,
        }
    }

    /// Creates a physical-qubit resource binding.
    #[must_use]
    pub const fn physical_qubit(
        resource_id: ResourceId,
        qubit: PhysicalQubitId,
    ) -> Self {
        Self {
            resource_id,
            kind: ResourceKind::PhysicalQubit,
            capacity: ResourceCapacity::Finite(1),
            physical_qubit: Some(qubit),
        }
    }

    /// Creates a physical-qubit binding with an explicitly supplied capacity.
    #[must_use]
    pub const fn physical_qubit_with_capacity(
        resource_id: ResourceId,
        qubit: PhysicalQubitId,
        capacity: ResourceCapacity,
    ) -> Self {
        Self {
            resource_id,
            kind: ResourceKind::PhysicalQubit,
            capacity,
            physical_qubit: Some(qubit),
        }
    }

    /// Returns the resource identity.
    #[must_use]
    pub const fn resource_id(&self) -> ResourceId {
        self.resource_id
    }

    /// Returns the generic resource category.
    #[must_use]
    pub const fn kind(&self) -> ResourceKind {
        self.kind
    }

    /// Returns the resource capacity.
    #[must_use]
    pub const fn capacity(&self) -> ResourceCapacity {
        self.capacity
    }

    /// Returns the associated physical qubit, if any.
    #[must_use]
    pub const fn physical_qubit(&self) -> Option<PhysicalQubitId> {
        self.physical_qubit
    }

    /// Returns whether this binding represents a physical qubit.
    #[must_use]
    pub const fn is_physical_qubit(&self) -> bool {
        self.physical_qubit.is_some()
    }
}

// =============================================================================
// Resource inventory
// =============================================================================

/// Immutable hardware resource inventory prepared for scheduling.
///
/// This structure intentionally does not assume that all resources are
/// physical qubits.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HardwareResourceInventory {
    resources: BTreeMap<ResourceId, HardwareResourceBinding>,
}

impl HardwareResourceInventory {
    /// Creates an empty inventory.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an inventory from bindings.
    ///
    /// Duplicate resource identifiers are rejected.
    pub fn from_bindings<I>(
        bindings: I,
        enforce_unique_physical_qubits: bool,
    ) -> Result<Self, HardwareAdapterError>
    where
        I: IntoIterator<Item = HardwareResourceBinding>,
    {
        let mut inventory = Self::new();
        let mut physical_qubits: BTreeMap<PhysicalQubitId, ResourceId> =
            BTreeMap::new();

        for binding in bindings {
            let resource_id = binding.resource_id();

            if resource_id == ResourceId::new(0) {
                return Err(HardwareAdapterError::InvalidResourceId {
                    resource_id,
                });
            }

            if binding.capacity().is_zero() {
                return Err(HardwareAdapterError::ZeroResourceCapacity {
                    resource_id,
                });
            }

            if inventory.resources.contains_key(&resource_id) {
                return Err(HardwareAdapterError::DuplicateResource {
                    resource_id,
                });
            }

            if enforce_unique_physical_qubits {
                if let Some(qubit) = binding.physical_qubit() {
                    if let Some(first_resource) =
                        physical_qubits.insert(qubit, resource_id)
                    {
                        return Err(
                            HardwareAdapterError::DuplicatePhysicalQubit {
                                qubit,
                                first_resource,
                                second_resource: resource_id,
                            },
                        );
                    }
                }
            }

            inventory.resources.insert(resource_id, binding);
        }

        Ok(inventory)
    }

    /// Returns the number of explicitly supplied resources.
    #[must_use]
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    /// Returns whether the inventory contains no resources.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    /// Returns one resource binding.
    #[must_use]
    pub fn get(
        &self,
        resource_id: ResourceId,
    ) -> Option<&HardwareResourceBinding> {
        self.resources.get(&resource_id)
    }

    /// Returns all resources in deterministic identifier order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&ResourceId, &HardwareResourceBinding)> {
        self.resources.iter()
    }

    /// Returns whether a resource exists.
    #[must_use]
    pub fn contains(&self, resource_id: ResourceId) -> bool {
        self.resources.contains_key(&resource_id)
    }

    /// Returns the underlying deterministic map.
    #[must_use]
    pub fn as_map(&self) -> &BTreeMap<ResourceId, HardwareResourceBinding> {
        &self.resources
    }

    /// Validates that every requested resource is present.
    pub fn validate_required_resources<I>(
        &self,
        required: I,
    ) -> Result<(), HardwareAdapterError>
    where
        I: IntoIterator<Item = ResourceId>,
    {
        for resource_id in required {
            if !self.contains(resource_id) {
                return Err(HardwareAdapterError::MissingResource {
                    resource_id,
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Immutable borrowed hardware view
// =============================================================================

/// Immutable scheduling view over an authoritative hardware descriptor.
///
/// This is the preferred representation when the scheduler can retain an
/// `Arc<BackendDescriptor>` or otherwise keep the backend alive for the
/// scheduling invocation.
///
/// No hardware information is copied unnecessarily.
#[derive(Debug, Clone, Copy)]
pub struct HardwareSchedulingView<'a> {
    backend: &'a BackendDescriptor,
}

impl<'a> HardwareSchedulingView<'a> {
    /// Creates an immutable scheduling view.
    #[must_use]
    pub const fn new(backend: &'a BackendDescriptor) -> Self {
        Self { backend }
    }

    /// Returns the authoritative backend descriptor.
    #[must_use]
    pub const fn backend(&self) -> &'a BackendDescriptor {
        self.backend
    }

    /// Returns backend metadata.
    #[must_use]
    pub fn metadata(&self) -> &crate::quantum::hardware::backend::BackendMetadata {
        self.backend.metadata()
    }

    /// Returns the backend kind.
    #[must_use]
    pub fn kind(&self) -> BackendKind {
        self.backend.kind()
    }

    /// Returns the backend status.
    #[must_use]
    pub fn status(&self) -> BackendStatus {
        self.backend.status()
    }

    /// Returns backend capabilities.
    #[must_use]
    pub fn capabilities(&self) -> &BackendCapabilities {
        self.backend.capabilities()
    }

    /// Returns backend resource limits.
    #[must_use]
    pub fn limits(&self) -> &BackendLimits {
        self.backend.limits()
    }

    /// Returns the authoritative hardware topology.
    #[must_use]
    pub fn topology(&self) -> &HardwareTopology {
        self.backend.topology()
    }

    /// Returns the provider identifier.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        &self.backend.metadata().provider_id
    }

    /// Returns the backend identifier.
    #[must_use]
    pub fn backend_id(&self) -> &str {
        &self.backend.metadata().backend_id
    }

    /// Returns whether the backend status permits execution.
    #[must_use]
    pub fn is_available(&self) -> bool {
        self.status().is_usable()
    }

    /// Returns the number of topology resources known to the authoritative
    /// topology model.
    #[must_use]
    pub fn topology_qubit_count(&self) -> usize {
        self.topology().qubit_count()
    }

    /// Returns the number of topology couplings known to the authoritative
    /// topology model.
    #[must_use]
    pub fn topology_coupling_count(&self) -> usize {
        self.topology().coupling_count()
    }

    /// Validates the view against adapter configuration.
    pub fn validate(
        &self,
        config: HardwareAdapterConfig,
    ) -> Result<(), HardwareAdapterError> {
        if self.backend_id().trim().is_empty() {
            return Err(HardwareAdapterError::EmptyTargetId);
        }

        if config.require_available_backend && !self.is_available() {
            return Err(HardwareAdapterError::BackendUnavailable {
                status: self.status(),
            });
        }

        if config.require_topology && self.capabilities().topology_information {
            if self.topology_qubit_count() == 0
                && self.topology_coupling_count() == 0
            {
                return Err(HardwareAdapterError::TopologyUnavailable);
            }
        }

        Ok(())
    }
}

// =============================================================================
// Owned scheduling snapshot
// =============================================================================

/// Owned immutable hardware snapshot for a scheduling invocation.
///
/// This is the preferred object for long-lived compilation pipelines because
/// it decouples the schedule from mutable provider/backend registries.
///
/// The snapshot intentionally preserves the authoritative backend types rather
/// than flattening them into a second scheduler-specific capability model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareSchedulingSnapshot {
    backend_id: String,
    provider_id: String,
    kind: BackendKind,
    status: BackendStatus,
    capabilities: BackendCapabilities,
    limits: BackendLimits,
    topology_qubit_count: usize,
    topology_coupling_count: usize,
    resources: HardwareResourceInventory,
}

impl HardwareSchedulingSnapshot {
    /// Creates a snapshot from an authoritative backend descriptor.
    ///
    /// No resource IDs are synthesized from backend limits.
    pub fn from_backend(
        backend: &BackendDescriptor,
        config: HardwareAdapterConfig,
    ) -> Result<Self, HardwareAdapterError> {
        let view = HardwareSchedulingView::new(backend);

        view.validate(config)?;

        Ok(Self {
            backend_id: view.backend_id().to_owned(),
            provider_id: view.provider_id().to_owned(),
            kind: view.kind(),
            status: view.status(),
            capabilities: view.capabilities().clone(),
            limits: *view.limits(),
            topology_qubit_count: view.topology_qubit_count(),
            topology_coupling_count: view.topology_coupling_count(),
            resources: HardwareResourceInventory::new(),
        })
    }

    /// Adds the explicit resource inventory for this snapshot.
    ///
    /// The returned snapshot is new; the existing snapshot remains unchanged.
    pub fn with_resources<I>(
        mut self,
        bindings: I,
        enforce_unique_physical_qubits: bool,
    ) -> Result<Self, HardwareAdapterError>
    where
        I: IntoIterator<Item = HardwareResourceBinding>,
    {
        self.resources = HardwareResourceInventory::from_bindings(
            bindings,
            enforce_unique_physical_qubits,
        )?;

        Ok(self)
    }

    /// Returns the backend identifier.
    #[must_use]
    pub fn backend_id(&self) -> &str {
        &self.backend_id
    }

    /// Returns the provider identifier.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    /// Returns the backend kind.
    #[must_use]
    pub const fn kind(&self) -> BackendKind {
        self.kind
    }

    /// Returns the backend status captured at snapshot creation.
    #[must_use]
    pub const fn status(&self) -> BackendStatus {
        self.status
    }

    /// Returns backend capabilities.
    #[must_use]
    pub const fn capabilities(&self) -> &BackendCapabilities {
        &self.capabilities
    }

    /// Returns backend limits.
    #[must_use]
    pub const fn limits(&self) -> &BackendLimits {
        &self.limits
    }

    /// Returns topology qubit count captured at snapshot creation.
    #[must_use]
    pub const fn topology_qubit_count(&self) -> usize {
        self.topology_qubit_count
    }

    /// Returns topology coupling count captured at snapshot creation.
    #[must_use]
    pub const fn topology_coupling_count(&self) -> usize {
        self.topology_coupling_count
    }

    /// Returns the explicit scheduler resource inventory.
    #[must_use]
    pub const fn resources(&self) -> &HardwareResourceInventory {
        &self.resources
    }

    /// Returns whether the captured backend status permits execution.
    #[must_use]
    pub const fn is_available(&self) -> bool {
        self.status.is_usable()
    }

    /// Validates that all requested resources exist in the snapshot.
    pub fn validate_required_resources<I>(
        &self,
        required: I,
    ) -> Result<(), HardwareAdapterError>
    where
        I: IntoIterator<Item = ResourceId>,
    {
        self.resources.validate_required_resources(required)
    }
}

// =============================================================================
// Adapter
// =============================================================================

/// Provider-neutral hardware-to-scheduler adapter.
///
/// The adapter is stateless. All state belongs to the produced view or
/// snapshot.
///
/// This makes the adapter:
///
/// - reusable;
/// - deterministic;
/// - thread-safe;
/// - cheap to construct;
/// - free of global state;
/// - independent of provider SDKs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct HardwareAdapter;

impl HardwareAdapter {
    /// Creates a new stateless hardware adapter.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Creates an immutable borrowed scheduling view.
    #[must_use]
    pub const fn view<'a>(
        &self,
        backend: &'a BackendDescriptor,
    ) -> HardwareSchedulingView<'a> {
        HardwareSchedulingView::new(backend)
    }

    /// Creates an owned scheduling snapshot.
    pub fn snapshot(
        &self,
        backend: &BackendDescriptor,
        config: HardwareAdapterConfig,
    ) -> Result<HardwareSchedulingSnapshot, HardwareAdapterError> {
        HardwareSchedulingSnapshot::from_backend(backend, config)
    }

    /// Creates an owned snapshot and attaches explicit hardware resources.
    pub fn snapshot_with_resources<I>(
        &self,
        backend: &BackendDescriptor,
        config: HardwareAdapterConfig,
        bindings: I,
    ) -> Result<HardwareSchedulingSnapshot, HardwareAdapterError>
    where
        I: IntoIterator<Item = HardwareResourceBinding>,
    {
        let snapshot = self.snapshot(backend, config)?;

        snapshot.with_resources(
            bindings,
            config.enforce_unique_physical_qubits,
        )
    }

    /// Validates a backend without constructing a snapshot.
    pub fn validate(
        &self,
        backend: &BackendDescriptor,
        config: HardwareAdapterConfig,
    ) -> Result<(), HardwareAdapterError> {
        self.view(backend).validate(config)
    }
}

// =============================================================================
// Convenience functions
// =============================================================================

/// Creates an immutable hardware scheduling view.
#[must_use]
pub const fn adapt(
    backend: &BackendDescriptor,
) -> HardwareSchedulingView<'_> {
    HardwareSchedulingView::new(backend)
}

/// Creates an owned hardware scheduling snapshot.
pub fn snapshot(
    backend: &BackendDescriptor,
    config: HardwareAdapterConfig,
) -> Result<HardwareSchedulingSnapshot, HardwareAdapterError> {
    HardwareAdapter::new().snapshot(backend, config)
}

/// Creates an owned hardware scheduling snapshot with explicit resources.
pub fn snapshot_with_resources<I>(
    backend: &BackendDescriptor,
    config: HardwareAdapterConfig,
    bindings: I,
) -> Result<HardwareSchedulingSnapshot, HardwareAdapterError>
where
    I: IntoIterator<Item = HardwareResourceBinding>,
{
    HardwareAdapter::new().snapshot_with_resources(
        backend,
        config,
        bindings,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn resource(value: u64) -> ResourceId {
        ResourceId::new(value)
    }

    #[test]
    fn adapter_is_stateless_and_copyable() {
        let first = HardwareAdapter::new();
        let second = first;

        assert_eq!(first, second);
    }

    #[test]
    fn generic_resource_binding_preserves_identity() {
        let binding = HardwareResourceBinding::new(
            resource(7),
            ResourceKind::ControlChannel,
            ResourceCapacity::Finite(2),
        );

        assert_eq!(binding.resource_id(), resource(7));
        assert_eq!(binding.kind(), ResourceKind::ControlChannel);
        assert_eq!(
            binding.capacity(),
            ResourceCapacity::Finite(2)
        );
        assert_eq!(binding.physical_qubit(), None);
    }

    #[test]
    fn physical_qubit_binding_uses_canonical_identity() {
        let qubit = PhysicalQubitId::new(42);

        let binding =
            HardwareResourceBinding::physical_qubit(resource(42), qubit);

        assert_eq!(binding.kind(), ResourceKind::PhysicalQubit);
        assert_eq!(binding.physical_qubit(), Some(qubit));
    }

    #[test]
    fn duplicate_resource_ids_are_rejected() {
        let bindings = [
            HardwareResourceBinding::new(
                resource(1),
                ResourceKind::ControlChannel,
                ResourceCapacity::Finite(1),
            ),
            HardwareResourceBinding::new(
                resource(1),
                ResourceKind::MeasurementChannel,
                ResourceCapacity::Finite(1),
            ),
        ];

        let result = HardwareResourceInventory::from_bindings(
            bindings,
            true,
        );

        assert!(matches!(
            result,
            Err(HardwareAdapterError::DuplicateResource { .. })
        ));
    }

    #[test]
    fn zero_capacity_is_rejected() {
        let bindings = [HardwareResourceBinding::new(
            resource(1),
            ResourceKind::ControlChannel,
            ResourceCapacity::Finite(0),
        )];

        let result = HardwareResourceInventory::from_bindings(
            bindings,
            true,
        );

        assert!(matches!(
            result,
            Err(HardwareAdapterError::ZeroResourceCapacity { .. })
        ));
    }

    #[test]
    fn duplicate_physical_qubits_are_rejected_when_enabled() {
        let qubit = PhysicalQubitId::new(3);

        let bindings = [
            HardwareResourceBinding::physical_qubit(resource(1), qubit),
            HardwareResourceBinding::physical_qubit(resource(2), qubit),
        ];

        let result = HardwareResourceInventory::from_bindings(
            bindings,
            true,
        );

        assert!(matches!(
            result,
            Err(HardwareAdapterError::DuplicatePhysicalQubit { .. })
        ));
    }

    #[test]
    fn duplicate_physical_qubits_can_be_allowed_for_explicit_aliasing() {
        let qubit = PhysicalQubitId::new(3);

        let bindings = [
            HardwareResourceBinding::physical_qubit(resource(1), qubit),
            HardwareResourceBinding::physical_qubit(resource(2), qubit),
        ];

        let result = HardwareResourceInventory::from_bindings(
            bindings,
            false,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn inventory_is_deterministically_ordered() {
        let bindings = [
            HardwareResourceBinding::new(
                resource(9),
                ResourceKind::ControlChannel,
                ResourceCapacity::Finite(1),
            ),
            HardwareResourceBinding::new(
                resource(2),
                ResourceKind::MeasurementChannel,
                ResourceCapacity::Finite(1),
            ),
            HardwareResourceBinding::new(
                resource(5),
                ResourceKind::Coupler,
                ResourceCapacity::Finite(1),
            ),
        ];

        let inventory =
            HardwareResourceInventory::from_bindings(bindings, true)
                .expect("inventory must be valid");

        let ids: Vec<ResourceId> =
            inventory.iter().map(|(id, _)| *id).collect();

        assert_eq!(
            ids,
            vec![
                resource(2),
                resource(5),
                resource(9)
            ]
        );
    }

    #[test]
    fn missing_required_resource_is_reported() {
        let bindings = [HardwareResourceBinding::new(
            resource(1),
            ResourceKind::ControlChannel,
            ResourceCapacity::Finite(1),
        )];

        let inventory =
            HardwareResourceInventory::from_bindings(bindings, true)
                .expect("inventory must be valid");

        let result =
            inventory.validate_required_resources([resource(1), resource(99)]);

        assert!(matches!(
            result,
            Err(HardwareAdapterError::MissingResource {
                resource_id
            }) if resource_id == resource(99)
        ));
    }

    #[test]
    fn unlimited_capacity_is_preserved() {
        let binding = HardwareResourceBinding::new(
            resource(11),
            ResourceKind::Compute,
            ResourceCapacity::Unlimited,
        );

        assert_eq!(
            binding.capacity(),
            ResourceCapacity::Unlimited
        );
    }

    #[test]
    fn offline_compilation_configuration_does_not_require_available_status() {
        let config = HardwareAdapterConfig::offline_compilation();

        assert!(!config.require_available_backend);
        assert!(!config.require_topology);
        assert!(config.enforce_unique_physical_qubits);
    }

    #[test]
    fn production_configuration_requires_available_backend() {
        let config = HardwareAdapterConfig::production();

        assert!(config.require_available_backend);
        assert!(!config.require_topology);
        assert!(config.enforce_unique_physical_qubits);
    }

    #[test]
    fn schema_is_stable() {
        assert_eq!(
            HARDWARE_ADAPTER_SCHEMA_ID,
            "zamani.quantum.scheduling.adapters.hardware"
        );

        assert_eq!(HARDWARE_ADAPTER_SCHEMA_VERSION, 1);
    }
}