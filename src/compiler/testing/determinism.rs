
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani compiler determinism conformance tests.
//!
//! File:
//!     src/compiler/testing/determinism.rs
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
//! This module verifies the compiler's determinism integration across:
//!
//! - canonical determinism policies;
//! - compilation dependency ordering;
//! - parallel result collection;
//! - worker-completion-order independence;
//! - dependency registration-order independence;
//! - stable logical ordering keys;
//! - non-ordering dependency relationships;
//! - empty and non-empty compilation plans;
//! - deterministic behavior under different input permutations.
//!
//! # Canonical authorities
//!
//! Language-level determinism policy:
//!
//!     crate::classical::concurrency::determinism
//!
//! Compilation-unit identity and dependency relationships:
//!
//!     crate::compiler::context::dependency_context
//!
//! Dependency-ready compilation planning:
//!
//!     crate::compiler::parallel::dependencies
//!
//! Compiler determinism adapter and result collection:
//!
//!     crate::compiler::parallel::determinism
//!
//! Worker execution:
//!
//!     crate::compiler::parallel::workers
//!
//! Dependency-aware scheduling:
//!
//!     crate::compiler::parallel::scheduler
//!
//! # Ownership
//!
//! This module owns only conformance tests.
//!
//! It does not define:
//!
//! - another determinism policy;
//! - another dependency graph;
//! - another compilation-unit identifier;
//! - another scheduler;
//! - another worker pool;
//! - another compiler pipeline;
//! - another IR;
//! - numerical reproducibility algorithms;
//! - runtime event ordering;
//! - random-number generation;
//! - distributed consensus;
//! - backend artifact serialization.
//!
//! # Semantic distinction
//!
//! These tests verify deterministic compilation-result ordering.
//!
//! They do not prove that arbitrary compiler callbacks, filesystem operations,
//! external tools, floating-point operations, random-number generators,
//! backend implementations, or runtime side effects are deterministic.
//!
//! Those guarantees require their respective semantic, numeric, build,
//! backend, and runtime conformance tests.
//!
//! # Grammar integration
//!
//! Source-level deterministic parallel intent is specified by:
//!
//!     grammar/concurrency/deterministic-parallelism.g4
//!
//! Source-level deterministic-build intent is specified by:
//!
//!     grammar/compile/deterministic-builds.g4
//!
//! Normative determinism semantics are specified by:
//!
//!     grammar/spec/determinism.md
//!
//! This file tests compiler behavior after semantic analysis. It does not
//! parse grammar files or introduce new source-language syntax.
//!
//! # Scalability
//!
//! The tests impose no production limit on:
//!
//! - compilation units;
//! - dependencies;
//! - dependency waves;
//! - result entries;
//! - worker counts;
//! - computing domains;
//! - CPUs;
//! - GPUs;
//! - FPGAs;
//! - QPUs;
//! - distributed nodes.
//!
//! Test workloads are finite test inputs, not language or compiler limits.
//!
//! Production collection remains subject to available host memory and
//! representational limits.
//!
//! # Integration
//!
//! The compiler module tree must expose:
//!
//!     crate::compiler::context::dependency_context
//!     crate::compiler::parallel
//!
//! The test module must be registered under:
//!
//!     crate::compiler::testing
//!
//! No production compiler behavior is added by this file.

#[cfg(test)]
mod tests {
    use crate::classical::concurrency::determinism::{
        DeterminismMode,
        DeterminismPolicy,
        NumericReproducibility,
        ObservableOrder,
    };

    use crate::compiler::context::dependency_context::{
        CompilationUnit,
        CompilationUnitId,
        CompilationUnitKind,
        DependencyContext,
        DependencyContextBuilder,
        DependencyKind,
    };

    use crate::compiler::parallel::dependencies::DependencyPlan;

    use crate::compiler::parallel::determinism::{
        ParallelDeterminism,
        ParallelDeterminismError,
        ParallelDeterminismPlan,
    };

    // ------------------------------------------------------------------------
    // Test construction helpers
    // ------------------------------------------------------------------------

    fn id(value: &str) -> CompilationUnitId {
        CompilationUnitId::new(value)
            .expect("test compilation-unit identifier must be valid")
    }

    fn unit(value: &str) -> CompilationUnit {
        CompilationUnit::new(
            id(value),
            CompilationUnitKind::new("source")
                .expect("test compilation-unit kind must be valid"),
            value,
        )
        .expect("test compilation unit must be valid")
    }

    /// Build a canonical dependency context.
    ///
    /// Each dependency tuple is:
    ///
    ///     (dependent, dependency)
    ///
    /// Therefore:
    ///
    ///     ("application", "library")
    ///
    /// means that the application depends on the library.
    fn build_context(
        identifiers: &[&str],
        dependencies: &[(&str, &str)],
    ) -> DependencyContext {
        let mut builder = DependencyContextBuilder::new();

        for identifier in identifiers {
            builder
                .add_unit(unit(identifier))
                .expect("test unit registration must succeed");
        }

        for (dependent, dependency) in dependencies {
            builder
                .add_dependency(
                    id(dependent),
                    id(dependency),
                    DependencyKind::ModuleImport,
                )
                .expect("test dependency registration must succeed");
        }

        builder
            .build()
            .expect("test dependency graph must be valid")
    }

    fn build_plan(
        identifiers: &[&str],
        dependencies: &[(&str, &str)],
    ) -> DependencyPlan {
        let context = build_context(identifiers, dependencies);

        DependencyPlan::from_context(&context)
            .expect("test dependency plan must be valid")
    }

    fn deterministic_policy() -> DeterminismPolicy {
        DeterminismPolicy::deterministic()
    }

    fn collection_plan(
        plan: &DependencyPlan,
    ) -> ParallelDeterminismPlan {
        ParallelDeterminism::new(deterministic_policy())
            .for_dependency_plan(plan)
            .expect("deterministic collection plan must be valid")
    }

    /// Collect values in an arbitrary arrival order and return the canonical
    /// dependency-first compilation-unit identifiers.
    fn collect_order(
        plan: &ParallelDeterminismPlan,
        arrival_order: &[&str],
    ) -> Vec<String> {
        let results = arrival_order
            .iter()
            .map(|name| {
                (id(name), String::from(*name))
            });

        plan.collect_results(results)
            .expect("every planned unit must have exactly one result")
            .into_iter()
            .map(|result| {
                let (unit, _value) = result.into_parts();
                unit.as_str().to_owned()
            })
            .collect()
    }

    // ------------------------------------------------------------------------
    // Canonical policy integration
    // ------------------------------------------------------------------------

    #[test]
    fn adapter_preserves_canonical_determinism_policy() {
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
        assert!(!adapter.requires_stable_reduction());
    }

    #[test]
    fn logical_key_policy_is_preserved() {
        let policy = DeterminismPolicy::new(
            DeterminismMode::Deterministic,
            ObservableOrder::LogicalKeyOrder,
            NumericReproducibility::StableReduction,
        );

        let adapter = ParallelDeterminism::new(policy);

        assert!(adapter.requires_determinism());
        assert!(!adapter.requires_reproducibility());
        assert!(!adapter.requires_source_order());
        assert!(adapter.requires_logical_key_order());
        assert!(adapter.requires_stable_reduction());
    }

    #[test]
    fn combining_policies_never_weakens_reproducibility() {
        let unspecified = DeterminismPolicy::unspecified();
        let deterministic = DeterminismPolicy::deterministic();
        let reproducible = DeterminismPolicy::reproducible();

        assert_eq!(
            reproducible.combine(unspecified),
            reproducible
        );

        assert_eq!(
            reproducible.combine(deterministic),
            reproducible
        );

        assert_eq!(
            deterministic.combine(unspecified),
            deterministic
        );
    }

    // ------------------------------------------------------------------------
    // Dependency ordering
    // ------------------------------------------------------------------------

    #[test]
    fn dependency_order_is_preserved() {
        let plan = build_plan(
            &["application", "library", "core"],
            &[
                ("application", "library"),
                ("library", "core"),
            ],
        );

        let ordered = plan.ordered_units();

        let core_position = ordered
            .iter()
            .position(|unit| unit.as_str() == "core")
            .expect("core must be present");

        let library_position = ordered
            .iter()
            .position(|unit| unit.as_str() == "library")
            .expect("library must be present");

        let application_position = ordered
            .iter()
            .position(|unit| unit.as_str() == "application")
            .expect("application must be present");

        assert!(core_position < library_position);
        assert!(library_position < application_position);
    }

    #[test]
    fn independent_units_have_stable_canonical_order() {
        let first = build_plan(
            &["zeta", "alpha", "mu", "beta"],
            &[],
        );

        let second = build_plan(
            &["beta", "mu", "alpha", "zeta"],
            &[],
        );

        assert_eq!(
            first.ordered_units(),
            second.ordered_units()
        );
    }

    #[test]
    fn dependency_registration_order_does_not_change_the_plan() {
        let identifiers = [
            "application",
            "left",
            "right",
            "core",
        ];

        let first = build_plan(
            &identifiers,
            &[
                ("application", "left"),
                ("application", "right"),
                ("left", "core"),
                ("right", "core"),
            ],
        );

        let second = build_plan(
            &identifiers,
            &[
                ("right", "core"),
                ("left", "core"),
                ("application", "right"),
                ("application", "left"),
            ],
        );

        assert_eq!(
            first.ordered_units(),
            second.ordered_units()
        );

        assert_eq!(
            first.waves().len(),
            second.waves().len()
        );

        for (left, right) in first.waves().iter().zip(second.waves()) {
            assert_eq!(left.index(), right.index());
            assert_eq!(left.units(), right.units());
        }
    }

    #[test]
    fn non_ordering_relationships_do_not_create_compile_order() {
        let mut builder = DependencyContextBuilder::new();

        builder
            .add_unit(unit("alpha"))
            .expect("alpha registration");

        builder
            .add_unit(unit("beta"))
            .expect("beta registration");

        // Data flow is a semantic relationship, not automatically a
        // compile-time ordering dependency.
        builder
            .add_dependency(
                id("alpha"),
                id("beta"),
                DependencyKind::DataFlow,
            )
            .expect("data-flow relationship registration");

        let context = builder
            .build()
            .expect("non-ordering relationship must be valid");

        let plan = DependencyPlan::from_context(&context)
            .expect("non-ordering relationship must not create a cycle");

        assert_eq!(
            plan.ordered_units(),
            &[id("alpha"), id("beta")]
        );
    }

    // ------------------------------------------------------------------------
    // Result-order determinism
    // ------------------------------------------------------------------------

    #[test]
    fn result_order_is_independent_of_arrival_order() {
        let plan = build_plan(
            &[
                "application",
                "library",
                "utility",
                "core",
            ],
            &[
                ("application", "library"),
                ("library", "core"),
            ],
        );

        let collection = collection_plan(&plan);

        let forward = [
            "application",
            "library",
            "utility",
            "core",
        ];

        let reverse = [
            "core",
            "utility",
            "library",
            "application",
        ];

        assert_eq!(
            collect_order(&collection, &forward),
            collect_order(&collection, &reverse)
        );

        assert_eq!(
            collect_order(&collection, &forward),
            plan.ordered_units()
                .iter()
                .map(|unit| unit.as_str().to_owned())
                .collect::<Vec<_>>()
        );
    }

    /// Generate every permutation of a small test input.
    ///
    /// This helper is deliberately test-only. Its finite test input does not
    /// impose a production limit on compilation units or result entries.
    fn visit_permutations(
        values: &mut [String],
        position: usize,
        visit: &mut impl FnMut(&[String]),
    ) {
        if position == values.len() {
            visit(values);
            return;
        }

        for index in position..values.len() {
            values.swap(position, index);

            visit_permutations(
                values,
                position + 1,
                visit,
            );

            values.swap(position, index);
        }
    }

    #[test]
    fn every_completion_permutation_has_the_same_result_order() {
        let plan = build_plan(
            &[
                "application",
                "library",
                "utility",
                "core",
            ],
            &[
                ("application", "library"),
                ("library", "core"),
            ],
        );

        let collection = collection_plan(&plan);

        let expected: Vec<String> = plan
            .ordered_units()
            .iter()
            .map(|unit| unit.as_str().to_owned())
            .collect();

        let mut arrivals = vec![
            String::from("application"),
            String::from("library"),
            String::from("utility"),
            String::from("core"),
        ];

        let mut permutations_checked = 0usize;

        visit_permutations(
            &mut arrivals,
            0,
            &mut |permutation| {
                let input: Vec<&str> = permutation
                    .iter()
                    .map(String::as_str)
                    .collect();

                let actual = collect_order(
                    &collection,
                    &input,
                );

                assert_eq!(
                    actual,
                    expected,
                    "result order changed for arrival permutation: {permutation:?}"
                );

                permutations_checked += 1;
            },
        );

        assert_eq!(permutations_checked, 24);
    }

    #[test]
    fn logical_order_keys_are_stable_across_equivalent_plans() {
        let first = build_plan(
            &["application", "library", "core"],
            &[
                ("application", "library"),
                ("library", "core"),
            ],
        );

        let second = build_plan(
            &["core", "library", "application"],
            &[
                ("library", "core"),
                ("application", "library"),
            ],
        );

        let first_collection = collection_plan(&first);
        let second_collection = collection_plan(&second);

        assert_eq!(
            first_collection.ordered_units(),
            second_collection.ordered_units()
        );

        for unit in first_collection.ordered_units() {
            assert_eq!(
                first_collection.order_key(unit),
                second_collection.order_key(unit),
                "logical ordering key changed for {unit}"
            );
        }
    }

    // ------------------------------------------------------------------------
    // Empty and singleton boundaries
    // ------------------------------------------------------------------------

    #[test]
    fn empty_dependency_plan_has_empty_result_order() {
        let context = DependencyContext::new();

        let plan = DependencyPlan::from_context(&context)
            .expect("empty dependency context must be valid");

        let collection = collection_plan(&plan);

        assert!(collection.is_empty());
        assert_eq!(collection.unit_count(), 0);

        let results = collection
            .collect_results(
                Vec::<(CompilationUnitId, ())>::new()
            )
            .expect("empty result collection must succeed");

        assert!(results.is_empty());
    }

    #[test]
    fn singleton_dependency_plan_preserves_its_only_result() {
        let plan = build_plan(&["single"], &[]);
        let collection = collection_plan(&plan);

        let results = collection
            .collect_results(vec![
                (id("single"), String::from("completed")),
            ])
            .expect("singleton result collection must succeed");

        assert_eq!(results.len(), 1);

        let (unit, value) = results
            .into_iter()
            .next()
            .expect("singleton result must exist")
            .into_parts();

        assert_eq!(unit.as_str(), "single");
        assert_eq!(value, "completed");
    }

    // ------------------------------------------------------------------------
    // Error behavior
    // ------------------------------------------------------------------------

    #[test]
    fn duplicate_results_are_rejected() {
        let plan = build_plan(&["module"], &[]);
        let collection = collection_plan(&plan);

        let result = collection.collect_results(vec![
            (id("module"), 1usize),
            (id("module"), 2usize),
        ]);

        assert!(matches!(
            result,
            Err(ParallelDeterminismError::DuplicateResult { .. })
        ));
    }

    #[test]
    fn unknown_results_are_rejected() {
        let plan = build_plan(&["module"], &[]);
        let collection = collection_plan(&plan);

        let result = collection.collect_results(vec![
            (id("unknown"), 1usize),
        ]);

        assert!(matches!(
            result,
            Err(ParallelDeterminismError::UnknownResultUnit { .. })
        ));
    }

    #[test]
    fn missing_results_are_rejected() {
        let plan = build_plan(
            &["first", "second"],
            &[],
        );

        let collection = collection_plan(&plan);

        let result = collection.collect_results(vec![
            (id("first"), 1usize),
        ]);

        assert!(matches!(
            result,
            Err(ParallelDeterminismError::MissingResult { .. })
        ));
    }

    // ------------------------------------------------------------------------
    // Resource and architecture independence
    // ------------------------------------------------------------------------

    #[test]
    fn collection_order_does_not_depend_on_worker_identity() {
        let plan = build_plan(
            &["application", "library", "core"],
            &[
                ("application", "library"),
                ("library", "core"),
            ],
        );

        let collection = collection_plan(&plan);

        // These represent different possible completion sequences from
        // different workers. Worker identity is deliberately not part of
        // the canonical collection key.
        let completion_a = [
            "application",
            "library",
            "core",
        ];

        let completion_b = [
            "core",
            "application",
            "library",
        ];

        assert_eq!(
            collect_order(&collection, &completion_a),
            collect_order(&collection, &completion_b)
        );
    }

    #[test]
    fn compilation_unit_identity_is_not_a_physical_resource_identity() {
        let plan = build_plan(
            &["quantum.module", "classical.module", "ai.module"],
            &[],
        );

        let collection = collection_plan(&plan);

        let results = collection
            .collect_results(vec![
                (id("ai.module"), "ai"),
                (id("quantum.module"), "quantum"),
                (id("classical.module"), "classical"),
            ])
            .expect("domain-independent result collection must succeed");

        let identifiers: Vec<String> = results
            .into_iter()
            .map(|result| {
                let (unit, _) = result.into_parts();
                unit.as_str().to_owned()
            })
            .collect();

        assert_eq!(
            identifiers,
            vec![
                "ai.module",
                "classical.module",
                "quantum.module",
            ]
        );
    }
}
