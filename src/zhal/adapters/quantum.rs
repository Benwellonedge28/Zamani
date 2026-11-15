
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Quantum Hardware Adapter
//!
//! Path: `src/zhal/adapters/quantum.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the integration boundary between the universal ZHAL hardware
//! provider architecture and the existing authoritative quantum hardware
//! execution architecture.
//!
//! This module composes:
//!
//! - The canonical ZHAL HardwareProvider contract.
//! - The canonical standard-library Provider contract.
//! - The canonical standard-library ProviderLifecycle contract.
//! - The canonical quantum QuantumBackendAdapter contract.
//! - The canonical quantum QuantumBackend descriptor.
//! - The canonical ZHAL hardware-device identity.
//!
//! It does not introduce another quantum backend, provider registry,
//! hardware inventory, capability authority, execution engine or IR.
//!
//! # Architectural ownership
//!
//! ZHAL owns:
//!
//! - Universal hardware-provider integration.
//! - Generic device discovery and inspection contracts.
//! - Generic hardware interface descriptions.
//! - Provider-managed hardware sessions.
//! - Provider-independent hardware identity composition.
//!
//! Quantum hardware owns:
//!
//! - Quantum backend descriptions.
//! - Quantum capabilities.
//! - Quantum workload compatibility.
//! - Quantum execution requests.
//! - Quantum program payloads.
//! - Quantum job lifecycle.
//! - Quantum result normalization.
//! - Quantum cancellation.
//! - Quantum provider-specific execution.
//! - Quantum topology, routing and scheduling.
//! - Quantum calibration.
//!
//! This adapter joins these authorities without transferring ownership.
//!
//! # Composite-provider model
//!
//! A QuantumHardwareAdapter combines:
//!
//! 1. An existing ZHAL HardwareProvider.
//! 2. An existing QuantumBackendAdapter.
//! 3. An explicit binding between a ZHAL device identity and a quantum
//!    backend identity.
//!
//! The generic provider remains responsible for:
//!
//! - Provider identity and metadata.
//! - Provider status.
//! - Capability evidence snapshots.
//! - Device discovery.
//! - Device inspection.
//! - Interface inspection.
//! - Hardware session creation.
//! - Generic provider lifecycle.
//!
//! The quantum adapter remains responsible for:
//!
//! - Quantum preflight.
//! - Quantum submission.
//! - Quantum job status.
//! - Quantum result retrieval.
//! - Quantum cancellation.
//! - Quantum queue information.
//! - Quantum backend health.
//! - Synchronous execution where supported.
//!
//! The quantum backend's provider-specific display metadata is not
//! substituted for the canonical standard-library ProviderId.
//!
//! # Identity binding
//!
//! Quantum backend identifiers and ZHAL device identifiers are distinct
//! identity authorities.
//!
//! The binding is supplied explicitly by the integration owner.
//!
//! This module never guesses that two identifiers refer to the same
//! physical device.
//!
//! The binding validates:
//!
//! - The ZHAL device belongs to the wrapped hardware provider.
//! - The quantum backend identifier matches the supplied backend.
//!
//! # Capability boundary
//!
//! Generic provider capability evidence remains owned by
//! `stdlib::capability`.
//!
//! Quantum backend capabilities remain owned by
//! `quantum::hardware::backend`.
//!
//! This module does not flatten quantum capabilities into generic
//! capabilities or manufacture capability evidence.
//!
//! Consumers can inspect both authorities through this adapter.
//!
//! # Quantum IR boundary
//!
//! The canonical quantum semantic representation remains `quantum::ir`.
//!
//! This adapter receives the already prepared provider-neutral
//! `BackendProgram` and `ExecutionRequest`.
//!
//! It does not parse Zamani source, construct quantum IR, translate
//! OpenQASM, generate QIR, transpile circuits or modify workloads.
//!
//! # Scalability and POCO-REAF
//!
//! This module introduces no universal hardware ceilings.
//!
//! It does not impose fixed limits on:
//!
//! - Quantum backends.
//! - Qubits.
//! - Logical qubits.
//! - Quantum devices.
//! - Providers.
//! - Quantum operations.
//! - Quantum jobs.
//! - Hardware resources.
//! - Quantum technologies.
//! - Quantum workload types.
//! - Provider-defined identifiers.
//!
//! Capacity and feasibility are determined by the actual quantum backend,
//! its validated capabilities, provider observations, applicable policies
//! and available physical resources.
//!
//! Backend-reported physical limits remain legitimate hardware facts.
//!
//! # Failure semantics
//!
//! Errors from the quantum backend are returned unchanged.
//!
//! Errors from the generic hardware provider are returned unchanged.
//!
//! This adapter never:
//!
//! - Converts an execution failure into success.
//! - Manufactures a job identifier.
//! - Claims a result is complete without backend confirmation.
//! - Claims cancellation stopped physical execution.
//! - Claims discovery guarantees future device availability.
//! - Claims a provider session reserves hardware.
//!
//! # Security
//!
//! This adapter stores no credentials.
//!
//! It does not perform authentication or authorization itself.
//!
//! Those operations remain with the existing provider, security and
//! runtime authorities.
//!
//! Debug output intentionally avoids formatting the wrapped providers
//! and their internal state.
//!
//! # Thread safety
//!
//! The wrapped provider and quantum adapter are held through Arc.
//!
//! Both existing contracts require Send + Sync.
//!
//! This module introduces no global mutable state.
//!
//! # Integration
//!
//! Generic provider integration:
//!
//! `crate::stdlib::provider`
//! `crate::stdlib::capability`
//! `crate::stdlib::context`
//! `crate::zhal::providers::contract`
//!
//! Generic hardware identity:
//!
//! `crate::zhal::identity`
//!
//! Quantum execution:
//!
//! `crate::quantum::hardware::backend`
//! `crate::quantum::hardware::backend_trait`
//!
//! Quantum provider adapters:
//!
//! `crate::quantum::hardware::adapters`
//!
//! Runtime access and orchestration:
//!
//! `crate::zhal::devices`
//! `crate::zur`
//!
//! # Completion invariant
//!
//! This file is complete when:
//!
//! 1. It implements the existing Provider contract.
//! 2. It implements the existing HardwareProvider contract.
//! 3. It delegates generic hardware operations without duplicating them.
//! 4. It delegates quantum execution without duplicating it.
//! 5. It implements ProviderLifecycle only when the wrapped provider
//!    implements that existing lifecycle contract.
//! 6. It validates the explicit provider/device/backend binding.
//! 7. It preserves quantum errors and results.
//! 8. It preserves both generic and quantum capability authorities.
//! 9. It introduces no arbitrary hardware ceilings.
//! 10. It contains no unsafe Rust.

use core::fmt;

use std::sync::Arc;

use crate::stdlib::capability::CapabilitySnapshot;

use crate::stdlib::context::OperationContext;

use crate::stdlib::provider::{
    Provider,
    ProviderDescriptor,
    ProviderFuture,
    ProviderLifecycle,
    ProviderStatus,
};

use crate::zhal::context::ZhalContext;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

use crate::zhal::identity::HardwareDeviceIdentity;

use crate::zhal::providers::contract::{
    DeviceDiscoveryPage,
    DeviceDiscoveryRequest,
    HardwareProvider,
    HardwareProviderFuture,
    HardwareSession,
};

use crate::quantum::hardware::backend::{
    ExecutionRequest,
    ExecutionResult,
    QuantumBackend,
};

use crate::quantum::hardware::backend_trait::{
    BackendCancellation,
    BackendHealth,
    BackendJob,
    BackendJobId,
    BackendJobStatus,
    BackendProgram,
    BackendQueueInfo,
    QuantumBackendAdapter,
};

/// Stable component identifier.
pub const COMPONENT: &str = "zhal.adapters.quantum";

/// Quantum adapter integration schema identifier.
pub const QUANTUM_ADAPTER_SCHEMA_ID: &str =
    "zamani.zhal.adapters.quantum";

/// Quantum adapter integration schema version.
pub const QUANTUM_ADAPTER_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// VALIDATION
// ============================================================================

fn invalid_input(
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(
        ZhalErrorKind::InvalidInput,
        message,
    )
    .in_component(COMPONENT)
}

fn validate_backend_id(
    backend_id: &str,
) -> ZhalResult<()> {
    if backend_id.is_empty() {
        return Err(invalid_input(
            "quantum backend identity must not be empty",
        ));
    }

    if backend_id.trim() != backend_id {
        return Err(invalid_input(
            "quantum backend identity must not contain surrounding whitespace",
        ));
    }

    if backend_id.chars().any(|character| {
        character.is_control() || character.is_whitespace()
    }) {
        return Err(invalid_input(
            "quantum backend identity must not contain whitespace or control characters",
        ));
    }

    Ok(())
}

// ============================================================================
// EXPLICIT QUANTUM DEVICE BINDING
// ============================================================================

/// Explicit association between a ZHAL hardware device and a quantum
/// execution backend.
///
/// A ZHAL device identity and a quantum backend identity belong to
/// different canonical identity systems.
///
/// This structure records their integration relationship without
/// merging or redefining either identity.
///
/// The binding does not prove that the physical device is currently
/// available, authorized, calibrated or capable of executing a
/// particular workload.
///
/// Those properties must be established through their respective
/// authorities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuantumDeviceBinding {
    device: HardwareDeviceIdentity,
    backend_id: String,
}

impl QuantumDeviceBinding {
    /// Creates a validated device-to-backend binding.
    ///
    /// The backend identifier must be non-empty and must not contain
    /// whitespace or control characters.
    ///
    /// No arbitrary identifier-length ceiling is introduced here.
    ///
    /// # Errors
    ///
    /// Returns a ZHAL input error when the backend identifier is invalid.
    pub fn new(
        device: HardwareDeviceIdentity,
        backend_id: impl Into<String>,
    ) -> ZhalResult<Self> {
        let backend_id = backend_id.into();

        validate_backend_id(&backend_id)?;

        Ok(Self {
            device,
            backend_id,
        })
    }

    /// Returns the canonical ZHAL device identity.
    #[must_use]
    pub fn device(&self) -> &HardwareDeviceIdentity {
        &self.device
    }

    /// Returns the canonical quantum backend identifier.
    #[must_use]
    pub fn backend_id(&self) -> &str {
        &self.backend_id
    }
}

// ============================================================================
// QUANTUM HARDWARE ADAPTER
// ============================================================================

/// Composite ZHAL provider adapter for an existing quantum backend.
///
/// The adapter combines a generic ZHAL hardware provider with one
/// authoritative quantum execution adapter.
///
/// The generic provider owns physical-device discovery, inspection,
/// interface descriptions, hardware sessions and common provider
/// lifecycle.
///
/// The quantum adapter owns quantum execution and job lifecycle.
///
/// No quantum execution semantics are implemented here.
///
/// # Type parameters
///
/// `P` is the existing ZHAL hardware provider.
///
/// `A` is the existing quantum backend execution adapter.
///
/// Both types may be concrete provider implementations or trait objects.
pub struct QuantumHardwareAdapter<
    P: HardwareProvider + ?Sized,
    A: QuantumBackendAdapter + ?Sized,
> {
    provider: Arc<P>,
    quantum: Arc<A>,
    binding: QuantumDeviceBinding,
}

impl<
        P: HardwareProvider + ?Sized,
        A: QuantumBackendAdapter + ?Sized,
    > QuantumHardwareAdapter<P, A>
{
    /// Constructs a composite quantum hardware adapter.
    ///
    /// Construction performs structural identity validation only.
    ///
    /// It performs no:
    ///
    /// - Hardware discovery.
    /// - Network requests.
    /// - Provider initialization.
    /// - Authentication.
    /// - Authorization.
    /// - Device access.
    /// - Quantum job submission.
    /// - Resource reservation.
    ///
    /// The supplied provider must own the device identity in the binding.
    ///
    /// The supplied quantum adapter must expose the exact backend
    /// identifier recorded in the binding.
    ///
    /// The generic provider and quantum backend remain independently
    /// managed by their existing authorities.
    ///
    /// # Errors
    ///
    /// Returns an error when the device belongs to another provider
    /// or the backend identifier does not match.
    pub fn new(
        provider: Arc<P>,
        quantum: Arc<A>,
        binding: QuantumDeviceBinding,
    ) -> ZhalResult<Self> {
        if provider.descriptor().id()
            != binding.device().provider_id()
        {
            return Err(invalid_input(
                "quantum device binding belongs to a different ZHAL provider",
            ));
        }

        if quantum.backend().id() != binding.backend_id() {
            return Err(invalid_input(
                "quantum device binding references a different quantum backend",
            ));
        }

        Ok(Self {
            provider,
            quantum,
            binding,
        })
    }

    /// Returns the explicit device-to-backend binding.
    #[must_use]
    pub fn binding(&self) -> &QuantumDeviceBinding {
        &self.binding
    }

    /// Returns the wrapped generic hardware provider.
    ///
    /// The provider remains owned by the existing provider authority.
    #[must_use]
    pub fn hardware_provider(&self) -> &P {
        self.provider.as_ref()
    }

    /// Returns the wrapped quantum execution adapter.
    ///
    /// The adapter remains the authoritative implementation of
    /// quantum execution operations.
    #[must_use]
    pub fn quantum_backend_adapter(&self) -> &A {
        self.quantum.as_ref()
    }

    /// Returns the authoritative quantum backend descriptor.
    #[must_use]
    pub fn quantum_backend(&self) -> &QuantumBackend {
        self.quantum.backend()
    }

    // ------------------------------------------------------------------------
    // QUANTUM EXECUTION
    // ------------------------------------------------------------------------

    /// Performs quantum backend preflight.
    ///
    /// This delegates to the canonical QuantumBackendAdapter.
    ///
    /// It does not submit a job or reserve resources.
    ///
    /// # Errors
    ///
    /// Returns the original quantum backend validation error.
    pub fn preflight(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<(), crate::quantum::hardware::backend::BackendError> {
        self.quantum.preflight(request, program)
    }

    /// Submits a quantum workload.
    ///
    /// The authoritative quantum adapter validates and submits the
    /// workload.
    ///
    /// The returned job identity and initial state are provider-derived.
    ///
    /// # Errors
    ///
    /// Returns the original quantum backend error.
    pub fn submit(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<BackendJob, crate::quantum::hardware::backend::BackendError> {
        self.quantum.submit(request, program)
    }

    /// Retrieves the normalized status of a quantum job.
    ///
    /// # Errors
    ///
    /// Returns the original quantum backend error.
    pub fn status(
        &self,
        job: &BackendJobId,
    ) -> Result<BackendJobStatus, crate::quantum::hardware::backend::BackendError> {
        self.quantum.status(job)
    }

    /// Retrieves the normalized result of a completed quantum job.
    ///
    /// The wrapped adapter remains responsible for ensuring that
    /// the result is actually available and complete.
    ///
    /// # Errors
    ///
    /// Returns the original quantum backend error.
    pub fn result(
        &self,
        job: &BackendJobId,
    ) -> Result<ExecutionResult, crate::quantum::hardware::backend::BackendError> {
        self.quantum.result(job)
    }

    /// Requests cancellation of a quantum job.
    ///
    /// The returned cancellation outcome describes the provider's
    /// response. It does not independently prove that physical
    /// quantum execution has stopped.
    ///
    /// # Errors
    ///
    /// Returns the original quantum backend error.
    pub fn cancel(
        &self,
        job: &BackendJobId,
    ) -> Result<BackendCancellation, crate::quantum::hardware::backend::BackendError> {
        self.quantum.cancel(job)
    }

    /// Retrieves quantum backend queue information.
    ///
    /// # Errors
    ///
    /// Returns the original quantum backend error, including an
    /// unsupported-operation error where queue information is unavailable.
    pub fn queue_info(
        &self,
    ) -> Result<BackendQueueInfo, crate::quantum::hardware::backend::BackendError> {
        self.quantum.queue_info()
    }

    /// Retrieves quantum backend health.
    ///
    /// Unknown health remains unknown. It is never converted into
    /// a healthy state.
    ///
    /// # Errors
    ///
    /// Returns the original quantum backend error.
    pub fn health(
        &self,
    ) -> Result<BackendHealth, crate::quantum::hardware::backend::BackendError> {
        self.quantum.health()
    }

    /// Executes synchronously when the wrapped backend supports it.
    ///
    /// The wrapped adapter determines whether synchronous execution
    /// is actually supported.
    ///
    /// # Errors
    ///
    /// Returns the original quantum backend error.
    pub fn execute(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<ExecutionResult, crate::quantum::hardware::backend::BackendError> {
        self.quantum.execute(request, program)
    }

    /// Returns whether quantum cancellation is supported.
    #[must_use]
    pub fn supports_cancellation(&self) -> bool {
        self.quantum.supports_cancellation()
    }

    /// Returns whether quantum queue information is supported.
    #[must_use]
    pub fn supports_queue_info(&self) -> bool {
        self.quantum.supports_queue_info()
    }

    /// Returns whether native synchronous execution is supported.
    #[must_use]
    pub fn supports_synchronous_execution(&self) -> bool {
        self.quantum.supports_synchronous_execution()
    }
}

// ============================================================================
// DEBUG
// ============================================================================

impl<
        P: HardwareProvider + ?Sized,
        A: QuantumBackendAdapter + ?Sized,
    > fmt::Debug for QuantumHardwareAdapter<P, A>
{
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("QuantumHardwareAdapter")
            .field(
                "provider_id",
                self.provider.descriptor().id(),
            )
            .field(
                "device_identity",
                self.binding.device(),
            )
            .field(
                "quantum_backend_id",
                &self.binding.backend_id(),
            )
            .field(
                "quantum_adapter_id",
                &self.quantum.adapter_info().adapter_id,
            )
            .finish()
    }
}

// ============================================================================
// STANDARD-LIBRARY PROVIDER INTEGRATION
// ============================================================================

/// Delegates common provider identity, status and capability observations
/// to the canonical wrapped provider.
///
/// Quantum-specific capabilities are deliberately not copied into this
/// generic capability snapshot.
impl<
        P: HardwareProvider + ?Sized,
        A: QuantumBackendAdapter + ?Sized,
    > Provider for QuantumHardwareAdapter<P, A>
{
    fn descriptor(&self) -> &ProviderDescriptor {
        self.provider.descriptor()
    }

    fn status(&self) -> ProviderStatus {
        self.provider.status()
    }

    fn capability_snapshot<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, CapabilitySnapshot> {
        self.provider.capability_snapshot(context)
    }
}

// ============================================================================
// UNIVERSAL ZHAL HARDWARE PROVIDER INTEGRATION
// ============================================================================

/// Delegates generic physical hardware operations to the existing
/// canonical HardwareProvider.
///
/// Quantum-specific execution is deliberately exposed separately through
/// the QuantumBackendAdapter interface.
impl<
        P: HardwareProvider + ?Sized,
        A: QuantumBackendAdapter + ?Sized,
    > HardwareProvider for QuantumHardwareAdapter<P, A>
{
    fn discover_devices_page<'a>(
        &'a self,
        request: &'a DeviceDiscoveryRequest,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, DeviceDiscoveryPage> {
        self.provider
            .discover_devices_page(request, context)
    }

    fn inspect_device<'a>(
        &'a self,
        identity: &'a HardwareDeviceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, crate::zhal::model::HardwareDeviceDescriptor> {
        self.provider.inspect_device(identity, context)
    }

    fn describe_interface<'a>(
        &'a self,
        identity: &'a crate::zhal::identity::HardwareInterfaceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, crate::zhal::model::HardwareInterfaceDescriptor> {
        self.provider.describe_interface(identity, context)
    }

    fn open_session<'a>(
        &'a self,
        device: &'a HardwareDeviceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, Box<dyn HardwareSession>> {
        self.provider.open_session(device, context)
    }
}

// ============================================================================
// EXISTING PROVIDER LIFECYCLE INTEGRATION
// ============================================================================

/// Delegates initialization and shutdown to the existing provider lifecycle.
///
/// This implementation is available only when the wrapped provider
/// already implements ProviderLifecycle.
///
/// The adapter does not create another lifecycle manager or state machine.
impl<
        P: HardwareProvider + ProviderLifecycle + ?Sized,
        A: QuantumBackendAdapter + ?Sized,
    > ProviderLifecycle for QuantumHardwareAdapter<P, A>
{
    fn initialize<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, ()> {
        self.provider.initialize(context)
    }

    fn shutdown<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, ()> {
        self.provider.shutdown(context)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_backend_identity_is_accepted() {
        assert!(
            validate_backend_id("quantum.provider.backend")
                .is_ok()
        );
    }

    #[test]
    fn empty_backend_identity_is_rejected() {
        assert!(
            validate_backend_id("")
                .is_err()
        );
    }

    #[test]
    fn whitespace_backend_identity_is_rejected() {
        assert!(
            validate_backend_id("quantum backend")
                .is_err()
        );

        assert!(
            validate_backend_id(" quantum.backend")
                .is_err()
        );

        assert!(
            validate_backend_id("quantum.backend ")
                .is_err()
        );
    }

    #[test]
    fn control_character_backend_identity_is_rejected() {
        assert!(
            validate_backend_id("quantum\u{0000}backend")
                .is_err()
        );
    }

    #[test]
    fn backend_identity_has_no_arbitrary_length_ceiling() {
        let backend_id = "q".repeat(16_384);

        assert!(
            validate_backend_id(&backend_id)
                .is_ok()
        );
    }

    #[test]
    fn adapter_schema_identity_is_stable() {
        assert_eq!(
            QUANTUM_ADAPTER_SCHEMA_ID,
            "zamani.zhal.adapters.quantum"
        );

        assert_eq!(
            QUANTUM_ADAPTER_SCHEMA_VERSION,
            1
        );
    }
}
