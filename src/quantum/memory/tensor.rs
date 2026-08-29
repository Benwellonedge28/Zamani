//! Zamani Quantum Memory — Generic Tensor Storage
//!
//! Production-grade, provider-neutral dense tensor abstraction for the
//! quantum-memory subsystem.
//!
//! # Purpose
//!
//! `tensor.rs` is the generic multidimensional tensor substrate used by:
//!
//! - `state_vector.rs`;
//! - `density_matrix.rs`;
//! - `tensor_network.rs`;
//! - future tensor-network representations;
//! - SIMD kernels;
//! - GPU backends;
//! - distributed memory;
//! - state migration;
//! - snapshots/checkpoints;
//! - quantum simulation;
//! - QEC state representations;
//! - hardware-independent numerical kernels.
//!
//! It owns tensor semantics, shape, strides, indexing, views, reshaping,
//! permutation, slicing, contraction and common numerical operations.
//!
//! It does NOT own:
//!
//! - quantum IR;
//! - qubit identity;
//! - gate definitions;
//! - routing;
//! - scheduling;
//! - QPU communication;
//! - CUDA/HIP/Metal/Vulkan APIs;
//! - MPI/RDMA;
//! - tensor-network algorithms;
//! - state-vector semantics;
//! - density-matrix semantics;
//! - serialization formats;
//! - memory allocation policy;
//! - hardware topology.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir
//!      |
//!      v
//! quantum::memory
//!      |
//!      +------------------------------+
//!      |                              |
//!      v                              v
//!   indexing                       tensor
//!                                      |
//!             +------------------------+------------------------+
//!             |                        |                        |
//!             v                        v                        v
//!       state_vector            density_matrix          tensor_network
//!             |                        |                        |
//!             +------------------------+------------------------+
//!                                      |
//!                                      v
//!                              CPU / SIMD / GPU / distributed
//! ```
//!
//! `Tensor<T>` is deliberately not a quantum-state type. A tensor can
//! represent a state tensor, operator tensor, density-matrix block, MPS site,
//! contraction workspace, or arbitrary numerical data.
//!
//! # Hardware/QPU neutrality
//!
//! A tensor is a mathematical/data representation, not a QPU.
//!
//! Real QPUs generally do not expose their internal quantum state. Consequently
//! this module never attempts to manufacture a host-side tensor from a remote
//! QPU state unless an explicit backend provides the data.
//!
//! Instead, this file provides provider-neutral metadata:
//!
//! - `TensorStorageLocation`;
//! - `TensorDeviceKind`;
//! - `TensorDescriptor`;
//!
//! These are hand-off contracts for `memory::gpu`, `memory::distributed` and
//! hardware adapters.
//!
//! No device pointer, FFI handle or vendor type is stored here.
//!
//! # Layout
//!
//! The canonical dense tensor layout is row-major / C-order:
//!
//! ```text
//! shape = [d0, d1, d2]
//! strides = [d1*d2, d2, 1]
//! ```
//!
//! A tensor index is therefore:
//!
//! ```text
//! offset = i0*s0 + i1*s1 + i2*s2
//! ```
//!
//! All arithmetic is checked.
//!
//! # Empty tensors
//!
//! Zero-dimensional tensors are supported as scalar tensors:
//!
//! ```text
//! shape = []
//! element_count = 1
//! ```
//!
//! A dimension of zero is also supported for mathematical compatibility,
//! producing an empty tensor with zero elements. This is useful for generic
//! algorithms and is explicitly different from a malformed shape.
//!
//! # Safety
//!
//! This file contains no unsafe Rust.
//!
//! ```text
//! #![deny(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! #![deny(unused_must_use)]
//! ```
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! No external dependency is required.
//!
//! # Integration contract
//!
//! `memory::indexing` remains the authoritative subsystem for low-level
//! quantum bit/index calculations. This module deliberately keeps its own
//! generic tensor coordinate machinery because arbitrary tensors are not
//! necessarily quantum tensors.
//!
//! Quantum-specific callers should validate logical qubit indices through
//! `memory::indexing` before translating them into tensor axes.
//!
//! `tensor_network.rs` can use `Tensor<T>` as the storage for individual MPS
//! sites or temporary contraction matrices without changing MPS semantics.
//!
//! `state_vector.rs` can use a rank-1 tensor as an amplitude buffer.
//!
//! `density_matrix.rs` can use a rank-2 tensor as matrix storage.
//!
//! `gpu.rs` and `distributed.rs` can consume `TensorDescriptor` and copy data
//! through their own provider implementations.
//!
//! `migration.rs` can use `TensorDescriptor` to describe representation
//! movement without this file knowing the destination hardware.
//!
//! `snapshot.rs` and `serialization.rs` can persist shape/layout metadata and
//! values without depending on a particular hardware provider.
//!
//! The tensor API intentionally does not depend on `state.rs`, preventing a
//! dependency cycle between the generic tensor substrate and quantum-state
//! abstractions.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use core::ops::{Add, Div, Mul, Sub};

// =============================================================================
// Constants
// =============================================================================

/// Stable schema identifier for generic dense tensors.
pub const TENSOR_SCHEMA_ID: &str = "zamani.quantum.memory.tensor";

/// Semantic version of the tensor contract.
pub const TENSOR_SCHEMA_VERSION: u16 = 1;

/// Maximum number of axes accepted by the safe dynamic-rank implementation.
///
/// This is a correctness/resource guard rather than a mathematical
/// restriction. Higher-rank tensors can be represented by changing this
/// policy at a higher memory layer if required.
pub const DEFAULT_MAX_TENSOR_RANK: usize = 65_536;

/// Maximum number of elements accepted by constructors that use the default
/// resource policy.
///
/// This is intentionally conservative. Production applications with larger
/// memory budgets should validate against `memory::limits` before construction
/// and may use `from_vec_unchecked_by_policy`-style functionality in a future
/// dedicated allocator layer. No unchecked constructor is provided here.
pub const DEFAULT_MAX_TENSOR_ELEMENTS: usize = usize::MAX;

// =============================================================================
// Error model
// =============================================================================

/// Result type for tensor operations.
pub type TensorResult<T> = Result<T, TensorError>;

/// Errors produced by generic tensor operations.
///
/// The variants are deliberately independent of the memory subsystem's
/// `MemoryError` so this foundational module does not create a dependency
/// cycle. Higher memory modules may wrap these errors into `MemoryError`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TensorError {
    /// A tensor rank exceeds the configured maximum.
    RankTooLarge {
        rank: usize,
        maximum: usize,
    },

    /// A shape and value buffer disagree.
    ElementCountMismatch {
        expected: usize,
        actual: usize,
    },

    /// Shape multiplication overflowed.
    ShapeOverflow,

    /// Stride computation overflowed.
    StrideOverflow,

    /// An axis is outside the tensor rank.
    AxisOutOfBounds {
        axis: usize,
        rank: usize,
    },

    /// A tensor coordinate is outside its dimension.
    CoordinateOutOfBounds {
        axis: usize,
        coordinate: usize,
        dimension: usize,
    },

    /// The coordinate rank does not equal tensor rank.
    RankMismatch {
        expected: usize,
        actual: usize,
    },

    /// Two tensors have incompatible shapes.
    ShapeMismatch {
        left: Vec<usize>,
        right: Vec<usize>,
    },

    /// Two tensors cannot be contracted using the requested axes.
    ContractionMismatch {
        left_axis: usize,
        right_axis: usize,
        left_dimension: usize,
        right_dimension: usize,
    },

    /// The requested contraction axes are invalid.
    InvalidContractionAxes,

    /// A permutation does not describe exactly one occurrence of every axis.
    InvalidPermutation,

    /// A permutation contains a duplicate axis.
    DuplicateAxis {
        axis: usize,
    },

    /// A requested reshape has a different element count.
    ReshapeElementCountMismatch {
        current: usize,
        requested: usize,
    },

    /// A slice range is invalid.
    InvalidSlice {
        axis: usize,
        start: usize,
        end: usize,
        dimension: usize,
    },

    /// A stride is invalid for the associated shape.
    InvalidStride,

    /// A custom-stride layout would address outside the supplied storage.
    StrideOutOfBounds,

    /// An operation requires a contiguous tensor.
    NotContiguous,

    /// A matrix operation was requested on a tensor with the wrong rank.
    NotMatrix,

    /// A vector operation was requested on a tensor with the wrong rank.
    NotVector,

    /// An operation would require an invalid empty tensor.
    EmptyTensor,

    /// An operation received invalid numerical data.
    NonFiniteValue,

    /// A numerical operation would divide by zero.
    DivisionByZero,

    /// A numerical reduction has no mathematically defined result.
    UndefinedReduction,

    /// A requested operation is unsupported by this representation.
    UnsupportedOperation(&'static str),

    /// A tensor location is incompatible with the requested operation.
    InvalidStorageLocation,

    /// A tensor descriptor does not match the actual tensor.
    DescriptorMismatch,

    /// A tensor requires more elements than the configured policy permits.
    ResourceLimitExceeded {
        requested: usize,
        maximum: usize,
    },
}

impl fmt::Display for TensorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RankTooLarge { rank, maximum } => write!(
                formatter,
                "tensor rank {rank} exceeds maximum supported rank {maximum}"
            ),

            Self::ElementCountMismatch { expected, actual } => write!(
                formatter,
                "tensor contains {actual} elements but shape requires {expected}"
            ),

            Self::ShapeOverflow => {
                write!(formatter, "tensor shape element-count calculation overflowed")
            }

            Self::StrideOverflow => {
                write!(formatter, "tensor stride calculation overflowed")
            }

            Self::AxisOutOfBounds { axis, rank } => write!(
                formatter,
                "tensor axis {axis} is outside rank {rank}"
            ),

            Self::CoordinateOutOfBounds {
                axis,
                coordinate,
                dimension,
            } => write!(
                formatter,
                "tensor coordinate {coordinate} at axis {axis} is outside dimension {dimension}"
            ),

            Self::RankMismatch { expected, actual } => write!(
                formatter,
                "tensor rank mismatch: expected {expected}, got {actual}"
            ),

            Self::ShapeMismatch { left, right } => write!(
                formatter,
                "tensor shapes are incompatible: left={left:?}, right={right:?}"
            ),

            Self::ContractionMismatch {
                left_axis,
                right_axis,
                left_dimension,
                right_dimension,
            } => write!(
                formatter,
                "contraction axis mismatch: left axis {left_axis} has dimension \
                 {left_dimension}, right axis {right_axis} has dimension {right_dimension}"
            ),

            Self::InvalidContractionAxes => {
                write!(formatter, "invalid tensor contraction axes")
            }

            Self::InvalidPermutation => {
                write!(formatter, "invalid tensor axis permutation")
            }

            Self::DuplicateAxis { axis } => {
                write!(formatter, "tensor permutation contains duplicate axis {axis}")
            }

            Self::ReshapeElementCountMismatch { current, requested } => write!(
                formatter,
                "cannot reshape tensor with {current} elements into {requested} elements"
            ),

            Self::InvalidSlice {
                axis,
                start,
                end,
                dimension,
            } => write!(
                formatter,
                "invalid slice {start}..{end} on axis {axis} with dimension {dimension}"
            ),

            Self::InvalidStride => {
                write!(formatter, "tensor stride is invalid for its shape")
            }

            Self::StrideOutOfBounds => {
                write!(formatter, "tensor strides address outside supplied storage")
            }

            Self::NotContiguous => {
                write!(formatter, "operation requires a contiguous tensor")
            }

            Self::NotMatrix => {
                write!(formatter, "operation requires a rank-2 tensor")
            }

            Self::NotVector => {
                write!(formatter, "operation requires a rank-1 tensor")
            }

            Self::EmptyTensor => {
                write!(formatter, "operation is undefined for an empty tensor")
            }

            Self::NonFiniteValue => {
                write!(formatter, "tensor contains a non-finite numerical value")
            }

            Self::DivisionByZero => {
                write!(formatter, "tensor operation would divide by zero")
            }

            Self::UndefinedReduction => {
                write!(formatter, "tensor reduction has no defined result")
            }

            Self::UnsupportedOperation(operation) => {
                write!(formatter, "unsupported tensor operation: {operation}")
            }

            Self::InvalidStorageLocation => {
                write!(formatter, "invalid tensor storage location")
            }

            Self::DescriptorMismatch => {
                write!(formatter, "tensor descriptor does not match tensor metadata")
            }

            Self::ResourceLimitExceeded { requested, maximum } => write!(
                formatter,
                "tensor requires {requested} elements but configured maximum is {maximum}"
            ),
        }
    }
}

impl std::error::Error for TensorError {}

// =============================================================================
// Shape
// =============================================================================

/// Dynamic tensor shape.
///
/// An empty shape represents a scalar tensor containing exactly one element.
///
/// A shape containing a zero dimension represents an empty tensor.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TensorShape {
    dimensions: Vec<usize>,
}

impl fmt::Debug for TensorShape {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("TensorShape")
            .field(&self.dimensions)
            .finish()
    }
}

impl fmt::Display for TensorShape {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "(")?;

        for (index, dimension) in self.dimensions.iter().enumerate() {
            if index != 0 {
                write!(formatter, ", ")?;
            }

            write!(formatter, "{dimension}")?;
        }

        if self.dimensions.len() == 1 {
            write!(formatter, ",")?;
        }

        write!(formatter, ")")
    }
}

impl TensorShape {
    /// Creates a shape from dimensions.
    pub fn new(dimensions: Vec<usize>) -> TensorResult<Self> {
        if dimensions.len() > DEFAULT_MAX_TENSOR_RANK {
            return Err(TensorError::RankTooLarge {
                rank: dimensions.len(),
                maximum: DEFAULT_MAX_TENSOR_RANK,
            });
        }

        Ok(Self { dimensions })
    }

    /// Creates a scalar shape.
    pub fn scalar() -> Self {
        Self {
            dimensions: Vec::new(),
        }
    }

    /// Creates a one-dimensional shape.
    pub fn vector(length: usize) -> Self {
        Self {
            dimensions: vec![length],
        }
    }

    /// Creates a matrix shape.
    pub fn matrix(rows: usize, columns: usize) -> Self {
        Self {
            dimensions: vec![rows, columns],
        }
    }

    /// Returns the rank.
    pub fn rank(&self) -> usize {
        self.dimensions.len()
    }

    /// Returns whether this is a scalar.
    pub fn is_scalar(&self) -> bool {
        self.dimensions.is_empty()
    }

    /// Returns whether any dimension is zero.
    pub fn is_empty(&self) -> bool {
        self.dimensions.iter().any(|&dimension| dimension == 0)
    }

    /// Returns a dimension.
    pub fn dimension(&self, axis: usize) -> TensorResult<usize> {
        self.dimensions
            .get(axis)
            .copied()
            .ok_or(TensorError::AxisOutOfBounds {
                axis,
                rank: self.rank(),
            })
    }

    /// Returns all dimensions.
    pub fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }

    /// Computes the number of elements.
    ///
    /// Scalar tensors have one element.
    pub fn element_count(&self) -> TensorResult<usize> {
        let mut count = 1usize;

        for &dimension in &self.dimensions {
            count = count
                .checked_mul(dimension)
                .ok_or(TensorError::ShapeOverflow)?;
        }

        Ok(count)
    }

    /// Returns a copy with one axis inserted.
    pub fn insert_axis(&self, axis: usize, dimension: usize) -> TensorResult<Self> {
        if axis > self.rank() {
            return Err(TensorError::AxisOutOfBounds {
                axis,
                rank: self.rank(),
            });
        }

        let mut dimensions = Vec::with_capacity(self.rank() + 1);

        for index in 0..=self.rank() {
            if index == axis {
                dimensions.push(dimension);
            }

            if index < self.rank() {
                dimensions.push(self.dimensions[index]);
            }
        }

        Self::new(dimensions)
    }

    /// Removes one axis.
    pub fn remove_axis(&self, axis: usize) -> TensorResult<Self> {
        if axis >= self.rank() {
            return Err(TensorError::AxisOutOfBounds {
                axis,
                rank: self.rank(),
            });
        }

        let mut dimensions = Vec::with_capacity(self.rank().saturating_sub(1));

        for (index, &dimension) in self.dimensions.iter().enumerate() {
            if index != axis {
                dimensions.push(dimension);
            }
        }

        Self::new(dimensions)
    }
}

impl From<Vec<usize>> for TensorShape {
    fn from(dimensions: Vec<usize>) -> Self {
        Self { dimensions }
    }
}

impl AsRef<[usize]> for TensorShape {
    fn as_ref(&self) -> &[usize] {
        self.dimensions()
    }
}

// =============================================================================
// Strides
// =============================================================================

/// Row-major dense tensor strides.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TensorStrides {
    values: Vec<usize>,
}

impl fmt::Debug for TensorStrides {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("TensorStrides")
            .field(&self.values)
            .finish()
    }
}

impl TensorStrides {
    /// Creates canonical row-major strides.
    pub fn contiguous(shape: &TensorShape) -> TensorResult<Self> {
        let rank = shape.rank();

        if rank == 0 {
            return Ok(Self { values: Vec::new() });
        }

        let mut values = vec![0usize; rank];
        let mut stride = 1usize;

        for axis in (0..rank).rev() {
            values[axis] = stride;

            stride = stride
                .checked_mul(shape.dimensions()[axis])
                .ok_or(TensorError::StrideOverflow)?;
        }

        Ok(Self { values })
    }

    /// Creates explicit strides.
    pub fn new(values: Vec<usize>, rank: usize) -> TensorResult<Self> {
        if values.len() != rank {
            return Err(TensorError::RankMismatch {
                expected: rank,
                actual: values.len(),
            });
        }

        Ok(Self { values })
    }

    /// Returns the stride for an axis.
    pub fn stride(&self, axis: usize) -> TensorResult<usize> {
        self.values
            .get(axis)
            .copied()
            .ok_or(TensorError::AxisOutOfBounds {
                axis,
                rank: self.values.len(),
            })
    }

    /// Returns all strides.
    pub fn values(&self) -> &[usize] {
        &self.values
    }
}

// =============================================================================
// Storage/provider metadata
// =============================================================================

/// Provider-neutral tensor storage location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TensorStorageLocation {
    /// Ordinary host memory.
    Host,

    /// Host memory intended for accelerated transfers.
    PinnedHost,

    /// Device-local accelerator memory.
    Device,

    /// Unified/shared CPU-device address space.
    Unified,

    /// Partitioned memory spanning multiple execution nodes.
    Distributed,

    /// Memory owned by a remote execution provider.
    Remote,
}

/// Provider-neutral device class.
///
/// This is metadata only. It does not allocate or access a device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TensorDeviceKind {
    /// No device; ordinary host execution.
    Cpu,

    /// Generic accelerator.
    Accelerator,

    /// GPU-class device.
    Gpu,

    /// FPGA-class accelerator.
    Fpga,

    /// Quantum-processing-unit boundary.
    Qpu,

    /// Distributed execution fabric.
    Distributed,

    /// Unknown or externally defined device.
    Other,
}

/// Provider-neutral tensor descriptor.
///
/// Hardware layers can use this descriptor to exchange tensor metadata without
/// importing this module's implementation details into vendor-specific code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TensorDescriptor {
    shape: TensorShape,
    location: TensorStorageLocation,
    device: TensorDeviceKind,
    element_size: usize,
}

impl TensorDescriptor {
    /// Creates a tensor descriptor.
    pub fn new(
        shape: TensorShape,
        location: TensorStorageLocation,
        device: TensorDeviceKind,
        element_size: usize,
    ) -> TensorResult<Self> {
        if element_size == 0 {
            return Err(TensorError::InvalidStorageLocation);
        }

        Ok(Self {
            shape,
            location,
            device,
            element_size,
        })
    }

    /// Returns the tensor shape.
    pub fn shape(&self) -> &TensorShape {
        &self.shape
    }

    /// Returns the storage location.
    pub fn location(&self) -> TensorStorageLocation {
        self.location
    }

    /// Returns the device kind.
    pub fn device(&self) -> TensorDeviceKind {
        self.device
    }

    /// Returns the element size.
    pub fn element_size(&self) -> usize {
        self.element_size
    }

    /// Computes the logical payload size in bytes.
    pub fn byte_size(&self) -> TensorResult<usize> {
        self.shape
            .element_count()?
            .checked_mul(self.element_size)
            .ok_or(TensorError::ShapeOverflow)
    }
}

// =============================================================================
// Tensor view
// =============================================================================

/// Immutable non-owning tensor view.
///
/// The view does not allocate and does not own the underlying storage.
#[derive(Debug, Clone, Copy)]
pub struct TensorView<'a, T> {
    data: &'a [T],
    shape: &'a TensorShape,
    strides: &'a TensorStrides,
    base_offset: usize,
}

impl<'a, T> TensorView<'a, T> {
    /// Creates a view after validating that the supplied layout fits in the
    /// backing storage.
    pub fn new(
        data: &'a [T],
        shape: &'a TensorShape,
        strides: &'a TensorStrides,
    ) -> TensorResult<Self> {
        validate_layout(data.len(), shape, strides)?;

        Ok(Self {
            data,
            shape,
            strides,
            base_offset: 0,
        })
    }

    /// Returns the view shape.
    pub fn shape(&self) -> &TensorShape {
        self.shape
    }

    /// Returns the view rank.
    pub fn rank(&self) -> usize {
        self.shape.rank()
    }

    /// Returns the view element count.
    pub fn element_count(&self) -> TensorResult<usize> {
        self.shape.element_count()
    }

    /// Reads an element.
    pub fn get(&self, coordinates: &[usize]) -> TensorResult<&'a T> {
        let offset = coordinate_offset(
            self.shape,
            self.strides,
            coordinates,
        )?;

        self.data
            .get(self.base_offset + offset)
            .ok_or(TensorError::StrideOutOfBounds)
    }

    /// Returns the backing storage.
    pub fn storage(&self) -> &'a [T] {
        self.data
    }
}

/// Mutable non-owning tensor view.
///
/// The view does not allocate and does not own the underlying storage.
pub struct TensorViewMut<'a, T> {
    data: &'a mut [T],
    shape: TensorShape,
    strides: TensorStrides,
    base_offset: usize,
}

impl<'a, T> TensorViewMut<'a, T> {
    /// Creates a mutable view.
    pub fn new(
        data: &'a mut [T],
        shape: TensorShape,
        strides: TensorStrides,
    ) -> TensorResult<Self> {
        validate_layout(data.len(), &shape, &strides)?;

        Ok(Self {
            data,
            shape,
            strides,
            base_offset: 0,
        })
    }

    /// Returns the shape.
    pub fn shape(&self) -> &TensorShape {
        &self.shape
    }

    /// Returns the rank.
    pub fn rank(&self) -> usize {
        self.shape.rank()
    }

    /// Reads an element.
    pub fn get(&self, coordinates: &[usize]) -> TensorResult<&T> {
        let offset = coordinate_offset(
            &self.shape,
            &self.strides,
            coordinates,
        )?;

        self.data
            .get(self.base_offset + offset)
            .ok_or(TensorError::StrideOutOfBounds)
    }

    /// Returns a mutable element.
    pub fn get_mut(&mut self, coordinates: &[usize]) -> TensorResult<&mut T> {
        let offset = coordinate_offset(
            &self.shape,
            &self.strides,
            coordinates,
        )?;

        self.data
            .get_mut(self.base_offset + offset)
            .ok_or(TensorError::StrideOutOfBounds)
    }
}

// =============================================================================
// Tensor
// =============================================================================

/// Generic owned dense tensor.
///
/// `Tensor<T>` is intentionally generic over the scalar type and therefore
/// does not assume complex amplitudes, floating point numbers, Pauli values,
/// probabilities or any other quantum-specific representation.
#[derive(Clone, PartialEq)]
pub struct Tensor<T> {
    shape: TensorShape,
    strides: TensorStrides,
    data: Vec<T>,
}

impl<T: fmt::Debug> fmt::Debug for Tensor<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Tensor")
            .field("shape", &self.shape)
            .field("strides", &self.strides)
            .field("element_count", &self.data.len())
            .finish()
    }
}

impl<T> Tensor<T> {
    /// Creates a tensor from an explicit shape and storage.
    pub fn from_vec(
        shape: TensorShape,
        data: Vec<T>,
    ) -> TensorResult<Self> {
        let expected = shape.element_count()?;

        if expected != data.len() {
            return Err(TensorError::ElementCountMismatch {
                expected,
                actual: data.len(),
            });
        }

        if expected > DEFAULT_MAX_TENSOR_ELEMENTS {
            return Err(TensorError::ResourceLimitExceeded {
                requested: expected,
                maximum: DEFAULT_MAX_TENSOR_ELEMENTS,
            });
        }

        let strides = TensorStrides::contiguous(&shape)?;

        Ok(Self {
            shape,
            strides,
            data,
        })
    }

    /// Creates a tensor from a shape and generator.
    pub fn from_fn<F>(
        shape: TensorShape,
        mut function: F,
    ) -> TensorResult<Self>
    where
        F: FnMut(&[usize]) -> T,
    {
        let element_count = shape.element_count()?;

        if element_count > DEFAULT_MAX_TENSOR_ELEMENTS {
            return Err(TensorError::ResourceLimitExceeded {
                requested: element_count,
                maximum: DEFAULT_MAX_TENSOR_ELEMENTS,
            });
        }

        let mut data = Vec::new();

        data.try_reserve_exact(element_count)
            .map_err(|_| TensorError::ResourceLimitExceeded {
                requested: element_count,
                maximum: DEFAULT_MAX_TENSOR_ELEMENTS,
            })?;

        if element_count == 0 {
            return Self::from_vec(shape, data);
        }

        let mut coordinates = vec![0usize; shape.rank()];

        for _ in 0..element_count {
            data.push(function(&coordinates));
            increment_coordinates(
                &mut coordinates,
                shape.dimensions(),
            );
        }

        Self::from_vec(shape, data)
    }

    /// Returns the tensor shape.
    pub fn shape(&self) -> &TensorShape {
        &self.shape
    }

    /// Returns the strides.
    pub fn strides(&self) -> &TensorStrides {
        &self.strides
    }

    /// Returns the rank.
    pub fn rank(&self) -> usize {
        self.shape.rank()
    }

    /// Returns the number of elements.
    pub fn element_count(&self) -> usize {
        self.data.len()
    }

    /// Returns whether the tensor is scalar.
    pub fn is_scalar(&self) -> bool {
        self.shape.is_scalar()
    }

    /// Returns whether the tensor contains zero elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns whether the tensor is contiguous.
    pub fn is_contiguous(&self) -> bool {
        match TensorStrides::contiguous(&self.shape) {
            Ok(expected) => expected == self.strides,
            Err(_) => false,
        }
    }

    /// Returns the logical descriptor for a tensor.
    ///
    /// `T` does not expose a universal compile-time byte size, so the caller
    /// supplies the element size.
    pub fn descriptor(
        &self,
        element_size: usize,
    ) -> TensorResult<TensorDescriptor> {
        TensorDescriptor::new(
            self.shape.clone(),
            TensorStorageLocation::Host,
            TensorDeviceKind::Cpu,
            element_size,
        )
    }

    /// Returns the underlying storage.
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Returns mutable underlying storage.
    ///
    /// Mutating storage directly preserves memory safety but can invalidate
    /// higher-level mathematical invariants. State representations should
    /// therefore validate themselves after arbitrary mutation.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    /// Creates an immutable view.
    pub fn view(&self) -> TensorView<'_, T> {
        TensorView {
            data: &self.data,
            shape: &self.shape,
            strides: &self.strides,
            base_offset: 0,
        }
    }

    /// Creates a mutable view.
    pub fn view_mut(&mut self) -> TensorViewMut<'_, T> {
        TensorViewMut {
            data: &mut self.data,
            shape: self.shape.clone(),
            strides: self.strides.clone(),
            base_offset: 0,
        }
    }

    /// Returns an element by coordinates.
    pub fn get(&self, coordinates: &[usize]) -> TensorResult<&T> {
        let offset =
            coordinate_offset(&self.shape, &self.strides, coordinates)?;

        self.data
            .get(offset)
            .ok_or(TensorError::StrideOutOfBounds)
    }

    /// Returns a mutable element by coordinates.
    pub fn get_mut(
        &mut self,
        coordinates: &[usize],
    ) -> TensorResult<&mut T> {
        let offset =
            coordinate_offset(&self.shape, &self.strides, coordinates)?;

        self.data
            .get_mut(offset)
            .ok_or(TensorError::StrideOutOfBounds)
    }

    /// Returns an element by linear offset.
    pub fn get_linear(&self, offset: usize) -> TensorResult<&T> {
        self.data
            .get(offset)
            .ok_or(TensorError::CoordinateOutOfBounds {
                axis: 0,
                coordinate: offset,
                dimension: self.data.len(),
            })
    }

    /// Returns a mutable element by linear offset.
    pub fn get_linear_mut(
        &mut self,
        offset: usize,
    ) -> TensorResult<&mut T> {
        self.data
            .get_mut(offset)
            .ok_or(TensorError::CoordinateOutOfBounds {
                axis: 0,
                coordinate: offset,
                dimension: self.data.len(),
            })
    }

    /// Creates a scalar tensor.
    pub fn scalar(value: T) -> Self {
        Self {
            shape: TensorShape::scalar(),
            strides: TensorStrides {
                values: Vec::new(),
            },
            data: vec![value],
        }
    }

    /// Creates a vector tensor.
    pub fn vector(values: Vec<T>) -> Self {
        let length = values.len();

        Self {
            shape: TensorShape::vector(length),
            strides: TensorStrides {
                values: vec![1],
            },
            data: values,
        }
    }

    /// Creates a matrix tensor.
    pub fn matrix(
        rows: usize,
        columns: usize,
        values: Vec<T>,
    ) -> TensorResult<Self> {
        Self::from_vec(
            TensorShape::matrix(rows, columns),
            values,
        )
    }

    /// Fills every element with a cloned value.
    pub fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        for element in &mut self.data {
            *element = value.clone();
        }
    }

    /// Maps every element into another tensor without changing shape.
    pub fn map<U, F>(&self, mut function: F) -> TensorResult<U>
    where
        F: FnMut(&T) -> U,
    {
        let mut data = Vec::new();

        data.try_reserve_exact(self.data.len())
            .map_err(|_| TensorError::ResourceLimitExceeded {
                requested: self.data.len(),
                maximum: DEFAULT_MAX_TENSOR_ELEMENTS,
            })?;

        for value in &self.data {
            data.push(function(value));
        }

        Tensor::from_vec(self.shape.clone(), data)
    }

    /// Reshapes a contiguous tensor without changing its storage.
    ///
    /// The returned tensor owns the same values conceptually but the operation
    /// itself currently constructs a new tensor metadata object while moving
    /// the existing `Vec`.
    pub fn reshape(
        mut self,
        new_shape: TensorShape,
    ) -> TensorResult<Self> {
        let current = self.data.len();
        let requested = new_shape.element_count()?;

        if current != requested {
            return Err(TensorError::ReshapeElementCountMismatch {
                current,
                requested,
            });
        }

        self.strides = TensorStrides::contiguous(&new_shape)?;
        self.shape = new_shape;

        Ok(self)
    }

    /// Returns a reshaped clone while retaining the original tensor.
    pub fn reshaped(
        &self,
        new_shape: TensorShape,
    ) -> TensorResult<Self>
    where
        T: Clone,
    {
        self.clone().reshape(new_shape)
    }

    /// Permutes axes.
    ///
    /// This operation returns a physically materialized tensor in the requested
    /// order. It deliberately does not expose a fake contiguous layout over
    /// non-contiguous storage.
    pub fn permute(
        &self,
        permutation: &[usize],
    ) -> TensorResult<Self>
    where
        T: Clone,
    {
        validate_permutation(permutation, self.rank())?;

        let mut new_shape = Vec::with_capacity(self.rank());

        for &axis in permutation {
            new_shape.push(self.shape.dimensions()[axis]);
        }

        let shape = TensorShape::new(new_shape)?;

        Tensor::from_fn(shape, |coordinates| {
            let mut source = vec![0usize; self.rank()];

            for (new_axis, &old_axis) in permutation.iter().enumerate() {
                source[old_axis] = coordinates[new_axis];
            }

            self.get(&source)
                .expect("validated permutation and coordinates")
                .clone()
        })
    }

    /// Transposes a rank-2 tensor.
    pub fn transpose(&self) -> TensorResult<Self>
    where
        T: Clone,
    {
        if self.rank() != 2 {
            return Err(TensorError::NotMatrix);
        }

        self.permute(&[1, 0])
    }

    /// Extracts an owned slice along one axis.
    ///
    /// `start..end` is half-open.
    pub fn slice(
        &self,
        axis: usize,
        start: usize,
        end: usize,
    ) -> TensorResult<Self>
    where
        T: Clone,
    {
        let dimension = self.shape.dimension(axis)?;

        if start > end || end > dimension {
            return Err(TensorError::InvalidSlice {
                axis,
                start,
                end,
                dimension,
            });
        }

        let mut new_dimensions = self.shape.dimensions().to_vec();
        new_dimensions[axis] = end - start;

        let new_shape = TensorShape::new(new_dimensions)?;

        Tensor::from_fn(new_shape, |coordinates| {
            let mut source = coordinates.to_vec();
            source[axis] = source[axis]
                .checked_add(start)
                .expect("validated slice arithmetic");

            self.get(&source)
                .expect("validated slice coordinate")
                .clone()
        })
    }

    /// Returns a tensor with one dimension inserted.
    ///
    /// The new axis is populated by repeating the original tensor values.
    pub fn broadcast_axis(
        &self,
        axis: usize,
        dimension: usize,
    ) -> TensorResult<Self>
    where
        T: Clone,
    {
        let new_shape = self.shape.insert_axis(axis, dimension)?;

        Tensor::from_fn(new_shape, |coordinates| {
            let mut source = Vec::with_capacity(self.rank());

            for old_axis in 0..self.rank() {
                let new_axis = if old_axis < axis {
                    old_axis
                } else {
                    old_axis + 1
                };

                source.push(coordinates[new_axis]);
            }

            self.get(&source)
                .expect("validated broadcast coordinate")
                .clone()
        })
    }

    /// Computes an outer/tensor product.
    pub fn outer<U>(
        &self,
        rhs: &Tensor<U>,
    ) -> TensorResult<Tensor<(T, U)>>
    where
        T: Clone,
        U: Clone,
    {
        let mut dimensions = Vec::with_capacity(
            self.rank() + rhs.rank(),
        );

        dimensions.extend_from_slice(self.shape.dimensions());
        dimensions.extend_from_slice(rhs.shape.dimensions());

        let shape = TensorShape::new(dimensions)?;

        Tensor::from_fn(shape, |coordinates| {
            let left_rank = self.rank();

            let left = self
                .get(&coordinates[..left_rank])
                .expect("validated outer-product coordinates")
                .clone();

            let right = rhs
                .get(&coordinates[left_rank..])
                .expect("validated outer-product coordinates")
                .clone();

            (left, right)
        })
    }

    /// Computes the tensor product using multiplication.
    pub fn tensor_product(
        &self,
        rhs: &Tensor<T>,
    ) -> TensorResult<Self>
    where
        T: Clone + Mul<Output = T>,
    {
        let mut dimensions = Vec::with_capacity(
            self.rank() + rhs.rank(),
        );

        dimensions.extend_from_slice(self.shape.dimensions());
        dimensions.extend_from_slice(rhs.shape.dimensions());

        let shape = TensorShape::new(dimensions)?;

        Tensor::from_fn(shape, |coordinates| {
            let left_rank = self.rank();

            let left = self
                .get(&coordinates[..left_rank])
                .expect("validated tensor-product coordinates")
                .clone();

            let right = rhs
                .get(&coordinates[left_rank..])
                .expect("validated tensor-product coordinates")
                .clone();

            left * right
        })
    }

    /// Performs a matrix multiplication.
    ///
    /// `self` and `rhs` must both be rank-2 tensors and the inner dimensions
    /// must agree.
    pub fn matmul(
        &self,
        rhs: &Tensor<T>,
    ) -> TensorResult<Self>
    where
        T: Clone + Default + Add<Output = T> + Mul<Output = T>,
    {
        if self.rank() != 2 || rhs.rank() != 2 {
            return Err(TensorError::NotMatrix);
        }

        let m = self.shape.dimensions()[0];
        let k_left = self.shape.dimensions()[1];

        let k_right = rhs.shape.dimensions()[0];
        let n = rhs.shape.dimensions()[1];

        if k_left != k_right {
            return Err(TensorError::ShapeMismatch {
                left: self.shape.dimensions().to_vec(),
                right: rhs.shape.dimensions().to_vec(),
            });
        }

        let shape = TensorShape::matrix(m, n);

        Tensor::from_fn(shape, |coordinates| {
            let row = coordinates[0];
            let column = coordinates[1];

            let mut result = T::default();

            for k in 0..k_left {
                let a = self
                    .get(&[row, k])
                    .expect("validated matrix coordinate")
                    .clone();

                let b = rhs
                    .get(&[k, column])
                    .expect("validated matrix coordinate")
                    .clone();

                result = result + a * b;
            }

            result
        })
    }

    /// Contracts one axis of `self` against one axis of `rhs`.
    ///
    /// Example:
    ///
    /// ```text
    /// A[a,b,c] · B[c,d,e]
    /// -------------------
    /// result[a,b,d,e]
    /// ```
    pub fn contract(
        &self,
        rhs: &Tensor<T>,
        self_axis: usize,
        rhs_axis: usize,
    ) -> TensorResult<Self>
    where
        T: Clone + Default + Add<Output = T> + Mul<Output = T>,
    {
        let self_dimension = self.shape.dimension(self_axis)?;
        let rhs_dimension = rhs.shape.dimension(rhs_axis)?;

        if self_dimension != rhs_dimension {
            return Err(TensorError::ContractionMismatch {
                left_axis: self_axis,
                right_axis: rhs_axis,
                left_dimension: self_dimension,
                right_dimension: rhs_dimension,
            });
        }

        let mut result_dimensions = Vec::new();

        for (axis, &dimension) in self.shape.dimensions().iter().enumerate() {
            if axis != self_axis {
                result_dimensions.push(dimension);
            }
        }

        for (axis, &dimension) in rhs.shape.dimensions().iter().enumerate() {
            if axis != rhs_axis {
                result_dimensions.push(dimension);
            }
        }

        let result_shape = TensorShape::new(result_dimensions)?;

        Tensor::from_fn(result_shape, |coordinates| {
            let left_result_rank = self.rank() - 1;

            let left_coordinates =
                insert_contracted_coordinate(
                    coordinates,
                    self.rank(),
                    self_axis,
                    0,
                );

            let right_coordinates =
                insert_contracted_coordinate(
                    &coordinates[left_result_rank..],
                    rhs.rank(),
                    rhs_axis,
                    0,
                );

            let mut result = T::default();

            for contracted in 0..self_dimension {
                let mut left = left_coordinates.clone();
                let mut right = right_coordinates.clone();

                left[self_axis] = contracted;
                right[rhs_axis] = contracted;

                let left_value = self
                    .get(&left)
                    .expect("validated contraction coordinate")
                    .clone();

                let right_value = rhs
                    .get(&right)
                    .expect("validated contraction coordinate")
                    .clone();

                result = result + left_value * right_value;
            }

            result
        })
    }

    /// Sums all elements.
    pub fn sum(&self) -> TensorResult<T>
    where
        T: Clone + Default + Add<Output = T>,
    {
        if self.data.is_empty() {
            return Err(TensorError::UndefinedReduction);
        }

        let mut result = T::default();

        for value in &self.data {
            result = result + value.clone();
        }

        Ok(result)
    }

    /// Computes an elementwise addition.
    pub fn add_tensor(
        &self,
        rhs: &Tensor<T>,
    ) -> TensorResult<Self>
    where
        T: Clone + Add<Output = T>,
    {
        self.require_same_shape(rhs)?;

        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(left, right)| left.clone() + right.clone())
            .collect();

        Tensor::from_vec(self.shape.clone(), data)
    }

    /// Computes an elementwise subtraction.
    pub fn sub_tensor(
        &self,
        rhs: &Tensor<T>,
    ) -> TensorResult<Self>
    where
        T: Clone + Sub<Output = T>,
    {
        self.require_same_shape(rhs)?;

        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(left, right)| left.clone() - right.clone())
            .collect();

        Tensor::from_vec(self.shape.clone(), data)
    }

    /// Computes an elementwise multiplication.
    pub fn mul_tensor(
        &self,
        rhs: &Tensor<T>,
    ) -> TensorResult<Self>
    where
        T: Clone + Mul<Output = T>,
    {
        self.require_same_shape(rhs)?;

        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(left, right)| left.clone() * right.clone())
            .collect();

        Tensor::from_vec(self.shape.clone(), data)
    }

    /// Computes an elementwise division.
    pub fn div_tensor(
        &self,
        rhs: &Tensor<T>,
    ) -> TensorResult<Self>
    where
        T: Clone + Div<Output = T> + PartialEq + Default,
    {
        self.require_same_shape(rhs)?;

        let zero = T::default();

        for value in &rhs.data {
            if *value == zero {
                return Err(TensorError::DivisionByZero);
            }
        }

        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(left, right)| left.clone() / right.clone())
            .collect();

        Tensor::from_vec(self.shape.clone(), data)
    }

    /// Requires two tensors to have identical shapes.
    pub fn require_same_shape(
        &self,
        rhs: &Tensor<T>,
    ) -> TensorResult<()> {
        if self.shape != rhs.shape {
            return Err(TensorError::ShapeMismatch {
                left: self.shape.dimensions().to_vec(),
                right: rhs.shape.dimensions().to_vec(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Numerical helpers
// =============================================================================

impl<T> Tensor<T>
where
    T: Clone + Default + Add<Output = T> + Mul<Output = T>,
{
    /// Computes the algebraic inner product without conjugation.
    ///
    /// For complex quantum amplitudes, callers that require the Hermitian
    /// inner product must provide a conjugating scalar operation at the
    /// appropriate quantum/numerical layer.
    pub fn algebraic_inner_product(
        &self,
        rhs: &Tensor<T>,
    ) -> TensorResult<T> {
        self.require_same_shape(rhs)?;

        if self.data.is_empty() {
            return Err(TensorError::UndefinedReduction);
        }

        let mut result = T::default();

        for (left, right) in
            self.data.iter().zip(rhs.data.iter())
        {
            result = result + left.clone() * right.clone();
        }

        Ok(result)
    }
}

// =============================================================================
// Layout helpers
// =============================================================================

fn validate_layout<T>(
    storage_len: usize,
    shape: &TensorShape,
    strides: &TensorStrides,
) -> TensorResult<()> {
    if shape.rank() != strides.values().len() {
        return Err(TensorError::RankMismatch {
            expected: shape.rank(),
            actual: strides.values().len(),
        });
    }

    let element_count = shape.element_count()?;

    if element_count == 0 {
        return Ok(());
    }

    let mut maximum_offset = 0usize;

    for axis in 0..shape.rank() {
        let dimension = shape.dimensions()[axis];

        if dimension == 0 {
            return Ok(());
        }

        let contribution = (dimension - 1)
            .checked_mul(strides.values()[axis])
            .ok_or(TensorError::StrideOverflow)?;

        maximum_offset = maximum_offset
            .checked_add(contribution)
            .ok_or(TensorError::StrideOverflow)?;
    }

    if maximum_offset >= storage_len {
        return Err(TensorError::StrideOutOfBounds);
    }

    Ok(())
}

fn coordinate_offset(
    shape: &TensorShape,
    strides: &TensorStrides,
    coordinates: &[usize],
) -> TensorResult<usize> {
    if shape.rank() != coordinates.len() {
        return Err(TensorError::RankMismatch {
            expected: shape.rank(),
            actual: coordinates.len(),
        });
    }

    if strides.values().len() != shape.rank() {
        return Err(TensorError::RankMismatch {
            expected: shape.rank(),
            actual: strides.values().len(),
        });
    }

    let mut offset = 0usize;

    for axis in 0..shape.rank() {
        let coordinate = coordinates[axis];
        let dimension = shape.dimensions()[axis];

        if coordinate >= dimension {
            return Err(TensorError::CoordinateOutOfBounds {
                axis,
                coordinate,
                dimension,
            });
        }

        let contribution = coordinate
            .checked_mul(strides.values()[axis])
            .ok_or(TensorError::StrideOverflow)?;

        offset = offset
            .checked_add(contribution)
            .ok_or(TensorError::StrideOverflow)?;
    }

    Ok(offset)
}

fn validate_permutation(
    permutation: &[usize],
    rank: usize,
) -> TensorResult<()> {
    if permutation.len() != rank {
        return Err(TensorError::InvalidPermutation);
    }

    let mut seen = vec![false; rank];

    for &axis in permutation {
        if axis >= rank {
            return Err(TensorError::AxisOutOfBounds {
                axis,
                rank,
            });
        }

        if seen[axis] {
            return Err(TensorError::DuplicateAxis { axis });
        }

        seen[axis] = true;
    }

    Ok(())
}

fn increment_coordinates(
    coordinates: &mut [usize],
    dimensions: &[usize],
) {
    if coordinates.is_empty() {
        return;
    }

    for axis in (0..coordinates.len()).rev() {
        coordinates[axis] += 1;

        if coordinates[axis] < dimensions[axis] {
            return;
        }

        coordinates[axis] = 0;
    }
}

fn insert_contracted_coordinate(
    result_coordinates: &[usize],
    source_rank: usize,
    contracted_axis: usize,
    contracted_value: usize,
) -> Vec<usize> {
    let mut result = Vec::with_capacity(source_rank);

    let mut source_index = 0usize;

    for axis in 0..source_rank {
        if axis == contracted_axis {
            result.push(contracted_value);
        } else {
            result.push(result_coordinates[source_index]);
            source_index += 1;
        }
    }

    result
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_shape_has_one_element() {
        let shape = TensorShape::scalar();

        assert_eq!(shape.rank(), 0);
        assert_eq!(shape.element_count().unwrap(), 1);
    }

    #[test]
    fn vector_shape_is_contiguous() {
        let tensor =
            Tensor::vector(vec![1u32, 2, 3, 4]);

        assert_eq!(tensor.shape().dimensions(), &[4]);
        assert_eq!(tensor.strides().values(), &[1]);
        assert!(tensor.is_contiguous());
    }

    #[test]
    fn matrix_indexing_is_correct() {
        let tensor =
            Tensor::matrix(2, 3, vec![1u32, 2, 3, 4, 5, 6])
                .unwrap();

        assert_eq!(*tensor.get(&[0, 0]).unwrap(), 1);
        assert_eq!(*tensor.get(&[0, 2]).unwrap(), 3);
        assert_eq!(*tensor.get(&[1, 0]).unwrap(), 4);
        assert_eq!(*tensor.get(&[1, 2]).unwrap(), 6);
    }

    #[test]
    fn invalid_coordinates_are_rejected() {
        let tensor =
            Tensor::matrix(2, 2, vec![1u32, 2, 3, 4])
                .unwrap();

        assert!(matches!(
            tensor.get(&[2, 0]),
            Err(TensorError::CoordinateOutOfBounds { .. })
        ));
    }

    #[test]
    fn reshape_preserves_element_count() {
        let tensor =
            Tensor::vector(vec![1u32, 2, 3, 4]);

        let reshaped = tensor
            .reshape(TensorShape::matrix(2, 2))
            .unwrap();

        assert_eq!(reshaped.shape().dimensions(), &[2, 2]);
        assert_eq!(reshaped.as_slice(), &[1, 2, 3, 4]);
    }

    #[test]
    fn invalid_reshape_is_rejected() {
        let tensor =
            Tensor::vector(vec![1u32, 2, 3, 4]);

        let result =
            tensor.reshape(TensorShape::matrix(3, 2));

        assert!(matches!(
            result,
            Err(TensorError::ReshapeElementCountMismatch { .. })
        ));
    }

    #[test]
    fn transpose_is_correct() {
        let tensor =
            Tensor::matrix(2, 3, vec![1u32, 2, 3, 4, 5, 6])
                .unwrap();

        let transposed = tensor.transpose().unwrap();

        assert_eq!(
            transposed.as_slice(),
            &[1, 4, 2, 5, 3, 6]
        );

        assert_eq!(transposed.shape().dimensions(), &[3, 2]);
    }

    #[test]
    fn permutation_is_correct() {
        let tensor = Tensor::from_vec(
            TensorShape::new(vec![2, 3, 4]).unwrap(),
            (0u32..24).collect(),
        )
        .unwrap();

        let permuted = tensor.permute(&[2, 0, 1]).unwrap();

        assert_eq!(
            permuted.shape().dimensions(),
            &[4, 2, 3]
        );

        assert_eq!(
            *permuted.get(&[0, 0, 0]).unwrap(),
            0
        );

        assert_eq!(
            *permuted.get(&[1, 0, 0]).unwrap(),
            12
        );
    }

    #[test]
    fn slicing_is_correct() {
        let tensor =
            Tensor::matrix(3, 3, vec![
                0u32, 1, 2,
                3, 4, 5,
                6, 7, 8,
            ])
            .unwrap();

        let slice = tensor.slice(0, 1, 3).unwrap();

        assert_eq!(
            slice.shape().dimensions(),
            &[2, 3]
        );

        assert_eq!(
            slice.as_slice(),
            &[3, 4, 5, 6, 7, 8]
        );
    }

    #[test]
    fn matrix_multiplication_is_correct() {
        let left =
            Tensor::matrix(2, 3, vec![
                1i32, 2, 3,
                4, 5, 6,
            ])
            .unwrap();

        let right =
            Tensor::matrix(3, 2, vec![
                7i32, 8,
                9, 10,
                11, 12,
            ])
            .unwrap();

        let result = left.matmul(&right).unwrap();

        assert_eq!(
            result.as_slice(),
            &[58, 64, 139, 154]
        );
    }

    #[test]
    fn contraction_is_correct() {
        let left =
            Tensor::matrix(2, 3, vec![
                1i32, 2, 3,
                4, 5, 6,
            ])
            .unwrap();

        let right =
            Tensor::matrix(3, 2, vec![
                7i32, 8,
                9, 10,
                11, 12,
            ])
            .unwrap();

        let result =
            left.contract(&right, 1, 0).unwrap();

        assert_eq!(
            result.shape().dimensions(),
            &[2, 2]
        );

        assert_eq!(
            result.as_slice(),
            &[58, 64, 139, 154]
        );
    }

    #[test]
    fn tensor_product_is_correct() {
        let left = Tensor::vector(vec![1i32, 2]);
        let right = Tensor::vector(vec![3i32, 4]);

        let result =
            left.tensor_product(&right).unwrap();

        assert_eq!(
            result.shape().dimensions(),
            &[2, 2]
        );

        assert_eq!(
            result.as_slice(),
            &[3, 4, 6, 8]
        );
    }

    #[test]
    fn outer_product_preserves_values() {
        let left = Tensor::vector(vec![1i32, 2]);
        let right = Tensor::vector(vec![3i32, 4]);

        let result =
            left.outer(&right).unwrap();

        assert_eq!(
            result.shape().dimensions(),
            &[2, 2]
        );

        assert_eq!(
            *result.get(&[1, 0]).unwrap(),
            (2, 3)
        );
    }

    #[test]
    fn scalar_sum_is_correct() {
        let tensor = Tensor::vector(vec![1i32, 2, 3, 4]);

        assert_eq!(tensor.sum().unwrap(), 10);
    }

    #[test]
    fn empty_sum_is_rejected() {
        let tensor =
            Tensor::from_vec(
                TensorShape::new(vec![0, 4]).unwrap(),
                Vec::<i32>::new(),
            )
            .unwrap();

        assert!(matches!(
            tensor.sum(),
            Err(TensorError::UndefinedReduction)
        ));
    }

    #[test]
    fn elementwise_operations_are_correct() {
        let left = Tensor::vector(vec![1i32, 2, 3]);
        let right = Tensor::vector(vec![4i32, 5, 6]);

        assert_eq!(
            left.add_tensor(&right)
                .unwrap()
                .as_slice(),
            &[5, 7, 9]
        );

        assert_eq!(
            left.sub_tensor(&right)
                .unwrap()
                .as_slice(),
            &[-3, -3, -3]
        );

        assert_eq!(
            left.mul_tensor(&right)
                .unwrap()
                .as_slice(),
            &[4, 10, 18]
        );
    }

    #[test]
    fn descriptor_is_provider_neutral() {
        let tensor =
            Tensor::vector(vec![1u64, 2, 3, 4]);

        let descriptor =
            tensor.descriptor(core::mem::size_of::<u64>())
                .unwrap();

        assert_eq!(
            descriptor.location(),
            TensorStorageLocation::Host
        );

        assert_eq!(
            descriptor.device(),
            TensorDeviceKind::Cpu
        );

        assert_eq!(descriptor.byte_size().unwrap(), 32);
    }

    #[test]
    fn mutable_view_can_modify_tensor() {
        let mut tensor =
            Tensor::vector(vec![1u32, 2, 3]);

        {
            let mut view = tensor.view_mut();

            *view.get_mut(&[1]).unwrap() = 42;
        }

        assert_eq!(tensor.as_slice(), &[1, 42, 3]);
    }

    #[test]
    fn rank_zero_tensor_permutation_is_valid() {
        let tensor = Tensor::scalar(42u32);

        let result = tensor.permute(&[]).unwrap();

        assert_eq!(result.as_slice(), &[42]);
        assert!(result.is_scalar());
    }
}