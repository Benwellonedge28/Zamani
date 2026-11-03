
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani AI Quantization
//!
//! Path: src/ai/optimization/quantization.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Defines target-independent quantization semantics, quantization
//! parameters, calibration contracts, validation evidence and precision
//! transformation interoperability.
//!
//! OWNERSHIP
//!
//! This module owns:
//! - Quantization scheme descriptions.
//! - Quantization granularity.
//! - Scale and zero-point metadata.
//! - Calibration specifications.
//! - Calibration evidence references.
//! - Quantization parameter descriptions.
//! - Quantization transformation contracts.
//! - Quantization metadata validation.
//! - Quantization/dequantization semantic descriptions.
//!
//! DOES NOT OWN
//!
//! - Canonical tensor storage.
//! - Canonical tensor shape semantics.
//! - Canonical numerical data types.
//! - Numerical precision legality.
//! - Calibration execution.
//! - Quantization kernels.
//! - Model execution.
//! - Hardware discovery.
//! - Hardware-specific numerical formats.
//! - Device allocation.
//! - Scheduling.
//! - Placement.
//! - ZUIR construction.
//! - Runtime execution.
//!
//! AUTHORITIES
//!
//! Tensor numerical types:
//!     crate::ai::tensor::dtypes
//!
//! Precision transformation legality:
//!     crate::ai::optimization::precision
//!
//! AI requirements:
//!     crate::ai::requirements
//!
//! AI errors:
//!     crate::ai::error
//!
//! Tensor storage:
//!     crate::classical::collections::tensor
//!
//! Universal tensor representation:
//!     crate::zuir::dialects::tensor
//!
//! Source optimization intent:
//!     grammar/ai/optimization.g4
//!
//! Tensor source semantics:
//!     grammar/ai/tensors.g4
//!
//! SCALABILITY
//!
//! This module imposes no universal limits on:
//! - tensor rank;
//! - tensor dimensions;
//! - model size;
//! - parameter count;
//! - quantization groups;
//! - calibration samples;
//! - calibration datasets;
//! - number of quantization transformations;
//! - numerical representation width;
//! - execution targets.
//!
//! Representation widths use the canonical DType descriptors.
//!
//! Resource feasibility belongs downstream.
//!
//! POCO-REAF
//!
//! Quantization semantics are independent of physical hardware.
//!
//! A quantization transformation must explicitly describe:
//! - source numerical semantics;
//! - destination numerical semantics;
//! - quantization scheme;
//! - granularity;
//! - rounding;
//! - calibration provenance;
//! - approximation requirements.
//!
//! Unknown calibration or validation evidence must never be
//! interpreted as proof of correctness.
//!
//! SAFETY
//!
//! Safe Rust only.

use std::collections::BTreeSet;
use std::fmt;

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiResult,
};

use crate::ai::requirements::{
    NumericalFormat,
    RequirementKey,
};

use crate::ai::tensor::dtypes::{
    DType,
    IntegerSignedness,
    QuantizationScale,
    QuantizationZeroPoint,
};

use crate::ai::optimization::precision::{
    OverflowPolicy,
    PrecisionFormat,
    PrecisionTransformation,
    PrecisionTransformationKind,
    RoundingPolicy,
    SpecialValuePolicy,
    UnderflowPolicy,
};


// ============================================================================
// IDENTITIES
// ============================================================================

/// Stable identity of a quantization scheme.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct QuantizationId(RequirementKey);

impl QuantizationId {
    /// Creates a quantization identity.
    #[must_use]
    pub fn new(key: RequirementKey) -> Self {
        Self(key)
    }

    /// Returns the canonical identity.
    #[must_use]
    pub fn key(&self) -> &RequirementKey {
        &self.0
    }
}

impl fmt::Display for QuantizationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


// ============================================================================
// SCHEME
// ============================================================================

/// Mathematical quantization scheme.
///
/// This describes quantization semantics, not an implementation kernel.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum QuantizationScheme {
    /// Affine quantization.
    ///
    /// The mathematical mapping is:
    ///
    /// q = round(x / scale) + zero_point
    ///
    /// The inverse mapping is:
    ///
    /// x = scale * (q - zero_point)
    Affine,

    /// Symmetric quantization.
    ///
    /// The representation is centered around zero.
    Symmetric,

    /// Asymmetric quantization.
    ///
    /// The representation permits a nonzero zero point.
    Asymmetric,

    /// Logarithmic quantization.
    Logarithmic {
        /// Stable logarithmic base identity.
        base: RequirementKey,
    },

    /// Custom quantization scheme.
    ///
    /// Its mathematical semantics must be registered and validated
    /// by the corresponding extension.
    Custom {
        /// Stable extension identity.
        extension: RequirementKey,
    },
}

impl QuantizationScheme {
    /// Returns whether this scheme is defined by a custom provider.
    #[must_use]
    pub const fn is_custom(&self) -> bool {
        matches!(self, Self::Custom { .. })
    }

    /// Returns whether the scheme requires external semantic validation.
    #[must_use]
    pub const fn requires_extension_validation(&self) -> bool {
        matches!(
            self,
            Self::Custom { .. } | Self::Logarithmic { .. }
        )
    }
}


// ============================================================================
// GRANULARITY
// ============================================================================

/// Scope at which quantization parameters apply.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum QuantizationGranularity {
    /// One parameter set applies to the complete tensor.
    PerTensor,

    /// Parameters are associated with an identified tensor axis.
    PerAxis {
        /// Semantic axis identity.
        axis: String,
    },

    /// Parameters apply to explicitly identified groups.
    ///
    /// Group boundaries must be established by the canonical
    /// tensor-shape and partitioning authorities.
    PerGroup {
        /// Stable group definition identity.
        group_definition: RequirementKey,
    },

    /// Parameters apply independently to individual elements.
    PerElement,

    /// Extension-defined granularity.
    Custom {
        /// Extension identity.
        extension: RequirementKey,
    },
}

impl QuantizationGranularity {
    /// Validates locally checkable invariants.
    pub fn validate(&self) -> AiResult<()> {
        match self {
            Self::PerTensor | Self::PerElement => Ok(()),

            Self::PerAxis { axis } => {
                validate_text(axis, "quantization axis")
            }

            Self::PerGroup { .. } | Self::Custom { .. } => Ok(()),
        }
    }

    /// Returns whether the granularity requires external validation.
    #[must_use]
    pub const fn requires_extension_validation(&self) -> bool {
        matches!(self, Self::Custom { .. })
    }
}


// ============================================================================
// EXACT DECIMAL VALUE
// ============================================================================

/// An exact decimal value represented without binary floating-point loss.
///
/// The value is represented as text so that scale metadata does not
/// silently lose precision through an f64 conversion.
///
/// Accepted syntax:
///
/// - Integer decimal.
/// - Fractional decimal.
/// - Scientific notation.
/// - Optional leading sign.
///
/// Examples:
///
/// - `1`
/// - `0.125`
/// - `1.25e-4`
/// - `-2.5`
///
/// This type does not perform arbitrary-precision arithmetic.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DecimalValue(String);

impl DecimalValue {
    /// Creates a validated decimal value.
    pub fn new(value: impl Into<String>) -> AiResult<Self> {
        let value = value.into();

        validate_decimal(&value)?;

        Ok(Self(value))
    }

    /// Creates a strictly positive decimal scale.
    pub fn positive(value: impl Into<String>) -> AiResult<Self> {
        let value = value.into();

        validate_decimal(&value)?;

        if !decimal_is_positive_nonzero(&value) {
            return Err(quantization_error(
                "Quantization scale must be strictly positive.",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the original decimal representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DecimalValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn validate_decimal(value: &str) -> AiResult<()> {
    if value.is_empty() || value.trim() != value {
        return Err(quantization_error(
            "Decimal value is empty or contains surrounding whitespace.",
        ));
    }

    let bytes = value.as_bytes();
    let mut index = 0;

    if matches!(bytes.first(), Some(b'+') | Some(b'-')) {
        index += 1;
    }

    let mut integer_digits = 0usize;
    let mut fraction_digits = 0usize;

    while index < bytes.len() && bytes[index].is_ascii_digit() {
        integer_digits += 1;
        index += 1;
    }

    if index < bytes.len() && bytes[index] == b'.' {
        index += 1;

        while index < bytes.len() && bytes[index].is_ascii_digit() {
            fraction_digits += 1;
            index += 1;
        }
    }

    if integer_digits + fraction_digits == 0 {
        return Err(quantization_error(
            "Decimal value must contain at least one digit.",
        ));
    }

    if index < bytes.len()
        && matches!(bytes[index], b'e' | b'E')
    {
        index += 1;

        if index < bytes.len()
            && matches!(bytes[index], b'+' | b'-')
        {
            index += 1;
        }

        let exponent_start = index;

        while index < bytes.len()
            && bytes[index].is_ascii_digit()
        {
            index += 1;
        }

        if exponent_start == index {
            return Err(quantization_error(
                "Decimal exponent must contain digits.",
            ));
        }
    }

    if index != bytes.len() {
        return Err(quantization_error(
            "Decimal value contains invalid characters.",
        ));
    }

    Ok(())
}

fn decimal_is_positive_nonzero(value: &str) -> bool {
    if value.starts_with('-') {
        return false;
    }

    let mantissa = value
        .split_once(['e', 'E'])
        .map_or(value, |(mantissa, _)| mantissa);

    mantissa.bytes().any(|byte| {
        byte.is_ascii_digit() && byte != b'0'
    })
}


// ============================================================================
// ZERO POINT
// ============================================================================

/// Quantization zero-point declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ZeroPoint {
    /// Zero point is mathematically zero.
    Symmetric,

    /// Explicit signed integer zero point.
    ///
    /// Text representation avoids imposing an artificial i128
    /// numerical range on the descriptor.
    Explicit {
        /// Exact integer representation.
        value: String,
    },

    /// Zero point is obtained from calibration.
    Calibrated,

    /// Provider-defined zero point.
    ProviderDefined {
        /// Stable provider identity.
        provider: RequirementKey,
    },
}

impl ZeroPoint {
    /// Creates an explicit integer zero point.
    pub fn explicit(value: impl Into<String>) -> AiResult<Self> {
        let value = value.into();

        validate_integer(&value)?;

        Ok(Self::Explicit { value })
    }

    /// Validates the zero-point declaration.
    pub fn validate(&self) -> AiResult<()> {
        match self {
            Self::Symmetric | Self::Calibrated => Ok(()),

            Self::Explicit { value } => validate_integer(value),

            Self::ProviderDefined { .. } => Ok(()),
        }
    }

    /// Returns whether the zero point requires external resolution.
    #[must_use]
    pub const fn requires_external_resolution(&self) -> bool {
        matches!(
            self,
            Self::Calibrated | Self::ProviderDefined { .. }
        )
    }
}

fn validate_integer(value: &str) -> AiResult<()> {
    if value.is_empty() {
        return Err(quantization_error(
            "Zero-point integer cannot be empty.",
        ));
    }

    let bytes = value.as_bytes();
    let start = usize::from(
        matches!(bytes.first(), Some(b'+') | Some(b'-')),
    );

    if start == bytes.len()
        || !bytes[start..].iter().all(u8::is_ascii_digit)
    {
        return Err(quantization_error(
            "Zero point must be a valid signed decimal integer.",
        ));
    }

    Ok(())
}


// ============================================================================
// CALIBRATION METHOD
// ============================================================================

/// Calibration method identity.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CalibrationMethod {
    identity: RequirementKey,
}

impl CalibrationMethod {
    /// Creates a calibration method identity.
    #[must_use]
    pub fn new(identity: RequirementKey) -> Self {
        Self { identity }
    }

    /// Returns the method identity.
    #[must_use]
    pub fn identity(&self) -> &RequirementKey {
        &self.identity
    }
}


// ============================================================================
// CALIBRATION SOURCE
// ============================================================================

/// Origin of calibration information.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CalibrationSource {
    /// Calibration based on a dataset.
    Dataset {
        /// Dataset identity.
        dataset: RequirementKey,

        /// Dataset version or immutable snapshot identity.
        version: String,
    },

    /// Calibration based on a streaming source.
    Stream {
        /// Stable source identity.
        source: RequirementKey,

        /// Stream window or snapshot identity.
        window: String,
    },

    /// Calibration based on an explicit external report.
    External {
        /// Report identity.
        report: RequirementKey,
    },

    /// Calibration based on a provider.
    Provider {
        /// Provider identity.
        provider: RequirementKey,
    },
}

impl CalibrationSource {
    /// Validates source metadata.
    pub fn validate(&self) -> AiResult<()> {
        match self {
            Self::Dataset { version, .. } => {
                validate_text(version, "dataset version")
            }

            Self::Stream { window, .. } => {
                validate_text(window, "stream window")
            }

            Self::External { .. } | Self::Provider { .. } => Ok(()),
        }
    }
}


// ============================================================================
// CALIBRATION STATUS
// ============================================================================

/// Calibration evidence status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CalibrationStatus {
    /// Calibration has not been performed.
    NotPerformed,

    /// Calibration was performed but has not been independently validated.
    Unverified,

    /// Calibration has been validated.
    Verified,

    /// Calibration was demonstrated to be invalid.
    Invalid,

    /// Calibration validity is unknown.
    Unknown,
}


// ============================================================================
// CALIBRATION EVIDENCE
// ============================================================================

/// Evidence describing a calibration result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalibrationEvidence {
    source: CalibrationSource,
    method: CalibrationMethod,
    status: CalibrationStatus,
    result_reference: String,
    validation_references: BTreeSet<String>,
    deterministic: Option<bool>,
}

impl CalibrationEvidence {
    /// Creates calibration evidence.
    ///
    /// A verified status is a declaration that the independent
    /// calibration validator has completed its checks. This constructor
    /// does not itself perform or establish those checks.
    pub fn new(
        source: CalibrationSource,
        method: CalibrationMethod,
        status: CalibrationStatus,
        result_reference: impl Into<String>,
        validation_references: impl IntoIterator<Item = String>,
        deterministic: Option<bool>,
    ) -> AiResult<Self> {
        source.validate()?;

        let result_reference = result_reference.into();

        validate_text(
            &result_reference,
            "calibration result reference",
        )?;

        let mut references = BTreeSet::new();

        for reference in validation_references {
            validate_text(
                &reference,
                "calibration validation reference",
            )?;

            references.insert(reference);
        }

        if status == CalibrationStatus::Verified
            && references.is_empty()
        {
            return Err(quantization_error(
                "Verified calibration requires validation evidence.",
            ));
        }

        Ok(Self {
            source,
            method,
            status,
            result_reference,
            validation_references: references,
            deterministic,
        })
    }

    /// Returns the calibration source.
    #[must_use]
    pub fn source(&self) -> &CalibrationSource {
        &self.source
    }

    /// Returns the calibration method.
    #[must_use]
    pub fn method(&self) -> &CalibrationMethod {
        &self.method
    }

    /// Returns calibration status.
    #[must_use]
    pub const fn status(&self) -> CalibrationStatus {
        self.status
    }

    /// Returns the result reference.
    #[must_use]
    pub fn result_reference(&self) -> &str {
        &self.result_reference
    }

    /// Returns validation references.
    #[must_use]
    pub fn validation_references(&self) -> &BTreeSet<String> {
        &self.validation_references
    }

    /// Returns whether deterministic calibration is established.
    #[must_use]
    pub const fn deterministic(&self) -> Option<bool> {
        self.deterministic
    }

    /// Returns whether calibration has been verified.
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        matches!(self.status, CalibrationStatus::Verified)
    }
}


// ============================================================================
// CALIBRATION REQUIREMENT
// ============================================================================

/// Calibration requirement for a quantization transformation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CalibrationRequirement {
    /// Parameters are explicitly supplied.
    ExplicitParameters,

    /// Calibration evidence is required.
    Required,

    /// Calibration may be performed by an external provider.
    ProviderDefined {
        /// Provider identity.
        provider: RequirementKey,
    },
}


// ============================================================================
// QUANTIZATION PARAMETERS
// ============================================================================

/// Parameters associated with one quantization scope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuantizationParameters {
    granularity: QuantizationGranularity,
    scale: DecimalValue,
    zero_point: ZeroPoint,
}

impl QuantizationParameters {
    /// Creates quantization parameters.
    pub fn new(
        granularity: QuantizationGranularity,
        scale: DecimalValue,
        zero_point: ZeroPoint,
    ) -> AiResult<Self> {
        granularity.validate()?;

        if !decimal_is_positive_nonzero(scale.as_str()) {
            return Err(quantization_error(
                "Quantization scale must be positive and nonzero.",
            ));
        }

        zero_point.validate()?;

        Ok(Self {
            granularity,
            scale,
            zero_point,
        })
    }

    /// Returns the parameter granularity.
    #[must_use]
    pub fn granularity(&self) -> &QuantizationGranularity {
        &self.granularity
    }

    /// Returns the exact scale.
    #[must_use]
    pub fn scale(&self) -> &DecimalValue {
        &self.scale
    }

    /// Returns the zero point.
    #[must_use]
    pub fn zero_point(&self) -> &ZeroPoint {
        &self.zero_point
    }
}


// ============================================================================
// PER-SCOPE PARAMETERS
// ============================================================================

/// Parameters for an independently identified quantization scope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScopedQuantizationParameters {
    scope: RequirementKey,
    parameters: QuantizationParameters,
}

impl ScopedQuantizationParameters {
    /// Creates scoped parameters.
    #[must_use]
    pub fn new(
        scope: RequirementKey,
        parameters: QuantizationParameters,
    ) -> Self {
        Self {
            scope,
            parameters,
        }
    }

    /// Returns the scope identity.
    #[must_use]
    pub fn scope(&self) -> &RequirementKey {
        &self.scope
    }

    /// Returns the parameters.
    #[must_use]
    pub fn parameters(&self) -> &QuantizationParameters {
        &self.parameters
    }
}


// ============================================================================
// PARAMETER SET
// ============================================================================

/// Complete parameter description for a quantization transformation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuantizationParameterSet {
    global: Option<QuantizationParameters>,
    scoped: Vec<ScopedQuantizationParameters>,
}

impl QuantizationParameterSet {
    /// Creates a parameter set.
    ///
    /// Exactly one of the following must be supplied:
    ///
    /// - global parameters;
    /// - one or more scoped parameter sets.
    pub fn new(
        global: Option<QuantizationParameters>,
        scoped: Vec<ScopedQuantizationParameters>,
    ) -> AiResult<Self> {
        if global.is_some() == !scoped.is_empty() {
            return Err(quantization_error(
                "Parameter set must contain either global parameters or scoped parameters, but not both.",
            ));
        }

        let mut identities = BTreeSet::new();

        for entry in &scoped {
            if !identities.insert(entry.scope().clone()) {
                return Err(quantization_error(
                    "Duplicate quantization parameter scope.",
                ));
            }
        }

        Ok(Self { global, scoped })
    }

    /// Creates a global parameter set.
    pub fn global(
        parameters: QuantizationParameters,
    ) -> AiResult<Self> {
        Self::new(Some(parameters), Vec::new())
    }

    /// Creates a scoped parameter set.
    pub fn scoped(
        parameters: Vec<ScopedQuantizationParameters>,
    ) -> AiResult<Self> {
        Self::new(None, parameters)
    }

    /// Returns global parameters.
    #[must_use]
    pub fn global_parameters(&self) -> Option<&QuantizationParameters> {
        self.global.as_ref()
    }

    /// Returns scoped parameters.
    #[must_use]
    pub fn scoped_parameters(&self) -> &[ScopedQuantizationParameters] {
        &self.scoped
    }

    /// Returns whether parameters are globally scoped.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        self.global.is_some()
    }
}


// ============================================================================
// QUANTIZATION SPECIFICATION
// ============================================================================

/// Complete target-independent quantization specification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuantizationSpec {
    identity: QuantizationId,
    scheme: QuantizationScheme,
    granularity: QuantizationGranularity,
    parameters: Option<QuantizationParameterSet>,
    calibration_requirement: CalibrationRequirement,
    calibration_evidence: Option<CalibrationEvidence>,
    rounding: RoundingPolicy,
    overflow: OverflowPolicy,
    underflow: UnderflowPolicy,
    special_values: SpecialValuePolicy,
}

impl QuantizationSpec {
    /// Creates a quantization specification.
    pub fn new(
        identity: QuantizationId,
        scheme: QuantizationScheme,
        granularity: QuantizationGranularity,
        parameters: Option<QuantizationParameterSet>,
        calibration_requirement: CalibrationRequirement,
        calibration_evidence: Option<CalibrationEvidence>,
        rounding: RoundingPolicy,
        overflow: OverflowPolicy,
        underflow: UnderflowPolicy,
        special_values: SpecialValuePolicy,
    ) -> AiResult<Self> {
        granularity.validate()?;

        if let Some(evidence) = &calibration_evidence {
            if evidence.status() == CalibrationStatus::Invalid {
                return Err(quantization_error(
                    "Invalid calibration evidence cannot be used.",
                ));
            }
        }

        if matches!(
            calibration_requirement,
            CalibrationRequirement::ExplicitParameters
        ) && parameters.is_none()
        {
            return Err(quantization_error(
                "Explicit parameter calibration requires quantization parameters.",
            ));
        }

        if matches!(
            calibration_requirement,
            CalibrationRequirement::Required
        ) && calibration_evidence.is_none()
        {
            return Err(quantization_error(
                "Required calibration evidence was not supplied.",
            ));
        }

        if let Some(parameters) = &parameters {
            validate_parameter_granularity(
                &granularity,
                parameters,
            )?;
        }

        Ok(Self {
            identity,
            scheme,
            granularity,
            parameters,
            calibration_requirement,
            calibration_evidence,
            rounding,
            overflow,
            underflow,
            special_values,
        })
    }

    /// Returns the quantization identity.
    #[must_use]
    pub fn identity(&self) -> &QuantizationId {
        &self.identity
    }

    /// Returns the scheme.
    #[must_use]
    pub fn scheme(&self) -> &QuantizationScheme {
        &self.scheme
    }

    /// Returns the granularity.
    #[must_use]
    pub fn granularity(&self) -> &QuantizationGranularity {
        &self.granularity
    }

    /// Returns the parameter set.
    #[must_use]
    pub fn parameters(&self) -> Option<&QuantizationParameterSet> {
        self.parameters.as_ref()
    }

    /// Returns calibration evidence.
    #[must_use]
    pub fn calibration_evidence(&self) -> Option<&CalibrationEvidence> {
        self.calibration_evidence.as_ref()
    }

    /// Returns the rounding policy.
    #[must_use]
    pub fn rounding(&self) -> &RoundingPolicy {
        &self.rounding
    }

    /// Returns the overflow policy.
    #[must_use]
    pub fn overflow(&self) -> &OverflowPolicy {
        &self.overflow
    }

    /// Returns the underflow policy.
    #[must_use]
    pub fn underflow(&self) -> &UnderflowPolicy {
        &self.underflow
    }

    /// Returns the special-value policy.
    #[must_use]
    pub const fn special_values(&self) -> SpecialValuePolicy {
        self.special_values
    }

    /// Returns whether all locally required calibration is verified.
    #[must_use]
    pub fn calibration_is_verified(&self) -> bool {
        match self.calibration_requirement {
            CalibrationRequirement::ExplicitParameters => true,

            CalibrationRequirement::Required
            | CalibrationRequirement::ProviderDefined { .. } => {
                self.calibration_evidence
                    .as_ref()
                    .is_some_and(CalibrationEvidence::is_verified)
            }
        }
    }

    /// Returns whether the specification needs extension validation.
    #[must_use]
    pub fn requires_extension_validation(&self) -> bool {
        self.scheme.requires_extension_validation()
            || self.granularity.requires_extension_validation()
            || matches!(
                self.calibration_requirement,
                CalibrationRequirement::ProviderDefined { .. }
            )
    }
}

fn validate_parameter_granularity(
    expected: &QuantizationGranularity,
    parameters: &QuantizationParameterSet,
) -> AiResult<()> {
    if let Some(global) = parameters.global_parameters() {
        if global.granularity() != expected {
            return Err(quantization_error(
                "Global parameter granularity does not match the specification.",
            ));
        }
    }

    for scoped in parameters.scoped_parameters() {
        if scoped.parameters().granularity() != expected {
            return Err(quantization_error(
                "Scoped parameter granularity does not match the specification.",
            ));
        }
    }

    Ok(())
}


// ============================================================================
// QUANTIZATION TRANSFORMATION
// ============================================================================

/// A proposed quantization transformation.
///
/// This object describes a transformation. It does not modify tensor
/// storage, execute kernels or mutate model parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuantizationTransformation {
    source: PrecisionFormat,
    destination: PrecisionFormat,
    specification: QuantizationSpec,
}

impl QuantizationTransformation {
    /// Creates a quantization transformation.
    pub fn new(
        source: PrecisionFormat,
        destination: PrecisionFormat,
        specification: QuantizationSpec,
    ) -> AiResult<Self> {
        if !matches!(
            destination.dtype(),
            DType::Quantized(_)
        ) {
            return Err(quantization_error(
                "Quantization destination must use the canonical quantized DType.",
            ));
        }

        if matches!(
            source.dtype(),
            DType::Bool | DType::Quantized(_)
        ) {
            return Err(quantization_error(
                "Source dtype must be a supported non-boolean, non-quantized numerical type.",
            ));
        }

        validate_destination_semantics(
            destination.dtype(),
            specification.scheme(),
        )?;

        Ok(Self {
            source,
            destination,
            specification,
        })
    }

    /// Returns the source precision format.
    #[must_use]
    pub fn source(&self) -> &PrecisionFormat {
        &self.source
    }

    /// Returns the destination precision format.
    #[must_use]
    pub fn destination(&self) -> &PrecisionFormat {
        &self.destination
    }

    /// Returns the quantization specification.
    #[must_use]
    pub fn specification(&self) -> &QuantizationSpec {
        &self.specification
    }

    /// Returns whether calibration is verified.
    #[must_use]
    pub fn calibration_is_verified(&self) -> bool {
        self.specification.calibration_is_verified()
    }

    /// Converts the quantization proposal into the canonical precision
    /// transformation contract.
    ///
    /// Numerical legality is still decided by PrecisionValidator.
    ///
    /// This method does not claim that the transformation is legal.
    pub fn as_precision_transformation(
        &self,
    ) -> AiResult<PrecisionTransformation> {
        let error_contract = self
            .specification
            .identity()
            .key()
            .clone();

        PrecisionTransformation::new(
            error_contract,
            self.source.clone(),
            self.destination.clone(),
            PrecisionTransformationKind::Approximate {
                error_contract: self
                    .specification
                    .identity()
                    .key()
                    .clone(),
            },
            self.specification.rounding().clone(),
            self.specification.overflow().clone(),
            self.specification.underflow().clone(),
            self.specification.special_values(),
        )
    }

    /// Validates locally available quantization invariants.
    ///
    /// This does not replace semantic validation or numerical proof.
    pub fn validate_metadata(&self) -> AiResult<()> {
        validate_destination_semantics(
            self.destination.dtype(),
            self.specification.scheme(),
        )?;

        if self.specification.requires_extension_validation() {
            return Err(quantization_error(
                "Quantization scheme or granularity requires independent extension validation.",
            ));
        }

        if !self.specification.calibration_is_verified() {
            return Err(quantization_error(
                "Calibration has not been verified.",
            ));
        }

        if self.specification.parameters().is_none()
            && self.specification.calibration_evidence().is_none()
        {
            return Err(quantization_error(
                "Quantization parameters and calibration evidence are both absent.",
            ));
        }

        Ok(())
    }
}


// ============================================================================
// DESTINATION VALIDATION
// ============================================================================

fn validate_destination_semantics(
    dtype: &DType,
    scheme: &QuantizationScheme,
) -> AiResult<()> {
    let quantized = match dtype {
        DType::Quantized(value) => value,

        _ => {
            return Err(quantization_error(
                "Destination dtype is not quantized.",
            ));
        }
    };

    if quantized.value_bits == 0 {
        return Err(quantization_error(
            "Quantized destination has zero value width.",
        ));
    }

    match scheme {
        QuantizationScheme::Symmetric => {
            if quantized.zero_point != QuantizationZeroPoint::Symmetric {
                return Err(quantization_error(
                    "Symmetric quantization requires symmetric zero-point semantics.",
                ));
            }
        }

        QuantizationScheme::Affine => {
            // Affine quantization permits symmetric or asymmetric
            // zero-point semantics, provided the declared contract
            // and its validation evidence agree.
        }

        QuantizationScheme::Asymmetric => {
            if quantized.zero_point == QuantizationZeroPoint::Symmetric {
                return Err(quantization_error(
                    "Asymmetric quantization cannot declare a symmetric zero point.",
                ));
            }
        }

        QuantizationScheme::Logarithmic { .. }
        | QuantizationScheme::Custom { .. } => {
            // Extension-owned mathematical validation is required.
        }
    }

    Ok(())
}


// ============================================================================
// QUANTIZATION RANGE
// ============================================================================

/// Describes the representable quantized integer range.
///
/// The range is calculated without imposing a fixed integer-width ceiling.
///
/// The returned decimal strings are exact mathematical bounds.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuantizedRange {
    minimum: String,
    maximum: String,
}

impl QuantizedRange {
    /// Returns the minimum representable value.
    #[must_use]
    pub fn minimum(&self) -> &str {
        &self.minimum
    }

    /// Returns the maximum representable value.
    #[must_use]
    pub fn maximum(&self) -> &str {
        &self.maximum
    }
}

/// Calculates the representable integer range.
///
/// This calculation supports widths represented by u128 without
/// converting them into a fixed-width machine integer.
///
/// Very large decimal results still require available memory.
pub fn representable_range(
    signedness: IntegerSignedness,
    value_bits: u128,
) -> AiResult<QuantizedRange> {
    if value_bits == 0 {
        return Err(quantization_error(
            "Quantized integer width must be nonzero.",
        ));
    }

    let magnitude = power_of_two_decimal(value_bits - u128::from(
        signedness == IntegerSignedness::Signed,
    ))?;

    match signedness {
        IntegerSignedness::Unsigned => {
            let maximum = subtract_one_decimal(&magnitude)?;

            Ok(QuantizedRange {
                minimum: "0".to_owned(),
                maximum,
            })
        }

        IntegerSignedness::Signed => {
            let maximum = subtract_one_decimal(&magnitude)?;
            let minimum = format!("-{magnitude}");

            Ok(QuantizedRange {
                minimum,
                maximum,
            })
        }
    }
}

fn power_of_two_decimal(exponent: u128) -> AiResult<String> {
    let mut digits = vec![1u8];

    let mut remaining = exponent;

    while remaining > 0 {
        let mut carry = 0u16;

        for digit in &mut digits {
            let value = u16::from(*digit) * 2 + carry;

            *digit = (value % 10) as u8;
            carry = value / 10;
        }

        while carry > 0 {
            digits.push((carry % 10) as u8);
            carry /= 10;
        }

        remaining -= 1;
    }

    let mut result = String::with_capacity(digits.len());

    for digit in digits.iter().rev() {
        result.push(char::from(b'0' + *digit));
    }

    Ok(result)
}

fn subtract_one_decimal(value: &str) -> AiResult<String> {
    let mut digits: Vec<u8> = value
        .bytes()
        .map(|byte| byte - b'0')
        .collect();

    let mut index = digits.len();

    while index > 0 {
        index -= 1;

        if digits[index] > 0 {
            digits[index] -= 1;
            break;
        }

        digits[index] = 9;
    }

    while digits.len() > 1 && digits.first() == Some(&0) {
        digits.remove(0);
    }

    let result: String = digits
        .iter()
        .map(|digit| char::from(b'0' + *digit))
        .collect();

    if result.is_empty() {
        return Err(quantization_error(
            "Failed to calculate quantized numerical range.",
        ));
    }

    Ok(result)
}


// ============================================================================
// DEQUANTIZATION
// ============================================================================

/// Dequantization semantics.
///
/// Dequantization restores values into a declared numerical format.
/// It does not guarantee recovery of the original pre-quantization values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DequantizationContract {
    source: PrecisionFormat,
    destination: PrecisionFormat,
    quantization: QuantizationId,
    rounding: RoundingPolicy,
}

impl DequantizationContract {
    /// Creates a dequantization contract.
    pub fn new(
        source: PrecisionFormat,
        destination: PrecisionFormat,
        quantization: QuantizationId,
        rounding: RoundingPolicy,
    ) -> AiResult<Self> {
        if !matches!(
            source.dtype(),
            DType::Quantized(_)
        ) {
            return Err(quantization_error(
                "Dequantization source must use a quantized dtype.",
            ));
        }

        if matches!(
            destination.dtype(),
            DType::Quantized(_) | DType::Bool
        ) {
            return Err(quantization_error(
                "Dequantization destination must use a non-quantized numerical dtype.",
            ));
        }

        Ok(Self {
            source,
            destination,
            quantization,
            rounding,
        })
    }

    /// Returns the source format.
    #[must_use]
    pub fn source(&self) -> &PrecisionFormat {
        &self.source
    }

    /// Returns the destination format.
    #[must_use]
    pub fn destination(&self) -> &PrecisionFormat {
        &self.destination
    }

    /// Returns the quantization identity.
    #[must_use]
    pub fn quantization(&self) -> &QuantizationId {
        &self.quantization
    }

    /// Returns the rounding policy.
    #[must_use]
    pub fn rounding(&self) -> &RoundingPolicy {
        &self.rounding
    }
}


// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Result of quantization metadata validation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum QuantizationValidation {
    /// All locally checkable requirements were satisfied.
    MetadataValid {
        /// Validation references.
        evidence_references: BTreeSet<String>,
    },

    /// The specification violates a declared invariant.
    Invalid {
        /// Explanations of the violations.
        reasons: Vec<String>,
    },

    /// Additional independent validation is necessary.
    Indeterminate {
        /// Unresolved obligations.
        unresolved: Vec<String>,
    },
}


// ============================================================================
// VALIDATOR
// ============================================================================

/// Stateless quantization metadata validator.
#[derive(Clone, Copy, Debug, Default)]
pub struct QuantizationValidator;

impl QuantizationValidator {
    /// Creates a quantization validator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Validates a quantization transformation.
    ///
    /// The validator never claims numerical equivalence merely because
    /// metadata is structurally valid.
    pub fn validate(
        &self,
        transformation: &QuantizationTransformation,
    ) -> QuantizationValidation {
        let mut unresolved = Vec::new();
        let mut evidence = BTreeSet::new();

        if let Err(error) = transformation.validate_metadata() {
            let message = error.to_string();

            if message.contains("not been verified")
                || message.contains("extension validation")
            {
                unresolved.push(message);
            } else {
                return QuantizationValidation::Invalid {
                    reasons: vec![message],
                };
            }
        }

        if let Some(calibration) =
            transformation.specification.calibration_evidence()
        {
            evidence.insert(
                calibration.result_reference().to_owned(),
            );

            evidence.extend(
                calibration.validation_references().iter().cloned(),
            );

            if calibration.status() != CalibrationStatus::Verified {
                unresolved.push(
                    "Calibration evidence is not verified.".to_owned(),
                );
            }

            if calibration.deterministic() != Some(true) {
                unresolved.push(
                    "Deterministic calibration has not been established."
                        .to_owned(),
                );
            }
        }

        if transformation
            .specification
            .requires_extension_validation()
        {
            unresolved.push(
                "Extension-defined quantization semantics require external validation."
                    .to_owned(),
            );
        }

        if unresolved.is_empty() {
            QuantizationValidation::MetadataValid {
                evidence_references: evidence,
            }
        } else {
            unresolved.sort();
            unresolved.dedup();

            QuantizationValidation::Indeterminate {
                unresolved,
            }
        }
    }
}


// ============================================================================
// ERRORS
// ============================================================================

fn validate_text(
    value: &str,
    description: &str,
) -> AiResult<()> {
    if value.trim().is_empty()
        || value.chars().any(char::is_control)
    {
        return Err(quantization_error(format!(
            "{description} is empty or contains control characters."
        )));
    }

    Ok(())
}

fn quantization_error(
    message: impl Into<String>,
) -> AiError {
    AiError::new(
        AiErrorKind::OptimizationFailed,
        message,
    )
}


// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_values_preserve_exact_text() {
        let value = DecimalValue::positive("0.000000125")
            .expect("valid scale");

        assert_eq!(value.as_str(), "0.000000125");
    }

    #[test]
    fn zero_and_negative_scales_are_rejected() {
        assert!(DecimalValue::positive("0").is_err());
        assert!(DecimalValue::positive("-0.25").is_err());
    }

    #[test]
    fn malformed_decimal_is_rejected() {
        assert!(DecimalValue::new("").is_err());
        assert!(DecimalValue::new("1.2.3").is_err());
        assert!(DecimalValue::new("1e").is_err());
        assert!(DecimalValue::new("abc").is_err());
    }

    #[test]
    fn scientific_notation_is_supported() {
        let value = DecimalValue::positive("1.25e-8")
            .expect("valid scientific notation");

        assert_eq!(value.as_str(), "1.25e-8");
    }

    #[test]
    fn explicit_zero_point_is_validated() {
        assert!(ZeroPoint::explicit("-12").is_ok());
        assert!(ZeroPoint::explicit("0").is_ok());
        assert!(ZeroPoint::explicit("12.5").is_err());
    }

    #[test]
    fn duplicate_scopes_are_rejected() {
        let key = RequirementKey::new("zamani", "channel")
            .expect("valid identity");

        let granularity = QuantizationGranularity::PerAxis {
            axis: "channel".to_owned(),
        };

        let parameters = QuantizationParameters::new(
            granularity,
            DecimalValue::positive("0.125").expect("scale"),
            ZeroPoint::Symmetric,
        )
        .expect("parameters");

        let first = ScopedQuantizationParameters::new(
            key.clone(),
            parameters.clone(),
        );

        let second = ScopedQuantizationParameters::new(
            key,
            parameters,
        );

        assert!(
            QuantizationParameterSet::scoped(vec![
                first,
                second,
            ]).is_err()
        );
    }

    #[test]
    fn signed_eight_bit_range_is_correct() {
        let range = representable_range(
            IntegerSignedness::Signed,
            8,
        ).expect("range");

        assert_eq!(range.minimum(), "-128");
        assert_eq!(range.maximum(), "127");
    }

    #[test]
    fn unsigned_eight_bit_range_is_correct() {
        let range = representable_range(
            IntegerSignedness::Unsigned,
            8,
        ).expect("range");

        assert_eq!(range.minimum(), "0");
        assert_eq!(range.maximum(), "255");
    }

    #[test]
    fn arbitrary_width_range_is_not_limited_to_machine_integer_width() {
        let range = representable_range(
            IntegerSignedness::Unsigned,
            256,
        ).expect("range");

        assert_eq!(
            range.maximum(),
            "115792089237316195423570985008687907853269984665640564039457584007913129639935"
        );
    }

    #[test]
    fn zero_width_is_rejected() {
        assert!(
            representable_range(
                IntegerSignedness::Unsigned,
                0,
            ).is_err()
        );
    }

    #[test]
    fn parameter_set_requires_one_scope_model() {
        assert!(
            QuantizationParameterSet::new(
                None,
                Vec::new(),
            ).is_err()
        );
    }

    #[test]
    fn calibration_requires_validation_references_when_verified() {
        let source = CalibrationSource::External {
            report: RequirementKey::new(
                "zamani",
                "calibration-report",
            ).expect("identity"),
        };

        let method = CalibrationMethod::new(
            RequirementKey::new(
                "zamani",
                "calibration-method",
            ).expect("identity"),
        );

        assert!(
            CalibrationEvidence::new(
                source,
                method,
                CalibrationStatus::Verified,
                "result-1",
                Vec::new(),
                Some(true),
            ).is_err()
        );
    }

    #[test]
    fn calibration_unknown_is_not_verified() {
        let source = CalibrationSource::External {
            report: RequirementKey::new(
                "zamani",
                "report",
            ).expect("identity"),
        };

        let method = CalibrationMethod::new(
            RequirementKey::new(
                "zamani",
                "method",
            ).expect("identity"),
        );

        let evidence = CalibrationEvidence::new(
            source,
            method,
            CalibrationStatus::Unknown,
            "result",
            vec!["validation".to_owned()],
            None,
        ).expect("evidence");

        assert!(!evidence.is_verified());
    }
}
