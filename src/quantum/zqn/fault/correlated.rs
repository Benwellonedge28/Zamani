//! Zamani Quantum Noise (ZQN) — Correlated Faults.
//!
//! # Ownership
//!
//! This module owns the representation and validation of a correlation
//! relationship between multiple realized [`Fault`] values.
//!
//! It answers:
//!
//! > "Which independently represented faults belong to the same correlated
//! > event/domain, and which quantum resources participate in that event?"
//!
//! This module owns:
//!
//! - [`CorrelatedFault`];
//! - [`CorrelatedFaultBuilder`];
//! - correlation identity association;
//! - deterministic canonical member ordering;
//! - duplicate-resource detection;
//! - correlated-group structural validation;
//! - correlation membership queries;
//! - streaming construction helpers;
//! - immutable inspection APIs;
//! - resource-count inspection.
//!
//! This module does NOT own:
//!
//! - the canonical [`Fault`] representation;
//! - quantum-resource identity;
//! - probability distributions;
//! - quantum channels;
//! - stochastic sampling;
//! - random-number generation;
//! - correlation probability laws;
//! - temporal correlation models;
//! - spatial correlation models;
//! - calibration;
//! - characterization;
//! - routing;
//! - scheduling;
//! - QEC decoding;
//! - logical correction;
//! - hardware APIs;
//! - serialization formats;
//! - runtime resource accounting.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//! crate::quantum::ir::qubit
//!             │
//!             ├── QubitId
//!             └── PhysicalQubitId
//!
//!                    │
//!                    ▼
//!       zqn::fault::fault::Fault
//!                    │
//!                    ▼
//!       zqn::fault::correlated
//!                    │
//!              CorrelatedFault
//!                    │
//!       ┌────────────┼────────────┐
//!       ▼            ▼            ▼
//!     noise         QEC        simulation
//!       │            │            │
//!       └────────────┼────────────┘
//!                    ▼
//!                 runtime
//! ```
//!
//! The canonical fault semantics remain in `fault.rs`.
//!
//! This module MUST NOT introduce a second `Fault`, `QubitId`, or
//! `PhysicalQubitId` representation.
//!
//! # Why correlated faults are a separate abstraction
//!
//! A [`Fault`] represents one realized fault.
//!
//! A [`CorrelatedFault`] represents a deterministic grouping relationship
//! between multiple realized faults.
//!
//! Therefore:
//!
//! ```text
//! Fault
//!     = one realized deviation
//!
//! CorrelatedFault
//!     = one correlation domain containing multiple realized deviations
//! ```
//!
//! This distinction is important because a correlated physical event can
//! produce different effects on different resources.
//!
//! For example:
//!
//! ```text
//! resource A → X fault
//! resource B → Z fault
//! resource C → leakage
//! ```
//!
//! These should not be forced into a single `FaultEffect` merely because they
//! belong to one physical correlated event.
//!
//! The existing `Fault::correlated(...)` constructor remains useful when one
//! effect is intentionally shared across a composite location. This module
//! provides the more general multi-fault relationship required for production
//! noise modeling.
//!
//! # Canonical quantum-resource identity
//!
//! This module intentionally does not define quantum-resource identifiers.
//!
//! Each member [`Fault`] already uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Consequently:
//!
//! ```text
//! CorrelatedFault
//!     → Fault
//!         → canonical IR qubit identity
//! ```
//!
//! There is no:
//!
//! ```text
//! ZqnQubitId
//! CorrelatedQubitId
//! CorrelatedPhysicalQubitId
//! ```
//!
//! This preserves the repository's canonical identity boundary.
//!
//! # Write once, scale everywhere
//!
//! There is intentionally no:
//!
//! ```text
//! MAX_CORRELATED_FAULTS
//! MAX_CORRELATED_QUBITS
//! MAX_CORRELATED_RESOURCES
//! MAX_CORRELATION_SIZE
//! ```
//!
//! A correlation group may contain any finite number of members representable
//! by the selected host/storage representation and permitted by the caller's
//! explicit resource policy.
//!
//! "Infinity" in the Zamani architecture means that this semantic module does
//! not impose an artificial finite machine-size ceiling.
//!
//! Actual execution remains bounded by:
//!
//! - available memory;
//! - storage;
//! - CPU/GPU capacity;
//! - distributed capacity;
//! - runtime policy;
//! - target capabilities;
//! - caller-selected `ZqnLimits`;
//! - operating-system limits.
//!
//! # Resource policy
//!
//! This module deliberately does not embed a maximum group size.
//!
//! The existing `zqn::core::limits::ZqnLimits` owns the optional
//! `correlated_resources` execution policy.
//!
//! The intended relationship is:
//!
//! ```text
//! CorrelatedFault
//!     = semantic representation
//!
//! ZqnLimits
//!     = optional admission policy
//!
//! Runtime
//!     = actual resource accounting
//! ```
//!
//! This prevents a semantic data structure from becoming an accidental
//! hardware-size limit.
//!
//! # Determinism
//!
//! Correlated-fault construction is deterministic.
//!
//! This module:
//!
//! - owns no RNG;
//! - uses no global mutable state;
//! - does not inspect memory addresses;
//! - does not depend on hash-map iteration order;
//! - does not use system time implicitly;
//! - canonicalizes member order;
//! - validates duplicate resources deterministically.
//!
//! Given the same correlation identity and the same set of fault values, the
//! resulting canonical representation is deterministic.
//!
//! # Canonical ordering
//!
//! Members are stored in deterministic order.
//!
//! Ordering is based on the complete semantic fault value rather than memory
//! address or insertion order.
//!
//! Consumers MUST NOT interpret this ordering as:
//!
//! - temporal order;
//! - severity;
//! - execution priority;
//! - topology order;
//! - causal order.
//!
//! Temporal information belongs to [`FaultTiming`] inside each [`Fault`].
//!
//! # Duplicate resources
//!
//! A correlated event must not silently contain two independent fault members
//! targeting the exact same location.
//!
//! Therefore this module rejects duplicate [`FaultLocation`] values.
//!
//! This avoids silently turning:
//!
//! ```text
//! A → X
//! A → Z
//! ```
//!
//! into an ambiguous correlation group.
//!
//! If multiple effects on one resource are semantically required, they should
//! be represented explicitly by the owning noise/channel model rather than
//! being hidden inside this grouping abstraction.
//!
//! # Nested correlated faults
//!
//! A [`CorrelatedFault`] is itself a grouping abstraction, not an individual
//! [`Fault`].
//!
//! A member of a `CorrelatedFault` therefore MUST be a non-correlated
//! individual `Fault`.
//!
//! This prevents unbounded semantic nesting such as:
//!
//! ```text
//! correlation
//!   └── correlation
//!       └── correlation
//!           └── ...
//! ```
//!
//! Higher-level correlation hierarchies belong in the correlation-model
//! subsystem, not in the realized-fault grouping type.
//!
//! # Empty groups
//!
//! An empty correlation group is invalid.
//!
//! A correlation domain represents an actual realized correlated event and
//! therefore requires at least one realized fault.
//!
//! A production caller that requires *multi-resource* correlation can use
//! [`CorrelatedFault::is_multi_resource`] or the corresponding builder
//! validation.
//!
//! The lower-level representation permits one member so that correlation
//! metadata can be attached before another member arrives in a streaming
//! construction pipeline.
//!
//! # Streaming construction
//!
//! [`CorrelatedFaultBuilder`] exists so callers do not need to construct a
//! temporary `Vec<Fault>` before validation.
//!
//! This is important for large systems.
//!
//! ```text
//! stream
//!   │
//!   ├── fault
//!   ├── fault
//!   ├── fault
//!   └── ...
//!        │
//!        ▼
//! CorrelatedFaultBuilder
//!        │
//!        ▼
//! canonical CorrelatedFault
//! ```
//!
//! The builder still necessarily stores members because the final semantic
//! object owns its members. It avoids requiring the caller to materialize a
//! second duplicate collection first.
//!
//! # Memory model
//!
//! The implementation uses a contiguous `Vec<Fault>` because:
//!
//! - it has predictable ownership;
//! - it has good locality;
//! - it is deterministic;
//! - it can be transferred without copying;
//! - it works naturally with Rust iterators;
//! - it does not require unsafe code;
//! - it does not require a fixed upper bound.
//!
//! The module does not promise that an arbitrarily large correlation group can
//! physically fit in one process. That is a resource-management concern.
//!
//! Distributed/streaming correlation models can consume the same semantic
//! members incrementally without changing this representation's invariants.
//!
//! # Serialization
//!
//! This module intentionally does not depend on serde or another serialization
//! framework.
//!
//! The future ZQN `io` subsystem owns:
//!
//! - schema;
//! - encoding;
//! - canonical serialization;
//! - migration;
//! - compatibility.
//!
//! A serialized `CorrelatedFault` must preserve:
//!
//! - correlation identity;
//! - canonical member sequence;
//! - each member's complete fault identity;
//! - each member's complete fault semantics;
//! - schema/version context.
//!
//! Serialization MUST NOT convert logical and physical qubit IDs into one
//! untyped identity domain.
//!
//! # Thread safety
//!
//! `CorrelatedFault` is immutable after construction.
//!
//! It contains no interior mutability or global state.
//!
//! Its constituent values are immutable semantic values.
//!
//! Therefore it is suitable for concurrent use when placed inside a
//! thread-safe execution context.
//!
//! # Security
//!
//! Correlated faults are data, not capabilities.
//!
//! Possession of a correlation ID or fault ID MUST NOT grant:
//!
//! - QPU access;
//! - hardware control;
//! - credentials;
//! - calibration write access;
//! - execution authorization.
//!
//! Untrusted correlation streams must be admitted under explicit resource
//! policies before materializing arbitrarily large collections.
//!
//! # Numerical safety
//!
//! This module performs no probability calculations.
//!
//! Correlation probability, covariance, copulas, joint distributions,
//! correlation kernels, and stochastic sampling belong to the probability and
//! noise subsystems.
//!
//! # Error contract
//!
//! Construction fails when:
//!
//! - the member collection is empty;
//! - a member fault is structurally invalid;
//! - a member is itself correlated;
//! - duplicate fault locations exist;
//! - canonicalization cannot be completed because the caller violates an
//!   explicit invariant.
//!
//! Resource-limit failures belong to the caller's execution policy and should
//! be checked before materialization when processing untrusted large inputs.
//!
//! # Integration contract
//!
//! `fault.rs`
//!     owns `Fault`, `FaultLocation`, `FaultEffect`, and fault semantics.
//!
//! `core::ids`
//!     owns `CorrelationId`.
//!
//! `core::limits`
//!     owns optional `correlated_resources` admission policy.
//!
//! `noise::correlation`
//!     owns correlation laws/models and may produce `CorrelatedFault`.
//!
//! `noise::spatial`
//!     may use `CorrelatedFault` for realized spatial events.
//!
//! `noise::temporal`
//!     may attach temporal information to individual members.
//!
//! `simulation`
//!     may consume `CorrelatedFault` as a realized event.
//!
//! `integration::qec`
//!     may translate each member into QEC-specific physical faults.
//!
//! `integration::routing`
//!     may inspect participating resources when calculating correlated-error
//!     costs.
//!
//! `integration::scheduling`
//!     may inspect participating resources when estimating crosstalk or
//!     concurrency effects.
//!
//! `io`
//!     owns serialization.
//!
//! `benchmarking`
//!     may consume correlated-fault observations without redefining their
//!     semantics.
//!
//! # No dependency on future modules
//!
//! This file intentionally depends only on:
//!
//! - `core::fmt`;
//! - `std::vec::Vec`/iterator infrastructure;
//! - existing `fault.rs`;
//! - existing ZQN IDs;
//! - existing ZQN errors.
//!
//! It does not depend on:
//!
//! - `noise`;
//! - `channel`;
//! - `simulation`;
//! - `QEC`;
//! - `hardware`;
//! - `routing`;
//! - `scheduling`.
//!
//! Therefore this file can be completed before those subsystems are
//! implemented.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. it owns only correlation-group semantics;
//! 2. it reuses canonical `Fault`;
//! 3. it indirectly uses canonical IR `QubitId`/`PhysicalQubitId` through
//!    `Fault`;
//! 4. it uses `CorrelationId` from `core::ids`;
//! 5. it has no artificial correlation-size limit;
//! 6. duplicate resource locations are rejected;
//! 7. nested correlated faults are rejected;
//! 8. empty groups are rejected;
//! 9. member ordering is deterministic;
//! 10. construction can consume an iterator;
//! 11. no global state exists;
//! 12. no RNG exists;
//! 13. no unsafe code exists;
//! 14. serialization is left to `io`;
//! 15. resource limits remain external policy;
//! 16. QEC/noise/simulation do not need to modify this file to integrate;
//! 17. tests cover the invariants and scaling behavior.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::cmp::Ordering;
use core::fmt;

use crate::quantum::zqn::core::errors::{ZqnError, ZqnResult};
use crate::quantum::zqn::core::ids::{CorrelationId, FaultId};
use crate::quantum::zqn::fault::fault::{
    Fault,
    FaultLocation,
};

/// Canonical realized correlation group.
///
/// A `CorrelatedFault` groups multiple independently represented [`Fault`]
/// values that belong to one correlated physical/semantic event.
///
/// The individual faults remain authoritative for their own:
///
/// - identity;
/// - location;
/// - classification;
/// - effect;
/// - timing;
/// - operation association;
/// - annotations.
///
/// The correlation object adds only the relationship between them.
///
/// # Invariants
///
/// A valid `CorrelatedFault` satisfies:
///
/// - at least one member exists;
/// - every member validates;
/// - no member is itself correlated;
/// - no two members have the same location;
/// - members are stored in deterministic canonical order.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CorrelatedFault {
    correlation_id: CorrelationId,
    members: Vec<Fault>,
}

impl CorrelatedFault {
    /// Constructs a correlated fault group from an owned collection.
    ///
    /// The input collection is consumed and canonicalized in place.
    ///
    /// This method does not impose an artificial maximum number of members.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - `members` is empty;
    /// - a member is invalid;
    /// - a member is already correlated;
    /// - two members target the same location.
    pub fn new(
        correlation_id: CorrelationId,
        mut members: Vec<Fault>,
    ) -> ZqnResult<Self> {
        validate_members(&members)?;

        canonicalize_members(&mut members);

        validate_canonical_members(&members)?;

        Ok(Self {
            correlation_id,
            members,
        })
    }

    /// Constructs a correlation group from any iterator.
    ///
    /// This is the preferred API for callers that already have a stream or
    /// iterator of realized faults.
    ///
    /// The iterator is consumed exactly once.
    ///
    /// The resulting object owns the collected members.
    pub fn from_iter<I>(
        correlation_id: CorrelationId,
        members: I,
    ) -> ZqnResult<Self>
    where
        I: IntoIterator<Item = Fault>,
    {
        let members = members.into_iter().collect::<Vec<_>>();

        Self::new(correlation_id, members)
    }

    /// Returns the correlation identity.
    ///
    /// `CorrelationId` identifies the correlation domain/event object. It does
    /// not identify a qubit or a fault.
    #[must_use]
    pub const fn correlation_id(&self) -> CorrelationId {
        self.correlation_id
    }

    /// Returns all member faults in deterministic canonical order.
    ///
    /// The returned slice is immutable.
    #[must_use]
    pub fn members(&self) -> &[Fault] {
        &self.members
    }

    /// Returns the number of member faults.
    #[must_use]
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// Returns true if there are no members.
    ///
    /// A valid `CorrelatedFault` can never be empty. This method is provided
    /// for generic collection-style APIs and future-proofing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// Returns true if the correlation contains at least two distinct
    /// resources.
    ///
    /// This is the predicate to use when a caller specifically requires
    /// multi-resource correlation.
    #[must_use]
    pub fn is_multi_resource(&self) -> bool {
        self.members.len() >= 2
    }

    /// Returns the number of distinct resource locations participating in the
    /// correlation.
    ///
    /// Construction guarantees that this equals [`Self::len`].
    #[must_use]
    pub fn resource_count(&self) -> usize {
        self.members.len()
    }

    /// Returns true if any member directly targets a logical qubit.
    #[must_use]
    pub fn contains_logical_faults(&self) -> bool {
        self.members.iter().any(Fault::is_logical)
    }

    /// Returns true if any member directly targets a physical qubit.
    #[must_use]
    pub fn contains_physical_faults(&self) -> bool {
        self.members.iter().any(Fault::is_physical)
    }

    /// Returns true if any member represents leakage.
    #[must_use]
    pub fn contains_leakage(&self) -> bool {
        self.members.iter().any(Fault::is_leakage)
    }

    /// Returns true if any member represents loss or erasure.
    #[must_use]
    pub fn contains_loss_like_faults(&self) -> bool {
        self.members.iter().any(Fault::is_loss_like)
    }

    /// Returns true if any member has explicit timing.
    #[must_use]
    pub fn contains_timed_faults(&self) -> bool {
        self.members.iter().any(Fault::has_timing)
    }

    /// Returns true if a member with the supplied fault identity exists.
    #[must_use]
    pub fn contains_fault_id(&self, id: FaultId) -> bool {
        self.members.iter().any(|fault| fault.id() == id)
    }

    /// Returns a member by fault identity.
    #[must_use]
    pub fn get_by_fault_id(&self, id: FaultId) -> Option<&Fault> {
        self.members.iter().find(|fault| fault.id() == id)
    }

    /// Returns a member whose location exactly matches `location`.
    #[must_use]
    pub fn get_by_location(
        &self,
        location: &FaultLocation,
    ) -> Option<&Fault> {
        self.members
            .iter()
            .find(|fault| fault.location() == location)
    }

    /// Returns whether a location participates in this correlation.
    #[must_use]
    pub fn contains_location(
        &self,
        location: &FaultLocation,
    ) -> bool {
        self.get_by_location(location).is_some()
    }

    /// Returns the canonical member fault IDs.
    ///
    /// The returned vector is ordered identically to [`Self::members`].
    ///
    /// This method intentionally allocates because the caller explicitly asks
    /// for an owned collection.
    #[must_use]
    pub fn fault_ids(&self) -> Vec<FaultId> {
        self.members.iter().map(Fault::id).collect()
    }

    /// Returns all member locations as an owned vector.
    ///
    /// The operation is deterministic and preserves canonical member order.
    #[must_use]
    pub fn locations(&self) -> Vec<FaultLocation> {
        self.members
            .iter()
            .map(|fault| fault.location().clone())
            .collect()
    }

    /// Validates all correlation invariants.
    ///
    /// This method performs no mutation.
    pub fn validate(&self) -> ZqnResult<()> {
        validate_members(&self.members)?;
        validate_canonical_members(&self.members)
    }

    /// Returns a deterministic semantic description.
    ///
    /// This is diagnostic text, not a serialization format.
    #[must_use]
    pub fn describe(&self) -> String {
        let mut result = format!(
            "correlation:{}:members={}",
            self.correlation_id,
            self.members.len()
        );

        for member in &self.members {
            result.push('|');
            result.push_str(&member.describe());
        }

        result
    }

    /// Returns an iterator over immutable member faults.
    pub fn iter(&self) -> core::slice::Iter<'_, Fault> {
        self.members.iter()
    }

    /// Consumes the correlation group and returns its canonical members.
    ///
    /// This is useful at integration boundaries where another subsystem owns
    /// the next stage of processing.
    #[must_use]
    pub fn into_members(self) -> Vec<Fault> {
        self.members
    }

    /// Consumes the correlation group and returns its identity and members.
    #[must_use]
    pub fn into_parts(self) -> (CorrelationId, Vec<Fault>) {
        (self.correlation_id, self.members)
    }
}

impl fmt::Display for CorrelatedFault {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.describe())
    }
}

impl IntoIterator for CorrelatedFault {
    type Item = Fault;
    type IntoIter = std::vec::IntoIter<Fault>;

    fn into_iter(self) -> Self::IntoIter {
        self.members.into_iter()
    }
}

impl<'a> IntoIterator for &'a CorrelatedFault {
    type Item = &'a Fault;
    type IntoIter = core::slice::Iter<'a, Fault>;

    fn into_iter(self) -> Self::IntoIter {
        self.members.iter()
    }
}

/// Streaming builder for [`CorrelatedFault`].
///
/// The builder permits a caller to feed faults incrementally without first
/// constructing a separate temporary collection.
///
/// It is intentionally not tied to any RNG, simulator, QEC decoder, or
/// hardware backend.
///
/// # Example
///
/// ```text
/// let mut builder = CorrelatedFaultBuilder::new(correlation_id);
///
/// builder.push(first_fault)?;
/// builder.push(second_fault)?;
///
/// let correlated = builder.finish()?;
/// ```
///
/// No semantic member-count maximum is imposed here.
#[derive(Clone, Debug)]
pub struct CorrelatedFaultBuilder {
    correlation_id: CorrelationId,
    members: Vec<Fault>,
}

impl CorrelatedFaultBuilder {
    /// Creates an empty correlation builder.
    #[must_use]
    pub const fn new(correlation_id: CorrelationId) -> Self {
        Self {
            correlation_id,
            members: Vec::new(),
        }
    }

    /// Creates a builder with caller-provided allocation capacity.
    ///
    /// `capacity` is an allocation hint only.
    ///
    /// It is NOT a semantic limit and is never interpreted as the maximum
    /// number of correlated resources.
    #[must_use]
    pub fn with_capacity(
        correlation_id: CorrelationId,
        capacity: usize,
    ) -> Self {
        Self {
            correlation_id,
            members: Vec::with_capacity(capacity),
        }
    }

    /// Returns the correlation identity.
    #[must_use]
    pub const fn correlation_id(&self) -> CorrelationId {
        self.correlation_id
    }

    /// Returns the number of currently accumulated members.
    #[must_use]
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// Returns true when no members have been added.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// Adds one fault to the builder.
    ///
    /// The fault is validated immediately.
    ///
    /// Duplicate locations are rejected before the fault is retained.
    pub fn push(
        &mut self,
        fault: Fault,
    ) -> ZqnResult<()> {
        validate_member(&fault)?;

        if self
            .members
            .iter()
            .any(|existing| existing.location() == fault.location())
        {
            return Err(ZqnError::invalid_correlated_fault(
                "correlated fault contains duplicate resource location",
            ));
        }

        self.members.push(fault);

        Ok(())
    }

    /// Extends the builder from an iterator.
    ///
    /// Validation is performed incrementally.
    ///
    /// If an error occurs, previously accepted members remain in the builder.
    /// The caller may either continue using the builder or discard it.
    pub fn extend<I>(
        &mut self,
        faults: I,
    ) -> ZqnResult<()>
    where
        I: IntoIterator<Item = Fault>,
    {
        for fault in faults {
            self.push(fault)?;
        }

        Ok(())
    }

    /// Returns the current immutable members without consuming the builder.
    #[must_use]
    pub fn members(&self) -> &[Fault] {
        &self.members
    }

    /// Finalizes the builder into a canonical correlation group.
    ///
    /// Members are deterministically ordered during finalization.
    pub fn finish(self) -> ZqnResult<CorrelatedFault> {
        CorrelatedFault::new(self.correlation_id, self.members)
    }

    /// Discards the builder and returns its currently accumulated members.
    #[must_use]
    pub fn into_members(self) -> Vec<Fault> {
        self.members
    }
}

/// Validates a collection of correlated-fault members.
///
/// This function does not impose a member-count limit.
///
/// It performs only semantic validation.
fn validate_members(members: &[Fault]) -> ZqnResult<()> {
    if members.is_empty() {
        return Err(ZqnError::invalid_correlated_fault(
            "correlated fault group cannot be empty",
        ));
    }

    for fault in members {
        validate_member(fault)?;
    }

    Ok(())
}

/// Validates one member for use inside a correlation group.
fn validate_member(fault: &Fault) -> ZqnResult<()> {
    fault.validate()?;

    if fault.is_correlated() {
        return Err(ZqnError::invalid_correlated_fault(
            "nested correlated faults are not permitted",
        ));
    }

    Ok(())
}

/// Canonicalizes member order.
///
/// The ordering is deterministic and independent of insertion order.
///
/// `Fault` already implements `Ord`, but this module deliberately makes the
/// ordering contract explicit instead of relying on the internal field order
/// of `Fault`.
fn canonicalize_members(members: &mut [Fault]) {
    members.sort_by(compare_faults_canonically);
}

/// Performs post-canonicalization structural validation.
///
/// Duplicate locations are checked after sorting. This gives deterministic
/// behavior without requiring a hash map and without depending on randomized
/// hash state.
///
/// Complexity:
///
/// ```text
/// sorting     O(n log n)
/// validation  O(n)
/// memory      O(n) including the owned members
/// ```
///
/// No O(n²) duplicate scan is required.
fn validate_canonical_members(
    members: &[Fault],
) -> ZqnResult<()> {
    for window in members.windows(2) {
        let left = &window[0];
        let right = &window[1];

        if left.location() == right.location() {
            return Err(ZqnError::invalid_correlated_fault(
                "correlated fault contains duplicate resource location",
            ));
        }
    }

    Ok(())
}

/// Deterministic semantic ordering for faults.
///
/// The ordering deliberately ignores insertion order.
///
/// The comparison hierarchy is:
///
/// 1. location;
/// 2. classification;
/// 3. effect;
/// 4. timing;
/// 5. operation association;
/// 6. annotation;
/// 7. fault identity as a final deterministic tie-breaker.
///
/// Fault identity is last because it is an object identity rather than part of
/// the physical location semantics.
///
/// This ordering is not temporal or causal ordering.
fn compare_faults_canonically(
    left: &Fault,
    right: &Fault,
) -> Ordering {
    left.location()
        .cmp(right.location())
        .then_with(|| left.classification().cmp(right.classification()))
        .then_with(|| left.effect().cmp(right.effect()))
        .then_with(|| left.timing().cmp(&right.timing()))
        .then_with(|| left.operation().cmp(&right.operation()))
        .then_with(|| left.annotation().cmp(&right.annotation()))
        .then_with(|| left.id().cmp(&right.id()))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::qubit::{
        PhysicalQubitId,
        QubitId,
    };
    use crate::quantum::zqn::core::ids::{
        CorrelationId,
        FaultId,
        ZqnIdValue,
    };
    use crate::quantum::zqn::fault::fault::{
        FaultClassification,
        FaultEffect,
        PauliEffect,
    };

    fn correlation_id(value: ZqnIdValue) -> CorrelationId {
        CorrelationId::new(value)
    }

    fn fault_id(value: ZqnIdValue) -> FaultId {
        FaultId::new(value)
    }

    fn logical_x(
        id: ZqnIdValue,
        qubit: ZqnIdValue,
    ) -> Fault {
        Fault::logical_pauli(
            fault_id(id),
            FaultClassification::Gate,
            QubitId::new(qubit),
            PauliEffect::X,
        )
        .expect("valid logical X fault")
    }

    fn logical_z(
        id: ZqnIdValue,
        qubit: ZqnIdValue,
    ) -> Fault {
        Fault::logical_pauli(
            fault_id(id),
            FaultClassification::Gate,
            QubitId::new(qubit),
            PauliEffect::Z,
        )
        .expect("valid logical Z fault")
    }

    fn physical_x(
        id: ZqnIdValue,
        qubit: ZqnIdValue,
    ) -> Fault {
        Fault::physical_pauli(
            fault_id(id),
            FaultClassification::Gate,
            PhysicalQubitId::new(qubit),
            PauliEffect::X,
        )
        .expect("valid physical X fault")
    }

    #[test]
    fn constructs_single_member_correlation() {
        let fault = logical_x(1, 7);

        let correlated = CorrelatedFault::new(
            correlation_id(100),
            vec![fault.clone()],
        )
        .expect("single-member correlation is structurally valid");

        assert_eq!(correlated.correlation_id(), correlation_id(100));
        assert_eq!(correlated.len(), 1);
        assert_eq!(correlated.members(), &[fault]);
        assert!(!correlated.is_multi_resource());
    }

    #[test]
    fn constructs_multi_resource_correlation() {
        let first = logical_x(1, 7);
        let second = logical_z(2, 8);

        let correlated = CorrelatedFault::new(
            correlation_id(100),
            vec![first, second],
        )
        .expect("valid correlated group");

        assert_eq!(correlated.len(), 2);
        assert!(correlated.is_multi_resource());
        assert!(correlated.contains_logical_faults());
    }

    #[test]
    fn canonical_order_is_independent_of_insertion_order() {
        let first = logical_x(1, 7);
        let second = logical_z(2, 8);

        let left = CorrelatedFault::new(
            correlation_id(100),
            vec![first.clone(), second.clone()],
        )
        .expect("valid correlation");

        let right = CorrelatedFault::new(
            correlation_id(100),
            vec![second, first],
        )
        .expect("valid correlation");

        assert_eq!(left.members(), right.members());
        assert_eq!(left, right);
    }

    #[test]
    fn empty_group_is_rejected() {
        let result = CorrelatedFault::new(
            correlation_id(100),
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_location_is_rejected() {
        let first = logical_x(1, 7);
        let second = logical_z(2, 7);

        let result = CorrelatedFault::new(
            correlation_id(100),
            vec![first, second],
        );

        assert!(result.is_err());
    }

    #[test]
    fn distinct_logical_and_physical_domains_are_not_duplicates() {
        let logical = logical_x(1, 7);
        let physical = physical_x(2, 7);

        let result = CorrelatedFault::new(
            correlation_id(100),
            vec![logical, physical],
        );

        assert!(result.is_ok());
    }

    #[test]
    fn nested_correlation_is_rejected() {
        let inner = Fault::correlated(
            fault_id(1),
            vec![
                FaultLocation::LogicalQubit(QubitId::new(1)),
                FaultLocation::LogicalQubit(QubitId::new(2)),
            ],
            FaultEffect::Pauli(PauliEffect::X),
        )
        .expect("valid inner correlated fault");

        let result = CorrelatedFault::new(
            correlation_id(100),
            vec![inner],
        );

        assert!(result.is_err());
    }

    #[test]
    fn builder_accepts_incremental_members() {
        let mut builder =
            CorrelatedFaultBuilder::new(correlation_id(100));

        builder
            .push(logical_x(1, 7))
            .expect("first member is valid");

        builder
            .push(logical_z(2, 8))
            .expect("second member is valid");

        let correlated =
            builder.finish().expect("builder should finish");

        assert_eq!(correlated.len(), 2);
        assert!(correlated.is_multi_resource());
    }

    #[test]
    fn builder_rejects_duplicate_locations_immediately() {
        let mut builder =
            CorrelatedFaultBuilder::new(correlation_id(100));

        builder
            .push(logical_x(1, 7))
            .expect("first member is valid");

        let result = builder.push(logical_z(2, 7));

        assert!(result.is_err());
        assert_eq!(builder.len(), 1);
    }

    #[test]
    fn builder_extend_consumes_iterator_once() {
        let faults = vec![
            logical_x(1, 1),
            logical_z(2, 2),
            logical_x(3, 3),
        ];

        let mut builder =
            CorrelatedFaultBuilder::new(correlation_id(100));

        builder
            .extend(faults.into_iter())
            .expect("iterator should be accepted");

        let correlated =
            builder.finish().expect("builder should finish");

        assert_eq!(correlated.len(), 3);
    }

    #[test]
    fn lookup_by_fault_id_works() {
        let first = logical_x(11, 1);
        let second = logical_z(12, 2);

        let correlated = CorrelatedFault::new(
            correlation_id(100),
            vec![first, second],
        )
        .expect("valid correlation");

        assert!(correlated.contains_fault_id(fault_id(11)));
        assert!(correlated.contains_fault_id(fault_id(12)));
        assert!(!correlated.contains_fault_id(fault_id(99)));
    }

    #[test]
    fn lookup_by_location_works() {
        let first = logical_x(11, 1);
        let second = logical_z(12, 2);

        let correlated = CorrelatedFault::new(
            correlation_id(100),
            vec![first, second],
        )
        .expect("valid correlation");

        let location =
            FaultLocation::LogicalQubit(QubitId::new(2));

        assert!(correlated.contains_location(&location));
    }

    #[test]
    fn predicates_reflect_member_content() {
        let first = logical_x(1, 1);
        let second = physical_x(2, 2);

        let correlated = CorrelatedFault::new(
            correlation_id(100),
            vec![first, second],
        )
        .expect("valid correlation");

        assert!(correlated.contains_logical_faults());
        assert!(correlated.contains_physical_faults());
        assert!(!correlated.contains_leakage());
        assert!(!correlated.contains_loss_like_faults());
    }

    #[test]
    fn into_parts_preserves_identity_and_members() {
        let first = logical_x(1, 1);
        let second = logical_z(2, 2);

        let correlated = CorrelatedFault::new(
            correlation_id(100),
            vec![first, second],
        )
        .expect("valid correlation");

        let (id, members) = correlated.into_parts();

        assert_eq!(id, correlation_id(100));
        assert_eq!(members.len(), 2);
    }

    #[test]
    fn into_iterator_is_deterministic() {
        let first = logical_x(1, 7);
        let second = logical_z(2, 8);

        let correlated = CorrelatedFault::new(
            correlation_id(100),
            vec![second, first],
        )
        .expect("valid correlation");

        let collected = correlated.into_iter().collect::<Vec<_>>();

        assert_eq!(collected.len(), 2);
        assert!(collected[0].location() <= collected[1].location());
    }

    #[test]
    fn validation_is_idempotent() {
        let correlated = CorrelatedFault::new(
            correlation_id(100),
            vec![
                logical_x(1, 1),
                logical_z(2, 2),
            ],
        )
        .expect("valid correlation");

        assert!(correlated.validate().is_ok());
        assert!(correlated.validate().is_ok());
    }

    #[test]
    fn correlation_id_does_not_change_member_semantics() {
        let members = vec![
            logical_x(1, 1),
            logical_z(2, 2),
        ];

        let first = CorrelatedFault::new(
            correlation_id(100),
            members.clone(),
        )
        .expect("valid correlation");

        let second = CorrelatedFault::new(
            correlation_id(200),
            members,
        )
        .expect("valid correlation");

        assert_ne!(first, second);
        assert_eq!(first.members(), second.members());
    }

    #[test]
    fn description_is_deterministic() {
        let first = logical_x(1, 1);
        let second = logical_z(2, 2);

        let a = CorrelatedFault::new(
            correlation_id(100),
            vec![first.clone(), second.clone()],
        )
        .expect("valid correlation");

        let b = CorrelatedFault::new(
            correlation_id(100),
            vec![second, first],
        )
        .expect("valid correlation");

        assert_eq!(a.describe(), b.describe());
    }

    #[test]
    fn correlation_can_contain_different_effects() {
        let first = logical_x(1, 1);
        let second = logical_z(2, 2);

        let correlated = CorrelatedFault::new(
            correlation_id(100),
            vec![first, second],
        )
        .expect("valid correlation");

        assert_eq!(correlated.len(), 2);
        assert_eq!(
            correlated.members()[0].effect().category(),
            "pauli"
        );
        assert_eq!(
            correlated.members()[1].effect().category(),
            "pauli"
        );
    }
}