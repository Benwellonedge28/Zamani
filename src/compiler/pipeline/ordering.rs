
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Dependency-Aware Pipeline Ordering.
//!
//! Path:
//!     src/compiler/pipeline/ordering.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version: Rust 1.97
//!
//! # Purpose
//!
//! Provides deterministic dependency-aware ordering for compiler stages.
//!
//! This module consumes the canonical StageDescriptor and StageId
//! contracts defined in:
//!
//!     crate::compiler::pipeline::stage
//!
//! It validates the registered stage dependency graph and produces:
//!
//! - A deterministic topological order.
//! - Dependency-independent execution levels.
//! - Direct dependency queries.
//! - Direct dependent queries.
//! - Relative ordering queries.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - StageOrdering.
//! - OrderingError.
//! - Dependency graph validation.
//! - Deterministic topological ordering.
//! - Dependency-level construction.
//!
//! This module does NOT own:
//!
//! - Stage implementation.
//! - Stage registration.
//! - Stage execution.
//! - Stage activation.
//! - Conditional stage selection.
//! - Compilation lifecycle.
//! - Cancellation.
//! - Resource budgets.
//! - Semantic analysis.
//! - Type inference.
//! - Ownership analysis.
//! - Canonical IR.
//! - IR verification.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Hardware allocation.
//! - Physical scheduling.
//! - Physical routing.
//! - Backend implementation.
//! - Runtime execution.
//!
//! # Dependency direction
//!
//!     pipeline::stage
//!          |
//!          v
//!     pipeline::ordering
//!          |
//!          v
//!     pipeline::execution
//!          |
//!          v
//!     pipeline::orchestration
//!
//! The ordering module does not depend on execution or orchestration.
//!
//! # POCO-REAF
//!
//! This module imposes no universal limits on:
//!
//! - Pipeline stages.
//! - Stage dependencies.
//! - Compilation units.
//! - Functions.
//! - Modules.
//! - CPUs.
//! - Cores.
//! - Workers.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Accelerators.
//! - Nodes.
//! - Devices.
//! - Memory.
//! - Storage.
//! - Qubits.
//! - Tensor dimensions.
//! - Tensor rank.
//! - IR operations.
//!
//! Graph size is bounded only by the resources available to the compiler.
//!
//! This does not imply physically infinite memory or processing capacity.
//!
//! # Determinism
//!
//! Registration order does not determine the resulting order.
//!
//! Among currently eligible stages, StageId lexical ordering is used.
//!
//! Consequently, identical stage identifiers and dependency relationships
//! produce identical ordering results regardless of registration order.
//!
//! # Conditional stages
//!
//! Ordering describes the registered dependency graph.
//!
//! It does not decide whether a stage applies to a particular program.
//!
//! A conditional stage that is skipped remains a skipped stage.
//! It must not automatically be interpreted as successfully completed.
//!
//! The executor must enforce dependency-outcome policy before executing
//! dependent stages.
//!
//! If a stage is not part of a compilation's active graph, orchestration
//! must construct and validate the appropriate active graph while ensuring
//! that no active stage silently loses a required prerequisite.
//!
//! # Grammar boundary
//!
//! The authoritative grammar describes source-level compilation intent.
//!
//! Relevant grammar authorities include:
//!
//!     grammar/Zamani.g4
//!     grammar/antlr/ZamaniParser.g4
//!     grammar/core/program.g4
//!     grammar/compile/compile.g4
//!
//! Grammar declarations are not compiler StageDescriptors.
//!
//! Source-level intent must pass through the frontend and semantic
//! authorities before it can influence stage activation or planning.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::compiler::pipeline::stage::{StageDescriptor, StageId};

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of the pipeline-ordering contract.
///
/// This version is independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Compiler release version.
/// - Semantic-model version.
/// - AST version.
/// - IR version.
/// - Backend ABI version.
pub const PIPELINE_ORDERING_API_VERSION: u16 = 1;

// ============================================================================
// ORDERING ERROR
// ============================================================================

/// Errors detected while validating or ordering a stage dependency graph.
///
/// Errors are structural.
///
/// They are not stage-execution failures.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OrderingError {
    /// A stage identifier occurs more than once in the registry.
    DuplicateStage {
        stage: StageId,
    },

    /// A stage references a prerequisite that is not registered.
    MissingDependency {
        stage: StageId,
        dependency: StageId,
    },

    /// A dependency cycle prevents a complete topological ordering.
    ///
    /// `unresolved_stages` contains all stages that could not be removed
    /// by the topological ordering algorithm.
    ///
    /// This set can include:
    ///
    /// - Stages directly participating in a cycle.
    /// - Stages transitively blocked by a cycle.
    ///
    /// It is therefore deliberately not described as an exact list
    /// of cycle members.
    CycleDetected {
        unresolved_stages: Vec<StageId>,
    },

    /// An internal graph invariant was violated.
    ///
    /// This indicates an implementation defect rather than invalid
    /// user program semantics.
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
                    if index > 0 {
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

/// Validated deterministic ordering of a compiler-stage dependency graph.
///
/// The result is independent of the lifetime of the original descriptors.
///
/// It owns stage identifiers and graph relationships, but does not own
/// stage implementations.
///
/// # Ordering guarantees
///
/// `ordered_stages()` returns a complete topological ordering.
///
/// For every stage S and every declared dependency D:
///
///     position(D) < position(S)
///
/// `levels()` groups stages that have no dependency relationship requiring
/// one of them to precede another in the graph's current topological
/// progression.
///
/// A level does not grant permission to execute stages concurrently.
/// The executor must also check each stage's concurrency contract,
/// resource requirements, cancellation state and operational budgets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageOrdering {
    /// Complete deterministic topological ordering.
    ordered_stages: Vec<StageId>,

    /// Dependency-independent topological levels.
    levels: Vec<Vec<StageId>>,

    /// Direct prerequisites for each registered stage.
    dependencies: BTreeMap<StageId, Vec<StageId>>,

    /// Direct dependents for each registered stage.
    dependents: BTreeMap<StageId, Vec<StageId>>,

    /// Position of each stage in the complete topological ordering.
    positions: BTreeMap<StageId, usize>,
}

impl StageOrdering {
    // ========================================================================
    // CONSTRUCTION
    // ========================================================================

    /// Builds and validates an ordering from registered stage descriptors.
    ///
    /// The input may be any iterator of borrowed StageDescriptors.
    ///
    /// The function:
    ///
    /// 1. Validates unique stage identifiers.
    /// 2. Validates that every dependency is registered.
    /// 3. Builds direct dependency and dependent relationships.
    /// 4. Performs deterministic topological ordering.
    /// 5. Detects unresolved dependency cycles.
    /// 6. Constructs dependency-independent levels.
    ///
    /// No arbitrary stage-count or dependency-count limit is imposed.
    ///
    /// # Determinism
    ///
    /// Eligible stages are selected in StageId lexical order.
    ///
    /// Registration order therefore cannot change the result.
    pub fn build<'a>(
        descriptors: impl IntoIterator<Item = &'a StageDescriptor>,
    ) -> Result<Self, OrderingError> {
        let mut registered = BTreeMap::<StageId, &StageDescriptor>::new();

        // --------------------------------------------------------------------
        // 1. Validate and index registered stages.
        // --------------------------------------------------------------------

        for descriptor in descriptors {
            let id = descriptor.id().clone();

            if registered.insert(id.clone(), descriptor).is_some() {
                return Err(OrderingError::DuplicateStage {
                    stage: id,
                });
            }
        }

        // --------------------------------------------------------------------
        // 2. Initialize graph structures.
        // --------------------------------------------------------------------

        let mut indegree = BTreeMap::<StageId, usize>::new();

        let mut dependencies =
            BTreeMap::<StageId, Vec<StageId>>::new();

        let mut dependents =
            BTreeMap::<StageId, Vec<StageId>>::new();

        for id in registered.keys() {
            indegree.insert(id.clone(), 0);
            dependencies.insert(id.clone(), Vec::new());
            dependents.insert(id.clone(), Vec::new());
        }

        // --------------------------------------------------------------------
        // 3. Validate and construct dependency relationships.
        // --------------------------------------------------------------------

        for (stage_id, descriptor) in &registered {
            let mut stage_dependencies = descriptor.dependencies().to_vec();

            // Normalize dependency ordering independently of the order
            // in which a stage declared its prerequisites.
            stage_dependencies.sort();

            // StageDescriptor already rejects duplicate dependencies.
            // We still validate the graph boundary defensively because
            // ordering must not rely on an unchecked external assumption.
            for pair in stage_dependencies.windows(2) {
                if pair[0] == pair[1] {
                    return Err(OrderingError::InternalInvariant {
                        message:
                            "StageDescriptor exposed duplicate dependencies",
                    });
                }
            }

            for dependency in &stage_dependencies {
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
                            "registered dependency has no dependent entry",
                    })?;

                dependency_dependents.push(stage_id.clone());
            }

            dependencies.insert(stage_id.clone(), stage_dependencies);
        }

        // Normalize dependent lists.
        for stage_dependents in dependents.values_mut() {
            stage_dependents.sort();
        }

        // --------------------------------------------------------------------
        // 4. Initialize deterministic topological processing.
        // --------------------------------------------------------------------

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
        // 5. Deterministic Kahn topological ordering.
        // --------------------------------------------------------------------

        while !ready.is_empty() {
            // Every stage in this level has all its prerequisites
            // satisfied by preceding levels.
            //
            // BTreeSet guarantees stable lexical ordering.
            let current_level: Vec<StageId> =
                std::mem::take(&mut ready).into_iter().collect();

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
                                "dependency counter became negative",
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
        // 6. Detect cycles or stages blocked by cycles.
        // --------------------------------------------------------------------

        if ordered_stages.len() != registered.len() {
            let unresolved_stages = indegree
                .iter()
                .filter_map(|(stage_id, remaining_dependencies)| {
                    (*remaining_dependencies > 0)
                        .then(|| stage_id.clone())
                })
                .collect::<Vec<_>>();

            return Err(OrderingError::CycleDetected {
                unresolved_stages,
            });
        }

        // --------------------------------------------------------------------
        // 7. Build stable position index.
        // --------------------------------------------------------------------

        let mut positions =
            BTreeMap::<StageId, usize>::new();

        for (position, stage_id) in
            ordered_stages.iter().enumerate()
        {
            positions.insert(stage_id.clone(), position);
        }

        // --------------------------------------------------------------------
        // 8. Return validated graph.
        // --------------------------------------------------------------------

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
    /// This is a convenience wrapper around `build`.
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
    /// Every stage appears exactly once.
    ///
    /// Every prerequisite appears before its dependent.
    #[must_use]
    pub fn ordered_stages(&self) -> &[StageId] {
        &self.ordered_stages
    }

    /// Returns dependency-independent topological levels.
    ///
    /// Each level is deterministically ordered by StageId.
    ///
    /// Stages in one level are not automatically safe to execute
    /// concurrently. The executor must enforce their StageConcurrency
    /// declarations and available operational resources.
    #[must_use]
    pub fn levels(&self) -> &[Vec<StageId>] {
        &self.levels
    }

    /// Returns the number of registered stages in this ordering.
    ///
    /// This is descriptive metadata, not a configured stage limit.
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

    /// Returns the zero-based position of a stage in the total ordering.
    ///
    /// Returns `None` if the stage is not registered.
    #[must_use]
    pub fn position_of(
        &self,
        stage: &StageId,
    ) -> Option<usize> {
        self.positions.get(stage).copied()
    }

    /// Returns whether `first` must precede `second` in the total ordering.
    ///
    /// This reports the generated deterministic topological order.
    ///
    /// It does not claim that two unrelated stages have a semantic
    /// dependency relationship.
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

    /// Returns whether the graph is empty.
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
    // Empty graph
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

    // ------------------------------------------------------------------------
    // Single stage
    // ------------------------------------------------------------------------

    #[test]
    fn single_stage_is_ordered() {
        let descriptors = vec![stage("compile.initialize", &[])];

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
    // Linear dependency graph
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

    // ------------------------------------------------------------------------
    // Multiple independent stages
    // ------------------------------------------------------------------------

    #[test]
    fn independent_stages_share_a_level() {
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

    // ------------------------------------------------------------------------
    // Determinism
    // ------------------------------------------------------------------------

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
    // Duplicate stage identity
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

    // ------------------------------------------------------------------------
    // Missing dependency
    // ------------------------------------------------------------------------

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

    // ------------------------------------------------------------------------
    // Direct cycle
    // ------------------------------------------------------------------------

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

    // ------------------------------------------------------------------------
    // Indirect cycle
    // ------------------------------------------------------------------------

    #[test]
    fn indirect_cycle_is_rejected() {
        let descriptors = vec![
            stage("stage.a", &["stage.c"]),
            stage("stage.b", &["stage.a"]),
            stage("stage.c", &["stage.b"]),
        ];

        let error = StageOrdering::from_slice(&descriptors)
            .expect_err("indirect cycle must be rejected");

        assert!(matches!(
            error,
            OrderingError::CycleDetected { .. }
        ));
    }

    // ------------------------------------------------------------------------
    // Cycle with downstream blocked stage
    // ------------------------------------------------------------------------

    #[test]
    fn cycle_reports_all_unresolved_stages() {
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
    // Direct dependency queries
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

    // ------------------------------------------------------------------------
    // Direct dependent queries
    // ------------------------------------------------------------------------

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
    // Position queries
    // ------------------------------------------------------------------------

    #[test]
    fn position_queries_reflect_dependency_order() {
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

    // ------------------------------------------------------------------------
    // Unknown stage queries
    // ------------------------------------------------------------------------

    #[test]
    fn unknown_stage_queries_return_none() {
        let descriptors = vec![stage("stage.a", &[])];

        let ordering = StageOrdering::from_slice(&descriptors)
            .expect("graph must be valid");

        let unknown = StageId::new("stage.unknown")
            .expect("identifier must be valid");

        assert!(!ordering.contains(&unknown));
        assert_eq!(ordering.position_of(&unknown), None);
        assert_eq!(ordering.dependencies_of(&unknown), None);
        assert_eq!(ordering.dependents_of(&unknown), None);
        assert_eq!(ordering.appears_before(&unknown, &unknown), None);
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

        let planning = StageId::new("planning.hybrid")
            .expect("valid ID");

        let dependencies = ordering
            .dependencies_of(&planning)
            .expect("planning stage must exist");

        assert_eq!(dependencies.len(), 5);

        for domain in [
            "classical.analyze",
            "quantum.analyze",
            "ai.analyze",
            "hdl.analyze",
            "distributed.analyze",
        ] {
            let domain_id =
                StageId::new(domain).expect("valid domain ID");

            assert_eq!(
                ordering.appears_before(&domain_id, &planning),
                Some(true)
            );
        }
    }

    // ------------------------------------------------------------------------
    // Empty dependencies and multiple roots
    // ------------------------------------------------------------------------

    #[test]
    fn multiple_roots_are_deterministically_ordered() {
        let descriptors = vec![
            stage("root.z", &[]),
            stage("root.a", &[]),
            stage("root.m", &[]),
        ];

        let ordering = StageOrdering::from_slice(&descriptors)
            .expect("multiple roots must be valid");

        assert_eq!(
            ids(&ordering),
            vec![
                "root.a",
                "root.m",
                "root.z",
            ]
        );

        assert_eq!(ordering.levels().len(), 1);
    }
}
