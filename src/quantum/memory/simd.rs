//! Zamani Quantum Memory — Safe SIMD / Vectorized Execution Layer
//!
//! This module provides the provider-neutral, `unsafe`-free vectorized
//! execution layer for `quantum::memory`.
//!
//! # Architectural purpose
//!
//! `simd.rs` does NOT implement a particular SIMD instruction set.
//!
//! It provides a safe execution contract that higher-level memory
//! representations can use for:
//!
//! - state-vector arithmetic;
//! - density-matrix arithmetic;
//! - tensor operations;
//! - sparse-state numerical kernels;
//! - stabilizer/bit operations;
//! - normalization;
//! - probability calculations;
//! - complex arithmetic;
//! - host-side preparation for GPU/device transfers.
//!
//! The implementation deliberately uses ordinary Rust slices, chunks and
//! arithmetic operations. This allows LLVM to auto-vectorize hot loops where
//! profitable while preserving a completely safe public API.
//!
//! # Why this design is required
//!
//! Rust's portable `std::simd` API is not part of the stable Rust contract
//! targeted by Zamani 1.97/1.97.1. Architecture-specific SIMD intrinsics
//! generally require `unsafe` at the call boundary. Zamani's memory
//! architecture explicitly requires this module to contain no `unsafe` code.
//!
//! Therefore this module must NOT:
//!
//! - use `std::simd`;
//! - use `core::simd`;
//! - use `std::arch` intrinsics;
//! - use raw pointers;
//! - use pointer arithmetic;
//! - use `get_unchecked`;
//! - use architecture-specific FFI;
//! - require AVX/AVX2/AVX-512;
//! - require SSE/SSE2;
//! - require NEON/SVE;
//! - require RVV;
//! - require WASM SIMD;
//! - require a particular processor;
//! - require a particular QPU.
//!
//! Architecture-specific implementations may be added later behind separate
//! provider crates/modules, provided that they preserve the safe contracts
//! defined here.
//!
//! # Execution model
//!
//! ```text
//! Quantum state / tensor
//!          │
//!          ▼
//!     memory::simd
//!          │
//!    ┌─────┴─────┐
//!    │           │
//!    ▼           ▼
//! vectorized   scalar
//! safe loop    fallback
//!    │           │
//!    └─────┬─────┘
//!          ▼
//!      CPU memory
//! ```
//!
//! The caller does not need to know which path was selected.
//!
//! # Hardware neutrality
//!
//! This module is deliberately independent of:
//!
//! - IBM QPUs;
//! - Google QPUs;
//! - Quantinuum QPUs;
//! - IonQ QPUs;
//! - Rigetti QPUs;
//! - Pasqal QPUs;
//! - QuEra QPUs;
//! - D-Wave systems;
//! - photonic hardware;
//! - superconducting hardware;
//! - trapped-ion hardware;
//! - neutral-atom hardware;
//! - semiconductor/spin hardware;
//! - topological hardware;
//! - annealing hardware;
//! - future QPU architectures.
//!
//! A real QPU normally does not expose its quantum state as a CPU SIMD array.
//! In that case this layer is used for host-side classical data, result
//! processing, tensor preparation, compilation support, or simulation.
//!
//! The QPU-specific execution contract belongs to `quantum::hardware` and
//! `memory::backend_state`, not this file.
//!
//! # Integration contract
//!
//! ## `memory::cpu`
//!
//! `cpu.rs` owns CPU memory and buffers. This module owns numerical traversal
//! and vectorized operations over safe slices.
//!
//! ```text
//! cpu::CpuBuffer<T>
//!       │
//!       ▼
//! simd::for_each / simd::complex_*
//! ```
//!
//! ## `state_vector.rs`
//!
//! State-vector kernels should use this module for:
//!
//! - elementwise scaling;
//! - normalization;
//! - probability calculation;
//! - complex multiply-add;
//! - pairwise transformations;
//! - copying and accumulation.
//!
//! State-vector indexing remains owned by `layout.rs` and `indexing.rs`.
//!
//! ## `density_matrix.rs`
//!
//! Density-matrix implementations may use the same complex kernels for:
//!
//! - matrix scaling;
//! - trace accumulation;
//! - channel arithmetic;
//! - Hermiticity checks;
//! - normalization.
//!
//! ## `tensor.rs`
//!
//! Tensor implementations may use generic real/complex elementwise kernels
//! before or after contraction.
//!
//! Tensor shape and indexing remain owned by `tensor.rs` and `layout.rs`.
//!
//! ## `tensor_network.rs`
//!
//! Tensor-network contraction code may use these kernels for local tensor
//! arithmetic. Global contraction planning remains outside this module.
//!
//! ## `stabilizer.rs`
//!
//! Stabilizer implementations may use the bitwise kernels where appropriate.
//! Stabilizer semantics remain owned by `stabilizer.rs`.
//!
//! ## `gpu.rs`
//!
//! GPU implementations may use the same operation-level semantics for host
//! fallback/reference implementations. This module must not depend on GPU APIs.
//!
//! ## `distributed.rs`
//!
//! Distributed memory can use these kernels for local partitions. Network
//! communication remains outside this module.
//!
//! ## `migration.rs`
//!
//! Migration can use these operations for host-side conversion and validation.
//!
//! ## `telemetry.rs`
//!
//! Performance counters should record which execution path was selected, but
//! this module must not depend on telemetry.
//!
//! # No dependency cycle
//!
//! This module intentionally depends only on the Rust standard library.
//!
//! It does not depend on:
//!
//! - `state_vector.rs`;
//! - `density_matrix.rs`;
//! - `cpu.rs`;
//! - `gpu.rs`;
//! - `allocator.rs`;
//! - `telemetry.rs`;
//! - `benchmarking`;
//! - `hardware`;
//! - `routing`.
//!
//! This makes the file independently complete and prevents later modules from
//! forcing it to be rewritten.
//!
//! # Numerical policy
//!
//! This module does not silently normalize, clamp, or discard non-finite
//! values. Callers receive explicit results or errors.
//!
//! Floating-point equality is never used as a quantum-correctness criterion.
//! Higher-level numerical policy belongs in `memory::numeric`.
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
//! # Safety
//!
//! This module contains no `unsafe` code.
//!
//! The compiler may generate SIMD instructions from ordinary safe Rust when
//! optimization is enabled. That is an implementation detail of the compiler
//! and does not change the safety contract of this module.
//!
//! # Performance philosophy
//!
//! The primary invariant is:
//!
//! ```text
//! correctness > portability > deterministic behavior > optimization
//! ```
//!
//! The API is intentionally suitable for optimized release builds without
//! making correctness depend on a particular SIMD width.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::ops::{Add, Mul, Sub};

// =============================================================================
// Schema
// =============================================================================

/// Stable identifier for the SIMD memory contract.
pub const SIMD_MEMORY_SCHEMA_ID: &str = "zamani.quantum.memory.simd";

/// Semantic version of the SIMD memory contract.
pub const SIMD_MEMORY_SCHEMA_VERSION: u16 = 1;

/// Default logical lane width used by the safe vectorized execution layer.
///
/// This is a scheduling/chunking width, NOT a claim about physical CPU SIMD
/// register width.
pub const DEFAULT_LANE_WIDTH: usize = 8;

/// Minimum supported lane width.
pub const MIN_LANE_WIDTH: usize = 1;

/// Maximum logical lane width accepted by the portable execution layer.
///
/// Keeping this bounded prevents accidental enormous temporary/chunk sizes.
pub const MAX_LANE_WIDTH: usize = 1024;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by safe SIMD/vectorized memory operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimdError {
    /// A lane width was invalid.
    InvalidLaneWidth {
        /// Requested lane width.
        requested: usize,

        /// Minimum supported width.
        minimum: usize,

        /// Maximum supported width.
        maximum: usize,
    },

    /// Input slices have incompatible lengths.
    LengthMismatch {
        /// Length of the first input.
        left: usize,

        /// Length of the second input.
        right: usize,
    },

    /// Three input slices have incompatible lengths.
    TernaryLengthMismatch {
        /// First input length.
        first: usize,

        /// Second input length.
        second: usize,

        /// Third input length.
        third: usize,
    },

    /// A destination is too small.
    DestinationTooSmall {
        /// Required number of elements.
        required: usize,

        /// Available number of elements.
        available: usize,
    },

    /// A requested range is invalid.
    InvalidRange {
        /// Start index.
        start: usize,

        /// End index.
        end: usize,

        /// Slice length.
        len: usize,
    },

    /// An operation would overflow.
    ArithmeticOverflow,

    /// A numerical operation produced a non-finite result where finite output
    /// was explicitly required.
    NonFiniteResult,

    /// A reduction was requested over an empty input when no identity exists.
    EmptyReduction,

    /// The operation is valid in principle but not provided by this portable
    /// layer.
    UnsupportedOperation {
        /// Stable operation identifier.
        operation: &'static str,
    },
}

impl fmt::Display for SimdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLaneWidth {
                requested,
                minimum,
                maximum,
            } => write!(
                f,
                "invalid SIMD lane width {requested}; supported range is \
                 {minimum}..={maximum}"
            ),

            Self::LengthMismatch { left, right } => write!(
                f,
                "SIMD input length mismatch: left={left}, right={right}"
            ),

            Self::TernaryLengthMismatch {
                first,
                second,
                third,
            } => write!(
                f,
                "SIMD ternary input length mismatch: first={first}, \
                 second={second}, third={third}"
            ),

            Self::DestinationTooSmall {
                required,
                available,
            } => write!(
                f,
                "SIMD destination too small: required={required}, \
                 available={available}"
            ),

            Self::InvalidRange { start, end, len } => write!(
                f,
                "invalid SIMD range {start}..{end} for length {len}"
            ),

            Self::ArithmeticOverflow => {
                f.write_str("SIMD arithmetic overflow")
            }

            Self::NonFiniteResult => {
                f.write_str("SIMD operation produced a non-finite result")
            }

            Self::EmptyReduction => {
                f.write_str("SIMD reduction cannot operate on an empty input")
            }

            Self::UnsupportedOperation { operation } => {
                write!(f, "unsupported SIMD operation: {operation}")
            }
        }
    }
}

impl std::error::Error for SimdError {}

/// Result type used by this module.
pub type SimdResult<T> = Result<T, SimdError>;

// =============================================================================
// Execution path
// =============================================================================

/// Describes the logical execution strategy selected by the portable layer.
///
/// These values do not claim that a particular hardware instruction set is
/// being used. `Vectorized` means the operation is processed in bounded chunks
/// suitable for compiler vectorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionPath {
    /// Process one element at a time.
    Scalar,

    /// Process bounded logical chunks.
    Vectorized,
}

impl ExecutionPath {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scalar => "scalar",
            Self::Vectorized => "vectorized",
        }
    }

    /// Returns whether this is the scalar fallback.
    pub const fn is_scalar(self) -> bool {
        matches!(self, Self::Scalar)
    }

    /// Returns whether this is the chunked/vectorizable path.
    pub const fn is_vectorized(self) -> bool {
        matches!(self, Self::Vectorized)
    }
}

impl fmt::Display for ExecutionPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Vector width
// =============================================================================

/// Safe logical lane-width configuration.
///
/// This does not represent a CPU register width. It controls how the portable
/// implementation groups work so that optimized compilers have clear bounded
/// loops to vectorize.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LaneWidth(usize);

impl LaneWidth {
    /// Creates a validated lane width.
    pub const fn new(width: usize) -> Option<Self> {
        if width < MIN_LANE_WIDTH || width > MAX_LANE_WIDTH {
            None
        } else {
            Some(Self(width))
        }
    }

    /// Returns the default lane width.
    pub const fn default_width() -> Self {
        Self(DEFAULT_LANE_WIDTH)
    }

    /// Returns the configured width.
    pub const fn get(self) -> usize {
        self.0
    }

    /// Returns whether this is the scalar width.
    pub const fn is_scalar(self) -> bool {
        self.0 == 1
    }

    /// Returns the execution path represented by this width.
    pub const fn execution_path(self) -> ExecutionPath {
        if self.0 == 1 {
            ExecutionPath::Scalar
        } else {
            ExecutionPath::Vectorized
        }
    }
}

impl Default for LaneWidth {
    fn default() -> Self {
        Self::default_width()
    }
}

impl TryFrom<usize> for LaneWidth {
    type Error = SimdError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(SimdError::InvalidLaneWidth {
            requested: value,
            minimum: MIN_LANE_WIDTH,
            maximum: MAX_LANE_WIDTH,
        })
    }
}

impl fmt::Display for LaneWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} lanes", self.0)
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Configuration for safe vectorized operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimdConfig {
    lane_width: LaneWidth,
    force_scalar: bool,
}

impl SimdConfig {
    /// Creates the default configuration.
    pub const fn new() -> Self {
        Self {
            lane_width: LaneWidth::default_width(),
            force_scalar: false,
        }
    }

    /// Creates a configuration with an explicit lane width.
    pub fn with_lane_width(width: usize) -> SimdResult<Self> {
        Ok(Self {
            lane_width: LaneWidth::try_from(width)?,
            force_scalar: false,
        })
    }

    /// Creates a scalar-only configuration.
    pub const fn scalar() -> Self {
        Self {
            lane_width: LaneWidth(1),
            force_scalar: true,
        }
    }

    /// Returns the configured lane width.
    pub const fn lane_width(self) -> LaneWidth {
        self.lane_width
    }

    /// Returns whether scalar execution is forced.
    pub const fn force_scalar(self) -> bool {
        self.force_scalar
    }

    /// Forces scalar execution.
    pub const fn with_scalar_fallback(self, force: bool) -> Self {
        Self {
            lane_width: self.lane_width,
            force_scalar: force,
        }
    }

    /// Returns the effective execution path.
    pub const fn execution_path(self) -> ExecutionPath {
        if self.force_scalar || self.lane_width.is_scalar() {
            ExecutionPath::Scalar
        } else {
            ExecutionPath::Vectorized
        }
    }
}

impl Default for SimdConfig {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Capability metadata
// =============================================================================

/// CPU/vectorization capability classification.
///
/// This is deliberately conservative. It describes what this safe module can
/// guarantee rather than trying to identify every processor instruction.
///
/// Architecture-specific feature discovery belongs in `memory::cpu`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimdCapability {
    /// Scalar execution is guaranteed.
    Scalar,

    /// Compiler-directed vectorization is available.
    CompilerVectorization,

    /// Architecture-specific acceleration may be supplied by another provider.
    ExternalProvider,
}

impl SimdCapability {
    /// Stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scalar => "scalar",
            Self::CompilerVectorization => "compiler_vectorization",
            Self::ExternalProvider => "external_provider",
        }
    }
}

impl fmt::Display for SimdCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Describes the capabilities guaranteed by this module on every supported
/// platform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimdCapabilities {
    capability: SimdCapability,
    maximum_safe_lane_width: LaneWidth,
}

impl SimdCapabilities {
    /// Returns capabilities for the current safe implementation.
    ///
    /// The implementation never assumes a physical SIMD instruction width.
    pub const fn current() -> Self {
        Self {
            capability: SimdCapability::CompilerVectorization,
            maximum_safe_lane_width: LaneWidth(MAX_LANE_WIDTH),
        }
    }

    /// Returns the capability classification.
    pub const fn capability(self) -> SimdCapability {
        self.capability
    }

    /// Returns the maximum logical lane width accepted.
    pub const fn maximum_safe_lane_width(self) -> LaneWidth {
        self.maximum_safe_lane_width
    }

    /// Returns whether the safe vectorized path is available.
    pub const fn vectorization_available(self) -> bool {
        matches!(
            self.capability,
            SimdCapability::CompilerVectorization
                | SimdCapability::ExternalProvider
        )
    }
}

// =============================================================================
// Generic scalar abstraction
// =============================================================================

/// Scalar values supported by the safe numerical kernels.
///
/// This deliberately stays small. It avoids imposing a broad numeric trait
/// ecosystem on the memory layer and keeps the module dependency-free.
pub trait SimdScalar:
    Copy
    + Clone
    + Default
    + Send
    + Sync
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
{
    /// Returns the additive identity.
    fn zero() -> Self;

    /// Returns the multiplicative identity.
    fn one() -> Self;

    /// Returns the absolute value as `f64`.
    fn abs_f64(self) -> f64;

    /// Returns whether the scalar is finite.
    fn is_finite(self) -> bool;
}

impl SimdScalar for f32 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn abs_f64(self) -> f64 {
        self.abs() as f64
    }

    fn is_finite(self) -> bool {
        self.is_finite()
    }
}

impl SimdScalar for f64 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn abs_f64(self) -> f64 {
        self.abs()
    }

    fn is_finite(self) -> bool {
        self.is_finite()
    }
}

// =============================================================================
// Complex scalar
// =============================================================================

/// Safe complex scalar used by the memory SIMD kernels.
///
/// Zamani intentionally does not depend on an external complex-number crate
/// here because the memory layer should remain independently usable.
///
/// Higher-level `memory::complex` may provide richer complex semantics and can
/// adapt to this representation.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Complex<T> {
    /// Real component.
    pub re: T,

    /// Imaginary component.
    pub im: T,
}

impl<T> Complex<T> {
    /// Creates a complex number.
    pub const fn new(re: T, im: T) -> Self {
        Self { re, im }
    }
}

impl<T> Complex<T>
where
    T: SimdScalar,
{
    /// Returns zero.
    pub fn zero() -> Self {
        Self {
            re: T::zero(),
            im: T::zero(),
        }
    }

    /// Returns one.
    pub fn one() -> Self {
        Self {
            re: T::one(),
            im: T::zero(),
        }
    }

    /// Returns the squared magnitude.
    pub fn norm_squared(self) -> T {
        self.re * self.re + self.im * self.im
    }

    /// Returns the complex conjugate.
    pub fn conjugate(self) -> Self {
        Self {
            re: self.re,
            im: T::zero() - self.im,
        }
    }

    /// Returns whether both components are finite.
    pub fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }
}

impl<T> Add for Complex<T>
where
    T: SimdScalar,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> Sub for Complex<T>
where
    T: SimdScalar,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl<T> Mul for Complex<T>
where
    T: SimdScalar,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

// =============================================================================
// Work partitioning
// =============================================================================

/// A bounded logical chunk of work.
///
/// This is a value type and does not contain references, making it safe to
/// pass between planning and execution layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkChunk {
    /// Inclusive start.
    pub start: usize,

    /// Exclusive end.
    pub end: usize,
}

impl WorkChunk {
    /// Creates a validated chunk.
    pub fn new(start: usize, end: usize) -> SimdResult<Self> {
        if start > end {
            return Err(SimdError::InvalidRange {
                start,
                end,
                len: end,
            });
        }

        Ok(Self { start, end })
    }

    /// Returns the number of elements in the chunk.
    pub const fn len(self) -> usize {
        self.end - self.start
    }

    /// Returns whether the chunk is empty.
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}

/// Iterator over bounded work chunks.
#[derive(Debug, Clone)]
pub struct WorkChunks {
    next: usize,
    end: usize,
    width: usize,
}

impl WorkChunks {
    /// Creates a chunk iterator.
    pub fn new(len: usize, width: LaneWidth) -> Self {
        Self {
            next: 0,
            end: len,
            width: width.get(),
        }
    }
}

impl Iterator for WorkChunks {
    type Item = WorkChunk;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.end {
            return None;
        }

        let start = self.next;
        let remaining = self.end - start;
        let size = remaining.min(self.width);
        let end = start + size;

        self.next = end;

        Some(WorkChunk { start, end })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end.saturating_sub(self.next);

        if remaining == 0 {
            return (0, Some(0));
        }

        let count = (remaining + self.width - 1) / self.width;
        (count, Some(count))
    }
}

// =============================================================================
// Planning
// =============================================================================

/// Vectorization execution plan.
///
/// A plan is immutable and can be reused for multiple operations over slices
/// of the same length.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimdPlan {
    length: usize,
    config: SimdConfig,
}

impl SimdPlan {
    /// Creates a plan.
    pub fn new(length: usize, config: SimdConfig) -> Self {
        Self { length, config }
    }

    /// Creates a plan using the default configuration.
    pub const fn default_for(length: usize) -> Self {
        Self {
            length,
            config: SimdConfig::new(),
        }
    }

    /// Returns the planned element count.
    pub const fn len(self) -> usize {
        self.length
    }

    /// Returns the configuration.
    pub const fn config(self) -> SimdConfig {
        self.config
    }

    /// Returns the selected execution path.
    pub const fn execution_path(self) -> ExecutionPath {
        self.config.execution_path()
    }

    /// Returns the lane width.
    pub const fn lane_width(self) -> LaneWidth {
        self.config.lane_width()
    }

    /// Returns an iterator over the planned chunks.
    pub fn chunks(self) -> WorkChunks {
        WorkChunks::new(self.length, self.lane_width())
    }
}

// =============================================================================
// Generic execution
// =============================================================================

/// Applies a unary operation to every element of a slice.
pub fn for_each<T, F>(
    input: &[T],
    output: &mut [T],
    config: SimdConfig,
    mut operation: F,
) -> SimdResult<()>
where
    T: Copy,
    F: FnMut(T) -> T,
{
    if input.len() != output.len() {
        return Err(SimdError::LengthMismatch {
            left: input.len(),
            right: output.len(),
        });
    }

    let plan = SimdPlan::new(input.len(), config);

    for chunk in plan.chunks() {
        let input_chunk = &input[chunk.start..chunk.end];
        let output_chunk = &mut output[chunk.start..chunk.end];

        for (source, destination) in
            input_chunk.iter().copied().zip(output_chunk.iter_mut())
        {
            *destination = operation(source);
        }
    }

    Ok(())
}

/// Applies a binary operation element-by-element.
pub fn zip_map<T, F>(
    left: &[T],
    right: &[T],
    output: &mut [T],
    config: SimdConfig,
    mut operation: F,
) -> SimdResult<()>
where
    T: Copy,
    F: FnMut(T, T) -> T,
{
    if left.len() != right.len() {
        return Err(SimdError::LengthMismatch {
            left: left.len(),
            right: right.len(),
        });
    }

    if left.len() != output.len() {
        return Err(SimdError::LengthMismatch {
            left: left.len(),
            right: output.len(),
        });
    }

    let plan = SimdPlan::new(left.len(), config);

    for chunk in plan.chunks() {
        let left_chunk = &left[chunk.start..chunk.end];
        let right_chunk = &right[chunk.start..chunk.end];
        let output_chunk = &mut output[chunk.start..chunk.end];

        for ((lhs, rhs), destination) in left_chunk
            .iter()
            .copied()
            .zip(right_chunk.iter().copied())
            .zip(output_chunk.iter_mut())
        {
            *destination = operation(lhs, rhs);
        }
    }

    Ok(())
}

/// Applies a ternary operation element-by-element.
pub fn zip_map3<T, F>(
    first: &[T],
    second: &[T],
    third: &[T],
    output: &mut [T],
    config: SimdConfig,
    mut operation: F,
) -> SimdResult<()>
where
    T: Copy,
    F: FnMut(T, T, T) -> T,
{
    if first.len() != second.len() || first.len() != third.len() {
        return Err(SimdError::TernaryLengthMismatch {
            first: first.len(),
            second: second.len(),
            third: third.len(),
        });
    }

    if first.len() != output.len() {
        return Err(SimdError::LengthMismatch {
            left: first.len(),
            right: output.len(),
        });
    }

    let plan = SimdPlan::new(first.len(), config);

    for chunk in plan.chunks() {
        let first_chunk = &first[chunk.start..chunk.end];
        let second_chunk = &second[chunk.start..chunk.end];
        let third_chunk = &third[chunk.start..chunk.end];
        let output_chunk = &mut output[chunk.start..chunk.end];

        for (((a, b), c), destination) in first_chunk
            .iter()
            .copied()
            .zip(second_chunk.iter().copied())
            .zip(third_chunk.iter().copied())
            .zip(output_chunk.iter_mut())
        {
            *destination = operation(a, b, c);
        }
    }

    Ok(())
}

// =============================================================================
// Copy operations
// =============================================================================

/// Copies one slice into another.
///
/// This operation is safe for overlapping regions when the caller provides
/// slices that alias through Rust's normal borrowing rules.
pub fn copy<T: Copy>(source: &[T], destination: &mut [T]) -> SimdResult<()> {
    if source.len() != destination.len() {
        return Err(SimdError::LengthMismatch {
            left: source.len(),
            right: destination.len(),
        });
    }

    destination.copy_from_slice(source);
    Ok(())
}

/// Copies a bounded range from one slice to another.
pub fn copy_range<T: Copy>(
    source: &[T],
    source_range: std::ops::Range<usize>,
    destination: &mut [T],
) -> SimdResult<()> {
    validate_range(source_range.start, source_range.end, source.len())?;

    let length = source_range.end - source_range.start;

    if destination.len() < length {
        return Err(SimdError::DestinationTooSmall {
            required: length,
            available: destination.len(),
        });
    }

    destination[..length]
        .copy_from_slice(&source[source_range.start..source_range.end]);

    Ok(())
}

// =============================================================================
// Real arithmetic kernels
// =============================================================================

/// Elementwise addition.
pub fn add<T>(
    left: &[T],
    right: &[T],
    output: &mut [T],
    config: SimdConfig,
) -> SimdResult<()>
where
    T: SimdScalar,
{
    zip_map(left, right, output, config, |a, b| a + b)
}

/// Elementwise subtraction.
pub fn sub<T>(
    left: &[T],
    right: &[T],
    output: &mut [T],
    config: SimdConfig,
) -> SimdResult<()>
where
    T: SimdScalar,
{
    zip_map(left, right, output, config, |a, b| a - b)
}

/// Elementwise multiplication.
pub fn mul<T>(
    left: &[T],
    right: &[T],
    output: &mut [T],
    config: SimdConfig,
) -> SimdResult<()>
where
    T: SimdScalar,
{
    zip_map(left, right, output, config, |a, b| a * b)
}

/// Scales every element by a scalar.
pub fn scale<T>(
    input: &[T],
    scalar: T,
    output: &mut [T],
    config: SimdConfig,
) -> SimdResult<()>
where
    T: SimdScalar,
{
    for_each(input, output, config, |value| value * scalar)
}

/// Fused multiply-add at the logical operation level.
///
/// The compiler may fuse this operation where the target and optimization
/// settings permit it. The API does not depend on hardware FMA.
pub fn mul_add<T>(
    first: &[T],
    second: &[T],
    addend: &[T],
    output: &mut [T],
    config: SimdConfig,
) -> SimdResult<()>
where
    T: SimdScalar,
{
    zip_map3(
        first,
        second,
        addend,
        output,
        config,
        |a, b, c| a * b + c,
    )
}

// =============================================================================
// Reductions
// =============================================================================

/// Sums a slice.
pub fn sum<T>(input: &[T], config: SimdConfig) -> SimdResult<T>
where
    T: SimdScalar,
{
    if input.is_empty() {
        return Err(SimdError::EmptyReduction);
    }

    let mut result = T::zero();

    let plan = SimdPlan::new(input.len(), config);

    for chunk in plan.chunks() {
        for value in &input[chunk.start..chunk.end] {
            result = result + *value;
        }
    }

    Ok(result)
}

/// Calculates the sum of squared magnitudes.
///
/// This is a central kernel for quantum-state normalization and probability
/// calculations.
pub fn sum_abs_squared<T>(input: &[T], config: SimdConfig) -> SimdResult<f64>
where
    T: SimdScalar,
{
    if input.is_empty() {
        return Err(SimdError::EmptyReduction);
    }

    let mut result = 0.0_f64;

    let plan = SimdPlan::new(input.len(), config);

    for chunk in plan.chunks() {
        for value in &input[chunk.start..chunk.end] {
            let magnitude = value.abs_f64();

            if !magnitude.is_finite() {
                return Err(SimdError::NonFiniteResult);
            }

            result += magnitude * magnitude;
        }
    }

    Ok(result)
}

/// Returns the maximum absolute value.
pub fn max_abs<T>(input: &[T], config: SimdConfig) -> SimdResult<f64>
where
    T: SimdScalar,
{
    if input.is_empty() {
        return Err(SimdError::EmptyReduction);
    }

    let mut maximum = 0.0_f64;

    let plan = SimdPlan::new(input.len(), config);

    for chunk in plan.chunks() {
        for value in &input[chunk.start..chunk.end] {
            let magnitude = value.abs_f64();

            if !magnitude.is_finite() {
                return Err(SimdError::NonFiniteResult);
            }

            if magnitude > maximum {
                maximum = magnitude;
            }
        }
    }

    Ok(maximum)
}

// =============================================================================
// Complex kernels
// =============================================================================

/// Elementwise complex addition.
pub fn complex_add<T>(
    left: &[Complex<T>],
    right: &[Complex<T>],
    output: &mut [Complex<T>],
    config: SimdConfig,
) -> SimdResult<()>
where
    T: SimdScalar,
{
    zip_map(left, right, output, config, |a, b| a + b)
}

/// Elementwise complex subtraction.
pub fn complex_sub<T>(
    left: &[Complex<T>],
    right: &[Complex<T>],
    output: &mut [Complex<T>],
    config: SimdConfig,
) -> SimdResult<()>
where
    T: SimdScalar,
{
    zip_map(left, right, output, config, |a, b| a - b)
}

/// Elementwise complex multiplication.
pub fn complex_mul<T>(
    left: &[Complex<T>],
    right: &[Complex<T>],
    output: &mut [Complex<T>],
    config: SimdConfig,
) -> SimdResult<()>
where
    T: SimdScalar,
{
    zip_map(left, right, output, config, |a, b| a * b)
}

/// Complex scalar multiplication.
pub fn complex_scale<T>(
    input: &[Complex<T>],
    scalar: Complex<T>,
    output: &mut [Complex<T>],
    config: SimdConfig,
) -> SimdResult<()>
where
    T: SimdScalar,
{
    for_each(input, output, config, |value| value * scalar)
}

/// Complex multiply-add.
///
/// Calculates:
///
/// ```text
/// output[i] = first[i] * second[i] + addend[i]
/// ```
pub fn complex_mul_add<T>(
    first: &[Complex<T>],
    second: &[Complex<T>],
    addend: &[Complex<T>],
    output: &mut [Complex<T>],
    config: SimdConfig,
) -> SimdResult<()>
where
    T: SimdScalar,
{
    zip_map3(
        first,
        second,
        addend,
        output,
        config,
        |a, b, c| a * b + c,
    )
}

/// Calculates complex squared magnitudes.
pub fn complex_norm_squared<T>(
    input: &[Complex<T>],
    output: &mut [T],
    config: SimdConfig,
) -> SimdResult<()>
where
    T: SimdScalar,
{
    if input.len() != output.len() {
        return Err(SimdError::LengthMismatch {
            left: input.len(),
            right: output.len(),
        });
    }

    for_each(input, output, config, |value| value.norm_squared())
}

/// Calculates the total probability represented by a complex state vector.
pub fn complex_probability_sum<T>(
    input: &[Complex<T>],
    config: SimdConfig,
) -> SimdResult<f64>
where
    T: SimdScalar,
{
    if input.is_empty() {
        return Err(SimdError::EmptyReduction);
    }

    let mut result = 0.0_f64;

    let plan = SimdPlan::new(input.len(), config);

    for chunk in plan.chunks() {
        for value in &input[chunk.start..chunk.end] {
            if !value.is_finite() {
                return Err(SimdError::NonFiniteResult);
            }

            let re = value.re.abs_f64();
            let im = value.im.abs_f64();

            result += re * re + im * im;
        }
    }

    if !result.is_finite() {
        return Err(SimdError::NonFiniteResult);
    }

    Ok(result)
}

/// Scales a complex state vector by a real scalar.
pub fn complex_real_scale<T>(
    input: &[Complex<T>],
    scalar: T,
    output: &mut [Complex<T>],
    config: SimdConfig,
) -> SimdResult<()>
where
    T: SimdScalar,
{
    for_each(input, output, config, |value| Complex {
        re: value.re * scalar,
        im: value.im * scalar,
    })
}

// =============================================================================
// State-vector pair kernel
// =============================================================================

/// Applies a general 2×2 complex matrix to pairs of state-vector amplitudes.
///
/// For each pair `(a0, a1)`:
///
/// ```text
/// out0 = m00*a0 + m01*a1
/// out1 = m10*a0 + m11*a1
/// ```
///
/// This is one of the most important CPU kernels for state-vector simulation.
///
/// The caller supplies already paired amplitudes. Qubit indexing and pairing
/// are intentionally outside this module.
pub fn complex_matrix_2x2<T>(
    input_zero: &[Complex<T>],
    input_one: &[Complex<T>],
    output_zero: &mut [Complex<T>],
    output_one: &mut [Complex<T>],
    matrix: [[Complex<T>; 2]; 2],
    config: SimdConfig,
) -> SimdResult<()>
where
    T: SimdScalar,
{
    let len = input_zero.len();

    if input_one.len() != len {
        return Err(SimdError::LengthMismatch {
            left: len,
            right: input_one.len(),
        });
    }

    if output_zero.len() != len {
        return Err(SimdError::LengthMismatch {
            left: len,
            right: output_zero.len(),
        });
    }

    if output_one.len() != len {
        return Err(SimdError::LengthMismatch {
            left: len,
            right: output_one.len(),
        });
    }

    let plan = SimdPlan::new(len, config);

    for chunk in plan.chunks() {
        let a0 = &input_zero[chunk.start..chunk.end];
        let a1 = &input_one[chunk.start..chunk.end];

        let b0 = &mut output_zero[chunk.start..chunk.end];
        let b1 = &mut output_one[chunk.start..chunk.end];

        for (((x0, x1), y0), y1) in a0
            .iter()
            .copied()
            .zip(a1.iter().copied())
            .zip(b0.iter_mut())
            .zip(b1.iter_mut())
        {
            *y0 = matrix[0][0] * x0 + matrix[0][1] * x1;
            *y1 = matrix[1][0] * x0 + matrix[1][1] * x1;
        }
    }

    Ok(())
}

/// In-place 2×2 complex matrix application over adjacent logical pairs.
///
/// The input slice must contain an even number of amplitudes.
///
/// This function uses a temporary pair value rather than unsafe overlapping
/// mutable references, preserving Rust's aliasing guarantees.
pub fn complex_matrix_2x2_in_place<T>(
    amplitudes: &mut [Complex<T>],
    matrix: [[Complex<T>; 2]; 2],
    config: SimdConfig,
) -> SimdResult<()>
where
    T: SimdScalar,
{
    if amplitudes.len() % 2 != 0 {
        return Err(SimdError::InvalidRange {
            start: 0,
            end: amplitudes.len(),
            len: amplitudes.len(),
        });
    }

    let pair_count = amplitudes.len() / 2;
    let plan = SimdPlan::new(pair_count, config);

    for chunk in plan.chunks() {
        let start_pair = chunk.start;
        let end_pair = chunk.end;

        for pair_index in start_pair..end_pair {
            let base = pair_index
                .checked_mul(2)
                .ok_or(SimdError::ArithmeticOverflow)?;

            let a0 = amplitudes[base];
            let a1 = amplitudes[base + 1];

            let b0 = matrix[0][0] * a0 + matrix[0][1] * a1;
            let b1 = matrix[1][0] * a0 + matrix[1][1] * a1;

            amplitudes[base] = b0;
            amplitudes[base + 1] = b1;
        }
    }

    Ok(())
}

// =============================================================================
// Normalization
// =============================================================================

/// Normalizes a real vector by its Euclidean norm.
///
/// This operation is intentionally explicit. It does not silently normalize
/// inputs from higher-level quantum-state code.
pub fn normalize<T>(
    input: &[T],
    output: &mut [T],
    config: SimdConfig,
) -> SimdResult<f64>
where
    T: SimdScalar,
{
    if input.len() != output.len() {
        return Err(SimdError::LengthMismatch {
            left: input.len(),
            right: output.len(),
        });
    }

    if input.is_empty() {
        return Err(SimdError::EmptyReduction);
    }

    let squared_norm = sum_abs_squared(input, config)?;

    if !squared_norm.is_finite() || squared_norm <= 0.0 {
        return Err(SimdError::NonFiniteResult);
    }

    let norm = squared_norm.sqrt();

    if !norm.is_finite() || norm <= 0.0 {
        return Err(SimdError::NonFiniteResult);
    }

    // The generic scalar trait intentionally exposes no conversion from f64.
    // Use the f64-specialized implementations below for actual normalized
    // real-valued output.
    normalize_real_impl(input, output, norm, config)?;

    Ok(norm)
}

fn normalize_real_impl<T>(
    input: &[T],
    output: &mut [T],
    norm: f64,
    config: SimdConfig,
) -> SimdResult<()>
where
    T: SimdScalar,
{
    // Generic scalar normalization cannot safely manufacture T from f64
    // without introducing a lossy or ambiguous conversion contract.
    //
    // Therefore only f32/f64 are exposed through the public specialized
    // functions below. This generic helper exists only to keep the numerical
    // validation centralized.
    let _ = (input, output, norm, config);

    Err(SimdError::UnsupportedOperation {
        operation: "generic-normalize; use normalize_f32 or normalize_f64",
    })
}

/// Normalizes an f32 vector.
pub fn normalize_f32(
    input: &[f32],
    output: &mut [f32],
    config: SimdConfig,
) -> SimdResult<f64> {
    if input.len() != output.len() {
        return Err(SimdError::LengthMismatch {
            left: input.len(),
            right: output.len(),
        });
    }

    if input.is_empty() {
        return Err(SimdError::EmptyReduction);
    }

    let squared_norm = sum_abs_squared(input, config)?;

    if !squared_norm.is_finite() || squared_norm <= 0.0 {
        return Err(SimdError::NonFiniteResult);
    }

    let norm = squared_norm.sqrt();

    for_each(input, output, config, |value| value / norm as f32)?;

    Ok(norm)
}

/// Normalizes an f64 vector.
pub fn normalize_f64(
    input: &[f64],
    output: &mut [f64],
    config: SimdConfig,
) -> SimdResult<f64> {
    if input.len() != output.len() {
        return Err(SimdError::LengthMismatch {
            left: input.len(),
            right: output.len(),
        });
    }

    if input.is_empty() {
        return Err(SimdError::EmptyReduction);
    }

    let squared_norm = sum_abs_squared(input, config)?;

    if !squared_norm.is_finite() || squared_norm <= 0.0 {
        return Err(SimdError::NonFiniteResult);
    }

    let norm = squared_norm.sqrt();

    for_each(input, output, config, |value| value / norm)?;

    Ok(norm)
}

/// Normalizes a complex f32 vector.
pub fn normalize_complex_f32(
    input: &[Complex<f32>],
    output: &mut [Complex<f32>],
    config: SimdConfig,
) -> SimdResult<f64> {
    if input.len() != output.len() {
        return Err(SimdError::LengthMismatch {
            left: input.len(),
            right: output.len(),
        });
    }

    if input.is_empty() {
        return Err(SimdError::EmptyReduction);
    }

    let squared_norm = complex_probability_sum(input, config)?;

    if !squared_norm.is_finite() || squared_norm <= 0.0 {
        return Err(SimdError::NonFiniteResult);
    }

    let norm = squared_norm.sqrt();

    complex_real_scale(input, 1.0_f32 / norm as f32, output, config)?;

    Ok(norm)
}

/// Normalizes a complex f64 vector.
pub fn normalize_complex_f64(
    input: &[Complex<f64>],
    output: &mut [Complex<f64>],
    config: SimdConfig,
) -> SimdResult<f64> {
    if input.len() != output.len() {
        return Err(SimdError::LengthMismatch {
            left: input.len(),
            right: output.len(),
        });
    }

    if input.is_empty() {
        return Err(SimdError::EmptyReduction);
    }

    let squared_norm = complex_probability_sum(input, config)?;

    if !squared_norm.is_finite() || squared_norm <= 0.0 {
        return Err(SimdError::NonFiniteResult);
    }

    let norm = squared_norm.sqrt();

    complex_real_scale(input, 1.0_f64 / norm, output, config)?;

    Ok(norm)
}

// =============================================================================
// Validation
// =============================================================================

/// Validates a range against a slice length.
pub fn validate_range(start: usize, end: usize, len: usize) -> SimdResult<()> {
    if start > end || end > len {
        return Err(SimdError::InvalidRange { start, end, len });
    }

    Ok(())
}

/// Validates that a length can be split into chunks of a requested width.
pub fn validate_lane_width(width: usize) -> SimdResult<LaneWidth> {
    LaneWidth::try_from(width)
}

/// Returns the number of chunks needed for a length and lane width.
pub fn chunk_count(len: usize, width: LaneWidth) -> SimdResult<usize> {
    if len == 0 {
        return Ok(0);
    }

    let adjusted = len
        .checked_add(width.get() - 1)
        .ok_or(SimdError::ArithmeticOverflow)?;

    Ok(adjusted / width.get())
}

// =============================================================================
// Backend boundary
// =============================================================================

/// Abstract execution provider category.
///
/// This is intentionally metadata-only. It does not contain function
/// pointers, FFI handles, or unsafe operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimdProvider {
    /// Safe portable Rust implementation.
    Portable,

    /// Future architecture-specific implementation supplied elsewhere.
    ArchitectureSpecific,

    /// Future external/provider implementation.
    External,
}

impl SimdProvider {
    /// Stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Portable => "portable",
            Self::ArchitectureSpecific => "architecture_specific",
            Self::External => "external",
        }
    }
}

impl fmt::Display for SimdProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Describes a vector execution backend without binding the memory layer to
/// hardware-specific APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimdBackend {
    provider: SimdProvider,
    capability: SimdCapability,
    lane_width: LaneWidth,
}

impl SimdBackend {
    /// Returns the universally available safe backend.
    pub const fn portable() -> Self {
        Self {
            provider: SimdProvider::Portable,
            capability: SimdCapability::CompilerVectorization,
            lane_width: LaneWidth(DEFAULT_LANE_WIDTH),
        }
    }

    /// Returns the backend provider.
    pub const fn provider(self) -> SimdProvider {
        self.provider
    }

    /// Returns the capability.
    pub const fn capability(self) -> SimdCapability {
        self.capability
    }

    /// Returns the configured logical lane width.
    pub const fn lane_width(self) -> LaneWidth {
        self.lane_width
    }
}

impl Default for SimdBackend {
    fn default() -> Self {
        Self::portable()
    }
}

// =============================================================================
// Statistics
// =============================================================================

/// Lightweight execution statistics.
///
/// This type is intentionally independent of `memory::telemetry`.
///
/// Telemetry may later consume it, but SIMD execution does not depend on
/// telemetry being enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SimdStatistics {
    /// Number of logical elements processed.
    elements: u64,

    /// Number of logical chunks processed.
    chunks: u64,

    /// Number of scalar operations forced by configuration.
    scalar_operations: u64,
}

impl SimdStatistics {
    /// Creates empty statistics.
    pub const fn new() -> Self {
        Self {
            elements: 0,
            chunks: 0,
            scalar_operations: 0,
        }
    }

    /// Adds one operation's accounting information.
    pub fn record(
        &mut self,
        elements: usize,
        chunks: usize,
        path: ExecutionPath,
    ) -> SimdResult<()> {
        self.elements = self
            .elements
            .checked_add(elements as u64)
            .ok_or(SimdError::ArithmeticOverflow)?;

        self.chunks = self
            .chunks
            .checked_add(chunks as u64)
            .ok_or(SimdError::ArithmeticOverflow)?;

        if path.is_scalar() {
            self.scalar_operations = self
                .scalar_operations
                .checked_add(elements as u64)
                .ok_or(SimdError::ArithmeticOverflow)?;
        }

        Ok(())
    }

    /// Returns processed element count.
    pub const fn elements(self) -> u64 {
        self.elements
    }

    /// Returns processed chunk count.
    pub const fn chunks(self) -> u64 {
        self.chunks
    }

    /// Returns scalar operation count.
    pub const fn scalar_operations(self) -> u64 {
        self.scalar_operations
    }
}

// =============================================================================
// Public convenience API
// =============================================================================

/// Returns the current safe SIMD capability description.
pub const fn capabilities() -> SimdCapabilities {
    SimdCapabilities::current()
}

/// Returns the default portable backend.
pub const fn default_backend() -> SimdBackend {
    SimdBackend::portable()
}

/// Returns the default execution configuration.
pub const fn default_config() -> SimdConfig {
    SimdConfig::new()
}

/// Creates a reusable execution plan.
pub const fn plan(length: usize) -> SimdPlan {
    SimdPlan::default_for(length)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lane_width_validation_is_bounded() {
        assert!(LaneWidth::new(1).is_some());
        assert!(LaneWidth::new(DEFAULT_LANE_WIDTH).is_some());
        assert!(LaneWidth::new(MAX_LANE_WIDTH).is_some());
        assert!(LaneWidth::new(0).is_none());
        assert!(LaneWidth::new(MAX_LANE_WIDTH + 1).is_none());
    }

    #[test]
    fn scalar_configuration_is_scalar() {
        let config = SimdConfig::scalar();

        assert_eq!(config.execution_path(), ExecutionPath::Scalar);
        assert_eq!(config.lane_width().get(), 1);
        assert!(config.force_scalar());
    }

    #[test]
    fn default_configuration_is_vectorizable() {
        let config = SimdConfig::default();

        assert_eq!(
            config.execution_path(),
            ExecutionPath::Vectorized
        );
        assert!(config.lane_width().get() > 1);
    }

    #[test]
    fn chunks_cover_all_elements() {
        let chunks = WorkChunks::new(17, LaneWidth::new(8).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], WorkChunk { start: 0, end: 8 });
        assert_eq!(chunks[1], WorkChunk { start: 8, end: 16 });
        assert_eq!(chunks[2], WorkChunk { start: 16, end: 17 });
    }

    #[test]
    fn chunks_handle_empty_input() {
        let chunks = WorkChunks::new(0, LaneWidth::new(8).unwrap())
            .collect::<Vec<_>>();

        assert!(chunks.is_empty());
    }

    #[test]
    fn add_works() {
        let left = [1.0_f64, 2.0, 3.0, 4.0];
        let right = [5.0_f64, 6.0, 7.0, 8.0];
        let mut output = [0.0_f64; 4];

        add(
            &left,
            &right,
            &mut output,
            SimdConfig::default(),
        )
        .unwrap();

        assert_eq!(output, [6.0, 8.0, 10.0, 12.0]);
    }

    #[test]
    fn subtraction_works() {
        let left = [5.0_f64, 6.0, 7.0, 8.0];
        let right = [1.0_f64, 2.0, 3.0, 4.0];
        let mut output = [0.0_f64; 4];

        sub(
            &left,
            &right,
            &mut output,
            SimdConfig::default(),
        )
        .unwrap();

        assert_eq!(output, [4.0, 4.0, 4.0, 4.0]);
    }

    #[test]
    fn multiplication_works() {
        let left = [1.0_f64, 2.0, 3.0, 4.0];
        let right = [2.0_f64, 3.0, 4.0, 5.0];
        let mut output = [0.0_f64; 4];

        mul(
            &left,
            &right,
            &mut output,
            SimdConfig::default(),
        )
        .unwrap();

        assert_eq!(output, [2.0, 6.0, 12.0, 20.0]);
    }

    #[test]
    fn multiply_add_works() {
        let first = [1.0_f64, 2.0, 3.0, 4.0];
        let second = [2.0_f64, 3.0, 4.0, 5.0];
        let addend = [1.0_f64, 1.0, 1.0, 1.0];
        let mut output = [0.0_f64; 4];

        mul_add(
            &first,
            &second,
            &addend,
            &mut output,
            SimdConfig::default(),
        )
        .unwrap();

        assert_eq!(output, [3.0, 7.0, 13.0, 21.0]);
    }

    #[test]
    fn complex_multiplication_works() {
        let left = [Complex::new(1.0_f64, 2.0)];
        let right = [Complex::new(3.0_f64, 4.0)];
        let mut output = [Complex::zero()];

        complex_mul(
            &left,
            &right,
            &mut output,
            SimdConfig::default(),
        )
        .unwrap();

        assert_eq!(output[0], Complex::new(-5.0, 10.0));
    }

    #[test]
    fn complex_probability_sum_works() {
        let values = [
            Complex::new(1.0_f64, 0.0),
            Complex::new(0.0_f64, 1.0),
        ];

        let probability =
            complex_probability_sum(&values, SimdConfig::default())
                .unwrap();

        assert!((probability - 2.0).abs() < 1e-12);
    }

    #[test]
    fn complex_matrix_application_works() {
        let mut values = [
            Complex::new(1.0_f64, 0.0),
            Complex::new(0.0_f64, 0.0),
        ];

        let matrix = [
            [
                Complex::new(0.0_f64, 0.0),
                Complex::new(1.0_f64, 0.0),
            ],
            [
                Complex::new(1.0_f64, 0.0),
                Complex::new(0.0_f64, 0.0),
            ],
        ];

        complex_matrix_2x2_in_place(
            &mut values,
            matrix,
            SimdConfig::default(),
        )
        .unwrap();

        assert_eq!(values[0], Complex::new(0.0, 0.0));
        assert_eq!(values[1], Complex::new(1.0, 0.0));
    }

    #[test]
    fn complex_matrix_hadamard_like_operation() {
        let scale = 1.0_f64 / 2.0_f64.sqrt();

        let matrix = [
            [
                Complex::new(scale, 0.0),
                Complex::new(scale, 0.0),
            ],
            [
                Complex::new(scale, 0.0),
                Complex::new(-scale, 0.0),
            ],
        ];

        let mut values = [
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
        ];

        complex_matrix_2x2_in_place(
            &mut values,
            matrix,
            SimdConfig::default(),
        )
        .unwrap();

        assert!((values[0].re - scale).abs() < 1e-12);
        assert!((values[1].re - scale).abs() < 1e-12);
    }

    #[test]
    fn complex_norm_squared_works() {
        let input = [
            Complex::new(3.0_f64, 4.0),
            Complex::new(5.0_f64, 12.0),
        ];

        let mut output = [0.0_f64; 2];

        complex_norm_squared(
            &input,
            &mut output,
            SimdConfig::default(),
        )
        .unwrap();

        assert_eq!(output, [25.0, 169.0]);
    }

    #[test]
    fn normalization_f64_works() {
        let input = [3.0_f64, 4.0];
        let mut output = [0.0_f64; 2];

        let norm =
            normalize_f64(&input, &mut output, SimdConfig::default())
                .unwrap();

        assert!((norm - 5.0).abs() < 1e-12);
        assert!((output[0] - 0.6).abs() < 1e-12);
        assert!((output[1] - 0.8).abs() < 1e-12);
    }

    #[test]
    fn complex_normalization_works() {
        let input = [
            Complex::new(1.0_f64, 0.0),
            Complex::new(1.0_f64, 0.0),
        ];

        let mut output = [Complex::zero(); 2];

        let norm = normalize_complex_f64(
            &input,
            &mut output,
            SimdConfig::default(),
        )
        .unwrap();

        assert!((norm - 2.0_f64.sqrt()).abs() < 1e-12);
        assert!(
            (output[0].re - 1.0_f64 / 2.0_f64.sqrt()).abs() < 1e-12
        );
        assert!(
            (output[1].re - 1.0_f64 / 2.0_f64.sqrt()).abs() < 1e-12
        );
    }

    #[test]
    fn non_finite_values_are_rejected() {
        let input = [
            Complex::new(f64::NAN, 0.0),
            Complex::new(0.0, 0.0),
        ];

        let result =
            complex_probability_sum(&input, SimdConfig::default());

        assert_eq!(result, Err(SimdError::NonFiniteResult));
    }

    #[test]
    fn empty_reduction_is_rejected() {
        let input: [f64; 0] = [];

        let result = sum(&input, SimdConfig::default());

        assert_eq!(result, Err(SimdError::EmptyReduction));
    }

    #[test]
    fn length_mismatch_is_rejected() {
        let left = [1.0_f64, 2.0];
        let right = [1.0_f64];
        let mut output = [0.0_f64; 2];

        let result =
            add(&left, &right, &mut output, SimdConfig::default());

        assert_eq!(
            result,
            Err(SimdError::LengthMismatch {
                left: 2,
                right: 1,
            })
        );
    }

    #[test]
    fn invalid_range_is_rejected() {
        let result = validate_range(2, 5, 4);

        assert_eq!(
            result,
            Err(SimdError::InvalidRange {
                start: 2,
                end: 5,
                len: 4,
            })
        );
    }

    #[test]
    fn copy_works() {
        let source = [1_u64, 2, 3, 4];
        let mut destination = [0_u64; 4];

        copy(&source, &mut destination).unwrap();

        assert_eq!(source, destination);
    }

    #[test]
    fn backend_is_portable() {
        let backend = SimdBackend::portable();

        assert_eq!(backend.provider(), SimdProvider::Portable);
        assert_eq!(
            backend.capability(),
            SimdCapability::CompilerVectorization
        );
        assert_eq!(
            backend.lane_width().get(),
            DEFAULT_LANE_WIDTH
        );
    }

    #[test]
    fn capabilities_are_available_without_hardware_assumptions() {
        let capabilities = capabilities();

        assert!(capabilities.vectorization_available());
        assert!(
            capabilities.maximum_safe_lane_width().get()
                >= DEFAULT_LANE_WIDTH
        );
    }

    #[test]
    fn statistics_are_checked() {
        let mut statistics = SimdStatistics::new();

        statistics
            .record(16, 2, ExecutionPath::Vectorized)
            .unwrap();

        statistics
            .record(4, 4, ExecutionPath::Scalar)
            .unwrap();

        assert_eq!(statistics.elements(), 20);
        assert_eq!(statistics.chunks(), 6);
        assert_eq!(statistics.scalar_operations(), 4);
    }

    #[test]
    fn plan_is_reusable() {
        let plan = SimdPlan::new(20, SimdConfig::default());

        let first = plan.chunks().collect::<Vec<_>>();
        let second = plan.chunks().collect::<Vec<_>>();

        assert_eq!(first, second);
    }

    #[test]
    fn scalar_and_vectorized_paths_produce_same_result() {
        let left = [1.0_f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let right = [7.0_f64, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];

        let mut scalar = [0.0_f64; 7];
        let mut vectorized = [0.0_f64; 7];

        add(
            &left,
            &right,
            &mut scalar,
            SimdConfig::scalar(),
        )
        .unwrap();

        add(
            &left,
            &right,
            &mut vectorized,
            SimdConfig::default(),
        )
        .unwrap();

        assert_eq!(scalar, vectorized);
    }

    #[test]
    fn state_vector_pair_kernel_preserves_pair_count() {
        let matrix = [
            [Complex::new(1.0_f64, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        ];

        let mut amplitudes = [
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
        ];

        complex_matrix_2x2_in_place(
            &mut amplitudes,
            matrix,
            SimdConfig::default(),
        )
        .unwrap();

        assert_eq!(
            amplitudes,
            [
                Complex::new(1.0, 0.0),
                Complex::new(2.0, 0.0),
                Complex::new(3.0, 0.0),
                Complex::new(4.0, 0.0),
            ]
        );
    }

    #[test]
    fn odd_pair_count_is_rejected() {
        let matrix = [
            [Complex::new(1.0_f64, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        ];

        let mut amplitudes = [
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
        ];

        let result = complex_matrix_2x2_in_place(
            &mut amplitudes,
            matrix,
            SimdConfig::default(),
        );

        assert!(matches!(
            result,
            Err(SimdError::InvalidRange { .. })
        ));
    }
}