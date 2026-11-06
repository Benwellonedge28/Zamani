
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Parallel Determinism.
//!
//! Path:
//!     src/compiler/parallel/determinism.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only. Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Provides deterministic result ordering and validation for parallel
//! compiler operations.
//!
//! This module ensures that results collected from parallel compilation
//! jobs are independent of worker completion order.
//!
//! # Canonical authorities
//!
//! General determinism semantics:
//!
//!     crate::classical::concurrency::determinism
//!
//! Canonical compilation dependency ordering:
//!
//!     crate::compiler::parallel::dependencies
//!
//! Canonical compilation-unit identity:
//!
//!     crate::compiler::context::dependency_context
//!
//! # Ownership
//!
//! This module owns:
//!
//! - The compiler-facing determinism adapter.
//! - Deterministic result-collection plans.
//! - Compilation-result ordering validation.
//! - Duplicate, unknown and missing result detection.
//! - Ordered compilation-result representations.
//!
//! This module does not own:
//!
//! - Language-level determinism semantics.
//! - Source syntax or grammar.
//! - AST or semantic analysis.
//! - Dependency discovery.
//! - Dependency graph construction.
//! - Worker execution.
//! - Thread creation.
//! - Cancellation.
//! - Numerical reproducibility algorithms.
//! - Random-number generation.
//! - Distributed consensus.
//! - Runtime event ordering.
//! - Physical hardware scheduling.
//! - Compiler IR.
//!
//! # Determinism model
//!
//! Parallel compilation:
//!
//!     canonical dependency plan
//!               |
//!               v
//!       parallel execution
//!               |
//!               v
//!       unordered completion
//!               |
//!               v
//!       deterministic collection
//!               |
//!               v
//!       dependency-first results
//!
//! Result ordering is independent of:
//!
//! - Worker identity.
//! - Thread identity.
//! - Worker completion time.
//! - Operating-system scheduling.
//! - Hash-map iteration.
//! - Physical machine topology.
//!
//! # Important semantic distinction
//!
//! Deterministic result collection does not prove that arbitrary concurrent
//! callbacks, external side effects, floating-point operations, or backend
//! implementations are deterministic.
//!
//! Those guarantees must be established by semantic analysis, effect
//! analysis, the canonical determinism policy, and the relevant downstream
//! implementation.
//!
//! # Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Compilation units.
//! - Dependency waves.
//! - Result entries.
//! - Logical ordering keys.
//! - Computing domains.
//! - Worker counts.
//! - CPU counts.
//! - GPU counts.
//! - FPGA counts.
//! - QPU counts.
//! - Distributed nodes.
//!
//! Storage grows with the number of compilation units and their identifiers.
//! Actual memory availability and host representation limits remain real
//! operational constraints.
//!
//! No finite implementation promises infinite physical resources.

use std::collections::{BTreeMap, BTreeSet};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

use crate::classical::concurrency::{
    DeterminismError,
    DeterminismPolicy,
    DeterminismMode,
    NumericReproducibility,
    ObservableOrder,
    OrderKey,
    OrderedEvents,
};

use crate::compiler::context::dependency_context::CompilationUnitId;

use crate::compiler::parallel::dependencies::DependencyPlan;

/// Public API version of the compiler parallel-determinism contract.
///
/// Independent of the Zamani language, grammar, semantic-model, IR,
/// backend and runtime versions.
pub const PARALLEL_DETERMINISM_API_VERSION: u16 = 1;

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by deterministic parallel result collection.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParallelDeterminismError {
    /// A compilation unit occurs more than once in a dependency plan.
    DuplicatePlannedUnit {
        unit: CompilationUnitId,
    },

    /// A result was supplied for a unit not present in the dependency plan.
    UnknownResultUnit {
        unit: CompilationUnitId,
    },

    /// More than one result was supplied for the same compilation unit.
    DuplicateResult {
        unit: CompilationUnitId,
    },

    /// A planned compilation unit did not produce a result.
    MissingResult {
        unit: CompilationUnitId,
    },

    /// The dependency plan's reported count does not match its ordering.
    PlanCountMismatch {
        reported: usize,
        actual: usize,
    },

    /// The host cannot represent the generated logical ordering index.
    ///
    /// This is a representation failure, not a language-level capacity
    /// restriction.
    OrderingIndexOverflow {
        index: usize,
    },

    /// The canonical ordered-event collection rejected an ordering key.
    CanonicalOrdering {
        source: DeterminismError,
    },
}

impl fmt::Display for ParallelDeterminismError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicatePlannedUnit { unit } => {
                write!(
                    formatter,
                    "compilation unit '{unit}' occurs more than once in the dependency plan"
                )
            }

            Self::UnknownResultUnit { unit } => {
                write!(
                    formatter,
                    "parallel compilation returned an unknown unit '{unit}'"
                )
            }

            Self::DuplicateResult { unit } => {
                write!(
                    formatter,
                    "parallel compilation returned more than one result for '{unit}'"
                )
            }

            Self::MissingResult { unit } => {
                write!(
                    formatter,
                    "parallel compilation did not return a result for '{unit}'"
                )
            }

            Self::PlanCountMismatch {
                reported,
                actual,
            } => {
                write!(
                    formatter,
                    "dependency plan reports {reported} units but contains {actual}"
                )
            }

            Self::OrderingIndexOverflow { index } => {
                write!(
                    formatter,
                    "host cannot represent deterministic ordering index {index}"
                )
            }

            Self::CanonicalOrdering { source } => {
                write!(
                    formatter,
                    "canonical deterministic ordering failed: {source}"
                )
            }
        }
    }
}

impl Error for ParallelDeterminismError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CanonicalOrdering { source } => Some(source),
            _ => None,
        }
    }
}

// ============================================================================
// ORDERED COMPILATION RESULT
// ============================================================================

/// One compilation result associated with its canonical compilation-unit
/// identity.
///
/// The result may contain any scheduler outcome, including successful
/// compilation, an ordinary compilation error, a panic report, a blocked
/// prerequisite, or cancellation.
///
/// This type does not reinterpret those outcomes.
#[derive(Debug)]
pub struct OrderedCompilationResult<T> {
    unit: CompilationUnitId,
    value: T,
}

impl<T> OrderedCompilationResult<T> {
    /// Creates an ordered compilation result.
    #[must_use]
    pub fn new(unit: CompilationUnitId, value: T) -> Self {
        Self {
            unit,
            value,
        }
    }

    /// Returns the compilation-unit identity.
    #[must_use]
    pub fn unit(&self) -> &CompilationUnitId {
        &self.unit
    }

    /// Returns the result value.
    #[must_use]
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Consumes the result and returns its components.
    #[must_use]
    pub fn into_parts(self) -> (CompilationUnitId, T) {
        (self.unit, self.value)
    }
}

// ============================================================================
// DETERMINISM ADAPTER
// ============================================================================

/// Compiler-facing adapter for the canonical Zamani determinism policy.
///
/// This adapter deliberately does not define another determinism policy.
///
/// It exposes the relevant policy properties to parallel compiler
/// components while preserving the canonical policy as the authority.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ParallelDeterminism {
    policy: DeterminismPolicy,
}

impl ParallelDeterminism {
    /// Creates an adapter from the canonical determinism policy.
    #[must_use]
    pub const fn new(policy: DeterminismPolicy) -> Self {
        Self {
            policy,
        }
    }

    /// Creates an adapter with the canonical unspecified policy.
    #[must_use]
    pub const fn unspecified() -> Self {
        Self::new(DeterminismPolicy::unspecified())
    }

    /// Returns the canonical policy without modifying it.
    #[must_use]
    pub const fn policy(&self) -> DeterminismPolicy {
        self.policy
    }

    /// Whether deterministic observable behavior is explicitly required.
    #[must_use]
    pub const fn requires_determinism(&self) -> bool {
        self.policy.requires_determinism()
    }

    /// Whether reproducible behavior is explicitly required.
    #[must_use]
    pub const fn requires_reproducibility(&self) -> bool {
        self.policy.requires_reproducibility()
    }

    /// Whether the policy requires source-order observability.
    #[must_use]
    pub const fn requires_source_order(&self) -> bool {
        matches!(
            self.policy.observable_order(),
            ObservableOrder::SourceOrder
        )
    }

    /// Whether the policy requires logical-key observable ordering.
    #[must_use]
    pub const fn requires_logical_key_order(&self) -> bool {
        matches!(
            self.policy.observable_order(),
            ObservableOrder::LogicalKeyOrder
        )
    }

    /// Whether the numerical policy requires an explicitly stable reduction.
    ///
    /// This reports policy intent. It does not implement or certify a
    /// numerical reduction algorithm.
    #[must_use]
    pub const fn requires_stable_reduction(&self) -> bool {
        matches!(
            self.policy.numeric_reproducibility(),
            NumericReproducibility::StableReduction
                | NumericReproducibility::RequireReproducibleResult
        )
    }

    /// Builds a deterministic result-collection plan from the canonical
    /// dependency plan.
    ///
    /// The dependency plan remains the authority for compilation ordering.
    ///
    /// The resulting collection plan does not change the scheduler's
    /// execution strategy or serialize independent compilation jobs.
    pub fn for_dependency_plan(
        &self,
        plan: &DependencyPlan,
    ) -> Result<ParallelDeterminismPlan, ParallelDeterminismError> {
        ParallelDeterminismPlan::from_dependency_plan(
            self.policy,
            plan,
        )
    }
}

// ============================================================================
// DETERMINISTIC RESULT-COLLECTION PLAN
// ============================================================================

/// Immutable result-collection plan for one canonical dependency plan.
///
/// Each compilation unit receives a unique logical ordering key derived
/// from its position in the canonical dependency-first order.
///
/// The keys are internal collection identities. They are not source-level
/// event keys, runtime task IDs, physical worker IDs, or hardware IDs.
#[derive(Clone, Debug)]
pub struct ParallelDeterminismPlan {
    policy: DeterminismPolicy,

    /// Canonical dependency-first compilation-unit order.
    ordered_units: Vec<CompilationUnitId>,

    /// Stable logical result-order key for each compilation unit.
    order_keys: BTreeMap<CompilationUnitId, OrderKey>,
}

impl ParallelDeterminismPlan {
    /// Constructs a deterministic collection plan from the canonical
    /// dependency plan.
    ///
    /// The dependency plan is the only source of compilation-unit ordering.
    pub fn from_dependency_plan(
        policy: DeterminismPolicy,
        plan: &DependencyPlan,
    ) -> Result<Self, ParallelDeterminismError> {
        let ordered_units = plan.ordered_units();

        if ordered_units.len() != plan.unit_count() {
            return Err(
                ParallelDeterminismError::PlanCountMismatch {
                    reported: plan.unit_count(),
                    actual: ordered_units.len(),
                },
            );
        }

        let mut seen = BTreeSet::new();
        let mut order_keys = BTreeMap::new();

        for (index, unit) in ordered_units.iter().enumerate() {
            if !seen.insert(unit.clone()) {
                return Err(
                    ParallelDeterminismError::DuplicatePlannedUnit {
                        unit: unit.clone(),
                    },
                );
            }

            let component = u128::try_from(index).map_err(|_| {
                ParallelDeterminismError::OrderingIndexOverflow {
                    index,
                }
            })?;

            order_keys.insert(
                unit.clone(),
                OrderKey::from(component),
            );
        }

        Ok(Self {
            policy,
            ordered_units: ordered_units.to_vec(),
            order_keys,
        })
    }

    /// Returns the canonical determinism policy.
    #[must_use]
    pub const fn policy(&self) -> DeterminismPolicy {
        self.policy
    }

    /// Returns the expected compilation-unit order.
    #[must_use]
    pub fn ordered_units(&self) -> &[CompilationUnitId] {
        &self.ordered_units
    }

    /// Returns the expected number of compilation results.
    #[must_use]
    pub fn unit_count(&self) -> usize {
        self.ordered_units.len()
    }

    /// Returns whether the plan contains no compilation units.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ordered_units.is_empty()
    }

    /// Returns the canonical logical ordering key for a compilation unit.
    #[must_use]
    pub fn order_key(
        &self,
        unit: &CompilationUnitId,
    ) -> Option<&OrderKey> {
        self.order_keys.get(unit)
    }

    /// Collects compilation results in canonical dependency-first order.
    ///
    /// Input results may arrive in any order, including physical worker
    /// completion order.
    ///
    /// Every expected compilation unit must occur exactly once.
    ///
    /// Unknown, duplicate, and missing results are errors.
    ///
    /// No result is silently overwritten or discarded.
    pub fn collect_results<I, T>(
        &self,
        results: I,
    ) -> Result<Vec<OrderedCompilationResult<T>>, ParallelDeterminismError>
    where
        I: IntoIterator<Item = (CompilationUnitId, T)>,
    {
        let mut seen = BTreeSet::new();

        // Reuse the canonical OrderedEvents implementation. It provides
        // deterministic key ordering and rejects duplicate keys.
        let mut events = OrderedEvents::new();

        for (unit, value) in results {
            let Some(order_key) = self.order_keys.get(&unit) else {
                return Err(
                    ParallelDeterminismError::UnknownResultUnit {
                        unit,
                    },
                );
            };

            if !seen.insert(unit.clone()) {
                return Err(
                    ParallelDeterminismError::DuplicateResult {
                        unit,
                    },
                );
            }

            events
                .insert(
                    order_key.clone(),
                    (unit, value),
                )
                .map_err(|(source, _rejected_value)| {
                    ParallelDeterminismError::CanonicalOrdering {
                        source,
                    }
                })?;
        }

        // A successful report must account for every planned unit.
        for unit in &self.ordered_units {
            if !seen.contains(unit) {
                return Err(
                    ParallelDeterminismError::MissingResult {
                        unit: unit.clone(),
                    },
                );
            }
        }

        // OrderedEvents yields the canonical dependency-first ordering,
        // regardless of the order in which results arrived.
        Ok(events
            .into_ordered()
            .into_iter()
            .map(|event| {
                let (_key, (unit, value)) = event.into_parts();

                OrderedCompilationResult {
                    unit,
                    value,
                }
            })
            .collect())
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
        DependencyContext,
        DependencyContextBuilder,
        DependencyKind,
    };

    use crate::compiler::parallel::dependencies::DependencyPlan;

    fn id(value: &str) -> CompilationUnitId {
        CompilationUnitId::new(value)
            .expect("test identifier must be valid")
    }

    fn unit(value: &str) -> CompilationUnit {
        CompilationUnit::new(
            id(value),
            CompilationUnitKind::new("source")
                .expect("test kind must be valid"),
            value,
        )
        .expect("test compilation unit must be valid")
    }

    fn plan_with_units(
        identifiers: &[&str],
    ) -> DependencyPlan {
        let mut builder = DependencyContextBuilder::new();

        for identifier in identifiers {
            builder
                .add_unit(unit(identifier))
                .expect("test unit registration must succeed");
        }

        let context = builder
            .build()
            .expect("test dependency context must be valid");

        DependencyPlan::from_context(&context)
            .expect("test dependency plan must be valid")
    }

    fn policy() -> DeterminismPolicy {
        DeterminismPolicy::deterministic()
    }

    #[test]
    fn adapter_preserves_the_canonical_policy() {
        let policy = DeterminismPolicy::reproducible();
        let adapter = ParallelDeterminism::new(policy);

        assert_eq!(adapter.policy(), policy);
        assert!(adapter.requires_determinism());
        assert!(adapter.requires_reproducibility());
        assert!(adapter.requires_source_order());
        assert!(adapter.requires_stable_reduction());
    }

    #[test]
    fn unspecified_policy_does_not_claim_reproducibility() {
        let adapter = ParallelDeterminism::unspecified();

        assert!(!adapter.requires_determinism());
        assert!(!adapter.requires_reproducibility());
        assert!(!adapter.requires_source_order());
        assert!(!adapter.requires_logical_key_order());
    }

    #[test]
    fn dependency_plan_defines_result_order() {
        let plan = plan_with_units(&[
            "application",
            "library",
            "utility",
        ]);

        let collection =
            ParallelDeterminism::new(policy())
                .for_dependency_plan(&plan)
                .expect("collection plan");

        assert_eq!(
            collection.ordered_units(),
            plan.ordered_units()
        );
    }

    #[test]
    fn result_collection_ignores_arrival_order() {
        let plan = plan_with_units(&[
            "application",
            "library",
            "utility",
        ]);

        let collection =
            ParallelDeterminism::new(policy())
                .for_dependency_plan(&plan)
                .expect("collection plan");

        let results = vec![
            (id("utility"), 30),
            (id("application"), 10),
            (id("library"), 20),
        ];

        let ordered = collection
            .collect_results(results)
            .expect("all results must be collected");

        let actual: Vec<_> = ordered
            .into_iter()
            .map(|result| result.into_parts())
            .collect();

        let expected: Vec<_> = plan
            .ordered_units()
            .iter()
            .cloned()
            .zip(
                plan.ordered_units()
                    .iter()
                    .map(|unit| match unit.as_str() {
                        "application" => 10,
                        "library" => 20,
                        "utility" => 30,
                        _ => unreachable!("known test unit"),
                    }),
            )
            .collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn duplicate_result_is_rejected() {
        let plan = plan_with_units(&["module"]);
        let collection =
            ParallelDeterminism::new(policy())
                .for_dependency_plan(&plan)
                .expect("collection plan");

        let result = collection.collect_results(vec![
            (id("module"), 1),
            (id("module"), 2),
        ]);

        assert!(matches!(
            result,
            Err(ParallelDeterminismError::DuplicateResult { .. })
        ));
    }

    #[test]
    fn unknown_result_is_rejected() {
        let plan = plan_with_units(&["module"]);
        let collection =
            ParallelDeterminism::new(policy())
                .for_dependency_plan(&plan)
                .expect("collection plan");

        let result =
            collection.collect_results(vec![(id("unknown"), 1)]);

        assert!(matches!(
            result,
            Err(ParallelDeterminismError::UnknownResultUnit { .. })
        ));
    }

    #[test]
    fn missing_result_is_rejected() {
        let plan = plan_with_units(&["first", "second"]);
        let collection =
            ParallelDeterminism::new(policy())
                .for_dependency_plan(&plan)
                .expect("collection plan");

        let first = plan.ordered_units()[0].clone();

        let result =
            collection.collect_results(vec![(first, 1)]);

        assert!(matches!(
            result,
            Err(ParallelDeterminismError::MissingResult { .. })
        ));
    }

    #[test]
    fn empty_plan_accepts_empty_results() {
        let context = DependencyContext::new();
        let plan = DependencyPlan::from_context(&context)
            .expect("empty dependency plan");

        let collection =
            ParallelDeterminism::new(policy())
                .for_dependency_plan(&plan)
                .expect("empty collection plan");

        assert!(collection.is_empty());

        let results = collection
            .collect_results(Vec::<(CompilationUnitId, ())>::new())
            .expect("empty result collection");

        assert!(results.is_empty());
    }

    #[test]
    fn ordering_keys_are_unique_and_stable() {
        let plan = plan_with_units(&[
            "application",
            "library",
            "utility",
        ]);

        let first =
            ParallelDeterminism::new(policy())
                .for_dependency_plan(&plan)
                .expect("first collection plan");

        let second =
            ParallelDeterminism::new(policy())
                .for_dependency_plan(&plan)
                .expect("second collection plan");

        for unit in plan.ordered_units() {
            assert_eq!(
                first.order_key(unit),
                second.order_key(unit)
            );
        }

        let keys: BTreeSet<_> = plan
            .ordered_units()
            .iter()
            .map(|unit| {
                first
                    .order_key(unit)
                    .expect("every unit has an ordering key")
                    .clone()
            })
            .collect();

        assert_eq!(keys.len(), plan.unit_count());
    }

    #[test]
    fn logical_key_order_is_reported_without_redefining_policy() {
        let policy = DeterminismPolicy::new(
            DeterminismMode::Deterministic,
            ObservableOrder::LogicalKeyOrder,
            NumericReproducibility::StableReduction,
        );

        let adapter = ParallelDeterminism::new(policy);

        assert!(adapter.requires_determinism());
        assert!(adapter.requires_logical_key_order());
        assert!(adapter.requires_stable_reduction());
        assert!(!adapter.requires_source_order());
    }

    #[test]
    fn compilation_result_preserves_its_unit_identity() {
        let result = OrderedCompilationResult::new(
            id("module"),
            "compiled",
        );

        assert_eq!(result.unit().as_str(), "module");
        assert_eq!(result.value(), &"compiled");

        let (unit, value) = result.into_parts();

        assert_eq!(unit.as_str(), "module");
        assert_eq!(value, "compiled");
    }
}
