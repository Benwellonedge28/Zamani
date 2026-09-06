//! Zamani Quantum Scheduling — Resource-Constrained Project Scheduling
//!
//! Path:
//!     src/quantum/scheduling/algorithms/rcpsp.rs
//!
//! # Purpose
//!
//! This module provides the production RCPSP algorithm entry point for Zamani.
//!
//! RCPSP answers:
//!
//! > Given a dependency-constrained quantum workload, arbitrary resource
//! > requirements, temporal constraints, and target-supplied resource
//! > availability, when can each operation execute legally?
//!
//! This module owns the RCPSP algorithm.
//!
//! It does NOT own:
//!
//! - Zamani parsing;
//! - canonical quantum semantics;
//! - logical-to-physical routing;
//! - hardware discovery;
//! - QPU communication;
//! - calibration acquisition;
//! - hardware execution;
//! - noise modelling;
//! - QEC decoding;
//! - dependency-graph ownership;
//! - resource-calendar ownership;
//! - final schedule verification;
//! - serialization;
//! - benchmark execution.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! quantum::frontend
//!      |
//!      v
//! quantum::ir
//!      |
//!      v
//! optimization
//!      |
//!      v
//! routing
//!      |
//!      v
//! scheduling::adapters
//!      |
//!      v
//! SchedulingContext / scheduling model
//!      |
//!      +-----------------------------+
//!      |                             |
//!      v                             v
//! dependency graph             resource/timing model
//!      |                             |
//!      +-------------+---------------+
//!                    |
//!                    v
//!             RCPSP algorithm
//!                    |
//!                    v
//!              candidate schedule
//!                    |
//!                    v
//!              verification
//!                    |
//!                    v
//!             hardware/runtime
//! ```
//!
//! # Relationship to `planners::resource_constrained`
//!
//! The established resource-constrained planner owns the canonical
//! algorithm-facing types:
//!
//! ```text
//! ResourceConstrainedOperation
//! ResourceRequirement
//! ResourceModel
//! Reservation
//! ResourceConstrainedSchedule
//! ResourceConstrainedProblem
//! ```
//!
//! This module deliberately reuses those types rather than defining another
//! operation, resource, reservation, or qubit model.
//!
//! The algorithm implementation here is intentionally separate from the
//! planner facade so that algorithm selection and planner registration can
//! evolve independently.
//!
//! # RCPSP semantics
//!
//! For every operation `o`:
//!
//! ```text
//! start(o) >= release(o)
//! finish(o) = start(o) + duration(o)
//! ```
//!
//! For every dependency:
//!
//! ```text
//! A -> B
//!
//! finish(A) <= start(B)
//! ```
//!
//! For every resource:
//!
//! ```text
//! simultaneous_usage(resource) <= capacity(resource)
//! ```
//!
//! For every temporal constraint:
//!
//! ```text
//! earliest_start <= start
//! start <= latest_start
//! finish <= deadline
//! ```
//!
//! All arithmetic is checked.
//!
//! # Algorithmic guarantee
//!
//! RCPSP is an NP-hard optimization family in the general case. This module
//! therefore does not claim globally optimal schedules for arbitrary inputs.
//!
//! The production implementation provides a deterministic resource-aware
//! constructive heuristic with:
//!
//! - precedence preservation;
//! - resource-capacity preservation;
//! - temporal-window preservation;
//! - deterministic candidate ordering;
//! - downstream criticality;
//! - deadline slack awareness;
//! - arbitrary operation arity;
//! - arbitrary resource count;
//! - arbitrary resource capacity;
//! - checked time arithmetic;
//! - no fixed machine dimensions.
//!
//! If an exact solver is later introduced, it can implement the same algorithm
//! boundary without changing this file's public data model.
//!
//! # Scalability
//!
//! There are intentionally no constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_RESOURCES
//! MAX_CHANNELS
//! MAX_DEPTH
//! ```
//!
//! The algorithm does not allocate:
//!
//! ```text
//! qubits × time
//! resources × maximum_time
//! ```
//!
//! It stores only workload, dependency, candidate, and committed reservation
//! state.
//!
//! Dependency preprocessing is O(V + E) for V operations and E dependency
//! edges, excluding ordered-map costs and resource-model operations.
//!
//! The resource model supplied by the caller determines the cost of resource
//! availability queries. This algorithm never assumes a particular calendar
//! representation.
//!
//! Practical execution remains bounded by:
//!
//! - available host memory;
//! - host address space;
//! - compilation time;
//! - explicit caller limits;
//! - target resource availability.
//!
//! "Infinity" therefore means no artificial machine-size ceiling is imposed by
//! this algorithm.
//!
//! # Canonical qubit identity
//!
//! This module does not define a qubit identity.
//!
//! Canonical logical and physical qubits remain:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! If a physical qubit is represented as a scheduling resource, the adapter
//! that constructs the `ResourceRequirement` must preserve that canonical
//! identity.
//!
//! This algorithm does not need to import `QubitId` directly because it does
//! not inspect qubit semantics; it schedules the resource identities supplied
//! by the adapter.
//!
//! # Resource neutrality
//!
//! A resource may represent:
//!
//! - logical qubits;
//! - physical qubits;
//! - control channels;
//! - readout channels;
//! - resonators;
//! - couplers;
//! - lasers;
//! - classical processors;
//! - feedback paths;
//! - communication links;
//! - QEC ancillas;
//! - network resources;
//! - future target-defined resources.
//!
//! No resource kind is hard-coded.
//!
//! # Timing neutrality
//!
//! No physical time unit is assumed.
//!
//! `TimePoint` and `Duration` are the canonical scheduling types supplied by
//! `scheduling::types` through the established resource-constrained planner
//! contract.
//!
//! # Determinism
//!
//! Candidate ordering is deterministic:
//!
//! 1. finite smaller slack first;
//! 2. higher operation priority;
//! 3. longer downstream criticality;
//! 4. smaller canonical `OperationId`.
//!
//! No hash-map iteration order, pointer address, wall clock, thread timing, or
//! implicit randomness is used as a scheduling decision.
//!
//! # Dynamic circuits
//!
//! This algorithm schedules dependencies known to the supplied model.
//!
//! It must not manufacture static dependencies from runtime information that
//! is not known at planning time.
//!
//! Runtime-only scheduling belongs to `scheduling::dynamic` and may invoke this
//! algorithm incrementally for newly available regions.
//!
//! # Distributed quantum computing
//!
//! Distributed communication is represented through the same resource and
//! dependency model:
//!
//! ```text
//! local operation
//! remote operation
//! entanglement generation
//! teleportation
//! classical communication
//! synchronization
//! ```
//!
//! Network semantics remain outside this algorithm.
//!
//! # QEC
//!
//! QEC adapters may expose syndrome extraction, ancilla, round, measurement,
//! and recovery operations as ordinary scheduled operations and resources.
//!
//! This module does not implement:
//!
//! - stabilizer codes;
//! - surface-code construction;
//! - syndrome decoding;
//! - recovery decoding.
//!
//! # Transactional scheduling
//!
//! Resource reservations are represented by the canonical
//! `planners::resource_constrained::Reservation` value.
//!
//! The resource model itself remains authoritative for availability.
//!
//! The algorithm never mutates hardware state.
//!
//! # Important implementation property
//!
//! A resource model is queried using the reservations already committed by this
//! scheduling invocation. Therefore the algorithm can construct a complete
//! candidate schedule without requiring a mutable global resource manager.
//!
//! # Rust contract
//!
//! Compatible with:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use crate::quantum::ir::core::identity::OperationId;
use crate::quantum::scheduling::planners::resource_constrained::{
    Reservation,
    ResourceConstrainedOperation,
    ResourceConstrainedProblem,
    ResourceConstrainedResult,
    ResourceConstrainedSchedule,
    ResourceConstrainedSchedulingError,
    ResourceModel,
    ScheduledOperation,
};
use crate::quantum::scheduling::types::{Duration, TimePoint};

/// Stable algorithm identifier.
///
/// This identifier is independent of target size and hardware technology.
pub const RCPSP_ALGORITHM_ID: &str =
    "scheduling.algorithms.rcpsp";

/// Stable human-readable algorithm name.
pub const RCPSP_ALGORITHM_NAME: &str =
    "resource-constrained-project-scheduling";

/// Stable semantic version of this algorithm implementation.
pub const RCPSP_ALGORITHM_VERSION: u32 = 1;

/// Algorithm capability description.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RcpspCapabilities {
    /// RCPSP is dependency aware.
    pub dependency_aware: bool,

    /// RCPSP is resource aware.
    pub resource_aware: bool,

    /// RCPSP supports temporal constraints.
    pub timing_aware: bool,

    /// Candidate selection is deterministic.
    pub deterministic: bool,

    /// No machine-size ceiling is encoded.
    pub machine_size_unbounded: bool,

    /// Resource capacities are target supplied.
    pub target_defined_capacity: bool,
}

impl Default for RcpspCapabilities {
    fn default() -> Self {
        Self {
            dependency_aware: true,
            resource_aware: true,
            timing_aware: true,
            deterministic: true,
            machine_size_unbounded: true,
            target_defined_capacity: true,
        }
    }
}

/// Explicit execution limits for one RCPSP invocation.
///
/// A value of zero means that the corresponding limit is disabled.
///
/// These are compiler-safety limits, not machine-size limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RcpspLimits {
    /// Maximum candidate-selection decisions.
    pub max_decisions: u64,

    /// Maximum resource-placement attempts.
    pub max_resource_attempts: u64,
}

impl RcpspLimits {
    /// Creates an unlimited configuration.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_decisions: 0,
            max_resource_attempts: 0,
        }
    }

    fn decision_allowed(self, count: u64) -> bool {
        self.max_decisions == 0 || count < self.max_decisions
    }

    fn resource_attempt_allowed(self, count: u64) -> bool {
        self.max_resource_attempts == 0 || count < self.max_resource_attempts
    }
}

/// Configuration for the RCPSP algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RcpspConfig {
    /// Accept an empty workload.
    pub allow_empty: bool,

    /// Global scheduling release time.
    pub release_time: TimePoint,

    /// Optional global deadline.
    pub deadline: Option<TimePoint>,

    /// Explicit invocation limits.
    pub limits: RcpspLimits,
}

impl Default for RcpspConfig {
    fn default() -> Self {
        Self {
            allow_empty: true,
            release_time: TimePoint::ZERO,
            deadline: None,
            limits: RcpspLimits::unlimited(),
        }
    }
}

impl RcpspConfig {
    /// Production configuration.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            allow_empty: true,
            release_time: TimePoint::ZERO,
            deadline: None,
            limits: RcpspLimits::unlimited(),
        }
    }

    /// Sets empty-workload handling.
    #[must_use]
    pub const fn with_allow_empty(mut self, allow: bool) -> Self {
        self.allow_empty = allow;
        self
    }

    /// Sets the global release time.
    #[must_use]
    pub const fn with_release_time(
        mut self,
        release_time: TimePoint,
    ) -> Self {
        self.release_time = release_time;
        self
    }

    /// Sets the global deadline.
    #[must_use]
    pub const fn with_deadline(
        mut self,
        deadline: Option<TimePoint>,
    ) -> Self {
        self.deadline = deadline;
        self
    }

    /// Sets explicit execution limits.
    #[must_use]
    pub const fn with_limits(
        mut self,
        limits: RcpspLimits,
    ) -> Self {
        self.limits = limits;
        self
    }
}

/// Internal candidate ordering key.
///
/// `BinaryHeap` is a max-heap, so comparisons are intentionally reversed for
/// fields where smaller values are more urgent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CandidateKey {
    /// Finite scheduling slack, when a deadline exists.
    slack: Option<Duration>,

    /// User/compiler priority.
    priority: i64,

    /// Downstream critical-path contribution.
    downstream: Duration,

    /// Stable operation identity.
    operation: OperationId,
}

impl CandidateKey {
    fn new(
        slack: Option<Duration>,
        priority: i64,
        downstream: Duration,
        operation: OperationId,
    ) -> Self {
        Self {
            slack,
            priority,
            downstream,
            operation,
        }
    }
}

impl Ord for CandidateKey {
    fn cmp(&self, other: &Self) -> Ordering {
        // Smaller finite slack is more urgent.
        let slack_order = match (self.slack, other.slack) {
            (Some(left), Some(right)) => right.cmp(&left),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
        };

        if slack_order != Ordering::Equal {
            return slack_order;
        }

        // Higher explicit priority wins.
        let priority_order = self.priority.cmp(&other.priority);

        if priority_order != Ordering::Equal {
            return priority_order;
        }

        // Longer downstream work is more urgent.
        let downstream_order = self.downstream.cmp(&other.downstream);

        if downstream_order != Ordering::Equal {
            return downstream_order;
        }

        // Smaller operation ID wins deterministically.
        other.operation.cmp(&self.operation)
    }
}

impl PartialOrd for CandidateKey {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Production RCPSP algorithm.
///
/// The type is stateless and therefore reusable across scheduling invocations.
///
/// No hardware, program, resource calendar, or global state is retained.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RcpspScheduler {
    config: RcpspConfig,
}

impl Default for RcpspScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl RcpspScheduler {
    /// Creates the production RCPSP scheduler.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: RcpspConfig::production(),
        }
    }

    /// Creates an RCPSP scheduler with explicit configuration.
    #[must_use]
    pub const fn with_config(config: RcpspConfig) -> Self {
        Self { config }
    }

    /// Returns the configuration.
    #[must_use]
    pub const fn config(&self) -> RcpspConfig {
        self.config
    }

    /// Returns the stable algorithm identifier.
    #[must_use]
    pub const fn algorithm_id(&self) -> &'static str {
        RCPSP_ALGORITHM_ID
    }

    /// Returns the stable algorithm name.
    #[must_use]
    pub const fn algorithm_name(&self) -> &'static str {
        RCPSP_ALGORITHM_NAME
    }

    /// Returns the algorithm implementation version.
    #[must_use]
    pub const fn algorithm_version(&self) -> u32 {
        RCPSP_ALGORITHM_VERSION
    }

    /// Returns algorithm capabilities.
    #[must_use]
    pub const fn capabilities(&self) -> RcpspCapabilities {
        RcpspCapabilities::default()
    }

    /// Schedules an RCPSP problem.
    ///
    /// The supplied problem uses the canonical resource-constrained scheduling
    /// model from `planners::resource_constrained`.
    ///
    /// This function does not:
    ///
    /// - mutate canonical quantum IR;
    /// - route qubits;
    /// - access hardware;
    /// - discover resources;
    /// - execute a QPU job.
    ///
    /// It only constructs an invocation-local schedule.
    pub fn schedule<R>(
        &self,
        problem: &ResourceConstrainedProblem<'_, R>,
    ) -> ResourceConstrainedResult<ResourceConstrainedSchedule>
    where
        R: ResourceModel,
    {
        if problem.operations.is_empty() {
            if self.config.allow_empty {
                return Ok(ResourceConstrainedSchedule {
                    operations: BTreeMap::new(),
                    reservations: Vec::new(),
                    makespan: problem.release_time,
                    operation_count: 0,
                });
            }

            return Err(
                ResourceConstrainedSchedulingError::InvalidModel {
                    operation: OperationId::from(0_u64),
                },
            );
        }

        let operation_map = self.validate_operations(problem.operations)?;

        let successors =
            self.build_successors(problem.operations)?;

        let topological =
            self.topological_order(
                problem.operations,
                &successors,
                &operation_map,
            )?;

        let downstream =
            self.calculate_downstream_lengths(
                &topological,
                &successors,
                &operation_map,
            )?;

        let mut remaining_predecessors =
            BTreeMap::<OperationId, usize>::new();

        for operation in problem.operations {
            remaining_predecessors.insert(
                operation.id,
                operation.predecessors.len(),
            );
        }

        let mut ready =
            BinaryHeap::<CandidateKey>::new();

        for operation in problem.operations {
            if operation.predecessors.is_empty() {
                let earliest =
                    self.initial_earliest_start(
                        operation,
                        problem.release_time,
                    )?;

                let slack =
                    self.calculate_slack(
                        operation,
                        earliest,
                        problem.deadline,
                    )?;

                ready.push(CandidateKey::new(
                    slack,
                    operation.priority,
                    downstream
                        .get(&operation.id)
                        .copied()
                        .unwrap_or(Duration::ZERO),
                    operation.id,
                ));
            }
        }

        let mut scheduled =
            BTreeMap::<OperationId, ScheduledOperation>::new();

        let mut reservations =
            Vec::<Reservation>::new();

        let mut makespan =
            problem.release_time;

        let mut decisions = 0_u64;
        let mut resource_attempts = 0_u64;

        while scheduled.len() < problem.operations.len() {
            if !self
                .config
                .limits
                .decision_allowed(decisions)
            {
                return Err(
                    ResourceConstrainedSchedulingError::LimitExceeded {
                        limit: "max_decisions",
                    },
                );
            }

            let candidate =
                ready.pop().ok_or_else(|| {
                    let operation =
                        problem
                            .operations
                            .iter()
                            .find(|operation| {
                                !scheduled.contains_key(
                                    &operation.id,
                                )
                            })
                            .map(|operation| operation.id)
                            .unwrap_or(
                                OperationId::from(0_u64),
                            );

                    ResourceConstrainedSchedulingError::CycleDetected {
                        operation,
                    }
                })?;

            if scheduled.contains_key(&candidate.operation) {
                continue;
            }

            decisions = decisions
                .checked_add(1)
                .ok_or(
                    ResourceConstrainedSchedulingError::LimitExceeded {
                        limit: "decision-counter-overflow",
                    },
                )?;

            let operation =
                operation_map
                    .get(&candidate.operation)
                    .copied()
                    .ok_or(
                        ResourceConstrainedSchedulingError::UnknownOperation {
                            operation: candidate.operation,
                        },
                    )?;

            let earliest =
                self.predecessor_finish_time(
                    operation,
                    &scheduled,
                    problem.release_time,
                )?;

            if !self
                .config
                .limits
                .resource_attempt_allowed(
                    resource_attempts,
                )
            {
                return Err(
                    ResourceConstrainedSchedulingError::LimitExceeded {
                        limit: "max_resource_attempts",
                    },
                );
            }

            resource_attempts = resource_attempts
                .checked_add(1)
                .ok_or(
                    ResourceConstrainedSchedulingError::LimitExceeded {
                        limit: "resource-attempt-counter-overflow",
                    },
                )?;

            let start =
                self.find_feasible_start(
                    operation,
                    earliest,
                    problem.resources,
                    &reservations,
                )?;

            let finish =
                operation.finish(start)?;

            self.validate_global_deadline(
                operation.id,
                finish,
                problem.deadline,
            )?;

            let new_reservations =
                self.build_reservations(
                    operation,
                    start,
                    finish,
                    problem.resources,
                    &reservations,
                )?;

            reservations.extend(new_reservations);

            scheduled.insert(
                operation.id,
                ScheduledOperation {
                    operation: operation.id,
                    start,
                    finish,
                },
            );

            if finish > makespan {
                makespan = finish;
            }

            if let Some(operation_successors) =
                successors.get(&operation.id)
            {
                for successor_id in operation_successors {
                    let count =
                        remaining_predecessors
                            .get_mut(successor_id)
                            .ok_or(
                                ResourceConstrainedSchedulingError::InvalidModel {
                                    operation: *successor_id,
                                },
                            )?;

                    if *count == 0 {
                        return Err(
                            ResourceConstrainedSchedulingError::InvalidModel {
                                operation: *successor_id,
                            },
                        );
                    }

                    *count -= 1;

                    if *count == 0 {
                        let successor =
                            operation_map
                                .get(successor_id)
                                .copied()
                                .ok_or(
                                    ResourceConstrainedSchedulingError::UnknownOperation {
                                        operation: *successor_id,
                                    },
                                )?;

                        let successor_earliest =
                            self.predecessor_finish_time(
                                successor,
                                &scheduled,
                                problem.release_time,
                            )?;

                        let slack =
                            self.calculate_slack(
                                successor,
                                successor_earliest,
                                problem.deadline,
                            )?;

                        ready.push(
                            CandidateKey::new(
                                slack,
                                successor.priority,
                                downstream
                                    .get(successor_id)
                                    .copied()
                                    .unwrap_or(
                                        Duration::ZERO,
                                    ),
                                *successor_id,
                            ),
                        );
                    }
                }
            }
        }

        if scheduled.len() != problem.operations.len() {
            let operation =
                problem
                    .operations
                    .iter()
                    .find(|operation| {
                        !scheduled.contains_key(
                            &operation.id,
                        )
                    })
                    .map(|operation| operation.id)
                    .unwrap_or(
                        OperationId::from(0_u64),
                    );

            return Err(
                ResourceConstrainedSchedulingError::InvalidModel {
                    operation,
                },
            );
        }

        Ok(ResourceConstrainedSchedule {
            operations: scheduled,
            reservations,
            makespan,
            operation_count: problem.operations.len(),
        })
    }

    /// Schedules directly from an established problem model while applying
    /// this scheduler's configured global release/deadline.
    ///
    /// This convenience method deliberately does not modify the supplied
    /// operation collection.
    pub fn schedule_with_configured_window<R>(
        &self,
        operations: &[ResourceConstrainedOperation],
        resources: &R,
    ) -> ResourceConstrainedResult<ResourceConstrainedSchedule>
    where
        R: ResourceModel,
    {
        let problem =
            ResourceConstrainedProblem::new(
                operations,
                resources,
            )
            .with_release_time(
                self.config.release_time,
            );

        let problem =
            match self.config.deadline {
                Some(deadline) => {
                    problem.with_deadline(deadline)
                }
                None => problem,
            };

        self.schedule(&problem)
    }

    fn validate_operations<'a>(
        &self,
        operations: &'a [
            ResourceConstrainedOperation,
        ],
    ) -> ResourceConstrainedResult<
        BTreeMap<
            OperationId,
            &'a ResourceConstrainedOperation,
        >,
    > {
        let mut map = BTreeMap::new();

        for operation in operations {
            if map
                .insert(operation.id, operation)
                .is_some()
            {
                return Err(
                    ResourceConstrainedSchedulingError::InvalidModel {
                        operation: operation.id,
                    },
                );
            }

            if !operation.timing.is_valid() {
                return Err(
                    ResourceConstrainedSchedulingError::InvalidTimingWindow {
                        operation: operation.id,
                    },
                );
            }

            if let Some(deadline) =
                operation.timing.deadline
            {
                let earliest =
                    operation
                        .timing
                        .earliest_start
                        .unwrap_or(
                            TimePoint::ZERO,
                        );

                if earliest > deadline {
                    return Err(
                        ResourceConstrainedSchedulingError::InvalidTimingWindow {
                            operation: operation.id,
                        },
                    );
                }

                if operation.duration
                    .checked_add(
                        Duration::ZERO,
                    )
                    .is_none()
                {
                    return Err(
                        ResourceConstrainedSchedulingError::InvalidDuration {
                            operation: operation.id,
                        },
                    );
                }
            }
        }

        Ok(map)
    }

    fn build_successors(
        &self,
        operations: &[ResourceConstrainedOperation],
    ) -> ResourceConstrainedResult<
        BTreeMap<OperationId, Vec<OperationId>>,
    > {
        let mut successors =
            BTreeMap::<
                OperationId,
                Vec<OperationId>,
            >::new();

        for operation in operations {
            successors
                .entry(operation.id)
                .or_default();

            let mut seen =
                BTreeSet::<OperationId>::new();

            for predecessor in &operation.predecessors {
                if *predecessor == operation.id {
                    return Err(
                        ResourceConstrainedSchedulingError::CycleDetected {
                            operation: operation.id,
                        },
                    );
                }

                if !seen.insert(*predecessor) {
                    return Err(
                        ResourceConstrainedSchedulingError::InvalidModel {
                            operation: operation.id,
                        },
                    );
                }

                successors
                    .entry(*predecessor)
                    .or_default()
                    .push(operation.id);
            }
        }

        for values in successors.values_mut() {
            values.sort();
            values.dedup();
        }

        Ok(successors)
    }

    fn topological_order(
        &self,
        operations: &[ResourceConstrainedOperation],
        successors: &BTreeMap<
            OperationId,
            Vec<OperationId>,
        >,
        operation_map: &BTreeMap<
            OperationId,
            &ResourceConstrainedOperation,
        >,
    ) -> ResourceConstrainedResult<
        Vec<OperationId>,
    > {
        let mut indegree =
            BTreeMap::<OperationId, usize>::new();

        for operation in operations {
            indegree.insert(
                operation.id,
                operation.predecessors.len(),
            );
        }

        let mut ready =
            BTreeSet::<OperationId>::new();

        for operation in operations {
            if operation.predecessors.is_empty() {
                ready.insert(operation.id);
            }
        }

        let mut order =
            Vec::<OperationId>::with_capacity(
                operations.len(),
            );

        while let Some(operation) =
            ready.pop_first()
        {
            order.push(operation);

            if let Some(next) =
                successors.get(&operation)
            {
                for successor in next {
                    if !operation_map
                        .contains_key(successor)
                    {
                        return Err(
                            ResourceConstrainedSchedulingError::UnknownOperation {
                                operation: *successor,
                            },
                        );
                    }

                    let count =
                        indegree
                            .get_mut(successor)
                            .ok_or(
                                ResourceConstrainedSchedulingError::InvalidModel {
                                    operation: *successor,
                                },
                            )?;

                    if *count == 0 {
                        return Err(
                            ResourceConstrainedSchedulingError::InvalidModel {
                                operation: *successor,
                            },
                        );
                    }

                    *count -= 1;

                    if *count == 0 {
                        ready.insert(*successor);
                    }
                }
            }
        }

        if order.len() != operations.len() {
            let operation =
                operations
                    .iter()
                    .find(|operation| {
                        !order
                            .binary_search(&operation.id)
                            .is_ok()
                    })
                    .map(|operation| operation.id)
                    .unwrap_or(
                        OperationId::from(0_u64),
                    );

            return Err(
                ResourceConstrainedSchedulingError::CycleDetected {
                    operation,
                },
            );
        }

        Ok(order)
    }

    fn calculate_downstream_lengths(
        &self,
        topological: &[OperationId],
        successors: &BTreeMap<
            OperationId,
            Vec<OperationId>,
        >,
        operation_map: &BTreeMap<
            OperationId,
            &ResourceConstrainedOperation,
        >,
    ) -> ResourceConstrainedResult<
        BTreeMap<OperationId, Duration>,
    > {
        let mut downstream =
            BTreeMap::<OperationId, Duration>::new();

        for operation_id in topological.iter().rev() {
            let mut longest =
                Duration::ZERO;

            if let Some(next) =
                successors.get(operation_id)
            {
                for successor in next {
                    let successor_operation =
                        operation_map
                            .get(successor)
                            .copied()
                            .ok_or(
                                ResourceConstrainedSchedulingError::UnknownOperation {
                                    operation: *successor,
                                },
                            )?;

                    let successor_tail =
                        downstream
                            .get(successor)
                            .copied()
                            .unwrap_or(
                                Duration::ZERO,
                            );

                    let total =
                        successor_operation
                            .duration
                            .checked_add(
                                successor_tail,
                            )
                            .ok_or(
                                ResourceConstrainedSchedulingError::TimeOverflow {
                                    operation: *successor,
                                },
                            )?;

                    if total > longest {
                        longest = total;
                    }
                }
            }

            downstream.insert(
                *operation_id,
                longest,
            );
        }

        Ok(downstream)
    }

    fn initial_earliest_start(
        &self,
        operation: &ResourceConstrainedOperation,
        release_time: TimePoint,
    ) -> ResourceConstrainedResult<TimePoint> {
        let mut earliest =
            release_time;

        if let Some(operation_release) =
            operation.timing.earliest_start
        {
            if operation_release > earliest {
                earliest = operation_release;
            }
        }

        if let Some(latest_start) =
            operation.timing.latest_start
        {
            if earliest > latest_start {
                return Err(
                    ResourceConstrainedSchedulingError::InvalidTimingWindow {
                        operation: operation.id,
                    },
                );
            }
        }

        Ok(earliest)
    }

    fn predecessor_finish_time(
        &self,
        operation: &ResourceConstrainedOperation,
        scheduled: &BTreeMap<
            OperationId,
            ScheduledOperation,
        >,
        release_time: TimePoint,
    ) -> ResourceConstrainedResult<TimePoint> {
        let mut earliest =
            release_time;

        if let Some(operation_release) =
            operation.timing.earliest_start
        {
            if operation_release > earliest {
                earliest = operation_release;
            }
        }

        for predecessor in &operation.predecessors {
            let predecessor_schedule =
                scheduled
                    .get(predecessor)
                    .ok_or(
                        ResourceConstrainedSchedulingError::InvalidModel {
                            operation: operation.id,
                        },
                    )?;

            if predecessor_schedule.finish
                > earliest
            {
                earliest =
                    predecessor_schedule.finish;
            }
        }

        if let Some(latest_start) =
            operation.timing.latest_start
        {
            if earliest > latest_start {
                return Err(
                    ResourceConstrainedSchedulingError::Unschedulable {
                        operation: operation.id,
                    },
                );
            }
        }

        Ok(earliest)
    }

    fn calculate_slack(
        &self,
        operation: &ResourceConstrainedOperation,
        earliest: TimePoint,
        global_deadline: Option<TimePoint>,
    ) -> ResourceConstrainedResult<Option<Duration>> {
        let deadline =
            operation
                .timing
                .deadline
                .or(global_deadline);

        let Some(deadline) = deadline else {
            return Ok(None);
        };

        let finish =
            operation.finish(earliest)?;

        if finish > deadline {
            return Err(
                ResourceConstrainedSchedulingError::DeadlineExceeded {
                    operation: operation.id,
                },
            );
        }

        Ok(
            finish
                .checked_duration_until(
                    deadline,
                ),
        )
    }

    fn validate_global_deadline(
        &self,
        operation: OperationId,
        finish: TimePoint,
        deadline: Option<TimePoint>,
    ) -> ResourceConstrainedResult<()> {
        if let Some(deadline) = deadline {
            if finish > deadline {
                return Err(
                    ResourceConstrainedSchedulingError::DeadlineExceeded {
                        operation,
                    },
                );
            }
        }

        Ok(())
    }

    fn find_feasible_start<R>(
        &self,
        operation: &ResourceConstrainedOperation,
        earliest: TimePoint,
        resources: &R,
        reservations: &[Reservation],
    ) -> ResourceConstrainedResult<TimePoint>
    where
        R: ResourceModel,
    {
        self.validate_resource_requirements(
            operation,
            resources,
        )?;

        let mut candidate =
            earliest;

        loop {
            let finish =
                operation.finish(candidate)?;

            if let Some(latest_start) =
                operation.timing.latest_start
            {
                if candidate > latest_start {
                    return Err(
                        ResourceConstrainedSchedulingError::Unschedulable {
                            operation: operation.id,
                        },
                    );
                }
            }

            if let Some(deadline) =
                operation.timing.deadline
            {
                if finish > deadline {
                    return Err(
                        ResourceConstrainedSchedulingError::DeadlineExceeded {
                            operation: operation.id,
                        },
                    );
                }
            }

            let mut next_boundary =
                None::<TimePoint>;

            for requirement in &operation.resources {
                if requirement.amount == 0 {
                    continue;
                }

                if !resources.available(
                    requirement.resource,
                    candidate,
                    finish,
                    requirement.amount,
                    reservations,
                ) {
                    let boundary =
                        self.next_resource_boundary(
                            requirement.resource,
                            candidate,
                            reservations,
                        )?;

                    next_boundary =
                        Some(match next_boundary {
                            Some(current)
                                if current > boundary =>
                            {
                                current
                            }
                            _ => boundary,
                        });
                }
            }

            match next_boundary {
                Some(next) if next > candidate => {
                    candidate = next;
                }

                Some(_) => {
                    return Err(
                        ResourceConstrainedSchedulingError::Unschedulable {
                            operation: operation.id,
                        },
                    );
                }

                None => {
                    return Ok(candidate);
                }
            }
        }
    }

    fn validate_resource_requirements<R>(
        &self,
        operation: &ResourceConstrainedOperation,
        resources: &R,
    ) -> ResourceConstrainedResult<()>
    where
        R: ResourceModel,
    {
        let mut aggregate =
            BTreeMap::<
                crate::quantum::ir::core::identity::ResourceId,
                u64,
            >::new();

        for requirement in &operation.resources {
            if requirement.amount == 0 {
                continue;
            }

            let capacity =
                resources
                    .capacity(
                        requirement.resource,
                    )
                    .ok_or(
                        ResourceConstrainedSchedulingError::UnknownResource {
                            operation: operation.id,
                            resource: requirement.resource,
                        },
                    )?;

            let current =
                aggregate
                    .get(
                        &requirement.resource,
                    )
                    .copied()
                    .unwrap_or(0);

            let total =
                current
                    .checked_add(
                        requirement.amount,
                    )
                    .ok_or(
                        ResourceConstrainedSchedulingError::CapacityExceeded {
                            operation: operation.id,
                            resource: requirement.resource,
                        },
                    )?;

            if total > capacity {
                return Err(
                    ResourceConstrainedSchedulingError::CapacityExceeded {
                        operation: operation.id,
                        resource: requirement.resource,
                    },
                );
            }

            aggregate.insert(
                requirement.resource,
                total,
            );
        }

        Ok(())
    }

    fn build_reservations<R>(
        &self,
        operation: &ResourceConstrainedOperation,
        start: TimePoint,
        finish: TimePoint,
        resources: &R,
        reservations: &[Reservation],
    ) -> ResourceConstrainedResult<Vec<Reservation>>
    where
        R: ResourceModel,
    {
        self.validate_resource_requirements(
            operation,
            resources,
        )?;

        let mut result =
            Vec::<Reservation>::new();

        for requirement in &operation.resources {
            if requirement.amount == 0 {
                continue;
            }

            if !resources.available(
                requirement.resource,
                start,
                finish,
                requirement.amount,
                reservations,
            ) {
                return Err(
                    ResourceConstrainedSchedulingError::Unschedulable {
                        operation: operation.id,
                    },
                );
            }

            result.push(
                Reservation::new(
                    operation.id,
                    requirement.resource,
                    start,
                    finish,
                    requirement.amount,
                ),
            );
        }

        Ok(result)
    }

    fn next_resource_boundary(
        &self,
        resource: crate::quantum::ir::core::identity::ResourceId,
        candidate: TimePoint,
        reservations: &[Reservation],
    ) -> ResourceConstrainedResult<TimePoint> {
        let mut boundary =
            None::<TimePoint>;

        for reservation in reservations {
            if reservation.resource != resource {
                continue;
            }

            if reservation.finish <= candidate {
                continue;
            }

            let next =
                if reservation.start > candidate {
                    reservation.start
                } else {
                    reservation.finish
                };

            boundary =
                Some(match boundary {
                    Some(current)
                        if current < next =>
                    {
                        current
                    }
                    _ => next,
                });
        }

        boundary.ok_or(
            ResourceConstrainedSchedulingError::Unschedulable {
                operation: OperationId::from(0_u64),
            },
        )
    }
}

/// Convenience function for one-shot RCPSP scheduling.
///
/// This function uses production defaults and introduces no machine-size
/// assumptions.
pub fn schedule<R>(
    problem: &ResourceConstrainedProblem<'_, R>,
) -> ResourceConstrainedResult<ResourceConstrainedSchedule>
where
    R: ResourceModel,
{
    RcpspScheduler::new().schedule(problem)
}

/// Returns the stable RCPSP algorithm identifier.
#[must_use]
pub const fn algorithm_id() -> &'static str {
    RCPSP_ALGORITHM_ID
}

/// Returns the stable RCPSP algorithm name.
#[must_use]
pub const fn algorithm_name() -> &'static str {
    RCPSP_ALGORITHM_NAME
}

/// Returns the RCPSP implementation version.
#[must_use]
pub const fn algorithm_version() -> u32 {
    RCPSP_ALGORITHM_VERSION
}

/// Returns whether the implementation is deterministic.
#[must_use]
pub const fn is_deterministic() -> bool {
    true
}

/// Returns whether the implementation introduces an artificial machine-size
/// limit.
///
/// `false` means the algorithm has no scheduler-defined finite machine-size
/// ceiling.
#[must_use]
pub const fn has_machine_size_limit() -> bool {
    false
}

/// Returns whether the implementation uses unsafe Rust.
///
/// Always false; this module has a compile-time `forbid(unsafe_code)` boundary.
#[must_use]
pub const fn uses_unsafe() -> bool {
    false
}

/// Returns whether the algorithm is dependency aware.
#[must_use]
pub const fn is_dependency_aware() -> bool {
    true
}

/// Returns whether the algorithm is resource aware.
#[must_use]
pub const fn is_resource_aware() -> bool {
    true
}

/// Returns whether the algorithm is timing aware.
#[must_use]
pub const fn is_timing_aware() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::core::identity::ResourceId;

    #[derive(Debug, Default)]
    struct TestResources {
        capacities: BTreeMap<ResourceId, u64>,
    }

    impl TestResources {
        fn with(
            mut self,
            resource: ResourceId,
            capacity: u64,
        ) -> Self {
            self.capacities
                .insert(resource, capacity);

            self
        }
    }

    impl ResourceModel for TestResources {
        fn capacity(
            &self,
            resource: ResourceId,
        ) -> Option<u64> {
            self.capacities
                .get(&resource)
                .copied()
        }

        fn available(
            &self,
            resource: ResourceId,
            start: TimePoint,
            finish: TimePoint,
            amount: u64,
            reservations: &[Reservation],
        ) -> bool {
            let capacity =
                match self.capacity(resource) {
                    Some(capacity) => capacity,
                    None => return false,
                };

            let mut used = 0_u64;

            for reservation in reservations {
                if reservation.resource != resource {
                    continue;
                }

                if reservation.start < finish
                    && start < reservation.finish
                {
                    used =
                        match used.checked_add(
                            reservation.amount,
                        ) {
                            Some(value) => value,
                            None => return false,
                        };
                }
            }

            match used.checked_add(amount) {
                Some(total) => total <= capacity,
                None => false,
            }
        }
    }

    fn operation(
        id: u64,
        duration: u128,
    ) -> ResourceConstrainedOperation {
        ResourceConstrainedOperation::new(
            OperationId::from(id),
            Duration::new(duration),
        )
    }

    #[test]
    fn empty_problem_is_allowed_by_default() {
        let operations =
            Vec::<ResourceConstrainedOperation>::new();

        let resources =
            TestResources::default();

        let problem =
            ResourceConstrainedProblem::new(
                &operations,
                &resources,
            );

        let result =
            RcpspScheduler::new()
                .schedule(&problem)
                .expect("empty problem must be accepted");

        assert_eq!(
            result.operation_count,
            0
        );
    }

    #[test]
    fn dependency_chain_is_preserved() {
        let operations =
            vec![
                operation(1, 10),
                operation(2, 10)
                    .with_predecessor(
                        OperationId::from(1),
                    ),
            ];

        let resources =
            TestResources::default();

        let problem =
            ResourceConstrainedProblem::new(
                &operations,
                &resources,
            );

        let result =
            schedule(&problem)
                .expect("dependency chain should schedule");

        let first =
            result
                .operation(
                    OperationId::from(1),
                )
                .expect("first operation");

        let second =
            result
                .operation(
                    OperationId::from(2),
                )
                .expect("second operation");

        assert!(
            second.start >= first.finish
        );
    }

    #[test]
    fn exclusive_resource_serializes_operations() {
        let resource =
            ResourceId::from(1_u64);

        let operations =
            vec![
                operation(1, 10)
                    .with_resources(
                        vec![
                            crate::quantum::scheduling
                                ::planners
                                ::resource_constrained
                                ::ResourceRequirement
                                ::new(
                                    resource,
                                    1,
                                ),
                        ],
                    ),
                operation(2, 10)
                    .with_resources(
                        vec![
                            crate::quantum::scheduling
                                ::planners
                                ::resource_constrained
                                ::ResourceRequirement
                                ::new(
                                    resource,
                                    1,
                                ),
                        ],
                    ),
            ];

        let resources =
            TestResources::default()
                .with(resource, 1);

        let problem =
            ResourceConstrainedProblem::new(
                &operations,
                &resources,
            );

        let result =
            schedule(&problem)
                .expect("resource-constrained schedule should succeed");

        assert_eq!(
            result.makespan,
            TimePoint::new(20)
        );
    }

    #[test]
    fn shared_resource_allows_parallelism() {
        let resource =
            ResourceId::from(1_u64);

        let operations =
            vec![
                operation(1, 10)
                    .with_resources(
                        vec![
                            crate::quantum::scheduling
                                ::planners
                                ::resource_constrained
                                ::ResourceRequirement
                                ::new(
                                    resource,
                                    1,
                                ),
                        ],
                    ),
                operation(2, 10)
                    .with_resources(
                        vec![
                            crate::quantum::scheduling
                                ::planners
                                ::resource_constrained
                                ::ResourceRequirement
                                ::new(
                                    resource,
                                    1,
                                ),
                        ],
                    ),
            ];

        let resources =
            TestResources::default()
                .with(resource, 2);

        let problem =
            ResourceConstrainedProblem::new(
                &operations,
                &resources,
            );

        let result =
            schedule(&problem)
                .expect("shared resource should allow parallelism");

        assert_eq!(
            result.makespan,
            TimePoint::new(10)
        );
    }

    #[test]
    fn cycle_is_rejected() {
        let operations =
            vec![
                operation(1, 1)
                    .with_predecessor(
                        OperationId::from(2),
                    ),
                operation(2, 1)
                    .with_predecessor(
                        OperationId::from(1),
                    ),
            ];

        let resources =
            TestResources::default();

        let problem =
            ResourceConstrainedProblem::new(
                &operations,
                &resources,
            );

        let result =
            schedule(&problem);

        assert!(matches!(
            result,
            Err(
                ResourceConstrainedSchedulingError::CycleDetected {
                    ..
                }
            )
        ));
    }

    #[test]
    fn deadline_is_enforced() {
        let operations =
            vec![
                operation(1, 10)
                    .with_timing(
                        crate::quantum::scheduling
                            ::planners
                            ::resource_constrained
                            ::TimingBounds {
                                earliest_start: None,
                                latest_start: None,
                                deadline: Some(
                                    TimePoint::new(
                                        5,
                                    ),
                                ),
                            },
                    ),
            ];

        let resources =
            TestResources::default();

        let problem =
            ResourceConstrainedProblem::new(
                &operations,
                &resources,
            );

        let result =
            schedule(&problem);

        assert!(matches!(
            result,
            Err(
                ResourceConstrainedSchedulingError::DeadlineExceeded {
                    ..
                }
            )
        ));
    }

    #[test]
    fn latest_start_is_enforced() {
        let operations =
            vec![
                operation(1, 10)
                    .with_timing(
                        crate::quantum::scheduling
                            ::planners
                            ::resource_constrained
                            ::TimingBounds {
                                earliest_start: Some(
                                    TimePoint::new(
                                        5,
                                    ),
                                ),
                                latest_start: Some(
                                    TimePoint::new(
                                        4,
                                    ),
                                ),
                                deadline: None,
                            },
                    ),
            ];

        let resources =
            TestResources::default();

        let problem =
            ResourceConstrainedProblem::new(
                &operations,
                &resources,
            );

        let result =
            schedule(&problem);

        assert!(matches!(
            result,
            Err(
                ResourceConstrainedSchedulingError::InvalidTimingWindow {
                    ..
                }
            )
        ));
    }

    #[test]
    fn deterministic_tie_breaking_uses_operation_id() {
        let first =
            CandidateKey::new(
                None,
                0,
                Duration::ZERO,
                OperationId::from(1),
            );

        let second =
            CandidateKey::new(
                None,
                0,
                Duration::ZERO,
                OperationId::from(2),
            );

        assert!(
            first > second,
            "smaller operation identity must win"
        );
    }

    #[test]
    fn algorithm_contract_reports_no_machine_limit() {
        assert!(!has_machine_size_limit());
        assert!(is_dependency_aware());
        assert!(is_resource_aware());
        assert!(is_timing_aware());
        assert!(!uses_unsafe());
    }
}