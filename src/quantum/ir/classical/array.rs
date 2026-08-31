//! Zamani Quantum IR — Classical Array Model
//!
//! Production-grade, hardware-independent classical array semantics for the
//! canonical Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `quantum::ir::classical::array` owns the semantic representation of
//! classical arrays used by quantum programs.
//!
//! It provides:
//!
//! - logical classical-array identity and shape;
//! - dynamically sized and statically sized arrays;
//! - one-dimensional array indexing;
//! - multidimensional array shape metadata;
//! - checked index calculations;
//! - deterministic sparse classical-array storage;
//! - dense storage helpers;
//! - lazy array index iteration;
//! - array slicing/ranges;
//! - array membership and bounds validation;
//! - array shape/size validation;
//! - deterministic structural equality;
//! - array-level resource accounting;
//! - explicit validation-policy integration.
//!
//! It does NOT own:
//!
//! - classical scalar value semantics;
//! - classical expression evaluation;
//! - classical type definitions;
//! - quantum-bit identity;
//! - quantum registers;
//! - gate semantics;
//! - measurement semantics;
//! - control-flow semantics;
//! - pulse semantics;
//! - scheduling;
//! - routing;
//! - hardware memory;
//! - CPU memory addresses;
//! - simulator state;
//! - backend execution;
//! - frontend parsing.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Canonical dependency direction
//!
//! ```text
//! quantum::ir::qubit
//!         │
//!         │ independent identity vocabulary
//!         ▼
//! classical::array
//!         │
//!         ├── classical values
//!         ├── expressions
//!         ├── types
//!         ├── operations
//!         ├── validation
//!         ├── serialization
//!         └── analysis
//! ```
//!
//! `array.rs` intentionally does not import `QubitId` because classical array
//! semantics must not confuse a classical array index with a quantum-qubit
//! identity.
//!
//! When an operation needs both a classical array and quantum operands, the
//! operation layer combines:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! with the classical-array references defined here.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once and may be compiled for:
//!
//! - a tiny quantum processor;
//! - a large quantum processor;
//! - a simulator;
//! - a fault-tolerant machine;
//! - a distributed quantum system;
//! - a future quantum architecture.
//!
//! Therefore this file contains no architectural array-size ceiling.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_ARRAY_SIZE = 64
//! MAX_ARRAY_ELEMENTS = 4096
//! MAX_CLASSICAL_BITS = 1_000_000
//! ```
//!
//! Array dimensions and lengths are program data.
//!
//! Concrete limits belong to an explicit compilation/security/resource policy.
//!
//! # Important scalability distinction
//!
//! An array *description* must not necessarily allocate storage for every
//! element.
//!
//! For example:
//!
//! ```text
//! array<bool, 1_000_000_000>
//! ```
//!
//! can be represented by shape metadata without allocating one billion Rust
//! values.
//!
//! This file therefore distinguishes:
//!
//! ```text
//! ClassicalArrayShape
//!     semantic dimensions
//!
//! ClassicalArray
//!     logical array container
//!
//! ClassicalArrayStorage
//!     optional concrete storage representation
//!
//! ClassicalArrayIndex
//!     checked logical index
//!
//! ClassicalArraySlice
//!     logical subrange
//! ```
//!
//! # Memory model
//!
//! This module does not promise that an arbitrarily large concrete array can
//! be materialized in memory. No finite software representation can do that.
//!
//! Instead:
//!
//! - metadata can remain compact;
//! - sparse storage stores only present elements;
//! - dense storage requires explicit allocation by the caller;
//! - validation can apply an external resource policy;
//! - index calculations are checked;
//! - allocation is never performed implicitly by metadata constructors.
//!
//! This is the correct meaning of "scale from atom to everywhere": the
//! semantic model has no artificial machine-size ceiling, while actual
//! materialization remains constrained by available resources.
//!
//! # Determinism
//!
//! Sparse storage uses `BTreeMap` rather than `HashMap` so that:
//!
//! - iteration is deterministic;
//! - structural serialization can be deterministic;
//! - structural hashing can be deterministic;
//! - compilation is reproducible;
//! - tests do not depend on randomized hash ordering.
//!
//! # Integer safety
//!
//! All multiplication and addition involved in multidimensional indexing is
//! checked for overflow.
//!
//! An overflowing shape or index calculation returns an explicit error rather
//! than wrapping.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition.
//!
//! Requirements:
//!
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe` code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! `classical/value.rs`
//!     supplies the semantic classical element value type.
//!
//! `classical.rs`
//!     owns the broader classical namespace and classical resource model.
//!
//! `types.rs`
//!     owns canonical array type definitions.
//!
//! `operation.rs`
//!     may use array references as operation operands/results.
//!
//! `expression.rs`
//!     may represent array indexing and slicing expressions.
//!
//! `assignment.rs`
//!     may use array references as assignment destinations.
//!
//! `predicate.rs`
//!     may consume array elements or slices.
//!
//! `measurement.rs`
//!     may write measurement results into classical arrays.
//!
//! `validation.rs`
//!     validates array shape, bounds and resource requirements.
//!
//! `analysis.rs`
//!     may inspect array dimensions and resource usage.
//!
//! `serialization.rs`
//!     serializes deterministic array metadata/storage.
//!
//! `hash.rs`
//!     may hash the structural representation.
//!
//! `program.rs`
//!     owns the lifetime and declaration scope of arrays.
//!
//! `quantum::ir::qubit`
//!     remains the canonical owner of quantum-qubit identity. This file does
//!     not duplicate or redefine `QubitId`.
//!
//! # Ownership rule
//!
//! This file owns array semantics only.
//!
//! It does not become a second type-system implementation.
//!
//! In particular, a future canonical `ClassicalValue` implementation should
//! be referenced by the higher classical layer rather than duplicated here.
//!
//! The generic `ClassicalArray<T>` below therefore models array structure and
//! storage without forcing one particular scalar-value implementation into
//! this low-level container.
//!
//! # API stability rule
//!
//! Public constructors validate their invariants immediately where practical.
//!
//! Methods that can fail because of:
//!
//! - integer overflow;
//! - out-of-bounds access;
//! - invalid dimensionality;
//! - invalid slicing;
//!
//! return explicit errors.
//!
//! No method silently wraps, truncates or ignores invalid data.
//!
//! # No implicit execution
//!
//! This file does not execute classical expressions.
//!
//! It only represents array structure and, when requested by a caller, stores
//! already-computed values.
//!
//! =============================================================================

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::fmt;
use std::ops::Range;

// =============================================================================
// Errors
// =============================================================================

/// Error returned by classical-array construction, indexing, slicing or
/// validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalArrayError {
    /// The array has no dimensions when at least one dimension is required.
    EmptyShape,

    /// A dimension has an invalid size for the requested operation.
    InvalidDimension,

    /// Shape multiplication overflowed `usize`.
    ShapeSizeOverflow,

    /// A flattened index calculation overflowed `usize`.
    IndexOverflow,

    /// The supplied index has the wrong number of dimensions.
    RankMismatch {
        /// Number of indices supplied.
        expected: usize,

        /// Number of indices received.
        actual: usize,
    },

    /// An index lies outside the array.
    IndexOutOfBounds {
        /// Dimension containing the invalid index.
        dimension: usize,

        /// Requested index.
        index: usize,

        /// Exclusive upper bound.
        bound: usize,
    },

    /// A one-dimensional range is invalid.
    InvalidRange {
        /// Inclusive start.
        start: usize,

        /// Exclusive end.
        end: usize,

        /// Dimension length.
        length: usize,
    },

    /// A slice has the wrong rank.
    SliceRankMismatch {
        /// Expected rank.
        expected: usize,

        /// Actual rank.
        actual: usize,
    },

    /// A sparse value is outside the declared shape.
    SparseIndexOutOfBounds {
        /// Flattened index.
        index: usize,

        /// Number of elements represented by the shape.
        size: usize,
    },

    /// An array storage representation does not agree with its shape.
    StorageShapeMismatch,

    /// A requested operation would require materializing more elements than
    /// the supplied resource policy permits.
    ResourceLimitExceeded {
        /// Requested element count.
        requested: usize,

        /// Permitted element count.
        limit: usize,
    },
}

impl fmt::Display for ClassicalArrayError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyShape => {
                formatter.write_str("classical array shape cannot be empty")
            }

            Self::InvalidDimension => {
                formatter.write_str("classical array dimension is invalid")
            }

            Self::ShapeSizeOverflow => {
                formatter.write_str(
                    "classical array shape size overflowed the host index type",
                )
            }

            Self::IndexOverflow => {
                formatter.write_str(
                    "classical array index calculation overflowed the host index type",
                )
            }

            Self::RankMismatch { expected, actual } => {
                write!(
                    formatter,
                    "array rank mismatch: expected {expected}, found {actual}"
                )
            }

            Self::IndexOutOfBounds {
                dimension,
                index,
                bound,
            } => {
                write!(
                    formatter,
                    "array index {index} is out of bounds in dimension \
                     {dimension}; dimension length is {bound}"
                )
            }

            Self::InvalidRange {
                start,
                end,
                length,
            } => {
                write!(
                    formatter,
                    "invalid array range {start}..{end} for dimension \
                     length {length}"
                )
            }

            Self::SliceRankMismatch { expected, actual } => {
                write!(
                    formatter,
                    "array slice rank mismatch: expected {expected}, found {actual}"
                )
            }

            Self::SparseIndexOutOfBounds { index, size } => {
                write!(
                    formatter,
                    "sparse array index {index} is outside array size {size}"
                )
            }

            Self::StorageShapeMismatch => {
                formatter.write_str(
                    "array storage is incompatible with the declared shape",
                )
            }

            Self::ResourceLimitExceeded { requested, limit } => {
                write!(
                    formatter,
                    "array operation requests {requested} elements, \
                     exceeding resource policy limit {limit}"
                )
            }
        }
    }
}

impl std::error::Error for ClassicalArrayError {}

// =============================================================================
// Array index
// =============================================================================

/// A validated flattened classical-array index.
///
/// A flattened index is an implementation-neutral logical position inside an
/// array's storage domain.
///
/// It does not represent:
///
/// - a memory address;
/// - a CPU address;
/// - a hardware register;
/// - a physical quantum resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalArrayIndex(usize);

impl ClassicalArrayIndex {
    /// Creates an index without validating it against a particular shape.
    ///
    /// This constructor is useful when the index will subsequently be checked
    /// against an array shape.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the underlying logical index.
    #[must_use]
    pub const fn value(self) -> usize {
        self.0
    }

    /// Validates this index against an array size.
    pub fn validate(
        self,
        size: usize,
    ) -> Result<(), ClassicalArrayError> {
        if self.0 < size {
            Ok(())
        } else {
            Err(ClassicalArrayError::SparseIndexOutOfBounds {
                index: self.0,
                size,
            })
        }
    }
}

impl From<usize> for ClassicalArrayIndex {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl From<ClassicalArrayIndex> for usize {
    fn from(value: ClassicalArrayIndex) -> Self {
        value.value()
    }
}

impl fmt::Display for ClassicalArrayIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// =============================================================================
// Shape
// =============================================================================

/// Immutable logical shape of a classical array.
///
/// A shape contains one or more dimensions.
///
/// Examples:
///
/// ```text
/// [4]       -> four elements
/// [3, 4]    -> twelve elements
/// [2, 3, 4] -> twenty-four elements
/// ```
///
/// The shape stores only dimension metadata. It does not allocate element
/// storage.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalArrayShape {
    dimensions: Vec<usize>,
}

impl ClassicalArrayShape {
    /// Creates a shape from explicit dimensions.
    ///
    /// Empty shapes are rejected because this type represents arrays rather
    /// than scalar values.
    pub fn new<I>(
        dimensions: I,
    ) -> Result<Self, ClassicalArrayError>
    where
        I: IntoIterator<Item = usize>,
    {
        let dimensions: Vec<usize> = dimensions.into_iter().collect();

        if dimensions.is_empty() {
            return Err(ClassicalArrayError::EmptyShape);
        }

        for &dimension in &dimensions {
            if dimension == 0 {
                return Err(ClassicalArrayError::InvalidDimension);
            }
        }

        let mut size = 1usize;

        for &dimension in &dimensions {
            size = size
                .checked_mul(dimension)
                .ok_or(ClassicalArrayError::ShapeSizeOverflow)?;
        }

        Ok(Self { dimensions })
    }

    /// Creates a one-dimensional shape.
    pub fn one_dimensional(
        length: usize,
    ) -> Result<Self, ClassicalArrayError> {
        Self::new([length])
    }

    /// Returns the number of dimensions.
    #[must_use]
    pub fn rank(&self) -> usize {
        self.dimensions.len()
    }

    /// Returns the dimensions.
    #[must_use]
    pub fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }

    /// Returns one dimension by position.
    #[must_use]
    pub fn dimension(&self, axis: usize) -> Option<usize> {
        self.dimensions.get(axis).copied()
    }

    /// Returns the total number of logical elements.
    ///
    /// Construction guarantees this calculation cannot overflow.
    #[must_use]
    pub fn size(&self) -> usize {
        self.dimensions
            .iter()
            .copied()
            .fold(1usize, |accumulator, dimension| {
                accumulator * dimension
            })
    }

    /// Returns whether this shape represents a one-dimensional array.
    #[must_use]
    pub fn is_one_dimensional(&self) -> bool {
        self.rank() == 1
    }

    /// Validates the shape invariants.
    pub fn validate(&self) -> Result<(), ClassicalArrayError> {
        if self.dimensions.is_empty() {
            return Err(ClassicalArrayError::EmptyShape);
        }

        let mut size = 1usize;

        for &dimension in &self.dimensions {
            if dimension == 0 {
                return Err(ClassicalArrayError::InvalidDimension);
            }

            size = size
                .checked_mul(dimension)
                .ok_or(ClassicalArrayError::ShapeSizeOverflow)?;
        }

        let _ = size;

        Ok(())
    }

    /// Converts a multidimensional index into a flattened row-major index.
    ///
    /// The operation is fully checked and never wraps.
    pub fn flatten(
        &self,
        indices: &[usize],
    ) -> Result<ClassicalArrayIndex, ClassicalArrayError> {
        if indices.len() != self.rank() {
            return Err(ClassicalArrayError::RankMismatch {
                expected: self.rank(),
                actual: indices.len(),
            });
        }

        let mut flat = 0usize;

        for (axis, (&index, &dimension)) in
            indices.iter().zip(self.dimensions.iter()).enumerate()
        {
            if index >= dimension {
                return Err(ClassicalArrayError::IndexOutOfBounds {
                    dimension: axis,
                    index,
                    bound: dimension,
                });
            }

            flat = flat
                .checked_mul(dimension)
                .ok_or(ClassicalArrayError::IndexOverflow)?;

            flat = flat
                .checked_add(index)
                .ok_or(ClassicalArrayError::IndexOverflow)?;
        }

        Ok(ClassicalArrayIndex::new(flat))
    }

    /// Converts a flattened index to a multidimensional row-major index.
    pub fn unflatten(
        &self,
        index: ClassicalArrayIndex,
    ) -> Result<Vec<usize>, ClassicalArrayError> {
        index.validate(self.size())?;

        let mut remaining = index.value();
        let mut indices = vec![0usize; self.rank()];

        for axis in (0..self.rank()).rev() {
            let dimension = self.dimensions[axis];

            indices[axis] = remaining % dimension;
            remaining /= dimension;
        }

        Ok(indices)
    }

    /// Returns a lazy flattened-index iterator.
    #[must_use]
    pub fn indices(&self) -> ClassicalArrayIndexIter {
        ClassicalArrayIndexIter {
            next: 0,
            remaining: self.size(),
        }
    }
}

// =============================================================================
// Index iterator
// =============================================================================

/// Lazy iterator over flattened array indices.
///
/// It does not allocate one index object per element.
#[derive(Debug, Clone)]
pub struct ClassicalArrayIndexIter {
    next: usize,
    remaining: usize,
}

impl Iterator for ClassicalArrayIndexIter {
    type Item = ClassicalArrayIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let index = ClassicalArrayIndex::new(self.next);

        self.next = self.next.checked_add(1)?;
        self.remaining -= 1;

        Some(index)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for ClassicalArrayIndexIter {}

impl std::iter::FusedIterator for ClassicalArrayIndexIter {}

// =============================================================================
// One-dimensional slice
// =============================================================================

/// A checked one-dimensional classical-array slice.
///
/// The slice uses half-open semantics:
///
/// ```text
/// start..end
/// ```
///
/// where `start` is included and `end` is excluded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalArraySlice {
    start: usize,
    end: usize,
}

impl ClassicalArraySlice {
    /// Creates a slice without associating it with an array.
    ///
    /// Bounds are checked for `start <= end`; the array-specific upper bound
    /// is checked by [`Self::validate_against`].
    pub fn new(
        start: usize,
        end: usize,
    ) -> Result<Self, ClassicalArrayError> {
        if start > end {
            return Err(ClassicalArrayError::InvalidRange {
                start,
                end,
                length: 0,
            });
        }

        Ok(Self { start, end })
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

    /// Returns the slice length.
    #[must_use]
    pub const fn len(self) -> usize {
        self.end - self.start
    }

    /// Returns whether the slice is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// Validates the slice against an array dimension.
    pub fn validate_against(
        self,
        length: usize,
    ) -> Result<(), ClassicalArrayError> {
        if self.end <= length {
            Ok(())
        } else {
            Err(ClassicalArrayError::InvalidRange {
                start: self.start,
                end: self.end,
                length,
            })
        }
    }

    /// Returns the corresponding Rust range after validation.
    pub fn range(
        self,
        length: usize,
    ) -> Result<Range<usize>, ClassicalArrayError> {
        self.validate_against(length)?;
        Ok(self.start..self.end)
    }
}

impl From<Range<usize>> for ClassicalArraySlice {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

// =============================================================================
// Resource policy
// =============================================================================

/// Explicit policy controlling operations that materialize classical-array
/// elements.
///
/// This is a resource/security policy and is NOT a semantic array-size limit.
///
/// A compiler or runtime may construct a different policy appropriate to its
/// available resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassicalArrayResourcePolicy {
    /// Maximum number of elements an operation may materialize.
    ///
    /// `None` means that this particular policy imposes no additional
    /// materialization limit.
    pub max_materialized_elements: Option<usize>,
}

impl Default for ClassicalArrayResourcePolicy {
    fn default() -> Self {
        Self {
            max_materialized_elements: None,
        }
    }
}

impl ClassicalArrayResourcePolicy {
    /// Creates an unrestricted policy.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_materialized_elements: None,
        }
    }

    /// Creates a policy with an explicit materialization limit.
    #[must_use]
    pub const fn with_max_materialized_elements(
        maximum: usize,
    ) -> Self {
        Self {
            max_materialized_elements: Some(maximum),
        }
    }

    /// Checks an element count against the policy.
    pub fn check_materialization(
        &self,
        requested: usize,
    ) -> Result<(), ClassicalArrayError> {
        if let Some(limit) = self.max_materialized_elements {
            if requested > limit {
                return Err(ClassicalArrayError::ResourceLimitExceeded {
                    requested,
                    limit,
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Sparse storage
// =============================================================================

/// Deterministic sparse storage for classical-array elements.
///
/// Only explicitly stored elements consume storage.
///
/// `BTreeMap` guarantees deterministic iteration order.
///
/// The value type is generic so this low-level container does not duplicate
/// the canonical `ClassicalValue` definition owned by the classical value
/// layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalSparseArray<T> {
    shape: ClassicalArrayShape,
    elements: BTreeMap<ClassicalArrayIndex, T>,
}

impl<T> ClassicalSparseArray<T> {
    /// Creates an empty sparse array with the supplied shape.
    pub fn new(
        shape: ClassicalArrayShape,
    ) -> Result<Self, ClassicalArrayError> {
        shape.validate()?;

        Ok(Self {
            shape,
            elements: BTreeMap::new(),
        })
    }

    /// Creates a sparse array from flattened index/value pairs.
    ///
    /// Every index is checked against the declared shape.
    pub fn from_elements<I>(
        shape: ClassicalArrayShape,
        elements: I,
    ) -> Result<Self, ClassicalArrayError>
    where
        I: IntoIterator<Item = (ClassicalArrayIndex, T)>,
    {
        shape.validate()?;

        let mut storage = Self {
            shape,
            elements: BTreeMap::new(),
        };

        for (index, value) in elements {
            storage.insert(index, value)?;
        }

        Ok(storage)
    }

    /// Returns the array shape.
    #[must_use]
    pub fn shape(&self) -> &ClassicalArrayShape {
        &self.shape
    }

    /// Returns the number of explicitly stored elements.
    #[must_use]
    pub fn stored_len(&self) -> usize {
        self.elements.len()
    }

    /// Returns whether no values are explicitly stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Inserts or replaces a value.
    pub fn insert(
        &mut self,
        index: ClassicalArrayIndex,
        value: T,
    ) -> Result<Option<T>, ClassicalArrayError> {
        index.validate(self.shape.size())?;

        Ok(self.elements.insert(index, value))
    }

    /// Returns an element by flattened index.
    #[must_use]
    pub fn get(
        &self,
        index: ClassicalArrayIndex,
    ) -> Option<&T> {
        self.elements.get(&index)
    }

    /// Returns a mutable element by flattened index.
    #[must_use]
    pub fn get_mut(
        &mut self,
        index: ClassicalArrayIndex,
    ) -> Option<&mut T> {
        self.elements.get_mut(&index)
    }

    /// Removes an explicitly stored element.
    pub fn remove(
        &mut self,
        index: ClassicalArrayIndex,
    ) -> Option<T> {
        self.elements.remove(&index)
    }

    /// Returns whether an element is explicitly stored.
    #[must_use]
    pub fn contains(
        &self,
        index: ClassicalArrayIndex,
    ) -> bool {
        self.elements.contains_key(&index)
    }

    /// Removes every explicitly stored element.
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    /// Returns deterministic iteration over stored elements.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&ClassicalArrayIndex, &T)> {
        self.elements.iter()
    }

    /// Returns deterministic mutable iteration over stored elements.
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (&ClassicalArrayIndex, &mut T)> {
        self.elements.iter_mut()
    }

    /// Consumes the storage and returns its deterministic map.
    #[must_use]
    pub fn into_elements(
        self,
    ) -> BTreeMap<ClassicalArrayIndex, T> {
        self.elements
    }

    /// Validates all stored indices against the shape.
    pub fn validate(&self) -> Result<(), ClassicalArrayError> {
        self.shape.validate()?;

        let size = self.shape.size();

        for index in self.elements.keys() {
            index.validate(size)?;
        }

        Ok(())
    }
}

// =============================================================================
// Dense storage
// =============================================================================

/// Dense classical-array storage.
///
/// Construction requires explicit element materialization.
///
/// This type never allocates implicitly from a shape alone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalDenseArray<T> {
    shape: ClassicalArrayShape,
    elements: Vec<T>,
}

impl<T> ClassicalDenseArray<T> {
    /// Creates dense storage from an already materialized vector.
    pub fn from_vec(
        shape: ClassicalArrayShape,
        elements: Vec<T>,
    ) -> Result<Self, ClassicalArrayError> {
        shape.validate()?;

        if elements.len() != shape.size() {
            return Err(ClassicalArrayError::StorageShapeMismatch);
        }

        Ok(Self { shape, elements })
    }

    /// Creates dense storage by repeatedly invoking `factory`.
    ///
    /// The resource policy is checked before allocation begins.
    pub fn try_generate<F>(
        shape: ClassicalArrayShape,
        policy: ClassicalArrayResourcePolicy,
        mut factory: F,
    ) -> Result<Self, ClassicalArrayError>
    where
        F: FnMut(ClassicalArrayIndex) -> T,
    {
        shape.validate()?;

        let size = shape.size();

        policy.check_materialization(size)?;

        let mut elements = Vec::new();

        elements
            .try_reserve_exact(size)
            .map_err(|_| ClassicalArrayError::ResourceLimitExceeded {
                requested: size,
                limit: elements.capacity(),
            })?;

        for index in shape.indices() {
            elements.push(factory(index));
        }

        Ok(Self { shape, elements })
    }

    /// Returns the shape.
    #[must_use]
    pub fn shape(&self) -> &ClassicalArrayShape {
        &self.shape
    }

    /// Returns the number of elements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns whether the storage is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns an element by flattened index.
    #[must_use]
    pub fn get(
        &self,
        index: ClassicalArrayIndex,
    ) -> Option<&T> {
        self.elements.get(index.value())
    }

    /// Returns a mutable element by flattened index.
    #[must_use]
    pub fn get_mut(
        &mut self,
        index: ClassicalArrayIndex,
    ) -> Option<&mut T> {
        self.elements.get_mut(index.value())
    }

    /// Returns the underlying element slice.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.elements
    }

    /// Returns the underlying mutable element slice.
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.elements
    }

    /// Consumes the dense storage and returns its vector.
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.elements
    }

    /// Returns deterministic iteration over all elements.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &T> {
        self.elements.iter()
    }

    /// Returns deterministic mutable iteration over all elements.
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut T> {
        self.elements.iter_mut()
    }
}

// =============================================================================
// Unified array storage
// =============================================================================

/// Concrete classical-array storage representation.
///
/// The semantic array shape is stored once, while the storage representation
/// is selected explicitly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalArrayStorage<T> {
    /// No concrete element storage.
    ///
    /// Useful for declarations and symbolic/resource-only IR.
    Unmaterialized,

    /// Deterministic sparse storage.
    Sparse(ClassicalSparseArray<T>),

    /// Dense storage.
    Dense(ClassicalDenseArray<T>),
}

impl<T> ClassicalArrayStorage<T> {
    /// Returns whether concrete elements are materialized.
    #[must_use]
    pub fn is_materialized(&self) -> bool {
        !matches!(self, Self::Unmaterialized)
    }

    /// Returns the number of explicitly materialized elements.
    #[must_use]
    pub fn materialized_len(&self) -> usize {
        match self {
            Self::Unmaterialized => 0,
            Self::Sparse(storage) => storage.stored_len(),
            Self::Dense(storage) => storage.len(),
        }
    }

    /// Returns whether storage is dense.
    #[must_use]
    pub fn is_dense(&self) -> bool {
        matches!(self, Self::Dense(_))
    }

    /// Returns whether storage is sparse.
    #[must_use]
    pub fn is_sparse(&self) -> bool {
        matches!(self, Self::Sparse(_))
    }

    /// Returns whether storage is unmaterialized.
    #[must_use]
    pub fn is_unmaterialized(&self) -> bool {
        matches!(self, Self::Unmaterialized)
    }
}

// =============================================================================
// Classical array
// =============================================================================

/// Canonical logical classical array.
///
/// `ClassicalArray<T>` owns the semantic relationship between an array shape
/// and its optional storage representation.
///
/// It does not own the canonical scalar value type. The generic `T` is
/// intentionally supplied by the surrounding classical value layer.
///
/// Typical integration:
///
/// ```text
/// ClassicalArray<ClassicalValue>
/// ```
///
/// The type can also be used by lower-level infrastructure with another
/// semantically valid element representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalArray<T> {
    shape: ClassicalArrayShape,
    storage: ClassicalArrayStorage<T>,
}

impl<T> ClassicalArray<T> {
    /// Creates a declaration-only array.
    ///
    /// No element allocation occurs.
    pub fn declaration(
        shape: ClassicalArrayShape,
    ) -> Result<Self, ClassicalArrayError> {
        shape.validate()?;

        Ok(Self {
            shape,
            storage: ClassicalArrayStorage::Unmaterialized,
        })
    }

    /// Creates an empty sparse array.
    pub fn sparse(
        shape: ClassicalArrayShape,
    ) -> Result<Self, ClassicalArrayError> {
        let sparse = ClassicalSparseArray::new(shape.clone())?;

        Ok(Self {
            shape,
            storage: ClassicalArrayStorage::Sparse(sparse),
        })
    }

    /// Creates a dense array from materialized elements.
    pub fn dense(
        shape: ClassicalArrayShape,
        elements: Vec<T>,
    ) -> Result<Self, ClassicalArrayError> {
        let dense = ClassicalDenseArray::from_vec(
            shape.clone(),
            elements,
        )?;

        Ok(Self {
            shape,
            storage: ClassicalArrayStorage::Dense(dense),
        })
    }

    /// Creates a dense array using a checked resource policy.
    pub fn dense_generate<F>(
        shape: ClassicalArrayShape,
        policy: ClassicalArrayResourcePolicy,
        factory: F,
    ) -> Result<Self, ClassicalArrayError>
    where
        F: FnMut(ClassicalArrayIndex) -> T,
    {
        let dense = ClassicalDenseArray::try_generate(
            shape.clone(),
            policy,
            factory,
        )?;

        Ok(Self {
            shape,
            storage: ClassicalArrayStorage::Dense(dense),
        })
    }

    /// Creates a sparse array from explicit elements.
    pub fn sparse_from_elements<I>(
        shape: ClassicalArrayShape,
        elements: I,
    ) -> Result<Self, ClassicalArrayError>
    where
        I: IntoIterator<Item = (ClassicalArrayIndex, T)>,
    {
        let sparse = ClassicalSparseArray::from_elements(
            shape.clone(),
            elements,
        )?;

        Ok(Self {
            shape,
            storage: ClassicalArrayStorage::Sparse(sparse),
        })
    }

    /// Returns the logical shape.
    #[must_use]
    pub fn shape(&self) -> &ClassicalArrayShape {
        &self.shape
    }

    /// Returns the logical number of elements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.shape.size()
    }

    /// Returns whether the logical array contains no elements.
    ///
    /// Because zero-sized dimensions are rejected by the canonical shape,
    /// this is currently always false for a valid array.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.shape.size() == 0
    }

    /// Returns the storage representation.
    #[must_use]
    pub fn storage(&self) -> &ClassicalArrayStorage<T> {
        &self.storage
    }

    /// Returns mutable access to the storage representation.
    #[must_use]
    pub fn storage_mut(&mut self) -> &mut ClassicalArrayStorage<T> {
        &mut self.storage
    }

    /// Returns whether the array has materialized storage.
    #[must_use]
    pub fn is_materialized(&self) -> bool {
        self.storage.is_materialized()
    }

    /// Returns the number of materialized elements.
    #[must_use]
    pub fn materialized_len(&self) -> usize {
        self.storage.materialized_len()
    }

    /// Returns an element by multidimensional index when the underlying
    /// representation contains it.
    ///
    /// For sparse arrays, absent elements return `None`.
    ///
    /// For unmaterialized arrays, `None` is always returned.
    #[must_use]
    pub fn get(
        &self,
        indices: &[usize],
    ) -> Option<&T> {
        let index = self.shape.flatten(indices).ok()?;

        match &self.storage {
            ClassicalArrayStorage::Unmaterialized => None,

            ClassicalArrayStorage::Sparse(storage) => {
                storage.get(index)
            }

            ClassicalArrayStorage::Dense(storage) => {
                storage.get(index)
            }
        }
    }

    /// Returns a mutable element by multidimensional index.
    #[must_use]
    pub fn get_mut(
        &mut self,
        indices: &[usize],
    ) -> Option<&mut T> {
        let index = self.shape.flatten(indices).ok()?;

        match &mut self.storage {
            ClassicalArrayStorage::Unmaterialized => None,

            ClassicalArrayStorage::Sparse(storage) => {
                storage.get_mut(index)
            }

            ClassicalArrayStorage::Dense(storage) => {
                storage.get_mut(index)
            }
        }
    }

    /// Inserts a value into sparse storage.
    ///
    /// This operation fails if the array is not sparse.
    pub fn insert_sparse(
        &mut self,
        indices: &[usize],
        value: T,
    ) -> Result<Option<T>, ClassicalArrayError> {
        let index = self.shape.flatten(indices)?;

        match &mut self.storage {
            ClassicalArrayStorage::Sparse(storage) => {
                storage.insert(index, value)
            }

            ClassicalArrayStorage::Unmaterialized
            | ClassicalArrayStorage::Dense(_) => {
                Err(ClassicalArrayError::StorageShapeMismatch)
            }
        }
    }

    /// Returns the flattened index for a multidimensional index.
    pub fn index_of(
        &self,
        indices: &[usize],
    ) -> Result<ClassicalArrayIndex, ClassicalArrayError> {
        self.shape.flatten(indices)
    }

    /// Converts a flattened index into a multidimensional index.
    pub fn indices_of(
        &self,
        index: ClassicalArrayIndex,
    ) -> Result<Vec<usize>, ClassicalArrayError> {
        self.shape.unflatten(index)
    }

    /// Validates the complete array structure.
    pub fn validate(&self) -> Result<(), ClassicalArrayError> {
        self.shape.validate()?;

        match &self.storage {
            ClassicalArrayStorage::Unmaterialized => Ok(()),

            ClassicalArrayStorage::Sparse(storage) => {
                if storage.shape() != &self.shape {
                    return Err(ClassicalArrayError::StorageShapeMismatch);
                }

                storage.validate()
            }

            ClassicalArrayStorage::Dense(storage) => {
                if storage.shape() != &self.shape {
                    return Err(ClassicalArrayError::StorageShapeMismatch);
                }

                if storage.len() != self.shape.size() {
                    return Err(ClassicalArrayError::StorageShapeMismatch);
                }

                Ok(())
            }
        }
    }
}

// =============================================================================
// Convenience constructors
// =============================================================================

impl<T> ClassicalArray<T> {
    /// Creates a one-dimensional declaration.
    pub fn one_dimensional_declaration(
        length: usize,
    ) -> Result<Self, ClassicalArrayError> {
        Self::declaration(ClassicalArrayShape::one_dimensional(length)?)
    }

    /// Creates a one-dimensional sparse array.
    pub fn one_dimensional_sparse(
        length: usize,
    ) -> Result<Self, ClassicalArrayError> {
        Self::sparse(ClassicalArrayShape::one_dimensional(length)?)
    }

    /// Creates a one-dimensional dense array.
    pub fn one_dimensional_dense(
        elements: Vec<T>,
    ) -> Result<Self, ClassicalArrayError> {
        let shape = ClassicalArrayShape::one_dimensional(elements.len())?;

        Self::dense(shape, elements)
    }
}

// =============================================================================
// Formatting
// =============================================================================

impl fmt::Display for ClassicalArrayShape {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str("[")?;

        for (index, dimension) in self.dimensions.iter().enumerate() {
            if index != 0 {
                formatter.write_str(", ")?;
            }

            write!(formatter, "{dimension}")?;
        }

        formatter.write_str("]")
    }
}

impl fmt::Display for ClassicalArraySlice {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "{}..{}", self.start, self.end)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_dimensional_shape_has_correct_size() {
        let shape = ClassicalArrayShape::one_dimensional(8)
            .expect("valid shape");

        assert_eq!(shape.rank(), 1);
        assert_eq!(shape.size(), 8);
        assert_eq!(shape.dimension(0), Some(8));
    }

    #[test]
    fn multidimensional_shape_has_correct_size() {
        let shape = ClassicalArrayShape::new([2, 3, 4])
            .expect("valid shape");

        assert_eq!(shape.rank(), 3);
        assert_eq!(shape.size(), 24);
    }

    #[test]
    fn flatten_and_unflatten_are_inverse_operations() {
        let shape = ClassicalArrayShape::new([2, 3, 4])
            .expect("valid shape");

        let flat = shape
            .flatten(&[1, 2, 3])
            .expect("valid index");

        assert_eq!(flat.value(), 23);

        let indices = shape
            .unflatten(flat)
            .expect("valid flat index");

        assert_eq!(indices, vec![1, 2, 3]);
    }

    #[test]
    fn invalid_rank_is_rejected() {
        let shape = ClassicalArrayShape::new([2, 3])
            .expect("valid shape");

        let result = shape.flatten(&[1]);

        assert_eq!(
            result,
            Err(ClassicalArrayError::RankMismatch {
                expected: 2,
                actual: 1,
            })
        );
    }

    #[test]
    fn invalid_index_is_rejected() {
        let shape = ClassicalArrayShape::new([2, 3])
            .expect("valid shape");

        let result = shape.flatten(&[0, 3]);

        assert_eq!(
            result,
            Err(ClassicalArrayError::IndexOutOfBounds {
                dimension: 1,
                index: 3,
                bound: 3,
            })
        );
    }

    #[test]
    fn sparse_storage_is_deterministic() {
        let shape = ClassicalArrayShape::one_dimensional(10)
            .expect("valid shape");

        let mut storage =
            ClassicalSparseArray::<u8>::new(shape)
                .expect("valid sparse storage");

        storage
            .insert(ClassicalArrayIndex::new(7), 7)
            .expect("valid index");

        storage
            .insert(ClassicalArrayIndex::new(2), 2)
            .expect("valid index");

        let indices: Vec<usize> = storage
            .iter()
            .map(|(index, _)| index.value())
            .collect();

        assert_eq!(indices, vec![2, 7]);
    }

    #[test]
    fn declaration_does_not_materialize_elements() {
        let shape = ClassicalArrayShape::new([1_000, 1_000])
            .expect("valid shape");

        let array =
            ClassicalArray::<u8>::declaration(shape)
                .expect("valid declaration");

        assert_eq!(array.len(), 1_000_000);
        assert_eq!(array.materialized_len(), 0);
        assert!(!array.is_materialized());
    }

    #[test]
    fn sparse_array_only_materializes_inserted_values() {
        let shape = ClassicalArrayShape::one_dimensional(1_000_000)
            .expect("valid shape");

        let mut array =
            ClassicalArray::<u8>::sparse(shape)
                .expect("valid sparse array");

        array
            .insert_sparse(&[999_999], 1)
            .expect("valid insertion");

        assert_eq!(array.len(), 1_000_000);
        assert_eq!(array.materialized_len(), 1);
        assert_eq!(array.get(&[999_999]), Some(&1));
        assert_eq!(array.get(&[0]), None);
    }

    #[test]
    fn dense_generation_obeys_resource_policy() {
        let shape = ClassicalArrayShape::one_dimensional(10)
            .expect("valid shape");

        let policy =
            ClassicalArrayResourcePolicy::with_max_materialized_elements(5);

        let result =
            ClassicalArray::<u8>::dense_generate(
                shape,
                policy,
                |_| 0,
            );

        assert!(matches!(
            result,
            Err(ClassicalArrayError::ResourceLimitExceeded {
                requested: 10,
                limit: 5,
            })
        ));
    }

    #[test]
    fn dense_generation_is_checked_and_correct() {
        let shape = ClassicalArrayShape::one_dimensional(4)
            .expect("valid shape");

        let array =
            ClassicalArray::<usize>::dense_generate(
                shape,
                ClassicalArrayResourcePolicy::unrestricted(),
                |index| index.value(),
            )
            .expect("valid dense array");

        assert_eq!(array.get(&[0]), Some(&0));
        assert_eq!(array.get(&[1]), Some(&1));
        assert_eq!(array.get(&[2]), Some(&2));
        assert_eq!(array.get(&[3]), Some(&3));
    }

    #[test]
    fn slice_validates_against_dimension() {
        let slice =
            ClassicalArraySlice::new(2, 5)
                .expect("valid slice");

        assert_eq!(
            slice
                .range(10)
                .expect("valid range"),
            2..5
        );
    }

    #[test]
    fn invalid_slice_is_rejected() {
        let result =
            ClassicalArraySlice::new(5, 2);

        assert_eq!(
            result,
            Err(ClassicalArrayError::InvalidRange {
                start: 5,
                end: 2,
                length: 0,
            })
        );
    }

    #[test]
    fn index_iterator_is_lazy() {
        let shape = ClassicalArrayShape::one_dimensional(4)
            .expect("valid shape");

        let values: Vec<usize> = shape
            .indices()
            .map(ClassicalArrayIndex::value)
            .collect();

        assert_eq!(values, vec![0, 1, 2, 3]);
    }

    #[test]
    fn array_validation_accepts_matching_storage() {
        let shape = ClassicalArrayShape::one_dimensional(3)
            .expect("valid shape");

        let array =
            ClassicalArray::<u8>::dense(
                shape,
                vec![1, 2, 3],
            )
            .expect("valid dense array");

        assert!(array.validate().is_ok());
    }

    #[test]
    fn sparse_insert_rejects_out_of_bounds_index() {
        let shape = ClassicalArrayShape::one_dimensional(4)
            .expect("valid shape");

        let mut storage =
            ClassicalSparseArray::<u8>::new(shape)
                .expect("valid storage");

        let result =
            storage.insert(
                ClassicalArrayIndex::new(4),
                1,
            );

        assert!(matches!(
            result,
            Err(ClassicalArrayError::SparseIndexOutOfBounds {
                index: 4,
                size: 4,
            })
        ));
    }

    #[test]
    fn zero_dimension_is_rejected() {
        let result =
            ClassicalArrayShape::new([4, 0, 8]);

        assert_eq!(
            result,
            Err(ClassicalArrayError::InvalidDimension)
        );
    }
}