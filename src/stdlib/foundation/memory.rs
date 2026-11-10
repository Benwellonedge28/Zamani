
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Foundation — Memory Abstractions
//!
//! Path: src/foundation/memory.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! # Purpose
//!
//! Provides domain-neutral, ownership-safe memory value abstractions
//! for the Zamani programming language.
//!
//! This module provides:
//!
//! - Generic owned memory buffers.
//! - Borrowed immutable and mutable memory views.
//! - Fallible buffer construction.
//! - Fallible memory growth.
//! - Fallible insertion and extension.
//! - Checked memory-footprint calculations.
//! - Structured memory-operation errors.
//!
//! # Architectural boundaries
//!
//! This module does not implement:
//!
//! - Physical memory allocation.
//! - Hardware memory management.
//! - Device memory placement.
//! - Runtime scheduling.
//! - Garbage collection.
//! - Ownership or borrow checking.
//! - Quantum memory.
//! - Tensor semantics.
//! - Physical addresses.
//!
//! Physical resource allocation belongs to the appropriate
//! runtime or allocation provider.
//!
//! Semantic ownership and borrowing belong to semantic analysis.
//!
//! Classical tensor storage remains owned by src/classical/.
//!
//! # Scalability
//!
//! No arbitrary maximum buffer size, element count, or capacity
//! is imposed by this module.
//!
//! Actual limits are determined by:
//!
//! - Available resources.
//! - Rust's Vec representation.
//! - The host address space.
//! - The platform allocator.
//! - Explicit external operational policies.
//!
//! No finite implementation guarantees literal infinite memory.
//!
//! # Safety
//!
//! This implementation uses safe Rust only.
//!
//! No raw pointers, unsafe blocks, unsafe traits or unsafe
//! memory operations are required.

use std::alloc::Layout;
use std::collections::TryReserveError;
use std::error::Error as StdError;
use std::fmt;
use std::mem::size_of;
use std::ops::{Deref, DerefMut};

/// Result type for foundational memory operations.
pub type MemoryResult<T> = Result<T, MemoryError>;


// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by foundational memory operations.
#[derive(Debug)]
pub enum MemoryError {

    /// The requested capacity could not be reserved.
    Allocation(TryReserveError),

    /// An element count or byte calculation overflowed.
    SizeOverflow {
        elements: usize,
        element_size: usize,
    },

    /// A requested index is outside the buffer.
    IndexOutOfBounds {
        index: usize,
        length: usize,
    },

    /// A requested range is invalid.
    InvalidRange {
        start: usize,
        end: usize,
        length: usize,
    },

    /// A requested capacity cannot be represented by the
    /// platform's allocation layout.
    InvalidLayout {
        elements: usize,
        element_size: usize,
    },
}

impl fmt::Display for MemoryError {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        match self {

            Self::Allocation(error) => {
                write!(
                    formatter,
                    "Memory reservation failed: {error}"
                )
            }

            Self::SizeOverflow {
                elements,
                element_size,
            } => {
                write!(
                    formatter,
                    "Memory size overflow: {elements} elements \
                     of {element_size} bytes"
                )
            }

            Self::IndexOutOfBounds {
                index,
                length,
            } => {
                write!(
                    formatter,
                    "Memory index {index} exceeds buffer length {length}"
                )
            }

            Self::InvalidRange {
                start,
                end,
                length,
            } => {
                write!(
                    formatter,
                    "Invalid memory range {start}..{end} \
                     for buffer length {length}"
                )
            }

            Self::InvalidLayout {
                elements,
                element_size,
            } => {
                write!(
                    formatter,
                    "Invalid allocation layout for {elements} \
                     elements of {element_size} bytes"
                )
            }
        }
    }
}

impl StdError for MemoryError {

    fn source(
        &self,
    ) -> Option<&(dyn StdError + 'static)> {

        match self {
            Self::Allocation(error) => Some(error),
            _ => None,
        }
    }
}

impl From<TryReserveError> for MemoryError {

    fn from(error: TryReserveError) -> Self {
        Self::Allocation(error)
    }
}


// ============================================================================
// MEMORY FOOTPRINT
// ============================================================================

/// Describes the memory footprint of a buffer.
///
/// Values are observations of the current buffer, not
/// resource guarantees or allocation budgets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryFootprint {

    /// Number of initialized elements.
    pub elements: usize,

    /// Number of bytes represented by one element.
    pub element_size: usize,

    /// Logical bytes occupied by initialized elements.
    pub logical_bytes: usize,

    /// Number of elements the buffer can hold without growth.
    pub capacity_elements: usize,

    /// Bytes represented by the buffer's element capacity.
    pub capacity_bytes: usize,
}

impl MemoryFootprint {

    /// Calculate a checked memory footprint.
    pub fn calculate<T>(
        elements: usize,
        capacity_elements: usize,
    ) -> MemoryResult<Self> {

        let element_size = size_of::<T>();

        let logical_bytes = elements
            .checked_mul(element_size)
            .ok_or(MemoryError::SizeOverflow {
                elements,
                element_size,
            })?;

        let capacity_bytes = capacity_elements
            .checked_mul(element_size)
            .ok_or(MemoryError::SizeOverflow {
                elements: capacity_elements,
                element_size,
            })?;

        Ok(Self {
            elements,
            element_size,
            logical_bytes,
            capacity_elements,
            capacity_bytes,
        })
    }
}


// ============================================================================
// BORROWED MEMORY VIEWS
// ============================================================================

/// An immutable borrowed view into existing memory.
///
/// This type does not own or allocate the underlying storage.
#[derive(Debug, Clone, Copy)]
pub struct MemoryView<'a, T> {

    data: &'a [T],
}

impl<'a, T> MemoryView<'a, T> {

    /// Create a view over an existing slice.
    #[must_use]
    pub const fn new(data: &'a [T]) -> Self {
        Self { data }
    }

    /// Return the number of elements.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.data.len()
    }

    /// Whether the view is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Return the underlying immutable slice.
    #[must_use]
    pub const fn as_slice(&self) -> &'a [T] {
        self.data
    }

    /// Access an element.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&'a T> {
        self.data.get(index)
    }

    /// Return a checked subview.
    pub fn slice(
        &self,
        start: usize,
        end: usize,
    ) -> MemoryResult<MemoryView<'a, T>> {

        let Some(data) = self.data.get(start..end) else {
            return Err(MemoryError::InvalidRange {
                start,
                end,
                length: self.data.len(),
            });
        };

        Ok(MemoryView::new(data))
    }

    /// Return an iterator over the elements.
    pub fn iter(&self) -> std::slice::Iter<'a, T> {
        self.data.iter()
    }
}

impl<T> Deref for MemoryView<'_, T> {

    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.data
    }
}


/// A mutable borrowed view into existing memory.
///
/// This type does not own or allocate the underlying storage.
#[derive(Debug)]
pub struct MemoryViewMut<'a, T> {

    data: &'a mut [T],
}

impl<'a, T> MemoryViewMut<'a, T> {

    /// Create a mutable view over an existing slice.
    #[must_use]
    pub fn new(data: &'a mut [T]) -> Self {
        Self { data }
    }

    /// Return the number of elements.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.data.len()
    }

    /// Whether the view is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Return the underlying mutable slice.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.data
    }

    /// Access an element mutably.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    /// Return a checked mutable subview.
    pub fn slice_mut(
        &mut self,
        start: usize,
        end: usize,
    ) -> MemoryResult<MemoryViewMut<'_, T>> {

        let length = self.data.len();

        let Some(data) = self.data.get_mut(start..end) else {
            return Err(MemoryError::InvalidRange {
                start,
                end,
                length,
            });
        };

        Ok(MemoryViewMut::new(data))
    }
}

impl<T> Deref for MemoryViewMut<'_, T> {

    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<T> DerefMut for MemoryViewMut<'_, T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}


// ============================================================================
// OWNED MEMORY BUFFER
// ============================================================================

/// An owned, dynamically sized, type-safe memory buffer.
///
/// The buffer owns its elements through Rust's Vec.
///
/// It provides fallible capacity management without introducing
/// universal memory limits.
///
/// This is a host-owned value container, not a physical device
/// allocator or a runtime memory manager.
///
/// The type intentionally does not implement Clone automatically.
/// Cloning potentially large buffers must be an explicit decision.
#[derive(Debug, PartialEq, Eq)]
pub struct MemoryBuffer<T> {

    storage: Vec<T>,
}

impl<T> MemoryBuffer<T> {

    /// Create an empty memory buffer.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            storage: Vec::new(),
        }
    }

    /// Create an empty buffer with fallibly reserved capacity.
    ///
    /// No fixed capacity ceiling is imposed.
    ///
    /// # Errors
    ///
    /// Returns an error if the requested reservation cannot
    /// be satisfied.
    pub fn try_with_capacity(
        capacity: usize,
    ) -> MemoryResult<Self> {

        let mut storage = Vec::new();

        storage.try_reserve_exact(capacity)?;

        Ok(Self { storage })
    }

    /// Construct a buffer from an existing owned vector.
    ///
    /// This transfers ownership without copying its elements.
    #[must_use]
    pub fn from_vec(storage: Vec<T>) -> Self {
        Self { storage }
    }

    /// Return the number of initialized elements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Whether the buffer contains no initialized elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    /// Return the current element capacity.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.storage.capacity()
    }

    /// Return the logical memory footprint.
    pub fn footprint(&self) -> MemoryResult<MemoryFootprint> {

        MemoryFootprint::calculate::<T>(
            self.storage.len(),
            self.storage.capacity(),
        )
    }

    /// Return the initialized elements as an immutable slice.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.storage.as_slice()
    }

    /// Return the initialized elements as a mutable slice.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.storage.as_mut_slice()
    }

    /// Create an immutable borrowed memory view.
    #[must_use]
    pub fn view(&self) -> MemoryView<'_, T> {
        MemoryView::new(self.storage.as_slice())
    }

    /// Create a mutable borrowed memory view.
    pub fn view_mut(&mut self) -> MemoryViewMut<'_, T> {
        MemoryViewMut::new(self.storage.as_mut_slice())
    }

    /// Access an element.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.storage.get(index)
    }

    /// Access an element mutably.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.storage.get_mut(index)
    }

    /// Iterate over initialized elements.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.storage.iter()
    }

    /// Iterate mutably over initialized elements.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.storage.iter_mut()
    }

    /// Reserve additional capacity.
    ///
    /// Existing elements are preserved.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> MemoryResult<()> {

        self.storage.try_reserve(additional)?;

        Ok(())
    }

    /// Reserve additional capacity without amortized growth.
    pub fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> MemoryResult<()> {

        self.storage.try_reserve_exact(additional)?;

        Ok(())
    }

    /// Add one element using fallible reservation.
    pub fn try_push(
        &mut self,
        value: T,
    ) -> MemoryResult<()> {

        self.storage.try_reserve(1)?;

        self.storage.push(value);

        Ok(())
    }

    /// Insert an element at a checked position.
    pub fn try_insert(
        &mut self,
        index: usize,
        value: T,
    ) -> MemoryResult<()> {

        let length = self.storage.len();

        if index > length {
            return Err(MemoryError::IndexOutOfBounds {
                index,
                length,
            });
        }

        self.storage.try_reserve(1)?;

        self.storage.insert(index, value);

        Ok(())
    }

    /// Remove an element at a checked position.
    pub fn try_remove(
        &mut self,
        index: usize,
    ) -> MemoryResult<T> {

        if index >= self.storage.len() {
            return Err(MemoryError::IndexOutOfBounds {
                index,
                length: self.storage.len(),
            });
        }

        Ok(self.storage.remove(index))
    }

    /// Remove an element without preserving ordering.
    pub fn try_swap_remove(
        &mut self,
        index: usize,
    ) -> MemoryResult<T> {

        if index >= self.storage.len() {
            return Err(MemoryError::IndexOutOfBounds {
                index,
                length: self.storage.len(),
            });
        }

        Ok(self.storage.swap_remove(index))
    }

    /// Extend the buffer from an iterator.
    ///
    /// Capacity is reserved before each insertion.
    ///
    /// If reservation fails after some values have been
    /// inserted, the successfully inserted prefix remains.
    ///
    /// This partial-progress behavior is intentional and
    /// documented rather than silently rolling back user data.
    pub fn try_extend<I>(
        &mut self,
        values: I,
    ) -> MemoryResult<()>
    where
        I: IntoIterator<Item = T>,
    {

        for value in values {

            self.storage.try_reserve(1)?;

            self.storage.push(value);
        }

        Ok(())
    }

    /// Extend from a borrowed slice.
    ///
    /// Elements are cloned into the buffer.
    pub fn try_extend_from_slice(
        &mut self,
        values: &[T],
    ) -> MemoryResult<()>
    where
        T: Clone,
    {

        self.storage.try_reserve(values.len())?;

        self.storage.extend_from_slice(values);

        Ok(())
    }

    /// Truncate the buffer to the specified length.
    ///
    /// This never increases the buffer's length.
    pub fn truncate(&mut self, length: usize) {
        self.storage.truncate(length);
    }

    /// Clear all initialized elements.
    ///
    /// Allocated capacity is retained.
    pub fn clear(&mut self) {
        self.storage.clear();
    }

    /// Split the buffer into two mutable non-overlapping views.
    pub fn split_at_mut(
        &mut self,
        index: usize,
    ) -> MemoryResult<(
        MemoryViewMut<'_, T>,
        MemoryViewMut<'_, T>,
    )> {

        let length = self.storage.len();

        let Some((left, right)) =
            self.storage.get_mut(..).map(|slice| {
                if index <= slice.len() {
                    let (left, right) = slice.split_at_mut(index);
                    Some((left, right))
                } else {
                    None
                }
            }).flatten()
        else {
            return Err(MemoryError::IndexOutOfBounds {
                index,
                length,
            });
        };

        Ok((
            MemoryViewMut::new(left),
            MemoryViewMut::new(right),
        ))
    }

    /// Consume the buffer and return its owned vector.
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.storage
    }

    /// Return the allocation layout for initialized element capacity.
    ///
    /// Zero-sized types are valid and require no element storage.
    pub fn layout(&self) -> MemoryResult<Layout> {

        Layout::array::<T>(self.storage.capacity())
            .map_err(|_| MemoryError::InvalidLayout {
                elements: self.storage.capacity(),
                element_size: size_of::<T>(),
            })
    }
}

impl<T: Clone> MemoryBuffer<T> {

    /// Create a buffer containing `length` copies of a value.
    ///
    /// Capacity reservation is fallible.
    pub fn try_filled(
        length: usize,
        value: T,
    ) -> MemoryResult<Self> {

        let mut buffer = Self::try_with_capacity(length)?;

        buffer.storage.resize(length, value);

        Ok(buffer)
    }

    /// Resize the buffer using a cloned value.
    ///
    /// Shrinking is infallible.
    /// Growing performs fallible reservation first.
    pub fn try_resize(
        &mut self,
        new_length: usize,
        value: T,
    ) -> MemoryResult<()> {

        if new_length > self.storage.len() {

            let additional =
                new_length - self.storage.len();

            self.storage.try_reserve(additional)?;
        }

        self.storage.resize(new_length, value);

        Ok(())
    }
}

impl<T> Default for MemoryBuffer<T> {

    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<Vec<T>> for MemoryBuffer<T> {

    fn from(storage: Vec<T>) -> Self {
        Self::from_vec(storage)
    }
}

impl<T> AsRef<[T]> for MemoryBuffer<T> {

    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> AsMut<[T]> for MemoryBuffer<T> {

    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}


// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn creates_empty_buffer() {

        let buffer = MemoryBuffer::<u8>::new();

        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn creates_buffer_with_capacity() {

        let buffer =
            MemoryBuffer::<u64>::try_with_capacity(128)
                .unwrap();

        assert!(buffer.capacity() >= 128);
        assert!(buffer.is_empty());
    }

    #[test]
    fn transfers_vector_ownership() {

        let values = vec![1, 2, 3];

        let buffer = MemoryBuffer::from_vec(values);

        assert_eq!(buffer.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn supports_dynamic_growth() {

        let mut buffer = MemoryBuffer::new();

        for value in 0..1024 {
            buffer.try_push(value).unwrap();
        }

        assert_eq!(buffer.len(), 1024);
        assert_eq!(buffer.get(1023), Some(&1023));
    }

    #[test]
    fn supports_extension() {

        let mut buffer = MemoryBuffer::new();

        buffer.try_extend(0..10).unwrap();

        assert_eq!(
            buffer.as_slice(),
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn supports_slice_extension() {

        let mut buffer = MemoryBuffer::new();

        buffer
            .try_extend_from_slice(&[1, 2, 3])
            .unwrap();

        assert_eq!(buffer.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn supports_checked_insertion() {

        let mut buffer =
            MemoryBuffer::from_vec(vec![1, 3]);

        buffer.try_insert(1, 2).unwrap();

        assert_eq!(buffer.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn rejects_invalid_insertion() {

        let mut buffer =
            MemoryBuffer::from_vec(vec![1, 2]);

        assert!(matches!(
            buffer.try_insert(4, 3),
            Err(MemoryError::IndexOutOfBounds { .. })
        ));
    }

    #[test]
    fn supports_removal() {

        let mut buffer =
            MemoryBuffer::from_vec(vec![1, 2, 3]);

        assert_eq!(buffer.try_remove(1).unwrap(), 2);

        assert_eq!(buffer.as_slice(), &[1, 3]);
    }

    #[test]
    fn supports_resize() {

        let mut buffer =
            MemoryBuffer::from_vec(vec![1, 2]);

        buffer.try_resize(5, 9).unwrap();

        assert_eq!(
            buffer.as_slice(),
            &[1, 2, 9, 9, 9]
        );

        buffer.try_resize(1, 0).unwrap();

        assert_eq!(buffer.as_slice(), &[1]);
    }

    #[test]
    fn supports_borrowed_views() {

        let buffer =
            MemoryBuffer::from_vec(vec![10, 20, 30]);

        let view = buffer.view();

        assert_eq!(view.len(), 3);
        assert_eq!(view.get(1), Some(&20));
    }

    #[test]
    fn supports_mutable_views() {

        let mut buffer =
            MemoryBuffer::from_vec(vec![10, 20, 30]);

        {
            let mut view = buffer.view_mut();

            *view.get_mut(1).unwrap() = 99;
        }

        assert_eq!(
            buffer.as_slice(),
            &[10, 99, 30]
        );
    }

    #[test]
    fn supports_non_overlapping_mutable_splits() {

        let mut buffer =
            MemoryBuffer::from_vec(vec![1, 2, 3, 4]);

        {
            let (mut left, mut right) =
                buffer.split_at_mut(2).unwrap();

            left.as_mut_slice()[0] = 10;
            right.as_mut_slice()[0] = 30;
        }

        assert_eq!(
            buffer.as_slice(),
            &[10, 2, 30, 4]
        );
    }

    #[test]
    fn rejects_invalid_views() {

        let buffer =
            MemoryBuffer::from_vec(vec![1, 2, 3]);

        assert!(matches!(
            buffer.view().slice(0, 10),
            Err(MemoryError::InvalidRange { .. })
        ));
    }

    #[test]
    fn reports_memory_footprint() {

        let buffer =
            MemoryBuffer::from_vec(vec![1_u64, 2, 3]);

        let footprint = buffer.footprint().unwrap();

        assert_eq!(footprint.elements, 3);
        assert_eq!(footprint.element_size, 8);
        assert_eq!(footprint.logical_bytes, 24);
        assert!(footprint.capacity_elements >= 3);
    }

    #[test]
    fn supports_zero_sized_types() {

        let buffer =
            MemoryBuffer::from_vec(vec![(); 100]);

        let footprint = buffer.footprint().unwrap();

        assert_eq!(footprint.elements, 100);
        assert_eq!(footprint.logical_bytes, 0);
    }

    #[test]
    fn supports_large_logical_metadata_without_allocation() {

        let footprint =
            MemoryFootprint::calculate::<u8>(
                usize::MAX,
                usize::MAX,
            )
            .unwrap();

        assert_eq!(
            footprint.logical_bytes,
            usize::MAX
        );
    }

    #[test]
    fn rejects_footprint_overflow() {

        assert!(matches!(
            MemoryFootprint::calculate::<u64>(
                usize::MAX,
                usize::MAX,
            ),
            Err(MemoryError::SizeOverflow { .. })
        ));
    }

    #[test]
    fn preserves_contents_after_failed_index_operation() {

        let mut buffer =
            MemoryBuffer::from_vec(vec![1, 2, 3]);

        let result = buffer.try_remove(10);

        assert!(result.is_err());

        assert_eq!(
            buffer.as_slice(),
            &[1, 2, 3]
        );
    }

    #[test]
    fn retains_capacity_after_clear() {

        let mut buffer =
            MemoryBuffer::try_with_capacity(100)
                .unwrap();

        buffer.try_push(42).unwrap();

        let capacity = buffer.capacity();

        buffer.clear();

        assert!(buffer.is_empty());
        assert_eq!(buffer.capacity(), capacity);
    }

    #[test]
    fn consumes_buffer_without_copying_elements() {

        let buffer =
            MemoryBuffer::from_vec(vec![1, 2, 3]);

        let values = buffer.into_vec();

        assert_eq!(values, vec![1, 2, 3]);
    }
}
