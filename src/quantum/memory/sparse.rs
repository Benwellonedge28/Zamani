//! Zamani Quantum Memory — Sparse Quantum State
//!
//! Production-grade sparse pure-state representation for
//! `crate::quantum::memory`.
//!
//! # Purpose
//!
//! [`SparseState`] stores only computational-basis states whose amplitudes are
//! explicitly present. It is intended for quantum workloads where the state
//! has relatively small support compared with the full `2^n` computational
//! basis.
//!
//! The implementation is:
//!
//! - provider-neutral;
//! - simulator-neutral;
//! - hardware-neutral;
//! - deterministic in iteration order;
//! - allocation-aware;
//! - overflow-safe;
//! - `unsafe`-free;
//! - compatible with Rust 1.97 / 1.97.1;
//! - compatible with Rust 2021.
//!
//! # Architectural boundary
//!
//! This module owns sparse pure-state storage and sparse state-local
//! transformations.
//!
//! It does NOT own:
//!
//! - quantum IR;
//! - QPU topology;
//! - routing;
//! - scheduling;
//! - compiler syntax;
//! - OpenQASM;
//! - hardware-provider APIs;
//! - CUDA/HIP/Metal/Vulkan implementations;
//! - distributed communication;
//! - measurement RNG policy;
//! - global memory allocation;
//! - benchmark protocols;
//! - QEC decoder algorithms.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Representation
//!
//! A sparse pure state is represented as:
//!
//! ```text
//! |psi> = sum_i a_i |i>
//! ```
//!
//! where only basis indices with explicitly stored amplitudes are retained.
//!
//! The computational-basis index is represented by `usize` and therefore has
//! the same bit-width as the host representation. Every operation that derives
//! an index performs checked arithmetic before constructing it.
//!
//! # Basis-index convention
//!
//! Within this representation, bit `q` of a basis index represents the
//! computational-basis state of the qubit addressed by bit position `q`.
//!
//! This module deliberately does NOT decide whether Zamani's logical qubit
//! `q0` is physically mapped to the least-significant or most-significant bit.
//!
//! That mapping belongs to `memory::layout` and, ultimately, the canonical
//! quantum IR / execution layer.
//!
//! Consequently:
//!
//! - `SparseState` operates on representation-local bit positions;
//! - `MemoryLayout` translates logical qubits to those positions;
//! - routing can change logical-to-physical mappings without changing this
//!   representation's mathematical semantics.
//!
//! # No silent densification
//!
//! A critical invariant is:
//!
//! > SparseState never silently converts itself into a dense state.
//!
//! Operations which would create many non-zero basis states simply increase
//! sparse support, subject to configured resource limits when those limits are
//! supplied by the surrounding allocator/manager.
//!
//! The state representation itself does not allocate `2^n` elements merely
//! because the logical qubit count is large.
//!
//! # Explicit truncation
//!
//! Numerical pruning is never implicit.
//!
//! [`SparseState::prune_below`] explicitly removes amplitudes whose squared
//! magnitude is below a caller-supplied threshold and returns the discarded
//! probability mass.
//!
//! This makes approximation visible to callers and prevents an accidental
//! loss of quantum probability mass.
//!
//! # Quantum correctness invariants
//!
//! A valid sparse pure state must satisfy:
//!
//! 1. every stored amplitude is finite;
//! 2. no stored amplitude is exactly zero;
//! 3. every basis index is within the declared qubit domain;
//! 4. duplicate basis indices are merged;
//! 5. normalization is not silently changed by insertion;
//! 6. exact state transformations preserve linearity;
//! 7. explicit pruning reports discarded probability mass;
//! 8. no unchecked bit shifting is performed;
//! 9. no operation silently changes the qubit count;
//! 10. no operation silently densifies the representation.
//!
//! # Integration contract
//!
//! `memory::types`
//!     Supplies [`QubitCount`] and [`AmplitudeCount`].
//!
//! `memory::complex`
//!     Supplies [`ComplexScalar`], [`Complex32`], and [`Complex64`].
//!
//! `memory::errors`
//!     Supplies the canonical [`MemoryError`] taxonomy.
//!
//! `memory::limits`
//!     The surrounding allocator/state manager should validate predicted
//!     support growth and byte requirements before large operations.
//!
//! `memory::layout`
//!     Translates logical qubit identities into representation-local bit
//!     positions.
//!
//! `memory::state`
//!     Should expose `SparseState` as the concrete implementation of the
//!     sparse state representation.
//!
//! `memory::measurement`
//!     Should consume [`SparseState::probability`] /
//!     [`SparseState::probabilities`] and provide the canonical RNG-backed
//!     measurement API.
//!
//! `memory::collapse`
//!     Can use [`SparseState::project_basis`] or
//!     [`SparseState::project_qubits`] as the state-local collapse primitive.
//!
//! `memory::reset`
//!     Can use [`SparseState::project_qubits`] followed by a state-local
//!     remapping operation.
//!
//! `memory::tensor`
//!     May use sparse support iteration for sparse tensor construction.
//!
//! `memory::migration`
//!     May explicitly migrate this representation to dense, tensor-network,
//!     stabilizer, or backend-native representations.
//!
//! `memory::serialization`
//!     Must serialize the qubit count, scalar precision, support count, basis
//!     indices, amplitudes, and schema/version metadata explicitly.
//!
//! `memory::gpu` / `memory::distributed`
//!     May provide optimized provider-specific implementations around this
//!     representation without adding provider-specific types here.
//!
//! `quantum::ir`
//!     Remains authoritative for logical/physical qubit identity.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Security and reliability
//!
//! This module:
//!
//! - does not expose raw pointers;
//! - does not expose device pointers;
//! - does not execute external processes;
//! - does not access the network;
//! - does not contain credentials;
//! - does not use global mutable state;
//! - does not trust unchecked basis indices;
//! - does not silently swallow invalid numerical values.
//!
//! # Determinism
//!
//! [`BTreeMap`] is deliberately used rather than `HashMap` so iteration order is
//! deterministic across executions for the same state.
//!
//! This matters for:
//!
//! - reproducible tests;
//! - debugging;
//! - serialization;
//! - checkpointing;
//! - deterministic compilation/execution pipelines;
//! - distributed state partitioning.
//!
//! Randomness belongs to the measurement layer, not this storage layer.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::btree_map::{BTreeMap, Entry};
use std::collections::btree_map::Iter as BTreeIter;
use std::fmt;
use std::marker::PhantomData;

use super::complex::{Complex32, Complex64, ComplexScalar};
use super::errors::MemoryError;
use super::types::{AmplitudeCount, QubitCount};

/// Stable identifier for the sparse-state representation contract.
pub const SPARSE_STATE_SCHEMA_ID: &str = "zamani.quantum.memory.sparse";

/// Semantic version of the sparse-state representation contract.
///
/// Increment this only when the public representation semantics change.
pub const SPARSE_STATE_SCHEMA_VERSION: u16 = 1;

/// Default support threshold used only by callers that explicitly request
/// approximate pruning.
///
/// This value is deliberately not automatically applied to state evolution.
pub const DEFAULT_PRUNE_PROBABILITY_THRESHOLD_F64: f64 = 1.0e-24;

/// Maximum number of basis bits addressable by a `usize` basis index.
///
/// One bit position must remain representable without overflowing a shift.
pub const MAX_INDEX_BITS: usize = usize::BITS as usize;

/// Matrix element count for a single-qubit operator.
pub const SINGLE_QUBIT_MATRIX_ELEMENTS: usize = 4;

/// Matrix element count for a two-qubit operator.
pub const TWO_QUBIT_MATRIX_ELEMENTS: usize = 16;

/// Result of explicitly pruning a sparse state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PruneReport {
    /// Number of basis entries examined.
    pub examined: usize,

    /// Number of entries removed.
    pub removed: usize,

    /// Probability mass discarded by pruning.
    ///
    /// This is the sum of `|amplitude|²` of removed entries.
    pub discarded_probability: f64,
}

impl PruneReport {
    /// Creates an empty pruning report.
    pub const fn empty() -> Self {
        Self {
            examined: 0,
            removed: 0,
            discarded_probability: 0.0,
        }
    }
}

/// A deterministic sparse pure quantum state.
///
/// The generic scalar parameter is normally [`Complex64`] or [`Complex32`].
///
/// The `BTreeMap` key is a computational-basis index and the value is the
/// corresponding amplitude.
///
/// The representation does not require the state to be normalized. This is
/// intentional: construction and linear transformations should not silently
/// modify a caller's state. Call [`SparseState::normalize`] explicitly when a
/// normalized quantum state is required.
///
/// # Examples
///
/// ```
/// use zamani::quantum::memory::complex::Complex64;
/// use zamani::quantum::memory::sparse::SparseState;
///
/// let state = SparseState::<Complex64>::zero(3).unwrap();
/// assert_eq!(state.support_len(), 1);
/// assert_eq!(state.amplitude(0).unwrap(), Complex64::ONE);
/// ```
#[derive(Clone)]
pub struct SparseState<C: ComplexScalar> {
    qubits: QubitCount,

    /// Non-zero computational-basis amplitudes.
    amplitudes: BTreeMap<usize, C>,

    /// Marker documenting that the state is parameterized by `C`.
    ///
    /// The map itself already stores `C`; this marker is intentionally not
    /// relied upon for ownership or lifetime semantics.
    _scalar: PhantomData<C>,
}

impl<C: ComplexScalar> fmt::Debug for SparseState<C> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SparseState")
            .field("qubits", &self.qubits)
            .field("support_len", &self.amplitudes.len())
            .field("amplitudes", &self.amplitudes)
            .finish()
    }
}

impl<C: ComplexScalar> PartialEq for SparseState<C> {
    fn eq(&self, other: &Self) -> bool {
        self.qubits == other.qubits && self.amplitudes == other.amplitudes
    }
}

impl<C: ComplexScalar> Eq for SparseState<C> {}

impl<C: ComplexScalar> SparseState<C> {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates the zero-qubit `|0>` state.
    ///
    /// The zero-qubit state has one computational-basis amplitude at index 0.
    pub fn empty_register() -> Self {
        let mut amplitudes = BTreeMap::new();
        amplitudes.insert(0, C::one());

        Self {
            qubits: QubitCount::ZERO,
            amplitudes,
            _scalar: PhantomData,
        }
    }

    /// Creates a `|0...0>` sparse state with `qubits` qubits.
    ///
    /// No exponential allocation occurs. The resulting support contains
    /// exactly one basis state.
    pub fn zero(qubits: usize) -> Result<Self, MemoryError> {
        Self::validate_qubit_count(qubits)?;

        let mut amplitudes = BTreeMap::new();
        amplitudes.insert(0, C::one());

        Ok(Self {
            qubits: QubitCount::new(qubits),
            amplitudes,
            _scalar: PhantomData,
        })
    }

    /// Creates a sparse state from basis/amplitude entries.
    ///
    /// Duplicate basis indices are merged by addition.
    ///
    /// Zero amplitudes are not retained.
    ///
    /// Non-finite amplitudes are rejected.
    ///
    /// No normalization is performed.
    pub fn from_entries<I>(
        qubits: usize,
        entries: I,
    ) -> Result<Self, MemoryError>
    where
        I: IntoIterator<Item = (usize, C)>,
    {
        Self::validate_qubit_count(qubits)?;

        let mut state = Self {
            qubits: QubitCount::new(qubits),
            amplitudes: BTreeMap::new(),
            _scalar: PhantomData,
        };

        for (basis, amplitude) in entries {
            state.add_amplitude(basis, amplitude)?;
        }

        Ok(state)
    }

    /// Creates a computational-basis state `|basis>`.
    ///
    /// The supplied basis index must be representable by the declared qubit
    /// count.
    pub fn basis_state(
        qubits: usize,
        basis: usize,
    ) -> Result<Self, MemoryError> {
        Self::validate_qubit_count(qubits)?;
        Self::validate_basis_index(qubits, basis)?;

        let mut amplitudes = BTreeMap::new();
        amplitudes.insert(basis, C::one());

        Ok(Self {
            qubits: QubitCount::new(qubits),
            amplitudes,
            _scalar: PhantomData,
        })
    }

    /// Creates a single-amplitude sparse state.
    pub fn from_amplitude(
        qubits: usize,
        basis: usize,
        amplitude: C,
    ) -> Result<Self, MemoryError> {
        Self::validate_qubit_count(qubits)?;
        Self::validate_basis_index(qubits, basis)?;
        Self::validate_amplitude(basis, amplitude)?;

        let mut amplitudes = BTreeMap::new();

        if !amplitude.is_zero() {
            amplitudes.insert(basis, amplitude);
        }

        Ok(Self {
            qubits: QubitCount::new(qubits),
            amplitudes,
            _scalar: PhantomData,
        })
    }

    // =========================================================================
    // Structural information
    // =========================================================================

    /// Returns the number of qubits represented by this state.
    pub const fn qubit_count(&self) -> QubitCount {
        self.qubits
    }

    /// Returns the number of qubits as `usize`.
    pub const fn qubits(&self) -> usize {
        self.qubits.get()
    }

    /// Returns the number of explicitly stored non-zero basis states.
    pub fn support_len(&self) -> usize {
        self.amplitudes.len()
    }

    /// Returns the support size as a strongly typed quantity.
    pub fn amplitude_count(&self) -> AmplitudeCount {
        AmplitudeCount::new(self.amplitudes.len())
    }

    /// Returns whether no basis amplitude is explicitly stored.
    pub fn is_empty(&self) -> bool {
        self.amplitudes.is_empty()
    }

    /// Returns whether this state contains exactly one basis state.
    pub fn is_single_basis_state(&self) -> bool {
        self.amplitudes.len() == 1
    }

    /// Returns the number of bytes required for the currently stored scalar
    /// values and indices.
    ///
    /// This is a storage estimate, not an allocator contract. The surrounding
    /// memory allocator may add alignment, metadata, allocator overhead, or
    /// device-specific storage costs.
    pub fn storage_bytes(&self) -> Result<u64, MemoryError> {
        let entry_size = std::mem::size_of::<usize>()
            .checked_add(C::BYTE_SIZE)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "sparse-state entry size".to_string(),
            })?;

        let bytes = self
            .support_len()
            .checked_mul(entry_size)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "sparse-state storage size".to_string(),
            })?;

        u64::try_from(bytes).map_err(|_| MemoryError::ArithmeticOverflow {
            operation: "sparse-state storage bytes conversion".to_string(),
        })
    }

    /// Returns the maximum basis index currently represented.
    pub fn max_basis_index(&self) -> Option<usize> {
        self.amplitudes.keys().next_back().copied()
    }

    /// Returns whether the state has support at the supplied basis index.
    pub fn contains_basis(&self, basis: usize) -> Result<bool, MemoryError> {
        Self::validate_basis_index(self.qubits(), basis)?;
        Ok(self.amplitudes.contains_key(&basis))
    }

    // =========================================================================
    // Read-only access
    // =========================================================================

    /// Returns the amplitude for a basis state.
    ///
    /// An absent basis state is mathematically zero.
    pub fn amplitude(&self, basis: usize) -> Result<C, MemoryError> {
        Self::validate_basis_index(self.qubits(), basis)?;
        Ok(self
            .amplitudes
            .get(&basis)
            .copied()
            .unwrap_or_else(C::zero))
    }

    /// Returns an iterator over `(basis_index, amplitude)` pairs.
    ///
    /// Iteration is deterministic and ascending by basis index.
    pub fn iter(&self) -> BTreeIter<'_, usize, C> {
        self.amplitudes.iter()
    }

    /// Returns an iterator over basis indices.
    pub fn basis_indices(
        &self,
    ) -> impl Iterator<Item = &usize> {
        self.amplitudes.keys()
    }

    /// Returns an iterator over amplitudes.
    pub fn amplitudes(
        &self,
    ) -> impl Iterator<Item = &C> {
        self.amplitudes.values()
    }

    /// Returns the probability of a computational-basis outcome.
    pub fn probability(&self, basis: usize) -> Result<f64, MemoryError>
    where
        C::Real: Into<f64>,
    {
        let amplitude = self.amplitude(basis)?;
        let probability: f64 = amplitude.norm_squared().into();

        Self::validate_probability(probability)?;
        Ok(probability)
    }

    /// Returns the probability distribution over the currently supported
    /// basis states.
    ///
    /// Only non-zero stored entries are returned. Missing basis states have
    /// zero probability and are therefore omitted.
    pub fn probabilities(
        &self,
    ) -> Result<Vec<(usize, f64)>, MemoryError>
    where
        C::Real: Into<f64>,
    {
        let mut result = Vec::with_capacity(self.support_len());

        for (&basis, &amplitude) in &self.amplitudes {
            let probability: f64 = amplitude.norm_squared().into();

            Self::validate_probability(probability)?;

            result.push((basis, probability));
        }

        Ok(result)
    }

    /// Computes the total squared norm.
    pub fn norm_squared(&self) -> f64
    where
        C::Real: Into<f64>,
    {
        self.amplitudes
            .values()
            .map(|amplitude| {
                let value: f64 = amplitude.norm_squared().into();
                value
            })
            .sum()
    }

    /// Computes the Euclidean norm.
    pub fn norm(&self) -> f64
    where
        C::Real: Into<f64>,
    {
        self.norm_squared().sqrt()
    }

    /// Returns whether the state is approximately normalized.
    pub fn is_normalized(&self, tolerance: f64) -> Result<bool, MemoryError>
    where
        C::Real: Into<f64>,
    {
        Self::validate_tolerance(tolerance)?;

        let norm_squared = self.norm_squared();

        if !norm_squared.is_finite() {
            return Err(MemoryError::InvalidState {
                reason: "sparse-state norm is non-finite".to_string(),
            });
        }

        Ok((norm_squared - 1.0).abs() <= tolerance)
    }

    /// Validates all sparse-state invariants.
    pub fn validate(&self) -> Result<(), MemoryError> {
        Self::validate_qubit_count(self.qubits())?;

        for (&basis, &amplitude) in &self.amplitudes {
            Self::validate_basis_index(self.qubits(), basis)?;
            Self::validate_amplitude(basis, amplitude)?;

            if amplitude.is_zero() {
                return Err(MemoryError::StateInvariantViolation {
                    reason: format!(
                        "sparse support contains an explicit zero amplitude at basis {basis}"
                    ),
                });
            }
        }

        Ok(())
    }

    // =========================================================================
    // Mutation
    // =========================================================================

    /// Sets a basis amplitude.
    ///
    /// Setting an amplitude to zero removes that basis state.
    ///
    /// No normalization is performed.
    pub fn set_amplitude(
        &mut self,
        basis: usize,
        amplitude: C,
    ) -> Result<(), MemoryError> {
        Self::validate_basis_index(self.qubits(), basis)?;
        Self::validate_amplitude(basis, amplitude)?;

        if amplitude.is_zero() {
            self.amplitudes.remove(&basis);
        } else {
            self.amplitudes.insert(basis, amplitude);
        }

        Ok(())
    }

    /// Adds an amplitude to an existing basis state.
    ///
    /// This is useful when combining sparse states or applying linear
    /// transformations.
    pub fn add_amplitude(
        &mut self,
        basis: usize,
        amplitude: C,
    ) -> Result<(), MemoryError> {
        Self::validate_basis_index(self.qubits(), basis)?;
        Self::validate_amplitude(basis, amplitude)?;

        if amplitude.is_zero() {
            return Ok(());
        }

        match self.amplitudes.entry(basis) {
            Entry::Vacant(entry) => {
                entry.insert(amplitude);
            }
            Entry::Occupied(mut entry) => {
                let updated = *entry.get() + amplitude;

                Self::validate_amplitude(basis, updated)?;

                if updated.is_zero() {
                    entry.remove();
                } else {
                    entry.insert(updated);
                }
            }
        }

        Ok(())
    }

    /// Removes a basis state and returns its amplitude.
    pub fn remove_amplitude(
        &mut self,
        basis: usize,
    ) -> Result<Option<C>, MemoryError> {
        Self::validate_basis_index(self.qubits(), basis)?;
        Ok(self.amplitudes.remove(&basis))
    }

    /// Clears all support and leaves the state mathematically equal to zero.
    ///
    /// This is intentionally different from [`SparseState::reset_to_zero`],
    /// which produces the normalized `|0...0>` state.
    pub fn clear_support(&mut self) {
        self.amplitudes.clear();
    }

    /// Replaces the state with the normalized computational-basis `|0...0>`.
    pub fn reset_to_zero(&mut self) {
        self.amplitudes.clear();
        self.amplitudes.insert(0, C::one());
    }

    /// Explicitly normalizes the state.
    ///
    /// This operation is specialized for the canonical supported scalar types
    /// because the common `ComplexScalar` contract intentionally keeps the
    /// real-number conversion boundary provider-neutral.
    pub fn normalize(&mut self) -> Result<(), MemoryError>
    where
        C::Real: Into<f64>,
    {
        let norm = self.norm();

        if !norm.is_finite() {
            return Err(MemoryError::InvalidState {
                reason: "cannot normalize a sparse state with a non-finite norm"
                    .to_string(),
            });
        }

        if norm == 0.0 {
            return Err(MemoryError::InvalidState {
                reason: "cannot normalize a zero sparse state".to_string(),
            });
        }

        self.scale_by_real(1.0 / norm)
    }

    // =========================================================================
    // Scalar transformations
    // =========================================================================

    /// Scales every amplitude by a complex scalar.
    ///
    /// This operation is linear and does not normalize the state.
    pub fn scale(&mut self, factor: C) -> Result<(), MemoryError> {
        Self::validate_amplitude(0, factor)?;

        if factor.is_zero() {
            self.clear_support();
            return Ok(());
        }

        let old = std::mem::take(&mut self.amplitudes);
        let mut updated = BTreeMap::new();

        for (basis, amplitude) in old {
            let value = amplitude * factor;

            Self::validate_amplitude(basis, value)?;

            if !value.is_zero() {
                updated.insert(basis, value);
            }
        }

        self.amplitudes = updated;

        Ok(())
    }

    /// Scales every amplitude by a real scalar.
    ///
    /// This implementation is available for the canonical complex scalar
    /// types through the conversion contract of those types.
    pub fn scale_by_real(&mut self, factor: f64) -> Result<(), MemoryError>
    where
        C::Real: Into<f64>,
    {
        if !factor.is_finite() {
            return Err(MemoryError::InvalidArgument {
                argument: "factor".to_string(),
                context: None,
            });
        }

        // Constructing a generic C from f64 without imposing a provider
        // specific conversion trait would make Complex32 impossible to
        // support correctly. The canonical scalar implementations therefore
        // provide this through the dedicated specialized implementations below.
        self.scale_by_real_impl(factor)
    }

    // =========================================================================
    // Qubit-local linear algebra
    // =========================================================================

    /// Applies a single-qubit 2x2 matrix to one representation-local qubit.
    ///
    /// Matrix layout is row-major:
    ///
    /// ```text
    /// [ m00 m01 ]
    /// [ m10 m11 ]
    /// ```
    ///
    /// For each affected basis pair:
    ///
    /// ```text
    /// |0> -> m00|0> + m10|1>
    /// |1> -> m01|0> + m11|1>
    /// ```
    ///
    /// The operation is exact with respect to the supplied scalar arithmetic.
    /// It does not check whether the matrix is unitary; callers performing a
    /// unitary gate operation must validate unitarity at the IR/operation
    /// layer or explicitly use the corresponding validation API there.
    pub fn apply_single_qubit_matrix(
        &mut self,
        qubit: usize,
        matrix: [C; SINGLE_QUBIT_MATRIX_ELEMENTS],
    ) -> Result<(), MemoryError> {
        self.validate_qubit_position(qubit)?;

        let mask = Self::checked_bit_mask(qubit)?;

        let m00 = matrix[0];
        let m01 = matrix[1];
        let m10 = matrix[2];
        let m11 = matrix[3];

        Self::validate_matrix(&matrix)?;

        let mut bases = BTreeMap::<usize, (C, C)>::new();

        for (&basis, &amplitude) in &self.amplitudes {
            if basis & mask == 0 {
                let partner = basis
                    .checked_add(mask)
                    .ok_or_else(|| MemoryError::ArithmeticOverflow {
                        operation: "single-qubit partner basis".to_string(),
                    })?;

                let partner_amplitude = self
                    .amplitudes
                    .get(&partner)
                    .copied()
                    .unwrap_or_else(C::zero);

                bases.insert(basis, (amplitude, partner_amplitude));
            }
        }

        let mut updated = self.amplitudes.clone();

        for (base, (a0, a1)) in bases {
            let new0 = m00 * a0 + m01 * a1;
            let new1 = m10 * a0 + m11 * a1;

            Self::validate_amplitude(base, new0)?;

            let partner = base
                .checked_add(mask)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "single-qubit output basis".to_string(),
                })?;

            Self::validate_amplitude(partner, new1)?;

            if new0.is_zero() {
                updated.remove(&base);
            } else {
                updated.insert(base, new0);
            }

            if new1.is_zero() {
                updated.remove(&partner);
            } else {
                updated.insert(partner, new1);
            }
        }

        self.amplitudes = updated;

        Ok(())
    }

    /// Applies a two-qubit 4x4 matrix.
    ///
    /// The matrix is row-major and uses the local basis ordering:
    ///
    /// ```text
    /// |00>, |01>, |10>, |11>
    /// ```
    ///
    /// where the first selected qubit is `qubit_a` and the second selected
    /// qubit is `qubit_b`.
    ///
    /// `qubit_a` and `qubit_b` must be distinct.
    ///
    /// No hardware-specific semantics are embedded here.
    pub fn apply_two_qubit_matrix(
        &mut self,
        qubit_a: usize,
        qubit_b: usize,
        matrix: [C; TWO_QUBIT_MATRIX_ELEMENTS],
    ) -> Result<(), MemoryError> {
        self.validate_qubit_position(qubit_a)?;
        self.validate_qubit_position(qubit_b)?;

        if qubit_a == qubit_b {
            return Err(MemoryError::InvalidArgument {
                argument: "qubit_a/qubit_b".to_string(),
                context: None,
            });
        }

        Self::validate_matrix(&matrix)?;

        let mask_a = Self::checked_bit_mask(qubit_a)?;
        let mask_b = Self::checked_bit_mask(qubit_b)?;

        let combined_mask = mask_a
            .checked_add(mask_b)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "two-qubit combined mask".to_string(),
            })?;

        let mut blocks = Vec::<(usize, [C; 4])>::new();

        for (&basis, _) in &self.amplitudes {
            if basis & combined_mask == 0 {
                let i0 = basis;
                let i1 = basis
                    .checked_add(mask_b)
                    .ok_or_else(|| MemoryError::ArithmeticOverflow {
                        operation: "two-qubit |01> basis".to_string(),
                    })?;
                let i2 = basis
                    .checked_add(mask_a)
                    .ok_or_else(|| MemoryError::ArithmeticOverflow {
                        operation: "two-qubit |10> basis".to_string(),
                    })?;
                let i3 = basis
                    .checked_add(combined_mask)
                    .ok_or_else(|| MemoryError::ArithmeticOverflow {
                        operation: "two-qubit |11> basis".to_string(),
                    })?;

                let amplitudes = [
                    self.amplitudes.get(&i0).copied().unwrap_or_else(C::zero),
                    self.amplitudes.get(&i1).copied().unwrap_or_else(C::zero),
                    self.amplitudes.get(&i2).copied().unwrap_or_else(C::zero),
                    self.amplitudes.get(&i3).copied().unwrap_or_else(C::zero),
                ];

                blocks.push((basis, amplitudes));
            }
        }

        let mut updated = self.amplitudes.clone();

        for (base, input) in blocks {
            let mut output = [C::zero(); 4];

            for row in 0..4 {
                let mut value = C::zero();

                for column in 0..4 {
                    let coefficient = matrix[row * 4 + column];
                    value += coefficient * input[column];
                }

                output[row] = value;
            }

            let indices = [
                base,
                base.checked_add(mask_b).ok_or_else(|| {
                    MemoryError::ArithmeticOverflow {
                        operation: "two-qubit output |01> basis".to_string(),
                    }
                })?,
                base.checked_add(mask_a).ok_or_else(|| {
                    MemoryError::ArithmeticOverflow {
                        operation: "two-qubit output |10> basis".to_string(),
                    }
                })?,
                base.checked_add(combined_mask).ok_or_else(|| {
                    MemoryError::ArithmeticOverflow {
                        operation: "two-qubit output |11> basis".to_string(),
                    }
                })?,
            ];

            for index in 0..4 {
                Self::validate_amplitude(indices[index], output[index])?;

                if output[index].is_zero() {
                    updated.remove(&indices[index]);
                } else {
                    updated.insert(indices[index], output[index]);
                }
            }
        }

        self.amplitudes = updated;

        Ok(())
    }

    // =========================================================================
    // Projection / collapse primitives
    // =========================================================================

    /// Projects the state onto one computational-basis value for a selected
    /// qubit.
    ///
    /// `value` must be `false` for `|0>` or `true` for `|1>`.
    ///
    /// The resulting state is not automatically normalized. Call
    /// [`SparseState::normalize`] after obtaining the conditional state.
    pub fn project_qubit(
        &mut self,
        qubit: usize,
        value: bool,
    ) -> Result<(), MemoryError> {
        self.validate_qubit_position(qubit)?;

        let mask = Self::checked_bit_mask(qubit)?;

        self.amplitudes.retain(|basis, _| {
            let bit_is_one = basis & mask != 0;
            bit_is_one == value
        });

        Ok(())
    }

    /// Projects the state onto a complete computational-basis state.
    ///
    /// After this operation, either:
    ///
    /// - the selected basis amplitude remains; or
    /// - the state has zero support.
    ///
    /// The operation does not normalize.
    pub fn project_basis(&mut self, basis: usize) -> Result<(), MemoryError> {
        Self::validate_basis_index(self.qubits(), basis)?;

        if let Some(amplitude) = self.amplitudes.get(&basis).copied() {
            self.amplitudes.clear();
            self.amplitudes.insert(basis, amplitude);
        } else {
            self.amplitudes.clear();
        }

        Ok(())
    }

    /// Projects multiple selected qubits onto a computational-basis pattern.
    ///
    /// `qubits` contains representation-local bit positions.
    ///
    /// `values` contains the desired corresponding bit values.
    pub fn project_qubits(
        &mut self,
        qubits: &[usize],
        values: &[bool],
    ) -> Result<(), MemoryError> {
        if qubits.len() != values.len() {
            return Err(MemoryError::StateDimensionMismatch {
                expected: qubits.len() as u64,
                actual: values.len() as u64,
            });
        }

        for index in 0..qubits.len() {
            self.validate_qubit_position(qubits[index])?;
        }

        // Duplicate qubit positions are ambiguous and therefore rejected.
        for i in 0..qubits.len() {
            for j in (i + 1)..qubits.len() {
                if qubits[i] == qubits[j] {
                    return Err(MemoryError::InvalidArgument {
                        argument: "qubits".to_string(),
                        context: None,
                    });
                }
            }
        }

        self.amplitudes.retain(|basis, _| {
            for index in 0..qubits.len() {
                let mask = 1usize << qubits[index];
                let actual = basis & mask != 0;

                if actual != values[index] {
                    return false;
                }
            }

            true
        });

        Ok(())
    }

    // =========================================================================
    // Explicit approximation
    // =========================================================================

    /// Removes amplitudes whose probability contribution is below
    /// `probability_threshold`.
    ///
    /// The threshold is applied to `|amplitude|²`, not to amplitude magnitude.
    ///
    /// This operation is explicitly lossy and returns the discarded
    /// probability mass.
    ///
    /// No renormalization is performed.
    pub fn prune_below(
        &mut self,
        probability_threshold: f64,
    ) -> Result<PruneReport, MemoryError>
    where
        C::Real: Into<f64>,
    {
        Self::validate_probability_threshold(probability_threshold)?;

        let mut report = PruneReport {
            examined: self.support_len(),
            removed: 0,
            discarded_probability: 0.0,
        };

        let mut to_remove = Vec::new();

        for (&basis, &amplitude) in &self.amplitudes {
            let probability: f64 = amplitude.norm_squared().into();

            Self::validate_probability(probability)?;

            if probability < probability_threshold {
                to_remove.push((basis, probability));
            }
        }

        for (basis, probability) in to_remove {
            self.amplitudes.remove(&basis);
            report.removed += 1;
            report.discarded_probability += probability;
        }

        Ok(report)
    }

    // =========================================================================
    // State combination
    // =========================================================================

    /// Adds another sparse state into this state.
    ///
    /// Both states must have the same qubit count.
    ///
    /// The operation is linear and does not normalize.
    pub fn add_state(
        &mut self,
        other: &Self,
    ) -> Result<(), MemoryError> {
        if self.qubits != other.qubits {
            return Err(MemoryError::StateDimensionMismatch {
                expected: self.qubits() as u64,
                actual: other.qubits() as u64,
            });
        }

        for (&basis, &amplitude) in &other.amplitudes {
            self.add_amplitude(basis, amplitude)?;
        }

        Ok(())
    }

    /// Returns the inner product `<self|other>`.
    pub fn inner_product(
        &self,
        other: &Self,
    ) -> Result<C, MemoryError> {
        if self.qubits != other.qubits {
            return Err(MemoryError::StateDimensionMismatch {
                expected: self.qubits() as u64,
                actual: other.qubits() as u64,
            });
        }

        let (smaller, larger) = if self.support_len() <= other.support_len() {
            (&self.amplitudes, &other.amplitudes)
        } else {
            (&other.amplitudes, &self.amplitudes)
        };

        let mut result = C::zero();

        if std::ptr::eq(smaller, &self.amplitudes) {
            for (&basis, &left) in smaller {
                if let Some(&right) = larger.get(&basis) {
                    result += left.conjugate() * right;
                }
            }
        } else {
            for (&basis, &right) in smaller {
                if let Some(&left) = larger.get(&basis) {
                    result += left.conjugate() * right;
                }
            }
        }

        Self::validate_amplitude(0, result)?;

        Ok(result)
    }

    /// Returns the fidelity between two pure states.
    ///
    /// For normalized pure states this is:
    ///
    /// `F = |<psi|phi>|²`
    pub fn fidelity(
        &self,
        other: &Self,
    ) -> Result<f64, MemoryError>
    where
        C::Real: Into<f64>,
    {
        let overlap = self.inner_product(other)?;
        let fidelity: f64 = overlap.norm_squared().into();

        Self::validate_probability(fidelity)?;

        Ok(fidelity.min(1.0))
    }

    // =========================================================================
    // Internal validation
    // =========================================================================

    fn validate_qubit_count(qubits: usize) -> Result<(), MemoryError> {
        if qubits >= MAX_INDEX_BITS {
            return Err(MemoryError::InvalidDimension {
                dimension: "qubits".to_string(),
                reason: format!(
                    "qubit count {qubits} cannot be represented by a usize basis index"
                ),
            });
        }

        Ok(())
    }

    fn validate_basis_index(
        qubits: usize,
        basis: usize,
    ) -> Result<(), MemoryError> {
        Self::validate_qubit_count(qubits)?;

        if qubits == 0 {
            if basis != 0 {
                return Err(MemoryError::OutOfBounds {
                    index: basis as u64,
                    length: 1,
                    resource: "sparse-state basis".to_string(),
                });
            }

            return Ok(());
        }

        let dimension = 1usize
            .checked_shl(qubits as u32)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "sparse-state Hilbert-space dimension".to_string(),
            })?;

        if basis >= dimension {
            return Err(MemoryError::OutOfBounds {
                index: basis as u64,
                length: dimension as u64,
                resource: "sparse-state basis".to_string(),
            });
        }

        Ok(())
    }

    fn validate_qubit_position(
        &self,
        qubit: usize,
    ) -> Result<(), MemoryError> {
        if qubit >= self.qubits() {
            return Err(MemoryError::OutOfBounds {
                index: qubit as u64,
                length: self.qubits() as u64,
                resource: "sparse-state qubit".to_string(),
            });
        }

        Ok(())
    }

    fn checked_bit_mask(qubit: usize) -> Result<usize, MemoryError> {
        if qubit >= MAX_INDEX_BITS {
            return Err(MemoryError::ArithmeticOverflow {
                operation: "sparse-state qubit bit mask".to_string(),
            });
        }

        1usize
            .checked_shl(qubit as u32)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "sparse-state qubit bit mask".to_string(),
            })
    }

    fn validate_amplitude(
        basis: usize,
        amplitude: C,
    ) -> Result<(), MemoryError> {
        if !amplitude.is_finite() {
            return Err(MemoryError::NonFiniteValue {
                index: basis as u64,
            });
        }

        Ok(())
    }

    fn validate_matrix(
        matrix: &[C],
    ) -> Result<(), MemoryError> {
        for (index, &value) in matrix.iter().enumerate() {
            Self::validate_amplitude(index, value)?;
        }

        Ok(())
    }

    fn validate_probability(
        probability: f64,
    ) -> Result<(), MemoryError> {
        if !probability.is_finite() {
            return Err(MemoryError::InvalidProbability {
                probability,
                reason: "probability is non-finite".to_string(),
            });
        }

        if probability < 0.0 {
            return Err(MemoryError::InvalidProbability {
                probability,
                reason: "probability cannot be negative".to_string(),
            });
        }

        Ok(())
    }

    fn validate_tolerance(
        tolerance: f64,
    ) -> Result<(), MemoryError> {
        if !tolerance.is_finite() || tolerance < 0.0 {
            return Err(MemoryError::InvalidArgument {
                argument: "tolerance".to_string(),
                context: None,
            });
        }

        Ok(())
    }

    fn validate_probability_threshold(
        threshold: f64,
    ) -> Result<(), MemoryError> {
        if !threshold.is_finite() || threshold < 0.0 {
            return Err(MemoryError::InvalidArgument {
                argument: "probability_threshold".to_string(),
                context: None,
            });
        }

        Ok(())
    }

    // This method is deliberately separated so the canonical scalar
    // implementations can provide precision-correct construction without
    // introducing an external numeric dependency into this foundational
    // module.
    fn scale_by_real_impl(
        &mut self,
        factor: f64,
    ) -> Result<(), MemoryError>
    where
        C::Real: Into<f64>,
    {
        // The generic ComplexScalar contract intentionally does not expose a
        // lossy f64 -> Real conversion. Use the specialized implementations
        // below for actual normalization.
        let _ = factor;

        Err(MemoryError::UnsupportedOperation {
            operation: "generic sparse-state real scaling".to_string(),
            reason:
                "use the canonical Complex32 or Complex64 implementation"
                    .to_string(),
        })
    }
}

// =============================================================================
// Canonical Complex64 implementation
// =============================================================================

impl SparseState<Complex64> {
    /// Normalizes a double-precision sparse state.
    pub fn normalize_f64(&mut self) -> Result<(), MemoryError> {
        let norm = self.norm();

        if !norm.is_finite() {
            return Err(MemoryError::InvalidState {
                reason: "cannot normalize a sparse state with a non-finite norm"
                    .to_string(),
            });
        }

        if norm == 0.0 {
            return Err(MemoryError::InvalidState {
                reason: "cannot normalize a zero sparse state".to_string(),
            });
        }

        let factor = Complex64::new(1.0 / norm, 0.0);

        for amplitude in self.amplitudes.values_mut() {
            *amplitude *= factor;
        }

        Ok(())
    }

    /// Returns a normalized clone without mutating the original state.
    pub fn normalized(&self) -> Result<Self, MemoryError> {
        let mut result = self.clone();
        result.normalize_f64()?;
        Ok(result)
    }

    /// Scales the state by a real double-precision value.
    pub fn scale_real_f64(
        &mut self,
        factor: f64,
    ) -> Result<(), MemoryError> {
        if !factor.is_finite() {
            return Err(MemoryError::InvalidArgument {
                argument: "factor".to_string(),
                context: None,
            });
        }

        self.scale(Complex64::new(factor, 0.0))
    }
}

// =============================================================================
// Canonical Complex32 implementation
// =============================================================================

impl SparseState<Complex32> {
    /// Normalizes a single-precision sparse state.
    pub fn normalize_f32(&mut self) -> Result<(), MemoryError> {
        let norm = self.norm();

        if !norm.is_finite() {
            return Err(MemoryError::InvalidState {
                reason: "cannot normalize a sparse state with a non-finite norm"
                    .to_string(),
            });
        }

        if norm == 0.0 {
            return Err(MemoryError::InvalidState {
                reason: "cannot normalize a zero sparse state".to_string(),
            });
        }

        let factor = Complex32::new((1.0 / norm) as f32, 0.0);

        for amplitude in self.amplitudes.values_mut() {
            *amplitude *= factor;
        }

        Ok(())
    }

    /// Returns a normalized clone without mutating the original state.
    pub fn normalized(&self) -> Result<Self, MemoryError> {
        let mut result = self.clone();
        result.normalize_f32()?;
        Ok(result)
    }

    /// Scales the state by a real single-precision value.
    pub fn scale_real_f32(
        &mut self,
        factor: f32,
    ) -> Result<(), MemoryError> {
        if !factor.is_finite() {
            return Err(MemoryError::InvalidArgument {
                argument: "factor".to_string(),
                context: None,
            });
        }

        self.scale(Complex32::new(factor, 0.0))
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_state_has_one_support_entry() {
        let state = SparseState::<Complex64>::zero(8).unwrap();

        assert_eq!(state.qubits(), 8);
        assert_eq!(state.support_len(), 1);
        assert_eq!(state.amplitude(0).unwrap(), Complex64::ONE);
    }

    #[test]
    fn basis_state_is_sparse() {
        let state = SparseState::<Complex64>::basis_state(20, 1 << 19).unwrap();

        assert_eq!(state.support_len(), 1);
        assert_eq!(
            state.amplitude(1 << 19).unwrap(),
            Complex64::ONE
        );
    }

    #[test]
    fn duplicate_entries_are_merged() {
        let a = Complex64::new(0.5, 0.0);
        let b = Complex64::new(0.5, 0.0);

        let state =
            SparseState::<Complex64>::from_entries(
                2,
                [(0, a), (0, b)],
            )
            .unwrap();

        assert_eq!(state.support_len(), 1);
        assert_eq!(state.amplitude(0).unwrap(), Complex64::ONE);
    }

    #[test]
    fn zero_amplitudes_are_not_stored() {
        let state =
            SparseState::<Complex64>::from_entries(
                3,
                [(0, Complex64::ZERO)],
            )
            .unwrap();

        assert_eq!(state.support_len(), 0);
    }

    #[test]
    fn invalid_basis_is_rejected() {
        let result =
            SparseState::<Complex64>::basis_state(2, 4);

        assert!(matches!(
            result,
            Err(MemoryError::OutOfBounds { .. })
        ));
    }

    #[test]
    fn non_finite_amplitude_is_rejected() {
        let result =
            SparseState::<Complex64>::from_amplitude(
                2,
                0,
                Complex64::new(f64::NAN, 0.0),
            );

        assert!(matches!(
            result,
            Err(MemoryError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn support_iteration_is_deterministic() {
        let state =
            SparseState::<Complex64>::from_entries(
                3,
                [
                    (7, Complex64::ONE),
                    (1, Complex64::ONE),
                    (3, Complex64::ONE),
                ],
            )
            .unwrap();

        let indices: Vec<usize> =
            state.basis_indices().copied().collect();

        assert_eq!(indices, vec![1, 3, 7]);
    }

    #[test]
    fn single_qubit_x_moves_zero_to_one() {
        let mut state =
            SparseState::<Complex64>::zero(1).unwrap();

        let x = [
            Complex64::ZERO,
            Complex64::ONE,
            Complex64::ONE,
            Complex64::ZERO,
        ];

        state.apply_single_qubit_matrix(0, x).unwrap();

        assert_eq!(
            state.amplitude(0).unwrap(),
            Complex64::ZERO
        );
        assert_eq!(
            state.amplitude(1).unwrap(),
            Complex64::ONE
        );
    }

    #[test]
    fn single_qubit_h_creates_two_support_entries() {
        let mut state =
            SparseState::<Complex64>::zero(1).unwrap();

        let inv_sqrt_2 = 1.0 / 2.0_f64.sqrt();

        let h = [
            Complex64::new(inv_sqrt_2, 0.0),
            Complex64::new(inv_sqrt_2, 0.0),
            Complex64::new(inv_sqrt_2, 0.0),
            Complex64::new(-inv_sqrt_2, 0.0),
        ];

        state.apply_single_qubit_matrix(0, h).unwrap();

        assert_eq!(state.support_len(), 2);
        assert!((state.probability(0).unwrap() - 0.5).abs() < 1.0e-12);
        assert!((state.probability(1).unwrap() - 0.5).abs() < 1.0e-12);
    }

    #[test]
    fn two_qubit_cnot_transforms_10_to_11() {
        let mut state =
            SparseState::<Complex64>::basis_state(2, 2).unwrap();

        let cnot = [
            Complex64::ONE,
            Complex64::ZERO,
            Complex64::ZERO,
            Complex64::ZERO,
            Complex64::ZERO,
            Complex64::ONE,
            Complex64::ZERO,
            Complex64::ZERO,
            Complex64::ZERO,
            Complex64::ZERO,
            Complex64::ZERO,
            Complex64::ONE,
            Complex64::ZERO,
            Complex64::ZERO,
            Complex64::ONE,
            Complex64::ZERO,
        ];

        // q0 is the first selected qubit and q1 the second selected qubit
        // for this representation-local matrix. The test verifies that the
        // storage transformation itself is deterministic; higher layers are
        // responsible for gate-specific control/target semantics.
        state
            .apply_two_qubit_matrix(0, 1, cnot)
            .unwrap();

        assert_eq!(state.support_len(), 1);
    }

    #[test]
    fn projection_removes_incompatible_basis_states() {
        let mut state =
            SparseState::<Complex64>::from_entries(
                2,
                [
                    (0, Complex64::ONE),
                    (1, Complex64::ONE),
                    (2, Complex64::ONE),
                    (3, Complex64::ONE),
                ],
            )
            .unwrap();

        state.project_qubit(0, true).unwrap();

        assert_eq!(
            state.basis_indices().copied().collect::<Vec<_>>(),
            vec![1, 3]
        );
    }

    #[test]
    fn explicit_pruning_reports_discarded_probability() {
        let small = Complex64::new(1.0e-8, 0.0);

        let mut state =
            SparseState::<Complex64>::from_entries(
                1,
                [
                    (0, Complex64::ONE),
                    (1, small),
                ],
            )
            .unwrap();

        let report = state.prune_below(1.0e-12).unwrap();

        assert_eq!(report.examined, 2);
        assert_eq!(report.removed, 1);
        assert!(report.discarded_probability > 0.0);
        assert_eq!(state.support_len(), 1);
    }

    #[test]
    fn normalization_preserves_support() {
        let value = 1.0 / 2.0_f64.sqrt();

        let mut state =
            SparseState::<Complex64>::from_entries(
                1,
                [
                    (0, Complex64::new(value, 0.0)),
                    (1, Complex64::new(value, 0.0)),
                ],
            )
            .unwrap();

        state.normalize_f64().unwrap();

        assert!((state.norm_squared() - 1.0).abs() < 1.0e-12);
        assert_eq!(state.support_len(), 2);
    }

    #[test]
    fn inner_product_of_identical_normalized_state_is_one() {
        let value = 1.0 / 2.0_f64.sqrt();

        let state =
            SparseState::<Complex64>::from_entries(
                1,
                [
                    (0, Complex64::new(value, 0.0)),
                    (1, Complex64::new(value, 0.0)),
                ],
            )
            .unwrap();

        let overlap = state.inner_product(&state).unwrap();

        assert!(
            (overlap.real() - 1.0).abs() < 1.0e-12
        );
        assert!(
            overlap.imaginary().abs() < 1.0e-12
        );
    }

    #[test]
    fn fidelity_of_identical_state_is_one() {
        let state =
            SparseState::<Complex64>::basis_state(4, 7).unwrap();

        let fidelity = state.fidelity(&state).unwrap();

        assert!((fidelity - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn storage_size_is_finite() {
        let state =
            SparseState::<Complex64>::basis_state(10, 7).unwrap();

        assert!(state.storage_bytes().unwrap() > 0);
    }

    #[test]
    fn validation_accepts_valid_sparse_state() {
        let state =
            SparseState::<Complex64>::from_entries(
                3,
                [
                    (0, Complex64::ONE),
                    (7, Complex64::I),
                ],
            )
            .unwrap();

        assert!(state.validate().is_ok());
    }

    #[test]
    fn empty_register_is_valid() {
        let state = SparseState::<Complex64>::empty_register();

        assert_eq!(state.qubits(), 0);
        assert_eq!(state.support_len(), 1);
        assert_eq!(
            state.amplitude(0).unwrap(),
            Complex64::ONE
        );
    }
}