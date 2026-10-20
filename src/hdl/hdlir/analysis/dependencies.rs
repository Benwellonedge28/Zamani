
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Dependency Analysis
//!
//! Path: `src/hdl/hdlir/analysis/dependencies.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Construct a deterministic, read-only dependency graph from existing
//! HDLIR graph components. The analysis records which entities depend on
//! which prerequisites, including operation operands, operation results,
//! values, modules, regions, blocks, ports, connectivity, processes,
//! clocks, and abstract memories.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and applicable delegate grammars
//! - `grammar/specification/semantic-model.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//! - `src/hdl/hdlir/config.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/ir/`
//! - `src/hdl/hdlir/types/`
//! - `src/hdl/hdlir/values/`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns syntax. The canonical frontend owns source semantics.
//! ZUIR owns the universal computational representation. HDLIR owns the
//! hardware-domain graph and this graph's derived dependency analysis.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Dependency-node and dependency-edge descriptions.
//! - Deterministic graph construction from borrowed HDLIR components.
//! - Direct prerequisite and dependent indexes.
//! - Explicit work accounting and budget enforcement.
//!
//! This module does not own:
//!
//! - The authoritative HDLIR graph or any of its entities.
//! - Value definitions, operation-use indexes, or module registries.
//! - Operation semantics or extension-defined attribute interpretation.
//! - Whole-graph verification, dominance, type checking, or equivalence proofs.
//! - Physical resource allocation, synthesis, scheduling, placement, or routing.
//! - A replacement for ZUIR or the canonical semantic model.
//!
//! ## Edge direction
//!
//! An edge's `dependent` is the entity that relies on another entity.
//! Its `prerequisite` is the referenced entity.
//!
//! For example, an operation consuming a value creates:
//!
//! `operation -> operand value`
//!
//! An operation result creates:
//!
//! `result value -> defining operation`
//!
//! These relationships describe dependencies, not physical data movement.
//! Structural containment edges are separately identified by their kind.
//!
//! ## Scalability
//!
//! There are no fixed limits on hardware entities, operations, ports,
//! operands, regions, blocks, memories, or dependency edges.
//!
//! The analysis is bounded by available resources, representable identities,
//! collection limits, and explicitly configured work budgets. No finite
//! implementation can process literally infinite input.
//!
//! All cumulative work counters use checked arithmetic. Exhaustion returns
//! an explicit error; incomplete analysis is never returned as complete.
//!
//! ## Determinism
//!
//! Nodes, edges, and adjacency indexes use ordered collections. Operand
//! positions are preserved in edge metadata, including repeated operands.
//! No pointer address or unordered-map iteration order contributes to output.
//!
//! ## Integration
//!
//! `ir/values.rs` remains authoritative for value definitions and uses.
//! This module derives a broader graph and does not mutate that index.
//!
//! Callers must provide all graph components relevant to their analysis.
//! An omitted component is not evidence that the corresponding entity does
//! not exist. Reference existence and complete graph ownership are verified
//! by the appropriate graph-verification subsystem.
//!
//! ## Failure behavior
//!
//! Duplicate entity identities, malformed local structures, arithmetic
//! overflow, and exhausted analysis budgets are reported explicitly.
//! Unknown extension semantics are not guessed.
//!
//! ## Completion criteria
//!
//! - Construction is deterministic for equivalent input.
//! - Dependency edges preserve their documented direction.
//! - Repeated operands retain distinct operand positions.
//! - Recursive attribute references are indexed without interpretation.
//! - Budget exhaustion never produces a successful complete result.
//! - The module compiles and its tests pass in the actual repository tree.

use std::collections::{BTreeMap, BTreeSet};

use crate::hdl::hdlir::config::{BudgetKind, HdlirConfig};
use crate::hdl::hdlir::errors::{HdlirBudgetKind, HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{
    HdlirBlockId, HdlirClockDomainId, HdlirInstanceId, HdlirMemoryId,
    HdlirModuleId, HdlirNetId, HdlirOperationId, HdlirPortId,
    HdlirProcessId, HdlirRegionId, HdlirSignalId, HdlirTypeId,
    HdlirValueId,
};
use crate::hdl::hdlir::ir::blocks::HdlirBlock;
use crate::hdl::hdlir::ir::clocks::HdlirClockDomain;
use crate::hdl::hdlir::ir::connectivity::{
    HdlirConnection, HdlirEndpoint,
};
use crate::hdl::hdlir::ir::memories::{
    HdlirMemory, HdlirMemoryEndpoint,
};
use crate::hdl::hdlir::ir::module::HdlirModule;
use crate::hdl::hdlir::ir::operations::{
    HdlirOperation, OperationAttributeValue,
};
use crate::hdl::hdlir::ir::ports::HdlirPort;
use crate::hdl::hdlir::ir::processes::HdlirProcess;
use crate::hdl::hdlir::ir::regions::HdlirRegion;
use crate::hdl::hdlir::ir::values::{
    ValueDefinition, ValueTable,
};

/// An entity participating in HDLIR dependency analysis.
///
/// IDs retain their domain-specific types. An identity in one namespace
/// cannot accidentally be used as an identity in another namespace.
///
/// A node may represent a referenced entity whose definition was not
/// supplied to the analysis. Node existence therefore does not establish
/// that a reference resolves in the authoritative design.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum DependencyNode {
    /// A logical module.
    Module(HdlirModuleId),

    /// A logical module-boundary port.
    Port(HdlirPortId),

    /// An HDLIR operation.
    Operation(HdlirOperationId),

    /// An HDLIR value.
    Value(HdlirValueId),

    /// An HDLIR type referenced by an entity.
    Type(HdlirTypeId),

    /// A region containing blocks.
    Region(HdlirRegionId),

    /// An ordered block.
    Block(HdlirBlockId),

    /// A logical signal.
    Signal(HdlirSignalId),

    /// A logical connection.
    Connection(HdlirNetId),

    /// A module instance referenced by a connection.
    ///
    /// This identity does not imply that an instance-definition subsystem
    /// has been supplied or that the instance reference resolves.
    Instance(HdlirInstanceId),

    /// An abstract memory.
    Memory(HdlirMemoryId),

    /// A logical clock domain.
    ClockDomain(HdlirClockDomainId),

    /// A behavioral process.
    Process(HdlirProcessId),
}

/// The semantic relationship represented by a dependency edge.
///
/// Edge kinds intentionally distinguish data dependencies from structural
/// references. This prevents clients from treating every edge as a
/// combinational dataflow dependency.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum DependencyKind {
    /// An operation requires an operand value.
    OperationOperand,

    /// A value is defined by an operation.
    OperationResult,

    /// An operation references a nested region.
    OperationRegion,

    /// An operation attribute contains a value reference.
    OperationAttributeValue,

    /// A module references a parameter's declared type.
    ModuleParameterType,

    /// A module parameter uses a default value.
    ModuleParameterDefault,

    /// A module references a port declaration.
    ModulePort,

    /// A module references a body region.
    ModuleRegion,

    /// A port uses a default value.
    PortDefaultValue,

    /// A region references a block.
    RegionBlock,

    /// A block references an operation.
    BlockOperation,

    /// A block owns a block-argument value.
    BlockArgument,

    /// A value uses a type identity.
    ValueType,

    /// A value is defined by a block.
    BlockValueDefinition,

    /// A value is defined by a port.
    PortValueDefinition,

    /// A value is associated with a logical signal.
    SignalValueDefinition,

    /// A logical connection references an endpoint.
    ConnectionEndpoint,

    /// A memory references its containing module.
    MemoryModule,

    /// A memory binding references a logical endpoint.
    MemoryEndpoint,

    /// A memory clause references a value through an attribute.
    MemoryClauseValue,

    /// A process references its body region.
    ProcessRegion,

    /// A process references a logical clock domain.
    ProcessClockDomain,

    /// A clock domain references its parent clock domain.
    ClockParent,
}

/// One direct dependency between two typed HDLIR entities.
///
/// `dependent` is the entity whose requirements include `prerequisite`.
/// `position` records an optional zero-based local sequence position,
/// such as an operation operand index. It is not a persistent entity ID.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DependencyEdge {
    dependent: DependencyNode,
    prerequisite: DependencyNode,
    kind: DependencyKind,
    position: Option<usize>,
}

impl DependencyEdge {
    /// Return the entity that depends on the prerequisite.
    #[must_use]
    pub const fn dependent(self) -> DependencyNode {
        self.dependent
    }

    /// Return the entity referenced as a prerequisite.
    #[must_use]
    pub const fn prerequisite(self) -> DependencyNode {
        self.prerequisite
    }

    /// Return the relationship represented by this edge.
    #[must_use]
    pub const fn kind(self) -> DependencyKind {
        self.kind
    }

    /// Return the optional local sequence position.
    #[must_use]
    pub const fn position(self) -> Option<usize> {
        self.position
    }
}

/// Borrowed HDLIR components supplied to dependency analysis.
///
/// The slices describe the components visible to this invocation. The
/// caller must not interpret an omitted component as an empty component
/// in the authoritative design.
///
/// Use empty slices for genuinely empty categories. The value table is
/// borrowed separately because it is the canonical graph-level value table.
pub struct DependencyAnalysisInput<'a> {
    /// Logical modules included in the analysis.
    pub modules: &'a [HdlirModule],

    /// Logical ports included in the analysis.
    pub ports: &'a [HdlirPort],

    /// Operations included in the analysis.
    pub operations: &'a [HdlirOperation],

    /// The canonical graph-level value table.
    pub values: &'a ValueTable,

    /// Regions included in the analysis.
    pub regions: &'a [HdlirRegion],

    /// Blocks included in the analysis.
    pub blocks: &'a [HdlirBlock],

    /// Logical connections included in the analysis.
    pub connections: &'a [HdlirConnection],

    /// Abstract memories included in the analysis.
    pub memories: &'a [HdlirMemory],

    /// Behavioral processes included in the analysis.
    pub processes: &'a [HdlirProcess],

    /// Clock domains included in the analysis.
    pub clock_domains: &'a [HdlirClockDomain],
}

/// Statistics about one completed dependency analysis.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DependencyAnalysisStats {
    entities_observed: u64,
    edges_observed: u64,
    work_units: u64,
}

impl DependencyAnalysisStats {
    /// Return the number of entity records observed.
    #[must_use]
    pub const fn entities_observed(self) -> u64 {
        self.entities_observed
    }

    /// Return the number of dependency edges emitted.
    #[must_use]
    pub const fn edges_observed(self) -> u64 {
        self.edges_observed
    }

    /// Return the cumulative work units charged.
    #[must_use]
    pub const fn work_units(self) -> u64 {
        self.work_units
    }
}

/// A deterministic, read-only dependency graph derived from HDLIR.
///
/// The graph does not own or mutate the source IR. It stores a canonical
/// set of nodes, ordered direct edges, and indexed adjacency lists.
///
/// This analysis is not a verifier. Missing referenced definitions may
/// still appear as dependency nodes and must be checked by graph-wide
/// verification before downstream stages trust the graph.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DependencyAnalysis {
    nodes: BTreeSet<DependencyNode>,
    edges: Vec<DependencyEdge>,
    prerequisites: BTreeMap<DependencyNode, Vec<DependencyEdge>>,
    dependents: BTreeMap<DependencyNode, Vec<DependencyEdge>>,
    stats: DependencyAnalysisStats,
}

impl DependencyAnalysis {
    /// Build a dependency graph from the supplied HDLIR components.
    ///
    /// The default configuration has no explicit work budget. Callers
    /// processing untrusted or potentially very large designs should supply
    /// appropriate limits through `HdlirConfig`.
    ///
    /// This method does not perform target discovery, operation evaluation,
    /// semantic verification, or physical-resource estimation.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - A supplied entity violates its local structural contract.
    /// - Duplicate IDs are found within a supplied entity category.
    /// - Checked work accounting overflows.
    /// - A configured entity or work budget is exhausted.
    pub fn build(
        input: DependencyAnalysisInput<'_>,
        config: &HdlirConfig,
    ) -> HdlirResult<Self> {
        let mut builder = DependencyBuilder::new(config);

        builder.add_modules(input.modules)?;
        builder.add_ports(input.ports)?;
        builder.add_operations(input.operations)?;
        builder.add_values(input.values)?;
        builder.add_regions(input.regions)?;
        builder.add_blocks(input.blocks)?;
        builder.add_connections(input.connections)?;
        builder.add_memories(input.memories)?;
        builder.add_processes(input.processes)?;
        builder.add_clock_domains(input.clock_domains)?;

        Ok(builder.finish())
    }

    /// Return all observed nodes in deterministic identity order.
    pub fn nodes(
        &self,
    ) -> impl ExactSizeIterator<Item = &DependencyNode> {
        self.nodes.iter()
    }

    /// Return all edges in deterministic order.
    ///
    /// Operand positions and edge kinds are part of edge ordering, so
    /// distinct uses of the same value are not collapsed.
    pub fn edges(&self) -> &[DependencyEdge] {
        &self.edges
    }

    /// Return the direct prerequisites of an entity.
    ///
    /// An entity with no prerequisites returns an empty slice.
    #[must_use]
    pub fn prerequisites_of(
        &self,
        node: DependencyNode,
    ) -> &[DependencyEdge] {
        self.prerequisites
            .get(&node)
            .map_or(&[], Vec::as_slice)
    }

    /// Return the direct dependents of an entity.
    ///
    /// An entity with no dependents returns an empty slice.
    #[must_use]
    pub fn dependents_of(
        &self,
        node: DependencyNode,
    ) -> &[DependencyEdge] {
        self.dependents
            .get(&node)
            .map_or(&[], Vec::as_slice)
    }

    /// Return whether the analysis contains the specified node.
    #[must_use]
    pub fn contains_node(&self, node: DependencyNode) -> bool {
        self.nodes.contains(&node)
    }

    /// Return the number of distinct nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Return the number of direct edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Return statistics from the completed analysis.
    #[must_use]
    pub const fn stats(&self) -> DependencyAnalysisStats {
        self.stats
    }
}

/// Mutable construction state private to the analysis implementation.
struct DependencyBuilder<'a> {
    config: &'a HdlirConfig,
    graph: DependencyAnalysis,
    seen_modules: BTreeSet<HdlirModuleId>,
    seen_ports: BTreeSet<HdlirPortId>,
    seen_operations: BTreeSet<HdlirOperationId>,
    seen_regions: BTreeSet<HdlirRegionId>,
    seen_blocks: BTreeSet<HdlirBlockId>,
    seen_connections: BTreeSet<HdlirNetId>,
    seen_memories: BTreeSet<HdlirMemoryId>,
    seen_processes: BTreeSet<HdlirProcessId>,
    seen_clocks: BTreeSet<HdlirClockDomainId>,
}

impl<'a> DependencyBuilder<'a> {
    fn new(config: &'a HdlirConfig) -> Self {
        Self {
            config,
            graph: DependencyAnalysis::default(),
            seen_modules: BTreeSet::new(),
            seen_ports: BTreeSet::new(),
            seen_operations: BTreeSet::new(),
            seen_regions: BTreeSet::new(),
            seen_blocks: BTreeSet::new(),
            seen_connections: BTreeSet::new(),
            seen_memories: BTreeSet::new(),
            seen_processes: BTreeSet::new(),
            seen_clocks: BTreeSet::new(),
        }
    }

    fn charge_work(&mut self, amount: u64) -> HdlirResult<()> {
        let observed = self
            .graph
            .stats
            .work_units
            .checked_add(amount)
            .ok_or(HdlirError::ArithmeticOverflow {
                operation: "HDLIR dependency analysis work accounting",
                context: Some("cumulative work units".to_owned()),
            })?;

        let budget = BudgetKind::VerificationWork;
        if !self.config.budget(&budget).permits(observed) {
            return Err(HdlirError::BudgetExceeded {
                budget: HdlirBudgetKind::Analysis,
                limit: self.config.budget(&budget).limit().map(u128::from),
                observed: Some(u128::from(observed)),
                operation: "HDLIR dependency analysis".to_owned(),
            });
        }

        self.graph.stats.work_units = observed;
        Ok(())
    }

    fn charge_entity(&mut self) -> HdlirResult<()> {
        let observed = self
            .graph
            .stats
            .entities_observed
            .checked_add(1)
            .ok_or(HdlirError::ArithmeticOverflow {
                operation: "HDLIR dependency entity accounting",
                context: Some("entity count".to_owned()),
            })?;

        let budget = BudgetKind::IrEntities;
        if !self.config.budget(&budget).permits(observed) {
            return Err(HdlirError::BudgetExceeded {
                budget: HdlirBudgetKind::Analysis,
                limit: self.config.budget(&budget).limit().map(u128::from),
                observed: Some(u128::from(observed)),
                operation: "HDLIR dependency entity collection".to_owned(),
            });
        }

        self.graph.stats.entities_observed = observed;
        self.charge_work(1)
    }

    fn add_node(&mut self, node: DependencyNode) -> HdlirResult<()> {
        self.charge_work(1)?;
        self.graph.nodes.insert(node);
        Ok(())
    }

    fn add_edge(
        &mut self,
        dependent: DependencyNode,
        prerequisite: DependencyNode,
        kind: DependencyKind,
        position: Option<usize>,
    ) -> HdlirResult<()> {
        self.charge_work(1)?;

        self.graph.nodes.insert(dependent);
        self.graph.nodes.insert(prerequisite);

        self.graph.edges.push(DependencyEdge {
            dependent,
            prerequisite,
            kind,
            position,
        });

        self.graph.stats.edges_observed = self
            .graph
            .stats
            .edges_observed
            .checked_add(1)
            .ok_or(HdlirError::ArithmeticOverflow {
                operation: "HDLIR dependency edge accounting",
                context: Some("edge count".to_owned()),
            })?;

        Ok(())
    }

    fn add_modules(
        &mut self,
        modules: &[HdlirModule],
    ) -> HdlirResult<()> {
        for module in modules {
            self.charge_entity()?;

            if !self.seen_modules.insert(module.id()) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR dependency-analysis module",
                    identity: module.id().to_string(),
                    scope: None,
                });
            }

            let node = DependencyNode::Module(module.id());
            self.add_node(node)?;

            for parameter in module.parameters() {
                self.add_edge(
                    node,
                    DependencyNode::Type(parameter.type_id()),
                    DependencyKind::ModuleParameterType,
                    None,
                )?;

                if let Some(value) = parameter.default_value() {
                    self.add_edge(
                        node,
                        DependencyNode::Value(value),
                        DependencyKind::ModuleParameterDefault,
                        None,
                    )?;
                }
            }

            for (position, port) in module.ports().iter().enumerate() {
                self.add_edge(
                    node,
                    DependencyNode::Port(*port),
                    DependencyKind::ModulePort,
                    Some(position),
                )?;
            }

            for (position, region) in
                module.body_regions().iter().enumerate()
            {
                self.add_edge(
                    node,
                    DependencyNode::Region(*region),
                    DependencyKind::ModuleRegion,
                    Some(position),
                )?;
            }
        }

        Ok(())
    }

    fn add_ports(&mut self, ports: &[HdlirPort]) -> HdlirResult<()> {
        for port in ports {
            self.charge_entity()?;

            if !self.seen_ports.insert(port.id()) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR dependency-analysis port",
                    identity: port.id().to_string(),
                    scope: None,
                });
            }

            let node = DependencyNode::Port(port.id());
            self.add_node(node)?;

            if let Some(value) = port.default_value() {
                self.add_edge(
                    node,
                    DependencyNode::Value(value),
                    DependencyKind::PortDefaultValue,
                    None,
                )?;
            }
        }

        Ok(())
    }

    fn add_operations(
        &mut self,
        operations: &[HdlirOperation],
    ) -> HdlirResult<()> {
        for operation in operations {
            self.charge_entity()?;
            operation.validate_structure()?;

            if !self.seen_operations.insert(operation.id()) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR dependency-analysis operation",
                    identity: operation.id().to_string(),
                    scope: None,
                });
            }

            let node = DependencyNode::Operation(operation.id());
            self.add_node(node)?;

            for (position, operand) in
                operation.operands().iter().enumerate()
            {
                self.add_edge(
                    node,
                    DependencyNode::Value(*operand),
                    DependencyKind::OperationOperand,
                    Some(position),
                )?;
            }

            for (position, result) in
                operation.results().iter().enumerate()
            {
                self.add_edge(
                    DependencyNode::Value(result.id()),
                    node,
                    DependencyKind::OperationResult,
                    Some(position),
                )?;
                self.add_edge(
                    node,
                    DependencyNode::Type(result.type_id()),
                    DependencyKind::ValueType,
                    Some(position),
                )?;
            }

            for (position, region) in
                operation.regions().iter().enumerate()
            {
                self.add_edge(
                    node,
                    DependencyNode::Region(*region),
                    DependencyKind::OperationRegion,
                    Some(position),
                )?;
            }

            for (name, value) in operation.attributes() {
                self.add_attribute_value_edges(
                    node,
                    value,
                    DependencyKind::OperationAttributeValue,
                )?;

                // Attribute names are schema-defined. This analysis does
                // not infer meaning from a name or interpret opaque payloads.
                let _ = name;
            }
        }

        Ok(())
    }

    fn add_values(&mut self, values: &ValueTable) -> HdlirResult<()> {
        values.validate_structure()?;

        for (_, value) in values.iter() {
            self.charge_entity()?;

            let node = DependencyNode::Value(value.id());
            self.add_node(node)?;

            self.add_edge(
                node,
                DependencyNode::Type(value.type_id()),
                DependencyKind::ValueType,
                None,
            )?;

            match value.definition() {
                ValueDefinition::OperationResult { operation } => {
                    self.add_edge(
                        node,
                        DependencyNode::Operation(*operation),
                        DependencyKind::OperationResult,
                        None,
                    )?;
                }
                ValueDefinition::BlockArgument { block, position } => {
                    let position = usize::try_from(*position).map_err(|_| {
                        HdlirError::ConversionOutOfRange {
                            value: position.to_string(),
                            destination: "usize",
                        }
                    })?;

                    self.add_edge(
                        node,
                        DependencyNode::Block(*block),
                        DependencyKind::BlockValueDefinition,
                        Some(position),
                    )?;
                }
                ValueDefinition::ModulePort { port } => {
                    self.add_edge(
                        node,
                        DependencyNode::Port(*port),
                        DependencyKind::PortValueDefinition,
                        None,
                    )?;
                }
                ValueDefinition::Signal { signal } => {
                    self.add_edge(
                        node,
                        DependencyNode::Signal(*signal),
                        DependencyKind::SignalValueDefinition,
                        None,
                    )?;
                }
                ValueDefinition::External { .. }
                | ValueDefinition::Extension { .. } => {
                    // External and extension-defined origins remain opaque.
                    // Their meaning must be supplied by their owning contract.
                }
                _ => {
                    // ValueDefinition is non-exhaustive. Future variants
                    // must not be silently assigned guessed semantics.
                }
            }
        }

        Ok(())
    }

    fn add_regions(
        &mut self,
        regions: &[HdlirRegion],
    ) -> HdlirResult<()> {
        for region in regions {
            self.charge_entity()?;
            region.validate_structure()?;

            if !self.seen_regions.insert(region.id()) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR dependency-analysis region",
                    identity: region.id().to_string(),
                    scope: None,
                });
            }

            let node = DependencyNode::Region(region.id());
            self.add_node(node)?;

            for (position, block) in
                region.blocks().iter().enumerate()
            {
                self.add_edge(
                    node,
                    DependencyNode::Block(*block),
                    DependencyKind::RegionBlock,
                    Some(position),
                )?;
            }
        }

        Ok(())
    }

    fn add_blocks(
        &mut self,
        blocks: &[HdlirBlock],
    ) -> HdlirResult<()> {
        for block in blocks {
            self.charge_entity()?;
            block.validate_structure()?;

            if !self.seen_blocks.insert(block.id()) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR dependency-analysis block",
                    identity: block.id().to_string(),
                    scope: None,
                });
            }

            let node = DependencyNode::Block(block.id());
            self.add_node(node)?;

            for (position, operation) in
                block.operations().iter().enumerate()
            {
                self.add_edge(
                    node,
                    DependencyNode::Operation(*operation),
                    DependencyKind::BlockOperation,
                    Some(position),
                )?;
            }

            for (position, argument) in
                block.arguments().iter().enumerate()
            {
                self.add_edge(
                    DependencyNode::Value(argument.id()),
                    node,
                    DependencyKind::BlockArgument,
                    Some(position),
                )?;
                self.add_edge(
                    DependencyNode::Value(argument.id()),
                    DependencyNode::Type(argument.type_id()),
                    DependencyKind::ValueType,
                    Some(position),
                )?;
            }
        }

        Ok(())
    }

    fn add_connections(
        &mut self,
        connections: &[HdlirConnection],
    ) -> HdlirResult<()> {
        for connection in connections {
            self.charge_entity()?;
            connection.validate_structure()?;

            if !self.seen_connections.insert(connection.id()) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR dependency-analysis connection",
                    identity: connection.id().to_string(),
                    scope: None,
                });
            }

            let node = DependencyNode::Connection(connection.id());
            self.add_node(node)?;

            for (position, endpoint) in connection
                .drivers()
                .iter()
                .chain(connection.sinks())
                .enumerate()
            {
                self.add_edge(
                    node,
                    endpoint_node(*endpoint),
                    DependencyKind::ConnectionEndpoint,
                    Some(position),
                )?;
            }
        }

        Ok(())
    }

    fn add_memories(
        &mut self,
        memories: &[HdlirMemory],
    ) -> HdlirResult<()> {
        for memory in memories {
            self.charge_entity()?;
            memory.validate_structure()?;

            if !self.seen_memories.insert(memory.id()) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR dependency-analysis memory",
                    identity: memory.id().to_string(),
                    scope: None,
                });
            }

            let node = DependencyNode::Memory(memory.id());

            self.add_node(node)?;
            self.add_edge(
                node,
                DependencyNode::Module(memory.module_id()),
                DependencyKind::MemoryModule,
                None,
            )?;

            for (binding_position, binding) in
                memory.port_bindings().iter().enumerate()
            {
                for (endpoint_position, endpoint) in
                    binding.bindings().values().enumerate()
                {
                    self.add_edge(
                        node,
                        memory_endpoint_node(*endpoint),
                        DependencyKind::MemoryEndpoint,
                        Some(binding_position.saturating_add(endpoint_position)),
                    )?;
                }
            }

            for clause in memory.clauses() {
                self.add_attribute_value_edges(
                    node,
                    clause.value(),
                    DependencyKind::MemoryClauseValue,
                )?;
            }
        }

        Ok(())
    }

    fn add_processes(
        &mut self,
        processes: &[HdlirProcess],
    ) -> HdlirResult<()> {
        for process in processes {
            self.charge_entity()?;
            process.validate_structure()?;

            if !self.seen_processes.insert(process.id()) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR dependency-analysis process",
                    identity: process.id().to_string(),
                    scope: None,
                });
            }

            let node = DependencyNode::Process(process.id());
            self.add_node(node)?;

            self.add_edge(
                node,
                DependencyNode::Region(process.body_region()),
                DependencyKind::ProcessRegion,
                None,
            )?;

            if let Some(clock_domain) = process.clock_domain() {
                self.add_edge(
                    node,
                    DependencyNode::ClockDomain(clock_domain),
                    DependencyKind::ProcessClockDomain,
                    None,
                )?;
            }
        }

        Ok(())
    }

    fn add_clock_domains(
        &mut self,
        clock_domains: &[HdlirClockDomain],
    ) -> HdlirResult<()> {
        for domain in clock_domains {
            self.charge_entity()?;
            domain.validate_structure()?;

            if !self.seen_clocks.insert(domain.id()) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR dependency-analysis clock domain",
                    identity: domain.id().to_string(),
                    scope: None,
                });
            }

            let node = DependencyNode::ClockDomain(domain.id());
            self.add_node(node)?;

            if let Some(parent) = domain.parent() {
                self.add_edge(
                    node,
                    DependencyNode::ClockDomain(parent),
                    DependencyKind::ClockParent,
                    None,
                )?;
            }
        }

        Ok(())
    }

    fn add_attribute_value_edges(
        &mut self,
        owner: DependencyNode,
        attribute: &OperationAttributeValue,
        kind: DependencyKind,
    ) -> HdlirResult<()> {
        match attribute {
            OperationAttributeValue::Value(value) => {
                self.add_edge(
                    owner,
                    DependencyNode::Value(*value),
                    kind,
                    None,
                )?;
            }
            OperationAttributeValue::Array(values) => {
                for value in values {
                    self.add_attribute_value_edges(owner, value, kind)?;
                }
            }
            OperationAttributeValue::Dictionary(values) => {
                for value in values.values() {
                    self.add_attribute_value_edges(owner, value, kind)?;
                }
            }
            OperationAttributeValue::Boolean(_)
            | OperationAttributeValue::SignedInteger(_)
            | OperationAttributeValue::UnsignedInteger(_)
            | OperationAttributeValue::String(_)
            | OperationAttributeValue::QualifiedName(_)
            | OperationAttributeValue::Type(_)
            | OperationAttributeValue::ConstantPayload { .. }
            | OperationAttributeValue::SymbolicExpressionPayload { .. }
            | OperationAttributeValue::ExtensionPayload { .. } => {
                // Type references and opaque payloads are not interpreted
                // as value dependencies. Their schemas own their semantics.
            }
            _ => {
                // Keep future extension variants uninterpreted unless their
                // contracts explicitly define dependency extraction.
            }
        }

        Ok(())
    }

    fn finish(mut self) -> DependencyAnalysis {
        self.graph.edges.sort_unstable();

        for edge in &self.graph.edges {
            self.graph
                .prerequisites
                .entry(edge.dependent)
                .or_default()
                .push(*edge);

            self.graph
                .dependents
                .entry(edge.prerequisite)
                .or_default()
                .push(*edge);
        }

        self.graph
    }
}

fn endpoint_node(endpoint: HdlirEndpoint) -> DependencyNode {
    match endpoint {
        HdlirEndpoint::Signal(id) => DependencyNode::Signal(id),
        HdlirEndpoint::Value(id) => DependencyNode::Value(id),
        HdlirEndpoint::Port(id) => DependencyNode::Port(id),
        HdlirEndpoint::InstancePort { instance, port } => {
            // The instance-port relationship is retained as two explicit
            // prerequisites. A port ID alone does not identify an instance
            // interface binding.
            let _ = port;
            DependencyNode::Instance(instance)
        }
    }
}

fn memory_endpoint_node(
    endpoint: HdlirMemoryEndpoint,
) -> DependencyNode {
    match endpoint {
        HdlirMemoryEndpoint::Signal(id) => DependencyNode::Signal(id),
        HdlirMemoryEndpoint::Value(id) => DependencyNode::Value(id),
        HdlirMemoryEndpoint::ClockDomain(id) => {
            DependencyNode::ClockDomain(id)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_analysis_is_deterministic() {
        let values = ValueTable::new();
        let input = DependencyAnalysisInput {
            modules: &[],
            ports: &[],
            operations: &[],
            values: &values,
            regions: &[],
            blocks: &[],
            connections: &[],
            memories: &[],
            processes: &[],
            clock_domains: &[],
        };

        let first =
            DependencyAnalysis::build(input, &HdlirConfig::default())
                .unwrap();

        assert_eq!(first.node_count(), 0);
        assert_eq!(first.edge_count(), 0);
        assert_eq!(first.stats().edges_observed(), 0);
    }

    #[test]
    fn edge_direction_is_dependent_to_prerequisite() {
        let edge = DependencyEdge {
            dependent: DependencyNode::Operation(
                HdlirOperationId::new(1).unwrap(),
            ),
            prerequisite: DependencyNode::Value(
                HdlirValueId::new(2).unwrap(),
            ),
            kind: DependencyKind::OperationOperand,
            position: Some(0),
        };

        assert_eq!(
            edge.dependent(),
            DependencyNode::Operation(
                HdlirOperationId::new(1).unwrap()
            )
        );
        assert_eq!(
            edge.prerequisite(),
            DependencyNode::Value(HdlirValueId::new(2).unwrap())
        );
        assert_eq!(edge.position(), Some(0));
    }

    #[test]
    fn typed_ids_remain_distinct_dependency_nodes() {
        let value = DependencyNode::Value(
            HdlirValueId::new(7).unwrap(),
        );
        let operation = DependencyNode::Operation(
            HdlirOperationId::new(7).unwrap(),
        );

        assert_ne!(value, operation);
    }
}
