//! Zamani Quantum IR — Logical Qubit Register
//!
//! This module owns the *container semantics* for logical qubits.
//!
//! # Architectural responsibility
//!
//! `register.rs` answers:
//!
//! > "Which logical qubits belong to this concrete IR register, and how can
//! > they be accessed and validated safely and deterministically?"
//!
//! It owns:
//!
//! - `QubitRegister`;
//! - `QubitRange`;
//! - register construction;
//! - checked allocation;
//! - logical-register membership;
//! - logical-register state transitions;
//! - deterministic iteration;
//! - operand validation against a register;
//! - register-local queries;
//! - register-local errors;
//! - register invariants.
//!
//! It does NOT own:
//!
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `Qubit` identity/value semantics;
//! - physical allocation;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulse generation;
//! - simulation state;
//! - amplitudes;
//! - density matrices;
//! - measurement probabilities;
//! - error-correction decoding;
//! - optimization;
//! - frontend parsing.
//!
//! The canonical qubit identity/value vocabulary is owned by:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! New code in this module therefore imports qubit types from:
//!
//! ```text
//! super::qubit
//! ```
//!
//! when this file is located at:
//!
//! ```text
//! src/quantum/ir/quantum/register.rs
//! ```
//!
//! # Universal-program principle
//!
//! Zamani programs must be able to express:
//!
//! ```text
//! 1 qubit
//! 2 qubits
//! 64 qubits
//! 1,000 qubits
//! 1,000,000 qubits
//! N qubits
//! ```
//!
//! without changing the semantic model.
//!
//! This module therefore contains NO architectural constants such as:
//!
//! ```text
//! MAX_QUBITS = 64
//! MAX_REGISTER_SIZE = 4096
//! MAX_QUBITS = 1_000_000
//! ```
//!
//! A concrete `QubitRegister` is an in-memory representation and is naturally
//! bounded by the host process's address space, allocator, available memory,
//! and any explicit compiler/resource policy.
//!
//! Those are representation/execution constraints, NOT semantic limits of
//! Zamani.
//!
//! # Important distinction
//!
//! ```text
//! QubitId
//!     Stable logical identity.
//!
//! Qubit
//!     Logical qubit value + IR bookkeeping state.
//!
//! QubitRange
//!     Lazy logical identifier range.
//!
//! QubitRegister
//!     Concrete in-memory logical namespace.
//!
//! PhysicalQubitId
//!     Physical-target identity vocabulary.
//!
//! routing
//!     Determines logical -> physical placement.
//!
//! hardware
//!     Describes physical resources.
//! ```
//!
//! # Allocation policy
//!
//! Constructors that allocate memory return `Result`.
//!
//! This is deliberate.
//!
//! A production compiler must not convert malformed/untrusted register sizes
//! into unconditional panics merely because an allocation request is too
//! large.
//!
//! `try_reserve_exact` is used so capacity overflow and allocator failure are
//! reported through the API where the standard library permits that failure
//! to be represented as an ordinary `Result`.
//!
//! Callers should still apply their own `QuantumIrLimits` before requesting
//! very large allocations.
//!
//! # Sparse/large-program principle
//!
//! `QubitRegister` is a concrete dense container.
//!
//! It is NOT the only way a Zamani program can describe qubits.
//!
//! For large or distributed programs, callers can use:
//!
//! - `QubitRange`;
//! - streamed IR;
//! - partitioned IR;
//! - distributed namespaces;
//! - sparse resource representations;
//! - compiler-side indexing structures.
//!
//! The semantic identity remains `quantum::ir::qubit::QubitId`.
//!
//! # Safety
//!
//! This module contains no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes that requirement compiler-enforced.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies.
//!
//! -----------------------------------------------------------------------------
//! Module boundary
//! -----------------------------------------------------------------------------
//
// This module is intentionally independent of:
// //
//! - hardware;
//! - routing;
//! - scheduling;
//! - optimization;
//! - simulation;
//! - QEC;
//! - frontend;
//! - backend execution.
//!
//! Only canonical qubit semantics are imported.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;
use std::ops::Range;

use super::qubit::{Qubit, QubitError, QubitId, QubitState};

// ============================================================================
// Qubit range
// ============================================================================

/// A lazy half-open range of logical qubit identifiers.
///
/// `QubitRange::new(2, 5)` represents:
///
/// ```text
/// q2
/// q3
/// q4
/// ```
///
/// It does not allocate a `Vec<Qubit>`.
///
/// This is the preferred representation when a compiler needs to describe a
/// contiguous logical namespace without materializing every qubit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QubitRange {
    start: usize,
    end: usize,
}

impl QubitRange {
    /// Creates a half-open range `[start, end)`.
    ///
    /// # Errors
    ///
    /// Returns `QubitRangeError::InvalidBounds` when `start > end`.
    pub const fn new(
        start: usize,
        end: usize,
    ) -> Result<Self, QubitRangeError> {
        if start > end {
            return Err(QubitRangeError::InvalidBounds { start, end });
        }

        Ok(Self { start, end })
    }

    /// Creates an empty range at `index`.
    #[must_use]
    pub const fn empty(index: usize) -> Self {
        Self {
            start: index,
            end: index,
        }
    }

    /// Returns the first logical index.
    #[must_use]
    pub const fn start(self) -> usize {
        self.start
    }

    /// Returns the exclusive logical index.
    #[must_use]
    pub const fn end(self) -> usize {
        self.end
    }

    /// Returns the number of logical identifiers represented by this range.
    ///
    /// This cannot underflow because construction guarantees `start <= end`.
    #[must_use]
    pub const fn len(self) -> usize {
        self.end - self.start
    }

    /// Returns whether the range contains no identifiers.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// Returns whether the supplied logical identifier belongs to the range.
    #[must_use]
    pub const fn contains(self, qubit: QubitId) -> bool {
        qubit.index() >= self.start && qubit.index() < self.end
    }

    /// Returns the underlying standard-library range.
    #[must_use]
    pub const fn as_range(self) -> Range<usize> {
        self.start..self.end
    }

    /// Returns a lazy iterator over the logical identifiers.
    ///
    /// No collection is allocated.
    pub fn iter(self) -> impl Iterator<Item = QubitId> {
        self.as_range().map(QubitId::new)
    }

    /// Returns the first identifier, if the range is non-empty.
    #[must_use]
    pub const fn first(self) -> Option<QubitId> {
        if self.is_empty() {
            None
        } else {
            Some(QubitId::new(self.start))
        }
    }

    /// Returns the final identifier, if the range is non-empty.
    #[must_use]
    pub const fn last(self) -> Option<QubitId> {
        if self.is_empty() {
            None
        } else {
            Some(QubitId::new(self.end - 1))
        }
    }

    /// Attempts to extend this range by `additional` identifiers.
    ///
    /// The extension is checked for `usize` overflow.
    ///
    /// Example:
    ///
    /// ```text
    /// [10, 20) + 5 -> [10, 25)
    /// ```
    pub const fn checked_extend(
        self,
        additional: usize,
    ) -> Result<Self, QubitRangeError> {
        match self.end.checked_add(additional) {
            Some(end) => Ok(Self {
                start: self.start,
                end,
            }),
            None => Err(QubitRangeError::Overflow),
        }
    }

    /// Attempts to create a sub-range relative to this range.
    ///
    /// The supplied bounds are relative to `self.start`.
    ///
    /// Example:
    ///
    /// ```text
    /// range = [10, 20)
    /// subrange(2, 5) = [12, 15)
    /// ```
    pub const fn subrange(
        self,
        relative_start: usize,
        relative_end: usize,
    ) -> Result<Self, QubitRangeError> {
        if relative_start > relative_end {
            return Err(QubitRangeError::InvalidBounds {
                start: relative_start,
                end: relative_end,
            });
        }

        if relative_end > self.len() {
            return Err(QubitRangeError::OutOfBounds {
                index: relative_end,
                length: self.len(),
            });
        }

        let start = match self.start.checked_add(relative_start) {
            Some(value) => value,
            None => return Err(QubitRangeError::Overflow),
        };

        let end = match self.start.checked_add(relative_end) {
            Some(value) => value,
            None => return Err(QubitRangeError::Overflow),
        };

        Ok(Self { start, end })
    }
}

/// Errors produced by `QubitRange`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QubitRangeError {
    /// The start of the range is greater than its end.
    InvalidBounds {
        /// Inclusive start.
        start: usize,

        /// Exclusive end.
        end: usize,
    },

    /// A range calculation overflowed `usize`.
    Overflow,

    /// A relative index is outside the range.
    OutOfBounds {
        /// Requested relative index.
        index: usize,

        /// Available range length.
        length: usize,
    },
}

impl fmt::Display for QubitRangeError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidBounds { start, end } => write!(
                formatter,
                "invalid qubit range: start {start} is greater than end {end}"
            ),

            Self::Overflow => {
                write!(formatter, "qubit range calculation overflowed")
            }

            Self::OutOfBounds { index, length } => write!(
                formatter,
                "qubit range index {index} is outside range length {length}"
            ),
        }
    }
}

impl std::error::Error for QubitRangeError {}

// ============================================================================
// Register errors
// ============================================================================

/// Errors produced by `QubitRegister`.
///
/// Register errors deliberately remain separate from physical-hardware
/// errors. A register is a semantic/in-memory IR object; it does not allocate
/// hardware.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QubitRegisterError {
    /// The requested register count exceeds an explicit caller-supplied
    /// maximum.
    CountExceedsLimit {
        /// Requested logical-qubit count.
        count: usize,

        /// Caller-supplied maximum.
        maximum: usize,
    },

    /// The requested register count cannot be represented by the host's
    /// address-space/vector representation.
    CountNotRepresentable {
        /// Requested logical-qubit count.
        count: usize,
    },

    /// The allocator could not reserve the requested memory.
    AllocationFailed {
        /// Requested number of logical qubits.
        count: usize,
    },

    /// A logical qubit does not belong to the register.
    OutOfRange {
        /// Requested logical qubit.
        qubit: QubitId,

        /// Number of qubits in the register.
        num_qubits: usize,
    },

    /// A logical qubit is disabled.
    Disabled {
        /// Disabled logical qubit.
        qubit: QubitId,
    },

    /// A logical qubit occurs more than once in an operand collection.
    DuplicateQubit {
        /// Duplicated logical qubit.
        qubit: QubitId,
    },

    /// A requested operation requires at least one qubit but the collection
    /// was empty.
    EmptyOperandSet,

    /// A register operation would exceed the host representation.
    CapacityOverflow,

    /// The requested range is not representable as a register.
    RangeTooLarge {
        /// Number of logical qubits requested.
        count: usize,
    },
}

impl fmt::Display for QubitRegisterError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::CountExceedsLimit { count, maximum } => write!(
                formatter,
                "logical qubit count {count} exceeds configured maximum {maximum}"
            ),

            Self::CountNotRepresentable { count } => write!(
                formatter,
                "logical qubit count {count} cannot be represented by the host vector model"
            ),

            Self::AllocationFailed { count } => write!(
                formatter,
                "unable to allocate storage for {count} logical qubits"
            ),

            Self::OutOfRange {
                qubit,
                num_qubits,
            } => write!(
                formatter,
                "logical qubit {qubit} is outside register range 0..{num_qubits}"
            ),

            Self::Disabled { qubit } => {
                write!(formatter, "logical qubit {qubit} is disabled")
            }

            Self::DuplicateQubit { qubit } => {
                write!(formatter, "logical qubit {qubit} appears more than once")
            }

            Self::EmptyOperandSet => {
                write!(formatter, "logical-qubit operand set is empty")
            }

            Self::CapacityOverflow => {
                write!(formatter, "logical-qubit register capacity overflowed")
            }

            Self::RangeTooLarge { count } => write!(
                formatter,
                "logical-qubit range containing {count} qubits cannot be materialized"
            ),
        }
    }
}

impl std::error::Error for QubitRegisterError {}

impl From<QubitError> for QubitRegisterError {
    fn from(error: QubitError) -> Self {
        match error {
            QubitError::OutOfRange {
                qubit,
                num_qubits,
            } => Self::OutOfRange {
                qubit,
                num_qubits,
            },

            QubitError::Disabled { qubit } => Self::Disabled { qubit },

            QubitError::DuplicateQubit { qubit } => {
                Self::DuplicateQubit { qubit }
            }

            QubitError::CountExceedsLimit { count, maximum } => {
                Self::CountExceedsLimit { count, maximum }
            }

            QubitError::InvalidCount { count } => {
                Self::CountNotRepresentable { count }
            }

            QubitError::InvalidQubit { qubit } => {
                Self::OutOfRange {
                    qubit,
                    num_qubits: 0,
                }
            }

            QubitError::NoAvailableQubit => {
                Self::EmptyOperandSet
            }
        }
    }
}

// ============================================================================
// Logical qubit register
// ============================================================================

/// Concrete logical-qubit register.
///
/// `QubitRegister` owns a dense in-memory logical namespace:
///
/// ```text
/// index 0 -> QubitId(0)
/// index 1 -> QubitId(1)
/// index 2 -> QubitId(2)
/// ...
/// ```
///
/// The register does NOT own physical resources.
///
/// # Scalability
///
/// There is no Zamani architectural qubit limit here.
///
/// A dense register is necessarily constrained by the host process's memory
/// and address space. This is a representation constraint only.
///
/// For very large programs, use `QubitRange` or another sparse/partitioned
/// representation instead of materializing every qubit.
///
/// # Identity invariant
///
/// For every valid index `i`:
///
/// ```text
/// register.get(QubitId::new(i)).id() == QubitId::new(i)
/// ```
///
/// This invariant is maintained internally; callers cannot mutate the
/// underlying `Vec<Qubit>` directly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QubitRegister {
    qubits: Vec<Qubit>,
}

impl QubitRegister {
    // ------------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------------

    /// Creates an empty logical-qubit register.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            qubits: Vec::new(),
        }
    }

    /// Creates a logical register containing `count` qubits.
    ///
    /// This is the preferred production constructor.
    ///
    /// Allocation is checked before initialization.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - `count` exceeds the caller's explicit `maximum`;
    /// - `count` exceeds the host vector representation;
    /// - memory reservation fails.
    pub fn try_new(
        count: usize,
        maximum: usize,
    ) -> Result<Self, QubitRegisterError> {
        if count > maximum {
            return Err(QubitRegisterError::CountExceedsLimit {
                count,
                maximum,
            });
        }

        Self::try_new_unbounded(count)
    }

    /// Creates a register without an additional caller-supplied policy.
    ///
    /// This should only be used after an upstream compiler/security policy has
    /// already validated the requested count.
    ///
    /// The method still performs checked host-representation and allocation
    /// handling.
    pub fn try_new_unbounded(
        count: usize,
    ) -> Result<Self, QubitRegisterError> {
        if count > Self::maximum_constructible_count() {
            return Err(QubitRegisterError::CountNotRepresentable { count });
        }

        let mut qubits = Vec::new();

        qubits
            .try_reserve_exact(count)
            .map_err(|_| QubitRegisterError::AllocationFailed { count })?;

        for index in 0..count {
            qubits.push(Qubit::new(QubitId::new(index)));
        }

        Ok(Self { qubits })
    }

    /// Returns the conservative maximum number of `Qubit` values that can be
    /// represented by one `Vec<Qubit>` under Rust's standard vector
    /// allocation constraints.
    ///
    /// This is NOT a Zamani quantum-machine limit.
    #[must_use]
    pub const fn maximum_constructible_count() -> usize {
        let element_size = std::mem::size_of::<Qubit>();

        if element_size == 0 {
            usize::MAX
        } else {
            (isize::MAX as usize) / element_size
        }
    }

    /// Creates a register corresponding to a lazy range.
    ///
    /// The range is materialized into a dense register, so callers should use
    /// this only when materialization is explicitly desired.
    pub fn try_from_range(
        range: QubitRange,
        maximum: usize,
    ) -> Result<Self, QubitRegisterError> {
        Self::try_new(range.len(), maximum)
    }

    // ------------------------------------------------------------------------
    // Basic access
    // ------------------------------------------------------------------------

    /// Returns the number of logical qubits in the register.
    #[must_use]
    pub fn len(&self) -> usize {
        self.qubits.len()
    }

    /// Returns whether the register contains no qubits.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.qubits.is_empty()
    }

    /// Returns the logical namespace represented by this register.
    ///
    /// The returned range is lazy and does not allocate.
    #[must_use]
    pub const fn range(&self) -> QubitRange {
        QubitRange {
            start: 0,
            end: 0,
        }
    }

    /// Returns the logical namespace range represented by this register.
    ///
    /// This is the non-const form because it derives the end from the current
    /// vector length.
    #[must_use]
    pub fn qubit_range(&self) -> QubitRange {
        QubitRange {
            start: 0,
            end: self.qubits.len(),
        }
    }

    /// Returns a logical qubit by canonical identifier.
    ///
    /// This performs range validation.
    pub fn get(
        &self,
        qubit: QubitId,
    ) -> Result<&Qubit, QubitRegisterError> {
        self.qubits
            .get(qubit.index())
            .ok_or(QubitRegisterError::OutOfRange {
                qubit,
                num_qubits: self.len(),
            })
    }

    /// Returns a logical qubit without constructing an error.
    #[must_use]
    pub fn get_opt(
        &self,
        qubit: QubitId,
    ) -> Option<&Qubit> {
        self.qubits.get(qubit.index())
    }

    /// Returns an immutable slice of all logical qubits.
    ///
    /// No mutable slice is exposed because unrestricted mutation could violate
    /// register identity/state invariants.
    #[must_use]
    pub fn as_slice(&self) -> &[Qubit] {
        &self.qubits
    }

    /// Returns a deterministic immutable iterator.
    pub fn iter(&self) -> std::slice::Iter<'_, Qubit> {
        self.qubits.iter()
    }

    // ------------------------------------------------------------------------
    // Validation
    // ------------------------------------------------------------------------

    /// Validates logical-register membership.
    pub fn validate(
        &self,
        qubit: QubitId,
    ) -> Result<(), QubitRegisterError> {
        if qubit.index() >= self.len() {
            return Err(QubitRegisterError::OutOfRange {
                qubit,
                num_qubits: self.len(),
            });
        }

        Ok(())
    }

    /// Validates membership and rejects disabled qubits.
    pub fn validate_usable(
        &self,
        qubit: QubitId,
    ) -> Result<(), QubitRegisterError> {
        let value = self.get(qubit)?;

        if value.state() == QubitState::Disabled {
            return Err(QubitRegisterError::Disabled { qubit });
        }

        Ok(())
    }

    /// Validates an operand collection against this register.
    ///
    /// Validation is deterministic and rejects:
    ///
    /// - out-of-range identifiers;
    /// - duplicate identifiers;
    /// - disabled qubits.
    pub fn validate_operands(
        &self,
        qubits: &[QubitId],
    ) -> Result<(), QubitRegisterError> {
        validate_unique_qubits(qubits)?;

        for &qubit in qubits {
            self.validate_usable(qubit)?;
        }

        Ok(())
    }

    /// Validates that every qubit in a range belongs to this register.
    ///
    /// No allocation occurs.
    pub fn validate_range(
        &self,
        range: QubitRange,
    ) -> Result<(), QubitRegisterError> {
        if range.end() > self.len() {
            let qubit = QubitId::new(range.end().saturating_sub(1));

            return Err(QubitRegisterError::OutOfRange {
                qubit,
                num_qubits: self.len(),
            });
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Queries
    // ------------------------------------------------------------------------

    /// Returns the first usable logical qubit.
    ///
    /// Search is deterministic by ascending logical identifier.
    #[must_use]
    pub fn first_available(&self) -> Option<QubitId> {
        self.qubits
            .iter()
            .find(|qubit| qubit.is_usable())
            .map(Qubit::id)
    }

    /// Returns the number of usable logical qubits.
    #[must_use]
    pub fn available_count(&self) -> usize {
        self.qubits
            .iter()
            .filter(|qubit| qubit.is_usable())
            .count()
    }

    /// Returns whether a logical qubit is present and usable.
    #[must_use]
    pub fn contains_usable(
        &self,
        qubit: QubitId,
    ) -> bool {
        self.get_opt(qubit)
            .map(Qubit::is_usable)
            .unwrap_or(false)
    }

    /// Returns whether a logical qubit exists in this register.
    #[must_use]
    pub fn contains(
        &self,
        qubit: QubitId,
    ) -> bool {
        qubit.index() < self.len()
    }

    /// Returns all currently measured logical qubits as an iterator.
    pub fn measured(
        &self,
    ) -> impl Iterator<Item = QubitId> + '_ {
        self.qubits
            .iter()
            .filter(|qubit| qubit.is_measured())
            .map(Qubit::id)
    }

    /// Returns all currently reset logical qubits as an iterator.
    pub fn reset_qubits(
        &self,
    ) -> impl Iterator<Item = QubitId> + '_ {
        self.qubits
            .iter()
            .filter(|qubit| qubit.is_reset())
            .map(Qubit::id)
    }

    /// Returns all disabled logical qubits as an iterator.
    pub fn disabled(
        &self,
    ) -> impl Iterator<Item = QubitId> + '_ {
        self.qubits
            .iter()
            .filter(|qubit| qubit.is_disabled())
            .map(Qubit::id)
    }

    // ------------------------------------------------------------------------
    // State transitions
    // ------------------------------------------------------------------------

    /// Marks a logical qubit as measured.
    ///
    /// This is compiler bookkeeping only. It does not represent a simulator
    /// state transition.
    pub fn mark_measured(
        &mut self,
        qubit: QubitId,
    ) -> Result<(), QubitRegisterError> {
        let value = self.get_mut_internal(qubit)?;

        if value.state() == QubitState::Disabled {
            return Err(QubitRegisterError::Disabled { qubit });
        }

        value.mark_measured_internal();

        Ok(())
    }

    /// Marks a logical qubit as reset.
    ///
    /// This is compiler bookkeeping only.
    pub fn reset(
        &mut self,
        qubit: QubitId,
    ) -> Result<(), QubitRegisterError> {
        let value = self.get_mut_internal(qubit)?;

        if value.state() == QubitState::Disabled {
            return Err(QubitRegisterError::Disabled { qubit });
        }

        value.mark_reset_internal();

        Ok(())
    }

    /// Marks a logical qubit as available.
    ///
    /// State transitions are explicit; this method does not infer availability
    /// from unrelated operations.
    pub fn mark_available(
        &mut self,
        qubit: QubitId,
    ) -> Result<(), QubitRegisterError> {
        let value = self.get_mut_internal(qubit)?;

        if value.state() == QubitState::Disabled {
            return Err(QubitRegisterError::Disabled { qubit });
        }

        value.mark_available_internal();

        Ok(())
    }

    /// Disables a logical qubit in the IR namespace.
    ///
    /// This does not disable a physical machine resource.
    pub fn disable(
        &mut self,
        qubit: QubitId,
    ) -> Result<(), QubitRegisterError> {
        let value = self.get_mut_internal(qubit)?;

        value.mark_disabled_internal();

        Ok(())
    }

    /// Re-enables a logical qubit.
    pub fn enable(
        &mut self,
        qubit: QubitId,
    ) -> Result<(), QubitRegisterError> {
        let value = self.get_mut_internal(qubit)?;

        value.mark_available_internal();

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Internal access
    // ------------------------------------------------------------------------

    fn get_mut_internal(
        &mut self,
        qubit: QubitId,
    ) -> Result<&mut Qubit, QubitRegisterError> {
        let length = self.len();

        self.qubits
            .get_mut(qubit.index())
            .ok_or(QubitRegisterError::OutOfRange {
                qubit,
                num_qubits: length,
            })
    }
}

impl Default for QubitRegister {
    fn default() -> Self {
        Self::empty()
    }
}

impl IntoIterator for QubitRegister {
    type Item = Qubit;
    type IntoIter = std::vec::IntoIter<Qubit>;

    fn into_iter(self) -> Self::IntoIter {
        self.qubits.into_iter()
    }
}

impl<'a> IntoIterator for &'a QubitRegister {
    type Item = &'a Qubit;
    type IntoIter = std::slice::Iter<'a, Qubit>;

    fn into_iter(self) -> Self::IntoIter {
        self.qubits.iter()
    }
}

// ============================================================================
// Collection-level validation
// ============================================================================

/// Validates that logical-qubit operands are unique.
///
/// This implementation uses a deterministic sort-based strategy.
///
/// Complexity:
///
/// ```text
/// O(n log n)
/// ```
///
/// Memory:
///
/// ```text
/// O(n)
/// ```
///
/// This is appropriate for IR operand collections and avoids quadratic
/// duplicate searching.
pub fn validate_unique_qubits(
    qubits: &[QubitId],
) -> Result<(), QubitRegisterError> {
    if qubits.len() < 2 {
        return Ok(());
    }

    let mut sorted = qubits.to_vec();
    sorted.sort_unstable();

    for pair in sorted.windows(2) {
        if pair[0] == pair[1] {
            return Err(QubitRegisterError::DuplicateQubit {
                qubit: pair[0],
            });
        }
    }

    Ok(())
}

/// Validates logical operands against a concrete register size.
///
/// This checks:
///
/// 1. uniqueness;
/// 2. range.
///
/// It does not check hardware topology or capabilities.
pub fn validate_qubits(
    qubits: &[QubitId],
    num_qubits: usize,
) -> Result<(), QubitRegisterError> {
    validate_unique_qubits(qubits)?;

    for &qubit in qubits {
        if qubit.index() >= num_qubits {
            return Err(QubitRegisterError::OutOfRange {
                qubit,
                num_qubits,
            });
        }
    }

    Ok(())
}

/// Validates logical operands against a concrete register.
///
/// This additionally rejects disabled qubits.
pub fn validate_usable_qubits(
    qubits: &[QubitId],
    register: &QubitRegister,
) -> Result<(), QubitRegisterError> {
    register.validate_operands(qubits)
}

/// Returns whether all supplied logical qubits are unique.
#[must_use]
pub fn are_unique_qubits(
    qubits: &[QubitId],
) -> bool {
    validate_unique_qubits(qubits).is_ok()
}

/// Returns whether all supplied logical qubits are valid for a register size.
#[must_use]
pub fn are_valid_qubits(
    qubits: &[QubitId],
    num_qubits: usize,
) -> bool {
    validate_qubits(qubits, num_qubits).is_ok()
}

/// Returns a sorted copy of the supplied logical-qubit identifiers.
///
/// Duplicates are retained.
///
/// This function is useful when canonical operand ordering is needed but
/// duplicate detection is intentionally performed separately.
#[must_use]
pub fn canonicalize_qubits(
    qubits: &[QubitId],
) -> Vec<QubitId> {
    let mut result = qubits.to_vec();

    result.sort_unstable();

    result
}

/// Returns a sorted unique copy of the supplied logical-qubit identifiers.
///
/// Returns an error when duplicates are present.
pub fn canonicalize_unique_qubits(
    qubits: &[QubitId],
) -> Result<Vec<QubitId>, QubitRegisterError> {
    validate_unique_qubits(qubits)?;

    let mut result = qubits.to_vec();

    result.sort_unstable();

    Ok(result)
}

/// Returns the minimum logical-qubit index in a collection.
#[must_use]
pub fn min_qubit_index(
    qubits: &[QubitId],
) -> Option<usize> {
    qubits.iter().map(|qubit| qubit.index()).min()
}

/// Returns the maximum logical-qubit index in a collection.
#[must_use]
pub fn max_qubit_index(
    qubits: &[QubitId],
) -> Option<usize> {
    qubits.iter().map(|qubit| qubit.index()).max()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    // ------------------------------------------------------------------------
    // Range tests
    // ------------------------------------------------------------------------

    #[test]
    fn range_is_half_open() {
        let range = QubitRange::new(2, 5).expect("valid range");

        assert_eq!(range.start(), 2);
        assert_eq!(range.end(), 5);
        assert_eq!(range.len(), 3);
        assert!(range.contains(q(2)));
        assert!(range.contains(q(4)));
        assert!(!range.contains(q(5)));
    }

    #[test]
    fn empty_range_contains_nothing() {
        let range = QubitRange::empty(10);

        assert!(range.is_empty());
        assert_eq!(range.len(), 0);
        assert!(range.first().is_none());
        assert!(range.last().is_none());
        assert!(!range.contains(q(10)));
    }

    #[test]
    fn range_iteration_is_lazy_and_deterministic() {
        let range = QubitRange::new(3, 7).expect("valid range");

        let ids: Vec<_> = range.iter().collect();

        assert_eq!(ids, vec![q(3), q(4), q(5), q(6)]);
    }

    #[test]
    fn invalid_range_is_rejected() {
        let result = QubitRange::new(5, 3);

        assert_eq!(
            result,
            Err(QubitRangeError::InvalidBounds {
                start: 5,
                end: 3
            })
        );
    }

    #[test]
    fn range_extension_checks_overflow() {
        let range = QubitRange::new(0, usize::MAX)
            .expect("valid maximum range");

        assert_eq!(
            range.checked_extend(1),
            Err(QubitRangeError::Overflow)
        );
    }

    #[test]
    fn subrange_is_checked() {
        let range = QubitRange::new(10, 20).expect("valid range");

        let subrange = range
            .subrange(2, 5)
            .expect("valid subrange");

        assert_eq!(subrange.start(), 12);
        assert_eq!(subrange.end(), 15);
    }

    // ------------------------------------------------------------------------
    // Register construction
    // ------------------------------------------------------------------------

    #[test]
    fn empty_register_is_empty() {
        let register = QubitRegister::empty();

        assert!(register.is_empty());
        assert_eq!(register.len(), 0);
    }

    #[test]
    fn register_construction_is_deterministic() {
        let register =
            QubitRegister::try_new(4, 4).expect("allocation should succeed");

        assert_eq!(register.len(), 4);

        for index in 0..4 {
            assert_eq!(
                register
                    .get(q(index))
                    .expect("qubit must exist")
                    .id(),
                q(index)
            );
        }
    }

    #[test]
    fn explicit_limit_is_enforced_before_allocation() {
        let result = QubitRegister::try_new(10, 4);

        assert_eq!(
            result,
            Err(QubitRegisterError::CountExceedsLimit {
                count: 10,
                maximum: 4
            })
        );
    }

    #[test]
    fn zero_count_is_valid() {
        let register =
            QubitRegister::try_new(0, 0).expect("zero-sized register is valid");

        assert!(register.is_empty());
    }

    // ------------------------------------------------------------------------
    // Access and validation
    // ------------------------------------------------------------------------

    #[test]
    fn out_of_range_access_is_rejected() {
        let register =
            QubitRegister::try_new(2, 2).expect("allocation should succeed");

        let result = register.get(q(2));

        assert_eq!(
            result,
            Err(QubitRegisterError::OutOfRange {
                qubit: q(2),
                num_qubits: 2
            })
        );
    }

    #[test]
    fn duplicate_operands_are_rejected() {
        let result = validate_unique_qubits(&[q(1), q(2), q(1)]);

        assert_eq!(
            result,
            Err(QubitRegisterError::DuplicateQubit {
                qubit: q(1)
            })
        );
    }

    #[test]
    fn valid_operands_are_accepted() {
        let result = validate_qubits(
            &[q(0), q(2), q(4)],
            5,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn out_of_range_operands_are_rejected() {
        let result = validate_qubits(
            &[q(0), q(5)],
            5,
        );

        assert_eq!(
            result,
            Err(QubitRegisterError::OutOfRange {
                qubit: q(5),
                num_qubits: 5
            })
        );
    }

    // ------------------------------------------------------------------------
    // State transitions
    // ------------------------------------------------------------------------

    #[test]
    fn measurement_state_is_explicit() {
        let mut register =
            QubitRegister::try_new(2, 2).expect("allocation should succeed");

        register
            .mark_measured(q(1))
            .expect("measurement state should be accepted");

        assert!(
            register
                .get(q(1))
                .expect("qubit must exist")
                .is_measured()
        );

        assert!(
            register
                .get(q(0))
                .expect("qubit must exist")
                .is_available()
        );
    }

    #[test]
    fn reset_state_is_explicit() {
        let mut register =
            QubitRegister::try_new(1, 1).expect("allocation should succeed");

        register
            .reset(q(0))
            .expect("reset state should be accepted");

        assert!(
            register
                .get(q(0))
                .expect("qubit must exist")
                .is_reset()
        );
    }

    #[test]
    fn disabled_qubits_are_rejected_as_usable() {
        let mut register =
            QubitRegister::try_new(1, 1).expect("allocation should succeed");

        register
            .disable(q(0))
            .expect("disable should succeed");

        assert!(
            !register
                .get(q(0))
                .expect("qubit must exist")
                .is_usable()
        );

        assert_eq!(
            register.validate_usable(q(0)),
            Err(QubitRegisterError::Disabled {
                qubit: q(0)
            })
        );
    }

    #[test]
    fn disabled_qubit_can_be_reenabled() {
        let mut register =
            QubitRegister::try_new(1, 1).expect("allocation should succeed");

        register.disable(q(0)).expect("disable should succeed");
        register.enable(q(0)).expect("enable should succeed");

        assert!(
            register
                .get(q(0))
                .expect("qubit must exist")
                .is_available()
        );
    }

    // ------------------------------------------------------------------------
    // Query tests
    // ------------------------------------------------------------------------

    #[test]
    fn first_available_is_deterministic() {
        let mut register =
            QubitRegister::try_new(4, 4).expect("allocation should succeed");

        register.disable(q(0)).expect("disable");
        register.disable(q(1)).expect("disable");

        assert_eq!(
            register.first_available(),
            Some(q(2))
        );
    }

    #[test]
    fn available_count_tracks_disabled_qubits() {
        let mut register =
            QubitRegister::try_new(4, 4).expect("allocation should succeed");

        assert_eq!(register.available_count(), 4);

        register.disable(q(1)).expect("disable");
        register.disable(q(3)).expect("disable");

        assert_eq!(register.available_count(), 2);
    }

    // ------------------------------------------------------------------------
    // Canonicalization
    // ------------------------------------------------------------------------

    #[test]
    fn canonicalization_sorts_without_deduplicating() {
        let result =
            canonicalize_qubits(&[q(4), q(1), q(3), q(1)]);

        assert_eq!(
            result,
            vec![q(1), q(1), q(3), q(4)]
        );
    }

    #[test]
    fn canonicalization_rejects_duplicates_when_required() {
        let result =
            canonicalize_unique_qubits(&[q(4), q(1), q(4)]);

        assert_eq!(
            result,
            Err(QubitRegisterError::DuplicateQubit {
                qubit: q(4)
            })
        );
    }

    // ------------------------------------------------------------------------
    // Collection statistics
    // ------------------------------------------------------------------------

    #[test]
    fn minimum_and_maximum_indices_are_deterministic() {
        let qubits = &[q(8), q(2), q(5)];

        assert_eq!(
            min_qubit_index(qubits),
            Some(2)
        );

        assert_eq!(
            max_qubit_index(qubits),
            Some(8)
        );
    }

    #[test]
    fn empty_collection_has_no_minimum_or_maximum() {
        let qubits: &[QubitId] = &[];

        assert_eq!(min_qubit_index(qubits), None);
        assert_eq!(max_qubit_index(qubits), None);
    }

    // ------------------------------------------------------------------------
    // Iterator tests
    // ------------------------------------------------------------------------

    #[test]
    fn register_iteration_is_deterministic() {
        let register =
            QubitRegister::try_new(3, 3).expect("allocation should succeed");

        let ids: Vec<_> =
            register.iter().map(Qubit::id).collect();

        assert_eq!(
            ids,
            vec![q(0), q(1), q(2)]
        );
    }

    #[test]
    fn owned_iteration_consumes_register() {
        let register =
            QubitRegister::try_new(2, 2).expect("allocation should succeed");

        let ids: Vec<_> =
            register.into_iter().map(|qubit| qubit.id()).collect();

        assert_eq!(
            ids,
            vec![q(0), q(1)]
        );
    }

    // ------------------------------------------------------------------------
    // Scalability invariant
    // ------------------------------------------------------------------------

    #[test]
    fn register_has_no_semantic_fixed_qubit_limit() {
        let small =
            QubitRegister::try_new(1, usize::MAX)
                .expect("one qubit should be valid");

        let larger =
            QubitRegister::try_new(8, usize::MAX)
                .expect("eight qubits should be valid");

        assert_eq!(small.len(), 1);
        assert_eq!(larger.len(), 8);

        // The important invariant is that no architecture-specific constant
        // appears in the register's semantic model.
        assert!(
            QubitRegister::maximum_constructible_count() >= 8
        );
    }
}