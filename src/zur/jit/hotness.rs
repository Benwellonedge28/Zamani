
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — JIT Hotness Detection
//!
//! Path: src/zur/jit/hotness.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Provides configurable, evidence-based hotness detection for JIT
//! compilation units.
//!
//! Hotness detection determines whether a compilation unit has accumulated
//! sufficient recent execution activity to justify requesting an optional
//! JIT compilation or specialization.
//!
//! This module consumes execution observations from:
//!
//!     crate::zur::jit::profiling
//!
//! It produces decisions for:
//!
//!     crate::zur::jit::engine
//!
//! It does not implement compilation, optimization, scheduling,
//! specialization, code generation or execution.
//!
//! # OWNERSHIP
//!
//! This module owns:
//!
//! - Hotness policies.
//! - Configurable hotness thresholds.
//! - Threshold evaluation.
//! - Successive-snapshot comparison.
//! - Execution-activity windows.
//! - Hotness decisions.
//! - Explicit profile-baseline management.
//! - Hotness decision explanations.
//!
//! It does not own:
//!
//! - Execution observations.
//! - Profile collection.
//! - Compiler profiling.
//! - Compiler optimization.
//! - JIT compilation.
//! - Code generation.
//! - Hardware discovery.
//! - Hardware allocation.
//! - Runtime scheduling.
//! - Specialization correctness.
//! - Deoptimization.
//! - Code invalidation.
//! - Physical execution.
//! - Computational-domain semantics.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! # CANONICAL INTEGRATION
//!
//! JIT compilation identity:
//!     crate::zur::jit::engine::JitCacheKey
//!
//! JIT compilation request:
//!     crate::zur::jit::engine::JitCompilationReason
//!
//! Execution observations:
//!     crate::zur::jit::profiling::JitProfileSnapshot
//!
//! Execution profile:
//!     crate::zur::jit::profiling::JitProfileEntrySnapshot
//!
//! JIT engine:
//!     crate::zur::jit::engine::JitEngine
//!
//! Runtime configuration:
//!     crate::zur::config
//!
//! Runtime operational policy:
//!     crate::zur::policy
//!
//! # GRAMMAR BOUNDARY
//!
//! Relevant grammar:
//!
//!     grammar/execution/runtime.g4
//!     grammar/execution/runtime-capabilities.g4
//!     grammar/compile/optimization.g4
//!     grammar/resources/portability.g4
//!
//! Relevant specifications:
//!
//!     grammar/spec/portability.md
//!     grammar/specification/execution-model.md
//!     grammar/specification/semantic-model.md
//!
//! The grammar and semantic-analysis subsystem establish source-level
//! optimization intent and execution contracts.
//!
//! This module implements runtime operational hotness evaluation.
//!
//! It does not parse grammar, inspect AST nodes or reinterpret source
//! optimization declarations.
//!
//! # SCALABILITY
//!
//! No arbitrary limits are imposed on:
//!
//! - Computational domains.
//! - Compilation units.
//! - Invocation counts.
//! - Execution observations.
//! - Runtime sessions.
//! - Providers.
//! - Devices.
//! - Processors.
//! - Workers.
//! - Hardware architectures.
//!
//! Thresholds are explicitly configurable.
//!
//! Numeric counters use u128 and checked subtraction.
//!
//! Actual storage and execution remain subject to available resources.
//!
//! Literal infinite computation is not promised.
//!
//! # SAFETY
//!
//! - No unsafe Rust.
//! - No raw pointers.
//! - No global mutable state.
//! - No hidden worker threads.
//! - No implicit compilation.
//! - No fabricated execution observations.
//! - No implicit hardware discovery.
//! - No automatic resource reservation.
//! - No automatic specialization.
//!
//! # IMPORTANT INVARIANTS
//!
//! 1. Profiling remains the authoritative source of observations.
//! 2. Hotness thresholds are configurable.
//! 3. Historical lifetime counts are not automatically treated as
//!    current execution activity.
//! 4. First observations establish a baseline rather than declaring
//!    a compilation unit hot.
//! 5. Profile counter resets cannot generate artificial hotness.
//! 6. Insufficient observations remain distinguishable from coldness.
//! 7. Failed invocations are not counted as successful invocations.
//! 8. Hotness does not prove compilation feasibility.
//! 9. Hotness does not authorize a change to program semantics.
//! 10. Only the JIT engine performs compilation.
//!
//! ============================================================================

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::num::NonZeroU128;
use std::sync::Mutex;

use crate::zur::error::{ZurError, ZurResult};

use crate::zur::jit::engine::{
    JitCacheKey,
    JitCompilationReason,
};

use crate::zur::jit::profiling::{
    JitProfileEntrySnapshot,
    JitProfileSnapshot,
};

// =============================================================================
// API VERSION
// =============================================================================

/// Public API version for ZUR JIT hotness detection.
pub const JIT_HOTNESS_API_VERSION: u16 = 1;

/// Component identity used in structured errors.
const COMPONENT: &str = "zur.jit.hotness";

// =============================================================================
// HOTNESS MATCHING
// =============================================================================

/// Defines how multiple hotness criteria are combined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum HotnessMatchMode {
    /// Every configured criterion must be satisfied.
    ///
    /// This is the conservative default.
    #[default]
    All,

    /// At least one configured criterion must be satisfied.
    Any,
}

// =============================================================================
// HOTNESS POLICY
// =============================================================================

/// Immutable policy defining when a compilation unit is considered hot.
///
/// At least one successful invocation threshold is required.
///
/// An additional elapsed-time threshold may be configured.
///
/// Elapsed-time observations originate from the existing profiler.
/// They represent sampled completed invocations, not exclusively
/// successful invocations.
///
/// A configurable timing-sample requirement establishes the minimum
/// evidence needed before elapsed-time decisions are trusted.
///
/// No default numeric hotness threshold is imposed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotnessPolicy {
    minimum_successful_invocations: NonZeroU128,

    minimum_total_elapsed_ns: Option<NonZeroU128>,

    minimum_timing_samples: Option<NonZeroU128>,

    match_mode: HotnessMatchMode,
}

impl HotnessPolicy {
    /// Creates a policy with an explicitly supplied successful-invocation
    /// threshold.
    ///
    /// A compilation unit becomes eligible for hotness when its observed
    /// successful invocation count reaches this threshold.
    ///
    /// The threshold applies to an observation interval, not the
    /// compilation unit's entire lifetime.
    #[must_use]
    pub const fn new(
        minimum_successful_invocations: NonZeroU128,
    ) -> Self {
        Self {
            minimum_successful_invocations,
            minimum_total_elapsed_ns: None,
            minimum_timing_samples: None,
            match_mode: HotnessMatchMode::All,
        }
    }

    /// Adds a minimum sampled execution-time threshold.
    ///
    /// The duration is expressed in nanoseconds.
    ///
    /// The measurement is taken from the existing JIT profiler.
    ///
    /// When this criterion is enabled, at least one timing sample
    /// is required unless a larger minimum is explicitly configured.
    #[must_use]
    pub const fn with_minimum_total_elapsed_ns(
        mut self,
        minimum: NonZeroU128,
    ) -> Self {
        self.minimum_total_elapsed_ns = Some(minimum);
        self
    }

    /// Configures the minimum number of timing samples required
    /// before evaluating elapsed-time hotness.
    ///
    /// This setting requires an elapsed-time criterion.
    #[must_use]
    pub const fn with_minimum_timing_samples(
        mut self,
        minimum: NonZeroU128,
    ) -> Self {
        self.minimum_timing_samples = Some(minimum);
        self
    }

    /// Configures how multiple criteria are combined.
    #[must_use]
    pub const fn with_match_mode(
        mut self,
        mode: HotnessMatchMode,
    ) -> Self {
        self.match_mode = mode;
        self
    }

    /// Returns the successful-invocation threshold.
    #[must_use]
    pub const fn minimum_successful_invocations(
        &self,
    ) -> NonZeroU128 {
        self.minimum_successful_invocations
    }

    /// Returns the optional elapsed-time threshold.
    #[must_use]
    pub const fn minimum_total_elapsed_ns(
        &self,
    ) -> Option<NonZeroU128> {
        self.minimum_total_elapsed_ns
    }

    /// Returns the optional minimum timing-sample requirement.
    #[must_use]
    pub const fn minimum_timing_samples(
        &self,
    ) -> Option<NonZeroU128> {
        self.minimum_timing_samples
    }

    /// Returns the criterion matching mode.
    #[must_use]
    pub const fn match_mode(
        &self,
    ) -> HotnessMatchMode {
        self.match_mode
    }

    /// Validates the complete policy.
    pub fn validate(&self) -> ZurResult<()> {
        if self.minimum_timing_samples.is_some()
            && self.minimum_total_elapsed_ns.is_none()
        {
            return Err(
                ZurError::configuration(
                    "minimum timing samples require an elapsed-time criterion",
                )
                .with_component(COMPONENT),
            );
        }

        Ok(())
    }
}

// =============================================================================
// HOTNESS STATUS
// =============================================================================

/// Result of evaluating one compilation unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HotnessStatus {
    /// Sufficient evidence establishes that the unit is hot.
    Hot,

    /// Sufficient evidence establishes that the unit is below
    /// its configured thresholds.
    Cold,

    /// Insufficient observations are available for a reliable decision.
    InsufficientEvidence,
}

impl HotnessStatus {
    /// Returns whether the unit satisfies its hotness policy.
    #[must_use]
    pub const fn is_hot(self) -> bool {
        matches!(self, Self::Hot)
    }

    /// Returns whether the unit is conclusively cold.
    #[must_use]
    pub const fn is_cold(self) -> bool {
        matches!(self, Self::Cold)
    }

    /// Returns whether additional observations are needed.
    #[must_use]
    pub const fn needs_more_evidence(self) -> bool {
        matches!(self, Self::InsufficientEvidence)
    }
}

// =============================================================================
// HOTNESS REASONS
// =============================================================================

/// Explains why a hotness decision was produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HotnessReason {
    /// No previous observation existed.
    ///
    /// The current profile establishes the baseline.
    InitialObservation,

    /// The profiler's counters decreased between observations.
    ///
    /// This indicates a reset or profile replacement.
    ProfileReset,

    /// Successful invocation activity reached the configured threshold.
    InvocationThresholdReached,

    /// Elapsed execution time reached the configured threshold.
    ElapsedTimeThresholdReached,

    /// All configured criteria were satisfied.
    AllCriteriaSatisfied,

    /// The configured thresholds were not satisfied.
    BelowThreshold,

    /// Insufficient timing observations are available.
    InsufficientTimingSamples,
}

// =============================================================================
// OBSERVATION WINDOW
// =============================================================================

/// Immutable execution activity observed between two snapshots.
///
/// These values are deltas, not lifetime totals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HotnessWindowMetrics {
    successful_invocations: u128,

    completed_timing_samples: u128,

    total_elapsed_ns: u128,
}

impl HotnessWindowMetrics {
    /// Returns successful invocations in the observation interval.
    #[must_use]
    pub const fn successful_invocations(self) -> u128 {
        self.successful_invocations
    }

    /// Returns completed timing samples in the interval.
    #[must_use]
    pub const fn completed_timing_samples(self) -> u128 {
        self.completed_timing_samples
    }

    /// Returns accumulated sampled elapsed nanoseconds.
    #[must_use]
    pub const fn total_elapsed_ns(self) -> u128 {
        self.total_elapsed_ns
    }
}

// =============================================================================
// HOTNESS DECISION
// =============================================================================

/// Immutable decision for one JIT compilation unit.
///
/// A hotness decision is an optimization recommendation.
///
/// It does not establish that:
///
/// - JIT is enabled;
/// - the compiler is available;
/// - a backend is compatible;
/// - resources can be acquired;
/// - specialization is valid;
/// - executable memory is available;
/// - execution will succeed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotnessDecision {
    key: JitCacheKey,

    domain_identity: String,

    status: HotnessStatus,

    reason: HotnessReason,

    metrics: HotnessWindowMetrics,
}

impl HotnessDecision {
    /// Returns the canonical JIT compilation-unit identity.
    #[must_use]
    pub fn key(&self) -> &JitCacheKey {
        &self.key
    }

    /// Returns the namespaced computational-domain identity.
    #[must_use]
    pub fn domain_identity(&self) -> &str {
        &self.domain_identity
    }

    /// Returns the evaluated hotness status.
    #[must_use]
    pub const fn status(&self) -> HotnessStatus {
        self.status
    }

    /// Returns the explanation for the decision.
    #[must_use]
    pub const fn reason(&self) -> HotnessReason {
        self.reason
    }

    /// Returns the execution activity metrics for this interval.
    #[must_use]
    pub const fn metrics(&self) -> HotnessWindowMetrics {
        self.metrics
    }

    /// Returns whether this decision recommends requesting JIT compilation.
    #[must_use]
    pub const fn recommends_jit(&self) -> bool {
        self.status.is_hot()
    }

    /// Returns the JIT compilation reason when hotness is established.
    ///
    /// The returned reason can be supplied to the existing JIT engine.
    ///
    /// No compilation is initiated by this method.
    #[must_use]
    pub const fn compilation_reason(
        &self,
    ) -> Option<JitCompilationReason> {
        if self.recommends_jit() {
            Some(JitCompilationReason::HotPath)
        } else {
            None
        }
    }
}

// =============================================================================
// PROFILE BASELINE
// =============================================================================

/// Internal cumulative counters used to calculate observation deltas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProfileBaseline {
    successful_invocations: u128,

    timing_samples: u128,

    total_elapsed_ns: u128,
}

impl ProfileBaseline {
    /// Creates a baseline from a canonical profiler snapshot.
    fn from_profile(
        profile: &JitProfileEntrySnapshot,
    ) -> Self {
        Self {
            successful_invocations: profile.succeeded(),
            timing_samples: profile.timing_samples(),
            total_elapsed_ns: profile.total_elapsed_ns(),
        }
    }

    /// Returns whether all cumulative counters are monotonic.
    fn is_monotonic_from(
        &self,
        previous: &Self,
    ) -> bool {
        self.successful_invocations
            >= previous.successful_invocations
            && self.timing_samples >= previous.timing_samples
            && self.total_elapsed_ns >= previous.total_elapsed_ns
    }

    /// Calculates the observation interval.
    fn difference(
        &self,
        previous: &Self,
    ) -> Option<HotnessWindowMetrics> {
        Some(HotnessWindowMetrics {
            successful_invocations: self.successful_invocations
                .checked_sub(previous.successful_invocations)?,

            completed_timing_samples: self.timing_samples
                .checked_sub(previous.timing_samples)?,

            total_elapsed_ns: self.total_elapsed_ns
                .checked_sub(previous.total_elapsed_ns)?,
        })
    }
}

// =============================================================================
// TRACKER
// =============================================================================

/// Thread-safe, explicitly owned JIT hotness tracker.
///
/// The tracker compares successive profiler snapshots.
///
/// It does not create background workers, timers or global state.
///
/// The caller determines the observation cadence.
///
/// A tracker may be shared between threads because its internal
/// baseline state is protected by a standard-library mutex.
pub struct HotnessTracker {
    policy: HotnessPolicy,

    baselines: Mutex<BTreeMap<JitCacheKey, ProfileBaseline>>,
}

impl fmt::Debug for HotnessTracker {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("HotnessTracker")
            .field("policy", &self.policy)
            .finish_non_exhaustive()
    }
}

impl HotnessTracker {
    /// Creates a tracker using an explicitly configured policy.
    pub fn new(
        policy: HotnessPolicy,
    ) -> ZurResult<Self> {
        policy.validate()?;

        Ok(Self {
            policy,
            baselines: Mutex::new(BTreeMap::new()),
        })
    }

    /// Returns the immutable hotness policy.
    #[must_use]
    pub const fn policy(&self) -> &HotnessPolicy {
        &self.policy
    }

    /// Observes a canonical profiler snapshot.
    ///
    /// The first observation for a compilation unit establishes
    /// its baseline and returns InsufficientEvidence.
    ///
    /// Subsequent observations calculate counter deltas and evaluate
    /// the configured policy.
    ///
    /// A profile reset establishes a new baseline instead of producing
    /// an artificial hotness signal.
    ///
    /// Profiles absent from the current snapshot are removed from
    /// the tracker's baseline collection.
    ///
    /// The returned decisions follow the deterministic ordering
    /// supplied by the canonical profile snapshot.
    pub fn observe(
        &self,
        snapshot: &JitProfileSnapshot,
    ) -> ZurResult<Vec<HotnessDecision>> {
        let mut baselines = self.baselines.lock().map_err(|_| {
            ZurError::internal(
                "JIT hotness baseline synchronization was poisoned",
            )
            .with_component(COMPONENT)
            .with_operation("observe")
        })?;

        let mut observed_keys = BTreeSet::new();

        let mut decisions = Vec::with_capacity(
            snapshot.profiles().len(),
        );

        for profile in snapshot.profiles() {
            let key = profile.key();

            observed_keys.insert(key.clone());

            let current = ProfileBaseline::from_profile(profile);

            let previous = baselines.get(key).copied();

            let decision = match previous {
                None => {
                    baselines.insert(key.clone(), current);

                    self.decision(
                        profile,
                        HotnessStatus::InsufficientEvidence,
                        HotnessReason::InitialObservation,
                        HotnessWindowMetrics {
                            successful_invocations: 0,
                            completed_timing_samples: 0,
                            total_elapsed_ns: 0,
                        },
                    )
                }

                Some(previous) if !current.is_monotonic_from(&previous) => {
                    baselines.insert(key.clone(), current);

                    self.decision(
                        profile,
                        HotnessStatus::InsufficientEvidence,
                        HotnessReason::ProfileReset,
                        HotnessWindowMetrics {
                            successful_invocations: 0,
                            completed_timing_samples: 0,
                            total_elapsed_ns: 0,
                        },
                    )
                }

                Some(previous) => {
                    let metrics = current.difference(&previous)
                        .ok_or_else(|| {
                            ZurError::internal(
                                "JIT hotness counter difference failed",
                            )
                            .with_component(COMPONENT)
                            .with_operation("calculate_window")
                        })?;

                    baselines.insert(key.clone(), current);

                    let (status, reason) =
                        self.evaluate_metrics(metrics);

                    self.decision(
                        profile,
                        status,
                        reason,
                        metrics,
                    )
                }
            };

            decisions.push(decision);
        }

        // Remove stale baselines for profiles no longer tracked
        // by the canonical profiler.
        //
        // The profiler remains the authoritative owner of profile
        // retention and removal.
        baselines.retain(|key, _| observed_keys.contains(key));

        Ok(decisions)
    }

    /// Removes one compilation unit's baseline.
    ///
    /// Call this when a profile is explicitly removed and the caller
    /// needs to invalidate its corresponding hotness history.
    ///
    /// Returns true when a baseline existed.
    pub fn forget(
        &self,
        key: &JitCacheKey,
    ) -> ZurResult<bool> {
        let mut baselines = self.baselines.lock().map_err(|_| {
            ZurError::internal(
                "JIT hotness baseline synchronization was poisoned",
            )
            .with_component(COMPONENT)
            .with_operation("forget")
        })?;

        Ok(baselines.remove(key).is_some())
    }

    /// Clears all retained observation baselines.
    ///
    /// This does not modify the canonical profiler or its observations.
    pub fn reset(&self) -> ZurResult<()> {
        let mut baselines = self.baselines.lock().map_err(|_| {
            ZurError::internal(
                "JIT hotness baseline synchronization was poisoned",
            )
            .with_component(COMPONENT)
            .with_operation("reset")
        })?;

        baselines.clear();

        Ok(())
    }

    /// Returns the number of currently retained baselines.
    pub fn tracked_units(&self) -> ZurResult<usize> {
        let baselines = self.baselines.lock().map_err(|_| {
            ZurError::internal(
                "JIT hotness baseline synchronization was poisoned",
            )
            .with_component(COMPONENT)
            .with_operation("tracked_units")
        })?;

        Ok(baselines.len())
    }

    /// Evaluates an observation interval against the policy.
    fn evaluate_metrics(
        &self,
        metrics: HotnessWindowMetrics,
    ) -> (HotnessStatus, HotnessReason) {
        let invocation_satisfied =
            metrics.successful_invocations
                >= self.policy.minimum_successful_invocations.get();

        let elapsed_requirement =
            self.policy.minimum_total_elapsed_ns;

        let elapsed_satisfied = match elapsed_requirement {
            Some(threshold) => {
                let minimum_samples = self.policy
                    .minimum_timing_samples
                    .map_or(1, NonZeroU128::get);

                if metrics.completed_timing_samples < minimum_samples {
                    return (
                        HotnessStatus::InsufficientEvidence,
                        HotnessReason::InsufficientTimingSamples,
                    );
                }

                Some(metrics.total_elapsed_ns >= threshold.get())
            }

            None => None,
        };

        match self.policy.match_mode {
            HotnessMatchMode::All => {
                if !invocation_satisfied
                    || elapsed_satisfied == Some(false)
                {
                    return (
                        HotnessStatus::Cold,
                        HotnessReason::BelowThreshold,
                    );
                }

                if elapsed_satisfied == Some(true) {
                    (
                        HotnessStatus::Hot,
                        HotnessReason::AllCriteriaSatisfied,
                    )
                } else {
                    (
                        HotnessStatus::Hot,
                        HotnessReason::InvocationThresholdReached,
                    )
                }
            }

            HotnessMatchMode::Any => {
                if invocation_satisfied {
                    return (
                        HotnessStatus::Hot,
                        HotnessReason::InvocationThresholdReached,
                    );
                }

                if elapsed_satisfied == Some(true) {
                    return (
                        HotnessStatus::Hot,
                        HotnessReason::ElapsedTimeThresholdReached,
                    );
                }

                if elapsed_satisfied.is_none() {
                    return (
                        HotnessStatus::Cold,
                        HotnessReason::BelowThreshold,
                    );
                }

                (
                    HotnessStatus::Cold,
                    HotnessReason::BelowThreshold,
                )
            }
        }
    }

    /// Constructs a complete immutable decision.
    fn decision(
        &self,
        profile: &JitProfileEntrySnapshot,
        status: HotnessStatus,
        reason: HotnessReason,
        metrics: HotnessWindowMetrics,
    ) -> HotnessDecision {
        HotnessDecision {
            key: profile.key().clone(),
            domain_identity: profile.domain_identity().to_owned(),
            status,
            reason,
            metrics,
        }
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::num::NonZeroU64;

    use crate::zur::jit::profiling::{
        JitInvocationOutcome,
        JitProfileCollector,
        JitProfileConfig,
        JitSamplingPolicy,
    };

    fn nonzero(value: u128) -> NonZeroU128 {
        NonZeroU128::new(value).expect("test value must be nonzero")
    }

    fn key() -> JitCacheKey {
        JitCacheKey::new("compiler.test.function")
            .expect("test cache key must be valid")
    }

    fn collector() -> JitProfileCollector {
        JitProfileCollector::new(
            JitProfileConfig::new()
                .with_sampling(
                    JitSamplingPolicy::EveryNth(
                        NonZeroU64::new(1).expect("nonzero"),
                    ),
                ),
        )
    }

    fn record_success(
        collector: &JitProfileCollector,
        cache_key: &JitCacheKey,
    ) {
        let token = collector
            .begin(cache_key, "classical.cpu")
            .expect("begin profiling");

        collector
            .finish(token, JitInvocationOutcome::Succeeded)
            .expect("finish profiling");
    }

    #[test]
    fn api_version_is_available() {
        assert_eq!(JIT_HOTNESS_API_VERSION, 1);
    }

    #[test]
    fn policy_requires_an_explicit_threshold() {
        let policy = HotnessPolicy::new(nonzero(10));

        assert_eq!(
            policy.minimum_successful_invocations().get(),
            10
        );

        assert!(policy.validate().is_ok());
    }

    #[test]
    fn timing_samples_require_an_elapsed_time_criterion() {
        let policy = HotnessPolicy::new(nonzero(10))
            .with_minimum_timing_samples(nonzero(5));

        assert!(policy.validate().is_err());
    }

    #[test]
    fn first_snapshot_establishes_baseline() {
        let collector = collector();
        let cache_key = key();

        record_success(&collector, &cache_key);

        let tracker = HotnessTracker::new(
            HotnessPolicy::new(nonzero(1)),
        )
        .expect("valid tracker");

        let snapshot = collector.snapshot()
            .expect("snapshot");

        let decisions = tracker.observe(&snapshot)
            .expect("observe");

        assert_eq!(decisions.len(), 1);

        assert_eq!(
            decisions[0].status(),
            HotnessStatus::InsufficientEvidence
        );

        assert_eq!(
            decisions[0].reason(),
            HotnessReason::InitialObservation
        );
    }

    #[test]
    fn successive_snapshots_detect_hot_execution() {
        let collector = collector();
        let cache_key = key();

        let tracker = HotnessTracker::new(
            HotnessPolicy::new(nonzero(3)),
        )
        .expect("valid tracker");

        let first = collector.snapshot()
            .expect("first snapshot");

        tracker.observe(&first)
            .expect("establish baseline");

        record_success(&collector, &cache_key);
        record_success(&collector, &cache_key);
        record_success(&collector, &cache_key);

        let second = collector.snapshot()
            .expect("second snapshot");

        let decisions = tracker.observe(&second)
            .expect("evaluate hotness");

        assert_eq!(
            decisions[0].status(),
            HotnessStatus::Hot
        );

        assert!(decisions[0].recommends_jit());

        assert_eq!(
            decisions[0].compilation_reason(),
            Some(JitCompilationReason::HotPath)
        );

        assert_eq!(
            decisions[0].metrics().successful_invocations(),
            3
        );
    }

    #[test]
    fn below_threshold_is_cold() {
        let collector = collector();
        let cache_key = key();

        let tracker = HotnessTracker::new(
            HotnessPolicy::new(nonzero(5)),
        )
        .expect("valid tracker");

        tracker.observe(
            &collector.snapshot().expect("baseline"),
        )
        .expect("establish baseline");

        record_success(&collector, &cache_key);

        let decisions = tracker.observe(
            &collector.snapshot().expect("snapshot"),
        )
        .expect("evaluate");

        assert_eq!(
            decisions[0].status(),
            HotnessStatus::Cold
        );

        assert!(!decisions[0].recommends_jit());
    }

    #[test]
    fn failed_invocations_do_not_increase_successful_activity() {
        let collector = collector();
        let cache_key = key();

        let tracker = HotnessTracker::new(
            HotnessPolicy::new(nonzero(2)),
        )
        .expect("valid tracker");

        tracker.observe(
            &collector.snapshot().expect("baseline"),
        )
        .expect("baseline established");

        let token = collector
            .begin(&cache_key, "classical.cpu")
            .expect("begin");

        collector
            .finish(token, JitInvocationOutcome::Failed)
            .expect("finish");

        let decisions = tracker.observe(
            &collector.snapshot().expect("snapshot"),
        )
        .expect("evaluate");

        assert_eq!(
            decisions[0].metrics().successful_invocations(),
            0
        );

        assert_eq!(
            decisions[0].status(),
            HotnessStatus::Cold
        );
    }

    #[test]
    fn elapsed_time_requires_sufficient_samples() {
        let collector = collector();
        let cache_key = key();

        let tracker = HotnessTracker::new(
            HotnessPolicy::new(nonzero(100))
                .with_minimum_total_elapsed_ns(nonzero(1))
                .with_minimum_timing_samples(nonzero(2)),
        )
        .expect("valid tracker");

        tracker.observe(
            &collector.snapshot().expect("baseline"),
        )
        .expect("baseline");

        record_success(&collector, &cache_key);

        let decisions = tracker.observe(
            &collector.snapshot().expect("snapshot"),
        )
        .expect("evaluate");

        assert_eq!(
            decisions[0].status(),
            HotnessStatus::InsufficientEvidence
        );

        assert_eq!(
            decisions[0].reason(),
            HotnessReason::InsufficientTimingSamples
        );
    }

    #[test]
    fn counter_reset_does_not_create_false_hotness() {
        let collector = collector();
        let cache_key = key();

        let tracker = HotnessTracker::new(
            HotnessPolicy::new(nonzero(1)),
        )
        .expect("valid tracker");

        record_success(&collector, &cache_key);

        tracker.observe(
            &collector.snapshot().expect("initial"),
        )
        .expect("baseline");

        collector.remove(&cache_key)
            .expect("remove profile");

        record_success(&collector, &cache_key);

        let decisions = tracker.observe(
            &collector.snapshot().expect("replacement"),
        )
        .expect("observe replacement");

        assert_eq!(
            decisions[0].status(),
            HotnessStatus::InsufficientEvidence
        );

        assert_eq!(
            decisions[0].reason(),
            HotnessReason::ProfileReset
        );
    }

    #[test]
    fn explicit_forget_removes_baseline() {
        let collector = collector();
        let cache_key = key();

        let tracker = HotnessTracker::new(
            HotnessPolicy::new(nonzero(10)),
        )
        .expect("valid tracker");

        tracker.observe(
            &collector.snapshot().expect("baseline"),
        )
        .expect("observe");

        assert_eq!(
            tracker.tracked_units().expect("count"),
            0
        );

        record_success(&collector, &cache_key);

        tracker.observe(
            &collector.snapshot().expect("snapshot"),
        )
        .expect("observe");

        assert_eq!(
            tracker.tracked_units().expect("count"),
            1
        );

        assert!(tracker.forget(&cache_key).expect("forget"));

        assert_eq!(
            tracker.tracked_units().expect("count"),
            0
        );
    }

    #[test]
    fn reset_clears_all_baselines() {
        let collector = collector();
        let cache_key = key();

        let tracker = HotnessTracker::new(
            HotnessPolicy::new(nonzero(10)),
        )
        .expect("valid tracker");

        record_success(&collector, &cache_key);

        tracker.observe(
            &collector.snapshot().expect("snapshot"),
        )
        .expect("observe");

        tracker.reset().expect("reset");

        assert_eq!(
            tracker.tracked_units().expect("count"),
            0
        );
    }

    #[test]
    fn different_domains_do_not_require_different_hotness_implementations() {
        let collector = collector();

        let domains = [
            "classical.cpu",
            "quantum.qpu",
            "hdl.fpga",
            "ai.tensor",
            "hybrid.execution",
            "distributed.execution",
            "photonic.processor",
        ];

        let tracker = HotnessTracker::new(
            HotnessPolicy::new(nonzero(2)),
        )
        .expect("valid tracker");

        let baseline = collector.snapshot()
            .expect("baseline");

        tracker.observe(&baseline)
            .expect("establish baseline");

        for (index, domain) in domains.iter().enumerate() {
            let cache_key = JitCacheKey::new(
                format!("extension.test.unit.{index}"),
            )
            .expect("valid key");

            let token = collector.begin(&cache_key, domain)
                .expect("begin");

            collector.finish(
                token,
                JitInvocationOutcome::Succeeded,
            )
            .expect("finish");
        }

        let snapshot = collector.snapshot()
            .expect("snapshot");

        let decisions = tracker.observe(&snapshot)
            .expect("evaluate");

        assert_eq!(decisions.len(), domains.len());

        assert!(decisions.iter().all(|decision| {
            decision.status() == HotnessStatus::Cold
        }));
    }
}
