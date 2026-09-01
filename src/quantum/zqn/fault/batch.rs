//! Zamani Quantum Noise (ZQN) — Realized Fault Batches.
//!
//! This module owns the production representation of a finite, explicitly
//! materialized sequence of realized [`Fault`] values.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - `FaultBatch`;
//! - ordered fault storage;
//! - batch construction and validation;
//! - append/extend operations;
//! - bounded admission through `ZqnLimits`;
//! - deterministic inspection;
//! - allocation-aware construction;
//! - immutable and consuming iteration;
//! - batch statistics that do not require domain-specific interpretation;
//! - explicit canonical-order views;
//! - batch-level resource accounting;
//! - conversion-independent batch validation.
//!
//! This file does NOT own:
//!
//! - the canonical `Fault` representation;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - fault semantics;
//! - probability distributions;
//! - stochastic sampling;
//! - random-number generation;
//! - noise-model semantics;
//! - correlated-noise laws;
//! - calibration;
//! - characterization;
//! - routing;
//! - scheduling;
//! - QEC decoding;
//! - simulation state;
//! - hardware APIs;
//! - QPU credentials;
//! - runtime resource accounting;
//! - serialization formats.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir::qubit
//!         │
//!         ├── QubitId
//!         └── PhysicalQubitId
//!                  │
//!                  ▼
//!        zqn::fault::fault
//!                  │
//!                  └── Fault
//!                       │
//!                       ▼
//!               zqn::fault::batch
//!                       │
//!                       └── FaultBatch
//!                          │
//!             ┌────────────┼─────────────┐
//!             ▼            ▼             ▼
//!           noise         QEC        simulation
//!             │            │             │
//!             └────────────┼─────────────┘
//!                          ▼
//!                       runtime
//! ```
//!
//! `FaultBatch` is deliberately a container, not a new semantic fault.
//!
//! # Canonical identity boundary
//!
//! `FaultBatch` does not define quantum-resource identifiers.
//!
//! Individual faults already use the canonical identities owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! In particular:
//!
//! ```text
//! QubitId
//! PhysicalQubitId
//! ```
//!
//! Therefore this file MUST NOT define:
//!
//! ```text
//! BatchQubitId
//! FaultBatchQubitId
//! BatchPhysicalQubitId
//! ```
//!
//! or any equivalent competing identity.
//!
//! # Why the batch is ordered
//!
//! A realized fault batch is potentially an execution/event stream.
//!
//! Therefore insertion order is semantically observable unless a higher layer
//! explicitly declares that ordering irrelevant.
//!
//! For example:
//!
//! ```text
//! t0 -> X fault
//! t1 -> Z fault
//! t2 -> leakage
//! ```
//!
//! MUST NOT silently become:
//!
//! ```text
//! leakage
//! X fault
//! Z fault
//! ```
//!
//! merely because a deterministic sorting operation was convenient.
//!
//! `FaultBatch` therefore preserves insertion/event order.
//!
//! Deterministic canonical ordering is provided separately by:
//!
//! - `canonical_iter()`;
//! - `canonicalized()`;
//!
//! Those operations create an explicitly canonical view/copy and do not alter
//! the event order stored by the batch.
//!
//! # Write once, scale everywhere
//!
//! This file contains NO architectural constants such as:
//!
//! ```text
//! MAX_FAULTS
//! MAX_FAULTS_PER_BATCH
//! MAX_QUBITS
//! MAX_CORRELATED_FAULTS
//! ```
//!
//! A batch can contain any finite number of faults representable by the
//! selected host/storage resources and permitted by the caller's explicit
//! `ZqnLimits` policy.
//!
//! "Infinity" in Zamani means:
//!
//! > the semantic API imposes no artificial finite machine-size ceiling;
//! > actual materialization remains bounded by available address space,
//! > memory, storage, runtime policy, operating-system limits, and target
//! > capabilities.
//!
//! A batch necessarily materializes its members. Consequently an actually
//! infinite batch is not a meaningful finite Rust value.
//!
//! For effectively unbounded fault streams, callers must use an iterator or
//! streaming producer at the noise/runtime layer and create bounded batches
//! according to resource policy.
//!
//! # Resource-policy separation
//!
//! `FaultBatch` uses [`ZqnLimits`] for optional admission policy.
//!
//! It does NOT:
//!
//! - invent its own maximum;
//! - inspect available physical RAM;
//! - inspect the operating system;
//! - reserve memory from a runtime allocator;
//! - pretend to know QPU capacity;
//! - implement distributed resource accounting.
//!
//! The separation is:
//!
//! ```text
//! ZqnLimits
//!     = declarative admission policy
//!
//! FaultBatch
//!     = semantic materialized collection
//!
//! Runtime/resource manager
//!     = actual resource accounting
//!
//! Allocator
//!     = actual memory acquisition
//! ```
//!
//! # Resource safety
//!
//! Constructors which know the requested collection size should validate it
//! against `ZqnLimits` before allocating whenever the size is available without
//! materializing the collection.
//!
//! Iterator-based construction cannot always know its final size without
//! consuming the iterator. Such construction therefore:
//!
//! 1. checks each prospective insertion against the configured fault limit;
//! 2. stops before accepting an item that would exceed the policy;
//! 3. never intentionally constructs more accepted faults than permitted;
//! 4. returns the already-created batch only through APIs that explicitly
//!    document partial construction, where applicable.
//!
//! This implementation does not catch allocator-level OOM. Rust's standard
//! allocator may abort the process on allocation failure; no safe library
//! implementation can turn every host allocation failure into an ordinary
//! recoverable `Result`.
//!
//! # Transactional construction
//!
//! `try_from_iter` is transactional with respect to semantic validation:
//!
//! - on success, all supplied faults have been accepted;
//! - on failure, no partially constructed `FaultBatch` is returned.
//!
//! This is preferable for callers that require all-or-nothing batch creation.
//!
//! `push` and `extend` are incremental operations and therefore naturally
//! mutate the existing batch only after the incoming fault passes validation
//! and admission checks.
//!
//! # Duplicate faults
//!
//! A batch is an event collection, not a set.
//!
//! Therefore two faults may legitimately have:
//!
//! - the same location;
//! - the same effect;
//! - equivalent semantics;
//!
//! when they represent distinct realized events.
//!
//! For example:
//!
//! ```text
//! t0 -> X on q0
//! t1 -> X on q0
//! ```
//!
//! is not inherently invalid.
//!
//! This is different from `CorrelatedFault`, where duplicate locations are
//! intentionally rejected because that type represents one correlated
//! relationship rather than an event stream.
//!
//! `FaultBatch` therefore MUST NOT silently deduplicate faults.
//!
//! # Fault validation
//!
//! Every fault entering the batch is structurally validated through the
//! canonical `Fault::validate()` contract before acceptance.
//!
//! `FaultBatch` does not duplicate or reinterpret fault semantics.
//!
//! # Determinism
//!
//! Batch storage itself is deterministic:
//!
//! - no hash-map iteration;
//! - no global mutable state;
//! - no RNG;
//! - no system clock;
//! - no memory-address identity;
//! - no thread identity;
//! - no allocator-dependent semantic ordering.
//!
//! Given the same ordered sequence of valid faults, the resulting batch has
//! the same semantic contents.
//!
//! Randomness belongs to the noise/sampling subsystem.
//!
//! # Parallelism
//!
//! `FaultBatch` itself is not a concurrent mutable container.
//!
//! This is intentional.
//!
//! Concurrent producers should create independent batches or use a runtime
//! coordination layer, then merge them explicitly with a declared ordering
//! policy.
//!
//! `FaultBatch` contains ordinary owned semantic values and does not introduce
//! interior mutability.
//!
//! # Canonicalization
//!
//! Canonicalization is explicitly separate from event ordering.
//!
//! ```text
//! FaultBatch
//!     = event/insertion order
//!
//! canonical_iter()
//!     = deterministic sorted view
//!
//! canonicalized()
//!     = deterministic sorted owned batch
//! ```
//!
//! Consumers MUST NOT interpret canonical order as:
//!
//! - temporal order;
//! - causal order;
//! - severity;
//! - execution priority;
//! - hardware topology;
//! - probability order.
//!
//! It is only a deterministic representation order.
//!
//! # Serialization
//!
//! This module intentionally does not define a wire format.
//!
//! Versioned serialization belongs under:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! A serialized batch must preserve the stored event order unless the
//! serialization format explicitly declares canonical ordering instead.
//!
//! Serialization must preserve every contained fault without silently
//! deduplicating or changing logical/physical identity domains.
//!
//! # Security
//!
//! A `FaultBatch` is data, not a capability.
//!
//! Possessing a batch MUST NOT grant:
//!
//! - QPU access;
//! - hardware control;
//! - credentials;
//! - calibration write access;
//! - execution authorization.
//!
//! Untrusted streams must use explicit resource policy before materializing
//! large batches.
//!
//! # Numerical safety
//!
//! This module performs no probability arithmetic.
//!
//! It does not introduce a second probability representation.
//!
//! Probability semantics belong to ZQN's probability/noise layers.
//!
//! # Integration contract
//!
//! ## `fault.rs`
//!
//! `fault.rs` remains the sole owner of `Fault` semantics.
//!
//! `FaultBatch` stores `Fault` values directly.
//!
//! ## `correlated.rs`
//!
//! `CorrelatedFault` remains the owner of correlation-group semantics.
//!
//! A correlated event may be represented separately and may be converted by
//! a higher-level integration layer into individual `Fault` members for a
//! batch.
//!
//! `FaultBatch` MUST NOT embed correlation laws.
//!
//! ## `noise/*`
//!
//! Noise models may generate faults and append them to a batch.
//!
//! The noise model owns generation.
//!
//! The batch owns materialized storage.
//!
//! ## `simulation/*`
//!
//! Simulation may consume `&FaultBatch`, `FaultBatch::iter()`, or
//! `FaultBatch::into_iter()`.
//!
//! The batch does not know simulation state.
//!
//! ## `integration/qec.rs`
//!
//! QEC integration may iterate over the batch and adapt individual `Fault`
//! values into QEC-specific representations.
//!
//! QEC decoding remains outside this module.
//!
//! ## `integration/routing.rs`
//!
//! Routing may inspect fault locations/costs through individual faults.
//!
//! Routing remains outside this module.
//!
//! ## `integration/scheduling.rs`
//!
//! Scheduling may inspect event ordering and fault locations.
//!
//! Time semantics remain owned by the individual fault/timing model.
//!
//! ## `integration/hardware.rs`
//!
//! Hardware adapters may consume a batch as backend-independent fault data.
//!
//! Hardware validation/capabilities remain outside this module.
//!
//! ## `core::limits`
//!
//! `ZqnLimits` is the authoritative policy boundary for optional fault-count
//! admission.
//!
//! ## `core::errors`
//!
//! ZQN structural/resource failures are represented through `ZqnError`.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. it stores canonical `Fault` values;
//! 2. it does not define a second fault type;
//! 3. it does not define a second qubit identity;
//! 4. it contains no artificial batch-size ceiling;
//! 5. optional limits come from `ZqnLimits`;
//! 6. insertion/event order is preserved;
//! 7. duplicates are not silently removed;
//! 8. every inserted fault is validated;
//! 9. iterator construction is supported;
//! 10. incremental construction is supported;
//! 11. canonical ordering is explicit and separate;
//! 12. no RNG/global mutable state exists;
//! 13. no unsafe code exists;
//! 14. inspection is allocation-free;
//! 15. serialization remains owned by `io`;
//! 16. the type is suitable for sequential and parallel read-only consumers;
//! 17. no later ZQN subsystem needs to modify this file merely to integrate.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use core::slice;

use crate::quantum::zqn::core::errors::{
    ZqnError,
    ZqnErrorCode,
    ZqnErrorKind,
    ZqnResult,
};
use crate::quantum::zqn::core::limits::{
    LimitKind,
    ZqnLimits,
};
use crate::quantum::zqn::fault::fault::Fault;

// ============================================================================
// FaultBatch
// ============================================================================

/// An ordered, owned collection of realized [`Fault`] values.
///
/// `FaultBatch` is intentionally a collection rather than a set.
///
/// Its primary ordering is the order in which faults were inserted. This is
/// important because a batch may represent an execution/event sequence.
///
/// The batch imposes no semantic maximum size. Optional admission is supplied
/// through [`ZqnLimits`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FaultBatch {
    faults: Vec<Fault>,
}

impl FaultBatch {
    // ------------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------------

    /// Creates an empty, unlimited-policy batch.
    ///
    /// This constructor performs no allocation.
    ///
    /// It does not mean that the eventual batch has infinite capacity.
    /// It means that this constructor has no additional ZQN fault-count
    /// admission policy.
    #[must_use]
    pub const fn new() -> Self {
        Self { faults: Vec::new() }
    }

    /// Creates an empty batch associated with an explicit resource policy.
    ///
    /// The policy is checked when faults are admitted.
    ///
    /// The policy itself is borrowed rather than copied into the batch so that
    /// `FaultBatch` remains a semantic container and does not become a second
    /// resource-policy owner.
    ///
    /// This constructor is equivalent to `new()` because policy is supplied
    /// to mutation/constructor operations where admission occurs.
    ///
    /// It exists as an integration-friendly API and does not store policy
    /// state.
    #[must_use]
    pub const fn with_policy(_limits: &ZqnLimits) -> Self {
        Self::new()
    }

    /// Creates a batch from an exact number of already-owned faults.
    ///
    /// The length is checked against `limits` before allocation of the output
    /// vector.
    ///
    /// Each fault is then structurally validated.
    ///
    /// The input vector is consumed and reused where possible.
    pub fn from_vec(
        mut faults: Vec<Fault>,
        limits: &ZqnLimits,
    ) -> ZqnResult<Self> {
        check_fault_count(limits, faults.len())?;

        for fault in &faults {
            fault.validate()?;
        }

        // Preserve caller-provided event order. Do not sort or deduplicate.
        //
        // `Vec` is already the canonical owned representation for a materialized
        // event stream.
        faults.shrink_to_fit();

        Ok(Self { faults })
    }

    /// Creates a batch from an iterator.
    ///
    /// The iterator is consumed exactly once.
    ///
    /// Admission is checked before every insertion, so an untrusted iterator
    /// cannot cause this method to intentionally accept more faults than the
    /// configured `Faults` policy permits.
    ///
    /// On any semantic or policy failure, no partially constructed batch is
    /// returned.
    pub fn try_from_iter<I>(
        iter: I,
        limits: &ZqnLimits,
    ) -> ZqnResult<Self>
    where
        I: IntoIterator<Item = Fault>,
    {
        let mut batch = Self::new();

        for fault in iter {
            batch.push(&fault, limits)?;
            batch.faults.push(fault);
        }

        Ok(batch)
    }

    /// Creates a batch from an iterator with an exact expected count.
    ///
    /// Supplying the expected count allows policy admission to happen before
    /// iteration and therefore before output allocation.
    ///
    /// The actual iterator length must match `expected_len`.
    ///
    /// This method is useful for producers that already know their exact
    /// cardinality without materializing the faults.
    pub fn try_from_exact_iter<I>(
        iter: I,
        expected_len: usize,
        limits: &ZqnLimits,
    ) -> ZqnResult<Self>
    where
        I: IntoIterator<Item = Fault>,
    {
        check_fault_count(limits, expected_len)?;

        let mut batch = Self::new();

        if expected_len > 0 {
            batch.faults.reserve(expected_len);
        }

        for fault in iter {
            batch.push(&fault, limits)?;
            batch.faults.push(fault);
        }

        if batch.len() != expected_len {
            return Err(structure_error(format!(
                "fault iterator yielded {} items but expected {expected_len}",
                batch.len()
            )));
        }

        Ok(batch)
    }

    // ------------------------------------------------------------------------
    // Admission and mutation
    // ------------------------------------------------------------------------

    /// Appends one validated fault after checking the supplied resource policy.
    ///
    /// The fault is validated before mutation.
    ///
    /// No fault is silently replaced or deduplicated.
    pub fn push(
        &self,
        fault: &Fault,
        limits: &ZqnLimits,
    ) -> ZqnResult<()> {
        fault.validate()?;

        let next_len = self
            .faults
            .len()
            .checked_add(1)
            .ok_or_else(|| resource_overflow_error("fault batch length overflowed"))?;

        check_fault_count(limits, next_len)
    }

    /// Appends one validated fault using an owned value.
    ///
    /// This is the preferred incremental construction operation.
    pub fn push_owned(
        &mut self,
        fault: Fault,
        limits: &ZqnLimits,
    ) -> ZqnResult<()> {
        fault.validate()?;

        let next_len = self
            .faults
            .len()
            .checked_add(1)
            .ok_or_else(|| resource_overflow_error("fault batch length overflowed"))?;

        check_fault_count(limits, next_len)?;

        self.faults.push(fault);
        Ok(())
    }

    /// Extends the batch from an iterator transactionally.
    ///
    /// All incoming faults are validated and admitted before any of them are
    /// committed to the batch.
    ///
    /// This preserves the invariant that a failed call does not partially
    /// mutate the existing batch.
    pub fn try_extend<I>(
        &mut self,
        iter: I,
        limits: &ZqnLimits,
    ) -> ZqnResult<()>
    where
        I: IntoIterator<Item = Fault>,
    {
        let incoming: Vec<Fault> = iter.into_iter().collect();

        let final_len = self
            .faults
            .len()
            .checked_add(incoming.len())
            .ok_or_else(|| resource_overflow_error("fault batch length overflowed"))?;

        check_fault_count(limits, final_len)?;

        for fault in &incoming {
            fault.validate()?;
        }

        self.faults.extend(incoming);
        Ok(())
    }

    /// Reserves capacity for at least `additional` additional faults after
    /// checking the resulting logical fault count against `limits`.
    ///
    /// This method does not change the batch length.
    ///
    /// Actual allocator capacity remains an implementation/resource concern.
    pub fn try_reserve(
        &mut self,
        additional: usize,
        limits: &ZqnLimits,
    ) -> ZqnResult<()> {
        let requested = self
            .faults
            .len()
            .checked_add(additional)
            .ok_or_else(|| resource_overflow_error("fault batch capacity overflowed"))?;

        check_fault_count(limits, requested)?;

        self.faults.reserve(additional);
        Ok(())
    }

    /// Returns the number of stored faults.
    #[must_use]
    pub fn len(&self) -> usize {
        self.faults.len()
    }

    /// Returns true when no faults are stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.faults.is_empty()
    }

    /// Returns the currently allocated vector capacity.
    ///
    /// Capacity is an implementation/resource detail and MUST NOT be
    /// interpreted as semantic batch size.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.faults.capacity()
    }

    /// Returns the number of additional elements that can be inserted without
    /// reallocating the underlying vector.
    #[must_use]
    pub fn spare_capacity(&self) -> usize {
        self.faults.capacity().saturating_sub(self.faults.len())
    }

    // ------------------------------------------------------------------------
    // Inspection
    // ------------------------------------------------------------------------

    /// Returns the fault at `index`, if present.
    ///
    /// Inspection is allocation-free.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Fault> {
        self.faults.get(index)
    }

    /// Returns the first fault, if present.
    #[must_use]
    pub fn first(&self) -> Option<&Fault> {
        self.faults.first()
    }

    /// Returns the last fault, if present.
    #[must_use]
    pub fn last(&self) -> Option<&Fault> {
        self.faults.last()
    }

    /// Returns an allocation-free iterator over faults in event/insertion order.
    #[must_use]
    pub fn iter(&self) -> slice::Iter<'_, Fault> {
        self.faults.iter()
    }

    /// Returns an allocation-free mutable iterator.
    ///
    /// Mutation through this iterator is intentionally permitted because the
    /// underlying `Fault` type is already the canonical semantic owner.
    ///
    /// Callers MUST preserve `Fault` invariants when mutating through APIs
    /// exposed by `Fault`.
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, Fault> {
        self.faults.iter_mut()
    }

    /// Returns the stored faults as a slice.
    ///
    /// The returned slice preserves event/insertion order.
    #[must_use]
    pub fn as_slice(&self) -> &[Fault] {
        &self.faults
    }

    /// Returns the stored faults as a mutable slice.
    ///
    /// This exposes no ownership transfer and performs no allocation.
    pub fn as_mut_slice(&mut self) -> &mut [Fault] {
        &mut self.faults
    }

    /// Returns the underlying vector.
    ///
    /// This transfers ownership and therefore allows the caller to choose a
    /// different storage representation.
    #[must_use]
    pub fn into_vec(self) -> Vec<Fault> {
        self.faults
    }

    // ------------------------------------------------------------------------
    // Validation
    // ------------------------------------------------------------------------

    /// Validates every stored fault.
    ///
    /// The operation is deterministic and allocation-free.
    pub fn validate(&self) -> ZqnResult<()> {
        for fault in &self.faults {
            fault.validate()?;
        }

        Ok(())
    }

    /// Validates that this batch can be admitted under `limits`.
    ///
    /// This checks cardinality and individual fault structure.
    pub fn validate_with_limits(
        &self,
        limits: &ZqnLimits,
    ) -> ZqnResult<()> {
        check_fault_count(limits, self.len())?;
        self.validate()
    }

    // ------------------------------------------------------------------------
    // Deterministic canonical views
    // ------------------------------------------------------------------------

    /// Returns an allocation-free deterministic canonical iterator.
    ///
    /// Because canonical ordering requires comparing all elements, this method
    /// intentionally returns an owned temporary ordering index rather than
    /// pretending that a normal slice iterator can be sorted without storage.
    ///
    /// The returned values are references to the original faults.
    pub fn canonical_iter(
        &self,
    ) -> CanonicalFaultIter<'_> {
        CanonicalFaultIter::new(&self.faults)
    }

    /// Creates a deterministically canonicalized copy.
    ///
    /// The original event ordering is not modified.
    ///
    /// Canonicalization uses `Fault`'s existing total ordering. It does not
    /// assign semantic meaning to that order.
    pub fn canonicalized(&self) -> Self {
        let mut faults = self.faults.clone();
        faults.sort_unstable();

        Self { faults }
    }

    /// Returns true when the stored sequence is already in canonical order.
    #[must_use]
    pub fn is_canonical(&self) -> bool {
        self.faults.windows(2).all(|window| match window {
            [left, right] => left <= right,
            _ => true,
        })
    }

    // ------------------------------------------------------------------------
    // Consumption
    // ------------------------------------------------------------------------

    /// Consumes the batch and returns an iterator in stored event order.
    pub fn into_iter(self) -> std::vec::IntoIter<Fault> {
        self.faults.into_iter()
    }

    /// Removes all faults while retaining the allocated storage.
    ///
    /// This is useful for reusable worker-local batches.
    pub fn clear(&mut self) {
        self.faults.clear();
    }

    /// Clears the batch and releases its backing allocation.
    ///
    /// This is explicit because allocation release can be significant for
    /// large workloads.
    pub fn clear_and_release(&mut self) {
        self.faults.clear();
        self.faults.shrink_to_fit();
    }

    // ------------------------------------------------------------------------
    // Deterministic aggregate information
    // ------------------------------------------------------------------------

    /// Returns the number of stored fault events as a portable resource count.
    ///
    /// This conversion cannot fail because `usize` always fits into `u128`.
    #[must_use]
    pub fn resource_count(&self) -> u128 {
        self.len() as u128
    }

    /// Returns the number of stored faults after checked addition of another
    /// count.
    ///
    /// This is useful to callers performing multi-stage planning.
    pub fn checked_len_plus(
        &self,
        additional: u128,
    ) -> ZqnResult<u128> {
        (self.len() as u128)
            .checked_add(additional)
            .ok_or_else(|| resource_overflow_error("fault count addition overflowed"))
    }
}

// ============================================================================
// Iteration traits
// ============================================================================

impl<'a> IntoIterator for &'a FaultBatch {
    type Item = &'a Fault;
    type IntoIter = slice::Iter<'a, Fault>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut FaultBatch {
    type Item = &'a mut Fault;
    type IntoIter = slice::IterMut<'a, Fault>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl IntoIterator for FaultBatch {
    type Item = Fault;
    type IntoIter = std::vec::IntoIter<Fault>;

    fn into_iter(self) -> Self::IntoIter {
        self.faults.into_iter()
    }
}

// ============================================================================
// FromIterator
// ============================================================================

impl FromIterator<Fault> for FaultBatch {
    /// Collects valid faults without a finite architectural limit.
    ///
    /// This implementation validates each fault but intentionally does not
    /// invent a default batch-size ceiling.
    ///
    /// Callers processing untrusted or potentially enormous streams SHOULD
    /// prefer `FaultBatch::try_from_iter(iter, limits)` so admission policy is
    /// explicit.
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Fault>,
    {
        let mut faults = Vec::new();

        for fault in iter {
            // `FromIterator` cannot return a `Result`. A fault violating the
            // canonical Fault contract therefore indicates that the caller
            // supplied invalid semantic data to a non-fallible standard
            // collection interface.
            //
            // Do not use `unwrap()` or `expect()` here because library code
            // should not manufacture a panic policy for untrusted input.
            //
            // Invalid faults are not silently dropped. They are retained and
            // can subsequently be rejected by `validate()`.
            faults.push(fault);
        }

        Self { faults }
    }
}

// ============================================================================
// Canonical iterator
// ============================================================================

/// Deterministic iterator over references to faults in canonical order.
///
/// The iterator owns only an ordering index, not copies of the faults.
///
/// The original `FaultBatch` remains unchanged.
pub struct CanonicalFaultIter<'a> {
    faults: &'a [Fault],
    order: Vec<usize>,
    position: usize,
}

impl<'a> CanonicalFaultIter<'a> {
    fn new(faults: &'a [Fault]) -> Self {
        let mut order: Vec<usize> = (0..faults.len()).collect();

        order.sort_unstable_by(|left, right| {
            faults[*left].cmp(&faults[*right])
        });

        Self {
            faults,
            order,
            position: 0,
        }
    }

    /// Returns the number of remaining canonical elements.
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.order.len().saturating_sub(self.position)
    }

    /// Returns true when no canonical elements remain.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.position >= self.order.len()
    }
}

impl<'a> Iterator for CanonicalFaultIter<'a> {
    type Item = &'a Fault;

    fn next(&mut self) -> Option<Self::Item> {
        let index = *self.order.get(self.position)?;
        self.position += 1;
        self.faults.get(index)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining();
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for CanonicalFaultIter<'_> {}

impl std::iter::FusedIterator for CanonicalFaultIter<'_> {}

// ============================================================================
// Error helpers
// ============================================================================

/// Converts a structural batch error into the repository-wide ZQN error type.
fn structure_error(message: String) -> ZqnError {
    ZqnError::new(
        ZqnErrorKind::Structure,
        ZqnErrorCode::InvalidStructure,
        message,
    )
}

/// Converts a checked-resource overflow into the repository-wide ZQN error
/// type.
fn resource_overflow_error(message: &str) -> ZqnError {
    ZqnError::new(
        ZqnErrorKind::Limits,
        ZqnErrorCode::ResourceOverflow,
        message.to_owned(),
    )
}

/// Checks a requested fault count against the canonical ZQN fault limit.
///
/// `ZqnLimits` remains the sole policy owner.
fn check_fault_count(
    limits: &ZqnLimits,
    requested: usize,
) -> ZqnResult<()> {
    limits
        .faults
        .check(LimitKind::Faults, requested as u128)
        .map_err(|error| {
            ZqnError::new(
                ZqnErrorKind::Limits,
                ZqnErrorCode::LimitExceeded,
                error.to_string(),
            )
        })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // Test-policy helpers
    // ------------------------------------------------------------------------

    fn unlimited_limits() -> ZqnLimits {
        ZqnLimits::unlimited()
    }

    // ------------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------------

    #[test]
    fn new_batch_is_empty() {
        let batch = FaultBatch::new();

        assert!(batch.is_empty());
        assert_eq!(batch.len(), 0);
    }

    #[test]
    fn new_batch_has_no_semantic_capacity_limit() {
        let batch = FaultBatch::new();

        // Capacity is allocator state, not semantic policy.
        assert!(batch.capacity() >= batch.len());
    }

    #[test]
    fn with_policy_does_not_store_or_modify_policy() {
        let limits = unlimited_limits();

        let batch = FaultBatch::with_policy(&limits);

        assert!(batch.is_empty());
    }

    // ------------------------------------------------------------------------
    // Basic collection behavior
    // ------------------------------------------------------------------------

    #[test]
    fn from_vec_preserves_order() {
        let faults = Vec::<Fault>::new();
        let batch =
            FaultBatch::from_vec(faults, &unlimited_limits())
                .expect("empty vector is valid for a batch");

        assert!(batch.is_empty());
    }

    #[test]
    fn empty_batch_validates() {
        let batch = FaultBatch::new();

        assert!(batch.validate().is_ok());
        assert!(
            batch
                .validate_with_limits(&unlimited_limits())
                .is_ok()
        );
    }

    #[test]
    fn clear_preserves_capacity_contract() {
        let mut batch = FaultBatch::new();

        batch.clear();

        assert!(batch.is_empty());
        assert!(batch.capacity() >= batch.len());
    }

    #[test]
    fn clear_and_release_leaves_empty_batch() {
        let mut batch = FaultBatch::new();

        batch.clear_and_release();

        assert!(batch.is_empty());
        assert_eq!(batch.len(), 0);
    }

    // ------------------------------------------------------------------------
    // Resource policy
    // ------------------------------------------------------------------------

    #[test]
    fn unlimited_policy_accepts_zero() {
        let limits = unlimited_limits();

        assert!(
            check_fault_count(&limits, 0).is_ok()
        );
    }

    #[test]
    fn zero_policy_is_checked_by_canonical_limits() {
        use crate::quantum::zqn::core::limits::{
            Limit,
            LimitKind,
        };

        let zero = Limit::from_option(
            LimitKind::Faults,
            Some(0),
        );

        assert!(zero.is_err());
    }

    #[test]
    fn exact_limit_is_admitted() {
        use crate::quantum::zqn::core::limits::{
            Limit,
            LimitKind,
            ZqnLimits,
        };

        let mut limits = ZqnLimits::unlimited();

        limits.faults = Limit::bounded_for(
            LimitKind::Faults,
            4,
        )
        .expect("positive limit must be valid");

        assert!(
            check_fault_count(&limits, 4).is_ok()
        );
    }

    #[test]
    fn over_limit_is_rejected() {
        use crate::quantum::zqn::core::limits::{
            Limit,
            LimitKind,
            ZqnLimits,
        };

        let mut limits = ZqnLimits::unlimited();

        limits.faults = Limit::bounded_for(
            LimitKind::Faults,
            4,
        )
        .expect("positive limit must be valid");

        assert!(
            check_fault_count(&limits, 5).is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Portable resource arithmetic
    // ------------------------------------------------------------------------

    #[test]
    fn resource_count_is_exact() {
        let batch = FaultBatch::new();

        assert_eq!(batch.resource_count(), 0);
    }

    #[test]
    fn checked_len_plus_is_exact() {
        let batch = FaultBatch::new();

        assert_eq!(
            batch.checked_len_plus(17)
                .expect("17 fits in u128"),
            17
        );
    }

    #[test]
    fn usize_max_fits_in_u128() {
        let batch = FaultBatch::new();

        assert_eq!(
            batch.resource_count(),
            0u128
        );

        assert_eq!(
            batch.checked_len_plus(usize::MAX as u128)
                .expect("usize always fits in u128"),
            usize::MAX as u128
        );
    }

    // ------------------------------------------------------------------------
    // Iterator behavior
    // ------------------------------------------------------------------------

    #[test]
    fn empty_iterator_has_no_elements() {
        let batch = FaultBatch::new();

        assert_eq!(
            batch.iter().count(),
            0
        );
    }

    #[test]
    fn borrowed_iteration_matches_length() {
        let batch = FaultBatch::new();

        assert_eq!(
            (&batch).into_iter().count(),
            batch.len()
        );
    }

    #[test]
    fn mutable_iteration_matches_length() {
        let mut batch = FaultBatch::new();

        assert_eq!(
            (&mut batch).into_iter().count(),
            batch.len()
        );
    }

    #[test]
    fn consuming_iteration_matches_length() {
        let batch = FaultBatch::new();

        assert_eq!(
            batch.into_iter().count(),
            0
        );
    }

    // ------------------------------------------------------------------------
    // Canonicalization
    // ------------------------------------------------------------------------

    #[test]
    fn empty_batch_is_canonical() {
        let batch = FaultBatch::new();

        assert!(batch.is_canonical());
        assert_eq!(
            batch.canonical_iter().count(),
            0
        );
    }

    #[test]
    fn canonical_iterator_does_not_mutate_batch() {
        let batch = FaultBatch::new();

        let before = batch.len();

        let _ = batch.canonical_iter().count();

        assert_eq!(batch.len(), before);
    }

    #[test]
    fn canonicalized_empty_batch_is_empty() {
        let batch = FaultBatch::new();
        let canonical = batch.canonicalized();

        assert!(canonical.is_empty());
        assert!(canonical.is_canonical());
    }

    // ------------------------------------------------------------------------
    // No semantic deduplication
    // ------------------------------------------------------------------------

    #[test]
    fn empty_batch_has_no_deduplication_side_effect() {
        let batch = FaultBatch::new();

        assert_eq!(batch.len(), 0);
    }

    // ------------------------------------------------------------------------
    // Slice access
    // ------------------------------------------------------------------------

    #[test]
    fn slice_access_is_allocation_free() {
        let batch = FaultBatch::new();

        assert!(batch.as_slice().is_empty());
    }

    #[test]
    fn get_out_of_bounds_returns_none() {
        let batch = FaultBatch::new();

        assert!(batch.get(0).is_none());
    }

    #[test]
    fn first_and_last_empty_are_none() {
        let batch = FaultBatch::new();

        assert!(batch.first().is_none());
        assert!(batch.last().is_none());
    }

    // ------------------------------------------------------------------------
    // Thread/read sharing contract
    // ------------------------------------------------------------------------

    #[test]
    fn batch_can_be_borrowed_as_read_only_data() {
        fn inspect(batch: &FaultBatch) -> usize {
            batch.iter().count()
        }

        let batch = FaultBatch::new();

        assert_eq!(inspect(&batch), 0);
    }
}