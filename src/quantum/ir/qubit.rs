//! Zamani Quantum IR — Qubit Model
//!
//! Canonical, hardware-independent representation of logical and physical
//! qubit identities and logical-qubit collections.
//!
//! # Architectural role
//!
//! `quantum::ir::qubit` owns the canonical qubit namespace of the Zamani
//! Quantum IR.
//!
//! It defines:
//!
//! - logical qubit identity;
//! - physical qubit identity;
//! - logical qubit state markers;
//! - logical qubit values;
//! - logical qubit registers;
//! - logical qubit ranges;
//! - deterministic logical-qubit collections;
//! - logical-qubit operand validation;
//! - local qubit-related errors.
//!
//! It does NOT own:
//!
//! - physical hardware allocation;
//! - hardware topology;
//! - routing algorithms;
//! - logical-to-physical placement algorithms;
//! - scheduling;
//! - calibration;
//! - pulse generation;
//! - pulse compilation;
//! - backend execution;
//! - QPU communication;
//! - simulation state;
//! - quantum amplitudes or density matrices;
//! - error-correction decoding;
//! - optimization policy;
//! - frontend parsing.
//!
//! Those responsibilities belong to the corresponding downstream
//! subsystems.
//!
//! # Canonical identity boundary
//!
//! The canonical identities are:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! `identity.rs` intentionally does not define these types.
//!
//! Routing has its own routing-local logical and physical identifiers.
//! Conversion between routing identifiers and canonical IR identifiers belongs
//! to the routing/IR integration boundary, not to this module.
//!
//! # Universal quantum-program principle
//!
//! A Zamani quantum program is written independently of the target machine.
//!
//! Consequently, this module does NOT define a fixed architectural maximum
//! such as:
//!
//! ```text
//! 63
//! 64
//! 4096
//! 1_000_000
//! ```
//!
//! as a quantum-machine limit.
//!
//! A program containing one logical qubit and a program containing an
//! arbitrarily large finite number of logical qubits use the same semantic
//! model.
//!
//! Concrete limits belong to an explicit policy such as `QuantumIrLimits`.
//! Physical capacity belongs to `quantum::hardware`.
//!
//! # Important distinction
//!
//! ```text
//! QubitId
//!     = logical identity
//!
//! PhysicalQubitId
//!     = physical identity vocabulary
//!
//! QubitRegister
//!     = logical namespace/container
//!
//! Routing
//!     = decides logical -> physical placement
//!
//! Hardware
//!     = describes actual physical resources
//!
//! Scheduling
//!     = decides temporal execution
//! ```
//!
//! # State semantics
//!
//! `QubitState` is compiler/IR bookkeeping.
//!
//! It is NOT a representation of the quantum state vector, density matrix,
//! wavefunction, amplitudes, probabilities, or physical decoherence state.
//!
//! In particular, this module must never be used as a simulator.
//!
//! # Allocation safety
//!
//! The preferred constructor for externally supplied counts is:
//!
//! ```text
//! QubitRegister::try_new(count, maximum)
//! ```
//!
//! The explicit maximum is a policy boundary and is checked before allocation.
//!
//! `QubitRegister::new` is retained as a compatibility convenience for
//! trusted/internal callers.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features.
//! No external dependencies.
//! No `unsafe` code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fmt;
use std::ops::Range;

// =============================================================================
// Logical qubit identifier
// =============================================================================

/// Stable logical-qubit identifier.
///
/// A `QubitId` identifies a logical qubit in the canonical Quantum IR.
///
/// It does not identify:
///
/// - a physical hardware qubit;
/// - a simulator state-vector position;
/// - a routing-local identifier;
/// - a hardware topology node.
///
/// The numeric value is a logical namespace identifier.
///
/// `usize` is intentionally used because the identifier is frequently used
/// for indexing a local in-memory collection. It does not impose a fixed
/// quantum-machine size limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QubitId(usize);

impl QubitId {
    /// Creates a logical-qubit identifier.
    ///
    /// This constructor does not establish register membership.
    ///
    /// Membership must be checked against the owning `QubitRegister`.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the underlying logical index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }

    /// Returns the next identifier when it exists.
    ///
    /// This is useful when constructing namespaces without allowing integer
    /// overflow.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<usize> for QubitId {
    fn from(index: usize) -> Self {
        Self::new(index)
    }
}

impl From<QubitId> for usize {
    fn from(qubit: QubitId) -> usize {
        qubit.index()
    }
}

impl fmt::Display for QubitId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "q{}", self.0)
    }
}

// =============================================================================
// Physical qubit identifier
// =============================================================================

/// Physical hardware-qubit identifier.
///
/// `PhysicalQubitId` exists in the canonical IR vocabulary so later
/// compilation stages can represent a logical-to-physical mapping without
/// confusing the two identity domains.
///
/// Constructing this identifier does NOT establish that:
///
/// - the physical qubit exists;
/// - the physical qubit is available;
/// - the physical qubit is calibrated;
/// - the physical qubit is healthy;
/// - the physical qubit supports an operation;
/// - the physical qubit is connected to another qubit.
///
/// Those properties belong to the hardware layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PhysicalQubitId(usize);

impl PhysicalQubitId {
    /// Creates a physical-qubit identifier.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the underlying physical index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }

    /// Returns the next identifier when it exists.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<usize> for PhysicalQubitId {
    fn from(index: usize) -> Self {
        Self::new(index)
    }
}

impl From<PhysicalQubitId> for usize {
    fn from(qubit: PhysicalQubitId) -> usize {
        qubit.index()
    }
}

impl fmt::Display for PhysicalQubitId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "p{}", self.0)
    }
}

// =============================================================================
// Logical / physical namespace reference
// =============================================================================

/// Explicitly identifies which qubit identity namespace is being referenced.
///
/// This type is useful at compiler integration boundaries where accepting
/// either a logical or physical qubit is intentional.
///
/// It prevents a caller from having to encode the distinction using a raw
/// integer or an untyped tuple.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QubitRef {
    /// Logical program qubit.
    Logical(QubitId),

    /// Physical target qubit.
    Physical(PhysicalQubitId),
}

impl QubitRef {
    /// Returns the logical identifier when this is a logical reference.
    #[must_use]
    pub const fn logical(self) -> Option<QubitId> {
        match self {
            Self::Logical(id) => Some(id),
            Self::Physical(_) => None,
        }
    }

    /// Returns the physical identifier when this is a physical reference.
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
    fn from(id: QubitId) -> Self {
        Self::Logical(id)
    }
}

impl From<PhysicalQubitId> for QubitRef {
    fn from(id: PhysicalQubitId) -> Self {
        Self::Physical(id)
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

// =============================================================================
// Logical qubit state
// =============================================================================

/// Compiler/IR bookkeeping state for a logical qubit.
///
/// This is NOT a quantum-mechanical state.
///
/// It does not represent:
///
/// - |0>;
/// - |1>;
/// - superposition;
/// - amplitudes;
/// - density matrices;
/// - probabilities;
/// - entanglement;
/// - decoherence.
///
/// Simulation state belongs to the simulator subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QubitState {
    /// Logical qubit is available for normal use.
    Available,

    /// A reset operation has established reset semantics.
    Reset,

    /// A measurement has been applied.
    ///
    /// This is bookkeeping only. It does not mean the qubit is physically
    /// destroyed or permanently unusable.
    Measured,

    /// The logical qubit is disabled/reserved at the IR namespace level.
    Disabled,
}

impl Default for QubitState {
    fn default() -> Self {
        Self::Available
    }
}

impl QubitState {
    /// Returns whether the qubit is available.
    #[must_use]
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }

    /// Returns whether the qubit is marked as reset.
    #[must_use]
    pub const fn is_reset(self) -> bool {
        matches!(self, Self::Reset)
    }

    /// Returns whether the qubit is marked as measured.
    #[must_use]
    pub const fn is_measured(self) -> bool {
        matches!(self, Self::Measured)
    }

    /// Returns whether the qubit is disabled.
    #[must_use]
    pub const fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }

    /// Returns whether the state permits ordinary IR use.
    ///
    /// `Measured` and `Reset` remain usable.
    /// Only `Disabled` is unavailable.
    #[must_use]
    pub const fn is_usable(self) -> bool {
        !self.is_disabled()
    }
}

// =============================================================================
// Logical qubit
// =============================================================================

/// Canonical logical qubit value.
///
/// A `Qubit` contains only logical identity and IR bookkeeping state.
///
/// It deliberately contains no physical topology, calibration, pulse,
/// frequency, control channel, or hardware information.
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

    /// Returns the logical identifier.
    #[must_use]
    pub const fn id(&self) -> QubitId {
        self.id
    }

    /// Returns the IR bookkeeping state.
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
    ///
    /// Only `Disabled` makes a qubit unusable through this namespace API.
    #[must_use]
    pub const fn is_usable(&self) -> bool {
        self.state.is_usable()
    }

    /// Returns whether the qubit is disabled.
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.state.is_disabled()
    }

    /// Returns whether the qubit is measured.
    #[must_use]
    pub const fn is_measured(&self) -> bool {
        self.state.is_measured()
    }

    /// Returns whether the qubit is reset.
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

// =============================================================================
// Qubit range
// =============================================================================

/// Half-open logical-qubit range.
///
/// `QubitRange::new(2, 5)` represents:
///
/// ```text
/// q2, q3, q4
/// ```
///
/// It does not allocate qubits.
///
/// This makes it suitable for large symbolic/resource declarations where
/// materializing every identifier is unnecessary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QubitRange {
    start: usize,
    end: usize,
}

impl QubitRange {
    /// Creates a half-open range `[start, end)`.
    ///
    /// Returns an error if `start > end`.
    pub const fn new(start: usize, end: usize) -> Result<Self, QubitRangeError> {
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

    /// Returns the first index.
    #[must_use]
    pub const fn start(self) -> usize {
        self.start
    }

    /// Returns the exclusive end index.
    #[must_use]
    pub const fn end(self) -> usize {
        self.end
    }

    /// Returns the number of identifiers represented by the range.
    ///
    /// Because the range is half-open, this calculation cannot underflow
    /// after construction has succeeded.
    #[must_use]
    pub const fn len(self) -> usize {
        self.end - self.start
    }

    /// Returns whether the range contains no identifiers.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// Returns whether a logical identifier belongs to the range.
    #[must_use]
    pub const fn contains(self, id: QubitId) -> bool {
        id.index() >= self.start && id.index() < self.end
    }

    /// Returns a lazy logical-qubit iterator.
    ///
    /// No collection is allocated.
    pub fn iter(self) -> impl Iterator<Item = QubitId> {
        (self.start..self.end).map(QubitId::new)
    }

    /// Returns the equivalent standard-library index range.
    #[must_use]
    pub const fn as_range(self) -> Range<usize> {
        self.start..self.end
    }
}

/// Errors produced while constructing a logical-qubit range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QubitRangeError {
    /// Range start is greater than range end.
    InvalidBounds {
        /// Inclusive start.
        start: usize,

        /// Exclusive end.
        end: usize,
    },
}

impl fmt::Display for QubitRangeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBounds { start, end } => write!(
                formatter,
                "invalid qubit range: start {start} is greater than end {end}"
            ),
        }
    }
}

impl std::error::Error for QubitRangeError {}

// =============================================================================
// Qubit errors
// =============================================================================

/// Errors produced by canonical logical-qubit operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QubitError {
    /// Requested register size exceeds an explicitly supplied policy.
    CountExceedsLimit {
        /// Requested number of logical qubits.
        count: usize,

        /// Maximum permitted by the caller's policy.
        maximum: usize,
    },

    /// Identifier does not belong to a register.
    OutOfRange {
        /// Requested identifier.
        qubit: QubitId,

        /// Current register size.
        num_qubits: usize,
    },

    /// Identifier refers to a disabled logical qubit.
    Disabled {
        /// Disabled logical qubit.
        qubit: QubitId,
    },

    /// The same logical qubit appears more than once.
    DuplicateQubit {
        /// Duplicated logical qubit.
        qubit: QubitId,
    },

    /// A supplied logical-qubit collection contains an invalid identifier.
    InvalidQubit {
        /// Invalid identifier.
        qubit: QubitId,
    },

    /// No currently available logical qubit exists.
    NoAvailableQubit,

    /// The requested count cannot be represented by a single in-memory
    /// `Vec<Qubit>` on this target.
    InvalidCount {
        /// Requested number of logical qubits.
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
                write!(formatter, "logical qubit {qubit} appears more than once")
            }

            Self::InvalidQubit { qubit } => {
                write!(formatter, "invalid logical qubit {qubit}")
            }

            Self::NoAvailableQubit => {
                write!(formatter, "no available logical qubit exists")
            }

            Self::InvalidCount { count } => {
                write!(formatter, "logical qubit count {count} cannot be represented safely")
            }
        }
    }
}

impl std::error::Error for QubitError {}

// =============================================================================
// Logical qubit register
// =============================================================================

/// Deterministic logical-qubit namespace.
///
/// A `QubitRegister` owns logical identifiers for a concrete in-memory IR
/// object.
///
/// It does NOT allocate physical hardware resources.
///
/// # Scalability
///
/// The semantic model has no fixed quantum-machine-size ceiling.
///
/// The concrete `Vec<Qubit>` representation is intentionally bounded by:
///
/// 1. the caller's explicit policy; and
/// 2. the host's actual memory/address space.
///
/// Very large or distributed programs can use ranges, streamed IR,
/// partitioned program structures, or other higher-level representations
/// without changing the meaning of `QubitId`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QubitRegister {
    qubits: Vec<Qubit>,
}

impl QubitRegister {
    /// Creates an empty logical-qubit register.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            qubits: Vec::new(),
        }
    }

    /// Creates a logical register with `count` qubits.
    ///
    /// This constructor is intended for trusted/internal callers.
    ///
    /// For external or untrusted input, prefer:
    ///
    /// ```text
    /// QubitRegister::try_new(count, maximum)
    /// ```
    ///
    /// because it permits a caller-defined allocation policy.
    ///
    /// The only rejection performed here is the platform-level vector-size
    /// safety check. An allocation can still fail due to the operating
    /// system's available memory; Rust does not provide a safe standard
    /// library API that converts allocator OOM into an ordinary `Result`.
    ///
    /// Therefore externally controlled counts must always be constrained by
    /// an explicit IR policy before reaching this constructor.
    pub fn new(count: usize) -> Self {
        assert!(
            count <= Self::maximum_constructible_count(),
            "logical qubit count exceeds the safe Vec construction bound"
        );

        let mut qubits = Vec::with_capacity(count);

        for index in 0..count {
            qubits.push(Qubit::new(QubitId::new(index)));
        }

        Self { qubits }
    }

    /// Creates a logical register under an explicit count policy.
    ///
    /// The policy is checked before any vector allocation.
    ///
    /// This is the preferred API at compiler/deserialization/input
    /// boundaries.
    pub fn try_new(
        count: usize,
        maximum: usize,
    ) -> Result<Self, QubitError> {
        if count > maximum {
            return Err(QubitError::CountExceedsLimit {
                count,
                maximum,
            });
        }

        if count > Self::maximum_constructible_count() {
            return Err(QubitError::InvalidCount { count });
        }

        let mut qubits = Vec::with_capacity(count);

        for index in 0..count {
            qubits.push(Qubit::new(QubitId::new(index)));
        }

        Ok(Self { qubits })
    }

    /// Returns a conservative upper bound for a single `Vec<Qubit>`.
    ///
    /// This is a representational bound, not a quantum-machine limit.
    ///
    /// The actual usable size is always smaller or equal to the available
    /// process memory and allocator constraints.
    #[must_use]
    pub const fn maximum_constructible_count() -> usize {
        isize::MAX as usize / std::mem::size_of::<Qubit>()
    }

    /// Returns the number of logical qubits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.qubits.len()
    }

    /// Returns whether the register is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.qubits.is_empty()
    }

    /// Returns the logical qubit at `id`.
    pub fn get(
        &self,
        id: QubitId,
    ) -> Result<&Qubit, QubitError> {
        self.qubits
            .get(id.index())
            .ok_or(QubitError::OutOfRange {
                qubit: id,
                num_qubits: self.len(),
            })
    }

    /// Returns a logical qubit without constructing an error.
    #[must_use]
    pub fn get_opt(
        &self,
        id: QubitId,
    ) -> Option<&Qubit> {
        self.qubits.get(id.index())
    }

    /// Returns an immutable slice of the logical register.
    ///
    /// A mutable slice is intentionally not exposed because unrestricted
    /// mutation could violate register identity/state invariants.
    #[must_use]
    pub fn as_slice(&self) -> &[Qubit] {
        &self.qubits
    }

    /// Returns an immutable deterministic iterator.
    pub fn iter(&self) -> std::slice::Iter<'_, Qubit> {
        self.qubits.iter()
    }

    /// Returns the first usable logical qubit.
    ///
    /// Search order is deterministic by logical identifier.
    #[must_use]
    pub fn first_available(&self) -> Option<QubitId> {
        self.qubits
            .iter()
            .find(|qubit| qubit.is_usable())
            .map(Qubit::id)
    }

    /// Validates logical membership.
    pub fn validate(
        &self,
        id: QubitId,
    ) -> Result<(), QubitError> {
        if id.index() >= self.len() {
            return Err(QubitError::OutOfRange {
                qubit: id,
                num_qubits: self.len(),
            });
        }

        Ok(())
    }

    /// Validates membership and usability.
    pub fn validate_usable(
        &self,
        id: QubitId,
    ) -> Result<(), QubitError> {
        let qubit = self.get(id)?;

        if qubit.is_disabled() {
            return Err(QubitError::Disabled { qubit: id });
        }

        Ok(())
    }

    /// Marks a logical qubit as measured.
    ///
    /// This does not make the qubit permanently unusable.
    pub fn mark_measured(
        &mut self,
        id: QubitId,
    ) -> Result<(), QubitError> {
        let qubit = self.get_mut_internal(id)?;

        if qubit.is_disabled() {
            return Err(QubitError::Disabled { qubit: id });
        }

        qubit.mark_measured();

        Ok(())
    }

    /// Marks a logical qubit as reset.
    pub fn reset(
        &mut self,
        id: QubitId,
    ) -> Result<(), QubitError> {
        let qubit = self.get_mut_internal(id)?;

        if qubit.is_disabled() {
            return Err(QubitError::Disabled { qubit: id });
        }

        qubit.mark_reset();

        Ok(())
    }

    /// Marks a logical qubit available.
    ///
    /// This is explicit so later compiler stages do not infer state
    /// transitions from unrelated operations.
    pub fn mark_available(
        &mut self,
        id: QubitId,
    ) -> Result<(), QubitError> {
        let qubit = self.get_mut_internal(id)?;

        if qubit.is_disabled() {
            return Err(QubitError::Disabled { qubit: id });
        }

        qubit.mark_available();

        Ok(())
    }

    /// Disables a logical qubit.
    ///
    /// This is an IR namespace operation, not a physical hardware operation.
    pub fn disable(
        &mut self,
        id: QubitId,
    ) -> Result<(), QubitError> {
        let qubit = self.get_mut_internal(id)?;

        qubit.mark_disabled();

        Ok(())
    }

    /// Re-enables a logical qubit.
    pub fn enable(
        &mut self,
        id: QubitId,
    ) -> Result<(), QubitError> {
        let qubit = self.get_mut_internal(id)?;

        qubit.mark_available();

        Ok(())
    }

    /// Validates every supplied logical qubit against this register and
    /// rejects duplicate operands.
    pub fn validate_operands(
        &self,
        qubits: &[QubitId],
    ) -> Result<(), QubitError> {
        validate_qubits(qubits, self.len())?;

        for &qubit in qubits {
            self.validate_usable(qubit)?;
        }

        Ok(())
    }

    fn get_mut_internal(
        &mut self,
        id: QubitId,
    ) -> Result<&mut Qubit, QubitError> {
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

// =============================================================================
// Logical qubit collection
// =============================================================================

/// Validates that all logical-qubit operands are unique.
///
/// Unlike the previous implementation, this uses `BTreeSet` rather than
/// repeated slice searches.
///
/// Complexity:
///
/// ```text
/// O(n log n)
/// ```
///
/// rather than:
///
/// ```text
/// O(n²)
/// ```
///
/// The ordering is deterministic.
///
/// The caller should enforce `QuantumIrLimits::max_operands` before passing
/// attacker-controlled collections to this function.
pub fn validate_unique_qubits(
    qubits: &[QubitId],
) -> Result<(), QubitError> {
    let mut seen = BTreeSet::new();

    for &qubit in qubits {
        if !seen.insert(qubit) {
            return Err(QubitError::DuplicateQubit { qubit });
        }
    }

    Ok(())
}

/// Validates logical operands against a register size and uniqueness.
///
/// Validation order is:
///
/// 1. duplicate detection;
/// 2. range validation.
///
/// This produces deterministic errors.
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

/// Validates that all supplied logical operands are unique and usable.
///
/// This is useful for gate/operation construction before the operation itself
/// is created.
pub fn validate_usable_qubits(
    qubits: &[QubitId],
    register: &QubitRegister,
) -> Result<(), QubitError> {
    register.validate_operands(qubits)
}

/// Returns whether all logical qubits are unique.
///
/// This convenience API does not expose the underlying error.
#[must_use]
pub fn are_unique_qubits(
    qubits: &[QubitId],
) -> bool {
    validate_unique_qubits(qubits).is_ok()
}

/// Returns whether all logical qubits are valid for the supplied register
/// size.
///
/// This checks uniqueness and range.
#[must_use]
pub fn are_valid_qubits(
    qubits: &[QubitId],
    num_qubits: usize,
) -> bool {
    validate_qubits(qubits, num_qubits).is_ok()
}

// =============================================================================
// Logical-qubit collection helpers
// =============================================================================

/// Returns a deterministic sorted-and-deduplicated copy of logical qubit IDs.
///
/// This is useful at compiler boundaries where canonical operand ordering is
/// required.
///
/// The returned collection is newly allocated.
pub fn canonicalize_qubits(
    qubits: &[QubitId],
) -> Vec<QubitId> {
    let mut result = qubits.to_vec();

    result.sort_unstable();
    result.dedup();

    result
}

/// Returns a deterministic sorted copy and rejects duplicates.
///
/// Unlike `canonicalize_qubits`, this function preserves the invariant that
/// duplicate operands are errors.
pub fn canonicalize_unique_qubits(
    qubits: &[QubitId],
) -> Result<Vec<QubitId>, QubitError> {
    validate_unique_qubits(qubits)?;

    let mut result = qubits.to_vec();

    result.sort_unstable();

    Ok(result)
}

/// Returns the maximum logical-qubit index in a collection.
///
/// Returns `None` for an empty collection.
#[must_use]
pub fn max_qubit_index(
    qubits: &[QubitId],
) -> Option<usize> {
    qubits.iter().map(|qubit| qubit.index()).max()
}

/// Returns the minimum logical-qubit index in a collection.
///
/// Returns `None` for an empty collection.
#[must_use]
pub fn min_qubit_index(
    qubits: &[QubitId],
) -> Option<usize> {
    qubits.iter().map(|qubit| qubit.index()).min()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn p(index: usize) -> PhysicalQubitId {
        PhysicalQubitId::new(index)
    }

    // -------------------------------------------------------------------------
    // Identity tests
    // -------------------------------------------------------------------------

    #[test]
    fn logical_identifier_is_stable() {
        let id = q(7);

        assert_eq!(id.index(), 7);
        assert_eq!(id.to_string(), "q7");
    }

    #[test]
    fn physical_identifier_is_stable() {
        let id = p(7);

        assert_eq!(id.index(), 7);
        assert_eq!(id.to_string(), "p7");
    }

    #[test]
    fn logical_and_physical_ids_are_distinct_types() {
        let logical = q(3);
        let physical = p(3);

        assert_eq!(logical.index(), physical.index());
        assert_ne!(
            QubitRef::Logical(logical),
            QubitRef::Physical(physical)
        );
    }

    #[test]
    fn checked_identifier_increment_handles_overflow() {
        let id = QubitId::new(usize::MAX);

        assert_eq!(id.checked_next(), None);

        let physical = PhysicalQubitId::new(usize::MAX);

        assert_eq!(physical.checked_next(), None);
    }

    #[test]
    fn checked_identifier_increment_is_correct() {
        assert_eq!(
            q(10).checked_next(),
            Some(q(11))
        );

        assert_eq!(
            p(10).checked_next(),
            Some(p(11))
        );
    }

    // -------------------------------------------------------------------------
    // Qubit state tests
    // -------------------------------------------------------------------------

    #[test]
    fn new_qubit_is_available() {
        let qubit = Qubit::new(q(0));

        assert_eq!(qubit.id(), q(0));
        assert_eq!(qubit.state(), QubitState::Available);
        assert!(qubit.is_available());
        assert!(qubit.is_usable());
        assert!(!qubit.is_disabled());
    }

    #[test]
    fn reset_state_remains_usable() {
        let mut register = QubitRegister::new(1);

        register.reset(q(0)).unwrap();

        let qubit = register.get(q(0)).unwrap();

        assert!(qubit.is_reset());
        assert!(qubit.is_usable());
    }

    #[test]
    fn measured_state_remains_usable() {
        let mut register = QubitRegister::new(1);

        register.mark_measured(q(0)).unwrap();

        let qubit = register.get(q(0)).unwrap();

        assert!(qubit.is_measured());
        assert!(qubit.is_usable());
    }

    #[test]
    fn disabled_state_is_not_usable() {
        let mut register = QubitRegister::new(1);

        register.disable(q(0)).unwrap();

        let qubit = register.get(q(0)).unwrap();

        assert!(qubit.is_disabled());
        assert!(!qubit.is_usable());
    }

    // -------------------------------------------------------------------------
    // Register tests
    // -------------------------------------------------------------------------

    #[test]
    fn empty_register_is_safe() {
        let register = QubitRegister::empty();

        assert_eq!(register.len(), 0);
        assert!(register.is_empty());
        assert_eq!(register.first_available(), None);
    }

    #[test]
    fn register_contains_deterministic_ids() {
        let register = QubitRegister::new(4);

        assert_eq!(register.len(), 4);

        for index in 0..4 {
            assert_eq!(
                register.get(q(index)).unwrap().id(),
                q(index)
            );
        }
    }

    #[test]
    fn first_available_is_deterministic() {
        let register = QubitRegister::new(4);

        assert_eq!(
            register.first_available(),
            Some(q(0))
        );
    }

    #[test]
    fn first_available_skips_disabled_qubits() {
        let mut register = QubitRegister::new(4);

        register.disable(q(0)).unwrap();
        register.disable(q(1)).unwrap();

        assert_eq!(
            register.first_available(),
            Some(q(2))
        );
    }

    #[test]
    fn explicit_limit_rejects_before_allocation() {
        let result = QubitRegister::try_new(8, 4);

        assert_eq!(
            result,
            Err(QubitError::CountExceedsLimit {
                count: 8,
                maximum: 4,
            })
        );
    }

    #[test]
    fn explicit_limit_accepts_valid_count() {
        let register =
            QubitRegister::try_new(4, 4).unwrap();

        assert_eq!(register.len(), 4);
    }

    #[test]
    fn zero_count_is_valid() {
        let register =
            QubitRegister::try_new(0, 0).unwrap();

        assert!(register.is_empty());
    }

    #[test]
    fn out_of_range_is_rejected() {
        let register = QubitRegister::new(2);

        assert_eq!(
            register.get(q(2)),
            Err(QubitError::OutOfRange {
                qubit: q(2),
                num_qubits: 2,
            })
        );
    }

    #[test]
    fn optional_lookup_is_allocation_free() {
        let register = QubitRegister::new(2);

        assert!(register.get_opt(q(0)).is_some());
        assert!(register.get_opt(q(1)).is_some());
        assert!(register.get_opt(q(2)).is_none());
    }

    // -------------------------------------------------------------------------
    // State transition tests
    // -------------------------------------------------------------------------

    #[test]
    fn measurement_transition_is_controlled() {
        let mut register = QubitRegister::new(2);

        register.mark_measured(q(1)).unwrap();

        assert_eq!(
            register.get(q(1)).unwrap().state(),
            QubitState::Measured
        );
    }

    #[test]
    fn reset_transition_is_controlled() {
        let mut register = QubitRegister::new(2);

        register.reset(q(0)).unwrap();

        assert_eq!(
            register.get(q(0)).unwrap().state(),
            QubitState::Reset
        );
    }

    #[test]
    fn disabled_qubit_cannot_be_measured() {
        let mut register = QubitRegister::new(1);

        register.disable(q(0)).unwrap();

        assert_eq!(
            register.mark_measured(q(0)),
            Err(QubitError::Disabled { qubit: q(0) })
        );
    }

    #[test]
    fn disabled_qubit_cannot_be_reset() {
        let mut register = QubitRegister::new(1);

        register.disable(q(0)).unwrap();

        assert_eq!(
            register.reset(q(0)),
            Err(QubitError::Disabled { qubit: q(0) })
        );
    }

    #[test]
    fn disabled_qubit_can_be_reenabled() {
        let mut register = QubitRegister::new(1);

        register.disable(q(0)).unwrap();
        register.enable(q(0)).unwrap();

        assert!(register.get(q(0)).unwrap().is_usable());
        assert!(register.get(q(0)).unwrap().is_available());
    }

    #[test]
    fn measured_qubit_can_be_marked_available() {
        let mut register = QubitRegister::new(1);

        register.mark_measured(q(0)).unwrap();
        register.mark_available(q(0)).unwrap();

        assert!(register.get(q(0)).unwrap().is_available());
    }

    // -------------------------------------------------------------------------
    // Operand validation tests
    // -------------------------------------------------------------------------

    #[test]
    fn duplicate_qubits_are_rejected() {
        let qubits = [q(0), q(1), q(0)];

        assert_eq!(
            validate_unique_qubits(&qubits),
            Err(QubitError::DuplicateQubit {
                qubit: q(0)
            })
        );
    }

    #[test]
    fn unique_qubits_are_accepted() {
        let qubits = [q(0), q(1), q(2)];

        assert_eq!(
            validate_unique_qubits(&qubits),
            Ok(())
        );
    }

    #[test]
    fn duplicate_detection_is_deterministic() {
        let qubits = [
            q(7),
            q(2),
            q(7),
            q(2),
        ];

        assert_eq!(
            validate_unique_qubits(&qubits),
            Err(QubitError::DuplicateQubit {
                qubit: q(7)
            })
        );
    }

    #[test]
    fn out_of_range_qubits_are_rejected() {
        let qubits = [q(0), q(2)];

        assert_eq!(
            validate_qubits(&qubits, 2),
            Err(QubitError::OutOfRange {
                qubit: q(2),
                num_qubits: 2,
            })
        );
    }

    #[test]
    fn valid_qubits_are_accepted() {
        let qubits = [q(0), q(1)];

        assert_eq!(
            validate_qubits(&qubits, 2),
            Ok(())
        );
    }

    #[test]
    fn disabled_operands_are_rejected() {
        let mut register = QubitRegister::new(2);

        register.disable(q(1)).unwrap();

        assert_eq!(
            register.validate_operands(&[q(0), q(1)]),
            Err(QubitError::Disabled {
                qubit: q(1)
            })
        );
    }

    #[test]
    fn usable_operands_are_accepted() {
        let register = QubitRegister::new(3);

        assert_eq!(
            register.validate_operands(
                &[q(0), q(2)]
            ),
            Ok(())
        );
    }

    #[test]
    fn boolean_unique_helper_is_correct() {
        assert!(are_unique_qubits(&[
            q(0),
            q(1)
        ]));

        assert!(!are_unique_qubits(&[
            q(0),
            q(0)
        ]));
    }

    #[test]
    fn boolean_validity_helper_is_correct() {
        assert!(are_valid_qubits(
            &[q(0), q(1)],
            2
        ));

        assert!(!are_valid_qubits(
            &[q(0), q(2)],
            2
        ));
    }

    // -------------------------------------------------------------------------
    // Range tests
    // -------------------------------------------------------------------------

    #[test]
    fn qubit_range_is_half_open() {
        let range =
            QubitRange::new(2, 5).unwrap();

        assert_eq!(range.start(), 2);
        assert_eq!(range.end(), 5);
        assert_eq!(range.len(), 3);

        let ids: Vec<QubitId> =
            range.iter().collect();

        assert_eq!(
            ids,
            vec![q(2), q(3), q(4)]
        );
    }

    #[test]
    fn empty_range_contains_no_qubits() {
        let range =
            QubitRange::empty(5);

        assert!(range.is_empty());
        assert_eq!(range.len(), 0);
        assert!(!range.contains(q(5)));
        assert_eq!(
            range.iter().count(),
            0
        );
    }

    #[test]
    fn invalid_range_is_rejected() {
        assert_eq!(
            QubitRange::new(5, 2),
            Err(
                QubitRangeError::InvalidBounds {
                    start: 5,
                    end: 2,
                }
            )
        );
    }

    #[test]
    fn range_contains_expected_qubits() {
        let range =
            QubitRange::new(10, 20).unwrap();

        assert!(!range.contains(q(9)));
        assert!(range.contains(q(10)));
        assert!(range.contains(q(19)));
        assert!(!range.contains(q(20)));
    }

    #[test]
    fn large_range_is_lazy() {
        let range =
            QubitRange::new(
                usize::MAX - 3,
                usize::MAX,
            )
            .unwrap();

        let ids: Vec<QubitId> =
            range.iter().collect();

        assert_eq!(
            ids,
            vec![
                q(usize::MAX - 3),
                q(usize::MAX - 2),
                q(usize::MAX - 1),
            ]
        );
    }

    // -------------------------------------------------------------------------
    // Collection canonicalization tests
    // -------------------------------------------------------------------------

    #[test]
    fn canonicalization_sorts_and_deduplicates() {
        let result =
            canonicalize_qubits(&[
                q(3),
                q(1),
                q(3),
                q(0),
                q(1),
            ]);

        assert_eq!(
            result,
            vec![
                q(0),
                q(1),
                q(3),
            ]
        );
    }

    #[test]
    fn canonicalize_unique_rejects_duplicates() {
        assert_eq!(
            canonicalize_unique_qubits(&[
                q(2),
                q(1),
                q(2),
            ]),
            Err(
                QubitError::DuplicateQubit {
                    qubit: q(2)
                }
            )
        );
    }

    #[test]
    fn canonicalize_unique_sorts_without_duplicates() {
        assert_eq!(
            canonicalize_unique_qubits(&[
                q(3),
                q(1),
                q(2),
            ])
            .unwrap(),
            vec![
                q(1),
                q(2),
                q(3),
            ]
        );
    }

    #[test]
    fn min_and_max_indices_are_correct() {
        let qubits = [
            q(9),
            q(2),
            q(17),
            q(4),
        ];

        assert_eq!(
            min_qubit_index(&qubits),
            Some(2)
        );

        assert_eq!(
            max_qubit_index(&qubits),
            Some(17)
        );
    }

    #[test]
    fn min_and_max_indices_handle_empty_collection() {
        let qubits: [QubitId; 0] = [];

        assert_eq!(
            min_qubit_index(&qubits),
            None
        );

        assert_eq!(
            max_qubit_index(&qubits),
            None
        );
    }

    // -------------------------------------------------------------------------
    // Iteration tests
    // -------------------------------------------------------------------------

    #[test]
    fn iteration_is_in_logical_order() {
        let register =
            QubitRegister::new(4);

        let ids: Vec<QubitId> =
            register
                .iter()
                .map(Qubit::id)
                .collect();

        assert_eq!(
            ids,
            vec![
                q(0),
                q(1),
                q(2),
                q(3),
            ]
        );
    }

    #[test]
    fn immutable_slice_does_not_expose_mutation() {
        let register =
            QubitRegister::new(2);

        let slice =
            register.as_slice();

        assert_eq!(slice.len(), 2);
        assert_eq!(slice[0].id(), q(0));
        assert_eq!(slice[1].id(), q(1));
    }

    // -------------------------------------------------------------------------
    // Qubit reference tests
    // -------------------------------------------------------------------------

    #[test]
    fn logical_reference_reports_logical_namespace() {
        let reference =
            QubitRef::Logical(q(4));

        assert!(reference.is_logical());
        assert!(!reference.is_physical());
        assert_eq!(
            reference.logical(),
            Some(q(4))
        );
        assert_eq!(
            reference.physical(),
            None
        );
    }

    #[test]
    fn physical_reference_reports_physical_namespace() {
        let reference =
            QubitRef::Physical(p(4));

        assert!(!reference.is_logical());
        assert!(reference.is_physical());
        assert_eq!(
            reference.logical(),
            None
        );
        assert_eq!(
            reference.physical(),
            Some(p(4))
        );
    }

    // -------------------------------------------------------------------------
    // Scalability-boundary tests
    // -------------------------------------------------------------------------

    #[test]
    fn identifier_model_has_no_small_fixed_machine_limit() {
        let values = [
            0usize,
            1,
            63,
            64,
            127,
            128,
            4096,
            1_000_000,
        ];

        for value in values {
            assert_eq!(
                QubitId::new(value).index(),
                value
            );

            assert_eq!(
                PhysicalQubitId::new(value).index(),
                value
            );
        }
    }

    #[test]
    fn identifier_supports_platform_maximum() {
        let logical =
            QubitId::new(usize::MAX);

        let physical =
            PhysicalQubitId::new(usize::MAX);

        assert_eq!(
            logical.index(),
            usize::MAX
        );

        assert_eq!(
            physical.index(),
            usize::MAX
        );
    }
}