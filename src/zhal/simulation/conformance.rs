
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # ZHAL Simulation Provider Conformance
//!
//! Path: `src/zhal/simulation/conformance.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//!
//! Provides a reusable, asynchronous conformance runner for implementations
//! of the canonical ZHAL `HardwareProvider` contract.
//!
//! The runner is intended primarily for simulated providers and provider
//! development. Session testing is configurable because opening a session
//! against a non-simulated provider may access real hardware.
//!
//! ## Architectural ownership
//!
//! Reuses:
//!
//! - `crate::zhal::providers::contract::HardwareProvider`
//! - `crate::zhal::providers::contract::DeviceDiscoveryRequest`
//! - `crate::zhal::context::ZhalContext`
//! - `crate::stdlib::provider::Provider`
//!
//! Does not own:
//!
//! - Provider registration or lifecycle management.
//! - Provider identity or device identity.
//! - Capability semantics or feasibility evaluation.
//! - Resource allocation or reservation.
//! - Domain-specific execution semantics.
//! - Physical hardware access.
//! - An asynchronous executor.
//!
//! ## Conformance guarantees
//!
//! The runner checks:
//!
//! - Provider readiness before probing.
//! - Capability snapshot observation.
//! - Paginated discovery and cursor progress.
//! - Provider-scoped device identities.
//! - Duplicate identities across discovery pages.
//! - Validated device inspection.
//! - Optional session creation, association and closure.
//! - Cancellation and deadline observation between operations.
//!
//! A successful report means only that the checks enabled by the supplied
//! options passed for the observations made during this run. It is not a
//! guarantee of future availability, physical execution, authorization,
//! resource reservation or universal POCO-REAF feasibility.
//!
//! ## Scalability
//!
//! No fixed maximum is imposed on providers, devices, pages, identifiers,
//! hardware categories or computational domains.
//!
//! Optional page and device budgets are caller-selected operational budgets.
//! With no budgets, the runner continues until discovery completes or a
//! provider/context error occurs. Duplicate and cursor-cycle detection
//! requires memory proportional to the identities and cursors observed.
//! The caller can use explicit budgets when that memory cost is unsuitable.
//!
//! Literal infinite inventories cannot be completely enumerated or tested.
//!
//! ## Grammar boundary
//!
//! Simulation and resource grammars describe source-level intent. This
//! module tests runtime provider contracts; it does not parse grammar,
//! interpret requirements, select devices or allocate resources.

use core::num::NonZeroUsize;

use std::collections::BTreeSet;

use crate::stdlib::provider::Provider;
use crate::zhal::context::ZhalContext;
use crate::zhal::identity::HardwareDeviceIdentity;
use crate::zhal::providers::contract::{
    DeviceDiscoveryCursor,
    DeviceDiscoveryRequest,
    HardwareProvider,
};

// ============================================================================
// OPTIONS
// ============================================================================

/// Operational limits and optional checks for one conformance run.
///
/// `None` budgets mean that the runner does not impose a caller-side limit.
/// They do not override limits imposed by the host, provider, operating
/// system, available memory or other explicit operational policies.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConformanceOptions {
    page_size: NonZeroUsize,
    max_pages: Option<NonZeroUsize>,
    max_devices: Option<NonZeroUsize>,
    inspect_devices: bool,
    exercise_sessions: bool,
}

impl ConformanceOptions {
    /// Creates conformance options.
    ///
    /// `page_size` is a requested operational page size, not a hardware
    /// capacity limit. Providers may return fewer devices.
    ///
    /// Session testing is disabled by default because opening a session
    /// against a physical provider can have external effects.
    #[must_use]
    pub const fn new(page_size: NonZeroUsize) -> Self {
        Self {
            page_size,
            max_pages: None,
            max_devices: None,
            inspect_devices: true,
            exercise_sessions: false,
        }
    }

    /// Sets an optional maximum number of discovery pages to examine.
    ///
    /// Reaching this budget while discovery remains incomplete produces
    /// an `Incomplete` report, not a false pass or an automatic failure.
    #[must_use]
    pub const fn with_max_pages(
        mut self,
        max_pages: Option<NonZeroUsize>,
    ) -> Self {
        self.max_pages = max_pages;
        self
    }

    /// Sets an optional maximum number of devices to examine.
    ///
    /// This is a conformance-run budget, not a supported-device ceiling.
    #[must_use]
    pub const fn with_max_devices(
        mut self,
        max_devices: Option<NonZeroUsize>,
    ) -> Self {
        self.max_devices = max_devices;
        self
    }

    /// Enables or disables validated device inspection.
    #[must_use]
    pub const fn with_device_inspection(
        mut self,
        enabled: bool,
    ) -> Self {
        self.inspect_devices = enabled;
        self
    }

    /// Enables or disables provider-managed session testing.
    ///
    /// Enable this by default only for providers whose session creation
    /// and closure are safe to exercise in the test environment.
    #[must_use]
    pub const fn with_session_testing(
        mut self,
        enabled: bool,
    ) -> Self {
        self.exercise_sessions = enabled;
        self
    }

    /// Returns the requested discovery page size.
    #[must_use]
    pub const fn page_size(&self) -> NonZeroUsize {
        self.page_size
    }

    /// Returns the optional page budget.
    #[must_use]
    pub const fn max_pages(&self) -> Option<NonZeroUsize> {
        self.max_pages
    }

    /// Returns the optional device budget.
    #[must_use]
    pub const fn max_devices(&self) -> Option<NonZeroUsize> {
        self.max_devices
    }

    /// Returns whether device inspection is enabled.
    #[must_use]
    pub const fn inspects_devices(&self) -> bool {
        self.inspect_devices
    }

    /// Returns whether session testing is enabled.
    #[must_use]
    pub const fn exercises_sessions(&self) -> bool {
        self.exercise_sessions
    }
}

// ============================================================================
// REPORT MODEL
// ============================================================================

/// Result of one individual conformance check.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConformanceCheckStatus {
    /// The check completed and its contract was satisfied.
    Passed,

    /// The check completed and identified a contract violation or failure.
    Failed,

    /// The check could not be completed within the configured scope.
    Incomplete,

    /// The check was deliberately disabled or did not apply.
    Skipped,
}

/// Overall conclusion of a conformance run.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConformanceStatus {
    /// Every applicable check completed successfully.
    Passed,

    /// At least one check failed.
    Failed,

    /// No check failed, but at least one check was incomplete or skipped.
    Incomplete,
}

/// One structured conformance finding.
///
/// The diagnostic is descriptive and must not contain credentials,
/// access tokens, private keys or other secrets.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConformanceFinding {
    check: &'static str,
    status: ConformanceCheckStatus,
    diagnostic: String,
}

impl ConformanceFinding {
    fn new(
        check: &'static str,
        status: ConformanceCheckStatus,
        diagnostic: impl Into<String>,
    ) -> Self {
        Self {
            check,
            status,
            diagnostic: diagnostic.into(),
        }
    }

    /// Returns the stable check identifier.
    #[must_use]
    pub const fn check(&self) -> &'static str {
        self.check
    }

    /// Returns the check result.
    #[must_use]
    pub const fn status(&self) -> ConformanceCheckStatus {
        self.status
    }

    /// Returns the diagnostic associated with the finding.
    #[must_use]
    pub fn diagnostic(&self) -> &str {
        &self.diagnostic
    }
}

/// Aggregate report for one provider conformance run.
///
/// Counts describe work actually completed by this run. They are not
/// declarations of provider capacity or supported hardware limits.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConformanceReport {
    provider_id: String,
    provider_status: String,
    pages_examined: usize,
    devices_examined: usize,
    findings: Vec<ConformanceFinding>,
}

impl ConformanceReport {
    fn new(
        provider_id: String,
        provider_status: String,
    ) -> Self {
        Self {
            provider_id,
            provider_status,
            pages_examined: 0,
            devices_examined: 0,
            findings: Vec::new(),
        }
    }

    fn record(
        &mut self,
        check: &'static str,
        status: ConformanceCheckStatus,
        diagnostic: impl Into<String>,
    ) {
        self.findings.push(ConformanceFinding::new(
            check,
            status,
            diagnostic,
        ));
    }

    /// Returns the tested provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    /// Returns the provider status observed when the run began.
    #[must_use]
    pub fn provider_status(&self) -> &str {
        &self.provider_status
    }

    /// Returns the number of discovery pages examined.
    #[must_use]
    pub const fn pages_examined(&self) -> usize {
        self.pages_examined
    }

    /// Returns the number of devices examined.
    #[must_use]
    pub const fn devices_examined(&self) -> usize {
        self.devices_examined
    }

    /// Returns all findings in execution order.
    #[must_use]
    pub fn findings(&self) -> &[ConformanceFinding] {
        &self.findings
    }

    /// Returns the number of findings with the requested status.
    #[must_use]
    pub fn count(
        &self,
        status: ConformanceCheckStatus,
    ) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.status() == status)
            .count()
    }

    /// Derives the overall conformance conclusion.
    ///
    /// Failure takes precedence over incompleteness. A skipped check does
    /// not become a pass.
    #[must_use]
    pub fn status(&self) -> ConformanceStatus {
        if self.count(ConformanceCheckStatus::Failed) > 0 {
            ConformanceStatus::Failed
        } else if self.count(ConformanceCheckStatus::Incomplete) > 0
            || self.count(ConformanceCheckStatus::Skipped) > 0
        {
            ConformanceStatus::Incomplete
        } else {
            ConformanceStatus::Passed
        }
    }

    /// Returns whether all enabled checks completed successfully.
    #[must_use]
    pub fn is_passed(&self) -> bool {
        self.status() == ConformanceStatus::Passed
    }
}

// ============================================================================
// CONFORMANCE RUNNER
// ============================================================================

/// Runs the common ZHAL hardware-provider conformance checks.
///
/// The runner does not initialize or shut down the provider. The caller
/// must use the canonical provider lifecycle manager and supply a context
/// valid for the duration of the run.
///
/// A report is returned even when an individual provider check fails.
/// Provider errors are retained as descriptive diagnostics in the report.
/// The runner does not convert a failed check into a successful result.
///
/// # Checks
///
/// 1. Provider readiness.
/// 2. Capability snapshot observation.
/// 3. Paginated discovery and cursor progress.
/// 4. Provider-scoped and globally unique discovered identities.
/// 5. Optional validated inspection.
/// 6. Optional session association and closure.
///
/// # Resource use
///
/// When no budgets are configured, the runner examines all pages until
/// the provider declares discovery complete. It retains discovered
/// identities and cursors to detect duplicates and cursor cycles.
/// Those sets grow with the observed inventory. Configure explicit
/// budgets when the test environment requires bounded memory or time.
///
/// # Errors
///
/// This function returns a report for provider and context failures.
/// It does not panic for ordinary provider errors.
pub async fn run_provider_conformance(
    provider: &dyn HardwareProvider,
    context: &ZhalContext,
    options: &ConformanceOptions,
) -> ConformanceReport {
    let descriptor = provider.descriptor();
    let provider_id = descriptor.id().to_string();
    let observed_status = provider.status();

    let mut report = ConformanceReport::new(
        provider_id,
        observed_status.to_string(),
    );

    if let Err(error) = context.check_active() {
        report.record(
            "context.active",
            ConformanceCheckStatus::Failed,
            format!("operation context is not active: {error}"),
        );
        return report;
    }

    if !observed_status.may_accept_operations() {
        report.record(
            "provider.readiness",
            ConformanceCheckStatus::Failed,
            format!(
                "provider status '{observed_status}' does not indicate readiness"
            ),
        );
        return report;
    }

    report.record(
        "provider.readiness",
        ConformanceCheckStatus::Passed,
        "provider status indicates that operations may be accepted",
    );

    // Capability observation is deliberately observational. The suite
    // verifies that the provider can supply its canonical snapshot; it
    // does not reinterpret capability semantics or infer feasibility.
    match provider
        .capability_snapshot(context.operation_context())
        .await
    {
        Ok(_) => report.record(
            "provider.capability_snapshot",
            ConformanceCheckStatus::Passed,
            "provider returned a capability snapshot",
        ),
        Err(error) => {
            report.record(
                "provider.capability_snapshot",
                ConformanceCheckStatus::Failed,
                format!("capability observation failed: {error}"),
            );
            return report;
        }
    }

    let mut next_cursor: Option<DeviceDiscoveryCursor> = None;
    let mut seen_cursors = BTreeSet::new();
    let mut seen_devices: BTreeSet<HardwareDeviceIdentity> =
        BTreeSet::new();

    loop {
        if let Err(error) = context.check_active() {
            report.record(
                "discovery.context",
                ConformanceCheckStatus::Failed,
                format!("context became inactive during discovery: {error}"),
            );
            return report;
        }

        if let Some(limit) = options.max_pages {
            if report.pages_examined >= limit.get() {
                report.record(
                    "discovery.page_budget",
                    ConformanceCheckStatus::Incomplete,
                    "configured page budget was reached before discovery completed",
                );
                return report;
            }
        }

        let request = DeviceDiscoveryRequest::new(
            next_cursor.clone(),
            Some(options.page_size),
        );

        let page = match provider
            .discover_devices(&request, context)
            .await
        {
            Ok(page) => page,
            Err(error) => {
                report.record(
                    "discovery.page",
                    ConformanceCheckStatus::Failed,
                    format!("validated discovery failed: {error}"),
                );
                return report;
            }
        };

        report.pages_examined = match report.pages_examined.checked_add(1) {
            Some(value) => value,
            None => {
                report.record(
                    "discovery.page_count",
                    ConformanceCheckStatus::Failed,
                    "page counter exceeded the platform's usize representation",
                );
                return report;
            }
        };

        let mut stop_after_page = false;

        for device in page.devices() {
            if let Some(limit) = options.max_devices {
                if report.devices_examined >= limit.get() {
                    report.record(
                        "discovery.device_budget",
                        ConformanceCheckStatus::Incomplete,
                        "configured device budget was reached before discovery completed",
                    );
                    stop_after_page = true;
                    break;
                }
            }

            let identity = device.identity().clone();

            if identity.provider_id() != descriptor.id() {
                report.record(
                    "discovery.provider_scope",
                    ConformanceCheckStatus::Failed,
                    format!(
                        "discovered device '{identity}' is not owned by the tested provider"
                    ),
                );
                return report;
            }

            if !seen_devices.insert(identity.clone()) {
                report.record(
                    "discovery.unique_identity",
                    ConformanceCheckStatus::Failed,
                    format!(
                        "device identity '{identity}' was repeated across discovery pages"
                    ),
                );
                return report;
            }

            report.devices_examined = match report.devices_examined.checked_add(1) {
                Some(value) => value,
                None => {
                    report.record(
                        "discovery.device_count",
                        ConformanceCheckStatus::Failed,
                        "device counter exceeded the platform's usize representation",
                    );
                    return report;
                }
            };

            if options.inspect_devices {
                if let Err(error) = context.check_active() {
                    report.record(
                        "inspection.context",
                        ConformanceCheckStatus::Failed,
                        format!("context became inactive before inspection: {error}"),
                    );
                    return report;
                }

                match provider
                    .inspect_device_validated(&identity, context)
                    .await
                {
                    Ok(inspected) if inspected.identity() == &identity => {
                        report.record(
                            "device.inspection",
                            ConformanceCheckStatus::Passed,
                            format!("device '{identity}' was inspected successfully"),
                        );
                    }
                    Ok(_) => {
                        report.record(
                            "device.inspection",
                            ConformanceCheckStatus::Failed,
                            format!(
                                "inspection returned an identity different from '{identity}'"
                            ),
                        );
                        return report;
                    }
                    Err(error) => {
                        report.record(
                            "device.inspection",
                            ConformanceCheckStatus::Failed,
                            format!("inspection of '{identity}' failed: {error}"),
                        );
                        return report;
                    }
                }
            } else {
                report.record(
                    "device.inspection",
                    ConformanceCheckStatus::Skipped,
                    "device inspection was disabled by conformance options",
                );
            }

            if options.exercise_sessions {
                if let Err(error) = context.check_active() {
                    report.record(
                        "session.context",
                        ConformanceCheckStatus::Failed,
                        format!("context became inactive before session creation: {error}"),
                    );
                    return report;
                }

                let mut session = match provider
                    .open_session(&identity, context)
                    .await
                {
                    Ok(session) => session,
                    Err(error) => {
                        report.record(
                            "device.session",
                            ConformanceCheckStatus::Failed,
                            format!(
                                "session creation for '{identity}' failed: {error}"
                            ),
                        );
                        return report;
                    }
                };

                if session.device_identity() != &identity {
                    let close_result = session.close(context).await;
                    report.record(
                        "device.session",
                        ConformanceCheckStatus::Failed,
                        format!(
                            "provider associated the session with the wrong device; cleanup result: {}",
                            match close_result {
                                Ok(()) => "closed".to_owned(),
                                Err(error) => format!("close failed: {error}"),
                            }
                        ),
                    );
                    return report;
                }

                if session.is_closed() {
                    report.record(
                        "device.session",
                        ConformanceCheckStatus::Failed,
                        format!(
                            "provider returned an already-closed session for '{identity}'"
                        ),
                    );
                    return report;
                }

                match session.close(context).await {
                    Ok(()) if session.is_closed() => {
                        report.record(
                            "device.session",
                            ConformanceCheckStatus::Passed,
                            format!(
                                "session for '{identity}' was associated and closed successfully"
                            ),
                        );
                    }
                    Ok(()) => {
                        report.record(
                            "device.session",
                            ConformanceCheckStatus::Failed,
                            format!(
                                "session close returned success but session '{identity}' remained open"
                            ),
                        );
                        return report;
                    }
                    Err(error) => {
                        report.record(
                            "device.session",
                            ConformanceCheckStatus::Failed,
                            format!(
                                "session closure for '{identity}' failed: {error}"
                            ),
                        );
                        return report;
                    }
                }
            } else {
                report.record(
                    "device.session",
                    ConformanceCheckStatus::Skipped,
                    "session testing was disabled by conformance options",
                );
            }
        }

        if stop_after_page {
            return report;
        }

        match page.next_cursor() {
            None => {
                report.record(
                    "discovery.pagination",
                    ConformanceCheckStatus::Passed,
                    "provider completed discovery without another continuation cursor",
                );
                break;
            }
            Some(cursor) => {
                let cursor = cursor.clone();

                if request.cursor() == Some(&cursor)
                    || !seen_cursors.insert(cursor.clone())
                {
                    report.record(
                        "discovery.cursor_progress",
                        ConformanceCheckStatus::Failed,
                        "provider repeated a discovery cursor and pagination cannot make progress",
                    );
                    return report;
                }

                next_cursor = Some(cursor);
            }
        }
    }

    if !options.inspect_devices {
        report.record(
            "device.inspection_scope",
            ConformanceCheckStatus::Skipped,
            "the run did not inspect discovered devices",
        );
    }

    if !options.exercise_sessions {
        report.record(
            "device.session_scope",
            ConformanceCheckStatus::Skipped,
            "the run did not open or close provider-managed sessions",
        );
    }

    report
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_options_do_not_impose_inventory_budgets() {
        let options = ConformanceOptions::new(
            NonZeroUsize::new(17).expect("nonzero test page size"),
        );

        assert_eq!(options.page_size().get(), 17);
        assert_eq!(options.max_pages(), None);
        assert_eq!(options.max_devices(), None);
        assert!(options.inspects_devices());
        assert!(!options.exercises_sessions());
    }

    #[test]
    fn explicit_budgets_are_operational_configuration() {
        let options = ConformanceOptions::new(
            NonZeroUsize::new(3).expect("nonzero test page size"),
        )
        .with_max_pages(NonZeroUsize::new(11))
        .with_max_devices(NonZeroUsize::new(29));

        assert_eq!(options.max_pages().map(NonZeroUsize::get), Some(11));
        assert_eq!(options.max_devices().map(NonZeroUsize::get), Some(29));
    }

    #[test]
    fn report_failure_takes_precedence_over_incompleteness() {
        let mut report = ConformanceReport::new(
            "org.example.simulation".to_owned(),
            "ready".to_owned(),
        );

        report.record(
            "discovery.page_budget",
            ConformanceCheckStatus::Incomplete,
            "budget reached",
        );
        report.record(
            "device.inspection",
            ConformanceCheckStatus::Failed,
            "inspection failed",
        );

        assert_eq!(report.status(), ConformanceStatus::Failed);
        assert!(!report.is_passed());
    }

    #[test]
    fn skipped_checks_do_not_count_as_passed() {
        let mut report = ConformanceReport::new(
            "org.example.simulation".to_owned(),
            "ready".to_owned(),
        );

        report.record(
            "device.session",
            ConformanceCheckStatus::Skipped,
            "disabled",
        );

        assert_eq!(report.status(), ConformanceStatus::Incomplete);
    }

    #[test]
    fn only_completed_checks_can_produce_a_pass() {
        let mut report = ConformanceReport::new(
            "org.example.simulation".to_owned(),
            "ready".to_owned(),
        );

        report.record(
            "provider.readiness",
            ConformanceCheckStatus::Passed,
            "ready",
        );
        report.record(
            "discovery.pagination",
            ConformanceCheckStatus::Passed,
            "complete",
        );

        assert_eq!(report.status(), ConformanceStatus::Passed);
        assert!(report.is_passed());
    }
}
