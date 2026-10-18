#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Resource Integration
//!
//! Path: `src/hdl/resources.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97+
//! Safety: Safe Rust only.
//!
//! ## Status
//!
//! Integration implementation. The API must pass the repository's actual
//! workspace checks and HDL conformance tests before it is declared frozen.
//!
//! ## Purpose
//!
//! Provide the HDL-facing boundary for canonical resource requirements and
//! target-independent resource estimates.
//!
//! This module adapts the shared semantic resource model. It does not create
//! a second resource identity, quantity, unit, symbolic-expression, or
//! constraint-evaluation system.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/semantic-boundaries.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/parameters.g4`
//! - `grammar/resources/resource.g4`
//! - `grammar/resources/resource-expressions.g4`
//! - `src/frontend/ast/node/resources/resource.rs`
//! - `src/frontend/ast/node/resources/resource_kind.rs`
//! - `src/frontend/semantic/resources/model.rs`
//! - `src/frontend/semantic/resources/symbolic.rs`
//! - `src/frontend/semantic/resources/requirements.rs`
//! - `src/frontend/semantic/resources/constraints.rs`
//!
//! The grammar owns syntax. The canonical AST owns source structure and
//! source-node identities. The shared semantic resource model owns resource
//! kinds, quantities, units, requirements, constraints, and resource intent.
//! The shared symbolic and constraint modules own their respective algorithms.
//!
//! ## Owns
//!
//! - HDL-facing names for canonical resource-model types.
//! - A constructor for open-ended HDL-namespaced resource kinds.
//! - An ordered collection of canonical resource requirements and estimates.
//! - A target-independent record of an HDL resource estimate.
//! - Local validation of estimate records.
//!
//! ## Does not own
//!
//! - A competing resource or quantity model.
//! - Source parsing or a replacement AST.
//! - Symbol resolution or general constant-expression evaluation.
//! - Unit conversion or dimensional analysis.
//! - Capability identity, evidence, or compatibility evaluation.
//! - Target inventories, target feasibility, or resource allocation.
//! - Scheduling, placement, routing, synthesis, or physical timing closure.
//! - Classical IR, quantum IR, or HDLIR.
//! - Runtime resource handles or physical device identifiers.
//!
//! ## Scalability and POCO-REAF
//!
//! This module defines no universal maximum for resource kinds, requirements,
//! estimates, bit widths, memory capacities, devices, or hardware instances.
//!
//! Resource quantities reuse the shared model, which represents exact
//! non-negative quantities as decimal values and can preserve symbolic or
//! runtime expression references without forcing them into `usize`, `u64`,
//! or `u128`.
//!
//! Collections grow subject to available memory and explicit operational
//! policies. This is not a claim of literal infinity or unlimited compilation.
//! Operational exhaustion must be reported by the owning compilation or
//! analysis session; incomplete analysis must not be presented as complete.
//!
//! ## Integration
//!
//! 1. The frontend converts source declarations into canonical semantic
//!    `ResourceRequirement` values.
//! 2. The HDL adapter collects those values into `HdlResourceProfile`.
//! 3. HDL analysis may attach `HdlResourceEstimate` records without claiming
//!    that the estimates are physical measurements or allocations.
//! 4. Later target-feasibility code compares requirements and estimates with
//!    explicit target descriptions using the appropriate shared contracts.
//! 5. Physical realization and HDLIR remain downstream.
//!
//! The `crate::frontend` module path must be exposed and compile in the
//! current crate before this module can compile as part of the workspace.
//!
//! ## Determinism
//!
//! Profiles preserve insertion order. This module does not reorder, merge,
//! or silently discard requirements or estimates. Consumers requiring
//! canonical serialization must apply an explicitly defined canonical ordering
//! at the serialization boundary.
//!
//! ## Freeze criteria
//!
//! - Shared resource types are reused rather than duplicated.
//! - Symbolic quantities remain symbolic until their canonical evaluator can
//!   resolve them.
//! - Requirements and estimates remain distinguishable.
//! - Unknown resource kinds remain representable.
//! - No physical allocation or target feasibility is implied.
//! - Tests and workspace checks pass on the supported Rust toolchain.

use core::fmt;
use std::error::Error;

use crate::frontend::ast::node::resources::resource_kind::{
    ResourceKind,
    ResourceKindError,
};
use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::semantic::resources::model::{
    ResourceConstraint,
    ResourceIntent,
    ResourceModelError,
    ResourceQuantity,
    ResourceRequirement,
    ResourceUnit,
};

/// Canonical resource kind used by HDL resource declarations.
///
/// This is an alias for the existing AST resource-kind identity, not a second
/// HDL-specific identity type. Resource kinds are open-ended; consumers must
/// not require a closed list of supported hardware resources.
pub type HdlResourceKind = ResourceKind;

/// Canonical semantic resource quantity.
///
/// Known quantities, symbolic expressions, runtime-derived quantities, and
/// unspecified quantities retain the meaning assigned by the shared model.
pub type HdlResourceQuantity = ResourceQuantity;

/// Canonical semantic resource unit.
///
/// Unit compatibility and conversion remain owned by the shared resource
/// subsystem and its unit-registry or constraint-evaluation services.
pub type HdlResourceUnit = ResourceUnit;

/// Canonical semantic resource requirement.
///
/// The shared requirement retains resource intent and any available source
/// provenance. It does not represent a physical allocation.
pub type HdlResourceRequirement = ResourceRequirement;

/// Canonical semantic resource constraint.
///
/// The shared constraint evaluator determines whether a constraint is
/// satisfied, violated, deferred, or invalid when sufficient inputs exist.
pub type HdlResourceConstraint = ResourceConstraint;

/// Namespace used by the convenience constructor for HDL-specific resources.
///
/// This is a naming convention, not a closed registry of hardware resources.
/// General-purpose resources such as memory, bandwidth, and compute may use
/// their existing shared namespaces instead.
pub const HDL_RESOURCE_NAMESPACE: &str = "hdl";

/// Construct an open-ended resource kind in the HDL namespace.
///
/// `name` is validated by the canonical AST resource-kind constructor.
/// Names are preserved according to that constructor's rules; this function
/// does not normalize aliases or assert that a kind is implemented.
///
/// # Examples
///
/// ```
/// use zamani::hdl::resources::hdl_resource_kind;
///
/// let kind = hdl_resource_kind("logic-elements").unwrap();
/// assert_eq!(kind.namespace(), "hdl");
/// assert_eq!(kind.name(), "logic-elements");
/// ```
///
/// The example assumes the crate exposes `hdl` from its public library root.
pub fn hdl_resource_kind(
    name: impl Into<String>,
) -> Result<HdlResourceKind, ResourceKindError> {
    ResourceKind::new(HDL_RESOURCE_NAMESPACE, name)
}

/// Classification of an HDL resource estimate.
///
/// The classification describes the origin or precision of an estimate. It
/// does not claim that a target has accepted the estimate or allocated the
/// corresponding physical resources.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HdlEstimateKind {
    /// A static estimate derived from a design-level analysis.
    Static,

    /// An estimate that depends on symbolic parameters or assumptions.
    Parametric,

    /// An estimate derived from a simulation or an external analysis result.
    Observed,
}

/// A target-independent estimate of a design's resource use.
///
/// This record is intentionally separate from `HdlResourceRequirement`.
/// Requirements express what a design needs; estimates describe a computed
/// or observed expectation. Neither one establishes target feasibility.
///
/// `quantity` may be exact, symbolic, or runtime-derived according to the
/// canonical resource model. An unspecified quantity is rejected because it
/// cannot express an estimate.
///
/// The resource kind and unit use the canonical shared types. The optional
/// source node identifies existing AST provenance when an estimate can be
/// traced to a source declaration or expression.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlResourceEstimate {
    kind: HdlResourceKind,
    quantity: HdlResourceQuantity,
    unit: Option<HdlResourceUnit>,
    estimate_kind: HdlEstimateKind,
    source_node: Option<NodeId>,
}

impl HdlResourceEstimate {
    /// Construct a validated resource estimate.
    ///
    /// This constructor checks local invariants only. It does not evaluate
    /// symbolic quantities, convert units, validate target capacities, or
    /// allocate physical resources.
    pub fn new(
        kind: HdlResourceKind,
        quantity: HdlResourceQuantity,
        unit: Option<HdlResourceUnit>,
        estimate_kind: HdlEstimateKind,
        source_node: Option<NodeId>,
    ) -> Result<Self, HdlResourceError> {
        kind.validate().map_err(|error| {
            HdlResourceError::InvalidResourceKind(error)
        })?;

        if matches!(quantity, ResourceQuantity::Unspecified) {
            return Err(HdlResourceError::UnspecifiedEstimateQuantity);
        }

        Ok(Self {
            kind,
            quantity,
            unit,
            estimate_kind,
            source_node,
        })
    }

    /// Return the canonical resource kind.
    #[must_use]
    pub fn kind(&self) -> &HdlResourceKind {
        &self.kind
    }

    /// Return the canonical quantity without forcing symbolic resolution.
    #[must_use]
    pub fn quantity(&self) -> &HdlResourceQuantity {
        &self.quantity
    }

    /// Return the declared unit, if one was supplied.
    #[must_use]
    pub fn unit(&self) -> Option<&HdlResourceUnit> {
        self.unit.as_ref()
    }

    /// Return the estimate classification.
    #[must_use]
    pub const fn estimate_kind(&self) -> HdlEstimateKind {
        self.estimate_kind
    }

    /// Return the originating AST node, if available.
    #[must_use]
    pub const fn source_node(&self) -> Option<NodeId> {
        self.source_node
    }
}

/// Errors from constructing an HDL resource estimate.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HdlResourceError {
    /// The supplied resource kind failed canonical validation.
    InvalidResourceKind(ResourceKindError),

    /// An estimate was created with an unspecified quantity.
    UnspecifiedEstimateQuantity,

    /// The shared semantic resource model rejected an input.
    SharedModel(ResourceModelError),
}

impl fmt::Display for HdlResourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidResourceKind(error) => {
                write!(formatter, "invalid HDL resource kind: {error}")
            }
            Self::UnspecifiedEstimateQuantity => {
                formatter.write_str(
                    "an HDL resource estimate requires a known, symbolic, \
                     or runtime-derived quantity",
                )
            }
            Self::SharedModel(error) => {
                write!(formatter, "shared resource-model error: {error}")
            }
        }
    }
}

impl Error for HdlResourceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidResourceKind(error) => Some(error),
            Self::UnspecifiedEstimateQuantity => None,
            Self::SharedModel(error) => Some(error),
        }
    }
}

impl From<ResourceModelError> for HdlResourceError {
    fn from(error: ResourceModelError) -> Self {
        Self::SharedModel(error)
    }
}

/// An ordered collection of canonical HDL resource requirements and estimates.
///
/// The profile is a grouping and integration boundary, not a resource solver.
/// It preserves insertion order and repeated entries because multiple
/// requirements for the same resource kind can be meaningful when they differ
/// in intent, scope, units, or provenance.
///
/// Target capacity and physical allocation do not belong to this profile;
/// those facts must come from explicit target and realization interfaces.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HdlResourceProfile {
    requirements: Vec<HdlResourceRequirement>,
    estimates: Vec<HdlResourceEstimate>,
}

impl HdlResourceProfile {
    /// Create an empty resource profile.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            requirements: Vec::new(),
            estimates: Vec::new(),
        }
    }

    /// Append a canonical requirement without changing its intent or
    /// provenance.
    pub fn push_requirement(&mut self, requirement: HdlResourceRequirement) {
        self.requirements.push(requirement);
    }

    /// Append a previously validated estimate.
    pub fn push_estimate(&mut self, estimate: HdlResourceEstimate) {
        self.estimates.push(estimate);
    }

    /// Return all requirements in insertion order.
    #[must_use]
    pub fn requirements(&self) -> &[HdlResourceRequirement] {
        &self.requirements
    }

    /// Return all estimates in insertion order.
    #[must_use]
    pub fn estimates(&self) -> &[HdlResourceEstimate] {
        &self.estimates
    }

    /// Return the number of requirements.
    #[must_use]
    pub fn requirement_count(&self) -> usize {
        self.requirements.len()
    }

    /// Return the number of estimates.
    #[must_use]
    pub fn estimate_count(&self) -> usize {
        self.estimates.len()
    }

    /// Return whether the profile contains no requirements or estimates.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty() && self.estimates.is_empty()
    }

    /// Consume the profile and return its requirements and estimates.
    ///
    /// This operation preserves insertion order in both collections.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (Vec<HdlResourceRequirement>, Vec<HdlResourceEstimate>) {
        (self.requirements, self.estimates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn logic_resource() -> HdlResourceKind {
        hdl_resource_kind("logic-elements")
            .expect("valid open-ended HDL resource kind")
    }

    #[test]
    fn hdl_resource_kinds_are_open_ended() {
        let first = hdl_resource_kind("future-reconfigurable-fabric")
            .expect("unknown resource kinds remain representable");
        let second = hdl_resource_kind("photonic-switching-elements")
            .expect("future resource kinds remain representable");

        assert_eq!(first.namespace(), HDL_RESOURCE_NAMESPACE);
        assert_eq!(first.name(), "future-reconfigurable-fabric");
        assert_ne!(first, second);
    }

    #[test]
    fn shared_requirement_model_is_used() {
        let requirement = HdlResourceRequirement::new(
            logic_resource(),
            HdlResourceQuantity::known("100000000000000000000000000000000000000")
                .expect("exact quantity"),
            None,
            ResourceIntent::Required,
        )
        .expect("valid canonical requirement");

        let mut profile = HdlResourceProfile::new();
        profile.push_requirement(requirement);

        assert_eq!(profile.requirement_count(), 1);
        assert_eq!(profile.estimate_count(), 0);
    }

    #[test]
    fn estimates_preserve_symbolic_quantities() {
        let expression = NodeId::default();
        let estimate = HdlResourceEstimate::new(
            logic_resource(),
            ResourceQuantity::Symbolic(expression),
            None,
            HdlEstimateKind::Parametric,
            Some(expression),
        )
        .expect("symbolic estimates are valid");

        assert_eq!(estimate.quantity(), &ResourceQuantity::Symbolic(expression));
        assert_eq!(estimate.source_node(), Some(expression));
        assert_eq!(estimate.estimate_kind(), HdlEstimateKind::Parametric);
    }

    #[test]
    fn unspecified_estimate_quantities_are_rejected() {
        let result = HdlResourceEstimate::new(
            logic_resource(),
            ResourceQuantity::Unspecified,
            None,
            HdlEstimateKind::Static,
            None,
        );

        assert_eq!(
            result,
            Err(HdlResourceError::UnspecifiedEstimateQuantity)
        );
    }

    #[test]
    fn profile_preserves_repeated_requirements_and_insertion_order() {
        let first = HdlResourceRequirement::new(
            logic_resource(),
            ResourceQuantity::known("4").expect("valid quantity"),
            None,
            ResourceIntent::Required,
        )
        .expect("valid requirement");

        let second = HdlResourceRequirement::new(
            logic_resource(),
            ResourceQuantity::known("8").expect("valid quantity"),
            None,
            ResourceIntent::Preferred,
        )
        .expect("valid requirement");

        let mut profile = HdlResourceProfile::new();
        profile.push_requirement(first.clone());
        profile.push_requirement(second.clone());

        assert_eq!(profile.requirements(), &[first, second]);
        assert_eq!(profile.requirement_count(), 2);
    }

    #[test]
    fn estimate_and_requirement_remain_distinct() {
        let requirement = HdlResourceRequirement::new(
            logic_resource(),
            ResourceQuantity::known("16").expect("valid quantity"),
            None,
            ResourceIntent::Required,
        )
        .expect("valid requirement");

        let estimate = HdlResourceEstimate::new(
            logic_resource(),
            ResourceQuantity::known("12").expect("valid quantity"),
            None,
            HdlEstimateKind::Static,
            None,
        )
        .expect("valid estimate");

        let mut profile = HdlResourceProfile::new();
        profile.push_requirement(requirement);
        profile.push_estimate(estimate);

        assert_eq!(profile.requirement_count(), 1);
        assert_eq!(profile.estimate_count(), 1);
    }
}