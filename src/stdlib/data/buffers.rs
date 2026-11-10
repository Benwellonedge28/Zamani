
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Data Buffers
//!
//! Path: src/stdlib/data/buffers.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides portable, host-side byte-buffer abstractions for Zamani.
//!
//! Supports:
//!
//! - owned dynamically sized byte buffers;
//! - immutable borrowed views;
//! - mutable borrowed views;
//! - checked slicing;
//! - fallible allocation;
//! - fallible copying;
//! - fallible capacity growth;
//! - safe mutation;
//! - byte-oriented interoperability.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! This module owns host-side byte-buffer abstractions only.
//!
//! Existing authorities:
//!
//! - src/stdlib/error.rs:
//!   standard-library error infrastructure.
//!
//! - src/stdlib/context.rs:
//!   shared operation context.
//!
//! - src/classical/collections/array.rs:
//!   canonical classical array shape and storage.
//!
//! - src/classical/collections/tensor.rs:
//!   canonical classical tensor convenience API.
//!
//! - src/quantum/:
//!   quantum computation and quantum-specific representations.
//!
//! - src/ai/:
//!   AI computation and tensor-related integration.
//!
//! - src/hdl/:
//!   hardware-description representations.
//!
//! - src/runtime/:
//!   execution, allocation and runtime resource management.
//!
//! - HAL/provider implementations:
//!   actual hardware memory and device-buffer access.
//!
//! ## DOES NOT OWN
//!
//! This module does not define:
//!
//! - tensor shape;
//! - tensor rank;
//! - tensor element types;
//! - array indexing semantics;
//! - quantum registers;
//! - physical qubits;
//! - hardware memory allocation;
//! - device addresses;
//! - DMA;
//! - distributed memory placement;
//! - execution scheduling;
//! - compiler IR;
//! - source-language ownership semantics.
//!
//! ## SCALABILITY
//!
//! No arbitrary universal maximum buffer size is imposed.
//!
//! No fixed maximum number of buffers is imposed.
//!
//! No fixed maximum number of devices is imposed.
//!
//! No fixed maximum memory capacity is imposed.
//!
//! Allocation is constrained by:
//!
//! - host address-space representation;
//! - Vec capacity representation;
//! - allocator availability;
//! - actual available resources;
//! - explicitly configured external operational budgets.
//!
//! These implementation constraints are not language-level limits.
//!
//! ## MEMORY MODEL
//!
//! Buffer owns its byte storage.
//!
//! BufferView borrows immutable bytes.
//!
//! BufferViewMut borrows mutable bytes.
//!
//! Views never allocate or copy the underlying bytes.
//!
//! Buffer does not silently convert borrowed data into owned data.
//!
//! ## ERROR MODEL
//!
//! Operations use the canonical standard-library Error type.
//!
//! BufferError preserves buffer-specific causal information.
//!
//! Allocation failures are reported rather than silently ignored.
//!
//! ## POCO-REAF
//!
//! Buffers describe host-side data access.
//!
//! They do not prescribe:
//!
//! - CPU architecture;
//! - GPU architecture;
//! - FPGA architecture;
//! - QPU architecture;
//! - memory-bank arrangement;
//! - cache-line size;
//! - register width;
//! - hardware vendor;
//! - physical memory address.
//!
//! Hardware-specific buffer implementations belong behind runtime/HAL
//! provider contracts.
//!
//! ## FREEZE CRITERIA
//!
//! This module is ready to freeze when:
//!
//! 1. src/stdlib/error.rs is exposed by the standard-library root.
//! 2. src/stdlib/data/mod.rs exports this module.
//! 3. src/stdlib/mod.rs exports data.
//! 4. All buffer operations have unit tests.
//! 5. Allocation failures propagate correctly.
//! 6. Borrowing invariants are preserved.
//! 7. Existing classical array and tensor storage remain authoritative.
//! 8. Formatting, compilation, tests and Clippy pass.

// =============================================================================
// IMPORTS
// =============================================================================

use core::fmt;
use core::ops::Range;

use crate::stdlib::error::{
    Error as StdlibError,
    ErrorKind,
};

// =============================================================================
// RESULT
// =============================================================================

/// Standard-library result type used by buffer operations.
pub type BufferResult<T> = std::result::Result<T, StdlibError>;

// =============================================================================
// BUFFER ERRORS
// =============================================================================

/// Detailed buffer-specific failures.
///
/// These errors are retained as the causal source of the canonical
/// standard-library Error returned by public buffer operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum BufferError {

    /// A requested range starts beyond the buffer.
    RangeStartOutOfBounds {
        /// Requested starting position.
        start: usize,

        /// Actual buffer length.
        buffer_len: usize,
    },

    /// A requested range ends beyond the buffer.
    RangeEndOutOfBounds {
        /// Requested ending position.
        end: usize,

        /// Actual buffer length.
        buffer_len: usize,
    },

    /// A range starts after its ending position.
    InvalidRange {
        /// Requested starting position.
        start: usize,

        /// Requested ending position.
        end: usize,
    },

    /// Two buffers have incompatible lengths for an exact operation.
    LengthMismatch {
        /// Expected number of bytes.
        expected: usize,

        /// Actual number of bytes.
        actual: usize,
    },

    /// A requested allocation failed.
    AllocationFailed {
        /// Requested additional or total capacity.
        requested: usize,
    },

    /// A requested capacity is not representable.
    CapacityOverflow {
        /// Requested additional capacity.
        additional: usize,
    },
}

impl fmt::Display for BufferError {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        match self {

            Self::RangeStartOutOfBounds {
                start,
                buffer_len,
            } => write!(
                formatter,
                "buffer range starts at {start}, \
                 beyond buffer length {buffer_len}"
            ),

            Self::RangeEndOutOfBounds {
                end,
                buffer_len,
            } => write!(
                formatter,
                "buffer range ends at {end}, \
                 beyond buffer length {buffer_len}"
            ),

            Self::InvalidRange {
                start,
                end,
            } => write!(
                formatter,
                "invalid buffer range {start}..{end}"
            ),

            Self::LengthMismatch {
                expected,
                actual,
            } => write!(
                formatter,
                "buffer length mismatch: \
                 expected {expected}, received {actual}"
            ),

            Self::AllocationFailed {
                requested,
            } => write!(
                formatter,
                "buffer allocation failed for \
                 requested capacity {requested}"
            ),

            Self::CapacityOverflow {
                additional,
            } => write!(
                formatter,
                "buffer capacity overflow while requesting \
                 {additional} additional bytes"
            ),
        }
    }
}

impl std::error::Error for BufferError {}

// =============================================================================
// STANDARD ERROR INTEGRATION
// =============================================================================

impl From<BufferError> for StdlibError {

    fn from(error: BufferError) -> Self {

        let kind = match &error {

            BufferError::InvalidRange { .. }
            | BufferError::RangeStartOutOfBounds { .. }
            | BufferError::RangeEndOutOfBounds { .. }
            | BufferError::LengthMismatch { .. } => {
                ErrorKind::InvalidInput
            }

            BufferError::AllocationFailed { .. }
            | BufferError::CapacityOverflow { .. } => {
                ErrorKind::ResourceUnavailable
            }
        };

        let message = error.to_string();

        StdlibError::new(kind, message)
            .in_component("stdlib.data.buffers")
            .with_source(error)
    }
}

// =============================================================================
// OWNED BUFFER
// =============================================================================

/// An owned, dynamically sized host-side byte buffer.
///
/// The buffer owns its bytes and may grow or shrink subject to available
/// resources and the host's address-space representation.
///
/// It deliberately does not implement infallible Clone. Use
/// [`Buffer::try_clone`] when an independent copy is required.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Buffer {

    bytes: Vec<u8>,
}

impl Buffer {

    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    /// Creates an empty buffer.
    #[must_use]
    pub const fn new() -> Self {

        Self {
            bytes: Vec::new(),
        }
    }

    /// Creates an empty buffer with fallibly reserved capacity.
    ///
    /// No bytes are initialized beyond the current length.
    pub fn try_with_capacity(
        capacity: usize,
    ) -> BufferResult<Self> {

        let mut bytes = Vec::new();

        bytes
            .try_reserve_exact(capacity)
            .map_err(|_| {
                BufferError::AllocationFailed {
                    requested: capacity,
                }
            })?;

        Ok(Self { bytes })
    }

    /// Takes ownership of an existing vector without copying its bytes.
    ///
    /// This operation performs no additional buffer allocation.
    #[must_use]
    pub fn from_vec(bytes: Vec<u8>) -> Self {

        Self { bytes }
    }

    /// Copies bytes from a borrowed slice using fallible allocation.
    pub fn try_from_slice(
        source: &[u8],
    ) -> BufferResult<Self> {

        let mut bytes = Vec::new();

        bytes
            .try_reserve_exact(source.len())
            .map_err(|_| {
                BufferError::AllocationFailed {
                    requested: source.len(),
                }
            })?;

        bytes.extend_from_slice(source);

        Ok(Self { bytes })
    }

    /// Creates a zero-initialized buffer of the requested length.
    ///
    /// Allocation failure is reported.
    pub fn try_zeroed(
        length: usize,
    ) -> BufferResult<Self> {

        let mut bytes = Vec::new();

        bytes
            .try_reserve_exact(length)
            .map_err(|_| {
                BufferError::AllocationFailed {
                    requested: length,
                }
            })?;

        bytes.resize(length, 0);

        Ok(Self { bytes })
    }

    // -------------------------------------------------------------------------
    // Metadata
    // -------------------------------------------------------------------------

    /// Returns the number of initialized bytes.
    #[must_use]
    pub fn len(&self) -> usize {

        self.bytes.len()
    }

    /// Returns whether the buffer contains no bytes.
    #[must_use]
    pub fn is_empty(&self) -> bool {

        self.bytes.is_empty()
    }

    /// Returns the currently reserved capacity.
    #[must_use]
    pub fn capacity(&self) -> usize {

        self.bytes.capacity()
    }

    /// Returns the number of initialized bytes.
    #[must_use]
    pub fn byte_len(&self) -> usize {

        self.bytes.len()
    }

    // -------------------------------------------------------------------------
    // Borrowing
    // -------------------------------------------------------------------------

    /// Returns an immutable view without copying.
    #[must_use]
    pub fn view(&self) -> BufferView<'_> {

        BufferView {
            bytes: &self.bytes,
        }
    }

    /// Returns a mutable view without copying.
    pub fn view_mut(&mut self) -> BufferViewMut<'_> {

        BufferViewMut {
            bytes: &mut self.bytes,
        }
    }

    /// Returns the initialized bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {

        &self.bytes
    }

    /// Returns the initialized bytes mutably.
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {

        &mut self.bytes
    }

    // -------------------------------------------------------------------------
    // Checked access
    // -------------------------------------------------------------------------

    /// Returns a byte at the requested position.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&u8> {

        self.bytes.get(index)
    }

    /// Returns a mutable byte at the requested position.
    pub fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut u8> {

        self.bytes.get_mut(index)
    }

    /// Returns a checked immutable range.
    pub fn slice(
        &self,
        range: Range<usize>,
    ) -> BufferResult<BufferView<'_>> {

        self.view().slice(range)
    }

    /// Returns a checked mutable range.
    pub fn slice_mut(
        &mut self,
        range: Range<usize>,
    ) -> BufferResult<BufferViewMut<'_>> {

        self.view_mut().slice(range)
    }

    // -------------------------------------------------------------------------
    // Capacity management
    // -------------------------------------------------------------------------

    /// Reserves additional capacity fallibly.
    ///
    /// The existing contents are preserved.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> BufferResult<()> {

        let requested = self
            .bytes
            .len()
            .checked_add(additional)
            .ok_or(
                BufferError::CapacityOverflow {
                    additional,
                }
            )?;

        self.bytes
            .try_reserve(additional)
            .map_err(|_| {
                BufferError::AllocationFailed {
                    requested,
                }
            })?;

        Ok(())
    }

    /// Reserves additional capacity as exactly as the allocator permits.
    pub fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> BufferResult<()> {

        let requested = self
            .bytes
            .len()
            .checked_add(additional)
            .ok_or(
                BufferError::CapacityOverflow {
                    additional,
                }
            )?;

        self.bytes
            .try_reserve_exact(additional)
            .map_err(|_| {
                BufferError::AllocationFailed {
                    requested,
                }
            })?;

        Ok(())
    }

    /// Changes the initialized length, filling newly added bytes with zero.
    ///
    /// Shrinking truncates the buffer.
    ///
    /// Growing reserves the required capacity before changing the length.
    pub fn try_resize_zeroed(
        &mut self,
        new_length: usize,
    ) -> BufferResult<()> {

        if new_length > self.bytes.len() {

            let additional = new_length - self.bytes.len();

            self.try_reserve(additional)?;
        }

        self.bytes.resize(new_length, 0);

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Mutation
    // -------------------------------------------------------------------------

    /// Appends one byte.
    pub fn try_push(
        &mut self,
        byte: u8,
    ) -> BufferResult<()> {

        self.try_reserve(1)?;

        self.bytes.push(byte);

        Ok(())
    }

    /// Appends a borrowed byte slice.
    pub fn try_extend_from_slice(
        &mut self,
        source: &[u8],
    ) -> BufferResult<()> {

        self.try_reserve(source.len())?;

        self.bytes.extend_from_slice(source);

        Ok(())
    }

    /// Replaces the contents with a copy of the supplied bytes.
    ///
    /// The operation reserves the necessary capacity before modifying
    /// the current contents.
    pub fn try_replace(
        &mut self,
        source: &[u8],
    ) -> BufferResult<()> {

        if source.len() > self.bytes.capacity() {

            self.bytes
                .try_reserve_exact(
                    source.len().saturating_sub(self.bytes.len())
                )
                .map_err(|_| {
                    BufferError::AllocationFailed {
                        requested: source.len(),
                    }
                })?;
        }

        self.bytes.clear();

        self.bytes.extend_from_slice(source);

        Ok(())
    }

    /// Removes and returns the final byte.
    pub fn pop(&mut self) -> Option<u8> {

        self.bytes.pop()
    }

    /// Removes all initialized bytes while retaining allocated capacity.
    pub fn clear(&mut self) {

        self.bytes.clear();
    }

    // -------------------------------------------------------------------------
    // Copying and ownership transfer
    // -------------------------------------------------------------------------

    /// Creates an independent copy using fallible allocation.
    pub fn try_clone(&self) -> BufferResult<Self> {

        Self::try_from_slice(&self.bytes)
    }

    /// Consumes the buffer and returns its underlying vector.
    #[must_use]
    pub fn into_vec(self) -> Vec<u8> {

        self.bytes
    }
}

// =============================================================================
// IMMUTABLE BUFFER VIEW
// =============================================================================

/// A borrowed, immutable view of contiguous bytes.
///
/// Creating or cloning a view does not copy its underlying bytes.
///
/// The lifetime prevents the view from outliving its source storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferView<'a> {

    bytes: &'a [u8],
}

impl<'a> BufferView<'a> {

    /// Creates a view over an existing byte slice.
    #[must_use]
    pub const fn new(
        bytes: &'a [u8],
    ) -> Self {

        Self { bytes }
    }

    /// Returns the viewed bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &'a [u8] {

        self.bytes
    }

    /// Returns the number of viewed bytes.
    #[must_use]
    pub const fn len(&self) -> usize {

        self.bytes.len()
    }

    /// Returns whether the view is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {

        self.bytes.is_empty()
    }

    /// Returns a byte at the requested index.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&'a u8> {

        self.bytes.get(index)
    }

    /// Returns a checked subview.
    ///
    /// No allocation or copying occurs.
    pub fn slice(
        &self,
        range: Range<usize>,
    ) -> BufferResult<BufferView<'a>> {

        validate_range(
            range.start,
            range.end,
            self.bytes.len(),
        )?;

        let bytes = self
            .bytes
            .get(range)
            .ok_or_else(|| {
                BufferError::InvalidRange {
                    start: 0,
                    end: 0,
                }
            })?;

        Ok(BufferView { bytes })
    }

    /// Returns the underlying byte slice as a standard Rust slice.
    #[must_use]
    pub const fn as_slice(&self) -> &'a [u8] {

        self.bytes
    }
}

impl<'a> From<&'a [u8]> for BufferView<'a> {

    fn from(bytes: &'a [u8]) -> Self {

        Self::new(bytes)
    }
}

// =============================================================================
// MUTABLE BUFFER VIEW
// =============================================================================

/// A borrowed, mutable view of contiguous bytes.
///
/// This view never owns the underlying allocation.
///
/// Rust's borrowing rules prevent simultaneous conflicting mutable access.
#[derive(Debug, PartialEq, Eq)]
pub struct BufferViewMut<'a> {

    bytes: &'a mut [u8],
}

impl<'a> BufferViewMut<'a> {

    /// Creates a mutable view over an existing mutable slice.
    #[must_use]
    pub fn new(
        bytes: &'a mut [u8],
    ) -> Self {

        Self { bytes }
    }

    /// Returns the viewed bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {

        self.bytes
    }

    /// Returns the viewed bytes mutably.
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {

        self.bytes
    }

    /// Returns the number of viewed bytes.
    #[must_use]
    pub fn len(&self) -> usize {

        self.bytes.len()
    }

    /// Returns whether the view is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {

        self.bytes.is_empty()
    }

    /// Returns a byte at the requested position.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&u8> {

        self.bytes.get(index)
    }

    /// Returns a mutable byte at the requested position.
    pub fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut u8> {

        self.bytes.get_mut(index)
    }

    /// Replaces all bytes with the supplied byte.
    pub fn fill(
        &mut self,
        value: u8,
    ) {

        self.bytes.fill(value);
    }

    /// Copies bytes into this view.
    ///
    /// The source and destination lengths must match exactly.
    pub fn copy_from_slice(
        &mut self,
        source: &[u8],
    ) -> BufferResult<()> {

        if source.len() != self.bytes.len() {

            return Err(
                BufferError::LengthMismatch {
                    expected: self.bytes.len(),
                    actual: source.len(),
                }
                .into()
            );
        }

        self.bytes.copy_from_slice(source);

        Ok(())
    }

    /// Returns a checked mutable subview.
    pub fn slice(
        &mut self,
        range: Range<usize>,
    ) -> BufferResult<BufferViewMut<'_>> {

        validate_range(
            range.start,
            range.end,
            self.bytes.len(),
        )?;

        let bytes = self
            .bytes
            .get_mut(range)
            .ok_or_else(|| {
                BufferError::InvalidRange {
                    start: 0,
                    end: 0,
                }
            })?;

        Ok(BufferViewMut { bytes })
    }

    /// Returns an immutable view of the same bytes.
    #[must_use]
    pub fn as_view(&self) -> BufferView<'_> {

        BufferView {
            bytes: self.bytes,
        }
    }
}

// =============================================================================
// RANGE VALIDATION
// =============================================================================

/// Validates a half-open byte range.
///
/// Both the start and end positions may equal the buffer length.
/// Empty ranges are valid.
///
/// Returns detailed errors for invalid ranges.
fn validate_range(
    start: usize,
    end: usize,
    buffer_len: usize,
) -> std::result::Result<(), BufferError> {

    if start > end {

        return Err(
            BufferError::InvalidRange {
                start,
                end,
            }
        );
    }

    if start > buffer_len {

        return Err(
            BufferError::RangeStartOutOfBounds {
                start,
                buffer_len,
            }
        );
    }

    if end > buffer_len {

        return Err(
            BufferError::RangeEndOutOfBounds {
                end,
                buffer_len,
            }
        );
    }

    Ok(())
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn creates_empty_buffer() {

        let buffer = Buffer::new();

        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.byte_len(), 0);
    }

    #[test]
    fn creates_buffer_from_vector_without_copying_semantics() {

        let source = vec![1, 2, 3, 4];

        let buffer = Buffer::from_vec(source);

        assert_eq!(
            buffer.as_bytes(),
            &[1, 2, 3, 4]
        );
    }

    #[test]
    fn creates_buffer_from_slice() {

        let source = [10, 20, 30];

        let buffer = Buffer::try_from_slice(&source)
            .expect("buffer allocation should succeed");

        assert_eq!(
            buffer.as_bytes(),
            &[10, 20, 30]
        );
    }

    #[test]
    fn creates_zeroed_buffer() {

        let buffer = Buffer::try_zeroed(8)
            .expect("allocation should succeed");

        assert_eq!(buffer.len(), 8);

        assert!(buffer.as_bytes().iter().all(|byte| *byte == 0));
    }

    #[test]
    fn grows_buffer() {

        let mut buffer = Buffer::new();

        buffer
            .try_extend_from_slice(&[1, 2, 3])
            .expect("allocation should succeed");

        buffer
            .try_resize_zeroed(5)
            .expect("growth should succeed");

        assert_eq!(
            buffer.as_bytes(),
            &[1, 2, 3, 0, 0]
        );
    }

    #[test]
    fn shrinks_buffer() {

        let mut buffer = Buffer::from_vec(
            vec![1, 2, 3, 4, 5]
        );

        buffer
            .try_resize_zeroed(2)
            .expect("shrinking should succeed");

        assert_eq!(
            buffer.as_bytes(),
            &[1, 2]
        );
    }

    #[test]
    fn immutable_view_does_not_copy() {

        let source = [1, 2, 3];

        let view = BufferView::new(&source);

        assert_eq!(view.as_bytes(), &source);
        assert_eq!(view.len(), 3);
    }

    #[test]
    fn immutable_view_supports_slicing() {

        let source = [10, 20, 30, 40];

        let view = BufferView::new(&source);

        let slice = view
            .slice(1..3)
            .expect("range should be valid");

        assert_eq!(
            slice.as_bytes(),
            &[20, 30]
        );
    }

    #[test]
    fn mutable_view_changes_original_storage() {

        let mut source = [1, 2, 3];

        {
            let mut view = BufferViewMut::new(&mut source);

            view.fill(9);
        }

        assert_eq!(source, [9, 9, 9]);
    }

    #[test]
    fn mutable_view_copies_exact_length() {

        let mut source = [0, 0, 0];

        let mut view = BufferViewMut::new(&mut source);

        view.copy_from_slice(&[4, 5, 6])
            .expect("lengths should match");

        assert_eq!(
            view.as_bytes(),
            &[4, 5, 6]
        );
    }

    #[test]
    fn mutable_view_rejects_length_mismatch() {

        let mut source = [0, 0];

        let mut view = BufferViewMut::new(&mut source);

        let result = view.copy_from_slice(&[1, 2, 3]);

        assert!(result.is_err());
    }

    #[test]
    fn invalid_start_is_rejected() {

        let source = [1, 2, 3];

        let view = BufferView::new(&source);

        assert!(view.slice(4..4).is_err());
    }

    #[test]
    fn invalid_end_is_rejected() {

        let source = [1, 2, 3];

        let view = BufferView::new(&source);

        assert!(view.slice(0..4).is_err());
    }

    #[test]
    fn reversed_range_is_rejected() {

        let source = [1, 2, 3];

        let view = BufferView::new(&source);

        assert!(view.slice(2..1).is_err());
    }

    #[test]
    fn empty_range_is_valid() {

        let source = [1, 2, 3];

        let view = BufferView::new(&source);

        let slice = view
            .slice(1..1)
            .expect("empty range should be valid");

        assert!(slice.is_empty());
    }

    #[test]
    fn clone_is_fallible_and_independent() {

        let buffer = Buffer::from_vec(
            vec![1, 2, 3]
        );

        let mut copied = buffer
            .try_clone()
            .expect("copy allocation should succeed");

        copied
            .try_push(4)
            .expect("growth should succeed");

        assert_eq!(
            buffer.as_bytes(),
            &[1, 2, 3]
        );

        assert_eq!(
            copied.as_bytes(),
            &[1, 2, 3, 4]
        );
    }

    #[test]
    fn clear_preserves_valid_empty_state() {

        let mut buffer = Buffer::from_vec(
            vec![1, 2, 3]
        );

        buffer.clear();

        assert!(buffer.is_empty());
    }

    #[test]
    fn pop_removes_last_byte() {

        let mut buffer = Buffer::from_vec(
            vec![1, 2, 3]
        );

        assert_eq!(buffer.pop(), Some(3));
        assert_eq!(buffer.as_bytes(), &[1, 2]);
    }

    #[test]
    fn ownership_can_be_transferred() {

        let buffer = Buffer::from_vec(
            vec![1, 2, 3]
        );

        let bytes = buffer.into_vec();

        assert_eq!(bytes, vec![1, 2, 3]);
    }

    #[test]
    fn buffer_errors_preserve_their_cause() {

        let error = StdlibError::from(
            BufferError::LengthMismatch {
                expected: 4,
                actual: 3,
            }
        );

        assert_eq!(
            error.kind(),
            ErrorKind::InvalidInput
        );

        assert!(error.source().is_some());
    }
}
