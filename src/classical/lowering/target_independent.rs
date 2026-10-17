
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! Target-independent coordination for Zamani classical lowering.
//!
//! Path: `src/classical/lowering/target_independent.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Coordinates existing scalar and aggregate lowering plans while preserving
//! their semantic information and recording lowering progress in the shared
//! classical lowering context.
//!
//! # Authority and ownership
//!
//! Source syntax is governed by the Zamani grammar, whose composition root is
//! `grammar/Zamani.g4` and whose public ANTLR boundaries are
//! `grammar/antlr/ZamaniLexer.g4` and `grammar/antlr/ZamaniParser.g4`.
//!
//! Source meaning and compilation boundaries are governed by:
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! This file owns only coordination of target-independent scalar and
//! aggregate plans. It does not own source syntax, semantic types, canonical
//! IDs, canonical IR, target capabilities, physical layouts, execution, or
//! backend-specific representations.
//!
//! # POCO-REAF and scalability
//!
//! No universal hardware or collection-size ceilings are introduced.
//! Explicit work budgets are inherited from `LoweringContext` and remain
//! operational policies rather than language-semantic restrictions.
//! Practical allocation failures, finite address spaces, and target
//! capabilities remain real constraints.
//!
//! # Integration
//!
//! - `lowering/context.rs` owns the shared context, identity mappings,
//!   symbolic requirements, provenance, dispositions, and work accounting.
//! - `lowering/scalar.rs` owns scalar lowering plans.
//! - `lowering/aggregate.rs` owns aggregate lowering plans.
//! - `types/scalar.rs` and `types/aggregate.rs` remain the canonical owners
//!   of the descriptors consumed by those planners.
//! - A future canonical-IR adapter must consume these plans, preserve
//!   semantics and identities, and invoke the canonical verifier.
//! - `src/quantum/ir/classical/` and `src/zuir/` remain canonical IR
//!   boundaries; this module creates no IR nodes and no canonical IDs.
//! - Target realization, layout, capability checks, scheduling, allocation,
//!   and runtime execution remain downstream responsibilities.
//!
//! # Important guarantee
//!
//! Successfully creating a plan means only that the local descriptor and
//! planning contracts passed. It does not prove source-program validity,
//! target feasibility, concrete layout, semantic preservation by a backend,
//! or canonical IR verification.

use core::fmt;
use std::error::Error;

use crate::classical::lowering::aggregate::{
    AggregateLoweringError, AggregateLoweringOptions, AggregateLoweringPlan,
};
use crate::classical::lowering::context::{
    EntityKey, EntityKind, LoweringContext, LoweringContextError,
    LoweringDisposition,
};
use crate::classical::lowering::scalar::{
    ScalarLoweringError, ScalarLoweringPlan,
};
use crate::classical::types::aggregate::AggregateTypeDescriptor;
use crate::classical::types::scalar::ScalarTypeDescriptor;

/// Result type for target-independent planning operations.
pub type TargetIndependentResult<T> =
    Result<T, TargetIndependentLoweringError>;

/// A target-independent plan produced for one semantic type entity.
///
/// The contained plans preserve source-level type meaning. They are not
/// target-specific layouts, executable instructions, or canonical IR nodes.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TargetIndependentPlanKind {
    /// A scalar plan preserving the canonical scalar descriptor.
    Scalar(ScalarLoweringPlan),

    /// An aggregate plan preserving aggregate structure and member order.
    Aggregate(AggregateLoweringPlan),
}

/// One planned semantic type and its source identity.
///
/// The identity is supplied by the upstream semantic or compiler adapter.
/// This module does not manufacture a replacement canonical identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetIndependentPlan {
    source: EntityKey,
    kind: TargetIndependentPlanKind,
}

impl TargetIndependentPlan {
    /// Returns the upstream semantic entity associated with this plan.
    #[must_use]
    pub fn source(&self) -> &EntityKey {
        &self.source
    }

    /// Returns the plan without discarding its semantic details.
    #[must_use]
    pub fn kind(&self) -> &TargetIndependentPlanKind {
        &self.kind
    }

    /// Returns the scalar plan, or `None` for an aggregate plan.
    #[must_use]
    pub fn as_scalar(&self) -> Option<&ScalarLoweringPlan> {
        match &self.kind {
            TargetIndependentPlanKind::Scalar(plan) => Some(plan),
            TargetIndependentPlanKind::Aggregate(_) => None,
        }
    }

    /// Returns the aggregate plan, or `None` for a scalar plan.
    #[must_use]
    pub fn as_aggregate(&self) -> Option<&AggregateLoweringPlan> {
        match &self.kind {
            TargetIndependentPlanKind::Aggregate(plan) => Some(plan),
            TargetIndependentPlanKind::Scalar(_) => None,
        }
    }
}

/// Errors returned by target-independent planning coordination.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TargetIndependentLoweringError {
    /// The supplied source entity is not a semantic type entity.
    InvalidSourceEntityKind {
        /// The unexpected entity kind.
        actual: EntityKind,
    },

    /// The shared context rejected a transaction or work charge.
    Context(LoweringContextError),

    /// Scalar plan creation or validation failed.
    Scalar(ScalarLoweringError),

    /// Aggregate plan creation or validation failed.
    Aggregate(AggregateLoweringError),
}

impl fmt::Display for TargetIndependentLoweringError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSourceEntityKind { actual } => {
                write!(
                    formatter,
                    "target-independent type lowering requires a Type entity, got {actual:?}"
                )
            }
            Self::Context(error) => {
                write!(formatter, "lowering context rejected the operation: {error}")
            }
            Self::Scalar(error) => {
                write!(formatter, "scalar lowering failed: {error}")
            }
            Self::Aggregate(error) => {
                write!(formatter, "aggregate lowering failed: {error}")
            }
        }
    }
}

impl Error for TargetIndependentLoweringError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidSourceEntityKind { .. } => None,
            Self::Context(error) => Some(error),
            Self::Scalar(error) => Some(error),
            Self::Aggregate(error) => Some(error),
        }
    }
}

impl From<LoweringContextError> for TargetIndependentLoweringError {
    fn from(error: LoweringContextError) -> Self {
        Self::Context(error)
    }
}

impl From<ScalarLoweringError> for TargetIndependentLoweringError {
    fn from(error: ScalarLoweringError) -> Self {
        Self::Scalar(error)
    }
}

impl From<AggregateLoweringError> for TargetIndependentLoweringError {
    fn from(error: AggregateLoweringError) -> Self {
        Self::Aggregate(error)
    }
}

/// Coordinates target-independent classical type planning.
///
/// A coordinator is intended to be owned by one lowering operation. It
/// contains no process-global state. Plans are retained in input order,
/// making output deterministic for the same ordered inputs and configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetIndependentLowerer {
    context: LoweringContext,
    plans: Vec<TargetIndependentPlan>,
}

impl TargetIndependentLowerer {
    /// Creates a coordinator using an existing lowering context.
    ///
    /// Passing the context preserves the caller's configured work budget,
    /// requirements, provenance, mappings, and previously recorded notes.
    #[must_use]
    pub fn new(context: LoweringContext) -> Self {
        Self {
            context,
            plans: Vec::new(),
        }
    }

    /// Creates a coordinator with the default lowering policy.
    #[must_use]
    pub fn with_defaults() -> Self {
        Self::new(LoweringContext::with_defaults())
    }

    /// Returns the shared lowering context.
    #[must_use]
    pub fn context(&self) -> &LoweringContext {
        &self.context
    }

    /// Returns the plans accumulated so far in deterministic input order.
    #[must_use]
    pub fn plans(&self) -> &[TargetIndependentPlan] {
        &self.plans
    }

    /// Consumes the coordinator and returns its context and plans.
    #[must_use]
    pub fn into_parts(self) -> (LoweringContext, Vec<TargetIndependentPlan>) {
        (self.context, self.plans)
    }

    /// Plans a scalar semantic type.
    ///
    /// The descriptor must already have passed the applicable upstream
    /// semantic checks. This method validates the scalar descriptor, creates
    /// its target-independent plan, charges one coordinator work unit, and
    /// records whether further lowering obligations remain.
    ///
    /// It does not choose a machine width or silently resolve symbolic,
    /// arbitrary, or unspecified widths.
    pub fn plan_scalar(
        &mut self,
        source: EntityKey,
        descriptor: &ScalarTypeDescriptor,
    ) -> TargetIndependentResult<&TargetIndependentPlan> {
        self.validate_source(&source)?;

        let plan = ScalarLoweringPlan::from_descriptor(descriptor)?;
        plan.validate()?;

        // Reserve before mutating the shared context. Once the context
        // transaction commits, appending the plan will not need to grow Vec.
        self.plans
            .try_reserve(1)
            .map_err(|_| {
                TargetIndependentLoweringError::Context(
                    LoweringContextError::WorkCounterOverflow,
                )
            })?;

        let disposition = if plan.has_obligations() {
            LoweringDisposition::Conditional {
                conditions: vec![
                    "Discharge scalar lowering obligations before target realization"
                        .to_owned(),
                ],
            }
        } else {
            LoweringDisposition::Legal
        };

        self.context
            .transaction(|context| -> TargetIndependentResult<()> {
                context.charge_work(1)?;
                context.set_disposition(source.clone(), disposition)?;
                Ok(())
            })?;

        self.plans.push(TargetIndependentPlan {
            source,
            kind: TargetIndependentPlanKind::Scalar(plan),
        });

        // The preceding push guarantees at least one element.
        Ok(self
            .plans
            .last()
            .expect("a plan was appended immediately before this lookup"))
    }

    /// Plans an aggregate semantic type.
    ///
    /// The aggregate planner receives the remaining shared work budget, if
    /// one is configured. Its measured work is then charged to the shared
    /// context in a transaction. A failure leaves the context unchanged and
    /// does not append a partial plan.
    ///
    /// This method preserves symbolic named-type references and layout
    /// obligations; it does not resolve names or calculate physical layout.
    pub fn plan_aggregate(
        &mut self,
        source: EntityKey,
        descriptor: &AggregateTypeDescriptor,
    ) -> TargetIndependentResult<&TargetIndependentPlan> {
        self.validate_source(&source)?;

        let remaining_budget = self
            .context
            .work_budget()
            .map(|budget| budget.saturating_sub(self.context.consumed_work()));

        let options = AggregateLoweringOptions {
            work_budget: remaining_budget,
        };
        let plan = AggregateLoweringPlan::build(descriptor, &options)?;
        plan.validate()?;

        self.plans
            .try_reserve(1)
            .map_err(|_| {
                TargetIndependentLoweringError::Context(
                    LoweringContextError::WorkCounterOverflow,
                )
            })?;

        let work = plan.charged_work();
        let disposition = if plan.has_obligations() {
            LoweringDisposition::Conditional {
                conditions: vec![
                    "Discharge aggregate lowering obligations before target realization"
                        .to_owned(),
                ],
            }
        } else {
            LoweringDisposition::Legal
        };

        self.context
            .transaction(|context| -> TargetIndependentResult<()> {
                context.charge_work(work)?;
                context.set_disposition(source.clone(), disposition)?;
                Ok(())
            })?;

        self.plans.push(TargetIndependentPlan {
            source,
            kind: TargetIndependentPlanKind::Aggregate(plan),
        });

        Ok(self
            .plans
            .last()
            .expect("a plan was appended immediately before this lookup"))
    }

    fn validate_source(
        &self,
        source: &EntityKey,
    ) -> TargetIndependentResult<()> {
        if source.kind() != &EntityKind::Type {
            return Err(
                TargetIndependentLoweringError::InvalidSourceEntityKind {
                    actual: source.kind().clone(),
                },
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::classical::lowering::context::{
        EntityKey, EntityKind, LoweringPolicy,
    };
    use crate::classical::types::aggregate::AggregateTypeDescriptor;
    use crate::classical::types::scalar::{
        ScalarKind, ScalarTypeDescriptor,
    };

    fn type_key(name: &str) -> EntityKey {
        EntityKey::new(EntityKind::Type, "semantic", name)
            .expect("test type identity should be valid")
    }

    fn scalar_descriptor() -> ScalarTypeDescriptor {
        ScalarTypeDescriptor::unspecified(ScalarKind::SignedInteger)
            .expect("the scalar descriptor should be valid")
    }

    #[test]
    fn scalar_plan_preserves_unresolved_width_and_records_follow_up() {
        let mut lowerer = TargetIndependentLowerer::with_defaults();

        let result = lowerer
            .plan_scalar(type_key("Integer"), &scalar_descriptor())
            .expect("valid scalar should be planned");

        assert!(result.as_scalar().is_some());
        assert!(result.as_aggregate().is_none());
        assert_eq!(lowerer.context().consumed_work(), 1);
        assert_eq!(
            lowerer.context().disposition(&type_key("Integer")),
            Some(&LoweringDisposition::Conditional {
                conditions: vec![
                    "Discharge scalar lowering obligations before target realization"
                        .to_owned(),
                ],
            })
        );
    }

    #[test]
    fn aggregate_plan_charges_the_shared_budget() {
        let context = LoweringContext::new(LoweringPolicy {
            work_budget: Some(10),
            ..LoweringPolicy::default()
        });
        let mut lowerer = TargetIndependentLowerer::new(context);
        let descriptor =
            AggregateTypeDescriptor::tuple(Vec::new())
                .expect("an empty tuple is a valid aggregate descriptor");

        let result = lowerer
            .plan_aggregate(type_key("EmptyTuple"), &descriptor)
            .expect("valid aggregate should be planned");

        assert!(result.as_aggregate().is_some());
        assert_eq!(
            lowerer.context().consumed_work(),
            result
                .as_aggregate()
                .expect("aggregate plan was just created")
                .charged_work()
        );
    }

    #[test]
    fn wrong_entity_kind_is_rejected_before_context_mutation() {
        let mut lowerer = TargetIndependentLowerer::with_defaults();
        let source = EntityKey::new(EntityKind::Value, "semantic", "v")
            .expect("test value identity should be valid");

        let result = lowerer.plan_scalar(source, &scalar_descriptor());

        assert!(matches!(
            result,
            Err(TargetIndependentLoweringError::InvalidSourceEntityKind {
                actual: EntityKind::Value,
            })
        ));
        assert_eq!(lowerer.context().consumed_work(), 0);
        assert!(lowerer.plans().is_empty());
    }

    #[test]
    fn exhausted_shared_budget_does_not_append_a_scalar_plan() {
        let context = LoweringContext::new(LoweringPolicy {
            work_budget: Some(0),
            ..LoweringPolicy::default()
        });
        let mut lowerer = TargetIndependentLowerer::new(context);

        let result =
            lowerer.plan_scalar(type_key("Integer"), &scalar_descriptor());

        assert!(matches!(
            result,
            Err(TargetIndependentLoweringError::Context(
                LoweringContextError::WorkBudgetExceeded { .. }
            ))
        ));
        assert_eq!(lowerer.context().consumed_work(), 0);
        assert!(lowerer.plans().is_empty());
        assert!(lowerer
            .context()
            .disposition(&type_key("Integer"))
            .is_none());
    }

    #[test]
    fn many_scalar_plans_have_no_artificial_count_ceiling() {
        let mut lowerer = TargetIndependentLowerer::with_defaults();
        let descriptor = scalar_descriptor();

        for index in 0..512_u64 {
            let source = type_key(&format!("Integer{index}"));
            lowerer
                .plan_scalar(source, &descriptor)
                .expect("planning should continue while resources permit");
        }

        assert_eq!(lowerer.plans().len(), 512);
        assert_eq!(lowerer.context().consumed_work(), 512);
    }
}
