
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Provider Discovery
//!
//! Path: src/zhal/providers/discovery.rs
//!
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Coordinates hardware discovery across independently implemented
//! ZHAL hardware providers.
//!
//! It aggregates provider observations without taking ownership of:
//!
//! - provider identity;
//! - provider registration;
//! - provider lifecycle;
//! - hardware-provider implementation contracts;
//! - device inventory storage;
//! - capability evaluation;
//! - resource allocation;
//! - hardware scheduling;
//! - hardware routing;
//! - domain-specific execution.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - DiscoveryOptions.
//! - DiscoveryFailurePolicy.
//! - DiscoveryFailure.
//! - DiscoveryFailurePhase.
//! - DiscoveryReport.
//! - ProviderDiscoveryPage.
//! - DiscoverySession.
//! - Deterministic provider enumeration.
//! - Round-robin discovery coordination.
//! - Paginated discovery progression.
//! - Cross-page duplicate detection.
//! - Continuation cursor cycle detection.
//! - Strict and best-effort failure handling.
//! - Incremental discovery reporting.
//!
//! # Canonical authorities
//!
//! Provider identity:
//!     crate::stdlib::provider
//!
//! Provider registration:
//!     crate::zhal::providers::registry
//!
//! Hardware-provider contract:
//!     crate::zhal::providers::contract
//!
//! Provider lifecycle:
//!     crate::zhal::providers::lifecycle
//!
//! Hardware context:
//!     crate::zhal::context
//!
//! Hardware errors:
//!     crate::zhal::errors
//!
//! Hardware device identity:
//!     crate::zhal::identity
//!
//! Hardware device descriptions:
//!     crate::zhal::model::device
//!
//! Device inventory:
//!     crate::zhal::devices::inventory
//!
//! Runtime resource management:
//!     crate::zur
//!
//! # Discovery semantics
//!
//! Discovery is observational.
//!
//! It does not:
//!
//! - reserve hardware;
//! - allocate physical resources;
//! - authorize hardware access;
//! - establish hardware sessions;
//! - select execution targets;
//! - guarantee future device availability;
//! - guarantee execution feasibility.
//!
//! Discovery results must be treated as observations.
//!
//! # Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - providers;
//! - devices;
//! - discovery pages;
//! - continuation cursors;
//! - hardware categories;
//! - computational domains;
//! - provider implementations.
//!
//! Discovery is incremental. One page is returned at a time.
//!
//! Internal validation state grows according to the observations
//! processed and available memory.
//!
//! No finite implementation promises literal infinite storage
//! or computation.
//!
//! # Determinism
//!
//! Providers are ordered by canonical ProviderId.
//!
//! Providers are visited using deterministic round-robin progression.
//!
//! Devices within each returned page are ordered by their canonical
//! HardwareDeviceIdentity.
//!
//! Determinism assumes that each provider implements deterministic
//! pagination for an unchanged discovery snapshot.
//!
//! # Failure isolation
//!
//! Strict:
//!     Stop discovery when a provider fails.
//!
//! BestEffort:
//!     Record the provider failure and continue with other providers.
//!
//! Cancellation and caller deadline expiration always propagate.
//!
//! Provider failures are never silently converted into successful
//! discovery.
//!
//! # Security
//!
//! This module does not authenticate providers or authorize hardware.
//!
//! Discovery cursors are opaque provider-owned values.
//!
//! Credentials must not be placed in discovery metadata or cursors.
//!
//! # Grammar boundary
//!
//! Source-level hardware intent remains owned by the grammar,
//! frontend and semantic model.
//!
//! This module consumes already-established hardware providers.
//!
//! It does not parse grammar or reinterpret requirements,
//! capabilities, preferences, constraints or hints.

use core::num::NonZeroUsize;

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::stdlib::provider::ProviderId;

use crate::zhal::context::ZhalContext;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

use crate::zhal::identity::HardwareDeviceIdentity;

use crate::zhal::model::device::HardwareDeviceDescriptor;

use crate::zhal::providers::contract::{
    DeviceDiscoveryCursor,
    DeviceDiscoveryPage,
    DeviceDiscoveryRequest,
    HardwareProvider,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable discovery component identifier.
pub const COMPONENT: &str = "zhal.providers.discovery";

// ============================================================================
// FAILURE POLICY
// ============================================================================

/// Controls how provider-specific discovery failures are handled.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DiscoveryFailurePolicy {
    /// Stop discovery when any provider fails.
    ///
    /// The partial discovery report remains accessible through
    /// DiscoverySession::report().
    Strict,

    /// Record provider failures and continue discovering
    /// from the remaining providers.
    ///
    /// A partial report must not be interpreted as a complete
    /// hardware inventory.
    #[default]
    BestEffort,
}

// ============================================================================
// DISCOVERY OPTIONS
// ============================================================================

/// Immutable configuration for one discovery session.
///
/// Page size is an operational request, not a universal hardware
/// or language-level limitation.
///
/// When omitted, each provider chooses its own configured
/// discovery page size.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DiscoveryOptions {
    page_size: Option<NonZeroUsize>,
    failure_policy: DiscoveryFailurePolicy,
}

impl DiscoveryOptions {
    /// Creates default discovery options.
    ///
    /// Defaults:
    ///
    /// - Provider-defined page size.
    /// - Best-effort provider failure handling.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets an optional requested discovery page size.
    ///
    /// This does not impose a global hardware limit.
    ///
    /// Providers may return fewer devices than requested.
    #[must_use]
    pub const fn with_page_size(
        mut self,
        page_size: Option<NonZeroUsize>,
    ) -> Self {
        self.page_size = page_size;
        self
    }

    /// Sets the provider failure policy.
    #[must_use]
    pub const fn with_failure_policy(
        mut self,
        policy: DiscoveryFailurePolicy,
    ) -> Self {
        self.failure_policy = policy;
        self
    }

    /// Returns the optional requested page size.
    #[must_use]
    pub const fn page_size(&self) -> Option<NonZeroUsize> {
        self.page_size
    }

    /// Returns the configured failure policy.
    #[must_use]
    pub const fn failure_policy(&self) -> DiscoveryFailurePolicy {
        self.failure_policy
    }
}

// ============================================================================
// FAILURE PHASE
// ============================================================================

/// Identifies the discovery stage where a provider failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiscoveryFailurePhase {
    /// The provider did not report an operational status.
    ProviderReadiness,

    /// The provider failed while returning a discovery page.
    PageDiscovery,

    /// The provider returned an invalid continuation sequence.
    PaginationValidation,

    /// The provider returned a duplicate device across pages.
    DeviceValidation,

    /// Discovery accounting could not be updated safely.
    Accounting,
}

// ============================================================================
// PROVIDER FAILURE
// ============================================================================

/// A provider-specific discovery failure.
///
/// The original ZHAL error is preserved rather than discarded.
#[derive(Debug)]
pub struct DiscoveryFailure {
    provider_id: ProviderId,
    phase: DiscoveryFailurePhase,
    error: ZhalError,
}

impl DiscoveryFailure {
    fn new(
        provider_id: ProviderId,
        phase: DiscoveryFailurePhase,
        error: ZhalError,
    ) -> Self {
        Self {
            provider_id,
            phase,
            error,
        }
    }

    /// Returns the provider that failed.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the discovery phase.
    #[must_use]
    pub const fn phase(&self) -> DiscoveryFailurePhase {
        self.phase
    }

    /// Returns the original structured provider error.
    #[must_use]
    pub fn error(&self) -> &ZhalError {
        &self.error
    }
}

// ============================================================================
// DISCOVERY REPORT
// ============================================================================

/// Incremental summary of a discovery session.
///
/// Device descriptors are delivered separately through
/// ProviderDiscoveryPage.
///
/// The report does not accumulate the entire hardware inventory.
///
/// This allows callers to process large inventories incrementally.
#[derive(Debug, Default)]
pub struct DiscoveryReport {
    provider_count: usize,
    providers_completed: usize,
    providers_failed: usize,
    pages_observed: usize,
    devices_observed: usize,
    failures: Vec<DiscoveryFailure>,
}

impl DiscoveryReport {
    /// Returns the number of providers included in the session.
    #[must_use]
    pub const fn provider_count(&self) -> usize {
        self.provider_count
    }

    /// Returns the number of providers whose discovery completed.
    #[must_use]
    pub const fn providers_completed(&self) -> usize {
        self.providers_completed
    }

    /// Returns the number of providers that failed.
    #[must_use]
    pub const fn providers_failed(&self) -> usize {
        self.providers_failed
    }

    /// Returns the number of successfully observed pages.
    #[must_use]
    pub const fn pages_observed(&self) -> usize {
        self.pages_observed
    }

    /// Returns the total number of device observations delivered.
    #[must_use]
    pub const fn devices_observed(&self) -> usize {
        self.devices_observed
    }

    /// Returns all recorded provider failures.
    ///
    /// Errors preserve their original structured information.
    #[must_use]
    pub fn failures(&self) -> &[DiscoveryFailure] {
        &self.failures
    }

    /// Returns whether any provider failed.
    #[must_use]
    pub fn has_failures(&self) -> bool {
        !self.failures.is_empty()
    }

    /// Returns whether every provider completed successfully.
    ///
    /// A report with zero providers is considered successfully complete.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.providers_completed == self.provider_count
            && self.providers_failed == 0
    }
}

// ============================================================================
// PROVIDER DISCOVERY PAGE
// ============================================================================

/// One validated and deterministically ordered provider discovery page.
///
/// The page is returned to the caller immediately rather than
/// accumulated into a global hardware inventory.
#[derive(Clone, Debug)]
pub struct ProviderDiscoveryPage {
    provider_id: ProviderId,
    page_index: usize,
    devices: Vec<HardwareDeviceDescriptor>,
    next_cursor: Option<DeviceDiscoveryCursor>,
    snapshot_revision: Option<String>,
}

impl ProviderDiscoveryPage {
    /// Returns the provider that produced this page.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the zero-based page position for this provider.
    #[must_use]
    pub const fn page_index(&self) -> usize {
        self.page_index
    }

    /// Returns the deterministically ordered device descriptors.
    #[must_use]
    pub fn devices(&self) -> &[HardwareDeviceDescriptor] {
        &self.devices
    }

    /// Returns the provider's continuation cursor.
    #[must_use]
    pub fn next_cursor(&self) -> Option<&DeviceDiscoveryCursor> {
        self.next_cursor.as_ref()
    }

    /// Returns the provider's optional discovery snapshot revision.
    ///
    /// This is provider-supplied provenance, not an independently
    /// authenticated version.
    #[must_use]
    pub fn snapshot_revision(&self) -> Option<&str> {
        self.snapshot_revision.as_deref()
    }

    /// Returns whether this provider has completed its current
    /// paginated discovery operation.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.next_cursor.is_none()
    }
}

// ============================================================================
// INTERNAL PROVIDER STATE
// ============================================================================

struct ProviderState {
    provider_id: ProviderId,
    implementation: Arc<dyn HardwareProvider>,

    cursor: Option<DeviceDiscoveryCursor>,

    seen_cursors: BTreeSet<DeviceDiscoveryCursor>,

    seen_devices: BTreeSet<HardwareDeviceIdentity>,

    page_index: usize,

    completed: bool,
    failed: bool,
}

// ============================================================================
// DISCOVERY SESSION
// ============================================================================

/// Incremental coordinator for multiple hardware providers.
///
/// A session retains provider handles so that removal from a registry
/// does not invalidate an already-started discovery operation.
///
/// The session does not initialize or shut down providers.
///
/// Provider lifecycle remains owned by the lifecycle manager.
///
/// Providers are processed in deterministic round-robin order.
///
/// Each call to next_page returns at most one provider page.
pub struct DiscoverySession {
    providers: Vec<ProviderState>,

    next_provider_index: usize,

    options: DiscoveryOptions,

    report: DiscoveryReport,

    terminated: bool,
}

impl DiscoverySession {
    /// Creates a discovery session from hardware-provider handles.
    ///
    /// Providers are sorted by canonical ProviderId.
    ///
    /// Duplicate provider identities are rejected.
    ///
    /// The caller is responsible for obtaining the provider handles
    /// from the canonical provider registry.
    ///
    /// # Errors
    ///
    /// Returns an error when two implementations claim the same
    /// canonical provider identity.
    pub fn new(
        providers: impl IntoIterator<Item = Arc<dyn HardwareProvider>>,
        options: DiscoveryOptions,
    ) -> ZhalResult<Self> {
        let mut entries = Vec::new();

        for implementation in providers {
            let provider_id =
                implementation.descriptor().id().clone();

            entries.push(ProviderState {
                provider_id,
                implementation,
                cursor: None,
                seen_cursors: BTreeSet::new(),
                seen_devices: BTreeSet::new(),
                page_index: 0,
                completed: false,
                failed: false,
            });
        }

        entries.sort_by(|left, right| {
            left.provider_id.cmp(&right.provider_id)
        });

        let mut identities = BTreeSet::new();

        for entry in &entries {
            if !identities.insert(entry.provider_id.clone()) {
                return Err(
                    discovery_error(
                        ZhalErrorKind::ContractViolation,
                        format!(
                            "multiple hardware-provider implementations claim identity {}",
                            entry.provider_id
                        ),
                    )
                    .with_provider(entry.provider_id.clone()),
                );
            }
        }

        let provider_count = entries.len();

        Ok(Self {
            providers: entries,
            next_provider_index: 0,
            options,
            report: DiscoveryReport {
                provider_count,
                ..DiscoveryReport::default()
            },
            terminated: false,
        })
    }

    /// Returns the current discovery report.
    ///
    /// The report remains available after strict-policy failure.
    #[must_use]
    pub fn report(&self) -> &DiscoveryReport {
        &self.report
    }

    /// Returns the discovery configuration.
    #[must_use]
    pub fn options(&self) -> &DiscoveryOptions {
        &self.options
    }

    /// Returns whether discovery has reached a terminal state
    /// for every provider.
    ///
    /// A strict-policy early termination does not count as complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.providers.iter().all(|provider| {
            provider.completed || provider.failed
        }) && !self.terminated
    }

    /// Returns whether strict failure handling terminated the session.
    #[must_use]
    pub const fn is_terminated(&self) -> bool {
        self.terminated
    }

    /// Consumes the session and returns its final partial or complete report.
    #[must_use]
    pub fn into_report(self) -> DiscoveryReport {
        self.report
    }

    /// Retrieves the next validated discovery page.
    ///
    /// At most one page is returned per call.
    ///
    /// Providers are visited in deterministic round-robin order.
    ///
    /// In best-effort mode, an individual provider failure is recorded
    /// and discovery continues with other providers.
    ///
    /// In strict mode, the first provider failure terminates the session.
    ///
    /// Cancellation and caller deadline expiration always propagate,
    /// independently of the failure policy.
    ///
    /// # Errors
    ///
    /// Returns an error for:
    ///
    /// - cancellation;
    /// - deadline expiration;
    /// - strict-policy provider failure;
    /// - discovery accounting failure;
    /// - internal discovery coordination failure.
    ///
    /// Best-effort provider failures are available through report().
    pub async fn next_page(
        &mut self,
        context: &ZhalContext,
    ) -> ZhalResult<Option<ProviderDiscoveryPage>> {
        if self.terminated {
            return Ok(None);
        }

        check_context(context)?;

        let provider_count = self.providers.len();

        if provider_count == 0 {
            return Ok(None);
        }

        let mut visited = 0;

        while visited < provider_count {
            let index = self.next_provider_index;

            self.next_provider_index =
                (self.next_provider_index + 1) % provider_count;

            visited += 1;

            if self.providers[index].completed
                || self.providers[index].failed
            {
                continue;
            }

            let provider = Arc::clone(
                &self.providers[index].implementation,
            );

            let provider_id =
                self.providers[index].provider_id.clone();

            if !provider.status().may_accept_operations() {
                let error = discovery_error(
                    ZhalErrorKind::ProviderUnavailable,
                    format!(
                        "hardware provider {} is not currently accepting operations",
                        provider_id
                    ),
                )
                .with_provider(provider_id.clone());

                self.handle_provider_failure(
                    index,
                    DiscoveryFailurePhase::ProviderReadiness,
                    error,
                )?;

                if self.terminated {
                    return Err(discovery_error(
                        ZhalErrorKind::ProviderUnavailable,
                        format!(
                            "strict discovery terminated because provider {} is unavailable",
                            provider_id
                        ),
                    )
                    .with_provider(provider_id));
                }

                continue;
            }

            let request = DeviceDiscoveryRequest::new(
                self.providers[index].cursor.clone(),
                self.options.page_size,
            );

            check_context(context)?;

            let page_result = provider
                .discover_devices(&request, context)
                .await;

            check_context(context)?;

            let page = match page_result {
                Ok(page) => page,

                Err(error) => {
                    self.handle_provider_failure(
                        index,
                        DiscoveryFailurePhase::PageDiscovery,
                        error,
                    )?;

                    if self.terminated {
                        return Err(discovery_error(
                            ZhalErrorKind::DiscoveryFailed,
                            format!(
                                "strict discovery terminated after provider {} failed",
                                provider_id
                            ),
                        )
                        .with_provider(provider_id));
                    }

                    continue;
                }
            };

            // ------------------------------------------------------------
            // Continuation validation
            // ------------------------------------------------------------

            if let Some(next_cursor) = page.next_cursor() {
                if self.providers[index]
                    .seen_cursors
                    .contains(next_cursor)
                {
                    let error = discovery_error(
                        ZhalErrorKind::ContractViolation,
                        "provider returned a previously observed discovery cursor",
                    )
                    .with_provider(provider_id.clone());

                    self.handle_provider_failure(
                        index,
                        DiscoveryFailurePhase::PaginationValidation,
                        error,
                    )?;

                    if self.terminated {
                        return Err(discovery_error(
                            ZhalErrorKind::ContractViolation,
                            "strict discovery terminated after a pagination cycle",
                        )
                        .with_provider(provider_id));
                    }

                    continue;
                }
            }

            // ------------------------------------------------------------
            // Cross-page device identity validation
            // ------------------------------------------------------------

            let mut page_identities =
                BTreeSet::new();

            let mut invalid_duplicate = false;

            for device in page.devices() {
                let identity = device.identity().clone();

                if !page_identities.insert(identity.clone())
                    || self.providers[index]
                        .seen_devices
                        .contains(&identity)
                {
                    invalid_duplicate = true;
                    break;
                }
            }

            if invalid_duplicate {
                let error = discovery_error(
                    ZhalErrorKind::ContractViolation,
                    "provider returned a duplicate hardware device across discovery pages",
                )
                .with_provider(provider_id.clone());

                self.handle_provider_failure(
                    index,
                    DiscoveryFailurePhase::DeviceValidation,
                    error,
                )?;

                if self.terminated {
                    return Err(discovery_error(
                        ZhalErrorKind::ContractViolation,
                        "strict discovery terminated after duplicate device detection",
                    )
                    .with_provider(provider_id));
                }

                continue;
            }

            // ------------------------------------------------------------
            // Accounting validation
            // ------------------------------------------------------------

            let next_page_index =
                self.providers[index].page_index.checked_add(1);

            let next_pages_observed =
                self.report.pages_observed.checked_add(1);

            let next_devices_observed =
                self.report.devices_observed
                    .checked_add(page.devices().len());

            if next_page_index.is_none()
                || next_pages_observed.is_none()
                || next_devices_observed.is_none()
            {
                let error = discovery_error(
                    ZhalErrorKind::Internal,
                    "discovery accounting exceeded the representable counter range",
                )
                .with_provider(provider_id.clone());

                self.handle_provider_failure(
                    index,
                    DiscoveryFailurePhase::Accounting,
                    error,
                )?;

                if self.terminated {
                    return Err(discovery_error(
                        ZhalErrorKind::Internal,
                        "strict discovery terminated after an accounting failure",
                    )
                    .with_provider(provider_id));
                }

                continue;
            }

            // ------------------------------------------------------------
            // Commit validated observations
            // ------------------------------------------------------------

            for identity in page_identities {
                self.providers[index]
                    .seen_devices
                    .insert(identity);
            }

            if let Some(next_cursor) = page.next_cursor() {
                self.providers[index]
                    .seen_cursors
                    .insert(next_cursor.clone());
            }

            self.providers[index].cursor =
                page.next_cursor().cloned();

            self.providers[index].page_index =
                next_page_index.expect("checked above");

            self.report.pages_observed =
                next_pages_observed.expect("checked above");

            self.report.devices_observed =
                next_devices_observed.expect("checked above");

            let complete = page.is_complete();

            if complete {
                self.providers[index].completed = true;

                self.report.providers_completed =
                    self.report.providers_completed
                        .checked_add(1)
                        .ok_or_else(|| {
                            discovery_error(
                                ZhalErrorKind::Internal,
                                "provider completion counter exhausted",
                            )
                        })?;
            }

            // ------------------------------------------------------------
            // Deterministic page ordering
            // ------------------------------------------------------------

            let mut devices =
                page.devices().to_vec();

            devices.sort_by(|left, right| {
                left.identity().cmp(right.identity())
            });

            return Ok(Some(ProviderDiscoveryPage {
                provider_id,
                page_index: next_page_index
                    .expect("checked above") - 1,
                devices,
                next_cursor: page.next_cursor().cloned(),
                snapshot_revision:
                    page.snapshot_revision().map(str::to_owned),
            }));
        }

        Ok(None)
    }

    // --------------------------------------------------------------------
    // FAILURE HANDLING
    // --------------------------------------------------------------------

    fn handle_provider_failure(
        &mut self,
        index: usize,
        phase: DiscoveryFailurePhase,
        error: ZhalError,
    ) -> ZhalResult<()> {
        let provider_id =
            self.providers[index].provider_id.clone();

        let error = if error.provider_id().is_none() {
            error.with_provider(provider_id.clone())
        } else {
            error
        };

        self.providers[index].failed = true;

        self.report.providers_failed =
            self.report.providers_failed
                .checked_add(1)
                .ok_or_else(|| {
                    discovery_error(
                        ZhalErrorKind::Internal,
                        "provider failure counter exhausted",
                    )
                })?;

        self.report.failures.push(
            DiscoveryFailure::new(
                provider_id,
                phase,
                error,
            ),
        );

        if self.options.failure_policy
            == DiscoveryFailurePolicy::Strict
        {
            self.terminated = true;
        }

        Ok(())
    }
}

// ============================================================================
// CONTEXT VALIDATION
// ============================================================================

/// Checks caller-owned cancellation and deadline observations.
///
/// No second cancellation or deadline mechanism is introduced.
fn check_context(
    context: &ZhalContext,
) -> ZhalResult<()> {
    if context.is_cancelled() {
        return Err(
            discovery_error(
                ZhalErrorKind::Cancelled,
                "hardware discovery was cancelled",
            ),
        );
    }

    if context.is_deadline_expired() {
        return Err(
            discovery_error(
                ZhalErrorKind::OperationTimedOut,
                "hardware discovery exceeded its caller-supplied deadline",
            ),
        );
    }

    Ok(())
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn discovery_error(
    kind: ZhalErrorKind,
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(kind, message)
        .in_component(COMPONENT)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_options_are_unbounded_by_page_size() {
        let options = DiscoveryOptions::new();

        assert!(options.page_size().is_none());

        assert_eq!(
            options.failure_policy(),
            DiscoveryFailurePolicy::BestEffort,
        );
    }

    #[test]
    fn caller_can_configure_page_size() {
        let size = NonZeroUsize::new(4096)
            .expect("nonzero page size");

        let options = DiscoveryOptions::new()
            .with_page_size(Some(size));

        assert_eq!(
            options.page_size(),
            Some(size),
        );
    }

    #[test]
    fn caller_can_select_strict_failure_handling() {
        let options = DiscoveryOptions::new()
            .with_failure_policy(
                DiscoveryFailurePolicy::Strict,
            );

        assert_eq!(
            options.failure_policy(),
            DiscoveryFailurePolicy::Strict,
        );
    }

    #[test]
    fn empty_provider_collection_is_valid() {
        let session = DiscoverySession::new(
            Vec::<Arc<dyn HardwareProvider>>::new(),
            DiscoveryOptions::default(),
        )
        .expect("empty provider collection is valid");

        assert_eq!(
            session.report().provider_count(),
            0,
        );

        assert!(session.is_complete());
        assert!(session.report().is_successful());
    }
}
