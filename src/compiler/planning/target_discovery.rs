
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
//! # Purpose
//!
//! Discover and aggregate registered target descriptors.
//!
//! Discovery establishes that registered providers have reported
//! target descriptions. It does not establish physical availability,
//! capability compatibility, resource feasibility, authorization,
//! backend realizability, or execution availability.
//!
//! # Architectural authorities
//!
//! Target identity and descriptors:
//!     crate::compiler::configuration::target
//!
//! Capability requirements and evidence:
//!     crate::frontend::semantic
//!
//! Capability matching:
//!     crate::compiler::planning::capability_matching
//!
//! Resource feasibility:
//!     crate::compiler::planning::resource_planning
//!
//! Backend registration:
//!     crate::backend::registry
//!
//! Backend compatibility:
//!     crate::backend::compatibility
//!
//! Physical allocation and execution:
//!     crate::runtime
//!
//! # Ownership
//!
//! This module owns:
//! - Discovery provider identities.
//! - Discovery requests and policies.
//! - Instance-owned discovery provider registration.
//! - Provider invocation.
//! - Descriptor validation coordination.
//! - Deterministic descriptor aggregation.
//! - Duplicate identity detection.
//! - Identity conflict reporting.
//! - Provider provenance.
//! - Partial discovery reporting.
//!
//! This module does not own:
//! - Canonical target descriptor definitions.
//! - Semantic capability definitions.
//! - Semantic resource requirements.
//! - Backend registration.
//! - Backend implementations.
//! - Physical resource inventory.
//! - Device allocation.
//! - Physical scheduling.
//! - Physical routing.
//! - Runtime execution.
//! - Source-language grammar.
//! - Canonical IR.
//!
//! # POCO-REAF
//!
//! No universal hardware capacity limits are imposed.
//!
//! The number of providers and discovered targets is determined
//! by actual registrations, provider results and available resources.
//!
//! No finite Rust process can promise literal infinity.
//!
//! Operational limits must be explicit, configurable and owned
//! by the relevant orchestration or configuration subsystem.
//!
//! # Determinism
//!
//! Provider identities and target identities use ordered collections.
//! Provider failures and conflicts are reported deterministically.
//! Conflicting identities are never silently selected.
//!
//! # Integration
//!
//! Downstream consumers:
//! - capability_matching.rs
//! - resource_planning.rs
//! - feasibility.rs
//! - partitioning.rs
//! - routing.rs
//! - scheduling.rs
//!
//! Backend discovery adapts the canonical backend registry to this
//! subsystem. It must not introduce another target registry.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::compiler::configuration::target::{
    TargetDescriptor,
    TargetId,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable identity of the target discovery subsystem.
pub const TARGET_DISCOVERY_MODULE_NAME: &str =
    "zamani.compiler.planning.target_discovery";

/// Target discovery API version.
pub const TARGET_DISCOVERY_API_VERSION: u16 = 2;

// ============================================================================
// PROVIDER IDENTITY
// ============================================================================

/// Stable identity of a target discovery provider.
///
/// This is an open-world identifier.
///
/// It is not a hardware classification or backend identity.
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

        if value.trim() != value {
            return Err(DiscoveryError::InvalidProviderId {
                value,
                reason: "surrounding whitespace is forbidden".to_owned(),
            });
        }

        if value.chars().any(char::is_control) {
            return Err(DiscoveryError::InvalidProviderId {
                value,
                reason: "control characters are forbidden".to_owned(),
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
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

/// Controls the scope of target discovery.
///
/// An empty target set means unrestricted identity discovery.
///
/// It does not imply that every possible target exists.
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

    /// Creates a request restricted to specified target identities.
    pub fn for_targets(
        targets: impl IntoIterator<Item = TargetId>,
    ) -> Self {
        Self {
            requested_targets: targets.into_iter().collect(),
            include_metadata_only: true,
        }
    }

    /// Restricts the request to specified target identities.
    pub fn with_targets(
        mut self,
        targets: impl IntoIterator<Item = TargetId>,
    ) -> Self {
        self.requested_targets = targets.into_iter().collect();
        self
    }

    /// Controls whether non-execution target roles are included.
    ///
    /// The canonical target descriptor currently exposes open-world
    /// roles rather than a dedicated metadata-only classification.
    ///
    /// For compatibility, false retains the existing execution-role
    /// filtering behavior. New integrations should avoid using role
    /// strings as a substitute for explicit target classification.
    pub fn include_metadata_only(
        mut self,
        include: bool,
    ) -> Self {
        self.include_metadata_only = include;
        self
    }

    /// Returns requested target identities.
    #[must_use]
    pub fn requested_targets(&self) -> &BTreeSet<TargetId> {
        &self.requested_targets
    }

    /// Returns whether non-execution roles are included.
    #[must_use]
    pub fn includes_metadata_only(&self) -> bool {
        self.include_metadata_only
    }

    /// Returns whether discovery is identity-restricted.
    #[must_use]
    pub fn is_restricted(&self) -> bool {
        !self.requested_targets.is_empty()
    }

    /// Determines whether a target identity was requested.
    #[must_use]
    pub fn accepts_identity(
        &self,
        target: &TargetId,
    ) -> bool {
        self.requested_targets.is_empty()
            || self.requested_targets.contains(target)
    }
}

// ============================================================================
// DISCOVERY POLICY
// ============================================================================

/// Controls discovery failure handling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiscoveryPolicy {
    require_all_providers: bool,
    reject_duplicate_target_identities: bool,
}

impl DiscoveryPolicy {
    /// Creates a best-effort discovery policy.
    ///
    /// Successful providers may contribute results even when
    /// another provider fails.
    pub const fn best_effort() -> Self {
        Self {
            require_all_providers: false,
            reject_duplicate_target_identities: true,
        }
    }

    /// Creates a strict discovery policy.
    ///
    /// Every provider must succeed and target identities must
    /// be unambiguous.
    pub const fn strict() -> Self {
        Self {
            require_all_providers: true,
            reject_duplicate_target_identities: true,
        }
    }

    /// Controls whether every provider must succeed.
    pub const fn require_all_providers(
        mut self,
        required: bool,
    ) -> Self {
        self.require_all_providers = required;
        self
    }

    /// Controls whether identity conflicts invalidate discovery.
    ///
    /// Conflicts are always recorded, even when rejection is disabled.
    ///
    /// Disabling rejection never allows an ambiguous identity
    /// to become an accepted target.
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

    /// Returns whether conflicts invalidate discovery.
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
// PROVIDER FAILURE
// ============================================================================

/// Failure reported by a discovery provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderFailure {
    /// Registered provider identity.
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
/// Providers return canonical target descriptors from their
/// own authoritative information sources.
///
/// Providers must not allocate hardware or execute programs.
///
/// They must not claim capability compatibility or resource
/// feasibility merely because a target descriptor exists.
///
/// Implementations must be thread-safe.
pub trait TargetDiscoveryProvider: Send + Sync {
    /// Returns the stable provider identity.
    fn id(&self) -> &DiscoveryProviderId;

    /// Discovers target descriptors.
    ///
    /// Returned descriptors are validated and filtered by the
    /// discovery registry.
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
/// This registry is deliberately separate from the canonical
/// backend registry.
///
/// Registration order does not determine discovery result order.
#[derive(Default)]
pub struct TargetDiscoveryRegistry {
    providers: BTreeMap<
        DiscoveryProviderId,
        Arc<dyn TargetDiscoveryProvider>,
    >,
}

impl TargetDiscoveryRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a provider.
    ///
    /// Existing registrations are never silently overwritten.
    pub fn register<P>(
        &mut self,
        provider: P,
    ) -> Result<(), DiscoveryError>
    where
        P: TargetDiscoveryProvider + 'static,
    {
        self.register_shared(Arc::new(provider))
    }

    /// Registers a shared provider.
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

    /// Removes a registered provider.
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

    /// Returns provider identities in deterministic order.
    pub fn provider_ids(
        &self,
    ) -> impl Iterator<Item = &DiscoveryProviderId> {
        self.providers.keys()
    }

    /// Performs discovery through registered providers.
    ///
    /// Every provider is invoked exactly once during this operation.
    ///
    /// Provider execution is sequential. Parallel execution, cancellation
    /// and time budgets belong to the orchestration layer.
    pub fn discover(
        &self,
        request: &DiscoveryRequest,
        policy: DiscoveryPolicy,
    ) -> Result<DiscoveryReport, DiscoveryError> {
        let mut report = DiscoveryReport::new();

        // Collect all valid observations before accepting any target.
        //
        // This prevents a conflicting identity from being accidentally
        // reintroduced by a later provider.
        let mut observations: BTreeMap<
            TargetId,
            Vec<DescriptorObservation>,
        > = BTreeMap::new();

        for (registered_id, provider) in &self.providers {
            match provider.discover(request) {
                Ok(descriptors) => {
                    report.successful_providers += 1;

                    for descriptor in descriptors {
                        let target_id = descriptor.id().clone();

                        if !request.accepts_identity(&target_id) {
                            report.rejected_descriptors.push(
                                RejectedDescriptor {
                                    provider: registered_id.clone(),
                                    target: target_id,
                                    reason: RejectionReason::NotRequested,
                                },
                            );

                            continue;
                        }

                        if let Err(error) = descriptor.validate() {
                            report.rejected_descriptors.push(
                                RejectedDescriptor {
                                    provider: registered_id.clone(),
                                    target: target_id,
                                    reason: RejectionReason::InvalidDescriptor(
                                        error.to_string(),
                                    ),
                                },
                            );

                            continue;
                        }

                        // Preserve the existing public filtering behavior.
                        //
                        // The role is supplied by the canonical target
                        // descriptor; no hardware taxonomy is introduced.
                        if !request.includes_metadata_only()
                            && descriptor.role().as_str() != "execution"
                        {
                            report.rejected_descriptors.push(
                                RejectedDescriptor {
                                    provider: registered_id.clone(),
                                    target: target_id,
                                    reason: RejectionReason::MetadataOnly,
                                },
                            );

                            continue;
                        }

                        let variants = observations
                            .entry(target_id)
                            .or_default();

                        if let Some(existing) = variants
                            .iter_mut()
                            .find(|entry| entry.descriptor == descriptor)
                        {
                            existing.providers.insert(registered_id.clone());
                        } else {
                            let mut providers = BTreeSet::new();
                            providers.insert(registered_id.clone());

                            variants.push(DescriptorObservation {
                                descriptor,
                                providers,
                            });
                        }
                    }
                }

                Err(failure) => {
                    // The registry, not an untrusted provider response,
                    // is authoritative for provider provenance.
                    report.failures.push(
                        ProviderFailure::new(
                            registered_id.clone(),
                            failure.message,
                        ),
                    );
                }
            }
        }

        // Finalize identities only after every provider has been processed.
        for (target_id, variants) in observations {
            if variants.len() == 1 {
                let observation = variants.into_iter().next()
                    .expect("one observation was checked");

                report.targets.insert(
                    target_id,
                    DiscoveredTarget {
                        descriptor: observation.descriptor,
                        providers: observation.providers,
                    },
                );
            } else {
                let mut providers = BTreeSet::new();

                for observation in &variants {
                    providers.extend(
                        observation.providers.iter().cloned(),
                    );
                }

                report.conflicts.push(TargetIdentityConflict {
                    target: target_id,
                    providers,
                    descriptor_count: variants.len(),
                });
            }
        }

        report.failures.sort_by(|left, right| {
            left.provider.cmp(&right.provider)
                .then_with(|| left.message.cmp(&right.message))
        });

        report.rejected_descriptors.sort_by(|left, right| {
            left.target.cmp(&right.target)
                .then_with(|| left.provider.cmp(&right.provider))
        });

        // Strict mode must preserve the complete report if both provider
        // failures and target conflicts exist.
        let has_provider_failure =
            policy.requires_all_providers()
                && report.has_provider_failures();

        let has_conflict =
            policy.rejects_duplicate_target_identities()
                && report.has_conflicts();

        if has_provider_failure || has_conflict {
            if has_provider_failure && has_conflict {
                return Err(DiscoveryError::IncompleteReport {
                    report: Box::new(report),
                });
            }

            if has_provider_failure {
                return Err(DiscoveryError::ProviderFailures {
                    failures: report.failures.clone(),
                });
            }

            return Err(DiscoveryError::TargetIdentityConflicts {
                conflicts: report.conflicts.clone(),
            });
        }

        Ok(report)
    }
}

// ============================================================================
// INTERNAL DESCRIPTOR OBSERVATION
// ============================================================================

/// One distinct descriptor and all providers reporting it.
#[derive(Clone, Debug, Eq, PartialEq)]
struct DescriptorObservation {
    descriptor: TargetDescriptor,
    providers: BTreeSet<DiscoveryProviderId>,
}

// ============================================================================
// DISCOVERED TARGET
// ============================================================================

/// An unambiguous target descriptor and its discovery provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveredTarget {
    descriptor: TargetDescriptor,
    providers: BTreeSet<DiscoveryProviderId>,
}

impl DiscoveredTarget {
    /// Returns the canonical target descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &TargetDescriptor {
        &self.descriptor
    }

    /// Returns the target identity.
    #[must_use]
    pub fn id(&self) -> &TargetId {
        self.descriptor.id()
    }

    /// Returns providers reporting this identical descriptor.
    #[must_use]
    pub fn providers(&self) -> &BTreeSet<DiscoveryProviderId> {
        &self.providers
    }

    /// Returns the number of reporting providers.
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
/// Conflicting identities are never admitted as discovered targets.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetIdentityConflict {
    /// Conflicting target identity.
    pub target: TargetId,

    /// Every provider that reported one of the conflicting descriptors.
    pub providers: BTreeSet<DiscoveryProviderId>,

    /// Number of distinct descriptors reported.
    pub descriptor_count: usize,
}

// ============================================================================
// REJECTED DESCRIPTORS
// ============================================================================

/// Reason a descriptor was rejected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RejectionReason {
    /// Target identity was not requested.
    NotRequested,

    /// Target has a non-execution role and metadata-only targets
    /// were not requested.
    MetadataOnly,

    /// Descriptor validation failed.
    InvalidDescriptor(String),
}

/// A rejected provider-reported descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RejectedDescriptor {
    /// Provider that supplied the descriptor.
    pub provider: DiscoveryProviderId,

    /// Target identity.
    pub target: TargetId,

    /// Rejection reason.
    pub reason: RejectionReason,
}

// ============================================================================
// DISCOVERY REPORT
// ============================================================================

/// Complete result of target discovery.
///
/// An empty report does not prove that no hardware exists.
///
/// It means registered providers supplied no accepted target descriptors.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DiscoveryReport {
    targets: BTreeMap<TargetId, DiscoveredTarget>,
    conflicts: Vec<TargetIdentityConflict>,
    failures: Vec<ProviderFailure>,
    rejected_descriptors: Vec<RejectedDescriptor>,
    successful_providers: usize,
}

impl DiscoveryReport {
    /// Creates an empty report.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns accepted targets in deterministic identity order.
    pub fn targets(
        &self,
    ) -> impl Iterator<Item = &DiscoveredTarget> {
        self.targets.values()
    }

    /// Returns an unambiguous target by identity.
    #[must_use]
    pub fn target(
        &self,
        id: &TargetId,
    ) -> Option<&DiscoveredTarget> {
        self.targets.get(id)
    }

    /// Returns target identity conflicts.
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

    /// Returns whether a provider failed.
    #[must_use]
    pub fn has_provider_failures(&self) -> bool {
        !self.failures.is_empty()
    }

    /// Returns whether identity conflicts exist.
    #[must_use]
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }

    /// Returns whether any descriptor was rejected.
    #[must_use]
    pub fn has_rejections(&self) -> bool {
        !self.rejected_descriptors.is_empty()
    }

    /// Returns whether discovery completed without reported problems.
    ///
    /// This does not establish capability compatibility,
    /// resource feasibility or physical availability.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.failures.is_empty()
            && self.conflicts.is_empty()
            && self.rejected_descriptors.is_empty()
    }
}

// ============================================================================
// DISCOVERY ERRORS
// ============================================================================

/// Errors produced by target discovery.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DiscoveryError {
    /// Provider identifier is empty.
    EmptyProviderId,

    /// Provider identifier is invalid.
    InvalidProviderId {
        /// Invalid value.
        value: String,

        /// Validation explanation.
        reason: String,
    },

    /// Provider identity is already registered.
    DuplicateProvider {
        /// Existing identity.
        provider: DiscoveryProviderId,
    },

    /// Providers failed under strict discovery.
    ProviderFailures {
        /// Reported provider failures.
        failures: Vec<ProviderFailure>,
    },

    /// Conflicting target identities were discovered.
    TargetIdentityConflicts {
        /// All conflicts.
        conflicts: Vec<TargetIdentityConflict>,
    },

    /// Both provider failures and target conflicts occurred.
    ///
    /// This variant preserves the complete discovery report.
    IncompleteReport {
        /// Complete report.
        report: Box<DiscoveryReport>,
    },

    /// An invalid target was encountered during construction.
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
                write!(
                    formatter,
                    "provider identifier cannot be empty"
                )
            }

            Self::InvalidProviderId { value, reason } => {
                write!(
                    formatter,
                    "invalid provider identifier `{value}`: {reason}"
                )
            }

            Self::DuplicateProvider { provider } => {
                write!(
                    formatter,
                    "provider `{provider}` is already registered"
                )
            }

            Self::ProviderFailures { failures } => {
                write!(
                    formatter,
                    "{} discovery provider(s) failed",
                    failures.len()
                )
            }

            Self::TargetIdentityConflicts { conflicts } => {
                write!(
                    formatter,
                    "{} target identity conflict(s)",
                    conflicts.len()
                )
            }

            Self::IncompleteReport { report } => {
                write!(
                    formatter,
                    "discovery incomplete: {} provider failure(s), \
                     {} target conflict(s)",
                    report.failures.len(),
                    report.conflicts.len()
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
pub struct TargetDiscovery;

impl TargetDiscovery {
    /// Returns the subsystem identity.
    #[must_use]
    pub const fn module_name() -> &'static str {
        TARGET_DISCOVERY_MODULE_NAME
    }

    /// Returns the subsystem API version.
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
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct StaticProvider {
        id: DiscoveryProviderId,
        descriptors: Vec<TargetDescriptor>,
        failure: Option<String>,
    }

    impl StaticProvider {
        fn new(
            id: &str,
            descriptors: Vec<TargetDescriptor>,
        ) -> Self {
            Self {
                id: DiscoveryProviderId::new(id).unwrap(),
                descriptors,
                failure: None,
            }
        }

        fn failing(id: &str) -> Self {
            Self {
                id: DiscoveryProviderId::new(id).unwrap(),
                descriptors: Vec::new(),
                failure: Some("test provider failure".to_owned()),
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
            if let Some(message) = &self.failure {
                return Err(ProviderFailure::new(
                    self.id.clone(),
                    message.clone(),
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

        assert_eq!(
            TargetDiscovery::api_version(),
            TARGET_DISCOVERY_API_VERSION
        );
    }

    #[test]
    fn empty_provider_identifier_is_rejected() {
        assert!(
            DiscoveryProviderId::new(" ").is_err()
        );
    }

    #[test]
    fn whitespace_provider_identifier_is_rejected() {
        assert!(
            DiscoveryProviderId::new(" provider ").is_err()
        );
    }

    #[test]
    fn duplicate_provider_registration_is_rejected() {
        let mut registry = TargetDiscoveryRegistry::new();

        registry.register(
            StaticProvider::new("provider.a", Vec::new())
        ).unwrap();

        assert!(matches!(
            registry.register(
                StaticProvider::new("provider.a", Vec::new())
            ),
            Err(DiscoveryError::DuplicateProvider { .. })
        ));
    }

    #[test]
    fn provider_can_be_unregistered() {
        let mut registry = TargetDiscoveryRegistry::new();

        let provider = StaticProvider::new(
            "provider.a",
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
                "provider.cpu",
                vec![descriptor("cpu:test", "Test CPU")],
            ),
        ]);

        let report = registry.discover(
            &DiscoveryRequest::all_registered(),
            DiscoveryPolicy::best_effort(),
        ).unwrap();

        assert_eq!(report.len(), 1);
        assert!(report.is_complete());
    }

    #[test]
    fn target_order_is_deterministic() {
        let registry = registry_with(vec![
            StaticProvider::new(
                "provider.test",
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
    fn identical_descriptors_merge_provider_provenance() {
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

        let discovered = report.target(
            &TargetId::new("shared:target").unwrap()
        ).unwrap();

        assert_eq!(discovered.provider_count(), 2);
    }

    #[test]
    fn conflicting_descriptors_are_rejected() {
        let registry = registry_with(vec![
            StaticProvider::new(
                "provider.a",
                vec![descriptor(
                    "shared:target",
                    "First",
                )],
            ),
            StaticProvider::new(
                "provider.b",
                vec![descriptor(
                    "shared:target",
                    "Second",
                )],
            ),
        ]);

        assert!(matches!(
            registry.discover(
                &DiscoveryRequest::all_registered(),
                DiscoveryPolicy::strict(),
            ),
            Err(DiscoveryError::TargetIdentityConflicts { .. })
        ));
    }

    #[test]
    fn third_provider_cannot_reintroduce_conflicting_identity() {
        let registry = registry_with(vec![
            StaticProvider::new(
                "provider.a",
                vec![descriptor("target:x", "First")],
            ),
            StaticProvider::new(
                "provider.b",
                vec![descriptor("target:x", "Second")],
            ),
            StaticProvider::new(
                "provider.c",
                vec![descriptor("target:x", "Third")],
            ),
        ]);

        let report = registry.discover(
            &DiscoveryRequest::all_registered(),
            DiscoveryPolicy::best_effort()
                .reject_duplicate_target_identities(false),
        ).unwrap();

        assert_eq!(report.len(), 0);
        assert_eq!(report.conflicts().len(), 1);
        assert_eq!(
            report.conflicts()[0].descriptor_count,
            3
        );
        assert_eq!(
            report.conflicts()[0].providers.len(),
            3
        );
    }

    #[test]
    fn repeated_identical_descriptor_from_same_provider_is_not_a_conflict() {
        let target = descriptor("target:x", "Target X");

        let registry = registry_with(vec![
            StaticProvider::new(
                "provider.a",
                vec![target.clone(), target],
            ),
        ]);

        let report = registry.discover(
            &DiscoveryRequest::all_registered(),
            DiscoveryPolicy::strict(),
        ).unwrap();

        assert_eq!(report.len(), 1);
        assert!(report.conflicts().is_empty());
    }

    #[test]
    fn best_effort_preserves_provider_failures() {
        let registry = registry_with(vec![
            StaticProvider::failing("provider.failure"),
            StaticProvider::new(
                "provider.success",
                vec![descriptor("target:x", "X")],
            ),
        ]);

        let report = registry.discover(
            &DiscoveryRequest::all_registered(),
            DiscoveryPolicy::best_effort(),
        ).unwrap();

        assert_eq!(report.len(), 1);
        assert_eq!(report.failures().len(), 1);
        assert!(!report.is_complete());
    }

    #[test]
    fn strict_policy_rejects_provider_failures() {
        let registry = registry_with(vec![
            StaticProvider::failing("provider.failure"),
        ]);

        assert!(matches!(
            registry.discover(
                &DiscoveryRequest::all_registered(),
                DiscoveryPolicy::strict(),
            ),
            Err(DiscoveryError::ProviderFailures { .. })
        ));
    }

    #[test]
    fn strict_policy_preserves_combined_failures_and_conflicts() {
        let registry = registry_with(vec![
            StaticProvider::failing("provider.failure"),
            StaticProvider::new(
                "provider.a",
                vec![descriptor("target:x", "First")],
            ),
            StaticProvider::new(
                "provider.b",
                vec![descriptor("target:x", "Second")],
            ),
        ]);

        let result = registry.discover(
            &DiscoveryRequest::all_registered(),
            DiscoveryPolicy::strict(),
        );

        match result {
            Err(DiscoveryError::IncompleteReport { report }) => {
                assert_eq!(report.failures().len(), 1);
                assert_eq!(report.conflicts().len(), 1);
            }
            _ => panic!("combined discovery failure was not preserved"),
        }
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
            TargetId::new("target:a").unwrap(),
        ]);

        let report = registry.discover(
            &request,
            DiscoveryPolicy::best_effort(),
        ).unwrap();

        assert_eq!(report.len(), 1);
        assert_eq!(report.rejected_descriptors().len(), 1);
    }

    #[test]
    fn empty_registry_is_a_valid_empty_discovery() {
        let registry = TargetDiscoveryRegistry::new();

        let report = registry.discover(
            &DiscoveryRequest::all_registered(),
            DiscoveryPolicy::strict(),
        ).unwrap();

        assert!(report.is_empty());
        assert!(report.is_complete());
    }
}
