// src/zuir/dialect/operation.rs
//
// Rust edition: 2021
// Minimum supported Rust version: 1.97
// Safety: unsafe Rust is forbidden.
//
// STATUS: Proposed foundational implementation.
// AUTHORITY:
// - src/zuir/ZUIR-SPEC.md
// - src/zuir/dialect/definition.rs
// - src/zuir/dialect/interfaces.rs
// - src/zuir/dialect/registry.rs
// - src/zuir/schema/types.rs
// - src/zuir/schema/attributes.rs
// - src/zuir/version.rs
// - grammar/DESIGN.md
// - grammar/Zamani.g4
// - grammar/lexer/tokens.g4
// - grammar/specification/semantic-model.md
// - grammar/specification/compatibility.md
//
// PURPOSE:
// Define open-ended, versioned, declarative operation schemas for ZUIR.
//
// OWNS:
// - Operation schema identity and version.
// - Operand, result, and region-group declarations.
// - Cardinality contracts.
// - Declared versus unknown effects.
// - Terminator contracts.
// - Required interface declarations.
// - Local operation-schema validation.
//
// DOES_NOT_OWN:
// - Concrete IR operation instances.
// - Runtime execution or extension-code loading.
// - Whole-program verification.
// - Semantic type inference.
// - Dialect registration and dependency resolution.
// - Physical hardware selection, allocation, or scheduling.
// - Source-language grammar or AST construction.
//
// SCALABILITY:
// All operation groups use dynamically sized collections.
// Cardinality describes a schema contract, not a universal machine limit.
// No maximum number of operations, operands, results, regions, qubits,
// devices, threads, tensor dimensions, or hardware units is imposed.
//
// IMPORTANT:
// Deserialization does not prove validity. Call validate() before registering
// or relying on a decoded operation definition.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use crate::zuir::dialect::definition::{
    DialectExportName,
    DialectName,
};
use crate::zuir::dialect::interfaces::InterfaceRequirement;
use crate::zuir::schema::attributes::{
    AttributeSchema,
    UnknownAttributePolicy,
};
use crate::zuir::schema::types::{
    QualifiedName,
    SchemaType,
};
use crate::zuir::version::DialectVersion;

// =============================================================================
// Errors
// =============================================================================

/// Errors raised while declaring or validating an operation schema.
///
/// This type describes schema errors. It does not replace whole-program
/// verifier diagnostics or target-lowering errors.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum OperationSchemaError {
    #[error("invalid operation identity: {0}")]
    InvalidIdentity(String),

    #[error("invalid schema name `{name}`: {reason}")]
    InvalidName {
        name: String,
        reason: String,
    },

    #[error("invalid cardinality: minimum {minimum} exceeds maximum {maximum}")]
    InvalidCardinality {
        minimum: usize,
        maximum: usize,
    },

    #[error("duplicate {category} group name `{name}`")]
    DuplicateGroup {
        category: &'static str,
        name: String,
    },

    #[error("invalid schema type: {0}")]
    InvalidSchemaType(String),

    #[error("invalid attribute schema: {0}")]
    InvalidAttributeSchema(String),

    #[error("invalid interface requirement: {0}")]
    InvalidInterfaceRequirement(String),

    #[error("invalid effect declaration: {0}")]
    InvalidEffect(String),

    #[error("invalid terminator contract: {0}")]
    InvalidTerminator(String),

    #[error("group-count list has {actual} entries; schema requires {expected}")]
    GroupCountLength {
        expected: usize,
        actual: usize,
    },

    #[error("group `{name}` accepts {minimum}..={maximum:?} entries, received {actual}")]
    GroupCardinality {
        name: String,
        minimum: usize,
        maximum: Option<usize>,
        actual: usize,
    },

    #[error("total cardinality arithmetic overflow")]
    CardinalityOverflow,

    #[error("total arity {actual} is outside the permitted range {minimum}..={maximum:?}")]
    TotalArity {
        minimum: usize,
        maximum: Option<usize>,
        actual: usize,
    },
}

// =============================================================================
// Cardinality
// =============================================================================

/// Cardinality contract for a group of operands, results, or regions.
///
/// `maximum: None` means that the schema declares no upper cardinality bound
/// for this group. It does not mean that a process has infinite memory.
///
/// A finite maximum, when supplied, is part of that operation's schema
/// semantics; it is not a global ZUIR or hardware capacity limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cardinality {
    minimum: usize,
    maximum: Option<usize>,
}

impl Cardinality {
    /// Construct a validated cardinality contract.
    pub fn new(
        minimum: usize,
        maximum: Option<usize>,
    ) -> Result<Self, OperationSchemaError> {
        if let Some(maximum) = maximum {
            if minimum > maximum {
                return Err(OperationSchemaError::InvalidCardinality {
                    minimum,
                    maximum,
                });
            }
        }

        Ok(Self { minimum, maximum })
    }

    /// Exactly one item.
    #[must_use]
    pub const fn one() -> Self {
        Self {
            minimum: 1,
            maximum: Some(1),
        }
    }

    /// Zero or one item.
    #[must_use]
    pub const fn optional() -> Self {
        Self {
            minimum: 0,
            maximum: Some(1),
        }
    }

    /// Zero or more items.
    #[must_use]
    pub const fn variadic() -> Self {
        Self {
            minimum: 0,
            maximum: None,
        }
    }

    /// One or more items.
    #[must_use]
    pub const fn one_or_more() -> Self {
        Self {
            minimum: 1,
            maximum: None,
        }
    }

    #[must_use]
    pub const fn minimum(self) -> usize {
        self.minimum
    }

    #[must_use]
    pub const fn maximum(self) -> Option<usize> {
        self.maximum
    }

    /// Validate the contract, including after deserialization.
    pub fn validate(self) -> Result<(), OperationSchemaError> {
        if let Some(maximum) = self.maximum {
            if self.minimum > maximum {
                return Err(OperationSchemaError::InvalidCardinality {
                    minimum: self.minimum,
                    maximum,
                });
            }
        }

        Ok(())
    }

    /// Check one observed count against this contract.
    pub fn validate_count(
        self,
        name: impl Into<String>,
        actual: usize,
    ) -> Result<(), OperationSchemaError> {
        self.validate()?;

        if actual < self.minimum
            || self.maximum.is_some_and(|maximum| actual > maximum)
        {
            return Err(OperationSchemaError::GroupCardinality {
                name: name.into(),
                minimum: self.minimum,
                maximum: self.maximum,
                actual,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Operand and result groups
// =============================================================================

/// Declares a named group of operation operands.
///
/// `value_type` is a schema-level descriptor. It does not replace the
/// semantic type system or prove that a concrete IR value has that type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperandGroup {
    name: QualifiedName,
    value_type: SchemaType,
    cardinality: Cardinality,
    role: Option<QualifiedName>,
}

impl OperandGroup {
    pub fn new(
        name: QualifiedName,
        value_type: SchemaType,
        cardinality: Cardinality,
    ) -> Result<Self, OperationSchemaError> {
        name.validate().map_err(|error| {
            OperationSchemaError::InvalidName {
                name: name.canonical_string(),
                reason: error.to_string(),
            }
        })?;
        value_type.validate().map_err(|error| {
            OperationSchemaError::InvalidSchemaType(error.to_string())
        })?;
        cardinality.validate()?;

        Ok(Self {
            name,
            value_type,
            cardinality,
            role: None,
        })
    }

    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    #[must_use]
    pub fn value_type(&self) -> &SchemaType {
        &self.value_type
    }

    #[must_use]
    pub const fn cardinality(&self) -> Cardinality {
        self.cardinality
    }

    #[must_use]
    pub fn role(&self) -> Option<&QualifiedName> {
        self.role.as_ref()
    }

    /// Assign a namespaced role, such as a domain-defined control role.
    pub fn with_role(
        mut self,
        role: QualifiedName,
    ) -> Result<Self, OperationSchemaError> {
        role.validate().map_err(|error| {
            OperationSchemaError::InvalidName {
                name: role.canonical_string(),
                reason: error.to_string(),
            }
        })?;

        self.role = Some(role);
        Ok(self)
    }

    pub fn validate(&self) -> Result<(), OperationSchemaError> {
        self.name.validate().map_err(|error| {
            OperationSchemaError::InvalidName {
                name: self.name.canonical_string(),
                reason: error.to_string(),
            }
        })?;
        self.value_type.validate().map_err(|error| {
            OperationSchemaError::InvalidSchemaType(error.to_string())
        })?;
        self.cardinality.validate()?;

        if let Some(role) = &self.role {
            role.validate().map_err(|error| {
                OperationSchemaError::InvalidName {
                    name: role.canonical_string(),
                    reason: error.to_string(),
                }
            })?;
        }

        Ok(())
    }
}

/// Declares a named group of operation results.
///
/// Result types are descriptors. The concrete IR verifier remains responsible
/// for checking result definitions and consistency with semantic types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultGroup {
    name: QualifiedName,
    value_type: SchemaType,
    cardinality: Cardinality,
}

impl ResultGroup {
    pub fn new(
        name: QualifiedName,
        value_type: SchemaType,
        cardinality: Cardinality,
    ) -> Result<Self, OperationSchemaError> {
        name.validate().map_err(|error| {
            OperationSchemaError::InvalidName {
                name: name.canonical_string(),
                reason: error.to_string(),
            }
        })?;
        value_type.validate().map_err(|error| {
            OperationSchemaError::InvalidSchemaType(error.to_string())
        })?;
        cardinality.validate()?;

        Ok(Self {
            name,
            value_type,
            cardinality,
        })
    }

    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    #[must_use]
    pub fn value_type(&self) -> &SchemaType {
        &self.value_type
    }

    #[must_use]
    pub const fn cardinality(&self) -> Cardinality {
        self.cardinality
    }

    pub fn validate(&self) -> Result<(), OperationSchemaError> {
        self.name.validate().map_err(|error| {
            OperationSchemaError::InvalidName {
                name: self.name.canonical_string(),
                reason: error.to_string(),
            }
        })?;
        self.value_type.validate().map_err(|error| {
            OperationSchemaError::InvalidSchemaType(error.to_string())
        })?;
        self.cardinality.validate()
    }
}

// =============================================================================
// Regions
// =============================================================================

/// Declares a group of nested regions accepted by an operation.
///
/// `kind` is a qualified, extensible semantic identifier. The core does not
/// enumerate every possible region kind or assume that all regions are
/// ordinary control-flow regions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionGroup {
    name: QualifiedName,
    kind: QualifiedName,
    cardinality: Cardinality,
}

impl RegionGroup {
    pub fn new(
        name: QualifiedName,
        kind: QualifiedName,
        cardinality: Cardinality,
    ) -> Result<Self, OperationSchemaError> {
        name.validate().map_err(|error| {
            OperationSchemaError::InvalidName {
                name: name.canonical_string(),
                reason: error.to_string(),
            }
        })?;
        kind.validate().map_err(|error| {
            OperationSchemaError::InvalidName {
                name: kind.canonical_string(),
                reason: error.to_string(),
            }
        })?;
        cardinality.validate()?;

        Ok(Self {
            name,
            kind,
            cardinality,
        })
    }

    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    #[must_use]
    pub fn kind(&self) -> &QualifiedName {
        &self.kind
    }

    #[must_use]
    pub const fn cardinality(&self) -> Cardinality {
        self.cardinality
    }

    pub fn validate(&self) -> Result<(), OperationSchemaError> {
        self.name.validate().map_err(|error| {
            OperationSchemaError::InvalidName {
                name: self.name.canonical_string(),
                reason: error.to_string(),
            }
        })?;
        self.kind.validate().map_err(|error| {
            OperationSchemaError::InvalidName {
                name: self.kind.canonical_string(),
                reason: error.to_string(),
            }
        })?;
        self.cardinality.validate()
    }
}

// =============================================================================
// Effects and terminators
// =============================================================================

/// Whether an operation's effect contract is known.
///
/// An unknown effect declaration is not equivalent to a pure operation.
/// Optimizers and lowering stages must conservatively preserve such operations
/// until a trusted definition establishes their behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "effects", rename_all = "snake_case")]
pub enum EffectDeclaration {
    /// The complete declared effect set.
    ///
    /// An explicitly declared empty set means no effects are declared by this
    /// schema. The verifier must still check that the declaration is trusted
    /// and consistent with the applicable dialect contract.
    Known(BTreeSet<QualifiedName>),

    /// Effect behavior has not been established.
    Unknown,
}

impl EffectDeclaration {
    /// Construct a known effect set after validating all effect identifiers.
    pub fn known(
        effects: impl IntoIterator<Item = QualifiedName>,
    ) -> Result<Self, OperationSchemaError> {
        let mut validated = BTreeSet::new();

        for effect in effects {
            effect.validate().map_err(|error| {
                OperationSchemaError::InvalidEffect(error.to_string())
            })?;
            validated.insert(effect);
        }

        Ok(Self::Known(validated))
    }

    #[must_use]
    pub const fn unknown() -> Self {
        Self::Unknown
    }

    #[must_use]
    pub const fn is_known(&self) -> bool {
        matches!(self, Self::Known(_))
    }

    #[must_use]
    pub fn effects(&self) -> Option<&BTreeSet<QualifiedName>> {
        match self {
            Self::Known(effects) => Some(effects),
            Self::Unknown => None,
        }
    }

    pub fn validate(&self) -> Result<(), OperationSchemaError> {
        if let Self::Known(effects) = self {
            for effect in effects {
                effect.validate().map_err(|error| {
                    OperationSchemaError::InvalidEffect(error.to_string())
                })?;
            }
        }

        Ok(())
    }
}

/// Control-flow contract for an operation that terminates a block.
///
/// Successor target validity and edge arguments are checked by the control-flow
/// verifier, which has access to the actual IR graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminatorContract {
    successor_count: Cardinality,
    requires_edge_arguments: bool,
}

impl TerminatorContract {
    pub fn new(
        successor_count: Cardinality,
        requires_edge_arguments: bool,
    ) -> Result<Self, OperationSchemaError> {
        successor_count.validate()?;

        Ok(Self {
            successor_count,
            requires_edge_arguments,
        })
    }

    /// Contract for a terminator with no control-flow successors.
    pub fn exit() -> Self {
        Self {
            successor_count: Cardinality {
                minimum: 0,
                maximum: Some(0),
            },
            requires_edge_arguments: false,
        }
    }

    #[must_use]
    pub const fn successor_count(&self) -> Cardinality {
        self.successor_count
    }

    #[must_use]
    pub const fn requires_edge_arguments(&self) -> bool {
        self.requires_edge_arguments
    }

    pub fn validate(&self) -> Result<(), OperationSchemaError> {
        self.successor_count.validate()
    }
}

// =============================================================================
// Operation definition
// =============================================================================

/// Versioned declarative schema for one operation in a dialect.
///
/// This is not an operation instance and contains no executable callback.
/// Implementations for execution, folding, differentiation, lowering, or
/// verification must be bound through separately governed interfaces.
///
/// The operation's identity is the pair `(dialect, name)`. Operation names
/// remain extensible; this structure contains no universal operation enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationDefinition {
    dialect: DialectName,
    name: DialectExportName,
    version: DialectVersion,

    operands: Vec<OperandGroup>,
    results: Vec<ResultGroup>,
    regions: Vec<RegionGroup>,

    attributes: AttributeSchema,
    effects: EffectDeclaration,
    interfaces: Vec<InterfaceRequirement>,
    terminator: Option<TerminatorContract>,

    /// Optional descriptive documentation, not executable semantics.
    description: Option<String>,
}

impl OperationDefinition {
    /// Construct an operation schema with a conservative effect contract.
    ///
    /// New schemas default to unknown effects. A dialect author must explicitly
    /// declare known effects before a consumer can treat the operation as
    /// effect-free.
    pub fn new(
        dialect: DialectName,
        name: DialectExportName,
        version: DialectVersion,
    ) -> Result<Self, OperationSchemaError> {
        dialect.validate().map_err(|error| {
            OperationSchemaError::InvalidIdentity(error.to_string())
        })?;
        name.validate().map_err(|error| {
            OperationSchemaError::InvalidIdentity(error.to_string())
        })?;

        Ok(Self {
            dialect,
            name,
            version,
            operands: Vec::new(),
            results: Vec::new(),
            regions: Vec::new(),
            attributes: AttributeSchema::new(UnknownAttributePolicy::Reject),
            effects: EffectDeclaration::Unknown,
            interfaces: Vec::new(),
            terminator: None,
            description: None,
        })
    }

    /// Attach documentation without changing operation semantics.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Return the owning dialect.
    #[must_use]
    pub fn dialect(&self) -> &DialectName {
        &self.dialect
    }

    /// Return the exported operation name.
    #[must_use]
    pub fn name(&self) -> &DialectExportName {
        &self.name
    }

    /// Return the declared schema version.
    #[must_use]
    pub const fn version(&self) -> DialectVersion {
        self.version
    }

    #[must_use]
    pub fn operands(&self) -> &[OperandGroup] {
        &self.operands
    }

    #[must_use]
    pub fn results(&self) -> &[ResultGroup] {
        &self.results
    }

    #[must_use]
    pub fn regions(&self) -> &[RegionGroup] {
        &self.regions
    }

    #[must_use]
    pub fn attributes(&self) -> &AttributeSchema {
        &self.attributes
    }

    #[must_use]
    pub fn effects(&self) -> &EffectDeclaration {
        &self.effects
    }

    #[must_use]
    pub fn interfaces(&self) -> &[InterfaceRequirement] {
        &self.interfaces
    }

    #[must_use]
    pub fn terminator(&self) -> Option<&TerminatorContract> {
        self.terminator.as_ref()
    }

    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Append an operand group.
    pub fn add_operand_group(
        &mut self,
        group: OperandGroup,
    ) -> Result<(), OperationSchemaError> {
        group.validate()?;
        ensure_unique_name(
            "operand",
            group.name(),
            self.operands.iter().map(OperandGroup::name),
        )?;
        self.operands.push(group);
        Ok(())
    }

    /// Append a result group.
    pub fn add_result_group(
        &mut self,
        group: ResultGroup,
    ) -> Result<(), OperationSchemaError> {
        group.validate()?;
        ensure_unique_name(
            "result",
            group.name(),
            self.results.iter().map(ResultGroup::name),
        )?;
        self.results.push(group);
        Ok(())
    }

    /// Append a region group.
    pub fn add_region_group(
        &mut self,
        group: RegionGroup,
    ) -> Result<(), OperationSchemaError> {
        group.validate()?;
        ensure_unique_name(
            "region",
            group.name(),
            self.regions.iter().map(RegionGroup::name),
        )?;
        self.regions.push(group);
        Ok(())
    }

    /// Replace the operation's attribute schema.
    pub fn set_attribute_schema(
        &mut self,
        attributes: AttributeSchema,
    ) -> Result<(), OperationSchemaError> {
        attributes.validate().map_err(|error| {
            OperationSchemaError::InvalidAttributeSchema(error.to_string())
        })?;

        self.attributes = attributes;
        Ok(())
    }

    /// Declare the complete effect contract.
    pub fn set_effects(
        &mut self,
        effects: EffectDeclaration,
    ) -> Result<(), OperationSchemaError> {
        effects.validate()?;
        self.effects = effects;
        Ok(())
    }

    /// Add a required interface contract.
    pub fn require_interface(
        &mut self,
        requirement: InterfaceRequirement,
    ) -> Result<(), OperationSchemaError> {
        requirement.validate().map_err(|error| {
            OperationSchemaError::InvalidInterfaceRequirement(error.to_string())
        })?;

        if self.interfaces.iter().any(|existing| {
            existing.name() == requirement.name()
        }) {
            return Err(OperationSchemaError::InvalidInterfaceRequirement(
                format!(
                    "duplicate interface requirement `{}`",
                    requirement.name()
                ),
            ));
        }

        self.interfaces.push(requirement);
        Ok(())
    }

    /// Declare that this operation terminates a block.
    pub fn set_terminator(
        &mut self,
        contract: TerminatorContract,
    ) -> Result<(), OperationSchemaError> {
        contract.validate()?;
        self.terminator = Some(contract);
        Ok(())
    }

    /// Remove a terminator declaration.
    ///
    /// This changes the schema contract and should only be used during
    /// construction before registration.
    pub fn clear_terminator(&mut self) {
        self.terminator = None;
    }

    /// Validate the complete declaration, including values reconstructed
    /// through deserialization.
    pub fn validate(&self) -> Result<(), OperationSchemaError> {
        self.dialect.validate().map_err(|error| {
            OperationSchemaError::InvalidIdentity(error.to_string())
        })?;
        self.name.validate().map_err(|error| {
            OperationSchemaError::InvalidIdentity(error.to_string())
        })?;

        self.attributes.validate().map_err(|error| {
            OperationSchemaError::InvalidAttributeSchema(error.to_string())
        })?;
        self.effects.validate()?;

        if let Some(description) = &self.description {
            if description.contains('\0') {
                return Err(OperationSchemaError::InvalidIdentity(
                    "description contains a NUL character".to_owned(),
                ));
            }
        }

        validate_unique_names(
            "operand",
            self.operands.iter().map(OperandGroup::name),
        )?;
        for group in &self.operands {
            group.validate()?;
        }

        validate_unique_names(
            "result",
            self.results.iter().map(ResultGroup::name),
        )?;
        for group in &self.results {
            group.validate()?;
        }

        validate_unique_names(
            "region",
            self.regions.iter().map(RegionGroup::name),
        )?;
        for group in &self.regions {
            group.validate()?;
        }

        let mut seen_interfaces = BTreeSet::new();
        for requirement in &self.interfaces {
            requirement.validate().map_err(|error| {
                OperationSchemaError::InvalidInterfaceRequirement(error.to_string())
            })?;

            if !seen_interfaces.insert(requirement.name().clone()) {
                return Err(OperationSchemaError::InvalidInterfaceRequirement(
                    format!(
                        "duplicate interface requirement `{}`",
                        requirement.name()
                    ),
                ));
            }
        }

        if let Some(terminator) = &self.terminator {
            terminator.validate()?;
        }

        Ok(())
    }

    /// Validate the total operand count against the sum of declared groups.
    ///
    /// This is a useful coarse check. When an operation instance preserves
    /// group boundaries, prefer `validate_operand_group_counts` so each group
    /// is checked individually.
    pub fn validate_operand_count(
        &self,
        actual: usize,
    ) -> Result<(), OperationSchemaError> {
        validate_total_cardinality(
            self.operands.iter().map(OperandGroup::cardinality),
            actual,
        )
    }

    /// Validate operand counts for each declared group.
    pub fn validate_operand_group_counts(
        &self,
        counts: &[usize],
    ) -> Result<(), OperationSchemaError> {
        validate_group_counts(
            self.operands.iter().map(|group| {
                (group.name(), group.cardinality())
            }),
            counts,
        )
    }

    /// Validate the total result count against the sum of declared groups.
    pub fn validate_result_count(
        &self,
        actual: usize,
    ) -> Result<(), OperationSchemaError> {
        validate_total_cardinality(
            self.results.iter().map(ResultGroup::cardinality),
            actual,
        )
    }

    /// Validate result counts for each declared group.
    pub fn validate_result_group_counts(
        &self,
        counts: &[usize],
    ) -> Result<(), OperationSchemaError> {
        validate_group_counts(
            self.results.iter().map(|group| {
                (group.name(), group.cardinality())
            }),
            counts,
        )
    }

    /// Validate region counts for each declared group.
    pub fn validate_region_group_counts(
        &self,
        counts: &[usize],
    ) -> Result<(), OperationSchemaError> {
        validate_group_counts(
            self.regions.iter().map(|group| {
                (group.name(), group.cardinality())
            }),
            counts,
        )
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn ensure_unique_name<'a>(
    category: &'static str,
    name: &QualifiedName,
    existing: impl Iterator<Item = &'a QualifiedName>,
) -> Result<(), OperationSchemaError> {
    if existing.into_iter().any(|candidate| candidate == name) {
        return Err(OperationSchemaError::DuplicateGroup {
            category,
            name: name.canonical_string(),
        });
    }

    Ok(())
}

fn validate_unique_names<'a>(
    category: &'static str,
    names: impl Iterator<Item = &'a QualifiedName>,
) -> Result<(), OperationSchemaError> {
    let mut seen = BTreeSet::new();

    for name in names {
        name.validate().map_err(|error| {
            OperationSchemaError::InvalidName {
                name: name.canonical_string(),
                reason: error.to_string(),
            }
        })?;

        if !seen.insert(name.clone()) {
            return Err(OperationSchemaError::DuplicateGroup {
                category,
                name: name.canonical_string(),
            });
        }
    }

    Ok(())
}

fn validate_group_counts<'a>(
    groups: impl Iterator<Item = (&'a QualifiedName, Cardinality)>,
    counts: &[usize],
) -> Result<(), OperationSchemaError> {
    let groups: Vec<(&QualifiedName, Cardinality)> = groups.collect();

    if groups.len() != counts.len() {
        return Err(OperationSchemaError::GroupCountLength {
            expected: groups.len(),
            actual: counts.len(),
        });
    }

    for ((name, cardinality), actual) in groups.into_iter().zip(counts.iter().copied()) {
        cardinality.validate_count(name.canonical_string(), actual)?;
    }

    Ok(())
}

fn validate_total_cardinality(
    cardinalities: impl Iterator<Item = Cardinality>,
    actual: usize,
) -> Result<(), OperationSchemaError> {
    let mut minimum = 0usize;
    let mut maximum = Some(0usize);

    for cardinality in cardinalities {
        cardinality.validate()?;

        minimum = minimum
            .checked_add(cardinality.minimum())
            .ok_or(OperationSchemaError::CardinalityOverflow)?;

        maximum = match (maximum, cardinality.maximum()) {
            (Some(current), Some(additional)) => Some(
                current
                    .checked_add(additional)
                    .ok_or(OperationSchemaError::CardinalityOverflow)?,
            ),
            _ => None,
        };
    }

    if actual < minimum || maximum.is_some_and(|limit| actual > limit) {
        return Err(OperationSchemaError::TotalArity {
            minimum,
            maximum,
            actual,
        });
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::schema::types::SchemaType;

    fn qname(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("test qualified name must be valid")
    }

    fn dialect_version() -> DialectVersion {
        DialectVersion::new(1, 0, 0)
    }

    fn operation() -> OperationDefinition {
        OperationDefinition::new(
            DialectName::parse("core").expect("valid dialect"),
            DialectExportName::parse("compute").expect("valid export"),
            dialect_version(),
        )
        .expect("valid operation schema")
    }

    #[test]
    fn cardinality_supports_unbounded_schema_groups() {
        let cardinality = Cardinality::one_or_more();

        assert_eq!(cardinality.minimum(), 1);
        assert_eq!(cardinality.maximum(), None);
        assert!(cardinality.validate_count("items", 1).is_ok());
        assert!(cardinality.validate_count("items", usize::MAX).is_ok());
    }

    #[test]
    fn cardinality_rejects_invalid_bounds() {
        assert!(matches!(
            Cardinality::new(4, Some(3)),
            Err(OperationSchemaError::InvalidCardinality {
                minimum: 4,
                maximum: 3
            })
        ));
    }

    #[test]
    fn operation_names_are_not_a_closed_operation_enum() {
        let mut schema = operation();

        let operand = OperandGroup::new(
            qname("input"),
            SchemaType::Any,
            Cardinality::one_or_more(),
        )
        .expect("valid operand schema");

        schema
            .add_operand_group(operand)
            .expect("new operand group should be accepted");

        schema.validate().expect("schema should validate");
        assert_eq!(schema.operands().len(), 1);
    }

    #[test]
    fn unknown_effects_are_not_treated_as_known_empty_effects() {
        let schema = operation();

        assert!(!schema.effects().is_known());
        assert!(schema.effects().effects().is_none());
    }

    #[test]
    fn explicit_empty_effect_set_is_distinct_from_unknown() {
        let effects = EffectDeclaration::known([]).expect("empty set is valid");

        assert!(effects.is_known());
        assert_eq!(effects.effects().map(BTreeSet::len), Some(0));
    }

    #[test]
    fn duplicate_operand_groups_are_rejected() {
        let mut schema = operation();

        let first = OperandGroup::new(
            qname("input"),
            SchemaType::Any,
            Cardinality::one(),
        )
        .expect("valid first group");

        let second = OperandGroup::new(
            qname("input"),
            SchemaType::Boolean,
            Cardinality::optional(),
        )
        .expect("valid second group");

        schema.add_operand_group(first).expect("first group");

        assert!(matches!(
            schema.add_operand_group(second),
            Err(OperationSchemaError::DuplicateGroup {
                category: "operand",
                ..
            })
        ));
    }

    #[test]
    fn operand_groups_are_checked_individually() {
        let mut schema = operation();

        schema
            .add_operand_group(
                OperandGroup::new(
                    qname("control"),
                    SchemaType::Boolean,
                    Cardinality::optional(),
                )
                .expect("valid control group"),
            )
            .expect("add control");

        schema
            .add_operand_group(
                OperandGroup::new(
                    qname("targets"),
                    SchemaType::Any,
                    Cardinality::one_or_more(),
                )
                .expect("valid targets group"),
            )
            .expect("add targets");

        assert!(schema.validate_operand_group_counts(&[0, 3]).is_ok());
        assert!(schema.validate_operand_group_counts(&[2, 3]).is_err());
        assert!(schema.validate_operand_group_counts(&[1, 0]).is_err());
    }

    #[test]
    fn total_arity_overflow_is_reported() {
        let result = validate_total_cardinality(
            [
                Cardinality::new(usize::MAX, None).expect("valid cardinality"),
                Cardinality::one(),
            ]
            .into_iter(),
            usize::MAX,
        );

        assert_eq!(result, Err(OperationSchemaError::CardinalityOverflow));
    }

    #[test]
    fn terminator_contract_is_explicit() {
        let mut schema = operation();

        schema
            .set_terminator(TerminatorContract::exit())
            .expect("valid exit contract");

        schema.validate().expect("schema should validate");
        assert!(schema.terminator().is_some());
    }

    #[test]
    fn schema_revalidates_after_construction() {
        let schema = operation();

        schema.validate().expect("constructed schema is valid");
    }
}