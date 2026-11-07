
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Compiler — Dependency Analysis.
//!
//! Path: src/compiler/analysis/dependency.rs
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//!
//! # Purpose
//!
//! Provides compiler-facing analysis of compilation-unit dependencies.
//!
//! This module consumes the authoritative immutable DependencyContext
//! owned by compiler/context/dependency_context.rs.
//!
//! It does not create another dependency graph.
//!
//! # Responsibilities
//!
//! - Validate canonical dependency contexts.
//! - Obtain deterministic dependency-first compilation ordering.
//! - Preserve typed dependency relationships.
//! - Distinguish ordering from non-ordering relationships.
//! - Calculate dependency statistics.
//! - Query direct dependencies and dependents.
//! - Query transitive dependencies and dependents.
//! - Determine dependency reachability.
//! - Expose dependency information to other compiler subsystems.
//! - Provide structured error reporting.
//!
//! # Architectural boundaries
//!
//! Dependency discovery belongs to:
//!
//!     src/frontend/semantic/
//!
//! Compilation-unit graph storage belongs to:
//!
//!     src/compiler/context/dependency_context.rs
//!
//! Incremental fingerprints and cache invalidation belong to:
//!
//!     src/compiler/incremental/
//!
//! Parallel execution belongs to:
//!
//!     src/compiler/parallel/
//!
//! Program representation belongs to:
//!
//!     src/zuir/
//!
//! Physical resource discovery, allocation, scheduling, routing,
//! backend selection and execution belong to their respective
//! authoritative subsystems.
//!
//! This module does not implement those responsibilities.
//!
//! # Grammar boundary
//!
//! Source syntax is governed by:
//!
//!     grammar/Zamani.g4
//!     grammar/modules/imports.g4
//!     grammar/declarations/domains.g4
//!
//! Semantic meaning is governed by:
//!
//!     grammar/specification/semantic-model.md
//!
//! Scalability is governed by:
//!
//!     grammar/specification/scalability-model.md
//!
//! POCO-REAF is governed by:
//!
//!     grammar/specification/poco-reaf.md
//!
//! Grammar productions are not dependency graph implementations.
//!
//! The frontend resolves source-level declarations and supplies
//! established relationships to the compiler.
//!
//! # Edge direction
//!
//!     dependent -> dependency
//!
//! An edge:
//!
//!     application -> library
//!
//! means that application requires library.
//!
//! Consequently, library must be processed first.
//!
//! # Ordering semantics
//!
//! Only relationships for which requires_ordering() returns true
//! participate in compilation ordering.
//!
//! Data flow, control flow, feedback, synchronization and completion
//! relationships remain available for analysis but do not
//! automatically impose compilation ordering.
//!
//! # Scalability
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Compilation units.
//! - Dependency relationships.
//! - Modules.
//! - Functions.
//! - Computational domains.
//! - Classical processors.
//! - GPUs.
//! - QPUs.
//! - FPGAs.
//! - Distributed participants.
//! - Tensor dimensions.
//! - IR operations.
//!
//! Traversals are iterative.
//!
//! Actual processing is bounded by available memory, processing
//! capacity, representation limits and explicitly configured
//! operational budgets.
//!
//! Literal infinite physical resources are not promised.
//!
//! # Determinism
//!
//! Deterministic behavior is inherited from the ordered canonical
//! DependencyContext.
//!
//! No randomized graph iteration or machine-specific identity
//! ordering is introduced.
//!
//! # Concurrency
//!
//! DependencyContext is immutable and internally shared.
//!
//! DependencyAnalysis is immutable after construction.
//!
//! Concurrent consumers can share an analysis result.
//!
//! This module introduces no global mutable state or worker threads.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::compiler::context::{
    CompilationUnitId,
    DependencyContext,
    DependencyGraphError,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of compiler dependency analysis.
///
/// Independent of the language, grammar, semantic-model and IR versions.
pub const DEPENDENCY_ANALYSIS_API_VERSION: u16 = 1;

// ============================================================================
// RESULT
// ============================================================================

/// Result type for dependency analysis.
pub type DependencyAnalysisResult<T> =
    Result<T, DependencyAnalysisError>;

// ============================================================================
// STATISTICS
// ============================================================================

/// Statistics describing a canonical compilation dependency context.
///
/// Relationship counts describe distinct typed edges.
///
/// Multiple relationship kinds between the same two units count
/// as separate relationships.
///
/// `independent_units` retains its established API meaning:
/// units having no direct prerequisites of any relationship kind.
///
/// It does not mean that the units have no incoming relationships.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DependencyAnalysisStatistics {
    /// Number of registered compilation units.
    pub compilation_units: usize,

    /// Number of unique typed relationships.
    pub relationships: usize,

    /// Number of relationships imposing compilation ordering.
    pub ordering_relationships: usize,

    /// Number of relationships not imposing compilation ordering.
    pub non_ordering_relationships: usize,

    /// Number of units with no direct dependencies.
    ///
    /// These are prerequisite-free units in the dependent-to-dependency
    /// graph orientation.
    pub independent_units: usize,
}

impl DependencyAnalysisStatistics {
    /// Returns whether the graph contains no relationships.
    #[must_use]
    pub const fn has_no_relationships(&self) -> bool {
        self.relationships == 0
    }

    /// Returns whether all relationships impose compilation ordering.
    #[must_use]
    pub const fn all_relationships_are_ordering(&self) -> bool {
        self.non_ordering_relationships == 0
    }

    /// Returns whether at least one non-ordering relationship exists.
    #[must_use]
    pub const fn has_non_ordering_relationships(&self) -> bool {
        self.non_ordering_relationships != 0
    }

    /// Returns whether the graph contains any compilation units.
    #[must_use]
    pub const fn has_units(&self) -> bool {
        self.compilation_units != 0
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by compiler dependency analysis.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DependencyAnalysisError {
    /// The canonical dependency graph failed validation.
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
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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
                    "unknown compilation unit `{unit}`"
                )
            }

            Self::CountOverflow { counter } => {
                write!(
                    formatter,
                    "dependency analysis counter overflow: {counter}"
                )
            }
        }
    }
}

impl Error for DependencyAnalysisError {
    fn source(
        &self,
    ) -> Option<&(dyn Error + 'static)> {
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
// ANALYSIS
// ============================================================================

/// Immutable compiler-facing dependency-analysis result.
///
/// The authoritative graph remains owned by DependencyContext.
///
/// Cloning this structure shares the underlying canonical context
/// rather than constructing another dependency graph.
#[derive(Clone, Debug)]
pub struct DependencyAnalysis {
    context: DependencyContext,

    /// Deterministic dependency-first compilation ordering.
    compilation_order: Vec<CompilationUnitId>,

    /// Validated dependency statistics.
    statistics: DependencyAnalysisStatistics,
}

impl DependencyAnalysis {
    /// Analyze an existing canonical dependency context.
    ///
    /// The operation:
    ///
    /// 1. Validates the canonical context.
    /// 2. Obtains dependency-first compilation ordering.
    /// 3. Counts typed relationships.
    /// 4. Separates ordering and non-ordering relationships.
    /// 5. Calculates prerequisite-free units.
    ///
    /// Source-level dependency discovery is deliberately excluded.
    ///
    /// # Errors
    ///
    /// Returns the canonical graph validation error or a statistics
    /// representation error.
    pub fn analyze(
        context: &DependencyContext,
    ) -> DependencyAnalysisResult<Self> {
        context.validate()?;

        let compilation_order =
            context.dependency_first_order()?;

        let mut statistics =
            DependencyAnalysisStatistics {
                compilation_units: context.unit_count(),
                relationships: context.edge_count(),
                ..DependencyAnalysisStatistics::default()
            };

        for edge in context.edges() {
            if edge.requires_ordering() {
                statistics.ordering_relationships =
                    statistics
                        .ordering_relationships
                        .checked_add(1)
                        .ok_or(
                            DependencyAnalysisError::CountOverflow {
                                counter: "ordering_relationships",
                            },
                        )?;
            } else {
                statistics.non_ordering_relationships =
                    statistics
                        .non_ordering_relationships
                        .checked_add(1)
                        .ok_or(
                            DependencyAnalysisError::CountOverflow {
                                counter: "non_ordering_relationships",
                            },
                        )?;
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
                statistics.independent_units =
                    statistics
                        .independent_units
                        .checked_add(1)
                        .ok_or(
                            DependencyAnalysisError::CountOverflow {
                                counter: "independent_units",
                            },
                        )?;
            }
        }

        Ok(Self {
            context: context.clone(),
            compilation_order,
            statistics,
        })
    }

    // ========================================================================
    // BASIC ACCESSORS
    // ========================================================================

    /// Return the authoritative immutable dependency context.
    #[must_use]
    pub fn context(&self) -> &DependencyContext {
        &self.context
    }

    /// Return dependency-first compilation ordering.
    ///
    /// Every ordering prerequisite appears before its dependent.
    ///
    /// Non-ordering relationships do not determine this ordering.
    #[must_use]
    pub fn compilation_order(
        &self,
    ) -> &[CompilationUnitId] {
        &self.compilation_order
    }

    /// Return dependency analysis statistics.
    #[must_use]
    pub const fn statistics(
        &self,
    ) -> &DependencyAnalysisStatistics {
        &self.statistics
    }

    /// Return the number of compilation units.
    #[must_use]
    pub const fn unit_count(&self) -> usize {
        self.statistics.compilation_units
    }

    /// Return the number of typed relationships.
    #[must_use]
    pub const fn relationship_count(&self) -> usize {
        self.statistics.relationships
    }

    /// Return whether the graph contains no units.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.statistics.compilation_units == 0
    }

    /// Return whether a compilation unit exists.
    #[must_use]
    pub fn contains_unit(
        &self,
        unit: &CompilationUnitId,
    ) -> bool {
        self.context.contains_unit(unit)
    }

    // ========================================================================
    // DIRECT DEPENDENCY QUERIES
    // ========================================================================

    /// Return direct dependencies of a unit.
    ///
    /// All relationship kinds are considered.
    ///
    /// The returned unit identities are unique and deterministic.
    pub fn direct_dependencies(
        &self,
        unit: &CompilationUnitId,
    ) -> DependencyAnalysisResult<Vec<CompilationUnitId>> {
        self.context
            .dependencies_of(unit)
            .ok_or_else(|| {
                DependencyAnalysisError::UnknownCompilationUnit {
                    unit: unit.clone(),
                }
            })
    }

    /// Return direct dependents of a unit.
    ///
    /// All relationship kinds are considered.
    pub fn direct_dependents(
        &self,
        unit: &CompilationUnitId,
    ) -> DependencyAnalysisResult<Vec<CompilationUnitId>> {
        self.context
            .dependents_of(unit)
            .ok_or_else(|| {
                DependencyAnalysisError::UnknownCompilationUnit {
                    unit: unit.clone(),
                }
            })
    }

    /// Return direct compilation-order dependencies.
    ///
    /// Non-ordering relationships are excluded.
    pub fn direct_ordering_dependencies(
        &self,
        unit: &CompilationUnitId,
    ) -> DependencyAnalysisResult<Vec<CompilationUnitId>> {
        self.context
            .ordering_dependencies_of(unit)
            .ok_or_else(|| {
                DependencyAnalysisError::UnknownCompilationUnit {
                    unit: unit.clone(),
                }
            })
    }

    /// Return direct compilation-order dependents.
    ///
    /// Non-ordering relationships are excluded.
    pub fn direct_ordering_dependents(
        &self,
        unit: &CompilationUnitId,
    ) -> DependencyAnalysisResult<Vec<CompilationUnitId>> {
        self.context
            .ordering_dependents_of(unit)
            .ok_or_else(|| {
                DependencyAnalysisError::UnknownCompilationUnit {
                    unit: unit.clone(),
                }
            })
    }

    /// Return all typed relationship kinds between two units.
    ///
    /// The canonical context remains responsible for relationship storage.
    #[must_use]
    pub fn relationship_kinds_between(
        &self,
        dependent: &CompilationUnitId,
        dependency: &CompilationUnitId,
    ) -> Vec<crate::compiler::context::DependencyKind> {
        self.context
            .relationships_between(dependent, dependency)
    }

    // ========================================================================
    // TRANSITIVE DEPENDENCY QUERIES
    // ========================================================================

    /// Return every transitive compilation-order dependency.
    ///
    /// Results follow dependency-first compilation ordering.
    ///
    /// The starting unit is excluded.
    ///
    /// Traversal is iterative and cycle-safe.
    ///
    /// Non-ordering relationships are excluded.
    pub fn transitive_dependencies(
        &self,
        unit: &CompilationUnitId,
    ) -> DependencyAnalysisResult<Vec<CompilationUnitId>> {
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
                if dependency != *unit
                    && discovered.insert(dependency.clone())
                {
                    pending.push(dependency);
                }
            }
        }

        Ok(
            self.compilation_order
                .iter()
                .filter(|candidate| discovered.contains(*candidate))
                .cloned()
                .collect()
        )
    }

    /// Return every transitive compilation-order dependent.
    ///
    /// Results follow dependency-first compilation ordering.
    ///
    /// This is useful for incremental invalidation and rebuilding.
    ///
    /// This operation does not mutate incremental state.
    pub fn transitive_dependents(
        &self,
        unit: &CompilationUnitId,
    ) -> DependencyAnalysisResult<Vec<CompilationUnitId>> {
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
                if dependent != *unit
                    && discovered.insert(dependent.clone())
                {
                    pending.push(dependent);
                }
            }
        }

        Ok(
            self.compilation_order
                .iter()
                .filter(|candidate| discovered.contains(*candidate))
                .cloned()
                .collect()
        )
    }

    /// Return every transitive dependency across all relationship kinds.
    ///
    /// Unlike transitive_dependencies(), this follows data-flow,
    /// control-flow, synchronization and other non-ordering edges too.
    ///
    /// This is an analytical relationship query, not a compile-order
    /// query.
    pub fn all_transitive_dependencies(
        &self,
        unit: &CompilationUnitId,
    ) -> DependencyAnalysisResult<Vec<CompilationUnitId>> {
        self.ensure_unit_exists(unit)?;

        let mut discovered = BTreeSet::new();
        let mut pending = vec![unit.clone()];

        while let Some(current) = pending.pop() {
            let dependencies = self
                .context
                .dependencies_of(&current)
                .ok_or_else(|| {
                    DependencyAnalysisError::UnknownCompilationUnit {
                        unit: current.clone(),
                    }
                })?;

            for dependency in dependencies {
                if dependency != *unit
                    && discovered.insert(dependency.clone())
                {
                    pending.push(dependency);
                }
            }
        }

        Ok(discovered.into_iter().collect())
    }

    /// Return every transitive dependent across all relationship kinds.
    ///
    /// This is an analytical relationship query, not an incremental
    /// compilation invalidation decision.
    pub fn all_transitive_dependents(
        &self,
        unit: &CompilationUnitId,
    ) -> DependencyAnalysisResult<Vec<CompilationUnitId>> {
        self.ensure_unit_exists(unit)?;

        let mut discovered = BTreeSet::new();
        let mut pending = vec![unit.clone()];

        while let Some(current) = pending.pop() {
            let dependents = self
                .context
                .dependents_of(&current)
                .ok_or_else(|| {
                    DependencyAnalysisError::UnknownCompilationUnit {
                        unit: current.clone(),
                    }
                })?;

            for dependent in dependents {
                if dependent != *unit
                    && discovered.insert(dependent.clone())
                {
                    pending.push(dependent);
                }
            }
        }

        Ok(discovered.into_iter().collect())
    }

    // ========================================================================
    // REACHABILITY
    // ========================================================================

    /// Check whether a dependent transitively requires a dependency
    /// for compilation ordering.
    ///
    /// This performs an early-exit iterative traversal rather than
    /// constructing the complete transitive dependency result.
    pub fn depends_on(
        &self,
        dependent: &CompilationUnitId,
        dependency: &CompilationUnitId,
    ) -> DependencyAnalysisResult<bool> {
        self.ensure_unit_exists(dependent)?;
        self.ensure_unit_exists(dependency)?;

        if dependent == dependency {
            return Ok(false);
        }

        let mut visited = BTreeSet::new();
        let mut pending = vec![dependent.clone()];

        while let Some(current) = pending.pop() {
            let dependencies = self
                .context
                .ordering_dependencies_of(&current)
                .ok_or_else(|| {
                    DependencyAnalysisError::UnknownCompilationUnit {
                        unit: current.clone(),
                    }
                })?;

            for candidate in dependencies {
                if candidate == *dependency {
                    return Ok(true);
                }

                if visited.insert(candidate.clone()) {
                    pending.push(candidate);
                }
            }
        }

        Ok(false)
    }

    /// Check whether a unit directly requires another unit
    /// for compilation ordering.
    pub fn directly_depends_on(
        &self,
        dependent: &CompilationUnitId,
        dependency: &CompilationUnitId,
    ) -> DependencyAnalysisResult<bool> {
        self.ensure_unit_exists(dependent)?;
        self.ensure_unit_exists(dependency)?;

        if dependent == dependency {
            return Ok(false);
        }

        let dependencies =
            self.direct_ordering_dependencies(dependent)?;

        Ok(dependencies.contains(dependency))
    }

    /// Check whether any typed relationship exists between two units.
    #[must_use]
    pub fn has_relationship(
        &self,
        dependent: &CompilationUnitId,
        dependency: &CompilationUnitId,
    ) -> bool {
        !self
            .relationship_kinds_between(dependent, dependency)
            .is_empty()
    }

    // ========================================================================
    // VALIDATION
    // ========================================================================

    /// Revalidate the retained canonical dependency context.
    ///
    /// Useful at compiler subsystem integration boundaries.
    pub fn validate(&self) -> DependencyAnalysisResult<()> {
        self.context.validate()?;
        Ok(())
    }

    /// Ensure that a requested compilation unit exists.
    fn ensure_unit_exists(
        &self,
        unit: &CompilationUnitId,
    ) -> DependencyAnalysisResult<()> {
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
            .expect("valid test unit identifier")
    }

    fn compilation_unit(value: &str) -> CompilationUnit {
        CompilationUnit::new(
            unit_id(value),
            CompilationUnitKind::new("source")
                .expect("valid test unit kind"),
            value,
        )
        .expect("valid compilation unit")
    }

    fn builder_with_units(
        identifiers: &[&str],
    ) -> DependencyContextBuilder {
        let mut builder = DependencyContextBuilder::new();

        for identifier in identifiers {
            builder
                .add_unit(compilation_unit(identifier))
                .expect("unique test unit");
        }

        builder
    }

    fn add_dependency(
        builder: &mut DependencyContextBuilder,
        dependent: &str,
        dependency: &str,
        kind: DependencyKind,
    ) {
        builder
            .add_dependency(
                unit_id(dependent),
                unit_id(dependency),
                kind,
            )
            .expect("valid test relationship");
    }

    fn analyze(
        builder: DependencyContextBuilder,
    ) -> DependencyAnalysis {
        let context = builder
            .build()
            .expect("valid test dependency context");

        DependencyAnalysis::analyze(&context)
            .expect("valid dependency analysis")
    }

    #[test]
    fn empty_context_is_valid() {
        let analysis =
            analyze(DependencyContextBuilder::new());

        assert!(analysis.is_empty());
        assert_eq!(analysis.unit_count(), 0);
        assert!(analysis.compilation_order().is_empty());
    }

    #[test]
    fn independent_units_are_deterministically_ordered() {
        let analysis =
            analyze(builder_with_units(&["z", "a", "m"]));

        assert_eq!(
            analysis.compilation_order(),
            &[unit_id("a"), unit_id("m"), unit_id("z")]
        );
    }

    #[test]
    fn dependencies_precede_dependents() {
        let mut builder = builder_with_units(&[
            "application",
            "library",
            "foundation",
        ]);

        add_dependency(
            &mut builder,
            "application",
            "library",
            DependencyKind::ModuleImport,
        );

        add_dependency(
            &mut builder,
            "library",
            "foundation",
            DependencyKind::ModuleImport,
        );

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
    fn direct_dependencies_are_available() {
        let mut builder =
            builder_with_units(&["application", "library"]);

        add_dependency(
            &mut builder,
            "application",
            "library",
            DependencyKind::ModuleImport,
        );

        let analysis = analyze(builder);

        assert_eq!(
            analysis.direct_dependencies(&unit_id("application"))
                .expect("known unit"),
            vec![unit_id("library")]
        );
    }

    #[test]
    fn transitive_dependencies_are_available() {
        let mut builder =
            builder_with_units(&["a", "b", "c", "d"]);

        add_dependency(
            &mut builder, "a", "b", DependencyKind::ModuleImport,
        );

        add_dependency(
            &mut builder, "b", "c", DependencyKind::ModuleImport,
        );

        add_dependency(
            &mut builder, "c", "d", DependencyKind::ModuleImport,
        );

        let analysis = analyze(builder);

        assert_eq!(
            analysis.transitive_dependencies(&unit_id("a"))
                .expect("known unit"),
            vec![unit_id("d"), unit_id("c"), unit_id("b")]
        );
    }

    #[test]
    fn transitive_dependents_are_available() {
        let mut builder =
            builder_with_units(&["a", "b", "c"]);

        add_dependency(
            &mut builder, "b", "a", DependencyKind::ModuleImport,
        );

        add_dependency(
            &mut builder, "c", "b", DependencyKind::ModuleImport,
        );

        let analysis = analyze(builder);

        assert_eq!(
            analysis.transitive_dependents(&unit_id("a"))
                .expect("known unit"),
            vec![unit_id("b"), unit_id("c")]
        );
    }

    #[test]
    fn non_ordering_relationships_do_not_impose_compile_order() {
        let mut builder =
            builder_with_units(&["a", "b"]);

        add_dependency(
            &mut builder, "a", "b", DependencyKind::Feedback,
        );

        let analysis = analyze(builder);

        assert!(
            analysis.transitive_dependencies(&unit_id("a"))
                .expect("known unit")
                .is_empty()
        );

        assert_eq!(
            analysis.statistics().non_ordering_relationships,
            1
        );

        assert_eq!(
            analysis.all_transitive_dependencies(&unit_id("a"))
                .expect("known unit"),
            vec![unit_id("b")]
        );
    }

    #[test]
    fn hybrid_feedback_cycles_do_not_create_compile_cycles() {
        let mut builder =
            builder_with_units(&["classical", "quantum"]);

        add_dependency(
            &mut builder,
            "classical",
            "quantum",
            DependencyKind::Feedback,
        );

        add_dependency(
            &mut builder,
            "quantum",
            "classical",
            DependencyKind::Feedback,
        );

        let analysis = analyze(builder);

        assert_eq!(analysis.unit_count(), 2);
        assert_eq!(analysis.relationship_count(), 2);
        assert_eq!(
            analysis.statistics().ordering_relationships,
            0
        );

        assert_eq!(analysis.compilation_order().len(), 2);
    }

    #[test]
    fn statistics_distinguish_relationship_types() {
        let mut builder =
            builder_with_units(&["a", "b", "c"]);

        add_dependency(
            &mut builder,
            "a",
            "b",
            DependencyKind::ModuleImport,
        );

        add_dependency(
            &mut builder,
            "a",
            "c",
            DependencyKind::DataFlow,
        );

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
    fn transitive_dependencies_exclude_the_starting_unit() {
        let mut builder =
            builder_with_units(&["a", "b"]);

        add_dependency(
            &mut builder,
            "a",
            "b",
            DependencyKind::ModuleImport,
        );

        let analysis = analyze(builder);

        assert!(
            !analysis.transitive_dependencies(&unit_id("a"))
                .expect("known unit")
                .contains(&unit_id("a"))
        );
    }

    #[test]
    fn unknown_units_are_rejected() {
        let analysis =
            analyze(builder_with_units(&["a"]));

        assert!(matches!(
            analysis.transitive_dependencies(&unit_id("missing")),
            Err(DependencyAnalysisError::UnknownCompilationUnit { .. })
        ));
    }

    #[test]
    fn direct_and_transitive_reachability_are_correct() {
        let mut builder =
            builder_with_units(&["a", "b", "c"]);

        add_dependency(
            &mut builder, "a", "b", DependencyKind::ModuleImport,
        );

        add_dependency(
            &mut builder, "b", "c", DependencyKind::ModuleImport,
        );

        let analysis = analyze(builder);

        assert!(
            analysis.directly_depends_on(&unit_id("a"), &unit_id("b"))
                .expect("known units")
        );

        assert!(
            analysis.depends_on(&unit_id("a"), &unit_id("c"))
                .expect("known units")
        );

        assert!(
            !analysis.depends_on(&unit_id("c"), &unit_id("a"))
                .expect("known units")
        );
    }

    #[test]
    fn non_ordering_relationships_are_queryable() {
        let mut builder =
            builder_with_units(&["classical", "quantum"]);

        add_dependency(
            &mut builder,
            "classical",
            "quantum",
            DependencyKind::Synchronization,
        );

        let analysis = analyze(builder);

        assert!(
            analysis.has_relationship(
                &unit_id("classical"),
                &unit_id("quantum")
            )
        );

        assert!(
            !analysis.directly_depends_on(
                &unit_id("classical"),
                &unit_id("quantum")
            ).expect("known units")
        );
    }

    #[test]
    fn repeated_queries_are_deterministic() {
        let mut builder =
            builder_with_units(&["application", "a", "b", "c"]);

        for dependency in ["c", "a", "b"] {
            add_dependency(
                &mut builder,
                "application",
                dependency,
                DependencyKind::BuildInput,
            );
        }

        let analysis = analyze(builder);

        let first = analysis
            .transitive_dependencies(&unit_id("application"))
            .expect("first query");

        let second = analysis
            .transitive_dependencies(&unit_id("application"))
            .expect("second query");

        assert_eq!(first, second);
    }

    #[test]
    fn cloned_analysis_shares_the_same_logical_context() {
        let analysis =
            analyze(builder_with_units(&["a", "b"]));

        let cloned = analysis.clone();

        assert_eq!(
            analysis.compilation_order(),
            cloned.compilation_order()
        );

        assert_eq!(
            analysis.statistics(),
            cloned.statistics()
        );
    }

    #[test]
    fn analysis_validation_succeeds_for_valid_context() {
        let analysis =
            analyze(builder_with_units(&["a", "b"]));

        assert!(analysis.validate().is_ok());
    }
}
