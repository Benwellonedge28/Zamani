
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Compilation Dependency Context.
//!
//! Path: src/compiler/context/dependency_context.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Owns the immutable dependency graph for one compilation operation.
//!
//! The graph records:
//! - Compilation-unit identities and descriptive metadata.
//! - Typed relationships between compilation units.
//! - Direct and transitive dependencies.
//! - Direct and transitive dependents.
//! - Compile-order dependencies.
//! - Non-ordering relationships.
//! - Deterministic dependency-first ordering.
//! - Dependency impact queries.
//! - Ordering-cycle validation.
//!
//! # Architectural ownership
//!
//! This module owns the compiler's compilation-unit dependency snapshot.
//!
//! It does not own:
//! - Source parsing, AST construction or source-map storage.
//! - Name resolution or semantic dependency discovery.
//! - Type, ownership, effect or capability analysis.
//! - Source fingerprints or incremental cache invalidation.
//! - Dirty-unit propagation or persistent build-cache storage.
//! - Parallel worker execution or runtime scheduling.
//! - Pipeline orchestration.
//! - IR definitions or domain-specific semantics.
//! - Hardware discovery, allocation or backend selection.
//! - Distributed execution or runtime communication.
//!
//! Dependency discovery belongs to the frontend and semantic authorities.
//! Fingerprints and invalidation belong to incremental compilation.
//! Parallel execution belongs to the parallel compiler subsystem.
//!
//! # Edge direction
//!
//! An edge is expressed as:
//!
//!     dependent -> dependency
//!
//! For example:
//!
//!     application -> library
//!
//! means that application requires library.
//!
//! Dependency-first compilation order is:
//!
//!     library, application
//!
//! # Ordering semantics
//!
//! Only relationships whose `requires_ordering()` method returns true
//! participate in dependency-first compilation ordering.
//!
//! Data flow, control flow, completion, feedback and synchronization
//! relationships are retained without automatically imposing compile order.
//!
//! # Scalability
//!
//! This module imposes no universal maximum on:
//! - Compilation units.
//! - Dependency relationships.
//! - Dependents.
//! - Modules or functions.
//! - Computing domains.
//! - Hardware types or devices.
//! - CPUs, GPUs, QPUs or FPGAs.
//! - Distributed nodes.
//! - Tensor dimensions or IR operations.
//!
//! Actual limits are determined by available memory, host representation
//! limits, execution time and explicitly configured operational budgets.
//!
//! No finite implementation promises infinite physical resources.
//!
//! # Determinism
//!
//! Ordered maps and sets provide stable iteration.
//! Independent compilation units are ordered by stable identifiers.
//! Transitive queries return identifiers in deterministic order.
//!
//! # Concurrency
//!
//! A built context is immutable and shared through `Arc`.
//! Concurrent readers require no additional graph lock.
//! Mutation is performed through an independently owned builder.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

/// Public API version for the dependency-context contract.
///
/// Independent of the Zamani language, grammar, semantic-model, IR,
/// backend and runtime versions.
pub const DEPENDENCY_CONTEXT_API_VERSION: u16 = 1;

// ============================================================================
// IDENTITIES
// ============================================================================

/// Stable logical identity of a compilation unit.
///
/// The identity is independent of filesystem paths, machines, operating
/// systems, backends, devices and compilation workers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompilationUnitId(Arc<str>);

impl CompilationUnitId {
    /// Creates a compilation-unit identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, DependencyGraphError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(DependencyGraphError::EmptyIdentifier);
        }

        if value.chars().any(char::is_control) {
            return Err(DependencyGraphError::InvalidIdentifier);
        }

        Ok(Self(value))
    }

    /// Returns the identity string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CompilationUnitId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Extensible compilation-unit classification.
///
/// This is an open identifier rather than a closed enum of computing
/// domains. New domains and provider-defined unit kinds do not require
/// changes to this module.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompilationUnitKind(Arc<str>);

impl CompilationUnitKind {
    /// Creates a compilation-unit kind.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, DependencyGraphError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(DependencyGraphError::EmptyUnitKind);
        }

        if value.chars().any(char::is_control) {
            return Err(DependencyGraphError::InvalidUnitKind);
        }

        Ok(Self(value))
    }

    /// Returns the stable kind identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CompilationUnitKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// COMPILATION UNIT
// ============================================================================

/// Immutable description of a compilation unit.
///
/// `logical_name` is descriptive metadata, not a filesystem path.
/// Source locations remain owned by the source-map subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationUnit {
    id: CompilationUnitId,
    kind: CompilationUnitKind,
    logical_name: Arc<str>,
}

impl CompilationUnit {
    /// Creates a compilation unit.
    pub fn new(
        id: CompilationUnitId,
        kind: CompilationUnitKind,
        logical_name: impl Into<Arc<str>>,
    ) -> Result<Self, DependencyGraphError> {
        let logical_name = logical_name.into();

        if logical_name.trim().is_empty() {
            return Err(DependencyGraphError::EmptyLogicalName);
        }

        if logical_name.chars().any(char::is_control) {
            return Err(DependencyGraphError::InvalidLogicalName);
        }

        Ok(Self {
            id,
            kind,
            logical_name,
        })
    }

    /// Returns the stable identity.
    #[must_use]
    pub fn id(&self) -> &CompilationUnitId {
        &self.id
    }

    /// Returns the extensible unit kind.
    #[must_use]
    pub fn kind(&self) -> &CompilationUnitKind {
        &self.kind
    }

    /// Returns the descriptive logical name.
    #[must_use]
    pub fn logical_name(&self) -> &str {
        &self.logical_name
    }
}

// ============================================================================
// RELATIONSHIPS
// ============================================================================

/// Typed relationship between compilation units.
///
/// Ordering relationships require the dependency to be processed before
/// the dependent unit.
///
/// Non-ordering relationships remain represented in the graph but do not
/// participate in dependency-first topological ordering.
///
/// Custom relationships permit extensions to introduce new relationship
/// identifiers without modifying this module.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum DependencyKind {
    /// Source unit imports another source unit.
    ModuleImport,

    /// A compilation unit requires another unit at compile time.
    CompileTime,

    /// Macro or compile-time expansion input.
    MacroExpansion,

    /// A type or interface definition must be available first.
    TypeDefinition,

    /// A unit is generated from another compilation unit.
    GeneratedFrom,

    /// A required build input.
    BuildInput,

    /// A required artifact input.
    ArtifactInput,

    /// An interface declaration must be available first.
    Interface,

    /// Runtime or semantic data-flow relationship.
    DataFlow,

    /// Control-flow relationship.
    ControlFlow,

    /// Completion relationship.
    Completion,

    /// Feedback relationship.
    Feedback,

    /// Synchronization relationship.
    Synchronization,

    /// Provider-defined relationship.
    Custom {
        /// Stable extension-defined relationship identifier.
        identifier: Arc<str>,

        /// Whether the relationship imposes compile ordering.
        requires_ordering: bool,
    },
}

impl DependencyKind {
    /// Returns whether this relationship imposes compile ordering.
    #[must_use]
    pub const fn requires_ordering(&self) -> bool {
        match self {
            Self::ModuleImport
            | Self::CompileTime
            | Self::MacroExpansion
            | Self::TypeDefinition
            | Self::GeneratedFrom
            | Self::BuildInput
            | Self::ArtifactInput
            | Self::Interface => true,

            Self::DataFlow
            | Self::ControlFlow
            | Self::Completion
            | Self::Feedback
            | Self::Synchronization => false,

            Self::Custom {
                requires_ordering,
                ..
            } => *requires_ordering,
        }
    }

    /// Returns the stable relationship identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::ModuleImport => "module_import",
            Self::CompileTime => "compile_time",
            Self::MacroExpansion => "macro_expansion",
            Self::TypeDefinition => "type_definition",
            Self::GeneratedFrom => "generated_from",
            Self::BuildInput => "build_input",
            Self::ArtifactInput => "artifact_input",
            Self::Interface => "interface",
            Self::DataFlow => "data_flow",
            Self::ControlFlow => "control_flow",
            Self::Completion => "completion",
            Self::Feedback => "feedback",
            Self::Synchronization => "synchronization",
            Self::Custom { identifier, .. } => identifier,
        }
    }
}

/// Directed dependency relationship.
///
/// Direction is `dependent -> dependency`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyEdge {
    dependent: CompilationUnitId,
    dependency: CompilationUnitId,
    kind: DependencyKind,
}

impl DependencyEdge {
    /// Creates a typed dependency edge.
    pub fn new(
        dependent: CompilationUnitId,
        dependency: CompilationUnitId,
        kind: DependencyKind,
    ) -> Result<Self, DependencyGraphError> {
        if let DependencyKind::Custom { identifier, .. } = &kind {
            if identifier.trim().is_empty() {
                return Err(DependencyGraphError::EmptyRelationshipIdentifier);
            }

            if identifier.chars().any(char::is_control) {
                return Err(DependencyGraphError::InvalidRelationshipIdentifier);
            }
        }

        if dependent == dependency && kind.requires_ordering() {
            return Err(DependencyGraphError::SelfOrderingDependency {
                unit: dependent,
            });
        }

        Ok(Self {
            dependent,
            dependency,
            kind,
        })
    }

    /// Returns the dependent unit.
    #[must_use]
    pub fn dependent(&self) -> &CompilationUnitId {
        &self.dependent
    }

    /// Returns the required unit.
    #[must_use]
    pub fn dependency(&self) -> &CompilationUnitId {
        &self.dependency
    }

    /// Returns the relationship kind.
    #[must_use]
    pub fn kind(&self) -> &DependencyKind {
        &self.kind
    }

    /// Returns whether this edge imposes compile ordering.
    #[must_use]
    pub fn requires_ordering(&self) -> bool {
        self.kind.requires_ordering()
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced while constructing or validating a dependency graph.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DependencyGraphError {
    EmptyIdentifier,
    InvalidIdentifier,
    EmptyUnitKind,
    InvalidUnitKind,
    EmptyLogicalName,
    InvalidLogicalName,
    EmptyRelationshipIdentifier,
    InvalidRelationshipIdentifier,

    DuplicateUnit {
        unit: CompilationUnitId,
    },

    UnknownDependent {
        unit: CompilationUnitId,
    },

    UnknownDependency {
        dependent: CompilationUnitId,
        dependency: CompilationUnitId,
    },

    SelfOrderingDependency {
        unit: CompilationUnitId,
    },

    /// Units blocked by an ordering cycle.
    ///
    /// This includes units in the cycle and units that depend on the cycle.
    /// It is the set of units that cannot be emitted by topological ordering,
    /// not necessarily the minimal strongly connected component.
    OrderingCycle {
        blocked_units: Vec<CompilationUnitId>,
    },
}

impl fmt::Display for DependencyGraphError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => {
                formatter.write_str("compilation-unit identifier is empty")
            }
            Self::InvalidIdentifier => {
                formatter.write_str(
                    "compilation-unit identifier contains control characters",
                )
            }
            Self::EmptyUnitKind => {
                formatter.write_str("compilation-unit kind is empty")
            }
            Self::InvalidUnitKind => {
                formatter.write_str(
                    "compilation-unit kind contains control characters",
                )
            }
            Self::EmptyLogicalName => {
                formatter.write_str("compilation-unit logical name is empty")
            }
            Self::InvalidLogicalName => {
                formatter.write_str(
                    "compilation-unit logical name contains control characters",
                )
            }
            Self::EmptyRelationshipIdentifier => {
                formatter.write_str("custom relationship identifier is empty")
            }
            Self::InvalidRelationshipIdentifier => {
                formatter.write_str(
                    "custom relationship identifier contains control characters",
                )
            }
            Self::DuplicateUnit { unit } => {
                write!(formatter, "duplicate compilation unit: {unit}")
            }
            Self::UnknownDependent { unit } => {
                write!(formatter, "unknown dependent compilation unit: {unit}")
            }
            Self::UnknownDependency {
                dependent,
                dependency,
            } => {
                write!(
                    formatter,
                    "compilation unit '{dependent}' references unknown dependency '{dependency}'"
                )
            }
            Self::SelfOrderingDependency { unit } => {
                write!(
                    formatter,
                    "compilation unit '{unit}' has an invalid self-ordering dependency"
                )
            }
            Self::OrderingCycle { blocked_units } => {
                formatter.write_str(
                    "compilation dependency ordering contains a cycle; blocked units: ",
                )?;

                for (index, unit) in blocked_units.iter().enumerate() {
                    if index != 0 {
                        formatter.write_str(", ")?;
                    }

                    write!(formatter, "{unit}")?;
                }

                Ok(())
            }
        }
    }
}

impl Error for DependencyGraphError {}

// ============================================================================
// IMMUTABLE GRAPH STORAGE
// ============================================================================

#[derive(Debug)]
struct DependencyGraphData {
    units: BTreeMap<CompilationUnitId, CompilationUnit>,
    edges: BTreeSet<DependencyEdge>,

    /// Direct dependencies, including all relationship kinds:
    /// dependent -> dependencies.
    dependencies: BTreeMap<CompilationUnitId, BTreeSet<CompilationUnitId>>,

    /// Direct dependents, including all relationship kinds:
    /// dependency -> dependents.
    dependents: BTreeMap<CompilationUnitId, BTreeSet<CompilationUnitId>>,

    /// Direct compile-order dependencies:
    /// dependent -> dependencies.
    ordering_dependencies:
        BTreeMap<CompilationUnitId, BTreeSet<CompilationUnitId>>,

    /// Reverse compile-order relationships:
    /// dependency -> dependents.
    ordering_dependents:
        BTreeMap<CompilationUnitId, BTreeSet<CompilationUnitId>>,

    /// Relationships indexed by (dependent, dependency).
    ///
    /// This avoids scanning the entire edge set for a pair query.
    relationships: BTreeMap<
        (CompilationUnitId, CompilationUnitId),
        BTreeSet<DependencyKind>,
    >,
}

impl DependencyGraphData {
    fn empty() -> Self {
        Self {
            units: BTreeMap::new(),
            edges: BTreeSet::new(),
            dependencies: BTreeMap::new(),
            dependents: BTreeMap::new(),
            ordering_dependencies: BTreeMap::new(),
            ordering_dependents: BTreeMap::new(),
            relationships: BTreeMap::new(),
        }
    }
}

// ============================================================================
// DEPENDENCY CONTEXT
// ============================================================================

/// Immutable dependency snapshot for a compilation operation.
///
/// Cloning shares graph data through `Arc`. The graph can therefore be
/// consumed by concurrent compiler stages without reader locks.
#[derive(Debug, Clone)]
pub struct DependencyContext {
    data: Arc<DependencyGraphData>,
}

impl Default for DependencyContext {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyContext {
    /// Creates an empty dependency context.
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: Arc::new(DependencyGraphData::empty()),
        }
    }

    /// Returns the number of registered compilation units.
    #[must_use]
    pub fn unit_count(&self) -> usize {
        self.data.units.len()
    }

    /// Returns the number of unique typed relationships.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.data.edges.len()
    }

    /// Returns whether the graph contains no units.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.units.is_empty()
    }

    /// Returns a unit by stable identity.
    #[must_use]
    pub fn unit(&self, id: &CompilationUnitId) -> Option<&CompilationUnit> {
        self.data.units.get(id)
    }

    /// Returns units in deterministic identifier order.
    pub fn units(&self) -> impl ExactSizeIterator<Item = &CompilationUnit> {
        self.data.units.values()
    }

    /// Returns unit identities in deterministic order.
    pub fn unit_ids(&self) -> impl ExactSizeIterator<Item = &CompilationUnitId> {
        self.data.units.keys()
    }

    /// Returns all typed edges in deterministic order.
    pub fn edges(&self) -> impl ExactSizeIterator<Item = &DependencyEdge> {
        self.data.edges.iter()
    }

    /// Returns whether a unit is registered.
    #[must_use]
    pub fn contains_unit(&self, id: &CompilationUnitId) -> bool {
        self.data.units.contains_key(id)
    }

    /// Returns direct dependencies of a unit.
    ///
    /// All relationship kinds are included. `None` means the requested
    /// compilation unit is not present in this graph.
    #[must_use]
    pub fn dependencies_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        self.data
            .dependencies
            .get(id)
            .map(|items| items.iter().cloned().collect())
    }

    /// Returns direct dependents of a unit.
    ///
    /// All relationship kinds are included. `None` means the requested
    /// compilation unit is not present in this graph.
    #[must_use]
    pub fn dependents_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        self.data
            .dependents
            .get(id)
            .map(|items| items.iter().cloned().collect())
    }

    /// Returns direct compile-order dependencies.
    #[must_use]
    pub fn ordering_dependencies_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        self.data
            .ordering_dependencies
            .get(id)
            .map(|items| items.iter().cloned().collect())
    }

    /// Returns direct compile-order dependents.
    #[must_use]
    pub fn ordering_dependents_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        self.data
            .ordering_dependents
            .get(id)
            .map(|items| items.iter().cloned().collect())
    }

    /// Returns all relationship kinds between two units.
    ///
    /// Results are in deterministic `DependencyKind` order.
    #[must_use]
    pub fn relationships_between(
        &self,
        dependent: &CompilationUnitId,
        dependency: &CompilationUnitId,
    ) -> Vec<DependencyKind> {
        self.data
            .relationships
            .get(&(dependent.clone(), dependency.clone()))
            .map(|items| items.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Returns whether a typed relationship exists.
    #[must_use]
    pub fn contains_edge(&self, edge: &DependencyEdge) -> bool {
        self.data.edges.contains(edge)
    }

    /// Returns the transitive dependency closure of a unit.
    ///
    /// All relationship kinds are followed. The starting unit is excluded
    /// from the result, even if non-ordering relationships form a cycle.
    ///
    /// The returned identifiers are sorted and unique.
    ///
    /// `None` means the requested unit is unknown.
    pub fn transitive_dependencies_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        if !self.contains_unit(id) {
            return None;
        }

        Some(self.transitive_closure(
            id,
            &self.data.dependencies,
        ))
    }

    /// Returns all transitive dependents of a unit.
    ///
    /// All relationship kinds are followed. The starting unit is excluded.
    /// The returned identifiers are sorted and unique.
    ///
    /// `None` means the requested unit is unknown.
    pub fn transitive_dependents_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        if !self.contains_unit(id) {
            return None;
        }

        Some(self.transitive_closure(
            id,
            &self.data.dependents,
        ))
    }

    /// Returns transitive compile-order dependencies.
    ///
    /// Non-ordering relationships are excluded.
    pub fn transitive_ordering_dependencies_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        if !self.contains_unit(id) {
            return None;
        }

        Some(self.transitive_closure(
            id,
            &self.data.ordering_dependencies,
        ))
    }

    /// Returns transitive compile-order dependents.
    ///
    /// Non-ordering relationships are excluded.
    pub fn transitive_ordering_dependents_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        if !self.contains_unit(id) {
            return None;
        }

        Some(self.transitive_closure(
            id,
            &self.data.ordering_dependents,
        ))
    }

    fn transitive_closure(
        &self,
        start: &CompilationUnitId,
        adjacency: &BTreeMap<
            CompilationUnitId,
            BTreeSet<CompilationUnitId>,
        >,
    ) -> Vec<CompilationUnitId> {
        let mut visited = BTreeSet::new();
        let mut pending = Vec::new();

        if let Some(neighbors) = adjacency.get(start) {
            pending.extend(neighbors.iter().cloned());
        }

        while let Some(current) = pending.pop() {
            if &current == start || !visited.insert(current.clone()) {
                continue;
            }

            if let Some(neighbors) = adjacency.get(&current) {
                pending.extend(neighbors.iter().cloned());
            }
        }

        visited.into_iter().collect()
    }

    /// Returns deterministic dependency-first compilation order.
    ///
    /// Only ordering relationships participate.
    ///
    /// The algorithm is iterative Kahn topological sorting. It does not
    /// recurse and therefore does not consume call-stack space proportional
    /// to dependency depth.
    ///
    /// Independent units are selected by stable identifier order.
    pub fn dependency_first_order(
        &self,
    ) -> Result<Vec<CompilationUnitId>, DependencyGraphError> {
        let mut remaining_dependencies = BTreeMap::new();
        let mut ready = BTreeSet::new();

        for id in self.data.units.keys() {
            let count = self
                .data
                .ordering_dependencies
                .get(id)
                .map_or(0, BTreeSet::len);

            remaining_dependencies.insert(id.clone(), count);

            if count == 0 {
                ready.insert(id.clone());
            }
        }

        let mut order = Vec::new();

        while let Some(next) = ready.pop_first() {
            order.push(next.clone());

            if let Some(dependents) =
                self.data.ordering_dependents.get(&next)
            {
                for dependent in dependents {
                    let Some(remaining) =
                        remaining_dependencies.get_mut(dependent)
                    else {
                        continue;
                    };

                    if *remaining == 0 {
                        continue;
                    }

                    *remaining -= 1;

                    if *remaining == 0 {
                        ready.insert(dependent.clone());
                    }
                }
            }
        }

        if order.len() != self.data.units.len() {
            let blocked_units = remaining_dependencies
                .into_iter()
                .filter_map(|(id, remaining)| {
                    (remaining != 0).then_some(id)
                })
                .collect();

            return Err(DependencyGraphError::OrderingCycle {
                blocked_units,
            });
        }

        Ok(order)
    }

    /// Validates all graph endpoints and compile-order constraints.
    ///
    /// Non-ordering cycles are permitted.
    pub fn validate(&self) -> Result<(), DependencyGraphError> {
        for edge in &self.data.edges {
            if !self.data.units.contains_key(&edge.dependent) {
                return Err(DependencyGraphError::UnknownDependent {
                    unit: edge.dependent.clone(),
                });
            }

            if !self.data.units.contains_key(&edge.dependency) {
                return Err(DependencyGraphError::UnknownDependency {
                    dependent: edge.dependent.clone(),
                    dependency: edge.dependency.clone(),
                });
            }
        }

        self.dependency_first_order().map(|_| ())
    }

    /// Returns the number of strong references to the shared graph.
    ///
    /// Informational only; do not use for synchronization.
    #[must_use]
    pub fn shared_reference_count(&self) -> usize {
        Arc::strong_count(&self.data)
    }

    /// Creates a builder initialized from this graph.
    ///
    /// The original graph remains immutable.
    #[must_use]
    pub fn to_builder(&self) -> DependencyContextBuilder {
        DependencyContextBuilder {
            units: self.data.units.clone(),
            edges: self.data.edges.clone(),
        }
    }
}

// ============================================================================
// BUILDER
// ============================================================================

/// Mutable builder for an immutable dependency context.
///
/// A builder should be owned by one graph-construction operation.
/// Build the context before sharing it with concurrent compiler stages.
#[derive(Debug, Default)]
pub struct DependencyContextBuilder {
    units: BTreeMap<CompilationUnitId, CompilationUnit>,
    edges: BTreeSet<DependencyEdge>,
}

impl DependencyContextBuilder {
    /// Creates an empty builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a compilation unit.
    ///
    /// Duplicate identities are rejected; existing metadata is never
    /// silently overwritten.
    pub fn add_unit(
        &mut self,
        unit: CompilationUnit,
    ) -> Result<(), DependencyGraphError> {
        let id = unit.id.clone();

        if self.units.contains_key(&id) {
            return Err(DependencyGraphError::DuplicateUnit { unit: id });
        }

        self.units.insert(id, unit);
        Ok(())
    }

    /// Registers a typed dependency relationship.
    ///
    /// Both endpoints must already exist. Identical edges are idempotent.
    pub fn add_dependency(
        &mut self,
        dependent: CompilationUnitId,
        dependency: CompilationUnitId,
        kind: DependencyKind,
    ) -> Result<(), DependencyGraphError> {
        let edge = DependencyEdge::new(
            dependent,
            dependency,
            kind,
        )?;

        self.add_edge(edge)
    }

    /// Registers a preconstructed edge.
    ///
    /// Endpoint validation is performed before insertion.
    pub fn add_edge(
        &mut self,
        edge: DependencyEdge,
    ) -> Result<(), DependencyGraphError> {
        if !self.units.contains_key(&edge.dependent) {
            return Err(DependencyGraphError::UnknownDependent {
                unit: edge.dependent,
            });
        }

        if !self.units.contains_key(&edge.dependency) {
            return Err(DependencyGraphError::UnknownDependency {
                dependent: edge.dependent,
                dependency: edge.dependency,
            });
        }

        self.edges.insert(edge);
        Ok(())
    }

    /// Returns the number of registered units.
    #[must_use]
    pub fn unit_count(&self) -> usize {
        self.units.len()
    }

    /// Returns the number of unique typed relationships.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Builds and validates an immutable dependency context.
    ///
    /// Ordering cycles are rejected. Non-ordering cycles are permitted.
    pub fn build(self) -> Result<DependencyContext, DependencyGraphError> {
        let mut dependencies = BTreeMap::new();
        let mut dependents = BTreeMap::new();
        let mut ordering_dependencies = BTreeMap::new();
        let mut ordering_dependents = BTreeMap::new();
        let mut relationships = BTreeMap::new();

        for id in self.units.keys() {
            dependencies.insert(id.clone(), BTreeSet::new());
            dependents.insert(id.clone(), BTreeSet::new());
            ordering_dependencies.insert(id.clone(), BTreeSet::new());
            ordering_dependents.insert(id.clone(), BTreeSet::new());
        }

        for edge in &self.edges {
            let Some(unit_dependencies) =
                dependencies.get_mut(&edge.dependent)
            else {
                return Err(DependencyGraphError::UnknownDependent {
                    unit: edge.dependent.clone(),
                });
            };

            unit_dependencies.insert(edge.dependency.clone());

            let Some(unit_dependents) =
                dependents.get_mut(&edge.dependency)
            else {
                return Err(DependencyGraphError::UnknownDependency {
                    dependent: edge.dependent.clone(),
                    dependency: edge.dependency.clone(),
                });
            };

            unit_dependents.insert(edge.dependent.clone());

            if edge.requires_ordering() {
                let Some(items) =
                    ordering_dependencies.get_mut(&edge.dependent)
                else {
                    return Err(DependencyGraphError::UnknownDependent {
                        unit: edge.dependent.clone(),
                    });
                };

                items.insert(edge.dependency.clone());

                let Some(items) =
                    ordering_dependents.get_mut(&edge.dependency)
                else {
                    return Err(DependencyGraphError::UnknownDependency {
                        dependent: edge.dependent.clone(),
                        dependency: edge.dependency.clone(),
                    });
                };

                items.insert(edge.dependent.clone());
            }

            relationships
                .entry((
                    edge.dependent.clone(),
                    edge.dependency.clone(),
                ))
                .or_insert_with(BTreeSet::new)
                .insert(edge.kind.clone());
        }

        let context = DependencyContext {
            data: Arc::new(DependencyGraphData {
                units: self.units,
                edges: self.edges,
                dependencies,
                dependents,
                ordering_dependencies,
                ordering_dependents,
                relationships,
            }),
        };

        context.validate()?;
        Ok(context)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> CompilationUnitId {
        CompilationUnitId::new(value).expect("valid identifier")
    }

    fn kind(value: &str) -> CompilationUnitKind {
        CompilationUnitKind::new(value).expect("valid kind")
    }

    fn unit(value: &str) -> CompilationUnit {
        CompilationUnit::new(
            id(value),
            kind("source"),
            value,
        )
        .expect("valid unit")
    }

    fn graph_with_units(
        identifiers: &[&str],
    ) -> DependencyContextBuilder {
        let mut builder = DependencyContextBuilder::new();

        for identifier in identifiers {
            builder
                .add_unit(unit(identifier))
                .expect("unit registration");
        }

        builder
    }

    #[test]
    fn empty_context_is_valid() {
        let context = DependencyContext::new();

        assert!(context.is_empty());
        assert_eq!(context.unit_count(), 0);
        assert_eq!(context.edge_count(), 0);
        assert_eq!(
            context.dependency_first_order().expect("valid order"),
            Vec::<CompilationUnitId>::new()
        );
    }

    #[test]
    fn identifiers_reject_empty_values() {
        assert_eq!(
            CompilationUnitId::new(""),
            Err(DependencyGraphError::EmptyIdentifier)
        );

        assert_eq!(
            CompilationUnitKind::new("  "),
            Err(DependencyGraphError::EmptyUnitKind)
        );
    }

    #[test]
    fn identifiers_reject_control_characters() {
        assert_eq!(
            CompilationUnitId::new("module\nname"),
            Err(DependencyGraphError::InvalidIdentifier)
        );

        assert_eq!(
            CompilationUnitKind::new("source\tkind"),
            Err(DependencyGraphError::InvalidUnitKind)
        );
    }

    #[test]
    fn logical_names_are_validated() {
        assert_eq!(
            CompilationUnit::new(id("unit"), kind("source"), " "),
            Err(DependencyGraphError::EmptyLogicalName)
        );

        assert_eq!(
            CompilationUnit::new(id("unit"), kind("source"), "bad\nname"),
            Err(DependencyGraphError::InvalidLogicalName)
        );
    }

    #[test]
    fn extensible_unit_kinds_are_supported() {
        let extension_kind = kind("plugin:future-computing-domain");

        let item = CompilationUnit::new(
            id("future-unit"),
            extension_kind.clone(),
            "future-unit",
        )
        .expect("unit");

        assert_eq!(item.kind(), &extension_kind);
        assert_eq!(
            item.kind().as_str(),
            "plugin:future-computing-domain"
        );
    }

    #[test]
    fn duplicate_units_are_rejected() {
        let mut builder = DependencyContextBuilder::new();

        builder.add_unit(unit("module")).expect("first unit");

        assert_eq!(
            builder.add_unit(unit("module")),
            Err(DependencyGraphError::DuplicateUnit {
                unit: id("module"),
            })
        );
    }

    #[test]
    fn unknown_dependency_is_rejected() {
        let mut builder = graph_with_units(&["application"]);

        assert_eq!(
            builder.add_dependency(
                id("application"),
                id("missing-library"),
                DependencyKind::ModuleImport,
            ),
            Err(DependencyGraphError::UnknownDependency {
                dependent: id("application"),
                dependency: id("missing-library"),
            })
        );
    }

    #[test]
    fn unknown_dependent_is_rejected() {
        let mut builder = graph_with_units(&["library"]);

        assert_eq!(
            builder.add_dependency(
                id("missing-application"),
                id("library"),
                DependencyKind::ModuleImport,
            ),
            Err(DependencyGraphError::UnknownDependent {
                unit: id("missing-application"),
            })
        );
    }

    #[test]
    fn dependency_direction_is_dependent_to_dependency() {
        let mut builder =
            graph_with_units(&["application", "library"]);

        builder
            .add_dependency(
                id("application"),
                id("library"),
                DependencyKind::ModuleImport,
            )
            .expect("edge");

        let context = builder.build().expect("context");

        assert_eq!(
            context.dependencies_of(&id("application")),
            Some(vec![id("library")])
        );

        assert_eq!(
            context.dependents_of(&id("library")),
            Some(vec![id("application")])
        );
    }

    #[test]
    fn compilation_order_is_dependency_first() {
        let mut builder =
            graph_with_units(&["application", "library", "foundation"]);

        builder
            .add_dependency(
                id("application"),
                id("library"),
                DependencyKind::ModuleImport,
            )
            .expect("application dependency");

        builder
            .add_dependency(
                id("library"),
                id("foundation"),
                DependencyKind::ModuleImport,
            )
            .expect("library dependency");

        let context = builder.build().expect("valid graph");

        assert_eq!(
            context.dependency_first_order().expect("order"),
            vec![
                id("foundation"),
                id("library"),
                id("application"),
            ]
        );
    }

    #[test]
    fn independent_units_have_deterministic_order() {
        let mut builder =
            graph_with_units(&["zeta", "alpha", "middle"]);

        let context = builder.build().expect("context");

        assert_eq!(
            context.dependency_first_order().expect("order"),
            vec![id("alpha"), id("middle"), id("zeta")]
        );
    }

    #[test]
    fn ordering_cycle_is_rejected() {
        let mut builder = graph_with_units(&["first", "second"]);

        builder
            .add_dependency(
                id("first"),
                id("second"),
                DependencyKind::ModuleImport,
            )
            .expect("first edge");

        builder
            .add_dependency(
                id("second"),
                id("first"),
                DependencyKind::ModuleImport,
            )
            .expect("second edge");

        assert!(matches!(
            builder.build(),
            Err(DependencyGraphError::OrderingCycle { .. })
        ));
    }

    #[test]
    fn self_ordering_dependency_is_rejected() {
        let mut builder = graph_with_units(&["module"]);

        assert_eq!(
            builder.add_dependency(
                id("module"),
                id("module"),
                DependencyKind::CompileTime,
            ),
            Err(DependencyGraphError::SelfOrderingDependency {
                unit: id("module"),
            })
        );
    }

    #[test]
    fn non_ordering_self_relationship_is_permitted() {
        let mut builder = graph_with_units(&["unit"]);

        builder
            .add_dependency(
                id("unit"),
                id("unit"),
                DependencyKind::Feedback,
            )
            .expect("feedback relationship");

        let context = builder.build().expect("valid non-ordering graph");

        assert_eq!(context.edge_count(), 1);
        assert_eq!(
            context.dependency_first_order().expect("order"),
            vec![id("unit")]
        );
    }

    #[test]
    fn feedback_cycle_does_not_create_ordering_cycle() {
        let mut builder =
            graph_with_units(&["controller", "processor"]);

        builder
            .add_dependency(
                id("controller"),
                id("processor"),
                DependencyKind::Feedback,
            )
            .expect("feedback edge");

        builder
            .add_dependency(
                id("processor"),
                id("controller"),
                DependencyKind::Feedback,
            )
            .expect("reverse feedback edge");

        let context = builder.build().expect("feedback graph");

        assert_eq!(context.edge_count(), 2);
        assert_eq!(
            context.dependency_first_order().expect("order").len(),
            2
        );
    }

    #[test]
    fn data_flow_does_not_impose_compilation_order() {
        let mut builder =
            graph_with_units(&["producer", "consumer"]);

        builder
            .add_dependency(
                id("consumer"),
                id("producer"),
                DependencyKind::DataFlow,
            )
            .expect("data-flow edge");

        let context = builder.build().expect("context");

        assert_eq!(
            context.ordering_dependencies_of(&id("consumer")),
            Some(Vec::new())
        );

        assert_eq!(context.edge_count(), 1);
    }

    #[test]
    fn custom_ordering_relationship_is_supported() {
        let mut builder =
            graph_with_units(&["application", "provider"]);

        builder
            .add_dependency(
                id("application"),
                id("provider"),
                DependencyKind::Custom {
                    identifier: Arc::from("plugin:custom-build-input"),
                    requires_ordering: true,
                },
            )
            .expect("custom relationship");

        let context = builder.build().expect("context");

        assert_eq!(
            context.dependency_first_order().expect("order"),
            vec![id("provider"), id("application")]
        );
    }

    #[test]
    fn invalid_custom_relationship_identifiers_are_rejected() {
        let mut builder =
            graph_with_units(&["application", "provider"]);

        assert_eq!(
            builder.add_dependency(
                id("application"),
                id("provider"),
                DependencyKind::Custom {
                    identifier: Arc::from(" "),
                    requires_ordering: false,
                },
            ),
            Err(DependencyGraphError::EmptyRelationshipIdentifier)
        );
    }

    #[test]
    fn duplicate_identical_edges_are_idempotent() {
        let mut builder =
            graph_with_units(&["application", "library"]);

        for _ in 0..2 {
            builder
                .add_dependency(
                    id("application"),
                    id("library"),
                    DependencyKind::ModuleImport,
                )
                .expect("edge");
        }

        assert_eq!(builder.edge_count(), 1);
    }

    #[test]
    fn distinct_relationships_between_same_units_are_preserved() {
        let mut builder =
            graph_with_units(&["application", "library"]);

        builder
            .add_dependency(
                id("application"),
                id("library"),
                DependencyKind::ModuleImport,
            )
            .expect("module import");

        builder
            .add_dependency(
                id("application"),
                id("library"),
                DependencyKind::Interface,
            )
            .expect("interface");

        let context = builder.build().expect("context");

        assert_eq!(context.edge_count(), 2);
        assert_eq!(
            context.relationships_between(
                &id("application"),
                &id("library"),
            ),
            vec![
                DependencyKind::Interface,
                DependencyKind::ModuleImport,
            ]
        );
    }

    #[test]
    fn relationship_lookup_uses_the_pair_index() {
        let mut builder =
            graph_with_units(&["application", "library", "other"]);

        builder
            .add_dependency(
                id("application"),
                id("library"),
                DependencyKind::ModuleImport,
            )
            .expect("edge");

        let context = builder.build().expect("context");

        assert_eq!(
            context.relationships_between(
                &id("application"),
                &id("library"),
            ),
            vec![DependencyKind::ModuleImport]
        );

        assert!(context.relationships_between(
            &id("application"),
            &id("other"),
        ).is_empty());
    }

    #[test]
    fn transitive_dependencies_are_deterministic_and_unique() {
        let mut builder = graph_with_units(&[
            "application",
            "library-a",
            "library-b",
            "foundation",
        ]);

        builder
            .add_dependency(
                id("application"),
                id("library-a"),
                DependencyKind::ModuleImport,
            )
            .expect("edge");

        builder
            .add_dependency(
                id("application"),
                id("library-b"),
                DependencyKind::ModuleImport,
            )
            .expect("edge");

        builder
            .add_dependency(
                id("library-a"),
                id("foundation"),
                DependencyKind::ModuleImport,
            )
            .expect("edge");

        builder
            .add_dependency(
                id("library-b"),
                id("foundation"),
                DependencyKind::ModuleImport,
            )
            .expect("edge");

        let context = builder.build().expect("context");

        assert_eq!(
            context.transitive_dependencies_of(&id("application")),
            Some(vec![
                id("foundation"),
                id("library-a"),
                id("library-b"),
            ])
        );
    }

    #[test]
    fn transitive_dependents_support_impact_analysis() {
        let mut builder = graph_with_units(&[
            "application",
            "library",
            "foundation",
        ]);

        builder
            .add_dependency(
                id("application"),
                id("library"),
                DependencyKind::ModuleImport,
            )
            .expect("edge");

        builder
            .add_dependency(
                id("library"),
                id("foundation"),
                DependencyKind::ModuleImport,
            )
            .expect("edge");

        let context = builder.build().expect("context");

        assert_eq!(
            context.transitive_dependents_of(&id("foundation")),
            Some(vec![id("application"), id("library")])
        );
    }

    #[test]
    fn unknown_unit_queries_return_none() {
        let context = DependencyContext::new();

        assert_eq!(
            context.transitive_dependencies_of(&id("missing")),
            None
        );

        assert_eq!(
            context.transitive_dependents_of(&id("missing")),
            None
        );
    }

    #[test]
    fn cloned_contexts_share_immutable_graph() {
        let mut builder = graph_with_units(&["module"]);
        let context = builder.build().expect("context");
        let clone = context.clone();

        assert_eq!(context.shared_reference_count(), 2);
        assert_eq!(clone.unit_count(), 1);
        assert!(clone.contains_unit(&id("module")));
    }

    #[test]
    fn builder_from_context_does_not_mutate_original() {
        let original = graph_with_units(&["first"])
            .build()
            .expect("original");

        let mut modified_builder = original.to_builder();

        modified_builder
            .add_unit(unit("second"))
            .expect("second unit");

        let modified = modified_builder.build().expect("modified");

        assert_eq!(original.unit_count(), 1);
        assert_eq!(modified.unit_count(), 2);
    }

    #[test]
    fn long_dependency_chains_do_not_use_recursive_traversal() {
        let count = 2_000usize;
        let mut builder = DependencyContextBuilder::new();

        for index in 0..count {
            let identifier = format!("unit-{index:08}");

            builder
                .add_unit(unit(&identifier))
                .expect("unit");
        }

        for index in 1..count {
            let dependent = format!("unit-{index:08}");
            let dependency = format!("unit-{:08}", index - 1);

            builder
                .add_dependency(
                    id(&dependent),
                    id(&dependency),
                    DependencyKind::CompileTime,
                )
                .expect("edge");
        }

        let context = builder.build().expect("context");
        let order = context.dependency_first_order().expect("order");

        assert_eq!(order.len(), count);
        assert_eq!(order.first(), Some(&id("unit-00000000")));
        assert_eq!(order.last(), Some(&id("unit-00001999")));
    }

    #[test]
    fn non_ordering_cycles_are_safe_for_transitive_queries() {
        let mut builder = graph_with_units(&["first", "second"]);

        builder
            .add_dependency(
                id("first"),
                id("second"),
                DependencyKind::Feedback,
            )
            .expect("edge");

        builder
            .add_dependency(
                id("second"),
                id("first"),
                DependencyKind::Feedback,
            )
            .expect("edge");

        let context = builder.build().expect("context");

        assert_eq!(
            context.transitive_dependencies_of(&id("first")),
            Some(vec![id("second")])
        );

        assert_eq!(
            context.transitive_dependents_of(&id("first")),
            Some(vec![id("second")])
        );
    }
}
