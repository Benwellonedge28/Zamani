
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — I/O Streams
//!
//! Path: src/stdlib/io/streams.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides portable, provider-independent byte-stream interfaces
//! for incremental input and output.
//!
//! The implementation supports:
//!
//! - Generic synchronous readers.
//! - Generic synchronous writers.
//! - Configurable buffering.
//! - Partial reads and writes.
//! - End-of-stream detection.
//! - Explicit flushing.
//! - Explicit stream completion.
//! - Cooperative cancellation.
//! - Structured error propagation.
//! - Byte-transfer accounting.
//! - Integration with storage and serialization providers.
//!
//! ## Architectural ownership
//!
//! This module owns stream adapters and stream-operation contracts.
//!
//! It does not own:
//!
//! - Filesystem implementations.
//! - Network transports.
//! - Socket implementations.
//! - Distributed routing.
//! - Hardware allocation.
//! - Device memory.
//! - Quantum execution.
//! - Runtime scheduling.
//! - Provider discovery.
//! - Tensor storage.
//! - Serialization formats.
//! - Source-language semantics.
//! - Intermediate representations.
//!
//! Those responsibilities remain with their respective authorities.
//!
//! ## Integration
//!
//! Structured errors:
//! crate::stdlib::error
//!
//! Operation context:
//! crate::stdlib::context
//!
//! Storage:
//! crate::stdlib::data::storage
//!
//! Serialization:
//! crate::stdlib::data::serialization
//!
//! Byte buffers:
//! crate::stdlib::data::buffers
//!
//! Filesystem compatibility:
//! crate::stdlib::fs
//!
//! Networking:
//! crate::stdlib::net
//! crate::stdlib::network
//!
//! Runtime execution:
//! crate::runtime
//!
//! ## Scalability
//!
//! No arbitrary stream-size, transfer-size, buffer-size,
//! stream-count, provider-count, or event-count ceiling is imposed.
//!
//! Transfers are incremental.
//!
//! Buffer capacities are caller-configurable.
//!
//! Transfer accounting does not limit the amount of data transferred.
//!
//! Actual limits are determined by:
//!
//! - Available memory.
//! - Platform address-space limits.
//! - Provider capabilities.
//! - Underlying reader/writer behavior.
//! - Explicit operational budgets.
//!
//! Physical limitations must not become language-level limits.
//!
//! ## Error guarantees
//!
//! Successful writes represent bytes accepted by the underlying writer.
//!
//! Successful flushes represent successful completion of the
//! underlying writer's flush operation.
//!
//! Successful completion represents successful flushing.
//!
//! A failed operation is never reported as successful.
//!
//! ## Cancellation
//!
//! Cancellation is cooperative.
//!
//! Reads and writes check the supplied operation context.
//!
//! Flush is deliberately allowed to proceed after cancellation,
//! because flushing may be necessary to preserve already accepted data.
//!
//! Cancellation cannot forcibly interrupt a blocking operating-system
//! call or remote provider operation.
//!
//! ## Safety
//!
//! This module uses safe Rust only.

use core::fmt;
use core::num::NonZeroUsize;

use std::io::{
    self,
    BufReader,
    BufWriter,
    Read,
    Write,
};

use crate::stdlib::context::OperationContext;

use crate::stdlib::error::{
    Error,
    ErrorKind,
    Result,
};

// ============================================================================
// CONSTANTS AND IDENTIFICATION
// ============================================================================

/// Stable standard-library component identifier.
pub const COMPONENT: &str = "stdlib.io.streams";

/// Stable read-operation identifier.
pub const READ_OPERATION: &str = "io.stream.read";

/// Stable write-operation identifier.
pub const WRITE_OPERATION: &str = "io.stream.write";

/// Stable flush-operation identifier.
pub const FLUSH_OPERATION: &str = "io.stream.flush";

/// Stable completion-operation identifier.
pub const FINISH_OPERATION: &str = "io.stream.finish";

// ============================================================================
// STREAM ACCOUNTING
// ============================================================================

/// Records the number of bytes transferred by a stream adapter.
///
/// Accounting is descriptive only.
///
/// Counter saturation never stops or changes a stream transfer.
/// The `counter_saturated` field indicates that the accounting
/// representation could no longer express the exact total.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TransferStats {
    bytes: u128,
    counter_saturated: bool,
}

impl TransferStats {
    /// Creates empty transfer statistics.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bytes: 0,
            counter_saturated: false,
        }
    }

    /// Returns the recorded number of bytes.
    ///
    /// When `counter_saturated` is true, this is a lower-bound
    /// accounting value rather than an exact total.
    #[must_use]
    pub const fn bytes(&self) -> u128 {
        self.bytes
    }

    /// Returns whether the accounting counter saturated.
    #[must_use]
    pub const fn counter_saturated(&self) -> bool {
        self.counter_saturated
    }

    /// Records an additional successful transfer.
    ///
    /// This operation never rejects a valid I/O transfer.
    fn record(&mut self, amount: usize) {
        match self.bytes.checked_add(amount as u128) {
            Some(total) => {
                self.bytes = total;
            }
            None => {
                self.bytes = u128::MAX;
                self.counter_saturated = true;
            }
        }
    }
}

// ============================================================================
// STREAM STATUS
// ============================================================================

/// Describes the observed state of a stream reader.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReaderState {
    /// The stream may provide additional bytes.
    Open,

    /// The underlying reader reported end-of-stream.
    EndOfStream,
}

// ============================================================================
// ERROR CONVERSION
// ============================================================================

/// Converts an underlying I/O error into a structured Zamani error.
///
/// The original error is retained as the causal source.
fn io_error(
    error: io::Error,
    operation: &'static str,
) -> Error {
    let kind = match error.kind() {
        io::ErrorKind::PermissionDenied => ErrorKind::PermissionDenied,

        io::ErrorKind::NotFound => ErrorKind::NotFound,

        io::ErrorKind::AlreadyExists => ErrorKind::AlreadyExists,

        io::ErrorKind::InvalidInput => ErrorKind::InvalidInput,

        io::ErrorKind::InvalidData => ErrorKind::InvalidData,

        io::ErrorKind::TimedOut => ErrorKind::TimedOut,

        io::ErrorKind::Interrupted => ErrorKind::Cancelled,

        io::ErrorKind::WouldBlock => ErrorKind::ResourceUnavailable,

        io::ErrorKind::OutOfMemory => ErrorKind::ResourceUnavailable,

        _ => ErrorKind::Io,
    };

    Error::new(kind, error.to_string())
        .in_component(COMPONENT)
        .during_operation(operation)
        .with_source(error)
}

/// Converts a structured Zamani cancellation error into an I/O error.
fn cancellation_io_error(
    error: Error,
) -> io::Error {
    io::Error::new(
        io::ErrorKind::Interrupted,
        error,
    )
}

// ============================================================================
// BYTE STREAM CONTRACTS
// ============================================================================

/// A readable byte-stream contract.
///
/// Implementations must:
///
/// - Support incremental reads.
/// - Return zero only when the stream has reached EOF or the
///   supplied buffer is empty.
/// - Preserve the underlying reader's error behavior.
/// - Avoid fabricating input.
///
/// The standard library does not require a particular transport.
pub trait ByteSource: Read + Send {}

impl<T> ByteSource for T
where
    T: Read + Send,
{
}

/// A writable byte-stream contract.
///
/// Implementations must:
///
/// - Support incremental writes.
/// - Report the actual number of bytes accepted.
/// - Preserve partial-write behavior.
/// - Report underlying failures.
/// - Implement flush according to their documented contract.
pub trait ByteSink: Write + Send {}

impl<T> ByteSink for T
where
    T: Write + Send,
{
}

// ============================================================================
// STREAM READER
// ============================================================================

/// A portable, cancellation-aware stream reader.
///
/// The reader wraps any Rust `Read` implementation.
///
/// It does not require the complete input to be materialized
/// in memory.
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
///
/// use zamani::stdlib::io::streams::StreamReader;
///
/// let source = Cursor::new(b"Zamani");
///
/// let mut reader = StreamReader::new(source);
///
/// let mut buffer = [0_u8; 3];
///
/// let amount = reader.read_checked(&mut buffer).unwrap();
///
/// assert_eq!(amount, 3);
/// assert_eq!(&buffer, b"Zam");
/// ```
pub struct StreamReader<R> {
    inner: R,
    context: OperationContext,
    state: ReaderState,
    stats: TransferStats,
}

impl<R> StreamReader<R>
where
    R: Read,
{
    /// Creates a stream reader with a default operation context.
    #[must_use]
    pub fn new(inner: R) -> Self {
        Self::with_context(
            inner,
            OperationContext::builder().build(),
        )
    }

    /// Creates a reader with an explicit operation context.
    #[must_use]
    pub fn with_context(
        inner: R,
        context: OperationContext,
    ) -> Self {
        Self {
            inner,
            context,
            state: ReaderState::Open,
            stats: TransferStats::new(),
        }
    }

    /// Creates a reader using a configurable buffer.
    ///
    /// The supplied capacity is a buffering preference, not a
    /// maximum stream size or maximum transfer size.
    ///
    /// No arbitrary maximum buffer capacity is imposed.
    #[must_use]
    pub fn buffered(
        inner: R,
        capacity: NonZeroUsize,
    ) -> StreamReader<BufReader<R>> {
        StreamReader::new(
            BufReader::with_capacity(
                capacity.get(),
                inner,
            ),
        )
    }

    /// Creates a buffered reader with an explicit context.
    #[must_use]
    pub fn buffered_with_context(
        inner: R,
        capacity: NonZeroUsize,
        context: OperationContext,
    ) -> StreamReader<BufReader<R>> {
        StreamReader::with_context(
            BufReader::with_capacity(
                capacity.get(),
                inner,
            ),
            context,
        )
    }

    /// Reads bytes and returns a structured Zamani error.
    ///
    /// An empty destination buffer returns zero without
    /// changing the stream state.
    pub fn read_checked(
        &mut self,
        buffer: &mut [u8],
    ) -> Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }

        self.context.check_cancelled()?;

        self.read(buffer)
            .map_err(|error| {
                io_error(error, READ_OPERATION)
            })
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

    /// Returns transfer statistics.
    #[must_use]
    pub const fn stats(&self) -> &TransferStats {
        &self.stats
    }

    /// Returns the operation context.
    #[must_use]
    pub const fn context(&self) -> &OperationContext {
        &self.context
    }

    /// Returns a shared reference to the underlying reader.
    #[must_use]
    pub const fn get_ref(&self) -> &R {
        &self.inner
    }

    /// Returns a mutable reference to the underlying reader.
    ///
    /// Direct access bypasses this adapter's cancellation and
    /// transfer-accounting behavior.
    ///
    /// Callers should use this only when such bypass is intentional.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    /// Consumes the adapter and returns its underlying reader.
    ///
    /// Any unread bytes already buffered inside the reader remain
    /// inside that reader and are not discarded.
    #[must_use]
    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R> Read for StreamReader<R>
where
    R: Read,
{
    fn read(
        &mut self,
        buffer: &mut [u8],
    ) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }

        if self.is_eof() {
            return Ok(0);
        }

        if let Err(error) = self.context.check_cancelled() {
            return Err(cancellation_io_error(error));
        }

        let amount = self.inner.read(buffer)?;

        if amount == 0 {
            self.state = ReaderState::EndOfStream;
        } else {
            self.stats.record(amount);
        }

        Ok(amount)
    }
}

// ============================================================================
// STREAM WRITER
// ============================================================================

/// A portable, cancellation-aware stream writer.
///
/// The writer wraps any Rust `Write` implementation.
///
/// Writes are incremental and do not require the entire output
/// to be retained in memory.
///
/// Successful writes indicate that the underlying writer
/// accepted the reported number of bytes.
///
/// They do not independently guarantee durable persistence.
///
/// Durability and publication are provider-specific contracts.
///
/// # Examples
///
/// ```
/// use zamani::stdlib::io::streams::StreamWriter;
///
/// let mut writer = StreamWriter::new(Vec::new());
///
/// writer.write_all_checked(b"Zamani").unwrap();
///
/// let output = writer.finish().unwrap();
///
/// assert_eq!(output, b"Zamani");
/// ```
pub struct StreamWriter<W> {
    inner: W,
    context: OperationContext,
    stats: TransferStats,
}

impl<W> StreamWriter<W>
where
    W: Write,
{
    /// Creates a writer with a default operation context.
    #[must_use]
    pub fn new(inner: W) -> Self {
        Self::with_context(
            inner,
            OperationContext::builder().build(),
        )
    }

    /// Creates a writer with an explicit operation context.
    #[must_use]
    pub fn with_context(
        inner: W,
        context: OperationContext,
    ) -> Self {
        Self {
            inner,
            context,
            stats: TransferStats::new(),
        }
    }

    /// Creates a buffered writer with configurable capacity.
    ///
    /// The capacity is a buffering preference, not a limit on
    /// the total amount of data that can be written.
    #[must_use]
    pub fn buffered(
        inner: W,
        capacity: NonZeroUsize,
    ) -> StreamWriter<BufWriter<W>> {
        StreamWriter::new(
            BufWriter::with_capacity(
                capacity.get(),
                inner,
            ),
        )
    }

    /// Creates a buffered writer with an explicit context.
    #[must_use]
    pub fn buffered_with_context(
        inner: W,
        capacity: NonZeroUsize,
        context: OperationContext,
    ) -> StreamWriter<BufWriter<W>> {
        StreamWriter::with_context(
            BufWriter::with_capacity(
                capacity.get(),
                inner,
            ),
            context,
        )
    }

    /// Writes bytes and returns the number accepted.
    ///
    /// The result may be smaller than the requested length.
    ///
    /// Callers requiring complete transfer should use
    /// [`Self::write_all_checked`].
    pub fn write_checked(
        &mut self,
        buffer: &[u8],
    ) -> Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }

        self.context.check_cancelled()?;

        self.write(buffer)
            .map_err(|error| {
                io_error(error, WRITE_OPERATION)
            })
    }

    /// Writes the entire supplied buffer.
    ///
    /// This operation correctly handles partial writes by
    /// repeatedly invoking the underlying writer.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - cancellation is requested;
    /// - the underlying writer fails;
    /// - the writer reports that it cannot make progress.
    pub fn write_all_checked(
        &mut self,
        buffer: &[u8],
    ) -> Result<()> {
        let mut remaining = buffer;

        while !remaining.is_empty() {
            self.context.check_cancelled()?;

            let amount = self.write_checked(remaining)?;

            if amount == 0 {
                return Err(
                    Error::new(
                        ErrorKind::Io,
                        "writer made no progress while data remained",
                    )
                    .in_component(COMPONENT)
                    .during_operation(WRITE_OPERATION),
                );
            }

            remaining = &remaining[amount..];
        }

        Ok(())
    }

    /// Flushes the underlying writer.
    ///
    /// Flush is deliberately not blocked by cancellation.
    ///
    /// This permits already accepted output to be flushed
    /// during cancellation and orderly completion.
    pub fn flush_checked(&mut self) -> Result<()> {
        self.inner.flush().map_err(|error| {
            io_error(error, FLUSH_OPERATION)
        })
    }

    /// Completes the stream and returns its underlying writer.
    ///
    /// Completion always attempts to flush first.
    ///
    /// If flushing fails, the error is returned and the
    /// underlying writer is dropped as ownership is consumed.
    ///
    /// This method does not claim that the destination has
    /// durably persisted the bytes. That guarantee belongs
    /// to the underlying provider.
    pub fn finish(mut self) -> Result<W> {
        self.flush_checked()?;
        Ok(self.inner)
    }

    /// Flushes and closes the adapter.
    ///
    /// Ownership of the underlying writer is consumed.
    ///
    /// The underlying writer is dropped after a successful flush.
    ///
    /// Implementations requiring a remote shutdown handshake,
    /// transactional commit, or provider-specific close operation
    /// must expose that through their own provider contract.
    pub fn close(self) -> Result<()> {
        drop(self.finish()?);
        Ok(())
    }

    /// Returns the number of bytes accepted by the writer.
    #[must_use]
    pub const fn stats(&self) -> &TransferStats {
        &self.stats
    }

    /// Returns the operation context.
    #[must_use]
    pub const fn context(&self) -> &OperationContext {
        &self.context
    }

    /// Returns a shared reference to the underlying writer.
    #[must_use]
    pub const fn get_ref(&self) -> &W {
        &self.inner
    }

    /// Returns a mutable reference to the underlying writer.
    ///
    /// Direct access bypasses this adapter's cancellation
    /// and accounting behavior.
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }
}

impl<W> Write for StreamWriter<W>
where
    W: Write,
{
    fn write(
        &mut self,
        buffer: &[u8],
    ) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }

        if let Err(error) = self.context.check_cancelled() {
            return Err(cancellation_io_error(error));
        }

        let amount = self.inner.write(buffer)?;

        self.stats.record(amount);

        Ok(amount)
    }

    fn flush(&mut self) -> io::Result<()> {
        // Do not block flushing merely because cancellation
        // was requested.
        self.inner.flush()
    }
}

// ============================================================================
// STREAM COPY
// ============================================================================

/// Copies bytes incrementally between a source and destination.
///
/// This operation does not materialize the entire source.
///
/// The caller chooses the transfer buffer size.
///
/// The buffer size is not a maximum stream size.
///
/// # Errors
///
/// Returns a structured error if reading, writing, flushing,
/// or cancellation fails.
///
/// Bytes already accepted by the destination cannot necessarily
/// be rolled back after a subsequent failure.
pub fn copy_stream<R, W>(
    reader: &mut StreamReader<R>,
    writer: &mut StreamWriter<W>,
    buffer: &mut [u8],
) -> Result<u128>
where
    R: Read,
    W: Write,
{
    if buffer.is_empty() {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                "stream copy buffer must not be empty",
            )
            .in_component(COMPONENT)
            .during_operation("io.stream.copy"),
        );
    }

    let initial_read = reader.stats().bytes();
    let initial_write = writer.stats().bytes();

    loop {
        let amount = reader.read_checked(buffer)?;

        if amount == 0 {
            break;
        }

        writer.write_all_checked(&buffer[..amount])?;
    }

    writer.flush_checked()?;

    let read_total = reader.stats().bytes()
        .saturating_sub(initial_read);

    let write_total = writer.stats().bytes()
        .saturating_sub(initial_write);

    // A successful write-all operation must have accepted
    // every byte read from the source.
    if !reader.stats().counter_saturated()
        && !writer.stats().counter_saturated()
        && read_total != write_total
    {
        return Err(
            Error::new(
                ErrorKind::ContractViolation,
                "stream copy accounting mismatch",
            )
            .in_component(COMPONENT)
            .during_operation("io.stream.copy"),
        );
    }

    Ok(read_total)
}

// ============================================================================
// DEBUG SUPPORT
// ============================================================================

impl<R> fmt::Debug for StreamReader<R> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("StreamReader")
            .field("state", &self.state)
            .field("stats", &self.stats)
            .finish_non_exhaustive()
    }
}

impl<W> fmt::Debug for StreamWriter<W> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("StreamWriter")
            .field("stats", &self.stats)
            .finish_non_exhaustive()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    #[test]
    fn reader_returns_real_input() {
        let source = Cursor::new(b"Zamani");

        let mut reader = StreamReader::new(source);

        let mut output = [0_u8; 6];

        let amount = reader
            .read_checked(&mut output)
            .expect("read should succeed");

        assert_eq!(amount, 6);
        assert_eq!(&output, b"Zamani");
        assert_eq!(reader.stats().bytes(), 6);
    }

    #[test]
    fn reader_detects_end_of_stream() {
        let source = Cursor::new(b"");

        let mut reader = StreamReader::new(source);

        let mut output = [0_u8; 1];

        assert_eq!(
            reader.read_checked(&mut output).unwrap(),
            0
        );

        assert!(reader.is_eof());
    }

    #[test]
    fn empty_read_does_not_change_eof_state() {
        let source = Cursor::new(b"Z");

        let mut reader = StreamReader::new(source);

        let mut output = [];

        assert_eq!(
            reader.read_checked(&mut output).unwrap(),
            0
        );

        assert_eq!(reader.state(), ReaderState::Open);
    }

    #[test]
    fn writer_preserves_real_output() {
        let mut writer = StreamWriter::new(Vec::new());

        writer
            .write_all_checked(b"Zamani")
            .expect("write should succeed");

        let output = writer
            .finish()
            .expect("flush should succeed");

        assert_eq!(output, b"Zamani");
    }

    #[test]
    fn writer_accounts_for_accepted_bytes() {
        let mut writer = StreamWriter::new(Vec::new());

        writer
            .write_all_checked(b"portable")
            .unwrap();

        assert_eq!(writer.stats().bytes(), 8);
    }

    #[test]
    fn buffered_reader_reads_incrementally() {
        let source = Cursor::new(b"portable streams");

        let capacity = NonZeroUsize::new(4).unwrap();

        let mut reader = StreamReader::buffered(
            source,
            capacity,
        );

        let mut output = Vec::new();

        reader
            .read_to_end(&mut output)
            .unwrap();

        assert_eq!(output, b"portable streams");
    }

    #[test]
    fn buffered_writer_flushes_on_finish() {
        let capacity = NonZeroUsize::new(4).unwrap();

        let mut writer = StreamWriter::buffered(
            Vec::new(),
            capacity,
        );

        writer
            .write_all_checked(b"buffered output")
            .unwrap();

        let output = writer.finish().unwrap();

        assert_eq!(output, b"buffered output");
    }

    #[test]
    fn stream_copy_transfers_all_bytes() {
        let source = Cursor::new(b"copy across providers");

        let mut reader = StreamReader::new(source);

        let mut writer = StreamWriter::new(Vec::new());

        let mut buffer = [0_u8; 5];

        let transferred = copy_stream(
            &mut reader,
            &mut writer,
            &mut buffer,
        )
        .unwrap();

        assert_eq!(transferred, 21);

        assert_eq!(
            writer.finish().unwrap(),
            b"copy across providers"
        );
    }

    #[test]
    fn zero_copy_buffer_is_rejected() {
        let source = Cursor::new(b"data");

        let mut reader = StreamReader::new(source);

        let mut writer = StreamWriter::new(Vec::new());

        let mut buffer = [];

        assert!(
            copy_stream(
                &mut reader,
                &mut writer,
                &mut buffer,
            )
            .is_err()
        );
    }

    #[test]
    fn reader_into_inner_preserves_ownership() {
        let source = Cursor::new(b"ownership");

        let reader = StreamReader::new(source);

        let source = reader.into_inner();

        assert_eq!(source.get_ref(), b"ownership");
    }

    #[test]
    fn transfer_stats_default_to_zero() {
        let stats = TransferStats::new();

        assert_eq!(stats.bytes(), 0);
        assert!(!stats.counter_saturated());
    }
}
