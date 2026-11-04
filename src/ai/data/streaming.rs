
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Dataset Streaming
//!
//! Path: `src/ai/data/streaming.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides a target-independent, incremental record-consumption interface
//! for logical AI datasets.
//!
//! A dataset stream exposes records progressively rather than requiring the
//! complete dataset to be loaded into memory.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The dataset record-source interface.
//! - Iterator-to-stream adaptation.
//! - Fallible source adaptation.
//! - Dataset-associated stream lifecycle.
//! - End-of-stream and terminal-error behavior.
//!
//! This module does not own:
//!
//! - Dataset identity allocation.
//! - Dataset definitions or schemas.
//! - Physical storage.
//! - Filesystem or database access.
//! - Network transport.
//! - Serialization formats.
//! - Tensor storage.
//! - Data transformation execution.
//! - Buffer allocation policies.
//! - Distributed partitioning or scheduling.
//! - Hardware discovery or selection.
//!
//! ## Canonical dependencies
//!
//! - `crate::ai::ids::AiDatasetId`
//! - `crate::ai::error::{AiError, AiErrorKind, AiErrorPhase, AiResult}`
//! - `crate::ai::data::datasets::DatasetDefinition`
//!
//! ## Streaming model
//!
//! The stream uses pull-based consumption.
//!
//! The consumer requests the next record. The source produces at most one
//! record for that request.
//!
//! This provides natural backpressure without requiring a fixed-size
//! internal queue or background worker.
//!
//! Sources that need asynchronous or concurrent production can implement
//! `DatasetRecordSource` and coordinate with their own execution layer.
//!
//! ## Failure behavior
//!
//! A source error is returned to the consumer exactly once.
//!
//! After an error, the stream becomes terminal. It will not silently resume,
//! skip the failed record, or report successful completion.
//!
//! End-of-stream is also terminal.
//!
//! ## POCO-REAF
//!
//! This module does not impose a universal maximum on:
//!
//! - Dataset records.
//! - Stream duration.
//! - Dataset size.
//! - Number of streams.
//! - Number of providers.
//! - Tensor rank.
//! - Worker count.
//! - Device count.
//! - Memory capacity.
//!
//! The stream itself does not buffer records. Memory use is therefore
//! determined by the source, the current record, and the consumer.
//!
//! Actual allocation, provider, process, and resource limitations remain
//! explicit operational concerns.
//!
//! Literal infinite computation is not promised.
//!
//! ## Grammar integration
//!
//! Source syntax is owned by:
//!
//! - `grammar/data/datasets.g4`
//! - `grammar/ai/datasets.g4`
//!
//! Their streaming declarations express source-level intent.
//!
//! The frontend AST and semantic subsystem must resolve that intent before
//! a runtime provider is connected to this interface.
//!
//! This module does not parse source syntax or evaluate expressions.
//!
//! ## Integration
//!
//! - `data/datasets.rs` supplies `AiDatasetId` through `DatasetDefinition`.
//! - `data/schemas.rs` remains the schema authority.
//! - `data/provenance.rs` remains the dataset-lineage authority.
//! - `data/preprocessing.rs` may consume this stream as an input boundary.
//! - `data/validation.rs` may validate records as they are consumed.
//! - `learning/training.rs` may consume validated dataset streams.
//! - `learning/federated.rs` may use provider adapters for local data.
//! - `src/distributed/` owns distributed transport and coordination.
//! - `src/runtime/` owns runtime lifecycle and resource management.
//!
//! ## Freeze criteria
//!
//! - No unsafe code.
//! - No arbitrary fixed record or buffer limit.
//! - No complete-dataset materialization.
//! - Source failures are explicit.
//! - Errors are terminal and reported once.
//! - End-of-stream is terminal.
//! - Dataset identity is preserved.
//! - Ordinary iterators can be adapted.
//! - Fallible providers can implement the source interface.
//! - No hardware or provider assumptions are embedded here.

use core::iter::FusedIterator;

use crate::ai::data::datasets::DatasetDefinition;
use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiErrorPhase,
    AiResult,
};
use crate::ai::ids::AiDatasetId;

// ============================================================================
// DATASET RECORD SOURCE
// ============================================================================

/// A source capable of incrementally producing dataset records.
///
/// Implementations must not interpret `Ok(None)` as a temporary lack of
/// currently available data. It means that the source has completed.
///
/// A source that can temporarily have no record available must represent
/// that condition through a separate asynchronous or readiness-aware
/// adapter rather than incorrectly signalling end-of-stream.
///
/// The source owns its provider-specific state and resource lifecycle.
pub trait DatasetRecordSource<T> {
    /// Produce the next record.
    ///
    /// Returns:
    ///
    /// - `Ok(Some(record))` when a record is available.
    /// - `Ok(None)` when the source has permanently completed.
    /// - `Err(error)` when the source has failed.
    ///
    /// # Errors
    ///
    /// Returns the provider's structured AI error when record production
    /// fails.
    fn next_record(&mut self) -> AiResult<Option<T>>;
}

// ============================================================================
// FALLIBLE CLOSURE SOURCE
// ============================================================================

/// Allows a fallible closure to act as a dataset record source.
///
/// The closure is called once for each requested record.
impl<T, F> DatasetRecordSource<T> for F
where
    F: FnMut() -> AiResult<Option<T>>,
{
    fn next_record(&mut self) -> AiResult<Option<T>> {
        self()
    }
}

// ============================================================================
// ITERATOR ADAPTER
// ============================================================================

/// Adapts an ordinary Rust iterator to the dataset record-source interface.
///
/// The iterator remains responsible for producing records. This adapter
/// does not collect, clone, prefetch, or buffer the iterator's contents.
#[derive(Debug, Clone)]
pub struct IteratorSource<I> {
    iterator: I,
}

impl<I> IteratorSource<I> {
    /// Construct an iterator-backed source.
    #[must_use]
    pub const fn new(iterator: I) -> Self {
        Self { iterator }
    }

    /// Recover the underlying iterator.
    #[must_use]
    pub fn into_inner(self) -> I {
        self.iterator
    }

    /// Borrow the underlying iterator.
    #[must_use]
    pub const fn get_ref(&self) -> &I {
        &self.iterator
    }

    /// Mutably borrow the underlying iterator.
    pub fn get_mut(&mut self) -> &mut I {
        &mut self.iterator
    }
}

impl<T, I> DatasetRecordSource<T> for IteratorSource<I>
where
    I: Iterator<Item = T>,
{
    fn next_record(&mut self) -> AiResult<Option<T>> {
        Ok(self.iterator.next())
    }
}

// ============================================================================
// STREAM LIFECYCLE
// ============================================================================

/// The lifecycle state of a dataset stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamState {
    /// The source may still produce records.
    Active,

    /// The source has completed normally.
    Exhausted,

    /// The source failed and the stream cannot continue.
    Failed,
}

// ============================================================================
// DATASET STREAM
// ============================================================================

/// A dataset-associated incremental record stream.
///
/// The stream associates a provider with one logical dataset identity.
///
/// It does not own the dataset definition or the underlying physical data.
///
/// The source is consumed incrementally. The stream does not accumulate
/// records or impose an internal buffering limit.
#[derive(Debug)]
pub struct DatasetStream<S> {
    dataset_id: AiDatasetId,
    source: S,
    state: StreamState,
}

impl<S> DatasetStream<S> {
    /// Construct a stream for a logical dataset identity.
    ///
    /// This operation does not open a file, connect to a provider, allocate
    /// a buffer, or start background execution.
    #[must_use]
    pub const fn new(
        dataset_id: AiDatasetId,
        source: S,
    ) -> Self {
        Self {
            dataset_id,
            source,
            state: StreamState::Active,
        }
    }

    /// Construct a stream associated with an existing dataset definition.
    ///
    /// The definition is borrowed only to obtain its canonical identity.
    /// It is not copied or modified.
    #[must_use]
    pub fn for_dataset(
        dataset: &DatasetDefinition,
        source: S,
    ) -> Self {
        Self::new(dataset.id(), source)
    }

    /// Return the logical dataset identity.
    #[must_use]
    pub const fn dataset_id(&self) -> AiDatasetId {
        self.dataset_id
    }

    /// Return the current stream lifecycle state.
    #[must_use]
    pub const fn state(&self) -> StreamState {
        self.state
    }

    /// Return whether the stream has permanently terminated.
    #[must_use]
    pub const fn is_terminated(&self) -> bool {
        !matches!(self.state, StreamState::Active)
    }

    /// Borrow the underlying source.
    ///
    /// This is intended for provider inspection and integration.
    /// It does not change the stream lifecycle.
    #[must_use]
    pub const fn source(&self) -> &S {
        &self.source
    }

    /// Mutably borrow the underlying source.
    ///
    /// Implementations should not use this to bypass the stream's
    /// lifecycle contract.
    pub fn source_mut(&mut self) -> &mut S {
        &mut self.source
    }

    /// Consume the stream and recover its underlying source.
    ///
    /// Any provider-specific cleanup remains the responsibility of the
    /// source implementation and its ownership semantics.
    #[must_use]
    pub fn into_source(self) -> S {
        self.source
    }
}

impl<S> DatasetStream<S> {
    /// Request the next record.
    ///
    /// This method performs one pull from the underlying source.
    ///
    /// It never prefetches or buffers additional records.
    ///
    /// # Results
    ///
    /// - `Ok(Some(record))`: one record was produced.
    /// - `Ok(None)`: the source completed or the stream was already
    ///   terminated.
    /// - `Err(error)`: the source failed. The stream becomes terminal.
    ///
    /// A source failure is returned exactly once. Subsequent calls return
    /// `Ok(None)` because the stream is no longer active.
    ///
    /// # Errors
    ///
    /// Returns the provider's error with the logical dataset identity
    /// and data-processing phase attached.
    pub fn next_record<T>(&mut self) -> AiResult<Option<T>>
    where
        S: DatasetRecordSource<T>,
    {
        if self.state != StreamState::Active {
            return Ok(None);
        }

        match self.source.next_record() {
            Ok(Some(record)) => Ok(Some(record)),

            Ok(None) => {
                self.state = StreamState::Exhausted;
                Ok(None)
            }

            Err(error) => {
                self.state = StreamState::Failed;

                Err(error
                    .with_phase(AiErrorPhase::DataProcessing)
                    .with_dataset(self.dataset_id.to_string()))
            }
        }
    }
}

// ============================================================================
// ITERATOR CONSTRUCTION
// ============================================================================

impl<S> DatasetStream<S> {
    /// Adapt an ordinary iterator into a dataset stream.
    ///
    /// The iterator is consumed lazily. Its records are not collected into
    /// a vector or copied into an internal queue.
    #[must_use]
    pub fn from_iterator<I>(
        dataset_id: AiDatasetId,
        iterator: I,
    ) -> DatasetStream<IteratorSource<I>>
    where
        I: Iterator,
    {
        DatasetStream::new(
            dataset_id,
            IteratorSource::new(iterator),
        )
    }

    /// Adapt an ordinary iterator associated with a dataset definition.
    #[must_use]
    pub fn from_dataset_iterator<I>(
        dataset: &DatasetDefinition,
        iterator: I,
    ) -> DatasetStream<IteratorSource<I>>
    where
        I: Iterator,
    {
        Self::from_iterator(dataset.id(), iterator)
    }
}

// ============================================================================
// ITERATOR INTEROPERABILITY
// ============================================================================

impl<T, S> Iterator for DatasetStream<S>
where
    S: DatasetRecordSource<T>,
{
    type Item = AiResult<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_record() {
            Ok(Some(record)) => Some(Ok(record)),
            Ok(None) => None,
            Err(error) => Some(Err(error)),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.state {
            StreamState::Active => (0, None),
            StreamState::Exhausted | StreamState::Failed => (0, Some(0)),
        }
    }
}

impl<T, S> FusedIterator for DatasetStream<S>
where
    S: DatasetRecordSource<T>,
{
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::ids::AiDatasetId;

    fn dataset_id() -> AiDatasetId {
        AiDatasetId::new(1).expect("valid dataset identity")
    }

    #[test]
    fn iterator_source_produces_records_incrementally() {
        let source = IteratorSource::new([10, 20, 30].into_iter());

        let mut stream = DatasetStream::new(dataset_id(), source);

        assert_eq!(stream.next_record().expect("record"), Some(10));
        assert_eq!(stream.next_record().expect("record"), Some(20));
        assert_eq!(stream.next_record().expect("record"), Some(30));
        assert_eq!(stream.next_record().expect("end"), None);

        assert_eq!(stream.state(), StreamState::Exhausted);
        assert!(stream.is_terminated());
    }

    #[test]
    fn ordinary_iterator_can_be_adapted_directly() {
        let mut stream =
            DatasetStream::from_iterator(dataset_id(), 0..5);

        let records = stream
            .by_ref()
            .collect::<AiResult<Vec<_>>>()
            .expect("all records");

        assert_eq!(records, vec![0, 1, 2, 3, 4]);
        assert_eq!(stream.state(), StreamState::Exhausted);
    }

    #[test]
    fn stream_preserves_dataset_identity() {
        let stream = DatasetStream::from_iterator(
            dataset_id(),
            std::iter::empty::<u8>(),
        );

        assert_eq!(stream.dataset_id(), dataset_id());
    }

    #[test]
    fn completed_stream_is_fused() {
        let mut stream =
            DatasetStream::from_iterator(dataset_id(), [1, 2].into_iter());

        assert_eq!(stream.next(), Some(Ok(1)));
        assert_eq!(stream.next(), Some(Ok(2)));
        assert_eq!(stream.next(), None);

        assert_eq!(stream.state(), StreamState::Exhausted);
        assert_eq!(stream.next(), None);
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn fallible_closure_can_provide_records() {
        let mut records = [Some(4), Some(8), None].into_iter();

        let source = move || Ok(records.next().flatten());

        let mut stream = DatasetStream::new(dataset_id(), source);

        assert_eq!(stream.next_record().expect("record"), Some(4));
        assert_eq!(stream.next_record().expect("record"), Some(8));
        assert_eq!(stream.next_record().expect("end"), None);
        assert_eq!(stream.state(), StreamState::Exhausted);
    }

    #[test]
    fn source_failure_is_reported_once() {
        let mut calls = 0usize;

        let source = move || {
            calls += 1;

            if calls == 1 {
                Ok(Some(7))
            } else {
                Err(AiError::new(
                    AiErrorKind::DataFailure,
                    "test source failure",
                ))
            }
        };

        let mut stream = DatasetStream::new(dataset_id(), source);

        assert_eq!(stream.next_record().expect("first record"), Some(7));

        let error = stream
            .next_record::<u8>()
            .expect_err("source must fail");

        assert_eq!(error.kind(), AiErrorKind::DataFailure);
        assert_eq!(stream.state(), StreamState::Failed);

        assert_eq!(stream.next_record::<u8>().expect("terminal"), None);
        assert_eq!(stream.next_record::<u8>().expect("terminal"), None);
    }

    #[test]
    fn iterator_adapter_reports_errors_as_iterator_items() {
        let mut calls = 0usize;

        let source = move || {
            calls += 1;

            if calls == 1 {
                Ok(Some("record"))
            } else {
                Err(AiError::new(
                    AiErrorKind::DataFailure,
                    "provider unavailable",
                ))
            }
        };

        let mut stream = DatasetStream::new(dataset_id(), source);

        assert_eq!(stream.next(), Some(Ok("record")));

        let error = stream
            .next()
            .expect("one error item")
            .expect_err("provider error");

        assert_eq!(error.kind(), AiErrorKind::DataFailure);
        assert_eq!(stream.state(), StreamState::Failed);

        assert_eq!(stream.next(), None);
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn iterator_size_hint_does_not_invent_an_upper_bound() {
        let stream =
            DatasetStream::from_iterator(dataset_id(), 0..);

        assert_eq!(stream.size_hint(), (0, None));
    }

    #[test]
    fn stream_does_not_consume_records_without_a_pull() {
        use std::cell::Cell;
        use std::rc::Rc;

        let calls = Rc::new(Cell::new(0));
        let source_calls = Rc::clone(&calls);

        let source = move || {
            source_calls.set(source_calls.get() + 1);
            Ok(Some(source_calls.get()))
        };

        let mut stream = DatasetStream::new(dataset_id(), source);

        assert_eq!(calls.get(), 0);

        assert_eq!(stream.next_record().expect("record"), Some(1));
        assert_eq!(calls.get(), 1);

        assert_eq!(stream.next_record().expect("record"), Some(2));
        assert_eq!(calls.get(), 2);
    }

    #[test]
    fn source_can_be_recovered() {
        let source = IteratorSource::new([1, 2, 3].into_iter());

        let stream = DatasetStream::new(dataset_id(), source);

        let iterator = stream.into_source().into_inner();

        assert_eq!(iterator.collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    #[test]
    fn empty_source_terminates_normally() {
        let mut stream = DatasetStream::from_iterator(
            dataset_id(),
            std::iter::empty::<u64>(),
        );

        assert_eq!(stream.next_record().expect("end"), None);
        assert_eq!(stream.state(), StreamState::Exhausted);
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn a_large_logical_range_is_consumed_without_dataset_collection() {
        let mut stream =
            DatasetStream::from_iterator(dataset_id(), 0_u64..);

        for expected in 0..10_000_u64 {
            assert_eq!(
                stream.next_record().expect("record"),
                Some(expected)
            );
        }

        // The source remains active. No complete dataset was materialized.
        assert_eq!(stream.state(), StreamState::Active);
    }
}
