
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Target Discovery
//!
//! Path: src/compiler/planning/target_discovery.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Discover and aggregate registered compilation target descriptors.
//!
//! A target describes a possible compilation or realization environment.
//! Discovery establishes that a provider has reported a target. It does
//! not establish that:
//!
//! - The target is physically available.
//! - The target is currently operational.
//! - A backend can generate executable output for it.
//! - The target satisfies a program's capabilities.
//! - The target has sufficient resources.
//! - The caller is authorized to use the target.
//!
//! Those questions belong to their respective authoritative subsystems.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Discovery provider identity.
//! - Discovery request and policy.
//! - Instance-owned discovery provider registration.
//! - Provider invocation.
//! - Target descriptor aggregation.
//! - Duplicate identity detection.
//! - Discovery diagnostics.
//! - Discovery result ordering.
//! - Partial discovery failure reporting.
//!
//! This module does not own:
//!
//! - Target descriptor definitions.
//! - Capability definitions.
//! - Semantic requirements.
//! - Backend implementations.
//! - Backend registration.
//! - Physical hardware inspection.
//! - Device allocation.
//! - Runtime scheduling.
//! - Physical routing.
//! - Resource management.
//! - IR representations.
//! - Source-language grammar.
//!
//! ## AUTHORITATIVE DEPENDENCIES
//!
//! Target descriptors:
//!     crate::compiler::configuration::target
//!
//! Capability analysis:
//!     crate::compiler::analysis::capabilities
//!
//! Existing compiler orchestration:
//!     crate::compiler
//!
//! Backend implementation:
//!     crate::backend
//!
//! Runtime:
//!     crate::runtime
//!
//! ## SCALABILITY
//!
//! No fixed maximum is imposed on:
//!
//! - Registered providers.
//! - Discovered targets.
//! - Architectures.
//! - Vendors.
//! - Processing elements.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Accelerators.
//! - Distributed nodes.
//! - Future computing domains.
//!
//! Collections grow according to available resources.
//!
//! An explicit caller-controlled discovery budget can be introduced by
//! the orchestration layer without changing target identity semantics.
//!
//! Literal infinity is not a realizable finite-computer guarantee.
//!
//! ## DETERMINISM
//!
//! Provider registration and target aggregation use ordered maps.
//! Results are sorted by stable target identity.
//! Provider registration order does not determine target ordering.
//!
//! ## CONCURRENCY
//!
//! The registry is instance-owned.
//! No global mutable registry is used.
//!
//! Providers must be safe to invoke through shared references.
//! Concurrent discovery execution is deliberately left to the caller's
//! orchestration policy.
//!
//! ## INTEGRATION
//!
//! Parent module:
//!     src/compiler/planning/mod.rs
//!
//! Required declaration:
//!     pub mod target_discovery;
//!
//! Existing compiler root:
//!     src/compiler/mod.rs
//!
//! Required declaration:
//!     pub mod planning;
//!
//! Existing crate root already exposes the compiler subsystem.
//!
//! The next planning stages consume DiscoveryReport:
//!
//!     capability_matching.rs
//!     resource_planning.rs
//!     partitioning.rs
//!     feasibility.rs
//!
//! None of those modules needs to redefine target discovery.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::compiler::configuration::target::{
    TargetDescriptor,
    TargetError,
    TargetId,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable identity of the target discovery subsystem.
pub const TARGET_DISCOVERY_MODULE_NAME: &str =
    "zamani.compiler.planning.target_discovery";

/// Target discovery API version.
pub const TARGET_DISCOVERY_API_VERSION: u16 = 1;

// ============================================================================
// PROVIDER IDENTITY
// ============================================================================

/// Stable identity of a target discovery provider.
///
/// Provider identifiers are open-ended and are not hardware classifications.
///
/// Examples:
///
/// - `backend.llvm`
— `backend.wasm`
/// - `toolchain.custom`
/// - `hardware.inventory`
/// - `quantum.provider`
/// - `distributed.inventory`
///
/// These examples do not constitute a closed provider registry.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DiscoveryProviderId(String);

impl DiscoveryProviderId {
    /// Creates a validated provider identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, DiscoveryError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(DiscoveryError::EmptyProviderId);
        }

        if value.chars().any(char::is_control) {
            return Err(DiscoveryError::InvalidProviderId {
                value,
                reason: "control characters are forbidden".to_owned(),
            });
        }

        if value.trim() != value {
            return Err(DiscoveryError::InvalidProviderId {
                value,
                reason: "surrounding whitespace is forbidden".to_owned(),
            });
        }

        Ok(Self(value))
    }

    /// Returns the stable provider identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DiscoveryProviderId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// DISCOVERY REQUEST
// ============================================================================

/// Controls the scope of a target discovery operation.
///
/// An empty requested-target set means that the caller has not restricted
/// discovery by target identity.
///
/// This does not mean that every conceivable target exists.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DiscoveryRequest {
    requested_targets: BTreeSet<TargetId>,

    include_metadata_only: bool,
}

impl DiscoveryRequest {
    /// Creates an unrestricted discovery request.
    pub fn all_registered() -> Self {
        Self::default()
    }

    /// Creates a discovery request restricted to specified target IDs.
    pub fn for_targets(
        targets: impl IntoIterator<Item = TargetId>,
    ) -> Self {
        Self {
            requested_targets: targets.into_iter().collect(),
            include_metadata_only: true,
        }
    }

    /// Restricts the request to explicitly supplied target identifiers.
    pub fn with_targets(
        mut self,
        targets: impl IntoIterator<Item = TargetId>,
    ) -> Self {
        self.requested_targets = targets.into_iter().collect();
        self
    }

    /// Sets whether descriptive and non-executable targets are included.
    ///
    /// The default is false for general discovery.
    ///
    /// Target roles are still determined by their canonical descriptor.
    pub fn include_metadata_only(
        mut self,
        include: bool,
    ) -> Self {
        self.include_metadata_only = include;
        self
    }

    /// Returns the requested target identifiers.
    #[must_use]
    pub fn requested_targets(&self) -> &BTreeSet<TargetId> {
        &self.requested_targets
    }

    /// Returns whether metadata-only targets are requested.
    #[must_use]
    pub fn includes_metadata_only(&self) -> bool {
        self.include_metadata_only
    }

    /// Returns whether the request restricts target identity.
    #[must_use]
    pub fn is_restricted(&self) -> bool {
        !self.requested_targets.is_empty()
    }

    /// Determines whether a target identity is requested.
    #[must_use]
    pub fn accepts_identity(&self, target: &TargetId) -> bool {
        self.requested_targets.is_empty()
            || self.requested_targets.contains(target)
    }
}

// ============================================================================
// DISCOVERY POLICY
// ============================================================================

/// Controls discovery failure handling.
///
/// A failed provider must never silently disappear from the result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiscoveryPolicy {
    require_all_providers: bool,

    reject_duplicate_target_identities: bool,
}

impl DiscoveryPolicy {
    /// Creates a policy suitable for ordinary discovery.
    ///
    /// Provider failures are reported while successful providers can
    /// still contribute valid results.
    pub const fn best_effort() -> Self {
        Self {
            require_all_providers: false,
            reject_duplicate_target_identities: true,
        }
    }

    /// Creates a policy requiring every registered provider to succeed.
    pub const fn strict() -> Self {
        Self {
            require_all_providers: true,
            reject_duplicate_target_identities: true,
        }
    }

    /// Controls whether any provider failure invalidates the discovery.
    pub const fn require_all_providers(
        mut self,
        required: bool,
    ) -> Self {
        self.require_all_providers = required;
        self
    }

    /// Controls whether conflicting target identities invalidate discovery.
    ///
    /// Conflicts are always preserved in the report.
    ///
    /// Disabling strict rejection does not permit the system to silently
    /// select one conflicting descriptor.
    pub const fn reject_duplicate_target_identities(
        mut self,
        reject: bool,
    ) -> Self {
        self.reject_duplicate_target_identities = reject;
        self
    }

    /// Returns whether every provider must succeed.
    #[must_use]
    pub const fn requires_all_providers(self) -> bool {
        self.require_all_providers
    }

    /// Returns whether identity conflicts invalidate discovery.
    #[must_use]
    pub const fn rejects_duplicate_target_identities(self) -> bool {
        self.reject_duplicate_target_identities
    }
}

impl Default for DiscoveryPolicy {
    fn default() -> Self {
        Self::best_effort()
    }
}

// ============================================================================
// PROVIDER ERROR
// ============================================================================

/// Error reported by an individual discovery provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderFailure {
    /// Stable provider identity.
    pub provider: DiscoveryProviderId,

    /// Human-readable failure description.
    pub message: String,
}

impl ProviderFailure {
    /// Creates a provider failure.
    pub fn new(
        provider: DiscoveryProviderId,
        message: impl Into<String>,
    ) -> Self {
        Self {
            provider,
            message: message.into(),
        }
    }
}

impl fmt::Display for ProviderFailure {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "discovery provider `{}` failed: {}",
            self.provider,
            self.message
        )
    }
}

impl Error for ProviderFailure {}

// ============================================================================
// DISCOVERY PROVIDER
// ============================================================================

/// Interface implemented by registered target discovery providers.
///
/// A provider supplies target descriptors from its own authoritative source.
///
/// Examples include:
///
/// - A backend integration.
/// - A toolchain integration.
/// - A deployment inventory.
/// - A hardware inventory service.
/// - A remote computing provider.
///
/// A provider must not allocate devices as part of discovery.
///
/// Implementations must be deterministic when their underlying inventory
/// and request are unchanged.
///
/// Providers must report actual knowledge rather than fabricate targets.
///
/// Providers must return an error if they cannot establish trustworthy
/// descriptors for their discovery operation.
pub trait TargetDiscoveryProvider: Send + Sync {
    /// Returns the stable identity of this provider.
    fn id(&self) -> &DiscoveryProviderId;

    /// Discovers target descriptors.
    ///
    /// The method must not:
    ///
    /// - Allocate hardware.
    /// - Execute user programs.
    /// - Modify the canonical semantic model.
    /// - Modify the compiler IR.
    /// - Claim capability compatibility.
    ///
    /// Returned descriptors are validated by the discovery subsystem.
    fn discover(
        &self,
        request: &DiscoveryRequest,
    ) -> Result<Vec<TargetDescriptor>, ProviderFailure>;
}

// ============================================================================
// PROVIDER REGISTRY
// ============================================================================

/// Instance-owned target discovery provider registry.
///
/// Registration is explicit. There are no global mutable registries
/// or import-time registration side effects.
///
/// Provider identifiers must be unique within one registry.
///
/// Different compiler sessions can use different registries.
#[derive(Default)]
pub struct TargetDiscoveryRegistry {
    providers: BTreeMap<
        DiscoveryProviderId,
        Arc<dyn TargetDiscoveryProvider>,
    >,
}

impl TargetDiscoveryRegistry {
    /// Creates an empty provider registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a discovery provider.
    ///
    /// Duplicate provider IDs are rejected. Existing providers are never
    /// silently overwritten.
    pub fn register<P>(
        &mut self,
        provider: P,
    ) -> Result<(), DiscoveryError>
    where
        P: TargetDiscoveryProvider + 'static,
    {
        self.register_shared(Arc::new(provider))
    }

    /// Registers a shared discovery provider.
    pub fn register_shared(
        &mut self,
        provider: Arc<dyn TargetDiscoveryProvider>,
    ) -> Result<(), DiscoveryError> {
        let id = provider.id().clone();

        if self.providers.contains_key(&id) {
            return Err(DiscoveryError::DuplicateProvider {
                provider: id,
            });
        }

        self.providers.insert(id, provider);

        Ok(())
    }

    /// Unregisters a provider by stable identity.
    ///
    /// Returns true if a provider was removed.
    pub fn unregister(
        &mut self,
        id: &DiscoveryProviderId,
    ) -> bool {
        self.providers.remove(id).is_some()
    }

    /// Returns whether a provider is registered.
    #[must_use]
    pub fn contains(
        &self,
        id: &DiscoveryProviderId,
    ) -> bool {
        self.providers.contains_key(id)
    }

    /// Returns the number of registered providers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    /// Returns whether the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    /// Returns registered provider identities in deterministic order.
    pub fn provider_ids(
        &self,
    ) -> impl Iterator<Item = &DiscoveryProviderId> {
        self.providers.keys()
    }

    /// Discovers targets through all registered providers.
    ///
    /// The result preserves provider failures and identity conflicts.
    ///
    /// Strict policies return an error if a provider fails or an identity
    /// conflict is detected.
    pub fn discover(
        &self,
        request: &DiscoveryRequest,
        policy: DiscoveryPolicy,
    ) -> Result<DiscoveryReport, DiscoveryError> {
        let mut report = DiscoveryReport::new();

        for (provider_id, provider) in &self.providers {
            let result = provider.discover(request);

            match result {
                Ok(descriptors) => {
                    report.successful_providers += 1;

                    for descriptor in descriptors {
                        if !request.accepts_identity(descriptor.id()) {
                            report.rejected_descriptors.push(
                                RejectedDescriptor {
                                    provider: provider_id.clone(),
                                    target: descriptor.id().clone(),
                                    reason: RejectionReason::NotRequested,
                                },
                            );

                            continue;
                        }

                        if let Err(error) = descriptor.validate() {
                            report.rejected_descriptors.push(
                                RejectedDescriptor {
                                    provider: provider_id.clone(),
                                    target: descriptor.id().clone(),
                                    reason: RejectionReason::InvalidDescriptor(
                                        error.to_string(),
                                    ),
                                },
                            );

                            continue;
                        }

                        if !request.includes_metadata_only()
                            && descriptor.role().as_str()
                                != "execution"
                        {
                            report.rejected_descriptors.push(
                                RejectedDescriptor {
                                    provider: provider_id.clone(),
                                    target: descriptor.id().clone(),
                                    reason: RejectionReason::MetadataOnly,
                                },
                            );

                            continue;
                        }

                        report.insert_descriptor(
                            provider_id.clone(),
                            descriptor,
                        );
                    }
                }

                Err(failure) => {
                    report.failures.push(failure);
                }
            }
        }

        if policy.requires_all_providers()
            && !report.failures.is_empty()
        {
            return Err(DiscoveryError::ProviderFailures {
                failures: report.failures,
            });
        }

        if policy.rejects_duplicate_target_identities()
            && !report.conflicts.is_empty()
        {
            return Err(DiscoveryError::TargetIdentityConflicts {
                conflicts: report.conflicts,
            });
        }

        Ok(report)
    }
}

// ============================================================================
// DISCOVERED TARGET
// ============================================================================

/// A target descriptor together with its discovery provenance.
///
/// Multiple providers may report the same identical descriptor.
///
/// Identical reports are consolidated without losing provider provenance.
///
/// Conflicting descriptors are not consolidated.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveredTarget {
    descriptor: TargetDescriptor,

    providers: BTreeSet<DiscoveryProviderId>,
}

impl DiscoveredTarget {
    /// Creates a discovered target.
    pub fn new(
        descriptor: TargetDescriptor,
        provider: DiscoveryProviderId,
    ) -> Self {
        let mut providers = BTreeSet::new();
        providers.insert(provider);

        Self {
            descriptor,
            providers,
        }
    }

    /// Returns the target descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &TargetDescriptor {
        &self.descriptor
    }

    /// Returns the target identity.
    #[must_use]
    pub fn id(&self) -> &TargetId {
        self.descriptor.id()
    }

    /// Returns all providers reporting the identical descriptor.
    #[must_use]
    pub fn providers(&self) -> &BTreeSet<DiscoveryProviderId> {
        &self.providers
    }

    /// Returns the number of providers reporting this target.
    #[must_use]
    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }
}

// ============================================================================
// TARGET IDENTITY CONFLICT
// ============================================================================

/// Conflicting descriptors reported for one target identity.
///
/// The discovery system must not silently choose one descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetIdentityConflict {
    /// Conflicting target identity.
    pub target: TargetId,

    /// Providers that supplied conflicting descriptors.
    pub providers: BTreeSet<DiscoveryProviderId>,

    /// Number of distinct descriptors reported.
    pub descriptor_count: usize,
}

// ============================================================================
// REJECTED DESCRIPTORS
// ============================================================================

/// Reason a provider-reported descriptor was excluded.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RejectionReason {
    /// The descriptor was not requested.
    NotRequested,

    /// The descriptor does not represent an execution target.
    MetadataOnly,

    /// Structural validation failed.
    InvalidDescriptor(String),
}

/// A descriptor rejected during discovery.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RejectedDescriptor {
    /// Provider that supplied the descriptor.
    pub provider: DiscoveryProviderId,

    /// Rejected target identity.
    pub target: TargetId,

    /// Rejection reason.
    pub reason: RejectionReason,
}

// ============================================================================
// DISCOVERY REPORT
// ============================================================================

/// Complete result of a target discovery operation.
///
/// This report distinguishes:
///
/// - Successfully discovered targets.
/// - Providers that succeeded.
/// - Providers that failed.
/// - Invalid or unrequested descriptors.
/// - Conflicting target identities.
///
/// An empty report is not proof that no hardware exists. It means that
/// the registered providers supplied no accepted targets.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DiscoveryReport {
    targets: BTreeMap<TargetId, DiscoveredTarget>,

    conflicts: Vec<TargetIdentityConflict>,

    failures: Vec<ProviderFailure>,

    rejected_descriptors: Vec<RejectedDescriptor>,

    successful_providers: usize,
}

impl DiscoveryReport {
    /// Creates an empty discovery report.
    pub fn new() -> Self {
        Self::default()
    }

    fn insert_descriptor(
        &mut self,
        provider: DiscoveryProviderId,
        descriptor: TargetDescriptor,
    ) {
        let id = descriptor.id().clone();

        match self.targets.get_mut(&id) {
            None => {
                self.targets.insert(
                    id,
                    DiscoveredTarget::new(descriptor, provider),
                );
            }

            Some(existing) => {
                if existing.descriptor == descriptor {
                    existing.providers.insert(provider);
                } else {
                    let mut providers = existing.providers.clone();
                    providers.insert(provider.clone());

                    self.conflicts.push(
                        TargetIdentityConflict {
                            target: id,
                            providers,
                            descriptor_count: 2,
                        },
                    );

                    self.targets.remove(existing.id());
                }
            }
        }
    }

    /// Returns accepted targets in deterministic identity order.
    #[must_use]
    pub fn targets(
        &self,
    ) -> impl Iterator<Item = &DiscoveredTarget> {
        self.targets.values()
    }

    /// Returns a target by stable identity.
    #[must_use]
    pub fn target(
        &self,
        id: &TargetId,
    ) -> Option<&DiscoveredTarget> {
        self.targets.get(id)
    }

    /// Returns identity conflicts.
    #[must_use]
    pub fn conflicts(&self) -> &[TargetIdentityConflict] {
        &self.conflicts
    }

    /// Returns provider failures.
    #[must_use]
    pub fn failures(&self) -> &[ProviderFailure] {
        &self.failures
    }

    /// Returns rejected descriptors.
    #[must_use]
    pub fn rejected_descriptors(&self) -> &[RejectedDescriptor] {
        &self.rejected_descriptors
    }

    /// Returns the number of accepted targets.
    #[must_use]
    pub fn len(&self) -> usize {
        self.targets.len()
    }

    /// Returns whether no targets were accepted.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }

    /// Returns the number of successful providers.
    #[must_use]
    pub fn successful_provider_count(&self) -> usize {
        self.successful_providers
    }

    /// Returns whether any provider failed.
    #[must_use]
    pub fn has_provider_failures(&self) -> bool {
        !self.failures.is_empty()
    }

    /// Returns whether identity conflicts exist.
    #[must_use]
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }

    /// Returns whether discovery was entirely successful.
    ///
    /// A successful empty discovery is possible.
    ///
    /// This does not establish hardware availability or feasibility.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.failures.is_empty()
            && self.conflicts.is_empty()
            && self.rejected_descriptors.is_empty()
    }
}

// ============================================================================
// DISCOVERY ERROR
// ============================================================================

/// Errors produced by target discovery.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DiscoveryError {
    /// Provider identity is empty.
    EmptyProviderId,

    /// Provider identity is invalid.
    InvalidProviderId {
        /// Invalid value.
        value: String,

        /// Validation explanation.
        reason: String,
    },

    /// Provider is already registered.
    DuplicateProvider {
        /// Duplicate provider.
        provider: DiscoveryProviderId,
    },

    /// One or more providers failed under a strict policy.
    ProviderFailures {
        /// All reported provider failures.
        failures: Vec<ProviderFailure>,
    },

    /// Conflicting target descriptors were discovered.
    TargetIdentityConflicts {
        /// All discovered conflicts.
        conflicts: Vec<TargetIdentityConflict>,
    },

    /// Target descriptor construction failed.
    InvalidTarget {
        /// Target identity.
        target: TargetId,

        /// Original error.
        message: String,
    },
}

impl fmt::Display for DiscoveryError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyProviderId => {
                write!(formatter, "provider identifier cannot be empty")
            }

            Self::InvalidProviderId { value, reason } => {
                write!(
                    formatter,
                    "invalid discovery provider identifier `{value}`: {reason}"
                )
            }

            Self::DuplicateProvider { provider } => {
                write!(
                    formatter,
                    "discovery provider `{provider}` is already registered"
                )
            }

            Self::ProviderFailures { failures } => {
                write!(
                    formatter,
                    "{} target discovery provider(s) failed",
                    failures.len()
                )
            }

            Self::TargetIdentityConflicts { conflicts } => {
                write!(
                    formatter,
                    "{} target identity conflict(s) detected",
                    conflicts.len()
                )
            }

            Self::InvalidTarget { target, message } => {
                write!(
                    formatter,
                    "invalid target `{target}`: {message}"
                )
            }
        }
    }
}

impl Error for DiscoveryError {}

// ============================================================================
// DISCOVERY SERVICE
// ============================================================================

/// Stateless target discovery service.
///
/// The registry is supplied explicitly so that discovery remains
/// independently testable and session-specific.
pub struct TargetDiscovery;

impl TargetDiscovery {
    /// Returns the stable module identity.
    #[must_use]
    pub const fn module_name() -> &'static str {
        TARGET_DISCOVERY_MODULE_NAME
    }

    /// Returns the public API version.
    #[must_use]
    pub const fn api_version() -> u16 {
        TARGET_DISCOVERY_API_VERSION
    }

    /// Executes target discovery.
    pub fn discover(
        registry: &TargetDiscoveryRegistry,
        request: &DiscoveryRequest,
        policy: DiscoveryPolicy,
    ) -> Result<DiscoveryReport, DiscoveryError> {
        registry.discover(request, policy)
    }
}

// ============================================================================
// TEST PROVIDERS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::configuration::target::{
        TargetDescriptor,
        TargetId,
        TargetRole,
    };

    struct StaticProvider {
        id: DiscoveryProviderId,
        descriptors: Vec<TargetDescriptor>,
        should_fail: bool,
    }

    impl StaticProvider {
        fn new(
            id: &str,
            descriptors: Vec<TargetDescriptor>,
        ) -> Self {
            Self {
                id: DiscoveryProviderId::new(id).unwrap(),
                descriptors,
                should_fail: false,
            }
        }

        fn failing(id: &str) -> Self {
            Self {
                id: DiscoveryProviderId::new(id).unwrap(),
                descriptors: Vec::new(),
                should_fail: true,
            }
        }
    }

    impl TargetDiscoveryProvider for StaticProvider {
        fn id(&self) -> &DiscoveryProviderId {
            &self.id
        }

        fn discover(
            &self,
            _request: &DiscoveryRequest,
        ) -> Result<Vec<TargetDescriptor>, ProviderFailure> {
            if self.should_fail {
                return Err(ProviderFailure::new(
                    self.id.clone(),
                    "test provider failure",
                ));
            }

            Ok(self.descriptors.clone())
        }
    }

    fn descriptor(
        id: &str,
        name: &str,
    ) -> TargetDescriptor {
        TargetDescriptor::builder(
            TargetId::new(id).unwrap(),
            name,
        )
        .role(TargetRole::new("execution").unwrap())
        .build()
        .unwrap()
    }

    fn registry_with(
        providers: Vec<StaticProvider>,
    ) -> TargetDiscoveryRegistry {
        let mut registry = TargetDiscoveryRegistry::new();

        for provider in providers {
            registry.register(provider).unwrap();
        }

        registry
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            TargetDiscovery::module_name(),
            TARGET_DISCOVERY_MODULE_NAME
        );

        assert_ne!(
            TargetDiscovery::api_version(),
            0
        );
    }

    #[test]
    fn empty_provider_identifier_is_rejected() {
        assert!(
            DiscoveryProviderId::new(" ").is_err()
        );
    }

    #[test]
    fn provider_registry_rejects_duplicate_ids() {
        let mut registry = TargetDiscoveryRegistry::new();

        registry.register(
            StaticProvider::new("test.provider", Vec::new())
        ).unwrap();

        let result = registry.register(
            StaticProvider::new("test.provider", Vec::new())
        );

        assert!(matches!(
            result,
            Err(DiscoveryError::DuplicateProvider { .. })
        ));
    }

    #[test]
    fn provider_can_be_unregistered() {
        let mut registry = TargetDiscoveryRegistry::new();

        let provider = StaticProvider::new(
            "test.provider",
            Vec::new(),
        );

        let id = provider.id.clone();

        registry.register(provider).unwrap();

        assert!(registry.unregister(&id));
        assert!(!registry.contains(&id));
    }

    #[test]
    fn discovers_registered_targets() {
        let registry = registry_with(vec![
            StaticProvider::new(
                "test.cpu",
                vec![descriptor("cpu:test", "Test CPU")],
            ),
        ]);

        let report = TargetDiscovery::discover(
            &registry,
            &DiscoveryRequest::all_registered(),
            DiscoveryPolicy::best_effort(),
        ).unwrap();

        assert_eq!(report.len(), 1);
        assert!(!report.has_provider_failures());
        assert!(!report.has_conflicts());
    }

    #[test]
    fn target_order_is_deterministic() {
        let registry = registry_with(vec![
            StaticProvider::new(
                "test.provider",
                vec![
                    descriptor("target:z", "Z"),
                    descriptor("target:a", "A"),
                    descriptor("target:m", "M"),
                ],
            ),
        ]);

        let report = registry.discover(
            &DiscoveryRequest::all_registered(),
            DiscoveryPolicy::best_effort(),
        ).unwrap();

        let ids: Vec<_> = report.targets()
            .map(|target| target.id().as_str())
            .collect();

        assert_eq!(
            ids,
            vec!["target:a", "target:m", "target:z"]
        );
    }

    #[test]
    fn identical_descriptors_from_multiple_providers_are_merged() {
        let target = descriptor(
            "shared:target",
            "Shared Target",
        );

        let registry = registry_with(vec![
            StaticProvider::new(
                "provider.a",
                vec![target.clone()],
            ),
            StaticProvider::new(
                "provider.b",
                vec![target],
            ),
        ]);

        let report = registry.discover(
            &DiscoveryRequest::all_registered(),
            DiscoveryPolicy::strict(),
        ).unwrap();

        assert_eq!(report.len(), 1);

        let discovered = report.targets().next().unwrap();

        assert_eq!(discovered.provider_count(), 2);
    }

    #[test]
    fn conflicting_descriptors_are_not_silently_selected() {
        let registry = registry_with(vec![
            StaticProvider::new(
                "provider.a",
                vec![descriptor(
                    "shared:target",
                    "First Description",
                )],
            ),
            StaticProvider::new(
                "provider.b",
                vec![descriptor(
                    "shared:target",
                    "Different Description",
                )],
            ),
        ]);

        let result = registry.discover(
            &DiscoveryRequest::all_registered(),
            DiscoveryPolicy::strict(),
        );

        assert!(matches!(
            result,
            Err(DiscoveryError::TargetIdentityConflicts { .. })
        ));
    }

    #[test]
    fn best_effort_preserves_provider_failures() {
        let registry = registry_with(vec![
            StaticProvider::new(
                "provider.good",
                vec![descriptor("target:good", "Good")],
            ),
            StaticProvider::failing("provider.bad"),
        ]);

        let report = registry.discover(
            &DiscoveryRequest::all_registered(),
            DiscoveryPolicy::best_effort(),
        ).unwrap();

        assert_eq!(report.len(), 1);
        assert!(report.has_provider_failures());
        assert!(!report.is_complete());
    }

    #[test]
    fn strict_policy_rejects_provider_failures() {
        let registry = registry_with(vec![
            StaticProvider::failing("provider.bad"),
        ]);

        let result = registry.discover(
            &DiscoveryRequest::all_registered(),
            DiscoveryPolicy::strict(),
        );

        assert!(matches!(
            result,
            Err(DiscoveryError::ProviderFailures { .. })
        ));
    }

    #[test]
    fn requested_target_filter_is_respected() {
        let registry = registry_with(vec![
            StaticProvider::new(
                "provider.test",
                vec![
                    descriptor("target:a", "A"),
                    descriptor("target:b", "B"),
                ],
            ),
        ]);

        let request = DiscoveryRequest::for_targets([
            TargetId::new("target:b").unwrap(),
        ]);

        let report = registry.discover(
            &request,
            DiscoveryPolicy::best_effort(),
        ).unwrap();

        assert_eq!(report.len(), 1);

        assert_eq!(
            report.targets().next().unwrap().id().as_str(),
            "target:b"
        );

        assert_eq!(report.rejected_descriptors().len(), 1);
    }

    #[test]
    fn empty_registry_produces_empty_successful_report() {
        let registry = TargetDiscoveryRegistry::new();

        let report = registry.discover(
            &DiscoveryRequest::all_registered(),
            DiscoveryPolicy::strict(),
        ).unwrap();

        assert!(report.is_empty());
        assert!(report.is_complete());
    }

    #[test]
    fn metadata_only_targets_are_excluded_by_default() {
        let metadata_target = TargetDescriptor::builder(
            TargetId::new("format:portable-ir").unwrap(),
            "Portable IR",
        )
        .role(TargetRole::new("intermediate-representation").unwrap())
        .build()
        .unwrap();

        let registry = registry_with(vec![
            StaticProvider::new(
                "provider.ir",
                vec![metadata_target],
            ),
        ]);

        let report = registry.discover(
            &DiscoveryRequest::all_registered(),
            DiscoveryPolicy::best_effort(),
        ).unwrap();

        assert!(report.is_empty());
        assert_eq!(report.rejected_descriptors().len(), 1);
    }

    #[test]
    fn metadata_only_targets_can_be_requested_explicitly() {
        let metadata_target = TargetDescriptor::builder(
            TargetId::new("format:portable-ir").unwrap(),
            "Portable IR",
        )
        .role(TargetRole::new("intermediate-representation").unwrap())
        .build()
        .unwrap();

        let registry = registry_with(vec![
            StaticProvider::new(
                "provider.ir",
                vec![metadata_target],
            ),
        ]);

        let report = registry.discover(
            &DiscoveryRequest::all_registered()
                .include_metadata_only(true),
            DiscoveryPolicy::best_effort(),
        ).unwrap();

        assert_eq!(report.len(), 1);
    }

    #[test]
    fn discovery_does_not_claim_capability_compatibility() {
        let registry = registry_with(vec![
            StaticProvider::new(
                "provider.test",
                vec![descriptor("target:test", "Test")],
            ),
        ]);

        let report = registry.discover(
            &DiscoveryRequest::all_registered(),
            DiscoveryPolicy::best_effort(),
        ).unwrap();

        let discovered = report.targets().next().unwrap();

        // Discovery exposes a descriptor, not a compatibility verdict.
        assert_eq!(
            discovered.id().as_str(),
            "target:test"
        );
    }
}
