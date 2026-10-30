
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Model Definitions
//!
//! File: src/ai/model/definition.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define the canonical, target-independent semantic representation of an
//! AI model declaration after conversion from the native Zamani frontend AST.
//!
//! ## Architectural authority
//!
//! Source syntax:
//!     grammar/ai/models.g4
//!
//! Canonical AST:
//!     crate::frontend::ast
//!
//! Canonical semantic types:
//!     crate::frontend::semantic::types::ty
//!
//! Generic trait obligations:
//!     crate::frontend::semantic::types::traits
//!
//! AI semantic validation:
//!     crate::frontend::semantic::domains::ai
//!
//! Universal computational representation:
//!     crate::zuir
//!
//! ## Ownership
//!
//! This file owns:
//! - AI model semantic definitions.
//! - Model identity and declaration metadata.
//! - Generic model parameters and bounds.
//! - Model inheritance and interface references.
//! - Model member identities and semantic roles.
//! - Typed member declarations.
//! - Nested model regions.
//! - Model expression references.
//! - Model connection references.
//! - Local structural model validation.
//!
//! ## Does not own
//!
//! - Source grammar or AST construction.
//! - Canonical type definitions.
//! - Type inference or generic substitution.
//! - Expression evaluation.
//! - Tensor storage.
//! - Model graph execution.
//! - Training or inference implementation.
//! - Hardware selection or allocation.
//! - Scheduling or distributed execution.
//! - ZUIR operation implementation.
//! - Quantum IR.
//!
//! ## Scalability
//!
//! No fixed model size, parameter count, member count, nesting depth,
//! tensor rank, accelerator count, device count, or machine capacity is
//! imposed.
//!
//! Operational budgets belong to the compiler and execution configuration.
//! An operational resource failure must not alter model semantics.
//!
//! This does not promise literal infinite execution: actual operations remain
//! bounded by available resources, addressability, and explicit policies.
//!
//! ## Integration
//!
//! The frontend adapter resolves canonical AST declarations and supplies
//! canonical semantic types and identities.
//!
//! The AI semantic validator performs language-level checks.
//!
//! This module performs only local model-definition structural validation.
//!
//! Downstream ZUIR adapters translate validated model meaning into canonical
//! computational operations. No second AI IR is introduced.
//!
//! Quantum computation continues through quantum::ir.
//!
//! ## Freeze criteria
//!
//! - Canonical AST adapter is integrated.
//! - Semantic validation is integrated.
//! - Generic bounds are checked by the canonical type system.
//! - Model connection endpoints are resolved.
//! - All structural and malformed-input tests pass.
//! - Workspace compilation and lint checks pass.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;

use crate::frontend::semantic::types::traits::TraitRefPattern;

use crate::frontend::semantic::types::ty::{
    QualifiedName,
    SemanticName,
    SemanticType,
    TypeParameter,
};

// =============================================================================
// Model member identity
// =============================================================================

/// Stable identity of a member within an AI model.
///
/// The identity is opaque to this module. It must be assigned by the
/// canonical semantic adapter or a collision-checked model construction
/// context.
///
/// It is not a hardware resource identifier.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ModelMemberId(Arc<str>);

impl ModelMemberId {
    /// Creates a model member identity.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ModelDefinitionError> {
        let value = value.as_ref();

        if value.is_empty() || value.chars().any(char::is_control) {
            return Err(ModelDefinitionError::InvalidMemberIdentity);
        }

        Ok(Self(Arc::from(value)))
    }

    /// Returns the opaque identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// =============================================================================
// Canonical expression references
// =============================================================================

/// Reference to an expression already represented by the canonical
/// frontend/semantic infrastructure.
///
/// This type does not contain or duplicate an expression AST.
///
/// The identity must be assigned by the canonical semantic adapter.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ModelExpressionRef(Arc<str>);

impl ModelExpressionRef {
    /// Creates an expression reference.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ModelDefinitionError> {
        let value = value.as_ref();

        if value.is_empty() || value.chars().any(char::is_control) {
            return Err(ModelDefinitionError::InvalidExpressionReference);
        }

        Ok(Self(Arc::from(value)))
    }

    /// Returns the referenced semantic identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// =============================================================================
// Model annotations
// =============================================================================

/// A resolved annotation identity.
///
/// Annotation syntax and annotation argument evaluation remain owned by
/// the canonical frontend and semantic systems.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelAnnotation {
    /// Canonical annotation identity.
    pub name: QualifiedName,

    /// References to already-resolved annotation argument expressions.
    ///
    /// The references preserve the distinction between syntax and resolved
    /// semantic meaning without introducing another expression system.
    pub arguments: Vec<ModelExpressionRef>,
}

impl ModelAnnotation {
    /// Creates an annotation reference.
    #[must_use]
    pub fn new(
        name: QualifiedName,
        arguments: Vec<ModelExpressionRef>,
    ) -> Self {
        Self { name, arguments }
    }

    fn validate(&self) -> Result<(), ModelDefinitionError> {
        self.name
            .validate()
            .map_err(|_| ModelDefinitionError::InvalidAnnotation)?;

        Ok(())
    }
}

// =============================================================================
// Generic model parameters
// =============================================================================

/// A generic parameter declared by an AI model.
///
/// The actual generic parameter representation belongs to the canonical
/// semantic type subsystem.
///
/// Trait obligations are represented by the canonical TraitRefPattern.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelGenericParameter {
    /// Canonical generic parameter.
    pub parameter: TypeParameter,

    /// Required trait bounds.
    pub bounds: Vec<TraitRefPattern>,
}

impl ModelGenericParameter {
    /// Creates a generic parameter.
    #[must_use]
    pub fn new(
        parameter: TypeParameter,
        bounds: Vec<TraitRefPattern>,
    ) -> Self {
        Self { parameter, bounds }
    }

    fn validate(&self) -> Result<(), ModelDefinitionError> {
        self.parameter
            .name
            .validate()
            .map_err(|_| ModelDefinitionError::InvalidGenericParameter)?;

        if let Some(declaration) = &self.parameter.declaration {
            declaration
                .validate()
                .map_err(|_| ModelDefinitionError::InvalidGenericParameter)?;
        }

        for bound in &self.bounds {
            bound
                .name
                .validate()
                .map_err(|_| ModelDefinitionError::InvalidGenericBound)?;
        }

        Ok(())
    }
}

// =============================================================================
// Model member roles
// =============================================================================

/// Semantic role of a model member.
///
/// Custom permits future extensions without continually expanding a
/// closed list of model architectures or vendor-specific concepts.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[non_exhaustive]
pub enum ModelMemberRole {
    Input,
    Output,
    Parameter,
    State,
    Component,
    Layer,
    Submodel,
    Operation,
    Configuration,
    Metadata,
    Requirement,
    Capability,
    Constraint,
    Preference,
    Binding,
    Region,

    /// Extension-defined model member role.
    Custom(QualifiedName),
}

// =============================================================================
// Model member payload
// =============================================================================

/// Semantic payload of an AI model member.
///
/// The payload describes source-level model intent. It does not describe
/// how that member is physically represented or executed.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ModelMemberPayload {
    /// A member with a canonical semantic type and optional initializer.
    Typed {
        ty: SemanticType,
        initializer: Option<ModelExpressionRef>,
    },

    /// A member referring to an existing semantic expression.
    Expression {
        expression: ModelExpressionRef,
    },

    /// A member referencing an existing model, function, operation,
    /// or other canonical semantic declaration.
    Reference {
        target: QualifiedName,
    },

    /// A nested structural region.
    ///
    /// Its children are represented as ordinary model members whose
    /// parent field identifies this member.
    Block,

    /// A connection between model endpoints.
    ///
    /// Connections are normally stored in ModelDefinition::connections.
    /// This variant is available for source adapters that preserve
    /// connection members in their original order.
    Connection {
        connection_index: usize,
    },
}

// =============================================================================
// Model members
// =============================================================================

/// A single resolved AI model member.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelMember {
    /// Stable identity within this model.
    pub id: ModelMemberId,

    /// Source-level member name.
    pub name: SemanticName,

    /// Resolved semantic member role.
    pub role: ModelMemberRole,

    /// Parent region, if this member is nested.
    pub parent: Option<ModelMemberId>,

    /// Source annotations, represented by canonical identities.
    pub annotations: Vec<ModelAnnotation>,

    /// Member semantic payload.
    pub payload: ModelMemberPayload,
}

impl ModelMember {
    /// Creates a model member.
    #[must_use]
    pub fn new(
        id: ModelMemberId,
        name: SemanticName,
        role: ModelMemberRole,
        payload: ModelMemberPayload,
    ) -> Self {
        Self {
            id,
            name,
            role,
            parent: None,
            annotations: Vec::new(),
            payload,
        }
    }

    /// Assigns a parent region.
    #[must_use]
    pub fn with_parent(mut self, parent: ModelMemberId) -> Self {
        self.parent = Some(parent);
        self
    }

    /// Adds a resolved annotation.
    #[must_use]
    pub fn with_annotation(mut self, annotation: ModelAnnotation) -> Self {
        self.annotations.push(annotation);
        self
    }

    /// Validates local member invariants.
    fn validate(&self) -> Result<(), ModelDefinitionError> {
        self.name
            .validate()
            .map_err(|_| ModelDefinitionError::InvalidMemberName)?;

        for annotation in &self.annotations {
            annotation.validate()?;
        }

        if let ModelMemberRole::Custom(name) = &self.role {
            name.validate()
                .map_err(|_| ModelDefinitionError::InvalidMemberRole)?;
        }

        if let ModelMemberPayload::Reference { target } = &self.payload {
            target
                .validate()
                .map_err(|_| ModelDefinitionError::InvalidMemberReference)?;
        }

        Ok(())
    }
}

// =============================================================================
// Model connection endpoints
// =============================================================================

/// A resolved endpoint of a model connection.
///
/// The frontend adapter resolves source-level qualified references into
/// member identities and port names.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelEndpoint {
    /// Referenced model member.
    pub member: ModelMemberId,

    /// Referenced input/output port.
    pub port: SemanticName,
}

impl ModelEndpoint {
    /// Creates an endpoint.
    #[must_use]
    pub fn new(member: ModelMemberId, port: SemanticName) -> Self {
        Self { member, port }
    }
}

/// A directed model data-flow connection.
///
/// This is a semantic relationship, not an executable graph IR.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelConnection {
    pub source: ModelEndpoint,
    pub destination: ModelEndpoint,
}

// =============================================================================
// Model definition
// =============================================================================

/// Canonical target-independent AI model definition.
///
/// This is the semantic representation of a resolved model declaration.
///
/// It describes what the model contains and how its members relate.
/// It does not describe physical execution.
///
/// All collections preserve source ordering unless their owning semantic
/// specification explicitly defines another ordering.
///
/// A vector's length is not a language-level capacity limit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelDefinition {
    /// Canonical declaration identity.
    pub identity: QualifiedName,

    /// Model declaration annotations.
    pub annotations: Vec<ModelAnnotation>,

    /// Generic parameters.
    pub generic_parameters: Vec<ModelGenericParameter>,

    /// Model inheritance references.
    pub extends: Vec<QualifiedName>,

    /// Implemented interfaces/contracts.
    pub implements: Vec<QualifiedName>,

    /// Model members in source order.
    pub members: Vec<ModelMember>,

    /// Directed model connections.
    pub connections: Vec<ModelConnection>,
}

impl ModelDefinition {
    /// Creates a model definition with no members.
    #[must_use]
    pub fn new(identity: QualifiedName) -> Self {
        Self {
            identity,
            annotations: Vec::new(),
            generic_parameters: Vec::new(),
            extends: Vec::new(),
            implements: Vec::new(),
            members: Vec::new(),
            connections: Vec::new(),
        }
    }

    /// Adds a declaration annotation.
    #[must_use]
    pub fn with_annotation(mut self, annotation: ModelAnnotation) -> Self {
        self.annotations.push(annotation);
        self
    }

    /// Adds a generic parameter.
    #[must_use]
    pub fn with_generic_parameter(
        mut self,
        parameter: ModelGenericParameter,
    ) -> Self {
        self.generic_parameters.push(parameter);
        self
    }

    /// Adds an inheritance reference.
    #[must_use]
    pub fn with_parent_model(mut self, parent: QualifiedName) -> Self {
        self.extends.push(parent);
        self
    }

    /// Adds an implemented interface or contract.
    #[must_use]
    pub fn with_interface(mut self, interface: QualifiedName) -> Self {
        self.implements.push(interface);
        self
    }

    /// Adds a member.
    #[must_use]
    pub fn with_member(mut self, member: ModelMember) -> Self {
        self.members.push(member);
        self
    }

    /// Adds a connection.
    #[must_use]
    pub fn with_connection(mut self, connection: ModelConnection) -> Self {
        self.connections.push(connection);
        self
    }

    /// Finds a member by its stable identity.
    ///
    /// This lookup is linear. Callers performing many lookups should build
    /// an index in their semantic session rather than storing a second
    /// competing member collection inside this definition.
    #[must_use]
    pub fn member(&self, id: &ModelMemberId) -> Option<&ModelMember> {
        self.members.iter().find(|member| &member.id == id)
    }

    /// Validates local structural invariants.
    ///
    /// This method does not establish:
    /// - Name resolution.
    /// - Type compatibility.
    /// - Generic-bound satisfaction.
    /// - Inheritance legality.
    /// - Interface implementation correctness.
    /// - Expression correctness.
    /// - Runtime feasibility.
    ///
    /// Those responsibilities remain with their canonical semantic owners.
    pub fn validate(&self) -> Result<(), ModelDefinitionError> {
        self.identity
            .validate()
            .map_err(|_| ModelDefinitionError::InvalidModelIdentity)?;

        for annotation in &self.annotations {
            annotation.validate()?;
        }

        self.validate_generic_parameters()?;
        self.validate_references()?;
        self.validate_members()?;
        self.validate_connections()?;

        Ok(())
    }

    fn validate_generic_parameters(
        &self,
    ) -> Result<(), ModelDefinitionError> {
        let mut names = BTreeSet::new();

        for parameter in &self.generic_parameters {
            parameter.validate()?;

            if !names.insert(parameter.parameter.name.clone()) {
                return Err(ModelDefinitionError::DuplicateGenericParameter);
            }
        }

        Ok(())
    }

    fn validate_references(&self) -> Result<(), ModelDefinitionError> {
        for parent in &self.extends {
            parent
                .validate()
                .map_err(|_| ModelDefinitionError::InvalidInheritanceReference)?;
        }

        for interface in &self.implements {
            interface
                .validate()
                .map_err(|_| ModelDefinitionError::InvalidInterfaceReference)?;
        }

        Ok(())
    }

    fn validate_members(&self) -> Result<(), ModelDefinitionError> {
        let mut identities: BTreeMap<ModelMemberId, &ModelMember> =
            BTreeMap::new();

        let mut scoped_names: BTreeSet<(
            Option<ModelMemberId>,
            SemanticName,
        )> = BTreeSet::new();

        // First pass: validate identities and local member structure.
        for member in &self.members {
            member.validate()?;

            if identities.insert(member.id.clone(), member).is_some() {
                return Err(ModelDefinitionError::DuplicateMemberIdentity);
            }

            if !scoped_names.insert((
                member.parent.clone(),
                member.name.clone(),
            )) {
                return Err(ModelDefinitionError::DuplicateMemberName);
            }
        }

        // Second pass: validate parent references.
        for member in &self.members {
            if let Some(parent_id) = &member.parent {
                let parent = identities
                    .get(parent_id)
                    .ok_or(ModelDefinitionError::UnknownParentRegion)?;

                if !matches!(
                    parent.payload,
                    ModelMemberPayload::Block
                ) {
                    return Err(
                        ModelDefinitionError::ParentIsNotARegion
                    );
                }
            }

            if matches!(member.payload, ModelMemberPayload::Block)
                && member.role != ModelMemberRole::Region
            {
                return Err(
                    ModelDefinitionError::BlockRequiresRegionRole
                );
            }

            if member.role == ModelMemberRole::Region
                && !matches!(member.payload, ModelMemberPayload::Block)
            {
                return Err(
                    ModelDefinitionError::RegionRequiresBlockPayload
                );
            }
        }

        Ok(())
    }

    fn validate_connections(&self) -> Result<(), ModelDefinitionError> {
        for connection in &self.connections {
            let source = self
                .member(&connection.source.member)
                .ok_or(ModelDefinitionError::UnknownConnectionSource)?;

            let destination = self
                .member(&connection.destination.member)
                .ok_or(ModelDefinitionError::UnknownConnectionDestination)?;

            connection
                .source
                .port
                .validate()
                .map_err(|_| ModelDefinitionError::InvalidConnectionPort)?;

            connection
                .destination
                .port
                .validate()
                .map_err(|_| ModelDefinitionError::InvalidConnectionPort)?;

            if source.id == destination.id
                && connection.source.port == connection.destination.port
            {
                return Err(ModelDefinitionError::SelfConnection);
            }
        }

        Ok(())
    }

    /// Returns the number of declared members.
    #[must_use]
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Returns the number of declared generic parameters.
    #[must_use]
    pub fn generic_parameter_count(&self) -> usize {
        self.generic_parameters.len()
    }

    /// Returns the number of declared connections.
    #[must_use]
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Structural errors produced by model definition construction or validation.
///
/// These errors are deliberately distinct from compiler semantic errors,
/// type errors, execution failures, and resource feasibility failures.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ModelDefinitionError {
    InvalidModelIdentity,
    InvalidMemberIdentity,
    InvalidExpressionReference,
    InvalidAnnotation,
    InvalidGenericParameter,
    InvalidGenericBound,
    DuplicateGenericParameter,
    InvalidInheritanceReference,
    InvalidInterfaceReference,
    InvalidMemberName,
    InvalidMemberRole,
    InvalidMemberReference,
    DuplicateMemberIdentity,
    DuplicateMemberName,
    UnknownParentRegion,
    ParentIsNotARegion,
    BlockRequiresRegionRole,
    RegionRequiresBlockPayload,
    UnknownConnectionSource,
    UnknownConnectionDestination,
    InvalidConnectionPort,
    SelfConnection,
}

impl fmt::Display for ModelDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidModelIdentity =>
                "invalid model identity",
            Self::InvalidMemberIdentity =>
                "invalid model member identity",
            Self::InvalidExpressionReference =>
                "invalid semantic expression reference",
            Self::InvalidAnnotation =>
                "invalid model annotation",
            Self::InvalidGenericParameter =>
                "invalid model generic parameter",
            Self::InvalidGenericBound =>
                "invalid generic trait bound",
            Self::DuplicateGenericParameter =>
                "duplicate generic parameter",
            Self::InvalidInheritanceReference =>
                "invalid inheritance reference",
            Self::InvalidInterfaceReference =>
                "invalid interface reference",
            Self::InvalidMemberName =>
                "invalid model member name",
            Self::InvalidMemberRole =>
                "invalid model member role",
            Self::InvalidMemberReference =>
                "invalid model member reference",
            Self::DuplicateMemberIdentity =>
                "duplicate model member identity",
            Self::DuplicateMemberName =>
                "duplicate member name in the same scope",
            Self::UnknownParentRegion =>
                "model member references an unknown parent region",
            Self::ParentIsNotARegion =>
                "model member parent is not a block region",
            Self::BlockRequiresRegionRole =>
                "block payload requires the region member role",
            Self::RegionRequiresBlockPayload =>
                "region member requires a block payload",
            Self::UnknownConnectionSource =>
                "connection references an unknown source member",
            Self::UnknownConnectionDestination =>
                "connection references an unknown destination member",
            Self::InvalidConnectionPort =>
                "invalid connection port name",
            Self::SelfConnection =>
                "connection cannot connect a port to itself",
        };

        f.write_str(message)
    }
}

impl std::error::Error for ModelDefinitionError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn name(value: &str) -> SemanticName {
        SemanticName::new(value)
    }

    fn qualified(value: &str) -> QualifiedName {
        QualifiedName::single(value)
    }

    fn member_id(value: &str) -> ModelMemberId {
        ModelMemberId::new(value).expect("valid member identity")
    }

    fn input(id: &str, member_name: &str) -> ModelMember {
        ModelMember::new(
            member_id(id),
            name(member_name),
            ModelMemberRole::Input,
            ModelMemberPayload::Typed {
                ty: SemanticType::Unit,
                initializer: None,
            },
        )
    }

    #[test]
    fn creates_valid_empty_model() {
        let model = ModelDefinition::new(qualified("Encoder"));

        assert!(model.validate().is_ok());
        assert_eq!(model.member_count(), 0);
    }

    #[test]
    fn accepts_multiple_members() {
        let model = ModelDefinition::new(qualified("Encoder"))
            .with_member(input("input", "features"))
            .with_member(input("output", "result"));

        assert!(model.validate().is_ok());
        assert_eq!(model.member_count(), 2);
    }

    #[test]
    fn rejects_duplicate_member_identity() {
        let model = ModelDefinition::new(qualified("Encoder"))
            .with_member(input("member", "first"))
            .with_member(input("member", "second"));

        assert_eq!(
            model.validate(),
            Err(ModelDefinitionError::DuplicateMemberIdentity)
        );
    }

    #[test]
    fn rejects_duplicate_names_in_same_scope() {
        let model = ModelDefinition::new(qualified("Encoder"))
            .with_member(input("first", "features"))
            .with_member(input("second", "features"));

        assert_eq!(
            model.validate(),
            Err(ModelDefinitionError::DuplicateMemberName)
        );
    }

    #[test]
    fn permits_same_name_in_distinct_regions() {
        let left = member_id("left");
        let right = member_id("right");

        let model = ModelDefinition::new(qualified("Encoder"))
            .with_member(ModelMember::new(
                left.clone(),
                name("left"),
                ModelMemberRole::Region,
                ModelMemberPayload::Block,
            ))
            .with_member(ModelMember::new(
                right.clone(),
                name("right"),
                ModelMemberRole::Region,
                ModelMemberPayload::Block,
            ))
            .with_member(
                input("left_input", "features").with_parent(left)
            )
            .with_member(
                input("right_input", "features").with_parent(right)
            );

        assert!(model.validate().is_ok());
    }

    #[test]
    fn rejects_unknown_parent_region() {
        let model = ModelDefinition::new(qualified("Encoder"))
            .with_member(
                input("input", "features")
                    .with_parent(member_id("missing"))
            );

        assert_eq!(
            model.validate(),
            Err(ModelDefinitionError::UnknownParentRegion)
        );
    }

    #[test]
    fn rejects_non_region_parent() {
        let parent = ModelMember::new(
            member_id("parent"),
            name("component"),
            ModelMemberRole::Component,
            ModelMemberPayload::Typed {
                ty: SemanticType::Unit,
                initializer: None,
            },
        );

        let child = input("child", "features")
            .with_parent(member_id("parent"));

        let model = ModelDefinition::new(qualified("Encoder"))
            .with_member(parent)
            .with_member(child);

        assert_eq!(
            model.validate(),
            Err(ModelDefinitionError::ParentIsNotARegion)
        );
    }

    #[test]
    fn validates_model_connections() {
        let model = ModelDefinition::new(qualified("Encoder"))
            .with_member(input("source", "encoder"))
            .with_member(input("destination", "classifier"))
            .with_connection(ModelConnection {
                source: ModelEndpoint::new(
                    member_id("source"),
                    name("output"),
                ),
                destination: ModelEndpoint::new(
                    member_id("destination"),
                    name("input"),
                ),
            });

        assert!(model.validate().is_ok());
        assert_eq!(model.connection_count(), 1);
    }

    #[test]
    fn rejects_unknown_connection_endpoint() {
        let model = ModelDefinition::new(qualified("Encoder"))
            .with_member(input("destination", "classifier"))
            .with_connection(ModelConnection {
                source: ModelEndpoint::new(
                    member_id("missing"),
                    name("output"),
                ),
                destination: ModelEndpoint::new(
                    member_id("destination"),
                    name("input"),
                ),
            });

        assert_eq!(
            model.validate(),
            Err(ModelDefinitionError::UnknownConnectionSource)
        );
    }

    #[test]
    fn rejects_self_connection() {
        let model = ModelDefinition::new(qualified("Encoder"))
            .with_member(input("node", "node"))
            .with_connection(ModelConnection {
                source: ModelEndpoint::new(
                    member_id("node"),
                    name("output"),
                ),
                destination: ModelEndpoint::new(
                    member_id("node"),
                    name("output"),
                ),
            });

        assert_eq!(
            model.validate(),
            Err(ModelDefinitionError::SelfConnection)
        );
    }

    #[test]
    fn preserves_inheritance_and_interfaces() {
        let model = ModelDefinition::new(qualified("Encoder"))
            .with_parent_model(qualified("BaseModel"))
            .with_interface(qualified("Trainable"));

        assert!(model.validate().is_ok());
        assert_eq!(model.extends.len(), 1);
        assert_eq!(model.implements.len(), 1);
    }

    #[test]
    fn supports_custom_member_roles() {
        let model = ModelDefinition::new(qualified("Encoder"))
            .with_member(ModelMember::new(
                member_id("custom"),
                name("custom_member"),
                ModelMemberRole::Custom(qualified("future.extension")),
                ModelMemberPayload::Expression {
                    expression: ModelExpressionRef::new(
                        "semantic-expression-1",
                    ).expect("valid expression reference"),
                },
            ));

        assert!(model.validate().is_ok());
    }

    #[test]
    fn rejects_invalid_member_identity() {
        assert_eq!(
            ModelMemberId::new(""),
            Err(ModelDefinitionError::InvalidMemberIdentity)
        );
    }

    #[test]
    fn rejects_invalid_expression_reference() {
        assert_eq!(
            ModelExpressionRef::new(""),
            Err(ModelDefinitionError::InvalidExpressionReference)
        );
    }

    #[test]
    fn preserves_member_source_order() {
        let model = ModelDefinition::new(qualified("Encoder"))
            .with_member(input("first", "first"))
            .with_member(input("second", "second"))
            .with_member(input("third", "third"));

        assert_eq!(model.members[0].name, name("first"));
        assert_eq!(model.members[1].name, name("second"));
        assert_eq!(model.members[2].name, name("third"));
    }
}
