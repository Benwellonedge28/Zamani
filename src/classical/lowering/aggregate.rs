
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! Target-independent aggregate lowering plans for Zamani.
//!
//! Path: `src/classical/lowering/aggregate.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Convert classical aggregate type descriptors into a deterministic,
//! target-independent plan suitable for a later canonical-IR adapter.
//!
//! Supported descriptor categories are defined by
//! `crate::classical::types::aggregate::AggregateKind`.
//!
//! # Authority and ownership
//!
//! The grammar and normative specifications define language meaning.
//! The canonical frontend semantic model owns resolved source semantics.
//! `crate::classical::types::aggregate` owns the aggregate descriptors.
//! `crate::classical::types::scalar` owns scalar descriptors.
//! `crate::quantum::ir::classical` remains the canonical classical IR
//! boundary. ZUIR owns its canonical identifiers and IR representation.
//!
//! This file owns only the lowering-plan representation, structural
//! traversal, explicit work accounting, and plan-local validation.
//!
//! # Important boundary
//!
//! This module does NOT:
//!
//! - parse source code or resolve names;
//! - prove that named types exist or that recursive declarations are legal;
//! - replace canonical semantic types;
//! - define a second IR or allocate canonical IR identities;
//! - choose a target, ABI, physical layout, field offsets, or alignment;
//! - allocate program data or runtime memory;
//! - select a scheduler, execution strategy, or hardware backend;
//! - claim that an aggregate can be represented by every target.
//!
//! A successful plan is not proof of semantic validity, target feasibility,
//! concrete layout, or canonical-IR verification.
//!
//! # Scalability
//!
//! Traversal is iterative rather than recursively calling the Rust stack.
//! There is no fixed maximum field count, variant count, or nesting depth.
//! Optional work limits are explicit operational policy, not language
//! semantic restrictions. Actual memory, address-space, and allocator
//! limits still apply.
//!
//! # Integration contract
//!
//! 1. The frontend and semantic analyzer must validate source meaning,
//!    names, generic substitutions, recursion, and member types first.
//! 2. Convert validated semantic aggregate information into the existing
//!    `AggregateTypeDescriptor` adapter, without treating this descriptor
//!    as the canonical semantic type.
//! 3. Build a plan with [`AggregateLoweringPlan::build`].
//! 4. Resolve every reported obligation through the applicable semantic,
//!    layout, capability, or target-policy subsystem.
//! 5. An adapter in the canonical IR owner must consume the plan, construct
//!    canonical IR entities, and invoke the canonical verifier.
//! 6. Do not create a second classical IR or duplicate canonical ID types.
//!
//! `src/classical/lowering/mod.rs` must declare `pub mod aggregate;` after
//! this file is added. The parent `src/classical/mod.rs` must also be repaired
//! to register the existing classical modules. Neither parent file is
//! replaced by this implementation.

use core::fmt;
use std::collections::BTreeSet;
use std::error::Error;

use crate::classical::types::aggregate::{
    AggregateField, AggregateKind, AggregateMemberType, AggregateTypeDescriptor,
    AggregateVariant,
};
use crate::classical::types::scalar::ScalarTypeDescriptor;

/// Result type for aggregate-plan construction and validation.
pub type AggregateLoweringResult<T> = Result<T, AggregateLoweringError>;

/// An index into the flat aggregate-node table of one lowering plan.
///
/// This is a plan-local reference, not a canonical IR identifier, memory
/// address, declaration ID, or stable cross-compilation identity.
pub type AggregateNodeIndex = usize;

/// Explicit operational policy for aggregate-plan construction.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AggregateLoweringOptions {
    /// Optional maximum number of charged plan-building work units.
    ///
    /// One unit is charged for each visited aggregate descriptor and each
    /// field or variant payload member inspected. `None` means that this
    /// policy imposes no work-unit limit. It does not guarantee unlimited
    /// memory, time, or system resources.
    pub work_budget: Option<u64>,
}

/// A requirement that must be discharged before target realization.
///
/// These obligations preserve unresolved decisions instead of guessing
/// target-specific answers during target-independent planning.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum AggregateLoweringObligation {
    /// Resolve a named type through the canonical semantic environment.
    ResolveNamedType {
        /// Exact name supplied by the aggregate descriptor.
        name: String,
    },

    /// Apply a target or language policy to determine physical layout.
    ResolveRepresentationLayout,

    /// Establish the safety and representation contract for an untagged
    /// union before lowering it to a concrete target.
    ResolveUntaggedUnionPolicy,

    /// Resolve the semantics of an extension-defined aggregate kind.
    ResolveExtensionSemantics {
        /// Extension-defined aggregate category.
        name: String,
    },
}

/// A member type recorded in the flattened lowering plan.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AggregateMemberPlan {
    /// Scalar descriptor whose canonical semantic meaning is owned elsewhere.
    Scalar(ScalarTypeDescriptor),

    /// Reference to a nested aggregate node in this plan.
    Nested {
        /// Index of the nested node in [`AggregateLoweringPlan::nodes`].
        node: AggregateNodeIndex,
    },

    /// Unresolved named type reference.
    ///
    /// The reference must be resolved by the canonical semantic environment.
    Named {
        /// Exact qualified name retained from the descriptor.
        name: String,
    },
}

/// A field or positional element in declaration order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AggregateFieldPlan {
    name: Option<String>,
    member: AggregateMemberPlan,
}

impl AggregateFieldPlan {
    /// Returns the exact field name, if present.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the member's planned type reference.
    #[must_use]
    pub fn member(&self) -> &AggregateMemberPlan {
        &self.member
    }
}

/// One alternative in an enum or union, preserving declaration order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AggregateVariantPlan {
    name: String,
    fields: Vec<AggregateFieldPlan>,
}

impl AggregateVariantPlan {
    /// Returns the alternative name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns payload fields in declaration order.
    #[must_use]
    pub fn fields(&self) -> &[AggregateFieldPlan] {
        &self.fields
    }
}

/// One aggregate node in a flattened, target-independent plan.
///
/// Nested aggregate members refer to other nodes by plan-local index. This
/// avoids recursively nesting the plan's own Rust data structures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AggregateNodePlan {
    kind: AggregateKind,
    fields: Vec<AggregateFieldPlan>,
    variants: Vec<AggregateVariantPlan>,
}

impl AggregateNodePlan {
    /// Returns the language-level aggregate category.
    #[must_use]
    pub fn kind(&self) -> &AggregateKind {
        &self.kind
    }

    /// Returns aggregate-level fields in declaration order.
    #[must_use]
    pub fn fields(&self) -> &[AggregateFieldPlan] {
        &self.fields
    }

    /// Returns alternatives in declaration order.
    #[must_use]
    pub fn variants(&self) -> &[AggregateVariantPlan] {
        &self.variants
    }
}

/// A deterministic plan for adapting one aggregate descriptor.
///
/// The node table is flat. `root` identifies the root node; nested nodes are
/// referenced by [`AggregateMemberPlan::Nested`]. Node indices are valid only
/// for this plan instance and must not be persisted as canonical identities.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AggregateLoweringPlan {
    root: AggregateNodeIndex,
    nodes: Vec<AggregateNodePlan>,
    obligations: BTreeSet<AggregateLoweringObligation>,
    charged_work: u64,
}

impl AggregateLoweringPlan {
    /// Builds a target-independent plan from an aggregate descriptor.
    ///
    /// This function checks the descriptor invariants and scalar descriptor
    /// validity encountered during traversal. It does not perform semantic
    /// name resolution, generic substitution, recursive-type legality
    /// analysis, or canonical IR verification; those remain prerequisites
    /// owned by the appropriate compiler stages.
    pub fn build(
        descriptor: &AggregateTypeDescriptor,
        options: &AggregateLoweringOptions,
    ) -> AggregateLoweringResult<Self> {
        descriptor
            .validate()
            .map_err(|error| AggregateLoweringError::InvalidAggregateDescriptor {
                message: error.to_string(),
            })?;

        let mut plan = Self {
            root: 0,
            nodes: vec![AggregateNodePlan {
                kind: descriptor.kind().clone(),
                fields: Vec::new(),
                variants: Vec::new(),
            }],
            obligations: BTreeSet::new(),
            charged_work: 0,
        };

        let mut pending = Vec::new();
        pending
            .try_reserve(1)
            .map_err(|_| AggregateLoweringError::AllocationFailure)?;

        pending.push(PendingAggregate {
            descriptor,
            node: 0,
        });

        plan.obligations
            .insert(AggregateLoweringObligation::ResolveRepresentationLayout);

        while let Some(item) = pending.pop() {
            plan.charge_work(1, options.work_budget)?;

            item.descriptor.validate().map_err(|error| {
                AggregateLoweringError::InvalidAggregateDescriptor {
                    message: error.to_string(),
                }
            })?;

            let mut node = AggregateNodePlan {
                kind: item.descriptor.kind().clone(),
                fields: Vec::new(),
                variants: Vec::new(),
            };

            node.fields
                .try_reserve(item.descriptor.fields().len())
                .map_err(|_| AggregateLoweringError::AllocationFailure)?;

            for field in item.descriptor.fields() {
                plan.charge_work(1, options.work_budget)?;
                let lowered = plan.lower_field(field, options, &mut pending)?;
                node.fields.push(lowered);
            }

            node.variants
                .try_reserve(item.descriptor.variants().len())
                .map_err(|_| AggregateLoweringError::AllocationFailure)?;

            for variant in item.descriptor.variants() {
                let lowered = plan.lower_variant(variant, options, &mut pending)?;
                node.variants.push(lowered);
            }

            if matches!(node.kind, AggregateKind::UntaggedUnion) {
                plan.obligations
                    .insert(AggregateLoweringObligation::ResolveUntaggedUnionPolicy);
            }

            if let AggregateKind::Extension(name) = &node.kind {
                plan.obligations.insert(
                    AggregateLoweringObligation::ResolveExtensionSemantics {
                        name: name.clone(),
                    },
                );
            }

            let destination = plan
                .nodes
                .get_mut(item.node)
                .ok_or(AggregateLoweringError::InvalidPlanNodeIndex {
                    index: item.node,
                })?;

            *destination = node;
        }

        plan.validate()?;
        Ok(plan)
    }

    /// Returns the root node index.
    #[must_use]
    pub const fn root(&self) -> AggregateNodeIndex {
        self.root
    }

    /// Returns all aggregate nodes in the plan.
    ///
    /// The node table is flat; follow nested-member indices to inspect the
    /// aggregate tree. Node ordering is deterministic for a given descriptor.
    #[must_use]
    pub fn nodes(&self) -> &[AggregateNodePlan] {
        &self.nodes
    }

    /// Returns a node by plan-local index.
    #[must_use]
    pub fn node(&self, index: AggregateNodeIndex) -> Option<&AggregateNodePlan> {
        self.nodes.get(index)
    }

    /// Returns unresolved obligations in deterministic order.
    #[must_use]
    pub fn obligations(&self) -> &BTreeSet<AggregateLoweringObligation> {
        &self.obligations
    }

    /// Returns the number of charged work units used to build the plan.
    #[must_use]
    pub const fn charged_work(&self) -> u64 {
        self.charged_work
    }

    /// Returns whether any explicit follow-up obligation remains.
    #[must_use]
    pub fn has_obligations(&self) -> bool {
        !self.obligations.is_empty()
    }

    /// Revalidates plan-local structural invariants.
    ///
    /// This does not prove semantic validity, target feasibility, concrete
    /// layout, or canonical IR legality.
    pub fn validate(&self) -> AggregateLoweringResult<()> {
        let root = self
            .nodes
            .get(self.root)
            .ok_or(AggregateLoweringError::InvalidPlanNodeIndex {
                index: self.root,
            })?;

        if self.nodes.is_empty() {
            return Err(AggregateLoweringError::EmptyPlan);
        }

        // Every nested edge must point into this plan's node table.
        for node in &self.nodes {
            for field in &node.fields {
                self.validate_member(field.member())?;
            }

            for variant in &node.variants {
                for field in variant.fields() {
                    self.validate_member(field.member())?;
                }
            }
        }

        // The root must exist. Its kind is deliberately not restricted to
        // a subset of aggregate kinds; extension categories remain valid.
        let _ = root;
        Ok(())
    }

    fn validate_member(
        &self,
        member: &AggregateMemberPlan,
    ) -> AggregateLoweringResult<()> {
        match member {
            AggregateMemberPlan::Nested { node } => {
                if self.nodes.get(*node).is_none() {
                    return Err(AggregateLoweringError::InvalidPlanNodeIndex {
                        index: *node,
                    });
                }
            }
            AggregateMemberPlan::Scalar(scalar) => {
                scalar.validate().map_err(|error| {
                    AggregateLoweringError::InvalidScalarDescriptor {
                        message: error.to_string(),
                    }
                })?;
            }
            AggregateMemberPlan::Named { name } => {
                if name.trim().is_empty() {
                    return Err(AggregateLoweringError::InvalidNamedTypeReference);
                }
            }
        }

        Ok(())
    }

    fn lower_field(
        &mut self,
        field: &AggregateField,
        options: &AggregateLoweringOptions,
        pending: &mut Vec<PendingAggregate<'_>>,
    ) -> AggregateLoweringResult<AggregateFieldPlan> {
        let member = self.lower_member(field.ty(), options, pending)?;

        Ok(AggregateFieldPlan {
            name: field.name().map(str::to_owned),
            member,
        })
    }

    fn lower_variant(
        &mut self,
        variant: &AggregateVariant,
        options: &AggregateLoweringOptions,
        pending: &mut Vec<PendingAggregate<'_>>,
    ) -> AggregateLoweringResult<AggregateVariantPlan> {
        let mut fields = Vec::new();
        fields
            .try_reserve(variant.fields().len())
            .map_err(|_| AggregateLoweringError::AllocationFailure)?;

        for field in variant.fields() {
            self.charge_work(1, options.work_budget)?;
            fields.push(self.lower_field(field, options, pending)?);
        }

        Ok(AggregateVariantPlan {
            name: variant.name().to_owned(),
            fields,
        })
    }

    fn lower_member(
        &mut self,
        member: &AggregateMemberType,
        options: &AggregateLoweringOptions,
        pending: &mut Vec<PendingAggregate<'_>>,
    ) -> AggregateLoweringResult<AggregateMemberPlan> {
        match member {
            AggregateMemberType::Scalar(scalar) => {
                scalar.validate().map_err(|error| {
                    AggregateLoweringError::InvalidScalarDescriptor {
                        message: error.to_string(),
                    }
                })?;

                Ok(AggregateMemberPlan::Scalar(scalar.clone()))
            }

            AggregateMemberType::Named(name) => {
                if name.trim().is_empty() {
                    return Err(AggregateLoweringError::InvalidNamedTypeReference);
                }

                self.obligations.insert(
                    AggregateLoweringObligation::ResolveNamedType {
                        name: name.clone(),
                    },
                );

                Ok(AggregateMemberPlan::Named { name: name.clone() })
            }

            AggregateMemberType::Aggregate(nested) => {
                nested.validate().map_err(|error| {
                    AggregateLoweringError::InvalidAggregateDescriptor {
                        message: error.to_string(),
                    }
                })?;

                self.charge_work(1, options.work_budget)?;

                let node = self.nodes.len();

                self.nodes
                    .try_reserve(1)
                    .map_err(|_| AggregateLoweringError::AllocationFailure)?;

                pending
                    .try_reserve(1)
                    .map_err(|_| AggregateLoweringError::AllocationFailure)?;

                self.nodes.push(AggregateNodePlan {
                    kind: nested.kind().clone(),
                    fields: Vec::new(),
                    variants: Vec::new(),
                });

                pending.push(PendingAggregate {
                    descriptor: nested,
                    node,
                });

                Ok(AggregateMemberPlan::Nested { node })
            }
        }
    }

    fn charge_work(
        &mut self,
        requested: u64,
        budget: Option<u64>,
    ) -> AggregateLoweringResult<()> {
        let next = self
            .charged_work
            .checked_add(requested)
            .ok_or(AggregateLoweringError::WorkCounterOverflow)?;

        if let Some(limit) = budget {
            if next > limit {
                return Err(AggregateLoweringError::WorkBudgetExceeded {
                    consumed: self.charged_work,
                    requested,
                    budget: limit,
                });
            }
        }

        self.charged_work = next;
        Ok(())
    }
}

struct PendingAggregate<'a> {
    descriptor: &'a AggregateTypeDescriptor,
    node: AggregateNodeIndex,
}

/// Errors returned while building or validating an aggregate lowering plan.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AggregateLoweringError {
    /// The aggregate descriptor failed its local structural validation.
    InvalidAggregateDescriptor {
        /// Description of the descriptor validation failure.
        message: String,
    },

    /// A scalar descriptor failed its local validation.
    InvalidScalarDescriptor {
        /// Description of the scalar validation failure.
        message: String,
    },

    /// A named type reference was empty or whitespace-only.
    InvalidNamedTypeReference,

    /// A plan contains no root node.
    EmptyPlan,

    /// A nested-node index does not refer to a node in this plan.
    InvalidPlanNodeIndex {
        /// The invalid plan-local index.
        index: AggregateNodeIndex,
    },

    /// The configured operational work budget would be exceeded.
    WorkBudgetExceeded {
        /// Work already charged.
        consumed: u64,
        /// Additional work requested.
        requested: u64,
        /// Configured budget.
        budget: u64,
    },

    /// The work counter cannot represent the next value.
    WorkCounterOverflow,

    /// A required collection allocation could not be reserved.
    AllocationFailure,
}

impl fmt::Display for AggregateLoweringError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAggregateDescriptor { message } => {
                write!(formatter, "invalid aggregate descriptor: {message}")
            }
            Self::InvalidScalarDescriptor { message } => {
                write!(formatter, "invalid scalar descriptor: {message}")
            }
            Self::InvalidNamedTypeReference => {
                formatter.write_str("aggregate contains an empty named type reference")
            }
            Self::EmptyPlan => {
                formatter.write_str("aggregate lowering plan has no root node")
            }
            Self::InvalidPlanNodeIndex { index } => {
                write!(formatter, "aggregate plan node index {index} is invalid")
            }
            Self::WorkBudgetExceeded {
                consumed,
                requested,
                budget,
            } => write!(
                formatter,
                "aggregate lowering work budget exceeded: consumed {consumed}, \
                 requested {requested}, budget {budget}"
            ),
            Self::WorkCounterOverflow => {
                formatter.write_str("aggregate lowering work counter overflow")
            }
            Self::AllocationFailure => {
                formatter.write_str("could not reserve memory for aggregate lowering plan")
            }
        }
    }
}

impl Error for AggregateLoweringError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classical::types::aggregate::{
        AggregateField, AggregateMemberType, AggregateTypeDescriptor, AggregateVariant,
    };
    use crate::classical::types::scalar::{
        ScalarKind, ScalarTypeDescriptor, ScalarWidth,
    };

    fn scalar() -> AggregateMemberType {
        AggregateMemberType::Scalar(
            ScalarTypeDescriptor::new(ScalarKind::Boolean, ScalarWidth::Unspecified)
                .expect("valid Boolean descriptor"),
        )
    }

    fn named(name: &str) -> AggregateMemberType {
        AggregateMemberType::named(name).expect("valid named reference")
    }

    fn field(name: &str, member: AggregateMemberType) -> AggregateField {
        AggregateField::named(name, member).expect("valid field")
    }

    #[test]
    fn empty_tuple_is_preserved() {
        let descriptor =
            AggregateTypeDescriptor::tuple(Vec::new()).expect("empty tuple is valid");

        let plan = AggregateLoweringPlan::build(
            &descriptor,
            &AggregateLoweringOptions::default(),
        )
        .expect("plan construction");

        assert_eq!(plan.root(), 0);
        assert_eq!(plan.nodes().len(), 1);
        assert_eq!(plan.node(0).expect("root").kind(), descriptor.kind());
    }

    #[test]
    fn field_order_and_names_are_preserved() {
        let descriptor = AggregateTypeDescriptor::structure(vec![
            field("first", scalar()),
            field("second", named("module::Second")),
        ])
        .expect("valid structure");

        let plan = AggregateLoweringPlan::build(
            &descriptor,
            &AggregateLoweringOptions::default(),
        )
        .expect("plan construction");

        let root = plan.node(plan.root()).expect("root");
        assert_eq!(root.fields().len(), 2);
        assert_eq!(root.fields()[0].name(), Some("first"));
        assert_eq!(root.fields()[1].name(), Some("second"));
    }

    #[test]
    fn named_types_are_retained_as_obligations() {
        let descriptor = AggregateTypeDescriptor::tuple(vec![
            AggregateField::positional(named("package::Record")),
        ])
        .expect("valid tuple");

        let plan = AggregateLoweringPlan::build(
            &descriptor,
            &AggregateLoweringOptions::default(),
        )
        .expect("plan construction");

        assert!(plan.obligations().contains(
            &AggregateLoweringObligation::ResolveNamedType {
                name: "package::Record".to_owned(),
            }
        ));
    }

    #[test]
    fn nested_aggregate_is_flattened_without_losing_its_kind() {
        let inner = AggregateTypeDescriptor::tuple(vec![
            AggregateField::positional(scalar()),
        ])
        .expect("inner tuple");

        let outer = AggregateTypeDescriptor::structure(vec![
            field("payload", AggregateMemberType::aggregate(inner.clone())),
        ])
        .expect("outer structure");

        let plan = AggregateLoweringPlan::build(
            &outer,
            &AggregateLoweringOptions::default(),
        )
        .expect("plan construction");

        assert_eq!(plan.nodes().len(), 2);

        let root = plan.node(plan.root()).expect("root");
        let AggregateMemberPlan::Nested { node } = root.fields()[0].member() else {
            panic!("nested member should point to a plan node");
        };

        assert_eq!(plan.node(*node).expect("nested node").kind(), inner.kind());
    }

    #[test]
    fn variants_and_payload_order_are_preserved() {
        let variant = AggregateVariant::new(
            "Present",
            vec![
                field("left", scalar()),
                field("right", named("RightType")),
            ],
        )
        .expect("valid variant");

        let descriptor =
            AggregateTypeDescriptor::tagged_union(vec![variant]).expect("tagged union");

        let plan = AggregateLoweringPlan::build(
            &descriptor,
            &AggregateLoweringOptions::default(),
        )
        .expect("plan construction");

        let root = plan.node(plan.root()).expect("root");
        assert_eq!(root.variants()[0].name(), "Present");
        assert_eq!(root.variants()[0].fields()[0].name(), Some("left"));
        assert_eq!(root.variants()[0].fields()[1].name(), Some("right"));
    }

    #[test]
    fn untagged_union_requires_an_explicit_policy() {
        let descriptor = AggregateTypeDescriptor::untagged_union(vec![
            AggregateVariant::unit("A").expect("variant"),
            AggregateVariant::unit("B").expect("variant"),
        ])
        .expect("valid union");

        let plan = AggregateLoweringPlan::build(
            &descriptor,
            &AggregateLoweringOptions::default(),
        )
        .expect("plan construction");

        assert!(plan
            .obligations()
            .contains(&AggregateLoweringObligation::ResolveUntaggedUnionPolicy));
    }

    #[test]
    fn extension_kinds_are_not_rejected_by_a_closed_enum() {
        let descriptor = AggregateTypeDescriptor::extension(
            "vendor.vector_record",
            vec![field("value", scalar())],
            Vec::new(),
        )
        .expect("extension descriptor");

        let plan = AggregateLoweringPlan::build(
            &descriptor,
            &AggregateLoweringOptions::default(),
        )
        .expect("plan construction");

        assert!(plan.obligations().contains(
            &AggregateLoweringObligation::ResolveExtensionSemantics {
                name: "vendor.vector_record".to_owned(),
            }
        ));
    }

    #[test]
    fn explicit_work_budget_is_enforced() {
        let descriptor = AggregateTypeDescriptor::structure(vec![
            field("first", scalar()),
            field("second", scalar()),
        ])
        .expect("valid structure");

        let options = AggregateLoweringOptions {
            work_budget: Some(1),
        };

        assert!(matches!(
            AggregateLoweringPlan::build(&descriptor, &options),
            Err(AggregateLoweringError::WorkBudgetExceeded { .. })
        ));
    }

    #[test]
    fn work_budget_failure_does_not_produce_a_partial_plan() {
        let descriptor = AggregateTypeDescriptor::tuple(vec![
            AggregateField::positional(scalar()),
            AggregateField::positional(scalar()),
        ])
        .expect("valid tuple");

        let result = AggregateLoweringPlan::build(
            &descriptor,
            &AggregateLoweringOptions {
                work_budget: Some(0),
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn plan_validation_checks_nested_indices() {
        let descriptor =
            AggregateTypeDescriptor::tuple(vec![AggregateField::positional(scalar())])
                .expect("valid tuple");

        let mut plan = AggregateLoweringPlan::build(
            &descriptor,
            &AggregateLoweringOptions::default(),
        )
        .expect("plan construction");

        plan.nodes[0].fields[0].member =
            AggregateMemberPlan::Nested { node: usize::MAX };

        assert!(matches!(
            plan.validate(),
            Err(AggregateLoweringError::InvalidPlanNodeIndex { .. })
        ));
    }

    #[test]
    fn layout_remains_an_explicit_follow_up() {
        let descriptor =
            AggregateTypeDescriptor::tuple(vec![AggregateField::positional(scalar())])
                .expect("valid tuple");

        let plan = AggregateLoweringPlan::build(
            &descriptor,
            &AggregateLoweringOptions::default(),
        )
        .expect("plan construction");

        assert!(plan
            .obligations()
            .contains(&AggregateLoweringObligation::ResolveRepresentationLayout));
    }
}
