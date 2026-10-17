
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Classical Computing — Algorithms
//!
//! Path: `src/classical/algorithms/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! Status: Module-boundary implementation. Repository-wide integration and
//! compilation must pass before this module can be considered production
//! verified.
//!
//! ## Purpose
//!
//! This module is the public composition boundary for generic classical
//! algorithms implemented within `src/classical/algorithms/`.
//!
//! It registers and exposes:
//!
//! - reduction and folding;
//! - inclusive and exclusive prefix scans;
//! - stable and unstable sorting;
//! - linear and binary searching;
//! - predicate-based partitioning;
//! - balanced logical partition-range planning.
//!
//! The algorithms operate on caller-provided values, slices, and iterators.
//! They do not own source syntax, ASTs, canonical IR, collection storage,
//! numeric semantics, hardware discovery, scheduling, or runtime execution.
//!
//! ## Architectural authority
//!
//! - `grammar/spec/classical.md` defines normative classical-computing
//!   semantics and POCO-REAF requirements.
//! - `grammar/spec/data.md` defines data, ordering, and partition semantics.
//! - `grammar/concurrency/data-parallel.g4` owns data-parallel source syntax,
//!   including `parallel::reduce`, `parallel::scan`, and
//!   `parallel::partition` operation categories.
//! - The frontend owns parsing, AST construction, and source locations.
//! - Semantic analysis owns type checking, callable resolution, effects,
//!   ordering, determinism, and operation legality.
//! - The existing canonical IR owns semantic operation representation.
//! - `src/classical/collections/` owns collection representations and
//!   collection invariants.
//! - Shared execution and resource infrastructure owns capabilities,
//!   budgets, placement, scheduling, cancellation, and resource failures.
//!
//! This module does not create a replacement AST, type system, IR, runtime,
//! resource registry, scheduler, or collection abstraction.
//!
//! ## POCO-REAF and scalability
//!
//! The algorithms introduce no universal source-language ceilings for:
//!
//! - input element counts;
//! - logical partition counts;
//! - CPU, GPU, FPGA, accelerator, or node counts;
//! - memory capacity or execution width;
//! - tensor rank or collection dimensions.
//!
//! Logical work size and physical execution width are different concepts.
//! Execution infrastructure may choose a sequential, parallel, vectorized,
//! accelerator, distributed, or other implementation when semantically legal
//! and supported by available resources.
//!
//! This module does not promise infinite physical resources or guarantee that
//! an arbitrary computation terminates. Concrete operations remain subject
//! to representable types, available memory, execution budgets, target
//! capabilities, and explicit resource policies.
//!
//! ## Module ownership
//!
//! - `reduction.rs`: sequential reductions and folds.
//! - `scan.rs`: lazy sequential prefix-scan adapters.
//! - `sorting.rs`: slice-sorting adapters.
//! - `searching.rs`: generic sequential search operations.
//! - `partitioning.rs`: in-place predicate partitioning and lazy logical
//!   range planning.
//!
//! The modules remain independently responsible for their algorithm
//! implementations and function-level contracts. This file only composes
//! their public interfaces.
//!
//! ## Integration contract
//!
//! 1. The parent `crate::classical` module must register `algorithms`.
//! 2. The crate root must expose `classical` if public crate-level access is
//!    intended.
//! 3. Frontend and semantic analysis must validate source operations before
//!    lowering them to the repository's canonical IR.
//! 4. Lowering may select these implementations only when their contracts
//!    match the source operation's required semantics.
//! 5. Collection adapters may delegate to these algorithms when their
//!    storage and borrowing contracts permit it.
//! 6. Parallel execution must remain in the shared concurrency/execution
//!    infrastructure; these sequential adapters must not silently introduce
//!    scheduling or reorder operations.
//!
//! These are integration requirements, not claims that every connection
//! currently exists or has been verified.
//!
//! ## Compatibility
//!
//! The module exposes both named submodules and selected root-level
//! re-exports. Existing callers can use explicit paths such as
//! `classical::algorithms::sorting::sort`, while concise imports may use
//! `classical::algorithms::sort`.
//!
//! A public API should be removed or have its behavior changed only through
//! an explicit compatibility review.
//!
//! ## Verification
//!
//! This module's verification gate includes:
//!
//! - compiling all declared submodules;
//! - testing the root-level re-exports;
//! - checking that the public API introduces no duplicate semantic owners;
//! - running the workspace's supported formatting, check, test, and lint
//!   commands;
//! - testing relevant empty-input, boundary, determinism, and scalability
//!   behavior in the individual algorithm modules.

// =============================================================================
// Algorithm modules
// =============================================================================

/// Sequential reductions and folds.
pub mod reduction;

/// Lazy sequential prefix scans.
pub mod scan;

/// Stable and unstable sorting adapters.
pub mod sorting;

/// Generic sequential search operations.
pub mod searching;

/// Predicate partitioning and logical partition-range planning.
pub mod partitioning;

// =============================================================================
// Public reduction API
// =============================================================================

pub use reduction::{
    fold,
    reduce,
    reduce_with,
    reduce_with_identity,
    try_fold,
    try_reduce,
    try_reduce_with_identity,
};

// =============================================================================
// Public scan API
// =============================================================================

pub use scan::{
    exclusive_scan,
    inclusive_scan,
    try_inclusive_scan,
    ExclusiveScan,
    InclusiveScan,
    TryInclusiveScan,
};

// =============================================================================
// Public sorting API
// =============================================================================

pub use sorting::{
    sort,
    sort_by,
    sort_by_key,
    sort_unstable,
    sort_unstable_by,
    sort_unstable_by_key,
};

// =============================================================================
// Public searching API
// =============================================================================

pub use searching::{
    binary_search,
    binary_search_by,
    binary_search_by_key,
    find_iter,
    linear_search,
    linear_search_by,
    linear_search_by_key,
    lower_bound,
    lower_bound_by,
    upper_bound,
    upper_bound_by,
};

// =============================================================================
// Public partitioning API
// =============================================================================

pub use partitioning::{
    partition_by,
    stable_partition_by,
    PartitionPlan,
    PartitionPlanError,
    PartitionRanges,
};

#[cfg(test)]
mod tests {
    use super::{
        inclusive_scan, partition_by, reduce, stable_partition_by, PartitionPlan,
        PartitionPlanError,
    };

    #[test]
    fn root_api_exposes_reduction() {
        assert_eq!(reduce([1_i64, 2, 3, 4]), Some(10));
    }

    #[test]
    fn root_api_exposes_inclusive_scan() {
        let values: Vec<_> =
            inclusive_scan([1_i64, 2, 3, 4], |left, right| left + right)
                .collect();

        assert_eq!(values, [1, 3, 6, 10]);
    }

    #[test]
    fn root_api_exposes_predicate_partitioning() {
        let mut values = [1, 2, 3, 4, 5, 6];

        let boundary = partition_by(&mut values, |value| value % 2 == 0);

        assert_eq!(boundary, 3);
        assert!(values[..boundary].iter().all(|value| value % 2 == 0));
        assert!(values[boundary..].iter().all(|value| value % 2 != 0));
    }

    #[test]
    fn stable_partition_preserves_relative_order() {
        let mut values = [1, 2, 3, 4, 5, 6];

        let boundary =
            stable_partition_by(&mut values, |value| value % 2 == 0);

        assert_eq!(boundary, 3);
        assert_eq!(values, [2, 4, 6, 1, 3, 5]);
    }

    #[test]
    fn logical_partition_plan_covers_domain_without_gaps() {
        let plan = PartitionPlan::new(10, 3).expect("positive partition count");

        assert_eq!(plan.length(), 10);
        assert_eq!(plan.partition_count(), 3);

        let ranges: Vec<_> = plan.ranges().collect();

        assert_eq!(ranges, [0..4, 4..7, 7..10]);
        assert_eq!(ranges.first().map(|range| range.start), Some(0));
        assert_eq!(ranges.last().map(|range| range.end), Some(10));

        for adjacent in ranges.windows(2) {
            assert_eq!(adjacent[0].end, adjacent[1].start);
        }
    }

    #[test]
    fn empty_domain_has_no_ranges() {
        let plan = PartitionPlan::new(0, 4).expect("positive partition count");

        assert!(plan.is_empty());
        assert_eq!(plan.partition_count(), 0);
        assert_eq!(plan.ranges().count(), 0);
    }

    #[test]
    fn zero_requested_partitions_is_rejected() {
        assert!(matches!(
            PartitionPlan::new(10, 0),
            Err(PartitionPlanError::ZeroPartitions)
        ));
    }

    #[test]
    fn requested_partitions_do_not_create_empty_ranges() {
        let plan = PartitionPlan::new(3, 100).expect("positive partition count");

        assert_eq!(plan.partition_count(), 3);
        assert_eq!(plan.ranges().collect::<Vec<_>>(), [0..1, 1..2, 2..3]);
    }
}
