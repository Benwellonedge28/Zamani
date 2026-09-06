//! Zamani Quantum Scheduling — Adaptive Scheduling Algorithm
//!
//! Path:
//!     src/quantum/scheduling/algorithms/adaptive.rs
//!
//! # Purpose
//!
//! This module provides the adaptive scheduling strategy for Zamani.
//!
//! Adaptive scheduling selects an appropriate already-implemented scheduling
//! planner according to the immutable `SchedulingContext` and explicit
//! scheduling configuration.
//!
//! The adaptive layer is deliberately a strategy selector. It does not:
//!
//! - redefine quantum semantics;
//! - create a competing QubitId;
//! - perform logical-to-physical routing;
//! - discover hardware;
//! - communicate with a QPU;
//! - authenticate;
//! - execute quantum jobs;
//! - acquire calibration data;
//! - implement QEC decoding;
//! - implement noise modelling;
//! - duplicate resource-calendar semantics;
//! - duplicate dependency-graph semantics;
//! - mutate the canonical quantum IR.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::frontend
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ▼
//! optimization
//!      │
//!      ▼
//! routing
//!      │
//!      ▼
//! SchedulingContext
//!      │
//!      ▼
//! AdaptiveAlgorithm
//!      │
//!      ├──────────────► List planner
//!      ├──────────────► Critical-path planner
//!      ├──────────────► Resource-constrained planner
//!      └──────────────► Event-driven planner
//!      │
//!      ▼
//! SchedulingResult
//!      │
//!      ▼
//! verification
//!      │
//!      ▼
//! transformations / optimization
//!      │
//!      ▼
//! hardware / runtime
//! ```
//!
//! # Why an adaptive layer?
//!
//! No single scheduling heuristic is best for every quantum workload.
//!
//! Examples:
//!
//! - a simple dependency-heavy circuit can benefit from critical-path
//!   scheduling;
//! - a resource-contended target benefits from resource-aware scheduling;
//! - a highly parallel workload benefits from list scheduling;
//! - runtime-dependent programs require an event/dynamic scheduler;
//! - distributed workloads require communication-aware scheduling.
//!
//! Adaptive scheduling therefore selects among planner capabilities rather
//! than encoding one universal heuristic.
//!
//! # Critical design rule
//!
//! Adaptive scheduling MUST NOT silently change program semantics.
//!
//! It may change:
//!
//! - operation ordering where dependencies permit;
//! - start times;
//! - resource reservations;
//! - planner choice;
//! - scheduling heuristics;
//! - optimization strategy.
//!
//! It must never change:
//!
//! - quantum operation meaning;
//! - operation operands;
//! - measurement semantics;
//! - classical conditions;
//! - logical/physical qubit identity;
//! - routing decisions;
//! - requested program behavior.
//!
//! # Write once, scale everywhere
//!
//! This implementation contains no:
//!
//! - maximum qubit count;
//! - maximum operation count;
//! - maximum resource count;
//! - fixed topology;
//! - fixed channel count;
//! - fixed QEC distance;
//! - fixed gate set;
//! - fixed gate arity;
//! - fixed timing unit;
//! - fixed machine size.
//!
//! The adaptive decision is based on information supplied by the
//! `SchedulingContext` and explicit configuration.
//!
//! There is intentionally no concept of "infinite memory" or "infinite
//! execution". "Infinity" means that this algorithm introduces no artificial
//! finite machine-size ceiling. Actual execution remains bounded by available
//! host resources, target resources, explicit deployment limits, and the
//! physical system.
//!
//! # Determinism
//!
//! Deterministic scheduling is preserved by:
//!
//! - using explicit configuration;
//! - avoiding hash-map iteration as a semantic decision;
//! - using stable planner identifiers;
//! - using deterministic scoring;
//! - never introducing hidden randomness.
//!
//! If randomized selection is added in the future, its random source must be
//! supplied explicitly through the scheduling configuration/context.
//!
//! # Complexity
//!
//! Adaptive selection should remain substantially cheaper than scheduling.
//!
//! Planner selection must not repeatedly traverse the complete operation DAG
//! merely to make a heuristic decision.
//!
//! Where context-level metrics are available, they should be consumed directly.
//!
//! The adaptive layer must never construct:
//!
//! ```text
//! qubits × time
//! resources × maximum_time
//! operations × maximum_depth
//! ```
//!
//! It operates on metadata and delegates actual scheduling to specialized
//! planners.
//!
//! # Planner independence
//!
//! The adaptive algorithm depends on planner contracts rather than concrete
//! resource implementations.
//!
//! This means new schedulers can be added without changing the adaptive
//! algorithm's semantic model.
//!
//! # Integration contract
//!
//! Upstream:
//!
//! - `quantum::ir`
//! - `quantum::routing`
//! - `scheduling::context`
//! - `scheduling::config`
//! - `scheduling::policies`
//! - `scheduling::resources`
//! - `scheduling::timing`
//!
//! Planner implementations:
//!
//! - `scheduling::planners::list`
//! - `scheduling::planners::critical_path`
//! - `scheduling::planners::resource_constrained`
//! - `scheduling::planners::event`
//!
//! Downstream:
//!
//! - `scheduling::result`
//! - `scheduling::verification`
//! - `scheduling::optimization`
//! - `scheduling::transformations`
//! - `scheduling::diagnostics`
//! - `quantum::hardware`
//! - runtime.
//!
//! # Frozen-contract rule
//!
//! This file intentionally exposes a stable adaptive-selection contract.
//!
//! Adding a new planner should normally require only:
//!
//! 1. implementing the existing planner contract;
//! 2. registering it with the planner registry/composition layer;
//! 3. exposing its capabilities.
//!
//! The adaptive selector must not acquire vendor-specific knowledge merely
//! because a new hardware backend is introduced.
//!
//! # Rust
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! ============================================================================
//! Safety boundary
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;

use crate::quantum::scheduling::context::SchedulingContext;
use crate::quantum::scheduling::planners::planner::{
    PlannerId,
    SchedulingPlanner,
};
use crate::quantum::scheduling::result::SchedulingResult;

// ============================================================================
// Public algorithm identity
// ============================================================================

/// Stable identifier for the adaptive scheduling algorithm.
pub const ADAPTIVE_ALGORITHM_ID: &str = "scheduling.adaptive";

/// Semantic version of this adaptive algorithm contract.
pub const ADAPTIVE_ALGORITHM_VERSION: u32 = 1;

// ============================================================================
// Selection strategy
// ============================================================================

/// Controls how the adaptive scheduler chooses a planner.
///
/// The adaptive implementation intentionally does not contain hardware-vendor
/// branches. Target-specific information must be represented by the
/// `SchedulingContext`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AdaptiveStrategy {
    /// Automatically choose a planner using context characteristics.
    Automatic,

    /// Prefer dependency/critical-path scheduling.
    CriticalPath,

    /// Prefer general resource-aware list scheduling.
    List,

    /// Prefer resource-constrained scheduling.
    ResourceConstrained,

    /// Prefer event-driven scheduling.
    EventDriven,
}

impl Default for AdaptiveStrategy {
    fn default() -> Self {
        Self::Automatic
    }
}

// ============================================================================
// Selection reason
// ============================================================================

/// Machine-readable reason explaining why a planner was selected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionReason {
    /// Explicit caller selection.
    Explicit,

    /// Static workload with no known resource pressure.
    StaticDependencyWorkload,

    /// Resource pressure makes resource-aware scheduling preferable.
    ResourcePressure,

    /// Runtime/dynamic execution requirements are present.
    DynamicExecution,

    /// The workload exposes significant dependency structure.
    CriticalPathStructure,

    /// The adaptive layer could not infer a stronger specialization.
    GeneralPurposeFallback,
}

impl fmt::Display for SelectionReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Explicit => formatter.write_str("explicit strategy selection"),

            Self::StaticDependencyWorkload => {
                formatter.write_str("static dependency workload")
            }

            Self::ResourcePressure => {
                formatter.write_str("resource pressure")
            }

            Self::DynamicExecution => {
                formatter.write_str("dynamic execution requirements")
            }

            Self::CriticalPathStructure => {
                formatter.write_str("critical-path dependency structure")
            }

            Self::GeneralPurposeFallback => {
                formatter.write_str("general-purpose fallback")
            }
        }
    }
}

// ============================================================================
// Selection
// ============================================================================

/// Result of adaptive planner selection.
///
/// This is intentionally separate from `SchedulingResult`: selection is a
/// planning decision, while `SchedulingResult` is the actual schedule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaptiveSelection {
    /// Stable planner identifier.
    planner: PlannerId,

    /// Why this planner was selected.
    reason: SelectionReason,
}

impl AdaptiveSelection {
    /// Creates a selection.
    pub fn new(
        planner: PlannerId,
        reason: SelectionReason,
    ) -> Self {
        Self {
            planner,
            reason,
        }
    }

    /// Returns the selected planner identifier.
    #[must_use]
    pub fn planner(&self) -> &PlannerId {
        &self.planner
    }

    /// Returns the selection reason.
    #[must_use]
    pub const fn reason(&self) -> &SelectionReason {
        &self.reason
    }
}

// ============================================================================
// Adaptive algorithm error
// ============================================================================

/// Errors produced by adaptive planner selection.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AdaptiveSchedulingError {
    /// No planner satisfying the required capabilities was available.
    NoCompatiblePlanner {
        /// Strategy that was requested.
        strategy: AdaptiveStrategy,
    },

    /// The selected planner rejected the supplied context.
    PlannerUnsupported {
        /// Planner that was selected.
        planner: PlannerId,
    },

    /// The selected planner failed during scheduling.
    PlannerFailed {
        /// Planner that failed.
        planner: PlannerId,

        /// Stable diagnostic.
        message: String,
    },
}

impl fmt::Display for AdaptiveSchedulingError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::NoCompatiblePlanner { strategy } => {
                write!(
                    formatter,
                    "no compatible scheduling planner is available for adaptive strategy `{strategy:?}`"
                )
            }

            Self::PlannerUnsupported { planner } => {
                write!(
                    formatter,
                    "selected scheduling planner `{planner}` does not support the supplied context"
                )
            }

            Self::PlannerFailed {
                planner,
                message,
            } => {
                write!(
                    formatter,
                    "scheduling planner `{planner}` failed: {message}"
                )
            }
        }
    }
}

impl std::error::Error for AdaptiveSchedulingError {}

/// Result type for adaptive scheduling.
pub type AdaptiveSchedulingResult<T> =
    Result<T, AdaptiveSchedulingError>;

// ============================================================================
// Planner provider
// ============================================================================

/// Provider of scheduling planners.
///
/// The provider is deliberately injected instead of using global mutable
/// registries.
///
/// This keeps adaptive scheduling:
///
/// - deterministic;
/// - testable;
/// - thread-safe when the supplied planners are thread-safe;
/// - independent of global state;
/// - suitable for embedded and distributed compilation.
///
/// A registry implementation can satisfy this trait later.
pub trait AdaptivePlannerProvider {
    /// Returns the planners available for this invocation.
    ///
    /// The returned collection must remain stable for the duration of one
    /// adaptive scheduling operation.
    fn planners(
        &self,
    ) -> &[Box<dyn SchedulingPlanner<Error = crate::quantum::scheduling::errors::SchedulingError>>];
}

// ============================================================================
// Static planner collection
// ============================================================================

/// A caller-owned planner collection.
///
/// This provides a simple production-safe dependency-injection boundary
/// without global mutable state.
pub struct PlannerSet {
    planners: Vec<
        Box<
            dyn SchedulingPlanner<
                Error = crate::quantum::scheduling::errors::SchedulingError,
            >,
        >,
    >,
}

impl PlannerSet {
    /// Creates an empty planner set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            planners: Vec::new(),
        }
    }

    /// Creates a planner set from an existing collection.
    #[must_use]
    pub fn from_planners(
        planners: Vec<
            Box<
                dyn SchedulingPlanner<
                    Error = crate::quantum::scheduling::errors::SchedulingError,
                >,
            >,
        >,
    ) -> Self {
        Self { planners }
    }

    /// Adds a planner.
    ///
    /// Planner ownership remains inside this invocation-local collection.
    pub fn push(
        &mut self,
        planner: Box<
            dyn SchedulingPlanner<
                Error = crate::quantum::scheduling::errors::SchedulingError,
            >,
        >,
    ) {
        self.planners.push(planner);
    }

    /// Returns the number of planners.
    #[must_use]
    pub fn len(&self) -> usize {
        self.planners.len()
    }

    /// Returns whether no planners are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.planners.is_empty()
    }
}

impl Default for PlannerSet {
    fn default() -> Self {
        Self::new()
    }
}

impl AdaptivePlannerProvider for PlannerSet {
    fn planners(
        &self,
    ) -> &[Box<
        dyn SchedulingPlanner<
            Error = crate::quantum::scheduling::errors::SchedulingError,
        >,
    >] {
        &self.planners
    }
}

// ============================================================================
// Adaptive scheduler
// ============================================================================

/// Adaptive scheduler.
///
/// This type owns no global state and no hardware state.
///
/// All decisions are made from:
///
/// - the supplied immutable `SchedulingContext`;
/// - the explicit `AdaptiveStrategy`;
/// - the caller-provided planner set.
///
/// The scheduler itself does not manufacture resource capacities or machine
/// dimensions.
pub struct AdaptiveScheduler<P> {
    provider: P,
    strategy: AdaptiveStrategy,
}

impl<P> AdaptiveScheduler<P>
where
    P: AdaptivePlannerProvider,
{
    /// Creates an adaptive scheduler using automatic strategy selection.
    #[must_use]
    pub const fn new(provider: P) -> Self {
        Self {
            provider,
            strategy: AdaptiveStrategy::Automatic,
        }
    }

    /// Creates an adaptive scheduler with an explicit strategy.
    #[must_use]
    pub const fn with_strategy(
        provider: P,
        strategy: AdaptiveStrategy,
    ) -> Self {
        Self {
            provider,
            strategy,
        }
    }

    /// Returns the configured strategy.
    #[must_use]
    pub const fn strategy(&self) -> AdaptiveStrategy {
        self.strategy
    }

    /// Returns the injected planner provider.
    #[must_use]
    pub const fn provider(&self) -> &P {
        &self.provider
    }

    /// Selects a planner without executing it.
    ///
    /// This method is useful for diagnostics, testing, explainability, and
    /// compilation planning.
    pub fn select(
        &self,
        context: &SchedulingContext,
    ) -> AdaptiveSchedulingResult<AdaptiveSelection> {
        let planners = self.provider.planners();

        if planners.is_empty() {
            return Err(
                AdaptiveSchedulingError::NoCompatiblePlanner {
                    strategy: self.strategy,
                },
            );
        }

        match self.strategy {
            AdaptiveStrategy::Automatic => {
                self.select_automatic(context, planners)
            }

            AdaptiveStrategy::CriticalPath => self.select_named(
                context,
                planners,
                "scheduling.critical_path",
                SelectionReason::Explicit,
            ),

            AdaptiveStrategy::List => self.select_named(
                context,
                planners,
                "scheduling.list",
                SelectionReason::Explicit,
            ),

            AdaptiveStrategy::ResourceConstrained => self.select_named(
                context,
                planners,
                "scheduling.resource_constrained",
                SelectionReason::Explicit,
            ),

            AdaptiveStrategy::EventDriven => self.select_named(
                context,
                planners,
                "scheduling.event",
                SelectionReason::Explicit,
            ),
        }
    }

    /// Executes the selected planner.
    pub fn schedule(
        &self,
        context: &SchedulingContext,
    ) -> AdaptiveSchedulingResult<SchedulingResult> {
        let selection = self.select(context)?;

        let planner = self
            .provider
            .planners()
            .iter()
            .find(|candidate| {
                candidate
                    .metadata()
                    .map(|metadata| {
                        metadata.id() == selection.planner()
                    })
                    .unwrap_or_else(|| {
                        candidate.name() == selection.planner().as_str()
                    })
            })
            .ok_or_else(|| {
                AdaptiveSchedulingError::NoCompatiblePlanner {
                    strategy: self.strategy,
                }
            })?;

        if !planner.supports(context) {
            return Err(
                AdaptiveSchedulingError::PlannerUnsupported {
                    planner: selection.planner().clone(),
                },
            );
        }

        planner
            .plan(context)
            .map_err(|error| {
                AdaptiveSchedulingError::PlannerFailed {
                    planner: selection.planner().clone(),
                    message: error.to_string(),
                }
            })
    }

    fn select_named(
        &self,
        context: &SchedulingContext,
        planners: &[Box<
            dyn SchedulingPlanner<
                Error = crate::quantum::scheduling::errors::SchedulingError,
            >,
        >],
        requested: &str,
        reason: SelectionReason,
    ) -> AdaptiveSchedulingResult<AdaptiveSelection> {
        let requested_id = PlannerId::new(requested.to_owned())
            .map_err(|_| {
                AdaptiveSchedulingError::NoCompatiblePlanner {
                    strategy: self.strategy,
                }
            })?;

        let planner = planners.iter().find(|candidate| {
            candidate
                .metadata()
                .map(|metadata| metadata.id() == &requested_id)
                .unwrap_or_else(|| candidate.name() == requested)
        });

        let planner = match planner {
            Some(planner) => planner,
            None => {
                return Err(
                    AdaptiveSchedulingError::NoCompatiblePlanner {
                        strategy: self.strategy,
                    },
                );
            }
        };

        if !planner.supports(context) {
            return Err(
                AdaptiveSchedulingError::PlannerUnsupported {
                    planner: requested_id,
                },
            );
        }

        Ok(AdaptiveSelection::new(
            requested_id,
            reason,
        ))
    }

    fn select_automatic(
        &self,
        context: &SchedulingContext,
        planners: &[Box<
            dyn SchedulingPlanner<
                Error = crate::quantum::scheduling::errors::SchedulingError,
            >,
        >],
    ) -> AdaptiveSchedulingResult<AdaptiveSelection> {
        /*
         * Automatic selection deliberately uses planner capability metadata
         * rather than inspecting private implementation details.
         *
         * The first compatible planner with the strongest recognized semantic
         * capability is selected. Stable planner metadata ordering provides
         * deterministic fallback behaviour.
         *
         * Dynamic/resource-specific specialization should eventually be
         * represented by planner metadata/capability information supplied by
         * the scheduling planner registry.
         */

        let mut best: Option<(
            u8,
            PlannerId,
            SelectionReason,
        )> = None;

        for planner in planners {
            if !planner.supports(context) {
                continue;
            }

            let metadata = match planner.metadata() {
                Some(metadata) => metadata,
                None => continue,
            };

            let id = match PlannerId::new(
                metadata.id().as_str().to_owned(),
            ) {
                Ok(id) => id,
                Err(_) => continue,
            };

            let (rank, reason) = Self::score_planner(
                metadata,
                context,
            );

            let candidate = (rank, id, reason);

            let replace = match &best {
                None => true,

                Some(current) => {
                    candidate.0 > current.0
                        || (
                            candidate.0 == current.0
                                && candidate.1 < current.1
                        )
                }
            };

            if replace {
                best = Some(candidate);
            }
        }

        /*
         * Metadata-free planners remain usable as a general fallback.
         *
         * This fallback is intentionally deterministic and does not depend on
         * hash-map ordering.
         */
        if best.is_none() {
            for planner in planners {
                if !planner.supports(context) {
                    continue;
                }

                let name = planner.name();

                let id = PlannerId::new(name.to_owned())
                    .map_err(|_| {
                        AdaptiveSchedulingError::NoCompatiblePlanner {
                            strategy: self.strategy,
                        }
                    })?;

                return Ok(AdaptiveSelection::new(
                    id,
                    SelectionReason::GeneralPurposeFallback,
                ));
            }
        }

        let (_, planner, reason) = best.ok_or_else(|| {
            AdaptiveSchedulingError::NoCompatiblePlanner {
                strategy: self.strategy,
            }
        })?;

        Ok(AdaptiveSelection::new(
            planner,
            reason,
        ))
    }

    fn score_planner(
        metadata: &crate::quantum::scheduling::planners::planner::PlannerMetadata,
        _context: &SchedulingContext,
    ) -> (u8, SelectionReason) {
        /*
         * This scoring function intentionally remains conservative.
         *
         * It recognizes stable planner identifiers instead of embedding
         * assumptions about hardware, qubit count, topology, or vendor.
         *
         * Future planner capability metadata can replace these identifier
         * checks without changing the adaptive scheduling contract.
         */

        match metadata.id().as_str() {
            "scheduling.event" => {
                /*
                 * Event scheduling is the most general representation for
                 * runtime/event-driven workloads. It receives the highest
                 * generic rank, but only when its planner declares support.
                 */
                (40, SelectionReason::DynamicExecution)
            }

            "scheduling.resource_constrained" => {
                (35, SelectionReason::ResourcePressure)
            }

            "scheduling.list" => {
                (30, SelectionReason::StaticDependencyWorkload)
            }

            "scheduling.critical_path" => {
                (25, SelectionReason::CriticalPathStructure)
            }

            _ => {
                (10, SelectionReason::GeneralPurposeFallback)
            }
        }
    }
}

// ============================================================================
// Convenience constructor
// ============================================================================

/// Creates an automatic adaptive scheduler.
#[must_use]
pub const fn adaptive_scheduler<P>(
    provider: P,
) -> AdaptiveScheduler<P>
where
    P: AdaptivePlannerProvider,
{
    AdaptiveScheduler::new(provider)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strategy_defaults_to_automatic() {
        assert_eq!(
            AdaptiveStrategy::default(),
            AdaptiveStrategy::Automatic
        );
    }

    #[test]
    fn planner_set_starts_empty() {
        let planners = PlannerSet::new();

        assert!(planners.is_empty());
        assert_eq!(planners.len(), 0);
    }

    #[test]
    fn selection_reason_is_displayable() {
        assert_eq!(
            SelectionReason::ResourcePressure.to_string(),
            "resource pressure"
        );
    }
}