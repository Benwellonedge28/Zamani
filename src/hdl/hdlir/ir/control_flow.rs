
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Control Flow
//!
//! Path: `src/hdl/hdlir/ir/control_flow.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//! Status: Core control-flow representation; freeze after repository
//! integration and conformance tests pass.
//!
//! ## Purpose
//!
//! Represent explicit control-flow successor relationships between blocks
//! in a hardware-domain IR region. This module records which blocks may
//! follow a terminator and which values are supplied to successor block
//! arguments.
//!
//! ## Authority
//!
//! This module follows the repository's HDL specification, applicable HDL
//! grammars, `src/zuir/ZUIR-SPEC.md`, and the HDLIR contracts in:
//!
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/schema/operation.rs`
//! - `src/hdl/hdlir/schema/interfaces.rs`
//! - `src/hdl/hdlir/ir/blocks.rs`
//! - `src/hdl/hdlir/ir/regions.rs`
//! - `src/hdl/hdlir/ir/operations.rs`
//!
//! The grammar defines syntax. The frontend owns source-level meaning.
//! ZUIR owns universal computational representation. HDLIR owns this
//! hardware-domain representation of control-flow relationships.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Explicit edges between HDLIR blocks.
//! - Ordered value arguments supplied along each edge.
//! - Associations between a block's declared terminator operation and
//!   its control-flow successor edges.
//! - A region-scoped collection of control-flow terminator records.
//! - Structural checks that can be performed using the supplied region
//!   and block definitions.
//!
//! This module does not own:
//!
//! - Block or operation definitions.
//! - The operation schema registry or operation semantics.
//! - Source parsing, source-level name resolution, or semantic analysis.
//! - General value-definition and use-def information.
//! - Value type compatibility or symbolic-width evaluation.
//! - Behavioral-process evaluation, clocked state transitions, or memory
//!   collision semantics.
//! - Physical scheduling, placement, routing, synthesis, or realization.
//!
//! ## Semantic boundary
//!
//! A control-flow edge describes a possible transfer from one block to
//! another in the control-flow model established by the owning dialect.
//! It does not, by itself, determine when that transfer occurs or which
//! condition selects it. The registered operation definition and dialect
//! determine those semantics.
//!
//! Clocked state transitions MUST NOT be reduced to ordinary control-flow
//! edges if doing so loses clock, reset, state-retention, or update
//! semantics. Those facts remain represented by the appropriate
//! sequential, process, state, and clock contracts.
//!
//! A terminator record references the operation that terminates its source
//! block. The operation schema and verifier must establish whether that
//! operation is a legal terminator in the enclosing region. This module
//! does not infer terminator meaning from an operation's name.
//!
//! Multiple edges to the same destination are permitted. For example,
//! different branch outcomes may reach the same block while supplying
//! different values. Edge order and edge argument order are preserved.
//!
//! ## Verification boundary
//!
//! `validate_against_region` checks:
//!
//! - The control-flow graph belongs to the supplied region.
//! - Referenced source and destination blocks belong to that region.
//! - Referenced blocks exist in the supplied block map.
//! - Each recorded terminator operation matches the source block's
//!   declared terminator.
//! - Each edge supplies the same number of values as the destination
//!   block declares arguments.
//!
//! These checks do NOT establish:
//!
//! - That each supplied value exists or is available at the edge.
//! - That values dominate their uses or are valid under the dialect's
//!   dataflow rules.
//! - That argument and supplied-value types are compatible.
//! - That a branch condition or terminator operation is well-typed.
//! - That every operation requiring terminator semantics has a record.
//! - That the control-flow graph is reachable, acyclic, terminating, or
//!   executable.
//! - That a transformation preserves behavior.
//!
//! Those obligations belong to whole-graph verification, schema-aware
//! operation verification, type verification, and applicable analyses.
//!
//! ## Scalability and determinism
//!
//! No fixed number of blocks, edges, branch outcomes, or edge arguments is
//! imposed. Collections grow subject to available memory, representable
//! collection sizes, supported semantics, and explicit operational budgets
//! enforced by the caller.
//!
//! This module does not promise to process literally infinite input.
//! It imposes no arbitrary universal hardware-capacity ceiling.
//!
//! Edges and edge arguments retain their input order. Terminator records
//! are indexed by typed block identity in a `BTreeMap` for deterministic
//! iteration. IDs are never treated as collection indices.
//!
//! ## Integration
//!
//! - `blocks.rs` owns the source block's operation sequence and terminator.
//! - `regions.rs` owns ordered region block references.
//! - `operations.rs` owns operation instances and their schema identities.
//! - `schema/` owns operation definitions and terminator interfaces.
//! - `verify/structure.rs` must integrate these checks with complete graph
//!   ownership, identity uniqueness, and reference validation.
//! - `verify/types.rs` must verify edge argument value types against the
//!   destination block arguments.
//! - `verify/` must use registered dialect contracts to establish which
//!   terminators and successor patterns are legal.
//! - Builders must use these checked APIs without treating local structural
//!   validation as proof that the whole design is valid.
//!
//! The graph stores references rather than duplicating blocks or operations.
//! This avoids competing ownership and allows the graph to be checked
//! against the canonical enclosing IR.
//!
//! ## Freeze criteria
//!
//! - Edges preserve target and argument order.
//! - Multiple edges to one target are supported.
//! - Local and region-scoped structural invariants are checked.
//! - Missing references and incompatible edge arities produce structured
//!   errors rather than panics or silent repairs.
//! - Type, schema, process, and sequential semantics remain with their
//!   authoritative owners.
//! - Tests cover valid and invalid edges, empty successor lists, duplicate
//!   records, region mismatch, and large typed IDs.
//! - Repository formatting, compilation, tests, and Clippy pass.

use std::collections::{BTreeMap, BTreeSet};

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{
    HdlirBlockId, HdlirOperationId, HdlirRegionId, HdlirValueId,
};

use super::blocks::HdlirBlock;
use super::regions::HdlirRegion;

/// One possible control-flow edge from a terminator to a successor block.
///
/// The values in `arguments` are supplied to the destination block's
/// arguments in declaration order. Their existence, availability, and type
/// compatibility are checked by the enclosing graph verifier.
///
/// The edge deliberately does not store a source block: its source is
/// established by the owning [`HdlirControlFlowTerminator`]. This prevents
/// inconsistent source identities within one terminator record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlirControlFlowEdge {
    target: HdlirBlockId,
    arguments: Vec<HdlirValueId>,
}

impl HdlirControlFlowEdge {
    /// Construct an edge to a destination block.
    ///
    /// This constructor preserves argument order. It does not establish
    /// that the destination exists or that the supplied arguments are valid.
    #[must_use]
    pub fn new(
        target: HdlirBlockId,
        arguments: Vec<HdlirValueId>,
    ) -> Self {
        Self { target, arguments }
    }

    /// Return the destination block identity.
    #[must_use]
    pub const fn target(&self) -> HdlirBlockId {
        self.target
    }

    /// Return the values supplied to the destination block in order.
    #[must_use]
    pub fn arguments(&self) -> &[HdlirValueId] {
        &self.arguments
    }

    /// Return the number of values supplied along this edge.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Return whether this edge supplies no block arguments.
    #[must_use]
    pub fn has_no_arguments(&self) -> bool {
        self.arguments.is_empty()
    }
}

/// The control-flow information associated with one block terminator.
///
/// `operation` identifies the operation already declared as the source
/// block's terminator. `successors` contains its ordered possible outgoing
/// edges. A terminator with no successors is representable; its legality
/// depends on the registered operation and region contract.
///
/// The operation's schema, rather than this record, defines whether it is
/// a return, unconditional branch, conditional branch, switch, yield, or
/// another extension-defined terminator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlirControlFlowTerminator {
    source: HdlirBlockId,
    operation: HdlirOperationId,
    successors: Vec<HdlirControlFlowEdge>,
}

impl HdlirControlFlowTerminator {
    /// Construct control-flow information for a block terminator.
    ///
    /// The successor list is preserved exactly as supplied. This method
    /// does not prove that `operation` is a registered terminator or that
    /// the source block declares it as its terminator.
    #[must_use]
    pub fn new(
        source: HdlirBlockId,
        operation: HdlirOperationId,
        successors: Vec<HdlirControlFlowEdge>,
    ) -> Self {
        Self {
            source,
            operation,
            successors,
        }
    }

    /// Return the source block identity.
    #[must_use]
    pub const fn source(&self) -> HdlirBlockId {
        self.source
    }

    /// Return the operation identity of the declared terminator.
    #[must_use]
    pub const fn operation(&self) -> HdlirOperationId {
        self.operation
    }

    /// Return successor edges in their declared order.
    #[must_use]
    pub fn successors(&self) -> &[HdlirControlFlowEdge] {
        &self.successors
    }

    /// Return the number of outgoing edges.
    #[must_use]
    pub fn successor_count(&self) -> usize {
        self.successors.len()
    }

    /// Return whether the terminator has no outgoing edges.
    #[must_use]
    pub fn has_no_successors(&self) -> bool {
        self.successors.is_empty()
    }
}

/// A region-scoped collection of explicit control-flow terminators.
///
/// The graph stores terminator records by source block identity. It does not
/// own the region, blocks, or operation instances. The caller supplies those
/// canonical definitions when validating the graph.
///
/// A graph may be empty while it is being constructed or when the region's
/// dialect does not use this control-flow representation. Completeness
/// requirements are determined by the registered region and operation
/// contracts, not guessed by this container.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlirControlFlowGraph {
    region: HdlirRegionId,
    terminators: BTreeMap<HdlirBlockId, HdlirControlFlowTerminator>,
}

impl HdlirControlFlowGraph {
    /// Construct a region-scoped control-flow graph.
    ///
    /// Terminators are indexed by source block identity. Duplicate source
    /// blocks and duplicate terminator operation identities are rejected.
    /// Successor order, duplicate destinations, and edge argument order are
    /// preserved.
    ///
    /// # Errors
    ///
    /// Returns an error if multiple records claim the same source block or
    /// the same terminator operation.
    pub fn new(
        region: HdlirRegionId,
        terminators: impl IntoIterator<Item = HdlirControlFlowTerminator>,
    ) -> HdlirResult<Self> {
        let mut by_source = BTreeMap::new();
        let mut operation_ids = BTreeSet::new();

        for terminator in terminators {
            let source = terminator.source();
            let operation = terminator.operation();

            if by_source.contains_key(&source) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR control-flow terminator source block",
                    identity: source.to_string(),
                    scope: Some(region.to_string()),
                });
            }

            if !operation_ids.insert(operation) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR control-flow terminator operation",
                    identity: operation.to_string(),
                    scope: Some(region.to_string()),
                });
            }

            by_source.insert(source, terminator);
        }

        Ok(Self {
            region,
            terminators: by_source,
        })
    }

    /// Return the region identity associated with this graph.
    #[must_use]
    pub const fn region(&self) -> HdlirRegionId {
        self.region
    }

    /// Return the number of terminator records.
    #[must_use]
    pub fn terminator_count(&self) -> usize {
        self.terminators.len()
    }

    /// Return whether this graph contains no terminator records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.terminators.is_empty()
    }

    /// Find control-flow information for a source block.
    #[must_use]
    pub fn terminator(
        &self,
        source: HdlirBlockId,
    ) -> Option<&HdlirControlFlowTerminator> {
        self.terminators.get(&source)
    }

    /// Iterate over terminator records in deterministic source-ID order.
    ///
    /// This is identity order, not source declaration or execution order.
    /// The order of successor edges inside each record remains unchanged.
    pub fn terminators(
        &self,
    ) -> impl Iterator<Item = &HdlirControlFlowTerminator> {
        self.terminators.values()
    }

    /// Validate this graph against its canonical region and block definitions.
    ///
    /// This method checks region membership, source terminator association,
    /// destination existence, and edge argument arity. It does not verify
    /// value definitions, dominance, types, operation schemas, reachability,
    /// or semantic completeness.
    ///
    /// `blocks` is keyed by each canonical block's identity. The method
    /// checks that every block referenced by the region exists in that map,
    /// but does not require the map to contain only blocks from this region.
    ///
    /// # Errors
    ///
    /// Returns a structured error if the region identity differs, a region
    /// block is missing, a source or destination lies outside the region,
    /// a source block has no matching declared terminator, or an edge's
    /// argument count differs from the target block's argument count.
    pub fn validate_against_region(
        &self,
        region: &HdlirRegion,
        blocks: &BTreeMap<HdlirBlockId, HdlirBlock>,
    ) -> HdlirResult<()> {
        if self.region != region.id() {
            return Err(HdlirError::invalid_invariant(
                "control_flow.region_matches",
                format!(
                    "control-flow graph belongs to region {}, not region {}",
                    self.region,
                    region.id()
                ),
            ));
        }

        region.validate_structure()?;

        // Check region membership and resolve every region block reference.
        for block_id in region.blocks() {
            if !blocks.contains_key(block_id) {
                return Err(HdlirError::MissingEntity {
                    entity_kind: "HDLIR region block",
                    identity: block_id.to_string(),
                    scope: Some(region.id().to_string()),
                });
            }
        }

        for terminator in self.terminators.values() {
            let source_id = terminator.source();

            if !region.contains_block(source_id) {
                return Err(HdlirError::invalid_invariant(
                    "control_flow.source_in_region",
                    format!(
                        "source block {source_id} is not a member of region {}",
                        region.id()
                    ),
                ));
            }

            let source_block = blocks.get(&source_id).ok_or_else(|| {
                HdlirError::MissingEntity {
                    entity_kind: "HDLIR control-flow source block",
                    identity: source_id.to_string(),
                    scope: Some(region.id().to_string()),
                }
            })?;

            source_block.validate_structure()?;

            if source_block.terminator() != Some(terminator.operation()) {
                return Err(HdlirError::invalid_invariant(
                    "control_flow.matches_block_terminator",
                    format!(
                        "control-flow record for block {source_id} references \
                         operation {}, but the block declares terminator {:?}",
                        terminator.operation(),
                        source_block.terminator()
                    ),
                ));
            }

            for edge in terminator.successors() {
                let target_id = edge.target();

                if !region.contains_block(target_id) {
                    return Err(HdlirError::invalid_invariant(
                        "control_flow.target_in_region",
                        format!(
                            "successor block {target_id} is not a member of \
                             region {}",
                            region.id()
                        ),
                    ));
                }

                let target_block = blocks.get(&target_id).ok_or_else(|| {
                    HdlirError::MissingEntity {
                        entity_kind: "HDLIR control-flow target block",
                        identity: target_id.to_string(),
                        scope: Some(region.id().to_string()),
                    }
                })?;

                target_block.validate_structure()?;

                if edge.argument_count() != target_block.argument_count() {
                    return Err(HdlirError::invalid_invariant(
                        "control_flow.edge_argument_arity",
                        format!(
                            "edge from block {source_id} to block {target_id} \
                             supplies {} value(s), but the target declares \
                             {} block argument(s)",
                            edge.argument_count(),
                            target_block.argument_count()
                        ),
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{
        HdlirControlFlowEdge, HdlirControlFlowGraph,
        HdlirControlFlowTerminator,
    };
    use crate::hdl::hdlir::errors::HdlirError;
    use crate::hdl::hdlir::ids::{
        HdlirBlockId, HdlirOperationId, HdlirRegionId, HdlirTypeId,
        HdlirValueId,
    };
    use crate::hdl::hdlir::ir::blocks::{
        HdlirBlock, HdlirBlockArgument,
    };
    use crate::hdl::hdlir::ir::regions::{
        HdlirRegion, HdlirRegionKind,
    };

    fn block_id(raw: u128) -> HdlirBlockId {
        HdlirBlockId::new(raw).expect("non-zero block ID")
    }

    fn operation_id(raw: u128) -> HdlirOperationId {
        HdlirOperationId::new(raw).expect("non-zero operation ID")
    }

    fn region_id(raw: u128) -> HdlirRegionId {
        HdlirRegionId::new(raw).expect("non-zero region ID")
    }

    fn value_id(raw: u128) -> HdlirValueId {
        HdlirValueId::new(raw).expect("non-zero value ID")
    }

    fn type_id(raw: u128) -> HdlirTypeId {
        HdlirTypeId::new(raw).expect("non-zero type ID")
    }

    fn argument(value: u128, ty: u128) -> HdlirBlockArgument {
        HdlirBlockArgument::new(value_id(value), type_id(ty))
    }

    fn block(
        id: u128,
        arguments: Vec<HdlirBlockArgument>,
        operations: Vec<u128>,
        terminator: Option<u128>,
    ) -> HdlirBlock {
        HdlirBlock::with_optional_terminator(
            block_id(id),
            arguments,
            operations.into_iter().map(operation_id).collect(),
            terminator.map(operation_id),
        )
        .expect("valid test block")
    }

    fn region(id: u128, block_ids: &[u128]) -> HdlirRegion {
        HdlirRegion::new(
            region_id(id),
            HdlirRegionKind::ControlFlow,
            block_ids.iter().copied().map(block_id).collect(),
        )
        .expect("valid test region")
    }

    fn block_map(
        blocks: impl IntoIterator<Item = HdlirBlock>,
    ) -> BTreeMap<HdlirBlockId, HdlirBlock> {
        blocks.into_iter().map(|block| (block.id(), block)).collect()
    }

    fn edge(target: u128, arguments: &[u128]) -> HdlirControlFlowEdge {
        HdlirControlFlowEdge::new(
            block_id(target),
            arguments.iter().copied().map(value_id).collect(),
        )
    }

    fn terminator(
        source: u128,
        operation: u128,
        successors: Vec<HdlirControlFlowEdge>,
    ) -> HdlirControlFlowTerminator {
        HdlirControlFlowTerminator::new(
            block_id(source),
            operation_id(operation),
            successors,
        )
    }

    #[test]
    fn preserves_successor_and_argument_order() {
        let term = terminator(
            1,
            10,
            vec![edge(3, &[8, 2]), edge(2, &[5])],
        );

        assert_eq!(term.source(), block_id(1));
        assert_eq!(term.operation(), operation_id(10));
        assert_eq!(term.successor_count(), 2);
        assert_eq!(term.successors()[0].target(), block_id(3));
        assert_eq!(
            term.successors()[0].arguments(),
            &[value_id(8), value_id(2)]
        );
        assert_eq!(term.successors()[1].target(), block_id(2));
    }

    #[test]
    fn permits_multiple_edges_to_the_same_target() {
        let term = terminator(
            1,
            10,
            vec![edge(2, &[3]), edge(2, &[4])],
        );

        assert_eq!(term.successor_count(), 2);
        assert_eq!(term.successors()[0].target(), block_id(2));
        assert_eq!(term.successors()[1].target(), block_id(2));
        assert_eq!(term.successors()[0].arguments(), &[value_id(3)]);
        assert_eq!(term.successors()[1].arguments(), &[value_id(4)]);
    }

    #[test]
    fn permits_a_terminator_without_successors() {
        let term = terminator(1, 10, Vec::new());

        assert!(term.has_no_successors());
        assert_eq!(term.successor_count(), 0);
    }

    #[test]
    fn constructs_an_empty_graph() {
        let graph =
            HdlirControlFlowGraph::new(region_id(1), Vec::new())
                .expect("empty graph is representable");

        assert!(graph.is_empty());
        assert_eq!(graph.terminator_count(), 0);
        assert_eq!(graph.region(), region_id(1));
    }

    #[test]
    fn rejects_duplicate_source_blocks() {
        let result = HdlirControlFlowGraph::new(
            region_id(1),
            vec![
                terminator(2, 10, Vec::new()),
                terminator(2, 11, Vec::new()),
            ],
        );

        assert!(matches!(
            result,
            Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR control-flow terminator source block",
                ..
            })
        ));
    }

    #[test]
    fn rejects_duplicate_terminator_operation_ids() {
        let result = HdlirControlFlowGraph::new(
            region_id(1),
            vec![
                terminator(2, 10, Vec::new()),
                terminator(3, 10, Vec::new()),
            ],
        );

        assert!(matches!(
            result,
            Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR control-flow terminator operation",
                ..
            })
        ));
    }

    #[test]
    fn validates_a_well_formed_region_graph() {
        let region = region(1, &[1, 2]);

        let blocks = block_map([
            block(1, Vec::new(), vec![10], Some(10)),
            block(2, vec![argument(20, 1)], vec![], None),
        ]);

        let graph = HdlirControlFlowGraph::new(
            region_id(1),
            vec![terminator(1, 10, vec![edge(2, &[30])])],
        )
        .expect("valid graph");

        assert!(
            graph.validate_against_region(&region, &blocks).is_ok()
        );
    }

    #[test]
    fn rejects_a_region_identity_mismatch() {
        let region = region(2, &[1]);
        let blocks = block_map([block(1, Vec::new(), vec![], None)]);
        let graph =
            HdlirControlFlowGraph::new(region_id(1), Vec::new())
                .expect("valid graph");

        assert!(matches!(
            graph.validate_against_region(&region, &blocks),
            Err(HdlirError::InvalidInvariant {
                invariant: "control_flow.region_matches",
                ..
            })
        ));
    }

    #[test]
    fn rejects_a_missing_region_block_definition() {
        let region = region(1, &[1, 2]);
        let blocks = block_map([block(1, Vec::new(), vec![], None)]);
        let graph =
            HdlirControlFlowGraph::new(region_id(1), Vec::new())
                .expect("valid graph");

        assert!(matches!(
            graph.validate_against_region(&region, &blocks),
            Err(HdlirError::MissingEntity {
                entity_kind: "HDLIR region block",
                ..
            })
        ));
    }

    #[test]
    fn rejects_a_source_outside_the_region() {
        let region = region(1, &[1]);
        let blocks = block_map([
            block(1, Vec::new(), vec![], None),
            block(2, Vec::new(), vec![10], Some(10)),
        ]);
        let graph = HdlirControlFlowGraph::new(
            region_id(1),
            vec![terminator(2, 10, Vec::new())],
        )
        .expect("graph records are locally unique");

        assert!(matches!(
            graph.validate_against_region(&region, &blocks),
            Err(HdlirError::InvalidInvariant {
                invariant: "control_flow.source_in_region",
                ..
            })
        ));
    }

    #[test]
    fn rejects_a_target_outside_the_region() {
        let region = region(1, &[1]);
        let blocks = block_map([
            block(1, Vec::new(), vec![10], Some(10)),
            block(2, Vec::new(), vec![], None),
        ]);
        let graph = HdlirControlFlowGraph::new(
            region_id(1),
            vec![terminator(1, 10, vec![edge(2, &[])])],
        )
        .expect("graph records are locally unique");

        assert!(matches!(
            graph.validate_against_region(&region, &blocks),
            Err(HdlirError::InvalidInvariant {
                invariant: "control_flow.target_in_region",
                ..
            })
        ));
    }

    #[test]
    fn rejects_a_terminator_that_disagrees_with_the_source_block() {
        let region = region(1, &[1]);
        let blocks =
            block_map([block(1, Vec::new(), vec![10], Some(10))]);
        let graph = HdlirControlFlowGraph::new(
            region_id(1),
            vec![terminator(1, 11, Vec::new())],
        )
        .expect("graph records are locally unique");

        assert!(matches!(
            graph.validate_against_region(&region, &blocks),
            Err(HdlirError::InvalidInvariant {
                invariant: "control_flow.matches_block_terminator",
                ..
            })
        ));
    }

    #[test]
    fn rejects_edge_argument_arity_mismatch() {
        let region = region(1, &[1, 2]);
        let blocks = block_map([
            block(1, Vec::new(), vec![10], Some(10)),
            block(2, vec![argument(20, 1)], vec![], None),
        ]);
        let graph = HdlirControlFlowGraph::new(
            region_id(1),
            vec![terminator(1, 10, vec![edge(2, &[])])],
        )
        .expect("graph records are locally unique");

        assert!(matches!(
            graph.validate_against_region(&region, &blocks),
            Err(HdlirError::InvalidInvariant {
                invariant: "control_flow.edge_argument_arity",
                ..
            })
        ));
    }

    #[test]
    fn accepts_large_typed_ids_without_truncation() {
        let largest = u128::MAX;
        let region = region(largest, &[largest, largest - 1]);

        let blocks = block_map([
            block(largest, Vec::new(), vec![largest], Some(largest)),
            block(
                largest - 1,
                vec![argument(largest - 2, largest)],
                Vec::new(),
                None,
            ),
        ]);

        let graph = HdlirControlFlowGraph::new(
            region_id(largest),
            vec![terminator(
                largest,
                largest,
                vec![edge(largest - 1, &[largest - 3])],
            )],
        )
        .expect("maximum non-zero IDs are representable");

        assert!(
            graph.validate_against_region(&region, &blocks).is_ok()
        );
    }
}
