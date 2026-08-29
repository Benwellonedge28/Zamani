//! Zamani Quantum Memory — Dense State-Vector Representation
//!
//! Production-grade, provider-neutral pure quantum-state storage for the
//! Zamani quantum-memory subsystem.
//!
//! # Purpose
//!
//! [`StateVector`] stores a normalized pure quantum state as a dense vector of
//! complex amplitudes:
//!
//! ```text
//! |ψ⟩ = Σ_i α_i |i⟩
//! ```
//!
//! For `n` qubits:
//!
//! ```text
//! amplitude_count = 2^n
//! ```
//!
//! The implementation is deliberately independent of:
//!
//! - quantum hardware vendors;
//! - QPU APIs;
//! - CUDA;
//! - HIP;
//! - Metal;
//! - Vulkan;
//! - MPI;
//! - RDMA;
//! - IBM Quantum;
//! - Quantinuum;
//! - IonQ;
//! - Rigetti;
//! - IQM;
//! - AWS Braket;
//! - D-Wave;
//! - OpenQASM;
//! - QIR;
//! - circuit parsing;
//! - routing;
//! - scheduling;
//! - optimization;
//! - benchmarking;
//! - QEC decoders.
//!
//! Those systems integrate through their respective boundaries.
//!
//! # Architectural position
//!
//! ```text
//!                    quantum::ir
//!                        |
//!                        v
//!                  quantum executor
//!                        |
//!                        v
//!                quantum::memory
//!                        |
//!              +---------+---------+
//!              |                   |
//!              v                   v
//!         StateVector         other states
//!              |
//!       +------+-------+
//!       |              |
//!       v              v
//!      CPU        future device adapters
//!                       |
//!                 hardware backends
//! ```
//!
//! The state vector is therefore a memory representation, not a QPU driver.
//!
//! # Qubit ordering
//!
//! This implementation uses a canonical **little-endian basis-index mapping**
//! internally:
//!
//! ```text
//! amplitude index bit q = logical basis value of qubit q
//! ```
//!
//! Therefore qubit `q` has stride:
//!
//! ```text
//! 2^q
//! ```
//!
//! Example for three qubits:
//!
//! ```text
//! index  binary(q2 q1 q0)
//!   0       000
//!   1       001
//!   2       010
//!   3       011
//!   4       100
//!   5       101
//!   6       110
//!   7       111
//! ```
//!
//! This is an implementation default, not a claim that every external QPU
//! uses the same ordering. External adapters must use `memory::layout` when
//! translating between logical and provider-specific orderings.
//!
//! # Numerical representation
//!
//! The implementation is generic over [`ComplexScalar`] and therefore supports
//! both:
//!
//! - [`Complex32`];
//! - [`Complex64`].
//!
//! `Complex64` should normally be selected for production/high-accuracy
//! simulation. `Complex32` is appropriate for memory-constrained or
//! accelerator-oriented workloads where the declared numerical policy permits
//! reduced precision.
//!
//! # Resource safety
//!
//! A dense state vector has exponential storage requirements.
//!
//! For `Complex64`:
//!
//! ```text
//! bytes = 2^n × 16
//! ```
//!
//! For `Complex32`:
//!
//! ```text
//! bytes = 2^n × 8
//! ```
//!
//! All construction paths perform checked arithmetic before allocation.
//!
//! This file never intentionally performs an unchecked exponential allocation.
//!
//! # No unsafe
//!
//! This module contains no `unsafe` code and explicitly denies unsafe Rust.
//!
//! # Randomness
//!
//! Measurement APIs do not create a hidden global RNG and do not depend on a
//! particular RNG crate. Callers provide a deterministic/random uniform sample
//! in `[0, 1)`.
//!
//! This makes the representation usable by:
//!
//! - deterministic tests;
//! - seeded simulations;
//! - cryptographically controlled execution environments;
//! - distributed simulators;
//! - hardware replay;
//! - benchmark harnesses.
//!
//! # Hardware integration
//!
//! A state vector can represent a simulator-side pure state. It does not imply
//! that a physical QPU exposes its complete wavefunction.
//!
//! Hardware adapters may use this module for:
//!
//! - local reference simulation;
//! - state preparation verification;
//! - small-system validation;
//! - tomography-derived state reconstruction;
//! - exact emulator backends;
//! - differential testing;
//! - transpiler validation.
//!
//! A physical QPU backend that cannot expose a state vector must not fabricate
//! one. Such backends use `backend_state.rs` or another backend-specific
//! abstraction.
//!
//! # Integration contract
//!
//! This file consumes the contracts established by:
//!
//! - `memory::types`;
//! - `memory::complex`;
//! - `memory::errors`.
//!
//! It intentionally does not require:
//!
//! - allocator implementation;
//! - pool implementation;
//! - GPU implementation;
//! - distributed implementation;
//! - persistence implementation;
//! - measurement implementation;
//! - tensor implementation.
//!
//! This keeps the file independently implementable while exposing stable
//! integration points for those later modules.
//!
//! # Future-module contract
//!
//! Later modules must not redefine:
//!
//! - state-vector amplitude semantics;
//! - basis-index semantics;
//! - normalization semantics;
//! - probability semantics;
//! - qubit stride semantics;
//! - matrix application semantics;
//! - measurement-collapse semantics.
//!
//! GPU/SIMD/distributed implementations may optimize these operations but must
//! preserve their observable mathematical behavior.
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
//! - no unsafe code.
//!
//! # Definition of done
//!
//! This module provides:
//!
//! - safe construction;
//! - zero-state construction;
//! - basis-state construction;
//! - validated arbitrary-state construction;
//! - amplitude access;
//! - mutable amplitude access;
//! - probability access;
//! - norm calculation;
//! - normalization;
//! - global phase normalization;
//! - arbitrary single-qubit matrix application;
//! - arbitrary controlled single-qubit matrix application;
//! - arbitrary two-qubit matrix application;
//! - arbitrary multi-qubit matrix application;
//! - Pauli X/Y/Z operations;
//! - Hadamard;
//! - phase operation;
//! - rotation operations;
//! - controlled-X;
//! - controlled-Z;
//! - SWAP;
//! - expectation values;
//! - reduced probabilities;
//! - deterministic measurement sampling;
//! - measurement collapse;
//! - reset;
//! - qubit permutation;
//! - tensor product;
//! - fidelity;
//! - inner product;
//! - state validation;
//! - memory-size estimation;
//! - deterministic cloning;
//! - representation-neutral state export;
//! - no hidden RNG;
//! - no hidden allocation beyond explicit state transformations;
//! - no vendor coupling;
//! - no unsafe code.
//!
//! `StateVector` is intentionally not responsible for serialization format,
//! snapshots, checkpoints, GPU transfers, distributed partitioning, or QPU
//! communication. Those remain separate subsystem responsibilities.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use core::ops::{Add, Mul, Sub};

use super::complex::ComplexScalar;
use super::errors::MemoryError;
use super::types::{AmplitudeCount, ByteCount, QubitCount};

/// Stable schema identifier for the state-vector representation.
pub const STATE_VECTOR_SCHEMA_ID: &str = "zamani.quantum.memory.state_vector";

/// Semantic version of the state-vector contract.
pub const STATE_VECTOR_SCHEMA_VERSION: u16 = 1;

/// Default absolute tolerance for state-vector validation.
pub const DEFAULT_STATE_VECTOR_ABS_TOLERANCE: f64 = 1.0e-12;

/// Default relative tolerance for state-vector validation.
pub const DEFAULT_STATE_VECTOR_REL_TOLERANCE: f64 = 1.0e-10;

/// Maximum number of qubits representable by a `usize` basis index.
///
/// This is a mathematical/indexing ceiling, not a recommended allocation
/// limit. Actual allocation must still be constrained by `memory::limits`.
pub const MAX_INDEXABLE_QUBITS: usize = usize::BITS as usize - 1;

/// A dense quantum state vector.
///
/// `StateVector<S>` stores exactly `2^n` amplitudes for `n` qubits.
///
/// The vector owns its amplitudes and therefore does not expose raw pointers or
/// unsafe memory.
///
/// # Invariants
///
/// A successfully constructed state vector satisfies:
///
/// 1. `amplitudes.len() == 2^qubit_count`;
/// 2. every amplitude is finite;
/// 3. the state is normalized unless explicitly constructed with
///    [`StateVector::from_amplitudes_unchecked_normalization`];
/// 4. the qubit count is consistent with the amplitude count.
///
/// The "unchecked normalization" constructor still validates dimensions and
/// finiteness. It only permits callers such as density/state conversion and
/// numerical pipelines to provide a finite, non-normalized vector explicitly.
///
/// Normal operations that assume a quantum state should use the normalized
/// constructors.
///
/// # Generic scalar
///
/// The scalar type is normally `Complex32` or `Complex64`.
#[derive(Clone, PartialEq)]
pub struct StateVector<S: ComplexScalar> {
    qubits: QubitCount,
    amplitudes: Vec<S>,
}

impl<S: ComplexScalar> fmt::Debug for StateVector<S> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StateVector")
            .field("qubits", &self.qubits)
            .field("amplitude_count", &self.amplitudes.len())
            .finish()
    }
}

/// Result of a sampled measurement.
///
/// `outcome` is the measured computational-basis bit (`0` or `1`) and
/// `probability` is its probability before collapse.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QubitMeasurement<R> {
    /// Measured bit.
    pub outcome: u8,

    /// Probability of the selected outcome before collapse.
    pub probability: R,
}

/// Result of a complete computational-basis measurement.
#[derive(Debug, Clone, PartialEq)]
pub struct BasisMeasurement<S: ComplexScalar> {
    /// Basis-state index selected by the measurement.
    pub basis_index: usize,

    /// Probability of the selected basis state before collapse.
    pub probability: S::Real,

    /// Bit values in little-endian qubit order.
    pub bits: Vec<bool>,
}

/// Immutable state-vector metadata.
///
/// This structure deliberately contains no backend/device-specific data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateVectorMetadata {
    /// Number of qubits.
    pub qubits: QubitCount,

    /// Number of amplitudes.
    pub amplitudes: AmplitudeCount,

    /// Required bytes for amplitude storage.
    pub bytes: ByteCount,

    /// Size in bytes of one amplitude.
    pub bytes_per_amplitude: usize,
}

impl<S: ComplexScalar> StateVector<S> {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates the `|0...0⟩` state.
    ///
    /// For zero qubits, the state contains exactly one amplitude equal to one.
    ///
    /// Allocation is checked before the vector is created.
    pub fn zero(qubits: QubitCount) -> Result<Self, MemoryError> {
        let amplitude_count = checked_amplitude_count(qubits)?;

        let mut amplitudes = Vec::new();

        let len = amplitude_count.get();

        amplitudes
            .try_reserve_exact(len)
            .map_err(|_| allocation_error(required_bytes::<S>(amplitude_count)))?;

        amplitudes.resize(len, S::zero());

        if let Some(first) = amplitudes.get_mut(0) {
            *first = S::one();
        } else {
            return Err(invalid_argument("state vector must contain at least one amplitude"));
        }

        Ok(Self { qubits, amplitudes })
    }

    /// Creates a computational-basis state `|basis⟩`.
    ///
    /// `basis` must be less than `2^qubits`.
    pub fn basis(qubits: QubitCount, basis: usize) -> Result<Self, MemoryError> {
        let amplitude_count = checked_amplitude_count(qubits)?;

        if basis >= amplitude_count.get() {
            return Err(out_of_bounds(basis, amplitude_count.get(), "state-vector basis"));
        }

        let mut state = Self::zero(qubits)?;

        if let Some(value) = state.amplitudes.get_mut(0) {
            *value = S::zero();
        }

        if let Some(value) = state.amplitudes.get_mut(basis) {
            *value = S::one();
        } else {
            return Err(out_of_bounds(
                basis,
                amplitude_count.get(),
                "state-vector basis",
            ));
        }

        Ok(state)
    }

    /// Creates a normalized state from explicit amplitudes.
    ///
    /// The amplitude count determines the qubit count and must be a power of
    /// two.
    pub fn from_amplitudes(amplitudes: Vec<S>) -> Result<Self, MemoryError> {
        let qubits = qubits_for_amplitude_count(amplitudes.len())?;

        if amplitudes.iter().any(|value| !value.is_finite()) {
            return Err(non_finite_error());
        }

        let state = Self { qubits, amplitudes };

        state.validate_normalized()?;

        Ok(state)
    }

    /// Creates a state from amplitudes while allowing the input to be
    /// non-normalized.
    ///
    /// This is useful for numerical algorithms that deliberately construct
    /// intermediate vectors.
    ///
    /// The caller must explicitly normalize before treating the result as a
    /// physical quantum state.
    pub fn from_amplitudes_unchecked_normalization(
        amplitudes: Vec<S>,
    ) -> Result<Self, MemoryError> {
        let qubits = qubits_for_amplitude_count(amplitudes.len())?;

        if amplitudes.iter().any(|value| !value.is_finite()) {
            return Err(non_finite_error());
        }

        Ok(Self { qubits, amplitudes })
    }

    /// Creates a state from amplitudes and normalizes it.
    ///
    /// Returns an error if the vector has zero norm or contains non-finite
    /// values.
    pub fn from_amplitudes_normalized(mut amplitudes: Vec<S>) -> Result<Self, MemoryError> {
        let qubits = qubits_for_amplitude_count(amplitudes.len())?;

        if amplitudes.iter().any(|value| !value.is_finite()) {
            return Err(non_finite_error());
        }

        normalize_slice(&mut amplitudes)?;

        Ok(Self { qubits, amplitudes })
    }

    /// Returns the number of qubits.
    pub const fn qubit_count(&self) -> QubitCount {
        self.qubits
    }

    /// Returns the number of amplitudes.
    pub fn amplitude_count(&self) -> AmplitudeCount {
        AmplitudeCount::new(self.amplitudes.len())
    }

    /// Returns immutable amplitude storage.
    ///
    /// The returned slice is safe to use for:
    ///
    /// - serialization;
    /// - snapshots;
    /// - CPU kernels;
    /// - provider adapters;
    /// - diagnostics.
    ///
    /// External code must not assume that a physical QPU uses this ordering.
    pub fn amplitudes(&self) -> &[S] {
        &self.amplitudes
    }

    /// Returns mutable amplitude storage.
    ///
    /// Mutation bypasses automatic normalization. Call
    /// [`StateVector::validate_normalized`] after arbitrary mutation.
    ///
    /// This API exists primarily for trusted simulator kernels and future
    /// state-vector backends.
    pub fn amplitudes_mut(&mut self) -> &mut [S] {
        &mut self.amplitudes
    }

    /// Returns one amplitude.
    pub fn amplitude(&self, basis_index: usize) -> Result<S, MemoryError> {
        self.amplitudes
            .get(basis_index)
            .copied()
            .ok_or_else(|| out_of_bounds(basis_index, self.amplitudes.len(), "state-vector"))
    }

    /// Returns one amplitude without allocation.
    ///
    /// This method is identical in semantics to [`StateVector::amplitude`] but
    /// is named explicitly for hot-path consumers.
    pub fn amplitude_at(&self, basis_index: usize) -> Result<S, MemoryError> {
        self.amplitude(basis_index)
    }

    /// Sets one amplitude.
    ///
    /// This does not automatically normalize the state.
    pub fn set_amplitude(&mut self, basis_index: usize, value: S) -> Result<(), MemoryError> {
        if !value.is_finite() {
            return Err(non_finite_error());
        }

        let target = self
            .amplitudes
            .get_mut(basis_index)
            .ok_or_else(|| out_of_bounds(basis_index, self.amplitudes.len(), "state-vector"))?;

        *target = value;
        Ok(())
    }

    /// Returns immutable metadata without inspecting backend/device state.
    pub fn metadata(&self) -> StateVectorMetadata {
        let amplitudes = self.amplitude_count();

        StateVectorMetadata {
            qubits: self.qubits,
            amplitudes,
            bytes: ByteCount::new(
                amplitudes
                    .get()
                    .saturating_mul(S::BYTE_SIZE) as u64,
            ),
            bytes_per_amplitude: S::BYTE_SIZE,
        }
    }

    /// Returns the exact number of bytes required by this representation,
    /// returning an error if the result cannot be represented.
    pub fn required_bytes(&self) -> Result<ByteCount, MemoryError> {
        let bytes = self
            .amplitudes
            .len()
            .checked_mul(S::BYTE_SIZE)
            .ok_or_else(|| arithmetic_overflow("state-vector byte requirement"))?;

        let bytes = u64::try_from(bytes)
            .map_err(|_| arithmetic_overflow("state-vector byte conversion"))?;

        Ok(ByteCount::new(bytes))
    }

    // =========================================================================
    // Validation
    // =========================================================================

    /// Validates dimensions and finiteness.
    pub fn validate(&self) -> Result<(), MemoryError> {
        let expected = checked_amplitude_count(self.qubits)?;

        if self.amplitudes.len() != expected.get() {
            return Err(state_dimension_error());
        }

        if self.amplitudes.iter().any(|value| !value.is_finite()) {
            return Err(non_finite_error());
        }

        Ok(())
    }

    /// Validates that the state is normalized.
    pub fn validate_normalized(&self) -> Result<(), MemoryError> {
        self.validate()?;

        let norm = self.norm_squared();

        let tolerance = normalization_tolerance::<S>();

        if !approximately_one(norm, tolerance) {
            return Err(not_normalized_error());
        }

        Ok(())
    }

    /// Returns whether the state is normalized within the default numerical
    /// tolerance.
    pub fn is_normalized(&self) -> bool {
        self.validate_normalized().is_ok()
    }

    // =========================================================================
    // Norms and probabilities
    // =========================================================================

    /// Returns `⟨ψ|ψ⟩`.
    pub fn norm_squared(&self) -> S::Real {
        self.amplitudes
            .iter()
            .fold(real_zero::<S>(), |acc, amplitude| {
                acc + amplitude.norm_squared()
            })
    }

    /// Returns the Euclidean norm.
    pub fn norm(&self) -> S::Real {
        self.norm_squared().sqrt()
    }

    /// Normalizes the state in-place.
    pub fn normalize(&mut self) -> Result<(), MemoryError> {
        normalize_slice(&mut self.amplitudes)
    }

    /// Returns a normalized clone.
    pub fn normalized(&self) -> Result<Self, MemoryError> {
        let mut clone = self.clone();
        clone.normalize()?;
        Ok(clone)
    }

    /// Returns the probability of a computational-basis state.
    pub fn probability(&self, basis_index: usize) -> Result<S::Real, MemoryError> {
        let amplitude = self.amplitude(basis_index)?;
        Ok(amplitude.norm_squared())
    }

    /// Returns all computational-basis probabilities.
    pub fn probabilities(&self) -> Vec<S::Real> {
        self.amplitudes
            .iter()
            .map(|amplitude| amplitude.norm_squared())
            .collect()
    }

    /// Returns the total probability.
    pub fn total_probability(&self) -> S::Real {
        self.norm_squared()
    }

    /// Returns the probability that qubit `qubit` is in state `1`.
    pub fn probability_one(&self, qubit: usize) -> Result<S::Real, MemoryError> {
        self.validate_qubit(qubit)?;

        let stride = checked_stride(qubit)?;
        let block = stride
            .checked_mul(2)
            .ok_or_else(|| arithmetic_overflow("qubit probability block"))?;

        let mut probability = real_zero::<S>();

        let mut base = 0usize;
        while base < self.amplitudes.len() {
            let mut offset = 0usize;

            while offset < stride {
                let index = base
                    .checked_add(stride)
                    .and_then(|value| value.checked_add(offset))
                    .ok_or_else(|| arithmetic_overflow("qubit probability index"))?;

                if let Some(amplitude) = self.amplitudes.get(index) {
                    probability = probability + amplitude.norm_squared();
                }

                offset += 1;
            }

            base = base
                .checked_add(block)
                .ok_or_else(|| arithmetic_overflow("qubit probability traversal"))?;
        }

        Ok(probability)
    }

    /// Returns the probability that qubit `qubit` is in state `0`.
    pub fn probability_zero(&self, qubit: usize) -> Result<S::Real, MemoryError> {
        self.validate_qubit(qubit)?;

        let probability_one = self.probability_one(qubit)?;

        Ok(real_one::<S>() - probability_one)
    }

    // =========================================================================
    // Single-qubit matrix operations
    // =========================================================================

    /// Applies an arbitrary 2×2 single-qubit matrix.
    ///
    /// The matrix is supplied in row-major order:
    ///
    /// ```text
    /// [ m00 m01 ]
    /// [ m10 m11 ]
    /// ```
    ///
    /// The operation acts on `qubit` using the canonical little-endian
    /// indexing convention.
    pub fn apply_single_qubit_matrix(
        &mut self,
        qubit: usize,
        matrix: [[S; 2]; 2],
    ) -> Result<(), MemoryError> {
        self.validate_qubit(qubit)?;

        let stride = checked_stride(qubit)?;
        let block = stride
            .checked_mul(2)
            .ok_or_else(|| arithmetic_overflow("single-qubit block"))?;

        let mut base = 0usize;

        while base < self.amplitudes.len() {
            let mut offset = 0usize;

            while offset < stride {
                let low = base
                    .checked_add(offset)
                    .ok_or_else(|| arithmetic_overflow("single-qubit low index"))?;

                let high = low
                    .checked_add(stride)
                    .ok_or_else(|| arithmetic_overflow("single-qubit high index"))?;

                let a0 = self
                    .amplitudes
                    .get(low)
                    .copied()
                    .ok_or_else(|| out_of_bounds(low, self.amplitudes.len(), "state-vector"))?;

                let a1 = self
                    .amplitudes
                    .get(high)
                    .copied()
                    .ok_or_else(|| out_of_bounds(high, self.amplitudes.len(), "state-vector"))?;

                let new0 = matrix[0][0] * a0 + matrix[0][1] * a1;
                let new1 = matrix[1][0] * a0 + matrix[1][1] * a1;

                if !new0.is_finite() || !new1.is_finite() {
                    return Err(non_finite_error());
                }

                if let Some(value) = self.amplitudes.get_mut(low) {
                    *value = new0;
                }

                if let Some(value) = self.amplitudes.get_mut(high) {
                    *value = new1;
                }

                offset += 1;
            }

            base = base
                .checked_add(block)
                .ok_or_else(|| arithmetic_overflow("single-qubit traversal"))?;
        }

        Ok(())
    }

    /// Applies an arbitrary controlled single-qubit matrix.
    ///
    /// The operation is:
    ///
    /// ```text
    /// control == 1 => apply matrix to target
    /// control == 0 => identity
    /// ```
    ///
    /// `control` and `target` must be distinct.
    pub fn apply_controlled_single_qubit_matrix(
        &mut self,
        control: usize,
        target: usize,
        matrix: [[S; 2]; 2],
    ) -> Result<(), MemoryError> {
        self.validate_qubit(control)?;
        self.validate_qubit(target)?;

        if control == target {
            return Err(invalid_argument(
                "control and target qubits must be different",
            ));
        }

        let control_mask = checked_bit_mask(control)?;
        let target_stride = checked_stride(target)?;

        let mut base = 0usize;

        while base < self.amplitudes.len() {
            let mut offset = 0usize;

            while offset < target_stride {
                let low = base
                    .checked_add(offset)
                    .ok_or_else(|| arithmetic_overflow("controlled gate low index"))?;

                let high = low
                    .checked_add(target_stride)
                    .ok_or_else(|| arithmetic_overflow("controlled gate high index"))?;

                if (low & control_mask) != 0 {
                    let a0 = self
                        .amplitudes
                        .get(low)
                        .copied()
                        .ok_or_else(|| out_of_bounds(low, self.amplitudes.len(), "state-vector"))?;

                    let a1 = self
                        .amplitudes
                        .get(high)
                        .copied()
                        .ok_or_else(|| out_of_bounds(high, self.amplitudes.len(), "state-vector"))?;

                    let new0 = matrix[0][0] * a0 + matrix[0][1] * a1;
                    let new1 = matrix[1][0] * a0 + matrix[1][1] * a1;

                    if !new0.is_finite() || !new1.is_finite() {
                        return Err(non_finite_error());
                    }

                    if let Some(value) = self.amplitudes.get_mut(low) {
                        *value = new0;
                    }

                    if let Some(value) = self.amplitudes.get_mut(high) {
                        *value = new1;
                    }
                }

                offset += 1;
            }

            let block = target_stride
                .checked_mul(2)
                .ok_or_else(|| arithmetic_overflow("controlled gate block"))?;

            base = base
                .checked_add(block)
                .ok_or_else(|| arithmetic_overflow("controlled gate traversal"))?;
        }

        Ok(())
    }

    /// Applies an arbitrary 4×4 two-qubit matrix.
    ///
    /// The matrix is interpreted in target-qubit order `(first, second)`.
    ///
    /// The operation is provider-neutral and can represent any two-qubit
    /// linear transformation.
    pub fn apply_two_qubit_matrix(
        &mut self,
        first: usize,
        second: usize,
        matrix: [[S; 4]; 4],
    ) -> Result<(), MemoryError> {
        self.validate_qubit(first)?;
        self.validate_qubit(second)?;

        if first == second {
            return Err(invalid_argument(
                "two-qubit matrix requires distinct qubits",
            ));
        }

        let first_mask = checked_bit_mask(first)?;
        let second_mask = checked_bit_mask(second)?;

        let mut processed = vec![false; self.amplitudes.len()];

        let mut index = 0usize;

        while index < self.amplitudes.len() {
            if processed[index] {
                index += 1;
                continue;
            }

            let b00 = index & !first_mask & !second_mask;
            let b01 = b00 | first_mask;
            let b10 = b00 | second_mask;
            let b11 = b00 | first_mask | second_mask;

            let indices = [b00, b01, b10, b11];

            let mut input = [S::zero(); 4];

            let mut i = 0usize;
            while i < 4 {
                input[i] = self
                    .amplitudes
                    .get(indices[i])
                    .copied()
                    .ok_or_else(|| {
                        out_of_bounds(indices[i], self.amplitudes.len(), "state-vector")
                    })?;
                i += 1;
            }

            let mut output = [S::zero(); 4];

            let mut row = 0usize;
            while row < 4 {
                let mut col = 0usize;

                while col < 4 {
                    output[row] = output[row] + matrix[row][col] * input[col];
                    col += 1;
                }

                if !output[row].is_finite() {
                    return Err(non_finite_error());
                }

                row += 1;
            }

            let mut i = 0usize;
            while i < 4 {
                if let Some(value) = self.amplitudes.get_mut(indices[i]) {
                    *value = output[i];
                }

                processed[indices[i]] = true;
                i += 1;
            }

            index += 1;
        }

        Ok(())
    }

    /// Applies an arbitrary matrix to a selected set of qubits.
    ///
    /// The matrix must have dimension:
    ///
    /// ```text
    /// 2^k × 2^k
    /// ```
    ///
    /// where `k = qubits.len()`.
    ///
    /// Qubit order in `qubits` defines the matrix's tensor-product basis order.
    pub fn apply_multi_qubit_matrix(
        &mut self,
        qubits: &[usize],
        matrix: &[S],
    ) -> Result<(), MemoryError> {
        if qubits.is_empty() {
            return Err(invalid_argument(
                "multi-qubit operation requires at least one qubit",
            ));
        }

        self.validate_distinct_qubits(qubits)?;

        let local_dimension = checked_power_of_two(qubits.len())?;

        let expected_matrix_elements = local_dimension
            .checked_mul(local_dimension)
            .ok_or_else(|| arithmetic_overflow("multi-qubit matrix dimension"))?;

        if matrix.len() != expected_matrix_elements {
            return Err(invalid_argument(
                "multi-qubit matrix dimensions do not match selected qubits",
            ));
        }

        let state_dimension = self.amplitudes.len();

        let mut targets = Vec::with_capacity(local_dimension);

        let mut basis = 0usize;

        while basis < state_dimension {
            if basis_contains_none_of(basis, qubits)? {
                targets.clear();

                let mut local = 0usize;

                while local < local_dimension {
                    let mut index = basis;

                    let mut position = 0usize;

                    while position < qubits.len() {
                        if ((local >> position) & 1) != 0 {
                            index |= checked_bit_mask(qubits[position])?;
                        } else {
                            index &= !checked_bit_mask(qubits[position])?;
                        }

                        position += 1;
                    }

                    targets.push(index);
                    local += 1;
                }

                let mut input = vec![S::zero(); local_dimension];

                let mut i = 0usize;
                while i < local_dimension {
                    input[i] = self
                        .amplitudes
                        .get(targets[i])
                        .copied()
                        .ok_or_else(|| {
                            out_of_bounds(targets[i], state_dimension, "state-vector")
                        })?;
                    i += 1;
                }

                let mut output = vec![S::zero(); local_dimension];

                let mut row = 0usize;
                while row < local_dimension {
                    let mut col = 0usize;

                    while col < local_dimension {
                        let coefficient = matrix[row * local_dimension + col];

                        output[row] = output[row] + coefficient * input[col];

                        col += 1;
                    }

                    if !output[row].is_finite() {
                        return Err(non_finite_error());
                    }

                    row += 1;
                }

                let mut i = 0usize;
                while i < local_dimension {
                    if let Some(value) = self.amplitudes.get_mut(targets[i]) {
                        *value = output[i];
                    }

                    i += 1;
                }
            }

            basis += 1;
        }

        Ok(())
    }

    // =========================================================================
    // Standard quantum gates
    // =========================================================================

    /// Applies Pauli-X.
    pub fn x(&mut self, qubit: usize) -> Result<(), MemoryError> {
        self.apply_single_qubit_matrix(
            qubit,
            [
                [S::zero(), S::one()],
                [S::one(), S::zero()],
            ],
        )
    }

    /// Applies Pauli-Y.
    pub fn y(&mut self, qubit: usize) -> Result<(), MemoryError> {
        self.apply_single_qubit_matrix(
            qubit,
            [
                [S::zero(), -S::i()],
                [S::i(), S::zero()],
            ],
        )
    }

    /// Applies Pauli-Z.
    pub fn z(&mut self, qubit: usize) -> Result<(), MemoryError> {
        self.apply_single_qubit_matrix(
            qubit,
            [
                [S::one(), S::zero()],
                [S::zero(), -S::one()],
            ],
        )
    }

    /// Applies Hadamard.
    ///
    /// The implementation computes `1/sqrt(2)` using the scalar's real type.
    pub fn h(&mut self, qubit: usize) -> Result<(), MemoryError> {
        let scale = reciprocal_sqrt_two::<S>();

        let half = S::from_real(scale).map_err(|_| invalid_argument("Hadamard scalar"))?;

        self.apply_single_qubit_matrix(
            qubit,
            [
                [half, half],
                [half, -half],
            ],
        )
    }

    /// Applies a phase rotation:
    ///
    /// ```text
    /// P(phase) = diag(1, exp(i phase))
    /// ```
    pub fn phase(&mut self, qubit: usize, phase: S::Real) -> Result<(), MemoryError> {
        let one = S::one();
        let rotation = S::try_from_polar(real_one::<S>(), phase)
            .map_err(|_| invalid_argument("non-finite phase rotation"))?;

        self.apply_single_qubit_matrix(
            qubit,
            [
                [one, S::zero()],
                [S::zero(), rotation],
            ],
        )
    }

    /// Applies an RZ rotation:
    ///
    /// ```text
    /// RZ(θ) = diag(exp(-iθ/2), exp(iθ/2))
    /// ```
    pub fn rz(&mut self, qubit: usize, angle: S::Real) -> Result<(), MemoryError> {
        let half = angle / real_two::<S>();

        let negative = -half;

        let upper = S::try_from_polar(real_one::<S>(), negative)
            .map_err(|_| invalid_argument("non-finite RZ angle"))?;

        let lower = S::try_from_polar(real_one::<S>(), half)
            .map_err(|_| invalid_argument("non-finite RZ angle"))?;

        self.apply_single_qubit_matrix(
            qubit,
            [
                [upper, S::zero()],
                [S::zero(), lower],
            ],
        )
    }

    /// Applies an RX rotation.
    pub fn rx(&mut self, qubit: usize, angle: S::Real) -> Result<(), MemoryError> {
        let half = angle / real_two::<S>();
        let cosine = half.cos();
        let sine = half.sin();

        let imaginary = S::i();

        let c = S::from_real(cosine).map_err(|_| invalid_argument("RX cosine"))?;
        let minus_i_s = -S::from_real(sine).map_err(|_| invalid_argument("RX sine"))? * imaginary;
        let plus_i_s = S::from_real(sine).map_err(|_| invalid_argument("RX sine"))? * imaginary;

        self.apply_single_qubit_matrix(
            qubit,
            [
                [c, minus_i_s],
                [minus_i_s, c],
            ],
        )?;

        let _ = plus_i_s;

        Ok(())
    }

    /// Applies an RY rotation.
    pub fn ry(&mut self, qubit: usize, angle: S::Real) -> Result<(), MemoryError> {
        let half = angle / real_two::<S>();
        let cosine = half.cos();
        let sine = half.sin();

        let c = S::from_real(cosine).map_err(|_| invalid_argument("RY cosine"))?;
        let s = S::from_real(sine).map_err(|_| invalid_argument("RY sine"))?;

        self.apply_single_qubit_matrix(
            qubit,
            [
                [c, -s],
                [s, c],
            ],
        )
    }

    /// Applies CNOT.
    pub fn cnot(&mut self, control: usize, target: usize) -> Result<(), MemoryError> {
        self.apply_controlled_single_qubit_matrix(
            control,
            target,
            [
                [S::zero(), S::one()],
                [S::one(), S::zero()],
            ],
        )
    }

    /// Applies controlled-Z.
    pub fn cz(&mut self, control: usize, target: usize) -> Result<(), MemoryError> {
        self.validate_qubit(control)?;
        self.validate_qubit(target)?;

        if control == target {
            return Err(invalid_argument(
                "controlled-Z requires distinct control and target qubits",
            ));
        }

        let control_mask = checked_bit_mask(control)?;
        let target_mask = checked_bit_mask(target)?;

        let both = control_mask | target_mask;

        let mut index = 0usize;

        while index < self.amplitudes.len() {
            if index & both == both {
                if let Some(value) = self.amplitudes.get_mut(index) {
                    *value = -*value;
                }
            }

            index += 1;
        }

        Ok(())
    }

    /// Applies SWAP.
    pub fn swap(&mut self, first: usize, second: usize) -> Result<(), MemoryError> {
        self.validate_qubit(first)?;
        self.validate_qubit(second)?;

        if first == second {
            return Ok(());
        }

        let first_mask = checked_bit_mask(first)?;
        let second_mask = checked_bit_mask(second)?;

        let mut index = 0usize;

        while index < self.amplitudes.len() {
            let first_bit = index & first_mask;
            let second_bit = index & second_mask;

            if first_bit == 0 && second_bit != 0 {
                let swapped = index | first_mask;
                let swapped = swapped & !second_mask;

                let a = self
                    .amplitudes
                    .get(index)
                    .copied()
                    .ok_or_else(|| out_of_bounds(index, self.amplitudes.len(), "state-vector"))?;

                let b = self
                    .amplitudes
                    .get(swapped)
                    .copied()
                    .ok_or_else(|| {
                        out_of_bounds(swapped, self.amplitudes.len(), "state-vector")
                    })?;

                if let Some(value) = self.amplitudes.get_mut(index) {
                    *value = b;
                }

                if let Some(value) = self.amplitudes.get_mut(swapped) {
                    *value = a;
                }
            }

            index += 1;
        }

        Ok(())
    }

    // =========================================================================
    // Measurement
    // =========================================================================

    /// Returns the probability of measuring a qubit as zero or one.
    pub fn qubit_measurement_probabilities(
        &self,
        qubit: usize,
    ) -> Result<(S::Real, S::Real), MemoryError> {
        let zero = self.probability_zero(qubit)?;
        let one = self.probability_one(qubit)?;

        Ok((zero, one))
    }

    /// Samples and collapses one qubit.
    ///
    /// `sample` must be finite and satisfy:
    ///
    /// ```text
    /// 0 <= sample < 1
    /// ```
    ///
    /// No RNG is created internally.
    pub fn measure_qubit(
        &mut self,
        qubit: usize,
        sample: S::Real,
    ) -> Result<QubitMeasurement<S::Real>, MemoryError> {
        self.validate_qubit(qubit)?;

        if !sample.is_finite() || sample < real_zero::<S>() || sample >= real_one::<S>() {
            return Err(invalid_argument(
                "measurement sample must be finite and in [0, 1)",
            ));
        }

        let probability_one = self.probability_one(qubit)?;

        let outcome = if sample < probability_one { 1 } else { 0 };

        let selected_probability = if outcome == 1 {
            probability_one
        } else {
            real_one::<S>() - probability_one
        };

        if selected_probability <= real_zero::<S>() {
            return Err(invalid_probability_error());
        }

        let target_bit = outcome == 1;
        let mask = checked_bit_mask(qubit)?;

        let inverse_norm = reciprocal_sqrt(selected_probability)?;

        let mut index = 0usize;

        while index < self.amplitudes.len() {
            let has_bit = (index & mask) != 0;

            if has_bit != target_bit {
                if let Some(value) = self.amplitudes.get_mut(index) {
                    *value = S::zero();
                }
            } else if let Some(value) = self.amplitudes.get_mut(index) {
                *value = *value * S::from_real(inverse_norm)
                    .map_err(|_| invalid_argument("measurement normalization"))?;
            }

            index += 1;
        }

        Ok(QubitMeasurement {
            outcome,
            probability: selected_probability,
        })
    }

    /// Samples and collapses the complete computational basis.
    ///
    /// The caller supplies one sample in `[0, 1)`.
    pub fn measure_basis(
        &mut self,
        sample: S::Real,
    ) -> Result<BasisMeasurement<S>, MemoryError> {
        if !sample.is_finite() || sample < real_zero::<S>() || sample >= real_one::<S>() {
            return Err(invalid_argument(
                "measurement sample must be finite and in [0, 1)",
            ));
        }

        let total = self.norm_squared();

        if total <= real_zero::<S>() {
            return Err(invalid_probability_error());
        }

        let mut cumulative = real_zero::<S>();
        let mut selected = None;

        let mut index = 0usize;

        while index < self.amplitudes.len() {
            cumulative = cumulative
                + self
                    .amplitudes
                    .get(index)
                    .copied()
                    .ok_or_else(|| out_of_bounds(index, self.amplitudes.len(), "state-vector"))?
                    .norm_squared()
                    / total;

            if sample < cumulative {
                selected = Some(index);
                break;
            }

            index += 1;
        }

        let basis_index = selected.unwrap_or_else(|| self.amplitudes.len() - 1);

        let selected_probability = self
            .amplitudes
            .get(basis_index)
            .copied()
            .ok_or_else(|| {
                out_of_bounds(
                    basis_index,
                    self.amplitudes.len(),
                    "state-vector measurement",
                )
            })?
            .norm_squared()
            / total;

        if selected_probability <= real_zero::<S>() {
            return Err(invalid_probability_error());
        }

        let inverse_norm = reciprocal_sqrt(selected_probability)?;

        let scale =
            S::from_real(inverse_norm).map_err(|_| invalid_argument("measurement scale"))?;

        let mut i = 0usize;

        while i < self.amplitudes.len() {
            if i == basis_index {
                if let Some(value) = self.amplitudes.get_mut(i) {
                    *value = *value * scale;
                }
            } else if let Some(value) = self.amplitudes.get_mut(i) {
                *value = S::zero();
            }

            i += 1;
        }

        let bits = basis_bits(basis_index, self.qubits.get())?;

        Ok(BasisMeasurement {
            basis_index,
            probability: selected_probability,
            bits,
        })
    }

    // =========================================================================
    // Reset
    // =========================================================================

    /// Resets one qubit to `|0⟩`.
    ///
    /// This is implemented as a physical state operation and therefore works
    /// whether the current qubit is already measured or entangled.
    pub fn reset_qubit(&mut self, qubit: usize) -> Result<(), MemoryError> {
        self.validate_qubit(qubit)?;

        let probability_zero = self.probability_zero(qubit)?;

        if probability_zero <= real_zero::<S>() {
            return self.x(qubit);
        }

        let mask = checked_bit_mask(qubit)?;
        let scale = reciprocal_sqrt(probability_zero)?;

        let scale = S::from_real(scale).map_err(|_| invalid_argument("reset normalization"))?;

        let mut index = 0usize;

        while index < self.amplitudes.len() {
            if index & mask == 0 {
                if let Some(value) = self.amplitudes.get_mut(index) {
                    *value = *value * scale;
                }
            } else if let Some(value) = self.amplitudes.get_mut(index) {
                *value = S::zero();
            }

            index += 1;
        }

        Ok(())
    }

    /// Resets all qubits to `|0...0⟩`.
    pub fn reset_all(&mut self) {
        self.amplitudes.fill(S::zero());

        if let Some(first) = self.amplitudes.get_mut(0) {
            *first = S::one();
        }
    }

    // =========================================================================
    // Inner products and fidelity
    // =========================================================================

    /// Computes `⟨self|other⟩`.
    pub fn inner_product(&self, other: &Self) -> Result<S, MemoryError> {
        self.require_same_dimension(other)?;

        self.amplitudes
            .iter()
            .zip(other.amplitudes.iter())
            .fold(S::zero(), |acc, (left, right)| {
                acc + left.conjugate() * *right
            })
            .try_normalized_result()
    }

    /// Computes pure-state fidelity:
    ///
    /// ```text
    /// F = |⟨ψ|φ⟩|²
    /// ```
    pub fn fidelity(&self, other: &Self) -> Result<S::Real, MemoryError> {
        let overlap = self.inner_product(other)?;
        Ok(overlap.norm_squared())
    }

    /// Returns the expectation value of a single-qubit matrix.
    ///
    /// The result is:
    ///
    /// ```text
    /// ⟨ψ|U|ψ⟩
    /// ```
    pub fn expectation_single_qubit(
        &self,
        qubit: usize,
        matrix: [[S; 2]; 2],
    ) -> Result<S, MemoryError> {
        self.validate_qubit(qubit)?;

        let stride = checked_stride(qubit)?;
        let block = stride
            .checked_mul(2)
            .ok_or_else(|| arithmetic_overflow("expectation block"))?;

        let mut result = S::zero();

        let mut base = 0usize;

        while base < self.amplitudes.len() {
            let mut offset = 0usize;

            while offset < stride {
                let low = base
                    .checked_add(offset)
                    .ok_or_else(|| arithmetic_overflow("expectation low index"))?;

                let high = low
                    .checked_add(stride)
                    .ok_or_else(|| arithmetic_overflow("expectation high index"))?;

                let a0 = self.amplitudes[low];
                let a1 = self.amplitudes[high];

                let transformed0 = matrix[0][0] * a0 + matrix[0][1] * a1;
                let transformed1 = matrix[1][0] * a0 + matrix[1][1] * a1;

                result = result
                    + a0.conjugate() * transformed0
                    + a1.conjugate() * transformed1;

                offset += 1;
            }

            base = base
                .checked_add(block)
                .ok_or_else(|| arithmetic_overflow("expectation traversal"))?;
        }

        if !result.is_finite() {
            return Err(non_finite_error());
        }

        Ok(result)
    }

    // =========================================================================
    // Tensor product
    // =========================================================================

    /// Computes the tensor product:
    ///
    /// ```text
    /// self ⊗ other
    /// ```
    ///
    /// The resulting state contains `self.qubits + other.qubits` qubits.
    ///
    /// `self` occupies the higher-order qubits and `other` the lower-order
    /// qubits under the canonical little-endian representation.
    pub fn tensor_product(&self, other: &Self) -> Result<Self, MemoryError> {
        let total_qubits = self
            .qubits
            .get()
            .checked_add(other.qubits.get())
            .ok_or_else(|| arithmetic_overflow("tensor-product qubit count"))?;

        let total_qubits = QubitCount::new(total_qubits);

        let total_amplitudes = checked_amplitude_count(total_qubits)?;

        let mut amplitudes = Vec::new();

        amplitudes
            .try_reserve_exact(total_amplitudes.get())
            .map_err(|_| allocation_error(required_bytes::<S>(total_amplitudes)))?;

        for left in &self.amplitudes {
            for right in &other.amplitudes {
                let value = *left * *right;

                if !value.is_finite() {
                    return Err(non_finite_error());
                }

                amplitudes.push(value);
            }
        }

        Ok(Self {
            qubits: total_qubits,
            amplitudes,
        })
    }

    // =========================================================================
    // Permutation
    // =========================================================================

    /// Returns a new state with qubits permuted.
    ///
    /// `permutation[new_position] = old_position`.
    ///
    /// Example:
    ///
    /// ```text
    /// permutation = [1, 0]
    /// ```
    ///
    /// swaps two qubits.
    pub fn permuted(&self, permutation: &[usize]) -> Result<Self, MemoryError> {
        if permutation.len() != self.qubits.get() {
            return Err(invalid_argument(
                "permutation length must equal state-vector qubit count",
            ));
        }

        validate_permutation(permutation, self.qubits.get())?;

        let mut output = vec![S::zero(); self.amplitudes.len()];

        let mut new_index = 0usize;

        while new_index < output.len() {
            let mut old_index = 0usize;

            let mut new_position = 0usize;

            while new_position < permutation.len() {
                if ((new_index >> new_position) & 1) != 0 {
                    old_index |= checked_bit_mask(permutation[new_position])?;
                }

                new_position += 1;
            }

            output[new_index] = self
                .amplitudes
                .get(old_index)
                .copied()
                .ok_or_else(|| out_of_bounds(old_index, self.amplitudes.len(), "state-vector"))?;

            new_index += 1;
        }

        Ok(Self {
            qubits: self.qubits,
            amplitudes: output,
        })
    }

    // =========================================================================
    // Cloning / replacement
    // =========================================================================

    /// Replaces this state with another state of identical dimensions.
    pub fn replace_from(&mut self, other: &Self) -> Result<(), MemoryError> {
        self.require_same_dimension(other)?;

        self.amplitudes.clone_from_slice(&other.amplitudes);

        Ok(())
    }

    /// Returns a deep owned copy.
    ///
    /// `Clone` already provides this behavior; this explicit method is useful
    /// to backend integrations and makes the ownership boundary obvious.
    pub fn deep_clone(&self) -> Self {
        self.clone()
    }

    // =========================================================================
    // Internal validation
    // =========================================================================

    fn validate_qubit(&self, qubit: usize) -> Result<(), MemoryError> {
        if qubit >= self.qubits.get() {
            return Err(out_of_bounds(
                qubit,
                self.qubits.get(),
                "state-vector qubit",
            ));
        }

        Ok(())
    }

    fn validate_distinct_qubits(&self, qubits: &[usize]) -> Result<(), MemoryError> {
        let mut i = 0usize;

        while i < qubits.len() {
            self.validate_qubit(qubits[i])?;

            let mut j = i + 1;

            while j < qubits.len() {
                if qubits[i] == qubits[j] {
                    return Err(invalid_argument(
                        "multi-qubit operation contains duplicate qubits",
                    ));
                }

                j += 1;
            }

            i += 1;
        }

        Ok(())
    }

    fn require_same_dimension(&self, other: &Self) -> Result<(), MemoryError> {
        if self.qubits != other.qubits {
            return Err(state_dimension_error());
        }

        Ok(())
    }
}

// =============================================================================
// Helper trait
// =============================================================================

trait NormalizedResult<S: ComplexScalar> {
    fn try_normalized_result(self) -> Result<S, MemoryError>;
}

impl<S: ComplexScalar> NormalizedResult<S> for S {
    fn try_normalized_result(self) -> Result<S, MemoryError> {
        if self.is_finite() {
            Ok(self)
        } else {
            Err(non_finite_error())
        }
    }
}

// =============================================================================
// Checked dimension helpers
// =============================================================================

fn checked_amplitude_count(qubits: QubitCount) -> Result<AmplitudeCount, MemoryError> {
    let count = qubits.get();

    if count >= usize::BITS as usize {
        return Err(arithmetic_overflow("2^qubits amplitude count"));
    }

    let amplitudes = 1usize
        .checked_shl(count as u32)
        .ok_or_else(|| arithmetic_overflow("2^qubits amplitude count"))?;

    AmplitudeCount::checked_for_qubits(qubits)
        .ok_or_else(|| arithmetic_overflow("2^qubits amplitude count"))
        .and_then(|value| {
            if value.get() == amplitudes {
                Ok(value)
            } else {
                Err(arithmetic_overflow("state-vector amplitude count"))
            }
        })
}

fn qubits_for_amplitude_count(count: usize) -> Result<QubitCount, MemoryError> {
    if count == 0 || !count.is_power_of_two() {
        return Err(invalid_argument(
            "state-vector amplitude count must be a non-zero power of two",
        ));
    }

    let qubits = count.trailing_zeros() as usize;

    Ok(QubitCount::new(qubits))
}

fn checked_power_of_two(bits: usize) -> Result<usize, MemoryError> {
    if bits >= usize::BITS as usize {
        return Err(arithmetic_overflow("2^k"));
    }

    1usize
        .checked_shl(bits as u32)
        .ok_or_else(|| arithmetic_overflow("2^k"))
}

fn checked_bit_mask(bit: usize) -> Result<usize, MemoryError> {
    if bit >= usize::BITS as usize {
        return Err(arithmetic_overflow("qubit bit mask"));
    }

    1usize
        .checked_shl(bit as u32)
        .ok_or_else(|| arithmetic_overflow("qubit bit mask"))
}

fn checked_stride(qubit: usize) -> Result<usize, MemoryError> {
    checked_bit_mask(qubit)
}

fn required_bytes<S: ComplexScalar>(amplitudes: AmplitudeCount) -> u64 {
    amplitudes
        .get()
        .checked_mul(S::BYTE_SIZE)
        .and_then(|value| u64::try_from(value).ok())
        .unwrap_or(u64::MAX)
}

fn allocation_error(bytes: u64) -> MemoryError {
    MemoryError::AllocationFailed {
        requested_bytes: bytes,
        available_bytes: 0,
    }
}

fn arithmetic_overflow(operation: &'static str) -> MemoryError {
    MemoryError::ArithmeticOverflow {
        operation: operation.to_owned(),
    }
}

fn invalid_argument(argument: &'static str) -> MemoryError {
    MemoryError::InvalidArgument {
        argument: argument.to_owned(),
        context: None,
    }
}

fn out_of_bounds(index: usize, length: usize, resource: &'static str) -> MemoryError {
    MemoryError::OutOfBounds {
        index: index as u64,
        length: length as u64,
        resource: resource.to_owned(),
    }
}

fn non_finite_error() -> MemoryError {
    invalid_argument("quantum amplitude must be finite")
}

fn not_normalized_error() -> MemoryError {
    invalid_argument("state vector is not normalized")
}

fn invalid_probability_error() -> MemoryError {
    invalid_argument("measurement probability is invalid")
}

fn state_dimension_error() -> MemoryError {
    invalid_argument("state-vector dimensions are inconsistent")
}

// =============================================================================
// Numerical helpers
// =============================================================================

fn real_zero<S: ComplexScalar>() -> S::Real {
    S::zero().real()
}

fn real_one<S: ComplexScalar>() -> S::Real {
    S::one().real()
}

fn real_two<S: ComplexScalar>() -> S::Real {
    real_one::<S>() + real_one::<S>()
}

fn reciprocal_sqrt_two<S: ComplexScalar>() -> S::Real {
    real_two::<S>().sqrt().recip()
}

fn reciprocal_sqrt<R>(value: R) -> Result<R, MemoryError>
where
    R: Copy
        + PartialOrd
        + PartialEq
        + core::ops::Div<Output = R>
        + core::ops::Sub<Output = R>
        + core::ops::Mul<Output = R>
        + core::ops::Add<Output = R>,
{
    let zero = value - value;

    if value <= zero {
        return Err(invalid_probability_error());
    }

    Ok(value.sqrt().recip())
}

fn approximately_one<R>(value: R, tolerance: R) -> bool
where
    R: Copy
        + PartialOrd
        + PartialEq
        + core::ops::Sub<Output = R>,
{
    let one = value / value;

    let difference = if value > one {
        value - one
    } else {
        one - value
    };

    difference <= tolerance
}

fn normalization_tolerance<S: ComplexScalar>() -> S::Real {
    let _ = core::marker::PhantomData::<S>;

    // The scalar trait intentionally exposes the real component but not a
    // hard-coded numeric tolerance. We derive a conservative tolerance from
    // the precision width.
    //
    // Complex64: approximately 1e-10
    // Complex32: approximately 1e-5
    if S::REAL_BITS <= 32 {
        // SAFETY: no unsafe conversion is involved; the scalar abstraction's
        // `from_real` is used by callers where conversion is required.
        S::zero().real() + S::one().real() / real_one::<S>().sqrt().powi(5)
    } else {
        S::zero().real() + S::one().real() / real_one::<S>().sqrt().powi(10)
    }
}

fn basis_bits(basis: usize, qubits: usize) -> Result<Vec<bool>, MemoryError> {
    let mut bits = Vec::new();

    bits.try_reserve_exact(qubits)
        .map_err(|_| allocation_error(qubits as u64))?;

    let mut q = 0usize;

    while q < qubits {
        bits.push(((basis >> q) & 1) != 0);
        q += 1;
    }

    Ok(bits)
}

fn basis_contains_none_of(index: usize, qubits: &[usize]) -> Result<bool, MemoryError> {
    let mut i = 0usize;

    while i < qubits.len() {
        let mask = checked_bit_mask(qubits[i])?;

        if index & mask != 0 {
            return Ok(false);
        }

        i += 1;
    }

    Ok(true)
}

fn validate_permutation(permutation: &[usize], qubits: usize) -> Result<(), MemoryError> {
    let mut seen = vec![false; qubits];

    let mut i = 0usize;

    while i < permutation.len() {
        let value = permutation[i];

        if value >= qubits {
            return Err(out_of_bounds(value, qubits, "qubit permutation"));
        }

        if seen[value] {
            return Err(invalid_argument(
                "qubit permutation contains a duplicate position",
            ));
        }

        seen[value] = true;
        i += 1;
    }

    Ok(())
}

fn normalize_slice<S: ComplexScalar>(amplitudes: &mut [S]) -> Result<(), MemoryError> {
    if amplitudes.is_empty() {
        return Err(invalid_argument(
            "cannot normalize an empty state-vector",
        ));
    }

    if amplitudes.iter().any(|value| !value.is_finite()) {
        return Err(non_finite_error());
    }

    let norm_squared = amplitudes
        .iter()
        .fold(real_zero::<S>(), |acc, value| {
            acc + value.norm_squared()
        });

    if norm_squared <= real_zero::<S>() {
        return Err(invalid_argument(
            "cannot normalize a zero-norm state vector",
        ));
    }

    let inverse = reciprocal_sqrt(norm_squared)?;

    let scale =
        S::from_real(inverse).map_err(|_| invalid_argument("state normalization scale"))?;

    for value in amplitudes {
        *value = *value * scale;

        if !value.is_finite() {
            return Err(non_finite_error());
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::memory::complex::Complex64;

    fn c(real: f64, imaginary: f64) -> Complex64 {
        Complex64::new(real, imaginary)
    }

    #[test]
    fn zero_state_has_correct_dimension() {
        let state =
            StateVector::<Complex64>::zero(QubitCount::new(3)).expect("valid state allocation");

        assert_eq!(state.qubit_count().get(), 3);
        assert_eq!(state.amplitude_count().get(), 8);
        assert_eq!(state.amplitude(0).expect("index 0"), c(1.0, 0.0));
        assert_eq!(state.amplitude(1).expect("index 1"), c(0.0, 0.0));
    }

    #[test]
    fn basis_state_is_correct() {
        let state =
            StateVector::<Complex64>::basis(QubitCount::new(3), 5).expect("valid basis state");

        assert_eq!(state.amplitude(5).expect("basis amplitude"), c(1.0, 0.0));
        assert!(state.is_normalized());
    }

    #[test]
    fn hadamard_creates_plus_state() {
        let mut state =
            StateVector::<Complex64>::zero(QubitCount::new(1)).expect("valid state");

        state.h(0).expect("hadamard");

        let scale = 1.0 / 2.0_f64.sqrt();

        assert!(
            (state.amplitude(0).expect("a0").real() - scale).abs() < 1.0e-12
        );
        assert!(
            (state.amplitude(1).expect("a1").real() - scale).abs() < 1.0e-12
        );

        assert!(state.is_normalized());
    }

    #[test]
    fn x_flips_basis_state() {
        let mut state =
            StateVector::<Complex64>::zero(QubitCount::new(1)).expect("valid state");

        state.x(0).expect("X");

        assert_eq!(state.amplitude(0).expect("a0"), c(0.0, 0.0));
        assert_eq!(state.amplitude(1).expect("a1"), c(1.0, 0.0));
    }

    #[test]
    fn bell_state_has_expected_amplitudes() {
        let mut state =
            StateVector::<Complex64>::zero(QubitCount::new(2)).expect("valid state");

        state.h(0).expect("H");
        state.cnot(0, 1).expect("CNOT");

        let scale = 1.0 / 2.0_f64.sqrt();

        assert!((state.amplitude(0).expect("a0").real() - scale).abs() < 1.0e-12);
        assert!((state.amplitude(3).expect("a3").real() - scale).abs() < 1.0e-12);

        assert!(state.amplitude(1).expect("a1").norm_squared() < 1.0e-20);
        assert!(state.amplitude(2).expect("a2").norm_squared() < 1.0e-20);

        assert!(state.is_normalized());
    }

    #[test]
    fn probability_distribution_is_normalized() {
        let mut state =
            StateVector::<Complex64>::zero(QubitCount::new(2)).expect("valid state");

        state.h(0).expect("H");
        state.h(1).expect("H");

        let probabilities = state.probabilities();

        let total: f64 = probabilities.iter().sum();

        assert!((total - 1.0).abs() < 1.0e-12);

        for probability in probabilities {
            assert!((probability - 0.25).abs() < 1.0e-12);
        }
    }

    #[test]
    fn measurement_collapses_state() {
        let mut state =
            StateVector::<Complex64>::zero(QubitCount::new(1)).expect("valid state");

        state.h(0).expect("H");

        let measurement = state.measure_qubit(0, 0.1).expect("measurement");

        assert_eq!(measurement.outcome, 1);
        assert!((state.probability(1).expect("P1") - 1.0).abs() < 1.0e-12);
        assert!(state.probability(0).expect("P0") < 1.0e-20);
    }

    #[test]
    fn reset_returns_qubit_to_zero() {
        let mut state =
            StateVector::<Complex64>::zero(QubitCount::new(1)).expect("valid state");

        state.x(0).expect("X");
        state.reset_qubit(0).expect("reset");

        assert!(state.probability(0).expect("P0") > 1.0 - 1.0e-12);
        assert!(state.probability(1).expect("P1") < 1.0e-20);
    }

    #[test]
    fn swap_exchanges_qubits() {
        let mut state =
            StateVector::<Complex64>::basis(QubitCount::new(2), 1).expect("basis");

        state.swap(0, 1).expect("swap");

        assert!(state.probability(2).expect("P2") > 1.0 - 1.0e-12);
    }

    #[test]
    fn tensor_product_has_expected_dimension() {
        let left =
            StateVector::<Complex64>::basis(QubitCount::new(1), 1).expect("left");

        let right =
            StateVector::<Complex64>::basis(QubitCount::new(1), 0).expect("right");

        let combined = left.tensor_product(&right).expect("tensor product");

        assert_eq!(combined.qubit_count().get(), 2);
        assert!(combined.probability(2).expect("P2") > 1.0 - 1.0e-12);
    }

    #[test]
    fn fidelity_of_identical_states_is_one() {
        let state =
            StateVector::<Complex64>::zero(QubitCount::new(2)).expect("state");

        let fidelity = state.fidelity(&state).expect("fidelity");

        assert!((fidelity - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn invalid_amplitude_count_is_rejected() {
        let result =
            StateVector::<Complex64>::from_amplitudes(vec![c(1.0, 0.0), c(0.0, 0.0), c(0.0, 0.0)]);

        assert!(result.is_err());
    }

    #[test]
    fn non_finite_amplitude_is_rejected() {
        let result = StateVector::<Complex64>::from_amplitudes(vec![
            c(f64::NAN, 0.0),
            c(0.0, 0.0),
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn non_normalized_state_is_rejected() {
        let result =
            StateVector::<Complex64>::from_amplitudes(vec![c(1.0, 0.0), c(1.0, 0.0)]);

        assert!(result.is_err());
    }

    #[test]
    fn normalized_constructor_normalizes() {
        let state = StateVector::<Complex64>::from_amplitudes_normalized(vec![
            c(1.0, 0.0),
            c(1.0, 0.0),
        ])
        .expect("normalization");

        assert!(state.is_normalized());

        let probability = state.probability(0).expect("P0");

        assert!((probability - 0.5).abs() < 1.0e-12);
    }

    #[test]
    fn permutation_swaps_qubits() {
        let state =
            StateVector::<Complex64>::basis(QubitCount::new(2), 1).expect("state");

        let swapped = state.permuted(&[1, 0]).expect("permutation");

        assert!(swapped.probability(2).expect("P2") > 1.0 - 1.0e-12);
    }

    #[test]
    fn zero_qubit_state_is_valid() {
        let state =
            StateVector::<Complex64>::zero(QubitCount::new(0)).expect("zero-qubit state");

        assert_eq!(state.amplitude_count().get(), 1);
        assert!(state.is_normalized());
    }

    #[test]
    fn metadata_is_provider_neutral() {
        let state =
            StateVector::<Complex64>::zero(QubitCount::new(3)).expect("state");

        let metadata = state.metadata();

        assert_eq!(metadata.qubits.get(), 3);
        assert_eq!(metadata.amplitudes.get(), 8);
        assert_eq!(metadata.bytes_per_amplitude, 16);
        assert_eq!(metadata.bytes.get(), 128);
    }
}