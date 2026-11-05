
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Dependency Analysis.
//!
//! Path:
//!     src/compiler/analysis/dependency.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module provides the compiler-facing analysis interface for
//! compilation-unit dependency relationships.
//!
//! It consumes the canonical DependencyContext owned by:
//!
//!     compiler/context/dependency_context.rs
//!
//! It does not introduce another dependency graph.
//!
//! Its responsibilities are:
//!
//! - Validate compilation dependency contexts.
//! - Produce deterministic dependency-first compilation ordering.
//! - Distinguish ordering from non-ordering relationships.
//! - Report dependency statistics.
//! - Calculate transitive compilation dependencies.
//! - Calculate transitive compilation dependents.
//! - Expose dependency information to compiler planning.
//! - Preserve relationships across computational domains.
//! - Provide explicit, structured failure reporting.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - Compiler-facing dependency analysis.
//! - Dependency-analysis results.
//! - Dependency-analysis statistics.
//! - Transitive dependency queries.
//! - Transitive dependent queries.
//! - Dependency-analysis errors.
//!
//! This module does NOT own:
//!
//! - Source parsing.
//! - Grammar productions.
//! - AST construction.
//! - Module discovery.
//! - Import resolution.
//! - Symbol resolution.
//! - Type inference.
//! - Ownership analysis.
//! - Semantic dependency discovery.
//! - Semantic dependency graph storage.
//! - Compilation-unit graph storage.
//! - Source hashing.
//! - Incremental cache storage.
//! - Dirty-state management.
//! - Parallel worker execution.
//! - Hardware discovery.
//! - Hardware allocation.
//! - Scheduling.
//! - Routing.
//! - Backend selection.
//! - Code generation.
//! - Runtime execution.
//!
//! Existing authoritative implementations must be reused.
//!
//! ============================================================================
//! CANONICAL DEPENDENCY AUTHORITIES
//! ============================================================================
//!
//! Semantic entity dependencies:
//!
//!     frontend/semantic/traversal/dependency_graph.rs
//!
//! Semantic incremental dependencies:
//!
//!     frontend/semantic/incremental/dependencies.rs
//!
//! Module imports and symbol relationships:
//!
//!     frontend/semantic/symbols/imports.rs
//!
//! Import-cycle analysis:
//!
//!     frontend/semantic/symbols/cycles.rs
//!
//! Compilation-unit dependencies:
//!
//!     compiler/context/dependency_context.rs
//!
//! File-level incremental dependencies:
//!
//!     compiler/incremental.rs
//!
//! Parallel compilation:
//!
//!     compiler/parallel_build.rs
//!
//! This module consumes the compilation-unit dependency context.
//!
//! It must not replace any of these authorities.
//!
//! ============================================================================
//! DEPENDENCY DIRECTION
//! ============================================================================
//!
//!     Grammar
//!        |
//!        v
//!     Frontend AST
//!        |
//!        v
//!     Semantic analysis
//!        |
//!        v
//!     Module and symbol resolution
//!        |
//!        v
//!     Semantic dependency discovery
//!        |
//!        v
//!     DependencyContextBuilder
//!        |
//!        v
//!     DependencyContext
//!        |
//!        v
//!     DependencyAnalysis
//!        |
//!        +--------------------+
//!        |                    |
//!        v                    v
//!     Pipeline             Incremental
//!     planning             planning
//!        |                    |
//!        +---------+----------+
//!                  |
//!                  v
//!           Parallel builds
//!
//! ============================================================================
//! EDGE DIRECTION
//! ============================================================================
//!
//! A dependency edge is represented as:
//!
//!     dependent -> dependency
//!
//! Example:
//!
//!     application -> library
//!
//! Means:
//!
//!     application requires library.
//!
//! Therefore:
//!
//!     library must be processed before application.
//!
//! The canonical DependencyContext owns this relationship convention.
//!
//! ============================================================================
//! ORDERING CONTRACT
//! ============================================================================
//!
//! Compile-time dependencies impose ordering.
//!
//! Examples:
//!
//! - Module imports.
//! - Compile-time requirements.
//! - Macro expansion.
//! - Type definitions.
//! - Generated-source inputs.
//! - Build inputs.
//! - Artifact inputs.
//! - Interface declarations.
//!
//! Non-ordering relationships include:
//!
//! - Data flow.
//! - Control flow.
//! - Completion.
//! - Feedback.
//! - Synchronization.
//!
//! A non-ordering relationship must not automatically become a
//! compilation-order prerequisite.
//!
//! In particular, hybrid feedback and synchronization relationships
//! must not be incorrectly rejected as compilation cycles.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! No fixed limits are imposed on:
//!
//! - Compilation units.
//! - Dependency relationships.
//! - Modules.
//! - Functions.
//! - Computing domains.
//! - Hardware types.
//! - Devices.
//! - CPUs.
//! - GPUs.
//! - QPUs.
//! - FPGAs.
//! - Distributed nodes.
//! - Tensor dimensions.
//! - IR operations.
//!
//! Graph traversal is iterative.
//!
//! There are no recursive dependency walks.
//!
//! No arbitrary maximum graph depth is introduced.
//!
//! Actual execution is constrained by:
//!
//! - Available memory.
//! - Available processing capacity.
//! - Rust collection representational limits.
//! - Explicit operational compilation budgets.
//!
//! A finite machine cannot promise infinite memory or processing capacity.
//!
//! Resource exhaustion must never be represented as successful
//! complete dependency analysis.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! The canonical DependencyContext uses ordered graph storage.
//!
//! DependencyAnalysis preserves that ordering.
//!
//! Equivalent dependency contexts produce equivalent analysis results.
//!
//! No random ordering or hash-map iteration dependency is introduced.
//!
//! ============================================================================
//! CONCURRENCY
//! ============================================================================
//!
//! DependencyContext is immutable and internally shared.
//!
//! DependencyAnalysis owns a cheap cloned context handle.
//!
//! Analysis results are immutable after construction.
//!
//! Concurrent consumers may share DependencyAnalysis.
//!
//! This module creates no worker threads and introduces no global
//! mutable state.
//!
//! ============================================================================
//! ERROR CONTRACT
//! ============================================================================
//!
//! Incomplete dependency analysis must never be reported as success.
//!
//! Existing graph validation errors are preserved.
//!
//! Unknown compilation units are explicitly reported.
//!
//! Counter overflow is explicitly reported.
//!
//! ============================================================================
//! INTEGRATION
//! ============================================================================
//!
//! Parent:
//!
//!     compiler/analysis/mod.rs
//!
//! Required:
//!
//!     pub mod dependency;
//!
//! Parent:
//!
//!     compiler/mod.rs
//!
//! Required:
//!
//!     pub mod analysis;
//!
//! The existing compiler context must also be exposed through:
//!
//!     pub mod context;
//!
//! The existing configuration module must be exposed through:
//!
//!     pub mod configuration;
//!
//! ============================================================================

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::compiler::context::{
    CompilationUnitId,
    DependencyContext,
    DependencyGraphError,
};

// ============================================================================
// STATISTICS
// ============================================================================

/// Statistics describing a compilation dependency context.
///
/// Counts describe distinct typed relationships, not necessarily
/// distinct endpoint pairs.
///
/// For example, two different relationship kinds between the same
/// compilation units count as two relationships.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DependencyAnalysisStatistics {
    /// Number of registered compilation units.
    pub compilation_units: usize,

    /// Total number of typed dependency relationships.
    pub relationships: usize,

    /// Number of relationships requiring compilation ordering.
    pub ordering_relationships: usize,

    /// Number of relationships that do not impose compilation ordering.
    pub non_ordering_relationships: usize,

    /// Number of units without any dependency relationships.
    pub independent_units: usize,
}

impl DependencyAnalysisStatistics {
    /// Returns whether the graph contains no dependency relationships.
    #[must_use]
    pub const fn has_no_relationships(&self) -> bool {
        self.relationships == 0
    }

    /// Returns whether all relationships impose compilation ordering.
    #[must_use]
    pub const fn all_relationships_are_ordering(&self) -> bool {
        self.non_ordering_relationships == 0
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by compiler dependency analysis.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DependencyAnalysisError {
    /// An existing canonical dependency graph failed validation.
    Graph(DependencyGraphError),

    /// A requested compilation unit does not exist.
    UnknownCompilationUnit {
        unit: CompilationUnitId,
    },

    /// A statistics counter exceeded its representational capacity.
    CountOverflow {
        counter: &'static str,
    },
}

impl fmt::Display for DependencyAnalysisError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Graph(error) => {
                write!(
                    formatter,
                    "compiler dependency analysis failed: {error}"
                )
            }

            Self::UnknownCompilationUnit { unit } => {
                write!(
                    formatter,
                    "compiler dependency analysis references unknown compilation unit `{unit}`"
                )
            }

            Self::CountOverflow { counter } => {
                write!(
                    formatter,
                    "compiler dependency analysis counter overflow: {counter}"
                )
            }
        }
    }
}

impl Error for DependencyAnalysisError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Graph(error) => Some(error),
            Self::UnknownCompilationUnit { .. }
            | Self::CountOverflow { .. } => None,
        }
    }
}

impl From<DependencyGraphError> for DependencyAnalysisError {
    fn from(error: DependencyGraphError) -> Self {
        Self::Graph(error)
    }
}

// ============================================================================
// DEPENDENCY ANALYSIS
// ============================================================================

/// Immutable compiler-facing dependency-analysis result.
///
/// This structure does not own a second graph.
///
/// It retains a shared handle to the canonical DependencyContext and
/// stores the validated dependency-first compilation order.
///
/// Cloning this structure does not duplicate the underlying graph.
#[derive(Clone, Debug)]
pub struct DependencyAnalysis {
    context: DependencyContext,

    /// Validated dependency-first compilation order.
    compilation_order: Vec<CompilationUnitId>,

    /// Dependency statistics.
    statistics: DependencyAnalysisStatistics,
}

impl DependencyAnalysis {
    /// Analyzes an existing canonical compilation dependency context.
    ///
    /// This operation:
    ///
    /// 1. Validates the graph.
    /// 2. Obtains dependency-first compilation ordering.
    /// 3. Calculates relationship statistics.
    /// 4. Preserves non-ordering relationships.
    ///
    /// It does not perform source-level dependency discovery.
    ///
    /// Semantic analysis and module resolution must have already supplied
    /// the appropriate compilation-unit relationships.
    pub fn analyze(
        context: &DependencyContext,
    ) -> Result<Self, DependencyAnalysisError> {
        context.validate()?;

        let compilation_order = context.dependency_first_order()?;

        let mut statistics = DependencyAnalysisStatistics {
            compilation_units: context.unit_count(),
            ..DependencyAnalysisStatistics::default()
        };

        for edge in context.edges() {
            statistics.relationships = statistics
                .relationships
                .checked_add(1)
                .ok_or(DependencyAnalysisError::CountOverflow {
                    counter: "relationships",
                })?;

            if edge.requires_ordering() {
                statistics.ordering_relationships = statistics
                    .ordering_relationships
                    .checked_add(1)
                    .ok_or(DependencyAnalysisError::CountOverflow {
                        counter: "ordering_relationships",
                    })?;
            } else {
                statistics.non_ordering_relationships = statistics
                    .non_ordering_relationships
                    .checked_add(1)
                    .ok_or(DependencyAnalysisError::CountOverflow {
                        counter: "non_ordering_relationships",
                    })?;
            }
        }

        for unit in context.units() {
            let dependencies = context
                .dependencies_of(unit.id())
                .ok_or_else(|| {
                    DependencyAnalysisError::UnknownCompilationUnit {
                        unit: unit.id().clone(),
                    }
                })?;

            if dependencies.is_empty() {
                statistics.independent_units = statistics
                    .independent_units
                    .checked_add(1)
                    .ok_or(DependencyAnalysisError::CountOverflow {
                        counter: "independent_units",
                    })?;
            }
        }

        Ok(Self {
            context: context.clone(),
            compilation_order,
            statistics,
        })
    }

    /// Returns the canonical dependency context.
    ///
    /// The returned context is immutable.
    #[must_use]
    pub fn context(&self) -> &DependencyContext {
        &self.context
    }

    /// Returns the dependency-first compilation order.
    ///
    /// Every ordering prerequisite appears before its dependent unit.
    ///
    /// Non-ordering relationships do not impose ordering.
    #[must_use]
    pub fn compilation_order(&self) -> &[CompilationUnitId] {
        &self.compilation_order
    }

    /// Returns dependency-analysis statistics.
    #[must_use]
    pub const fn statistics(&self) -> &DependencyAnalysisStatistics {
        &self.statistics
    }

    /// Returns the number of compilation units.
    #[must_use]
    pub const fn unit_count(&self) -> usize {
        self.statistics.compilation_units
    }

    /// Returns the number of typed dependency relationships.
    #[must_use]
    pub const fn relationship_count(&self) -> usize {
        self.statistics.relationships
    }

    /// Returns whether the compilation graph is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.statistics.compilation_units == 0
    }

    /// Returns the direct dependencies of a compilation unit.
    ///
    /// All relationship kinds are included.
    ///
    /// The result is deterministic.
    pub fn direct_dependencies(
        &self,
        unit: &CompilationUnitId,
    ) -> Result<Vec<CompilationUnitId>, DependencyAnalysisError> {
        self.context
            .dependencies_of(unit)
            .ok_or_else(|| {
                DependencyAnalysisError::UnknownCompilationUnit {
                    unit: unit.clone(),
                }
            })
    }

    /// Returns the direct dependents of a compilation unit.
    ///
    /// All relationship kinds are included.
    pub fn direct_dependents(
        &self,
        unit: &CompilationUnitId,
    ) -> Result<Vec<CompilationUnitId>, DependencyAnalysisError> {
        self.context
            .dependents_of(unit)
            .ok_or_else(|| {
                DependencyAnalysisError::UnknownCompilationUnit {
                    unit: unit.clone(),
                }
            })
    }

    /// Returns direct compilation-order prerequisites.
    ///
    /// Non-ordering relationships are excluded.
    pub fn direct_ordering_dependencies(
        &self,
        unit: &CompilationUnitId,
    ) -> Result<Vec<CompilationUnitId>, DependencyAnalysisError> {
        self.context
            .ordering_dependencies_of(unit)
            .ok_or_else(|| {
                DependencyAnalysisError::UnknownCompilationUnit {
                    unit: unit.clone(),
                }
            })
    }

    /// Returns direct compilation-order dependents.
    ///
    /// Non-ordering relationships are excluded.
    pub fn direct_ordering_dependents(
        &self,
        unit: &CompilationUnitId,
    ) -> Result<Vec<CompilationUnitId>, DependencyAnalysisError> {
        self.context
            .ordering_dependents_of(unit)
            .ok_or_else(|| {
                DependencyAnalysisError::UnknownCompilationUnit {
                    unit: unit.clone(),
                }
            })
    }

    /// Returns every transitive compilation-order dependency.
    ///
    /// The result is returned in the canonical dependency-first order.
    ///
    /// Traversal is iterative and does not consume call-stack space
    /// proportional to graph depth.
    ///
    /// Non-ordering relationships are excluded.
    pub fn transitive_dependencies(
        &self,
        unit: &CompilationUnitId,
    ) -> Result<Vec<CompilationUnitId>, DependencyAnalysisError> {
        self.ensure_unit_exists(unit)?;

        let mut discovered = BTreeSet::new();
        let mut pending = vec![unit.clone()];

        while let Some(current) = pending.pop() {
            let dependencies = self
                .context
                .ordering_dependencies_of(&current)
                .ok_or_else(|| {
                    DependencyAnalysisError::UnknownCompilationUnit {
                        unit: current.clone(),
                    }
                })?;

            for dependency in dependencies {
                if dependency != *unit && discovered.insert(dependency.clone()) {
                    pending.push(dependency);
                }
            }
        }

        Ok(self
            .compilation_order
            .iter()
            .filter(|candidate| discovered.contains(*candidate))
            .cloned()
            .collect())
    }

    /// Returns every transitive compilation-order dependent.
    ///
    /// The result is deterministic and follows dependency-first
    /// compilation ordering.
    ///
    /// This operation is useful for incremental invalidation planning.
    ///
    /// It does not itself mark any unit dirty or modify incremental state.
    pub fn transitive_dependents(
        &self,
        unit: &CompilationUnitId,
    ) -> Result<Vec<CompilationUnitId>, DependencyAnalysisError> {
        self.ensure_unit_exists(unit)?;

        let mut discovered = BTreeSet::new();
        let mut pending = vec![unit.clone()];

        while let Some(current) = pending.pop() {
            let dependents = self
                .context
                .ordering_dependents_of(&current)
                .ok_or_else(|| {
                    DependencyAnalysisError::UnknownCompilationUnit {
                        unit: current.clone(),
                    }
                })?;

            for dependent in dependents {
                if dependent != *unit && discovered.insert(dependent.clone()) {
                    pending.push(dependent);
                }
            }
        }

        Ok(self
            .compilation_order
            .iter()
            .filter(|candidate| discovered.contains(*candidate))
            .cloned()
            .collect())
    }

    /// Checks whether `dependent` transitively requires `dependency`
    /// for compilation ordering.
    pub fn depends_on(
        &self,
        dependent: &CompilationUnitId,
        dependency: &CompilationUnitId,
    ) -> Result<bool, DependencyAnalysisError> {
        self.ensure_unit_exists(dependent)?;
        self.ensure_unit_exists(dependency)?;

        if dependent == dependency {
            return Ok(false);
        }

        let dependencies = self.transitive_dependencies(dependent)?;

        Ok(dependencies.iter().any(|candidate| candidate == dependency))
    }

    /// Checks whether `dependent` directly requires `dependency`
    /// for compilation ordering.
    pub fn directly_depends_on(
        &self,
        dependent: &CompilationUnitId,
        dependency: &CompilationUnitId,
    ) -> Result<bool, DependencyAnalysisError> {
        self.ensure_unit_exists(dependent)?;
        self.ensure_unit_exists(dependency)?;

        let dependencies = self.direct_ordering_dependencies(dependent)?;

        Ok(dependencies.iter().any(|candidate| candidate == dependency))
    }

    /// Revalidates the retained immutable context.
    ///
    /// This is useful at integration boundaries where a downstream
    /// subsystem explicitly requires a fresh validation.
    pub fn validate(&self) -> Result<(), DependencyAnalysisError> {
        self.context.validate()?;
        Ok(())
    }

    fn ensure_unit_exists(
        &self,
        unit: &CompilationUnitId,
    ) -> Result<(), DependencyAnalysisError> {
        if !self.context.contains_unit(unit) {
            return Err(
                DependencyAnalysisError::UnknownCompilationUnit {
                    unit: unit.clone(),
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::context::{
        CompilationUnit,
        CompilationUnitKind,
        DependencyContextBuilder,
        DependencyKind,
    };

    fn unit_id(value: &str) -> CompilationUnitId {
        CompilationUnitId::new(value)
            .expect("test compilation-unit identifier must be valid")
    }

    fn compilation_unit(value: &str) -> CompilationUnit {
        CompilationUnit::new(
            unit_id(value),
            CompilationUnitKind::new("source")
                .expect("test compilation-unit kind must be valid"),
            value,
        )
        .expect("test compilation unit must be valid")
    }

    fn builder_with_units(
        identifiers: &[&str],
    ) -> DependencyContextBuilder {
        let mut builder = DependencyContextBuilder::new();

        for identifier in identifiers {
            builder
                .add_unit(compilation_unit(identifier))
                .expect("test units must be unique");
        }

        builder
    }

    fn analyze(
        builder: DependencyContextBuilder,
    ) -> DependencyAnalysis {
        let context = builder
            .build()
            .expect("test dependency context must be valid");

        DependencyAnalysis::analyze(&context)
            .expect("test dependency analysis must succeed")
    }

    #[test]
    fn empty_context_is_valid() {
        let analysis = analyze(DependencyContextBuilder::new());

        assert!(analysis.is_empty());
        assert_eq!(analysis.unit_count(), 0);
        assert!(analysis.compilation_order().is_empty());
    }

    #[test]
    fn independent_units_are_ordered_deterministically() {
        let builder = builder_with_units(&["z", "a", "m"]);

        let analysis = analyze(builder);

        assert_eq!(
            analysis.compilation_order(),
            &[unit_id("a"), unit_id("m"), unit_id("z")]
        );
    }

    #[test]
    fn dependencies_are_processed_before_dependents() {
        let mut builder = builder_with_units(&[
            "application",
            "library",
            "foundation",
        ]);

        builder
            .add_dependency(
                unit_id("application"),
                unit_id("library"),
                DependencyKind::ModuleImport,
            )
            .expect("dependency must be valid");

        builder
            .add_dependency(
                unit_id("library"),
                unit_id("foundation"),
                DependencyKind::ModuleImport,
            )
            .expect("dependency must be valid");

        let analysis = analyze(builder);

        assert_eq!(
            analysis.compilation_order(),
            &[
                unit_id("foundation"),
                unit_id("library"),
                unit_id("application"),
            ]
        );
    }

    #[test]
    fn direct_dependencies_are_reported() {
        let mut builder =
            builder_with_units(&["application", "library"]);

        builder
            .add_dependency(
                unit_id("application"),
                unit_id("library"),
                DependencyKind::ModuleImport,
            )
            .expect("dependency must be valid");

        let analysis = analyze(builder);

        assert_eq!(
            analysis
                .direct_dependencies(&unit_id("application"))
                .expect("unit must exist"),
            vec![unit_id("library")]
        );
    }

    #[test]
    fn transitive_dependencies_are_reported() {
        let mut builder = builder_with_units(&["a", "b", "c", "d"]);

        builder
            .add_dependency(
                unit_id("a"),
                unit_id("b"),
                DependencyKind::ModuleImport,
            )
            .expect("dependency must be valid");

        builder
            .add_dependency(
                unit_id("b"),
                unit_id("c"),
                DependencyKind::ModuleImport,
            )
            .expect("dependency must be valid");

        builder
            .add_dependency(
                unit_id("c"),
                unit_id("d"),
                DependencyKind::ModuleImport,
            )
            .expect("dependency must be valid");

        let analysis = analyze(builder);

        assert_eq!(
            analysis
                .transitive_dependencies(&unit_id("a"))
                .expect("transitive analysis must succeed"),
            vec![unit_id("d"), unit_id("c"), unit_id("b")]
        );
    }

    #[test]
    fn transitive_dependents_are_reported() {
        let mut builder = builder_with_units(&["a", "b", "c"]);

        builder
            .add_dependency(
                unit_id("b"),
                unit_id("a"),
                DependencyKind::ModuleImport,
            )
            .expect("dependency must be valid");

        builder
            .add_dependency(
                unit_id("c"),
                unit_id("b"),
                DependencyKind::ModuleImport,
            )
            .expect("dependency must be valid");

        let analysis = analyze(builder);

        assert_eq!(
            analysis
                .transitive_dependents(&unit_id("a"))
                .expect("dependent analysis must succeed"),
            vec![unit_id("b"), unit_id("c")]
        );
    }

    #[test]
    fn non_ordering_relationships_do_not_create_compile_dependencies() {
        let mut builder = builder_with_units(&["a", "b"]);

        builder
            .add_dependency(
                unit_id("a"),
                unit_id("b"),
                DependencyKind::Feedback,
            )
            .expect("feedback relationship must be valid");

        let analysis = analyze(builder);

        assert_eq!(
            analysis
                .transitive_dependencies(&unit_id("a"))
                .expect("analysis must succeed"),
            Vec::<CompilationUnitId>::new()
        );

        assert_eq!(
            analysis.statistics().non_ordering_relationships,
            1
        );
    }

    #[test]
    fn hybrid_feedback_cycles_do_not_impose_false_compile_cycles() {
        let mut builder = builder_with_units(&["classical", "quantum"]);

        builder
            .add_dependency(
                unit_id("classical"),
                unit_id("quantum"),
                DependencyKind::Feedback,
            )
            .expect("feedback relationship must be valid");

        builder
            .add_dependency(
                unit_id("quantum"),
                unit_id("classical"),
                DependencyKind::Feedback,
            )
            .expect("feedback relationship must be valid");

        let analysis = analyze(builder);

        assert_eq!(analysis.unit_count(), 2);
        assert_eq!(analysis.relationship_count(), 2);
        assert_eq!(
            analysis.statistics().ordering_relationships,
            0
        );
    }

    #[test]
    fn statistics_distinguish_relationship_types() {
        let mut builder = builder_with_units(&["a", "b", "c"]);

        builder
            .add_dependency(
                unit_id("a"),
                unit_id("b"),
                DependencyKind::ModuleImport,
            )
            .expect("dependency must be valid");

        builder
            .add_dependency(
                unit_id("a"),
                unit_id("c"),
                DependencyKind::DataFlow,
            )
            .expect("relationship must be valid");

        let analysis = analyze(builder);

        assert_eq!(
            analysis.statistics(),
            &DependencyAnalysisStatistics {
                compilation_units: 3,
                relationships: 2,
                ordering_relationships: 1,
                non_ordering_relationships: 1,
                independent_units: 2,
            }
        );
    }

    #[test]
    fn transitive_dependency_queries_are_reflexive_free() {
        let mut builder = builder_with_units(&["a", "b"]);

        builder
            .add_dependency(
                unit_id("a"),
                unit_id("b"),
                DependencyKind::ModuleImport,
            )
            .expect("dependency must be valid");

        let analysis = analyze(builder);

        let dependencies = analysis
            .transitive_dependencies(&unit_id("a"))
            .expect("analysis must succeed");

        assert!(!dependencies.contains(&unit_id("a")));
    }

    #[test]
    fn unknown_units_are_rejected() {
        let analysis = analyze(builder_with_units(&["a"]));

        let result =
            analysis.transitive_dependencies(&unit_id("missing"));

        assert!(matches!(
            result,
            Err(DependencyAnalysisError::UnknownCompilationUnit { .. })
        ));
    }

    #[test]
    fn dependency_queries_are_deterministic() {
        let mut builder =
            builder_with_units(&["application", "a", "b", "c"]);

        for dependency in ["c", "a", "b"] {
            builder
                .add_dependency(
                    unit_id("application"),
                    unit_id(dependency),
                    DependencyKind::BuildInput,
                )
                .expect("dependency must be valid");
        }

        let analysis = analyze(builder);

        let first = analysis
            .transitive_dependencies(&unit_id("application"))
            .expect("analysis must succeed");

        let second = analysis
            .transitive_dependencies(&unit_id("application"))
            .expect("analysis must succeed");

        assert_eq!(first, second);
    }
}
