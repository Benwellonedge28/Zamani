
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani classical collections
//!
//! Path: `src/classical/collections/mod.rs`
//! Edition: Rust 2021
//! Rust baseline: 1.97 or later
//! Safety: safe Rust only.
//!
//! ## Purpose
//!
//! Define the public module boundary for Zamani's classical collection APIs.
//! This module registers collection implementations, exposes their supported
//! public types, and documents the ownership and integration contracts shared
//! by the collection subsystem.
//!
//! This module deliberately contains no collection algorithms, storage
//! implementations, compiler passes, or runtime policy.
//!
//! ## Architectural authority
//!
//! - `array.rs` currently owns `ClassicalArray<T>`, its storage strategies,
//!   `ClassicalArrayShape`, and checked array operations.
//! - `vector.rs` owns the dynamic contiguous `ClassicalVector<T>` API.
//! - `matrix.rs` owns the two-dimensional `ClassicalMatrix<T>` API.
//! - `tensor.rs` owns the tensor convenience API and delegates storage to
//!   `ClassicalArray<T>`.
//! - `sparse.rs` is a one-dimensional façade over sparse array storage.
//! - `sequence.rs` owns ordered, duplicate-preserving sequence semantics.
//! - `map.rs` owns deterministic ordered map semantics.
//! - `set.rs` owns deterministic ordered set semantics.
//! - `slice.rs` owns safe borrowed collection views.
//! - `iterator.rs` owns collection-independent iterator utilities.
//!
//! The existing `shape.rs` also defines `ClassicalArrayShape`. It must not be
//! exported as an independent shape authority while `array.rs` defines that
//! type separately. Until the shape ownership migration is completed, the
//! array module remains the compatibility source for `ClassicalArrayShape`.
//!
//! ## Ownership boundaries
//!
//! This module does not own:
//!
//! - source grammar, lexing, parsing, or AST construction;
//! - canonical semantic types or canonical classical values;
//! - compiler or quantum IR definitions;
//! - source-language ownership and borrowing analysis;
//! - symbolic dimension evaluation or type inference;
//! - numerical semantics or mathematical algorithms;
//! - global allocation, security, or execution budgets;
//! - device discovery, hardware placement, or scheduling;
//! - distributed execution or backend-specific lowering.
//!
//! Grammar specifies source syntax. Frontend semantic analysis determines
//! source-level validity. Canonical IR remains owned by the existing compiler
//! and IR subsystems. These Rust collection APIs provide concrete collection
//! abstractions; they do not independently define language semantics.
//!
//! ## POCO-REAF and scalability
//!
//! No universal hardware count, collection length, matrix dimension, tensor
//! rank, or device count is defined here.
//!
//! Concrete implementations remain subject to their documented integer
//! representations, allocator behavior, target capabilities, and available
//! resources. A finite process cannot promise infinite physical storage.
//!
//! Resource budgets, where required, must be supplied by the appropriate
//! shared policy or runtime layer. They must not become arbitrary universal
//! language-level limits.
//!
//! Sparse collections must not silently become dense. Unmaterialized arrays
//! and tensors must not allocate their logical elements merely because their
//! metadata is inspected. Fallible allocation and checked indexing remain
//! responsibilities of the relevant collection implementation.
//!
//! ## Determinism
//!
//! The module re-exports existing collection implementations without changing
//! their ordering, indexing, allocation, or mutation behavior. Ordered maps,
//! sets, and sparse storage retain their underlying deterministic semantics.
//!
//! ## Compatibility
//!
//! Public re-exports are convenience paths, not replacements for the existing
//! module paths. Existing imports such as `collections::array::ClassicalArray`
//! remain available.
//!
//! New exports must not introduce ambiguous names, competing canonical types,
//! or a second storage implementation.
//!
//! ## Freeze rule
//!
//! Changes to module ownership, public exports, shape authority, or the
//! compatibility paths require API review and tests against downstream
//! consumers. Do not declare this module production-verified until the
//! repository's supported build and test checks pass.

// =============================================================================
// Collection modules
// =============================================================================

/// Multidimensional arrays and the current canonical concrete shape API.
pub mod array;

/// Dynamically shaped two-dimensional matrices.
pub mod matrix;

/// Arbitrary-rank tensors backed by the canonical array implementation.
pub mod tensor;

/// Dynamically sized contiguous vectors.
pub mod vector;

/// Ordered, duplicate-preserving sequences.
pub mod sequence;

/// Deterministic ordered maps.
pub mod map;

/// Deterministic ordered sets.
pub mod set;

/// Safe borrowed views over contiguous collection storage.
pub mod slice;

/// Sparse one-dimensional collection façade over array storage.
pub mod sparse;

/// Collection-independent iterator utilities.
pub mod iterator;

// =============================================================================
// Array and shape compatibility exports
// =============================================================================

pub use array::{
    ClassicalArray,
    ClassicalArrayError,
    ClassicalArrayShape,
    ClassicalArrayStorage,
    ClassicalArrayStorageKind,
};

// =============================================================================
// Vector exports
// =============================================================================

pub use vector::{
    ClassicalVector,
    ClassicalVectorError,
    ClassicalVectorIter,
};

// =============================================================================
// Matrix and tensor exports
// =============================================================================

pub use matrix::{
    ClassicalMatrix,
    ClassicalMatrixError,
};

pub use tensor::{
    ClassicalTensor,
    ClassicalTensorError,
};

// =============================================================================
// Sequence exports
// =============================================================================

pub use sequence::{
    ClassicalSequence,
    ClassicalSequenceError,
    ClassicalSequenceInvariantError,
};

// =============================================================================
// Map exports
// =============================================================================

pub use map::{
    ClassicalMap,
    ClassicalMapError,
};

// =============================================================================
// Set exports
// =============================================================================

pub use set::{
    ClassicalSet,
    ClassicalSetError,
    ClassicalSetInvariantError,
};

// =============================================================================
// Borrowed slice exports
// =============================================================================

pub use slice::{
    ClassicalSlice,
    ClassicalSliceError,
    ClassicalSliceMut,
};

// =============================================================================
// Sparse collection exports
// =============================================================================

pub use sparse::{
    ClassicalSparse,
    ClassicalSparseError,
};

// =============================================================================
// Iterator utility exports
// =============================================================================

pub use iterator::{
    ClassicalIteratorError,
    ClassicalIteratorExt,
};
