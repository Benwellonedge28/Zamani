//! Zamani Quantum IR — Canonical Qubit Model
//!
//! This module owns the canonical logical/physical qubit identity vocabulary
//! used by the Zamani Quantum IR.
//!
//! # Architectural responsibility
//!
//! This module answers:
//!
//! > "Which qubit resource is being referred to semantically?"
//!
//! It owns:
//!
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `QubitRef`;
//! - `QubitState` as IR bookkeeping state;
//! - `Qubit`;
//! - `QubitRange`;
//! - `QubitRegister`;
//! - deterministic qubit collection validation;
//! - qubit-related local errors.
//!
//! It does NOT own:
//!
//! - physical hardware allocation;
//! - device topology;
//! - routing;
//! - placement algorithms;
//! - scheduling;
//! - calibration;
//! - pulse generation;
//! - backend execution;
//! - simulation state;
//! - amplitudes;
//! - density matrices;
//! - quantum probabilities;
//! - error-correction decoding;
//! - optimization policy;
//! - frontend parsing.
//!
//! Those responsibilities belong to downstream IR consumers.
//!
//! # Canonical namespace
//!
//! New code must use:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! when `qubit` is directly exposed by `quantum::ir`, or:
//!
//! ```text
//! quantum::ir::quantum::qubit::QubitId
//! ```
//!
//! when this file is exposed through the new nested `quantum` IR module.
//!
//! `QubitId` is the canonical logical identity.
//!
//! `PhysicalQubitId` is the canonical physical identity vocabulary.
//!
//! These two identity domains are intentionally different types so that a
//! logical qubit cannot accidentally be passed where a physical qubit is
//! expected.
//!
//! # Universal-program principle
//!
//! A Zamani quantum program is written once at the semantic level and may be
//! lowered to compatible targets of different sizes and architectures.
//!
//! Therefore this module MUST NOT contain architectural limits such as:
//!
//! ```text
//! 32 qubits
//! 64 qubits
//! 128 qubits
//! 4096 qubits
//! 1_000_000 qubits
//! ```
//!
//! The number of qubits is program/resource data.
//!
//! Concrete limits belong to:
//!
//! - `QuantumIrLimits`;
//! - compiler policy;
//! - memory availability;
//! - execution runtime;
//! - target hardware capabilities.
//!
//! Those limits must never become semantic limits of `QubitId`.
//!
//! # Important distinction
//!
//! ```text
//! QubitId
//!     logical semantic identity
//!
//! PhysicalQubitId
//!     physical-target identity vocabulary
//!
//! QubitRef
//!     explicitly typed logical/physical reference
//!
//! QubitRegister
//!     concrete in-memory logical namespace
//!
//! QubitRange
//!     lazy logical namespace range
//!
//! routing
//!     determines logical -> physical placement
//!
//! hardware
//!     describes actual physical resources
//!
//! scheduling
//!     determines execution time
//! ```
//!
//! # State semantics
//!
//! `QubitState` is compiler bookkeeping.
//!
//! It is NOT a quantum-mechanical state representation.
//!
//! It does not represent:
//!
//! - amplitudes;
//! - wavefunctions;
//! - density matrices;
//! - probabilities;
//! - entanglement;
//! - decoherence;
//! - measurement probabilities.
//!
//! Simulation state belongs to the simulator subsystem.
//!
//! # Scalability
//!
//! `QubitId` uses `usize` because the current Zamani IR and surrounding
//! repository APIs use index-compatible identifiers extensively.
//!
//! This does NOT mean that Zamani semantically supports only a particular
//! number of qubits.
//!
//! `usize` is the host representation of an identifier. The quantum machine
//! size is not encoded into the type.
//!
//! A concrete `QubitRegister` is an in-memory representation and is therefore
//! naturally limited by host resources. Large compiler pipelines may instead
//! use `QubitRange`, streamed IR, partitioned IR, distributed IR, or sparse
//! representations without changing the semantic identity model.
//!
//! # Safety
//!
//! This module contains no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes this requirement compiler-enforced.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies.

// -----------------------------------------------------------------------------
// Safety contract
// -----------------------------------------------------------------------------

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fmt;
use std::ops::Range;

// ============================================================================
// Logical qubit identity
// ============================================================================

/// Stable logical-qubit identifier.
///
/// A `QubitId` identifies a logical qubit in the canonical Zamani IR.
///
/// It does not identify:
///
/// - a physical hardware qubit;
/// - a simulator array position;
/// - a routing-local node;
/// - a topology vertex;
/// - a backend handle.
///
/// The value is an identifier in the logical namespace.
///
/// `usize` is used for compatibility with existing Zamani IR indexing APIs.
/// It must never be interpreted as a hardware capacity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QubitId(usize);

impl QubitId {
    /// Creates a logical-qubit identifier.
    ///
    /// This does not establish membership in any register.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the underlying logical identifier value.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }

    /// Returns the next representable identifier.
    ///
    /// Returns `None` instead of overflowing.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<usize> for QubitId {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl From<QubitId> for usize {
    fn from(value: QubitId) -> Self {
        value.index()
    }
}

impl fmt::Display for QubitId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "q{}", self.0)
    }
}

// ============================================================================
// Physical qubit identity
// ============================================================================

/// Stable physical-qubit identifier vocabulary.
///
/// This type does not assert that a physical resource actually exists.
///
/// Hardware availability, calibration, topology and capabilities belong to
/// `quantum::hardware`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PhysicalQubitId(usize);

impl PhysicalQubitId {
    /// Creates a physical-qubit identifier.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the underlying physical identifier value.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }

    /// Returns the next representable identifier.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<usize> for PhysicalQubitId {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl From<PhysicalQubitId> for usize {
    fn from(value: PhysicalQubitId) -> Self {
        value.index()
    }
}

impl fmt::Display for PhysicalQubitId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "p{}", self.0)
    }
}

// ============================================================================
// Explicit logical/physical reference
// ============================================================================

/// Explicitly typed logical or physical qubit reference.
///
/// This type exists at compiler integration boundaries where either identity
/// domain is intentionally accepted.
///
/// Keeping the variants typed prevents accidental conversion of a logical
/// qubit into a physical qubit merely because both happen to have an integer
/// index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QubitRef {
    /// Logical program qubit.
    Logical(QubitId),

    /// Physical target qubit.
    Physical(PhysicalQubitId),
}

impl QubitRef {
    /// Returns the logical identifier, if this is a logical reference.
    #[must_use]
    pub const fn logical(self) -> Option<QubitId> {
        match self {
            Self::Logical(id) => Some(id),
            Self::Physical(_) => None,
        }
    }

    /// Returns the physical identifier, if this is a physical reference.
    #[must_use]
    pub const fn physical(self) -> Option<PhysicalQubitId> {
        match self {
            Self::Logical(_) => None,
            Self::Physical(id) => Some(id),
        }
    }

    /// Returns whether this is a logical reference.
    #[must_use]
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::Logical(_))
    }

    /// Returns whether this is a physical reference.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::Physical(_))
    }
}

impl From<QubitId> for QubitRef {
    fn from(value: QubitId) -> Self {
        Self::Logical(value)
    }
}

impl From<PhysicalQubitId> for QubitRef {
    fn from(value: PhysicalQubitId) -> Self {
        Self::Physical(value)
    }
}

impl fmt::Display for QubitRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Logical(id) => write!(formatter, "{id}"),
            Self::Physical(id) => write!(formatter, "{id}"),
        }
    }
}

// ============================================================================
// IR bookkeeping state
// ============================================================================

/// Compiler/IR bookkeeping state of a logical qubit.
///
/// This is NOT a physical quantum state.
///
/// In particular:
///
/// ```text
/// Measured != "the physical qubit no longer exists"
/// Reset    != "the simulator state is |0>"
/// Available != "the hardware is calibrated"
/// ```
///
/// Those meanings belong to the appropriate semantic, simulator and hardware
/// layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QubitState {
    /// Qubit has no special bookkeeping state.
    Available,

    /// A reset semantic has been applied.
    Reset,

    /// A measurement semantic has been applied.
    Measured,

    /// Qubit is unavailable in this IR namespace.
    Disabled,
}

impl Default for QubitState {
    fn default() -> Self {
        Self::Available
    }
}

impl QubitState {
    /// Returns whether the state is `Available`.
    #[must_use]
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }

    /// Returns whether the state is `Reset`.
    #[must_use]
    pub const fn is_reset(self) -> bool {
        matches!(self, Self::Reset)
    }

    /// Returns whether the state is `Measured`.
    #[must_use]
    pub const fn is_measured(self) -> bool {
        matches!(self, Self::Measured)
    }

    /// Returns whether the state is `Disabled`.
    #[must_use]
    pub const fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }

    /// Returns whether ordinary IR operations may reference the qubit.
    ///
    /// Measured and reset qubits remain semantically usable.
    /// Only the explicit `Disabled` bookkeeping state rejects use.
    #[must_use]
    pub const fn is_usable(self) -> bool {
        !self.is_disabled()
    }
}

// ============================================================================
// Logical qubit value
// ============================================================================

/// Canonical logical qubit value.
///
/// This type contains only:
///
/// - logical identity;
/// - compiler bookkeeping state.
///
/// It deliberately contains no:
///
/// - hardware location;
/// - frequency;
/// - pulse;
/// - calibration;
/// - topology;
/// - simulator state.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Qubit {
    id: QubitId,
    state: QubitState,
}

impl Qubit {
    /// Creates a new available logical qubit.
    #[must_use]
    pub const fn new(id: QubitId) -> Self {
        Self {
            id,
            state: QubitState::Available,
        }
    }

    /// Returns the canonical logical identifier.
    #[must_use]
    pub const fn id(&self) -> QubitId {
        self.id
    }

    /// Returns the current IR bookkeeping state.
    #[must_use]
    pub const fn state(&self) -> QubitState {
        self.state
    }

    /// Returns whether the qubit is available.
    #[must_use]
    pub const fn is_available(&self) -> bool {
        self.state.is_available()
    }

    /// Returns whether the qubit is usable.
    #[must_use]
    pub const fn is_usable(&self) -> bool {
        self.state.is_usable()
    }

    /// Returns whether the qubit is disabled.
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.state.is_disabled()
    }

    /// Returns whether the qubit has measurement bookkeeping state.
    #[must_use]
    pub const fn is_measured(&self) -> bool {
        self.state.is_measured()
    }

    /// Returns whether the qubit has reset bookkeeping state.
    #[must_use]
    pub const fn is_reset(&self) -> bool {
        self.state.is_reset()
    }

    fn mark_reset(&mut self) {
        self.state = QubitState::Reset;
    }

    fn mark_measured(&mut self) {
        self.state = QubitState::Measured;
    }

    fn mark_available(&mut self) {
        self.state = QubitState::Available;
    }

    fn mark_disabled(&mut self) {
        self.state = QubitState::Disabled;
    }
}

// ============================================================================
// Lazy qubit range
// ============================================================================

/// Half-open range of logical qubit identifiers.
///
/// ```text
/// QubitRange::new(2, 5)
///
/// q2, q3, q4
/// ```
///
/// Construction does not allocate a `Vec<Qubit>`.
///
/// This is important for large programs where a compiler needs to describe
/// ranges without materializing every identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QubitRange {
    start: usize,
    end: usize,
}

impl QubitRange {
    /// Creates `[start, end)`.
    ///
    /// Returns an error when `start > end`.
    pub const fn new(start: usize, end: usize) -> Result<Self, QubitRangeError> {
        if start > end {
            return Err(QubitRangeError::InvalidBounds { start, end });
        }

        Ok(Self { start, end })
    }

    /// Creates an empty range.
    #[must_use]
    pub const fn empty(index: usize) -> Self {
        Self {
            start: index,
            end: index,
        }
    }

    /// Returns the inclusive start.
    #[must_use]
    pub const fn start(self) -> usize {
        self.start
    }

    /// Returns the exclusive end.
    #[must_use]
    pub const fn end(self) -> usize {
        self.end
    }

    /// Returns the number of identifiers in the range.
    #[must_use]
    pub const fn len(self) -> usize {
        self.end - self.start
    }

    /// Returns whether the range is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// Tests membership.
    #[must_use]
    pub const fn contains(self, qubit: QubitId) -> bool {
        qubit.index() >= self.start && qubit.index() < self.end
    }

    /// Returns a lazy iterator.
    ///
    /// No collection is allocated by this method.
    pub fn iter(self) -> impl Iterator<Item = QubitId> {
        (self.start..self.end).map(QubitId::new)
    }

    /// Returns the equivalent standard-library range.
    #[must_use]
    pub const fn as_range(self) -> Range<usize> {
        self.start..self.end
    }
}

/// Error returned when constructing an invalid qubit range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QubitRangeError {
    /// The start is greater than the exclusive end.
    InvalidBounds {
        /// Range start.
        start: usize,

        /// Range end.
        end: usize,
    },
}

impl fmt::Display for QubitRangeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBounds { start, end } => write!(
                formatter,
                "invalid logical-qubit range: start {start} exceeds end {end}"
            ),
        }
    }
}

impl std::error::Error for QubitRangeError {}

// ============================================================================
// Qubit errors
// ============================================================================

/// Errors produced by canonical qubit operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QubitError {
    /// Requested logical-qubit count exceeds an explicit caller policy.
    CountExceedsLimit {
        /// Requested number of qubits.
        count: usize,

        /// Configured policy maximum.
        maximum: usize,
    },

    /// Requested identifier does not exist in the register.
    OutOfRange {
        /// Requested identifier.
        qubit: QubitId,

        /// Number of qubits in the register.
        num_qubits: usize,
    },

    /// Logical qubit is disabled.
    Disabled {
        /// Disabled qubit.
        qubit: QubitId,
    },

    /// Duplicate logical qubit in an operand list.
    DuplicateQubit {
        /// Duplicated qubit.
        qubit: QubitId,
    },

    /// Requested collection/count cannot be represented by the selected
    /// in-memory representation.
    InvalidCount {
        /// Requested count.
        count: usize,
    },

    /// The register cannot safely reserve the requested capacity.
    AllocationFailed {
        /// Requested number of qubits.
        count: usize,
    },
}

impl fmt::Display for QubitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CountExceedsLimit { count, maximum } => write!(
                formatter,
                "logical qubit count {count} exceeds configured maximum {maximum}"
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
                write!(formatter, "logical qubit {qubit} occurs more than once")
            }

            Self::InvalidCount { count } => {
                write!(
                    formatter,
                    "logical qubit count {count} cannot be represented safely"
                )
            }

            Self::AllocationFailed { count } => {
                write!(
                    formatter,
                    "unable to reserve memory for {count} logical qubits"
                )
            }
        }
    }
}

impl std::error::Error for QubitError {}

// ============================================================================
// Logical qubit register
// ============================================================================

/// Concrete in-memory logical-qubit namespace.
///
/// `QubitRegister` is an implementation structure, not the definition of
/// Zamani's maximum quantum-machine size.
///
/// For very large programs, callers should prefer:
///
/// - `QubitRange`;
/// - sparse structures;
/// - partitioned IR;
/// - streaming IR;
/// - distributed IR.
///
/// The semantic identity remains `QubitId`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QubitRegister {
    qubits: Vec<Qubit>,
}

impl QubitRegister {
    /// Creates an empty register.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            qubits: Vec::new(),
        }
    }

    /// Creates a register using the compatibility constructor.
    ///
    /// New untrusted/compiler-boundary code should use `try_new`.
    ///
    /// This constructor exists because existing Zamani code may construct
    /// registers directly.
    ///
    /// It never uses `unsafe`.
    ///
    /// If allocation itself cannot be satisfied, Rust's standard allocator
    /// may abort the process. Therefore externally controlled counts MUST use
    /// `try_new` with an explicit IR policy before reaching this compatibility
    /// constructor.
    pub fn new(count: usize) -> Self {
        Self::try_new(count, Self::maximum_constructible_count())
            .expect("logical qubit register allocation failed")
    }

    /// Creates a register subject to an explicit count policy.
    ///
    /// The policy is checked before allocation.
    ///
    /// This is the preferred API for:
    ///
    /// - deserialization;
    /// - frontend input;
    /// - network input;
    /// - user-controlled compilation;
    /// - service environments.
    pub fn try_new(count: usize, maximum: usize) -> Result<Self, QubitError> {
        if count > maximum {
            return Err(QubitError::CountExceedsLimit { count, maximum });
        }

        if count > Self::maximum_constructible_count() {
            return Err(QubitError::InvalidCount { count });
        }

        let mut qubits = Vec::new();

        qubits
            .try_reserve_exact(count)
            .map_err(|_| QubitError::AllocationFailed { count })?;

        for index in 0..count {
            qubits.push(Qubit::new(QubitId::new(index)));
        }

        Ok(Self { qubits })
    }

    /// Returns a conservative `Vec<Qubit>` construction bound.
    ///
    /// This is a host representation bound only.
    ///
    /// It is NOT a quantum-machine limit.
    #[must_use]
    pub const fn maximum_constructible_count() -> usize {
        isize::MAX as usize / std::mem::size_of::<Qubit>()
    }

    /// Returns the number of logical qubits.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.qubits.len()
    }

    /// Returns whether the register contains no logical qubits.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.qubits.is_empty()
    }

    /// Returns the logical qubit for an identifier.
    pub fn get(&self, id: QubitId) -> Result<&Qubit, QubitError> {
        self.qubits
            .get(id.index())
            .ok_or(QubitError::OutOfRange {
                qubit: id,
                num_qubits: self.len(),
            })
    }

    /// Returns an optional logical qubit.
    #[must_use]
    pub fn get_opt(&self, id: QubitId) -> Option<&Qubit> {
        self.qubits.get(id.index())
    }

    /// Returns all materialized logical qubits.
    ///
    /// This is an immutable view.
    #[must_use]
    pub fn as_slice(&self) -> &[Qubit] {
        &self.qubits
    }

    /// Returns a deterministic iterator.
    pub fn iter(&self) -> std::slice::Iter<'_, Qubit> {
        self.qubits.iter()
    }

    /// Returns the first usable logical qubit.
    ///
    /// Selection is deterministic and follows ascending logical identifier
    /// order.
    #[must_use]
    pub fn first_available(&self) -> Option<QubitId> {
        self.qubits
            .iter()
            .find(|qubit| qubit.is_usable())
            .map(Qubit::id)
    }

    /// Validates logical membership.
    pub fn validate(&self, id: QubitId) -> Result<(), QubitError> {
        if id.index() >= self.len() {
            return Err(QubitError::OutOfRange {
                qubit: id,
                num_qubits: self.len(),
            });
        }

        Ok(())
    }

    /// Validates membership and usability.
    pub fn validate_usable(&self, id: QubitId) -> Result<(), QubitError> {
        let qubit = self.get(id)?;

        if qubit.is_disabled() {
            return Err(QubitError::Disabled { qubit: id });
        }

        Ok(())
    }

    /// Marks a qubit as measured in IR bookkeeping.
    ///
    /// This does not mean the qubit is permanently unusable.
    pub fn mark_measured(&mut self, id: QubitId) -> Result<(), QubitError> {
        let qubit = self.get_mut(id)?;

        if qubit.is_disabled() {
            return Err(QubitError::Disabled { qubit: id });
        }

        qubit.mark_measured();

        Ok(())
    }

    /// Marks a qubit as reset in IR bookkeeping.
    pub fn reset(&mut self, id: QubitId) -> Result<(), QubitError> {
        let qubit = self.get_mut(id)?;

        if qubit.is_disabled() {
            return Err(QubitError::Disabled { qubit: id });
        }

        qubit.mark_reset();

        Ok(())
    }

    /// Returns a qubit to the normal available bookkeeping state.
    pub fn mark_available(&mut self, id: QubitId) -> Result<(), QubitError> {
        let qubit = self.get_mut(id)?;

        if qubit.is_disabled() {
            return Err(QubitError::Disabled { qubit: id });
        }

        qubit.mark_available();

        Ok(())
    }

    /// Disables a logical qubit in this namespace.
    ///
    /// This is not physical hardware disablement.
    pub fn disable(&mut self, id: QubitId) -> Result<(), QubitError> {
        let qubit = self.get_mut(id)?;
        qubit.mark_disabled();
        Ok(())
    }

    /// Enables a previously disabled logical qubit.
    pub fn enable(&mut self, id: QubitId) -> Result<(), QubitError> {
        let qubit = self.get_mut(id)?;
        qubit.mark_available();
        Ok(())
    }

    /// Validates a collection of logical operands.
    ///
    /// Checks:
    ///
    /// 1. duplicates;
    /// 2. range;
    /// 3. disabled state.
    pub fn validate_operands(&self, qubits: &[QubitId]) -> Result<(), QubitError> {
        validate_qubits(qubits, self.len())?;

        for &qubit in qubits {
            self.validate_usable(qubit)?;
        }

        Ok(())
    }

    fn get_mut(&mut self, id: QubitId) -> Result<&mut Qubit, QubitError> {
        let length = self.len();

        self.qubits
            .get_mut(id.index())
            .ok_or(QubitError::OutOfRange {
                qubit: id,
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
// Collection validation
// ============================================================================

/// Validates that every qubit in a collection is unique.
///
/// Complexity is `O(n log n)` and deterministic.
pub fn validate_unique_qubits(qubits: &[QubitId]) -> Result<(), QubitError> {
    let mut seen = BTreeSet::new();

    for &qubit in qubits {
        if !seen.insert(qubit) {
            return Err(QubitError::DuplicateQubit { qubit });
        }
    }

    Ok(())
}

/// Validates uniqueness and register membership.
///
/// This function does not require a `QubitRegister`, making it suitable for
/// operation/gate construction where only the logical namespace size is known.
pub fn validate_qubits(
    qubits: &[QubitId],
    num_qubits: usize,
) -> Result<(), QubitError> {
    validate_unique_qubits(qubits)?;

    for &qubit in qubits {
        if qubit.index() >= num_qubits {
            return Err(QubitError::OutOfRange {
                qubit,
                num_qubits,
            });
        }
    }

    Ok(())
}

/// Validates uniqueness, membership and usability.
pub fn validate_usable_qubits(
    qubits: &[QubitId],
    register: &QubitRegister,
) -> Result<(), QubitError> {
    register.validate_operands(qubits)
}

/// Returns whether all supplied logical qubits are unique.
#[must_use]
pub fn are_unique_qubits(qubits: &[QubitId]) -> bool {
    validate_unique_qubits(qubits).is_ok()
}

/// Returns whether all supplied logical qubits are unique and within range.
#[must_use]
pub fn are_valid_qubits(
    qubits: &[QubitId],
    num_qubits: usize,
) -> bool {
    validate_qubits(qubits, num_qubits).is_ok()
}

/// Returns a sorted, deduplicated copy of logical qubit IDs.
///
/// This function intentionally does NOT report duplicates. It is a
/// canonicalization utility where duplicate elimination is desired.
#[must_use]
pub fn canonicalize_qubits(qubits: &[QubitId]) -> Vec<QubitId> {
    let mut result = qubits.to_vec();

    result.sort_unstable();
    result.dedup();

    result
}

/// Returns a sorted copy while rejecting duplicate logical qubits.
pub fn canonicalize_unique_qubits(
    qubits: &[QubitId],
) -> Result<Vec<QubitId>, QubitError> {
    validate_unique_qubits(qubits)?;

    let mut result = qubits.to_vec();
    result.sort_unstable();

    Ok(result)
}

/// Returns the minimum logical-qubit index.
#[must_use]
pub fn min_qubit_index(qubits: &[QubitId]) -> Option<usize> {
    qubits.iter().map(|qubit| qubit.index()).min()
}

/// Returns the maximum logical-qubit index.
#[must_use]
pub fn max_qubit_index(qubits: &[QubitId]) -> Option<usize> {
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

    fn p(index: usize) -> PhysicalQubitId {
        PhysicalQubitId::new(index)
    }

    #[test]
    fn logical_id_round_trips() {
        let id = q(42);

        assert_eq!(id.index(), 42);
        assert_eq!(usize::from(id), 42);
        assert_eq!(id.to_string(), "q42");
    }

    #[test]
    fn physical_id_round_trips() {
        let id = p(42);

        assert_eq!(id.index(), 42);
        assert_eq!(usize::from(id), 42);
        assert_eq!(id.to_string(), "p42");
    }

    #[test]
    fn logical_and_physical_ids_are_not_interchangeable() {
        let logical = QubitRef::Logical(q(5));
        let physical = QubitRef::Physical(p(5));

        assert!(logical.is_logical());
        assert!(!logical.is_physical());

        assert!(physical.is_physical());
        assert!(!physical.is_logical());

        assert_ne!(logical, physical);
    }

    #[test]
    fn checked_next_prevents_overflow() {
        assert_eq!(
            QubitId::new(usize::MAX).checked_next(),
            None
        );

        assert_eq!(
            PhysicalQubitId::new(usize::MAX).checked_next(),
            None
        );
    }

    #[test]
    fn qubit_starts_available() {
        let qubit = Qubit::new(q(3));

        assert_eq!(qubit.id(), q(3));
        assert_eq!(qubit.state(), QubitState::Available);
        assert!(qubit.is_available());
        assert!(qubit.is_usable());
        assert!(!qubit.is_measured());
        assert!(!qubit.is_reset());
        assert!(!qubit.is_disabled());
    }

    #[test]
    fn qubit_range_is_lazy() {
        let range = QubitRange::new(2, 5).expect("valid range");

        assert_eq!(range.start(), 2);
        assert_eq!(range.end(), 5);
        assert_eq!(range.len(), 3);

        assert!(range.contains(q(2)));
        assert!(range.contains(q(4)));
        assert!(!range.contains(q(5)));

        let values: Vec<_> = range.iter().collect();

        assert_eq!(values, vec![q(2), q(3), q(4)]);
    }

    #[test]
    fn empty_range_is_empty() {
        let range = QubitRange::empty(7);

        assert!(range.is_empty());
        assert_eq!(range.len(), 0);
        assert!(!range.contains(q(7)));
    }

    #[test]
    fn invalid_range_is_rejected() {
        let result = QubitRange::new(5, 2);

        assert_eq!(
            result,
            Err(QubitRangeError::InvalidBounds {
                start: 5,
                end: 2
            })
        );
    }

    #[test]
    fn empty_register_is_valid() {
        let register = QubitRegister::empty();

        assert!(register.is_empty());
        assert_eq!(register.len(), 0);
        assert_eq!(register.first_available(), None);
    }

    #[test]
    fn register_contains_deterministic_ids() {
        let register =
            QubitRegister::try_new(4, 4).expect("allocation should succeed");

        assert_eq!(register.len(), 4);

        let ids: Vec<_> = register.iter().map(Qubit::id).collect();

        assert_eq!(ids, vec![q(0), q(1), q(2), q(3)]);
    }

    #[test]
    fn register_get_validates_range() {
        let register =
            QubitRegister::try_new(3, 3).expect("allocation should succeed");

        assert!(register.get(q(0)).is_ok());
        assert!(register.get(q(2)).is_ok());

        assert_eq!(
            register.get(q(3)),
            Err(QubitError::OutOfRange {
                qubit: q(3),
                num_qubits: 3
            })
        );
    }

    #[test]
    fn register_get_opt_is_non_panicking() {
        let register =
            QubitRegister::try_new(2, 2).expect("allocation should succeed");

        assert!(register.get_opt(q(0)).is_some());
        assert!(register.get_opt(q(2)).is_none());
    }

    #[test]
    fn register_state_transitions_are_explicit() {
        let mut register =
            QubitRegister::try_new(1, 1).expect("allocation should succeed");

        let id = q(0);

        register.mark_measured(id).expect("measurement should succeed");
        assert!(register.get(id).expect("qubit exists").is_measured());

        register.reset(id).expect("reset should succeed");
        assert!(register.get(id).expect("qubit exists").is_reset());

        register
            .mark_available(id)
            .expect("availability transition should succeed");

        assert!(register.get(id).expect("qubit exists").is_available());
    }

    #[test]
    fn disabled_qubit_is_not_usable() {
        let mut register =
            QubitRegister::try_new(1, 1).expect("allocation should succeed");

        let id = q(0);

        register.disable(id).expect("disable should succeed");

        assert!(register.get(id).expect("qubit exists").is_disabled());
        assert!(!register.get(id).expect("qubit exists").is_usable());

        assert_eq!(
            register.validate_usable(id),
            Err(QubitError::Disabled { qubit: id })
        );
    }

    #[test]
    fn disabled_qubit_can_be_reenabled() {
        let mut register =
            QubitRegister::try_new(1, 1).expect("allocation should succeed");

        let id = q(0);

        register.disable(id).expect("disable should succeed");
        register.enable(id).expect("enable should succeed");

        assert!(register.get(id).expect("qubit exists").is_available());
        assert!(register.get(id).expect("qubit exists").is_usable());
    }

    #[test]
    fn duplicate_qubits_are_rejected() {
        let result = validate_unique_qubits(&[q(0), q(1), q(0)]);

        assert_eq!(
            result,
            Err(QubitError::DuplicateQubit { qubit: q(0) })
        );
    }

    #[test]
    fn unique_qubits_are_accepted() {
        let result = validate_unique_qubits(&[q(0), q(1), q(2)]);

        assert!(result.is_ok());
    }

    #[test]
    fn range_validation_rejects_out_of_range_qubits() {
        let result = validate_qubits(&[q(0), q(3)], 3);

        assert_eq!(
            result,
            Err(QubitError::OutOfRange {
                qubit: q(3),
                num_qubits: 3
            })
        );
    }

    #[test]
    fn range_validation_accepts_valid_qubits() {
        let result = validate_qubits(&[q(0), q(2)], 3);

        assert!(result.is_ok());
    }

    #[test]
    fn canonicalization_is_deterministic() {
        let input = [q(4), q(1), q(3), q(1), q(2)];

        let result = canonicalize_qubits(&input);

        assert_eq!(
            result,
            vec![q(1), q(2), q(3), q(4)]
        );
    }

    #[test]
    fn unique_canonicalization_rejects_duplicates() {
        let input = [q(4), q(1), q(1)];

        assert_eq!(
            canonicalize_unique_qubits(&input),
            Err(QubitError::DuplicateQubit { qubit: q(1) })
        );
    }

    #[test]
    fn unique_canonicalization_sorts_without_duplicates() {
        let input = [q(4), q(1), q(3), q(2)];

        assert_eq!(
            canonicalize_unique_qubits(&input).expect("valid operands"),
            vec![q(1), q(2), q(3), q(4)]
        );
    }

    #[test]
    fn register_operand_validation_checks_all_invariants() {
        let mut register =
            QubitRegister::try_new(4, 4).expect("allocation should succeed");

        assert!(
            register
                .validate_operands(&[q(0), q(1), q(3)])
                .is_ok()
        );

        assert_eq!(
            register.validate_operands(&[q(0), q(0)]),
            Err(QubitError::DuplicateQubit { qubit: q(0) })
        );

        assert_eq!(
            register.validate_operands(&[q(0), q(4)]),
            Err(QubitError::OutOfRange {
                qubit: q(4),
                num_qubits: 4
            })
        );

        register.disable(q(2)).expect("disable should succeed");

        assert_eq!(
            register.validate_operands(&[q(2)]),
            Err(QubitError::Disabled { qubit: q(2) })
        );
    }

    #[test]
    fn first_available_is_deterministic() {
        let mut register =
            QubitRegister::try_new(4, 4).expect("allocation should succeed");

        assert_eq!(register.first_available(), Some(q(0)));

        register.disable(q(0)).expect("disable should succeed");

        assert_eq!(register.first_available(), Some(q(1)));

        register.disable(q(1)).expect("disable should succeed");

        assert_eq!(register.first_available(), Some(q(2)));
    }

    #[test]
    fn explicit_count_policy_is_checked_before_allocation() {
        let result = QubitRegister::try_new(100, 10);

        assert_eq!(
            result,
            Err(QubitError::CountExceedsLimit {
                count: 100,
                maximum: 10
            })
        );
    }

    #[test]
    fn large_identifier_does_not_imply_large_allocation() {
        let id = QubitId::new(usize::MAX - 1);

        assert_eq!(id.index(), usize::MAX - 1);

        let range = QubitRange::new(usize::MAX - 2, usize::MAX)
            .expect("valid range");

        assert_eq!(range.len(), 2);
        assert!(range.contains(id));
    }

    #[test]
    fn register_into_iterator_is_deterministic() {
        let register =
            QubitRegister::try_new(3, 3).expect("allocation should succeed");

        let ids: Vec<_> = register.into_iter().map(|q| q.id()).collect();

        assert_eq!(ids, vec![q(0), q(1), q(2)]);
    }
}