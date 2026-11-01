
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! # Zamani AI Model State
//!
//! Path: src/ai/model/state.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define target-independent AI model state declarations, state schemas,
//! state transition contracts, runtime-state references, and checkpoint
//! schema compatibility.
//!
//! Model state is semantic information. It is not an allocated tensor,
//! runtime value, physical memory location, or executable IR operation.
//!
//! ## Architectural authorities
//!
//! Source grammar:
//!     grammar/ai/models.g4
//!
//! Canonical frontend AST:
//!     crate::frontend::ast
//!
//! Canonical semantic types:
//!     crate::frontend::semantic::types::ty
//!
//! Canonical AI model definitions:
//!     crate::ai::model::definition
//!
//! Canonical AI parameter definitions:
//!     crate::ai::model::parameters
//!
//! AI identity:
//!     crate::ai::ids
//!
//! AI schema version:
//!     crate::ai::version
//!
//! Universal computational representation:
//!     crate::zuir
//!
//! Runtime state realization:
//!     crate::runtime
//!
//! Distributed checkpointing:
//!     crate::distributed::resilience::checkpoint
//!
//! Distributed recovery:
//!     crate::distributed::resilience::recovery
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - AI model state schema descriptions.
//! - AI model state slot identities.
//! - State declaration metadata.
//! - State transition descriptions.
//! - Opaque runtime state references.
//! - Runtime state binding validation.
//! - Checkpoint state-schema descriptions.
//! - Checkpoint schema compatibility analysis.
//!
//! ## Does not own
//!
//! - Source parsing or AST construction.
//! - Semantic type definitions.
//! - Parameter declaration ownership.
//! - Tensor storage or materialization.
//! - Runtime state allocation.
//! - Runtime state mutation.
//! - Device memory.
//! - Hardware selection.
//! - Scheduling.
//! - Distributed communication.
//! - Checkpoint serialization.
//! - Checkpoint storage.
//! - Recovery execution.
//! - ZUIR operations or values.
//!
//! ## Scalability
//!
//! No language-level maximum is imposed on:
//!
//! - Model state slots.
//! - State transitions.
//! - State references.
//! - Model state size.
//! - Tensor dimensions.
//! - Tensor rank.
//! - Model count.
//! - Execution devices.
//! - Distributed participants.
//!
//! Collections grow according to available resources.
//!
//! Operational budgets must be supplied by the relevant execution,
//! compiler, or deployment infrastructure.
//!
//! Literal infinite execution is not possible. The architecture instead
//! avoids artificial language-level machine-capacity ceilings.
//!
//! ## Checkpointing
//!
//! This module validates logical state-schema compatibility.
//!
//! It does not serialize or persist state values.
//!
//! Actual checkpoint capture and persistence belong to runtime and
//! distributed resilience infrastructure.
//!
//! ## Integration
//!
//! ModelDefinition supplies the canonical model declaration.
//!
//! ModelMemberRole::State identifies state declarations.
//!
//! ModelMemberPayload::Typed supplies canonical semantic types.
//!
//! ModelExpressionRef supplies canonical expression references.
//!
//! Runtime adapters associate state slots with runtime-managed handles.
//!
//! Distributed checkpoint providers persist the actual state payload.
//!
//! ## Freeze criteria
//!
//! - Existing model definitions are reused.
//! - Existing semantic types are reused.
//! - State member identities remain canonical.
//! - Duplicate state identities are rejected.
//! - Invalid state transitions are rejected.
//! - Invalid runtime bindings are rejected.
//! - Checkpoint schema mismatches are explicit.
//! - No tensor storage is duplicated.
//! - No runtime allocation is introduced.
//! - No hardware ceilings are introduced.
//! - Rust 1.97+, edition 2021.
//! - Safe Rust only.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::ai::ids::AiModelId;

use crate::ai::model::definition::{
    ModelDefinition,
    ModelExpressionRef,
    ModelMemberId,
    ModelMemberPayload,
    ModelMemberRole,
};

use crate::ai::version::AiModelSchemaVersion;

use crate::frontend::semantic::types::ty::{
    QualifiedName,
    SemanticName,
    SemanticType,
};

// =============================================================================
// State slot
// =============================================================================

/// A logical state slot declared by an AI model.
///
/// A state slot references an existing model member.
///
/// It does not own the actual state value.
///
/// The canonical semantic type describes the logical value independently
/// of its eventual physical representation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModelStateSlot {
    member_id: ModelMemberId,
    name: SemanticName,
    ty: SemanticType,
    parent: Option<ModelMemberId>,
    initializer: Option<ModelExpressionRef>,
}

impl ModelStateSlot {
    /// Construct a model state slot.
    ///
    /// The member identity must originate from the canonical model
    /// definition or its semantic adapter.
    pub fn new(
        member_id: ModelMemberId,
        name: SemanticName,
        ty: SemanticType,
        parent: Option<ModelMemberId>,
        initializer: Option<ModelExpressionRef>,
    ) -> Result<Self, ModelStateError> {
        name.validate().map_err(|error| {
            ModelStateError::InvalidStateName(
                error.to_string(),
            )
        })?;

        Ok(Self {
            member_id,
            name,
            ty,
            parent,
            initializer,
        })
    }

    /// Return the canonical model member identity.
    #[must_use]
    pub fn member_id(&self) -> &ModelMemberId {
        &self.member_id
    }

    /// Return the semantic state name.
    #[must_use]
    pub fn name(&self) -> &SemanticName {
        &self.name
    }

    /// Return the canonical semantic type.
    #[must_use]
    pub fn ty(&self) -> &SemanticType {
        &self.ty
    }

    /// Return the parent model region.
    #[must_use]
    pub fn parent(&self) -> Option<&ModelMemberId> {
        self.parent.as_ref()
    }

    /// Return the optional initializer expression reference.
    #[must_use]
    pub fn initializer(&self) -> Option<&ModelExpressionRef> {
        self.initializer.as_ref()
    }

    /// Validate the local state-slot invariants.
    pub fn validate(&self) -> Result<(), ModelStateError> {
        self.name.validate().map_err(|error| {
            ModelStateError::InvalidStateName(
                error.to_string(),
            )
        })
    }
}

// =============================================================================
// Model state schema
// =============================================================================

/// Canonical logical state schema for an AI model.
///
/// The schema references existing model declarations.
///
/// It does not create a second model definition.
///
/// State slots preserve declaration order.
///
/// A derived index provides identity-based lookup without changing
/// declaration ordering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModelStateSchema {
    model_id: AiModelId,
    model_identity: QualifiedName,
    schema_version: AiModelSchemaVersion,

    slots: Vec<ModelStateSlot>,

    by_id: BTreeMap<ModelMemberId, usize>,
}

impl ModelStateSchema {
    /// Construct a state schema from existing state slots.
    ///
    /// Duplicate member identities are rejected.
    pub fn new(
        model_id: AiModelId,
        model_identity: QualifiedName,
        schema_version: AiModelSchemaVersion,
        slots: Vec<ModelStateSlot>,
    ) -> Result<Self, ModelStateError> {
        model_identity.validate().map_err(|error| {
            ModelStateError::InvalidModelIdentity(
                error.to_string(),
            )
        })?;

        let mut by_id = BTreeMap::new();

        for (index, slot) in slots.iter().enumerate() {
            slot.validate()?;

            if by_id
                .insert(slot.member_id.clone(), index)
                .is_some()
            {
                return Err(
                    ModelStateError::DuplicateStateIdentity,
                );
            }
        }

        Ok(Self {
            model_id,
            model_identity,
            schema_version,
            slots,
            by_id,
        })
    }

    /// Construct a state schema from an existing model definition.
    ///
    /// Only members declared with ModelMemberRole::State are included.
    ///
    /// State members must contain typed declarations.
    ///
    /// The original ModelDefinition remains the authority for model
    /// structure and member identities.
    pub fn from_model(
        model_id: AiModelId,
        schema_version: AiModelSchemaVersion,
        definition: &ModelDefinition,
    ) -> Result<Self, ModelStateError> {
        definition.validate().map_err(|error| {
            ModelStateError::InvalidModelDefinition(
                error.to_string(),
            )
        })?;

        let mut slots = Vec::new();

        for member in &definition.members {
            if !matches!(
                &member.role,
                ModelMemberRole::State
            ) {
                continue;
            }

            let (ty, initializer) = match &member.payload {
                ModelMemberPayload::Typed {
                    ty,
                    initializer,
                } => (
                    ty.clone(),
                    initializer.clone(),
                ),

                _ => {
                    return Err(
                        ModelStateError::StateMustBeTyped {
                            member: member.id.clone(),
                        },
                    );
                }
            };

            slots.push(ModelStateSlot::new(
                member.id.clone(),
                member.name.clone(),
                ty,
                member.parent.clone(),
                initializer,
            )?);
        }

        Self::new(
            model_id,
            definition.identity.clone(),
            schema_version,
            slots,
        )
    }

    /// Return the model identity.
    #[must_use]
    pub const fn model_id(&self) -> AiModelId {
        self.model_id
    }

    /// Return the canonical model declaration identity.
    #[must_use]
    pub fn model_identity(&self) -> &QualifiedName {
        &self.model_identity
    }

    /// Return the AI model schema version.
    #[must_use]
    pub fn schema_version(&self) -> &AiModelSchemaVersion {
        &self.schema_version
    }

    /// Return state slots in declaration order.
    #[must_use]
    pub fn slots(&self) -> &[ModelStateSlot] {
        &self.slots
    }

    /// Return the number of declared state slots.
    #[must_use]
    pub fn len(&self) -> usize {
        self.slots.len()
    }

    /// Return whether the model declares any state slots.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    /// Find a state slot by its canonical member identity.
    #[must_use]
    pub fn get(
        &self,
        member_id: &ModelMemberId,
    ) -> Option<&ModelStateSlot> {
        self.by_id
            .get(member_id)
            .map(|index| &self.slots[*index])
    }

    /// Iterate over state slots in declaration order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = &ModelStateSlot> {
        self.slots.iter()
    }

    /// Validate schema structure and its derived index.
    pub fn validate(&self) -> Result<(), ModelStateError> {
        self.model_identity.validate().map_err(|error| {
            ModelStateError::InvalidModelIdentity(
                error.to_string(),
            )
        })?;

        if self.by_id.len() != self.slots.len() {
            return Err(
                ModelStateError::IndexInconsistency,
            );
        }

        for (index, slot) in self.slots.iter().enumerate() {
            slot.validate()?;

            if self.by_id.get(&slot.member_id) != Some(&index) {
                return Err(
                    ModelStateError::IndexInconsistency,
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// State transitions
// =============================================================================

/// Declarative state transition.
///
/// A transition references existing model state slots and a canonical
/// expression describing the update.
///
/// This structure does not evaluate the expression or mutate state.
///
/// Effect checking and expression type checking belong to the canonical
/// semantic infrastructure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModelStateTransition {
    source: ModelMemberId,
    destination: ModelMemberId,
    expression: ModelExpressionRef,
}

impl ModelStateTransition {
    /// Construct a state transition description.
    #[must_use]
    pub fn new(
        source: ModelMemberId,
        destination: ModelMemberId,
        expression: ModelExpressionRef,
    ) -> Self {
        Self {
            source,
            destination,
            expression,
        }
    }

    /// Return the source state slot.
    #[must_use]
    pub fn source(&self) -> &ModelMemberId {
        &self.source
    }

    /// Return the destination state slot.
    #[must_use]
    pub fn destination(&self) -> &ModelMemberId {
        &self.destination
    }

    /// Return the canonical update expression reference.
    #[must_use]
    pub fn expression(&self) -> &ModelExpressionRef {
        &self.expression
    }

    /// Validate the transition against a model state schema.
    pub fn validate(
        &self,
        schema: &ModelStateSchema,
    ) -> Result<(), ModelStateError> {
        if schema.get(&self.source).is_none() {
            return Err(
                ModelStateError::UnknownTransitionSource,
            );
        }

        if schema.get(&self.destination).is_none() {
            return Err(
                ModelStateError::UnknownTransitionDestination,
            );
        }

        Ok(())
    }
}

// =============================================================================
// Complete model state definition
// =============================================================================

/// Complete semantic state definition for an AI model.
///
/// Combines the state schema and declarative transition contracts.
///
/// Runtime state values remain outside this structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModelStateDefinition {
    schema: ModelStateSchema,
    transitions: Vec<ModelStateTransition>,
}

impl ModelStateDefinition {
    /// Construct and validate a complete model state definition.
    pub fn new(
        schema: ModelStateSchema,
        transitions: Vec<ModelStateTransition>,
    ) -> Result<Self, ModelStateError> {
        schema.validate()?;

        for transition in &transitions {
            transition.validate(&schema)?;
        }

        Ok(Self {
            schema,
            transitions,
        })
    }

    /// Return the state schema.
    #[must_use]
    pub fn schema(&self) -> &ModelStateSchema {
        &self.schema
    }

    /// Return declarative state transitions.
    #[must_use]
    pub fn transitions(&self) -> &[ModelStateTransition] {
        &self.transitions
    }

    /// Validate the complete state definition.
    pub fn validate(&self) -> Result<(), ModelStateError> {
        self.schema.validate()?;

        for transition in &self.transitions {
            transition.validate(&self.schema)?;
        }

        Ok(())
    }
}

// =============================================================================
// Runtime state references
// =============================================================================

/// Opaque reference to a runtime-managed state value.
///
/// This is a reference contract, not the actual runtime value.
///
/// The provider and handle are supplied by the runtime integration.
///
/// The AI subsystem does not interpret physical storage addresses,
/// device pointers, memory offsets, or backend-specific handles.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuntimeStateReference {
    model_id: AiModelId,
    slot_id: ModelMemberId,
    provider: String,
    handle: String,
}

impl RuntimeStateReference {
    /// Construct a runtime state reference.
    pub fn new(
        model_id: AiModelId,
        slot_id: ModelMemberId,
        provider: impl Into<String>,
        handle: impl Into<String>,
    ) -> Result<Self, ModelStateError> {
        let provider = provider.into();
        let handle = handle.into();

        validate_reference_text(
            "runtime state provider",
            &provider,
        )?;

        validate_reference_text(
            "runtime state handle",
            &handle,
        )?;

        Ok(Self {
            model_id,
            slot_id,
            provider,
            handle,
        })
    }

    /// Return the associated model identity.
    #[must_use]
    pub const fn model_id(&self) -> AiModelId {
        self.model_id
    }

    /// Return the associated model state slot.
    #[must_use]
    pub fn slot_id(&self) -> &ModelMemberId {
        &self.slot_id
    }

    /// Return the runtime provider identity.
    #[must_use]
    pub fn provider(&self) -> &str {
        &self.provider
    }

    /// Return the opaque runtime handle.
    #[must_use]
    pub fn handle(&self) -> &str {
        &self.handle
    }
}

// =============================================================================
// Runtime state bindings
// =============================================================================

/// Validated associations between model state slots and runtime references.
///
/// This collection does not own the runtime state values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeStateBindings {
    model_id: AiModelId,
    bindings: BTreeMap<
        ModelMemberId,
        RuntimeStateReference,
    >,
}

impl RuntimeStateBindings {
    /// Construct and validate runtime state bindings.
    ///
    /// Every reference must belong to the supplied model and reference
    /// an existing state slot.
    ///
    /// A state slot can have only one binding in this collection.
    pub fn new(
        schema: &ModelStateSchema,
        references: Vec<RuntimeStateReference>,
    ) -> Result<Self, ModelStateError> {
        let mut bindings = BTreeMap::new();

        for reference in references {
            if reference.model_id != schema.model_id {
                return Err(
                    ModelStateError::RuntimeModelMismatch,
                );
            }

            if schema.get(&reference.slot_id).is_none() {
                return Err(
                    ModelStateError::UnknownRuntimeStateSlot,
                );
            }

            let slot_id = reference.slot_id.clone();

            if bindings.insert(slot_id, reference).is_some() {
                return Err(
                    ModelStateError::DuplicateRuntimeBinding,
                );
            }
        }

        Ok(Self {
            model_id: schema.model_id,
            bindings,
        })
    }

    /// Return the model identity.
    #[must_use]
    pub const fn model_id(&self) -> AiModelId {
        self.model_id
    }

    /// Find a runtime reference for a state slot.
    #[must_use]
    pub fn get(
        &self,
        slot_id: &ModelMemberId,
    ) -> Option<&RuntimeStateReference> {
        self.bindings.get(slot_id)
    }

    /// Return the number of runtime bindings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Return whether there are no runtime bindings.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// Iterate over runtime bindings in deterministic identity order.
    pub fn iter(
        &self,
    ) -> impl Iterator<
        Item = (&ModelMemberId, &RuntimeStateReference),
    > {
        self.bindings.iter()
    }

    /// Validate bindings against a model state schema.
    pub fn validate(
        &self,
        schema: &ModelStateSchema,
    ) -> Result<(), ModelStateError> {
        if self.model_id != schema.model_id {
            return Err(
                ModelStateError::RuntimeModelMismatch,
            );
        }

        for (slot_id, reference) in &self.bindings {
            if schema.get(slot_id).is_none() {
                return Err(
                    ModelStateError::UnknownRuntimeStateSlot,
                );
            }

            if reference.slot_id != *slot_id
                || reference.model_id != self.model_id
            {
                return Err(
                    ModelStateError::RuntimeBindingInconsistency,
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Checkpoint state schema
// =============================================================================

/// Immutable logical state-schema description for checkpoint compatibility.
///
/// This contains schema metadata, not state payloads.
///
/// The runtime and distributed checkpoint provider remain responsible
/// for capturing and persisting the actual values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateCheckpointSchema {
    schema: ModelStateSchema,
}

impl StateCheckpointSchema {
    /// Create a checkpoint schema description from a validated model schema.
    pub fn from_schema(
        schema: &ModelStateSchema,
    ) -> Result<Self, ModelStateError> {
        schema.validate()?;

        Ok(Self {
            schema: schema.clone(),
        })
    }

    /// Return the checkpoint model identity.
    #[must_use]
    pub const fn model_id(&self) -> AiModelId {
        self.schema.model_id
    }

    /// Return the checkpoint state schema.
    #[must_use]
    pub fn schema(&self) -> &ModelStateSchema {
        &self.schema
    }

    /// Compare this checkpoint schema with a current model state schema.
    #[must_use]
    pub fn compare(
        &self,
        current: &ModelStateSchema,
    ) -> CheckpointCompatibilityReport {
        compare_state_schemas(
            current,
            &self.schema,
        )
    }
}

// =============================================================================
// Checkpoint compatibility
// =============================================================================

/// A specific reason that a checkpoint state schema is incompatible.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum StateSchemaDifference {
    /// The model identities differ.
    ModelIdentityMismatch,

    /// The model declaration identities differ.
    ModelDeclarationMismatch,

    /// The model schema versions differ.
    SchemaVersionMismatch,

    /// A state slot exists in the current model but not in the checkpoint.
    AddedStateSlot {
        slot: ModelMemberId,
    },

    /// A checkpoint contains a state slot absent from the current model.
    RemovedStateSlot {
        slot: ModelMemberId,
    },

    /// A state slot has a different semantic type.
    StateTypeChanged {
        slot: ModelMemberId,
    },

    /// A state slot has a different semantic name.
    StateNameChanged {
        slot: ModelMemberId,
    },

    /// A state slot has a different parent region.
    StateParentChanged {
        slot: ModelMemberId,
    },
}

/// Result of logical checkpoint state-schema compatibility analysis.
///
/// Compatibility is deliberately strict.
///
/// A differing schema is not automatically compatible merely because
/// its model names or parameter counts match.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckpointCompatibilityReport {
    compatible: bool,
    differences: Vec<StateSchemaDifference>,
}

impl CheckpointCompatibilityReport {
    /// Return whether the checkpoint schema is exactly compatible.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        self.compatible
    }

    /// Return all detected incompatibilities.
    #[must_use]
    pub fn differences(&self) -> &[StateSchemaDifference] {
        &self.differences
    }
}

/// Compare two logical state schemas.
///
/// This comparison does not deserialize or inspect checkpoint payloads.
///
/// Exact schema compatibility is required. Migration policies must be
/// implemented by a separate, explicitly versioned migration service.
pub fn compare_state_schemas(
    current: &ModelStateSchema,
    checkpoint: &ModelStateSchema,
) -> CheckpointCompatibilityReport {
    let mut differences = Vec::new();

    if current.model_id != checkpoint.model_id {
        differences.push(
            StateSchemaDifference::ModelIdentityMismatch,
        );
    }

    if current.model_identity != checkpoint.model_identity {
        differences.push(
            StateSchemaDifference::ModelDeclarationMismatch,
        );
    }

    if current.schema_version != checkpoint.schema_version {
        differences.push(
            StateSchemaDifference::SchemaVersionMismatch,
        );
    }

    for current_slot in &current.slots {
        match checkpoint.get(&current_slot.member_id) {
            None => {
                differences.push(
                    StateSchemaDifference::AddedStateSlot {
                        slot: current_slot.member_id.clone(),
                    },
                );
            }

            Some(checkpoint_slot) => {
                if current_slot.ty != checkpoint_slot.ty {
                    differences.push(
                        StateSchemaDifference::StateTypeChanged {
                            slot: current_slot.member_id.clone(),
                        },
                    );
                }

                if current_slot.name != checkpoint_slot.name {
                    differences.push(
                        StateSchemaDifference::StateNameChanged {
                            slot: current_slot.member_id.clone(),
                        },
                    );
                }

                if current_slot.parent != checkpoint_slot.parent {
                    differences.push(
                        StateSchemaDifference::StateParentChanged {
                            slot: current_slot.member_id.clone(),
                        },
                    );
                }
            }
        }
    }

    for checkpoint_slot in &checkpoint.slots {
        if current.get(&checkpoint_slot.member_id).is_none() {
            differences.push(
                StateSchemaDifference::RemovedStateSlot {
                    slot: checkpoint_slot.member_id.clone(),
                },
            );
        }
    }

    CheckpointCompatibilityReport {
        compatible: differences.is_empty(),
        differences,
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by AI model state validation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ModelStateError {
    /// The model declaration is structurally invalid.
    InvalidModelDefinition(String),

    /// The model identity is invalid.
    InvalidModelIdentity(String),

    /// A state member name is invalid.
    InvalidStateName(String),

    /// A duplicate state identity was encountered.
    DuplicateStateIdentity,

    /// A model state member does not have a typed declaration.
    StateMustBeTyped {
        member: ModelMemberId,
    },

    /// A state transition references an unknown source.
    UnknownTransitionSource,

    /// A state transition references an unknown destination.
    UnknownTransitionDestination,

    /// A runtime reference belongs to a different model.
    RuntimeModelMismatch,

    /// A runtime reference identifies an unknown state slot.
    UnknownRuntimeStateSlot,

    /// More than one runtime reference targets a state slot.
    DuplicateRuntimeBinding,

    /// A runtime binding is internally inconsistent.
    RuntimeBindingInconsistency,

    /// An identifying runtime reference is empty or malformed.
    InvalidReference {
        field: &'static str,
    },

    /// A derived index does not match its underlying collection.
    IndexInconsistency,
}

impl fmt::Display for ModelStateError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidModelDefinition(message) => {
                write!(
                    formatter,
                    "invalid AI model definition: {message}"
                )
            }

            Self::InvalidModelIdentity(message) => {
                write!(
                    formatter,
                    "invalid AI model identity: {message}"
                )
            }

            Self::InvalidStateName(message) => {
                write!(
                    formatter,
                    "invalid AI model state name: {message}"
                )
            }

            Self::DuplicateStateIdentity => {
                write!(
                    formatter,
                    "duplicate AI model state identity"
                )
            }

            Self::StateMustBeTyped { member } => {
                write!(
                    formatter,
                    "AI model state member {member:?} must have a typed declaration"
                )
            }

            Self::UnknownTransitionSource => {
                write!(
                    formatter,
                    "state transition references an unknown source slot"
                )
            }

            Self::UnknownTransitionDestination => {
                write!(
                    formatter,
                    "state transition references an unknown destination slot"
                )
            }

            Self::RuntimeModelMismatch => {
                write!(
                    formatter,
                    "runtime state reference belongs to another model"
                )
            }

            Self::UnknownRuntimeStateSlot => {
                write!(
                    formatter,
                    "runtime reference targets an unknown model state slot"
                )
            }

            Self::DuplicateRuntimeBinding => {
                write!(
                    formatter,
                    "duplicate runtime binding for an AI state slot"
                )
            }

            Self::RuntimeBindingInconsistency => {
                write!(
                    formatter,
                    "runtime state binding is internally inconsistent"
                )
            }

            Self::InvalidReference { field } => {
                write!(
                    formatter,
                    "invalid or empty {field}"
                )
            }

            Self::IndexInconsistency => {
                write!(
                    formatter,
                    "AI model state index is inconsistent"
                )
            }
        }
    }
}

impl Error for ModelStateError {}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_reference_text(
    field: &'static str,
    value: &str,
) -> Result<(), ModelStateError> {
    if value.trim().is_empty()
        || value.chars().any(char::is_control)
    {
        return Err(
            ModelStateError::InvalidReference {
                field,
            },
        );
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn model_id() -> AiModelId {
        AiModelId::new(1).expect("valid model identity")
    }

    fn slot_id() -> ModelMemberId {
        ModelMemberId::new("model.state.counter")
            .expect("valid state member identity")
    }

    #[test]
    fn runtime_reference_rejects_empty_provider() {
        let result = RuntimeStateReference::new(
            model_id(),
            slot_id(),
            "",
            "runtime.handle",
        );

        assert!(matches!(
            result,
            Err(ModelStateError::InvalidReference {
                field: "runtime state provider"
            })
        ));
    }

    #[test]
    fn runtime_reference_rejects_empty_handle() {
        let result = RuntimeStateReference::new(
            model_id(),
            slot_id(),
            "runtime.provider",
            "",
        );

        assert!(matches!(
            result,
            Err(ModelStateError::InvalidReference {
                field: "runtime state handle"
            })
        ));
    }

    #[test]
    fn runtime_reference_preserves_opaque_handle() {
        let reference = RuntimeStateReference::new(
            model_id(),
            slot_id(),
            "runtime.provider",
            "opaque://state/123",
        )
        .expect("valid runtime reference");

        assert_eq!(
            reference.provider(),
            "runtime.provider"
        );

        assert_eq!(
            reference.handle(),
            "opaque://state/123"
        );

        assert_eq!(
            reference.model_id(),
            model_id()
        );
    }

    #[test]
    fn runtime_reference_rejects_control_characters() {
        let result = RuntimeStateReference::new(
            model_id(),
            slot_id(),
            "runtime\nprovider",
            "state.handle",
        );

        assert!(result.is_err());
    }

    #[test]
    fn state_transition_preserves_canonical_expression_reference() {
        let expression = ModelExpressionRef::new(
            "semantic.expression.update_counter",
        )
        .expect("valid expression reference");

        let source = slot_id();

        let transition = ModelStateTransition::new(
            source.clone(),
            source.clone(),
            expression.clone(),
        );

        assert_eq!(
            transition.source(),
            &source
        );

        assert_eq!(
            transition.destination(),
            &source
        );

        assert_eq!(
            transition.expression(),
            &expression
        );
    }

    #[test]
    fn checkpoint_report_exposes_incompatibilities() {
        let report = CheckpointCompatibilityReport {
            compatible: false,
            differences: vec![
                StateSchemaDifference::SchemaVersionMismatch,
            ],
        };

        assert!(!report.is_compatible());

        assert_eq!(
            report.differences().len(),
            1
        );
    }

    #[test]
    fn checkpoint_report_can_represent_exact_compatibility() {
        let report = CheckpointCompatibilityReport {
            compatible: true,
            differences: Vec::new(),
        };

        assert!(report.is_compatible());

        assert!(report.differences().is_empty());
    }
}
