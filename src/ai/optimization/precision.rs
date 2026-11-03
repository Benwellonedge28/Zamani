
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani AI Precision Optimization
//!
//! Path: src/ai/optimization/precision.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Define target-independent numerical precision transformation contracts
//! and determine whether proposed precision transformations are legal.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Precision transformation descriptions.
//! - Source and destination numerical format identities.
//! - Explicit rounding policies.
//! - Overflow and underflow policies.
//! - Special-value preservation requirements.
//! - Exact and approximate transformation declarations.
//! - Numerical validation evidence.
//! - Precision transformation legality decisions.
//! - Deterministic precision contract validation.
//!
//! DOES NOT OWN
//!
//! - Canonical AI tensor data types.
//! - Canonical source-language types.
//! - Tensor storage.
//! - Tensor numerical kernels.
//! - Quantization algorithms.
//! - Automatic differentiation.
//! - Hardware discovery.
//! - Device-specific numerical representations.
//! - Resource allocation.
//! - Scheduling.
//! - Placement.
//! - ZUIR construction.
//! - Runtime execution.
//!
//! AUTHORITIES
//!
//! AI numerical types:
//!     crate::ai::tensor::dtypes
//!
//! AI precision requirements:
//!     crate::ai::requirements
//!
//! AI errors:
//!     crate::ai::error
//!
//! Numerical reproducibility:
//!     crate::classical::numeric::reproducibility
//!
//! Source optimization intent:
//!     grammar/ai/optimization.g4
//!
//! Floating-point source semantics:
//!     grammar/classical/floating-point.g4
//!
//! Universal tensor IR:
//!     crate::zuir::dialects::tensor
//!
//! SCALABILITY
//!
//! This module imposes no universal limit on:
//!
//! - Numerical precision.
//! - Integer width.
//! - Floating-point width.
//! - Tensor rank.
//! - Tensor dimensions.
//! - Model parameters.
//! - Model size.
//! - Number of precision transformations.
//! - Number of optimization passes.
//! - Number of execution targets.
//!
//! Representation descriptors use the canonical DType representation.
//!
//! Actual representability and execution feasibility are determined by
//! downstream implementations and available resources.
//!
//! POCO-REAF
//!
//! Numerical meaning is independent of the physical representation selected
//! by an execution target.
//!
//! A transformation must preserve the declared numerical contract.
//!
//! Lossy transformations must be explicit and supported by validation
//! evidence.
//!
//! An unknown validation result is never treated as success.
//!
//! SAFETY
//!
//! Safe Rust only.
//!
//! FREEZE CRITERIA
//!
//! - No duplicate numerical type system.
//! - No implicit precision reduction.
//! - No silent rounding or overflow-policy substitution.
//! - No unverified approximate transformation.
//! - No false claims of target support.
//! - No hidden hardware dependencies.
//! - All legality outcomes are explicit.

use core::fmt;
use std::collections::BTreeSet;

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiResult,
};

use crate::ai::requirements::{
    NumericalFormat,
    PrecisionRequirement,
    RequirementKey,
    RoundingRequirement,
};

use crate::ai::tensor::dtypes::{
    DType,
    NumericCategory,
};


// ============================================================================
// FORMAT IDENTITY
// ============================================================================

/// A numerical precision format.
///
/// The format combines an open-world numerical format identity with
/// the canonical AI tensor dtype descriptor.
///
/// The identity describes semantic numerical intent.
///
/// The dtype describes the mathematical numerical category and
/// representation properties.
///
/// Neither establishes hardware support.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrecisionFormat {
    identity: NumericalFormat,
    dtype: DType,
}

impl PrecisionFormat {
    /// Creates a precision format.
    ///
    /// The dtype remains owned by the canonical AI tensor type subsystem.
    pub fn new(
        identity: NumericalFormat,
        dtype: DType,
    ) -> AiResult<Self> {
        validate_dtype(&dtype)?;

        Ok(Self {
            identity,
            dtype,
        })
    }

    /// Returns the numerical format identity.
    #[must_use]
    pub fn identity(&self) -> &NumericalFormat {
        &self.identity
    }

    /// Returns the canonical dtype descriptor.
    #[must_use]
    pub fn dtype(&self) -> &DType {
        &self.dtype
    }

    /// Returns the canonical numerical format key.
    #[must_use]
    pub fn key(&self) -> &RequirementKey {
        self.identity.key()
    }

    /// Returns whether two precision formats have identical
    /// declared numerical semantics.
    #[must_use]
    pub fn has_identical_semantics(
        &self,
        other: &Self,
    ) -> bool {
        self.identity == other.identity
            && self.dtype == other.dtype
    }
}


// ============================================================================
// NUMERICAL FORMAT VALIDATION
// ============================================================================

fn validate_dtype(dtype: &DType) -> AiResult<()> {
    match dtype {
        DType::Bool => Ok(()),

        DType::Integer(integer) => {
            if integer.value_bits == 0 {
                return Err(invalid_precision(
                    "Integer precision must be nonzero.",
                ));
            }

            Ok(())
        }

        DType::Float(float) => {
            if float.exponent_bits == 0
                || float.fraction_bits == 0
            {
                return Err(invalid_precision(
                    "Floating-point exponent and fraction widths must be nonzero.",
                ));
            }

            Ok(())
        }

        DType::Complex(complex) => {
            if complex.component.exponent_bits == 0
                || complex.component.fraction_bits == 0
            {
                return Err(invalid_precision(
                    "Complex component precision is invalid.",
                ));
            }

            Ok(())
        }

        DType::Quantized(quantized) => {
            if quantized.value_bits == 0 {
                return Err(invalid_precision(
                    "Quantized precision must be nonzero.",
                ));
            }

            Ok(())
        }

        DType::Custom(custom) => {
            if custom.namespace.trim().is_empty()
                || custom.name.trim().is_empty()
            {
                return Err(invalid_precision(
                    "Custom numerical type identity is invalid.",
                ));
            }

            if let Some(representation) = &custom.representation {
                if representation.bit_width == 0 {
                    return Err(invalid_precision(
                        "Custom numerical representation has zero width.",
                    ));
                }
            }

            Ok(())
        }
    }
}


// ============================================================================
// ROUNDING
// ============================================================================

/// Rounding behavior for a precision transformation.
///
/// Named policies are resolved by their owning numerical semantic service.
/// This module does not silently substitute one policy for another.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RoundingPolicy {
    /// Preserve the rounding behavior defined by the original operation.
    OperationDefined,

    /// Use a specifically identified rounding contract.
    Named(RequirementKey),

    /// A provider-defined rounding implementation.
    ///
    /// Its behavior must be validated before the transformation is accepted.
    ProviderDefined(RequirementKey),
}

impl RoundingPolicy {
    /// Returns whether the rounding behavior is fully specified locally.
    #[must_use]
    pub const fn is_explicit(&self) -> bool {
        !matches!(self, Self::ProviderDefined(_))
    }

    /// Returns the named rounding policy, if one exists.
    #[must_use]
    pub fn key(&self) -> Option<&RequirementKey> {
        match self {
            Self::Named(key) | Self::ProviderDefined(key) => Some(key),
            Self::OperationDefined => None,
        }
    }
}


// ============================================================================
// OVERFLOW
// ============================================================================

/// Overflow behavior required by a precision transformation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum OverflowPolicy {
    /// Preserve the original operation's overflow semantics.
    PreserveSourceSemantics,

    /// Report overflow instead of silently changing the value.
    Trap,

    /// Saturate according to the declared numerical contract.
    Saturate,

    /// Wrap according to the declared integer semantics.
    Wrap,

    /// Delegate to a named provider contract.
    ProviderDefined(RequirementKey),
}


// ============================================================================
// UNDERFLOW
// ============================================================================

/// Underflow behavior required by a precision transformation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UnderflowPolicy {
    /// Preserve source-operation underflow semantics.
    PreserveSourceSemantics,

    /// Report underflow.
    Trap,

    /// Flush according to an explicitly validated contract.
    Flush,

    /// Preserve subnormal values where supported by the declared semantics.
    PreserveSubnormal,

    /// Use a named provider contract.
    ProviderDefined(RequirementKey),
}


// ============================================================================
// SPECIAL VALUES
// ============================================================================

/// Required treatment of special numerical values.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SpecialValueBehavior {
    /// Preserve the specified source behavior.
    Preserve,

    /// The source and destination semantics establish that the value
    /// cannot occur in the relevant computation.
    ProvenAbsent,

    /// The behavior is determined by an explicit numerical contract.
    ContractDefined,
}

/// Special-value preservation contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpecialValuePolicy {
    /// NaN behavior.
    pub nan: SpecialValueBehavior,

    /// Positive and negative infinity behavior.
    pub infinity: SpecialValueBehavior,

    /// Signed-zero behavior.
    pub signed_zero: SpecialValueBehavior,

    /// Subnormal-value behavior.
    pub subnormal: SpecialValueBehavior,
}

impl SpecialValuePolicy {
    /// Creates an explicit special-value policy.
    #[must_use]
    pub const fn new(
        nan: SpecialValueBehavior,
        infinity: SpecialValueBehavior,
        signed_zero: SpecialValueBehavior,
        subnormal: SpecialValueBehavior,
    ) -> Self {
        Self {
            nan,
            infinity,
            signed_zero,
            subnormal,
        }
    }

    /// Creates a policy requiring preservation of all special values.
    #[must_use]
    pub const fn preserve_all() -> Self {
        Self {
            nan: SpecialValueBehavior::Preserve,
            infinity: SpecialValueBehavior::Preserve,
            signed_zero: SpecialValueBehavior::Preserve,
            subnormal: SpecialValueBehavior::Preserve,
        }
    }
}


// ============================================================================
// TRANSFORMATION KIND
// ============================================================================

/// Numerical equivalence class of a precision transformation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PrecisionTransformationKind {
    /// The declared mathematical and observable numerical semantics
    /// must remain equivalent.
    Exact,

    /// The transformation permits a bounded numerical approximation.
    ///
    /// The contract identity must resolve to a separately validated
    /// numerical error contract.
    Approximate {
        /// Identity of the approximation contract.
        error_contract: RequirementKey,
    },

    /// A provider-defined transformation.
    ///
    /// Legality cannot be established without provider evidence.
    ProviderDefined {
        /// Stable provider transformation identity.
        provider: RequirementKey,
    },
}


// ============================================================================
// NUMERICAL ERROR EVIDENCE
// ============================================================================

/// Evidence status for a numerical preservation obligation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ProofStatus {
    /// The obligation has been demonstrated.
    Proven,

    /// The obligation has been demonstrated to be violated.
    Disproven,

    /// The obligation has not been established.
    Unknown,
}

/// Evidence for one numerical preservation obligation.
///
/// The evidence reference identifies an independently retained proof,
/// analysis report, test result, or validated numerical contract.
///
/// This module does not pretend that the presence of an identifier
/// independently proves the underlying mathematical claim.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrecisionProof {
    status: ProofStatus,
    evidence_reference: String,
}

impl PrecisionProof {
    /// Creates numerical proof evidence.
    pub fn new(
        status: ProofStatus,
        evidence_reference: impl Into<String>,
    ) -> AiResult<Self> {
        let evidence_reference = evidence_reference.into();

        if evidence_reference.trim().is_empty() {
            return Err(invalid_precision(
                "Precision proof must identify its evidence source.",
            ));
        }

        if evidence_reference.chars().any(char::is_control) {
            return Err(invalid_precision(
                "Precision evidence reference contains control characters.",
            ));
        }

        Ok(Self {
            status,
            evidence_reference,
        })
    }

    /// Returns the proof status.
    #[must_use]
    pub const fn status(&self) -> ProofStatus {
        self.status
    }

    /// Returns the evidence reference.
    #[must_use]
    pub fn evidence_reference(&self) -> &str {
        &self.evidence_reference
    }
}


// ============================================================================
// VALIDATION EVIDENCE
// ============================================================================

/// Evidence required to validate a precision transformation.
///
/// The obligations are deliberately separate. Proving that the numerical
/// error is bounded does not automatically prove that overflow, special
/// values, reproducibility, or observable behavior are preserved.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrecisionValidationEvidence {
    numerical_error: PrecisionProof,
    rounding: PrecisionProof,
    overflow: PrecisionProof,
    underflow: PrecisionProof,
    special_values: PrecisionProof,
    reproducibility: PrecisionProof,
    operation_semantics: PrecisionProof,
    evidence_references: BTreeSet<String>,
}

impl PrecisionValidationEvidence {
    /// Creates a complete precision validation evidence record.
    pub fn new(
        numerical_error: PrecisionProof,
        rounding: PrecisionProof,
        overflow: PrecisionProof,
        underflow: PrecisionProof,
        special_values: PrecisionProof,
        reproducibility: PrecisionProof,
        operation_semantics: PrecisionProof,
        additional_references: impl IntoIterator<Item = String>,
    ) -> AiResult<Self> {
        let mut evidence_references = BTreeSet::new();

        for reference in additional_references {
            if reference.trim().is_empty()
                || reference.chars().any(char::is_control)
            {
                return Err(invalid_precision(
                    "Additional precision evidence reference is invalid.",
                ));
            }

            evidence_references.insert(reference);
        }

        for proof in [
            &numerical_error,
            &rounding,
            &overflow,
            &underflow,
            &special_values,
            &reproducibility,
            &operation_semantics,
        ] {
            evidence_references.insert(
                proof.evidence_reference.clone(),
            );
        }

        Ok(Self {
            numerical_error,
            rounding,
            overflow,
            underflow,
            special_values,
            reproducibility,
            operation_semantics,
            evidence_references,
        })
    }

    /// Returns the numerical-error evidence.
    #[must_use]
    pub fn numerical_error(&self) -> &PrecisionProof {
        &self.numerical_error
    }

    /// Returns the operation-semantics evidence.
    #[must_use]
    pub fn operation_semantics(&self) -> &PrecisionProof {
        &self.operation_semantics
    }

    /// Returns all evidence references in deterministic order.
    #[must_use]
    pub fn evidence_references(&self) -> &BTreeSet<String> {
        &self.evidence_references
    }

    fn proofs(&self) -> [&PrecisionProof; 7] {
        [
            &self.numerical_error,
            &self.rounding,
            &self.overflow,
            &self.underflow,
            &self.special_values,
            &self.reproducibility,
            &self.operation_semantics,
        ]
    }
}


// ============================================================================
// PRECISION TRANSFORMATION
// ============================================================================

/// A proposed numerical precision transformation.
///
/// This describes an optimization proposal. It does not execute the
/// transformation or mutate model parameters or tensor storage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrecisionTransformation {
    identity: RequirementKey,
    source: PrecisionFormat,
    destination: PrecisionFormat,
    kind: PrecisionTransformationKind,
    rounding: RoundingPolicy,
    overflow: OverflowPolicy,
    underflow: UnderflowPolicy,
    special_values: SpecialValuePolicy,
}

impl PrecisionTransformation {
    /// Creates a precision transformation proposal.
    pub fn new(
        identity: RequirementKey,
        source: PrecisionFormat,
        destination: PrecisionFormat,
        kind: PrecisionTransformationKind,
        rounding: RoundingPolicy,
        overflow: OverflowPolicy,
        underflow: UnderflowPolicy,
        special_values: SpecialValuePolicy,
    ) -> AiResult<Self> {
        if source.has_identical_semantics(&destination)
            && !matches!(
                kind,
                PrecisionTransformationKind::Exact
            )
        {
            return Err(invalid_precision(
                "An identical-format transformation must use the exact transformation kind.",
            ));
        }

        validate_dtype(source.dtype())?;
        validate_dtype(destination.dtype())?;

        Ok(Self {
            identity,
            source,
            destination,
            kind,
            rounding,
            overflow,
            underflow,
            special_values,
        })
    }

    /// Returns the transformation identity.
    #[must_use]
    pub fn identity(&self) -> &RequirementKey {
        &self.identity
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

    /// Returns the transformation kind.
    #[must_use]
    pub fn kind(&self) -> &PrecisionTransformationKind {
        &self.kind
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

    /// Returns whether the transformation changes the declared format.
    #[must_use]
    pub fn changes_format(&self) -> bool {
        !self.source.has_identical_semantics(&self.destination)
    }

    /// Returns whether the transformation explicitly permits approximation.
    #[must_use]
    pub fn is_approximate(&self) -> bool {
        matches!(
            self.kind,
            PrecisionTransformationKind::Approximate { .. }
        )
    }
}


// ============================================================================
// LEGALITY CONTEXT
// ============================================================================

/// Numerical reproducibility requirement.
///
/// This records the guarantee that the transformation must preserve.
/// Actual cross-target reproducibility is verified downstream.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ReproducibilityRequirementLevel {
    /// Preserve observable semantic behavior.
    Semantic,

    /// Preserve the applicable numerical guarantees.
    Numerical,

    /// Require bitwise identity within the declared verification scope.
    Bitwise,
}

/// Context used to assess precision transformation legality.
///
/// The context is supplied by semantic analysis and optimization
/// orchestration. It is not inferred from the current hardware.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrecisionLegalityContext {
    requirement: PrecisionRequirement,
    reproducibility: ReproducibilityRequirementLevel,
}

impl PrecisionLegalityContext {
    /// Creates a precision legality context.
    #[must_use]
    pub fn new(
        requirement: PrecisionRequirement,
        reproducibility: ReproducibilityRequirementLevel,
    ) -> Self {
        Self {
            requirement,
            reproducibility,
        }
    }

    /// Returns the canonical numerical precision requirement.
    #[must_use]
    pub fn requirement(&self) -> &PrecisionRequirement {
        &self.requirement
    }

    /// Returns the reproducibility requirement.
    #[must_use]
    pub const fn reproducibility(
        &self,
    ) -> ReproducibilityRequirementLevel {
        self.reproducibility
    }
}


// ============================================================================
// LEGALITY OUTCOME
// ============================================================================

/// Result of a precision transformation legality assessment.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PrecisionLegality {
    /// The transformation satisfies all declared obligations.
    Legal {
        /// References to the evidence supporting legality.
        evidence_references: BTreeSet<String>,
    },

    /// The transformation violates a declared requirement.
    Illegal {
        /// Explanation of the violation.
        reasons: Vec<String>,
    },

    /// The available evidence is insufficient to decide legality.
    Indeterminate {
        /// Obligations that still require validation.
        unresolved: Vec<String>,
    },
}

impl PrecisionLegality {
    /// Returns whether the transformation has been established as legal.
    #[must_use]
    pub const fn is_legal(&self) -> bool {
        matches!(self, Self::Legal { .. })
    }

    /// Returns whether the transformation has been established as illegal.
    #[must_use]
    pub const fn is_illegal(&self) -> bool {
        matches!(self, Self::Illegal { .. })
    }

    /// Returns whether additional evidence is necessary.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        matches!(self, Self::Indeterminate { .. })
    }
}


// ============================================================================
// LEGALITY VALIDATOR
// ============================================================================

/// Validates numerical precision transformations.
///
/// This validator is deterministic and independent of hardware discovery.
/// It does not claim target feasibility.
#[derive(Clone, Copy, Debug, Default)]
pub struct PrecisionValidator;

impl PrecisionValidator {
    /// Creates a precision validator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Validates a precision transformation.
    ///
    /// An approximate transformation is not legal merely because a
    /// destination dtype exists. It must be explicitly permitted by the
    /// precision requirement and have complete numerical validation evidence.
    pub fn validate(
        &self,
        transformation: &PrecisionTransformation,
        context: &PrecisionLegalityContext,
        evidence: Option<&PrecisionValidationEvidence>,
    ) -> AiResult<PrecisionLegality> {
        let mut reasons = Vec::new();
        let mut unresolved = Vec::new();

        let requirement = context.requirement();

        // ------------------------------------------------------------
        // Required destination format
        // ------------------------------------------------------------

        if transformation.destination.key()
            != requirement.format().key()
        {
            reasons.push(
                "Destination format does not satisfy the required numerical format."
                    .to_string(),
            );
        }

        // ------------------------------------------------------------
        // Required rounding
        // ------------------------------------------------------------

        if let RoundingRequirement::Named(required) =
            requirement.rounding()
        {
            match transformation.rounding.key() {
                Some(actual) if actual == required => {}

                Some(_) => reasons.push(
                    "Transformation uses a different named rounding policy."
                        .to_string(),
                ),

                None => reasons.push(
                    "Transformation does not identify the required rounding policy."
                        .to_string(),
                ),
            }
        }

        // ------------------------------------------------------------
        // Approximation policy
        // ------------------------------------------------------------

        if transformation.is_approximate()
            && !requirement.allows_lossy_conversion()
        {
            reasons.push(
                "The canonical precision requirement prohibits lossy conversion."
                    .to_string(),
            );
        }

        // ------------------------------------------------------------
        // Exact reproducibility
        // ------------------------------------------------------------

        if context.reproducibility()
            == ReproducibilityRequirementLevel::Bitwise
            && transformation.changes_format()
        {
            reasons.push(
                "A format-changing transformation cannot claim bitwise preservation without a separately established bitwise-equivalence contract."
                    .to_string(),
            );
        }

        // ------------------------------------------------------------
        // Provider-defined semantics
        // ------------------------------------------------------------

        if matches!(
            transformation.kind,
            PrecisionTransformationKind::ProviderDefined { .. }
        ) {
            unresolved.push(
                "Provider-defined transformation semantics require provider validation."
                    .to_string(),
            );
        }

        if matches!(
            transformation.overflow,
            OverflowPolicy::ProviderDefined(_)
        ) {
            unresolved.push(
                "Provider-defined overflow semantics require validation."
                    .to_string(),
            );
        }

        if matches!(
            transformation.underflow,
            UnderflowPolicy::ProviderDefined(_)
        ) {
            unresolved.push(
                "Provider-defined underflow semantics require validation."
                    .to_string(),
            );
        }

        // ------------------------------------------------------------
        // Validation evidence
        // ------------------------------------------------------------

        let identical = !transformation.changes_format()
            && matches!(
                transformation.kind,
                PrecisionTransformationKind::Exact
            );

        let mut evidence_references = BTreeSet::new();

        if !identical || !reasons.is_empty() {
            match evidence {
                None => {
                    unresolved.push(
                        "Numerical preservation evidence has not been supplied."
                            .to_string(),
                    );
                }

                Some(evidence) => {
                    evidence_references.extend(
                        evidence.evidence_references.iter().cloned(),
                    );

                    for (name, proof) in [
                        ("numerical error", &evidence.numerical_error),
                        ("rounding", &evidence.rounding),
                        ("overflow", &evidence.overflow),
                        ("underflow", &evidence.underflow),
                        ("special values", &evidence.special_values),
                        ("reproducibility", &evidence.reproducibility),
                        ("operation semantics", &evidence.operation_semantics),
                    ] {
                        match proof.status {
                            ProofStatus::Proven => {}

                            ProofStatus::Disproven => {
                                reasons.push(format!(
                                    "{} preservation was disproven.",
                                    name,
                                ));
                            }

                            ProofStatus::Unknown => {
                                unresolved.push(format!(
                                    "{} preservation remains unverified.",
                                    name,
                                ));
                            }
                        }
                    }
                }
            }
        }

        // ------------------------------------------------------------
        // Final decision
        // ------------------------------------------------------------

        if !reasons.is_empty() {
            return Ok(PrecisionLegality::Illegal {
                reasons,
            });
        }

        if !unresolved.is_empty() {
            return Ok(PrecisionLegality::Indeterminate {
                unresolved,
            });
        }

        Ok(PrecisionLegality::Legal {
            evidence_references,
        })
    }

    /// Validates that a precision requirement and a proposed transformation
    /// have internally valid format descriptors.
    pub fn validate_structure(
        &self,
        transformation: &PrecisionTransformation,
    ) -> AiResult<()> {
        validate_dtype(transformation.source.dtype())?;
        validate_dtype(transformation.destination.dtype())?;

        if transformation.identity.namespace().trim().is_empty()
            || transformation.identity.name().trim().is_empty()
        {
            return Err(invalid_precision(
                "Precision transformation identity is invalid.",
            ));
        }

        Ok(())
    }
}


// ============================================================================
// OPTIMIZATION RESULT
// ============================================================================

/// Validated precision optimization result.
///
/// This is an immutable description of an accepted transformation.
/// It does not contain transformed tensor data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedPrecisionTransformation {
    transformation: PrecisionTransformation,
    legality: PrecisionLegality,
}

impl ValidatedPrecisionTransformation {
    /// Creates a validated transformation.
    ///
    /// Only a legal transformation can be wrapped in this type.
    pub fn new(
        transformation: PrecisionTransformation,
        legality: PrecisionLegality,
    ) -> AiResult<Self> {
        if !legality.is_legal() {
            return Err(AiError::new(
                AiErrorKind::RequirementUnsatisfied,
                "Cannot construct a validated precision transformation from a non-legal assessment.",
            ));
        }

        Ok(Self {
            transformation,
            legality,
        })
    }

    /// Returns the accepted transformation.
    #[must_use]
    pub fn transformation(&self) -> &PrecisionTransformation {
        &self.transformation
    }

    /// Returns its legality evidence.
    #[must_use]
    pub fn legality(&self) -> &PrecisionLegality {
        &self.legality
    }
}


// ============================================================================
// ERRORS
// ============================================================================

fn invalid_precision(message: &'static str) -> AiError {
    AiError::new(
        AiErrorKind::InvalidInput,
        message,
    )
}


// ============================================================================
// DISPLAY
// ============================================================================

impl fmt::Display for PrecisionLegality {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Legal { .. } => {
                formatter.write_str("precision transformation is legal")
            }

            Self::Illegal { reasons } => {
                write!(
                    formatter,
                    "precision transformation is illegal: {}",
                    reasons.join("; "),
                )
            }

            Self::Indeterminate { unresolved } => {
                write!(
                    formatter,
                    "precision transformation is indeterminate: {}",
                    unresolved.join("; "),
                )
            }
        }
    }
}


// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::tensor::dtypes::{
        binary_float,
        signed_integer,
    };

    use crate::ai::requirements::{
        NumericalFormat,
        PrecisionRequirement,
        RequirementKey,
        RoundingRequirement,
    };

    fn key(name: &str) -> RequirementKey {
        RequirementKey::new(
            "zamani.test",
            name,
        )
        .expect("valid requirement identity")
    }

    fn format(
        name: &str,
        dtype: DType,
    ) -> PrecisionFormat {
        PrecisionFormat::new(
            NumericalFormat::new(key(name)),
            dtype,
        )
        .expect("valid precision format")
    }

    fn exact_context(
        destination: &PrecisionFormat,
    ) -> PrecisionLegalityContext {
        PrecisionLegalityContext::new(
            PrecisionRequirement::new(
                destination.identity().clone(),
                RoundingRequirement::OperationDefined,
                None,
                false,
            ),
            ReproducibilityRequirementLevel::Semantic,
        )
    }

    fn exact_transformation(
        source: PrecisionFormat,
        destination: PrecisionFormat,
    ) -> PrecisionTransformation {
        PrecisionTransformation::new(
            key("exact_transform"),
            source,
            destination,
            PrecisionTransformationKind::Exact,
            RoundingPolicy::OperationDefined,
            OverflowPolicy::PreserveSourceSemantics,
            UnderflowPolicy::PreserveSourceSemantics,
            SpecialValuePolicy::preserve_all(),
        )
        .expect("valid transformation")
    }

    fn proven_evidence() -> PrecisionValidationEvidence {
        let proof = || {
            PrecisionProof::new(
                ProofStatus::Proven,
                "test:precision-proof",
            )
            .expect("valid proof")
        };

        PrecisionValidationEvidence::new(
            proof(),
            proof(),
            proof(),
            proof(),
            proof(),
            proof(),
            proof(),
            Vec::new(),
        )
        .expect("valid evidence")
    }

    #[test]
    fn identical_exact_format_is_legal_without_approximation_evidence() {
        let dtype = binary_float(8, 23)
            .expect("valid float");

        let source = format("binary32", dtype.clone());
        let destination = format("binary32", dtype);

        let transformation = exact_transformation(
            source,
            destination.clone(),
        );

        let result = PrecisionValidator::new()
            .validate(
                &transformation,
                &exact_context(&destination),
                None,
            )
            .expect("validation succeeds");

        assert!(result.is_legal());
    }

    #[test]
    fn changed_format_without_evidence_is_indeterminate() {
        let source = format(
            "wide",
            binary_float(11, 52).expect("valid float"),
        );

        let destination = format(
            "narrow",
            binary_float(8, 23).expect("valid float"),
        );

        let transformation = exact_transformation(
            source,
            destination.clone(),
        );

        let result = PrecisionValidator::new()
            .validate(
                &transformation,
                &exact_context(&destination),
                None,
            )
            .expect("validation succeeds");

        assert!(result.is_indeterminate());
    }

    #[test]
    fn disproven_numerical_preservation_is_illegal() {
        let source = format(
            "wide",
            binary_float(11, 52).expect("valid float"),
        );

        let destination = format(
            "narrow",
            binary_float(8, 23).expect("valid float"),
        );

        let transformation = exact_transformation(
            source,
            destination.clone(),
        );

        let proven = proven_evidence();

        let disproven = PrecisionProof::new(
            ProofStatus::Disproven,
            "test:failed-error-bound",
        )
        .expect("valid evidence");

        let evidence = PrecisionValidationEvidence::new(
            disproven,
            proven.rounding.clone(),
            proven.overflow.clone(),
            proven.underflow.clone(),
            proven.special_values.clone(),
            proven.reproducibility.clone(),
            proven.operation_semantics.clone(),
            Vec::new(),
        )
        .expect("valid evidence");

        let result = PrecisionValidator::new()
            .validate(
                &transformation,
                &exact_context(&destination),
                Some(&evidence),
            )
            .expect("validation succeeds");

        assert!(result.is_illegal());
    }

    #[test]
    fn lossy_conversion_is_rejected_when_requirement_disallows_it() {
        let source = format(
            "wide",
            binary_float(11, 52).expect("valid float"),
        );

        let destination = format(
            "narrow",
            binary_float(8, 23).expect("valid float"),
        );

        let transformation = PrecisionTransformation::new(
            key("lossy"),
            source,
            destination.clone(),
            PrecisionTransformationKind::Approximate {
                error_contract: key("error_bound"),
            },
            RoundingPolicy::Named(key("rounding")),
            OverflowPolicy::Trap,
            UnderflowPolicy::Trap,
            SpecialValuePolicy::preserve_all(),
        )
        .expect("valid transformation");

        let result = PrecisionValidator::new()
            .validate(
                &transformation,
                &exact_context(&destination),
                Some(&proven_evidence()),
            )
            .expect("validation succeeds");

        assert!(result.is_illegal());
    }

    #[test]
    fn bitwise_reproducibility_rejects_unproven_format_changes() {
        let source = format(
            "wide",
            binary_float(11, 52).expect("valid float"),
        );

        let destination = format(
            "narrow",
            binary_float(8, 23).expect("valid float"),
        );

        let transformation = exact_transformation(
            source,
            destination.clone(),
        );

        let context = PrecisionLegalityContext::new(
            PrecisionRequirement::new(
                destination.identity().clone(),
                RoundingRequirement::OperationDefined,
                None,
                false,
            ),
            ReproducibilityRequirementLevel::Bitwise,
        );

        let result = PrecisionValidator::new()
            .validate(
                &transformation,
                &context,
                Some(&proven_evidence()),
            )
            .expect("validation succeeds");

        assert!(result.is_illegal());
    }

    #[test]
    fn named_rounding_requirement_must_match() {
        let source = format(
            "source",
            signed_integer(32).expect("valid integer"),
        );

        let destination = format(
            "destination",
            signed_integer(64).expect("valid integer"),
        );

        let transformation = PrecisionTransformation::new(
            key("rounding_test"),
            source,
            destination.clone(),
            PrecisionTransformationKind::Exact,
            RoundingPolicy::Named(key("different_rounding")),
            OverflowPolicy::PreserveSourceSemantics,
            UnderflowPolicy::PreserveSourceSemantics,
            SpecialValuePolicy::preserve_all(),
        )
        .expect("valid transformation");

        let context = PrecisionLegalityContext::new(
            PrecisionRequirement::new(
                destination.identity().clone(),
                RoundingRequirement::Named(key("required_rounding")),
                None,
                false,
            ),
            ReproducibilityRequirementLevel::Semantic,
        );

        let result = PrecisionValidator::new()
            .validate(
                &transformation,
                &context,
                Some(&proven_evidence()),
            )
            .expect("validation succeeds");

        assert!(result.is_illegal());
    }

    #[test]
    fn arbitrary_integer_widths_are_supported_as_descriptors() {
        let dtype = signed_integer(1024)
            .expect("valid integer width");

        let numerical_format = format(
            "wide_integer",
            dtype,
        );

        assert_eq!(
            numerical_format.dtype().bit_width(),
            Some(1024),
        );
    }

    #[test]
    fn custom_numerical_formats_remain_open_world() {
        let custom = DType::Custom(
            crate::ai::tensor::dtypes::CustomDType::new(
                "example.extension".to_string(),
                "extended_number".to_string(),
                NumericCategory::Custom,
                None,
            )
            .expect("valid custom dtype"),
        );

        let numerical_format = format(
            "extension_number",
            custom,
        );

        assert_eq!(
            numerical_format.dtype().category(),
            NumericCategory::Custom,
        );

        assert_eq!(
            numerical_format.dtype().bit_width(),
            None,
        );
    }

    #[test]
    fn indeterminate_transformations_cannot_be_marked_validated() {
        let source = format(
            "source",
            binary_float(11, 52).expect("valid float"),
        );

        let destination = format(
            "destination",
            binary_float(8, 23).expect("valid float"),
        );

        let transformation = exact_transformation(
            source,
            destination.clone(),
        );

        let result = PrecisionLegality::Indeterminate {
            unresolved: vec![
                "numerical preservation is unknown".to_string(),
            ],
        };

        assert!(
            ValidatedPrecisionTransformation::new(
                transformation,
                result,
            )
            .is_err()
        );
    }

    #[test]
    fn legality_results_are_explicit() {
        assert!(
            PrecisionLegality::Legal {
                evidence_references: BTreeSet::new(),
            }
            .is_legal()
        );

        assert!(
            PrecisionLegality::Indeterminate {
                unresolved: Vec::new(),
            }
            .is_indeterminate()
        );

        assert!(
            PrecisionLegality::Illegal {
                reasons: Vec::new(),
            }
            .is_illegal()
        );
    }
}
