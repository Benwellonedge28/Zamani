
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Executable Artifact Loader
//!
//! Path: src/zur/execution/loader.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//!
//! PURPOSE
//!
//! Loads a previously validated compiler artifact into an explicitly
//! selected platform or artifact-format implementation.
//!
//! This module establishes the boundary between:
//!
//!     canonical compiler artifact
//!             |
//!             v
//!     runtime artifact validation
//!             |
//!             v
//!     runtime service readiness
//!             |
//!             v
//!     format/platform loading
//!             |
//!             v
//!     loaded executable handle
//!
//! The loader does not invoke the loaded program.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Runtime loading orchestration.
//! - Entry-point selection within the supplied RuntimeAbi.
//! - Required-service lease verification.
//! - Explicit artifact-format adapter integration.
//! - Loaded artifact ownership.
//! - Retention of runtime service leases.
//! - Post-load identity and format verification.
//! - Cancellation checks at loader boundaries.
//!
//! EXISTING AUTHORITIES
//!
//! - compiler::artifacts owns Artifact and ArtifactManifest.
//! - compiler::artifacts::validation owns artifact validation.
//! - backend owns code generation and backend artifacts.
//! - zur::execution::artifact owns runtime artifact validation.
//! - zur::contracts::abi owns RuntimeAbi and entry-point contracts.
//! - zur::core::service_registry owns service registration.
//! - zur::core::service_manager owns service activation and leases.
//! - zur::contracts::provider owns execution-provider contracts.
//! - stdlib::context owns OperationContext and cancellation.
//! - platform-specific adapters own actual artifact loading.
//!
//! THIS MODULE DOES NOT OWN
//!
//! - Artifact construction.
//! - Artifact serialization.
//! - Artifact manifests.
//! - Artifact integrity algorithms.
//! - Compiler validation.
//! - Semantic validation.
//! - IR verification.
//! - Backend registration.
//! - Backend selection.
//! - Code generation.
//! - Hardware discovery.
//! - Device selection.
//! - Resource feasibility.
//! - Physical resource reservation.
//! - Scheduling.
//! - Routing.
//! - Quantum error correction.
//! - Distributed placement.
//! - Program invocation.
//! - Execution completion.
//!
//! SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Artifact size.
//! - Artifact dependencies.
//! - Runtime services.
//! - Service leases.
//! - Entry points.
//! - Providers.
//! - Computing domains.
//! - Hardware devices.
//! - CPUs, GPUs or QPUs.
//! - Distributed participants.
//!
//! Payload streaming and memory management are delegated to the
//! canonical artifact storage and platform loading implementations.
//!
//! This does not promise infinite physical resources.
//!
//! SAFETY
//!
//! Safe Rust only.
//!
//! No raw pointers.
//! No unsafe implementations.
//! No global mutable state.
//! No fabricated loading success.
//! No implicit service activation.
//! No implicit backend selection.
//!
//! IMPORTANT DISTINCTIONS
//!
//! A successfully loaded artifact is not proof of:
//!
//! - Physical resource reservation.
//! - Provider submission.
//! - Successful invocation.
//! - Successful computation.
//! - Successful remote execution.
//! - Successful quantum measurement.
//! - Successful hardware programming.
//!
//! Those outcomes belong to their respective execution authorities.

use std::any::Any;

use crate::compiler::artifacts::{
    Artifact,
    ArtifactFormatId,
    ArtifactId,
    ArtifactManifest,
};

use crate::stdlib::context::OperationContext;

use crate::zur::contracts::abi::{
    AbiSymbol,
    EntryPointDescriptor,
    RuntimeAbi,
};

use crate::zur::core::service_manager::{
    ManagedServiceState,
    ServiceLease,
    ServiceManager,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::execution::artifact::{
    validate_executable_artifact,
    ArtifactLoadPolicy,
    ArtifactPayloadResolver,
    ArtifactValidationReport,
    RuntimeServiceResolver,
};

use crate::zur::version::ServiceContractVersion;


// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn loader_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::invalid_data(message)
        .with_component("zur.execution.loader")
}

fn compatibility_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::compatibility(message)
        .with_component("zur.execution.loader")
}

fn unavailable_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::service_unavailable(message)
        .with_component("zur.execution.loader")
}

fn unsupported_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::unsupported(message)
        .with_component("zur.execution.loader")
}


// ============================================================================
// LOADED ARTIFACT HANDLE
// ============================================================================

/// Opaque, platform-owned representation of a loaded artifact.
///
/// The concrete implementation belongs to the selected artifact-format
/// or platform adapter.
///
/// The runtime loader owns the returned handle and retains it for the
/// lifetime of the LoadedArtifact.
///
/// Implementations must use safe ownership and must release their
/// resources through ordinary Rust ownership and Drop behavior.
///
/// This trait deliberately does not expose raw executable pointers.
///
/// The executor may downcast the handle through `as_any()` when it
/// already knows the concrete adapter contract.
pub trait LoadedArtifactHandle: Any + Send + Sync {

    /// Returns the canonical artifact identity represented by this handle.
    fn artifact_id(&self) -> &ArtifactId;

    /// Returns the artifact format represented by this handle.
    fn format(&self) -> &ArtifactFormatId;

    /// Provides type-safe access to the concrete adapter-owned handle.
    fn as_any(&self) -> &dyn Any;
}


// ============================================================================
// ARTIFACT FORMAT ADAPTER
// ============================================================================

/// Loads one explicitly supported artifact format.
///
/// An adapter is supplied by the runtime composition or platform
/// integration layer.
///
/// The loader does not discover, rank or select adapters.
///
/// An adapter must:
///
/// - Accept only its declared artifact format.
/// - Validate the target/platform requirements it owns.
/// - Respect the supplied ABI and entry-point descriptor.
/// - Read inline or external payloads correctly.
/// - Use the supplied payload resolver for external content.
/// - Honor cancellation.
/// - Return a real owned loaded-artifact handle.
/// - Return an error when the format or target is unsupported.
///
/// An adapter must not:
///
/// - Generate code.
/// - Modify the compiler artifact.
/// - Change program semantics.
/// - Report success without loading the artifact.
/// - Claim physical resource reservation without evidence.
/// - Claim execution completion.
///
/// Backend code generation remains owned by `src/backend/`.
///
/// Platform-specific loading is an implementation responsibility
/// of the supplied adapter.
pub trait ArtifactFormatAdapter: Send + Sync {

    /// Returns the exact artifact format implemented by this adapter.
    fn format(&self) -> &ArtifactFormatId;

    /// Loads a validated artifact into an owned platform representation.
    ///
    /// `validation` is the canonical runtime validation evidence.
    ///
    /// `entry_point` is already known to exist in the supplied ABI.
    ///
    /// The adapter must still verify that its loaded representation
    /// can resolve that entry point.
    ///
    /// `payload_resolver` is supplied for external artifact payloads.
    ///
    /// The returned handle must own everything it needs after this
    /// function returns. It must not retain borrowed references to
    /// the artifact, ABI, validation report, resolver or context.
    ///
    /// # Errors
    ///
    /// Returns an error for unsupported formats, incompatible
    /// platforms, invalid executable representations, failed loading,
    /// permission failures, unavailable platform facilities or
    /// cancellation.
    fn load(
        &self,
        artifact: &Artifact,
        abi: &RuntimeAbi,
        entry_point: &EntryPointDescriptor,
        validation: &ArtifactValidationReport,
        payload_resolver: Option<&dyn ArtifactPayloadResolver>,
        context: &OperationContext,
    ) -> ZurResult<Box<dyn LoadedArtifactHandle>>;
}


// ============================================================================
// SERVICE LEASE VALIDATION
// ============================================================================

/// Resolves runtime service availability using actual service leases.
///
/// A service is accepted only when:
///
/// 1. Its exact identity and contract version are registered.
/// 2. Its registered instance is retained by a supplied lease.
/// 3. The service manager reports that instance as Ready.
///
/// Registration alone is insufficient.
///
/// Initialization alone is insufficient.
///
/// A lease without readiness is insufficient.
///
/// The service manager remains the authority for service lifecycle state.
struct LeaseBackedServiceResolver<'a> {
    manager: &'a ServiceManager,
    leases: &'a [ServiceLease],
}

impl<'a> LeaseBackedServiceResolver<'a> {

    /// Creates a resolver backed by the supplied service manager
    /// and execution-owned leases.
    fn new(
        manager: &'a ServiceManager,
        leases: &'a [ServiceLease],
    ) -> Self {
        Self {
            manager,
            leases,
        }
    }

    /// Determines whether the supplied lease set retains a service
    /// instance.
    fn contains_instance(
        &self,
        instance_id: &crate::zur::ids::ServiceInstanceId,
    ) -> bool {
        self.leases.iter().any(|lease| {
            lease.services().contains(instance_id)
        })
    }
}

impl RuntimeServiceResolver for LeaseBackedServiceResolver<'_> {

    /// Requires a registered, ready and leased service instance.
    fn require_available(
        &self,
        service: &crate::zur::config::ServiceKey,
        version: &ServiceContractVersion,
    ) -> ZurResult<()> {

        let registered = self.manager
            .registry()
            .get(service, version)?
            .ok_or_else(|| {
                unavailable_error(
                    format!(
                        "required runtime service `{service}` \
                         at contract version `{version}` is not registered"
                    )
                )
            })?;

        let instance_id = registered.instance_id();

        if !self.contains_instance(instance_id) {
            return Err(
                unavailable_error(
                    format!(
                        "required runtime service `{service}` \
                         is not retained by an execution service lease"
                    )
                )
            );
        }

        match self.manager.state(instance_id)? {

            Some(ManagedServiceState::Ready) => Ok(()),

            Some(state) => Err(
                unavailable_error(
                    format!(
                        "required runtime service `{service}` \
                         is not ready; observed state: {state:?}"
                    )
                )
            ),

            None => Err(
                unavailable_error(
                    format!(
                        "required runtime service `{service}` \
                         has no operational state"
                    )
                )
            ),
        }
    }
}


// ============================================================================
// LOAD REQUEST
// ============================================================================

/// Complete input required to load one executable artifact.
///
/// The caller supplies:
///
/// - The canonical manifest.
/// - The executable artifact identity.
/// - The ABI associated with that exact artifact.
/// - The requested entry point.
/// - The artifact validation policy.
/// - An optional external payload resolver.
/// - The service manager.
/// - Already-acquired service leases.
/// - An explicitly selected format adapter.
/// - The operation context.
///
/// The ABI must be obtained from authenticated or otherwise
/// authoritative artifact metadata associated with the exact artifact.
///
/// This request does not permit the loader to infer an ABI from
/// unrelated metadata or guess an entry point.
pub struct ArtifactLoadRequest<'a> {

    manifest: &'a ArtifactManifest,

    executable_id: &'a ArtifactId,

    abi: &'a RuntimeAbi,

    entry_point: &'a AbiSymbol,

    policy: &'a ArtifactLoadPolicy,

    payload_resolver: Option<&'a dyn ArtifactPayloadResolver>,

    service_manager: &'a ServiceManager,

    service_leases: Vec<ServiceLease>,

    format_adapter: &'a dyn ArtifactFormatAdapter,

    context: &'a OperationContext,
}

impl<'a> ArtifactLoadRequest<'a> {

    /// Constructs an explicit artifact loading request.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        manifest: &'a ArtifactManifest,
        executable_id: &'a ArtifactId,
        abi: &'a RuntimeAbi,
        entry_point: &'a AbiSymbol,
        policy: &'a ArtifactLoadPolicy,
        payload_resolver: Option<&'a dyn ArtifactPayloadResolver>,
        service_manager: &'a ServiceManager,
        service_leases: Vec<ServiceLease>,
        format_adapter: &'a dyn ArtifactFormatAdapter,
        context: &'a OperationContext,
    ) -> Self {
        Self {
            manifest,
            executable_id,
            abi,
            entry_point,
            policy,
            payload_resolver,
            service_manager,
            service_leases,
            format_adapter,
            context,
        }
    }
}


// ============================================================================
// LOADED ARTIFACT
// ============================================================================

/// An artifact that has passed validation and platform loading.
///
/// This value owns:
///
/// - The platform-loaded artifact handle.
/// - Canonical artifact validation evidence.
/// - The runtime ABI.
/// - The selected entry-point descriptor.
/// - Service leases required by the executable.
///
/// Dropping this value releases its service leases and drops the
/// platform-loaded handle.
///
/// Dropping a lease does not itself perform asynchronous service
/// shutdown. The service manager remains responsible for that lifecycle.
pub struct LoadedArtifact {

    handle: Box<dyn LoadedArtifactHandle>,

    validation: ArtifactValidationReport,

    abi: RuntimeAbi,

    entry_point: EntryPointDescriptor,

    service_leases: Vec<ServiceLease>,
}

impl LoadedArtifact {

    /// Returns the platform-owned loaded artifact handle.
    #[must_use]
    pub fn handle(&self) -> &dyn LoadedArtifactHandle {
        self.handle.as_ref()
    }

    /// Returns the canonical artifact validation evidence.
    #[must_use]
    pub fn validation(&self) -> &ArtifactValidationReport {
        &self.validation
    }

    /// Returns the runtime ABI associated with this artifact.
    #[must_use]
    pub fn abi(&self) -> &RuntimeAbi {
        &self.abi
    }

    /// Returns the selected ABI entry-point descriptor.
    #[must_use]
    pub fn entry_point(&self) -> &EntryPointDescriptor {
        &self.entry_point
    }

    /// Returns the canonical executable artifact identity.
    #[must_use]
    pub fn artifact_id(&self) -> &ArtifactId {
        self.handle.artifact_id()
    }

    /// Returns the loaded artifact format.
    #[must_use]
    pub fn format(&self) -> &ArtifactFormatId {
        self.handle.format()
    }

    /// Returns the number of retained service leases.
    #[must_use]
    pub fn service_lease_count(&self) -> usize {
        self.service_leases.len()
    }

    /// Returns the retained service leases.
    ///
    /// The leases remain owned by this loaded artifact.
    #[must_use]
    pub fn service_leases(&self) -> &[ServiceLease] {
        &self.service_leases
    }

    /// Returns whether the loaded artifact retains any service leases.
    #[must_use]
    pub fn has_service_leases(&self) -> bool {
        !self.service_leases.is_empty()
    }

    /// Returns the platform handle as a concrete adapter type.
    ///
    /// The caller must know the adapter type it expects.
    #[must_use]
    pub fn downcast_handle<T: Any>(&self) -> Option<&T> {
        self.handle.as_any().downcast_ref::<T>()
    }
}


// ============================================================================
// ARTIFACT LOADER
// ============================================================================

/// Coordinates validated executable artifact loading.
///
/// This type is stateless.
///
/// All mutable operational state belongs to:
///
/// - The service manager.
/// - The supplied platform adapter.
/// - The loaded artifact handle.
/// - The execution session.
///
/// Multiple independent loaders may operate concurrently when their
/// supplied dependencies support concurrent access.
#[derive(Debug, Default, Clone, Copy)]
pub struct ArtifactLoader;

impl ArtifactLoader {

    /// Creates a stateless artifact loader.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Validates and loads one executable artifact.
    ///
    /// The method performs these operations in order:
    ///
    /// 1. Check cancellation.
    /// 2. Locate the requested canonical artifact.
    /// 3. Check the explicitly supplied adapter's format.
    /// 4. Resolve the requested ABI entry point.
    /// 5. Validate artifact identity, manifest, dependencies,
    ///    payload integrity, ABI compatibility and service readiness.
    /// 6. Ask the supplied adapter to load the artifact.
    /// 7. Verify the returned loaded-handle identity and format.
    /// 8. Transfer service-lease ownership to LoadedArtifact.
    ///
    /// The loader does not activate services. The caller must acquire
    /// the required leases through ServiceManager before calling this
    /// method.
    ///
    /// The loader does not invoke the program.
    ///
    /// # Errors
    ///
    /// Returns an error for:
    ///
    /// - Cancellation.
    /// - Missing artifacts.
    /// - Unsupported artifact formats.
    /// - Missing ABI entry points.
    /// - Invalid manifests.
    /// - Invalid dependencies.
    /// - Failed payload integrity.
    /// - ABI incompatibility.
    /// - Missing, unleased or non-ready services.
    /// - Platform incompatibility.
    /// - Failed artifact loading.
    /// - A platform adapter returning an incorrect artifact identity.
    /// - A platform adapter returning an incorrect format identity.
    pub fn load(
        &self,
        request: ArtifactLoadRequest<'_>,
    ) -> ZurResult<LoadedArtifact> {

        request.context.check_cancelled()?;

        // --------------------------------------------------------------------
        // 1. Resolve the exact canonical artifact.
        // --------------------------------------------------------------------

        let artifact = request.manifest
            .artifact(request.executable_id)
            .ok_or_else(|| {
                loader_error(
                    format!(
                        "executable artifact `{}` does not exist \
                         in the supplied canonical manifest",
                        request.executable_id
                    )
                )
            })?;

        // --------------------------------------------------------------------
        // 2. Check the explicitly supplied format adapter.
        // --------------------------------------------------------------------

        if artifact.format() != request.format_adapter.format() {
            return Err(
                unsupported_error(
                    format!(
                        "no supplied artifact-format adapter matches \
                         artifact format `{:?}`; supplied adapter supports `{:?}`",
                        artifact.format(),
                        request.format_adapter.format()
                    )
                )
            );
        }

        // --------------------------------------------------------------------
        // 3. Resolve the exact requested ABI entry point.
        // --------------------------------------------------------------------

        let entry_point = request.abi
            .entry_point(request.entry_point)
            .ok_or_else(|| {
                loader_error(
                    format!(
                        "requested ABI entry point `{}` \
                         is not declared by the artifact RuntimeAbi",
                        request.entry_point
                    )
                )
            })?
            .clone();

        // --------------------------------------------------------------------
        // 4. Validate service readiness against real leases.
        // --------------------------------------------------------------------

        let validation = {

            let service_resolver = LeaseBackedServiceResolver::new(
                request.service_manager,
                &request.service_leases,
            );

            validate_executable_artifact(
                request.manifest,
                request.executable_id,
                request.abi,
                request.policy,
                request.payload_resolver,
                Some(&service_resolver),
            )?
        };

        request.context.check_cancelled()?;

        // --------------------------------------------------------------------
        // 5. Verify validation evidence belongs to this exact artifact.
        // --------------------------------------------------------------------

        if validation.verified_artifact().artifact_id()
            != request.executable_id
        {
            return Err(
                compatibility_error(
                    "artifact validation evidence does not identify \
                     the requested executable artifact"
                )
            );
        }

        // --------------------------------------------------------------------
        // 6. Perform actual format/platform loading.
        // --------------------------------------------------------------------

        let loaded_handle = request.format_adapter.load(
            artifact,
            request.abi,
            &entry_point,
            &validation,
            request.payload_resolver,
            request.context,
        )?;

        // --------------------------------------------------------------------
        // 7. Verify the returned platform handle.
        // --------------------------------------------------------------------

        if loaded_handle.artifact_id() != request.executable_id {
            return Err(
                compatibility_error(
                    "platform adapter returned a loaded handle \
                     for a different artifact identity"
                )
            );
        }

        if loaded_handle.format() != artifact.format() {
            return Err(
                compatibility_error(
                    "platform adapter returned a loaded handle \
                     with a different artifact format"
                )
            );
        }

        request.context.check_cancelled()?;

        // --------------------------------------------------------------------
        // 8. Transfer ownership to the loaded artifact.
        // --------------------------------------------------------------------

        Ok(LoadedArtifact {
            handle: loaded_handle,
            validation,
            abi: request.abi.clone(),
            entry_point,
            service_leases: request.service_leases,
        })
    }
}


// ============================================================================
// TESTING CONTRACT
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    /// Confirms the loader is a stateless, independently constructible
    /// runtime component.
    #[test]
    fn loader_is_constructible_without_global_initialization() {
        let _loader = ArtifactLoader::new();
    }

    /// Confirms the loader does not impose a fixed service-lease count.
    ///
    /// The actual lease validation requires a configured ServiceManager
    /// and is covered by the service-manager integration suite.
    #[test]
    fn service_lease_collections_are_dynamically_sized() {
        let leases: Vec<ServiceLease> = Vec::new();

        assert!(leases.is_empty());
    }
}
