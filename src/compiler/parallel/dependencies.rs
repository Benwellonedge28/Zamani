
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Parallel Dependency Planning.
//!
//! File:
//!     src/compiler/parallel/dependencies.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only. Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module adapts the canonical compiler dependency graph into
//! deterministic dependency-ready waves for parallel compilation.
//!
//! A dependency wave contains compilation units whose ordering dependencies
//! have already been satisfied by earlier waves.
//!
//! Units in the same wave have no ordering dependency on one another and
//! therefore may be considered concurrently by a downstream scheduler,
//! subject to other compilation constraints and resource budgets.
//!
//! This module does not execute compilation jobs.
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! Canonical dependency graph:
//!
//!     crate::compiler::context::dependency_context
//!
//! This module reuses:
//!
//!     CompilationUnitId
//!     DependencyContext
//!     DependencyGraphError
//!
//! The canonical context remains the sole owner of:
//!
//!     - Compilation-unit identity.
//!     - Compilation-unit metadata.
//!     - Dependency relationship types.
//!     - Ordering versus non-ordering relationship semantics.
//!     - Graph validation.
//!     - Dependency-first topological ordering.
//!     - Ordering-cycle detection.
//!
//! This module owns only:
//!
//!     - Dependency-ready wave construction.
//!     - Deterministic wave ordering.
//!     - Unit-to-wave lookup.
//!     - The immutable scheduler-facing dependency plan.
//!
//! This module does NOT own:
//!
//!     - Source parsing.
//!     - AST construction.
//!     - Module or symbol resolution.
//!     - Semantic dependency discovery.
//!     - Type inference.
//!     - Ownership analysis.
//!     - Incremental fingerprints.
//!     - Cache invalidation.
//!     - Worker creation.
//!     - Thread-pool execution.
//!     - Task execution.
//!     - Runtime scheduling.
//!     - Distributed scheduling.
//!     - Hardware discovery.
//!     - Physical resource allocation.
//!     - Backend selection.
//!     - IR definitions.
//!
//! ============================================================================
//! DEPENDENCY DIRECTION
//! ============================================================================
//!
//! Frontend and semantic analysis
//!             |
//!             v
//! DependencyContextBuilder
//!             |
//!             v
//!     DependencyContext
//!             |
//!             v
//!      DependencyPlan
//!             |
//!             v
//!   parallel::scheduler
//!             |
//!             v
//!    parallel::workers
//!             |
//!             v
//!      parallel_build
//!
//! Incremental compilation remains an independent authority.
//!
//! It may provide the set of compilation units requiring rebuilding, but
//! cache invalidation and dependency discovery do not belong here.
//!
//! ============================================================================
//! ORDERING SEMANTICS
//! ============================================================================
//!
//! The canonical graph uses this edge direction:
//!
//!     dependent -> dependency
//!
//! Example:
//!
//!     application -> library
//!
//! The library must be processed before the application.
//!
//! This module uses only relationships for which:
//!
//!     DependencyKind::requires_ordering() == true
//!
//! Data-flow, control-flow, feedback, completion and synchronization
//! relationships are retained in the canonical graph but do not
//! automatically impose compilation ordering.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! There are no hardcoded limits on:
//!
//!     - Compilation units.
//!     - Dependencies.
//!     - Dependents.
//!     - Dependency waves.
//!     - Computing domains.
//!     - CPUs.
//!     - Workers.
//!     - GPUs.
//!     - FPGAs.
//!     - QPUs.
//!     - Distributed nodes.
//!
//! The implementation uses host-sized collection indices and the existing
//! canonical graph representation.
//!
//! Actual memory availability and configured operational budgets remain
//! external constraints.
//!
//! This module does not promise infinite physical memory or execution
//! capacity.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! The canonical DependencyContext supplies deterministic dependency-first
//! ordering.
//!
//! Units are grouped by their earliest dependency-ready wave.
//!
//! Units within a wave preserve the deterministic order supplied by the
//! canonical dependency context.
//!
//! A downstream scheduler may execute units within one wave concurrently.
//! Completion order must not be confused with deterministic result order.
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! Required existing dependency:
//!
//!     crate::compiler::context::dependency_context
//!
//! Required public API:
//!
//!     CompilationUnitId
//!     DependencyContext
//!     DependencyGraphError
//!
//! Downstream consumers:
//!
//!     src/compiler/parallel/scheduler.rs
//!     src/compiler/parallel/workers.rs
//!     src/compiler/parallel_build.rs
//!
//! The scheduler consumes DependencyPlan::waves().
//!
//! The worker executor executes only jobs that the scheduler has declared
//! ready.
//!
//! parallel_build.rs must preserve its existing public compatibility API
//! while delegating dependency-aware execution to the parallel subsystem.
//!
//! No grammar modification is required for this implementation.
//!
//! ============================================================================

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::compiler::context::dependency_context::{
    CompilationUnitId,
    DependencyContext,
    DependencyGraphError,
};

/// Public API version of the parallel dependency-plan contract.
///
/// This version is independent of the Zamani language, grammar,
/// semantic-model, IR, backend and runtime versions.
pub const PARALLEL_DEPENDENCY_API_VERSION: u16 = 1;

// ============================================================================
// DEPENDENCY WAVE
// ============================================================================

/// A group of compilation units at the same dependency depth.
///
/// Every ordering dependency of every unit in this wave belongs to an
/// earlier wave.
///
/// Units in the same wave have no ordering dependency on one another.
///
/// The wave index is zero-based.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyWave {
    index: usize,
    units: Vec<CompilationUnitId>,
}

impl DependencyWave {
    /// Creates a dependency wave internally.
    fn new(
        index: usize,
        units: Vec<CompilationUnitId>,
    ) -> Self {
        Self {
            index,
            units,
        }
    }

    /// Returns this wave's zero-based index.
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Returns the compilation units in deterministic order.
    #[must_use]
    pub fn units(&self) -> &[CompilationUnitId] {
        &self.units
    }

    /// Returns the number of compilation units in this wave.
    #[must_use]
    pub fn len(&self) -> usize {
        self.units.len()
    }

    /// Returns whether this wave contains no compilation units.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }

    /// Returns whether this wave contains multiple independently
    /// ordering-ready compilation units.
    ///
    /// This indicates potential parallelism, not a guarantee that the
    /// available resource budget or other compilation constraints permit
    /// simultaneous execution.
    #[must_use]
    pub fn permits_parallel_execution(&self) -> bool {
        self.units.len() > 1
    }
}

// ============================================================================
// DEPENDENCY PLAN ERRORS
// ============================================================================

/// Errors produced while adapting a canonical dependency context.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DependencyPlanError {
    /// The canonical dependency graph is invalid.
    Graph(DependencyGraphError),

    /// A compilation unit was not present in the canonical context.
    UnknownUnit {
        unit: CompilationUnitId,
    },

    /// A dependency was not encountered before its dependent unit.
    ///
    /// This should not occur when the canonical dependency context returns
    /// a valid dependency-first ordering.
    DependencyNotScheduled {
        unit: CompilationUnitId,
        dependency: CompilationUnitId,
    },

    /// The dependency depth exceeded the host's index representation.
    ///
    /// This is a representation failure, not a universal language limit.
    WaveIndexOverflow {
        unit: CompilationUnitId,
    },
}

impl fmt::Display for DependencyPlanError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Graph(error) => {
                write!(
                    formatter,
                    "invalid compilation dependency graph: {error}"
                )
            }

            Self::UnknownUnit { unit } => {
                write!(
                    formatter,
                    "compilation unit '{unit}' is missing from the dependency context"
                )
            }

            Self::DependencyNotScheduled {
                unit,
                dependency,
            } => {
                write!(
                    formatter,
                    "dependency '{dependency}' was not scheduled before dependent '{unit}'"
                )
            }

            Self::WaveIndexOverflow { unit } => {
                write!(
                    formatter,
                    "dependency wave index overflow while planning '{unit}'"
                )
            }
        }
    }
}

impl Error for DependencyPlanError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Graph(error) => Some(error),
            _ => None,
        }
    }
}

impl From<DependencyGraphError> for DependencyPlanError {
    fn from(error: DependencyGraphError) -> Self {
        Self::Graph(error)
    }
}

// ============================================================================
// IMMUTABLE DEPENDENCY PLAN
// ============================================================================

/// Immutable dependency-ready plan for parallel compilation.
///
/// The plan retains a clone of the canonical DependencyContext. Cloning that
/// context shares its immutable graph through its existing Arc-backed
/// implementation.
///
/// This prevents the parallel subsystem from creating a second dependency
/// graph or losing relationship metadata.
///
/// `waves` contains dependency-ready groups.
///
/// `ordered_units` contains the complete deterministic dependency-first
/// ordering.
///
/// `unit_waves` provides constant-logarithmic lookup of a unit's wave index.
#[derive(Debug, Clone)]
pub struct DependencyPlan {
    context: DependencyContext,
    waves: Vec<DependencyWave>,
    ordered_units: Vec<CompilationUnitId>,
    unit_waves: BTreeMap<CompilationUnitId, usize>,
    edge_count: usize,
}

impl DependencyPlan {
    /// Builds a parallel dependency plan from the canonical dependency
    /// context.
    ///
    /// The canonical context is validated and supplies the authoritative
    /// dependency-first ordering.
    ///
    /// This method does not independently implement another graph
    /// topological-sort algorithm.
    pub fn from_context(
        context: &DependencyContext,
    ) -> Result<Self, DependencyPlanError> {
        // Reuse canonical graph validation and ordering.
        let ordered_units = context.dependency_first_order()?;

        let mut unit_waves =
            BTreeMap::<CompilationUnitId, usize>::new();

        let mut grouped =
            BTreeMap::<usize, Vec<CompilationUnitId>>::new();

        // Each unit's wave is one greater than the maximum wave of its
        // ordering dependencies. Units without ordering dependencies
        // belong to wave zero.
        for unit in &ordered_units {
            if !context.contains_unit(unit) {
                return Err(
                    DependencyPlanError::UnknownUnit {
                        unit: unit.clone(),
                    },
                );
            }

            let dependencies = context
                .ordering_dependencies_of(unit)
                .ok_or_else(|| {
                    DependencyPlanError::UnknownUnit {
                        unit: unit.clone(),
                    }
                })?;

            let mut wave_index = 0usize;

            for dependency in dependencies {
                let dependency_wave = unit_waves
                    .get(&dependency)
                    .copied()
                    .ok_or_else(|| {
                        DependencyPlanError::DependencyNotScheduled {
                            unit: unit.clone(),
                            dependency: dependency.clone(),
                        }
                    })?;

                let next_wave = dependency_wave
                    .checked_add(1)
                    .ok_or_else(|| {
                        DependencyPlanError::WaveIndexOverflow {
                            unit: unit.clone(),
                        }
                    })?;

                wave_index = wave_index.max(next_wave);
            }

            unit_waves.insert(unit.clone(), wave_index);

            grouped
                .entry(wave_index)
                .or_default()
                .push(unit.clone());
        }

        // BTreeMap ordering gives deterministic wave ordering.
        // The unit order inside each wave is inherited from the canonical
        // dependency-first ordering.
        let waves = grouped
            .into_iter()
            .map(|(index, units)| {
                DependencyWave::new(index, units)
            })
            .collect();

        Ok(Self {
            context: context.clone(),
            waves,
            ordered_units,
            unit_waves,
            edge_count: context.edge_count(),
        })
    }

    /// Returns the canonical dependency context used to build this plan.
    ///
    /// The returned context is immutable and shares its graph storage.
    #[must_use]
    pub fn context(&self) -> &DependencyContext {
        &self.context
    }

    /// Returns all dependency-ready waves in deterministic order.
    #[must_use]
    pub fn waves(&self) -> &[DependencyWave] {
        &self.waves
    }

    /// Returns the complete deterministic dependency-first order.
    #[must_use]
    pub fn ordered_units(&self) -> &[CompilationUnitId] {
        &self.ordered_units
    }

    /// Returns the wave index for a compilation unit.
    ///
    /// Returns `None` if the unit is not part of this plan.
    #[must_use]
    pub fn wave_index(
        &self,
        unit: &CompilationUnitId,
    ) -> Option<usize> {
        self.unit_waves.get(unit).copied()
    }

    /// Returns the number of compilation units in this plan.
    #[must_use]
    pub fn unit_count(&self) -> usize {
        self.ordered_units.len()
    }

    /// Returns the number of relationships in the canonical graph.
    ///
    /// This includes non-ordering relationships.
    #[must_use]
    pub const fn edge_count(&self) -> usize {
        self.edge_count
    }

    /// Returns the number of dependency-ready waves.
    #[must_use]
    pub fn wave_count(&self) -> usize {
        self.waves.len()
    }

    /// Returns whether the plan contains no compilation units.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ordered_units.is_empty()
    }

    /// Returns whether at least one wave contains multiple units.
    ///
    /// This is a structural property of the dependency graph. It does not
    /// promise that resources, budgets, or other compilation constraints
    /// permit those units to execute simultaneously.
    #[must_use]
    pub fn has_parallelism(&self) -> bool {
        self.waves
            .iter()
            .any(DependencyWave::permits_parallel_execution)
    }

    /// Returns the number of units in the largest dependency-ready wave.
    #[must_use]
    pub fn maximum_wave_width(&self) -> usize {
        self.waves
            .iter()
            .map(DependencyWave::len)
            .max()
            .unwrap_or(0)
    }
}

impl TryFrom<&DependencyContext> for DependencyPlan {
    type Error = DependencyPlanError;

    fn try_from(
        context: &DependencyContext,
    ) -> Result<Self, Self::Error> {
        Self::from_context(context)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::context::dependency_context::{
        CompilationUnit,
        CompilationUnitKind,
        DependencyContextBuilder,
        DependencyKind,
    };

    fn unit(
        builder: &mut DependencyContextBuilder,
        identifier: &str,
    ) -> CompilationUnitId {
        let id = CompilationUnitId::new(identifier)
            .expect("test unit identifier must be valid");

        let kind = CompilationUnitKind::new("module")
            .expect("test unit kind must be valid");

        let compilation_unit = CompilationUnit::new(
            id.clone(),
            kind,
            identifier,
        )
        .expect("test compilation unit must be valid");

        builder
            .add_unit(compilation_unit)
            .expect("test unit must be registered");

        id
    }

    fn add_ordering_dependency(
        builder: &mut DependencyContextBuilder,
        dependent: &CompilationUnitId,
        dependency: &CompilationUnitId,
    ) {
        builder
            .add_dependency(
                dependent.clone(),
                dependency.clone(),
                DependencyKind::ModuleImport,
            )
            .expect("test dependency must be valid");
    }

    #[test]
    fn empty_context_produces_empty_plan() {
        let context = DependencyContext::new();

        let plan = DependencyPlan::from_context(&context)
            .expect("empty context must be valid");

        assert!(plan.is_empty());
        assert_eq!(plan.unit_count(), 0);
        assert_eq!(plan.edge_count(), 0);
        assert_eq!(plan.wave_count(), 0);
        assert_eq!(plan.maximum_wave_width(), 0);
        assert!(!plan.has_parallelism());
    }

    #[test]
    fn independent_units_share_the_first_wave() {
        let mut builder = DependencyContextBuilder::new();

        let first = unit(&mut builder, "module:first");
        let second = unit(&mut builder, "module:second");
        let third = unit(&mut builder, "module:third");

        let context = builder
            .build()
            .expect("independent graph must be valid");

        let plan = DependencyPlan::from_context(&context)
            .expect("independent graph must produce a plan");

        assert_eq!(plan.wave_count(), 1);
        assert_eq!(plan.maximum_wave_width(), 3);
        assert!(plan.has_parallelism());

        assert_eq!(
            plan.waves()[0].units(),
            &[first, second, third],
        );
    }

    #[test]
    fn dependencies_are_assigned_to_earlier_waves() {
        let mut builder = DependencyContextBuilder::new();

        let library = unit(&mut builder, "library");
        let frontend = unit(&mut builder, "frontend");
        let backend = unit(&mut builder, "backend");
        let application = unit(&mut builder, "application");

        add_ordering_dependency(
            &mut builder,
            &frontend,
            &library,
        );

        add_ordering_dependency(
            &mut builder,
            &backend,
            &library,
        );

        add_ordering_dependency(
            &mut builder,
            &application,
            &frontend,
        );

        add_ordering_dependency(
            &mut builder,
            &application,
            &backend,
        );

        let context = builder
            .build()
            .expect("dependency graph must be valid");

        let plan = DependencyPlan::from_context(&context)
            .expect("dependency plan must be valid");

        assert_eq!(plan.wave_count(), 3);

        assert_eq!(
            plan.wave_index(&library),
            Some(0),
        );

        assert_eq!(
            plan.wave_index(&frontend),
            Some(1),
        );

        assert_eq!(
            plan.wave_index(&backend),
            Some(1),
        );

        assert_eq!(
            plan.wave_index(&application),
            Some(2),
        );

        assert_eq!(
            plan.waves()[0].units(),
            &[library],
        );

        assert_eq!(
            plan.waves()[1].units(),
            &[frontend, backend],
        );

        assert_eq!(
            plan.waves()[2].units(),
            &[application],
        );
    }

    #[test]
    fn non_ordering_relationships_do_not_create_false_waves() {
        let mut builder = DependencyContextBuilder::new();

        let first = unit(&mut builder, "first");
        let second = unit(&mut builder, "second");

        builder
            .add_dependency(
                first.clone(),
                second.clone(),
                DependencyKind::DataFlow,
            )
            .expect("data-flow relationship must be valid");

        builder
            .add_dependency(
                second.clone(),
                first.clone(),
                DependencyKind::Feedback,
            )
            .expect("feedback relationship must be valid");

        let context = builder
            .build()
            .expect("non-ordering cycles must be permitted");

        let plan = DependencyPlan::from_context(&context)
            .expect("non-ordering cycles must not block compilation order");

        assert_eq!(plan.wave_count(), 1);
        assert_eq!(plan.maximum_wave_width(), 2);
        assert!(plan.has_parallelism());

        // The complete relationship graph remains available.
        assert_eq!(plan.edge_count(), 2);
        assert_eq!(plan.context().edge_count(), 2);
    }

    #[test]
    fn ordering_cycles_are_rejected_by_the_canonical_graph() {
        let mut builder = DependencyContextBuilder::new();

        let first = unit(&mut builder, "first");
        let second = unit(&mut builder, "second");

        add_ordering_dependency(
            &mut builder,
            &first,
            &second,
        );

        add_ordering_dependency(
            &mut builder,
            &second,
            &first,
        );

        let result = builder.build();

        assert!(matches!(
            result,
            Err(DependencyGraphError::OrderingCycle { .. })
        ));
    }

    #[test]
    fn unit_wave_lookup_rejects_unknown_units() {
        let context = DependencyContext::new();

        let plan = DependencyPlan::from_context(&context)
            .expect("empty context must be valid");

        let unknown = CompilationUnitId::new("unknown")
            .expect("test identity must be valid");

        assert_eq!(plan.wave_index(&unknown), None);
    }

    #[test]
    fn try_from_uses_the_canonical_context() {
        let context = DependencyContext::new();

        let plan = DependencyPlan::try_from(&context)
            .expect("conversion must succeed");

        assert!(plan.is_empty());
    }

    #[test]
    fn plan_clone_shares_the_canonical_graph() {
        let mut builder = DependencyContextBuilder::new();

        unit(&mut builder, "module");

        let context = builder
            .build()
            .expect("test graph must be valid");

        let plan = DependencyPlan::from_context(&context)
            .expect("test plan must be valid");

        let cloned = plan.clone();

        assert_eq!(cloned.unit_count(), plan.unit_count());
        assert_eq!(cloned.waves(), plan.waves());

        assert!(
            cloned.context().shared_reference_count() >= 2
        );
    }

    #[test]
    fn api_version_is_stable_and_available() {
        assert_eq!(PARALLEL_DEPENDENCY_API_VERSION, 1);
    }
}
