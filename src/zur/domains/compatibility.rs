
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Domain Compatibility
//!
//! Path: `src/zur/domains/compatibility.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! ## PURPOSE
//!
//! Evaluates whether a structurally valid ZUR provider contract implements
//! at least one interface declared by a computational-domain descriptor,
//! and whether the provider accepts the domain's declared contract version.
//!
//! This is a contract-compatibility decision, not an execution-feasibility
//! decision.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Domain/provider interface compatibility evaluation.
//! - Domain contract-version acceptance checks.
//! - Deterministic compatibility reports.
//! - Explicit compatibility classifications.
//! - Provider contract-version policy evaluation through the canonical
//!   ZUR version compatibility policy.
//!
//! This module does NOT own:
//!
//! - Domain identity or domain semantics.
//! - Provider identity or provider lifecycle.
//! - Provider registration or discovery.
//! - Provider authentication or trust.
//! - Capability definitions or capability evidence authentication.
//! - Capability availability discovery.
//! - Resource requirements or resource capacity.
//! - Resource allocation, reservation or leases.
//! - Execution feasibility.
//! - Provider selection or hardware selection.
//! - Compiler planning or backend selection.
//! - Classical IR, quantum IR, HDLIR, AI/tensor IR or ZUIR.
//! - Hybrid or distributed execution planning.
//! - Scheduling, routing or physical execution.
//!
//! Those responsibilities remain with their canonical repository owners.
//!
//! ## COMPATIBILITY INVARIANTS
//!
//! Domain identity != provider identity.
//! Domain identity != provider interface.
//! Provider interface != capability.
//! Capability != capability evidence.
//! Capability evidence != authenticated evidence.
//! Capability availability != resource availability.
//! Resource requirement != resource reservation.
//! Compatibility != feasibility.
//! Feasibility != resource reservation.
//! Resource reservation != execution.
//! Submission != successful completion.
//!
//! A provider's declared name or domain label is never sufficient to
//! establish compatibility.
//!
//! Compatibility requires a structurally valid provider contract, an
//! exact interface identity match, and acceptance of the domain contract
//! version by that interface.
//!
//! A domain descriptor may declare alternative provider interfaces.
//! A provider is interface-compatible when it implements at least one
//! declared interface at an accepted version.
//!
//! This rule does not imply that every interface is mandatory or that
//! one provider must implement every interface declared by a domain.
//!
//! ## VERSION OWNERSHIP
//!
//! Domain contract versions are checked against the accepted version range
//! of the matching provider interface.
//!
//! The provider's own ZUR contract version is a separate compatibility
//! dimension. Its compatibility must be checked against the consuming
//! runtime's explicit ServiceContractCompatibilityPolicy.
//!
//! No implicit major-version heuristic, migration or fallback is applied.
//!
//! ## EVIDENCE AND RESOURCES
//!
//! Capability-evidence references and resource-requirement references are
//! preserved as metadata in the report.
//!
//! They are not resolved or interpreted here.
//!
//! Evidence resolution belongs to domains/validation.rs and the canonical
//! capability authority.
//!
//! Resource requirement interpretation belongs to the semantic, compiler,
//! resource and planning authorities.
//!
//! This module never treats a reference as proof of availability, capacity,
//! feasibility or reservation.
//!
//! ## OPEN-WORLD EXTENSIBILITY
//!
//! Domain and interface identities use existing canonical types.
//!
//! No closed list of computational domains, interfaces, devices, vendors,
//! architectures or execution models is introduced.
//!
//! New domains can use the same compatibility contract without modifying
//! this module.
//!
//! ## SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Domains.
//! - Provider interfaces.
//! - Providers.
//! - Capability-evidence references.
//! - Resource-requirement references.
//! - Identifier lengths.
//! - Version component sizes.
//!
//! Ordered collections provide deterministic lookup and reporting.
//!
//! Actual memory and processing remain bounded by available resources.
//! Literal infinite computation is not promised.
//!
//! ## SAFETY
//!
//! Safe Rust only.
//! No unsafe code.
//! No raw pointers.
//! No global mutable state.
//! No filesystem or network access.
//! No hardware discovery.
//! No implicit provider selection.
//! No resource allocation.
//! No fabricated execution success.
//!
//! ## GRAMMAR AUTHORITY
//!
//! This module consumes canonical descriptors and contracts after parsing
//! and semantic analysis.
//!
//! It does not parse grammar or reinterpret source-language meaning.
//!
//! Relevant normative specifications include:
//!
//! - grammar/spec/compatibility.md
//! - grammar/spec/resources.md
//! - grammar/spec/interoperability.md
//! - grammar/specification/domains.md
//! - grammar/specification/semantic-model.md
//!
//! Grammar and semantic analysis establish program meaning.
//! Compatibility evaluates already-established runtime-facing contracts.
//!
//! ## INTEGRATION
//!
//! Canonical domain identity:
//!     frontend::ast::node::domains::domain::Domain
//!
//! Domain descriptors:
//!     zur::domains::descriptor::DomainDescriptor
//!
//! Structural validation:
//!     zur::domains::validation
//!
//! Provider identity and descriptor:
//!     stdlib::provider
//!
//! Provider contract and interface:
//!     zur::contracts::provider
//!
//! Version compatibility:
//!     zur::version
//!
//! Operational errors:
//!     zur::error
//!
//! Downstream consumers:
//!     zur::domains::registry
//!     zur::domains::composition
//!     zur::domains::conformance
//!     zur::execution::dispatch
//!
//! Compatibility does not register, initialize, select or execute providers.

use std::collections::BTreeMap;
use std::fmt;

use crate::stdlib::provider::{
    ProviderDescriptor,
    ProviderId,
};

use crate::zur::contracts::provider::{
    validate_provider_contract,
    CapabilityEvidenceRef,
    ProviderContract,
    ProviderInterface,
    ProviderInterfaceId,
    ResourceRequirementRef,
};

use crate::zur::domains::descriptor::DomainDescriptor;

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::version::{
    Compatibility,
    CompatibilityDecision,
    ServiceContractCompatibilityPolicy,
    ServiceContractVersion,
};

// ============================================================================
// COMPONENT IDENTIFICATION
// ============================================================================

const COMPONENT: &str = "zur.domains.compatibility";

// ============================================================================
// COMPATIBILITY CLASSIFICATION
// ============================================================================

/// Overall compatibility classification for one domain/provider pair.
///
/// This classification describes interface and domain-contract-version
/// compatibility only.
///
/// It does not describe execution feasibility or resource availability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DomainCompatibilityStatus {
    /// At least one declared domain interface is implemented by the
    /// provider at an accepted contract version.
    Compatible,

    /// No declared domain interface is implemented at an accepted version.
    Incompatible,
}

impl DomainCompatibilityStatus {
    /// Returns whether the interface and domain-version contracts match.
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        matches!(self, Self::Compatible)
    }
}

impl fmt::Display for DomainCompatibilityStatus {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(match self {
            Self::Compatible => "compatible",
            Self::Incompatible => "incompatible",
        })
    }
}

// ============================================================================
// INTERFACE COMPATIBILITY
// ============================================================================

/// Compatibility classification for one declared domain interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum InterfaceCompatibilityStatus {
    /// The provider implements the exact interface and accepts the
    /// domain's declared contract version.
    Compatible,

    /// The provider does not implement the declared interface.
    NotImplemented,

    /// The provider implements the interface but does not accept the
    /// domain's declared contract version.
    VersionNotAccepted,
}

impl InterfaceCompatibilityStatus {
    /// Returns whether this interface is compatible.
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        matches!(self, Self::Compatible)
    }
}

impl fmt::Display for InterfaceCompatibilityStatus {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(match self {
            Self::Compatible => "compatible",
            Self::NotImplemented => "not_implemented",
            Self::VersionNotAccepted => "version_not_accepted",
        })
    }
}

/// Compatibility result for one domain-declared provider interface.
///
/// The accepted version range is copied from the canonical provider
/// contract. It is not independently invented by this module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceCompatibility {
    interface: ProviderInterfaceId,
    status: InterfaceCompatibilityStatus,
    accepted_versions: Option<
        crate::zur::version::VersionRange<ServiceContractVersion>,
    >,
}

impl InterfaceCompatibility {
    /// Creates a compatibility result for a declared interface.
    fn new(
        interface: ProviderInterfaceId,
        status: InterfaceCompatibilityStatus,
        accepted_versions: Option<
            crate::zur::version::VersionRange<ServiceContractVersion>,
        >,
    ) -> Self {
        Self {
            interface,
            status,
            accepted_versions,
        }
    }

    /// Returns the canonical interface identity.
    #[must_use]
    pub fn interface(&self) -> &ProviderInterfaceId {
        &self.interface
    }

    /// Returns the compatibility classification.
    #[must_use]
    pub const fn status(&self) -> InterfaceCompatibilityStatus {
        self.status
    }

    /// Returns whether this interface is compatible.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        self.status.is_compatible()
    }

    /// Returns the provider's accepted version range when it implements
    /// this interface.
    ///
    /// `None` means the provider does not implement the interface.
    #[must_use]
    pub fn accepted_versions(
        &self,
    ) -> Option<
        &crate::zur::version::VersionRange<ServiceContractVersion>,
    > {
        self.accepted_versions.as_ref()
    }
}

// ============================================================================
// DOMAIN COMPATIBILITY REPORT
// ============================================================================

/// Deterministic compatibility report for one domain/provider pair.
///
/// The report preserves the canonical identities and contract versions
/// used for the decision.
///
/// Capability-evidence and resource-requirement references remain
/// descriptive metadata. They are not resolved or interpreted here.
///
/// A compatible report does not establish provider availability,
/// execution feasibility, resource availability, reservation, trust,
/// authorization or successful execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainCompatibilityReport {
    provider_id: ProviderId,
    domain: crate::frontend::ast::node::domains::domain::Domain,
    domain_contract_version: ServiceContractVersion,
    provider_contract_version: ServiceContractVersion,
    status: DomainCompatibilityStatus,
    interfaces: Vec<InterfaceCompatibility>,
    domain_capability_evidence: Vec<CapabilityEvidenceRef>,
    provider_capability_evidence: Vec<CapabilityEvidenceRef>,
    domain_resource_requirements: Vec<ResourceRequirementRef>,
    provider_resource_requirements: Vec<ResourceRequirementRef>,
}

impl DomainCompatibilityReport {
    /// Returns the canonical provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the canonical computational-domain identity.
    #[must_use]
    pub fn domain(
        &self,
    ) -> &crate::frontend::ast::node::domains::domain::Domain {
        &self.domain
    }

    /// Returns the domain contract version evaluated by this report.
    #[must_use]
    pub fn domain_contract_version(&self) -> &ServiceContractVersion {
        &self.domain_contract_version
    }

    /// Returns the provider's declared ZUR contract version.
    ///
    /// This version is not implicitly compared with the domain contract
    /// version. Use the explicit provider-contract compatibility policy
    /// for that separate decision.
    #[must_use]
    pub fn provider_contract_version(&self) -> &ServiceContractVersion {
        &self.provider_contract_version
    }

    /// Returns the overall domain/provider compatibility classification.
    #[must_use]
    pub const fn status(&self) -> DomainCompatibilityStatus {
        self.status
    }

    /// Returns whether at least one declared interface is compatible.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        self.status.is_compatible()
    }

    /// Returns the results for all interfaces declared by the domain.
    ///
    /// Results follow the canonical descriptor's deterministic ordering.
    #[must_use]
    pub fn interfaces(&self) -> &[InterfaceCompatibility] {
        &self.interfaces
    }

    /// Returns the number of compatible interfaces.
    #[must_use]
    pub fn compatible_interface_count(&self) -> usize {
        self.interfaces
            .iter()
            .filter(|result| result.is_compatible())
            .count()
    }

    /// Returns capability-evidence references declared by the domain.
    ///
    /// These references are not authenticated or resolved by this report.
    #[must_use]
    pub fn domain_capability_evidence(
        &self,
    ) -> &[CapabilityEvidenceRef] {
        &self.domain_capability_evidence
    }

    /// Returns capability-evidence references declared by the provider.
    ///
    /// These references are not authenticated or resolved by this report.
    #[must_use]
    pub fn provider_capability_evidence(
        &self,
    ) -> &[CapabilityEvidenceRef] {
        &self.provider_capability_evidence
    }

    /// Returns resource requirements declared by the domain.
    ///
    /// These remain requirements, not reservations or resource claims.
    #[must_use]
    pub fn domain_resource_requirements(
        &self,
    ) -> &[ResourceRequirementRef] {
        &self.domain_resource_requirements
    }

    /// Returns resource requirements declared by the provider.
    ///
    /// These describe the provider's own declared requirements and must
    /// not be confused with the workload's resource requirements.
    #[must_use]
    pub fn provider_resource_requirements(
        &self,
    ) -> &[ResourceRequirementRef] {
        &self.provider_resource_requirements
    }
}

// ============================================================================
// STRUCTURAL INPUT VALIDATION
// ============================================================================

/// Validates the provider descriptor and contract before compatibility
/// evaluation.
///
/// Structural validity is delegated to the canonical provider-contract
/// validator. This function does not duplicate provider validation logic.
fn validate_inputs(
    provider_descriptor: &ProviderDescriptor,
    provider_contract: &ProviderContract,
    domain_descriptor: &DomainDescriptor,
) -> ZurResult<()> {
    if provider_descriptor != provider_contract.descriptor() {
        return Err(
            ZurError::invalid_data(
                "provider descriptor does not match the descriptor \
                 embedded in its ZUR provider contract",
            )
            .with_component(COMPONENT)
            .with_operation("validate_inputs")
            .with_provider(provider_descriptor.id().to_string()),
        );
    }

    validate_provider_contract(provider_contract).map_err(|error| {
        error
            .with_component(COMPONENT)
            .with_operation("validate_inputs")
            .with_provider(provider_descriptor.id().to_string())
    })?;

    domain_descriptor.validate().map_err(|error| {
        error
            .with_component(COMPONENT)
            .with_operation("validate_inputs")
            .with_provider(provider_descriptor.id().to_string())
    })?;

    Ok(())
}

// ============================================================================
// DOMAIN / PROVIDER COMPATIBILITY
// ============================================================================

/// Evaluates compatibility between one domain descriptor and one provider.
///
/// The function checks:
///
/// 1. The supplied provider descriptor matches the descriptor embedded in
///    the canonical provider contract.
/// 2. The provider contract passes canonical structural validation.
/// 3. The domain descriptor passes canonical structural validation.
/// 4. Each domain-declared interface is checked by exact interface identity.
/// 5. The provider's accepted version range contains the domain's declared
///    contract version.
///
/// A provider is interface-compatible when at least one declared domain
/// interface is implemented at an accepted version.
///
/// Unsupported interfaces and version mismatches are reported separately.
/// They do not cause an error because they are compatibility outcomes,
/// not malformed input.
///
/// # Errors
///
/// Returns an error when the supplied descriptors or provider contract
/// are structurally invalid or inconsistent.
///
/// # Important
///
/// A successful compatibility report does not prove:
///
/// - that the provider is registered or initialized;
/// - that the provider is trusted or authorized;
/// - that the provider is currently available;
/// - that capability evidence is authentic or fresh;
/// - that resources exist or are available;
/// - that the computation is feasible;
/// - that resources have been reserved;
/// - that execution will succeed.
///
/// Those decisions belong to their respective authorities.
pub fn check_domain_provider_compatibility(
    provider_descriptor: &ProviderDescriptor,
    provider_contract: &ProviderContract,
    domain_descriptor: &DomainDescriptor,
) -> ZurResult<DomainCompatibilityReport> {
    validate_inputs(
        provider_descriptor,
        provider_contract,
        domain_descriptor,
    )?;

    // Build an ordered lookup once. This avoids repeatedly scanning the
    // provider's interface vector for each domain-declared interface.
    let provider_interfaces: BTreeMap<
        &ProviderInterfaceId,
        &ProviderInterface,
    > = provider_contract
        .interfaces()
        .iter()
        .map(|interface| (interface.id(), interface))
        .collect();

    let mut interface_results =
        Vec::with_capacity(domain_descriptor.interfaces().len());

    let mut has_compatible_interface = false;

    for required_interface in domain_descriptor.interfaces() {
        let result = match provider_interfaces.get(required_interface) {
            None => InterfaceCompatibility::new(
                required_interface.clone(),
                InterfaceCompatibilityStatus::NotImplemented,
                None,
            ),

            Some(provider_interface) => {
                let accepted_versions =
                    provider_interface.accepted_versions().clone();

                let status = if accepted_versions.contains(
                    domain_descriptor.contract_version(),
                ) {
                    has_compatible_interface = true;

                    InterfaceCompatibilityStatus::Compatible
                } else {
                    InterfaceCompatibilityStatus::VersionNotAccepted
                };

                InterfaceCompatibility::new(
                    required_interface.clone(),
                    status,
                    Some(accepted_versions),
                )
            }
        };

        interface_results.push(result);
    }

    let status = if has_compatible_interface {
        DomainCompatibilityStatus::Compatible
    } else {
        DomainCompatibilityStatus::Incompatible
    };

    Ok(DomainCompatibilityReport {
        provider_id: provider_contract.provider_id().clone(),
        domain: domain_descriptor.domain().clone(),
        domain_contract_version:
            domain_descriptor.contract_version().clone(),
        provider_contract_version:
            provider_contract.contract_version().clone(),
        status,
        interfaces: interface_results,
        domain_capability_evidence: domain_descriptor
            .capability_evidence()
            .iter()
            .cloned()
            .collect(),
        provider_capability_evidence: provider_contract
            .capability_evidence()
            .to_vec(),
        domain_resource_requirements: domain_descriptor
            .resource_requirements()
            .iter()
            .cloned()
            .collect(),
        provider_resource_requirements: provider_contract
            .resource_requirements()
            .to_vec(),
    })
}

// ============================================================================
// MULTI-DOMAIN COMPATIBILITY
// ============================================================================

/// Evaluates one provider against multiple domain descriptors.
///
/// Every descriptor is evaluated independently and returned in input order.
///
/// An incompatible domain produces an incompatible report rather than
/// aborting the complete operation.
///
/// Structurally invalid descriptors, duplicate domain identities, or an
/// invalid provider contract return an error.
///
/// An empty descriptor collection produces an empty result. This function
/// does not impose an arbitrary domain-count limit.
pub fn check_provider_domain_compatibilities(
    provider_descriptor: &ProviderDescriptor,
    provider_contract: &ProviderContract,
    domain_descriptors: &[DomainDescriptor],
) -> ZurResult<Vec<DomainCompatibilityReport>> {
    crate::zur::domains::validation::validate_domain_descriptors(
        domain_descriptors,
    )
    .map_err(|error| {
        error
            .with_component(COMPONENT)
            .with_operation("check_provider_domain_compatibilities")
            .with_provider(provider_descriptor.id().to_string())
    })?;

    validate_provider_contract(provider_contract).map_err(|error| {
        error
            .with_component(COMPONENT)
            .with_operation("check_provider_domain_compatibilities")
            .with_provider(provider_descriptor.id().to_string())
    })?;

    if provider_descriptor != provider_contract.descriptor() {
        return Err(
            ZurError::invalid_data(
                "provider descriptor does not match the descriptor \
                 embedded in its ZUR provider contract",
            )
            .with_component(COMPONENT)
            .with_operation("check_provider_domain_compatibilities")
            .with_provider(provider_descriptor.id().to_string()),
        );
    }

    domain_descriptors
        .iter()
        .map(|descriptor| {
            check_domain_provider_compatibility(
                provider_descriptor,
                provider_contract,
                descriptor,
            )
        })
        .collect()
}

// ============================================================================
// PROVIDER CONTRACT VERSION POLICY
// ============================================================================

/// Checks a provider's ZUR contract version against an explicitly supplied
/// runtime compatibility policy.
///
/// This is deliberately separate from domain interface compatibility:
///
/// - Domain contract versions are checked against provider-interface ranges.
/// - Provider ZUR contract versions are checked against the consuming
///   runtime's ServiceContractCompatibilityPolicy.
///
/// A migration decision does not execute a migration. The caller must use
/// the appropriate migration authority before consuming a contract that
/// requires migration.
#[must_use]
pub fn check_provider_contract_version(
    provider_contract: &ProviderContract,
    policy: &ServiceContractCompatibilityPolicy,
) -> CompatibilityDecision<ServiceContractVersion> {
    policy.check(provider_contract.contract_version().clone())
}

/// Returns whether a provider contract version is directly compatible with
/// the supplied runtime policy.
///
/// A migration-required version is not considered directly compatible.
#[must_use]
pub fn is_provider_contract_directly_compatible(
    provider_contract: &ProviderContract,
    policy: &ServiceContractCompatibilityPolicy,
) -> bool {
    matches!(
        check_provider_contract_version(provider_contract, policy)
            .compatibility(),
        Compatibility::Compatible
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::domains::domain::Domain;

    use crate::stdlib::provider::{
        ProviderKind,
    };

    use crate::zur::contracts::provider::{
        ProviderInterface,
        ProviderInvocationMode,
    };

    use crate::zur::domains::descriptor::DomainDescriptor;

    use crate::zur::version::{
        ServiceContractVersion,
        Version,
        VersionRange,
    };

    fn version(
        major: &str,
        minor: &str,
        patch: &str,
    ) -> Version {
        Version::new(major, minor, patch)
            .expect("test version must be valid")
    }

    fn service_version(
        major: &str,
        minor: &str,
        patch: &str,
    ) -> ServiceContractVersion {
        ServiceContractVersion::new(major, minor, patch)
            .expect("test service contract version must be valid")
    }

    fn provider_descriptor() -> ProviderDescriptor {
        ProviderDescriptor::new(
            ProviderId::new("org.example.provider")
                .expect("valid provider identity"),
            ProviderKind::new("computation")
                .expect("valid provider kind"),
            "Example provider",
            version("1", "0", "0"),
            version("1", "0", "0"),
        )
        .expect("valid provider descriptor")
    }

    fn provider_interface(
        name: &str,
        minimum: ServiceContractVersion,
        maximum: ServiceContractVersion,
    ) -> ProviderInterface {
        ProviderInterface::new(
            ProviderInterfaceId::new(name)
                .expect("valid interface identity"),
            VersionRange::new(minimum, maximum)
                .expect("valid version range"),
            [ProviderInvocationMode::Synchronous],
        )
        .expect("valid provider interface")
    }

    fn provider_contract(
        interfaces: Vec<ProviderInterface>,
    ) -> ProviderContract {
        ProviderContract::new(
            provider_descriptor(),
            service_version("1", "0", "0"),
            interfaces,
        )
        .expect("valid provider contract")
    }

    fn domain_descriptor(
        namespace: &str,
        name: &str,
        contract_version: ServiceContractVersion,
        interfaces: &[&str],
    ) -> DomainDescriptor {
        let domain = Domain::new(namespace, name)
            .expect("valid domain identity");

        let mut descriptor = DomainDescriptor::new(
            domain,
            contract_version,
            format!("{name} domain"),
        )
        .expect("valid domain descriptor");

        for interface in interfaces {
            descriptor
                .insert_interface(
                    ProviderInterfaceId::new(*interface)
                        .expect("valid interface identity"),
                )
                .expect("interface insertion must succeed");
        }

        descriptor
    }

    #[test]
    fn accepts_exact_interface_and_domain_version() {
        let provider = provider_descriptor();

        let contract = provider_contract(vec![
            provider_interface(
                "zamani.classical.execution",
                service_version("1", "0", "0"),
                service_version("2", "0", "0"),
            ),
        ]);

        let domain = domain_descriptor(
            "zamani",
            "classical",
            service_version("1", "4", "0"),
            &["zamani.classical.execution"],
        );

        let report = check_domain_provider_compatibility(
            &provider,
            &contract,
            &domain,
        )
        .expect("compatibility evaluation must succeed");

        assert_eq!(
            report.status(),
            DomainCompatibilityStatus::Compatible,
        );

        assert_eq!(report.compatible_interface_count(), 1);
        assert_eq!(
            report.interfaces()[0].status(),
            InterfaceCompatibilityStatus::Compatible,
        );
    }

    #[test]
    fn rejects_a_provider_that_does_not_implement_the_interface() {
        let provider = provider_descriptor();

        let contract = provider_contract(vec![
            provider_interface(
                "zamani.quantum.execution",
                service_version("1", "0", "0"),
                service_version("2", "0", "0"),
            ),
        ]);

        let domain = domain_descriptor(
            "zamani",
            "classical",
            service_version("1", "0", "0"),
            &["zamani.classical.execution"],
        );

        let report = check_domain_provider_compatibility(
            &provider,
            &contract,
            &domain,
        )
        .expect("valid but unsupported interface is a report");

        assert_eq!(
            report.status(),
            DomainCompatibilityStatus::Incompatible,
        );

        assert_eq!(
            report.interfaces()[0].status(),
            InterfaceCompatibilityStatus::NotImplemented,
        );
    }

    #[test]
    fn rejects_an_unaccepted_domain_contract_version() {
        let provider = provider_descriptor();

        let contract = provider_contract(vec![
            provider_interface(
                "zamani.classical.execution",
                service_version("1", "0", "0"),
                service_version("1", "5", "0"),
            ),
        ]);

        let domain = domain_descriptor(
            "zamani",
            "classical",
            service_version("2", "0", "0"),
            &["zamani.classical.execution"],
        );

        let report = check_domain_provider_compatibility(
            &provider,
            &contract,
            &domain,
        )
        .expect("version incompatibility is a report");

        assert_eq!(
            report.status(),
            DomainCompatibilityStatus::Incompatible,
        );

        assert_eq!(
            report.interfaces()[0].status(),
            InterfaceCompatibilityStatus::VersionNotAccepted,
        );
    }

    #[test]
    fn accepts_one_compatible_alternative_interface() {
        let provider = provider_descriptor();

        let contract = provider_contract(vec![
            provider_interface(
                "zamani.classical.native",
                service_version("1", "0", "0"),
                service_version("1", "0", "0"),
            ),
        ]);

        let domain = domain_descriptor(
            "zamani",
            "classical",
            service_version("1", "0", "0"),
            &[
                "zamani.classical.native",
                "zamani.classical.interpreted",
            ],
        );

        let report = check_domain_provider_compatibility(
            &provider,
            &contract,
            &domain,
        )
        .expect("compatibility evaluation must succeed");

        assert_eq!(
            report.status(),
            DomainCompatibilityStatus::Compatible,
        );

        assert_eq!(report.compatible_interface_count(), 1);

        assert_eq!(
            report.interfaces()[1].status(),
            InterfaceCompatibilityStatus::NotImplemented,
        );
    }

    #[test]
    fn supports_future_open_world_domains() {
        let provider = provider_descriptor();

        let contract = provider_contract(vec![
            provider_interface(
                "future.photonics.execution",
                service_version("1", "0", "0"),
                service_version("3", "0", "0"),
            ),
        ]);

        let domain = domain_descriptor(
            "future",
            "photonics",
            service_version("2", "0", "0"),
            &["future.photonics.execution"],
        );

        let report = check_domain_provider_compatibility(
            &provider,
            &contract,
            &domain,
        )
        .expect("future domain must not require a new enum variant");

        assert!(report.is_compatible());
        assert_eq!(
            report.domain().qualified_name(),
            "future.photonics",
        );
    }

    #[test]
    fn rejects_mismatched_provider_descriptors() {
        let supplied = provider_descriptor();

        let other = ProviderDescriptor::new(
            ProviderId::new("org.example.other")
                .expect("valid provider identity"),
            ProviderKind::new("computation")
                .expect("valid provider kind"),
            "Other provider",
            version("1", "0", "0"),
            version("1", "0", "0"),
        )
        .expect("valid provider descriptor");

        let contract = ProviderContract::new(
            other,
            service_version("1", "0", "0"),
            vec![
                provider_interface(
                    "zamani.classical.execution",
                    service_version("1", "0", "0"),
                    service_version("1", "0", "0"),
                ),
            ],
        )
        .expect("valid provider contract");

        let domain = domain_descriptor(
            "zamani",
            "classical",
            service_version("1", "0", "0"),
            &["zamani.classical.execution"],
        );

        assert!(
            check_domain_provider_compatibility(
                &supplied,
                &contract,
                &domain,
            )
            .is_err()
        );
    }

    #[test]
    fn checks_provider_contract_version_using_explicit_policy() {
        let contract = provider_contract(vec![
            provider_interface(
                "zamani.classical.execution",
                service_version("1", "0", "0"),
                service_version("1", "0", "0"),
            ),
        ]);

        let policy = ServiceContractCompatibilityPolicy::exact(
            service_version("1", "0", "0"),
        )
        .expect("exact compatibility policy");

        let decision =
            check_provider_contract_version(&contract, &policy);

        assert_eq!(
            decision.compatibility(),
            Compatibility::Compatible,
        );

        assert!(
            is_provider_contract_directly_compatible(
                &contract,
                &policy,
            )
        );
    }

    #[test]
    fn empty_multi_domain_input_produces_empty_results() {
        let provider = provider_descriptor();

        let contract = provider_contract(vec![]);

        let reports = check_provider_domain_compatibilities(
            &provider,
            &contract,
            &[],
        )
        .expect("empty domain collection is valid");

        assert!(reports.is_empty());
    }
}
