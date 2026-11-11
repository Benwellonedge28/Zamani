
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Data Buffers
//!
//! Portable, target-independent byte storage and access.
//!
//! This module owns host-side byte-buffer abstractions only.
//!
//! It does not own:
//! - tensor shapes or tensor storage;
//! - classical array semantics;
//! - quantum registers;
//! - physical memory allocation;
//! - device memory or DMA;
//! - distributed placement;
//! - execution scheduling;
//! - compiler intermediate representations.
//!
//! The canonical authorities remain:
//! - `crate::classical::collections`;
//! - `crate::quantum`;
//! - `crate::ai`;
//! - `crate::hdl`;
//! - `crate::runtime`;
//! - the applicable HAL and provider implementations.
//!
//! # Scalability
//!
//! No arbitrary universal buffer-size ceiling is introduced.
//!
//! `Buffer` provides contiguous storage.
//! `BufferChain` provides segmented storage.
//!
//! Actual allocation is subject to:
//! - address-space representation;
//! - allocator availability;
//! - platform limitations;
//! - available resources;
//! - explicitly configured operational budgets.
//!
//! These are implementation constraints, not language semantics.
//!
//! # Safety
//!
//! Safe Rust only.

use core::fmt;
use core::ops::Range;

use crate::stdlib::error::{
    Error as StdlibError,
    ErrorKind,
};

/// Result type used by buffer operations.
pub type BufferResult<T> = std::result::Result<T, StdlibError>;

/// Detailed buffer-specific failures.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum BufferError {
    /// Range starts beyond the buffer.
    RangeStartOutOfBounds {
        /// Requested starting position.
        start: usize,
        /// Actual buffer length.
        buffer_len: usize,
    },

    /// Range ends beyond the buffer.
    RangeEndOutOfBounds {
        /// Requested ending position.
        end: usize,
        /// Actual buffer length.
        buffer_len: usize,
    },

    /// Range starts after its ending position.
    InvalidRange {
        /// Requested starting position.
        start: usize,
        /// Requested ending position.
        end: usize,
    },

    /// An exact-length operation received different lengths.
    LengthMismatch {
        /// Expected number of bytes.
        expected: usize,
        /// Actual number of bytes.
        actual: usize,
    },

    /// Allocation failed.
    AllocationFailed {
        /// Requested capacity or length.
        requested: usize,
    },

    /// Capacity arithmetic overflowed.
    CapacityOverflow {
        /// Additional requested capacity.
        additional: usize,
    },

    /// A requested segment does not exist.
    SegmentNotFound {
        /// Requested segment index.
        index: usize,
        /// Number of available segments.
        segment_count: usize,
    },
}

impl fmt::Display for BufferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RangeStartOutOfBounds {
                start,
                buffer_len,
            } => write!(
                f,
                "range start {start} exceeds buffer length {buffer_len}"
            ),

            Self::RangeEndOutOfBounds {
                end,
                buffer_len,
            } => write!(
                f,
                "range end {end} exceeds buffer length {buffer_len}"
            ),

            Self::InvalidRange { start, end } => {
                write!(f, "invalid range {start}..{end}")
            }

            Self::LengthMismatch { expected, actual } => {
                write!(
                    f,
                    "length mismatch: expected {expected}, received {actual}"
                )
            }

            Self::AllocationFailed { requested } => {
                write!(
                    f,
                    "allocation failed for requested size {requested}"
                )
            }

            Self::CapacityOverflow { additional } => {
                write!(
                    f,
                    "capacity overflow while requesting {additional} additional bytes"
                )
            }

            Self::SegmentNotFound {
                index,
                segment_count,
            } => {
                write!(
                    f,
                    "segment {index} does not exist; segment count is {segment_count}"
                )
            }
        }
    }
}

impl std::error::Error for BufferError {}

impl From<BufferError> for StdlibError {
    fn from(error: BufferError) -> Self {
        let kind = match &error {
            BufferError::InvalidRange { .. }
            | BufferError::RangeStartOutOfBounds { .. }
            | BufferError::RangeEndOutOfBounds { .. }
            | BufferError::LengthMismatch { .. }
            | BufferError::SegmentNotFound { .. } => ErrorKind::InvalidInput,

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

/// An owned, dynamically sized contiguous byte buffer.
///
/// Allocation-sensitive operations use fallible APIs.
///
/// Independent copying is explicit through [`Buffer::try_clone`].
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Buffer {
    bytes: Vec<u8>,
}

impl Buffer {
    /// Creates an empty buffer.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bytes: Vec::new(),
        }
    }

    /// Creates an empty buffer with reserved capacity.
    pub fn try_with_capacity(capacity: usize) -> BufferResult<Self> {
        let mut bytes = Vec::new();

        bytes.try_reserve_exact(capacity).map_err(|_| {
            BufferError::AllocationFailed {
                requested: capacity,
            }
        })?;

        Ok(Self { bytes })
    }

    /// Takes ownership of an existing vector without copying.
    #[must_use]
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Copies bytes from a borrowed slice.
    pub fn try_from_slice(source: &[u8]) -> BufferResult<Self> {
        let mut buffer = Self::try_with_capacity(source.len())?;
        buffer.bytes.extend_from_slice(source);
        Ok(buffer)
    }

    /// Creates a zero-initialized buffer.
    pub fn try_zeroed(length: usize) -> BufferResult<Self> {
        let mut buffer = Self::try_with_capacity(length)?;
        buffer.bytes.resize(length, 0);
        Ok(buffer)
    }

    /// Returns the initialized byte count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the buffer is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns allocated capacity.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.bytes.capacity()
    }

    /// Returns the initialized byte count.
    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.len()
    }

    /// Returns an immutable borrowed view.
    #[must_use]
    pub fn view(&self) -> BufferView<'_> {
        BufferView::new(&self.bytes)
    }

    /// Returns a mutable borrowed view.
    pub fn view_mut(&mut self) -> BufferViewMut<'_> {
        BufferViewMut::new(&mut self.bytes)
    }

    /// Returns the underlying initialized bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the underlying initialized bytes mutably.
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }

    /// Returns a byte by index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&u8> {
        self.bytes.get(index)
    }

    /// Returns a mutable byte by index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut u8> {
        self.bytes.get_mut(index)
    }

    /// Returns a checked immutable slice.
    pub fn slice(
        &self,
        range: Range<usize>,
    ) -> BufferResult<BufferView<'_>> {
        self.view().slice(range)
    }

    /// Returns a checked mutable slice.
    pub fn slice_mut(
        &mut self,
        range: Range<usize>,
    ) -> BufferResult<BufferViewMut<'_>> {
        self.view_mut().slice(range)
    }

    /// Reserves additional capacity.
    ///
    /// Existing initialized bytes are preserved.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> BufferResult<()> {
        let requested = self.bytes.len().checked_add(additional)
            .ok_or(BufferError::CapacityOverflow { additional })?;

        self.bytes.try_reserve(additional).map_err(|_| {
            BufferError::AllocationFailed { requested }
        })?;

        Ok(())
    }

    /// Reserves additional capacity as exactly as possible.
    pub fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> BufferResult<()> {
        let requested = self.bytes.len().checked_add(additional)
            .ok_or(BufferError::CapacityOverflow { additional })?;

        self.bytes.try_reserve_exact(additional).map_err(|_| {
            BufferError::AllocationFailed { requested }
        })?;

        Ok(())
    }

    /// Resizes the buffer, zero-initializing newly added bytes.
    ///
    /// Shrinking truncates the buffer.
    pub fn try_resize_zeroed(
        &mut self,
        new_length: usize,
    ) -> BufferResult<()> {
        if new_length > self.len() {
            self.try_reserve(new_length - self.len())?;
        }

        self.bytes.resize(new_length, 0);
        Ok(())
    }

    /// Appends one byte.
    pub fn try_push(&mut self, byte: u8) -> BufferResult<()> {
        self.try_reserve(1)?;
        self.bytes.push(byte);
        Ok(())
    }

    /// Appends bytes from a borrowed slice.
    pub fn try_extend_from_slice(
        &mut self,
        source: &[u8],
    ) -> BufferResult<()> {
        self.try_reserve(source.len())?;
        self.bytes.extend_from_slice(source);
        Ok(())
    }

    /// Replaces the complete buffer contents.
    ///
    /// Allocation is completed before existing contents are modified.
    pub fn try_replace(
        &mut self,
        source: &[u8],
    ) -> BufferResult<()> {
        if source.len() > self.bytes.capacity() {
            let additional = source.len()
                .checked_sub(self.bytes.len())
                .ok_or(BufferError::CapacityOverflow {
                    additional: source.len(),
                })?;

            self.try_reserve_exact(additional)?;
        }

        self.bytes.clear();
        self.bytes.extend_from_slice(source);

        Ok(())
    }

    /// Removes and returns the final byte.
    pub fn pop(&mut self) -> Option<u8> {
        self.bytes.pop()
    }

    /// Truncates the buffer to the specified length.
    ///
    /// This operation never increases the buffer length.
    pub fn truncate(&mut self, length: usize) {
        self.bytes.truncate(length);
    }

    /// Removes all initialized bytes while retaining capacity.
    pub fn clear(&mut self) {
        self.bytes.clear();
    }

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

/// An immutable borrowed view over contiguous bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferView<'a> {
    bytes: &'a [u8],
}

impl<'a> BufferView<'a> {
    /// Creates a borrowed view without copying.
    #[must_use]
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    /// Returns the underlying bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Returns the number of bytes.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the view is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns a byte by index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&'a u8> {
        self.bytes.get(index)
    }

    /// Returns a checked immutable subview.
    pub fn slice(
        &self,
        range: Range<usize>,
    ) -> BufferResult<BufferView<'a>> {
        validate_range(range.start, range.end, self.len())?;

        match self.bytes.get(range.clone()) {
            Some(bytes) => Ok(BufferView::new(bytes)),
            None => Err(BufferError::InvalidRange {
                start: range.start,
                end: range.end,
            }.into()),
        }
    }

    /// Returns the underlying slice.
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

/// A mutable borrowed view over contiguous bytes.
#[derive(Debug, PartialEq, Eq)]
pub struct BufferViewMut<'a> {
    bytes: &'a mut [u8],
}

impl<'a> BufferViewMut<'a> {
    /// Creates a mutable view without allocating.
    #[must_use]
    pub fn new(bytes: &'a mut [u8]) -> Self {
        Self { bytes }
    }

    /// Returns an immutable view.
    #[must_use]
    pub fn as_view(&self) -> BufferView<'_> {
        BufferView::new(self.bytes)
    }

    /// Returns the underlying bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes
    }

    /// Returns the underlying bytes mutably.
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.bytes
    }

    /// Returns the number of bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the view is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns a byte by index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&u8> {
        self.bytes.get(index)
    }

    /// Returns a mutable byte by index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut u8> {
        self.bytes.get_mut(index)
    }

    /// Fills the view with a byte.
    pub fn fill(&mut self, value: u8) {
        self.bytes.fill(value);
    }

    /// Copies an exactly matching slice into this view.
    pub fn copy_from_slice(
        &mut self,
        source: &[u8],
    ) -> BufferResult<()> {
        if source.len() != self.len() {
            return Err(BufferError::LengthMismatch {
                expected: self.len(),
                actual: source.len(),
            }.into());
        }

        self.bytes.copy_from_slice(source);
        Ok(())
    }

    /// Returns a checked mutable subview.
    pub fn slice(
        &mut self,
        range: Range<usize>,
    ) -> BufferResult<BufferViewMut<'_>> {
        validate_range(range.start, range.end, self.len())?;

        let start = range.start;
        let end = range.end;

        match self.bytes.get_mut(range) {
            Some(bytes) => Ok(BufferViewMut::new(bytes)),
            None => Err(BufferError::InvalidRange {
                start,
                end,
            }.into()),
        }
    }
}

/// A segmented collection of owned byte buffers.
///
/// Each segment owns an independent allocation. The chain does not require
/// all bytes to occupy one contiguous address range.
///
/// Segment boundaries are preserved until an explicit flattening operation.
///
/// The total byte length is checked for arithmetic overflow.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct BufferChain {
    segments: Vec<Buffer>,
    total_len: usize,
}

impl BufferChain {
    /// Creates an empty buffer chain.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            segments: Vec::new(),
            total_len: 0,
        }
    }

    /// Returns the number of segments.
    #[must_use]
    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    /// Returns the total initialized byte count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.total_len
    }

    /// Returns whether the chain contains no bytes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.total_len == 0
    }

    /// Appends an existing owned buffer without copying its bytes.
    pub fn try_push(&mut self, segment: Buffer) -> BufferResult<()> {
        let new_total = self.total_len.checked_add(segment.len())
            .ok_or(BufferError::CapacityOverflow {
                additional: segment.len(),
            })?;

        self.segments.try_reserve(1).map_err(|_| {
            BufferError::AllocationFailed {
                requested: self.segments.len().saturating_add(1),
            }
        })?;

        self.segments.push(segment);
        self.total_len = new_total;

        Ok(())
    }

    /// Copies a byte slice into a new segment and appends it.
    pub fn try_push_slice(
        &mut self,
        bytes: &[u8],
    ) -> BufferResult<()> {
        let segment = Buffer::try_from_slice(bytes)?;
        self.try_push(segment)
    }

    /// Returns an iterator over borrowed segment slices.
    pub fn segments(&self) -> impl ExactSizeIterator<Item = &[u8]> {
        self.segments.iter().map(Buffer::as_bytes)
    }

    /// Returns the segment at the requested index.
    pub fn get_segment(
        &self,
        index: usize,
    ) -> BufferResult<BufferView<'_>> {
        match self.segments.get(index) {
            Some(segment) => Ok(segment.view()),
            None => Err(BufferError::SegmentNotFound {
                index,
                segment_count: self.segments.len(),
            }.into()),
        }
    }

    /// Flattens all segments into one contiguous buffer.
    ///
    /// This operation requires a contiguous allocation of the complete
    /// chain length. Use `segments()` when contiguous storage is unnecessary.
    pub fn try_flatten(&self) -> BufferResult<Buffer> {
        let mut result = Buffer::try_with_capacity(self.total_len)?;

        for segment in &self.segments {
            result.try_extend_from_slice(segment.as_bytes())?;
        }

        Ok(result)
    }

    /// Consumes the chain and returns its owned segments.
    #[must_use]
    pub fn into_segments(self) -> Vec<Buffer> {
        self.segments
    }
}

/// Validates a half-open byte range.
///
/// Empty ranges are valid, including a range at the end of the buffer.
fn validate_range(
    start: usize,
    end: usize,
    buffer_len: usize,
) -> std::result::Result<(), BufferError> {
    if start > end {
        return Err(BufferError::InvalidRange { start, end });
    }

    if start > buffer_len {
        return Err(BufferError::RangeStartOutOfBounds {
            start,
            buffer_len,
        });
    }

    if end > buffer_len {
        return Err(BufferError::RangeEndOutOfBounds {
            end,
            buffer_len,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_buffer_is_valid() {
        let buffer = Buffer::new();
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn vector_ownership_is_transferred() {
        let source = vec![1, 2, 3];
        let buffer = Buffer::from_vec(source);
        assert_eq!(buffer.as_bytes(), &[1, 2, 3]);
    }

    #[test]
    fn slice_copy_is_correct() {
        let buffer = Buffer::try_from_slice(&[1, 2, 3]).unwrap();
        assert_eq!(buffer.as_bytes(), &[1, 2, 3]);
    }

    #[test]
    fn zeroed_allocation_is_correct() {
        let buffer = Buffer::try_zeroed(16).unwrap();
        assert_eq!(buffer.len(), 16);
        assert!(buffer.as_bytes().iter().all(|byte| *byte == 0));
    }

    #[test]
    fn growth_initializes_new_bytes() {
        let mut buffer = Buffer::try_from_slice(&[1, 2]).unwrap();
        buffer.try_resize_zeroed(4).unwrap();
        assert_eq!(buffer.as_bytes(), &[1, 2, 0, 0]);
    }

    #[test]
    fn shrinking_truncates() {
        let mut buffer = Buffer::try_from_slice(&[1, 2, 3]).unwrap();
        buffer.try_resize_zeroed(1).unwrap();
        assert_eq!(buffer.as_bytes(), &[1]);
    }

    #[test]
    fn replacement_preserves_new_contents() {
        let mut buffer = Buffer::try_from_slice(&[1, 2]).unwrap();
        buffer.try_replace(&[7, 8, 9]).unwrap();
        assert_eq!(buffer.as_bytes(), &[7, 8, 9]);
    }

    #[test]
    fn replacement_can_shrink() {
        let mut buffer = Buffer::try_from_slice(&[1, 2, 3, 4]).unwrap();
        buffer.try_replace(&[9]).unwrap();
        assert_eq!(buffer.as_bytes(), &[9]);
    }

    #[test]
    fn immutable_view_is_zero_copy() {
        let source = [1, 2, 3];
        let view = BufferView::new(&source);
        assert_eq!(view.as_bytes(), &source);
    }

    #[test]
    fn immutable_view_slices_correctly() {
        let source = [1, 2, 3, 4];
        let view = BufferView::new(&source);
        assert_eq!(view.slice(1..3).unwrap().as_bytes(), &[2, 3]);
    }

    #[test]
    fn empty_end_range_is_valid() {
        let source = [1, 2, 3];
        let view = BufferView::new(&source);
        assert!(view.slice(3..3).unwrap().is_empty());
    }

    #[test]
    fn reversed_range_is_rejected() {
        let source = [1, 2, 3];
        let view = BufferView::new(&source);
        assert!(view.slice(2..1).is_err());
    }

    #[test]
    fn out_of_bounds_range_is_rejected() {
        let source = [1, 2, 3];
        let view = BufferView::new(&source);
        assert!(view.slice(0..4).is_err());
    }

    #[test]
    fn mutable_view_changes_original_storage() {
        let mut source = [1, 2, 3];

        {
            let mut view = BufferViewMut::new(&mut source);
            view.fill(8);
        }

        assert_eq!(source, [8, 8, 8]);
    }

    #[test]
    fn mutable_view_checks_copy_length() {
        let mut source = [0, 0];
        let mut view = BufferViewMut::new(&mut source);

        assert!(view.copy_from_slice(&[1, 2, 3]).is_err());
        assert_eq!(view.as_bytes(), &[0, 0]);
    }

    #[test]
    fn independent_copy_is_correct() {
        let original = Buffer::try_from_slice(&[1, 2]).unwrap();
        let mut copy = original.try_clone().unwrap();

        copy.try_push(3).unwrap();

        assert_eq!(original.as_bytes(), &[1, 2]);
        assert_eq!(copy.as_bytes(), &[1, 2, 3]);
    }

    #[test]
    fn chain_preserves_segments() {
        let mut chain = BufferChain::new();

        chain.try_push_slice(&[1, 2]).unwrap();
        chain.try_push_slice(&[3, 4]).unwrap();

        assert_eq!(chain.segment_count(), 2);
        assert_eq!(chain.len(), 4);
        assert_eq!(
            chain.segments().collect::<Vec<_>>(),
            vec![&[1, 2][..], &[3, 4][..]]
        );
    }

    #[test]
    fn chain_flattens_correctly() {
        let mut chain = BufferChain::new();

        chain.try_push_slice(&[1, 2]).unwrap();
        chain.try_push_slice(&[3, 4]).unwrap();

        assert_eq!(
            chain.try_flatten().unwrap().as_bytes(),
            &[1, 2, 3, 4]
        );
    }

    #[test]
    fn chain_reports_missing_segment() {
        let chain = BufferChain::new();
        assert!(chain.get_segment(0).is_err());
    }

    #[test]
    fn clear_retains_valid_empty_state() {
        let mut buffer = Buffer::try_from_slice(&[1, 2]).unwrap();
        buffer.clear();
        assert!(buffer.is_empty());
    }

    #[test]
    fn pop_removes_final_byte() {
        let mut buffer = Buffer::try_from_slice(&[1, 2]).unwrap();
        assert_eq!(buffer.pop(), Some(2));
        assert_eq!(buffer.as_bytes(), &[1]);
    }

    #[test]
    fn standard_error_preserves_cause() {
        let error = StdlibError::from(
            BufferError::LengthMismatch {
                expected: 4,
                actual: 3,
            }
        );

        assert_eq!(error.kind(), ErrorKind::InvalidInput);
        assert!(std::error::Error::source(&error).is_some());
    }
}
