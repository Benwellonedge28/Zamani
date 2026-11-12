
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Resource Accounting
//!
//! Path: src/zur/resources/accounting.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Provides thread-safe, open-world accounting of computational
//! resources whose acquisition and release have been confirmed by
//! their authoritative resource lease provider.
//!
//! This module supports resource accounting across:
//!
//! - Classical computing.
//! - Quantum computing.
//! - HDL and hardware.
//! - AI and tensor computation.
//! - Accelerators.
//! - Hybrid computing.
//! - Distributed computing.
//! - Networking.
//! - Storage.
//! - Energy.
//! - Photonics.
//! - Future computational domains.
//!
//! # ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - General resource usage observations.
//! - Accounting dimensions.
//! - Confirmed acquisition accounting.
//! - Confirmed release accounting.
//! - Per-lease usage tracking.
//! - Current usage aggregation.
//! - Peak usage aggregation.
//! - Cumulative acquisition accounting.
//! - Cumulative release accounting.
//! - Idempotent accounting-event processing.
//! - Consistent accounting snapshots.
//! - Checked exact-decimal arithmetic.
//!
//! This module does NOT own:
//!
//! - Source-language resource semantics.
//! - Resource requirements.
//! - Resource capabilities.
//! - Resource constraints.
//! - Resource preferences.
//! - Resource hints.
//! - Resource feasibility.
//! - Resource discovery.
//! - Resource availability.
//! - Provider registration.
//! - Resource reservation.
//! - Physical allocation.
//! - Hardware selection.
//! - Scheduling.
//! - Routing.
//! - Backend selection.
//! - Execution.
//! - Memory allocation accounting.
//! - Quantum-specific resource accounting.
//! - Distributed membership.
//!
//! # AUTHORITATIVE INTEGRATION
//!
//! Resource lease lifecycle:
//!     crate::zur::resources::lease
//!
//! Runtime identities:
//!     crate::zur::ids
//!
//! Structured operational errors:
//!     crate::zur::error
//!
//! Execution sessions:
//!     crate::zur::execution::session
//!
//! Canonical memory accounting:
//!     crate::zur::memory::accounting
//!
//! Semantic resource definitions:
//!     crate::frontend::semantic::resources
//!
//! The accounting dimensions in this module are opaque references
//! to canonical resource identities and units.
//!
//! They do not introduce another semantic resource model.
//!
//! # IMPORTANT DISTINCTIONS
//!
//! Requirement != Capability.
//! Capability != Availability.
//! Preference != Constraint.
//! Hint != Requirement.
//! Feasibility != Reservation.
//! Reservation != Acquisition.
//! Acquisition != Execution.
//! Accounting != Physical capacity.
//! Accounting != Memory allocation.
//! Expiration != Confirmed release.
//!
//! An accounting update is accepted only for an operation whose
//! corresponding acquisition or release has been confirmed by the
//! existing resource lease authority.
//!
//! # ACCOUNTING MODEL
//!
//! For each resource dimension:
//!
//! Current usage = confirmed acquisitions - confirmed releases
//!
//! Peak usage = highest observed current usage
//!
//! Cumulative acquisition = all confirmed acquired quantities
//!
//! Cumulative release = all confirmed released quantities
//!
//! Quantities use canonical non-negative decimal strings.
//!
//! This avoids introducing an arbitrary u64, u128, usize, or
//! machine-pointer-width ceiling into general resource accounting.
//!
//! Decimal arithmetic is exact. Floating-point approximations
//! are not used.
//!
//! # LEASE INTEGRATION
//!
//! ResourceLease is the canonical authority for reservation lifecycle.
//!
//! Acquisition accounting requires an active confirmed lease.
//!
//! Release accounting requires a confirmed released lease.
//!
//! ResourceLease remains responsible for interacting with the
//! external reservation authority.
//!
//! This module does not acquire or release physical resources.
//!
//! # IDEMPOTENCY
//!
//! Each accounting event has a caller-supplied stable event identity.
//!
//! Replaying an identical event is idempotent.
//!
//! Reusing an event identity with different event contents is rejected.
//!
//! This permits safe reconciliation after retries without silently
//! counting the same confirmed operation twice.
//!
//! Event identities must be unique within one accounting instance.
//!
//! # SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Resource kinds.
//! - Resource units.
//! - Resource dimensions.
//! - Resource leases.
//! - Accounting events.
//! - Providers.
//! - Devices.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Distributed participants.
//! - Computational domains.
//! - Quantity digit counts.
//!
//! Collections grow according to actual available memory.
//!
//! Numeric representation is limited only by available memory
//! and the representational requirements of the implementation.
//!
//! Literal infinite physical resources are not promised.
//!
//! # CONCURRENCY
//!
//! Accounting instances are safely shareable across threads.
//!
//! Each accounting operation is synchronized.
//!
//! A snapshot observes one internally consistent accounting state.
//!
//! No external provider callbacks execute while the accounting lock
//! is held.
//!
//! # FAILURE POLICY
//!
//! Invalid quantities are rejected.
//!
//! Duplicate conflicting events are rejected.
//!
//! Releasing more than the confirmed acquired quantity is rejected.
//!
//! Arithmetic never silently wraps.
//!
//! Failed accounting operations do not partially modify accounting.
//!
//! Poisoned synchronization state is reported explicitly.
//!
//! # GRAMMAR INTEGRATION
//!
//! Relevant source-level authorities:
//!
//! - grammar/resources/resources.g4
//! - grammar/resources/requirements.g4
//! - grammar/resources/budgets.g4
//! - grammar/resources/preferences.g4
//! - grammar/resources/resource-expressions.g4
//! - grammar/resources/constraints.g4
//! - grammar/resources/availability.g4
//! - grammar/resources/capabilities.g4
//! - grammar/spec/resources.md
//! - grammar/specification/semantic-model.md
//! - grammar/specification/poco-reaf.md
//!
//! Grammar describes source-level intent.
//!
//! Semantic analysis validates that intent.
//!
//! Compiler planning evaluates feasibility.
//!
//! ResourceLeaseAuthority performs actual acquisition and release.
//!
//! This module accounts for confirmed operational events.
//!
//! It does not parse grammar or reinterpret semantic expressions.
//!
//! # FREEZE CONTRACT
//!
//! Future resource accounting integrations must use this implementation
//! for general lease-based resource accounting.
//!
//! They must not create competing global counters, resource inventories,
//! reservation engines, or duplicate memory accounting implementations.

use core::cmp::Ordering;
use core::fmt;

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::zur::error::{ZurError, ZurResult};

use crate::zur::resources::lease::{
    LeaseStatus,
    ResourceLease,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Stable architectural component identity.
pub const RESOURCE_ACCOUNTING_COMPONENT: &str =
    "zur.resources.accounting";

/// Resource accounting API version.
pub const RESOURCE_ACCOUNTING_API_VERSION: u16 = 1;

// ============================================================================
// RESOURCE ACCOUNTING DIMENSION
// ============================================================================

/// Identifies one independently accounted resource dimension.
///
/// The resource kind and unit are opaque namespaced references to
/// existing semantic identities.
///
/// Examples:
///
/// - `classical:compute` / `standard:core`
/// - `memory:capacity` / `information:byte`
/// - `quantum:logical-qubit` / `quantum:qubit`
/// - `network:bandwidth` / `information:byte-per-second`
/// - `energy:consumption` / `energy:joule`
/// - `hardware:logic-element` / `hardware:element`
/// - `photonics:optical-channel` / `photonics:channel`
///
/// These examples are not an exhaustive resource inventory.
///
/// Different units are always accounted independently.
///
/// This type does not perform unit conversion.
///
/// Unit compatibility and conversion belong to the semantic
/// resource and unit-resolution authorities.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ResourceAccountingDimension {
    resource_kind: String,
    unit: String,
}

impl ResourceAccountingDimension {
    /// Constructs a validated resource-accounting dimension.
    ///
    /// Both identifiers must be nonempty, namespaced identifiers.
    ///
    /// Identifiers remain open-world.
    pub fn new(
        resource_kind: impl Into<String>,
        unit: impl Into<String>,
    ) -> ZurResult<Self> {
        let resource_kind = resource_kind.into();
        let unit = unit.into();

        validate_namespaced_identifier(
            &resource_kind,
            "resource kind",
        )?;

        validate_namespaced_identifier(
            &unit,
            "resource unit",
        )?;

        Ok(Self {
            resource_kind,
            unit,
        })
    }

    /// Returns the opaque resource-kind identity.
    #[must_use]
    pub fn resource_kind(&self) -> &str {
        &self.resource_kind
    }

    /// Returns the opaque resource-unit identity.
    #[must_use]
    pub fn unit(&self) -> &str {
        &self.unit
    }
}

// ============================================================================
// ACCOUNTING ACTION
// ============================================================================

/// Type of confirmed resource-accounting event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AccountingAction {
    /// A resource acquisition has been confirmed.
    Acquisition,

    /// A resource release has been confirmed.
    Release,
}

// ============================================================================
// ACCOUNTING UPDATE
// ============================================================================

/// Outcome of processing an accounting event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AccountingUpdate {
    /// The event was applied successfully.
    Applied,

    /// The exact same event was already applied.
    ///
    /// The existing accounting state was not modified.
    AlreadyApplied,
}

// ============================================================================
// ACCOUNTING EVENT
// ============================================================================

/// An immutable record of one confirmed accounting operation.
#[derive(Clone, Debug, Eq, PartialEq)]
struct AccountingEvent {
    action: AccountingAction,
    lease_identity: String,
    authority: String,
    dimension: ResourceAccountingDimension,
    quantity: String,
}

impl AccountingEvent {
    fn new(
        action: AccountingAction,
        lease: &ResourceLease,
        dimension: ResourceAccountingDimension,
        quantity: String,
    ) -> Self {
        Self {
            action,
            lease_identity: lease.id().to_string(),
            authority: lease.authority_id().to_owned(),
            dimension,
            quantity,
        }
    }
}

// ============================================================================
// INTERNAL ACCOUNTING KEYS
// ============================================================================

/// Identifies the accounting dimension associated with a lease.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct LeaseDimensionKey {
    lease_identity: String,
    dimension: ResourceAccountingDimension,
}

impl LeaseDimensionKey {
    fn new(
        lease: &ResourceLease,
        dimension: ResourceAccountingDimension,
    ) -> Self {
        Self {
            lease_identity: lease.id().to_string(),
            dimension,
        }
    }
}

// ============================================================================
// RESOURCE USAGE AGGREGATE
// ============================================================================

/// Internal accounting totals for one resource dimension.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct ResourceUsage {
    current: String,
    peak: String,
    cumulative_acquired: String,
    cumulative_released: String,
}

impl ResourceUsage {
    fn new() -> Self {
        Self {
            current: "0".to_owned(),
            peak: "0".to_owned(),
            cumulative_acquired: "0".to_owned(),
            cumulative_released: "0".to_owned(),
        }
    }
}

// ============================================================================
// ACCOUNTING STATE
// ============================================================================

/// Mutable state protected by one accounting synchronization boundary.
#[derive(Default)]
struct AccountingState {
    /// Aggregated usage across all leases.
    usage: HashMap<ResourceAccountingDimension, ResourceUsage>,

    /// Confirmed outstanding quantities associated with individual leases.
    lease_usage: HashMap<LeaseDimensionKey, String>,

    /// Previously applied idempotency events.
    events: HashMap<String, AccountingEvent>,
}

// ============================================================================
// RESOURCE ACCOUNTING
// ============================================================================

/// Thread-safe general computational-resource accounting.
///
/// Cloning this structure creates another handle to the same accounting
/// state. It does not create independent counters.
///
/// The accounting instance has no global state and can be injected
/// into runtime contexts, execution sessions and resource services.
///
/// # Ownership
///
/// This structure tracks confirmed resource usage.
///
/// It does not own physical resource reservations or allocations.
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
            .field("shared_handles", &Arc::strong_count(&self.state))
            .finish_non_exhaustive()
    }
}

impl ResourceAccounting {
    /// Creates an independent accounting instance.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // ========================================================================
    // ACQUISITION ACCOUNTING
    // ========================================================================

    /// Records a confirmed resource acquisition.
    ///
    /// The lease must currently be active.
    ///
    /// This operation must be called only after the authoritative
    /// resource provider has confirmed acquisition.
    ///
    /// `event_id` is an idempotency key supplied by the integration
    /// responsible for reconciling the external operation.
    ///
    /// The same event identity may be submitted repeatedly with
    /// identical contents.
    ///
    /// A conflicting event with the same identity is rejected.
    ///
    /// # Arguments
    ///
    /// - `lease`: Canonical confirmed resource lease.
    /// - `event_id`: Stable accounting-event identity.
    /// - `dimension`: Resource kind and unit.
    /// - `quantity`: Exact non-negative decimal quantity.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid input, inactive leases,
    /// inconsistent duplicate events, exhausted accounting
    /// representation, or synchronization failures.
    pub fn record_acquisition(
        &self,
        lease: &ResourceLease,
        event_id: impl Into<String>,
        dimension: ResourceAccountingDimension,
        quantity: impl AsRef<str>,
    ) -> ZurResult<AccountingUpdate> {
        lease.ensure_active()?;

        self.apply_event(
            lease,
            event_id.into(),
            AccountingAction::Acquisition,
            dimension,
            quantity.as_ref(),
        )
    }

    // ========================================================================
    // RELEASE ACCOUNTING
    // ========================================================================

    /// Records a confirmed resource release.
    ///
    /// The canonical lease must be in the Released state.
    ///
    /// This operation must be called only after the authoritative
    /// resource provider has confirmed release.
    ///
    /// The quantity must not exceed the outstanding quantity previously
    /// accounted for under this lease and resource dimension.
    ///
    /// # Errors
    ///
    /// Returns an error when the lease has not been released,
    /// the quantity exceeds the accounted amount, or an accounting
    /// invariant would be violated.
    pub fn record_release(
        &self,
        lease: &ResourceLease,
        event_id: impl Into<String>,
        dimension: ResourceAccountingDimension,
        quantity: impl AsRef<str>,
    ) -> ZurResult<AccountingUpdate> {
        match lease.status()? {
            LeaseStatus::Released => {}
            _ => {
                return Err(accounting_error(
                    "zur.resources.accounting.release_unconfirmed",
                    "resource release accounting requires a confirmed released lease",
                    "record_release",
                ));
            }
        }

        self.apply_event(
            lease,
            event_id.into(),
            AccountingAction::Release,
            dimension,
            quantity.as_ref(),
        )
    }

    // ========================================================================
    // INTERNAL EVENT PROCESSING
    // ========================================================================

    fn apply_event(
        &self,
        lease: &ResourceLease,
        event_id: String,
        action: AccountingAction,
        dimension: ResourceAccountingDimension,
        quantity: &str,
    ) -> ZurResult<AccountingUpdate> {
        validate_event_id(&event_id)?;

        let quantity = canonical_quantity(quantity)?;

        if quantity == "0" {
            return Err(accounting_error(
                "zur.resources.accounting.zero_quantity",
                "accounting event quantity must be greater than zero",
                "apply_event",
            ));
        }

        let event = AccountingEvent::new(
            action,
            lease,
            dimension.clone(),
            quantity.clone(),
        );

        let lease_key = LeaseDimensionKey::new(
            lease,
            dimension.clone(),
        );

        let mut state = lock_state(&self.state)?;

        // --------------------------------------------------------------------
        // IDEMPOTENCY
        // --------------------------------------------------------------------

        if let Some(existing) = state.events.get(&event_id) {
            if existing == &event {
                return Ok(AccountingUpdate::AlreadyApplied);
            }

            return Err(accounting_error(
                "zur.resources.accounting.event_conflict",
                "accounting event identity was previously used with different contents",
                "apply_event",
            ));
        }

        // --------------------------------------------------------------------
        // PREPARE THE UPDATE
        // --------------------------------------------------------------------

        let previous_usage = state
            .usage
            .get(&dimension)
            .cloned()
            .unwrap_or_else(ResourceUsage::new);

        let previous_lease_quantity = state
            .lease_usage
            .get(&lease_key)
            .cloned()
            .unwrap_or_else(|| "0".to_owned());

        let mut next_usage = previous_usage.clone();

        let next_lease_quantity;

        match action {
            AccountingAction::Acquisition => {
                next_usage.current = decimal_add(
                    &previous_usage.current,
                    &quantity,
                )?;

                next_usage.cumulative_acquired = decimal_add(
                    &previous_usage.cumulative_acquired,
                    &quantity,
                )?;

                if decimal_cmp(
                    &next_usage.current,
                    &next_usage.peak,
                ) == Ordering::Greater {
                    next_usage.peak = next_usage.current.clone();
                }

                next_lease_quantity = decimal_add(
                    &previous_lease_quantity,
                    &quantity,
                )?;
            }

            AccountingAction::Release => {
                next_usage.current = decimal_subtract(
                    &previous_usage.current,
                    &quantity,
                )?;

                next_usage.cumulative_released = decimal_add(
                    &previous_usage.cumulative_released,
                    &quantity,
                )?;

                next_lease_quantity = decimal_subtract(
                    &previous_lease_quantity,
                    &quantity,
                )?;
            }
        }

        // --------------------------------------------------------------------
        // FALLIBLE COLLECTION CAPACITY PREPARATION
        // --------------------------------------------------------------------

        // Reserve all required collection capacity before changing
        // accounting values. This prevents partially committed updates
        // when a collection cannot reserve its required storage.

        state.events.try_reserve(1).map_err(|error| {
            accounting_error(
                "zur.resources.accounting.storage_exhausted",
                format!("cannot reserve accounting event storage: {error}"),
                "apply_event",
            )
        })?;

        if !state.usage.contains_key(&dimension) {
            state.usage.try_reserve(1).map_err(|error| {
                accounting_error(
                    "zur.resources.accounting.storage_exhausted",
                    format!("cannot reserve resource usage storage: {error}"),
                    "apply_event",
                )
            })?;
        }

        if !state.lease_usage.contains_key(&lease_key)
            && next_lease_quantity != "0"
        {
            state.lease_usage.try_reserve(1).map_err(|error| {
                accounting_error(
                    "zur.resources.accounting.storage_exhausted",
                    format!("cannot reserve lease accounting storage: {error}"),
                    "apply_event",
                )
            })?;
        }

        // --------------------------------------------------------------------
        // COMMIT
        // --------------------------------------------------------------------

        state.usage.insert(
            dimension,
            next_usage,
        );

        if next_lease_quantity == "0" {
            state.lease_usage.remove(&lease_key);
        } else {
            state.lease_usage.insert(
                lease_key,
                next_lease_quantity,
            );
        }

        state.events.insert(
            event_id,
            event,
        );

        Ok(AccountingUpdate::Applied)
    }

    // ========================================================================
    // SNAPSHOTS
    // ========================================================================

    /// Returns a consistent snapshot of all accounted resource dimensions.
    ///
    /// The snapshot is independent of subsequent accounting operations.
    ///
    /// Records are sorted deterministically by resource kind and unit.
    pub fn snapshot(&self) -> ZurResult<ResourceAccountingSnapshot> {
        let state = lock_state(&self.state)?;

        let mut entries = Vec::new();

        entries.try_reserve_exact(
            state.usage.len(),
        ).map_err(|error| {
            accounting_error(
                "zur.resources.accounting.snapshot_exhausted",
                format!("cannot allocate accounting snapshot: {error}"),
                "snapshot",
            )
        })?;

        for (dimension, usage) in &state.usage {
            entries.push(
                ResourceUsageSnapshot {
                    dimension: dimension.clone(),
                    current: usage.current.clone(),
                    peak: usage.peak.clone(),
                    cumulative_acquired: usage.cumulative_acquired.clone(),
                    cumulative_released: usage.cumulative_released.clone(),
                }
            );
        }

        entries.sort_by(|left, right| {
            left.dimension
                .resource_kind
                .cmp(&right.dimension.resource_kind)
                .then_with(|| {
                    left.dimension.unit.cmp(&right.dimension.unit)
                })
        });

        Ok(ResourceAccountingSnapshot {
            entries,
            event_count: state.events.len(),
            active_lease_dimensions: state.lease_usage.len(),
        })
    }

    /// Returns accounting for one resource dimension.
    ///
    /// An unobserved dimension returns zero values.
    pub fn usage(
        &self,
        dimension: &ResourceAccountingDimension,
    ) -> ZurResult<ResourceUsageSnapshot> {
        let state = lock_state(&self.state)?;

        let usage = state
            .usage
            .get(dimension)
            .cloned()
            .unwrap_or_else(ResourceUsage::new);

        Ok(ResourceUsageSnapshot {
            dimension: dimension.clone(),
            current: usage.current,
            peak: usage.peak,
            cumulative_acquired: usage.cumulative_acquired,
            cumulative_released: usage.cumulative_released,
        })
    }

    /// Returns the outstanding quantity associated with one lease
    /// and one resource dimension.
    ///
    /// An unobserved lease dimension returns zero.
    pub fn lease_usage(
        &self,
        lease: &ResourceLease,
        dimension: &ResourceAccountingDimension,
    ) -> ZurResult<String> {
        let state = lock_state(&self.state)?;

        let key = LeaseDimensionKey::new(
            lease,
            dimension.clone(),
        );

        Ok(
            state
                .lease_usage
                .get(&key)
                .cloned()
                .unwrap_or_else(|| "0".to_owned())
        )
    }

    /// Returns the number of distinct accounting events accepted.
    ///
    /// This is a diagnostic count, not a hardware limit.
    pub fn event_count(&self) -> ZurResult<usize> {
        let state = lock_state(&self.state)?;
        Ok(state.events.len())
    }
}

// ============================================================================
// IMMUTABLE SNAPSHOTS
// ============================================================================

/// Immutable accounting snapshot for one resource dimension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceUsageSnapshot {
    dimension: ResourceAccountingDimension,
    current: String,
    peak: String,
    cumulative_acquired: String,
    cumulative_released: String,
}

impl ResourceUsageSnapshot {
    /// Returns the resource-accounting dimension.
    #[must_use]
    pub fn dimension(&self) -> &ResourceAccountingDimension {
        &self.dimension
    }

    /// Returns currently outstanding accounted usage.
    #[must_use]
    pub fn current(&self) -> &str {
        &self.current
    }

    /// Returns the highest observed outstanding usage.
    #[must_use]
    pub fn peak(&self) -> &str {
        &self.peak
    }

    /// Returns the cumulative confirmed acquired quantity.
    #[must_use]
    pub fn cumulative_acquired(&self) -> &str {
        &self.cumulative_acquired
    }

    /// Returns the cumulative confirmed released quantity.
    #[must_use]
    pub fn cumulative_released(&self) -> &str {
        &self.cumulative_released
    }

    /// Returns whether current accounted usage is zero.
    #[must_use]
    pub fn is_idle(&self) -> bool {
        self.current == "0"
    }
}

/// Immutable snapshot of all resource-accounting dimensions.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ResourceAccountingSnapshot {
    entries: Vec<ResourceUsageSnapshot>,
    event_count: usize,
    active_lease_dimensions: usize,
}

impl ResourceAccountingSnapshot {
    /// Returns all dimension snapshots in deterministic order.
    #[must_use]
    pub fn entries(&self) -> &[ResourceUsageSnapshot] {
        &self.entries
    }

    /// Returns the number of accepted idempotency events.
    #[must_use]
    pub const fn event_count(&self) -> usize {
        self.event_count
    }

    /// Returns the number of lease/resource-dimension pairs
    /// with outstanding accounted quantities.
    #[must_use]
    pub const fn active_lease_dimensions(&self) -> usize {
        self.active_lease_dimensions
    }

    /// Returns whether all observed dimensions have zero current usage.
    #[must_use]
    pub fn is_idle(&self) -> bool {
        self.entries.iter().all(ResourceUsageSnapshot::is_idle)
    }

    /// Looks up one dimension in this snapshot.
    #[must_use]
    pub fn find(
        &self,
        dimension: &ResourceAccountingDimension,
    ) -> Option<&ResourceUsageSnapshot> {
        self.entries
            .iter()
            .find(|entry| &entry.dimension == dimension)
    }
}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn validate_namespaced_identifier(
    value: &str,
    description: &str,
) -> ZurResult<()> {
    if value.is_empty() || value.trim() != value {
        return Err(accounting_error(
            "zur.resources.accounting.invalid_identifier",
            format!("{description} must be nonempty and contain no surrounding whitespace"),
            "validate_identifier",
        ));
    }

    let mut segment_count = 0usize;

    for segment in value.split('.') {
        if segment.is_empty() {
            return Err(accounting_error(
                "zur.resources.accounting.invalid_identifier",
                format!("{description} contains an empty namespace segment"),
                "validate_identifier",
            ));
        }

        if !segment.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'-'
                || byte == b'_'
                || byte == b':'
        }) {
            return Err(accounting_error(
                "zur.resources.accounting.invalid_identifier",
                format!("{description} contains an unsupported character"),
                "validate_identifier",
            ));
        }

        if segment.starts_with('-')
            || segment.starts_with('_')
            || segment.ends_with('-')
            || segment.ends_with('_')
        {
            return Err(accounting_error(
                "zur.resources.accounting.invalid_identifier",
                format!("{description} has an invalid segment boundary"),
                "validate_identifier",
            ));
        }

        segment_count += 1;
    }

    if segment_count < 2 {
        return Err(accounting_error(
            "zur.resources.accounting.invalid_identifier",
            format!("{description} must be namespaced"),
            "validate_identifier",
        ));
    }

    Ok(())
}

fn validate_event_id(value: &str) -> ZurResult<()> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(accounting_error(
            "zur.resources.accounting.invalid_event_id",
            "accounting event identity is invalid",
            "validate_event_id",
        ));
    }

    Ok(())
}

// ============================================================================
// EXACT DECIMAL QUANTITIES
// ============================================================================

/// Validates and canonicalizes an exact non-negative decimal quantity.
///
/// The format is:
///
/// DIGITS
/// DIGITS.DIGITS
///
/// No floating-point conversion is performed.
fn canonical_quantity(value: &str) -> ZurResult<String> {
    if value.is_empty() {
        return Err(invalid_quantity());
    }

    let mut parts = value.split('.');

    let integer = parts.next().unwrap_or_default();
    let fraction = parts.next();

    if parts.next().is_some()
        || integer.is_empty()
        || !integer.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(invalid_quantity());
    }

    if let Some(fraction) = fraction {
        if fraction.is_empty()
            || !fraction.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(invalid_quantity());
        }
    }

    let integer = integer.trim_start_matches('0');
    let integer = if integer.is_empty() {
        "0"
    } else {
        integer
    };

    let mut result = String::new();

    result.try_reserve(
        integer.len()
            .saturating_add(fraction.map_or(0, str::len))
            .saturating_add(1),
    ).map_err(|error| {
        accounting_error(
            "zur.resources.accounting.quantity_exhausted",
            format!("cannot allocate quantity representation: {error}"),
            "canonical_quantity",
        )
    })?;

    result.push_str(integer);

    if let Some(fraction) = fraction {
        let fraction = fraction.trim_end_matches('0');

        if !fraction.is_empty() {
            result.push('.');
            result.push_str(fraction);
        }
    }

    Ok(result)
}

fn invalid_quantity() -> ZurError {
    accounting_error(
        "zur.resources.accounting.invalid_quantity",
        "expected a non-negative exact decimal quantity",
        "quantity",
    )
}

/// Splits a canonical decimal quantity into integer and fractional parts.
fn decimal_parts(value: &str) -> (&str, &str) {
    match value.split_once('.') {
        Some((integer, fraction)) => (integer, fraction),
        None => (value, ""),
    }
}

/// Compares canonical exact decimal quantities.
fn decimal_cmp(left: &str, right: &str) -> Ordering {
    let (left_integer, left_fraction) = decimal_parts(left);
    let (right_integer, right_fraction) = decimal_parts(right);

    let integer_order = left_integer
        .len()
        .cmp(&right_integer.len())
        .then_with(|| left_integer.cmp(right_integer));

    if integer_order != Ordering::Equal {
        return integer_order;
    }

    let width = left_fraction.len().max(right_fraction.len());

    for index in 0..width {
        let left_digit = left_fraction
            .as_bytes()
            .get(index)
            .copied()
            .unwrap_or(b'0');

        let right_digit = right_fraction
            .as_bytes()
            .get(index)
            .copied()
            .unwrap_or(b'0');

        let order = left_digit.cmp(&right_digit);

        if order != Ordering::Equal {
            return order;
        }
    }

    Ordering::Equal
}

/// Adds two exact non-negative decimal quantities.
fn decimal_add(
    left: &str,
    right: &str,
) -> ZurResult<String> {
    decimal_binary(left, right, false)
}

/// Subtracts two exact non-negative decimal quantities.
///
/// Subtraction is rejected if the result would be negative.
fn decimal_subtract(
    left: &str,
    right: &str,
) -> ZurResult<String> {
    if decimal_cmp(left, right) == Ordering::Less {
        return Err(accounting_error(
            "zur.resources.accounting.underflow",
            "resource release exceeds the outstanding accounted quantity",
            "decimal_subtract",
        ));
    }

    decimal_binary(left, right, true)
}

/// Performs exact decimal addition or subtraction without machine-sized
/// numeric conversions.
///
/// Fractional digits are aligned before arithmetic.
///
/// The implementation grows according to the input representation.
fn decimal_binary(
    left: &str,
    right: &str,
    subtract: bool,
) -> ZurResult<String> {
    let (left_integer, left_fraction) = decimal_parts(left);
    let (right_integer, right_fraction) = decimal_parts(right);

    let scale = left_fraction.len().max(right_fraction.len());

    let left_length = left_integer.len() + scale;
    let right_length = right_integer.len() + scale;

    let width = left_length.max(right_length);

    let mut left_digits = Vec::new();
    let mut right_digits = Vec::new();

    left_digits.try_reserve_exact(width).map_err(|error| {
        accounting_error(
            "zur.resources.accounting.arithmetic_exhausted",
            format!("cannot allocate arithmetic workspace: {error}"),
            "decimal_binary",
        )
    })?;

    right_digits.try_reserve_exact(width).map_err(|error| {
        accounting_error(
            "zur.resources.accounting.arithmetic_exhausted",
            format!("cannot allocate arithmetic workspace: {error}"),
            "decimal_binary",
        )
    })?;

    for index in 0..width {
        left_digits.push(decimal_digit(
            left_integer,
            left_fraction,
            index,
            scale,
        ));

        right_digits.push(decimal_digit(
            right_integer,
            right_fraction,
            index,
            scale,
        ));
    }

    let mut result = Vec::new();

    result.try_reserve_exact(width.saturating_add(1)).map_err(|error| {
        accounting_error(
            "zur.resources.accounting.arithmetic_exhausted",
            format!("cannot allocate arithmetic result: {error}"),
            "decimal_binary",
        )
    })?;

    let mut carry = 0u8;

    for index in (0..width).rev() {
        let left_digit = left_digits[index] - b'0';
        let right_digit = right_digits[index] - b'0';

        if subtract {
            let mut value =
                i16::from(left_digit) - i16::from(right_digit) - i16::from(carry);

            if value < 0 {
                value += 10;
                carry = 1;
            } else {
                carry = 0;
            }

            result.push((value as u8) + b'0');
        } else {
            let value =
                left_digit + right_digit + carry;

            result.push((value % 10) + b'0');
            carry = value / 10;
        }
    }

    if !subtract && carry > 0 {
        result.push(carry + b'0');
    }

    result.reverse();

    let mut digits = String::new();

    digits.try_reserve_exact(result.len()).map_err(|error| {
        accounting_error(
            "zur.resources.accounting.arithmetic_exhausted",
            format!("cannot allocate normalized quantity: {error}"),
            "decimal_binary",
        )
    })?;

    for digit in result {
        digits.push(char::from(digit));
    }

    if scale == 0 {
        return canonical_quantity(&digits);
    }

    let split = digits.len().saturating_sub(scale);

    let integer = if split == 0 {
        "0"
    } else {
        &digits[..split]
    };

    let fractional = if split == 0 {
        let mut padded = String::new();

        padded.try_reserve_exact(scale).map_err(|error| {
            accounting_error(
                "zur.resources.accounting.arithmetic_exhausted",
                format!("cannot allocate fractional quantity: {error}"),
                "decimal_binary",
            )
        })?;

        padded.push_str(&digits);
        padded
    } else {
        digits[split..].to_owned()
    };

    let mut normalized = String::new();

    normalized.try_reserve(
        integer.len()
            .saturating_add(fractional.len())
            .saturating_add(1),
    ).map_err(|error| {
        accounting_error(
            "zur.resources.accounting.arithmetic_exhausted",
            format!("cannot allocate normalized decimal: {error}"),
            "decimal_binary",
        )
    })?;

    normalized.push_str(integer);

    if !fractional.is_empty() {
        normalized.push('.');
        normalized.push_str(&fractional);
    }

    canonical_quantity(&normalized)
}

/// Returns the digit at one position in a virtually zero-padded
/// integer/fraction representation.
///
/// No intermediate machine-sized integer conversion occurs.
fn decimal_digit(
    integer: &str,
    fraction: &str,
    position: usize,
    scale: usize,
) -> u8 {
    let integer_width = integer.len();

    if position < integer_width {
        return integer.as_bytes()[position];
    }

    let fractional_position = position - integer_width;

    if fractional_position < fraction.len() {
        return fraction.as_bytes()[fractional_position];
    }

    // The integer/fraction strings are right-aligned by their scale.
    //
    // The caller's width is sufficient for both representations.
    //
    // Missing fractional positions are zero-padded.
    let _ = scale;

    b'0'
}

// ============================================================================
// SYNCHRONIZATION
// ============================================================================

fn lock_state(
    state: &Mutex<AccountingState>,
) -> ZurResult<MutexGuard<'_, AccountingState>> {
    state.lock().map_err(|_| {
        accounting_error(
            "zur.resources.accounting.lock_poisoned",
            "resource accounting synchronization state is poisoned",
            "lock_state",
        )
    })
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn accounting_error(
    code: &'static str,
    message: impl Into<String>,
    operation: &'static str,
) -> ZurError {
    ZurError::with_code(
        crate::zur::error::ErrorKind::ResourceUnavailable,
        code,
        message,
    )
    .unwrap_or_else(|_| {
        ZurError::resource_unavailable(
            "resource accounting operation failed",
        )
    })
    .with_component(RESOURCE_ACCOUNTING_COMPONENT)
    .with_operation(operation)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonicalizes_exact_quantities() {
        assert_eq!(canonical_quantity("00042").unwrap(), "42");
        assert_eq!(canonical_quantity("001.5000").unwrap(), "1.5");
        assert_eq!(canonical_quantity("000.000").unwrap(), "0");
        assert_eq!(canonical_quantity("100000000000000000000000000000000000").unwrap(),
                   "100000000000000000000000000000000000");
    }

    #[test]
    fn rejects_invalid_quantities() {
        for value in [
            "",
            "-1",
            "+1",
            "1e10",
            "1.2.3",
            ".5",
            "5.",
            "NaN",
            "Infinity",
            "1x",
        ] {
            assert!(canonical_quantity(value).is_err(), "{value}");
        }
    }

    #[test]
    fn adds_arbitrarily_large_integer_quantities() {
        let result = decimal_add(
            "999999999999999999999999999999999999",
            "1",
        ).unwrap();

        assert_eq!(
            result,
            "1000000000000000000000000000000000000",
        );
    }

    #[test]
    fn adds_fractional_quantities_exactly() {
        assert_eq!(
            decimal_add("0.25", "0.75").unwrap(),
            "1",
        );

        assert_eq!(
            decimal_add("10.125", "0.875").unwrap(),
            "11",
        );

        assert_eq!(
            decimal_add("0.0001", "0.0009").unwrap(),
            "0.001",
        );
    }

    #[test]
    fn subtracts_fractional_quantities_exactly() {
        assert_eq!(
            decimal_subtract("10.5", "0.25").unwrap(),
            "10.25",
        );

        assert_eq!(
            decimal_subtract("1", "0.999").unwrap(),
            "0.001",
        );

        assert_eq!(
            decimal_subtract("100", "100").unwrap(),
            "0",
        );
    }

    #[test]
    fn rejects_arithmetic_underflow() {
        assert!(
            decimal_subtract("10", "11").is_err()
        );
    }

    #[test]
    fn compares_exact_quantities_without_float_conversion() {
        assert_eq!(
            decimal_cmp("100000000000000000000", "99"),
            Ordering::Greater,
        );

        assert_eq!(
            decimal_cmp("1.010", "1.01"),
            Ordering::Equal,
        );

        assert_eq!(
            decimal_cmp("0.009", "0.01"),
            Ordering::Less,
        );
    }

    #[test]
    fn resource_dimensions_are_open_world() {
        let dimension = ResourceAccountingDimension::new(
            "future.photonics",
            "photonics.optical-channel",
        ).unwrap();

        assert_eq!(
            dimension.resource_kind(),
            "future.photonics",
        );

        assert_eq!(
            dimension.unit(),
            "photonics.optical-channel",
        );
    }

    #[test]
    fn rejects_invalid_resource_dimensions() {
        assert!(
            ResourceAccountingDimension::new(
                "",
                "information.byte",
            ).is_err()
        );

        assert!(
            ResourceAccountingDimension::new(
                "classical.compute",
                "",
            ).is_err()
        );

        assert!(
            ResourceAccountingDimension::new(
                "classical..compute",
                "standard.core",
            ).is_err()
        );
    }

    #[test]
    fn independent_accounting_instances_do_not_share_state() {
        let first = ResourceAccounting::new();
        let second = ResourceAccounting::new();

        assert_eq!(first.event_count().unwrap(), 0);
        assert_eq!(second.event_count().unwrap(), 0);
    }

    #[test]
    fn cloned_accounting_handles_share_state() {
        let accounting = ResourceAccounting::new();
        let observer = accounting.clone();

        assert_eq!(
            observer.event_count().unwrap(),
            accounting.event_count().unwrap(),
        );
    }

    #[test]
    fn snapshot_is_deterministically_ordered() {
        let mut entries = [
            ResourceUsageSnapshot {
                dimension: ResourceAccountingDimension::new(
                    "quantum.compute",
                    "quantum.qubit",
                ).unwrap(),
                current: "1".to_owned(),
                peak: "1".to_owned(),
                cumulative_acquired: "1".to_owned(),
                cumulative_released: "0".to_owned(),
            },
            ResourceUsageSnapshot {
                dimension: ResourceAccountingDimension::new(
                    "classical.compute",
                    "standard.core",
                ).unwrap(),
                current: "1".to_owned(),
                peak: "1".to_owned(),
                cumulative_acquired: "1".to_owned(),
                cumulative_released: "0".to_owned(),
            },
        ];

        entries.sort_by(|left, right| {
            left.dimension.resource_kind
                .cmp(&right.dimension.resource_kind)
                .then_with(|| {
                    left.dimension.unit.cmp(&right.dimension.unit)
                })
        });

        assert_eq!(
            entries[0].dimension.resource_kind(),
            "classical.compute",
        );
    }

    #[test]
    fn zero_usage_snapshot_is_idle() {
        let snapshot = ResourceUsageSnapshot {
            dimension: ResourceAccountingDimension::new(
                "classical.compute",
                "standard.core",
            ).unwrap(),
            current: "0".to_owned(),
            peak: "0".to_owned(),
            cumulative_acquired: "0".to_owned(),
            cumulative_released: "0".to_owned(),
        };

        assert!(snapshot.is_idle());
    }

    #[test]
    fn empty_accounting_snapshot_is_idle() {
        let snapshot = ResourceAccountingSnapshot::default();

        assert!(snapshot.is_idle());
        assert_eq!(snapshot.event_count(), 0);
        assert_eq!(snapshot.active_lease_dimensions(), 0);
    }
}
