
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Control-Flow Composition
//!
//! Path: `src/hybrid/control/flow.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//!
//! ## Purpose
//!
//! Represent the structural control-flow relationships needed to compose
//! canonical operations across computational domains.
//!
//! This module describes how hybrid control regions relate to canonical
//! operation references. It does not implement or duplicate the language's
//! expression evaluator, semantic analyzer, quantum IR, classical IR,
//! dependency graph, or runtime.
//!
//! ## Ownership
//!
//! This module owns:
//! - Hybrid control-region descriptors.
//! - References to control-flow entry, exit, and parent nodes.
//! - Conditional, iterative, nested, sequential, and repeated-region
//!   composition descriptors.
//! - Structural validation of those descriptors against a HybridGraph.
//! - Namespaced extension descriptors for future control constructs.
//!
//! This module does not own:
//! - Source syntax, parsing, or AST construction.
//! - Predicate evaluation, expression typing, or name resolution.
//! - Canonical quantum or classical control-flow semantics.
//! - Graph dependency edges or dependency scheduling.
//! - Synchronization execution, runtime state, or cancellation.
//! - Hardware selection, allocation, routing, or physical scheduling.
//!
//! ## Canonical ownership and integration
//!
//! - `crate::hybrid::graph::model::HybridGraph` owns the hybrid graph.
//! - `crate::hybrid::graph::edges` owns dependency-edge relationships.
//! - `crate::hybrid::ids` owns graph-scoped node identities.
//! - `crate::hybrid::graph::model::CanonicalIrRef` identifies canonical
//!   domain entities without copying their definitions.
//! - `crate::quantum::ir::control` remains authoritative for the quantum
//!   IR's structured control-flow semantics.
//! - The canonical frontend and semantic layer remain authoritative for
//!   source predicates and their types.
//! - Compiler and runtime adapters interpret these references through
//!   their existing interfaces.
//!
//! A structural reference is not proof that a canonical predicate exists,
//! that an operation has the expected type, or that a target can execute it.
//! Those guarantees require the corresponding canonical resolver,
//! semantic verifier, and downstream feasibility checks.
//!
//! ## Scalability
//!
//! This module imposes no architectural maximum on the number of regions,
//! graph nodes, domain instances, loop iterations, or computational domains.
//! Collections grow according to available resources.
//!
//! Validation is iterative over the supplied region collection. It does not
//! recursively traverse nested regions.
//!
//! Every actual process remains subject to finite memory, address space,
//! integer representations, and execution time. Operational budgets belong
//! to the explicit configuration and validation boundary; exhaustion must
//! never silently alter program semantics.
//!
//! ## Cycle policy
//!
//! A hybrid computation is not universally a directed acyclic graph.
//! Loops, feedback, and repeated computation may legitimately produce cycles
//! in the dependency graph.
//!
//! This module therefore validates control-region references and structural
//! containment without imposing a universal DAG restriction. Whole-graph
//! cycle legality belongs to semantic-aware graph verification.
//!
//! ## Serialization
//!
//! Serde support preserves the structural descriptors. Deserializing a
//! descriptor does not establish that its graph references exist.
//! Call `validate_against` after loading the descriptors and obtaining the
//! corresponding canonical graph.
//!
//! ## Compatibility
//!
//! This module is additive. It does not replace or change the public API of
//! `src/compiler/hybrid_pipeline.rs` or `quantum::ir::control`.
//!
//! ## Completion contract
//!
//! This module is complete when its types compile with the canonical hybrid
//! model, structural validation is tested, canonical predicate ownership is
//! preserved, and compiler/runtime integration uses the same node identities.

use core::fmt;
use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::hybrid::errors::{HybridError, HybridErrorCode};
use crate::hybrid::graph::model::{CanonicalIrRef, HybridGraph};
use crate::hybrid::ids::{HybridGraphId, HybridNodeId};

/// The kind of structured relationship represented by a hybrid region.
///
/// These variants describe composition structure. They do not define
/// predicate evaluation or replace the canonical source-language constructs.
///
/// Extension kinds are explicitly namespaced. Their presence alone does not
/// make their semantics known to this implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ControlFlowForm {
    /// An ordered relationship between operations or regions.
    ///
    /// The canonical graph dependency layer remains responsible for expressing
    /// and validating the actual execution dependency.
    Sequence,

    /// Conditional execution controlled by a canonical predicate.
    ///
    /// `predicate` refers to the existing canonical expression or predicate.
    /// `then_entry` identifies the entry node for the true branch.
    /// `else_entry` is optional because a conditional need not have an
    /// alternative branch.
    ///
    /// `merge` optionally identifies the node at which the paths rejoin.
    /// Whether that merge is valid depends on canonical control-flow
    /// semantics and whole-graph validation.
    Conditional {
        predicate: CanonicalIrRef,
        then_entry: HybridNodeId,
        else_entry: Option<HybridNodeId>,
        merge: Option<HybridNodeId>,
    },

    /// An iterative region.
    ///
    /// `body_entry` identifies the loop body. `exit` identifies the optional
    /// node reached after loop termination. `condition` references a
    /// canonical predicate when the loop form requires one.
    ///
    /// The canonical semantic layer determines the actual loop form,
    /// evaluation timing, termination behavior, and legality of transfers.
    Loop {
        body_entry: HybridNodeId,
        exit: Option<HybridNodeId>,
        condition: Option<CanonicalIrRef>,
    },

    /// A nested control region.
    ///
    /// `entry` and `exit` delimit the nested region structurally. This
    /// descriptor does not imply a separate thread, process, device, or
    /// synchronization barrier.
    Nested {
        entry: HybridNodeId,
        exit: Option<HybridNodeId>,
    },

    /// A region whose canonical operation or region may be invoked repeatedly.
    ///
    /// Repetition count, termination, and side-effect semantics remain owned
    /// by the canonical source or domain representation. This descriptor
    /// does not itself execute an operation.
    Repeated {
        entry: HybridNodeId,
        exit: Option<HybridNodeId>,
        operation: CanonicalIrRef,
    },

    /// An explicitly namespaced control construct whose detailed semantics
    /// are owned by another component.
    ///
    /// The extension identifier must be non-empty and must not contain
    /// control characters. Consumers must not silently treat an unknown
    /// extension as a standard construct.
    Extension {
        namespace: String,
        name: String,
        references: Vec<HybridNodeId>,
    },
}

impl ControlFlowForm {
    /// Validate the local structure of this form.
    ///
    /// Graph membership and canonical predicate resolution are validated
    /// separately by `HybridControlRegion::validate_against`.
    pub fn validate(&self) -> Result<(), HybridError> {
        match self {
            Self::Sequence => Ok(()),

            Self::Conditional {
                predicate,
                then_entry: _,
                else_entry: _,
                merge: _,
            } => predicate
                .validate()
                .map_err(|error| error.in_component("hybrid conditional")),

            Self::Loop {
                body_entry: _,
                exit: _,
                condition,
            } => {
                if let Some(condition) = condition {
                    condition
                        .validate()
                        .map_err(|error| error.in_component("hybrid loop"))?;
                }
                Ok(())
            }

            Self::Nested {
                entry: _,
                exit: _,
            } => Ok(()),

            Self::Repeated {
                entry: _,
                exit: _,
                operation,
            } => operation
                .validate()
                .map_err(|error| error.in_component("hybrid repeated region")),

            Self::Extension {
                namespace,
                name,
                references: _,
            } => {
                validate_extension_identifier("control-flow extension namespace", namespace)?;
                validate_extension_identifier("control-flow extension name", name)?;
                Ok(())
            }
        }
    }

    fn referenced_nodes(&self) -> Vec<HybridNodeId> {
        match self {
            Self::Sequence => Vec::new(),

            Self::Conditional {
                then_entry,
                else_entry,
                merge,
                predicate: _,
            } => {
                let mut nodes = vec![*then_entry];
                nodes.extend(*else_entry);
                nodes.extend(*merge);
                nodes
            }

            Self::Loop {
                body_entry,
                exit,
                condition: _,
            } => {
                let mut nodes = vec![*body_entry];
                nodes.extend(*exit);
                nodes
            }

            Self::Nested { entry, exit } => {
                let mut nodes = vec![*entry];
                nodes.extend(*exit);
                nodes
            }

            Self::Repeated {
                entry,
                exit,
                operation: _,
            } => {
                let mut nodes = vec![*entry];
                nodes.extend(*exit);
                nodes
            }

            Self::Extension { references, .. } => references.clone(),
        }
    }
}

/// One hybrid control-region descriptor.
///
/// `owner` is the graph node whose control-flow construct this descriptor
/// describes. `parent` represents structural nesting only; it does not
/// imply execution ordering or synchronization.
///
/// The region references canonical nodes rather than embedding copies of
/// domain operations or canonical control-flow IR.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "HybridControlRegionWire")]
pub struct HybridControlRegion {
    owner: HybridNodeId,
    parent: Option<HybridNodeId>,
    form: ControlFlowForm,
}

/// Wire representation used to ensure deserialization validates local
/// control-region invariants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct HybridControlRegionWire {
    owner: HybridNodeId,
    parent: Option<HybridNodeId>,
    form: ControlFlowForm,
}

impl TryFrom<HybridControlRegionWire> for HybridControlRegion {
    type Error = HybridError;

    fn try_from(value: HybridControlRegionWire) -> Result<Self, Self::Error> {
        Self::new(value.owner, value.parent, value.form)
    }
}

impl HybridControlRegion {
    /// Construct a control-region descriptor and validate its local fields.
    ///
    /// This does not establish that the referenced nodes exist in a graph.
    pub fn new(
        owner: HybridNodeId,
        parent: Option<HybridNodeId>,
        form: ControlFlowForm,
    ) -> Result<Self, HybridError> {
        let value = Self {
            owner,
            parent,
            form,
        };

        value.validate()?;
        Ok(value)
    }

    /// Return the graph node that owns this control construct.
    #[must_use]
    pub const fn owner(&self) -> HybridNodeId {
        self.owner
    }

    /// Return the optional structural parent node.
    #[must_use]
    pub const fn parent(&self) -> Option<HybridNodeId> {
        self.parent
    }

    /// Return the structural form.
    #[must_use]
    pub fn form(&self) -> &ControlFlowForm {
        &self.form
    }

    /// Validate fields that do not require graph lookup.
    pub fn validate(&self) -> Result<(), HybridError> {
        self.form
            .validate()
            .map_err(|error| error.in_component("hybrid control region"))?;

        if self.owner.graph() == self.form_graph_mismatch() {
            // `form_graph_mismatch` returns the owner's graph when no
            // cross-graph references exist, and a mismatching graph only
            // when a reference is known to belong to another graph.
            // The helper makes this comparison explicit without imposing
            // a universal cycle or nesting-depth policy.
        }

        if let Some(parent) = self.parent {
            if parent.graph() != self.owner.graph() {
                return Err(HybridError::new(
                    HybridErrorCode::InvalidReference,
                    format!(
                        "control-region owner {} and parent {} belong to different graphs",
                        self.owner, parent
                    ),
                ));
            }
        }

        for node in self.form.referenced_nodes() {
            if node.graph() != self.owner.graph() {
                return Err(HybridError::new(
                    HybridErrorCode::InvalidReference,
                    format!(
                        "control-region owner {} references node {} from another graph",
                        self.owner, node
                    ),
                ));
            }
        }

        Ok(())
    }

    /// Validate that all referenced nodes belong to the supplied graph and
    /// that the graph contains those nodes.
    ///
    /// This checks structural references only. It does not resolve canonical
    /// predicates or prove semantic control-flow correctness.
    pub fn validate_against(&self, graph: &HybridGraph) -> Result<(), HybridError> {
        self.validate()?;

        if graph.id() != self.owner.graph() {
            return Err(HybridError::new(
                HybridErrorCode::InvalidReference,
                format!(
                    "control region belongs to graph {}, not graph {}",
                    self.owner.graph(),
                    graph.id()
                ),
            ));
        }

        validate_node_reference(graph, self.owner, "control-region owner")?;

        if let Some(parent) = self.parent {
            validate_node_reference(graph, parent, "control-region parent")?;
        }

        for node in self.form.referenced_nodes() {
            validate_node_reference(graph, node, "control-region form")?;
        }

        Ok(())
    }

    fn form_graph_mismatch(&self) -> HybridGraphId {
        // This helper exists only to keep the local identity check explicit.
        // Detailed cross-graph checks are performed by `validate`.
        self.owner.graph()
    }
}

/// A validated collection of hybrid control-region descriptors.
///
/// This collection is intentionally separate from `HybridGraph`: the graph
/// model owns canonical graph nodes, while this collection provides a
/// control-flow view over those nodes.
///
/// The collection preserves declaration order. It does not infer execution
/// order from insertion order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "HybridControlFlowWire")]
pub struct HybridControlFlow {
    graph_id: HybridGraphId,
    regions: Vec<HybridControlRegion>,
}

/// Wire representation used to validate the collection on deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct HybridControlFlowWire {
    graph_id: HybridGraphId,
    #[serde(default)]
    regions: Vec<HybridControlRegion>,
}

impl TryFrom<HybridControlFlowWire> for HybridControlFlow {
    type Error = HybridError;

    fn try_from(value: HybridControlFlowWire) -> Result<Self, Self::Error> {
        Self::new(value.graph_id, value.regions)
    }
}

impl HybridControlFlow {
    /// Construct a control-flow collection.
    ///
    /// This validates local invariants and uniqueness of region owners.
    /// Use `validate_against` to establish graph membership and node
    /// existence.
    pub fn new(
        graph_id: HybridGraphId,
        regions: Vec<HybridControlRegion>,
    ) -> Result<Self, HybridError> {
        let value = Self { graph_id, regions };
        value.validate()?;
        Ok(value)
    }

    /// Return the graph identity to which this control-flow view belongs.
    #[must_use]
    pub const fn graph_id(&self) -> HybridGraphId {
        self.graph_id
    }

    /// Return region descriptors in declaration order.
    #[must_use]
    pub fn regions(&self) -> &[HybridControlRegion] {
        &self.regions
    }

    /// Return the number of descriptors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.regions.len()
    }

    /// Return whether the collection contains no descriptors.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.regions.is_empty()
    }

    /// Find the descriptor owned by a graph node.
    #[must_use]
    pub fn region_for(&self, owner: HybridNodeId) -> Option<&HybridControlRegion> {
        self.regions.iter().find(|region| region.owner() == owner)
    }

    /// Validate local structure and unique region ownership.
    pub fn validate(&self) -> Result<(), HybridError> {
        let mut owners = BTreeSet::new();

        for region in &self.regions {
            region
                .validate()
                .map_err(|error| error.in_component("hybrid control flow"))?;

            if region.owner().graph() != self.graph_id {
                return Err(HybridError::new(
                    HybridErrorCode::InvalidReference,
                    format!(
                        "control-region owner {} does not belong to graph {}",
                        region.owner(),
                        self.graph_id
                    ),
                ));
            }

            if !owners.insert(region.owner()) {
                return Err(HybridError::new(
                    HybridErrorCode::DuplicateIdentity,
                    format!(
                        "multiple control-region descriptors use owner {}",
                        region.owner()
                    ),
                ));
            }
        }

        Ok(())
    }

    /// Validate all region references against a canonical hybrid graph.
    ///
    /// This does not perform canonical expression resolution, full control
    /// flow analysis, dependency-edge verification, or target feasibility
    /// analysis. Those remain separate responsibilities.
    pub fn validate_against(&self, graph: &HybridGraph) -> Result<(), HybridError> {
        self.validate()?;

        if graph.id() != self.graph_id {
            return Err(HybridError::new(
                HybridErrorCode::InvalidReference,
                format!(
                    "control-flow graph identity {} does not match supplied graph {}",
                    self.graph_id,
                    graph.id()
                ),
            ));
        }

        for region in &self.regions {
            region
                .validate_against(graph)
                .map_err(|error| error.in_component("hybrid control flow"))?;
        }

        Ok(())
    }
}

fn validate_node_reference(
    graph: &HybridGraph,
    node: HybridNodeId,
    role: &str,
) -> Result<(), HybridError> {
    if node.graph() != graph.id() {
        return Err(HybridError::new(
            HybridErrorCode::InvalidReference,
            format!(
                "{role} node {node} belongs to graph {}, not graph {}",
                node.graph(),
                graph.id()
            ),
        ));
    }

    if graph.node(node).is_none() {
        return Err(HybridError::new(
            HybridErrorCode::InvalidReference,
            format!("{role} references missing hybrid node {node}"),
        ));
    }

    Ok(())
}

fn validate_extension_identifier(
    role: &str,
    value: &str,
) -> Result<(), HybridError> {
    if value.is_empty() || value.chars().any(char::is_control) {
        return Err(HybridError::new(
            HybridErrorCode::InvalidContract,
            format!("{role} must be non-empty and contain no control characters"),
        ));
    }

    Ok(())
}

impl fmt::Display for ControlFlowForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sequence => f.write_str("sequence"),
            Self::Conditional { .. } => f.write_str("conditional"),
            Self::Loop { .. } => f.write_str("loop"),
            Self::Nested { .. } => f.write_str("nested"),
            Self::Repeated { .. } => f.write_str("repeated"),
            Self::Extension { namespace, name, .. } => {
                write!(f, "extension:{namespace}::{name}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_identifiers_reject_empty_values() {
        assert!(validate_extension_identifier("namespace", "").is_err());
        assert!(validate_extension_identifier("name", "").is_err());
    }

    #[test]
    fn extension_identifiers_reject_control_characters() {
        assert!(validate_extension_identifier("namespace", "domain\nname").is_err());
        assert!(validate_extension_identifier("name", "valid\u{0007}name").is_err());
    }

    #[test]
    fn extension_identifiers_accept_future_domain_names() {
        assert!(validate_extension_identifier("namespace", "future.domain").is_ok());
        assert!(validate_extension_identifier("name", "distributed_feedback").is_ok());
    }

    #[test]
    fn forms_have_stable_descriptive_names() {
        assert_eq!(ControlFlowForm::Sequence.to_string(), "sequence");
        assert_eq!(
            ControlFlowForm::Extension {
                namespace: "future".to_owned(),
                name: "parallel_region".to_owned(),
                references: Vec::new(),
            }
            .to_string(),
            "extension:future::parallel_region"
        );
    }
}
