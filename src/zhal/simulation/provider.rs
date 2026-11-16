
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani ZHAL — Simulation Hardware Provider
//!
//! Path: src/zhal/simulation/provider.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Implements a configurable, extensible simulated hardware provider
//! for development, testing, provider conformance and fault injection.
//!
//! The provider integrates with the canonical Zamani hardware-provider
//! and standard-library provider contracts.
//!
//! Supported simulation subjects are open-world. They may describe
//! classical, quantum, HDL, accelerator, embedded, distributed,
//! hybrid and future computational technologies.
//!
//! ## ARCHITECTURAL AUTHORITIES
//!
//! Standard provider:
//!     crate::stdlib::provider
//!
//! Capability descriptions and snapshots:
//!     crate::stdlib::capability
//!
//! Operation context and cancellation:
//!     crate::stdlib::context
//!
//! Hardware provider:
//!     crate::zhal::providers::contract
//!
//! Hardware identity:
//!     crate::zhal::identity
//!
//! Device descriptions:
//!     crate::zhal::model::device
//!
//! Interface descriptions:
//!     crate::zhal::model::interface
//!
//! Resource observations:
//!     crate::zhal::model::resource
//!
//! Hardware context:
//!     crate::zhal::context
//!
//! Hardware errors:
//!     crate::zhal::errors
//!
//! Runtime orchestration and allocation:
//!     crate::zur
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - SimulationProvider.
//! - SimulationProviderConfig.
//! - SimulatedDevice.
//! - SimulationFault.
//! - SimulationFaultPoint.
//! - SimulatedOutcome.
//! - SimulatedOperationReport.
//! - Simulation inventory state.
//! - Simulated provider sessions.
//! - Simulation-specific failure injection.
//! - Simulation-specific latency modelling.
//!
//! It does not own:
//!
//! - Physical hardware drivers.
//! - Physical hardware discovery.
//! - Provider registration infrastructure.
//! - Provider identity.
//! - Capability identity.
//! - Capability semantics.
//! - Hardware device identity.
//! - Hardware resource identity.
//! - Resource allocation.
//! - Resource reservation.
//! - Runtime scheduling.
//! - Quantum execution.
//! - Quantum routing.
//! - HDL realization.
//! - Distributed transport.
//! - Hybrid execution.
//! - Source grammar.
//! - AST.
//! - ZUIR or domain-specific IR.
//! - Physical execution.
//!
//! ## FUNDAMENTAL INVARIANTS
//!
//! 1. Simulation is never physical execution.
//! 2. Simulated observations are not physical measurements.
//! 3. Simulated capability availability is not a real hardware guarantee.
//! 4. Discovery does not allocate or reserve resources.
//! 5. Device inspection does not authorize access.
//! 6. Unknown availability is never treated as available.
//! 7. Cancellation is cooperative.
//! 8. Partial completion is not successful full completion.
//! 9. Unknown outcomes are not silently converted into success.
//! 10. Provider identity is stable for the provider instance.
//!
//! ## SCALABILITY
//!
//! No arbitrary hardware ceilings are imposed.
//!
//! There are no fixed maximums for:
//!
//! - Simulated devices.
//! - Providers.
//! - Capabilities.
//! - Interfaces.
//! - Resources.
//! - Hardware categories.
//! - Computational domains.
//! - Simulation operations.
//! - Discovery pages.
//! - Session identifiers.
//! - Identifier lengths.
//!
//! Discovery is paginated.
//!
//! Inventory growth is governed by available memory and explicit
//! operational policies.
//!
//! Decimal sequence identifiers are used where a monotonically
//! increasing identifier is required.
//!
//! Literal infinite computation or memory is not promised.
//!
//! ## GRAMMAR BOUNDARY
//!
//! Relevant grammar:
//!
//! - grammar/execution/simulation.g4
//! - grammar/policies/simulation.g4
//! - grammar/hdl/simulation.g4
//! - grammar/resources/requirements.g4
//! - grammar/types/resource.g4
//!
//! Grammar expresses simulation and resource intent.
//!
//! This module does not parse source code, evaluate semantic
//! requirements, choose physical targets or allocate resources.
//!
//! ## LATENCY
//!
//! Configured latency is simulated metadata.
//!
//! It is returned in simulation reports.
//!
//! This implementation does not block an asynchronous runtime
//! using thread sleeps.
//!
//! A runtime-specific clock or scheduling adapter may be added
//! by the appropriate execution subsystem.
//!
//! ## SECURITY
//!
//! Simulation configuration and metadata must not contain credentials.
//!
//! Fault injection is an explicit testing capability.
//!
//! It must not be confused with real hardware security verification.
//!
//! ## THREAD SAFETY
//!
//! Shared simulation state is protected by RwLock.
//!
//! Session closure uses atomic state.
//!
//! No mutable global state is introduced.

use core::fmt;
use core::num::NonZeroUsize;

use std::collections::{BTreeMap, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, RwLock};

use std::time::Duration;

use crate::stdlib::capability::{
    CapabilitySnapshot,
};

use crate::stdlib::context::OperationContext;

use crate::stdlib::error::{
    Error,
    ErrorKind,
};

use crate::stdlib::io::devices::{
    DeviceId,
    DeviceKindId,
};

use crate::stdlib::provider::{
    Provider,
    ProviderDescriptor,
    ProviderFuture,
    ProviderId,
    ProviderLifecycle,
    ProviderOperationId,
    ProviderStatus,
};

use crate::stdlib::version::Version;

use crate::zhal::context::ZhalContext;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

use crate::zhal::identity::{
    HardwareDeviceIdentity,
    HardwareInterfaceId,
    HardwareInterfaceIdentity,
    HardwareSessionId,
    HardwareSessionIdentity,
};

use crate::zhal::model::device::{
    DeviceAvailability,
    HardwareDeviceDescriptor,
    HardwareDeviceProvenance,
};

use crate::zhal::model::interface::{
    CommunicationCharacteristics,
    HardwareInterfaceDescriptor,
    HardwareInterfaceKind,
    InterfaceAvailability,
    InterfaceAvailabilityState,
    InterfaceDirection,
};

use crate::zhal::model::resource::{
    ResourceObservation,
};

use crate::zhal::providers::contract::{
    DeviceDiscoveryCursor,
    DeviceDiscoveryPage,
    DeviceDiscoveryRequest,
    HardwareProvider,
    HardwareProviderFuture,
    HardwareSession,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable simulation-provider component identifier.
pub const COMPONENT: &str = "zhal.simulation.provider";

/// Simulation-provider schema identifier.
pub const SIMULATION_PROVIDER_SCHEMA_ID: &str =
    "zamani.zhal.simulation.provider";

/// Simulation-provider schema version.
pub const SIMULATION_PROVIDER_SCHEMA_VERSION: &str = "1";

// ============================================================================
// ERROR HELPERS
// ============================================================================

fn simulation_error(
    kind: ZhalErrorKind,
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(kind, message)
        .in_component(COMPONENT)
}

fn context_error(error: Error) -> ZhalError {
    let kind = match error.kind() {
        ErrorKind::Cancelled => ZhalErrorKind::Cancelled,
        ErrorKind::TimedOut => ZhalErrorKind::OperationTimedOut,
        ErrorKind::PermissionDenied => ZhalErrorKind::PermissionDenied,
        _ => ZhalErrorKind::Other,
    };

    simulation_error(kind, error.to_string())
        .with_source(error)
}

fn check_context(context: &ZhalContext) -> ZhalResult<()> {
    context.check_active().map_err(context_error)
}

fn lock_error() -> ZhalError {
    simulation_error(
        ZhalErrorKind::Internal,
        "simulation state lock is poisoned",
    )
}

// ============================================================================
// UNBOUNDED DECIMAL SEQUENCE SUPPORT
// ============================================================================

/// Increments a non-negative decimal sequence without imposing
/// an artificial integer-width ceiling.
///
/// The input must contain decimal digits.
fn increment_decimal(value: &mut String) {
    let mut digits: Vec<u8> = value.bytes().collect();

    if digits.is_empty() {
        digits.push(b'1');
        *value = String::from("1");
        return;
    }

    let mut carry = true;

    for digit in digits.iter_mut().rev() {
        if !carry {
            break;
        }

        if *digit == b'9' {
            *digit = b'0';
        } else {
            *digit += 1;
            carry = false;
        }
    }

    if carry {
        digits.insert(0, b'1');
    }

    // All bytes originate from ASCII decimal digits.
    *value = digits.into_iter().map(char::from).collect();
}

// ============================================================================
// SIMULATED DEVICE
// ============================================================================

/// A simulated device and its associated provider-owned observations.
///
/// The descriptor and interface contracts are reused from canonical ZHAL
/// models. This type does not introduce alternative device identities.
///
/// Resource observations are descriptive. They do not represent
/// allocations or reservations.
#[derive(Clone, Debug)]
pub struct SimulatedDevice {
    descriptor: HardwareDeviceDescriptor,
    interfaces: BTreeMap<
        HardwareInterfaceIdentity,
        HardwareInterfaceDescriptor,
    >,
    resource_observations: Vec<ResourceObservation>,
}

impl SimulatedDevice {
    /// Creates a simulated device from an existing canonical descriptor.
    #[must_use]
    pub fn new(
        descriptor: HardwareDeviceDescriptor,
    ) -> Self {
        Self {
            descriptor,
            interfaces: BTreeMap::new(),
            resource_observations: Vec::new(),
        }
    }

    /// Returns the canonical device descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &HardwareDeviceDescriptor {
        &self.descriptor
    }

    /// Returns associated simulated interface descriptions.
    #[must_use]
    pub fn interfaces(
        &self,
    ) -> impl Iterator<Item = &HardwareInterfaceDescriptor> {
        self.interfaces.values()
    }

    /// Returns associated simulated resource observations.
    ///
    /// These observations are not physical measurements.
    #[must_use]
    pub fn resource_observations(
        &self,
    ) -> &[ResourceObservation] {
        &self.resource_observations
    }

    /// Adds a device-scoped interface description.
    ///
    /// The interface must belong to this device.
    ///
    /// # Errors
    ///
    /// Returns a contract error for a foreign interface or duplicate
    /// interface identity.
    pub fn with_interface(
        mut self,
        interface: HardwareInterfaceDescriptor,
    ) -> ZhalResult<Self> {
        if interface.device() != self.descriptor.identity() {
            return Err(simulation_error(
                ZhalErrorKind::ContractViolation,
                "simulated interface belongs to another device",
            ));
        }

        let identity = interface.identity().clone();

        if self.interfaces.contains_key(&identity) {
            return Err(simulation_error(
                ZhalErrorKind::ContractViolation,
                "duplicate simulated interface identity",
            ));
        }

        self.interfaces.insert(identity, interface);

        Ok(self)
    }

    /// Adds simulated resource observations.
    ///
    /// These observations are retained as supplied by the canonical
    /// resource model. They do not allocate or reserve anything.
    #[must_use]
    pub fn with_resource_observations(
        mut self,
        observations: impl IntoIterator<Item = ResourceObservation>,
    ) -> Self {
        self.resource_observations.extend(observations);
        self
    }

    /// Validates the device and all its interfaces.
    ///
    /// # Errors
    ///
    /// Returns an error when a device or interface contract is invalid.
    pub fn validate(&self) -> ZhalResult<()> {
        self.descriptor.validate().map_err(|error| {
            simulation_error(
                ZhalErrorKind::InvalidData,
                format!("invalid simulated device: {error}"),
            )
        })?;

        for interface in self.interfaces.values() {
            if interface.device() != self.descriptor.identity() {
                return Err(simulation_error(
                    ZhalErrorKind::ContractViolation,
                    "simulated interface has an inconsistent device identity",
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// SIMULATION CONFIGURATION
// ============================================================================

/// Configuration for one simulation-provider instance.
///
/// This is provider-specific configuration, not a replacement for
/// canonical ZHAL configuration or runtime policy.
///
/// The default discovery page size is explicitly supplied rather than
/// hidden behind a universal device-count assumption.
#[derive(Clone, Debug)]
pub struct SimulationProviderConfig {
    /// Canonical provider descriptor.
    pub descriptor: ProviderDescriptor,

    /// Evidence-backed simulated capability observations.
    pub capabilities: CapabilitySnapshot,

    /// Initial simulated hardware inventory.
    pub devices: Vec<SimulatedDevice>,

    /// Default operational discovery page size.
    pub discovery_page_size: NonZeroUsize,

    /// Modeled latency returned by simulated operation reports.
    pub modeled_latency: Duration,
}

impl SimulationProviderConfig {
    /// Creates a simulation-provider configuration.
    #[must_use]
    pub fn new(
        descriptor: ProviderDescriptor,
        capabilities: CapabilitySnapshot,
        devices: impl IntoIterator<Item = SimulatedDevice>,
        discovery_page_size: NonZeroUsize,
        modeled_latency: Duration,
    ) -> Self {
        Self {
            descriptor,
            capabilities,
            devices: devices.into_iter().collect(),
            discovery_page_size,
            modeled_latency,
        }
    }
}

// ============================================================================
// FAULT INJECTION
// ============================================================================

/// Provider operation boundary at which a simulated fault is injected.
///
/// These are implementation-level testing boundaries, not hardware
/// technology categories.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SimulationFaultPoint {
    /// Capability observation.
    CapabilityObservation,

    /// Device discovery.
    Discovery,

    /// Device inspection.
    Inspection,

    /// Interface inspection.
    InterfaceDescription,

    /// Hardware session creation.
    SessionCreation,

    /// Simulated operation evaluation.
    OperationSimulation,
}

/// A configurable simulated provider failure.
///
/// Partial completion and unknown outcomes are represented separately
/// from ordinary failures.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SimulationFault {
    /// A simulated hardware fault.
    HardwareFault {
        /// Explanatory message.
        message: String,
    },

    /// A simulated communication failure.
    CommunicationFailure {
        /// Explanatory message.
        message: String,
    },

    /// A simulated provider implementation failure.
    ProviderFailure {
        /// Explanatory message.
        message: String,
    },

    /// A simulated partial completion.
    PartialCompletion {
        /// Completed work quantity, represented without a fixed integer
        /// width.
        completed: String,

        /// Requested work quantity, represented without a fixed integer
        /// width.
        requested: String,
    },

    /// A simulated operation with an unknown final outcome.
    OutcomeUnknown {
        /// Explanatory message.
        message: String,
    },
}

impl SimulationFault {
    fn into_error(
        self,
        provider_id: &ProviderId,
    ) -> ZhalError {
        let (kind, message) = match self {
            Self::HardwareFault { message } => {
                (ZhalErrorKind::HardwareFault, message)
            }

            Self::CommunicationFailure { message } => {
                (ZhalErrorKind::HardwareCommunication, message)
            }

            Self::ProviderFailure { message } => {
                (ZhalErrorKind::ProviderFailure, message)
            }

            Self::PartialCompletion {
                completed,
                requested,
            } => (
                ZhalErrorKind::PartialCompletion,
                format!(
                    "simulated partial completion: {completed} of {requested}"
                ),
            ),

            Self::OutcomeUnknown { message } => {
                (ZhalErrorKind::OutcomeUnknown, message)
            }
        };

        simulation_error(kind, message)
            .with_provider(provider_id.clone())
    }
}

// ============================================================================
// SIMULATED OPERATION OUTCOMES
// ============================================================================

/// Result classification for a simulated operation.
///
/// A completed simulation is not a physical execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SimulatedOutcome {
    /// The simulated scenario completed.
    Completed,

    /// The simulated scenario completed only partially.
    PartiallyCompleted {
        /// Completed work quantity.
        completed: String,

        /// Requested work quantity.
        requested: String,
    },

    /// The simulated scenario failed.
    Failed {
        /// Failure description.
        reason: String,
    },

    /// The final simulated outcome is unknown.
    Unknown {
        /// Explanation.
        reason: String,
    },
}

/// A report describing a simulated operation scenario.
///
/// The operation identity is reused from the canonical provider API.
/// This report does not define or execute a universal instruction set.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimulatedOperationReport {
    provider_id: ProviderId,
    operation_id: ProviderOperationId,
    outcome: SimulatedOutcome,
    modeled_latency: Duration,
}

impl SimulatedOperationReport {
    /// Returns the provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the operation contract identity.
    #[must_use]
    pub fn operation_id(&self) -> &ProviderOperationId {
        &self.operation_id
    }

    /// Returns the simulated outcome.
    #[must_use]
    pub fn outcome(&self) -> &SimulatedOutcome {
        &self.outcome
    }

    /// Returns the modeled latency.
    ///
    /// This is not a physical measurement or wall-clock guarantee.
    #[must_use]
    pub const fn modeled_latency(&self) -> Duration {
        self.modeled_latency
    }

    /// Always returns false.
    ///
    /// This prevents a simulation report from being interpreted
    /// as proof of physical hardware execution.
    #[must_use]
    pub const fn is_physical_execution(&self) -> bool {
        false
    }
}

// ============================================================================
// INTERNAL SIMULATION STATE
// ============================================================================

struct SimulationState {
    status: ProviderStatus,
    capabilities: CapabilitySnapshot,

    devices: BTreeMap<
        HardwareDeviceIdentity,
        SimulatedDevice,
    >,

    faults: BTreeMap<
        SimulationFaultPoint,
        VecDeque<SimulationFault>,
    >,

    inventory_revision: String,
    modeled_latency: Duration,
}

impl SimulationState {
    fn bump_revision(&mut self) {
        increment_decimal(&mut self.inventory_revision);
    }
}

// ============================================================================
// SIMULATION PROVIDER
// ============================================================================

/// Thread-safe simulated hardware provider.
///
/// Implements the canonical standard-library Provider,
/// ProviderLifecycle and ZHAL HardwareProvider contracts.
///
/// It can describe and simulate hardware observations without
/// requiring physical hardware.
pub struct SimulationProvider {
    descriptor: ProviderDescriptor,
    discovery_page_size: NonZeroUsize,
    state: RwLock<SimulationState>,
    session_sequence: Mutex<String>,
}

impl fmt::Debug for SimulationProvider {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let state = self.state.read();

        formatter
            .debug_struct("SimulationProvider")
            .field("provider_id", self.descriptor.id())
            .field(
                "status",
                &state.as_ref().map(|state| state.status),
            )
            .field(
                "device_count",
                &state.as_ref().map(|state| state.devices.len()),
            )
            .finish()
    }
}

impl SimulationProvider {
    /// Constructs and validates a simulation provider.
    ///
    /// Construction performs no hardware discovery, network access,
    /// physical probing or background-thread creation.
    ///
    /// The provider begins in the Stopped state and must be initialized
    /// through ProviderLifecycle before it accepts operations.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid devices, duplicate identities,
    /// foreign interfaces or undeclared capability observations.
    pub fn new(
        config: SimulationProviderConfig,
    ) -> ZhalResult<Self> {
        let provider_id = config.descriptor.id().clone();

        let mut devices = BTreeMap::new();

        for device in config.devices {
            device.validate()?;

            if device.descriptor().provider_id() != &provider_id {
                return Err(simulation_error(
                    ZhalErrorKind::ContractViolation,
                    "simulated device belongs to another provider",
                )
                .with_provider(provider_id.clone()));
            }

            let identity = device.descriptor().identity().clone();

            if devices.insert(identity, device).is_some() {
                return Err(simulation_error(
                    ZhalErrorKind::ContractViolation,
                    "duplicate simulated device identity",
                )
                .with_provider(provider_id.clone()));
            }
        }

        for (id, _) in config.capabilities.iter() {
            if config.descriptor.capability(id).is_none() {
                return Err(simulation_error(
                    ZhalErrorKind::ContractViolation,
                    format!(
                        "simulated capability {id} is not declared \
                         by the provider descriptor"
                    ),
                )
                .with_provider(provider_id.clone()));
            }
        }

        Ok(Self {
            descriptor: config.descriptor,
            discovery_page_size: config.discovery_page_size,
            state: RwLock::new(SimulationState {
                status: ProviderStatus::Stopped,
                capabilities: config.capabilities,
                devices,
                faults: BTreeMap::new(),
                inventory_revision: String::from("1"),
                modeled_latency: config.modeled_latency,
            }),
            session_sequence: Mutex::new(String::from("0")),
        })
    }

    /// Returns the canonical provider descriptor.
    #[must_use]
    pub fn provider_descriptor(&self) -> &ProviderDescriptor {
        &self.descriptor
    }

    /// Returns the current number of simulated devices.
    ///
    /// This is an observation, not a capacity limit.
    pub fn device_count(&self) -> ZhalResult<usize> {
        let state = self.state.read().map_err(|_| lock_error())?;
        Ok(state.devices.len())
    }

    /// Returns the current inventory revision.
    ///
    /// A revision changes whenever the simulated inventory or
    /// capability snapshot changes.
    pub fn inventory_revision(&self) -> ZhalResult<String> {
        let state = self.state.read().map_err(|_| lock_error())?;
        Ok(state.inventory_revision.clone())
    }

    /// Registers or replaces a simulated device.
    ///
    /// Replacement is explicit through this method.
    /// Inventory revision changes after successful registration.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid devices or a foreign provider.
    pub fn register_device(
        &self,
        device: SimulatedDevice,
    ) -> ZhalResult<Option<SimulatedDevice>> {
        device.validate()?;

        let provider_id = self.descriptor.id();

        if device.descriptor().provider_id() != provider_id {
            return Err(simulation_error(
                ZhalErrorKind::ContractViolation,
                "cannot register a device owned by another provider",
            )
            .with_provider(provider_id.clone()));
        }

        let identity = device.descriptor().identity().clone();

        let mut state = self.state.write().map_err(|_| lock_error())?;

        let previous = state.devices.insert(identity, device);

        state.bump_revision();

        Ok(previous)
    }

    /// Removes a simulated device from the provider inventory.
    ///
    /// Existing sessions are not represented as physical device handles
    /// and are not claimed to be terminated by inventory removal.
    pub fn remove_device(
        &self,
        identity: &HardwareDeviceIdentity,
    ) -> ZhalResult<Option<SimulatedDevice>> {
        if identity.provider_id() != self.descriptor.id() {
            return Err(simulation_error(
                ZhalErrorKind::InvalidInput,
                "device belongs to another provider",
            ));
        }

        let mut state = self.state.write().map_err(|_| lock_error())?;

        let removed = state.devices.remove(identity);

        if removed.is_some() {
            state.bump_revision();
        }

        Ok(removed)
    }

    /// Updates the observed availability of a simulated device.
    ///
    /// This operation does not establish authorization or reservation.
    ///
    /// # Errors
    ///
    /// Returns an error if the device is not in this provider's inventory.
    pub fn set_device_availability(
        &self,
        identity: &HardwareDeviceIdentity,
        availability: DeviceAvailability,
    ) -> ZhalResult<()> {
        if identity.provider_id() != self.descriptor.id() {
            return Err(simulation_error(
                ZhalErrorKind::InvalidInput,
                "device belongs to another provider",
            ));
        }

        let mut state = self.state.write().map_err(|_| lock_error())?;

        let device = state.devices.get_mut(identity).ok_or_else(|| {
            simulation_error(
                ZhalErrorKind::DeviceNotFound,
                "simulated device was not found",
            )
        })?;

        let updated = device
            .descriptor
            .clone()
            .with_availability(availability)
            .map_err(|error| {
                simulation_error(
                    ZhalErrorKind::InvalidData,
                    error.to_string(),
                )
            })?;

        device.descriptor = updated;

        state.bump_revision();

        Ok(())
    }

    /// Replaces the simulated capability snapshot.
    ///
    /// The snapshot must refer only to capabilities declared
    /// by the immutable provider descriptor.
    ///
    /// # Errors
    ///
    /// Returns an error if the snapshot contains undeclared capabilities.
    pub fn replace_capabilities(
        &self,
        snapshot: CapabilitySnapshot,
    ) -> ZhalResult<()> {
        for (id, _) in snapshot.iter() {
            if self.descriptor.capability(id).is_none() {
                return Err(simulation_error(
                    ZhalErrorKind::ContractViolation,
                    format!(
                        "capability {id} is not declared by this provider"
                    ),
                ));
            }
        }

        let mut state = self.state.write().map_err(|_| lock_error())?;

        state.capabilities = snapshot;
        state.bump_revision();

        Ok(())
    }

    /// Changes the modeled latency.
    ///
    /// This changes future simulation reports. It does not block
    /// or delay the calling thread.
    pub fn set_modeled_latency(
        &self,
        latency: Duration,
    ) -> ZhalResult<()> {
        let mut state = self.state.write().map_err(|_| lock_error())?;

        state.modeled_latency = latency;

        Ok(())
    }

    /// Schedules a fault for the next matching provider operation.
    ///
    /// Faults are consumed in insertion order for each fault point.
    ///
    /// # Errors
    ///
    /// Returns an error if the simulation state cannot be accessed.
    pub fn inject_fault(
        &self,
        point: SimulationFaultPoint,
        fault: SimulationFault,
    ) -> ZhalResult<()> {
        let mut state = self.state.write().map_err(|_| lock_error())?;

        state
            .faults
            .entry(point)
            .or_default()
            .push_back(fault);

        Ok(())
    }

    /// Clears pending faults at the specified operation boundary.
    pub fn clear_faults(
        &self,
        point: SimulationFaultPoint,
    ) -> ZhalResult<usize> {
        let mut state = self.state.write().map_err(|_| lock_error())?;

        Ok(state
            .faults
            .remove(&point)
            .map_or(0, |faults| faults.len()))
    }

    /// Evaluates a generic simulated operation scenario.
    ///
    /// This method is a simulation and testing facility.
    ///
    /// It does not execute a domain-specific instruction, quantum
    /// circuit, HDL realization or physical operation.
    ///
    /// # Errors
    ///
    /// Returns a structured provider error when the scenario fails.
    pub fn simulate_operation(
        &self,
        operation: ProviderOperationId,
        context: &ZhalContext,
    ) -> ZhalResult<SimulatedOperationReport> {
        check_context(context)?;
        self.ensure_ready()?;

        let fault = self.take_fault(
            SimulationFaultPoint::OperationSimulation,
        )?;

        let (modeled_latency, outcome) = {
            let state = self.state.read().map_err(|_| lock_error())?;

            let outcome = match fault {
                None => SimulatedOutcome::Completed,

                Some(SimulationFault::PartialCompletion {
                    completed,
                    requested,
                }) => SimulatedOutcome::PartiallyCompleted {
                    completed,
                    requested,
                },

                Some(SimulationFault::OutcomeUnknown { message }) => {
                    SimulatedOutcome::Unknown {
                        reason: message,
                    }
                }

                Some(SimulationFault::HardwareFault { message }) => {
                    SimulatedOutcome::Failed {
                        reason: message,
                    }
                }

                Some(other) => {
                    return Err(other.into_error(self.descriptor.id()));
                }
            };

            (state.modeled_latency, outcome)
        };

        check_context(context)?;

        Ok(SimulatedOperationReport {
            provider_id: self.descriptor.id().clone(),
            operation_id: operation,
            outcome,
            modeled_latency,
        })
    }

    fn ensure_ready(&self) -> ZhalResult<()> {
        let state = self.state.read().map_err(|_| lock_error())?;

        if !matches!(
            state.status,
            ProviderStatus::Ready | ProviderStatus::Degraded
        ) {
            return Err(simulation_error(
                ZhalErrorKind::ProviderUnavailable,
                format!(
                    "simulation provider is not ready: {}",
                    state.status
                ),
            )
            .with_provider(self.descriptor.id().clone()));
        }

        Ok(())
    }

    fn take_fault(
        &self,
        point: SimulationFaultPoint,
    ) -> ZhalResult<Option<SimulationFault>> {
        let mut state = self.state.write().map_err(|_| lock_error())?;

        Ok(state
            .faults
            .get_mut(&point)
            .and_then(VecDeque::pop_front))
    }

    fn next_session_identity(
        &self,
    ) -> ZhalResult<HardwareSessionIdentity> {
        let mut sequence = self
            .session_sequence
            .lock()
            .map_err(|_| lock_error())?;

        increment_decimal(&mut sequence);

        let session_id = HardwareSessionId::new(
            format!("simulation-session-{sequence}"),
        )
        .map_err(|error| {
            simulation_error(
                ZhalErrorKind::Internal,
                error.to_string(),
            )
        })?;

        Ok(HardwareSessionIdentity::new(
            self.descriptor.id().clone(),
            session_id,
        ))
    }

    fn take_fault_or_error(
        &self,
        point: SimulationFaultPoint,
    ) -> ZhalResult<()> {
        if let Some(fault) = self.take_fault(point)? {
            return Err(fault.into_error(self.descriptor.id()));
        }

        Ok(())
    }
}

// ============================================================================
// STANDARD PROVIDER CONTRACT
// ============================================================================

impl Provider for SimulationProvider {
    fn descriptor(&self) -> &ProviderDescriptor {
        &self.descriptor
    }

    fn status(&self) -> ProviderStatus {
        self.state
            .read()
            .map(|state| state.status)
            .unwrap_or(ProviderStatus::Failed)
    }

    fn capability_snapshot<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, CapabilitySnapshot> {
        Box::pin(async move {
            context.check_cancelled().map_err(context_error)?;

            self.ensure_ready()?;

            self.take_fault_or_error(
                SimulationFaultPoint::CapabilityObservation,
            )?;

            let snapshot = {
                let state = self.state.read().map_err(|_| lock_error())?;
                state.capabilities.clone()
            };

            context.check_cancelled().map_err(context_error)?;

            Ok(snapshot)
        })
    }
}

// ============================================================================
// PROVIDER LIFECYCLE
// ============================================================================

impl ProviderLifecycle for SimulationProvider {
    fn initialize<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, ()> {
        Box::pin(async move {
            context.check_cancelled().map_err(context_error)?;

            let mut state = self.state.write().map_err(|_| lock_error())?;

            match state.status {
                ProviderStatus::Failed => {
                    return Err(simulation_error(
                        ZhalErrorKind::ProviderFailure,
                        "failed simulation provider cannot be initialized",
                    )
                    .with_provider(self.descriptor.id().clone()));
                }

                ProviderStatus::Stopping => {
                    return Err(simulation_error(
                        ZhalErrorKind::ConcurrencyConflict,
                        "simulation provider is stopping",
                    ));
                }

                _ => {}
            }

            state.status = ProviderStatus::Ready;

            Ok(())
        })
    }

    fn shutdown<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, ()> {
        Box::pin(async move {
            context.check_cancelled().map_err(context_error)?;

            let mut state = self.state.write().map_err(|_| lock_error())?;

            state.status = ProviderStatus::Stopped;

            Ok(())
        })
    }
}

// ============================================================================
// HARDWARE SESSION
// ============================================================================

struct SimulatedHardwareSession {
    identity: HardwareSessionIdentity,
    device_identity: HardwareDeviceIdentity,
    closed: AtomicBool,
}

impl HardwareSession for SimulatedHardwareSession {
    fn identity(&self) -> &HardwareSessionIdentity {
        &self.identity
    }

    fn device_identity(&self) -> &HardwareDeviceIdentity {
        &self.device_identity
    }

    fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Acquire)
    }

    fn close<'a>(
        &'a mut self,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, ()> {
        Box::pin(async move {
            if self.is_closed() {
                return Ok(());
            }

            // Local simulated cleanup must not be skipped simply because
            // the caller has requested cancellation.
            self.closed.store(true, Ordering::Release);

            check_context(context)
        })
    }
}

// ============================================================================
// UNIVERSAL HARDWARE PROVIDER CONTRACT
// ============================================================================

impl HardwareProvider for SimulationProvider {
    fn discover_devices_page<'a>(
        &'a self,
        request: &'a DeviceDiscoveryRequest,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, DeviceDiscoveryPage> {
        Box::pin(async move {
            check_context(context)?;
            self.ensure_ready()?;

            self.take_fault_or_error(
                SimulationFaultPoint::Discovery,
            )?;

            let (devices, next_cursor, revision) = {
                let state = self.state.read().map_err(|_| lock_error())?;

                let start = match request.cursor() {
                    None => 0,

                    Some(cursor) => {
                        let mut parts = cursor.as_str().splitn(3, '|');

                        let provider = parts.next();
                        let revision = parts.next();
                        let offset = parts.next();

                        if provider != Some(self.descriptor.id().as_str()) {
                            return Err(simulation_error(
                                ZhalErrorKind::InvalidInput,
                                "discovery cursor belongs to another provider",
                            ));
                        }

                        if revision != Some(state.inventory_revision.as_str()) {
                            return Err(simulation_error(
                                ZhalErrorKind::ConcurrencyConflict,
                                "discovery cursor references a stale inventory revision",
                            ));
                        }

                        offset
                            .and_then(|value| value.parse::<usize>().ok())
                            .ok_or_else(|| {
                                simulation_error(
                                    ZhalErrorKind::InvalidInput,
                                    "discovery cursor contains an invalid offset",
                                )
                            })?
                    }
                };

                if start > state.devices.len() {
                    return Err(simulation_error(
                        ZhalErrorKind::InvalidInput,
                        "discovery cursor exceeds the current inventory",
                    ));
                }

                let page_size = request
                    .page_size()
                    .unwrap_or(self.discovery_page_size)
                    .get();

                let end = start.saturating_add(page_size)
                    .min(state.devices.len());

                let devices = state
                    .devices
                    .values()
                    .skip(start)
                    .take(end.saturating_sub(start))
                    .map(|device| device.descriptor.clone())
                    .collect::<Vec<_>>();

                let next_cursor = if end < state.devices.len() {
                    Some(
                        DeviceDiscoveryCursor::new(
                            format!(
                                "{}|{}|{}",
                                self.descriptor.id(),
                                state.inventory_revision,
                                end,
                            ),
                        )?
                    )
                } else {
                    None
                };

                (
                    devices,
                    next_cursor,
                    state.inventory_revision.clone(),
                )
            };

            check_context(context)?;

            DeviceDiscoveryPage::new(
                devices,
                next_cursor,
                Some(revision),
            )
        })
    }

    fn inspect_device<'a>(
        &'a self,
        identity: &'a HardwareDeviceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, HardwareDeviceDescriptor> {
        Box::pin(async move {
            check_context(context)?;
            self.ensure_ready()?;

            if identity.provider_id() != self.descriptor.id() {
                return Err(simulation_error(
                    ZhalErrorKind::InvalidInput,
                    "requested device belongs to another provider",
                ));
            }

            self.take_fault_or_error(
                SimulationFaultPoint::Inspection,
            )?;

            let descriptor = {
                let state = self.state.read().map_err(|_| lock_error())?;

                state.devices
                    .get(identity)
                    .map(|device| device.descriptor.clone())
                    .ok_or_else(|| {
                        simulation_error(
                            ZhalErrorKind::DeviceNotFound,
                            "simulated device was not found",
                        )
                        .with_device(identity.clone())
                    })?
            };

            check_context(context)?;

            Ok(descriptor)
        })
    }

    fn describe_interface<'a>(
        &'a self,
        identity: &'a HardwareInterfaceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, HardwareInterfaceDescriptor> {
        Box::pin(async move {
            check_context(context)?;
            self.ensure_ready()?;

            if identity.device().provider_id() != self.descriptor.id() {
                return Err(simulation_error(
                    ZhalErrorKind::InvalidInput,
                    "interface belongs to another provider",
                ));
            }

            self.take_fault_or_error(
                SimulationFaultPoint::InterfaceDescription,
            )?;

            let descriptor = {
                let state = self.state.read().map_err(|_| lock_error())?;

                let device = state
                    .devices
                    .get(identity.device())
                    .ok_or_else(|| {
                        simulation_error(
                            ZhalErrorKind::DeviceNotFound,
                            "interface's simulated device was not found",
                        )
                    })?;

                device.interfaces
                    .get(identity)
                    .cloned()
                    .ok_or_else(|| {
                        simulation_error(
                            ZhalErrorKind::DeviceNotFound,
                            "simulated interface was not found",
                        )
                    })?
            };

            check_context(context)?;

            Ok(descriptor)
        })
    }

    fn open_session<'a>(
        &'a self,
        device: &'a HardwareDeviceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, Box<dyn HardwareSession>> {
        Box::pin(async move {
            check_context(context)?;
            self.ensure_ready()?;

            if device.provider_id() != self.descriptor.id() {
                return Err(simulation_error(
                    ZhalErrorKind::InvalidInput,
                    "requested device belongs to another provider",
                ));
            }

            self.take_fault_or_error(
                SimulationFaultPoint::SessionCreation,
            )?;

            {
                let state = self.state.read().map_err(|_| lock_error())?;

                let descriptor = state.devices
                    .get(device)
                    .map(|entry| &entry.descriptor)
                    .ok_or_else(|| {
                        simulation_error(
                            ZhalErrorKind::DeviceNotFound,
                            "simulated device was not found",
                        )
                    })?;

                if !descriptor.availability().is_available() {
                    return Err(simulation_error(
                        ZhalErrorKind::DeviceUnavailable,
                        "simulated device availability has not been established",
                    )
                    .with_device(device.clone()));
                }
            }

            check_context(context)?;

            let session_identity = self.next_session_identity()?;

            let session = SimulatedHardwareSession {
                identity: session_identity,
                device_identity: device.clone(),
                closed: AtomicBool::new(false),
            };

            Ok(Box::new(session) as Box<dyn HardwareSession>)
        })
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::capability::{
        Capability,
        CapabilityId,
        CapabilityKind,
        CapabilityState,
        CapabilityStatus,
    };

    use crate::stdlib::context::OperationContext;

    use crate::stdlib::provider::{
        ProviderKind,
    };

    use crate::stdlib::version::Version;

    use crate::zhal::identity::{
        HardwareDeviceIdentity,
    };

    fn provider_id() -> ProviderId {
        ProviderId::new("test.simulation.provider")
            .expect("valid provider identity")
    }

    fn descriptor() -> ProviderDescriptor {
        ProviderDescriptor::new(
            provider_id(),
            ProviderKind::new("simulation")
                .expect("valid provider kind"),
            "Zamani Simulation Provider",
            Version::parse("1.0.0")
                .expect("valid implementation version"),
            Version::parse("1.0.0")
                .expect("valid API version"),
        )
        .expect("valid provider descriptor")
    }

    fn device() -> SimulatedDevice {
        let identity = HardwareDeviceIdentity::new(
            provider_id(),
            DeviceId::new("device.test")
                .expect("valid device ID"),
        );

        let descriptor = HardwareDeviceDescriptor::new(
            identity,
            DeviceKindId::new("test.hardware.device")
                .expect("valid device kind"),
            "Simulated test device",
        )
        .expect("valid device descriptor");

        SimulatedDevice::new(descriptor)
    }

    fn configuration() -> SimulationProviderConfig {
        SimulationProviderConfig::new(
            descriptor(),
            CapabilitySnapshot::new(),
            [device()],
            NonZeroUsize::new(1)
                .expect("nonzero page size"),
            Duration::ZERO,
        )
    }

    fn provider() -> SimulationProvider {
        SimulationProvider::new(configuration())
            .expect("valid simulation provider")
    }

    fn initialize(provider: &SimulationProvider) {
        let context = OperationContext::default();

        let mut future = ProviderLifecycle::initialize(
            provider,
            &context,
        );

        use std::future::Future;
        use std::sync::Arc;
        use std::task::{Context, Wake, Waker};

        struct NoopWake;

        impl Wake for NoopWake {
            fn wake(self: Arc<Self>) {}
        }

        let waker = Waker::from(Arc::new(NoopWake));
        let mut context = Context::from_waker(&waker);

        assert!(matches!(
            future.as_mut().poll(&mut context),
            std::task::Poll::Ready(Ok(()))
        ));
    }

    #[test]
    fn provider_starts_stopped() {
        let value = provider();

        assert_eq!(
            value.status(),
            ProviderStatus::Stopped,
        );
    }

    #[test]
    fn initialization_makes_provider_ready() {
        let value = provider();

        initialize(&value);

        assert_eq!(
            value.status(),
            ProviderStatus::Ready,
        );
    }

    #[test]
    fn initial_inventory_is_registered() {
        let value = provider();

        assert_eq!(
            value.device_count().expect("inventory readable"),
            1,
        );
    }

    #[test]
    fn inventory_revision_is_versioned() {
        let value = provider();

        let first = value.inventory_revision()
            .expect("revision available");

        value.register_device(device())
            .expect("device replacement succeeds");

        let second = value.inventory_revision()
            .expect("revision available");

        assert_ne!(first, second);
    }

    #[test]
    fn inventory_accepts_future_device_categories() {
        let identity = HardwareDeviceIdentity::new(
            provider_id(),
            DeviceId::new("future.device")
                .expect("valid ID"),
        );

        let descriptor = HardwareDeviceDescriptor::new(
            identity,
            DeviceKindId::new(
                "future.photonic.computing.device"
            )
            .expect("future category accepted"),
            "Future photonic device",
        )
        .expect("valid descriptor");

        let simulated = SimulatedDevice::new(descriptor);

        let value = provider();

        value.register_device(simulated)
            .expect("future device accepted");

        assert_eq!(
            value.device_count().unwrap(),
            2,
        );
    }

    #[test]
    fn device_removal_updates_inventory() {
        let value = provider();

        let identity = device().descriptor.identity().clone();

        let removed = value.remove_device(&identity)
            .expect("removal succeeds");

        assert!(removed.is_some());

        assert_eq!(
            value.device_count().unwrap(),
            0,
        );
    }

    #[test]
    fn decimal_sequence_has_no_fixed_integer_width() {
        let mut value = String::from(
            "999999999999999999999999999999999999999"
        );

        increment_decimal(&mut value);

        assert_eq!(
            value,
            "1000000000000000000000000000000000000000",
        );
    }

    #[test]
    fn capability_declarations_are_validated() {
        let mut declarations = descriptor();

        declarations.insert_capability(
            Capability::new(
                CapabilityId::new("simulation.test")
                    .expect("valid capability"),
                CapabilityKind::new("simulation")
                    .expect("valid kind"),
            ),
        )
        .expect("capability declaration");

        let mut snapshot = CapabilitySnapshot::new();

        snapshot.insert(
            CapabilityState::new(
                Capability::new(
                    CapabilityId::new("simulation.test")
                        .expect("valid capability"),
                    CapabilityKind::new("simulation")
                        .expect("valid kind"),
                ),
                CapabilityStatus::Available,
            ),
        )
        .expect("valid snapshot");

        let config = SimulationProviderConfig::new(
            declarations,
            snapshot,
            [],
            NonZeroUsize::new(2).unwrap(),
            Duration::ZERO,
        );

        assert!(SimulationProvider::new(config).is_ok());
    }

    #[test]
    fn undeclared_capability_observations_are_rejected() {
        let mut snapshot = CapabilitySnapshot::new();

        snapshot.insert(
            CapabilityState::new(
                Capability::new(
                    CapabilityId::new("future.undeclared")
                        .expect("valid capability"),
                    CapabilityKind::new("computation")
                        .expect("valid kind"),
                ),
                CapabilityStatus::Available,
            ),
        )
        .expect("valid snapshot");

        let config = SimulationProviderConfig::new(
            descriptor(),
            snapshot,
            [],
            NonZeroUsize::new(1).unwrap(),
            Duration::ZERO,
        );

        assert!(SimulationProvider::new(config).is_err());
    }

    #[test]
    fn multiple_faults_can_be_queued() {
        let value = provider();

        value.inject_fault(
            SimulationFaultPoint::Discovery,
            SimulationFault::CommunicationFailure {
                message: "simulated network failure".to_owned(),
            },
        )
        .expect("fault injection");

        value.inject_fault(
            SimulationFaultPoint::Discovery,
            SimulationFault::HardwareFault {
                message: "simulated device failure".to_owned(),
            },
        )
        .expect("fault injection");

        assert_eq!(
            value.clear_faults(SimulationFaultPoint::Discovery)
                .unwrap(),
            2,
        );
    }

    #[test]
    fn modeled_latency_is_configurable() {
        let value = provider();

        value.set_modeled_latency(
            Duration::from_millis(250),
        )
        .expect("latency configuration");

        let state = value.state.read().unwrap();

        assert_eq!(
            state.modeled_latency,
            Duration::from_millis(250),
        );
    }

    #[test]
    fn simulation_report_cannot_claim_physical_execution() {
        let report = SimulatedOperationReport {
            provider_id: provider_id(),
            operation_id: ProviderOperationId::new(
                "simulation.test.operation"
            )
            .expect("valid operation"),
            outcome: SimulatedOutcome::Completed,
            modeled_latency: Duration::from_millis(10),
        };

        assert!(!report.is_physical_execution());
    }

    #[test]
    fn partial_completion_is_distinct_from_completion() {
        let outcome = SimulatedOutcome::PartiallyCompleted {
            completed: "7".to_owned(),
            requested: "10".to_owned(),
        };

        assert_ne!(
            outcome,
            SimulatedOutcome::Completed,
        );
    }

    #[test]
    fn unknown_outcome_is_distinct_from_failure() {
        let unknown = SimulatedOutcome::Unknown {
            reason: "outcome could not be established".to_owned(),
        };

        let failed = SimulatedOutcome::Failed {
            reason: "hardware fault".to_owned(),
        };

        assert_ne!(unknown, failed);
    }

    #[test]
    fn provider_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<SimulationProvider>();
        assert_send_sync::<SimulatedDevice>();
    }

    #[test]
    fn schema_identity_is_stable() {
        assert_eq!(
            SIMULATION_PROVIDER_SCHEMA_ID,
            "zamani.zhal.simulation.provider",
        );

        assert_eq!(
            SIMULATION_PROVIDER_SCHEMA_VERSION,
            "1",
        );
    }
}
