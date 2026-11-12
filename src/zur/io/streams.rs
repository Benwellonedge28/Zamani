
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — I/O Streams
//!
//! Path: src/zur/io/streams.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Provides the runtime-facing integration layer for incremental
//! synchronous and asynchronous byte-stream operations.
//!
//! ZUR reuses the canonical standard-library stream implementation
//! and adds asynchronous provider coordination.
//!
//! Supported functionality:
//!
//! - Canonical synchronous stream integration.
//! - Asynchronous byte sources.
//! - Asynchronous byte sinks.
//! - Incremental reads and writes.
//! - Partial-transfer handling.
//! - Cooperative cancellation.
//! - Explicit flushing.
//! - Explicit stream closure.
//! - Transfer accounting.
//! - Asynchronous stream copying.
//! - Structured runtime errors.
//! - Provider-independent operation.
//!
//! # ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - ZUR-facing asynchronous byte-stream contracts.
//! - Asynchronous stream-reader coordination.
//! - Asynchronous stream-writer coordination.
//! - Asynchronous stream transfer accounting.
//! - Asynchronous stream-copy coordination.
//! - Runtime cancellation checks at operation boundaries.
//!
//! It does NOT own:
//!
//! - Standard-library synchronous stream implementations.
//! - File and filesystem operations.
//! - Socket implementations.
//! - Network protocols.
//! - Distributed transport.
//! - Device discovery.
//! - Device allocation.
//! - Device memory.
//! - Quantum execution.
//! - HDL execution.
//! - AI computation.
//! - Tensor storage.
//! - Serialization.
//! - Compression.
//! - Encryption.
//! - Authentication.
//! - Authorization.
//! - Runtime scheduling.
//! - Async executors.
//! - Worker pools.
//! - Source-language stream semantics.
//! - Grammar parsing.
//! - Intermediate representations.
//!
//! Those responsibilities remain with their canonical owners.
//!
//! # CANONICAL AUTHORITIES
//!
//! Synchronous streams:
//!     crate::stdlib::io::streams
//!
//! Standard-library errors:
//!     crate::stdlib::error
//!
//! Operation context and cancellation:
//!     crate::stdlib::context
//!
//! Runtime errors:
//!     crate::zur::error
//!
//! Runtime context:
//!     crate::zur::core::context
//!
//! Asynchronous provider future:
//!     crate::zur::contracts::provider
//!
//! Async computation:
//!     crate::zur::concurrency::async
//!
//! Local channels:
//!     crate::zur::concurrency::channel
//!
//! Device transport:
//!     crate::stdlib::io::devices
//!
//! Distributed transport:
//!     crate::distributed
//!
//! Networking:
//!     crate::stdlib::net
//!     crate::stdlib::network
//!
//! # FUNDAMENTAL INVARIANTS
//!
//! 1. A stream is a logical data-flow abstraction.
//! 2. A stream does not identify a physical device.
//! 3. A stream does not imply a particular transport.
//! 4. A successful read represents actual bytes received.
//! 5. A successful write represents actual bytes accepted.
//! 6. Partial reads are valid.
//! 7. Partial writes are valid.
//! 8. Zero-length reads do not establish EOF.
//! 9. Zero-length writes do not establish closure.
//! 10. A zero-byte read from a non-empty destination establishes EOF.
//! 11. A zero-byte write with data remaining is a no-progress condition.
//! 12. Flush does not establish durable persistence.
//! 13. Cancellation is cooperative.
//! 14. Closing a stream does not imply remote transactional completion.
//! 15. Provider failures must remain failures.
//! 16. Unknown capabilities must not be treated as available.
//!
//! # SCALABILITY
//!
//! This module introduces no arbitrary maximum for:
//!
//! - Stream length.
//! - Transfer size.
//! - Number of streams.
//! - Number of providers.
//! - Number of operations.
//! - Number of devices.
//! - Number of computational domains.
//! - Number of distributed participants.
//! - Buffer size.
//! - Data volume.
//!
//! Transfers are incremental.
//!
//! Buffers are supplied by callers.
//!
//! Transfer accounting uses saturating counters and never prevents
//! otherwise valid data transfers.
//!
//! Actual limits depend on available memory, address space,
//! operational budgets, provider capabilities and platform limits.
//!
//! Literal infinite physical storage or computation is not promised.
//!
//! # CANCELLATION
//!
//! Cancellation uses the existing OperationContext.
//!
//! No second cancellation-token implementation is introduced.
//!
//! Cancellation is checked before starting new read/write operations.
//!
//! An operation already submitted to an external provider may
//! complete after cancellation has been requested.
//!
//! Successful data returned by an in-flight operation is preserved.
//!
//! Cleanup and flushing are permitted after cancellation.
//!
//! # ASYNCHRONOUS EXECUTION
//!
//! This module does not implement an executor.
//!
//! Futures are driven by the existing ZUR asynchronous execution
//! infrastructure or another compatible execution authority.
//!
//! Creating a future does not guarantee that it will be polled.
//!
//! # GRAMMAR INTEGRATION
//!
//! Canonical grammar authorities include:
//!
//! - grammar/networking/streaming.g4
//! - grammar/data/streams.g4
//! - grammar/networking/networking.g4
//! - grammar/concurrency/channels.g4
//! - grammar/spec/networking.md
//! - grammar/spec/concurrency.md
//! - grammar/specification/execution-model.md
//!
//! Grammar describes logical stream intent.
//!
//! The frontend and semantic layer validate that intent.
//!
//! Compiler lowering produces the appropriate canonical
//! representation.
//!
//! ZUR consumes the resulting runtime execution requests.
//!
//! This module never parses grammar or interprets source syntax.

// =============================================================================
// IMPORTS
// =============================================================================

use core::fmt;
use core::future::Future;
use core::num::NonZeroUsize;
use core::task::{Context, Poll};

use std::sync::Arc;

use crate::stdlib::context::OperationContext;

use crate::zur::contracts::provider::ZurProviderFuture;

use crate::zur::error::{
    ErrorKind,
    ZurError,
    ZurResult,
};

// =============================================================================
// COMPONENT IDENTITY
// =============================================================================

/// Stable ZUR stream component identifier.
pub const COMPONENT: &str = "zur.io.streams";

/// Stable asynchronous read-operation identifier.
pub const READ_OPERATION: &str = "zur.io.stream.read";

/// Stable asynchronous write-operation identifier.
pub const WRITE_OPERATION: &str = "zur.io.stream.write";

/// Stable asynchronous flush-operation identifier.
pub const FLUSH_OPERATION: &str = "zur.io.stream.flush";

/// Stable asynchronous close-operation identifier.
pub const CLOSE_OPERATION: &str = "zur.io.stream.close";

/// Stable asynchronous stream-copy identifier.
pub const COPY_OPERATION: &str = "zur.io.stream.copy";

// =============================================================================
// CANONICAL SYNCHRONOUS STREAM INTEGRATION
// =============================================================================

/// Canonical synchronous byte-source contract.
///
/// Reuses the standard-library implementation.
pub use crate::stdlib::io::streams::ByteSource;

/// Canonical synchronous byte-sink contract.
///
/// Reuses the standard-library implementation.
pub use crate::stdlib::io::streams::ByteSink;

/// Canonical synchronous stream reader.
///
/// This is a re-export, not a second implementation.
pub use crate::stdlib::io::streams::StreamReader;

/// Canonical synchronous stream writer.
///
/// This is a re-export, not a second implementation.
pub use crate::stdlib::io::streams::StreamWriter;

/// Canonical synchronous reader state.
pub use crate::stdlib::io::streams::ReaderState;

/// Canonical synchronous transfer statistics.
pub use crate::stdlib::io::streams::TransferStats;

/// Canonical synchronous incremental stream-copy operation.
///
/// The standard library remains its implementation owner.
pub use crate::stdlib::io::streams::copy_stream as copy_sync_stream;

// =============================================================================
// CANCELLATION
// =============================================================================

/// Checks cooperative cancellation before beginning a new operation.
///
/// The original standard-library cancellation error is preserved
/// as the causal source of the ZUR error.
fn check_cancelled(
    context: &OperationContext,
    operation: &'static str,
) -> ZurResult<()> {
    context.check_cancelled().map_err(|source| {
        ZurError::cancelled(
            "stream operation cancelled before execution",
        )
        .with_component(COMPONENT)
        .with_operation(operation)
        .with_source(source)
    })
}

// =============================================================================
// CONTRACT VALIDATION
// =============================================================================

/// Creates an error for a provider that violates the byte-stream contract.
fn provider_contract_error(
    message: impl Into<String>,
    operation: &'static str,
) -> ZurError {
    ZurError::new(
        ErrorKind::ContractViolation,
        crate::zur::error::ErrorCode::new(
            "zur.io.stream.provider_contract_violation",
        )
        .unwrap_or_else(|_| {
            // The literal above is a valid, reviewed error identifier.
            // This fallback keeps the function total if that invariant
            // is accidentally changed in a future edit.
            crate::zur::error::ErrorCode::new(
                "zur.io.stream.invalid",
            )
            .unwrap_or_else(|_| unreachable!())
        }),
        message,
    )
    .with_component(COMPONENT)
    .with_operation(operation)
}

// =============================================================================
// ASYNCHRONOUS TRANSFER ACCOUNTING
// =============================================================================

/// Records asynchronous stream-transfer statistics.
///
/// Accounting is descriptive only.
///
/// Counter saturation never rejects or stops a valid transfer.
///
/// When saturated, `bytes()` returns a lower-bound count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AsyncTransferStats {
    bytes: u128,
    saturated: bool,
}

impl AsyncTransferStats {
    /// Constructs empty transfer statistics.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bytes: 0,
            saturated: false,
        }
    }

    /// Returns the recorded number of bytes.
    ///
    /// When saturated, this is a lower-bound value.
    #[must_use]
    pub const fn bytes(&self) -> u128 {
        self.bytes
    }

    /// Returns whether the accounting counter saturated.
    #[must_use]
    pub const fn is_saturated(&self) -> bool {
        self.saturated
    }

    /// Records an additional successful transfer.
    ///
    /// This operation never prevents a data transfer.
    fn record(&mut self, amount: usize) {
        match self.bytes.checked_add(amount as u128) {
            Some(total) => {
                self.bytes = total;
            }
            None => {
                self.bytes = u128::MAX;
                self.saturated = true;
            }
        }
    }
}

// =============================================================================
// ASYNCHRONOUS SOURCE CONTRACT
// =============================================================================

/// Asynchronous readable byte-stream provider.
///
/// Implementations represent an actual byte source.
///
/// Examples include:
///
/// - An asynchronous file adapter.
/// - A network transport adapter.
/// - A distributed transport adapter.
/// - A device transport adapter.
/// - An accelerator data source.
/// - A storage-backed source.
/// - An in-process asynchronous source.
///
/// These examples are not a closed provider inventory.
///
/// # Contract
///
/// `read` must:
///
/// - Return the actual number of bytes received.
/// - Never return a count greater than the supplied buffer length.
/// - Support partial reads.
/// - Return zero for EOF when the destination is non-empty.
/// - Return zero for an empty destination without consuming data.
/// - Preserve actual provider failures.
///
/// `close` must perform the provider's actual close operation.
///
/// It must not report successful closure when closure failed.
///
/// # Cancellation
///
/// The supplied operation context provides cooperative cancellation.
///
/// Implementations should propagate cancellation to their underlying
/// operations where supported.
///
/// Cancellation cannot forcibly interrupt every external operation.
pub trait AsyncByteSource: Send {
    /// Reads bytes from the source.
    ///
    /// The returned count must not exceed `destination.len()`.
    ///
    /// A zero count for a non-empty destination means EOF.
    ///
    /// An empty destination must return zero without consuming input.
    fn read<'a>(
        &'a mut self,
        destination: &'a mut [u8],
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, usize>;

    /// Performs the actual provider-specific close operation.
    ///
    /// Closing is permitted after cancellation.
    fn close<'a>(
        &'a mut self,
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, ()>;
}

// =============================================================================
// ASYNCHRONOUS SINK CONTRACT
// =============================================================================

/// Asynchronous writable byte-stream provider.
///
/// # Contract
///
/// Implementations must:
///
/// - Report the actual number of bytes accepted.
/// - Support partial writes.
/// - Never report more bytes accepted than supplied.
/// - Return zero when no progress can be made.
/// - Preserve actual provider failures.
/// - Implement flush according to their provider contract.
/// - Implement actual provider-specific closure.
///
/// A successful write does not establish durable persistence.
///
/// A successful flush does not necessarily establish durable persistence.
///
/// Durability and transaction guarantees belong to the actual provider.
pub trait AsyncByteSink: Send {
    /// Writes bytes to the destination.
    ///
    /// The returned count must not exceed `source.len()`.
    ///
    /// Partial writes are valid.
    ///
    /// A zero result with non-empty input indicates no progress.
    fn write<'a>(
        &'a mut self,
        source: &'a [u8],
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, usize>;

    /// Flushes accepted output.
    ///
    /// This operation is deliberately allowed after cancellation.
    fn flush<'a>(
        &'a mut self,
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, ()>;

    /// Performs the actual provider-specific close operation.
    ///
    /// Cleanup must remain possible after cancellation.
    fn close<'a>(
        &'a mut self,
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, ()>;
}

// =============================================================================
// ASYNCHRONOUS STREAM READER
// =============================================================================

/// Runtime-managed asynchronous byte-stream reader.
///
/// The reader coordinates asynchronous provider operations.
///
/// It does not implement a transport or executor.
///
/// Input is consumed incrementally without materializing the
/// complete stream.
///
/// # EOF
///
/// EOF is recorded only when the provider returns zero for
/// a non-empty destination.
///
/// An empty destination never changes EOF state.
pub struct AsyncStreamReader<S> {
    inner: S,
    context: OperationContext,
    state: ReaderState,
    stats: AsyncTransferStats,
}

impl<S> AsyncStreamReader<S>
where
    S: AsyncByteSource,
{
    /// Creates an asynchronous reader with a default operation context.
    #[must_use]
    pub fn new(inner: S) -> Self {
        Self::with_context(
            inner,
            OperationContext::builder().build(),
        )
    }

    /// Creates an asynchronous reader with an explicit context.
    #[must_use]
    pub fn with_context(
        inner: S,
        context: OperationContext,
    ) -> Self {
        Self {
            inner,
            context,
            state: ReaderState::Open,
            stats: AsyncTransferStats::new(),
        }
    }

    /// Reads bytes incrementally.
    ///
    /// # Errors
    ///
    /// Returns a structured ZUR error when:
    ///
    /// - Cancellation is requested before the operation.
    /// - The provider fails.
    /// - The provider violates its transfer contract.
    pub async fn read(
        &mut self,
        destination: &mut [u8],
    ) -> ZurResult<usize> {
        if destination.is_empty() {
            return Ok(0);
        }

        if self.is_eof() {
            return Ok(0);
        }

        check_cancelled(
            &self.context,
            READ_OPERATION,
        )?;

        let amount = self
            .inner
            .read(
                destination,
                &self.context,
            )
            .await?;

        if amount > destination.len() {
            return Err(provider_contract_error(
                "source reported more bytes than the destination can hold",
                READ_OPERATION,
            ));
        }

        if amount == 0 {
            self.state = ReaderState::EndOfStream;
        } else {
            self.stats.record(amount);
        }

        Ok(amount)
    }

    /// Returns the observed reader state.
    #[must_use]
    pub const fn state(&self) -> ReaderState {
        self.state
    }

    /// Returns whether EOF has been observed.
    #[must_use]
    pub const fn is_eof(&self) -> bool {
        matches!(
            self.state,
            ReaderState::EndOfStream
        )
    }

    /// Returns asynchronous transfer statistics.
    #[must_use]
    pub const fn stats(&self) -> &AsyncTransferStats {
        &self.stats
    }

    /// Returns the operation context.
    #[must_use]
    pub const fn context(&self) -> &OperationContext {
        &self.context
    }

    /// Returns a shared reference to the underlying provider.
    #[must_use]
    pub const fn get_ref(&self) -> &S {
        &self.inner
    }

    /// Returns a mutable reference to the underlying provider.
    ///
    /// Direct access bypasses reader state and transfer accounting.
    pub fn get_mut(&mut self) -> &mut S {
        &mut self.inner
    }

    /// Consumes the reader and performs actual provider closure.
    ///
    /// Closure is not prevented by cancellation.
    ///
    /// # Errors
    ///
    /// Returns the actual provider close failure.
    pub async fn close(mut self) -> ZurResult<()> {
        self.inner
            .close(&self.context)
            .await
            .map_err(|error| {
                error
                    .with_component(COMPONENT)
                    .with_operation(CLOSE_OPERATION)
            })
    }

    /// Consumes the reader and returns its underlying provider.
    ///
    /// This deliberately does not claim that the provider was closed.
    #[must_use]
    pub fn into_inner(self) -> S {
        self.inner
    }
}

impl<S> fmt::Debug for AsyncStreamReader<S> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("AsyncStreamReader")
            .field("state", &self.state)
            .field("stats", &self.stats)
            .finish_non_exhaustive()
    }
}

// =============================================================================
// ASYNCHRONOUS STREAM WRITER
// =============================================================================

/// Runtime-managed asynchronous byte-stream writer.
///
/// Writes are incremental.
///
/// The complete output is never required to fit into one allocation.
///
/// Partial writes are handled by `write_all`.
///
/// The underlying provider owns the actual transport and storage behavior.
pub struct AsyncStreamWriter<W> {
    inner: W,
    context: OperationContext,
    stats: AsyncTransferStats,
}

impl<W> AsyncStreamWriter<W>
where
    W: AsyncByteSink,
{
    /// Creates an asynchronous writer with a default operation context.
    #[must_use]
    pub fn new(inner: W) -> Self {
        Self::with_context(
            inner,
            OperationContext::builder().build(),
        )
    }

    /// Creates an asynchronous writer with an explicit context.
    #[must_use]
    pub fn with_context(
        inner: W,
        context: OperationContext,
    ) -> Self {
        Self {
            inner,
            context,
            stats: AsyncTransferStats::new(),
        }
    }

    /// Writes a portion of the supplied data.
    ///
    /// The returned count is the actual number of bytes accepted.
    ///
    /// A partial result is valid.
    ///
    /// # Errors
    ///
    /// Returns a structured error on cancellation, provider failure
    /// or provider contract violation.
    pub async fn write(
        &mut self,
        source: &[u8],
    ) -> ZurResult<usize> {
        if source.is_empty() {
            return Ok(0);
        }

        check_cancelled(
            &self.context,
            WRITE_OPERATION,
        )?;

        let amount = self
            .inner
            .write(
                source,
                &self.context,
            )
            .await?;

        if amount > source.len() {
            return Err(provider_contract_error(
                "sink reported accepting more bytes than were supplied",
                WRITE_OPERATION,
            ));
        }

        self.stats.record(amount);

        Ok(amount)
    }

    /// Writes all supplied bytes.
    ///
    /// Correctly handles partial writes.
    ///
    /// A provider that makes no progress while bytes remain
    /// produces an explicit contract error instead of an infinite loop.
    pub async fn write_all(
        &mut self,
        mut remaining: &[u8],
    ) -> ZurResult<()> {
        while !remaining.is_empty() {
            check_cancelled(
                &self.context,
                WRITE_OPERATION,
            )?;

            let amount = self.write(remaining).await?;

            if amount == 0 {
                return Err(provider_contract_error(
                    "sink made no progress while data remained",
                    WRITE_OPERATION,
                ));
            }

            remaining = &remaining[amount..];
        }

        Ok(())
    }

    /// Flushes previously accepted output.
    ///
    /// Flush is permitted after cancellation.
    ///
    /// It does not independently guarantee durable persistence.
    pub async fn flush(&mut self) -> ZurResult<()> {
        self.inner
            .flush(&self.context)
            .await
            .map_err(|error| {
                error
                    .with_component(COMPONENT)
                    .with_operation(FLUSH_OPERATION)
            })
    }

    /// Completes the stream.
    ///
    /// Both flushing and provider closure are attempted.
    ///
    /// If flushing fails, that error is preserved even if closure
    /// also fails.
    ///
    /// If flushing succeeds but closure fails, the close error
    /// is returned.
    ///
    /// Successful completion means both operations succeeded.
    ///
    /// It does not establish durable persistence unless the
    /// provider explicitly guarantees it.
    pub async fn finish(
        mut self,
    ) -> ZurResult<W> {
        let flush_result = self.flush().await;

        let close_result = self
            .inner
            .close(&self.context)
            .await
            .map_err(|error| {
                error
                    .with_component(COMPONENT)
                    .with_operation(CLOSE_OPERATION)
            });

        match (flush_result, close_result) {
            (Err(error), _) => Err(error),

            (Ok(()), Err(error)) => Err(error),

            (Ok(()), Ok(())) => Ok(self.inner),
        }
    }

    /// Flushes, closes and drops the underlying provider.
    pub async fn close(
        self,
    ) -> ZurResult<()> {
        drop(self.finish().await?);
        Ok(())
    }

    /// Returns asynchronous transfer statistics.
    #[must_use]
    pub const fn stats(&self) -> &AsyncTransferStats {
        &self.stats
    }

    /// Returns the operation context.
    #[must_use]
    pub const fn context(&self) -> &OperationContext {
        &self.context
    }

    /// Returns a shared reference to the underlying provider.
    #[must_use]
    pub const fn get_ref(&self) -> &W {
        &self.inner
    }

    /// Returns a mutable reference to the underlying provider.
    ///
    /// Direct access bypasses transfer accounting.
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }
}

impl<W> fmt::Debug for AsyncStreamWriter<W> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("AsyncStreamWriter")
            .field("stats", &self.stats)
            .finish_non_exhaustive()
    }
}

// =============================================================================
// ASYNCHRONOUS STREAM COPY
// =============================================================================

/// Copies data incrementally between asynchronous stream endpoints.
///
/// The caller supplies the transfer buffer.
///
/// The complete source is never materialized in memory.
///
/// Partial reads and writes are supported.
///
/// The destination is flushed after successful completion.
///
/// This function does not close either endpoint.
///
/// # Arguments
///
/// * `reader` - Actual asynchronous source.
/// * `writer` - Actual asynchronous destination.
/// * `buffer` - Caller-owned transfer buffer.
/// 
/// # Returns
///
/// The number of bytes read from the source, subject to the
/// documented saturating accounting representation.
///
/// # Errors
///
/// Returns an error when:
///
/// - The buffer is empty.
/// - Cancellation is requested.
/// - The source fails.
/// - The destination fails.
/// - The destination makes no progress.
/// - A provider violates its transfer contract.
/// - Final flushing fails.
///
/// Bytes already accepted by the destination cannot necessarily
/// be rolled back after a subsequent failure.
pub async fn copy_async_stream<S, W>(
    reader: &mut AsyncStreamReader<S>,
    writer: &mut AsyncStreamWriter<W>,
    buffer: &mut [u8],
) -> ZurResult<u128>
where
    S: AsyncByteSource,
    W: AsyncByteSink,
{
    if buffer.is_empty() {
        return Err(
            ZurError::invalid_input(
                "asynchronous stream-copy buffer must not be empty",
            )
            .with_component(COMPONENT)
            .with_operation(COPY_OPERATION),
        );
    }

    let initial_read = reader.stats().bytes();

    let mut transferred = 0_u128;
    let mut saturated = false;

    loop {
        check_cancelled(
            reader.context(),
            COPY_OPERATION,
        )?;

        let amount = reader.read(buffer).await?;

        if amount == 0 {
            break;
        }

        writer.write_all(&buffer[..amount]).await?;

        match transferred.checked_add(amount as u128) {
            Some(total) => {
                transferred = total;
            }
            None => {
                transferred = u128::MAX;
                saturated = true;
            }
        }
    }

    writer.flush().await?;

    // Reader statistics are descriptive. If their counter saturated,
    // the returned transfer count is still a lower-bound value.
    if reader.stats().is_saturated() {
        saturated = true;
    }

    if !saturated {
        let observed = reader
            .stats()
            .bytes()
            .saturating_sub(initial_read);

        if observed != transferred {
            return Err(provider_contract_error(
                "asynchronous stream-copy accounting mismatch",
                COPY_OPERATION,
            ));
        }
    }

    Ok(transferred)
}

// =============================================================================
// PUBLIC COMPONENT METADATA
// =============================================================================

/// Returns the stable ZUR stream component identifier.
#[must_use]
pub const fn component_name() -> &'static str {
    COMPONENT
}

/// Returns the current stream integration generation.
///
/// This is not a language version, ABI version or provider version.
#[must_use]
pub const fn integration_generation() -> u32 {
    1
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use core::future::Future;
    use core::task::{Wake, Waker};

    use std::io::{Cursor, Read};
    use std::sync::atomic::{
        AtomicBool,
        Ordering,
    };

    // ------------------------------------------------------------------------
    // TEST FUTURE DRIVER
    // ------------------------------------------------------------------------

    struct NoopWake;

    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {}

        fn wake_by_ref(self: &Arc<Self>) {}
    }

    fn poll_ready<F>(future: F) -> F::Output
    where
        F: Future,
    {
        let waker = Waker::from(Arc::new(NoopWake));

        let mut context = Context::from_waker(&waker);

        let mut future = Box::pin(future);

        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => output,

            Poll::Pending => {
                panic!("test future unexpectedly requires an executor");
            }
        }
    }

    // ------------------------------------------------------------------------
    // TEST PROVIDERS
    // ------------------------------------------------------------------------

    struct TestSource {
        input: Cursor<Vec<u8>>,
        maximum_read: usize,
        closed: bool,
    }

    impl AsyncByteSource for TestSource {
        fn read<'a>(
            &'a mut self,
            destination: &'a mut [u8],
            _context: &'a OperationContext,
        ) -> ZurProviderFuture<'a, usize> {
            Box::pin(async move {
                let length = destination
                    .len()
                    .min(self.maximum_read);

                self.input
                    .read(&mut destination[..length])
                    .map_err(ZurError::from)
            })
        }

        fn close<'a>(
            &'a mut self,
            _context: &'a OperationContext,
        ) -> ZurProviderFuture<'a, ()> {
            Box::pin(async move {
                self.closed = true;
                Ok(())
            })
        }
    }

    #[derive(Default)]
    struct TestSink {
        output: Vec<u8>,
        maximum_write: usize,
        closed: bool,
        flushes: usize,
    }

    impl AsyncByteSink for TestSink {
        fn write<'a>(
            &'a mut self,
            source: &'a [u8],
            _context: &'a OperationContext,
        ) -> ZurProviderFuture<'a, usize> {
            Box::pin(async move {
                let amount = source
                    .len()
                    .min(self.maximum_write);

                self.output.extend_from_slice(
                    &source[..amount],
                );

                Ok(amount)
            })
        }

        fn flush<'a>(
            &'a mut self,
            _context: &'a OperationContext,
        ) -> ZurProviderFuture<'a, ()> {
            Box::pin(async move {
                self.flushes += 1;
                Ok(())
            })
        }

        fn close<'a>(
            &'a mut self,
            _context: &'a OperationContext,
        ) -> ZurProviderFuture<'a, ()> {
            Box::pin(async move {
                self.closed = true;
                Ok(())
            })
        }
    }

    // ------------------------------------------------------------------------
    // CANCELLATION PROVIDER
    // ------------------------------------------------------------------------

    struct TestCancellation {
        cancelled: AtomicBool,
    }

    impl crate::stdlib::context::CancellationSignal
        for TestCancellation
    {
        fn is_cancelled(&self) -> bool {
            self.cancelled.load(Ordering::Acquire)
        }
    }

    fn cancelled_context() -> OperationContext {
        let signal = Arc::new(TestCancellation {
            cancelled: AtomicBool::new(true),
        });

        OperationContext::builder()
            .with_cancellation(signal)
            .build()
    }

    // ------------------------------------------------------------------------
    // SOURCE CONSTRUCTION
    // ------------------------------------------------------------------------

    fn source(data: &[u8], maximum_read: usize) -> TestSource {
        TestSource {
            input: Cursor::new(data.to_vec()),
            maximum_read,
            closed: false,
        }
    }

    fn sink(maximum_write: usize) -> TestSink {
        TestSink {
            maximum_write,
            ..TestSink::default()
        }
    }

    // ------------------------------------------------------------------------
    // COMPONENT CONTRACT
    // ------------------------------------------------------------------------

    #[test]
    fn exposes_component_identity() {
        assert_eq!(
            component_name(),
            "zur.io.streams",
        );
    }

    #[test]
    fn exposes_integration_generation() {
        assert_eq!(
            integration_generation(),
            1,
        );
    }

    // ------------------------------------------------------------------------
    // TRANSFER ACCOUNTING
    // ------------------------------------------------------------------------

    #[test]
    fn transfer_statistics_start_at_zero() {
        let stats = AsyncTransferStats::new();

        assert_eq!(stats.bytes(), 0);
        assert!(!stats.is_saturated());
    }

    // ------------------------------------------------------------------------
    // READING
    // ------------------------------------------------------------------------

    #[test]
    fn reader_returns_actual_input() {
        let mut reader = AsyncStreamReader::new(
            source(b"Zamani", 6),
        );

        let mut buffer = [0_u8; 6];

        let amount = poll_ready(
            reader.read(&mut buffer),
        )
        .unwrap();

        assert_eq!(amount, 6);
        assert_eq!(&buffer, b"Zamani");
        assert_eq!(reader.stats().bytes(), 6);
    }

    #[test]
    fn reader_supports_partial_reads() {
        let mut reader = AsyncStreamReader::new(
            source(b"abcdef", 2),
        );

        let mut output = Vec::new();

        let mut buffer = [0_u8; 4];

        loop {
            let amount = poll_ready(
                reader.read(&mut buffer),
            )
            .unwrap();

            if amount == 0 {
                break;
            }

            output.extend_from_slice(
                &buffer[..amount],
            );
        }

        assert_eq!(output, b"abcdef");
    }

    #[test]
    fn reader_detects_eof() {
        let mut reader = AsyncStreamReader::new(
            source(b"", 4),
        );

        let mut buffer = [0_u8; 4];

        let amount = poll_ready(
            reader.read(&mut buffer),
        )
        .unwrap();

        assert_eq!(amount, 0);
        assert!(reader.is_eof());
    }

    #[test]
    fn empty_read_does_not_establish_eof() {
        let mut reader = AsyncStreamReader::new(
            source(b"data", 4),
        );

        let mut buffer = [];

        assert_eq!(
            poll_ready(reader.read(&mut buffer)).unwrap(),
            0,
        );

        assert_eq!(
            reader.state(),
            ReaderState::Open,
        );
    }

    #[test]
    fn eof_is_stable() {
        let mut reader = AsyncStreamReader::new(
            source(b"", 4),
        );

        let mut buffer = [0_u8; 4];

        assert_eq!(
            poll_ready(reader.read(&mut buffer)).unwrap(),
            0,
        );

        assert_eq!(
            poll_ready(reader.read(&mut buffer)).unwrap(),
            0,
        );

        assert!(reader.is_eof());
    }

    // ------------------------------------------------------------------------
    // WRITING
    // ------------------------------------------------------------------------

    #[test]
    fn writer_accepts_real_output() {
        let mut writer = AsyncStreamWriter::new(
            sink(8),
        );

        poll_ready(
            writer.write_all(b"Zamani"),
        )
        .unwrap();

        let provider = poll_ready(
            writer.finish(),
        )
        .unwrap();

        assert_eq!(provider.output, b"Zamani");
        assert!(provider.closed);
        assert_eq!(provider.flushes, 1);
    }

    #[test]
    fn writer_handles_partial_writes() {
        let mut writer = AsyncStreamWriter::new(
            sink(2),
        );

        poll_ready(
            writer.write_all(b"partial transfer"),
        )
        .unwrap();

        assert_eq!(
            writer.stats().bytes(),
            16,
        );

        let provider = poll_ready(
            writer.finish(),
        )
        .unwrap();

        assert_eq!(
            provider.output,
            b"partial transfer",
        );
    }

    #[test]
    fn empty_write_does_not_modify_accounting() {
        let mut writer = AsyncStreamWriter::new(
            sink(4),
        );

        assert_eq!(
            poll_ready(writer.write(&[])).unwrap(),
            0,
        );

        assert_eq!(
            writer.stats().bytes(),
            0,
        );
    }

    // ------------------------------------------------------------------------
    // STREAM COPYING
    // ------------------------------------------------------------------------

    #[test]
    fn asynchronous_copy_transfers_actual_bytes() {
        let mut reader = AsyncStreamReader::new(
            source(b"streaming across providers", 3),
        );

        let mut writer = AsyncStreamWriter::new(
            sink(2),
        );

        let mut buffer = [0_u8; 5];

        let transferred = poll_ready(
            copy_async_stream(
                &mut reader,
                &mut writer,
                &mut buffer,
            ),
        )
        .unwrap();

        assert_eq!(transferred, 27);

        let provider = poll_ready(
            writer.finish(),
        )
        .unwrap();

        assert_eq!(
            provider.output,
            b"streaming across providers",
        );
    }

    #[test]
    fn asynchronous_copy_rejects_empty_buffer() {
        let mut reader = AsyncStreamReader::new(
            source(b"data", 2),
        );

        let mut writer = AsyncStreamWriter::new(
            sink(2),
        );

        let mut buffer = [];

        let result = poll_ready(
            copy_async_stream(
                &mut reader,
                &mut writer,
                &mut buffer,
            ),
        );

        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // CANCELLATION
    // ------------------------------------------------------------------------

    #[test]
    fn reader_rejects_pre_cancelled_operation() {
        let mut reader = AsyncStreamReader::with_context(
            source(b"data", 4),
            cancelled_context(),
        );

        let mut buffer = [0_u8; 4];

        let result = poll_ready(
            reader.read(&mut buffer),
        );

        assert!(result.is_err());

        assert_eq!(
            reader.stats().bytes(),
            0,
        );
    }

    #[test]
    fn writer_rejects_pre_cancelled_operation() {
        let mut writer = AsyncStreamWriter::with_context(
            sink(4),
            cancelled_context(),
        );

        let result = poll_ready(
            writer.write_all(b"data"),
        );

        assert!(result.is_err());

        assert_eq!(
            writer.stats().bytes(),
            0,
        );
    }

    // ------------------------------------------------------------------------
    // CLOSURE
    // ------------------------------------------------------------------------

    #[test]
    fn reader_closes_actual_provider() {
        let reader = AsyncStreamReader::new(
            source(b"data", 4),
        );

        let provider = poll_ready(
            reader.close(),
        );

        assert!(provider.is_ok());
    }

    #[test]
    fn writer_flushes_and_closes_provider() {
        let writer = AsyncStreamWriter::new(
            sink(4),
        );

        let provider = poll_ready(
            writer.finish(),
        )
        .unwrap();

        assert!(provider.closed);
        assert_eq!(provider.flushes, 1);
    }

    // ------------------------------------------------------------------------
    // CONTRACT ERRORS
    // ------------------------------------------------------------------------

    struct InvalidSource;

    impl AsyncByteSource for InvalidSource {
        fn read<'a>(
            &'a mut self,
            _destination: &'a mut [u8],
            _context: &'a OperationContext,
        ) -> ZurProviderFuture<'a, usize> {
            Box::pin(async {
                Ok(100)
            })
        }

        fn close<'a>(
            &'a mut self,
            _context: &'a OperationContext,
        ) -> ZurProviderFuture<'a, ()> {
            Box::pin(async {
                Ok(())
            })
        }
    }

    #[test]
    fn reader_rejects_invalid_provider_transfer_count() {
        let mut reader = AsyncStreamReader::new(
            InvalidSource,
        );

        let mut buffer = [0_u8; 4];

        let result = poll_ready(
            reader.read(&mut buffer),
        );

        assert!(result.is_err());
    }

    struct NoProgressSink;

    impl AsyncByteSink for NoProgressSink {
        fn write<'a>(
            &'a mut self,
            _source: &'a [u8],
            _context: &'a OperationContext,
        ) -> ZurProviderFuture<'a, usize> {
            Box::pin(async {
                Ok(0)
            })
        }

        fn flush<'a>(
            &'a mut self,
            _context: &'a OperationContext,
        ) -> ZurProviderFuture<'a, ()> {
            Box::pin(async {
                Ok(())
            })
        }

        fn close<'a>(
            &'a mut self,
            _context: &'a OperationContext,
        ) -> ZurProviderFuture<'a, ()> {
            Box::pin(async {
                Ok(())
            })
        }
    }

    #[test]
    fn writer_rejects_no_progress() {
        let mut writer = AsyncStreamWriter::new(
            NoProgressSink,
        );

        let result = poll_ready(
            writer.write_all(b"data"),
        );

        assert!(result.is_err());
    }
}
