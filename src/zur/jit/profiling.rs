
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — JIT Profiling
//!
//! Path: src/zur/jit/profiling.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97+
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Provides runtime observations and streaming aggregation for
//! just-in-time execution.
//!
//! Profiling records operational execution facts that may be
//! consumed by hotness analysis, adaptive execution, diagnostics,
//! optimization policy and runtime observability.
//!
//! Profiling does not compile, optimize, schedule or execute programs.
//!
//! # OWNERSHIP
//!
//! This module owns:
//!
//! - JIT invocation observations.
//! - Execution-duration measurements.
//! - Invocation outcome counts.
//! - Configurable sampling.
//! - Per-compilation-unit aggregation.
//! - Profile snapshots.
//! - Explicit profile-cardinality budgets.
//! - Profile overflow behavior.
//! - Thread-safe profile collection.
//! - Profile observation provenance.
//!
//! This module does not own:
//!
//! - Compiler intermediate representations.
//! - Compiler optimization.
//! - Compiler lowering.
//! - Backend registration.
//! - Code generation.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Runtime scheduling.
//! - JIT compilation.
//! - Hotness policy decisions.
//! - Code specialization.
//! - Deoptimization.
//! - Code invalidation.
//! - Executable memory.
//! - Source-language profiling semantics.
//!
//! # CANONICAL INTEGRATION
//!
//! JIT compilation identity:
//!     crate::zur::jit::engine::JitCacheKey
//!
//! JIT compilation:
//!     crate::zur::jit::engine
//!
//! Runtime configuration:
//!     crate::zur::config
//!
//! Runtime operational policy:
//!     crate::zur::policy
//!
//! Canonical compiler profiling:
//!     crate::compiler::profiling
//!
//! Runtime observability:
//!     crate::zur::observability
//!
//! JIT hotness analysis:
//!     crate::zur::jit::hotness
//!
//! JIT specialization:
//!     crate::zur::jit::specialization
//!
//! # IMPORTANT OWNERSHIP DISTINCTION
//!
//! Compiler profiling measures compiler operations and compiler stages.
//!
//! This module measures execution of JIT compilation units.
//!
//! These observations must not be conflated.
//!
//! Compiler profiling remains owned by:
//!     crate::compiler::profiling
//!
//! Runtime profiling remains owned by:
//!     crate::zur::jit::profiling
//!
//! # SCALABILITY
//!
//! No universal maximum is imposed on:
//!
//! - Computational domains.
//! - Compilation units.
//! - Invocations.
//! - Profile observations.
//! - Runtime sessions.
//! - Providers.
//! - Devices.
//! - Workers.
//! - Execution targets.
//! - Profile identifiers.
//!
//! An optional maximum number of tracked compilation units
//! may be supplied as an operational memory budget.
//!
//! Sampling is configurable.
//!
//! Aggregation retains summaries rather than raw sample histories.
//!
//! All numeric accumulation uses checked arithmetic.
//!
//! Physical memory and processing capacity remain real constraints.
//! Literal infinite computation is not promised.
//!
//! # SAFETY
//!
//! - No unsafe Rust.
//! - No raw pointers.
//! - No global mutable state.
//! - No hidden worker threads.
//! - No unbounded observation history.
//! - No fabricated execution results.
//! - No implicit hardware discovery.
//! - No implicit resource allocation.
//! - No implicit compiler invocation.
//!
//! # OBSERVATIONAL NON-INTERFERENCE
//!
//! Profiling must not alter program semantics.
//!
//! Profiling failures are reported independently from execution
//! failures.
//!
//! Callers determine the appropriate observability failure policy.
//!
//! # INTEGRATION INVARIANTS
//!
//! 1. JIT cache identity comes from the existing JIT engine.
//! 2. Runtime profiling does not duplicate compiler measurements.
//! 3. Sampling never changes execution behavior.
//! 4. Failed invocations are not reported as successful.
//! 5. Cancelled invocations remain distinguishable.
//! 6. Hotness decisions remain outside this module.
//! 7. Profiling does not imply resource availability.
//! 8. Profiling observations do not establish feasibility.
//! 9. No arbitrary hardware or domain limits are introduced.
//! 10. Every retained profile can be explicitly inspected or removed.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::num::{NonZeroU64, NonZeroUsize};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::zur::jit::engine::JitCacheKey;

// =============================================================================
// API VERSION
// =============================================================================

/// Public API version of ZUR JIT profiling.
pub const JIT_PROFILING_API_VERSION: u16 = 1;

// =============================================================================
// RESULT
// =============================================================================

/// Result returned by JIT profiling operations.
pub type JitProfileResult<T> = Result<T, JitProfileError>;

// =============================================================================
// ERRORS
// =============================================================================

/// Errors produced by JIT profiling.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum JitProfileError {
    /// A computational-domain identity is invalid.
    InvalidDomainIdentity,

    /// A compilation unit was registered under conflicting domains.
    DomainIdentityMismatch,

    /// The configured number of tracked units has been exhausted.
    ProfileBudgetExceeded {
        /// Configured number of tracked units.
        maximum: usize,
    },

    /// A profiling counter exceeded its representable range.
    CounterOverflow,

    /// The profile synchronization primitive was poisoned.
    SynchronizationPoisoned,

    /// An invocation token belongs to another collector.
    ForeignInvocationToken,

    /// A profile with active invocations cannot be removed.
    ActiveInvocations,

    /// The requested profile does not exist.
    ProfileNotFound,
}

impl fmt::Display for JitProfileError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidDomainIdentity => {
                formatter.write_str(
                    "invalid JIT profiling domain identity",
                )
            }

            Self::DomainIdentityMismatch => {
                formatter.write_str(
                    "JIT profiling domain identity mismatch",
                )
            }

            Self::ProfileBudgetExceeded { maximum } => {
                write!(
                    formatter,
                    "JIT profiling unit budget exceeded: {maximum}"
                )
            }

            Self::CounterOverflow => {
                formatter.write_str(
                    "JIT profiling counter overflow",
                )
            }

            Self::SynchronizationPoisoned => {
                formatter.write_str(
                    "JIT profiling synchronization was poisoned",
                )
            }

            Self::ForeignInvocationToken => {
                formatter.write_str(
                    "JIT invocation token belongs to another collector",
                )
            }

            Self::ActiveInvocations => {
                formatter.write_str(
                    "cannot remove a profile with active invocations",
                )
            }

            Self::ProfileNotFound => {
                formatter.write_str(
                    "JIT execution profile was not found",
                )
            }
        }
    }
}

impl Error for JitProfileError {}

// =============================================================================
// DOMAIN IDENTITY
// =============================================================================

/// Validates an extensible computational-domain identifier.
///
/// Format:
///
///     namespace.segment.identifier
///
/// Permitted characters:
///
/// - Lowercase ASCII letters.
/// - ASCII digits.
/// - Hyphens.
/// - Underscores.
/// - Dots separating nonempty segments.
///
/// There is no arbitrary namespace-depth or identifier-length limit.
///
/// This validates identity structure only. It does not establish
/// domain support or execution capability.
fn validate_domain_identity(value: &str) -> bool {
    if value.is_empty() || value.trim() != value {
        return false;
    }

    let mut segments = 0usize;

    for segment in value.split('.') {
        if segment.is_empty() {
            return false;
        }

        if !segment.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'-'
                || byte == b'_'
        }) {
            return false;
        }

        if segment.starts_with('-')
            || segment.starts_with('_')
            || segment.ends_with('-')
            || segment.ends_with('_')
        {
            return false;
        }

        segments += 1;
    }

    segments >= 2
}

// =============================================================================
// SAMPLING
// =============================================================================

/// Configurable JIT profiling sampling strategy.
///
/// Sampling affects observation collection only.
///
/// It must never change whether an invocation executes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum JitSamplingPolicy {
    /// Measure every invocation.
    ///
    /// This is the default policy.
    #[default]
    EveryInvocation,

    /// Measure every Nth invocation for each compilation unit.
    ///
    /// The first sampled invocation is the Nth invocation.
    EveryNth(NonZeroU64),
}

impl JitSamplingPolicy {
    /// Determines whether an invocation should be sampled.
    ///
    /// `ordinal` is the one-based invocation number for its
    /// compilation unit.
    #[must_use]
    pub const fn should_sample(
        self,
        ordinal: u128,
    ) -> bool {
        match self {
            Self::EveryInvocation => true,

            Self::EveryNth(interval) => {
                ordinal % interval.get() as u128 == 0
            }
        }
    }
}

// =============================================================================
// OVERFLOW POLICY
// =============================================================================

/// Defines behavior when the configured profile-unit budget
/// is exhausted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum ProfileOverflowPolicy {
    /// Reject registration of a new profile.
    ///
    /// Existing profiles remain unchanged.
    #[default]
    RejectNew,

    /// Do not retain the new profile.
    ///
    /// Execution may continue, but its profiling observations
    /// are explicitly reported as unrecorded.
    DropNew,
}

// =============================================================================
// CONFIGURATION
// =============================================================================

/// Configuration for one JIT profiling collector.
///
/// An absent unit budget means that this component does not impose
/// an additional unit-count ceiling.
///
/// It does not mean unlimited physical memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JitProfileConfig {
    sampling: JitSamplingPolicy,

    max_tracked_units: Option<NonZeroUsize>,

    overflow_policy: ProfileOverflowPolicy,
}

impl Default for JitProfileConfig {
    fn default() -> Self {
        Self {
            sampling: JitSamplingPolicy::EveryInvocation,

            max_tracked_units: None,

            overflow_policy: ProfileOverflowPolicy::RejectNew,
        }
    }
}

impl JitProfileConfig {
    /// Creates the default profiling configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            sampling: JitSamplingPolicy::EveryInvocation,

            max_tracked_units: None,

            overflow_policy: ProfileOverflowPolicy::RejectNew,
        }
    }

    /// Configures the sampling strategy.
    #[must_use]
    pub const fn with_sampling(
        mut self,
        sampling: JitSamplingPolicy,
    ) -> Self {
        self.sampling = sampling;
        self
    }

    /// Configures an optional tracked-unit budget.
    ///
    /// `None` means no additional unit-count ceiling.
    #[must_use]
    pub const fn with_max_tracked_units(
        mut self,
        maximum: Option<NonZeroUsize>,
    ) -> Self {
        self.max_tracked_units = maximum;
        self
    }

    /// Configures behavior when the profile budget is exhausted.
    #[must_use]
    pub const fn with_overflow_policy(
        mut self,
        policy: ProfileOverflowPolicy,
    ) -> Self {
        self.overflow_policy = policy;
        self
    }

    /// Returns the configured sampling policy.
    #[must_use]
    pub const fn sampling(self) -> JitSamplingPolicy {
        self.sampling
    }

    /// Returns the optional tracked-unit budget.
    #[must_use]
    pub const fn max_tracked_units(
        self,
    ) -> Option<NonZeroUsize> {
        self.max_tracked_units
    }

    /// Returns the configured overflow policy.
    #[must_use]
    pub const fn overflow_policy(
        self,
    ) -> ProfileOverflowPolicy {
        self.overflow_policy
    }
}

// =============================================================================
// INVOCATION OUTCOME
// =============================================================================

/// Actual outcome of one JIT invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum JitInvocationOutcome {
    /// The invocation completed successfully.
    Succeeded,

    /// The invocation failed.
    Failed,

    /// The invocation was cancelled.
    Cancelled,

    /// The invocation ended without a normal completion result.
    Abandoned,
}

// =============================================================================
// PROFILE DATA
// =============================================================================

/// Internal aggregate for one JIT compilation unit.
#[derive(Debug, Clone)]
struct UnitProfile {
    domain_identity: String,

    invocations_started: u128,

    invocations_completed: u128,

    invocations_in_flight: u128,

    succeeded: u128,

    failed: u128,

    cancelled: u128,

    abandoned: u128,

    sampled_invocations: u128,

    timing_samples: u128,

    total_elapsed_ns: u128,

    minimum_elapsed_ns: Option<u128>,

    maximum_elapsed_ns: Option<u128>,
}

impl UnitProfile {
    fn new(domain_identity: &str) -> Self {
        Self {
            domain_identity: domain_identity.to_owned(),

            invocations_started: 0,

            invocations_completed: 0,

            invocations_in_flight: 0,

            succeeded: 0,

            failed: 0,

            cancelled: 0,

            abandoned: 0,

            sampled_invocations: 0,

            timing_samples: 0,

            total_elapsed_ns: 0,

            minimum_elapsed_ns: None,

            maximum_elapsed_ns: None,
        }
    }
}

// =============================================================================
// SHARED STATE
// =============================================================================

#[derive(Debug, Default)]
struct ProfileState {
    profiles: BTreeMap<JitCacheKey, UnitProfile>,

    dropped_invocations: u128,
}

// =============================================================================
// COLLECTOR
// =============================================================================

/// Thread-safe JIT execution profiling collector.
///
/// The collector is explicitly owned and dependency-injected.
///
/// It creates no global state and starts no background workers.
///
/// Multiple execution threads may submit observations concurrently.
#[derive(Debug, Clone)]
pub struct JitProfileCollector {
    config: JitProfileConfig,

    state: Arc<Mutex<ProfileState>>,
}

impl Default for JitProfileCollector {
    fn default() -> Self {
        Self::new(JitProfileConfig::default())
    }
}

impl JitProfileCollector {
    /// Creates a collector with the supplied configuration.
    #[must_use]
    pub fn new(config: JitProfileConfig) -> Self {
        Self {
            config,
            state: Arc::new(Mutex::new(ProfileState::default())),
        }
    }

    /// Returns the collector configuration.
    #[must_use]
    pub const fn config(&self) -> JitProfileConfig {
        self.config
    }

    /// Begins observing a JIT invocation.
    ///
    /// The cache key must come from the canonical JIT engine.
    ///
    /// The domain identity must describe the compilation unit's
    /// established computational domain.
    ///
    /// A returned token must be completed through `finish`.
    ///
    /// When the configured profile budget is exhausted:
    ///
    /// - RejectNew returns an explicit error.
    /// - DropNew returns a token whose observation is unrecorded.
    ///
    /// Neither policy prevents execution by itself.
    pub fn begin(
        &self,
        key: &JitCacheKey,
        domain_identity: &str,
    ) -> JitProfileResult<JitInvocationToken> {
        if !validate_domain_identity(domain_identity) {
            return Err(JitProfileError::InvalidDomainIdentity);
        }

        let mut state = self.state
            .lock()
            .map_err(|_| JitProfileError::SynchronizationPoisoned)?;

        if let Some(profile) = state.profiles.get(key) {
            if profile.domain_identity != domain_identity {
                return Err(JitProfileError::DomainIdentityMismatch);
            }
        } else {
            if let Some(maximum) = self.config.max_tracked_units {
                if state.profiles.len() >= maximum.get() {
                    match self.config.overflow_policy {
                        ProfileOverflowPolicy::RejectNew => {
                            return Err(
                                JitProfileError::ProfileBudgetExceeded {
                                    maximum: maximum.get(),
                                },
                            );
                        }

                        ProfileOverflowPolicy::DropNew => {
                            state.dropped_invocations = state
                                .dropped_invocations
                                .checked_add(1)
                                .ok_or(JitProfileError::CounterOverflow)?;

                            return Ok(JitInvocationToken {
                                collector: Arc::clone(&self.state),

                                key: key.clone(),

                                domain_identity: domain_identity.to_owned(),

                                started_at: None,

                                tracked: false,

                                sampled: false,
                            });
                        }
                    }
                }
            }

            state.profiles.insert(
                key.clone(),
                UnitProfile::new(domain_identity),
            );
        }

        let profile = state.profiles
            .get_mut(key)
            .ok_or(JitProfileError::ProfileNotFound)?;

        let next_ordinal = profile.invocations_started
            .checked_add(1)
            .ok_or(JitProfileError::CounterOverflow)?;

        let next_in_flight = profile.invocations_in_flight
            .checked_add(1)
            .ok_or(JitProfileError::CounterOverflow)?;

        let sampled = self.config.sampling
            .should_sample(next_ordinal);

        let next_sampled = if sampled {
            profile.sampled_invocations
                .checked_add(1)
                .ok_or(JitProfileError::CounterOverflow)?
        } else {
            profile.sampled_invocations
        };

        profile.invocations_started = next_ordinal;

        profile.invocations_in_flight = next_in_flight;

        profile.sampled_invocations = next_sampled;

        Ok(JitInvocationToken {
            collector: Arc::clone(&self.state),

            key: key.clone(),

            domain_identity: domain_identity.to_owned(),

            started_at: sampled.then(Instant::now),

            tracked: true,

            sampled,
        })
    }

    /// Completes an invocation and records its actual outcome.
    ///
    /// The invocation token is consumed, preventing accidental
    /// completion of the same invocation twice.
    ///
    /// Timing is recorded only when the invocation was sampled.
    ///
    /// This method does not determine whether execution itself
    /// succeeded. The caller must supply the actual outcome.
    pub fn finish(
        &self,
        token: JitInvocationToken,
        outcome: JitInvocationOutcome,
    ) -> JitProfileResult<JitProfileObservation> {
        if !Arc::ptr_eq(&self.state, &token.collector) {
            return Err(JitProfileError::ForeignInvocationToken);
        }

        let elapsed_ns = token.started_at
            .map(|started| started.elapsed().as_nanos());

        if !token.tracked {
            return Ok(JitProfileObservation {
                key: token.key,

                domain_identity: token.domain_identity,

                outcome,

                elapsed_ns: None,

                recorded: false,
            });
        }

        let mut state = self.state
            .lock()
            .map_err(|_| JitProfileError::SynchronizationPoisoned)?;

        let profile = state.profiles
            .get_mut(&token.key)
            .ok_or(JitProfileError::ProfileNotFound)?;

        if profile.invocations_in_flight == 0 {
            return Err(JitProfileError::CounterOverflow);
        }

        let completed = profile.invocations_completed
            .checked_add(1)
            .ok_or(JitProfileError::CounterOverflow)?;

        let in_flight = profile.invocations_in_flight - 1;

        let next_succeeded = if outcome == JitInvocationOutcome::Succeeded {
            Some(
                profile.succeeded
                    .checked_add(1)
                    .ok_or(JitProfileError::CounterOverflow)?
            )
        } else {
            None
        };

        let next_failed = if outcome == JitInvocationOutcome::Failed {
            Some(
                profile.failed
                    .checked_add(1)
                    .ok_or(JitProfileError::CounterOverflow)?
            )
        } else {
            None
        };

        let next_cancelled = if outcome == JitInvocationOutcome::Cancelled {
            Some(
                profile.cancelled
                    .checked_add(1)
                    .ok_or(JitProfileError::CounterOverflow)?
            )
        } else {
            None
        };

        let next_abandoned = if outcome == JitInvocationOutcome::Abandoned {
            Some(
                profile.abandoned
                    .checked_add(1)
                    .ok_or(JitProfileError::CounterOverflow)?
            )
        } else {
            None
        };

        let next_timing = match elapsed_ns {
            Some(elapsed) => {
                let count = profile.timing_samples
                    .checked_add(1)
                    .ok_or(JitProfileError::CounterOverflow)?;

                let total = profile.total_elapsed_ns
                    .checked_add(elapsed)
                    .ok_or(JitProfileError::CounterOverflow)?;

                Some((count, total))
            }

            None => None,
        };

        // Commit updates only after all arithmetic has been validated.

        profile.invocations_completed = completed;

        profile.invocations_in_flight = in_flight;

        if let Some(value) = next_succeeded {
            profile.succeeded = value;
        }

        if let Some(value) = next_failed {
            profile.failed = value;
        }

        if let Some(value) = next_cancelled {
            profile.cancelled = value;
        }

        if let Some(value) = next_abandoned {
            profile.abandoned = value;
        }

        if let Some((count, total)) = next_timing {
            let elapsed = elapsed_ns
                .ok_or(JitProfileError::CounterOverflow)?;

            profile.timing_samples = count;

            profile.total_elapsed_ns = total;

            profile.minimum_elapsed_ns = Some(
                profile.minimum_elapsed_ns
                    .map_or(elapsed, |current| current.min(elapsed))
            );

            profile.maximum_elapsed_ns = Some(
                profile.maximum_elapsed_ns
                    .map_or(elapsed, |current| current.max(elapsed))
            );
        }

        Ok(JitProfileObservation {
            key: token.key,

            domain_identity: token.domain_identity,

            outcome,

            elapsed_ns,

            recorded: true,
        })
    }

    /// Returns a deterministic snapshot of all tracked profiles.
    ///
    /// Raw invocation histories are never retained.
    pub fn snapshot(
        &self,
    ) -> JitProfileResult<JitProfileSnapshot> {
        let state = self.state
            .lock()
            .map_err(|_| JitProfileError::SynchronizationPoisoned)?;

        let profiles = state.profiles
            .iter()
            .map(|(key, profile)| {
                JitProfileEntrySnapshot {
                    key: key.clone(),

                    domain_identity: profile.domain_identity.clone(),

                    invocations_started: profile.invocations_started,

                    invocations_completed: profile.invocations_completed,

                    invocations_in_flight: profile.invocations_in_flight,

                    succeeded: profile.succeeded,

                    failed: profile.failed,

                    cancelled: profile.cancelled,

                    abandoned: profile.abandoned,

                    sampled_invocations: profile.sampled_invocations,

                    timing_samples: profile.timing_samples,

                    total_elapsed_ns: profile.total_elapsed_ns,

                    minimum_elapsed_ns: profile.minimum_elapsed_ns,

                    maximum_elapsed_ns: profile.maximum_elapsed_ns,
                }
            })
            .collect();

        Ok(JitProfileSnapshot {
            api_version: JIT_PROFILING_API_VERSION,

            profiles,

            dropped_invocations: state.dropped_invocations,
        })
    }

    /// Removes a tracked profile.
    ///
    /// Active invocations prevent removal.
    ///
    /// Removal is useful for explicit retention and lifecycle policies.
    pub fn remove(
        &self,
        key: &JitCacheKey,
    ) -> JitProfileResult<JitProfileEntrySnapshot> {
        let mut state = self.state
            .lock()
            .map_err(|_| JitProfileError::SynchronizationPoisoned)?;

        let profile = state.profiles
            .get(key)
            .ok_or(JitProfileError::ProfileNotFound)?;

        if profile.invocations_in_flight != 0 {
            return Err(JitProfileError::ActiveInvocations);
        }

        let profile = state.profiles
            .remove(key)
            .ok_or(JitProfileError::ProfileNotFound)?;

        Ok(JitProfileEntrySnapshot {
            key: key.clone(),

            domain_identity: profile.domain_identity,

            invocations_started: profile.invocations_started,

            invocations_completed: profile.invocations_completed,

            invocations_in_flight: profile.invocations_in_flight,

            succeeded: profile.succeeded,

            failed: profile.failed,

            cancelled: profile.cancelled,

            abandoned: profile.abandoned,

            sampled_invocations: profile.sampled_invocations,

            timing_samples: profile.timing_samples,

            total_elapsed_ns: profile.total_elapsed_ns,

            minimum_elapsed_ns: profile.minimum_elapsed_ns,

            maximum_elapsed_ns: profile.maximum_elapsed_ns,
        })
    }

    /// Returns the number of currently tracked compilation units.
    pub fn tracked_units(&self) -> JitProfileResult<usize> {
        let state = self.state
            .lock()
            .map_err(|_| JitProfileError::SynchronizationPoisoned)?;

        Ok(state.profiles.len())
    }
}

// =============================================================================
// INVOCATION TOKEN
// =============================================================================

/// Ownership-bearing token representing one active invocation.
///
/// Tokens must be completed through the collector that created them.
///
/// Dropping an unfinished token leaves the invocation visible as
/// in-flight in the profile snapshot. It does not fabricate a result.
#[derive(Debug)]
pub struct JitInvocationToken {
    collector: Arc<Mutex<ProfileState>>,

    key: JitCacheKey,

    domain_identity: String,

    started_at: Option<Instant>,

    tracked: bool,

    sampled: bool,
}

impl JitInvocationToken {
    /// Returns the associated compilation-unit identity.
    #[must_use]
    pub fn key(&self) -> &JitCacheKey {
        &self.key
    }

    /// Returns the associated computational domain.
    #[must_use]
    pub fn domain_identity(&self) -> &str {
        &self.domain_identity
    }

    /// Returns whether this invocation was selected for timing.
    #[must_use]
    pub const fn is_sampled(&self) -> bool {
        self.sampled
    }

    /// Returns whether the invocation is tracked.
    #[must_use]
    pub const fn is_tracked(&self) -> bool {
        self.tracked
    }
}

// =============================================================================
// INDIVIDUAL OBSERVATION
// =============================================================================

/// Resulting observation from one completed invocation.
///
/// `recorded` is false when the configured profile budget caused
/// the invocation to be excluded from retained profiles.
///
/// This is an instrumentation result, not an execution result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JitProfileObservation {
    key: JitCacheKey,

    domain_identity: String,

    outcome: JitInvocationOutcome,

    elapsed_ns: Option<u128>,

    recorded: bool,
}

impl JitProfileObservation {
    /// Returns the compilation-unit identity.
    #[must_use]
    pub fn key(&self) -> &JitCacheKey {
        &self.key
    }

    /// Returns the computational domain.
    #[must_use]
    pub fn domain_identity(&self) -> &str {
        &self.domain_identity
    }

    /// Returns the actual invocation outcome.
    #[must_use]
    pub const fn outcome(&self) -> JitInvocationOutcome {
        self.outcome
    }

    /// Returns the measured elapsed nanoseconds, if sampled.
    #[must_use]
    pub const fn elapsed_ns(&self) -> Option<u128> {
        self.elapsed_ns
    }

    /// Returns whether the observation was retained.
    #[must_use]
    pub const fn recorded(&self) -> bool {
        self.recorded
    }
}

// =============================================================================
// PROFILE ENTRY SNAPSHOT
// =============================================================================

/// Immutable aggregate snapshot for one JIT compilation unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JitProfileEntrySnapshot {
    key: JitCacheKey,

    domain_identity: String,

    invocations_started: u128,

    invocations_completed: u128,

    invocations_in_flight: u128,

    succeeded: u128,

    failed: u128,

    cancelled: u128,

    abandoned: u128,

    sampled_invocations: u128,

    timing_samples: u128,

    total_elapsed_ns: u128,

    minimum_elapsed_ns: Option<u128>,

    maximum_elapsed_ns: Option<u128>,
}

impl JitProfileEntrySnapshot {
    /// Returns the compilation-unit identity.
    #[must_use]
    pub fn key(&self) -> &JitCacheKey {
        &self.key
    }

    /// Returns the computational-domain identity.
    #[must_use]
    pub fn domain_identity(&self) -> &str {
        &self.domain_identity
    }

    /// Returns the total number of started invocations.
    #[must_use]
    pub const fn invocations_started(&self) -> u128 {
        self.invocations_started
    }

    /// Returns the total number of completed invocations.
    #[must_use]
    pub const fn invocations_completed(&self) -> u128 {
        self.invocations_completed
    }

    /// Returns the number of currently active invocations.
    #[must_use]
    pub const fn invocations_in_flight(&self) -> u128 {
        self.invocations_in_flight
    }

    /// Returns successful invocations.
    #[must_use]
    pub const fn succeeded(&self) -> u128 {
        self.succeeded
    }

    /// Returns failed invocations.
    #[must_use]
    pub const fn failed(&self) -> u128 {
        self.failed
    }

    /// Returns cancelled invocations.
    #[must_use]
    pub const fn cancelled(&self) -> u128 {
        self.cancelled
    }

    /// Returns abandoned invocations.
    #[must_use]
    pub const fn abandoned(&self) -> u128 {
        self.abandoned
    }

    /// Returns the number of sampled invocations.
    #[must_use]
    pub const fn sampled_invocations(&self) -> u128 {
        self.sampled_invocations
    }

    /// Returns the number of completed timing samples.
    #[must_use]
    pub const fn timing_samples(&self) -> u128 {
        self.timing_samples
    }

    /// Returns the exact accumulated elapsed nanoseconds.
    #[must_use]
    pub const fn total_elapsed_ns(&self) -> u128 {
        self.total_elapsed_ns
    }

    /// Returns the minimum sampled duration in nanoseconds.
    #[must_use]
    pub const fn minimum_elapsed_ns(&self) -> Option<u128> {
        self.minimum_elapsed_ns
    }

    /// Returns the maximum sampled duration in nanoseconds.
    #[must_use]
    pub const fn maximum_elapsed_ns(&self) -> Option<u128> {
        self.maximum_elapsed_ns
    }

    /// Returns the arithmetic mean elapsed nanoseconds.
    ///
    /// Floating-point conversion is used only for presentation.
    /// Exact accumulation remains available through total_elapsed_ns.
    #[must_use]
    pub fn mean_elapsed_ns(&self) -> Option<f64> {
        if self.timing_samples == 0 {
            return None;
        }

        let mean = self.total_elapsed_ns as f64
            / self.timing_samples as f64;

        mean.is_finite().then_some(mean)
    }
}

// =============================================================================
// COMPLETE SNAPSHOT
// =============================================================================

/// Immutable deterministic snapshot of JIT profiling state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JitProfileSnapshot {
    api_version: u16,

    profiles: Vec<JitProfileEntrySnapshot>,

    dropped_invocations: u128,
}

impl JitProfileSnapshot {
    /// Returns the profiling API version.
    #[must_use]
    pub const fn api_version(&self) -> u16 {
        self.api_version
    }

    /// Returns tracked compilation-unit profiles.
    ///
    /// Entries are ordered by canonical JIT cache identity.
    #[must_use]
    pub fn profiles(&self) -> &[JitProfileEntrySnapshot] {
        &self.profiles
    }

    /// Returns the number of tracked compilation units.
    #[must_use]
    pub fn tracked_units(&self) -> usize {
        self.profiles.len()
    }

    /// Returns whether the snapshot is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }

    /// Returns the number of invocations excluded by the
    /// configured profile-cardinality budget.
    #[must_use]
    pub const fn dropped_invocations(&self) -> u128 {
        self.dropped_invocations
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::jit::engine::JitCacheKey;

    fn key(value: &str) -> JitCacheKey {
        JitCacheKey::new(value)
            .expect("valid JIT cache key")
    }

    fn collector() -> JitProfileCollector {
        JitProfileCollector::default()
    }

    #[test]
    fn validates_open_domain_identifiers() {
        assert!(validate_domain_identity("zamani.classical"));

        assert!(validate_domain_identity("vendor.photonics"));

        assert!(
            validate_domain_identity(
                "future.neuromorphic.compute"
            )
        );

        assert!(!validate_domain_identity("classical"));

        assert!(!validate_domain_identity("vendor..domain"));

        assert!(!validate_domain_identity("Vendor.domain"));
    }

    #[test]
    fn records_successful_invocations() {
        let collector = collector();

        let token = collector
            .begin(&key("sha256:unit-a"), "zamani.classical")
            .expect("invocation begins");

        let observation = collector
            .finish(token, JitInvocationOutcome::Succeeded)
            .expect("invocation finishes");

        assert!(observation.recorded());

        assert_eq!(
            observation.outcome(),
            JitInvocationOutcome::Succeeded
        );

        let snapshot = collector.snapshot()
            .expect("snapshot succeeds");

        let profile = &snapshot.profiles()[0];

        assert_eq!(profile.invocations_started(), 1);

        assert_eq!(profile.invocations_completed(), 1);

        assert_eq!(profile.succeeded(), 1);

        assert_eq!(profile.invocations_in_flight(), 0);
    }

    #[test]
    fn records_failed_invocations_without_fabricating_success() {
        let collector = collector();

        let token = collector
            .begin(&key("sha256:unit-b"), "zamani.quantum")
            .expect("invocation begins");

        collector
            .finish(token, JitInvocationOutcome::Failed)
            .expect("failure is recorded");

        let snapshot = collector.snapshot()
            .expect("snapshot succeeds");

        let profile = &snapshot.profiles()[0];

        assert_eq!(profile.failed(), 1);

        assert_eq!(profile.succeeded(), 0);
    }

    #[test]
    fn records_cancelled_invocations() {
        let collector = collector();

        let token = collector
            .begin(&key("sha256:unit-c"), "zamani.classical")
            .expect("invocation begins");

        collector
            .finish(token, JitInvocationOutcome::Cancelled)
            .expect("cancellation is recorded");

        let snapshot = collector.snapshot()
            .expect("snapshot succeeds");

        assert_eq!(
            snapshot.profiles()[0].cancelled(),
            1
        );
    }

    #[test]
    fn sampling_is_configurable() {
        let config = JitProfileConfig::new()
            .with_sampling(
                JitSamplingPolicy::EveryNth(
                    NonZeroU64::new(3).expect("nonzero")
                )
            );

        let collector = JitProfileCollector::new(config);

        for _ in 0..6 {
            let token = collector
                .begin(
                    &key("sha256:sampling"),
                    "zamani.classical",
                )
                .expect("begin");

            collector
                .finish(token, JitInvocationOutcome::Succeeded)
                .expect("finish");
        }

        let snapshot = collector.snapshot()
            .expect("snapshot");

        let profile = &snapshot.profiles()[0];

        assert_eq!(profile.invocations_started(), 6);

        assert_eq!(profile.sampled_invocations(), 2);

        assert_eq!(profile.timing_samples(), 2);
    }

    #[test]
    fn rejects_domain_identity_reassignment() {
        let collector = collector();

        let first = key("sha256:domain");

        let token = collector
            .begin(&first, "zamani.classical")
            .expect("first domain");

        collector
            .finish(token, JitInvocationOutcome::Succeeded)
            .expect("finish");

        assert!(
            collector
                .begin(&first, "zamani.quantum")
                .is_err()
        );
    }

    #[test]
    fn rejects_new_profiles_when_budget_is_exhausted() {
        let config = JitProfileConfig::new()
            .with_max_tracked_units(
                NonZeroUsize::new(1)
            );

        let collector = JitProfileCollector::new(config);

        let token = collector
            .begin(
                &key("sha256:first"),
                "zamani.classical",
            )
            .expect("first profile");

        collector
            .finish(token, JitInvocationOutcome::Succeeded)
            .expect("finish");

        let result = collector.begin(
            &key("sha256:second"),
            "zamani.classical",
        );

        assert!(
            matches!(
                result,
                Err(JitProfileError::ProfileBudgetExceeded { .. })
            )
        );
    }

    #[test]
    fn drop_new_policy_reports_unrecorded_observations() {
        let config = JitProfileConfig::new()
            .with_max_tracked_units(
                NonZeroUsize::new(1)
            )
            .with_overflow_policy(
                ProfileOverflowPolicy::DropNew
            );

        let collector = JitProfileCollector::new(config);

        let first = collector
            .begin(
                &key("sha256:first"),
                "zamani.classical",
            )
            .expect("first profile");

        collector
            .finish(first, JitInvocationOutcome::Succeeded)
            .expect("first finish");

        let second = collector
            .begin(
                &key("sha256:second"),
                "zamani.classical",
            )
            .expect("drop-new returns an untracked token");

        let observation = collector
            .finish(second, JitInvocationOutcome::Succeeded)
            .expect("untracked completion");

        assert!(!observation.recorded());

        assert_eq!(
            collector.snapshot()
                .expect("snapshot")
                .dropped_invocations(),
            1
        );
    }

    #[test]
    fn prevents_removing_active_profiles() {
        let collector = collector();

        let cache_key = key("sha256:active");

        let _token = collector
            .begin(&cache_key, "zamani.classical")
            .expect("begin");

        assert!(
            matches!(
                collector.remove(&cache_key),
                Err(JitProfileError::ActiveInvocations)
            )
        );
    }

    #[test]
    fn removes_completed_profiles() {
        let collector = collector();

        let cache_key = key("sha256:remove");

        let token = collector
            .begin(&cache_key, "zamani.classical")
            .expect("begin");

        collector
            .finish(token, JitInvocationOutcome::Succeeded)
            .expect("finish");

        let removed = collector
            .remove(&cache_key)
            .expect("remove");

        assert_eq!(removed.succeeded(), 1);

        assert_eq!(collector.tracked_units().expect("count"), 0);
    }

    #[test]
    fn snapshots_are_deterministically_ordered() {
        let collector = collector();

        for identity in ["sha256:z", "sha256:a", "sha256:m"] {
            let token = collector
                .begin(
                    &key(identity),
                    "zamani.classical",
                )
                .expect("begin");

            collector
                .finish(token, JitInvocationOutcome::Succeeded)
                .expect("finish");
        }

        let snapshot = collector.snapshot()
            .expect("snapshot");

        let identities: Vec<&str> = snapshot
            .profiles()
            .iter()
            .map(|profile| profile.key().as_str())
            .collect();

        assert_eq!(
            identities,
            vec![
                "sha256:a",
                "sha256:m",
                "sha256:z"
            ]
        );
    }

    #[test]
    fn supports_multiple_collectors_without_shared_state() {
        let first = collector();

        let second = collector();

        let token = first
            .begin(
                &key("sha256:isolation"),
                "zamani.classical",
            )
            .expect("begin");

        assert!(
            matches!(
                second.finish(
                    token,
                    JitInvocationOutcome::Succeeded
                ),
                Err(JitProfileError::ForeignInvocationToken)
            )
        );

        assert_eq!(
            first.tracked_units().expect("count"),
            1
        );

        assert_eq!(
            second.tracked_units().expect("count"),
            0
        );
    }

    #[test]
    fn unfinished_invocations_remain_visible() {
        let collector = collector();

        let _token = collector
            .begin(
                &key("sha256:unfinished"),
                "zamani.classical",
            )
            .expect("begin");

        let snapshot = collector.snapshot()
            .expect("snapshot");

        assert_eq!(
            snapshot.profiles()[0].invocations_in_flight(),
            1
        );

        assert_eq!(
            snapshot.profiles()[0].invocations_completed(),
            0
        );
    }

    #[test]
    fn api_version_is_available() {
        assert_eq!(JIT_PROFILING_API_VERSION, 1);
    }
}
