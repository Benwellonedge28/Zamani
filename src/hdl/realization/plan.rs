
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Realization Plan
//!
//! Path: `src/hdl/realization/plan.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97+.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Represent a deterministic, declarative request to realize an HDL design
//! for one explicitly identified target description.
//!
//! This is a planning contract. It records the selected target identity,
//! canonical source requirements, resource information, target-scoped
//! constraints, requested provider operations, unresolved decisions, and
//! optional feasibility evidence.
//!
//! A plan is not an executable hardware implementation, a physical
//! allocation, or proof that a provider can successfully realize a design.
//!
//! ## Normative authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/physical-intent.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/semantic-boundaries.md`
//! - `grammar/specification/scalability-model.md`
//!
//! ## Existing implementation dependencies
//!
//! - `crate::hdl::requirements` owns canonical HDL requirement records.
//! - `crate::hdl::resources` owns the HDL resource profile.
//! - `crate::hdl::targets::model` owns target identity and validation.
//! - `crate::hdl::targets::constraints` owns target-scoped constraints.
//! - `crate::hdl::targets::feasibility` owns feasibility assessment.
//!
//! The grammar owns syntax, the frontend owns source meaning, the shared
//! semantic models own resource and capability semantics, and the target
//! subsystem owns target descriptions and feasibility assessment.
//!
//! ## Owns
//!
//! - The realization-plan data contract.
//! - Deterministically ordered provider-operation requests.
//! - Explicitly recorded unresolved implementation decisions.
//! - Target-scope consistency between the plan, constraints, and feasibility
//!   evidence.
//! - Plan-local structural validation.
//!
//! ## Does not own
//!
//! - Parsing, AST construction, source spans, or semantic validation.
//! - A replacement requirements, capability, resource, or target model.
//! - Target discovery, provider authentication, or provider availability.
//! - Feasibility evaluation or resource-capacity evaluation.
//! - Hardware allocation, synthesis, scheduling, placement, or routing.
//! - Physical timing closure, deployment, or runtime device handles.
//! - Classical IR, quantum IR, ZUIR, or HDLIR.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is deferred.
//!
//! ## Plan semantics
//!
//! A plan describes a request, not a completed action.
//!
//! - A target ID identifies the target description, not a physical device.
//! - A provider operation is an open-ended namespaced identifier, not a
//!   promise that an implementation exists.
//! - An unresolved decision is an explicit blocker or pending choice.
//! - Requirements and resource profiles preserve their canonical semantics.
//! - Target constraints must be associated with the same target identity.
//! - Feasibility evidence is optional and remains a snapshot of an earlier
//!   assessment; attaching it does not rerun or authenticate that assessment.
//! - An indeterminate assessment must never be upgraded to feasible here.
//!
//! This module deliberately does not invent a source-design fingerprint.
//! Reproducible content fingerprints belong to a separately specified,
//! versioned canonicalization contract.
//!
//! ## Scalability
//!
//! There are no universal maximum counts for plan operations, decisions,
//! requirements, constraints, resource kinds, targets, or hardware units.
//!
//! Collections grow subject to available memory, address space, execution
//! time, and explicit operational budgets supplied by the owning workflow.
//! Rust's collection and string types have finite representational limits;
//! literal infinite storage or computation is not promised.
//!
//! This module defines no fixed hardware capacities, register widths, device
//! counts, memory sizes, or vendor-specific resource assumptions.
//!
//! ## Determinism
//!
//! Operations and unresolved decisions are keyed by validated identifiers
//! in `BTreeMap`s. Iteration order is lexicographic by identifier.
//! Re-inserting an identical record is idempotent. Reusing an identifier for
//! different data is rejected without mutating the plan.
//!
//! Requirement and resource-profile ordering is delegated to their canonical
//! models and is not silently rewritten by this module.
//!
//! ## Integration sequence
//!
//! 1. Frontend semantic analysis constructs canonical HDL requirements.
//! 2. Resource analysis supplies an `HdlResourceProfile`.
//! 3. A target provider supplies a validated `HdlTargetDescription`.
//! 4. Target-scoped constraints are validated against that description.
//! 5. Target feasibility is assessed independently.
//! 6. This module assembles a declarative plan using those existing contracts.
//! 7. A future realization coordinator consumes the plan and negotiates with
//!    a provider.
//! 8. HDLIR integration, lowering, and physical implementation remain outside
//!    this file and within their separately owned subsystems.
//!
//! ## Freeze criteria
//!
//! - Existing canonical models are reused rather than duplicated.
//! - A target-scope mismatch is rejected.
//! - Conflicting identifiers cannot silently overwrite existing entries.
//! - Missing feasibility evidence is not treated as success.
//! - Unresolved decisions remain observable.
//! - Plan creation performs no hardware allocation or provider invocation.
//! - Unit tests and workspace checks pass on the actual supported toolchain.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::hdl::requirements::HdlRequirements;
use crate::hdl::resources::HdlResourceProfile;
use crate::hdl::targets::constraints::{
    HdlTargetConstraints,
    HdlTargetConstraintsError,
};
use crate::hdl::targets::feasibility::{
    FeasibilityStatus,
    HdlTargetFeasibilityReport,
};
use crate::hdl::targets::model::{
    HdlTargetDescription,
    HdlTargetModelError,
    TargetId,
};

/// Stable architectural name of this module.
pub const HDL_REALIZATION_PLAN_MODULE_NAME: &str =
    "zamani.hdl.realization.plan";

/// Version of the in-memory realization-plan contract.
///
/// This is not the Zamani language version, Rust version, target provider
/// version, or HDLIR schema version.
pub const HDL_REALIZATION_PLAN_SCHEMA_VERSION: u16 = 1;

/// An open-ended operation requested from a realization provider.
///
/// Examples of possible namespaced identifiers include
/// `synthesis.prepare`, `timing.analyze`, or `implementation.package`.
//!
//! The identifiers are descriptive contracts. This module does not maintain
//! a closed operation registry and does not claim that any operation is
//! supported by a particular provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlRealizationOperation {
    id: String,
    description: Option<String>,
    required: bool,
}

impl HdlRealizationOperation {
    /// Construct an operation request.
    ///
    /// The ID must be nonempty and contain no control characters. Its exact
    /// spelling is preserved. This constructor does not check whether an
    /// implementation provider supports the operation.
    pub fn new(
        id: impl Into<String>,
        description: Option<String>,
        required: bool,
    ) -> Result<Self, HdlRealizationPlanError> {
        let id = id.into();
        validate_text(&id, "operation identifier")?;

        if let Some(value) = description.as_deref() {
            validate_text(value, "operation description")?;
        }

        Ok(Self {
            id,
            description,
            required,
        })
    }

    /// Return the exact operation identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the optional human-readable description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Return whether the operation is mandatory for this request.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.required
    }
}

/// An implementation choice that has not yet been resolved.
///
/// A decision can remain unresolved because it requires provider information,
/// target evidence, an explicit policy choice, or a later implementation
/// stage. Recording it does not imply that it can be resolved automatically.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlUnresolvedDecision {
    id: String,
    description: String,
    blocking: bool,
}

impl HdlUnresolvedDecision {
    /// Construct an unresolved implementation decision.
    ///
    /// Both the ID and description must be nonempty and contain no control
    /// characters. `blocking` declares whether the caller considers the
    /// unresolved decision a blocker for proceeding to the next stage.
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
        blocking: bool,
    ) -> Result<Self, HdlRealizationPlanError> {
        let id = id.into();
        let description = description.into();

        validate_text(&id, "decision identifier")?;
        validate_text(&description, "decision description")?;

        Ok(Self {
            id,
            description,
            blocking,
        })
    }

    /// Return the exact decision identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the decision description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Return whether this decision blocks the next realization stage.
    #[must_use]
    pub const fn is_blocking(&self) -> bool {
        self.blocking
    }
}

/// A declarative HDL realization plan.
///
/// The plan owns its collections but reuses the repository's canonical
/// requirements, resource profile, target constraints, target ID, and
/// feasibility report. It does not duplicate or reinterpret those models.
///
/// The plan can be constructed without feasibility evidence. In that case,
/// its feasibility status is reported as unknown rather than inferred from
/// the presence or absence of requirements.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlRealizationPlan {
    target_id: TargetId,
    requirements: HdlRequirements,
    resources: HdlResourceProfile,
    constraints: HdlTargetConstraints,
    operations: BTreeMap<String, HdlRealizationOperation>,
    unresolved_decisions: BTreeMap<String, HdlUnresolvedDecision>,
    feasibility: Option<HdlTargetFeasibilityReport>,
}

impl HdlRealizationPlan {
    /// Create a plan for a validated target description.
    ///
    /// An empty requirement set, resource profile, and constraint collection
    /// do not imply that the design is feasible. This constructor validates
    /// the target description and initializes empty plan-local collections.
    ///
    /// # Errors
    ///
    /// Returns an error if the target description fails its canonical local
    /// validation.
    pub fn new(
        target: &HdlTargetDescription,
    ) -> Result<Self, HdlRealizationPlanError> {
        target
            .validate()
            .map_err(HdlRealizationPlanError::InvalidTarget)?;

        let target_id = target.id().clone();
        let constraints = HdlTargetConstraints::new(target_id.clone());

        Ok(Self {
            target_id,
            requirements: HdlRequirements::new(),
            resources: HdlResourceProfile::new(),
            constraints,
            operations: BTreeMap::new(),
            unresolved_decisions: BTreeMap::new(),
            feasibility: None,
        })
    }

    /// Return the target-description identity associated with this plan.
    #[must_use]
    pub const fn target_id(&self) -> &TargetId {
        &self.target_id
    }

    /// Return the canonical HDL requirements.
    #[must_use]
    pub const fn requirements(&self) -> &HdlRequirements {
        &self.requirements
    }

    /// Return the canonical HDL resource profile.
    #[must_use]
    pub const fn resources(&self) -> &HdlResourceProfile {
        &self.resources
    }

    /// Return the target-scoped constraints.
    #[must_use]
    pub const fn constraints(&self) -> &HdlTargetConstraints {
        &self.constraints
    }

    /// Return the requested operations in deterministic identifier order.
    pub fn operations(
        &self,
    ) -> impl ExactSizeIterator<Item = &HdlRealizationOperation> {
        self.operations.values()
    }

    /// Return the unresolved decisions in deterministic identifier order.
    pub fn unresolved_decisions(
        &self,
    ) -> impl ExactSizeIterator<Item = &HdlUnresolvedDecision> {
        self.unresolved_decisions.values()
    }

    /// Return the optional feasibility assessment snapshot.
    ///
    /// The returned assessment is evidence from a previous assessment
    /// operation. This accessor does not establish that provider state,
    /// target capabilities, or resources remain unchanged since it was made.
    #[must_use]
    pub const fn feasibility(
        &self,
    ) -> Option<&HdlTargetFeasibilityReport> {
        self.feasibility.as_ref()
    }

    /// Return the recorded feasibility status, if an assessment is attached.
    ///
    /// `None` means no assessment has been attached. It does not mean
    /// feasible, infeasible, or that no assessment is needed.
    #[must_use]
    pub fn feasibility_status(&self) -> Option<FeasibilityStatus> {
        self.feasibility.as_ref().map(|report| report.status())
    }

    /// Return whether the plan contains any blocking unresolved decisions.
    #[must_use]
    pub fn has_blocking_decisions(&self) -> bool {
        self.unresolved_decisions
            .values()
            .any(HdlUnresolvedDecision::is_blocking)
    }

    /// Return whether a feasibility assessment has been attached.
    #[must_use]
    pub const fn has_feasibility_assessment(&self) -> bool {
        self.feasibility.is_some()
    }

    /// Return the number of requested operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Return the number of unresolved decisions.
    #[must_use]
    pub fn unresolved_decision_count(&self) -> usize {
        self.unresolved_decisions.len()
    }

    /// Replace the plan's canonical HDL requirements.
    ///
    /// The supplied collection is used as-is. This operation does not resolve
    /// symbolic requirements, evaluate feasibility, or change source meaning.
    pub fn set_requirements(&mut self, requirements: HdlRequirements) {
        self.requirements = requirements;
    }

    /// Replace the plan's resource profile.
    ///
    /// A resource profile represents requirements and estimates, not a target
    /// inventory or physical allocation. Replacing it invalidates no source
    /// semantics, but callers must reassess feasibility when resource evidence
    /// changes.
    pub fn set_resources(&mut self, resources: HdlResourceProfile) {
        self.resources = resources;
    }

    /// Replace target-scoped constraints after validating their target scope.
    ///
    /// The constraints must belong to the exact target ID associated with
    /// this plan. This check establishes scope consistency only; it does not
    /// evaluate any property or prove feasibility.
    ///
    /// # Errors
    ///
    /// Returns an error if the constraints do not match the plan's target or
    /// fail the canonical constraint validation.
    pub fn set_constraints(
        &mut self,
        constraints: HdlTargetConstraints,
        target: &HdlTargetDescription,
    ) -> Result<(), HdlRealizationPlanError> {
        self.ensure_target_matches(target)?;

        constraints
            .validate_for(target)
            .map_err(HdlRealizationPlanError::InvalidConstraints)?;

        if constraints.target_id() != &self.target_id {
            return Err(HdlRealizationPlanError::TargetMismatch {
                expected: self.target_id.as_str().to_owned(),
                actual: constraints.target_id().as_str().to_owned(),
            });
        }

        self.constraints = constraints;
        Ok(())
    }

    /// Add a provider-operation request.
    ///
    /// Inserting an identical operation with the same ID is idempotent.
    /// Inserting different operation data under an existing ID returns an
    /// error and leaves the plan unchanged.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation ID is already associated with
    /// different operation data.
    pub fn add_operation(
        &mut self,
        operation: HdlRealizationOperation,
    ) -> Result<(), HdlRealizationPlanError> {
        match self.operations.get(operation.id()) {
            Some(existing) if existing == &operation => Ok(()),
            Some(_) => Err(HdlRealizationPlanError::ConflictingIdentifier {
                kind: "operation",
                id: operation.id().to_owned(),
            }),
            None => {
                self.operations.insert(operation.id.clone(), operation);
                Ok(())
            }
        }
    }

    /// Add an unresolved implementation decision.
    ///
    /// Inserting an identical decision with the same ID is idempotent.
    /// Different data under an existing ID is rejected without mutation.
    ///
    /// # Errors
    ///
    /// Returns an error if the decision ID is already associated with
    /// different decision data.
    pub fn add_unresolved_decision(
        &mut self,
        decision: HdlUnresolvedDecision,
    ) -> Result<(), HdlRealizationPlanError> {
        match self.unresolved_decisions.get(decision.id()) {
            Some(existing) if existing == &decision => Ok(()),
            Some(_) => Err(HdlRealizationPlanError::ConflictingIdentifier {
                kind: "decision",
                id: decision.id().to_owned(),
            }),
            None => {
                self.unresolved_decisions
                    .insert(decision.id.clone(), decision);
                Ok(())
            }
        }
    }

    /// Attach a feasibility assessment for the plan's target.
    ///
    /// This validates target identity only. The report remains a snapshot and
    /// is not independently authenticated or recomputed here.
    ///
    /// Attaching an indeterminate report does not make the plan feasible.
    ///
    /// # Errors
    ///
    /// Returns an error if the report refers to a different target.
    pub fn set_feasibility(
        &mut self,
        report: HdlTargetFeasibilityReport,
    ) -> Result<(), HdlRealizationPlanError> {
        if report.target_id() != &self.target_id {
            return Err(HdlRealizationPlanError::TargetMismatch {
                expected: self.target_id.as_str().to_owned(),
                actual: report.target_id().as_str().to_owned(),
            });
        }

        self.feasibility = Some(report);
        Ok(())
    }

    /// Remove any attached feasibility assessment.
    ///
    /// This is useful after changing inputs that make the previous assessment
    /// stale. It does not run a replacement assessment.
    pub fn clear_feasibility(&mut self) {
        self.feasibility = None;
    }

    /// Validate plan-local and target-scope invariants.
    ///
    /// This method validates the supplied target description, confirms that
    /// its ID matches the plan, and validates the target-scoped constraints.
    /// It does not re-run semantic analysis, resolve symbolic expressions,
    /// assess physical capacities, or execute provider operations.
    ///
    /// # Errors
    ///
    /// Returns an error if the target or its constraint association is
    /// invalid.
    pub fn validate_for(
        &self,
        target: &HdlTargetDescription,
    ) -> Result<(), HdlRealizationPlanError> {
        self.ensure_target_matches(target)?;

        self.constraints
            .validate_for(target)
            .map_err(HdlRealizationPlanError::InvalidConstraints)?;

        // Recheck collection-key invariants defensively. The public API
        // currently maintains these invariants at insertion time.
        for (key, operation) in &self.operations {
            if key != operation.id() {
                return Err(HdlRealizationPlanError::InvariantViolation {
                    invariant: "operation map key equals operation ID",
                });
            }
        }

        for (key, decision) in &self.unresolved_decisions {
            if key != decision.id() {
                return Err(HdlRealizationPlanError::InvariantViolation {
                    invariant: "decision map key equals decision ID",
                });
            }
        }

        if let Some(report) = &self.feasibility {
            if report.target_id() != &self.target_id {
                return Err(HdlRealizationPlanError::InvariantViolation {
                    invariant: "feasibility report target equals plan target",
                });
            }
        }

        Ok(())
    }

    /// Return the stable architectural module name.
    #[must_use]
    pub const fn module_name() -> &'static str {
        HDL_REALIZATION_PLAN_MODULE_NAME
    }

    /// Return the in-memory plan schema version.
    #[must_use]
    pub const fn schema_version() -> u16 {
        HDL_REALIZATION_PLAN_SCHEMA_VERSION
    }

    fn ensure_target_matches(
        &self,
        target: &HdlTargetDescription,
    ) -> Result<(), HdlRealizationPlanError> {
        target
            .validate()
            .map_err(HdlRealizationPlanError::InvalidTarget)?;

        if target.id() != &self.target_id {
            return Err(HdlRealizationPlanError::TargetMismatch {
                expected: self.target_id.as_str().to_owned(),
                actual: target.id().as_str().to_owned(),
            });
        }

        Ok(())
    }
}

/// Errors raised while constructing or validating a realization plan.
///
/// These describe malformed plan data or inconsistent target associations.
/// They are not target-feasibility outcomes and must not be used to represent
/// a source-language semantic error.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HdlRealizationPlanError {
    /// The target description failed its canonical validation.
    InvalidTarget(HdlTargetModelError),

    /// Target-scoped constraints failed validation.
    InvalidConstraints(HdlTargetConstraintsError),

    /// Two different targets or target-scoped records were combined.
    TargetMismatch {
        /// Target identity expected by the plan.
        expected: String,
        /// Target identity supplied by the caller.
        actual: String,
    },

    /// An identifier was empty or contained control characters.
    InvalidText {
        /// Name of the invalid field.
        field: &'static str,
        /// Reason the field was rejected.
        reason: &'static str,
    },

    /// A previously used identifier was supplied with different data.
    ConflictingIdentifier {
        /// Kind of record using the identifier.
        kind: &'static str,
        /// Identifier that conflicts with an existing record.
        id: String,
    },

    /// A plan-local invariant failed validation.
    InvariantViolation {
        /// Stable descriptive name of the invariant.
        invariant: &'static str,
    },
}

impl fmt::Display for HdlRealizationPlanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTarget(error) => {
                write!(formatter, "invalid HDL realization target: {error}")
            }
            Self::InvalidConstraints(error) => {
                write!(
                    formatter,
                    "invalid HDL realization target constraints: {error}"
                )
            }
            Self::TargetMismatch { expected, actual } => {
                write!(
                    formatter,
                    "HDL realization target mismatch: expected `{expected}`, \
                     received `{actual}`"
                )
            }
            Self::InvalidText { field, reason } => {
                write!(formatter, "invalid {field}: {reason}")
            }
            Self::ConflictingIdentifier { kind, id } => {
                write!(
                    formatter,
                    "conflicting HDL realization {kind} identifier `{id}`"
                )
            }
            Self::InvariantViolation { invariant } => {
                write!(
                    formatter,
                    "HDL realization plan invariant violated: {invariant}"
                )
            }
        }
    }
}

impl Error for HdlRealizationPlanError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidTarget(error) => Some(error),
            Self::InvalidConstraints(error) => Some(error),
            Self::TargetMismatch { .. }
            | Self::InvalidText { .. }
            | Self::ConflictingIdentifier { .. }
            | Self::InvariantViolation { .. } => None,
        }
    }
}

/// Validate a required textual field without normalizing its value.
///
/// Exact spelling is preserved because operation IDs and decision IDs may
/// be defined by external providers. This check is representation validation,
/// not authentication or authorization.
fn validate_text(
    value: &str,
    field: &'static str,
) -> Result<(), HdlRealizationPlanError> {
    if value.trim().is_empty() {
        return Err(HdlRealizationPlanError::InvalidText {
            field,
            reason: "must not be empty",
        });
    }

    if value.chars().any(char::is_control) {
        return Err(HdlRealizationPlanError::InvalidText {
            field,
            reason: "must not contain control characters",
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hdl::targets::model::TargetId;

    fn target(id: &str) -> HdlTargetDescription {
        HdlTargetDescription::new(
            TargetId::new(id).expect("valid target ID"),
            "Test target",
            "test.provider",
        )
        .expect("valid target description")
    }

    fn operation(id: &str, required: bool) -> HdlRealizationOperation {
        HdlRealizationOperation::new(
            id,
            Some("test operation".to_owned()),
            required,
        )
        .expect("valid operation")
    }

    fn decision(id: &str, blocking: bool) -> HdlUnresolvedDecision {
        HdlUnresolvedDecision::new(
            id,
            "pending test decision",
            blocking,
        )
        .expect("valid decision")
    }

    #[test]
    fn new_plan_is_bound_to_the_supplied_target() {
        let target = target("test.simulator");
        let plan = HdlRealizationPlan::new(&target)
            .expect("target is valid");

        assert_eq!(plan.target_id().as_str(), "test.simulator");
        assert_eq!(plan.operation_count(), 0);
        assert_eq!(plan.unresolved_decision_count(), 0);
        assert!(plan.feasibility().is_none());
        assert_eq!(plan.feasibility_status(), None);
        assert!(plan.validate_for(&target).is_ok());
    }

    #[test]
    fn rejects_validation_for_another_target() {
        let first = target("target.alpha");
        let second = target("target.beta");
        let plan = HdlRealizationPlan::new(&first)
            .expect("target is valid");

        assert!(matches!(
            plan.validate_for(&second),
            Err(HdlRealizationPlanError::TargetMismatch { .. })
        ));
    }

    #[test]
    fn identical_operation_insertion_is_idempotent() {
        let target = target("test.target");
        let mut plan = HdlRealizationPlan::new(&target)
            .expect("target is valid");

        let entry = operation("synthesis.prepare", true);
        plan.add_operation(entry.clone()).expect("first insertion");
        plan.add_operation(entry).expect("identical insertion");

        assert_eq!(plan.operation_count(), 1);
    }

    #[test]
    fn conflicting_operation_identifier_is_rejected_atomically() {
        let target = target("test.target");
        let mut plan = HdlRealizationPlan::new(&target)
            .expect("target is valid");

        plan.add_operation(operation("synthesis.prepare", true))
            .expect("first insertion");

        let error = plan
            .add_operation(operation("synthesis.prepare", false))
            .expect_err("different data must conflict");

        assert!(matches!(
            error,
            HdlRealizationPlanError::ConflictingIdentifier {
                kind: "operation",
                ..
            }
        ));
        assert_eq!(plan.operation_count(), 1);
        assert!(plan.operations().next().expect("one operation").is_required());
    }

    #[test]
    fn operations_have_deterministic_identifier_order() {
        let target = target("test.target");
        let mut plan = HdlRealizationPlan::new(&target)
            .expect("target is valid");

        plan.add_operation(operation("z.operation", false))
            .expect("insert z");
        plan.add_operation(operation("a.operation", true))
            .expect("insert a");

        let ids: Vec<_> = plan.operations().map(|entry| entry.id()).collect();

        assert_eq!(ids, vec!["a.operation", "z.operation"]);
    }

    #[test]
    fn blocking_decisions_are_visible() {
        let target = target("test.target");
        let mut plan = HdlRealizationPlan::new(&target)
            .expect("target is valid");

        plan.add_unresolved_decision(decision("provider.selection", true))
            .expect("insert blocking decision");
        plan.add_unresolved_decision(decision("optional.report", false))
            .expect("insert nonblocking decision");

        assert!(plan.has_blocking_decisions());
        assert_eq!(plan.unresolved_decision_count(), 2);
    }

    #[test]
    fn conflicting_decision_does_not_replace_original() {
        let target = target("test.target");
        let mut plan = HdlRealizationPlan::new(&target)
            .expect("target is valid");

        plan.add_unresolved_decision(decision("target.policy", true))
            .expect("first insertion");

        assert!(matches!(
            plan.add_unresolved_decision(decision("target.policy", false)),
            Err(HdlRealizationPlanError::ConflictingIdentifier {
                kind: "decision",
                ..
            })
        ));

        assert!(plan
            .unresolved_decisions()
            .next()
            .expect("original decision remains")
            .is_blocking());
    }

    #[test]
    fn rejects_empty_or_control_character_identifiers() {
        assert!(HdlRealizationOperation::new(" ", None, true).is_err());
        assert!(HdlRealizationOperation::new("bad\nid", None, true).is_err());

        assert!(HdlUnresolvedDecision::new(" ", "description", true).is_err());
        assert!(HdlUnresolvedDecision::new("valid.id", " ", true).is_err());
    }

    #[test]
    fn clearing_feasibility_does_not_change_other_plan_data() {
        let target = target("test.target");
        let mut plan = HdlRealizationPlan::new(&target)
            .expect("target is valid");

        plan.add_operation(operation("analysis.run", true))
            .expect("operation insertion");

        plan.clear_feasibility();

        assert!(!plan.has_feasibility_assessment());
        assert_eq!(plan.operation_count(), 1);
    }

    #[test]
    fn schema_contract_is_stable() {
        assert_eq!(
            HdlRealizationPlan::module_name(),
            "zamani.hdl.realization.plan"
        );
        assert_eq!(HdlRealizationPlan::schema_version(), 1);
    }
}
