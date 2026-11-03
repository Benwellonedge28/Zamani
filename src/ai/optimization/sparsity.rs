
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani AI Sparse Optimization Contracts
//!
//! Path: `src/ai/optimization/sparsity.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Define target-independent sparse computation and sparsity transformation
//! contracts for Zamani AI programs.
//!
//! This module describes sparse representations and the semantic obligations
//! associated with transformations between them. It does not allocate sparse
//! tensors, execute sparse kernels, mutate model graphs, or select hardware.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Sparse encoding descriptions.
//! - Structural sparsity descriptions.
//! - Implicit-value semantics.
//! - Density metadata.
//! - Sparse transformation contracts.
//! - Sparse transformation validation evidence.
//! - Sparse transformation legality classification.
//!
//! ## Does not own
//!
//! - Canonical tensor shapes or dimensions.
//! - Tensor storage or allocation.
//! - Sparse numerical kernels.
//! - Model or pipeline graph ownership.
//! - General graph transformations.
//! - Numerical precision semantics.
//! - Quantization semantics or calibration.
//! - Hardware-specific sparse representations.
//! - Resource discovery or feasibility.
//! - Scheduling, placement, routing, or execution.
//! - ZUIR construction or lowering.
//!
//! ## Authorities
//!
//! - `crate::ai::error`:
//!   structured AI errors.
//! - `crate::ai::requirements`:
//!   open-world semantic identities.
//! - `crate::ai::optimization::precision`:
//!   numerical precision transformation contracts.
//! - `crate::ai::optimization::quantization`:
//!   quantization and calibration contracts.
//! - `crate::ai::optimization::transformations`:
//!   general transformation identities and transactional graph changes.
//! - `crate::ai::tensor::dtypes`:
//!   canonical numerical type descriptions.
//! - `crate::zuir::dialects::tensor`:
//!   eventual universal tensor operation representation.
//!
//! ## Semantic invariants
//!
//! A sparse representation is not proof that a sparse transformation is
//! legal.
//!
//! In particular:
//!
//! - Implicit values must have a declared meaning.
//! - Zero elision must be justified by the operation semantics.
//! - Duplicate-coordinate behavior must be preserved where applicable.
//! - Ordering and index semantics must be preserved where applicable.
//! - Shape compatibility must be established by the canonical shape system.
//! - Differentiation semantics must be preserved where required.
//! - Precision and quantization contracts must not be silently changed.
//! - Effects, provenance, and reproducibility obligations must be preserved.
//! - Unknown evidence must never be treated as proof of legality.
//!
//! This module validates the declared evidence status and its structural
//! completeness. It does not independently prove mathematical claims or
//! authenticate external evidence.
//!
//! ## Scalability
//!
//! This module imposes no language-level maximum on:
//!
//! - Tensor rank.
//! - Tensor dimensions.
//! - Tensor element count.
//! - Number of sparse tensors.
//! - Number of nonzero values.
//! - Number of sparse axes.
//! - Number of sparse blocks.
//! - Number of sparse transformations.
//! - Number of model operations.
//! - Number of execution targets.
//!
//! Integer widths in metadata describe representable values, not guaranteed
//! allocation or execution capacity.
//!
//! Actual resource feasibility belongs to downstream compiler and runtime
//! infrastructure.
//!
//! ## POCO-REAF
//!
//! Sparse intent is independent of physical storage and execution targets.
//!
//! A target may realize a sparse representation differently, or may not
//! support it. Unsupported realization must be reported explicitly.
//!
//! ## Safety
//!
//! Safe Rust only.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::ai::error::{AiError, AiErrorKind, AiResult};
use crate::ai::requirements::RequirementKey;
use crate::ai::optimization::transformations::{
    TransformationId,
    TransformationVersion,
};

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn sparsity_error(message: impl Into<String>) -> AiError {
    AiError::new(AiErrorKind::InvalidInput, message)
}

fn evidence_error(message: impl Into<String>) -> AiError {
    AiError::new(AiErrorKind::RequirementUnsatisfied, message)
}

fn validate_reference(value: &str, description: &str) -> AiResult<()> {
    if value.trim().is_empty() {
        return Err(sparsity_error(format!(
            "{description} must not be empty."
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(sparsity_error(format!(
            "{description} contains control characters."
        )));
    }

    Ok(())
}

// ============================================================================
// SPARSE ENCODING
// ============================================================================

/// A target-independent sparse encoding description.
///
/// These variants describe logical encoding families. They do not promise
/// that a particular device, backend, or runtime supports the encoding.
///
/// Custom formats use open-world identities rather than a closed vendor list.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum SparseEncoding {
    /// Coordinate-based sparse representation.
    ///
    /// Coordinates identify explicitly represented elements. Duplicate
    /// coordinate behavior must be established by the operation contract.
    Coordinate,

    /// Compressed sparse representation along a declared axis.
    ///
    /// The axis is a logical axis index interpreted by the canonical tensor
    /// shape authority.
    Compressed {
        /// Compressed logical axis.
        axis: usize,
    },

    /// Block-compressed sparse representation.
    ///
    /// The block shape is a logical property, not a physical tile size.
    BlockCompressed {
        /// Compressed logical axis.
        axis: usize,

        /// Logical block dimensions.
        ///
        /// The canonical shape system determines compatibility with the
        /// tensor being transformed.
        block_shape: Vec<u128>,
    },

    /// Diagonal sparse representation.
    ///
    /// Offsets are relative to the main diagonal. Positive and negative
    /// offsets are both meaningful.
    Diagonal {
        /// Declared diagonal offsets.
        offsets: BTreeSet<i128>,
    },

    /// Banded sparse representation.
    ///
    /// Bandwidths describe the logical distance from the main diagonal.
    Banded {
        /// Lower bandwidth.
        lower: u128,

        /// Upper bandwidth.
        upper: u128,
    },

    /// Extension-defined sparse encoding.
    ///
    /// The extension owns its format semantics and validation.
    Custom {
        /// Stable extension format identity.
        format: RequirementKey,
    },
}

impl SparseEncoding {
    /// Validates the local structural invariants of an encoding.
    ///
    /// This does not validate compatibility with a particular tensor shape.
    pub fn validate(&self) -> AiResult<()> {
        match self {
            Self::Coordinate => Ok(()),

            Self::Compressed { .. } => Ok(()),

            Self::BlockCompressed { block_shape, .. } => {
                if block_shape.is_empty() {
                    return Err(sparsity_error(
                        "A block-compressed encoding requires a nonempty block shape.",
                    ));
                }

                if block_shape.iter().any(|dimension| *dimension == 0) {
                    return Err(sparsity_error(
                        "Sparse block dimensions must be nonzero.",
                    ));
                }

                Ok(())
            }

            Self::Diagonal { offsets } => {
                if offsets.is_empty() {
                    return Err(sparsity_error(
                        "A diagonal encoding requires at least one diagonal offset.",
                    ));
                }

                Ok(())
            }

            Self::Banded { .. } => Ok(()),

            Self::Custom { .. } => Ok(()),
        }
    }

    /// Returns whether the encoding requires extension-specific validation.
    #[must_use]
    pub const fn requires_extension_validation(&self) -> bool {
        matches!(self, Self::Custom { .. })
    }

    /// Returns whether the encoding describes block structure.
    #[must_use]
    pub const fn is_block_structured(&self) -> bool {
        matches!(self, Self::BlockCompressed { .. })
    }
}

// ============================================================================
// SPARSITY STRUCTURE
// ============================================================================

/// The structural organization of nonzero values.
///
/// Structure is distinct from the encoding used to store or communicate it.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum SparsityStructure {
    /// Nonzero locations have no additional declared structure.
    Unstructured,

    /// Nonzero locations follow a block structure.
    BlockStructured {
        /// Logical block dimensions.
        block_shape: Vec<u128>,
    },

    /// Nonzero values occur on declared diagonals.
    Diagonal {
        /// Diagonal offsets.
        offsets: BTreeSet<i128>,
    },

    /// Nonzero values are restricted to a declared band.
    Banded {
        /// Lower bandwidth.
        lower: u128,

        /// Upper bandwidth.
        upper: u128,
    },

    /// Structure defined by an extension.
    Custom {
        /// Stable extension identity.
        structure: RequirementKey,
    },

    /// The structure may change during execution.
    ///
    /// Dynamic structure requires an explicit provider contract. It must not
    /// be treated as a statically proven sparsity pattern.
    Dynamic {
        /// Provider responsible for describing dynamic structure.
        provider: RequirementKey,
    },
}

impl SparsityStructure {
    /// Validates local structural invariants.
    pub fn validate(&self) -> AiResult<()> {
        match self {
            Self::Unstructured => Ok(()),

            Self::BlockStructured { block_shape } => {
                if block_shape.is_empty()
                    || block_shape.iter().any(|dimension| *dimension == 0)
                {
                    return Err(sparsity_error(
                        "Block sparsity requires nonzero block dimensions.",
                    ));
                }

                Ok(())
            }

            Self::Diagonal { offsets } => {
                if offsets.is_empty() {
                    return Err(sparsity_error(
                        "Diagonal sparsity requires at least one offset.",
                    ));
                }

                Ok(())
            }

            Self::Banded { .. } => Ok(()),

            Self::Custom { .. } | Self::Dynamic { .. } => Ok(()),
        }
    }

    /// Returns whether the structure requires provider validation.
    #[must_use]
    pub const fn requires_provider_validation(&self) -> bool {
        matches!(
            self,
            Self::Custom { .. } | Self::Dynamic { .. }
        )
    }
}

// ============================================================================
// IMPLICIT VALUE SEMANTICS
// ============================================================================

/// Meaning of values omitted from a sparse representation.
///
/// An omitted value is not automatically interchangeable with numerical zero.
/// The operation semantics must establish that the chosen implicit value
/// preserves the computation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum ImplicitValue {
    /// Omitted entries represent exact numerical zero.
    ExactZero,

    /// Omitted entries represent an operation-defined additive identity.
    ///
    /// The identity is resolved by the semantic operation contract.
    AdditiveIdentity {
        /// Identity of the additive-identity contract.
        contract: RequirementKey,
    },

    /// Omitted entries have extension-defined meaning.
    ProviderDefined {
        /// Provider responsible for the implicit-value semantics.
        provider: RequirementKey,
    },
}

impl ImplicitValue {
    /// Returns whether the implicit value has a provider-defined meaning.
    #[must_use]
    pub const fn requires_provider_validation(&self) -> bool {
        matches!(self, Self::ProviderDefined { .. })
    }
}

// ============================================================================
// DENSITY METADATA
// ============================================================================

/// An exact rational density value.
///
/// Density is the ratio of explicitly represented nonzero elements to
/// logical tensor elements.
///
/// This is metadata, not an allocation request or a promise that the
/// representation is beneficial.
///
/// The fraction is kept exact; floating-point rounding is avoided.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SparsityDensity {
    numerator: u128,
    denominator: u128,
}

impl SparsityDensity {
    /// Creates a density fraction.
    ///
    /// The denominator must be nonzero and the numerator must not exceed it.
    pub fn new(numerator: u128, denominator: u128) -> AiResult<Self> {
        if denominator == 0 {
            return Err(sparsity_error(
                "Density denominator must be nonzero.",
            ));
        }

        if numerator > denominator {
            return Err(sparsity_error(
                "Density cannot exceed one.",
            ));
        }

        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// Returns the numerator.
    #[must_use]
    pub const fn numerator(self) -> u128 {
        self.numerator
    }

    /// Returns the denominator.
    #[must_use]
    pub const fn denominator(self) -> u128 {
        self.denominator
    }

    /// Returns whether the density is exactly zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.numerator == 0
    }

    /// Returns whether the density is exactly one.
    #[must_use]
    pub const fn is_dense(self) -> bool {
        self.numerator == self.denominator
    }
}

// ============================================================================
// SPARSITY CONTRACT
// ============================================================================

/// A target-independent sparse representation contract.
///
/// It combines encoding, structure, implicit-value semantics, and optional
/// density metadata.
///
/// It does not contain tensor storage, numerical values, or physical indices.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SparsityContract {
    encoding: SparseEncoding,
    structure: SparsityStructure,
    implicit_value: ImplicitValue,
    density: Option<SparsityDensity>,
    zero_elision_contract: RequirementKey,
}

impl SparsityContract {
    /// Creates a validated sparse representation contract.
    ///
    /// `zero_elision_contract` identifies the semantic rule under which
    /// omitted values may be treated as the declared implicit value.
    ///
    /// The existence of this identity does not itself prove that zero
    /// elision is legal. Validation evidence is required separately.
    pub fn new(
        encoding: SparseEncoding,
        structure: SparsityStructure,
        implicit_value: ImplicitValue,
        density: Option<SparsityDensity>,
        zero_elision_contract: RequirementKey,
    ) -> AiResult<Self> {
        encoding.validate()?;
        structure.validate()?;

        if let (
            SparseEncoding::BlockCompressed {
                block_shape: encoding_shape,
                ..
            },
            SparsityStructure::BlockStructured {
                block_shape: structure_shape,
            },
        ) = (&encoding, &structure)
        {
            if encoding_shape != structure_shape {
                return Err(sparsity_error(
                    "Encoding block dimensions disagree with the declared sparsity structure.",
                ));
            }
        }

        Ok(Self {
            encoding,
            structure,
            implicit_value,
            density,
            zero_elision_contract,
        })
    }

    /// Returns the sparse encoding.
    #[must_use]
    pub fn encoding(&self) -> &SparseEncoding {
        &self.encoding
    }

    /// Returns the declared sparsity structure.
    #[must_use]
    pub fn structure(&self) -> &SparsityStructure {
        &self.structure
    }

    /// Returns the implicit-value semantics.
    #[must_use]
    pub fn implicit_value(&self) -> &ImplicitValue {
        &self.implicit_value
    }

    /// Returns optional density metadata.
    #[must_use]
    pub const fn density(&self) -> Option<SparsityDensity> {
        self.density
    }

    /// Returns the zero-elision semantic contract identity.
    #[must_use]
    pub fn zero_elision_contract(&self) -> &RequirementKey {
        &self.zero_elision_contract
    }

    /// Returns whether this contract requires provider validation.
    #[must_use]
    pub const fn requires_provider_validation(&self) -> bool {
        self.encoding.requires_extension_validation()
            || self.structure.requires_provider_validation()
            || self.implicit_value.requires_provider_validation()
    }
}

// ============================================================================
// SEMANTIC EVIDENCE
// ============================================================================

/// Status of one sparse semantic preservation obligation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum SparsityProofStatus {
    /// The obligation has been established by an independent validator.
    Proven,

    /// The obligation has been demonstrated to be violated.
    Disproven,

    /// The obligation has not been established.
    Unknown,

    /// The obligation does not apply to this transformation.
    ///
    /// This status must be justified by the validator that produced the
    /// evidence; it must not be used to bypass a relevant obligation.
    NotApplicable,
}

/// Evidence for one sparse transformation obligation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SparsityProof {
    status: SparsityProofStatus,
    reference: String,
}

impl SparsityProof {
    /// Creates a sparse semantic proof record.
    ///
    /// The reference must identify the independently retained validation
    /// report, proof, test result, or contract evaluation.
    pub fn new(
        status: SparsityProofStatus,
        reference: impl Into<String>,
    ) -> AiResult<Self> {
        let reference = reference.into();
        validate_reference(&reference, "Sparsity evidence reference")?;

        Ok(Self {
            status,
            reference,
        })
    }

    /// Returns the evidence status.
    #[must_use]
    pub const fn status(&self) -> SparsityProofStatus {
        self.status
    }

    /// Returns the evidence reference.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }
}

// ============================================================================
// VALIDATION EVIDENCE
// ============================================================================

/// Independent evidence for sparse transformation legality.
///
/// The proof obligations are intentionally separate. For example, proving
/// that zero elision is valid does not prove that differentiation or
/// reproducibility requirements are preserved.
///
/// A proof record is a reference to externally established evidence. This
/// structure does not authenticate the report or independently prove its
/// mathematical conclusions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SparsityValidationEvidence {
    zero_elision: SparsityProof,
    operation_semantics: SparsityProof,
    shape_compatibility: SparsityProof,
    duplicate_coordinates: SparsityProof,
    index_ordering: SparsityProof,
    differentiation: SparsityProof,
    precision_interaction: SparsityProof,
    quantization_interaction: SparsityProof,
    effects: SparsityProof,
    provenance: SparsityProof,
    reproducibility: SparsityProof,
    additional_references: BTreeSet<String>,
}

impl SparsityValidationEvidence {
    /// Creates a complete sparse validation evidence record.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        zero_elision: SparsityProof,
        operation_semantics: SparsityProof,
        shape_compatibility: SparsityProof,
        duplicate_coordinates: SparsityProof,
        index_ordering: SparsityProof,
        differentiation: SparsityProof,
        precision_interaction: SparsityProof,
        quantization_interaction: SparsityProof,
        effects: SparsityProof,
        provenance: SparsityProof,
        reproducibility: SparsityProof,
        additional_references: impl IntoIterator<Item = String>,
    ) -> AiResult<Self> {
        let mut references = BTreeSet::new();

        for reference in additional_references {
            validate_reference(
                &reference,
                "Additional sparsity evidence reference",
            )?;

            references.insert(reference);
        }

        for proof in [
            &zero_elision,
            &operation_semantics,
            &shape_compatibility,
            &duplicate_coordinates,
            &index_ordering,
            &differentiation,
            &precision_interaction,
            &quantization_interaction,
            &effects,
            &provenance,
            &reproducibility,
        ] {
            references.insert(proof.reference.clone());
        }

        Ok(Self {
            zero_elision,
            operation_semantics,
            shape_compatibility,
            duplicate_coordinates,
            index_ordering,
            differentiation,
            precision_interaction,
            quantization_interaction,
            effects,
            provenance,
            reproducibility,
            additional_references: references,
        })
    }

    /// Returns the zero-elision evidence.
    #[must_use]
    pub fn zero_elision(&self) -> &SparsityProof {
        &self.zero_elision
    }

    /// Returns the operation-semantics evidence.
    #[must_use]
    pub fn operation_semantics(&self) -> &SparsityProof {
        &self.operation_semantics
    }

    /// Returns the shape-compatibility evidence.
    #[must_use]
    pub fn shape_compatibility(&self) -> &SparsityProof {
        &self.shape_compatibility
    }

    /// Returns the duplicate-coordinate evidence.
    #[must_use]
    pub fn duplicate_coordinates(&self) -> &SparsityProof {
        &self.duplicate_coordinates
    }

    /// Returns the index-ordering evidence.
    #[must_use]
    pub fn index_ordering(&self) -> &SparsityProof {
        &self.index_ordering
    }

    /// Returns the differentiation evidence.
    #[must_use]
    pub fn differentiation(&self) -> &SparsityProof {
        &self.differentiation
    }

    /// Returns the precision-interaction evidence.
    #[must_use]
    pub fn precision_interaction(&self) -> &SparsityProof {
        &self.precision_interaction
    }

    /// Returns the quantization-interaction evidence.
    #[must_use]
    pub fn quantization_interaction(&self) -> &SparsityProof {
        &self.quantization_interaction
    }

    /// Returns the effect-preservation evidence.
    #[must_use]
    pub fn effects(&self) -> &SparsityProof {
        &self.effects
    }

    /// Returns the provenance-preservation evidence.
    #[must_use]
    pub fn provenance(&self) -> &SparsityProof {
        &self.provenance
    }

    /// Returns the reproducibility evidence.
    #[must_use]
    pub fn reproducibility(&self) -> &SparsityProof {
        &self.reproducibility
    }

    /// Returns all evidence references in deterministic order.
    #[must_use]
    pub fn references(&self) -> &BTreeSet<String> {
        &self.additional_references
    }

    fn obligations(&self) -> [(&'static str, &SparsityProof); 11] {
        [
            ("zero elision", &self.zero_elision),
            ("operation semantics", &self.operation_semantics),
            ("shape compatibility", &self.shape_compatibility),
            ("duplicate-coordinate behavior", &self.duplicate_coordinates),
            ("index ordering", &self.index_ordering),
            ("differentiation", &self.differentiation),
            ("precision interaction", &self.precision_interaction),
            ("quantization interaction", &self.quantization_interaction),
            ("effects", &self.effects),
            ("provenance", &self.provenance),
            ("reproducibility", &self.reproducibility),
        ]
    }
}

// ============================================================================
// SPARSE TRANSFORMATION
// ============================================================================

/// A target-independent sparse representation transformation.
///
/// This is a semantic proposal, not a graph edit or execution request.
///
/// General graph mutation must be represented through
/// `optimization::transformations`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SparsityTransformation {
    identity: TransformationId,
    version: TransformationVersion,
    source: SparsityContract,
    destination: SparsityContract,
    operation_contract: RequirementKey,
    precision_contract: Option<RequirementKey>,
    quantization_contract: Option<RequirementKey>,
    evidence: SparsityValidationEvidence,
}

impl SparsityTransformation {
    /// Creates a sparse transformation proposal.
    ///
    /// Precision and quantization identities are references to the existing
    /// authorities. They do not create replacement precision or
    /// quantization contracts.
    pub fn new(
        identity: TransformationId,
        version: TransformationVersion,
        source: SparsityContract,
        destination: SparsityContract,
        operation_contract: RequirementKey,
        precision_contract: Option<RequirementKey>,
        quantization_contract: Option<RequirementKey>,
        evidence: SparsityValidationEvidence,
    ) -> AiResult<Self> {
        source.encoding.validate()?;
        source.structure.validate()?;
        destination.encoding.validate()?;
        destination.structure.validate()?;

        Ok(Self {
            identity,
            version,
            source,
            destination,
            operation_contract,
            precision_contract,
            quantization_contract,
            evidence,
        })
    }

    /// Returns the stable transformation identity.
    #[must_use]
    pub fn identity(&self) -> &TransformationId {
        &self.identity
    }

    /// Returns the transformation version.
    #[must_use]
    pub const fn version(&self) -> TransformationVersion {
        self.version
    }

    /// Returns the source sparse contract.
    #[must_use]
    pub fn source(&self) -> &SparsityContract {
        &self.source
    }

    /// Returns the destination sparse contract.
    #[must_use]
    pub fn destination(&self) -> &SparsityContract {
        &self.destination
    }

    /// Returns the operation semantics identity.
    #[must_use]
    pub fn operation_contract(&self) -> &RequirementKey {
        &self.operation_contract
    }

    /// Returns the referenced precision contract, if supplied.
    #[must_use]
    pub fn precision_contract(&self) -> Option<&RequirementKey> {
        self.precision_contract.as_ref()
    }

    /// Returns the referenced quantization contract, if supplied.
    #[must_use]
    pub fn quantization_contract(&self) -> Option<&RequirementKey> {
        self.quantization_contract.as_ref()
    }

    /// Returns the independent validation evidence.
    #[must_use]
    pub fn evidence(&self) -> &SparsityValidationEvidence {
        &self.evidence
    }
}

// ============================================================================
// LEGALITY RESULT
// ============================================================================

/// Legality classification for a sparse transformation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SparsityLegality {
    /// All required obligations have been established.
    Legal {
        /// Evidence references supporting the decision.
        evidence_references: BTreeSet<String>,
    },

    /// At least one obligation has been demonstrated to be violated.
    Illegal {
        /// Reasons for rejection.
        reasons: Vec<String>,
    },

    /// One or more required obligations remain unestablished.
    Indeterminate {
        /// Unresolved obligations.
        unresolved: Vec<String>,
    },
}

impl SparsityLegality {
    /// Returns whether the transformation is certified legal.
    #[must_use]
    pub const fn is_legal(&self) -> bool {
        matches!(self, Self::Legal { .. })
    }

    /// Returns whether the transformation is definitely illegal.
    #[must_use]
    pub const fn is_illegal(&self) -> bool {
        matches!(self, Self::Illegal { .. })
    }

    /// Returns whether legality remains unresolved.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        matches!(self, Self::Indeterminate { .. })
    }
}

// ============================================================================
// VALIDATOR
// ============================================================================

/// Validates sparse transformation declarations and evidence status.
///
/// This validator does not execute sparse operations, inspect tensor data,
/// authenticate evidence providers, or replace the canonical semantic
/// legality service.
///
/// A `Legal` result means that every supplied required obligation is marked
/// `Proven` or explicitly `NotApplicable` by the upstream evidence producer.
/// The caller remains responsible for validating the evidence source and
/// confirming that all applicable obligations were supplied correctly.
#[derive(Clone, Copy, Debug, Default)]
pub struct SparsityValidator;

impl SparsityValidator {
    /// Creates a sparsity validator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Validates a sparse transformation.
    pub fn validate(
        &self,
        transformation: &SparsityTransformation,
    ) -> AiResult<SparsityLegality> {
        transformation.source.encoding.validate()?;
        transformation.source.structure.validate()?;
        transformation.destination.encoding.validate()?;
        transformation.destination.structure.validate()?;

        let mut rejected = Vec::new();
        let mut unresolved = Vec::new();

        for (name, proof) in transformation.evidence.obligations() {
            match proof.status {
                SparsityProofStatus::Proven
                | SparsityProofStatus::NotApplicable => {}

                SparsityProofStatus::Disproven => {
                    rejected.push(format!(
                        "The {name} obligation was disproven."
                    ));
                }

                SparsityProofStatus::Unknown => {
                    unresolved.push(format!(
                        "The {name} obligation has not been established."
                    ));
                }
            }
        }

        if transformation.source.requires_provider_validation()
            || transformation.destination.requires_provider_validation()
        {
            let provider_proofs = [
                transformation.evidence.operation_semantics.status,
                transformation.evidence.zero_elision.status,
            ];

            if provider_proofs.iter().any(|status| {
                *status != SparsityProofStatus::Proven
            }) {
                unresolved.push(
                    "Provider-defined sparse semantics require complete provider validation."
                        .to_string(),
                );
            }
        }

        if !rejected.is_empty() {
            return Ok(SparsityLegality::Illegal {
                reasons: rejected,
            });
        }

        if !unresolved.is_empty() {
            return Ok(SparsityLegality::Indeterminate {
                unresolved,
            });
        }

        Ok(SparsityLegality::Legal {
            evidence_references: transformation
                .evidence
                .additional_references
                .clone(),
        })
    }

    /// Requires a complete legal result.
    ///
    /// This method converts illegal or indeterminate outcomes into an
    /// explicit AI-domain error. It never treats unknown evidence as success.
    pub fn require_legal(
        &self,
        transformation: &SparsityTransformation,
    ) -> AiResult<SparsityLegality> {
        let result = self.validate(transformation)?;

        match &result {
            SparsityLegality::Legal { .. } => Ok(result),

            SparsityLegality::Illegal { reasons } => {
                Err(evidence_error(format!(
                    "Sparse transformation is illegal: {}",
                    reasons.join("; ")
                )))
            }

            SparsityLegality::Indeterminate { unresolved } => {
                Err(evidence_error(format!(
                    "Sparse transformation legality is indeterminate: {}",
                    unresolved.join("; ")
                )))
            }
        }
    }
}

// ============================================================================
// EXTENSION CONTRACT
// ============================================================================

/// Describes an extension-defined sparse encoding implementation.
///
/// This is a registration descriptor only. It does not register the provider
/// in a global registry or claim that a target can execute the format.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SparseEncodingExtension {
    identity: RequirementKey,
    version: TransformationVersion,
    semantic_contract: RequirementKey,
    validation_contract: RequirementKey,
}

impl SparseEncodingExtension {
    /// Creates an extension descriptor.
    pub fn new(
        identity: RequirementKey,
        version: TransformationVersion,
        semantic_contract: RequirementKey,
        validation_contract: RequirementKey,
    ) -> Self {
        Self {
            identity,
            version,
            semantic_contract,
            validation_contract,
        }
    }

    /// Returns the extension identity.
    #[must_use]
    pub fn identity(&self) -> &RequirementKey {
        &self.identity
    }

    /// Returns the extension version.
    #[must_use]
    pub const fn version(&self) -> TransformationVersion {
        self.version
    }

    /// Returns the semantic contract identity.
    #[must_use]
    pub fn semantic_contract(&self) -> &RequirementKey {
        &self.semantic_contract
    }

    /// Returns the validation contract identity.
    #[must_use]
    pub fn validation_contract(&self) -> &RequirementKey {
        &self.validation_contract
    }
}

// ============================================================================
// DISPLAY
// ============================================================================

impl fmt::Display for SparsityDensity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}/{}",
            self.numerator,
            self.denominator
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn key(namespace: &str, name: &str) -> RequirementKey {
        RequirementKey::new(namespace, name)
            .expect("test identity must be valid")
    }

    fn proof(
        status: SparsityProofStatus,
        name: &str,
    ) -> SparsityProof {
        SparsityProof::new(status, name)
            .expect("test evidence reference must be valid")
    }

    fn contract(
        encoding: SparseEncoding,
        structure: SparsityStructure,
    ) -> SparsityContract {
        SparsityContract::new(
            encoding,
            structure,
            ImplicitValue::ExactZero,
            None,
            key("ai.sparsity", "zero-elision"),
        )
        .expect("test sparse contract must be valid")
    }

    fn evidence(
        status: SparsityProofStatus,
    ) -> SparsityValidationEvidence {
        SparsityValidationEvidence::new(
            proof(status, "zero-elision"),
            proof(status, "operation-semantics"),
            proof(status, "shape"),
            proof(SparsityProofStatus::NotApplicable, "duplicates"),
            proof(SparsityProofStatus::NotApplicable, "ordering"),
            proof(status, "differentiation"),
            proof(status, "precision"),
            proof(status, "quantization"),
            proof(status, "effects"),
            proof(status, "provenance"),
            proof(status, "reproducibility"),
            Vec::<String>::new(),
        )
        .expect("test evidence must be valid")
    }

    fn transformation(
        status: SparsityProofStatus,
    ) -> SparsityTransformation {
        let source = contract(
            SparseEncoding::Coordinate,
            SparsityStructure::Unstructured,
        );

        let destination = contract(
            SparseEncoding::Compressed { axis: 0 },
            SparsityStructure::Unstructured,
        );

        SparsityTransformation::new(
            TransformationId::new("ai.sparsity.convert")
                .expect("valid transformation identity"),
            TransformationVersion::new(1, 0, 0),
            source,
            destination,
            key("ai.operation", "matrix-multiply"),
            Some(key("ai.precision", "required-format")),
            Some(key("ai.quantization", "quantization-contract")),
            evidence(status),
        )
        .expect("test transformation must be valid")
    }

    #[test]
    fn density_accepts_exact_zero_and_one() {
        assert!(SparsityDensity::new(0, 1)
            .expect("zero density")
            .is_zero());

        assert!(SparsityDensity::new(1, 1)
            .expect("dense density")
            .is_dense());
    }

    #[test]
    fn density_rejects_zero_denominator() {
        assert!(SparsityDensity::new(0, 0).is_err());
    }

    #[test]
    fn density_rejects_values_above_one() {
        assert!(SparsityDensity::new(2, 1).is_err());
    }

    #[test]
    fn block_encoding_rejects_empty_shape() {
        let encoding = SparseEncoding::BlockCompressed {
            axis: 0,
            block_shape: Vec::new(),
        };

        assert!(encoding.validate().is_err());
    }

    #[test]
    fn block_encoding_rejects_zero_dimension() {
        let encoding = SparseEncoding::BlockCompressed {
            axis: 0,
            block_shape: vec![4, 0],
        };

        assert!(encoding.validate().is_err());
    }

    #[test]
    fn block_structure_must_match_encoding() {
        let result = SparsityContract::new(
            SparseEncoding::BlockCompressed {
                axis: 0,
                block_shape: vec![4, 4],
            },
            SparsityStructure::BlockStructured {
                block_shape: vec![8, 8],
            },
            ImplicitValue::ExactZero,
            None,
            key("ai.sparsity", "zero"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn diagonal_encoding_requires_offsets() {
        let encoding = SparseEncoding::Diagonal {
            offsets: BTreeSet::new(),
        };

        assert!(encoding.validate().is_err());
    }

    #[test]
    fn proven_evidence_is_legal() {
        let transformation = transformation(SparsityProofStatus::Proven);

        let result = SparsityValidator::new()
            .validate(&transformation)
            .expect("validation should complete");

        assert!(result.is_legal());
    }

    #[test]
    fn unknown_evidence_is_indeterminate() {
        let transformation = transformation(SparsityProofStatus::Unknown);

        let result = SparsityValidator::new()
            .validate(&transformation)
            .expect("validation should complete");

        assert!(result.is_indeterminate());
    }

    #[test]
    fn disproven_evidence_is_illegal() {
        let transformation =
            transformation(SparsityProofStatus::Disproven);

        let result = SparsityValidator::new()
            .validate(&transformation)
            .expect("validation should complete");

        assert!(result.is_illegal());
    }

    #[test]
    fn unknown_evidence_cannot_be_accepted() {
        let transformation = transformation(SparsityProofStatus::Unknown);

        assert!(
            SparsityValidator::new()
                .require_legal(&transformation)
                .is_err()
        );
    }

    #[test]
    fn provider_defined_encoding_requires_validation() {
        let encoding = SparseEncoding::Custom {
            format: key("extension", "sparse-format"),
        };

        assert!(encoding.requires_extension_validation());
    }

    #[test]
    fn dynamic_structure_requires_provider_validation() {
        let structure = SparsityStructure::Dynamic {
            provider: key("extension", "dynamic-pattern"),
        };

        assert!(structure.requires_provider_validation());
    }

    #[test]
    fn exact_density_display_is_deterministic() {
        let density = SparsityDensity::new(1, 3)
            .expect("valid density");

        assert_eq!(density.to_string(), "1/3");
    }
}
