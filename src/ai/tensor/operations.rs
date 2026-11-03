
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Tensor Operations
//!
//! Path: src/ai/tensor/operations.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define target-independent contracts for AI tensor operations.
//!
//! This module describes what a tensor operation means and validates
//! its structural, type, and shape contracts. It does not execute
//! numerical kernels, allocate storage, select hardware, or create IR.
//!
//! ## Authority
//!
//! Canonical AI semantic dimensions:
//!     crate::frontend::semantic::domains::ai
//!
//! AI tensor shape analysis:
//!     crate::ai::tensor::shapes
//!
//! AI tensor data types:
//!     crate::ai::tensor::dtypes
//!
//! Canonical tensor storage:
//!     crate::classical::collections::tensor
//!
//! Universal tensor IR:
//!     crate::zuir::dialects::tensor
//!
//! ## Scalability
//!
//! No universal maximum tensor rank, dimension count, model size,
//! operation count, or hardware capacity is imposed.
//!
//! Vec-backed collections describe dynamically sized contracts.
//! Concrete execution remains subject to available resources,
//! representation limits, and target capabilities.
//!
//! ## Ownership
//!
//! This module owns:
//! - Tensor operation identities.
//! - Operation signatures.
//! - Tensor operand/result descriptions.
//! - Operation attributes.
//! - Structural operation validation.
//! - Shape-contract validation.
//! - Extensible operation descriptors.
//!
//! It does not own:
//! - Canonical semantic types.
//! - Tensor storage.
//! - Numerical kernels.
//! - Compiler IR.
//! - Hardware selection.
//! - Device allocation.
//! - Scheduling.
//! - Runtime execution.
//!
//! ## Integration
//!
//! `tensor/mod.rs` exports this module.
//!
//! `tensor/shapes.rs` supplies shape analysis.
//!
//! `tensor/dtypes.rs` supplies numerical type descriptors.
//!
//! The frontend semantic adapter converts canonical AST operations
//! into these operation contracts.
//!
//! ZUIR tensor lowering converts validated contracts into canonical
//! tensor-dialect operations.
//!
//! No competing tensor IR is introduced.

use std::collections::BTreeMap;
use std::fmt;

use crate::ai::tensor::dtypes::AiTensorDType;
use crate::ai::tensor::shapes::{
    DimensionExpr,
    ShapeAnalysis,
    ShapeConstraint,
    TensorShape,
};

// ============================================================================
// OPERATION IDENTITIES
// ============================================================================

/// A stable, extensible identity for a tensor operation.
///
/// The identity is independent of a particular backend, device, or
/// numerical implementation.
///
/// Names are opaque. Their meaning is established by the operation
/// registry and the language's semantic specifications.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TensorOperationId {
    namespace: String,
    name: String,
    version: Option<String>,
}

impl TensorOperationId {
    /// Creates an operation identity.
    ///
    /// Empty identifiers and control characters are rejected.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
        version: Option<String>,
    ) -> Result<Self, TensorOperationError> {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.trim().is_empty()
            || name.trim().is_empty()
            || namespace.chars().any(char::is_control)
            || name.chars().any(char::is_control)
            || version
                .as_ref()
                .is_some_and(|v| v.is_empty() || v.chars().any(char::is_control))
        {
            return Err(TensorOperationError::InvalidOperationIdentity);
        }

        Ok(Self {
            namespace,
            name,
            version,
        })
    }

    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
}

impl fmt::Display for TensorOperationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.namespace, self.name)?;

        if let Some(version) = &self.version {
            write!(f, "@{version}")?;
        }

        Ok(())
    }
}

// ============================================================================
// VALUE IDENTITIES
// ============================================================================

/// An opaque identity for a tensor value.
///
/// This is an operation-contract reference, not a ZUIR value or a
/// runtime allocation handle.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TensorValueId(String);

impl TensorValueId {
    pub fn new(value: impl Into<String>) -> Result<Self, TensorOperationError> {
        let value = value.into();

        if value.is_empty() || value.chars().any(char::is_control) {
            return Err(TensorOperationError::InvalidValueIdentity);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// TENSOR VALUE CONTRACTS
// ============================================================================

/// A tensor value's target-independent contract.
///
/// Shape and dtype are descriptions. They do not imply that the tensor
/// has been materialized or that a particular target can execute it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorValueContract {
    id: TensorValueId,
    shape: TensorShape,
    dtype: AiTensorDType,
}

impl TensorValueContract {
    pub fn new(
        id: TensorValueId,
        shape: TensorShape,
        dtype: AiTensorDType,
    ) -> Self {
        Self { id, shape, dtype }
    }

    #[must_use]
    pub fn id(&self) -> &TensorValueId {
        &self.id
    }

    #[must_use]
    pub fn shape(&self) -> &TensorShape {
        &self.shape
    }

    #[must_use]
    pub fn dtype(&self) -> &AiTensorDType {
        &self.dtype
    }
}

// ============================================================================
// OPERATION ATTRIBUTES
// ============================================================================

/// A portable, typed attribute value.
///
/// This is intentionally independent of any IR attribute representation.
/// An adapter converts these values into the canonical IR attributes.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TensorAttributeValue {
    Boolean(bool),
    Integer(i128),
    Unsigned(u128),
    Text(String),
    Integers(Vec<i128>),
    UnsignedIntegers(Vec<u128>),
    Texts(Vec<String>),
    SymbolicDimensions(Vec<DimensionExpr>),
}

/// A named operation attribute.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TensorAttribute {
    name: String,
    value: TensorAttributeValue,
}

impl TensorAttribute {
    pub fn new(
        name: impl Into<String>,
        value: TensorAttributeValue,
    ) -> Result<Self, TensorOperationError> {
        let name = name.into();

        if name.is_empty() || name.chars().any(char::is_control) {
            return Err(TensorOperationError::InvalidAttributeName);
        }

        Ok(Self { name, value })
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn value(&self) -> &TensorAttributeValue {
        &self.value
    }
}

// ============================================================================
// OPERATION EFFECTS AND REQUIREMENTS
// ============================================================================

/// Describes an operation's semantic effect.
///
/// These descriptions do not authorize effects. The canonical semantic
/// effect system remains authoritative.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[non_exhaustive]
pub enum TensorOperationEffect {
    Pure,
    ReadsState,
    WritesState,
    Random,
    Communicates,
    Custom(String),
}

/// A target-independent capability requirement.
///
/// Capability evidence and feasibility are determined downstream.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TensorCapabilityRequirement {
    namespace: String,
    name: String,
}

impl TensorCapabilityRequirement {
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, TensorOperationError> {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.is_empty()
            || name.is_empty()
            || namespace.chars().any(char::is_control)
            || name.chars().any(char::is_control)
        {
            return Err(TensorOperationError::InvalidCapabilityRequirement);
        }

        Ok(Self { namespace, name })
    }

    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

// ============================================================================
// OPERATION CONTRACT
// ============================================================================

/// A validated-or-pending tensor operation description.
///
/// The operation is a semantic description. It is not an executable
/// kernel and does not own runtime tensor storage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorOperation {
    id: TensorOperationId,
    inputs: Vec<TensorValueContract>,
    outputs: Vec<TensorValueContract>,
    attributes: BTreeMap<String, TensorAttributeValue>,
    effects: Vec<TensorOperationEffect>,
    requirements: Vec<TensorCapabilityRequirement>,
}

impl TensorOperation {
    pub fn new(
        id: TensorOperationId,
        inputs: Vec<TensorValueContract>,
        outputs: Vec<TensorValueContract>,
    ) -> Result<Self, TensorOperationError> {
        if outputs.is_empty() {
            return Err(TensorOperationError::MissingOperationOutput);
        }

        Ok(Self {
            id,
            inputs,
            outputs,
            attributes: BTreeMap::new(),
            effects: vec![TensorOperationEffect::Pure],
            requirements: Vec::new(),
        })
    }

    pub fn with_attribute(
        mut self,
        attribute: TensorAttribute,
    ) -> Result<Self, TensorOperationError> {
        if self
            .attributes
            .insert(attribute.name.clone(), attribute.value)
            .is_some()
        {
            return Err(TensorOperationError::DuplicateAttribute);
        }

        Ok(self)
    }

    pub fn with_effect(mut self, effect: TensorOperationEffect) -> Self {
        self.effects.push(effect);
        self
    }

    pub fn with_requirement(
        mut self,
        requirement: TensorCapabilityRequirement,
    ) -> Self {
        self.requirements.push(requirement);
        self
    }

    #[must_use]
    pub fn id(&self) -> &TensorOperationId {
        &self.id
    }

    #[must_use]
    pub fn inputs(&self) -> &[TensorValueContract] {
        &self.inputs
    }

    #[must_use]
    pub fn outputs(&self) -> &[TensorValueContract] {
        &self.outputs
    }

    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, TensorAttributeValue> {
        &self.attributes
    }

    #[must_use]
    pub fn effects(&self) -> &[TensorOperationEffect] {
        &self.effects
    }

    #[must_use]
    pub fn requirements(&self) -> &[TensorCapabilityRequirement] {
        &self.requirements
    }
}

// ============================================================================
// SHAPE VALIDATION RESULTS
// ============================================================================

/// A shape-validation result.
///
/// An unresolved result is not equivalent to a successful concrete
/// shape validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TensorShapeValidation {
    Compatible,
    Incompatible {
        reason: String,
    },
    Unresolved {
        constraints: Vec<ShapeConstraint>,
    },
}

/// Converts canonical shape analysis into an operation-level result.
///
/// The canonical shape analyzer remains responsible for determining
/// compatibility. This adapter preserves unresolved constraints.
pub fn classify_shape_analysis(
    analysis: ShapeAnalysis,
) -> TensorShapeValidation {
    match analysis {
        ShapeAnalysis::Compatible => TensorShapeValidation::Compatible,

        ShapeAnalysis::Incompatible { reason } => {
            TensorShapeValidation::Incompatible { reason }
        }

        ShapeAnalysis::Unresolved { constraints } => {
            TensorShapeValidation::Unresolved { constraints }
        }
    }
}

// ============================================================================
// VALIDATION
// ============================================================================

/// A structural validation issue.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TensorOperationIssue {
    DuplicateInputValue(TensorValueId),
    DuplicateOutputValue(TensorValueId),
    InputOutputIdentityCollision(TensorValueId),
    InvalidOperationArity,
    InvalidAttribute,
}

/// Validates the operation's structural invariants.
///
/// This does not validate operation-specific mathematics. A registered
/// operation schema must validate its own arity, attributes, shape rules,
/// dtype rules, effects, and capabilities.
pub fn validate_operation_structure(
    operation: &TensorOperation,
) -> Vec<TensorOperationIssue> {
    let mut issues = Vec::new();
    let mut input_ids = std::collections::BTreeSet::new();
    let mut output_ids = std::collections::BTreeSet::new();

    for input in operation.inputs() {
        if !input_ids.insert(input.id().clone()) {
            issues.push(
                TensorOperationIssue::DuplicateInputValue(input.id().clone()),
            );
        }
    }

    for output in operation.outputs() {
        if !output_ids.insert(output.id().clone()) {
            issues.push(
                TensorOperationIssue::DuplicateOutputValue(output.id().clone()),
            );
        }
    }

    for input_id in &input_ids {
        if output_ids.contains(input_id) {
            issues.push(
                TensorOperationIssue::InputOutputIdentityCollision(
                    input_id.clone(),
                ),
            );
        }
    }

    issues
}

// ============================================================================
// BUILT-IN MATHEMATICAL OPERATION DESCRIPTORS
// ============================================================================

/// Target-independent tensor operation family.
///
/// This is an extensible descriptor, not a closed list of all operations.
/// Custom operation identities remain supported.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[non_exhaustive]
pub enum TensorOperationKind {
    Elementwise,
    Broadcast,
    Reshape,
    Transpose,
    Reduce,
    MatMul,
    Contract,
    Einsum,
    Index,
    Slice,
    Convolution,
    Attention,
    Normalize,
    Sparse,
    Custom(TensorOperationId),
}

// ============================================================================
// OPERATION SCHEMA
// ============================================================================

/// Describes the semantic contract of a registered operation.
///
/// The registry owns schema registration and version compatibility.
/// This structure does not contain executable kernel code.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorOperationSchema {
    id: TensorOperationId,
    kind: TensorOperationKind,
    effects: Vec<TensorOperationEffect>,
    requirements: Vec<TensorCapabilityRequirement>,
}

impl TensorOperationSchema {
    pub fn new(
        id: TensorOperationId,
        kind: TensorOperationKind,
    ) -> Self {
        Self {
            id,
            kind,
            effects: vec![TensorOperationEffect::Pure],
            requirements: Vec::new(),
        }
    }

    pub fn with_effect(mut self, effect: TensorOperationEffect) -> Self {
        self.effects.push(effect);
        self
    }

    pub fn with_requirement(
        mut self,
        requirement: TensorCapabilityRequirement,
    ) -> Self {
        self.requirements.push(requirement);
        self
    }

    #[must_use]
    pub fn id(&self) -> &TensorOperationId {
        &self.id
    }

    #[must_use]
    pub fn kind(&self) -> &TensorOperationKind {
        &self.kind
    }

    #[must_use]
    pub fn effects(&self) -> &[TensorOperationEffect] {
        &self.effects
    }

    #[must_use]
    pub fn requirements(&self) -> &[TensorCapabilityRequirement] {
        &self.requirements
    }
}

// ============================================================================
// ERROR CONTRACT
// ============================================================================

/// Errors produced by tensor operation contract construction.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TensorOperationError {
    InvalidOperationIdentity,
    InvalidValueIdentity,
    InvalidAttributeName,
    InvalidCapabilityRequirement,
    MissingOperationOutput,
    DuplicateAttribute,
    InvalidOperationArity,
    InvalidShapeContract,
    InvalidDTypeContract,
    UnsupportedOperationSemantics,
}

impl fmt::Display for TensorOperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOperationIdentity => {
                write!(f, "invalid tensor operation identity")
            }
            Self::InvalidValueIdentity => {
                write!(f, "invalid tensor value identity")
            }
            Self::InvalidAttributeName => {
                write!(f, "invalid tensor operation attribute name")
            }
            Self::InvalidCapabilityRequirement => {
                write!(f, "invalid tensor capability requirement")
            }
            Self::MissingOperationOutput => {
                write!(f, "tensor operation must declare an output")
            }
            Self::DuplicateAttribute => {
                write!(f, "duplicate tensor operation attribute")
            }
            Self::InvalidOperationArity => {
                write!(f, "invalid tensor operation arity")
            }
            Self::InvalidShapeContract => {
                write!(f, "invalid tensor operation shape contract")
            }
            Self::InvalidDTypeContract => {
                write!(f, "invalid tensor operation dtype contract")
            }
            Self::UnsupportedOperationSemantics => {
                write!(f, "unsupported tensor operation semantics")
            }
        }
    }
}

impl std::error::Error for TensorOperationError {}
