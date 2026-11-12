
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Platform Provider
//!
//! Path: src/zur/platforms/provider.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! PURPOSE
//!
//! Defines the ZUR-facing platform-provider integration boundary.
//!
//! A platform provider supplies explicitly obtained observations
//! about a host execution environment.
//!
//! Providers may represent:
//!
//! - Operating-system environments.
//! - Native execution environments.
//! - Embedded and freestanding environments.
//! - Sandboxed environments.
//! - Virtualized environments.
//! - Containerized environments.
//! - Remote execution environments.
//! - Heterogeneous computing environments.
//! - Future platform implementations.
//!
//! These examples do not define a closed platform inventory.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - PlatformProvider.
//! - PlatformObservation.
//! - The common host-environment observation operation.
//! - Cancellation-aware observation coordination.
//!
//! EXISTING AUTHORITIES
//!
//! - stdlib/provider.rs owns ProviderId, Provider,
//!   ProviderLifecycle, ProviderDescriptor and ProviderStatus.
//!
//! - stdlib/context.rs owns OperationContext and cancellation.
//!
//! - zur/platforms/host.rs owns HostEnvironment and
//!   its constituent platform, facility and interface descriptions.
//!
//! - zur/contracts/provider.rs owns execution-provider
//!   contracts and provider operation interfaces.
//!
//! - zur/error.rs owns ZurError and ZurResult.
//!
//! THIS MODULE DOES NOT OWN
//!
//! - Provider registration.
//! - Provider discovery.
//! - Provider authentication.
//! - Provider authorization.
//! - Platform selection.
//! - Hardware discovery.
//! - Device enumeration.
//! - Capability semantics.
//! - Requirement evaluation.
//! - Feasibility analysis.
//! - Resource allocation.
//! - Resource reservation.
//! - Backend registration.
//! - Compilation.
//! - Artifact manifests.
//! - Executable loading.
//! - Execution scheduling.
//! - Process creation.
//! - Operating-system system calls.
//! - Device drivers.
//! - HAL implementation.
//! - Domain-specific execution.
//!
//! These responsibilities remain with their existing authorities.
//!
//! FUNDAMENTAL INVARIANTS
//!
//! Provider registration != provider initialization.
//! Provider initialization != provider availability.
//! Platform observation != hardware discovery.
//! Platform observation != capability verification.
//! Platform observation != resource reservation.
//! Platform observation != execution feasibility.
//! Provider metadata != authenticated identity.
//! Provider status != guaranteed operation success.
//!
//! SCALABILITY
//!
//! No fixed maximum is introduced for:
//!
//! - Platform providers.
//! - Platform implementations.
//! - Host facilities.
//! - Execution interfaces.
//! - Host attributes.
//! - Operating systems.
//! - Architectures.
//! - Computational domains.
//! - Devices.
//! - Resources.
//! - Execution environments.
//!
//! Actual representation and storage are bounded by available
//! resources and the underlying platform.
//!
//! Literal infinite physical computation is not promised.
//!
//! SAFETY
//!
//! Safe Rust only.
//! No unsafe blocks.
//! No unsafe implementations.
//! No raw pointers.
//! No global mutable state.
//! No implicit hardware selection.
//! No fabricated availability.
//! No automatic resource allocation.
//!
//! GRAMMAR BOUNDARY
//!
//! Source-language hardware intent is defined by the grammar
//! and semantic authorities.
//!
//! Relevant existing grammar architecture includes:
//!
//! - grammar/hardware/
//! - grammar/spec/resources.md
//! - grammar/specification/
//! - grammar/execution/
//!
//! This module does not parse grammar, inspect AST nodes,
//! reinterpret semantic requirements or select hardware.
//!
//! The grammar expresses computational and hardware intent.
//! The compiler and runtime consume validated representations.
//!
//! INTEGRATION
//!
//! Upstream:
//!
//! - stdlib/provider.rs
//! - stdlib/context.rs
//! - zur/platforms/host.rs
//! - zur/error.rs
//!
//! Downstream:
//!
//! - zur/platforms/compatibility.rs
//! - zur/core/context.rs
//! - zur/execution/loader.rs
//! - zur/execution/dispatch.rs
//!
//! Platform observations are supplied explicitly.
//! Compatibility and execution decisions remain downstream.

use crate::stdlib::context::OperationContext;

use crate::stdlib::provider::{
    Provider,
    ProviderDescriptor,
    ProviderId,
    ProviderLifecycle,
    ProviderStatus,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::platforms::host::HostEnvironment;

use crate::zur::contracts::provider::ZurProviderFuture;

// =============================================================================
// COMPONENT IDENTIFICATION
// =============================================================================

const COMPONENT: &str = "zur.platforms.provider";

// =============================================================================
// PLATFORM PROVIDER CONTRACT
// =============================================================================

/// ZUR platform-provider integration contract.
///
/// A platform provider is an independently implemented source
/// of host-platform observations.
///
/// Implementations must also implement the canonical standard-library
/// Provider and ProviderLifecycle contracts.
///
/// The provider identity, descriptor, status, initialization and
/// shutdown operations remain owned by those existing contracts.
///
/// This trait adds only the platform-specific observation operation.
///
/// # Architectural boundaries
///
/// Implementations may obtain observations from:
///
/// - Operating-system facilities.
/// - Platform APIs.
/// - Explicitly supplied environment descriptions.
/// - Authorized platform integration services.
/// - External platform-management systems.
///
/// Actual observation mechanisms belong to the implementation.
///
/// This trait does not prescribe a particular operating system,
/// processor architecture, hardware vendor, or platform SDK.
///
/// # Cancellation
///
/// Implementations must observe the supplied OperationContext.
///
/// Long-running platform observations should check cancellation
/// at appropriate operation boundaries.
///
/// Cancellation is cooperative. It does not guarantee that an
/// underlying operating-system or external operation has stopped.
///
/// # Concurrency
///
/// The inherited Provider contract requires Send + Sync.
///
/// Implementations must therefore provide appropriate synchronization
/// for their internal mutable state.
///
/// No global mutable state is required.
///
/// # Consistency
///
/// An observation is a snapshot of information supplied by the
/// provider at the time the observation was obtained.
///
/// It is not a reservation, feasibility decision, authorization
/// decision, or guarantee that the environment remains unchanged.
///
/// # Errors
///
/// Implementations must return structured errors when observations
/// cannot be obtained or validated.
///
/// Unknown information must remain unknown rather than being guessed.
pub trait PlatformProvider: ProviderLifecycle {

    /// Obtains an immutable description of the host environment.
    ///
    /// The returned HostEnvironment is owned by the canonical
    /// host-platform module.
    ///
    /// The provider must not fabricate platform facilities,
    /// execution interfaces or availability observations.
    ///
    /// Missing information must remain absent or explicitly unknown.
    ///
    /// This operation must not:
    ///
    /// - Reserve physical resources.
    /// - Select execution devices.
    /// - Compile programs.
    /// - Load executable artifacts.
    /// - Execute programs.
    /// - Modify source-language semantics.
    ///
    /// Those operations belong to other architectural authorities.
    ///
    /// # Arguments
    ///
    /// * `context` - Shared operation context containing cancellation
    ///   and explicitly supplied operation extensions.
    ///
    /// # Returns
    ///
    /// An immutable host-environment description.
    ///
    /// # Errors
    ///
    /// Returns a structured error when observation fails,
    /// is cancelled, or cannot be completed.
    fn observe_host_environment<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, HostEnvironment>;
}

// =============================================================================
// PLATFORM OBSERVATION
// =============================================================================

/// An immutable host-platform observation.
///
/// This structure associates an observation with the canonical
/// identity of the provider that supplied it.
///
/// It deliberately reuses:
///
/// - ProviderId from stdlib/provider.rs.
/// - HostEnvironment from zur/platforms/host.rs.
///
/// It does not introduce a second platform identity system
/// or host-environment representation.
///
/// # Important
///
/// A PlatformObservation is evidence of what a provider reported.
///
/// It is not independent authentication of the provider or
/// verification of every reported property.
///
/// Capability verification and compatibility validation
/// remain with their respective authorities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformObservation {

    /// Identity of the provider that supplied the observation.
    provider_id: ProviderId,

    /// Immutable host-environment description.
    environment: HostEnvironment,
}

impl PlatformObservation {

    /// Creates a platform observation from an explicit provider
    /// identity and an existing host-environment description.
    ///
    /// This constructor does not establish provider authenticity,
    /// capability availability or hardware feasibility.
    ///
    /// It is public to support independently implemented platform
    /// adapters and integration tests.
    #[must_use]
    pub fn new(
        provider_id: ProviderId,
        environment: HostEnvironment,
    ) -> Self {
        Self {
            provider_id,
            environment,
        }
    }

    /// Returns the identity of the observing provider.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the immutable host-environment description.
    #[must_use]
    pub fn environment(&self) -> &HostEnvironment {
        &self.environment
    }

    /// Consumes the observation and returns its host environment.
    #[must_use]
    pub fn into_environment(self) -> HostEnvironment {
        self.environment
    }
}

// =============================================================================
// CANCELLATION-AWARE OBSERVATION
// =============================================================================

/// Obtains a host-platform observation through an explicitly
/// supplied platform provider.
///
/// This helper coordinates the observation operation without
/// introducing provider discovery, provider selection or
/// provider registration.
///
/// The caller supplies the provider explicitly.
///
/// The operation:
///
/// 1. Checks cancellation before starting.
/// 2. Invokes the supplied platform provider.
/// 3. Propagates actual provider failures.
/// 4. Checks cancellation after the provider returns.
/// 5. Associates the result with the provider's canonical identity.
///
/// It does not independently verify provider authenticity.
///
/// It does not establish that reported resources remain available.
///
/// # Arguments
///
/// * `provider` - Explicitly selected platform provider.
/// * `context` - Shared operation context.
///
/// # Returns
///
/// An immutable PlatformObservation.
///
/// # Errors
///
/// Returns:
///
/// - Cancellation if the operation was cancelled.
/// - The original provider error if observation failed.
/// - No fabricated successful observation.
pub async fn observe_platform(
    provider: &dyn PlatformProvider,
    context: &OperationContext,
) -> ZurResult<PlatformObservation> {

    if context.is_cancelled() {
        return Err(
            ZurError::cancelled(
                "platform observation cancelled before execution",
            )
            .with_component(COMPONENT)
            .with_operation("observe_platform"),
        );
    }

    let environment = provider
        .observe_host_environment(context)
        .await?;

    if context.is_cancelled() {
        return Err(
            ZurError::cancelled(
                "platform observation cancelled before completion",
            )
            .with_component(COMPONENT)
            .with_operation("observe_platform"),
        );
    }

    let provider_id = provider
        .descriptor()
        .id()
        .clone();

    Ok(PlatformObservation::new(
        provider_id,
        environment,
    ))
}

// =============================================================================
// PROVIDER METADATA HELPERS
// =============================================================================

/// Returns the canonical descriptor of a platform provider.
///
/// This helper reuses the standard-library Provider contract.
///
/// It does not create a platform-specific provider descriptor.
#[must_use]
pub fn platform_provider_descriptor(
    provider: &dyn PlatformProvider,
) -> &ProviderDescriptor {
    provider.descriptor()
}

/// Returns the canonical identity of a platform provider.
///
/// Provider identity remains owned by stdlib/provider.rs.
#[must_use]
pub fn platform_provider_id(
    provider: &dyn PlatformProvider,
) -> &ProviderId {
    provider.descriptor().id()
}

/// Returns the current observed provider status.
///
/// Provider status is not proof of hardware availability,
/// resource reservation or execution feasibility.
#[must_use]
pub fn platform_provider_status(
    provider: &dyn PlatformProvider,
) -> ProviderStatus {
    provider.status()
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    use crate::zur::platforms::host::{
        HostPlatformDescriptor,
    };

    fn provider_id() -> ProviderId {
        ProviderId::new(
            "org.zamani.test.platform",
        )
        .expect("test provider identity must be valid")
    }

    fn host_environment() -> HostEnvironment {
        let platform = HostPlatformDescriptor::from_id(
            "org.example.test-platform",
        )
        .expect("platform identity must be valid");

        HostEnvironment::builder(platform)
            .build()
            .expect("host environment must be constructible")
    }

    #[test]
    fn observation_preserves_provider_identity() {

        let identity = provider_id();

        let observation = PlatformObservation::new(
            identity.clone(),
            host_environment(),
        );

        assert_eq!(
            observation.provider_id(),
            &identity,
        );
    }

    #[test]
    fn observation_preserves_host_environment() {

        let environment = host_environment();

        let observation = PlatformObservation::new(
            provider_id(),
            environment.clone(),
        );

        assert_eq!(
            observation.environment(),
            &environment,
        );
    }

    #[test]
    fn observation_can_be_consumed() {

        let observation = PlatformObservation::new(
            provider_id(),
            host_environment(),
        );

        let environment = observation.into_environment();

        assert_eq!(
            environment.platform().id().as_str(),
            "org.example.test-platform",
        );
    }

    #[test]
    fn platform_observation_supports_open_world_provider_ids() {

        let identity = ProviderId::new(
            "future.photonics.platform.integration",
        )
        .expect("future provider identities must be accepted");

        let observation = PlatformObservation::new(
            identity.clone(),
            host_environment(),
        );

        assert_eq!(
            observation.provider_id(),
            &identity,
        );
    }

    #[test]
    fn platform_observation_is_cloneable() {

        let observation = PlatformObservation::new(
            provider_id(),
            host_environment(),
        );

        let cloned = observation.clone();

        assert_eq!(
            observation,
            cloned,
        );
    }
}
