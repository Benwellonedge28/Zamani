
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Operation Builder
//!
//! Path: src/zuir/builder/operation.rs
//!
//! STATUS: Proposed implementation; freeze only after repository compilation
//! and integration tests pass.
//!
//! AUTHORITY:
//! - src/zuir/ZUIR-SPEC.md
//! - src/zuir/ids.rs
//! - src/zuir/ir/operation.rs
//! - src/zuir/ir/operand.rs
//! - src/zuir/ir/result.rs
//! - src/zuir/dialect/operation.rs
//! - src/zuir/schema/attributes.rs
//! - grammar/DESIGN.md
//! - grammar/Zamani.g4
//! - grammar/specification/semantic-model.md
//! - grammar/specification/compatibility.md
//!
//! OWNS:
//! - Validated construction of concrete operation instances.
//! - Construction-time schema and cardinality checks.
//! - Explicit handling of unknown effect contracts.
//! - Atomic publication of a completed operation.
//!
//! DOES_NOT_OWN:
//! - Operation semantics, dialect registration, or executable plugins.
//! - Global SSA, type resolution, dominance, or def-use verification.
//! - Block termination and successor verification.
//! - Region storage or region ownership verification.
//! - Source grammar, AST construction, or semantic analysis.
//! - Target selection, hardware allocation, routing, or execution.
//!
//! SCALABILITY:
//! No universal machine, operation, operand, result, region, qubit,
//! register-width, tensor-rank, node, or device ceiling is imposed.
//! Fallible reservation is exposed for builder-owned collections.
//! Actual allocation and process resources remain finite.
//!
//! FREEZE GATE:
//! The referenced modules and APIs must compile together. Whole-IR
//! verification must run before optimization, lowering, or execution.

use std::collections::BTreeMap;
use std::fmt;

use crate::zuir::dialect::operation::{
    OperationDefinition,
    OperationSchemaError,
};
use crate::zuir::ids::{
    OperationId,
    RegionId,
    TypeId,
    ValueId,
};
use crate::zuir::ir::operand::OperationOperand;
use crate::zuir::ir::operation::{
    OperationError,
    ZuirOperation,
};
use crate::zuir::ir::region::ZuirRegion;
use crate::zuir::ir::result::OperationValueResult;
use crate::zuir::schema::attributes::{
    AttributeKey,
    AttributeSet,
    AttributeValue,
};
use crate::zuir::schema::types::QualifiedName;

// =============================================================================
// Errors
// =============================================================================

/// Failure while constructing an operation from its registered schema.
///
/// Errors from this builder do not replace whole-program verifier errors.
/// Human-readable messages are explanatory; callers should match variants.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OperationBuildError {
    InvalidSchema(String),

    InvalidOperationName {
        expected: String,
        actual: String,
    },

    InvalidOperand(String),

    InvalidResult(String),

    InvalidRegion(String),

    InvalidAttribute(String),

    InvalidEffect(String),

    UnknownEffectContract,

    Cardinality {
        group: String,
        expected_minimum: usize,
        expected_maximum: Option<usize>,
        actual: usize,
    },

    InvalidRegionKind {
        region: RegionId,
        expected: String,
        actual: String,
    },

    MissingRegion {
        region: RegionId,
    },

    DuplicateRegion(RegionId),

    AllocationFailure {
        collection: &'static str,
    },

    Operation(String),
}

impl fmt::Display for OperationBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSchema(message) =>
                write!(f, "invalid operation schema: {message}"),

            Self::InvalidOperationName { expected, actual } =>
                write!(
                    f,
                    "operation name mismatch: expected `{expected}`, got `{actual}`"
                ),

            Self::InvalidOperand(message) =>
                write!(f, "invalid operation operands: {message}"),

            Self::InvalidResult(message) =>
                write!(f, "invalid operation results: {message}"),

            Self::InvalidRegion(message) =>
                write!(f, "invalid operation regions: {message}"),

            Self::InvalidAttribute(message) =>
                write!(f, "invalid operation attributes: {message}"),

            Self::InvalidEffect(message) =>
                write!(f, "invalid operation effects: {message}"),

            Self::UnknownEffectContract =>
                f.write_str(
                    "operation has unknown effects; an explicit trusted \
                     effect contract is required before construction"
                ),

            Self::Cardinality {
                group,
                expected_minimum,
                expected_maximum,
                actual,
            } => write!(
                f,
                "group `{group}` expects {}..={} entries, got {actual}",
                expected_minimum,
                expected_maximum
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| "unbounded".to_owned())
            ),

            Self::InvalidRegionKind {
                region,
                expected,
                actual,
            } => write!(
                f,
                "region {region} has kind `{actual}`, expected `{expected}`"
            ),

            Self::MissingRegion { region } =>
                write!(f, "region {region} is not available"),

            Self::DuplicateRegion(region) =>
                write!(f, "region {region} is attached more than once"),

            Self::AllocationFailure { collection } =>
                write!(f, "unable to reserve builder storage for {collection}"),

            Self::Operation(message) =>
                write!(f, "operation construction failed: {message}"),
        }
    }
}

impl std::error::Error for OperationBuildError {}

impl From<OperationSchemaError> for OperationBuildError {
    fn from(error: OperationSchemaError) -> Self {
        Self::InvalidSchema(error.to_string())
    }
}

impl From<OperationError> for OperationBuildError {
    fn from(error: OperationError) -> Self {
        Self::Operation(error.to_string())
    }
}

pub type OperationBuildResult<T> = Result<T, OperationBuildError>;

// =============================================================================
// Region bindings
// =============================================================================

/// A region supplied to an operation builder.
///
/// The region's declared kind is checked against the operation schema.
/// This descriptor borrows the canonical region rather than copying or
/// creating a second region representation.
#[derive(Debug, Clone, Copy)]
pub struct RegionBinding<'a> {
    region: &'a ZuirRegion,
}

impl<'a> RegionBinding<'a> {
    #[must_use]
    pub const fn new(region: &'a ZuirRegion) -> Self {
        Self { region }
    }

    #[must_use]
    pub fn region(&self) -> &'a ZuirRegion {
        self.region
    }

    #[must_use]
    pub fn id(&self) -> RegionId {
        self.region.id()
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builds one canonical ZUIR operation from an authoritative schema.
///
/// The builder owns only pending construction data. The resulting operation
/// is the existing canonical `ZuirOperation`, not a second IR node type.
///
/// Operand and result types are checked against the complete semantic model
/// by the whole-IR verifier. This builder does not guess types from IDs.
pub struct OperationBuilder<'schema> {
    schema: &'schema OperationDefinition,
    id: OperationId,
    operands: Vec<OperationOperand>,
    results: Vec<OperationValueResult>,
    attributes: BTreeMap<AttributeKey, AttributeValue>,
    regions: Vec<RegionBinding<'schema>>,
}

impl<'schema> OperationBuilder<'schema> {
    /// Start constructing an operation under a validated operation schema.
    ///
    /// The ID must belong to the enclosing program's identity-allocation
    /// scope. The whole-IR verifier remains responsible for global uniqueness.
    pub fn new(
        id: OperationId,
        schema: &'schema OperationDefinition,
    ) -> OperationBuildResult<Self> {
        schema.validate()?;

        Ok(Self {
            schema,
            id,
            operands: Vec::new(),
            results: Vec::new(),
            attributes: BTreeMap::new(),
            regions: Vec::new(),
        })
    }

    #[must_use]
    pub const fn schema(&self) -> &'schema OperationDefinition {
        self.schema
    }

    /// Reserve operand capacity before a large append batch.
    pub fn try_reserve_operands(
        &mut self,
        additional: usize,
    ) -> OperationBuildResult<()> {
        self.operands
            .try_reserve(additional)
            .map_err(|_| OperationBuildError::AllocationFailure {
                collection: "operands",
            })
    }

    /// Add an operand reference.
    ///
    /// This validates local operand metadata. Value existence, visibility,
    /// type compatibility, and dominance belong to the whole-IR verifier.
    pub fn add_operand(
        &mut self,
        operand: OperationOperand,
    ) -> OperationBuildResult<&mut Self> {
        operand
            .validate()
            .map_err(|e| OperationBuildError::InvalidOperand(e.to_string()))?;

        self.operands.push(operand);
        Ok(self)
    }

    /// Append multiple operands without imposing an artificial arity limit.
    pub fn extend_operands<I>(
        &mut self,
        operands: I,
    ) -> OperationBuildResult<&mut Self>
    where
        I: IntoIterator<Item = OperationOperand>,
    {
        let iterator = operands.into_iter();
        let (lower, _) = iterator.size_hint();

        self.try_reserve_operands(lower)?;

        for operand in iterator {
            self.add_operand(operand)?;
        }

        Ok(self)
    }

    /// Reserve result capacity.
    pub fn try_reserve_results(
        &mut self,
        additional: usize,
    ) -> OperationBuildResult<()> {
        self.results
            .try_reserve(additional)
            .map_err(|_| OperationBuildError::AllocationFailure {
                collection: "results",
            })
    }

    /// Add a typed result.
    ///
    /// The result's TypeId is retained exactly. It is not converted into
    /// a host integer width, register type, physical qubit, or signal width.
    pub fn add_result(
        &mut self,
        result: OperationValueResult,
    ) -> OperationBuildResult<&mut Self> {
        if self
            .results
            .iter()
            .any(|existing| existing.value() == result.value())
        {
            return Err(OperationBuildError::InvalidResult(format!(
                "duplicate value identity {}",
                result.value()
            )));
        }

        self.results.push(result);
        Ok(self)
    }

    /// Append multiple typed results.
    pub fn extend_results<I>(
        &mut self,
        results: I,
    ) -> OperationBuildResult<&mut Self>
    where
        I: IntoIterator<Item = OperationValueResult>,
    {
        let iterator = results.into_iter();
        let (lower, _) = iterator.size_hint();

        self.try_reserve_results(lower)?;

        for result in iterator {
            self.add_result(result)?;
        }

        Ok(self)
    }

    /// Add or replace an operation attribute.
    ///
    /// Attribute names and values are validated immediately. Requiredness,
    /// schema types, multiplicity, and unknown-key policy are checked at build.
    pub fn set_attribute(
        &mut self,
        key: AttributeKey,
        value: AttributeValue,
    ) -> OperationBuildResult<&mut Self> {
        key.validate()
            .map_err(|e| OperationBuildError::InvalidAttribute(e.to_string()))?;

        value
            .validate()
            .map_err(|e| OperationBuildError::InvalidAttribute(e.to_string()))?;

        self.attributes.insert(key, value);
        Ok(self)
    }

    /// Reserve nested-region reference capacity.
    pub fn try_reserve_regions(
        &mut self,
        additional: usize,
    ) -> OperationBuildResult<()> {
        self.regions
            .try_reserve(additional)
            .map_err(|_| OperationBuildError::AllocationFailure {
                collection: "regions",
            })
    }

    /// Attach an existing canonical region.
    ///
    /// The region is borrowed only during construction; the final operation
    /// stores its RegionId. Ownership and reciprocal references are checked
    /// by the whole-IR verifier.
    pub fn add_region(
        &mut self,
        binding: RegionBinding<'schema>,
    ) -> OperationBuildResult<&mut Self> {
        let id = binding.id();

        if self.regions.iter().any(|existing| existing.id() == id) {
            return Err(OperationBuildError::DuplicateRegion(id));
        }

        self.regions.push(binding);
        Ok(self)
    }

    /// Replace the pending operands after local validation.
    pub fn set_operands(
        &mut self,
        operands: Vec<OperationOperand>,
    ) -> OperationBuildResult<&mut Self> {
        for operand in &operands {
            operand
                .validate()
                .map_err(|e| OperationBuildError::InvalidOperand(e.to_string()))?;
        }

        self.operands = operands;
        Ok(self)
    }

    /// Replace pending results after local uniqueness validation.
    pub fn set_results(
        &mut self,
        results: Vec<OperationValueResult>,
    ) -> OperationBuildResult<&mut Self> {
        let mut seen = std::collections::BTreeSet::<ValueId>::new();

        for result in &results {
            if !seen.insert(result.value()) {
                return Err(OperationBuildError::InvalidResult(format!(
                    "duplicate value identity {}",
                    result.value()
                )));
            }
        }

        self.results = results;
        Ok(self)
    }

    /// Validate schema cardinalities, operation identity, regions, attributes,
    /// and effects, then construct the canonical operation.
    ///
    /// Construction is transactional from the caller's perspective:
    /// an error returns no partially built operation.
    pub fn build(self) -> OperationBuildResult<ZuirOperation> {
        self.schema.validate()?;

        let expected_name = QualifiedName::parse(&format!(
            "{}::{}",
            self.schema.dialect(),
            self.schema.name()
        ))
        .map_err(|e| OperationBuildError::InvalidOperationName {
            expected: format!(
                "{}::{}",
                self.schema.dialect(),
                self.schema.name()
            ),
            actual: e.to_string(),
        })?;

        // Validate flattened operand/result/region cardinalities before
        // materializing the canonical IR node.
        validate_group_counts(
            "operand",
            self.schema.operands().iter().map(|g| {
                (g.name().canonical_string(), g.cardinality())
            }),
            self.operands.len(),
        )?;

        validate_group_counts(
            "result",
            self.schema.results().iter().map(|g| {
                (g.name().canonical_string(), g.cardinality())
            }),
            self.results.len(),
        )?;

        validate_group_counts(
            "region",
            self.schema.regions().iter().map(|g| {
                (g.name().canonical_string(), g.cardinality())
            }),
            self.regions.len(),
        )?;

        validate_operand_roles(self.schema, &self.operands)?;

        for binding in &self.regions {
            binding
                .region()
                .validate()
                .map_err(|e| OperationBuildError::InvalidRegion(e.to_string()))?;
        }

        validate_region_kinds(self.schema, &self.regions)?;
        validate_attributes(self.schema, &self.attributes)?;

        let declared_effects = self
            .schema
            .effects()
            .effects()
            .ok_or(OperationBuildError::UnknownEffectContract)?;

        let mut operation = ZuirOperation::new(self.id, expected_name)?;

        operation.set_operands(self.operands)?;
        operation.set_results(self.results)?;

        let region_ids = self
            .regions
            .iter()
            .map(RegionBinding::id)
            .collect::<Vec<_>>();

        operation.set_regions(region_ids)?;

        for (key, value) in self.attributes {
            operation.set_attribute(key, value)?;
        }

        operation.set_effects(declared_effects.clone())?;
        operation.validate()?;

        Ok(operation)
    }
}

// =============================================================================
// Shared validation helpers
// =============================================================================

use crate::zuir::dialect::operation::Cardinality;

/// Validate total flattened cardinality without introducing a machine ceiling.
///
/// This is an aggregate arity check. The authoritative verifier must also
/// check each group's detailed membership and schema-specific type rules.
fn validate_group_counts<I>(
    category: &'static str,
    groups: I,
    actual: usize,
) -> OperationBuildResult<()>
where
    I: IntoIterator<Item = (String, Cardinality)>,
{
    let mut minimum = 0usize;
    let mut maximum = Some(0usize);
    let mut names = Vec::new();

    for (name, cardinality) in groups {
        cardinality
            .validate()
            .map_err(OperationBuildError::from)?;

        minimum = minimum
            .checked_add(cardinality.minimum())
            .ok_or_else(|| OperationBuildError::InvalidSchema(
                format!("{category} minimum cardinality overflow")
            ))?;

        maximum = match (maximum, cardinality.maximum()) {
            (Some(current), Some(next)) => Some(
                current.checked_add(next).ok_or_else(|| {
                    OperationBuildError::InvalidSchema(format!(
                        "{category} maximum cardinality overflow"
                    ))
                })?
            ),
            _ => None,
        };

        names.push(name);
    }

    if actual < minimum || maximum.is_some_and(|limit| actual > limit) {
        return Err(OperationBuildError::Cardinality {
            group: format!("{category} groups [{}]", names.join(", ")),
            expected_minimum: minimum,
            expected_maximum: maximum,
            actual,
        });
    }

    Ok(())
}

/// Ensure operands with explicit roles use roles declared by the schema.
///
/// An unlabelled group is intentionally not assigned an invented role.
/// Detailed group-to-operand partitioning and schema type checking remain
/// the responsibility of the dialect-aware verifier.
fn validate_operand_roles(
    schema: &OperationDefinition,
    operands: &[OperationOperand],
) -> OperationBuildResult<()> {
    let declared_roles = schema
        .operands()
        .iter()
        .filter_map(|group| group.role())
        .collect::<std::collections::BTreeSet<_>>();

    for operand in operands {
        if let Some(role) = operand.role() {
            if !declared_roles.contains(role) {
                return Err(OperationBuildError::InvalidOperand(format!(
                    "undeclared operand role `{role}` for operation \
                     `{}::{}`",
                    schema.dialect(),
                    schema.name()
                )));
            }
        }
    }

    Ok(())
}

/// Check region-kind compatibility without taking ownership of regions.
///
/// Region order is retained. As with operands, exact group partitioning is
/// schema-specific and must be enforced by the authoritative verifier.
fn validate_region_kinds(
    schema: &OperationDefinition,
    regions: &[RegionBinding<'_>],
) -> OperationBuildResult<()> {
    for binding in regions {
        let actual = binding.region().kind().qualified_name();

        if !schema.regions().iter().any(|group| {
            group.kind() == actual
        }) {
            return Err(OperationBuildError::InvalidRegionKind {
                region: binding.id(),
                expected: schema
                    .regions()
                    .iter()
                    .map(|group| group.kind().canonical_string())
                    .collect::<Vec<_>>()
                    .join(" or "),
                actual: actual.canonical_string(),
            });
        }
    }

    Ok(())
}

/// Validate required attributes, unknown keys, multiplicity, and value types.
///
/// Concrete ZUIR operations currently store one value per key, so each key
/// maps to a single-entry AttributeSet here. Repeated-value attribute support
/// requires an explicit IR schema change, not silent truncation in this file.
fn validate_attributes(
    schema: &OperationDefinition,
    attributes: &BTreeMap<AttributeKey, AttributeValue>,
) -> OperationBuildResult<()> {
    let mut values = AttributeSet::new();

    for (key, value) in attributes {
        values
            .push(key.clone(), value.clone())
            .map_err(|e| OperationBuildError::InvalidAttribute(e.to_string()))?;
    }

    schema
        .attributes()
        .validate_values(&values)
        .map_err(|e| OperationBuildError::InvalidAttribute(e.to_string()))
}
