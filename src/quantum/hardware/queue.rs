//! Zamani Quantum Hardware — Production Queue Model
//!
//! This module defines the provider-neutral queue abstraction used by
//! `quantum::hardware` for quantum jobs/tasks waiting for execution.
//!
//! # Responsibility
//!
//! `queue.rs` owns:
//!
//! - deterministic queue ordering;
//! - queue priorities;
//! - normal/priority/reservation queue classes;
//! - FIFO ordering within an equivalent scheduling key;
//! - bounded queue capacity;
//! - queue positions;
//! - queue depth;
//! - per-class queue depth;
//! - cancellation of queued entries;
//! - dispatch/removal of queued entries;
//! - concurrent access to a queue;
//! - queue snapshots;
//! - queue statistics;
//! - queue configuration and invariants;
//! - provider-neutral queue observations;
//! - deterministic queue identifiers/tickets;
//! - queue-level validation;
//! - retry-safe observation of queue state.
//!
//! It deliberately does NOT own:
//!
//! - quantum job lifecycle semantics;
//! - provider authentication;
//! - provider networking;
//! - provider SDKs;
//! - backend capability discovery;
//! - backend status;
//! - quantum circuit semantics;
//! - transpilation;
//! - routing algorithms;
//! - scheduling algorithms;
//! - execution;
//! - result retrieval;
//! - credentials;
//! - pricing;
//! - calibration;
//! - benchmarking.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Architectural position
//!
//! ```text
//! Zamani Quantum IR
//!        |
//!        v
//! Workload / Execution Request
//!        |
//!        v
//! Backend validation
//!        |
//!        v
//! Job creation
//!        |
//!        v
//! +-----------------------+
//! |       Queue           |
//! |                       |
//! |  priority             |
//! |  reservation          |
//! |  FIFO                 |
//! |  capacity              |
//! |  cancellation          |
//! |  position              |
//! +-----------+-----------+
//!             |
//!             v
//!         dispatch
//!             |
//!             v
//!       provider/backend
//!             |
//!             v
//!          execution
//! ```
//!
//! `queue.rs` is therefore an execution-infrastructure primitive. It does not
//! decide *what* a quantum program means or *how* it should be transpiled.
//!
//! # Provider interoperability
//!
//! The model intentionally supports provider behaviors such as:
//!
//! - normal queues;
//! - priority queues;
//! - reservation-associated work;
//! - queue depth;
//! - queue position;
//! - best-effort cancellation;
//! - queue visibility;
//! - multiple queue classes.
//!
//! Amazon Braket currently exposes normal and priority task queues, queue
//! position/depth, device availability windows, reservations, and
//! best-effort cancellation for queued tasks. IBM Quantum similarly supports
//! cancellation of jobs that are queued or running.
//!
//! Provider-specific semantics MUST be translated into this provider-neutral
//! model by adapters.
//!
//! # Important semantic rule
//!
//! A queue position is an observation, not a guarantee.
//!
//! ```text
//! position == 0
//! ```
//!
//! means that no other currently queued entry is ahead of the entry according
//! to this queue's ordering policy at the instant of observation.
//!
//! It does NOT guarantee immediate execution because:
//!
//! - a provider may have availability windows;
//! - a provider may have another queue not visible to Zamani;
//! - a reservation may not yet be active;
//! - a running job may still occupy the backend;
//! - provider scheduling may change;
//! - the queue may be remote.
//!
//! # Determinism
//!
//! The local queue never uses wall-clock time or randomness for ordering.
//!
//! Ordering is based on:
//!
//! 1. queue class;
//! 2. configured priority;
//! 3. reservation precedence where configured;
//! 4. monotonic enqueue sequence.
//!
//! This makes queue behavior reproducible in tests and local execution.
//!
//! # Concurrency
//!
//! `QuantumQueue` is safe to share between threads using `Arc`.
//!
//! The implementation uses `std::sync::Mutex` internally and never exposes
//! mutable internal collections to callers.
//!
//! Poisoned locks are surfaced as explicit errors rather than silently
//! recovering potentially inconsistent queue state.
//!
//! # Security
//!
//! Queue entries contain identifiers and scheduling metadata only.
//!
//! Queue entries MUST NOT contain:
//!
//! - API keys;
//! - access tokens;
//! - passwords;
//! - private keys;
//! - authorization headers;
//! - provider session cookies;
//! - quantum program secrets.
//!
//! If provider metadata is needed, store a non-secret reference in `metadata`
//! at a higher layer.
//!
//! # Integration contract
//!
//! Future modules consume this file as follows:
//!
//! - `job.rs` owns `JobId` and `JobState` and maps them to `QueueEntry`;
//! - `execution.rs` submits validated work to a queue;
//! - `backend.rs` supplies resource limits such as concurrent-job limits;
//! - `backend_status.rs` determines whether dispatch is allowed;
//! - `provider.rs` exposes provider-side queue observations;
//! - provider adapters translate remote queue semantics into `QueueSnapshot`;
//! - `cancellation.rs` coordinates queue cancellation with provider
//!   cancellation;
//! - `telemetry.rs` consumes queue metrics;
//! - `benchmarking` records queue delay as execution provenance;
//! - `Danga` can expose queue inspection and cancellation through the hardware
//!   API.
//!
//! No future module should require modifications to this file merely because
//! that module was added.
//!
//! # Stability
//!
//! Stable provider-neutral concepts:
//!
//! - `QueueClass`;
//! - `QueuePriority`;
//! - `QueueEntry`;
//! - `QueueEntryState`;
//! - `QueuePosition`;
//! - `QueueConfig`;
//! - `QueueSnapshot`;
//! - `QueueStatistics`;
//! - `QueueError`;
//! - `QuantumQueue`.
//!
//! Provider-specific queue types belong in provider adapters.
//!
//! # Rust compatibility
//!
//! This file targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! ```text
//! #![deny(unsafe_code)]
//! ```
//!
//! # No external dependency
//!
//! The queue deliberately uses only the Rust standard library. This keeps it
//! independently buildable and prevents serialization, provider, or async
//! dependencies from leaking into the foundational queue contract.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;
use std::sync::{Mutex, MutexGuard};

// =============================================================================
// Schema
// =============================================================================

/// Stable queue schema identifier.
pub const QUEUE_SCHEMA_ID: &str = "zamani.quantum.hardware.queue";

/// Semantic version of the queue contract.
///
/// Increment only when the meaning of the public queue contract changes
/// incompatibly.
pub const QUEUE_SCHEMA_VERSION: u16 = 1;

/// Maximum job identifier length.
pub const MAX_JOB_ID_LENGTH: usize = 512;

/// Maximum backend identifier length.
pub const MAX_BACKEND_ID_LENGTH: usize = 512;

/// Maximum reservation identifier length.
pub const MAX_RESERVATION_ID_LENGTH: usize = 512;

/// Maximum queue name length.
pub const MAX_QUEUE_NAME_LENGTH: usize = 256;

/// Maximum metadata key length.
pub const MAX_METADATA_KEY_LENGTH: usize = 256;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum number of metadata entries on one queue item.
pub const MAX_METADATA_PROPERTIES: usize = 128;

/// Maximum number of entries in one queue.
pub const DEFAULT_MAX_QUEUE_DEPTH: usize = 100_000;

/// Maximum number of concurrent running jobs.
///
/// Zero means "not specified by this queue"; backend limits may impose the
/// actual limit.
pub const DEFAULT_MAX_CONCURRENT_JOBS: usize = 0;

// =============================================================================
// Queue class
// =============================================================================

/// Logical class of queued work.
///
/// Provider-specific queue names MUST be translated into these stable
/// semantics by adapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QueueClass {
    /// Ordinary user workload.
    Normal,

    /// Workload explicitly assigned elevated scheduling priority.
    Priority,

    /// Workload associated with an exclusive/dedicated reservation.
    Reservation,
}

impl QueueClass {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Priority => "priority",
            Self::Reservation => "reservation",
        }
    }

    /// Returns whether this class represents reservation-associated work.
    pub const fn is_reservation(self) -> bool {
        matches!(self, Self::Reservation)
    }
}

impl fmt::Display for QueueClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Queue priority
// =============================================================================

/// Provider-neutral bounded queue priority.
///
/// Higher values have higher scheduling priority.
///
/// `QueuePriority::NORMAL` is the default.
///
/// The explicit bounded representation avoids unvalidated signed integers
/// while leaving sufficient space for provider mappings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QueuePriority(u8);

impl QueuePriority {
    /// Default normal priority.
    pub const NORMAL: Self = Self(0);

    /// Elevated priority.
    pub const HIGH: Self = Self(64);

    /// Very high priority.
    pub const URGENT: Self = Self(128);

    /// Maximum representable priority.
    pub const MAX: Self = Self(u8::MAX);

    /// Creates a priority from a raw value.
    pub const fn new(value: u8) -> Self {
        Self(value)
    }

    /// Returns the raw priority.
    pub const fn value(self) -> u8 {
        self.0
    }

    /// Returns true when this is the normal priority.
    pub const fn is_normal(self) -> bool {
        self.0 == Self::NORMAL.0
    }
}

impl Default for QueuePriority {
    fn default() -> Self {
        Self::NORMAL
    }
}

impl fmt::Display for QueuePriority {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// =============================================================================
// Queue entry state
// =============================================================================

/// Lifecycle state owned by the queue for a queued entry.
///
/// The complete quantum job lifecycle belongs to `job.rs`. This state only
/// describes the queue's ownership of the entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QueueEntryState {
    /// Entry has been accepted by the queue and is waiting.
    Queued,

    /// Entry has been selected for dispatch and removed from the waiting set.
    Dispatched,

    /// Entry was cancelled before dispatch.
    Cancelled,
}

impl QueueEntryState {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Dispatched => "dispatched",
            Self::Cancelled => "cancelled",
        }
    }

    /// Returns true if the entry is still waiting.
    pub const fn is_queued(self) -> bool {
        matches!(self, Self::Queued)
    }

    /// Returns true if the entry has left the queue for execution.
    pub const fn is_dispatched(self) -> bool {
        matches!(self, Self::Dispatched)
    }

    /// Returns true if the entry was cancelled in the queue.
    pub const fn is_cancelled(self) -> bool {
        matches!(self, Self::Cancelled)
    }
}

impl fmt::Display for QueueEntryState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Queue entry
// =============================================================================

/// Immutable metadata describing one queued quantum job.
///
/// The queue owns scheduling metadata. The complete quantum job is owned by
/// `job.rs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueEntry {
    /// Provider-neutral job identifier.
    ///
    /// `job.rs::JobId` can be converted to/from this canonical string at the
    /// integration boundary without creating a dependency cycle.
    pub job_id: String,

    /// Backend/device targeted by the job.
    pub backend_id: String,

    /// Queue class.
    pub class: QueueClass,

    /// Scheduling priority.
    pub priority: QueuePriority,

    /// Optional reservation identifier.
    pub reservation_id: Option<String>,

    /// Deterministic monotonic sequence assigned by the queue.
    pub sequence: u64,

    /// Queue-local lifecycle state.
    pub state: QueueEntryState,

    /// Non-secret caller/provider metadata.
    pub metadata: BTreeMap<String, String>,
}

impl QueueEntry {
    /// Creates an entry before queue admission.
    ///
    /// `sequence` is assigned by `QuantumQueue::enqueue`.
    pub fn new(
        job_id: impl Into<String>,
        backend_id: impl Into<String>,
        class: QueueClass,
        priority: QueuePriority,
    ) -> Result<Self, QueueError> {
        let job_id = validate_identifier(
            job_id.into(),
            "job_id",
            MAX_JOB_ID_LENGTH,
        )?;

        let backend_id = validate_identifier(
            backend_id.into(),
            "backend_id",
            MAX_BACKEND_ID_LENGTH,
        )?;

        if class == QueueClass::Reservation {
            return Err(QueueError::ReservationRequired);
        }

        Ok(Self {
            job_id,
            backend_id,
            class,
            priority,
            reservation_id: None,
            sequence: 0,
            state: QueueEntryState::Queued,
            metadata: BTreeMap::new(),
        })
    }

    /// Creates reservation-associated work.
    pub fn reservation(
        job_id: impl Into<String>,
        backend_id: impl Into<String>,
        reservation_id: impl Into<String>,
        priority: QueuePriority,
    ) -> Result<Self, QueueError> {
        let job_id = validate_identifier(
            job_id.into(),
            "job_id",
            MAX_JOB_ID_LENGTH,
        )?;

        let backend_id = validate_identifier(
            backend_id.into(),
            "backend_id",
            MAX_BACKEND_ID_LENGTH,
        )?;

        let reservation_id = validate_identifier(
            reservation_id.into(),
            "reservation_id",
            MAX_RESERVATION_ID_LENGTH,
        )?;

        Ok(Self {
            job_id,
            backend_id,
            class: QueueClass::Reservation,
            priority,
            reservation_id: Some(reservation_id),
            sequence: 0,
            state: QueueEntryState::Queued,
            metadata: BTreeMap::new(),
        })
    }

    /// Adds non-secret metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, QueueError> {
        self.insert_metadata(key, value)?;
        Ok(self)
    }

    /// Adds non-secret metadata in-place.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), QueueError> {
        let key = key.into();
        let value = value.into();

        validate_metadata(&key, &value)?;

        if self.metadata.len() >= MAX_METADATA_PROPERTIES
            && !self.metadata.contains_key(&key)
        {
            return Err(QueueError::MetadataLimitExceeded {
                maximum: MAX_METADATA_PROPERTIES,
            });
        }

        self.metadata.insert(key, value);
        Ok(())
    }

    /// Returns the stable scheduling key.
    ///
    /// Larger class rank/priority sorts first; smaller sequence sorts first.
    fn scheduling_key(&self, reservation_first: bool) -> (u8, u8, u64) {
        let class_rank = match self.class {
            QueueClass::Normal => 0,
            QueueClass::Priority => 1,
            QueueClass::Reservation => {
                if reservation_first {
                    2
                } else {
                    1
                }
            }
        };

        (class_rank, self.priority.value(), self.sequence)
    }
}

// =============================================================================
// Queue configuration
// =============================================================================

/// Queue ordering configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QueueConfig {
    /// Maximum number of waiting entries.
    ///
    /// Zero means unlimited.
    pub max_depth: usize,

    /// Maximum number of concurrently running jobs known to this queue.
    ///
    /// Zero means the queue does not enforce a running-job limit.
    pub max_concurrent_jobs: usize,

    /// Reservation-associated work is placed ahead of normal/priority work.
    pub reservation_first: bool,

    /// Whether cancellation of a queued item is enabled.
    pub cancellation_enabled: bool,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            max_depth: DEFAULT_MAX_QUEUE_DEPTH,
            max_concurrent_jobs: DEFAULT_MAX_CONCURRENT_JOBS,
            reservation_first: true,
            cancellation_enabled: true,
        }
    }
}

impl QueueConfig {
    /// Creates a queue configuration with no local capacity limits.
    pub const fn unlimited() -> Self {
        Self {
            max_depth: 0,
            max_concurrent_jobs: 0,
            reservation_first: true,
            cancellation_enabled: true,
        }
    }

    /// Sets maximum waiting depth.
    pub const fn with_max_depth(mut self, maximum: usize) -> Self {
        self.max_depth = maximum;
        self
    }

    /// Sets maximum running-job count.
    pub const fn with_max_concurrent_jobs(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_concurrent_jobs = maximum;
        self
    }

    /// Controls reservation precedence.
    pub const fn with_reservation_first(
        mut self,
        enabled: bool,
    ) -> Self {
        self.reservation_first = enabled;
        self
    }

    /// Controls queued cancellation.
    pub const fn with_cancellation(
        mut self,
        enabled: bool,
    ) -> Self {
        self.cancellation_enabled = enabled;
        self
    }

    /// Validates configuration invariants.
    pub const fn validate(&self) -> Result<(), QueueError> {
        // Zero means "unspecified/unbounded", therefore every usize value is
        // structurally valid. This method exists as the stable validation
        // boundary for future configuration invariants.
        Ok(())
    }
}

// =============================================================================
// Queue position
// =============================================================================

/// Position of a queued entry.
///
/// `ahead` is zero-based: it is the number of currently queued entries ahead
/// of the target entry.
///
/// `rank` is one-based and exists for user-facing interfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QueuePosition {
    /// Number of currently queued entries ahead of the job.
    pub ahead: usize,

    /// One-based human-facing rank.
    pub rank: usize,
}

impl QueuePosition {
    /// Creates a position from the number of entries ahead.
    pub const fn from_ahead(ahead: usize) -> Self {
        Self {
            ahead,
            rank: ahead.saturating_add(1),
        }
    }

    /// Returns whether this entry is currently first.
    pub const fn is_first(self) -> bool {
        self.ahead == 0
    }
}

// =============================================================================
// Queue depth
// =============================================================================

/// Per-class queue depth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct QueueDepth {
    /// Normal queue entries.
    pub normal: usize,

    /// Priority queue entries.
    pub priority: usize,

    /// Reservation queue entries.
    pub reservation: usize,
}

impl QueueDepth {
    /// Total waiting entries.
    pub const fn total(self) -> usize {
        self.normal + self.priority + self.reservation
    }

    /// Returns the count for one queue class.
    pub const fn for_class(self, class: QueueClass) -> usize {
        match class {
            QueueClass::Normal => self.normal,
            QueueClass::Priority => self.priority,
            QueueClass::Reservation => self.reservation,
        }
    }
}

// =============================================================================
// Queue snapshot
// =============================================================================

/// Immutable point-in-time view of queue state.
///
/// This is the primary type that provider adapters, telemetry and Danga
/// should consume instead of inspecting queue internals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueSnapshot {
    /// Stable queue schema identifier.
    pub schema_id: &'static str,

    /// Queue schema version.
    pub schema_version: u16,

    /// Stable queue name.
    pub name: String,

    /// Target backend identifier, when the queue is backend-specific.
    pub backend_id: Option<String>,

    /// Waiting queue depth.
    pub depth: QueueDepth,

    /// Number of locally running jobs known to this queue.
    pub running_jobs: usize,

    /// Maximum waiting depth, or zero for unlimited.
    pub max_depth: usize,

    /// Maximum running jobs, or zero for unlimited.
    pub max_concurrent_jobs: usize,

    /// Whether the queue accepts new work.
    pub accepting: bool,

    /// Whether queued cancellation is enabled.
    pub cancellation_enabled: bool,

    /// Highest observed sequence number.
    pub sequence: u64,
}

impl QueueSnapshot {
    /// Returns true if no work is waiting.
    pub const fn is_empty(&self) -> bool {
        self.depth.total() == 0
    }

    /// Returns true if the queue is full.
    pub const fn is_full(&self) -> bool {
        self.max_depth != 0 && self.depth.total() >= self.max_depth
    }

    /// Returns remaining local waiting capacity.
    ///
    /// `None` means unlimited.
    pub const fn remaining_capacity(&self) -> Option<usize> {
        if self.max_depth == 0 {
            None
        } else {
            Some(self.max_depth.saturating_sub(self.depth.total()))
        }
    }
}

// =============================================================================
// Queue statistics
// =============================================================================

/// Monotonic queue statistics.
///
/// These values are suitable for telemetry and benchmarking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct QueueStatistics {
    /// Number of accepted entries.
    pub enqueued: u64,

    /// Number of entries dispatched.
    pub dispatched: u64,

    /// Number of entries cancelled while waiting.
    pub cancelled: u64,

    /// Number of rejected submissions due to capacity.
    pub rejected_full: u64,

    /// Number of duplicate job IDs rejected.
    pub rejected_duplicate: u64,

    /// Number of failed queue operations.
    pub operation_errors: u64,
}

impl QueueStatistics {
    /// Returns total completed queue decisions.
    pub const fn completed_decisions(self) -> u64 {
        self.dispatched + self.cancelled
    }
}

// =============================================================================
// Queue error
// =============================================================================

/// Stable provider-neutral queue error.
///
/// The queue deliberately uses a custom error type rather than depending on
/// `anyhow` or `thiserror`, keeping this foundational module independently
/// compilable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueueError {
    /// Queue configuration is invalid.
    InvalidConfiguration {
        /// Human-readable reason.
        reason: &'static str,
    },

    /// Queue is not accepting new entries.
    NotAccepting,

    /// Waiting queue has reached its configured maximum.
    QueueFull {
        /// Configured maximum.
        maximum: usize,
    },

    /// Job ID already exists in the queue.
    DuplicateJobId {
        /// Conflicting identifier.
        job_id: String,
    },

    /// Job ID was not found.
    JobNotFound {
        /// Requested identifier.
        job_id: String,
    },

    /// Operation is invalid for the entry's current queue state.
    InvalidState {
        /// Job identifier.
        job_id: String,

        /// Current state.
        state: QueueEntryState,
    },

    /// Cancellation is disabled.
    CancellationDisabled,

    /// Reservation queue entry lacks a reservation identifier.
    ReservationRequired,

    /// Identifier is invalid.
    InvalidIdentifier {
        /// Field name.
        field: &'static str,

        /// Validation reason.
        reason: &'static str,
    },

    /// Metadata is too large.
    MetadataLimitExceeded {
        /// Maximum allowed properties.
        maximum: usize,
    },

    /// Metadata looks like secret material.
    SecretLikeMetadata {
        /// Rejected metadata key.
        key: String,
    },

    /// A mutex became poisoned.
    LockPoisoned,

    /// Internal sequence counter cannot allocate another ticket.
    SequenceExhausted,
}

impl fmt::Display for QueueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { reason } => {
                write!(formatter, "invalid queue configuration: {reason}")
            }
            Self::NotAccepting => {
                formatter.write_str("queue is not accepting new entries")
            }
            Self::QueueFull { maximum } => {
                write!(formatter, "queue is full (maximum depth {maximum})")
            }
            Self::DuplicateJobId { job_id } => {
                write!(formatter, "job is already queued: {job_id}")
            }
            Self::JobNotFound { job_id } => {
                write!(formatter, "job was not found in queue: {job_id}")
            }
            Self::InvalidState { job_id, state } => {
                write!(
                    formatter,
                    "invalid queue state for job {job_id}: {state}"
                )
            }
            Self::CancellationDisabled => {
                formatter.write_str("queued cancellation is disabled")
            }
            Self::ReservationRequired => {
                formatter.write_str(
                    "reservation queue entries require a reservation identifier",
                )
            }
            Self::InvalidIdentifier { field, reason } => {
                write!(formatter, "invalid {field}: {reason}")
            }
            Self::MetadataLimitExceeded { maximum } => {
                write!(
                    formatter,
                    "queue metadata property limit exceeded: {maximum}"
                )
            }
            Self::SecretLikeMetadata { key } => {
                write!(
                    formatter,
                    "metadata key appears to contain secret material: {key}"
                )
            }
            Self::LockPoisoned => {
                formatter.write_str("queue lock is poisoned")
            }
            Self::SequenceExhausted => {
                formatter.write_str("queue sequence number exhausted")
            }
        }
    }
}

impl std::error::Error for QueueError {}

// =============================================================================
// Internal queue state
// =============================================================================

#[derive(Debug)]
struct QueueState {
    name: String,
    backend_id: Option<String>,
    config: QueueConfig,
    accepting: bool,

    /// All currently waiting entries indexed by job ID.
    entries: BTreeMap<String, QueueEntry>,

    /// Monotonic sequence allocator.
    next_sequence: u64,

    /// Locally running jobs known to this queue.
    running_jobs: BTreeSet<String>,

    /// Monotonic statistics.
    statistics: QueueStatistics,
}

// =============================================================================
// QuantumQueue
// =============================================================================

/// Thread-safe provider-neutral quantum execution queue.
///
/// The queue is local state. Remote provider queues should be represented by
/// adapter-generated `QueueSnapshot` values rather than pretending that a
/// remote queue is controlled by this object.
#[derive(Debug)]
pub struct QuantumQueue {
    state: Mutex<QueueState>,
}

impl QuantumQueue {
    /// Creates a queue with default production configuration.
    pub fn new(name: impl Into<String>) -> Result<Self, QueueError> {
        Self::with_config(name, QueueConfig::default())
    }

    /// Creates a queue with explicit configuration.
    pub fn with_config(
        name: impl Into<String>,
        config: QueueConfig,
    ) -> Result<Self, QueueError> {
        config.validate()?;

        let name = validate_identifier(
            name.into(),
            "queue_name",
            MAX_QUEUE_NAME_LENGTH,
        )?;

        Ok(Self {
            state: Mutex::new(QueueState {
                name,
                backend_id: None,
                config,
                accepting: true,
                entries: BTreeMap::new(),
                next_sequence: 0,
                running_jobs: BTreeSet::new(),
                statistics: QueueStatistics::default(),
            }),
        })
    }

    /// Creates a backend-specific queue.
    pub fn for_backend(
        name: impl Into<String>,
        backend_id: impl Into<String>,
        config: QueueConfig,
    ) -> Result<Self, QueueError> {
        let queue = Self::with_config(name, config)?;
        let backend_id = validate_identifier(
            backend_id.into(),
            "backend_id",
            MAX_BACKEND_ID_LENGTH,
        )?;

        {
            let mut state = queue.lock()?;
            state.backend_id = Some(backend_id);
        }

        Ok(queue)
    }

    /// Returns the queue name.
    pub fn name(&self) -> Result<String, QueueError> {
        Ok(self.lock()?.name.clone())
    }

    /// Returns the configured backend ID, if any.
    pub fn backend_id(&self) -> Result<Option<String>, QueueError> {
        Ok(self.lock()?.backend_id.clone())
    }

    /// Changes whether the queue accepts new work.
    ///
    /// Existing queued work is not modified.
    pub fn set_accepting(&self, accepting: bool) -> Result<(), QueueError> {
        self.lock()?.accepting = accepting;
        Ok(())
    }

    /// Returns whether the queue accepts new work.
    pub fn is_accepting(&self) -> Result<bool, QueueError> {
        Ok(self.lock()?.accepting)
    }

    /// Returns a copy of the queue configuration.
    pub fn config(&self) -> Result<QueueConfig, QueueError> {
        Ok(self.lock()?.config)
    }

    /// Enqueues one job.
    ///
    /// The queue assigns a monotonic sequence number. If the job ID already
    /// exists, no mutation occurs.
    pub fn enqueue(
        &self,
        mut entry: QueueEntry,
    ) -> Result<QueuePosition, QueueError> {
        let mut state = self.lock()?;

        if !state.accepting {
            state.statistics.operation_errors =
                state.statistics.operation_errors.saturating_add(1);

            return Err(QueueError::NotAccepting);
        }

        if state.config.max_depth != 0
            && state.entries.len() >= state.config.max_depth
        {
            state.statistics.rejected_full =
                state.statistics.rejected_full.saturating_add(1);

            return Err(QueueError::QueueFull {
                maximum: state.config.max_depth,
            });
        }

        if state.entries.contains_key(&entry.job_id) {
            state.statistics.rejected_duplicate =
                state.statistics.rejected_duplicate.saturating_add(1);

            return Err(QueueError::DuplicateJobId {
                job_id: entry.job_id,
            });
        }

        if entry.class == QueueClass::Reservation
            && entry.reservation_id.is_none()
        {
            state.statistics.operation_errors =
                state.statistics.operation_errors.saturating_add(1);

            return Err(QueueError::ReservationRequired);
        }

        let sequence = state
            .next_sequence
            .checked_add(1)
            .ok_or(QueueError::SequenceExhausted)?;

        state.next_sequence = sequence;

        entry.sequence = sequence;
        entry.state = QueueEntryState::Queued;

        let job_id = entry.job_id.clone();

        state.entries.insert(job_id.clone(), entry);

        state.statistics.enqueued =
            state.statistics.enqueued.saturating_add(1);

        position_locked(&state, &job_id)
    }

    /// Returns the next entry according to queue ordering without removing it.
    pub fn peek(&self) -> Result<Option<QueueEntry>, QueueError> {
        let state = self.lock()?;
        Ok(next_entry_locked(&state).cloned())
    }

    /// Dispatches the next entry.
    ///
    /// Dispatch removes the entry from the waiting queue and records the job as
    /// running. The caller is responsible for actually submitting the job to
    /// the provider/backend.
    pub fn dispatch_next(&self) -> Result<Option<QueueEntry>, QueueError> {
        let mut state = self.lock()?;

        if state.config.max_concurrent_jobs != 0
            && state.running_jobs.len()
                >= state.config.max_concurrent_jobs
        {
            return Err(QueueError::QueueFull {
                maximum: state.config.max_concurrent_jobs,
            });
        }

        let job_id = match next_job_id_locked(&state) {
            Some(id) => id,
            None => return Ok(None),
        };

        let mut entry = match state.entries.remove(&job_id) {
            Some(entry) => entry,
            None => {
                state.statistics.operation_errors =
                    state.statistics.operation_errors.saturating_add(1);

                return Err(QueueError::JobNotFound { job_id });
            }
        };

        entry.state = QueueEntryState::Dispatched;

        state.running_jobs.insert(entry.job_id.clone());

        state.statistics.dispatched =
            state.statistics.dispatched.saturating_add(1);

        Ok(Some(entry))
    }

    /// Removes a running marker after provider submission/execution finishes.
    ///
    /// This does not alter the job lifecycle. `job.rs` owns the actual job
    /// state.
    pub fn mark_finished(
        &self,
        job_id: &str,
    ) -> Result<bool, QueueError> {
        let mut state = self.lock()?;
        Ok(state.running_jobs.remove(job_id))
    }

    /// Marks a job as running without dispatching a new queue entry.
    ///
    /// This is useful when a remote provider reports that a job entered
    /// execution before the local queue observed the transition.
    pub fn mark_running(
        &self,
        job_id: impl Into<String>,
    ) -> Result<(), QueueError> {
        let job_id = validate_identifier(
            job_id.into(),
            "job_id",
            MAX_JOB_ID_LENGTH,
        )?;

        let mut state = self.lock()?;

        if state.config.max_concurrent_jobs != 0
            && !state.running_jobs.contains(&job_id)
            && state.running_jobs.len()
                >= state.config.max_concurrent_jobs
        {
            return Err(QueueError::QueueFull {
                maximum: state.config.max_concurrent_jobs,
            });
        }

        state.running_jobs.insert(job_id);
        Ok(())
    }

    /// Cancels a queued job.
    ///
    /// A queued cancellation is local queue cancellation. Provider-side
    /// cancellation MUST be performed by `cancellation.rs` / the provider
    /// adapter when the job has already crossed the provider boundary.
    pub fn cancel(&self, job_id: &str) -> Result<QueueEntry, QueueError> {
        let mut state = self.lock()?;

        if !state.config.cancellation_enabled {
            return Err(QueueError::CancellationDisabled);
        }

        let mut entry = state
            .entries
            .remove(job_id)
            .ok_or_else(|| QueueError::JobNotFound {
                job_id: job_id.to_string(),
            })?;

        entry.state = QueueEntryState::Cancelled;

        state.statistics.cancelled =
            state.statistics.cancelled.saturating_add(1);

        Ok(entry)
    }

    /// Returns whether a job is currently waiting.
    pub fn contains(&self, job_id: &str) -> Result<bool, QueueError> {
        Ok(self.lock()?.entries.contains_key(job_id))
    }

    /// Returns a queued entry by ID.
    pub fn get(
        &self,
        job_id: &str,
    ) -> Result<Option<QueueEntry>, QueueError> {
        Ok(self.lock()?.entries.get(job_id).cloned())
    }

    /// Returns the current position of a queued job.
    pub fn position(
        &self,
        job_id: &str,
    ) -> Result<QueuePosition, QueueError> {
        let state = self.lock()?;
        position_locked(&state, job_id)
    }

    /// Returns current queue depth.
    pub fn depth(&self) -> Result<QueueDepth, QueueError> {
        let state = self.lock()?;

        Ok(depth_locked(&state))
    }

    /// Returns current queue length.
    pub fn len(&self) -> Result<usize, QueueError> {
        Ok(self.lock()?.entries.len())
    }

    /// Returns true when no entries are waiting.
    pub fn is_empty(&self) -> Result<bool, QueueError> {
        Ok(self.lock()?.entries.is_empty())
    }

    /// Returns the number of locally running jobs.
    pub fn running_count(&self) -> Result<usize, QueueError> {
        Ok(self.lock()?.running_jobs.len())
    }

    /// Returns whether a job is marked as running.
    pub fn is_running(&self, job_id: &str) -> Result<bool, QueueError> {
        Ok(self.lock()?.running_jobs.contains(job_id))
    }

    /// Returns all currently running job IDs in deterministic order.
    pub fn running_jobs(&self) -> Result<Vec<String>, QueueError> {
        Ok(self
            .lock()?
            .running_jobs
            .iter()
            .cloned()
            .collect())
    }

    /// Returns an immutable snapshot.
    pub fn snapshot(&self) -> Result<QueueSnapshot, QueueError> {
        let state = self.lock()?;

        Ok(QueueSnapshot {
            schema_id: QUEUE_SCHEMA_ID,
            schema_version: QUEUE_SCHEMA_VERSION,
            name: state.name.clone(),
            backend_id: state.backend_id.clone(),
            depth: depth_locked(&state),
            running_jobs: state.running_jobs.len(),
            max_depth: state.config.max_depth,
            max_concurrent_jobs: state.config.max_concurrent_jobs,
            accepting: state.accepting,
            cancellation_enabled: state.config.cancellation_enabled,
            sequence: state.next_sequence,
        })
    }

    /// Returns monotonic queue statistics.
    pub fn statistics(&self) -> Result<QueueStatistics, QueueError> {
        Ok(self.lock()?.statistics)
    }

    /// Removes all queued entries.
    ///
    /// This operation is intentionally explicit and returns every cancelled
    /// entry to the caller so the higher layer can update job state.
    pub fn drain(&self) -> Result<Vec<QueueEntry>, QueueError> {
        let mut state = self.lock()?;

        if !state.config.cancellation_enabled {
            return Err(QueueError::CancellationDisabled);
        }

        let mut entries = Vec::with_capacity(state.entries.len());

        for (_, mut entry) in std::mem::take(&mut state.entries) {
            entry.state = QueueEntryState::Cancelled;
            entries.push(entry);
        }

        entries.sort_by_key(|entry| entry.sequence);

        state.statistics.cancelled = state
            .statistics
            .cancelled
            .saturating_add(entries.len() as u64);

        Ok(entries)
    }

    /// Removes a running marker without changing queue history.
    ///
    /// Returns whether the marker existed.
    pub fn unmark_running(
        &self,
        job_id: &str,
    ) -> Result<bool, QueueError> {
        Ok(self.lock()?.running_jobs.remove(job_id))
    }

    /// Returns all queued entries in dispatch order.
    ///
    /// This is a snapshot and does not mutate the queue.
    pub fn entries(&self) -> Result<Vec<QueueEntry>, QueueError> {
        let state = self.lock()?;

        let mut entries: Vec<QueueEntry> =
            state.entries.values().cloned().collect();

        entries.sort_by(|left, right| {
            left.scheduling_key(state.config.reservation_first)
                .cmp(&right.scheduling_key(state.config.reservation_first))
                .reverse()
        });

        // Sequence is the final FIFO discriminator. The ordering above uses
        // reverse globally, so explicitly normalize the final result below.
        entries.sort_by(|left, right| {
            let left_key = (
                class_rank(left.class, state.config.reservation_first),
                left.priority.value(),
            );

            let right_key = (
                class_rank(right.class, state.config.reservation_first),
                right.priority.value(),
            );

            right_key
                .cmp(&left_key)
                .then_with(|| left.sequence.cmp(&right.sequence))
        });

        Ok(entries)
    }

    /// Returns the next job ID without removing it.
    pub fn next_job_id(&self) -> Result<Option<String>, QueueError> {
        let state = self.lock()?;
        Ok(next_job_id_locked(&state))
    }

    /// Returns the number of jobs that can currently be admitted.
    ///
    /// `None` means unlimited.
    pub fn remaining_capacity(&self) -> Result<Option<usize>, QueueError> {
        let state = self.lock()?;

        if state.config.max_depth == 0 {
            Ok(None)
        } else {
            Ok(Some(
                state
                    .config
                    .max_depth
                    .saturating_sub(state.entries.len()),
            ))
        }
    }

    /// Returns the number of additional jobs that may currently be marked
    /// running.
    ///
    /// `None` means unlimited.
    pub fn remaining_running_capacity(
        &self,
    ) -> Result<Option<usize>, QueueError> {
        let state = self.lock()?;

        if state.config.max_concurrent_jobs == 0 {
            Ok(None)
        } else {
            Ok(Some(
                state
                    .config
                    .max_concurrent_jobs
                    .saturating_sub(state.running_jobs.len()),
            ))
        }
    }

    /// Validates all queue invariants.
    ///
    /// This is intentionally cheap enough for diagnostics and tests and does
    /// not mutate the queue.
    pub fn validate(&self) -> Result<(), QueueError> {
        let state = self.lock()?;

        if state.config.validate().is_err() {
            return Err(QueueError::InvalidConfiguration {
                reason: "configuration invariant failed",
            });
        }

        if state.entries.len() > state.config.max_depth
            && state.config.max_depth != 0
        {
            return Err(QueueError::InvalidConfiguration {
                reason: "queue depth exceeds configured maximum",
            });
        }

        let mut seen_sequences = BTreeSet::new();

        for (job_id, entry) in &state.entries {
            if job_id != &entry.job_id {
                return Err(QueueError::InvalidConfiguration {
                    reason: "queue index does not match entry job ID",
                });
            }

            if !entry.state.is_queued() {
                return Err(QueueError::InvalidConfiguration {
                    reason: "waiting queue contains non-queued entry",
                });
            }

            if !seen_sequences.insert(entry.sequence) {
                return Err(QueueError::InvalidConfiguration {
                    reason: "duplicate queue sequence",
                });
            }

            if entry.class == QueueClass::Reservation
                && entry.reservation_id.is_none()
            {
                return Err(QueueError::InvalidConfiguration {
                    reason: "reservation entry has no reservation ID",
                });
            }
        }

        Ok(())
    }

    /// Locks the internal queue state.
    fn lock(&self) -> Result<MutexGuard<'_, QueueState>, QueueError> {
        self.state.lock().map_err(|_| QueueError::LockPoisoned)
    }
}

// =============================================================================
// Ordering helpers
// =============================================================================

fn class_rank(class: QueueClass, reservation_first: bool) -> u8 {
    match class {
        QueueClass::Normal => 0,
        QueueClass::Priority => 1,
        QueueClass::Reservation => {
            if reservation_first {
                2
            } else {
                1
            }
        }
    }
}

fn next_entry_locked<'a>(
    state: &'a QueueState,
) -> Option<&'a QueueEntry> {
    state.entries.values().min_by(|left, right| {
        let left_class =
            class_rank(left.class, state.config.reservation_first);
        let right_class =
            class_rank(right.class, state.config.reservation_first);

        right_class
            .cmp(&left_class)
            .then_with(|| {
                right
                    .priority
                    .value()
                    .cmp(&left.priority.value())
            })
            .then_with(|| left.sequence.cmp(&right.sequence))
    })
}

fn next_job_id_locked(state: &QueueState) -> Option<String> {
    next_entry_locked(state).map(|entry| entry.job_id.clone())
}

fn position_locked(
    state: &QueueState,
    job_id: &str,
) -> Result<QueuePosition, QueueError> {
    if !state.entries.contains_key(job_id) {
        return Err(QueueError::JobNotFound {
            job_id: job_id.to_string(),
        });
    }

    let target = state
        .entries
        .get(job_id)
        .expect("entry existence checked immediately above");

    let target_class =
        class_rank(target.class, state.config.reservation_first);
    let target_priority = target.priority.value();
    let target_sequence = target.sequence;

    let ahead = state
        .entries
        .values()
        .filter(|entry| {
            if entry.job_id == target.job_id {
                return false;
            }

            let class = class_rank(
                entry.class,
                state.config.reservation_first,
            );

            class > target_class
                || (class == target_class
                    && entry.priority.value() > target_priority)
                || (class == target_class
                    && entry.priority.value() == target_priority
                    && entry.sequence < target_sequence)
        })
        .count();

    Ok(QueuePosition::from_ahead(ahead))
}

fn depth_locked(state: &QueueState) -> QueueDepth {
    let mut depth = QueueDepth::default();

    for entry in state.entries.values() {
        match entry.class {
            QueueClass::Normal => depth.normal += 1,
            QueueClass::Priority => depth.priority += 1,
            QueueClass::Reservation => depth.reservation += 1,
        }
    }

    depth
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_identifier(
    value: String,
    field: &'static str,
    maximum: usize,
) -> Result<String, QueueError> {
    if value.is_empty() {
        return Err(QueueError::InvalidIdentifier {
            field,
            reason: "identifier must not be empty",
        });
    }

    if value.len() > maximum {
        return Err(QueueError::InvalidIdentifier {
            field,
            reason: "identifier exceeds maximum length",
        });
    }

    if value.chars().any(char::is_control) {
        return Err(QueueError::InvalidIdentifier {
            field,
            reason: "identifier must not contain control characters",
        });
    }

    if value.trim() != value {
        return Err(QueueError::InvalidIdentifier {
            field,
            reason: "identifier must not contain leading or trailing whitespace",
        });
    }

    Ok(value)
}

fn validate_metadata(
    key: &str,
    value: &str,
) -> Result<(), QueueError> {
    if key.is_empty() || key.len() > MAX_METADATA_KEY_LENGTH {
        return Err(QueueError::InvalidIdentifier {
            field: "metadata_key",
            reason: "metadata key is empty or too long",
        });
    }

    if value.len() > MAX_METADATA_VALUE_LENGTH {
        return Err(QueueError::InvalidIdentifier {
            field: "metadata_value",
            reason: "metadata value is too long",
        });
    }

    if key.chars().any(char::is_control)
        || value.chars().any(char::is_control)
    {
        return Err(QueueError::InvalidIdentifier {
            field: "metadata",
            reason: "metadata must not contain control characters",
        });
    }

    if looks_like_secret_key(key) {
        return Err(QueueError::SecretLikeMetadata {
            key: key.to_string(),
        });
    }

    Ok(())
}

fn looks_like_secret_key(key: &str) -> bool {
    let normalized = key
        .chars()
        .map(|character| match character {
            '-' | '.' | '/' | ' ' => '_',
            character => character.to_ascii_lowercase(),
        })
        .collect::<String>();

    const FORBIDDEN: &[&str] = &[
        "api_key",
        "apikey",
        "access_token",
        "authorization",
        "auth_header",
        "password",
        "passwd",
        "private_key",
        "secret",
        "session_cookie",
        "cookie",
        "bearer_token",
        "client_secret",
    ];

    FORBIDDEN.iter().any(|name| normalized == *name)
}

// =============================================================================
// Provider queue observation
// =============================================================================

/// Provider-neutral observation of a remote queue.
///
/// This is deliberately separate from `QuantumQueue`.
///
/// `QuantumQueue` controls local queue state.
///
/// `RemoteQueueSnapshot` describes provider-owned queue state.
///
/// This distinction prevents Zamani from pretending that a local queue has
/// authority over a remote provider scheduler.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteQueueSnapshot {
    /// Provider-neutral backend identifier.
    pub backend_id: String,

    /// Provider-reported queue name.
    pub queue_name: String,

    /// Number of entries in the queue.
    pub depth: usize,

    /// Number of priority entries, if known.
    pub priority_depth: Option<usize>,

    /// Number of normal entries, if known.
    pub normal_depth: Option<usize>,

    /// Number of reservation-associated entries, if known.
    pub reservation_depth: Option<usize>,

    /// Position of the inspected job, if requested and known.
    pub position: Option<QueuePosition>,

    /// Whether cancellation is supported by the provider for this queue.
    pub cancellation_supported: bool,

    /// Provider-specific explanatory message.
    ///
    /// This must already be sanitized by the adapter.
    pub message: Option<String>,
}

impl RemoteQueueSnapshot {
    /// Validates remote queue observation.
    pub fn validate(&self) -> Result<(), QueueError> {
        validate_identifier(
            self.backend_id.clone(),
            "backend_id",
            MAX_BACKEND_ID_LENGTH,
        )?;

        validate_identifier(
            self.queue_name.clone(),
            "queue_name",
            MAX_QUEUE_NAME_LENGTH,
        )?;

        if let Some(position) = self.position {
            if position.rank != position.ahead.saturating_add(1) {
                return Err(QueueError::InvalidConfiguration {
                    reason: "remote queue position rank is inconsistent",
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn normal(
        id: &str,
    ) -> QueueEntry {
        QueueEntry::new(
            id,
            "local:qpu",
            QueueClass::Normal,
            QueuePriority::NORMAL,
        )
        .expect("test entry should be valid")
    }

    fn priority(
        id: &str,
        priority: u8,
    ) -> QueueEntry {
        QueueEntry::new(
            id,
            "local:qpu",
            QueueClass::Priority,
            QueuePriority::new(priority),
        )
        .expect("test entry should be valid")
    }

    #[test]
    fn default_configuration_is_production_bounded() {
        let config = QueueConfig::default();

        assert_eq!(config.max_depth, DEFAULT_MAX_QUEUE_DEPTH);
        assert!(config.cancellation_enabled);
        assert!(config.reservation_first);
    }

    #[test]
    fn unlimited_configuration_is_unbounded() {
        let config = QueueConfig::unlimited();

        assert_eq!(config.max_depth, 0);
        assert_eq!(config.max_concurrent_jobs, 0);
    }

    #[test]
    fn queue_preserves_fifo_with_equal_priority() {
        let queue =
            QuantumQueue::new("test").expect("queue should be valid");

        queue.enqueue(normal("job-a"))
            .expect("enqueue should work");

        queue.enqueue(normal("job-b"))
            .expect("enqueue should work");

        queue.enqueue(normal("job-c"))
            .expect("enqueue should work");

        let entries = queue.entries().expect("entries should work");

        assert_eq!(entries[0].job_id, "job-a");
        assert_eq!(entries[1].job_id, "job-b");
        assert_eq!(entries[2].job_id, "job-c");
    }

    #[test]
    fn higher_priority_precedes_lower_priority() {
        let queue =
            QuantumQueue::new("test").expect("queue should be valid");

        queue.enqueue(normal("normal"))
            .expect("enqueue should work");

        queue.enqueue(priority("high", 200))
            .expect("enqueue should work");

        queue.enqueue(priority("low", 50))
            .expect("enqueue should work");

        let first = queue
            .peek()
            .expect("peek should work")
            .expect("entry should exist");

        assert_eq!(first.job_id, "high");
    }

    #[test]
    fn equal_priority_remains_fifo() {
        let queue =
            QuantumQueue::new("test").expect("queue should be valid");

        queue.enqueue(priority("first", 100))
            .expect("enqueue should work");

        queue.enqueue(priority("second", 100))
            .expect("enqueue should work");

        assert_eq!(
            queue
                .next_job_id()
                .expect("next should work")
                .expect("job should exist"),
            "first"
        );
    }

    #[test]
    fn queue_position_is_zero_based_ahead_and_one_based_rank() {
        let queue =
            QuantumQueue::new("test").expect("queue should be valid");

        queue.enqueue(normal("first"))
            .expect("enqueue should work");

        queue.enqueue(normal("second"))
            .expect("enqueue should work");

        queue.enqueue(normal("third"))
            .expect("enqueue should work");

        let position = queue
            .position("third")
            .expect("position should exist");

        assert_eq!(position.ahead, 2);
        assert_eq!(position.rank, 3);
    }

    #[test]
    fn cancellation_removes_only_requested_job() {
        let queue =
            QuantumQueue::new("test").expect("queue should be valid");

        queue.enqueue(normal("first"))
            .expect("enqueue should work");

        queue.enqueue(normal("second"))
            .expect("enqueue should work");

        let cancelled = queue
            .cancel("first")
            .expect("cancellation should work");

        assert_eq!(cancelled.state, QueueEntryState::Cancelled);
        assert!(!queue.contains("first").expect("contains should work"));
        assert!(queue.contains("second").expect("contains should work"));
    }

    #[test]
    fn duplicate_job_ids_are_rejected_without_replacing_existing_entry() {
        let queue =
            QuantumQueue::new("test").expect("queue should be valid");

        queue.enqueue(normal("same"))
            .expect("first enqueue should work");

        let result = queue.enqueue(normal("same"));

        assert!(matches!(
            result,
            Err(QueueError::DuplicateJobId { .. })
        ));

        assert_eq!(
            queue.len().expect("length should work"),
            1
        );
    }

    #[test]
    fn queue_capacity_is_enforced() {
        let config = QueueConfig::unlimited().with_max_depth(1);

        let queue =
            QuantumQueue::with_config("test", config)
                .expect("queue should be valid");

        queue.enqueue(normal("first"))
            .expect("first enqueue should work");

        let result = queue.enqueue(normal("second"));

        assert!(matches!(
            result,
            Err(QueueError::QueueFull { maximum: 1 })
        ));
    }

    #[test]
    fn dispatch_moves_job_to_running_state() {
        let config =
            QueueConfig::unlimited().with_max_concurrent_jobs(1);

        let queue =
            QuantumQueue::with_config("test", config)
                .expect("queue should be valid");

        queue.enqueue(normal("job"))
            .expect("enqueue should work");

        let dispatched = queue
            .dispatch_next()
            .expect("dispatch should work")
            .expect("job should exist");

        assert_eq!(dispatched.state, QueueEntryState::Dispatched);
        assert!(queue.is_running("job").expect("running should work"));
        assert_eq!(queue.len().expect("length should work"), 0);
    }

    #[test]
    fn concurrent_job_limit_is_enforced() {
        let config =
            QueueConfig::unlimited().with_max_concurrent_jobs(1);

        let queue =
            QuantumQueue::with_config("test", config)
                .expect("queue should be valid");

        queue.enqueue(normal("first"))
            .expect("enqueue should work");

        queue.enqueue(normal("second"))
            .expect("enqueue should work");

        queue.dispatch_next()
            .expect("first dispatch should work");

        let result = queue.dispatch_next();

        assert!(matches!(
            result,
            Err(QueueError::QueueFull { maximum: 1 })
        ));
    }

    #[test]
    fn finished_job_frees_running_capacity() {
        let config =
            QueueConfig::unlimited().with_max_concurrent_jobs(1);

        let queue =
            QuantumQueue::with_config("test", config)
                .expect("queue should be valid");

        queue.mark_running("job")
            .expect("mark running should work");

        assert!(queue
            .mark_finished("job")
            .expect("mark finished should work"));

        assert_eq!(
            queue
                .remaining_running_capacity()
                .expect("capacity should work"),
            Some(1)
        );
    }

    #[test]
    fn reservation_entries_require_reservation_id() {
        let result = QueueEntry::new(
            "job",
            "backend",
            QueueClass::Reservation,
            QueuePriority::NORMAL,
        );

        assert!(matches!(
            result,
            Err(QueueError::ReservationRequired)
        ));
    }

    #[test]
    fn reservation_entries_can_be_created_with_reservation() {
        let entry = QueueEntry::reservation(
            "job",
            "backend",
            "reservation-1",
            QueuePriority::NORMAL,
        )
        .expect("reservation entry should be valid");

        assert_eq!(entry.class, QueueClass::Reservation);
        assert_eq!(
            entry.reservation_id.as_deref(),
            Some("reservation-1")
        );
    }

    #[test]
    fn reservation_precedence_is_deterministic() {
        let queue =
            QuantumQueue::new("test").expect("queue should be valid");

        queue.enqueue(normal("normal"))
            .expect("enqueue should work");

        queue.enqueue(
            QueueEntry::reservation(
                "reservation",
                "backend",
                "reservation-1",
                QueuePriority::NORMAL,
            )
            .expect("reservation entry should be valid"),
        )
        .expect("enqueue should work");

        assert_eq!(
            queue
                .next_job_id()
                .expect("next should work")
                .expect("job should exist"),
            "reservation"
        );
    }

    #[test]
    fn reservation_precedence_can_be_disabled() {
        let config =
            QueueConfig::unlimited().with_reservation_first(false);

        let queue =
            QuantumQueue::with_config("test", config)
                .expect("queue should be valid");

        queue.enqueue(normal("normal"))
            .expect("enqueue should work");

        queue.enqueue(
            QueueEntry::reservation(
                "reservation",
                "backend",
                "reservation-1",
                QueuePriority::NORMAL,
            )
            .expect("reservation entry should be valid"),
        )
        .expect("enqueue should work");

        // With reservation precedence disabled, reservation entries share the
        // priority class with priority work. Normal work remains lower.
        assert_eq!(
            queue
                .next_job_id()
                .expect("next should work")
                .expect("job should exist"),
            "reservation"
        );
    }

    #[test]
    fn queue_snapshot_is_consistent() {
        let config = QueueConfig::unlimited().with_max_depth(10);

        let queue =
            QuantumQueue::with_config("test", config)
                .expect("queue should be valid");

        queue.enqueue(normal("one"))
            .expect("enqueue should work");

        queue.enqueue(normal("two"))
            .expect("enqueue should work");

        let snapshot =
            queue.snapshot().expect("snapshot should work");

        assert_eq!(snapshot.schema_id, QUEUE_SCHEMA_ID);
        assert_eq!(
            snapshot.schema_version,
            QUEUE_SCHEMA_VERSION
        );
        assert_eq!(snapshot.depth.total(), 2);
        assert_eq!(snapshot.max_depth, 10);
        assert!(snapshot.accepting);
    }

    #[test]
    fn queue_statistics_are_monotonic() {
        let queue =
            QuantumQueue::new("test").expect("queue should be valid");

        queue.enqueue(normal("one"))
            .expect("enqueue should work");

        queue.enqueue(normal("two"))
            .expect("enqueue should work");

        queue.cancel("one")
            .expect("cancellation should work");

        queue.dispatch_next()
            .expect("dispatch should work");

        let statistics =
            queue.statistics().expect("statistics should work");

        assert_eq!(statistics.enqueued, 2);
        assert_eq!(statistics.cancelled, 1);
        assert_eq!(statistics.dispatched, 1);
    }

    #[test]
    fn drain_returns_cancelled_entries_in_sequence_order() {
        let queue =
            QuantumQueue::new("test").expect("queue should be valid");

        queue.enqueue(normal("one"))
            .expect("enqueue should work");

        queue.enqueue(normal("two"))
            .expect("enqueue should work");

        queue.enqueue(normal("three"))
            .expect("enqueue should work");

        let entries =
            queue.drain().expect("drain should work");

        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].job_id, "one");
        assert_eq!(entries[1].job_id, "two");
        assert_eq!(entries[2].job_id, "three");

        assert!(entries
            .iter()
            .all(|entry| entry.state == QueueEntryState::Cancelled));

        assert!(queue.is_empty().expect("queue should be empty"));
    }

    #[test]
    fn metadata_rejects_secret_like_keys() {
        let result = QueueEntry::new(
            "job",
            "backend",
            QueueClass::Normal,
            QueuePriority::NORMAL,
        )
        .expect("entry should be valid")
        .with_metadata("api_key", "secret");

        assert!(matches!(
            result,
            Err(QueueError::SecretLikeMetadata { .. })
        ));
    }

    #[test]
    fn metadata_accepts_safe_keys() {
        let entry = QueueEntry::new(
            "job",
            "backend",
            QueueClass::Normal,
            QueuePriority::NORMAL,
        )
        .expect("entry should be valid")
        .with_metadata("compiler_version", "1.0")
        .expect("metadata should be valid");

        assert_eq!(
            entry.metadata.get("compiler_version"),
            Some(&"1.0".to_string())
        );
    }

    #[test]
    fn invalid_identifiers_are_rejected() {
        let result = QueueEntry::new(
            "",
            "backend",
            QueueClass::Normal,
            QueuePriority::NORMAL,
        );

        assert!(matches!(
            result,
            Err(QueueError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn remote_snapshot_validates() {
        let snapshot = RemoteQueueSnapshot {
            backend_id: "provider:backend".to_string(),
            queue_name: "quantum_tasks".to_string(),
            depth: 4,
            priority_depth: Some(1),
            normal_depth: Some(3),
            reservation_depth: Some(0),
            position: Some(QueuePosition::from_ahead(2)),
            cancellation_supported: true,
            message: None,
        };

        assert!(snapshot.validate().is_ok());
    }

    #[test]
    fn remote_snapshot_rejects_inconsistent_position() {
        let snapshot = RemoteQueueSnapshot {
            backend_id: "provider:backend".to_string(),
            queue_name: "quantum_tasks".to_string(),
            depth: 4,
            priority_depth: Some(1),
            normal_depth: Some(3),
            reservation_depth: Some(0),
            position: Some(QueuePosition {
                ahead: 2,
                rank: 99,
            }),
            cancellation_supported: true,
            message: None,
        };

        assert!(snapshot.validate().is_err());
    }

    #[test]
    fn queue_invariants_hold_after_normal_operations() {
        let queue =
            QuantumQueue::new("test").expect("queue should be valid");

        queue.enqueue(normal("one"))
            .expect("enqueue should work");

        queue.enqueue(priority("two", 100))
            .expect("enqueue should work");

        queue.cancel("one")
            .expect("cancel should work");

        queue.dispatch_next()
            .expect("dispatch should work");

        assert!(queue.validate().is_ok());
    }

    #[test]
    fn queue_is_thread_safe_for_independent_operations() {
        use std::sync::Arc;
        use std::thread;

        let queue = Arc::new(
            QuantumQueue::new("concurrent")
                .expect("queue should be valid"),
        );

        let mut handles = Vec::new();

        for index in 0..8 {
            let queue = Arc::clone(&queue);

            handles.push(thread::spawn(move || {
                let id = format!("job-{index}");

                queue
                    .enqueue(normal(&id))
                    .expect("enqueue should work");
            }));
        }

        for handle in handles {
            handle.join().expect("worker should finish");
        }

        assert_eq!(
            queue.len().expect("length should work"),
            8
        );

        assert!(queue.validate().is_ok());
    }

    #[test]
    fn no_randomness_or_wall_clock_is_required_for_ordering() {
        let queue =
            QuantumQueue::new("deterministic")
                .expect("queue should be valid");

        for id in ["a", "b", "c", "d"] {
            queue.enqueue(normal(id))
                .expect("enqueue should work");
        }

        let first = queue
            .entries()
            .expect("entries should work")
            .into_iter()
            .map(|entry| entry.job_id)
            .collect::<Vec<_>>();

        let second = queue
            .entries()
            .expect("entries should work")
            .into_iter()
            .map(|entry| entry.job_id)
            .collect::<Vec<_>>();

        assert_eq!(first, second);
    }
}