
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Accelerator Hardware Adapter
//!
//! Canonical path:
//!     src/zhal/adapters/accelerator.rs
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Provides the integration boundary between universal ZHAL hardware
//! providers and the existing Zamani AI execution architecture.
//!
//! This adapter supports independently implemented accelerator
//! execution integrations, including:
//!
//! - General-purpose GPU computation.
//! - Neural processing.
//! - Tensor processing.
//! - Digital signal processing.
//! - Reconfigurable acceleration.
//! - AI inference and training.
//! - Heterogeneous acceleration.
//! - Remote acceleration.
//! - Future computational accelerators.
//!
//! These are examples, not a closed list.
//!
//! ============================================================================
//! ARCHITECTURAL AUTHORITIES
//! ============================================================================
//!
//! Provider identity and lifecycle:
//!     crate::stdlib::provider
//!
//! Capability identity and evidence:
//!     crate::stdlib::capability
//!
//! Hardware provider contract:
//!     crate::zhal::providers::contract
//!
//! Hardware device identity:
//!     crate::zhal::identity
//!
//! Hardware device descriptions:
//!     crate::zhal::model
//!
//! Hardware operation context:
//!     crate::zhal::context
//!
//! AI execution:
//!     crate::ai::execution
//!
//! AI requirements:
//!     crate::ai::requirements
//!
//! AI operation registration:
//!     crate::ai::registry
//!
//! AI tensor semantics:
//!     crate::ai::tensor
//!
//! Canonical tensor storage:
//!     crate::classical::collections::tensor
//!
//! Universal tensor representation:
//!     crate::zuir::dialects::tensor
//!
//! Compilation and target realization:
//!     crate::compiler
//!     crate::backend
//!
//! Physical resource management and execution:
//!     crate::zur
//!     crate::runtime
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - Accelerator execution adapter integration.
//! - Explicit accelerator-provider association.
//! - Accelerator capability observation.
//! - Accelerator capability assessment reports.
//! - AI execution delegation through the canonical AI execution engine.
//! - Generic ZHAL provider delegation.
//! - Accelerator integration validation.
//!
//! ============================================================================
//! DOES NOT OWN
//! ============================================================================
//!
//! This module does not implement or duplicate:
//!
//! - AI semantic analysis.
//! - Source grammar.
//! - AST construction.
//! - Tensor shape semantics.
//! - Tensor data types.
//! - Tensor storage.
//! - Tensor numerical kernels.
//! - AI model definitions.
//! - AI operation registration.
//! - AI execution result semantics.
//! - Classical IR.
//! - Quantum IR.
//! - HDLIR.
//! - ZUIR.
//! - Backend code generation.
//! - Physical resource allocation.
//! - Memory allocation.
//! - Device scheduling.
//! - Device discovery.
//! - Hardware communication.
//! - Accelerator driver implementations.
//! - Distributed transport.
//! - Hybrid scheduling.
//! - Authentication.
//! - Authorization.
//! - Provider lifecycle management.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This adapter introduces no universal maximum for:
//!
//! - Accelerators.
//! - GPU devices.
//! - NPU devices.
//! - TPU devices.
//! - DSP devices.
//! - Providers.
//! - Hardware devices.
//! - Compute units.
//! - Processing elements.
//! - Tensor dimensions.
//! - Tensor rank.
//! - Tensor elements.
//! - Model parameters.
//! - Memory.
//! - Hardware resources.
//! - Operations.
//! - Execution requests.
//! - Distributed participants.
//! - Hardware technologies.
//!
//! Actual physical capacity is determined by hardware observations,
//! provider implementations, resource availability and applicable
//! operational policies.
//!
//! Literal infinite physical computation is not promised.
//!
//! No arbitrary universal hardware ceiling is introduced.
//!
//! ============================================================================
//! TENSOR OWNERSHIP
//! ============================================================================
//!
//! The following authorities remain distinct:
//!
//! frontend::semantic::domains::ai:
//!     Canonical semantic tensor shape and dimensions.
//!
//! ai::tensor:
//!     AI tensor descriptions, shapes, dtypes and operation contracts.
//!
//! classical::collections::tensor:
//!     Canonical concrete classical tensor storage.
//!
//! zuir::dialects::tensor:
//!     Universal tensor operation representation.
//!
//! compiler / backend / runtime:
//!     Lowering, realization, allocation and execution.
//!
//! The accelerator adapter must not copy, reinterpret or replace
//! these representations.
//!
//! The accelerator execution dispatcher must assess tensor
//! compatibility using the authoritative AI and compiler contracts.
//!
//! ============================================================================
//! EXECUTION SAFETY
//! ============================================================================
//!
//! AI execution is delegated through AiExecutionEngine.
//!
//! This preserves:
//!
//! - Workload validation.
//! - Complete requirements validation.
//! - Exact operation-version resolution.
//! - Capability assessment.
//! - Resource feasibility assessment.
//! - Authorization assessment.
//! - Cancellation.
//! - Deadline handling.
//! - Execution dispatch.
//! - Provenance requirements.
//! - Execution result validation.
//!
//! An indeterminate execution assessment never authorizes execution.
//!
//! ============================================================================
//! HARDWARE BOUNDARY
//! ============================================================================
//!
//! The generic HardwareProvider owns:
//!
//! - Hardware discovery.
//! - Device inspection.
//! - Interface inspection.
//! - Hardware sessions.
//! - Generic provider identity.
//! - Generic capability observations.
//!
//! The accelerator dispatcher owns:
//!
//! - Accelerator workload compatibility.
//! - Accelerator execution feasibility.
//! - Accelerator-specific execution.
//! - Integration with authoritative backend/runtime services.
//!
//! The runtime remains responsible for physical resource orchestration.
//!
//! ============================================================================
//! FAILURE SEMANTICS
//! ============================================================================
//!
//! The adapter must never:
//!
//! - Treat missing capability evidence as available.
//! - Convert unknown availability into unavailability.
//! - Convert execution failure into success.
//! - Manufacture execution results.
//! - Manufacture hardware identities.
//! - Assume a provider is ready.
//! - Claim that capability availability reserves resources.
//! - Claim that cancellation proves physical execution stopped.
//! - Claim that all accelerators support every numerical format.
//! - Silently substitute a different AI operation version.
//!
//! ============================================================================
//! THREAD SAFETY
//! ============================================================================
//!
//! Provider and execution dispatcher implementations are shared
//! through Arc.
//!
//! Their canonical contracts require Send + Sync.
//!
//! No global mutable state is introduced.
//!
//! ============================================================================
//! SECURITY
//! ============================================================================
//!
//! This module stores no credentials.
//!
//! Provider trust, authentication, authorization and isolation
//! remain with the existing ZHAL/ZUR security authorities.
//!
//! Debug output does not expose provider internals or credentials.
//!
//! ============================================================================

use core::fmt;

use std::sync::Arc;

use crate::stdlib::capability::{
    CapabilityId,
    CapabilitySnapshot,
    CapabilityStatus,
};

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

use crate::ai::context::AiContext;

use crate::ai::execution::{
    AiExecutable,
    AiExecutionDispatcher,
    AiExecutionEngine,
    AiExecutionRequest,
    AiExecutionResult,
};

use crate::distributed::execution::executor::ExecutionControl;


// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Stable component identity.
pub const COMPONENT: &str = "zhal.adapters.accelerator";

/// Accelerator adapter schema identifier.
pub const ACCELERATOR_ADAPTER_SCHEMA_ID: &str =
    "zamani.zhal.adapters.accelerator";

/// Accelerator adapter schema version.
///
/// This is independent of the language, compiler, backend,
/// hardware firmware and AI operation-set versions.
pub const ACCELERATOR_ADAPTER_SCHEMA_VERSION: u16 = 1;


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

fn validate_adapter_id(
    value: &str,
) -> ZhalResult<()> {
    if value.is_empty() {
        return Err(invalid_input(
            "accelerator execution adapter identity must not be empty",
        ));
    }

    if value.trim() != value {
        return Err(invalid_input(
            "accelerator execution adapter identity must not contain surrounding whitespace",
        ));
    }

    if value.chars().any(|character| {
        character.is_control() || character.is_whitespace()
    }) {
        return Err(invalid_input(
            "accelerator execution adapter identity must not contain whitespace or control characters",
        ));
    }

    Ok(())
}


// ============================================================================
// ACCELERATOR EXECUTION DISPATCHER CONTRACT
// ============================================================================

/// Integration contract for an AI execution dispatcher associated
/// with a particular universal hardware provider.
///
/// This trait extends the canonical AI execution dispatcher.
///
/// It does not introduce another execution engine.
///
/// An implementation must preserve all existing AI execution
/// requirements and delegate actual realization to the appropriate
/// compiler, backend and runtime infrastructure.
///
/// # Hardware association
///
/// Every dispatcher explicitly identifies the ZHAL provider
/// whose hardware it represents.
///
/// An optional device identity can identify a device-scoped dispatcher.
///
/// When the device identity is absent, the dispatcher may perform
/// dynamic target selection within its associated provider, subject
/// to the canonical runtime's selection and authorization policies.
///
/// The dispatcher must never silently execute against an unrelated
/// provider.
///
/// # Tensor compatibility
///
/// Implementations must assess:
///
/// - Symbolic tensor shapes.
/// - Concrete tensor dimensions.
/// - Tensor rank.
/// - Numerical data types.
/// - Numerical precision requirements.
/// - Logical tensor layouts.
/// - Operation compatibility.
/// - Memory and transfer requirements.
/// - Provider-supported computation.
/// - Runtime resource feasibility.
///
/// Canonical tensor and AI contracts remain authoritative.
///
/// # Execution
///
/// The dispatcher must:
///
/// - Preserve workload semantics.
/// - Preserve exact operation versions.
/// - Revalidate volatile hardware conditions.
/// - Respect execution controls.
/// - Preserve provider failures.
/// - Preserve execution provenance.
/// - Avoid fabricating results.
/// - Use the authoritative backend/runtime integration.
///
/// # Type parameter
///
/// `W` is an existing AI executable workload.
pub trait AcceleratorExecutionDispatcher<W: AiExecutable>:
    AiExecutionDispatcher<W>
{
    /// Returns the canonical provider identity associated with
    /// this accelerator execution dispatcher.
    fn hardware_provider_id(
        &self,
    ) -> &crate::stdlib::provider::ProviderId;

    /// Returns the associated physical device, if this dispatcher
    /// is explicitly scoped to one device.
    ///
    /// Returning None permits provider-scoped dynamic selection.
    ///
    /// None does not establish that a device is available.
    fn device_identity(
        &self,
    ) -> Option<&HardwareDeviceIdentity>;

    /// Returns the stable identity of this execution integration.
    ///
    /// This is adapter metadata, not a provider identity or
    /// physical hardware identity.
    fn adapter_id(&self) -> &str;
}


// ============================================================================
// ACCELERATOR HARDWARE ADAPTER
// ============================================================================

/// Composite integration of a universal hardware provider and
/// an authoritative AI execution dispatcher.
///
/// The generic hardware provider owns hardware discovery,
/// inspection, interface descriptions, generic capability
/// observations and hardware sessions.
///
/// The AI execution dispatcher owns accelerator execution
/// feasibility and actual workload realization.
///
/// The adapter connects these authorities without replacing either.
///
/// # Type parameters
///
/// `P`:
/// Existing ZHAL hardware provider.
///
/// `W`:
/// Existing validated AI executable workload.
///
/// `D`:
/// Accelerator-associated AI execution dispatcher.
///
/// Concrete implementations and compatible trait objects are supported.
pub struct AcceleratorHardwareAdapter<
    P,
    W,
    D,
>
where
    P: HardwareProvider + ?Sized,
    W: AiExecutable,
    D: AcceleratorExecutionDispatcher<W> + ?Sized,
{
    provider: Arc<P>,
    dispatcher: Arc<D>,
    workload: core::marker::PhantomData<fn(&W)>,
}

impl<
        P,
        W,
        D,
    > AcceleratorHardwareAdapter<P, W, D>
where
    P: HardwareProvider + ?Sized,
    W: AiExecutable,
    D: AcceleratorExecutionDispatcher<W> + ?Sized,
{
    // ------------------------------------------------------------------------
    // CONSTRUCTION
    // ------------------------------------------------------------------------

    /// Constructs an accelerator hardware adapter.
    ///
    /// Construction validates the relationship between the
    /// generic hardware provider and accelerator dispatcher.
    ///
    /// It performs no:
    ///
    /// - Hardware discovery.
    /// - Network communication.
    /// - Provider initialization.
    /// - Device allocation.
    /// - Resource reservation.
    /// - Authentication.
    /// - Authorization.
    /// - Accelerator execution.
    ///
    /// # Validation
    ///
    /// The dispatcher must identify the same provider as the
    /// generic hardware provider.
    ///
    /// If the dispatcher identifies an individual device,
    /// that device must belong to the same provider.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput when the provider association
    /// or dispatcher identity is invalid.
    pub fn new(
        provider: Arc<P>,
        dispatcher: Arc<D>,
    ) -> ZhalResult<Self> {
        let provider_id = provider.descriptor().id();

        if dispatcher.hardware_provider_id() != provider_id {
            return Err(invalid_input(
                "accelerator dispatcher belongs to a different hardware provider",
            ));
        }

        validate_adapter_id(
            dispatcher.adapter_id(),
        )?;

        if let Some(device) = dispatcher.device_identity() {
            if device.provider_id() != provider_id {
                return Err(invalid_input(
                    "accelerator execution device belongs to a different hardware provider",
                ));
            }
        }

        Ok(Self {
            provider,
            dispatcher,
            workload: core::marker::PhantomData,
        })
    }

    // ------------------------------------------------------------------------
    // PROVIDER ACCESS
    // ------------------------------------------------------------------------

    /// Returns the underlying universal hardware provider.
    ///
    /// Ownership remains with the wrapped provider.
    #[must_use]
    pub fn hardware_provider(&self) -> &P {
        self.provider.as_ref()
    }

    /// Returns the shared provider handle.
    #[must_use]
    pub fn hardware_provider_handle(&self) -> Arc<P> {
        Arc::clone(&self.provider)
    }

    /// Returns the associated accelerator execution dispatcher.
    #[must_use]
    pub fn execution_dispatcher(&self) -> &D {
        self.dispatcher.as_ref()
    }

    /// Returns the shared execution-dispatcher handle.
    #[must_use]
    pub fn execution_dispatcher_handle(&self) -> Arc<D> {
        Arc::clone(&self.dispatcher)
    }

    /// Returns the canonical provider descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ProviderDescriptor {
        self.provider.descriptor()
    }

    /// Returns the current provider status.
    ///
    /// This is an observation, not an execution guarantee.
    #[must_use]
    pub fn provider_status(&self) -> ProviderStatus {
        self.provider.status()
    }

    /// Returns the accelerator execution adapter identity.
    #[must_use]
    pub fn adapter_id(&self) -> &str {
        self.dispatcher.adapter_id()
    }

    /// Returns the associated hardware device identity when
    /// the execution dispatcher is device-scoped.
    #[must_use]
    pub fn device_identity(&self) -> Option<&HardwareDeviceIdentity> {
        self.dispatcher.device_identity()
    }

    /// Returns whether the generic provider may accept operations.
    ///
    /// Capability availability, authorization and physical
    /// resource feasibility must still be established separately.
    #[must_use]
    pub fn may_accept_operations(&self) -> bool {
        self.provider_status().may_accept_operations()
    }

    // ------------------------------------------------------------------------
    // AI EXECUTION
    // ------------------------------------------------------------------------

    /// Executes an AI workload through the canonical AI execution engine.
    ///
    /// This method delegates to AiExecutionEngine rather than
    /// directly invoking the dispatcher's execute method.
    ///
    /// Consequently, the canonical AI execution validation remains
    /// mandatory.
    ///
    /// The AI engine performs:
    ///
    /// 1. Workload validation.
    /// 2. Requirements completeness validation.
    /// 3. Exact AI operation registration resolution.
    /// 4. Accelerator execution eligibility assessment.
    /// 5. Cancellation and deadline checks.
    /// 6. Actual execution delegation.
    /// 7. Provenance validation.
    /// 8. Result construction.
    ///
    /// The dispatcher must use the backend/runtime authority
    /// associated with this accelerator provider.
    ///
    /// No tensor data is copied or transformed by this adapter.
    ///
    /// # Errors
    ///
    /// Returns the original structured AI execution error.
    ///
    /// Errors include invalid workloads, incomplete requirements,
    /// unsupported operations, infeasibility, indeterminate
    /// eligibility, cancellation, deadline expiry, execution
    /// failures and provenance violations.
    pub fn execute_ai<'work, 'services>(
        &self,
        context: &AiContext<'services>,
        request: &AiExecutionRequest<'work, W>,
        control: &ExecutionControl,
    ) -> crate::ai::error::AiResult<
        AiExecutionResult<D::Output>
    > {
        AiExecutionEngine::execute(
            context,
            request,
            self.dispatcher.as_ref(),
            control,
        )
    }

    // ------------------------------------------------------------------------
    // CAPABILITY OBSERVATION
    // ------------------------------------------------------------------------

    /// Obtains a fresh generic provider capability snapshot.
    ///
    /// The snapshot is provider-owned evidence.
    ///
    /// It is not copied into a competing accelerator capability system.
    ///
    /// # Cancellation
    ///
    /// Cancellation is checked before and after observation.
    ///
    /// # Errors
    ///
    /// Returns the original standard-library provider error.
    pub fn observe_capabilities<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, CapabilitySnapshot> {
        Box::pin(async move {
            context.check_cancelled()?;

            let snapshot = self.provider
                .capability_snapshot(context)
                .await?;

            context.check_cancelled()?;

            Ok(snapshot)
        })
    }

    /// Assesses explicitly supplied accelerator capability requirements.
    ///
    /// Capability identities must originate from the canonical
    /// semantic, compiler or execution requirement authorities.
    ///
    /// The assessment preserves the exact capability states
    /// returned by the provider.
    ///
    /// Unknown capability evidence is never treated as available.
    ///
    /// This method does not perform resource allocation or
    /// complete execution feasibility assessment.
    ///
    /// # Errors
    ///
    /// Returns the original provider error or cancellation error.
    pub fn assess_capabilities<'a>(
        &'a self,
        context: &'a OperationContext,
        requirements: &'a [CapabilityId],
    ) -> ProviderFuture<'a, AcceleratorCapabilityReport> {
        Box::pin(async move {
            context.check_cancelled()?;

            let snapshot = self.provider
                .capability_snapshot(context)
                .await?;

            context.check_cancelled()?;

            Ok(
                AcceleratorCapabilityReport::from_snapshot(
                    self.provider.status(),
                    &snapshot,
                    requirements.iter().cloned(),
                )
            )
        })
    }

    /// Returns the canonical provider identity.
    #[must_use]
    pub fn provider_id(
        &self,
    ) -> &crate::stdlib::provider::ProviderId {
        self.provider.descriptor().id()
    }
}


// ============================================================================
// ACCELERATOR CAPABILITY ASSESSMENT
// ============================================================================

/// Assessment of one canonical accelerator capability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcceleratorCapabilityAssessment {
    capability: CapabilityId,
    status: CapabilityStatus,
}

impl AcceleratorCapabilityAssessment {
    /// Constructs an accelerator capability assessment.
    #[must_use]
    pub fn new(
        capability: CapabilityId,
        status: CapabilityStatus,
    ) -> Self {
        Self {
            capability,
            status,
        }
    }

    /// Returns the canonical capability identity.
    #[must_use]
    pub fn capability(&self) -> &CapabilityId {
        &self.capability
    }

    /// Returns the exact provider-reported capability state.
    #[must_use]
    pub const fn status(&self) -> CapabilityStatus {
        self.status
    }

    /// Returns whether availability has been established.
    #[must_use]
    pub const fn is_available(&self) -> bool {
        self.status.is_available()
    }

    /// Returns whether the capability is unavailable.
    #[must_use]
    pub const fn is_unavailable(&self) -> bool {
        self.status.is_unavailable()
    }

    /// Returns whether availability is conditional.
    #[must_use]
    pub const fn is_conditional(&self) -> bool {
        self.status.is_conditional()
    }

    /// Returns whether availability is unknown.
    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        self.status.is_unknown()
    }
}


// ============================================================================
// ACCELERATOR CAPABILITY REPORT
// ============================================================================

/// Evidence-backed report of accelerator provider capabilities.
///
/// The report retains the original canonical capability snapshot.
///
/// Individual assessments are derived from that snapshot.
///
/// This report is observational and does not establish:
///
/// - Resource reservation.
/// - Physical resource allocation.
/// - Execution authorization.
/// - Workload feasibility.
/// - Backend compilation success.
/// - Successful accelerator execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcceleratorCapabilityReport {
    provider_status: ProviderStatus,
    snapshot: CapabilitySnapshot,
    assessments: Vec<AcceleratorCapabilityAssessment>,
}

impl AcceleratorCapabilityReport {
    /// Constructs a capability report from an existing snapshot.
    ///
    /// Requirement order is preserved.
    ///
    /// No arbitrary requirement-count limit is imposed.
    #[must_use]
    pub fn from_snapshot(
        provider_status: ProviderStatus,
        snapshot: &CapabilitySnapshot,
        requirements: impl IntoIterator<Item = CapabilityId>,
    ) -> Self {
        let assessments = requirements
            .into_iter()
            .map(|capability| {
                AcceleratorCapabilityAssessment::new(
                    capability.clone(),
                    snapshot.status(&capability),
                )
            })
            .collect();

        Self {
            provider_status,
            snapshot: snapshot.clone(),
            assessments,
        }
    }

    /// Returns the observed provider status.
    #[must_use]
    pub const fn provider_status(&self) -> ProviderStatus {
        self.provider_status
    }

    /// Returns the original canonical capability snapshot.
    #[must_use]
    pub fn snapshot(&self) -> &CapabilitySnapshot {
        &self.snapshot
    }

    /// Returns individual capability assessments.
    #[must_use]
    pub fn assessments(&self) -> &[AcceleratorCapabilityAssessment] {
        &self.assessments
    }

    /// Returns the number of assessed capabilities.
    #[must_use]
    pub fn requirement_count(&self) -> usize {
        self.assessments.len()
    }

    /// Returns whether no capability requirements were supplied.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.assessments.is_empty()
    }

    /// Returns whether every requested capability is established
    /// as unconditionally available.
    ///
    /// An empty requirement set is vacuously satisfied.
    ///
    /// This does not establish provider readiness or resource feasibility.
    #[must_use]
    pub fn all_capabilities_available(&self) -> bool {
        self.assessments
            .iter()
            .all(
                AcceleratorCapabilityAssessment::is_available
            )
    }

    /// Returns whether the provider may accept operations.
    #[must_use]
    pub const fn provider_may_accept_operations(&self) -> bool {
        self.provider_status.may_accept_operations()
    }

    /// Returns whether the provider and capability gate is satisfied.
    ///
    /// This requires:
    ///
    /// - Provider readiness.
    /// - Established availability of every requested capability.
    ///
    /// This is not a complete execution-feasibility certificate.
    #[must_use]
    pub fn provider_capability_gate_satisfied(&self) -> bool {
        self.provider_may_accept_operations()
            && self.all_capabilities_available()
    }

    /// Returns whether any requested capability is unavailable.
    #[must_use]
    pub fn has_unavailable_capability(&self) -> bool {
        self.assessments
            .iter()
            .any(
                AcceleratorCapabilityAssessment::is_unavailable
            )
    }

    /// Returns whether any requested capability is conditional.
    #[must_use]
    pub fn has_conditional_capability(&self) -> bool {
        self.assessments
            .iter()
            .any(
                AcceleratorCapabilityAssessment::is_conditional
            )
    }

    /// Returns whether any requested capability is unknown.
    #[must_use]
    pub fn has_unknown_capability(&self) -> bool {
        self.assessments
            .iter()
            .any(
                AcceleratorCapabilityAssessment::is_unknown
            )
    }
}


// ============================================================================
// DEBUG
// ============================================================================

impl<
        P,
        W,
        D,
    > fmt::Debug for AcceleratorHardwareAdapter<P, W, D>
where
    P: HardwareProvider + ?Sized,
    W: AiExecutable,
    D: AcceleratorExecutionDispatcher<W> + ?Sized,
{
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("AcceleratorHardwareAdapter")
            .field(
                "provider_id",
                self.provider.descriptor().id(),
            )
            .field(
                "provider_status",
                &self.provider.status(),
            )
            .field(
                "adapter_id",
                &self.dispatcher.adapter_id(),
            )
            .field(
                "device_identity",
                &self.dispatcher.device_identity(),
            )
            .finish()
    }
}


// ============================================================================
// STANDARD-LIBRARY PROVIDER INTEGRATION
// ============================================================================

/// Delegates provider identity, status and capability observations
/// to the canonical wrapped hardware provider.
///
/// Accelerator-specific capabilities are not fabricated or
/// independently inserted into the generic capability snapshot.
impl<
        P,
        W,
        D,
    > Provider for AcceleratorHardwareAdapter<P, W, D>
where
    P: HardwareProvider + ?Sized,
    W: AiExecutable,
    D: AcceleratorExecutionDispatcher<W> + ?Sized,
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
// UNIVERSAL HARDWARE PROVIDER INTEGRATION
// ============================================================================

/// Delegates generic physical hardware operations to the
/// authoritative wrapped HardwareProvider.
///
/// Accelerator-specific execution remains available through
/// the AI execution dispatcher.
impl<
        P,
        W,
        D,
    > HardwareProvider for AcceleratorHardwareAdapter<P, W, D>
where
    P: HardwareProvider + ?Sized,
    W: AiExecutable,
    D: AcceleratorExecutionDispatcher<W> + ?Sized,
{
    fn discover_devices_page<'a>(
        &'a self,
        request: &'a DeviceDiscoveryRequest,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, DeviceDiscoveryPage> {
        self.provider
            .discover_devices_page(
                request,
                context,
            )
    }

    fn inspect_device<'a>(
        &'a self,
        identity: &'a HardwareDeviceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<
        'a,
        crate::zhal::model::HardwareDeviceDescriptor,
    > {
        self.provider.inspect_device(
            identity,
            context,
        )
    }

    fn describe_interface<'a>(
        &'a self,
        identity: &'a crate::zhal::identity::HardwareInterfaceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<
        'a,
        crate::zhal::model::HardwareInterfaceDescriptor,
    > {
        self.provider.describe_interface(
            identity,
            context,
        )
    }

    fn open_session<'a>(
        &'a self,
        device: &'a HardwareDeviceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<
        'a,
        Box<dyn HardwareSession>,
    > {
        self.provider.open_session(
            device,
            context,
        )
    }
}


// ============================================================================
// EXISTING PROVIDER LIFECYCLE INTEGRATION
// ============================================================================

/// Delegates provider initialization and shutdown to the
/// existing canonical ProviderLifecycle implementation.
///
/// This implementation exists only when the wrapped provider
/// implements ProviderLifecycle.
///
/// The adapter introduces no competing lifecycle state machine.
impl<
        P,
        W,
        D,
    > ProviderLifecycle for AcceleratorHardwareAdapter<P, W, D>
where
    P: HardwareProvider + ProviderLifecycle + ?Sized,
    W: AiExecutable,
    D: AcceleratorExecutionDispatcher<W> + ?Sized,
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

    use crate::stdlib::capability::{
        Capability,
        CapabilityKind,
        CapabilityState,
    };

    fn capability(
        identity: &str,
    ) -> Capability {
        Capability::new(
            CapabilityId::new(identity)
                .expect("valid capability identity"),
            CapabilityKind::new("computation")
                .expect("valid capability kind"),
        )
    }

    fn available_snapshot(
        identity: &str,
    ) -> CapabilitySnapshot {
        let mut snapshot = CapabilitySnapshot::new();

        snapshot
            .insert(
                CapabilityState::new(
                    capability(identity),
                    CapabilityStatus::Available,
                )
            )
            .expect("unique capability");

        snapshot
    }

    // ------------------------------------------------------------------------
    // IDENTIFIER VALIDATION
    // ------------------------------------------------------------------------

    #[test]
    fn valid_adapter_identity_is_accepted() {
        assert!(
            validate_adapter_id(
                "zamani.accelerator.execution"
            )
            .is_ok()
        );
    }

    #[test]
    fn empty_adapter_identity_is_rejected() {
        assert!(
            validate_adapter_id("").is_err()
        );
    }

    #[test]
    fn whitespace_adapter_identity_is_rejected() {
        assert!(
            validate_adapter_id(
                "accelerator execution"
            )
            .is_err()
        );
    }

    #[test]
    fn surrounding_whitespace_is_rejected() {
        assert!(
            validate_adapter_id(
                " accelerator.execution"
            )
            .is_err()
        );
    }

    #[test]
    fn control_characters_are_rejected() {
        assert!(
            validate_adapter_id(
                "accelerator\u{0000}execution"
            )
            .is_err()
        );
    }

    #[test]
    fn adapter_identity_has_no_arbitrary_length_ceiling() {
        let identity = "accelerator".repeat(4096);

        assert!(
            validate_adapter_id(&identity).is_ok()
        );
    }

    // ------------------------------------------------------------------------
    // CAPABILITY REPORTS
    // ------------------------------------------------------------------------

    #[test]
    fn available_capability_is_preserved() {
        let snapshot = available_snapshot(
            "accelerator.tensor.compute"
        );

        let requirement = CapabilityId::new(
            "accelerator.tensor.compute"
        )
        .expect("valid requirement");

        let report = AcceleratorCapabilityReport::from_snapshot(
            ProviderStatus::Ready,
            &snapshot,
            [requirement],
        );

        assert!(
            report.all_capabilities_available()
        );

        assert!(
            report.provider_capability_gate_satisfied()
        );
    }

    #[test]
    fn unknown_capability_is_not_treated_as_available() {
        let snapshot = CapabilitySnapshot::new();

        let requirement = CapabilityId::new(
            "accelerator.future.compute"
        )
        .expect("valid requirement");

        let report = AcceleratorCapabilityReport::from_snapshot(
            ProviderStatus::Ready,
            &snapshot,
            [requirement],
        );

        assert!(
            !report.all_capabilities_available()
        );

        assert!(
            !report.provider_capability_gate_satisfied()
        );

        assert!(
            report.has_unknown_capability()
        );
    }

    #[test]
    fn unavailable_capability_remains_unavailable() {
        let mut snapshot = CapabilitySnapshot::new();

        snapshot
            .insert(
                CapabilityState::new(
                    capability(
                        "accelerator.tensor.compute"
                    ),
                    CapabilityStatus::Unavailable,
                )
            )
            .expect("unique capability");

        let requirement = CapabilityId::new(
            "accelerator.tensor.compute"
        )
        .expect("valid requirement");

        let report = AcceleratorCapabilityReport::from_snapshot(
            ProviderStatus::Ready,
            &snapshot,
            [requirement],
        );

        assert!(
            report.has_unavailable_capability()
        );

        assert!(
            !report.all_capabilities_available()
        );
    }

    #[test]
    fn conditional_capability_remains_conditional() {
        let mut snapshot = CapabilitySnapshot::new();

        snapshot
            .insert(
                CapabilityState::new(
                    capability(
                        "accelerator.tensor.compute"
                    ),
                    CapabilityStatus::Conditional,
                )
            )
            .expect("unique capability");

        let requirement = CapabilityId::new(
            "accelerator.tensor.compute"
        )
        .expect("valid requirement");

        let report = AcceleratorCapabilityReport::from_snapshot(
            ProviderStatus::Ready,
            &snapshot,
            [requirement],
        );

        assert!(
            report.has_conditional_capability()
        );

        assert!(
            !report.all_capabilities_available()
        );
    }

    #[test]
    fn provider_readiness_is_independent_of_capability_availability() {
        let snapshot = available_snapshot(
            "accelerator.tensor.compute"
        );

        let requirement = CapabilityId::new(
            "accelerator.tensor.compute"
        )
        .expect("valid requirement");

        let report = AcceleratorCapabilityReport::from_snapshot(
            ProviderStatus::Unavailable,
            &snapshot,
            [requirement],
        );

        assert!(
            report.all_capabilities_available()
        );

        assert!(
            !report.provider_capability_gate_satisfied()
        );
    }

    #[test]
    fn empty_requirements_do_not_invent_capabilities() {
        let snapshot = CapabilitySnapshot::new();

        let report = AcceleratorCapabilityReport::from_snapshot(
            ProviderStatus::Ready,
            &snapshot,
            [],
        );

        assert!(report.is_empty());

        assert_eq!(
            report.requirement_count(),
            0
        );

        assert!(
            report.all_capabilities_available()
        );

        assert!(
            report.provider_capability_gate_satisfied()
        );

        assert!(report.snapshot().is_empty());
    }

    #[test]
    fn requirement_order_is_preserved() {
        let snapshot = CapabilitySnapshot::new();

        let first = CapabilityId::new(
            "accelerator.tensor.compute"
        )
        .expect("valid capability");

        let second = CapabilityId::new(
            "accelerator.neural.compute"
        )
        .expect("valid capability");

        let report = AcceleratorCapabilityReport::from_snapshot(
            ProviderStatus::Ready,
            &snapshot,
            [first.clone(), second.clone()],
        );

        assert_eq!(
            report.assessments()[0].capability(),
            &first
        );

        assert_eq!(
            report.assessments()[1].capability(),
            &second
        );
    }

    // ------------------------------------------------------------------------
    // SCHEMA
    // ------------------------------------------------------------------------

    #[test]
    fn accelerator_adapter_schema_is_stable() {
        assert_eq!(
            ACCELERATOR_ADAPTER_SCHEMA_ID,
            "zamani.zhal.adapters.accelerator"
        );

        assert_eq!(
            ACCELERATOR_ADAPTER_SCHEMA_VERSION,
            1
        );
    }
}
