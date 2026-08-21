//! Zamani Quantum Error Correction — bounded deterministic syndrome streaming.
//!
//! # Architectural contract
//!
//! `streaming.rs` owns the execution boundary for bounded syndrome streams.
//!
//! It owns:
//!
//! - bounded input buffering;
//! - explicit backpressure;
//! - lossless/lossy admission policy;
//! - deterministic stream ordering;
//! - stream lifecycle;
//! - bounded processed-history retention;
//! - incremental processing through `SyndromeProcessor`;
//! - cancellation propagation;
//! - stream-local metrics;
//! - in-memory stream snapshots;
//! - `SyndromeSource` integration.
//!
//! It does NOT own:
//!
//! - stabilizer mathematics (`stabilizer.rs`);
//! - syndrome mathematics (`syndrome.rs`);
//! - decoding (`decoder.rs`, `mwpm.rs`, `union_find.rs`);
//! - decoding graph construction (`decoding_graph.rs`);
//! - partitioning (`partition.rs`);
//! - distributed transport (`distributed.rs`);
//! - scheduling (`scheduler.rs`);
//! - durable checkpoint serialization (`checkpoint.rs`);
//! - QPU transport (`qpu_adapter.rs`);
//! - telemetry transport (`telemetry.rs`).
//!
//! # Integration
//!
//! ```text
//!                 QPU / simulator / replay / source
//!                              │
//!                              ▼
//!                    ┌──────────────────┐
//!                    │ Syndrome         │
//!                    │ validation       │
//!                    └────────┬─────────┘
//!                             │
//!                             ▼
//!                    ┌──────────────────┐
//!                    │ SyndromeStream   │
//!                    │                  │
//!                    │ bounded buffer   │
//!                    │ ordering        │
//!                    │ backpressure    │
//!                    │ cancellation    │
//!                    └────────┬─────────┘
//!                             │
//!                             ▼
//!                    SyndromeProcessor
//!                             │
//!                             ▼
//!                    DetectionEvent batch
//!                             │
//!                ┌────────────┼────────────┐
//!                ▼            ▼            ▼
//!             decoder       graph       partition
//!
//! ```
//!
//! # Resource model
//!
//! `limits.rs` is the sole declarative production resource policy.
//!
//! ```text
//! QecLimits
//!     │
//!     ├── buffer capacity
//!     ├── syndrome count
//!     ├── measurement rounds
//!     ├── detection events
//!     └── memory admission
//!
//! streaming.rs
//!     │
//!     └── bounded execution
//!
//! resources.rs
//!     │
//!     └── runtime accounting
//!
//! memory.rs
//!     │
//!     └── allocation enforcement
//! ```
//!
//! This module deliberately does not introduce another `ResourceLimits`
//! structure.
//!
//! # Losslessness
//!
//! Mathematical QEC decoding is lossless by default.
//!
//! `BackpressurePolicy::Reject` is therefore the canonical production mode.
//! `DropNewest` and `DropOldest` require explicit opt-in through
//! `allow_lossy(true)`.
//!
//! If any item is dropped, the stream is no longer suitable for claiming an
//! exact decoding result from the complete submitted syndrome history.
//!
//! # Determinism
//!
//! The stream guarantees:
//!
//! - FIFO submission order;
//! - monotonically increasing stream sequence numbers;
//! - consecutive syndrome-round validation;
//! - deterministic processing order;
//! - deterministic history ordering;
//! - no sequence-number wrapping;
//! - no hidden concurrent mutation.
//!
//! Scheduling and distributed execution are deliberately outside this module.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1.
//!
//! No nightly-only language features are used.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::VecDeque;
use std::fmt;

use super::cancellation::CancellationToken;
use super::errors::{QecError, QecResult, ResourceKind};
use super::limits::QecLimits;
use super::syndrome::{
    DetectionEvent,
    MeasurementRound,
    Syndrome,
    SyndromeProcessor,
    SyndromeSource,
};

// ============================================================================
// Constants
// ============================================================================

/// Current in-memory stream-state schema.
pub const STREAM_STATE_VERSION: u16 = 3;

/// Conservative default bounded input capacity.
///
/// This is an implementation default, not a second resource policy.
/// `QecLimits::max_stream_buffer_events` can reduce it.
pub const DEFAULT_MAX_BUFFERED_SYNDROMES: usize = 1_024;

/// Maximum outputs returned by one `poll()` call.
pub const DEFAULT_MAX_POLL_ROUNDS: usize = 1_024;

/// `u64::MAX` is reserved so stream sequence numbers never wrap into a
/// potentially valid sequence value.
pub const MAX_STREAM_SEQUENCE: u64 = u64::MAX - 1;

/// Minimum history required for consecutive-round detection.
pub const MIN_HISTORY_ROUNDS: usize = 1;

// ============================================================================
// Backpressure
// ============================================================================

/// Policy applied when the bounded input buffer is full.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackpressurePolicy {
    /// Reject the incoming syndrome.
    ///
    /// This is the only lossless policy and is the production default.
    Reject,

    /// Reject the incoming syndrome and retain existing buffered data.
    ///
    /// This is explicitly lossy because the submitted syndrome is not
    /// processed.
    DropNewest,

    /// Remove the oldest buffered syndrome and accept the new syndrome.
    ///
    /// This is explicitly lossy because an already accepted syndrome is
    /// discarded before processing.
    DropOldest,
}

impl BackpressurePolicy {
    /// Returns `true` when every submitted syndrome is preserved.
    #[must_use]
    pub const fn is_lossless(self) -> bool {
        matches!(self, Self::Reject)
    }

    /// Returns `true` when this policy can discard input.
    #[must_use]
    pub const fn is_lossy(self) -> bool {
        !self.is_lossless()
    }
}

// ============================================================================
// History mode
// ============================================================================

/// Controls how much processed syndrome history is retained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamMode {
    /// Retain only the previous syndrome required for incremental decoding.
    Minimal,

    /// Retain a bounded number of processed syndrome rounds.
    Windowed {
        /// Number of rounds retained.
        rounds: usize,
    },
}

impl StreamMode {
    /// Returns the effective history capacity.
    #[must_use]
    pub const fn history_capacity(self) -> usize {
        match self {
            Self::Minimal => MIN_HISTORY_ROUNDS,
            Self::Windowed { rounds } if rounds < MIN_HISTORY_ROUNDS => {
                MIN_HISTORY_ROUNDS
            }
            Self::Windowed { rounds } => rounds,
        }
    }

    /// Validates the mode.
    pub fn validate(self) -> Result<(), StreamingError> {
        match self {
            Self::Minimal => Ok(()),

            Self::Windowed { rounds } if rounds < MIN_HISTORY_ROUNDS => {
                Err(StreamingError::InvalidHistorySize {
                    requested: rounds,
                })
            }

            Self::Windowed { .. } => Ok(()),
        }
    }
}

// ============================================================================
// Lifecycle
// ============================================================================

/// Lifecycle of a syndrome stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamStatus {
    /// Accepting new syndrome input.
    Open,

    /// Input is closed; buffered work is still being drained.
    Closing,

    /// Input is closed and all buffered work has completed.
    Closed,

    /// Cancellation was observed.
    Cancelled,

    /// A terminal processing failure occurred.
    Failed,
}

impl StreamStatus {
    /// Returns whether new input can be submitted.
    #[must_use]
    pub const fn accepts_input(self) -> bool {
        matches!(self, Self::Open)
    }

    /// Returns whether processing can continue.
    #[must_use]
    pub const fn can_process(self) -> bool {
        matches!(self, Self::Open | Self::Closing)
    }

    /// Returns whether the lifecycle is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Closed | Self::Cancelled | Self::Failed
        )
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Streaming-specific errors.
///
/// These are converted into the repository-wide `QecError` boundary at the
/// public API boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamingError {
    /// Input was submitted after the stream stopped accepting input.
    NotAcceptingInput,

    /// The stream is already terminal.
    TerminalState,

    /// A syndrome round was not consecutive with the accepted predecessor.
    OutOfOrderRound {
        /// Expected round.
        expected: u64,

        /// Received round.
        received: u64,
    },

    /// A stream sequence number would overflow.
    SequenceOverflow,

    /// The configured input buffer is full.
    BufferFull {
        /// Buffer capacity.
        capacity: usize,
    },

    /// The configured history size is invalid.
    InvalidHistorySize {
        /// Requested history size.
        requested: usize,
    },

    /// A polling request is invalid.
    InvalidPollSize {
        /// Requested number of rounds.
        requested: usize,
    },

    /// A lossy backpressure policy was selected without authorization.
    LossyBackpressureNotAllowed,

    /// A stream snapshot has an unsupported schema.
    UnsupportedStateVersion {
        /// Found schema.
        version: u16,
    },

    /// A stream snapshot is internally inconsistent.
    InvalidState {
        /// Human-readable diagnostic.
        message: String,
    },
}

impl fmt::Display for StreamingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAcceptingInput => {
                formatter.write_str(
                    "syndrome stream is not accepting input",
                )
            }

            Self::TerminalState => {
                formatter.write_str(
                    "syndrome stream is in a terminal state",
                )
            }

            Self::OutOfOrderRound {
                expected,
                received,
            } => {
                write!(
                    formatter,
                    "out-of-order syndrome round: \
                     expected {expected}, received {received}"
                )
            }

            Self::SequenceOverflow => {
                formatter.write_str(
                    "syndrome stream sequence number overflow",
                )
            }

            Self::BufferFull { capacity } => {
                write!(
                    formatter,
                    "syndrome stream input buffer is full \
                     (capacity {capacity})"
                )
            }

            Self::InvalidHistorySize { requested } => {
                write!(
                    formatter,
                    "invalid stream history size {requested}; \
                     minimum is {MIN_HISTORY_ROUNDS}"
                )
            }

            Self::InvalidPollSize { requested } => {
                write!(
                    formatter,
                    "invalid stream poll size {requested}; minimum is 1"
                )
            }

            Self::LossyBackpressureNotAllowed => {
                formatter.write_str(
                    "lossy backpressure requires explicit authorization",
                )
            }

            Self::UnsupportedStateVersion { version } => {
                write!(
                    formatter,
                    "unsupported streaming state version {version}"
                )
            }

            Self::InvalidState { message } => {
                write!(formatter, "invalid streaming state: {message}")
            }
        }
    }
}

impl std::error::Error for StreamingError {}

impl From<StreamingError> for QecError {
    fn from(error: StreamingError) -> Self {
        match &error {
            StreamingError::BufferFull { capacity } => {
                QecError::resource_limit(
                    ResourceKind::StreamBuffer,
                    *capacity as u128 + 1,
                    *capacity as u128,
                    *capacity as u128,
                    error.to_string(),
                )
            }

            StreamingError::SequenceOverflow => {
                QecError::invalid_input(error.to_string())
            }

            StreamingError::OutOfOrderRound { .. }
            | StreamingError::InvalidHistorySize { .. }
            | StreamingError::InvalidPollSize { .. }
            | StreamingError::LossyBackpressureNotAllowed
            | StreamingError::UnsupportedStateVersion { .. }
            | StreamingError::InvalidState { .. }
            | StreamingError::NotAcceptingInput
            | StreamingError::TerminalState => {
                QecError::invalid_input(error.to_string())
            }
        }
    }
}

// ============================================================================
// Metrics
// ============================================================================

/// Stream-local deterministic counters.
///
/// These are execution metrics, not telemetry. `telemetry.rs` remains
/// responsible for deciding whether metrics may leave the process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StreamMetrics {
    /// Number of accepted syndrome submissions.
    pub syndromes_accepted: u64,

    /// Number of rejected submissions.
    pub syndromes_rejected: u64,

    /// Number of generated detection events.
    pub detection_events_generated: u64,

    /// Number of events delivered to the consumer.
    pub detection_events_delivered: u64,

    /// Current buffered syndrome count.
    pub buffered_syndromes: u64,

    /// Maximum observed buffer occupancy.
    pub peak_buffered_syndromes: u64,

    /// Number of cancellation polling operations.
    pub cancellation_checks: u64,

    /// Number of full-buffer incidents.
    pub backpressure_events: u64,

    /// Number of successfully processed rounds.
    pub rounds_processed: u64,

    /// Number of polling operations.
    pub polls: u64,

    /// Number of flush operations.
    pub flushes: u64,

    /// Number of intentionally discarded syndromes.
    pub syndromes_dropped: u64,
}

impl StreamMetrics {
    /// Returns whether this stream has remained lossless.
    #[must_use]
    pub const fn is_lossless(self) -> bool {
        self.syndromes_dropped == 0
    }
}

// ============================================================================
// Stream item
// ============================================================================

/// One accepted syndrome together with its deterministic stream sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamItem {
    sequence: u64,
    syndrome: Syndrome,
}

impl StreamItem {
    /// Creates a stream item.
    fn new(sequence: u64, syndrome: Syndrome) -> Self {
        Self {
            sequence,
            syndrome,
        }
    }

    /// Returns the stream sequence.
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the syndrome.
    #[must_use]
    pub const fn syndrome(&self) -> &Syndrome {
        &self.syndrome
    }

    /// Consumes the item and returns its syndrome.
    #[must_use]
    pub fn into_syndrome(self) -> Syndrome {
        self.syndrome
    }
}

// ============================================================================
// Stream output
// ============================================================================

/// Result produced after processing one accepted syndrome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamOutput {
    /// Stream-local sequence number.
    pub sequence: u64,

    /// Measurement round represented by this output.
    pub round: MeasurementRound,

    /// Detection events generated from the previous round.
    pub events: Vec<DetectionEvent>,
}

impl StreamOutput {
    /// Returns `true` when no detection event was generated.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Returns the number of detection events.
    #[must_use]
    pub fn len(&self) -> usize {
        self.events.len()
    }
}

// ============================================================================
// Snapshot
// ============================================================================

/// In-memory snapshot of stream execution state.
///
/// This is intentionally **not** the durable checkpoint format. `checkpoint.rs`
/// owns serialization, integrity protection, schema compatibility and durable
/// storage.
///
/// A checkpoint implementation can consume this structure rather than
/// reimplementing the streaming state machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamSnapshot {
    /// Snapshot schema.
    pub version: u16,

    /// Lifecycle state.
    pub status: StreamStatus,

    /// Next stream sequence number.
    pub next_sequence: u64,

    /// Stream mode.
    pub mode: StreamMode,

    /// Backpressure policy.
    pub backpressure: BackpressurePolicy,

    /// Whether lossy processing was explicitly authorized.
    pub lossy_allowed: bool,

    /// Input buffer capacity.
    pub buffer_capacity: usize,

    /// Buffered input in FIFO order.
    pub buffered_input: Vec<StreamItem>,

    /// Retained processed syndrome history in chronological order.
    pub history: Vec<Syndrome>,

    /// Stream metrics.
    pub metrics: StreamMetrics,
}

// ============================================================================
// Syndrome stream
// ============================================================================

/// Bounded deterministic syndrome execution stream.
///
/// The type is intentionally synchronous. It does not create threads, own an
/// async runtime, communicate over a network, or submit QPU work.
pub struct SyndromeStream {
    limits: QecLimits,
    cancellation: CancellationToken,

    mode: StreamMode,
    backpressure: BackpressurePolicy,
    lossy_allowed: bool,

    buffer_capacity: usize,
    buffer: VecDeque<StreamItem>,

    history: VecDeque<Syndrome>,

    processor: SyndromeProcessor,

    next_sequence: u64,
    status: StreamStatus,

    metrics: StreamMetrics,
}

impl fmt::Debug for SyndromeStream {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SyndromeStream")
            .field("limits", &self.limits)
            .field("mode", &self.mode)
            .field("backpressure", &self.backpressure)
            .field("lossy_allowed", &self.lossy_allowed)
            .field("buffer_capacity", &self.buffer_capacity)
            .field("buffered_syndromes", &self.buffer.len())
            .field("history_len", &self.history.len())
            .field("next_sequence", &self.next_sequence)
            .field("status", &self.status)
            .field("metrics", &self.metrics)
            .finish()
    }
}

impl SyndromeStream {
    // ------------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------------

    /// Creates a stream using the canonical QEC limits and cancellation token.
    pub fn new() -> QecResult<Self> {
        Self::with_limits_and_cancellation(
            QecLimits::default(),
            CancellationToken::new(),
        )
    }

    /// Creates a stream with explicit QEC limits.
    pub fn with_limits(limits: QecLimits) -> QecResult<Self> {
        Self::with_limits_and_cancellation(
            limits,
            CancellationToken::new(),
        )
    }

    /// Creates a stream with explicit limits and cancellation.
    pub fn with_limits_and_cancellation(
        limits: QecLimits,
        cancellation: CancellationToken,
    ) -> QecResult<Self> {
        limits.validate().map_err(|error| {
            QecError::invalid_input(format!(
                "invalid QEC streaming limits: {error}"
            ))
        })?;

        let capacity = DEFAULT_MAX_BUFFERED_SYNDROMES
            .min(limits.max_stream_buffer_events)
            .min(limits.max_syndrome_events);

        if capacity == 0 {
            return Err(QecError::resource_limit(
                ResourceKind::StreamBuffer,
                1,
                0,
                limits.max_stream_buffer_events as u128,
                "stream buffer capacity must be greater than zero",
            ));
        }

        let processor =
            SyndromeProcessor::with_limits(
                limits,
                cancellation.clone(),
            )?;

        Ok(Self {
            limits,
            cancellation,

            mode: StreamMode::Minimal,
            backpressure: BackpressurePolicy::Reject,
            lossy_allowed: false,

            buffer_capacity: capacity,
            buffer: VecDeque::with_capacity(capacity),

            history: VecDeque::with_capacity(
                MIN_HISTORY_ROUNDS,
            ),

            processor,

            next_sequence: 0,
            status: StreamStatus::Open,

            metrics: StreamMetrics::default(),
        })
    }

    // ------------------------------------------------------------------------
    // Configuration
    // ------------------------------------------------------------------------

    /// Changes the retained-history mode.
    ///
    /// The requested history is checked against the canonical stream-buffer
    /// resource limit before any allocation is attempted.
    pub fn set_mode(
        &mut self,
        mode: StreamMode,
    ) -> QecResult<()> {
        self.check_cancelled()?;

        mode.validate().map_err(QecError::from)?;

        let requested = mode.history_capacity();

        if requested > self.limits.max_stream_buffer_events {
            return Err(QecError::resource_limit(
                ResourceKind::StreamBuffer,
                requested as u128,
                self.history.len() as u128,
                self.limits.max_stream_buffer_events as u128,
                format!(
                    "requested history capacity {requested} exceeds \
                     configured stream-buffer limit {}",
                    self.limits.max_stream_buffer_events
                ),
            ));
        }

        self.mode = mode;

        while self.history.len() > requested {
            self.history.pop_front();
        }

        Ok(())
    }

    /// Returns the current stream mode.
    #[must_use]
    pub const fn mode(&self) -> StreamMode {
        self.mode
    }

    /// Selects the backpressure policy.
    ///
    /// Lossy policies cannot be selected until `allow_lossy(true)` has been
    /// explicitly called.
    pub fn set_backpressure(
        &mut self,
        policy: BackpressurePolicy,
    ) -> QecResult<()> {
        self.check_cancelled()?;

        if policy.is_lossy() && !self.lossy_allowed {
            return Err(
                StreamingError::LossyBackpressureNotAllowed.into()
            );
        }

        self.backpressure = policy;

        Ok(())
    }

    /// Explicitly authorizes or revokes lossy processing.
    ///
    /// Revoking authorization immediately restores lossless rejection mode.
    pub fn allow_lossy(
        &mut self,
        allowed: bool,
    ) -> QecResult<()> {
        self.check_cancelled()?;

        self.lossy_allowed = allowed;

        if !allowed && self.backpressure.is_lossy() {
            self.backpressure = BackpressurePolicy::Reject;
        }

        Ok(())
    }

    /// Returns whether lossy processing is explicitly authorized.
    #[must_use]
    pub const fn lossy_allowed(&self) -> bool {
        self.lossy_allowed
    }

    /// Returns the active backpressure policy.
    #[must_use]
    pub const fn backpressure(&self) -> BackpressurePolicy {
        self.backpressure
    }

    // ------------------------------------------------------------------------
    // State
    // ------------------------------------------------------------------------

    /// Returns the current lifecycle status.
    #[must_use]
    pub const fn status(&self) -> StreamStatus {
        self.status
    }

    /// Returns whether the stream is terminal.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        self.status.is_terminal()
    }

    /// Returns the number of currently buffered syndromes.
    #[must_use]
    pub fn buffered_len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns whether the input buffer is empty.
    #[must_use]
    pub fn is_buffer_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Returns the admitted buffer capacity.
    #[must_use]
    pub const fn buffer_capacity(&self) -> usize {
        self.buffer_capacity
    }

    /// Returns the next sequence number that will be assigned.
    #[must_use]
    pub const fn next_sequence(&self) -> u64 {
        self.next_sequence
    }

    /// Returns stream metrics.
    #[must_use]
    pub const fn metrics(&self) -> StreamMetrics {
        self.metrics
    }

    /// Returns the configured QEC limits.
    #[must_use]
    pub const fn limits(&self) -> QecLimits {
        self.limits
    }

    /// Returns the retained history.
    pub fn history(&self) -> impl Iterator<Item = &Syndrome> {
        self.history.iter()
    }

    /// Returns the current processor baseline.
    #[must_use]
    pub fn previous_syndrome(&self) -> Option<&Syndrome> {
        self.processor.previous()
    }

    // ------------------------------------------------------------------------
    // Cancellation
    // ------------------------------------------------------------------------

    /// Returns the stream cancellation token.
    ///
    /// Callers should normally retain the corresponding `CancellationSource`
    /// externally when they need to request cancellation.
    #[must_use]
    pub fn cancellation_token(&self) -> CancellationToken {
        self.cancellation.clone()
    }

    fn check_cancelled(&mut self) -> QecResult<()> {
        self.metrics.cancellation_checks = self
            .metrics
            .cancellation_checks
            .checked_add(1)
            .ok_or_else(|| {
                QecError::invalid_input(
                    "stream cancellation-check counter overflow",
                )
            })?;

        match self.cancellation.check() {
            Ok(()) => Ok(()),

            Err(error) => {
                self.status = StreamStatus::Cancelled;
                Err(error)
            }
        }
    }

    // ------------------------------------------------------------------------
    // Admission
    // ------------------------------------------------------------------------

    /// Validates the next sequence number without modifying stream state.
    fn next_sequence_value(&self) -> Result<u64, StreamingError> {
        if self.next_sequence > MAX_STREAM_SEQUENCE {
            return Err(StreamingError::SequenceOverflow);
        }

        Ok(self.next_sequence)
    }

    /// Validates syndrome admission without modifying the queue.
    fn validate_submission(
        &mut self,
        syndrome: &Syndrome,
    ) -> QecResult<()> {
        syndrome.preflight()?;

        let expected_round = if let Some(last) =
            self.buffer.back().map(StreamItem::syndrome)
        {
            last.round().next().map_err(QecError::from)?
        } else if let Some(last) = self.history.back() {
            last.round().next().map_err(QecError::from)?
        } else if let Some(previous) =
            self.processor.previous()
        {
            previous.round().next().map_err(QecError::from)?
        } else {
            // First syndrome establishes the baseline.
            return Ok(());
        };

        if syndrome.round() != expected_round {
            return Err(
                StreamingError::OutOfOrderRound {
                    expected: expected_round.value(),
                    received: syndrome.round().value(),
                }
                .into(),
            );
        }

        Ok(())
    }

    /// Submits one syndrome into the bounded stream.
    ///
    /// In lossless mode, a full buffer rejects the submission without
    /// modifying existing buffered data.
    pub fn submit(
        &mut self,
        syndrome: Syndrome,
    ) -> QecResult<u64> {
        self.check_cancelled()?;

        if !self.status.accepts_input() {
            self.metrics.syndromes_rejected = self
                .metrics
                .syndromes_rejected
                .saturating_add(1);

            return Err(
                StreamingError::NotAcceptingInput.into()
            );
        }

        self.validate_submission(&syndrome)?;

        let sequence =
            self.next_sequence_value().map_err(QecError::from)?;

        if self.buffer.len() >= self.buffer_capacity {
            self.metrics.backpressure_events = self
                .metrics
                .backpressure_events
                .saturating_add(1);

            match self.backpressure {
                BackpressurePolicy::Reject => {
                    self.metrics.syndromes_rejected = self
                        .metrics
                        .syndromes_rejected
                        .saturating_add(1);

                    return Err(
                        StreamingError::BufferFull {
                            capacity: self.buffer_capacity,
                        }
                        .into(),
                    );
                }

                BackpressurePolicy::DropNewest => {
                    if !self.lossy_allowed {
                        return Err(
                            StreamingError::LossyBackpressureNotAllowed
                                .into()
                        );
                    }

                    self.metrics.syndromes_dropped = self
                        .metrics
                        .syndromes_dropped
                        .checked_add(1)
                        .ok_or_else(|| {
                            QecError::invalid_input(
                                "stream dropped-syndrome counter overflow",
                            )
                        })?;

                    self.metrics.syndromes_rejected = self
                        .metrics
                        .syndromes_rejected
                        .saturating_add(1);

                    return Ok(sequence);
                }

                BackpressurePolicy::DropOldest => {
                    if !self.lossy_allowed {
                        return Err(
                            StreamingError::LossyBackpressureNotAllowed
                                .into()
                        );
                    }

                    let removed = self.buffer.pop_front();

                    if removed.is_none() {
                        return Err(
                            StreamingError::InvalidState {
                                message:
                                    "drop-oldest policy observed \
                                     an empty full buffer"
                                        .to_owned(),
                            }
                            .into(),
                        );
                    }

                    self.metrics.syndromes_dropped = self
                        .metrics
                        .syndromes_dropped
                        .checked_add(1)
                        .ok_or_else(|| {
                            QecError::invalid_input(
                                "stream dropped-syndrome counter overflow",
                            )
                        })?;
                }
            }
        }

        self.buffer.push_back(
            StreamItem::new(sequence, syndrome)
        );

        self.next_sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or_else(|| {
                QecError::invalid_input(
                    StreamingError::SequenceOverflow.to_string(),
                )
            })?;

        self.metrics.syndromes_accepted = self
            .metrics
            .syndromes_accepted
            .checked_add(1)
            .ok_or_else(|| {
                QecError::invalid_input(
                    "stream accepted-syndrome counter overflow",
                )
            })?;

        self.metrics.buffered_syndromes =
            self.buffer.len() as u64;

        let occupancy =
            self.buffer.len() as u64;

        if occupancy > self.metrics.peak_buffered_syndromes {
            self.metrics.peak_buffered_syndromes =
                occupancy;
        }

        Ok(sequence)
    }

    // ------------------------------------------------------------------------
    // Processing
    // ------------------------------------------------------------------------

    /// Processes one buffered syndrome.
    pub fn process_one(
        &mut self,
    ) -> QecResult<Option<StreamOutput>> {
        self.check_cancelled()?;

        if !self.status.can_process() {
            return Ok(None);
        }

        let Some(item) = self.buffer.pop_front() else {
            if self.status == StreamStatus::Closing {
                self.status = StreamStatus::Closed;
            }

            self.metrics.buffered_syndromes =
                self.buffer.len() as u64;

            return Ok(None);
        };

        self.metrics.buffered_syndromes =
            self.buffer.len() as u64;

        let sequence = item.sequence();
        let syndrome = item.into_syndrome();

        let round = syndrome.round();

        let events = match self.processor.push(syndrome.clone()) {
            Ok(events) => events,

            Err(error) => {
                self.status = StreamStatus::Failed;
                return Err(error);
            }
        };

        self.retain_history(syndrome)?;

        self.metrics.rounds_processed = self
            .metrics
            .rounds_processed
            .checked_add(1)
            .ok_or_else(|| {
                self.status = StreamStatus::Failed;

                QecError::invalid_input(
                    "stream processed-round counter overflow",
                )
            })?;

        self.metrics.detection_events_generated = self
            .metrics
            .detection_events_generated
            .checked_add(events.len() as u64)
            .ok_or_else(|| {
                self.status = StreamStatus::Failed;

                QecError::invalid_input(
                    "stream detection-event counter overflow",
                )
            })?;

        if self.status == StreamStatus::Closing
            && self.buffer.is_empty()
        {
            self.status = StreamStatus::Closed;
        }

        Ok(Some(StreamOutput {
            sequence,
            round,
            events,
        }))
    }

    /// Processes up to `max_rounds` buffered syndromes.
    pub fn poll(
        &mut self,
        max_rounds: usize,
    ) -> QecResult<Vec<StreamOutput>> {
        if max_rounds == 0 {
            return Err(
                StreamingError::InvalidPollSize {
                    requested: max_rounds,
                }
                .into(),
            );
        }

        if max_rounds > DEFAULT_MAX_POLL_ROUNDS {
            return Err(
                QecError::resource_limit(
                    ResourceKind::StreamBuffer,
                    max_rounds as u128,
                    0,
                    DEFAULT_MAX_POLL_ROUNDS as u128,
                    format!(
                        "poll size {max_rounds} exceeds \
                         maximum poll size {DEFAULT_MAX_POLL_ROUNDS}"
                    ),
                )
            );
        }

        self.check_cancelled()?;

        let mut outputs = Vec::new();

        for _ in 0..max_rounds {
            self.check_cancelled()?;

            let Some(output) = self.process_one()? else {
                break;
            };

            outputs.push(output);
        }

        self.metrics.polls = self
            .metrics
            .polls
            .checked_add(1)
            .ok_or_else(|| {
                QecError::invalid_input(
                    "stream poll counter overflow",
                )
            })?;

        Ok(outputs)
    }

    /// Processes all currently buffered input.
    ///
    /// This does not close the stream.
    pub fn flush(&mut self) -> QecResult<Vec<StreamOutput>> {
        self.check_cancelled()?;

        let mut outputs = Vec::new();

        while !self.buffer.is_empty() {
            self.check_cancelled()?;

            let Some(output) = self.process_one()? else {
                break;
            };

            outputs.push(output);
        }

        self.metrics.flushes = self
            .metrics
            .flushes
            .checked_add(1)
            .ok_or_else(|| {
                QecError::invalid_input(
                    "stream flush counter overflow",
                )
            })?;

        Ok(outputs)
    }

    /// Closes input and drains all already accepted syndromes.
    pub fn close(
        &mut self,
    ) -> QecResult<Vec<StreamOutput>> {
        self.check_cancelled()?;

        if self.status.is_terminal() {
            return Ok(Vec::new());
        }

        self.status = StreamStatus::Closing;

        self.flush()
    }

    // ------------------------------------------------------------------------
    // History
    // ------------------------------------------------------------------------

    fn retain_history(
        &mut self,
        syndrome: Syndrome,
    ) -> QecResult<()> {
        let capacity =
            self.mode.history_capacity();

        if capacity == 0 {
            return Err(
                StreamingError::InvalidHistorySize {
                    requested: capacity,
                }
                .into(),
            );
        }

        if self.history.len() >= capacity {
            self.history.pop_front();
        }

        self.history.push_back(syndrome);

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Snapshot
    // ------------------------------------------------------------------------

    /// Creates a deterministic in-memory snapshot.
    pub fn snapshot(&mut self) -> QecResult<StreamSnapshot> {
        self.check_cancelled()?;

        let buffered_input =
            self.buffer.iter().cloned().collect::<Vec<_>>();

        let history =
            self.history.iter().cloned().collect::<Vec<_>>();

        Ok(StreamSnapshot {
            version: STREAM_STATE_VERSION,
            status: self.status,
            next_sequence: self.next_sequence,
            mode: self.mode,
            backpressure: self.backpressure,
            lossy_allowed: self.lossy_allowed,
            buffer_capacity: self.buffer_capacity,
            buffered_input,
            history,
            metrics: self.metrics,
        })
    }

    /// Validates a stream snapshot without constructing a stream.
    pub fn validate_snapshot(
        snapshot: &StreamSnapshot,
        limits: QecLimits,
    ) -> QecResult<()> {
        if snapshot.version != STREAM_STATE_VERSION {
            return Err(
                StreamingError::UnsupportedStateVersion {
                    version: snapshot.version,
                }
                .into(),
            );
        }

        limits.validate().map_err(|error| {
            QecError::invalid_input(format!(
                "invalid QEC limits for stream restore: {error}"
            ))
        })?;

        snapshot.mode.validate().map_err(QecError::from)?;

        if snapshot.buffer_capacity == 0 {
            return Err(
                StreamingError::InvalidState {
                    message:
                        "snapshot buffer capacity is zero"
                            .to_owned(),
                }
                .into(),
            );
        }

        if snapshot.buffer_capacity
            > limits.max_stream_buffer_events
        {
            return Err(QecError::resource_limit(
                ResourceKind::StreamBuffer,
                snapshot.buffer_capacity as u128,
                0,
                limits.max_stream_buffer_events as u128,
                "snapshot buffer capacity exceeds QEC stream-buffer limit",
            ));
        }

        if snapshot.buffered_input.len()
            > snapshot.buffer_capacity
        {
            return Err(
                StreamingError::InvalidState {
                    message:
                        "snapshot contains more buffered items \
                         than its admitted capacity"
                            .to_owned(),
                }
                .into(),
            );
        }

        if snapshot.history.len()
            > snapshot.mode.history_capacity()
        {
            return Err(
                StreamingError::InvalidState {
                    message:
                        "snapshot history exceeds configured history window"
                            .to_owned(),
                }
                .into(),
            );
        }

        if snapshot.backpressure.is_lossy()
            && !snapshot.lossy_allowed
        {
            return Err(
                StreamingError::InvalidState {
                    message:
                        "snapshot enables lossy backpressure without \
                         explicit authorization"
                            .to_owned(),
                }
                .into(),
            );
        }

        if snapshot.next_sequence
            > MAX_STREAM_SEQUENCE + 1
        {
            return Err(
                StreamingError::SequenceOverflow.into()
            );
        }

        // Validate FIFO sequence ordering.
        let mut expected_sequence = snapshot
            .buffered_input
            .first()
            .map(StreamItem::sequence);

        for item in &snapshot.buffered_input {
            if let Some(expected) = expected_sequence {
                if item.sequence() != expected {
                    return Err(
                        StreamingError::InvalidState {
                            message:
                                "buffered stream sequence numbers \
                                 are not contiguous"
                                    .to_owned(),
                        }
                        .into(),
                    );
                }

                expected_sequence =
                    expected.checked_add(1);
            }
        }

        if let Some(last) =
            snapshot.buffered_input.last()
        {
            let expected_next =
                last.sequence()
                    .checked_add(1)
                    .ok_or_else(|| {
                        QecError::invalid_input(
                            "snapshot sequence overflow",
                        )
                    })?;

            if snapshot.next_sequence != expected_next {
                return Err(
                    StreamingError::InvalidState {
                        message:
                            "snapshot next sequence does not follow \
                             the buffered sequence"
                                .to_owned(),
                    }
                    .into(),
                );
            }
        }

        // Validate retained history ordering.
        let mut previous_round: Option<u64> = None;

        for syndrome in &snapshot.history {
            let round = syndrome.round().value();

            if let Some(previous) = previous_round {
                let expected =
                    previous.checked_add(1).ok_or_else(|| {
                        QecError::invalid_input(
                            "snapshot history round overflow",
                        )
                    })?;

                if round != expected {
                    return Err(
                        StreamingError::InvalidState {
                            message:
                                "snapshot history contains \
                                 non-consecutive rounds"
                                    .to_owned(),
                        }
                        .into(),
                    );
                }
            }

            previous_round = Some(round);

            syndrome.preflight()?;
        }

        for item in &snapshot.buffered_input {
            item.syndrome().preflight()?;
        }

        Ok(())
    }

    /// Restores a stream from an in-memory snapshot.
    ///
    /// The cancellation token is intentionally supplied separately. A
    /// cancellation token is execution state, not durable stream data.
    pub fn from_snapshot(
        snapshot: StreamSnapshot,
        limits: QecLimits,
        cancellation: CancellationToken,
    ) -> QecResult<Self> {
        Self::validate_snapshot(&snapshot, limits)?;

        if snapshot.status == StreamStatus::Cancelled {
            return Err(
                StreamingError::InvalidState {
                    message:
                        "a cancelled stream cannot be restored without \
                         creating a new execution context"
                            .to_owned(),
                }
                .into(),
            );
        }

        let mut stream =
            Self::with_limits_and_cancellation(
                limits,
                cancellation,
            )?;

        stream.mode = snapshot.mode;
        stream.backpressure = snapshot.backpressure;
        stream.lossy_allowed =
            snapshot.lossy_allowed;
        stream.buffer_capacity =
            snapshot.buffer_capacity;
        stream.buffer =
            VecDeque::from(snapshot.buffered_input);
        stream.history =
            VecDeque::from(snapshot.history);
        stream.next_sequence =
            snapshot.next_sequence;
        stream.status =
            snapshot.status;
        stream.metrics =
            snapshot.metrics;

        // Reconstruct the incremental processor from retained history.
        //
        // Minimal mode retains one baseline. Windowed mode may retain more
        // history, but the processor only needs consecutive input in order.
        stream.processor.reset();

        for syndrome in stream.history.iter().cloned() {
            stream.processor.push(syndrome)?;
        }

        Ok(stream)
    }

    // ------------------------------------------------------------------------
    // Consumer integration
    // ------------------------------------------------------------------------

    /// Processes all input from a `SyndromeSource`.
    ///
    /// Detection-event batches are delivered immediately to `on_output`.
    /// The complete event history is therefore not accumulated by this method.
    pub fn process_source<S, F>(
        &mut self,
        source: &mut S,
        mut on_output: F,
    ) -> QecResult<()>
    where
        S: SyndromeSource,
        F: FnMut(StreamOutput) -> QecResult<()>,
    {
        loop {
            self.check_cancelled()?;

            match source.next_syndrome()? {
                Some(syndrome) => {
                    self.submit(syndrome)?;

                    while let Some(output) =
                        self.process_one()?
                    {
                        on_output(output)?;
                    }
                }

                None => {
                    self.close()?;

                    while let Some(output) =
                        self.process_one()?
                    {
                        on_output(output)?;
                    }

                    break;
                }
            }
        }

        Ok(())
    }

    /// Implements `SyndromeSource` for the stream itself.
    ///
    /// This exposes processed buffered input to another execution layer while
    /// retaining FIFO ordering.
    pub fn next_buffered_syndrome(
        &mut self,
    ) -> QecResult<Option<Syndrome>> {
        self.check_cancelled()?;

        if let Some(item) = self.buffer.pop_front() {
            self.metrics.buffered_syndromes =
                self.buffer.len() as u64;

            Ok(Some(item.into_syndrome()))
        } else {
            Ok(None)
        }
    }
}

// ============================================================================
// SyndromeSource integration
// ============================================================================

impl SyndromeSource for SyndromeStream {
    fn next_syndrome(&mut self) -> QecResult<Option<Syndrome>> {
        self.next_buffered_syndrome()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::syndrome::{
        MeasurementConfidence,
        MeasurementRound,
        MeasurementTimestamp,
        StabilizerId,
        SyndromeMeasurement,
    };

    fn syndrome(
        round: u64,
        value: bool,
    ) -> Syndrome {
        let mut syndrome = Syndrome::new(
            MeasurementRound::new(round)
                .expect("test round must be valid"),
            MeasurementTimestamp::new(round)
                .expect("test timestamp must be valid"),
        );

        syndrome
            .insert(SyndromeMeasurement::new(
                StabilizerId::new(0),
                value,
                MeasurementConfidence::FULL,
            ))
            .expect("test measurement must be valid");

        syndrome
    }

    fn small_limits() -> QecLimits {
        let mut limits = QecLimits::default();

        limits.max_stream_buffer_events = 4;
        limits.max_syndrome_events = 64;
        limits.max_rounds = 64;

        limits
    }

    #[test]
    fn stream_starts_open() {
        let stream =
            SyndromeStream::with_limits(
                small_limits(),
            )
            .expect("stream must construct");

        assert_eq!(
            stream.status(),
            StreamStatus::Open
        );

        assert_eq!(
            stream.buffered_len(),
            0
        );

        assert_eq!(
            stream.backpressure(),
            BackpressurePolicy::Reject
        );
    }

    #[test]
    fn submission_is_fifo_and_sequenced() {
        let mut stream =
            SyndromeStream::with_limits(
                small_limits(),
            )
            .unwrap();

        assert_eq!(
            stream.submit(syndrome(0, false))
                .unwrap(),
            0
        );

        assert_eq!(
            stream.submit(syndrome(1, true))
                .unwrap(),
            1
        );

        assert_eq!(
            stream.submit(syndrome(2, false))
                .unwrap(),
            2
        );

        assert_eq!(
            stream.next_sequence(),
            3
        );
    }

    #[test]
    fn out_of_order_round_is_rejected_before_queue_mutation() {
        let mut stream =
            SyndromeStream::with_limits(
                small_limits(),
            )
            .unwrap();

        stream.submit(
            syndrome(0, false),
        )
        .unwrap();

        let result =
            stream.submit(
                syndrome(2, true),
            );

        assert!(matches!(
            result,
            Err(QecError::InvalidInput { .. })
                | Err(QecError::InvalidSyndrome { .. })
        ));

        assert_eq!(
            stream.buffered_len(),
            1
        );
    }

    #[test]
    fn full_lossless_buffer_rejects_input() {
        let mut stream =
            SyndromeStream::with_limits(
                small_limits(),
            )
            .unwrap();

        for round in 0..4 {
            stream
                .submit(
                    syndrome(round, false),
                )
                .unwrap();
        }

        let result =
            stream.submit(
                syndrome(4, true),
            );

        assert!(result.is_err());

        assert_eq!(
            stream.buffered_len(),
            4
        );
    }

    #[test]
    fn lossy_mode_requires_explicit_authorization() {
        let mut stream =
            SyndromeStream::with_limits(
                small_limits(),
            )
            .unwrap();

        assert!(
            stream
                .set_backpressure(
                    BackpressurePolicy::DropNewest,
                )
                .is_err()
        );

        stream.allow_lossy(true).unwrap();

        stream
            .set_backpressure(
                BackpressurePolicy::DropNewest,
            )
            .unwrap();

        assert_eq!(
            stream.backpressure(),
            BackpressurePolicy::DropNewest
        );
    }

    #[test]
    fn processor_generates_detection_events() {
        let mut stream =
            SyndromeStream::with_limits(
                small_limits(),
            )
            .unwrap();

        stream
            .submit(syndrome(0, false))
            .unwrap();

        stream
            .submit(syndrome(1, true))
            .unwrap();

        let first =
            stream.process_one()
                .unwrap()
                .unwrap();

        assert!(first.events.is_empty());

        let second =
            stream.process_one()
                .unwrap()
                .unwrap();

        assert_eq!(
            second.events.len(),
            1
        );
    }

    #[test]
    fn process_one_preserves_sequence() {
        let mut stream =
            SyndromeStream::with_limits(
                small_limits(),
            )
            .unwrap();

        stream
            .submit(syndrome(0, false))
            .unwrap();

        let output =
            stream.process_one()
                .unwrap()
                .unwrap();

        assert_eq!(
            output.sequence,
            0
        );

        assert_eq!(
            output.round.value(),
            0
        );
    }

    #[test]
    fn close_drains_buffer() {
        let mut stream =
            SyndromeStream::with_limits(
                small_limits(),
            )
            .unwrap();

        for round in 0..3 {
            stream
                .submit(
                    syndrome(round, false),
                )
                .unwrap();
        }

        let outputs =
            stream.close()
                .unwrap();

        assert_eq!(
            outputs.len(),
            3
        );

        assert_eq!(
            stream.status(),
            StreamStatus::Closed
        );

        assert_eq!(
            stream.buffered_len(),
            0
        );
    }

    #[test]
    fn minimal_history_retains_previous_round() {
        let mut stream =
            SyndromeStream::with_limits(
                small_limits(),
            )
            .unwrap();

        stream
            .submit(syndrome(0, false))
            .unwrap();

        stream
            .process_one()
            .unwrap();

        assert_eq!(
            stream.history().count(),
            1
        );

        assert_eq!(
            stream.previous_syndrome()
                .unwrap()
                .round()
                .value(),
            0
        );
    }

    #[test]
    fn windowed_history_is_bounded() {
        let mut stream =
            SyndromeStream::with_limits(
                small_limits(),
            )
            .unwrap();

        stream
            .set_mode(
                StreamMode::Windowed {
                    rounds: 2,
                },
            )
            .unwrap();

        for round in 0..4 {
            stream
                .submit(
                    syndrome(round, false),
                )
                .unwrap();

            stream
                .process_one()
                .unwrap();
        }

        assert_eq!(
            stream.history().count(),
            2
        );
    }

    #[test]
    fn cancellation_stops_submission() {
        let (source, token) =
            super::super::cancellation::CancellationSource::new_pair();

        source.request();

        let mut stream =
            SyndromeStream::with_limits_and_cancellation(
                small_limits(),
                token,
            )
            .unwrap();

        let result =
            stream.submit(
                syndrome(0, false),
            );

        assert!(result.is_err());

        assert_eq!(
            stream.status(),
            StreamStatus::Cancelled
        );
    }

    #[test]
    fn snapshot_round_trip() {
        let limits =
            small_limits();

        let mut stream =
            SyndromeStream::with_limits(
                limits,
            )
            .unwrap();

        stream
            .submit(syndrome(0, false))
            .unwrap();

        stream
            .process_one()
            .unwrap();

        stream
            .submit(syndrome(1, true))
            .unwrap();

        let snapshot =
            stream.snapshot()
                .unwrap();

        let restored =
            SyndromeStream::from_snapshot(
                snapshot,
                limits,
                CancellationToken::new(),
            )
            .unwrap();

        assert_eq!(
            restored.status(),
            StreamStatus::Open
        );

        assert_eq!(
            restored.next_sequence(),
            stream.next_sequence()
        );

        assert_eq!(
            restored.buffered_len(),
            stream.buffered_len()
        );
    }

    #[test]
    fn snapshot_rejects_invalid_version() {
        let limits =
            small_limits();

        let mut stream =
            SyndromeStream::with_limits(
                limits,
            )
            .unwrap();

        let mut snapshot =
            stream.snapshot()
                .unwrap();

        snapshot.version =
            STREAM_STATE_VERSION + 1;

        assert!(
            SyndromeStream::validate_snapshot(
                &snapshot,
                limits,
            )
            .is_err()
        );
    }

    #[test]
    fn stream_implements_syndrome_source() {
        let mut stream =
            SyndromeStream::with_limits(
                small_limits(),
            )
            .unwrap();

        stream
            .submit(syndrome(0, false))
            .unwrap();

        let result =
            SyndromeSource::next_syndrome(
                &mut stream,
            )
            .unwrap();

        assert!(result.is_some());
        assert_eq!(
            stream.buffered_len(),
            0
        );
    }

    #[test]
    fn metrics_remain_lossless_by_default() {
        let mut stream =
            SyndromeStream::with_limits(
                small_limits(),
            )
            .unwrap();

        stream
            .submit(syndrome(0, false))
            .unwrap();

        stream
            .process_one()
            .unwrap();

        assert!(
            stream.metrics().is_lossless()
        );

        assert_eq!(
            stream
                .metrics()
                .syndromes_dropped,
            0
        );
    }

    #[test]
    fn drop_oldest_is_explicitly_lossy() {
        let mut stream =
            SyndromeStream::with_limits(
                small_limits(),
            )
            .unwrap();

        stream.allow_lossy(true).unwrap();

        stream
            .set_backpressure(
                BackpressurePolicy::DropOldest,
            )
            .unwrap();

        for round in 0..4 {
            stream
                .submit(
                    syndrome(round, false),
                )
                .unwrap();
        }

        stream
            .submit(syndrome(4, true))
            .unwrap();

        assert_eq!(
            stream
                .metrics()
                .syndromes_dropped,
            1
        );

        assert!(
            !stream.metrics().is_lossless()
        );
    }
}