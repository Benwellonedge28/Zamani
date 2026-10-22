
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Abstract Data Layout
//!
//! Path: `src/hybrid/data/layout.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//!
//! ## Purpose
//!
//! Describes abstract data-layout requirements for values participating in
//! hybrid computation. A layout may describe known or symbolic dimensions,
//! rank, ordering, strides, alignment, and extensible domain-specific
//! metadata.
//!
//! This module specifies representation requirements. It does not allocate
//! memory, calculate physical addresses, materialize values, copy buffers,
//! select memory banks, or choose a device.
//!
//! ## Canonical ownership
//!
//! - `data::values` owns hybrid value references and canonical type references.
//! - The canonical semantic layer owns type meaning and type compatibility.
//! - Domain-specific IRs own domain-specific representation semantics.
//! - `data::layout` owns abstract layout descriptors and their structural
//!   validation.
//! - `data::exchange` owns cross-domain representation compatibility.
//! - `data::transfer` owns abstract transfer requirements.
//! - The compiler, backend, and runtime realize layouts on actual targets.
//!
//! A layout descriptor is not proof that the referenced type, value, or
//! layout entity exists. Resolution belongs to the authoritative subsystem.
//!
//! ## Scalability
//!
//! This module imposes no fixed tensor-rank, dimension-count, register-width,
//! memory-size, alignment-size, domain-count, or metadata-count ceiling.
//! Growable collections are used for dimensions, strides, and metadata.
//!
//! Symbolic expressions use a flat postfix instruction sequence rather than
//! a recursively nested expression tree. This avoids making expression depth
//! depend on recursive Rust data structures.
//!
//! Concrete representations are necessarily finite, and all operations
//! remain subject to available memory, address space, and explicitly
//! configured operational budgets. This module does not promise infinite
//! physical resources.
//!
//! ## Integration
//!
//! - Uses `HybridError` for structural validation failures.
//! - Uses `CanonicalEntityRef` and `CanonicalTypeRef` from `data::values`.
//! - Does not depend on frontend AST, compiler, quantum IR, backend, runtime,
//!   hardware discovery, or vendor-specific implementations.
//! - Exchange and planning code should consume validated descriptors rather
//!   than introduce duplicate layout types.
//!
//! ## Serialization
//!
//! Deserialization passes through validating conversions. Structural
//! validation does not resolve canonical references or establish that a
//! target can realize a declared layout.
//!
//! ## Grammar conformance
//!
//! This file implements abstract representation contracts required by
//! `grammar/spec/hybrid.md` and `grammar/hybrid/README.md`. It does not
//! introduce source-language syntax or replace the grammar's authority.

use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use crate::hybrid::errors::{HybridError, HybridErrorCode};
use crate::hybrid::data::values::{CanonicalEntityRef, CanonicalTypeRef};

const COMPONENT: &str = "hybrid data layout";

/// Construct a structured layout validation error.
fn invalid_layout(message: impl Into<String>) -> HybridError {
    HybridError::new(HybridErrorCode::InvalidRepresentation, message)
        .in_component(COMPONENT)
}

/// Construct a structured reference validation error.
fn invalid_reference(message: impl Into<String>) -> HybridError {
    HybridError::new(HybridErrorCode::InvalidReference, message)
        .in_component(COMPONENT)
}

/// Validate a namespaced metadata identifier.
///
/// Identifiers must be non-empty and must not contain control characters.
/// No fixed length limit is imposed.
fn validate_identifier(
    description: &str,
    value: &str,
) -> Result<(), HybridError> {
    if value.trim().is_empty() {
        return Err(invalid_layout(format!(
            "{description} must not be empty"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_layout(format!(
            "{description} must not contain control characters"
        )));
    }

    Ok(())
}

/// A key for extensible layout metadata.
///
/// Namespaces prevent unrelated domains from accidentally assigning the
/// same meaning to a metadata name. The key is an identifier, not a
/// hardware-specific instruction.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(try_from = "NamespacedKeyWire")]
pub struct NamespacedKey {
    namespace: String,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct NamespacedKeyWire {
    namespace: String,
    name: String,
}

impl NamespacedKey {
    /// Construct a validated metadata key.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, HybridError> {
        let key = Self {
            namespace: namespace.into(),
            name: name.into(),
        };
        key.validate()?;
        Ok(key)
    }

    /// Return the metadata namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the metadata name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Validate the key's structure.
    pub fn validate(&self) -> Result<(), HybridError> {
        validate_identifier("metadata namespace", &self.namespace)?;
        validate_identifier("metadata name", &self.name)
    }
}

impl TryFrom<NamespacedKeyWire> for NamespacedKey {
    type Error = HybridError;

    fn try_from(value: NamespacedKeyWire) -> Result<Self, Self::Error> {
        Self::new(value.namespace, value.name)
    }
}

/// A symbolic dimension expression in postfix form.
///
/// Each instruction consumes zero, one, or two operands from an abstract
/// evaluation stack and optionally pushes one result. A valid expression
/// finishes with exactly one result.
///
/// Postfix encoding supports arbitrary expression length without a
/// recursively nested Rust expression tree. It does not evaluate the
/// expression or prove that all runtime operands will be available.
///
/// Arithmetic semantics, overflow behavior, and the validity of runtime
/// dimension values must be established by the authoritative semantic
/// system and the domain adapter that realizes the expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(try_from = "DimensionExprWire")]
pub struct DimensionExpr {
    instructions: Vec<DimensionInstruction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct DimensionExprWire {
    instructions: Vec<DimensionInstruction>,
}

/// One instruction in a symbolic dimension expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DimensionInstruction {
    /// Push a non-negative integer constant.
    Constant(u128),

    /// Push a symbolic dimension supplied by a canonical authority.
    Symbol {
        namespace: String,
        name: String,
    },

    /// Push a runtime dimension obtained from a canonical entity.
    RuntimeValue(CanonicalEntityRef),

    /// Add the top two operands.
    Add,

    /// Multiply the top two operands.
    Multiply,

    /// Subtract the top operand from the preceding operand.
    ///
    /// The expression's evaluator must reject an invalid negative result.
    Subtract,

    /// Divide the preceding operand by the top operand, rounding upward.
    ///
    /// Division by zero is invalid.
    CeilDivide,

    /// Push the smaller of the top two operands.
    Minimum,

    /// Push the larger of the top two operands.
    Maximum,
}

impl DimensionExpr {
    /// Construct and validate a symbolic dimension expression.
    pub fn new(
        instructions: Vec<DimensionInstruction>,
    ) -> Result<Self, HybridError> {
        let expression = Self { instructions };
        expression.validate()?;
        Ok(expression)
    }

    /// Return the postfix instruction sequence.
    #[must_use]
    pub fn instructions(&self) -> &[DimensionInstruction] {
        &self.instructions
    }

    /// Validate instruction operands and expression stack structure.
    ///
    /// This method does not evaluate arithmetic or resolve symbolic names.
    pub fn validate(&self) -> Result<(), HybridError> {
        if self.instructions.is_empty() {
            return Err(invalid_layout(
                "a symbolic dimension expression must contain instructions",
            ));
        }

        // The stack depth is tracked as a count, not represented by a
        // fixed-size stack or an arbitrary expression-depth ceiling.
        let mut depth: usize = 0;

        for instruction in &self.instructions {
            match instruction {
                DimensionInstruction::Constant(_) => {
                    depth = depth.checked_add(1).ok_or_else(|| {
                        invalid_layout("dimension expression stack count overflow")
                    })?;
                }

                DimensionInstruction::Symbol { namespace, name } => {
                    validate_identifier("dimension symbol namespace", namespace)?;
                    validate_identifier("dimension symbol name", name)?;

                    depth = depth.checked_add(1).ok_or_else(|| {
                        invalid_layout("dimension expression stack count overflow")
                    })?;
                }

                DimensionInstruction::RuntimeValue(reference) => {
                    reference.validate().map_err(|error| {
                        error.in_component(COMPONENT)
                    })?;

                    depth = depth.checked_add(1).ok_or_else(|| {
                        invalid_layout("dimension expression stack count overflow")
                    })?;
                }

                DimensionInstruction::Add
                | DimensionInstruction::Multiply
                | DimensionInstruction::Subtract
                | DimensionInstruction::CeilDivide
                | DimensionInstruction::Minimum
                | DimensionInstruction::Maximum => {
                    if depth < 2 {
                        return Err(invalid_layout(
                            "binary dimension instruction has fewer than two operands",
                        ));
                    }

                    // Two operands are replaced by one result.
                    depth -= 1;
                }
            }
        }

        if depth != 1 {
            return Err(invalid_layout(format!(
                "dimension expression must finish with one result; found {depth}"
            )));
        }

        Ok(())
    }
}

impl TryFrom<DimensionExprWire> for DimensionExpr {
    type Error = HybridError;

    fn try_from(value: DimensionExprWire) -> Result<Self, Self::Error> {
        Self::new(value.instructions)
    }
}

/// One abstract dimension.
///
/// `Known` is a constant dimension; `Symbolic` is an expression; `Runtime`
/// refers to a dimension value whose actual value is supplied by a canonical
/// authority. A runtime reference is not proof that the referenced entity
/// exists or that it evaluates to a valid dimension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(try_from = "DimensionWire")]
pub enum Dimension {
    /// A statically known non-negative dimension.
    Known(u128),

    /// A symbolic dimension expression.
    Symbolic(DimensionExpr),

    /// A dimension resolved from a canonical runtime or semantic entity.
    Runtime(CanonicalEntityRef),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
enum DimensionWire {
    Known(u128),
    Symbolic(DimensionExpr),
    Runtime(CanonicalEntityRef),
}

impl Dimension {
    /// Create a known dimension.
    #[must_use]
    pub const fn known(value: u128) -> Self {
        Self::Known(value)
    }

    /// Create a symbolic dimension from a validated expression.
    pub fn symbolic(
        expression: DimensionExpr,
    ) -> Result<Self, HybridError> {
        expression.validate()?;
        Ok(Self::Symbolic(expression))
    }

    /// Create a dimension represented by a canonical entity reference.
    pub fn runtime(
        reference: CanonicalEntityRef,
    ) -> Result<Self, HybridError> {
        reference.validate()?;
        Ok(Self::Runtime(reference))
    }

    /// Validate the dimension.
    pub fn validate(&self) -> Result<(), HybridError> {
        match self {
            Self::Known(_) => Ok(()),
            Self::Symbolic(expression) => expression.validate(),
            Self::Runtime(reference) => reference.validate().map_err(|error| {
                error.in_component(COMPONENT)
            }),
        }
    }
}

impl TryFrom<DimensionWire> for Dimension {
    type Error = HybridError;

    fn try_from(value: DimensionWire) -> Result<Self, Self::Error> {
        match value {
            DimensionWire::Known(value) => Ok(Self::Known(value)),
            DimensionWire::Symbolic(expression) => Self::symbolic(expression),
            DimensionWire::Runtime(reference) => Self::runtime(reference),
        }
    }
}

/// A declaration of the rank of a value's abstract shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(try_from = "RankSpecWire")]
pub enum RankSpec {
    /// Rank and each dimension are represented explicitly.
    Known(Vec<Dimension>),

    /// Rank is symbolic and must be resolved by a canonical authority.
    Symbolic(DimensionExpr),

    /// The layout does not constrain rank.
    ///
    /// This does not imply that every rank is valid for the referenced type.
    Unspecified,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
enum RankSpecWire {
    Known(Vec<Dimension>),
    Symbolic(DimensionExpr),
    Unspecified,
}

impl RankSpec {
    /// Create a known-rank shape.
    pub fn known(dimensions: Vec<Dimension>) -> Result<Self, HybridError> {
        let rank = Self::Known(dimensions);
        rank.validate()?;
        Ok(rank)
    }

    /// Create a symbolic-rank declaration.
    pub fn symbolic(expression: DimensionExpr) -> Result<Self, HybridError> {
        expression.validate()?;
        Ok(Self::Symbolic(expression))
    }

    /// Return known dimensions, if rank is statically represented.
    #[must_use]
    pub fn dimensions(&self) -> Option<&[Dimension]> {
        match self {
            Self::Known(dimensions) => Some(dimensions),
            Self::Symbolic(_) | Self::Unspecified => None,
        }
    }

    /// Return whether the rank is statically represented.
    #[must_use]
    pub const fn is_known(&self) -> bool {
        matches!(self, Self::Known(_))
    }

    /// Validate the rank and all known dimensions.
    pub fn validate(&self) -> Result<(), HybridError> {
        match self {
            Self::Known(dimensions) => {
                for dimension in dimensions {
                    dimension.validate()?;
                }
                Ok(())
            }
            Self::Symbolic(expression) => expression.validate(),
            Self::Unspecified => Ok(()),
        }
    }
}

impl TryFrom<RankSpecWire> for RankSpec {
    type Error = HybridError;

    fn try_from(value: RankSpecWire) -> Result<Self, Self::Error> {
        match value {
            RankSpecWire::Known(dimensions) => Self::known(dimensions),
            RankSpecWire::Symbolic(expression) => Self::symbolic(expression),
            RankSpecWire::Unspecified => Ok(Self::Unspecified),
        }
    }
}

/// The ordering of dimensions in a representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(try_from = "DimensionOrderWire")]
pub enum DimensionOrder {
    /// The first dimension varies slowest in the abstract linear ordering.
    RowMajor,

    /// The first dimension varies fastest in the abstract linear ordering.
    ColumnMajor,

    /// An explicit permutation describing dimension traversal order.
    ///
    /// Entries are zero-based dimension indices. This variant can be
    /// structurally validated only when the rank is known.
    Explicit(Vec<u64>),

    /// Ordering is defined by a canonical domain-specific layout entity.
    DomainDefined(CanonicalEntityRef),

    /// The descriptor makes no ordering assertion.
    Unspecified,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
enum DimensionOrderWire {
    RowMajor,
    ColumnMajor,
    Explicit(Vec<u64>),
    DomainDefined(CanonicalEntityRef),
    Unspecified,
}

impl DimensionOrder {
    /// Validate this ordering independently of a rank.
    pub fn validate(&self) -> Result<(), HybridError> {
        match self {
            Self::DomainDefined(reference) => {
                reference.validate().map_err(|error| {
                    error.in_component(COMPONENT)
                })
            }
            Self::RowMajor
            | Self::ColumnMajor
            | Self::Explicit(_)
            | Self::Unspecified => Ok(()),
        }
    }

    fn validate_for_rank(&self, dimensions: &[Dimension]) -> Result<(), HybridError> {
        self.validate()?;

        let Self::Explicit(order) = self else {
            return Ok(());
        };

        let rank = u64::try_from(dimensions.len()).map_err(|_| {
            invalid_layout("known rank cannot be represented by the ordering index type")
        })?;

        if order.len() != dimensions.len() {
            return Err(invalid_layout(format!(
                "explicit dimension order contains {} entries for rank {}",
                order.len(),
                dimensions.len()
            )));
        }

        let mut seen = BTreeSet::new();

        for index in order {
            if *index >= rank {
                return Err(invalid_layout(format!(
                    "explicit dimension index {index} is outside rank {rank}"
                )));
            }

            if !seen.insert(*index) {
                return Err(invalid_layout(format!(
                    "explicit dimension order repeats index {index}"
                )));
            }
        }

        Ok(())
    }
}

impl TryFrom<DimensionOrderWire> for DimensionOrder {
    type Error = HybridError;

    fn try_from(value: DimensionOrderWire) -> Result<Self, Self::Error> {
        let order = match value {
            DimensionOrderWire::RowMajor => Self::RowMajor,
            DimensionOrderWire::ColumnMajor => Self::ColumnMajor,
            DimensionOrderWire::Explicit(indices) => Self::Explicit(indices),
            DimensionOrderWire::DomainDefined(reference) => {
                Self::DomainDefined(reference)
            }
            DimensionOrderWire::Unspecified => Self::Unspecified,
        };

        order.validate()?;
        Ok(order)
    }
}

/// Abstract mapping from logical dimensions to a representation.
///
/// These are declarative choices, not physical address calculations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(try_from = "LayoutMappingWire")]
pub enum LayoutMapping {
    /// A dense representation using the declared dimension order.
    Dense {
        order: DimensionOrder,
    },

    /// A strided representation.
    ///
    /// Each stride is a symbolic element-step expression. The vector length
    /// must match a statically known rank. Stride units are logical element
    /// steps, not byte addresses.
    Strided {
        strides: Vec<DimensionExpr>,
    },

    /// A mapping whose meaning is defined by a canonical layout entity.
    Opaque(CanonicalEntityRef),

    /// No mapping assertion is made.
    Unspecified,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
enum LayoutMappingWire {
    Dense {
        order: DimensionOrder,
    },
    Strided {
        strides: Vec<DimensionExpr>,
    },
    Opaque(CanonicalEntityRef),
    Unspecified,
}

impl LayoutMapping {
    /// Validate a mapping independently of rank.
    pub fn validate(&self) -> Result<(), HybridError> {
        match self {
            Self::Dense { order } => order.validate(),
            Self::Strided { strides } => {
                for stride in strides {
                    stride.validate()?;
                }
                Ok(())
            }
            Self::Opaque(reference) => reference.validate().map_err(|error| {
                error.in_component(COMPONENT)
            }),
            Self::Unspecified => Ok(()),
        }
    }

    fn validate_for_rank(&self, rank: &RankSpec) -> Result<(), HybridError> {
        self.validate()?;

        match (self, rank.dimensions()) {
            (Self::Dense { order }, Some(dimensions)) => {
                order.validate_for_rank(dimensions)
            }
            (Self::Dense { order }, None) => {
                if matches!(order, DimensionOrder::Explicit(_)) {
                    return Err(invalid_layout(
                        "explicit dimension ordering requires a known rank",
                    ));
                }
                Ok(())
            }
            (Self::Strided { strides }, Some(dimensions)) => {
                if strides.len() != dimensions.len() {
                    return Err(invalid_layout(format!(
                        "strided mapping has {} strides for rank {}",
                        strides.len(),
                        dimensions.len()
                    )));
                }
                Ok(())
            }
            (Self::Strided { .. }, None) => Err(invalid_layout(
                "strided mapping requires a statically known rank; use a domain-defined or unspecified mapping when rank is unresolved",
            )),
            (Self::Opaque(_), _) | (Self::Unspecified, _) => Ok(()),
        }
    }
}

impl TryFrom<LayoutMappingWire> for LayoutMapping {
    type Error = HybridError;

    fn try_from(value: LayoutMappingWire) -> Result<Self, Self::Error> {
        let mapping = match value {
            LayoutMappingWire::Dense { order } => Self::Dense { order },
            LayoutMappingWire::Strided { strides } => Self::Strided { strides },
            LayoutMappingWire::Opaque(reference) => Self::Opaque(reference),
            LayoutMappingWire::Unspecified => Self::Unspecified,
        };

        mapping.validate()?;
        Ok(mapping)
    }
}

/// An abstract alignment requirement.
///
/// The values describe a requested guarantee, not an actual memory address.
/// A downstream representation adapter must establish whether its target can
/// satisfy the requirement.
///
/// Both values use `u128` to avoid unnecessarily narrowing the public
/// contract to the native pointer width. A target may still have narrower
/// limits, which must be reported by the relevant realization layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(try_from = "AlignmentConstraintWire")]
pub struct AlignmentConstraint {
    minimum_bytes: u128,
    multiple_bytes: Option<u128>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct AlignmentConstraintWire {
    minimum_bytes: u128,
    multiple_bytes: Option<u128>,
}

impl AlignmentConstraint {
    /// Construct an alignment requirement.
    ///
    /// `minimum_bytes` must be non-zero. If `multiple_bytes` is supplied,
    /// it must also be non-zero. The multiple is an additional abstract
    /// constraint; this constructor does not claim a target can satisfy it.
    pub fn new(
        minimum_bytes: u128,
        multiple_bytes: Option<u128>,
    ) -> Result<Self, HybridError> {
        let alignment = Self {
            minimum_bytes,
            multiple_bytes,
        };
        alignment.validate()?;
        Ok(alignment)
    }

    /// Return the minimum required alignment in bytes.
    #[must_use]
    pub const fn minimum_bytes(&self) -> u128 {
        self.minimum_bytes
    }

    /// Return the optional required alignment multiple in bytes.
    #[must_use]
    pub const fn multiple_bytes(&self) -> Option<u128> {
        self.multiple_bytes
    }

    /// Validate this alignment declaration.
    pub fn validate(&self) -> Result<(), HybridError> {
        if self.minimum_bytes == 0 {
            return Err(invalid_layout(
                "minimum alignment must be greater than zero",
            ));
        }

        if self.multiple_bytes == Some(0) {
            return Err(invalid_layout(
                "alignment multiple must be greater than zero when specified",
            ));
        }

        Ok(())
    }
}

impl TryFrom<AlignmentConstraintWire> for AlignmentConstraint {
    type Error = HybridError;

    fn try_from(value: AlignmentConstraintWire) -> Result<Self, Self::Error> {
        Self::new(value.minimum_bytes, value.multiple_bytes)
    }
}

/// Abstract byte-order requirement.
///
/// This is meaningful only for representations whose canonical domain
/// contract assigns a byte order. It does not define a universal encoding
/// for quantum states, opaque handles, or distributed objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
    NativeOrDomainDefined,
}

/// A validated abstract data-layout descriptor.
///
/// The descriptor refers to a canonical semantic type and optionally to a
/// canonical value. It does not define either entity or duplicate its payload.
///
/// All fields are private to preserve constructor and deserialization
/// validation. New layout requirements should be added through explicit,
/// backward-compatible fields and schema-version policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(try_from = "DataLayoutWire")]
pub struct DataLayout {
    layout_ref: CanonicalEntityRef,
    semantic_type: CanonicalTypeRef,
    value_ref: Option<CanonicalEntityRef>,
    rank: RankSpec,
    mapping: LayoutMapping,
    alignment: Option<AlignmentConstraint>,
    byte_order: Option<ByteOrder>,
    metadata: std::collections::BTreeMap<NamespacedKey, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct DataLayoutWire {
    layout_ref: CanonicalEntityRef,
    semantic_type: CanonicalTypeRef,
    value_ref: Option<CanonicalEntityRef>,
    rank: RankSpec,
    mapping: LayoutMapping,
    alignment: Option<AlignmentConstraint>,
    byte_order: Option<ByteOrder>,
    metadata: std::collections::BTreeMap<NamespacedKey, String>,
}

impl DataLayout {
    /// Construct and validate an abstract layout descriptor.
    ///
    /// `layout_ref` is the identity of this layout contract in a canonical
    /// namespace. `semantic_type` is a reference to the authoritative type.
    /// `value_ref`, when present, identifies a canonical value to which the
    /// descriptor applies.
    ///
    /// The constructor performs structural checks only. It does not resolve
    /// canonical references or establish target feasibility.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        layout_ref: CanonicalEntityRef,
        semantic_type: CanonicalTypeRef,
        value_ref: Option<CanonicalEntityRef>,
        rank: RankSpec,
        mapping: LayoutMapping,
        alignment: Option<AlignmentConstraint>,
        byte_order: Option<ByteOrder>,
        metadata: std::collections::BTreeMap<NamespacedKey, String>,
    ) -> Result<Self, HybridError> {
        let layout = Self {
            layout_ref,
            semantic_type,
            value_ref,
            rank,
            mapping,
            alignment,
            byte_order,
            metadata,
        };

        layout.validate()?;
        Ok(layout)
    }

    /// Return the canonical identity of this layout contract.
    #[must_use]
    pub fn layout_ref(&self) -> &CanonicalEntityRef {
        &self.layout_ref
    }

    /// Return the authoritative semantic type reference.
    #[must_use]
    pub fn semantic_type(&self) -> &CanonicalTypeRef {
        &self.semantic_type
    }

    /// Return the optional canonical value to which this layout applies.
    #[must_use]
    pub fn value_ref(&self) -> Option<&CanonicalEntityRef> {
        self.value_ref.as_ref()
    }

    /// Return the abstract rank/shape requirement.
    #[must_use]
    pub fn rank(&self) -> &RankSpec {
        &self.rank
    }

    /// Return the abstract representation mapping.
    #[must_use]
    pub fn mapping(&self) -> &LayoutMapping {
        &self.mapping
    }

    /// Return the optional alignment requirement.
    #[must_use]
    pub fn alignment(&self) -> Option<&AlignmentConstraint> {
        self.alignment.as_ref()
    }

    /// Return the optional byte-order requirement.
    #[must_use]
    pub const fn byte_order(&self) -> Option<ByteOrder> {
        self.byte_order
    }

    /// Return namespaced domain-specific metadata.
    ///
    /// The map is ordered to provide deterministic iteration and serialization
    /// when the selected serializer preserves map iteration order.
    #[must_use]
    pub fn metadata(
        &self,
    ) -> &std::collections::BTreeMap<NamespacedKey, String> {
        &self.metadata
    }

    /// Validate the descriptor's internal structure.
    ///
    /// Reference resolution, semantic type compatibility, and target
    /// feasibility belong to their authoritative integration layers.
    pub fn validate(&self) -> Result<(), HybridError> {
        self.layout_ref
            .validate()
            .map_err(|error| error.in_component(COMPONENT))?;

        self.semantic_type
            .validate()
            .map_err(|error| error.in_component(COMPONENT))?;

        if let Some(value_ref) = &self.value_ref {
            value_ref
                .validate()
                .map_err(|error| error.in_component(COMPONENT))?;
        }

        self.rank.validate()?;
        self.mapping.validate_for_rank(&self.rank)?;

        if let Some(alignment) = &self.alignment {
            alignment.validate()?;
        }

        for (key, value) in &self.metadata {
            key.validate()?;

            if value.chars().any(char::is_control) {
                return Err(invalid_layout(format!(
                    "metadata value for '{}::{}' contains control characters",
                    key.namespace(),
                    key.name()
                )));
            }
        }

        Ok(())
    }

    /// Check that this descriptor refers to the supplied canonical type.
    ///
    /// This compares canonical references only. It does not infer type
    /// equivalence or prove that a target representation is compatible.
    pub fn validate_type_reference(
        &self,
        semantic_type: &CanonicalTypeRef,
    ) -> Result<(), HybridError> {
        self.validate()?;
        semantic_type.validate()?;

        if &self.semantic_type != semantic_type {
            return Err(HybridError::new(
                HybridErrorCode::ValueIncompatibility,
                format!(
                    "layout type '{}' does not match supplied canonical type '{}'",
                    self.semantic_type.display_name(),
                    semantic_type.display_name()
                ),
            )
            .in_component(COMPONENT));
        }

        Ok(())
    }
}

impl TryFrom<DataLayoutWire> for DataLayout {
    type Error = HybridError;

    fn try_from(value: DataLayoutWire) -> Result<Self, Self::Error> {
        Self::new(
            value.layout_ref,
            value.semantic_type,
            value.value_ref,
            value.rank,
            value.mapping,
            value.alignment,
            value.byte_order,
            value.metadata,
        )
    }
}

impl fmt::Display for NamespacedKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}::{}", self.namespace, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entity(
        namespace: &str,
        kind: &str,
        identifier: &str,
    ) -> CanonicalEntityRef {
        CanonicalEntityRef::new(namespace, kind, identifier)
            .expect("valid test entity reference")
    }

    fn semantic_type() -> CanonicalTypeRef {
        CanonicalTypeRef::new("semantic", "tensor")
            .expect("valid test type reference")
    }

    fn layout_ref() -> CanonicalEntityRef {
        entity("hybrid.data", "layout", "layout-1")
    }

    #[test]
    fn accepts_known_rank_with_dense_mapping() {
        let rank = RankSpec::known(vec![
            Dimension::known(4),
            Dimension::known(8),
        ])
        .expect("valid rank");

        let layout = DataLayout::new(
            layout_ref(),
            semantic_type(),
            None,
            rank,
            LayoutMapping::Dense {
                order: DimensionOrder::RowMajor,
            },
            Some(
                AlignmentConstraint::new(16, Some(16))
                    .expect("valid alignment"),
            ),
            Some(ByteOrder::LittleEndian),
            std::collections::BTreeMap::new(),
        )
        .expect("valid layout");

        assert!(layout.validate().is_ok());
    }

    #[test]
    fn accepts_symbolic_dimensions_without_evaluating_them() {
        let expression = DimensionExpr::new(vec![
            DimensionInstruction::Symbol {
                namespace: "semantic".to_owned(),
                name: "batch".to_owned(),
            },
            DimensionInstruction::Constant(2),
            DimensionInstruction::Multiply,
        ])
        .expect("valid expression");

        let rank = RankSpec::known(vec![
            Dimension::symbolic(expression).expect("valid symbolic dimension"),
        ])
        .expect("valid rank");

        let layout = DataLayout::new(
            layout_ref(),
            semantic_type(),
            None,
            rank,
            LayoutMapping::Unspecified,
            None,
            None,
            std::collections::BTreeMap::new(),
        )
        .expect("valid symbolic layout");

        assert!(layout.validate().is_ok());
    }

    #[test]
    fn rejects_empty_symbolic_expression() {
        let result = DimensionExpr::new(Vec::new());
        assert!(result.is_err());
    }

    #[test]
    fn rejects_symbolic_expression_with_missing_operand() {
        let result = DimensionExpr::new(vec![
            DimensionInstruction::Constant(1),
            DimensionInstruction::Add,
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_symbolic_expression_with_multiple_results() {
        let result = DimensionExpr::new(vec![
            DimensionInstruction::Constant(1),
            DimensionInstruction::Constant(2),
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_zero_alignment() {
        assert!(AlignmentConstraint::new(0, None).is_err());
        assert!(AlignmentConstraint::new(1, Some(0)).is_err());
    }

    #[test]
    fn rejects_strides_that_do_not_match_known_rank() {
        let rank = RankSpec::known(vec![
            Dimension::known(3),
            Dimension::known(5),
        ])
        .expect("valid rank");

        let layout = DataLayout::new(
            layout_ref(),
            semantic_type(),
            None,
            rank,
            LayoutMapping::Strided {
                strides: vec![
                    DimensionExpr::new(vec![
                        DimensionInstruction::Constant(1),
                    ])
                    .expect("valid stride"),
                ],
            },
            None,
            None,
            std::collections::BTreeMap::new(),
        );

        assert!(layout.is_err());
    }

    #[test]
    fn rejects_duplicate_explicit_dimension_order() {
        let dimensions = vec![
            Dimension::known(2),
            Dimension::known(3),
        ];

        let order = DimensionOrder::Explicit(vec![0, 0]);
        assert!(order.validate_for_rank(&dimensions).is_err());
    }

    #[test]
    fn rejects_explicit_order_outside_known_rank() {
        let dimensions = vec![Dimension::known(2)];
        let order = DimensionOrder::Explicit(vec![1]);

        assert!(order.validate_for_rank(&dimensions).is_err());
    }

    #[test]
    fn rejects_explicit_order_with_symbolic_rank() {
        let expression = DimensionExpr::new(vec![
            DimensionInstruction::Symbol {
                namespace: "semantic".to_owned(),
                name: "rank".to_owned(),
            },
        ])
        .expect("valid rank expression");

        let layout = DataLayout::new(
            layout_ref(),
            semantic_type(),
            None,
            RankSpec::symbolic(expression).expect("valid symbolic rank"),
            LayoutMapping::Dense {
                order: DimensionOrder::Explicit(vec![0]),
            },
            None,
            None,
            std::collections::BTreeMap::new(),
        );

        assert!(layout.is_err());
    }

    #[test]
    fn rejects_control_characters_in_metadata_values() {
        let mut metadata = std::collections::BTreeMap::new();
        metadata.insert(
            NamespacedKey::new("domain.example", "format")
                .expect("valid metadata key"),
            "bad\nvalue".to_owned(),
        );

        let layout = DataLayout::new(
            layout_ref(),
            semantic_type(),
            None,
            RankSpec::Unspecified,
            LayoutMapping::Unspecified,
            None,
            None,
            metadata,
        );

        assert!(layout.is_err());
    }

    #[test]
    fn accepts_extensible_namespaced_metadata() {
        let mut metadata = std::collections::BTreeMap::new();
        metadata.insert(
            NamespacedKey::new("future.domain", "layout-policy")
                .expect("valid metadata key"),
            "symbolic".to_owned(),
        );

        let layout = DataLayout::new(
            layout_ref(),
            semantic_type(),
            None,
            RankSpec::Unspecified,
            LayoutMapping::Unspecified,
            None,
            None,
            metadata,
        )
        .expect("valid extensible metadata");

        assert_eq!(layout.metadata().len(), 1);
    }

    #[test]
    fn rejects_mismatched_canonical_type_reference() {
        let layout = DataLayout::new(
            layout_ref(),
            semantic_type(),
            None,
            RankSpec::Unspecified,
            LayoutMapping::Unspecified,
            None,
            None,
            std::collections::BTreeMap::new(),
        )
        .expect("valid layout");

        let other_type = CanonicalTypeRef::new("semantic", "scalar")
            .expect("valid type reference");

        assert!(layout.validate_type_reference(&other_type).is_err());
    }

    #[test]
    fn supports_empty_known_rank() {
        // Rank zero can describe a scalar-like shape when the canonical type
        // authority permits it. This module does not impose type semantics.
        let rank = RankSpec::known(Vec::new()).expect("valid rank");

        let layout = DataLayout::new(
            layout_ref(),
            semantic_type(),
            None,
            rank,
            LayoutMapping::Dense {
                order: DimensionOrder::RowMajor,
            },
            None,
            None,
            std::collections::BTreeMap::new(),
        )
        .expect("valid rank-zero layout");

        assert_eq!(layout.rank().dimensions(), Some([].as_slice()));
    }
}
