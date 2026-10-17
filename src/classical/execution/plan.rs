
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Target-Independent Execution Plans.
//!
//! Path: `src/classical/execution/plan.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Represent and validate a target-independent execution plan for classical
//! computation. A plan describes work, dependencies, capability requirements,
//! and resource demands without deciding where or how the work executes.
//!
//! # Authority and ownership
//!
//! - `grammar/specification/semantic-model.md` defines the semantic boundary.
//! - `grammar/spec/resources.md` defines the resource-intent boundary.
//! - `src/frontend/semantic/capabilities/model.rs` owns semantic capability
//!   identities and requirements.
//! - `src/zuir/` owns canonical computation IR and its operation identities.
//! - `src/classical/config.rs` owns classical execution policies and budgets.
//! - Target-specific infrastructure owns feasibility, placement, scheduling,
//!   allocation, lowering, and execution.
//!
//! This module owns only its target-independent plan representation,
//! structural validation, and deterministic dependency ordering.
//!
//! # Important integration rule
//!
//! Capability names in this plan are opaque references supplied by an adapter.
//! They are not replacements for the canonical semantic capability model.
//! Likewise, `StepId` identifies a step within a plan; it is not a ZUIR
//! `OperationId`, an AST `NodeId`, or a globally unique identity.
//!
//! An integration adapter must retain the original canonical identities and
//! provenance in its surrounding compilation context.
//!
//! # Scalability
//!
//! No fixed semantic limit is imposed on steps, dependencies, capabilities,
//! resource classes, or resource quantities. Storage and processing are
//! necessarily bounded by available memory, representable values, and any
//! explicit operational budgets enforced by the caller.
//!
//! This module does not claim literal infinite capacity.
//!
//! # Non-responsibilities
//!
//! This module does not:
//!
//! - parse Zamani source or construct AST nodes;
//! - infer types, effects, capabilities, or resource requirements;
//! - define a second classical or universal IR;
//! - discover hardware or authenticate capability evidence;
//! - select a backend, device, worker, or physical location;
//! - allocate memory, create threads, or execute operations;
//! - silently relax a requirement when a target cannot satisfy it.
//!
//! # Determinism
//!
//! For the same valid plan, topological ordering is deterministic. Among
//! currently ready steps, ordering is by `StepId`.
//!
//! Deterministic ordering does not imply that every possible runtime
//! execution is deterministic.
//!
//! # Completion status
//!
//! This file is designed as an independent foundation. It becomes integrated
//! only after the execution module, semantic adapters, and repository tests
//! compile against its contract.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

/// Current version of the in-memory execution-plan contract.
///
/// This is not the Zamani language version, ZUIR version, or target ABI
/// version. Increment it only when the plan contract itself changes.
pub const EXECUTION_PLAN_SCHEMA_VERSION: u32 = 1;

/// Result of validating or operating on an execution plan.
pub type ExecutionPlanResult<T> = Result<T, ExecutionPlanError>;

/// An identifier unique within one execution plan.
///
/// This identifier is intentionally separate from canonical IR identities.
/// The adapter constructing the plan is responsible for stable mapping
/// between plan steps and their originating IR operations.
///
/// Identifiers must contain at least one ASCII letter or digit. ASCII
/// letters, digits, `.`, `_`, `-`, `:`, and `/` are permitted.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StepId(String);

impl StepId {
    /// Construct a validated plan-local step identifier.
    pub fn new(value: impl Into<String>) -> ExecutionPlanResult<Self> {
        let value = value.into();

        if !is_valid_identifier(&value) {
            return Err(ExecutionPlanError::InvalidIdentifier {
                kind: IdentifierKind::Step,
                value,
            });
        }

        Ok(Self(value))
    }

    /// Return the identifier's stable textual representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StepId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// The kind of identifier rejected by plan validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentifierKind {
    /// A plan identifier.
    Plan,
    /// A plan-local step identifier.
    Step,
    /// A capability reference.
    Capability,
    /// A resource class identifier.
    ResourceClass,
    /// A unit identifier.
    Unit,
    /// An operation-kind identifier.
    OperationKind,
}

impl fmt::Display for IdentifierKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Plan => "plan",
            Self::Step => "step",
            Self::Capability => "capability",
            Self::ResourceClass => "resource class",
            Self::Unit => "unit",
            Self::OperationKind => "operation kind",
        })
    }
}

/// Structured execution-plan validation failures.
///
/// Variants are machine-readable. Consumers must not parse display messages.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExecutionPlanError {
    /// A supplied identifier does not follow the identifier contract.
    InvalidIdentifier {
        /// Identifier category.
        kind: IdentifierKind,
        /// Rejected value.
        value: String,
    },

    /// The plan schema is not supported.
    UnsupportedSchemaVersion {
        /// Supplied schema version.
        supplied: u32,
        /// Supported schema version.
        supported: u32,
    },

    /// A plan contains the same step identifier more than once.
    DuplicateStep {
        /// Duplicated identifier.
        step: StepId,
    },

    /// A step depends on an identifier that does not occur in the plan.
    UnknownDependency {
        /// Step declaring the dependency.
        step: StepId,
        /// Missing dependency.
        dependency: StepId,
    },

    /// A step depends on itself.
    SelfDependency {
        /// Invalid step.
        step: StepId,
    },

    /// The dependency graph contains a cycle.
    DependencyCycle {
        /// Number of steps that could not be ordered.
        unresolved_steps: usize,
    },

    /// The same resource class and unit were specified more than once.
    DuplicateResourceDemand {
        /// Resource class.
        resource_class: String,
        /// Unit associated with the resource quantity.
        unit: String,
    },

    /// The plan has no valid identifier.
    InvalidPlanIdentifier {
        /// Rejected identifier.
        value: String,
    },
}

impl fmt::Display for ExecutionPlanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { kind, value } => {
                write!(formatter, "invalid {kind} identifier `{value}`")
            }
            Self::UnsupportedSchemaVersion {
                supplied,
                supported,
            } => write!(
                formatter,
                "unsupported execution-plan schema version {supplied}; \
                 supported version is {supported}"
            ),
            Self::DuplicateStep { step } => {
                write!(formatter, "duplicate execution-plan step `{step}`")
            }
            Self::UnknownDependency { step, dependency } => write!(
                formatter,
                "step `{step}` depends on unknown step `{dependency}`"
            ),
            Self::SelfDependency { step } => {
                write!(formatter, "step `{step}` depends on itself")
            }
            Self::DependencyCycle { unresolved_steps } => write!(
                formatter,
                "execution-plan dependency graph contains a cycle; \
                 {unresolved_steps} step(s) could not be ordered"
            ),
            Self::DuplicateResourceDemand {
                resource_class,
                unit,
            } => write!(
                formatter,
                "duplicate demand for resource class `{resource_class}` \
                 and unit `{unit}`"
            ),
            Self::InvalidPlanIdentifier { value } => {
                write!(formatter, "invalid execution-plan identifier `{value}`")
            }
        }
    }
}

impl Error for ExecutionPlanError {}

/// A target-independent, non-executable description of one unit of work.
///
/// `operation_kind` is an open identifier supplied by the producer. This
/// structure does not prescribe a closed catalogue of classical operations.
///
/// Dependencies reference other steps in the same plan. They represent
/// ordering constraints, not scheduling decisions or synchronization
/// guarantees for concurrent runtime execution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlanStep {
    id: StepId,
    operation_kind: String,
    dependencies: BTreeSet<StepId>,
}

impl PlanStep {
    /// Construct a plan step with no dependencies.
    pub fn new(
        id: StepId,
        operation_kind: impl Into<String>,
    ) -> ExecutionPlanResult<Self> {
        let operation_kind = operation_kind.into();

        if !is_valid_identifier(&operation_kind) {
            return Err(ExecutionPlanError::InvalidIdentifier {
                kind: IdentifierKind::OperationKind,
                value: operation_kind,
            });
        }

        Ok(Self {
            id,
            operation_kind,
            dependencies: BTreeSet::new(),
        })
    }

    /// Add a dependency that must complete before this step is eligible.
    ///
    /// The dependency's existence is checked when the plan is validated.
    pub fn depends_on(&mut self, dependency: StepId) {
        self.dependencies.insert(dependency);
    }

    /// Return the plan-local step identifier.
    #[must_use]
    pub fn id(&self) -> &StepId {
        &self.id
    }

    /// Return the extensible operation-kind identifier.
    #[must_use]
    pub fn operation_kind(&self) -> &str {
        &self.operation_kind
    }

    /// Return this step's dependencies in deterministic order.
    #[must_use]
    pub fn dependencies(&self) -> &BTreeSet<StepId> {
        &self.dependencies
    }
}

/// An opaque capability reference required by the proposed plan.
///
/// This is deliberately not a full semantic capability definition. The
/// frontend adapter must preserve and retain the canonical requirement and
/// its source provenance separately.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CapabilityReference(String);

impl CapabilityReference {
    /// Construct a validated capability reference.
    pub fn new(value: impl Into<String>) -> ExecutionPlanResult<Self> {
        let value = value.into();

        if !is_valid_identifier(&value) {
            return Err(ExecutionPlanError::InvalidIdentifier {
                kind: IdentifierKind::Capability,
                value,
            });
        }

        Ok(Self(value))
    }

    /// Return the opaque capability name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A symbolic resource demand expressed in a named unit.
///
/// `minimum` is an application-defined quantity. Its meaning must be
/// documented by the adapter and the authoritative resource model. This
/// structure does not assume that every resource is additive or fungible.
///
/// A demand is an intent, not a reservation or allocation. A planner must
/// check feasibility against actual target evidence and resource descriptions.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ResourceDemand {
    resource_class: String,
    unit: String,
    minimum: u128,
}

impl ResourceDemand {
    /// Construct a resource demand.
    ///
    /// Zero is accepted because zero can be meaningful for some resource
    /// models. The authoritative resource adapter decides whether a given
    /// quantity is meaningful for its resource class.
    pub fn new(
        resource_class: impl Into<String>,
        unit: impl Into<String>,
        minimum: u128,
    ) -> ExecutionPlanResult<Self> {
        let resource_class = resource_class.into();
        let unit = unit.into();

        if !is_valid_identifier(&resource_class) {
            return Err(ExecutionPlanError::InvalidIdentifier {
                kind: IdentifierKind::ResourceClass,
                value: resource_class,
            });
        }

        if !is_valid_identifier(&unit) {
            return Err(ExecutionPlanError::InvalidIdentifier {
                kind: IdentifierKind::Unit,
                value: unit,
            });
        }

        Ok(Self {
            resource_class,
            unit,
            minimum,
        })
    }

    /// Return the resource class identifier.
    #[must_use]
    pub fn resource_class(&self) -> &str {
        &self.resource_class
    }

    /// Return the quantity unit.
    #[must_use]
    pub fn unit(&self) -> &str {
        &self.unit
    }

    /// Return the declared minimum quantity.
    #[must_use]
    pub const fn minimum(&self) -> u128 {
        self.minimum
    }
}

/// A validated or not-yet-validated target-independent execution plan.
///
/// This structure records requirements and a dependency graph. It does not
/// certify that any target can execute the plan.
///
/// Fields remain private so mutations pass through the provided constructors
/// and insertion methods. Validation must still be performed at subsystem
/// boundaries, especially when data originates outside the compiler.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionPlan {
    schema_version: u32,
    id: String,
    steps: Vec<PlanStep>,
    required_capabilities: BTreeSet<CapabilityReference>,
    resource_demands: Vec<ResourceDemand>,
}

impl ExecutionPlan {
    /// Create an empty plan using the current plan-schema version.
    pub fn new(id: impl Into<String>) -> ExecutionPlanResult<Self> {
        let id = id.into();

        if !is_valid_identifier(&id) {
            return Err(ExecutionPlanError::InvalidPlanIdentifier { value: id });
        }

        Ok(Self {
            schema_version: EXECUTION_PLAN_SCHEMA_VERSION,
            id,
            steps: Vec::new(),
            required_capabilities: BTreeSet::new(),
            resource_demands: Vec::new(),
        })
    }

    /// Return the plan identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the plan-schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Return the plan steps in insertion order.
    ///
    /// Insertion order is not an execution order. Use
    /// [`ExecutionPlan::topological_order`] to obtain a valid deterministic
    /// dependency order.
    #[must_use]
    pub fn steps(&self) -> &[PlanStep] {
        &self.steps
    }

    /// Return the capability references required by this plan.
    #[must_use]
    pub fn required_capabilities(&self) -> &BTreeSet<CapabilityReference> {
        &self.required_capabilities
    }

    /// Return the declared resource demands.
    #[must_use]
    pub fn resource_demands(&self) -> &[ResourceDemand] {
        &self.resource_demands
    }

    /// Append a step.
    ///
    /// Duplicate identifiers are detected by [`ExecutionPlan::validate`].
    /// This allows callers to assemble a plan incrementally and then validate
    /// it as a whole.
    pub fn add_step(&mut self, step: PlanStep) {
        self.steps.push(step);
    }

    /// Record a required capability reference.
    ///
    /// Repeated insertion of the same reference is idempotent. This operation
    /// does not establish that the capability exists or is available.
    pub fn require_capability(&mut self, capability: CapabilityReference) {
        self.required_capabilities.insert(capability);
    }

    /// Record a resource demand.
    ///
    /// Conflicting or repeated demands for the same class and unit are
    /// rejected during validation instead of being silently added together.
    /// Aggregation semantics belong to the authoritative resource model.
    pub fn require_resource(&mut self, demand: ResourceDemand) {
        self.resource_demands.push(demand);
    }

    /// Validate schema, identifiers, dependency references, and resource
    /// demand uniqueness.
    ///
    /// This checks structural consistency only. It does not perform semantic
    /// validation, capability evidence evaluation, or target feasibility.
    pub fn validate(&self) -> ExecutionPlanResult<()> {
        if self.schema_version != EXECUTION_PLAN_SCHEMA_VERSION {
            return Err(ExecutionPlanError::UnsupportedSchemaVersion {
                supplied: self.schema_version,
                supported: EXECUTION_PLAN_SCHEMA_VERSION,
            });
        }

        if !is_valid_identifier(&self.id) {
            return Err(ExecutionPlanError::InvalidPlanIdentifier {
                value: self.id.clone(),
            });
        }

        let mut index_by_id = BTreeMap::new();

        for (index, step) in self.steps.iter().enumerate() {
            if index_by_id.insert(step.id.clone(), index).is_some() {
                return Err(ExecutionPlanError::DuplicateStep {
                    step: step.id.clone(),
                });
            }

            if !is_valid_identifier(&step.operation_kind) {
                return Err(ExecutionPlanError::InvalidIdentifier {
                    kind: IdentifierKind::OperationKind,
                    value: step.operation_kind.clone(),
                });
            }
        }

        for step in &self.steps {
            for dependency in &step.dependencies {
                if dependency == &step.id {
                    return Err(ExecutionPlanError::SelfDependency {
                        step: step.id.clone(),
                    });
                }

                if !index_by_id.contains_key(dependency) {
                    return Err(ExecutionPlanError::UnknownDependency {
                        step: step.id.clone(),
                        dependency: dependency.clone(),
                    });
                }
            }
        }

        let mut seen_demands = BTreeSet::new();

        for demand in &self.resource_demands {
            let key = (demand.resource_class.clone(), demand.unit.clone());

            if !seen_demands.insert(key) {
                return Err(ExecutionPlanError::DuplicateResourceDemand {
                    resource_class: demand.resource_class.clone(),
                    unit: demand.unit.clone(),
                });
            }
        }

        // Detect cycles as part of validation.
        let _ = self.topological_order_unchecked(&index_by_id)?;

        Ok(())
    }

    /// Return all steps in deterministic topological order.
    ///
    /// Every dependency precedes the step that depends on it. When multiple
    /// steps are ready, the lexicographically smallest `StepId` is selected.
    ///
    /// The algorithm uses Kahn's method and ordered collections, avoiding
    /// recursive graph traversal and its call-stack growth on long chains.
    pub fn topological_order(&self) -> ExecutionPlanResult<Vec<StepId>> {
        let index_by_id = self.validate_without_cycle_check()?;
        self.topological_order_unchecked(&index_by_id)
    }

    fn validate_without_cycle_check(
        &self,
    ) -> ExecutionPlanResult<BTreeMap<StepId, usize>> {
        if self.schema_version != EXECUTION_PLAN_SCHEMA_VERSION {
            return Err(ExecutionPlanError::UnsupportedSchemaVersion {
                supplied: self.schema_version,
                supported: EXECUTION_PLAN_SCHEMA_VERSION,
            });
        }

        if !is_valid_identifier(&self.id) {
            return Err(ExecutionPlanError::InvalidPlanIdentifier {
                value: self.id.clone(),
            });
        }

        let mut index_by_id = BTreeMap::new();

        for (index, step) in self.steps.iter().enumerate() {
            if index_by_id.insert(step.id.clone(), index).is_some() {
                return Err(ExecutionPlanError::DuplicateStep {
                    step: step.id.clone(),
                });
            }

            if !is_valid_identifier(&step.operation_kind) {
                return Err(ExecutionPlanError::InvalidIdentifier {
                    kind: IdentifierKind::OperationKind,
                    value: step.operation_kind.clone(),
                });
            }
        }

        for step in &self.steps {
            for dependency in &step.dependencies {
                if dependency == &step.id {
                    return Err(ExecutionPlanError::SelfDependency {
                        step: step.id.clone(),
                    });
                }

                if !index_by_id.contains_key(dependency) {
                    return Err(ExecutionPlanError::UnknownDependency {
                        step: step.id.clone(),
                        dependency: dependency.clone(),
                    });
                }
            }
        }

        let mut seen_demands = BTreeSet::new();

        for demand in &self.resource_demands {
            if !seen_demands.insert((
                demand.resource_class.clone(),
                demand.unit.clone(),
            )) {
                return Err(ExecutionPlanError::DuplicateResourceDemand {
                    resource_class: demand.resource_class.clone(),
                    unit: demand.unit.clone(),
                });
            }
        }

        Ok(index_by_id)
    }

    fn topological_order_unchecked(
        &self,
        index_by_id: &BTreeMap<StepId, usize>,
    ) -> ExecutionPlanResult<Vec<StepId>> {
        let mut remaining_dependencies: BTreeMap<StepId, usize> = BTreeMap::new();
        let mut dependents: BTreeMap<StepId, BTreeSet<StepId>> = BTreeMap::new();
        let mut ready = BTreeSet::new();

        for step in &self.steps {
            remaining_dependencies.insert(step.id.clone(), step.dependencies.len());

            if step.dependencies.is_empty() {
                ready.insert(step.id.clone());
            }

            for dependency in &step.dependencies {
                dependents
                    .entry(dependency.clone())
                    .or_default()
                    .insert(step.id.clone());
            }
        }

        let mut ordered = Vec::with_capacity(self.steps.len());

        while let Some(next) = ready.pop_first() {
            ordered.push(next.clone());

            if let Some(children) = dependents.get(&next) {
                for child in children {
                    let remaining = remaining_dependencies
                        .get_mut(child)
                        .expect("validated dependency graph contains every step");

                    *remaining -= 1;

                    if *remaining == 0 {
                        ready.insert(child.clone());
                    }
                }
            }
        }

        if ordered.len() != index_by_id.len() {
            return Err(ExecutionPlanError::DependencyCycle {
                unresolved_steps: index_by_id.len() - ordered.len(),
            });
        }

        Ok(ordered)
    }
}

/// Validate one open-ended identifier.
///
/// This is a representation contract, not a grammar for Zamani source names.
/// Source names must already have been parsed and resolved by the frontend.
fn is_valid_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().any(|byte| byte.is_ascii_alphanumeric())
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric()
                || matches!(byte, b'.' | b'_' | b'-' | b':' | b'/')
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn step(id: &str) -> PlanStep {
        PlanStep::new(
            StepId::new(id).expect("valid step ID"),
            "classical.operation",
        )
        .expect("valid operation kind")
    }

    #[test]
    fn empty_plan_is_valid() {
        let plan = ExecutionPlan::new("example.plan").expect("valid plan");
        assert!(plan.validate().is_ok());
        assert!(plan.topological_order().expect("valid order").is_empty());
    }

    #[test]
    fn dependencies_precede_dependent_steps() {
        let mut plan = ExecutionPlan::new("dependency.test").expect("valid plan");

        let first = step("first");
        let mut second = step("second");
        second.depends_on(first.id().clone());

        plan.add_step(second);
        plan.add_step(first);

        let order = plan.topological_order().expect("acyclic plan");
        let first_index = order.iter().position(|id| id.as_str() == "first");
        let second_index = order.iter().position(|id| id.as_str() == "second");

        assert!(first_index.is_some());
        assert!(second_index.is_some());
        assert!(first_index < second_index);
    }

    #[test]
    fn ready_steps_are_ordered_deterministically() {
        let mut plan = ExecutionPlan::new("determinism.test").expect("valid plan");
        plan.add_step(step("z"));
        plan.add_step(step("a"));
        plan.add_step(step("m"));

        let order = plan.topological_order().expect("valid order");

        assert_eq!(
            order.iter().map(StepId::as_str).collect::<Vec<_>>(),
            vec!["a", "m", "z"]
        );
    }

    #[test]
    fn duplicate_step_identifiers_are_rejected() {
        let mut plan = ExecutionPlan::new("duplicate.test").expect("valid plan");
        plan.add_step(step("same"));
        plan.add_step(step("same"));

        assert!(matches!(
            plan.validate(),
            Err(ExecutionPlanError::DuplicateStep { .. })
        ));
    }

    #[test]
    fn unknown_dependencies_are_rejected() {
        let mut plan = ExecutionPlan::new("unknown-dependency.test")
            .expect("valid plan");

        let mut current = step("current");
        current.depends_on(StepId::new("missing").expect("valid ID"));
        plan.add_step(current);

        assert!(matches!(
            plan.validate(),
            Err(ExecutionPlanError::UnknownDependency { .. })
        ));
    }

    #[test]
    fn self_dependencies_are_rejected() {
        let mut plan = ExecutionPlan::new("self-dependency.test")
            .expect("valid plan");

        let mut current = step("current");
        current.depends_on(current.id().clone());
        plan.add_step(current);

        assert!(matches!(
            plan.validate(),
            Err(ExecutionPlanError::SelfDependency { .. })
        ));
    }

    #[test]
    fn dependency_cycles_are_rejected() {
        let mut plan = ExecutionPlan::new("cycle.test").expect("valid plan");

        let mut first = step("first");
        first.depends_on(StepId::new("second").expect("valid ID"));

        let mut second = step("second");
        second.depends_on(StepId::new("first").expect("valid ID"));

        plan.add_step(first);
        plan.add_step(second);

        assert!(matches!(
            plan.validate(),
            Err(ExecutionPlanError::DependencyCycle { .. })
        ));
    }

    #[test]
    fn capability_references_are_deduplicated() {
        let mut plan = ExecutionPlan::new("capability.test").expect("valid plan");
        let capability =
            CapabilityReference::new("classical.integer.checked").expect("valid capability");

        plan.require_capability(capability.clone());
        plan.require_capability(capability);

        assert_eq!(plan.required_capabilities().len(), 1);
    }

    #[test]
    fn duplicate_resource_demands_are_not_silently_merged() {
        let mut plan = ExecutionPlan::new("resource.test").expect("valid plan");

        plan.require_resource(
            ResourceDemand::new("memory.bytes", "bytes", 1024)
                .expect("valid resource demand"),
        );
        plan.require_resource(
            ResourceDemand::new("memory.bytes", "bytes", 2048)
                .expect("valid resource demand"),
        );

        assert!(matches!(
            plan.validate(),
            Err(ExecutionPlanError::DuplicateResourceDemand { .. })
        ));
    }

    #[test]
    fn large_quantities_are_not_reduced_to_machine_word_size() {
        let quantity = u128::MAX;
        let demand = ResourceDemand::new("custom.resource", "units", quantity)
            .expect("valid demand");

        assert_eq!(demand.minimum(), u128::MAX);
    }

    #[test]
    fn invalid_identifiers_are_rejected() {
        assert!(StepId::new("").is_err());
        assert!(StepId::new("contains whitespace").is_err());
        assert!(CapabilityReference::new("capability name").is_err());
        assert!(ResourceDemand::new("memory", "", 1).is_err());
    }

    #[test]
    fn plans_do_not_depend_on_step_insertion_order_for_valid_ordering() {
        let mut left = ExecutionPlan::new("left").expect("valid plan");
        left.add_step(step("b"));
        left.add_step(step("a"));

        let mut right = ExecutionPlan::new("right").expect("valid plan");
        right.add_step(step("a"));
        right.add_step(step("b"));

        assert_eq!(
            left.topological_order().expect("valid order"),
            right.topological_order().expect("valid order")
        );
    }
}
