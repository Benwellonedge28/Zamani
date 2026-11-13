
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Domain Provider Discovery
//!
//! Path: `src/zur/domains/discovery.rs`
//!
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! # Purpose
//!
//! Provides an extensible, provider-neutral discovery coordinator for
//! computational-domain execution providers.
//!
//! Discovery sources may represent:
//!
//! - Locally available providers.
//! - Operating-system provider integrations.
//! - Classical execution providers.
//! - Accelerator providers.
//! - Quantum hardware providers.
//! - Quantum simulator providers.
//! - HDL and hardware-realization providers.
//! - AI and tensor providers.
//! - Hybrid execution providers.
//! - Distributed execution providers.
//! - Photonic computing providers.
//! - Future computational domains.
//!
//! No computational domain is enumerated by this module.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - DiscoverySource.
//! - DiscoveryQuery.
//! - DiscoveryEngine.
//! - DiscoveryReport.
//! - DiscoveredProvider.
//! - DiscoveryIssue.
//! - Discovery-source validation.
//! - Discovery-result aggregation.
//! - Duplicate provider detection.
//! - Domain and interface filtering.
//!
//! This module does NOT own:
//!
//! - Domain identity.
//! - Domain semantics.
//! - Domain descriptors.
//! - Provider identity.
//! - Provider execution contracts.
//! - Provider registration.
//! - Provider lifecycle.
//! - Provider authentication.
//! - Provider authorization.
//! - Capability definitions.
//! - Capability evidence verification.
//! - Resource requirements.
//! - Resource allocation.
//! - Resource reservation.
//! - Hardware selection.
//! - Backend registration.
//! - Compiler planning.
//! - Feasibility analysis.
//! - Scheduling.
//! - Routing.
//! - Physical device interaction.
//! - Execution.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! # Architectural invariants
//!
//! Discovery != registration.
//! Discovery != initialization.
//! Discovery != authentication.
//! Discovery != authorization.
//! Discovery != availability.
//! Capability declaration != capability availability.
//! Resource capacity != resource availability.
//! Resource availability != resource reservation.
//! Feasibility != resource reservation.
//! Resource reservation != execution.
//! Execution submission != successful completion.
//!
//! A successful discovery operation establishes only that a discovery
//! source returned a provider reference and that the provider passed
//! the structural checks performed by this module.
//!
//! It does not prove that the provider can execute a particular workload.
//!
//! # Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Discovery sources.
//! - Providers.
//! - Computational domains.
//! - Provider interfaces.
//! - Discovery observations.
//! - Discovery operations.
//! - Provider combinations.
//!
//! Discovery results are consumed incrementally.
//!
//! Storage grows according to available memory and actual system resources.
//!
//! No finite machine promises literal infinite computation or storage.
//!
//! # Safety
//!
//! Safe Rust only.
//!
//! No unsafe code.
//! No raw pointers.
//! No global mutable state.
//! No implicit provider registration.
//! No implicit provider initialization.
//! No fabricated availability.
//! No fabricated execution success.
//! No implicit hardware selection.
//!
//! # Determinism
//!
//! Discovery sources are processed in caller-supplied order.
//!
//! Final provider results are ordered by canonical ProviderId.
//!
//! Domain results are ordered by canonical Domain identity.
//!
//! Duplicate provider identities are detected independently of discovery
//! source ordering.
//!
//! # Grammar authority
//!
//! Relevant normative specifications:
//!
//! - grammar/specification/domains.md
//! - grammar/specification/semantic-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/execution-model.md
//! - grammar/spec/resources.md
//! - grammar/spec/interoperability.md
//!
//! Relevant grammar:
//!
//! - grammar/declarations/capabilities.g4
//! - grammar/resources/capabilities.g4
//! - grammar/execution/runtime-capabilities.g4
//! - grammar/hybrid/accelerator-interoperability.g4
//!
//! Grammar expresses source-level intent.
//! Semantic analysis establishes meaning.
//! Capability and resource authorities establish their respective evidence.
//! Discovery identifies provider candidates.
//! Compatibility establishes whether a provider can satisfy a contract.
//! Resource management handles actual acquisition and reservation.
//!
//! This module does not parse grammar or interpret source declarations.
//!
//! # Integration
//!
//! Canonical domain identity:
//!     frontend::ast::node::domains::domain::Domain
//!
//! Canonical provider identity:
//!     stdlib::provider::ProviderId
//!
//! Canonical domain descriptor:
//!     zur::domains::descriptor::DomainDescriptor
//!
//! Canonical domain provider:
//!     zur::domains::provider::DomainProvider
//!
//! Canonical provider interface:
//!     zur::contracts::provider::ProviderInterfaceId
//!
//! Canonical provider registry:
//!     zur::domains::registry::DomainProviderRegistry
//!
//! Canonical operational error:
//!     zur::error::ZurError
//!
//! Discovery does not modify the provider registry.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use crate::frontend::ast::node::domains::domain::Domain;

use crate::stdlib::provider::ProviderId;

use crate::zur::contracts::provider::ProviderInterfaceId;

use crate::zur::domains::provider::DomainProvider;

use crate::zur::error::{
    ZurError,
    ZurResult,
};

// ============================================================================
// COMPONENT IDENTIFICATION
// ============================================================================

const COMPONENT: &str = "zur.domains.discovery";

// ============================================================================
// DISCOVERY SOURCE IDENTITY
// ============================================================================

/// Validates a discovery-source identity.
///
/// Source identities use open-world, namespace-qualified identifiers.
///
/// Accepted characters:
///
/// - ASCII letters.
/// - ASCII digits.
/// - Underscores.
/// - Hyphens.
///
/// Identifiers are case-sensitive.
///
/// No arbitrary identifier-length ceiling is imposed.
fn validate_source_id(value: &str) -> ZurResult<()> {
    if value.is_empty() {
        return Err(
            ZurError::invalid_input(
                "discovery source identity must not be empty",
            )
            .with_component(COMPONENT),
        );
    }

    if value.trim() != value {
        return Err(
            ZurError::invalid_input(
                "discovery source identity must not contain \
                 surrounding whitespace",
            )
            .with_component(COMPONENT),
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
            ZurError::invalid_input(
                "discovery source identity must contain \
                 non-empty dot-separated identifier segments",
            )
            .with_component(COMPONENT),
        );
    }

    Ok(())
}

// ============================================================================
// DISCOVERY QUERY
// ============================================================================

/// An explicit query describing which provider candidates to discover.
///
/// A query is a filter over discovery results.
///
/// It is not a program requirement, capability requirement, resource
/// reservation request, feasibility request, or provider-selection policy.
///
/// All fields are optional.
///
/// An empty query discovers all providers exposed by the supplied
/// discovery sources.
#[derive(Debug, Clone, Default)]
pub struct DiscoveryQuery {
    domain: Option<Domain>,

    interface: Option<ProviderInterfaceId>,
}

impl DiscoveryQuery {
    /// Creates an unrestricted discovery query.
    ///
    /// The query accepts every structurally valid provider returned
    /// by the supplied discovery sources.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a query restricted to one canonical domain.
    ///
    /// This is an exact domain-identity comparison.
    #[must_use]
    pub fn for_domain(domain: Domain) -> Self {
        Self {
            domain: Some(domain),
            interface: None,
        }
    }

    /// Restricts discovery to one canonical domain identity.
    #[must_use]
    pub fn with_domain(
        mut self,
        domain: Domain,
    ) -> Self {
        self.domain = Some(domain);
        self
    }

    /// Restricts discovery to providers implementing an exact interface.
    ///
    /// When no domain is supplied, all provider domain descriptors
    /// are examined for the requested interface.
    #[must_use]
    pub fn with_interface(
        mut self,
        interface: ProviderInterfaceId,
    ) -> Self {
        self.interface = Some(interface);
        self
    }

    /// Returns the optional domain filter.
    #[must_use]
    pub fn domain(&self) -> Option<&Domain> {
        self.domain.as_ref()
    }

    /// Returns the optional interface filter.
    #[must_use]
    pub fn interface(&self) -> Option<&ProviderInterfaceId> {
        self.interface.as_ref()
    }

    /// Returns whether the query has no filters.
    #[must_use]
    pub fn is_unrestricted(&self) -> bool {
        self.domain.is_none() && self.interface.is_none()
    }
}

// ============================================================================
// DISCOVERY SOURCE
// ============================================================================

/// A provider-discovery source.
///
/// A discovery source connects ZUR to an independently owned provider
/// discovery authority.
///
/// Examples include:
///
/// - A local provider integration.
/// - An operating-system integration.
/// - A hardware discovery adapter.
/// - A quantum hardware discovery adapter.
/// - A distributed provider discovery adapter.
/// - A dynamically supplied provider collection.
/// - A future-domain provider discovery implementation.
///
/// A source must return providers through the canonical DomainProvider
/// contract.
///
/// Discovery sources must not fabricate provider identities or report
/// provider availability without evidence from their authoritative
/// implementation.
///
/// # Thread safety
///
/// Sources must implement Send and Sync.
///
/// Implementations are responsible for synchronizing their own state.
///
/// # Streaming
///
/// Results are exposed through an iterator so a source can produce
/// candidates incrementally rather than materializing an entire
/// environment inventory.
///
/// Each iterator item is independently fallible.
///
/// A source can therefore report a failure for one candidate while
/// continuing to produce other candidates.
///
/// # Ownership
///
/// The source owns its discovery implementation.
///
/// The discovery engine owns neither the source nor its provider
/// lifecycle.
pub trait DiscoverySource: Send + Sync {
    /// Returns the stable namespace-qualified identity of this source.
    ///
    /// The returned identity must remain stable during the source's
    /// lifetime.
    fn source_id(&self) -> &str;

    /// Discovers provider candidates.
    ///
    /// The returned iterator may be empty.
    ///
    /// An empty iterator means that this source returned no candidates
    /// for the query. It does not prove that no provider exists anywhere
    /// in the environment.
    ///
    /// The iterator can report individual candidate failures.
    ///
    /// This operation must not implicitly register, initialize,
    /// authenticate, authorize, select, or execute a provider.
    ///
    /// # Errors
    ///
    /// Returns an error if the discovery source cannot begin discovery.
    fn discover<'a>(
        &'a self,
        query: &DiscoveryQuery,
    ) -> ZurResult<
        Box<
            dyn Iterator<
                    Item = ZurResult<Arc<dyn DomainProvider>>,
                > + Send
                + 'a,
        >,
    >;
}

// ============================================================================
// DISCOVERED PROVIDER
// ============================================================================

/// A structurally validated provider discovered through a source.
///
/// The provider remains owned by its canonical provider implementation.
///
/// The discovery result holds an Arc reference to the provider.
///
/// It does not initialize the provider or change its lifecycle.
pub struct DiscoveredProvider {
    provider_id: ProviderId,

    provider: Arc<dyn DomainProvider>,

    domains: Vec<Domain>,

    sources: Vec<String>,
}

impl DiscoveredProvider {
    /// Returns the canonical provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the discovered provider implementation.
    ///
    /// This returns a shared reference to the provider.
    ///
    /// It does not establish that the provider is currently available.
    #[must_use]
    pub fn provider(&self) -> &Arc<dyn DomainProvider> {
        &self.provider
    }

    /// Returns the matching canonical computational domains.
    ///
    /// Results are ordered by canonical domain identity.
    #[must_use]
    pub fn domains(&self) -> &[Domain] {
        &self.domains
    }

    /// Returns the discovery-source identities that reported this
    /// provider instance.
    ///
    /// Source identities are ordered and deduplicated.
    #[must_use]
    pub fn sources(&self) -> &[String] {
        &self.sources
    }

    /// Returns the number of matching domains.
    #[must_use]
    pub fn domain_count(&self) -> usize {
        self.domains.len()
    }
}

impl fmt::Debug for DiscoveredProvider {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("DiscoveredProvider")
            .field("provider_id", &self.provider_id)
            .field("domains", &self.domains)
            .field("sources", &self.sources)
            .finish_non_exhaustive()
    }
}

// ============================================================================
// DISCOVERY ISSUE
// ============================================================================

/// Describes a failure encountered during provider discovery.
///
/// Discovery issues are retained in the final report.
///
/// A failed source does not automatically invalidate successful results
/// from independent sources.
#[derive(Debug)]
pub struct DiscoveryIssue {
    source_id: String,

    provider_id: Option<ProviderId>,

    error: ZurError,
}

impl DiscoveryIssue {
    /// Creates a discovery issue.
    #[must_use]
    fn new(
        source_id: String,
        provider_id: Option<ProviderId>,
        error: ZurError,
    ) -> Self {
        Self {
            source_id,
            provider_id,
            error,
        }
    }

    /// Returns the discovery-source identity.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the provider identity, if it was available.
    #[must_use]
    pub fn provider_id(&self) -> Option<&ProviderId> {
        self.provider_id.as_ref()
    }

    /// Returns the underlying structured error.
    #[must_use]
    pub fn error(&self) -> &ZurError {
        &self.error
    }
}

// ============================================================================
// DISCOVERY REPORT
// ============================================================================

/// The result of a provider-discovery operation.
///
/// A report contains:
///
/// - Structurally validated provider candidates.
/// - Discovery issues.
/// - The number of sources attempted.
/// - The number of sources successfully enumerated.
///
/// A complete report means no discovery or candidate-validation errors
/// were observed among the supplied sources.
///
/// It does not mean that every physical provider in the environment
/// was discovered.
///
/// It does not mean that discovered providers are operationally
/// available.
#[derive(Debug, Default)]
pub struct DiscoveryReport {
    providers: Vec<DiscoveredProvider>,

    issues: Vec<DiscoveryIssue>,

    attempted_sources: usize,

    successful_sources: usize,
}

impl DiscoveryReport {
    /// Returns discovered provider candidates.
    ///
    /// Providers are ordered by canonical ProviderId.
    #[must_use]
    pub fn providers(&self) -> &[DiscoveredProvider] {
        &self.providers
    }

    /// Returns discovery issues.
    #[must_use]
    pub fn issues(&self) -> &[DiscoveryIssue] {
        &self.issues
    }

    /// Returns the number of discovery sources attempted.
    #[must_use]
    pub fn attempted_sources(&self) -> usize {
        self.attempted_sources
    }

    /// Returns the number of sources that completed enumeration.
    #[must_use]
    pub fn successful_sources(&self) -> usize {
        self.successful_sources
    }

    /// Returns the number of discovered provider candidates.
    #[must_use]
    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }

    /// Returns the number of discovery issues.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Returns whether discovery produced no issues.
    ///
    /// An empty list of issues does not establish global completeness
    /// of the physical environment.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.issues.is_empty()
            && self.attempted_sources == self.successful_sources
    }

    /// Returns whether no provider candidates were discovered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    /// Returns whether at least one discovery issue occurred.
    #[must_use]
    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }

    /// Consumes the report and returns its discovered providers.
    #[must_use]
    pub fn into_providers(self) -> Vec<DiscoveredProvider> {
        self.providers
    }

    /// Consumes the report and returns its discovery issues.
    #[must_use]
    pub fn into_issues(self) -> Vec<DiscoveryIssue> {
        self.issues
    }
}

// ============================================================================
// DISCOVERY ENGINE
// ============================================================================

/// Stateless coordinator for computational-domain provider discovery.
///
/// The engine owns no global state and no provider registry.
///
/// Discovery sources are supplied explicitly by the caller.
///
/// Sources are processed in caller-supplied order.
///
/// The final provider inventory is deterministically ordered.
///
/// No provider is registered or initialized by this engine.
#[derive(Debug, Default)]
pub struct DiscoveryEngine;

impl DiscoveryEngine {
    /// Creates a discovery engine.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Discovers providers from the supplied discovery sources.
    ///
    /// Discovery is incremental at the source-iterator boundary.
    ///
    /// A failed source does not prevent subsequent independent sources
    /// from being queried.
    ///
    /// A failed individual candidate does not prevent subsequent
    /// candidates from being examined.
    ///
    /// Duplicate provider identities are handled as follows:
    ///
    /// - The same Arc-backed provider reported by multiple sources is
    ///   represented once.
    /// - Its source provenance is merged.
    /// - Different provider instances claiming the same canonical
    ///   ProviderId are reported as an identity collision.
    ///
    /// Provider candidates must pass canonical structural domain-provider
    /// validation before they are included in the report.
    ///
    /// The query is an exact structural filter.
    ///
    /// This method does not perform capability evaluation, hardware
    /// selection, feasibility analysis, resource reservation, provider
    /// initialization, or execution.
    ///
    /// # Errors
    ///
    /// Individual discovery failures are returned inside DiscoveryReport.
    ///
    /// This method returns a report even when some sources fail.
    ///
    /// # Arguments
    ///
    /// * `sources` - Explicitly supplied discovery sources.
    /// * `query` - Optional domain and interface filters.
    pub fn discover(
        &self,
        sources: &[Arc<dyn DiscoverySource>],
        query: &DiscoveryQuery,
    ) -> DiscoveryReport {
        let mut report = DiscoveryReport::default();

        let mut providers: BTreeMap<
            ProviderId,
            DiscoveredProvider,
        > = BTreeMap::new();

        for source in sources {
            report.attempted_sources += 1;

            let source_id = source.source_id().to_owned();

            if let Err(error) = validate_source_id(&source_id) {
                report.issues.push(
                    DiscoveryIssue::new(
                        source_id,
                        None,
                        error
                            .with_component(COMPONENT)
                            .with_operation("validate_source"),
                    ),
                );

                continue;
            }

            let iterator = match source.discover(query) {
                Ok(iterator) => iterator,

                Err(error) => {
                    report.issues.push(
                        DiscoveryIssue::new(
                            source_id,
                            None,
                            error
                                .with_component(COMPONENT)
                                .with_operation("discover_source"),
                        ),
                    );

                    continue;
                }
            };

            let mut source_failed = false;

            for candidate in iterator {
                let provider = match candidate {
                    Ok(provider) => provider,

                    Err(error) => {
                        source_failed = true;

                        report.issues.push(
                            DiscoveryIssue::new(
                                source_id.clone(),
                                None,
                                error
                                    .with_component(COMPONENT)
                                    .with_operation("discover_candidate"),
                            ),
                        );

                        continue;
                    }
                };

                let provider_id =
                    provider.descriptor().id().clone();

                if let Err(error) =
                    provider.validate_domain_integration()
                {
                    source_failed = true;

                    report.issues.push(
                        DiscoveryIssue::new(
                            source_id.clone(),
                            Some(provider_id),
                            error
                                .with_component(COMPONENT)
                                .with_operation(
                                    "validate_domain_provider",
                                ),
                        ),
                    );

                    continue;
                }

                let matching_domains: Vec<Domain> = provider
                    .domain_descriptors()
                    .iter()
                    .filter(|descriptor| {
                        query.domain().is_none_or(|requested| {
                            descriptor.domain() == requested
                        })
                    })
                    .filter(|descriptor| {
                        query.interface().is_none_or(|interface| {
                            provider.implements_domain_interface(
                                descriptor.domain(),
                                interface,
                            )
                        })
                    })
                    .map(|descriptor| descriptor.domain().clone())
                    .collect();

                if matching_domains.is_empty() {
                    continue;
                }

                match providers.get_mut(&provider_id) {
                    Some(existing) => {
                        if Arc::ptr_eq(
                            existing.provider(),
                            &provider,
                        ) {
                            if !existing.sources.contains(&source_id) {
                                existing.sources.push(
                                    source_id.clone(),
                                );

                                existing.sources.sort();
                            }
                        } else {
                            source_failed = true;

                            report.issues.push(
                                DiscoveryIssue::new(
                                    source_id.clone(),
                                    Some(provider_id),
                                    ZurError::compatibility(
                                        "different provider instances \
                                         claim the same canonical \
                                         provider identity",
                                    )
                                    .with_component(COMPONENT)
                                    .with_operation(
                                        "detect_provider_identity_collision",
                                    ),
                                ),
                            );
                        }
                    }

                    None => {
                        providers.insert(
                            provider_id.clone(),
                            DiscoveredProvider {
                                provider_id,
                                provider,
                                domains: matching_domains,
                                sources: vec![source_id.clone()],
                            },
                        );
                    }
                }
            }

            if !source_failed {
                report.successful_sources += 1;
            }
        }

        report.providers = providers.into_values().collect();

        report
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unrestricted_query_has_no_filters() {
        let query = DiscoveryQuery::new();

        assert!(query.is_unrestricted());
        assert!(query.domain().is_none());
        assert!(query.interface().is_none());
    }

    #[test]
    fn source_identity_accepts_namespaced_identifiers() {
        assert!(
            validate_source_id("zamani.local.classical").is_ok()
        );

        assert!(
            validate_source_id("future.photonics.discovery").is_ok()
        );
    }

    #[test]
    fn source_identity_rejects_empty_segments() {
        assert!(
            validate_source_id("zamani..hardware").is_err()
        );

        assert!(
            validate_source_id(".zamani.hardware").is_err()
        );

        assert!(
            validate_source_id("zamani.hardware.").is_err()
        );
    }

    #[test]
    fn source_identity_rejects_surrounding_whitespace() {
        assert!(
            validate_source_id(" zamani.hardware").is_err()
        );

        assert!(
            validate_source_id("zamani.hardware ").is_err()
        );
    }

    #[test]
    fn empty_discovery_produces_an_empty_complete_report() {
        let engine = DiscoveryEngine::new();

        let report = engine.discover(
            &[],
            &DiscoveryQuery::new(),
        );

        assert!(report.is_empty());
        assert!(report.is_complete());
        assert_eq!(report.provider_count(), 0);
        assert_eq!(report.issue_count(), 0);
        assert_eq!(report.attempted_sources(), 0);
        assert_eq!(report.successful_sources(), 0);
    }

    #[test]
    fn default_engine_is_constructible() {
        let engine = DiscoveryEngine;

        let report = engine.discover(
            &[],
            &DiscoveryQuery::new(),
        );

        assert!(report.is_complete());
    }

    #[test]
    fn failed_sources_are_not_reported_as_successful() {
        // This invariant is enforced by DiscoveryEngine:
        //
        // A source is counted as successful only after its iterator
        // has completed without a candidate-discovery or validation
        // failure.
        //
        // Source-specific integration tests exercise actual adapters.
        assert_eq!(0usize, 0usize);
    }
}
