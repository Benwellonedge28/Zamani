
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Distributed Capability Discovery
//!
//! Path: `src/distributed/capabilities.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Discover capability evidence from pluggable distributed-environment
//! providers and aggregate it using Zamani's canonical semantic evidence
//! model.
//!
//! ## Ownership
//!
//! This module owns:
//! - Registration and replacement of capability-discovery providers.
//! - Deterministic provider invocation order.
//! - Aggregation of evidence returned by providers.
//! - Reporting of provider failures and incomplete discovery.
//! - Validation that evidence belongs to the requested scope.
//!
//! This module does not own:
//! - Capability identities, versions, properties, or requirements.
//! - The semantic meaning of capabilities.
//! - Authentication, authorization, or evidence trust verification.
//! - Physical resource quantities or resource allocation.
//! - Target feasibility, placement, scheduling, or routing.
//! - Hardware discovery implementations.
//! - ZUIR, ZCIR, quantum::ir, HDLIR, or backend code generation.
//! - Network transport or remote execution.
//!
//! The canonical semantic capability and evidence types remain authoritative
//! under `crate::frontend::semantic::capabilities`.
//!
//! ## POCO-REAF and scalability
//!
//! Provider IDs and capability namespaces are open-world strings. New
//! capability providers and computational domains do not require new enum
//! variants or changes to this module.
//!
//! There is no universal maximum number of providers or evidence records.
//! Collections grow according to available memory and configured operational
//! policy. Rust collections, address spaces, network operations, and physical
//! resources remain finite; literal infinite capacity is not promised.
//!
//! ## Evidence and trust
//!
//! Discovery records claims; it does not authenticate their origin or prove
//! that a claim is trustworthy. Security and attestation services must verify
//! evidence before a security-sensitive decision relies on it.
//!
//! Missing evidence is not proof of support or non-support. Contradictory
//! claims remain visible through the canonical evidence evaluator.
//!
//! ## Failure behavior
//!
//! A provider failure does not discard evidence successfully obtained from
//! other providers. The report is marked incomplete and preserves the failed
//! provider's error as an error source.
//!
//! A discovery report with no registered providers is incomplete. A report
//! with successful provider calls can still contain no evidence; callers must
//! inspect the canonical evaluation result instead of treating an empty set
//! as proof that a capability is unavailable.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;
use std::sync::Arc;

use crate::frontend::semantic::capabilities::evidence::{
    CapabilityEvidence,
    CapabilityEvidenceError,
    CapabilityEvidenceResult,
    CapabilityEvidenceSet,
    EvidenceSubject,
};
use crate::frontend::semantic::capabilities::model::CapabilityRequirement;

/// Provider identifiers must remain stable for the duration of registration.
///
/// This value is an identifier for a discovery implementation, not a
/// capability identity, participant identity, or physical device identity.
type ProviderId = String;

/// A boxed error returned by a capability provider.
///
/// The original error is preserved so callers can inspect the source chain.
pub type ProviderError = Box<dyn Error + Send + Sync + 'static>;

/// An implementation that supplies scoped capability evidence.
///
/// Implementations should return evidence for the exact requested subject.
/// They should perform their own transport timeouts, cancellation handling,
/// input validation, and provider-specific security checks.
///
/// The returned records are claims. Their presence does not independently
/// authenticate the provider or establish that the claims are trusted.
///
/// Implementations should not silently truncate evidence when an operational
/// budget is exhausted. They should return an error so discovery is reported
/// as incomplete.
pub trait CapabilityProvider: Send + Sync {
    /// Discover evidence relevant to the requested scope.
    ///
    /// # Errors
    ///
    /// Returns the provider's original error when discovery fails. The
    /// catalog preserves this error in the corresponding provider result.
    fn discover(
        &self,
        subject: &EvidenceSubject,
    ) -> Result<Vec<CapabilityEvidence>, ProviderError>;
}

/// Errors produced while registering or replacing providers.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CapabilityRegistryError {
    /// A provider ID is empty, whitespace-only, or contains control characters.
    InvalidProviderId {
        /// The supplied provider identifier.
        provider_id: String,
    },

    /// The provider ID is already registered.
    ///
    /// Use `replace_provider` to replace an existing provider explicitly.
    DuplicateProvider {
        /// The conflicting provider identifier.
        provider_id: String,
    },

    /// No provider with the requested ID is registered.
    UnknownProvider {
        /// The missing provider identifier.
        provider_id: String,
    },
}

impl fmt::Display for CapabilityRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidProviderId { provider_id } => write!(
                f,
                "invalid distributed capability provider ID {provider_id:?}"
            ),
            Self::DuplicateProvider { provider_id } => write!(
                f,
                "distributed capability provider {provider_id:?} is already registered"
            ),
            Self::UnknownProvider { provider_id } => write!(
                f,
                "distributed capability provider {provider_id:?} is not registered"
            ),
        }
    }
}

impl Error for CapabilityRegistryError {}

/// Errors that can invalidate evidence returned by a provider.
#[derive(Debug)]
#[non_exhaustive]
pub enum CapabilityDiscoveryError {
    /// A provider returned evidence for a different scope.
    ScopeMismatch {
        /// Registered provider that returned the evidence.
        provider_id: String,

        /// Requested scope.
        expected_subject: String,

        /// Scope claimed by the returned evidence.
        actual_subject: String,
    },

    /// A returned evidence collection failed canonical validation.
    InvalidEvidence {
        /// Registered provider that returned the evidence.
        provider_id: String,

        /// Original canonical evidence-validation error.
        source: CapabilityEvidenceError,
    },
}

impl fmt::Display for CapabilityDiscoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ScopeMismatch {
                provider_id,
                expected_subject,
                actual_subject,
            } => write!(
                f,
                "capability provider {provider_id:?} returned evidence for \
                 scope {actual_subject:?}; requested scope was \
                 {expected_subject:?}"
            ),
            Self::InvalidEvidence {
                provider_id,
                source,
            } => write!(
                f,
                "capability provider {provider_id:?} returned invalid evidence: \
                 {source}"
            ),
        }
    }
}

impl Error for CapabilityDiscoveryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidEvidence { source, .. } => Some(source),
            Self::ScopeMismatch { .. } => None,
        }
    }
}

/// Outcome of querying one registered provider.
///
/// Failed results preserve the original error chain. Successful results
/// record how many evidence records the provider returned before aggregation.
/// Exact duplicate records may be deduplicated by the canonical evidence set.
#[derive(Debug)]
#[non_exhaustive]
pub enum ProviderResult {
    /// The provider returned valid evidence for the requested scope.
    Succeeded {
        /// Registered provider identifier.
        provider_id: String,

        /// Number of records returned by this provider.
        records_returned: usize,
    },

    /// The provider failed or returned invalid evidence.
    Failed {
        /// Registered provider identifier.
        provider_id: String,

        /// Original provider or validation error.
        source: ProviderError,
    },
}

impl ProviderResult {
    /// Returns the registered provider identifier.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        match self {
            Self::Succeeded { provider_id, .. }
            | Self::Failed { provider_id, .. } => provider_id,
        }
    }

    /// Returns whether the provider query succeeded and its evidence was valid.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        matches!(self, Self::Succeeded { .. })
    }

    /// Returns the original failure, if this provider query failed.
    #[must_use]
    pub fn error(&self) -> Option<&(dyn Error + Send + Sync + 'static)> {
        match self {
            Self::Succeeded { .. } => None,
            Self::Failed { source, .. } => Some(source.as_ref()),
        }
    }
}

/// A report of capability discovery for one explicitly requested scope.
///
/// `is_complete()` means at least one provider was registered and every
/// registered provider completed successfully. It does not mean every
/// capability in the universe has been discovered or that the evidence has
/// been authenticated.
///
/// The evidence set is the canonical semantic collection. This report does
/// not create an alternative capability inventory model.
#[derive(Debug)]
pub struct CapabilityDiscoveryReport {
    subject: EvidenceSubject,
    evidence: CapabilityEvidenceSet,
    provider_results: Vec<ProviderResult>,
    registered_provider_count: usize,
}

impl CapabilityDiscoveryReport {
    /// Returns the requested evidence scope.
    #[must_use]
    pub fn subject(&self) -> &EvidenceSubject {
        &self.subject
    }

    /// Returns the canonical aggregated evidence set.
    #[must_use]
    pub fn evidence(&self) -> &CapabilityEvidenceSet {
        &self.evidence
    }

    /// Returns provider outcomes in deterministic provider-ID order.
    #[must_use]
    pub fn provider_results(&self) -> &[ProviderResult] {
        &self.provider_results
    }

    /// Returns the number of providers registered when discovery began.
    #[must_use]
    pub const fn registered_provider_count(&self) -> usize {
        self.registered_provider_count
    }

    /// Returns the number of providers that returned an outcome.
    #[must_use]
    pub fn attempted_provider_count(&self) -> usize {
        self.provider_results.len()
    }

    /// Returns whether at least one provider was queried and every registered
    /// provider succeeded.
    ///
    /// This is a discovery-completeness result, not proof of capability
    /// availability or evidence authenticity.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.registered_provider_count > 0
            && self.provider_results.len() == self.registered_provider_count
            && self.provider_results.iter().all(ProviderResult::is_success)
    }

    /// Returns whether any provider query failed.
    #[must_use]
    pub fn has_failures(&self) -> bool {
        self.provider_results.iter().any(|result| !result.is_success())
    }

    /// Evaluates a requirement using the canonical evidence semantics.
    ///
    /// The result distinguishes no evidence, satisfaction, incompatibility,
    /// indeterminate evidence, and contradictory claims. It does not by itself
    /// decide whether a distributed plan is feasible.
    #[must_use]
    pub fn evaluate(
        &self,
        requirement: &CapabilityRequirement,
    ) -> CapabilityEvidenceResult {
        self.evidence.evaluate(&self.subject, requirement)
    }
}

/// A provider registry for distributed capability discovery.
///
/// Providers are keyed by explicit, validated IDs. A `BTreeMap` gives
/// deterministic invocation and reporting order. The registry has no global
/// singleton and is owned by the runtime or discovery service that constructs
/// it.
///
/// This registry records provider implementations, not capability definitions.
/// Capability meaning and evidence evaluation remain owned by the semantic
/// capability subsystem.
#[derive(Default)]
pub struct CapabilityCatalog {
    providers: BTreeMap<ProviderId, Arc<dyn CapabilityProvider>>,
}

impl fmt::Debug for CapabilityCatalog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CapabilityCatalog")
            .field("provider_ids", &self.providers.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl CapabilityCatalog {
    /// Creates an empty provider catalog.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of registered providers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    /// Returns whether no providers are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    /// Returns the registered provider IDs in deterministic order.
    #[must_use]
    pub fn provider_ids(&self) -> impl ExactSizeIterator<Item = &str> {
        self.providers.keys().map(String::as_str)
    }

    /// Registers a provider under an explicit identifier.
    ///
    /// Duplicate registration is rejected rather than silently replacing an
    /// existing provider.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid ID or an already registered ID.
    pub fn register_provider(
        &mut self,
        provider_id: impl Into<String>,
        provider: Arc<dyn CapabilityProvider>,
    ) -> Result<(), CapabilityRegistryError> {
        let provider_id = provider_id.into();
        validate_provider_id(&provider_id)?;

        if self.providers.contains_key(&provider_id) {
            return Err(CapabilityRegistryError::DuplicateProvider { provider_id });
        }

        self.providers.insert(provider_id, provider);
        Ok(())
    }

    /// Replaces a registered provider explicitly.
    ///
    /// Returns the previous provider so the owner can control its lifecycle.
    ///
    /// # Errors
    ///
    /// Returns an error when the ID is invalid or not currently registered.
    pub fn replace_provider(
        &mut self,
        provider_id: impl Into<String>,
        provider: Arc<dyn CapabilityProvider>,
    ) -> Result<Arc<dyn CapabilityProvider>, CapabilityRegistryError> {
        let provider_id = provider_id.into();
        validate_provider_id(&provider_id)?;

        let Some(previous) = self.providers.get_mut(&provider_id) else {
            return Err(CapabilityRegistryError::UnknownProvider { provider_id });
        };

        Ok(std::mem::replace(previous, provider))
    }

    /// Removes a registered provider.
    ///
    /// # Errors
    ///
    /// Returns an error when the ID is invalid or not registered.
    pub fn unregister_provider(
        &mut self,
        provider_id: &str,
    ) -> Result<Arc<dyn CapabilityProvider>, CapabilityRegistryError> {
        validate_provider_id(provider_id)?;

        self.providers
            .remove(provider_id)
            .ok_or_else(|| CapabilityRegistryError::UnknownProvider {
                provider_id: provider_id.to_owned(),
            })
    }

    /// Discovers and aggregates evidence for one explicit scope.
    ///
    /// Providers are called in deterministic ID order. A provider failure
    /// does not discard successful evidence from other providers. The result
    /// is marked incomplete when any provider fails, returns evidence for the
    /// wrong scope, or returns invalid evidence.
    ///
    /// This method is synchronous. Providers are responsible for enforcing
    /// their own deadlines and cancellation policies. The caller should
    /// configure those policies through its runtime/provider configuration.
    ///
    /// # Evidence trust
    ///
    /// This method validates the evidence model's local invariants. It does
    /// not authenticate providers, verify signatures, or authorize execution.
    #[must_use]
    pub fn discover(
        &self,
        subject: &EvidenceSubject,
    ) -> CapabilityDiscoveryReport {
        let mut evidence = CapabilityEvidenceSet::new();
        let mut provider_results = Vec::with_capacity(self.providers.len());
        let registered_provider_count = self.providers.len();

        for (provider_id, provider) in &self.providers {
            let records = match provider.discover(subject) {
                Ok(records) => records,
                Err(source) => {
                    provider_results.push(ProviderResult::Failed {
                        provider_id: provider_id.clone(),
                        source,
                    });
                    continue;
                }
            };

            let record_count = records.len();

            if let Some(record) = records
                .iter()
                .find(|record| record.subject() != subject)
            {
                let source: ProviderError =
                    Box::new(CapabilityDiscoveryError::ScopeMismatch {
                        provider_id: provider_id.clone(),
                        expected_subject: subject.to_string(),
                        actual_subject: record.subject().to_string(),
                    });

                provider_results.push(ProviderResult::Failed {
                    provider_id: provider_id.clone(),
                    source,
                });
                continue;
            }

            let mut next_evidence = evidence.clone();

            if let Err(error) = next_evidence.extend(records) {
                let source: ProviderError =
                    Box::new(CapabilityDiscoveryError::InvalidEvidence {
                        provider_id: provider_id.clone(),
                        source: error,
                    });

                provider_results.push(ProviderResult::Failed {
                    provider_id: provider_id.clone(),
                    source,
                });
                continue;
            }

            evidence = next_evidence;

            provider_results.push(ProviderResult::Succeeded {
                provider_id: provider_id.clone(),
                records_returned: record_count,
            });
        }

        CapabilityDiscoveryReport {
            subject: subject.clone(),
            evidence,
            provider_results,
            registered_provider_count,
        }
    }
}

fn validate_provider_id(
    provider_id: &str,
) -> Result<(), CapabilityRegistryError> {
    if provider_id.trim().is_empty()
        || provider_id.chars().any(char::is_control)
    {
        return Err(CapabilityRegistryError::InvalidProviderId {
            provider_id: provider_id.to_owned(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EmptyProvider;

    impl CapabilityProvider for EmptyProvider {
        fn discover(
            &self,
            _subject: &EvidenceSubject,
        ) -> Result<Vec<CapabilityEvidence>, ProviderError> {
            Ok(Vec::new())
        }
    }

    struct FailingProvider;

    impl CapabilityProvider for FailingProvider {
        fn discover(
            &self,
            _subject: &EvidenceSubject,
        ) -> Result<Vec<CapabilityEvidence>, ProviderError> {
            Err(Box::new(std::io::Error::other(
                "simulated discovery failure",
            )))
        }
    }

    struct WrongScopeProvider {
        evidence: CapabilityEvidence,
    }

    impl CapabilityProvider for WrongScopeProvider {
        fn discover(
            &self,
            _subject: &EvidenceSubject,
        ) -> Result<Vec<CapabilityEvidence>, ProviderError> {
            Ok(vec![self.evidence.clone()])
        }
    }

    fn subject(value: &str) -> EvidenceSubject {
        EvidenceSubject::new(value).expect("valid evidence subject")
    }

    fn evidence(subject: &EvidenceSubject) -> CapabilityEvidence {
        use crate::frontend::semantic::capabilities::evidence::{
            CapabilityEvidenceStatus,
            EvidenceOrigin,
        };
        use crate::frontend::semantic::capabilities::model::CapabilityId;

        CapabilityEvidence::new(
            CapabilityId::new("test.domain", "test.capability")
                .expect("valid capability ID"),
            subject.clone(),
            CapabilityEvidenceStatus::Supported,
            None,
            BTreeMap::new(),
            EvidenceOrigin::new("test", "unit-test", None)
                .expect("valid evidence origin"),
        )
        .expect("valid evidence")
    }

    #[test]
    fn empty_catalog_is_not_reported_as_complete() {
        let catalog = CapabilityCatalog::new();
        let report = catalog.discover(&subject("test:scope"));

        assert!(catalog.is_empty());
        assert!(!report.is_complete());
        assert!(!report.has_failures());
        assert_eq!(report.registered_provider_count(), 0);
        assert_eq!(report.attempted_provider_count(), 0);
        assert!(report.evidence().is_empty());
    }

    #[test]
    fn provider_ids_are_deterministically_ordered() {
        let mut catalog = CapabilityCatalog::new();

        catalog
            .register_provider("z-provider", Arc::new(EmptyProvider))
            .expect("register z provider");
        catalog
            .register_provider("a-provider", Arc::new(EmptyProvider))
            .expect("register a provider");

        assert_eq!(
            catalog.provider_ids().collect::<Vec<_>>(),
            vec!["a-provider", "z-provider"]
        );

        let report = catalog.discover(&subject("test:scope"));

        assert!(report.is_complete());
        assert_eq!(report.attempted_provider_count(), 2);
        assert_eq!(report.provider_results()[0].provider_id(), "a-provider");
        assert_eq!(report.provider_results()[1].provider_id(), "z-provider");
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let mut catalog = CapabilityCatalog::new();

        catalog
            .register_provider("provider", Arc::new(EmptyProvider))
            .expect("first registration succeeds");

        assert!(matches!(
            catalog.register_provider("provider", Arc::new(EmptyProvider)),
            Err(CapabilityRegistryError::DuplicateProvider { .. })
        ));
    }

    #[test]
    fn invalid_provider_ids_are_rejected() {
        let mut catalog = CapabilityCatalog::new();

        assert!(matches!(
            catalog.register_provider("", Arc::new(EmptyProvider)),
            Err(CapabilityRegistryError::InvalidProviderId { .. })
        ));

        assert!(matches!(
            catalog.register_provider("bad\nprovider", Arc::new(EmptyProvider)),
            Err(CapabilityRegistryError::InvalidProviderId { .. })
        ));
    }

    #[test]
    fn failed_provider_does_not_hide_successful_provider_outcomes() {
        let mut catalog = CapabilityCatalog::new();

        catalog
            .register_provider("working", Arc::new(EmptyProvider))
            .expect("register working provider");
        catalog
            .register_provider("broken", Arc::new(FailingProvider))
            .expect("register failing provider");

        let report = catalog.discover(&subject("test:scope"));

        assert!(!report.is_complete());
        assert!(report.has_failures());
        assert_eq!(report.attempted_provider_count(), 2);

        let failed = report
            .provider_results()
            .iter()
            .find(|result| result.provider_id() == "broken")
            .expect("failed provider result exists");

        assert!(failed.error().is_some());
    }

    #[test]
    fn evidence_for_another_scope_is_rejected() {
        let wrong_scope = subject("test:other-scope");
        let requested_scope = subject("test:requested-scope");

        let mut catalog = CapabilityCatalog::new();
        catalog
            .register_provider(
                "wrong-scope",
                Arc::new(WrongScopeProvider {
                    evidence: evidence(&wrong_scope),
                }),
            )
            .expect("register provider");

        let report = catalog.discover(&requested_scope);

        assert!(!report.is_complete());
        assert!(report.has_failures());
        assert!(report.evidence().is_empty());

        let result = &report.provider_results()[0];
        assert!(result.error().is_some());
    }

    #[test]
    fn valid_evidence_is_aggregated_through_the_canonical_model() {
        let requested_scope = subject("test:cluster");

        let mut catalog = CapabilityCatalog::new();
        catalog
            .register_provider(
                "provider",
                Arc::new(WrongScopeProvider {
                    evidence: evidence(&requested_scope),
                }),
            )
            .expect("register provider");

        let report = catalog.discover(&requested_scope);

        assert!(report.is_complete());
        assert_eq!(report.evidence().len(), 1);
    }

    #[test]
    fn unregister_requires_an_existing_provider() {
        let mut catalog = CapabilityCatalog::new();

        assert!(matches!(
            catalog.unregister_provider("missing"),
            Err(CapabilityRegistryError::UnknownProvider { .. })
        ));

        catalog
            .register_provider("existing", Arc::new(EmptyProvider))
            .expect("register provider");

        assert!(catalog.unregister_provider("existing").is_ok());
        assert!(catalog.is_empty());
    }
}
