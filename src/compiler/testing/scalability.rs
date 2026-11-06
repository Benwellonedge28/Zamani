
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Compiler Scalability Testing
//!
//! Path:
//!     src/compiler/testing/scalability.rs
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
//! Provides a reusable streaming test harness for evaluating whether
//! compiler subsystems preserve correctness as workloads grow.
//!
//! The harness executes actual caller-supplied work. It does not infer
//! scalability merely from successful allocation, construction of metadata,
//! or the existence of a large numeric parameter.
//!
//! It supports:
//!
//! - Empty and minimal workloads.
//! - Progressively growing workloads.
//! - Streaming workloads.
//! - Caller-defined compiler operations.
//! - Explicit operational budgets.
//! - Shared compiler budget accounting.
//! - Workload failure reporting.
//! - Expected-workload-count verification.
//! - Checked work counters.
//! - Deterministic workload ordering when the caller supplies it.
//! - Resource exhaustion reporting.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! THIS MODULE OWNS:
//!
//! - Scalability-test execution.
//! - Streaming workload accounting.
//! - Scalability-test result reporting.
//! - Expected workload completion checks.
//! - Explicit test failure classification.
//! - Integration adapters through caller-supplied closures.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - Compiler configuration.
//! - Resource-budget semantics.
//! - Physical memory allocation.
//! - Hardware discovery.
//! - Target capability semantics.
//! - Source grammar.
//! - Parsing.
//! - Semantic analysis.
//! - Ownership analysis.
//! - Canonical IR.
//! - Optimization.
//! - Hardware partitioning.
//! - Backend implementation.
//! - Runtime execution.
//! - Distributed scheduling.
//!
//! Existing authorities remain:
//!
//! - compiler::configuration::budget
//! - compiler::context
//! - compiler::testing::fixtures
//! - compiler::testing::conformance
//! - compiler::testing::differential
//! - compiler::pipeline
//! - compiler::ir
//! - compiler::planning
//! - compiler::backends
//! - compiler::artifacts
//!
//! ============================================================================
//! POCO-REAF AND SCALABILITY
//! ============================================================================
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This module introduces no universal maximum for:
//!
//! - Workload items.
//! - Source bytes.
//! - Compilation units.
//! - Modules.
//! - Functions.
//! - IR operations.
//! - IR blocks.
//! - Optimization passes.
//! - Computing domains.
//! - Hardware targets.
//! - Devices.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Distributed nodes.
//! - Tensor rank.
//! - Tensor dimensions.
//!
//! Workloads are processed incrementally.
//!
//! A finite computer cannot execute literally infinite work. The meaning
//! of scalability here is that no arbitrary language or test-harness
//! ceiling is introduced. Execution proceeds subject to available
//! implementation resources and explicitly configured operational budgets.
//!
//! Resource exhaustion is an explicit test failure, not a successful
//! completion and not a language-level restriction.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! This module does not reorder caller-supplied workloads.
//!
//! It does not introduce random test ordering, timestamps in test
//! expectations, environment-dependent workload selection, or global
//! mutable state.
//!
//! Elapsed time is reported as an observation only. It must not be used
//! as a deterministic correctness assertion.
//!
//! ============================================================================
//! INTEGRATION
//! ============================================================================
//!
//! The caller supplies actual compiler work through a closure.
//!
//! Examples:
//!
//! - Parse a fixture using the existing frontend.
//! - Compile a fixture using the existing compiler API.
//! - Verify a canonical IR module.
//! - Execute an optimizer pass.
//! - Lower an IR module through an existing backend.
//! - Validate an artifact.
//! - Run a conformance case.
//! - Compare multiple target realizations.
//!
//! The caller may supply a BudgetTracker obtained from the existing
//! CompilationBudget or CompilationContext.
//!
//! This module does not construct a second compiler context or budget.
//!
//! ============================================================================
//! FAILURE CONTRACT
//! ============================================================================
//!
//! A scalability run succeeds only when:
//!
//! 1. Every supplied item is processed successfully.
//! 2. No configured budget is exceeded.
//! 3. The processed count is representable.
//! 4. An optional expected item count is satisfied.
//!
//! A workload callback failure stops the run immediately.
//!
//! Budget exhaustion stops the run immediately.
//!
//! No failure is converted into a successful partial result.
//!
//! ============================================================================

use core::fmt;
use std::error::Error;
use std::time::{Duration, Instant};

use crate::compiler::configuration::budget::{
    BudgetDimension,
    BudgetError,
    BudgetTracker,
};

/// Public API version for the scalability testing harness.
///
/// This is independent of the Zamani language, grammar, AST, IR,
/// artifact, and runtime versions.
pub const SCALABILITY_TESTING_API_VERSION: u16 = 1;

// ============================================================================
// OPTIONS
// ============================================================================

/// Configuration for one scalability test run.
///
/// The expected item count is optional because some streaming workloads
/// are intentionally open-ended.
///
/// A budget tracker is optional. When supplied, it is the existing
/// compiler budget tracker and is shared with other compiler stages.
///
/// The supplied budget dimension defines what one workload item consumes.
/// The harness consumes one unit for each attempted item.
///
/// A workload adapter must choose a dimension whose unit is meaningful
/// for its operation.
#[derive(Clone, Debug)]
pub struct ScalabilityRunOptions<'a> {
    expected_items: Option<u128>,
    budget: Option<(&'a BudgetTracker, BudgetDimension)>,
}

impl<'a> Default for ScalabilityRunOptions<'a> {
    fn default() -> Self {
        Self {
            expected_items: None,
            budget: None,
        }
    }
}

impl<'a> ScalabilityRunOptions<'a> {
    /// Creates options without an explicit expected count or budget.
    ///
    /// This does not create an unlimited physical resource guarantee.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the expected number of workload items.
    ///
    /// The expected count is a test assertion, not a universal workload
    /// ceiling.
    #[must_use]
    pub fn with_expected_items(mut self, expected: u128) -> Self {
        self.expected_items = Some(expected);
        self
    }

    /// Uses an existing shared compiler budget tracker.
    ///
    /// Each successfully attempted workload item consumes one unit from
    /// the supplied dimension before its callback is executed.
    ///
    /// The tracker is not cloned into a new accounting ledger.
    #[must_use]
    pub fn with_budget(
        mut self,
        tracker: &'a BudgetTracker,
        dimension: BudgetDimension,
    ) -> Self {
        self.budget = Some((tracker, dimension));
        self
    }

    /// Returns the expected item count, if configured.
    #[must_use]
    pub const fn expected_items(&self) -> Option<u128> {
        self.expected_items
    }

    /// Returns whether this run has an explicit operational budget.
    #[must_use]
    pub const fn has_budget(&self) -> bool {
        self.budget.is_some()
    }
}

// ============================================================================
// REPORT
// ============================================================================

/// Successful result of a completed scalability workload.
///
/// `processed_items` is the number of callback invocations that completed
/// successfully.
///
/// `elapsed` is observational telemetry. It is not a reproducible-build
/// fingerprint and must not be used as a semantic correctness value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScalabilityReport {
    processed_items: u128,
    elapsed: Duration,
}

impl ScalabilityReport {
    /// Returns the number of successfully processed items.
    #[must_use]
    pub const fn processed_items(&self) -> u128 {
        self.processed_items
    }

    /// Returns the observed elapsed duration.
    #[must_use]
    pub const fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// Returns whether the workload contained no items.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.processed_items == 0
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by a scalability test run.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ScalabilityError {
    /// The shared compiler budget rejected another workload item.
    BudgetExceeded {
        /// Number of callbacks that completed before the failure.
        processed_items: u128,

        /// Original compiler budget error.
        source: BudgetError,
    },

    /// The caller-supplied workload operation failed.
    WorkloadFailed {
        /// Number of callbacks that completed before the failure.
        processed_items: u128,

        /// Human-readable explanation from the caller.
        reason: String,
    },

    /// The workload counter exceeded its representation range.
    ///
    /// The harness never wraps or saturates this counter.
    CounterOverflow {
        /// Number of callbacks completed before overflow was detected.
        processed_items: u128,
    },

    /// The workload completed with a different count than expected.
    ExpectedCountMismatch {
        /// Number of items expected by the caller.
        expected: u128,

        /// Number of items actually processed.
        actual: u128,
    },
}

impl fmt::Display for ScalabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BudgetExceeded {
                processed_items,
                source,
            } => write!(
                formatter,
                "scalability workload exhausted its operational budget \
                 after {processed_items} completed items: {source}"
            ),

            Self::WorkloadFailed {
                processed_items,
                reason,
            } => write!(
                formatter,
                "scalability workload failed after \
                 {processed_items} completed items: {reason}"
            ),

            Self::CounterOverflow { processed_items } => write!(
                formatter,
                "scalability workload counter overflow after \
                 {processed_items} completed items"
            ),

            Self::ExpectedCountMismatch { expected, actual } => write!(
                formatter,
                "scalability workload count mismatch: \
                 expected {expected}, processed {actual}"
            ),
        }
    }
}

impl Error for ScalabilityError {}

/// Result type for scalability test operations.
pub type ScalabilityResult<T> = Result<T, ScalabilityError>;

// ============================================================================
// STREAMING EXECUTION
// ============================================================================

/// Executes a caller-supplied workload incrementally.
///
/// The iterator is consumed exactly once and in its supplied order.
/// Items are not collected into an intermediate `Vec`.
///
/// The callback receives ownership of each item. This allows callers to
/// move large test inputs through the harness without requiring an
/// additional clone.
///
/// When a budget is configured, one unit is consumed before the callback
/// is invoked. This accounts for attempted work, including an item whose
/// callback subsequently fails.
///
/// A failed callback stops iteration immediately.
///
/// The function does not catch panics. Test infrastructure should allow
/// unexpected panics to remain visible as test failures.
///
/// # Type parameters
///
/// - `I`: One workload item.
/// - `It`: A single-pass or reusable iterator.
/// - `F`: The operation that exercises the actual compiler subsystem.
/// - `E`: The callback's error type.
///
/// # Errors
///
/// Returns an error when:
///
/// - The shared budget rejects an item.
/// - The callback fails.
/// - The processed-item counter overflows.
/// - The completed count differs from the optional expected count.
///
/// # Example
///
/// ```ignore
/// let report = run_streaming(
///     (0..workload_size),
///     ScalabilityRunOptions::new()
///         .with_expected_items(workload_size as u128),
///     |item| {
///         compile_test_item(item)?;
///         Ok::<(), MyCompilerError>(())
///     },
/// )?;
/// ```
pub fn run_streaming<I, It, F, E>(
    workload: It,
    options: ScalabilityRunOptions<'_>,
    mut operation: F,
) -> ScalabilityResult<ScalabilityReport>
where
    It: IntoIterator<Item = I>,
    F: FnMut(I) -> Result<(), E>,
    E: fmt::Display,
{
    let started = Instant::now();
    let mut processed_items = 0_u128;

    for item in workload {
        let next_count = processed_items
            .checked_add(1)
            .ok_or(ScalabilityError::CounterOverflow {
                processed_items,
            })?;

        if let Some((tracker, dimension)) = &options.budget {
            tracker
                .consume(dimension.clone(), 1)
                .map_err(|source| ScalabilityError::BudgetExceeded {
                    processed_items,
                    source,
                })?;
        }

        operation(item).map_err(|error| ScalabilityError::WorkloadFailed {
            processed_items,
            reason: error.to_string(),
        })?;

        processed_items = next_count;
    }

    if let Some(expected) = options.expected_items {
        if processed_items != expected {
            return Err(ScalabilityError::ExpectedCountMismatch {
                expected,
                actual: processed_items,
            });
        }
    }

    Ok(ScalabilityReport {
        processed_items,
        elapsed: started.elapsed(),
    })
}

// ============================================================================
// TEST SUPPORT
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::configuration::budget::{
        BudgetDimension,
        BudgetLimit,
        CompilationBudget,
    };

    use std::cell::Cell;

    /// These are representative test workloads, not maximum supported
    /// compiler workload sizes.
    const PROGRESSIVE_WORKLOADS: &[usize] = &[
        0,
        1,
        2,
        8,
        32,
        128,
        512,
        1024,
    ];

    #[test]
    fn empty_workload_completes_successfully() {
        let report = run_streaming(
            std::iter::empty::<usize>(),
            ScalabilityRunOptions::new().with_expected_items(0),
            |_| Ok::<(), &'static str>(()),
        )
        .expect("empty workload should complete");

        assert!(report.is_empty());
        assert_eq!(report.processed_items(), 0);
    }

    #[test]
    fn singleton_workload_executes_actual_operation() {
        let executed = Cell::new(0_u128);

        let report = run_streaming(
            std::iter::once(42_u64),
            ScalabilityRunOptions::new().with_expected_items(1),
            |value| {
                assert_eq!(value, 42);
                executed.set(executed.get() + 1);
                Ok::<(), &'static str>(())
            },
        )
        .expect("singleton workload should complete");

        assert_eq!(report.processed_items(), 1);
        assert_eq!(executed.get(), 1);
    }

    #[test]
    fn progressively_larger_workloads_preserve_every_item() {
        for &workload_size in PROGRESSIVE_WORKLOADS {
            let executed = Cell::new(0_usize);

            let report = run_streaming(
                0..workload_size,
                ScalabilityRunOptions::new()
                    .with_expected_items(workload_size as u128),
                |_| {
                    executed.set(executed.get() + 1);
                    Ok::<(), &'static str>(())
                },
            )
            .expect("workload should complete");

            assert_eq!(report.processed_items(), workload_size as u128);
            assert_eq!(executed.get(), workload_size);
        }
    }

    #[test]
    fn workload_order_is_preserved() {
        let mut observed = Vec::new();

        let report = run_streaming(
            [7_u64, 2, 9, 1, 5],
            ScalabilityRunOptions::new().with_expected_items(5),
            |item| {
                observed.push(item);
                Ok::<(), &'static str>(())
            },
        )
        .expect("workload should complete");

        assert_eq!(report.processed_items(), 5);
        assert_eq!(observed, vec![7, 2, 9, 1, 5]);
    }

    #[test]
    fn iterator_is_consumed_incrementally() {
        let generated = Cell::new(0_usize);
        let executed = Cell::new(0_usize);

        let workload = std::iter::from_fn(|| {
            let current = generated.get();

            if current == 128 {
                return None;
            }

            generated.set(current + 1);
            Some(current)
        });

        let report = run_streaming(
            workload,
            ScalabilityRunOptions::new().with_expected_items(128),
            |_| {
                executed.set(executed.get() + 1);
                Ok::<(), &'static str>(())
            },
        )
        .expect("streaming workload should complete");

        assert_eq!(generated.get(), 128);
        assert_eq!(executed.get(), 128);
        assert_eq!(report.processed_items(), 128);
    }

    #[test]
    fn bounded_budget_stops_workload_without_silent_truncation() {
        let dimension = BudgetDimension::ir_entities();

        let budget = CompilationBudget::new().with_limit(
            dimension.clone(),
            BudgetLimit::bounded(3),
        );

        let tracker = budget.tracker();
        let executed = Cell::new(0_usize);

        let result = run_streaming(
            0..8,
            ScalabilityRunOptions::new()
                .with_expected_items(8)
                .with_budget(&tracker, dimension.clone()),
            |_| {
                executed.set(executed.get() + 1);
                Ok::<(), &'static str>(())
            },
        );

        assert!(matches!(
            result,
            Err(ScalabilityError::BudgetExceeded {
                processed_items: 3,
                ..
            })
        ));

        assert_eq!(executed.get(), 3);
        assert_eq!(tracker.consumed(&dimension).unwrap(), 3);
    }

    #[test]
    fn unlimited_budget_does_not_impose_a_test_harness_ceiling() {
        let dimension = BudgetDimension::ir_entities();
        let tracker = CompilationBudget::unlimited().tracker();

        let report = run_streaming(
            0..1024,
            ScalabilityRunOptions::new()
                .with_expected_items(1024)
                .with_budget(&tracker, dimension.clone()),
            |_| Ok::<(), &'static str>(()),
        )
        .expect("unlimited operational budget should permit this workload");

        assert_eq!(report.processed_items(), 1024);
        assert_eq!(tracker.consumed(&dimension).unwrap(), 1024);
        assert_eq!(tracker.remaining(&dimension).unwrap(), None);
    }

    #[test]
    fn budget_is_shared_with_other_compiler_consumers() {
        let dimension = BudgetDimension::pass_applications();

        let budget = CompilationBudget::new().with_limit(
            dimension.clone(),
            BudgetLimit::bounded(5),
        );

        let tracker = budget.tracker();

        tracker
            .consume(dimension.clone(), 2)
            .expect("another compiler stage consumes two units");

        let result = run_streaming(
            0..4,
            ScalabilityRunOptions::new()
                .with_expected_items(4)
                .with_budget(&tracker, dimension.clone()),
            |_| Ok::<(), &'static str>(()),
        );

        assert!(matches!(
            result,
            Err(ScalabilityError::BudgetExceeded {
                processed_items: 3,
                ..
            })
        ));

        assert_eq!(tracker.consumed(&dimension).unwrap(), 5);
    }

    #[test]
    fn callback_failure_stops_the_workload() {
        let executed = Cell::new(0_usize);

        let result = run_streaming(
            0..10,
            ScalabilityRunOptions::new(),
            |item| {
                executed.set(executed.get() + 1);

                if item == 4 {
                    return Err("injected compiler failure");
                }

                Ok::<(), &'static str>(())
            },
        );

        assert!(matches!(
            result,
            Err(ScalabilityError::WorkloadFailed {
                processed_items: 4,
                ..
            })
        ));

        assert_eq!(executed.get(), 5);
    }

    #[test]
    fn attempted_work_is_charged_even_when_callback_fails() {
        let dimension = BudgetDimension::optimizer_work();
        let tracker = CompilationBudget::unlimited().tracker();

        let result = run_streaming(
            0..10,
            ScalabilityRunOptions::new()
                .with_budget(&tracker, dimension.clone()),
            |item| {
                if item == 2 {
                    Err("optimizer failure")
                } else {
                    Ok::<(), &'static str>(())
                }
            },
        );

        assert!(result.is_err());

        assert_eq!(
            tracker.consumed(&dimension).unwrap(),
            3,
            "the failed callback's attempted work must also be accounted"
        );
    }

    #[test]
    fn expected_count_mismatch_is_not_reported_as_success() {
        let result = run_streaming(
            0..4,
            ScalabilityRunOptions::new().with_expected_items(5),
            |_| Ok::<(), &'static str>(()),
        );

        assert!(matches!(
            result,
            Err(ScalabilityError::ExpectedCountMismatch {
                expected: 5,
                actual: 4,
            })
        ));
    }

    #[test]
    fn arbitrary_budget_dimensions_are_supported() {
        let dimension = BudgetDimension::new(
            "future_domain.scalability_work",
            "operations",
        )
        .expect("custom budget dimension");

        let tracker = CompilationBudget::unlimited().tracker();

        let report = run_streaming(
            0..16,
            ScalabilityRunOptions::new()
                .with_expected_items(16)
                .with_budget(&tracker, dimension.clone()),
            |_| Ok::<(), &'static str>(()),
        )
        .expect("custom dimension workload");

        assert_eq!(report.processed_items(), 16);
        assert_eq!(tracker.consumed(&dimension).unwrap(), 16);
    }

    #[test]
    fn elapsed_time_is_observational_only() {
        let report = run_streaming(
            0..1,
            ScalabilityRunOptions::new().with_expected_items(1),
            |_| Ok::<(), &'static str>(()),
        )
        .expect("workload");

        // Do not assert an exact duration: elapsed time is not deterministic.
        let _observed_duration = report.elapsed();
    }

    #[test]
    fn report_count_is_independent_of_elapsed_time() {
        let first = run_streaming(
            0..32,
            ScalabilityRunOptions::new().with_expected_items(32),
            |_| Ok::<(), &'static str>(()),
        )
        .expect("first run");

        let second = run_streaming(
            0..32,
            ScalabilityRunOptions::new().with_expected_items(32),
            |_| Ok::<(), &'static str>(()),
        )
        .expect("second run");

        assert_eq!(first.processed_items(), second.processed_items());
    }
}
