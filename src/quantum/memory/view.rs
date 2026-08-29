//! Zamani Quantum Memory — Representation-independent memory views.
//!
//! `view.rs` defines the non-owning addressing contract used to expose a
//! selected portion of quantum memory without copying the underlying state.
//!
//! # Architectural boundary
//!
//! A [`MemoryView`] is a descriptor, not a state container. It does not own
//! amplitudes, density-matrix elements, tensors, device buffers, QPU handles,
//! or allocator objects.
//!
//! It describes which logical qubits and storage positions an operation is
//! allowed to observe or modify.
//!
//! The same abstraction is intended to work with:
//!
//! - dense state vectors;
//! - density matrices;
//! - stabilizer/tableau states;
//! - sparse states;
//! - tensor networks;
//! - CPU/SIMD memory;
//! - GPU/device memory;
//! - distributed state partitions;
//! - remote/QPU-backed execution.
//!
//! No vendor-specific API is referenced here.
//!
//! # Important distinction
//!
//! A memory view is not:
//!
//! - a state-vector slice;
//! - a density-matrix partial trace;
//! - a tensor contraction;
//! - a copied state;
//! - a device pointer;
//! - a QPU handle.
//!
//! It is an addressing and permission projection. The mathematical meaning
//! of an operation performed through the view remains owned by the relevant
//! state/operation subsystem.
//!
//! # Ownership
//!
//! This module owns no storage and exposes no raw pointers.
//!
//! There is deliberately no `unsafe` code.
//!
//! A view is immutable after construction. The underlying allocation owner
//! remains responsible for validating allocation liveness before using the
//! descriptor against actual storage.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::ir::QubitId
//!          │
//!          ▼
//! quantum::memory::layout
//!          │
//!          ▼
//! quantum::memory::view
//!          │
//!     ┌────┼──────────────┐
//!     ▼    ▼              ▼
//! state  tensor       accelerator
//! vector network       / QPU
//! ```
//!
//! `view.rs` must not depend on:
//!
//! - routing;
//! - scheduling;
//! - algorithms;
//! - benchmarking;
//! - hardware vendors;
//! - a particular state representation.
//!
//! # Integration contract
//!
//! Later memory modules should consume this API instead of defining local
//! view abstractions:
//!
//! - `state.rs` uses [`MemoryView`] as the common addressing boundary.
//! - `state_vector.rs` uses basis/index mapping helpers.
//! - `density_matrix.rs` uses the same logical selection for rows/columns.
//! - `stabilizer.rs` uses selected logical qubits without assuming amplitudes.
//! - `sparse.rs` uses logical-coordinate selection without implicit densification.
//! - `tensor_network.rs` uses ordered logical selections.
//! - `slice.rs` owns mathematical slicing/projection semantics.
//! - `permutation.rs` owns permutation construction.
//! - `gpu.rs` translates logical/storage selections into device addresses.
//! - `distributed.rs` translates selections into partition-local coordinates.
//! - `migration.rs` may rebind a view after destination-layout validation.
//! - `measurement.rs` should use read-only views unless mutation is required.
//! - `collapse.rs` and `reset.rs` require explicit writable views.
//! - `allocator.rs` validates allocation identity/liveness.
//!
//! The view itself never dereferences memory.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//! No `unsafe` code is used.

use std::fmt;

use crate::quantum::ir::QubitId;

use super::layout::{LayoutError, MemoryLayout};

// =============================================================================
// Result / error model
// =============================================================================

/// Result type used by the memory-view subsystem.
pub type ViewResult<T> = Result<T, ViewError>;

/// Errors produced by memory-view construction and checked transformations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewError {
    /// A view requires at least one selected qubit.
    EmptySelection,

    /// A selected logical qubit is not part of the base layout.
    QubitOutOfRange {
        qubit: QubitId,
        num_qubits: usize,
    },

    /// A logical qubit occurs more than once.
    DuplicateQubit {
        qubit: QubitId,
    },

    /// A storage position is outside the base layout.
    StoragePositionOutOfRange {
        position: usize,
        num_positions: usize,
    },

    /// A storage position occurs more than once.
    DuplicateStoragePosition {
        position: usize,
    },

    /// A base basis index is invalid.
    BasisOutOfRange {
        basis: usize,
        num_qubits: usize,
    },

    /// A local basis index is invalid for this view.
    LocalBasisOutOfRange {
        basis: usize,
        num_qubits: usize,
    },

    /// A read-only view was used where mutation was required.
    ReadOnlyView,

    /// Explicit mutation authority was not supplied.
    WritePermissionRequired,

    /// Two view layouts cannot safely be combined.
    IncompatibleLayouts {
        reason: String,
    },

    /// A view cannot be rebound to a new layout.
    InvalidRebind {
        reason: String,
    },

    /// Checked arithmetic failed.
    ArithmeticOverflow,

    /// A local view index is invalid.
    ViewIndexOutOfRange {
        index: usize,
        len: usize,
    },

    /// A requested projection cannot be represented safely.
    InvalidProjection {
        reason: String,
    },

    /// Underlying layout operation failed.
    Layout(LayoutError),
}

impl From<LayoutError> for ViewError {
    fn from(error: LayoutError) -> Self {
        Self::Layout(error)
    }
}

impl fmt::Display for ViewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySelection => {
                write!(f, "memory view selection is empty")
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    f,
                    "qubit {qubit} is outside the base layout range 0..{num_qubits}"
                )
            }

            Self::DuplicateQubit { qubit } => {
                write!(f, "qubit {qubit} occurs more than once in the view")
            }

            Self::StoragePositionOutOfRange {
                position,
                num_positions,
            } => {
                write!(
                    f,
                    "storage position {position} is outside base range 0..{num_positions}"
                )
            }

            Self::DuplicateStoragePosition { position } => {
                write!(
                    f,
                    "storage position {position} occurs more than once in the view"
                )
            }

            Self::BasisOutOfRange {
                basis,
                num_qubits,
            } => {
                write!(
                    f,
                    "basis index {basis} is outside the {num_qubits}-qubit basis space"
                )
            }

            Self::LocalBasisOutOfRange {
                basis,
                num_qubits,
            } => {
                write!(
                    f,
                    "local basis index {basis} is outside the {num_qubits}-qubit view basis space"
                )
            }

            Self::ReadOnlyView => {
                write!(f, "memory view is read-only")
            }

            Self::WritePermissionRequired => {
                write!(
                    f,
                    "a writable view requires explicit write permission"
                )
            }

            Self::IncompatibleLayouts { reason } => {
                write!(f, "incompatible memory-view layouts: {reason}")
            }

            Self::InvalidRebind { reason } => {
                write!(f, "invalid memory-view layout rebind: {reason}")
            }

            Self::ArithmeticOverflow => {
                write!(f, "memory-view arithmetic overflow")
            }

            Self::ViewIndexOutOfRange { index, len } => {
                write!(
                    f,
                    "view index {index} is outside range 0..{len}"
                )
            }

            Self::InvalidProjection { reason } => {
                write!(f, "invalid memory-view projection: {reason}")
            }

            Self::Layout(error) => {
                write!(f, "memory layout error: {error}")
            }
        }
    }
}

impl std::error::Error for ViewError {}

// =============================================================================
// Permission
// =============================================================================

/// Permission associated with a view.
///
/// A read-only view cannot be used for mutation. A writable view must be
/// explicitly created by an owner that already has mutation authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ViewAccess {
    /// Observation only.
    ReadOnly,

    /// Mutation is permitted by the view contract.
    ReadWrite,
}

impl Default for ViewAccess {
    fn default() -> Self {
        Self::ReadOnly
    }
}

impl ViewAccess {
    /// Returns true when mutation is permitted.
    pub const fn is_writable(self) -> bool {
        matches!(self, Self::ReadWrite)
    }

    /// Returns true when the view is read-only.
    pub const fn is_read_only(self) -> bool {
        matches!(self, Self::ReadOnly)
    }

    /// Downgrades permission to read-only.
    pub const fn read_only(self) -> Self {
        let _ = self;
        Self::ReadOnly
    }
}

// =============================================================================
// View kind
// =============================================================================

/// Describes the purpose/shape of a view.
///
/// This is metadata only. It does not impose mathematical semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ViewKind {
    /// Entire logical memory.
    Full,

    /// Arbitrary logical-qubit selection.
    QubitSubset,

    /// Selection originally expressed in storage coordinates.
    StorageSubset,

    /// Exactly one logical qubit.
    SingleQubit,

    /// Exactly one storage position.
    SingleStoragePosition,

    /// View derived from another view.
    Composed,
}

impl Default for ViewKind {
    fn default() -> Self {
        Self::Full
    }
}

// =============================================================================
// Allocation identity
// =============================================================================

/// Opaque allocation identity carried by a view descriptor.
///
/// This is deliberately independent of the allocator implementation.
///
/// The allocation owner is responsible for checking whether this identity is
/// still live before using the view against actual storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ViewAllocationId(u64);

impl ViewAllocationId {
    /// Creates an allocation identity.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric identity.
    pub const fn get(self) -> u64 {
        self.0
    }
}

// =============================================================================
// MemoryView
// =============================================================================

/// Immutable, non-owning view of a quantum-memory layout.
///
/// The selection is stored in **view order**.
///
/// For example:
///
/// ```text
/// selection = [q3, q0, q2]
/// ```
///
/// means:
///
/// ```text
/// local index 0 -> q3
/// local index 1 -> q0
/// local index 2 -> q2
/// ```
///
/// The underlying memory is never copied by this type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryView {
    layout: MemoryLayout,
    selection: Vec<QubitId>,
    access: ViewAccess,
    kind: ViewKind,
    allocation_id: Option<ViewAllocationId>,
}

impl MemoryView {
    // =========================================================================
    // Constructors
    // =========================================================================

    /// Creates a read-only view covering the entire layout.
    pub fn full(layout: MemoryLayout) -> Self {
        let selection = (0..layout.num_qubits())
            .map(QubitId::new)
            .collect();

        Self {
            layout,
            selection,
            access: ViewAccess::ReadOnly,
            kind: ViewKind::Full,
            allocation_id: None,
        }
    }

    /// Creates a writable full-memory view.
    ///
    /// This does not acquire allocator ownership. The caller must already
    /// possess mutation authority for the underlying allocation.
    pub fn full_read_write(layout: MemoryLayout) -> Self {
        let mut view = Self::full(layout);
        view.access = ViewAccess::ReadWrite;
        view
    }

    /// Creates a read-only view over selected logical qubits.
    pub fn qubits<I>(
        layout: MemoryLayout,
        qubits: I,
    ) -> ViewResult<Self>
    where
        I: IntoIterator<Item = QubitId>,
    {
        Self::with_selection(
            layout,
            qubits.into_iter().collect(),
            ViewAccess::ReadOnly,
            ViewKind::QubitSubset,
            None,
        )
    }

    /// Creates a writable view over selected logical qubits.
    pub fn qubits_read_write<I>(
        layout: MemoryLayout,
        qubits: I,
    ) -> ViewResult<Self>
    where
        I: IntoIterator<Item = QubitId>,
    {
        Self::with_selection(
            layout,
            qubits.into_iter().collect(),
            ViewAccess::ReadWrite,
            ViewKind::QubitSubset,
            None,
        )
    }

    /// Creates a read-only single-qubit view.
    pub fn qubit(
        layout: MemoryLayout,
        qubit: QubitId,
    ) -> ViewResult<Self> {
        Self::with_selection(
            layout,
            vec![qubit],
            ViewAccess::ReadOnly,
            ViewKind::SingleQubit,
            None,
        )
    }

    /// Creates a writable single-qubit view.
    pub fn qubit_read_write(
        layout: MemoryLayout,
        qubit: QubitId,
    ) -> ViewResult<Self> {
        Self::with_selection(
            layout,
            vec![qubit],
            ViewAccess::ReadWrite,
            ViewKind::SingleQubit,
            None,
        )
    }

    /// Creates a read-only view from storage positions.
    ///
    /// Storage positions are immediately translated through `MemoryLayout`
    /// into canonical logical qubit identities.
    pub fn storage_positions<I>(
        layout: MemoryLayout,
        positions: I,
    ) -> ViewResult<Self>
    where
        I: IntoIterator<Item = usize>,
    {
        let positions = positions.into_iter().collect::<Vec<_>>();

        let mut qubits = Vec::with_capacity(positions.len());
        let mut seen = Vec::with_capacity(positions.len());

        for position in positions {
            layout.validate_storage_position(position)?;

            if seen.iter().any(|candidate| *candidate == position) {
                return Err(ViewError::DuplicateStoragePosition { position });
            }

            seen.push(position);
            qubits.push(layout.logical_at(position)?);
        }

        Self::with_selection(
            layout,
            qubits,
            ViewAccess::ReadOnly,
            ViewKind::StorageSubset,
            None,
        )
    }

    /// Creates a writable view from storage positions.
    pub fn storage_positions_read_write<I>(
        layout: MemoryLayout,
        positions: I,
    ) -> ViewResult<Self>
    where
        I: IntoIterator<Item = usize>,
    {
        let positions = positions.into_iter().collect::<Vec<_>>();

        let mut qubits = Vec::with_capacity(positions.len());
        let mut seen = Vec::with_capacity(positions.len());

        for position in positions {
            layout.validate_storage_position(position)?;

            if seen.iter().any(|candidate| *candidate == position) {
                return Err(ViewError::DuplicateStoragePosition { position });
            }

            seen.push(position);
            qubits.push(layout.logical_at(position)?);
        }

        Self::with_selection(
            layout,
            qubits,
            ViewAccess::ReadWrite,
            ViewKind::StorageSubset,
            None,
        )
    }

    fn with_selection(
        layout: MemoryLayout,
        selection: Vec<QubitId>,
        access: ViewAccess,
        kind: ViewKind,
        allocation_id: Option<ViewAllocationId>,
    ) -> ViewResult<Self> {
        if selection.is_empty() {
            return Err(ViewError::EmptySelection);
        }

        let mut seen = Vec::with_capacity(selection.len());

        for &qubit in &selection {
            layout.validate_logical_qubit(qubit)?;

            if seen.iter().any(|candidate| *candidate == qubit) {
                return Err(ViewError::DuplicateQubit { qubit });
            }

            seen.push(qubit);
        }

        Ok(Self {
            layout,
            selection,
            access,
            kind,
            allocation_id,
        })
    }

    // =========================================================================
    // Metadata
    // =========================================================================

    /// Returns the base memory layout.
    pub fn layout(&self) -> &MemoryLayout {
        &self.layout
    }

    /// Returns selected qubits in view order.
    pub fn qubits(&self) -> &[QubitId] {
        &self.selection
    }

    /// Returns the number of selected qubits.
    pub fn len(&self) -> usize {
        self.selection.len()
    }

    /// Returns true when the view has no selected qubits.
    pub fn is_empty(&self) -> bool {
        self.selection.is_empty()
    }

    /// Returns permission.
    pub const fn access(&self) -> ViewAccess {
        self.access
    }

    /// Returns semantic kind.
    pub const fn kind(&self) -> ViewKind {
        self.kind
    }

    /// Returns allocation identity, when attached.
    pub const fn allocation_id(&self) -> Option<ViewAllocationId> {
        self.allocation_id
    }

    /// Associates an allocation identity with this descriptor.
    ///
    /// This does not acquire or validate the allocation.
    pub const fn with_allocation_id(
        mut self,
        id: ViewAllocationId,
    ) -> Self {
        self.allocation_id = Some(id);
        self
    }

    /// Removes allocation identity.
    pub const fn without_allocation_id(mut self) -> Self {
        self.allocation_id = None;
        self
    }

    /// Returns true when writable.
    pub const fn is_writable(&self) -> bool {
        self.access.is_writable()
    }

    /// Returns true when read-only.
    pub const fn is_read_only(&self) -> bool {
        self.access.is_read_only()
    }

    // =========================================================================
    // Permission
    // =========================================================================

    /// Produces a read-only version of this view.
    pub fn as_read_only(&self) -> Self {
        let mut result = self.clone();
        result.access = ViewAccess::ReadOnly;
        result
    }

    /// Upgrades to writable only when the caller explicitly supplies
    /// mutation authority.
    ///
    /// The allocator/runtime should only pass `true` after validating the
    /// caller's ownership of the underlying allocation.
    pub fn try_as_read_write(
        &self,
        mutation_authorized: bool,
    ) -> ViewResult<Self> {
        if !mutation_authorized {
            return Err(ViewError::WritePermissionRequired);
        }

        let mut result = self.clone();
        result.access = ViewAccess::ReadWrite;
        Ok(result)
    }

    /// Requires write permission.
    pub fn require_write(&self) -> ViewResult<()> {
        if self.is_writable() {
            Ok(())
        } else {
            Err(ViewError::ReadOnlyView)
        }
    }

    // =========================================================================
    // Selection
    // =========================================================================

    /// Returns a qubit at local view index.
    pub fn qubit_at(&self, index: usize) -> ViewResult<QubitId> {
        self.selection
            .get(index)
            .copied()
            .ok_or(ViewError::ViewIndexOutOfRange {
                index,
                len: self.selection.len(),
            })
    }

    /// Returns the local index of a selected qubit.
    pub fn local_index_of(&self, qubit: QubitId) -> ViewResult<usize> {
        self.layout.validate_logical_qubit(qubit)?;

        self.selection
            .iter()
            .position(|candidate| *candidate == qubit)
            .ok_or(ViewError::QubitOutOfRange {
                qubit,
                num_qubits: self.layout.num_qubits(),
            })
    }

    /// Returns whether a logical qubit belongs to this view.
    pub fn contains(&self, qubit: QubitId) -> bool {
        self.selection
            .iter()
            .any(|candidate| *candidate == qubit)
    }

    /// Returns the base storage position of a selected logical qubit.
    pub fn storage_position_of(
        &self,
        qubit: QubitId,
    ) -> ViewResult<usize> {
        if !self.contains(qubit) {
            return Err(ViewError::QubitOutOfRange {
                qubit,
                num_qubits: self.layout.num_qubits(),
            });
        }

        Ok(self.layout.position_of(qubit)?)
    }

    /// Returns the base storage position at local index.
    pub fn storage_position_at(
        &self,
        index: usize,
    ) -> ViewResult<usize> {
        let qubit = self.qubit_at(index)?;
        Ok(self.layout.position_of(qubit)?)
    }

    /// Returns selected storage positions in view order.
    pub fn storage_positions(&self) -> ViewResult<Vec<usize>> {
        self.selection
            .iter()
            .map(|&qubit| self.layout.position_of(qubit))
            .collect::<Result<Vec<_>, _>>()
            .map_err(ViewError::from)
    }

    /// Returns the base-layout bit position of a selected qubit.
    pub fn bit_position_of(
        &self,
        qubit: QubitId,
    ) -> ViewResult<usize> {
        if !self.contains(qubit) {
            return Err(ViewError::QubitOutOfRange {
                qubit,
                num_qubits: self.layout.num_qubits(),
            });
        }

        Ok(self.layout.bit_position(qubit)?)
    }

    /// Returns the base-layout bit mask of a selected qubit.
    pub fn bit_mask_of(
        &self,
        qubit: QubitId,
    ) -> ViewResult<usize> {
        if !self.contains(qubit) {
            return Err(ViewError::QubitOutOfRange {
                qubit,
                num_qubits: self.layout.num_qubits(),
            });
        }

        Ok(self.layout.bit_mask(qubit)?)
    }

    /// Returns the base basis-index bit position of a local view index.
    pub fn local_bit_position(
        &self,
        local_index: usize,
    ) -> ViewResult<usize> {
        let qubit = self.qubit_at(local_index)?;
        Ok(self.layout.bit_position(qubit)?)
    }

    /// Returns the base basis-index mask of a local view index.
    pub fn local_bit_mask(
        &self,
        local_index: usize,
    ) -> ViewResult<usize> {
        let qubit = self.qubit_at(local_index)?;
        Ok(self.layout.bit_mask(qubit)?)
    }

    // =========================================================================
    // Basis-index translation
    // =========================================================================

    /// Validates a basis index against the base layout.
    pub fn validate_basis(&self, basis: usize) -> ViewResult<()> {
        self.layout.validate_basis(basis)?;
        Ok(())
    }

    /// Extracts selected qubit values from a base basis index.
    ///
    /// The resulting local basis index uses view order:
    ///
    /// ```text
    /// local bit 0 -> selection[0]
    /// local bit 1 -> selection[1]
    /// ...
    /// ```
    pub fn local_basis_index(
        &self,
        basis: usize,
    ) -> ViewResult<usize> {
        self.layout.validate_basis(basis)?;

        let mut local = 0usize;

        for (local_index, &qubit) in
            self.selection.iter().enumerate()
        {
            let mask = self.layout.bit_mask(qubit)?;

            if (basis & mask) != 0 {
                local |= checked_bit_mask(local_index)?;
            }
        }

        Ok(local)
    }

    /// Embeds a local basis assignment into the base basis space.
    ///
    /// All non-selected base qubits are set to zero.
    ///
    /// This is an embedding operation. It is not a partial trace and does not
    /// represent a physical state transformation by itself.
    pub fn base_basis_from_local(
        &self,
        local_basis: usize,
    ) -> ViewResult<usize> {
        self.validate_local_basis(local_basis)?;

        let mut basis = 0usize;

        for (local_index, &qubit) in
            self.selection.iter().enumerate()
        {
            let local_mask = checked_bit_mask(local_index)?;

            if (local_basis & local_mask) != 0 {
                basis |= self.layout.bit_mask(qubit)?;
            }
        }

        Ok(basis)
    }

    /// Validates a compact local basis index.
    pub fn validate_local_basis(
        &self,
        basis: usize,
    ) -> ViewResult<()> {
        if self.selection.len() >= usize::BITS as usize {
            return Err(ViewError::InvalidProjection {
                reason:
                    "view contains too many qubits for a usize local basis index"
                        .to_owned(),
            });
        }

        let dimension = 1usize
            .checked_shl(self.selection.len() as u32)
            .ok_or(ViewError::ArithmeticOverflow)?;

        let maximum = dimension
            .checked_sub(1)
            .ok_or(ViewError::ArithmeticOverflow)?;

        if basis > maximum {
            return Err(ViewError::LocalBasisOutOfRange {
                basis,
                num_qubits: self.selection.len(),
            });
        }

        Ok(())
    }

    /// Returns local basis-space dimension.
    pub fn local_basis_dimension(&self) -> ViewResult<usize> {
        if self.selection.len() >= usize::BITS as usize {
            return Err(ViewError::InvalidProjection {
                reason:
                    "view contains too many qubits for a usize basis dimension"
                        .to_owned(),
            });
        }

        1usize
            .checked_shl(self.selection.len() as u32)
            .ok_or(ViewError::ArithmeticOverflow)
    }

    /// Extracts a local assignment and embeds it again.
    ///
    /// This provides a useful invariant for representation implementations:
    ///
    /// ```text
    /// base basis
    ///     │
    ///     ▼
    /// local_basis_index
    ///     │
    ///     ▼
    /// base_basis_from_local
    /// ```
    ///
    /// Non-selected base bits are intentionally cleared.
    pub fn local_basis_round_trip(
        &self,
        basis: usize,
    ) -> ViewResult<usize> {
        let local = self.local_basis_index(basis)?;
        self.base_basis_from_local(local)
    }

    // =========================================================================
    // View composition
    // =========================================================================

    /// Creates a read-only subview using local indices.
    pub fn select_local<I>(
        &self,
        local_indices: I,
    ) -> ViewResult<Self>
    where
        I: IntoIterator<Item = usize>,
    {
        let indices =
            local_indices.into_iter().collect::<Vec<_>>();

        if indices.is_empty() {
            return Err(ViewError::EmptySelection);
        }

        let mut qubits = Vec::with_capacity(indices.len());
        let mut seen = Vec::with_capacity(indices.len());

        for index in indices {
            let qubit = self.qubit_at(index)?;

            if seen.iter().any(|candidate| *candidate == qubit) {
                return Err(ViewError::DuplicateQubit { qubit });
            }

            seen.push(qubit);
            qubits.push(qubit);
        }

        Self::with_selection(
            self.layout.clone(),
            qubits,
            ViewAccess::ReadOnly,
            ViewKind::Composed,
            self.allocation_id,
        )
    }

    /// Creates a subview while preserving the parent's permission.
    pub fn select_local_preserving_access<I>(
        &self,
        local_indices: I,
    ) -> ViewResult<Self>
    where
        I: IntoIterator<Item = usize>,
    {
        let mut view = self.select_local(local_indices)?;
        view.access = self.access;
        Ok(view)
    }

    /// Creates a read-only subview by logical qubit identifiers.
    pub fn select_qubits<I>(
        &self,
        qubits: I,
    ) -> ViewResult<Self>
    where
        I: IntoIterator<Item = QubitId>,
    {
        let qubits =
            qubits.into_iter().collect::<Vec<_>>();

        if qubits.is_empty() {
            return Err(ViewError::EmptySelection);
        }

        for &qubit in &qubits {
            if !self.contains(qubit) {
                return Err(ViewError::QubitOutOfRange {
                    qubit,
                    num_qubits: self.layout.num_qubits(),
                });
            }
        }

        Self::with_selection(
            self.layout.clone(),
            qubits,
            ViewAccess::ReadOnly,
            ViewKind::Composed,
            self.allocation_id,
        )
    }

    /// Creates a subview preserving the parent's permission.
    pub fn select_qubits_preserving_access<I>(
        &self,
        qubits: I,
    ) -> ViewResult<Self>
    where
        I: IntoIterator<Item = QubitId>,
    {
        let mut view = self.select_qubits(qubits)?;
        view.access = self.access;
        Ok(view)
    }

    /// Reverses the view order.
    pub fn reversed(&self) -> ViewResult<Self> {
        let selection =
            self.selection.iter().rev().copied().collect();

        let mut view = Self::with_selection(
            self.layout.clone(),
            selection,
            self.access,
            ViewKind::Composed,
            self.allocation_id,
        )?;

        view.access = self.access;

        Ok(view)
    }

    // =========================================================================
    // Layout rebinding
    // =========================================================================

    /// Rebinds this view to another layout without copying state.
    ///
    /// Every selected logical qubit must exist in the new layout.
    ///
    /// This is intended for:
    ///
    /// - routing/permutation transitions;
    /// - state migration;
    /// - backend layout changes;
    /// - CPU/GPU/distributed representation changes.
    ///
    /// The operation does not infer a physical mapping and does not move
    /// storage.
    pub fn rebind_layout(
        &self,
        new_layout: MemoryLayout,
    ) -> ViewResult<Self> {
        for &qubit in &self.selection {
            if let Err(error) =
                new_layout.validate_logical_qubit(qubit)
            {
                return Err(ViewError::InvalidRebind {
                    reason: error.to_string(),
                });
            }
        }

        let mut result = self.clone();
        result.layout = new_layout;
        Ok(result)
    }

    /// Returns whether all selected logical qubits exist in another layout.
    pub fn can_rebind_layout(
        &self,
        new_layout: &MemoryLayout,
    ) -> bool {
        self.selection.iter().all(|&qubit| {
            new_layout
                .validate_logical_qubit(qubit)
                .is_ok()
        })
    }

    // =========================================================================
    // Backend-neutral layout information
    // =========================================================================

    /// Returns the base layout bit order.
    pub const fn bit_order(&self) -> super::layout::BitOrder {
        self.layout.bit_order()
    }

    /// Returns true when the view covers every logical qubit in canonical
    /// identity order.
    pub fn is_full(&self) -> bool {
        self.selection.len() == self.layout.num_qubits()
            && self
                .selection
                .iter()
                .enumerate()
                .all(|(index, qubit)| {
                    qubit.index() == index
                })
    }

    /// Returns true when selected qubits occupy consecutive storage
    /// positions, independent of their requested view order.
    pub fn is_storage_contiguous(&self) -> ViewResult<bool> {
        if self.selection.len() <= 1 {
            return Ok(true);
        }

        let mut positions = self.storage_positions()?;
        positions.sort_unstable();

        Ok(positions
            .windows(2)
            .all(|window| {
                window[1] == window[0].saturating_add(1)
            }))
    }

    /// Returns true when selected logical IDs are consecutive, independent
    /// of their requested view order.
    pub fn is_logically_contiguous(&self) -> bool {
        if self.selection.len() <= 1 {
            return true;
        }

        let mut indices = self
            .selection
            .iter()
            .map(|qubit| qubit.index())
            .collect::<Vec<_>>();

        indices.sort_unstable();

        indices.windows(2).all(|window| {
            window[1] == window[0].saturating_add(1)
        })
    }
}

// =============================================================================
// Internal helpers
// =============================================================================

/// Creates `1 << bit` with explicit overflow protection.
fn checked_bit_mask(bit: usize) -> ViewResult<usize> {
    if bit >= usize::BITS as usize {
        return Err(ViewError::ArithmeticOverflow);
    }

    1usize
        .checked_shl(bit as u32)
        .ok_or(ViewError::ArithmeticOverflow)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::memory::layout::{
        BitOrder,
        MemoryLayout,
        QubitOrder,
    };

    fn layout(qubits: usize) -> MemoryLayout {
        MemoryLayout::new(qubits)
            .expect("test layout must be valid")
    }

    #[test]
    fn full_view_covers_complete_namespace() {
        let view = MemoryView::full(layout(3));

        assert_eq!(view.len(), 3);
        assert!(view.is_full());

        assert_eq!(
            view.qubits(),
            &[
                QubitId::new(0),
                QubitId::new(1),
                QubitId::new(2),
            ]
        );

        assert!(view.is_read_only());
    }

    #[test]
    fn selected_view_preserves_requested_order() {
        let view = MemoryView::qubits(
            layout(4),
            [
                QubitId::new(3),
                QubitId::new(0),
                QubitId::new(2),
            ],
        )
        .expect("selection must be valid");

        assert_eq!(
            view.qubit_at(0).unwrap(),
            QubitId::new(3)
        );

        assert_eq!(
            view.qubit_at(1).unwrap(),
            QubitId::new(0)
        );

        assert_eq!(
            view.qubit_at(2).unwrap(),
            QubitId::new(2)
        );

        assert_eq!(
            view.local_index_of(QubitId::new(0)).unwrap(),
            1
        );
    }

    #[test]
    fn duplicate_qubits_are_rejected() {
        let error = MemoryView::qubits(
            layout(3),
            [
                QubitId::new(0),
                QubitId::new(0),
            ],
        )
        .expect_err("duplicate selection must fail");

        assert_eq!(
            error,
            ViewError::DuplicateQubit {
                qubit: QubitId::new(0)
            }
        );
    }

    #[test]
    fn out_of_range_qubits_are_rejected() {
        let error = MemoryView::qubit(
            layout(2),
            QubitId::new(2),
        )
        .expect_err("out-of-range qubit must fail");

        assert!(matches!(
            error,
            ViewError::Layout(
                LayoutError::LogicalQubitOutOfRange { .. }
            )
        ));
    }

    #[test]
    fn read_only_view_cannot_be_upgraded_without_authority() {
        let view = MemoryView::full(layout(2));

        assert_eq!(
            view.try_as_read_write(false),
            Err(ViewError::WritePermissionRequired)
        );
    }

    #[test]
    fn explicit_authority_can_create_writable_view() {
        let view = MemoryView::full(layout(2));

        let writable = view
            .try_as_read_write(true)
            .expect(
                "explicit authority should permit write view"
            );

        assert!(writable.is_writable());
        assert!(writable.require_write().is_ok());
    }

    #[test]
    fn local_basis_index_extracts_selected_bits() {
        let view = MemoryView::qubits(
            layout(4),
            [
                QubitId::new(3),
                QubitId::new(1),
            ],
        )
        .unwrap();

        // Base basis 0b1010:
        //
        // q3 = 1
        // q1 = 1
        //
        // local = 0b11.
        assert_eq!(
            view.local_basis_index(0b1010).unwrap(),
            0b11
        );

        // q3 = 1, q1 = 0.
        assert_eq!(
            view.local_basis_index(0b1000).unwrap(),
            0b01
        );
    }

    #[test]
    fn local_basis_embedding_round_trips_selected_bits() {
        let view = MemoryView::qubits(
            layout(4),
            [
                QubitId::new(3),
                QubitId::new(1),
            ],
        )
        .unwrap();

        assert_eq!(
            view.base_basis_from_local(0b00).unwrap(),
            0b0000
        );

        assert_eq!(
            view.base_basis_from_local(0b01).unwrap(),
            0b1000
        );

        assert_eq!(
            view.base_basis_from_local(0b10).unwrap(),
            0b0010
        );

        assert_eq!(
            view.base_basis_from_local(0b11).unwrap(),
            0b1010
        );
    }

    #[test]
    fn big_endian_layout_is_respected() {
        let base =
            MemoryLayout::with_bit_order(
                3,
                BitOrder::BigEndian,
            )
            .unwrap();

        let view = MemoryView::qubits(
            base,
            [
                QubitId::new(0),
                QubitId::new(2),
            ],
        )
        .unwrap();

        // q0 = bit 2
        // q2 = bit 0
        //
        // basis = 0b101
        // local = 0b11.
        assert_eq!(
            view.local_basis_index(0b101).unwrap(),
            0b11
        );
    }

    #[test]
    fn storage_positions_translate_through_layout() {
        let order =
            QubitOrder::try_from_logical_order(
                3,
                vec![
                    QubitId::new(2),
                    QubitId::new(0),
                    QubitId::new(1),
                ],
            )
            .unwrap();

        let base =
            MemoryLayout::from_order(
                3,
                BitOrder::LittleEndian,
                order,
            )
            .unwrap();

        let view =
            MemoryView::storage_positions(
                base,
                [0, 2],
            )
            .unwrap();

        assert_eq!(
            view.qubits(),
            &[
                QubitId::new(2),
                QubitId::new(1),
            ]
        );

        assert_eq!(
            view.storage_positions().unwrap(),
            vec![0, 2]
        );
    }

    #[test]
    fn subview_preserves_requested_local_order() {
        let view = MemoryView::qubits(
            layout(4),
            [
                QubitId::new(0),
                QubitId::new(1),
                QubitId::new(2),
                QubitId::new(3),
            ],
        )
        .unwrap();

        let sub =
            view.select_local([3, 1]).unwrap();

        assert_eq!(
            sub.qubits(),
            &[
                QubitId::new(3),
                QubitId::new(1),
            ]
        );

        assert!(sub.is_read_only());
    }

    #[test]
    fn writable_subview_retains_permission() {
        let view =
            MemoryView::full_read_write(layout(4));

        let sub = view
            .select_local_preserving_access([3, 1])
            .unwrap();

        assert!(sub.is_writable());

        let downgraded =
            sub.as_read_only();

        assert!(downgraded.is_read_only());
    }

    #[test]
    fn rebind_preserves_logical_selection() {
        let view = MemoryView::qubits(
            layout(3),
            [
                QubitId::new(0),
                QubitId::new(2),
            ],
        )
        .unwrap();

        let reordered =
            MemoryLayout::from_order(
                3,
                BitOrder::LittleEndian,
                QubitOrder::try_from_logical_order(
                    3,
                    vec![
                        QubitId::new(2),
                        QubitId::new(1),
                        QubitId::new(0),
                    ],
                )
                .unwrap(),
            )
            .unwrap();

        let rebound =
            view.rebind_layout(reordered)
                .unwrap();

        assert_eq!(
            rebound.qubits(),
            view.qubits()
        );

        assert_eq!(
            rebound.storage_positions().unwrap(),
            vec![2, 0]
        );
    }

    #[test]
    fn allocation_identity_is_metadata_only() {
        let view =
            MemoryView::full(layout(2))
                .with_allocation_id(
                    ViewAllocationId::new(42)
                );

        assert_eq!(
            view.allocation_id(),
            Some(ViewAllocationId::new(42))
        );

        assert_eq!(
            view.without_allocation_id()
                .allocation_id(),
            None
        );
    }

    #[test]
    fn local_dimension_is_checked() {
        let view =
            MemoryView::qubits(
                layout(8),
                (0..8).map(QubitId::new),
            )
            .unwrap();

        assert_eq!(
            view.local_basis_dimension().unwrap(),
            256
        );

        assert!(
            view.validate_local_basis(255)
                .is_ok()
        );

        assert!(
            view.validate_local_basis(256)
                .is_err()
        );
    }

    #[test]
    fn non_selected_qubit_is_not_accessible_through_view() {
        let view =
            MemoryView::qubits(
                layout(4),
                [
                    QubitId::new(0),
                    QubitId::new(2),
                ],
            )
            .unwrap();

        assert!(
            view.contains(QubitId::new(0))
        );

        assert!(
            !view.contains(QubitId::new(1))
        );

        assert!(
            view.local_index_of(QubitId::new(1))
                .is_err()
        );
    }

    #[test]
    fn storage_contiguity_is_based_on_base_positions() {
        let view =
            MemoryView::qubits(
                layout(5),
                [
                    QubitId::new(4),
                    QubitId::new(2),
                    QubitId::new(3),
                ],
            )
            .unwrap();

        assert!(
            !view.is_storage_contiguous().unwrap()
        );
    }

    #[test]
    fn reverse_view_is_representation_independent() {
        let view =
            MemoryView::qubits(
                layout(3),
                [
                    QubitId::new(0),
                    QubitId::new(1),
                    QubitId::new(2),
                ],
            )
            .unwrap();

        let reversed =
            view.reversed().unwrap();

        assert_eq!(
            reversed.qubits(),
            &[
                QubitId::new(2),
                QubitId::new(1),
                QubitId::new(0),
            ]
        );
    }
}