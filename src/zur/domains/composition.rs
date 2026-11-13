
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Domain Composition
//!
//! Path: src/zur/domains/composition.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Provides two strictly separated domain-composition integrations:
//!
//! 1. ZUR-to-Hybrid composition integration.
//! 2. Explicit multi-provider composition.
//!
//! Both integrations reuse canonical domain identities, domain-provider
//! contracts, provider registration, compatibility validation, and hybrid
//! composition representations.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Hybrid composition resolution against the ZUR provider registry.
//! - Explicit provider assignments for hybrid domain instances.
//! - Composition-level structural validation.
//! - Composition resolution reports.
//! - Domain-identity adaptation between hybrid and ZUR representations.
//!
//! This module does NOT own:
//!
//! - Domain semantics or AST identity.
//! - Domain descriptors.
//! - Provider identities.
//! - Provider registration or discovery.
//! - Provider lifecycle.
//! - Provider execution.
//! - Provider authentication or authorization.
//! - Capability semantics or evidence authentication.
//! - Resource requirements or resource allocation.
//! - Resource reservations or leases.
//! - Feasibility analysis.
//! - Compiler planning.
//! - Hybrid graph construction.
//! - Hybrid dependencies or synchronization.
//! - Scheduling, routing, or placement.
//! - Classical, quantum, HDL, AI, or tensor IR.
//! - ZUIR.
//! - Hardware abstraction or device drivers.
//!
//! ## STRICT SEPARATION
//!
//! HybridCompositionAdapter resolves existing hybrid composition instances.
//!
//! ProviderComposition resolves explicitly supplied provider assignments.
//!
//! Neither API constructs another hybrid graph or provider registry.
//!
//! ## ARCHITECTURAL INVARIANTS
//!
//! Requirement != Capability.
//! Capability != CapabilityEvidence.
//! CapabilityEvidence != CapabilityAvailability.
//! Preference != Requirement.
//! Constraint != Hint.
//! Compatibility != Feasibility.
//! Feasibility != ResourceReservation.
//! ResourceReservation != Execution.
//! Submission != SuccessfulCompletion.
//!
//! Registration != Initialization.
//! Initialization != Availability.
//! Availability != CapabilityAvailability.
//!
//! A successful composition resolution proves structural provider
//! compatibility only.
//!
//! It does not prove that a provider is currently available or that
//! a computation can execute.
//!
//! ## OPEN-WORLD EXTENSIBILITY
//!
//! No closed computational-domain enum is introduced.
//!
//! Domain identities are obtained from the canonical frontend AST through
//! an explicit resolver.
//!
//! New domains and providers do not require modifications to this module.
//!
//! ## SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Domain instances.
//! - Computational domains.
//! - Providers.
//! - Providers per domain.
//! - Provider assignments.
//! - Domain combinations.
//! - Graph size.
//! - Hardware resources.
//!
//! Collections grow according to available memory and explicit operational
//! policies owned by their respective authorities.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## GRAMMAR BOUNDARY
//!
//! This module consumes validated representations.
//!
//! It does not parse or interpret grammar.
//!
//! Relevant authorities:
//!
//! - grammar/specification/domains.md
//! - grammar/specification/semantic-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/spec/interoperability.md
//! - grammar/spec/hybrid.md
//! - grammar/hybrid/accelerator-interoperability.g4
//!
//! ## INTEGRATION
//!
//! Domain identity:
//!     frontend::ast::node::domains::domain::Domain
//!
//! Domain providers:
//!     zur::domains::provider
//!
//! Provider registry:
//!     zur::domains::registry
//!
//! Compatibility:
//!     zur::domains::compatibility
//!
//! Hybrid composition:
//!     hybrid::domain::composition
//!
//! Hybrid identities:
//!     hybrid::ids
//!
//! Hybrid versions:
//!     hybrid::version
//!
//! Runtime contract versions:
//!     zur::version
//!
//! Errors:
//!     zur::error

use core::fmt;

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use crate::frontend::ast::node::domains::domain::Domain;

use crate::hybrid::contract::DomainInterfaceRef;
use crate::hybrid::domain::composition::DomainComposition;
use crate::hybrid::ids::{
    DomainInstanceId,
    HybridGraphId,
};
use crate::hybrid::version::DomainContractVersion;

use crate::stdlib::provider::ProviderId;

use crate::zur::contracts::provider::ProviderInterfaceId;

use crate::zur::domains::compatibility::{
    check_domain_provider_compatibility,
    DomainCompatibilityReport,
};

use crate::zur::domains::descriptor::DomainDescriptor;

use crate::zur::domains::provider::DomainProvider;

use crate::zur::domains::registry::DomainProviderRegistry;

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::version::ServiceContractVersion;

// ============================================================================
// COMPONENT IDENTIFICATION
// ============================================================================

const COMPONENT: &str = "zur.domains.composition";

// ============================================================================
// DOMAIN IDENTITY RESOLUTION
// ============================================================================

/// Resolves a hybrid domain identity into the canonical AST domain type.
///
/// Hybrid composition and ZUR deliberately retain their respective
/// canonical representations.
///
/// An implementation of this trait performs an explicit integration
/// between those representations.
///
/// It must not invent domain aliases or silently change domain identity.
pub trait HybridDomainIdentityResolver: Send + Sync {

    /// Resolves a hybrid domain identifier.
    ///
    /// Returns the canonical AST identity when the supplied identity
    /// is known to the resolver.
    ///
    /// An unknown domain must produce an explicit error.
    fn resolve_domain(
        &self,
        hybrid_domain: &str,
    ) -> ZurResult<Domain>;
}

/// Explicit mapping between hybrid domain identifiers and canonical
/// frontend AST domain identities.
///
/// This adapter is useful when semantic analysis or another canonical
/// domain authority has already established the mapping.
///
/// No implicit namespace inference is performed.
#[derive(Debug, Clone, Default)]
pub struct DomainIdentityMap {
    mappings: BTreeMap<String, Domain>,
}

impl DomainIdentityMap {

    /// Creates an empty identity mapping.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts an explicit hybrid-to-AST identity mapping.
    ///
    /// Duplicate hybrid identifiers are rejected.
    pub fn insert(
        &mut self,
        hybrid_domain: impl Into<String>,
        canonical_domain: Domain,
    ) -> ZurResult<()> {

        let hybrid_domain = hybrid_domain.into();

        validate_identifier(
            &hybrid_domain,
            "hybrid domain identity",
        )?;

        canonical_domain.validate().map_err(|error| {
            composition_error(
                ZurError::invalid_input(
                    format!(
                        "invalid canonical AST domain identity: {error}"
                    ),
                ),
                "insert_domain_identity",
            )
        })?;

        if self.mappings.contains_key(&hybrid_domain) {
            return Err(
                composition_error(
                    ZurError::invalid_input(
                        format!(
                            "hybrid domain identity {hybrid_domain} \
                             is already mapped"
                        ),
                    ),
                    "insert_domain_identity",
                ),
            );
        }

        self.mappings.insert(
            hybrid_domain,
            canonical_domain,
        );

        Ok(())
    }

    /// Returns the number of explicitly established mappings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.mappings.len()
    }

    /// Returns whether the mapping is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.mappings.is_empty()
    }
}

impl HybridDomainIdentityResolver for DomainIdentityMap {

    fn resolve_domain(
        &self,
        hybrid_domain: &str,
    ) -> ZurResult<Domain> {

        self.mappings
            .get(hybrid_domain)
            .cloned()
            .ok_or_else(|| {
                composition_error(
                    ZurError::compatibility(
                        format!(
                            "hybrid domain {hybrid_domain} \
                             has no canonical AST identity mapping"
                        ),
                    ),
                    "resolve_domain_identity",
                )
            })
    }
}

/// Resolves fully qualified hybrid identifiers into AST identities.
///
/// The final identifier segment becomes the domain name.
/// All preceding segments become its namespace.
///
/// Example:
///
/// `zamani.classical`
///
/// becomes:
///
/// namespace = `zamani`
/// name = `classical`
///
/// This resolver deliberately rejects unqualified identifiers.
///
/// When hybrid domain identities use aliases or other naming
/// conventions, use an explicit DomainIdentityMap instead.
#[derive(Debug, Clone, Copy, Default)]
pub struct QualifiedDomainIdentityResolver;

impl HybridDomainIdentityResolver
    for QualifiedDomainIdentityResolver
{
    fn resolve_domain(
        &self,
        hybrid_domain: &str,
    ) -> ZurResult<Domain> {

        let Some((namespace, name)) =
            hybrid_domain.rsplit_once('.')
        else {
            return Err(
                composition_error(
                    ZurError::invalid_input(
                        format!(
                            "hybrid domain {hybrid_domain} is not \
                             fully qualified; supply an explicit \
                             DomainIdentityMap"
                        ),
                    ),
                    "resolve_domain_identity",
                ),
            );
        };

        Domain::new(namespace, name).map_err(|error| {
            composition_error(
                ZurError::invalid_input(
                    format!(
                        "cannot convert hybrid domain {hybrid_domain} \
                         into canonical AST identity: {error}"
                    ),
                ),
                "resolve_domain_identity",
            )
        })
    }
}

// ============================================================================
// DOMAIN INTERFACE ADAPTATION
// ============================================================================

/// Canonical ZUR interface identity resolved from a hybrid interface.
///
/// This value is an adapter result, not another interface contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedDomainInterface {
    domain: Domain,
    interface: ProviderInterfaceId,
    version: ServiceContractVersion,
}

impl ResolvedDomainInterface {

    /// Returns the canonical AST domain identity.
    #[must_use]
    pub fn domain(&self) -> &Domain {
        &self.domain
    }

    /// Returns the canonical ZUR provider interface identity.
    #[must_use]
    pub fn interface(&self) -> &ProviderInterfaceId {
        &self.interface
    }

    /// Returns the adapted ZUR service-contract version.
    #[must_use]
    pub fn version(&self) -> &ServiceContractVersion {
        &self.version
    }
}

/// Converts a hybrid domain-interface reference into the corresponding
/// ZUR domain/interface identity.
///
/// Domain identity resolution is delegated to the supplied canonical
/// resolver.
///
/// The version conversion preserves all three numeric components.
pub fn resolve_hybrid_interface(
    reference: &DomainInterfaceRef,
    resolver: &dyn HybridDomainIdentityResolver,
) -> ZurResult<ResolvedDomainInterface> {

    reference.validate().map_err(|error| {
        composition_error(
            ZurError::invalid_input(
                format!("invalid hybrid interface reference: {error}"),
            ),
            "resolve_hybrid_interface",
        )
    })?;

    let domain = resolver.resolve_domain(
        reference.domain(),
    )?;

    let interface = ProviderInterfaceId::new(
        reference.interface().to_owned(),
    )?;

    let hybrid_version = reference.version();

    let version = ServiceContractVersion::new(
        hybrid_version.major().to_string(),
        hybrid_version.minor().to_string(),
        hybrid_version.patch().to_string(),
    )
    .map_err(|error| {
        composition_error(
            ZurError::invalid_input(
                format!(
                    "cannot adapt hybrid interface version: {error}"
                ),
            ),
            "resolve_hybrid_interface",
        )
    })?;

    Ok(ResolvedDomainInterface {
        domain,
        interface,
        version,
    })
}

// ============================================================================
// HYBRID COMPOSITION RESOLUTION — OPTION A
// ============================================================================

/// A compatible registered provider candidate for one hybrid instance.
///
/// Compatibility is structural and version-aware.
///
/// This type does not represent provider availability, feasibility,
/// resource reservation, or execution.
#[derive(Clone)]
pub struct HybridProviderCandidate {
    provider_id: ProviderId,
    provider: Arc<dyn DomainProvider>,
    compatibility: DomainCompatibilityReport,
}

impl HybridProviderCandidate {

    /// Returns the canonical provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the canonical provider implementation reference.
    ///
    /// This is a borrowed reference to the existing provider.
    #[must_use]
    pub fn provider(&self) -> &Arc<dyn DomainProvider> {
        &self.provider
    }

    /// Returns the compatibility report.
    #[must_use]
    pub fn compatibility(&self) -> &DomainCompatibilityReport {
        &self.compatibility
    }

    /// Returns whether the provider implements a compatible interface.
    #[must_use]
    pub fn is_compatible(&self) -> bool {
        self.compatibility.is_compatible()
    }
}

impl fmt::Debug for HybridProviderCandidate {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        formatter
            .debug_struct("HybridProviderCandidate")
            .field("provider_id", &self.provider_id)
            .field(
                "compatible",
                &self.compatibility.is_compatible(),
            )
            .finish()
    }
}

/// Resolution result for one hybrid domain instance.
///
/// Empty candidate lists are valid resolution observations:
/// they indicate that no matching registered provider was found.
///
/// They must not be interpreted as successful execution.
#[derive(Debug, Clone)]
pub struct HybridInstanceResolution {
    instance_id: DomainInstanceId,
    interface: ResolvedDomainInterface,
    candidates: Vec<HybridProviderCandidate>,
}

impl HybridInstanceResolution {

    /// Returns the hybrid domain-instance identity.
    #[must_use]
    pub const fn instance_id(&self) -> DomainInstanceId {
        self.instance_id
    }

    /// Returns the adapted domain interface.
    #[must_use]
    pub fn interface(&self) -> &ResolvedDomainInterface {
        &self.interface
    }

    /// Returns registered provider candidates.
    ///
    /// Results follow the registry's canonical provider-identity order.
    #[must_use]
    pub fn candidates(&self) -> &[HybridProviderCandidate] {
        &self.candidates
    }

    /// Returns compatible candidates only.
    #[must_use]
    pub fn compatible_candidates(
        &self,
    ) -> impl Iterator<Item = &HybridProviderCandidate> {
        self.candidates
            .iter()
            .filter(|candidate| candidate.is_compatible())
    }

    /// Returns whether at least one structurally compatible provider
    /// was found.
    #[must_use]
    pub fn has_compatible_provider(&self) -> bool {
        self.candidates
            .iter()
            .any(HybridProviderCandidate::is_compatible)
    }
}

/// Resolution of an existing canonical hybrid domain composition.
///
/// This is an adapter report, not another hybrid composition model.
///
/// The original `DomainComposition` remains owned by
/// `crate::hybrid::domain::composition`.
#[derive(Debug, Clone)]
pub struct HybridCompositionResolution {
    graph_id: HybridGraphId,
    instances: Vec<HybridInstanceResolution>,
}

impl HybridCompositionResolution {

    /// Returns the hybrid graph identity.
    #[must_use]
    pub const fn graph_id(&self) -> HybridGraphId {
        self.graph_id
    }

    /// Returns all resolved domain instances.
    #[must_use]
    pub fn instances(&self) -> &[HybridInstanceResolution] {
        &self.instances
    }

    /// Looks up an instance resolution.
    #[must_use]
    pub fn get(
        &self,
        id: DomainInstanceId,
    ) -> Option<&HybridInstanceResolution> {
        self.instances
            .iter()
            .find(|instance| instance.instance_id == id)
    }

    /// Returns the number of instances without a compatible provider.
    #[must_use]
    pub fn unresolved_instance_count(&self) -> usize {
        self.instances
            .iter()
            .filter(|instance| !instance.has_compatible_provider())
            .count()
    }

    /// Returns whether every hybrid instance has at least one
    /// structurally compatible provider.
    ///
    /// This does not prove execution feasibility or availability.
    #[must_use]
    pub fn has_complete_structural_coverage(&self) -> bool {
        self.unresolved_instance_count() == 0
    }
}

/// Adapter between canonical hybrid domain compositions and the
/// ZUR domain-provider registry.
///
/// The adapter performs lookup and compatibility evaluation only.
///
/// It never selects a provider automatically.
pub struct HybridCompositionAdapter<'a> {
    registry: &'a DomainProviderRegistry,
    resolver: &'a dyn HybridDomainIdentityResolver,
}

impl<'a> HybridCompositionAdapter<'a> {

    /// Creates the adapter using explicit registry and identity authorities.
    #[must_use]
    pub fn new(
        registry: &'a DomainProviderRegistry,
        resolver: &'a dyn HybridDomainIdentityResolver,
    ) -> Self {
        Self {
            registry,
            resolver,
        }
    }

    /// Resolves every interface in an existing hybrid composition.
    ///
    /// Every hybrid instance is preserved, including instances for which
    /// no compatible provider is registered.
    ///
    /// The adapter:
    ///
    /// 1. Validates the canonical hybrid composition.
    /// 2. Resolves domain identity through the supplied resolver.
    /// 3. Resolves the interface identity.
    /// 4. Queries the canonical ZUR provider registry.
    /// 5. Evaluates canonical provider compatibility.
    /// 6. Produces deterministic resolution records.
    ///
    /// No provider is initialized, selected, reserved, or executed.
    pub fn resolve(
        &self,
        composition: &DomainComposition,
    ) -> ZurResult<HybridCompositionResolution> {

        composition.validate().map_err(|error| {
            composition_error(
                ZurError::invalid_input(
                    format!(
                        "hybrid composition failed structural validation: {error}"
                    ),
                ),
                "resolve_hybrid_composition",
            )
        })?;

        let mut resolutions =
            Vec::with_capacity(composition.len());

        for instance in composition.instances() {

            let interface = resolve_hybrid_interface(
                instance.interface(),
                self.resolver,
            )?;

            let providers =
                self.registry.providers_for_domain_interface(
                    interface.domain(),
                    interface.interface(),
                )?;

            let mut candidates =
                Vec::with_capacity(providers.len());

            for provider in providers {

                let descriptor =
                    find_matching_descriptor(
                        provider.as_ref(),
                        interface.domain(),
                        interface.interface(),
                    )?;

                let report =
                    check_domain_provider_compatibility(
                        provider.descriptor(),
                        provider.execution_contract(),
                        descriptor,
                    )?;

                candidates.push(HybridProviderCandidate {
                    provider_id: provider.descriptor().id().clone(),
                    provider,
                    compatibility: report,
                });
            }

            resolutions.push(HybridInstanceResolution {
                instance_id: instance.id(),
                interface,
                candidates,
            });
        }

        Ok(HybridCompositionResolution {
            graph_id: composition.graph_id(),
            instances: resolutions,
        })
    }
}

// ============================================================================
// EXPLICIT MULTI-PROVIDER COMPOSITION — OPTION B
// ============================================================================

/// An explicit provider assignment for one hybrid domain instance.
///
/// The caller supplies the provider identity.
///
/// This module never chooses a provider based on registration order,
/// vendor identity, device type, or an implicit preference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderAssignment {
    instance_id: DomainInstanceId,
    provider_id: ProviderId,
}

impl ProviderAssignment {

    /// Creates an explicit provider assignment.
    #[must_use]
    pub fn new(
        instance_id: DomainInstanceId,
        provider_id: ProviderId,
    ) -> Self {
        Self {
            instance_id,
            provider_id,
        }
    }

    /// Returns the target hybrid instance.
    #[must_use]
    pub const fn instance_id(&self) -> DomainInstanceId {
        self.instance_id
    }

    /// Returns the explicitly requested provider.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }
}

/// One validated provider binding within a provider composition.
///
/// The provider remains the original registered implementation.
/// No provider wrapper or execution engine is introduced.
#[derive(Clone)]
pub struct ProviderBinding {
    instance_id: DomainInstanceId,
    provider: Arc<dyn DomainProvider>,
    compatibility: DomainCompatibilityReport,
}

impl ProviderBinding {

    /// Returns the hybrid instance identity.
    #[must_use]
    pub const fn instance_id(&self) -> DomainInstanceId {
        self.instance_id
    }

    /// Returns the canonical provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        self.provider.descriptor().id()
    }

    /// Returns the registered provider implementation.
    #[must_use]
    pub fn provider(&self) -> &Arc<dyn DomainProvider> {
        &self.provider
    }

    /// Returns the canonical compatibility report.
    #[must_use]
    pub fn compatibility(&self) -> &DomainCompatibilityReport {
        &self.compatibility
    }
}

impl fmt::Debug for ProviderBinding {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        formatter
            .debug_struct("ProviderBinding")
            .field("instance_id", &self.instance_id)
            .field("provider_id", self.provider_id())
            .field(
                "compatible",
                &self.compatibility.is_compatible(),
            )
            .finish()
    }
}

/// Validated explicit composition of multiple registered providers.
///
/// Each hybrid instance receives exactly one explicit provider assignment.
///
/// The same provider may serve multiple instances.
///
/// Different providers may implement different domains or interfaces.
///
/// This is a structural binding model, not an execution graph.
///
/// Data exchange, execution dependencies, synchronization, resource
/// allocation, scheduling, and failure recovery remain with their
/// canonical hybrid and execution authorities.
#[derive(Debug, Clone)]
pub struct ProviderComposition {
    graph_id: HybridGraphId,
    bindings: Vec<ProviderBinding>,
}

impl ProviderComposition {

    /// Returns the associated hybrid graph identity.
    #[must_use]
    pub const fn graph_id(&self) -> HybridGraphId {
        self.graph_id
    }

    /// Returns provider bindings in hybrid declaration order.
    #[must_use]
    pub fn bindings(&self) -> &[ProviderBinding] {
        &self.bindings
    }

    /// Returns the number of explicitly assigned domain instances.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Returns whether the provider composition is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// Looks up the binding for one hybrid domain instance.
    #[must_use]
    pub fn get(
        &self,
        instance_id: DomainInstanceId,
    ) -> Option<&ProviderBinding> {
        self.bindings
            .iter()
            .find(|binding| binding.instance_id == instance_id)
    }

    /// Returns the distinct registered provider identities used.
    ///
    /// The returned list is deterministically ordered.
    #[must_use]
    pub fn provider_ids(&self) -> Vec<ProviderId> {

        self.bindings
            .iter()
            .map(|binding| {
                binding.provider.descriptor().id().clone()
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    /// Returns whether all bindings are structurally compatible.
    #[must_use]
    pub fn is_compatible(&self) -> bool {
        self.bindings
            .iter()
            .all(|binding| binding.compatibility.is_compatible())
    }
}

/// Constructs explicit multi-provider compositions.
///
/// This component does not own a registry.
/// It borrows the existing canonical ZUR registry.
pub struct ProviderCompositionBuilder<'a> {
    registry: &'a DomainProviderRegistry,
    resolver: &'a dyn HybridDomainIdentityResolver,
}

impl<'a> ProviderCompositionBuilder<'a> {

    /// Creates a builder using existing canonical authorities.
    #[must_use]
    pub fn new(
        registry: &'a DomainProviderRegistry,
        resolver: &'a dyn HybridDomainIdentityResolver,
    ) -> Self {
        Self {
            registry,
            resolver,
        }
    }

    /// Resolves an explicit set of provider assignments.
    ///
    /// Every domain instance in the hybrid composition must have exactly
    /// one assignment.
    ///
    /// Duplicate assignments, unknown instances, unregistered providers,
    /// and incompatible provider contracts are rejected.
    ///
    /// The same provider may be assigned to multiple instances.
    ///
    /// An empty hybrid composition accepts an empty assignment collection.
    ///
    /// This method does not perform provider selection or execution.
    pub fn build(
        &self,
        composition: &DomainComposition,
        assignments: impl IntoIterator<Item = ProviderAssignment>,
    ) -> ZurResult<ProviderComposition> {

        composition.validate().map_err(|error| {
            composition_error(
                ZurError::invalid_input(
                    format!(
                        "hybrid composition failed structural validation: {error}"
                    ),
                ),
                "build_provider_composition",
            )
        })?;

        let mut assignment_map =
            BTreeMap::new();

        for assignment in assignments {

            if !assignment_map.contains_key(&assignment.instance_id) {
                assignment_map.insert(
                    assignment.instance_id,
                    assignment.provider_id,
                );
            } else {
                return Err(
                    composition_error(
                        ZurError::invalid_input(
                            format!(
                                "hybrid instance {} has more than one \
                                 explicit provider assignment",
                                assignment.instance_id,
                            ),
                        ),
                        "build_provider_composition",
                    ),
                );
            }
        }

        let known_instances: BTreeSet<_> =
            composition
                .instances()
                .iter()
                .map(|instance| instance.id())
                .collect();

        for instance_id in assignment_map.keys() {
            if !known_instances.contains(instance_id) {
                return Err(
                    composition_error(
                        ZurError::invalid_input(
                            format!(
                                "provider assignment references unknown \
                                 hybrid instance {instance_id}"
                            ),
                        ),
                        "build_provider_composition",
                    ),
                );
            }
        }

        let mut bindings =
            Vec::with_capacity(composition.len());

        for instance in composition.instances() {

            let provider_id =
                assignment_map
                    .get(&instance.id())
                    .ok_or_else(|| {
                        composition_error(
                            ZurError::invalid_input(
                                format!(
                                    "hybrid instance {} has no explicit \
                                     provider assignment",
                                    instance.id(),
                                ),
                            ),
                            "build_provider_composition",
                        )
                    })?;

            let interface = resolve_hybrid_interface(
                instance.interface(),
                self.resolver,
            )?;

            let provider =
                self.registry
                    .get(provider_id)?
                    .ok_or_else(|| {
                        composition_error(
                            ZurError::service_unavailable(
                                format!(
                                    "provider {provider_id} is not \
                                     registered in the supplied registry"
                                ),
                            ),
                            "build_provider_composition",
                        )
                    })?;

            let descriptor =
                find_matching_descriptor(
                    provider.as_ref(),
                    interface.domain(),
                    interface.interface(),
                )?;

            let compatibility =
                check_domain_provider_compatibility(
                    provider.descriptor(),
                    provider.execution_contract(),
                    descriptor,
                )?;

            if !compatibility.is_compatible() {
                return Err(
                    composition_error(
                        ZurError::compatibility(
                            format!(
                                "provider {provider_id} is not compatible \
                                 with hybrid instance {}",
                                instance.id(),
                            ),
                        ),
                        "build_provider_composition",
                    ),
                );
            }

            bindings.push(ProviderBinding {
                instance_id: instance.id(),
                provider,
                compatibility,
            });
        }

        Ok(ProviderComposition {
            graph_id: composition.graph_id(),
            bindings,
        })
    }
}

// ============================================================================
// SHARED VALIDATION
// ============================================================================

/// Finds the canonical domain descriptor that declares the requested
/// domain and provider interface.
///
/// The descriptor remains owned by the provider.
///
/// No descriptor is copied into a competing composition model.
fn find_matching_descriptor<'a>(
    provider: &'a dyn DomainProvider,
    domain: &Domain,
    interface: &ProviderInterfaceId,
) -> ZurResult<&'a DomainDescriptor> {

    provider
        .domain_descriptors()
        .iter()
        .find(|descriptor| {
            descriptor.domain() == domain
                && descriptor.interfaces().contains(interface)
        })
        .ok_or_else(|| {
            composition_error(
                ZurError::compatibility(
                    format!(
                        "provider {} does not declare interface {} \
                         for domain {}",
                        provider.descriptor().id(),
                        interface,
                        domain.qualified_name(),
                    ),
                ),
                "find_matching_descriptor",
            )
        })
}

/// Validates a namespace-qualified identifier.
///
/// This validation is used only for adapter mapping keys.
/// Canonical AST and provider identifiers retain their own validators.
fn validate_identifier(
    value: &str,
    field: &'static str,
) -> ZurResult<()> {

    if value.is_empty() || value.trim() != value {
        return Err(
            composition_error(
                ZurError::invalid_input(
                    format!("{field} must be non-empty and trimmed"),
                ),
                "validate_identifier",
            ),
        );
    }

    let valid = value.split('.').all(|segment| {
        !segment.is_empty()
            && segment.bytes().all(|byte| {
                byte.is_ascii_alphanumeric()
                    || byte == b'_'
                    || byte == b'-'
            })
    });

    if !valid {
        return Err(
            composition_error(
                ZurError::invalid_input(
                    format!(
                        "{field} must contain valid \
                         dot-separated identifier segments"
                    ),
                ),
                "validate_identifier",
            ),
        );
    }

    Ok(())
}

/// Adds canonical ZUR composition diagnostics.
fn composition_error(
    error: ZurError,
    operation: &'static str,
) -> ZurError {

    error
        .with_component(COMPONENT)
        .with_operation(operation)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    use crate::hybrid::contract::DomainInterfaceRef;
    use crate::hybrid::version::DomainContractVersion;

    #[test]
    fn identity_map_preserves_canonical_ast_identity() {

        let canonical =
            Domain::new("zamani", "classical")
                .expect("valid domain");

        let mut map = DomainIdentityMap::new();

        map.insert(
            "classical",
            canonical.clone(),
        )
        .expect("mapping should succeed");

        let resolved =
            map.resolve_domain("classical")
                .expect("domain should resolve");

        assert_eq!(resolved, canonical);
    }

    #[test]
    fn identity_map_rejects_duplicate_keys() {

        let mut map = DomainIdentityMap::new();

        let first =
            Domain::new("zamani", "classical")
                .expect("valid domain");

        let second =
            Domain::new("zamani", "quantum")
                .expect("valid domain");

        map.insert("classical", first)
            .expect("first insertion");

        assert!(
            map.insert("classical", second).is_err()
        );
    }

    #[test]
    fn identity_map_rejects_unknown_domains() {

        let map = DomainIdentityMap::new();

        assert!(
            map.resolve_domain("unknown.domain").is_err()
        );
    }

    #[test]
    fn qualified_resolver_preserves_namespace() {

        let resolver =
            QualifiedDomainIdentityResolver;

        let resolved =
            resolver
                .resolve_domain("zamani.quantum")
                .expect("qualified domain");

        assert_eq!(resolved.namespace(), "zamani");
        assert_eq!(resolved.name(), "quantum");
    }

    #[test]
    fn qualified_resolver_rejects_ambiguous_names() {

        let resolver =
            QualifiedDomainIdentityResolver;

        assert!(
            resolver.resolve_domain("quantum").is_err()
        );
    }

    #[test]
    fn hybrid_interface_conversion_preserves_version() {

        let resolver =
            QualifiedDomainIdentityResolver;

        let reference =
            DomainInterfaceRef::new(
                "zamani.quantum",
                "execution",
                DomainContractVersion::new(3, 2, 1),
            )
            .expect("valid hybrid interface");

        let resolved =
            resolve_hybrid_interface(
                &reference,
                &resolver,
            )
            .expect("conversion should succeed");

        assert_eq!(
            resolved.domain().qualified_name(),
            "zamani.quantum"
        );

        assert_eq!(
            resolved.interface().as_str(),
            "execution"
        );

        assert_eq!(
            resolved.version().to_string(),
            "3.2.1"
        );
    }

    #[test]
    fn explicit_assignment_retains_provider_identity() {

        let graph =
            HybridGraphId::new(1)
                .expect("valid graph identity");

        let instance =
            DomainInstanceId::new(
                graph,
                crate::hybrid::ids::IdComponent::new(1)
                    .expect("valid local identity"),
            );

        let provider_id =
            ProviderId::new("example.provider")
                .expect("valid provider identity");

        let assignment =
            ProviderAssignment::new(
                instance,
                provider_id.clone(),
            );

        assert_eq!(
            assignment.provider_id(),
            &provider_id
        );

        assert_eq!(
            assignment.instance_id(),
            instance
        );
    }
}
