
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Compilation Dependency Context.
//!
//! Path:
//!     src/compiler/context/dependency_context.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only. Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Provides an immutable, shareable dependency snapshot for the compilation
//! units participating in one compilation operation.
//!
//! This module records:
//!
//! - Compilation-unit identities.
//! - Extensible compilation-unit kinds.
//! - Typed relationships between compilation units.
//! - Direct dependencies.
//! - Direct dependents.
//! - Compilation-order dependencies.
//! - Non-ordering relationships.
//! - Deterministic dependency-first ordering.
//! - Dependency-graph validation.
//! - Explicit ordering-cycle detection.
//!
//! # Ownership boundaries
//!
//! This module owns the dependency snapshot used by compiler stages.
//!
//! It does not own:
//!
//! - Source parsing or AST construction.
//! - Name or module resolution.
//! - Semantic dependency discovery.
//! - Type or ownership analysis.
//! - Source-file content hashing.
//! - Incremental cache invalidation.
//! - Dirty-file propagation.
//! - Persistent build-cache storage.
//! - Parallel job execution.
//! - Pipeline orchestration.
//! - IR definitions.
//! - Hardware discovery or allocation.
//! - Backend selection.
//! - Runtime execution.
//! - Distributed scheduling.
//!
//! Existing authorities:
//!
//! `crate::compiler::incremental`
//!     File-level fingerprints, dependency invalidation, dirty propagation,
//!     and incremental compilation planning.
//!
//! `crate::compiler::parallel_build`
//!     Parallel execution of independent compilation jobs.
//!
//! `crate::compiler::context::compilation_context`
//!     Shared configuration, resource budgets, source identity,
//!     and cancellation.
//!
//! `crate::compiler::context::compilation_state`
//!     Compilation lifecycle and progress.
//!
//! `crate::frontend` and the semantic implementation
//!     Source-level declarations, imports, names, and semantic validity.
//!
//! # Dependency direction
//!
//! Frontend / semantic dependency discovery
//!                 |
//!                 v
//!       DependencyContextBuilder
//!                 |
//!                 v
//!         DependencyContext
//!                 |
//!          +------+-------+
//!          |              |
//!          v              v
//!     Pipeline order   Build planning
//!          |              |
//!          v              v
//!     Compiler stages  Incremental compiler
//!                         |
//!                         v
//!                   Parallel build engine
//!
//! # Relationship direction
//!
//! An edge is expressed as:
//!
//!     dependent -> dependency
//!
//! For example:
//!
//!     application -> library
//!
//! means that `application` depends on `library`.
//!
//! Dependency-first compilation order is therefore:
//!
//!     library, application
//!
//! # Ordering versus non-ordering relationships
//!
//! Not every program relationship imposes compilation order.
//!
//! Compile-time imports, macro expansion inputs, generated-source inputs,
//! and required build inputs impose ordering.
//!
//! Data flow, control flow, completion, feedback, and synchronization
//! relationships are recorded but do not automatically impose compilation
//! order.
//!
//! This distinction allows hybrid, cyclic, concurrent, and feedback-based
//! computation relationships to be represented without incorrectly treating
//! every relationship as a compile-time dependency.
//!
//! # POCO-REAF
//!
//! This module has no fixed limits on:
//!
//! - Compilation units.
//! - Dependencies.
//! - Dependents.
//! - Modules.
//! - Functions.
//! - Computing domains.
//! - Hardware types.
//! - Devices.
//! - Nodes.
//! - CPUs.
//! - GPUs.
//! - QPUs.
//! - FPGAs.
//! - Tensor dimensions.
//! - IR operations.
//!
//! Actual memory and execution constraints remain subject to available
//! resources and the compilation context's configured resource budgets.
//!
//! No finite implementation can promise infinite physical memory.
//!
//! # Determinism
//!
//! Ordered maps and sets provide deterministic iteration.
//!
//! The dependency-first ordering algorithm uses stable identifiers to
//! resolve otherwise independent units deterministically.
//!
//! This module does not calculate source fingerprints. Fingerprint
//! ownership remains with the incremental compiler.
//!
//! # Concurrency
//!
//! A built DependencyContext is immutable.
//!
//! Cloning it shares the underlying graph through Arc.
//!
//! Concurrent readers require no additional locking.
//!
//! Construction is isolated in DependencyContextBuilder.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version for the dependency-context contract.
///
/// This version is independent of the Zamani language, grammar, compiler,
/// semantic-model, IR, and backend ABI versions.
pub const DEPENDENCY_CONTEXT_API_VERSION: u16 = 1;

// ============================================================================
// COMPILATION UNIT IDENTITY
// ============================================================================

/// Stable logical identity of a compilation unit.
///
/// The identifier is independent of:
///
/// - Filesystem location.
/// - Physical machine.
/// - Operating system.
/// - Backend.
/// - Device.
/// - Runtime allocation.
/// - Compilation worker.
///
/// A frontend, module resolver, build system, or generated-source provider
/// supplies the identity.
///
/// No arbitrary identifier-length ceiling is imposed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompilationUnitId(Arc<str>);

impl CompilationUnitId {
    /// Creates a compilation-unit identifier.
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

    /// Returns the stable identifier.
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

// ============================================================================
// OPEN COMPILATION UNIT KIND
// ============================================================================

/// Extensible classification of a compilation unit.
///
/// This is intentionally an open string identifier rather than a closed
/// enum of every possible computing domain.
///
/// Examples:
///
/// - `source`
— `module`
— `library`
— `classical`
— `quantum`
— `hdl`
— `ai`
— `tensor`
— `distributed`
— `generated`
— `interface`
— `plugin:vendor.extension`
///
/// The compiler does not need a source change merely because a new
/// compilation-unit category is introduced.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompilationUnitKind(Arc<str>);

impl CompilationUnitKind {
    /// Creates an extensible compilation-unit kind.
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
/// `logical_name` is descriptive metadata. It is not a filesystem path
/// and is not used as a substitute for canonical source-map identity.
///
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

    /// Returns the stable compilation-unit identifier.
    #[must_use]
    pub fn id(&self) -> &CompilationUnitId {
        &self.id
    }

    /// Returns the extensible compilation-unit kind.
    #[must_use]
    pub fn kind(&self) -> &CompilationUnitKind {
        &self.kind
    }

    /// Returns the logical display name.
    #[must_use]
    pub fn logical_name(&self) -> &str {
        &self.logical_name
    }
}

// ============================================================================
// DEPENDENCY RELATIONSHIPS
// ============================================================================

/// Typed relationship between compilation units.
///
/// Ordering relationships require the dependency to be processed before
/// the dependent unit.
///
/// Non-ordering relationships are preserved as graph information but do
/// not participate in the dependency-first topological order.
///
/// The `Custom` variant permits extensions to introduce relationships
/// without modifying this module's public enum.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum DependencyKind {
    /// A source unit imports another source unit.
    ModuleImport,

    /// A compilation unit requires another unit during compile time.
    CompileTime,

    /// A macro or compile-time expansion requires another unit.
    MacroExpansion,

    /// A type or interface definition must be available first.
    TypeDefinition,

    /// A unit is generated from another compilation unit.
    GeneratedFrom,

    /// A unit requires another unit as a build input.
    BuildInput,

    /// A unit requires another artifact as an input.
    ArtifactInput,

    /// A unit requires an interface declaration before compilation.
    Interface,

    /// Runtime or semantic data-flow relationship.
    DataFlow,

    /// Control-flow relationship.
    ControlFlow,

    /// Completion dependency or completion notification.
    Completion,

    /// Feedback relationship.
    ///
    /// Feedback does not automatically impose compile-time ordering.
    Feedback,

    /// Synchronization relationship.
    Synchronization,

    /// Extensible relationship.
    ///
    /// `requires_ordering` explicitly determines whether this relationship
    /// participates in dependency-first compilation ordering.
    Custom {
        /// Stable extension-defined relationship identifier.
        identifier: Arc<str>,

        /// Whether the relationship imposes compilation ordering.
        requires_ordering: bool,
    },
}

impl DependencyKind {
    /// Returns whether this relationship imposes compilation ordering.
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

    /// Returns a stable relationship identifier.
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

// ============================================================================
// DEPENDENCY EDGE
// ============================================================================

/// Directed dependency relationship.
///
/// Direction:
///
///     dependent -> dependency
///
/// The dependency must be processed before the dependent when
/// `kind.requires_ordering()` is true.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyEdge {
    dependent: CompilationUnitId,
    dependency: CompilationUnitId,
    kind: DependencyKind,
}

impl DependencyEdge {
    /// Creates a dependency edge.
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

    /// Returns the unit that depends on another unit.
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

    /// Returns whether this edge imposes compilation ordering.
    #[must_use]
    pub fn requires_ordering(&self) -> bool {
        self.kind.requires_ordering()
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by dependency-context construction and validation.
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

    /// Units blocked by a cycle in ordering relationships.
    ///
    /// This is the set of nodes that cannot be emitted by topological
    /// ordering. It can include units downstream of the actual cycle.
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
                    "compilation dependency ordering contains a cycle; \
                     blocked units: ",
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

    /// All direct dependencies:
    ///
    /// dependent -> dependencies
    dependencies: BTreeMap<CompilationUnitId, BTreeSet<CompilationUnitId>>,

    /// All direct dependents:
    ///
    /// dependency -> dependents
    dependents: BTreeMap<CompilationUnitId, BTreeSet<CompilationUnitId>>,

    /// Ordering dependencies only:
    ///
    /// dependent -> dependencies
    ordering_dependencies:
        BTreeMap<CompilationUnitId, BTreeSet<CompilationUnitId>>,

    /// Reverse ordering dependencies:
    ///
    /// dependency -> dependents
    ordering_dependents:
        BTreeMap<CompilationUnitId, BTreeSet<CompilationUnitId>>,
}

// ============================================================================
// DEPENDENCY CONTEXT
// ============================================================================

/// Immutable dependency snapshot for a compilation operation.
///
/// Clones share the graph data through `Arc`.
///
/// The context is safe for concurrent reads and requires no reader locks.
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
            data: Arc::new(DependencyGraphData {
                units: BTreeMap::new(),
                edges: BTreeSet::new(),
                dependencies: BTreeMap::new(),
                dependents: BTreeMap::new(),
                ordering_dependencies: BTreeMap::new(),
                ordering_dependents: BTreeMap::new(),
            }),
        }
    }

    /// Returns the number of registered compilation units.
    #[must_use]
    pub fn unit_count(&self) -> usize {
        self.data.units.len()
    }

    /// Returns the number of registered dependency relationships.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.data.edges.len()
    }

    /// Returns whether the context contains no compilation units.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.units.is_empty()
    }

    /// Returns a compilation unit by stable identity.
    #[must_use]
    pub fn unit(&self, id: &CompilationUnitId) -> Option<&CompilationUnit> {
        self.data.units.get(id)
    }

    /// Returns all units in deterministic identifier order.
    pub fn units(
        &self,
    ) -> impl ExactSizeIterator<Item = &CompilationUnit> {
        self.data.units.values()
    }

    /// Returns all dependency edges in deterministic order.
    pub fn edges(
        &self,
    ) -> impl ExactSizeIterator<Item = &DependencyEdge> {
        self.data.edges.iter()
    }

    /// Returns whether a compilation unit is registered.
    #[must_use]
    pub fn contains_unit(&self, id: &CompilationUnitId) -> bool {
        self.data.units.contains_key(id)
    }

    /// Returns the direct dependencies of a unit.
    ///
    /// All relationship kinds are included.
    ///
    /// The returned identifiers are deterministically ordered.
    #[must_use]
    pub fn dependencies_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        self.data
            .dependencies
            .get(id)
            .map(|dependencies| dependencies.iter().cloned().collect())
    }

    /// Returns the direct dependents of a unit.
    ///
    /// All relationship kinds are included.
    #[must_use]
    pub fn dependents_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        self.data
            .dependents
            .get(id)
            .map(|dependents| dependents.iter().cloned().collect())
    }

    /// Returns direct ordering dependencies of a unit.
    #[must_use]
    pub fn ordering_dependencies_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        self.data
            .ordering_dependencies
            .get(id)
            .map(|dependencies| dependencies.iter().cloned().collect())
    }

    /// Returns direct ordering dependents of a unit.
    #[must_use]
    pub fn ordering_dependents_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        self.data
            .ordering_dependents
            .get(id)
            .map(|dependents| dependents.iter().cloned().collect())
    }

    /// Returns all relationship kinds between two units.
    #[must_use]
    pub fn relationships_between(
        &self,
        dependent: &CompilationUnitId,
        dependency: &CompilationUnitId,
    ) -> Vec<DependencyKind> {
        self.data
            .edges
            .iter()
            .filter(|edge| {
                &edge.dependent == dependent
                    && &edge.dependency == dependency
            })
            .map(|edge| edge.kind.clone())
            .collect()
    }

    /// Returns a deterministic dependency-first compilation order.
    ///
    /// Only ordering relationships participate.
    ///
    /// Non-ordering relationships such as feedback and data flow remain
    /// represented in the graph but do not create false compilation cycles.
    ///
    /// Independent units are ordered by their stable identifiers.
    ///
    /// The implementation uses Kahn's algorithm and does not recurse,
    /// avoiding call-stack growth proportional to graph depth.
    pub fn dependency_first_order(
        &self,
    ) -> Result<Vec<CompilationUnitId>, DependencyGraphError> {
        let mut remaining_dependencies: BTreeMap<
            CompilationUnitId,
            usize,
        > = BTreeMap::new();

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

        let mut order = Vec::with_capacity(self.data.units.len());

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

    /// Validates the graph and its compilation-order constraints.
    ///
    /// A feedback or data-flow cycle is permitted because it is not
    /// automatically a compile-time ordering cycle.
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
    /// This is informational only and must not be used for synchronization.
    #[must_use]
    pub fn shared_reference_count(&self) -> usize {
        Arc::strong_count(&self.data)
    }

    /// Returns a builder for a modified graph.
    ///
    /// The existing context remains immutable. The builder starts with
    /// a copy of its logical graph and creates a new snapshot on build.
    #[must_use]
    pub fn to_builder(&self) -> DependencyContextBuilder {
        DependencyContextBuilder {
            units: self.data.units.clone(),
            edges: self.data.edges.clone(),
        }
    }
}

// ============================================================================
// DEPENDENCY CONTEXT BUILDER
// ============================================================================

/// Mutable builder for an immutable dependency context.
///
/// A builder is intended to be owned by one graph-construction operation.
/// Build the immutable context before sharing it between compiler stages.
#[derive(Debug, Default)]
pub struct DependencyContextBuilder {
    units: BTreeMap<CompilationUnitId, CompilationUnit>,
    edges: BTreeSet<DependencyEdge>,
}

impl DependencyContextBuilder {
    /// Creates an empty dependency-context builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a compilation unit.
    ///
    /// Duplicate identifiers are rejected rather than silently replacing
    /// an existing unit with potentially different metadata.
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

    /// Registers a typed relationship.
    ///
    /// Both endpoint units must already have been registered.
    ///
    /// Duplicate identical relationships are idempotent.
    pub fn add_dependency(
        &mut self,
        dependent: CompilationUnitId,
        dependency: CompilationUnitId,
        kind: DependencyKind,
    ) -> Result<(), DependencyGraphError> {
        if !self.units.contains_key(&dependent) {
            return Err(DependencyGraphError::UnknownDependent {
                unit: dependent,
            });
        }

        if !self.units.contains_key(&dependency) {
            return Err(DependencyGraphError::UnknownDependency {
                dependent,
                dependency,
            });
        }

        let edge = DependencyEdge::new(
            dependent,
            dependency,
            kind,
        )?;

        self.edges.insert(edge);
        Ok(())
    }

    /// Returns the number of registered units.
    #[must_use]
    pub fn unit_count(&self) -> usize {
        self.units.len()
    }

    /// Returns the number of unique registered relationships.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Builds and validates an immutable dependency context.
    ///
    /// Ordering cycles are rejected.
    ///
    /// Non-ordering cycles are permitted.
    pub fn build(self) -> Result<DependencyContext, DependencyGraphError> {
        let mut dependencies: BTreeMap<
            CompilationUnitId,
            BTreeSet<CompilationUnitId>,
        > = BTreeMap::new();

        let mut dependents: BTreeMap<
            CompilationUnitId,
            BTreeSet<CompilationUnitId>,
        > = BTreeMap::new();

        let mut ordering_dependencies: BTreeMap<
            CompilationUnitId,
            BTreeSet<CompilationUnitId>,
        > = BTreeMap::new();

        let mut ordering_dependents: BTreeMap<
            CompilationUnitId,
            BTreeSet<CompilationUnitId>,
        > = BTreeMap::new();

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
                ordering_dependencies
                    .get_mut(&edge.dependent)
                    .expect("registered unit must have an ordering entry")
                    .insert(edge.dependency.clone());

                ordering_dependents
                    .get_mut(&edge.dependency)
                    .expect("registered unit must have an ordering entry")
                    .insert(edge.dependent.clone());
            }
        }

        let context = DependencyContext {
            data: Arc::new(DependencyGraphData {
                units: self.units,
                edges: self.edges,
                dependencies,
                dependents,
                ordering_dependencies,
                ordering_dependents,
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
            builder.add_unit(unit(identifier)).expect("unit registration");
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
        let mut builder = graph_with_units(&[
            "application",
            "library",
        ]);

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
        let mut builder = graph_with_units(&[
            "zeta",
            "alpha",
            "middle",
        ]);

        let context = builder.build().expect("context");

        assert_eq!(
            context.dependency_first_order().expect("order"),
            vec![
                id("alpha"),
                id("middle"),
                id("zeta"),
            ]
        );
    }

    #[test]
    fn ordering_cycle_is_rejected() {
        let mut builder = graph_with_units(&[
            "first",
            "second",
        ]);

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
    fn feedback_relationship_does_not_create_ordering_cycle() {
        let mut builder = graph_with_units(&[
            "controller",
            "processor",
        ]);

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

        let context = builder.build().expect("feedback graph is valid");

        assert_eq!(context.edge_count(), 2);
        assert_eq!(
            context.dependency_first_order().expect("order").len(),
            2
        );
    }

    #[test]
    fn data_flow_does_not_impose_compilation_order() {
        let mut builder = graph_with_units(&[
            "producer",
            "consumer",
        ]);

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
        let mut builder = graph_with_units(&[
            "application",
            "provider",
        ]);

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
            vec![
                id("provider"),
                id("application"),
            ]
        );
    }

    #[test]
    fn duplicate_identical_edges_are_idempotent() {
        let mut builder = graph_with_units(&[
            "application",
            "library",
        ]);

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
        let mut builder = graph_with_units(&[
            "application",
            "library",
        ]);

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
            .expect("interface dependency");

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
        let mut original_builder = graph_with_units(&["first"]);
        let original = original_builder.build().expect("original");

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
    fn empty_custom_relationship_identifier_is_rejected() {
        let mut builder = graph_with_units(&[
            "application",
            "provider",
        ]);

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
}
