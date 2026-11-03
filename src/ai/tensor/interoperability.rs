
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Tensor Interoperability
//!
//! Path: `src/ai/tensor/interoperability.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide a target-independent interoperability boundary for AI tensor
//! descriptions and concrete tensor representations.
//!
//! The module preserves:
//!
//! - Canonical symbolic tensor shapes.
//! - AI numerical data-type semantics.
//! - Logical tensor layout.
//! - Source and semantic provenance.
//! - Explicit representation identity.
//! - Conversion fidelity and compatibility requirements.
//!
//! It supports conversion between representations through explicit,
//! typed adapters. It does not assume that every representation has
//! identical semantics or storage behavior.
//!
//! ## Canonical ownership
//!
//! Semantic dimensions and shapes:
//!     frontend::semantic::domains::ai
//!
//! AI numerical data types:
//!     ai::tensor::dtypes
//!
//! Logical tensor layouts:
//!     ai::tensor::layouts
//!
//! Concrete classical tensor storage:
//!     classical::collections::tensor
//!
//! Universal tensor operations:
//!     zuir::dialects::tensor
//!
//! AI operations:
//!     zuir::dialects::ai
//!
//! Quantum computation:
//!     quantum::ir
//!
//! Cross-domain composition:
//!     hybrid
//!
//! Distributed placement and communication:
//!     distributed
//!
//! ## Does not own
//!
//! This module does not define:
//!
//! - Another tensor shape or dimension language.
//! - Another tensor element storage implementation.
//! - Another universal intermediate representation.
//! - Numerical tensor kernels.
//! - Quantum state or circuit representations.
//! - Physical memory layouts.
//! - Hardware discovery or device selection.
//! - Resource allocation.
//! - Scheduling or distributed placement.
//! - Runtime execution.
//! - External format implementations.
//!
//! ## POCO-REAF
//!
//! No universal maximum is imposed on:
//!
//! - Tensor rank.
//! - Dimension count.
//! - Logical tensor size.
//! - Tensor representation count.
//! - Conversion adapter count.
//! - Supported computational domains.
//!
//! Conversion to concrete representations is subject to their actual
//! representational capabilities. Such limitations are explicit errors,
//! never silent changes to program meaning.
//!
//! ## Important scalar distinction
//!
//! The canonical AI semantic TensorShape supports rank-zero scalars.
//! The current ClassicalArrayShape implementation rejects rank-zero shapes.
//!
//! Consequently, this module does not silently convert a scalar into a
//! one-element vector. Such a conversion would change its shape semantics.
//!
//! ## Integration
//!
//! The frontend supplies validated semantic tensor descriptions.
//! The tensor subsystem supplies dtype and layout descriptions.
//! Classical storage remains owned by classical::collections::tensor.
//! ZUIR, quantum, HDL, hybrid, distributed and external representations
//! provide their own adapters when their canonical APIs are available.
//!
//! ## Freeze criteria
//!
//! - Canonical semantic shapes are reused.
//! - Dtype and layout metadata are preserved.
//! - Scalar incompatibility is explicit.
//! - Symbolic dimensions are never guessed.
//! - Integer conversion is checked.
//! - Unsupported conversions fail explicitly.
//! - No physical placement is introduced.
//! - Adapter behavior is deterministic.
//! - Tests cover concrete, symbolic, scalar and invalid conversions.

use core::fmt;

use crate::ai::tensor::dtypes::DType;
use crate::ai::tensor::layouts::TensorLayout;
use crate::classical::collections::array::{
    ClassicalArrayShape,
    ClassicalArrayStorageKind,
};
use crate::classical::collections::tensor::{
    ClassicalTensor,
    ClassicalTensorError,
};
use crate::frontend::semantic::domains::ai::{
    AiOrigin,
    DimensionExpr,
    TensorShape,
};

// ============================================================================
// REPRESENTATION IDENTITY
// ============================================================================

/// Stable logical identity for a tensor representation.
///
/// This identity describes a representation contract. It is not a device
/// identifier, physical address, vendor identifier or runtime handle.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TensorRepresentationId(String);

impl TensorRepresentationId {
    /// Creates a representation identity.
    ///
    /// Empty values and control characters are rejected. Names remain
    /// otherwise open-ended so that extensions can define new formats.
    pub fn new(value: impl Into<String>) -> Result<Self, InteroperabilityError> {
        let value = value.into();

        if value.is_empty() || value.chars().any(char::is_control) {
            return Err(InteroperabilityError::InvalidRepresentationId);
        }

        Ok(Self(value))
    }

    /// Returns the identity as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TensorRepresentationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// CANONICAL TENSOR DESCRIPTION
// ============================================================================

/// A target-independent description of an AI tensor.
///
/// This is metadata, not tensor storage or a competing tensor IR.
///
/// The shape is the canonical semantic shape. The dtype and layout are
/// the AI tensor subsystem's existing descriptors. Provenance retains
/// the canonical AI semantic origin record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorDescriptor {
    /// Canonical symbolic tensor shape.
    pub shape: TensorShape,

    /// Mathematical and representation semantics of tensor elements.
    pub dtype: DType,

    /// Logical tensor organization.
    pub layout: TensorLayout,

    /// Origin supplied by the canonical semantic adapter.
    pub origin: AiOrigin,
}

impl TensorDescriptor {
    /// Constructs a tensor descriptor without allocating tensor elements.
    #[must_use]
    pub fn new(
        shape: TensorShape,
        dtype: DType,
        layout: TensorLayout,
        origin: AiOrigin,
    ) -> Self {
        Self {
            shape,
            dtype,
            layout,
            origin,
        }
    }

    /// Returns the logical rank.
    ///
    /// Rank zero is a valid semantic scalar.
    #[must_use]
    pub fn rank(&self) -> usize {
        self.shape.rank()
    }

    /// Returns whether this descriptor represents a scalar.
    #[must_use]
    pub fn is_scalar(&self) -> bool {
        self.shape.is_scalar()
    }

    /// Returns whether every dimension is represented by a constant
    /// expression.
    ///
    /// This method does not claim that the dimensions have passed
    /// semantic validation or constraint solving.
    #[must_use]
    pub fn has_only_constant_dimensions(&self) -> bool {
        self.shape
            .dimensions
            .iter()
            .all(|dimension| matches!(dimension, DimensionExpr::Constant(_)))
    }
}

// ============================================================================
// CONVERSION FIDELITY
// ============================================================================

/// Describes whether an interoperability operation may discard information.
///
/// The default policy is lossless conversion. An adapter must not silently
/// weaken shape, dtype, layout, precision or provenance semantics.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConversionFidelity {
    /// All semantically relevant properties must be preserved.
    Lossless,

    /// The caller explicitly permits documented metadata loss.
    ///
    /// This does not authorize changing mathematical tensor semantics.
    AllowDeclaredMetadataLoss,
}

/// Identifies a property that a conversion cannot preserve.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum LostTensorProperty {
    Shape,
    DType,
    Precision,
    Layout,
    Provenance,
    Representation,
}

/// Describes the fidelity of a completed conversion.
///
/// An adapter must report only losses that actually occurred.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConversionReport {
    /// Properties that were not preserved exactly.
    pub lost_properties: Vec<LostTensorProperty>,

    /// Explanations supplied by the adapter.
    pub explanations: Vec<String>,
}

impl ConversionReport {
    /// Returns true when no semantic or metadata loss was reported.
    #[must_use]
    pub fn is_lossless(&self) -> bool {
        self.lost_properties.is_empty()
    }
}

// ============================================================================
// INTEROPERABILITY ERRORS
// ============================================================================

/// Errors produced by tensor interoperability operations.
#[derive(Debug)]
pub enum InteroperabilityError {
    /// A representation identity is malformed.
    InvalidRepresentationId,

    /// The requested representation is not supported by the selected
    /// adapter.
    UnsupportedRepresentation {
        representation: TensorRepresentationId,
    },

    /// The requested conversion would discard information under a
    /// lossless conversion policy.
    InformationLoss {
        properties: Vec<LostTensorProperty>,
    },

    /// A symbolic dimension has not been resolved.
    UnresolvedDimension {
        axis: usize,
    },

    /// A dimension cannot be represented by the destination's integer type.
    DimensionNotRepresentable {
        axis: usize,
        value: u128,
    },

    /// The destination representation cannot represent scalar tensors.
    ScalarNotRepresentable,

    /// The supplied shape and concrete tensor shape do not agree.
    ShapeMismatch {
        expected_rank: usize,
        actual_rank: usize,
    },

    /// The supplied concrete dimension differs from the semantic dimension.
    DimensionMismatch {
        axis: usize,
        expected: u128,
        actual: usize,
    },

    /// The supplied dimension resolver established that a dimension
    /// is invalid.
    InvalidDimension {
        axis: usize,
    },

    /// An adapter or metadata collection could not reserve memory.
    AllocationFailure,

    /// A destination rejected a conversion.
    AdapterFailure {
        message: String,
    },

    /// The canonical classical tensor implementation rejected an operation.
    ClassicalTensor(ClassicalTensorError),
}

impl fmt::Display for InteroperabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRepresentationId => {
                formatter.write_str("invalid tensor representation identity")
            }

            Self::UnsupportedRepresentation { representation } => {
                write!(
                    formatter,
                    "unsupported tensor representation: {representation}"
                )
            }

            Self::InformationLoss { properties } => {
                write!(
                    formatter,
                    "tensor conversion would lose information: {properties:?}"
                )
            }

            Self::UnresolvedDimension { axis } => {
                write!(
                    formatter,
                    "tensor dimension at axis {axis} is unresolved"
                )
            }

            Self::DimensionNotRepresentable { axis, value } => {
                write!(
                    formatter,
                    "dimension {value} at axis {axis} is not representable \
                     by the destination"
                )
            }

            Self::ScalarNotRepresentable => {
                formatter.write_str(
                    "the destination tensor representation does not support \
                     rank-zero scalar shapes"
                )
            }

            Self::ShapeMismatch {
                expected_rank,
                actual_rank,
            } => {
                write!(
                    formatter,
                    "tensor rank mismatch: expected {expected_rank}, \
                     received {actual_rank}"
                )
            }

            Self::DimensionMismatch {
                axis,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "tensor dimension mismatch at axis {axis}: \
                     expected {expected}, received {actual}"
                )
            }

            Self::InvalidDimension { axis } => {
                write!(
                    formatter,
                    "tensor dimension at axis {axis} is invalid"
                )
            }

            Self::AllocationFailure => {
                formatter.write_str(
                    "unable to reserve memory for tensor interoperability metadata"
                )
            }

            Self::AdapterFailure { message } => {
                write!(formatter, "tensor interoperability adapter failed: {message}")
            }

            Self::ClassicalTensor(error) => {
                write!(formatter, "classical tensor conversion failed: {error}")
            }
        }
    }
}

impl std::error::Error for InteroperabilityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ClassicalTensor(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ClassicalTensorError> for InteroperabilityError {
    fn from(error: ClassicalTensorError) -> Self {
        Self::ClassicalTensor(error)
    }
}

// ============================================================================
// DIMENSION RESOLUTION
// ============================================================================

/// Knowledge about a canonical semantic dimension.
///
/// `Unknown` is not treated as zero, one or any other guessed value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InteropDimensionKnowledge {
    /// The dimension has been resolved to this mathematical value.
    Known(u128),

    /// The dimension remains symbolic or otherwise unresolved.
    Unknown,

    /// Semantic analysis established that the dimension is invalid.
    Invalid,
}

/// Supplies dimension facts from the canonical semantic constraint system.
///
/// Implementations should delegate to the frontend's actual dimension
/// and constraint-solving services. This module does not implement a
/// competing symbolic evaluator.
pub trait InteropDimensionResolver {
    /// Resolves the supplied canonical dimension expression.
    fn resolve(
        &self,
        expression: &DimensionExpr,
    ) -> InteropDimensionKnowledge;
}

// ============================================================================
// CLASSICAL SHAPE INTEROPERABILITY
// ============================================================================

/// Converts a canonical semantic shape into the concrete classical
/// collection shape.
///
/// This operation converts shape metadata only. It does not allocate
/// tensor elements.
///
/// The destination currently rejects scalar shapes and requires every
/// dimension to be resolved and representable by `usize`.
pub fn to_classical_shape<R>(
    shape: &TensorShape,
    resolver: &R,
) -> Result<ClassicalArrayShape, InteroperabilityError>
where
    R: InteropDimensionResolver,
{
    if shape.is_scalar() {
        return Err(InteroperabilityError::ScalarNotRepresentable);
    }

    let mut dimensions = Vec::new();

    dimensions
        .try_reserve_exact(shape.dimensions.len())
        .map_err(|_| InteroperabilityError::AllocationFailure)?;

    for (axis, expression) in shape.dimensions.iter().enumerate() {
        let value = match resolver.resolve(expression) {
            InteropDimensionKnowledge::Known(value) => value,

            InteropDimensionKnowledge::Unknown => {
                return Err(
                    InteroperabilityError::UnresolvedDimension { axis },
                );
            }

            InteropDimensionKnowledge::Invalid => {
                return Err(
                    InteroperabilityError::InvalidDimension { axis },
                );
            }
        };

        let dimension = usize::try_from(value).map_err(|_| {
            InteroperabilityError::DimensionNotRepresentable {
                axis,
                value,
            }
        })?;

        dimensions.push(dimension);
    }

    ClassicalArrayShape::new(dimensions)
        .map_err(|error| {
            InteroperabilityError::AdapterFailure {
                message: error.to_string(),
            }
        })
}

/// Converts a concrete classical shape into the canonical semantic shape.
///
/// A classical shape always has at least one dimension. Its dimensions
/// are represented as canonical constant expressions.
pub fn from_classical_shape(
    shape: &ClassicalArrayShape,
) -> Result<TensorShape, InteroperabilityError> {
    let mut dimensions = Vec::new();

    dimensions
        .try_reserve_exact(shape.rank())
        .map_err(|_| InteroperabilityError::AllocationFailure)?;

    for dimension in shape.dimensions() {
        dimensions.push(DimensionExpr::Constant(*dimension as u128));
    }

    Ok(TensorShape::new(dimensions))
}

// ============================================================================
// CLASSICAL TENSOR INTEROPERABILITY
// ============================================================================

/// Validates that a classical tensor's concrete shape agrees with a
/// canonical semantic tensor descriptor.
///
/// This function does not copy or materialize tensor elements.
pub fn validate_classical_tensor<T, R>(
    tensor: &ClassicalTensor<T>,
    descriptor: &TensorDescriptor,
    resolver: &R,
) -> Result<(), InteroperabilityError>
where
    R: InteropDimensionResolver,
{
    if descriptor.is_scalar() {
        return Err(InteroperabilityError::ScalarNotRepresentable);
    }

    if tensor.rank() != descriptor.rank() {
        return Err(InteroperabilityError::ShapeMismatch {
            expected_rank: descriptor.rank(),
            actual_rank: tensor.rank(),
        });
    }

    for (axis, expression) in descriptor.shape.dimensions.iter().enumerate() {
        let expected = match resolver.resolve(expression) {
            InteropDimensionKnowledge::Known(value) => value,

            InteropDimensionKnowledge::Unknown => {
                return Err(
                    InteroperabilityError::UnresolvedDimension { axis },
                );
            }

            InteropDimensionKnowledge::Invalid => {
                return Err(
                    InteroperabilityError::InvalidDimension { axis },
                );
            }
        };

        let actual = tensor.dimensions()[axis];

        if expected != actual as u128 {
            return Err(InteroperabilityError::DimensionMismatch {
                axis,
                expected,
                actual,
            });
        }
    }

    Ok(())
}

/// A borrowed classical tensor together with its AI semantic descriptor.
///
/// The tensor remains owned by the canonical classical collection system.
/// The descriptor supplies information that classical element storage
/// does not own, such as AI dtype, logical layout and semantic provenance.
#[derive(Debug)]
pub struct ClassicalTensorView<'a, T> {
    tensor: &'a ClassicalTensor<T>,
    descriptor: &'a TensorDescriptor,
}

impl<'a, T> ClassicalTensorView<'a, T> {
    /// Creates a checked view over an existing classical tensor.
    pub fn new<R>(
        tensor: &'a ClassicalTensor<T>,
        descriptor: &'a TensorDescriptor,
        resolver: &R,
    ) -> Result<Self, InteroperabilityError>
    where
        R: InteropDimensionResolver,
    {
        validate_classical_tensor(tensor, descriptor, resolver)?;

        Ok(Self {
            tensor,
            descriptor,
        })
    }

    /// Returns the canonical classical tensor.
    #[must_use]
    pub fn tensor(&self) -> &'a ClassicalTensor<T> {
        self.tensor
    }

    /// Returns the associated AI tensor descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &'a TensorDescriptor {
        self.descriptor
    }

    /// Returns the concrete storage strategy.
    #[must_use]
    pub fn storage_kind(&self) -> ClassicalArrayStorageKind {
        self.tensor.storage_kind()
    }
}

// ============================================================================
// EXTENSIBLE REPRESENTATION ADAPTER CONTRACT
// ============================================================================

/// Describes an explicit conversion between tensor representations.
///
/// Each representation owner implements this trait for its own canonical
/// representation. The adapter is responsible for validating its destination
/// contract and reporting any information loss.
///
/// Examples of future adapter owners include:
///
/// - The canonical ZUIR tensor dialect.
/// - The classical tensor collection.
/// - An external tensor artifact format.
/// - A quantum-specific tensor-network representation.
/// - A hybrid domain representation.
/// - A distributed logical tensor representation.
/// - An HDL-backed tensor interface.
///
/// These are integration points, not claims that those adapters already
/// exist in the repository.
pub trait TensorInteropAdapter<Target> {
    /// Returns the stable identity of the destination representation.
    fn representation_id(&self) -> &TensorRepresentationId;

    /// Converts a descriptor and source value into the destination type.
    ///
    /// The returned report must describe any metadata loss.
    fn convert(
        &self,
        descriptor: &TensorDescriptor,
        source: Target,
        fidelity: ConversionFidelity,
    ) -> Result<(Target, ConversionReport), InteroperabilityError>;
}

/// A typed request describing the intended conversion.
///
/// This does not select a physical device or perform scheduling.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorConversionRequest {
    /// Requested destination representation.
    pub destination: TensorRepresentationId,

    /// Required conversion fidelity.
    pub fidelity: ConversionFidelity,
}

impl TensorConversionRequest {
    /// Creates a conversion request.
    #[must_use]
    pub fn new(
        destination: TensorRepresentationId,
        fidelity: ConversionFidelity,
    ) -> Self {
        Self {
            destination,
            fidelity,
        }
    }
}

/// Validates a conversion report against the caller's fidelity policy.
///
/// A lossless request fails if the adapter reports any lost property.
pub fn validate_conversion_report(
    report: &ConversionReport,
    fidelity: ConversionFidelity,
) -> Result<(), InteroperabilityError> {
    if fidelity == ConversionFidelity::Lossless
        && !report.is_lossless()
    {
        return Err(InteroperabilityError::InformationLoss {
            properties: report.lost_properties.clone(),
        });
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct TestResolver;

    impl InteropDimensionResolver for TestResolver {
        fn resolve(
            &self,
            expression: &DimensionExpr,
        ) -> InteropDimensionKnowledge {
            match expression {
                DimensionExpr::Constant(value) => {
                    InteropDimensionKnowledge::Known(*value)
                }

                _ => InteropDimensionKnowledge::Unknown,
            }
        }
    }

    fn descriptor(shape: TensorShape) -> TensorDescriptor {
        TensorDescriptor::new(
            shape,
            DType::Bool,
            TensorLayout::row_major(),
            AiOrigin::default(),
        )
    }

    #[test]
    fn concrete_shape_converts_to_classical_shape() {
        let shape = TensorShape::new(vec![
            DimensionExpr::Constant(3),
            DimensionExpr::Constant(5),
        ]);

        let converted = to_classical_shape(&shape, &TestResolver)
            .expect("concrete dimensions should convert");

        assert_eq!(converted.dimensions(), &[3, 5]);
    }

    #[test]
    fn classical_shape_converts_to_canonical_shape() {
        let shape = ClassicalArrayShape::new(vec![3, 5])
            .expect("non-scalar classical shape");

        let converted = from_classical_shape(&shape)
            .expect("classical dimensions should convert");

        assert_eq!(
            converted.dimensions,
            vec![
                DimensionExpr::Constant(3),
                DimensionExpr::Constant(5),
            ]
        );
    }

    #[test]
    fn scalar_is_not_silently_converted_to_vector() {
        let shape = TensorShape::new(Vec::new());

        let result = to_classical_shape(&shape, &TestResolver);

        assert!(matches!(
            result,
            Err(InteroperabilityError::ScalarNotRepresentable)
        ));
    }

    #[test]
    fn unresolved_dimensions_are_rejected() {
        let shape = TensorShape::new(vec![
            DimensionExpr::Symbol(
                crate::frontend::semantic::domains::ai::AiId::new("batch")
                    .expect("valid identity"),
            ),
        ]);

        let result = to_classical_shape(&shape, &TestResolver);

        assert!(matches!(
            result,
            Err(InteroperabilityError::UnresolvedDimension {
                axis: 0
            })
        ));
    }

    #[test]
    fn descriptor_preserves_scalar_semantics() {
        let value = descriptor(TensorShape::new(Vec::new()));

        assert!(value.is_scalar());
        assert_eq!(value.rank(), 0);
    }

    #[test]
    fn lossless_conversion_rejects_metadata_loss() {
        let report = ConversionReport {
            lost_properties: vec![LostTensorProperty::Layout],
            explanations: vec![
                "destination does not preserve logical layout".to_owned(),
            ],
        };

        assert!(matches!(
            validate_conversion_report(
                &report,
                ConversionFidelity::Lossless
            ),
            Err(InteroperabilityError::InformationLoss { .. })
        ));
    }

    #[test]
    fn explicitly_permitted_metadata_loss_is_reportable() {
        let report = ConversionReport {
            lost_properties: vec![LostTensorProperty::Provenance],
            explanations: vec!["destination has no provenance field".to_owned()],
        };

        assert!(
            validate_conversion_report(
                &report,
                ConversionFidelity::AllowDeclaredMetadataLoss
            )
            .is_ok()
        );
    }

    #[test]
    fn representation_identity_rejects_empty_values() {
        assert!(
            TensorRepresentationId::new("").is_err()
        );
    }

    #[test]
    fn representation_identity_preserves_extension_names() {
        let identity = TensorRepresentationId::new(
            "org.example.tensor.custom",
        )
        .expect("valid extension identity");

        assert_eq!(
            identity.as_str(),
            "org.example.tensor.custom"
        );
    }
}
