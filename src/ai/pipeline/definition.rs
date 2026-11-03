
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI — Pipeline Definitions
//!
//! Path: `src/ai/pipeline/definition.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines the target-independent, declarative definition of an AI pipeline.
//!
//! A pipeline definition describes:
//!
//! - pipeline identity and name;
//! - pipeline version;
//! - the declared stages;
//! - stage operation identities;
//! - stage input and output ports;
//! - data-contract references;
//! - optional stage metadata;
//! - pipeline-level metadata.
//!
//! This module describes computational intent. It does not execute stages.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - AI pipeline definition data;
//! - stage declaration data;
//! - stage port declaration data;
//! - pipeline definition construction;
//! - immutable pipeline definition access;
//! - local declaration invariants;
//! - stable declaration ordering.
//!
//! It does not own:
//!
//! - pipeline graph algorithms;
//! - dependency analysis;
//! - cycle detection;
//! - pipeline validation coordination;
//! - execution plans;
//! - scheduling;
//! - hardware selection;
//! - resource allocation;
//! - model execution;
//! - distributed execution;
//! - source AST construction;
//! - semantic type checking;
//! - universal IR definitions.
//!
//! ## Integration
//!
//! `crate::ai::ids::AiPipelineId`
//!     Canonical AI pipeline identity.
//!
//! `crate::ai::pipeline::graph`
//!     Consumes stage declarations and establishes graph relationships.
//!
//! `crate::ai::pipeline::dependencies`
//!     Analyzes declared data and control dependencies.
//!
//! `crate::ai::pipeline::validation`
//!     Coordinates complete pipeline validation.
//!
//! `crate::ai::pipeline::execution_plan`
//!     Produces an abstract execution plan from a validated pipeline.
//!
//! `crate::ai::pipeline::transformations`
//!     Defines target-independent transformations.
//!
//! `crate::frontend::semantic::domains::ai`
//!     Remains the AI semantic authority.
//!
//! `crate::zuir::dialects::ai`
//!     Owns universal AI operation representations.
//!
//! `crate::distributed`
//!     Owns distributed placement, scheduling, and coordination.
//!
//! `crate::runtime`
//!     Owns execution.
//!
//! ## Scalability
//!
//! No language-level maximum is imposed on:
//!
//! - pipeline stages;
//! - pipeline inputs;
//! - pipeline outputs;
//! - metadata entries;
//! - stage dependencies;
//! - model sizes;
//! - tensor dimensions;
//! - distributed participants.
//!
//! The collections used here are dynamically sized Rust collections.
//!
//! This does not promise literal infinite storage. Actual memory, address
//! space, allocator capacity, and operational budgets remain finite.
//!
//! A capacity or allocation failure must be reported rather than silently
//! changing the definition.
//!
//! ## POCO-REAF
//!
//! A pipeline definition contains no physical device identity, hardware
//! address, accelerator count, worker count, or vendor-specific operation.
//!
//! Its meaning is independent of the machine that eventually realizes it.
//!
//! A target that cannot satisfy the pipeline's semantic requirements must
//! report incompatibility or infeasibility downstream.
//!
//! ## Determinism
//!
//! Stage declaration order is preserved.
//!
//! Metadata is stored in ordered maps.
//!
//! This module does not claim that stage declaration order is execution order.
//!
//! ## Freeze criteria
//!
//! - Pipeline identity uses the canonical AI identifier.
//! - No competing pipeline ID system is introduced.
//! - Stage and port declarations are validated.
//! - Definition access is immutable.
//! - No scheduler or execution engine is introduced.
//! - No arbitrary semantic capacity ceiling is imposed.
//! - Construction and error behavior are tested.
//! - Safe Rust only.

use crate::ai::ids::AiPipelineId;

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

/// A pipeline definition schema version.
///
/// This version describes the in-memory definition contract. It is not the
/// Zamani language version, semantic version, ZUIR dialect version, or model
/// artifact version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineDefinitionVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl PipelineDefinitionVersion {
    /// Construct a definition version.
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Return the major version.
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Return the minor version.
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Return the patch version.
    pub const fn patch(self) -> u32 {
        self.patch
    }
}

impl Default for PipelineDefinitionVersion {
    fn default() -> Self {
        Self::new(1, 0, 0)
    }
}

/// A stable, opaque stage identity within a pipeline definition.
///
/// Stage IDs are local to the owning pipeline definition. They are not
/// universal ZUIR operation IDs, source AST node IDs, or runtime task IDs.
///
/// The stage ID is intentionally represented as an owned string so that
/// pipeline definitions can preserve source-level names and extension
/// identifiers without introducing another global identifier authority.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineStageId(String);

impl PipelineStageId {
    /// Construct a stage ID.
    ///
    /// Empty values and control characters are rejected. Whitespace is
    /// preserved rather than silently normalized.
    pub fn new(value: impl Into<String>) -> Result<Self, PipelineDefinitionError> {
        let value = value.into();

        validate_identifier("stage ID", &value)?;

        Ok(Self(value))
    }

    /// Return the stage ID as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PipelineStageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A qualified operation identity.
///
/// The identity is resolved through the AI operation registry or the
/// appropriate canonical IR dialect registry.
///
/// This type does not define operation semantics.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineOperationRef {
    namespace: String,
    name: String,
}

impl PipelineOperationRef {
    /// Construct an operation reference.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, PipelineDefinitionError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_identifier("operation namespace", &namespace)?;
        validate_identifier("operation name", &name)?;

        Ok(Self {
            namespace,
            name,
        })
    }

    /// Return the operation namespace.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the operation name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// A declared pipeline port direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PipelinePortDirection {
    /// A value consumed by a stage.
    Input,

    /// A value produced by a stage.
    Output,
}

/// A stage port declaration.
///
/// `contract` is an opaque reference to the canonical semantic contract,
/// schema, or type identity associated with the port.
///
/// This module deliberately does not create a competing type system.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelinePort {
    name: String,
    direction: PipelinePortDirection,
    contract: Option<String>,
}

impl PipelinePort {
    /// Construct a port declaration.
    pub fn new(
        name: impl Into<String>,
        direction: PipelinePortDirection,
        contract: Option<String>,
    ) -> Result<Self, PipelineDefinitionError> {
        let name = name.into();

        validate_identifier("port name", &name)?;

        if let Some(contract) = contract.as_deref() {
            validate_identifier("port contract", contract)?;
        }

        Ok(Self {
            name,
            direction,
            contract,
        })
    }

    /// Return the port name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the port direction.
    pub const fn direction(&self) -> PipelinePortDirection {
        self.direction
    }

    /// Return the optional canonical contract reference.
    pub fn contract(&self) -> Option<&str> {
        self.contract.as_deref()
    }
}

/// Declarative metadata associated with a pipeline stage.
///
/// Metadata is descriptive. It must not be used as an implicit hardware
/// selection mechanism or as a substitute for semantic requirements.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PipelineStageMetadata {
    entries: BTreeMap<String, String>,
}

impl PipelineStageMetadata {
    /// Construct empty stage metadata.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a metadata entry.
    ///
    /// Replacing an existing key is explicit through the returned value.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, PipelineDefinitionError> {
        let key = key.into();
        let value = value.into();

        validate_identifier("metadata key", &key)?;

        Ok(self.entries.insert(key, value))
    }

    /// Return a metadata value.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    /// Return the metadata entries.
    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.entries
    }

    /// Return whether the metadata is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// A declarative AI pipeline stage.
///
/// A stage identifies an operation and declares its data interface.
///
/// It does not contain an execution closure, runtime handle, device pointer,
/// scheduler task, or backend-specific implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PipelineStageDefinition {
    id: PipelineStageId,
    operation: PipelineOperationRef,
    ports: Vec<PipelinePort>,
    metadata: PipelineStageMetadata,
}

impl PipelineStageDefinition {
    /// Construct a stage definition.
    pub fn new(
        id: PipelineStageId,
        operation: PipelineOperationRef,
    ) -> Self {
        Self {
            id,
            operation,
            ports: Vec::new(),
            metadata: PipelineStageMetadata::new(),
        }
    }

    /// Return the stage identity.
    pub fn id(&self) -> &PipelineStageId {
        &self.id
    }

    /// Return the referenced operation.
    pub fn operation(&self) -> &PipelineOperationRef {
        &self.operation
    }

    /// Return all declared ports in declaration order.
    pub fn ports(&self) -> &[PipelinePort] {
        &self.ports
    }

    /// Return stage metadata.
    pub fn metadata(&self) -> &PipelineStageMetadata {
        &self.metadata
    }

    /// Add a port declaration.
    ///
    /// Port names must be unique within this stage, independent of direction.
    pub fn add_port(
        &mut self,
        port: PipelinePort,
    ) -> Result<(), PipelineDefinitionError> {
        if self.ports.iter().any(|existing| existing.name == port.name) {
            return Err(PipelineDefinitionError::DuplicatePort {
                stage: self.id.clone(),
                port: port.name,
            });
        }

        self.ports.push(port);

        Ok(())
    }

    /// Insert or replace stage metadata.
    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, PipelineDefinitionError> {
        self.metadata.insert(key, value)
    }

    /// Return input ports.
    pub fn inputs(&self) -> impl Iterator<Item = &PipelinePort> {
        self.ports
            .iter()
            .filter(|port| port.direction == PipelinePortDirection::Input)
    }

    /// Return output ports.
    pub fn outputs(&self) -> impl Iterator<Item = &PipelinePort> {
        self.ports
            .iter()
            .filter(|port| port.direction == PipelinePortDirection::Output)
    }
}

/// Pipeline-level metadata.
///
/// Metadata is descriptive and does not change the semantic meaning of the
/// pipeline unless a separately specified extension explicitly defines it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PipelineMetadata {
    entries: BTreeMap<String, String>,
}

impl PipelineMetadata {
    /// Construct empty metadata.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a metadata entry.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, PipelineDefinitionError> {
        let key = key.into();
        let value = value.into();

        validate_identifier("metadata key", &key)?;

        Ok(self.entries.insert(key, value))
    }

    /// Return a metadata value.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    /// Return all metadata entries.
    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.entries
    }

    /// Return whether metadata is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// An immutable, declarative AI pipeline definition.
///
/// The definition preserves stage declaration order. That order is not
/// interpreted as execution order. Graph and dependency modules determine
/// the relationships between stages.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PipelineDefinition {
    id: AiPipelineId,
    name: String,
    version: PipelineDefinitionVersion,
    stages: Vec<PipelineStageDefinition>,
    metadata: PipelineMetadata,
}

impl PipelineDefinition {
    /// Return the pipeline identity.
    pub fn id(&self) -> AiPipelineId {
        self.id
    }

    /// Return the pipeline name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the pipeline definition version.
    pub const fn version(&self) -> PipelineDefinitionVersion {
        self.version
    }

    /// Return the stages in declaration order.
    pub fn stages(&self) -> &[PipelineStageDefinition] {
        &self.stages
    }

    /// Return pipeline metadata.
    pub fn metadata(&self) -> &PipelineMetadata {
        &self.metadata
    }

    /// Find a stage by its local identity.
    pub fn stage(&self, id: &PipelineStageId) -> Option<&PipelineStageDefinition> {
        self.stages.iter().find(|stage| &stage.id == id)
    }

    /// Return whether the definition declares any stages.
    pub fn is_empty(&self) -> bool {
        self.stages.is_empty()
    }

    /// Return the number of declared stages.
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }
}

/// Builder for a pipeline definition.
///
/// The builder is mutable during construction. The resulting definition is
/// immutable and can be shared safely through ordinary Rust ownership.
///
/// This builder validates local declaration invariants. Complete graph,
/// semantic, type, capability, and resource validation is delegated to the
/// corresponding repository authorities.
#[derive(Clone, Debug)]
pub struct PipelineDefinitionBuilder {
    id: AiPipelineId,
    name: String,
    version: PipelineDefinitionVersion,
    stages: Vec<PipelineStageDefinition>,
    metadata: PipelineMetadata,
}

impl PipelineDefinitionBuilder {
    /// Construct a pipeline builder.
    pub fn new(
        id: AiPipelineId,
        name: impl Into<String>,
    ) -> Result<Self, PipelineDefinitionError> {
        let name = name.into();

        validate_identifier("pipeline name", &name)?;

        Ok(Self {
            id,
            name,
            version: PipelineDefinitionVersion::default(),
            stages: Vec::new(),
            metadata: PipelineMetadata::new(),
        })
    }

    /// Set the pipeline definition version.
    pub fn version(
        mut self,
        version: PipelineDefinitionVersion,
    ) -> Self {
        self.version = version;
        self
    }

    /// Add a stage declaration.
    ///
    /// Duplicate stage identities are rejected.
    pub fn add_stage(
        &mut self,
        stage: PipelineStageDefinition,
    ) -> Result<(), PipelineDefinitionError> {
        if self.stages.iter().any(|existing| existing.id == stage.id) {
            return Err(PipelineDefinitionError::DuplicateStage {
                stage: stage.id,
            });
        }

        self.stages.push(stage);

        Ok(())
    }

    /// Insert or replace pipeline metadata.
    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, PipelineDefinitionError> {
        self.metadata.insert(key, value)
    }

    /// Build the immutable definition.
    ///
    /// Empty pipelines are permitted at this structural layer. Whether an
    /// empty pipeline is semantically valid is decided by pipeline
    /// validation and the source-language specification.
    pub fn build(self) -> PipelineDefinition {
        PipelineDefinition {
            id: self.id,
            name: self.name,
            version: self.version,
            stages: self.stages,
            metadata: self.metadata,
        }
    }
}

/// Errors raised while constructing a pipeline definition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PipelineDefinitionError {
    /// A required identifier or name was empty or contained a control
    /// character.
    InvalidIdentifier {
        field: &'static str,
    },

    /// A stage identity was already declared in this pipeline.
    DuplicateStage {
        stage: PipelineStageId,
    },

    /// A port identity was already declared in this stage.
    DuplicatePort {
        stage: PipelineStageId,
        port: String,
    },
}

impl fmt::Display for PipelineDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field } => {
                write!(f, "{field} must be non-empty and contain no control characters")
            }

            Self::DuplicateStage { stage } => {
                write!(f, "pipeline stage '{stage}' is already declared")
            }

            Self::DuplicatePort { stage, port } => {
                write!(
                    f,
                    "port '{port}' is already declared in pipeline stage '{stage}'"
                )
            }
        }
    }
}

impl Error for PipelineDefinitionError {}

fn validate_identifier(
    field: &'static str,
    value: &str,
) -> Result<(), PipelineDefinitionError> {
    if value.is_empty() || value.chars().any(char::is_control) {
        return Err(PipelineDefinitionError::InvalidIdentifier { field });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pipeline_id(value: u128) -> AiPipelineId {
        AiPipelineId::new(value).expect("nonzero pipeline ID")
    }

    fn stage_id(value: &str) -> PipelineStageId {
        PipelineStageId::new(value).expect("valid stage ID")
    }

    fn operation() -> PipelineOperationRef {
        PipelineOperationRef::new("zamani.ai", "transform")
            .expect("valid operation reference")
    }

    fn stage(id: &str) -> PipelineStageDefinition {
        PipelineStageDefinition::new(stage_id(id), operation())
    }

    #[test]
    fn creates_definition_with_canonical_pipeline_identity() {
        let definition = PipelineDefinitionBuilder::new(
            pipeline_id(1),
            "example",
        )
        .expect("valid pipeline")
        .build();

        assert_eq!(definition.id(), pipeline_id(1));
        assert_eq!(definition.name(), "example");
        assert_eq!(definition.version(), PipelineDefinitionVersion::default());
        assert!(definition.is_empty());
        assert_eq!(definition.stage_count(), 0);
    }

    #[test]
    fn preserves_stage_declaration_order() {
        let mut builder = PipelineDefinitionBuilder::new(
            pipeline_id(2),
            "ordered",
        )
        .expect("valid pipeline");

        builder.add_stage(stage("first")).expect("first stage");
        builder.add_stage(stage("second")).expect("second stage");
        builder.add_stage(stage("third")).expect("third stage");

        let definition = builder.build();

        let names: Vec<_> = definition
            .stages()
            .iter()
            .map(|stage| stage.id().as_str())
            .collect();

        assert_eq!(names, ["first", "second", "third"]);
    }

    #[test]
    fn rejects_duplicate_stage_ids() {
        let mut builder = PipelineDefinitionBuilder::new(
            pipeline_id(3),
            "duplicate",
        )
        .expect("valid pipeline");

        builder.add_stage(stage("stage")).expect("first declaration");

        let error = builder
            .add_stage(stage("stage"))
            .expect_err("duplicate must fail");

        assert!(matches!(
            error,
            PipelineDefinitionError::DuplicateStage { .. }
        ));
    }

    #[test]
    fn rejects_duplicate_port_names() {
        let mut stage = stage("stage");

        stage
            .add_port(
                PipelinePort::new(
                    "input",
                    PipelinePortDirection::Input,
                    None,
                )
                .expect("valid port"),
            )
            .expect("first port");

        let error = stage
            .add_port(
                PipelinePort::new(
                    "input",
                    PipelinePortDirection::Output,
                    None,
                )
                .expect("valid port"),
            )
            .expect_err("port names must be unique");

        assert!(matches!(
            error,
            PipelineDefinitionError::DuplicatePort { .. }
        ));
    }

    #[test]
    fn separates_input_and_output_ports() {
        let mut stage = stage("stage");

        stage
            .add_port(
                PipelinePort::new(
                    "source",
                    PipelinePortDirection::Input,
                    Some("tensor<f32>".to_owned()),
                )
                .expect("input port"),
            )
            .expect("input declaration");

        stage
            .add_port(
                PipelinePort::new(
                    "result",
                    PipelinePortDirection::Output,
                    Some("tensor<f32>".to_owned()),
                )
                .expect("output port"),
            )
            .expect("output declaration");

        assert_eq!(stage.inputs().count(), 1);
        assert_eq!(stage.outputs().count(), 1);
    }

    #[test]
    fn rejects_empty_names() {
        assert!(PipelineStageId::new("").is_err());

        assert!(
            PipelineOperationRef::new("namespace", "")
                .is_err()
        );

        assert!(
            PipelineDefinitionBuilder::new(
                pipeline_id(4),
                "",
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_control_characters() {
        assert!(PipelineStageId::new("stage\nname").is_err());

        assert!(
            PipelineOperationRef::new(
                "namespace",
                "operation\0name",
            )
            .is_err()
        );
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let mut metadata = PipelineMetadata::new();

        metadata
            .insert("z", "last")
            .expect("metadata entry");

        metadata
            .insert("a", "first")
            .expect("metadata entry");

        let keys: Vec<_> = metadata.entries().keys().map(String::as_str).collect();

        assert_eq!(keys, ["a", "z"]);
    }

    #[test]
    fn metadata_replacement_is_explicit() {
        let mut metadata = PipelineMetadata::new();

        assert_eq!(
            metadata.insert("key", "first").expect("first value"),
            None,
        );

        assert_eq!(
            metadata.insert("key", "second").expect("replacement"),
            Some("first".to_owned()),
        );

        assert_eq!(metadata.get("key"), Some("second"));
    }

    #[test]
    fn stage_lookup_uses_local_identity() {
        let mut builder = PipelineDefinitionBuilder::new(
            pipeline_id(5),
            "lookup",
        )
        .expect("valid pipeline");

        builder.add_stage(stage("known")).expect("stage");

        let definition = builder.build();

        assert!(definition.stage(&stage_id("known")).is_some());
        assert!(definition.stage(&stage_id("missing")).is_none());
    }

    #[test]
    fn custom_definition_version_is_preserved() {
        let version = PipelineDefinitionVersion::new(2, 4, 7);

        let definition = PipelineDefinitionBuilder::new(
            pipeline_id(6),
            "versioned",
        )
        .expect("valid pipeline")
        .version(version)
        .build();

        assert_eq!(definition.version(), version);
    }

    #[test]
    fn no_hardware_identity_is_part_of_the_definition() {
        let mut builder = PipelineDefinitionBuilder::new(
            pipeline_id(7),
            "portable",
        )
        .expect("valid pipeline");

        builder.add_stage(stage("compute")).expect("stage");

        let definition = builder.build();

        assert_eq!(
            definition.stages()[0].operation().namespace(),
            "zamani.ai",
        );

        assert_eq!(
            definition.stages()[0].operation().name(),
            "transform",
        );
    }
}
