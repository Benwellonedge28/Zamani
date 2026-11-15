
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Provider-Reported Physical Resource Accounting
//!
//! Path: `src/zhal/resources/accounting.rs`
//!
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.1.
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Records immutable, provider-reported physical resource usage
//! measurements and their supporting evidence.
//!
//! Supports:
//!
//! - Classical computing.
//! - Quantum computing.
//! - HDL and programmable hardware.
//! - GPUs and accelerators.
//! - AI and tensor hardware.
//! - Embedded computing.
//! - Distributed hardware.
//! - Hybrid computing.
//! - Networking.
//! - Storage.
//! - Energy.
//! - Neuromorphic hardware.
//! - Photonic computing.
//! - Future computational technologies.
//!
//! # ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - Provider-reported usage records.
//! - Measurement identities.
//! - Measurement classifications.
//! - Measurement timestamps.
//! - Measurement intervals.
//! - Measurement uncertainty.
//! - Evidence quality.
//! - Partial-report descriptions.
//! - Idempotent observation ingestion.
//! - Immutable accounting snapshots.
//! - Exact aggregation of compatible delta measurements.
//!
//! This module does NOT own:
//!
//! - Resource discovery.
//! - Resource descriptions.
//! - Resource capacity.
//! - Resource availability.
//! - Resource requirements.
//! - Resource feasibility.
//! - Resource reservation.
//! - Resource leases.
//! - Runtime allocation.
//! - Confirmed runtime acquisition accounting.
//! - Confirmed runtime release accounting.
//! - Scheduling.
//! - Placement.
//! - Routing.
//! - Hardware execution.
//! - Memory allocation accounting.
//! - Unit conversion.
//! - Source-language semantics.
//!
//! # CANONICAL AUTHORITIES
//!
//! Physical resource model:
//!     crate::zhal::model::resource
//!
//! Hardware identity:
//!     crate::zhal::identity
//!
//! Physical resource observations:
//!     crate::zhal::resources::observation
//!
//! Hardware errors:
//!     crate::zhal::errors
//!
//! Runtime resource accounting:
//!     crate::zur::resources::accounting
//!
//! Runtime resource leases:
//!     crate::zur::resources::lease
//!
//! Semantic resource definitions:
//!     crate::frontend::semantic::resources
//!
//! # FUNDAMENTAL DISTINCTIONS
//!
//! Provider report != independently verified physical measurement.
//!
//! Measurement != resource capacity.
//!
//! Measurement != resource availability.
//!
//! Measurement != reservation.
//!
//! Reservation != acquisition.
//!
//! Acquisition != execution.
//!
//! Gauge != cumulative counter.
//!
//! Cumulative counter != delta.
//!
//! Unknown != zero.
//!
//! Partial != complete.
//!
//! Estimated != independently verified.
//!
//! # ACCOUNTING POLICY
//!
//! This module preserves provider-reported evidence.
//!
//! It does not fabricate consumption measurements.
//!
//! It does not infer resource consumption from availability,
//! capacity, reservations or execution duration.
//!
//! Only compatible delta measurements may be aggregated.
//!
//! Gauge and cumulative-counter observations remain individual
//! measurements unless an explicitly established domain-specific
//! reconciliation authority interprets them.
//!
//! # SCALABILITY
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Resource categories.
//! - Resource identities.
//! - Measurement identities.
//! - Measurement records.
//! - Providers.
//! - Devices.
//! - Computational domains.
//! - Quantity digit counts.
//!
//! Storage grows according to actual available resources.
//!
//! Exact decimal quantities use the canonical ResourceAmount model.
//!
//! Literal infinite storage or infinite physical resources
//! are not promised.
//!
//! # CONCURRENCY
//!
//! Accounting instances are safely shareable between threads.
//!
//! Cloned handles share the same accounting state.
//!
//! Individual ingestion operations are synchronized.
//!
//! Snapshots observe a consistent state.
//!
//! No provider callbacks execute while the accounting lock is held.
//!
//! # FAILURE POLICY
//!
//! Invalid reports are rejected.
//!
//! Conflicting duplicate identities are rejected.
//!
//! Different units are never silently combined.
//!
//! Arithmetic is exact.
//!
//! Arithmetic errors never silently wrap.
//!
//! A failed ingestion does not partially modify accounting.
//!
//! Poisoned synchronization state is reported explicitly.

use core::fmt;

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, SystemTime};

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

use crate::zhal::identity::HardwareResourceIdentity;

use crate::zhal::model::resource::{
    HardwareResource,
    ResourceAmount,
    ResourceKindId,
    ResourceProvenance,
    ResourceUnitId,
};

// ============================================================================
// COMPONENT CONTRACT
// ============================================================================

/// Stable architectural component identity.
pub const COMPONENT: &str = "zhal.resources.accounting";

/// Accounting schema identifier.
pub const ACCOUNTING_SCHEMA_ID: &str =
    "zamani.zhal.resources.accounting";

/// Accounting API version.
pub const ACCOUNTING_API_VERSION: u16 = 1;

// ============================================================================
// ERRORS
// ============================================================================

fn accounting_error(
    kind: ZhalErrorKind,
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(kind, message)
        .in_component(COMPONENT)
}

fn invalid_input(
    message: impl Into<String>,
) -> ZhalError {
    accounting_error(
        ZhalErrorKind::InvalidInput,
        message,
    )
}

fn invalid_data(
    message: impl Into<String>,
) -> ZhalError {
    accounting_error(
        ZhalErrorKind::InvalidData,
        message,
    )
}

fn storage_error(
    message: impl Into<String>,
) -> ZhalError {
    accounting_error(
        ZhalErrorKind::ResourceExhausted,
        message,
    )
}

// ============================================================================
// VALIDATION
// ============================================================================

fn validate_identifier(
    value: &str,
    field: &str,
) -> ZhalResult<()> {
    if value.is_empty() {
        return Err(invalid_input(
            format!("{field} must not be empty"),
        ));
    }

    if value.trim() != value {
        return Err(invalid_input(
            format!("{field} must not contain surrounding whitespace"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_input(
            format!("{field} must not contain control characters"),
        ));
    }

    Ok(())
}

fn validate_report_id(
    value: &str,
) -> ZhalResult<()> {
    validate_identifier(value, "report identity")
}

fn validate_metric_id(
    value: &str,
) -> ZhalResult<()> {
    validate_identifier(value, "measurement identity")
}

// ============================================================================
// PROVIDER TIMESTAMP
// ============================================================================

/// A provider-supplied timestamp with an explicit clock domain.
///
/// The timestamp is preserved as opaque provider data.
///
/// This implementation deliberately does not assume that every
/// provider uses UTC, synchronized clocks or the same time scale.
///
/// A timestamp must be interpreted by its declared clock domain.
///
/// `uncertainty` describes the provider-reported timestamp uncertainty.
/// It is not an independently verified clock measurement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderTimestamp {
    value: String,
    clock_domain: String,
    uncertainty: Option<Duration>,
}

impl ProviderTimestamp {
    /// Constructs a provider timestamp.
    ///
    /// The timestamp value and clock domain must be nonempty.
    ///
    /// The timestamp is not silently converted to UTC.
    pub fn new(
        value: impl Into<String>,
        clock_domain: impl Into<String>,
        uncertainty: Option<Duration>,
    ) -> ZhalResult<Self> {
        let value = value.into();
        let clock_domain = clock_domain.into();

        validate_identifier(
            &value,
            "provider timestamp",
        )?;

        validate_identifier(
            &clock_domain,
            "provider clock domain",
        )?;

        Ok(Self {
            value,
            clock_domain,
            uncertainty,
        })
    }

    /// Returns the exact provider timestamp.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the provider clock domain.
    #[must_use]
    pub fn clock_domain(&self) -> &str {
        &self.clock_domain
    }

    /// Returns the provider-reported timestamp uncertainty.
    #[must_use]
    pub const fn uncertainty(&self) -> Option<Duration> {
        self.uncertainty
    }
}

// ============================================================================
// MEASUREMENT INTERVAL
// ============================================================================

/// A provider-reported measurement interval.
///
/// Both endpoints must belong to the same clock domain.
///
/// The values are preserved without assuming a particular timestamp
/// serialization or silently converting between clock domains.
///
/// Establishing interval ordering requires an appropriate timestamp
/// interpretation authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeasurementInterval {
    start: ProviderTimestamp,
    end: ProviderTimestamp,
}

impl MeasurementInterval {
    /// Creates a measurement interval.
    ///
    /// # Errors
    ///
    /// Returns an error when the endpoints belong to different
    /// clock domains.
    pub fn new(
        start: ProviderTimestamp,
        end: ProviderTimestamp,
    ) -> ZhalResult<Self> {
        if start.clock_domain() != end.clock_domain() {
            return Err(invalid_input(
                "measurement interval endpoints must use the same clock domain",
            ));
        }

        Ok(Self {
            start,
            end,
        })
    }

    /// Returns the interval start.
    #[must_use]
    pub fn start(&self) -> &ProviderTimestamp {
        &self.start
    }

    /// Returns the interval end.
    #[must_use]
    pub fn end(&self) -> &ProviderTimestamp {
        &self.end
    }
}

// ============================================================================
// MEASUREMENT CLASSIFICATION
// ============================================================================

/// Classification of a provider-reported measurement.
///
/// These classifications have deliberately different semantics.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum MeasurementKind {
    /// A point-in-time measurement.
    ///
    /// Examples include instantaneous power or current temperature.
    ///
    /// Gauge values must not be accumulated as consumption.
    Gauge,

    /// An absolute cumulative counter reading.
    ///
    /// Examples include cumulative energy or cumulative transferred bytes.
    ///
    /// Counter readings are preserved independently.
    ///
    /// Counter resets, rollovers and source changes require explicit
    /// reconciliation. This module does not infer counter deltas.
    CumulativeCounter,

    /// A provider-reported quantity consumed during an interval
    /// or event.
    ///
    /// Delta measurements may be aggregated when their resource
    /// identity, metric identity and unit match.
    Delta,
}

impl MeasurementKind {
    /// Returns whether this measurement represents a delta.
    #[must_use]
    pub const fn is_delta(self) -> bool {
        matches!(self, Self::Delta)
    }

    /// Returns whether this measurement represents a cumulative counter.
    #[must_use]
    pub const fn is_cumulative_counter(self) -> bool {
        matches!(self, Self::CumulativeCounter)
    }

    /// Returns whether this measurement represents a gauge.
    #[must_use]
    pub const fn is_gauge(self) -> bool {
        matches!(self, Self::Gauge)
    }
}

impl fmt::Display for MeasurementKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let value = match self {
            Self::Gauge => "gauge",
            Self::CumulativeCounter => "cumulative_counter",
            Self::Delta => "delta",
        };

        formatter.write_str(value)
    }
}

// ============================================================================
// EVIDENCE QUALITY
// ============================================================================

/// Quality classification reported by the measurement source.
///
/// These values describe the source's declaration, not independent
/// verification by ZHAL.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum MeasurementQuality {
    /// The provider reports a direct measurement.
    Reported,

    /// The provider reports an estimated measurement.
    Estimated,

    /// The provider reports that the measurement is calibrated.
    Calibrated,

    /// The provider does not establish measurement quality.
    Unknown,
}

// ============================================================================
// MEASUREMENT UNCERTAINTY
// ============================================================================

/// Uncertainty associated with a provider-reported measurement.
///
/// Absolute uncertainty uses the same unit as the measured resource.
///
/// Confidence information is preserved as an opaque provider-defined
/// description.
///
/// No unsupported statistical interpretation is performed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeasurementUncertainty {
    absolute: Option<ResourceAmount>,
    confidence: Option<String>,
}

impl MeasurementUncertainty {
    /// Constructs measurement uncertainty.
    ///
    /// `absolute` must use the resource's declared unit.
    ///
    /// The caller is responsible for supplying compatible values.
    pub fn new(
        absolute: Option<ResourceAmount>,
        confidence: Option<String>,
    ) -> ZhalResult<Self> {
        if let Some(value) = &confidence {
            validate_identifier(
                value,
                "measurement confidence description",
            )?;
        }

        Ok(Self {
            absolute,
            confidence,
        })
    }

    /// Creates an uncertainty record with unknown uncertainty.
    #[must_use]
    pub const fn unknown() -> Self {
        Self {
            absolute: None,
            confidence: None,
        }
    }

    /// Returns the reported absolute uncertainty.
    #[must_use]
    pub fn absolute(&self) -> Option<&ResourceAmount> {
        self.absolute.as_ref()
    }

    /// Returns the provider-defined confidence description.
    #[must_use]
    pub fn confidence(&self) -> Option<&str> {
        self.confidence.as_deref()
    }

    /// Returns whether absolute uncertainty is known.
    #[must_use]
    pub fn has_absolute_uncertainty(&self) -> bool {
        self.absolute.is_some()
    }
}

impl Default for MeasurementUncertainty {
    fn default() -> Self {
        Self::unknown()
    }
}

// ============================================================================
// REPORT COMPLETENESS
// ============================================================================

/// Completeness of a provider usage report.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ReportCompleteness {
    /// The provider declares the report complete.
    ///
    /// This is a provider declaration, not independent verification.
    Complete,

    /// The provider declares that information is missing.
    Partial {
        /// Provider-reported omitted information.
        omitted_fields: Vec<String>,
    },

    /// The provider cannot establish report completeness.
    Unknown,
}

impl ReportCompleteness {
    /// Returns whether the provider declares the report complete.
    #[must_use]
    pub const fn is_declared_complete(&self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Returns whether the report is explicitly partial.
    #[must_use]
    pub const fn is_partial(&self) -> bool {
        matches!(self, Self::Partial { .. })
    }
}

// ============================================================================
// PROVIDER USAGE REPORT
// ============================================================================

/// Immutable provider-reported physical resource usage measurement.
///
/// A report contains:
///
/// - A stable report identity.
/// - A physical resource description.
/// - A measurement identity.
/// - A measurement classification.
/// - An exact quantity.
/// - A provider timestamp.
/// - An ingestion timestamp.
/// - Measurement uncertainty.
/// - Provider provenance.
/// - Evidence quality.
/// - Report completeness.
///
/// The resource unit is taken directly from the canonical
/// HardwareResource model.
///
/// No separate resource-unit authority is introduced.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderUsageReport {
    report_id: String,
    resource: HardwareResource,
    metric_id: String,
    kind: MeasurementKind,
    quantity: ResourceAmount,
    timestamp: ProviderTimestamp,
    interval: Option<MeasurementInterval>,
    received_at: SystemTime,
    uncertainty: MeasurementUncertainty,
    quality: MeasurementQuality,
    completeness: ReportCompleteness,
    provenance: ResourceProvenance,
    source_sequence: Option<String>,
}

impl ProviderUsageReport {
    /// Constructs a validated provider usage report.
    ///
    /// The caller must supply the ingestion time.
    ///
    /// This avoids implicitly replacing remote timestamps with local
    /// timestamps or assuming synchronized clocks.
    ///
    /// The resource provider and provenance provider must match.
    ///
    /// Delta measurements must include an explicit measurement interval.
    ///
    /// Gauge and cumulative-counter readings may include an interval
    /// when the provider defines one.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid identifiers, incompatible
    /// provenance or missing delta intervals.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        report_id: impl Into<String>,
        resource: HardwareResource,
        metric_id: impl Into<String>,
        kind: MeasurementKind,
        quantity: ResourceAmount,
        timestamp: ProviderTimestamp,
        interval: Option<MeasurementInterval>,
        received_at: SystemTime,
        uncertainty: MeasurementUncertainty,
        quality: MeasurementQuality,
        completeness: ReportCompleteness,
        provenance: ResourceProvenance,
        source_sequence: Option<String>,
    ) -> ZhalResult<Self> {
        let report_id = report_id.into();
        let metric_id = metric_id.into();

        validate_report_id(&report_id)?;
        validate_metric_id(&metric_id)?;

        let resource_provider =
            resource.identity().device().provider_id();

        if resource_provider != provenance.provider_id() {
            return Err(invalid_data(
                "usage report provenance does not match the resource provider",
            ));
        }

        if matches!(kind, MeasurementKind::Delta)
            && interval.is_none()
        {
            return Err(invalid_input(
                "delta measurements require an explicit measurement interval",
            ));
        }

        if let Some(sequence) = &source_sequence {
            validate_identifier(
                sequence,
                "provider source sequence",
            )?;
        }

        if let ReportCompleteness::Partial {
            omitted_fields,
        } = &completeness
        {
            if omitted_fields.is_empty() {
                return Err(invalid_input(
                    "partial reports must identify at least one omitted field",
                ));
            }

            for field in omitted_fields {
                validate_identifier(
                    field,
                    "omitted report field",
                )?;
            }
        }

        Ok(Self {
            report_id,
            resource,
            metric_id,
            kind,
            quantity,
            timestamp,
            interval,
            received_at,
            uncertainty,
            quality,
            completeness,
            provenance,
            source_sequence,
        })
    }

    /// Returns the stable report identity.
    #[must_use]
    pub fn report_id(&self) -> &str {
        &self.report_id
    }

    /// Returns the physical resource.
    #[must_use]
    pub fn resource(&self) -> &HardwareResource {
        &self.resource
    }

    /// Returns the physical resource identity.
    #[must_use]
    pub fn resource_identity(&self) -> &HardwareResourceIdentity {
        self.resource.identity()
    }

    /// Returns the resource category.
    #[must_use]
    pub fn resource_kind(&self) -> &ResourceKindId {
        self.resource.kind()
    }

    /// Returns the canonical resource unit.
    #[must_use]
    pub fn unit(&self) -> &ResourceUnitId {
        self.resource.unit()
    }

    /// Returns the measurement identity.
    #[must_use]
    pub fn metric_id(&self) -> &str {
        &self.metric_id
    }

    /// Returns the measurement classification.
    #[must_use]
    pub const fn kind(&self) -> MeasurementKind {
        self.kind
    }

    /// Returns the exact provider-reported quantity.
    #[must_use]
    pub fn quantity(&self) -> &ResourceAmount {
        &self.quantity
    }

    /// Returns the provider timestamp.
    #[must_use]
    pub fn timestamp(&self) -> &ProviderTimestamp {
        &self.timestamp
    }

    /// Returns the optional measurement interval.
    #[must_use]
    pub fn interval(&self) -> Option<&MeasurementInterval> {
        self.interval.as_ref()
    }

    /// Returns the local ingestion timestamp.
    #[must_use]
    pub const fn received_at(&self) -> SystemTime {
        self.received_at
    }

    /// Returns measurement uncertainty.
    #[must_use]
    pub fn uncertainty(&self) -> &MeasurementUncertainty {
        &self.uncertainty
    }

    /// Returns provider-declared measurement quality.
    #[must_use]
    pub const fn quality(&self) -> MeasurementQuality {
        self.quality
    }

    /// Returns report completeness.
    #[must_use]
    pub fn completeness(&self) -> &ReportCompleteness {
        &self.completeness
    }

    /// Returns provider provenance.
    #[must_use]
    pub fn provenance(&self) -> &ResourceProvenance {
        &self.provenance
    }

    /// Returns the optional provider sequence identifier.
    #[must_use]
    pub fn source_sequence(&self) -> Option<&str> {
        self.source_sequence.as_deref()
    }
}

// ============================================================================
// ACCOUNTING UPDATE
// ============================================================================

/// Outcome of ingesting a provider usage report.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum AccountingUpdate {
    /// A new report was accepted.
    Recorded,

    /// An identical report was already recorded.
    ///
    /// The accounting state was not modified.
    AlreadyRecorded,
}

// ============================================================================
// ACCOUNTING MEASUREMENT KEY
// ============================================================================

/// Identifies measurements that can be aggregated together.
///
/// Resource identity, resource kind, unit and measurement identity
/// must all match.
///
/// Different units are never automatically converted.
///
/// Different metrics are never combined.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct MeasurementKey {
    resource: HardwareResourceIdentity,
    resource_kind: ResourceKindId,
    unit: ResourceUnitId,
    metric_id: String,
}

impl MeasurementKey {
    fn from_report(
        report: &ProviderUsageReport,
    ) -> Self {
        Self {
            resource: report.resource_identity().clone(),
            resource_kind: report.resource_kind().clone(),
            unit: report.unit().clone(),
            metric_id: report.metric_id().to_owned(),
        }
    }
}

// ============================================================================
// ACCOUNTING STATE
// ============================================================================

#[derive(Default)]
struct AccountingState {
    reports: HashMap<String, ProviderUsageReport>,
}

// ============================================================================
// RESOURCE ACCOUNTING
// ============================================================================

/// Thread-safe provider-reported physical resource accounting.
///
/// Cloned instances share the same state.
///
/// This structure records provider evidence only.
///
/// It does not perform physical reservation, acquisition,
/// allocation or release.
///
/// Runtime accounting remains owned by ZUR.
#[derive(Clone, Default)]
pub struct ResourceAccounting {
    state: Arc<Mutex<AccountingState>>,
}

impl fmt::Debug for ResourceAccounting {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("ResourceAccounting")
            .field(
                "shared_handles",
                &Arc::strong_count(&self.state),
            )
            .finish_non_exhaustive()
    }
}

impl ResourceAccounting {
    /// Creates an independent accounting instance.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of currently retained reports.
    ///
    /// This is the number of records, not the number of resources.
    ///
    /// # Errors
    ///
    /// Returns an error when the accounting lock is poisoned.
    pub fn report_count(&self) -> ZhalResult<usize> {
        let state = lock_state(&self.state)?;

        Ok(state.reports.len())
    }

    /// Records a provider-reported usage measurement.
    ///
    /// Report identities are idempotency keys.
    ///
    /// Replaying an identical report is harmless.
    ///
    /// Reusing an identity with different contents is rejected.
    ///
    /// The operation reserves required hash-map capacity before
    /// changing the accounting state.
    ///
    /// # Errors
    ///
    /// Returns an error for conflicting identities, invalid data,
    /// storage exhaustion or synchronization failure.
    pub fn record(
        &self,
        report: ProviderUsageReport,
    ) -> ZhalResult<AccountingUpdate> {
        let mut state = lock_state(&self.state)?;

        if let Some(existing) = state.reports.get(
            report.report_id(),
        ) {
            if existing == &report {
                return Ok(AccountingUpdate::AlreadyRecorded);
            }

            return Err(accounting_error(
                ZhalErrorKind::ConcurrencyConflict,
                "provider report identity was previously used with different contents",
            ));
        }

        state.reports.try_reserve(1).map_err(|error| {
            storage_error(
                format!(
                    "cannot reserve provider usage report storage: {error}"
                ),
            )
        })?;

        state.reports.insert(
            report.report_id().to_owned(),
            report,
        );

        Ok(AccountingUpdate::Recorded)
    }

    /// Retrieves a report by its stable identity.
    ///
    /// # Errors
    ///
    /// Returns an error when synchronization fails.
    pub fn get(
        &self,
        report_id: &str,
    ) -> ZhalResult<Option<ProviderUsageReport>> {
        validate_report_id(report_id)?;

        let state = lock_state(&self.state)?;

        Ok(state.reports.get(report_id).cloned())
    }

    /// Returns an immutable snapshot of all retained reports.
    ///
    /// Reports are sorted by stable report identity.
    ///
    /// The snapshot represents one internally consistent
    /// accounting state.
    ///
    /// # Errors
    ///
    /// Returns an error when synchronization fails.
    pub fn snapshot(&self) -> ZhalResult<AccountingSnapshot> {
        let state = lock_state(&self.state)?;

        let mut reports: Vec<ProviderUsageReport> =
            state.reports.values().cloned().collect();

        reports.sort_by(|left, right| {
            left.report_id().cmp(right.report_id())
        });

        Ok(AccountingSnapshot {
            reports,
        })
    }

    /// Returns all reports associated with a resource identity.
    ///
    /// The returned reports preserve their original measurement
    /// classifications, quantities, units and provenance.
    ///
    /// # Errors
    ///
    /// Returns an error when synchronization fails.
    pub fn reports_for_resource(
        &self,
        identity: &HardwareResourceIdentity,
    ) -> ZhalResult<Vec<ProviderUsageReport>> {
        let state = lock_state(&self.state)?;

        let mut reports: Vec<ProviderUsageReport> = state
            .reports
            .values()
            .filter(|report| {
                report.resource_identity() == identity
            })
            .cloned()
            .collect();

        reports.sort_by(|left, right| {
            left.report_id().cmp(right.report_id())
        });

        Ok(reports)
    }

    /// Returns a consistent accounting snapshot for a resource.
    ///
    /// This is a read-only operation.
    ///
    /// It does not refresh hardware observations or contact providers.
    ///
    /// # Errors
    ///
    /// Returns an error when synchronization fails.
    pub fn snapshot_for_resource(
        &self,
        identity: &HardwareResourceIdentity,
    ) -> ZhalResult<AccountingSnapshot> {
        let reports = self.reports_for_resource(identity)?;

        Ok(AccountingSnapshot {
            reports,
        })
    }
}

// ============================================================================
// ACCOUNTING SNAPSHOT
// ============================================================================

/// Immutable snapshot of provider-reported usage measurements.
#[derive(Clone, Debug, Default)]
pub struct AccountingSnapshot {
    reports: Vec<ProviderUsageReport>,
}

impl AccountingSnapshot {
    /// Returns all reports in deterministic identity order.
    #[must_use]
    pub fn reports(&self) -> &[ProviderUsageReport] {
        &self.reports
    }

    /// Returns the number of retained reports.
    #[must_use]
    pub fn len(&self) -> usize {
        self.reports.len()
    }

    /// Returns whether the snapshot contains no reports.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.reports.is_empty()
    }

    /// Returns the number of reports declared partial.
    #[must_use]
    pub fn partial_report_count(&self) -> usize {
        self.reports
            .iter()
            .filter(|report| {
                report.completeness().is_partial()
            })
            .count()
    }

    /// Returns the number of reports with unknown completeness.
    #[must_use]
    pub fn unknown_completeness_count(&self) -> usize {
        self.reports
            .iter()
            .filter(|report| {
                matches!(
                    report.completeness(),
                    ReportCompleteness::Unknown
                )
            })
            .count()
    }

    /// Returns the number of gauge observations.
    #[must_use]
    pub fn gauge_count(&self) -> usize {
        self.count_kind(MeasurementKind::Gauge)
    }

    /// Returns the number of cumulative-counter observations.
    #[must_use]
    pub fn cumulative_counter_count(&self) -> usize {
        self.count_kind(
            MeasurementKind::CumulativeCounter,
        )
    }

    /// Returns the number of delta measurements.
    #[must_use]
    pub fn delta_count(&self) -> usize {
        self.count_kind(MeasurementKind::Delta)
    }

    fn count_kind(
        &self,
        kind: MeasurementKind,
    ) -> usize {
        self.reports
            .iter()
            .filter(|report| report.kind() == kind)
            .count()
    }

    /// Aggregates compatible delta measurements.
    ///
    /// Only measurements matching the complete measurement key
    /// are combined:
    ///
    /// - Physical resource identity.
    /// - Resource kind.
    /// - Resource unit.
    /// - Measurement identity.
    ///
    /// Gauge and cumulative-counter values are excluded.
    ///
    /// Different units are never converted.
    ///
    /// Provider provenance and report identities remain available
    /// in the original reports.
    ///
    /// # Errors
    ///
    /// Returns an error when exact decimal addition fails.
    pub fn aggregate_delta(
        &self,
        resource: &HardwareResourceIdentity,
        metric_id: &str,
    ) -> ZhalResult<Option<ResourceAmount>> {
        validate_metric_id(metric_id)?;

        let mut total: Option<ResourceAmount> = None;

        let mut expected_key: Option<MeasurementKey> = None;

        for report in &self.reports {
            if report.resource_identity() != resource
                || report.metric_id() != metric_id
                || !report.kind().is_delta()
            {
                continue;
            }

            let key = MeasurementKey::from_report(report);

            if let Some(expected) = &expected_key {
                if expected != &key {
                    return Err(invalid_data(
                        "delta aggregation encountered incompatible resource measurement dimensions",
                    ));
                }
            } else {
                expected_key = Some(key);
            }

            total = Some(match total {
                Some(previous) => previous
                    .checked_add(report.quantity())
                    .map_err(|error| {
                        invalid_data(
                            format!(
                                "exact delta aggregation failed: {error}"
                            ),
                        )
                    })?,

                None => report.quantity().clone(),
            });
        }

        Ok(total)
    }

    /// Returns all cumulative-counter readings for one resource
    /// and measurement identity.
    ///
    /// No subtraction or reset inference is performed.
    #[must_use]
    pub fn cumulative_readings(
        &self,
        resource: &HardwareResourceIdentity,
        metric_id: &str,
    ) -> Vec<&ProviderUsageReport> {
        self.reports
            .iter()
            .filter(|report| {
                report.resource_identity() == resource
                    && report.metric_id() == metric_id
                    && report.kind().is_cumulative_counter()
            })
            .collect()
    }

    /// Returns all gauge readings for one resource and metric.
    #[must_use]
    pub fn gauge_readings(
        &self,
        resource: &HardwareResourceIdentity,
        metric_id: &str,
    ) -> Vec<&ProviderUsageReport> {
        self.reports
            .iter()
            .filter(|report| {
                report.resource_identity() == resource
                    && report.metric_id() == metric_id
                    && report.kind().is_gauge()
            })
            .collect()
    }
}

// ============================================================================
// SYNCHRONIZATION
// ============================================================================

fn lock_state<'a>(
    state: &'a Arc<Mutex<AccountingState>>,
) -> ZhalResult<MutexGuard<'a, AccountingState>> {
    state.lock().map_err(|_| {
        accounting_error(
            ZhalErrorKind::ConcurrencyConflict,
            "provider usage accounting synchronization state is poisoned",
        )
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::io::devices::DeviceId;

    use crate::stdlib::provider::ProviderId;

    use crate::zhal::identity::{
        HardwareDeviceIdentity,
        HardwareResourceId,
    };

    use crate::zhal::model::resource::{
        ResourceKindId,
        ResourceUnitId,
    };

    fn provider() -> ProviderId {
        ProviderId::new("provider.test")
            .expect("valid provider")
    }

    fn resource() -> HardwareResource {
        HardwareResource::new(
            HardwareResourceIdentity::new(
                HardwareDeviceIdentity::new(
                    provider(),
                    DeviceId::new("device.test")
                        .expect("valid device"),
                ),
                HardwareResourceId::new("resource.test")
                    .expect("valid resource"),
            ),
            ResourceKindId::new("classical.compute")
                .expect("valid kind"),
            ResourceUnitId::new("compute", "operation")
                .expect("valid unit"),
        )
    }

    fn amount(value: &str) -> ResourceAmount {
        ResourceAmount::new(value)
            .expect("valid amount")
    }

    fn timestamp(value: &str) -> ProviderTimestamp {
        ProviderTimestamp::new(
            value,
            "provider.clock.utc",
            None,
        )
        .expect("valid timestamp")
    }

    fn report(
        id: &str,
        kind: MeasurementKind,
        quantity: &str,
        metric: &str,
    ) -> ProviderUsageReport {
        let interval = if kind.is_delta() {
            Some(
                MeasurementInterval::new(
                    timestamp("start"),
                    timestamp("end"),
                )
                .expect("valid interval"),
            )
        } else {
            None
        };

        ProviderUsageReport::new(
            id,
            resource(),
            metric,
            kind,
            amount(quantity),
            timestamp("2026-11-15T10:00:00Z"),
            interval,
            SystemTime::UNIX_EPOCH,
            MeasurementUncertainty::unknown(),
            MeasurementQuality::Reported,
            ReportCompleteness::Complete,
            ResourceProvenance::new(provider()),
            None,
        )
        .expect("valid report")
    }

    // ------------------------------------------------------------------------
    // Measurement classifications
    // ------------------------------------------------------------------------

    #[test]
    fn measurement_kinds_remain_distinct() {
        assert!(MeasurementKind::Gauge.is_gauge());

        assert!(
            MeasurementKind::CumulativeCounter
                .is_cumulative_counter()
        );

        assert!(MeasurementKind::Delta.is_delta());
    }

    #[test]
    fn delta_requires_an_interval() {
        let result = ProviderUsageReport::new(
            "report.1",
            resource(),
            "compute.operations",
            MeasurementKind::Delta,
            amount("10"),
            timestamp("timestamp"),
            None,
            SystemTime::UNIX_EPOCH,
            MeasurementUncertainty::unknown(),
            MeasurementQuality::Reported,
            ReportCompleteness::Complete,
            ResourceProvenance::new(provider()),
            None,
        );

        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // Provenance
    // ------------------------------------------------------------------------

    #[test]
    fn provider_provenance_must_match_resource_owner() {
        let different = ProviderId::new("provider.other")
            .unwrap();

        let result = ProviderUsageReport::new(
            "report.1",
            resource(),
            "compute.operations",
            MeasurementKind::Gauge,
            amount("10"),
            timestamp("timestamp"),
            None,
            SystemTime::UNIX_EPOCH,
            MeasurementUncertainty::unknown(),
            MeasurementQuality::Reported,
            ReportCompleteness::Complete,
            ResourceProvenance::new(different),
            None,
        );

        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // Idempotency
    // ------------------------------------------------------------------------

    #[test]
    fn identical_report_replay_is_idempotent() {
        let accounting = ResourceAccounting::new();

        let first = report(
            "report.1",
            MeasurementKind::Delta,
            "10",
            "compute.operations",
        );

        let second = first.clone();

        assert_eq!(
            accounting.record(first).unwrap(),
            AccountingUpdate::Recorded,
        );

        assert_eq!(
            accounting.record(second).unwrap(),
            AccountingUpdate::AlreadyRecorded,
        );

        assert_eq!(accounting.report_count().unwrap(), 1);
    }

    #[test]
    fn conflicting_report_identity_is_rejected() {
        let accounting = ResourceAccounting::new();

        accounting.record(
            report(
                "report.1",
                MeasurementKind::Gauge,
                "10",
                "compute.operations",
            ),
        ).unwrap();

        let result = accounting.record(
            report(
                "report.1",
                MeasurementKind::Gauge,
                "20",
                "compute.operations",
            ),
        );

        assert!(result.is_err());

        assert_eq!(accounting.report_count().unwrap(), 1);
    }

    // ------------------------------------------------------------------------
    // Exact arithmetic
    // ------------------------------------------------------------------------

    #[test]
    fn delta_measurements_are_aggregated_exactly() {
        let accounting = ResourceAccounting::new();

        accounting.record(
            report(
                "report.1",
                MeasurementKind::Delta,
                "999999999999999999999",
                "compute.operations",
            ),
        ).unwrap();

        accounting.record(
            report(
                "report.2",
                MeasurementKind::Delta,
                "1",
                "compute.operations",
            ),
        ).unwrap();

        let snapshot = accounting.snapshot().unwrap();

        let total = snapshot.aggregate_delta(
            &resource().identity().clone(),
            "compute.operations",
        ).unwrap().unwrap();

        assert_eq!(
            total.as_str(),
            "1000000000000000000000",
        );
    }

    // ------------------------------------------------------------------------
    // Classification isolation
    // ------------------------------------------------------------------------

    #[test]
    fn gauge_measurements_are_not_aggregated_as_deltas() {
        let accounting = ResourceAccounting::new();

        accounting.record(
            report(
                "report.1",
                MeasurementKind::Gauge,
                "100",
                "compute.operations",
            ),
        ).unwrap();

        let snapshot = accounting.snapshot().unwrap();

        assert_eq!(
            snapshot.aggregate_delta(
                &resource().identity().clone(),
                "compute.operations",
            ).unwrap(),
            None,
        );
    }

    #[test]
    fn cumulative_counters_are_not_aggregated_as_deltas() {
        let accounting = ResourceAccounting::new();

        accounting.record(
            report(
                "report.1",
                MeasurementKind::CumulativeCounter,
                "100",
                "compute.operations",
            ),
        ).unwrap();

        accounting.record(
            report(
                "report.2",
                MeasurementKind::CumulativeCounter,
                "200",
                "compute.operations",
            ),
        ).unwrap();

        let snapshot = accounting.snapshot().unwrap();

        assert_eq!(
            snapshot.cumulative_readings(
                &resource().identity().clone(),
                "compute.operations",
            ).len(),
            2,
        );

        assert_eq!(
            snapshot.aggregate_delta(
                &resource().identity().clone(),
                "compute.operations",
            ).unwrap(),
            None,
        );
    }

    // ------------------------------------------------------------------------
    // Partial reports
    // ------------------------------------------------------------------------

    #[test]
    fn partial_reports_are_preserved() {
        let accounting = ResourceAccounting::new();

        let partial = ProviderUsageReport::new(
            "report.partial",
            resource(),
            "compute.operations",
            MeasurementKind::Gauge,
            amount("10"),
            timestamp("timestamp"),
            None,
            SystemTime::UNIX_EPOCH,
            MeasurementUncertainty::unknown(),
            MeasurementQuality::Unknown,
            ReportCompleteness::Partial {
                omitted_fields: vec![
                    "sampling_interval".to_owned(),
                ],
            },
            ResourceProvenance::new(provider()),
            None,
        ).unwrap();

        accounting.record(partial).unwrap();

        let snapshot = accounting.snapshot().unwrap();

        assert_eq!(snapshot.partial_report_count(), 1);
    }

    // ------------------------------------------------------------------------
    // Clock domains
    // ------------------------------------------------------------------------

    #[test]
    fn different_clock_domains_are_not_silently_combined() {
        let result = MeasurementInterval::new(
            timestamp("start"),
            ProviderTimestamp::new(
                "end",
                "device.clock.other",
                None,
            ).unwrap(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn remote_clock_domain_is_preserved() {
        let value = ProviderTimestamp::new(
            "remote-timestamp",
            "device.clock.domain",
            Some(Duration::from_millis(2)),
        ).unwrap();

        assert_eq!(
            value.clock_domain(),
            "device.clock.domain",
        );

        assert_eq!(
            value.uncertainty(),
            Some(Duration::from_millis(2)),
        );
    }

    // ------------------------------------------------------------------------
    // Snapshot behavior
    // ------------------------------------------------------------------------

    #[test]
    fn snapshots_are_sorted_by_report_identity() {
        let accounting = ResourceAccounting::new();

        accounting.record(
            report(
                "report.z",
                MeasurementKind::Gauge,
                "1",
                "metric",
            ),
        ).unwrap();

        accounting.record(
            report(
                "report.a",
                MeasurementKind::Gauge,
                "2",
                "metric",
            ),
        ).unwrap();

        let snapshot = accounting.snapshot().unwrap();

        assert_eq!(
            snapshot.reports()[0].report_id(),
            "report.a",
        );

        assert_eq!(
            snapshot.reports()[1].report_id(),
            "report.z",
        );
    }

    #[test]
    fn snapshots_are_independent_of_future_updates() {
        let accounting = ResourceAccounting::new();

        accounting.record(
            report(
                "report.1",
                MeasurementKind::Gauge,
                "10",
                "metric",
            ),
        ).unwrap();

        let snapshot = accounting.snapshot().unwrap();

        accounting.record(
            report(
                "report.2",
                MeasurementKind::Gauge,
                "20",
                "metric",
            ),
        ).unwrap();

        assert_eq!(snapshot.len(), 1);

        assert_eq!(accounting.report_count().unwrap(), 2);
    }

    // ------------------------------------------------------------------------
    // Scalability
    // ------------------------------------------------------------------------

    #[test]
    fn supports_large_exact_quantities() {
        let quantity = amount(
            "340282366920938463463374607431768211456000000000000000000",
        );

        assert!(!quantity.is_zero());
    }

    #[test]
    fn supports_growing_report_collections() {
        let accounting = ResourceAccounting::new();

        for index in 0..512 {
            accounting.record(
                report(
                    &format!("report.{index}"),
                    MeasurementKind::Gauge,
                    "1",
                    "metric",
                ),
            ).unwrap();
        }

        assert_eq!(
            accounting.report_count().unwrap(),
            512,
        );
    }

    #[test]
    fn cloned_handles_share_accounting_state() {
        let first = ResourceAccounting::new();

        let second = first.clone();

        first.record(
            report(
                "report.shared",
                MeasurementKind::Gauge,
                "10",
                "metric",
            ),
        ).unwrap();

        assert_eq!(
            second.report_count().unwrap(),
            1,
        );
    }
}
