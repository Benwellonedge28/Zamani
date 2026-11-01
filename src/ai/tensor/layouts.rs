
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Tensor Layouts
//!
//! Path: `src/ai/tensor/layouts.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Describe target-independent logical tensor layouts for AI computation.
//!
//! This module defines layout intent. It does not allocate tensor storage,
//! select hardware, assign physical addresses, or schedule execution.
//!
//! ## Ownership
//!
//! This module owns:
//! - Logical layout descriptions.
//! - Layout-specific metadata.
//! - Layout metadata validation.
//! - Layout compatibility descriptions.
//! - Layout transformation descriptions.
//! - Extensible layout identifiers.
//!
//! The following remain authoritative elsewhere:
//! - Tensor shapes: frontend::semantic::domains::ai.
//! - Dimension analysis: ai::tensor::dimensions.
//! - Tensor element storage: classical::collections::tensor.
//! - Universal tensor operations: zuir::dialects::tensor.
//! - Physical placement: compiler/runtime/distributed infrastructure.
//!
//! ## POCO-REAF
//!
//! No language-level maximum tensor rank, dimension count, tile count,
//! partition count, device count, or memory capacity is imposed.
//!
//! Metadata collections grow according to available resources.
//!
//! Host representation and allocation failures are explicit limitations,
//! not changes to the meaning of a tensor program.
//!
//! ## Integration
//!
//! - ai::tensor::shapes supplies canonical symbolic shapes.
//! - ai::tensor::dimensions supplies dimension analysis.
//! - ai::tensor::dtypes supplies numerical type descriptions.
//! - ai::tensor::operations consumes layout descriptions.
//! - ai::tensor::interoperability converts validated descriptions.
//! - classical::collections::tensor remains the storage authority.
//! - zuir::dialects::tensor remains the universal tensor IR authority.
//!
//! The grammar entry point is grammar/ai/tensors.g4.
//!
//! ## Important distinction
//!
//! Logical layout is not physical memory placement.
//!
//! A distributed layout describes logical partitioning intent. It does not
//! identify machines, devices, network addresses, or physical allocations.
//!
//! A tiled layout describes logical tile structure. It does not promise
//! that a target has a matching hardware tile.
//!
//! ## Freeze criteria
//!
//! - Layout metadata does not duplicate canonical tensor shapes.
//! - Invalid concrete metadata is rejected.
//! - Unresolved metadata is represented explicitly.
//! - Unknown extensions are never silently interpreted.
//! - No physical placement is encoded.
//! - All public behavior is deterministic.
//! - Error and validation behavior is documented.

use core::fmt;
use std::collections::BTreeMap;

// ============================================================================
// EXTENSIBLE IDENTIFIERS
// ============================================================================

/// Identifies an extensible tensor layout implementation.
///
/// The identifier is a stable logical name, not a hardware vendor name.
///
/// Examples:
///
/// - `org.zamani.tensor.layout.custom`
—
///
/// - `org.example.tensor.layout.blocked`
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LayoutId(String);

impl LayoutId {
    /// Construct a validated layout identifier.
    ///
    /// The identifier must be non-empty and must not contain whitespace.
    /// Names are case-sensitive.
    pub fn new(value: impl Into<String>) -> Result<Self, LayoutError> {
        let value = value.into();

        if value.is_empty() {
            return Err(LayoutError::EmptyIdentifier);
        }

        if value.chars().any(char::is_whitespace) {
            return Err(LayoutError::InvalidIdentifier);
        }

        Ok(Self(value))
    }

    /// Return the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LayoutId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// LOGICAL LAYOUT KINDS
// ============================================================================

/// The logical organization of tensor elements.
///
/// These variants describe computation intent only.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TensorLayoutKind {
    /// Elements are logically dense.
    Dense(DenseLayout),

    /// Elements use an explicit logical stride description.
    Strided(StridedLayout),

    /// Elements use a logical sparse representation.
    Sparse(SparseLayout),

    /// Elements are organized into logical tiles.
    Tiled(TiledLayout),

    /// Elements are partitioned across an abstract collection of partitions.
    ///
    /// This does not identify physical devices or machines.
    Distributed(DistributedLayout),

    /// The concrete layout is intentionally deferred.
    Lazy(LazyLayout),

    /// A registered extension defines the layout semantics.
    Extension(ExtensionLayout),
}

impl TensorLayoutKind {
    /// Return the stable kind name.
    #[must_use]
    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::Dense(_) => "dense",
            Self::Strided(_) => "strided",
            Self::Sparse(_) => "sparse",
            Self::Tiled(_) => "tiled",
            Self::Distributed(_) => "distributed",
            Self::Lazy(_) => "lazy",
            Self::Extension(_) => "extension",
        }
    }

    /// Return whether this layout requires an extension to interpret.
    #[must_use]
    pub fn requires_extension(&self) -> bool {
        matches!(self, Self::Extension(_))
    }
}

// ============================================================================
// DENSE LAYOUT
// ============================================================================

/// Logical dense tensor ordering.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DenseOrder {
    /// The final logical dimension varies fastest.
    RowMajor,

    /// The first logical dimension varies fastest.
    ColumnMajor,

    /// Ordering is specified by an extension or later transformation.
    Unspecified,
}

/// Dense logical layout description.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DenseLayout {
    /// Logical ordering.
    pub order: DenseOrder,
}

impl DenseLayout {
    /// Construct a dense layout with the supplied logical ordering.
    #[must_use]
    pub const fn new(order: DenseOrder) -> Self {
        Self { order }
    }

    /// Construct a row-major dense layout.
    #[must_use]
    pub const fn row_major() -> Self {
        Self::new(DenseOrder::RowMajor)
    }

    /// Construct a column-major dense layout.
    #[must_use]
    pub const fn column_major() -> Self {
        Self::new(DenseOrder::ColumnMajor)
    }
}

// ============================================================================
// STRIDED LAYOUT
// ============================================================================

/// A logical stride value.
///
/// A stride is an abstract logical mapping parameter.
///
/// It is not a physical byte offset or device address.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum StrideValue {
    /// A statically known stride.
    Known(u128),

    /// A stride whose value is unresolved.
    Symbolic(String),
}

/// Logical strided layout.
///
/// The number of stride entries must equal the canonical tensor rank
/// whenever the rank is known.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StridedLayout {
    /// Logical strides, in canonical dimension order.
    pub strides: Vec<StrideValue>,
}

impl StridedLayout {
    /// Construct a strided layout.
    #[must_use]
    pub fn new(strides: Vec<StrideValue>) -> Self {
        Self { strides }
    }
}

// ============================================================================
// SPARSE LAYOUT
// ============================================================================

/// Logical sparse representation family.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SparseFormat {
    /// Coordinate-based sparse representation.
    Coordinate,

    /// Compressed sparse row logical representation.
    CompressedRow,

    /// Compressed sparse column logical representation.
    CompressedColumn,

    /// Block sparse logical representation.
    BlockSparse,

    /// Bit-mask sparse logical representation.
    Mask,

    /// An extensible sparse representation.
    Extension(LayoutId),
}

/// Sparse tensor layout description.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SparseLayout {
    /// Logical sparse representation.
    pub format: SparseFormat,

    /// Optional logical block dimensions.
    ///
    /// Required by block-oriented formats when their semantics require
    /// explicit block dimensions.
    pub block_dimensions: Option<Vec<u128>>,
}

impl SparseLayout {
    /// Construct a sparse layout.
    #[must_use]
    pub fn new(
        format: SparseFormat,
        block_dimensions: Option<Vec<u128>>,
    ) -> Self {
        Self {
            format,
            block_dimensions,
        }
    }
}

// ============================================================================
// TILED LAYOUT
// ============================================================================

/// Logical tiled tensor layout.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TiledLayout {
    /// Logical tile dimensions.
    ///
    /// Each tile dimension must be non-zero.
    pub tile_dimensions: Vec<u128>,

    /// Optional logical ordering of dimensions.
    ///
    /// When supplied, it must be a permutation of the tensor dimensions.
    pub dimension_order: Option<Vec<usize>>,
}

impl TiledLayout {
    /// Construct a tiled layout.
    #[must_use]
    pub fn new(
        tile_dimensions: Vec<u128>,
        dimension_order: Option<Vec<usize>>,
    ) -> Self {
        Self {
            tile_dimensions,
            dimension_order,
        }
    }
}

// ============================================================================
// DISTRIBUTED LAYOUT
// ============================================================================

/// Logical partitioning strategy.
///
/// These are logical partition descriptions, not physical placement rules.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PartitionStrategy {
    /// Partition along one logical dimension.
    AlongDimension {
        dimension: usize,
    },

    /// Partition according to an abstract multidimensional partition shape.
    Multidimensional {
        partition_dimensions: Vec<u128>,
    },

    /// Partitioning is delegated to a downstream planner.
    Deferred,

    /// Extension-defined partitioning.
    Extension(LayoutId),
}

/// Logical distributed tensor layout.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistributedLayout {
    /// Logical partitioning strategy.
    pub strategy: PartitionStrategy,

    /// Whether partition boundaries must preserve logical ordering.
    pub preserve_order: bool,
}

impl DistributedLayout {
    /// Construct a distributed logical layout.
    #[must_use]
    pub fn new(
        strategy: PartitionStrategy,
        preserve_order: bool,
    ) -> Self {
        Self {
            strategy,
            preserve_order,
        }
    }
}

// ============================================================================
// LAZY LAYOUT
// ============================================================================

/// A layout whose concrete organization is deferred.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LazyLayout {
    /// Optional preferred logical layout.
    ///
    /// This is a preference, not a requirement.
    pub preferred: Option<Box<TensorLayoutKind>>,

    /// Whether downstream realization may choose another compatible layout.
    pub allow_reorganization: bool,
}

impl LazyLayout {
    /// Construct a lazy layout.
    #[must_use]
    pub fn new(
        preferred: Option<TensorLayoutKind>,
        allow_reorganization: bool,
    ) -> Self {
        Self {
            preferred: preferred.map(Box::new),
            allow_reorganization,
        }
    }
}

// ============================================================================
// EXTENSION LAYOUT
// ============================================================================

/// Extension-defined logical layout.
///
/// The payload is opaque to this module. Only the registered extension
/// may interpret its semantics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtensionLayout {
    /// Stable extension identifier.
    pub id: LayoutId,

    /// Extension schema version.
    pub version: String,

    /// Opaque extension attributes.
    ///
    /// Keys are ordered for deterministic iteration and serialization.
    pub attributes: BTreeMap<String, String>,
}

impl ExtensionLayout {
    /// Construct an extension layout.
    pub fn new(
        id: LayoutId,
        version: impl Into<String>,
        attributes: BTreeMap<String, String>,
    ) -> Result<Self, LayoutError> {
        let version = version.into();

        if version.is_empty() {
            return Err(LayoutError::EmptyExtensionVersion);
        }

        Ok(Self {
            id,
            version,
            attributes,
        })
    }
}

// ============================================================================
// LAYOUT DESCRIPTOR
// ============================================================================

/// A target-independent logical tensor layout descriptor.
///
/// Shape and dtype are deliberately not duplicated here. They remain owned
/// by canonical semantic and tensor type authorities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorLayout {
    /// Logical layout organization.
    pub kind: TensorLayoutKind,
}

impl TensorLayout {
    /// Construct a logical layout descriptor.
    #[must_use]
    pub const fn new(kind: TensorLayoutKind) -> Self {
        Self { kind }
    }

    /// Construct a row-major dense layout.
    #[must_use]
    pub fn row_major() -> Self {
        Self::new(TensorLayoutKind::Dense(DenseLayout::row_major()))
    }

    /// Construct a column-major dense layout.
    #[must_use]
    pub fn column_major() -> Self {
        Self::new(TensorLayoutKind::Dense(DenseLayout::column_major()))
    }

    /// Construct a strided layout.
    #[must_use]
    pub fn strided(strides: Vec<StrideValue>) -> Self {
        Self::new(TensorLayoutKind::Strided(
            StridedLayout::new(strides),
        ))
    }

    /// Construct a sparse layout.
    #[must_use]
    pub fn sparse(layout: SparseLayout) -> Self {
        Self::new(TensorLayoutKind::Sparse(layout))
    }

    /// Construct a tiled layout.
    #[must_use]
    pub fn tiled(layout: TiledLayout) -> Self {
        Self::new(TensorLayoutKind::Tiled(layout))
    }

    /// Construct a distributed layout.
    #[must_use]
    pub fn distributed(layout: DistributedLayout) -> Self {
        Self::new(TensorLayoutKind::Distributed(layout))
    }

    /// Construct a lazy layout.
    #[must_use]
    pub fn lazy(layout: LazyLayout) -> Self {
        Self::new(TensorLayoutKind::Lazy(layout))
    }

    /// Construct an extension layout.
    #[must_use]
    pub fn extension(layout: ExtensionLayout) -> Self {
        Self::new(TensorLayoutKind::Extension(layout))
    }

    /// Return the logical layout kind.
    #[must_use]
    pub fn kind(&self) -> &TensorLayoutKind {
        &self.kind
    }

    /// Return the stable kind name.
    #[must_use]
    pub fn kind_name(&self) -> &'static str {
        self.kind.kind_name()
    }
}

// ============================================================================
// VALIDATION
// ============================================================================

/// Information needed to validate layout metadata.
///
/// This is deliberately not a tensor shape representation.
///
/// It contains only facts supplied by canonical shape analysis.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LayoutValidationContext {
    /// Canonical tensor rank, if known.
    pub rank: Option<usize>,
}

impl LayoutValidationContext {
    /// Construct a validation context with known rank.
    #[must_use]
    pub const fn known_rank(rank: usize) -> Self {
        Self { rank: Some(rank) }
    }

    /// Construct a context with unresolved rank.
    #[must_use]
    pub const fn unresolved_rank() -> Self {
        Self { rank: None }
    }
}

/// Result of layout metadata validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LayoutValidation {
    /// All statically checkable invariants are satisfied.
    Valid,

    /// The layout is structurally valid, but some checks require
    /// canonical semantic analysis or downstream realization.
    Unresolved(Vec<LayoutValidationIssue>),
}

/// A validation issue that cannot be resolved locally.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum LayoutValidationIssue {
    /// Tensor rank is not known.
    UnknownRank,

    /// A symbolic stride requires semantic resolution.
    SymbolicStride {
        dimension: usize,
    },

    /// An extension must validate its own payload.
    ExtensionValidationRequired {
        id: LayoutId,
    },

    /// A deferred layout needs downstream realization.
    DeferredRealization,
}

/// A locally detectable layout validation failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LayoutError {
    /// The layout identifier is empty.
    EmptyIdentifier,

    /// The layout identifier contains whitespace.
    InvalidIdentifier,

    /// The extension version is empty.
    EmptyExtensionVersion,

    /// A layout's metadata rank differs from the canonical tensor rank.
    RankMismatch {
        expected: usize,
        actual: usize,
    },

    /// A tile dimension must be non-zero.
    ZeroTileDimension {
        dimension: usize,
    },

    /// A block dimension must be non-zero.
    ZeroBlockDimension {
        dimension: usize,
    },

    /// A dimension permutation contains an out-of-range index.
    InvalidDimensionIndex {
        dimension: usize,
        rank: usize,
    },

    /// A dimension permutation repeats an index.
    DuplicateDimensionIndex {
        dimension: usize,
    },

    /// A partition shape contains a zero dimension.
    ZeroPartitionDimension {
        dimension: usize,
    },

    /// A dimension-based partition references a dimension outside rank.
    PartitionDimensionOutOfRange {
        dimension: usize,
        rank: usize,
    },

    /// A sparse block layout requires explicit block dimensions.
    MissingBlockDimensions,

    /// The number of block dimensions does not match rank.
    BlockRankMismatch {
        expected: usize,
        actual: usize,
    },
}

impl fmt::Display for LayoutError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => {
                formatter.write_str("layout identifier cannot be empty")
            }

            Self::InvalidIdentifier => {
                formatter.write_str(
                    "layout identifier cannot contain whitespace",
                )
            }

            Self::EmptyExtensionVersion => {
                formatter.write_str(
                    "extension layout version cannot be empty",
                )
            }

            Self::RankMismatch { expected, actual } => {
                write!(
                    formatter,
                    "layout rank mismatch: expected {expected}, got {actual}"
                )
            }

            Self::ZeroTileDimension { dimension } => {
                write!(
                    formatter,
                    "tile dimension {dimension} must be non-zero"
                )
            }

            Self::ZeroBlockDimension { dimension } => {
                write!(
                    formatter,
                    "block dimension {dimension} must be non-zero"
                )
            }

            Self::InvalidDimensionIndex { dimension, rank } => {
                write!(
                    formatter,
                    "dimension index {dimension} is outside rank {rank}"
                )
            }

            Self::DuplicateDimensionIndex { dimension } => {
                write!(
                    formatter,
                    "dimension index {dimension} occurs more than once"
                )
            }

            Self::ZeroPartitionDimension { dimension } => {
                write!(
                    formatter,
                    "partition dimension {dimension} must be non-zero"
                )
            }

            Self::PartitionDimensionOutOfRange { dimension, rank } => {
                write!(
                    formatter,
                    "partition dimension {dimension} is outside rank {rank}"
                )
            }

            Self::MissingBlockDimensions => {
                formatter.write_str(
                    "block sparse layout requires block dimensions",
                )
            }

            Self::BlockRankMismatch { expected, actual } => {
                write!(
                    formatter,
                    "block rank mismatch: expected {expected}, got {actual}"
                )
            }
        }
    }
}

impl std::error::Error for LayoutError {}

impl TensorLayout {
    /// Validate all locally checkable layout invariants.
    ///
    /// Symbolic and deferred information is represented explicitly.
    /// No symbolic value is invented.
    pub fn validate(
        &self,
        context: LayoutValidationContext,
    ) -> Result<LayoutValidation, LayoutError> {
        let mut unresolved = Vec::new();

        match &self.kind {
            TensorLayoutKind::Dense(_) => {}

            TensorLayoutKind::Strided(layout) => {
                if let Some(rank) = context.rank {
                    validate_rank(rank, layout.strides.len())?;
                } else {
                    unresolved.push(LayoutValidationIssue::UnknownRank);
                }

                for (dimension, stride) in
                    layout.strides.iter().enumerate()
                {
                    if matches!(stride, StrideValue::Symbolic(_)) {
                        unresolved.push(
                            LayoutValidationIssue::SymbolicStride {
                                dimension,
                            },
                        );
                    }
                }
            }

            TensorLayoutKind::Sparse(layout) => {
                if let Some(block_dimensions) =
                    &layout.block_dimensions
                {
                    validate_nonzero_dimensions(
                        block_dimensions,
                        LayoutError::ZeroBlockDimension,
                    )?;

                    if let Some(rank) = context.rank {
                        validate_rank(
                            rank,
                            block_dimensions.len(),
                        )?;
                    } else {
                        unresolved.push(
                            LayoutValidationIssue::UnknownRank,
                        );
                    }
                } else if layout.format
                    == SparseFormat::BlockSparse
                {
                    return Err(
                        LayoutError::MissingBlockDimensions,
                    );
                }
            }

            TensorLayoutKind::Tiled(layout) => {
                validate_nonzero_dimensions(
                    &layout.tile_dimensions,
                    LayoutError::ZeroTileDimension,
                )?;

                if let Some(rank) = context.rank {
                    validate_rank(
                        rank,
                        layout.tile_dimensions.len(),
                    )?;

                    if let Some(order) = &layout.dimension_order {
                        validate_permutation(order, rank)?;
                    }
                } else {
                    unresolved.push(
                        LayoutValidationIssue::UnknownRank,
                    );

                    if let Some(order) = &layout.dimension_order {
                        validate_permutation(order, order.len())?;
                    }
                }
            }

            TensorLayoutKind::Distributed(layout) => {
                match &layout.strategy {
                    PartitionStrategy::AlongDimension {
                        dimension,
                    } => {
                        if let Some(rank) = context.rank {
                            if *dimension >= rank {
                                return Err(
                                    LayoutError::PartitionDimensionOutOfRange {
                                        dimension: *dimension,
                                        rank,
                                    },
                                );
                            }
                        } else {
                            unresolved.push(
                                LayoutValidationIssue::UnknownRank,
                            );
                        }
                    }

                    PartitionStrategy::Multidimensional {
                        partition_dimensions,
                    } => {
                        validate_nonzero_dimensions(
                            partition_dimensions,
                            LayoutError::ZeroPartitionDimension,
                        )?;

                        if let Some(rank) = context.rank {
                            validate_rank(
                                rank,
                                partition_dimensions.len(),
                            )?;
                        } else {
                            unresolved.push(
                                LayoutValidationIssue::UnknownRank,
                            );
                        }
                    }

                    PartitionStrategy::Deferred => {
                        unresolved.push(
                            LayoutValidationIssue::DeferredRealization,
                        );
                    }

                    PartitionStrategy::Extension(_) => {
                        unresolved.push(
                            LayoutValidationIssue::DeferredRealization,
                        );
                    }
                }
            }

            TensorLayoutKind::Lazy(layout) => {
                if layout.preferred.is_none() {
                    unresolved.push(
                        LayoutValidationIssue::DeferredRealization,
                    );
                }

                if let Some(preferred) = &layout.preferred {
                    let nested = TensorLayout::new(
                        preferred.as_ref().clone(),
                    );

                    match nested.validate(context)? {
                        LayoutValidation::Valid => {}

                        LayoutValidation::Unresolved(issues) => {
                            unresolved.extend(issues);
                        }
                    }
                }
            }

            TensorLayoutKind::Extension(layout) => {
                unresolved.push(
                    LayoutValidationIssue::ExtensionValidationRequired {
                        id: layout.id.clone(),
                    },
                );
            }
        }

        if unresolved.is_empty() {
            Ok(LayoutValidation::Valid)
        } else {
            Ok(LayoutValidation::Unresolved(unresolved))
        }
    }
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn validate_rank(
    expected: usize,
    actual: usize,
) -> Result<(), LayoutError> {
    if expected != actual {
        return Err(LayoutError::RankMismatch {
            expected,
            actual,
        });
    }

    Ok(())
}

fn validate_nonzero_dimensions(
    dimensions: &[u128],
    error: fn(usize) -> LayoutError,
) -> Result<(), LayoutError> {
    for (dimension, value) in dimensions.iter().enumerate() {
        if *value == 0 {
            return Err(error(dimension));
        }
    }

    Ok(())
}

fn validate_permutation(
    order: &[usize],
    rank: usize,
) -> Result<(), LayoutError> {
    if order.len() != rank {
        return Err(LayoutError::RankMismatch {
            expected: rank,
            actual: order.len(),
        });
    }

    let mut seen = BTreeMap::new();

    for dimension in order {
        if *dimension >= rank {
            return Err(LayoutError::InvalidDimensionIndex {
                dimension: *dimension,
                rank,
            });
        }

        if seen.insert(*dimension, ()).is_some() {
            return Err(LayoutError::DuplicateDimensionIndex {
                dimension: *dimension,
            });
        }
    }

    Ok(())
}

// ============================================================================
// LAYOUT TRANSFORMATION DESCRIPTIONS
// ============================================================================

/// A logical layout transformation.
///
/// This is a declarative description. It does not execute the transformation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LayoutTransformation {
    /// Reinterpret the logical dimension ordering.
    PermuteDimensions {
        order: Vec<usize>,
    },

    /// Change the logical tile structure.
    Retile {
        tile_dimensions: Vec<u128>,
    },

    /// Convert to another logical layout.
    Convert {
        target: TensorLayout,
    },

    /// Delegate layout selection to a downstream realization stage.
    ResolveLazy,
}

// ============================================================================
// LAYOUT COMPATIBILITY
// ============================================================================

/// Compatibility result between two logical layouts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LayoutCompatibility {
    /// The layouts are identical.
    Identical,

    /// The layouts are different but may be converted.
    Convertible,

    /// Compatibility requires semantic or downstream analysis.
    Unresolved,

    /// The layouts cannot be reconciled under the supplied contract.
    Incompatible,
}

/// Compare two logical layout descriptions.
///
/// This function does not claim that a particular hardware target can
/// realize either layout.
#[must_use]
pub fn compare_layouts(
    left: &TensorLayout,
    right: &TensorLayout,
) -> LayoutCompatibility {
    if left == right {
        return LayoutCompatibility::Identical;
    }

    if left.kind.requires_extension()
        || right.kind.requires_extension()
    {
        return LayoutCompatibility::Unresolved;
    }

    match (&left.kind, &right.kind) {
        (TensorLayoutKind::Lazy(_), _)
        | (_, TensorLayoutKind::Lazy(_)) => {
            LayoutCompatibility::Unresolved
        }

        _ => LayoutCompatibility::Convertible,
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dense_layouts_are_constructible() {
        let row_major = TensorLayout::row_major();
        let column_major = TensorLayout::column_major();

        assert_eq!(row_major.kind_name(), "dense");
        assert_eq!(column_major.kind_name(), "dense");

        assert_eq!(
            row_major.validate(
                LayoutValidationContext::known_rank(3),
            ),
            Ok(LayoutValidation::Valid),
        );
    }

    #[test]
    fn strided_layout_checks_rank() {
        let layout = TensorLayout::strided(vec![
            StrideValue::Known(8),
            StrideValue::Known(1),
        ]);

        assert_eq!(
            layout.validate(
                LayoutValidationContext::known_rank(2),
            ),
            Ok(LayoutValidation::Valid),
        );

        assert_eq!(
            layout.validate(
                LayoutValidationContext::known_rank(3),
            ),
            Err(LayoutError::RankMismatch {
                expected: 3,
                actual: 2,
            }),
        );
    }

    #[test]
    fn symbolic_strides_remain_unresolved() {
        let layout = TensorLayout::strided(vec![
            StrideValue::Symbolic("stride_n".to_owned()),
        ]);

        assert_eq!(
            layout.validate(
                LayoutValidationContext::known_rank(1),
            ),
            Ok(LayoutValidation::Unresolved(vec![
                LayoutValidationIssue::SymbolicStride {
                    dimension: 0,
                },
            ])),
        );
    }

    #[test]
    fn tiled_layout_rejects_zero_tile_dimensions() {
        let layout = TensorLayout::tiled(
            TiledLayout::new(vec![4, 0], None),
        );

        assert_eq!(
            layout.validate(
                LayoutValidationContext::known_rank(2),
            ),
            Err(LayoutError::ZeroTileDimension {
                dimension: 1,
            }),
        );
    }

    #[test]
    fn tiled_layout_rejects_duplicate_permutation_indices() {
        let layout = TensorLayout::tiled(
            TiledLayout::new(
                vec![4, 8],
                Some(vec![0, 0]),
            ),
        );

        assert_eq!(
            layout.validate(
                LayoutValidationContext::known_rank(2),
            ),
            Err(LayoutError::DuplicateDimensionIndex {
                dimension: 0,
            }),
        );
    }

    #[test]
    fn distributed_layout_does_not_select_devices() {
        let layout = TensorLayout::distributed(
            DistributedLayout::new(
                PartitionStrategy::AlongDimension {
                    dimension: 1,
                },
                true,
            ),
        );

        assert_eq!(
            layout.validate(
                LayoutValidationContext::known_rank(2),
            ),
            Ok(LayoutValidation::Valid),
        );
    }

    #[test]
    fn lazy_layout_is_explicitly_unresolved() {
        let layout = TensorLayout::lazy(
            LazyLayout::new(None, true),
        );

        assert_eq!(
            layout.validate(
                LayoutValidationContext::known_rank(2),
            ),
            Ok(LayoutValidation::Unresolved(vec![
                LayoutValidationIssue::DeferredRealization,
            ])),
        );
    }

    #[test]
    fn extension_layout_is_not_silently_interpreted() {
        let id = LayoutId::new(
            "org.example.tensor.layout.custom",
        )
        .expect("valid layout identifier");

        let extension = ExtensionLayout::new(
            id.clone(),
            "1",
            BTreeMap::new(),
        )
        .expect("valid extension");

        let layout = TensorLayout::extension(extension);

        assert_eq!(
            layout.validate(
                LayoutValidationContext::known_rank(2),
            ),
            Ok(LayoutValidation::Unresolved(vec![
                LayoutValidationIssue::ExtensionValidationRequired {
                    id,
                },
            ])),
        );
    }

    #[test]
    fn equal_layouts_are_identical() {
        let left = TensorLayout::row_major();
        let right = TensorLayout::row_major();

        assert_eq!(
            compare_layouts(&left, &right),
            LayoutCompatibility::Identical,
        );
    }

    #[test]
    fn distinct_builtin_layouts_are_convertible() {
        let left = TensorLayout::row_major();
        let right = TensorLayout::column_major();

        assert_eq!(
            compare_layouts(&left, &right),
            LayoutCompatibility::Convertible,
        );
    }

    #[test]
    fn layout_identifier_rejects_whitespace() {
        assert_eq!(
            LayoutId::new("invalid layout"),
            Err(LayoutError::InvalidIdentifier),
        );
    }
}
