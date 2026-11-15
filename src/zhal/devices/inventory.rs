
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Device Inventory
//!
//! Path: `src/zhal/devices/inventory.rs`
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the canonical, thread-safe inventory of observed
//! hardware-device descriptors exposed through ZHAL.
//!
//! The inventory supports:
//!
//! - Classical hardware.
//! - Quantum hardware.
//! - FPGA and ASIC hardware.
//! - GPUs, TPUs, NPUs and DSPs.
//! - Embedded hardware.
//! - Distributed hardware.
//! - Hybrid computing hardware.
//! - Neuromorphic hardware.
//! - Photonic hardware.
//! - Analog hardware.
//! - Future computational technologies.
//!
//! Hardware categories remain open-world.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - HardwareDeviceInventory.
//! - InventorySnapshot.
//! - InventoryQuery.
//! - InventoryPage.
//! - InventoryCursor.
//! - ProviderInventoryRecord.
//! - ProviderRefreshToken.
//! - Provider refresh staging.
//! - Atomic provider snapshot replacement.
//! - Deterministic inventory indexes.
//! - Inventory generation tracking.
//! - Inventory consistency validation.
//!
//! This module does NOT own:
//!
//! - Provider registration.
//! - Provider identity.
//! - Hardware-device identity.
//! - Hardware-device descriptors.
//! - Hardware discovery algorithms.
//! - Provider lifecycle.
//! - Capability semantics.
//! - Semantic requirements.
//! - Hardware allocation.
//! - Hardware reservation.
//! - Hardware authorization.
//! - Hardware execution.
//! - Quantum routing.
//! - HDL realization.
//! - Distributed scheduling.
//! - Runtime orchestration.
//! - Source-language grammar.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! # Canonical dependencies
//!
//! Provider identity:
//!     crate::stdlib::provider::ProviderId
//!
//! Device identity:
//!     crate::zhal::identity::HardwareDeviceIdentity
//!
//! Device descriptor:
//!     crate::zhal::model::device::HardwareDeviceDescriptor
//!
//! Device category:
//!     crate::stdlib::io::devices::DeviceKindId
//!
//! Capability snapshot identity:
//!     crate::zhal::identity::HardwareSnapshotId
//!
//! Device discovery:
//!     crate::zhal::devices::discovery
//!
//! Provider discovery:
//!     crate::zhal::providers::discovery
//!
//! Runtime allocation:
//!     crate::zur
//!
//! # Discovery consistency
//!
//! Provider discovery is incremental.
//!
//! Pages are staged privately until a complete provider snapshot
//! has been received.
//!
//! Only a complete snapshot may replace the currently published
//! observations for that provider.
//!
//! A failed, cancelled, incomplete or invalid refresh preserves
//! the previously published inventory.
//!
//! # Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Number of devices.
//! - Number of providers.
//! - Number of hardware categories.
//! - Number of hardware technologies.
//! - Number of inventory entries.
//! - Device identifier length.
//! - Provider identifier length.
//! - Discovery page count.
//!
//! Memory consumption is determined by actual observations,
//! indexes, pending refreshes and available system resources.
//!
//! Pagination is available for bounded retrieval.
//!
//! Literal infinite physical storage or computation is not promised.
//!
//! # Grammar boundary
//!
//! Relevant source grammar:
//!
//! - grammar/hardware/devices.g4
//! - grammar/hardware/hardware.g4
//! - grammar/effects/hardware.g4
//!
//! Relevant specifications:
//!
//! - grammar/specification/poco-reaf.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/execution-model.md
//! - grammar/validation/semantic-boundaries.md
//!
//! Source-language grammar describes logical hardware intent.
//!
//! This inventory describes observed physical hardware.
//!
//! It does not parse grammar or reinterpret semantic requirements.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! No global mutable state is introduced.
//!
//! Provider callbacks are never executed while holding the
//! inventory lock.

use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use core::num::NonZeroUsize;

use crate::stdlib::io::devices::DeviceKindId;
use crate::stdlib::provider::ProviderId;

use crate::zhal::devices::discovery::DeviceDiscoveryPage;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

use crate::zhal::identity::{
    HardwareDeviceIdentity,
    HardwareSnapshotId,
};

use crate::zhal::model::device::{
    HardwareDeviceDescriptor,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Stable inventory component identifier.
pub const COMPONENT: &str = "zhal.devices.inventory";

/// Inventory snapshot schema identifier.
pub const INVENTORY_SCHEMA_ID: &str =
    "zamani.zhal.device-inventory";

/// Inventory snapshot schema major version.
pub const INVENTORY_SCHEMA_MAJOR: u16 = 1;

/// Inventory snapshot schema minor version.
pub const INVENTORY_SCHEMA_MINOR: u16 = 0;

/// Inventory snapshot schema patch version.
pub const INVENTORY_SCHEMA_PATCH: u16 = 0;

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn inventory_error(
    kind: ZhalErrorKind,
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(kind, message).in_component(COMPONENT)
}

fn lock_error() -> ZhalError {
    inventory_error(
        ZhalErrorKind::Internal,
        "hardware inventory lock was poisoned",
    )
}

fn next_generation(value: u64) -> ZhalResult<u64> {
    value.checked_add(1).ok_or_else(|| {
        inventory_error(
            ZhalErrorKind::ResourceExhausted,
            "inventory generation counter is exhausted",
        )
    })
}

// ============================================================================
// PROVIDER REFRESH TOKEN
// ============================================================================

/// Identifies one provider inventory refresh operation.
///
/// Tokens are created by the inventory and must not be
/// manufactured by callers.
///
/// Each token is associated with exactly one provider.
///
/// A refresh token does not authorize hardware access.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRefreshToken {
    provider_id: ProviderId,
    sequence: u64,
}

impl ProviderRefreshToken {
    /// Returns the provider associated with this refresh.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the refresh sequence identifier.
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }
}

// ============================================================================
// PROVIDER INVENTORY RECORD
// ============================================================================

/// Immutable summary of one published provider inventory.
///
/// This record describes an observed snapshot.
///
/// It does not establish provider trust, hardware availability,
/// authorization or successful execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderInventoryRecord {
    provider_id: ProviderId,
    inventory_generation: u64,
    snapshot_revision: Option<String>,
    device_count: usize,
}

impl ProviderInventoryRecord {
    /// Returns the provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the inventory generation at publication.
    #[must_use]
    pub const fn inventory_generation(&self) -> u64 {
        self.inventory_generation
    }

    /// Returns the provider-defined discovery revision.
    #[must_use]
    pub fn snapshot_revision(&self) -> Option<&str> {
        self.snapshot_revision.as_deref()
    }

    /// Returns the number of devices in the published snapshot.
    #[must_use]
    pub const fn device_count(&self) -> usize {
        self.device_count
    }
}

// ============================================================================
// INTERNAL REFRESH STATE
// ============================================================================

struct PendingProviderRefresh {
    token: ProviderRefreshToken,

    staged: BTreeMap<
        HardwareDeviceIdentity,
        Arc<HardwareDeviceDescriptor>,
    >,

    next_page_index: usize,

    snapshot_revision: Option<String>,

    revision_initialized: bool,

    complete: bool,
}

impl PendingProviderRefresh {
    fn new(token: ProviderRefreshToken) -> Self {
        Self {
            token,
            staged: BTreeMap::new(),
            next_page_index: 0,
            snapshot_revision: None,
            revision_initialized: false,
            complete: false,
        }
    }
}

// ============================================================================
// INTERNAL INVENTORY STATE
// ============================================================================

struct InventoryState {
    generation: u64,

    next_refresh_sequence: u64,

    devices: BTreeMap<
        HardwareDeviceIdentity,
        Arc<HardwareDeviceDescriptor>,
    >,

    provider_index: BTreeMap<
        ProviderId,
        BTreeSet<HardwareDeviceIdentity>,
    >,

    category_index: BTreeMap<
        DeviceKindId,
        BTreeSet<HardwareDeviceIdentity>,
    >,

    availability_index: BTreeMap<
        String,
        BTreeSet<HardwareDeviceIdentity>,
    >,

    capability_snapshot_index: BTreeMap<
        HardwareSnapshotId,
        BTreeSet<HardwareDeviceIdentity>,
    >,

    providers: BTreeMap<
        ProviderId,
        ProviderInventoryRecord,
    >,

    pending: BTreeMap<
        ProviderId,
        PendingProviderRefresh,
    >,
}

impl InventoryState {
    fn new() -> Self {
        Self {
            generation: 0,
            next_refresh_sequence: 0,
            devices: BTreeMap::new(),
            provider_index: BTreeMap::new(),
            category_index: BTreeMap::new(),
            availability_index: BTreeMap::new(),
            capability_snapshot_index: BTreeMap::new(),
            providers: BTreeMap::new(),
            pending: BTreeMap::new(),
        }
    }

    fn insert_device(
        &mut self,
        descriptor: Arc<HardwareDeviceDescriptor>,
    ) {
        let identity = descriptor.identity().clone();

        self.provider_index
            .entry(descriptor.provider_id().clone())
            .or_default()
            .insert(identity.clone());

        self.category_index
            .entry(descriptor.category().clone())
            .or_default()
            .insert(identity.clone());

        self.availability_index
            .entry(descriptor.availability().state().to_owned())
            .or_default()
            .insert(identity.clone());

        if let Some(snapshot) = descriptor.capability_snapshot() {
            self.capability_snapshot_index
                .entry(snapshot.clone())
                .or_default()
                .insert(identity.clone());
        }

        self.devices.insert(identity, descriptor);
    }

    fn remove_device(
        &mut self,
        identity: &HardwareDeviceIdentity,
    ) {
        let Some(descriptor) = self.devices.remove(identity) else {
            return;
        };

        remove_index_entry(
            &mut self.provider_index,
            descriptor.provider_id(),
            identity,
        );

        remove_index_entry(
            &mut self.category_index,
            descriptor.category(),
            identity,
        );

        remove_index_entry(
            &mut self.availability_index,
            descriptor.availability().state(),
            identity,
        );

        if let Some(snapshot) = descriptor.capability_snapshot() {
            remove_index_entry(
                &mut self.capability_snapshot_index,
                snapshot,
                identity,
            );
        }
    }

    fn remove_provider_devices(
        &mut self,
        provider_id: &ProviderId,
    ) {
        let identities = self.provider_index
            .get(provider_id)
            .cloned()
            .unwrap_or_default();

        for identity in identities {
            self.remove_device(&identity);
        }

        self.provider_index.remove(provider_id);
    }

    fn rebuild_indexes_are_consistent(&self) -> bool {
        for (identity, descriptor) in &self.devices {
            if descriptor.identity() != identity {
                return false;
            }

            if !self.provider_index
                .get(descriptor.provider_id())
                .is_some_and(|set| set.contains(identity))
            {
                return false;
            }

            if !self.category_index
                .get(descriptor.category())
                .is_some_and(|set| set.contains(identity))
            {
                return false;
            }

            if !self.availability_index
                .get(descriptor.availability().state())
                .is_some_and(|set| set.contains(identity))
            {
                return false;
            }

            if let Some(snapshot) = descriptor.capability_snapshot() {
                if !self.capability_snapshot_index
                    .get(snapshot)
                    .is_some_and(|set| set.contains(identity))
                {
                    return false;
                }
            }
        }

        true
    }
}

fn remove_index_entry<K>(
    index: &mut BTreeMap<K, BTreeSet<HardwareDeviceIdentity>>,
    key: &K,
    identity: &HardwareDeviceIdentity,
)
where
    K: Ord,
{
    let remove_key = if let Some(entries) = index.get_mut(key) {
        entries.remove(identity);
        entries.is_empty()
    } else {
        false
    };

    if remove_key {
        index.remove(key);
    }
}

// ============================================================================
// INVENTORY QUERY
// ============================================================================

/// Immutable inventory query criteria.
///
/// Every populated criterion is combined using logical AND.
///
/// All identities use their canonical repository definitions.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InventoryQuery {
    provider: Option<ProviderId>,

    category: Option<DeviceKindId>,

    availability_state: Option<String>,

    capability_snapshot: Option<HardwareSnapshotId>,
}

impl InventoryQuery {
    /// Creates an unrestricted inventory query.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Restricts results to one provider.
    #[must_use]
    pub fn with_provider(
        mut self,
        provider: ProviderId,
    ) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Restricts results to one hardware category.
    #[must_use]
    pub fn with_category(
        mut self,
        category: DeviceKindId,
    ) -> Self {
        self.category = Some(category);
        self
    }

    /// Restricts results to one exact availability state.
    ///
    /// Availability classifications remain open-world.
    #[must_use]
    pub fn with_availability_state(
        mut self,
        state: impl Into<String>,
    ) -> Self {
        self.availability_state = Some(state.into());
        self
    }

    /// Restricts results to one capability snapshot identity.
    ///
    /// This compares snapshot references only.
    ///
    /// It does not evaluate whether a workload requirement
    /// is satisfied by the capability evidence.
    #[must_use]
    pub fn with_capability_snapshot(
        mut self,
        snapshot: HardwareSnapshotId,
    ) -> Self {
        self.capability_snapshot = Some(snapshot);
        self
    }

    /// Returns the provider restriction.
    #[must_use]
    pub fn provider(&self) -> Option<&ProviderId> {
        self.provider.as_ref()
    }

    /// Returns the category restriction.
    #[must_use]
    pub fn category(&self) -> Option<&DeviceKindId> {
        self.category.as_ref()
    }

    /// Returns the availability restriction.
    #[must_use]
    pub fn availability_state(&self) -> Option<&str> {
        self.availability_state.as_deref()
    }

    /// Returns the capability snapshot restriction.
    #[must_use]
    pub fn capability_snapshot(&self) -> Option<&HardwareSnapshotId> {
        self.capability_snapshot.as_ref()
    }

    fn matches(
        &self,
        descriptor: &HardwareDeviceDescriptor,
    ) -> bool {
        if let Some(provider) = &self.provider {
            if descriptor.provider_id() != provider {
                return false;
            }
        }

        if let Some(category) = &self.category {
            if descriptor.category() != category {
                return false;
            }
        }

        if let Some(state) = &self.availability_state {
            if descriptor.availability().state() != state {
                return false;
            }
        }

        if let Some(snapshot) = &self.capability_snapshot {
            if descriptor.capability_snapshot() != Some(snapshot) {
                return false;
            }
        }

        true
    }
}

// ============================================================================
// INVENTORY CURSOR
// ============================================================================

/// Continuation cursor for deterministic inventory pagination.
///
/// A cursor belongs to the inventory generation from which
/// it was created.
///
/// Any subsequent inventory mutation invalidates the cursor.
///
/// This prevents mixing entries from different inventory snapshots.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InventoryCursor {
    generation: u64,

    after: HardwareDeviceIdentity,
}

impl InventoryCursor {
    /// Returns the generation associated with this cursor.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Returns the identity after which pagination continues.
    #[must_use]
    pub fn after(&self) -> &HardwareDeviceIdentity {
        &self.after
    }
}

// ============================================================================
// INVENTORY PAGE
// ============================================================================

/// One immutable page of inventory query results.
#[derive(Clone, Debug)]
pub struct InventoryPage {
    generation: u64,

    devices: Vec<Arc<HardwareDeviceDescriptor>>,

    next_cursor: Option<InventoryCursor>,
}

impl InventoryPage {
    /// Returns the inventory generation.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Returns the page's device descriptors.
    #[must_use]
    pub fn devices(&self) -> &[Arc<HardwareDeviceDescriptor>] {
        &self.devices
    }

    /// Returns the number of devices in the page.
    #[must_use]
    pub fn len(&self) -> usize {
        self.devices.len()
    }

    /// Returns whether the page is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Returns the next pagination cursor.
    #[must_use]
    pub fn next_cursor(&self) -> Option<&InventoryCursor> {
        self.next_cursor.as_ref()
    }

    /// Returns whether more matching results are available.
    #[must_use]
    pub fn has_more(&self) -> bool {
        self.next_cursor.is_some()
    }
}

// ============================================================================
// IMMUTABLE INVENTORY SNAPSHOT
// ============================================================================

/// Immutable inventory snapshot.
///
/// Snapshot creation clones Arc handles rather than duplicating
/// the complete device descriptors.
///
/// A snapshot remains consistent even after the live inventory
/// is subsequently modified.
#[derive(Clone, Debug)]
pub struct InventorySnapshot {
    generation: u64,

    devices: Vec<Arc<HardwareDeviceDescriptor>>,

    providers: Vec<ProviderInventoryRecord>,
}

impl InventorySnapshot {
    /// Returns the snapshot schema identifier.
    #[must_use]
    pub const fn schema_id(&self) -> &'static str {
        INVENTORY_SCHEMA_ID
    }

    /// Returns the snapshot schema version.
    #[must_use]
    pub const fn schema_version(&self) -> (u16, u16, u16) {
        (
            INVENTORY_SCHEMA_MAJOR,
            INVENTORY_SCHEMA_MINOR,
            INVENTORY_SCHEMA_PATCH,
        )
    }

    /// Returns the captured inventory generation.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Returns all observed devices in deterministic order.
    #[must_use]
    pub fn devices(&self) -> &[Arc<HardwareDeviceDescriptor>] {
        &self.devices
    }

    /// Returns all provider inventory records.
    #[must_use]
    pub fn providers(&self) -> &[ProviderInventoryRecord] {
        &self.providers
    }

    /// Returns the number of observed devices.
    #[must_use]
    pub fn len(&self) -> usize {
        self.devices.len()
    }

    /// Returns whether the snapshot contains no devices.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }
}

// ============================================================================
// HARDWARE DEVICE INVENTORY
// ============================================================================

/// Thread-safe, provider-independent hardware-device inventory.
///
/// The inventory owns observed descriptors and their indexes.
///
/// It does not own provider implementations or execute provider
/// callbacks.
///
/// Multiple independent inventories may coexist.
///
/// Cloning this object shares the same underlying inventory state.
#[derive(Clone)]
pub struct HardwareDeviceInventory {
    state: Arc<RwLock<InventoryState>>,
}

impl Default for HardwareDeviceInventory {
    fn default() -> Self {
        Self::new()
    }
}

impl HardwareDeviceInventory {
    /// Creates an empty hardware-device inventory.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Arc::new(
                RwLock::new(InventoryState::new()),
            ),
        }
    }

    // ------------------------------------------------------------------------
    // LOCK ACCESS
    // ------------------------------------------------------------------------

    fn read_state(
        &self,
    ) -> ZhalResult<RwLockReadGuard<'_, InventoryState>> {
        self.state.read().map_err(|_| lock_error())
    }

    fn write_state(
        &self,
    ) -> ZhalResult<RwLockWriteGuard<'_, InventoryState>> {
        self.state.write().map_err(|_| lock_error())
    }

    // ------------------------------------------------------------------------
    // BASIC INFORMATION
    // ------------------------------------------------------------------------

    /// Returns the current inventory generation.
    ///
    /// Every successful publication or provider removal advances
    /// the generation.
    pub fn generation(&self) -> ZhalResult<u64> {
        Ok(self.read_state()?.generation)
    }

    /// Returns the number of observed devices.
    pub fn len(&self) -> ZhalResult<usize> {
        Ok(self.read_state()?.devices.len())
    }

    /// Returns whether the inventory is empty.
    pub fn is_empty(&self) -> ZhalResult<bool> {
        Ok(self.read_state()?.devices.is_empty())
    }

    /// Returns the number of providers with published inventories.
    pub fn provider_count(&self) -> ZhalResult<usize> {
        Ok(self.read_state()?.providers.len())
    }

    /// Returns the number of providers currently being refreshed.
    pub fn pending_refresh_count(&self) -> ZhalResult<usize> {
        Ok(self.read_state()?.pending.len())
    }

    // ------------------------------------------------------------------------
    // DEVICE LOOKUP
    // ------------------------------------------------------------------------

    /// Looks up one device using its canonical composite identity.
    ///
    /// Provider-scoped identities prevent collisions between
    /// devices belonging to different providers.
    pub fn get(
        &self,
        identity: &HardwareDeviceIdentity,
    ) -> ZhalResult<Option<Arc<HardwareDeviceDescriptor>>> {
        Ok(
            self.read_state()?
                .devices
                .get(identity)
                .cloned(),
        )
    }

    /// Returns every device belonging to one provider.
    ///
    /// Results use canonical deterministic identity ordering.
    pub fn devices_by_provider(
        &self,
        provider: &ProviderId,
    ) -> ZhalResult<Vec<Arc<HardwareDeviceDescriptor>>> {
        let state = self.read_state()?;

        let Some(identities) = state.provider_index.get(provider) else {
            return Ok(Vec::new());
        };

        Ok(
            identities
                .iter()
                .filter_map(|identity| {
                    state.devices.get(identity).cloned()
                })
                .collect(),
        )
    }

    /// Returns every device belonging to one hardware category.
    ///
    /// Categories remain open-world.
    pub fn devices_by_category(
        &self,
        category: &DeviceKindId,
    ) -> ZhalResult<Vec<Arc<HardwareDeviceDescriptor>>> {
        let state = self.read_state()?;

        let Some(identities) = state.category_index.get(category) else {
            return Ok(Vec::new());
        };

        Ok(
            identities
                .iter()
                .filter_map(|identity| {
                    state.devices.get(identity).cloned()
                })
                .collect(),
        )
    }

    /// Returns every device with one exact availability state.
    pub fn devices_by_availability(
        &self,
        availability: &str,
    ) -> ZhalResult<Vec<Arc<HardwareDeviceDescriptor>>> {
        let state = self.read_state()?;

        let Some(identities) = state.availability_index.get(availability) else {
            return Ok(Vec::new());
        };

        Ok(
            identities
                .iter()
                .filter_map(|identity| {
                    state.devices.get(identity).cloned()
                })
                .collect(),
        )
    }

    /// Returns devices referencing one capability snapshot.
    ///
    /// This is an identity lookup only.
    ///
    /// Capability feasibility must be evaluated by the
    /// canonical capability subsystem.
    pub fn devices_by_capability_snapshot(
        &self,
        snapshot: &HardwareSnapshotId,
    ) -> ZhalResult<Vec<Arc<HardwareDeviceDescriptor>>> {
        let state = self.read_state()?;

        let Some(identities) =
            state.capability_snapshot_index.get(snapshot)
        else {
            return Ok(Vec::new());
        };

        Ok(
            identities
                .iter()
                .filter_map(|identity| {
                    state.devices.get(identity).cloned()
                })
                .collect(),
        )
    }

    // ------------------------------------------------------------------------
    // GENERAL QUERIES
    // ------------------------------------------------------------------------

    /// Returns all devices matching an inventory query.
    ///
    /// Results are deterministic.
    ///
    /// This method is unbounded. Callers requiring bounded
    /// retrieval should use query_page.
    pub fn query(
        &self,
        query: &InventoryQuery,
    ) -> ZhalResult<Vec<Arc<HardwareDeviceDescriptor>>> {
        let state = self.read_state()?;

        let candidates = candidate_identities(&state, query);

        Ok(
            candidates
                .into_iter()
                .filter_map(|identity| {
                    state.devices.get(&identity)
                })
                .filter(|descriptor| query.matches(descriptor))
                .cloned()
                .collect(),
        )
    }

    /// Retrieves a bounded page of matching devices.
    ///
    /// The cursor is valid only for the inventory generation
    /// from which it was issued.
    ///
    /// A stale cursor returns a version-mismatch error.
    ///
    /// The page size is an operational retrieval budget,
    /// not a hardware capacity limit.
    pub fn query_page(
        &self,
        query: &InventoryQuery,
        page_size: NonZeroUsize,
        cursor: Option<&InventoryCursor>,
    ) -> ZhalResult<InventoryPage> {
        let state = self.read_state()?;

        if let Some(cursor) = cursor {
            if cursor.generation != state.generation {
                return Err(inventory_error(
                    ZhalErrorKind::VersionMismatch,
                    "inventory pagination cursor belongs to a stale generation",
                ));
            }
        }

        let candidates = candidate_identities(&state, query);

        let mut devices = Vec::new();

        let mut next_cursor = None;

        let limit = page_size.get();

        for identity in candidates {
            if let Some(cursor) = cursor {
                if identity <= cursor.after {
                    continue;
                }
            }

            let Some(descriptor) = state.devices.get(&identity) else {
                continue;
            };

            if !query.matches(descriptor) {
                continue;
            }

            if devices.len() == limit {
                next_cursor = devices.last().map(|device: &Arc<HardwareDeviceDescriptor>| {
                    InventoryCursor {
                        generation: state.generation,
                        after: device.identity().clone(),
                    }
                });

                break;
            }

            devices.push(Arc::clone(descriptor));
        }

        Ok(InventoryPage {
            generation: state.generation,
            devices,
            next_cursor,
        })
    }

    // ------------------------------------------------------------------------
    // IMMUTABLE SNAPSHOTS
    // ------------------------------------------------------------------------

    /// Captures a consistent immutable inventory snapshot.
    ///
    /// Device descriptors and provider records are captured
    /// under the same read lock.
    ///
    /// The lock is released before the snapshot is returned.
    pub fn snapshot(&self) -> ZhalResult<InventorySnapshot> {
        let state = self.read_state()?;

        let devices = state.devices
            .values()
            .cloned()
            .collect();

        let providers = state.providers
            .values()
            .cloned()
            .collect();

        Ok(InventorySnapshot {
            generation: state.generation,
            devices,
            providers,
        })
    }

    /// Looks up the published inventory record for a provider.
    pub fn provider_record(
        &self,
        provider: &ProviderId,
    ) -> ZhalResult<Option<ProviderInventoryRecord>> {
        Ok(
            self.read_state()?
                .providers
                .get(provider)
                .cloned(),
        )
    }

    // ------------------------------------------------------------------------
    // PROVIDER REFRESH
    // ------------------------------------------------------------------------

    /// Begins a new provider inventory refresh.
    ///
    /// Only one pending refresh is permitted per provider.
    ///
    /// Previously published devices remain visible throughout
    /// the refresh.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider already has an active
    /// refresh or the refresh sequence cannot advance.
    pub fn begin_provider_refresh(
        &self,
        provider: ProviderId,
    ) -> ZhalResult<ProviderRefreshToken> {
        let mut state = self.write_state()?;

        if state.pending.contains_key(&provider) {
            return Err(inventory_error(
                ZhalErrorKind::ConcurrencyConflict,
                "provider already has an active inventory refresh",
            ));
        }

        let sequence = state.next_refresh_sequence
            .checked_add(1)
            .ok_or_else(|| {
                inventory_error(
                    ZhalErrorKind::ResourceExhausted,
                    "provider refresh sequence is exhausted",
                )
            })?;

        let token = ProviderRefreshToken {
            provider_id: provider.clone(),
            sequence,
        };

        state.next_refresh_sequence = sequence;

        state.pending.insert(
            provider,
            PendingProviderRefresh::new(token.clone()),
        );

        Ok(token)
    }

    /// Stages one provider discovery page.
    ///
    /// The page is validated and retained privately.
    ///
    /// No published inventory is modified.
    ///
    /// Pages must arrive in their original provider-defined order.
    ///
    /// Duplicate device identities are rejected.
    ///
    /// The page must belong to the provider associated with
    /// the refresh token.
    ///
    /// # Errors
    ///
    /// Returns an error for:
    ///
    /// - Invalid refresh tokens.
    /// - Wrong provider identity.
    /// - Unexpected page ordering.
    /// - Duplicate device identities.
    /// - Invalid descriptors.
    /// - Inconsistent snapshot revisions.
    /// - Pages received after completion.
    pub fn stage_page(
        &self,
        token: &ProviderRefreshToken,
        page: &DeviceDiscoveryPage,
    ) -> ZhalResult<()> {
        let mut state = self.write_state()?;

        if page.provider_id() != token.provider_id() {
            return Err(inventory_error(
                ZhalErrorKind::ContractViolation,
                "discovery page belongs to a different provider",
            ));
        }

        let pending = state.pending
            .get_mut(token.provider_id())
            .ok_or_else(|| {
                inventory_error(
                    ZhalErrorKind::DeviceNotFound,
                    "provider refresh does not exist",
                )
            })?;

        if pending.token != *token {
            return Err(inventory_error(
                ZhalErrorKind::ConcurrencyConflict,
                "provider refresh token is stale or invalid",
            ));
        }

        if pending.complete {
            return Err(inventory_error(
                ZhalErrorKind::ContractViolation,
                "cannot stage a page after discovery completion",
            ));
        }

        if page.page_index() != pending.next_page_index {
            return Err(inventory_error(
                ZhalErrorKind::ContractViolation,
                "provider discovery pages are not in expected order",
            ));
        }

        let revision = page.snapshot_revision();

        if pending.revision_initialized {
            if pending.snapshot_revision.as_deref() != revision {
                return Err(inventory_error(
                    ZhalErrorKind::VersionMismatch,
                    "provider snapshot revision changed during discovery",
                ));
            }
        } else {
            pending.snapshot_revision = revision.map(str::to_owned);
            pending.revision_initialized = true;
        }

        // Validate the entire page before adding any of its
        // descriptors to the pending refresh.

        for descriptor in page.devices() {
            if descriptor.provider_id() != token.provider_id() {
                return Err(inventory_error(
                    ZhalErrorKind::ContractViolation,
                    "device descriptor belongs to a different provider",
                ));
            }

            descriptor.validate().map_err(|error| {
                inventory_error(
                    ZhalErrorKind::InvalidData,
                    format!(
                        "provider supplied an invalid device descriptor: {error}"
                    ),
                )
            })?;

            if pending.staged.contains_key(descriptor.identity()) {
                return Err(inventory_error(
                    ZhalErrorKind::ContractViolation,
                    "provider discovery contains a duplicate device identity",
                ));
            }
        }

        // Commit the validated page to private staging.

        for descriptor in page.devices() {
            pending.staged.insert(
                descriptor.identity().clone(),
                Arc::new(descriptor.clone()),
            );
        }

        pending.next_page_index =
            pending.next_page_index.checked_add(1).ok_or_else(|| {
                inventory_error(
                    ZhalErrorKind::ResourceExhausted,
                    "provider discovery page counter is exhausted",
                )
            })?;

        pending.complete = page.is_complete();

        Ok(())
    }

    /// Publishes a complete provider inventory atomically.
    ///
    /// Existing devices for this provider are replaced only
    /// after every discovery page has been staged successfully.
    ///
    /// Other providers remain untouched.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The refresh does not exist.
    /// - The token is invalid.
    /// - Discovery is incomplete.
    /// - The inventory generation cannot advance.
    ///
    /// An incomplete refresh must be aborted or continued.
    pub fn commit_provider_refresh(
        &self,
        token: &ProviderRefreshToken,
    ) -> ZhalResult<ProviderInventoryRecord> {
        let mut state = self.write_state()?;

        let pending = state.pending
            .get(token.provider_id())
            .ok_or_else(|| {
                inventory_error(
                    ZhalErrorKind::DeviceNotFound,
                    "provider refresh does not exist",
                )
            })?;

        if pending.token != *token {
            return Err(inventory_error(
                ZhalErrorKind::ConcurrencyConflict,
                "provider refresh token is stale or invalid",
            ));
        }

        if !pending.complete {
            return Err(inventory_error(
                ZhalErrorKind::ContractViolation,
                "cannot publish an incomplete provider inventory",
            ));
        }

        let generation = next_generation(state.generation)?;

        let pending = state.pending.remove(token.provider_id())
            .ok_or_else(|| {
                inventory_error(
                    ZhalErrorKind::Internal,
                    "provider refresh disappeared during publication",
                )
            })?;

        // Remove the old provider inventory.
        //
        // All changes occur under the same write lock, so readers
        // cannot observe a partially replaced provider inventory.

        state.remove_provider_devices(token.provider_id());

        // Insert the complete replacement.

        let device_count = pending.staged.len();

        for descriptor in pending.staged.into_values() {
            state.insert_device(descriptor);
        }

        let record = ProviderInventoryRecord {
            provider_id: token.provider_id().clone(),
            inventory_generation: generation,
            snapshot_revision: pending.snapshot_revision,
            device_count,
        };

        state.providers.insert(
            token.provider_id().clone(),
            record.clone(),
        );

        state.generation = generation;

        Ok(record)
    }

    /// Aborts a pending provider refresh.
    ///
    /// Previously published observations remain unchanged.
    ///
    /// Aborting a refresh does not remove the provider's
    /// existing published inventory.
    pub fn abort_provider_refresh(
        &self,
        token: &ProviderRefreshToken,
    ) -> ZhalResult<()> {
        let mut state = self.write_state()?;

        let pending = state.pending
            .get(token.provider_id())
            .ok_or_else(|| {
                inventory_error(
                    ZhalErrorKind::DeviceNotFound,
                    "provider refresh does not exist",
                )
            })?;

        if pending.token != *token {
            return Err(inventory_error(
                ZhalErrorKind::ConcurrencyConflict,
                "provider refresh token is stale or invalid",
            ));
        }

        state.pending.remove(token.provider_id());

        Ok(())
    }

    // ------------------------------------------------------------------------
    // DIRECT COMPLETE SNAPSHOT PUBLICATION
    // ------------------------------------------------------------------------

    /// Atomically replaces one provider's complete device snapshot.
    ///
    /// This method is useful when a trusted upstream discovery
    /// coordinator has already assembled and validated the
    /// complete provider snapshot.
    ///
    /// It must not be used to publish an incomplete discovery page.
    ///
    /// All descriptors must belong to the supplied provider.
    ///
    /// Duplicate identities are rejected.
    ///
    /// The previous published inventory remains unchanged
    /// if validation fails.
    pub fn replace_provider_snapshot(
        &self,
        provider: ProviderId,
        devices: impl IntoIterator<Item = HardwareDeviceDescriptor>,
        snapshot_revision: Option<String>,
    ) -> ZhalResult<ProviderInventoryRecord> {
        let mut staged = BTreeMap::new();

        for descriptor in devices {
            if descriptor.provider_id() != &provider {
                return Err(inventory_error(
                    ZhalErrorKind::ContractViolation,
                    "device descriptor belongs to a different provider",
                ));
            }

            descriptor.validate().map_err(|error| {
                inventory_error(
                    ZhalErrorKind::InvalidData,
                    format!("invalid device descriptor: {error}"),
                )
            })?;

            let identity = descriptor.identity().clone();

            if staged.contains_key(&identity) {
                return Err(inventory_error(
                    ZhalErrorKind::ContractViolation,
                    "provider snapshot contains duplicate device identities",
                ));
            }

            staged.insert(
                identity,
                Arc::new(descriptor),
            );
        }

        let mut state = self.write_state()?;

        if state.pending.contains_key(&provider) {
            return Err(inventory_error(
                ZhalErrorKind::ConcurrencyConflict,
                "cannot directly replace a provider inventory during an active refresh",
            ));
        }

        let generation = next_generation(state.generation)?;

        state.remove_provider_devices(&provider);

        let device_count = staged.len();

        for descriptor in staged.into_values() {
            state.insert_device(descriptor);
        }

        let record = ProviderInventoryRecord {
            provider_id: provider.clone(),
            inventory_generation: generation,
            snapshot_revision,
            device_count,
        };

        state.providers.insert(provider, record.clone());

        state.generation = generation;

        Ok(record)
    }

    // ------------------------------------------------------------------------
    // PROVIDER REMOVAL
    // ------------------------------------------------------------------------

    /// Removes one provider's published inventory.
    ///
    /// Pending refreshes are not implicitly aborted.
    ///
    /// Provider lifecycle and shutdown remain owned by the
    /// canonical provider lifecycle authority.
    ///
    /// Returns whether a published provider inventory existed.
    pub fn remove_provider(
        &self,
        provider: &ProviderId,
    ) -> ZhalResult<bool> {
        let mut state = self.write_state()?;

        if !state.providers.contains_key(provider) {
            return Ok(false);
        }

        let generation = next_generation(state.generation)?;

        state.remove_provider_devices(provider);

        state.providers.remove(provider);

        state.generation = generation;

        Ok(true)
    }

    // ------------------------------------------------------------------------
    // INTEGRITY VALIDATION
    // ------------------------------------------------------------------------

    /// Validates internal inventory indexes and provider records.
    ///
    /// This is a consistency check, not hardware attestation.
    pub fn validate_integrity(&self) -> ZhalResult<()> {
        let state = self.read_state()?;

        if !state.rebuild_indexes_are_consistent() {
            return Err(inventory_error(
                ZhalErrorKind::ContractViolation,
                "hardware inventory indexes are inconsistent",
            ));
        }

        for (provider, record) in &state.providers {
            if provider != record.provider_id() {
                return Err(inventory_error(
                    ZhalErrorKind::ContractViolation,
                    "provider inventory record has an inconsistent identity",
                ));
            }

            let observed_count = state.provider_index
                .get(provider)
                .map_or(0, BTreeSet::len);

            if observed_count != record.device_count() {
                return Err(inventory_error(
                    ZhalErrorKind::ContractViolation,
                    "provider inventory device count is inconsistent",
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// QUERY INDEX SELECTION
// ============================================================================

fn candidate_identities(
    state: &InventoryState,
    query: &InventoryQuery,
) -> BTreeSet<HardwareDeviceIdentity> {
    // Select the smallest applicable indexed candidate set.
    //
    // Additional criteria are evaluated against the resulting
    // descriptors to preserve exact AND semantics.

    let mut candidates: Option<&BTreeSet<HardwareDeviceIdentity>> = None;

    if let Some(provider) = &query.provider {
        candidates = state.provider_index.get(provider);
    }

    if let Some(category) = &query.category {
        if let Some(index) = state.category_index.get(category) {
            if candidates.is_none_or(|current| index.len() < current.len()) {
                candidates = Some(index);
            }
        } else {
            return BTreeSet::new();
        }
    }

    if let Some(availability) = &query.availability_state {
        if let Some(index) = state.availability_index.get(availability) {
            if candidates.is_none_or(|current| index.len() < current.len()) {
                candidates = Some(index);
            }
        } else {
            return BTreeSet::new();
        }
    }

    if let Some(snapshot) = &query.capability_snapshot {
        if let Some(index) = state.capability_snapshot_index.get(snapshot) {
            if candidates.is_none_or(|current| index.len() < current.len()) {
                candidates = Some(index);
            }
        } else {
            return BTreeSet::new();
        }
    }

    match candidates {
        Some(index) => index.clone(),
        None => state.devices.keys().cloned().collect(),
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::io::devices::DeviceId;

    use crate::zhal::identity::HardwareDeviceIdentity;

    fn provider(value: &str) -> ProviderId {
        ProviderId::new(value).expect("valid provider")
    }

    fn category(value: &str) -> DeviceKindId {
        DeviceKindId::new(value).expect("valid category")
    }

    fn descriptor(
        provider_name: &str,
        device_name: &str,
        category_name: &str,
    ) -> HardwareDeviceDescriptor {
        let identity = HardwareDeviceIdentity::new(
            provider(provider_name),
            DeviceId::new(device_name).expect("valid device ID"),
        );

        HardwareDeviceDescriptor::new(
            identity,
            category(category_name),
            device_name,
        )
        .expect("valid device descriptor")
    }

    #[test]
    fn creates_empty_inventory() {
        let inventory = HardwareDeviceInventory::new();

        assert_eq!(inventory.len().unwrap(), 0);
        assert!(inventory.is_empty().unwrap());
        assert_eq!(inventory.generation().unwrap(), 0);
    }

    #[test]
    fn publishes_complete_provider_snapshot() {
        let inventory = HardwareDeviceInventory::new();

        let record = inventory.replace_provider_snapshot(
            provider("provider.classical"),
            vec![
                descriptor(
                    "provider.classical",
                    "cpu.0",
                    "processor",
                ),
                descriptor(
                    "provider.classical",
                    "cpu.1",
                    "processor",
                ),
            ],
            Some("revision.1".to_owned()),
        ).unwrap();

        assert_eq!(record.device_count(), 2);
        assert_eq!(inventory.len().unwrap(), 2);
        assert_eq!(inventory.generation().unwrap(), 1);
        inventory.validate_integrity().unwrap();
    }

    #[test]
    fn provider_identity_prevents_device_id_collisions() {
        let inventory = HardwareDeviceInventory::new();

        inventory.replace_provider_snapshot(
            provider("provider.one"),
            vec![descriptor(
                "provider.one",
                "device.0",
                "processor",
            )],
            None,
        ).unwrap();

        inventory.replace_provider_snapshot(
            provider("provider.two"),
            vec![descriptor(
                "provider.two",
                "device.0",
                "accelerator",
            )],
            None,
        ).unwrap();

        assert_eq!(inventory.len().unwrap(), 2);
    }

    #[test]
    fn provider_replacement_removes_stale_devices() {
        let inventory = HardwareDeviceInventory::new();

        let id = provider("provider.test");

        inventory.replace_provider_snapshot(
            id.clone(),
            vec![
                descriptor("provider.test", "device.0", "processor"),
                descriptor("provider.test", "device.1", "processor"),
            ],
            None,
        ).unwrap();

        inventory.replace_provider_snapshot(
            id.clone(),
            vec![
                descriptor("provider.test", "device.0", "processor"),
            ],
            None,
        ).unwrap();

        assert_eq!(
            inventory.devices_by_provider(&id).unwrap().len(),
            1,
        );

        inventory.validate_integrity().unwrap();
    }

    #[test]
    fn failed_validation_preserves_previous_inventory() {
        let inventory = HardwareDeviceInventory::new();

        let id = provider("provider.test");

        inventory.replace_provider_snapshot(
            id.clone(),
            vec![
                descriptor("provider.test", "device.0", "processor"),
            ],
            None,
        ).unwrap();

        let before = inventory.snapshot().unwrap();

        let invalid = descriptor(
            "provider.other",
            "device.1",
            "processor",
        );

        assert!(
            inventory.replace_provider_snapshot(
                id,
                vec![invalid],
                None,
            ).is_err()
        );

        let after = inventory.snapshot().unwrap();

        assert_eq!(before.generation(), after.generation());
        assert_eq!(before.len(), after.len());
    }

    #[test]
    fn category_index_returns_correct_devices() {
        let inventory = HardwareDeviceInventory::new();

        inventory.replace_provider_snapshot(
            provider("provider.test"),
            vec![
                descriptor("provider.test", "cpu.0", "processor"),
                descriptor("provider.test", "gpu.0", "accelerator"),
            ],
            None,
        ).unwrap();

        assert_eq!(
            inventory.devices_by_category(
                &category("processor"),
            ).unwrap().len(),
            1,
        );
    }

    #[test]
    fn provider_refresh_preserves_published_inventory_until_commit() {
        let inventory = HardwareDeviceInventory::new();

        let provider_id = provider("provider.test");

        inventory.replace_provider_snapshot(
            provider_id.clone(),
            vec![
                descriptor("provider.test", "old.device", "processor"),
            ],
            None,
        ).unwrap();

        let token = inventory.begin_provider_refresh(
            provider_id.clone(),
        ).unwrap();

        assert_eq!(inventory.len().unwrap(), 1);

        inventory.abort_provider_refresh(&token).unwrap();

        assert_eq!(inventory.len().unwrap(), 1);
        assert!(
            inventory.get(
                &descriptor(
                    "provider.test",
                    "old.device",
                    "processor",
                ).identity().clone()
            ).unwrap().is_some()
        );
    }

    #[test]
    fn refresh_tokens_are_provider_scoped() {
        let inventory = HardwareDeviceInventory::new();

        let token = inventory.begin_provider_refresh(
            provider("provider.one"),
        ).unwrap();

        assert_eq!(
            token.provider_id().as_str(),
            "provider.one",
        );

        inventory.abort_provider_refresh(&token).unwrap();
    }

    #[test]
    fn concurrent_refreshes_for_same_provider_are_rejected() {
        let inventory = HardwareDeviceInventory::new();

        let id = provider("provider.test");

        let token = inventory.begin_provider_refresh(
            id.clone(),
        ).unwrap();

        assert!(
            inventory.begin_provider_refresh(id).is_err()
        );

        inventory.abort_provider_refresh(&token).unwrap();
    }

    #[test]
    fn provider_removal_updates_indexes() {
        let inventory = HardwareDeviceInventory::new();

        let id = provider("provider.test");

        inventory.replace_provider_snapshot(
            id.clone(),
            vec![
                descriptor("provider.test", "device.0", "processor"),
            ],
            None,
        ).unwrap();

        assert!(inventory.remove_provider(&id).unwrap());
        assert_eq!(inventory.len().unwrap(), 0);
        assert_eq!(inventory.provider_count().unwrap(), 0);

        inventory.validate_integrity().unwrap();
    }

    #[test]
    fn pagination_returns_deterministic_results() {
        let inventory = HardwareDeviceInventory::new();

        inventory.replace_provider_snapshot(
            provider("provider.test"),
            vec![
                descriptor("provider.test", "device.a", "processor"),
                descriptor("provider.test", "device.b", "processor"),
                descriptor("provider.test", "device.c", "processor"),
            ],
            None,
        ).unwrap();

        let query = InventoryQuery::new();

        let first = inventory.query_page(
            &query,
            NonZeroUsize::new(2).unwrap(),
            None,
        ).unwrap();

        assert_eq!(first.len(), 2);
        assert!(first.has_more());

        let second = inventory.query_page(
            &query,
            NonZeroUsize::new(2).unwrap(),
            first.next_cursor(),
        ).unwrap();

        assert_eq!(second.len(), 1);
        assert!(!second.has_more());
    }

    #[test]
    fn inventory_mutation_invalidates_old_cursors() {
        let inventory = HardwareDeviceInventory::new();

        inventory.replace_provider_snapshot(
            provider("provider.test"),
            vec![
                descriptor("provider.test", "device.a", "processor"),
                descriptor("provider.test", "device.b", "processor"),
            ],
            None,
        ).unwrap();

        let first = inventory.query_page(
            &InventoryQuery::new(),
            NonZeroUsize::new(1).unwrap(),
            None,
        ).unwrap();

        inventory.replace_provider_snapshot(
            provider("provider.test"),
            vec![
                descriptor("provider.test", "device.c", "processor"),
            ],
            None,
        ).unwrap();

        assert!(
            inventory.query_page(
                &InventoryQuery::new(),
                NonZeroUsize::new(1).unwrap(),
                first.next_cursor(),
            ).is_err()
        );
    }

    #[test]
    fn empty_provider_snapshot_is_valid() {
        let inventory = HardwareDeviceInventory::new();

        inventory.replace_provider_snapshot(
            provider("provider.test"),
            Vec::new(),
            None,
        ).unwrap();

        assert_eq!(inventory.len().unwrap(), 0);
        assert_eq!(inventory.provider_count().unwrap(), 1);
    }

    #[test]
    fn inventory_supports_open_world_categories() {
        let inventory = HardwareDeviceInventory::new();

        inventory.replace_provider_snapshot(
            provider("provider.future"),
            vec![
                descriptor(
                    "provider.future",
                    "photonic.0",
                    "future.photonic.processor",
                ),
            ],
            None,
        ).unwrap();

        assert_eq!(
            inventory.devices_by_category(
                &category("future.photonic.processor"),
            ).unwrap().len(),
            1,
        );
    }

    #[test]
    fn snapshots_remain_immutable_after_inventory_changes() {
        let inventory = HardwareDeviceInventory::new();

        inventory.replace_provider_snapshot(
            provider("provider.test"),
            vec![
                descriptor("provider.test", "device.0", "processor"),
            ],
            None,
        ).unwrap();

        let snapshot = inventory.snapshot().unwrap();

        inventory.remove_provider(
            &provider("provider.test"),
        ).unwrap();

        assert_eq!(snapshot.len(), 1);
        assert_eq!(inventory.len().unwrap(), 0);
    }

    #[test]
    fn clones_share_inventory_state() {
        let first = HardwareDeviceInventory::new();
        let second = first.clone();

        first.replace_provider_snapshot(
            provider("provider.test"),
            vec![
                descriptor("provider.test", "device.0", "processor"),
            ],
            None,
        ).unwrap();

        assert_eq!(second.len().unwrap(), 1);
    }

    #[test]
    fn query_combines_multiple_constraints() {
        let inventory = HardwareDeviceInventory::new();

        inventory.replace_provider_snapshot(
            provider("provider.test"),
            vec![
                descriptor("provider.test", "cpu.0", "processor"),
                descriptor("provider.test", "gpu.0", "accelerator"),
            ],
            None,
        ).unwrap();

        let query = InventoryQuery::new()
            .with_provider(provider("provider.test"))
            .with_category(category("processor"));

        assert_eq!(
            inventory.query(&query).unwrap().len(),
            1,
        );
    }

    #[test]
    fn inventory_integrity_is_valid_after_replacements() {
        let inventory = HardwareDeviceInventory::new();

        let id = provider("provider.test");

        for index in 0..32 {
            inventory.replace_provider_snapshot(
                id.clone(),
                vec![
                    descriptor(
                        "provider.test",
                        &format!("device.{index}"),
                        "processor",
                    ),
                ],
                Some(format!("revision.{index}")),
            ).unwrap();

            inventory.validate_integrity().unwrap();
        }

        assert_eq!(inventory.len().unwrap(), 1);
    }

    #[test]
    fn inventory_has_no_fixed_device_count_ceiling() {
        let inventory = HardwareDeviceInventory::new();

        let devices = (0..1024)
            .map(|index| {
                descriptor(
                    "provider.test",
                    &format!("device.{index}"),
                    "processor",
                )
            })
            .collect::<Vec<_>>();

        inventory.replace_provider_snapshot(
            provider("provider.test"),
            devices,
            None,
        ).unwrap();

        assert_eq!(inventory.len().unwrap(), 1024);
        inventory.validate_integrity().unwrap();
    }
}
