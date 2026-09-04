//! Zamani Quantum Scheduling — Resource Pool
//!
//! This module defines the scheduler's scalable resource-pool abstraction.
//!
//! # Architectural responsibility
//!
//! A [`ResourcePool`] represents a collection of scheduler-visible resources
//! and their currently configured capacities.
//!
//! The pool answers:
//!
//! > "Which resources exist in this scheduling context, and how much
//! > concurrent capacity does each resource provide?"
//!
//! It does NOT answer:
//!
//! - when a resource is free;
//! - which operation owns a reservation;
//! - how reservations overlap;
//! - how hardware is discovered;
//! - how calibration is obtained;
//! - how routing is performed;
//! - how a schedule is constructed;
//! - how a QPU is executed;
//! - how quantum semantics are represented.
//!
//! Those responsibilities belong to other scheduler and quantum subsystems.
//!
//! # Ownership boundaries
//!
//! Resource identity remains owned by the canonical Quantum IR identity layer:
//!
//! ```text
//! crate::quantum::ir::core::identity::ResourceId
//! ```
//!
//! Logical and physical qubit identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module deliberately does not create replacement identity types.
//!
//! # Relationship to the scheduler architecture
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir::core::identity::ResourceId
//!                         │
//!                         ▼
//!              scheduling::resources::pool
//!                         │
//!          ┌──────────────┼──────────────┐
//!          ▼              ▼              ▼
//!      resource      reservation      calendar
//!          │              │              │
//!          └──────────────┼──────────────┘
//!                         ▼
//!                       planner
//!                         │
//!                         ▼
//!                    verification
//! ```
//!
//! The pool is therefore deliberately lower-level than reservation and
//! scheduling algorithms.
//!
//! # Resource model
//!
//! A resource is represented by:
//!
//! ```text
//! ResourceId
//!     + capacity
//!     + scheduling availability
//!     + optional metadata
//! ```
//!
//! Capacity is expressed as a non-negative integer quantity.
//!
//! A capacity of:
//!
//! ```text
//! 1
//! ```
//!
//! represents an exclusive resource.
//!
//! A capacity greater than one represents a resource capable of serving
//! multiple compatible simultaneous users, subject to reservation policy.
//!
//! A capacity of zero represents a known resource that is currently unusable.
//!
//! A zero-capacity resource remains representable because target descriptions
//! may legitimately contain disabled, unavailable, reserved, or degraded
//! resources.
//!
//! # Important distinction: capacity is not availability
//!
//! `ResourcePool` stores configured capacity.
//!
//! It does NOT store time-dependent availability.
//!
//! For example:
//!
//! ```text
//! ResourcePool:
//!     readout-0 -> capacity 4
//!
//! ResourceCalendar:
//!     readout-0 -> unavailable during [100, 200)
//! ```
//!
//! Keeping these concerns separate prevents the pool from becoming a hidden
//! scheduling calendar.
//!
//! # Scalability
//!
//! This module intentionally contains no constants for:
//!
//! - maximum resources;
//! - maximum qubits;
//! - maximum operations;
//! - maximum resource capacity;
//! - maximum machines;
//! - maximum resource kinds;
//! - maximum resource pools;
//! - maximum scheduling horizon.
//!
//! The implementation scales according to the resources supplied by the
//! compilation target and the memory available to the host process.
//!
//! "Infinity" in Zamani therefore means:
//!
//! > the scheduler contains no artificial finite machine-size ceiling.
//!
//! A particular compilation remains bounded by physical realities such as
//! address space, memory, execution time, operating-system limits, explicit
//! user limits, and target capacity.
//!
//! # Determinism
//!
//! Resource iteration is deterministic.
//!
//! The pool uses `BTreeMap` rather than `HashMap` so that:
//!
//! - iteration order is stable;
//! - diagnostics are reproducible;
//! - serialized traversal can be deterministic;
//! - scheduler algorithms can request deterministic resource enumeration;
//! - tests do not depend on randomized hash state.
//!
//! # Transactional mutation
//!
//! Mutating operations validate their input before changing the pool.
//!
//! In particular:
//!
//! - duplicate resource insertion is rejected;
//! - removal of an unknown resource is rejected;
//! - capacity updates require an existing resource;
//! - invalid batch mutations do not partially modify the pool.
//!
//! This is important because future reservation and scheduling layers may keep
//! references to a pool snapshot. Partial mutations would make those snapshots
//! difficult to reason about.
//!
//! # Thread safety
//!
//! `ResourcePool` contains ordinary owned Rust data and no interior
//! mutability.
//!
//! It does not create locks or global state.
//!
//! Callers that need concurrent read access may place an immutable pool inside
//! an appropriate synchronization primitive such as `Arc`/`RwLock`.
//!
//! The pool itself remains synchronization-agnostic.
//!
//! # No hardware I/O
//!
//! This module must never:
//!
//! - connect to a QPU;
//! - query a provider;
//! - fetch calibration;
//! - inspect device state;
//! - perform authentication;
//! - submit jobs.
//!
//! Hardware adapters construct or update the scheduler's target/resource
//! snapshot before planning.
//!
//! # No scheduling logic
//!
//! The pool must never decide:
//!
//! - ASAP vs ALAP;
//! - critical-path priority;
//! - operation ordering;
//! - routing;
//! - gate decomposition;
//! - QEC rounds;
//! - dynamic feedback.
//!
//! It only provides resource inventory and capacity information.
//!
//! # Resource identity
//!
//! The canonical [`ResourceId`] is imported from:
//!
//! ```text
//! crate::quantum::ir::core::identity::ResourceId
//! ```
//!
//! No local `ResourceId` alias is created.
//!
//! This follows the repository-wide identity contract.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust.
//!
//! No nightly features.
//! No `unsafe`.
//! No `unsafe` dependencies are required by this module.
//!
//! # Integration contract
//!
//! Hardware integration:
//!
//! ```text
//! quantum::hardware
//!       │
//!       ▼
//! hardware adapter
//!       │
//!       ├── canonical ResourceId
//!       ├── target capacity
//!       └── target metadata
//!       │
//!       ▼
//! ResourcePool::try_from_entries
//!       │
//!       ▼
//! SchedulingContext
//! ```
//!
//! Operation/resource requirements:
//!
//! ```text
//! scheduling::ir::operation
//!       │
//!       ▼
//! ResourceRequirement
//!       │
//!       ▼
//! ResourcePool::capacity
//! ```
//!
//! Reservation:
//!
//! ```text
//! ResourcePool
//!       │
//!       ▼
//! ResourceCalendar
//!       │
//!       ▼
//! ResourceReservation
//! ```
//!
//! Planner:
//!
//! ```text
//! Planner
//!   │
//!   ├── reads ResourcePool
//!   ├── checks resource capacity
//!   └── creates reservations elsewhere
//! ```
//!
//! Verification:
//!
//! ```text
//! ResourcePool
//!       │
//!       ▼
//! ResourceVerifier
//!       │
//!       ▼
//! reservation usage <= configured capacity
//! ```
//!
//! # Future extension rule
//!
//! Future resource metadata must not be added to this file merely because it
//! is convenient.
//!
//! If a property has independent semantics, create a dedicated resource model
//! in `resource.rs`.
//!
//! If a property changes over time, put it in `availability.rs` or
//! `calendar.rs`.
//!
//! If a property describes a reservation, put it in `reservation.rs`.
//!
//! If a property describes hardware capability, keep it in the hardware
//! subsystem and expose it through an adapter.
//!
//! # Security
//!
//! Resource identifiers are opaque semantic identifiers.
//!
//! They must not be interpreted as:
//!
//! - memory addresses;
//! - pointers;
//! - array indexes unless explicitly converted by a caller;
//! - filesystem paths;
//! - provider credentials;
//! - network addresses.
//!
//! This module performs no unsafe memory access.
//!
//! # Invariants
//!
//! A valid `ResourcePool` always satisfies:
//!
//! 1. Every resource identifier occurs at most once.
//! 2. Every capacity is a non-negative integer.
//! 3. Resource count is represented by the actual collection size.
//! 4. No machine-size constant limits resource count.
//! 5. Removing a resource removes its pool entry completely.
//! 6. Updating a resource cannot create a duplicate resource.
//! 7. Failed mutations do not partially modify the pool.
//! 8. Resource identity remains the canonical IR identity.
//! 9. The pool contains no time-dependent reservation state.
//! 10. The pool performs no hardware I/O.
//! 11. The pool contains no scheduling policy.
//! 12. The pool contains no global mutable state.
//!
//! # Why this file is foundational
//!
//! `ResourcePool` is intentionally implemented before:
//!
//! - `reservation.rs`;
//! - `calendar.rs`;
//! - `availability.rs`;
//! - resource-constrained planners.
//!
//! Those modules need a stable answer to the foundational question:
//!
//! > "What resources and capacities exist?"
//!
//! They should not need to modify this file merely because they are added.
//!
//! # API stability
//!
//! The following concepts form the stable core of this module:
//!
//! ```text
//! ResourcePool
//! ResourcePoolEntry
//! ResourcePoolError
//! ResourcePoolSnapshot
//! ResourcePoolBuilder
//! ```
//!
//! Additional resource metadata should be introduced through composition rather
//! than by changing the meaning of capacity.
//!
//! # Example
//!
//! ```rust
//! use crate::quantum::ir::core::identity::ResourceId;
//! use crate::quantum::scheduling::resources::pool::ResourcePool;
//!
//! let mut pool = ResourcePool::new();
//!
//! pool.insert(ResourceId::new(0), 1).unwrap();
//! pool.insert(ResourceId::new(1), 2).unwrap();
//!
//! assert_eq!(pool.capacity(ResourceId::new(0)), Some(1));
//! assert_eq!(pool.capacity(ResourceId::new(1)), Some(2));
//! assert_eq!(pool.len(), 2);
//! ```
//!
//! The example deliberately does not assume a fixed number of qubits,
//! channels, processors, or devices.
//!
//! # Design conclusion
//!
//! The pool is a declarative resource inventory.
//!
//! It is deliberately boring.
//!
//! That is a feature.
//!
//! Scheduling algorithms should be sophisticated because they consume this
//! abstraction—not because resource inventory is mixed into the algorithm.
//!

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::btree_map;
use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::ir::core::identity::ResourceId;

/// Maximum representable resource capacity is determined by `u64`.
///
/// This is NOT a machine-size limit. It is the numeric representation of one
/// capacity value.
///
/// A scheduler requiring a larger semantic quantity should use a different
/// quantity abstraction at the resource-model boundary rather than silently
/// overflowing this type.
pub type ResourceCapacity = u64;

/// One entry in a [`ResourcePool`].
///
/// This type deliberately contains only the invariant resource information
/// owned by the pool: canonical identity and configured capacity.
///
/// Time-dependent availability belongs elsewhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourcePoolEntry {
    resource: ResourceId,
    capacity: ResourceCapacity,
}

impl ResourcePoolEntry {
    /// Creates a resource-pool entry.
    #[must_use]
    pub const fn new(
        resource: ResourceId,
        capacity: ResourceCapacity,
    ) -> Self {
        Self {
            resource,
            capacity,
        }
    }

    /// Returns the canonical resource identity.
    #[must_use]
    pub const fn resource(self) -> ResourceId {
        self.resource
    }

    /// Returns configured concurrent capacity.
    #[must_use]
    pub const fn capacity(self) -> ResourceCapacity {
        self.capacity
    }

    /// Returns whether the configured capacity is zero.
    #[must_use]
    pub const fn is_disabled(self) -> bool {
        self.capacity == 0
    }
}

impl fmt::Display for ResourcePoolEntry {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{} capacity={}",
            self.resource,
            self.capacity
        )
    }
}

/// Errors produced by resource-pool operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourcePoolError {
    /// An insertion attempted to add a resource that already exists.
    DuplicateResource {
        resource: ResourceId,
    },

    /// An operation referred to a resource that is not in the pool.
    UnknownResource {
        resource: ResourceId,
    },

    /// A batch contains the same resource more than once.
    DuplicateResourceInBatch {
        resource: ResourceId,
    },

    /// A batch update referred to a resource that does not exist.
    MissingResourceInBatch {
        resource: ResourceId,
    },
}

impl fmt::Display for ResourcePoolError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::DuplicateResource { resource } => {
                write!(
                    formatter,
                    "resource {} already exists in the resource pool",
                    resource
                )
            }
            Self::UnknownResource { resource } => {
                write!(
                    formatter,
                    "resource {} does not exist in the resource pool",
                    resource
                )
            }
            Self::DuplicateResourceInBatch { resource } => {
                write!(
                    formatter,
                    "resource {} occurs more than once in the resource batch",
                    resource
                )
            }
            Self::MissingResourceInBatch { resource } => {
                write!(
                    formatter,
                    "resource {} required by the batch does not exist",
                    resource
                )
            }
        }
    }
}

impl std::error::Error for ResourcePoolError {}

/// Immutable snapshot of a [`ResourcePool`].
///
/// A snapshot is useful when constructing a [`SchedulingContext`] because the
/// planner can consume a stable target/resource view without owning mutable
/// hardware state.
///
/// The snapshot owns its entries and therefore remains valid independently of
/// the source pool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourcePoolSnapshot {
    entries: BTreeMap<ResourceId, ResourceCapacity>,
}

impl ResourcePoolSnapshot {
    /// Creates an empty snapshot.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    /// Creates a snapshot from entries.
    ///
    /// This constructor is private to the pool so that snapshot invariants are
    /// established through validated pool operations.
    fn from_entries(
        entries: BTreeMap<ResourceId, ResourceCapacity>,
    ) -> Self {
        Self { entries }
    }

    /// Returns the number of resources.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the snapshot contains no resources.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the configured capacity of a resource.
    #[must_use]
    pub fn capacity(
        &self,
        resource: ResourceId,
    ) -> Option<ResourceCapacity> {
        self.entries.get(&resource).copied()
    }

    /// Returns whether a resource exists.
    #[must_use]
    pub fn contains(&self, resource: ResourceId) -> bool {
        self.entries.contains_key(&resource)
    }

    /// Returns an iterator over resources in deterministic order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = ResourcePoolEntry> + '_ {
        self.entries
            .iter()
            .map(|(&resource, &capacity)| {
                ResourcePoolEntry::new(resource, capacity)
            })
    }

    /// Returns the total configured capacity using checked arithmetic.
    ///
    /// `None` means that the mathematical total cannot be represented by
    /// `u64`.
    #[must_use]
    pub fn checked_total_capacity(&self) -> Option<ResourceCapacity> {
        self.entries.values().try_fold(0_u64, |total, capacity| {
            total.checked_add(*capacity)
        })
    }
}

impl Default for ResourcePoolSnapshot {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for constructing a [`ResourcePool`] transactionally.
///
/// The builder is useful for hardware adapters and target descriptions that
/// discover their resource inventory before constructing a scheduling context.
///
/// Duplicate identities are rejected during insertion.
///
/// The builder never partially constructs a pool.
#[derive(Debug, Clone, Default)]
pub struct ResourcePoolBuilder {
    entries: BTreeMap<ResourceId, ResourceCapacity>,
}

impl ResourcePoolBuilder {
    /// Creates an empty builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds one resource to the builder.
    ///
    /// Returns an error if the resource already exists.
    pub fn insert(
        &mut self,
        resource: ResourceId,
        capacity: ResourceCapacity,
    ) -> Result<(), ResourcePoolError> {
        if self.entries.contains_key(&resource) {
            return Err(ResourcePoolError::DuplicateResource { resource });
        }

        self.entries.insert(resource, capacity);
        Ok(())
    }

    /// Adds a resource and returns the builder for fluent construction.
    ///
    /// This method intentionally returns `Result<Self, ...>` rather than
    /// silently replacing a previous resource.
    pub fn with_resource(
        mut self,
        resource: ResourceId,
        capacity: ResourceCapacity,
    ) -> Result<Self, ResourcePoolError> {
        self.insert(resource, capacity)?;
        Ok(self)
    }

    /// Builds the immutable pool.
    #[must_use]
    pub fn build(self) -> ResourcePool {
        ResourcePool {
            entries: self.entries,
        }
    }

    /// Returns the number of resources currently staged.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether no resources have been staged.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Scalable inventory of scheduling resources.
///
/// `ResourcePool` owns resource identities and configured capacities.
///
/// It does not own temporal reservations.
///
/// # Complexity
///
/// Let `N` be the number of resources in the pool.
///
/// - lookup: `O(log N)`
/// - insertion: `O(log N)`
/// - removal: `O(log N)`
/// - capacity update: `O(log N)`
/// - deterministic iteration: `O(N)`
/// - snapshot creation: `O(N)`
///
/// The implementation intentionally favors deterministic behavior and strong
/// invariants over hash-table average-case lookup.
///
/// The pool has no artificial upper bound on `N`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourcePool {
    entries: BTreeMap<ResourceId, ResourceCapacity>,
}

impl ResourcePool {
    /// Creates an empty resource pool.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    /// Creates a pool from an iterator of `(ResourceId, capacity)` entries.
    ///
    /// The entire input is validated before the resulting pool is returned.
    ///
    /// This is useful for hardware adapters because they can prepare their
    /// complete target description without exposing a partially built pool to
    /// the scheduler.
    pub fn try_from_entries<I>(
        entries: I,
    ) -> Result<Self, ResourcePoolError>
    where
        I: IntoIterator<Item = (ResourceId, ResourceCapacity)>,
    {
        let mut builder = Self::builder();

        for (resource, capacity) in entries {
            builder.insert(resource, capacity)?;
        }

        Ok(builder.build())
    }

    /// Creates a builder for transactional pool construction.
    #[must_use]
    pub fn builder() -> ResourcePoolBuilder {
        ResourcePoolBuilder::new()
    }

    /// Returns the number of resources in the pool.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the pool contains no resources.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns whether the pool contains a particular resource.
    #[must_use]
    pub fn contains(
        &self,
        resource: ResourceId,
    ) -> bool {
        self.entries.contains_key(&resource)
    }

    /// Returns the configured capacity of a resource.
    ///
    /// `None` means the resource is not present.
    #[must_use]
    pub fn capacity(
        &self,
        resource: ResourceId,
    ) -> Option<ResourceCapacity> {
        self.entries.get(&resource).copied()
    }

    /// Returns an entry describing one resource.
    #[must_use]
    pub fn entry(
        &self,
        resource: ResourceId,
    ) -> Option<ResourcePoolEntry> {
        self.entries
            .get(&resource)
            .copied()
            .map(|capacity| ResourcePoolEntry::new(resource, capacity))
    }

    /// Inserts a new resource.
    ///
    /// Existing resources are never silently replaced.
    pub fn insert(
        &mut self,
        resource: ResourceId,
        capacity: ResourceCapacity,
    ) -> Result<(), ResourcePoolError> {
        if self.entries.contains_key(&resource) {
            return Err(ResourcePoolError::DuplicateResource { resource });
        }

        self.entries.insert(resource, capacity);
        Ok(())
    }

    /// Updates an existing resource's capacity.
    ///
    /// This operation cannot create a new resource accidentally.
    pub fn set_capacity(
        &mut self,
        resource: ResourceId,
        capacity: ResourceCapacity,
    ) -> Result<(), ResourcePoolError> {
        match self.entries.get_mut(&resource) {
            Some(existing) => {
                *existing = capacity;
                Ok(())
            }
            None => Err(ResourcePoolError::UnknownResource { resource }),
        }
    }

    /// Removes a resource and returns its previous entry.
    pub fn remove(
        &mut self,
        resource: ResourceId,
    ) -> Result<ResourcePoolEntry, ResourcePoolError> {
        match self.entries.remove(&resource) {
            Some(capacity) => {
                Ok(ResourcePoolEntry::new(resource, capacity))
            }
            None => Err(ResourcePoolError::UnknownResource { resource }),
        }
    }

    /// Removes a resource if present.
    ///
    /// This is intentionally distinct from [`Self::remove`]:
    /// callers performing cleanup can explicitly choose idempotent behavior.
    pub fn remove_if_present(
        &mut self,
        resource: ResourceId,
    ) -> Option<ResourcePoolEntry> {
        self.entries
            .remove(&resource)
            .map(|capacity| ResourcePoolEntry::new(resource, capacity))
    }

    /// Returns an iterator over entries in canonical deterministic order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = ResourcePoolEntry> + '_ {
        self.entries
            .iter()
            .map(|(&resource, &capacity)| {
                ResourcePoolEntry::new(resource, capacity)
            })
    }

    /// Returns an iterator over canonical resource identifiers in deterministic
    /// order.
    pub fn resource_ids(
        &self,
    ) -> impl Iterator<Item = ResourceId> + '_ {
        self.entries.keys().copied()
    }

    /// Returns the total configured capacity using checked arithmetic.
    ///
    /// `None` indicates that the mathematical total cannot be represented by
    /// `u64`.
    ///
    /// This method does not change the pool and never wraps.
    #[must_use]
    pub fn checked_total_capacity(&self) -> Option<ResourceCapacity> {
        self.entries.values().try_fold(0_u64, |total, capacity| {
            total.checked_add(*capacity)
        })
    }

    /// Returns an immutable snapshot of this pool.
    ///
    /// The snapshot owns its underlying map and can therefore be moved into a
    /// scheduling context without retaining mutable access to the source pool.
    #[must_use]
    pub fn snapshot(&self) -> ResourcePoolSnapshot {
        ResourcePoolSnapshot::from_entries(self.entries.clone())
    }

    /// Returns the number of resources whose configured capacity is non-zero.
    #[must_use]
    pub fn active_resource_count(&self) -> usize {
        self.entries
            .values()
            .filter(|capacity| **capacity != 0)
            .count()
    }

    /// Returns the number of resources whose configured capacity is zero.
    #[must_use]
    pub fn zero_capacity_resource_count(&self) -> usize {
        self.entries
            .values()
            .filter(|capacity| **capacity == 0)
            .count()
    }

    /// Returns whether every resource has at least one unit of capacity.
    #[must_use]
    pub fn all_resources_active(&self) -> bool {
        self.entries.values().all(|capacity| *capacity != 0)
    }

    /// Returns whether every resource has zero capacity.
    #[must_use]
    pub fn all_resources_disabled(&self) -> bool {
        self.entries.values().all(|capacity| *capacity == 0)
    }

    /// Returns the resource with the smallest configured capacity.
    ///
    /// When several resources have the same minimum capacity, the
    /// deterministically smallest `ResourceId` is returned.
    #[must_use]
    pub fn minimum_capacity_entry(&self) -> Option<ResourcePoolEntry> {
        self.entries
            .iter()
            .min_by(|(resource_a, capacity_a), (resource_b, capacity_b)| {
                capacity_a
                    .cmp(capacity_b)
                    .then_with(|| resource_a.cmp(resource_b))
            })
            .map(|(&resource, &capacity)| {
                ResourcePoolEntry::new(resource, capacity)
            })
    }

    /// Returns the resource with the largest configured capacity.
    ///
    /// When several resources have the same maximum capacity, the
    /// deterministically smallest `ResourceId` is returned.
    #[must_use]
    pub fn maximum_capacity_entry(&self) -> Option<ResourcePoolEntry> {
        self.entries
            .iter()
            .max_by(|(resource_a, capacity_a), (resource_b, capacity_b)| {
                capacity_a
                    .cmp(capacity_b)
                    .then_with(|| resource_b.cmp(resource_a))
            })
            .map(|(&resource, &capacity)| {
                ResourcePoolEntry::new(resource, capacity)
            })
    }

    /// Returns whether a requested amount can fit within configured capacity.
    ///
    /// This method performs only a static capacity check.
    ///
    /// It does NOT inspect active reservations.
    ///
    /// Therefore:
    ///
    /// ```text
    /// pool.can_accommodate(resource, amount)
    /// ```
    ///
    /// means:
    ///
    /// > "Could this resource theoretically provide this amount?"
    ///
    /// It does NOT mean:
    ///
    /// > "Is this resource free for this amount at time T?"
    ///
    /// The latter belongs to the reservation/calendar layer.
    #[must_use]
    pub fn can_accommodate(
        &self,
        resource: ResourceId,
        requested: ResourceCapacity,
    ) -> bool {
        self.capacity(resource)
            .is_some_and(|capacity| requested <= capacity)
    }

    /// Returns whether all requested resource capacities can be satisfied
    /// statically by this pool.
    ///
    /// The operation is transactional in the logical sense: it performs all
    /// checks without mutating the pool.
    ///
    /// Duplicate resource requests are accepted only when their individual
    /// requested amounts are each independently valid. The caller should use
    /// a normalized resource requirement representation when multiple requests
    /// for the same resource are semantically additive.
    #[must_use]
    pub fn can_accommodate_all<I>(
        &self,
        requests: I,
    ) -> bool
    where
        I: IntoIterator<Item = (ResourceId, ResourceCapacity)>,
    {
        requests
            .into_iter()
            .all(|(resource, requested)| {
                self.can_accommodate(resource, requested)
            })
    }

    /// Replaces the entire pool with a validated set of entries.
    ///
    /// The replacement is atomic from the caller's perspective: the new map is
    /// constructed independently and only installed after all validation has
    /// succeeded.
    pub fn replace<I>(
        &mut self,
        entries: I,
    ) -> Result<(), ResourcePoolError>
    where
        I: IntoIterator<Item = (ResourceId, ResourceCapacity)>,
    {
        let replacement = Self::try_from_entries(entries)?;
        self.entries = replacement.entries;
        Ok(())
    }

    /// Clears all resources.
    ///
    /// This is intentionally explicit because removing resources may invalidate
    /// external reservations. Higher-level callers must therefore ensure that
    /// no live schedule references the pool before clearing it.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns a read-only view over the underlying deterministic map.
    ///
    /// This method is provided for advanced scheduler infrastructure that needs
    /// map-native operations without taking ownership of the pool.
    ///
    /// Callers must not rely on the underlying container type as a long-term
    /// serialization contract.
    #[must_use]
    pub fn as_map(
        &self,
    ) -> &BTreeMap<ResourceId, ResourceCapacity> {
        &self.entries
    }

    /// Returns the first resource entry, according to deterministic resource
    /// identity ordering.
    #[must_use]
    pub fn first(&self) -> Option<ResourcePoolEntry> {
        self.entries
            .first_key_value()
            .map(|(&resource, &capacity)| {
                ResourcePoolEntry::new(resource, capacity)
            })
    }

    /// Returns the last resource entry, according to deterministic resource
    /// identity ordering.
    #[must_use]
    pub fn last(&self) -> Option<ResourcePoolEntry> {
        self.entries
            .last_key_value()
            .map(|(&resource, &capacity)| {
                ResourcePoolEntry::new(resource, capacity)
            })
    }

    /// Returns the number of resources in a capacity range.
    ///
    /// Both bounds are inclusive.
    #[must_use]
    pub fn count_capacity_range(
        &self,
        minimum: ResourceCapacity,
        maximum: ResourceCapacity,
    ) -> usize {
        if minimum > maximum {
            return 0;
        }

        self.entries
            .values()
            .filter(|capacity| {
                **capacity >= minimum && **capacity <= maximum
            })
            .count()
    }
}

impl Default for ResourcePool {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a ResourcePool {
    type Item = ResourcePoolEntry;
    type IntoIter = ResourcePoolIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ResourcePoolIter {
            inner: self.entries.iter(),
        }
    }
}

/// Deterministic iterator over resource-pool entries.
pub struct ResourcePoolIter<'a> {
    inner: btree_map::Iter<'a, ResourceId, ResourceCapacity>,
}

impl<'a> Iterator for ResourcePoolIter<'a> {
    type Item = ResourcePoolEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(&resource, &capacity)| {
                ResourcePoolEntry::new(resource, capacity)
            })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl ExactSizeIterator for ResourcePoolIter<'_> {}

impl fmt::Display for ResourcePool {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "ResourcePool(resources={}, capacity={})",
            self.len(),
            self.checked_total_capacity()
                .map_or_else(
                    || "overflow".to_owned(),
                    |capacity| capacity.to_string()
                )
        )
    }
}

// =============================================================================
// Tests
// =============================================================================
//
// These tests deliberately exercise invariants rather than implementation
// details. They are local to this foundational file so that the file can be
// considered complete independently of future reservation/calendar modules.
//
// Larger cross-module tests belong under:
//     scheduling/tests/unit/
//     scheduling/tests/integration/
//     scheduling/tests/property/
//     scheduling/tests/scalability/
//

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::core::identity::ResourceId;

    fn resource(value: u64) -> ResourceId {
        ResourceId::new(value)
    }

    #[test]
    fn new_pool_is_empty() {
        let pool = ResourcePool::new();

        assert!(pool.is_empty());
        assert_eq!(pool.len(), 0);
        assert_eq!(pool.checked_total_capacity(), Some(0));
    }

    #[test]
    fn insert_and_lookup_resource() {
        let mut pool = ResourcePool::new();

        pool.insert(resource(7), 3).unwrap();

        assert_eq!(pool.len(), 1);
        assert!(pool.contains(resource(7)));
        assert_eq!(pool.capacity(resource(7)), Some(3));
    }

    #[test]
    fn duplicate_insert_is_rejected() {
        let mut pool = ResourcePool::new();

        pool.insert(resource(1), 4).unwrap();

        let error = pool.insert(resource(1), 8).unwrap_err();

        assert_eq!(
            error,
            ResourcePoolError::DuplicateResource {
                resource: resource(1),
            }
        );

        assert_eq!(pool.capacity(resource(1)), Some(4));
    }

    #[test]
    fn unknown_capacity_update_is_rejected() {
        let mut pool = ResourcePool::new();

        let error = pool.set_capacity(resource(99), 4).unwrap_err();

        assert_eq!(
            error,
            ResourcePoolError::UnknownResource {
                resource: resource(99),
            }
        );
    }

    #[test]
    fn capacity_update_preserves_identity() {
        let mut pool = ResourcePool::new();

        pool.insert(resource(2), 1).unwrap();
        pool.set_capacity(resource(2), 9).unwrap();

        assert_eq!(pool.capacity(resource(2)), Some(9));
        assert!(pool.contains(resource(2)));
    }

    #[test]
    fn remove_returns_previous_entry() {
        let mut pool = ResourcePool::new();

        pool.insert(resource(3), 11).unwrap();

        let removed = pool.remove(resource(3)).unwrap();

        assert_eq!(
            removed,
            ResourcePoolEntry::new(resource(3), 11)
        );
        assert!(!pool.contains(resource(3)));
        assert!(pool.is_empty());
    }

    #[test]
    fn unknown_remove_is_rejected() {
        let mut pool = ResourcePool::new();

        let error = pool.remove(resource(404)).unwrap_err();

        assert_eq!(
            error,
            ResourcePoolError::UnknownResource {
                resource: resource(404),
            }
        );
    }

    #[test]
    fn remove_if_present_is_idempotent() {
        let mut pool = ResourcePool::new();

        assert_eq!(
            pool.remove_if_present(resource(1)),
            None
        );

        pool.insert(resource(1), 5).unwrap();

        assert_eq!(
            pool.remove_if_present(resource(1)),
            Some(ResourcePoolEntry::new(resource(1), 5))
        );

        assert_eq!(
            pool.remove_if_present(resource(1)),
            None
        );
    }

    #[test]
    fn deterministic_iteration_is_resource_id_ordered() {
        let mut pool = ResourcePool::new();

        pool.insert(resource(9), 1).unwrap();
        pool.insert(resource(2), 4).unwrap();
        pool.insert(resource(7), 3).unwrap();

        let ids: Vec<_> = pool.resource_ids().collect();

        assert_eq!(
            ids,
            vec![resource(2), resource(7), resource(9)]
        );
    }

    #[test]
    fn total_capacity_uses_checked_arithmetic() {
        let mut pool = ResourcePool::new();

        pool.insert(resource(1), 10).unwrap();
        pool.insert(resource(2), 20).unwrap();
        pool.insert(resource(3), 30).unwrap();

        assert_eq!(pool.checked_total_capacity(), Some(60));
    }

    #[test]
    fn zero_capacity_is_representable() {
        let mut pool = ResourcePool::new();

        pool.insert(resource(1), 0).unwrap();

        assert_eq!(pool.capacity(resource(1)), Some(0));
        assert_eq!(pool.active_resource_count(), 0);
        assert_eq!(pool.zero_capacity_resource_count(), 1);
        assert!(!pool.all_resources_active());
        assert!(pool.all_resources_disabled());
    }

    #[test]
    fn mixed_capacity_state_is_reported_correctly() {
        let mut pool = ResourcePool::new();

        pool.insert(resource(1), 0).unwrap();
        pool.insert(resource(2), 2).unwrap();
        pool.insert(resource(3), 5).unwrap();

        assert_eq!(pool.active_resource_count(), 2);
        assert_eq!(pool.zero_capacity_resource_count(), 1);
        assert!(!pool.all_resources_active());
        assert!(!pool.all_resources_disabled());
    }

    #[test]
    fn static_capacity_check_does_not_confuse_missing_with_zero() {
        let mut pool = ResourcePool::new();

        pool.insert(resource(1), 0).unwrap();

        assert!(pool.contains(resource(1)));
        assert!(!pool.can_accommodate(resource(1), 1));
        assert!(!pool.can_accommodate(resource(999), 0));
        assert!(pool.can_accommodate(resource(1), 0));
    }

    #[test]
    fn snapshot_is_independent_from_pool_mutation() {
        let mut pool = ResourcePool::new();

        pool.insert(resource(1), 4).unwrap();

        let snapshot = pool.snapshot();

        pool.set_capacity(resource(1), 9).unwrap();
        pool.insert(resource(2), 3).unwrap();

        assert_eq!(snapshot.len(), 1);
        assert_eq!(snapshot.capacity(resource(1)), Some(4));
        assert_eq!(snapshot.capacity(resource(2)), None);

        assert_eq!(pool.capacity(resource(1)), Some(9));
        assert_eq!(pool.capacity(resource(2)), Some(3));
    }

    #[test]
    fn builder_rejects_duplicate_resources() {
        let mut builder = ResourcePool::builder();

        builder.insert(resource(1), 2).unwrap();

        let error = builder.insert(resource(1), 7).unwrap_err();

        assert_eq!(
            error,
            ResourcePoolError::DuplicateResource {
                resource: resource(1),
            }
        );

        assert_eq!(builder.len(), 1);
    }

    #[test]
    fn builder_constructs_pool() {
        let pool = ResourcePool::builder()
            .with_resource(resource(1), 2)
            .unwrap()
            .with_resource(resource(2), 4)
            .unwrap()
            .build();

        assert_eq!(pool.len(), 2);
        assert_eq!(pool.capacity(resource(1)), Some(2));
        assert_eq!(pool.capacity(resource(2)), Some(4));
    }

    #[test]
    fn batch_construction_rejects_duplicate_without_partial_pool() {
        let result = ResourcePool::try_from_entries([
            (resource(1), 2),
            (resource(2), 3),
            (resource(1), 7),
        ]);

        assert!(matches!(
            result,
            Err(ResourcePoolError::DuplicateResource { .. })
        ));
    }

    #[test]
    fn replace_is_atomic() {
        let mut pool = ResourcePool::new();

        pool.insert(resource(1), 10).unwrap();

        let result = pool.replace([
            (resource(2), 20),
            (resource(2), 30),
        ]);

        assert!(result.is_err());

        assert_eq!(pool.len(), 1);
        assert_eq!(pool.capacity(resource(1)), Some(10));
        assert_eq!(pool.capacity(resource(2)), None);
    }

    #[test]
    fn first_and_last_are_deterministic() {
        let mut pool = ResourcePool::new();

        pool.insert(resource(10), 1).unwrap();
        pool.insert(resource(2), 2).unwrap();
        pool.insert(resource(7), 3).unwrap();

        assert_eq!(
            pool.first(),
            Some(ResourcePoolEntry::new(resource(2), 2))
        );

        assert_eq!(
            pool.last(),
            Some(ResourcePoolEntry::new(resource(10), 1))
        );
    }

    #[test]
    fn minimum_capacity_is_deterministic() {
        let mut pool = ResourcePool::new();

        pool.insert(resource(10), 3).unwrap();
        pool.insert(resource(2), 1).unwrap();
        pool.insert(resource(7), 1).unwrap();

        assert_eq!(
            pool.minimum_capacity_entry(),
            Some(ResourcePoolEntry::new(resource(2), 1))
        );
    }

    #[test]
    fn maximum_capacity_is_deterministic() {
        let mut pool = ResourcePool::new();

        pool.insert(resource(10), 8).unwrap();
        pool.insert(resource(2), 4).unwrap();
        pool.insert(resource(7), 8).unwrap();

        assert_eq!(
            pool.maximum_capacity_entry(),
            Some(ResourcePoolEntry::new(resource(7), 8))
        );
    }

    #[test]
    fn capacity_range_handles_invalid_range() {
        let mut pool = ResourcePool::new();

        pool.insert(resource(1), 1).unwrap();
        pool.insert(resource(2), 2).unwrap();
        pool.insert(resource(3), 3).unwrap();

        assert_eq!(pool.count_capacity_range(3, 1), 0);
        assert_eq!(pool.count_capacity_range(1, 2), 2);
        assert_eq!(pool.count_capacity_range(2, 3), 2);
    }

    #[test]
    fn into_iterator_is_deterministic() {
        let mut pool = ResourcePool::new();

        pool.insert(resource(8), 1).unwrap();
        pool.insert(resource(1), 5).unwrap();
        pool.insert(resource(4), 3).unwrap();

        let entries: Vec<_> = (&pool).into_iter().collect();

        assert_eq!(
            entries,
            vec![
                ResourcePoolEntry::new(resource(1), 5),
                ResourcePoolEntry::new(resource(4), 3),
                ResourcePoolEntry::new(resource(8), 1),
            ]
        );
    }

    #[test]
    fn display_is_stable() {
        let mut pool = ResourcePool::new();

        pool.insert(resource(1), 2).unwrap();
        pool.insert(resource(2), 3).unwrap();

        assert_eq!(
            pool.to_string(),
            "ResourcePool(resources=2, capacity=5)"
        );
    }
}