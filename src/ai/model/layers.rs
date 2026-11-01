#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Model Layers
//!
//! Path: src/ai/model/layers.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define target-independent, declarative AI model layer descriptions.
//!
//! Layers describe computational components used to construct AI models.
//! They do not execute computations or allocate physical resources.
//!
//! A layer can represent:
//!
//! - Dense and fully connected computation.
//! - Convolutional computation.
//! - Attention.
//! - Recurrent computation.
//! - Normalization.
//! - Embedding.
//! - Pooling.
//! - Sparse computation.
//! - Symbolic computation.
//! - Probabilistic computation.
//! - Custom AI operations.
//! - Future AI computational paradigms.
//!
//! The set of layer kinds is open-ended.
//!
//! ## Architectural ownership
//!
//! Source syntax:
//!     grammar/ai/
//!
//! Canonical AST:
//!     crate::frontend::ast
//!
//! Canonical semantic types:
//!     crate::frontend::semantic::types::ty
//!
//! AI model identity:
//!     crate::ai::ids
//!
//! Model member identity:
//!     crate::ai::model::definition
//!
//! Model parameters:
//!     crate::ai::model::parameters
//!
//! Model graph:
//!     crate::ai::model::graph
//!
//! Model operations:
//!     crate::ai::model::operations
//!
//! Universal computational representation:
//!     crate::zuir
//!
//! ## Owns
//!
//! - Layer identity.
//! - Extensible layer kind identifiers.
//! - Layer input and output declarations.
//! - Layer parameter references.
//! - Layer operation references.
//! - Layer configuration metadata.
//! - Layer-local structural validation.
//! - Layer schema compatibility metadata.
//!
//! ## Does not own
//!
//! - Source parsing.
//! - AST construction.
//! - Semantic type definitions.
//! - Tensor shape authorities.
//! - Tensor storage.
//! - Parameter allocation.
//! - Model graph connectivity.
//! - Mathematical kernel implementations.
//! - Automatic differentiation.
//! - Hardware selection.
//! - Scheduling.
//! - Distributed execution.
//! - Universal IR.
//!
//! ## Scalability
//!
//! No fixed maximum number of layers, inputs, outputs, attributes,
//! parameters, or model members is imposed.
//!
//! No fixed model depth, tensor rank, parameter count, or hardware
//! capacity is imposed.
//!
//! Operational budgets belong to compiler and execution configuration.
//!
//! Physical feasibility is determined by downstream compilation and
//! execution infrastructure.
//!
//! This module does not claim literal infinite execution.
//!
//! ## Integration
//!
//! model/definition.rs:
//!     Owns model member declarations and overall model structure.
//!
//! model/graph.rs:
//!     Owns layer connectivity and graph relationships.
//!
//! model/operations.rs:
//!     Owns operation contracts and computational semantics.
//!
//! model/parameters.rs:
//!     Owns parameter declarations and parameter constraints.
//!
//! model/signatures.rs:
//!     Owns model input/output signatures.
//!
//! frontend/semantic/domains/ai.rs:
//!     Owns canonical AI semantic validation.
//!
//! zuir/dialects/ai:
//!     Owns universal AI IR operation definitions.
//!
//! zuir/dialects/tensor:
//!     Owns universal tensor IR operations.
//!
//! classical/collections/tensor.rs:
//!     Owns canonical classical tensor storage.
//!
//! This module provides declarations consumed by those systems.
//!
//! ## Freeze criteria
//!
//! - Layer kinds remain extensible.
//! - Duplicate layer ports are rejected.
//! - Duplicate parameter references are rejected.
//! - Invalid identifiers are rejected.
//! - Invalid layer schemas are rejected.
//! - Semantic types are canonical.
//! - No tensor storage is introduced.
//! - No execution engine is introduced.
//! - No fixed capacity ceilings are introduced.
//! - Rust 1.97+, edition 2021, safe Rust only.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::ai::ids::AiParameterId;

use crate::ai::model::definition::{
    ModelExpressionRef,
    ModelMemberId,
};

use crate::frontend::semantic::types::ty::{
    SemanticName,
    SemanticType,
};

// =============================================================================
// Layer identity
// =============================================================================

/// Identity of a layer within the AI model domain.
///
/// This identity is opaque and independent of physical hardware.
///
/// The canonical semantic adapter or model construction context is
/// responsible for assigning collision-free identities.
///
/// Layer identity is not a model graph edge or an IR operation ID.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
pub struct AiLayerId(Arc<str>);

impl AiLayerId {
    /// Construct a layer identity.
    pub fn new(
        value: impl AsRef<str>,
    ) -> Result<Self, LayerError> {
        let value = value.as_ref();

        validate_identifier(
            "layer identity",
            value,
        )?;

        Ok(Self(Arc::from(value)))
    }

    /// Return the opaque layer identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AiLayerId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Extensible layer kind
// =============================================================================

/// Open-ended identifier for a layer computational family.
///
/// The identifier is deliberately not a Rust enum.
///
/// A closed enum would require modifying this file whenever a new AI
/// computational paradigm is introduced.
///
/// Examples:
///
/// - ai.layer.dense
/// - ai.layer.convolution
/// - ai.layer.attention
/// - ai.layer.recurrent
/// - ai.layer.normalization
/// - ai.layer.embedding
/// - ai.layer.pooling
/// - ai.layer.sparse
/// - ai.layer.symbolic
/// - ai.layer.probabilistic
/// - extension.vendor.custom_layer
///
/// The identifier describes a computational family, not a kernel.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
pub struct LayerKindId(Arc<str>);

impl LayerKindId {
    /// Construct a qualified layer kind identifier.
    pub fn new(
        value: impl AsRef<str>,
    ) -> Result<Self, LayerError> {
        let value = value.as_ref();

        validate_qualified_identifier(
            "layer kind",
            value,
        )?;

        Ok(Self(Arc::from(value)))
    }

    /// Return the qualified layer kind identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LayerKindId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Layer port declarations
// =============================================================================

/// Semantic role of a layer port.
///
/// This is intentionally descriptive.
///
/// More detailed execution semantics belong to the operation contract
/// and the canonical AI semantic system.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
pub enum LayerPortRole {
    /// Ordinary computational input.
    Input,

    /// Ordinary computational output.
    Output,

    /// Persistent state input.
    StateInput,

    /// Persistent state output.
    StateOutput,

    /// Auxiliary computational input.
    AuxiliaryInput,

    /// Auxiliary computational output.
    AuxiliaryOutput,
}

/// A typed port in a layer declaration.
///
/// `SemanticType` is supplied by the canonical frontend semantic system.
///
/// This file does not introduce another tensor type system.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub struct LayerPort {
    /// Source-level port name.
    pub name: SemanticName,

    /// Canonical semantic type.
    pub ty: SemanticType,

    /// Port role.
    pub role: LayerPortRole,

    /// Optional canonical expression describing a port constraint.
    ///
    /// The expression is not evaluated here.
    pub constraint: Option<ModelExpressionRef>,
}

impl LayerPort {
    /// Construct a typed layer port.
    #[must_use]
    pub fn new(
        name: SemanticName,
        ty: SemanticType,
        role: LayerPortRole,
    ) -> Self {
        Self {
            name,
            ty,
            role,
            constraint: None,
        }
    }

    /// Attach a canonical constraint expression.
    #[must_use]
    pub fn with_constraint(
        mut self,
        constraint: ModelExpressionRef,
    ) -> Self {
        self.constraint = Some(constraint);
        self
    }
}

// =============================================================================
// Layer parameter references
// =============================================================================

/// Reference to a parameter declared by the model parameter subsystem.
///
/// This is not a parameter value.
///
/// No tensor is allocated by constructing this reference.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
pub struct LayerParameterRef {
    /// Canonical AI parameter identity.
    pub id: AiParameterId,

    /// Optional semantic role of the parameter.
    ///
    /// Examples:
    ///
    /// - weight
    /// - bias
    /// - scale
    /// - offset
    /// - query_projection
    /// - key_projection
    /// - value_projection
    ///
    /// This is metadata, not a closed enumeration.
    pub role: Option<String>,
}

impl LayerParameterRef {
    /// Construct a parameter reference.
    #[must_use]
    pub fn new(
        id: AiParameterId,
    ) -> Self {
        Self {
            id,
            role: None,
        }
    }

    /// Attach a descriptive parameter role.
    pub fn with_role(
        mut self,
        role: impl Into<String>,
    ) -> Result<Self, LayerError> {
        let role = role.into();

        validate_identifier(
            "parameter role",
            &role,
        )?;

        self.role = Some(role);

        Ok(self)
    }
}

// =============================================================================
// Layer attributes
// =============================================================================

/// A declarative layer attribute.
///
/// Attributes refer to canonical expressions rather than introducing
/// another expression AST.
///
/// Examples:
///
/// - activation function reference
/// - convolution stride
/// - attention configuration
/// - normalization epsilon
/// - recurrent state configuration
/// - pooling configuration
///
/// The semantic meaning of an attribute is defined by the layer kind
/// schema registered with the AI operation infrastructure.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
pub struct LayerAttribute {
    /// Qualified attribute name.
    pub name: String,

    /// Canonical expression reference.
    pub value: ModelExpressionRef,
}

impl LayerAttribute {
    /// Construct a layer attribute.
    pub fn new(
        name: impl Into<String>,
        value: ModelExpressionRef,
    ) -> Result<Self, LayerError> {
        let name = name.into();

        validate_qualified_identifier(
            "layer attribute",
            &name,
        )?;

        Ok(Self {
            name,
            value,
        })
    }
}

// =============================================================================
// Layer operation reference
// =============================================================================

/// Reference to the canonical AI operation represented by a layer.
///
/// The operation definition is owned by model/operations.rs and the
/// universal operation semantics are owned by ZUIR.
///
/// This reference does not contain an executable kernel.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
pub struct LayerOperationRef {
    /// Qualified operation identity.
    pub operation: String,

    /// Optional operation schema version.
    pub schema_version: Option<String>,
}

impl LayerOperationRef {
    /// Construct an operation reference.
    pub fn new(
        operation: impl Into<String>,
    ) -> Result<Self, LayerError> {
        let operation = operation.into();

        validate_qualified_identifier(
            "layer operation",
            &operation,
        )?;

        Ok(Self {
            operation,
            schema_version: None,
        })
    }

    /// Attach an operation schema version.
    pub fn with_schema_version(
        mut self,
        version: impl Into<String>,
    ) -> Result<Self, LayerError> {
        let version = version.into();

        validate_identifier(
            "operation schema version",
            &version,
        )?;

        self.schema_version = Some(version);

        Ok(self)
    }
}

// =============================================================================
// Layer schema version
// =============================================================================

/// Version of the declarative layer schema.
///
/// This is not a Zamani source-language version, a ZUIR dialect version,
/// or a model artifact version.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
pub struct LayerSchemaVersion {
    /// Major schema version.
    pub major: u64,

    /// Minor schema version.
    pub minor: u64,

    /// Patch schema version.
    pub patch: u64,
}

impl LayerSchemaVersion {
    /// Construct a schema version.
    #[must_use]
    pub const fn new(
        major: u64,
        minor: u64,
        patch: u64,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

// =============================================================================
// Layer declaration
// =============================================================================

/// Declarative, target-independent AI model layer.
///
/// A layer is a model component, not an execution engine.
///
/// The model graph owns connectivity. The parameter subsystem owns
/// parameter declarations. The operation subsystem owns operation
/// contracts.
///
/// This structure combines references to those declarations without
/// taking ownership of their execution semantics.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub struct ModelLayer {
    /// Layer identity.
    pub id: AiLayerId,

    /// Extensible computational family.
    pub kind: LayerKindId,

    /// Typed layer input declarations.
    pub inputs: Vec<LayerPort>,

    /// Typed layer output declarations.
    pub outputs: Vec<LayerPort>,

    /// References to canonical model parameters.
    pub parameters: Vec<LayerParameterRef>,

    /// Declarative layer attributes.
    pub attributes: Vec<LayerAttribute>,

    /// Canonical operation reference.
    pub operation: LayerOperationRef,

    /// Layer schema version.
    pub schema_version: LayerSchemaVersion,

    /// Optional model member identity.
    ///
    /// This reference allows the layer to be associated with the
    /// canonical model definition without duplicating model structure.
    pub member: Option<ModelMemberId>,
}

impl ModelLayer {
    /// Construct a layer declaration.
    ///
    /// Construction performs local structural validation.
    ///
    /// It does not perform canonical semantic type checking, parameter
    /// resolution, graph validation, or operation schema resolution.
    pub fn new(
        id: AiLayerId,
        kind: LayerKindId,
        operation: LayerOperationRef,
        schema_version: LayerSchemaVersion,
    ) -> Self {
        Self {
            id,
            kind,
            inputs: Vec::new(),
            outputs: Vec::new(),
            parameters: Vec::new(),
            attributes: Vec::new(),
            operation,
            schema_version,
            member: None,
        }
    }

    /// Attach a canonical model member identity.
    #[must_use]
    pub fn with_member(
        mut self,
        member: ModelMemberId,
    ) -> Self {
        self.member = Some(member);
        self
    }

    /// Append an input port.
    pub fn with_input(
        mut self,
        port: LayerPort,
    ) -> Result<Self, LayerError> {
        if port.role != LayerPortRole::Input
            && port.role != LayerPortRole::StateInput
            && port.role != LayerPortRole::AuxiliaryInput
        {
            return Err(
                LayerError::InvalidPortRole {
                    port: port.name.to_string(),
                    role: port.role,
                },
            );
        }

        ensure_unique_port(
            &self.inputs,
            &port,
        )?;

        self.inputs.push(port);

        Ok(self)
    }

    /// Append an output port.
    pub fn with_output(
        mut self,
        port: LayerPort,
    ) -> Result<Self, LayerError> {
        if port.role != LayerPortRole::Output
            && port.role != LayerPortRole::StateOutput
            && port.role != LayerPortRole::AuxiliaryOutput
        {
            return Err(
                LayerError::InvalidPortRole {
                    port: port.name.to_string(),
                    role: port.role,
                },
            );
        }

        ensure_unique_port(
            &self.outputs,
            &port,
        )?;

        self.outputs.push(port);

        Ok(self)
    }

    /// Append a parameter reference.
    pub fn with_parameter(
        mut self,
        parameter: LayerParameterRef,
    ) -> Result<Self, LayerError> {
        if self.parameters.iter().any(
            |existing| existing.id == parameter.id,
        ) {
            return Err(
                LayerError::DuplicateParameter(
                    parameter.id,
                ),
            );
        }

        self.parameters.push(parameter);

        Ok(self)
    }

    /// Append a layer attribute.
    pub fn with_attribute(
        mut self,
        attribute: LayerAttribute,
    ) -> Result<Self, LayerError> {
        if self.attributes.iter().any(
            |existing| existing.name == attribute.name,
        ) {
            return Err(
                LayerError::DuplicateAttribute(
                    attribute.name,
                ),
            );
        }

        self.attributes.push(attribute);

        Ok(self)
    }

    /// Validate local structural invariants.
    ///
    /// This method does not resolve external references.
    pub fn validate(&self) -> Result<(), LayerError> {
        validate_identifier(
            "layer identity",
            self.id.as_str(),
        )?;

        validate_qualified_identifier(
            "layer kind",
            self.kind.as_str(),
        )?;

        validate_qualified_identifier(
            "layer operation",
            &self.operation.operation,
        )?;

        validate_port_collection(
            &self.inputs,
        )?;

        validate_port_collection(
            &self.outputs,
        )?;

        let mut parameter_ids = BTreeSet::new();

        for parameter in &self.parameters {
            if !parameter_ids.insert(parameter.id) {
                return Err(
                    LayerError::DuplicateParameter(
                        parameter.id,
                    ),
                );
            }

            if let Some(role) = &parameter.role {
                validate_identifier(
                    "parameter role",
                    role,
                )?;
            }
        }

        let mut attribute_names = BTreeSet::new();

        for attribute in &self.attributes {
            validate_qualified_identifier(
                "layer attribute",
                &attribute.name,
            )?;

            if !attribute_names.insert(
                attribute.name.as_str(),
            ) {
                return Err(
                    LayerError::DuplicateAttribute(
                        attribute.name.clone(),
                    ),
                );
            }
        }

        Ok(())
    }

    /// Return whether the layer has no input ports.
    #[must_use]
    pub fn has_no_inputs(&self) -> bool {
        self.inputs.is_empty()
    }

    /// Return whether the layer has no output ports.
    #[must_use]
    pub fn has_no_outputs(&self) -> bool {
        self.outputs.is_empty()
    }

    /// Return the number of declared inputs.
    #[must_use]
    pub fn input_count(&self) -> usize {
        self.inputs.len()
    }

    /// Return the number of declared outputs.
    #[must_use]
    pub fn output_count(&self) -> usize {
        self.outputs.len()
    }

    /// Return the number of parameter references.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }
}

// =============================================================================
// Layer collection
// =============================================================================

/// Validated collection of AI model layer declarations.
///
/// This collection owns declarations only.
///
/// It does not own graph edges, execution state, or parameter storage.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub struct ModelLayers {
    layers: Vec<ModelLayer>,
    by_id: BTreeMap<AiLayerId, usize>,
}

impl ModelLayers {
    /// Construct and validate a collection of layers.
    ///
    /// The original declaration order is preserved.
    ///
    /// Duplicate identities are rejected.
    ///
    /// No fixed layer count is imposed.
    pub fn new(
        layers: Vec<ModelLayer>,
    ) -> Result<Self, LayerError> {
        let mut by_id = BTreeMap::new();

        for (index, layer) in layers.iter().enumerate() {
            layer.validate()?;

            if by_id.insert(
                layer.id.clone(),
                index,
            ).is_some() {
                return Err(
                    LayerError::DuplicateLayer(
                        layer.id.clone(),
                    ),
                );
            }
        }

        Ok(Self {
            layers,
            by_id,
        })
    }

    /// Return layers in declaration order.
    #[must_use]
    pub fn as_slice(&self) -> &[ModelLayer] {
        &self.layers
    }

    /// Consume the collection.
    #[must_use]
    pub fn into_vec(self) -> Vec<ModelLayer> {
        self.layers
    }

    /// Return the number of layers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.layers.len()
    }

    /// Return whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }

    /// Find a layer by its canonical identity.
    #[must_use]
    pub fn get(
        &self,
        id: &AiLayerId,
    ) -> Option<&ModelLayer> {
        self.by_id
            .get(id)
            .map(|&index| &self.layers[index])
    }

    /// Iterate over layers in declaration order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &ModelLayer> {
        self.layers.iter()
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_identifier(
    field: &'static str,
    value: &str,
) -> Result<(), LayerError> {
    if value.is_empty() {
        return Err(
            LayerError::EmptyIdentifier {
                field,
            },
        );
    }

    if value.chars().any(
        char::is_control,
    ) {
        return Err(
            LayerError::ControlCharacter {
                field,
            },
        );
    }

    Ok(())
}

fn validate_qualified_identifier(
    field: &'static str,
    value: &str,
) -> Result<(), LayerError> {
    validate_identifier(
        field,
        value,
    )?;

    if value.split('.').any(
        str::is_empty,
    ) {
        return Err(
            LayerError::InvalidQualifiedIdentifier {
                field,
            },
        );
    }

    Ok(())
}

fn ensure_unique_port(
    existing: &[LayerPort],
    candidate: &LayerPort,
) -> Result<(), LayerError> {
    if existing.iter().any(
        |port| port.name == candidate.name,
    ) {
        return Err(
            LayerError::DuplicatePort(
                candidate.name.to_string(),
            ),
        );
    }

    Ok(())
}

fn validate_port_collection(
    ports: &[LayerPort],
) -> Result<(), LayerError> {
    let mut names = BTreeSet::new();

    for port in ports {
        let name = port.name.to_string();

        validate_identifier(
            "port name",
            &name,
        )?;

        if !names.insert(name.clone()) {
            return Err(
                LayerError::DuplicatePort(
                    name,
                ),
            );
        }
    }

    Ok(())
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by layer declaration validation.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub enum LayerError {
    /// An identifier was empty.
    EmptyIdentifier {
        field: &'static str,
    },

    /// An identifier contained a control character.
    ControlCharacter {
        field: &'static str,
    },

    /// A qualified identifier was malformed.
    InvalidQualifiedIdentifier {
        field: &'static str,
    },

    /// A port was declared more than once.
    DuplicatePort(
        String,
    ),

    /// A parameter reference was declared more than once.
    DuplicateParameter(
        AiParameterId,
    ),

    /// An attribute was declared more than once.
    DuplicateAttribute(
        String,
    ),

    /// A layer identity was declared more than once.
    DuplicateLayer(
        AiLayerId,
    ),

    /// A port used an invalid role for its collection.
    InvalidPortRole {
        port: String,
        role: LayerPortRole,
    },
}

impl fmt::Display for LayerError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentifier {
                field,
            } => {
                write!(
                    formatter,
                    "{field} must not be empty"
                )
            }

            Self::ControlCharacter {
                field,
            } => {
                write!(
                    formatter,
                    "{field} contains a control character"
                )
            }

            Self::InvalidQualifiedIdentifier {
                field,
            } => {
                write!(
                    formatter,
                    "{field} is not a valid qualified identifier"
                )
            }

            Self::DuplicatePort(
                name,
            ) => {
                write!(
                    formatter,
                    "duplicate layer port: {name}"
                )
            }

            Self::DuplicateParameter(
                id,
            ) => {
                write!(
                    formatter,
                    "duplicate layer parameter reference: {id:?}"
                )
            }

            Self::DuplicateAttribute(
                name,
            ) => {
                write!(
                    formatter,
                    "duplicate layer attribute: {name}"
                )
            }

            Self::DuplicateLayer(
                id,
            ) => {
                write!(
                    formatter,
                    "duplicate layer identity: {id}"
                )
            }

            Self::InvalidPortRole {
                port,
                role,
            } => {
                write!(
                    formatter,
                    "invalid role {role:?} for layer port {port}"
                )
            }
        }
    }
}

impl Error for LayerError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_kind_is_extensible() {
        let kind = LayerKindId::new(
            "extension.custom.future_ai_layer",
        )
        .expect("valid layer kind");

        assert_eq!(
            kind.as_str(),
            "extension.custom.future_ai_layer",
        );
    }

    #[test]
    fn empty_layer_kind_is_rejected() {
        assert!(
            LayerKindId::new("").is_err()
        );
    }

    #[test]
    fn malformed_qualified_layer_kind_is_rejected() {
        assert!(
            LayerKindId::new(
                "ai..layer",
            ).is_err()
        );
    }

    #[test]
    fn layer_identity_is_opaque() {
        let id = AiLayerId::new(
            "model.encoder.layer_1",
        )
        .expect("valid identity");

        assert_eq!(
            id.as_str(),
            "model.encoder.layer_1",
        );
    }

    #[test]
    fn duplicate_layer_identity_is_rejected() {
        let id = AiLayerId::new(
            "layer.one",
        )
        .expect("valid identity");

        let kind = LayerKindId::new(
            "ai.layer.dense",
        )
        .expect("valid kind");

        let operation = LayerOperationRef::new(
            "ai.operation.dense",
        )
        .expect("valid operation");

        let first = ModelLayer::new(
            id.clone(),
            kind.clone(),
            operation.clone(),
            LayerSchemaVersion::new(
                1,
                0,
                0,
            ),
        );

        let second = ModelLayer::new(
            id,
            kind,
            operation,
            LayerSchemaVersion::new(
                1,
                0,
                0,
            ),
        );

        assert!(
            ModelLayers::new(
                vec![
                    first,
                    second,
                ],
            ).is_err()
        );
    }

    #[test]
    fn empty_layer_collection_is_valid() {
        let layers = ModelLayers::new(
            Vec::new(),
        )
        .expect("empty collection is structurally valid");

        assert!(
            layers.is_empty()
        );
    }
}