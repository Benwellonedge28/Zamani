//! Production-grade resource accounting for Zamani QEC.
//!
//! This module provides bounded, thread-safe resource accounting for
//! quantum-error-correction workloads.
//!
//! The design target is not literally infinite execution. Instead,
//! arbitrarily large workloads are supported subject to explicit,
//! configurable resource policies.
//!
//! # Tracked resources
//!
//! - allocated memory
//! - peak memory
//! - syndrome events
//! - decoding-graph nodes
//! - decoding-graph edges
//! - decoder iterations
//! - parallel workers
//! - wall-clock time
//! - backend-reported compute/CPU time
//!
//! # Safety model
//!
//! Resource exhaustion is represented as `Result` errors rather than
//! panics. Memory and worker reservations use RAII so resources are
//! automatically released when their guards are dropped.
//!
//! This module intentionally has no dependency on other future QEC
//! infrastructure modules such as `limits.rs`, `errors.rs`, or
//! `scheduler.rs`. Those modules can build on these primitives.

use std::fmt;
use std::sync::atomic::{
    AtomicBool,
    AtomicU64,
    AtomicUsize,
    Ordering,
};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Sentinel representing an explicitly unlimited resource dimension.
///
/// "Unlimited" here means that Zamani does not impose an additional
/// application-level limit. It does not mean that physical memory,
/// address space, integer ranges, or execution time are infinite.
pub const UNLIMITED_U64: u64 = u64::MAX;

/// Sentinel representing unlimited parallelism at the policy layer.
pub const UNLIMITED_USIZE: usize = usize::MAX;

/// Resource dimensions tracked by the QEC resource manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    MemoryBytes,
    SyndromeEvents,
    GraphNodes,
    GraphEdges,
    DecoderIterations,
    ParallelWorkers,
}

impl fmt::Display for ResourceKind {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let name = match self {
            Self::MemoryBytes => "memory bytes",
            Self::SyndromeEvents => "syndrome events",
            Self::GraphNodes => "graph nodes",
            Self::GraphEdges => "graph edges",
            Self::DecoderIterations => "decoder iterations",
            Self::ParallelWorkers => "parallel workers",
        };

        f.write_str(name)
    }
}

/// Global QEC resource policy.
///
/// Every finite resource limit must be greater than zero.
///
/// `u64::MAX` and `usize::MAX` are accepted as explicit unlimited
/// sentinels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceLimits {
    /// Maximum simultaneously allocated QEC memory.
    pub max_memory_bytes: u64,

    /// Maximum syndrome events processed by the workload.
    pub max_syndrome_events: u64,

    /// Maximum decoding graph nodes.
    pub max_graph_nodes: u64,

    /// Maximum decoding graph edges.
    pub max_graph_edges: u64,

    /// Maximum decoder iterations.
    pub max_decoder_iterations: u64,

    /// Maximum number of concurrently registered workers.
    pub max_parallelism: usize,

    /// Optional wall-clock deadline for the resource manager.
    pub max_wall_time: Option<Duration>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_bytes: 1024 * 1024 * 1024,
            max_syndrome_events: 100_000_000,
            max_graph_nodes: 100_000_000,
            max_graph_edges: 500_000_000,
            max_decoder_iterations: 1_000_000_000,
            max_parallelism: 256,
            max_wall_time: None,
        }
    }
}

impl ResourceLimits {
    /// Creates a policy without application-level finite limits.
    pub const fn unlimited() -> Self {
        Self {
            max_memory_bytes: UNLIMITED_U64,
            max_syndrome_events: UNLIMITED_U64,
            max_graph_nodes: UNLIMITED_U64,
            max_graph_edges: UNLIMITED_U64,
            max_decoder_iterations: UNLIMITED_U64,
            max_parallelism: UNLIMITED_USIZE,
            max_wall_time: None,
        }
    }

    /// Validates the resource policy itself.
    pub fn validate(&self) -> Result<(), ResourceError> {
        if self.max_memory_bytes == 0
            || self.max_syndrome_events == 0
            || self.max_graph_nodes == 0
            || self.max_graph_edges == 0
            || self.max_decoder_iterations == 0
            || self.max_parallelism == 0
        {
            return Err(
                ResourceError::InvalidLimit {
                    reason:
                        "finite resource limits must be greater than zero",
                },
            );
        }

        Ok(())
    }
}

/// Optional stricter quota for an individual decoding operation.
///
/// Global limits still apply even when a quota is configured.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceQuota {
    pub max_memory_bytes: Option<u64>,
    pub max_syndrome_events: Option<u64>,
    pub max_graph_nodes: Option<u64>,
    pub max_graph_edges: Option<u64>,
    pub max_decoder_iterations: Option<u64>,
    pub max_parallelism: Option<usize>,
    pub max_wall_time: Option<Duration>,
}

impl Default for ResourceQuota {
    fn default() -> Self {
        Self {
            max_memory_bytes: None,
            max_syndrome_events: None,
            max_graph_nodes: None,
            max_graph_edges: None,
            max_decoder_iterations: None,
            max_parallelism: None,
            max_wall_time: None,
        }
    }
}

impl ResourceQuota {
    pub fn validate(&self) -> Result<(), ResourceError> {
        let limits = [
            self.max_memory_bytes,
            self.max_syndrome_events,
            self.max_graph_nodes,
            self.max_graph_edges,
            self.max_decoder_iterations,
        ];

        if limits.iter().flatten().any(|value| *value == 0) {
            return Err(
                ResourceError::InvalidLimit {
                    reason:
                        "finite operation quotas must be greater than zero",
                },
            );
        }

        if self.max_parallelism == Some(0) {
            return Err(
                ResourceError::InvalidLimit {
                    reason:
                        "operation parallelism quota must be greater than zero",
                },
            );
        }

        Ok(())
    }
}

/// Immutable point-in-time resource state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceSnapshot {
    pub allocated_bytes: u64,
    pub peak_bytes: u64,
    pub syndrome_events: u64,
    pub graph_nodes: u64,
    pub graph_edges: u64,
    pub decoder_iterations: u64,
    pub parallel_workers: usize,

    /// Wall-clock time since manager creation.
    pub wall_time: Duration,

    /// Backend-reported compute/CPU time.
    ///
    /// Rust's portable standard library does not expose process CPU time,
    /// so this value must be supplied by the execution backend.
    pub compute_time: Duration,
}

impl ResourceSnapshot {
    pub fn is_idle(&self) -> bool {
        self.allocated_bytes == 0
            && self.parallel_workers == 0
    }
}

/// Structured resource errors.
///
/// The eventual `errors.rs` module can wrap or translate these into the
/// unified `QecError` hierarchy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceError {
    InvalidLimit {
        reason: &'static str,
    },

    LimitExceeded {
        resource: ResourceKind,
        requested: u64,
        current: u64,
        limit: u64,
    },

    ParallelismLimitExceeded {
        requested: usize,
        current: usize,
        limit: usize,
    },

    QuotaExceeded {
        resource: ResourceKind,
        requested: u64,
        current: u64,
        limit: u64,
    },

    ParallelismQuotaExceeded {
        requested: usize,
        current: usize,
        limit: usize,
    },

    ArithmeticOverflow {
        resource: ResourceKind,
    },

    WallTimeLimitExceeded {
        elapsed: Duration,
        limit: Duration,
    },

    Cancelled,
}

impl fmt::Display for ResourceError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidLimit { reason } => {
                write!(f, "invalid resource policy: {reason}")
            }

            Self::LimitExceeded {
                resource,
                requested,
                current,
                limit,
            } => {
                write!(
                    f,
                    "{resource} limit exceeded: \
                     requested {requested}, \
                     current {current}, \
                     limit {limit}"
                )
            }

            Self::ParallelismLimitExceeded {
                requested,
                current,
                limit,
            } => {
                write!(
                    f,
                    "parallelism limit exceeded: \
                     requested {requested}, \
                     current {current}, \
                     limit {limit}"
                )
            }

            Self::QuotaExceeded {
                resource,
                requested,
                current,
                limit,
            } => {
                write!(
                    f,
                    "{resource} operation quota exceeded: \
                     requested {requested}, \
                     current {current}, \
                     quota {limit}"
                )
            }

            Self::ParallelismQuotaExceeded {
                requested,
                current,
                limit,
            } => {
                write!(
                    f,
                    "parallelism operation quota exceeded: \
                     requested {requested}, \
                     current {current}, \
                     quota {limit}"
                )
            }

            Self::ArithmeticOverflow { resource } => {
                write!(
                    f,
                    "resource counter overflow for {resource}"
                )
            }

            Self::WallTimeLimitExceeded {
                elapsed,
                limit,
            } => {
                write!(
                    f,
                    "wall-time limit exceeded: \
                     elapsed {elapsed:?}, \
                     limit {limit:?}"
                )
            }

            Self::Cancelled => {
                f.write_str("resource operation cancelled")
            }
        }
    }
}

impl std::error::Error for ResourceError {}

#[derive(Debug, Default)]
struct ResourceCounters {
    allocated_bytes: AtomicU64,
    peak_bytes: AtomicU64,

    syndrome_events: AtomicU64,
    graph_nodes: AtomicU64,
    graph_edges: AtomicU64,
    decoder_iterations: AtomicU64,

    parallel_workers: AtomicUsize,

    compute_time_nanos: AtomicU64,

    cancelled: AtomicBool,
}

/// Thread-safe resource manager.
///
/// It can safely be shared between QEC decoder workers through `Arc`.
#[derive(Debug)]
pub struct ResourceManager {
    limits: ResourceLimits,
    counters: ResourceCounters,
    started: Instant,
}

impl ResourceManager {
    /// Creates a resource manager after validating its policy.
    pub fn new(
        limits: ResourceLimits,
    ) -> Result<Self, ResourceError> {
        limits.validate()?;

        Ok(Self {
            limits,
            counters: ResourceCounters::default(),
            started: Instant::now(),
        })
    }

    /// Returns the configured global limits.
    pub const fn limits(&self) -> ResourceLimits {
        self.limits
    }

    // ---------------------------------------------------------------------
    // Cancellation
    // ---------------------------------------------------------------------

    /// Requests cancellation.
    pub fn cancel(&self) {
        self.counters
            .cancelled
            .store(true, Ordering::Release);
    }

    /// Clears cancellation.
    ///
    /// This should normally only be called before beginning a new logical
    /// operation.
    pub fn reset_cancellation(&self) {
        self.counters
            .cancelled
            .store(false, Ordering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.counters
            .cancelled
            .load(Ordering::Acquire)
    }

    /// Checks cancellation and the global wall-time limit.
    pub fn check(&self) -> Result<(), ResourceError> {
        if self.is_cancelled() {
            return Err(ResourceError::Cancelled);
        }

        if let Some(limit) = self.limits.max_wall_time {
            let elapsed = self.started.elapsed();

            if elapsed > limit {
                return Err(
                    ResourceError::WallTimeLimitExceeded {
                        elapsed,
                        limit,
                    },
                );
            }
        }

        Ok(())
    }

    // ---------------------------------------------------------------------
    // Snapshots
    // ---------------------------------------------------------------------

    pub fn snapshot(&self) -> ResourceSnapshot {
        ResourceSnapshot {
            allocated_bytes: self
                .counters
                .allocated_bytes
                .load(Ordering::Acquire),

            peak_bytes: self
                .counters
                .peak_bytes
                .load(Ordering::Acquire),

            syndrome_events: self
                .counters
                .syndrome_events
                .load(Ordering::Acquire),

            graph_nodes: self
                .counters
                .graph_nodes
                .load(Ordering::Acquire),

            graph_edges: self
                .counters
                .graph_edges
                .load(Ordering::Acquire),

            decoder_iterations: self
                .counters
                .decoder_iterations
                .load(Ordering::Acquire),

            parallel_workers: self
                .counters
                .parallel_workers
                .load(Ordering::Acquire),

            wall_time: self.started.elapsed(),

            compute_time: Duration::from_nanos(
                self.counters
                    .compute_time_nanos
                    .load(Ordering::Acquire),
            ),
        }
    }

    // ---------------------------------------------------------------------
    // Memory
    // ---------------------------------------------------------------------

    /// Reserves memory using the global limit.
    pub fn reserve_memory(
        &self,
        bytes: u64,
    ) -> Result<MemoryReservation<'_>, ResourceError> {
        self.reserve_memory_with_quota(bytes, None)?;

        Ok(MemoryReservation {
            manager: self,
            bytes,
            active: true,
        })
    }

    /// Reserves memory subject to an optional per-operation quota.
    pub fn reserve_memory_with_quota(
        &self,
        bytes: u64,
        quota: Option<u64>,
    ) -> Result<(), ResourceError> {
        self.check()?;

        self.try_add(
            ResourceKind::MemoryBytes,
            &self.counters.allocated_bytes,
            bytes,
            self.limits.max_memory_bytes,
            quota,
        )?;

        self.update_peak();

        Ok(())
    }

    /// Releases memory previously reserved.
    pub fn release_memory(&self, bytes: u64) {
        saturating_sub_u64(
            &self.counters.allocated_bytes,
            bytes,
        );
    }

    // ---------------------------------------------------------------------
    // Syndrome events
    // ---------------------------------------------------------------------

    pub fn record_syndrome_events(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.check()?;

        self.try_add(
            ResourceKind::SyndromeEvents,
            &self.counters.syndrome_events,
            count,
            self.limits.max_syndrome_events,
            None,
        )
    }

    // ---------------------------------------------------------------------
    // Graph nodes
    // ---------------------------------------------------------------------

    pub fn record_graph_nodes(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.check()?;

        self.try_add(
            ResourceKind::GraphNodes,
            &self.counters.graph_nodes,
            count,
            self.limits.max_graph_nodes,
            None,
        )
    }

    // ---------------------------------------------------------------------
    // Graph edges
    // ---------------------------------------------------------------------

    pub fn record_graph_edges(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.check()?;

        self.try_add(
            ResourceKind::GraphEdges,
            &self.counters.graph_edges,
            count,
            self.limits.max_graph_edges,
            None,
        )
    }

    // ---------------------------------------------------------------------
    // Decoder iterations
    // ---------------------------------------------------------------------

    pub fn record_decoder_iterations(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.check()?;

        self.try_add(
            ResourceKind::DecoderIterations,
            &self.counters.decoder_iterations,
            count,
            self.limits.max_decoder_iterations,
            None,
        )
    }

    // ---------------------------------------------------------------------
    // Worker accounting
    // ---------------------------------------------------------------------

    pub fn acquire_workers(
        &self,
        workers: usize,
    ) -> Result<WorkerReservation<'_>, ResourceError> {
        self.acquire_workers_with_quota(workers, None)?;

        Ok(WorkerReservation {
            manager: self,
            workers,
            active: true,
        })
    }

    pub fn acquire_workers_with_quota(
        &self,
        workers: usize,
        quota: Option<usize>,
    ) -> Result<(), ResourceError> {
        self.check()?;

        if workers == 0 {
            return Ok(());
        }

        let mut current = self
            .counters
            .parallel_workers
            .load(Ordering::Acquire);

        loop {
            let next = current.checked_add(workers).ok_or(
                ResourceError::ArithmeticOverflow {
                    resource:
                        ResourceKind::ParallelWorkers,
                },
            )?;

            if next > self.limits.max_parallelism {
                return Err(
                    ResourceError::ParallelismLimitExceeded {
                        requested: workers,
                        current,
                        limit: self.limits.max_parallelism,
                    },
                );
            }

            if let Some(limit) = quota {
                if next > limit {
                    return Err(
                        ResourceError::ParallelismQuotaExceeded {
                            requested: workers,
                            current,
                            limit,
                        },
                    );
                }
            }

            match self
                .counters
                .parallel_workers
                .compare_exchange_weak(
                    current,
                    next,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
            {
                Ok(_) => return Ok(()),

                Err(actual) => {
                    current = actual;
                }
            }
        }
    }

    pub fn release_workers(&self, workers: usize) {
        saturating_sub_usize(
            &self.counters.parallel_workers,
            workers,
        );
    }

    // ---------------------------------------------------------------------
    // Compute/CPU accounting
    // ---------------------------------------------------------------------

    /// Records backend-measured compute time.
    ///
    /// This is deliberately separate from wall-clock time. A portable
    /// Rust 1.70 implementation cannot directly obtain process CPU time
    /// through the standard library.
    pub fn record_compute_time(
        &self,
        duration: Duration,
    ) -> Result<(), ResourceError> {
        let nanos = u64::try_from(
            duration.as_nanos(),
        )
        .map_err(|_| {
            ResourceError::ArithmeticOverflow {
                resource:
                    ResourceKind::DecoderIterations,
            }
        })?;

        let mut current = self
            .counters
            .compute_time_nanos
            .load(Ordering::Acquire);

        loop {
            let next = current.checked_add(nanos).ok_or(
                ResourceError::ArithmeticOverflow {
                    resource:
                        ResourceKind::DecoderIterations,
                },
            )?;

            match self
                .counters
                .compute_time_nanos
                .compare_exchange_weak(
                    current,
                    next,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
            {
                Ok(_) => return Ok(()),

                Err(actual) => {
                    current = actual;
                }
            }
        }
    }

    // ---------------------------------------------------------------------
    // Operation scopes
    // ---------------------------------------------------------------------

    pub fn scope<'a>(
        &'a self,
        name: impl Into<String>,
        quota: ResourceQuota,
    ) -> Result<ResourceScope<'a>, ResourceError> {
        quota.validate()?;
        self.check()?;

        Ok(ResourceScope {
            manager: self,
            name: name.into(),
            quota,
            started: Instant::now(),
        })
    }

    fn try_add(
        &self,
        resource: ResourceKind,
        counter: &AtomicU64,
        requested: u64,
        global_limit: u64,
        quota: Option<u64>,
    ) -> Result<(), ResourceError> {
        let mut current =
            counter.load(Ordering::Acquire);

        loop {
            let next = current
                .checked_add(requested)
                .ok_or(
                    ResourceError::ArithmeticOverflow {
                        resource,
                    },
                )?;

            if next > global_limit {
                return Err(
                    ResourceError::LimitExceeded {
                        resource,
                        requested,
                        current,
                        limit: global_limit,
                    },
                );
            }

            if let Some(limit) = quota {
                if next > limit {
                    return Err(
                        ResourceError::QuotaExceeded {
                            resource,
                            requested,
                            current,
                            limit,
                        },
                    );
                }
            }

            match counter.compare_exchange_weak(
                current,
                next,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return Ok(()),

                Err(actual) => {
                    current = actual;
                }
            }
        }
    }

    fn update_peak(&self) {
        let current = self
            .counters
            .allocated_bytes
            .load(Ordering::Acquire);

        let mut peak = self
            .counters
            .peak_bytes
            .load(Ordering::Acquire);

        while current > peak {
            match self
                .counters
                .peak_bytes
                .compare_exchange_weak(
                    peak,
                    current,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
            {
                Ok(_) => break,

                Err(actual) => {
                    peak = actual;
                }
            }
        }
    }
}

/// RAII memory reservation.
pub struct MemoryReservation<'a> {
    manager: &'a ResourceManager,
    bytes: u64,
    active: bool,
}

impl<'a> MemoryReservation<'a> {
    pub fn bytes(&self) -> u64 {
        self.bytes
    }

    /// Explicitly releases the reservation.
    pub fn release(
        mut self,
    ) -> bool {
        if !self.active {
            return false;
        }

        self.manager
            .release_memory(self.bytes);

        self.active = false;

        true
    }
}

impl Drop for MemoryReservation<'_> {
    fn drop(&mut self) {
        if self.active {
            self.manager
                .release_memory(self.bytes);

            self.active = false;
        }
    }
}

/// RAII worker reservation.
pub struct WorkerReservation<'a> {
    manager: &'a ResourceManager,
    workers: usize,
    active: bool,
}

impl<'a> WorkerReservation<'a> {
    pub fn workers(&self) -> usize {
        self.workers
    }

    pub fn release(
        mut self,
    ) -> bool {
        if !self.active {
            return false;
        }

        self.manager
            .release_workers(self.workers);

        self.active = false;

        true
    }
}

impl Drop for WorkerReservation<'_> {
    fn drop(&mut self) {
        if self.active {
            self.manager
                .release_workers(self.workers);

            self.active = false;
        }
    }
}

/// A named QEC operation with a stricter resource quota.
pub struct ResourceScope<'a> {
    manager: &'a ResourceManager,
    name: String,
    quota: ResourceQuota,
    started: Instant,
}

impl<'a> ResourceScope<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn quota(&self) -> ResourceQuota {
        self.quota
    }

    pub fn elapsed(&self) -> Duration {
        self.started.elapsed()
    }

    /// Checks both global and per-operation limits.
    pub fn check(&self) -> Result<(), ResourceError> {
        self.manager.check()?;

        if let Some(limit) = self.quota.max_wall_time {
            let elapsed = self.elapsed();

            if elapsed > limit {
                return Err(
                    ResourceError::WallTimeLimitExceeded {
                        elapsed,
                        limit,
                    },
                );
            }
        }

        Ok(())
    }

    /// Reserves memory under this operation's quota.
    pub fn reserve_memory(
        &self,
        bytes: u64,
    ) -> Result<MemoryReservation<'a>, ResourceError> {
        self.manager.reserve_memory_with_quota(
            bytes,
            self.quota.max_memory_bytes,
        )?;

        Ok(MemoryReservation {
            manager: self.manager,
            bytes,
            active: true,
        })
    }

    /// Records syndrome events under this operation's quota.
    pub fn record_syndrome_events(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.manager.check()?;

        self.manager.try_add(
            ResourceKind::SyndromeEvents,
            &self.manager.counters.syndrome_events,
            count,
            self.manager.limits.max_syndrome_events,
            self.quota.max_syndrome_events,
        )
    }

    /// Records graph nodes under this operation's quota.
    pub fn record_graph_nodes(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.manager.check()?;

        self.manager.try_add(
            ResourceKind::GraphNodes,
            &self.manager.counters.graph_nodes,
            count,
            self.manager.limits.max_graph_nodes,
            self.quota.max_graph_nodes,
        )
    }

    /// Records graph edges under this operation's quota.
    pub fn record_graph_edges(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.manager.check()?;

        self.manager.try_add(
            ResourceKind::GraphEdges,
            &self.manager.counters.graph_edges,
            count,
            self.manager.limits.max_graph_edges,
            self.quota.max_graph_edges,
        )
    }

    /// Records decoder iterations under this operation's quota.
    pub fn record_decoder_iterations(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.manager.check()?;

        self.manager.try_add(
            ResourceKind::DecoderIterations,
            &self.manager.counters.decoder_iterations,
            count,
            self.manager.limits.max_decoder_iterations,
            self.quota.max_decoder_iterations,
        )
    }

    /// Acquires worker slots under this operation's quota.
    pub fn acquire_workers(
        &self,
        workers: usize,
    ) -> Result<WorkerReservation<'a>, ResourceError> {
        self.manager.acquire_workers_with_quota(
            workers,
            self.quota.max_parallelism,
        )?;

        Ok(WorkerReservation {
            manager: self.manager,
            workers,
            active: true,
        })
    }

    pub fn snapshot(&self) -> ResourceSnapshot {
        self.manager.snapshot()
    }
}

/// Creates a shareable resource manager.
pub fn shared(
    limits: ResourceLimits,
) -> Result<Arc<ResourceManager>, ResourceError> {
    Ok(Arc::new(
        ResourceManager::new(limits)?,
    ))
}

fn saturating_sub_u64(
    counter: &AtomicU64,
    amount: u64,
) {
    let mut current =
        counter.load(Ordering::Acquire);

    loop {
        let next =
            current.saturating_sub(amount);

        match counter.compare_exchange_weak(
            current,
            next,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => return,

            Err(actual) => {
                current = actual;
            }
        }
    }
}

fn saturating_sub_usize(
    counter: &AtomicUsize,
    amount: usize,
) {
    let mut current =
        counter.load(Ordering::Acquire);

    loop {
        let next =
            current.saturating_sub(amount);

        match counter.compare_exchange_weak(
            current,
            next,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => return,

            Err(actual) => {
                current = actual;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    fn test_limits() -> ResourceLimits {
        ResourceLimits {
            max_memory_bytes: 1024,
            max_syndrome_events: 10,
            max_graph_nodes: 10,
            max_graph_edges: 20,
            max_decoder_iterations: 100,
            max_parallelism: 4,
            max_wall_time: None,
        }
    }

    #[test]
    fn memory_is_released_by_raii() {
        let manager =
            ResourceManager::new(test_limits())
                .expect("valid limits");

        {
            let reservation =
                manager
                    .reserve_memory(256)
                    .expect("reservation");

            assert_eq!(
                reservation.bytes(),
                256
            );

            assert_eq!(
                manager.snapshot().allocated_bytes,
                256
            );

            assert_eq!(
                manager.snapshot().peak_bytes,
                256
            );
        }

        assert_eq!(
            manager.snapshot().allocated_bytes,
            0
        );

        assert_eq!(
            manager.snapshot().peak_bytes,
            256
        );
    }

    #[test]
    fn memory_limit_is_enforced() {
        let manager =
            ResourceManager::new(test_limits())
                .expect("valid limits");

        let result =
            manager.reserve_memory(1025);

        assert!(
            matches!(
                result,
                Err(
                    ResourceError::LimitExceeded { .. }
                )
            )
        );

        assert_eq!(
            manager.snapshot().allocated_bytes,
            0
        );
    }

    #[test]
    fn graph_and_syndrome_limits_are_enforced() {
        let manager =
            ResourceManager::new(test_limits())
                .expect("valid limits");

        manager
            .record_syndrome_events(10)
            .expect("within limit");

        assert!(
            manager
                .record_syndrome_events(1)
                .is_err()
        );

        manager
            .record_graph_nodes(10)
            .expect("within limit");

        assert!(
            manager
                .record_graph_nodes(1)
                .is_err()
        );

        manager
            .record_graph_edges(20)
            .expect("within limit");

        assert!(
            manager
                .record_graph_edges(1)
                .is_err()
        );
    }

    #[test]
    fn worker_reservation_is_bounded() {
        let manager =
            ResourceManager::new(test_limits())
                .expect("valid limits");

        {
            let workers =
                manager
                    .acquire_workers(2)
                    .expect("workers");

            assert_eq!(
                workers.workers(),
                2
            );

            assert_eq!(
                manager.snapshot().parallel_workers,
                2
            );
        }

        assert_eq!(
            manager.snapshot().parallel_workers,
            0
        );

        assert!(
            manager.acquire_workers(5).is_err()
        );
    }

    #[test]
    fn cancellation_is_observable() {
        let manager =
            ResourceManager::new(test_limits())
                .expect("valid limits");

        manager.cancel();

        assert!(
            matches!(
                manager.check(),
                Err(ResourceError::Cancelled)
            )
        );
    }

    #[test]
    fn operation_quota_is_stricter_than_global_limit() {
        let manager =
            ResourceManager::new(test_limits())
                .expect("valid limits");

        let quota = ResourceQuota {
            max_memory_bytes: Some(128),
            ..ResourceQuota::default()
        };

        let scope =
            manager
                .scope("decoder", quota)
                .expect("scope");

        let first =
            scope
                .reserve_memory(128)
                .expect("quota");

        assert_eq!(
            first.bytes(),
            128
        );

        let second =
            scope.reserve_memory(1);

        assert!(
            matches!(
                second,
                Err(
                    ResourceError::QuotaExceeded { .. }
                )
            )
        );
    }

    #[test]
    fn operation_scope_enforces_wall_time() {
        let manager =
            ResourceManager::new(test_limits())
                .expect("valid limits");

        let quota = ResourceQuota {
            max_wall_time: Some(
                Duration::ZERO
            ),
            ..ResourceQuota::default()
        };

        let scope =
            manager
                .scope("deadline-test", quota)
                .expect("scope");

        thread::yield_now();

        assert!(
            matches!(
                scope.check(),
                Err(
                    ResourceError::WallTimeLimitExceeded {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn concurrent_reservations_respect_global_limit() {
        let manager = shared(
            ResourceLimits {
                max_memory_bytes: 1024,
                ..test_limits()
            },
        )
        .expect("manager");

        let mut handles = Vec::new();

        for _ in 0..16 {
            let manager =
                Arc::clone(&manager);

            handles.push(
                thread::spawn(move || {
                    let reservation =
                        manager.reserve_memory(128);

                    if reservation.is_ok() {
                        thread::yield_now();

                        // Drop the reservation before
                        // returning from the worker.
                        drop(reservation);
                        true
                    } else {
                        false
                    }
                }),
            );
        }

        let mut successful = 0;

        for handle in handles {
            if handle
                .join()
                .expect("worker must not panic")
            {
                successful += 1;
            }
        }

        assert!(
            successful <= 16
        );

        assert!(
            manager.snapshot().peak_bytes
                <= 1024
        );

        assert_eq!(
            manager.snapshot().allocated_bytes,
            0
        );
    }

    #[test]
    fn compute_time_is_explicit() {
        let manager =
            ResourceManager::new(test_limits())
                .expect("valid limits");

        manager
            .record_compute_time(
                Duration::from_millis(5),
            )
            .expect("compute time");

        assert!(
            manager.snapshot().compute_time
                >= Duration::from_millis(5)
        );
    }

    #[test]
    fn decoder_iteration_limit_is_enforced() {
        let manager =
            ResourceManager::new(test_limits())
                .expect("valid limits");

        manager
            .record_decoder_iterations(100)
            .expect("within limit");

        assert!(
            manager
                .record_decoder_iterations(1)
                .is_err()
        );
    }

    #[test]
    fn unlimited_policy_is_valid() {
        assert!(
            ResourceLimits::unlimited()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn zero_limits_are_rejected() {
        let limits = ResourceLimits {
            max_memory_bytes: 0,
            ..test_limits()
        };

        assert!(
            ResourceManager::new(limits)
                .is_err()
        );
    }

    #[test]
    fn snapshot_reports_idle_state() {
        let manager =
            ResourceManager::new(test_limits())
                .expect("valid limits");

        assert!(
            manager.snapshot().is_idle()
        );

        let reservation =
            manager
                .reserve_memory(64)
                .expect("memory");

        assert!(
            !manager.snapshot().is_idle()
        );

        drop(reservation);

        assert!(
            manager.snapshot().is_idle()
        );
    }
}