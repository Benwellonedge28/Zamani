
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Model Operations
//!
//! File: src/ai/model/operations.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define target-independent semantic contracts for operations used in
//! AI model declarations.
//!
//! An operation describes a computational relationship between typed
//! inputs, outputs, attributes, effects, and differentiation behavior.
//!
//! It does not implement the operation.
//!
//! ## Canonical ownership
//!
//! - Source syntax: grammar/ai/models.g4 and related AI grammars.
//! - Canonical AST: crate::frontend::ast.
//! - Canonical semantic types: crate::frontend::semantic::types::ty.
//! - Canonical model definitions: crate::ai::model::definition.
//! - Canonical model graph: crate::ai::model::graph.
//! - AI semantic validation: crate::frontend::semantic::domains::ai.
//! - Universal AI operations: crate::zuir::dialects::ai.
//! - Tensor storage: crate::classical::collections::tensor.
//!
//! ## Scalability
//!
//! This module defines no universal ceilings on:
//!
//! - operation count;
//! - input or output count;
//! - attribute count;
//! - tensor rank;
//! - model size;
//! - model depth;
//! - accelerator count;
//! - device count;
//! - memory capacity.
//!
//! Actual resource limits belong to explicit operational policies and
//! the execution infrastructure.
//!
//! ## Integration
//!
//! The frontend adapter converts canonical AST declarations into these
//! operation contracts.
//!
//! The AI semantic subsystem validates language-level meaning.
//!
//! The AI registry resolves operation identities and versions.
//!
//! ZUIR owns universal computational operation schemas and lowering.
//!
//! Runtime and backend implementations execute lowered operations.
//!
//! This module introduces neither a second AST nor a second IR.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::ai::model::definition::ModelExpressionRef;
use crate::frontend::semantic::types::ty::{
    QualifiedName,
    SemanticName,
    SemanticType,
};

// ============================================================================
// OPERATION IDENTITY
// ============================================================================

/// Stable semantic identity of an AI model operation.
///
/// The identity is independent of the implementation provider, device,
/// execution backend, and physical hardware.
///
/// Qualified operation identities should be resolved through the
/// canonical AI operation registry.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ModelOperationId(Arc<str>);

impl ModelOperationId {
    /// Creates an operation identity.
    ///
    /// The identifier must be non-empty and contain no control characters.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ModelOperationError> {
        let value = value.as_ref();

        validate_non_empty_text("operation identity", value)?;

        Ok(Self(Arc::from(value)))
    }

    /// Returns the identity string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ModelOperationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// OPERATION VERSION
// ============================================================================

/// Version of an operation contract.
///
/// This version is distinct from:
///
/// - the Zamani source-language version;
/// - the AI subsystem version;
/// - the ZUIR dialect version;
/// - the implementation provider version.
///
/// The registry and compatibility subsystem determine whether two
/// operation versions are compatible.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ModelOperationVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl ModelOperationVersion {
    /// Creates an operation contract version.
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major version.
    #[must_use]
    pub const fn major(&self) -> u64 {
        self.major
    }

    /// Returns the minor version.
    #[must_use]
    pub const fn minor(&self) -> u64 {
        self.minor
    }

    /// Returns the patch version.
    #[must_use]
    pub const fn patch(&self) -> u64 {
        self.patch
    }
}

impl fmt::Display for ModelOperationVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

// ============================================================================
// PORT DIRECTION
// ============================================================================

/// Direction of an operation port.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ModelOperationPortDirection {
    Input,
    Output,
}

// ============================================================================
// PORT IDENTITY
// ============================================================================

/// Stable identity of a port within an operation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ModelOperationPortId(Arc<str>);

impl ModelOperationPortId {
    /// Creates a port identity.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ModelOperationError> {
        let value = value.as_ref();

        validate_non_empty_text("port identity", value)?;

        Ok(Self(Arc::from(value)))
    }

    /// Returns the identity string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ModelOperationPortId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// PORT CARDINALITY
// ============================================================================

/// Cardinality contract for an operation port.
///
/// Cardinality is not a universal capacity limit.
///
/// It describes the number of values accepted or produced by the
/// operation contract.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ModelOperationPortCardinality {
    /// Exactly one value.
    One,

    /// Zero or one value.
    Optional,

    /// A dynamically sized collection of values.
    ///
    /// The collection is constrained by the supplied semantic expression
    /// when a constraint is present.
    Many {
        constraint: Option<ModelExpressionRef>,
    },

    /// An extension-defined cardinality contract.
    Custom {
        identity: QualifiedName,
        constraint: Option<ModelExpressionRef>,
    },
}

impl ModelOperationPortCardinality {
    fn validate(&self) -> Result<(), ModelOperationError> {
        if let Self::Custom { identity, .. } = self {
            identity
                .validate()
                .map_err(|_| ModelOperationError::InvalidCardinality)?;
        }

        Ok(())
    }
}

// ============================================================================
// OPERATION PORT
// ============================================================================

/// A typed input or output port of an operation.
///
/// The semantic type is provided by the canonical type system.
///
/// This type does not define another tensor type system.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelOperationPort {
    /// Stable identity within the operation.
    pub id: ModelOperationPortId,

    /// Source-level or semantic port name.
    pub name: SemanticName,

    /// Port direction.
    pub direction: ModelOperationPortDirection,

    /// Canonical semantic type.
    pub ty: SemanticType,

    /// Port cardinality.
    pub cardinality: ModelOperationPortCardinality,

    /// Whether the port is required.
    pub required: bool,

    /// Optional semantic expression describing a default value.
    ///
    /// The expression is resolved and evaluated by the canonical
    /// frontend and semantic infrastructure.
    pub default_value: Option<ModelExpressionRef>,
}

impl ModelOperationPort {
    /// Creates a required single-value port.
    #[must_use]
    pub fn new(
        id: ModelOperationPortId,
        name: SemanticName,
        direction: ModelOperationPortDirection,
        ty: SemanticType,
    ) -> Self {
        Self {
            id,
            name,
            direction,
            ty,
            cardinality: ModelOperationPortCardinality::One,
            required: true,
            default_value: None,
        }
    }

    /// Sets the port cardinality.
    #[must_use]
    pub fn with_cardinality(
        mut self,
        cardinality: ModelOperationPortCardinality,
    ) -> Self {
        self.cardinality = cardinality;
        self
    }

    /// Sets whether the port is required.
    #[must_use]
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Attaches a canonical default expression reference.
    #[must_use]
    pub fn with_default_value(
        mut self,
        expression: ModelExpressionRef,
    ) -> Self {
        self.default_value = Some(expression);
        self
    }

    /// Validates local port invariants.
    pub fn validate(&self) -> Result<(), ModelOperationError> {
        self.name
            .validate()
            .map_err(|_| ModelOperationError::InvalidPortName)?;

        self.cardinality.validate()?;

        if self.direction == ModelOperationPortDirection::Output
            && self.default_value.is_some()
        {
            return Err(ModelOperationError::OutputHasDefaultValue);
        }

        if self.required && self.default_value.is_some() {
            return Err(ModelOperationError::RequiredPortHasDefaultValue);
        }

        Ok(())
    }
}

// ============================================================================
// ATTRIBUTE IDENTITY
// ============================================================================

/// Stable identity of an operation attribute.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ModelOperationAttributeId(Arc<str>);

impl ModelOperationAttributeId {
    /// Creates an attribute identity.
    pub fn new(
        value: impl AsRef<str>,
    ) -> Result<Self, ModelOperationError> {
        let value = value.as_ref();

        validate_non_empty_text("attribute identity", value)?;

        Ok(Self(Arc::from(value)))
    }

    /// Returns the identity string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// ATTRIBUTE DECLARATION
// ============================================================================

/// Declarative attribute of an operation.
///
/// Attributes describe operation configuration or semantic parameters.
///
/// Attribute expressions are not evaluated by this module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelOperationAttribute {
    /// Stable attribute identity.
    pub id: ModelOperationAttributeId,

    /// Semantic attribute name.
    pub name: SemanticName,

    /// Canonical semantic type.
    pub ty: SemanticType,

    /// Whether the attribute is required.
    pub required: bool,

    /// Optional canonical expression reference for its default.
    pub default_value: Option<ModelExpressionRef>,

    /// Optional canonical expression reference for its constraint.
    pub constraint: Option<ModelExpressionRef>,
}

impl ModelOperationAttribute {
    /// Creates an attribute declaration.
    #[must_use]
    pub fn new(
        id: ModelOperationAttributeId,
        name: SemanticName,
        ty: SemanticType,
    ) -> Self {
        Self {
            id,
            name,
            ty,
            required: true,
            default_value: None,
            constraint: None,
        }
    }

    /// Sets whether the attribute is required.
    #[must_use]
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Attaches a default expression.
    #[must_use]
    pub fn with_default_value(
        mut self,
        expression: ModelExpressionRef,
    ) -> Self {
        self.default_value = Some(expression);
        self
    }

    /// Attaches a constraint expression.
    #[must_use]
    pub fn with_constraint(
        mut self,
        expression: ModelExpressionRef,
    ) -> Self {
        self.constraint = Some(expression);
        self
    }

    /// Validates local attribute invariants.
    pub fn validate(&self) -> Result<(), ModelOperationError> {
        self.name
            .validate()
            .map_err(|_| ModelOperationError::InvalidAttributeName)?;

        if self.required && self.default_value.is_some() {
            return Err(
                ModelOperationError::RequiredAttributeHasDefaultValue,
            );
        }

        Ok(())
    }
}

// ============================================================================
// EFFECT CONTRACTS
// ============================================================================

/// Effect classification for an operation.
///
/// These declarations are references to semantic effects.
///
/// The canonical effect system remains authoritative for effect
/// definitions, composition, and checking.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum ModelOperationEffect {
    /// The operation has no externally observable side effects.
    Pure,

    /// The operation reads state.
    ReadState,

    /// The operation writes state.
    WriteState,

    /// The operation performs communication.
    Communication,

    /// The operation consumes random or nondeterministic input.
    Nondeterministic,

    /// The operation accesses external data.
    ExternalData,

    /// An extension-defined effect.
    Custom(QualifiedName),
}

impl ModelOperationEffect {
    fn validate(&self) -> Result<(), ModelOperationError> {
        if let Self::Custom(identity) = self {
            identity
                .validate()
                .map_err(|_| ModelOperationError::InvalidEffect)?;
        }

        Ok(())
    }
}

// ============================================================================
// DIFFERENTIABILITY CONTRACT
// ============================================================================

/// Differentiability contract of an operation.
///
/// This is a semantic declaration, not a differentiation implementation.
///
/// Actual derivative construction belongs to the differentiation
/// subsystem and canonical ZUIR lowering.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ModelOperationDifferentiability {
    /// The operation is not differentiable.
    NotDifferentiable,

    /// Differentiability is determined by canonical semantic analysis.
    Inferred,

    /// The operation is differentiable under the supplied contract.
    Differentiable {
        derivative: QualifiedName,
    },

    /// The operation supports a custom differentiation contract.
    Custom {
        identity: QualifiedName,
        derivative: Option<QualifiedName>,
    },
}

impl ModelOperationDifferentiability {
    fn validate(&self) -> Result<(), ModelOperationError> {
        match self {
            Self::Differentiable { derivative } => {
                derivative
                    .validate()
                    .map_err(|_| ModelOperationError::InvalidDifferentiability)?;
            }

            Self::Custom {
                identity,
                derivative,
            } => {
                identity
                    .validate()
                    .map_err(|_| ModelOperationError::InvalidDifferentiability)?;

                if let Some(derivative) = derivative {
                    derivative
                        .validate()
                        .map_err(|_| {
                            ModelOperationError::InvalidDifferentiability
                        })?;
                }
            }

            Self::NotDifferentiable | Self::Inferred => {}
        }

        Ok(())
    }
}

// ============================================================================
// OPERATION COMPATIBILITY
// ============================================================================

/// Compatibility classification for an operation contract.
///
/// Compatibility is checked against another operation contract or
/// implementation descriptor.
///
/// This module does not select an implementation provider.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ModelOperationCompatibility {
    /// Compatibility is determined by the canonical registry.
    Registry,

    /// The operation requires exact contract compatibility.
    Exact,

    /// Compatibility may be established through a declared adapter.
    AdapterRequired {
        adapter: QualifiedName,
    },

    /// An extension-defined compatibility policy.
    Custom {
        identity: QualifiedName,
    },
}

impl ModelOperationCompatibility {
    fn validate(&self) -> Result<(), ModelOperationError> {
        match self {
            Self::AdapterRequired { adapter } => {
                adapter
                    .validate()
                    .map_err(|_| ModelOperationError::InvalidCompatibility)?;
            }

            Self::Custom { identity } => {
                identity
                    .validate()
                    .map_err(|_| ModelOperationError::InvalidCompatibility)?;
            }

            Self::Registry | Self::Exact => {}
        }

        Ok(())
    }
}

// ============================================================================
// OPERATION SIGNATURE
// ============================================================================

/// Signature of a model operation.
///
/// Input and output ordering is preserved.
///
/// The signature uses canonical semantic types and does not contain
/// executable values or tensor storage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelOperationSignature {
    /// Input ports in declaration order.
    pub inputs: Vec<ModelOperationPort>,

    /// Output ports in declaration order.
    pub outputs: Vec<ModelOperationPort>,
}

impl ModelOperationSignature {
    /// Creates an operation signature.
    #[must_use]
    pub fn new(
        inputs: Vec<ModelOperationPort>,
        outputs: Vec<ModelOperationPort>,
    ) -> Self {
        Self { inputs, outputs }
    }

    /// Validates port direction and identity invariants.
    pub fn validate(&self) -> Result<(), ModelOperationError> {
        let mut identities = BTreeSet::new();
        let mut names = BTreeSet::new();

        for input in &self.inputs {
            input.validate()?;

            if input.direction != ModelOperationPortDirection::Input {
                return Err(ModelOperationError::InvalidInputDirection);
            }

            if !identities.insert(input.id.clone()) {
                return Err(ModelOperationError::DuplicatePortIdentity);
            }

            if !names.insert(input.name.clone()) {
                return Err(ModelOperationError::DuplicatePortName);
            }
        }

        for output in &self.outputs {
            output.validate()?;

            if output.direction != ModelOperationPortDirection::Output {
                return Err(ModelOperationError::InvalidOutputDirection);
            }

            if !identities.insert(output.id.clone()) {
                return Err(ModelOperationError::DuplicatePortIdentity);
            }

            if !names.insert(output.name.clone()) {
                return Err(ModelOperationError::DuplicatePortName);
            }
        }

        Ok(())
    }
}

// ============================================================================
// OPERATION CONTRACT
// ============================================================================

/// Target-independent semantic contract for an AI model operation.
///
/// The contract describes what the operation means.
///
/// It does not define:
///
/// - a CPU implementation;
/// - a GPU kernel;
/// - a QPU implementation;
/// - an FPGA implementation;
/// - a hardware allocation;
/// - an execution schedule;
/// - an executable IR.
///
/// Those responsibilities remain with the canonical compiler,
/// dialect, and runtime infrastructure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelOperation {
    /// Stable operation identity.
    pub id: ModelOperationId,

    /// Operation contract version.
    pub version: ModelOperationVersion,

    /// Canonical operation name.
    pub name: QualifiedName,

    /// Semantic operation signature.
    pub signature: ModelOperationSignature,

    /// Operation attributes.
    pub attributes: Vec<ModelOperationAttribute>,

    /// Declared effects.
    pub effects: Vec<ModelOperationEffect>,

    /// Differentiability contract.
    pub differentiability: ModelOperationDifferentiability,

    /// Compatibility policy.
    pub compatibility: ModelOperationCompatibility,

    /// Optional canonical expression reference describing a precondition.
    pub precondition: Option<ModelExpressionRef>,

    /// Optional canonical expression reference describing a postcondition.
    pub postcondition: Option<ModelExpressionRef>,
}

impl ModelOperation {
    /// Creates an operation contract.
    #[must_use]
    pub fn new(
        id: ModelOperationId,
        version: ModelOperationVersion,
        name: QualifiedName,
        signature: ModelOperationSignature,
    ) -> Self {
        Self {
            id,
            version,
            name,
            signature,
            attributes: Vec::new(),
            effects: Vec::new(),
            differentiability: ModelOperationDifferentiability::Inferred,
            compatibility: ModelOperationCompatibility::Registry,
            precondition: None,
            postcondition: None,
        }
    }

    /// Adds an attribute declaration.
    #[must_use]
    pub fn with_attribute(
        mut self,
        attribute: ModelOperationAttribute,
    ) -> Self {
        self.attributes.push(attribute);
        self
    }

    /// Adds an effect declaration.
    #[must_use]
    pub fn with_effect(
        mut self,
        effect: ModelOperationEffect,
    ) -> Self {
        self.effects.push(effect);
        self
    }

    /// Sets the differentiability contract.
    #[must_use]
    pub fn with_differentiability(
        mut self,
        differentiability: ModelOperationDifferentiability,
    ) -> Self {
        self.differentiability = differentiability;
        self
    }

    /// Sets the compatibility policy.
    #[must_use]
    pub fn with_compatibility(
        mut self,
        compatibility: ModelOperationCompatibility,
    ) -> Self {
        self.compatibility = compatibility;
        self
    }

    /// Sets a canonical precondition reference.
    #[must_use]
    pub fn with_precondition(
        mut self,
        precondition: ModelExpressionRef,
    ) -> Self {
        self.precondition = Some(precondition);
        self
    }

    /// Sets a canonical postcondition reference.
    #[must_use]
    pub fn with_postcondition(
        mut self,
        postcondition: ModelExpressionRef,
    ) -> Self {
        self.postcondition = Some(postcondition);
        self
    }

    /// Validates local structural invariants.
    ///
    /// This method does not perform:
    ///
    /// - name resolution;
    /// - type inference;
    /// - shape inference;
    /// - effect checking;
    /// - capability checking;
    /// - resource feasibility;
    /// - implementation selection;
    /// - execution.
    ///
    /// These responsibilities remain with their canonical owners.
    pub fn validate(&self) -> Result<(), ModelOperationError> {
        self.name
            .validate()
            .map_err(|_| ModelOperationError::InvalidOperationName)?;

        self.signature.validate()?;

        self.differentiability.validate()?;
        self.compatibility.validate()?;

        self.validate_attributes()?;
        self.validate_effects()?;

        Ok(())
    }

    fn validate_attributes(
        &self,
    ) -> Result<(), ModelOperationError> {
        let mut identities = BTreeSet::new();
        let mut names = BTreeSet::new();

        for attribute in &self.attributes {
            attribute.validate()?;

            if !identities.insert(attribute.id.clone()) {
                return Err(
                    ModelOperationError::DuplicateAttributeIdentity,
                );
            }

            if !names.insert(attribute.name.clone()) {
                return Err(ModelOperationError::DuplicateAttributeName);
            }
        }

        Ok(())
    }

    fn validate_effects(
        &self,
    ) -> Result<(), ModelOperationError> {
        let mut effects = BTreeSet::new();

        for effect in &self.effects {
            effect.validate()?;

            if !effects.insert(effect.clone()) {
                return Err(ModelOperationError::DuplicateEffect);
            }
        }

        if effects.contains(&ModelOperationEffect::Pure)
            && effects.len() > 1
        {
            return Err(ModelOperationError::ConflictingEffects);
        }

        Ok(())
    }
}

// ============================================================================
// VALIDATION ERRORS
// ============================================================================

/// Structural errors in an AI model operation contract.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ModelOperationError {
    InvalidText(&'static str),

    InvalidOperationName,
    InvalidPortName,
    InvalidAttributeName,

    InvalidCardinality,
    InvalidEffect,
    InvalidDifferentiability,
    InvalidCompatibility,

    InvalidInputDirection,
    InvalidOutputDirection,

    DuplicatePortIdentity,
    DuplicatePortName,

    DuplicateAttributeIdentity,
    DuplicateAttributeName,

    DuplicateEffect,
    ConflictingEffects,

    OutputHasDefaultValue,
    RequiredPortHasDefaultValue,
    RequiredAttributeHasDefaultValue,
}

impl fmt::Display for ModelOperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidText(field) => {
                write!(f, "invalid {field}")
            }

            Self::InvalidOperationName => {
                f.write_str("invalid operation name")
            }

            Self::InvalidPortName => {
                f.write_str("invalid operation port name")
            }

            Self::InvalidAttributeName => {
                f.write_str("invalid operation attribute name")
            }

            Self::InvalidCardinality => {
                f.write_str("invalid operation port cardinality")
            }

            Self::InvalidEffect => {
                f.write_str("invalid operation effect")
            }

            Self::InvalidDifferentiability => {
                f.write_str("invalid differentiability contract")
            }

            Self::InvalidCompatibility => {
                f.write_str("invalid operation compatibility policy")
            }

            Self::InvalidInputDirection => {
                f.write_str("input port has output direction")
            }

            Self::InvalidOutputDirection => {
                f.write_str("output port has input direction")
            }

            Self::DuplicatePortIdentity => {
                f.write_str("duplicate operation port identity")
            }

            Self::DuplicatePortName => {
                f.write_str("duplicate operation port name")
            }

            Self::DuplicateAttributeIdentity => {
                f.write_str("duplicate operation attribute identity")
            }

            Self::DuplicateAttributeName => {
                f.write_str("duplicate operation attribute name")
            }

            Self::DuplicateEffect => {
                f.write_str("duplicate operation effect")
            }

            Self::ConflictingEffects => {
                f.write_str("pure effect conflicts with other effects")
            }

            Self::OutputHasDefaultValue => {
                f.write_str("output port cannot declare a default value")
            }

            Self::RequiredPortHasDefaultValue => {
                f.write_str(
                    "required input port cannot declare a default value",
                )
            }

            Self::RequiredAttributeHasDefaultValue => {
                f.write_str(
                    "required attribute cannot declare a default value",
                )
            }
        }
    }
}

impl Error for ModelOperationError {}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn validate_non_empty_text(
    field: &'static str,
    value: &str,
) -> Result<(), ModelOperationError> {
    if value.trim().is_empty()
        || value.chars().any(char::is_control)
    {
        return Err(ModelOperationError::InvalidText(field));
    }

    Ok(())
}
