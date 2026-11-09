
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Dependency-Aware Pipeline Ordering.
//!
//! Path: src/compiler/pipeline/ordering.rs
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//!
//! # Purpose
//!
//! Validate compiler-stage dependency graphs and produce deterministic
//! topological ordering and dependency generations.
//!
//! # Ownership
//!
//! This module owns:
//! - StageOrdering.
//! - OrderingError.
//! - Dependency graph validation.
//! - Deterministic topological ordering.
//! - Dependency-generation construction.
//! - Dependency relationship queries.
//!
//! This module does not own:
//! - Stage identity or descriptor construction.
//! - Stage registration or implementation.
//! - Stage activation decisions.
//! - Stage execution.
//! - Pipeline orchestration or lifecycle.
//! - Retry or recovery execution.
//! - Semantic analysis.
//! - Canonical IR.
//! - Optimization algorithms.
//! - Hardware discovery or allocation.
//! - Physical scheduling or routing.
//! - Backend implementation.
//! - Runtime execution.
//!
//! # Canonical integration
//!
//! Stage contracts:
//!     compiler::pipeline::stage
//!
//! Standard stage declarations:
//!     compiler::pipeline::stages
//!
//! Execution:
//!     compiler::pipeline::execution
//!
//! Lifecycle and orchestration:
//!     compiler::pipeline::orchestration
//!
//! # Determinism
//!
//! The graph is independent of descriptor registration order.
//!
//! Stages in each dependency generation are ordered lexically by StageId.
//! Generations are processed in dependency order.
//!
//! This is a deterministic generation-based ordering policy. It is not
//! a promise to globally select the lexically smallest currently-ready
//! stage after every individual graph operation.
//!
//! # Conditional stages
//!
//! This module orders the graph it receives. It does not decide whether
//! a conditional stage applies to a particular compilation.
//!
//! Orchestration must construct and validate the active graph for the
//! compilation. It must not remove a prerequisite while leaving an
//! active dependent that requires that prerequisite.
//!
//! A skipped stage is not a completed stage. Execution remains responsible
//! for enforcing dependency outcome requirements.
//!
//! # Grammar boundary
//!
//! Source-level compilation intent is defined by the grammar and validated
//! by the frontend and semantic authorities.
//!
//! A grammar declaration is not a StageDescriptor or StageId.
//!
//! # Scalability
//!
//! This module imposes no configured maximum on:
//! - stages;
//! - dependencies;
//! - computing domains;
//! - compilation units;
//! - hardware resources;
//! - devices;
//! - workers;
//! - memory;
//! - qubits;
//! - tensor dimensions;
//! - IR operations.
//!
//! The implementation uses dynamically sized collections. Actual capacity
//! remains bounded by available memory, address space and processing
//! resources. No software implementation can guarantee infinite resources.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::compiler::pipeline::stage::{StageDescriptor, StageId};

/// Public API version of the pipeline-ordering contract.
///
/// Independent of language, grammar, compiler, semantic-model, IR and
/// backend ABI versions.
pub const PIPELINE_ORDERING_API_VERSION: u16 = 1;

// ============================================================================
// ERRORS
// ============================================================================

/// Structural errors detected while validating a stage dependency graph.
///
/// These are graph-construction errors, not stage-execution failures.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OrderingError {
    /// A stage identifier occurs more than once.
    DuplicateStage {
        stage: StageId,
    },

    /// A stage refers to an unregistered prerequisite.
    MissingDependency {
        stage: StageId,
        dependency: StageId,
    },

    /// A complete topological ordering could not be produced.
    ///
    /// The unresolved stages include:
    /// - stages participating in a cycle;
    /// - stages transitively blocked by a cycle.
    ///
    /// This is not necessarily an exact list of cycle members.
    CycleDetected {
        unresolved_stages: Vec<StageId>,
    },

    /// An internal graph invariant was violated.
    InternalInvariant {
        message: &'static str,
    },
}

impl fmt::Display for OrderingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateStage { stage } => {
                write!(
                    formatter,
                    "duplicate compiler pipeline stage `{stage}`"
                )
            }

            Self::MissingDependency {
                stage,
                dependency,
            } => {
                write!(
                    formatter,
                    "compiler pipeline stage `{stage}` depends on \
                     unregistered stage `{dependency}`"
                )
            }

            Self::CycleDetected {
                unresolved_stages,
            } => {
                write!(
                    formatter,
                    "compiler pipeline dependency cycle prevents \
                     complete ordering; unresolved stages: "
                )?;

                for (index, stage) in unresolved_stages.iter().enumerate() {
                    if index != 0 {
                        formatter.write_str(", ")?;
                    }

                    write!(formatter, "{stage}")?;
                }

                Ok(())
            }

            Self::InternalInvariant { message } => {
                write!(
                    formatter,
                    "compiler pipeline ordering invariant violated: {message}"
                )
            }
        }
    }
}

impl Error for OrderingError {}

// ============================================================================
// ORDERING RESULT
// ============================================================================

/// Validated, deterministic ordering of a compiler-stage dependency graph.
///
/// This value owns its stage identifiers and graph relationships. It does
/// not retain references to the original descriptors or stage implementations.
///
/// # Ordering guarantees
///
/// For every stage and every declared prerequisite, the prerequisite appears
/// earlier in `ordered_stages()`.
///
/// `levels()` returns dependency generations. Every prerequisite of a stage
/// in generation N belongs to a preceding generation.
///
/// Stages in the same generation have no dependency path between them.
/// This does not automatically authorize concurrent execution.
///
/// # Execution boundary
///
/// `execution.rs` remains responsible for:
/// - implementation registration validation;
/// - required-stage enforcement;
/// - cancellation;
/// - dependency-output delivery;
/// - skipped and blocked outcomes;
/// - execution policy and operational budgets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageOrdering {
    /// Complete deterministic topological ordering.
    ordered_stages: Vec<StageId>,

    /// Dependency generations, with lexical ordering inside each generation.
    levels: Vec<Vec<StageId>>,

    /// Direct prerequisites for each registered stage.
    dependencies: BTreeMap<StageId, Vec<StageId>>,

    /// Direct dependents for each registered stage.
    dependents: BTreeMap<StageId, Vec<StageId>>,

    /// Zero-based position in the complete deterministic ordering.
    positions: BTreeMap<StageId, usize>,
}

impl StageOrdering {
    // ========================================================================
    // CONSTRUCTION
    // ========================================================================

    /// Builds and validates a dependency ordering from borrowed descriptors.
    ///
    /// The operation:
    ///
    /// 1. Indexes registered descriptors.
    /// 2. Rejects duplicate stage identities.
    /// 3. Validates that every prerequisite exists.
    /// 4. Builds direct dependency and dependent relationships.
    /// 5. Rejects invalid dependency metadata.
    /// 6. Performs deterministic topological ordering.
    /// 7. Detects cycles and reports unresolved stages.
    /// 8. Builds stable position indexes.
    ///
    /// The input iterator may be any iterator of borrowed StageDescriptors.
    ///
    /// No arbitrary stage-count or dependency-count ceiling is imposed.
    pub fn build<'a>(
        descriptors: impl IntoIterator<Item = &'a StageDescriptor>,
    ) -> Result<Self, OrderingError> {
        // --------------------------------------------------------------------
        // 1. Index descriptors and reject duplicate identities.
        // --------------------------------------------------------------------

        let mut registered =
            BTreeMap::<StageId, &'a StageDescriptor>::new();

        for descriptor in descriptors {
            let id = descriptor.id().clone();

            if registered.contains_key(&id) {
                return Err(OrderingError::DuplicateStage {
                    stage: id,
                });
            }

            registered.insert(id, descriptor);
        }

        // --------------------------------------------------------------------
        // 2. Initialize graph structures.
        // --------------------------------------------------------------------

        let mut indegree = BTreeMap::<StageId, usize>::new();

        let mut dependencies =
            BTreeMap::<StageId, Vec<StageId>>::new();

        let mut dependents =
            BTreeMap::<StageId, Vec<StageId>>::new();

        for stage_id in registered.keys() {
            indegree.insert(stage_id.clone(), 0);
            dependencies.insert(stage_id.clone(), Vec::new());
            dependents.insert(stage_id.clone(), Vec::new());
        }

        // --------------------------------------------------------------------
        // 3. Validate and construct dependency relationships.
        // --------------------------------------------------------------------

        for (stage_id, descriptor) in &registered {
            let mut stage_dependencies =
                descriptor.dependencies().to_vec();

            // StageDescriptor normally keeps dependencies sorted and
            // rejects duplicates. Normalize and validate defensively at
            // this graph boundary.
            stage_dependencies.sort_unstable();

            for pair in stage_dependencies.windows(2) {
                if pair[0] == pair[1] {
                    return Err(OrderingError::InternalInvariant {
                        message:
                            "stage descriptor contains duplicate dependencies",
                    });
                }
            }

            for dependency in &stage_dependencies {
                if dependency == stage_id {
                    return Err(OrderingError::InternalInvariant {
                        message:
                            "stage descriptor contains a self-dependency",
                    });
                }

                if !registered.contains_key(dependency) {
                    return Err(OrderingError::MissingDependency {
                        stage: stage_id.clone(),
                        dependency: dependency.clone(),
                    });
                }

                let dependency_count = indegree
                    .get_mut(stage_id)
                    .ok_or(OrderingError::InternalInvariant {
                        message:
                            "registered stage has no indegree entry",
                    })?;

                *dependency_count = dependency_count
                    .checked_add(1)
                    .ok_or(OrderingError::InternalInvariant {
                        message:
                            "dependency counter overflowed",
                    })?;

                let dependency_dependents = dependents
                    .get_mut(dependency)
                    .ok_or(OrderingError::InternalInvariant {
                        message:
                            "registered prerequisite has no dependent entry",
                    })?;

                dependency_dependents.push(stage_id.clone());
            }

            dependencies.insert(
                stage_id.clone(),
                stage_dependencies,
            );
        }

        // Normalize dependent lists.
        for stage_dependents in dependents.values_mut() {
            stage_dependents.sort_unstable();
        }

        // --------------------------------------------------------------------
        // 4. Initialize deterministic topological processing.
        // --------------------------------------------------------------------

        // BTreeSet provides deterministic lexical ordering for each
        // dependency generation.
        let mut ready = BTreeSet::<StageId>::new();

        for (stage_id, dependency_count) in &indegree {
            if *dependency_count == 0 {
                ready.insert(stage_id.clone());
            }
        }

        let mut ordered_stages =
            Vec::with_capacity(registered.len());

        let mut levels = Vec::<Vec<StageId>>::new();

        // --------------------------------------------------------------------
        // 5. Kahn's algorithm, preserving dependency generations.
        // --------------------------------------------------------------------

        while !ready.is_empty() {
            // All stages in this generation have their prerequisites
            // satisfied by earlier generations.
            //
            // Taking the complete ready set before processing ensures
            // that newly-ready dependents enter the next generation.
            let current_level: Vec<StageId> =
                std::mem::take(&mut ready)
                    .into_iter()
                    .collect();

            let mut next_ready = BTreeSet::<StageId>::new();

            for stage_id in &current_level {
                ordered_stages.push(stage_id.clone());

                let stage_dependents = dependents
                    .get(stage_id)
                    .ok_or(OrderingError::InternalInvariant {
                        message:
                            "ordered stage has no dependent entry",
                    })?;

                for dependent in stage_dependents {
                    let dependency_count = indegree
                        .get_mut(dependent)
                        .ok_or(OrderingError::InternalInvariant {
                            message:
                                "dependent stage has no indegree entry",
                        })?;

                    *dependency_count = dependency_count
                        .checked_sub(1)
                        .ok_or(OrderingError::InternalInvariant {
                            message:
                                "dependency counter underflowed",
                        })?;

                    if *dependency_count == 0 {
                        next_ready.insert(dependent.clone());
                    }
                }
            }

            levels.push(current_level);
            ready = next_ready;
        }

        // --------------------------------------------------------------------
        // 6. Detect cycles and their unresolved downstream stages.
        // --------------------------------------------------------------------

        if ordered_stages.len() != registered.len() {
            let unresolved_stages = indegree
                .iter()
                .filter_map(|(stage_id, remaining_dependencies)| {
                    (*remaining_dependencies > 0)
                        .then(|| stage_id.clone())
                })
                .collect();

            return Err(OrderingError::CycleDetected {
                unresolved_stages,
            });
        }

        // --------------------------------------------------------------------
        // 7. Build the position index.
        // --------------------------------------------------------------------

        let mut positions =
            BTreeMap::<StageId, usize>::new();

        for (position, stage_id) in
            ordered_stages.iter().enumerate()
        {
            if positions
                .insert(stage_id.clone(), position)
                .is_some()
            {
                return Err(OrderingError::InternalInvariant {
                    message:
                        "topological ordering contains a duplicate stage",
                });
            }
        }

        // --------------------------------------------------------------------
        // 8. Validate the final graph snapshot.
        // --------------------------------------------------------------------

        if positions.len() != registered.len()
            || dependencies.len() != registered.len()
            || dependents.len() != registered.len()
        {
            return Err(OrderingError::InternalInvariant {
                message:
                    "validated graph indexes have inconsistent stage counts",
            });
        }

        for (stage_id, stage_dependencies) in &dependencies {
            let stage_position = positions
                .get(stage_id)
                .copied()
                .ok_or(OrderingError::InternalInvariant {
                    message:
                        "dependency graph references an unordered stage",
                })?;

            for dependency in stage_dependencies {
                let dependency_position = positions
                    .get(dependency)
                    .copied()
                    .ok_or(OrderingError::InternalInvariant {
                        message:
                            "dependency graph references an unordered prerequisite",
                    })?;

                if dependency_position >= stage_position {
                    return Err(OrderingError::InternalInvariant {
                        message:
                            "topological ordering violates a dependency edge",
                    });
                }
            }
        }

        Ok(Self {
            ordered_stages,
            levels,
            dependencies,
            dependents,
            positions,
        })
    }

    /// Builds an ordering from a descriptor slice.
    ///
    /// Convenience wrapper around `build`.
    pub fn from_slice(
        descriptors: &[StageDescriptor],
    ) -> Result<Self, OrderingError> {
        Self::build(descriptors.iter())
    }

    // ========================================================================
    // ORDER QUERIES
    // ========================================================================

    /// Returns the complete deterministic topological ordering.
    ///
    /// Every registered stage appears exactly once.
    ///
    /// Every prerequisite appears before its dependent.
    #[must_use]
    pub fn ordered_stages(&self) -> &[StageId] {
        &self.ordered_stages
    }

    /// Returns dependency generations.
    ///
    /// Stages are sorted lexically inside each generation.
    ///
    /// Generations are processed in dependency order.
    ///
    /// These levels do not authorize parallel execution.
    #[must_use]
    pub fn levels(&self) -> &[Vec<StageId>] {
        &self.levels
    }

    /// Returns the number of registered stages.
    ///
    /// This is descriptive metadata, not a configured capacity limit.
    #[must_use]
    pub fn stage_count(&self) -> usize {
        self.ordered_stages.len()
    }

    /// Returns whether the ordering contains the supplied stage.
    #[must_use]
    pub fn contains(&self, stage: &StageId) -> bool {
        self.positions.contains_key(stage)
    }

    /// Returns the direct prerequisites of a stage.
    ///
    /// The returned slice is sorted by StageId.
    ///
    /// Returns `None` if the stage is not registered.
    #[must_use]
    pub fn dependencies_of(
        &self,
        stage: &StageId,
    ) -> Option<&[StageId]> {
        self.dependencies
            .get(stage)
            .map(Vec::as_slice)
    }

    /// Returns the direct dependents of a stage.
    ///
    /// The returned slice is sorted by StageId.
    ///
    /// Returns `None` if the stage is not registered.
    #[must_use]
    pub fn dependents_of(
        &self,
        stage: &StageId,
    ) -> Option<&[StageId]> {
        self.dependents
            .get(stage)
            .map(Vec::as_slice)
    }

    /// Returns the zero-based position in the deterministic total order.
    ///
    /// Returns `None` if the stage is not registered.
    #[must_use]
    pub fn position_of(
        &self,
        stage: &StageId,
    ) -> Option<usize> {
        self.positions.get(stage).copied()
    }

    /// Returns whether `first` appears before `second` in the generated
    /// total order.
    ///
    /// This reports ordering position, including deterministic tie-breaking
    /// between independent stages. It does not prove a dependency path.
    ///
    /// Returns `None` if either stage is not registered.
    #[must_use]
    pub fn appears_before(
        &self,
        first: &StageId,
        second: &StageId,
    ) -> Option<bool> {
        let first_position = self.position_of(first)?;
        let second_position = self.position_of(second)?;

        Some(first_position < second_position)
    }

    /// Returns whether `stage` directly or transitively depends on
    /// `prerequisite`.
    ///
    /// Unlike `appears_before`, this query checks actual graph reachability.
    ///
    /// The search is iterative and does not use recursive graph traversal.
    ///
    /// Returns:
    /// - `Some(true)` when a dependency path exists;
    /// - `Some(false)` when both stages exist but no path exists;
    /// - `None` when either stage is not registered.
    ///
    /// A stage is not considered to depend on itself.
    #[must_use]
    pub fn depends_on(
        &self,
        stage: &StageId,
        prerequisite: &StageId,
    ) -> Option<bool> {
        if !self.contains(stage) || !self.contains(prerequisite) {
            return None;
        }

        if stage == prerequisite {
            return Some(false);
        }

        let mut visited = BTreeSet::<StageId>::new();
        let mut pending = vec![stage.clone()];

        while let Some(current) = pending.pop() {
            let current_dependencies = self
                .dependencies
                .get(&current)?;

            for dependency in current_dependencies {
                if dependency == prerequisite {
                    return Some(true);
                }

                if visited.insert(dependency.clone()) {
                    pending.push(dependency.clone());
                }
            }
        }

        Some(false)
    }

    /// Returns whether the graph contains no stages.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ordered_stages.is_empty()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::context::compilation_state::CompilationPhase;
    use crate::compiler::pipeline::stage::StageDescriptor;

    fn stage(
        id: &str,
        dependencies: &[&str],
    ) -> StageDescriptor {
        let stage_id = StageId::new(id)
            .expect("test stage identifier must be valid");

        let mut descriptor = StageDescriptor::new(
            stage_id,
            id,
            "1",
            format!("Test stage {id}"),
        )
        .expect("test stage descriptor must be valid")
        .with_phase(CompilationPhase::Initialization);

        for dependency in dependencies {
            let dependency_id = StageId::new(*dependency)
                .expect("test dependency identifier must be valid");

            descriptor = descriptor
                .with_dependency(dependency_id)
                .expect("test dependency must be valid");
        }

        descriptor
    }

    fn ids(ordering: &StageOrdering) -> Vec<String> {
        ordering
            .ordered_stages()
            .iter()
            .map(|stage_id| stage_id.as_str().to_owned())
            .collect()
    }

    // ------------------------------------------------------------------------
    // Empty and single-stage graphs
    // ------------------------------------------------------------------------

    #[test]
    fn empty_graph_is_valid() {
        let ordering = StageOrdering::from_slice(&[])
            .expect("empty graph must be valid");

        assert!(ordering.is_empty());
        assert_eq!(ordering.stage_count(), 0);
        assert!(ordering.ordered_stages().is_empty());
        assert!(ordering.levels().is_empty());
    }

    #[test]
    fn single_stage_is_ordered() {
        let descriptors = vec![
            stage("compile.initialize", &[]),
        ];

        let ordering = StageOrdering::from_slice(&descriptors)
            .expect("single-stage graph must be valid");

        assert_eq!(
            ids(&ordering),
            vec!["compile.initialize".to_owned()]
        );

        assert_eq!(ordering.levels().len(), 1);
        assert_eq!(ordering.stage_count(), 1);
    }

    // ------------------------------------------------------------------------
    // Dependency ordering
    // ------------------------------------------------------------------------

    #[test]
    fn linear_dependencies_are_ordered_correctly() {
        let descriptors = vec![
            stage("compile.finalize", &["backend.codegen"]),
            stage("backend.codegen", &["ir.verify"]),
            stage("ir.verify", &["ir.generate"]),
            stage("ir.generate", &["semantic.analyze"]),
            stage("semantic.analyze", &[]),
        ];

        let ordering = StageOrdering::from_slice(&descriptors)
            .expect("linear dependency graph must be valid");

        assert_eq!(
            ids(&ordering),
            vec![
                "semantic.analyze",
                "ir.generate",
                "ir.verify",
                "backend.codegen",
                "compile.finalize",
            ]
        );
    }

    #[test]
    fn independent_stages_share_a_generation() {
        let descriptors = vec![
            stage("frontend.parse", &[]),
            stage("frontend.preprocess", &[]),
            stage("semantic.analyze", &["frontend.parse"]),
        ];

        let ordering = StageOrdering::from_slice(&descriptors)
            .expect("independent stages must be valid");

        assert_eq!(
            ordering.levels()[0]
                .iter()
                .map(StageId::as_str)
                .collect::<Vec<_>>(),
            vec![
                "frontend.parse",
                "frontend.preprocess",
            ]
        );

        assert_eq!(
            ordering.levels()[1]
                .iter()
                .map(StageId::as_str)
                .collect::<Vec<_>>(),
            vec!["semantic.analyze"]
        );
    }

    #[test]
    fn each_dependency_generation_is_lexically_ordered() {
        let descriptors = vec![
            stage("z.root", &[]),
            stage("a.root", &[]),
            stage("m.child", &["a.root"]),
            stage("b.child", &["z.root"]),
        ];

        let ordering = StageOrdering::from_slice(&descriptors)
            .expect("graph must be valid");

        assert_eq!(
            ordering.levels()[0]
                .iter()
                .map(StageId::as_str)
                .collect::<Vec<_>>(),
            vec!["a.root", "z.root"]
        );

        assert_eq!(
            ordering.levels()[1]
                .iter()
                .map(StageId::as_str)
                .collect::<Vec<_>>(),
            vec!["b.child", "m.child"]
        );
    }

    #[test]
    fn registration_order_does_not_change_result() {
        let first = vec![
            stage("stage.c", &["stage.a"]),
            stage("stage.b", &[]),
            stage("stage.a", &[]),
        ];

        let second = vec![
            stage("stage.a", &[]),
            stage("stage.c", &["stage.a"]),
            stage("stage.b", &[]),
        ];

        let first_order = StageOrdering::from_slice(&first)
            .expect("first graph must be valid");

        let second_order = StageOrdering::from_slice(&second)
            .expect("second graph must be valid");

        assert_eq!(
            first_order.ordered_stages(),
            second_order.ordered_stages()
        );

        assert_eq!(
            first_order.levels(),
            second_order.levels()
        );
    }

    // ------------------------------------------------------------------------
    // Invalid graph structures
    // ------------------------------------------------------------------------

    #[test]
    fn duplicate_stage_identity_is_rejected() {
        let descriptors = vec![
            stage("stage.a", &[]),
            stage("stage.a", &[]),
        ];

        let error = StageOrdering::from_slice(&descriptors)
            .expect_err("duplicate stage must be rejected");

        assert!(matches!(
            error,
            OrderingError::DuplicateStage { .. }
        ));
    }

    #[test]
    fn missing_dependency_is_rejected() {
        let descriptors = vec![
            stage("stage.a", &["stage.missing"]),
        ];

        let error = StageOrdering::from_slice(&descriptors)
            .expect_err("missing dependency must be rejected");

        assert!(matches!(
            error,
            OrderingError::MissingDependency { .. }
        ));
    }

    #[test]
    fn direct_cycle_is_rejected() {
        let descriptors = vec![
            stage("stage.a", &["stage.b"]),
            stage("stage.b", &["stage.a"]),
        ];

        let error = StageOrdering::from_slice(&descriptors)
            .expect_err("cycle must be rejected");

        match error {
            OrderingError::CycleDetected {
                unresolved_stages,
            } => {
                assert_eq!(
                    unresolved_stages
                        .iter()
                        .map(StageId::as_str)
                        .collect::<Vec<_>>(),
                    vec!["stage.a", "stage.b"]
                );
            }

            other => panic!("unexpected error: {other}"),
        }
    }

    #[test]
    fn indirect_cycle_is_rejected() {
        let descriptors = vec![
            stage("stage.a", &["stage.c"]),
            stage("stage.b", &["stage.a"]),
            stage("stage.c", &["stage.b"]),
        ];

        let error = StageOrdering::from_slice(&descriptors)
            .expect_err("cycle must be rejected");

        assert!(matches!(
            error,
            OrderingError::CycleDetected { .. }
        ));
    }

    #[test]
    fn cycle_reports_downstream_blocked_stages() {
        let descriptors = vec![
            stage("stage.a", &["stage.b"]),
            stage("stage.b", &["stage.a"]),
            stage("stage.c", &["stage.b"]),
            stage("stage.independent", &[]),
        ];

        let error = StageOrdering::from_slice(&descriptors)
            .expect_err("cyclic graph must be rejected");

        match error {
            OrderingError::CycleDetected {
                unresolved_stages,
            } => {
                let unresolved = unresolved_stages
                    .iter()
                    .map(StageId::as_str)
                    .collect::<Vec<_>>();

                assert!(unresolved.contains(&"stage.a"));
                assert!(unresolved.contains(&"stage.b"));
                assert!(unresolved.contains(&"stage.c"));
                assert!(!unresolved.contains(&"stage.independent"));
            }

            other => panic!("unexpected error: {other}"),
        }
    }

    // ------------------------------------------------------------------------
    // Direct relationship queries
    // ------------------------------------------------------------------------

    #[test]
    fn direct_dependency_queries_are_correct() {
        let descriptors = vec![
            stage("stage.a", &[]),
            stage("stage.b", &["stage.a"]),
            stage("stage.c", &["stage.a", "stage.b"]),
        ];

        let ordering = StageOrdering::from_slice(&descriptors)
            .expect("graph must be valid");

        let stage_c = StageId::new("stage.c")
            .expect("identifier must be valid");

        let dependencies = ordering
            .dependencies_of(&stage_c)
            .expect("stage must exist");

        assert_eq!(
            dependencies
                .iter()
                .map(StageId::as_str)
                .collect::<Vec<_>>(),
            vec!["stage.a", "stage.b"]
        );
    }

    #[test]
    fn direct_dependent_queries_are_correct() {
        let descriptors = vec![
            stage("stage.a", &[]),
            stage("stage.b", &["stage.a"]),
            stage("stage.c", &["stage.a"]),
            stage("stage.d", &["stage.b"]),
        ];

        let ordering = StageOrdering::from_slice(&descriptors)
            .expect("graph must be valid");

        let stage_a = StageId::new("stage.a")
            .expect("identifier must be valid");

        let dependents = ordering
            .dependents_of(&stage_a)
            .expect("stage must exist");

        assert_eq!(
            dependents
                .iter()
                .map(StageId::as_str)
                .collect::<Vec<_>>(),
            vec!["stage.b", "stage.c"]
        );
    }

    // ------------------------------------------------------------------------
    // Ordering and dependency are different concepts
    // ------------------------------------------------------------------------

    #[test]
    fn position_queries_reflect_deterministic_order() {
        let descriptors = vec![
            stage("stage.c", &["stage.b"]),
            stage("stage.b", &["stage.a"]),
            stage("stage.a", &[]),
        ];

        let ordering = StageOrdering::from_slice(&descriptors)
            .expect("graph must be valid");

        let a = StageId::new("stage.a").expect("valid ID");
        let b = StageId::new("stage.b").expect("valid ID");
        let c = StageId::new("stage.c").expect("valid ID");

        assert_eq!(ordering.appears_before(&a, &b), Some(true));
        assert_eq!(ordering.appears_before(&b, &c), Some(true));
        assert_eq!(ordering.appears_before(&a, &c), Some(true));
        assert_eq!(ordering.appears_before(&c, &a), Some(false));
    }

    #[test]
    fn transitive_dependency_queries_are_correct() {
        let descriptors = vec![
            stage("stage.a", &[]),
            stage("stage.b", &["stage.a"]),
            stage("stage.c", &["stage.b"]),
            stage("stage.independent", &[]),
        ];

        let ordering = StageOrdering::from_slice(&descriptors)
            .expect("graph must be valid");

        let a = StageId::new("stage.a").expect("valid ID");
        let b = StageId::new("stage.b").expect("valid ID");
        let c = StageId::new("stage.c").expect("valid ID");
        let independent = StageId::new("stage.independent")
            .expect("valid ID");

        assert_eq!(ordering.depends_on(&c, &a), Some(true));
        assert_eq!(ordering.depends_on(&c, &b), Some(true));
        assert_eq!(ordering.depends_on(&b, &c), Some(false));
        assert_eq!(
            ordering.depends_on(&independent, &a),
            Some(false)
        );
        assert_eq!(ordering.depends_on(&a, &a), Some(false));
    }

    #[test]
    fn unknown_stage_queries_return_none() {
        let descriptors = vec![
            stage("stage.a", &[]),
        ];

        let ordering = StageOrdering::from_slice(&descriptors)
            .expect("graph must be valid");

        let unknown = StageId::new("stage.unknown")
            .expect("identifier must be valid");

        assert!(!ordering.contains(&unknown));
        assert_eq!(ordering.position_of(&unknown), None);
        assert_eq!(ordering.dependencies_of(&unknown), None);
        assert_eq!(ordering.dependents_of(&unknown), None);
        assert_eq!(ordering.appears_before(&unknown, &unknown), None);
        assert_eq!(ordering.depends_on(&unknown, &unknown), None);
    }

    // ------------------------------------------------------------------------
    // Cross-domain dependency preservation
    // ------------------------------------------------------------------------

    #[test]
    fn cross_domain_dependencies_are_preserved() {
        let descriptors = vec![
            stage("backend.realize", &["planning.hybrid"]),
            stage(
                "planning.hybrid",
                &[
                    "classical.analyze",
                    "quantum.analyze",
                    "ai.analyze",
                    "hdl.analyze",
                    "distributed.analyze",
                ],
            ),
            stage("classical.analyze", &[]),
            stage("quantum.analyze", &[]),
            stage("ai.analyze", &[]),
            stage("hdl.analyze", &[]),
            stage("distributed.analyze", &[]),
        ];

        let ordering = StageOrdering::from_slice(&descriptors)
            .expect("cross-domain graph must be valid");

        let hybrid = StageId::new("planning.hybrid")
            .expect("valid ID");

        let backend = StageId::new("backend.realize")
            .expect("valid ID");

        assert_eq!(
            ordering.depends_on(&backend, &hybrid),
            Some(true)
        );

        for domain in [
            "classical.analyze",
            "quantum.analyze",
            "ai.analyze",
            "hdl.analyze",
            "distributed.analyze",
        ] {
            let domain_id = StageId::new(domain)
                .expect("valid ID");

            assert_eq!(
                ordering.depends_on(&hybrid, &domain_id),
                Some(true)
            );
        }
    }

    // ------------------------------------------------------------------------
    // Large dynamically sized graph
    // ------------------------------------------------------------------------

    #[test]
    fn dynamically_sized_graph_has_no_small_fixed_stage_ceiling() {
        let descriptors = (0..1_024)
            .map(|index| {
                let id = format!("stage.{index:04}");

                if index == 0 {
                    stage(&id, &[])
                } else {
                    let dependency = format!(
                        "stage.{:04}",
                        index - 1
                    );

                    let dependency_refs = [dependency.as_str()];

                    stage(&id, &dependency_refs)
                }
            })
            .collect::<Vec<_>>();

        let ordering = StageOrdering::from_slice(&descriptors)
            .expect("dynamically sized graph must be valid");

        assert_eq!(ordering.stage_count(), descriptors.len());
        assert_eq!(ordering.levels().len(), descriptors.len());
    }
}
